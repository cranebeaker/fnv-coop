// L12 world-state core (game-free, unit-tested). Everything here is plain C++ over synccore types; the engine glue lives
// in world_hooks.cpp / world_apply.cpp and only feeds this code.
//
// Mutation twins: FNVCOOP_MUT=<n> at run time (test builds only, tools/mut/fnvcoop_mut.h) switches on a deliberately broken variant; the test binary must fail for every n.
#pragma once
#include <cstdint>
#include <functional>
#include <map>
#include <optional>
#include <string>
#include <unordered_map>
#include <vector>

#include "engine_hooks.h"  // nvhooks::FormResolver (declaration only; game-free)
#include "fnvcoop/effect.h"
#include "world_effects.h"

#include "fnvcoop_mut.h"  // FNV_MUT(n): runtime mutation twins (env FNVCOOP_MUT=n); constant false in production builds

namespace nvworld {

// ---------------------------------------------------------------------------------------------------------------------
// 1. Created refs: host ref (FormRef "fnvcoop.created":0xFFxxxxxx) <-> this machine's local refID
// ---------------------------------------------------------------------------------------------------------------------
class CreatedRefMap {
public:
    void bind(const fnvcoop::FormRef& host, uint32_t localRefID);  // rebinding replaces both directions
    uint32_t local(const fnvcoop::FormRef& host) const;            // 0 = unknown
    bool hostOf(uint32_t localRefID, fnvcoop::FormRef* out) const;
    bool forget(const fnvcoop::FormRef& host);                     // true if it was bound
    size_t size() const { return toLocal_.size(); }
    const std::map<fnvcoop::FormRef, uint32_t>& all() const { return toLocal_; }
    void clear();

private:
    std::map<fnvcoop::FormRef, uint32_t> toLocal_;
    std::unordered_map<uint32_t, fnvcoop::FormRef> toHost_;
};

// FormResolver decorator: the engine resolver cannot name created (0xFF) refs. On the host a created ref keeps the engine id
// (identity); on a client the map translates in both directions. Everything else is delegated.
class WorldResolver : public nvhooks::FormResolver {
public:
    WorldResolver(nvhooks::FormResolver& inner, CreatedRefMap& map, bool hostSide) : inner_(inner), map_(map), host_(hostSide) {}
    bool toRef(uint32_t refID, fnvcoop::FormRef* out) override;
    uint32_t fromRef(const fnvcoop::FormRef& r) override;
    void* lookup(uint32_t refID) override { return inner_.lookup(refID); }

private:
    nvhooks::FormResolver& inner_;
    CreatedRefMap& map_;
    bool host_;
};

// ---------------------------------------------------------------------------------------------------------------------
// 1b. Effects that target created refs, and the queue that holds them until the local object exists
// ---------------------------------------------------------------------------------------------------------------------
// The created (0xFF) ref an effect acts on, if any. SpawnRef/DespawnRef themselves are not "targets".
bool createdTargetOf(const fnvcoop::Effect& e, fnvcoop::FormRef* out);
bool createdTargetOf(const WorldEffect& e, fnvcoop::FormRef* out);

struct HeldEffect {
    bool world = false;       // true: wfx is meaningful, else fx
    fnvcoop::Effect fx;
    WorldEffect wfx;
    fnvcoop::FormRef waitOn;  // created host ref (onCreated) or the missing form (retry)
    bool onCreated = false;   // released when `waitOn` gets bound; otherwise released by the retry tick
    int attempts = 0;
};

// What a client does with an effect before applying it: acting on a created ref that has no local object yet means "hold it".
enum class Route { Apply, HoldForCreated };
Route routeEffect(const fnvcoop::Effect& e, const CreatedRefMap& m, fnvcoop::FormRef* waitOn);
Route routeEffect(const WorldEffect& e, const CreatedRefMap& m, fnvcoop::FormRef* waitOn);
// After a failed apply: is it worth trying again later? (form not loaded yet, ref script instance not created yet)
enum class Outcome { Ok, Missing, Rejected, Other };
bool retriable(const fnvcoop::Effect& e, Outcome o);
bool retriable(const WorldEffect& e, Outcome o);

class DeferredQueue {
public:
    explicit DeferredQueue(size_t cap = 256) : cap_(cap) {}
    void hold(HeldEffect h);  // arrival order; when full the OLDEST is dropped (counted)
    // Created-ref waiters whose host ref is bound now, in arrival order (removed from the queue).
    std::vector<HeldEffect> takeBound(const CreatedRefMap& m);
    // Retry tick: every retry waiter (attempts++) is returned unless it has used up `maxAttempts` (then it expires). Created-ref
    // waiters are aged by the same tick and expire after 4x as many (their spawn never arrived).
    std::vector<HeldEffect> takeRetries(int maxAttempts);
    // The created ref was despawned: its held follow-ups are moot.
    size_t dropFor(const fnvcoop::FormRef& host);
    size_t size() const { return q_.size(); }
    size_t holds() const { return holds_; }  // total hold() calls; a flush uses it to tell "re-held" from "ran"
    size_t dropped() const { return dropped_; }
    size_t expired() const { return expired_; }
    void clear() { q_.clear(); }

private:
    std::vector<HeldEffect> q_;
    size_t cap_, dropped_ = 0, expired_ = 0, holds_ = 0;
};

// ---------------------------------------------------------------------------------------------------------------------
// 2. Spawn ledger: SpawnRef / DespawnRef bookkeeping on a client
// ---------------------------------------------------------------------------------------------------------------------
enum class SpawnAction { Create, Queued, Duplicate };
enum class DespawnAction { CancelledPending, DeleteLocal, Unknown };
struct DespawnResult {
    DespawnAction action = DespawnAction::Unknown;
    uint32_t localRefID = 0;  // DeleteLocal: the created ref to MarkForDelete (already forgotten by the ledger)
};

class SpawnLedger {
public:
    // cellReady: the spawn's cell is loaded here, so the object can be created now. Otherwise it is queued.
    // Duplicate: this host ref is already bound or queued (a replayed/late-join txn) -- never create twice.
    // (DespawnRef only ever names created refs: synccore's DespawnRef carries just the 0xFF id.)
    SpawnAction onSpawn(const fnvcoop::SpawnRef& s, bool cellReady);
    // The glue created the local object for `host`.
    void onCreated(const fnvcoop::FormRef& host, uint32_t localRefID) { map_.bind(host, localRefID); }
    DespawnResult onDespawn(const fnvcoop::DespawnRef& d);
    // Queued spawns for which `ready(spawn)` is true, in arrival order; they are removed from the queue.
    std::vector<fnvcoop::SpawnRef> takeReady(const std::function<bool(const fnvcoop::SpawnRef&)>& ready);
    size_t pending() const { return pending_.size(); }
    // A new world was loaded (save load / new game / back to menu): created refs of the old world no longer exist.
    void clear() {
        map_.clear();
        if (!FNV_MUT(27)) {
            pending_.clear();  // MUTANT 27: queued spawns of the old world survive a load
        }
        deferred_.clear();
    }
    CreatedRefMap& map() { return map_; }
    const CreatedRefMap& map() const { return map_; }
    // Effects held because they act on a created ref that is not bound yet (released by the glue after onCreated).
    DeferredQueue& deferred() { return deferred_; }
    const DeferredQueue& deferred() const { return deferred_; }
    // Rejoin: `live` is the host's current set of spawns (WorldBaseline). Created refs this client still holds that the host no
    // longer has (despawned while away) are forgotten and returned for deletion; queued spawns that are gone are cancelled.
    std::vector<uint32_t> reconcile(const std::vector<fnvcoop::SpawnRef>& live);

private:
    CreatedRefMap map_;
    std::vector<fnvcoop::SpawnRef> pending_;
    DeferredQueue deferred_;
};

// ---------------------------------------------------------------------------------------------------------------------
// 3. Ref script variables (set ref.var to x): change tracking with a per-poll budget
// ---------------------------------------------------------------------------------------------------------------------
class RefVarTracker {
public:
    // First sight of (ref,index) records the baseline and returns false (a ref's initial values come from the baseline
    // transfer, not from the change stream). Later calls return true when the value differs from the last REPORTED one
    // and the poll budget is not exhausted; a deferred change is reported by a later poll.
    bool observe(uint32_t refID, uint16_t index, double value);
    void beginPoll(size_t budget) { budget_ = budget; reported_ = 0; }
    size_t reportedThisPoll() const { return reported_; }
    void forgetRef(uint32_t refID);  // cell unloaded: the next sighting re-baselines
    size_t size() const { return last_.size(); }
    void clear() { last_.clear(); }

private:
    std::unordered_map<uint64_t, double> last_;
    size_t budget_ = 0, reported_ = 0;
};

// ---------------------------------------------------------------------------------------------------------------------
// 4. Weather / time-scale: host observes, emits only genuine, not-yet-sent changes
// ---------------------------------------------------------------------------------------------------------------------
class WeatherTimeSync {
public:
    void reset() { weather_.reset(); scale_.reset(); }
    // Host poll. weatherRefID 0 = unknown (interior): no effect. Returns effects to record (host order).
    std::vector<fnvcoop::Effect> poll(const fnvcoop::FormRef& weather, bool haveWeather, float timeScale);
    // A SetWeather/SetTimeScale already went out through the capture hub (script-driven): do not re-send it on the next poll.
    void noteSent(const fnvcoop::Effect& e);
    // Client side: should a LOCAL weather/time change be blocked? Only a host txn being applied may change them.
    static bool blockLocal(bool guest, bool applying);

private:
    std::optional<fnvcoop::FormRef> weather_;
    std::optional<float> scale_;
};

// ---------------------------------------------------------------------------------------------------------------------
// 5. Non-player actor values
// ---------------------------------------------------------------------------------------------------------------------
enum class AvCmdKind : uint8_t { Set = 0, Mod = 1, Force = 2, Damage = 3, Restore = 4 };
// before/after = the actor's current AV value around the command. Returns nullopt when nothing changed.
std::optional<SetActorValueRef> makeAvEffect(const fnvcoop::FormRef& actor, uint32_t av, AvCmdKind kind, float before, float after);
// Console name of an actor value code ("" if unknown).
const char* avName(uint32_t av);

// ---------------------------------------------------------------------------------------------------------------------
// 6. Console lines for applying world effects (run with the target as the calling reference where noted)
// ---------------------------------------------------------------------------------------------------------------------
std::string markForDeleteLine();                                     // on the ref
std::string setEssentialLine(const SetEssential& e, uint32_t baseOrRefID);  // refOnly: on the ref; else SetEssential <base> n
std::string setRestrainedLine(bool on);                              // on the ref
std::string setRelationLine(Relation kind, uint32_t factionA, uint32_t factionB);
std::string scriptPackageLine(bool add, uint32_t packageID);         // on the actor
std::string ignoreCrimeLine(bool on);                                // on the actor
std::string setPCEnemyOfFactionLine(uint32_t factionID, bool on);
std::string setOwnershipLine(uint32_t ownerID);                      // on the ref; 0 = no argument (the player owns it)
std::string clearOwnershipLine();                                    // on the ref
std::string setCellOwnershipLine(uint32_t cellID, uint32_t ownerID);
std::string setCellPublicFlagLine(uint32_t cellID, bool on);
std::string avLine(const SetActorValueRef& e);                       // on the actor; empty if the AV has no console name
std::string placeAtMeLine(uint32_t baseID, int count);               // on a reference in the target cell
std::string setPosLine(char axis, float v);

// Host: actors the host saw alive and then dead (a kill in combat, a fall, a trap: deaths no script command reports). A corpse that was
// already dead when first seen is not reported (the save has it dead on every machine). An actor that is not seen in a pass keeps its
// last state (it unloaded; it may die out of sight and is reported when it is seen dead). Game-free: the plugin feeds the loaded actors.
class DeathWatch {
public:
    // `seen`: (refID, dead) for the actors loaded now. Returns the refIDs that died since a pass saw them alive.
    std::vector<uint32_t> update(const std::vector<std::pair<uint32_t, bool>>& seen);
    void reset() { alive_.clear(); }
    size_t tracked() const { return alive_.size(); }

private:
    std::unordered_map<uint32_t, bool> alive_;
};

// ---------------------------------------------------------------------------------------------------------------------
// 7. WorldLog: lane-local effect buffer for the effects L4 has no type for yet (host drains, clients apply)
// ---------------------------------------------------------------------------------------------------------------------
class WorldLog {
public:
    void push(const WorldEffect& e);       // coalesces: a second write to the same (actor, av, Set) / essential / restrained replaces the first
    std::vector<WorldEffect> drain();
    size_t size() const { return q_.size(); }
    uint64_t repeats() const { return repeats_; }   // state writes dropped because they repeated the last value pushed

private:
    std::vector<WorldEffect> q_;
    std::vector<WorldEffect> sent_;   // the last value pushed per state key (never actor values)
    uint64_t repeats_ = 0;
};

// ---------------------------------------------------------------------------------------------------------------------
// 8. WorldBaseline: host-side NET state of this lane's effects, for a client that joins or rejoins
// ---------------------------------------------------------------------------------------------------------------------
struct BaselineSnapshot {
    std::vector<fnvcoop::Effect> effects;  // SpawnRef (creation order), then the last RefScriptVar of every ref/var seen
    std::vector<WorldEffect> world;        // net NPC / faction state, folded (a Set replaces earlier deltas, deltas of one kind add up)
};
class WorldBaseline {
public:
    // Feed every effect the host emits (all txns); only SpawnRef / DespawnRef / RefScriptVar matter here.
    void onEffect(const fnvcoop::Effect& e);
    void onWorld(const WorldEffect& e);
    BaselineSnapshot snapshot() const;
    // Host ids of spawns that are alive (the host polls their script variables even when the player is elsewhere).
    std::vector<fnvcoop::FormRef> liveSpawnRefs() const;
    size_t spawns() const { return spawns_.size(); }
    size_t worldEntries() const { return world_.size(); }
    void clear();

private:
    void purge(const fnvcoop::FormRef& createdRef);
    std::vector<fnvcoop::SpawnRef> spawns_;
    std::map<std::pair<fnvcoop::FormRef, uint16_t>, double> vars_;
    std::vector<WorldEffect> world_;
};

// JSON for WorldEffect: {"type":"SetEssential","actor":"falloutnv.esm:104C79","essential":true,"refOnly":false}
std::string worldEffectToJson(const WorldEffect& e);
bool worldEffectFromJson(const std::string& json, WorldEffect* out, std::string* err);

}  // namespace nvworld
