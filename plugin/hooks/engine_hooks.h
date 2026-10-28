// Game-side glue (x86 only, compiled against xNVSE headers): installs the prologue-checked detours and feeds the
// CaptureHub. NOTHING happens until Init() is called by plugin main (L1) -- including on the first load, so a build
// that links this library is inert by default.
//
// Threading: Init/Shutdown/hooks all run on the game main thread (script execution is single-threaded).
#pragma once
#include <functional>
#include <map>
#include <string>
#include <vector>

#include "capture_hub.h"
#include "hook_patch.h"
#include "info_scope.h"
#include "party_cond.h"
#include "per_slot_run.h"
#include "player_facts.h"
#include "player_kill.h"
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



// Script::Execute accounting (proof that replaying a txn ran no result script).
struct ScriptStats {
    uint64_t gated = 0;             // calls swallowed by a closed ScriptGate
    uint64_t ran = 0;               // calls that executed
    uint64_t ranWhileApplying = 0;  // subset of `ran` during a RemoteApplier::apply (must be 0 with the gate on)
    uint64_t localRan = 0;          // GuestLocalRun allow-listed scripts that executed (subset of `ran`)
    uint64_t applierLines = 0;      // console lines the applier itself executed (kill/resurrect/move); not counted above
    uint64_t questVarRollbacks = 0; // guest: shared quest vars an object / NPC / effect script wrote and that were put back (host is authoritative)
    uint64_t questVarGuarded = 0;   // guest: executions of scripts known to write a shared quest var
    uint64_t stagesBlocked = 0;     // guest: SetStage calls a running script / console line made and the suppression dropped
    uint64_t stagesForwarded = 0;   // ... of which the stage request observer sent to the host (the host decides; docs/protocol.md section 19, Kind SetStage)
};
const ScriptStats& scriptStats();
// Player facts (P3 handover R1): while a PlayerFactsScope is alive, `player.GetItemCount` / `GetAV` / `HasPerk` / `GetEquipped` (the script commands AND the
// condition evaluators of the same names) answer from these facts instead of the host's own player, for a fact the vector holds; everything else
// goes to the engine. P4's IntentService::runDialogueResult wraps RunResultScript in one with the Pick's facts (converted field by field from
// fnvcoop_dlg::Fact). The vector must outlive the scope.
void setPlayerFacts(const std::vector<PlayerFact>* facts);
class PlayerFactsScope {
public:
    explicit PlayerFactsScope(const std::vector<PlayerFact>* facts);
    ~PlayerFactsScope();
    PlayerFactsScope(const PlayerFactsScope&) = delete;
    PlayerFactsScope& operator=(const PlayerFactsScope&) = delete;

private:
    const std::vector<PlayerFact>* prev_;
};

// Script-run observer (P5 request 1, G15 same-frame flips): called by the Script::Execute detour AFTER the engine ran a script whose calling reference
// (thisObj) is not null, with that reference's runtime id. hooks_world registers noteRefScriptRan here. One null check and one call per script run.
using ScriptRunObserver = std::function<void(uint32_t thisRefID)>;
void setScriptRunObserver(ScriptRunObserver obs);
// Script-run scope (W3a item 4): called by the Script::Execute detour right BEFORE the engine runs a script (before = true) and right AFTER it returned (before = false),
// with the script form's runtime id (a quest, object, effect or INFO result script; never a skipped / gated one). The session uses it to put the acting player's view of the
// companion "hired" flags into the shared quest variables for the length of a guest's run, except for scripts that read them party-wide (companion_sets.txt view-party).
// One null check per script run when unset.
using ScriptRunScopeHook = std::function<void(uint32_t scriptRefID, bool before)>;
void setScriptRunScope(ScriptRunScopeHook hook);
// W3a item 5 (P6 request 1): called by the SetPlayerControls hook (Disable / EnablePlayerControls) while a GUEST is the acting player (actingSlot() != 0), inside the open
// capture scope, with that slot and whether it is a lock (disable) or a release. true = the lock/unlock concerns that guest's player only (rules scope=initiator): the host's own engine call is skipped, so the host is
// never locked for a guest's scene. Unset or false = the engine call runs (the rules engine's host effect gives the host its bits back when needed).
using ControlsRouter = std::function<bool(uint8_t actingSlot, bool disable)>;
void setControlsRouter(ControlsRouter router);
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

// Player control lock (v1 for the baseline transfer): DisablePlayerControls / EnablePlayerControls (0x5CAE30 / 0x5CAD20) with all
// seven flags, issued as console lines through the executor injected by the plugin (hooks_module sets it). Idempotent: only the
// transitions run. Unlock re-enables ALL flags (it cannot know what a cutscene had disabled before).
void setConsoleExec(std::function<bool(const std::string&, uint32_t refID)> f);  // refID 0 = no calling reference
void SetPlayerControlsLocked(bool locked);
bool playerControlsLocked();

// Host: let the rules engine defer a SetStage that the engine is about to perform (not ours, not while applying, not inside a
// StageGateBypass). `quest` is the engine refID (resolver().toRef() converts). Return false = the engine SetStage is skipped and
// the detour returns false. The owner re-issues the stage later under a StageGateBypass.
// Guest: a script or console line called SetStage and the suppression dropped it (SuppressPolicy::blockMutation). The observer gets the quest and the stage
// so the intents layer can ask the host to advance it. Return true when the request went out. Unset = the drop stays silent (counted in stagesBlocked).
using StageRequestObserver = std::function<bool(const fnvcoop::FormRef& quest, uint32_t stage)>;
void setStageRequestObserver(StageRequestObserver obs);
// Host: how many SetStage calls the stage gate has held back so far (engine counter; a guest's SetStage intent compares it before / after).
uint64_t stageGateDeferred();

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
// Local read of every objective of a quest: objective id -> status bits (bit0 displayed, bit1 completed). False if the quest is unknown.
bool localQuestObjectives(const fnvcoop::FormRef& quest, std::map<uint32_t, uint8_t>& out);
// Local read of one global's value (TESGlobal::data). False if the form is unknown or not a global.
bool localGlobalValue(const fnvcoop::FormRef& global, double& out);
// Execute a console line through the injected executor (for the owner of a deferred stage: "setstage <hex> <n>").
bool execConsoleLine(const std::string& line);
// Same, with `refID` as the console's calling reference (so `KillActor`, `MoveTo ...`, `GetDead` act on that reference).
bool execConsoleLineOn(const std::string& line, uint32_t refID);
// INFO result scripts the engine started for a spectator's Say and that were skipped (the host's INFO result runner already ran them).
uint64_t spectatorSayBlocked();

// Diagnostic: every ScriptLocal (id, value) of a quest's ScriptEventList exactly as pollHostState sees it (empty if the quest has none).
std::vector<std::pair<uint32_t, double>> questVarsDump(const fnvcoop::FormRef& quest);
// Quest script variables by NAME (G24: the rules engine's per-player variables are named, the engine's ScriptEventList is indexed). questVarName: the name of
// variable index `idx` (what a condition view gets from the engine); questVarValue / setQuestVarValue read / write the live value for the acting-player copies
// (RulesEngine::beginActing / endActing). False = no such quest, script or variable. The write is a plain store: it is NOT captured as an effect.
bool questVarName(const fnvcoop::FormRef& quest, uint32_t idx, std::string* name);
bool questVarValue(const fnvcoop::FormRef& quest, const std::string& name, double* out);
bool setQuestVarValue(const fnvcoop::FormRef& quest, const std::string& name, double value);

// While alive, nvhooks::actingPlayer() returns `actorRefID` (a guest's ghost) instead of the host player 0x14.
class ActingPlayerScope {
public:
    // forcedSlot >= 0: the window is for that party slot even when actorRefID is the host's own player (a guest with no puppet: its script runs with the host's player standing in).
    explicit ActingPlayerScope(uint32_t actorRefID, int forcedSlot = -1);
    ~ActingPlayerScope();
    ActingPlayerScope(const ActingPlayerScope&) = delete;
    ActingPlayerScope& operator=(const ActingPlayerScope&) = delete;
};
// While alive, a `player.RemoveItem` is the removal of the guest in `slot` (a guest's dialogue result runs on the host): the host's own pack is not touched, the removal is recorded
// for the rules and the other machines (guest_item.h).
class GuestItemScope {
public:
    explicit GuestItemScope(uint8_t slot);
    ~GuestItemScope();
    GuestItemScope(const GuestItemScope&) = delete;
    GuestItemScope& operator=(const GuestItemScope&) = delete;
};
uint32_t actingPlayer();
// Party slot of actingPlayer(): 0 = host player; guests' ghosts are registered by the session (L6/L10) when they join.
void registerPlayerSlot(uint32_t ghostRefID, uint8_t slot);
uint8_t actingSlot();
// Forget every registered ghost / puppet (the table holds 8 and never forgets one; after 8 distinct refs actingSlot() would fall back to 0). Call it when the
// party is rebuilt, e.g. on PostLoadGame, then register the live ones again.
void clearPlayerSlots();

// Party-aware condition evaluation (party_cond.h). The provider lists the PRESENT party members (the local player as refID 0x14 + every
// other member's ghost reference); the reducer is RulesEngine::reduceCondition (values per member -> one answer). Both unset = vanilla.
using PartyProvider = std::function<std::vector<PartyMember>()>;
void setPartyConditions(PartyProvider provider, ConditionReducer reducer);
// Per-member answers a ghost reference cannot give (HasPerk, GetEquipped, IsSneaking, GetHasNote, IsPlayerInRegion, and the owner of a
// GetPlayerTeammate NPC): see party_cond.h MemberView. Unset = those members are skipped and the local player's answer stands.
// P6 call site: setPartyStateProvider(...) next to setPartyConditions(...) in the session wiring.
void setPartyStateProvider(StateProvider provider);
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
// A note reached THIS machine's journal by itself (a pickup, the Pip-Boy), outside a script and not through an applied txn, on a machine that does
// not capture (a guest). The session forwards it to the host as an AddNote so the host's HasNote / GetHasNote sees it (G10). The host captures the
// same event itself (an AddNote effect in its own txn).
using NoteObserver = std::function<void(const fnvcoop::FormRef& note)>;
void setNoteObserver(NoteObserver obs);

// `player.KillActor` (or the ghost of a guest as the target): ONE player dies, not the party (G42). The kill is captured as KillActor{<that player's own ref>}:
// the player ref for the host's own player, `setPlayerRefProvider(slot)` for a guest. The rules engine's PerPlayerState rule (bomb collar, cloud, gas trap)
// rewrites it into a Health drain addressed to that slot (RulesEngine::Options::playerRef is the same mapping); a quest without the rule keeps the shared kill.
// The engine kill runs only on the machine whose own player it concerns (the host's player: slot 0); a kill meant for a guest is skipped on the host.
// The observer, when registered, is told as well: `fn(slot, onHost)`; it is a notification (a log line, a downed state), not the way the guest dies.
// Neither registered = the old behaviour (KillActor{operand} recorded, engine kill run).
using PlayerKillObserver = std::function<void(uint8_t slot, bool onHost)>;
void setPlayerKillObserver(PlayerKillObserver obs);
void setPlayerRefProvider(PlayerRefOf f);

// Gear that moves between a PLAYER and a container (G35): `player.RemoveAllItems [container]` (intoPlayer = false) and
// `container.RemoveAllItems player` (intoPlayer = true). `container` is empty when the script named none. `items` is what the host's own player /
// the container held just before (absolute counts); it is empty when the event is for a guest, whose machine reads its own gear when it performs
// the move on request. The original command is skipped for a guest (the host's player is not the guest's). Unset = players are not captured
// (the old behaviour: only non-player containers are recorded, as a RemoveAllItems effect).
struct PlayerStashEvent {
    bool intoPlayer = false;
    uint8_t slot = 0;
    fnvcoop::FormRef container;
    bool retainOwnership = false;   // the command's second parameter ("Ownership Added")
    std::vector<std::pair<fnvcoop::FormRef, int32_t>> items;
    // `player.RemoveAllTypedItems container [ownership] [noMessage] type [holdoutList]` (the Dead Money failure ending, the Fort's 131 calls): only the items of that
    // form type (TESObjectWEAP = 40, TESObjectARMO = 24 ...) move, the holdout list stays on the player. typeFilter < 0 = every item (RemoveAllItems).
    int32_t typeFilter = -1;
    bool noMessage = false;
    fnvcoop::FormRef holdout;       // empty = none
};
using PlayerStashObserver = std::function<void(const PlayerStashEvent&)>;
void setPlayerStashObserver(PlayerStashObserver obs);

// W3d item 1: scripts that run once per joined slot (per_slot_run.h). `scripts` are the ones the rules declare (RulesEngine::perPlayerScripts), `runner` calls back
// once per joined guest with a puppet (the session brackets each call with the slot's quest copies). Both empty / null = the engine's own single run, as before.
// Host only. Replaces the previous table.
void setPerSlotScripts(std::vector<PerSlotScript> scripts, PerSlotRunner runner);
// The per-instance, per-slot object variable copies of the re-runs. They travel with the host's save game (plugin/coop/objvar_sidecar.h, the `.fnvobjvars` sidecar).
ObjVarCopies& objVarCopies();
struct PerSlotStats {
    uint64_t runs = 0;        // re-runs done (one per script run per guest slot)
    uint64_t objVarSwaps = 0; // runs that swapped object variable copies
};
const PerSlotStats& perSlotStats();

// W3d item 1 (W3b request 1): GetItemCount of ONE player, for RulesEngine::Options::itemCount (pooled hand-ins): the engine's own evaluator, without the party
// reduction, run on `whoRefID` (0x14 = the host's player, else a guest's puppet). False = the item or the reference is not loaded.
bool playerItemCount(uint32_t whoRefID, const fnvcoop::FormRef& item, int64_t* count);

// W3d item 2 (W3b request 6): INFO -> quest for the conditions of a dialogue menu. While the engine evaluates one INFO's conditions (TESTopicInfo::CheckConditions)
// the party conditions are reduced under that quest's PartyAwareCond rule (the Jed weight gate, the Shaman's roots). Unset = no scope, the old behaviour.
void setInfoQuestScopes(InfoQuestScopes::Lookup lookup);
uint64_t infoScopesOpened();

// Kill attribution (G04, hook half). The engine keeps the killer of a dead actor at actor+0xC0 (Actor::Kill 0x89D900 stores it; the IsKiller
// condition evaluator 0x5A3F00 compares its first parameter with it). Both only know the ONE local player, so a kill by a guest has to name that
// guest's ghost reference to make `IsKiller player` come out true for the party (the condition hook above asks once per member).
//   killWithAttribution: runs `KillActor <killer>` on the victim, so the engine itself stores the killer before it raises OnDeath (preferred).
//   setKillerOf: writes actor+0xC0 only (a victim that is already dead).
bool killWithAttribution(uint32_t victimRefID, uint32_t killerRefID);
bool setKillerOf(uint32_t victimRefID, uint32_t killerRefID);
uint32_t killerOf(uint32_t victimRefID);   // 0 = none / unknown

// Non-player actor values (SetActorValue/ModActorValue/ForceActorValue/DamageActorValue on an actor that is not the player), reported after the
// original ran with the actor value before/after, only while capturing. kind: 0 set, 1 mod, 2 force, 3 damage. The observer turns it into
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
