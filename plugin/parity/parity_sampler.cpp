#include "parity/parity_sampler.h"

#include <algorithm>
#include <cmath>
#include <set>

#include "parity/parity_anim.h"
#include "parity/parity_compare.h"
#include "fnvcoop_mut.h"

// Twins: 1812 = the actor cap is not applied; 1813 = the list is farthest first; 1815 (file) is in parity_file.cpp; 1816 = an attack counts on every frame it is
//        playing, not once when it starts; 1818 (service) is in parity_service.cpp; 1822 = the speed is measured against the previous sample however close in time
//        (a sample taken a few milliseconds after the last one reads speed 0); 1823 = a puppet's own reference is listed as an actor too
namespace nvcoop::parity {

namespace {
constexpr uint64_t kMinSpeedWindowMs = 700, kMaxSpeedWindowMs = 8000;
constexpr uint64_t kFrameEveryMs = 33;
}  // namespace

uint8_t Sampler::speedOf(uint32_t key, float x, float y, float z, uint64_t nowMs) {
    std::deque<Pt>& h = history_[key];
    uint8_t speed = 0;
    for (auto it = h.rbegin(); it != h.rend(); ++it) {   // newest first: the most recent point at least kMinSpeedWindowMs old
        const uint64_t dt = nowMs - it->ms;
        if ((dt >= kMinSpeedWindowMs || FNV_MUT(1822)) && dt <= kMaxSpeedWindowMs && dt > 0) {
            const double dx = double(x) - it->x, dy = double(y) - it->y, dz = double(z) - it->z;
            const double v = std::sqrt(dx * dx + dy * dy + dz * dz) / (double(dt) / 1000.0);
            speed = static_cast<uint8_t>(std::min(255.0, std::floor(v / 16.0 + 0.5)));
            break;
        }
    }
    if (h.empty() || nowMs - h.back().ms >= 200) h.push_back(Pt{x, y, z, nowMs});
    while (h.size() > 6) h.pop_front();
    return speed;
}

void Sampler::count(uint8_t slot, const std::vector<int>& groups) {
    Track& t = tracks_[slot];
    uint8_t now[256] = {};
    for (int g : groups) {
        if (!isAttackStartGroup(g)) continue;
        now[g] = 1;
        if (!t.prev[g] || FNV_MUT(1816)) ++t.count;
    }
    std::copy(now, now + 256, t.prev);
}

uint32_t Sampler::attacksOf(uint8_t slot) const {
    auto it = tracks_.find(slot);
    return it == tracks_.end() ? 0 : it->second.count;
}

void Sampler::frame(uint64_t nowMs) {
    if (nowMs - lastFrameMs_ < kFrameEveryMs) return;
    lastFrameMs_ = nowMs;
    if (!src_.ready()) return;
    std::vector<int> g;
    if (src_.groups(src_.playerRef(), true, g)) count(src_.localSlot(), g);
    std::vector<std::pair<uint8_t, uint32_t>> pup;
    src_.puppets(pup);
    for (const auto& p : pup) {
        g.clear();
        if (src_.groups(p.second, false, g)) count(p.first, g);
    }
}

bool Sampler::take(uint64_t nowMs, uint64_t wallMs, Sample& out) {
    RawPlayer pl;
    if (!src_.ready() || !src_.player(pl)) return false;
    Sample s;
    s.slot = src_.localSlot();
    s.seq = ++seq_;
    s.ms = nowMs;
    s.wallMs = wallMs;
    s.gameTime = src_.gameTime();
    s.cell = pl.cell;
    s.ws = pl.ws;
    s.rad = set_.radius;
    s.cov = set_.radius;
    s.menu = pl.menu;

    std::vector<std::pair<uint8_t, uint32_t>> pup;
    src_.puppets(pup);
    std::set<uint32_t> skip;
    skip.insert(src_.playerRef());
    if (!FNV_MUT(1823)) for (const auto& p : pup) skip.insert(p.second);

    std::vector<RawActor> all;
    src_.actors(all);
    struct Cand { RawActor a; double d; };
    std::vector<Cand> cands;
    for (const RawActor& a : all) {
        if (skip.count(a.ref)) continue;
        const double dx = double(a.x) - pl.x, dy = double(a.y) - pl.y, dz = double(a.z) - pl.z;
        const double d = std::sqrt(dx * dx + dy * dy + dz * dz);
        if (d > double(set_.radius)) continue;
        cands.push_back(Cand{a, d});
    }
    std::sort(cands.begin(), cands.end(), [](const Cand& a, const Cand& b) { return a.d != b.d ? a.d < b.d : a.a.ref < b.a.ref; });
    if (FNV_MUT(1813)) std::reverse(cands.begin(), cands.end());
    if (cands.size() > set_.cap && !FNV_MUT(1812)) {
        s.cov = static_cast<uint32_t>(cands[set_.cap].d);   // complete only as far as the nearest actor that did not fit
        cands.resize(set_.cap);
    }
    std::set<uint32_t> seen;
    auto fill = [&](Ent& e, const RawActor& a, bool isPlayer, uint32_t key) {
        e.ref = a.ref;
        e.base = a.base;
        e.x = static_cast<int32_t>(std::lround(a.x));
        e.y = static_cast<int32_t>(std::lround(a.y));
        e.z = static_cast<int32_t>(std::lround(a.z));
        e.disabled = a.disabled;
        e.speed16 = speedOf(key, a.x, a.y, a.z, nowMs);
        seen.insert(key);
        RawState st;
        if (src_.state(a.ref, isPlayer, st) && st.ok) {
            e.seated = st.seatedOk && st.seated;
            e.dead = st.dead;
            e.weaponOut = st.weaponOut;
            e.hp = hpBucket(st.hpOk, st.hp);
            e.group = primaryGroup(st.groups);
        } else if (st.hpOk || st.dead) {
            e.dead = st.dead;
            e.hp = hpBucket(st.hpOk, st.hp);
        }
    };
    for (const Cand& c : cands) {
        Ent e;
        e.kind = EntKind::Npc;
        fill(e, c.a, false, c.a.ref);
        s.ents.push_back(e);
    }
    for (const auto& p : pup) {
        RawActor a;
        if (!src_.actor(p.second, a)) continue;
        Ent e;
        e.kind = EntKind::Puppet;
        e.slot = p.first;
        fill(e, a, false, 0xFE000000u | p.first);
        e.acts = attacksOf(p.first);
        RawLook lk;
        if (src_.look(p.second, lk) && lk.ok) { e.look = lk.look; e.look.known = true; }
        s.ents.push_back(e);
    }
    {
        RawActor a;
        a.ref = src_.playerRef();
        a.x = pl.x; a.y = pl.y; a.z = pl.z;
        Ent e;
        e.kind = EntKind::Self;
        e.slot = s.slot;
        fill(e, a, true, 0xFE000000u | s.slot);
        e.acts = attacksOf(s.slot);
        RawLook lk;
        if (src_.look(a.ref, lk) && lk.ok) { e.look = lk.look; e.look.known = true; }
        s.ents.push_back(e);
    }
    for (auto it = history_.begin(); it != history_.end();) {
        if (!seen.count(it->first)) it = history_.erase(it);
        else ++it;
    }
    out = std::move(s);
    return true;
}

Sample netSample(const Sample& s, size_t netCap) {
    Sample o = s;
    o.ents.clear();
    const Ent* self = s.bySlot(s.slot);
    size_t npcs = 0;
    bool cut = false;
    for (const Ent& e : s.ents) {
        if (e.kind != EntKind::Npc) continue;
        if (npcs >= netCap) {
            if (!cut && self) o.cov = std::min<uint32_t>(o.cov, static_cast<uint32_t>(dist3(e, *self)));
            else if (!cut) o.cov = 0;
            cut = true;
            continue;
        }
        ++npcs;
        o.ents.push_back(e);
    }
    for (const Ent& e : s.ents) if (e.kind != EntKind::Npc) o.ents.push_back(e);
    return o;
}

}  // namespace nvcoop::parity
