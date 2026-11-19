// Game-side glue (x86 only, compiled against xNVSE headers): installs the prologue-checked detours and feeds the
// CaptureHub. NOTHING happens until Init() is called by plugin main (L1) -- including on the first load, so a build
// that links this library is inert by default.
//
// Threading: Init/Shutdown/hooks all run on the game main thread (script execution is single-threaded).
#pragma once
#include <functional>
#include <string>
#include <vector>

#include "capture_hub.h"
#include "hook_patch.h"
#include "party_cond.h"
#include "suppress_policy.h"

namespace nvhooks {

// refID <-> (plugin name, local id). The default implementation reads the engine's load order
// (DataHandler at 0x11C3F2C) and form map (LookupFormByID 0x4839C0). L1 may inject its own.
struct FormResolver {
    virtual ~FormResolver() = default;
    virtual bool toRef(uint32_t refID, fnvcoop::FormRef* out) = 0;
    virtual uint32_t fromRef(const fnvcoop::FormRef& r) = 0;  // 0 = plugin not loaded
    virtual void* lookup(uint32_t refID) = 0;                 // TESForm*, or nullptr
};
FormResolver& defaultResolver();

struct HookReport {
    std::string name;
    PatchStatus status;
};
struct InitReport {
    std::vector<HookReport> hooks;
    int ok = 0;
    int failed = 0;
    bool allCritical() const;  // SetStage + TopicInfoRunResult + ScriptExecute installed
};

// Host: capture + play normally. Guest: all hooks installed, capture off, and LOCAL script-driven mutations of shared
// state are blocked (see below). Client: only the ScriptGate hook.
enum class Role { Host, Client, Guest };

// Installs the hooks. Host: capture hooks (SetStage, topic result, objective/quest/rep/karma/ref/item/AV, ...).
// Client: only the ScriptGate hook (used while applying remote effects). A hook whose prologue does not match the
// expected bytes is skipped (reported in InitReport), never forced.
InitReport Init(CaptureHub& hub, Role role, FormResolver* resolver = nullptr);
void Shutdown();  // restores every patched byte (only if still ours)
// Hot reload preflight: how many of our installed hooks another patcher has chained over (0 = safe to unload).
int auditTamperedHooks();
std::string tamperedHookTargets();   // "0x0058A3B0 0x...": the hook targets whose bytes someone overwrote (hot-reload refusals name them)
std::string chainedHookTargets();    // "0x005AC750->0x0B6A1234 ...": hooks installed on top of another patcher's entry jump (hook_patch rule 4)

CaptureHub* hub();
FormResolver& resolver();

// While alive, Script::Execute (0x5AC1E0) returns true without running the script. Used by RemoteApplier so that
// replaying a quest stage runs its engine-native parts (journal text, quest flags, notifications) but NOT the
// result script, whose effects arrive as separate captured effects.
class ScriptGate {
public:
    ScriptGate();
    ~ScriptGate();
    ScriptGate(const ScriptGate&) = delete;
    ScriptGate& operator=(const ScriptGate&) = delete;
};
int scriptGateDepth();
// Lifts every ScriptGate alive on this thread for its own lifetime. A console line IS a script: an applier that runs one inside a
// gate (WorldApplier: the plan's console steps) must lift it, or Script::Execute skips the line and the command reports success.
class ScriptGateLift {
public:
    ScriptGateLift();
    ~ScriptGateLift();
    ScriptGateLift(const ScriptGateLift&) = delete;
    ScriptGateLift& operator=(const ScriptGateLift&) = delete;
private:
    int saved_;
};

// Non-host suppression (Role::Guest). A local script that mutates shared state is blocked when a script is running
// (Script::Execute depth > 0) and no host txn is being applied. INFO result scripts never run locally on a guest.
// Wrap the application of a host EffectTxn (RemoteApplier does) in ApplyScope to lift the block.
// This also covers NV:MP's local ghost-NPC kill: the death-event script still fires, but its SetStage/AddItem/... no-op.
class ApplyScope {
public:
    ApplyScope();
    ~ApplyScope();
    ApplyScope(const ApplyScope&) = delete;
    ApplyScope& operator=(const ApplyScope&) = delete;
};
// Suppressed (counters) is declared in suppress_policy.h.
// Rules: quest-owned scripts are skipped on guests (setGuestQuestGate to disable); INFO/stage result scripts never run;
// object/NPC scripts run but shared-state mutators (incl. faction rank, perks, script packages, combat, lock/unlock,
// script Activate, FailQuest/CompleteQuest) are dropped.
void setGuestQuestGate(bool on);
const Suppressed& suppressed();
bool applying();
// A RewardXP script command is running on this thread (main thread): its XP travels as a captured reward.
bool inRewardXpCommand();
// AddItem / RemoveItem / RemoveAllItems calls made by NV:MP itself (they bypass the guest gate and the capture).
uint64_t nvmpItemCalls();
// NV:MP's RemoveAllItems calls that faulted inside the engine and were abandoned instead of crashing (engine_hooks.cpp), and where to report them.
uint64_t nvmpItemFaults();
void setNvmpItemGuardReport(void (*report)(const char*));
// NV:MP's NPC inventory resyncs (RemoveAllItems + AddItems) done incrementally (engine_hooks.cpp): call once per frame to finish them.
void tickNvmpResync();
struct NvmpResyncStats { uint64_t resyncs = 0, addsAbsorbed = 0, addsTrimmed = 0, removed = 0; };   // removed: item units
NvmpResyncStats nvmpResyncStats();
// Full item counts (base container + changes) of every plain item `ref` holds: (item refID, count).
std::vector<std::pair<uint32_t, int>> heldItemCounts(void* ref);
// Calls of TESObjectREFR::IsDestructible answered "no" because NV:MP flagged a reference whose base has no destruction data.
uint64_t bogusDestructibleRefs();
// Actor movement updates whose character controller was gone (the call at 0x9303C3 skipped instead of crashing at 0xCA9600).
uint64_t nullMoveControllers();



// Script::Execute accounting (proof that replaying a txn ran no result script).
struct ScriptStats {
    uint64_t gated = 0;             // calls swallowed by a closed ScriptGate
    uint64_t ran = 0;               // calls that executed
    uint64_t ranWhileApplying = 0;  // subset of `ran` during a RemoteApplier::apply (must be 0 with the gate on)
    uint64_t localRan = 0;          // GuestLocalRun allow-listed scripts that executed (subset of `ran`)
    uint64_t applierLines = 0;      // console lines the applier itself executed (kill/resurrect/move); not counted above
    uint64_t questVarRollbacks = 0; // guest: shared quest vars an object / NPC / effect script wrote and that were put back (host is authoritative)
    uint64_t questVarGuarded = 0;   // guest: executions of scripts known to write a shared quest var
    uint64_t ownActivations = 0;    // guest: OnActivate hand-backs (`Activate` on what the player just activated) let through
};
const ScriptStats& scriptStats();
// Distinct GuestLocalRun script refIDs that ran, with run counts (diagnostic; at most 64 entries).
std::vector<std::pair<uint32_t, uint32_t>> localRunCounts();
// Cost of OUR code inside the Script::Execute detour (everything except the engine's own execution), measured with QueryPerformanceCounter
// while enabled. Diagnostic for the per-frame budget (L4: 33 ms frame, host tick p50 38 us).
struct HookPerf {
    uint64_t calls = 0;         // detour calls measured
    uint64_t overheadTicks = 0; // QPC ticks spent in our pre/post code
    uint64_t freq = 1;          // QPC ticks per second
};
void setHookPerf(bool on);
HookPerf hookPerf();
const Suppressed& suppressStats();
// Bracket the applier's own console-line executions so they are not mistaken for leaked result scripts.
class ApplierExecScope {
public:
    ApplierExecScope();
    ~ApplierExecScope();
    ApplierExecScope(const ApplierExecScope&) = delete;
    ApplierExecScope& operator=(const ApplierExecScope&) = delete;
};

// Host-authoritative quest vars / globals (L4 StateReplicator). Quest scripts only run on the host and write vars every
// frame, so these are NOT put into capture txns: the host polls each tick and feeds the replicator.
//   nvhooks::enableStatePolling(true);            // turns the per-scope var/global diff off (no duplicates in txns)
//   nvhooks::pollHostState(sink);                 // once per host tick: sink.scriptVar()/global() for every CHANGED value
// The first poll reports every value (baseline) and later polls only changes, unless the poll was enabled with prime=true (a host WITHOUT a
// world baseline): then the first pass only records what is already there and nothing is reported until it changes.
// Cost is one pass over all quests/globals.
struct StateSink {
    virtual ~StateSink() = default;
    virtual void scriptVar(const fnvcoop::FormRef& quest, uint16_t index, double value) = 0;
    virtual void global(const fnvcoop::FormRef& global, float value) = 0;
};
void enableStatePolling(bool on, bool prime = false);
size_t pollHostState(StateSink& sink);

// Guest-originated activation (TESObjectREFR::Activate 0x573170, thiscall(ref, activator, u8, u32, u32), ret 0x10).
// Installed on Role::Guest only. Called when the LOCAL PLAYER activates `target` outside any running script and not while
// applying a host txn. The observer (L10) decides: Proceed = engine activates locally (UI / animation; shared-state script
// effects are still dropped by the gate), Suppress = the engine call is skipped entirely. To send the intent to the host call
// ClientSession::submitAction(IntentKind::ActivateRef, target) from inside the observer.
enum class ActivateAction { Proceed, Suppress };
using ActivateObserver = std::function<ActivateAction(const fnvcoop::FormRef& target)>;
void setActivateObserver(ActivateObserver obs);
// An activator (ACTI / talking activator) with a script attached: what it does lives in its OnActivate block (a repair check, a quest step).
bool isScriptedActivator(uint32_t refID);
// Under NV:MP: an unlocked door that is not a load door. A guest's use of it is the host's to run (NV:MP puts a guest's open back).
bool isForwardableDoor(uint32_t refID);
// The same kind of door, locked on this machine (a guest who picked the lock sends its use while the host's copy is still locked).
bool isLockedPlainDoor(uint32_t refID);
// This player opened / closed an ordinary door (unlocked, not a load door, NV:MP present), host or guest: refID and where it ends up.
// NV:MP keeps only one copy's door state (the cell's "owner", either player) and puts the other's change back, so the plugin sends the
// result to the other players, who set their copy the same way (coop_module.cpp, channel 0xD9).
using DoorUseObserver = std::function<void(uint32_t refID, bool open)>;
void setDoorUseObserver(DoorUseObserver obs);
int doorOpenStateNow(uint32_t refID);   // GetOpenState of a loaded door (1 open, 2 opening, 3 closed, 4 closing; 0 = unknown)
// The host opened this door for a guest: its own script's auto-close / re-lock is held for a while (TenpennyDoorScript took the console
// activation for an NPC's and shut and locked the door again; a guest's door opened by NV:MP gets the same).
void holdDoorForGuest(uint32_t refID);
// Guest: conversations open only when the local player activated the NPC (talk_gate.h; hook 0x705070, Role::Guest only). Off until enabled.
class TalkGate;
TalkGate& talkGate();
// Guest: this player's own conversation reached an INFO result script (blocked here; guests never run one). The observer asks the host
// to run it (IntentKind::DialogueResult). Only for the DialogMenu the local player opened, never for barks or replicated scripts.
using DialogueResultObserver = std::function<void(const fnvcoop::FormRef& info, const fnvcoop::FormRef& npc, bool onEnd)>;
void setDialogueResultObserver(DialogueResultObserver obs);
// A line this machine replays from another player's conversation (echo_lines.h): its result scripts are skipped for `forMs` (begin and end),
// unless the local player is in their own conversation with that NPC.
void markEchoLine(uint32_t infoRefID, uint32_t npcRefID, uint32_t forMs);
uint64_t echoLinesSkipped();
// Host: run INFO `infoRefID`'s begin/end result script with `npcRefID` as the speaker, captured, while `actorRefID` (a guest's NV:MP ghost,
// or the host player as stand-in) is the acting player of party slot `slot`. False when the hook is missing or the forms are not loaded.
bool runInfoResultFor(uint32_t actorRefID, uint8_t slot, uint32_t infoRefID, uint32_t npcRefID, bool onEnd);
// Guest: lines of other conversations (barks, NPC to NPC) that came 4+ times within a minute here: a conversation loop on this copy.
struct RepeatedLine { uint32_t info = 0, speaker = 0, perMinute = 0; uint64_t atMs = 0; };
const std::vector<RepeatedLine>& repeatedLines();
// NV:MP's door calls (SetOpenState / Activate) seen here, door scripts' closes held on doors NV:MP opened, doors held open now.
struct DoorSyncStats { uint64_t nvmpCalls = 0, closesHeld = 0, held = 0, notSent = 0; };   // notSent: door states left to NV:MP
DoorSyncStats doorSyncStats();
// Story capture of Activate on a door: the door's GetOpenState after the activation (0 = unknown). Set by the plugin (the world module
// owns the condition evaluator); unset = the Activate is recorded as the command, as before.
void setDoorStateQuery(std::function<int(uint32_t refID)> f);

// Player control lock (v1 for the baseline transfer): DisablePlayerControls / EnablePlayerControls (0x5CAE30 / 0x5CAD20) with all
// seven flags, issued as console lines through the executor injected by the plugin (hooks_module sets it). Idempotent: only the
// transitions run. Unlock re-enables ALL flags (it cannot know what a cutscene had disabled before).
void setConsoleExec(std::function<bool(const std::string&, uint32_t refID)> f);  // refID 0 = no calling reference
void SetPlayerControlsLocked(bool locked);
bool playerControlsLocked();

// Host: let the rules engine defer a SetStage that the engine is about to perform (not ours, not while applying, not inside a
// StageGateBypass). `quest` is the engine refID (resolver().toRef() converts). Return false = the engine SetStage is skipped and
// the detour returns false. The owner re-issues the stage later under a StageGateBypass.
using StageGateFn = std::function<bool(const fnvcoop::FormRef& quest, uint32_t stage)>;
void setStageGate(StageGateFn f);
class StageGateBypass {
public:
    StageGateBypass();
    ~StageGateBypass();
    StageGateBypass(const StageGateBypass&) = delete;
    StageGateBypass& operator=(const StageGateBypass&) = delete;
};
// Local read of a quest's current stage (TESQuest+0x60) for scene-pass detection (e.g. VCG01 chargen finished). -1 if unknown.
int localQuestStage(const fnvcoop::FormRef& quest);
// Local read of a quest's TESQuest::flags byte (bit0 running, bit1 completed, bit6 failed; a failed quest also has bit1). -1 if unknown.
int localQuestFlags(const fnvcoop::FormRef& quest);
// Execute a console line through the injected executor (for the owner of a deferred stage: "setstage <hex> <n>").
bool execConsoleLine(const std::string& line);
// Same, with `refID` as the console's calling reference (so `KillActor`, `MoveTo ...`, `GetDead` act on that reference).
bool execConsoleLineOn(const std::string& line, uint32_t refID);

// Diagnostic: every ScriptLocal (id, value) of a quest's ScriptEventList exactly as pollHostState sees it (empty if the quest has none).
std::vector<std::pair<uint32_t, double>> questVarsDump(const fnvcoop::FormRef& quest);

// While alive, nvhooks::actingPlayer() returns `actorRefID` (a guest's ghost) instead of the host player 0x14.
class ActingPlayerScope {
public:
    explicit ActingPlayerScope(uint32_t actorRefID);
    ~ActingPlayerScope();
    ActingPlayerScope(const ActingPlayerScope&) = delete;
    ActingPlayerScope& operator=(const ActingPlayerScope&) = delete;
};
uint32_t actingPlayer();
// Party slot of actingPlayer(): 0 = host player; guests' ghosts are registered by the session (L6/L10) when they join.
void registerPlayerSlot(uint32_t ghostRefID, uint8_t slot);
uint8_t actingSlot();

// Party-aware condition evaluation (party_cond.h). The provider lists the PRESENT party members (the local player as refID 0x14 + every
// other member's ghost reference); the reducer is RulesEngine::reduceCondition (values per member -> one answer). Both unset = vanilla.
using PartyProvider = std::function<std::vector<PartyMember>()>;
void setPartyConditions(PartyProvider provider, ConditionReducer reducer);
struct CondStats {
    uint64_t evals = 0;       // evaluator calls seen (party-aware functions only)
    uint64_t partyEvals = 0;  // calls that had the player as operand and ran once per member
    uint64_t reduced = 0;     // calls whose result came from the reducer
};
const CondStats& condStats();
// While alive, party conditions are reduced under this quest's PartyAwareCond rule (set by dialogue / quest-condition callers that know it).
class ConditionQuestScope {
public:
    explicit ConditionQuestScope(const fnvcoop::FormRef& quest, fnvcoop::CondPhase phase = fnvcoop::CondPhase::Entry);
    ~ConditionQuestScope();
    ConditionQuestScope(const ConditionQuestScope&) = delete;
    ConditionQuestScope& operator=(const ConditionQuestScope&) = delete;
};

// GuestLocalRun allow-list (rules.json): quest-owned SCRIPT forms that keep running on guests (achievements, rumble, DLC setup, fades).
// Unknown plugins/ids are ignored. Replaces the previous list.
void setGuestLocalRun(const std::vector<fnvcoop::FormRef>& scripts);

// Guest-originated script events. Every engine-raised script event (OnTrigger/OnTriggerEnter/OnTriggerLeave/OnAdd/OnHit/OnOpen/...)
// goes through MarkBaseExtraListScriptEvent(object form, target's BaseExtraList, mask) @0x5AC750 (cdecl). `mask` is one
// ScriptEventList::kEvent_* bit; the target reference is recovered from its embedded extraDataList (+0x44) when the list belongs to a
// reference. Installed on Role::Guest only; called only when the LOCAL PLAYER is the event's object, outside scripts and outside applies.
// Proceed = the event is raised locally as usual; Suppress = it is not raised (the host runs the block via an Intent).
enum class EventAction { Proceed, Suppress };
using EventObserver = std::function<EventAction(uint32_t mask, const fnvcoop::FormRef& target)>;
void setEventObserver(EventObserver obs);
constexpr uint32_t kEvOnAdd = 0x1, kEvOnDrop = 0x4, kEvOnHit = 0x80, kEvOnOpen = 0x10000, kEvOnClose = 0x20000, kEvOnTrigger = 0x10000000,
                   kEvOnTriggerEnter = 0x20000000, kEvOnTriggerLeave = 0x40000000;

// Host-side guard for other lanes' mutation hooks: true = a local script on a guest is mutating shared state, drop it (counts as suppressed).
bool blockMutationNow();
// True when `ret` (a return address) is inside NV:MP's client.dll: the command call is NV:MP's own sync, not a script's.
bool isNvmpCaller(const void* ret);
// Non-player actor values (SetActorValue/ModActorValue/ForceActorValue/DamageActorValue on an actor that is not the player), reported after the
// original ran with the actor value before/after, only while capturing. kind: 0 set, 1 mod, 2 force, 3 damage, 4 restore (hooks_world AvCmdKind). The observer turns it into
// effects (hub()->record). Unset = only counted in unmapped().avOnNonPlayer.
using NonPlayerAvObserver = std::function<void(const fnvcoop::FormRef& actor, uint32_t av, uint8_t kind, float before, float after)>;
void setNonPlayerAvObserver(NonPlayerAvObserver obs);
// AddScriptPackage / RemoveScriptPackage (host, capturing, after the original command succeeded).
using ScriptPackageObserver = std::function<void(const fnvcoop::FormRef& actor, const fnvcoop::FormRef& package, bool add)>;
void setScriptPackageObserver(ScriptPackageObserver obs);

// Story-capture (docs/tools/story-capture.md): hooks ~75 more script commands (AI orders, spells, ownership, chargen ...) and records them as
// RefCommand effects (plugin/hooks/ref_ops.h). OFF by default; call BEFORE Init(). nvcoop.ini [hooks] story_capture=1 turns it on (hooks_module.cpp).
void setStoryCapture(bool on);
bool storyCaptureEnabled();
// The console executor injected with setConsoleExec (empty when none): RemoteApplier falls back to it when it has no executor of its own.
std::function<bool(const std::string&, uint32_t)> consoleExec();

// Counters for effects the engine produced that have no Effect type yet (e.g. CompleteQuest, container AddItem on a
// non-player). Exposed so tests/telemetry can show gaps instead of silently dropping them.
struct Unmapped {
    uint64_t questCompleted = 0;
    uint64_t itemOnNonPlayer = 0;
    uint64_t avOnNonPlayer = 0;
    uint64_t questFailed = 0;
    uint64_t storyDropped = 0;   // story-capture: a command ran but could not be recorded (calling reference or form without a stable name)
};
const Unmapped& unmapped();

}  // namespace nvhooks
