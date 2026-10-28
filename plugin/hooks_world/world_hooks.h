// L12 game-side glue (x86 only, compiled against xNVSE headers): world-state capture on the host, apply on clients.
// Like L5's hooks, NOTHING happens until Init() is called (module does it at DeferredInit).
//
// Scope (see internal lane notes for the full table and which part is L5's):
//   host   : MarkForDelete -> DespawnRef, SetEssential/SetActorRefEssential/SetRestrained/SetEnemy/SetAlly/RestoreActorValue
//            -> WorldLog (lane-local effects until L4 lands types), ref script var poll -> RefScriptVar, weather/time poll
//            -> SetWeather/SetTimeScale.
//   client : WorldApplier::apply() for SpawnRef / DespawnRef / RefScriptVar / WorldEffect; queued spawns flush on cell load.
#pragma once
#include <functional>
#include <memory>
#include <string>
#include <vector>

#include "apply_remote.h"
#include "engine_hooks.h"
#include "world_core.h"
#include "world_refvars.h"
#include "world_plan.h"

namespace nvworld {

struct HookReport {
    std::string name;
    nvhooks::PatchStatus status;
};
struct InitReport {
    std::vector<HookReport> hooks;
    int ok = 0;
    int failed = 0;
};

// The resolver the whole plugin should use (pass it to nvhooks::Init): defaultResolver + created-ref translation.
// `hostSide`: created refs keep their engine id (identity); a client translates through the spawn ledger.
nvhooks::FormResolver& makeResolver(bool hostSide);
SpawnLedger& ledger();

// Installs the new-handler detours. Idempotent; a hook whose prologue differs is skipped and reported.
InitReport Init(nvhooks::CaptureHub& hub, nvhooks::Role role);
void Shutdown();
int auditTamperedHooks();  // hot reload preflight: our hooks that another patcher chained over (0 = safe to unload)
// A different world is now loaded (PostLoadGame / NewGame / ExitToMainMenu): forget created refs, queued spawns, var and weather baselines.
void resetSession();

// Local (not-yet-L4) world effects recorded since the last drain, host order.
std::vector<WorldEffect> drainWorldEffects();
// Host: net spawn / ref-var / NPC state for a client that joins or rejoins (see WorldBaseline). Apply with WorldApplier::applyBaseline.
BaselineSnapshot hostBaseline();
// Called by the NPC AV hooks (here and L5's observer): decides + records.
void recordNonPlayerAv(const fnvcoop::FormRef& actor, uint32_t av, AvCmdKind kind, float before, float after);
void setWorldEffectSink(std::function<void(const WorldEffect&)> sink);
// Every outermost txn the host captures, seen BEFORE the coop runtime's sink takes its effects (dll-wiring: spawn arbitration needs the host's
// PlaceAtMe SpawnRef with its cause). Read-only. null = off.
void setTxnObserver(std::function<void(const fnvcoop::EffectTxn&)> observer);

// Host tick (call once per frame outside scripts; does its own throttling): polls weather/time-scale and watched ref script
// variables and records them as ordinary txns (cause Script).
void hostTick();
// Refs whose script variables are polled in addition to the player's current cell (quest-critical NPCs/objects).
void watchRef(const fnvcoop::FormRef& r);
void clearWatched();
// Poll schedule (ini [world] refvar_fast_every / refvar_sweep_every / refvar_budget, read by the module).
void setRefVarPollConfig(const RefVarPollCfg& cfg);
struct RefVarPollStats {
    uint64_t sweeps = 0, fastReads = 0;
    size_t tracked = 0, pending = 0, watched = 0;
};
RefVarPollStats refVarPollStats();
// The script of this reference just ended (the script-run observer calls this, see docs/handover/P5-requests.md): its variables are read now, so
// a flag set and cleared by two runs inside one frame is two values, not none. Only watched references are read (every script run would be too many).
void noteRefScriptRan(uint32_t refID);
size_t pollRefVarsNow(std::vector<fnvcoop::RefScriptVar>* out);  // test/diagnostic: one full pass, returns number of changes

// Client side. Applies SpawnRef/DespawnRef/RefScriptVar and WorldEffects without running scripts.
class WorldApplier {
public:
    WorldApplier(nvhooks::CaptureHub& hub, nvhooks::FormResolver& res) : hub_(hub), res_(res) {}
    void setExecLine(std::function<bool(const std::string&, uint32_t)> f) { exec_ = std::move(f); }
    // Effects of every lane go through here: world-lane effects are applied by this class, all others by `fallback` (L10 sets it to
    // L5's RemoteApplier::apply, which must share the world resolver so created-ref targets are translated). An effect on a
    // created ref that does not exist locally yet is HELD and replayed once the spawn is created (dropped on despawn); one whose
    // form / script instance is not there yet is retried by tick(true). A held effect reports Ok.
    void setFallback(std::function<nvhooks::ApplyStatus(const fnvcoop::Effect&)> f) { fallback_ = std::move(f); }
    nvhooks::ApplyStatus applyAny(const fnvcoop::Effect& e);
    nvhooks::ApplyStatus apply(const fnvcoop::Effect& e);  // == applyAny
    nvhooks::ApplyStatus applyWorld(const WorldEffect& e);
    // Join / rejoin: created refs the host no longer has are deleted, then every baseline effect is applied (idempotent).
    size_t applyBaseline(const BaselineSnapshot& b);
    // Create queued spawns whose cell has loaded, release held follow-ups of created refs; retry=true also retries the waiters for
    // forms that were not loaded yet (call about every 2 s, and on cell load with retry=false). Returns how many were applied.
    size_t tick(bool retry);
    size_t flushQueued();  // == tick(false)
    std::string lastError() const { return err_; }

private:
    nvhooks::ApplyStatus run(const Plan& p);
    int gateSteps_ = 0;   // > 0: run() holds a ScriptGate around every step that is not a console line (see stepRunsScripts)
    nvhooks::CaptureHub& hub_;
    nvhooks::FormResolver& res_;
    std::function<bool(const std::string&, uint32_t)> exec_;
    std::function<nvhooks::ApplyStatus(const fnvcoop::Effect&)> fallback_;
    std::string err_;
};

// ---- world-time lane (plugin/world_time) ----
// The host's wait / sleep reaches a client as the AdvanceTime effect; the world-time module owns the game clock and installs this.
// Return false to report the effect as not applied. Without a handler an AdvanceTime effect is Unsupported (module off).
void setAdvanceClockHandler(std::function<bool(uint32_t minutes)> f);
// Every ObjectState this machine learns about: a host capture (script commands, the world-time poll) and, on a client, every one received
// from the host BEFORE it is applied (so the book holds the host's word even for objects that are not loaded yet).
void setObjectStateObserver(std::function<void(const fnvcoop::ObjectState&)> f);
// Placed doors / containers / terminals of the player's cell, `max` of them from index `start` on (a round robin); *total = how many exist.
struct WorldObject {
    uint32_t refID = 0;
    uint32_t baseType = 0;  // form type of the BASE form (door 0x1C-ish ids come from xNVSE's enum, not from this header)
    int kind = 0;           // 0 door, 1 container, 2 terminal
    std::string ref;
};
std::vector<WorldObject> objectsInPlayerCell(size_t start, size_t max, size_t* total);
struct ObjectReading {
    bool ok = false;
    int openState = 0;  // GetOpenState: 1 open, 2 opening, 3 closed, 4 closing (doors only; 0 otherwise)
    bool locked = false;
    int lockLevel = 0;
    bool destroyed = false;
};
ObjectReading readObject(const WorldObject& o);
// The engine's own IsInCombat for a reference (the host's player is 0x14). False when the evaluator cannot be called.
bool queryInCombat(uint32_t refID, bool* out);
bool refToString(uint32_t refID, fnvcoop::FormRef* out);   // resolver.toRef with the created-ref rule

// Raw memory read for address verification (bridge world.peek, defined in world_peek.cpp). SEH-protected; false when unreadable.
bool peekMemory(uint32_t addr, void* out, size_t n);

// ---- diagnostics used by the bridge methods and the in-game proofs ----
struct RefProbe {
    bool exists = false;
    uint32_t refID = 0, baseID = 0, cellID = 0, flags = 0;
    float x = 0, y = 0, z = 0;
    int nvars = -1;  // -1: no script variable list
};
bool probeRef(uint32_t refID, RefProbe* out);
struct ScriptedRef {
    uint32_t refID = 0, baseID = 0;
    std::string ref;
    int nvars = 0;
};
std::vector<ScriptedRef> scriptedRefsInPlayerCell(size_t max);
bool getRefVar(uint32_t refID, uint16_t index, double* out);
bool setRefVarDirect(uint32_t refID, uint16_t index, double value);  // test helper (what `set ref.var to` writes)
bool currentWeather(fnvcoop::FormRef* out);                          // false: unknown (interior / accessor drifted)
float currentTimeScale();
uint32_t playerCellID();
// Query a condition function of the engine on a reference (the same evaluators a quest condition uses; test/diagnostic only).
// fn: GetRestrained | IsEssential | GetActorValue(p1 = AV code) | GetFactionRelation(p1 = faction refID).
bool queryCondition(const std::string& fn, uint32_t refID, uint32_t p1, double* out);
struct ActorRef {
    uint32_t refID = 0, baseID = 0;
    std::string ref;
};
// formType 0x3B = characters (default), 0x1B containers, 0x1C doors.
std::vector<ActorRef> actorsInPlayerCell(size_t max, uint32_t formType = 0x3B);

}  // namespace nvworld
