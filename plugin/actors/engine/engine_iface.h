// plugin/actors/engine: everything the actor port needs from the running game, behind ONE interface (game-free header).
//   * production : RealActorEngine (engine_real.cpp, x86, xNVSE layouts + SEH-guarded reads) wrapped by ScriptActorEngine, which turns
//                  every write into one console line run with the actor as the calling reference (the bridge's exec{cmd, ref} path)
//   * tests      : FakeActorEngine (tests/fake_engine.h), a small world with a scripted "native AI"
// All calls happen on the game main thread. A write returns true on success; false is an ORDINARY event (ref gone, engine refused,
// console parse error) that the caller counts and retries or ignores, never an exception.
#pragma once
#include <cstdint>
#include <string>
#include <vector>

#include "actors/engine/actor_engine_api.h"

namespace nvcoop::actors::engine {

// ---- actor values we mirror (xNVSE GameAPI.h eActorVal_*, FalloutNV.exe 1.4.0.525) ---------------------------------------------------
constexpr uint8_t kAvActionPoints = 12, kAvHealth = 16, kAvFatigue = 22;
constexpr uint8_t kAvHead = 25, kAvTorso = 26, kAvLeftArm = 27, kAvRightArm = 28, kAvLeftLeg = 29, kAvRightLeg = 30, kAvBrain = 31;
// Published by the owner on every sample. Limb conditions are the crippling state; Health decides death.
const std::vector<uint8_t>& watchedAvs();
// The name the console's ActorValue parameter accepts (strings found in FalloutNV.exe at 0x1066110..0x1066258). nullptr = not mirrored.
const char* avConsoleName(uint8_t av);

struct CellInfo {
    bool known = false;
    bool exterior = false;
    uint32_t world = 0;     // worldspace form id (exterior)
    int32_t x = 0, y = 0;   // grid coordinates (exterior)
};

// One read of a loaded actor reference. Every field has a safe default; `valid` false means the ref is not (or no longer) a loaded actor.
struct ActorRaw {
    bool wantRestrained = false;  // INPUT: set before read() to get `restrained` (one condition evaluation per call, so scans leave it off)
    bool valid = false;
    RefId ref = 0;
    uint32_t baseId = 0;          // runtime id of the base form the ref uses now (after a leveled roll: the rolled NPC / creature)
    bool isPlayer = false;
    bool disabled = false;        // ref is disabled (not in the world)
    bool deleted = false;         // marked for delete
    bool has3D = false;
    CellId cell = 0;              // parent cell form id (0 = none)
    live::Vec3 pos, rot;          // rot in radians, as stored in the ref
    uint32_t lifeState = 0;       // Actor+0x108: 0 alive, 1 dying, 2 dead (Actor::IsDead treats 1 and 2 as dead)
    bool baseEssential = false;   // base form ACBS flag "essential"
    bool baseUnique = false;      // base form ACBS flag "unique" (a named story NPC / creature)
    // BaseProcess (needs a high process; all false/0 otherwise)
    bool haveProc = false;
    bool weaponOut = false, aiming = false, swimming = false;
    bool inCombat = false;        // has a combat target
    RefId combatTarget = 0;
    uint32_t package = 0;         // current AI package form id (runtime id; 0 = none)
    bool restrainedKnown = false;
    bool restrained = false;      // GetRestrained condition value (set only when restrainedKnown)
    bool haveMove = false;
    uint32_t moveFlags = 0;       // ActorMover flags; bit 9 = sneaking (xNVSE header comment), see sneakFromMoveFlags
    bool dead() const { return lifeState == 1 || lifeState == 2; }
    bool created() const { return (ref >> 24) == 0xFF; }
};
inline bool sneakFromMoveFlags(uint32_t f) { return (f & 0x200u) != 0; }

class IActorEngine {
public:
    virtual ~IActorEngine() = default;

    // ---- reads ----------------------------------------------------------------------------------------------------------------
    virtual bool worldReady() = 0;                     // a game is loaded and the player ref exists (false in menus / loading)
    virtual uint32_t worldEpoch() = 0;                 // changes on every load, new game and return to the main menu
    virtual RefId playerRef() = 0;
    virtual bool read(RefId ref, ActorRaw& out) = 0;   // false = not a loaded actor (`out` is then default). Honour out.wantRestrained.
    // Cheap read of where a reference is (called for every non-owned actor every frame). Default: a full read().
    virtual bool readPose(RefId ref, CellId& cell, live::Vec3& pos, float& rotZ) {
        ActorRaw r;
        if (!read(ref, r)) return false;
        cell = r.cell; pos = r.pos; rotZ = r.rot.z;
        return true;
    }
    virtual bool actorValue(RefId ref, uint8_t av, float& out) = 0;     // current value
    // Actor refs of the loaded world: the player's cell and attached neighbours (high / middle-high process lists), at most `max`.
    virtual void loadedActors(std::vector<RefId>& out, size_t max) = 0;
    virtual bool cellLoaded(CellId cell) = 0;          // attached (its refs exist in memory)
    virtual bool cellInfo(CellId cell, CellInfo& out) = 0;
    virtual bool toFormRef(uint32_t formId, fnvcoop::FormRef& out) = 0;     // runtime id -> (plugin, local id); false for created ids
    virtual uint32_t fromFormRef(const fnvcoop::FormRef& r) = 0;            // 0 = plugin / form not loaded
    // A reference inside `cell` to anchor placement / cell moves on (the player when it is there). 0 = none loaded.
    virtual RefId anchorInCell(CellId cell, live::Vec3& anchorPos) = 0;

    // ---- writes ---------------------------------------------------------------------------------------------------------------
    virtual RefId placeAtMe(RefId anchor, uint32_t baseId) = 0;             // new reference of `baseId` next to `anchor`; 0 = failed
    virtual bool setPos(RefId ref, const live::Vec3& p, uint8_t axisMask) = 0;      // same-cell absolute move; mask bit0 x, bit1 y, bit2 z
    virtual bool setYaw(RefId ref, float yawRad) = 0;
    virtual bool moveToCell(RefId ref, CellId cell, const live::Vec3& p, float yawRad) = 0;   // cross-cell move (anchor based)
    virtual bool setRestrained(RefId ref, bool on) = 0;
    virtual bool stopCombat(RefId ref) = 0;
    virtual bool startCombat(RefId ref, RefId target) = 0;
    virtual bool evaluatePackage(RefId ref) = 0;
    virtual bool setEssential(RefId ref, bool on) = 0;                      // the REFERENCE-level essential flag
    virtual bool forceAv(RefId ref, uint8_t av, float value) = 0;           // set the current value
    virtual bool killActor(RefId ref) = 0;
    // G04: the kill of a copy that a REMOTE player's shot caused, attributed to that player's puppet (`killer`), so the engine stores it as the killer before OnDeath and
    // `IsKiller player` comes out true for that player. Default (an engine without attribution): the plain kill.
    virtual bool killActorBy(RefId ref, RefId /*killer*/) { return killActor(ref); }
    virtual bool resurrect(RefId ref) = 0;
    virtual bool setDisabled(RefId ref, bool disabled) = 0;
    virtual bool markForDelete(RefId ref) = 0;

    // Cost accounting: console lines (or engine calls) issued / failed so far. The port budgets its per-frame work on the delta.
    virtual uint64_t linesRun() const { return 0; }
    virtual uint64_t linesFailed() const { return 0; }
};

// ---- the primitives a real engine host must provide; ScriptActorEngine builds the whole IActorEngine on them ------------------------
class IEngineHost {
public:
    virtual ~IEngineHost() = default;
    virtual bool worldReady() = 0;
    virtual uint32_t worldEpoch() = 0;
    virtual RefId playerRef() = 0;
    virtual bool read(RefId ref, ActorRaw& out) = 0;
    virtual bool readPose(RefId ref, CellId& cell, live::Vec3& pos, float& rotZ) {
        ActorRaw r;
        if (!read(ref, r)) return false;
        cell = r.cell; pos = r.pos; rotZ = r.rot.z;
        return true;
    }
    virtual bool actorValue(RefId ref, uint8_t av, float& out) = 0;
    virtual void loadedActors(std::vector<RefId>& out, size_t max) = 0;
    virtual bool cellLoaded(CellId cell) = 0;
    virtual bool cellInfo(CellId cell, CellInfo& out) = 0;
    virtual bool toFormRef(uint32_t formId, fnvcoop::FormRef& out) = 0;
    virtual uint32_t fromFormRef(const fnvcoop::FormRef& r) = 0;
    virtual RefId anchorInCell(CellId cell, live::Vec3& anchorPos) = 0;
    virtual RefId placeAtMe(RefId anchor, uint32_t baseId) = 0;
    // Runs one console line with `on` as the calling reference (0 = none). true = the engine ran it.
    virtual bool console(const std::string& line, RefId on) = 0;
    // `KillActor <killer>` on `victim` with the sync layer's own gate scope (nvhooks::killWithAttribution). false = unavailable here (the caller falls back to the plain kill).
    virtual bool killWithAttribution(RefId /*victim*/, RefId /*killer*/) { return false; }
    // Direct pose write, if the host has one (TODO-RE: no verified address, the real host returns false). Lets a later RE result replace
    // the 3-line SetPos without touching the port.
    virtual bool directPose(RefId ref, const live::Vec3& p, uint8_t axisMask) { (void)ref; (void)p; (void)axisMask; return false; }
};

}  // namespace nvcoop::actors::engine
