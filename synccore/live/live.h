// Live layer core (FULL-SYNC-PLAN section 2 and 3.1/3.2): actor snapshots, delta encoding against the last acked
// snapshot, receiver-side stale rejection, interpolation buffer, interest management and the channel mapping.
// Game-free, no sockets, no clocks: every call is handed the time. Used by plugin/avatar (player puppets) and
// plugin/actors (NPCs), and by the engine model / tests.
//
// Flow (one actor, owner machine O -> host H -> client C):
//   O: SeqAllocator::next() -> ActorSnapshot -> SnapshotSender::queue(peer H) -> flush -> Channel::Unreliable
//   H: SnapshotReceiver::receive() (checks sender == owner) -> relays through its own SnapshotSender per other peer
//      (InterestManager::collect picks who gets which actor, and at which rate)
//   C: SnapshotReceiver::receive() -> InterpBuffer::push() -> InterpBuffer::sample(now) each frame -> drive puppet
//   Both receivers return acks with takeAcks(); the sender's onAckPacket() moves the delta baseline forward.
//
// Quantization: positions 1/8 unit, velocity 1/4 unit/s, angles 2pi/65536. The sender stores the QUANTIZED state as
// the baseline, so sender and receiver reconstruct bit-identical states and deltas never drift.
//
// Units: FNV game units. 70 units are about 1 metre, which is how the "10 m snap" and "2 m smooth" thresholds of the
// plan are converted (override through InterpConfig::unitsPerMetre).
#pragma once
#include <cstddef>
#include <cstdint>
#include <deque>
#include <map>
#include <set>
#include <string>
#include <utility>
#include <vector>
#include "fnvcoop/types.h"

namespace fnvcoop::live {

using NetId = uint32_t;
using OwnerEpoch = uint32_t;
using CellId = uint32_t;
using Bytes = std::vector<uint8_t>;

struct Vec3 {
    float x = 0, y = 0, z = 0;
    friend bool operator==(const Vec3& a, const Vec3& b) { return a.x == b.x && a.y == b.y && a.z == b.z; }
};
float distance(const Vec3& a, const Vec3& b);

// ---- channel mapping -------------------------------------------------------------------------------------------
// ENet channel 0 = reliable ordered (events, ownership, spawn, death, hits), channel 1 = unreliable sequenced
// (snapshots and their acks). Matches fnvcoop::ITransport::Channel (Reliable = 0, Unreliable = 1).
enum class Channel : uint8_t { Reliable = 0, Unreliable = 1 };
enum class MsgKind : uint8_t {
    SnapshotBatch = 1,   // actor snapshots, delta encoded
    SnapshotAck = 2,     // receiver -> sender: highest seq applied per actor, plus "send me a full one" requests
    Ownership = 3,       // OwnMsg traffic (Grant / Transfer / Ack / Commit / Release)
    Event = 4,           // spawn, death, hit, drop, container: anything that must not be lost or reordered
};
Channel channelFor(MsgKind k);
constexpr uint8_t kEnetChannelReliable = 0;
constexpr uint8_t kEnetChannelUnreliable = 1;
inline uint8_t enetChannel(Channel c) { return c == Channel::Reliable ? kEnetChannelReliable : kEnetChannelUnreliable; }
const char* toString(MsgKind k);

// ---- snapshot --------------------------------------------------------------------------------------------------
// Schema version of the snapshot datagram. v1: locomotion + 5 flag bits. v2 (enemy-anim round): adds the "animation extension"
// (one-shot action group + a counter that bumps per one-shot + dead / knocked / swim / talk / block / ragdoll bits), carried only
// when something in it is non-zero. A v2 sender whose extension is all zero emits byte-identical v1 datagrams; a v1 receiver
// rejects any datagram that has the extension bit set (as Malformed), so both ends of a session must run the same schema.
// Wire compatibility rules for future changes: new fields go AFTER the existing ones behind a flag bit that old receivers reject,
// the version below is bumped, and the actor module's kActorWireVersion (MSnapFull) is bumped with it.
constexpr uint8_t kLiveSchemaVersion = 2;

// Locomotion codes are free-form (the animation agents own the meaning); 0 is idle.
// The extension fields below are written by plugin/actors/anim (NPCs) and may be used by the avatar layer too.
struct AnimState {
    uint8_t locomotion = 0;
    bool weaponDrawn = false, sneak = false, jump = false, reload = false, aim = false;
    // ---- schema v2 extension -----------------------------------------------------------------------------------------------
    // Base animation group id (TESAnimGroup, 1..244) of the one-shot the owner is playing right now (an attack swing / shot, a
    // stagger or hit reaction, block, dodge, ...); 0 = none. The owner holds it for at least a few snapshots so a lost datagram or a
    // slow frame cannot skip it.
    uint8_t action = 0;
    // Wraps at 256. Bumps every time the owner STARTS a one-shot, so two attacks in a row (same group) are two events.
    uint8_t actionSeq = 0;
    bool dead = false;      // the actor is dead (death declaration is a separate reliable event; this is the pose hint)
    bool knocked = false;   // knocked down / ragdolling on the floor but alive
    bool swim = false;
    bool talk = false;
    bool block = false;
    bool ragdoll = false;   // the physics ragdoll controller is active (visual only)
    friend bool operator==(const AnimState& a, const AnimState& b) {
        return a.locomotion == b.locomotion && a.weaponDrawn == b.weaponDrawn && a.sneak == b.sneak &&
               a.jump == b.jump && a.reload == b.reload && a.aim == b.aim && a.action == b.action && a.actionSeq == b.actionSeq &&
               a.dead == b.dead && a.knocked == b.knocked && a.swim == b.swim && a.talk == b.talk && a.block == b.block &&
               a.ragdoll == b.ragdoll;
    }
};
// Wire packing, shared by the datagram codec and the actor module's reliable full-state message.
constexpr uint8_t kAnimFlagExt = 0x80;        // flags byte: the extension (action, actionSeq, extBits) follows
constexpr uint8_t kAnimFlagsReserved = 0x60;  // must be zero
uint8_t packAnimFlags(const AnimState& a);    // the five v1 bits (never sets kAnimFlagExt)
uint8_t packAnimExt(const AnimState& a);      // dead/knocked/swim/talk/block/ragdoll bits
bool animHasExt(const AnimState& a);          // anything in the extension is non-zero
AnimState unpackAnim(uint8_t loco, uint8_t flags, uint8_t action, uint8_t actionSeq, uint8_t extBits);
// Serial compare for the 8 bit counter: true when `b` is newer than `a` (wrap safe, half range).
inline bool actionSeqNewer(uint8_t a, uint8_t b) { return a != b && static_cast<uint8_t>(b - a) < 128; }
struct AiState {
    NetId combatTarget = 0;   // 0 = none
    uint32_t package = 0;     // AI package form id (plugin-local, mapped by the caller)
    uint8_t alert = 0;        // 0 calm, 1 caution, 2 danger, 3 combat (caller's meaning)
    friend bool operator==(const AiState& a, const AiState& b) {
        return a.combatTarget == b.combatTarget && a.package == b.package && a.alert == b.alert;
    }
};
// One actor value (health, action points...). Absolute value, NOT an increment: an unreliable stream that lost a
// packet must not drift. "avDeltas" means "the values that changed since the baseline".
struct AvValue {
    uint8_t av = 0;
    int32_t value = 0;
    friend bool operator==(const AvValue& a, const AvValue& b) { return a.av == b.av && a.value == b.value; }
};

struct ActorSnapshot {
    NetId netId = 0;
    OwnerEpoch ownerEpoch = 0;
    uint32_t seq = 0;      // per (actor, epoch), starts at 1, rises by >= 1 per snapshot (SeqAllocator)
    uint32_t tick = 0;     // sender clock in ms (wraps after 49 days; compared with serial arithmetic)
    CellId cell = 0;
    Vec3 pos, rot, vel;    // rot = (pitch, yaw, roll) radians
    AnimState anim;
    AiState ai;
    std::vector<AvValue> avDeltas;   // encode input: values to publish. Decode output: the values that changed
};

// Quantized form (what travels, and what baselines are made of).
struct QState {
    CellId cell = 0;
    int32_t pos[3] = {0, 0, 0};
    uint16_t rot[3] = {0, 0, 0};
    int32_t vel[3] = {0, 0, 0};
    uint8_t loco = 0, animFlags = 0;
    uint8_t action = 0, actionSeq = 0, animExt = 0;   // schema v2 extension (all zero = absent on the wire)
    NetId target = 0;
    uint32_t package = 0;
    uint8_t alert = 0;
    std::map<uint8_t, int32_t> av;   // merged absolute values
    friend bool operator==(const QState& a, const QState& b);
};
constexpr float kPosStep = 0.125f;
constexpr float kVelStep = 0.25f;
QState quantize(const ActorSnapshot& s);                 // av = s.avDeltas as absolute values
ActorSnapshot dequantize(const QState& q, NetId id, OwnerEpoch epoch, uint32_t seq, uint32_t tick);
// Snapshot after a quantize/dequantize round trip (what a receiver will see). avDeltas are sorted by av.
ActorSnapshot normalized(const ActorSnapshot& s);

// Allocates per-actor sequence numbers; restarts at 1 when the epoch changes.
class SeqAllocator {
public:
    uint32_t next(NetId id, OwnerEpoch epoch);
private:
    std::map<NetId, std::pair<OwnerEpoch, uint32_t>> st_;
};

// Serial-number arithmetic (RFC 1982 style) for seq and tick.
inline bool serialLess(uint32_t a, uint32_t b) { return a != b && static_cast<uint32_t>(b - a) < 0x80000000u; }
inline bool serialLE(uint32_t a, uint32_t b) { return a == b || serialLess(a, b); }

// ---- test-only fault switches (mutation twins, switchable at run time) ------------------------------------------
struct LiveFaults {
    bool deltaFromLatest = false;     // sender deltas against the newest SENT snapshot, not the newest ACKED one
    bool acceptStaleSeq = false;      // receiver applies snapshots with seq <= last
    bool acceptStaleEpoch = false;    // receiver applies snapshots from an older owner epoch
    bool acceptNonOwner = false;      // receiver does not check the sender against the recorded owner
    bool noExtrapolationCap = false;  // InterpBuffer extrapolates forever
    bool extrapolateOnly = false;     // InterpBuffer never interpolates, always dead-reckons from the newest snapshot
    bool noSnap = false;              // InterpBuffer never snaps (teleports are smoothed across the map)
    bool noSmooth = false;            // InterpBuffer never smooths (pops by 2 to 10 m are applied at once)
    bool snapIgnoresCell = false;     // a cell change is interpolated instead of snapped
    bool nearRateAsFar = false;       // near actors are scheduled at the far rate
    bool noFarTier = false;           // far actors are never sent
    bool ackIgnored = false;          // sender never advances its baseline
    bool dropAvChanges = false;       // delta encoder forgets AV changes
    bool noResyncRequest = false;     // receiver never asks for a full snapshot when the baseline is missing
    bool dropAnimExt = false;         // encoder forgets the animation extension (one-shots, dead, knocked ...)
    bool animExtNoDelta = false;      // delta encoder does not notice an extension-only change (a lone actionSeq bump is lost)
    bool acceptBadAnimExt = false;    // decoder accepts reserved bits / an all-zero extension (non-canonical forms)
    // InterpBuffer low-latency mode (the player avatar stream, InterpConfig::lowLatency)
    bool llFixedDelay = false;        // the render delay ignores the measured jitter (stays at the ceiling): the old, laggy buffer
    bool llNoDamping = false;         // dead reckoning is undamped (v * t): a stop overshoots by the whole extrapolated stretch
    bool llNoBlend = false;           // a snapshot that disagrees with the prediction pops the puppet instead of being faded out
    bool llNoSlew = false;            // the render clock follows the offset filter / delay changes at once (lumpy steps)
    bool llOldInterval = false;       // low-latency mode: the stream's interval is the EMA of every gap, silence included (the idle heartbeat drives the delay to its slow-stream ceiling)
    bool noTeleportSnap = false;      // InterpBuffer: a target that jumped by more than teleportUnits in one step is eased in (the catch-up) instead of snapped
};
inline LiveFaults& liveFaults() { static LiveFaults f; return f; }
struct LiveFaultScope {
    LiveFaults saved;
    explicit LiveFaultScope(const LiveFaults& f) : saved(liveFaults()) { liveFaults() = f; }
    ~LiveFaultScope() { liveFaults() = saved; }
};

// ---- sender -----------------------------------------------------------------------------------------------------
struct SenderConfig {
    size_t maxPacketBytes = 1200;     // one datagram; entries are packed until the next would not fit
    uint32_t maxUnacked = 48;         // inflight snapshots kept per (peer, actor); beyond it a full snapshot is sent
    uint32_t idleHeartbeatMs = 1000;  // an unchanged actor is re-sent at least this often (0 = never skip)
};
struct SenderStats {
    uint64_t queued = 0, skippedIdle = 0, fullSent = 0, deltaSent = 0, packets = 0, bytes = 0, acked = 0, resyncs = 0;
};

class SnapshotSender {
public:
    explicit SnapshotSender(SenderConfig cfg = {}) : cfg_(cfg) {}
    // Queue one snapshot for `peer`. Returns false when skipped (identical to the acked baseline and the heartbeat is
    // not due, or the seq is not newer than the last queued one for this actor/epoch).
    bool queue(PlayerId peer, const ActorSnapshot& s, uint32_t nowMs);
    // Packs everything queued for `peer` into datagrams (MsgKind::SnapshotBatch, Channel::Unreliable).
    std::vector<Bytes> flush(PlayerId peer, uint32_t nowMs);
    // Feed a SnapshotAck datagram from `peer`. False if malformed.
    bool onAckPacket(PlayerId peer, const uint8_t* data, size_t size);
    void forgetPeer(PlayerId peer);
    void forget(PlayerId peer, NetId id);
    const SenderStats& stats() const { return stats_; }
    // Seq of the acked baseline for (peer, actor); 0 = none (next snapshot is full).
    uint32_t ackedSeq(PlayerId peer, NetId id) const;
private:
    struct Entry { uint32_t seq = 0; QState q; };
    struct Track {
        OwnerEpoch epoch = 0;
        bool haveAcked = false;
        Entry acked;
        std::deque<Entry> inflight;
        uint32_t lastQueuedSeq = 0;
        uint32_t lastSentTick = 0;
        bool everSent = false;
        bool forceFull = false;
        std::map<uint8_t, int32_t> avAcc;   // latest absolute AV values published so far
    };
    struct Pending { NetId id; OwnerEpoch epoch; uint32_t seq, tick, baseSeq; Bytes body; };
    struct Peer { std::map<NetId, Track> tracks; std::vector<Pending> pending; };
    SenderConfig cfg_;
    std::map<PlayerId, Peer> peers_;
    SenderStats stats_;
};

// ---- receiver ---------------------------------------------------------------------------------------------------
struct ReceiverConfig {
    bool checkSenderIsOwner = true;   // host: true. Clients receive host-relayed packets, so false there
    uint32_t historyDepth = 64;       // received snapshots kept per actor to resolve delta baselines
    uint32_t maxActors = 16384;       // tracked actors; a hostile sender cannot grow the table past this
};
enum class Verdict : uint8_t { Accepted, StaleEpoch, StaleSeq, WrongOwner, NeedBaseline, Malformed };
const char* toString(Verdict v);
struct ReceivedSnapshot {
    Verdict verdict = Verdict::Malformed;
    ActorSnapshot snap;   // valid when Accepted; avDeltas = the AV values this snapshot changed
    std::vector<AvValue> avAll;   // valid when Accepted; every AV value known after applying it (use this to relay)
};
struct ReceiverStats {
    uint64_t accepted = 0, staleEpoch = 0, staleSeq = 0, wrongOwner = 0, needBaseline = 0, malformed = 0, resyncRequests = 0;
};

class SnapshotReceiver {
public:
    explicit SnapshotReceiver(ReceiverConfig cfg = {}) : cfg_(cfg) {}
    // Records the authoritative owner from ownership traffic (Grant / Commit). Snapshots with a lower epoch are
    // rejected afterwards; at the same epoch only `owner` is accepted (when checkSenderIsOwner).
    void setAuthority(NetId id, OwnerEpoch epoch, PlayerId owner);
    void clearAuthority(NetId id);   // Release: nobody owns it; any epoch above the floor may start publishing
    // Decodes one SnapshotBatch datagram from `from`. One result per entry (a malformed datagram yields one Malformed).
    std::vector<ReceivedSnapshot> receive(PlayerId from, const uint8_t* data, size_t size);
    // Builds the pending SnapshotAck datagram for `to` (empty when there is nothing to say).
    Bytes takeAcks(PlayerId to);
    void forget(NetId id);
    const ReceiverStats& stats() const { return stats_; }
    OwnerEpoch epochOf(NetId id) const;
    uint32_t lastSeqOf(NetId id) const;
private:
    struct Track {
        OwnerEpoch epoch = 0;
        uint32_t lastSeq = 0;
        std::map<uint32_t, QState> hist;   // seq -> reconstructed state
    };
    struct Auth { OwnerEpoch epoch = 0; PlayerId owner = kNoPlayer; };
    struct AckState { std::map<NetId, std::pair<OwnerEpoch, uint32_t>> highest; std::set<NetId> needFull; };
    ReceiverConfig cfg_;
    std::map<NetId, Track> tracks_;
    std::map<NetId, Auth> auth_;
    std::map<PlayerId, AckState> acks_;
    ReceiverStats stats_;
};

// ---- interpolation buffer ---------------------------------------------------------------------------------------
struct InterpConfig {
    uint32_t delayMs = 100;             // render this far behind the newest snapshot clock
    uint32_t maxExtrapolateMs = 250;    // dead-reckon at most this long past the newest snapshot
    float unitsPerMetre = 70.0f;
    float snapMetres = 10.0f;           // error above this (or a different cell) snaps
    float smoothMetres = 2.0f;          // error above this is eased out
    float smoothEndMetres = 0.25f;      // once easing started it continues until the error is below this (no 2 m pop at the end)
    uint32_t smoothMs = 200;            // time constant of the easing
    size_t maxSnapshots = 64;
    uint32_t offsetWindow = 16;         // samples of (arrival - tick) used for the clock offset (min filter)
    bool adaptiveDelay = true;          // slow streams (far tier, 5 Hz) need a deeper buffer: delay >= 1.25 x interval
    uint32_t maxAdaptiveDelayMs = 350;
    // ---- low-latency mode (the player avatar stream; false = everything above, unchanged, which is what NPCs use) ---------------------
    // The render delay is no longer a fixed 100 ms: it follows the measured arrival jitter (delay = minDelayMs + jitterFactor * the
    // upper-quantile lateness of recent snapshots over the fastest one, at most maxLowLatencyDelayMs), and the buffer renders AHEAD of the
    // newest snapshot when it must (dead reckoning on the snapshot's velocity, damped so a stop cannot run away). A snapshot that
    // disagrees with the prediction is not applied as a pop: the difference is faded out over blendMs. The render clock itself (offset
    // + delay) moves at most slewPerMille/1000 ms per ms, so a changing jitter estimate never makes a visible step. A stream slower than
    // slowStreamMs (the far tier) keeps the old rule (delay >= 1.25 x interval): it is interpolated, not predicted.
    bool lowLatency = false;
    uint32_t minDelayMs = 20;           // floor of the render delay
    float jitterFactor = 1.0f;          // delay = minDelayMs + jitterFactor * jitter
    uint32_t maxLowLatencyDelayMs = 120;// ceiling (a link worse than this is carried by the extrapolation and the blend)
    uint32_t extrapolateTauMs = 150;    // damping time constant: displacement = v * tau * (1 - exp(-t / tau)), never more than v * tau
    uint32_t blendMs = 70;              // time constant of the fade of a prediction miss (0 = no blending)
    uint32_t slowStreamMs = 150;        // snapshot interval above this: the old interpolation rule applies
    uint32_t intervalCapMs = 400;       // a gap between snapshots longer than this is silence (idle heartbeat, outage), not the stream's rate: it does not enter the interval estimate
    uint32_t slewPerMille = 100;        // the render clock may drift by 10 percent at most
    // ---- teleports (both modes) -------------------------------------------------------------------------------------------------------
    // A step of the TARGET (or between two neighbouring snapshots) longer than teleportUnits + teleportSpeed * dt cannot be walking: it is a teleport and the
    // puppet snaps there. It is never eased in (the old rule eased every jump between 2 and 10 m over 200 ms time constants, so a 500 unit teleport took
    // 0.6 s to finish) and never interpolated across.
    bool teleportRules = true;          // false: the rules of master before the round of 2026-10-28 (a 10 m snap only; ini [avatar] legacyRules=1, for before / after runs)
    bool legacyInterval = false;        // true: the stream's interval is the EMA of every gap, silence included (same switch)
    float teleportUnits = 350.0f;       // 5 m
    float teleportSpeed = 800.0f;       // units/s a legitimate mover may add on top (a sprint is about 500; the sampler's own cap is 2200)
    float silentJumpUnits = 140.0f;     // a standing player who is this far (2 m) from where the previous snapshot stood, after a silence or as a standing one again, jumped
};
enum class InterpState : uint8_t {
    Empty,          // nothing pushed yet
    Interpolating,  // between two snapshots
    Extrapolating,  // past the newest snapshot, within the cap
    Holding,        // past the cap (or before the first snapshot): position frozen
    Smoothing,      // easing out a 2..10 m error
    Snapped,        // jumped (error above 10 m, or a different cell)
};
const char* toString(InterpState s);
struct InterpOut {
    InterpState state = InterpState::Empty;
    CellId cell = 0;
    Vec3 pos, rot, vel;
    AnimState anim;
    AiState ai;
    bool snapped = false;   // true on the sample where the puppet jumped
    float errorMetres = 0;  // distance between where the puppet was and where the buffer says it should be
};
// Where the engine says the puppet actually is right now (physics may have pushed it). Optional input to sample().
struct ActualPose { CellId cell = 0; Vec3 pos; };

// A running timing figure: the last value, the sum and count since the start (the difference of two readings gives the mean over the stretch between them),
// and the largest value of the last 256 samples. Pure data, no clock.
struct Gauge {
    double last = 0, sum = 0, max = 0;
    uint64_t n = 0;
    double prevMax = 0;
    void add(double v) {
        last = v; sum += v; ++n;
        if (v > max) max = v;
        if ((n & 255u) == 0) { prevMax = max; max = v; }
    }
    double mean() const { return n ? sum / static_cast<double>(n) : 0.0; }
    double recentMax() const { return max > prevMax ? max : prevMax; }
};
std::string gaugeJson(const Gauge& g);   // {"n":N,"last":x,"mean":x,"max":x,"sum":x}
// What an InterpBuffer is doing, for avatar.status (updated by sample() and push()).
struct InterpDiag {
    bool lowLatency = false;
    uint64_t pushes = 0, samples = 0, extrapolating = 0, holding = 0, snaps = 0, teleportSnaps = 0, eased = 0;
    int32_t offsetMs = 0;          // clock offset in use: the smallest (arrival - tick) of the window
    double jitterMs = 0;           // upper-quantile lateness of arrivals over the fastest one
    double frameMs = 0;            // the caller's smoothed frame length
    double wantDelayMs = 0;        // render delay the rule asks for (low-latency: min + jitter above one frame; legacy: the fixed / adaptive delay)
    double lagErrMs = 0;           // render clock minus its target, still slewing
    int32_t renderAgeMs = 0;       // nowMs - render time: how far behind the sender's clock (as mapped by the offset) the shown pose is = offset + delay in use
    int32_t newestAgeMs = 0;       // render time - newest snapshot tick: > 0 dead reckoning, < 0 buffered ahead
    Gauge delayInUse;              // renderAge - offset: the buffer delay actually applied (ms)
    Gauge hold;                    // arrival stamp -> the first sample() that could use the snapshot (ms): receive and dispatch against the frame order
    Gauge extrap;                  // render time - newest tick while it is positive (ms)
    Gauge arrivalGap;              // time between arrival stamps of consecutive snapshots (ms)
};

class InterpBuffer {
public:
    explicit InterpBuffer(InterpConfig cfg = {}) : cfg_(cfg) {}
    // `arrivalMs` is the local clock when the snapshot was received. Stale (tick not newer than the newest) snapshots
    // are ignored; returns whether it was stored.
    bool push(const ActorSnapshot& s, uint32_t arrivalMs);
    // Pose to display at local time `nowMs`. Call once per frame; the result depends on the call history (smoothing).
    InterpOut sample(uint32_t nowMs, const ActualPose* actual = nullptr);
    void reset();
    size_t size() const { return snaps_.size(); }
    const InterpConfig& config() const { return cfg_; }
    const InterpDiag& diag() const { return diag_; }
private:
    struct Snap { uint32_t tick; CellId cell; Vec3 pos, rot, vel; AnimState anim; AiState ai; bool jump = false; };   // jump: this snapshot is a teleport away from its predecessor
    bool isJump(const Snap& a, const Snap& b) const;
    struct Target {
        InterpState state = InterpState::Interpolating;
        CellId cell = 0;
        Vec3 pos, rot, vel;
        AnimState anim;
        AiState ai;
    };
    // Where the pose should be at sender time `rt`, from the first `n` stored snapshots.
    Target evalTarget(uint32_t rt, size_t n) const;
    InterpConfig cfg_;
    std::deque<Snap> snaps_;
    std::deque<int64_t> offsets_;   // arrival - tick samples
    bool haveShown_ = false;
    bool smoothing_ = false;
    bool haveTarget_ = false;
    Vec3 corr_, tposPrev_;          // outstanding easing offset (target space) and last frame's target
    CellId shownCell_ = 0, tcellPrev_ = 0;
    Vec3 shown_;
    uint32_t lastSampleMs_ = 0;
    uint32_t prevRt_ = 0;           // render time of the previous sample() (low-latency mode)
    double intervalMs_ = 0;         // smoothed time between snapshots
    // low-latency mode
    double jitterMs_ = 0;           // upper-quantile lateness of arrivals over the fastest one
    double frameMs_ = 0;            // smoothed time between sample() calls (the caller's frame length)
    double lagErr_ = 0;             // render clock minus its target (offset + delay), ms; shrinks at the slew rate
    uint32_t lagTPrev_ = 0;         // last target lag (offset + delay)
    bool haveLag_ = false;
    size_t newSince_ = 0;           // snapshots stored since the last sample()
    Vec3 soft_;                     // outstanding fade-out of a prediction miss (target space)
    InterpDiag diag_;
    std::deque<double> gaps_;       // the last four gaps between snapshots that were not silence (low-latency mode)
    size_t diagNew_ = 0;            // snapshots stored since the last sample() (diagnostics only)
    double lastWant_ = 0;
    uint32_t firstNewArrival_ = 0;  // arrival stamp of the oldest snapshot stored since the last sample()
    uint32_t lastArrival_ = 0;
    bool haveArrival_ = false;
};

// ---- interest management ----------------------------------------------------------------------------------------
struct InterestConfig {
    uint32_t nearPeriodMs = 50;     // 20 Hz
    uint32_t farPeriodMs = 200;     // 5 Hz
    float nearRadius = 2800.0f;     // units (about 40 m) in the observer's own cell
    float hysteresis = 1.25f;       // a near actor drops to far only beyond nearRadius * hysteresis
};
enum class Tier : uint8_t { None = 0, Far = 1, Near = 2 };
const char* toString(Tier t);
struct Observer {
    PlayerId player = kNoPlayer;
    CellId cell = 0;
    Vec3 pos;
    std::vector<CellId> adjacent;   // loaded neighbouring cells (far tier)
};
struct InterestActor {
    NetId id = 0;
    CellId cell = 0;
    Vec3 pos;
    PlayerId owner = kNoPlayer;     // never sent back to its owner
};

class InterestManager {
public:
    explicit InterestManager(InterestConfig cfg = {}) : cfg_(cfg) {}
    void setObserver(const Observer& o);
    void removeObserver(PlayerId p);
    void setActor(const InterestActor& a);
    void removeActor(NetId id);
    // Which tier would `player` get for this actor right now (with hysteresis against the previous answer)?
    Tier tierOf(PlayerId player, NetId id);
    // Actors due for `player` at `nowMs` (rate limited per actor by tier). Marks them as sent.
    std::vector<NetId> collect(PlayerId player, uint32_t nowMs);
    uint32_t periodMs(Tier t) const;
    const InterestConfig& config() const { return cfg_; }
private:
    struct Link { Tier tier = Tier::None; uint32_t lastSent = 0; bool sent = false; };
    InterestConfig cfg_;
    std::map<PlayerId, Observer> obs_;
    std::map<NetId, InterestActor> actors_;
    std::map<std::pair<PlayerId, NetId>, Link> links_;
};

}  // namespace fnvcoop::live
