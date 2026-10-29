// enemy-anim: owner-side sampler, corpse watcher and codec, script backend. Mutation twins: FNV_MUT(n), see the number in each comment.
#include "anim_helpers.h"
#include "script_npc_engine.h"

using namespace anim_test;

namespace {

live::ActorSnapshot snapAt(Vec3 vel, float yaw = 0) {
    live::ActorSnapshot s;
    s.netId = kId; s.cell = 1; s.pos = V(100, 100, 0); s.vel = vel; s.rot = V(0, yaw, 0);
    return s;
}

struct SampleRig {
    FakeNpcEngine eng;
    NpcAnimSampler smp;
    uint32_t now = 1000;
    SampleRig() : smp(eng) { eng.add(kId); }
    live::AnimState take(Vec3 vel = {}, float yaw = 0, uint32_t dt = 50) {
        eng.advance(dt); now = eng.now;
        live::ActorSnapshot s = snapAt(vel, yaw);
        smp.sample(kId, s, now);
        return s.anim;
    }
    FakeNpc& npc() { return eng.a[kId]; }
};

}  // namespace

// ---- locomotion code ---------------------------------------------------------------------------------------------------------------------
TEST(sampler, locomotion_code_has_speed_and_direction) {   // twin 6
    SampleRig r;
    CHECK(r.take(V(0, 0, 0)).locomotion == 0);
    CHECK(r.take(V(0, 130, 0)).locomotion == 1);     // walk forward (facing +Y)
    CHECK(r.take(V(0, 300, 0)).locomotion == 5);     // run forward
    CHECK(r.take(V(0, -130, 0)).locomotion == 2);    // walk backward
    CHECK(r.take(V(-130, 0, 0)).locomotion == 3);    // walk left
    CHECK(r.take(V(130, 0, 0)).locomotion == 4);     // walk right
    CHECK(r.take(V(300, 0, 0)).locomotion == 8);     // run right
    // the same velocity is "forward" when the actor faces that way
    CHECK(r.take(V(130, 0, 0), kPi / 2).locomotion == 1);
}

TEST(sampler, a_knocked_down_or_ragdolling_actor_publishes_no_locomotion) {
    SampleRig r;
    r.npc().ragdoll = true;
    CHECK(r.take(V(0, 300, 0)).locomotion == 0);
}

// ---- debounced flags ---------------------------------------------------------------------------------------------------------------------
TEST(sampler, flickering_flags_are_not_published) {   // twin 1
    SampleRig r;
    CHECK(!r.take().weaponDrawn);
    r.npc().weaponOut = true;
    CHECK(!r.take().weaponDrawn);        // one reading is not enough ...
    r.npc().weaponOut = false;
    CHECK(!r.take().weaponDrawn);        // ... and it flickered back
    r.npc().weaponOut = true;
    r.take();
    CHECK(r.take().weaponDrawn);         // two in a row: published
    r.npc().aiming = true;
    CHECK(!r.take().aim);
    CHECK(r.take().aim);
    CHECK(r.smp.stats().flickersAbsorbed >= 1);
}

TEST(sampler, the_first_reading_of_an_actor_is_taken_at_once) {
    SampleRig r;
    r.npc().weaponOut = true;
    CHECK(r.take().weaponDrawn);
}

TEST(sampler, a_creature_never_publishes_weapon_or_aim_or_sneak) {
    SampleRig r;
    r.npc().creature = true; r.npc().weaponOut = true; r.npc().aiming = true; r.npc().sneak = true;
    r.take();
    live::AnimState a = r.take();
    CHECK(!a.weaponDrawn && !a.aim && !a.sneak);
}

TEST(sampler, stance_flags_follow_the_engine) {
    SampleRig r;
    r.npc().sneak = true; r.npc().inAir = true; r.npc().swimming = true; r.npc().talking = true;
    r.take();
    live::AnimState a = r.take();
    CHECK(a.sneak && a.swim && a.talk);
    CHECK(!a.jump);   // swimming is not a jump
    r.npc().swimming = false;
    r.take();
    a = r.take();
    CHECK(a.jump);
}

TEST(sampler, a_fall_is_detected_from_the_vertical_speed_when_the_engine_cannot_say) {
    SampleRig r;
    CHECK(!r.take(V(0, 100, 0)).jump);
    CHECK(!r.take(V(0, 200, 210)).jump);    // a staircase
    CHECK(r.take(V(0, 100, -600)).jump);    // falling off a ledge
    CHECK(r.take(V(0, 0, 500)).jump);       // jumping up
}

TEST(sampler, knocked_and_blocking_are_published) {   // twins 7 and 8
    SampleRig r;
    r.npc().knocked = true;
    r.take();
    CHECK(r.take().knocked);
    r.npc().knocked = false;
    r.npc().actionGroup = kGroupBlockIdle; r.npc().actionStart = r.eng.now; r.npc().actionEnd = 0;
    live::AnimState a = r.take();
    CHECK(a.block && a.action == kGroupBlockIdle);
}

// ---- one-shots ---------------------------------------------------------------------------------------------------------------------------
TEST(sampler, an_attack_is_one_event_with_its_group) {   // twin 5
    SampleRig r;
    r.take();
    CHECK(r.take().actionSeq == 0);
    r.npc().actionGroup = 32; r.npc().actionStart = r.eng.now; r.npc().actionEnd = r.eng.now + 300;   // AttackRight, 300 ms
    live::AnimState first = r.take();
    CHECK(first.action == 32 && first.actionSeq == 1);
    for (int i = 0; i < 4; ++i) {
        live::AnimState a = r.take();
        CHECK(a.action == 32 && a.actionSeq == 1);   // still the same event while it plays
    }
    live::AnimState done;
    for (int i = 0; i < 10; ++i) done = r.take();
    CHECK(done.action == 0 && done.actionSeq == 1);  // the event is over; the counter stays
    CHECK(r.smp.stats().actionsStarted == 1);
}

TEST(sampler, a_short_one_shot_stays_in_the_stream_long_enough) {   // twin 2
    SampleRig r;
    r.take();
    r.npc().actionGroup = kGroupStagger; r.npc().actionStart = r.eng.now; r.npc().actionEnd = r.eng.now + 60;
    int shown = 0, firstSeen = -1;
    for (int i = 0; i < 12; ++i) {
        live::AnimState a = r.take();
        if (a.action == kGroupStagger) { ++shown; if (firstSeen < 0) firstSeen = i; }
    }
    CHECK(shown * 50 >= 250);   // at least actionHoldMs worth of 50 ms snapshots, so one lost datagram cannot skip it
    CHECK(firstSeen == 0);
}

TEST(sampler, the_same_group_played_again_is_a_new_event) {   // twin 3
    SampleRig r;
    r.take();
    r.npc().actionGroup = 32; r.npc().actionStart = r.eng.now; r.npc().actionEnd = r.eng.now + 600;
    r.take();
    for (int i = 0; i < 5; ++i) r.take();            // the swing plays on
    CHECK(r.take().actionSeq == 1);
    r.npc().actionStart = r.eng.now;                 // the owner swings again: the sequence restarts (play time goes back to zero)
    r.npc().actionEnd = r.eng.now + 600;
    live::AnimState a = r.take();
    CHECK(a.action == 32 && a.actionSeq == 2);
}

TEST(sampler, a_new_group_replaces_the_held_one_with_a_new_event) {
    SampleRig r;
    r.take();
    r.npc().actionGroup = 32; r.npc().actionStart = r.eng.now; r.npc().actionEnd = r.eng.now + 600;
    r.take();
    r.npc().actionGroup = kGroupStagger; r.npc().actionStart = r.eng.now; r.npc().actionEnd = r.eng.now + 600;
    live::AnimState a = r.take();
    CHECK(a.action == kGroupStagger && a.actionSeq == 2);
}

TEST(sampler, reload_sets_the_reload_flag_and_keeps_the_weapon_specific_group) {
    SampleRig r;
    r.take();
    r.npc().actionGroup = 179; r.npc().actionStart = r.eng.now; r.npc().actionEnd = r.eng.now + 2000;   // ReloadC
    live::AnimState a = r.take();
    CHECK(a.reload && a.action == 179);
}

TEST(sampler, a_non_action_group_is_not_an_event) {
    SampleRig r;
    r.take();
    r.npc().actionGroup = kGroupTalking; r.npc().actionStart = r.eng.now; r.npc().actionEnd = r.eng.now + 600;   // not a one-shot we replay
    live::AnimState a = r.take();
    CHECK(a.action == 0 && a.actionSeq == 0);
}

// ---- death -------------------------------------------------------------------------------------------------------------------------------
TEST(sampler, a_dead_actor_publishes_a_pose_and_nothing_else) {   // twin 4
    SampleRig r;
    r.take(V(0, 300, 0));
    r.npc().actionGroup = 32; r.npc().actionStart = r.eng.now; r.npc().actionEnd = r.eng.now + 600;
    live::AnimState before = r.take(V(0, 300, 0));
    CHECK(before.locomotion == 5 && before.action == 32);
    r.npc().dead = true; r.npc().ragdoll = true;
    live::AnimState a = r.take(V(0, 300, 0));
    CHECK(a.dead && a.locomotion == 0 && a.action == 0 && a.ragdoll);
    CHECK(a.actionSeq == before.actionSeq);   // the counter survives a resurrect
}

TEST(sampler, an_unreadable_actor_leaves_the_snapshot_alone) {
    FakeNpcEngine eng;
    NpcAnimSampler smp(eng);
    live::ActorSnapshot s = snapAt(V(0, 100, 0));
    s.anim.locomotion = 7; s.anim.weaponDrawn = true;
    CHECK(!smp.sample(99, s, 1000));
    CHECK(s.anim.locomotion == 7 && s.anim.weaponDrawn);
    CHECK(smp.stats().unreadable == 1);
}

// ---- corpse watcher + codec --------------------------------------------------------------------------------------------------------------
TEST(corpse, reported_once_where_the_body_stops) {   // twins 20, 21, 22
    CorpseWatcher w;
    w.begin(kId, 1000);
    CorpsePose p;
    uint32_t t = 1000;
    int reports = 0;
    Vec3 pos = V(0, 0, 0);
    for (int i = 0; i < 100; ++i) {
        t += 100;
        pos.x = std::min(250.0f, pos.x + 20.0f);   // slides for 1.25 s, then rests
        if (w.feed(kId, 1, pos, 0.5f, true, t, &p)) { ++reports; if (reports == 1) CHECK(elapsed(t, 1000) >= 1500); }
    }
    CHECK(reports == 1);
    CHECK(std::fabs(p.pos.x - 250.0f) < 1.0f && !p.forced && p.ragdoll && std::fabs(p.yawRad - 0.5f) < 1e-4f);
    CHECK(!w.watching(kId));
}

TEST(corpse, a_body_that_never_rests_is_reported_after_the_wait) {
    CorpseWatcher w;
    w.begin(kId, 0);
    CorpsePose p;
    bool got = false;
    uint32_t t = 0;
    for (int i = 0; i < 200 && !got; ++i) { t += 100; got = w.feed(kId, 1, V(static_cast<float>(i) * 50.0f, 0, 0), 0, true, t, &p); }
    CHECK(got && p.forced && t >= 8000);
}

TEST(corpse, a_vanished_body_is_reported_where_it_was_last_seen) {
    CorpseWatcher w;
    w.begin(kId, 0);
    CorpsePose p;
    CHECK(!w.feed(kId, 1, V(12, 34, 5), 0.25f, false, 100, &p));
    CHECK(w.flush(kId, 200, &p));
    CHECK(p.forced && p.pos.x == 12 && p.pos.y == 34 && !w.watching(kId));
    w.begin(2, 0);
    CHECK(!w.flush(2, 100, &p));   // never seen: nothing to say
}

TEST(corpse, payload_round_trips_and_rejects_garbage) {
    CorpsePose p; p.cell = 77; p.pos = V(1.5f, -2.5f, 3.25f); p.yawRad = 1.25f; p.ragdoll = true; p.forced = true;
    const std::vector<uint8_t> b = encodeCorpse(p);
    CorpsePose q;
    CHECK(decodeCorpse(b, q));
    CHECK(q.cell == 77 && q.pos.x == 1.5f && q.pos.y == -2.5f && q.pos.z == 3.25f && q.yawRad == 1.25f && q.ragdoll && q.forced);
    for (size_t n = 0; n < b.size(); ++n) { CorpsePose x; CHECK(!decodeCorpse(b.data(), n, x)); }   // every truncation
    std::vector<uint8_t> extra = b; extra.push_back(0);
    CorpsePose x;
    CHECK(!decodeCorpse(extra, x));
    std::vector<uint8_t> wrongKind = b; wrongKind[0] ^= 0x55;
    CHECK(!decodeCorpse(wrongKind, x));
    CorpsePose nan = p; nan.pos.x = std::nanf("");
    CHECK(!decodeCorpse(encodeCorpse(nan), x));
    CHECK(!decodeCorpse(nullptr, 0, x));
}

// ---- the system: who announces the corpse -------------------------------------------------------------------------------------------------
namespace {
struct SysRig {
    FakeNpcEngine eng;
    RecordingNet net;
    NpcAnimSystem sys;
    SysRig() : sys(eng, net) { eng.add(kId, V(100, 100, 0)); }
    void run(uint32_t ms, uint32_t dt = 33) {
        for (uint32_t t = 0; t < ms; t += dt) { eng.advance(dt); sys.tick(eng.now); }
    }
};
}  // namespace

TEST(system, the_owner_announces_the_resting_place_once) {   // twins 20, 21, 22, 60
    SysRig o;
    o.eng.startDeath(kId, V(400, 100, 0));
    const uint32_t t0 = o.eng.now;
    o.sys.onDied(kId, /*mine=*/true, /*fromNetwork=*/false, o.eng.now);
    o.run(6000);
    REQUIRE(o.net.sent.size() == 1);
    CorpsePose p;
    CHECK(decodeCorpse(o.net.sent[0].second, p));
    CHECK(std::fabs(p.pos.x - 400.0f) < 2.0f && o.net.sent[0].first == kId);
    CHECK(o.sys.stats().corpseSent == 1);
    // a machine that does NOT own the actor never announces anything
    SysRig n;
    n.eng.startDeath(kId, V(400, 100, 0));
    n.sys.onDied(kId, /*mine=*/false, /*fromNetwork=*/true, n.eng.now);
    n.run(6000);
    CHECK(n.net.sent.empty());
    (void)t0;
}

TEST(system, a_refused_event_is_retried_for_a_while) {   // twin 61
    SysRig o;
    o.net.refuse = 3;
    o.eng.startDeath(kId, V(400, 100, 0));
    o.sys.onDied(kId, true, false, o.eng.now);
    o.run(6000);
    CHECK(o.net.sent.size() == 1);
    CHECK(o.sys.stats().corpseSendRefused == 3);
}

TEST(system, a_refusal_that_never_ends_is_dropped_after_the_resend_window) {
    SysRig o;
    o.net.refuse = 100000;
    o.eng.startDeath(kId, V(400, 100, 0));
    o.sys.onDied(kId, true, false, o.eng.now);
    o.run(20000);
    CHECK(o.net.sent.empty() && o.sys.stats().corpseDropped == 1);
}

TEST(system, a_body_that_disappears_is_announced_where_it_was) {
    SysRig o;
    o.eng.startDeath(kId, V(400, 100, 0), 5000);
    o.sys.onDied(kId, true, false, o.eng.now);
    o.run(700);
    o.sys.onLost(kId);   // the cell unloaded under it
    o.run(500);
    REQUIRE(o.net.sent.size() == 1);
    CorpsePose p;
    CHECK(decodeCorpse(o.net.sent[0].second, p) && p.forced);
}

TEST(system, network_events_are_decoded_and_bad_ones_counted) {
    SysRig n;
    n.eng.startDeath(kId, V(100, 100, 0));
    n.sys.onDied(kId, false, true, n.eng.now);
    CorpsePose p; p.pos = V(500, 500, 0);
    n.sys.onNetEvent(kId, encodeCorpse(p), n.eng.now);
    n.sys.onNetEvent(kId, {}, n.eng.now);
    n.sys.onNetEvent(kId, {200, 1, 2}, n.eng.now);   // unknown kind (a newer build)
    n.sys.onNetEvent(kId, {kAnimEventCorpse, 1}, n.eng.now);   // truncated
    CHECK(n.sys.stats().netEvents == 4 && n.sys.stats().netEventsBad == 3);
    CHECK(n.sys.driver().deadInfo(kId).havePose);
    const std::string j = n.sys.statsJson();
    CHECK(j.find("\"driver\"") != std::string::npos && j.find("\"corpse_poses\":1") != std::string::npos);
}

// ---- script backend ----------------------------------------------------------------------------------------------------------------------
namespace {
struct FakeScript : INpcScript {
    std::vector<std::string> lines;
    bool ok = true;
    bool Exec(NetId, const std::string& l) override { lines.push_back(l); return ok; }
    std::vector<std::pair<uint32_t, std::string>> refLines;
    bool ExecOnRef(uint32_t ref, const std::string& l) override { refLines.push_back({ref, l}); return ok; }
};
struct FakeAccess : INpcGameAccess {
    Result weapon = Result::Unsupported;
    int weaponCalls = 0;
    bool WorldReady() override { return true; }
    uint32_t WorldEpoch() override { return 1; }
    bool MenuOpen() override { return false; }
    bool ReadActor(NetId, NpcReads&) override { return false; }
    uint32_t Generation3D(NetId) override { return 1; }
    bool GetPose(NetId, CellId&, Vec3&, float&) override { return false; }
    Result SetWeaponOut(NetId, bool) override { ++weaponCalls; return weapon; }
    Result SetAiming(NetId, bool) override { return Result::Unsupported; }
    Result SetGroupRate(NetId, float) override { return Result::Unsupported; }
    Result KnockDown(NetId, float, float) override { return Result::Unsupported; }
    Result GetUp(NetId) override { return Result::Unsupported; }
    uint32_t actorRef = 0x00ABCDEF;
    uint32_t RefIdOf(NetId) override { return actorRef; }
};
}  // namespace

TEST(script, the_lines_sent_for_each_request) {   // twin 70
    FakeScript sc; FakeAccess ga;
    ScriptNpcAnimEngine e(sc, ga);
    CHECK(e.PlayGroup(kId, kGroupForward, true) == Result::Ok);
    CHECK(e.PlayGroup(kId, 32, false) == Result::Ok);
    CHECK(e.PlayGroup(kId, kGroupStagger, false) == Result::Ok);
    CHECK(e.PlayGroup(kId, 250, false) == Result::Refused);    // not in the table: never guessed, nothing sent
    CHECK(e.PlayGroup(kId, -3, false) == Result::Refused);
    CHECK(e.SetSneak(kId, true) == Result::Ok && e.SetSneak(kId, false) == Result::Ok);
    CHECK(e.Kill(kId) == Result::Ok);
    CHECK(e.ReleaseHold(kId) == Result::Ok);
    REQUIRE(sc.lines.size() == 7);
    CHECK(sc.lines[0] == "PlayGroup Forward 1");
    CHECK(sc.lines[1] == "PlayGroup AttackRight 0");
    CHECK(sc.lines[2] == "PlayGroup Stagger 0");
    CHECK(sc.lines[3] == "SetForceSneak 1" && sc.lines[4] == "SetForceSneak 0");
    CHECK(sc.lines[5] == "KillActor" && sc.lines[6] == "PlayGroup Idle 0");
}

TEST(script, the_weapon_goes_through_the_direct_call_first) {
    FakeScript sc; FakeAccess ga;
    ScriptNpcAnimEngine e(sc, ga);
    ga.weapon = Result::Ok;
    CHECK(e.SetWeaponDrawn(kId, true) == Result::Ok);
    CHECK(sc.lines.empty() && ga.weaponCalls == 1);
    ga.weapon = Result::Unsupported;
    CHECK(e.SetWeaponDrawn(kId, true) == Result::Ok);
    CHECK(sc.lines.size() == 1 && sc.lines[0] == "SetAlert 1");
    CHECK(e.SetWeaponDrawn(kId, false) == Result::Ok && sc.lines[1] == "SetAlert 0");
    sc.ok = false;
    CHECK(e.SetWeaponDrawn(kId, true) == Result::Refused);
}

TEST(script, unsupported_requests_are_reported_as_such) {
    FakeScript sc; FakeAccess ga;
    ScriptNpcAnimEngine e(sc, ga);
    CHECK(e.SetGroupRate(kId, 1.2f) == Result::Unsupported);
    CHECK(e.SetAiming(kId, true) == Result::Unsupported);
    CHECK(e.KnockDown(kId, 0, 1) == Result::Unsupported);
    CHECK(e.GetUp(kId) == Result::Unsupported);
    CHECK(sc.lines.empty());
}

TEST(script, a_body_is_placed_with_four_plain_lines) {
    FakeScript sc; FakeAccess ga;
    ScriptNpcAnimEngine e(sc, ga);
    CHECK(e.PlaceCorpse(kId, V(12.5f, -3.25f, 0), kPi / 2) == Result::Ok);
    REQUIRE(sc.lines.size() == 4);
    CHECK(sc.lines[0] == "SetPos X 12.500" && sc.lines[1] == "SetPos Y -3.250" && sc.lines[2] == "SetPos Z 0.000");
    CHECK(sc.lines[3] == "SetAngle Z 90.000");
    sc.lines.clear();
    sc.ok = false;
    CHECK(e.PlaceCorpse(kId, V(1, 2, 3), 0) == Result::Refused);
    CHECK(sc.lines.size() == 1);   // stops at the first refused line
}

TEST(script, number_and_angle_formatting) {
    CHECK(formatNumber(-0.0) == "0.000");
    CHECK(formatNumber(1.0 / 3.0) == "0.333");
    CHECK(formatNumber(NAN) == "0.000");
    CHECK(formatNumber(-12.3456) == "-12.346");
    CHECK(std::fabs(yawRadToDegrees(3 * kPi / 2) + 90.0) < 1e-3);
    CHECK(std::fabs(yawRadToDegrees(-kPi) - 180.0) < 1e-3);
    CHECK(yawRadToDegrees(INFINITY) == 0);
}

TEST(groups, the_table_has_a_name_for_every_id_the_layer_can_send) {
    CHECK(std::string(groupName(kGroupIdle)) == "Idle");
    CHECK(std::string(groupName(kGroupStagger)) == "Stagger");
    CHECK(std::string(groupName(kGroupDeath)) == "Death");
    CHECK(std::string(groupName(kGroupLast)) == "JumpLandRight");
    CHECK(std::string(groupName(kGroupLast + 1)) == "");
    CHECK(std::string(groupName(-1)) == "");
    for (int g = 0; g < kGroupCount; ++g) CHECK_MSG(*groupName(g) != 0, std::to_string(g));
    // classification
    CHECK(isActionGroup(32) && isActionGroup(kGroupStagger) && isActionGroup(177) && isActionGroup(kGroupBlockIdle) && isActionGroup(kGroupDodgeForward));
    CHECK(!isActionGroup(kGroupForward) && !isActionGroup(kGroupIdle) && !isActionGroup(kGroupDeath) && !isActionGroup(kGroupJumpStart) && !isActionGroup(kGroupEquip) && !isActionGroup(kGroupTalking));
    CHECK(isHeldAction(kGroupBlockIdle) && isHeldAction(74) && !isHeldAction(32));
}
