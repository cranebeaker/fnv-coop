// The puppet driver on top of the REAL live layer: owner snapshots -> SnapshotSender (delta codec, quantization) -> a lossy,
// jittery link -> SnapshotReceiver -> InterpBuffer (100 ms delay) -> PuppetManager -> fake game. The output is judged
// with the same detectors tools/animcheck applies in the game (slide, T-pose, float/sink, jitter, rubber band), computed
// here on the fake puppet, so a regression in the driver or in the live layer shows up before any game is started.
//
// This is a model of the game, not the game: it proves the logic, not that the engine accepts the calls (docs/tools/avatar.md).
#include <algorithm>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <random>
#include <vector>

#include "avatar_faults.h"
#include "live_bridge.h"
#include "loco.h"
#include "puppet_manager.h"
#include "testing/fake_engine.h"

using namespace nvcoop::avatar;
namespace live = ::fnvcoop::live;

static int g_fail = 0, g_checks = 0;
#define CHECK(c)                                                                         \
    do {                                                                                 \
        ++g_checks;                                                                      \
        if (!(c)) {                                                                      \
            ++g_fail;                                                                    \
            std::printf("  FAIL %s:%d  %s\n", __FILE__, __LINE__, #c);                   \
        }                                                                                \
    } while (0)

namespace {

constexpr CellId kCell = 0x10;
constexpr float kRunSpeed = 300.f, kWalkSpeed = 120.f;

// What the owner's real player does, as a function of time (ms). Closed-form so the truth is known at any instant.
struct OwnerState {
    Vec3 pos, vel;
    float yaw = 0;
    Anim anim;
    bool silent = false;   // the owner's machine sends nothing (alt-tab / lag)
};

OwnerState Owner(uint32_t t) {
    OwnerState s;
    s.pos = Vec3{1000, 1000, 0};
    float ts = t / 1000.0f;
    if (ts < 2) {                                   // stand
    } else if (ts < 6) {                            // run forward (+Y)
        s.vel = Vec3{0, kRunSpeed, 0};
        s.pos.y += kRunSpeed * (ts - 2);
        s.anim.loco = kLocoRunFwd;
    } else if (ts < 8) {                            // walk forward, sneaking
        s.pos.y += kRunSpeed * 4 + kWalkSpeed * (ts - 6);
        s.vel = Vec3{0, kWalkSpeed, 0};
        s.anim.loco = kLocoWalkFwd;
        s.anim.sneak = true;
    } else if (ts < 10) {                           // turn east and strafe-walk right
        s.pos.y += kRunSpeed * 4 + kWalkSpeed * 2;
        s.pos.x += kWalkSpeed * (ts - 8);
        s.vel = Vec3{kWalkSpeed, 0, 0};
        s.anim.loco = kLocoWalkRight;
    } else if (ts < 12) {                           // stop, draw, aim
        s.pos.y += kRunSpeed * 4 + kWalkSpeed * 2;
        s.pos.x += kWalkSpeed * 2;
        s.anim.weaponDrawn = true;
        s.anim.aim = ts > 11;
    } else {                                        // holster, wait
        s.pos.y += kRunSpeed * 4 + kWalkSpeed * 2;
        s.pos.x += kWalkSpeed * 2;
    }
    if (ts >= 14 && ts < 14.6f) {                   // jump on the spot
        s.anim.jump = true;
        s.pos.z = 80.f * std::sin((ts - 14) / 0.6f * 3.14159f);
    }
    if (ts >= 16) s.pos.x += 2000;                  // teleport 28 m east at 16 s
    if (ts >= 16.6f) {                              // run forward again, stop dead at 18 s ...
        s.pos.y += kRunSpeed * (std::min(ts, 18.0f) - 16.6f);
        if (ts < 18.0f) {
            s.vel = Vec3{0, kRunSpeed, 0};
            s.anim.loco = kLocoRunFwd;
        }
    }
    if (ts >= 18 && ts < 20) s.silent = true;       // ... and the link dies exactly then: the last thing received says "running"
    s.yaw = ts < 8 ? 0.f : 1.5707963f;
    return s;
}

struct Metrics {
    unsigned slideFrames = 0, slideEpisodes = 0, tposeFrames = 0, floatFrames = 0, jitterFrames = 0, rubberJumps = 0, bigJumps = 0;
    unsigned skateFrames = 0;
    unsigned shoveFrames = 0;    // frames within 700 ms of an engine push where the puppet is still far from the owner
    float maxStep = 0, resumeStep = 0;
    uint32_t maxStepAt = 0;
    unsigned idleAfterGapMs = 0;
    bool idledDuringGap = false;
};

}  // namespace

struct Result {
    Metrics m;
    unsigned drivingFrames = 0, spawns = 0;
    bool ok = true;
    int finalGroup = -1;
    unsigned violations = 0;
};

// `shoves`: a body in the way pushes the puppet 3 to 5 m away four times (twice while it walks, twice while it stands).
static Result RunScenario(unsigned lossPercent, uint32_t seed, bool shoves = false, bool lowLatency = false) {
    Result res;
    FakePuppetEngine eng;
    eng.loadedCells = {kCell};
    Config cfg;
    cfg.localSlot = 0;
    cfg.templates = {{1, false, 0x1000}};
    PuppetManager pm(eng, cfg);
    Appearance app;
    app.race = 1;
    pm.AddPlayer(1, app);

    live::SnapshotSender sender;
    live::ReceiverConfig rcfg;
    rcfg.checkSenderIsOwner = false;
    live::SnapshotReceiver receiver(rcfg);
    live::InterpConfig icfg;
    icfg.lowLatency = lowLatency;   // the shipped playback (avatar_tuning.h DefaultLagTuning().interp) next to the old 100 ms buffer
    live::InterpBuffer buf(icfg);
    live::SeqAllocator seqs;
    const ::fnvcoop::PlayerId kOwner = 1, kMe = 2;
    const live::NetId kNet = 77;

    std::mt19937 rng(seed);
    struct InFlight { uint32_t arrive; live::Bytes data; bool ack; };
    std::vector<InFlight> wire;       // owner -> me (snapshots)
    std::vector<InFlight> backWire;   // me -> owner (acks)

    bool havePrev = false;
    Vec3 prev;
    float prevDy = 0;
    uint32_t lowSince = 0;
    uint32_t slideSince = 0;
    uint32_t gapStart = 0;
    bool inGap = false;

    const uint32_t kEnd = 24000;
    uint32_t nextSend = 0;
    struct Shove { uint32_t at; Vec3 d; };
    const Shove kShoves[] = {{7000, {300, 0, 0}}, {9000, {-100, -450, 0}}, {12500, {0, 500, 0}}, {13500, {-250, 0, 0}}};
    size_t nextShove = 0;
    uint32_t lastShoveAt = 0;
    bool haveShove = false;
    for (uint32_t now = 0; now <= kEnd; now += 16) {
        // owner: 20 Hz snapshots
        OwnerState o = Owner(now);
        if (now >= nextSend) {
            nextSend += 50;
            if (!o.silent) {
                Anim a = o.anim;
                live::ActorSnapshot s = MakeLocalSnapshot(kNet, 1, seqs.next(kNet, 1), now, kCell, o.pos, Vec3{0, o.yaw, 0}, o.vel, a);
                if (sender.queue(kMe, s, now))
                    for (auto& pkt : sender.flush(kMe, now)) {
                        if (rng() % 100 < lossPercent) continue;   // unreliable link
                        uint32_t lat = 40 + rng() % 25;
                        wire.push_back({now + lat, pkt, false});
                    }
            }
        }
        // deliver
        for (size_t i = 0; i < wire.size();) {
            if (wire[i].arrive <= now) {
                for (auto& rs : receiver.receive(kOwner, wire[i].data.data(), wire[i].data.size()))
                    if (rs.verdict == live::Verdict::Accepted) buf.push(rs.snap, now);
                live::Bytes ack = receiver.takeAcks(kOwner);
                if (!ack.empty()) backWire.push_back({now + 40, ack, true});
                wire.erase(wire.begin() + i);
            } else {
                ++i;
            }
        }
        for (size_t i = 0; i < backWire.size();) {
            if (backWire[i].arrive <= now) {
                sender.onAckPacket(kMe, backWire[i].data.data(), backWire[i].data.size());
                backWire.erase(backWire.begin() + i);
            } else {
                ++i;
            }
        }

        // receiver: one frame
        if (shoves && nextShove < sizeof kShoves / sizeof kShoves[0] && now >= kShoves[nextShove].at) {
            if (auto* pf = eng.Get(pm.RefOf(1))) {
                pf->pos.x += kShoves[nextShove].d.x;
                pf->pos.y += kShoves[nextShove].d.y;
                lastShoveAt = now;
                haveShove = true;
            }
            ++nextShove;
        }
        live::ActualPose ap;
        bool haveActual = false;
        CellId ac = 0;
        Vec3 apos;
        if (pm.ActualPose(1, ac, apos)) {
            ap.cell = ac;
            ap.pos = ToLive(apos);
            haveActual = true;
        }
        live::InterpOut io = buf.sample(now, haveActual ? &ap : nullptr);
        PuppetPose pose = FromInterp(io);
        eng.Frame();
        if (io.state != live::InterpState::Empty) pm.SetPose(1, pose);
        pm.Tick(now + 1000);   // manager clock = sim clock + 1 s (never 0)

        // ---- detectors on the puppet, the way tools/animcheck does it ----
        auto* f = eng.Get(pm.RefOf(1));
        if (!f || pm.PhaseOf(1) != Phase::Driving || now < 1000) {
            havePrev = false;
            continue;
        }
        ++res.drivingFrames;
        if (haveShove && now - lastShoveAt < 700) {
            const Vec3 truth = Owner(now).pos;
            if (std::hypot(f->pos.x - truth.x, f->pos.y - truth.y) > 100.f) ++res.m.shoveFrames;
        }
        if (!havePrev) {
            prev = f->pos;
            havePrev = true;
            continue;
        }
        float dx = f->pos.x - prev.x, dy = f->pos.y - prev.y, dz = f->pos.z - prev.z;
        float step = std::sqrt(dx * dx + dy * dy);
        float speed = step / 0.016f;
        prev = f->pos;
        if (std::getenv("AVATAR_DEBUG") && now > 2000 && now < 2500)
            std::printf("    t=%u step=%.2f dz=%.2f io.state=%d io.y=%.1f group=%d\n", now, step, dz, (int)io.state, io.pos.y, f->group);
        if (std::fabs(dz) > 25.f) res.m.floatFrames++;                 // vertical pops (jump arc is smooth, 80 units over 0.6 s)
        const float tsn = now / 1000.0f;
        if (step > 150.f) {
            ++res.m.bigJumps;                                          // planned teleport only
        } else if (tsn >= 20.0f && tsn < 21.0f) {
            res.m.resumeStep = std::max(res.m.resumeStep, step);       // the link comes back: the extrapolation overshoot is undone
        } else if (step > res.m.maxStep) {
            res.m.maxStep = step;
            res.m.maxStepAt = now;
        }
        // jitter: during the steady run the puppet must move monotonically along +Y
        float ts = now / 1000.0f;
        if (ts > 3.5f && ts < 5.8f && dy < -1.0f) res.m.jitterFrames++;
        if (ts > 3.5f && ts < 5.8f && std::fabs(dy - prevDy) > 40.f) res.m.jitterFrames++;
        prevDy = dy;
        // T-pose proxy: driving but no group was ever issued
        if (pm.LastGroup(1) < 0) res.m.tposeFrames++;
        // slide: moving fast with a non-locomotion group for longer than 300 ms
        int g = f->group;
        bool locoGroup = IsLocomotionGroup(g) || IsJumpGroup(g);
        if (speed > 60.f && !locoGroup && step <= 150.f) {
            if (!slideSince) slideSince = now;
            if (now - slideSince > 300) res.m.slideFrames++;
        } else {
            slideSince = 0;
        }
        // skate: locomotion group while standing still for longer than 600 ms
        if (speed < 5.f && IsLocomotionGroup(g)) {
            if (!lowSince) lowSince = now;
            if (now - lowSince > 600) res.m.skateFrames++;
        } else {
            lowSince = 0;
        }
        // the silent window: legs must stop
        if (o.silent) {
            if (!inGap) {
                inGap = true;
                gapStart = now;
            }
            if (now - gapStart >= 800 && g == kGrpIdle) res.m.idledDuringGap = true;
        }
    }
    res.spawns = static_cast<unsigned>(eng.spawnCalls);
    res.finalGroup = eng.Get(pm.RefOf(1)) ? eng.Get(pm.RefOf(1))->group : -1;
    res.violations = static_cast<unsigned>(eng.violations.size()) + eng.refusedNo3D;
    return res;
}

// Number of failed judgements for one scenario result (0 = the puppet looked right).
static int Judge(const Result& r) {
    int bad = 0;
    auto chk = [&](bool c) {
        ++g_checks;
        if (!c) ++bad;
    };
    chk(r.drivingFrames > 1000);
    chk(r.m.slideFrames == 0);
    chk(r.m.skateFrames == 0);
    chk(r.m.tposeFrames == 0);
    chk(r.m.floatFrames == 0);
    chk(r.m.shoveFrames == 0);   // an engine push is undone in the frame it is seen, never eased back over a few hundred ms
    chk(r.m.jitterFrames == 0);
    chk(r.m.bigJumps == 1);     // exactly the planned teleport, no rubber banding
    // 300 u/s at 62 fps is 4.8 u/frame. With 25 ms of latency jitter the live InterpBuffer's clock-offset filter makes the shown
    // time jump forward now and then, so single frames reach about 3x that (14 to 16 units seen, reported to live-core). The bound
    // therefore catches pops (a 40+ unit step, e.g. a smoothed-out teleport) but not that lumpiness.
    chk(r.m.maxStep < 20.f);
    chk(r.m.resumeStep < 100.f);   // at most the 250 ms extrapolation cap at 300 u/s (75 units), never a 150+ rubber band
    chk(r.m.idledDuringGap);
    chk(r.spawns == 1);
    chk(r.violations == 0);
    return bad;
}

static bool Twin(const char* name, int bad) {
    std::printf("  twin %-26s %s\n", name, bad ? "killed" : "SURVIVED");
    return bad != 0;
}

int main() {
    for (unsigned loss : {0u, 10u}) {
        for (uint32_t seed : {1u, 2u, 3u}) {
            Result r = RunScenario(loss, seed);
            std::printf("  loss=%u%% seed=%u: frames=%u slide=%u skate=%u tpose=%u float=%u jitter=%u bigJumps=%u maxStep=%.1f@%ums idledInGap=%d spawns=%u\n",
                        loss, seed, r.drivingFrames, r.m.slideFrames, r.m.skateFrames, r.m.tposeFrames, r.m.floatFrames, r.m.jitterFrames,
                        r.m.bigJumps, r.m.maxStep, (unsigned)r.m.maxStepAt, (int)r.m.idledDuringGap, r.spawns);
            int bad = Judge(r);
            if (bad) std::printf("  FAIL: %d judgements failed\n", bad);
            g_fail += bad;
        }
    }
    for (unsigned loss : {0u, 10u}) {
        Result r = RunScenario(loss, 4, true);
        std::printf("  pushes loss=%u%%: shoveFrames=%u bigJumps=%u maxStep=%.1f@%ums\n", loss, r.m.shoveFrames, r.m.bigJumps, r.m.maxStep, (unsigned)r.m.maxStepAt);
        int bad = Judge(r);
        if (bad) std::printf("  FAIL: %d judgements failed\n", bad);
        g_fail += bad;
    }
    // the same walk, outage, teleport and engine pushes through the low-latency playback that ships
    for (unsigned loss : {0u, 10u}) {
        for (uint32_t seed : {1u, 2u, 3u}) {
            Result r = RunScenario(loss, seed, false, true);
            std::printf("  low-latency loss=%u%% seed=%u: frames=%u slide=%u skate=%u tpose=%u float=%u jitter=%u bigJumps=%u maxStep=%.1f@%ums resumeStep=%.1f idledInGap=%d spawns=%u\n",
                        loss, seed, r.drivingFrames, r.m.slideFrames, r.m.skateFrames, r.m.tposeFrames, r.m.floatFrames, r.m.jitterFrames,
                        r.m.bigJumps, r.m.maxStep, (unsigned)r.m.maxStepAt, r.m.resumeStep, (int)r.m.idledDuringGap, r.spawns);
            int bad = Judge(r);
            if (bad) std::printf("  FAIL: %d judgements failed\n", bad);
            g_fail += bad;
        }
    }
    for (unsigned loss : {0u, 10u}) {
        Result r = RunScenario(loss, 4, true, true);
        std::printf("  low-latency pushes loss=%u%%: shoveFrames=%u bigJumps=%u maxStep=%.1f@%ums\n", loss, r.m.shoveFrames, r.m.bigJumps, r.m.maxStep, (unsigned)r.m.maxStepAt);
        int bad = Judge(r);
        if (bad) std::printf("  FAIL: %d judgements failed\n", bad);
        g_fail += bad;
    }
    std::printf("avatar live: %d checks, %d failed\n", g_checks, g_fail);
    if (g_fail) return 1;

    // mutation twins: the same scenario with one thing broken must be judged bad
    int survivors = 0;
    {
        AvatarFaults f;
        f.frozenKeepsRunning = true;   // the legs keep going when the owner goes silent
        AvatarFaultScope sc(f);
        survivors += Twin("frozenKeepsRunning", Judge(RunScenario(0, 1))) ? 0 : 1;
    }
    {
        AvatarFaults f;
        f.noWait3D = true;             // the driver commands a ref before its 3D exists
        AvatarFaultScope sc(f);
        survivors += Twin("noWait3D", Judge(RunScenario(0, 1))) ? 0 : 1;
    }
    {
        live::LiveFaults lf;
        lf.noSnap = true;              // the live layer smooths a 28 m teleport across the map
        live::LiveFaultScope sc(lf);
        survivors += Twin("live.noSnap", Judge(RunScenario(0, 1))) ? 0 : 1;
    }
    {
        live::LiveFaults lf;
        lf.extrapolateOnly = true;     // the live layer never interpolates
        live::LiveFaultScope sc(lf);
        survivors += Twin("live.extrapolateOnly", Judge(RunScenario(10, 2))) ? 0 : 1;
    }
    {
        AvatarFaults f;
        f.adoptPhysicsPush = true;     // the buffer is told the pushed place: it eases the puppet back from it
        AvatarFaultScope sc(f);
        survivors += Twin("adoptPhysicsPush", Judge(RunScenario(0, 4, true))) ? 0 : 1;
    }
    if (survivors) {
        std::printf("avatar live: %d twins survived\n", survivors);
        return 1;
    }
    std::printf("avatar live: %d checks, 0 failed, all twins killed - ALL OK\n", g_checks);
    return 0;
}
