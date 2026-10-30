// The narrow door between the puppet engine and the running game. IGameBackend has exactly the raw primitives the real
// IPuppetEngine (puppet_game_engine.h) and the local-player sampler (local_sampler.h) need; every policy decision (which ref may
// be touched, what a failed call means, when a console line is worth its cost) lives in game-free code ABOVE this interface and is
// unit tested against FakeGameBackend (testing/fake_backend.h). The only implementation that talks to the game is
// win/game_backend_win.cpp (x86, xNVSE headers, SEH around every raw read, prologue pins from engine_pins.h).
//
// Threading: game main thread only. Nothing here blocks or allocates unboundedly.
#pragma once
#include <cstdint>
#include <string>
#include <vector>

#include "../avatar_types.h"

namespace nvcoop::avatar::engine {

constexpr RefId kPlayerRef = 0x14;

// Condition functions the engine evaluates for us (the evaluators in engine_pins.h). Param is a form id for the two item ones.
enum class Cond : uint8_t { GetRestrained, IsWeaponOut, IsSneaking, GetDead, GetItemCount, GetEquipped, GetLevel, GetSitting, IsPlayerActionActive, GetActorValue };

// What one guarded read of a loaded reference says. All false/0 when the id is not a loaded reference.
struct RefInfo {
    bool exists = false;     // a form with this id is loaded and is a reference
    bool isActor = false;    // Character or Creature (the player is an actor too)
    bool isPlayer = false;   // the PlayerCharacter
    bool deleted = false;    // form flag "deleted" (kFlags_Deleted) or taken
    bool disabled = false;   // the reference is disabled (Disable ran on it)
    FormId base = 0;         // base form id
    CellId cell = 0;         // parent cell form id, 0 = none (not placed in a loaded cell)
    Vec3 pos;                // the reference's own position fields
    Vec3 rot;                // rotation fields, radians (x, y, z); z = heading
    bool has3D = false;      // the reference has a 3D scene object
    uint32_t node3D = 0;     // identity of that scene object; changes when the 3D is rebuilt
    bool havePos3D = false;  // the scene object's world translation could be read
    Vec3 pos3D;              // ... and this is it (diagnostic: does SetPos really move the body?)
};

struct CellInfo {
    bool exists = false;
    bool attached = false;   // fully loaded and attached (references can be driven)
    bool interior = false;
    uint32_t world = 0;      // worldspace form id (0 for interiors)
};

struct ActorEntry {
    RefId ref = 0;
    FormId base = 0;
};

// Running animation groups and stance, read from the process objects (the same reads as plugin/animprobe, which verified the vtable slots).
// One AnimData's nine sequence slots (see AnimRead::groups for the meaning of the fields).
struct AnimSet {
    bool valid = false;               // the AnimData was found and read
    int groups[9] = {-1, -1, -1, -1, -1, -1, -1, -1, -1};
    int state[9] = {-1, -1, -1, -1, -1, -1, -1, -1, -1};
    int playMs[9] = {-1, -1, -1, -1, -1, -1, -1, -1, -1};
    int nGroups = 0;
};

struct AnimRead {
    bool valid = false;               // the actor has a process with animation data
    int groups[9] = {-1, -1, -1, -1, -1, -1, -1, -1, -1};   // group id (low byte) of the sequence in each AnimData slot, -1 = none
    int nGroups = 0;                  // how many slots hold a sequence
    // Per slot: the sequence's play state (NiControllerSequence: 0 inactive, 1 animated, 2 ease in, 3 ease out ...) and how long it has been playing in ms.
    // -1 = this backend cannot tell (then a held group counts as playing and a restart of the same group cannot be seen).
    int state[9] = {-1, -1, -1, -1, -1, -1, -1, -1, -1};
    int playMs[9] = {-1, -1, -1, -1, -1, -1, -1, -1, -1};
    // The LOCAL PLAYER only: the other AnimData objects the engine keeps for him. [0] = the first-person set (PlayerCharacter+0x690), [1] = the alternate (+0x6A0).
    // A player in first person plays his attacks on [0], not on the set above (the process's, third person); not valid = not present / not the player.
    AnimSet alt[2];
    int action = -1;                  // BaseProcess animation action, -1 unknown
    bool aiming = false;              // process IsAiming
    bool weaponOut = false;           // process IsWeaponOut
    bool haveMover = false;           // an ActorMover / PlayerMover with a known vtable
    uint32_t moverFlags = 0;          // its flags: bit 7 walk, bit 8 run, bit 9 sneak, bit 11 swim (T7 notes, unmeasured)
    bool swimming = false;
    bool dead = false;
    bool knocked = false;
    bool firstPerson = false;         // local player only
};

struct EquippedItem {
    FormId form = 0;
    uint8_t formType = 0;             // TESForm::typeID (0x28 = weapon, 0x18 = armor, ...)
    uint8_t mods = 0;                 // ExtraWeaponModFlags of the worn entry (bit 0 = mod 1 ...), 0 when it has none
    bool modsKnown = false;           // the backend could read the extra data of this entry (false: `mods` says nothing)
};
constexpr uint8_t kFormTypeWeapon = 0x28;

class IGameBackend {
public:
    virtual ~IGameBackend() = default;

    // ---- world --------------------------------------------------------------------------------------------------------
    virtual uint32_t WorldEpoch() = 0;                  // changes on every load / new game / return to the main menu
    virtual bool MenuOpen() = 0;                        // MenuMode (any menu); loading screens count
    virtual bool ReadCell(CellId cell, CellInfo& out) = 0;
    // A reference in `cell` that is not an actor (a static, door or marker) to anchor placements on. 0 = none known.
    virtual RefId AnchorRef(CellId cell, Vec3& pos) = 0;
    virtual void ListActors(CellId cell, std::vector<ActorEntry>& out) = 0;   // actors currently in the cell's reference list
    virtual void LoadedCells(std::vector<CellId>& out) = 0;                   // the player's cell and every attached neighbour

    // ---- references ---------------------------------------------------------------------------------------------------
    virtual bool ReadRef(RefId ref, RefInfo& out) = 0;
    virtual bool ReadAnim(RefId ref, AnimRead& out) = 0;
    virtual bool ReadEquipped(RefId ref, std::vector<EquippedItem>& out) = 0;   // false = unknown
    virtual bool ReadAppearance(RefId ref, Appearance& out) = 0;                // race / sex / hair / eyes / body / face data of the actor's base
    // Whether CloneNpcBase can work at all on this game (its pins hold): false is permanent, a 0 from CloneNpcBase while this is true is a refusal that may pass.
    virtual bool CanCloneNpcBase() { return false; }
    // A private copy of an NPC base (a puppet's own look; the template stays untouched). 0 = not possible.
    virtual FormId CloneNpcBase(FormId /*base*/) { return 0; }
    // The look written into a copy made by CloneNpcBase (never into `tmpl`, the base it was copied from): sex, race, hair, eyes, hair colour,
    // height, weight, and the face morphs when the copy owns its FaceGen arrays (*faceWritten). False = nothing written.
    virtual bool WriteNpcLook(FormId /*base*/, FormId /*tmpl*/, const Appearance& /*a*/, bool* faceWritten) { if (faceWritten) *faceWritten = false; return false; }
    // What the last WriteNpcLook did with the parts that can fail quietly ("face written, head parts written"), for the log. Empty = nothing to say.
    virtual std::string LastLookNote() { return std::string(); }

    // ---- script -------------------------------------------------------------------------------------------------------
    // Runs one console line with `ctx` as the calling reference. 1 = ran, 0 = the engine refused or could not compile it,
    // -1 = an exception was caught, -2 = ctx is not a loaded reference.
    virtual int RunLine(RefId ctx, const std::string& line) = 0;
    // A condition function's value. False when the evaluator is unavailable (pin mismatch), the ref is gone, or it faulted.
    // `param` is a form id for GetItemCount / GetEquipped and the plain integer for IsPlayerActionActive and GetActorValue (the actor value code).
    virtual bool Eval(Cond fn, RefId ref, FormId param, double& out) = 0;
    // The region forms of the LOCAL player's PlayerCharacter::regionsList (at most max). False when unreadable. Default: unknown.
    virtual bool ReadPlayerRegions(std::vector<FormId>& out, size_t /*max*/) { out.clear(); return false; }

    // ---- time (cost accounting) ---------------------------------------------------------------------------------------
    virtual uint64_t NowUs() = 0;
};

}  // namespace nvcoop::avatar::engine
