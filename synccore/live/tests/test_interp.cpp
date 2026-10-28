// InterpBuffer tests: interpolation correctness, 100 ms delay, extrapolation cap, snap and smooth thresholds.
#include "minitest.h"
#include "scenarios.h"

using namespace live_test;

TEST(interp, interpolates_between_snapshots) { CHECK(interpMidpoint()); }
TEST(interp, renders_100ms_behind) { CHECK(interpDelayIs100ms()); }
TEST(interp, extrapolation_is_capped_at_250ms) { CHECK(extrapolationCapped()); }
TEST(interp, teleport_over_10m_snaps_without_intermediate_frames) { CHECK(teleportSnaps()); }
TEST(interp, cell_change_snaps) { CHECK(cellChangeSnaps()); }
TEST(interp, shove_over_2m_is_eased_without_overshoot) { CHECK(shoveIsSmoothed()); }
TEST(interp, small_error_follows_big_error_eases) { CHECK(smallErrorFollowsBigErrorEases()); }
TEST(interp, thresholds_are_exact) { CHECK(thresholdBoundaries()); }
TEST(interp, stale_and_duplicate_pushes_ignored) { CHECK(staleAndDuplicatePushIgnored()); }
TEST(interp, teleport_after_a_long_stand_is_a_jump_on_screen_within_three_frames) { CHECK(llTeleportAfterAStandIsAJump()); }
TEST(interp, tick_wraparound) { CHECK(tickWrapIsHandled()); }
TEST(interp, rotation_takes_the_short_arc) { CHECK(rotationTakesShortArc()); }
TEST(interp, slow_far_stream_stays_interpolated) { CHECK(slowStreamInterpolates()); }

TEST(interp, empty_buffer_reports_empty) {
    InterpBuffer ib;
    CHECK(ib.sample(1000).state == InterpState::Empty);
    ib.push(mk(1, 1, 1, 1000, 5), 1030);
    CHECK(ib.sample(1031).state != InterpState::Empty);
    ib.reset();
    CHECK(ib.sample(1100).state == InterpState::Empty);
}

TEST(interp, extrapolation_follows_velocity_inside_the_cap) {
    Feed f;
    f.pushUpTo(10);   // newest tick 1500, 500 units/s
    InterpOut o;
    uint32_t now = 1500 + 30 + 100 - 32;
    uint32_t last = now;
    for (; now <= 1500 + 30 + 100 + 100; now += 16) { o = f.ib.sample(now); last = now; }
    float past = static_cast<float>(last - 130 - 1500);   // ms past the newest snapshot
    CHECK(o.state == InterpState::Extrapolating);
    CHECK(past > 50.0f && past < 250.0f);
    CHECK(std::fabs(o.pos.x - (Feed::xAt(1500) + 500.0f * past / 1000.0f)) < 0.5f);
}

TEST(interp, recovers_smoothly_when_snapshots_resume_after_a_gap) {
    Feed f;
    f.pushUpTo(10);
    // 700 ms without snapshots (partition): the puppet extrapolates, then holds at the cap.
    uint32_t now = 1500 + 30 + 100;
    InterpOut o;
    for (; now <= 1500 + 30 + 800; now += 16) o = f.ib.sample(now);
    CHECK(o.state == InterpState::Holding);
    float prev = o.pos.x;
    // the stream resumes at the sender's current tick (snapshots 11..25 were lost)
    for (uint32_t k = 26; k <= 40; ++k) f.ib.push(f.at(k), f.t0 + k * 50 + f.latency);
    float maxStep = 0;
    uint32_t snaps = 0, smoothFrames = 0;
    uint32_t last = now;
    for (; now <= 3126; now += 16) {
        o = f.ib.sample(now);
        maxStep = std::fmax(maxStep, std::fabs(o.pos.x - prev));
        prev = o.pos.x;
        if (o.snapped) ++snaps;
        if (o.state == InterpState::Smoothing) ++smoothFrames;
        last = now;
    }
    CHECK(snaps == 0);              // the gap is only ~6 m of travel: eased, not popped
    CHECK(smoothFrames > 0);        // the 3 m catch-up is eased
    CHECK(maxStep < 140.0f);        // never a 2 m pop in one frame
    CHECK(std::fabs(o.pos.x - Feed::xAt(last - 130)) < 0.5f);   // and it has caught up
}

TEST(interp, anim_and_ai_follow_the_snapshots) {
    InterpBuffer ib;
    for (uint32_t k = 0; k < 10; ++k) {
        ActorSnapshot s = mk(2, 1, k + 1, 1000 + k * 50, 0);
        s.anim.locomotion = k < 5 ? 1 : 3;
        s.anim.weaponDrawn = k >= 5;
        s.ai.combatTarget = k >= 5 ? 99 : 0;
        ib.push(s, s.tick + 30);
    }
    InterpOut early = ib.sample(1000 + 30 + 100 + 25);
    InterpOut late = ib.sample(1450 + 30 + 100 - 10);
    // sample the later one on a fresh buffer state to avoid any smoothing history: only discrete fields are compared
    CHECK(early.anim.locomotion == 1 && !early.anim.weaponDrawn && early.ai.combatTarget == 0);
    CHECK(late.anim.locomotion == 3 && late.anim.weaponDrawn && late.ai.combatTarget == 99);
}

TEST(interp, actual_pose_in_another_cell_snaps) {
    InterpBuffer ib;
    for (uint32_t k = 0; k <= 60; ++k) ib.push(mk(3, 1, k + 1, 1000 + k * 50, 0), 1000 + k * 50 + 30);
    ib.sample(1130);
    ActualPose elsewhere{99, {0, 0, 0}};
    InterpOut o = ib.sample(1146, &elsewhere);
    CHECK(o.snapped);
}

// low-latency mode (player avatars)
TEST(interp, low_latency_delay_is_a_few_tens_of_ms_not_100) { CHECK(llDelayFollowsJitter()); }
TEST(interp, low_latency_extrapolation_is_damped) { CHECK(llExtrapolationIsDamped()); }
TEST(interp, low_latency_prediction_miss_is_faded_not_popped) { CHECK(llMissIsFaded()); }
TEST(interp, low_latency_render_clock_is_slewed) { CHECK(llClockIsSlewed()); }
