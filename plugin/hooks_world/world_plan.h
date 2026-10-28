// Apply planning for world effects (game-free, unit-tested): turns an Effect into the list of engine steps a client must
// run. The glue (world_apply.cpp) only executes steps; every decision (which command, on which ref, dedupe, queueing) is here.
#pragma once
#include <functional>
#include <string>
#include <vector>

#include "world_core.h"

namespace nvworld {

enum class StepKind : uint8_t {
    ConsoleOnRef,   // run `line` with `refID` as the calling reference
    ConsoleGlobal,  // run `line` with no calling reference
    CreateSpawn,    // create `baseID` at `spawn` (cell/pos); the glue then calls SpawnLedger::onCreated(spawn.ref, newRefID)
    SetRefVar,      // write script variable `varIndex` of ref `refID`
    AdvanceClock,   // the host's wait / sleep reached this machine: `minutes` of game time (the world-time module owns the clock)
};
// A console line IS a script: the engine runs it through Script::Execute, which an open nvhooks::ScriptGate answers with "pretend it ran". So a step that
// runs a console line must never run while a ScriptGate is open (the lab's door: a host-native open reached the client as an ObjectState whose
// "SetOpenState 1" was swallowed by the gate for ever, and so were the client's heals toward it). The other steps keep the gate: they must not run scripts.
bool stepRunsScripts(StepKind k);
struct Step {
    StepKind kind = StepKind::ConsoleGlobal;
    uint32_t refID = 0;
    std::string line;
    fnvcoop::SpawnRef spawn;
    uint32_t baseID = 0, cellID = 0;
    uint16_t varIndex = 0;
    double varValue = 0;
    uint32_t minutes = 0;
};

enum class PlanStatus : uint8_t { Ok, FormMissing, Queued, Unsupported };
struct Plan {
    PlanStatus status = PlanStatus::Ok;
    std::vector<Step> steps;
    std::string error;
};
const char* planStatusName(PlanStatus s);

struct PlanEnv {
    nvhooks::FormResolver& res;
    SpawnLedger& ledger;
    // Is the spawn's cell loaded on this machine right now? (Interiors: the player is inside it. Exteriors: it is in the grid.)
    std::function<bool(const fnvcoop::FormRef& cell)> cellReady;
};

// Effects of this lane: SpawnRef, DespawnRef, RefScriptVar, ObjectState (world-time lane: observed on its way in), AdvanceTime (the clock),
// RequestSave (acknowledged and ignored: only the host's save is the world). Anything else: Unsupported (L5's RemoteApplier owns it).
Plan planEffect(const fnvcoop::Effect& e, PlanEnv& env);
Plan planWorldEffect(const WorldEffect& e, PlanEnv& env);
// Spawns that were queued for a cell that has loaded since: one Plan per spawn, in arrival order.
std::vector<Plan> planQueuedSpawns(PlanEnv& env);

// True for effect types this lane applies.
bool handlesEffect(const fnvcoop::Effect& e);

// Rejoin: delete steps for created refs this client still has that the host no longer lists (see SpawnLedger::reconcile).
Plan planReconcile(const std::vector<fnvcoop::SpawnRef>& live, PlanEnv& env);

// ---- routed apply: ordering, remapping and retry around the plans (game-free; the glue supplies the executors) ----
using RunPlan = std::function<Outcome(const Plan&)>;                  // execute the steps of a plan
using FallbackApply = std::function<Outcome(const fnvcoop::Effect&)>;  // effects this lane does not own (L5's RemoteApplier)
// Any effect from the host. An effect on a created ref that has no local object yet is HELD until the spawn is created (or
// dropped when the ref is despawned); one that fails because a form / script instance is not there yet is held for the retry
// tick. A held effect reports Ok. Applying a SpawnRef releases its held follow-ups.
Outcome applyRouted(const fnvcoop::Effect& e, PlanEnv& env, const RunPlan& run, const FallbackApply& other, int attempts = 0);
Outcome applyWorldRouted(const WorldEffect& e, PlanEnv& env, const RunPlan& run, int attempts = 0);
// Release held effects: those whose created ref is bound now, plus (retryTick) the retry waiters. Returns how many were applied.
size_t flushDeferred(PlanEnv& env, const RunPlan& run, const FallbackApply& other, bool retryTick, int maxAttempts = 30);

}  // namespace nvworld
