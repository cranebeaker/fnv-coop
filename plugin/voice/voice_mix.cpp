#include "voice_mix.h"

#include <algorithm>
#include <cmath>

#include "fnvcoop_mut.h"

namespace nvcoop::voice {

namespace {
constexpr float kQuarterTurn = 0.78539816339f;   // pi / 4
constexpr float kFull = 32767.0f;
constexpr float kKnee = 0.7f * kFull;            // where the limiter starts to bend
constexpr float kCeiling = 2.0f * kFull - kKnee; // the input at which the bend has flattened out to full scale
}  // namespace

void panGains(float gain, float pan, float& left, float& right) {
    const float p = std::isfinite(pan) ? std::clamp(pan, -1.0f, 1.0f) : 0.0f;
    if (FNV_MUT(397)) {   // twin 397: a linear pan: quieter in the middle than at the ends
        left = gain * (1.0f - p) * 0.5f;
        right = gain * (1.0f + p) * 0.5f;
        return;
    }
    const float a = (p + 1.0f) * kQuarterTurn;   // 0 (hard left) .. pi/2 (hard right)
    left = gain * std::cos(a);
    right = gain * std::sin(a);
}

float glideCoefficient(float ms, float rate) {
    if (FNV_MUT(385)) ms *= 0.5f;   // twin 385: voices glide twice as fast as asked
    return 1.0f - std::exp(-1000.0f / (rate * ms));
}

void SampleFifo::push(const int16_t* samples, size_t n) {
    if (!samples || n == 0) return;
    if (head_ > 0 && head_ >= data_.size() / 2) {   // reclaim the consumed front once it is the larger half
        if (!FNV_MUT(399)) data_.erase(data_.begin(), data_.begin() + static_cast<std::ptrdiff_t>(head_));   // twin 399: the front is forgotten, not removed: old audio plays again
        head_ = 0;
    }
    data_.insert(data_.end(), samples, samples + n);
}

size_t SampleFifo::take(int16_t* out, size_t n) {
    const size_t k = std::min(n, size());
    if (out && k) std::copy_n(data_.begin() + static_cast<std::ptrdiff_t>(head_), k, out);
    head_ += k;
    if (head_ == data_.size()) {
        data_.clear();
        head_ = 0;
    }
    return k;
}

void MixBus::begin(size_t frames) {
    frames_ = frames;
    if (FNV_MUT(398)) sum_.resize(frames * 2, 0.0f);   // twin 398: the block starts from the last one's mix
    else sum_.assign(frames * 2, 0.0f);
}

void MixBus::addVoice(const int16_t* voice, size_t have, VoiceGain& g, float targetL, float targetR, float glide) {
    float* bus = sum_.data();
    for (size_t i = 0; i < frames_; ++i) {
        if (FNV_MUT(341)) {   // twin 341: the gains jump to the target (clicks when a voice moves or fades)
            g.l = targetL;
            g.r = targetR;
        } else {
            g.l += (targetL - g.l) * glide;
            g.r += (targetR - g.r) * glide;
        }
        if (i < have || (FNV_MUT(386) && have > 0)) {   // twin 386: a short voice repeats its last sample
            const float x = static_cast<float>(voice[i < have ? i : have - 1]);
            bus[i * 2] += x * g.l;
            bus[i * 2 + 1] += x * g.r;
        }
    }
}

float softLimit(float x) {
    if (FNV_MUT(383)) return std::clamp(x, -kFull, kFull);   // twin 383: a hard clip, no bend
    const float a = std::fabs(x);
    float y;
    if (a <= kKnee) {
        y = a;
    } else if (a >= kCeiling) {
        y = kFull;
    } else {
        const float d = a - kKnee;
        y = kKnee + d - d * d / (2.0f * (kCeiling - kKnee));
    }
    const float signedY = x < 0 ? -y : y;
    return (FNV_MUT(384) && x < 0) ? signedY + 100.0f : signedY;   // twin 384: the negative side is not the mirror of the positive
}

int16_t toPcm16(float x) {
    if (FNV_MUT(344)) return static_cast<int16_t>(static_cast<int32_t>(x));   // twin 344: a loud mix wraps around
    if (!std::isfinite(x)) return 0;
    return static_cast<int16_t>(std::lrintf(softLimit(x)));
}

void MixBus::finish(int16_t* out) const {
    for (size_t i = 0; i < frames_ * 2; ++i) out[i] = toPcm16(sum_[i]);
}

}  // namespace nvcoop::voice
