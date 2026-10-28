// HostSession / ClientSession: reliable, ordered, idempotent txn delivery over an unreliable ITransport.
// Time is passed in (tick(nowMs)); there are no threads and no clocks inside, so the simulator is deterministic.
#pragma once
#include <deque>
#include <functional>
#include <map>
#include <memory>
#include <set>
#include "fnvcoop/capture.h"
#include "fnvcoop/policy.h"
#include "fnvcoop/protocol.h"
#include "fnvcoop/storylog.h"
#include "fnvcoop/worker.h"

namespace fnvcoop {

// Implemented by the engine side. Called with post-merge effects to perform on the LOCAL player/world.
// The session holds an EchoGuard on the capture buffer for the duration of each call.
struct BaselineResult {
    uint64_t effectsApplied = 0, reverted = 0, unrevertable = 0, chunks = 0, entriesKept = 0;
};

class IEffectSink {
public:
    struct Context {
        uint64_t seq = 0;       // 0 for snapshot-diff effects
        PlayerId origin = 0;
        size_t index = 0;       // position within this txn's applied effects
        const Cause* cause = nullptr;
        bool fromSnapshot = false;
        uint32_t epoch = 0;     // host world epoch the txn belongs to (seq restarts at 1 in every epoch)
        bool repair = false;    // desync self-heal: force-set of a drifted key (implies baseline)
        bool baseline = false;  // part of the world-baseline transfer (force-set, gated scripts)
        bool catchUp = false;   // history replayed during join (events like ForcedDialogue should not re-fire)
    };
    virtual ~IEffectSink() = default;
    virtual void apply(const Effect& e, const Context& ctx) = 0;
    virtual void onJoined(PlayerId /*slot*/, const SharePolicy& /*policy*/) {}
    virtual void onBaselineComplete(const BaselineResult& /*r*/) {}
};

struct HostStats { uint64_t commits = 0, droppedByPolicy = 0, retransmitBatches = 0, welcomes = 0, snapshots = 0,
                   replays = 0, baselineJoins = 0, baselineChunksSent = 0, baselineEffectsSent = 0, baselineReverts = 0,
                   baselineRetransmits = 0, welcomesThrottled = 0, bulkDeferred = 0, baselineEntriesKept = 0, handshakeRejects = 0, digestsReceived = 0, digestsOk = 0, digestsMismatch = 0, digestsRateLimited = 0, digestsStale = 0,
                   desyncRepairs = 0, desyncEffectsRepaired = 0, repairsDeferredBudget = 0, reloads = 0, rebasesSent = 0, staleEpochAcks = 0, intentsCommitted = 0, actionsExecuted = 0, intentsDuplicate = 0, rejected = 0, badMessages = 0, followerCapDrops = 0, companionVetDrops = 0, controlReleases = 0,
                   questRunningAdopted = 0,   // client-only quest running keys settled by committing the host's own running state (issue #25)
                   stageZeroAdopted = 0,      // client-only "SetStage q = 0" keys settled by committing the same stage 0 on the host (its key space is then complete)
                   questStateAdopted = 0,     // client-only non-zero SetStage / CompleteQuest / FailQuest keys the host engine confirmed and committed
                   questStateDisagreed = 0,   // client-only quest state keys the host engine does NOT agree with (a real divergence: not adopted, reported every round)
                   objectiveZeroAdopted = 0,  // client-only "SetObjective flags 0" keys settled by committing the same flags 0 on the host
                   objectiveStateAdopted = 0,   // client-only SetObjective (non-zero flags) / CompleteAllObjectives keys the host engine confirmed and committed
                   objectiveStateDisagreed = 0, // client-only objective keys the host engine does NOT agree with (a real divergence: not adopted, reported every round)
                   scriptVarAdopted = 0,        // client-only SetScriptVar keys settled by committing the client's value on the host (0 outright, non-zero after the host engine confirmed it)
                   scriptVarDisagreed = 0,      // client-only SetScriptVar keys whose value the host engine does NOT share (a real divergence: not adopted, not resent, reported)
                   revertsSkippedNoInverse = 0, // client-only keys of a kind the client can never revert: counted, no longer sent as revert keys every round
                   effectsSettled = 0,          // effects committed by the host to settle client-only keys of the loaded save (every *Adopted above): host bookkeeping once per key, NOT drift repair, so not in desyncEffectsRepaired
                   settleRounds = 0,            // digest rounds that settled at least one key
                   settleFollowUps = 0,         // immediate digest requests after a settle round that left bad shards unasked
                   globalAdopted = 0,           // client-only SetGlobal keys the host engine confirmed and committed
                   globalDisagreed = 0,         // client-only SetGlobal keys whose value the host engine does NOT share
                   runningUnread = 0,           // client-only quest running keys the host engine could not read (left alone, acknowledged after a few rounds)
                   divergenceAcked = 0,         // keys acknowledged as a known divergence the host can neither adopt nor repair (reported once, no longer a digest mismatch)
                   ackDropped = 0,              // acknowledgements withdrawn because either side's value for the key changed
                   repairGaveUp = 0,            // keys repaired kMaxRepairAttempts times with the same two values and still different: acknowledged instead of resent forever
                   resumesRefused = 0; };     // Hellos whose resume cursor this host never handed out in this epoch (a save of another timeline): rebased

// Empty string = compatible; otherwise a player-facing explanation and the reject reason code.
std::string checkHandshake(const HandshakeInfo& host, const HandshakeInfo& client, uint8_t& reason);

class HostSession {
public:
    struct Options {
        uint32_t rtoMs = 200; size_t batch = 16;
        uint32_t bulkBytesPerSec = 1500000;   // per-peer cap on bulk traffic (Welcome snapshots, baseline chunks); excess waits
        uint32_t maxBacklogBytes = 1u << 20;  // hold bulk sends while this much is already queued towards the peer
        size_t autoCompactKeep = 2048;        // keep this many txns for replay; older ones fold into the base state (0 = never compact)
        // Follower caps (owner decision): per recruiting player and for the whole party. Recruits beyond a cap are dropped.
        struct FollowerCaps { uint8_t humanoidPerOwner = 1, critterPerOwner = 1, partyTotal = 4; } followers;
        uint32_t welcomeResendMs = 1000;      // a repeated Hello of the same session gets a Welcome at most this often (doubling to 16x)
    };
    HostSession(ITransport& t, SharePolicy policy, Options opt);
    explicit HostSession(ITransport& t, SharePolicy policy = SharePolicy()) : HostSession(t, policy, Options()) {}

    // PlaceAtMe / PlaceLeveledActorAtMe: the host allocates the stable 0xFF-prefixed ref id for a created object; commit a SpawnRef with it.
    // Ids are never reused (the counter survives save/load through the shadow state).
    uint32_t allocCreatedId();
    FormRef allocCreatedRef() { return createdRef(allocCreatedId()); }

    // Commit a script's captured effects as one txn. Normalizes per policy, appends to the log, broadcasts.
    // Returns the seq, or 0 if nothing remained to commit.
    uint64_t commit(PlayerId origin, const Cause& cause, std::vector<Effect> effects, uint8_t flags = 0,
                    uint32_t originEpoch = 0, uint32_t originIntent = 0);

    void tick(uint64_t nowMs);  // polls the transport, handles messages, retransmits

    const StoryLog& log() const { return log_; }
    StoryLog& mutableLog() { return log_; }
    const ShadowState& state() const { return log_.headState(); }
    const SharePolicy& policy() const { return policy_; }
    const HostStats& stats() const { return stats_; }

    // Join-in-progress support. Call ONCE, before any commit or client join, with the live engine's shared state at
    // session start (ShadowState; ledger[0].karma/rep = the host's ABSOLUTE values). The host then maintains
    // world() = baseline + every committed txn, and sends joiners that ask (Hello.wantBaseline) the difference
    // between that world and their own.
    void setWorldBaseline(const ShadowState& baseline, uint64_t contentHash);
    bool hasWorldBaseline() const { return haveBaseline_; }
    // Diagnostics for join diffs: how `probe` looks in the frozen snapshot H of `slot`'s current baseline job (or the live head if none).
    std::string explainJoinEntry(PlayerId slot, const Effect& probe) const;

    // ---- runtime desync detection + targeted self-heal ----
    // Clients periodically hash their REAL world (BaselineProvider dump) in 128 shards and send it with their applied seq. The host
    // compares with its own state AS OF THAT SEQ; on a mismatch it asks for the mismatching shards' entries, diffs them and force-sets
    // exactly the drifted keys (keys touched by newer txns are skipped, volatile keys never appear). Every mismatch is logged.
    struct DesyncOptions {
        uint32_t minDigestIntervalMs = 2000;       // per-client rate limit (digests arriving faster are ignored)
        uint32_t maxRepairEffectsPerMin = 1200;    // repair bandwidth budget (token bucket)
        size_t maxShardsPerRound = 32;             // shards inspected per mismatching digest (also limited by the reply cap kMaxShardEntries)
        size_t maxRepairEffects = 256;             // per repair message
        uint8_t compareClasses = 0xFF;             // entry classes 0..7 that are compared (the dumper's coverage)
        size_t reportLimit = 256;                  // reports kept
    };
    struct DesyncEntry {
        std::string key, host, client;             // key = host description (or client's for client-only entries)
        uint64_t keyHash = 0;
        bool clientOnly = false, skippedRace = false;
        bool acknowledged = false;                 // a known divergence the host could not settle: listed, but neither repaired nor a digest mismatch any more
        std::vector<uint64_t> lastSeqs;            // newest txns touching the key
    };
    struct DesyncReport {
        uint64_t id = 0, atMs = 0, atSeq = 0;
        uint32_t epoch = 0;
        PlayerId slot = 0;
        std::vector<uint32_t> shards;
        std::vector<DesyncEntry> entries;
        size_t totalEntries = 0, effectsSent = 0;
        bool incomplete = false, repaired = false;
    };
    // Heavy pure work (baseline diffs, digest hashing/compares, repair planning) runs on `w` instead of the main thread; results are
    // applied inside tick(). nullptr (default) = inline, which keeps simulations deterministic. The worker must outlive the session
    // or at least be idle when the session dies.
    void setWorker(IWorker* w) { worker_ = w; }
    int asyncOutstanding() const { return mq_->outstanding(); }
    void setDesyncOptions(const DesyncOptions& o) { dopt_ = o; }
    const std::deque<DesyncReport>& desyncs() const { return reports_; }
    std::string desyncsJson() const;                  // for coop.status / dashboards
    void requestDigest(PlayerId slot = kNoPlayer);    // ask clients for an immediate digest (after a barrier / scene)

    // ---- save / load semantics ----
    // World epoch: bumps whenever the host's world is replaced (loadSidecar / reloadWorld). Every TxnBatch/Ack carries it;
    // stale-epoch traffic is dropped, joined clients get a Rebase and must re-baseline (forced baseline).
    // Transport told us the peer is gone (ENet Disconnected event): the client stops counting as present (barriers, tethers) until it
    // sends a new Hello. Its slot and token are kept.
    void peerDisconnected(PeerId peer);
    // Build identity this host requires of joining clients (see HandshakeInfo; unset fields are not checked). Mismatches
    // produce a Reject with a text the UI can show.
    void setHandshake(const HandshakeInfo& h) { hs_ = h; }
    const HandshakeInfo& handshake() const { return hs_; }
    uint32_t epoch() const { return epoch_; }
    uint64_t sessionId() const { return sessionId_; }
    void setSessionId(uint64_t id) { sessionId_ = id; }
    // The sidecar to store next to the host's save game (call from the PreSave hook, write it next to the .fos).
    // Contains the shared world as of now (baseline + committed txns), session id, epoch, policy, per-slot karma/rep bases and
    // the token->slot map. It is small (the shared world), NOT the txn history.
    Bytes saveSidecar() const;
    // The host loaded a save (PostLoad hook): restore the matching sidecar. The world becomes that save's shared world, history
    // restarts at seq 0 in a NEW epoch, connected clients are told to rebase. False (nothing changed) on a corrupt sidecar or
    // one written by a session without a world baseline.
    bool loadSidecar(const Bytes& sidecar);
    // The host loaded a save with no (matching) sidecar, or wants a fresh dump: same effect as loadSidecar but from a live dump.
    bool reloadWorld(const ShadowState& baseline, uint64_t contentHash);
    const ShadowState& world() const { return log_.headState(); }  // baseline + every committed txn
    // Observer fired for every committed txn (after normalization). Used by the simulator's oracle.
    std::function<void(const EffectTxn&)> onCommit;
    // Opaque app channel (dialogue etc.). onApp fires for payloads sent by a joined client (slot = sender).
    // A guest asked the host to run a real activation / trigger / pickup. The host plugin executes it with its capture
    // open and commits the resulting effects (commit(slot, cause, effects)). Called exactly once per request.
    std::function<void(PlayerId slot, IntentKind kind, const FormRef& ref, uint32_t epoch, uint32_t id)> onAction;
    // Optional hook run on every committed txn BEFORE policy normalization (the quest rules engine rewrites recipients here).
    std::function<void(EffectTxn&)> txnFilter;
    // W3a item 6: judges the CompanionOwner effects of one txn against the companion book (synccore/social vetCompanionTxn, installed by plugin/companions): a dismissal of
    // another player's companion that rides with a recruit of the same slot group, and a second member of a one-slot group, are taken out of the txn BEFORE it is committed, so no
    // machine ever applies them. Returns how many effects it removed. Runs first in enforceFollowerCaps.
    std::function<size_t(EffectTxn&)> companionVet;
    // A companion that belongs to the whole party (RulesEngine owner=party, ED-E): it counts against no player's follower cap and never blocks a recruit.
    std::function<bool(const FormRef& npc)> isPartyCompanion;
    // Desync repair, issue #25: a client reported quest running states the host's shared world has no entry for. A quest's running
    // flag has no inverse a client could revert to, so the host settles the key itself: it commits ITS OWN running state for each such
    // quest (StartQuest / StopQuest, a normal host txn every client receives), which also completes the host's key space. This hook
    // reads the host engine: fill `running` for the quests it can read and return true (quests left out are not settled this round).
    // Unset or returning false: the completeness rule's default (a quest the host's dump does not list is not running) is committed.
    std::function<bool(const std::vector<FormRef>& quests, std::map<FormRef, bool>& running)> hostQuestRunning;
    // Desync repair: a client reported real save state (a non-zero SetStage, CompleteQuest, FailQuest) the host's shared world has no entry
    // for, because the host runs without a world baseline and never recorded what both copies loaded from the same save. SetStage merges by
    // max and a completed quest cannot be un-completed, so a revert key is skipped by the client and resent every round. The host settles
    // such a key itself, but ONLY after its own engine confirms the value, so a diverged client can never push state into the host world.
    // This hook reads the host engine: fill `out` for the quests it can read (stage < 0 = unknown) and return true.
    // A key is adopted (committed as a normal host txn of the client's value) when the engine's stage is >= the client's stage, or its
    // completed / failed flag equals the client's. When the engine disagrees the key is NOT adopted: it stays an open desync and keeps
    // being reported. Quests the hook leaves out are not settled this round.
    // Unset (the game-free default) or returning false: the old behaviour, such keys go out as revert keys (nothing is adopted blindly).
    struct HostQuestState { int32_t stage = -1; bool completed = false, failed = false; };
    std::function<bool(const std::vector<FormRef>& quests, std::map<FormRef, HostQuestState>& out)> hostQuestState;
    // Same idea for objectives: a client-only SetObjective with non-zero flags or CompleteAllObjectives. The hook fills `out` with the host
    // engine's status flags (bit0 displayed, bit1 completed) of EVERY objective of each quest it can read, and returns true. A SetObjective is
    // adopted when the engine's flags for that objective equal the client's; CompleteAllObjectives when the quest has objectives and all are
    // completed. A disagreement is a real divergence: not adopted, still reported. Quests the hook leaves out are not settled this round.
    // Unset or returning false: such keys go out as revert keys (the old behaviour). A client-only SetObjective with flags 0 needs no reader
    // (it is adopted unconditionally, like stage 0).
    std::function<bool(const std::vector<FormRef>& quests, std::map<FormRef, std::map<uint32_t, uint8_t>>& out)> hostObjectives;
    // Desync repair: a client reported script vars (SetScriptVar) the host's shared world has no entry for. The baseline-less host primes its
    // engine poll with the present values and drops them, so it only records vars that changed since the session started, while the client's
    // digest lists every var of every quest. SetScriptVar has no inverse (a revert key was skipped by the client and resent every round).
    // A value of 0 is the untouched default: the host commits the same 0 outright (the key space is then complete), unless this hook shows a
    // different engine value. A non-zero value is adopted only when the hook reads the same value on the host's own engine. The hook fills
    // `out` for the (owner, index) pairs it can read and returns true; pairs left out are unknown (zero: adopted as the default, non-zero:
    // left alone this round). A value that differs is a real divergence: not adopted, not resent, reported (scriptVarDisagreed).
    // Unset or returning false: zero is still adopted, non-zero is never adopted blindly (and never resent: nothing can revert it).
    std::function<bool(const std::vector<std::pair<FormRef, uint16_t>>& vars, std::map<std::pair<FormRef, uint16_t>, double>& out)> hostScriptVars;
    // Same for client-only globals (SetGlobal): the baseline-less host only records globals that changed since the session started, the client's
    // dump lists all of them. The hook fills `out` with the host engine's value for each global it can read and returns true. A global is adopted
    // (the client's value committed on the host) when the engine holds the same value (digest quantisation); a different value is a real
    // divergence (globalDisagreed). Unset or returning false: nothing is adopted blindly and nothing is resent (no inverse the client could run).
    std::function<bool(const std::vector<FormRef>& globals, std::map<FormRef, double>& out)> hostGlobals;
    static constexpr size_t kMaxRepairAttempts = 3;   // the same two values repaired this many times and still different: acknowledged, never resent again
    static constexpr size_t kUnreadRounds = 3;        // a key the host engine could not read this many digest rounds in a row is acknowledged
    static constexpr size_t kMaxAcks = 4096;          // per client
    std::function<void(PlayerId slot, uint16_t channel, const Bytes& payload)> onApp;
    void sendApp(PlayerId toSlot, uint16_t channel, const Bytes& payload);  // kNoPlayer = every joined client
    void relayApp(PlayerId fromSlot, uint16_t channel, const Bytes& payload);  // to every joined client except fromSlot

    struct ClientView { uint64_t token; PlayerId slot; PeerId peer; bool joined; uint64_t acked; bool baselineJobOpen = false; };
    std::vector<ClientView> clients() const;
    bool allSynced() const;  // every joined client has acked head
    // Re-attach after the host process reloaded a persisted log (clients keep their tokens -> slots only via Hello).
    void reset(const StoryLog& restored) { log_ = restored; }

private:
    static constexpr PeerId kNoPeer = ~PeerId(0);
    struct Job;
    struct Sync;
    struct Client {
        PlayerId slot = kNoPlayer; PeerId peer = kNoPeer; uint32_t session = 0; bool joined = false;
        uint64_t acked = 0, lastSend = 0, lastRetx = 0, retxAcked = 0; uint32_t retxBackoff = 0; bool sendNow = false; uint32_t intentEpoch = 0, lastIntent = 0;
        std::shared_ptr<Job> job;
        bool rebasePending = false; uint64_t lastRebaseSend = 0;
        std::shared_ptr<Sync> sync; uint64_t lastDigestAt = 0; bool digestRequested = false;
        // Known divergences the host could not settle (the host engine disagrees, cannot read the key, or the client has no inverse), and keys it
        // repaired in vain. Both are keyed by the entry key hash. An acknowledged key is treated as agreeing in the digest compare (the host sums the
        // client's term instead of its own, see ShardAdjust) so it stops being a mismatch and a repair; either side's value changing withdraws it.
        struct Known { size_t shard = 0; bool hostHas = false, clientHas = false; uint32_t hostVal = 0, clientVal = 0; uint64_t hostTerm = 0, clientTerm = 0; };
        struct Attempt { uint32_t hostVal = 0, clientVal = 0; size_t sent = 0; bool hostHas = false, clientHas = false; size_t shard = 0; };
        struct Unread { size_t rounds = 0; size_t shard = 0; };
        std::map<uint64_t, Known> acks;
        std::map<uint64_t, Attempt> attempts;
        std::map<uint64_t, Unread> unread;
        size_t askShards = 0;   // shards asked per round while replies get cut off (0 = DesyncOptions::maxShardsPerRound)
        size_t askCursor = 0;   // the bad shards are asked round-robin from here, so shards that never settle cannot starve the rest
        double bulkTokens = -1; uint64_t bulkAt = 0, lastWelcomeAt = 0; uint32_t welcomeResends = 0;
        uint32_t welcomedEpoch = 0;   // the world epoch this host instance last welcomed this token in (a resume cursor is only ours if it matches)
    };
    bool bulkOk(Client& c);
    size_t enforceFollowerCaps(EffectTxn& t);
    void releaseControlsOf(PlayerId s);
    void sendBulk(Client& c, const Message& m);
    void bumpEpochAndRebase();
    void sendJobWindow(Client& c, bool retransmit);
    void buildJobDiff(Client& c);
    void handle(PeerId from, const Message& m);
    void sendWelcome(Client& c, uint64_t lastSeq, bool fresh);
    void sendSnapshotWelcome(Client& c);
    void sendTo(PeerId to, const Message& m);
    Client* byPeer(PeerId p);

    ITransport& t_;
    SharePolicy policy_;
    Options opt_;
    StoryLog log_;
    std::map<uint64_t, Client> clients_;  // by token
    uint64_t now_ = 0;
    HostStats stats_;
    bool haveBaseline_ = false;
    uint64_t contentHash_ = 0, baselineCounter_ = 0;
    int64_t baseKarma_[kMaxSlots] = {};
    std::vector<RewardGrant> grants_[kMaxSlots];   // every reward committed this epoch, per slot (the ledger split by id); cleared with the ledger
    std::map<FormRef, std::pair<int64_t, int64_t>> baseRep_[kMaxSlots];
    uint32_t epoch_ = 1;
    uint32_t createdNext_ = 0;
    uint64_t sessionId_ = 0;
    HandshakeInfo hs_;
    IWorker* worker_ = nullptr;
    std::shared_ptr<MainQueue> mq_ = std::make_shared<MainQueue>();
    DesyncOptions dopt_;
    std::deque<DesyncReport> reports_;
    uint64_t reportCounter_ = 0;
    double repairTokens_ = -1;
    uint64_t repairRefillAt_ = 0;
};

class ClientSession {
public:
    struct Options { uint32_t helloIntervalMs = 250, rtoMs = 200; size_t window = 4096; uint32_t digestIntervalMs = 10000; /* 0 = off */ };
    ClientSession(ITransport& t, PeerId host, uint64_t token, IEffectSink& sink, EffectCapture* capture, Options opt);
    ClientSession(ITransport& t, PeerId host, uint64_t token, IEffectSink& sink, EffectCapture* capture = nullptr)
        : ClientSession(t, host, token, sink, capture, Options()) {}

    void connect(uint64_t nowMs, uint32_t session);  // session: strictly increasing per token, never 0
    void disconnect();                               // local drop; state is kept for a later rejoin
    void resetState();                               // models a fresh install/restart: forgets everything (including which rewards were delivered)
    void tick(uint64_t nowMs);

    // Report a local action already applied to the local world. Queued durably (survives disconnect, lost on
    // resetState), delivered in order exactly once. Returns the intent id (0 if the client never connected).
    uint32_t submitIntent(const Cause& cause, std::vector<Effect> effects);
    // Ask the host to perform ActivateRef / TriggerEnter / PickUp on `ref` (nothing is applied locally).
    uint32_t submitAction(IntentKind kind, const FormRef& ref);

    // Join a game in progress: give the session the client's OWN world (engine dump, same layout as the host's baseline)
    // before connect(). The Hello then asks for a baseline transfer; the sink receives the differences with
    // Context::baseline set, followed by the live stream. Cleared by resetState().
    void setBaselineWorld(const ShadowState& world, uint64_t contentHash);
    bool baselineActive() const { return baselineActive_; }
    // Diagnostics: how `probe` looks in the world dump this client's summary was built from.
    std::string explainSummaryEntry(const Effect& probe) const;
    bool baselineDone() const { return baselineDone_; }
    const BaselineResult& baselineResult() const { return baselineResult_; }
    uint8_t rejectReason() const { return rejectReason_; }
    const std::string& rejectText() const { return rejectText_; }   // ready to show to the player
    void setHandshake(const HandshakeInfo& h) { hs_ = h; }          // this client's build identity (sent in every Hello)

    // ---- save / load / rebase ----
    // Supplies a fresh dump of the client's CURRENT world (engine dump, same layout as setBaselineWorld). Used for Rebase and
    // for loading a save without a usable sidecar. Return false if the dump is impossible.
    using BaselineProvider = std::function<bool(ShadowState& world, uint64_t& contentHash)>;
    void setBaselineProvider(BaselineProvider p) { provider_ = std::move(p); }
    bool rebaseRequired() const { return rebaseRequired_; }   // rebase was requested but no provider: the app must restart the join itself
    uint32_t worldEpoch() const { return worldEpoch_; }
    // Runtime desync detection (needs a BaselineProvider): the digest interval in ms, 0 = off.
    void setDigestInterval(uint32_t ms) { opt_.digestIntervalMs = ms; }
    // See HostSession::setWorker: digest hashing and the join summary are built off the main thread.
    void setWorker(IWorker* w) { worker_ = w; }
    int asyncOutstanding() const { return mq_->outstanding(); }
    // repairRevertsUnrevertable: SyncRepair revert keys this client had no inverse for (left as they are; before issue #25 this was silent).
    struct SyncStats { uint64_t digestsSent = 0, repairsApplied = 0, effectsRepaired = 0, repairEffectsSkipped = 0, repairRevertsUnrevertable = 0; };
    const SyncStats& syncStats() const { return syncStats_; }
    uint32_t lastSession() const { return session_; }   // next connect() must use a larger one (rebase/load bump it internally)
    bool canSaveState() const { return joined_ && intents_.empty() && !baselineActive_ && pending_.empty(); }
    // Policy flag from the host: may the player load saves now? (false => the engine layer must refuse the load)
    bool clientLoadAllowed() const { return !(everJoined_ && policy_.clientLoad == SharePolicy::ClientLoad::Block); }
    // Client PreSave hook: sidecar to store next to the client's save. It only carries this player's view of the session
    // (shared shadow as of now + slot + epoch); progress is the host's. clean=false (intents in flight / baseline running) => unusable.
    Bytes saveState() const;
    // Client PostLoad hook. Returns false if the load is not allowed (policy Block). Otherwise: with a usable sidecar the session
    // resumes from the saved shadow (missed host txns arrive as replay/snapshot diff, own reward items are not duplicated); with
    // none/unusable/old-epoch it falls back to a full baseline rejoin via the provider. Reconnects under a new session.
    bool onLocalSaveLoaded(uint64_t nowMs, const Bytes* sidecar);

    bool joined() const { return joined_; }
    bool rejected() const { return rejected_; }
    PlayerId slot() const { return slot_; }
    uint64_t applied() const { return shadow_.seq; }
    const ShadowState& state() const { return shadow_; }
    // Opaque app channel (same session/transport as the story txns).
    std::function<void(PlayerId fromSlot, uint16_t channel, const Bytes& payload)> onApp;
    void sendApp(uint16_t channel, const Bytes& payload);
    bool intentsPending() const { return !intents_.empty(); }
    struct Stats { uint64_t rebases = 0, staleEpochDropped = 0, txnsApplied = 0, duplicatesDropped = 0, buffered = 0, snapshots = 0, replays = 0; };
    const Stats& stats() const { return stats_; }

private:
    void handle(const Message& m);
    void deliver(const EffectTxn& txn);
    void applyOne(const EffectTxn& txn);
    void drain();
    void dropCommittedIntents();
    void sendHeadIntent();
    Hello makeHello() const;
    void doRebase();
    bool loadStateBytes(const Bytes& b);
    void resetSession();
    void noteDelivered(const EffectTxn& txn);
    void sendSummaryWindow(bool retransmit);
    void stampSummary();
    void applyReadyDiffChunks();
    void finishBaseline();
    void applyBaselineEffect(const Effect& e);
    void sendDigest();
    void applyRepairEffect(const Effect& e);
    void send(const Message& m);
    void sendAck();

    ITransport& t_;
    PeerId host_;
    uint64_t token_;
    IEffectSink& sink_;
    EffectCapture* capture_;
    Options opt_;

    uint32_t session_ = 0, epoch_ = 0;
    uint64_t joinHead_ = 0;  // host head when we (re)joined; txns <= this are catch-up
    bool active_ = false, joined_ = false, rejected_ = false;
    PlayerId slot_ = kNoPlayer;
    SharePolicy policy_;
    ShadowState shadow_;
    std::map<uint64_t, EffectTxn> pending_;  // out-of-order buffer
    struct PendingIntent { uint32_t id; Cause cause; std::vector<Effect> effects; bool acked = false; IntentKind kind = IntentKind::AppliedEffects; FormRef ref; };
    std::vector<PendingIntent> intents_;
    uint32_t nextIntent_ = 1, seenIntent_ = 0;  // seenIntent_: highest own intent known committed
    uint64_t now_ = 0, lastHello_ = 0, lastIntentSend_ = 0;
    uint32_t helloRetries_ = 0, summarySent_ = 0, summaryBackoff_ = 0;
    uint64_t helloCursor_ = 0;   // Hello::cursorCheck of this connect (hash of the shadow we resume from)
    Stats stats_;
    // baseline join state
    bool wantBaseline_ = false, baselineActive_ = false, baselineDone_ = false;
    uint8_t rejectReason_ = 0, baselineAbs_ = 0;
    std::string rejectText_;
    HandshakeInfo hs_;
    uint32_t baselineId_ = 0;
    uint64_t contentHash_ = 0, lastSummarySend_ = 0;
    std::shared_ptr<const ShadowState> clientWorld_;
    std::vector<BaselineSummaryChunk> summaryChunks_;
    bool summaryReady_ = false;
    uint64_t sumGen_ = 0, connGen_ = 0;
    IWorker* worker_ = nullptr;
    std::shared_ptr<MainQueue> mq_ = std::make_shared<MainQueue>();
    bool digestInFlight_ = false;
    uint32_t summaryNext_ = 0;
    std::map<uint32_t, BaselineDiffChunk> diffGot_;
    uint32_t diffNext_ = 0, diffTotal_ = 0;
    std::vector<Effect> diffEffects_;       // baseline diff effects / reverts, applied together once the last chunk arrived (reverts first)
    std::vector<uint64_t> diffReverts_;
    std::vector<Effect> heldDiff_;
    std::vector<uint64_t> heldRewardIds_;   // ids of the rewards in heldDiff_: delivered once finishBaseline applies them
    std::set<uint64_t> delivered_;          // ids of the rewards this player's world already holds (persisted in the client sidecar)
    BaselineResult baselineResult_;
    uint32_t worldEpoch_ = 0;
    bool rebaseRequired_ = false, everJoined_ = false;
    BaselineProvider provider_;
    // desync digest state
    bool digestWanted_ = false, digestOutstanding_ = false;
    uint64_t lastDigest_ = 0, digestSeq_ = 0;
    uint32_t digestId_ = 0;
    std::shared_ptr<const ShadowState> digestWorld_;
    std::set<uint64_t> touchedSince_;
    SyncStats syncStats_;
};

}  // namespace fnvcoop
