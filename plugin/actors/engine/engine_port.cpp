#include "actors/engine/engine_port.h"

#include "actors/actor_trace.h"

#include <algorithm>
#include <cmath>

#include "actors/engine/registry.h"
#include "avatar/loco.h"
#include "fnvcoop_mut.h"

namespace nvcoop::actors::engine {

namespace {

constexpr float kRadToDeg = 57.29577951308232f;
constexpr uint64_t kSweepMs = 2000;

float wrapDeg(float d) {
    d = std::fmod(d, 360.0f);
    if (d > 180.0f) d -= 360.0f;
    if (d <= -180.0f) d += 360.0f;
    return d;
}
float yawDiffDeg(float aRad, float bRad) { return std::fabs(wrapDeg((aRad - bRad) * kRadToDeg)); }
float dist(const live::Vec3& a, const live::Vec3& b) { return live::distance(a, b); }

}  // namespace

EnginePort::EnginePort(IActorEngine& eng, Ledger& ledger, const Config& cfg) : eng_(eng), led_(ledger), cfg_(cfg) {}

// ---- small helpers -----------------------------------------------------------------------------------------------------------------

RefId EnginePort::activeOf(const Entry& e) const {
    if (e.kind == CloneKind::PickSwap && e.clone) return e.clone;
    if (e.real) return e.real;
    return e.clone;
}

void EnginePort::index(NetId id, RefId ref) { if (ref) byRef_[ref] = id; }
void EnginePort::unindex(RefId ref) { byRef_.erase(ref); }

void EnginePort::eraseIfEmpty(NetId id) {
    Entry* e = find(id);
    if (e && !e->real && !e->clone) e_.erase(id);
}

bool EnginePort::linesLeft() const {
    if (FNV_MUT(16)) return true;   // BAD: the per-frame console-line budget is ignored
    return lines() - frameStartLines_ < cfg_.maxLinesPerFrame;
}

bool EnginePort::sameSpace(CellId a, CellId b) {
    if (a == b) return true;
    CellInfo x, y;
    if (!eng_.cellInfo(a, x) || !eng_.cellInfo(b, y)) return false;
    return x.known && y.known && x.exterior && y.exterior && x.world == y.world;
}

void EnginePort::ledgerAdd(LedgerKind k, RefId ref) {
    led_.add(k, ref);
    live_.insert({static_cast<char>(k), ref});
}

void EnginePort::ledgerRemove(LedgerKind k, RefId ref) {
    led_.remove(k, ref);
    live_.erase({static_cast<char>(k), ref});
}

// ---- frame ---------------------------------------------------------------------------------------------------------------------------

void EnginePort::beginFrame(uint64_t nowMs) {
    now_ = nowMs;
    frameStartLines_ = lines();
    cellCache_.clear();
}

void EnginePort::flush() {
    // 0. removed clones whose quiet period is over
    reapDoomed(false);

    // 1. kill / revive retries: a console line can be refused; the target state is checked by reading the actor back
    std::vector<NetId> retry;
    for (auto& kv : e_)
        if (kv.second.killWanted || kv.second.reviveWanted) retry.push_back(kv.first);
    for (NetId id : retry) {
        Entry* e = find(id);
        if (!e || now_ < e->nextTryMs) continue;
        const RefId r = activeOf(*e);
        ActorRaw raw;
        if (!r || !eng_.read(r, raw)) { e->killWanted = e->reviveWanted = false; continue; }
        if (e->killWanted) {
            if (raw.dead()) e->killWanted = false;
            else if (e->killTries < 5) { killRef(r); ++e->killTries; e->nextTryMs = now_ + 200; }
            else e->killWanted = false;
        } else if (e->reviveWanted) {
            if (!raw.dead()) e->reviveWanted = false;
            else if (e->reviveTries < 5) { eng_.resurrect(r); ++e->reviveTries; e->nextTryMs = now_ + 200; }
            else e->reviveWanted = false;
        }
    }

    // 2. pending drives: snapped first, then the actor that has waited longest
    std::vector<Entry*> todo;
    for (auto& kv : e_)
        if (kv.second.pending) todo.push_back(&kv.second);
    std::sort(todo.begin(), todo.end(), [](const Entry* a, const Entry* b) {
        const bool sa = a->want.snapped || a->want.state == live::InterpState::Snapped;
        const bool sb = b->want.snapped || b->want.state == live::InterpState::Snapped;
        if (sa != sb) return sa;
        if (FNV_MUT(17)) return a->id < b->id;   // BAD: fixed order, the same low ids get the whole budget every frame
        if (a->appliedMs != b->appliedMs) return a->appliedMs < b->appliedMs;
        return a->id < b->id;
    });
    for (Entry* e : todo) {
        if (!linesLeft()) { ++st_.linesDeferred; break; }
        if (applyPose(*e, e->want, false)) e->pending = false;
    }

    // 3. housekeeping
    if (now_ >= lastSweepMs_ + kSweepMs) {
        lastSweepMs_ = now_;
        undoStale();
        sweepOrphans();
    }
    if (cfg_.persistLedger) {
        const uint64_t before = led_.writes();
        led_.flush(now_);
        st_.ledgerWrites += led_.writes() - before;
    }
}

// ---- directory -------------------------------------------------------------------------------------------------------------------------

void EnginePort::bindReal(const ActorMeta& meta, RefId ref) {
    if (!meta.netId || !ref) return;
    Entry& e = e_[meta.netId];
    e.id = meta.netId;
    e.meta = meta;
    if (e.real && e.real != ref) unindex(e.real);
    e.real = ref;
    index(meta.netId, ref);
}

void EnginePort::unbindReal(NetId id) {
    Entry* e = find(id);
    if (!e) return;
    if (e->kind == CloneKind::PickSwap) dropClone(*e, true);    // the clone belongs to the cell that just went away; the real actor must not stay disabled
    if (e->real) unindex(e->real);
    e->real = 0;
    e->pending = false;
    e->haveApplied = false;
    e->havePrev = false;
    e->aiRef = 0;   // the flag stays on the (unloaded) ref and in the ledger; it is undone when the ref comes back / by the sweep
    eraseIfEmpty(id);
}

bool EnginePort::bound(NetId id) const { const Entry* e = find(id); return e && activeOf(*e) != 0; }
RefId EnginePort::realRef(NetId id) const { const Entry* e = find(id); return e ? e->real : 0; }
RefId EnginePort::activeRef(NetId id) const { const Entry* e = find(id); return e ? activeOf(*e) : 0; }
NetId EnginePort::netOfRef(RefId ref) const { auto it = byRef_.find(ref); return it == byRef_.end() ? 0 : it->second; }

bool EnginePort::isOurClone(RefId ref) const {
    const NetId id = netOfRef(ref);
    const Entry* e = id ? find(id) : nullptr;
    return e && e->clone == ref;
}

bool EnginePort::isStandInRef(RefId ref) const {
    const NetId id = netOfRef(ref);
    const Entry* e = id ? find(id) : nullptr;
    return e && e->clone == ref && e->kind == CloneKind::StandIn;
}

std::vector<NetId> EnginePort::ids() const {
    std::vector<NetId> v;
    for (auto& kv : e_) v.push_back(kv.first);
    return v;
}

size_t EnginePort::standInCount() const {
    size_t n = 0;
    for (auto& kv : e_) if (kv.second.kind == CloneKind::StandIn && kv.second.clone) ++n;
    return n;
}

void EnginePort::rememberAi(NetId id, const live::AiState& ai) {
    Entry* e = find(id);
    if (!e) return;
    e->lastAi = ai;
    e->haveAi = true;
}

RefId EnginePort::resolveTarget(NetId target) const {
    if (!target) return 0;
    const Entry* e = find(target);
    if (e) return activeOf(*e);
    Registry& reg = registry();
    return reg.playerRefByNet ? reg.playerRefByNet(target) : 0;
}

bool EnginePort::aiOff(NetId id) const { const Entry* e = find(id); return e && e->aiOff; }
bool EnginePort::isProtected(NetId id) const { const Entry* e = find(id); return e && e->protOn; }
bool EnginePort::pendingDrive(NetId id) const { const Entry* e = find(id); return e && e->pending; }

// ---- IActorPort: reads -----------------------------------------------------------------------------------------------------------------

bool EnginePort::loaded(NetId id) {
    const Entry* e = find(id);
    return e && activeOf(*e) != 0;
}

bool EnginePort::cellLoaded(CellId cell) {
    auto it = cellCache_.find(cell);
    if (it != cellCache_.end()) return it->second;
    const bool v = eng_.cellLoaded(cell);
    cellCache_[cell] = v;
    return v;
}

bool EnginePort::pose(NetId id, live::ActualPose& out) {
    Entry* e = find(id);
    const RefId r = e ? activeOf(*e) : 0;
    CellId cell = 0;
    live::Vec3 pos;
    float rotZ = 0;
    if (!r || !eng_.readPose(r, cell, pos, rotZ)) return false;
    if (FNV_MUT(18) && e->haveApplied) { out.cell = e->appliedCell; out.pos = e->appliedPos; return true; }   // BAD: reports what we last wrote, not where the ref is
    out.cell = cell;
    out.pos = pos;
    return true;
}

bool EnginePort::sample(NetId id, live::ActorSnapshot& out) {
    Entry* e = find(id);
    const RefId r = e ? activeOf(*e) : 0;
    ActorRaw raw;
    if (!r || !eng_.read(r, raw)) return false;
    out = live::ActorSnapshot{};
    out.netId = id;
    out.cell = raw.cell;
    out.pos = raw.pos;
    // rot = (pitch, yaw, roll) as the avatar layer sends it: yaw is the REFR rotZ heading
    out.rot.x = raw.rot.x;
    out.rot.y = FNV_MUT(19) ? raw.rot.y : raw.rot.z;   // BAD (19): the wrong REFR angle is sent as the heading
    out.rot.z = raw.rot.y;

    // velocity from consecutive samples (the engine's own velocity is not read: no verified accessor)
    live::Vec3 vel;
    if (e->havePrev && now_ > e->prevMs) {
        const float dt = static_cast<float>(now_ - e->prevMs) / 1000.0f;
        if (dt >= 0.02f) {
            vel = {(raw.pos.x - e->prevPos.x) / dt, (raw.pos.y - e->prevPos.y) / dt, (raw.pos.z - e->prevPos.z) / dt};
            if (dist(raw.pos, e->prevPos) > 500.0f) vel = {};            // a teleport is not a velocity
            const float sp = std::sqrt(vel.x * vel.x + vel.y * vel.y + vel.z * vel.z);
            if (sp > 1500.0f) { const float k = 1500.0f / sp; vel.x *= k; vel.y *= k; vel.z *= k; }
            e->prevPos = raw.pos; e->prevMs = now_; e->prevVel = vel;
        } else {
            vel = e->prevVel;
        }
    } else {
        e->prevPos = raw.pos; e->prevMs = now_; e->prevVel = {};
        e->havePrev = true;
    }
    if (FNV_MUT(20)) vel = {};   // BAD: velocity never filled in (the receiver cannot extrapolate or pick a gait)
    out.vel = vel;

    // animation facts
    out.anim.weaponDrawn = raw.weaponOut;
    out.anim.aim = raw.aiming;
    out.anim.sneak = !FNV_MUT(21) && raw.haveMove && sneakFromMoveFlags(raw.moveFlags);
    avatar::LocalRead lr;
    lr.vel = {vel.x, vel.y, vel.z};
    lr.yaw = out.rot.y;
    lr.haveMoveFlags = raw.haveMove;
    lr.moverRun = raw.haveMove && (raw.moveFlags & 0x100u) != 0;
    lr.moverWalk = raw.haveMove && (raw.moveFlags & 0x80u) != 0;
    lr.moverSneak = out.anim.sneak;
    out.anim.locomotion = avatar::CaptureLoco(lr);

    // AI facts
    if (raw.combatTarget) {
        NetId t = netOfRef(raw.combatTarget);
        if (!t) {
            Registry& reg = registry();
            if (reg.playerNetByRef) t = reg.playerNetByRef(raw.combatTarget);
        }
        out.ai.combatTarget = t;
    }
    out.ai.package = raw.package & 0x00FFFFFFu;
    out.ai.alert = raw.inCombat ? 3 : (raw.weaponOut ? 2 : 0);
    // The final state of a handoff is sampled AFTER the AI went off (StopCombat clears the target): it must still say what the AI was doing,
    // or the new owner cannot take the fight up again.
    if (e->aiOff && e->haveSampledAi && !raw.combatTarget && !FNV_MUT(64)) out.ai = e->sampledAi;
    else if (!e->aiOff) { e->sampledAi = out.ai; e->haveSampledAi = true; }

    // actor values
    if (!FNV_MUT(22)) {
        for (uint8_t av : watchedAvs()) {
            float v = 0;
            if (!eng_.actorValue(r, av, v)) continue;
            live::AvValue x;
            x.av = av;
            x.value = static_cast<int32_t>(std::lround(v));
            out.avDeltas.push_back(x);
        }
    }

    Registry& reg = registry();
    if (reg.animSampler) reg.animSampler(r, out);
    return true;
}

// ---- IActorPort: driving ---------------------------------------------------------------------------------------------------------------

void EnginePort::drive(NetId id, const live::InterpOut& out) {
    Entry* e = find(id);
    if (!e) return;
    const RefId r = activeOf(*e);
    if (!r) return;
    e->lastAi = out.ai;
    e->haveAi = true;
    Registry& reg = registry();
    if (reg.hasDriveListeners()) {
        DriveEvent ev;
        ev.netId = id; ev.ref = r; ev.standIn = e->kind == CloneKind::StandIn && !e->real; ev.out = &out;
        reg.emit(ev);
    }
    if (e->killWanted) return;
    e->want = out;
    e->pending = true;
    ++st_.drives;
}

bool EnginePort::applyPose(Entry& e, const live::InterpOut& w, bool force) {
    const RefId r = activeOf(e);
    if (!r) return true;
    const bool snapped = force || w.snapped || w.state == live::InterpState::Snapped;
    const float yaw = w.rot.y;
    if (!snapped && !FNV_MUT(87)) {   // MUT 87 BAD: a copy that is still getting out of its chair is moved (slides across the room seated)
        const Registry& reg = registry();
        if (reg.poseLocked && reg.poseLocked(e.id)) {
            ++st_.drivesHeldSeated;
            if (thr_.allow(2, e.id, static_cast<uint32_t>(now_), 3000))
                nvcoop::actors::trace("port %s: position held (the copy is getting out of furniture); the owner is at (%.0f,%.0f,%.0f); %llu drives held so far", nvcoop::actors::who(e.id).c_str(),
                                      w.pos.x, w.pos.y, w.pos.z, (unsigned long long)st_.drivesHeldSeated);
            return false;
        }
    }
    if (!snapped && !FNV_MUT(15) && e.haveApplied && now_ < e.appliedMs + cfg_.driveMinMs) { ++st_.drivesSkippedRate; return false; }
    if (!snapped && !FNV_MUT(14) && e.haveApplied && w.cell == e.appliedCell) {
        const bool small = std::fabs(w.pos.x - e.appliedPos.x) <= cfg_.posEpsilonXY && std::fabs(w.pos.y - e.appliedPos.y) <= cfg_.posEpsilonXY &&
                           std::fabs(w.pos.z - e.appliedPos.z) <= cfg_.posEpsilonZ && yawDiffDeg(yaw, e.appliedYaw) <= cfg_.yawEpsilonDeg;
        if (small) {
            // "Small" is measured against what THIS port wrote last, not against where the actor is: a copy that something else moved (its own AI for a frame, a
            // furniture exit, a script's MoveTo, a push) while the owner stands still was never put back until the area digest noticed 2 m later. Look where it
            // really is, a few times a second, and write the pose again when it is further from our last write than driftXY (twin 88: never looked).
            if (!FNV_MUT(88) && cfg_.driftCheckMs && now_ >= e.driftCheckMs + cfg_.driftCheckMs) {
                e.driftCheckMs = now_;
                CellId rc = 0; live::Vec3 rp; float rz = 0;
                if (eng_.readPose(r, rc, rp, rz)) {
                    const float dxy = std::sqrt((rp.x - e.appliedPos.x) * (rp.x - e.appliedPos.x) + (rp.y - e.appliedPos.y) * (rp.y - e.appliedPos.y));
                    const float dz = std::fabs(rp.z - e.appliedPos.z);
                    if (dxy > cfg_.driftXY || dz > cfg_.driftZ) {
                        ++st_.driftsFixed;
                        if (thr_.allow(1, e.id, static_cast<uint32_t>(now_), 2000)) nvcoop::actors::trace("port %s was moved by something else: engine at (%.0f,%.0f,%.0f), our last write (%.0f,%.0f,%.0f), wanted (%.0f,%.0f,%.0f): written again", nvcoop::actors::who(e.id).c_str(),
                                              rp.x, rp.y, rp.z, e.appliedPos.x, e.appliedPos.y, e.appliedPos.z, w.pos.x, w.pos.y, w.pos.z);
                        e.haveApplied = false;
                        writePose(e, r, w.cell, w.pos, yaw, false, false);   // (a refused write leaves haveApplied false: the next frame tries again)
                        return true;
                    }
                }
            }
            ++st_.drivesSkippedSmall;
            return true;
        }
    }
    if (!writePose(e, r, w.cell, w.pos, yaw, snapped, false)) return true;   // the ref is gone or not placeable: the next drive() retries
    if (snapped) {
        ++st_.snaps;
        if (thr_.allow(3, e.id, static_cast<uint32_t>(now_), 2000))
            nvcoop::actors::trace("port %s snapped to (%.0f,%.0f,%.0f) cell %08X (%s)", nvcoop::actors::who(e.id).c_str(), w.pos.x, w.pos.y, w.pos.z, w.cell, force ? "forced" : "the stream says teleport");
    }
    return true;
}

bool EnginePort::writePose(Entry& e, RefId r, CellId cell, const live::Vec3& pos, float yaw, bool force, bool allAxes) {
    CellId rawCell = 0;
    live::Vec3 rawPos;
    float rawRotZ = 0;
    if (!eng_.readPose(r, rawCell, rawPos, rawRotZ)) return false;
    if (FNV_MUT(25)) cell = rawCell;   // BAD: the cell is never compared, a cell change becomes a SetPos into the void
    const bool crossed = cell && rawCell && cell != rawCell && !sameSpace(cell, rawCell);
    if (crossed) {
        if (!cellLoaded(cell)) return false;   // nobody here to hold it: the actor stays where it is, the record stays on the network
        if (!eng_.moveToCell(r, cell, pos, yaw)) return false;
        e.haveApplied = true; e.appliedPos = pos; e.appliedYaw = yaw; e.appliedCell = cell; e.appliedMs = now_;
        return true;
    }
    const live::Vec3 base = e.haveApplied ? e.appliedPos : rawPos;
    uint8_t mask = 0;
    if (allAxes || force || std::fabs(pos.x - base.x) > cfg_.posEpsilonXY) mask |= 1;
    if (allAxes || force || std::fabs(pos.y - base.y) > cfg_.posEpsilonXY) mask |= 2;
    if (allAxes || force || std::fabs(pos.z - base.z) > cfg_.posEpsilonZ) mask |= 4;
    bool ok = true;
    if (mask) ok = eng_.setPos(r, pos, mask);
    const float baseYaw = e.haveApplied ? e.appliedYaw : rawRotZ;
    if (allAxes || force || yawDiffDeg(yaw, baseYaw) > cfg_.yawEpsilonDeg) ok = eng_.setYaw(r, yaw) && ok;
    if (!e.haveApplied) { e.appliedPos = rawPos; e.appliedYaw = rawRotZ; }
    if (mask & 1) e.appliedPos.x = pos.x;
    if (mask & 2) e.appliedPos.y = pos.y;
    if (mask & 4) e.appliedPos.z = pos.z;
    e.appliedYaw = yaw;
    e.appliedCell = rawCell ? rawCell : cell;
    e.appliedMs = now_;
    e.haveApplied = true;
    return ok;
}

void EnginePort::applyAvs(Entry& e, RefId r, const std::vector<live::AvValue>& values) {
    for (const live::AvValue& v : values) {
        float target = static_cast<float>(v.value);
        // Health never reaches 0 through a mirrored value: death arrives as its own message (kill()). A health value of 0 on the wire
        // would kill the copy a moment before the owner's death message and look like a local death to the poller.
        if (v.av == kAvHealth && target < 1.0f && !FNV_MUT(23) && !e.killWanted) target = 1.0f;
        float cur = 0;
        if (eng_.actorValue(r, v.av, cur) && !FNV_MUT(24) && std::fabs(cur - target) < 1.0f) continue;   // twin 24: written even when equal
        eng_.forceAv(r, v.av, target);
    }
}

size_t EnginePort::auditAv(NetId id, const std::vector<live::AvValue>& expected) {
    Entry* e = find(id);
    const RefId r = e ? activeOf(*e) : 0;
    if (!r || e->killWanted) return 0;
    size_t fixed = 0;
    for (const live::AvValue& v : expected) {
        if (v.av != kAvHealth) continue;   // one read per copy and second; the limb conditions are written when they change and rarely drift
        float target = static_cast<float>(v.value);
        if (target < 1.0f) target = 1.0f;   // a health under 1 is a dying owner: the copy dies by the death message, never by a mirrored value
        float cur = 0;
        if (!eng_.actorValue(r, v.av, cur) || std::fabs(cur - target) < 1.0f) continue;
        ++st_.avDrifts; st_.lastDriftNet = id; st_.lastDriftCur = cur; st_.lastDriftWant = target;
        if (!FNV_MUT(77) && eng_.forceAv(r, v.av, target)) ++fixed;   // twin 77: the drift is seen and left
    }
    return fixed;
}

void EnginePort::applyAv(NetId id, const std::vector<live::AvValue>& values) {
    Entry* e = find(id);
    const RefId r = e ? activeOf(*e) : 0;
    if (!r || e->killWanted) return;
    applyAvs(*e, r, values);
}

void EnginePort::place(NetId id, const live::ActorSnapshot& s) {
    Entry* e = find(id);
    const RefId r = e ? activeOf(*e) : 0;
    if (!r) return;
    ++st_.places;
    e->pending = false;
    e->lastAi = s.ai;
    e->haveAi = true;
    writePose(*e, r, s.cell, s.pos, s.rot.y, true, true);
    if (!FNV_MUT(26)) applyAvs(*e, r, s.avDeltas);   // twin 26: the actor values of the last state are not applied
}

// ---- IActorPort: AI, protection ------------------------------------------------------------------------------------------------------

void EnginePort::applyAi(Entry& e, bool on) {
    const RefId r = activeOf(e);
    if (!r) return;
    const bool ours = led_.has(LedgerKind::Restrained, r);   // we restrained this ref earlier (also true after the actor was re-bound)
    if (!on) {
        if (e.aiOff && e.aiRef == r) return;
        if (!cfg_.restrainNonOwned) { e.aiOff = true; return; }
        ActorRaw raw;
        raw.wantRestrained = true;
        const bool readOk = eng_.read(r, raw);
        // A dead or dying actor has no AI to stop, and SetRestrained on it overwrites its life state with 5 ("restrained", which every reader
        // counts as ALIVE): in game 2026-10-22 the owner's copy of a ganger read dying (1) and half a second later restrained (5), so the
        // owner showed it alive while every other machine showed it dead (enemy_handoff "alive differs between machines").
        if (readOk && raw.dead() && !FNV_MUT(72)) {   // twin 72: the corpse is restrained like a living NPC
            e.aiOff = true; e.aiRef = 0;
            return;
        }
        const bool scriptRestrained = readOk && raw.restrainedKnown && raw.restrained;
        // a calm actor has no combat to stop: one console line less per actor (every actor is frozen for a moment at track time)
        if (cfg_.stopCombatOnRestrain && (!readOk || raw.inCombat) && !FNV_MUT(11)) eng_.stopCombat(r);
        if (ours) { e.aiOff = true; e.aiRef = r; return; }
        if (scriptRestrained && !FNV_MUT(12)) {
            // a script (or something we cannot see) already holds it: we must not be the one to let it go later
            e.aiOff = true; e.aiRef = 0;
            nvcoop::actors::trace("port %s AI left as it is: a script already holds the actor restrained", nvcoop::actors::who(e.id).c_str());
            return;
        }
        if (eng_.setRestrained(r, true)) {
            if (!FNV_MUT(13)) ledgerAdd(LedgerKind::Restrained, r);
            e.aiOff = true; e.aiRef = r;
            ++st_.aiOff;
            nvcoop::actors::trace("port %s AI off (SetRestrained 1 ran; combat=%d, script-restrained before=%d)", nvcoop::actors::who(e.id).c_str(), readOk && raw.inCombat ? 1 : 0, scriptRestrained ? 1 : 0);
        } else {
            ++st_.aiFailed;
            nvcoop::actors::trace("port %s AI off FAILED (SetRestrained 1 refused): the copy keeps running its own AI", nvcoop::actors::who(e.id).c_str());
        }
        return;
    }
    const bool weRestrained = e.aiRef != 0 || ours;
    e.aiOff = false;
    if (weRestrained || FNV_MUT(27)) {
        const RefId held = e.aiRef ? e.aiRef : r;   // twin 27: unrestrain whatever, even what a script restrained
        if (eng_.setRestrained(held, false)) {
            ledgerRemove(LedgerKind::Restrained, held);
            if (held == r) eng_.evaluatePackage(r);
            e.aiRef = 0;
            ++st_.aiOn;
            nvcoop::actors::trace("port %s AI on (this machine runs the actor now)", nvcoop::actors::who(e.id).c_str());
        } else {
            ++st_.aiFailed;
            nvcoop::actors::trace("port %s AI on FAILED (SetRestrained 0 refused)", nvcoop::actors::who(e.id).c_str());
        }
    }
    if (weRestrained && cfg_.startCombatOnTakeover && !FNV_MUT(28) && e.haveAi && e.lastAi.combatTarget) {
        const RefId t = resolveTarget(e.lastAi.combatTarget);
        if (t && t != r) { eng_.startCombat(r, t); ++st_.combatRestarted; }
        else ++st_.combatTargetUnresolved;
    }
}

void EnginePort::setAi(NetId id, bool on) {
    Entry* e = find(id);
    if (!e) return;
    applyAi(*e, on);
}

void EnginePort::setProtected(NetId id, bool on) {
    Entry* e = find(id);
    const RefId r = e ? activeOf(*e) : 0;
    if (!r) return;
    if (on) {
        if (e->protOn && e->protRef == r) return;
        ActorRaw raw;
        const bool baseEss = eng_.read(r, raw) && raw.baseEssential;
        e->protBaseEssential = baseEss;
        if (!baseEss) {
            if (eng_.setEssential(r, true)) ledgerAdd(LedgerKind::Essential, r);
        }
        e->protOn = true;
        e->protRef = r;
        ++st_.protectOn;
        return;
    }
    if (!e->protOn) return;
    if (!e->protBaseEssential || FNV_MUT(29)) {   // twin 29: a naturally essential actor loses nothing because the flag is cleared anyway... but ours is restored wrongly
        eng_.setEssential(e->protRef, e->protBaseEssential && FNV_MUT(29));
        ledgerRemove(LedgerKind::Essential, e->protRef);
    }
    e->protOn = false;
    e->protRef = 0;
    ++st_.protectOff;
}

// ---- IActorPort: clones ------------------------------------------------------------------------------------------------------------------

bool EnginePort::deleteRef(RefId r) {
    if (FNV_MUT(30)) return true;   // BAD: a removed clone is left standing in the world
    ActorRaw raw;
    if (eng_.read(r, raw) && raw.deleted) return true;   // an earlier, half-finished removal
    const bool a = eng_.setDisabled(r, true);
    const bool b = eng_.markForDelete(r);
    return a && b;
}

bool EnginePort::makeClone(Entry& e, CloneKind kind, uint32_t baseId, RefId anchor, CellId cell, const live::Vec3& pos, float yaw) {
    const RefId c = eng_.placeAtMe(anchor, baseId);
    if (!c) return false;
    e.clone = c;
    e.kind = kind;
    e.cloneBase = baseId;
    index(e.id, c);
    if (!FNV_MUT(31)) ledgerAdd(LedgerKind::StandIn, c);   // twin 31: a clone the ledger does not know survives a save
    e.haveApplied = false;
    writePose(e, c, cell, pos, yaw, true, true);
    return true;
}

// A clone is written to (SetPos) every frame it moves. The engine does not always run such a line at once: in game 2026-10-22 a SetPos on
// a stand-in's id ran after the stand-in was gone, found the id on a NavMesh (the faulting vtable was the NavMesh's) and called through
// text (three client crashes, EIP 6C676E61 = "angl"). So a removed clone is left alone for Config::deleteQuietMs first; the ledger entry
// stays until it is really deleted, so a crash or a save in between still cleans it up.
void EnginePort::retireClone(RefId c, bool immediate) {
    if (!immediate && cfg_.deleteQuietMs && !FNV_MUT(73)) {   // twin 73: deleted at once, with the last SetPos possibly still queued
        if (!isDoomed(c)) doomed_.emplace_back(c, now_ + cfg_.deleteQuietMs);
        return;
    }
    // When the engine refuses (the cell is gone already) the ledger entry stays: sweepOrphans() finishes the job when the ref is reachable.
    if (deleteRef(c)) {
        ledgerRemove(LedgerKind::StandIn, c);
        ledgerRemove(LedgerKind::Restrained, c);
    }
}

bool EnginePort::isDoomed(RefId r) const {
    for (const auto& d : doomed_) if (d.first == r) return true;
    return false;
}

void EnginePort::reapDoomed(bool all) {
    if (doomed_.empty()) return;
    std::vector<RefId> due;
    for (auto it = doomed_.begin(); it != doomed_.end();) {
        if (all || now_ >= it->second) { due.push_back(it->first); it = doomed_.erase(it); }
        else ++it;
    }
    for (RefId r : due) retireClone(r, true);
}

void EnginePort::dropClone(Entry& e, bool reenableReal, bool immediate) {
    const RefId c = e.clone;
    if (c) {
        retireClone(c, immediate);
        unindex(c);
        if (e.aiRef == c) e.aiRef = 0;
        if (e.protRef == c) { ledgerRemove(LedgerKind::Essential, c); e.protOn = false; e.protRef = 0; }
    }
    const bool wasSwap = e.kind == CloneKind::PickSwap;
    e.clone = 0;
    e.kind = CloneKind::None;
    e.cloneBase = 0;
    e.haveApplied = false;
    e.pending = false;
    if (wasSwap && e.real && reenableReal) {
        if (eng_.setDisabled(e.real, false)) ledgerRemove(LedgerKind::Disabled, e.real);
        if (e.aiOff && led_.has(LedgerKind::Restrained, e.real)) e.aiRef = e.real;   // the real actor is still the one we hold
    }
}

bool EnginePort::spawnStandIn(const ActorMeta& meta, const live::ActorSnapshot& at) {
    if (!meta.netId) return false;
    if (cloneHold()) return false;   // a save is being written
    if (cfg_.maxStandIns && standInCount() >= cfg_.maxStandIns && !FNV_MUT(32)) { ++st_.standInsRefused; return false; }   // twin 32: no cap
    Entry* ex = find(meta.netId);
    if (ex && ex->real && !FNV_MUT(33)) return false;   // twin 33: a clone although the real actor is here
    if (ex && ex->clone && ex->kind == CloneKind::StandIn) return true;
    auto retry = spawnRetryAt_.find(meta.netId);
    if (retry != spawnRetryAt_.end() && now_ < retry->second && !FNV_MUT(39)) return false;   // ActorSync asks every tick: back off after a refusal (twin 39: hammers)
    Entry& e = e_[meta.netId];
    e.id = meta.netId;
    e.meta = meta;
    const uint32_t baseId = eng_.fromFormRef(meta.base);
    live::Vec3 apos;
    const RefId anchor = baseId ? eng_.anchorInCell(at.cell, apos) : 0;
    if (!baseId || !anchor || !makeClone(e, CloneKind::StandIn, baseId, anchor, at.cell, at.pos, at.rot.y)) {
        spawnRetryAt_[meta.netId] = now_ + 500;
        eraseIfEmpty(meta.netId);
        return false;
    }
    spawnRetryAt_.erase(meta.netId);
    if (!FNV_MUT(34)) applyAi(e, false);   // twin 34: the clone keeps its own AI and fights the players
    e.lastAi = at.ai;
    e.haveAi = true;
    applyAvs(e, e.clone, at.avDeltas);
    ++st_.standInsMade;
    registry().emit(TrackedEvent{meta.netId, e.clone, meta.ref, meta.base, at.cell, meta.named, meta.created, true});
    return true;
}

void EnginePort::removeStandIn(NetId id) {
    Entry* e = find(id);
    if (!e || e->kind != CloneKind::StandIn || !e->clone) return;
    const RefId c = e->clone;
    dropClone(*e, false);
    ++st_.standInsRemoved;
    registry().emit(LostEvent{id, c, true});
    eraseIfEmpty(id);
}

void EnginePort::setBase(NetId id, const FormRef& pick) {
    Entry* e = find(id);
    if (!e) return;
    if (e->meta.named && !FNV_MUT(35)) return;   // a story NPC is never swapped for another body (twin 35)
    const RefId cur = activeOf(*e);
    if (!cur || !cfg_.swapLeveled || cloneHold()) return;
    const uint32_t pickId = eng_.fromFormRef(pick);
    if (!pickId) { ++st_.baseSwapFailed; return; }
    ActorRaw raw;
    if (!eng_.read(cur, raw)) return;
    if (raw.baseId == pickId && !FNV_MUT(36)) return;   // already that body (twin 36: swaps anyway)
    // Replace the body with a clone of the pick: no verified in-place base change exists (TODO-RE), and rebuilding the 3D of a live actor is
    // exactly what a handoff must avoid. The real actor is disabled while the clone stands in; both are undone together.
    const bool hadRealActive = e->kind == CloneKind::None && e->real;
    const RefId oldClone = e->clone;
    const RefId made = eng_.placeAtMe(cur, pickId);
    if (!made) { ++st_.baseSwapFailed; return; }
    ledgerAdd(LedgerKind::StandIn, made);
    // the pose of what is being replaced
    const live::Vec3 pos = raw.pos;
    const float yaw = raw.rot.z;
    const CellId cell = raw.cell;
    const bool wasOff = e->aiOff;
    if (oldClone) {
        retireClone(oldClone, false);
        unindex(oldClone);
        if (e->aiRef == oldClone) e->aiRef = 0;
    }
    e->clone = made;
    index(id, made);
    if (hadRealActive) {
        e->kind = CloneKind::PickSwap;
        eng_.setDisabled(e->real, true);
        ledgerAdd(LedgerKind::Disabled, e->real);
    } else if (e->kind == CloneKind::None) {
        e->kind = CloneKind::PickSwap;
    }
    e->cloneBase = pickId;
    e->haveApplied = false;
    writePose(*e, made, cell, pos, yaw, true, true);
    if (wasOff) { e->aiOff = false; e->aiRef = 0; applyAi(*e, false); }
    ++st_.baseSwaps;
    registry().emit(TrackedEvent{id, made, e->meta.ref, pick, cell, e->meta.named, e->meta.created, e->kind == CloneKind::StandIn});
}

// ---- IActorPort: death -------------------------------------------------------------------------------------------------------------------

// A copy dies because its owner's machine declared the death. When a REMOTE player's shot did it, the kill is credited to that player's puppet (G04).
void EnginePort::killRef(RefId r) {
    const RefId killer = registry().killerOf && !FNV_MUT(76) ? registry().killerOf(r) : 0;   // twin 76: nobody is ever credited
    if (killer) eng_.killActorBy(r, killer);
    else eng_.killActor(r);
}

void EnginePort::kill(NetId id) {
    Entry* e = find(id);
    const RefId r = e ? activeOf(*e) : 0;
    if (!r) return;
    ++st_.kills;
    e->pending = false;
    e->reviveWanted = false;
    e->killWanted = true;
    e->killTries = 1;
    e->nextTryMs = now_ + 200;
    killRef(r);
    ActorSync* s = registry().sync ? registry().sync() : nullptr;
    DiedEvent d;
    d.netId = id; d.ref = r; d.authorised = true; d.fromNetwork = true;
    d.mine = s && s->ownerOf(id).owner == s->self() && s->ownerOf(id).owner != kNoPlayer;
    registry().emit(d);
}

// ---- IActorPort: inventory ---------------------------------------------------------------------------------------------------------------

bool EnginePort::sampleInventory(NetId id, InvList& out) {
    out.clear();
    Entry* e = find(id);
    if (!e || !activeOf(*e)) return false;
    Registry& reg = registry();
    return reg.invRead && reg.invRead(id, out);
}

bool EnginePort::applyInventory(NetId id, const InvList& want) {
    Entry* e = find(id);
    if (!e || !activeOf(*e) || e->killWanted) return false;
    Registry& reg = registry();
    return reg.invWrite && reg.invWrite(id, want);
}

// ---- IActorPort: equipment ---------------------------------------------------------------------------------------------------------------

bool EnginePort::sampleEquipment(NetId id, EquipList& out) {
    out.clear();
    Entry* e = find(id);
    if (!e || !activeOf(*e)) return false;
    Registry& reg = registry();
    return reg.equipRead && reg.equipRead(id, out);
}

bool EnginePort::applyEquipment(NetId id, const EquipList& want) {
    Entry* e = find(id);
    if (!e || !activeOf(*e) || (e->killWanted && !FNV_MUT(85))) return false;   // twin 85: a dying actor is dressed
    Registry& reg = registry();
    if (FNV_MUT(86)) return true;   // twin 86: reported as written without asking the engine
    return reg.equipWrite && reg.equipWrite(id, want);
}

void EnginePort::revive(NetId id) {
    Entry* e = find(id);
    const RefId r = e ? activeOf(*e) : 0;
    if (!r) return;
    ++st_.revives;
    e->killWanted = false;
    e->reviveWanted = true;
    e->reviveTries = 1;
    e->nextTryMs = now_ + 200;
    e->haveApplied = false;
    eng_.resurrect(r);
}

bool EnginePort::reclaimDisabled(RefId ref) {
    if (!led_.has(LedgerKind::Disabled, ref)) return false;
    const NetId id = netOfRef(ref);
    const Entry* e = id ? find(id) : nullptr;
    if (e && e->kind == CloneKind::PickSwap && e->real == ref) return false;   // a live clone stands in for it
    if (!eng_.setDisabled(ref, false)) return false;
    ledgerRemove(LedgerKind::Disabled, ref);
    return true;
}

// ---- world / session -------------------------------------------------------------------------------------------------------------------------

void EnginePort::forgetWorld() {
    doomed_.clear();   // refs of the old world: nothing may be run on them any more (their ledger entries are undone, or dropped as foreign, by undoStale)
    e_.clear();
    byRef_.clear();
    live_.clear();
    cellCache_.clear();
}

size_t EnginePort::staleCount() const {
    size_t n = 0;
    for (LedgerKind k : {LedgerKind::Restrained, LedgerKind::StandIn, LedgerKind::Disabled, LedgerKind::Essential})
        for (uint32_t r : led_.all(k))
            if (!live_.count({static_cast<char>(k), r})) ++n;
    return n;
}

// A created ref (0xFF......) is not the same object after a save load or in a new process: the engine hands its id to the next
// object it creates. A stale ledger entry for one may therefore name an unrelated live object, and deleting or un-restraining that
// in the middle of a load hung the host (two-copy fight test 2026-10-21). Ours are actors we restrained (stand-ins always are), so
// a created ref that is not a restrained actor now is only dropped from the ledger, never touched.
bool EnginePort::staleCreatedRefIsForeign(LedgerKind k, uint32_t r) {
    if ((r >> 24) != 0xFF || FNV_MUT(67)) return false;   // twin 67: a reused created-ref id is acted on (the hang)
    ActorRaw raw;
    raw.wantRestrained = true;
    if (eng_.read(r, raw) && raw.valid && raw.restrainedKnown && raw.restrained && !raw.isPlayer) return false;
    led_.remove(k, r);
    ++st_.ledgerDropped;
    return true;
}

size_t EnginePort::undoStale() {
    if (FNV_MUT(37)) return 0;   // BAD: what a save, a crash or an earlier session left behind is never undone
    size_t n = 0;
    ActorRaw raw;
    // clones first (a restrained clone needs no separate undo), then the real actors they replaced, then the flags
    for (uint32_t r : led_.all(LedgerKind::StandIn)) {
        if (live_.count({'s', r})) continue;
        if (staleCreatedRefIsForeign(LedgerKind::StandIn, r)) {
            if (led_.has(LedgerKind::Restrained, r)) { led_.remove(LedgerKind::Restrained, r); ++st_.ledgerDropped; }
            continue;
        }
        if (!eng_.read(r, raw)) continue;
        if (!deleteRef(r)) continue;
        led_.remove(LedgerKind::StandIn, r);
        led_.remove(LedgerKind::Restrained, r);
        ++n; ++st_.ledgerReleased;
    }
    for (uint32_t r : led_.all(LedgerKind::Disabled)) {
        if (live_.count({'d', r}) || staleCreatedRefIsForeign(LedgerKind::Disabled, r) || !eng_.read(r, raw)) continue;
        eng_.setDisabled(r, false);
        led_.remove(LedgerKind::Disabled, r);
        ++n; ++st_.ledgerReleased;
    }
    for (uint32_t r : led_.all(LedgerKind::Restrained)) {
        if (live_.count({'r', r}) || staleCreatedRefIsForeign(LedgerKind::Restrained, r) || !eng_.read(r, raw)) continue;
        eng_.setRestrained(r, false);
        eng_.evaluatePackage(r);
        led_.remove(LedgerKind::Restrained, r);
        ++n; ++st_.ledgerReleased;
    }
    for (uint32_t r : led_.all(LedgerKind::Essential)) {
        if (live_.count({'e', r}) || staleCreatedRefIsForeign(LedgerKind::Essential, r) || !eng_.read(r, raw)) continue;
        eng_.setEssential(r, false);
        led_.remove(LedgerKind::Essential, r);
        ++n; ++st_.ledgerReleased;
    }
    return n;
}

// Live clones whose entry is gone (the cell unloaded before the removal could run) are removed as soon as the ref is reachable again.
void EnginePort::sweepOrphans() {
    ActorRaw raw;
    for (uint32_t r : led_.all(LedgerKind::StandIn)) {
        if (!live_.count({'s', r}) || byRef_.count(r) || isDoomed(r)) continue;
        if (!eng_.read(r, raw)) continue;
        if (!deleteRef(r)) continue;
        ledgerRemove(LedgerKind::StandIn, r);
        ledgerRemove(LedgerKind::Restrained, r);
        ++st_.standInsPurged;
    }
}

void EnginePort::releaseAll() {
    ActorRaw raw;
    reapDoomed(true);   // the session is over: what waits for its quiet period goes now
    // every live change, then forget
    for (auto& kv : e_) {
        Entry& e = kv.second;
        if (e.clone) {
            const RefId c = e.clone;
            const bool swap = e.kind == CloneKind::PickSwap;
            if (deleteRef(c)) {
                ledgerRemove(LedgerKind::StandIn, c);
                ledgerRemove(LedgerKind::Restrained, c);
            }
            ledgerRemove(LedgerKind::Essential, c);
            ++st_.standInsPurged;
            if (swap && e.real) { eng_.setDisabled(e.real, false); ledgerRemove(LedgerKind::Disabled, e.real); }
        }
    }
    for (uint32_t r : led_.all(LedgerKind::Restrained)) {
        if (!eng_.read(r, raw)) continue;
        if (!FNV_MUT(38)) { eng_.setRestrained(r, false); eng_.evaluatePackage(r); }   // twin 38: actors stay frozen after the session
        ledgerRemove(LedgerKind::Restrained, r);
        ++st_.aiOn;
    }
    for (uint32_t r : led_.all(LedgerKind::Essential)) {
        if (!eng_.read(r, raw)) continue;
        eng_.setEssential(r, false);
        ledgerRemove(LedgerKind::Essential, r);
        ++st_.protectOff;
    }
    for (uint32_t r : led_.all(LedgerKind::Disabled)) {
        if (!eng_.read(r, raw)) continue;
        eng_.setDisabled(r, false);
        ledgerRemove(LedgerKind::Disabled, r);
    }
    forgetWorld();
    if (cfg_.persistLedger) led_.flush(now_, true);
}

std::vector<NetId> EnginePort::beforeSave() {
    std::vector<NetId> reverted;
    cloneHoldUntilMs_ = now_ + cfg_.saveGraceMs;
    reapDoomed(true);   // the save must not contain a clone, quiet period or not
    for (auto& kv : e_) {
        Entry& e = kv.second;
        if (!e.clone) continue;
        const bool swap = e.kind == CloneKind::PickSwap;
        dropClone(e, true, true);
        ++st_.standInsPurged;
        if (swap) reverted.push_back(kv.first);
    }
    if (cfg_.persistLedger) led_.flush(now_, true);
    return reverted;
}

}  // namespace nvcoop::actors::engine
