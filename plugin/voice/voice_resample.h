// Sample-rate and sample-format conversion between what a sound device gives us (any rate, any channel count, 16 / 24 / 32 bit
// integer or 32 bit float) and what the voice codec wants (16 kHz, mono in, stereo out). Game-free and Windows-free, so it is
// unit tested; voice_audio_win.cpp only hands it the device buffers.
#pragma once
#include <cstddef>
#include <cstdint>
#include <functional>
#include <vector>

#include "voice_types.h"

namespace nvcoop::voice {

// Converts a stream of float samples from one rate to another (one channel; use one instance per channel). Windowed-sinc, with the
// cutoff pulled below the lower of the two rates so that going down does not fold high tones back into the speech band. Streaming:
// feed it any chunk sizes, the output does not depend on how the input was cut up. Costs a delay of about half a millisecond.
class Resampler {
public:
    Resampler(uint32_t inRate, uint32_t outRate);
    // Appends the converted samples of `in` (n samples) to `out`.
    void process(const float* in, size_t n, std::vector<float>& out);
    // Forgets the history (a new stream starts: the old tail must not leak into it).
    void reset();
    // Output samples per input sample in the long run.
    double ratio() const { return static_cast<double>(outStep_) / inStep_; }
    bool passthrough() const { return passthrough_; }

private:
    uint32_t inStep_, outStep_;    // the two rates divided by their common factor
    bool passthrough_;
    int half_ = 0;                 // taps on each side of the centre
    float cutoff_ = 1.0f;          // relative to the lower of the two Nyquist rates
    std::vector<float> table_;     // the kernel, sampled finely and read with linear interpolation
    std::vector<float> buf_;       // input history + input not yet consumed
    int64_t pos_ = 0;              // where the next output sits in buf_, in 1/outStep_ input samples

    void build();
    float kernel(float distance) const;
};

// What a device buffer contains.
enum class SampleKind { Int16, Int24, Int32, Float32 };

struct DeviceFormat {
    uint32_t rate = 48000;
    uint32_t channels = 2;
    SampleKind kind = SampleKind::Float32;
    size_t bytesPerSample() const { return kind == SampleKind::Int16 ? 2 : kind == SampleKind::Int24 ? 3 : 4; }
    size_t bytesPerFrame() const { return bytesPerSample() * channels; }
};

// `frames` frames of device samples -> mono float on the 16 bit scale (about -32768..32767), the average of all channels.
// Appends to `out`.
void deviceToMono(const uint8_t* src, size_t frames, const DeviceFormat& fmt, std::vector<float>& out);
// Stereo float (interleaved left, right; 16 bit scale) -> device frames. Channel 0 gets the left side, channel 1 the right, a mono
// device the average, any further channel silence. Clamps (never wraps).
void stereoToDevice(const float* stereo, size_t frames, const DeviceFormat& fmt, uint8_t* dst);
// Silence in the device's own format.
void deviceSilence(uint8_t* dst, size_t frames, const DeviceFormat& fmt);

// Microphone side of a device stream: device buffers in, 16 kHz mono 16 bit out (what the codec side wants).
class CaptureConverter {
public:
    explicit CaptureConverter(const DeviceFormat& fmt) : fmt_(fmt), rs_(fmt.rate, kSampleRate) {}
    // Appends the 16 kHz samples of `frames` device frames to `out`. `silent`: the device marked the buffer as silence (its bytes
    // are not to be trusted then).
    void feed(const uint8_t* data, size_t frames, bool silent, std::vector<int16_t>& out);
    void reset() { rs_.reset(); }

private:
    DeviceFormat fmt_;
    Resampler rs_;
    std::vector<float> mono_, conv_;
};

// Speaker side of a device stream: pulls 20 ms stereo blocks at 16 kHz from `render` as needed and hands the device exactly the
// number of frames it asks for, in its own rate and format.
class PlaybackFeeder {
public:
    using RenderFn = std::function<void(int16_t* stereo, size_t frames)>;
    PlaybackFeeder(const DeviceFormat& fmt, RenderFn render);
    void fill(uint8_t* dst, size_t frames);
    // Device frames converted but not handed out yet (always less than one block's worth).
    size_t pending() const { return (pend_.size() - head_) / 2; }
    void reset();

private:
    DeviceFormat fmt_;
    RenderFn render_;
    Resampler left_, right_;
    std::vector<int16_t> block_;
    std::vector<float> lf_, rf_, lo_, ro_, pend_;   // pend_: interleaved device-rate stereo waiting to go out
    size_t head_ = 0;
};

}  // namespace nvcoop::voice
