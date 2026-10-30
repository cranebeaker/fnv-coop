// Animation groups, health and speed as the parity checker compares them (game-free; the Python comparer in tools/parity/compare.py mirrors every
// function here and tools/parity/vectors.json pins both to the same answers).
#pragma once
#include <cstdint>
#include <vector>

#include "actors/anim/anim_groups.h"
#include "fnvcoop_mut.h"
#include "parity/parity_types.h"

namespace nvcoop::parity {

namespace ag = nvcoop::actors::anim;

// What a group means for "do the two screens show the same thing". Locomotion classes are compared; Transient ones (an attack, a reload, a flinch, a jump,
// a draw, a turn) last well under a sample interval, so a difference there is timing, not drift.
enum class GroupClass : uint8_t { Unknown = 0, Idle = 1, Walk = 2, Run = 3, Death = 4, Transient = 5 };

inline GroupClass groupClass(int g) {
    if (g < 0 || g > 255 || g == kNoGroup) return GroupClass::Unknown;
    if (g == ag::kGroupDeath) return GroupClass::Death;
    if (g >= ag::kGroupForward && g < ag::kGroupForward + 4) return GroupClass::Walk;                 // 3..6
    if (g >= ag::kGroupForward + 4 && g <= ag::kGroupFastRight) return GroupClass::Run;               // 7..10
    if (ag::isDodgeGroup(g) || ag::isTurnGroup(g) || g == ag::kGroupHolster || g == ag::kGroupEquip || g == ag::kGroupUnequip) return GroupClass::Transient;
    if (ag::isAttackGroup(g) || ag::isReloadGroup(g) || ag::isJamGroup(g) || ag::isReactionGroup(g) || ag::isJumpGroup(g) || g == ag::kGroupBlockIdle) return GroupClass::Transient;
    if (g >= ag::kGroupJumpStart && g <= ag::kGroupLast) return GroupClass::Transient;
    return GroupClass::Idle;   // idle, dynamic idle, special idle, aim, talking and anything else that is not movement
}

inline const char* className(GroupClass c) {
    switch (c) {
        case GroupClass::Idle: return "idle";
        case GroupClass::Walk: return "walk";
        case GroupClass::Run: return "run";
        case GroupClass::Death: return "death";
        case GroupClass::Transient: return "transient";
        default: return "unknown";
    }
}

// The one group that stands for an actor's whole AnimData: death beats everything, then a transient one-shot, then run, walk, idle.
// -1 entries (empty slots) are ignored. kNoGroup = nothing readable.
inline uint8_t primaryGroup(const std::vector<int>& groups) {
    int death = -1, transient = -1, run = -1, walk = -1, idle = -1;
    for (int g : groups) {
        if (g < 0 || g > 244) continue;
        switch (groupClass(g)) {
            case GroupClass::Death: death = g; break;
            case GroupClass::Transient: if (transient < 0) transient = g; break;
            case GroupClass::Run: if (run < 0) run = g; break;
            case GroupClass::Walk: if (walk < 0) walk = g; break;
            case GroupClass::Idle: if (idle < 0) idle = g; break;
            default: break;
        }
    }
    const int loco = FNV_MUT(1808) ? (walk >= 0 ? walk : run) : (run >= 0 ? run : walk);   // twin 1808: walk outranks run
    const int pick = death >= 0 ? death : transient >= 0 ? transient : loco >= 0 ? loco : idle;
    return pick < 0 ? kNoGroup : static_cast<uint8_t>(pick);
}

// 0 = at or below zero; 1.. = 25 hp per step, capped at 20 (500 hp). kNoHp = unreadable.
inline uint8_t hpBucket(bool readable, float hp) {
    if (!readable) return kNoHp;
    if (hp <= 0.0f) return 0;
    const int b = 1 + static_cast<int>(hp / 25.0f);
    return static_cast<uint8_t>(b > 20 ? 20 : b);
}

// An attack animation starting on an entity: the sampler sees the group ids playing every frame; a start is an attack group that was not playing the frame before.
inline bool isAttackStartGroup(int g) { return g >= 0 && g <= 244 && ag::isAttackGroup(g); }

}  // namespace nvcoop::parity
