#include "voice_resample.h"

#include <algorithm>
#include <cmath>
#include <cstring>

#include "fnvcoop_mut.h"

namespace nvcoop::voice {

namespace {
constexpr double kPi = 3.14159265358979323846;
constexpr int kTableRes = 256;   // kernel table entries per input sample of distance
constexpr int kLobes = 10;       // sinc zero crossings kept on each side

uint32_t gcd32(uint32_t a, uint32_t b) {
    while (b) {
        const uint32_t t = a % b;
        a = b;
        b = t;
    }
    return a ? a : 1;
}

float clampTo(float v, float lo, float hi) { return v < lo ? lo : v > hi ? hi : v; }
}  // namespace

Resampler::Resampler(uint32_t inRate, uint32_t outRate) : inStep_(inRate ? inRate : 1), outStep_(outRate ? outRate : 1), passthrough_(false) {
    const uint32_t g = gcd32(inStep_, outStep_);
    inStep_ /= g;    // only the ratio matters; small numbers keep the position counter small
    outStep_ /= g;
    passthrough_ = inStep_ == outStep_;
    build();
}

void Resampler::build() {
    if (passthrough_) return;
    // 92% of the lower Nyquist rate: speech is far below it, and the transition band fits inside the kernel.
    cutoff_ = (FNV_MUT(394) ? 1.0f : std::min(1.0f, static_cast<float>(outStep_) / static_cast<float>(inStep_))) * 0.92f;   // twin 394: no anti-alias filter when going down
    half_ = static_cast<int>(std::ceil(kLobes / cutoff_));
    const int n = half_ * kTableRes + 2;
    table_.assign(static_cast<size_t>(n), 0.0f);
    for (int i = 0; i < n; ++i) {
        const double d = static_cast<double>(i) / kTableRes;   // distance in input samples
        const double x = kPi * cutoff_ * d;
        const double sinc = d == 0.0 ? 1.0 : std::sin(x) / x;
        const double t = d / half_;                            // 0 .. 1 across the half kernel
        const double window = t >= 1.0 ? 0.0 : 0.42 + 0.5 * std::cos(kPi * t) + 0.08 * std::cos(2 * kPi * t);   // Blackman
        table_[static_cast<size_t>(i)] = static_cast<float>(cutoff_ * sinc * window);
    }
    reset();
}

float Resampler::kernel(float distance) const {
    const float f = std::fabs(distance) * kTableRes;
    const int i = static_cast<int>(f);
    if (i + 1 >= static_cast<int>(table_.size())) return 0.0f;
    const float frac = f - static_cast<float>(i);
    return table_[static_cast<size_t>(i)] * (1.0f - frac) + table_[static_cast<size_t>(i) + 1] * frac;
}

void Resampler::reset() {
    buf_.clear();
    if (passthrough_) return;
    buf_.assign(static_cast<size_t>(half_), 0.0f);   // the stream starts after `half_` samples of silence
    pos_ = static_cast<int64_t>(half_) * outStep_;
}

void Resampler::process(const float* in, size_t n, std::vector<float>& out) {
    if (!in || n == 0) return;
    if (passthrough_) {
        out.insert(out.end(), in, in + n);
        return;
    }
    if (FNV_MUT(395)) reset();   // twin 395: every chunk starts a new stream
    buf_.insert(buf_.end(), in, in + n);
    const int64_t size = static_cast<int64_t>(buf_.size());
    for (;;) {
        const int64_t base = pos_ / outStep_;   // the input sample at or just before the output instant
        if (base + half_ >= size) break;        // the kernel reaches input that has not arrived yet
        const float frac = static_cast<float>(pos_ % outStep_) / static_cast<float>(outStep_);
        float acc = 0.0f;
        for (int k = -half_ + 1; k <= half_; ++k) {
            const float d = static_cast<float>(k) - frac;   // distance from the output instant to input sample base+k
            acc += buf_[static_cast<size_t>(base + k)] * kernel(d);
        }
        out.push_back(FNV_MUT(391) ? acc * 0.5f : acc);   // twin 391: the resampler halves the level
        pos_ += inStep_;
    }
    // Throw away input the kernel will never reach again.
    const int64_t keepFrom = pos_ / outStep_ - half_ + 1;
    if (keepFrom > 0) {
        buf_.erase(buf_.begin(), buf_.begin() + static_cast<std::ptrdiff_t>(keepFrom));
        pos_ -= keepFrom * outStep_;
    }
}

namespace {
float readSample(const uint8_t* p, SampleKind kind) {
    switch (kind) {
    case SampleKind::Int16: {
        int16_t v;
        std::memcpy(&v, p, 2);
        return static_cast<float>(v);
    }
    case SampleKind::Int24: {
        const int32_t v = static_cast<int32_t>((static_cast<uint32_t>(p[2]) << 24) | (static_cast<uint32_t>(p[1]) << 16) | (static_cast<uint32_t>(p[0]) << 8));
        return static_cast<float>(v >> 8) / 256.0f;
    }
    case SampleKind::Int32: {
        int32_t v;
        std::memcpy(&v, p, 4);
        return static_cast<float>(v) / 65536.0f;
    }
    case SampleKind::Float32: {
        float v;
        std::memcpy(&v, p, 4);
        return std::isfinite(v) ? v * 32768.0f : 0.0f;
    }
    }
    return 0.0f;
}

void writeSample(uint8_t* p, SampleKind kind, float v) {   // v on the 16 bit scale
    switch (kind) {
    case SampleKind::Int16: {
        const int16_t s = static_cast<int16_t>(std::lrintf(clampTo(v, -32768.0f, 32767.0f)));
        std::memcpy(p, &s, 2);
        break;
    }
    case SampleKind::Int24: {
        const int32_t s = static_cast<int32_t>(std::lrintf(clampTo(v * 256.0f, -8388608.0f, 8388607.0f)));
        p[0] = static_cast<uint8_t>(s & 0xFF);
        p[1] = static_cast<uint8_t>((s >> 8) & 0xFF);
        p[2] = static_cast<uint8_t>((s >> 16) & 0xFF);
        break;
    }
    case SampleKind::Int32: {
        const double d = static_cast<double>(clampTo(v, -32768.0f, 32767.0f)) * 65536.0;
        const int32_t s = static_cast<int32_t>(std::lrint(d));
        std::memcpy(p, &s, 4);
        break;
    }
    case SampleKind::Float32: {
        const float s = clampTo(v, -32768.0f, 32768.0f) / 32768.0f;
        std::memcpy(p, &s, 4);
        break;
    }
    }
}
}  // namespace

void deviceToMono(const uint8_t* src, size_t frames, const DeviceFormat& fmt, std::vector<float>& out) {
    if (!src || frames == 0 || fmt.channels == 0) return;
    const size_t bps = fmt.bytesPerSample();
    for (size_t f = 0; f < frames; ++f) {
        const uint8_t* frame = src + f * fmt.bytesPerFrame();
        float sum = 0.0f;
        for (uint32_t c = 0; c < fmt.channels; ++c) sum += readSample(frame + c * bps, fmt.kind);
        out.push_back(FNV_MUT(392) ? sum : sum / static_cast<float>(fmt.channels));   // twin 392: channels are added, not averaged
    }
}

void stereoToDevice(const float* stereo, size_t frames, const DeviceFormat& fmt, uint8_t* dst) {
    if (!dst || fmt.channels == 0) return;
    const size_t bps = fmt.bytesPerSample();
    for (size_t f = 0; f < frames; ++f) {
        uint8_t* frame = dst + f * fmt.bytesPerFrame();
        const float l = stereo ? stereo[f * 2] : 0.0f;
        const float r = stereo ? stereo[f * 2 + 1] : 0.0f;
        for (uint32_t c = 0; c < fmt.channels; ++c) {
            float v = 0.0f;
            if (fmt.channels == 1) v = 0.5f * (l + r);
            else if (c == 0) v = l;
            else if (c == 1) v = FNV_MUT(393) ? l : r;   // twin 393: the right ear gets the left side too
            writeSample(frame + c * bps, fmt.kind, v);
        }
    }
}

void deviceSilence(uint8_t* dst, size_t frames, const DeviceFormat& fmt) {
    if (dst) std::memset(dst, 0, frames * fmt.bytesPerFrame());   // all zero bits is silence in every format handled here
}

void CaptureConverter::feed(const uint8_t* data, size_t frames, bool silent, std::vector<int16_t>& out) {
    if (frames == 0) return;
    mono_.clear();
    if ((silent && !FNV_MUT(387)) || !data) mono_.assign(frames, 0.0f);
    else deviceToMono(data, frames, fmt_, mono_);
    conv_.clear();
    rs_.process(mono_.data(), mono_.size(), conv_);
    for (float v : conv_) out.push_back(static_cast<int16_t>(std::lrintf(clampTo(v, -32768.0f, 32767.0f))));
}

PlaybackFeeder::PlaybackFeeder(const DeviceFormat& fmt, RenderFn render)
    : fmt_(fmt), render_(std::move(render)), left_(kSampleRate, fmt.rate), right_(kSampleRate, fmt.rate), block_(kFrameSamples * 2) {}

void PlaybackFeeder::reset() {
    left_.reset();
    right_.reset();
    pend_.clear();
    head_ = 0;
}

void PlaybackFeeder::fill(uint8_t* dst, size_t frames) {
    if (!dst || frames == 0) return;
    while (pending() < frames) {
        if (render_) render_(block_.data(), kFrameSamples);
        else std::fill(block_.begin(), block_.end(), int16_t(0));
        lf_.resize(kFrameSamples);
        rf_.resize(kFrameSamples);
        for (size_t i = 0; i < kFrameSamples; ++i) {
            lf_[i] = block_[i * 2];
            rf_[i] = block_[i * 2 + 1];
        }
        lo_.clear();
        ro_.clear();
        left_.process(lf_.data(), lf_.size(), lo_);
        right_.process(rf_.data(), rf_.size(), ro_);
        if (FNV_MUT(388)) lo_.swap(ro_);   // twin 388: left and right swapped
        const size_t n = std::min(lo_.size(), ro_.size());   // both resamplers see the same input, so these are equal
        if (head_ > 0 && head_ >= pend_.size() / 2) {
            pend_.erase(pend_.begin(), pend_.begin() + static_cast<std::ptrdiff_t>(head_));
            head_ = 0;
        }
        for (size_t i = 0; i < n; ++i) {
            pend_.push_back(lo_[i]);
            pend_.push_back(ro_[i]);
        }
    }
    stereoToDevice(pend_.data() + head_, frames, fmt_, dst);
    head_ += FNV_MUT(396) ? frames : frames * 2;   // twin 396: only half of what was handed out is used up, the rest plays again
    if (head_ == pend_.size()) {
        pend_.clear();
        head_ = 0;
    }
}

}  // namespace nvcoop::voice
