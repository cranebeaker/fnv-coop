// plugin/voice: the audio arithmetic that sits between the codec and the sound devices (voice_resample.*, voice_mix.*).
// Groups: resample, devfmt, device (capture / playback adapters), mix, limit. The FNV_MUT twins 383..399 must each fail this suite.
#include <algorithm>
#include <cmath>
#include <cstring>
#include <vector>

#include "minitest.h"
#include "voice_mix.h"
#include "voice_resample.h"
#include "voice_rig.h"

using namespace nvcoop::voice;

namespace {
constexpr double kTwoPi = 6.283185307179586;

std::vector<float> sine(double hz, double rate, size_t n, double amp) {
    std::vector<float> v(n);
    for (size_t i = 0; i < n; ++i) v[i] = static_cast<float>(amp * std::sin(kTwoPi * hz * static_cast<double>(i) / rate));
    return v;
}

// The amplitude of the tone `hz` inside x[from, to) (project onto sine and cosine). The window should hold whole cycles.
double toneAmp(const std::vector<float>& x, size_t from, size_t to, double hz, double rate) {
    double sr = 0, si = 0;
    for (size_t i = from; i < to; ++i) {
        const double ph = kTwoPi * hz * static_cast<double>(i) / rate;
        sr += x[i] * std::cos(ph);
        si += x[i] * std::sin(ph);
    }
    return 2.0 * std::sqrt(sr * sr + si * si) / static_cast<double>(to - from);
}

std::vector<float> convert(uint32_t inRate, uint32_t outRate, const std::vector<float>& in) {
    Resampler r(inRate, outRate);
    std::vector<float> out;
    r.process(in.data(), in.size(), out);
    return out;
}
}  // namespace

// ------------------------------------------------------------------------------------------------------------------- resample

TEST(resample, equal_rates_pass_the_samples_through_untouched) {
    Resampler r(16000, 16000);
    CHECK(r.passthrough());
    const std::vector<float> in = {1, -2, 3.5f, 4};
    std::vector<float> out;
    r.process(in.data(), in.size(), out);
    CHECK(out == in);
}

TEST(resample, the_output_length_follows_the_rate_ratio) {
    // 0.1 s in each case; the kernel delay costs a few samples at the end
    const auto down = convert(48000, 16000, sine(400, 48000, 4800, 1000));
    CHECK(down.size() >= 1600 - 40 && down.size() <= 1600);
    const auto up = convert(16000, 48000, sine(400, 16000, 1600, 1000));
    CHECK(up.size() >= 4800 - 40 * 3 && up.size() <= 4800);
    const auto odd = convert(44100, 16000, sine(400, 44100, 4410, 1000));
    CHECK(odd.size() >= 1600 - 40 && odd.size() <= 1600);
    Resampler r(44100, 16000);
    CHECK(std::fabs(r.ratio() - 16000.0 / 44100.0) < 1e-9);
}

TEST(resample, a_speech_band_tone_keeps_its_level_going_down_and_up) {
    const auto down = convert(48000, 16000, sine(400, 48000, 48000, 10000));
    CHECK(std::fabs(toneAmp(down, 200, 1800, 400, 16000) - 10000.0) < 200.0);
    const auto up = convert(16000, 48000, sine(400, 16000, 16000, 10000));
    CHECK(std::fabs(toneAmp(up, 600, 5400, 400, 48000) - 10000.0) < 200.0);
    const auto odd = convert(44100, 16000, sine(1000, 44100, 44100, 10000));
    CHECK(std::fabs(toneAmp(odd, 200, 1800, 1000, 16000) - 10000.0) < 250.0);
}

TEST(resample, a_constant_stays_constant) {
    const auto down = convert(48000, 16000, std::vector<float>(9600, 5000.0f));
    REQUIRE(down.size() > 400);
    for (size_t i = 200; i < down.size(); ++i) CHECK(std::fabs(down[i] - 5000.0f) < 25.0f);
    const auto up = convert(16000, 48000, std::vector<float>(3200, 5000.0f));
    for (size_t i = 600; i < up.size(); ++i) CHECK(std::fabs(up[i] - 5000.0f) < 25.0f);
}

TEST(resample, a_tone_above_the_new_rate_is_filtered_out_not_folded_back) {
    // 20 kHz sampled at 48 kHz would land on 4 kHz at 16 kHz if nothing filtered it
    const auto out = convert(48000, 16000, sine(20000, 48000, 48000, 10000));
    CHECK(toneAmp(out, 200, 1800, 4000, 16000) < 300.0);
}

TEST(resample, cutting_the_input_up_differently_gives_the_same_output) {
    const auto in = sine(700, 48000, 5000, 9000);
    const auto whole = convert(48000, 16000, in);
    Resampler r(48000, 16000);
    std::vector<float> pieces;
    const size_t cuts[] = {1, 7, 333, 2, 1024, 50};
    size_t at = 0, k = 0;
    while (at < in.size()) {
        const size_t n = std::min(cuts[k++ % 6], in.size() - at);
        r.process(in.data() + at, n, pieces);
        at += n;
    }
    CHECK(pieces == whole);
}

TEST(resample, reset_forgets_the_tail_of_the_old_stream) {
    Resampler r(48000, 16000);
    std::vector<float> out;
    const auto loud = sine(500, 48000, 2400, 20000);
    r.process(loud.data(), loud.size(), out);
    r.reset();
    out.clear();
    const std::vector<float> quiet(2400, 0.0f);
    r.process(quiet.data(), quiet.size(), out);
    REQUIRE(!out.empty());
    for (float v : out) CHECK(std::fabs(v) < 0.001f);
}

// -------------------------------------------------------------------------------------------------------------------- devfmt

TEST(devfmt, every_sample_kind_reads_on_the_16_bit_scale_and_channels_are_averaged) {
    std::vector<float> out;
    {   // 16 bit stereo
        const int16_t s[4] = {1000, 3000, -2000, -4000};
        deviceToMono(reinterpret_cast<const uint8_t*>(s), 2, DeviceFormat{48000, 2, SampleKind::Int16}, out);
        REQUIRE(out.size() == 2);
        CHECK(out[0] == 2000.0f);
        CHECK(out[1] == -3000.0f);
    }
    out.clear();
    {   // float: 0.5 is half of full scale
        const float s[2] = {0.5f, 0.5f};
        deviceToMono(reinterpret_cast<const uint8_t*>(s), 1, DeviceFormat{48000, 2, SampleKind::Float32}, out);
        REQUIRE(out.size() == 1);
        CHECK(out[0] == 16384.0f);
    }
    out.clear();
    {   // 24 bit mono, little endian: 0x400000 is half of full scale, 0xC00000 is minus half
        const uint8_t s[6] = {0x00, 0x00, 0x40, 0x00, 0x00, 0xC0};
        deviceToMono(s, 2, DeviceFormat{44100, 1, SampleKind::Int24}, out);
        REQUIRE(out.size() == 2);
        CHECK(out[0] == 16384.0f);
        CHECK(out[1] == -16384.0f);
    }
    out.clear();
    {   // 32 bit
        const int32_t s[1] = {0x20000000};
        deviceToMono(reinterpret_cast<const uint8_t*>(s), 1, DeviceFormat{44100, 1, SampleKind::Int32}, out);
        CHECK(out[0] == 8192.0f);
    }
    out.clear();
    {   // six channels (a surround device): all averaged
        const int16_t s[6] = {600, 0, 0, 0, 0, 0};
        deviceToMono(reinterpret_cast<const uint8_t*>(s), 1, DeviceFormat{48000, 6, SampleKind::Int16}, out);
        CHECK(out[0] == 100.0f);
    }
    out.clear();
    deviceToMono(nullptr, 5, DeviceFormat{}, out);
    CHECK(out.empty());
}

TEST(devfmt, stereo_goes_to_the_right_channels_in_every_format) {
    const float st[4] = {8000.0f, -4000.0f, 1000.0f, 2000.0f};   // two stereo frames
    {
        int16_t d[4] = {};
        stereoToDevice(st, 2, DeviceFormat{48000, 2, SampleKind::Int16}, reinterpret_cast<uint8_t*>(d));
        CHECK(d[0] == 8000);
        CHECK(d[1] == -4000);
        CHECK(d[2] == 1000);
        CHECK(d[3] == 2000);
    }
    {   // a mono device hears the average
        int16_t d[2] = {};
        stereoToDevice(st, 2, DeviceFormat{48000, 1, SampleKind::Int16}, reinterpret_cast<uint8_t*>(d));
        CHECK(d[0] == 2000);
        CHECK(d[1] == 1500);
    }
    {   // a surround device: front pair only, the rest silent
        int16_t d[6] = {7, 7, 7, 7, 7, 7};
        stereoToDevice(st, 1, DeviceFormat{48000, 6, SampleKind::Int16}, reinterpret_cast<uint8_t*>(d));
        CHECK(d[0] == 8000);
        CHECK(d[1] == -4000);
        for (int i = 2; i < 6; ++i) CHECK(d[i] == 0);
    }
    {
        float d[4] = {};
        stereoToDevice(st, 2, DeviceFormat{48000, 2, SampleKind::Float32}, reinterpret_cast<uint8_t*>(d));
        CHECK(std::fabs(d[0] - 8000.0f / 32768.0f) < 1e-6f);
        CHECK(std::fabs(d[1] + 4000.0f / 32768.0f) < 1e-6f);
    }
}

TEST(devfmt, what_goes_to_a_device_reads_back_the_same_in_every_format) {
    const float st[6] = {12345.0f, -23456.0f, 100.0f, -100.0f, 32000.0f, -32000.0f};
    for (SampleKind k : {SampleKind::Int16, SampleKind::Int24, SampleKind::Int32, SampleKind::Float32}) {
        DeviceFormat fmt{48000, 2, k};
        std::vector<uint8_t> bytes(3 * fmt.bytesPerFrame());
        stereoToDevice(st, 3, fmt, bytes.data());
        // read channel 0 back through a mono view of the same bytes: average of (l, r)
        std::vector<float> mono;
        deviceToMono(bytes.data(), 3, fmt, mono);
        REQUIRE(mono.size() == 3);
        for (int i = 0; i < 3; ++i) CHECK(std::fabs(mono[i] - 0.5f * (st[i * 2] + st[i * 2 + 1])) <= 1.0f);
    }
}

TEST(devfmt, a_loud_value_is_clamped_never_wrapped) {
    const float st[4] = {90000.0f, -90000.0f, 40000.0f, -40000.0f};
    int16_t d[4] = {};
    stereoToDevice(st, 2, DeviceFormat{48000, 2, SampleKind::Int16}, reinterpret_cast<uint8_t*>(d));
    CHECK(d[0] == 32767);
    CHECK(d[1] == -32768);
    CHECK(d[2] == 32767);
    CHECK(d[3] == -32768);
    int32_t e[2] = {};
    stereoToDevice(st, 1, DeviceFormat{48000, 2, SampleKind::Int32}, reinterpret_cast<uint8_t*>(e));
    CHECK(e[0] > 2000000000);
    CHECK(e[1] < -2000000000);
}

TEST(devfmt, silence_is_all_zero_in_the_devices_own_format) {
    std::vector<uint8_t> b(36, 0xAB);
    deviceSilence(b.data(), 6, DeviceFormat{48000, 2, SampleKind::Int24});
    for (uint8_t v : b) CHECK(v == 0);
}

// -------------------------------------------------------------------------------------------------------------------- device

TEST(device, the_microphone_adapter_turns_a_48k_float_stereo_buffer_into_16k_16_bit) {
    const DeviceFormat fmt{48000, 2, SampleKind::Float32};
    CaptureConverter cap(fmt);
    const size_t n = 4800;
    std::vector<float> dev(n * 2);
    for (size_t i = 0; i < n; ++i) dev[i * 2] = dev[i * 2 + 1] = static_cast<float>(0.25 * std::sin(kTwoPi * 400.0 * static_cast<double>(i) / 48000.0));
    std::vector<int16_t> out;
    // delivered in uneven pieces, like packets are
    size_t at = 0;
    const size_t cuts[] = {480, 441, 1000, 7};
    for (size_t k = 0; at < n; ++k) {
        const size_t c = std::min(cuts[k % 4], n - at);
        cap.feed(reinterpret_cast<const uint8_t*>(dev.data() + at * 2), c, false, out);
        at += c;
    }
    CHECK(out.size() >= 1600 - 40 && out.size() <= 1600);
    REQUIRE(out.size() > 1500);
    std::vector<float> f(out.begin(), out.end());
    CHECK(std::fabs(toneAmp(f, 200, 1500, 400, 16000) - 8192.0) < 250.0);
}

TEST(device, a_buffer_the_device_marks_silent_is_silence_whatever_its_bytes_say) {
    CaptureConverter cap(DeviceFormat{48000, 1, SampleKind::Int16});
    const std::vector<int16_t> noise(960, 20000);
    std::vector<int16_t> out;
    cap.feed(reinterpret_cast<const uint8_t*>(noise.data()), noise.size(), true, out);
    cap.feed(reinterpret_cast<const uint8_t*>(noise.data()), noise.size(), true, out);
    REQUIRE(!out.empty());
    for (int16_t v : out) CHECK(v == 0);
}

TEST(device, the_speaker_adapter_hands_out_exactly_what_the_device_asks_for_with_no_gaps_or_repeats) {
    // 16 kHz device: no resampling, so a counting signal must come out as exactly 0, 1, 2, ... whatever the request sizes
    int counter = 0, blocks = 0;
    PlaybackFeeder feeder(DeviceFormat{16000, 2, SampleKind::Int16}, [&](int16_t* st, size_t frames) {
        ++blocks;
        for (size_t i = 0; i < frames; ++i) {
            st[i * 2] = static_cast<int16_t>(counter);
            st[i * 2 + 1] = static_cast<int16_t>(-counter);
            ++counter;
        }
    });
    std::vector<int16_t> got;
    const size_t asks[] = {100, 333, 7, 160, 1, 480, 320, 50};
    size_t total = 0;
    for (int round = 0; round < 6; ++round) {
        for (size_t a : asks) {
            std::vector<int16_t> d(a * 2);
            feeder.fill(reinterpret_cast<uint8_t*>(d.data()), a);
            got.insert(got.end(), d.begin(), d.end());
            total += a;
            CHECK(feeder.pending() < kFrameSamples);
        }
    }
    REQUIRE(got.size() == total * 2);
    for (size_t i = 0; i < total; ++i) {
        CHECK(got[i * 2] == static_cast<int16_t>(i));
        CHECK(got[i * 2 + 1] == static_cast<int16_t>(-static_cast<int>(i)));
    }
    CHECK(static_cast<size_t>(blocks) == (total + kFrameSamples - 1) / kFrameSamples);   // asked for no more than needed
}

TEST(device, a_voice_played_through_a_44k_device_arrives_on_the_right_side_at_the_right_level) {
    vt::FakeCodecs codecs;
    VoiceReceiver rx(codecs);
    Placement p;
    p.audible = true;
    p.gain = 1.0f;
    p.pan = 1.0f;   // hard right
    rx.setPlacement(1, p);
    uint16_t seq = 0;
    PlaybackFeeder feeder(DeviceFormat{44100, 2, SampleKind::Int16}, [&](int16_t* st, size_t frames) {
        const Bytes f = vt::fakeFrame(10000);
        rx.push(1, seq, seq == 0 ? kFlagStart : 0, f.data(), f.size());
        ++seq;
        rx.render(st, frames);
    });
    std::vector<int16_t> d(441 * 2);
    int16_t lastL = 0, lastR = 0;
    for (int i = 0; i < 80; ++i) {   // 0.8 s
        feeder.fill(reinterpret_cast<uint8_t*>(d.data()), 441);
        lastL = d[880];
        lastR = d[881];
    }
    CHECK(std::abs(static_cast<int>(lastR) - 10000) < 150);
    CHECK(std::abs(static_cast<int>(lastL)) < 150);
}

// ----------------------------------------------------------------------------------------------------------------------- mix

TEST(mix, pan_is_equal_power_and_the_ends_are_one_ear) {
    float l = 0, r = 0;
    panGains(1.0f, 0.0f, l, r);
    CHECK(std::fabs(l - 0.70710678f) < 1e-4f);
    CHECK(std::fabs(r - 0.70710678f) < 1e-4f);
    panGains(0.8f, -1.0f, l, r);
    CHECK(std::fabs(l - 0.8f) < 1e-4f);
    CHECK(std::fabs(r) < 1e-4f);
    panGains(0.8f, 1.0f, l, r);
    CHECK(std::fabs(l) < 1e-4f);
    CHECK(std::fabs(r - 0.8f) < 1e-4f);
    for (float pan = -1.0f; pan <= 1.0f; pan += 0.125f) {
        panGains(0.6f, pan, l, r);
        CHECK(std::fabs(l * l + r * r - 0.36f) < 1e-4f);   // the same loudness wherever the voice is
    }
    // beyond the ends and broken numbers do not blow up
    panGains(1.0f, 5.0f, l, r);
    CHECK(std::fabs(r - 1.0f) < 1e-4f);
    panGains(1.0f, std::nanf(""), l, r);
    CHECK(std::fabs(l - r) < 1e-4f);
}

TEST(mix, a_gain_glides_to_its_target_with_the_given_time_constant) {
    const float glide = glideCoefficient(40.0f, 16000.0f);
    MixBus bus;
    VoiceGain g;
    const std::vector<int16_t> v(320, 10000);
    // after 40 ms (one time constant) a one-pole glide is at 63% of the way
    bus.begin(640);
    bus.addVoice(v.data(), 320, g, 1.0f, 0.0f, glide);
    CHECK(std::fabs(g.l - 0.632f) < 0.02f);
    CHECK(g.r == 0.0f);
    bus.begin(320);
    bus.addVoice(v.data(), 320, g, 1.0f, 0.0f, glide);
    CHECK(g.l > 0.74f && g.l < 0.81f);
}

TEST(mix, voices_add_up_and_a_short_voice_leaves_the_rest_of_the_block_silent) {
    MixBus bus;
    VoiceGain a{1, 0}, b{0, 1};
    const std::vector<int16_t> va(100, 3000), vb(60, 1000);
    bus.begin(100);
    bus.addVoice(va.data(), 100, a, 1.0f, 0.0f, 0.0f);   // glide 0: the gains stay where they are
    bus.addVoice(vb.data(), 60, b, 0.0f, 1.0f, 0.0f);
    std::vector<int16_t> out(200);
    bus.finish(out.data());
    CHECK(out[0] == 3000);        // left: voice a
    CHECK(out[1] == 1000);        // right: voice b
    CHECK(out[2 * 59 + 1] == 1000);
    CHECK(out[2 * 60 + 1] == 0);  // b has run out
    CHECK(out[2 * 99] == 3000);
}

TEST(mix, every_block_starts_from_silence) {
    MixBus bus;
    VoiceGain g{1, 1};
    const std::vector<int16_t> v(64, 4000);
    std::vector<int16_t> out(128);
    for (int i = 0; i < 3; ++i) {
        bus.begin(64);
        bus.addVoice(v.data(), 64, g, 1.0f, 1.0f, 0.0f);
        bus.finish(out.data());
        CHECK(out[0] == 4000);
        CHECK(out[127] == 4000);
    }
}

TEST(mix, many_loud_voices_never_wrap_around) {
    MixBus bus;
    VoiceGain g[5];
    for (auto& x : g) x = VoiceGain{1, 1};
    const std::vector<int16_t> v(32, 30000);
    bus.begin(32);
    for (auto& x : g) bus.addVoice(v.data(), 32, x, 1.0f, 1.0f, 0.0f);
    std::vector<int16_t> out(64);
    bus.finish(out.data());
    for (int16_t s : out) CHECK(s == 32767);
    bus.begin(32);
    for (auto& x : g) bus.addVoice(v.data(), 32, x, -1.0f, -1.0f, 1.0f);
    bus.finish(out.data());
    for (int16_t s : out) CHECK(s <= 0);   // inverted and still not wrapped to the other sign
}

TEST(mix, the_fifo_keeps_order_across_partial_takes_pushes_and_reclaiming) {
    SampleFifo f;
    std::vector<int16_t> a(100), b(10), got(200);
    for (int i = 0; i < 100; ++i) a[i] = static_cast<int16_t>(i);
    for (int i = 0; i < 10; ++i) b[i] = static_cast<int16_t>(100 + i);
    f.push(a.data(), a.size());
    CHECK(f.size() == 100);
    CHECK(f.take(got.data(), 60) == 60);
    f.push(b.data(), b.size());   // the consumed front is the larger part by now: it is reclaimed here
    CHECK(f.size() == 50);
    CHECK(f.take(got.data() + 60, 500) == 50);
    CHECK(f.size() == 0);
    for (int i = 0; i < 110; ++i) CHECK(got[static_cast<size_t>(i)] == i);
    CHECK(f.take(got.data(), 10) == 0);
    f.push(a.data(), 5);
    f.clear();
    CHECK(f.size() == 0);
}

// ---------------------------------------------------------------------------------------------------------------------- limit

TEST(limit, quiet_audio_is_not_touched) {
    for (float x : {0.0f, 1.0f, 5000.0f, 22000.0f, -22000.0f, -1.0f}) CHECK(softLimit(x) == x);
}

TEST(limit, loud_audio_is_bent_smoothly_and_never_passes_full_scale) {
    float prev = 0;
    for (float x = 0; x <= 90000.0f; x += 50.0f) {
        const float y = softLimit(x);
        CHECK(y <= 32767.0f);
        CHECK(y >= prev);                 // never goes down as the input goes up
        CHECK(y - prev <= 50.0f + 0.01f); // and never climbs faster than the input: no steps
        prev = y;
    }
    CHECK(softLimit(25000.0f) < 25000.0f);   // already bending
    CHECK(softLimit(25000.0f) > 24000.0f);
    CHECK(softLimit(100000.0f) == 32767.0f);
}

TEST(limit, both_signs_are_mirror_images) {
    for (float x : {100.0f, 23000.0f, 28000.0f, 40000.0f, 1e9f}) CHECK(softLimit(-x) == -softLimit(x));
}

TEST(limit, conversion_to_16_bit_rounds_and_survives_broken_numbers) {
    CHECK(toPcm16(100.4f) == 100);
    CHECK(toPcm16(100.6f) == 101);
    CHECK(toPcm16(-100.6f) == -101);
    CHECK(toPcm16(1e30f) == 32767);
    CHECK(toPcm16(-1e30f) == -32767);
    CHECK(toPcm16(std::nanf("")) == 0);
    CHECK(toPcm16(INFINITY) == 0);
    CHECK(toPcm16(70000.0f) == 32767);
}
