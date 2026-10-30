#include "areadigest/actor_source.h"

#include <algorithm>
#include <cmath>
#include "actors/actor_trace.h"
#include "fnvcoop_mut.h"

namespace nvcoop::areadigest {

namespace {
constexpr size_t kMaxStepsPerTick = 24;   // engine writes one smoothing step may cost per tick (the port budgets the same way)
}

ActorAreaSource::ActorAreaSource(IActorFacade& f, const Config& cfg, size_t maxActors) : f_(f), cfg_(cfg), maxActors_(maxActors), tr_(cfg) {}

void ActorAreaSource::collect(uint64_t nowMs, std::vector<Item>& items, std::vector<CellId>& loadedCells) {
    if (!f_.ready()) return;
    ++st_.scans;
    std::vector<ActorRead> v;
    f_.tracked(v, maxActors_ + 1);
    if (v.size() > maxActors_) { v.resize(maxActors_); ++st_.capped; }
    std::vector<Item> mine;
    mine.reserve(v.size());
    for (const ActorRead& r : v) {
        Item it;
        it.key = r.netId;
        it.cat = Cat::Actors;
        it.cell = r.cell;
        it.alive = FNV_MUT(70) ? true : r.alive;     // twin 70: a dead actor is reported as alive
        it.hp = r.hp;
        it.pos = r.pos;
        mine.push_back(it);
        loadedCells.push_back(r.cell);
    }
    tr_.update(mine, nowMs);
    for (Item& it : mine) items.push_back(it);
    CellId pc = 0;
    if (f_.playerCell(pc)) loadedCells.push_back(pc);
}

bool ActorAreaSource::find(Key key, Item& out, uint64_t nowMs) {
    ActorRead r;
    if (key > 0xFFFFFFFFull || !f_.read(static_cast<uint32_t>(key), r)) return false;
    out = Item();
    out.key = r.netId;
    out.cat = Cat::Actors;
    out.cell = r.cell;
    out.alive = r.alive;
    out.hp = r.hp;
    out.pos = r.pos;
    tr_.refresh(out, nowMs);
    return true;
}

bool ActorAreaSource::owner(Key key, OwnerView& out) {
    return key <= 0xFFFFFFFFull && f_.owner(static_cast<uint32_t>(key), out);
}

HealResult ActorAreaSource::heal(const Item& auth, const Diff& plan, uint64_t nowMs) {
    const uint32_t id = static_cast<uint32_t>(auth.key);
    ActorRead cur;
    if (auth.key > 0xFFFFFFFFull || !f_.read(id, cur)) { ++st_.missing; return HealResult::Missing; }
    bool did = false, delegated = false;

    if (plan.alive) {
        if (!auth.alive && cur.alive && (!cur.named || FNV_MUT(65))) {
            // The authority says dead and this copy still stands. Death is declared by the owner (a named NPC: by the story host); ask the actor
            // module to bring the owner's state over instead of killing the copy here (it would be revived after its death grace anyway).
            if (FNV_MUT(64) ? false : f_.requestOwnerResync(id)) { delegated = true; ++st_.deathRequests; }
            else ++st_.refused;
        } else if (auth.alive && !cur.alive) {
            if (FNV_MUT(66)) {   // twin 66: a corpse is revived because the authority has it alive
                if (f_.setHealth(id, std::max(1.0f, auth.hp))) did = true;
            } else {
                ++st_.refused;   // a resurrect is a story effect (ResurrectActor), never a heal
            }
        } else {
            ++st_.refused;
        }
    }

    if (cur.alive && auth.alive) {
        const bool movePlanned = plan.cell || plan.pos != fnvcoop::areadigest::RubberMode::None;
        const bool held = movePlanned && !FNV_MUT(83) && f_.moveHeld(id);   // twin 83: the digest drags a copy that is still getting out of its chair
        static nvcoop::actors::Throttle thr;
        if (held) {
            ++st_.heldByPoseLock;
            smooth_.erase(id);
            if (thr.allow(20, id, static_cast<uint32_t>(nowMs), 3000))
                nvcoop::actors::trace("areadigest: actor %s position heal left alone: the copy is held while it gets out of furniture (authority at (%.0f,%.0f,%.0f), copy at (%.0f,%.0f,%.0f))", nvcoop::actors::who(id).c_str(),
                                      auth.pos.x, auth.pos.y, auth.pos.z, cur.pos.x, cur.pos.y, cur.pos.z);
        } else {
            if (movePlanned && thr.allow(21, id, static_cast<uint32_t>(nowMs), 1000))
                nvcoop::actors::trace("areadigest: actor %s %s: copy at (%.0f,%.0f,%.0f) cell %08X, authority at (%.0f,%.0f,%.0f) cell %08X, %.1f m apart", nvcoop::actors::who(id).c_str(),
                                      plan.cell ? "snapped to another cell" : (plan.pos == fnvcoop::areadigest::RubberMode::Snap ? "snapped" : "smoothed"), cur.pos.x, cur.pos.y, cur.pos.z, cur.cell,
                                      auth.pos.x, auth.pos.y, auth.pos.z, auth.cell, plan.dist / 70.0f);
            if (plan.cell || plan.pos == fnvcoop::areadigest::RubberMode::Snap) {
                smooth_.erase(id);
                bool ok;
                if (plan.cell && !FNV_MUT(67)) ok = f_.moveToCell(id, auth.cell, auth.pos);   // twin 67: another cell is "fixed" with a same-cell move
                else ok = f_.setPos(id, auth.pos);
                if (ok) { ++st_.snaps; did = true; }
                else ++st_.moveFailed;
            } else if (plan.pos == fnvcoop::areadigest::RubberMode::Smooth) {
                if (FNV_MUT(68)) {   // twin 68: a 2 m .. 10 m error is snapped instead of smoothed
                    if (f_.setPos(id, auth.pos)) { ++st_.snaps; did = true; } else ++st_.moveFailed;
                } else {
                    fnvcoop::areadigest::Smoother s;
                    s.begin(nowMs, cur.pos, auth.pos, cfg_);
                    smooth_[id] = s;
                    ++st_.smooths;
                    did = true;
                }
            }
        }
        if (plan.hp) {
            const float v = auth.alive ? std::max(1.0f, auth.hp) : auth.hp;   // never write 0 into a living actor: that is a death
            if (f_.setHealth(id, v)) { ++st_.hpSet; did = true; }
        }
    }
    if (did) return HealResult::Applied;
    return delegated ? HealResult::Delegated : HealResult::Refused;
}

void ActorAreaSource::tick(uint64_t nowMs) {
    if (smooth_.empty()) return;
    size_t budget = kMaxStepsPerTick;
    for (auto it = smooth_.begin(); it != smooth_.end();) {
        if (budget == 0) break;
        --budget;
        ActorRead cur;
        if (!f_.read(it->first, cur) || !cur.alive) { it = smooth_.erase(it); continue; }   // gone or dead: nothing to move
        if (!FNV_MUT(83) && f_.moveHeld(it->first)) { ++it; continue; }   // held while it gets out of furniture: the job waits
        Vec3 p;
        const bool more = it->second.step(nowMs, p);
        if (more) p.z = cur.pos.z;   // height is left to the engine (gravity / navmesh) until the last step puts the exact target
        if (!f_.setPos(it->first, p)) ++st_.moveFailed;
        ++st_.smoothSteps;
        if (!more) it = smooth_.erase(it);
        else ++it;
    }
}

}  // namespace nvcoop::areadigest
