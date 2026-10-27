// TEST-ONLY fault switches used by the mutation-twin tests. All false in production.
// Each flag re-introduces one classic bug; the matching test must then fail.
#pragma once
#include <cstdint>
namespace fnvcoop {
struct Mutations {
    bool noEchoGuard = false;      // remote applies are captured as local actions again
    bool noIdempotency = false;    // re-delivered txns / intents are applied again
    bool stageLastWins = false;    // stage merge = overwrite instead of max
    bool flagsAnd = false;         // objective flag merge = AND instead of OR
    bool killsReplace = false;     // kill merge = replace instead of union
    bool applyOutOfOrder = false;  // apply txns as they arrive instead of in seq order
    bool slowHashing = false;           // digests hash via per-entry objects (the pre-optimization path)
    bool skipHandshake = false;         // the host accepts any build / rules / mod list
    bool chattyBaseline = false;        // pre-fix behaviour: every Hello re-mints the join job + Welcome, acks re-send whole windows
    bool healDisabled = false;          // desync is detected and reported but NOT repaired
    bool rulesDisabled = false;         // quest rules engine does nothing (no deferrals / recipient rewrites)
    bool barrierReleaseEarly = false;   // a barrier releases on the FIRST arrival
    bool snapshotFloodAfterBaseline = false;  // baseline joiner also applies the whole unfiltered snapshot diff
    bool noEpochBump = false;           // host load does not bump the world epoch (stale txns / no rebase)
    bool ignoreVolatile = false;        // volatile-keys list is not consulted
    bool baselineSkipped = false;       // joiner ignores the world baseline chunks (keeps its own divergent world)
    bool guestRunsQuestScripts = false;  // guests execute quest scripts themselves (divergent local var writes)
    bool failNotCompleted = false;     // FailQuest sets only the failed flag (not completed) - the old CompleteQuest-only capture
    bool sidecarLosesLedger = false;   // the client save sidecar forgets which rewards were delivered (restart re-delivers them)
    bool spawnResurrects = false;      // a replayed SpawnRef recreates a ref that was despawned
    bool stickyObjectsReset = false;   // destroyed / activate-once object flags can be un-set
    bool refInvNoClear = false;        // RemoveAllItems does not clear a ref inventory
    bool activeQuestShared = false;    // ForceActiveQuest (UI focus) is broadcast to everyone instead of the initiator
    bool ignoreFollowerCaps = false;   // recruits beyond the per-owner / party follower caps are accepted
    bool disconnectKeepsLock = false;  // a disconnecting player keeps their control lock (rejoin comes back locked)
    bool guestLocalReplicated = false; // GuestLocalRun quests are captured and replicated like any other script
    bool departedHoldBarrier = false;  // players who left still count as present (a barrier waits for them forever)
    bool objectiveOr = false;         // host objective writes OR into the flags again (a hide is lost on resync / late join)
    bool noRewardIds = false;         // a baseline or plain-snapshot rejoin ignores delivered reward ids (the whole ledger is re-delivered)
    bool paymentSharesAll = false;    // a RemoveItem in a topic result is charged to the whole party instead of the initiator
    bool tetherIgnoresAnchor = false; // anchor+radius tethers are always satisfied (the party is never gathered outdoors)
    bool stagesParamIgnored = false;  // a rule's per-stage `stages` list applies to every stage of the quest
    bool sumAsAny = false;            // sum-over-party / max-over-party degrade to any-party-member (0 or 1)
    bool exclusiveLastWins = false;   // ExclusiveStages: a later outcome replaces the first committed one
    bool readyProceedsAlone = false;  // ReadyCheck / time-skip consent releases on the first confirmation
    bool slideshowNoHold = false;     // the control lock after a slideshow barrier is released immediately
    bool perPlayerSceneShared = false;// PerPlayerScene routes nothing to the acting player (rewards stay party-wide)
    bool perPlayerVarsShared = false; // PerPlayerVars: every slot reads the last writer's value
    bool originsIgnored = false;       // weather / time scale / ref var / map marker / lock pre-images are not used: a rebase cannot restore them
    bool anglePreimageIgnored = false; // a rebase / diff cannot restore a ref axis the loaded save never rotated (the pre-image is not used)
    bool angleKeepsFirst = false;     // RefAngle: a later committed angle for the same (ref, axis) is ignored (host order no longer wins)
    bool refCmdKeepsFirst = false;    // RefCommand state ops: a later command for the same key is ignored (host order no longer wins)
    bool refCmdPlayerShared = false;  // a RefCommand on the player is shared with the whole party whatever the cause (a vending machine would pay everybody)
    bool stashOriginOnly = false;     // a ContainerTransfer (quest inventory stash) reaches only the acting player: the others keep their gear
    bool refCmdNarrow = false;        // wide RefCommand ops (UseWeapon) travel and hash without their extra forms / integer: the receiver fires with the wrong target
    bool refCmdAllState = false;      // RefCommand events are remembered like state ops (a late joiner replays them, an event fires twice)
    bool noGuestSuppression = false;  // guests keep script-originated effects (ghost-death double fire)
    bool overfullShardRequest = false; // a digest repair asks for every bad shard at once (the client's reply is cut off, the drift beyond it is never fixed)
    bool questRunningReverted = false; // issue #25: client-only quest running keys go out as revert keys again (no inverse: never converges)
    bool stageZeroReverted = false;    // client-only "SetStage q = 0" keys go out as revert keys again (SetStage merges by max, no inverse: the digest never converges, a repair every round)
    bool questStateReverted = false;   // client-only non-zero SetStage / CompleteQuest / FailQuest keys go out as revert keys again (no inverse: never converges, resent every round)
    bool digestKeepsRefVars = false;   // the host digest compares RefScriptVar (polled object / NPC script vars): the client's world dump never carries them, so every one is "missing" and repaired forever (digest never ok)
    bool digestKeepsWorldEnv = false;  // the digest compares the host's last SetWeather / SetTimeScale: the client's world dump cannot read either one, so both are "missing" and repaired every round
    bool trustForeignCursor = false;   // a Hello's resume cursor of the host's epoch is replayed / snapshotted without checking it is this host's timeline
    // Host-load revert twins, one effect at a time (originsIgnored above switches all of them off at once). Bits of PreimageBit.
    bool asyncDoneHoldsQueue = false;   // issue #46: runAsync's queued completion owns the queue it sits in (a reference cycle: leaks when a session dies with a result queued)
    uint32_t preimageOff = 0;
};
// One bit per effect whose host-load revert has its own twin.
enum PreimageBit : uint32_t {
    kPiWeather = 1, kPiTimeScale = 2, kPiRefVar = 4, kPiMarker = 8, kPiLock = 16, kPiObjectState = 32, kPiRefInv = 64, kPiSpawn = 128,
    kPiClearPre = 256,   // a RemoveAllItems sent by a diff / baseline does not carry the contents it discarded (the receiver cannot undo the clear)
    kPiStickyOpen = 512, // the open bit of an object that also has a sticky (destroyed / activated once) bit is not reverted by the baseline path
    kPiAll = 1023
};
inline Mutations& mutations() {
    static Mutations m;
    return m;
}
// True when the revert for this effect is switched off by a twin (production: always false).
inline bool preimageIgnored(uint32_t bit) { return mutations().originsIgnored || (mutations().preimageOff & bit) != 0; }
struct MutationScope {
    Mutations saved;
    explicit MutationScope(const Mutations& m) : saved(mutations()) { mutations() = m; }
    ~MutationScope() { mutations() = saved; }
};
}  // namespace fnvcoop
