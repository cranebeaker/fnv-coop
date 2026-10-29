// The smooth change of the profile: when the party size (or the mode) changes the numbers do not jump, they move to the new target over a few seconds, so a
// player who joins mid-fight does not make every enemy suddenly tougher. Game-free; the clock comes in as a parameter.
#pragma once
#include <cstdint>

#include "rebalance/balance.h"

namespace nvcoop::rebalance {

class ProfileRamp {
public:
    explicit ProfileRamp(uint32_t rampMs = 6000) : rampMs_(rampMs ? rampMs : 1) {}
    // The profile the numbers are heading to. A new target starts a new move from wherever the numbers are now; the same target again changes nothing.
    void setTarget(const Profile& target, uint64_t nowMs);
    // The profile at `nowMs`.
    const Profile& step(uint64_t nowMs);
    const Profile& current() const { return current_; }
    const Profile& target() const { return target_; }
    bool settled() const { return current_ == target_; }
    // Jump straight to the target (a game that is loading: nobody sees a ramp).
    void snap() { current_ = target_; from_ = target_; }
private:
    uint32_t rampMs_;
    Profile current_, from_, target_;
    uint64_t startedAt_ = 0;
};

}  // namespace nvcoop::rebalance
