// The game side of the NPC animation layer (enemy-anim): INpcScript over the hooks module's console executor, and INpcGameAccess over guarded
// memory reads of the xNVSE layouts plus the few direct process calls whose slots verify_slots.py proves. x86 only, compiled into nvcoop.dll
// (anim_module.cpp). Nothing in here has run in a game; every raw read is SEH guarded, so a wrong offset gives a missing field, never a crash.
#pragma once
#include <functional>
#include <string>

#include "script_npc_engine.h"

namespace nvcoop::actors::anim::game {

struct GameConfig {
    bool directWeaponOut = true;    // BaseProcess::SetWeaponOut (slot 0x116, pinned) before the SetAlert fallback
    bool directAiming = true;       // BaseProcess::SetIsAiming (slot 0x100, pinned)
    bool directRate = false;        // write the movement sequence's frequency (offset from the xNVSE header, not proven): off until measured
    bool directKnock = false;       // BaseProcess::PushActorAway / SetKnockedState (pinned, force units and effect unmeasured): off until measured
};

// Dev proof (anim.npc.do): a NetId that stands for a plain reference the actor port does not track. 0 removes nothing; ids are never reused for another ref.
void setRefOverride(NetId id, uint32_t ref);

using ConsoleRunner = std::function<bool(uint32_t ref, const std::string& line)>;   // false = the line did not run
using EpochProvider = std::function<uint32_t()>;

// Resolves NetId -> reference through actors::engine::refOf (the actor port's registry).
class GameScript : public INpcScript {
public:
    explicit GameScript(ConsoleRunner run) : run_(std::move(run)) {}
    bool Exec(NetId id, const std::string& line) override;
    bool ExecOnRef(uint32_t ref, const std::string& line) override { return ref && run_ && run_(ref, line); }
private:
    ConsoleRunner run_;
};

class GameAccess : public INpcGameAccess {
public:
    GameAccess(const GameConfig& cfg, EpochProvider epoch) : cfg_(cfg), epoch_(std::move(epoch)) {}
    bool WorldReady() override;
    uint32_t WorldEpoch() override { return epoch_ ? epoch_() : 1u; }
    bool MenuOpen() override;
    bool ReadActor(NetId id, NpcReads& out) override;
    uint32_t Generation3D(NetId id) override;
    bool GetPose(NetId id, CellId& cell, Vec3& pos, float& yawRad) override;
    Result SetWeaponOut(NetId id, bool on) override;
    Result SetAiming(NetId id, bool on) override;
    Result SetGroupRate(NetId id, float rate) override;
    Result KnockDown(NetId id, float yawRad, float force) override;
    Result GetUp(NetId id) override;
    uint32_t RefIdOf(NetId id) override;
    // Which direct calls passed their start-up check (shown by anim.npc).
    std::string pinStatusJson() const;
private:
    GameConfig cfg_;
    EpochProvider epoch_;
    bool menuPinChecked_ = false, menuPinOk_ = false;
};

}  // namespace nvcoop::actors::anim::game
