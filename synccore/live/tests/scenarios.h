// Scenarios for the live core. Each returns true when the property holds. The real tests CHECK(scenario()); the
// mutation twins switch on one LiveFaults flag and CHECK(!scenario()), so every scenario is proven to be able to fail.
#pragma once
#include <cmath>
#include <cstdint>
#include <cstdio>
#include <vector>
#include "live.h"
#include "fnvcoop/wire.h"

namespace live_test {

using namespace fnvcoop;
using namespace fnvcoop::live;

constexpr PlayerId kOwner = 1;   // the machine that owns the actor
constexpr PlayerId kHost = 0;
constexpr PlayerId kClient = 2;

inline ActorSnapshot mk(NetId id, OwnerEpoch epoch, uint32_t seq, uint32_t tick, float x = 0, float y = 0, float z = 0) {
    ActorSnapshot s;
    s.netId = id; s.ownerEpoch = epoch; s.seq = seq; s.tick = tick; s.cell = 7;
    s.pos = {x, y, z};
    return s;
}

// One snapshot through sender -> receiver with no loss. Returns the verdict of the first result.
struct Pair {
    SnapshotSender tx;
    SnapshotReceiver rx;
    explicit Pair(ReceiverConfig rc = {}, SenderConfig sc = {}) : tx(sc), rx(rc) {}
    // Sends `s` from sender to receiver (peer ids: sender talks to kHost, receiver sees it from kOwner).
    std::vector<ReceivedSnapshot> send(const ActorSnapshot& s, bool deliver = true, bool ack = true) {
        std::vector<ReceivedSnapshot> out;
        if (!tx.queue(kHost, s, s.tick)) return out;
        for (auto& pkt : tx.flush(kHost, s.tick)) {
            if (!deliver) continue;
            auto r = rx.receive(kOwner, pkt.data(), pkt.size());
            out.insert(out.end(), r.begin(), r.end());
        }
        if (ack) pumpAcks();
        return out;
    }
    void pumpAcks() {
        Bytes a = rx.takeAcks(kOwner);
        if (!a.empty()) tx.onAckPacket(kHost, a.data(), a.size());
    }
};

inline bool sameSnap(const ActorSnapshot& a, const ActorSnapshot& b) {
    if (a.netId != b.netId || a.ownerEpoch != b.ownerEpoch || a.seq != b.seq || a.tick != b.tick || a.cell != b.cell) return false;
    if (!(a.pos == b.pos) || !(a.rot == b.rot) || !(a.vel == b.vel)) return false;
    if (!(a.anim == b.anim) || !(a.ai == b.ai)) return false;
    return a.avDeltas == b.avDeltas;
}

// ---- codec / receiver scenarios ---------------------------------------------------------------------------------
// Snapshot 2 is lost; snapshot 3 must be a delta against the last ACKED one (1), which the receiver has.
inline bool deltaUsesAckedBaseline() {
    Pair p;
    auto s1 = mk(10, 1, 1, 1000, 100, 200, 300);
    auto s2 = mk(10, 1, 2, 1050, 110, 200, 300);
    auto s3 = mk(10, 1, 3, 1100, 120, 200, 300);
    auto r1 = p.send(s1);
    if (r1.size() != 1 || r1[0].verdict != Verdict::Accepted) return false;
    p.send(s2, /*deliver=*/false);
    auto r3 = p.send(s3);
    if (r3.size() != 1 || r3[0].verdict != Verdict::Accepted) return false;
    ActorSnapshot want = normalized(s3);
    want.avDeltas.clear();
    return sameSnap(r3[0].snap, want);
}

inline bool staleSeqRejected() {
    Pair p;
    Bytes replay;
    for (uint32_t i = 1; i <= 3; ++i) {
        auto s = mk(11, 1, i, 1000 + i * 50, 10.0f * i);
        p.tx.queue(kHost, s, s.tick);
        auto pk = p.tx.flush(kHost, s.tick);
        if (i == 2) replay = pk[0];
        p.rx.receive(kOwner, pk[0].data(), pk[0].size());
    }
    auto r = p.rx.receive(kOwner, replay.data(), replay.size());
    if (r.size() != 1) return false;
    return r[0].verdict == Verdict::StaleSeq && p.rx.lastSeqOf(11) == 3;
}

inline bool staleEpochRejected() {
    Pair a, b;   // a = new owner (epoch 2), b = the former owner still publishing at epoch 1
    SnapshotReceiver rx;
    for (uint32_t i = 1; i <= 2; ++i) {
        auto s = mk(12, 1, i, 1000 + i * 50);
        b.tx.queue(kHost, s, s.tick);
        for (auto& pk : b.tx.flush(kHost, s.tick)) rx.receive(kOwner, pk.data(), pk.size());
    }
    auto s2 = mk(12, 2, 1, 1200, 500);
    a.tx.queue(kHost, s2, s2.tick);
    for (auto& pk : a.tx.flush(kHost, s2.tick)) { auto r = rx.receive(kClient, pk.data(), pk.size()); if (r.empty() || r[0].verdict != Verdict::Accepted) return false; }
    auto old = mk(12, 1, 3, 1250, -999);
    b.tx.queue(kHost, old, old.tick);
    for (auto& pk : b.tx.flush(kHost, old.tick)) {
        auto r = rx.receive(kOwner, pk.data(), pk.size());
        if (r.size() != 1 || r[0].verdict != Verdict::StaleEpoch) return false;
    }
    return rx.epochOf(12) == 2;
}

inline bool epochRestartsSeq() {
    Pair p;
    for (uint32_t i = 1; i <= 5; ++i) p.send(mk(13, 1, i, 1000 + i * 50));
    SnapshotSender tx2;   // the new owner starts a fresh sender and a fresh seq
    auto s = mk(13, 2, 1, 1400, 5);
    tx2.queue(kHost, s, s.tick);
    for (auto& pk : tx2.flush(kHost, s.tick)) {
        auto r = p.rx.receive(kClient, pk.data(), pk.size());
        if (r.size() != 1 || r[0].verdict != Verdict::Accepted) return false;
    }
    return p.rx.epochOf(13) == 2 && p.rx.lastSeqOf(13) == 1;
}

inline bool wrongOwnerRejected() {
    Pair p;
    p.rx.setAuthority(14, 2, kOwner);
    auto s = mk(14, 2, 1, 1000);
    p.tx.queue(kHost, s, s.tick);
    auto pk = p.tx.flush(kHost, s.tick);
    auto bad = p.rx.receive(kClient, pk[0].data(), pk[0].size());   // sent by someone who is not the owner
    if (bad.size() != 1 || bad[0].verdict != Verdict::WrongOwner) return false;
    auto good = p.rx.receive(kOwner, pk[0].data(), pk[0].size());
    return good.size() == 1 && good[0].verdict == Verdict::Accepted;
}

// The host assigns every epoch and records it before the new owner can publish. A bystander that publishes a HIGHER epoch than the recorded one
// must be refused and must not raise the track's epoch (that would make the real owner's stream look stale).
inline bool forgedFutureEpochRejected() {
    Pair p;   // host-side receiver (checkSenderIsOwner)
    p.rx.setAuthority(20, 2, kOwner);
    auto r0 = p.send(mk(20, 2, 1, 1000));
    if (r0.size() != 1 || r0[0].verdict != Verdict::Accepted) return false;
    SnapshotSender forger;
    forger.queue(kHost, mk(20, 9, 1, 1050, 666), 1050);
    auto pk = forger.flush(kHost, 1050);
    if (pk.empty()) return false;
    auto bad = p.rx.receive(kClient, pk[0].data(), pk[0].size());
    if (bad.size() != 1 || bad[0].verdict != Verdict::WrongOwner) return false;
    if (p.rx.epochOf(20) != 2) return false;
    auto r1 = p.send(mk(20, 2, 2, 1100, 5));   // (a changed position: an identical state would be skipped as idle by the sender)
    return r1.size() == 1 && r1[0].verdict == Verdict::Accepted;
}

inline bool authorityFloorRejects() {
    Pair p;
    p.rx.setAuthority(15, 5, kOwner);
    auto s = mk(15, 4, 1, 1000);
    auto r = p.send(s);
    return r.size() == 1 && r[0].verdict == Verdict::StaleEpoch;
}

// An AV changed in a snapshot that was lost must still arrive with the next one even though the owner no longer lists it.
inline bool avSurvivesLoss() {
    Pair p;
    auto s1 = mk(16, 1, 1, 1000); s1.avDeltas = {{0, 100}, {1, 50}};
    auto s2 = mk(16, 1, 2, 1050); s2.avDeltas = {{0, 80}};
    auto s3 = mk(16, 1, 3, 1100);
    std::map<uint8_t, int32_t> seen;
    auto apply = [&](const std::vector<ReceivedSnapshot>& r) { for (auto& x : r) for (auto& v : x.snap.avDeltas) seen[v.av] = v.value; };
    apply(p.send(s1));
    p.send(s2, false);
    apply(p.send(s3));
    return seen[0] == 80 && seen[1] == 50;
}

// After the receiver loses its state, a full snapshot must arrive within a few snapshots (receiver asks for it).
inline bool resyncAfterReceiverLoss() {
    Pair p;
    uint32_t seq = 0;
    for (; seq < 10; ++seq) p.send(mk(17, 1, seq + 1, 1000 + seq * 50, static_cast<float>(seq)));
    p.rx.forget(17);
    int attempts = 0;
    bool ok = false;
    for (; attempts < 6 && !ok; ++attempts) {
        ++seq;
        auto r = p.send(mk(17, 1, seq + 1, 1000 + seq * 50, static_cast<float>(seq)));
        ok = !r.empty() && r[0].verdict == Verdict::Accepted;
    }
    return ok;
}

// Steady state: a moving actor is sent as small deltas; a baseline that never advances keeps sending full snapshots.
inline bool deltaIsSmall() {
    SenderConfig sc;
    Pair p({}, sc);
    for (uint32_t i = 1; i <= 40; ++i) {
        auto s = mk(18, 1, i, 1000 + i * 50, 5.0f * i, 2.0f * i, 0);
        s.vel = {100, 40, 0};
        s.avDeltas = {{0, 100}, {1, 50}, {2, 30}};
        p.send(s);
    }
    const SenderStats& st = p.tx.stats();
    double avg = static_cast<double>(st.bytes) / static_cast<double>(st.packets);
    return st.deltaSent >= 38 && avg < 22.0;
}

// Skip an unchanged actor until the heartbeat is due.
inline bool idleSkippedThenHeartbeat() {
    Pair p;
    int sent = 0;
    for (uint32_t i = 1; i <= 100; ++i) {   // 5 s at 20 Hz, never moving
        auto s = mk(19, 1, i, 1000 + i * 50, 3, 4, 5);
        if (!p.send(s).empty()) ++sent;
    }
    // 1 full + the 2nd (until acked) + heartbeats at 1 s: about 5 or 6; never one per snapshot.
    return sent >= 4 && sent <= 8;
}

// ---- interpolation scenarios ------------------------------------------------------------------------------------
// Linear motion at 500 units/s along x, snapshots every 50 ms, 30 ms latency.
struct Feed {
    InterpBuffer ib;
    uint32_t t0 = 1000, latency = 30;
    explicit Feed(InterpConfig c = {}) : ib(c) {}
    static float xAt(uint32_t tick) { return (static_cast<float>(tick) - 1000.0f) * 0.5f; }
    ActorSnapshot at(uint32_t k) const {
        ActorSnapshot s = mk(20, 1, k + 1, t0 + k * 50, xAt(t0 + k * 50));
        s.vel = {500, 0, 0};
        return s;
    }
    void pushUpTo(uint32_t k) { for (uint32_t i = 0; i <= k; ++i) ib.push(at(i), t0 + i * 50 + latency); }
};

inline bool interpMidpoint() {
    // Accelerating path (x = 10 k^2 units at snapshot k, no velocity data): only a real interpolation lands on the
    // midpoint of two snapshots; dead reckoning from the newest snapshot cannot.
    InterpBuffer ib;
    for (uint32_t k = 0; k <= 10; ++k) ib.push(mk(20, 1, k + 1, 1000 + k * 50, 10.0f * k * k), 1000 + k * 50 + 30);
    // Render time = now - 30 - 100. At now = 1500 + 30 + 25 the render tick is 1425: half way between snapshots 8 and 9.
    InterpOut o = ib.sample(1500 + 30 + 25);
    return o.state == InterpState::Interpolating && std::fabs(o.pos.x - 0.5f * (640.0f + 810.0f)) < 0.01f;
}

inline bool interpDelayIs100ms() {
    Feed f;
    f.pushUpTo(10);
    InterpOut o = f.ib.sample(1500 + 30);   // arrival time of the newest: renders 100 ms back
    return std::fabs(o.pos.x - Feed::xAt(1400)) < 0.01f;
}

inline bool extrapolationCapped(float* heldX = nullptr) {
    Feed f;
    f.pushUpTo(10);
    // Samples every 16 ms from when the newest snapshot should have been shown, up to 1.2 s later.
    InterpOut o;
    bool sawExtrap = false;
    for (uint32_t now = 1500 + 30 + 100; now <= 1500 + 30 + 100 + 1200; now += 16) {
        o = f.ib.sample(now);
        if (o.state == InterpState::Extrapolating) sawExtrap = true;
    }
    if (heldX) *heldX = o.pos.x;
    float cap = Feed::xAt(1500) + 500.0f * 0.25f;
    return sawExtrap && o.state == InterpState::Holding && std::fabs(o.pos.x - cap) < 0.5f;
}

inline bool teleportSnaps() {
    InterpBuffer ib;
    // stationary at x=1000 then a 30 m (2100 unit) teleport at tick 1500
    uint32_t snaps = 0;
    for (uint32_t k = 0; k <= 20; ++k) {
        float x = k < 10 ? 1000.0f : 3100.0f;
        ib.push(mk(21, 1, k + 1, 1000 + k * 50, x), 1000 + k * 50 + 30);
    }
    bool intermediate = false;
    float last = 1000;
    for (uint32_t now = 1000 + 30 + 100; now <= 2000 + 30; now += 16) {
        InterpOut o = ib.sample(now);
        if (o.snapped) { ++snaps; if (std::fabs(o.pos.x - 3100.0f) > 0.01f) intermediate = true; }
        if (std::fabs(o.pos.x - 1000.0f) > 0.01f && std::fabs(o.pos.x - 3100.0f) > 0.01f) intermediate = true;
        last = o.pos.x;
    }
    return snaps == 1 && !intermediate && std::fabs(last - 3100.0f) < 0.01f;
}

inline bool cellChangeSnaps() {
    InterpBuffer ib;
    for (uint32_t k = 0; k <= 20; ++k) {
        ActorSnapshot s = mk(22, 1, k + 1, 1000 + k * 50, k < 10 ? 0.0f : 10.0f);   // only 10 units (14 cm) apart
        s.cell = k < 10 ? 1 : 2;
        ib.push(s, 1000 + k * 50 + 30);
    }
    uint32_t snaps = 0;
    bool intermediate = false;
    for (uint32_t now = 1000 + 30 + 100; now <= 2000 + 30; now += 16) {
        InterpOut o = ib.sample(now);
        if (o.snapped) ++snaps;
        if (std::fabs(o.pos.x) > 0.01f && std::fabs(o.pos.x - 10.0f) > 0.01f) intermediate = true;
    }
    return snaps == 1 && !intermediate;
}

// The puppet was shoved 5 m by physics. The buffer must ease it back, not pop it, and not overshoot.
inline bool shoveIsSmoothed() {
    InterpBuffer ib;
    for (uint32_t k = 0; k <= 60; ++k) ib.push(mk(23, 1, k + 1, 1000 + k * 50, 0), 1000 + k * 50 + 30);
    ib.sample(1130);   // settle
    ActualPose actual{7, {350, 0, 0}};   // 5 m
    uint32_t now = 1130 + 16;
    InterpOut first = ib.sample(now, &actual);
    if (first.state != InterpState::Smoothing) return false;
    if (first.pos.x >= 350.0f || first.pos.x <= 0.0f) return false;
    float prevErr = first.pos.x;
    int frames = 0;
    InterpOut o = first;
    while (o.pos.x > 0.01f && frames < 400) {
        actual.pos = o.pos;
        now += 16; ++frames;
        o = ib.sample(now, &actual);
        if (o.pos.x < -0.01f) return false;              // overshoot
        if (o.pos.x > prevErr + 0.001f) return false;    // must keep approaching
        prevErr = o.pos.x;
    }
    return o.pos.x <= 0.01f && frames < 300;
}

// 1.5 m of error is followed exactly (no smoothing lag), 3 m is eased.
inline bool smallErrorFollowsBigErrorEases() {
    InterpBuffer ib;
    for (uint32_t k = 0; k <= 60; ++k) ib.push(mk(24, 1, k + 1, 1000 + k * 50, 0), 1000 + k * 50 + 30);
    ib.sample(1130);
    ActualPose small{7, {105, 0, 0}};   // 1.5 m
    InterpOut a = ib.sample(1146, &small);
    if (a.state == InterpState::Smoothing || std::fabs(a.pos.x) > 0.01f) return false;
    ActualPose big{7, {210, 0, 0}};     // 3 m
    InterpOut b = ib.sample(1162, &big);
    return b.state == InterpState::Smoothing && b.pos.x > 0.0f && b.pos.x < 210.0f;
}

// Exactly on the thresholds: 10 m is not a snap, 2 m is not a smooth.
inline bool thresholdBoundaries() {
    auto run = [](float metres, bool* snapped, bool* smoothing) {
        InterpBuffer ib;
        for (uint32_t k = 0; k <= 60; ++k) ib.push(mk(25, 1, k + 1, 1000 + k * 50, 0), 1000 + k * 50 + 30);
        ib.sample(1130);
        ActualPose a{7, {metres * 70.0f, 0, 0}};
        InterpOut o = ib.sample(1146, &a);
        *snapped = o.snapped;
        *smoothing = o.state == InterpState::Smoothing;
    };
    bool sn, sm;
    run(10.0f, &sn, &sm); if (sn || !sm) return false;
    run(10.1f, &sn, &sm); if (!sn) return false;
    run(2.0f, &sn, &sm); if (sn || sm) return false;
    run(2.1f, &sn, &sm); if (sn || !sm) return false;
    return true;
}

inline bool staleAndDuplicatePushIgnored() {
    InterpBuffer ib;
    if (!ib.push(mk(26, 1, 2, 1100, 5), 1130)) return false;
    if (ib.push(mk(26, 1, 2, 1100, 99), 1131)) return false;   // same tick
    if (ib.push(mk(26, 1, 1, 1050, 77), 1132)) return false;   // older tick
    return ib.size() == 1;
}

inline bool tickWrapIsHandled() {
    InterpBuffer ib;
    uint32_t base = 0xFFFFFFFFu - 200;
    for (uint32_t k = 0; k < 12; ++k) {
        ActorSnapshot s = mk(27, 1, k + 1, base + k * 50, static_cast<float>(k) * 25.0f);   // 500 units/s
        s.vel = {500, 0, 0};
        if (!ib.push(s, s.tick + 30)) return false;
    }
    InterpOut o = ib.sample(base + 11 * 50 + 30);   // renders tick base+450 => k = 9
    return std::fabs(o.pos.x - 9.0f * 25.0f) < 0.05f && o.state == InterpState::Interpolating;
}

inline bool rotationTakesShortArc() {
    InterpBuffer ib;
    const float twoPi = 6.2831853f;
    for (uint32_t k = 0; k < 10; ++k) {
        ActorSnapshot s = mk(28, 1, k + 1, 1000 + k * 50, 0);
        s.rot = {0, (k % 2 == 0) ? 0.1f : twoPi - 0.1f, 0};   // 0.1 <-> -0.1 across the wrap
        ib.push(s, s.tick + 30);
    }
    // halfway between snapshot 4 (tick 1200, yaw 0.1) and 5 (tick 1250, yaw -0.1): must be about 0, not about pi
    InterpOut o = ib.sample(1225 + 30 + 100);
    float y = o.rot.y;
    if (y > twoPi / 2) y -= twoPi;
    return std::fabs(y) < 0.02f;
}

// A 5 Hz stream (far tier) must still be interpolated, never extrapolated or held, while it keeps arriving.
inline bool slowStreamInterpolates() {
    Feed f;
    InterpOut o;
    int bad = 0, n = 0;
    for (uint32_t k = 0; k <= 25; ++k) {
        ActorSnapshot s = mk(29, 1, k + 1, 1000 + k * 200, (k * 200) * 0.5f);
        s.vel = {500, 0, 0};
        f.ib.push(s, s.tick + 30);
        for (uint32_t now = s.tick + 30; now < s.tick + 30 + 200; now += 16) {
            o = f.ib.sample(now);
            if (k >= 8) { ++n; if (o.state != InterpState::Interpolating) ++bad; }
        }
    }
    return n > 0 && bad == 0;
}

// ---- interest scenarios -----------------------------------------------------------------------------------------
inline InterestManager makeInterest(uint32_t* nearCount, uint32_t* farCount, uint32_t* adjCount, uint32_t* otherCount, uint32_t stepMs, uint32_t durationMs) {
    InterestManager im;
    Observer o; o.player = kClient; o.cell = 1; o.pos = {0, 0, 0}; o.adjacent = {2};
    im.setObserver(o);
    im.setActor({100, 1, {500, 0, 0}, kOwner});     // near
    im.setActor({101, 1, {9000, 0, 0}, kOwner});    // same cell, far away
    im.setActor({102, 2, {0, 0, 0}, kOwner});       // adjacent cell
    im.setActor({103, 3, {0, 0, 0}, kOwner});       // unrelated cell
    for (uint32_t t = 0; t < durationMs; t += stepMs) {
        for (NetId id : im.collect(kClient, t)) {
            if (id == 100) ++*nearCount; else if (id == 101) ++*farCount; else if (id == 102) ++*adjCount; else ++*otherCount;
        }
    }
    return im;
}

inline bool ratesAre20And5() {
    uint32_t n = 0, f = 0, a = 0, x = 0;
    makeInterest(&n, &f, &a, &x, 1, 10000);
    return n == 200 && f == 50 && a == 50 && x == 0;
}
inline bool ratesHoldAtFrameRate() {
    uint32_t n = 0, f = 0, a = 0, x = 0;
    makeInterest(&n, &f, &a, &x, 16, 10000);   // about 60 fps polling
    return n >= 198 && n <= 202 && f >= 49 && f <= 51;
}

// ---- schema v2 animation extension (enemy-anim) ---------------------------------------------------------------------
inline AnimState richAnim() {
    AnimState a;
    a.locomotion = 5; a.weaponDrawn = true; a.aim = true;
    a.action = 223; a.actionSeq = 254; a.dead = false; a.knocked = true; a.swim = true; a.talk = true; a.block = true; a.ragdoll = true;
    return a;
}

// The extension survives a full snapshot and a following delta, field by field.
inline bool animExtRoundTrips() {
    Pair p;
    ActorSnapshot s1 = mk(40, 1, 1, 1000, 10, 20, 30);
    s1.anim = richAnim();
    auto r1 = p.send(s1);
    if (r1.size() != 1 || r1[0].verdict != Verdict::Accepted || !(r1[0].snap.anim == s1.anim)) return false;
    ActorSnapshot s2 = s1; s2.seq = 2; s2.tick = 1050; s2.pos.x += 5;
    s2.anim.action = 172; s2.anim.actionSeq = 255; s2.anim.dead = true;
    auto r2 = p.send(s2);
    if (r2.size() != 1 || !(r2[0].snap.anim == s2.anim)) return false;
    ActorSnapshot s3 = s2; s3.seq = 3; s3.tick = 1100; s3.anim = AnimState{};   // everything back to zero: no extension on the wire
    auto r3 = p.send(s3);
    return r3.size() == 1 && r3[0].snap.anim == s3.anim && r3[0].snap.anim.action == 0 && r3[0].snap.anim.actionSeq == 0 && !r3[0].snap.anim.dead;
}

// A change that touches ONLY the one-shot counter (same group again, nothing else moved) must reach the receiver.
inline bool animSeqOnlyChangeSent() {
    Pair p;
    ActorSnapshot s1 = mk(41, 1, 1, 1000, 10, 20, 30);
    s1.anim.action = 32; s1.anim.actionSeq = 1;
    p.send(s1);
    ActorSnapshot s2 = s1; s2.seq = 2; s2.tick = 1050; s2.anim.actionSeq = 2;   // a second swing, same group
    auto r = p.send(s2);
    return r.size() == 1 && r[0].verdict == Verdict::Accepted && r[0].snap.anim.actionSeq == 2 && r[0].snap.anim.action == 32;
}

// A one-shot announced in a LOST datagram is still delivered by the next one (deltas are against the last ACKED snapshot).
inline bool animOneShotSurvivesLoss() {
    Pair p;
    ActorSnapshot s1 = mk(42, 1, 1, 1000, 10, 20, 30);
    p.send(s1);
    ActorSnapshot s2 = s1; s2.seq = 2; s2.tick = 1050; s2.anim.action = 223; s2.anim.actionSeq = 1;
    p.send(s2, /*deliver=*/false);
    ActorSnapshot s3 = s2; s3.seq = 3; s3.tick = 1100;   // same state again, the datagram before was lost
    auto r = p.send(s3);
    return r.size() == 1 && r[0].snap.anim.action == 223 && r[0].snap.anim.actionSeq == 1;
}

// Hand-built datagram with a given anim section (CRC valid) so the decoder's own checks are what rejects it.
inline Bytes craftAnimDatagram(uint8_t loco, uint8_t flags, const Bytes& extBytes) {
    Bytes b;
    b.push_back(static_cast<uint8_t>(MsgKind::SnapshotBatch));
    b.push_back(1);
    b.push_back(43);          // netId
    b.push_back(1);           // epoch
    b.push_back(1);           // seq
    b.push_back(0);           // baseSeq (full)
    b.push_back(10);          // tick
    b.push_back(0x7F);        // mask: everything
    b.push_back(7);           // cell
    for (int i = 0; i < 3; ++i) b.push_back(0);                       // pos deltas (varint zigzag)
    for (int i = 0; i < 6; ++i) b.push_back(0);                       // rot (3 x u16)
    for (int i = 0; i < 3; ++i) b.push_back(0);                       // vel
    b.push_back(loco); b.push_back(flags);
    b.insert(b.end(), extBytes.begin(), extBytes.end());
    b.push_back(0); b.push_back(0); b.push_back(0);                   // target, package, alert
    b.push_back(0);                                                   // av count
    uint32_t c = crc32(b.data(), b.size());
    for (int i = 0; i < 4; ++i) b.push_back(static_cast<uint8_t>(c >> (8 * i)));
    return b;
}
inline Verdict decodeAnimDatagram(uint8_t loco, uint8_t flags, const Bytes& ext) {
    SnapshotReceiver rx;
    Bytes d = craftAnimDatagram(loco, flags, ext);
    auto r = rx.receive(kOwner, d.data(), d.size());
    return r.empty() ? Verdict::Malformed : r[0].verdict;
}
// The decoder accepts exactly the canonical forms: reserved flag bits, reserved extension bits and an all-zero extension are Malformed.
inline bool animExtMalformedRejected() {
    if (decodeAnimDatagram(1, 0x01, {}) != Verdict::Accepted) return false;                          // plain v1 form
    if (decodeAnimDatagram(1, 0x81, {223, 4, 0x01}) != Verdict::Accepted) return false;              // with extension
    if (decodeAnimDatagram(1, 0x21, {}) == Verdict::Accepted) return false;                          // reserved flag bit 5
    if (decodeAnimDatagram(1, 0x41, {}) == Verdict::Accepted) return false;                          // reserved flag bit 6
    if (decodeAnimDatagram(1, 0x81, {223, 4, 0x40}) == Verdict::Accepted) return false;              // furniture block announced, none follows (the AI section is read as one)
    if (decodeAnimDatagram(1, 0x81, {223, 4, 0x80}) == Verdict::Accepted) return false;              // reserved ext bit 7
    if (decodeAnimDatagram(1, 0x81, {0, 0, 0}) == Verdict::Accepted) return false;                   // all-zero extension is never sent
    if (decodeAnimDatagram(1, 0x81, {223}) == Verdict::Accepted) return false;                       // truncated extension
    return true;
}

// ---- schema v3 furniture block (sit round, 2026-10-29) -------------------------------------------------------------------
inline AnimState sittingAnim(SitState st, uint32_t furniture, bool bed = false) {
    AnimState a;
    a.sit = st; a.sleep = bed; a.furniture = furniture;
    return a;
}

// The furniture block survives a full snapshot and the deltas that follow it, field by field, alongside the v2 extension; standing up
// again puts the stream back on the plain form.
inline bool sitRoundTrips() {
    Pair p;
    ActorSnapshot s1 = mk(50, 1, 1, 1000, 10, 20, 30);
    s1.anim = sittingAnim(SitState::Using, 0x00123456u);
    auto r1 = p.send(s1);
    if (r1.size() != 1 || r1[0].verdict != Verdict::Accepted || !(r1[0].snap.anim == s1.anim)) return false;
    if (r1[0].snap.anim.sit != SitState::Using || r1[0].snap.anim.furniture != 0x00123456u || r1[0].snap.anim.sleep) return false;
    ActorSnapshot s2 = s1; s2.seq = 2; s2.tick = 1050;
    s2.anim = sittingAnim(SitState::Leaving, 0x00123456u);
    s2.anim.action = 32; s2.anim.actionSeq = 3; s2.anim.talk = true;   // together with the v2 extension
    auto r2 = p.send(s2);
    if (r2.size() != 1 || !(r2[0].snap.anim == s2.anim) || r2[0].snap.anim.sit != SitState::Leaving || r2[0].snap.anim.action != 32) return false;
    ActorSnapshot s3 = s2; s3.seq = 3; s3.tick = 1100;
    s3.anim = sittingAnim(SitState::Entering, 0xFF000A01u, /*bed=*/true);   // another piece of furniture, a bed, a created reference
    auto r3 = p.send(s3);
    if (r3.size() != 1 || !(r3[0].snap.anim == s3.anim) || !r3[0].snap.anim.sleep || r3[0].snap.anim.furniture != 0xFF000A01u) return false;
    ActorSnapshot s4 = s3; s4.seq = 4; s4.tick = 1150; s4.anim = AnimState{};   // standing
    auto r4 = p.send(s4);
    return r4.size() == 1 && r4[0].snap.anim == s4.anim && r4[0].snap.anim.sit == SitState::None && r4[0].snap.anim.furniture == 0 && !r4[0].snap.anim.sleep;
}

// A change that touches ONLY the furniture block (the actor stays where it is) must reach the receiver: that is exactly what standing up looks like.
inline bool sitOnlyChangeSent() {
    Pair p;
    ActorSnapshot s1 = mk(51, 1, 1, 1000, 10, 20, 30);
    s1.anim = sittingAnim(SitState::Using, 77);
    p.send(s1);
    ActorSnapshot s2 = s1; s2.seq = 2; s2.tick = 1050; s2.anim = sittingAnim(SitState::Leaving, 77);
    auto r2 = p.send(s2);
    if (r2.size() != 1 || r2[0].verdict != Verdict::Accepted || r2[0].snap.anim.sit != SitState::Leaving) return false;
    ActorSnapshot s3 = s2; s3.seq = 3; s3.tick = 1100; s3.anim = AnimState{};   // standing, nothing else changed
    auto r3 = p.send(s3);
    if (r3.size() != 1 || r3[0].verdict != Verdict::Accepted || r3[0].snap.anim.sit != SitState::None) return false;
    ActorSnapshot s4 = s3; s4.seq = 4; s4.tick = 1150; s4.anim = sittingAnim(SitState::Using, 78);   // a chair
    ActorSnapshot s5 = s4; s5.seq = 5; s5.tick = 1200; s5.anim = sittingAnim(SitState::Using, 79);   // another chair, same state
    p.send(s4);
    auto r5 = p.send(s5);
    return r5.size() == 1 && r5[0].snap.anim.furniture == 79;
}

// A sit announced in a LOST datagram is still delivered by the next one (deltas are against the last ACKED snapshot).
inline bool sitSurvivesLoss() {
    Pair p;
    ActorSnapshot s1 = mk(52, 1, 1, 1000, 10, 20, 30);
    p.send(s1);
    ActorSnapshot s2 = s1; s2.seq = 2; s2.tick = 1050; s2.anim = sittingAnim(SitState::Using, 0x4242);
    p.send(s2, /*deliver=*/false);
    ActorSnapshot s3 = s2; s3.seq = 3; s3.tick = 1100;
    auto r = p.send(s3);
    return r.size() == 1 && r[0].snap.anim.sit == SitState::Using && r[0].snap.anim.furniture == 0x4242;
}

// Sizes: no furniture means the v2 wire form byte for byte; a block adds the sit byte and the varint furniture (alone, also the three extension bytes).
inline bool sitWireSizes() {
    Pair a, b, c, d;
    ActorSnapshot plain = mk(53, 1, 1, 1000, 1, 2, 3);
    ActorSnapshot withExt = plain; withExt.anim.action = 32; withExt.anim.actionSeq = 1;
    ActorSnapshot sitOnly = plain; sitOnly.anim = sittingAnim(SitState::Using, 0x7F);                              // 1 byte varint
    ActorSnapshot sitWide = withExt; sitWide.anim.sit = SitState::Using; sitWide.anim.furniture = 0x01234567u;   // on top of the v2 extension, 4 byte varint
    a.tx.queue(kHost, plain, plain.tick);
    b.tx.queue(kHost, withExt, withExt.tick);
    c.tx.queue(kHost, sitOnly, sitOnly.tick);
    d.tx.queue(kHost, sitWide, sitWide.tick);
    auto pa = a.tx.flush(kHost, 1000), pb = b.tx.flush(kHost, 1000), pc = c.tx.flush(kHost, 1000), pd = d.tx.flush(kHost, 1000);
    if (pa.size() != 1 || pb.size() != 1 || pc.size() != 1 || pd.size() != 1) return false;
    if (pb[0].size() != pa[0].size() + 3) return false;                 // the v2 extension, unchanged by v3
    if (pc[0].size() != pa[0].size() + 3 + 1 + 1) return false;         // 3 extension bytes + sit byte + 1 byte furniture
    if (pd[0].size() != pb[0].size() + 1 + 4) return false;             // sit byte + 4 byte furniture
    return !animHasExt(sitOnly.anim);                                   // the furniture block is not part of the v2 extension helpers
}

// A sitting actor whose reference could not be read (0) still sits; a standing actor's stray bed flag / reference never reach the wire.
inline bool sitCanonicalForms() {
    Pair p;
    ActorSnapshot s1 = mk(54, 1, 1, 1000, 10, 20, 30);
    s1.anim = sittingAnim(SitState::Using, 0);
    auto r1 = p.send(s1);
    if (r1.size() != 1 || r1[0].snap.anim.sit != SitState::Using || r1[0].snap.anim.furniture != 0) return false;
    ActorSnapshot s2 = s1; s2.seq = 2; s2.tick = 1050;
    s2.anim.sit = SitState::None; s2.anim.sleep = true; s2.anim.furniture = 99;   // inconsistent input from a careless sampler
    const ActorSnapshot n = normalized(s2);
    if (n.anim.sit != SitState::None || n.anim.sleep || n.anim.furniture != 0) return false;
    auto r2 = p.send(s2);
    return r2.size() == 1 && r2[0].snap.anim.sit == SitState::None && r2[0].snap.anim.furniture == 0 && !r2[0].snap.anim.sleep;
}

// The decoder accepts exactly the canonical furniture blocks: state 0, reserved sit bits, a missing block and an oversized reference are Malformed.
inline bool sitMalformedRejected() {
    if (decodeAnimDatagram(1, 0x81, {0, 0, 0x40, 0x02, 0x05}) != Verdict::Accepted) return false;            // using, furniture 5
    if (decodeAnimDatagram(1, 0x81, {0, 0, 0x40, 0x07, 0x00}) != Verdict::Accepted) return false;            // a bed, leaving, reference unknown
    if (decodeAnimDatagram(1, 0x81, {32, 4, 0x41, 0x01, 0x80, 0x01}) != Verdict::Accepted) return false;    // together with the v2 extension, 2 byte reference
    if (decodeAnimDatagram(1, 0x81, {0, 0, 0x40, 0x00, 0x00}) == Verdict::Accepted) return false;           // state 0 is never sent
    if (decodeAnimDatagram(1, 0x81, {0, 0, 0x40, 0x04, 0x00}) == Verdict::Accepted) return false;           // a bed flag without a state
    if (decodeAnimDatagram(1, 0x81, {0, 0, 0x40, 0x0A, 0x00}) == Verdict::Accepted) return false;           // reserved sit bit 3
    if (decodeAnimDatagram(1, 0x81, {0, 0, 0x40, 0x82, 0x00}) == Verdict::Accepted) return false;           // reserved sit bit 7
    if (decodeAnimDatagram(1, 0x81, {0, 0, 0x40, 0x02, 0xFF, 0xFF, 0xFF, 0xFF, 0x7F}) == Verdict::Accepted) return false;   // reference wider than 32 bits
    if (decodeAnimDatagram(1, 0x81, {0, 0, 0xC0, 0x02, 0x05}) == Verdict::Accepted) return false;           // reserved extension bit 7 next to the furniture bit
    if (decodeAnimDatagram(1, 0x01, {}) != Verdict::Accepted) return false;                                  // the plain form is untouched
    return true;
}

// ---- low-latency interpolation (the player avatar stream, InterpConfig::lowLatency; round of 2026-10-27) ------------------------------
// A runner at 400 units/s along x, a snapshot every 33 ms (30 Hz), `latency` ms on the wire, the receiver frame every 16 ms.
struct LLRun {
    InterpBuffer ib;
    uint32_t latency = 30;
    uint32_t earlyK = 1000000, earlyLatency = 10;   // one snapshot that crosses the wire faster than the rest
    uint32_t stopK = 1000000;                       // from this snapshot on the runner stands still (velocity 0)
    uint32_t dropK = 1000000;                       // this snapshot is lost
    uint32_t nextK = 0;
    explicit LLRun(InterpConfig c = lowLatencyConfig()) : ib(c) {}
    static InterpConfig lowLatencyConfig() { InterpConfig c; c.lowLatency = true; return c; }
    static float xAt(uint32_t tick) { return (static_cast<float>(tick) - 1000.0f) * 0.4f; }
    ActorSnapshot snap(uint32_t k) const {
        const uint32_t kk = k < stopK ? k : stopK - 1;   // standing still: the position of the last moving snapshot
        ActorSnapshot s = mk(21, 1, k + 1, 1000 + k * 33, xAt(1000 + kk * 33));
        s.vel = k < stopK ? Vec3{400, 0, 0} : Vec3{0, 0, 0};
        return s;
    }
    uint32_t arrival(uint32_t k) const { return 1000 + k * 33 + (k == earlyK ? earlyLatency : latency); }
    // Pushes the snapshots that have arrived by `now` and samples; returns the shown x.
    float frame(uint32_t now, uint32_t lastK = 1000000) {
        while (nextK <= lastK && arrival(nextK) <= now) {
            if (nextK != dropK) ib.push(snap(nextK), arrival(nextK));
            ++nextK;
        }
        return ib.sample(now).pos.x;
    }
};

// On a clean link the shown pose is a few tens of ms behind the runner, not the old 100 ms (40 units at this speed).
inline bool llDelayFollowsJitter() {
    LLRun r;
    float worst = 0;
    for (uint32_t now = 1000; now <= 4000; now += 16) {
        float x = r.frame(now);
        if (now >= 2500) worst = std::fmax(worst, LLRun::xAt(now - r.latency) - x);   // truth is the runner at the sender's "now"
    }
    return worst > 0.0f && worst <= 20.0f;
}

// The stream stops (outage): the dead reckoning is damped, so the pose settles at most v * tau (60 units) past the last snapshot instead of
// running on to the 250 ms cap (100 units).
inline bool llExtrapolationIsDamped() {
    LLRun r;
    const uint32_t lastK = 60;
    float x = 0;
    for (uint32_t now = 1000; now <= 1000 + lastK * 33 + r.latency + 1200; now += 16) x = r.frame(now, lastK);
    const float past = x - LLRun::xAt(1000 + lastK * 33);
    return past > 30.0f && past <= 61.0f;
}

// The runner stops dead and the snapshot that would have said so is lost: the pose ran on from the older ones, and when the next one arrives it
// is faded back, not popped back in one step.
inline bool llMissIsFaded() {
    LLRun r;
    r.stopK = 61;   // snapshot 60 is the last that moves
    r.dropK = 61;
    float prev = 0, back = 0;
    for (uint32_t now = 1000; now <= 1000 + 90 * 33; now += 16) {
        float x = r.frame(now, 90);
        if (now > 2500) back = std::fmax(back, prev - x);   // the biggest step backwards
        prev = x;
    }
    return back <= 5.0f && back > 0.0f;   // the miss is about 14 units; as one step it would show as 14
}

// One snapshot crosses the wire 30 ms faster than the rest: the offset filter takes it for the new minimum, and 16 snapshots later forgets it
// again. The render clock must not follow that in one step (30 ms at this speed are 12 units).
inline bool llClockIsSlewed() {
    LLRun r;
    r.latency = 40;
    r.earlyK = 50;
    float prev = 0, worst = 0;
    for (uint32_t now = 1000; now <= 4500; now += 16) {
        float x = r.frame(now);
        if (now > 1500) worst = std::fmax(worst, std::fabs((x - prev) - 6.4f));
        prev = x;
    }
    return worst <= 3.0f;
}

// A player stood still for 6 s (the stream is one heartbeat a second) and is then 450 units away (below the old 10 m snap, above 2 m): the first snapshot of the new
// place must be on screen within a few frames of its time, as a jump. The old rules took the silence for a slow stream (350 ms of delay) and eased the 450 units
// in with a 200 ms time constant.
inline bool llTeleportAfterAStandIsAJump(float* arrivedMs = nullptr) {
    InterpBuffer ib(LLRun::lowLatencyConfig());
    uint32_t seq = 0;
    for (uint32_t t = 1000; t <= 7000; t += 1000) ib.push(mk(21, 1, ++seq, t, 1000.0f), t + 10);   // the heartbeats
    // the standing player jumps at 7450; the sender repeats the new place until it is acked (every 33 ms)
    uint32_t next = 7450;
    const uint32_t arrival = 7460;
    uint32_t got = 0;
    bool eased = false;
    for (uint32_t now = 7000; now <= 7460 + 600; now += 16) {
        while (next <= 7450 + 99 && next + 10 <= now) { ib.push(mk(21, 1, ++seq, next, 1450.0f), next + 10); next += 33; }
        InterpOut o = ib.sample(now);
        if (o.state == InterpState::Smoothing) eased = true;
        if (!got && std::fabs(o.pos.x - 1450.0f) < 20.0f) got = now;
    }
    if (arrivedMs) *arrivedMs = got ? static_cast<float>(got) - arrival : 9999.0f;
    return got && got - arrival <= 48 && !eased;
}

}  // namespace live_test
