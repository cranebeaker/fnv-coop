#include "rv_hold.h"

#include <algorithm>

namespace nvmk {

void ReviveHold::reset() {
    target_ = kNoSlot;
    candidate_ = kNoSlot;
    haveCooldown_ = false;
}

float ReviveHold::progress(uint64_t now) const {
    if (target_ == kNoSlot || cfg_.holdMs == 0) return 0.0f;
    return std::min(1.0f, static_cast<float>(now - startMs_) / static_cast<float>(cfg_.holdMs));
}

HoldOut ReviveHold::step(const HoldInput& in) {
    HoldOut out;
    // the nearest friend who can be revived and is in reach
    candidate_ = kNoSlot;
    float best = 0;
    for (const HoldCandidate& c : in.candidates) {
        if (!c.revivable || c.busyWithOther) continue;
        if (c.distUnits > cfg_.reachUnits && !FNV_MUT(1383)) continue;                       // twin 1383: a downed friend anywhere in view can be revived
        if (candidate_ == kNoSlot || c.distUnits < best) { candidate_ = c.slot; best = c.distUnits; }
    }
    const bool cooling = haveCooldown_ && in.now < cooldownUntil_ && !FNV_MUT(1385);         // twin 1385: no pause after a revive, so one hold revives twice
    const bool want = in.activateHeld && !in.blocked && (in.selfUp || FNV_MUT(1384)) && candidate_ != kNoSlot && !cooling;   // twin 1384: a downed player can revive
    if (!want) {
        if (target_ != kNoSlot) {
            out = {HoldEvent::Cancel, target_};
            target_ = kNoSlot;
        }
        return out;
    }
    if (target_ == kNoSlot) {
        target_ = candidate_;
        startMs_ = in.now;
        out = {HoldEvent::Begin, target_};
        return out;
    }
    if (target_ != candidate_) {                                                              // somebody nearer: the hold starts over on him
        out = {HoldEvent::Cancel, target_};
        target_ = kNoSlot;
        return out;
    }
    if (in.now - startMs_ >= cfg_.holdMs || FNV_MUT(1386)) {                                  // twin 1386: done as soon as it began
        out = {HoldEvent::Done, target_};
        target_ = kNoSlot;
        cooldownUntil_ = in.now + cfg_.cooldownMs;
        haveCooldown_ = true;
    }
    return out;
}

}  // namespace nvmk
