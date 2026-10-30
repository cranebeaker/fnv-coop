// The arithmetic of the speaker mix, game-free and codec-free: how loud a voice is on each side, how a voice's gains glide to a new
// target, how the voices add up on a float bus, and how the bus is brought back to 16 bit without ever wrapping. voice_rx.cpp
// decides WHO is heard and how loud (placement, mute); this file only does the sums.
#pragma once
#include <cstddef>
#include <cstdint>
#include <vector>

namespace nvcoop::voice {

// Left and right gain for a voice with overall `gain` (0..1) and `pan` (-1 left .. 1 right): equal power, so a voice that sweeps from
// one ear to the other keeps the same loudness (in the middle both sides are 0.707 of the gain).
void panGains(float gain, float pan, float& left, float& right);

// The gains a voice is being played with right now. They glide towards the target instead of jumping.
struct VoiceGain {
    float l = 0, r = 0;
};

// Per-sample coefficient of a one-pole glide whose time constant is `ms` at `rate` Hz.
float glideCoefficient(float ms, float rate);

// Samples decoded but not played yet, in order. Cheap at both ends (no erase from the front on every block).
class SampleFifo {
public:
    void push(const int16_t* samples, size_t n);
    size_t size() const { return data_.size() - head_; }
    // Moves up to `n` samples into `out`; returns how many were there.
    size_t take(int16_t* out, size_t n);
    void clear() { data_.clear(); head_ = 0; }

private:
    std::vector<int16_t> data_;
    size_t head_ = 0;
};

// The float accumulator the voices are added to: interleaved left / right.
class MixBus {
public:
    // Starts a block of `frames` stereo frames, all silent. Reuses its memory (no allocation once it has been this large).
    void begin(size_t frames);
    // Adds a mono voice. `voice` has `have` samples (fewer than the block: the rest of the block is silent for this voice).
    // The voice's gains in `g` move towards (targetL, targetR) with coefficient `glide` a sample at a time.
    void addVoice(const int16_t* voice, size_t have, VoiceGain& g, float targetL, float targetR, float glide);
    // Converts the block to 16 bit through the limiter. `out` has frames * 2 samples.
    void finish(int16_t* out) const;
    size_t frames() const { return frames_; }
    const float* data() const { return sum_.data(); }

private:
    std::vector<float> sum_;
    size_t frames_ = 0;
};

// Soft limiter: untouched up to about 70% of full scale, then a smooth bend that reaches exactly full scale (never beyond), the same
// for both signs.
float softLimit(float x);
// Limiter plus rounding to 16 bit.
int16_t toPcm16(float x);

}  // namespace nvcoop::voice
