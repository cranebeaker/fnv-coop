// enemy-anim: the whole path on two fake machines. Owner engine -> sampler -> live codec (lossy link) -> interpolation buffer -> driver ->
// remote engine, plus the corpse event over the real ActorSync. Mutation twins: FNV_MUT(n) in the code under test.
#include <deque>
#include <set>

#include "anim_helpers.h"
#include "rig.h"

using namespace anim_test;
using nvcoop::actors::ActorSync;

namespace {

// Slide meter: the T7 sliding detector in miniature. A frame is "sliding" when the body moves but the group cannot explain the speed.
struct SlideMeter {
    uint32_t sinceBad = 0;
    bool bad = false;
    int events = 0;
    uint32_t lastT = 0;
    Vec3 lastPos;
    bool have = false;
    float maxSpeed = 0;
    void frame(uint32_t t, const FakeNpc& n) {
        if (have && t > lastT) {
            const float sec = static_cast<float>(t - lastT) / 1000.0f;
            const float sp = std::sqrt((n.pos.x - lastPos.x) * (n.pos.x - lastPos.x) + (n.pos.y - lastPos.y) * (n.pos.y - lastPos.y)) / sec;
            maxSpeed = std::max(maxSpeed, sp);
            const int g = n.moveGroup;
            bool now = false;
            if (sp > 40.0f) {
                const bool loco = isLocomotionGroup(g) || (g >= kGroupJumpStart && g <= kGroupLast);
                if (!loco) now = true;                                              // moving with no locomotion animation
                else if (g >= kGroupForward && g <= kGroupForward + 3 && sp > 280.0f) now = true;   // walk animation at run speed
                else if (g >= kGroupFastRight - 3 && g <= kGroupFastRight && sp < 120.0f) now = true; // run animation at walk speed
            }
            if (now) { if (!bad) { bad = true; sinceBad = t; } else if (t - sinceBad >= 300 && t - sinceBad < 300 + 40) ++events; }
            else bad = false;
        }
        lastT = t; lastPos = n.pos; have = true;
    }
};

struct AnimPipe {
    FakeNpcEngine owner, remote;
    RecordingNet netO, netR;
    NpcAnimSystem sysO, sysR;
    live::SnapshotSender tx;
    live::SnapshotReceiver rx;
    live::SeqAllocator seqs;
    live::InterpBuffer ib;
    fnvcoop::Rng rng{5};
    struct Pkt { uint32_t at; fnvcoop::Bytes data; bool ack; };
    std::deque<Pkt> link;
    uint32_t now = 1000, latency = 40, jitter = 0;
    double loss = 0;
    float ownerYaw = 0, speed = 0, moveOffset = 0;   // moveOffset: the body moves along yaw + offset (pi/2 = strafing right)
    SlideMeter meter;
    bool holdPoseWhileLocked = false;   // the actor port keeps a copy where it is while the driver says its pose is locked (poseLocked)
    uint32_t t0 = 1000;
    int epoch = 1;

    explicit AnimPipe(const SystemConfig& oc = {}, const SystemConfig& rc = {})
        : sysO(owner, netO, oc), sysR(remote, netR, rc), rx([] { live::ReceiverConfig c; c.checkSenderIsOwner = false; return c; }()) {
        owner.now = now; remote.now = now;
        owner.add(kId, V(1000, 1000, 0));
        remote.add(kId, V(1000, 1000, 0));
        owner.a[kId].moveGroup = kGroupIdle; owner.a[kId].hold = true;
    }
    FakeNpc& O() { return owner.a[kId]; }
    FakeNpc& R() { return remote.a[kId]; }
    void send(const fnvcoop::Bytes& d, bool ack) {
        if (loss > 0 && rng.chance(loss)) return;
        link.push_back({now + latency + (jitter ? rng.range(0, jitter) : 0), d, ack});
    }
    void step() {
        ++now;
        owner.advance(1); remote.advance(1);
        // the owner's body walks
        O().yaw = ownerYaw;
        const float heading = ownerYaw + moveOffset;
        O().pos.x += std::sin(heading) * speed * 0.001f;
        O().pos.y += std::cos(heading) * speed * 0.001f;
        if (now % 50 == 0) {
            live::ActorSnapshot s;
            s.netId = kId; s.ownerEpoch = static_cast<live::OwnerEpoch>(epoch); s.seq = seqs.next(kId, s.ownerEpoch); s.tick = now; s.cell = 1;
            s.pos = O().pos; s.rot = V(0, ownerYaw, 0); s.vel = V(std::sin(heading) * speed, std::cos(heading) * speed, 0);
            sysO.sampleOwned(kId, s, now);
            tx.queue(1, s, now);
            for (auto& p : tx.flush(1, now)) send(p, false);
        }
        while (!link.empty() && static_cast<int32_t>(now - link.front().at) >= 0) {
            Pkt p = link.front(); link.pop_front();
            if (p.ack) { tx.onAckPacket(1, p.data.data(), p.data.size()); continue; }
            for (auto& r : rx.receive(1, p.data.data(), p.data.size()))
                if (r.verdict == live::Verdict::Accepted) ib.push(r.snap, now);
            const fnvcoop::Bytes ack = rx.takeAcks(1);
            if (!ack.empty()) send(ack, true);
        }
        if (now % 33 == 0 && ib.size() > 0) {
            live::ActualPose act; act.cell = R().cell; act.pos = R().pos;
            live::InterpOut out = ib.sample(now, &act);
            const bool held = holdPoseWhileLocked && sysR.driver().poseLocked(kId);
            if (!R().dead && !held) { R().pos = out.pos; R().yaw = out.rot.y; R().cell = out.cell; }
            sysR.drive(kId, out, now);
            meter.frame(now, R());
        }
        // corpse events cross the link too (reliable, same latency)
        for (auto& e : netO.sent) corpseQueue.push_back({now + latency, e.second});
        netO.sent.clear();
        while (!corpseQueue.empty() && static_cast<int32_t>(now - corpseQueue.front().first) >= 0) {
            sysR.onNetEvent(kId, corpseQueue.front().second, now);
            corpseQueue.pop_front();
        }
        sysO.tick(now);
        sysR.tick(now);
    }
    std::deque<std::pair<uint32_t, std::vector<uint8_t>>> corpseQueue;
    void run(uint32_t ms) { for (uint32_t i = 0; i < ms; ++i) step(); }
};

}  // namespace

TEST(pipeline, walking_running_and_stopping_has_no_sliding_feet) {
    AnimPipe p;
    p.loss = 0.05; p.jitter = 15;
    p.run(1000);
    p.speed = 130; p.run(3000);
    CHECK(p.R().moveGroup == kGroupForward);
    p.speed = 300; p.run(3000);
    CHECK(p.R().moveGroup == kGroupFastRight - 3);
    p.speed = 0; p.run(2000);
    CHECK(p.R().moveGroup == kGroupIdle);
    CHECK(p.meter.events == 0);
    CHECK(p.sysR.driver().stats().refused == 0);
}

TEST(pipeline, strafing_while_facing_the_target_uses_the_strafe_groups) {
    AnimPipe p;
    p.run(500);
    p.moveOffset = kPi / 2;   // faces +Y, moves along +X: strafe right
    p.speed = 130; p.run(2500);
    CHECK(p.R().moveGroup == kGroupForward + 3);
    p.moveOffset = -kPi / 2;
    p.run(2500);
    CHECK(p.R().moveGroup == kGroupForward + 2);
    p.moveOffset = kPi;       // backing up
    p.run(2500);
    CHECK(p.R().moveGroup == kGroupForward + 1);
    CHECK(p.meter.events == 0);
}

TEST(pipeline, a_walk_code_at_run_speed_is_corrected_from_the_displayed_speed) {   // twin 30 (sliding feet)
    SystemConfig oc;
    oc.sampler.runEnter = 100000.0f; oc.sampler.runExit = 100000.0f;   // an owner whose sampler cannot tell a run from a walk: it publishes "walk"
    AnimPipe p(oc);
    p.run(800);
    p.speed = 300; p.run(4000);
    CHECK(p.O().moveGroup == kGroupIdle);   // (the owner fake does not play groups: only its velocity matters here)
    CHECK(p.R().moveGroup == kGroupFastRight - 3);
    CHECK(p.meter.events == 0);
    CHECK(p.meter.maxSpeed > 250.0f);
}

TEST(pipeline, an_attack_and_a_stagger_are_seen_once_each_on_the_other_machine) {   // twins 2, 3, 5, 35, 36
    AnimPipe p;
    p.loss = 0.2; p.jitter = 20;
    p.run(1500);
    p.O().actionGroup = 32; p.O().actionStart = p.owner.now; p.O().actionEnd = p.owner.now + 500;   // AttackRight
    const uint32_t tAttack = p.now;
    p.run(1200);
    CHECK(p.remote.plays(kId, 32) == 1);
    p.O().actionGroup = kGroupStagger; p.O().actionStart = p.owner.now; p.O().actionEnd = p.owner.now + 120;   // a very short hit reaction
    p.run(1200);
    CHECK(p.remote.plays(kId, kGroupStagger) == 1);
    // the second swing of the same group
    p.O().actionGroup = 32; p.O().actionStart = p.owner.now; p.O().actionEnd = p.owner.now + 500;
    p.run(1200);
    CHECK(p.remote.plays(kId, 32) == 2);
    // timing: the first attack started on the remote within the interpolation delay + link latency of the owner's start (and not before)
    uint32_t seenAt = 0;
    for (auto& l : p.R().log) {
        if (l.compare(0, 8, "play:32:") == 0) { seenAt = static_cast<uint32_t>(std::stoul(l.substr(l.find('@') + 1))); break; }
    }
    CHECK(seenAt > tAttack && seenAt - tAttack <= 450);
}

TEST(pipeline, the_weapon_state_follows_the_owner_even_when_the_engine_ignores_the_first_call) {   // twins 33, 34
    AnimPipe p;
    p.R().ignoreWeaponCalls = 1;
    p.run(1500);
    p.O().weaponOut = true;
    p.run(3500);
    CHECK(p.R().weaponOut);
    p.O().weaponOut = false;
    p.run(1500);
    CHECK(!p.R().weaponOut);
}

TEST(pipeline, the_corpse_lands_where_it_fell_on_the_owner) {   // twins 20, 21, 22, 42, 43, 60 (missing death pose)
    AnimPipe p;
    p.run(1500);
    p.speed = 200; p.run(800);
    p.speed = 0;
    // the owner's ragdoll tumbles to (1500, 1300); the copy's own ragdoll tumbles somewhere else
    const Vec3 ownerRest = V(1500, 1300, 0), copyRest = V(1420, 1370, 0);
    p.O().dead = true;
    p.owner.startDeath(kId, ownerRest);
    p.sysO.onDied(kId, true, false, p.now);
    p.remote.startDeath(kId, copyRest);
    p.sysR.onDied(kId, false, true, p.now);
    p.run(9000);
    CHECK(p.netR.sent.empty());   // the copy's machine never announces anything
    const Vec3 end = p.R().pos;
    CHECK(std::fabs(end.x - ownerRest.x) < 1.5f && std::fabs(end.y - ownerRest.y) < 1.5f);
    CHECK(!p.R().placedWhileSliding && p.R().placeCalls == 1);
    CHECK(p.sysR.driver().stats().corpseCorrections == 1);
    CHECK(p.R().playsAfterDeath == 0);
}

// ---- over the real ActorSync -------------------------------------------------------------------------------------------------------------
namespace {
struct SyncBridge : IAnimNet {
    nva_test::Machine* m = nullptr;
    bool sendEvent(NetId id, const std::vector<uint8_t>& payload) override { return m->sync->sendAnimEvent(id, payload); }
};
struct Node {
    FakeNpcEngine eng;
    SyncBridge net;
    std::unique_ptr<NpcAnimSystem> sys;
    void attach(nva_test::Machine& mc) {
        net.m = &mc;
        sys.reset(new NpcAnimSystem(eng, net));
        NpcAnimSystem* s = sys.get();
        mc.sync->setAnimEventHandler([this, s](const ActorSync::AnimEventIn& e) { s->onNetEvent(e.id, e.payload, eng.now); });
    }
};
}  // namespace

static std::string corpseOverActorSync(int owner, bool viaHostRelay) {
    (void)viaHostRelay;
    nva_test::Rig r(3);
    r.addActor(101, 3, V(1000), false, ~0u, 30, 0);
    r.setPose(0, 8, V(0)); r.setPose(1, 3, V(1200)); r.setPose(2, 3, V(2500));
    if (owner == 0) { r.setPose(0, 3, V(1200)); r.setPose(1, 8, V(0)); r.setPose(2, 3, V(2500)); }
    r.run(1500);
    if (r.host().sync->ownerOf(101).owner != owner) return "setup: wrong owner";
    std::vector<std::unique_ptr<Node>> nodes;
    for (int i = 0; i < 3; ++i) {
        nodes.emplace_back(new Node());
        nodes.back()->eng.add(101, V(1000, 0, 0));
        nodes.back()->attach(r.at(i));
        nodes.back()->eng.now = static_cast<uint32_t>(r.now);
    }
    // the owner's body tumbles to (1800, 90); everyone else's tumbles somewhere else
    if (!r.at(owner).sync->onLocalDeath(101)) return "owner may not kill";
    const Vec3 ownerRest = V(1800, 90, 0);
    for (int i = 0; i < 3; ++i) {
        Node& n = *nodes[static_cast<size_t>(i)];
        n.eng.startDeath(101, i == owner ? ownerRest : V(1650 + 40.0f * static_cast<float>(i), 0, 0));
        n.sys->onDied(101, i == owner, i != owner, static_cast<uint32_t>(r.now));
    }
    for (int step = 0; step < 900; ++step) {
        r.step(10);
        for (auto& np : nodes) { np->eng.advance(10); np->sys->tick(np->eng.now); }
    }
    for (int i = 0; i < 3; ++i) {
        const FakeNpc& n = nodes[static_cast<size_t>(i)]->eng.a[101];
        if (std::fabs(n.pos.x - ownerRest.x) > 1.5f || std::fabs(n.pos.y - ownerRest.y) > 1.5f)
            return "machine " + std::to_string(i) + " lies at (" + std::to_string(n.pos.x) + "," + std::to_string(n.pos.y) + ") instead of the owner's resting place";
        if (i != owner && n.placeCalls != 1) return "machine " + std::to_string(i) + " moved the body " + std::to_string(n.placeCalls) + " times";
        if (i == owner && n.placeCalls != 0) return "the owner moved its own body";
    }
    if (r.at(owner).sync->stats().animEventsSent != 1) return "the owner sent " + std::to_string(r.at(owner).sync->stats().animEventsSent) + " corpse events";
    return "";
}

TEST(actorsync, a_client_owned_corpse_reaches_the_host_and_the_other_client) { const std::string e = corpseOverActorSync(1, true); CHECK_MSG(e.empty(), e); }
TEST(actorsync, a_host_owned_corpse_reaches_both_clients) { const std::string e = corpseOverActorSync(0, false); CHECK_MSG(e.empty(), e); }

// ---- furniture: an NPC sits, stands up and walks off (2026-10-29: "he floated over sitting down") -----------------------------------------
TEST(pipeline, an_npc_sits_then_stands_up_and_walks_off_and_the_copy_does_the_same_in_that_order) {
    AnimPipe p;
    p.loss = 0.1;
    p.holdPoseWhileLocked = true;
    const Vec3 chair = V(1000, 1000, 0);
    p.O().sitSleep = 4; p.O().furniture = 0x1A2B;           // the owner's NPC starts in its chair
    p.run(3000);
    CHECK(p.R().sitSleep == 4 && p.R().furniture == 0x1A2B);
    CHECK(p.R().sitCalls == 1);
    CHECK(p.R().playsWhileInFurniture == 0);
    // the owner's NPC stands up (400 ms animation) and walks off
    p.O().sitSleep = 5; p.O().standDoneAt = p.now + 400;
    uint32_t displacedWhileInFurniture = 0;
    for (uint32_t t = 0; t < 400; ++t) {
        p.step();
        if (p.R().sitSleep != 0 && std::sqrt((p.R().pos.x - chair.x) * (p.R().pos.x - chair.x) + (p.R().pos.y - chair.y) * (p.R().pos.y - chair.y)) > 25.0f) ++displacedWhileInFurniture;
    }
    p.speed = 130; p.ownerYaw = 0;
    for (uint32_t t = 0; t < 3000; ++t) {
        p.step();
        if (p.R().sitSleep != 0 && std::sqrt((p.R().pos.x - chair.x) * (p.R().pos.x - chair.x) + (p.R().pos.y - chair.y) * (p.R().pos.y - chair.y)) > 25.0f) ++displacedWhileInFurniture;
    }
    CHECK(p.O().sitSleep == 0);
    CHECK(p.R().sitSleep == 0 && p.R().standCalls == 1 && p.R().sitCalls == 1);
    CHECK(p.R().playsWhileInFurniture == 0);                 // no movement group over the pose
    CHECK(displacedWhileInFurniture == 0);                   // it was not moved until it was out of the furniture
    CHECK(p.R().moveGroup == kGroupForward);
    CHECK(p.R().pos.y > chair.y + 100.0f);                   // and it did walk off
    CHECK(p.sysR.driver().stats().standsConfirmed == 1);
}

TEST(pipeline, without_the_pose_lock_the_copy_is_still_stood_up_before_a_movement_group_plays) {
    AnimPipe p;                                              // a port that moves the body regardless
    p.loss = 0.1;
    p.O().sitSleep = 4; p.O().furniture = 0x1A2B;
    p.run(3000);
    p.O().sitSleep = 5; p.O().standDoneAt = p.now + 400;
    p.run(400);
    p.speed = 130;
    p.run(3000);
    CHECK(p.R().sitSleep == 0 && p.R().standCalls == 1);
    CHECK(p.R().playsWhileInFurniture == 0);
    CHECK(p.R().moveGroup == kGroupForward);
}
