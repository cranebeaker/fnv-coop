#include "rv_authority.h"

namespace nvmk {

const char* verdictName(Verdict v) {
    switch (v) {
        case Verdict::Ok: return "ok";
        case Verdict::Disabled: return "disabled";
        case Verdict::UnknownPlayer: return "unknown player";
        case Verdict::SelfRevive: return "self revive";
        case Verdict::TargetNotDown: return "target not down";
        case Verdict::NotRevivable: return "not revivable";
        case Verdict::WindowOver: return "window over";
        case Verdict::ReviverDown: return "reviver down";
        case Verdict::OtherSpace: return "other space";
        case Verdict::TooFar: return "too far";
        case Verdict::Busy: return "busy";
        case Verdict::NotHolding: return "not holding";
        case Verdict::TooEarly: return "too early";
    }
    return "?";
}

void ReviveAuthority::sync(Slot s, bool downed, bool checkpointOnly, uint32_t windowMs, uint64_t now) {
    known_[s] = downed;
    auto it = down_.find(s);
    if (!downed) {
        if (it != down_.end()) down_.erase(it);
        return;
    }
    if (it == down_.end()) {
        Down d;
        d.since = now;
        d.windowMs = windowMs;
        d.checkpointOnly = checkpointOnly;
        down_[s] = d;
        return;
    }
    if (FNV_MUT(1366)) it->second.since = now;   // twin 1366: the window restarts every frame, so a downed player never runs out of time
    it->second.windowMs = windowMs;
    it->second.checkpointOnly = checkpointOnly;
}

void ReviveAuthority::forget(Slot s) {
    known_.erase(s);
    down_.erase(s);
    for (auto& kv : down_)
        if (kv.second.reviver == s) kv.second.reviver = kNoSlot;
}

void ReviveAuthority::clear() {
    known_.clear();
    down_.clear();
    revived_.clear();
}

bool ReviveAuthority::checkpointOnly(Slot s) const {
    auto it = down_.find(s);
    return it != down_.end() && it->second.checkpointOnly;
}

uint32_t ReviveAuthority::windowLeftMs(Slot s, uint64_t now) const {
    auto it = down_.find(s);
    if (it == down_.end() || it->second.checkpointOnly) return 0;
    const uint64_t end = it->second.since + it->second.windowMs;
    return now >= end ? 0u : static_cast<uint32_t>(end - now);
}

Slot ReviveAuthority::reviverOf(Slot s) const {
    auto it = down_.find(s);
    return it == down_.end() ? kNoSlot : it->second.reviver;
}

uint32_t ReviveAuthority::holdLeftMs(Slot s, uint64_t now) const {
    auto it = down_.find(s);
    if (it == down_.end() || it->second.reviver == kNoSlot) return 0;
    const uint64_t end = it->second.holdSince + rules_.holdMs;
    return now >= end ? 0u : static_cast<uint32_t>(end - now);
}

Verdict ReviveAuthority::check(Slot from, Slot target, uint64_t now, const PlaceFn& where) const {
    if (!rules_.enabled && !FNV_MUT(1351)) return Verdict::Disabled;                         // twin 1351: the off switch is ignored
    if (from == target && !FNV_MUT(1352)) return Verdict::SelfRevive;                       // twin 1352: a player revives himself
    auto f = known_.find(from);
    auto t = known_.find(target);
    if (f == known_.end() || t == known_.end()) return Verdict::UnknownPlayer;
    auto d = down_.find(target);
    if (d == down_.end() || !t->second) {
        if (!FNV_MUT(1353)) return Verdict::TargetNotDown;                                   // twin 1353: anybody can be "revived"
    }
    const bool ckpt = d != down_.end() && (d->second.checkpointOnly || d->second.windowMs == 0);
    if (ckpt && !FNV_MUT(1360)) return Verdict::NotRevivable;                                 // twin 1360: a collar death is revivable
    if (d != down_.end() && !d->second.checkpointOnly && d->second.windowMs > 0 && now >= d->second.since + d->second.windowMs && !FNV_MUT(1361))
        return Verdict::WindowOver;                                                           // twin 1361: revived after the respawn is due
    if (f->second && !FNV_MUT(1354)) return Verdict::ReviverDown;                           // twin 1354: a downed player revives
    Place pf, pt;
    if (!where || !where(from, pf) || !where(target, pt) || !pf.valid || !pt.valid) return Verdict::UnknownPlayer;
    if (!sameSpace(pf, pt) && !FNV_MUT(1356)) return Verdict::OtherSpace;                   // twin 1356: through a wall, a door, a loading screen
    if (sameSpace(pf, pt) && distance3(pf, pt) > rules_.hostReachUnits && !FNV_MUT(1355)) return Verdict::TooFar;   // twin 1355: from across the map
    return Verdict::Ok;
}

Verdict ReviveAuthority::request(Slot from, RevivePhase ph, Slot target, uint64_t now, const PlaceFn& where) {
    if (ph == RevivePhase::Cancel) {
        auto d = down_.find(target);
        if (d == down_.end() || (d->second.reviver != from && !FNV_MUT(1367))) return Verdict::NotHolding;   // twin 1367: anybody can stop somebody else's revive
        d->second.reviver = kNoSlot;
        return Verdict::Ok;
    }
    if (ph == RevivePhase::Begin) {
        const Verdict v = check(from, target, now, where);
        if (v != Verdict::Ok) return v;
        Down& d = down_[target];
        if (d.reviver != kNoSlot && d.reviver != from && !FNV_MUT(1357)) return Verdict::Busy;   // twin 1357: two friends on one downed player
        if (!FNV_MUT(1364))
            for (auto& kv : down_)
                if (kv.first != target && kv.second.reviver == from) kv.second.reviver = kNoSlot;   // twin 1364: one friend, two downed players at once
        d.reviver = from;
        d.holdSince = now;
        return Verdict::Ok;
    }
    // done
    auto d = down_.find(target);
    if (d == down_.end()) return Verdict::TargetNotDown;
    if (d->second.reviver != from && !FNV_MUT(1358)) return Verdict::NotHolding;             // twin 1358: a done with no begin gets him up
    const uint64_t held = now - d->second.holdSince;
    if (held + rules_.slackMs < rules_.holdMs && !FNV_MUT(1359)) return Verdict::TooEarly;   // twin 1359: no time needs to pass
    const Verdict v = check(from, target, now, where);
    if (v != Verdict::Ok) {
        if (d->second.reviver == from) d->second.reviver = kNoSlot;
        return v;
    }
    if (!FNV_MUT(1365)) revived_.push_back({target, from});                                   // twin 1365: the service is never told
    if (!FNV_MUT(1368)) {                                                                    // twin 1368: he stays on the downed list
        down_.erase(d);
        known_[target] = false;
    }
    return Verdict::Ok;
}

void ReviveAuthority::tick(uint64_t now, const PlaceFn& where) {
    for (auto& kv : down_) {
        Down& d = kv.second;
        if (d.reviver == kNoSlot) continue;
        bool drop = false;
        auto r = known_.find(d.reviver);
        if (r == known_.end()) drop = true;
        else if (r->second && !FNV_MUT(1362)) drop = true;                                    // twin 1362: a reviver who went down keeps the hold
        if (!drop && now - d.holdSince > rules_.holdTtlMs && !FNV_MUT(1363)) drop = true;    // twin 1363: a begin without an end blocks the target for good
        if (!drop) {
            Place pf, pt;
            if (where && where(d.reviver, pf) && where(kv.first, pt) && pf.valid && pt.valid &&
                (!sameSpace(pf, pt) || distance3(pf, pt) > rules_.hostReachUnits * 1.5f))
                drop = true;
        }
        if (drop) d.reviver = kNoSlot;
    }
}

std::vector<ReviveAuthority::Revived> ReviveAuthority::takeRevived() {
    std::vector<Revived> out;
    out.swap(revived_);
    return out;
}

}  // namespace nvmk
