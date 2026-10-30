// The engine as the NPC animation layer sees it (enemy-anim). Everything the layer reads from or does to a game actor goes through this
// one interface, so the whole layer is unit-tested against a fake (tests/fake_npc_engine.h). Production: ScriptNpcAnimEngine
// (script_npc_engine.h: console lines on a calling reference + guarded memory reads), see docs/tools/enemy-anim.md.
//
// Actors are addressed by NetId (the cross-machine id ActorSync uses); the implementation maps it to the live reference (real actor or
// stand-in clone). All calls happen on the game main thread. A write returns Refused when the engine did not do it (no 3D yet, the ref is
// gone, the command was rejected) - an ordinary event the driver counts and retries with backoff - and Unsupported when this backend has no
// way to do it at all (the driver then stops asking and uses its fallback).
#pragma once
#include <cstdint>

#include "live/live.h"

namespace nvcoop::actors::anim {

namespace live = fnvcoop::live;
using live::CellId;
using live::NetId;
using live::Vec3;

enum class Result : uint8_t { Ok, Refused, Unsupported };
inline bool ok(Result r) { return r == Result::Ok; }

// Everything the owner side reads about one actor in one call. Fields the backend cannot read keep their defaults and the matching
// `known` flag false, so a missing offset degrades one feature instead of faking a value.
struct NpcReads {
    bool valid = false;           // the reference exists and the reads worked
    bool has3D = false;           // the 3D (skeleton) is loaded; before that no animation command can work
    bool dead = false;            // life state dead or dying
    bool creature = false;        // not a humanoid: no weapon draw / aim / sneak / turn-in-place logic
    bool knocked = false;         // knocked down or getting up (BaseProcess knocked state != 0)
    bool ragdoll = false;         // the physics ragdoll controller is active
    bool swimming = false;
    bool sneaking = false;
    bool weaponOut = false;       // IsWeaponOut
    bool aiming = false;          // BaseProcess aiming flag
    bool inAir = false;           // jumping or falling
    bool talking = false;
    bool moveFlagsKnown = false;
    uint32_t moveFlags = 0;       // ActorMover bits (T7): 7 walk, 8 run, 9 sneak, 11 swim
    // Base group id (anim_groups.h) of the sequence playing on the movement / idle slot. kNoSequence = definitely nothing is playing (a
    // freshly loaded skeleton in its bind pose: the T-pose), kUnknownGroup = this backend cannot tell.
    static constexpr int kUnknownGroup = -1;
    static constexpr int kNoSequence = -2;
    int moveGroup = kUnknownGroup;
    // Base group id of the one-shot sequence playing on the weapon / arm / special-idle slot (attack, reload, stagger ...); -1 = none
    // or unknown, and how long it has been playing in ms (-1 unknown). A restart of the same group shows as playMs going backwards.
    int actionGroup = -1;
    int actionPlayMs = -1;
    // Furniture: BaseProcess::GetSitSleepState (anim_sit.h maps the raw value). sitKnown false = this backend cannot read it, and the layer then
    // neither publishes a sit nor verifies a copy's.
    bool sitKnown = false;
    int sitSleep = 0;
    // Reference id of the furniture the actor sits in or sleeps on; 0 = not readable (the engine has no cheap read for it, the game backend
    // looks for the furniture next to the actor).
    uint32_t furniture = 0;
};

class INpcAnimEngine {
public:
    virtual ~INpcAnimEngine() = default;

    // ---- world --------------------------------------------------------------------------------------------------------
    virtual bool WorldReady() = 0;          // a game is loaded and the player exists (false in the main menu / while loading)
    virtual uint32_t WorldEpoch() = 0;      // changes on every load, new game, return to the main menu
    virtual bool MenuOpen() = 0;            // a menu that stops the world or resets animation (Pip-Boy, dialogue, container, loading)

    // ---- reads --------------------------------------------------------------------------------------------------------
    virtual bool Read(NetId id, NpcReads& out) = 0;
    virtual uint32_t Generation3D(NetId id) = 0;   // changes when the 3D is rebuilt (cell reload, Reset3D); 0 = unknown / none
    virtual bool GetPose(NetId id, CellId& cell, Vec3& pos, float& yawRad) = 0;

    // ---- writes (non-owner copies) ------------------------------------------------------------------------------------
    virtual Result PlayGroup(NetId id, int baseGroup, bool hold) = 0;     // hold: stay in the group until another is played (script flag 1)
    virtual Result ReleaseHold(NetId id) = 0;                             // end a held group so the actor's own AI animates again (ownership taken)
    virtual Result SetGroupRate(NetId id, float rate) = 0;                // playback speed of the movement group; Unsupported is normal
    virtual Result SetSneak(NetId id, bool on) = 0;
    virtual Result SetWeaponDrawn(NetId id, bool on) = 0;                 // the weapon flag + its draw / holster notification
    virtual Result SetAiming(NetId id, bool on) = 0;
    virtual Result KnockDown(NetId id, float yawRad, float force) = 0;    // a physics knockdown; Unsupported -> the driver plays Stagger
    virtual Result GetUp(NetId id) = 0;
    virtual Result Kill(NetId id) = 0;                                    // death for an actor the layer was told is dead and that is not
    virtual Result PlaceCorpse(NetId id, const Vec3& pos, float yawRad) = 0;   // move a dead body (SetPos + SetAngle), nothing else
    // Furniture. Both only start the engine's own animation; the driver reads the sit state back before it believes either.
    virtual Result SitIn(NetId id, uint32_t furniture) = 0;                    // the actor uses that furniture reference (walks in / sits down by the engine's own logic)
    virtual Result StandUp(NetId id, uint32_t furniture, uint32_t attempt) = 0;   // leave the furniture; attempt 0 is the gentle way, higher ones are escalations
    // Last rung of the stand-up ladder: the reference is disabled / enabled again (the console's Disable / Enable). Rebuilds the actor's process and so its furniture
    // state; the caller waits between the two and puts the hold back. Unsupported -> the ladder gives up as before.
    virtual Result SetBodyEnabled(NetId /*id*/, bool /*enabled*/) { return Result::Unsupported; }
    virtual Result Restrain(NetId id) = 0;                                     // put the remote-driven hold back after an escalated StandUp let the actor's AI run
};

}  // namespace nvcoop::actors::anim
