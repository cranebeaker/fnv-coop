// enemy-anim: furniture. The owner's sit / sleep state travels in the actor stream and a copy sits or stands like the owner's actor
// (2026-10-29: the host's NPC stood up and walked off, the guest's copy stayed in the chair pose and slid across the room).
// Sampler (owner side), driver (copy side), script backend and the whole path. Mutation twins: FNV_MUT(n), see the number in each comment.
#include "anim_helpers.h"
#include "anim_sit.h"
#include "script_npc_engine.h"

using namespace anim_test;
using live::SitState;

namespace {

constexpr uint32_t kChair = 0x00001A2Bu, kOtherChair = 0x00001C3Du;

live::ActorSnapshot snapAt(Vec3 vel = {}) {
    live::ActorSnapshot s;
    s.netId = kId; s.cell = 1; s.pos = V(100, 100, 0); s.vel = vel; s.rot = V(0, 0, 0);
    return s;
}

struct SampleRig {
    FakeNpcEngine eng;
    NpcAnimSampler smp;
    uint32_t now = 1000;
    SampleRig() : smp(eng) { eng.add(kId); }
    live::AnimState take(uint32_t dt = 50) {
        eng.advance(dt); now = eng.now;
        live::ActorSnapshot s = snapAt();
        smp.sample(kId, s, now);
        return s.anim;
    }
    FakeNpc& npc() { return eng.a[kId]; }
};

// What the stream shows for a copy: the walker's animation state with the furniture block.
live::InterpOut seated(Walker& w, SitState st, uint32_t furniture, bool bed = false, uint32_t dt = 33, float speed = 0, uint8_t code = 0) {
    w.anim.sit = st; w.anim.furniture = st == SitState::None ? 0 : furniture; w.anim.sleep = st != SitState::None && bed;
    return w.step(dt, speed, code);
}

size_t indexOf(const FakeNpc& n, const std::string& prefix, size_t from = 0) {
    for (size_t i = from; i < n.log.size(); ++i) if (n.log[i].compare(0, prefix.size(), prefix) == 0) return i;
    return n.log.size();
}

// A copy that already sits in `furniture` for `ms`.
void sitDown(Bench& b, Walker& w, uint32_t furniture, uint32_t ms = 1000) {
    for (uint32_t t = 0; t < ms; t += 33) b.frame(seated(w, SitState::Using, furniture));
}

}  // namespace

// ---- the engine's states map to the wire's ---------------------------------------------------------------------------------------------
TEST(sit, engine_sit_sleep_states_map_to_the_wire_states) {   // twin 85 (the bed flag)
    CHECK(sitFromEngine(0) == SitView{});
    CHECK(sitFromEngine(1).state == SitState::Entering && sitFromEngine(2).state == SitState::Entering && sitFromEngine(3).state == SitState::Entering);
    CHECK(sitFromEngine(4).state == SitState::Using && !sitFromEngine(4).bed);
    CHECK(sitFromEngine(5).state == SitState::Leaving && !sitFromEngine(5).bed);
    CHECK(sitFromEngine(6).state == SitState::Entering && sitFromEngine(6).bed && sitFromEngine(8).bed);
    CHECK(sitFromEngine(9).state == SitState::Using && sitFromEngine(9).bed);
    CHECK(sitFromEngine(10).state == SitState::Leaving && sitFromEngine(10).bed);
    CHECK(sitFromEngine(11) == SitView{} && sitFromEngine(-1) == SitView{} && sitFromEngine(0x7FFF) == SitView{});   // a value nobody knows is no pose
    CHECK(sitFromEngine(4).seated() && sitFromEngine(2).seated() && !sitFromEngine(5).seated() && !sitFromEngine(0).seated());
}

// ---- owner side: the sampler -------------------------------------------------------------------------------------------------------------
TEST(sit, the_sampler_publishes_the_sit_state_and_the_furniture) {   // twins 72, 85
    SampleRig r;
    CHECK(r.take().sit == SitState::None);
    r.npc().sitSleep = 4; r.npc().furniture = kChair;
    r.take();
    live::AnimState a = r.take();
    CHECK(a.sit == SitState::Using && a.furniture == kChair && !a.sleep);
    r.npc().sitSleep = 5;
    r.take();
    CHECK(r.take().sit == SitState::Leaving);
    r.npc().sitSleep = 0; r.npc().furniture = 0;
    r.take();
    a = r.take();
    CHECK(a.sit == SitState::None && a.furniture == 0 && !a.sleep);
    r.npc().sitSleep = 9; r.npc().furniture = kOtherChair;   // a bed
    r.take();
    a = r.take();
    CHECK(a.sit == SitState::Using && a.sleep && a.furniture == kOtherChair);
}

TEST(sit, a_flickering_sit_state_is_not_published) {   // twin 71
    SampleRig r;
    r.npc().sitSleep = 4; r.npc().furniture = kChair;
    CHECK(r.take().sit == SitState::Using);        // the first reading of an actor is taken at once
    r.npc().sitSleep = 0;
    CHECK(r.take().sit == SitState::Using);        // one reading is not enough ...
    r.npc().sitSleep = 4;
    CHECK(r.take().sit == SitState::Using);        // ... and it flickered back
    r.npc().sitSleep = 0;
    r.take();
    CHECK(r.take().sit == SitState::None);         // two in a row: published
    CHECK(r.smp.stats().flickersAbsorbed >= 1);
}

TEST(sit, the_furniture_reference_is_kept_while_the_actor_stays_in_it) {   // twin 73
    SampleRig r;
    r.npc().sitSleep = 4; r.npc().furniture = kChair;
    r.take();
    CHECK(r.take().furniture == kChair);
    r.npc().furnitureReadable = false;             // one empty read (the engine could not tell for a moment)
    live::AnimState a = r.take();
    CHECK(a.sit == SitState::Using && a.furniture == kChair);
    r.npc().furnitureReadable = true; r.npc().furniture = kOtherChair;   // a really different one is followed
    CHECK(r.take().furniture == kOtherChair);
    r.npc().sitSleep = 0;
    r.take();
    r.take();
    r.npc().sitSleep = 4; r.npc().furnitureReadable = false;   // sits down again, reference not readable: nothing stale is published
    r.take();
    a = r.take();
    CHECK(a.sit == SitState::Using && a.furniture == 0);
}

TEST(sit, a_dead_actor_publishes_no_furniture) {   // twin 74
    SampleRig r;
    r.npc().sitSleep = 4; r.npc().furniture = kChair;
    r.take();
    CHECK(r.take().sit == SitState::Using);
    r.npc().dead = true;
    live::AnimState a = r.take();
    CHECK(a.dead && a.sit == SitState::None && a.furniture == 0);
}

TEST(sit, an_actor_that_cannot_be_read_for_sit_publishes_standing_and_a_body_on_the_floor_is_not_seated) {
    SampleRig r;
    r.npc().sitSleep = 4; r.npc().furniture = kChair; r.npc().sitReadable = false;
    r.take();
    CHECK(r.take().sit == SitState::None);
    r.npc().sitReadable = true;
    r.take();
    CHECK(r.take().sit == SitState::Using);
    r.npc().ragdoll = true;
    live::AnimState a = r.take();
    CHECK(a.sit == SitState::None && a.furniture == 0);
}

TEST(sit, the_published_state_survives_the_wire) {
    // sampler -> snapshot -> codec -> snapshot: the sit state and the furniture reach the receiver, and a sit-only change is a delta it gets too
    SampleRig r;
    live::SnapshotSender tx;
    live::ReceiverConfig rc; rc.checkSenderIsOwner = false;
    live::SnapshotReceiver rx(rc);
    uint32_t seq = 0;
    auto sampleAndSend = [&]() {
        r.eng.advance(50); r.now = r.eng.now;
        live::ActorSnapshot s = snapAt();
        r.smp.sample(kId, s, r.now);
        s.ownerEpoch = 1; s.seq = ++seq; s.tick = r.now;
        tx.queue(1, s, r.now);
        std::vector<live::ReceivedSnapshot> got;
        for (auto& pkt : tx.flush(1, r.now)) { auto g = rx.receive(1, pkt.data(), pkt.size()); got.insert(got.end(), g.begin(), g.end()); }
        const live::Bytes ack = rx.takeAcks(1);
        if (!ack.empty()) tx.onAckPacket(1, ack.data(), ack.size());
        return got;
    };
    r.npc().sitSleep = 4; r.npc().furniture = kChair;
    auto g1 = sampleAndSend();
    REQUIRE(g1.size() == 1);
    CHECK(g1[0].snap.anim.sit == SitState::Using && g1[0].snap.anim.furniture == kChair);
    r.npc().sitSleep = 5;
    sampleAndSend();                      // the first reading of the new state is absorbed (debounce) ...
    auto g3 = sampleAndSend();            // ... the second is published, and nothing else about the actor changed
    REQUIRE(g3.size() == 1);
    CHECK(g3[0].snap.anim.sit == SitState::Leaving && g3[0].snap.anim.furniture == kChair);
}

// ---- copy side: sitting down -------------------------------------------------------------------------------------------------------------
TEST(sit, a_copy_sits_down_in_the_same_furniture_once) {   // twins 75, 78
    Bench b;
    Walker w;
    b.walk(w, 300, 0, 0);                                           // standing in a plain idle
    for (int i = 0; i < 300; ++i) b.frame(seated(w, SitState::Using, kChair));   // ten seconds in the chair
    CHECK(b.npc().sitSleep == 4 && b.npc().furniture == kChair);
    CHECK(b.npc().sitCalls == 1);                                    // told once, not every frame
    CHECK(b.eng.count(kId, "sit:" + std::to_string(kChair)) == 1);
    CHECK(b.npc().playsWhileInFurniture == 0);                       // no movement group over the pose
    CHECK(b.drv.stats().sitsIssued == 1 && b.drv.info(kId).seated && b.drv.info(kId).furniture == kChair);
}

TEST(sit, a_copy_that_appears_while_the_owner_sits_sits_down_without_a_movement_group) {
    Bench b;
    Walker w;
    for (int i = 0; i < 40; ++i) b.frame(seated(w, SitState::Using, kChair));
    CHECK(b.npc().sitSleep == 4 && b.npc().sitCalls == 1);
    CHECK(b.eng.count(kId, "play:") == 0);                           // not even the baseline idle: the furniture owns the pose from the first frame
}

TEST(sit, a_copy_already_sitting_in_that_furniture_gets_no_command) {   // twin 79 (the first look)
    Bench b;
    b.npc().sitSleep = 4; b.npc().furniture = kChair;                // its own AI (or the save) put it there
    Walker w;
    for (int i = 0; i < 90; ++i) b.frame(seated(w, SitState::Using, kChair));
    CHECK(b.npc().sitCalls == 0 && b.npc().standCalls == 0);
    CHECK(b.eng.count(kId, "play:") == 0);
    CHECK(b.drv.info(kId).seated);
}

TEST(sit, another_piece_of_furniture_is_used_directly) {
    Bench b;
    Walker w;
    sitDown(b, w, kChair);
    for (int i = 0; i < 40; ++i) b.frame(seated(w, SitState::Using, kOtherChair));
    CHECK(b.npc().sitCalls == 2 && b.npc().standCalls == 0 && b.npc().furniture == kOtherChair);
}

TEST(sit, no_furniture_reference_means_no_command_until_it_arrives) {
    Bench b;
    Walker w;
    for (int i = 0; i < 30; ++i) b.frame(seated(w, SitState::Using, 0));
    CHECK(b.npc().sitCalls == 0);
    CHECK(b.drv.stats().sitNoFurniture == 1);                        // counted once, not per frame
    for (int i = 0; i < 30; ++i) b.frame(seated(w, SitState::Using, kChair));
    CHECK(b.npc().sitCalls == 1 && b.npc().sitSleep == 4);
}

TEST(sit, a_sit_the_engine_dropped_is_found_by_the_read_back_and_issued_again) {   // twin 82
    Bench b;
    Walker w;
    sitDown(b, w, kChair, 600);
    CHECK(b.npc().sitCalls == 1);
    b.npc().sitSleep = 0; b.npc().furniture = 0;                     // a menu, a cell change: the engine dropped the pose
    for (int i = 0; i < 90; ++i) b.frame(seated(w, SitState::Using, kChair));
    CHECK(b.npc().sitCalls == 2 && b.npc().sitSleep == 4);
    CHECK(b.drv.stats().sitDrops == 1);
}

TEST(sit, a_command_the_engine_ignores_is_retried_with_backoff_not_hammered) {
    Bench b;
    b.npc().ignoreSitCalls = 2;
    Walker w;
    for (int i = 0; i < 180; ++i) b.frame(seated(w, SitState::Using, kChair));   // six seconds
    CHECK(b.npc().sitSleep == 4);
    CHECK(b.npc().sitCalls == 3);
}

TEST(sit, a_refused_command_backs_off) {
    Bench b;
    b.npc().refuseAll = true;
    Walker w;
    for (int i = 0; i < 90; ++i) b.frame(seated(w, SitState::Using, kChair));
    CHECK(b.npc().sitCalls == 0);
    CHECK(b.eng.sitAttempts >= 2 && b.eng.sitAttempts <= 4);       // 0, 0.4, 1.2, 2.8 s
}

TEST(sit, a_backend_that_cannot_sit_is_asked_once) {
    Bench b;
    b.npc().sitSupported = false;
    Walker w;
    for (int i = 0; i < 90; ++i) b.frame(seated(w, SitState::Using, kChair));
    CHECK(b.drv.stats().unsupported == 1);
    CHECK(b.npc().moveGroup == kGroupIdle);                          // the copy still gets its plain idle
}

TEST(sit, the_switch_turns_the_feature_off) {
    DriverConfig c; c.playSit = false;
    Bench b(c);
    Walker w;
    for (int i = 0; i < 60; ++i) b.frame(seated(w, SitState::Using, kChair));
    CHECK(b.npc().sitCalls == 0 && b.npc().standCalls == 0);
}

TEST(sit, talking_does_not_play_over_the_seated_pose) {   // twin 87
    Bench b;
    Walker w;
    sitDown(b, w, kChair);
    w.anim.talk = true;
    for (int i = 0; i < 30; ++i) b.frame(seated(w, SitState::Using, kChair));
    CHECK(b.eng.plays(kId, kGroupTalking) == 0 && b.npc().playsWhileInFurniture == 0);
}

// ---- copy side: standing up ----------------------------------------------------------------------------------------------------------------
TEST(sit, the_copy_leaves_the_furniture_before_any_movement_group_plays) {   // twins 76, 77
    Bench b;
    Walker w;
    sitDown(b, w, kChair);
    CHECK(b.npc().sitSleep == 4);
    // the owner stood up and walked off; the datagrams in between were lost, so the copy hears "standing and walking" at once
    for (int i = 0; i < 60; ++i) b.frame(seated(w, SitState::None, 0, false, 33, 130, 1));
    const FakeNpc& n = b.npc();
    CHECK(n.standCalls == 1 && n.sitSleep == 0 && n.furniture == 0);
    CHECK(n.playsWhileInFurniture == 0);                             // nothing was played while it was still in (or getting out of) the furniture
    const size_t iStand = indexOf(n, "stand:"), iWalk = indexOf(n, "play:" + std::to_string(kGroupForward) + ":");
    CHECK(iStand < n.log.size() && iWalk < n.log.size() && iStand < iWalk);   // sit -> stand -> move
    CHECK(n.moveGroup == kGroupForward);
    CHECK(b.drv.stats().standsConfirmed == 1 && !b.drv.info(kId).seated);
}

TEST(sit, leaving_is_already_standing_up) {   // twin 81
    Bench b;
    Walker w;
    sitDown(b, w, kChair);
    b.frame(seated(w, SitState::Leaving, kChair));
    CHECK(b.npc().standCalls == 1);
}

TEST(sit, the_stand_up_is_one_command_and_the_pose_stays_locked_until_it_is_done) {
    Bench b;
    Walker w;
    sitDown(b, w, kChair);
    CHECK(!b.drv.poseLocked(kId));
    b.frame(seated(w, SitState::None, 0));
    CHECK(b.drv.poseLocked(kId) && b.drv.info(kId).standPending);
    for (int i = 0; i < 30; ++i) b.frame(seated(w, SitState::None, 0));
    CHECK(!b.drv.poseLocked(kId));
    CHECK(b.npc().standCalls == 1 && b.npc().restrainCalls == 0);    // the gentle way: the hold was never touched
    CHECK(b.npc().moveGroup == kGroupIdle);
}

TEST(sit, a_copy_that_loads_sitting_while_the_owner_stands_is_stood_up) {   // twins 76, 79
    Bench b;
    b.npc().sitSleep = 4; b.npc().furniture = kChair;                // the save had it in the chair
    Walker w;
    for (int i = 0; i < 60; ++i) b.frame(seated(w, SitState::None, 0));
    CHECK(b.npc().sitCalls == 0 && b.npc().standCalls == 1);
    CHECK(b.npc().sitSleep == 0 && b.npc().moveGroup == kGroupIdle);
    CHECK(b.npc().playsWhileInFurniture == 0);
}

TEST(sit, a_copy_that_sits_down_on_its_own_later_is_found_by_the_read_back) {   // twin 82
    Bench b;
    Walker w;
    b.walk(w, 600, 0, 0);
    b.npc().sitSleep = 4; b.npc().furniture = kChair;                // something sat it down behind our back
    for (int i = 0; i < 90; ++i) b.frame(seated(w, SitState::None, 0));
    CHECK(b.npc().standCalls == 1 && b.npc().sitSleep == 0);
    CHECK(b.drv.stats().sitDrifts >= 1);
}

TEST(sit, a_gentle_stand_up_that_does_nothing_is_escalated_and_the_hold_goes_back) {   // twin 83
    Bench b;
    b.npc().minStandAttempt = 1;                                     // the gentle way does not work on this actor
    Walker w;
    sitDown(b, w, kChair);
    for (int i = 0; i < 130; ++i) b.frame(seated(w, SitState::None, 0));
    const FakeNpc& n = b.npc();
    CHECK(n.standCalls == 2);
    CHECK(indexOf(n, "stand:0") < indexOf(n, "stand:1") && indexOf(n, "stand:1") < n.log.size());
    CHECK(n.sitSleep == 0 && n.moveGroup == kGroupIdle);
    CHECK(n.restrainCalls == 1 && n.restrained);                     // the escalation let the AI run; the hold is back
    CHECK(n.playsWhileInFurniture == 0);
    CHECK(b.drv.stats().restrains == 1);
}

TEST(sit, a_stand_up_that_never_works_gives_up_and_tries_again_later_not_for_ever_per_frame) {   // twin 80
    Bench b;
    b.npc().ignoreStandCalls = 1000;
    Walker w;
    sitDown(b, w, kChair);
    for (int i = 0; i < 360; ++i) b.frame(seated(w, SitState::None, 0));   // twelve seconds
    CHECK(b.drv.stats().standGiveUps >= 1);
    CHECK(b.npc().standCalls >= 3 && b.npc().standCalls <= 8);
    b.npc().ignoreStandCalls = 0;
    for (int i = 0; i < 240; ++i) b.frame(seated(w, SitState::None, 0));   // the engine finally lets go
    CHECK(b.npc().sitSleep == 0 && b.npc().moveGroup == kGroupIdle);
    CHECK(b.npc().restrained);                                       // whatever the escalations did, the hold is back
}

TEST(sit, a_backend_that_cannot_read_the_sit_state_lets_the_movement_group_go_after_the_settle_time) {
    Bench b;
    b.npc().sitReadable = false;
    Walker w;
    sitDown(b, w, kChair);
    for (int i = 0; i < 60; ++i) b.frame(seated(w, SitState::None, 0, false, 33, 130, 1));
    CHECK(b.npc().standCalls == 1);
    CHECK(b.npc().moveGroup == kGroupForward && b.drv.stats().standsConfirmed == 1);
}

TEST(sit, a_stand_up_the_backend_cannot_do_is_asked_once) {
    Bench b;
    Walker w;
    sitDown(b, w, kChair);
    b.npc().sitSupported = false;
    for (int i = 0; i < 60; ++i) b.frame(seated(w, SitState::None, 0, false, 33, 130, 1));
    CHECK(b.eng.standAttempts == 1 && b.npc().standCalls == 0);
    CHECK(b.npc().moveGroup == kGroupForward);                       // it cannot stand the copy up, but it does not freeze it either
}

// ---- nothing while not driving -----------------------------------------------------------------------------------------------------------
TEST(sit, nothing_is_sent_while_a_menu_is_open) {   // twin 84
    Bench b;
    Walker w;
    b.walk(w, 300, 0, 0);
    const uint64_t before = b.eng.totalCommands;
    b.eng.menuOpen = true;
    for (int i = 0; i < 40; ++i) b.frame(seated(w, SitState::Using, kChair));
    CHECK(b.eng.totalCommands == before && b.npc().sitCalls == 0);
    b.eng.closeMenu();
    for (int i = 0; i < 30; ++i) b.frame(seated(w, SitState::Using, kChair));
    CHECK(b.npc().sitCalls == 1);
}

TEST(sit, nothing_is_sent_when_no_game_is_loaded) {
    Bench b;
    Walker w;
    b.walk(w, 300, 0, 0);
    b.eng.worldReady = false;
    for (int i = 0; i < 40; ++i) b.frame(seated(w, SitState::Using, kChair));
    CHECK(b.npc().sitCalls == 0);
}

TEST(sit, an_actor_that_is_no_longer_driven_gets_no_furniture_commands) {
    Bench b;
    Walker w;
    sitDown(b, w, kChair);
    const uint64_t before = b.eng.totalCommands;
    b.drv.unload(kId);                                               // the reference went away locally
    for (int i = 0; i < 90; ++i) { b.eng.advance(33); b.drv.tick(b.eng.now); }
    CHECK(b.eng.totalCommands == before);
    // and one we took over: only the release goes out
    Bench c;
    Walker w2;
    sitDown(c, w2, kChair);
    const uint64_t before2 = c.eng.totalCommands;
    c.drv.release(kId);
    for (int i = 0; i < 90; ++i) { c.eng.advance(33); c.drv.tick(c.eng.now); }
    CHECK(c.eng.totalCommands == before2 + 1 && c.npc().releaseCalls == 1);
}

TEST(sit, a_dead_actor_is_not_driven_into_furniture) {
    Bench b;
    Walker w;
    b.walk(w, 300, 0, 0);
    b.drv.onDeath(kId, b.now, true);
    for (int i = 0; i < 40; ++i) b.frame(seated(w, SitState::Using, kChair));
    CHECK(b.npc().sitCalls == 0);
}

// ---- the script backend -------------------------------------------------------------------------------------------------------------------
namespace {
struct SitScript : INpcScript {
    std::vector<std::string> lines;
    std::vector<std::pair<uint32_t, std::string>> refLines;
    bool ok = true;
    bool Exec(NetId, const std::string& l) override { lines.push_back(l); return ok; }
    bool ExecOnRef(uint32_t ref, const std::string& l) override { refLines.push_back({ref, l}); return ok; }
};
struct SitAccess : INpcGameAccess {
    uint32_t actorRef = 0x00ABCDEFu;
    bool WorldReady() override { return true; }
    uint32_t WorldEpoch() override { return 1; }
    bool MenuOpen() override { return false; }
    bool ReadActor(NetId, NpcReads&) override { return false; }
    uint32_t Generation3D(NetId) override { return 1; }
    bool GetPose(NetId, CellId&, Vec3&, float&) override { return false; }
    Result SetWeaponOut(NetId, bool) override { return Result::Unsupported; }
    Result SetAiming(NetId, bool) override { return Result::Unsupported; }
    Result SetGroupRate(NetId, float) override { return Result::Unsupported; }
    Result KnockDown(NetId, float, float) override { return Result::Unsupported; }
    Result GetUp(NetId) override { return Result::Unsupported; }
    uint32_t RefIdOf(NetId) override { return actorRef; }
};
}  // namespace

TEST(sit, the_lines_sent_for_sitting_and_standing) {   // twin 86
    SitScript sc; SitAccess ga;
    ScriptNpcAnimEngine e(sc, ga);
    CHECK(e.SitIn(kId, kChair) == Result::Ok);
    REQUIRE(sc.refLines.size() == 1);
    CHECK(sc.refLines[0].first == kChair && sc.refLines[0].second == "Activate 00ABCDEF 1");   // the furniture is the calling reference, the actor the argument
    CHECK(e.StandUp(kId, kChair, 0) == Result::Ok);                  // gentle: the same line again
    CHECK(sc.refLines.size() == 2 && sc.refLines[1].first == kChair && sc.refLines[1].second == "Activate 00ABCDEF 1");
    CHECK(sc.lines.empty());
    CHECK(e.StandUp(kId, kChair, 1) == Result::Ok);                  // escalated: the actor's own AI
    REQUIRE(sc.lines.size() == 2);
    CHECK(sc.lines[0] == "SetRestrained 0" && sc.lines[1] == "EvaluatePackage");
    CHECK(e.StandUp(kId, 0, 0) == Result::Ok && sc.lines.size() == 4);   // no furniture reference: no gentle way either
    CHECK(e.Restrain(kId) == Result::Ok && sc.lines.back() == "SetRestrained 1");
}

TEST(sit, the_script_backend_refuses_what_it_cannot_address) {
    SitScript sc; SitAccess ga;
    ScriptNpcAnimEngine e(sc, ga);
    CHECK(e.SitIn(kId, 0) == Result::Refused);                       // no furniture
    ga.actorRef = 0;
    CHECK(e.SitIn(kId, kChair) == Result::Refused);                  // no live reference for the actor
    ga.actorRef = 0x10;
    sc.ok = false;
    CHECK(e.SitIn(kId, kChair) == Result::Refused);                  // the console did not run the line
    CHECK(e.StandUp(kId, 0, 1) == Result::Refused && sc.lines.size() == 1);   // stops at the first refused line
    CHECK(sc.refLines.size() == 1);
}

// ---- the whole path -----------------------------------------------------------------------------------------------------------------------
// (test_pipeline.cpp: an NPC stands up and walks off, over a lossy link)
