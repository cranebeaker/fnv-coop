// How far behind does a remote player's puppet appear? A game-free replay of walking tracks (start, run, turn, stop, strafe, walk, reverse)
// through the REAL sender chain (LocalSampler at frame rate -> StreamRatePlanner near tier -> SnapshotSender delta codec) over the chaos profiles of
// tools/chaos/profiles.json (latency, jitter, loss, reorder, duplicates, bandwidth: synccore/chaos/impair.h, the same model as the relay) into the REAL
// receiver (SnapshotReceiver -> InterpBuffer -> PuppetManager -> fake game), once with the behaviour before the avatar-lag round (LegacyLagTuning) and once
// with what ships (DefaultLagTuning). The puppet's position is compared with the walker's true position at the same instant.
//
//   avatar_lag_tests               regression run: the shipped tuning must stay inside the limits below on every profile (exit 1 on a failed check)
//   avatar_lag_tests --report      print the before / after table (error p95 and max, overshoot on a stop, teleports, jitter, bytes per second)
//   avatar_lag_tests --report --fps 30 --speed 250
//   FNVCOOP_MUT=201..208           mutation twins (tools/mut/run_twins.py): each breaks one part of the round and MUST fail a check
//   LAG_TUNE="minDelayMs=10,nearHz=60" ... --report   sweep the shipped tuning with nvcoop.ini [avatar] keys (avatar_tuning.h) without rebuilding
//   LAG_TRACE=<ms> / LAG_STEPS=<ms>   (with --report) print the walker and puppet positions / per-frame puppet steps from that time on
//
// A model of the game and the network, not the game: it proves the logic of the pipeline. What the engine adds on top (frame pacing of two copies on
// one PC, the bridge read) is not in it; the in-game walk room of the lab (tools/lab, "walk") is the check against the real thing.
#include <algorithm>
#include <cmath>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <map>
#include <string>
#include <vector>

#include "avatar_tuning.h"
#include "fnvcoop_mut.h"
#include "live_bridge.h"
#include "impair.h"
#include "puppet_manager.h"
#include "testing/fake_engine.h"

using namespace nvcoop::avatar;
using namespace nvcoop::avatar::engine;
namespace live = ::fnvcoop::live;
namespace chaos = ::fnvcoop::chaos;

// Mutation twins of this suite (FNVCOOP_MUT=n, the numbers sit above the 1..86 of the other avatar suites):
// 201 render delay ignores the jitter (always the ceiling)   202 dead reckoning undamped (a stop overshoots)   203 no fade of prediction misses (pops)
// 204 render clock follows the offset filter at once (lumpy)  205 the old fixed 100 ms buffer                   206 sender velocity filtered to death
// 207 near rate 10 Hz                                          208 no extrapolation at all

static int g_fail = 0, g_checks = 0;
#define CHECK(c)                                                                  \
    do {                                                                          \
        ++g_checks;                                                               \
        if (!(c)) {                                                               \
            ++g_fail;                                                             \
            std::printf("  FAIL %s:%d  %s\n", __FILE__, __LINE__, #c);            \
        }                                                                         \
    } while (0)

namespace {

constexpr CellId kCell = 0x10;
constexpr uint32_t kSenderClockOffset = 123456;   // the two machines do not share a clock

// ---- the walker --------------------------------------------------------------------------------------------------------------------
struct Knot { float t, speed, heading, facing; };   // seconds, units/s, degrees (heading = direction of travel, facing = where the body looks)

struct Track {
    std::vector<Vec3> pos;                      // per ms
    std::vector<float> speed, heading, facing;  // per ms (heading / facing in degrees)
    uint32_t durMs = 0;
    struct Stop { uint32_t atMs; float headingDeg; };
    std::vector<Stop> stops;
    struct Steady { uint32_t a, b; };
    std::vector<Steady> steady;
};

constexpr uint32_t kJumpAtMs = 3000, kStartAtMs = 6000;

// kind 0: the walking track below. kind 1: stand 3 s, jump 450 units (the lab's spawn room teleport was 773; 450 is below the old 10 m snap and above the 2 m smoothing), stand. kind 2: stand 6 s (a stand long enough for the
// stream to fall to its once-a-second heartbeat), then run.
Track MakeTrack(float speedScale, int kind = 0) {
    if (kind == 1) {
        Track tr;
        tr.durMs = 6000;
        tr.pos.assign(tr.durMs + 1, Vec3{1000.0f, 1000.0f, 0.0f});
        tr.speed.assign(tr.durMs + 1, 0.0f);
        tr.heading.assign(tr.durMs + 1, 0.0f);
        tr.facing.assign(tr.durMs + 1, 0.0f);
        for (uint32_t ms = kJumpAtMs; ms <= tr.durMs; ++ms) tr.pos[ms] = Vec3{1450.0f, 1000.0f, 0.0f};
        tr.steady.push_back({2000, 2500});
        tr.steady.push_back({5200, 5900});
        return tr;
    }
    // kind 3: the lab's walk room: four legs of 300 units on a square, run at 375 u/s (bot.move with run), each ended by a stop of 0.4 s (the bridge notices the arrival and
    // starts the next goto), 90 degree turns at the corners, twice round
    std::vector<Knot> k3;
    if (kind == 3) {
        float t = 0.0f;
        float hd = 0.0f;
        k3.push_back({0.0f, 0, 0, 0});
        t = 1.0f;
        k3.push_back({t, 0, 0, 0});
        for (int leg = 0; leg < 8; ++leg) {
            k3.push_back({t + 0.15f, 375, hd, hd});            // away in 0.15 s
            t += 0.15f + 0.65f;
            k3.push_back({t, 375, hd, hd});                      // 300 units covered
            k3.push_back({t + 0.1f, 0, hd, hd});               // stop in 0.1 s
            t += 0.1f + 0.4f;
            k3.push_back({t, 0, hd, hd});
            hd = std::fmod(hd + 90.0f, 360.0f);                // turn at the corner
            k3.push_back({t + 0.02f, 0, hd, hd});
            t += 0.02f;
        }
        k3.push_back({t + 1.0f, 0, hd, hd});
    }
    const std::vector<Knot> k2 = {{0.0f, 0, 0, 0}, {6.0f, 0, 0, 0}, {6.25f, 400, 0, 0}, {10.0f, 400, 0, 0}, {10.15f, 0, 0, 0}, {12.0f, 0, 0, 0}};
    const std::vector<Knot> k = kind == 3 ? k3 : (kind == 2 ? k2 : std::vector<Knot>{
        {0.0f, 0, 0, 0},       {1.0f, 0, 0, 0},       {1.25f, 400, 0, 0},    // stand, then start running north (0.25 s to full speed)
        {4.0f, 400, 0, 0},     {4.5f, 400, 90, 90},                           // run, turn right by 90 degrees while running
        {6.0f, 400, 90, 90},   {6.15f, 0, 90, 90},                            // run east, stop dead in 150 ms
        {7.5f, 0, 90, 90},     {7.7f, 250, 180, 90},                          // stand, then strafe right (travel south, still facing east)
        {9.7f, 250, 180, 90},  {9.85f, 0, 180, 90},                           // strafe, stop
        {11.0f, 0, 180, 90},   {11.15f, 150, 180, 180},                       // stand, turn and walk south
        {12.5f, 150, 180, 180},{13.0f, 150, 270, 270},                        // walk, turn right while walking
        {14.0f, 150, 270, 270},{14.15f, 0, 270, 270},                         // walk west, stop
        {15.5f, 0, 270, 270},  {15.6f, 0, 90, 90},    {15.85f, 400, 90, 90},  // stand, turn round, run east
        {17.0f, 400, 90, 90},  {17.2f, 400, 270, 90},                         // run, reverse the direction of travel in 0.2 s (zig zag)
        {18.0f, 400, 270, 90}, {18.15f, 0, 270, 90},                          // run back, stop
        {20.0f, 0, 270, 90}});
    Track tr;
    tr.durMs = static_cast<uint32_t>(k.back().t * 1000.0f);
    tr.pos.resize(tr.durMs + 1);
    tr.speed.resize(tr.durMs + 1);
    tr.heading.resize(tr.durMs + 1);
    tr.facing.resize(tr.durMs + 1);
    Vec3 p{1000.0f, 1000.0f, 0.0f};
    size_t seg = 0;
    for (uint32_t ms = 0; ms <= tr.durMs; ++ms) {
        const float t = ms / 1000.0f;
        while (seg + 2 < k.size() && t > k[seg + 1].t) ++seg;
        const Knot& a = k[seg];
        const Knot& b = k[seg + 1];
        const float u = b.t > a.t ? std::min(1.0f, std::max(0.0f, (t - a.t) / (b.t - a.t))) : 1.0f;
        const float sp = (a.speed + (b.speed - a.speed) * u) * speedScale;
        const float hd = a.heading + (b.heading - a.heading) * u;
        tr.speed[ms] = sp;
        tr.heading[ms] = hd;
        tr.facing[ms] = a.facing + (b.facing - a.facing) * u;
        tr.pos[ms] = p;
        const float h = hd * 3.14159265f / 180.0f;
        p.x += sp * std::sin(h) * 0.001f;
        p.y += sp * std::cos(h) * 0.001f;
    }
    for (size_t i = 1; i < k.size(); ++i)
        if (k[i].speed == 0 && k[i - 1].speed > 0) tr.stops.push_back({static_cast<uint32_t>(k[i].t * 1000.0f), k[i].heading});
    tr.steady.push_back({2000, 3900});
    tr.steady.push_back({5200, 5900});
    return tr;
}

// ---- one run -----------------------------------------------------------------------------------------------------------------------
struct SimOpts {
    LagTuning tune;
    std::string profile = "lan";
    uint64_t seed = 1;
    uint32_t frameMs = 16;
    float frameJitter = 0.15f;   // frame lengths vary by this fraction
    float speedScale = 1.0f;
    bool mutate = false;   // apply the mutation twins (only to the shipped tuning)
    // The order of one frame in the game (module order, plugin/common/nvcoop_api.h): the avatar module drives the puppets and samples the local player, THEN the co-op
    // module (a late module) polls the network (arrival stamps = this frame, the data can only be drawn by the NEXT frame) and sends what was sampled. The first
    // version of this model polled before it drove, which gave every packet a free frame.
    bool netAfterDrive = true;
    // The rate planner runs on the co-op module's clock, GetTickCount64, which moves in steps of 15.625 ms (0 = exact ms, the first version of the model).
    double plannerClockStepMs = 15.625;
    int track = 0;   // 0 walking, 1 teleport (arriveMs), 2 start from a long stand (startMax), 3 the lab's walk room (stop and go legs of 300 units)
};

struct SimResult {
    double errP95 = 0, errMax = 0;     // units, while the walker moves / over the whole track
    double overshoot = 0;              // units beyond the final stop position, worst stop
    double settle80Ms = 0, settle10Ms = 0;
    unsigned teleports = 0, snapped = 0;
    double jitterPct = 0;              // worst per-frame step deviation in the steady runs, percent of the nominal step
    unsigned reversals = 0;            // frames in a steady run where the puppet moved backwards
    double steadyLagMs = 0;            // mean distance behind in the steady run, as time
    double arriveMs = 0;               // teleport track: from the jump until the puppet is within 40 units for good
    double startMax = 0;               // start track: the largest error in the second after the first step
    double bytesPerSec = 0, packetsPerSec = 0, snapshotsPerSec = 0;
    double runBytesPerSec = 0;         // payload while running steadily (the stream when it matters), not averaged with standing still
    unsigned frames = 0;
};

double Dist2(const Vec3& a, const Vec3& b) { return std::hypot(a.x - b.x, a.y - b.y); }

SimResult RunSim(const SimOpts& o) {
    SimResult R;
    const Track tr = MakeTrack(o.speedScale, o.track);
    LagTuning tune = o.tune;
    live::LiveFaults lf;
    if (o.mutate) {
        lf.llFixedDelay = FNV_MUT(201);
        lf.llNoDamping = FNV_MUT(202);
        lf.llNoBlend = FNV_MUT(203);
        lf.llNoSlew = FNV_MUT(204);
        lf.llOldInterval = FNV_MUT(210);
        lf.noTeleportSnap = FNV_MUT(211);
        if (FNV_MUT(205)) tune.interp.lowLatency = false;
        if (FNV_MUT(206)) tune.sampler.velAlpha = 0.15f;
        if (FNV_MUT(207)) tune.sampler.periodMs = tune.rate.nearPeriodMs = 100;
        if (FNV_MUT(208)) tune.interp.maxExtrapolateMs = 0;
    }
    live::LiveFaultScope faultScope(lf);

    // receiver side: the puppet
    FakePuppetEngine eng;
    eng.loadedCells = {kCell};
    Config cfg;
    cfg.localSlot = 0;
    cfg.templates = {{1, false, 0x1000}};
    PuppetManager pm(eng, cfg);
    Appearance app;
    app.race = 1;
    pm.AddPlayer(1, app);
    live::InterpBuffer buf(tune.interp);
    live::ReceiverConfig rcfg;
    rcfg.checkSenderIsOwner = false;
    live::SnapshotReceiver receiver(rcfg);

    // sender side
    LocalSampler sampler(tune.sampler);
    const live::NetId kNet = 77;
    sampler.SetIdentity(kNet, 1);
    StreamRatePlanner planner(tune.rate);
    live::SnapshotSender sender;
    const ::fnvcoop::PlayerId kOwner = 1, kMe = 2;

    // the link, one impairment per direction like the relay
    chaos::Params prm = chaos::Params::make(1, 0, 0, 0, 0, 0.5, 1, 0, 0, 0, 0, 200);
    for (const auto& kv : chaos::allProfiles())
        if (kv.first == o.profile) prm = kv.second;
    prm.seed = prm.seed + o.seed * 7919;
    chaos::Impairment c2s(prm, "c2s"), s2c(prm, "s2c");
    std::multimap<double, live::Bytes> toMe, toOwner;

    std::vector<double> movingErr;
    std::vector<double> errAt(tr.durMs + 1, -1.0);
    std::vector<Vec3> puppetAt(tr.durMs + 1);
    std::vector<char> have(tr.durMs + 1, 0);
    Vec3 prev{};
    bool havePrev = false;
    uint32_t prevT = 0;
    double lastAck = 0;
    double steadyLagSum = 0;
    unsigned steadyLagN = 0;
    double snapshotsSent = 0;
    uint64_t bytesAt2s = 0, bytesAt39s = 0;

    // The two game copies run their own frames (fixed length with some jitter, the receiver's a third of a frame later). The walker's position is
    // what his copy last set (the lab reads the ref, so it is stale by the frame phase, exactly as the puppet is); both are compared at every ms.
    chaos::Rng frameRng(o.seed, "frames");
    auto gap = [&]() { return o.frameMs * (1.0 + o.frameJitter * (frameRng.random() * 2.0 - 1.0)); };
    double nextS = 0, nextR = o.frameMs / 3.0;
    uint32_t lastSFrame = 0;

    for (uint32_t now = 0; now <= tr.durMs; ++now) {
        if (now >= nextS) {
            nextS += gap();
            lastSFrame = now;
            const uint32_t sNow = now + kSenderClockOffset;
            // ---- owner machine: one frame (acks arrive here, then the local player is sampled and the stream goes out)
            while (!toOwner.empty() && toOwner.begin()->first <= now) {
                live::Bytes data = toOwner.begin()->second;
                toOwner.erase(toOwner.begin());
                sender.onAckPacket(kMe, data.data(), data.size());
            }
            LocalFrame f;
            f.valid = true;
            f.cell = kCell;
            f.pos = tr.pos[now];
            f.rot = Vec3{0.0f, tr.facing[now] * 3.14159265f / 180.0f, 0.0f};
            live::ActorSnapshot snap;
            const uint32_t pNow = o.plannerClockStepMs > 0 ? static_cast<uint32_t>(std::floor(sNow / o.plannerClockStepMs) * o.plannerClockStepMs) : sNow;
            if (sampler.Update(sNow, f, snap)) {
                live::ActorSnapshot out;
                if (planner.offerAtTier(StreamRatePlanner::kUplink, 1, StreamTier::Near, snap, pNow, out))
                    if (sender.queue(kMe, out, sNow)) snapshotsSent += 1;
            }
            std::vector<StreamRatePlanner::Release> rel;
            planner.poll(pNow, rel);
            for (auto& r : rel)
                if (sender.queue(kMe, r.snap, sNow)) snapshotsSent += 1;
            for (auto& pkt : sender.flush(kMe, sNow)) {
                chaos::Verdict v = c2s.decide(now, static_cast<uint32_t>(pkt.size()));
                for (double d : v.delaysMs) toMe.emplace(now + d, pkt);
            }
        }
        if (now == 2000) bytesAt2s = sender.stats().bytes;
        if (now == 3900) bytesAt39s = sender.stats().bytes;
        if (now >= nextR) {
            nextR += gap();
            // ---- receiving machine: one frame. The network pump (packets that arrived since the last frame are stamped with this frame's time) runs either
            // before the drive (the first version of the model) or after it (the game: the co-op module is a late one).
            auto pump = [&]() {
                while (!toMe.empty() && toMe.begin()->first <= now) {
                    live::Bytes data = toMe.begin()->second;
                    toMe.erase(toMe.begin());
                    for (auto& rs : receiver.receive(kOwner, data.data(), data.size()))
                        if (rs.verdict == live::Verdict::Accepted) buf.push(rs.snap, now);
                }
                if (now - lastAck >= 50) {
                    lastAck = now;
                    live::Bytes ack = receiver.takeAcks(kOwner);
                    if (!ack.empty()) {
                        chaos::Verdict v = s2c.decide(now, static_cast<uint32_t>(ack.size()));
                        for (double d : v.delaysMs) toOwner.emplace(now + d, ack);
                    }
                }
            };
            if (!o.netAfterDrive) pump();
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
            eng.Frame();
            if (io.state != live::InterpState::Empty) pm.SetPose(1, FromInterp(io));
            pm.Tick(now + 1000);
            if (o.netAfterDrive) pump();
            if (io.snapped && now >= 1000) ++R.snapped;

            // ---- frame-to-frame behaviour of the puppet
            auto* pf = eng.Get(pm.RefOf(1));
            if (!pf || pm.PhaseOf(1) != Phase::Driving || now < 1000) {
                havePrev = false;
            } else {
                if (std::getenv("LAG_TRACE") && now >= std::strtoul(std::getenv("LAG_TRACE"), nullptr, 10) && now < std::strtoul(std::getenv("LAG_TRACE"), nullptr, 10) + 1500)
                    std::printf("    t=%u truth=(%.1f,%.1f) puppet=(%.1f,%.1f) state=%d\n", now, tr.pos[lastSFrame].x, tr.pos[lastSFrame].y, pf->pos.x, pf->pos.y, (int)io.state);
                if (havePrev) {
                    const double step = Dist2(pf->pos, prev);
                    if (std::getenv("LAG_STEPS") && now >= std::strtoul(std::getenv("LAG_STEPS"), nullptr, 10) && now < std::strtoul(std::getenv("LAG_STEPS"), nullptr, 10) + 4000)
                        std::printf("    t=%u dt=%u step=%.1f nominal=%.1f state=%d\n", now, now - prevT, step, tr.speed[now] * (now - prevT) / 1000.0, (int)io.state);
                    if (step > 150.0) ++R.teleports;
                    for (const auto& sg : tr.steady) {
                        if (prevT >= sg.a && now <= sg.b) {
                            const float h = tr.heading[now] * 3.14159265f / 180.0f;
                            const double along = (pf->pos.x - prev.x) * std::sin(h) + (pf->pos.y - prev.y) * std::cos(h);
                            const double nominal = tr.speed[now] * (now - prevT) / 1000.0;
                            if (nominal > 0) R.jitterPct = std::max(R.jitterPct, std::fabs(along - nominal) / nominal * 100.0);
                            if (along < -0.5) ++R.reversals;
                        }
                    }
                }
                prev = pf->pos;
                prevT = now;
                havePrev = true;
            }
        }
        // ---- compare, every ms
        auto* pf = eng.Get(pm.RefOf(1));
        if (!pf || pm.PhaseOf(1) != Phase::Driving || now < 1000) continue;
        ++R.frames;
        const Vec3 truth = tr.pos[lastSFrame];
        const double err = Dist2(pf->pos, truth);
        errAt[now] = err;
        puppetAt[now] = pf->pos;
        have[now] = 1;
        if (tr.speed[now] > 20.0f) movingErr.push_back(err);
        R.errMax = std::max(R.errMax, err);
        if (now >= tr.steady[0].a && now <= tr.steady[0].b) {
            const float h = tr.heading[now] * 3.14159265f / 180.0f;
            const double behind = (truth.x - pf->pos.x) * std::sin(h) + (truth.y - pf->pos.y) * std::cos(h);
            steadyLagSum += behind / tr.speed[now] * 1000.0;
            ++steadyLagN;
        }
    }
    R.runBytesPerSec = (bytesAt39s - bytesAt2s) / 1.9;
    if (o.track == 1) {
        R.arriveMs = 9999;
        uint32_t lastBad = kJumpAtMs;
        bool any = false;
        for (uint32_t t = kJumpAtMs; t <= tr.durMs; ++t)
            if (have[t] && errAt[t] > 40.0) { lastBad = t + 1; any = true; }
        if (have[tr.durMs] && errAt[tr.durMs] <= 40.0) R.arriveMs = any ? lastBad - kJumpAtMs : 0;
    }
    if (o.track == 2)
        for (uint32_t t = kStartAtMs; t <= kStartAtMs + 1500 && t <= tr.durMs; ++t)
            if (have[t]) R.startMax = std::max(R.startMax, errAt[t]);

    std::sort(movingErr.begin(), movingErr.end());
    if (!movingErr.empty()) R.errP95 = movingErr[std::min(movingErr.size() - 1, static_cast<size_t>(0.95 * movingErr.size()))];
    R.steadyLagMs = steadyLagN ? steadyLagSum / steadyLagN : 0;

    // stops: overshoot beyond the final position, and the time to settle
    for (const auto& st : tr.stops) {
        const Vec3 fin = tr.pos[st.atMs];
        const float h = st.headingDeg * 3.14159265f / 180.0f;
        double settle80 = 0, settle10 = 0;
        uint32_t until = st.atMs;   // the window ends where the walker moves again
        while (until < tr.durMs && until < st.atMs + 1500 && tr.speed[until] < 5.0f) ++until;
        for (uint32_t t = st.atMs; t <= until; ++t) {
            if (!have[t]) continue;
            const double over = (puppetAt[t].x - fin.x) * std::sin(h) + (puppetAt[t].y - fin.y) * std::cos(h);
            R.overshoot = std::max(R.overshoot, over);
            if (errAt[t] > 80.0) settle80 = t + 1 - st.atMs;
            if (errAt[t] > 10.0) settle10 = t + 1 - st.atMs;
        }
        R.settle80Ms = std::max(R.settle80Ms, settle80);
        R.settle10Ms = std::max(R.settle10Ms, settle10);
    }
    const double secs = tr.durMs / 1000.0;
    R.bytesPerSec = sender.stats().bytes / secs;
    R.packetsPerSec = sender.stats().packets / secs;
    R.snapshotsPerSec = snapshotsSent / secs;
    return R;
}

SimResult Worst(const SimOpts& base, const std::vector<uint64_t>& seeds) {
    SimResult w;
    for (uint64_t s : seeds) {
        SimOpts o = base;
        o.seed = s;
        SimResult r = RunSim(o);
        w.errP95 = std::max(w.errP95, r.errP95);
        w.errMax = std::max(w.errMax, r.errMax);
        w.overshoot = std::max(w.overshoot, r.overshoot);
        w.settle80Ms = std::max(w.settle80Ms, r.settle80Ms);
        w.settle10Ms = std::max(w.settle10Ms, r.settle10Ms);
        w.teleports = std::max(w.teleports, r.teleports);
        w.snapped = std::max(w.snapped, r.snapped);
        w.jitterPct = std::max(w.jitterPct, r.jitterPct);
        w.reversals = std::max(w.reversals, r.reversals);
        w.steadyLagMs = std::max(w.steadyLagMs, r.steadyLagMs);
        w.bytesPerSec = std::max(w.bytesPerSec, r.bytesPerSec);
        w.packetsPerSec = std::max(w.packetsPerSec, r.packetsPerSec);
        w.snapshotsPerSec = std::max(w.snapshotsPerSec, r.snapshotsPerSec);
        w.runBytesPerSec = std::max(w.runBytesPerSec, r.runBytesPerSec);
        w.arriveMs = std::max(w.arriveMs, r.arriveMs);
        w.startMax = std::max(w.startMax, r.startMax);
        w.frames = r.frames;
    }
    return w;
}

const std::vector<std::string>& Profiles() {
    static const std::vector<std::string> v = {"lan", "good_wifi", "bad_wifi", "mobile_tether", "transatlantic"};
    return v;
}

void PrintRow(const char* what, const std::string& profile, const SimResult& r) {
    std::printf("  %-8s %-14s p95 %5.1f  max %6.1f  overshoot %5.1f  settle80 %4.0f ms  settle10 %4.0f ms  teleports %u snaps %u  jitter %5.1f%%  reverse %u  lag %5.1f ms  %4.0f B/s avg %4.0f B/s running\n",
                what, profile.c_str(), r.errP95, r.errMax, r.overshoot, r.settle80Ms, r.settle10Ms, r.teleports, r.snapped, r.jitterPct, r.reversals,
                r.steadyLagMs, r.bytesPerSec, r.runBytesPerSec);
}

const std::vector<uint64_t> kSeeds = {1, 2, 3};

// The limits the shipped tuning must keep (units; run speed 400 u/s, 60 fps, worst of 3 seeds). The goal of the round was p95 under 40 on a clean link and
// under 60 behind bad wifi; the measured values (tools: `avatar_lag_tests --report`) leave about a quarter of room. The "before" numbers are in --report.
struct Limit { const char* profile; double p95, max, overshoot, jitterPct; };
const Limit kLimits[] = {
    {"lan", 24, 45, 9, 40},          {"good_wifi", 28, 48, 9, 45},        {"bad_wifi", 45, 70, 9, 70},
    {"mobile_tether", 62, 80, 22, 70}, {"transatlantic", 64, 80, 9, 45},
};

}  // namespace

int main(int argc, char** argv) {
    bool report = false;
    uint32_t fps = 60;
    float speed = 1.0f;
    for (int i = 1; i < argc; ++i) {
        if (!std::strcmp(argv[i], "--report")) report = true;
        else if (!std::strcmp(argv[i], "--fps") && i + 1 < argc) fps = static_cast<uint32_t>(std::atoi(argv[++i]));
        else if (!std::strcmp(argv[i], "--speed") && i + 1 < argc) speed = static_cast<float>(std::atof(argv[++i])) / 400.0f;
    }
    const uint32_t frameMs = fps ? 1000 / fps : 16;
    if (report) {
        std::printf("avatar lag report: run speed %.0f u/s, %u fps (%u ms frames), worst of %zu seeds per row; units are game units (70 per metre)\n", 400.0f * speed, fps, frameMs, kSeeds.size());
        for (const std::string& p : Profiles()) {
            SimOpts o;
            o.profile = p;
            o.frameMs = frameMs;
            o.speedScale = speed;
            o.tune = LegacyLagTuning();
            PrintRow("before", p, Worst(o, kSeeds));
            o.tune = DefaultLagTuning();
            if (const char* e = std::getenv("LAG_TUNE")) {   // sweeps: LAG_TUNE="minDelayMs=10,jitterFactor=0.5" (same keys as nvcoop.ini [avatar])
                const std::string spec = std::string(",") + e + ",";
                ApplyLagIni(o.tune, [&](const char* key, const char*) -> std::string {
                    const size_t at = spec.find(std::string(",") + key + "=");
                    if (at == std::string::npos) return "";
                    const size_t b = at + std::strlen(key) + 2;
                    return spec.substr(b, spec.find(',', b) - b);
                });
            }
            PrintRow("after", p, Worst(o, kSeeds));
        }
        std::printf("the lab's walk room (legs of 300 units at 375 u/s, 0.4 s stops, 90 degree corners; error p95 / max while moving, lag of the steady stretches is not defined here):\n");
        for (const std::string& p : Profiles()) {
            for (int after = 0; after < 2; ++after) {
                SimOpts o;
                o.profile = p;
                o.frameMs = frameMs;
                o.tune = after ? DefaultLagTuning() : LegacyLagTuning();
                o.track = 3;
                const SimResult w = Worst(o, kSeeds);
                std::printf("  %-8s %-14s p95 %5.1f  max %6.1f  overshoot %5.1f  settle10 %4.0f ms\n", after ? "after" : "before", p.c_str(), w.errP95, w.errMax, w.overshoot, w.settle10Ms);
            }
        }
        std::printf("teleport of 450 units (arrival: until the puppet is within 40 units for good) and a start from a 6 s stand (largest error in the following 1.5 s):\n");
        for (const std::string& p : Profiles()) {
            for (int after = 0; after < 2; ++after) {
                SimOpts o;
                o.profile = p;
                o.frameMs = frameMs;
                o.tune = after ? DefaultLagTuning() : LegacyLagTuning();
                o.track = 1;
                const SimResult t = Worst(o, kSeeds);
                o.track = 2;
                const SimResult st = Worst(o, kSeeds);
                std::printf("  %-8s %-14s teleport arrives after %5.0f ms   start max error %5.1f units\n", after ? "after" : "before", p.c_str(), t.arriveMs, st.startMax);
            }
        }
        return 0;
    }

    // ---- regression: the shipped tuning, every profile
    for (const Limit& L : kLimits) {
        SimOpts o;
        o.profile = L.profile;
        o.tune = DefaultLagTuning();
        o.mutate = true;
        SimResult r = Worst(o, kSeeds);
        PrintRow("after", L.profile, r);
        CHECK(r.errP95 <= L.p95);
        CHECK(r.errMax <= L.max);
        CHECK(r.overshoot <= L.overshoot);    // a stop does not run past where the walker stopped
        CHECK(r.teleports == 0);              // no teleport, no rubber band
        CHECK(r.snapped == 0);
        CHECK(r.reversals == 0);              // a steady run never steps backwards
        CHECK(r.jitterPct <= L.jitterPct);    // and its per-frame steps stay close to the nominal one (the old buffer: 17 to 240 percent)
        CHECK(r.settle10Ms <= 250);           // the puppet is on the walker again 250 ms after a stop
        CHECK(r.runBytesPerSec <= 800.0);     // 30 Hz near stream: well under a kilobyte a second of snapshot payload per player
    }
    // ---- a teleport and a start from a long stand (the lab's spawn room: "the puppet arrives 650 to 2700 ms late, catching up from 773 units away")
    for (const char* p : {"lan", "good_wifi", "bad_wifi"}) {
        SimOpts o;
        o.profile = p;
        o.tune = DefaultLagTuning();
        o.mutate = true;
        o.track = 1;
        const SimResult t = Worst(o, kSeeds);
        o.track = 2;
        const SimResult st = Worst(o, kSeeds);
        std::printf("  after    %-14s teleport arrives after %5.0f ms   start max error %5.1f units\n", p, t.arriveMs, st.startMax);
        const bool lan = std::strcmp(p, "lan") == 0;
        CHECK(t.arriveMs <= (lan ? 80.0 : (std::strcmp(p, "good_wifi") == 0 ? 100.0 : 140.0)));   // the link and the buffer delay, not a catch-up (the old rules: 375 to 420 ms)
        CHECK(st.startMax <= (lan ? 40.0 : (std::strcmp(p, "good_wifi") == 0 ? 45.0 : 65.0)));    // the old rules: 100 to 130 units in the first 1.5 s after a long stand
    }
    // ---- slower game copies (two copies on one PC at 30 and 20 fps): the frame length sets the floor, the shipped tuning stays well inside the old numbers
    for (uint32_t frame : {33u, 50u}) {
        SimOpts o;
        o.frameMs = frame;
        double p95[2] = {0, 0};
        for (int i = 0; i < 2; ++i) {
            o.profile = i == 0 ? "lan" : "bad_wifi";
            o.tune = DefaultLagTuning();
            o.mutate = true;
            SimResult r = Worst(o, kSeeds);
            std::printf("  after @ %u ms frames:", frame);
            PrintRow("", o.profile, r);
            p95[i] = r.errP95;
            CHECK(r.teleports == 0);
            CHECK(r.reversals == 0);
        }
        CHECK(p95[0] <= (frame == 33 ? 32.0 : 48.0));
        CHECK(p95[1] <= (frame == 33 ? 50.0 : 66.0));
    }
    // ---- the claim itself: the shipped tuning is much closer than the old one, on a clean link and behind bad wifi
    for (const char* p : {"lan", "bad_wifi"}) {
        SimOpts o;
        o.profile = p;
        o.tune = LegacyLagTuning();
        SimResult before = Worst(o, kSeeds);
        o.tune = DefaultLagTuning();
        o.mutate = true;
        SimResult after = Worst(o, kSeeds);
        PrintRow("before", p, before);
        CHECK(after.errP95 * (std::strcmp(p, "lan") == 0 ? 2.0 : 1.4) <= before.errP95);
    }
    std::printf("avatar lag: %d checks, %d failed\n", g_checks, g_fail);
    return g_fail ? 1 : 0;
}
