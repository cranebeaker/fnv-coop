// plugin/voice: the real codec (third_party/opus). Group: opus. Speech-like test signals; checks that what goes in comes out, that
// frames fit the wire, that a lost frame is concealed, and that a whole voice session works with the real thing.
#include <algorithm>
#include <cmath>
#include <cstring>

#include "minitest.h"
#include "voice_rig.h"

using namespace vt;

namespace {

const double kPi = 3.14159265358979323846;

// A voiced-speech-like test tone: a 200 Hz fundamental with a few harmonics, slow amplitude wobble.
std::vector<int16_t> tone(size_t samples, double hz, double amp, size_t startAt = 0) {
    std::vector<int16_t> v(samples);
    for (size_t i = 0; i < samples; ++i) {
        const double t = static_cast<double>(startAt + i) / kSampleRate;
        v[i] = static_cast<int16_t>(amp * std::sin(2 * kPi * hz * t));
    }
    return v;
}

// Energy of `hz` in a block (Goertzel).
double energyAt(const std::vector<int16_t>& x, size_t from, size_t n, double hz) {
    const double w = 2 * kPi * hz / kSampleRate;
    const double c = 2 * std::cos(w);
    double s1 = 0, s2 = 0;
    for (size_t i = 0; i < n; ++i) {
        const double s = x[from + i] + c * s1 - s2;
        s2 = s1;
        s1 = s;
    }
    return (s1 * s1 + s2 * s2 - c * s1 * s2) / (static_cast<double>(n) * n);
}

double rms(const std::vector<int16_t>& x, size_t from, size_t n) {
    double s = 0;
    for (size_t i = 0; i < n; ++i) s += static_cast<double>(x[from + i]) * x[from + i];
    return std::sqrt(s / static_cast<double>(n));
}

}  // namespace

TEST(opus, a_tone_survives_the_round_trip_and_fits_the_wire) {
    auto f = makeOpusFactory(24000);
    REQUIRE(f != nullptr);
    auto enc = f->makeEncoder();
    auto dec = f->makeDecoder();
    REQUIRE(enc && dec);
    const size_t frames = 60;
    const std::vector<int16_t> in = tone(frames * kFrameSamples, 440.0, 8000.0);
    std::vector<int16_t> out;
    size_t bytes = 0, biggest = 0;
    for (size_t i = 0; i < frames; ++i) {
        uint8_t buf[kMaxPayload];
        const int n = enc->encode(in.data() + i * kFrameSamples, buf, sizeof buf);
        REQUIRE(n > 0);
        bytes += static_cast<size_t>(n);
        biggest = std::max<size_t>(biggest, static_cast<size_t>(n));
        int16_t pcm[kFrameSamples];
        REQUIRE(dec->decode(buf, static_cast<size_t>(n), pcm) == static_cast<int>(kFrameSamples));
        out.insert(out.end(), pcm, pcm + kFrameSamples);
    }
    CHECK(biggest <= kMaxPayload);
    CHECK(bytes / frames < 120);   // well under the datagram limit; 24 kbit/s is about 60 bytes a frame
    // after the start-up, the output has the tone's level and its pitch
    const size_t from = 20 * kFrameSamples, n = 30 * kFrameSamples;
    const double inRms = rms(in, from, n), outRms = rms(out, from, n);
    CHECK(outRms > inRms * 0.6 && outRms < inRms * 1.4);
    const double at440 = energyAt(out, from, n, 440.0);
    const double at3000 = energyAt(out, from, n, 3000.0);
    CHECK(at440 > at3000 * 100.0);
}

TEST(opus, silence_is_cheap_and_stays_quiet) {
    auto f = makeOpusFactory(24000);
    REQUIRE(f != nullptr);
    auto enc = f->makeEncoder();
    auto dec = f->makeDecoder();
    std::vector<int16_t> zero(kFrameSamples, 0);
    for (int i = 0; i < 20; ++i) {
        uint8_t buf[kMaxPayload];
        const int n = enc->encode(zero.data(), buf, sizeof buf);
        REQUIRE(n > 0);
        CHECK(n < 40);
        int16_t pcm[kFrameSamples];
        REQUIRE(dec->decode(buf, static_cast<size_t>(n), pcm) > 0);
        if (i > 5) for (int16_t v : pcm) CHECK(std::abs(v) < 50);
    }
}

TEST(opus, a_lost_frame_is_concealed_with_sound_not_a_hole) {
    auto f = makeOpusFactory(24000);
    auto enc = f->makeEncoder();
    auto dec = f->makeDecoder();
    const std::vector<int16_t> in = tone(40 * kFrameSamples, 300.0, 8000.0);
    int16_t pcm[kFrameSamples];
    for (size_t i = 0; i < 30; ++i) {
        uint8_t buf[kMaxPayload];
        const int n = enc->encode(in.data() + i * kFrameSamples, buf, sizeof buf);
        REQUIRE(dec->decode(buf, static_cast<size_t>(n), pcm) > 0);
    }
    REQUIRE(dec->decode(nullptr, 0, pcm) == static_cast<int>(kFrameSamples));   // the lost frame
    std::vector<int16_t> v(pcm, pcm + kFrameSamples);
    CHECK(rms(v, 0, v.size()) > 1000.0);   // it carries on with the sound, fading
}

TEST(opus, garbage_never_crashes_the_decoder) {
    auto f = makeOpusFactory(24000);
    auto dec = f->makeDecoder();
    uint32_t s = 99;
    auto rnd = [&s]() { s = s * 1664525u + 1013904223u; return s >> 16; };
    int16_t pcm[kFrameSamples];
    for (int i = 0; i < 3000; ++i) {
        uint8_t buf[kMaxPayload];
        const size_t n = 1 + rnd() % kMaxPayload;
        for (size_t k = 0; k < n; ++k) buf[k] = static_cast<uint8_t>(rnd());
        const int r = dec->decode(buf, n, pcm);
        CHECK(r <= static_cast<int>(kFrameSamples));
    }
    CHECK(dec->decode(nullptr, 0, pcm) == static_cast<int>(kFrameSamples));
}

TEST(opus, the_bit_rate_is_clamped_and_a_higher_rate_costs_more_bytes) {
    auto lo = makeOpusFactory(1);          // clamped up to 6000
    auto hi = makeOpusFactory(10000000);   // clamped down to 64000
    REQUIRE(lo && hi);
    auto el = lo->makeEncoder();
    auto eh = hi->makeEncoder();
    const std::vector<int16_t> in = tone(30 * kFrameSamples, 500.0, 6000.0);
    size_t bl = 0, bh = 0;
    for (size_t i = 0; i < 30; ++i) {
        uint8_t buf[kMaxPayload];
        bl += static_cast<size_t>(el->encode(in.data() + i * kFrameSamples, buf, sizeof buf));
        bh += static_cast<size_t>(eh->encode(in.data() + i * kFrameSamples, buf, sizeof buf));
    }
    CHECK(bl < bh);
}

TEST(opus, a_whole_session_with_the_real_codec_carries_a_voice_to_the_right_ear) {
    auto opus = makeOpusFactory(24000);
    REQUIRE(opus != nullptr);
    // host + one guest on the real codec
    std::vector<Bytes> toHost, toGuest;
    SessionConfig hc;
    hc.isHost = true;
    hc.self = 0;
    SessionConfig gc;
    gc.self = 1;
    VoiceSession host(hc, *opus, [&toGuest](uint8_t, const Bytes& b) { toGuest.push_back(b); }, []() { return std::vector<uint8_t>{1}; });
    VoiceSession guest(gc, *opus, [&toHost](uint8_t, const Bytes& b) { toHost.push_back(b); }, nullptr);
    Placement right;
    right.audible = true;
    right.gain = 1.0f;
    right.pan = 1.0f;
    host.rx().setPlacement(1, right);
    const std::vector<int16_t> in = tone(80 * kFrameSamples, 440.0, 8000.0);
    std::vector<int16_t> heard;
    for (size_t i = 0; i < 80; ++i) {
        guest.feedMic(in.data() + i * kFrameSamples, true, false);
        for (const Bytes& b : toHost) host.onDatagram(1, b.data(), b.size(), 1000 + i * 20);
        toHost.clear();
        int16_t buf[kFrameSamples * 2];
        host.render(buf, kFrameSamples);
        heard.insert(heard.end(), buf, buf + kFrameSamples * 2);
    }
    std::vector<int16_t> left, rightCh;
    for (size_t i = 0; i < heard.size() / 2; ++i) {
        left.push_back(heard[i * 2]);
        rightCh.push_back(heard[i * 2 + 1]);
    }
    const size_t from = 40 * kFrameSamples, n = 30 * kFrameSamples;
    CHECK(rms(rightCh, from, n) > 4000.0);
    CHECK(rms(left, from, n) < 100.0);
    CHECK(energyAt(rightCh, from, n, 440.0) > energyAt(rightCh, from, n, 3000.0) * 100.0);
    CHECK(toGuest.empty());   // the host never sent anything back
}
