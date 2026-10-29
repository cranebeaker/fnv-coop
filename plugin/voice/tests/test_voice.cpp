// plugin/voice tests (game-free). Groups: wire, jitter, place, tx, rx, session, config, pcm, hint.
// The FNV_MUT twins 301..399 of plugin/voice must each fail this suite (tools/mut/run_twins.py, plugin\voice\run_tests.cmd).
#include <algorithm>
#include <atomic>
#include <cstdio>
#include <cstring>
#include <map>
#include <thread>

#include "minitest.h"
#include "voice_config.h"
#include "voice_hint.h"
#include "voice_pcm_queue.h"
#include "voice_rig.h"

using namespace vt;

namespace {

// ------------------------------------------------------------------------------------------------------------------------ helpers

PushResult pushTag(JitterBuffer& jb, uint16_t seq, uint8_t flags = 0, int tag = -1) {
    const uint8_t b = static_cast<uint8_t>(tag < 0 ? (seq & 0xFF) : tag);
    return jb.push(seq, flags, &b, 1);
}
// The tag of the frame popped, -1 for none, -2 for lost.
int popTag(JitterBuffer& jb, bool* last = nullptr) {
    JitterOut o = jb.pop();
    if (last) *last = o.last;
    if (o.kind == PopKind::None) return -1;
    if (o.kind == PopKind::Lost) return -2;
    return o.payload[0];
}

Where here(float x, float y, float z = 0, uint32_t cell = 1, uint32_t world = 7, bool interior = false) {
    Where w;
    w.present = true;
    w.pos = Vec3{x, y, z};
    w.cell = cell;
    w.world = world;
    w.interior = interior;
    return w;
}

// Feeds one frame per block into a receiver and renders: a steady stream. Returns the last block's peaks.
Peak stream(VoiceReceiver& rx, uint8_t who, int16_t value, int blocks, uint16_t* seq, bool end = false, std::vector<int16_t>* all = nullptr) {
    Peak p;
    std::vector<int16_t> buf(kFrameSamples * 2);
    for (int b = 0; b < blocks; ++b) {
        const Bytes f = fakeFrame(value);
        uint8_t flags = 0;
        if (*seq == 0) flags |= kFlagStart;
        if (end && b == blocks - 1) flags |= kFlagEnd;
        rx.push(who, (*seq)++, flags, f.data(), f.size());
        rx.render(buf.data(), kFrameSamples);
        if (all) all->insert(all->end(), buf.begin(), buf.end());
        p = Peak{};
        for (size_t i = 0; i < kFrameSamples; ++i) {
            p.l = std::max(p.l, std::abs(static_cast<int>(buf[i * 2])));
            p.r = std::max(p.r, std::abs(static_cast<int>(buf[i * 2 + 1])));
        }
    }
    return p;
}

}  // namespace

// ------------------------------------------------------------------------------------------------------------------------- wire

TEST(wire, roundtrip_keeps_every_field) {
    const uint8_t pl[5] = {1, 2, 3, 4, 5};
    Bytes b = encodeVoice(3, 0x1234, kFlagStart | kFlagEnd, pl, 5);
    REQUIRE(b.size() == kHeaderSize + 5);
    CHECK(b[0] == kChannelVoice);
    CHECK(b[3] == 0x34 && b[4] == 0x12);   // little endian sequence number
    VoiceFrame f;
    REQUIRE(decodeVoice(b.data(), b.size(), &f));
    CHECK(f.speaker == 3);
    CHECK(f.seq == 0x1234);
    CHECK(f.flags == (kFlagStart | kFlagEnd));
    REQUIRE(f.size == 5);
    CHECK(std::memcmp(f.payload, pl, 5) == 0);
}

TEST(wire, rejects_everything_that_is_not_exactly_one_frame) {
    const uint8_t pl[4] = {9, 8, 7, 6};
    const Bytes good = encodeVoice(1, 7, 0, pl, 4);
    VoiceFrame f;
    REQUIRE(decodeVoice(good.data(), good.size(), &f));
    {   Bytes b = good; b[0] = 0x57; CHECK(!decodeVoice(b.data(), b.size(), &f)); }                  // another channel
    {   Bytes b = good; b[1] = kWireVersion + 1; CHECK(!decodeVoice(b.data(), b.size(), &f)); }      // another version
    {   Bytes b = good; b[2] = kMaxSlots; CHECK(!decodeVoice(b.data(), b.size(), &f)); }             // a speaker beyond the party
    {   Bytes b = good; b[5] = 0x80; CHECK(!decodeVoice(b.data(), b.size(), &f)); }                  // unknown flag bit
    {   Bytes b = good; b[6] = 0; b.resize(kHeaderSize); CHECK(!decodeVoice(b.data(), b.size(), &f)); }   // empty frame
    {   Bytes b = good; b.push_back(0); CHECK(!decodeVoice(b.data(), b.size(), &f)); }               // trailing byte
    {   Bytes b = good; b.pop_back(); CHECK(!decodeVoice(b.data(), b.size(), &f)); }                 // cut short
    {   Bytes b = good; b[6] = static_cast<uint8_t>(kMaxPayload + 1); b.resize(kHeaderSize + kMaxPayload + 1); CHECK(!decodeVoice(b.data(), b.size(), &f)); }   // too big
    CHECK(!decodeVoice(good.data(), 3, &f));
    CHECK(!decodeVoice(nullptr, 10, &f));
    CHECK(encodeVoice(0, 0, 0, nullptr, 0).empty());
    CHECK(encodeVoice(0, 0, 0, pl, kMaxPayload + 1).empty());
}

TEST(wire, garbage_never_decodes_into_something_inconsistent) {
    uint32_t s = 12345;
    auto rnd = [&s]() { s = s * 1664525u + 1013904223u; return s >> 16; };
    const uint8_t pl[6] = {1, 2, 3, 4, 5, 6};
    const Bytes good = encodeVoice(2, 99, kFlagStart, pl, 6);
    int accepted = 0;
    for (int i = 0; i < 20000; ++i) {
        Bytes b = good;
        const unsigned mode = rnd() % 4;
        if (mode == 0) b.assign(rnd() % 40, 0), std::generate(b.begin(), b.end(), [&rnd]() { return static_cast<uint8_t>(rnd()); });
        else if (mode == 1) b[rnd() % b.size()] = static_cast<uint8_t>(rnd());
        else if (mode == 2) b.resize(rnd() % (b.size() + 4));
        // mode 3: untouched
        VoiceFrame f;
        if (decodeVoice(b.data(), b.size(), &f)) {
            ++accepted;
            CHECK(b.size() == kHeaderSize + f.size && f.size >= 1 && f.size <= kMaxPayload && f.speaker < kMaxSlots && (f.flags & ~kFlagsKnown) == 0);
        }
    }
    CHECK(accepted > 1000);
}

// ---------------------------------------------------------------------------------------------------------------------- jitter

TEST(jitter, a_burst_starts_after_the_wait_and_plays_in_order) {
    JitterBuffer jb;
    CHECK(pushTag(jb, 10, kFlagStart) == PushResult::Queued);
    CHECK(popTag(jb) == -1);
    pushTag(jb, 11);
    CHECK(popTag(jb) == -1);   // two frames: still waiting for the third
    pushTag(jb, 12);
    CHECK(popTag(jb) == 10);
    CHECK(popTag(jb) == 11);
    CHECK(popTag(jb) == 12);
    CHECK(popTag(jb) == -1);   // starved
    CHECK(jb.stats().underruns == 1);
}

TEST(jitter, out_of_order_frames_are_put_back_in_order) {
    JitterBuffer jb;
    pushTag(jb, 0, kFlagStart);
    pushTag(jb, 2);
    pushTag(jb, 1);
    pushTag(jb, 4);
    pushTag(jb, 3);
    for (int want = 0; want < 5; ++want) CHECK(popTag(jb) == want);
    CHECK(jb.stats().lost == 0);
}

TEST(jitter, a_double_is_dropped_and_played_once) {
    JitterBuffer jb;
    pushTag(jb, 0, kFlagStart);
    pushTag(jb, 1);
    CHECK(pushTag(jb, 1) == PushResult::Duplicate);
    pushTag(jb, 2);
    CHECK(popTag(jb) == 0);
    CHECK(popTag(jb) == 1);
    CHECK(popTag(jb) == 2);
    CHECK(popTag(jb) == -1);
    CHECK(jb.stats().duplicates == 1);
}

TEST(jitter, a_frame_that_came_too_late_is_dropped) {
    JitterBuffer jb;
    pushTag(jb, 0, kFlagStart);
    pushTag(jb, 1);
    pushTag(jb, 2);
    CHECK(popTag(jb) == 0);
    CHECK(pushTag(jb, 0) == PushResult::Late);   // played already
    CHECK(popTag(jb) == 1);
    CHECK(jb.stats().late == 1);
}

TEST(jitter, a_missing_frame_is_reported_lost_and_the_stream_goes_on) {
    JitterBuffer jb;
    pushTag(jb, 0, kFlagStart);
    pushTag(jb, 1);
    pushTag(jb, 3);
    pushTag(jb, 4);
    pushTag(jb, 5);
    CHECK(popTag(jb) == 0);
    CHECK(popTag(jb) == 1);
    CHECK(popTag(jb) == -2);   // 2 is missing
    CHECK(popTag(jb) == 3);
    CHECK(popTag(jb) == 4);
    CHECK(jb.stats().lost == 1);
}

TEST(jitter, a_long_gap_is_skipped_not_concealed_for_ever) {
    JitterBuffer jb;
    pushTag(jb, 0, kFlagStart);
    pushTag(jb, 1);
    pushTag(jb, 2);
    CHECK(popTag(jb) == 0);
    CHECK(popTag(jb) == 1);
    CHECK(popTag(jb) == 2);
    pushTag(jb, 8);
    pushTag(jb, 9);
    pushTag(jb, 10);
    int lost = 0, got = -9;
    for (int i = 0; i < 20 && got < 0; ++i) {
        const int t = popTag(jb);
        if (t == -2) ++lost;
        else got = t;
    }
    CHECK(got == 8);
    CHECK(lost == 3);   // maxLostRun
}

TEST(jitter, the_stream_never_lags_more_than_the_cap) {
    JitterBuffer jb;   // maxFrames 12
    pushTag(jb, 0, kFlagStart);
    for (uint16_t s = 1; s < 40; ++s) {
        pushTag(jb, s);
        CHECK(jb.span() <= 12);
    }
    CHECK(jb.stats().skipped == 28);
    CHECK(popTag(jb) == 28);   // the oldest audio was skipped
    for (int want = 29; want < 40; ++want) CHECK(popTag(jb) == want);
    CHECK(popTag(jb) == -1);
}

TEST(jitter, a_starved_stream_waits_for_a_cushion_again) {
    JitterBuffer jb;
    pushTag(jb, 0, kFlagStart);
    pushTag(jb, 1);
    pushTag(jb, 2);
    for (int i = 0; i < 3; ++i) popTag(jb);
    CHECK(popTag(jb) == -1);
    CHECK(jb.stats().underruns == 1);
    pushTag(jb, 3);
    CHECK(popTag(jb) == -1);   // one frame is not a cushion
    pushTag(jb, 4);
    pushTag(jb, 5);
    CHECK(popTag(jb) == 3);
}

TEST(jitter, a_short_burst_with_an_end_flag_plays_at_once_and_goes_idle) {
    JitterBuffer jb;
    pushTag(jb, 5, kFlagStart | kFlagEnd);
    bool last = false;
    CHECK(popTag(jb, &last) == 5);
    CHECK(last);
    CHECK(jb.idle());
    CHECK(popTag(jb) == -1);
}

TEST(jitter, a_short_burst_whose_end_got_lost_plays_after_the_wait) {
    JitterBuffer jb;
    pushTag(jb, 5, kFlagStart);
    for (int i = 0; i < 9; ++i) CHECK(popTag(jb) == -1);
    CHECK(popTag(jb) == 5);
}

TEST(jitter, an_empty_starved_stream_goes_idle) {
    JitterBuffer jb;
    pushTag(jb, 0, kFlagStart);
    pushTag(jb, 1);
    pushTag(jb, 2);
    for (int i = 0; i < 3; ++i) popTag(jb);
    for (int i = 0; i < 30; ++i) popTag(jb);
    CHECK(jb.idle());
}

TEST(jitter, sequence_numbers_wrap_around) {
    JitterBuffer jb;
    pushTag(jb, 65534, kFlagStart, 1);
    pushTag(jb, 65535, 0, 2);
    pushTag(jb, 0, 0, 3);
    pushTag(jb, 1, 0, 4);
    CHECK(popTag(jb) == 1);
    CHECK(popTag(jb) == 2);
    CHECK(popTag(jb) == 3);
    CHECK(popTag(jb) == 4);
    CHECK(jb.stats().lost == 0);
}

TEST(jitter, a_start_flag_restarts_a_stream_whose_numbers_went_back) {
    JitterBuffer jb;
    pushTag(jb, 100, kFlagStart);
    pushTag(jb, 101);
    pushTag(jb, 102);
    CHECK(popTag(jb) == 100);
    CHECK(pushTag(jb, 5, kFlagStart, 200) == PushResult::Queued);   // the sender came back with new numbers
    pushTag(jb, 6, 0, 201);
    pushTag(jb, 7, 0, 202);
    CHECK(popTag(jb) == 200);
    CHECK(popTag(jb) == 201);
    CHECK(popTag(jb) == 202);
}

TEST(jitter, a_double_of_the_start_frame_does_not_restart_the_stream) {
    JitterBuffer jb;
    pushTag(jb, 100, kFlagStart);
    pushTag(jb, 101);
    pushTag(jb, 102);
    CHECK(popTag(jb) == 100);
    CHECK(popTag(jb) == 101);
    CHECK(pushTag(jb, 100, kFlagStart) == PushResult::Late);
    CHECK(popTag(jb) == 102);
    CHECK(jb.stats().resyncs == 0);
}

TEST(jitter, late_frames_in_a_row_mean_the_sender_restarted) {
    JitterBuffer jb;
    pushTag(jb, 100, kFlagStart);
    pushTag(jb, 101);
    pushTag(jb, 102);
    for (int i = 0; i < 3; ++i) popTag(jb);
    // the new numbers are far behind the old ones and carry no start flag (it was lost)
    for (uint16_t s = 20; s < 27; ++s) CHECK(pushTag(jb, s, 0, 250) == PushResult::Late);
    CHECK(pushTag(jb, 27, 0, 251) == PushResult::Queued);
    pushTag(jb, 28, 0, 252);
    pushTag(jb, 29, 0, 253);
    CHECK(popTag(jb) == 251);
    CHECK(popTag(jb) == 252);
    CHECK(jb.stats().resyncs == 1);
}

TEST(jitter, an_old_double_after_the_end_does_not_start_a_burst) {
    JitterBuffer jb;
    pushTag(jb, 4, kFlagStart);
    pushTag(jb, 5, kFlagEnd);
    CHECK(popTag(jb) == 4);
    bool last = false;
    CHECK(popTag(jb, &last) == 5);
    CHECK(jb.idle());
    CHECK(pushTag(jb, 5, kFlagEnd) == PushResult::Late);    // the end frame came twice
    CHECK(pushTag(jb, 4, 0) == PushResult::Late);
    CHECK(jb.idle());
    CHECK(pushTag(jb, 6, kFlagStart) == PushResult::Queued);   // the next real burst
    CHECK(!jb.idle());
}

// ---------------------------------------------------------------------------------------------------------------------- place

TEST(place, full_volume_close_silent_at_the_range_and_falling_in_between) {
    PlacementConfig cfg;
    const Where l = here(0, 0);
    Placement p = computePlacement(l, here(0, 100), cfg);
    CHECK(p.audible && p.gain == 1.0f);
    p = computePlacement(l, here(0, 2999), cfg);
    CHECK(p.audible && p.gain > 0.0f && p.gain < 0.01f);
    p = computePlacement(l, here(0, 3000), cfg);
    CHECK(!p.audible && p.gain == 0.0f);
    p = computePlacement(l, here(0, 9000), cfg);
    CHECK(!p.audible);
    float prev = 2.0f;
    for (float d = 100; d < 3000; d += 50) {
        const Placement q = computePlacement(l, here(0, d), cfg);
        CHECK(q.gain <= prev);
        prev = q.gain;
    }
    p = computePlacement(l, here(0, 1000), cfg);
    CHECK(p.gain > 0.5f && p.gain < 0.7f);
}

TEST(place, distance_is_three_dimensional) {
    PlacementConfig cfg;
    const Placement flat = computePlacement(here(0, 0, 0), here(0, 1000, 0), cfg);
    const Placement above = computePlacement(here(0, 0, 0), here(0, 1000, 1500), cfg);
    CHECK(above.gain < flat.gain);
}

TEST(place, the_range_is_configurable) {
    PlacementConfig cfg;
    cfg.range = 1000;
    CHECK(computePlacement(here(0, 0), here(0, 999), cfg).audible);
    CHECK(!computePlacement(here(0, 0), here(0, 1000), cfg).audible);
    cfg.range = 5000;
    CHECK(computePlacement(here(0, 0), here(0, 3500), cfg).audible);
}

TEST(place, left_and_right_follow_where_the_listener_faces) {
    PlacementConfig cfg;
    Where l = here(0, 0);
    l.yaw = 0;   // facing north (+Y)
    CHECK(std::fabs(computePlacement(l, here(0, 500), cfg).pan) < 0.01f);     // ahead
    CHECK(std::fabs(computePlacement(l, here(0, -500), cfg).pan) < 0.01f);    // behind
    CHECK(computePlacement(l, here(500, 0), cfg).pan > 0.99f);                // east is on the right
    CHECK(computePlacement(l, here(-500, 0), cfg).pan < -0.99f);              // west is on the left
    l.yaw = 1.57079633f;   // facing east
    CHECK(std::fabs(computePlacement(l, here(500, 0), cfg).pan) < 0.01f);     // ahead now
    CHECK(computePlacement(l, here(0, 500), cfg).pan < -0.99f);               // north is on the left
    CHECK(computePlacement(l, here(0, -500), cfg).pan > 0.99f);
    l.yaw = 3.14159265f;   // facing south: east is on the left
    CHECK(computePlacement(l, here(500, 0), cfg).pan < -0.99f);
}

TEST(place, a_voice_at_your_side_does_not_flip_hard) {
    PlacementConfig cfg;
    const Where l = here(0, 0);
    const float far = computePlacement(l, here(500, 0), cfg).pan;
    const float near60 = computePlacement(l, here(60, 0), cfg).pan;
    const float near10 = computePlacement(l, here(10, 0), cfg).pan;
    CHECK(far > 0.99f);
    CHECK(near60 > 0.4f && near60 < 0.6f);
    CHECK(near10 < 0.1f);
    CHECK(computePlacement(l, here(0, 0), cfg).pan == 0.0f);   // exactly on top of each other
}

TEST(place, other_cells_are_silent_unless_they_share_the_outside) {
    PlacementConfig cfg;
    Where l = here(0, 0, 0, 100, 0, true);
    CHECK(computePlacement(l, here(0, 200, 0, 100, 0, true), cfg).audible);    // same interior
    CHECK(!computePlacement(l, here(0, 200, 0, 101, 0, true), cfg).audible);   // another interior, even right "next" to us in its own coordinates
    CHECK(!computePlacement(l, here(0, 200, 0, 5, 7, false), cfg).audible);    // outside, we are inside
    Where out = here(0, 0, 0, 5, 7, false);
    CHECK(computePlacement(out, here(0, 200, 0, 6, 7, false), cfg).audible);   // neighbouring exterior cell, same worldspace
    CHECK(!computePlacement(out, here(0, 200, 0, 6, 8, false), cfg).audible);  // another worldspace
    CHECK(!computePlacement(out, here(0, 200, 0, 6, 0, false), cfg).audible);  // no worldspace known
}

TEST(place, a_missing_pose_or_broken_numbers_are_silent) {
    PlacementConfig cfg;
    Where l = here(0, 0), s = here(0, 100);
    CHECK(computePlacement(l, s, cfg).audible);
    Where gone = s;
    gone.present = false;
    CHECK(!computePlacement(l, gone, cfg).audible);
    Where blind = l;
    blind.present = false;
    CHECK(!computePlacement(blind, s, cfg).audible);
    Where nan = s;
    nan.pos.x = std::nanf("");
    CHECK(!computePlacement(l, nan, cfg).audible);
    CHECK(globalPlacement(0.5f).audible && globalPlacement(0.5f).pan == 0.0f);
    CHECK(!globalPlacement(0.0f).audible);
}

// -------------------------------------------------------------------------------------------------------------------------- tx

namespace {
struct TxRig {
    FakeCounters counters;
    FakeEncoder enc{&counters};
    TxConfig cfg;
    std::unique_ptr<VoiceTransmitter> tx;
    explicit TxRig(TxConfig c = {}) : cfg(c) { tx.reset(new VoiceTransmitter(cfg, enc, 3)); }
    std::vector<VoiceFrame> frames;
    std::vector<Bytes> keep;
    // feeds one frame of `v`; the decoded frames are appended to `frames`
    size_t feed(int16_t v, bool key, bool muted = false) {
        std::vector<Bytes> out;
        const auto pcm = level(v);
        tx->feed(pcm.data(), key, muted, &out);
        for (Bytes& b : out) {
            keep.push_back(std::move(b));
            VoiceFrame f;
            decodeVoice(keep.back().data(), keep.back().size(), &f);
            frames.push_back(f);
        }
        return out.size();
    }
    static int16_t levelOf(const VoiceFrame& f) { return static_cast<int16_t>(f.payload[0] | (f.payload[1] << 8)); }
};
}  // namespace

TEST(tx, push_to_talk_sends_only_while_the_key_is_held_plus_a_tail) {
    TxRig r;
    for (int i = 0; i < 4; ++i) CHECK(r.feed(1000, false) == 0);
    CHECK(!r.tx->talking());
    for (int i = 0; i < 5; ++i) CHECK(r.feed(1000, true) == 1);
    CHECK(r.tx->talking());
    for (int i = 0; i < 12; ++i) CHECK(r.feed(1000, false) == 1);   // the tail
    CHECK(!r.tx->talking());
    for (int i = 0; i < 4; ++i) CHECK(r.feed(1000, false) == 0);
    REQUIRE(r.frames.size() == 17);
    CHECK(r.frames[0].flags == kFlagStart);
    for (size_t i = 1; i < 16; ++i) CHECK(r.frames[i].flags == 0);
    CHECK(r.frames[16].flags == kFlagEnd);
    for (size_t i = 0; i < r.frames.size(); ++i) {
        CHECK(r.frames[i].seq == i);   // every frame has its own number
        CHECK(r.frames[i].speaker == 3);
        CHECK(TxRig::levelOf(r.frames[i]) == 1000);
    }
    CHECK(r.tx->stats().bursts == 1);
}

TEST(tx, pressing_again_during_the_tail_goes_on_in_the_same_burst) {
    TxRig r;
    for (int i = 0; i < 2; ++i) r.feed(500, true);
    for (int i = 0; i < 5; ++i) r.feed(500, false);
    for (int i = 0; i < 2; ++i) r.feed(500, true);
    for (int i = 0; i < 12; ++i) r.feed(500, false);
    int starts = 0, ends = 0;
    for (auto& f : r.frames) {
        starts += (f.flags & kFlagStart) ? 1 : 0;
        ends += (f.flags & kFlagEnd) ? 1 : 0;
    }
    CHECK(starts == 1);
    CHECK(ends == 1);
    CHECK(r.frames.size() == 21);
}

TEST(tx, a_second_burst_starts_with_a_start_flag_and_goes_on_numbering) {
    TxRig r;
    r.feed(500, true);
    for (int i = 0; i < 12; ++i) r.feed(500, false);
    r.feed(700, true);
    for (int i = 0; i < 12; ++i) r.feed(700, false);
    REQUIRE(r.frames.size() == 26);
    CHECK(r.frames[13].flags == kFlagStart);
    CHECK(r.frames[13].seq == 13);
    CHECK(r.tx->stats().bursts == 2);
}

TEST(tx, open_mic_opens_on_loudness_with_a_preroll_and_closes_after_the_hang) {
    TxConfig c;
    c.mode = Mode::OpenMic;
    TxRig r(c);
    for (int i = 0; i < 6; ++i) CHECK(r.feed(10, false) == 0);   // the room
    CHECK(r.feed(2000, false) == 4);                              // three frames from before, then the voice
    REQUIRE(r.frames.size() == 4);
    CHECK(r.frames[0].flags == kFlagStart);
    CHECK(TxRig::levelOf(r.frames[0]) == 10);
    CHECK(TxRig::levelOf(r.frames[3]) == 2000);
    for (int i = 0; i < 20; ++i) CHECK(r.feed(10, false) == 1);   // the hang
    CHECK(r.frames.back().flags == kFlagEnd);
    CHECK(!r.tx->talking());
    CHECK(r.feed(10, false) == 0);
}

TEST(tx, open_mic_ignores_noise_below_the_gate_and_the_key_forces_it_open) {
    TxConfig c;
    c.mode = Mode::OpenMic;
    c.openThreshold = 1000;
    TxRig r(c);
    for (int i = 0; i < 10; ++i) CHECK(r.feed(900, false) == 0);
    CHECK(r.feed(900, true) >= 1);
}

TEST(tx, a_muted_microphone_sends_nothing_and_muting_cuts_a_sentence_at_once) {
    TxRig r;
    for (int i = 0; i < 5; ++i) CHECK(r.feed(1000, true, true) == 0);
    CHECK(!r.tx->talking());
    for (int i = 0; i < 3; ++i) CHECK(r.feed(1000, true, false) == 1);
    CHECK(r.feed(1000, true, true) == 1);   // the cut: one last frame that says so
    CHECK(r.frames.back().flags == kFlagEnd);
    CHECK(!r.tx->talking());
    CHECK(r.feed(1000, true, true) == 0);
}

TEST(tx, the_microphone_gain_is_applied) {
    TxConfig c;
    c.micGain = 2.0f;
    TxRig r(c);
    r.feed(1000, true);
    REQUIRE(r.frames.size() == 1);
    CHECK(TxRig::levelOf(r.frames[0]) == 2000);
    TxConfig c2;
    c2.micGain = 100.0f;
    TxRig r2(c2);
    r2.feed(1000, true);
    CHECK(TxRig::levelOf(r2.frames[0]) == 32767);   // clipped, not wrapped
}

TEST(tx, an_encoder_that_fails_sends_nothing_and_counts_it) {
    TxRig r;
    r.counters.failEncode = 1;
    CHECK(r.feed(1000, true) == 0);
    CHECK(r.tx->stats().encodeErrors == 1);
}

TEST(tx, loudness_is_the_mean_absolute_sample) {
    const int16_t a[4] = {100, -100, 300, -300};
    CHECK(frameLoudness(a, 4) == 200.0f);
    CHECK(frameLoudness(a, 0) == 0.0f);
    CHECK(frameLoudness(nullptr, 4) == 0.0f);
}

// -------------------------------------------------------------------------------------------------------------------------- rx

TEST(rx, gains_move_smoothly_when_a_voice_appears) {
    FakeCodecs codecs;
    VoiceReceiver rx(codecs);
    uint16_t seq = 0;
    std::vector<int16_t> all;
    stream(rx, 1, 10000, 10, &seq);   // a voice nobody can hear yet (out of range)
    rx.setPlacement(1, nearPlace(0)); // ... comes into range: it fades in
    stream(rx, 1, 10000, 23, &seq, false, &all);
    REQUIRE(all.size() == 23 * kFrameSamples * 2);
    CHECK(std::abs(all[0]) < 50);
    int maxStep = 0;
    for (size_t i = 1; i < all.size() / 2; ++i) maxStep = std::max(maxStep, std::abs(all[i * 2] - all[(i - 1) * 2]));
    CHECK(maxStep < 100);   // 1% of the level per sample at most: no click
    // and when the voice moves from one side to the other
    rx.setPlacement(1, nearPlace(-1.0f));
    stream(rx, 1, 10000, 30, &seq);
    all.clear();
    rx.setPlacement(1, nearPlace(1.0f));
    stream(rx, 1, 10000, 20, &seq, false, &all);
    maxStep = 0;
    for (size_t i = 1; i < all.size() / 2; ++i) {
        maxStep = std::max(maxStep, std::abs(all[i * 2] - all[(i - 1) * 2]));
        maxStep = std::max(maxStep, std::abs(all[i * 2 + 1] - all[(i - 1) * 2 + 1]));
    }
    CHECK(maxStep < 100);
    rx.setPlacement(1, nearPlace(0));
    stream(rx, 1, 10000, 30, &seq);
    // the voice reaches the equal-power centre (0.707) after the ramp
    Peak p = stream(rx, 1, 10000, 20, &seq);
    CHECK(p.l > 6900 && p.l < 7200);
    CHECK(p.r > 6900 && p.r < 7200);
}

TEST(rx, pan_puts_the_voice_on_one_side) {
    FakeCodecs codecs;
    VoiceReceiver rx(codecs);
    uint16_t seq = 0;
    rx.setPlacement(1, nearPlace(1.0f));
    Peak p = stream(rx, 1, 10000, 40, &seq);
    CHECK(p.r > 9800);
    CHECK(p.l < 100);
    rx.setPlacement(1, nearPlace(-1.0f));
    p = stream(rx, 1, 10000, 40, &seq);
    CHECK(p.l > 9800);
    CHECK(p.r < 100);
}

TEST(rx, the_gain_scales_the_volume_and_the_master_volume_scales_everything) {
    FakeCodecs codecs;
    VoiceReceiver rx(codecs);
    uint16_t seq = 0;
    Placement half = nearPlace(1.0f);
    half.gain = 0.5f;
    rx.setPlacement(1, half);
    Peak p = stream(rx, 1, 10000, 40, &seq);
    CHECK(p.r > 4900 && p.r < 5100);
    rx.setMasterVolume(0.5f);
    p = stream(rx, 1, 10000, 40, &seq);
    CHECK(p.r > 2400 && p.r < 2600);
    rx.setMasterVolume(0.0f);
    p = stream(rx, 1, 10000, 40, &seq);
    CHECK(p.r == 0 && p.l == 0);
}

TEST(rx, a_muted_player_is_not_heard_and_unmuting_brings_them_back) {
    FakeCodecs codecs;
    VoiceReceiver rx(codecs);
    uint16_t seq = 0;
    rx.setPlacement(1, nearPlace(0));
    CHECK(stream(rx, 1, 10000, 30, &seq).l > 6900);
    rx.setMuted(1, true);
    CHECK(rx.muted(1));
    CHECK(stream(rx, 1, 10000, 30, &seq).l == 0);
    rx.setMuted(1, false);
    CHECK(stream(rx, 1, 10000, 30, &seq).l > 6900);
}

TEST(rx, a_mute_set_before_the_player_spoke_still_holds) {
    FakeCodecs codecs;
    VoiceReceiver rx(codecs);
    uint16_t seq = 0;
    rx.setMuted(2, true);
    rx.setPlacement(2, nearPlace(0));
    CHECK(stream(rx, 2, 10000, 30, &seq).l == 0);
}

TEST(rx, a_voice_nobody_can_hear_is_not_decoded_but_keeps_its_time) {
    FakeCodecs codecs;
    VoiceReceiver rx(codecs);
    uint16_t seq = 0;
    // no placement: silent
    const Peak p = stream(rx, 1, 10000, 40, &seq);
    CHECK(p.l == 0 && p.r == 0);
    CHECK(codecs.counters.decodes == 0);
    CHECK(rx.stats().droppedInaudible > 30);
    CHECK(rx.depthOf(1) <= 3);   // the buffer is drained at the speed of playback, it does not pile up
    // when the placement turns audible the voice is there at once, in time
    rx.setPlacement(1, nearPlace(0));
    CHECK(stream(rx, 1, 10000, 30, &seq).l > 6900);
}

TEST(rx, two_voices_add_up_and_a_loud_sum_is_clipped_not_wrapped) {
    FakeCodecs codecs;
    VoiceReceiver rx(codecs);
    rx.setPlacement(1, nearPlace(0));
    rx.setPlacement(2, nearPlace(0));
    uint16_t s1 = 0, s2 = 0;
    std::vector<int16_t> buf(kFrameSamples * 2);
    for (int b = 0; b < 60; ++b) {
        const Bytes f = fakeFrame(30000);
        rx.push(1, s1, s1 == 0 ? kFlagStart : 0, f.data(), f.size());
        rx.push(2, s2, s2 == 0 ? kFlagStart : 0, f.data(), f.size());
        ++s1;
        ++s2;
        rx.render(buf.data(), kFrameSamples);
    }
    for (size_t i = 0; i < buf.size(); ++i) CHECK(buf[i] == 32767);   // 2 x 21213 clipped
}

TEST(rx, each_voice_has_its_own_side) {
    FakeCodecs codecs;
    VoiceReceiver rx(codecs);
    rx.setPlacement(1, nearPlace(-1.0f));
    rx.setPlacement(2, nearPlace(1.0f));
    uint16_t s1 = 0, s2 = 0;
    std::vector<int16_t> buf(kFrameSamples * 2);
    for (int b = 0; b < 60; ++b) {
        const Bytes f1 = fakeFrame(8000), f2 = fakeFrame(2000);
        rx.push(1, s1, s1 == 0 ? kFlagStart : 0, f1.data(), f1.size());
        rx.push(2, s2, s2 == 0 ? kFlagStart : 0, f2.data(), f2.size());
        ++s1;
        ++s2;
        rx.render(buf.data(), kFrameSamples);
    }
    CHECK(buf[kFrameSamples] > 7900 && buf[kFrameSamples] < 8100);          // left: voice 1
    CHECK(buf[kFrameSamples + 1] > 1900 && buf[kFrameSamples + 1] < 2100);  // right: voice 2
    CHECK(rx.speakerCount() == 2);
}

TEST(rx, talking_is_true_while_frames_play_and_false_a_moment_after) {
    FakeCodecs codecs;
    VoiceReceiver rx(codecs);
    rx.setPlacement(1, nearPlace(0));
    uint16_t seq = 0;
    CHECK(!rx.talking(1));
    stream(rx, 1, 10000, 10, &seq, true);   // a burst that ends
    CHECK(rx.talking(1));
    std::vector<int16_t> buf(kFrameSamples * 2);
    for (int i = 0; i < 20; ++i) rx.render(buf.data(), kFrameSamples);
    CHECK(!rx.talking(1));
}

TEST(rx, a_lost_frame_is_concealed_by_the_decoder_and_the_end_resets_it) {
    FakeCodecs codecs;
    VoiceReceiver rx(codecs);
    rx.setPlacement(1, nearPlace(0));
    std::vector<int16_t> buf(kFrameSamples * 2);
    const Bytes f = fakeFrame(5000);
    for (int si : {0, 1, 3, 4, 5}) { const uint16_t s = static_cast<uint16_t>(si); rx.push(1, s, s == 0 ? kFlagStart : (s == 5 ? kFlagEnd : 0), f.data(), f.size()); }
    for (int i = 0; i < 8; ++i) rx.render(buf.data(), kFrameSamples);
    CHECK(codecs.counters.concealed == 1);
    CHECK(rx.stats().concealed == 1);
    CHECK(codecs.counters.resets == 1);   // the burst ended
}

TEST(rx, a_corrupt_frame_is_counted_and_plays_as_silence) {
    FakeCodecs codecs;
    VoiceReceiver rx(codecs);
    rx.setPlacement(1, nearPlace(0));
    std::vector<int16_t> buf(kFrameSamples * 2);
    const uint8_t bad[5] = {1, 2, 3, 4, 5};   // the fake decoder wants exactly 3 bytes
    for (uint16_t s = 0; s < 6; ++s) rx.push(1, s, s == 0 ? kFlagStart : 0, bad, sizeof bad);
    for (int i = 0; i < 6; ++i) rx.render(buf.data(), kFrameSamples);
    CHECK(rx.stats().decodeErrors > 0);
    for (int16_t v : buf) CHECK(v == 0);
}

TEST(rx, no_more_than_a_party_of_speakers_and_no_unknown_slots) {
    FakeCodecs codecs;
    VoiceReceiver rx(codecs);
    const Bytes f = fakeFrame(100);
    CHECK(rx.push(kMaxSlots, 0, 0, f.data(), f.size()) == PushResult::Late);
    CHECK(rx.speakerCount() == 0);
    for (uint8_t w = 0; w < kMaxSlots; ++w) rx.push(w, 0, 0, f.data(), f.size());
    CHECK(rx.speakerCount() == kMaxSlots);
}

// ---------------------------------------------------------------------------------------------------------------------- session

namespace {
void everyoneAudible(Net& n, int machines) {
    for (int i = 0; i < machines; ++i)
        for (int j = 0; j < machines; ++j)
            if (i != j) n.s(i).rx().setPlacement(static_cast<uint8_t>(j), nearPlace(0));
}
// One frame period on every machine: `talker` (if >= 0) feeds the microphone, the network delivers, everybody plays a block.
void tick(Net& n, int machines, int talker, int16_t v, bool key, std::vector<Peak>* peaks, uint64_t nowMs = 1000) {
    if (talker >= 0) {
        const auto pcm = level(v);
        n.s(talker).feedMic(pcm.data(), key, false);
    }
    n.deliver(nowMs);
    if (peaks) peaks->assign(machines, Peak{});
    for (int i = 0; i < machines; ++i) {
        const Peak p = renderBlocks(n.s(i), 1);
        if (peaks) (*peaks)[i] = p;
    }
}
}  // namespace

TEST(session, a_guest_is_heard_by_the_host_and_the_other_guest_but_not_by_itself) {
    Net n(2);
    everyoneAudible(n, 3);
    std::vector<Peak> pk, best(3);
    for (int f = 0; f < 30; ++f) {
        tick(n, 3, 1, 8000, true, &pk);
        for (int i = 0; i < 3; ++i) best[i] = Peak{std::max(best[i].l, pk[i].l), std::max(best[i].r, pk[i].r)};
    }
    CHECK(best[0].l > 5000);   // the host hears guest 1 (0.707 x 8000 centred)
    CHECK(best[2].l > 5000);   // and so does guest 2, relayed
    CHECK(best[1].l == 0 && best[1].r == 0);
    CHECK(n.s(1).stats().ownEcho == 0);
    CHECK(n.s(0).stats().relayed > 0);
}

TEST(session, the_host_relays_to_everybody_but_the_speaker) {
    Net n(2);
    everyoneAudible(n, 3);
    for (int f = 0; f < 10; ++f) tick(n, 3, 1, 8000, true, nullptr);
    int toGuest2 = 0, toGuest1FromHost = 0;
    for (auto& p : n.log) {
        if (p.from == 0 && p.to == 2) ++toGuest2;
        if (p.from == 0 && p.to == 1) ++toGuest1FromHost;
    }
    CHECK(toGuest2 == 10);
    CHECK(toGuest1FromHost == 0);   // never back to the one who spoke
}

TEST(session, the_host_voice_reaches_every_guest_and_a_guest_sends_to_the_host_only) {
    Net n(2);
    everyoneAudible(n, 3);
    std::vector<Peak> pk, best(3);
    for (int f = 0; f < 30; ++f) {
        tick(n, 3, 0, 6000, true, &pk);
        for (int i = 0; i < 3; ++i) best[i] = Peak{std::max(best[i].l, pk[i].l), std::max(best[i].r, pk[i].r)};
    }
    CHECK(best[1].l > 4000 && best[2].l > 4000);
    CHECK(best[0].l == 0);
    for (int f = 0; f < 3; ++f) tick(n, 3, 2, 6000, true, nullptr);
    for (auto& p : n.log) if (p.from == 2) CHECK(p.to == 0);   // a guest never talks to another guest directly
}

TEST(session, a_frame_in_your_own_name_coming_back_is_dropped) {
    Net n(2);
    everyoneAudible(n, 3);
    const Bytes f = fakeFrame(5000);
    const Bytes pkt = encodeVoice(1, 0, kFlagStart, f.data(), f.size());
    n.s(1).onDatagram(0, pkt.data(), pkt.size(), 1000);
    CHECK(n.s(1).stats().ownEcho == 1);
    CHECK(n.s(1).rx().speakerCount() == 0);
}

TEST(session, a_guest_cannot_speak_in_somebody_elses_name) {
    Net n(2);
    everyoneAudible(n, 3);
    const Bytes f = fakeFrame(5000);
    const Bytes pkt = encodeVoice(2, 0, kFlagStart, f.data(), f.size());   // from guest 1, claiming to be guest 2
    n.s(0).onDatagram(1, pkt.data(), pkt.size(), 1000);
    CHECK(n.s(0).stats().forged == 1);
    CHECK(n.s(0).rx().speakerCount() == 0);
    CHECK(n.s(0).stats().relayed == 0);
}

TEST(session, a_guest_only_listens_to_the_host) {
    Net n(2);
    const Bytes f = fakeFrame(5000);
    const Bytes pkt = encodeVoice(2, 0, kFlagStart, f.data(), f.size());
    n.s(1).onDatagram(2, pkt.data(), pkt.size(), 1000);   // straight from guest 2
    CHECK(n.s(1).stats().wrongSource == 1);
    CHECK(n.s(1).rx().speakerCount() == 0);
    n.s(1).onDatagram(0, pkt.data(), pkt.size(), 1000);   // relayed by the host: fine
    CHECK(n.s(1).rx().speakerCount() == 1);
}

TEST(session, a_flood_from_one_guest_is_limited) {
    Net n(2);
    const Bytes f = fakeFrame(5000);
    for (uint16_t s = 0; s < 300; ++s) {
        const Bytes pkt = encodeVoice(1, s, s == 0 ? kFlagStart : 0, f.data(), f.size());
        n.s(0).onDatagram(1, pkt.data(), pkt.size(), 5000);   // all in the same millisecond
    }
    CHECK(n.s(0).stats().limited >= 200);
    CHECK(n.s(0).stats().relayed <= VoiceLimiter::kPacketsPerSecond);
    // a normal talker (50 frames a second) is never limited
    Net m(1);
    for (uint16_t s = 0; s < 150; ++s) {
        const Bytes pkt = encodeVoice(1, s, s == 0 ? kFlagStart : 0, f.data(), f.size());
        m.s(0).onDatagram(1, pkt.data(), pkt.size(), 5000 + s * 20);
    }
    CHECK(m.s(0).stats().limited == 0);
}

TEST(session, garbage_is_counted_and_ignored) {
    Net n(1);
    const uint8_t junk[9] = {kChannelVoice, 9, 0, 0, 0, 0, 2, 1, 1};
    n.s(0).onDatagram(1, junk, sizeof junk, 1000);
    n.s(0).onDatagram(1, nullptr, 0, 1000);
    CHECK(n.s(0).stats().malformed == 2);
    CHECK(n.s(0).rx().speakerCount() == 0);
}

TEST(session, a_player_can_mute_one_other_player_without_affecting_the_rest) {
    Net n(2);
    everyoneAudible(n, 3);
    n.s(0).rx().setMuted(1, true);
    std::vector<Peak> pk, best(3);
    for (int f = 0; f < 40; ++f) {
        tick(n, 3, 1, 8000, true, &pk);
        for (int i = 0; i < 3; ++i) best[i] = Peak{std::max(best[i].l, pk[i].l), std::max(best[i].r, pk[i].r)};
    }
    // (the mute fades with the gain smoothing: after 40 blocks it is long gone)
    std::vector<Peak> last;
    tick(n, 3, 1, 8000, true, &last);
    CHECK(last[0].l == 0);       // muted at the host
    CHECK(last[2].l > 5000);     // guest 2 still hears
}

TEST(session, a_voice_is_heard_from_where_the_speaker_stands) {
    Net n(1);
    const PlacementConfig cfg;
    Where me = here(0, 0);   // the host's player, facing north
    struct Case { Where at; const char* what; };
    uint64_t clock = 1000;
    auto loudness = [&](const Where& at) {
        n.s(0).rx().setPlacement(1, computePlacement(me, at, cfg));
        Peak p;
        for (int f = 0; f < 50; ++f) {
            const auto pcm = level(10000);
            n.s(1).feedMic(pcm.data(), true, false);
            clock += 20;
            n.deliver(clock);
            p = renderBlocks(n.s(0), 1);
        }
        return p;
    };
    const Peak close = loudness(here(0, 100));       // 1.4 m ahead: full volume, centred
    CHECK(close.l > 6500 && close.r > 6500 && std::abs(close.l - close.r) < 300);
    const Peak east = loudness(here(1000, 0));       // to the right and further: quieter, on the right
    std::printf("close %d %d east %d %d\n", close.l, close.r, east.l, east.r);
    CHECK(east.r > east.l * 8);
    CHECK(east.r < close.r);
    const Peak west = loudness(here(-1000, 0));
    CHECK(west.l > west.r * 8);
    const Peak far = loudness(here(2500, 0));
    CHECK(far.r < east.r);
    const Peak gone = loudness(here(5000, 0));        // out of range
    CHECK(gone.l == 0 && gone.r == 0);
    const Peak wall = loudness(here(0, 100, 0, 77, 0, true));   // another cell, an interior
    CHECK(wall.l == 0 && wall.r == 0);
    me.yaw = 1.57079633f;                              // turn to face east: the voice from the east is now ahead
    const Peak turned = loudness(here(1000, 0));
    CHECK(std::abs(turned.l - turned.r) < 300);
}

TEST(session, a_player_who_left_is_forgotten) {
    Net n(2);
    everyoneAudible(n, 3);
    for (int f = 0; f < 5; ++f) tick(n, 3, 1, 8000, true, nullptr);
    CHECK(n.s(2).rx().speakerCount() == 1);
    n.s(2).playerLeft(1);
    CHECK(n.s(2).rx().speakerCount() == 0);
}

TEST(session, lost_frames_are_concealed_and_the_voice_goes_on) {
    Net n(1);
    everyoneAudible(n, 2);
    int count = 0;
    n.drop_ = [&count](int, int, const Bytes&) { return (++count % 5) == 0; };
    std::vector<Peak> pk;
    int heard = 0;
    for (int f = 0; f < 80; ++f) {
        tick(n, 2, 1, 8000, true, &pk);
        if (pk[0].l > 3000) ++heard;
    }
    CHECK(heard > 40);
    CHECK(n.s(0).rx().stats().concealed > 5);
}

TEST(session, a_stall_on_the_network_is_not_played_back_late) {
    Net n(1);
    everyoneAudible(n, 2);
    // the guest talks for 100 frames while the host's player does not hear a thing (no playback), then the audio runs again
    for (int f = 0; f < 100; ++f) {
        const auto pcm = level(8000);
        n.s(1).feedMic(pcm.data(), true, false);
        n.deliver(1000 + static_cast<uint64_t>(f) * 20);
    }
    const auto pcm = level(8000);
    for (int f = 0; f < 12; ++f) n.s(1).feedMic(pcm.data(), false, false);   // the tail and the end flag
    n.deliver(5000);
    int blocks = 0;
    std::vector<int16_t> all;
    for (int i = 0; i < 120; ++i) {
        const Peak p = renderBlocks(n.s(0), 1);
        if (p.l == 0 && p.r == 0 && i > 5) break;
        ++blocks;
    }
    CHECK(blocks <= 20);   // 12 frames of cap plus the ramp, not 112
}

TEST(session, no_codec_means_no_sending_but_no_crash) {
    struct NoCodec : IVoiceCodecFactory {
        std::unique_ptr<IVoiceEncoder> makeEncoder() override { return nullptr; }
        std::unique_ptr<IVoiceDecoder> makeDecoder() override { return nullptr; }
    } none;
    std::vector<Bytes> sent;
    SessionConfig c;
    c.self = 1;
    VoiceSession s(c, none, [&sent](uint8_t, const Bytes& b) { sent.push_back(b); }, nullptr);
    const auto pcm = level(1000);
    s.feedMic(pcm.data(), true, false);
    CHECK(sent.empty());
    const Bytes f = fakeFrame(5);
    const Bytes pkt = encodeVoice(0, 0, 0, f.data(), f.size());
    s.onDatagram(0, pkt.data(), pkt.size(), 0);
    CHECK(s.rx().speakerCount() == 0);
}

// ---------------------------------------------------------------------------------------------------------------------- config

namespace {
IniGetter fromMap(std::map<std::string, std::string> m) {
    return [m](const char* section, const char* key, const char* def) -> std::string {
        if (std::string(section) != "voice") return def;
        auto it = m.find(key);
        return it == m.end() ? std::string(def) : it->second;
    };
}
}  // namespace

TEST(config, defaults_are_push_to_talk_on_a_key_the_game_does_not_use) {
    const VoiceSettings s = readSettings(fromMap({}));
    CHECK(s.enabled);
    CHECK(s.mode == Mode::PushToTalk);
    CHECK(s.key == kVkLAlt);
    CHECK(s.volume == 1.0f && s.micVolume == 1.0f);
    CHECK(s.range == 3000.0f && s.fullRange == 400.0f);
    CHECK(s.bitrate == 24000);
    CHECK(s.hud && s.globalFallback);
    CHECK(s.muted.empty());
    CHECK(s.warnings.empty());
}

TEST(config, every_key_is_read) {
    const VoiceSettings s = readSettings(fromMap({{"enabled", "0"}, {"mode", "open"}, {"key", "F8"}, {"volume", "0.5"}, {"mic_volume", "2"}, {"range", "4000"},
                                                  {"full_range", "300"}, {"bitrate", "32000"}, {"open_threshold", "900"}, {"tail_ms", "400"},
                                                  {"hang_ms", "600"}, {"jitter_ms", "80"}, {"max_delay_ms", "300"}, {"hud", "0"},
                                                  {"global_fallback", "off"}, {"muted", "1, 3"}, {"mic_device", "2"}, {"speaker_device", "1"}}));
    CHECK(!s.enabled);
    CHECK(s.mode == Mode::OpenMic);
    CHECK(s.key == 0x77);
    CHECK(s.volume == 0.5f && s.micVolume == 2.0f);
    CHECK(s.range == 4000.0f && s.fullRange == 300.0f);
    CHECK(s.bitrate == 32000);
    CHECK(s.openThreshold == 900.0f);
    CHECK(s.tailMs == 400 && s.hangMs == 600 && s.jitterMs == 80 && s.maxDelayMs == 300);
    CHECK(!s.hud && !s.globalFallback);
    CHECK(s.muted == std::set<uint8_t>({1, 3}));
    CHECK(s.micDevice == 2 && s.speakerDevice == 1);
    CHECK(s.warnings.empty());
}

TEST(config, bad_values_fall_back_and_are_named) {
    const VoiceSettings s = readSettings(fromMap({{"enabled", "maybe"}, {"mode", "shout"}, {"key", "banana"}, {"volume", "loud"}, {"bitrate", "x"}, {"muted", "1,zz,9"}}));
    CHECK(s.enabled);
    CHECK(s.mode == Mode::PushToTalk);
    CHECK(s.key == kVkLAlt);
    CHECK(s.volume == 1.0f);
    CHECK(s.bitrate == 24000);
    CHECK(s.muted == std::set<uint8_t>({1}));
    for (const char* k : {"enabled", "mode", "key", "volume", "bitrate", "muted"}) CHECK(s.warnings.find(k) != std::string::npos);
}

TEST(config, values_are_clamped_to_a_sane_range) {
    VoiceSettings s = readSettings(fromMap({{"volume", "50"}, {"mic_volume", "-3"}, {"range", "1"}, {"bitrate", "1"}, {"jitter_ms", "0"}}));
    CHECK(s.volume == 2.0f);
    CHECK(s.micVolume == 0.0f);
    CHECK(s.range == 200.0f);
    CHECK(s.bitrate == 6000);
    CHECK(s.jitterMs == 20);
    s = readSettings(fromMap({{"range", "3000"}, {"full_range", "5000"}}));
    CHECK(s.fullRange == 750.0f);   // a full-volume radius past the range is pulled in
    s = readSettings(fromMap({{"jitter_ms", "400"}, {"max_delay_ms", "60"}}));
    CHECK(s.maxDelayMs == 440);     // the cap must leave room for the wait
}

TEST(config, key_names_and_codes) {
    CHECK(parseKey("v") == 'V');
    CHECK(parseKey("V") == 'V');
    CHECK(parseKey("7") == '7');
    CHECK(parseKey("F1") == 0x70);
    CHECK(parseKey("f12") == 0x7B);
    CHECK(parseKey("F24") == 0x87);
    CHECK(parseKey("F25") == 0);
    CHECK(parseKey("LAlt") == 0xA4);
    CHECK(parseKey("rctrl") == 0xA3);
    CHECK(parseKey("Insert") == 0x2D);
    CHECK(parseKey("Num5") == 0x65);
    CHECK(parseKey("Mouse4") == 0x05);
    CHECK(parseKey("0x41") == 0x41);
    CHECK(parseKey("65") == 65);
    CHECK(parseKey("0x1FF") == 0);
    CHECK(parseKey("") == 0);
    CHECK(parseKey("nonsense") == 0);
    for (int vk : {0x41, 0x39, 0x70, 0x87, 0x65, 0xA4, 0x2D, 0x05}) CHECK(parseKey(keyName(vk)) == vk);
}

TEST(config, milliseconds_become_frames_for_the_audio_layers) {
    VoiceSettings s;
    s.tailMs = 240;
    s.hangMs = 400;
    s.jitterMs = 60;
    s.maxDelayMs = 240;
    s.mode = Mode::OpenMic;
    s.openThreshold = 777;
    s.micVolume = 1.5f;
    TxConfig t = txConfigFrom(s);
    CHECK(t.tailFrames == 12 && t.hangFrames == 20);
    CHECK(t.mode == Mode::OpenMic && t.openThreshold == 777.0f && t.micGain == 1.5f);
    s.tailMs = 250;
    CHECK(txConfigFrom(s).tailFrames == 13);   // rounded up
    JitterConfig j = jitterConfigFrom(s);
    CHECK(j.startFrames == 3 && j.maxFrames == 12);
    s.jitterMs = 20;
    s.maxDelayMs = 60;
    j = jitterConfigFrom(s);
    CHECK(j.startFrames == 1 && j.maxFrames == 3);
    s.range = 1234;
    s.fullRange = 56;
    const PlacementConfig p = placementConfigFrom(s);
    CHECK(p.range == 1234.0f && p.fullRange == 56.0f);
    CHECK(!describe(s).empty());
}

// ------------------------------------------------------------------------------------------------------------------------- pcm

TEST(pcm, frames_come_out_in_order_and_whole) {
    PcmQueue q;
    int16_t out[kFrameSamples];
    CHECK(!q.popFrame(out));
    std::vector<int16_t> a(100, 7);
    q.push(a.data(), a.size());
    CHECK(!q.popFrame(out));   // 100 samples are not a frame
    q.push(a.data(), a.size());
    q.push(a.data(), a.size());
    q.push(a.data(), a.size());
    CHECK(q.popFrame(out));
    CHECK(out[0] == 7 && out[kFrameSamples - 1] == 7);
    CHECK(q.size() == 400 - kFrameSamples);
}

TEST(pcm, a_stall_drops_the_oldest_audio_and_keeps_frames_aligned) {
    PcmQueue q;   // 200 ms
    for (int i = 0; i < 30; ++i) {
        const auto f = level(static_cast<int16_t>(i));
        q.push(f.data(), f.size());
    }
    CHECK(q.size() <= kSampleRate / 5);
    CHECK(q.dropped() > 0);
    int16_t out[kFrameSamples];
    REQUIRE(q.popFrame(out));
    CHECK(out[0] >= 20);   // the newest audio survived
    for (size_t i = 0; i < kFrameSamples; ++i) CHECK(out[i] == out[0]);   // and the frame is whole
}

TEST(pcm, two_threads_can_use_it_at_once) {
    PcmQueue q;
    std::atomic<bool> stop{false};
    std::thread producer([&] {
        const auto f = level(3);
        for (int i = 0; i < 5000 && !stop; ++i) q.push(f.data(), f.size());
        stop = true;
    });
    int16_t out[kFrameSamples];
    int got = 0;
    while (!stop || q.size() >= kFrameSamples) {
        if (q.popFrame(out)) {
            ++got;
            CHECK(out[0] == 3 && out[kFrameSamples - 1] == 3);
        }
    }
    producer.join();
    CHECK(got > 0);
}

// ------------------------------------------------------------------------------------------------------------------------- hint

TEST(hint, shows_when_a_burst_starts_and_renews_while_talking) {
    HintGate g;
    CHECK(!g.talking(false, 0));
    CHECK(g.talking(true, 1000));            // the burst starts
    CHECK(!g.talking(true, 1500));
    CHECK(!g.talking(true, 3900));
    CHECK(g.talking(true, 4000));            // still talking 3 s later: renewed
    CHECK(!g.talking(false, 4100));
}

TEST(hint, tapping_the_key_does_not_fill_the_message_queue) {
    HintGate g;
    CHECK(g.talking(true, 1000));
    CHECK(!g.talking(false, 1100));
    CHECK(!g.talking(true, 1200));           // 200 ms after the last one: no
    CHECK(!g.talking(false, 1300));
    CHECK(g.talking(true, 2600));            // 1.6 s after: yes
}

TEST(hint, the_microphone_warning_comes_once) {
    HintGate g;
    CHECK(!g.micFailed(false));
    CHECK(g.micFailed(true));
    CHECK(!g.micFailed(true));
}
