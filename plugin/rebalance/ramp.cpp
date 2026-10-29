#include "rebalance/ramp.h"

namespace nvcoop::rebalance {

void ProfileRamp::setTarget(const Profile& target, uint64_t nowMs) {
    if (target == target_) return;
    from_ = current_;
    target_ = target;
    startedAt_ = nowMs;
}

const Profile& ProfileRamp::step(uint64_t nowMs) {
    if (current_ == target_) return current_;
    float t = nowMs <= startedAt_ ? 0.0f : static_cast<float>(nowMs - startedAt_) / static_cast<float>(rampMs_);
    if (FNV_MUT(1220)) t = 1.0f;   // MUTANT 1220: no ramp, the numbers jump
    if (t >= 1.0f) { current_ = target_; return current_; }
    const float smooth = t * t * (3.0f - 2.0f * t);
    current_ = mix(from_, target_, smooth);
    return current_;
}

}  // namespace nvcoop::rebalance
