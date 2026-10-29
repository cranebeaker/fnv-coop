// Production INpcAnimEngine (enemy-anim): console lines on a calling reference for the writes the game has a command for, and a separate
// read/direct-call interface (INpcGameAccess) for everything a console line cannot do. Game-free: both interfaces are implemented in the
// game by anim_game.cpp (and by fakes in the tests), so this file's whole job - which line is sent for which request - is unit-tested.
//
// Console lines used (names from the game's own script command set; the argument conventions of PlayGroup's flag, SetAngle's sign and
// whether PlayGroup by name resolves the weapon-type variant are UNMEASURED - docs/tools/enemy-anim.md, "in-game proof"):
//   PlayGroup <Name> <0|1>     movement groups (hold = 1), one-shots (0)
//   SetForceSneak <0|1>        sneak
//   SetAlert <0|1>             fallback when the direct weapon-out call is not available
//   KillActor                  a death the engine did not take (the command table name; "Kill" is not in it)
//   SetPos X|Y|Z <v>, SetAngle Z <deg>   a body moved to the owner's resting place
//   <furniture>.Activate <actor> 1        an actor uses a piece of furniture (the same line the guest-action executor already runs; the calling
//                                         reference is the FURNITURE, the argument the actor). UNMEASURED for an NPC activator on a restrained actor.
//   stand up, gentle: <furniture>.Activate <actor> 1 again; escalation: SetRestrained 0 + EvaluatePackage on the actor (its own AI then leaves
//                                         the furniture, if its package says so), and Restrain puts SetRestrained 1 back. UNMEASURED: the driver
//                                         reads the sit state back and climbs the ladder only while the copy is still in the furniture.
// Not available as a line (the backend answers Unsupported unless INpcGameAccess has the direct call): playback rate, aiming flag, knockdown,
// get up.
#pragma once
#include <string>

#include "npc_anim_engine.h"

namespace nvcoop::actors::anim {

// Console lines on a calling reference (the bridge's RunLineSEH / GameHost::ExecOn). Keyed by NetId; the implementation maps it to the live ref.
class INpcScript {
public:
    virtual ~INpcScript() = default;
    virtual bool Exec(NetId id, const std::string& line) = 0;   // false: the line did not run (no ref, no 3D, console error)
    virtual bool ExecOnRef(uint32_t ref, const std::string& line) = 0;   // the same on an explicit reference (a piece of furniture)
};

// Guarded memory reads and the few direct engine calls whose vtable slots were checked statically (verify_slots.py). Each direct call returns
// Unsupported when it is switched off in nvcoop.ini or its signature check failed at start-up.
class INpcGameAccess {
public:
    virtual ~INpcGameAccess() = default;
    virtual bool WorldReady() = 0;
    virtual uint32_t WorldEpoch() = 0;
    virtual bool MenuOpen() = 0;
    virtual bool ReadActor(NetId id, NpcReads& out) = 0;
    virtual uint32_t Generation3D(NetId id) = 0;
    virtual bool GetPose(NetId id, CellId& cell, Vec3& pos, float& yawRad) = 0;
    virtual Result SetWeaponOut(NetId id, bool on) = 0;           // BaseProcess::SetWeaponOut(Actor*, bool) (vtable slot 0x116)
    virtual Result SetAiming(NetId id, bool on) = 0;              // BaseProcess::SetIsAiming(bool) (slot 0x100)
    virtual Result SetGroupRate(NetId id, float rate) = 0;        // NiControllerSequence frequency of the movement sequence (unverified, off by default)
    virtual Result KnockDown(NetId id, float yawRad, float force) = 0;   // BaseProcess::PushActorAway (slot 0x106; semantics unverified, off by default)
    virtual Result GetUp(NetId id) = 0;
    virtual uint32_t RefIdOf(NetId id) = 0;   // the live reference of the actor (0 = none): the argument of the furniture lines
};

std::string formatNumber(double v);        // "%.3f", never a decimal comma
double yawRadToDegrees(float yawRad);      // wrapped to (-180, 180]

class ScriptNpcAnimEngine : public INpcAnimEngine {
public:
    ScriptNpcAnimEngine(INpcScript& script, INpcGameAccess& game) : run_(script), game_(game) {}

    bool WorldReady() override { return game_.WorldReady(); }
    uint32_t WorldEpoch() override { return game_.WorldEpoch(); }
    bool MenuOpen() override { return game_.MenuOpen(); }
    bool Read(NetId id, NpcReads& out) override { return game_.ReadActor(id, out); }
    uint32_t Generation3D(NetId id) override { return game_.Generation3D(id); }
    bool GetPose(NetId id, CellId& cell, Vec3& pos, float& yawRad) override { return game_.GetPose(id, cell, pos, yawRad); }

    Result PlayGroup(NetId id, int baseGroup, bool hold) override;
    Result ReleaseHold(NetId id) override;
    Result SetGroupRate(NetId id, float rate) override { return game_.SetGroupRate(id, rate); }
    Result SetSneak(NetId id, bool on) override;
    Result SetWeaponDrawn(NetId id, bool on) override;
    Result SetAiming(NetId id, bool on) override { return game_.SetAiming(id, on); }
    Result KnockDown(NetId id, float yawRad, float force) override { return game_.KnockDown(id, yawRad, force); }
    Result GetUp(NetId id) override { return game_.GetUp(id); }
    Result Kill(NetId id) override;
    Result PlaceCorpse(NetId id, const Vec3& pos, float yawRad) override;
    Result SitIn(NetId id, uint32_t furniture) override;
    Result StandUp(NetId id, uint32_t furniture, uint32_t attempt) override;
    Result Restrain(NetId id) override;

private:
    Result useFurniture(NetId id, uint32_t furniture);
    Result line(NetId id, const std::string& l) { return run_.Exec(id, l) ? Result::Ok : Result::Refused; }
    INpcScript& run_;
    INpcGameAccess& game_;
};

}  // namespace nvcoop::actors::anim
