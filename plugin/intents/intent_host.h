// Host side of the guest intents (FULL-SYNC-PLAN 3.7). Game-free: the engine, the network and the runtime are injected.
//
// A guest's intent arrives (or a legacy story-channel ActivateRef / TriggerEnter / PickUp does), the host
//   1. checks the sender is a joined player, the id is new (exactly-once per session), the rate, that the kind is switched on;
//   2. resolves the target (loaded? what is it?), picks the ACTING PLAYER: the guest's puppet, else NV:MP's ghost, else (policy permitting)
//      the host's own player as a stand-in, and checks the puppet is near enough;
//   3. runs the interaction inside Runtime::runAsGuest (origin forced to the guest) + an acting scope (capture scope, `player` = the puppet);
//   4. answers with a Verdict. Everything the interaction changed travels as an ordinary txn with the guest as origin.
//
// Shared state of its own, all bounded: item claims (first wins), container locks (one user, leased), trigger cooldowns, per-player id windows.
#pragma once
#include <deque>
#include <functional>
#include <map>
#include <string>
#include <vector>

#include "fnvcoop/protocol.h"
#include "intents/container_redirect.h"
#include "intents/intent_engine.h"
#include "intents/intent_policy.h"
#include "intents/intent_wire.h"
#include "intents/retry_list.h"
#include "intents/stage_pass.h"
#include "intents/storage_book.h"

namespace nvcoop::intents {

constexpr RefId kHostPlayerRef = 0x14;

// Who acts for a guest. Told to HostDeps::actorMode when it changes for a slot (the module logs it; a stand-in is the host's own player).
enum class ActorMode : uint8_t { Puppet, Ghost, StandIn };

struct HostDeps {
    IIntentEngine* eng = nullptr;
    std::function<bool(PlayerId slot, Bytes data)> send;                              // to one joined guest, reliable
    std::function<bool(PlayerId slot, const std::function<void()>& body)> runAsGuest; // Runtime::runAsGuest
    std::function<RefId(PlayerId slot)> puppetOf;                                     // avatar layer: the puppet of that player (0 = none)
    std::function<RefId(PlayerId slot)> ghostOf;                                      // NV:MP ghost (0 = none)
    std::function<void(PlayerId slot, ActorMode mode, RefId actor)> actorMode;         // optional: the acting player for a slot changed (puppet / ghost / stand-in)
    std::function<bool(PlayerId slot)> joined;                                        // is that slot a joined player right now
    std::function<uint64_t()> nowMs;
    std::function<void(const FormRef& npc, PlayerId slot)> dialogueStart;             // optional: tell the 0xD1 adapter
    // plugin/companions (nvcoop::social::glueAllowActivation / glueGuestInteraction; the module wires them under NVCOOP_WITH_COMPANIONS). Both optional.
    //   allowActivation  asked BEFORE an Activate / DialogueStart runs; false = refuse it (Result::Denied): somebody else's companion
    //   interacted       told AFTER a guest's Activate / PickUp / ContainerOpen ran (Result::Executed): theft, trespass and break-in are judged from it
    std::function<bool(PlayerId slot, const FormRef& target)> allowActivation;
    std::function<void(PlayerId slot, bool pickup, const FormRef& target)> interacted;
    // Kind::SetStage policy, asked AFTER the "quest is behind that stage" check and BEFORE anything runs. false = refuse (Result::Denied, counted in
    // stagesRefused and logged through stageRefused). The module wires it to the rules engine's ExclusiveStages (a losing alternative outcome would
    // run its result scripts on the host and only then have its SetStage dropped from the txn). Empty = allow.
    std::function<bool(PlayerId slot, const FormRef& quest, uint32_t stage)> allowStage;
    // Kind::SetStage on a Barrier stage of the rules engine is the guest's PASS (G26): asked after the "quest is behind that stage" check and BEFORE allowStage. Deferred = the
    // stage waits for the whole party and the rules engine performs it on release, so the host does NOT run it (Result::Deferred, stagesDeferred); NotPass / Proceed = unchanged.
    // Empty = every stage is NotPass. stage_pass.h has the adapter for the rules engine.
    GuestStagePassFn guestStagePass;
    std::function<void(PlayerId slot, const FormRef& quest, uint32_t stage, Result why)> stageRefused;   // optional: the host says it refused / held / skipped a guest stage
    const RetryList* retryList = nullptr;   // the activators the host's own player may redo after a puppet's activation did nothing (retry_list.h). null / empty = never redo
    std::function<void(const std::string& line)> log;   // optional: one line per refused intent (kind, reason, distance, both positions, ref, cell), rate-limited by the host
};

struct HostStats {
    uint64_t received = 0, badDatagrams = 0, legacy = 0, verdictsSent = 0, verdictsFailed = 0;
    uint64_t executed = 0, duplicates = 0;
    uint64_t rejected[static_cast<size_t>(Result::Count_)] = {};
    uint64_t byKind[static_cast<size_t>(Kind::Count_)] = {};          // executed per kind
    uint64_t actedAsPuppet = 0, actedAsGhost = 0, actedAsStandIn = 0;
    uint64_t reachChecked = 0, reachUnchecked = 0, scriptsSkipped = 0;
    uint64_t claimsWon = 0, claimsLost = 0, lockGrants = 0, lockBusy = 0, lockExpired = 0, lockReleased = 0, redirects = 0;
    uint64_t activationsRefused = 0, interactionsReported = 0;   // plugin/companions hooks (HostDeps::allowActivation / interacted)
    uint64_t noteGrants = 0;                                    // a picked-up note went to the journal (AddNote), not to an inventory (G10)
    uint64_t stayRaises = 0, stayStarted = 0, stayExpired = 0, stayDropped = 0;   // Kind::TriggerStay presences (G04)
    uint64_t eventScriptRuns = 0, eventScriptMissed = 0, eventsFlushed = 0;       // marked events whose script ran now as the guest / had no script to run / host marks run first
    uint64_t stashSwaps = 0, stashFailed = 0, stashKept = 0;    // gear stashes swapped in / failed to swap / left untouched (G35)
    uint64_t standInRetries = 0, standInGainsReverted = 0;   // activations the puppet did to no effect, redone by the host's player / items that player gained and gave back
    uint64_t ranUnloaded = 0;                         // activations run on a target whose cell the host has not loaded (HostConfig runUnloaded)
    uint64_t refusalsLogged = 0, refusalsQuiet = 0;   // refused intents written to HostDeps::log / skipped by its rate limit
    uint64_t stagesApplied = 0, stagesNoAdvance = 0, stagesRefused = 0, stagesDeferred = 0, stagesFailed = 0, stagesBad = 0;   // Kind::SetStage verdicts
};

class IntentHost {
public:
    // `stash`: the gear ledger (storage_book.h), owned by the caller so it outlives the session; null = no stashes.
    IntentHost(HostDeps deps, Config cfg, ContainerRedirect redirect = ContainerRedirect(), StorageBook* stash = nullptr);

    // A datagram from `from` (a transport peer already mapped to a slot). Malformed input is counted and dropped.
    void onDatagram(PlayerId from, const uint8_t* data, size_t size);
    // The whole pipeline for one decoded intent. Sends the verdict (when `send` is set) and returns it.
    Verdict handle(PlayerId slot, const Intent& in);
    // Story-channel intents (ClientSession::submitAction): same checks and execution, no id window, no verdict.
    Result runLegacy(PlayerId slot, fnvcoop::IntentKind kind, const FormRef& ref);

    void tick();                       // lock leases, bounded tables
    void playerLeft(PlayerId slot);    // releases the player's container locks and forgets their id window
    void worldReset();                 // the host loaded a save: claims, locks, cooldowns belong to the old world

    struct LockInfo { FormRef ref; PlayerId holder; uint64_t sinceMs; };
    std::vector<LockInfo> locks() const;
    bool claimed(const FormRef& item) const { return claims_.count(item) != 0; }
    size_t claimCount() const { return claims_.size(); }
    size_t presenceCount() const { return presence_.size(); }   // (player, trigger) pairs the host re-raises OnTrigger for

    // Gear stashes around a guest's interaction (G35). enter: the shared containers of the book hold exactly `slot`'s stash; exit: what they hold now becomes
    // `slot`'s stash again and the parked contents return. No-ops for the host's own player, without a book or with [intents] storage=0. runScoped does this
    // itself; the service uses it around a guest conversation's result scripts.
    struct StashHold {
        struct One { FormRef container; std::vector<StoredItem> parked, loaded; bool swapped = false; };
        std::vector<One> held;
        PlayerId slot = kNoPlayer;
        bool active = false;
    };
    StashHold stashEnter(PlayerId slot);
    void stashExit(StashHold& hold);
    const HostStats& stats() const { return stats_; }
    const Config& config() const { return cfg_; }
    std::string statusJson() const;

private:
    struct Actor { RefId ref = 0; bool puppet = false, ghost = false, standIn = false; };
    struct SlotState {
        uint32_t session = 0;
        uint32_t lastId = 0;
        std::deque<Verdict> recent;     // last verdicts, repeated for a duplicate id
        double tokens = 0;
        uint64_t refilledAt = 0;
        bool init = false;
    };
    Verdict decide(PlayerId slot, const Intent& in, bool legacy);
    Verdict dispatch(PlayerId slot, const Intent& in, const TargetInfo& t);
    bool rateOk(SlotState& s, uint64_t now);
    bool pickActor(PlayerId slot, Kind k, TargetClass cls, Actor* out);
public:
    // Reports the acting player of a slot to HostDeps::actorMode when the mode differs from the last one reported for it.
    void noteActor(PlayerId slot, ActorMode mode, RefId actor);
private:
    Result checkReach(const Actor& a, const TargetInfo& t, Kind k);
    bool retryAllowed(const TargetInfo& t) const;
    Result runScoped(PlayerId slot, const Actor& a, Kind k, const FormRef& target, const std::function<Result(IActingScope&)>& fn, bool stash = true);
    bool raiseNow(const Actor& a, const FormRef& target, uint32_t mask);   // inside an open acting scope: mark the event for `player` and run the target's script now
    Result raiseFor(PlayerId slot, const Actor& a, Kind k, const FormRef& target, uint32_t mask, bool stash = true);
    Verdict doStay(PlayerId slot, const Intent& in, const TargetInfo& t);
    void raisePresence();
    Verdict doActivate(PlayerId slot, const Intent& in, const TargetInfo& t);
    Verdict doPickUp(PlayerId slot, const Intent& in, const TargetInfo& t);
    Verdict doContainerOpen(PlayerId slot, const Intent& in, const TargetInfo& t);
    Verdict doContainerClose(PlayerId slot, const Intent& in);
    Verdict doEvent(PlayerId slot, const Intent& in, const TargetInfo& t, uint32_t mask);
    Verdict doTerminal(PlayerId slot, const Intent& in, const TargetInfo& t);
    Verdict doSetStage(PlayerId slot, const Intent& in);
    uint64_t now() const { return deps_.nowMs ? deps_.nowMs() : 0; }
    Verdict verdictOf(const Intent& in, Result r) const;
    void count(const Intent& in, Result r);
    // One log line for a refused intent, or a run one that is not a trigger (HostDeps::log): kind, reason, slot, the target ref / cell / class, the acting puppet, both positions and the distance.
    // At most one line per (kind, reason) per kRefusalLogGapMs; the lines skipped in between are counted in the next line.
    void logRefusal(PlayerId slot, const Intent& in, Result r);
    std::string refusalDetail(PlayerId slot, const Intent& in);
    struct RefusalGate { uint64_t lastMs = 0; uint32_t skipped = 0; };
    std::map<std::pair<uint8_t, uint8_t>, RefusalGate> refusalGate_;

    HostDeps deps_;
    Config cfg_;
    ContainerRedirect redirect_;
    StorageBook* stash_ = nullptr;
    int stashDepth_ = 0;
    struct Presence { uint64_t lastSeenMs = 0, lastRaiseMs = 0; };
    std::map<std::pair<PlayerId, FormRef>, Presence> presence_;   // who is inside which trigger volume (TriggerEnter / TriggerStay), re-raised every host frame
    std::map<PlayerId, SlotState> slots_;
    std::map<PlayerId, ActorMode> lastMode_;   // last acting-player mode reported per slot
    struct Claim { PlayerId slot; uint64_t atMs; };
    std::map<FormRef, Claim> claims_;
    std::deque<FormRef> claimOrder_;
    struct Lock { PlayerId slot; uint64_t sinceMs, refreshedMs; };
    std::map<FormRef, Lock> locks_;
    std::map<std::pair<PlayerId, FormRef>, uint64_t> lastTrigger_;
    HostStats stats_;
};

}  // namespace nvcoop::intents
