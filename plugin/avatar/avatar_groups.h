// Animation group ids the puppet driver may ask for. Ids follow the xNVSE enum TESAnimGroup::kAnimGroup_*, the same
// table tools/animcheck/animgroups_table.py is generated from. tools/avatar/check_groups.py proves this table equals
// that Python table line by line (game-free). That the running game uses the same numbering is NOT measured yet
// (docs/tools/T7-anim.md "First in-game check"). The name is what the script command `PlayGroup <name> <flag>` takes.
//
// Weapon-type variants: the engine picks the variant of a group that matches the equipped weapon. We only ever ask for
// the base group. TODO-RE: whether PlayGroup by name resolves the weapon variant itself.
#pragma once
#include <cstddef>
#include <cstring>

// The NPC animation layer's group table (every base group name, and what counts as an attack / reaction / held action). Read only: the player puppets
// play the same groups the NPC copies do, so there is exactly one definition of "an action group".
#include "../actors/anim/anim_groups.h"

namespace nvcoop::avatar {

enum Group : int {
    kGrpIdle = 0,
    kGrpForward = 3,
    kGrpBackward = 4,
    kGrpLeft = 5,
    kGrpRight = 6,
    kGrpFastForward = 7,
    kGrpFastBackward = 8,
    kGrpFastLeft = 9,
    kGrpFastRight = 10,
    kGrpAim = 17,
    kGrpAimUp = 18,
    kGrpAimDown = 19,
    kGrpAimIS = 20,
    kGrpAimISUp = 21,
    kGrpAimISDown = 22,
    kGrpHolster = 23,
    kGrpEquip = 24,
    kGrpUnequip = 25,
    kGrpReloadA = 177,
    kGrpDeath = 224,
    kGrpJumpStart = 227,
    kGrpJumpLoop = 228,
    kGrpJumpLand = 229,
    kGrpJumpLoopForward = 236,
    kGrpJumpLandForward = 241,
};

struct GroupInfo {
    int id;
    const char* name;
};

// One entry per line so a script can parse it. Keep sorted by id.
inline constexpr GroupInfo kGroupTable[] = {
    {0, "Idle"},
    {3, "Forward"},
    {4, "Backward"},
    {5, "Left"},
    {6, "Right"},
    {7, "FastForward"},
    {8, "FastBackward"},
    {9, "FastLeft"},
    {10, "FastRight"},
    {17, "Aim"},
    {18, "AimUp"},
    {19, "AimDown"},
    {20, "AimIS"},
    {21, "AimISUp"},
    {22, "AimISDown"},
    {23, "Holster"},
    {24, "Equip"},
    {25, "Unequip"},
    {177, "ReloadA"},
    {224, "Death"},
    {227, "JumpStart"},
    {228, "JumpLoop"},
    {229, "JumpLand"},
    {236, "JumpLoopForward"},
    {241, "JumpLandForward"},
};

// Action groups: the one-shots the owner reports in AnimState::action (attack swing / shot / throw, power attack, block, stagger, recoil, dodge, reload, jam).
// Same set as the NPC layer's sampler uses, so a player and an NPC are described the same way on the wire.
inline bool IsActionGroup(int g) { return actors::anim::validGroup(g) && actors::anim::isActionGroup(g); }
inline bool IsAttackGroup(int g) { return actors::anim::validGroup(g) && actors::anim::isAttackGroup(g); }
inline bool IsReloadActionGroup(int g) { return actors::anim::validGroup(g) && actors::anim::isReloadGroup(g); }
inline bool IsReactionGroup(int g) { return actors::anim::validGroup(g) && actors::anim::isReactionGroup(g); }
// Held actions keep playing while the owner holds them (block, automatic fire): played with the hold flag, ended when the owner stops reporting them.
inline bool IsHeldActionGroup(int g) { return actors::anim::validGroup(g) && actors::anim::isHeldAction(g); }
// Approximate playing time of a one-shot (ms), first guesses from the NPC layer, none measured.
inline unsigned OneShotMs(int g) { return actors::anim::nominalActionMs(g); }

// nullptr = not a group this layer knows (the script backend then refuses the call instead of guessing). The table above holds the groups the
// locomotion planner uses; an action group is named by the NPC layer's full table.
inline const char* GroupName(int id) {
    for (const GroupInfo& g : kGroupTable)
        if (g.id == id) return g.name;
    if (IsActionGroup(id)) {
        const char* n = actors::anim::groupName(id);
        if (n && *n) return n;
    }
    return nullptr;
}

// Group categories used by the planner and the tests.
inline bool IsLocomotionGroup(int g) { return g >= kGrpForward && g <= kGrpFastRight; }
inline bool IsRunGroup(int g) { return g >= kGrpFastForward && g <= kGrpFastRight; }
// Groups that play once and end on their own; the engine going back to idle after them is normal, so the
// readback-mismatch re-assert skips them.
inline bool IsOneShot(int g) { return g == kGrpHolster || g == kGrpEquip || g == kGrpUnequip || g == kGrpReloadA || g == kGrpJumpStart || g == kGrpJumpLand || g == kGrpJumpLandForward || IsActionGroup(g); }
inline bool IsJumpGroup(int g) { return g == kGrpJumpStart || g == kGrpJumpLoop || g == kGrpJumpLand || g == kGrpJumpLoopForward || g == kGrpJumpLandForward; }

}  // namespace nvcoop::avatar
