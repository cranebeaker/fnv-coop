//! `fallout/ai/actor.cpp` (Xbox PDB source unit), part 5: its functions from `008b00c0` up to
//! (not including) `008b8e90` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::actor`]; anything public there may be used here.
//!
//! The PC `Actor` is the Xbox PDB's with every field after `TESForm` 0x10 lower
//! (`TESForm` is 0x18 on PC), so `pCurrentProcess` (PDB +0x78) is at +0x68.
//! Several functions here are thunks of the `MagicTarget` base (the `Actor`
//! pointer plus 0xA4): they take that interior pointer as `this` and reach
//! the actor's process at `this - 0x3C`.

#[allow(unused_imports)]
use super::actor::*;
#[allow(unused_imports)]
use crate::prelude::*;

/// `PlayerCharacter::pSingleton`: the global holding the player's pointer.
const PLAYER_CHARACTER: u32 = 0x011d_ea3c;
/// `0.0` as a `double` in the exe's constants (compared against by `FCOMP`).
const ZERO_DOUBLE: u32 = 0x0101_2060;

/// `Actor::bPlayerTeammate` getter (the engine map names it
/// `MiddleHighProcess::GetForceNextUpdate` because the linker folded the
/// identical bodies; it reads the byte at actor +0x18D).
const ACTOR_IS_PLAYER_TEAMMATE: u32 = 0x0056_6950;
/// `Actor::GetActorAggression` (Xbox PDB): the actor's aggression, 3 meaning
/// "never fights".
const ACTOR_GET_ACTOR_AGGRESSION: u32 = 0x008b_ea10;
/// `Actor::GetActorAssistance` (Xbox PDB).
const ACTOR_GET_ACTOR_ASSISTANCE: u32 = 0x008b_eae0;
/// `Actor::IsAngryWithPlayer` (Xbox PDB).
const ACTOR_IS_ANGRY_WITH_PLAYER: u32 = 0x008b_ffc0;
/// `Actor::IsInCombatWithActor` (Xbox PDB): `(this, other)`.
const ACTOR_IS_IN_COMBAT_WITH_ACTOR: u32 = 0x008b_c700;
/// `Actor::GetFactionFightReaction` (Xbox PDB): `(this, other, out flag byte)`.
const ACTOR_GET_FACTION_FIGHT_REACTION: u32 = 0x008b_87a0;
/// `Actor::GetAnimation` (Xbox PDB): the process's animation, or 0.
const ACTOR_GET_ANIMATION: u32 = 0x008b_70d0;
/// `PlayerCharacter::GetAnimation` (Xbox PDB): `(this, first_person)`.
const PLAYER_GET_ANIMATION: u32 = 0x0095_0a60;
/// `Animation::ReloadTargets` (Xbox PDB): `(animation, flag)`.
const ANIMATION_RELOAD_TARGETS: u32 = 0x0049_9240;
/// `MiddleHighProcess::GetSavedAcquireObject` (Xbox PDB): the actor's
/// process (the body is `this->pCurrentProcess`).
const ACTOR_GET_PROCESS: u32 = 0x008d_8520;
/// `BGSEntryPoint::HandleEntryPoint` (Xbox PDB), cdecl.
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// Getter of `Actor::bInCombat` (Xbox PDB +0x114, PC +0x104), a byte.
const ACTOR_IS_IN_COMBAT: u32 = 0x0049_3bb0;
/// `PlayerCharacter::IsTargetPerceivedAndHostile` (Xbox PDB): `(player, target)`.
const PLAYER_IS_TARGET_PERCEIVED_AND_HOSTILE: u32 = 0x0093_dee0;
/// `"FollowerSwitchAggressive"` follower setting lookup: `(actor, name)`,
/// returns the value as a float in ST0.
const ACTOR_GET_FOLLOWER_SETTING: u32 = 0x008c_1740;

/// `ProcessLists` instance (`ProcessLists::GetActorRefInHigh`'s `this`).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// The lip sync background manager instance (`LipSyncBackgroundManager::CancelLipFile`'s `this`).
const LIP_SYNC_MANAGER: u32 = 0x011f_11e0;
/// `Actor::GetIronSights` (Xbox PDB): the process's slot `0x404`, false without a process.
const ACTOR_GET_IRON_SIGHTS: u32 = 0x008b_bc10;
/// `Actor::SetLifeState` (Xbox PDB): `(this, state)`.
const ACTOR_SET_LIFE_STATE: u32 = 0x008a_1800;

layout! {
    /// `Actor` (Xbox PDB), the fields this part uses, at their PC offsets
    /// (the PDB's minus 0x10).
    pub struct Actor: 0x1b4 {
        /// `pCurrentProcess` (Xbox PDB +0x78): `BaseProcess*`.
        0x68 pCurrentProcess: Ptr,
        /// `pMyKiller` (Xbox PDB +0xD0): `Actor*`.
        0xC0 pMyKiller: Ptr,
        /// `bMurderAlarm` (Xbox PDB +0xD4).
        0xC4 bMurderAlarm: u8,
        /// `bReloadTargetQueued` (Xbox PDB +0x101).
        0xF1 bReloadTargetQueued: u8,
        /// `iVisFlags` (Xbox PDB +0x12C).
        0x11C iVisFlags: u32,
        /// `bSetOnDeath` (Xbox PDB +0x184): set once the death was handled.
        0x174 bSetOnDeath: u8,
        /// `fGunSkillGun` (Xbox PDB +0x188): cached value for selector 1 of
        /// the entry-point value at `008b0dd0`.
        0x178 fGunSkillGun: f32,
        /// `fGunSkillHUD` (Xbox PDB +0x18C): cached value for selector 0.
        0x17C fGunSkillHUD: f32,
        /// `fGunSkillActor` (Xbox PDB +0x190): cached value for selector 2.
        0x180 fGunSkillActor: f32,
        /// `fGunSkillVATS` (Xbox PDB +0x194): cached value for selector 3.
        0x184 fGunSkillVATS: f32,
        /// `pActorMover` (Xbox PDB +0x1A0): `ActorMover*`.
        0x190 pActorMover: Ptr,
    }
}

// Translated from 008b00c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the character controller's `0x4000` state flag from the actor
/// value `0x15`: when the actor has a controller whose state does not have
/// `0x10000000` set, the flag is set exactly when that value is `0.0`
/// (`0087f9c0` returns the value as a float).
pub fn fn_008b00c0(e: &mut Engine, this: Ptr<Actor>) {
    // MobileObject::GetCharController (Xbox PDB)
    let controller = e.call(0x0093_06d0, &args![this]).u32();
    if controller == 0 {
        return;
    }
    let state = controller + 0x410;
    if e.call(0x0062_1270, &args![state, 0x1000_0000u32]).u32() != 0 {
        return;
    }
    let value = e.call(0x0087_f9c0, &args![this]).f64();
    let zero: f64 = e.global(ZERO_DOUBLE);
    let is_zero = value == zero;
    e.call(0x0062_9670, &args![state, 0x4000u32, is_zero as u32]);
}

// Translated from 008b0140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set` non-zero) or clears the bits of `mask` in `iVisFlags`.
pub fn fn_008b0140(e: &mut Engine, this: Ptr<Actor>, set: u8, mask: u32) {
    let flags = e.get(this, Actor::iVisFlags);
    let flags = if set != 0 {
        flags | mask
    } else {
        !mask & flags
    };
    e.set(this, Actor::iVisFlags, flags);
}

// Translated from 008b0190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsVisible` (Xbox PDB): true when every bit of `mask` is set in
/// `iVisFlags`.
pub fn actor_is_visible(e: &mut Engine, this: Ptr<Actor>, mask: u32) -> bool {
    let flags = e.get(this, Actor::iVisFlags);
    (!(flags & mask) & mask) == 0
}

// Translated from 008b0520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Decides whether the actor's process should hand its acquired object back
/// (`true` means yes). Virtual slots used are those of the actor (`0x160`,
/// `0x1D0`) and of its process (`0x22C`, `0x280`, `0x5F8`, `0x5FC`).
/// Returns false at once when slot `0x160` of the actor answers true.
pub fn fn_008b0520(e: &mut Engine, this: Ptr<Actor>) -> bool {
    if e.vcall(this.addr(), 0x160, &args![]).bool() {
        return false;
    }
    let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
    let object = e.vcall(process, 0x22c, &args![]).u32();
    if object != 0 {
        // The object's kind (`0041ca90`) must be 5 or 6 and the process
        // must answer slot 0x280 for it.
        let mut proceed = false;
        if e.call(0x0041_ca90, &args![object]).i32() == 5 {
            let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
            proceed = e.vcall(process, 0x280, &args![]).u32() != 0;
        }
        if !proceed && e.call(0x0041_ca90, &args![object]).i32() == 6 {
            let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
            proceed = e.vcall(process, 0x280, &args![]).u32() != 0;
        }
        if !proceed {
            return false;
        }
    }
    let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
    if e.vcall(process, 0x5fc, &args![]).bool() {
        let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
        e.vcall(process, 0x5f8, &args![0u32]);
        return false;
    }
    let target = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    if target != 0 && e.vcall(target, 0x10, &args![]).u32() != 0 {
        // `00524c90` takes nothing; the 0.8f constant is the second argument.
        let first = e.call(0x0052_4c90, &args![]).u32();
        let limit: f32 = e.global(0x0103_0ff0);
        if !e.call(0x00b4_e030, &args![target, first, limit]).bool() {
            return true;
        }
    }
    false
}

// Translated from 008b0670 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when `other` is not the player and neither is `this` (or `this` is
/// the player's teammate); otherwise asks
/// [`actor_get_should_attack_actor`] with the flag set. The `out` value of
/// that call is discarded.
pub fn fn_008b0670(e: &mut Engine, this: Ptr<Actor>, other: Ptr<Actor>) -> bool {
    let player: u32 = e.global(PLAYER_CHARACTER);
    let must_ask = if other.addr() == player {
        true
    } else if this.addr() == player {
        false
    } else {
        !e.call(ACTOR_IS_PLAYER_TEAMMATE, &args![this]).bool()
    };
    if !must_ask {
        return true;
    }
    e.with_stack(4, |e, out| {
        actor_get_should_attack_actor(e, this, other, true, out, false)
    })
}

// Translated from 008b06d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetShouldAttackActor` (Xbox PDB): whether `this` should attack
/// `target`. `engaged` is the caller's "already fighting" flag (set here when
/// `target` is in combat with `this`, or `this` is a teammate and the target
/// is in combat with the player, unless `ignore_combat`); `out` receives the
/// faction fight reaction. A teammate of the player judges for the player,
/// and with `FollowerSwitchAggressive` set treats its aggression as 1.
pub fn actor_get_should_attack_actor(
    e: &mut Engine,
    this: Ptr<Actor>,
    target: Ptr<Actor>,
    engaged: bool,
    out: Ptr,
    ignore_combat: bool,
) -> bool {
    let player: u32 = e.global(PLAYER_CHARACTER);
    let mut engaged = engaged;
    let mut follower = false;
    let teammate = e.call(ACTOR_IS_PLAYER_TEAMMATE, &args![this]).bool();
    if teammate {
        if target.addr() == player {
            return e.call(ACTOR_IS_ANGRY_WITH_PLAYER, &args![this]).bool();
        }
        if e.vcall(target.addr(), 0x1a0, &args![0u32]).bool() {
            return false;
        }
        let switch_aggressive = e
            .call(ACTOR_GET_FOLLOWER_SETTING, &args![this, 0x0107_1de0u32])
            .f64();
        let zero: f64 = e.global(ZERO_DOUBLE);
        if switch_aggressive == zero {
            if !e.call(ACTOR_IS_IN_COMBAT, &args![target]).bool() {
                return false;
            }
            return e
                .call(ACTOR_IS_IN_COMBAT_WITH_ACTOR, &args![target, player])
                .bool()
                || e.call(ACTOR_IS_IN_COMBAT_WITH_ACTOR, &args![target, this])
                    .bool();
        }
        follower = true;
    } else if e.call(ACTOR_IS_PLAYER_TEAMMATE, &args![target]).bool() {
        return e.with_stack(4, |e, ignored| {
            actor_get_should_attack_actor(e, this, Ptr::new(player), engaged, ignored, false)
        });
    }
    let aggression = if follower {
        1
    } else {
        e.call(ACTOR_GET_ACTOR_AGGRESSION, &args![this]).i32()
    };
    if aggression == 3 {
        return true;
    }
    if (e
        .call(ACTOR_IS_IN_COMBAT_WITH_ACTOR, &args![target, this])
        .bool()
        || (e.call(ACTOR_IS_PLAYER_TEAMMATE, &args![this]).bool()
            && e.call(ACTOR_IS_IN_COMBAT_WITH_ACTOR, &args![target, player])
                .bool()))
        && !ignore_combat
    {
        engaged = true;
    }
    // Eight bytes: the reaction function's flag out value at +0, the verdict
    // byte at +4.
    e.with_stack(8, |e, scratch| {
        let reaction = if follower && !target.is_null() {
            e.call(
                ACTOR_GET_FACTION_FIGHT_REACTION,
                &args![target, player, scratch],
            )
            .i32()
        } else {
            e.call(
                ACTOR_GET_FACTION_FIGHT_REACTION,
                &args![this, target, scratch],
            )
            .i32()
        };
        e.mem.set_i32(out.addr(), reaction);
        let verdict = scratch.byte_add(4);
        e.mem.set_u8(verdict.addr(), 0);
        if target.addr() == player && e.call(ACTOR_IS_ANGRY_WITH_PLAYER, &args![this]).bool() {
            e.mem.set_u8(verdict.addr(), 1);
        } else if engaged {
            if e.call(ACTOR_GET_ACTOR_AGGRESSION, &args![target]).i32() == 3
                || (reaction != 3 && reaction != 2)
            {
                e.mem.set_u8(verdict.addr(), 1);
            }
        } else {
            match aggression {
                1 if reaction == 1
                    && (!teammate
                        || e.call(
                            PLAYER_IS_TARGET_PERCEIVED_AND_HOSTILE,
                            &args![player, target],
                        )
                        .bool()) =>
                {
                    e.mem.set_u8(verdict.addr(), 1);
                }
                2 => {
                    if reaction <= 1 {
                        e.mem.set_u8(verdict.addr(), 1);
                    }
                }
                3 => e.mem.set_u8(verdict.addr(), 1),
                _ => {}
            }
        }
        e.call(HANDLE_ENTRY_POINT, &args![0xfu32, this, target, verdict]);
        e.mem.u8(verdict.addr()) != 0
    })
}

// Translated from 008b0970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetShouldHelp` (Xbox PDB): whether `this` should help `other`.
/// Nobody is never helped, oneself always is. A teammate of the player
/// helps whoever the player is not in combat against, and an actor facing a
/// teammate asks about the player instead; otherwise the answer follows the
/// actor's assistance setting and its faction's reaction to `other`.
pub fn actor_get_should_help(e: &mut Engine, this: Ptr<Actor>, other: Ptr<Actor>) -> bool {
    if other.is_null() {
        return false;
    }
    if other == this {
        return true;
    }
    let player: u32 = e.global(PLAYER_CHARACTER);
    if e.call(ACTOR_IS_PLAYER_TEAMMATE, &args![this]).bool() {
        if other.addr() == player {
            return !e.call(ACTOR_IS_ANGRY_WITH_PLAYER, &args![this]).bool();
        }
        if e.call(ACTOR_IS_IN_COMBAT, &args![other]).bool()
            && (e
                .call(ACTOR_IS_IN_COMBAT_WITH_ACTOR, &args![other, player])
                .bool()
                || e.call(ACTOR_IS_IN_COMBAT_WITH_ACTOR, &args![other, this])
                    .bool())
        {
            return false;
        }
        return actor_get_should_help(e, Ptr::new(player), other);
    }
    if e.call(ACTOR_IS_PLAYER_TEAMMATE, &args![other]).bool() {
        return actor_get_should_help(e, this, Ptr::new(player));
    }
    if e.call(ACTOR_GET_ACTOR_AGGRESSION, &args![other]).i32() == 3 {
        return false;
    }
    let assistance = e.call(ACTOR_GET_ACTOR_ASSISTANCE, &args![this]).i32();
    let reaction = e.with_stack(4, |e, flag| {
        e.call(ACTOR_GET_FACTION_FIGHT_REACTION, &args![this, other, flag])
            .i32()
    });
    match assistance {
        1 => reaction == 2,
        2 => reaction == 3 || reaction == 2,
        _ => false,
    }
}

// Translated from 008b0ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reloads the targets of the actor's animation (virtual slot `0x1E4`) with
/// the queued flag `bReloadTargetQueued`, then clears that flag.
pub fn fn_008b0ac0(e: &mut Engine, this: Ptr<Actor>) {
    let queued = e.get(this, Actor::bReloadTargetQueued);
    let animation = e.vcall(this.addr(), 0x1e4, &args![]).u32();
    e.call(ANIMATION_RELOAD_TARGETS, &args![animation, queued]);
    e.set(this, Actor::bReloadTargetQueued, 0);
}

// Translated from 008b0b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ReloadTargets` (Xbox PDB): reloads the targets of the actor's
/// animation. The player has two (first and third person); any other actor
/// needs both an animation and a process.
pub fn actor_reload_targets(e: &mut Engine, this: Ptr<Actor>, flag: u8) {
    let player: u32 = e.global(PLAYER_CHARACTER);
    if this.addr() == player {
        for first_person in [1u32, 0u32] {
            if e.call(PLAYER_GET_ANIMATION, &args![player, first_person])
                .u32()
                != 0
            {
                let animation = e
                    .call(PLAYER_GET_ANIMATION, &args![player, first_person])
                    .u32();
                e.call(ANIMATION_RELOAD_TARGETS, &args![animation, flag]);
            }
        }
    } else if e.call(ACTOR_GET_ANIMATION, &args![this]).u32() != 0
        && !e.get(this, Actor::pCurrentProcess).is_null()
    {
        let animation = e.call(ACTOR_GET_ANIMATION, &args![this]).u32();
        e.call(ANIMATION_RELOAD_TARGETS, &args![animation, flag]);
    }
}

// Translated from 008b0ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The process's virtual slot `0x52C` result, or 0 without a process.
pub fn fn_008b0ba0(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let process = e.get(this, Actor::pCurrentProcess);
    if process.is_null() {
        return 0;
    }
    e.vcall(process.addr(), 0x52c, &args![]).u32()
}

// Translated from 008b0bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::InitLightingPropertyPtr` (Xbox PDB): finds the lighting source
/// among the children of `node`'s first child (virtual slot `0xC` of the
/// node): the child named `"SkinAttachment"` if there is one, otherwise the
/// first child not named `"BIP..."` that has a child list and a lighting
/// property, otherwise the first child itself; then hands the lighting
/// property found to the process (virtual slot `0x4FC`).
pub fn actor_init_lighting_property_ptr(e: &mut Engine, this: Ptr<Actor>, node: Ptr) {
    let process = e.get(this, Actor::pCurrentProcess);
    if process.is_null() || node.is_null() {
        return;
    }
    let first = e.vcall(node.addr(), 0xc, &args![]).u32();
    let mut property = 0u32;
    if first == 0 {
        return;
    }
    // "SkinAttachment" (`0x101f724`)
    let mut chosen = e.call(0x004a_ae30, &args![node, 0x0101_f724u32]).u32();
    if chosen == 0 {
        let mut index = 0u32;
        loop {
            let count = e.call(0x0043_b480, &args![first]).u32();
            if index >= count || chosen != 0 {
                break;
            }
            let child = e.call(0x0043_b4a0, &args![first, index]).u32();
            if child != 0 {
                let name_holder = e.call(0x0041_3f40, &args![child]).u32();
                let name = e.call(0x0043_b1b0, &args![name_holder]).u32();
                // `_strnicmp(name, "BIP", 3)` (`0x1084e60`)
                if name != 0
                    && e.call(0x00ec_7ec0, &args![name, 0x0108_4e60u32, 3u32])
                        .i32()
                        != 0
                {
                    let grandchildren = e.vcall(child, 0xc, &args![]).u32();
                    if grandchildren != 0
                        && e.call(0x0043_b480, &args![grandchildren]).u32() != 0
                        && e.call(0x004b_2960, &args![grandchildren]).u32() != 0
                    {
                        property = e.call(0x004b_5710, &args![grandchildren]).u32();
                        if property != 0 {
                            chosen = grandchildren;
                        }
                    }
                }
            }
            index += 1;
        }
        if chosen == 0 {
            chosen = first;
        }
    }
    if property == 0 {
        property = e.call(0x004b_5710, &args![chosen]).u32();
    }
    e.vcall(process.addr(), 0x4fc, &args![property]);
}

// Translated from 008b0d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of the `MagicTarget` base (`this` is the actor + 0xA4): the
/// process's virtual slot `0x5E4` as a float, or `0.0` without a process.
pub fn fn_008b0d30(e: &mut Engine, this: Ptr, argument: u32) -> f32 {
    let process = e.mem.u32(this.addr().wrapping_sub(0x3c));
    if process == 0 {
        return 0.0;
    }
    e.vcall(process, 0x5e4, &args![argument]).f32()
}

// Translated from 008b0d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of the `MagicTarget` base (`this` is the actor + 0xA4): the
/// process's virtual slot `0x5EC` as a float, or `0.0` without a process.
pub fn fn_008b0d70(e: &mut Engine, this: Ptr, argument: u32) -> f32 {
    let process = e.mem.u32(this.addr().wrapping_sub(0x3c));
    if process == 0 {
        return 0.0;
    }
    e.vcall(process, 0x5ec, &args![argument]).f32()
}

// Translated from 008b0db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk of the `MagicTarget` base (`this` is the actor + 0xA4):
/// `ModifierList::GetModifier` (Xbox PDB, `00937730`) of the list embedded at
/// `this + 0x2C` (the actor's permanent modifiers), as a float.
pub fn fn_008b0db0(e: &mut Engine, this: Ptr, modifier: u8) -> f32 {
    e.call(0x0093_7730, &args![this.addr() + 0x2c, modifier])
        .f32()
}

// Translated from 008b0dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A cached value from the actor's entry-point calculation, by `selector`
/// (1, 2, 0 or 3: `fGunSkillGun`, `fGunSkillActor`, `fGunSkillHUD`,
/// `fGunSkillVATS`). A non-negative cache is returned as is (a negative one
/// for selector 1 is reset to 0); otherwise the value is computed from the
/// process's base value (virtual slot `0x42C`) with `00646910`, adjusted by
/// entry point `0x22` and, for selector 3, scaled by a game setting and added
/// to the process's base value again, then cached. Returns `0.0` without a
/// process or without the process's item (slot `0x148`).
pub fn fn_008b0dd0(e: &mut Engine, this: Ptr<Actor>, selector: i32) -> f32 {
    let process = e.get(this, Actor::pCurrentProcess);
    if process.is_null() || e.vcall(process.addr(), 0x148, &args![]).u32() == 0 {
        return 0.0;
    }
    let item = e.vcall(process.addr(), 0x148, &args![]).u32();
    let item_value = e.call(0x0044_ddc0, &args![item]).u32();
    match selector {
        1 => {
            let cached = e.get(this, Actor::fGunSkillGun);
            if cached >= 0.0 {
                return cached;
            }
            e.set(this, Actor::fGunSkillGun, 0.0);
            return 0.0;
        }
        2 => {
            let cached = e.get(this, Actor::fGunSkillActor);
            if cached >= 0.0 {
                return cached;
            }
        }
        0 => {
            let cached = e.get(this, Actor::fGunSkillHUD);
            if cached >= 0.0 {
                return cached;
            }
        }
        3 => {
            let cached = e.get(this, Actor::fGunSkillVATS);
            if cached >= 0.0 {
                return cached;
            }
        }
        _ => {}
    }
    let base = e.vcall(process.addr(), 0x42c, &args![]).f32();
    let mut flag_100 = false;
    let mut running = false;
    if selector != 3 {
        let flags = e.call(0x0088_46e0, &args![this]).u32();
        flag_100 = flags & 0x100 != 0;
        running = flags & 0xf != 0 && e.call(0x0088_4730, &args![this]).bool();
    }
    e.vcall(this.addr(), 0x1e4, &args![]);
    let mode = e.call(0x0044_ddc0, &args![0x011f_2250u32]).i32();
    let mut value = if mode == 2 || mode == 3 {
        e.call(
            0x0064_6910,
            &args![this, item_value, base, 1u32, 0u32, flag_100, running],
        )
        .f32()
    } else {
        let flag_400 = e.call(0x0049_97b0, &args![this]).bool();
        let iron_sights = e.call(0x008b_bc10, &args![this]).bool();
        e.call(
            0x0064_6910,
            &args![
                this,
                item_value,
                base,
                iron_sights,
                flag_400,
                flag_100,
                running
            ],
        )
        .f32()
    };
    e.with_stack(4, |e, slot| {
        e.mem.set_f32(slot.addr(), value);
        e.call(HANDLE_ENTRY_POINT, &args![0x22u32, this, item_value, slot]);
        value = e.mem.f32(slot.addr());
    });
    match selector {
        2 => e.set(this, Actor::fGunSkillActor, value),
        0 => e.set(this, Actor::fGunSkillHUD, value),
        3 => {
            let scale = e.call(0x0040_3e20, &args![0x011c_e034u32]).u32();
            let scale = e.mem.f32(scale);
            value = (value as f64 * scale as f64) as f32;
            let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
            let base = e.vcall(process, 0x42c, &args![]).f64();
            value = (base + value as f64) as f32;
            e.set(this, Actor::fGunSkillVATS, value);
        }
        _ => {}
    }
    value
}
// Translated from 008b01c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::DoDeathStuff` (Xbox PDB): what happens when the actor dies.
///
/// Does nothing for an actor that is essential (while the setting at
/// `0x011e0888` is on), or whose animation (virtual slot `0x1E4`) has the
/// death animation kind and a 3D model that is not flagged `0x800`. On the
/// first call it records the death (`bSetOnDeath`), raises the crime alarm
/// for a killer (`pMyKiller`, with `bMurderAlarm` set and a killer that
/// answers slot `0x218`), lets the player's process react, sets the script
/// action flags `0x20` and `0x10` on the killer, stops the process's
/// movement and sets the life state to 2; on later calls it only sets the
/// life state.
pub fn actor_do_death_stuff(e: &mut Engine, this: Ptr<Actor>) {
    let animation = e.vcall(this.addr(), 0x1e4, &args![]).u32();
    if animation != 0
        && e.call(0x0049_1040, &args![animation, 0u32]).u32() != 0
        && !e.call(0x0044_0da0, &args![this]).bool()
    {
        return;
    }
    let essential_only = e.call(0x0040_8d60, &args![0x011e_0888u32]).u32();
    if e.mem.u8(essential_only) != 0 && e.call(0x0087_f3d0, &args![this]).bool() {
        return;
    }
    if e.get(this, Actor::bSetOnDeath) != 0 {
        e.call(ACTOR_SET_LIFE_STATE, &args![this, 2u32]);
        return;
    }
    e.set(this, Actor::bSetOnDeath, 1);
    let player: u32 = e.global(PLAYER_CHARACTER);
    let killer = e.get(this, Actor::pMyKiller);
    if e.get(this, Actor::bMurderAlarm) != 0
        && !killer.is_null()
        && e.vcall(killer.addr(), 0x218, &args![]).bool()
    {
        let mut witness = this.addr();
        let owner = e.call(0x0056_7790, &args![this]).u32();
        let slot_21c = e.vcall(this.addr(), 0x21c, &args![]).bool();
        if !slot_21c && owner == 0 {
            e.call(0x008c_09e0, &args![this, killer]);
        } else {
            if owner != 0 {
                if e.call(0x0040_1170, &args![owner]).u32() == 0x2a {
                    witness = e
                        .call(0x0097_0a20, &args![PROCESS_LISTS, owner, 0u32])
                        .u32();
                } else if e.call(0x0040_1170, &args![owner]).u32() == 8 {
                    witness = e
                        .call(0x0097_0b30, &args![PROCESS_LISTS, owner, 0u32, 0u32])
                        .u32();
                }
            }
            set_action_flag(e, this, killer.addr(), 0x20);
            let killer = e.get(this, Actor::pMyKiller);
            if witness != 0 && !killer.is_null() {
                e.call(0x008c_0460, &args![witness, killer, 0u32, 1u32]);
            }
            e.set(this, Actor::bMurderAlarm, 0);
        }
    }
    let killer = e.get(this, Actor::pMyKiller);
    if killer.addr() == player {
        let process = e.call(ACTOR_GET_PROCESS, &args![killer]).u32();
        // 008b03a8: the words 0, 4, 0 pushed before slot 0x1f4 (no stack words) are the
        // last three of the process call through +0xfc (RET 0x1c).
        let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
        let (x, y, z) = (
            e.mem.u32(position),
            e.mem.u32(position + 4),
            e.mem.u32(position + 8),
        );
        e.vcall(process, 0xfc, &args![killer, x, y, z, 0u32, 4u32, 0u32]);
    }
    let killer = e.get(this, Actor::pMyKiller);
    if killer.addr() == player && e.get(this, Actor::bMurderAlarm) != 0 {
        let extra = e.call(0x0041_81e0, &args![this]).u32();
        if extra != 0 {
            e.call(0x0047_ead0, &args![extra + 0x30, 0u32]);
        }
    }
    let killer = e.get(this, Actor::pMyKiller);
    set_action_flag(e, this, killer.addr(), 0x10);
    if e.call(ACTOR_GET_PROCESS, &args![this]).u32() != 0 {
        let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
        let limit: f32 = e.global(0x0101_6970);
        e.vcall(process, 0x4b4, &args![limit]);
        let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
        e.vcall(process, 0x4c0, &args![this, 0u32, 0u32, 0x7fu32]);
    }
    e.call(ACTOR_SET_LIFE_STATE, &args![this, 2u32]);
    if e.call(ACTOR_IS_IN_COMBAT, &args![this]).bool() {
        e.vcall(this.addr(), 0x434, &args![0u32]);
    }
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null() {
        e.call(0x0088_1680, &args![this, 0u32]);
        e.vcall(process.addr(), 0x214, &args![]);
        e.vcall(process.addr(), 0x234, &args![]);
    }
    e.call(0x008b_ca90, &args![this, 0u32]);
}

/// `Script::SetActionFlag` (Xbox PDB, `005ac750`, cdecl) on `target`'s
/// extra data list (`005d43c0` returns it: the object plus 0x44) with
/// `flag`.
fn set_action_flag(e: &mut Engine, this: Ptr<Actor>, target: u32, flag: u32) {
    let extra = e.call(0x005d_43c0, &args![this]).u32();
    e.call(0x005a_c750, &args![target, extra, flag]);
}
// Translated from 008b1070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Re-applies the actor's dialogue package on its process.
///
/// The package is the process's current package (virtual slot `0x20C`) or,
/// failing that, its next one (slot `0x22C`); it counts as a dialogue
/// package when its kind (`0041ca90`) is `0x1C`. For one, the two speakers
/// (`009ee040` the one who started the conversation, `008d80e0` its target)
/// have their processes pointed at each other, their lip files cancelled and
/// the package re-run or cleared as the code below spells out; the player's
/// `0x208` field is cleared when it names the package. Without a dialogue
/// package the actor's own process is reset (slots `0x674`, `0x650`,
/// `0x624`, `0x71C`). Last, when the process has an item (slot `0x148`) and
/// no slot `0x14C` item, and that item's ammo regeneration rate is not
/// positive, slot `0x3EC` of the actor is called with the item's value.
pub fn fn_008b1070(e: &mut Engine, this: Ptr<Actor>) {
    let process = e.get(this, Actor::pCurrentProcess);
    if process.is_null() {
        return;
    }
    let setting_holder: u32 = e.global(0x011d_e45c);
    if e.call(0x0047_c850, &args![setting_holder]).bool() {
        return;
    }
    let mut current = true;
    let mut target_ok = true;
    let extra = e.call(0x005d_43c0, &args![this]).u32();
    e.call(0x0042_e130, &args![extra]);
    e.vcall(process.addr(), 0x24c, &args![0u32]);
    let mut package = e.vcall(process.addr(), 0x20c, &args![]).u32();
    if package == 0 || e.call(0x0041_ca90, &args![package]).i32() != 0x1c {
        current = false;
        package = e.vcall(process.addr(), 0x22c, &args![]).u32();
    }
    if package != 0 && e.call(0x0041_ca90, &args![package]).i32() == 0x1c {
        let player: u32 = e.global(PLAYER_CHARACTER);
        if package == e.mem.u32(player + 0x208) {
            e.mem.set_u32(player + 0x208, 0);
        }
        let starter = e.call(0x009e_e040, &args![package]).u32();
        let target = e.call(0x008d_80e0, &args![package]).u32();
        let reference = e.call(0x0040_36b0, &args![package]).u32();
        if reference != 0 {
            for _ in 0..2 {
                let holder = e.call(0x005e_3fa0, &args![reference]).u32();
                let extra = e.call(0x005d_43c0, &args![holder]).u32();
                e.call(0x0042_e130, &args![extra]);
            }
            e.call(0x0096_d470, &args![PROCESS_LISTS, reference, 0u32]);
            e.call(0x0096_e870, &args![PROCESS_LISTS, reference]);
        }
        if starter != target && current {
            let other = if this.addr() == starter {
                target
            } else {
                starter
            };
            let wait_type = e.mem.u32(package + 0x8c);
            if wait_type != 0 && e.call(0x0084_e3a0, &args![wait_type]).i32() == 0xef {
                if process_of(e, other) != 0
                    && process_call(e, other, 0x20c, &args![]).u32() == package
                {
                    process_call(e, this.addr(), 0x214, &args![]);
                    return;
                }
            } else if target != 0
                && (process_of(e, target) == 0
                    || process_call(e, target, 0x20c, &args![]).u32() != package)
            {
                target_ok = false;
            }
        }
        // "SoundHandle" of the package (+0x80): stop it when valid.
        if e.call(0x00ad_8ce0, &args![package + 0x80]).bool() {
            e.call(0x009e_f530, &args![package]);
        }
        for speaker in [starter, target] {
            if speaker != 0 && e.vcall(speaker, 0x100, &args![]).bool() {
                e.call(0x0090_6050, &args![LIP_SYNC_MANAGER, speaker]);
                e.mem.set_u8(speaker + 0x80, 1);
            }
        }
        e.call(0x0067_27b0, &args![package]);
        if starter != 0 && process_of(e, starter) != 0 {
            process_call(e, starter, 0x650, &args![1u32]);
            process_call(e, starter, 0x624, &args![target]);
            e.mem.set_f32(starter + 0x74, 1.0);
            process_call(e, starter, 0x374, &args![0u32]);
            process_call(e, starter, 0x2ec, &args![0u32]);
            if current {
                process_call(e, starter, 0x214, &args![]);
            } else {
                process_call(e, starter, 0x714, &args![starter]);
            }
            if !e.vcall(starter, 0x274, &args![1u32]).bool() {
                // (The game takes this actor's process here, not the starter's.)
                process_call(e, this.addr(), 0x71c, &args![0u32]);
                if e.vcall(starter, 0x1e4, &args![]).u32() != 0 && is_idle_kind(e, starter) {
                    process_call(e, starter, 0x614, &args![0x800u32]);
                    process_call(e, starter, 0x71c, &args![0u32]);
                }
            }
        }
        if target_ok
            && target != 0
            && starter != target
            && target != player
            && process_of(e, target) != 0
        {
            if !e.call(0x0067_27b0, &args![package]).bool() {
                process_call(e, target, 0x650, &args![1u32]);
            }
            if !e.call(0x0067_27b0, &args![package]).bool() {
                process_call(e, target, 0x624, &args![starter]);
                e.mem.set_f32(target + 0x74, 1.0);
            }
            process_call(e, target, 0x374, &args![0u32]);
            process_call(e, target, 0x2ec, &args![0u32]);
            if current {
                if e.call(0x0093_44a0, &args![target]).u32() == package {
                    process_call(e, target, 0x214, &args![]);
                }
            } else {
                let target_package = process_call(e, target, 0x20c, &args![]).u32();
                if e.call(0x0093_36c0, &args![target]).bool()
                    && !e.call(0x0067_27b0, &args![package]).bool()
                {
                    if target_package != 0
                        && e.call(0x0041_ca90, &args![target_package]).i32() == 0x1c
                    {
                        process_call(e, target, 0x214, &args![]);
                    } else {
                        process_call(e, target, 0x714, &args![target]);
                    }
                }
            }
            if !e.vcall(target, 0x274, &args![1u32]).bool()
                && e.vcall(target, 0x1e4, &args![]).u32() != 0
                && is_idle_kind(e, target)
            {
                process_call(e, target, 0x614, &args![0x800u32]);
                process_call(e, target, 0x71c, &args![0u32]);
            }
        }
    } else if process_of(e, this.addr()) != 0 {
        let value = process_call(e, this.addr(), 0x674, &args![4u32]).u32();
        process_call(e, this.addr(), 0x650, &args![1u32]);
        process_call(e, this.addr(), 0x624, &args![value]);
        e.mem.set_f32(this.addr() + 0x74, 1.0);
        process_call(e, this.addr(), 0x71c, &args![0u32]);
    }
    let process = e.get(this, Actor::pCurrentProcess);
    if !process.is_null()
        && e.vcall(process.addr(), 0x14c, &args![]).u32() == 0
        && e.vcall(process.addr(), 0x148, &args![]).u32() != 0
    {
        let item = e.vcall(process.addr(), 0x148, &args![]).u32();
        let flag = e.call(0x004b_da70, &args![item, 6u32]).bool();
        let item = e.vcall(process.addr(), 0x148, &args![]).u32();
        let item_value = e.call(0x0044_ddc0, &args![item]).u32();
        let rate = e.call(0x0070_9430, &args![item_value, flag]).f64();
        let zero: f64 = e.global(ZERO_DOUBLE);
        if rate <= zero {
            let item = e.vcall(process.addr(), 0x148, &args![]).u32();
            let second = e.call(0x004b_da70, &args![item, 2u32]).bool();
            let drawn = e.call(0x008a_16d0, &args![this]).bool();
            let item = e.vcall(process.addr(), 0x148, &args![]).u32();
            let item_value = e.call(0x0044_ddc0, &args![item]).u32();
            e.vcall(this.addr(), 0x3ec, &args![item_value, drawn, second, 0u32]);
        }
    }
}

/// `actor->pCurrentProcess` through `008d8520`.
fn process_of(e: &mut Engine, actor: u32) -> u32 {
    e.call(ACTOR_GET_PROCESS, &args![actor]).u32()
}

/// Virtual call on the process of `actor`, fetched through `008d8520` first.
fn process_call(e: &mut Engine, actor: u32, slot: u32, arguments: &[u32]) -> Ret {
    let process = process_of(e, actor);
    e.vcall(process, slot, arguments)
}

/// True when the actor's slot `0x214` answers 0, 9 or 4.
fn is_idle_kind(e: &mut Engine, actor: u32) -> bool {
    e.vcall(actor, 0x214, &args![]).u32() == 0
        || e.vcall(actor, 0x214, &args![]).u32() == 9
        || e.vcall(actor, 0x214, &args![]).u32() == 4
}
// Translated from 008b1910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Records `talker` as the talking actor in its extra data list
/// (`0042e060` with the form `007af430` gives), calls its virtual slot
/// `0x48` with `0x80000000`, and reads the talking-actor extra back
/// (`0042e110`). When that exists, its word at +0xC is stored at `out` and
/// becomes the result, replaced by the form's `0x90` field (`00516bf0`) when
/// that is not null; otherwise the result is `talker`. The result is handed
/// to the process (virtual slot `0x12C`) and returned.
pub fn fn_008b1910(e: &mut Engine, this: Ptr<Actor>, talker: Ptr, out: Ptr) -> u32 {
    let form = e.call(0x007a_f430, &args![talker]).u32();
    let extra = e.call(0x005d_43c0, &args![talker]).u32();
    e.call(0x0042_e060, &args![extra, form, talker]);
    e.vcall(talker.addr(), 0x48, &args![0x8000_0000u32]);
    let extra = e.call(0x005d_43c0, &args![talker]).u32();
    let talking = e.call(0x0042_e110, &args![extra]).u32();
    let mut result = talker.addr();
    if talking != 0 {
        result = e.mem.u32(talking + 0xc);
        e.mem.set_u32(out.addr(), result);
        if e.call(0x0051_6bf0, &args![form]).u32() != 0 {
            result = e.call(0x0051_6bf0, &args![form]).u32();
        }
    }
    let process = e.get(this, Actor::pCurrentProcess);
    e.vcall(process.addr(), 0x12c, &args![result]);
    result
}

// Translated from 008b1ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit 1 of the word at `this + 0x1C` is set (the class is not
/// named in the engine map; `008b19c0` calls it on its package argument).
pub fn fn_008b1ff0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x1c) & 2 != 0
}

// Translated from 008b2010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::CanUseIdle` (Xbox PDB): true when the idle `idle` (or, if that
/// one is a generic location, the end of its chain of `008041a0` links) is
/// among the root idle array or the loose idle array of the idle manager
/// for the directory name of the actor's model. The stack-cookie check of
/// the compiler is not translated.
pub fn actor_can_use_idle(e: &mut Engine, this: Ptr<Actor>, idle: u32) -> bool {
    if idle == 0 {
        return false;
    }
    let mut idle = idle;
    while e.call(0x0080_41a0, &args![idle]).u32() != 0 {
        idle = e.call(0x0080_41a0, &args![idle]).u32();
    }
    let idle_manager: u32 = e.global(0x011c_b6a0);
    e.with_stack(264, |e, directory| {
        // TESObjectREFR::GetModel (Xbox PDB), TESIdleManager::GetIdleDirName
        let model = e.call(0x0057_15d0, &args![this]).u32();
        e.call(0x0060_07f0, &args![model, directory]);
        for finder in [0x0060_0560u32, 0x0060_0630u32] {
            let array = e.call(finder, &args![idle_manager, directory]).u32();
            if array != 0 {
                let mut index = 0u32;
                while index < e.call(0x0084_e3a0, &args![array]).u32() {
                    if e.call(0x004d_6170, &args![array, index]).u32() == idle {
                        return true;
                    }
                    index += 1;
                }
            }
        }
        false
    })
}

/// `DialoguePackage::DialoguePackage` (Xbox PDB).
const DIALOGUE_PACKAGE_CONSTRUCTOR: u32 = 0x009e_dd80;
/// `operator new`.
const OPERATOR_NEW: u32 = 0x0040_1000;

// Translated from 008b19c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts a conversation: builds a dialogue package (`DialoguePackage`,
/// package kind `0x1C`) from `source`, a package whose settings it copies
/// (location, target, topic, flags, secondary location), and runs it.
///
/// Takes the talk partner from the process (slot `0x128`; when there is none
/// the process is first asked through slot `0x8C`), either directly when it
/// is a mobile object (slot `0xFC`) or through the extra talking-actor data
/// when its form kind is `0x16`; returns false without a partner that has a
/// process. The partner and this actor both get the package (slots `0x710`
/// and `0x2F4`) unless it is the player or this actor itself. The
/// compiler's exception frame is not translated. The original also has a
/// block at `008b1afc` (testing a partner found through slot `0x300`) that
/// can never run: the partner is still null at that point, so it is left out.
pub fn fn_008b19c0(e: &mut Engine, this: Ptr<Actor>, source: u32) -> bool {
    let player: u32 = e.global(PLAYER_CHARACTER);
    let process = process_of(e, this.addr());
    e.vcall(process, 0x22c, &args![]);
    let process = e.get(this, Actor::pCurrentProcess);
    let mut talker = e.vcall(process.addr(), 0x128, &args![]).u32();
    let mut partner = 0u32;
    let mut partner_extra = 0u32;
    if talker == 0 {
        let process = process_of(e, this.addr());
        e.vcall(process, 0x8c, &args![this]);
    }
    let process = e.get(this, Actor::pCurrentProcess);
    talker = e.vcall(process.addr(), 0x128, &args![]).u32();
    if talker == 0 {
        return false;
    }
    if e.vcall(talker, 0xfc, &args![]).bool() {
        partner = talker;
    } else {
        let form = e.call(0x007a_f430, &args![talker]).u32();
        if e.call(0x0040_1170, &args![form]).u32() == 0x16 {
            let form = e.call(0x007a_f430, &args![talker]).u32();
            let extra = e.call(0x005d_43c0, &args![talker]).u32();
            e.call(0x0042_e060, &args![extra, form, talker]);
            let extra = e.call(0x005d_43c0, &args![talker]).u32();
            let talking = e.call(0x0042_e110, &args![extra]).u32();
            partner = e.mem.u32(talking + 0xc);
            partner_extra = partner;
        }
    }
    if partner == 0 || process_of(e, partner) == 0 {
        return false;
    }
    if partner != this.addr() {
        e.vcall(partner, 0x288, &args![]);
    }
    let topic = e.call(0x0067_2760, &args![source]).u32();
    let process = e.get(this, Actor::pCurrentProcess);
    e.vcall(process.addr(), 0x214, &args![]);
    if !e.call(0x0067_a690, &args![source]).bool() && !e.call(0x0067_2800, &args![source]).bool() {
        process_call(e, this.addr(), 0x644, &args![1u32]);
        process_call(e, this.addr(), 0x634, &args![partner]);
    }
    let memory = e.call(OPERATOR_NEW, &args![0xd0u32]).u32();
    let package = if memory != 0 {
        // `005f36f0` (named `ActorMover::GetPreferredMoveMode` in the map,
        // but a folded getter) is tested for zero.
        let getter_is_zero = e.call(0x005f_36f0, &args![source]).u32() == 0;
        e.call(
            DIALOGUE_PACKAGE_CONSTRUCTOR,
            &args![
                memory,
                0u32,
                this,
                partner,
                partner_extra,
                0u32,
                getter_is_zero
            ],
        )
        .u32()
    } else {
        0
    };
    if source != 0 {
        e.call(0x0067_a1b0, &args![package, source]);
    }
    // TESPackage::SetPackType (Xbox PDB)
    e.call(0x0067_0fc0, &args![package, 0x1cu32]);
    let flag = fn_008b1ff0(e, Ptr::new(source));
    e.call(0x0082_6b40, &args![package, flag]);
    let flag = e.call(0x0067_efd0, &args![source]).bool();
    e.call(0x0082_6b90, &args![package, flag]);
    e.call(0x0098_4f60, &args![package, 10u32]);
    let location = e.call(0x0055_b980, &args![source]).u32();
    // TESPackage::SetPackageLocation (Xbox PDB)
    e.call(0x0067_1d30, &args![package, location]);
    e.mem.set_u32(package + 0x8c, topic);
    if e.call(0x0067_2dd0, &args![source]).u32() != 0 {
        let second = e.call(0x0067_2dd0, &args![source]).u32();
        // TESPackage::SetPackageSecondLocation (Xbox PDB)
        e.call(0x0067_1e10, &args![package, second]);
    }
    let memory = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let target = if memory != 0 {
        // PackageTarget::PackageTarget (Xbox PDB)
        e.call(0x0067_ff70, &args![memory]).u32()
    } else {
        0
    };
    // PackageTarget::SetTargReference, SetTargType (Xbox PDB)
    e.call(0x0068_0110, &args![target, partner]);
    e.call(0x0068_00b0, &args![target, 0u32]);
    let holder = e.call(0x0067_1d10, &args![source]).u32();
    let value = e.call(0x0044_ddc0, &args![holder]).u32();
    e.call(0x0040_3550, &args![target, value]);
    // TESPackage::SetPackageTarget (Xbox PDB)
    e.call(0x0067_2fc0, &args![package, target]);
    if target != 0 {
        e.call(0x007b_3fa0, &args![target, 1u32]);
    }
    let flag = e.call(0x0067_27b0, &args![source]).bool();
    e.call(0x0067_2aa0, &args![package, flag]);
    let seconds = e.call(0x0067_2850, &args![source]).f32();
    e.call(0x0067_2c40, &args![package, seconds]);
    let topic = e.call(0x0067_2760, &args![source]).u32();
    e.call(0x0067_29d0, &args![package, topic]);
    let flag = e.call(0x0067_2800, &args![source]).bool();
    e.call(0x0067_2b70, &args![package, flag]);
    process_call(e, this.addr(), 0x28, &args![]);
    // MiddleHighProcess::SetBSBound (Xbox PDB), on the player singleton
    e.call(0x005c_8a30, &args![player, package]);
    process_call(e, this.addr(), 0x710, &args![this]);
    e.vcall(this.addr(), 0x2f4, &args![package, 0u32, 1u32]);
    if process_call(e, this.addr(), 0x280, &args![]).u32() != 1 {
        process_call(e, this.addr(), 0x238, &args![1u32]);
    }
    let process = e.get(this, Actor::pCurrentProcess);
    e.vcall(process.addr(), 0x12c, &args![partner]);
    let process = e.get(this, Actor::pCurrentProcess);
    e.call(0x0048_3710, &args![process]);
    let process = e.get(this, Actor::pCurrentProcess);
    e.call(0x0048_3710, &args![process]);
    e.call(0x0057_bd60, &args![this, partner]);
    if !e.call(0x0067_27b0, &args![source]).bool() && e.call(0x0067_2dd0, &args![source]).u32() == 0
    {
        if partner != player && partner != this.addr() {
            process_call(e, partner, 0x710, &args![partner]);
            let current = e.call(0x0093_44a0, &args![this]).u32();
            e.vcall(partner, 0x2f4, &args![current, 0u32, 1u32]);
            process_call(e, partner, 0x12c, &args![partner]);
            process_call(e, partner, 0x288, &args![partner, 1u32]);
            e.call(0x0057_bd60, &args![partner, this]);
        } else if !e.vcall(partner, 0x22c, &args![0u32]).bool()
            && partner != player
            && partner != this.addr()
        {
            process_call(e, this.addr(), 0x288, &args![this, 3u32]);
            return true;
        }
    }
    let process = e.get(this, Actor::pCurrentProcess);
    let result = e.vcall(process.addr(), 0x90, &args![this, 1u32]).u32();
    e.mem.set_u32(package + 0x9c, result);
    true
}
/// `PackageLocation::PackageLocation` (Xbox PDB).
const PACKAGE_LOCATION_CONSTRUCTOR: u32 = 0x0067_f030;
/// The destructor of a `PackageLocation` (`0067f110`).
const PACKAGE_LOCATION_DESTRUCTOR: u32 = 0x0067_f110;
/// `Conversation::Conversation_ov2` (Xbox PDB).
const CONVERSATION_CONSTRUCTOR: u32 = 0x0083_b850;

// Translated from 008b2170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts a scripted conversation of this actor with `partner`: builds a
/// `Conversation` (unless the partner is the player) and a dialogue package
/// (`DialoguePackage`, kind `0x1C`) configured from the arguments and from
/// the process's current package (slot `0x22C`), makes both actors run it
/// and returns true.
///
/// Returns false without a partner or a partner process, when the partner is
/// a mobile object that is trying to enter furniture, when this actor is
/// dismembered in limb 1 or 2 while slot `0x22C` answers true, and when the
/// partner (not the player) is not given the package. `location` and
/// `second_location` (null or `PackageLocation`s) are copied into the
/// package's locations; the second one gets its radius from `00676280` plus
/// a game setting. `topic` is stored in the package (+0x8C) and set as its
/// dialogue topic. The compiler's exception frame is not translated.
#[allow(clippy::too_many_arguments)]
pub fn fn_008b2170(
    e: &mut Engine,
    this: Ptr<Actor>,
    partner: Ptr<Actor>,
    location: Ptr,
    second_location: Ptr,
    head_track: bool,
    option_a: bool,
    option_b: bool,
    topic: u32,
    option_c: bool,
    option_d: bool,
) -> bool {
    let player: u32 = e.global(PLAYER_CHARACTER);
    if process_of(e, this.addr()) == 0 {
        return false;
    }
    let current_package = process_call(e, this.addr(), 0x22c, &args![]).u32();
    let mut mobile = 0u32;
    if !partner.is_null() && e.vcall(partner.addr(), 0x100, &args![]).bool() {
        mobile = partner.addr();
    }
    if partner.is_null()
        || (mobile != 0 && e.call(0x008c_13d0, &args![mobile]).bool())
        || process_of(e, partner.addr()) == 0
    {
        return false;
    }
    if e.vcall(this.addr(), 0x22c, &args![0u32]).bool()
        && (e.call(0x0057_3090, &args![this, 1u32]).bool()
            || e.call(0x0057_3090, &args![this, 2u32]).bool())
    {
        return false;
    }
    let mut conversation = 0u32;
    if partner.addr() != player {
        let memory = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
        conversation = if memory != 0 {
            e.call(
                CONVERSATION_CONSTRUCTOR,
                &args![memory, this, partner, topic],
            )
            .u32()
        } else {
            0
        };
    }
    // MobileObject::StopCurrentDialogue (Xbox PDB)
    e.call(0x0093_4250, &args![this]);
    let process = e.get(this, Actor::pCurrentProcess);
    e.vcall(process.addr(), 0x214, &args![]);
    if partner.addr() != player && !e.call(0x0083_b9a0, &args![conversation]).bool() {
        // Conversation::FirstItem found nothing: drop the conversation.
        if conversation != 0 {
            e.call(0x0079_8450, &args![conversation, 1u32]);
        }
        return false;
    }
    if head_track {
        process_call(e, this.addr(), 0x644, &args![1u32]);
        process_call(e, this.addr(), 0x634, &args![partner]);
        process_call(e, partner.addr(), 0x644, &args![1u32]);
        process_call(e, partner.addr(), 0x634, &args![this]);
    }
    let memory = e.call(OPERATOR_NEW, &args![0xd0u32]).u32();
    let package = if memory != 0 {
        e.call(
            DIALOGUE_PACKAGE_CONSTRUCTOR,
            &args![memory, conversation, this, partner, 0u32, 1u32, 1u32],
        )
        .u32()
    } else {
        0
    };
    e.mem.set_u32(package + 0x8c, topic);
    if current_package != 0 {
        e.call(0x0067_a1b0, &args![package, current_package]);
    }
    e.call(0x0067_0fc0, &args![package, 0x1cu32]);
    e.call(0x0082_6b40, &args![package, 1u32]);
    e.call(0x0082_6b90, &args![package, 1u32]);
    e.call(0x0067_29d0, &args![package, topic]);
    fn_008b2860(e, Ptr::new(package), option_a as u8);
    fn_008b2880(e, Ptr::new(package), option_c as u8);
    e.call(0x0067_2aa0, &args![package, option_c]);
    e.call(0x0067_a6b0, &args![package, !head_track]);
    fn_008b28a0(e, Ptr::new(package), option_d as u8);
    // Two `PackageLocation`s on the stack, 0xC bytes each: the exe's frame (`SUB ESP,0xb4`
    // under the exception record, then the record link) has them at `EBP-0x34` (+0x90) and
    // `EBP-0x28` (+0x9c).
    e.with_stack(0xc4, |e, frame| {
        let (first, second) = (frame.addr() + 0x90, frame.addr() + 0x9c);
        e.call(PACKAGE_LOCATION_CONSTRUCTOR, &args![first]);
        if !location.is_null() {
            e.call(0x0067_f4d0, &args![first, location, 1u32]);
        }
        e.call(0x0067_f3c0, &args![first, this]);
        e.call(0x0067_1d30, &args![package, first]);
        e.call(PACKAGE_LOCATION_CONSTRUCTOR, &args![second]);
        if !second_location.is_null() {
            e.call(0x0067_f4d0, &args![second, second_location, 1u32]);
        }
        let result = 'conversation: {
            let distance = e.call(0x0067_6280, &args![package, this]).u32();
            let setting = e.call(0x0040_3e20, &args![0x011c_cf38u32]).u32();
            let setting = e.mem.f32(setting);
            let radius = (setting as f64 + distance as f64) as i64 as u32;
            e.call(0x0067_f1f0, &args![second, radius]);
            e.call(0x0067_1e10, &args![package, second]);
            let memory = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
            let target = if memory != 0 {
                e.call(0x0067_ff70, &args![memory]).u32()
            } else {
                0
            };
            e.call(0x0067_2fc0, &args![package, target]);
            if target != 0 {
                e.call(0x007b_3fa0, &args![target, 1u32]);
            }
            e.call(0x0098_4f60, &args![package, 10u32]);
            let holder = e.call(0x0067_1d10, &args![package]).u32();
            e.call(0x0068_00b0, &args![holder, 0u32]);
            let holder = e.call(0x0067_1d10, &args![package]).u32();
            e.call(0x0068_0110, &args![holder, partner]);
            let kind = process_call(e, this.addr(), 0x4bc, &args![]).u32();
            let holder = e.call(0x0067_1d10, &args![package]).u32();
            e.call(
                0x0040_3550,
                &args![holder, if kind == 4 { 0xc8u32 } else { 0x5au32 }],
            );
            process_call(e, this.addr(), 0x28, &args![]);
            e.vcall(this.addr(), 0x2f4, &args![package, 1u32, 1u32]);
            e.call(0x0057_bd60, &args![this, partner]);
            let process = e.get(this, Actor::pCurrentProcess);
            if option_a {
                e.vcall(process.addr(), 0x288, &args![this, 2u32]);
            } else if option_b {
                e.vcall(process.addr(), 0x288, &args![this, 1u32]);
            }
            let process = e.get(this, Actor::pCurrentProcess);
            e.call(0x0048_3710, &args![process]);
            let process = e.get(this, Actor::pCurrentProcess);
            e.call(0x0048_3710, &args![process]);
            if partner.addr() != player && partner != this && (!option_c || option_d) {
                e.vcall(partner.addr(), 0x2f4, &args![package, 1u32, 1u32]);
                if e.call(0x0093_44a0, &args![partner]).u32() != package {
                    e.vcall(this.addr(), 0x288, &args![]);
                    break 'conversation false;
                }
                e.call(0x0057_bd60, &args![partner, this]);
                if option_a {
                    process_call(e, partner.addr(), 0x288, &args![partner, 2u32]);
                } else {
                    process_call(e, partner.addr(), 0x288, &args![partner, 1u32]);
                }
            }
            true
        };
        e.call(PACKAGE_LOCATION_DESTRUCTOR, &args![second]);
        e.call(PACKAGE_LOCATION_DESTRUCTOR, &args![first]);
        result
    })
}

// Translated from 008b2860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at +0xBE of a `DialoguePackage`.
pub fn fn_008b2860(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0xbe, value);
}

// Translated from 008b2880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at +0xBF of a `DialoguePackage`.
pub fn fn_008b2880(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0xbf, value);
}

// Translated from 008b28a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at +0xC0 of a `DialoguePackage`.
pub fn fn_008b28a0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0xc0, value);
}

// Translated from 008b2b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds 3 to `base` when `animation` is (or is not) the player's
/// first-person animation and `item` agrees (`item` null, or for the
/// first-person animation [`fn_008b2bf0`] / otherwise [`fn_008b2c10`] true).
/// Returns `base` unchanged when `item` is set and its value (`00508070`,
/// a float) is not positive. `this` (ECX) is not read; the caller
/// (`008b28c0`) passes the equipped item's value there.
pub fn fn_008b2b60(e: &mut Engine, _unused_this: Ptr, animation: u32, base: i32, item: Ptr) -> i32 {
    if !item.is_null() {
        let value = e.call(0x0050_8070, &args![item]).f64();
        let zero: f64 = e.global(ZERO_DOUBLE);
        if !(value > zero || value.is_nan()) {
            return base;
        }
    }
    let player: u32 = e.global(PLAYER_CHARACTER);
    let first_person = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
    if animation == first_person && (item.is_null() || fn_008b2bf0(e, item)) {
        return base.wrapping_add(3);
    }
    let first_person = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
    if animation != first_person && (item.is_null() || fn_008b2c10(e, item)) {
        return base.wrapping_add(3);
    }
    base
}

// Translated from 008b2bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit `0x40` of the byte at `this + 0x100` is clear.
pub fn fn_008b2bf0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x100) & 0x40 == 0
}

// Translated from 008b2c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when bit `0x100` of the word at `this + 0x12C` is clear.
pub fn fn_008b2c10(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x12c) & 0x100 == 0
}
// Translated from 008b28c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Plays an animation group (and, for attack and aim actions, up to two
/// follow-up groups) on `animation` and tells the process (slot `0x424`).
///
/// `anim_group` is the requested group; for attack and aim actions
/// (`005f2540`, `005f2630`) three groups are tried (`anim_group + 0`, `+1`,
/// `+2`), otherwise one. With an item that has iron sights the group is
/// adjusted by `008b2b60` first. The first group is looked up with
/// `Actor::GetAnimGroup` (`00897910`); later ones continue from the best
/// found (`00495740`). A group found is played (`00494740`) and, unless
/// `animation` is the player's first-person animation, reported to the
/// process; none found clears group `i + 4` (`00496080`). Finally the process
/// is told it is done (slot `0x428`).
pub fn fn_008b28c0(e: &mut Engine, this: Ptr<Actor>, anim_group: u32, animation: Ptr) {
    let mut anim_group = anim_group;
    let player: u32 = e.global(PLAYER_CHARACTER);
    let mut count = 3i32;
    let mut best: u16 = 0xff;
    let attack_or_aim = e.call(0x005f_2540, &args![anim_group & 0xffff]).bool()
        || e.call(0x005f_2630, &args![anim_group & 0xffff]).bool();
    if !attack_or_aim {
        count = 1;
    }
    let mut i = 0i32;
    while i < count {
        let mut group = anim_group;
        let mut found: u16 = 0xff;
        let process = e.get(this, Actor::pCurrentProcess);
        let item_value = if e.vcall(process.addr(), 0x148, &args![]).u32() != 0 {
            let item = e.vcall(process.addr(), 0x148, &args![]).u32();
            e.call(0x0044_ddc0, &args![item]).u32()
        } else {
            0
        };
        if group != 0xff {
            if attack_or_aim && e.call(ACTOR_GET_IRON_SIGHTS, &args![this]).bool() {
                group = fn_008b2b60(
                    e,
                    this.cast(),
                    animation.addr(),
                    anim_group as i32,
                    Ptr::new(item_value),
                ) as u32;
                group = group.wrapping_add(i as u32);
            } else {
                group = anim_group.wrapping_add(i as u32);
            }
            if i == 0 {
                // Actor::GetAnimGroup (Xbox PDB)
                found = e
                    .call(0x0089_7910, &args![this, group, 0u32, 0u32, animation])
                    .u16();
                best = found;
                if attack_or_aim && e.call(0x005f_2750, &args![group]).bool() {
                    let kind = e.call(0x005f_2440, &args![found as u32]).u32();
                    if kind == group.wrapping_sub(3) {
                        group = group.wrapping_sub(3);
                        anim_group = anim_group.wrapping_sub(3);
                    }
                }
            } else if best != 0xff {
                best = best.wrapping_add(1);
                // Animation::PickBestAnimation (Xbox PDB)
                found = e
                    .call(0x0049_5740, &args![animation, best as u32, 0u32])
                    .u16();
            }
            if e.call(0x005f_2440, &args![found as u32]).u32() != group {
                found = 0xff;
            }
        }
        let first_person =
            |e: &mut Engine| e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
        if found != 0xff {
            // Animation::PlayGroup (Xbox PDB)
            let played = e
                .call(
                    0x0049_4740,
                    &args![animation, found as u32, 1u32, -1i32, -1i32],
                )
                .u32();
            if animation.addr() != first_person(e) {
                process_call(e, this.addr(), 0x424, &args![i, played]);
            }
        } else {
            // Animation::ClearGroup (Xbox PDB)
            e.call(0x0049_6080, &args![animation, i + 4, 0.0f32]);
            if animation.addr() != first_person(e) {
                process_call(e, this.addr(), 0x424, &args![i, 0u32]);
            }
        }
        i += 1;
    }
    let process = e.get(this, Actor::pCurrentProcess);
    e.vcall(process.addr(), 0x428, &args![this]);
}

// Translated from 008b2c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales the rotation of the player's weapon node by the weapon's cached
/// value (`008b0dd0` with selector 2, times a game setting unless `00894900`
/// says otherwise). Only runs for the player with an equipped item that
/// answers slot `0x454` of the process: finds the node (`008d6970`,
/// `004aae30`), makes sure it has a blend collision object
/// (`bhkBlendCollisionObjectAddRotation`, copied from an existing one of
/// the type at `0x12681cc`), then takes the Euler angles of the matrix built
/// from the animation's float at +0xD0, multiplies them by the scale and
/// sets the product with the identity-based matrix `0x011a9448` on the
/// collision object (`004f0110`). The compiler's exception frame is not
/// translated.
pub fn fn_008b2c30(e: &mut Engine, this: Ptr<Actor>) {
    let player: u32 = e.global(PLAYER_CHARACTER);
    if this.addr() != player {
        return;
    }
    let process = e.get(this, Actor::pCurrentProcess);
    if process.is_null() {
        return;
    }
    if e.vcall(process.addr(), 0x148, &args![]).u32() == 0 {
        return;
    }
    let process = e.get(this, Actor::pCurrentProcess);
    if !e.vcall(process.addr(), 0x454, &args![]).bool() {
        return;
    }
    let process = e.get(this, Actor::pCurrentProcess);
    let item = e.vcall(process.addr(), 0x148, &args![]).u32();
    let item_value = e.call(0x0044_ddc0, &args![item]).u32();
    // Three matrices of 9 floats (the rotation M, the constant copy R and a
    // product) and three Euler angles.
    e.with_stack(0x78, |e, block| {
        let matrix = block.addr();
        let constant = matrix + 0x24;
        let product = matrix + 0x48;
        let angles = matrix + 0x6c;
        e.call(0x0068_15c0, &args![matrix]);
        let animation = e.call(ACTOR_GET_ANIMATION, &args![this]).u32();
        // Float at animation +0xD0, handed on as an argument (the matrix
        // address stays on the stack for the call after).
        let rotation_value = e.call(0x0045_3700, &args![animation]).f32();
        let index = e.call(0x0044_6390, &args![item_value]).i32();
        let table_entry = e
            .mem
            .u32(0x0118_a838u32.wrapping_add((index as u32).wrapping_mul(4)));
        let node_name = e
            .call(0x008d_6970, &args![table_entry, rotation_value, matrix])
            .u32();
        if node_name == 0 {
            return;
        }
        let root = e.call(0x0043_fcd0, &args![this]).u32();
        let node = e.call(0x004a_ae30, &args![root, node_name]).u32();
        if node == 0 {
            return;
        }
        let mut scale = fn_008b0dd0(e, this, 2);
        if !e.call(0x0089_4900, &args![this]).bool() {
            let setting = e.call(0x0040_3e20, &args![0x011c_ec34u32]).u32();
            scale = (scale as f64 * e.mem.f32(setting) as f64) as f32;
        }
        let collision = e.call(0x0068_38b0, &args![node]).u32();
        if collision == 0 {
            let created = new_blend_collision_object(e);
            e.call(0x0062_bc90, &args![node, created]);
        } else {
            let collision = e.call(0x0068_38b0, &args![node]).u32();
            let source = e.call(0x0065_3270, &args![0x0126_7e64u32, collision]).u32();
            if source != 0 && e.call(0x0065_3270, &args![0x0126_81ccu32, source]).u32() == 0 {
                let created = new_blend_collision_object(e);
                e.call(0x00c9_0090, &args![created, source]);
                e.call(0x0062_bc90, &args![node, created]);
            }
        }
        let collision = e.call(0x0068_38b0, &args![node]).u32();
        let blend = e.call(0x0065_3270, &args![0x0126_81ccu32, collision]).u32();
        if blend == 0 {
            return;
        }
        for word in 0..9 {
            let value = e.mem.u32(0x011a_9448 + 4 * word);
            e.mem.set_u32(constant + 4 * word, value);
        }
        e.call(0x00a5_92c0, &args![matrix, angles, angles + 4, angles + 8]);
        let first = (e.mem.f32(angles) as f64 * scale as f64) as f32;
        let second = (e.mem.f32(angles + 4) as f64 * scale as f64) as f32;
        let third = (e.mem.f32(angles + 8) as f64 * scale as f64) as f32;
        e.call(0x00a5_9540, &args![matrix, first, second, third]);
        let result = e.call(0x0043_f8d0, &args![constant, product, matrix]).u32();
        for word in 0..9 {
            let value = e.mem.u32(result + 4 * word);
            e.mem.set_u32(constant + 4 * word, value);
        }
        e.call(0x004f_0110, &args![blend, constant]);
    });
}

/// A new `bhkBlendCollisionObjectAddRotation`: 0x50 bytes (`00aa13e0`)
/// constructed by `00c8ffd0`, or null when the allocation fails.
fn new_blend_collision_object(e: &mut Engine) -> u32 {
    let memory = e.call(0x00aa_13e0, &args![0x50u32]).u32();
    if memory != 0 {
        e.call(0x00c8_ffd0, &args![memory]).u32()
    } else {
        0
    }
}
// Translated from 008b2f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsAtPoint` (Xbox PDB): true when `point` is within `radius` of
/// the actor, `00439ef0` taking the vector from the point to the actor's
/// position (virtual slot `0x1F4`) and `004b5470` comparing it with the
/// radius (0 or less is "within").
///
/// The vertical part of the vector is dropped when the point lies within
/// 30 units of the actor's height (its height, `008853a0`, or 128 when that
/// is 0) — unless `force_height_window` is false and the character
/// controller (`005c0880`) says to drop it anyway. With `add_actor_radius`
/// the radius is increased by the controller's radius (`00c6e280`, through
/// `004587d0`; doubled when `008a5170` says so, at least 32) or by 32
/// without a controller.
pub fn actor_is_at_point(
    e: &mut Engine,
    this: Ptr<Actor>,
    point: Ptr,
    radius: f32,
    add_actor_radius: bool,
    force_height_window: bool,
) -> bool {
    let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
    let controller = e.call(0x0093_06d0, &args![this]).u32();
    let mut radius = radius;
    // The actor's position (12 bytes) and the vector from the point to it.
    e.with_stack(0x18, |e, block| {
        let (actor_position, delta) = (block.addr(), block.addr() + 0xc);
        for word in 0..3 {
            let value = e.mem.u32(position + 4 * word);
            e.mem.set_u32(actor_position + 4 * word, value);
        }
        e.call(0x0043_9ef0, &args![actor_position, delta, point]);
        let zero: f64 = e.global(ZERO_DOUBLE);
        if !force_height_window
            && controller != 0
            && e.call(0x005c_0880, &args![controller]).u32() != 0
        {
            e.mem.set_f32(delta + 8, 0.0);
        } else {
            let mut height = e.call(0x0088_53a0, &args![this]).f32();
            if height as f64 == zero {
                height = e.global(0x0101_e704);
            }
            let margin: f64 = e.global(0x0101_db88);
            let z = e.mem.f32(actor_position + 8);
            let low = (z as f64 - margin) as f32;
            let high = ((z as f64 + height as f64) + margin) as f32;
            let point_z = e.mem.f32(point.addr() + 8);
            if low < point_z && point_z < high {
                e.mem.set_f32(delta + 8, 0.0);
            }
        }
        if add_actor_radius {
            let mut extra: f32 = e.global(0x0101_e340);
            if controller != 0 {
                let controller_radius = e.call(0x00c6_e280, &args![controller]).f32();
                extra = e.call(0x0045_87d0, &args![controller_radius]).f32();
                if e.call(0x008a_5170, &args![controller + 0x410]).bool() {
                    extra = (extra as f64 + extra as f64) as f32;
                }
                let minimum: f64 = e.global(0x0102_f070);
                if (extra as f64) < minimum {
                    extra = e.global(0x0101_e340);
                }
            }
            radius = (radius as f64 + extra as f64) as f32;
        }
        let result = e.call(0x004b_5470, &args![delta, radius]).i32();
        result <= 0
    })
}

// Translated from 008b30f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the actor's `ActorMover` (0x88 bytes, `009dad00`) and stores it in
/// `pActorMover`. The compiler's exception frame is not translated.
pub fn fn_008b30f0(e: &mut Engine, this: Ptr<Actor>) {
    let memory = e.call(OPERATOR_NEW, &args![0x88u32]).u32();
    let mover = if memory != 0 {
        e.call(0x009d_ad00, &args![memory, this]).u32()
    } else {
        0
    };
    e.set(this, Actor::pActorMover, Ptr::new(mover));
}

// Translated from 008b3180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::DestroyActorMover` (Xbox PDB): when the actor has extra data
/// (`005d43c0`), tells the path manager to forget the inventory pointers of
/// the actor's container changes (`00418520`, `0047d0b0`, `006ebd10`); then
/// deletes the `ActorMover` (virtual destructor, slot 0, with the delete
/// flag) and clears `pActorMover`.
pub fn actor_destroy_actor_mover(e: &mut Engine, this: Ptr<Actor>) {
    if e.call(0x005d_43c0, &args![this]).u32() != 0 {
        let extra = e.call(0x005d_43c0, &args![this]).u32();
        let changes = e.call(0x0041_8520, &args![extra]).u32();
        // 008b31a5: `0047d0b0` takes no stack word (plain RET); the pushed changes are
        // the word of `006ebd10` (RET 4).
        let manager = e.call(0x0047_d0b0, &args![]).u32();
        e.call(0x006e_bd10, &args![manager, changes]);
    }
    let mover = e.get(this, Actor::pActorMover);
    if !mover.is_null() {
        e.vcall(mover.addr(), 0, &args![1u32]);
        e.set(this, Actor::pActorMover, Ptr::NULL);
    }
}

// Translated from 008b3200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes `value` on to the `ActorMover` (`009dc950`) when there is one.
pub fn fn_008b3200(e: &mut Engine, this: Ptr<Actor>, value: u32) {
    let mover = e.get(this, Actor::pActorMover);
    if !mover.is_null() {
        e.call(0x009d_c950, &args![mover, value]);
    }
}

/// The path-finding argument that is 0 means "the process's slot `0x254`
/// for this actor".
fn default_pathing_argument(e: &mut Engine, this: Ptr<Actor>, value: u32) -> u32 {
    if value != 0 {
        return value;
    }
    process_call(e, this.addr(), 0x254, &args![this]).u32()
}

// Translated from 008b3630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetPathfindingGoal` (Xbox PDB): for a goal that fails `0069e390`,
/// passes the process's slot `0x254` result to `006d61e0` of the goal; then
/// hands the goal to the `ActorMover` (`009dba30`) and answers its value (a
/// `bool` in the Xbox PDB: the exe ends on that call and leaves its `EAX`,
/// whose `AL` the callers test).
pub fn actor_set_pathfinding_goal(e: &mut Engine, this: Ptr<Actor>, goal: u32) -> bool {
    if goal != 0 && e.call(0x0069_e390, &args![goal]).u32() == 0 {
        let value = process_call(e, this.addr(), 0x254, &args![this]).u32();
        e.call(0x006d_61e0, &args![goal, value]);
    }
    let mover = e.get(this, Actor::pActorMover);
    e.call(0x009d_ba30, &args![mover, goal]).bool()
}

// Translated from 008b3690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetPathfindingGoal_ov2` (Xbox PDB): `ActorMover::SetGoal`
/// (`009dba50`) with `last` defaulting to the process's slot `0x254` result,
/// answering its value (the exe ends on that call and leaves its `EAX`; the
/// callers test `AL`).
pub fn actor_set_pathfinding_goal_ov2(
    e: &mut Engine,
    this: Ptr<Actor>,
    first: u32,
    second: u32,
    third: u32,
    fourth: f32,
    last: u32,
) -> bool {
    let last = default_pathing_argument(e, this, last);
    let mover = e.get(this, Actor::pActorMover);
    e.call(
        0x009d_ba50,
        &args![mover, first, second, third, fourth, last],
    )
    .bool()
}

// Translated from 008b36f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetPathfindingGoal_ov3` (Xbox PDB): `009dbb10` of the
/// `ActorMover`, with `last` defaulting to the process's slot `0x254` result,
/// answering its value (the exe ends on that call and leaves its `EAX`; the
/// caller tests `AL`).
pub fn actor_set_pathfinding_goal_ov3(
    e: &mut Engine,
    this: Ptr<Actor>,
    first: u32,
    second: f32,
    last: u32,
) -> bool {
    let last = default_pathing_argument(e, this, last);
    let mover = e.get(this, Actor::pActorMover);
    e.call(0x009d_bb10, &args![mover, first, second, last])
        .bool()
}

// Translated from 008b3750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `009dbb60` of the `ActorMover`, with `last` defaulting to the process's
/// slot `0x254` result, answering its value (the exe ends on that call and
/// leaves its `EAX`; the caller tests `AL`).
#[allow(clippy::too_many_arguments)]
pub fn fn_008b3750(
    e: &mut Engine,
    this: Ptr<Actor>,
    first: u32,
    second: u32,
    third: u32,
    fourth: f32,
    fifth: f32,
    last: u32,
) -> bool {
    let last = default_pathing_argument(e, this, last);
    let mover = e.get(this, Actor::pActorMover);
    e.call(
        0x009d_bb60,
        &args![mover, first, second, third, fourth, fifth, last],
    )
    .bool()
}

// Translated from 008b37c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `009dbc30` of the `ActorMover`, with `last` defaulting to the process's
/// slot `0x254` result.
pub fn fn_008b37c0(e: &mut Engine, this: Ptr<Actor>, first: u32, second: f32, last: u32) {
    let last = default_pathing_argument(e, this, last);
    let mover = e.get(this, Actor::pActorMover);
    e.call(0x009d_bc30, &args![mover, first, second, last]);
}

// Translated from 008b3230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame update of the actor's movement speed and creature sound, for
/// every actor but the player (inside a profiling scope guard,
/// `00404eb0`/`00404ee0`, for `Actor.cpp` line `0x5D06`).
///
/// With a process: the elapsed time (`delta`, or when 0 and the process type
/// is 0 the setting at `0x011f6394`, times the VATS update multiplier for
/// the VATS target) is multiplied by the animation's float (`004e3d00`) when
/// slot 8 of the `MagicTarget` base answers a positive value for `0x33`,
/// and handed to the `ActorMover` (slot `0x14`).
///
/// A creature-type actor (virtual slot `0x21C`; form kind 6 or 7) with a
/// creature sound (`TESCreature::PickCreatureSound(10)`, `005f92d0`) and
/// something to follow (slot `0x1D0`) plays that sound while it should (the
/// flag from `004938e0`/`008a3b30` and slot `0x22C`) and stops it when it
/// should not, keeping the `BSSoundHandle` in its extra data (`00418980`
/// loads it, `0041a280` stores it). The compiler's exception frame is not
/// translated.
pub fn fn_008b3230(e: &mut Engine, this: Ptr<Actor>, delta: f32) {
    // The exe's frame (`SUB ESP,0x74` under the exception record, then the record link): the
    // scope at `EBP-0x10` (+0x74); the body's locals are in the same frame.
    e.with_stack(0x84, |e, frame| {
        let guard = frame.addr() + 0x74;
        e.call(
            0x0040_4eb0,
            &args![guard, 0x2du32, 1u32, 0x0108_4918u32, 0x5d06u32],
        );
        actor_frame_update(e, frame.addr(), this, delta);
        e.call(0x0040_4ee0, &args![guard]);
    });
}

/// The body of [`fn_008b3230`] between the scope guard's calls.
fn actor_frame_update(e: &mut Engine, frame: u32, this: Ptr<Actor>, delta: f32) {
    let player: u32 = e.global(PLAYER_CHARACTER);
    if this.addr() == player {
        return;
    }
    if process_of(e, this.addr()) != 0 {
        let zero: f64 = e.global(ZERO_DOUBLE);
        let mut elapsed = delta;
        if elapsed as f64 == zero && e.call(0x0093_1850, &args![this]).u32() == 0 {
            let mode = e.call(0x0044_ddc0, &args![0x011f_2250u32]).i32();
            let vats_target: u32 = e.global(0x011f_21cc);
            if mode == 4 && this.addr() == vats_target {
                let setting = e.call(0x0084_d030, &args![0x011f_6394u32]).f64();
                let multiplier = e.call(0x009c_8d60, &args![0x011f_2250u32]).f64();
                elapsed = (multiplier * setting) as f32;
            } else {
                elapsed = e.call(0x0084_d030, &args![0x011f_6394u32]).f32();
            }
        }
        if e.vcall(this.addr() + 0xa4, 8, &args![0x33u32]).i32() > 0
            && e.vcall(this.addr(), 0x1e4, &args![]).u32() != 0
        {
            let animation = e.vcall(this.addr(), 0x1e4, &args![]).u32();
            let factor = e.call(0x004e_3d00, &args![animation]).f64();
            elapsed = (factor * elapsed as f64) as f32;
        }
        let mover = e.get(this, Actor::pActorMover);
        e.vcall(mover.addr(), 0x14, &args![elapsed]);
    }
    if !e.vcall(this.addr(), 0x21c, &args![]).bool() {
        return;
    }
    let form = e.call(0x007a_f430, &args![this]).u32();
    let kind: i8 = if form == 0 {
        -1
    } else {
        e.call(0x0059_f3a0, &args![form]).u8() as i8
    };
    if kind != 6 && kind != 7 {
        return;
    }
    let sound = if form != 0 {
        e.call(0x005f_92d0, &args![form, 0xau32]).u32()
    } else {
        0
    };
    if sound == 0 || e.vcall(this.addr(), 0x1d0, &args![]).u32() == 0 {
        return;
    }
    let should_play = (e.call(0x0049_38e0, &args![this]).bool()
        || e.call(0x008a_3b30, &args![this]).bool())
        && !e.vcall(this.addr(), 0x22c, &args![0u32]).bool();
    // The sound handle (16 bytes), a copy of the creature sound's 36-byte
    // data, and the temporary handle the audio system fills, at `EBP-0x30`, `EBP-0x54` and
    // `EBP-0x60` of the caller's frame.
    let handle = frame + 0x54;
    let sound_data = frame + 0x30;
    let temporary = frame + 0x24;
    e.call(0x0041_a250, &args![handle]);
    let extra = e.call(0x005d_43c0, &args![this]).u32();
    e.call(0x0041_8980, &args![extra, handle]);
    let store = |e: &mut Engine| {
        let extra = e.call(0x005d_43c0, &args![this]).u32();
        e.call(0x0041_a280, &args![extra, handle]);
    };
    if should_play && !e.call(0x00ad_8930, &args![handle]).bool() {
        if !e.call(0x00ad_8ce0, &args![handle]).bool() {
            let data = e.call(0x004e_75d0, &args![sound, sound_data]).u32();
            let first = e.mem.u32(data + 4);
            let flags = e.call(0x005e_39b0, &args![first]).u32() | 2;
            let file_name = e.call(0x0051_1840, &args![sound]).u32();
            let audio = e.call(0x0045_3a70, &args![]).u32();
            let created = e
                .call(
                    0x00ad_7480,
                    &args![audio, temporary, file_name, flags, sound],
                )
                .u32();
            e.call(0x0041_8900, &args![handle, created]);
            e.call(0x0048_3710, &args![temporary]);
            e.call(0x004f_15a0, &args![handle, 1u32]);
            store(e);
            let follow = e.vcall(this.addr(), 0x1d0, &args![]).u32();
            e.call(0x00ad_8f20, &args![handle, follow]);
        }
        if !e.call(0x00ad_8930, &args![handle]).bool() {
            let position = e.vcall(this.addr(), 0x1f4, &args![]).u32();
            e.call(0x0068_a7d0, &args![handle, position]);
            e.call(0x00ad_8830, &args![handle, 0u32]);
            store(e);
        }
    } else if !should_play && e.call(0x00ad_8930, &args![handle]).bool() {
        e.call(0x00ad_88f0, &args![handle]);
        store(e);
    }
    if e.vcall(this.addr(), 0x22c, &args![0u32]).bool()
        && e.call(0x00ad_8930, &args![handle]).bool()
    {
        e.call(0x00ad_88f0, &args![handle]);
        e.call(0x00ad_8d10, &args![handle]);
        store(e);
    }
    e.call(0x0048_3710, &args![handle]);
}

/// `ActorMover` (Xbox PDB) of the actor: `pActorMover` (PC +0x190).
fn actor_mover(e: &mut Engine, this: Ptr<Actor>) -> Ptr {
    e.get(this, Actor::pActorMover)
}

/// `ActorMover::bStopMovementNextUpdate` (Xbox PDB +0x72), `bFaceTargetPoint`
/// (+0x73) and `bWaitingForPath` (+0x74): byte fields of the mover.
const MOVER_STOP_MOVEMENT_NEXT_UPDATE: u32 = 0x72;
const MOVER_FACE_TARGET_POINT: u32 = 0x73;
const MOVER_WAITING_FOR_PATH: u32 = 0x74;

// Translated from 008b3820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetCurrentPathingRequest` (Xbox PDB): `006838b0` of the
/// `ActorMover`, its result passed through.
pub fn actor_get_current_pathing_request(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x0068_38b0, &args![mover]).u32()
}

// Translated from 008b3840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetCurrentPathfindingGoal` (Xbox PDB): `009de2b0` of the
/// `ActorMover`, its result passed through.
pub fn actor_get_current_pathfinding_goal(e: &mut Engine, this: Ptr<Actor>, goal: u32) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x009d_e2b0, &args![mover, goal]).u32()
}

// Translated from 008b3860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `009de330` of the `ActorMover`, its result passed through.
pub fn fn_008b3860(e: &mut Engine, this: Ptr<Actor>, argument: u32) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x009d_e330, &args![mover, argument]).u32()
}

// Translated from 008b3880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::BuildRequest` (Xbox PDB): `009dbc90` with the actor as its first
/// word; when the actor's in-combat byte (`00493bb0`) is set, `008ad8f0(target, 0)`
/// follows on the first argument.
#[allow(clippy::too_many_arguments)]
pub fn actor_build_request(
    e: &mut Engine,
    this: Ptr<Actor>,
    target: u32,
    second: u32,
    third: u32,
    fourth: u32,
    value: f32,
    last: u32,
) {
    e.call(
        0x009d_bc90,
        &args![this, target, second, third, fourth, value, last],
    );
    if e.call(ACTOR_IS_IN_COMBAT, &args![this]).u8() != 0 {
        e.call(0x008a_d8f0, &args![target, 0u32]);
    }
}

// Translated from 008b38d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::BuildRequest_ov2` (Xbox PDB): resolves the cell and world space
/// from `place` (the actor's own, `008d6f30` and `TESObjectREFR::GetWorldSpace`
/// `00575d70`, when 0; `place` itself when its form kind (`00401170`) is `0x39`
/// for the first or `0x41` for the second) and builds the request with
/// [`actor_build_request`].
pub fn actor_build_request_ov2(
    e: &mut Engine,
    this: Ptr<Actor>,
    target: u32,
    second: u32,
    place: u32,
    value: f32,
    last: u32,
) {
    let mut first_place = 0;
    let mut second_place = 0;
    if place == 0 {
        first_place = e.call(0x008d_6f30, &args![this]).u32();
        second_place = e.call(0x0057_5d70, &args![this]).u32();
    } else if e.call(0x0040_1170, &args![place]).u32() == 0x39 {
        first_place = place;
    } else if e.call(0x0040_1170, &args![place]).u32() == 0x41 {
        second_place = place;
    }
    actor_build_request(
        e,
        this,
        target,
        second,
        first_place,
        second_place,
        value,
        last,
    );
}

// Translated from 008b3960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsPathValid` (Xbox PDB): `ActorMover::IsPathValid` (`009de3e0`).
pub fn actor_is_path_valid(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x009d_e3e0, &args![mover]).u32()
}

// Translated from 008b3980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `009dcab0` of the `ActorMover`, its result passed through.
pub fn fn_008b3980(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x009d_cab0, &args![mover]).u32()
}

// Translated from 008b39a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `009de110` of the `ActorMover`, its result passed through.
pub fn fn_008b39a0(e: &mut Engine, this: Ptr<Actor>, first: u32, second: u32) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x009d_e110, &args![mover, first, second]).u32()
}

// Translated from 008b39d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetFailedPathDestination` (Xbox PDB):
/// `ActorMover::GetFailedPathDestination` (`009de460`).
pub fn actor_get_failed_path_destination(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x009d_e460, &args![mover]).u32()
}

// Translated from 008b39f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetMoveMode` (Xbox PDB): slot 4 of the `ActorMover`
/// (`SetMoveModePreference`).
pub fn actor_set_move_mode(e: &mut Engine, this: Ptr<Actor>, mode: u32) -> u32 {
    let mover = actor_mover(e, this);
    e.vcall(mover.addr(), 0x4, &args![mode]).u32()
}

// Translated from 008b3a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ForceMoveMode` (Xbox PDB): slot `0xC` of the `ActorMover`.
pub fn actor_force_move_mode(e: &mut Engine, this: Ptr<Actor>, mode: u32) -> u32 {
    let mover = actor_mover(e, this);
    e.vcall(mover.addr(), 0xc, &args![mode]).u32()
}

// Translated from 008b3a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ClearForcedMoveMode` (Xbox PDB): slot `0x10` of the `ActorMover`.
pub fn actor_clear_forced_move_mode(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = actor_mover(e, this);
    e.vcall(mover.addr(), 0x10, &args![]).u32()
}

// Translated from 008b3a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ClearMoveMode` (Xbox PDB): slot 8 of the `ActorMover`
/// (`ClearMoveModePreference`), one stack argument.
pub fn actor_clear_move_mode(e: &mut Engine, this: Ptr<Actor>, argument: u32) -> u32 {
    let mover = actor_mover(e, this);
    e.vcall(mover.addr(), 0x8, &args![argument]).u32()
}

// Translated from 008b3ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::StopMoving` (Xbox PDB): `ActorMover::StopMoving` (`009dd0a0`).
pub fn actor_stop_moving(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x009d_d0a0, &args![mover]).u32()
}

// Translated from 008b3ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ForceStopMoving` (Xbox PDB): `ActorMover::ForceStopMoving`
/// (`009dd0c0`).
pub fn actor_force_stop_moving(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x009d_d0c0, &args![mover]).u32()
}

// Translated from 008b3af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs [`fn_008b3b10`] on the actor's `ActorMover`.
pub fn fn_008b3af0(e: &mut Engine, this: Ptr<Actor>) {
    let mover = actor_mover(e, this);
    fn_008b3b10(e, mover);
}

// Translated from 008b3b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover` (`this`): clears `bStopMovementNextUpdate` (Xbox PDB +0x72).
pub fn fn_008b3b10(e: &mut Engine, this: Ptr) {
    e.mem
        .set_u8(this.addr() + MOVER_STOP_MOVEMENT_NEXT_UPDATE, 0);
}

// Translated from 008b3b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a 12-byte vector to `out` and returns `out`: with `value <= 0.0`
/// the 12 bytes `00436aa0` of the actor returns, otherwise the result of
/// `009dcc80(mover, out, value)`.
pub fn fn_008b3b30(e: &mut Engine, this: Ptr<Actor>, out: Ptr, value: f32) -> Ptr {
    let zero: f64 = e.global(ZERO_DOUBLE);
    if f64::from(value) <= zero {
        let source = e.call(0x0043_6aa0, &args![this]).u32();
        for offset in [0, 4, 8] {
            let word = e.mem.u32(source + offset);
            e.mem.set_u32(out.addr() + offset, word);
        }
    } else {
        let mover = actor_mover(e, this);
        e.call(0x009d_cc80, &args![mover, out, value]);
    }
    out
}

// Translated from 008b3b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `0097f2f0` of the `ActorMover`, its result passed through.
pub fn fn_008b3b90(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x0097_f2f0, &args![mover]).u32()
}

// Translated from 008b3bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsPathingComplete` (Xbox PDB): `ActorMover::IsPathingComplete`
/// (`009dcd40`).
pub fn actor_is_pathing_complete(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x009d_cd40, &args![mover]).u32()
}

// Translated from 008b3bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsPathing` (Xbox PDB): `ActorMover::IsPathing` (`009dcdb0`).
pub fn actor_is_pathing(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x009d_cdb0, &args![mover]).u32()
}

// Translated from 008b3bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsWaitingOnPath` (Xbox PDB): [`fn_008b3c10`] on the actor's
/// `ActorMover`.
pub fn actor_is_waiting_on_path(e: &mut Engine, this: Ptr<Actor>) -> u8 {
    let mover = actor_mover(e, this);
    fn_008b3c10(e, mover)
}

// Translated from 008b3c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover` (`this`): `bWaitingForPath` (Xbox PDB +0x74).
pub fn fn_008b3c10(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + MOVER_WAITING_FOR_PATH)
}

// Translated from 008b3c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsRotating` (Xbox PDB): `ActorMover::IsRotating` (`009ddb50`).
pub fn actor_is_rotating(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x009d_db50, &args![mover]).u32()
}

// Translated from 008b3c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `009dcb30` of the `ActorMover`, its result passed through.
pub fn fn_008b3c50(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x009d_cb30, &args![mover]).u32()
}

// Translated from 008b3c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_008b3c90`] on the actor's `ActorMover`.
pub fn fn_008b3c70(e: &mut Engine, this: Ptr<Actor>) -> u8 {
    let mover = actor_mover(e, this);
    fn_008b3c90(e, mover)
}

// Translated from 008b3c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover` (`this`): `bStopMovementNextUpdate` (Xbox PDB +0x72).
pub fn fn_008b3c90(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + MOVER_STOP_MOVEMENT_NEXT_UPDATE)
}

// Translated from 008b3cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `009de3a0` of the `ActorMover`, its result passed through.
pub fn fn_008b3cb0(e: &mut Engine, this: Ptr<Actor>, argument: u32) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x009d_e3a0, &args![mover, argument]).u32()
}

// Translated from 008b3cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetLookAtTarget` (Xbox PDB): `ActorMover::SetLookAtTarget`
/// (`009de160`) with the point's three floats.
pub fn actor_set_look_at_target(e: &mut Engine, this: Ptr<Actor>, x: f32, y: f32, z: f32) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x009d_e160, &args![mover, x, y, z]).u32()
}

// Translated from 008b3d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ActorMover` (`this`): `bFaceTargetPoint` (Xbox PDB +0x73).
pub fn fn_008b3d10(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + MOVER_FACE_TARGET_POINT)
}

// Translated from 008b3d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ClearLookAtTarget` (Xbox PDB): `ActorMover::ClearLookAtTarget`
/// (`009de230`).
pub fn actor_clear_look_at_target(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x009d_e230, &args![mover]).u32()
}

// Translated from 008b3d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00460140` of the `ActorMover`, its result passed through.
pub fn fn_008b3d50(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let mover = actor_mover(e, this);
    e.call(0x0046_0140, &args![mover]).u32()
}

/// `Actor::Dismember` (Xbox PDB): `(this, hit, actor value, body part, first extra,
/// second extra, chance, limit, flag)`.
const ACTOR_DISMEMBER: u32 = 0x008b_4d10;
/// The `ModelLoader` instance (`ModelLoader::LoadFile`'s `this`).
const MODEL_LOADER: u32 = 0x011c_3b3c;

// Translated from 008b3d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor's `0043fcd0` handle is used for a ray test along the direction
/// from the first point to the second (both passed by value, three floats
/// each): the segment is `512.0` long (`010231a0`), starts half of it (times
/// `01016248`) before the second point and goes along the normalised
/// direction (`00439ef0` subtracts, `00525340` normalises, `0045bb20`
/// scales); `008cfbc0` runs it. When that finds nothing a second test
/// (`00c806b0`, with a `006240d0` object configured by `008a50f0` and
/// `008a50d0(0102226c)`) is made and its hit is turned into an object
/// through `004ae750`, `004b5a20` and `0044ddc0`. Returns the result of
/// slot `0xC` of that object (0 without one). The compiler's stack
/// protector is not modelled.
#[allow(clippy::too_many_arguments)]
pub fn fn_008b3d70(
    e: &mut Engine,
    this: Ptr<Actor>,
    first_x: f32,
    first_y: f32,
    first_z: f32,
    second_x: f32,
    second_y: f32,
    second_z: f32,
) -> u32 {
    let world = e.call(0x0043_fcd0, &args![this]).u32();
    e.with_stack(0x80, |e, block| {
        let first = block.addr();
        let second = first + 0x0c;
        let direction = first + 0x18;
        let scaled = first + 0x24;
        let halved = first + 0x30;
        let start = first + 0x3c;
        let length = first + 0x48;
        let query = first + 0x50;
        let point_copy = first + 0x70;
        e.mem.set_f32(first, first_x);
        e.mem.set_f32(first + 4, first_y);
        e.mem.set_f32(first + 8, first_z);
        e.mem.set_f32(second, second_x);
        e.mem.set_f32(second + 4, second_y);
        e.mem.set_f32(second + 8, second_z);
        e.call(0x0043_9ef0, &args![second, direction, first]);
        e.call(0x0052_5340, &args![direction]);
        let segment: f32 = e.global(0x0102_31a0);
        e.mem.set_f32(length, segment);
        e.call(0x0045_bb20, &args![direction, scaled, segment]);
        let half: f32 = e.global(0x0101_6248);
        e.call(0x0045_bb20, &args![scaled, halved, half]);
        e.call(0x0043_9ef0, &args![second, start, halved]);
        let mut ray = vec![world];
        ray.extend((0..3).map(|word| e.mem.u32(start + 4 * word)));
        ray.extend((0..3).map(|word| e.mem.u32(direction + 4 * word)));
        ray.extend([length, 0]);
        let mut found = e.call(0x008c_fbc0, &ray).u32();
        if found == 0 {
            e.call(0x0062_40d0, &args![query]);
            e.call(0x0068_15c0, &args![point_copy]);
            e.call(0x004a_3e00, &args![point_copy, second]);
            e.call(0x008a_50f0, &args![query, point_copy]);
            let radius: f32 = e.global(0x0102_226c);
            e.call(0x008a_50d0, &args![query, radius]);
            let hit = e.call(0x00c8_06b0, &args![world, query, 0u32]).u32();
            if hit != 0 {
                let proxy = e.call(0x004a_e750, &args![hit]).u32();
                let owner = e.call(0x004b_5a20, &args![proxy]).u32();
                if owner != 0 {
                    found = e.call(0x0044_ddc0, &args![owner]).u32();
                }
            }
        }
        if found != 0 {
            e.vcall(found, 0xc, &args![]).u32()
        } else {
            0
        }
    })
}

// Translated from 008b3ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds which of the process's 15 slots (virtual slot `0x6D0` of the
/// process, index 0 to 14) holds `node` or one of its parents (`009611e0`),
/// walking up until the actor's own slot `0x1D0` object; `0xE` when a parent
/// equals the process's slot `0x190` answer for the actor's slot `0x1E8`;
/// -1 when nothing matches or the actor has no process.
pub fn fn_008b3ef0(e: &mut Engine, this: Ptr<Actor>, node: u32) -> i32 {
    let process = e.get(this, Actor::pCurrentProcess).addr();
    if node == 0 || process == 0 {
        return -1;
    }
    let stop = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    let mut current = node;
    let key = e.vcall(this.addr(), 0x1e8, &args![]).u32();
    let target = e.vcall(process, 0x190, &args![key]).u32();
    while current != 0 && current != stop {
        if e.call(0x0096_11e0, &args![current]).u32() == target {
            return 0xe;
        }
        for index in 0..0xf_u32 {
            let process = e.get(this, Actor::pCurrentProcess).addr();
            if e.vcall(process, 0x6d0, &args![index]).u32() == current {
                return index as i32;
            }
        }
        current = e.call(0x0096_11e0, &args![current]).u32();
    }
    -1
}

// Translated from 008b3fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Spawns the model of a destroyed body part (a dismemberment piece).
/// The body part `part_index` of the actor's body part data (`004181e0` then
/// slot `0x180` of the result; `BGSBodyPartData::GetBodyPart` `005e50f0`) is
/// used when the actor has a parent cell (`008d6f30`) and the part passes
/// [`fn_008b4360`].
///
/// With `spawn_effects` set: a part that has `005e3fa0` triggers `009ac9c0`
/// (an explosion; arguments: that value, 0, 0, the actor's `008d6f30`, the
/// three floats at `impact + 0x24`, the nine words at `reference + 0x68`);
/// and a part with an object at `+0x68` (`008d8520`) and a non-zero
/// [`fn_008b4380`] calls `004fa6f0` on that object with the actor's scale
/// (`TESObjectREFR::GetScale`) times `009b0bd0` of the part.
///
/// When the part has a model path (`005d8a70`/`0048cee0`) and `reference` is
/// not 0, the model is loaded (`ModelLoader::LoadFile`, `00447080`), cloned
/// (`NiObject::Clone`, `00a5d2c0`) and placed: its local transform is
/// `inverse(actor reference transform, 008b42e0) * (reference + 0x68)`
/// (`0062c250`), set through `00440460`, `0043fa80` and the scale
/// `00440490(LowProcess::GetTrackedDamage)`. The model is released
/// afterwards (`0045a5e0`). Returns the clone, or 0. The compiler's
/// exception frame is not modelled.
pub fn fn_008b3fe0(
    e: &mut Engine,
    this: Ptr<Actor>,
    part_index: u32,
    reference: u32,
    impact: u32,
    spawn_effects: bool,
) -> u32 {
    let mut clone = 0;
    let base_form = e.call(0x0041_81e0, &args![this]).u32();
    let body_parts = e.vcall(base_form, 0x180, &args![]).u32();
    if body_parts == 0 || e.call(0x008d_6f30, &args![this]).u32() == 0 {
        return clone;
    }
    let part = e.call(0x005e_50f0, &args![body_parts, part_index]).u32();
    if part == 0 || !fn_008b4360(e, Ptr::new(part)) {
        return clone;
    }
    if spawn_effects {
        if e.call(0x005e_3fa0, &args![part]).u32() != 0 {
            let source = e.call(0x0046_1130, &args![reference]).u32();
            let matrix: Vec<u32> = (0..9).map(|word| e.mem.u32(source + 4 * word)).collect();
            let impact_position: Vec<u32> = (0..3)
                .map(|word| e.mem.u32(impact + 0x24 + 4 * word))
                .collect();
            let place = e.call(0x008d_6f30, &args![this]).u32();
            let value = e.call(0x005e_3fa0, &args![part]).u32();
            let mut words = vec![value, 0, 0, place];
            words.extend(impact_position);
            words.extend(matrix);
            e.call(0x009a_c9c0, &words);
        }
        let object = e.call(ACTOR_GET_PROCESS, &args![part]).u32();
        if object != 0 && fn_008b4380(e, Ptr::new(part)) != 0 {
            let scale = e.call(0x0056_7400, &args![this]).f64();
            let factor = e.call(0x009b_0bd0, &args![part]).f64();
            let value = (factor * scale) as f32;
            let count = fn_008b4380(e, Ptr::new(part)) as u32;
            let place = e.call(0x008d_6f30, &args![this]).u32();
            let object = e.call(ACTOR_GET_PROCESS, &args![part]).u32();
            e.call(
                0x004f_a6f0,
                &args![object, place, impact + 0x24, count, value],
            );
        }
    }
    let path_owner = e.call(0x005d_8a70, &args![part]).u32();
    if e.call(0x0048_cee0, &args![path_owner]).u32() == 0 || reference == 0 {
        return clone;
    }
    let path_owner = e.call(0x005d_8a70, &args![part]).u32();
    // 008b41b6: the model's +0x14 (`00408da0`, plain RET) takes no stack word; the five
    // words pushed before it are the last five of `00447080` (RET 0x18).
    let path = e.vcall(path_owner, 0x14, &args![]).u32();
    let loader = e.global::<u32>(MODEL_LOADER);
    let model = e
        .call(
            0x0044_7080,
            &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
        )
        .u32();
    if model != 0 {
        clone = e.with_stack(0x100, |e, block| {
            let first_transform = block.addr();
            let second_transform = first_transform + 0x40;
            let scratch = first_transform + 0x80;
            let cloning = first_transform + 0xc0;
            e.call(0x004a_d050, &args![cloning, 1.0f32]);
            let clone = e.call(0x00a5_d2c0, &args![model, cloning]).u32();
            e.call(0x0047_6a80, &args![first_transform]);
            e.call(0x0047_6a80, &args![second_transform]);
            let owner = e.vcall(this.addr(), 0x1d0, &args![]).u32();
            let owner_transform = e.call(0x0046_1130, &args![owner]).u32();
            fn_008b42e0(e, Ptr::new(owner_transform), Ptr::new(first_transform));
            let reference_transform = e.call(0x0046_1130, &args![reference]).u32();
            let composed = e
                .call(
                    0x0062_c250,
                    &args![first_transform, scratch, reference_transform],
                )
                .u32();
            for word in 0..13 {
                let value = e.mem.u32(composed + 4 * word);
                e.mem.set_u32(second_transform + 4 * word, value);
            }
            e.call(0x0044_0460, &args![clone, second_transform + 0x24]);
            e.call(0x0043_fa80, &args![clone, second_transform]);
            let tracked = e.call(0x009b_88c0, &args![part]).f64() as f32;
            e.call(0x0044_0490, &args![clone, tracked]);
            e.call(0x004a_d270, &args![cloning]);
            clone
        });
    }
    let path_owner = e.call(0x005d_8a70, &args![part]).u32();
    let path = e.vcall(path_owner, 0x14, &args![]).u32();
    let loader = e.global::<u32>(MODEL_LOADER);
    e.call(0x0045_a5e0, &args![loader, path]);
    clone
}

// Translated from 008b42e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTransform` inverse: `dest` becomes the inverse of `this`. The rotation
/// is `NiMatrix3::Inverse_ov2` (`004b46a0`), the scale (`+0x30`) is
/// `1.0 / this.scale`, and the translation (`+0x24`) is `004a3760` of the
/// new scale and `dest.rotation` (`004b4500`) applied to `004a0bd0` of
/// `this`'s translation.
pub fn fn_008b42e0(e: &mut Engine, this: Ptr, dest: Ptr) {
    e.call(0x004b_46a0, &args![this, dest]);
    let scale = e.mem.f32(this.addr() + 0x30);
    let inverse_scale = (1.0 / f64::from(scale)) as f32;
    e.mem.set_f32(dest.addr() + 0x30, inverse_scale);
    e.with_stack(0x30, |e, scratch| {
        let first = scratch.addr();
        let negated = e.call(0x004a_0bd0, &args![this.addr() + 0x24, first]).u32();
        let rotated = e
            .call(0x004b_4500, &args![dest, first + 0x0c, negated])
            .u32();
        let scaled = e
            .call(0x004a_3760, &args![first + 0x18, inverse_scale, rotated])
            .u32();
        for offset in [0, 4, 8] {
            let word = e.mem.u32(scaled + offset);
            e.mem.set_u32(dest.addr() + 0x24 + offset, word);
        }
    });
}

// Translated from 008b4360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Body part (`BGSBodyPartData` entry, `this`): flag `0x08` of the byte at `+0x60`.
pub fn fn_008b4360(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x60) & 0x08 != 0
}

// Translated from 008b4380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Body part (`this`): the byte at `+0x66`.
pub fn fn_008b4380(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x66)
}

// Translated from 008b4cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Body part (`this`): flag `0x40` of the byte at `+0x60`.
pub fn fn_008b4cd0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x60) & 0x40 != 0
}

/// Number of dismemberment calls left this frame: the counter at
/// `011e077c` is bumped on every use of [`fn_008b43a0`] and compared with the
/// setting at `011e0a10`.
const DISMEMBER_COUNTER: u32 = 0x011e_077c;
const DISMEMBER_LIMIT_SETTING: u32 = 0x011e_0a10;
/// Setting whose byte, when non-zero, stops humanoid creatures from being dismembered.
const DISMEMBER_BLOCK_SETTING: u32 = 0x011d_f7f8;
const DISMEMBER_SETTING_LIMB_CHANCE: u32 = 0x011c_f18c;
const DISMEMBER_SETTING_FIRST_PART: u32 = 0x011c_e188;
const DISMEMBER_SETTING_OTHER_PART: u32 = 0x011c_fe90;
const DISMEMBER_SETTING_ROLL: u32 = 0x011c_e2b4;
const DISMEMBER_SETTING_ROLLED_PART: u32 = 0x011c_ee3c;

/// The value behind a setting object (`0043d4d0` returns its address).
fn setting_value(e: &mut Engine, setting: u32) -> u32 {
    let address = e.call(0x0043_d4d0, &args![setting]).u32();
    e.mem.u32(address)
}

/// Frame offsets (from the frame pointer) of the four 7-entry arrays of
/// [`fn_008b43a0`]. The original keeps them on the stack side by side and
/// can write one entry past the end; the frame is kept as one block so the
/// overlap is the same.
const DISMEMBER_PARTS: i32 = -0x20;
const DISMEMBER_VALUES: i32 = -0x7c;
const DISMEMBER_CHANCE: i32 = -0x5c;
const DISMEMBER_LIMIT: i32 = -0x40;

fn frame_index(base: i32, entry: u32) -> usize {
    ((0xdc + base + 4 * entry as i32) / 4) as usize
}

/// One list entry of [`fn_008b43a0`]: body part, actor value, two numbers.
fn dismember_push(frame: &mut [u32; 0x37], count: u32, entry: [u32; 4]) {
    frame[frame_index(DISMEMBER_PARTS, count)] = entry[0];
    frame[frame_index(DISMEMBER_VALUES, count)] = entry[1];
    frame[frame_index(DISMEMBER_CHANCE, count)] = entry[2];
    frame[frame_index(DISMEMBER_LIMIT, count)] = entry[3];
}

/// `BGSBodyPartData` entry test of `008b43a0`: the entry has flag `0x08`,
/// flag `0x40` and a byte `008b4cf0` of at least 100.
fn body_part_is_severable(e: &mut Engine, part: u32) -> bool {
    fn_008b4360(e, Ptr::new(part))
        && fn_008b4cd0(e, Ptr::new(part))
        && e.call(0x008b_4cf0, &args![part]).u8() >= 100
}

// Translated from 008b43a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks the body parts to dismember for a hit and calls `Actor::Dismember`
/// (`008b4d10`) for each; `hit` is the hit record (its fields at 0, 8 and
/// `0x10` are the attacker, a part list owner and a body part index).
///
/// Nothing happens for a humanoid creature or an actor whose slot `0x218`
/// answers true while the setting at `011df7f8` is non-zero, without body
/// part data (`004181e0` slot `0x180`), without a hit, or when the per-frame
/// counter `011e077c` has reached the setting at `011e0a10`; otherwise the
/// counter is bumped and up to 7 entries (part, actor value `ToActorValue(4,
/// i)`, and two numbers) are collected: from the hit's body part and the
/// actor values (slot 8 of the `MagicTarget` at `+0xA4`) when the attacker's
/// value `0x37` is positive, from the list owner's list when its slot `0x220`
/// answers true, else from the hit's body part alone. With no entry
/// `Dismember` is called once with -1 for part and value; entries with value
/// `0x1a` are dismembered with flag `0x40` of the hit temporarily set
/// (`00407e00`) when the roll flag was set and the flag was not already.
pub fn fn_008b43a0(
    e: &mut Engine,
    this: Ptr<Actor>,
    hit: u32,
    first_extra: u32,
    second_extra: u32,
) {
    let form = e.call(0x0041_81e0, &args![this]).u32();
    let cast = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, 0x0118_46e8u32, 0x0118_3a00u32, 0u32],
        )
        .u32();
    let humanoid = cast != 0 && e.call(0x005f_bf20, &args![cast]).bool();
    if humanoid || e.vcall(this.addr(), 0x218, &args![]).bool() {
        let setting = e.call(0x0040_8d60, &args![DISMEMBER_BLOCK_SETTING]).u32();
        if e.mem.u8(setting) != 0 {
            return;
        }
    }
    let form = e.call(0x0041_81e0, &args![this]).u32();
    let body_parts = e.vcall(form, 0x180, &args![]).u32();
    if body_parts == 0 || hit == 0 {
        return;
    }
    let limit = setting_value(e, DISMEMBER_LIMIT_SETTING) as i32;
    let counter = e.global::<u32>(DISMEMBER_COUNTER) as i32;
    if counter >= limit {
        return;
    }
    let counter = e.global::<u32>(DISMEMBER_COUNTER);
    e.set_global::<u32>(DISMEMBER_COUNTER, counter.wrapping_add(1));

    let magic_target = this.addr() + 0xa4;
    let mut frame = [0u32; 0x37];
    for entry in 0..7 {
        frame[frame_index(DISMEMBER_PARTS, entry)] = u32::MAX;
        frame[frame_index(DISMEMBER_VALUES, entry)] = u32::MAX;
        frame[frame_index(DISMEMBER_CHANCE, entry)] = 0;
        frame[frame_index(DISMEMBER_LIMIT, entry)] = 0;
    }
    let mut count: u32 = 0;
    let mut rolled = false;
    let hit_attacker = e.mem.u32(hit);
    let hit_list_owner = e.mem.u32(hit + 8);
    let hit_part = e.mem.u32(hit + 0x10);

    if hit_attacker != 0 && e.vcall(hit_attacker + 0xa4, 8, &args![0x37u32]).i32() > 0 {
        let mut chosen_value: i32 = -1;
        let part = e.call(0x005e_50f0, &args![body_parts, hit_part]).u32();
        e.vcall(magic_target, 8, &args![0x1au32]);
        let mut forced_part: u32 = u32::MAX;
        let mut severable: bool;
        if part != 0 {
            let flag_set = e.call(0x0058_cba0, &args![hit, 0x40u32]).bool();
            severable = if flag_set && fn_008b4360(e, Ptr::new(part)) {
                true
            } else {
                body_part_is_severable(e, part)
            };
            let value = e.call(0x005e_5190, &args![part]).u8() as i8 as i32;
            if value == 0x1a {
                let take = if severable {
                    true
                } else if fn_008b4360(e, Ptr::new(part)) {
                    let roll = e.call(0x0048_7f50, &args![]).u32() % 100;
                    roll < setting_value(e, DISMEMBER_SETTING_LIMB_CHANCE)
                } else {
                    false
                };
                if take {
                    forced_part = e.call(0x005e_5300, &args![part]).u32();
                    rolled = true;
                }
            } else if count == 0 {
                chosen_value = e.call(0x005e_5190, &args![part]).u8() as i8 as i32;
                let limit = if severable {
                    100
                } else {
                    setting_value(e, DISMEMBER_SETTING_FIRST_PART)
                };
                dismember_push(
                    &mut frame,
                    count,
                    [hit_part, chosen_value as u32, 100, limit],
                );
                count += 1;
            }
        }
        let mut index: u32 = 0;
        while index < 7 && count < 7 {
            let value = e.call(0x0066_ec10, &args![4u32, index & 0xff]).u32() as i32;
            if value != chosen_value && value != -1 && value != 0x1a {
                let part = e.call(0x005e_5130, &args![body_parts, value as u32]).u32();
                if part != 0 {
                    severable = e.call(0x0058_cba0, &args![hit, 0x40u32]).bool()
                        || body_part_is_severable(e, part);
                    let current = e.vcall(magic_target, 8, &args![value as u32]).i32();
                    if current <= 0 {
                        let chosen = e.call(0x005e_5300, &args![part]).u32();
                        let limit = if severable {
                            100
                        } else {
                            setting_value(e, DISMEMBER_SETTING_OTHER_PART)
                        };
                        dismember_push(&mut frame, count, [chosen, value as u32, 100, limit]);
                        count += 1;
                    } else if severable
                        || rolled
                        || e.call(0x0048_7f50, &args![]).u32() % 100
                            <= setting_value(e, DISMEMBER_SETTING_ROLL)
                    {
                        let chosen = e.call(0x005e_5300, &args![part]).u32();
                        let limit = if severable {
                            100
                        } else {
                            setting_value(e, DISMEMBER_SETTING_ROLLED_PART)
                        };
                        dismember_push(&mut frame, count, [chosen, value as u32, 100, limit]);
                        count += 1;
                    }
                }
            }
            index += 1;
        }
        if rolled && forced_part != u32::MAX {
            dismember_push(&mut frame, count, [forced_part, 0x1a, 100, 100]);
            count += 1;
        }
    } else if hit_list_owner != 0 && e.vcall(hit_list_owner, 0x220, &args![]).bool() {
        let mut node = e.call(0x009b_1720, &args![hit_list_owner, this]).u32();
        while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() && count < 7 {
            let slot = e.call(0x0068_15c0, &args![node]).u32();
            let entry = e.mem.u32(slot);
            node = e.call(0x0072_6070, &args![node]).u32();
            let part_key = e.mem.u32(entry);
            let part = e.call(0x005e_50f0, &args![body_parts, part_key]).u32();
            if part != 0 && e.mem.u32(entry) != 0 {
                let chosen = e.mem.u32(entry);
                let value = e.call(0x005e_5190, &args![part]).u8() as i8 as i32;
                frame[frame_index(DISMEMBER_PARTS, count)] = chosen;
                frame[frame_index(DISMEMBER_VALUES, count)] = value as u32;
                count += 1;
            }
        }
    } else {
        let part = e.call(0x005e_50f0, &args![body_parts, hit_part]).u32();
        let severable = part != 0 && body_part_is_severable(e, part);
        if part != 0 && (severable || hit_part != 0) {
            let value = e.call(0x005e_5190, &args![part]).u8() as i8 as i32;
            if value != -1 && e.vcall(magic_target, 8, &args![value as u32]).i32() <= 0 {
                frame[frame_index(DISMEMBER_PARTS, count)] = hit_part;
                frame[frame_index(DISMEMBER_VALUES, count)] = value as u32;
                count += 1;
            }
        }
    }

    if count == 0 {
        e.call(
            ACTOR_DISMEMBER,
            &args![
                this,
                hit,
                u32::MAX,
                u32::MAX,
                first_extra,
                second_extra,
                0u32,
                0u32,
                0u32
            ],
        );
    }
    for entry in 0..count {
        let part = frame[frame_index(DISMEMBER_PARTS, entry)];
        let value = frame[frame_index(DISMEMBER_VALUES, entry)];
        if part == u32::MAX || value == u32::MAX {
            continue;
        }
        let call_words = args![
            this,
            hit,
            value,
            part,
            first_extra,
            second_extra,
            frame[frame_index(DISMEMBER_CHANCE, entry)],
            frame[frame_index(DISMEMBER_LIMIT, entry)],
            rolled as u32
        ];
        if rolled && value == 0x1a && !e.call(0x0058_cba0, &args![hit, 0x40u32]).bool() {
            e.call(0x0040_7e00, &args![hit, 0x40u32, 1u32]);
            e.call(ACTOR_DISMEMBER, &call_words);
            e.call(0x0040_7e00, &args![hit, 0x40u32, 0u32]);
        } else {
            e.call(ACTOR_DISMEMBER, &call_words);
        }
    }
}
/// The default location (three floats) written when an actor has no usable position.
const DEFAULT_LOCATION: u32 = 0x011f_426c;
/// `"AI: Maximum number of factions %i reached in Actor::IntegrateFactionLists for actor '%s' (%08X)"`.
const INTEGRATE_OVERFLOW_MESSAGE: u32 = 0x0108_4f40;

// ---- 008b4cf0 .. 008b6df0: dismemberment ----------------------------------

/// Setting whose byte, when non-zero, makes the loop of [`actor_dismember`]
/// look at the part it was asked for only.
const DISMEMBER_SINGLE_PART_SETTING: u32 = 0x011d_f858;
/// Table of the names `"DismemberedLimb<n>_..."` of the limb nodes, indexed by
/// body part index (4 bytes each, pointers to the strings).
const LIMB_NAME_TABLE: u32 = 0x0119_6d6c;
/// `double` constant `pi / 180`.
const DEGREES_TO_RADIANS: u32 = 0x0102_3128;
/// The limb bone table filled by [`actor_fill_bone_array`]: up to 0x100
/// node pointers, and the number of entries before it.
const LIMB_BONES: u32 = 0x011d_f270;
const LIMB_BONE_COUNT: u32 = 0x011d_f268;
const LIMB_BONE_CAPACITY: u32 = 0x100;
/// The object whose `+4` pointer is the speed factor of a thrown limb
/// (`00403e20` returns the pointer).
const LIMB_VELOCITY_OWNER: u32 = 0x011c_ef14;
/// `"MODELS: Increase the MAX_LIMB_BONES (%s, %d, %d) define in Actor.cpp ..."`.
const LIMB_BONES_MESSAGE: u32 = 0x0108_4e78;
/// `"DismemberedLimb"` and its length, compared against node names.
const DISMEMBERED_LIMB_PREFIX: u32 = 0x0108_4e64;
/// The object (VATS instance) that `44ddc0`, `9c71c0` and `44edb0` are called on.
const VATS_INSTANCE: u32 = 0x011f_2250;
/// `ProcessLists` instance, receiver of `00974290`.
const PROCESS_LISTS_INSTANCE: u32 = 0x011e_0e80;
/// A `NiNode` is allocated with this many bytes (`00aa13e0`) and built with `00a5ecb0`.
const NODE_SIZE: u32 = 0xac;

// Translated from 008b4cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Body part (`this`): the byte at `+0x65`, which [`actor_dismember`] compares
/// against 100.
pub fn fn_008b4cf0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x65)
}

// Translated from 008b4d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::Dismember` (Xbox PDB): dismembers the body part `part_index` of an
/// actor hit for the actor value `actor_value` (the hit's kind, 0x19 to 0x1f
/// for a limb or head).
///
/// Nothing happens for a humanoid creature, or an actor whose slot `0x218`
/// answers true, while the setting at `011df7f8` is non-zero. An
/// `actor_value` outside 0x19..=0x1f only calls `TESObjectREFR::SetDismembered`
/// (`00572fc0`) with part -1, for the hit's part (or `part_index` without a
/// hit). Otherwise it stops when the part is already dismembered
/// (`00573090`), the actor is essential (`0087f3d0`) or slot `0x1a0` says so.
/// The body part data gives the part's flags (`008b4cd0`, `008b4360`) and
/// chance byte (`008b4cf0`); with the hit's flag `0x40` or `0x20`
/// (`0058cba0`) and `00646e50` the two outcome bytes are decided, and when
/// either is set the parts of the body (at most 15) that have the actor value
/// are tried: the first one with a limb node (process slot `0x6d0`) that
/// `00456610` does not reject and that is either listed (`008b5190`) or
/// severable (`008b4360`) is dismembered: `SetDismembered`, then
/// [`fn_008b52a0`] or, when `008c7aa0` says so, the task queue
/// (`004537b0`, `0087b440`).
#[allow(clippy::too_many_arguments)]
pub fn actor_dismember(
    e: &mut Engine,
    this: Ptr<Actor>,
    hit: u32,
    actor_value: i32,
    part_index: u32,
    first_extra: u32,
    second_extra: u32,
    chance: u32,
    limit: u32,
    rolled: u8,
) {
    let form = e.call(0x0041_81e0, &args![this]).u32();
    let cast = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, 0x0118_46e8u32, 0x0118_3a00u32, 0u32],
        )
        .u32();
    let humanoid = cast != 0 && e.call(0x005f_bf20, &args![cast]).bool();
    if humanoid || e.vcall(this.addr(), 0x218, &args![]).bool() {
        let setting = e.call(0x0040_8d60, &args![DISMEMBER_BLOCK_SETTING]).u32();
        if e.mem.u8(setting) != 0 {
            return;
        }
    }
    if !(0x19..=0x1f).contains(&actor_value) {
        let part = if hit != 0 {
            e.mem.u32(hit + 0x10)
        } else {
            part_index
        };
        e.call(
            0x0057_2fc0,
            &args![this, u32::MAX, first_extra, second_extra, part, 0u32],
        );
        return;
    }
    if e.call(0x0057_3090, &args![this, part_index]).bool() {
        return;
    }
    if e.call(0x0087_f3d0, &args![this]).bool() {
        return;
    }
    if e.vcall(this.addr(), 0x1a0, &args![0u32]).bool() {
        return;
    }
    let mut part_chance: u32 = 0;
    let mut part_flag = false;
    let mut severable = false;
    let form = e.call(0x0041_81e0, &args![this]).u32();
    let body_parts = e.vcall(form, 0x180, &args![]).u32();
    if body_parts != 0 {
        let part = e.call(0x005e_50f0, &args![body_parts, part_index]).u32();
        if part != 0 {
            part_flag = fn_008b4cd0(e, Ptr::new(part));
            if fn_008b4360(e, Ptr::new(part)) {
                part_chance = u32::from(fn_008b4cf0(e, Ptr::new(part)));
            }
            if part_flag && part_chance as i32 >= 100 {
                severable = true;
            }
        }
    }
    let other_part = hit != 0 && e.mem.u32(hit + 0x10) != part_index;
    let mut outcome_20: u8 = 0;
    let mut outcome_40: u8 = severable as u8;
    if hit != 0 {
        if e.call(0x0058_cba0, &args![hit, 0x40u32]).bool() {
            outcome_40 = 1;
        } else if e.call(0x0058_cba0, &args![hit, 0x20u32]).bool() {
            outcome_20 = 1;
        }
    }
    if outcome_20 == 0 && outcome_40 == 0 {
        let source = if other_part {
            0
        } else if first_extra != 0 && e.call(0x0040_1170, &args![first_extra]).u32() == 0x28 {
            first_extra
        } else if hit != 0 && e.mem.u32(hit + 0x30) != 0 {
            e.mem.u32(hit + 0x30)
        } else if hit != 0 && e.mem.u32(hit + 0xc) == 0x2d {
            e.global::<u32>(0x011c_a278)
        } else {
            0
        };
        let magic_target = if this.addr() == 0 {
            0
        } else {
            this.addr() + 0xa4
        };
        (outcome_20, outcome_40) = e.with_stack(8, |e, bytes| {
            let (first, second) = (bytes.addr(), bytes.addr() + 1);
            e.mem.set_u8(first, outcome_20);
            e.mem.set_u8(second, outcome_40);
            e.call(
                0x0064_6e50,
                &args![
                    magic_target,
                    source,
                    part_chance,
                    chance,
                    limit,
                    part_flag as u32,
                    first,
                    second
                ],
            );
            (e.mem.u8(first), e.mem.u8(second))
        });
    }
    if outcome_20 == 0 && outcome_40 == 0 {
        return;
    }
    let mut index: i32 = 0;
    while index < 0xf {
        let setting = e
            .call(0x0040_8d60, &args![DISMEMBER_SINGLE_PART_SETTING])
            .u32();
        if e.mem.u8(setting) != 0 && index != -1 {
            index = part_index as i32;
        }
        'part: {
            let form = e.call(0x0041_81e0, &args![this]).u32();
            let body_parts = e.vcall(form, 0x180, &args![]).u32();
            let part = if body_parts != 0 {
                e.call(0x005e_50f0, &args![body_parts, index as u32]).u32()
            } else {
                0
            };
            if part == 0 || i32::from(e.call(0x005e_5190, &args![part]).u8() as i8) != actor_value {
                break 'part;
            }
            let process = e.get(this, Actor::pCurrentProcess).addr();
            let limb = e.vcall(process, 0x6d0, &args![index as u32]).u32();
            if limb == 0 || e.call(0x0045_6610, &args![limb]).bool() {
                break 'part;
            }
            if outcome_40 != 0 && !fn_008b4360(e, Ptr::new(part)) {
                outcome_40 = 0;
            }
            if !fn_008b5190(e, Ptr::new(part)) && !fn_008b4360(e, Ptr::new(part)) {
                break 'part;
            }
            e.call(
                0x0057_2fc0,
                &args![
                    this,
                    index as u32,
                    first_extra,
                    second_extra,
                    index as u32,
                    outcome_40 as u32
                ],
            );
            if !e.call(0x008c_7aa0, &args![]).bool() {
                fn_008b52a0(
                    e,
                    this,
                    actor_value,
                    index as u32,
                    outcome_40,
                    (rolled == 0) as u8,
                    1,
                    0,
                );
            } else {
                let queue = e.call(0x0045_37b0, &args![]).u32();
                e.call(
                    0x0087_b440,
                    &args![
                        queue,
                        this,
                        actor_value,
                        index as u32,
                        outcome_40 as u32,
                        (rolled == 0) as u32
                    ],
                );
            }
            let setting = e
                .call(0x0040_8d60, &args![DISMEMBER_SINGLE_PART_SETTING])
                .u32();
            if e.mem.u8(setting) != 0 && index != -1 {
                return;
            }
        }
        index += 1;
    }
}

// Translated from 008b5190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Body part (`this`): flag `0x01` of the byte at `+0x60`.
pub fn fn_008b5190(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x60) & 1 != 0
}

// Translated from 008b51b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ScriptDismember` (Xbox PDB): dismembers the body part
/// `part_index` for a script. Needs the actor's 3D (slot `0x1d0`). The part
/// comes from the body part data (`005e50f0`); its actor value is `005e5190`,
/// and when the part exists, slot `0x3ac` is called with that value and
/// twice the `MagicTarget` slot-`0xc` answer for `(value, third)`.
/// `Actor::Dismember` is then called with no hit, the value (-1 without a
/// part), `part_index`, `second_extra`, chance 0 and limit 1000.
pub fn actor_script_dismember(
    e: &mut Engine,
    this: Ptr<Actor>,
    second_extra: u32,
    part_index: u32,
    third: u32,
) {
    if e.vcall(this.addr(), 0x1d0, &args![]).u32() == 0 {
        return;
    }
    let form = e.call(0x0041_81e0, &args![this]).u32();
    let body_parts = e.vcall(form, 0x180, &args![]).u32();
    let mut part = 0;
    let mut value: i32 = -1;
    if body_parts != 0 {
        part = e.call(0x005e_50f0, &args![body_parts, part_index]).u32();
    }
    if part != 0 {
        value = i32::from(e.call(0x005e_5190, &args![part]).u8() as i8);
        // 008b5244: the actor-value owner's +0xc takes one word (RET 4); the third
        // argument pushed first is the last word of the actor's +0x3ac (RET 0xc).
        let current = e.vcall(this.addr() + 0xa4, 0xc, &args![value as u32]).f64();
        let doubled = (current + current) as f32;
        e.vcall(this.addr(), 0x3ac, &args![value as u32, doubled, third]);
    }
    actor_dismember(
        e,
        this,
        0,
        value,
        part_index,
        0,
        second_extra,
        0,
        1000,
        (part_index == 0) as u8,
    );
}

/// Creates the node `NiNode` the limb pieces hang from (`00aa13e0` allocates
/// [`NODE_SIZE`] bytes, `00a5ecb0` builds it), or 0 when the allocation fails.
fn new_limb_node(e: &mut Engine) -> u32 {
    let block = e.call(0x00aa_13e0, &args![NODE_SIZE]).u32();
    if block == 0 {
        return 0;
    }
    e.call(0x00a5_ecb0, &args![block, 0u32]).u32()
}

/// A limb flying off: `bound_owner`'s world bound (`NiAVObject::GetWorldBound`,
/// `0043d450`) gives a centre; the direction from the centre to `node`'s
/// position (`node + 0x8c`) is normalised and scaled by the factor read
/// through `00403e20` on [`LIMB_VELOCITY_OWNER`], and added as a velocity
/// (`TESHavokUtilities::AddVelocity`, `0062b8d0`). `frame` is 0x40 bytes of
/// scratch.
/// `locals` are the caller's locals for the bound copy, the centre, the direction and the
/// velocity.
fn add_limb_velocity(e: &mut Engine, bound_owner: u32, node: u32, locals: [u32; 4]) {
    let [bound_copy, centre, direction, velocity] = locals;
    let bound = e.call(0x0043_d450, &args![bound_owner]).u32();
    for word in 0..4 {
        let value = e.mem.u32(bound + 4 * word);
        e.mem.set_u32(bound_copy + 4 * word, value);
    }
    let centre_source = e.call(0x0068_15c0, &args![bound_copy]).u32();
    for word in 0..3 {
        let value = e.mem.u32(centre_source + 4 * word);
        e.mem.set_u32(centre + 4 * word, value);
    }
    let position = e.call(0x0045_bb80, &args![node]).u32();
    e.call(0x0043_9ef0, &args![position, direction, centre]);
    e.call(0x004a_0c10, &args![direction]);
    let factor_at = e.call(0x0040_3e20, &args![LIMB_VELOCITY_OWNER]).u32();
    let factor = e.mem.f32(factor_at);
    e.call(0x0045_bb20, &args![direction, velocity, factor]);
    e.call(0x0062_b8d0, &args![node, velocity, 0u32]);
}

// Translated from 008b52a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Turns the body part `part_index` of a dismembered actor into a loose
/// piece. Called by [`actor_dismember`] and [`actor_create_dismembered_limbs`].
///
/// Does nothing for a humanoid creature (or an actor whose slot `0x218`
/// answers true) while the setting at `011df7f8` is non-zero. With
/// `hide_only` the node of the part's limb (process slot `0x6d0`) is only
/// searched for among the actor's skeleton nodes by its name
/// (`LIMB_NAME_TABLE`) and handed to [`actor_hide_dismembered_limb`]. Otherwise a node group
/// is added under the skeleton root, the part's transform is built from the
/// body part data (`008b6260`, `009f8300`, euler angles to a matrix with
/// `00a59540`, composed by `0062c250` with the parent's), and the piece is
/// either a clone of the limb (`NiObject::Clone`, `00a5d2c0`, placed with
/// `00440460`, `0043fa80`, `00440490`) or, with `hide_only`, the model
/// [`fn_008b3fe0`] builds; without either a plain node. The limb's own
/// collision objects are removed, the piece gets collision, motion and a
/// velocity ([`add_limb_velocity`]), and the optional effects follow: the
/// explosion and debris of the body part (`flag_18`), the decals / effect
/// objects (`flag_14`, `689210` ...), the hide call ([`actor_hide_dismembered_limb`] or slot
/// `0x73c` when `flag_1c` is clear), VATS notifications and the property
/// update. The compiler's exception frame is not modelled.
#[allow(clippy::too_many_arguments)]
pub fn fn_008b52a0(
    e: &mut Engine,
    this: Ptr<Actor>,
    actor_value: i32,
    part_index: u32,
    hide_only: u8,
    flag_14: u8,
    flag_18: u8,
    flag_1c: u8,
) {
    // The exe's frame (`SUB ESP,0x30c` under the exception record, then `ESI`, `EDI` and the
    // record link).
    e.with_stack(0x324, |e, frame| {
        limb_piece(
            e,
            this,
            actor_value,
            part_index,
            hide_only,
            flag_14,
            flag_18,
            flag_1c,
            frame.addr(),
        );
    });
}

#[allow(clippy::too_many_arguments)]
fn limb_piece(
    e: &mut Engine,
    this: Ptr<Actor>,
    actor_value: i32,
    part_index: u32,
    hide_only: u8,
    flag_14: u8,
    flag_18: u8,
    flag_1c: u8,
    frame: u32,
) {
    // The locals at their offsets in the exe's 0x324-byte frame (see `fn_008b52a0`).
    let name = frame + 0x314;
    let second_name = frame + 0x144;
    let cloning = frame + 0x248;
    let transform_a = frame + 0x264;
    let transform_b = frame + 0x2c4;
    let transform_c = frame + 0x1dc;
    let transform_d = frame + 0x210;
    let composed_part = frame + 0xe0;
    let composed_limb = frame + 0xac;
    let euler = frame + 0x298;
    let angle_out = frame + 0x114;
    let zero_vector = frame + 0x2ac;
    let part_offset = frame + 0x138;
    let inverse_scratch = frame + 0x78;
    let burst = [0x1a0, 0x1b0, 0x178, 0x184].map(|offset| frame + offset);
    let out_word = frame + 0x190;
    let handle_out = frame + 0x6c;

    let this_addr = this.addr();
    let form = e.call(0x0041_81e0, &args![this]).u32();
    let cast = e
        .call(
            0x00ec_43fb,
            &args![form, 0u32, 0x0118_46e8u32, 0x0118_3a00u32, 0u32],
        )
        .u32();
    let humanoid = cast != 0 && e.call(0x005f_bf20, &args![cast]).bool();
    if humanoid || e.vcall(this_addr, 0x218, &args![]).bool() {
        let setting = e.call(0x0040_8d60, &args![DISMEMBER_BLOCK_SETTING]).u32();
        if e.mem.u8(setting) != 0 {
            return;
        }
    }
    let root = e.call(0x0043_fcd0, &args![this]).u32();
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let havok = e.call(0x0045_43c0, &args![cell]).u32();
    let process = e.get(this, Actor::pCurrentProcess).addr();
    let limb = e.vcall(process, 0x6d0, &args![part_index]).u32();
    let limb_name = e.global::<u32>(LIMB_NAME_TABLE + 4 * part_index);
    e.call(0x0043_8170, &args![name, limb_name]);
    if hide_only != 0 {
        let count = e.call(0x0043_b480, &args![root]).u32();
        for child_index in 0..count {
            let node = e.call(0x0043_b4a0, &args![root, child_index]).u32();
            if node != 0 {
                let node_name = e.call(0x0041_3f40, &args![node]).u32();
                if e.call(0x009a_3830, &args![node_name, name]).bool() {
                    actor_hide_dismembered_limb(e, this, part_index, limb, node, hide_only);
                    e.call(0x0043_81b0, &args![name]);
                    return;
                }
            }
        }
    }
    let form = e.call(0x0041_81e0, &args![this]).u32();
    let body_parts = e.vcall(form, 0x180, &args![]).u32();
    let part_data = if body_parts != 0 {
        e.call(0x005e_50f0, &args![body_parts, part_index]).u32()
    } else {
        0
    };
    actor_fill_bone_array(e, this, limb, 0);
    let group = new_limb_node(e);
    e.call(0x00a5_b950, &args![group, name]);
    e.vcall(root, 0xdc, &args![group, 1u32]);
    e.call(0x0043_d410, &args![zero_vector, 0.0f32, 0u32, 0u32]);
    e.vcall(group, 0xb8, &args![zero_vector]);
    let path = fn_008b6200(e, Ptr::new(part_data));
    e.call(0x0043_8170, &args![second_name, path]);
    let mut parent = e.vcall(root, 0x9c, &args![second_name]).u32();
    e.call(0x0043_81b0, &args![second_name]);
    if parent == 0 {
        parent = limb;
    }
    e.call(0x0047_6a80, &args![transform_b]);
    e.call(0x0047_6a80, &args![transform_a]);
    // The part's offset and rotation (degrees) from the body part data.
    let offset = fn_008b6260(e, Ptr::new(part_data), Ptr::new(part_offset));
    for word in 0..3 {
        let value = e.mem.u32(offset.addr() + 4 * word);
        e.mem.set_u32(transform_a + 0x24 + 4 * word, value);
    }
    let factor: f64 = e.global(DEGREES_TO_RADIANS);
    let mut angles = [0.0f32; 3];
    for (slot, source_word) in [(2usize, 2u32), (1, 1), (0, 0)] {
        let out = angle_out + 12 * slot as u32;
        let at = e.call(0x009f_8300, &args![part_data, out]).u32();
        let degrees = e.mem.f32(at + 4 * source_word);
        angles[slot] = (f64::from(degrees) * factor) as f32;
    }
    e.call(0x0041_6870, &args![euler, angles[0], angles[1], angles[2]]);
    let (x, y, z) = (e.mem.f32(euler), e.mem.f32(euler + 4), e.mem.f32(euler + 8));
    e.call(0x00a5_9540, &args![transform_a, x, y, z]);
    e.mem.set_f32(transform_a + 0x30, 1.0);
    let parent_transform = e.call(0x0046_1130, &args![parent]).u32();
    let composed = e
        .call(
            0x0062_c250,
            &args![parent_transform, composed_part, transform_a],
        )
        .u32();
    for word in 0..13 {
        let value = e.mem.u32(composed + 4 * word);
        e.mem.set_u32(transform_b + 4 * word, value);
    }
    let mut clone: u32;
    if hide_only == 0 {
        e.call(0x004a_d050, &args![cloning, 1.0f32]);
        e.call(0x004a_d240, &args![cloning, 1.0f32]);
        clone = e.call(0x00a5_d2c0, &args![limb, cloning]).u32();
        let reference = e.call(0x0089_1350, &args![]).u32();
        let other = e.call(0x004a_de00, &args![clone, reference]).u32();
        let mut matched: u8 = 0;
        if other != 0 && other != clone && e.call(0x0096_11e0, &args![other]).u32() != 0 {
            let owner = if e.vcall(this_addr, 0x1e8, &args![]).u32() != 0 {
                let list = e.vcall(this_addr, 0x1e8, &args![]).u32();
                e.call(0x0040_7840, &args![list]).u32()
            } else {
                0
            };
            let head = if owner != 0 {
                e.call(0x0072_6070, &args![owner + 0xe0]).u32()
            } else {
                0
            };
            let mut node = if head != 0 {
                e.call(0x0050_0940, &args![head]).u32()
            } else {
                0
            };
            while node != 0 {
                let slot = e.call(0x0068_15c0, &args![node]).u32();
                if e.mem.u32(slot) == 0 || matched != 0 {
                    break;
                }
                let slot = e.call(0x0068_15c0, &args![node]).u32();
                let item = e.mem.u32(slot);
                let table = e.vcall(this_addr, 0x1e8, &args![]).u32();
                let found = e.call(0x004a_af30, &args![table, item]).u32();
                matched = if found != 0 { e.mem.u8(found + 0xc) } else { 0 };
                node = e.call(0x0072_6070, &args![node]).u32();
            }
            if matched == 0 {
                let world = e.call(0x0096_11e0, &args![other]).u32();
                e.vcall(world, 0xe8, &args![other]);
            }
        }
        e.call(0x0047_6a80, &args![transform_c]);
        e.call(0x0047_6a80, &args![transform_d]);
        let root_transform = e.call(0x0046_1130, &args![root]).u32();
        fn_008b42e0(e, Ptr::new(root_transform), Ptr::new(transform_c));
        let limb_transform = e.call(0x0046_1130, &args![limb]).u32();
        let composed = e
            .call(
                0x0062_c250,
                &args![transform_c, composed_limb, limb_transform],
            )
            .u32();
        for word in 0..13 {
            let value = e.mem.u32(composed + 4 * word);
            e.mem.set_u32(transform_d + 4 * word, value);
        }
        e.call(0x0044_0460, &args![clone, transform_d + 0x24]);
        e.call(0x0043_fa80, &args![clone, transform_d]);
        let scale = e.mem.f32(transform_d + 0x30);
        e.call(0x0044_0490, &args![clone, scale]);
        if flag_18 != 0 {
            let effect = e.call(0x004f_b070, &args![part_data]).u32();
            if effect != 0 {
                let place = e.call(0x008d_6f30, &args![this]).u32();
                let effect = e.call(0x004f_b070, &args![part_data]).u32();
                let mut words = vec![effect, 0, 0, place];
                for word in 0..3 {
                    words.push(e.mem.u32(transform_b + 0x24 + 4 * word));
                }
                for word in 0..9 {
                    words.push(e.mem.u32(transform_b + 4 * word));
                }
                e.call(0x009a_c9c0, &words);
            }
            if e.call(0x0040_7840, &args![part_data]).u32() != 0
                && fn_008b6220(e, Ptr::new(part_data)) > 0
            {
                let actor_scale = e.call(0x0056_7400, &args![this]).f64();
                let part_scale = e.call(0x0064_47b0, &args![part_data]).f64();
                let scale = (part_scale * actor_scale) as f32;
                let count = fn_008b6220(e, Ptr::new(part_data));
                let place = e.call(0x008d_6f30, &args![this]).u32();
                let object = e.call(0x0040_7840, &args![part_data]).u32();
                e.call(
                    0x004f_a6f0,
                    &args![object, place, transform_b + 0x24, u32::from(count), scale],
                );
            }
        }
        e.call(0x004a_d270, &args![cloning]);
    } else {
        clone = fn_008b3fe0(e, this, part_index, parent, transform_b, flag_18 != 0);
    }
    if clone == 0 {
        clone = new_limb_node(e);
    }
    if hide_only == 0 {
        let handle = e.vcall(clone, 0xc, &args![]).u32();
        let process = e.get(this, Actor::pCurrentProcess).addr();
        e.vcall(process, 0x6cc, &args![part_index, handle]);
    }
    if e.call(0x0068_38b0, &args![limb]).u32() != 0 && e.call(0x0045_3470, &args![limb]).u32() != 0
    {
        let mut child_index = 0;
        while child_index < e.call(0x0043_b480, &args![limb]).u32() {
            let node = e.call(0x0043_b4a0, &args![limb, child_index]).u32();
            if node != 0 {
                e.call(0x00c6_9ee0, &args![node, 1u32, 1u32]);
                e.call(0x00c6_a200, &args![node, 1u32, 1u32, 1u32]);
            }
            child_index += 1;
        }
    }
    if clone != 0 {
        let collision = e.call(0x004b_5260, &args![clone]).u32();
        if collision != 0 {
            let body = e.call(0x006f_a820, &args![collision]).u32();
            if body != 0 {
                e.call(0x00c8_ec60, &args![body]);
            }
        }
        e.vcall(group, 0xdc, &args![clone, 1u32]);
        e.call(0x00c6_bd00, &args![clone, 1u32]);
        e.call(0x0043_d410, &args![out_word, 0.0f32, 0u32, 0u32]);
        e.call(0x00a5_9c60, &args![clone, out_word]);
        e.call(0x00c6_a350, &args![clone, 1u32, 1u32, 1u32, 1u32]);
        e.vcall(havok, 0xd0, &args![clone, 1u32, 0u32, 0u32, 1u32]);
        let actor_root = e.vcall(this_addr, 0x1d0, &args![]).u32();
        add_limb_velocity(e, actor_root, clone, burst);
    }
    if flag_18 != 0 {
        let kind = if hide_only != 0 {
            e.call(0x005e_3fc0, &args![part_data]).u32()
        } else {
            e.call(0x004f_d400, &args![part_data]).u32()
        };
        let sound = if kind != 0 {
            let form = e.call(0x0041_81e0, &args![this]).u32();
            let selector = e.vcall(form + 0x30, 0x58, &args![]).u32();
            e.call(0x0058_e9d0, &args![kind, selector]).u32()
        } else {
            0
        };
        let list = if sound != 0 { sound + 0x18 } else { 0 };
        if list != 0
            && e.call(0x0048_cee0, &args![list]).u32() != 0
            && e.mem.u32(this_addr + 0x10c) == 0
        {
            for round in 0..2 {
                let (enabled, target) = if round == 0 {
                    (flag_14 != 0, parent)
                } else {
                    (hide_only == 0, clone)
                };
                if !enabled {
                    continue;
                }
                let actor_scale = e.call(0x0056_7400, &args![this]).f64() as f32;
                let mut matrix = [0u32; 9];
                if round == 0 {
                    for (word, slot) in matrix.iter_mut().enumerate() {
                        *slot = e.mem.u32(transform_b + 4 * word as u32);
                    }
                } else {
                    let inverse = e
                        .call(0x004b_45b0, &args![transform_b, inverse_scratch])
                        .u32();
                    for (word, slot) in matrix.iter_mut().enumerate() {
                        *slot = e.mem.u32(inverse + 4 * word as u32);
                    }
                }
                let handle = e.vcall(list, 0x14, &args![]).u32();
                let place = e.call(0x008d_6f30, &args![this]).u32();
                let mut words = vec![place, 1.0f32.to_bits(), handle];
                words.extend_from_slice(&matrix);
                for word in 0..3 {
                    words.push(e.mem.u32(transform_b + 0x24 + 4 * word));
                }
                words.push(actor_scale.to_bits());
                words.push(7);
                words.push(target);
                let created = e.call(0x0068_9210, &words).u32();
                if e.call(0x0067_33e0, &args![sound]).u32() != 0 {
                    e.call(0x0068_a8d0, &args![created, sound]);
                    let block = e.call(0x0040_1000, &args![0x10u32]).u32();
                    let object = if block != 0 {
                        let extra = fn_008b6240(e, Ptr::new(part_data));
                        e.call(0x004a_2c70, &args![block, created, sound, u32::from(extra)])
                            .u32()
                    } else {
                        0
                    };
                    e.call(0x004a_1a50, &args![object]);
                }
            }
        }
    }
    e.call(0x0045_0f90, &args![limb, 1u32]);
    let process = e.get(this, Actor::pCurrentProcess).addr();
    if hide_only != 0 && process != 0 && flag_1c == 0 {
        e.vcall(
            process,
            0x73c,
            &args![part_index, limb, clone, u32::from(hide_only)],
        );
    } else {
        actor_hide_dismembered_limb(e, this, part_index, limb, clone, hide_only);
    }
    if hide_only == 0 && e.call(0x0044_ddc0, &args![VATS_INSTANCE]).u32() == 4 && clone != 0 {
        let action = e.call(0x009c_71c0, &args![VATS_INSTANCE]).u32();
        if action != 0 && e.mem.u32(action + 0x10) == actor_value as u32 {
            let target = e.call(0x0044_edb0, &args![VATS_INSTANCE]).u32();
            if target != 0 {
                if e.call(0x0063_9b40, &args![target]).u32() == 2 {
                    e.call(0x0058_cbc0, &args![target, clone]);
                }
                if e.call(0x008d_6f30, &args![target]).u32() == 2 {
                    e.call(0x0058_ccf0, &args![target, clone]);
                }
            }
        }
    }
    e.call(0x00a5_a040, &args![root]);
    let sibling = e.mem.u32(this_addr + 0xb0);
    if sibling != 0 {
        let handle = e.call(0x0093_1ed0, &args![this, handle_out]).u32();
        let value = e.call(0x004a_3a20, &args![handle]).u32();
        let actor_root = e.vcall(this_addr, 0x1d0, &args![]).u32();
        e.call(0x00ca_2ad0, &args![sibling, actor_root, value]);
    }
    e.call(0x0043_81b0, &args![name]);
}

// Translated from 008b6200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Body part data entry (`this`): `00559450` of the member at `+0x24` (a path).
pub fn fn_008b6200(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0055_9450, &args![this.addr() + 0x24]).u32()
}

// Translated from 008b6220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Body part data entry (`this`): the byte at `+0x78`.
pub fn fn_008b6220(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x78)
}

// Translated from 008b6240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Body part data entry (`this`): the byte at `+0xa8`.
pub fn fn_008b6240(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xa8)
}

// Translated from 008b6260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Body part data entry (`this`): copies the three words at `+0x88` to `out`
/// and returns `out`.
pub fn fn_008b6260(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    for word in 0..3 {
        let value = e.mem.u32(this.addr() + 0x88 + 4 * word);
        e.mem.set_u32(out.addr() + 4 * word, value);
    }
    out
}

// Translated from 008b6290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::HideDismemberedLimb` (Xbox PDB): hides the skinned nodes of the
/// actor's 3D that belong to the limb `limb` of the body part `part_index`
/// (the body part data of the actor's base form, slot `0x180`, then
/// `005e50f0`). `owner` is the object (when not 0) whose world node
/// (`009611e0`) is left alone.
///
/// For each child of the actor's root (`0043fcd0`) that `004910d0` accepts,
/// that is not the owner's node and that [`actor_is_skinned_to_limb`]
/// accepts: with `hide_only`, `005e4730` hides it (and `00450f90` /
/// `005e55e0` follow); otherwise a clone (`NiObject::Clone`, `00a5d2c0`) is
/// hidden, its children shown (`005e5750`), attached to the owner's world
/// node (slot `0xdc`) and to the skeleton (`004ade40` with `owner` slot `0xc`).
/// The compiler's exception frame is not modelled.
pub fn actor_hide_dismembered_limb(
    e: &mut Engine,
    this: Ptr<Actor>,
    part_index: u32,
    limb: u32,
    owner: u32,
    hide_only: u8,
) {
    let root = e.call(0x0043_fcd0, &args![this]).u32();
    let world = if owner != 0 {
        e.call(0x0096_11e0, &args![owner]).u32()
    } else {
        0
    };
    let form = e.call(0x0041_81e0, &args![this]).u32();
    let body_parts = e.vcall(form, 0x180, &args![]).u32();
    let part = if body_parts != 0 {
        e.call(0x005e_50f0, &args![body_parts, part_index]).u32()
    } else {
        0
    };
    if part == 0 || limb == 0 {
        return;
    }
    let mut index = 0;
    while index < e.call(0x0043_b480, &args![root]).u32() {
        let node = e.call(0x0043_b4a0, &args![root, index]).u32();
        if node != 0
            && e.call(0x0049_10d0, &args![node]).bool()
            && node != world
            && actor_is_skinned_to_limb(e, this, node, limb)
        {
            if hide_only != 0 {
                e.call(0x005e_4730, &args![part_index, node]);
                if part_index == 0 {
                    e.call(0x0045_0f90, &args![node, 1u32]);
                } else {
                    e.call(
                        0x005e_55e0,
                        &args![body_parts, limb, root, part_index, node],
                    );
                }
            } else {
                e.with_stack(0x40, |e, cloning| {
                    let cloning = cloning.addr();
                    e.call(0x004a_d050, &args![cloning, 1.0f32]);
                    let clone = e.call(0x00a5_d2c0, &args![node, cloning]).u32();
                    e.call(0x005e_4730, &args![part_index, node]);
                    e.call(
                        0x005e_55e0,
                        &args![body_parts, limb, root, part_index, node],
                    );
                    e.call(0x005e_4970, &args![part_index, clone]);
                    e.call(
                        0x005e_5750,
                        &args![body_parts, limb, root, part_index, clone],
                    );
                    e.vcall(world, 0xdc, &args![clone, 1u32]);
                    let skeleton = e.vcall(owner, 0xc, &args![]).u32();
                    e.call(0x004a_de40, &args![skeleton, clone, 0u32, 0u32]);
                    e.call(0x004a_d270, &args![cloning]);
                });
            }
        }
        index += 1;
    }
}

// Translated from 008b64e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::UnhideDismemberedLimb` (Xbox PDB): shows again the nodes of the
/// actor's 3D that are skinned to `limb` for the body part `part_index`
/// (`005e4890`, and `00450f90(node, 0)` first when `part_index` is 0, then
/// `005e5750` on the body part data).
pub fn actor_unhide_dismembered_limb(e: &mut Engine, this: Ptr<Actor>, part_index: u32, limb: u32) {
    let root = e.call(0x0043_fcd0, &args![this]).u32();
    let form = e.call(0x0041_81e0, &args![this]).u32();
    let body_parts = e.vcall(form, 0x180, &args![]).u32();
    let part = if body_parts != 0 {
        e.call(0x005e_50f0, &args![body_parts, part_index]).u32()
    } else {
        0
    };
    if part == 0 || limb == 0 {
        return;
    }
    let mut index = 0;
    while index < e.call(0x0043_b480, &args![root]).u32() {
        let node = e.call(0x0043_b4a0, &args![root, index]).u32();
        if node != 0
            && e.call(0x0049_10d0, &args![node]).bool()
            && actor_is_skinned_to_limb(e, this, node, limb)
        {
            if part_index == 0 {
                e.call(0x0045_0f90, &args![node, 0u32]);
            }
            e.call(0x005e_4890, &args![part_index, node]);
            e.call(
                0x005e_5750,
                &args![body_parts, limb, root, part_index, node],
            );
        }
        index += 1;
    }
}

// Translated from 008b65f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::UpdateDismemberedLimbVel` (Xbox PDB): gives the dismembered limbs
/// of the actor a velocity. With the dismemberment extra data (`0042e8c0`
/// of the extra data list) and the actor's 3D skeleton node list (slot
/// `0x1d0`, then slot `0xc`), the world is activated (`00c6a270`) when
/// there are entries; for each entry (`00441420`, its first byte is the body
/// part index, whose name is looked up in the limb name table) the first
/// node of the list with that name (`009a3830`) is thrown
/// ([`add_limb_velocity`]). The compiler's exception frame is not modelled.
pub fn actor_update_dismembered_limb_vel(e: &mut Engine, this: Ptr<Actor>) {
    let list = e.call(0x005d_43c0, &args![this]).u32();
    let record = e.call(0x0042_e8c0, &args![list]).u32();
    if record == 0 {
        return;
    }
    if e.vcall(this.addr(), 0x1d0, &args![]).u32() == 0 {
        return;
    }
    let root = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    let nodes = e.vcall(root, 0xc, &args![]).u32();
    let count = fn_008b6800(e, Ptr::new(record));
    if nodes == 0 {
        return;
    }
    if count != 0 {
        e.call(0x00c6_a270, &args![nodes, 1u32, 1u32, 0u32]);
    }
    // The exe's frame (`SUB ESP,0x60` under the exception record, then the record link): the
    // limb name at `EBP-0x24` (+0x4c), the bound copy at `EBP-0x4c` (+0x24), the centre at
    // `EBP-0x3c` (+0x34), the direction at `EBP-0x64` (+0xc), the velocity at `EBP-0x58` (+0x18).
    e.with_stack(0x70, |e, frame| {
        let frame = frame.addr();
        let name = frame + 0x4c;
        let burst = [0x24, 0x34, 0xc, 0x18].map(|offset| frame + offset);
        for entry_index in 0..count {
            let mut found = 0;
            let entry = e.call(0x0044_1420, &args![record, entry_index]).u32();
            let limb_name = e.global::<u32>(LIMB_NAME_TABLE + 4 * u32::from(e.mem.u8(entry)));
            e.call(0x0043_8170, &args![name, limb_name]);
            let mut node_index = 0;
            while node_index < e.call(0x0043_b480, &args![nodes]).u32() && found == 0 {
                let node = e.call(0x0043_b4a0, &args![nodes, node_index]).u32();
                if node != 0 {
                    let node_name = e.call(0x0041_3f40, &args![node]).u32();
                    if e.call(0x009a_3830, &args![node_name, name]).bool() {
                        found = node;
                    }
                }
                node_index += 1;
            }
            if found != 0 {
                add_limb_velocity(e, nodes, found, burst);
            }
            e.call(0x0043_81b0, &args![name]);
        }
    });
}

// Translated from 008b6800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dismemberment extra data (`this`): `0044ddc0` of the member at `+0x20`
/// (the number of entries).
pub fn fn_008b6800(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0044_ddc0, &args![this.addr() + 0x20]).u32()
}

// Translated from 008b6820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::ClearDismemberedLimbs` (Xbox PDB): removes the dismemberment extra
/// data (`0042e8e0`) and, when the actor has 3D, a process and body part
/// data, restores the limbs.
///
/// Every node of the 3D skeleton list (slot `0x1d0`, then slot `0xc`) whose
/// name starts with `"DismemberedLimb"` (`004564f0`) has its objects removed
/// (`00c69ee0`), is dropped from the list (slot `0xf0`) and sets bit `n` of a
/// mask, `n` being the number after the prefix (`atol`). With a non-empty
/// mask the targets are reloaded ([`actor_reload_targets`]), the process
/// slot `0x6c8` is called, and each flagged part's limb (process slot
/// `0x6d0`) gets [`actor_fill_bone_array`], `00450f90(limb, 0)`,
/// [`actor_unhide_dismembered_limb`] and its collision objects enabled again (`00c6a200`,
/// then the havok world's slot `0xd0` when there is one).
pub fn actor_clear_dismembered_limbs(e: &mut Engine, this: Ptr<Actor>) {
    let list = e.call(0x005d_43c0, &args![this]).u32();
    e.call(0x0042_e8e0, &args![list]);
    let root = e.call(0x0043_fcd0, &args![this]).u32();
    let form = e.call(0x0041_81e0, &args![this]).u32();
    let body_parts = e.vcall(form, 0x180, &args![]).u32();
    if root == 0 || e.call(ACTOR_GET_PROCESS, &args![this]).u32() == 0 || body_parts == 0 {
        return;
    }
    let nodes = e.vcall(root, 0xc, &args![]).u32();
    if nodes == 0 {
        return;
    }
    let mut mask: u16 = 0;
    let count = e.call(0x0043_b480, &args![nodes]).u32();
    for index in 0..count {
        let node = e.call(0x0043_b4a0, &args![nodes, index]).u32();
        if node == 0 {
            continue;
        }
        let node_name = e.call(0x0041_3f40, &args![node]).u32();
        let name = e.call(0x0043_b1b0, &args![node_name]).u32();
        if name == 0 {
            continue;
        }
        if e.call(0x0045_64f0, &args![name, DISMEMBERED_LIMB_PREFIX, 0xfu32])
            .u32()
            != 0
        {
            continue;
        }
        let number = e.call(0x00ec_a6d3, &args![name + 0xf]).u32();
        e.call(0x00c6_9ee0, &args![node, 1u32, 1u32]);
        e.vcall(nodes, 0xf0, &args![index]);
        mask |= 1u32.wrapping_shl(number) as u16;
    }
    if mask == 0 {
        return;
    }
    actor_reload_targets(e, this, 0);
    let form = e.call(0x0041_81e0, &args![this]).u32();
    let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
    let body_parts = e.vcall(form, 0x180, &args![]).u32();
    e.vcall(process, 0x6c8, &args![root, body_parts]);
    let cell = e.call(0x008d_6f30, &args![this]).u32();
    let havok = e.call(0x0045_43c0, &args![cell]).u32();
    for part_index in 0..0xfu32 {
        if u32::from(mask) & 1u32.wrapping_shl(part_index) == 0 {
            continue;
        }
        let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
        let limb = e.vcall(process, 0x6d0, &args![part_index]).u32();
        if limb == 0 {
            continue;
        }
        actor_fill_bone_array(e, this, limb, 0);
        e.call(0x0045_0f90, &args![limb, 0u32]);
        actor_unhide_dismembered_limb(e, this, part_index, limb);
        if e.call(0x0068_38b0, &args![limb]).u32() != 0
            && e.call(0x0045_3470, &args![limb]).u32() != 0
        {
            let mut child_index = 0;
            while child_index < e.call(0x0043_b480, &args![limb]).u32() {
                let node = e.call(0x0043_b4a0, &args![limb, child_index]).u32();
                if node != 0 {
                    e.call(0x00c6_a200, &args![node, 0u32, 1u32, 1u32]);
                    if havok != 0 {
                        e.vcall(havok, 0xd0, &args![node, 1u32, 0u32, 0u32, 1u32]);
                    }
                }
                child_index += 1;
            }
        }
    }
}

// Translated from 008b6ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::CreateDismemberedLimbs` (Xbox PDB): rebuilds the dismembered
/// limbs of an actor from its dismemberment extra data (`0042e8c0`). With
/// 3D, a process and body part data, and a non-empty record (the word at
/// `+0xc`), each entry (`00441420`: part index, a byte given to
/// [`fn_008b52a0`], a byte that skips the equipment update, a byte that asks
/// for [`fn_008b6df0`]) whose body part exists is handled unless its limb
/// node is already among the nodes named by the limb name table
/// (`009a3830`). For an actor of the creature kind (slot `0x218`) without
/// the skip byte, the equipment list of the entry (`+4`) is compared
/// (`004387b0`) with the actor's current one and, when they differ,
/// applied (`00605e70`); then the bones are filled, the limb is shown
/// ([`actor_unhide_dismembered_limb`]) and the piece is made ([`fn_008b52a0`]). When any
/// equipment was applied, the targets are reloaded and process slot
/// `0x468` is called with 1. The compiler's exception frame is not
/// modelled.
pub fn actor_create_dismembered_limbs(e: &mut Engine, this: Ptr<Actor>) {
    let list = e.call(0x005d_43c0, &args![this]).u32();
    let record = e.call(0x0042_e8c0, &args![list]).u32();
    if record == 0 {
        return;
    }
    let root = e.vcall(this.addr(), 0x1d0, &args![]).u32();
    let form = e.call(0x0041_81e0, &args![this]).u32();
    let body_parts = e.vcall(form, 0x180, &args![]).u32();
    if root == 0 || e.call(ACTOR_GET_PROCESS, &args![this]).u32() == 0 || body_parts == 0 {
        return;
    }
    let mut applied = false;
    if e.mem.u16(record + 0xc) != 0 {
        let mut creature_form = 0;
        let mut creature = 0;
        if e.vcall(this.addr(), 0x218, &args![]).bool() {
            creature_form = e.call(0x0041_81e0, &args![this]).u32();
            creature = this.addr();
        }
        let mut compared = false;
        let count = fn_008b6800(e, Ptr::new(record));
        // The exe's frame (`SUB ESP,0x68` under the exception record, then the record link):
        // the limb name at `EBP-0x5c` (+0x1c), the equipment at `EBP-0x54` (+0x24).
        e.with_stack(0x78, |e, frame| {
            let name = frame.addr() + 0x1c;
            let equipment = frame.addr() + 0x24;
            for index in 0..count {
                let entry = e.call(0x0044_1420, &args![record, index]).u32();
                let part_index = u32::from(e.mem.u8(entry));
                let part = e.call(0x005e_50f0, &args![body_parts, part_index]).u32();
                if part == 0 {
                    continue;
                }
                let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
                let limb = e.vcall(process, 0x6d0, &args![part_index]).u32();
                if limb != 0 {
                    let world = e.call(0x0096_11e0, &args![limb]).u32();
                    let limb_name = e.global::<u32>(LIMB_NAME_TABLE + 4 * part_index);
                    e.call(0x0043_8170, &args![name, limb_name]);
                    let node_name = e.call(0x0041_3f40, &args![world]).u32();
                    let present = e.call(0x009a_3830, &args![node_name, name]).bool();
                    e.call(0x0043_81b0, &args![name]);
                    if present {
                        continue;
                    }
                }
                if creature_form != 0 && e.mem.u8(entry + 2) == 0 {
                    let mut same = false;
                    if !compared {
                        e.call(0x0042_f570, &args![equipment]);
                        // 008b6cdb: slot 0x1e8 takes no stack word; the pushed
                        // equipment is the last word of `00605fc0` (RET 0xc).
                        let worn = e.vcall(creature, 0x1e8, &args![]).u32();
                        e.call(0x0060_5fc0, &args![creature_form, this, worn, equipment]);
                        same = e.call(0x0043_87b0, &args![equipment, entry + 4]).bool();
                        compared = true;
                        e.call(0x0042_ff80, &args![equipment]);
                    }
                    if !same {
                        // 008b6d2c: the pushed entry + 4 is the last word of
                        // `00605e70` (RET 0xc).
                        let worn = e.vcall(creature, 0x1e8, &args![]).u32();
                        e.call(0x0060_5e70, &args![creature_form, this, worn, entry + 4]);
                        applied = true;
                    }
                }
                actor_fill_bone_array(e, this, limb, 0);
                actor_unhide_dismembered_limb(e, this, part_index, limb);
                let value = i32::from(e.call(0x005e_5190, &args![part]).u8() as i8);
                let second = e.mem.u8(entry + 1);
                fn_008b52a0(e, this, value, part_index, second, 0, 0, 1);
                if e.mem.u8(entry + 3) != 0 {
                    fn_008b6df0(e, this, part_index);
                }
            }
        });
    }
    if applied {
        actor_reload_targets(e, this, 0);
        let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
        e.vcall(process, 0x468, &args![1u32]);
    }
}

// Translated from 008b6df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the limb `part_index` from the world: `00573050(this, part_index,
/// 1)`; with a limb node (process slot `0x6d0`) that has a world node
/// (`009611e0`), that node is hidden (`00450f90(.., 1)`), its objects
/// removed (`00c69ee0`) and the limb handed to the process lists
/// (`00974290`). Returns whether a limb was removed.
pub fn fn_008b6df0(e: &mut Engine, this: Ptr<Actor>, part_index: u32) -> bool {
    e.call(0x0057_3050, &args![this, part_index, 1u32]);
    let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
    let limb = e.vcall(process, 0x6d0, &args![part_index]).u32();
    if limb != 0 {
        let world = e.call(0x0096_11e0, &args![limb]).u32();
        if world != 0 {
            e.call(0x0045_0f90, &args![world, 1u32]);
            e.call(0x00c6_9ee0, &args![world, 1u32, 1u32]);
            e.call(0x0097_4290, &args![PROCESS_LISTS_INSTANCE, limb]);
            return true;
        }
    }
    false
}

// ---- 008b6e80 .. 008b8e20: limb bones, animation sets, factions -----------

// Translated from 008b6e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IsSkinnedToLimb` (Xbox PDB): whether `node` (or a descendant)
/// is skinned (slot `0x18` answers non-zero) to one of the bones in the limb
/// bone table ([`actor_fill_bone_array`]). The check goes through the
/// children (slot `0xc`, when slot `0x10` is 0) with the same function; a
/// skinned node's skin (`0043fad0`) gives the bone list (`005585e0`, count
/// `008041a0`, list `00825c00`) that is searched in the table. `limb` is
/// only passed on to the recursion.
pub fn actor_is_skinned_to_limb(e: &mut Engine, _this: Ptr<Actor>, node: u32, _limb: u32) -> bool {
    if node == 0 {
        return false;
    }
    let mut skinned = 0;
    if e.vcall(node, 0x18, &args![]).u32() != 0 {
        skinned = node;
    } else if e.vcall(node, 0xc, &args![]).u32() != 0 && e.vcall(node, 0x10, &args![]).u32() == 0 {
        let children = e.vcall(node, 0xc, &args![]).u32();
        let mut index = 0;
        while index < e.call(0x0043_b480, &args![children]).u32() {
            let child = e.call(0x0043_b4a0, &args![children, index]).u32();
            if actor_is_skinned_to_limb(e, _this, child, _limb) {
                return true;
            }
            index += 1;
        }
    }
    if skinned == 0 || e.vcall(skinned, 0x18, &args![]).u32() == 0 {
        return false;
    }
    let skin = e.call(0x0043_fad0, &args![skinned]).u32();
    if skin != 0 {
        let bones = e.call(0x0055_85e0, &args![skin]).u32();
        let count = e.call(0x0080_41a0, &args![bones]).u32();
        let list = e.call(0x0082_5c00, &args![skin]).u32();
        for index in 0..count {
            let bone = e.mem.u32(list + 4 * index);
            let mut slot = 0;
            while slot < e.global::<u32>(LIMB_BONE_COUNT) {
                if e.global::<u32>(LIMB_BONES + 4 * slot) == bone {
                    return true;
                }
                slot += 1;
            }
        }
    }
    false
}

// Translated from 008b6ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::FillBoneArray` (Xbox PDB): adds `node` (when it answers non-zero
/// to slot `0xc`) and, recursively, its children to the limb bone table at
/// `011df270` (0x100 entries, counted at `011df268`). Without `keep` the table
/// is cleared first (`00403d30`, as a memset). A full table logs a message
/// instead (`005b5e40` with the actor's name from `0055d520`).
pub fn actor_fill_bone_array(e: &mut Engine, this: Ptr<Actor>, node: u32, keep: u8) {
    if keep == 0 {
        e.call(0x0040_3d30, &args![LIMB_BONES, 0u32, 0x400u32]);
        e.set_global::<u32>(LIMB_BONE_COUNT, 0);
    }
    if node == 0 {
        return;
    }
    if e.vcall(node, 0xc, &args![]).u32() == 0 {
        return;
    }
    let count = e.global::<u32>(LIMB_BONE_COUNT);
    if count >= LIMB_BONE_CAPACITY {
        let name = e.call(0x0055_d520, &args![this]).u32();
        e.call(
            0x005b_5e40,
            &args![LIMB_BONES_MESSAGE, name, count, LIMB_BONE_CAPACITY],
        );
    } else {
        e.set_global::<u32>(LIMB_BONES + 4 * count, node);
        e.set_global::<u32>(LIMB_BONE_COUNT, count + 1);
    }
    let mut index = 0;
    while index < e.call(0x0043_b480, &args![node]).u32() {
        let child = e.call(0x0043_b4a0, &args![node, index]).u32();
        actor_fill_bone_array(e, this, child, 1);
        index += 1;
    }
}

// Translated from 008b70d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetAnimation` (Xbox PDB): the process's animation (process slot
/// `0x1b8`), or 0 without a process.
pub fn actor_get_animation(e: &mut Engine, this: Ptr<Actor>) -> u32 {
    let process = e.get(this, Actor::pCurrentProcess).addr();
    if process == 0 {
        return 0;
    }
    e.vcall(process, 0x1b8, &args![]).u32()
}

// Translated from 008b7100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes a position (three floats) to `out` and returns whether the actor
/// has a meaningful one. Without 3D (slot `0x1d0`), not visible
/// ([`actor_is_visible`] with 7) or while the player sleeps or rests
/// (`0094df60`) `out` is the default location at `011f426c` and the result is
/// false. Otherwise, when slot `0x230` or slot `0x22c(0)` answers true, the
/// position comes from `bhkBlendCollisionObject::SetBipTransform`
/// (`00c821b0`) turned by a rotation about the actor's height
/// (`004a0c90`, `004b3ae0`); else the default location is used for an actor
/// whose process has a state (slot `0x28c` + `0x410`) that [`fn_008b7320`]
/// accepts or without slot `0x1e4`, and `00494390` of the `0x1e4` object
/// otherwise.
pub fn fn_008b7100(e: &mut Engine, this: Ptr<Actor>, out: Ptr) -> bool {
    let this_addr = this.addr();
    let default_location = |e: &mut Engine, out: u32| {
        for word in 0..3 {
            let value = e.mem.u32(DEFAULT_LOCATION + 4 * word);
            e.mem.set_u32(out + 4 * word, value);
        }
    };
    if e.vcall(this_addr, 0x1d0, &args![]).u32() == 0
        || !actor_is_visible(e, this, 7)
        || e.call(0x0094_df60, &args![e.global::<u32>(PLAYER_CHARACTER)])
            .bool()
    {
        default_location(e, out.addr());
        return false;
    }
    if e.vcall(this_addr, 0x230, &args![]).bool() || e.vcall(this_addr, 0x22c, &args![0u32]).bool()
    {
        let flag = !e.vcall(this_addr, 0x22c, &args![0u32]).bool();
        let root = e.call(0x0043_fcd0, &args![this]).u32();
        e.call(0x00c8_21b0, &args![root, out, flag as u32]);
        let position = e.call(0x0043_0830, &args![this]).u32();
        let height = e.mem.f32(position + 8);
        e.with_stack(0x40, |e, scratch| {
            let rotation = scratch.addr();
            let result_scratch = rotation + 0x30;
            e.call(0x0068_15c0, &args![rotation]);
            e.call(0x004a_0c90, &args![rotation, height]);
            let rotated = e
                .call(0x004b_3ae0, &args![result_scratch, out, rotation])
                .u32();
            for word in 0..3 {
                let value = e.mem.u32(rotated + 4 * word);
                e.mem.set_u32(out.addr() + 4 * word, value);
            }
        });
        return true;
    }
    let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
    let controller = e.vcall(process, 0x28c, &args![]).u32();
    let state = if controller == 0 {
        0
    } else {
        controller + 0x410
    };
    if state != 0 && fn_008b7320(e, Ptr::new(state)) {
        default_location(e, out.addr());
        return true;
    }
    let object = e.vcall(this_addr, 0x1e4, &args![]).u32();
    if object == 0 {
        default_location(e, out.addr());
        return true;
    }
    if e.call(0x0050_d4a0, &args![this]).bool() {
        e.call(0x0049_4390, &args![object, out, this, 1u32, 0u32]);
    } else {
        e.call(0x0049_4390, &args![object, out, this, 0u32, 1u32]);
    }
    true
}

// Translated from 008b7320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Character controller state (`this`): the byte at `+0x69` is set and the
/// word at `+0x6c` is non-zero.
pub fn fn_008b7320(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x69) != 0 && e.mem.u32(this.addr() + 0x6c) != 0
}

// Translated from 008b7360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reloads the locomotion animation lists ([`fn_008b73f0`]) of the actor,
/// for the player once for the third-person animation and once for the
/// first-person one (`PlayerCharacter::GetAnimation(1)`, with the model
/// path `00464f30(00403df0(011cdd78))`). `replacement` is passed on.
pub fn fn_008b7360(e: &mut Engine, this: Ptr<Actor>, hurt: u8, replacement: u32) {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    if this.addr() == player {
        let animation = e.call(PLAYER_GET_ANIMATION, &args![player, 0u32]).u32();
        fn_008b73f0(e, this, hurt, animation, 0, replacement);
        let pooled = e.call(0x0040_3df0, &args![0x011c_dd78u32]).u32();
        let path = e.call(0x0046_4f30, &args![pooled]).u32();
        let animation = e.call(PLAYER_GET_ANIMATION, &args![player, 1u32]).u32();
        fn_008b73f0(e, this, hurt, animation, path, replacement);
    } else {
        let animation = actor_get_animation(e, this);
        fn_008b73f0(e, this, hurt, animation, 0, replacement);
    }
}

// Translated from 008b73f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the locomotion animation lists of the actor's model into
/// `animation` (the actor's own when 0): the model path is `model_path`
/// (`TESObjectREFR::GetModel`, `005715d0`, when 0), cut at its last `'\'`
/// and extended with `"\Locomotion\"`, `"Hurt\"` (`hurt`) or `"Toddler\"`
/// (the player when `008397d0` and `model_path` are set), `"IdleAnims"`.
/// The list (`ModelLoader::BuildKFFileList`, `00447330`, with the process
/// option flag 0xC for the player, or from slot `0x148` / `008a6970`) is
/// installed, replacing the old one (`00445430`, `004465f0`) or, with
/// `replacement`, into it (`00446500`). Then, unless `hurt`, an actor with
/// slot `0x390` gets the child / female / male list too. The compiler's
/// stack cookie is not modelled.
pub fn fn_008b73f0(
    e: &mut Engine,
    this: Ptr<Actor>,
    hurt: u8,
    animation: u32,
    model_path: u32,
    replacement: u32,
) {
    let mut animation = animation;
    if animation == 0 {
        animation = actor_get_animation(e, this);
    }
    if animation == 0 && replacement == 0 {
        return;
    }
    e.with_stack(0x104, |e, buffer| {
        locomotion_lists(
            e,
            this,
            hurt,
            animation,
            model_path,
            replacement,
            buffer.addr(),
        )
    });
}

/// Installs a built animation list: `00445430` and `004465f0` replace the
/// animation's list, `00446500` (with `0043cc60` of the replacement) fills the
/// replacement; the list is released afterwards (`004702f0`).
fn install_animation_list(
    e: &mut Engine,
    list: u32,
    animation: u32,
    replacement: u32,
    cancel: bool,
) {
    let loader = e.global::<u32>(MODEL_LOADER);
    if replacement == 0 {
        if cancel {
            e.call(0x0044_5430, &args![loader, animation]);
        }
        e.call(
            0x0044_65f0,
            &args![loader, list, animation, 5u32, 0u32, 0u32],
        );
    } else {
        let first = e.call(0x0043_cc60, &args![replacement]).u32();
        e.call(0x0044_6500, &args![loader, list, first, replacement, 0u32]);
    }
    e.call(0x0047_02f0, &args![list, 1u32]);
}

fn locomotion_lists(
    e: &mut Engine,
    this: Ptr<Actor>,
    hurt: u8,
    animation: u32,
    model_path: u32,
    replacement: u32,
    buffer: u32,
) {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    if model_path != 0 {
        e.call(0x0040_6d30, &args![buffer, 0x104u32, model_path]);
    } else {
        let model = e.call(0x0057_15d0, &args![this]).u32();
        e.call(0x0040_6d30, &args![buffer, 0x104u32, model]);
    }
    let slash = e.call(0x0040_ab30, &args![buffer, 0x5cu32]).u32();
    if slash == 0 {
        return;
    }
    let room = |slash: u32| 0x104u32.wrapping_sub(slash.wrapping_sub(buffer));
    e.call(0x0040_6d30, &args![slash, room(slash), 0x0101_7114u32]);
    if hurt != 0 {
        e.call(0x0040_6d50, &args![slash, room(slash), 0x0101_70f8u32]);
    } else if this.addr() == player && e.call(0x0083_97d0, &args![player]).bool() && model_path != 0
    {
        e.call(0x0040_6d50, &args![slash, room(slash), 0x0108_4f20u32]);
    }
    e.call(0x0040_6d50, &args![slash, room(slash), 0x0101_70ecu32]);
    let mut options: u32 = 0;
    if this.addr() == player {
        options = 0xc;
    } else if e.call(ACTOR_GET_PROCESS, &args![this]).u32() != 0 {
        let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
        let kind = e.call(0x0045_cd60, &args![process]).i32();
        if (0..=1).contains(&kind) {
            let process = e.call(ACTOR_GET_PROCESS, &args![this]).u32();
            let source = e.vcall(process, 0x148, &args![]).u32();
            if source != 0 {
                let source = e.vcall(process, 0x148, &args![]).u32();
                let handle = e.call(0x0044_ddc0, &args![source]).u32();
                let index = e.call(0x0044_6390, &args![handle]).u32();
                options = e.global::<u32>(0x0118_a838 + 4 * index);
            } else if e.call(0x008a_6970, &args![this]).bool() {
                options = 1;
            }
        }
    }
    let loader = e.global::<u32>(MODEL_LOADER);
    let list = e
        .call(0x0044_7330, &args![loader, buffer, 0u32, 1u32, options])
        .u32();
    if list == 0 {
        return;
    }
    install_animation_list(e, list, animation, replacement, true);
    if hurt != 0 || e.vcall(this.addr(), 0x390, &args![]).u32() == 0 {
        return;
    }
    let tail = if e.vcall(this.addr(), 0x1a0, &args![1u32]).bool() {
        0x0108_4f0cu32
    } else if e.call(0x0087_f4c0, &args![this]).u32() == 1 {
        0x0108_4ef8
    } else {
        0x0108_4ee4
    };
    e.call(0x0040_6d30, &args![slash, room(slash), tail]);
    e.call(0x0040_6d50, &args![slash, room(slash), 0x0101_70ecu32]);
    let list = e
        .call(0x0044_7330, &args![loader, buffer, 0u32, 0u32, 0u32])
        .u32();
    if list != 0 {
        install_animation_list(e, list, animation, replacement, false);
    }
}

// Translated from 008b78c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reloads the locomotion animations ([`fn_008b7360`] with no replacement)
/// when the kinds of animation now wanted differ from the loaded ones. It
/// reads the names of the animations' current sequences (`0049c390` on
/// `00497280` of the animation, slot `0x10` of the sequence, `0043b1b0` of
/// `00413f40`) and records whether they contain `"Hurt\"`, `"Toddler\"`,
/// `"Child\"` and `"Female\"`; these are compared with the actor's present
/// state (`008b7b70`, `008397d0` for the player, slot `0x1a0(1)`,
/// `0087f4c0`). `force` reloads regardless. Any missing animation, sequence
/// or name ends the function without a reload.
pub fn fn_008b78c0(e: &mut Engine, this: Ptr<Actor>, force: u8) {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let mut has_hurt: u8 = 0;
    let mut has_female: u8 = 0;
    let mut has_toddler: u8 = 0;
    let mut has_child: u8 = 0;
    let rounds: u32 = if this.addr() == player { 2 } else { 1 };
    for round in 0..rounds {
        let animation = if this.addr() == player {
            e.call(PLAYER_GET_ANIMATION, &args![player, (round == 1) as u32])
                .u32()
        } else {
            actor_get_animation(e, this)
        };
        if animation == 0 {
            return;
        }
        if e.call(0x0049_7280, &args![animation]).u32() == 0 {
            return;
        }
        let found = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), 0);
            let sequences = e.call(0x0049_7280, &args![animation]).u32();
            let present = e
                .call(0x0049_c390, &args![sequences, 3u32, slot.addr()])
                .bool();
            (present, e.mem.u32(slot.addr()))
        });
        if !found.0 || found.1 == 0 {
            return;
        }
        let sequence = e.vcall(found.1, 0x10, &args![u32::MAX]).u32();
        if sequence == 0 {
            return;
        }
        let name_holder = e.call(0x0041_3f40, &args![sequence]).u32();
        if e.call(0x0043_b1b0, &args![name_holder]).u32() == 0 {
            return;
        }
        let name_holder = e.call(0x0041_3f40, &args![sequence]).u32();
        let text = e.call(0x0043_b1b0, &args![name_holder]).u32();
        if e.call(0x00ec_7750, &args![text, 0x0101_70f8u32]).u32() != 0 {
            has_hurt = 1;
        }
        if e.call(0x00ec_7750, &args![text, 0x0108_4f20u32]).u32() != 0 {
            has_toddler = 1;
        }
        if e.call(0x00ec_7750, &args![text, 0x0108_4f34u32]).u32() != 0 {
            has_child = 1;
        }
        if e.call(0x00ec_7750, &args![text, 0x0108_4f2cu32]).u32() != 0 {
            has_female = 1;
        }
    }
    let hurt = fn_008b7b70(e, this) as u8;
    let mut toddler: u8 = 0;
    let female = (e.call(0x0087_f4c0, &args![this]).u32() == 1) as u8;
    if this.addr() == player {
        toddler = e.call(0x0083_97d0, &args![player]).u8();
    }
    let child = e.vcall(this.addr(), 0x1a0, &args![1u32]).u8();
    if child != 0 {
        has_female = female;
    }
    if force == 0
        && child == has_child
        && toddler == has_toddler
        && hurt == has_hurt
        && has_female == female
    {
        return;
    }
    // The original has the body of `fn_008b7360` inlined here.
    fn_008b7360(e, this, hurt, 0);
}

// Translated from 008b7b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the actor is hurt: the actor values read through the
/// `MagicTarget` at `+0xa4` (slot `0xc`) are `0x1d` or `0x1e` zero, and `0x48` zero.
pub fn fn_008b7b70(e: &mut Engine, this: Ptr<Actor>) -> bool {
    let zero: f64 = e.global(ZERO_DOUBLE);
    let target = this.addr() + 0xa4;
    let first = e.vcall(target, 0xc, &args![0x1du32]).f64();
    let mut ok = first == zero;
    if !ok {
        let second = e.vcall(target, 0xc, &args![0x1eu32]).f64();
        ok = second == zero;
    }
    if !ok {
        return false;
    }
    e.vcall(target, 0xc, &args![0x48u32]).f64() == zero
}

/// The faction list head of an actor: `0042e800`'s ancestors in the
/// originals read `004181e0(actor) + 0x30` through `005d8a70`.
fn faction_list_head(e: &mut Engine, actor: u32) -> u32 {
    let form = e.call(0x0041_81e0, &args![actor]).u32();
    e.call(0x005d_8a70, &args![form + 0x30]).u32()
}

/// The extra faction changes of an actor (`0042e800` of its extra data
/// list `005d43c0`), or 0.
fn faction_changes(e: &mut Engine, actor: u32) -> u32 {
    let list = e.call(0x005d_43c0, &args![actor]).u32();
    e.call(0x0042_e800, &args![list]).u32()
}

/// Collects the actor's factions into `buffer` (0x80 entries) with
/// [`actor_integrate_faction_lists`]; returns the count.
fn collect_factions(e: &mut Engine, actor: Ptr<Actor>, buffer: u32) -> u32 {
    collect_factions_and_changes(e, actor, buffer).0
}

/// [`collect_factions`] that also returns the extra faction changes it read.
fn collect_factions_and_changes(e: &mut Engine, actor: Ptr<Actor>, buffer: u32) -> (u32, u32) {
    let head = faction_list_head(e, actor.addr());
    let changes = faction_changes(e, actor.addr());
    let extra = if changes == 0 {
        0
    } else {
        e.mem.u32(changes + 0xc)
    };
    (
        actor_integrate_faction_lists(e, actor, buffer, 0x80, head, extra),
        changes,
    )
}

// Translated from 008b7c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::AddFactionMinorCrime` (Xbox PDB): calls `005fda00(faction, a, b)`
/// on every faction of the actor that [`fn_008b7d00`] accepts.
pub fn actor_add_faction_minor_crime(e: &mut Engine, this: Ptr<Actor>, first: u32, second: u8) {
    // The faction list buffer sits at +0xc of the exe's 0x21c-byte frame (`SUB ESP,0x21c`).
    e.with_stack(0x21c, |e, frame| {
        let buffer = frame.addr() + 0xc;
        let count = collect_factions(e, this, buffer);
        for index in 0..count {
            let faction = e.mem.u32(buffer + 4 * index);
            if faction != 0 && fn_008b7d00(e, Ptr::new(faction)) {
                e.call(0x005f_da00, &args![faction, first, u32::from(second)]);
            }
        }
    });
}

// Translated from 008b7d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Faction (`this`): flag `0x100` of the word at `+0x34`.
pub fn fn_008b7d00(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x34) & 0x100 != 0
}

// Translated from 008b7d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::AddFactionMajorCrime` (Xbox PDB): calls `005fda50(faction, a, b)`
/// on every faction of the actor that [`fn_008b7d00`] accepts.
pub fn actor_add_faction_major_crime(e: &mut Engine, this: Ptr<Actor>, first: u32, second: u8) {
    // The faction list buffer sits at +0xc of the exe's 0x21c-byte frame (`SUB ESP,0x21c`).
    e.with_stack(0x21c, |e, frame| {
        let buffer = frame.addr() + 0xc;
        let count = collect_factions(e, this, buffer);
        for index in 0..count {
            let faction = e.mem.u32(buffer + 4 * index);
            if faction != 0 && fn_008b7d00(e, Ptr::new(faction)) {
                e.call(0x005f_da50, &args![faction, first, u32::from(second)]);
            }
        }
    });
}

// Translated from 008b7e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sum of `006733e0` over the actor's factions.
pub fn fn_008b7e20(e: &mut Engine, this: Ptr<Actor>) -> i32 {
    // The faction list buffer sits at +0xc of the exe's 0x21c-byte frame (`SUB ESP,0x21c`).
    e.with_stack(0x21c, |e, frame| {
        let buffer = frame.addr() + 0xc;
        let count = collect_factions(e, this, buffer);
        let mut sum: i32 = 0;
        for index in 0..count {
            let faction = e.mem.u32(buffer + 4 * index);
            if faction != 0 {
                sum = sum.wrapping_add(e.call(0x0067_33e0, &args![faction]).i32());
            }
        }
        sum
    })
}

// Translated from 008b7f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The sum of `008041a0` over the actor's factions.
pub fn fn_008b7f00(e: &mut Engine, this: Ptr<Actor>) -> i32 {
    // The faction list buffer sits at +0xc of the exe's 0x21c-byte frame (`SUB ESP,0x21c`).
    e.with_stack(0x21c, |e, frame| {
        let buffer = frame.addr() + 0xc;
        let count = collect_factions(e, this, buffer);
        let mut sum: i32 = 0;
        for index in 0..count {
            let faction = e.mem.u32(buffer + 4 * index);
            if faction != 0 {
                sum = sum.wrapping_add(e.call(0x0080_41a0, &args![faction]).i32());
            }
        }
        sum
    })
}

// Translated from 008b7fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the faction relation between two actors. Both actors' factions are
/// collected; for each pair, equal factions give their `0048bf50` value
/// (taken when above the best so far, starting at -100000), and pairs of
/// different factions give the `0048bf50` of `(first, second)` when no equal
/// pair was found and it is below the lowest so far (starting at 100000).
/// A taken pair sets `*found` to 1 and `*first` / `*second` to the two
/// factions. Returns the value taken last (0 when none).
pub fn fn_008b7fe0(
    e: &mut Engine,
    this: Ptr<Actor>,
    other: Ptr<Actor>,
    found: Ptr,
    first: Ptr,
    second: Ptr,
) -> i32 {
    let mut best: i32 = -100_000;
    let mut lowest: i32 = 100_000;
    let mut result: i32 = 0;
    let head_a = faction_list_head(e, this.addr());
    let head_b = faction_list_head(e, other.addr());
    let changes_a = faction_changes(e, this.addr());
    let changes_b = faction_changes(e, other.addr());
    // The two faction list buffers sit at +0x1c and +0x22c (`EBP-0x428`, `EBP-0x218`) of the
    // exe's 0x444-byte frame.
    e.with_stack(0x444, |e, frame| {
        let buffer_a = frame.addr() + 0x1c;
        let buffer_b = frame.addr() + 0x22c;
        let extra_a = if changes_a == 0 {
            0
        } else {
            e.mem.u32(changes_a + 0xc)
        };
        let count_a = actor_integrate_faction_lists(e, this, buffer_a, 0x80, head_a, extra_a);
        let extra_b = if changes_b == 0 {
            0
        } else {
            e.mem.u32(changes_b + 0xc)
        };
        let count_b = actor_integrate_faction_lists(e, this, buffer_b, 0x80, head_b, extra_b);
        for i in 0..count_a {
            let faction_a = e.mem.u32(buffer_a + 4 * i);
            if faction_a == 0 {
                continue;
            }
            for j in 0..count_b {
                let faction_b = e.mem.u32(buffer_b + 4 * j);
                if faction_b == 0 {
                    continue;
                }
                if faction_a == faction_b {
                    let value = e
                        .call(0x0048_bf50, &args![faction_a + 0x24, faction_a])
                        .i32();
                    if value > best {
                        best = value;
                        e.mem.set_u32(found.addr(), 1);
                        e.mem.set_u32(first.addr(), faction_a);
                        e.mem.set_u32(second.addr(), faction_a);
                        result = value;
                    }
                } else if best == -100_000 {
                    let value = e
                        .call(0x0048_bf50, &args![faction_a + 0x24, faction_b])
                        .i32();
                    if value < lowest {
                        lowest = value;
                        e.mem.set_u32(found.addr(), 1);
                        e.mem.set_u32(first.addr(), faction_a);
                        e.mem.set_u32(second.addr(), faction_b);
                        result = value;
                    }
                }
            }
        }
    });
    result
}

// Translated from 008b8290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetFactionRank` (Xbox PDB): 1 when `faction` is one of the
/// actor's factions, else -1. The second stack word is never read.
pub fn actor_get_faction_rank(
    e: &mut Engine,
    this: Ptr<Actor>,
    faction: u32,
    _unused_2: u32,
) -> i32 {
    // The faction list buffer sits at +0xc of the exe's 0x21c-byte frame (`SUB ESP,0x21c`).
    e.with_stack(0x21c, |e, frame| {
        let buffer = frame.addr() + 0xc;
        let count = collect_factions(e, this, buffer);
        let mut rank: i32 = -1;
        for index in 0..count {
            if e.mem.u32(buffer + 4 * index) == faction {
                rank = 1;
            }
        }
        rank
    })
}

// Translated from 008b8360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::SetFactionsThatCareAboutCrime` (Xbox PDB): for each faction of the
/// actor that [`fn_008b7d00`] accepts and that `other` is in (`008b8e90`),
/// `0047eb90(faction, 1)` and `009ebae0(third, faction)` are called. Returns
/// whether any faction qualified.
pub fn actor_set_factions_that_care_about_crime(
    e: &mut Engine,
    this: Ptr<Actor>,
    other: Ptr,
    third: Ptr,
) -> u8 {
    // The faction list buffer sits at +0x14 of the exe's 0x224-byte frame (`SUB ESP,0x224`).
    e.with_stack(0x224, |e, frame| {
        let buffer = frame.addr() + 0x14;
        let count = collect_factions(e, this, buffer);
        let mut any: u8 = 0;
        for index in 0..count {
            let faction = e.mem.u32(buffer + 4 * index);
            if faction != 0
                && fn_008b7d00(e, Ptr::new(faction))
                && e.call(0x008b_8e90, &args![other, faction]).bool()
            {
                e.call(0x0047_eb90, &args![faction, 1u32]);
                e.call(0x009e_bae0, &args![third, faction]);
                any = 1;
            }
        }
        any
    })
}

// Translated from 008b8490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::HasFactionThatCaresAboutCrime` (Xbox PDB): whether any faction of
/// the actor is accepted by [`fn_008b7d00`].
pub fn actor_has_faction_that_cares_about_crime(e: &mut Engine, this: Ptr<Actor>) -> u8 {
    // The faction list buffer sits at +0x14 of the exe's 0x224-byte frame (`SUB ESP,0x224`).
    e.with_stack(0x224, |e, frame| {
        let buffer = frame.addr() + 0x14;
        let count = collect_factions(e, this, buffer);
        let mut any: u8 = 0;
        for index in 0..count {
            let faction = e.mem.u32(buffer + 4 * index);
            if faction != 0 && fn_008b7d00(e, Ptr::new(faction)) {
                any = 1;
                break;
            }
        }
        any
    })
}

// Translated from 008b8580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the rank of `faction` for the actor: slot `0x48(0x80000000)` first,
/// then the faction lists are collected. A faction already in them is
/// changed (`rank` -1 removes it: `00436e10`, else `00436f20`), a new one is
/// added (`00436f80`) in the extra faction changes (created with `0042e760`
/// when missing). Nothing happens for faction 0.
pub fn fn_008b8580(e: &mut Engine, this: Ptr<Actor>, faction: u32, rank: u8) {
    if faction == 0 {
        return;
    }
    e.vcall(this.addr(), 0x48, &args![0x8000_0000u32]);
    // The faction list buffer sits at +0x14 of the exe's 0x224-byte frame (`SUB ESP,0x224`).
    e.with_stack(0x224, |e, frame| {
        let buffer = frame.addr() + 0x14;
        let (count, mut changes) = collect_factions_and_changes(e, this, buffer);
        if changes == 0 {
            let list = e.call(0x005d_43c0, &args![this]).u32();
            e.call(0x0042_e760, &args![list]);
            changes = faction_changes(e, this.addr());
        }
        let signed_rank = rank as i8 as i32 as u32;
        let mut known = false;
        for index in 0..count {
            if e.mem.u32(buffer + 4 * index) == faction {
                if rank as i8 == -1 {
                    e.call(0x0043_6e10, &args![changes, faction]);
                } else {
                    e.call(0x0043_6f20, &args![changes, faction, signed_rank]);
                }
                known = true;
                break;
            }
        }
        if !known {
            e.call(0x0043_6f80, &args![changes, faction, signed_rank]);
        }
    });
}

// Translated from 008b86e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Changes the rank of `faction` by `delta` (a signed byte): with a faction
/// the actor has (`GetFactionRank` 1 or more, `third` is only passed on),
/// the new rank is clamped at 0 and set with [`fn_008b8580`].
pub fn fn_008b86e0(e: &mut Engine, this: Ptr<Actor>, faction: u32, delta: u8, third: u8) {
    if faction == 0 {
        return;
    }
    let rank = actor_get_faction_rank(e, this, faction, u32::from(third));
    if rank > -1 {
        let mut rank = rank + i32::from(delta as i8);
        if rank < 0 {
            rank = 0;
        }
        fn_008b8580(e, this, faction, rank as u8);
    }
}

// Translated from 008b8740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Combines a running faction reaction with another (`__cdecl`): `other`'s
/// `0048c1b0(faction + 0x24)` is `reaction_2`. Reaction 0 becomes it,
/// reaction 1 becomes it when it is not 0, reaction 3 becomes it when it is
/// 2; anything else is kept.
pub fn fn_008b8740(e: &mut Engine, reaction: i32, faction: Ptr, other: u32) -> i32 {
    let new = e
        .call(0x0048_c1b0, &args![faction.addr() + 0x24, other])
        .i32();
    match reaction {
        0 => new,
        1 if new != 0 => new,
        3 if new == 2 => new,
        _ => reaction,
    }
}

// Translated from 008b87a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::GetFactionFightReaction` (Xbox PDB): the strongest reaction of
/// `this`'s factions towards `other`'s, each pair combined by
/// [`fn_008b8740`] (2 ends the search at once). The factions come from the
/// form's list and, when an extra faction change list exists, from
/// [`actor_integrate_faction_lists`]. When `other` is the player and
/// `005a2270` accepts the faction the result is 1 (and `*player_flag` is
/// set to 1 unless both actors have factions from the extra lists).
pub fn actor_get_faction_fight_reaction(
    e: &mut Engine,
    this: Ptr<Actor>,
    other: Ptr<Actor>,
    player_flag: Ptr,
) -> i32 {
    let player = e.global::<u32>(PLAYER_CHARACTER);
    let mut result: i32 = 0;
    let form_a = e.call(0x0041_81e0, &args![this]).u32();
    let head_a = e.call(0x005d_8a70, &args![form_a + 0x30]).u32();
    let form_b = e.call(0x0041_81e0, &args![other]).u32();
    let head_b = e.call(0x005d_8a70, &args![form_b + 0x30]).u32();
    let changes_a = faction_changes(e, this.addr());
    let changes_b = faction_changes(e, other.addr());
    let flag = player_flag.addr();

    // `*(list node) = faction ptr`, `(+4) = rank`: valid when the rank is >= 0.
    fn faction_of(e: &mut Engine, node: u32) -> Option<u32> {
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 && (e.mem.u8(entry + 4) as i8) > -1 {
            Some(entry)
        } else {
            None
        }
    }

    if changes_a == 0 && changes_b == 0 {
        let mut node_a = head_a;
        while node_a != 0 {
            if let Some(entry_a) = faction_of(e, node_a) {
                let mut node_b = head_b;
                if other.addr() == player && e.call(0x005a_2270, &args![e.mem.u32(entry_a)]).bool()
                {
                    result = 1;
                    e.mem.set_u8(flag, 1);
                } else {
                    while node_b != 0 {
                        if let Some(entry_b) = faction_of(e, node_b) {
                            let fa = e.mem.u32(entry_a);
                            let fb = e.mem.u32(entry_b);
                            result = fn_008b8740(e, result, Ptr::new(fa), fb);
                            if result == 2 {
                                return 2;
                            }
                        }
                        node_b = e.call(0x0072_6070, &args![node_b]).u32();
                    }
                }
            }
            node_a = e.call(0x0072_6070, &args![node_a]).u32();
        }
    } else if changes_a != 0 && changes_b == 0 {
        // The buffer at +0x644 (`EBP-0x228`) of the exe's 0x86c-byte frame.
        let done = e.with_stack(0x86c, |e, frame| {
            let buffer = frame.addr() + 0x644;
            let extra = e.mem.u32(changes_a + 0xc);
            let count = actor_integrate_faction_lists(e, this, buffer, 0x80, head_a, extra);
            let mut done = None;
            'outer: for i in 0..count {
                let faction_a = e.mem.u32(buffer + 4 * i);
                if faction_a == 0 {
                    continue;
                }
                if other.addr() == player && e.call(0x005a_2270, &args![faction_a]).bool() {
                    result = 1;
                    e.mem.set_u8(flag, 1);
                    continue;
                }
                let mut node_b = head_b;
                while node_b != 0 {
                    if let Some(entry_b) = faction_of(e, node_b) {
                        let fb = e.mem.u32(entry_b);
                        result = fn_008b8740(e, result, Ptr::new(faction_a), fb);
                        if result == 2 {
                            done = Some(2);
                            break 'outer;
                        }
                    }
                    node_b = e.call(0x0072_6070, &args![node_b]).u32();
                }
            }
            done
        });
        if let Some(value) = done {
            return value;
        }
    } else if changes_a == 0 && changes_b != 0 {
        // The buffer at +0x42c (`EBP-0x440`) of the exe's 0x86c-byte frame.
        let done = e.with_stack(0x86c, |e, frame| {
            let buffer = frame.addr() + 0x42c;
            let extra = e.mem.u32(changes_b + 0xc);
            let count = actor_integrate_faction_lists(e, this, buffer, 0x80, head_b, extra);
            let mut done = None;
            let mut node_a = head_a;
            'outer_b: while node_a != 0 {
                if let Some(entry_a) = faction_of(e, node_a) {
                    let faction_a = e.mem.u32(entry_a);
                    if other.addr() == player && e.call(0x005a_2270, &args![faction_a]).bool() {
                        result = 1;
                        e.mem.set_u8(flag, 1);
                    } else {
                        for j in 0..count {
                            let faction_b = e.mem.u32(buffer + 4 * j);
                            if faction_b != 0 {
                                result = fn_008b8740(e, result, Ptr::new(faction_a), faction_b);
                                if result == 2 {
                                    done = Some(2);
                                    break 'outer_b;
                                }
                            }
                        }
                    }
                }
                node_a = e.call(0x0072_6070, &args![node_a]).u32();
            }
            done
        });
        if let Some(value) = done {
            return value;
        }
    } else {
        // The buffers at +0x14 and +0x21c (`EBP-0x858`, `EBP-0x650`) of the exe's 0x86c-byte
        // frame.
        let done = e.with_stack(0x86c, |e, frame| {
            let buffer_a = frame.addr() + 0x14;
            let buffer_b = frame.addr() + 0x21c;
            let extra_a = e.mem.u32(changes_a + 0xc);
            let count_a = actor_integrate_faction_lists(e, this, buffer_a, 0x80, head_a, extra_a);
            let extra_b = e.mem.u32(changes_b + 0xc);
            let count_b = actor_integrate_faction_lists(e, this, buffer_b, 0x80, head_b, extra_b);
            let mut done = None;
            'outer_c: for i in 0..count_a {
                let faction_a = e.mem.u32(buffer_a + 4 * i);
                if faction_a == 0 {
                    continue;
                }
                if other.addr() == player && e.call(0x005a_2270, &args![faction_a]).bool() {
                    result = 1;
                    continue;
                }
                for j in 0..count_b {
                    let faction_b = e.mem.u32(buffer_b + 4 * j);
                    if faction_b != 0 {
                        result = fn_008b8740(e, result, Ptr::new(faction_a), faction_b);
                        if result == 2 {
                            done = Some(2);
                            break 'outer_c;
                        }
                    }
                }
            }
            done
        });
        if let Some(value) = done {
            return value;
        }
    }
    result
}

// Translated from 008b8ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Actor::IntegrateFactionLists` (Xbox PDB): fills `buffer` (room for
/// `capacity` entries) with the factions of the list `first` (node chain; an
/// entry counts when its rank byte is not negative), then merges the list
/// `second`: a faction already in the buffer is zeroed when the second
/// list's rank is -1, a new one is appended when its rank is not negative.
/// Reaching `capacity` logs a message (`005b5e40` with the actor's name and
/// form id) and returns `capacity`. Returns the number of entries.
pub fn actor_integrate_faction_lists(
    e: &mut Engine,
    this: Ptr<Actor>,
    buffer: u32,
    capacity: u32,
    first: u32,
    second: u32,
) -> u32 {
    let mut count: u32 = 0;
    let mut node = first;
    while node != 0 {
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 && (e.mem.u8(entry + 4) as i8) > -1 {
            let faction = e.mem.u32(entry);
            e.mem.set_u32(buffer + 4 * count, faction);
            count += 1;
            if count >= capacity {
                integrate_overflow(e, this, capacity);
                return capacity;
            }
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
    let mut node = second;
    while node != 0 {
        let slot = e.call(0x0068_15c0, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if entry != 0 {
            let faction = e.mem.u32(entry);
            let mut found = u32::MAX;
            for index in 0..count {
                if e.mem.u32(buffer + 4 * index) == faction {
                    found = index;
                }
            }
            if found == u32::MAX {
                if (e.mem.u8(entry + 4) as i8) > -1 {
                    e.mem.set_u32(buffer + 4 * count, faction);
                    count += 1;
                    if count >= capacity {
                        integrate_overflow(e, this, capacity);
                        return capacity;
                    }
                }
            } else if e.mem.u8(entry + 4) as i8 == -1 {
                e.mem.set_u32(buffer + 4 * found, 0);
            }
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
    count
}

/// The overflow message of [`actor_integrate_faction_lists`].
fn integrate_overflow(e: &mut Engine, this: Ptr<Actor>, capacity: u32) {
    let form_id = e.call(0x0084_e3a0, &args![this]).u32();
    let name = e.call(0x0055_d520, &args![this]).u32();
    e.call(
        0x005b_5e40,
        &args![3u32, INTEGRATE_OVERFLOW_MESSAGE, capacity, name, form_id],
    );
}

// Translated from 008b8e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the faction `faction` with `rank` in the actor's extra faction
/// changes (`00436f80`), after slot `0x48(0x80000000)` and creating the
/// changes (`0042e760`) when missing.
pub fn fn_008b8e20(e: &mut Engine, this: Ptr<Actor>, faction: u32, rank: u32) {
    e.vcall(this.addr(), 0x48, &args![0x8000_0000u32]);
    let mut changes = faction_changes(e, this.addr());
    if changes == 0 {
        let list = e.call(0x005d_43c0, &args![this]).u32();
        e.call(0x0042_e760, &args![list]);
        changes = faction_changes(e, this.addr());
    }
    if changes != 0 {
        e.call(0x0043_6f80, &args![changes, faction, rank]);
    }
}
/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008b00c0, fn_008b00c0(Ptr<Actor>)),
        entry!(0x008b0140, fn_008b0140(Ptr<Actor>, u8, u32)),
        entry!(0x008b0190, actor_is_visible(Ptr<Actor>, u32) -> bool),
        entry!(0x008b0520, fn_008b0520(Ptr<Actor>) -> bool),
        entry!(0x008b0670, fn_008b0670(Ptr<Actor>, Ptr<Actor>) -> bool),
        entry!(
            0x008b06d0,
            actor_get_should_attack_actor(Ptr<Actor>, Ptr<Actor>, bool, Ptr, bool) -> bool
        ),
        entry!(
            0x008b0970,
            actor_get_should_help(Ptr<Actor>, Ptr<Actor>) -> bool
        ),
        entry!(0x008b0ac0, fn_008b0ac0(Ptr<Actor>)),
        entry!(0x008b0b00, actor_reload_targets(Ptr<Actor>, u8)),
        entry!(0x008b0ba0, fn_008b0ba0(Ptr<Actor>) -> u32),
        entry!(
            0x008b0bd0,
            actor_init_lighting_property_ptr(Ptr<Actor>, Ptr)
        ),
        entry!(0x008b0d30, fn_008b0d30(Ptr, u32) -> f32),
        entry!(0x008b0d70, fn_008b0d70(Ptr, u32) -> f32),
        entry!(0x008b0db0, fn_008b0db0(Ptr, u8) -> f32),
        entry!(0x008b0dd0, fn_008b0dd0(Ptr<Actor>, i32) -> f32),
        entry!(0x008b01c0, actor_do_death_stuff(Ptr<Actor>)),
        entry!(0x008b1070, fn_008b1070(Ptr<Actor>)),
        entry!(0x008b1910, fn_008b1910(Ptr<Actor>, Ptr, Ptr) -> u32),
        entry!(0x008b19c0, fn_008b19c0(Ptr<Actor>, u32) -> bool),
        entry!(0x008b1ff0, fn_008b1ff0(Ptr) -> bool),
        entry!(0x008b2010, actor_can_use_idle(Ptr<Actor>, u32) -> bool),
        entry!(
            0x008b2170,
            fn_008b2170(
                Ptr<Actor>,
                Ptr<Actor>,
                Ptr,
                Ptr,
                bool,
                bool,
                bool,
                u32,
                bool,
                bool,
            ) -> bool
        ),
        entry!(0x008b2860, fn_008b2860(Ptr, u8)),
        entry!(0x008b2880, fn_008b2880(Ptr, u8)),
        entry!(0x008b28a0, fn_008b28a0(Ptr, u8)),
        entry!(0x008b2b60, fn_008b2b60(Ptr, u32, i32, Ptr) -> i32),
        entry!(0x008b2bf0, fn_008b2bf0(Ptr) -> bool),
        entry!(0x008b2c10, fn_008b2c10(Ptr) -> bool),
        entry!(0x008b28c0, fn_008b28c0(Ptr<Actor>, u32, Ptr)),
        entry!(0x008b2c30, fn_008b2c30(Ptr<Actor>)),
        entry!(
            0x008b2f90,
            actor_is_at_point(Ptr<Actor>, Ptr, f32, bool, bool) -> bool
        ),
        entry!(0x008b30f0, fn_008b30f0(Ptr<Actor>)),
        entry!(0x008b3180, actor_destroy_actor_mover(Ptr<Actor>)),
        entry!(0x008b3200, fn_008b3200(Ptr<Actor>, u32)),
        entry!(0x008b3230, fn_008b3230(Ptr<Actor>, f32)),
        entry!(
            0x008b3630,
            actor_set_pathfinding_goal(Ptr<Actor>, u32) -> bool
        ),
        entry!(
            0x008b3690,
            actor_set_pathfinding_goal_ov2(Ptr<Actor>, u32, u32, u32, f32, u32) -> bool
        ),
        entry!(
            0x008b36f0,
            actor_set_pathfinding_goal_ov3(Ptr<Actor>, u32, f32, u32) -> bool
        ),
        entry!(
            0x008b3750,
            fn_008b3750(Ptr<Actor>, u32, u32, u32, f32, f32, u32) -> bool
        ),
        entry!(0x008b37c0, fn_008b37c0(Ptr<Actor>, u32, f32, u32)),
        entry!(
            0x008b3820,
            actor_get_current_pathing_request(Ptr<Actor>) -> u32
        ),
        entry!(
            0x008b3840,
            actor_get_current_pathfinding_goal(Ptr<Actor>, u32) -> u32
        ),
        entry!(0x008b3860, fn_008b3860(Ptr<Actor>, u32) -> u32),
        entry!(
            0x008b3880,
            actor_build_request(Ptr<Actor>, u32, u32, u32, u32, f32, u32)
        ),
        entry!(
            0x008b38d0,
            actor_build_request_ov2(Ptr<Actor>, u32, u32, u32, f32, u32)
        ),
        entry!(0x008b3960, actor_is_path_valid(Ptr<Actor>) -> u32),
        entry!(0x008b3980, fn_008b3980(Ptr<Actor>) -> u32),
        entry!(0x008b39a0, fn_008b39a0(Ptr<Actor>, u32, u32) -> u32),
        entry!(
            0x008b39d0,
            actor_get_failed_path_destination(Ptr<Actor>) -> u32
        ),
        entry!(0x008b39f0, actor_set_move_mode(Ptr<Actor>, u32) -> u32),
        entry!(0x008b3a20, actor_force_move_mode(Ptr<Actor>, u32) -> u32),
        entry!(0x008b3a50, actor_clear_forced_move_mode(Ptr<Actor>) -> u32),
        entry!(0x008b3a80, actor_clear_move_mode(Ptr<Actor>, u32) -> u32),
        entry!(0x008b3ab0, actor_stop_moving(Ptr<Actor>) -> u32),
        entry!(0x008b3ad0, actor_force_stop_moving(Ptr<Actor>) -> u32),
        entry!(0x008b3af0, fn_008b3af0(Ptr<Actor>)),
        entry!(0x008b3b10, fn_008b3b10(Ptr)),
        entry!(0x008b3b30, fn_008b3b30(Ptr<Actor>, Ptr, f32) -> Ptr),
        entry!(0x008b3b90, fn_008b3b90(Ptr<Actor>) -> u32),
        entry!(0x008b3bb0, actor_is_pathing_complete(Ptr<Actor>) -> u32),
        entry!(0x008b3bd0, actor_is_pathing(Ptr<Actor>) -> u32),
        entry!(0x008b3bf0, actor_is_waiting_on_path(Ptr<Actor>) -> u8),
        entry!(0x008b3c10, fn_008b3c10(Ptr) -> u8),
        entry!(0x008b3c30, actor_is_rotating(Ptr<Actor>) -> u32),
        entry!(0x008b3c50, fn_008b3c50(Ptr<Actor>) -> u32),
        entry!(0x008b3c70, fn_008b3c70(Ptr<Actor>) -> u8),
        entry!(0x008b3c90, fn_008b3c90(Ptr) -> u8),
        entry!(0x008b3cb0, fn_008b3cb0(Ptr<Actor>, u32) -> u32),
        entry!(
            0x008b3cd0,
            actor_set_look_at_target(Ptr<Actor>, f32, f32, f32) -> u32
        ),
        entry!(0x008b3d10, fn_008b3d10(Ptr) -> u8),
        entry!(0x008b3d30, actor_clear_look_at_target(Ptr<Actor>) -> u32),
        entry!(0x008b3d50, fn_008b3d50(Ptr<Actor>) -> u32),
        entry!(
            0x008b3d70,
            fn_008b3d70(Ptr<Actor>, f32, f32, f32, f32, f32, f32) -> u32
        ),
        entry!(0x008b3ef0, fn_008b3ef0(Ptr<Actor>, u32) -> i32),
        entry!(
            0x008b3fe0,
            fn_008b3fe0(Ptr<Actor>, u32, u32, u32, bool) -> u32
        ),
        entry!(0x008b42e0, fn_008b42e0(Ptr, Ptr)),
        entry!(0x008b4360, fn_008b4360(Ptr) -> bool),
        entry!(0x008b4380, fn_008b4380(Ptr) -> u8),
        entry!(0x008b43a0, fn_008b43a0(Ptr<Actor>, u32, u32, u32)),
        entry!(0x008b4cd0, fn_008b4cd0(Ptr) -> bool),
        entry!(0x008b4cf0, fn_008b4cf0(Ptr) -> u8),
        entry!(
            0x008b4d10,
            actor_dismember(Ptr<Actor>, u32, i32, u32, u32, u32, u32, u32, u8)
        ),
        entry!(0x008b5190, fn_008b5190(Ptr) -> bool),
        entry!(
            0x008b51b0,
            actor_script_dismember(Ptr<Actor>, u32, u32, u32)
        ),
        entry!(
            0x008b52a0,
            fn_008b52a0(Ptr<Actor>, i32, u32, u8, u8, u8, u8)
        ),
        entry!(0x008b6200, fn_008b6200(Ptr) -> u32),
        entry!(0x008b6220, fn_008b6220(Ptr) -> u8),
        entry!(0x008b6240, fn_008b6240(Ptr) -> u8),
        entry!(0x008b6260, fn_008b6260(Ptr, Ptr) -> Ptr),
        entry!(
            0x008b6290,
            actor_hide_dismembered_limb(Ptr<Actor>, u32, u32, u32, u8)
        ),
        entry!(
            0x008b64e0,
            actor_unhide_dismembered_limb(Ptr<Actor>, u32, u32)
        ),
        entry!(0x008b65f0, actor_update_dismembered_limb_vel(Ptr<Actor>)),
        entry!(0x008b6800, fn_008b6800(Ptr) -> u32),
        entry!(0x008b6820, actor_clear_dismembered_limbs(Ptr<Actor>)),
        entry!(0x008b6ae0, actor_create_dismembered_limbs(Ptr<Actor>)),
        entry!(0x008b6df0, fn_008b6df0(Ptr<Actor>, u32) -> bool),
        entry!(
            0x008b6e80,
            actor_is_skinned_to_limb(Ptr<Actor>, u32, u32) -> bool
        ),
        entry!(0x008b6ff0, actor_fill_bone_array(Ptr<Actor>, u32, u8)),
        entry!(0x008b70d0, actor_get_animation(Ptr<Actor>) -> u32),
        entry!(0x008b7100, fn_008b7100(Ptr<Actor>, Ptr) -> bool),
        entry!(0x008b7320, fn_008b7320(Ptr) -> bool),
        entry!(0x008b7360, fn_008b7360(Ptr<Actor>, u8, u32)),
        entry!(0x008b73f0, fn_008b73f0(Ptr<Actor>, u8, u32, u32, u32)),
        entry!(0x008b78c0, fn_008b78c0(Ptr<Actor>, u8)),
        entry!(0x008b7b70, fn_008b7b70(Ptr<Actor>) -> bool),
        entry!(
            0x008b7c00,
            actor_add_faction_minor_crime(Ptr<Actor>, u32, u8)
        ),
        entry!(0x008b7d00, fn_008b7d00(Ptr) -> bool),
        entry!(
            0x008b7d20,
            actor_add_faction_major_crime(Ptr<Actor>, u32, u8)
        ),
        entry!(0x008b7e20, fn_008b7e20(Ptr<Actor>) -> i32),
        entry!(0x008b7f00, fn_008b7f00(Ptr<Actor>) -> i32),
        entry!(
            0x008b7fe0,
            fn_008b7fe0(Ptr<Actor>, Ptr<Actor>, Ptr, Ptr, Ptr) -> i32
        ),
        entry!(
            0x008b8290,
            actor_get_faction_rank(Ptr<Actor>, u32, u32) -> i32
        ),
        entry!(
            0x008b8360,
            actor_set_factions_that_care_about_crime(Ptr<Actor>, Ptr, Ptr) -> u8
        ),
        entry!(
            0x008b8490,
            actor_has_faction_that_cares_about_crime(Ptr<Actor>) -> u8
        ),
        entry!(0x008b8580, fn_008b8580(Ptr<Actor>, u32, u8)),
        entry!(0x008b86e0, fn_008b86e0(Ptr<Actor>, u32, u8, u8)),
        entry!(0x008b8740, fn_008b8740(i32, Ptr, u32) -> i32),
        entry!(
            0x008b87a0,
            actor_get_faction_fight_reaction(Ptr<Actor>, Ptr<Actor>, Ptr) -> i32
        ),
        entry!(
            0x008b8ca0,
            actor_integrate_faction_lists(Ptr<Actor>, u32, u32, u32, u32) -> u32
        ),
        entry!(0x008b8e20, fn_008b8e20(Ptr<Actor>, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    type SharedLog = Rc<std::cell::RefCell<Vec<(u32, Vec<u32>)>>>;
    type VectorLog = Rc<std::cell::RefCell<Vec<Vec<u32>>>>;
    type ConversationRun = (
        World,
        bool,
        Vec<(u32, Vec<u32>)>,
        Ptr<Actor>,
        Ptr<Actor>,
        Ptr,
        Ptr,
    );

    fn int(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// An engine with the pages the code reads constants and the player
    /// pointer from, and the player at a fresh block.
    fn engine() -> (Engine, Ptr<Actor>) {
        let mut e = Engine::new();
        for page in [
            0x0101_2000,
            0x0103_0000,
            0x0107_1000,
            0x0108_4000,
            0x011c_e000,
            0x011d_e000,
            0x011f_2000,
        ] {
            e.map(page, 0x1000);
        }
        let player = e.new_object::<Actor>();
        e.set_global(PLAYER_CHARACTER, player.addr());
        (e, player)
    }

    /// An object of `size` bytes whose vtable has the given `(offset,
    /// function address)` slots.
    fn object_with_slots(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> Ptr {
        let vtable = e.mem.alloc(0x800);
        for (offset, target) in slots {
            e.mem.set_u32(vtable + offset, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, vtable);
        Ptr::new(object)
    }

    /// A double that counts its calls and returns `value`.
    fn counting_double(e: &mut Engine, address: u32, value: u32) -> Rc<Cell<u32>> {
        let count = Rc::new(Cell::new(0));
        let seen = count.clone();
        e.register_double(address, move |_, _| {
            seen.set(seen.get() + 1);
            int(value)
        });
        count
    }

    /// An actor with the actor-level virtual slots the tests use, at fresh
    /// fake function addresses.
    fn actor_with_slots(e: &mut Engine, slots: &[(u32, u32)]) -> Ptr<Actor> {
        object_with_slots(e, 0x1b4, slots).cast()
    }

    #[test]
    fn controller_flag_follows_the_actor_value() {
        for (value, expected) in [(0.0, 1u32), (3.0, 0)] {
            let (mut e, _) = engine();
            let actor = e.new_object::<Actor>();
            let controller = e.mem.alloc(0x500);
            e.mem.set_u32(actor.addr() + 0x1b0, controller);
            e.register(0x0093_06d0, |e, a| int(e.mem.u32(a[0] + 0x1b0)));
            e.register(0x0062_1270, |_, _| int(0));
            e.register_double(0x0087_f9c0, move |_, _| float(value));
            e.register(0x0062_9670, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            e.call(0x008b_00c0, &args![actor]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                log.last().unwrap(),
                &(0x0062_9670, vec![controller + 0x410, 0x4000, expected])
            );
        }
    }

    #[test]
    fn controller_flag_is_skipped_without_controller_or_when_state_is_set() {
        let (mut e, _) = engine();
        let actor = e.new_object::<Actor>();
        e.register(0x0093_06d0, |_, _| int(0));
        e.call(0x008b_00c0, &args![actor]);
        let controller = e.mem.alloc(0x500);
        e.register_double(0x0093_06d0, move |_, _| int(controller));
        e.register(0x0062_1270, |_, _| int(1));
        // Any further call (value, flag setter) would panic as untranslated.
        e.call(0x008b_00c0, &args![actor]);
    }

    #[test]
    fn visibility_flags_are_set_cleared_and_tested() {
        let (mut e, _) = engine();
        let actor = e.new_object::<Actor>();
        e.call(0x008b_0140, &args![actor, 1u32, 0b0110u32]);
        assert_eq!(e.get(actor, Actor::iVisFlags), 0b0110);
        e.call(0x008b_0140, &args![actor, 0u32, 0b0010u32]);
        assert_eq!(e.get(actor, Actor::iVisFlags), 0b0100);
        assert!(e.call(0x008b_0190, &args![actor, 0b0100u32]).bool());
        assert!(!e.call(0x008b_0190, &args![actor, 0b0110u32]).bool());
        assert!(e.call(0x008b_0190, &args![actor, 0u32]).bool());
    }

    fn acquire_engine() -> (Engine, Ptr<Actor>, Rc<Cell<u32>>) {
        let (mut e, _) = engine();
        let released = Rc::new(Cell::new(0));
        let seen = released.clone();
        e.register_double(0x0f00_0001, move |_, a| {
            seen.set(seen.get() + 1 + a[1]);
            int(0)
        });
        e.register(0x0f00_0002, |_, _| int(0)); // actor slot 0x160: false
        e.register(0x0f00_0003, |_, _| int(0)); // process slot 0x5fc: false
        e.register(0x0f00_0004, |_, _| int(0)); // process slot 0x22c: nothing
        e.register(0x0f00_0005, |_, _| int(0)); // process slot 0x280
        e.register(0x0f00_0006, |_, _| int(0)); // actor slot 0x1d0: no target
        e.register(0x008d_8520, |e, a| int(e.mem.u32(a[0] + 0x68)));
        let process = object_with_slots(
            &mut e,
            0x600,
            &[
                (0x22c, 0x0f00_0004),
                (0x280, 0x0f00_0005),
                (0x5fc, 0x0f00_0003),
                (0x5f8, 0x0f00_0001),
            ],
        );
        let actor = actor_with_slots(&mut e, &[(0x160, 0x0f00_0002), (0x1d0, 0x0f00_0006)]);
        e.set(actor, Actor::pCurrentProcess, process);
        (e, actor, released)
    }

    #[test]
    fn acquire_check_without_object_or_target_is_false() {
        let (mut e, actor, released) = acquire_engine();
        assert!(!e.call(0x008b_0520, &args![actor]).bool());
        assert_eq!(released.get(), 0);
    }

    #[test]
    fn acquire_check_is_false_when_the_actor_slot_says_so() {
        let (mut e, actor, _) = acquire_engine();
        e.register(0x0f00_0002, |_, _| int(1));
        // Nothing else may run (the process getter would be reached first).
        e.register(0x008d_8520, |_, _| panic!("process must not be read"));
        assert!(!e.call(0x008b_0520, &args![actor]).bool());
    }

    #[test]
    fn acquire_check_releases_the_object_when_the_process_wants_it() {
        let (mut e, actor, released) = acquire_engine();
        e.register(0x0f00_0003, |_, _| int(1));
        assert!(!e.call(0x008b_0520, &args![actor]).bool());
        assert_eq!(released.get(), 1);
    }

    #[test]
    fn acquire_check_needs_the_object_kind_and_slot_0x280() {
        let (mut e, actor, _) = acquire_engine();
        e.register(0x0f00_0004, |_, _| int(0x1234)); // an object exists
        e.register_double(0x0041_ca90, |_, _| int(7));
        assert!(!e.call(0x008b_0520, &args![actor]).bool());
        // Kind 5 with slot 0x280 true proceeds; target is still missing.
        e.register_double(0x0041_ca90, |_, _| int(5));
        e.register(0x0f00_0005, |_, _| int(1));
        assert!(!e.call(0x008b_0520, &args![actor]).bool());
        // Kind 6 with slot 0x280 false stops.
        e.register_double(0x0041_ca90, |_, _| int(6));
        e.register(0x0f00_0005, |_, _| int(0));
        let before = e.mem.u32(actor.addr() + 0x68);
        assert_ne!(before, 0);
        assert!(!e.call(0x008b_0520, &args![actor]).bool());
    }

    #[test]
    fn acquire_check_with_a_target_depends_on_the_second_check() {
        for (second, expected) in [(0u32, true), (1, false)] {
            let (mut e, actor, _) = acquire_engine();
            let target = object_with_slots(&mut e, 0x20, &[(0x10, 0x0f00_0007)]);
            e.register_double(0x0f00_0006, move |_, _| int(target.addr()));
            e.register(0x0f00_0007, |_, _| int(1));
            e.register(0x0052_4c90, |_, _| int(0x55));
            e.set_global(0x0103_0ff0, 0.8f32);
            e.register_double(0x00b4_e030, move |_, a| {
                assert_eq!((a[0], a[1], a[2]), (target.addr(), 0x55, 0.8f32.to_bits()));
                int(second)
            });
            assert_eq!(e.call(0x008b_0520, &args![actor]).bool(), expected);
        }
    }

    /// Answers of the doubles that `GetShouldAttackActor` and
    /// `GetShouldHelp` ask.
    #[derive(Default)]
    struct Knobs {
        angry: Cell<bool>,
        in_combat_with: Cell<bool>,
        reaction: Cell<i32>,
        perceived_hostile: Cell<bool>,
        setting: Cell<f64>,
        verdict_seen: Cell<i32>,
    }

    /// Fields the doubles read from the actors (unused PC bytes of the test
    /// objects): `+0x18d` teammate, `+0x104` in combat, `+0x108`
    /// aggression, `+0x10c` assistance.
    fn relation_engine() -> (Engine, Ptr<Actor>, Ptr<Actor>, Ptr<Actor>, Rc<Knobs>) {
        let (mut e, player) = engine();
        let knobs = Rc::new(Knobs::default());
        e.register(ACTOR_IS_PLAYER_TEAMMATE, |e, a| {
            int(e.mem.u8(a[0] + 0x18d) as u32)
        });
        e.register(
            ACTOR_IS_IN_COMBAT,
            |e, a| int(e.mem.u8(a[0] + 0x104) as u32),
        );
        e.register(ACTOR_GET_ACTOR_AGGRESSION, |e, a| {
            int(e.mem.u32(a[0] + 0x108))
        });
        e.register(ACTOR_GET_ACTOR_ASSISTANCE, |e, a| {
            int(e.mem.u32(a[0] + 0x10c))
        });
        let k = knobs.clone();
        e.register_double(ACTOR_IS_ANGRY_WITH_PLAYER, move |_, _| {
            int(k.angry.get() as u32)
        });
        let k = knobs.clone();
        e.register_double(ACTOR_IS_IN_COMBAT_WITH_ACTOR, move |_, _| {
            int(k.in_combat_with.get() as u32)
        });
        let k = knobs.clone();
        e.register_double(ACTOR_GET_FACTION_FIGHT_REACTION, move |_, _| {
            int(k.reaction.get() as u32)
        });
        let k = knobs.clone();
        e.register_double(PLAYER_IS_TARGET_PERCEIVED_AND_HOSTILE, move |_, _| {
            int(k.perceived_hostile.get() as u32)
        });
        let k = knobs.clone();
        e.register_double(ACTOR_GET_FOLLOWER_SETTING, move |_, _| {
            float(k.setting.get())
        });
        let k = knobs.clone();
        e.register_double(HANDLE_ENTRY_POINT, move |e, a| {
            k.verdict_seen.set(e.mem.u8(a[3]) as i32);
            Ret::default()
        });
        e.register(0x0f00_0010, |_, _| int(0)); // target slot 0x1a0: false
        let this: Ptr<Actor> = actor_with_slots(&mut e, &[(0x1a0, 0x0f00_0010)]);
        let target: Ptr<Actor> = actor_with_slots(&mut e, &[(0x1a0, 0x0f00_0010)]);
        (e, player, this, target, knobs)
    }

    fn attack_ex(
        e: &mut Engine,
        this: Ptr<Actor>,
        target: Ptr<Actor>,
        engaged: bool,
        ignore: bool,
    ) -> bool {
        let out = e.mem.alloc(4);
        e.call(0x008b_06d0, &args![this, target, engaged, out, ignore])
            .bool()
    }

    fn attack(e: &mut Engine, this: Ptr<Actor>, target: Ptr<Actor>, engaged: bool) -> (bool, i32) {
        let out = e.mem.alloc(4);
        e.mem.set_i32(out, -77);
        let verdict = e
            .call(0x008b_06d0, &args![this, target, engaged, out, false])
            .bool();
        (verdict, e.mem.i32(out))
    }

    #[test]
    fn plain_actors_attack_by_aggression_and_reaction() {
        let (mut e, _, this, target, knobs) = relation_engine();
        e.mem.set_u32(this.addr() + 0x108, 1);
        knobs.reaction.set(1);
        assert_eq!(attack(&mut e, this, target, false), (true, 1));
        assert_eq!(knobs.verdict_seen.get(), 1);
        knobs.reaction.set(2);
        assert_eq!(attack(&mut e, this, target, false), (false, 2));
        e.mem.set_u32(this.addr() + 0x108, 2);
        knobs.reaction.set(1);
        assert_eq!(attack(&mut e, this, target, false), (true, 1));
        knobs.reaction.set(2);
        assert_eq!(attack(&mut e, this, target, false), (false, 2));
    }

    #[test]
    fn aggression_three_attacks_without_asking_the_reaction() {
        let (mut e, _, this, target, _) = relation_engine();
        e.mem.set_u32(this.addr() + 0x108, 3);
        e.register(ACTOR_GET_FACTION_FIGHT_REACTION, |_, _| panic!("not asked"));
        e.register(HANDLE_ENTRY_POINT, |_, _| panic!("not asked"));
        assert!(attack_ex(&mut e, this, target, false, false));
    }

    #[test]
    fn engaged_actors_attack_unless_the_reaction_is_friendly() {
        let (mut e, _, this, target, knobs) = relation_engine();
        e.mem.set_u32(this.addr() + 0x108, 1);
        e.mem.set_u32(target.addr() + 0x108, 1);
        knobs.reaction.set(3);
        assert_eq!(attack(&mut e, this, target, true), (false, 3));
        knobs.reaction.set(0);
        assert_eq!(attack(&mut e, this, target, true), (true, 0));
        // A target that never fights (aggression 3) is always attacked back.
        e.mem.set_u32(target.addr() + 0x108, 3);
        knobs.reaction.set(3);
        assert_eq!(attack(&mut e, this, target, true), (true, 3));
    }

    #[test]
    fn combat_with_the_target_engages_unless_ignored() {
        let (mut e, _, this, target, knobs) = relation_engine();
        e.mem.set_u32(this.addr() + 0x108, 1);
        e.mem.set_u32(target.addr() + 0x108, 1);
        knobs.in_combat_with.set(true);
        knobs.reaction.set(3);
        // Engaged by the combat: the friendly reaction does not attack.
        assert!(!attack_ex(&mut e, this, target, false, false));
        // Ignored: the aggression-1 rule with reaction 3 does not attack either,
        // but with reaction 1 it does.
        knobs.reaction.set(1);
        assert!(attack_ex(&mut e, this, target, false, true));
    }

    #[test]
    fn the_player_is_attacked_when_angry() {
        let (mut e, player, this, _, knobs) = relation_engine();
        e.mem.set_u32(this.addr() + 0x108, 1);
        knobs.reaction.set(3);
        knobs.angry.set(true);
        assert_eq!(
            attack(&mut e, this, Ptr::new(player.addr()), false),
            (true, 3)
        );
        knobs.angry.set(false);
        assert_eq!(
            attack(&mut e, this, Ptr::new(player.addr()), false),
            (false, 3)
        );
    }

    #[test]
    fn teammates_follow_the_player_and_the_follower_setting() {
        let (mut e, player, this, target, knobs) = relation_engine();
        e.mem.set_u8(this.addr() + 0x18d, 1);
        // Against the player: only anger matters.
        knobs.angry.set(true);
        assert!(attack_ex(&mut e, this, player, false, false));
        knobs.angry.set(false);
        assert!(!attack_ex(&mut e, this, player, false, false));
        // Setting 0: attack only what is in combat with the player or us.
        knobs.setting.set(0.0);
        assert!(!attack_ex(&mut e, this, target, false, false));
        e.mem.set_u8(target.addr() + 0x104, 1);
        assert!(!attack_ex(&mut e, this, target, false, false));
        knobs.in_combat_with.set(true);
        assert!(attack_ex(&mut e, this, target, false, false));
        // Setting set: aggression 1, hostile reaction needs a perceived target.
        knobs.setting.set(1.0);
        knobs.reaction.set(1);
        knobs.in_combat_with.set(false);
        assert!(!attack_ex(&mut e, this, target, false, false));
        knobs.perceived_hostile.set(true);
        assert!(attack_ex(&mut e, this, target, false, false));
    }

    #[test]
    fn a_target_that_never_fights_is_not_attacked_by_a_teammate() {
        let (mut e, _, this, target, _) = relation_engine();
        e.mem.set_u8(this.addr() + 0x18d, 1);
        e.register(0x0f00_0010, |_, _| int(1));
        assert!(!attack_ex(&mut e, this, target, false, false));
    }

    #[test]
    fn a_teammate_target_is_judged_as_the_player() {
        let (mut e, player, this, target, knobs) = relation_engine();
        e.mem.set_u8(target.addr() + 0x18d, 1);
        e.mem.set_u32(this.addr() + 0x108, 1);
        knobs.reaction.set(1);
        knobs.angry.set(true);
        // The recursion's target is the player: anger decides.
        assert!(attack_ex(&mut e, this, target, false, false));
        knobs.angry.set(false);
        knobs.reaction.set(3);
        assert!(!attack_ex(&mut e, this, target, false, false));
        let _ = player;
    }

    #[test]
    fn the_attack_check_of_008b0670() {
        let (mut e, player, this, target, knobs) = relation_engine();
        e.mem.set_u32(this.addr() + 0x108, 1);
        knobs.reaction.set(3);
        // Neither is the player and `this` is no teammate: asks, answer false.
        assert!(!e.call(0x008b_0670, &args![this, target]).bool());
        // `this` is a teammate: true without asking.
        e.mem.set_u8(this.addr() + 0x18d, 1);
        e.register(ACTOR_GET_FACTION_FIGHT_REACTION, |_, _| panic!("not asked"));
        assert!(e.call(0x008b_0670, &args![this, target]).bool());
        // `this` is the player: true without asking.
        assert!(e.call(0x008b_0670, &args![player, target]).bool());
    }

    #[test]
    fn help_follows_assistance_and_reaction() {
        let (mut e, _, this, other, knobs) = relation_engine();
        assert!(!e.call(0x008b_0970, &args![this, 0u32]).bool());
        assert!(e.call(0x008b_0970, &args![this, this]).bool());
        e.mem.set_u32(this.addr() + 0x10c, 1);
        knobs.reaction.set(2);
        assert!(e.call(0x008b_0970, &args![this, other]).bool());
        knobs.reaction.set(3);
        assert!(!e.call(0x008b_0970, &args![this, other]).bool());
        e.mem.set_u32(this.addr() + 0x10c, 2);
        assert!(e.call(0x008b_0970, &args![this, other]).bool());
        knobs.reaction.set(0);
        assert!(!e.call(0x008b_0970, &args![this, other]).bool());
        // Someone who never fights is not helped.
        e.mem.set_u32(other.addr() + 0x108, 3);
        knobs.reaction.set(3);
        assert!(!e.call(0x008b_0970, &args![this, other]).bool());
    }

    #[test]
    fn teammates_help_unless_the_other_is_in_combat_with_someone_relevant() {
        let (mut e, player, this, other, knobs) = relation_engine();
        e.mem.set_u8(this.addr() + 0x18d, 1);
        knobs.angry.set(true);
        assert!(!e.call(0x008b_0970, &args![this, player]).bool());
        knobs.angry.set(false);
        assert!(e.call(0x008b_0970, &args![this, player]).bool());
        // The other is in combat with the player or us: no help.
        e.mem.set_u8(other.addr() + 0x104, 1);
        knobs.in_combat_with.set(true);
        assert!(!e.call(0x008b_0970, &args![this, other]).bool());
        // Not in combat: asks as the player (the player's assistance is 1).
        e.mem.set_u8(other.addr() + 0x104, 0);
        e.mem.set_u32(player.addr() + 0x10c, 1);
        knobs.reaction.set(2);
        assert!(e.call(0x008b_0970, &args![this, other]).bool());
        // Helping a teammate means helping the player.
        e.mem.set_u8(this.addr() + 0x18d, 0);
        e.mem.set_u8(other.addr() + 0x18d, 1);
        e.mem.set_u32(this.addr() + 0x10c, 1);
        knobs.reaction.set(2);
        assert!(e.call(0x008b_0970, &args![this, other]).bool());
        knobs.reaction.set(3);
        assert!(!e.call(0x008b_0970, &args![this, other]).bool());
    }

    #[test]
    fn queued_reload_uses_the_flag_and_clears_it() {
        let (mut e, _) = engine();
        let actor = actor_with_slots(&mut e, &[(0x1e4, 0x0f00_0020)]);
        e.register(0x0f00_0020, |_, _| int(0xa11));
        e.register(ANIMATION_RELOAD_TARGETS, |_, _| Ret::default());
        e.set(actor, Actor::bReloadTargetQueued, 1);
        e.call_log = Some(vec![]);
        e.call(0x008b_0ac0, &args![actor]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.last().unwrap(),
            &(ANIMATION_RELOAD_TARGETS, vec![0xa11, 1])
        );
        assert_eq!(e.get(actor, Actor::bReloadTargetQueued), 0);
    }

    #[test]
    fn reload_targets_of_the_player_covers_both_animations() {
        let (mut e, player) = engine();
        e.register(PLAYER_GET_ANIMATION, |_, a| {
            int(if a[1] == 1 { 0xf1 } else { 0 })
        });
        let reloads = Rc::new(Cell::new(0));
        let seen = reloads.clone();
        e.register_double(ANIMATION_RELOAD_TARGETS, move |_, a| {
            assert_eq!((a[0], a[1]), (0xf1, 9));
            seen.set(seen.get() + 1);
            Ret::default()
        });
        e.call(0x008b_0b00, &args![player, 9u32]);
        assert_eq!(reloads.get(), 1);
    }

    #[test]
    fn reload_targets_of_another_actor_needs_animation_and_process() {
        let (mut e, _) = engine();
        let actor = e.new_object::<Actor>();
        e.register(ACTOR_GET_ANIMATION, |_, _| int(0xa2));
        let reloads = counting_double(&mut e, ANIMATION_RELOAD_TARGETS, 0);
        e.call(0x008b_0b00, &args![actor, 1u32]);
        assert_eq!(reloads.get(), 0);
        e.set(actor, Actor::pCurrentProcess, Ptr::new(0x1234));
        e.call(0x008b_0b00, &args![actor, 1u32]);
        assert_eq!(reloads.get(), 1);
        e.register(ACTOR_GET_ANIMATION, |_, _| int(0));
        e.call(0x008b_0b00, &args![actor, 1u32]);
        assert_eq!(reloads.get(), 1);
    }

    #[test]
    fn process_slot_0x52c_needs_a_process() {
        let (mut e, _) = engine();
        let actor = e.new_object::<Actor>();
        assert_eq!(e.call(0x008b_0ba0, &args![actor]).u32(), 0);
        e.register(0x0f00_0030, |_, _| int(0x77));
        let process = object_with_slots(&mut e, 0x600, &[(0x52c, 0x0f00_0030)]);
        e.set(actor, Actor::pCurrentProcess, process);
        assert_eq!(e.call(0x008b_0ba0, &args![actor]).u32(), 0x77);
    }

    /// Scene graph doubles: a node's children list is the node itself
    /// (`0043b480` counts, `0043b4a0` indexes) kept in test memory.
    /// A node: +4 count, +8.. children (4 bytes each), +0x30 name pointer,
    /// +0x34 has-lighting flag.
    fn node(e: &mut Engine, name: &[u8], children: &[u32], slot_c_target: u32) -> Ptr {
        let n = object_with_slots(e, 0x60, &[(0xc, slot_c_target)]);
        e.mem.set_u32(n.addr() + 4, children.len() as u32);
        for (i, c) in children.iter().enumerate() {
            e.mem.set_u32(n.addr() + 8 + 4 * i as u32, *c);
        }
        let text = e.mem.alloc(16);
        e.mem.set_cstr(text, name);
        e.mem.set_u32(n.addr() + 0x30, text);
        n
    }

    fn lighting_engine() -> (Engine, Ptr<Actor>, Rc<Cell<u32>>) {
        let (mut e, _) = engine();
        e.register(0x0043_b480, |e, a| int(e.mem.u32(a[0] + 4)));
        e.register(0x0043_b4a0, |e, a| int(e.mem.u32(a[0] + 8 + 4 * a[1])));
        e.register(0x0041_3f40, |_, a| int(a[0]));
        e.register(0x0043_b1b0, |e, a| int(e.mem.u32(a[0] + 0x30)));
        // `_strnicmp(name, "BIP", 3)`: the prefix is compared case-insensitively.
        e.register(0x00ec_7ec0, |e, a| {
            let name = e.mem.cstr(a[0]);
            let prefix = e.mem.cstr(a[1]);
            let same = name.len() >= 3 && name[..3].eq_ignore_ascii_case(&prefix[..3]);
            int(if same { 0 } else { 1 })
        });
        e.mem.set_cstr(0x0108_4e60, b"BIP");
        e.register(0x004b_2960, |e, a| int(e.mem.u32(a[0] + 0x34)));
        // The lighting property of a node is the node's address + 1.
        e.register(0x004b_5710, |_, a| int(a[0] + 1));
        // "SkinAttachment" search: a child whose slot-less name matches, via +0x38.
        e.register(0x004a_ae30, |e, a| int(e.mem.u32(a[0] + 0x38)));
        let received = Rc::new(Cell::new(u32::MAX));
        let seen = received.clone();
        e.register_double(0x0f00_0040, move |_, a| {
            seen.set(a[1]);
            Ret::default()
        });
        let process = object_with_slots(&mut e, 0x600, &[(0x4fc, 0x0f00_0040)]);
        let actor = e.new_object::<Actor>();
        e.set(actor, Actor::pCurrentProcess, process);
        (e, actor, received)
    }

    #[test]
    fn lighting_property_comes_from_the_skin_attachment_when_present() {
        let (mut e, actor, received) = lighting_engine();
        let first = node(&mut e, b"root", &[], 0);
        e.register_double(0x0f00_0041, move |_, _| int(first.addr()));
        let root = node(&mut e, b"Fo", &[], 0x0f00_0041);
        let skin = e.mem.alloc(0x60);
        e.mem.set_u32(root.addr() + 0x38, skin);
        e.call(0x008b_0bd0, &args![actor, root]);
        assert_eq!(received.get(), skin + 1);
    }

    #[test]
    fn lighting_property_skips_bip_children_and_takes_the_first_with_one() {
        let (mut e, actor, received) = lighting_engine();
        let grandchild = node(&mut e, b"g", &[0x1], 0);
        e.mem.set_u32(grandchild.addr() + 0x34, 1);
        let bip_inner = node(&mut e, b"x", &[0x1], 0);
        e.mem.set_u32(bip_inner.addr() + 0x34, 1);
        let g1 = grandchild;
        e.register_double(0x0f00_0042, move |_, _| int(g1.addr()));
        let b1 = bip_inner;
        e.register_double(0x0f00_0043, move |_, _| int(b1.addr()));
        let bip = node(&mut e, b"Bip01 Pelvis", &[], 0x0f00_0043);
        let body = node(&mut e, b"Body", &[], 0x0f00_0042);
        let first = node(&mut e, b"hierarchy", &[bip.addr(), body.addr()], 0);
        e.register_double(0x0f00_0041, move |_, _| int(first.addr()));
        let root = node(&mut e, b"root", &[], 0x0f00_0041);
        e.call(0x008b_0bd0, &args![actor, root]);
        assert_eq!(received.get(), grandchild.addr() + 1);
    }

    #[test]
    fn lighting_property_falls_back_to_the_first_child() {
        let (mut e, actor, received) = lighting_engine();
        let first = node(&mut e, b"hierarchy", &[], 0);
        e.register_double(0x0f00_0041, move |_, _| int(first.addr()));
        let root = node(&mut e, b"root", &[], 0x0f00_0041);
        e.call(0x008b_0bd0, &args![actor, root]);
        assert_eq!(received.get(), first.addr() + 1);
        // Without a process or without a node nothing is handed over.
        received.set(u32::MAX);
        e.set(actor, Actor::pCurrentProcess, Ptr::NULL);
        e.call(0x008b_0bd0, &args![actor, root]);
        assert_eq!(received.get(), u32::MAX);
    }

    #[test]
    fn magic_target_thunks_read_the_process_behind_the_interior_pointer() {
        let (mut e, _) = engine();
        let actor = e.new_object::<Actor>();
        let interior = Ptr::<()>::new(actor.addr() + 0xa4);
        assert_eq!(e.call(0x008b_0d30, &args![interior, 5u32]).f32(), 0.0);
        assert_eq!(e.call(0x008b_0d70, &args![interior, 5u32]).f32(), 0.0);
        e.register(0x0f00_0050, |_, a| float(a[1] as f64 + 0.5));
        e.register(0x0f00_0051, |_, a| float(a[1] as f64 + 0.25));
        let process =
            object_with_slots(&mut e, 0x600, &[(0x5e4, 0x0f00_0050), (0x5ec, 0x0f00_0051)]);
        e.set(actor, Actor::pCurrentProcess, process);
        assert_eq!(e.call(0x008b_0d30, &args![interior, 5u32]).f32(), 5.5);
        assert_eq!(e.call(0x008b_0d70, &args![interior, 5u32]).f32(), 5.25);
    }

    #[test]
    fn modifier_thunk_passes_the_embedded_list() {
        let (mut e, _) = engine();
        let actor = e.new_object::<Actor>();
        e.register(0x0093_7730, |_, a| float((a[0] + a[1]) as f64));
        let interior = Ptr::<()>::new(actor.addr() + 0xa4);
        let got = e.call(0x008b_0db0, &args![interior, 3u8]).f32();
        assert_eq!(got, (interior.addr() + 0x2c + 3) as f32);
    }

    /// An engine for the cached value: the process answers slot `0x148`
    /// with an item whose value (`0044ddc0` reads +8) is `0x99`, slot
    /// `0x42c` with the base 10.0, and `00646910` returns `base * 2`.
    fn cached_value_engine() -> (Engine, Ptr<Actor>, Rc<Cell<u32>>) {
        let (mut e, _) = engine();
        e.register(0x0f00_0060, |_, _| int(0x7000));
        e.register(0x0f00_0061, |_, _| float(10.0));
        let process =
            object_with_slots(&mut e, 0x600, &[(0x148, 0x0f00_0060), (0x42c, 0x0f00_0061)]);
        e.register(0x0f00_0062, |_, _| int(0xb10));
        let actor = actor_with_slots(&mut e, &[(0x1e4, 0x0f00_0062)]);
        e.set(actor, Actor::pCurrentProcess, process);
        e.register(0x0044_ddc0, |e, a| {
            int(if a[0] == 0x7000 {
                0x99
            } else {
                e.mem.u32(a[0] + 8)
            })
        });
        e.register(0x008d_8520, |e, a| int(e.mem.u32(a[0] + 0x68)));
        e.register(0x0088_46e0, |_, _| int(0x103));
        e.register(0x0088_4730, |_, _| int(1));
        e.register(0x0049_97b0, |_, _| int(1));
        e.register(0x008b_bc10, |_, _| int(0));
        let args_seen = Rc::new(Cell::new(0));
        let seen = args_seen.clone();
        e.register_double(0x0064_6910, move |_, a| {
            seen.set(a.len() as u32);
            float(f32::from_bits(a[2]) as f64 * 2.0)
        });
        // Entry point 0x22 adds 1.0 to the value it is handed.
        e.register(HANDLE_ENTRY_POINT, |e, a| {
            assert_eq!((a[0], a[2]), (0x22, 0x99));
            let v = e.mem.f32(a[3]);
            e.mem.set_f32(a[3], v + 1.0);
            Ret::default()
        });
        e.register(0x0040_3e20, |_, a| int(a[0] + 4));
        e.mem.set_f32(0x011c_e038, 0.5);
        (e, actor, args_seen)
    }

    #[test]
    fn cached_value_is_returned_when_not_negative() {
        let (mut e, actor, _) = cached_value_engine();
        e.set(actor, Actor::fGunSkillHUD, 4.0);
        e.set(actor, Actor::fGunSkillGun, 5.0);
        e.set(actor, Actor::fGunSkillActor, 6.0);
        e.set(actor, Actor::fGunSkillVATS, 7.0);
        for (selector, expected) in [(0, 4.0), (1, 5.0), (2, 6.0), (3, 7.0)] {
            assert_eq!(
                e.call(0x008b_0dd0, &args![actor, selector as u32]).f32(),
                expected
            );
        }
    }

    #[test]
    fn negative_selector_one_cache_is_reset_to_zero() {
        let (mut e, actor, _) = cached_value_engine();
        e.set(actor, Actor::fGunSkillGun, -1.0);
        assert_eq!(e.call(0x008b_0dd0, &args![actor, 1u32]).f32(), 0.0);
        assert_eq!(e.get(actor, Actor::fGunSkillGun), 0.0);
    }

    #[test]
    fn missing_process_or_item_gives_zero() {
        let (mut e, actor, _) = cached_value_engine();
        e.register(0x0f00_0060, |_, _| int(0));
        assert_eq!(e.call(0x008b_0dd0, &args![actor, 0u32]).f32(), 0.0);
        e.set(actor, Actor::pCurrentProcess, Ptr::NULL);
        assert_eq!(e.call(0x008b_0dd0, &args![actor, 0u32]).f32(), 0.0);
    }

    #[test]
    fn uncached_values_go_through_the_entry_point_and_are_cached() {
        let (mut e, actor, words) = cached_value_engine();
        e.set(actor, Actor::fGunSkillHUD, -1.0);
        e.set(actor, Actor::fGunSkillActor, -1.0);
        // (10 * 2) + 1
        assert_eq!(e.call(0x008b_0dd0, &args![actor, 0u32]).f32(), 21.0);
        assert_eq!(e.get(actor, Actor::fGunSkillHUD), 21.0);
        assert_eq!(words.get(), 7);
        // Setting 2 or 3 uses the other argument list but gives the same value.
        e.mem.set_u32(0x011f_2250 + 8, 2);
        assert_eq!(e.call(0x008b_0dd0, &args![actor, 2u32]).f32(), 21.0);
        assert_eq!(e.get(actor, Actor::fGunSkillActor), 21.0);
    }

    #[test]
    fn selector_three_scales_and_adds_the_base_again() {
        let (mut e, actor, _) = cached_value_engine();
        e.set(actor, Actor::fGunSkillVATS, -1.0);
        // ((10 * 2) + 1) * 0.5 + 10
        assert_eq!(e.call(0x008b_0dd0, &args![actor, 3u32]).f32(), 20.5);
        assert_eq!(e.get(actor, Actor::fGunSkillVATS), 20.5);
    }

    /// A dying actor (slots `0x1e4` animation, `0x21c`, `0x1f4` position,
    /// `0x434`) with a killer, doubles for everything `DoDeathStuff` calls,
    /// and the call log started.
    fn death_engine() -> (Engine, Ptr<Actor>, Ptr<Actor>) {
        let (mut e, _) = engine();
        for fake in [0x0f00_0070, 0x0f00_0072, 0x0f00_0073] {
            e.register(fake, |_, _| int(0));
        }
        e.register(0x0f00_0071, |_, _| int(1));
        let position = e.mem.alloc(12);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *v);
        }
        e.register_double(0x0f00_0074, move |_, _| int(position));
        let actor = actor_with_slots(
            &mut e,
            &[
                (0x1e4, 0x0f00_0070),
                (0x21c, 0x0f00_0072),
                (0x1f4, 0x0f00_0074),
                (0x434, 0x0f00_0073),
            ],
        );
        let killer = actor_with_slots(&mut e, &[(0x218, 0x0f00_0071)]);
        e.set(actor, Actor::pMyKiller, killer.cast());
        let setting = e.mem.alloc(8);
        e.register_double(0x0040_8d60, move |_, _| int(setting + 4));
        e.register(0x0087_f3d0, |_, _| int(0));
        e.register(ACTOR_SET_LIFE_STATE, |_, _| Ret::default());
        e.register(0x0056_7790, |_, _| int(0));
        e.register(0x008c_09e0, |_, _| Ret::default());
        e.register(0x005d_43c0, |_, a| int(a[0] + 0x44));
        e.register(0x005a_c750, |_, _| int(1));
        e.register(ACTOR_GET_PROCESS, |e, a| int(e.mem.u32(a[0] + 0x68)));
        e.register(
            ACTOR_IS_IN_COMBAT,
            |e, a| int(e.mem.u8(a[0] + 0x104) as u32),
        );
        e.register(0x0088_1680, |_, _| Ret::default());
        e.register(0x008b_ca90, |_, _| Ret::default());
        (e, actor, killer.cast())
    }

    fn calls(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap_or_default()
    }

    #[test]
    fn essential_actors_do_not_die() {
        let (mut e, actor, _) = death_engine();
        let setting = e.call(0x0040_8d60, &args![0u32]).u32();
        e.mem.set_u8(setting, 1);
        e.register(0x0087_f3d0, |_, _| int(1));
        e.call_log = Some(vec![]);
        e.call(0x008b_01c0, &args![actor]);
        let log = calls(&mut e);
        assert!(log
            .iter()
            .all(|(address, _)| *address != ACTOR_SET_LIFE_STATE));
        assert_eq!(e.get(actor, Actor::bSetOnDeath), 0);
    }

    #[test]
    fn a_death_animation_without_the_flag_waits() {
        let (mut e, actor, _) = death_engine();
        e.register(0x0f00_0070, |_, _| int(0xa11));
        e.register(0x0049_1040, |_, _| int(1));
        e.register(0x0044_0da0, |_, _| int(0));
        e.call(0x008b_01c0, &args![actor]);
        assert_eq!(e.get(actor, Actor::bSetOnDeath), 0);
        // With the flag set on the model the death goes ahead.
        e.register(0x0044_0da0, |_, _| int(1));
        e.call(0x008b_01c0, &args![actor]);
        assert_eq!(e.get(actor, Actor::bSetOnDeath), 1);
    }

    #[test]
    fn a_second_death_only_sets_the_life_state() {
        let (mut e, actor, _) = death_engine();
        e.set(actor, Actor::bSetOnDeath, 1);
        e.call_log = Some(vec![]);
        e.call(0x008b_01c0, &args![actor]);
        let log = calls(&mut e);
        assert_eq!(
            log,
            vec![
                (0x008b_01c0, vec![actor.addr()]),
                (0x0f00_0070, vec![actor.addr()]),
                (0x0040_8d60, vec![0x011e_0888]),
                (ACTOR_SET_LIFE_STATE, vec![actor.addr(), 2]),
            ]
        );
    }

    #[test]
    fn a_murder_without_witness_reports_through_the_actor() {
        let (mut e, actor, killer) = death_engine();
        e.set(actor, Actor::bMurderAlarm, 1);
        e.call_log = Some(vec![]);
        e.call(0x008b_01c0, &args![actor]);
        let log = calls(&mut e);
        let killer = killer.addr();
        let a = actor.addr();
        assert!(log.contains(&(0x008c_09e0, vec![a, killer])));
        assert!(!log.contains(&(0x005a_c750, vec![killer, a + 0x44, 0x20])));
        assert!(log.contains(&(0x005a_c750, vec![killer, a + 0x44, 0x10])));
        assert_eq!(e.get(actor, Actor::bMurderAlarm), 1);
        assert_eq!(log[log.len() - 3], (ACTOR_SET_LIFE_STATE, vec![a, 2]));
        assert_eq!(log[log.len() - 1], (0x008b_ca90, vec![a, 0]));
    }

    #[test]
    fn a_murder_of_an_owned_actor_raises_the_alarm_for_the_owner() {
        let (mut e, actor, killer) = death_engine();
        e.set(actor, Actor::bMurderAlarm, 1);
        let owner = e.mem.alloc(0x20);
        e.register_double(0x0056_7790, move |_, _| int(owner));
        // Form type 0x2a: the owner is an actor reference in the high process.
        e.register(0x0040_1170, |_, _| int(0x2a));
        e.register(0x0097_0a20, |_, a| int(a[1] + 0x100));
        e.register(0x008c_0460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008b_01c0, &args![actor]);
        let log = calls(&mut e);
        let (killer, a) = (killer.addr(), actor.addr());
        assert!(log.contains(&(0x0097_0a20, vec![PROCESS_LISTS, owner, 0])));
        assert!(log.contains(&(0x005a_c750, vec![killer, a + 0x44, 0x20])));
        assert!(log.contains(&(0x008c_0460, vec![owner + 0x100, killer, 0, 1])));
        assert!(!log.iter().any(|(address, _)| *address == 0x008c_09e0));
        assert_eq!(e.get(actor, Actor::bMurderAlarm), 0);
    }

    #[test]
    fn a_murder_by_an_owner_of_another_form_type_uses_the_other_lookup() {
        let (mut e, actor, _) = death_engine();
        e.set(actor, Actor::bMurderAlarm, 1);
        let owner = e.mem.alloc(0x20);
        e.register_double(0x0056_7790, move |_, _| int(owner));
        e.register(0x0040_1170, |_, _| int(8));
        e.register(0x0097_0b30, |_, _| int(0));
        e.call_log = Some(vec![]);
        e.call(0x008b_01c0, &args![actor]);
        let log = calls(&mut e);
        assert!(log.contains(&(0x0097_0b30, vec![PROCESS_LISTS, owner, 0, 0])));
        // No witness, so no attack alarm (it would be an untranslated call).
        assert_eq!(e.get(actor, Actor::bMurderAlarm), 0);
    }

    #[test]
    fn the_players_kill_reaches_the_players_process_and_the_dying_process_stops() {
        let (mut e, actor, _) = death_engine();
        let player = e.global::<u32>(PLAYER_CHARACTER);
        e.set(actor, Actor::pMyKiller, Ptr::<()>::new(player));
        e.mem.set_u8(actor.addr() + 0x104, 1);
        let received = Rc::new(Cell::new((0, 0, 0, 0)));
        let seen = received.clone();
        e.register_double(0x0f00_0080, move |_, a| {
            assert_eq!(a[5..], [0, 4, 0]);
            seen.set((a[1], a[2], a[3], a[4]));
            Ret::default()
        });
        let player_process = object_with_slots(&mut e, 0x600, &[(0xfc, 0x0f00_0080)]);
        e.mem.set_u32(player + 0x68, player_process.addr());
        e.register(0x0f00_0081, |_, _| Ret::default());
        let limit_seen = Rc::new(Cell::new(0));
        let limit = limit_seen.clone();
        e.register_double(0x0f00_0082, move |_, a| {
            limit.set(a[1]);
            Ret::default()
        });
        e.map(0x0101_6000, 0x1000);
        e.set_global(0x0101_6970, 3.0e38f32);
        let process = object_with_slots(
            &mut e,
            0x600,
            &[
                (0x4b4, 0x0f00_0082),
                (0x4c0, 0x0f00_0081),
                (0x214, 0x0f00_0081),
                (0x234, 0x0f00_0081),
            ],
        );
        e.set(actor, Actor::pCurrentProcess, process);
        e.register(0x0f00_0071, |_, _| int(1));
        e.call_log = Some(vec![]);
        e.call(0x008b_01c0, &args![actor]);
        let log = calls(&mut e);
        assert_eq!(
            received.get(),
            (player, 1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits())
        );
        // The player's kill is not an alarm and the process is told to stop:
        // end of interrupt package, then slots 0x4b4/0x4c0/0x214/0x234 ran.
        assert!(log.contains(&(0x0088_1680, vec![actor.addr(), 0])));
        assert_eq!(limit_seen.get(), 3.0e38f32.to_bits());
    }

    type SlotLog = Rc<std::cell::RefCell<Vec<(u32, u32, Vec<u32>)>>>;
    type Answers = Rc<std::cell::RefCell<std::collections::HashMap<(u32, u32), u32>>>;

    /// Every virtual slot used by `fn_008b1070` on actors and processes
    /// points at a recording double: `log` gets `(object, slot, arguments)`
    /// and the result is `answers[(object, slot)]` (default 0).
    struct World {
        e: Engine,
        log: SlotLog,
        answers: Answers,
    }

    const DIALOGUE_SLOTS: [u32; 42] = [
        0x24c, 0x20c, 0x22c, 0x650, 0x624, 0x374, 0x2ec, 0x214, 0x714, 0x71c, 0x614, 0x674, 0x14c,
        0x148, 0x3ec, 0x100, 0x274, 0x1e4, 0x128, 0x8c, 0x644, 0x634, 0x28, 0x710, 0x280, 0x238,
        0x90, 0xfc, 0x288, 0x2f4, 0x48, 0x12c, 0x4bc, 0x424, 0x428, 0x454, 0x14, 0x21c, 0x1d0,
        0x1f4, 0x8, 0x254,
    ];

    fn world() -> World {
        let (mut e, _) = engine();
        let log: SlotLog = Default::default();
        let answers: Answers = Default::default();
        for slot in DIALOGUE_SLOTS {
            let (log, answers) = (log.clone(), answers.clone());
            e.register_double(0x0f20_0000 + slot, move |_, a| {
                log.borrow_mut().push((a[0], slot, a[1..].to_vec()));
                int(answers.borrow().get(&(a[0], slot)).copied().unwrap_or(0))
            });
        }
        e.register(ACTOR_GET_PROCESS, |e, a| int(e.mem.u32(a[0] + 0x68)));
        e.register(0x0047_c850, |_, _| int(0));
        e.register(0x005d_43c0, |_, a| int(a[0] + 0x44));
        e.register(0x0042_e130, |_, _| Ret::default());
        e.register(0x0067_27b0, |_, _| int(0));
        e.register(0x00ad_8ce0, |_, _| int(0));
        e.register(0x0084_e3a0, |e, a| int(e.mem.u32(a[0] + 4)));
        e.register(0x0040_36b0, |_, _| int(0));
        e.register(0x0093_44a0, |e, a| int(e.mem.u32(a[0] + 0x1f0)));
        e.register(0x0093_36c0, |_, _| int(1));
        e.map(0x011f_1000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        World { e, log, answers }
    }

    impl World {
        fn slots() -> Vec<(u32, u32)> {
            DIALOGUE_SLOTS
                .iter()
                .map(|s| (*s, 0x0f20_0000 + s))
                .collect()
        }

        /// An actor with a process; both have every recorded slot.
        fn actor(&mut self) -> (Ptr<Actor>, Ptr) {
            let actor: Ptr<Actor> = object_with_slots(&mut self.e, 0x1b4, &Self::slots()).cast();
            let process = object_with_slots(&mut self.e, 0x800, &Self::slots());
            self.e.set(actor, Actor::pCurrentProcess, process);
            (actor, process)
        }

        fn answer(&self, object: u32, slot: u32, value: u32) {
            self.answers.borrow_mut().insert((object, slot), value);
        }

        fn calls_on(&self, object: u32) -> Vec<(u32, Vec<u32>)> {
            self.log
                .borrow()
                .iter()
                .filter(|(o, _, _)| *o == object)
                .map(|(_, s, a)| (*s, a.clone()))
                .collect()
        }

        /// A package with a kind-0x1c marker (`41ca90`), starter and target.
        fn dialogue(&mut self, starter: Ptr<Actor>, target: Ptr<Actor>) -> u32 {
            let package = self.e.mem.alloc(0x100);
            let (s, t) = (starter.addr(), target.addr());
            self.e.register_double(0x0041_ca90, move |_, a| {
                int(if a[0] == package { 0x1c } else { 0 })
            });
            self.e.register_double(0x009e_e040, move |_, _| int(s));
            self.e.register_double(0x008d_80e0, move |_, _| int(t));
            package
        }
    }

    #[test]
    fn package_change_needs_a_process_and_no_blocking_setting() {
        let mut w = world();
        let actor = w.e.new_object::<Actor>();
        w.e.call(0x008b_1070, &args![actor]);
        let (actor, process) = w.actor();
        w.e.register(0x0047_c850, |_, _| int(1));
        w.e.call(0x008b_1070, &args![actor]);
        assert!(w.log.borrow().is_empty());
        let _ = process;
    }

    #[test]
    fn without_a_dialogue_package_the_process_is_reset() {
        let mut w = world();
        let (actor, process) = w.actor();
        w.answer(process.addr(), 0x674, 0x44);
        w.e.register(0x0041_ca90, |_, _| int(0));
        w.e.call(0x008b_1070, &args![actor]);
        assert_eq!(
            w.calls_on(process.addr()),
            vec![
                (0x24c, vec![0]),
                (0x20c, vec![]),
                (0x22c, vec![]),
                (0x674, vec![4]),
                (0x650, vec![1]),
                (0x624, vec![0x44]),
                (0x71c, vec![0]),
                (0x14c, vec![]),
                (0x148, vec![]),
            ]
        );
        assert_eq!(w.e.mem.f32(actor.addr() + 0x74), 1.0);
    }

    #[test]
    fn the_current_dialogue_package_points_both_speakers_at_each_other() {
        let mut w = world();
        let (starter, starter_process) = w.actor();
        let (target, target_process) = w.actor();
        let package = w.dialogue(starter, target);
        w.answer(starter_process.addr(), 0x20c, package);
        w.answer(target_process.addr(), 0x20c, package);
        w.e.mem.set_u32(target.addr() + 0x1f0, package);
        w.e.call(0x008b_1070, &args![starter]);
        let (s, t) = (starter.addr(), target.addr());
        assert_eq!(
            w.calls_on(starter_process.addr()),
            vec![
                (0x24c, vec![0]),
                (0x20c, vec![]),
                (0x650, vec![1]),
                (0x624, vec![t]),
                (0x374, vec![0]),
                (0x2ec, vec![0]),
                (0x214, vec![]),
                (0x71c, vec![0]),
                (0x14c, vec![]),
                (0x148, vec![]),
            ]
        );
        assert_eq!(
            w.calls_on(target_process.addr()),
            vec![
                (0x20c, vec![]),
                (0x650, vec![1]),
                (0x624, vec![s]),
                (0x374, vec![0]),
                (0x2ec, vec![0]),
                (0x214, vec![]),
            ]
        );
        // Both speakers are asked (slots 0x274, 0x100) and the lip hold is not set.
        assert_eq!(w.e.mem.f32(s + 0x74), 1.0);
        assert_eq!(w.e.mem.f32(t + 0x74), 1.0);
        assert_eq!(w.e.mem.u8(s + 0x80), 0);
    }

    #[test]
    fn the_next_dialogue_package_uses_the_restart_slot_and_cancels_lip_files() {
        let mut w = world();
        let (starter, starter_process) = w.actor();
        let (target, target_process) = w.actor();
        let package = w.dialogue(starter, target);
        w.answer(starter_process.addr(), 0x22c, package);
        // `IsinDialogue` is true (set up) and the target's own package is not a dialogue.
        w.answer(starter.addr(), 0x100, 1);
        let lip = Rc::new(Cell::new(0));
        let seen = lip.clone();
        w.e.register_double(0x0090_6050, move |_, a| {
            assert_eq!(a[0], LIP_SYNC_MANAGER);
            seen.set(seen.get() + 1);
            Ret::default()
        });
        w.e.call(0x008b_1070, &args![starter]);
        let (s, t) = (starter.addr(), target.addr());
        assert_eq!(lip.get(), 1);
        assert_eq!(w.e.mem.u8(s + 0x80), 1);
        assert_eq!(w.e.mem.u8(t + 0x80), 0);
        let on_starter = w.calls_on(starter_process.addr());
        assert!(on_starter.contains(&(0x714, vec![s])));
        assert!(!on_starter.contains(&(0x214, vec![])));
        let on_target = w.calls_on(target_process.addr());
        // The target's package (answer 0) is not a dialogue: slot 0x714 with itself.
        assert!(on_target.contains(&(0x714, vec![t])));
    }

    #[test]
    fn a_target_that_is_already_in_the_package_ends_the_change_early() {
        let mut w = world();
        let (starter, starter_process) = w.actor();
        let (target, target_process) = w.actor();
        let package = w.dialogue(starter, target);
        w.answer(starter_process.addr(), 0x20c, package);
        w.answer(target_process.addr(), 0x20c, package);
        // The package waits on a type-0xef condition.
        let waiting = w.e.mem.alloc(0x20);
        w.e.mem.set_u32(package + 0x8c, waiting);
        w.e.mem.set_u32(waiting + 4, 0xef);
        w.e.call(0x008b_1070, &args![starter]);
        // Only the early slot calls ran, then the starter's process slot 0x214.
        assert_eq!(
            w.calls_on(starter_process.addr()),
            vec![(0x24c, vec![0]), (0x20c, vec![]), (0x214, vec![])]
        );
        assert!(w
            .calls_on(target_process.addr())
            .iter()
            .all(|(slot, _)| *slot == 0x20c));
    }

    #[test]
    fn a_target_busy_elsewhere_is_left_alone() {
        let mut w = world();
        let (starter, starter_process) = w.actor();
        let (target, target_process) = w.actor();
        let package = w.dialogue(starter, target);
        w.answer(starter_process.addr(), 0x20c, package);
        // The target's process runs another package (slot 0x20c answers 0).
        w.e.call(0x008b_1070, &args![starter]);
        assert!(w
            .calls_on(target_process.addr())
            .iter()
            .all(|(slot, _)| *slot == 0x20c));
    }

    #[test]
    fn the_players_conversation_field_is_cleared() {
        let mut w = world();
        let (starter, starter_process) = w.actor();
        let (target, _) = w.actor();
        let package = w.dialogue(starter, target);
        w.answer(starter_process.addr(), 0x20c, package);
        let player = w.e.global::<u32>(PLAYER_CHARACTER);
        w.e.mem.map(player, 0x300);
        w.e.mem.set_u32(player + 0x208, package);
        w.e.call(0x008b_1070, &args![starter]);
        assert_eq!(w.e.mem.u32(player + 0x208), 0);
    }

    #[test]
    fn a_weapon_without_regeneration_updates_the_actor() {
        let mut w = world();
        let (actor, process) = w.actor();
        w.e.register(0x0041_ca90, |_, _| int(0));
        w.answer(process.addr(), 0x148, 0x5000);
        w.e.register(0x004b_da70, |_, a| int(if a[1] == 6 { 1 } else { 0 }));
        w.e.register(0x0044_ddc0, |_, _| int(0x77));
        let rate = Rc::new(Cell::new(0.0));
        let shown = rate.clone();
        w.e.register_double(0x0070_9430, move |_, a| {
            assert_eq!(a, [0x77, 1]);
            float(shown.get())
        });
        w.e.register(0x008a_16d0, |_, _| int(1));
        w.e.call(0x008b_1070, &args![actor]);
        let on_actor = w.calls_on(actor.addr());
        assert_eq!(on_actor, vec![(0x3ec, vec![0x77, 1, 0, 0])]);
        // A positive regeneration rate leaves the actor alone.
        w.log.borrow_mut().clear();
        rate.set(0.5);
        w.e.call(0x008b_1070, &args![actor]);
        assert!(w.calls_on(actor.addr()).is_empty());
        // So does an item in the second slot.
        rate.set(-1.0);
        w.answer(process.addr(), 0x14c, 0x6000);
        w.e.call(0x008b_1070, &args![actor]);
        assert!(w.calls_on(actor.addr()).is_empty());
    }

    #[test]
    fn the_talker_is_recorded_and_handed_to_the_process() {
        let mut w = world();
        let (actor, process) = w.actor();
        let (talker, _) = w.actor();
        let form = w.e.mem.alloc(0x100);
        w.e.register_double(0x007a_f430, move |_, _| int(form));
        w.e.register(0x005d_43c0, |_, a| int(a[0] + 0x44));
        w.e.register(0x0042_e060, |_, _| Ret::default());
        let talking = w.e.mem.alloc(0x20);
        w.e.mem.set_u32(talking + 0xc, 0x4321);
        w.e.register_double(0x0042_e110, move |_, _| int(talking));
        w.e.register(0x0051_6bf0, |e, a| int(e.mem.u32(a[0] + 0x90)));
        let out = w.e.mem.alloc(4);
        w.e.call_log = Some(vec![]);
        let result = w.e.call(0x008b_1910, &args![actor, talker, out]).u32();
        let log = w.e.call_log.take().unwrap();
        assert_eq!(result, 0x4321);
        assert_eq!(w.e.mem.u32(out), 0x4321);
        assert!(log.contains(&(0x0042_e060, vec![talker.addr() + 0x44, form, talker.addr()])));
        assert_eq!(w.calls_on(talker.addr()), vec![(0x48, vec![0x8000_0000])]);
        assert_eq!(w.calls_on(process.addr()), vec![(0x12c, vec![0x4321])]);
        // The form's 0x90 field replaces it.
        w.e.mem.set_u32(form + 0x90, 0x8888);
        assert_eq!(
            w.e.call(0x008b_1910, &args![actor, talker, out]).u32(),
            0x8888
        );
        assert_eq!(w.e.mem.u32(out), 0x4321);
        // Without talking-actor extra the talker itself is handed over.
        w.e.register(0x0042_e110, |_, _| int(0));
        assert_eq!(
            w.e.call(0x008b_1910, &args![actor, talker, out]).u32(),
            talker.addr()
        );
    }

    #[test]
    fn bit_one_of_the_word_at_0x1c() {
        let (mut e, _) = engine();
        let object = e.mem.alloc(0x40);
        assert!(!e.call(0x008b_1ff0, &args![Ptr::<()>::new(object)]).bool());
        e.mem.set_u32(object + 0x1c, 2);
        assert!(e.call(0x008b_1ff0, &args![Ptr::<()>::new(object)]).bool());
        e.mem.set_u32(object + 0x1c, 0xfd);
        assert!(!e.call(0x008b_1ff0, &args![Ptr::<()>::new(object)]).bool());
    }

    /// Idle manager doubles: arrays are `[count at +4, forms from +8]`,
    /// the root array answers for `600560`, the loose one for `600630`.
    fn idle_engine(root: &[u32], loose: &[u32]) -> Engine {
        let (mut e, _) = engine();
        e.map(0x011c_b000, 0x1000);
        e.set_global(0x011c_b6a0, 0x7777u32);
        let mut array = |forms: &[u32]| {
            if forms.is_empty() {
                return 0;
            }
            let a = e.mem.alloc(8 + 4 * forms.len() as u32);
            e.mem.set_u32(a + 4, forms.len() as u32);
            for (i, f) in forms.iter().enumerate() {
                e.mem.set_u32(a + 8 + 4 * i as u32, *f);
            }
            a
        };
        let (root, loose) = (array(root), array(loose));
        e.register(0x0057_15d0, |_, _| int(0xaa));
        e.register(0x0060_07f0, |e, a| {
            assert_eq!(a[0], 0xaa);
            e.mem.set_cstr(a[1], b"meshes\\idles");
            Ret::default()
        });
        e.register_double(0x0060_0560, move |e, a| {
            assert_eq!(a[0], 0x7777);
            assert_eq!(e.mem.cstr(a[1]), b"meshes\\idles");
            int(root)
        });
        e.register_double(0x0060_0630, move |_, a| {
            assert_eq!(a[0], 0x7777);
            int(loose)
        });
        e.register(0x0084_e3a0, |e, a| int(e.mem.u32(a[0] + 4)));
        e.register(0x004d_6170, |e, a| int(e.mem.u32(a[0] + 8 + 4 * a[1])));
        // Idle 0x500 links to 0x600 (a location chain); nothing else links.
        e.register(0x0080_41a0, |_, a| {
            int(if a[0] == 0x500 { 0x600 } else { 0 })
        });
        e
    }

    #[test]
    fn idles_are_usable_when_listed_in_either_array() {
        let mut e = idle_engine(&[0x10, 0x20], &[0x30]);
        let actor = e.new_object::<Actor>();
        let can = |e: &mut Engine, idle: u32| e.call(0x008b_2010, &args![actor, idle]).bool();
        assert!(!can(&mut e, 0));
        assert!(can(&mut e, 0x20));
        assert!(can(&mut e, 0x30));
        assert!(!can(&mut e, 0x40));
    }

    #[test]
    fn idles_follow_their_location_chain_and_survive_empty_arrays() {
        let mut e = idle_engine(&[], &[0x600]);
        let actor = e.new_object::<Actor>();
        assert!(e.call(0x008b_2010, &args![actor, 0x500u32]).bool());
        let mut e = idle_engine(&[], &[]);
        let actor = e.new_object::<Actor>();
        assert!(!e.call(0x008b_2010, &args![actor, 0x500u32]).bool());
    }

    /// A conversation between `this` (with a process answering slot 0x128
    /// with the partner) and a mobile partner, with doubles for every
    /// package setter.
    struct Conversation {
        w: World,
        this: Ptr<Actor>,
        this_process: Ptr,
        partner: Ptr<Actor>,
        partner_process: Ptr,
        source: u32,
    }

    fn conversation() -> Conversation {
        let mut w = world();
        let (this, this_process) = w.actor();
        let (partner, partner_process) = w.actor();
        w.answer(this_process.addr(), 0x128, partner.addr());
        w.answer(partner.addr(), 0xfc, 1);
        w.answer(this_process.addr(), 0x90, 0x9090);
        for address in [
            0x0067_a1b0u32,
            0x0067_0fc0,
            0x0082_6b40,
            0x0082_6b90,
            0x0098_4f60,
            0x0067_1d30,
            0x0068_0110,
            0x0068_00b0,
            0x0040_3550,
            0x0067_2fc0,
            0x007b_3fa0,
            0x0067_2aa0,
            0x0067_2c40,
            0x0067_29d0,
            0x0067_2b70,
            0x005c_8a30,
            0x0048_3710,
            0x0057_bd60,
            0x0067_1e10,
        ] {
            w.e.register(address, |_, _| Ret::default());
        }
        for address in [
            0x0067_a690u32,
            0x0067_2800,
            0x0067_27b0,
            0x0067_2dd0,
            0x005f_36f0,
            0x0067_efd0,
            0x0055_b980,
            0x0067_1d10,
            0x0044_ddc0,
        ] {
            w.e.register(address, |_, _| int(0));
        }
        w.e.register(OPERATOR_NEW, |e, a| int(e.mem.alloc(a[0])));
        w.e.register(DIALOGUE_PACKAGE_CONSTRUCTOR, |_, a| int(a[0]));
        w.e.register(0x0067_ff70, |_, a| int(a[0]));
        w.e.register(0x0067_2760, |_, _| int(0x70));
        w.e.register(0x0067_2850, |_, _| float(2.5));
        w.e.register(0x0093_44a0, |_, _| int(0x1234));
        let source = w.e.mem.alloc(0x100);
        w.e.mem.set_u32(source + 0x1c, 2);
        Conversation {
            w,
            this,
            this_process,
            partner,
            partner_process,
            source,
        }
    }

    #[test]
    fn a_mobile_partner_gets_the_package_and_the_conversation_starts() {
        let mut c = conversation();
        c.w.e.call_log = Some(vec![]);
        let started = c.w.e.call(0x008b_19c0, &args![c.this, c.source]).bool();
        let log = c.w.e.call_log.take().unwrap();
        assert!(started);
        let (this, partner, source) = (c.this.addr(), c.partner.addr(), c.source);
        // The package is built from the source package.
        let build = log
            .iter()
            .find(|(address, _)| *address == DIALOGUE_PACKAGE_CONSTRUCTOR)
            .unwrap();
        let package = build.1[0];
        assert_eq!(build.1[1..], [0, this, partner, 0, 0, 1]);
        assert!(log.contains(&(0x0067_a1b0, vec![package, source])));
        assert!(log.contains(&(0x0067_0fc0, vec![package, 0x1c])));
        // Bit 1 of the source's word at +0x1C, then the other flag getter.
        assert!(log.contains(&(0x0082_6b40, vec![package, 1])));
        assert!(log.contains(&(0x0082_6b90, vec![package, 0])));
        assert!(log.contains(&(0x0098_4f60, vec![package, 10])));
        assert!(log.contains(&(0x0067_2c40, vec![package, 2.5f32.to_bits()])));
        assert!(log.contains(&(0x0067_29d0, vec![package, 0x70])));
        assert_eq!(c.w.e.mem.u32(package + 0x8c), 0x70);
        assert_eq!(c.w.e.mem.u32(package + 0x9c), 0x9090);
        // The package target refers to the partner (allocation is 0x10 bytes).
        let target_set = log
            .iter()
            .find(|(address, _)| *address == 0x0068_0110)
            .unwrap();
        assert_eq!(target_set.1[1], partner);
        assert!(log.contains(&(0x0068_00b0, vec![target_set.1[0], 0])));
        assert!(log.contains(&(0x0067_2fc0, vec![package, target_set.1[0]])));
        assert!(log.contains(&(0x007b_3fa0, vec![target_set.1[0], 1])));
        assert!(!log.iter().any(|(address, _)| *address == 0x0067_1e10));
        // The partner is told to talk back (slots 0x710, 0x2f4, 0x12c, 0x288).
        assert!(log.contains(&(0x0057_bd60, vec![this, partner])));
        assert!(log.contains(&(0x0057_bd60, vec![partner, this])));
        let on_partner = c.w.calls_on(c.partner_process.addr());
        assert_eq!(
            on_partner,
            vec![
                (0x710, vec![partner]),
                (0x12c, vec![partner]),
                (0x288, vec![partner, 1])
            ]
        );
        let on_this = c.w.calls_on(c.this_process.addr());
        assert!(on_this.contains(&(0x710, vec![this])));
        assert!(on_this.contains(&(0x12c, vec![partner])));
        assert!(on_this.contains(&(0x238, vec![1])));
        assert!(on_this.contains(&(0x644, vec![1])));
        assert!(on_this.contains(&(0x634, vec![partner])));
        assert!(c.w.calls_on(partner).contains(&(0x288, vec![])));
        assert!(c.w.calls_on(this).contains(&(0x2f4, vec![package, 0, 1])));
    }

    #[test]
    fn a_source_with_headtracking_off_skips_the_head_calls() {
        let mut c = conversation();
        c.w.e.register(0x0067_2800, |_, _| int(1));
        c.w.e.call(0x008b_19c0, &args![c.this, c.source]);
        let on_this = c.w.calls_on(c.this_process.addr());
        assert!(!on_this
            .iter()
            .any(|(slot, _)| *slot == 0x644 || *slot == 0x634));
        // The process already answering 1 to slot 0x280 skips slot 0x238.
        c.w.answer(c.this_process.addr(), 0x280, 1);
        c.w.log.borrow_mut().clear();
        c.w.e.call(0x008b_19c0, &args![c.this, c.source]);
        assert!(!c
            .w
            .calls_on(c.this_process.addr())
            .iter()
            .any(|(slot, _)| *slot == 0x238));
    }

    #[test]
    fn a_second_location_and_a_busy_target_are_forwarded() {
        let mut c = conversation();
        c.w.e.register(0x0067_2dd0, |_, _| int(0x2222));
        c.w.e.call_log = Some(vec![]);
        c.w.e.call(0x008b_19c0, &args![c.this, c.source]);
        let log = c.w.e.call_log.take().unwrap();
        let package = log
            .iter()
            .find(|(address, _)| *address == DIALOGUE_PACKAGE_CONSTRUCTOR)
            .unwrap()
            .1[0];
        assert!(log.contains(&(0x0067_1e10, vec![package, 0x2222])));
        // A second location suppresses the partner's own package.
        assert!(c.w.calls_on(c.partner_process.addr()).is_empty());
    }

    #[test]
    fn the_player_as_partner_is_not_given_the_package() {
        let mut c = conversation();
        let (player, _) = c.w.actor();
        c.w.e.set_global(PLAYER_CHARACTER, player.addr());
        c.w.answer(c.this_process.addr(), 0x128, player.addr());
        c.w.answer(player.addr(), 0xfc, 1);
        c.w.e.call(0x008b_19c0, &args![c.this, c.source]);
        // Slot 0x22c of the player is asked with 0 and nothing is sent to its process.
        assert!(c.w.calls_on(player.addr()).contains(&(0x22c, vec![0])));
        assert!(c.w.calls_on(c.partner_process.addr()).is_empty());
    }

    #[test]
    fn no_partner_or_a_partner_without_a_process_means_no_conversation() {
        let mut c = conversation();
        c.w.answer(c.this_process.addr(), 0x128, 0);
        assert!(!c.w.e.call(0x008b_19c0, &args![c.this, c.source]).bool());
        // The process was asked to look for one through slot 0x8c.
        assert!(c
            .w
            .calls_on(c.this_process.addr())
            .contains(&(0x8c, vec![c.this.addr()])));
        c.w.answer(c.this_process.addr(), 0x128, c.partner.addr());
        c.w.e.set(c.partner, Actor::pCurrentProcess, Ptr::NULL);
        assert!(!c.w.e.call(0x008b_19c0, &args![c.this, c.source]).bool());
    }

    #[test]
    fn a_non_mobile_talker_of_kind_0x16_yields_the_extra_data_actor() {
        let mut c = conversation();
        let talker = c.w.e.mem.alloc(0x40);
        // A talker object without slot 0xfc support: its vtable answers 0.
        let (not_mobile, _) = c.w.actor();
        c.w.answer(not_mobile.addr(), 0xfc, 0);
        c.w.answer(c.this_process.addr(), 0x128, not_mobile.addr());
        c.w.e.register_double(0x007a_f430, move |_, _| int(talker));
        c.w.e.register(0x0040_1170, |_, _| int(0x16));
        c.w.e.register(0x005d_43c0, |_, a| int(a[0] + 0x44));
        c.w.e.register(0x0042_e060, |_, _| Ret::default());
        let holder = c.w.e.mem.alloc(0x20);
        c.w.e.mem.set_u32(holder + 0xc, c.partner.addr());
        c.w.e.register_double(0x0042_e110, move |_, _| int(holder));
        c.w.e.call_log = Some(vec![]);
        assert!(c.w.e.call(0x008b_19c0, &args![c.this, c.source]).bool());
        let log = c.w.e.call_log.take().unwrap();
        let build = log
            .iter()
            .find(|(address, _)| *address == DIALOGUE_PACKAGE_CONSTRUCTOR)
            .unwrap();
        // Partner and the extra-data actor (argument 5) are the same.
        assert_eq!(build.1[3], c.partner.addr());
        assert_eq!(build.1[4], c.partner.addr());
        // Another kind of form ends the call.
        c.w.e.register(0x0040_1170, |_, _| int(0x17));
        assert!(!c.w.e.call(0x008b_19c0, &args![c.this, c.source]).bool());
    }

    /// `this`, a partner (both with processes) and doubles for everything
    /// `fn_008b2170` calls. Test knobs live in the partner's unused bytes:
    /// `+0x1f4` trying to enter furniture, `+0x1f8`/`+0x1fc` dismembered in
    /// limbs 1/2 (read from `this`), `+0x200` the first-item answer.
    fn talk_world() -> (World, Ptr<Actor>, Ptr, Ptr<Actor>, Ptr) {
        let mut w = world();
        let (this, this_process) = w.actor();
        let (partner, partner_process) = w.actor();
        w.answer(partner.addr(), 0x100, 1);
        w.e.register(0x008c_13d0, |e, a| int(e.mem.u8(a[0] + 0x1f4) as u32));
        w.e.register(
            0x0057_3090,
            |e, a| int(e.mem.u8(a[0] + 0x1f6 + a[1]) as u32),
        );
        w.e.register(OPERATOR_NEW, |e, a| int(e.mem.alloc(a[0])));
        w.e.register(CONVERSATION_CONSTRUCTOR, |_, a| int(a[0]));
        w.e.register(0x0093_4250, |_, _| Ret::default());
        w.e.register(0x0083_b9a0, |e, a| int(e.mem.u8(a[0] + 4) as u32));
        w.e.register(0x0079_8450, |_, _| Ret::default());
        w.e.register(DIALOGUE_PACKAGE_CONSTRUCTOR, |_, a| int(a[0]));
        for address in [
            0x0067_a1b0u32,
            0x0067_0fc0,
            0x0082_6b40,
            0x0082_6b90,
            0x0067_29d0,
            0x0067_2aa0,
            0x0067_a6b0,
            0x0067_f030,
            0x0067_f110,
            0x0067_f4d0,
            0x0067_f3c0,
            0x0067_1d30,
            0x0067_f1f0,
            0x0067_1e10,
            0x0067_2fc0,
            0x007b_3fa0,
            0x0098_4f60,
            0x0068_00b0,
            0x0068_0110,
            0x0040_3550,
            0x0057_bd60,
            0x0048_3710,
        ] {
            w.e.register(address, |_, _| Ret::default());
        }
        w.e.register(0x0067_ff70, |_, a| int(a[0]));
        w.e.register(0x0067_1d10, |_, a| int(a[0] + 0x30));
        w.e.register(0x0067_6280, |_, _| int(40));
        w.e.register(0x0040_3e20, |_, a| int(a[0] + 4));
        w.e.map(0x011c_c000, 0x1000);
        w.e.mem.set_f32(0x011c_cf3c, 2.5);
        w.e.register(0x0093_44a0, |e, a| int(e.mem.u32(a[0] + 0x204)));
        w.answer(this_process.addr(), 0x22c, 0x5151);
        (w, this, this_process, partner, partner_process)
    }

    fn talk(w: &mut World, this: Ptr<Actor>, partner: Ptr<Actor>, flags: [bool; 5]) -> bool {
        w.e.call(
            0x008b_2170,
            &args![
                this, partner, 0u32, 0u32, flags[0], flags[1], flags[2], 0x70u32, flags[3],
                flags[4]
            ],
        )
        .bool()
    }

    #[test]
    fn a_conversation_needs_a_partner_with_a_process() {
        let (mut w, this, _, partner, _) = talk_world();
        // No partner at all.
        assert!(!w
            .e
            .call(
                0x008b_2170,
                &args![this, 0u32, 0u32, 0u32, false, false, false, 0u32, false, false]
            )
            .bool());
        // Partner without a process.
        let (loner, _) = w.actor();
        w.e.set(loner, Actor::pCurrentProcess, Ptr::NULL);
        assert!(!talk(&mut w, this, loner, [false; 5]));
        // This actor without a process.
        let (idle, _) = w.actor();
        w.e.set(idle, Actor::pCurrentProcess, Ptr::NULL);
        assert!(!talk(&mut w, idle, partner, [false; 5]));
    }

    #[test]
    fn a_partner_entering_furniture_or_a_dismembered_speaker_stops_the_conversation() {
        let (mut w, this, _, partner, _) = talk_world();
        w.e.mem.set_u8(partner.addr() + 0x1f4, 1);
        assert!(!talk(&mut w, this, partner, [false; 5]));
        w.e.mem.set_u8(partner.addr() + 0x1f4, 0);
        // Slot 0x22c of this actor true and limb 2 dismembered.
        w.answer(this.addr(), 0x22c, 1);
        w.e.mem.set_u8(this.addr() + 0x1f8, 1);
        assert!(!talk(&mut w, this, partner, [false; 5]));
        // Without the dismemberment the conversation goes on to the item check.
        w.e.mem.set_u8(this.addr() + 0x1f8, 0);
        w.e.mem.set_u8(partner.addr() + 0x204, 0);
        let _ = talk(&mut w, this, partner, [false; 5]);
    }

    #[test]
    fn a_conversation_without_a_first_item_is_dropped() {
        let (mut w, this, _, partner, _) = talk_world();
        w.e.call_log = Some(vec![]);
        // The Conversation object is a fresh block whose byte +4 answers FirstItem: 0.
        assert!(!talk(&mut w, this, partner, [false; 5]));
        let log = w.e.call_log.take().unwrap();
        let conversation = log
            .iter()
            .find(|(address, _)| *address == CONVERSATION_CONSTRUCTOR)
            .unwrap()
            .1[0];
        assert!(log.contains(&(0x0079_8450, vec![conversation, 1])));
        assert!(!log
            .iter()
            .any(|(address, _)| *address == DIALOGUE_PACKAGE_CONSTRUCTOR));
    }

    /// The package the partner has to be running: its address is only
    /// known during the call, so the partner double reads it from the log.
    fn run_conversation(flags: [bool; 5], kind: u32) -> ConversationRun {
        let (mut w, this, this_process, partner, partner_process) = talk_world();
        // FirstItem answers true for every conversation block.
        w.e.register(0x0083_b9a0, |_, _| int(1));
        // The partner accepts the package: 9344a0 answers the package built last.
        let built = Rc::new(Cell::new(0));
        let seen = built.clone();
        w.e.register_double(DIALOGUE_PACKAGE_CONSTRUCTOR, move |_, a| {
            seen.set(a[0]);
            int(a[0])
        });
        let accepted = built.clone();
        w.e.register_double(0x0093_44a0, move |_, _| int(accepted.get()));
        w.answer(this_process.addr(), 0x4bc, kind);
        w.e.call_log = Some(vec![]);
        let result = talk(&mut w, this, partner, flags);
        let log = w.e.call_log.take().unwrap();
        (w, result, log, this, partner, this_process, partner_process)
    }

    #[test]
    fn a_conversation_builds_and_configures_the_package() {
        let (w, result, log, this, partner, this_process, partner_process) =
            run_conversation([true, true, false, false, false], 0);
        assert!(result);
        let (t, p) = (this.addr(), partner.addr());
        let build = log
            .iter()
            .find(|(address, _)| *address == DIALOGUE_PACKAGE_CONSTRUCTOR)
            .unwrap();
        let package = build.1[0];
        let conversation = build.1[1];
        assert_eq!(build.1[2..], [t, p, 0, 1, 1]);
        assert_eq!(w.e.mem.u32(package + 0x8c), 0x70);
        // Current package 0x5151 of the process is copied; kind 0x1c; topic.
        assert!(log.contains(&(0x0067_a1b0, vec![package, 0x5151])));
        assert!(log.contains(&(0x0067_0fc0, vec![package, 0x1c])));
        assert!(log.contains(&(0x0067_29d0, vec![package, 0x70])));
        // Option bytes: +0xbe = option_a (true), +0xbf and +0xc0 false.
        assert_eq!(w.e.mem.u8(package + 0xbe), 1);
        assert_eq!(w.e.mem.u8(package + 0xbf), 0);
        assert_eq!(w.e.mem.u8(package + 0xc0), 0);
        // The head-track flag turns off the dialogue-without-head flag call.
        assert!(log.contains(&(0x0067_a6b0, vec![package, 0])));
        // The second location radius: truncated 2.5 + 40.
        let radius = log
            .iter()
            .find(|(address, _)| *address == 0x0067_f1f0)
            .unwrap();
        assert_eq!(radius.1[1], 42);
        // Both locations are constructed and destroyed, the first one bound to this actor.
        let first = log
            .iter()
            .find(|(address, _)| *address == 0x0067_f3c0)
            .unwrap()
            .1[0];
        assert_eq!(radius.1[0], first + 0xc);
        assert!(log.contains(&(0x0067_f3c0, vec![first, t])));
        assert!(log.contains(&(0x0067_f110, vec![first + 0xc])));
        assert!(log.contains(&(0x0067_f110, vec![first])));
        // Targets: the partner as reference; 0x5a for process kind other than 4.
        assert!(log.contains(&(0x0068_0110, vec![package + 0x30, p])));
        assert!(log.contains(&(0x0068_00b0, vec![package + 0x30, 0])));
        assert!(log.contains(&(0x0040_3550, vec![package + 0x30, 0x5a])));
        // Head tracking: both processes face the other.
        let on_this = w.calls_on(this_process.addr());
        assert!(on_this.contains(&(0x644, vec![1])));
        assert!(on_this.contains(&(0x634, vec![p])));
        assert!(on_this.contains(&(0x288, vec![t, 2])));
        assert_eq!(
            w.calls_on(partner_process.addr()),
            vec![(0x644, vec![1]), (0x634, vec![t]), (0x288, vec![p, 2])]
        );
        assert!(w.calls_on(t).contains(&(0x2f4, vec![package, 1, 1])));
        assert!(w.calls_on(p).contains(&(0x2f4, vec![package, 1, 1])));
        assert!(log.contains(&(0x0057_bd60, vec![t, p])));
        assert!(log.contains(&(0x0057_bd60, vec![p, t])));
        let _ = conversation;
    }

    #[test]
    fn process_kind_four_and_the_second_option_change_the_calls() {
        let (w, result, log, this, _, this_process, _) =
            run_conversation([false, false, true, false, false], 4);
        assert!(result);
        let package = log
            .iter()
            .find(|(address, _)| *address == DIALOGUE_PACKAGE_CONSTRUCTOR)
            .unwrap()
            .1[0];
        assert!(log.contains(&(0x0040_3550, vec![package + 0x30, 0xc8])));
        // No head tracking; option_b gives slot 0x288 the value 1.
        let on_this = w.calls_on(this_process.addr());
        assert!(!on_this.iter().any(|(slot, _)| *slot == 0x644));
        assert!(on_this.contains(&(0x288, vec![this.addr(), 1])));
        assert!(log.contains(&(0x0067_a6b0, vec![package, 1])));
    }

    #[test]
    fn the_package_is_not_given_to_a_partner_when_option_c_without_option_d() {
        let (w, result, log, _, partner, _, partner_process) =
            run_conversation([false, false, false, true, false], 0);
        assert!(result);
        assert!(w.calls_on(partner_process.addr()).is_empty());
        assert!(!w
            .calls_on(partner.addr())
            .iter()
            .any(|(slot, _)| *slot == 0x2f4));
        let _ = log;
    }

    #[test]
    fn a_partner_that_refuses_the_package_fails_the_conversation() {
        let (mut w, this, _, partner, _) = talk_world();
        w.e.register(0x0083_b9a0, |_, _| int(1));
        w.e.mem.set_u32(partner.addr() + 0x204, 0x9999);
        w.e.call_log = Some(vec![]);
        assert!(!talk(&mut w, this, partner, [false; 5]));
        let log = w.e.call_log.take().unwrap();
        // This actor is told to give up (slot 0x288 without arguments), and both
        // locations are destroyed.
        assert!(w.calls_on(this.addr()).contains(&(0x288, vec![])));
        assert_eq!(
            log.iter()
                .filter(|(address, _)| *address == 0x0067_f110)
                .count(),
            2
        );
    }

    #[test]
    fn the_player_as_partner_gets_no_conversation_object() {
        let (mut w, this, _, _, _) = talk_world();
        let (player, player_process) = w.actor();
        w.e.set_global(PLAYER_CHARACTER, player.addr());
        w.e.call_log = Some(vec![]);
        assert!(talk(&mut w, this, player, [false; 5]));
        let log = w.e.call_log.take().unwrap();
        assert!(!log
            .iter()
            .any(|(address, _)| *address == CONVERSATION_CONSTRUCTOR));
        assert!(w.calls_on(player_process.addr()).is_empty());
    }

    #[test]
    fn dialogue_package_byte_setters() {
        let (mut e, _) = engine();
        let package = e.mem.alloc(0x100);
        e.call(0x008b_2860, &args![Ptr::<()>::new(package), 1u8]);
        e.call(0x008b_2880, &args![Ptr::<()>::new(package), 2u8]);
        e.call(0x008b_28a0, &args![Ptr::<()>::new(package), 3u8]);
        assert_eq!(
            [
                e.mem.u8(package + 0xbe),
                e.mem.u8(package + 0xbf),
                e.mem.u8(package + 0xc0)
            ],
            [1, 2, 3]
        );
    }

    #[test]
    fn the_player_animation_bonus_of_008b2b60() {
        let (mut e, _) = engine();
        e.register(PLAYER_GET_ANIMATION, |_, _| int(0xa11));
        let value = Rc::new(Cell::new(1.0));
        let seen = value.clone();
        e.register_double(0x0050_8070, move |_, _| float(seen.get()));
        let actor = e.mem.alloc(0x200);
        let call = |e: &mut Engine, animation: u32, actor: u32| {
            e.call(0x008b_2b60, &args![0u32, animation, 10u32, actor])
                .i32()
        };
        // The player's own animation with no actor, or with bit 0x40 clear.
        assert_eq!(call(&mut e, 0xa11, 0), 13);
        assert_eq!(call(&mut e, 0xa11, actor), 13);
        e.mem.set_u8(actor + 0x100, 0x40);
        assert_eq!(call(&mut e, 0xa11, actor), 10);
        // Another animation: bit 0x100 of the word at +0x12c must be clear.
        assert_eq!(call(&mut e, 0xb22, 0), 13);
        assert_eq!(call(&mut e, 0xb22, actor), 13);
        e.mem.set_u32(actor + 0x12c, 0x100);
        assert_eq!(call(&mut e, 0xb22, actor), 10);
        // A value that is not positive returns the base at once.
        e.mem.set_u32(actor + 0x12c, 0);
        value.set(0.0);
        assert_eq!(call(&mut e, 0xb22, actor), 10);
        value.set(-2.0);
        assert_eq!(call(&mut e, 0xa11, actor), 10);
    }

    #[test]
    fn flag_getters_008b2bf0_and_008b2c10() {
        let (mut e, _) = engine();
        let object = e.mem.alloc(0x200);
        assert!(e.call(0x008b_2bf0, &args![Ptr::<()>::new(object)]).bool());
        assert!(e.call(0x008b_2c10, &args![Ptr::<()>::new(object)]).bool());
        e.mem.set_u8(object + 0x100, 0x40);
        e.mem.set_u32(object + 0x12c, 0x100);
        assert!(!e.call(0x008b_2bf0, &args![Ptr::<()>::new(object)]).bool());
        assert!(!e.call(0x008b_2c10, &args![Ptr::<()>::new(object)]).bool());
        e.mem.set_u8(object + 0x100, 0xbf);
        e.mem.set_u32(object + 0x12c, 0xfffffeff);
        assert!(e.call(0x008b_2bf0, &args![Ptr::<()>::new(object)]).bool());
        assert!(e.call(0x008b_2c10, &args![Ptr::<()>::new(object)]).bool());
    }

    /// `fn_008b28c0` doubles: `GetType(x)` is `x + 0x3b`, `GetAnimGroup`
    /// answers 5, `PickBestAnimation(best)` answers `best`, `PlayGroup`
    /// answers 99; `5f2540` (attack) answers `attack`, aim false.
    fn play_world(attack: bool, iron_sights: bool) -> (World, Ptr<Actor>, Ptr) {
        let mut w = world();
        let (this, _) = w.actor();
        let animation = w.e.mem.alloc(0x40);
        w.e.register_double(0x005f_2540, move |_, _| int(attack as u32));
        w.e.register(0x005f_2630, |_, _| int(0));
        w.e.register(0x005f_2750, |_, _| int(1));
        w.e.register(0x005f_2440, |_, a| int(a[0] + 0x3b));
        w.e.register(0x0089_7910, |_, _| int(5));
        w.e.register(0x0049_5740, |_, a| int(a[1]));
        w.e.register(0x0049_4740, |_, _| int(99));
        w.e.register(0x0049_6080, |_, _| Ret::default());
        w.e.register_double(ACTOR_GET_IRON_SIGHTS, move |_, _| int(iron_sights as u32));
        w.e.register(0x0044_ddc0, |_, _| int(0));
        w.e.register(PLAYER_GET_ANIMATION, |_, _| int(0xa11));
        (w, this, Ptr::new(animation))
    }

    #[test]
    fn a_plain_group_is_played_and_reported() {
        let (mut w, this, animation) = play_world(false, false);
        let process = w.e.get(this, Actor::pCurrentProcess);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008b_28c0, &args![this, 0x40u32, animation]);
        let log = w.e.call_log.take().unwrap();
        assert!(log.contains(&(0x0089_7910, vec![this.addr(), 0x40, 0, 0, animation.addr()])));
        assert!(log.contains(&(
            0x0049_4740,
            vec![animation.addr(), 5, 1, u32::MAX, u32::MAX]
        )));
        assert_eq!(
            w.calls_on(process.addr()),
            vec![
                (0x148, vec![]),
                (0x424, vec![0, 99]),
                (0x428, vec![this.addr()])
            ]
        );
    }

    #[test]
    fn a_group_of_another_type_is_cleared_instead() {
        let (mut w, this, animation) = play_world(false, false);
        let process = w.e.get(this, Actor::pCurrentProcess);
        w.e.call_log = Some(vec![]);
        // GetType(5) = 0x40 is not 0x30.
        w.e.call(0x008b_28c0, &args![this, 0x30u32, animation]);
        let log = w.e.call_log.take().unwrap();
        assert!(log.contains(&(0x0049_6080, vec![animation.addr(), 4, 0.0f32.to_bits()])));
        assert!(!log.iter().any(|(address, _)| *address == 0x0049_4740));
        assert!(w.calls_on(process.addr()).contains(&(0x424, vec![0, 0])));
        // Group 0xff is never played.
        w.e.call_log = Some(vec![]);
        w.e.call(0x008b_28c0, &args![this, 0xffu32, animation]);
        let log = w.e.call_log.take().unwrap();
        assert!(!log.iter().any(|(address, _)| *address == 0x0089_7910));
    }

    #[test]
    fn attack_groups_try_three_followers_from_the_best() {
        let (mut w, this, animation) = play_world(true, false);
        w.e.call_log = Some(vec![]);
        // GetType(5) = 0x40 = the group; the followers 6 and 7 give 0x41 and 0x42.
        w.e.call(0x008b_28c0, &args![this, 0x40u32, animation]);
        let log = w.e.call_log.take().unwrap();
        let played: Vec<u32> = log
            .iter()
            .filter(|(address, _)| *address == 0x0049_4740)
            .map(|(_, a)| a[1])
            .collect();
        assert_eq!(played, vec![5, 6, 7]);
        assert!(log.contains(&(0x0049_5740, vec![animation.addr(), 6, 0])));
        assert!(log.contains(&(0x0049_5740, vec![animation.addr(), 7, 0])));
        let process = w.e.get(this, Actor::pCurrentProcess);
        let reported: Vec<_> = w
            .calls_on(process.addr())
            .into_iter()
            .filter(|(slot, _)| *slot == 0x424)
            .collect();
        assert_eq!(
            reported,
            vec![
                (0x424, vec![0, 99]),
                (0x424, vec![1, 99]),
                (0x424, vec![2, 99])
            ]
        );
    }

    #[test]
    fn the_players_own_animation_is_not_reported_to_the_process() {
        let (mut w, this, _) = play_world(false, false);
        let process = w.e.get(this, Actor::pCurrentProcess);
        w.e.call(0x008b_28c0, &args![this, 0x40u32, Ptr::<()>::new(0xa11)]);
        assert_eq!(
            w.calls_on(process.addr()),
            vec![(0x148, vec![]), (0x428, vec![this.addr()])]
        );
    }

    #[test]
    fn iron_sights_shift_the_group_and_are_undone_for_the_first_action() {
        let (mut w, this, animation) = play_world(true, true);
        // The animation is the player's first-person one and the item is null:
        // 008b2b60 adds 3, and 005f2750 + GetType undo it again for group 0.
        let first_person = Ptr::<()>::new(0xa11);
        let _ = animation;
        w.e.call_log = Some(vec![]);
        w.e.call(0x008b_28c0, &args![this, 0x40u32, first_person]);
        let log = w.e.call_log.take().unwrap();
        // i = 0: GetAnimGroup is asked for 0x40 + 3.
        assert!(log.contains(&(0x0089_7910, vec![this.addr(), 0x43, 0, 0, 0xa11])));
        let played: Vec<u32> = log
            .iter()
            .filter(|(address, _)| *address == 0x0049_4740)
            .map(|(_, a)| a[1])
            .collect();
        assert_eq!(played, vec![5, 6, 7]);
    }

    /// The player's weapon node: node block `+4` holds its collision object.
    fn recoil_world() -> (World, Ptr<Actor>, u32, SharedLog) {
        let mut w = world();
        let (player, process) = w.actor();
        w.e.set_global(PLAYER_CHARACTER, player.addr());
        w.answer(process.addr(), 0x148, 0x7000);
        w.answer(process.addr(), 0x454, 1);
        w.e.set(player, Actor::fGunSkillActor, 2.0);
        w.e.register(0x0044_ddc0, |_, _| int(0x5000));
        w.e.register(0x0068_15c0, |_, _| Ret::default());
        w.e.register(ACTOR_GET_ANIMATION, |_, _| int(0xa1));
        w.e.register(0x0045_3700, |_, _| float(0.5));
        w.e.register(0x0044_6390, |_, _| int(2));
        w.e.map(0x0118_a000, 0x1000);
        w.e.mem.set_u32(0x0118_a838 + 8, 0xdead);
        w.e.register(0x008d_6970, |_, a| {
            assert_eq!((a[0], f32::from_bits(a[1])), (0xdead, 0.5));
            int(0x3000)
        });
        w.e.register(0x0043_fcd0, |_, _| int(0x4000));
        let node = w.e.mem.alloc(0x20);
        w.e.register_double(0x004a_ae30, move |_, a| {
            assert_eq!((a[0], a[1]), (0x4000, 0x3000));
            int(node)
        });
        w.e.register(0x0089_4900, |_, _| int(0));
        w.e.register(0x0040_3e20, |_, a| int(a[0] + 4));
        w.e.map(0x011c_e000, 0x1000);
        w.e.mem.set_f32(0x011c_ec38, 3.0);
        w.e.register(0x0068_38b0, |e, a| int(e.mem.u32(a[0] + 4)));
        w.e.register(0x0062_bc90, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        w.e.register(0x00aa_13e0, |e, a| int(e.mem.alloc(a[0])));
        w.e.register(0x00c8_ffd0, |_, a| int(a[0]));
        w.e.register(0x0065_3270, |_, a| {
            int(if a[0] == 0x0126_81cc { a[1] } else { 0 })
        });
        w.e.map(0x011a_9000, 0x1000);
        for word in 0..9 {
            w.e.mem.set_f32(0x011a_9448 + 4 * word, word as f32);
        }
        w.e.register(0x00a5_92c0, |e, a| {
            e.mem.set_f32(a[1], 0.1);
            e.mem.set_f32(a[2], 0.2);
            e.mem.set_f32(a[3], 0.3);
            Ret::default()
        });
        let euler = Rc::new(std::cell::RefCell::new(vec![]));
        let seen = euler.clone();
        w.e.register_double(0x00a5_9540, move |_, a| {
            seen.borrow_mut().push((0x00a5_9540, a.to_vec()));
            Ret::default()
        });
        w.e.register(0x0043_f8d0, |e, a| {
            for word in 0..9 {
                e.mem.set_f32(a[1] + 4 * word, 100.0 + word as f32);
            }
            int(a[1])
        });
        let seen = euler.clone();
        w.e.register_double(0x004f_0110, move |e, a| {
            let first = e.mem.f32(a[1]);
            seen.borrow_mut()
                .push((0x004f_0110, vec![a[0], first.to_bits()]));
            Ret::default()
        });
        (w, player, node, euler)
    }

    #[test]
    fn the_recoil_rotation_is_scaled_and_set_on_a_new_collision_object() {
        let (mut w, player, node, calls) = recoil_world();
        w.e.call(0x008b_2c30, &args![player]);
        let collision = w.e.mem.u32(node + 4);
        assert_ne!(collision, 0);
        let calls = calls.borrow();
        // scale = 2.0 (cached) * 3.0 (setting); angles 0.1, 0.2, 0.3.
        let (_, euler) = &calls[0];
        assert_eq!(f32::from_bits(euler[1]), 0.1f32 * 6.0);
        assert_eq!(f32::from_bits(euler[2]), 0.2f32 * 6.0);
        assert_eq!(f32::from_bits(euler[3]), 0.3f32 * 6.0);
        // The product (starting at 100.0) is what the collision object receives.
        assert_eq!(calls[1], (0x004f_0110, vec![collision, 100.0f32.to_bits()]));
    }

    #[test]
    fn an_existing_collision_object_without_blend_is_replaced_by_a_copy() {
        let (mut w, player, node, _) = recoil_world();
        let existing = w.e.mem.alloc(0x20);
        w.e.mem.set_u32(node + 4, existing);
        // The first cast (0x1267e64) finds a source, the second (0x12681cc)
        // says it is not a blend object yet; afterwards the copy is one.
        let copied = Rc::new(Cell::new(0));
        let seen = copied.clone();
        w.e.register_double(0x00c9_0090, move |_, a| {
            seen.set(a[0] + 1000);
            assert_eq!(a[1], 0x6000);
            Ret::default()
        });
        w.e.register(0x0065_3270, |_, a| {
            int(if a[0] == 0x0126_7e64 {
                0x6000
            } else if a[1] == 0x6000 {
                0
            } else {
                a[1]
            })
        });
        w.e.call(0x008b_2c30, &args![player]);
        let replaced = w.e.mem.u32(node + 4);
        assert_ne!(replaced, existing);
        assert_eq!(copied.get(), replaced + 1000);
    }

    #[test]
    fn the_recoil_needs_the_player_a_process_an_item_and_slot_0x454() {
        let (mut w, player, node, calls) = recoil_world();
        let (other, _) = w.actor();
        w.e.call(0x008b_2c30, &args![other]);
        w.answer(w.e.get(player, Actor::pCurrentProcess).addr(), 0x454, 0);
        w.e.call(0x008b_2c30, &args![player]);
        w.answer(w.e.get(player, Actor::pCurrentProcess).addr(), 0x454, 1);
        w.answer(w.e.get(player, Actor::pCurrentProcess).addr(), 0x148, 0);
        w.e.call(0x008b_2c30, &args![player]);
        w.e.set(player, Actor::pCurrentProcess, Ptr::NULL);
        w.e.call(0x008b_2c30, &args![player]);
        assert_eq!(w.e.mem.u32(node + 4), 0);
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn the_recoil_stops_without_a_node_name_or_node() {
        let (mut w, player, node, calls) = recoil_world();
        w.e.register(0x008d_6970, |_, _| int(0));
        w.e.call(0x008b_2c30, &args![player]);
        w.e.register(0x008d_6970, |_, _| int(0x3000));
        w.e.register(0x004a_ae30, |_, _| int(0));
        w.e.call(0x008b_2c30, &args![player]);
        assert_eq!(w.e.mem.u32(node + 4), 0);
        assert!(calls.borrow().is_empty());
    }

    /// An actor whose slot `0x1f4` returns the position `(1, 2, 3)`, with
    /// doubles for `IsAtPoint`.
    fn point_world(height: f32) -> (Engine, Ptr<Actor>, u32, VectorLog) {
        let (mut e, _) = engine();
        e.map(0x0101_d000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.map(0x0102_f000, 0x1000);
        e.set_global(0x0101_db88, 30.0f64);
        e.set_global(0x0101_e704, 128.0f32);
        e.set_global(0x0101_e340, 32.0f32);
        e.set_global(0x0102_f070, 32.0f64);
        let position = e.mem.alloc(12);
        for (i, v) in [100.0f32, 200.0, 50.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * i as u32, *v);
        }
        e.register_double(0x0f00_0090, move |_, _| int(position));
        let actor = actor_with_slots(&mut e, &[(0x1f4, 0x0f00_0090)]);
        e.register(0x0093_06d0, |_, _| int(0));
        e.register_double(0x0088_53a0, move |_, _| float(height as f64));
        // The vector from the point to the actor.
        e.register(0x0043_9ef0, |e, a| {
            for word in 0..3 {
                let value = e.mem.f32(a[0] + 4 * word) - e.mem.f32(a[2] + 4 * word);
                e.mem.set_f32(a[1] + 4 * word, value);
            }
            int(a[1])
        });
        let seen = Rc::new(std::cell::RefCell::new(vec![]));
        let log = seen.clone();
        e.register_double(0x004b_5470, move |e, a| {
            log.borrow_mut().push(vec![
                e.mem.f32(a[0]).to_bits(),
                e.mem.f32(a[0] + 4).to_bits(),
                e.mem.f32(a[0] + 8).to_bits(),
                a[1],
            ]);
            int(0)
        });
        let point = e.mem.alloc(12);
        (e, actor, point, seen)
    }

    #[test]
    fn a_point_within_the_height_window_ignores_the_vertical_distance() {
        let (mut e, actor, point, seen) = point_world(0.0);
        for (i, v) in [90.0f32, 195.0, 100.0].iter().enumerate() {
            e.mem.set_f32(point + 4 * i as u32, *v);
        }
        // Height 0 means 128: the window is 50 - 30 .. 50 + 128 + 30.
        assert!(e
            .call(0x008b_2f90, &args![actor, point, 5.0f32, false, true])
            .bool());
        let vector = seen.borrow()[0].clone();
        assert_eq!(f32::from_bits(vector[0]), 10.0);
        assert_eq!(f32::from_bits(vector[1]), 5.0);
        assert_eq!(f32::from_bits(vector[2]), 0.0);
        assert_eq!(f32::from_bits(vector[3]), 5.0);
        // Outside the window the vertical part stays.
        e.mem.set_f32(point + 8, 300.0);
        e.call(0x008b_2f90, &args![actor, point, 5.0f32, false, true]);
        assert_eq!(f32::from_bits(seen.borrow()[1][2]), -250.0);
        // A given height narrows the window.
        e.register(0x0088_53a0, |_, _| float(10.0));
        e.mem.set_f32(point + 8, 95.0);
        e.call(0x008b_2f90, &args![actor, point, 5.0f32, false, true]);
        assert_eq!(f32::from_bits(seen.borrow()[2][2]), -45.0);
        e.mem.set_f32(point + 8, 80.0);
        e.call(0x008b_2f90, &args![actor, point, 5.0f32, false, true]);
        assert_eq!(f32::from_bits(seen.borrow()[3][2]), 0.0);
    }

    #[test]
    fn the_controller_can_drop_the_vertical_part_without_the_window() {
        let (mut e, actor, point, seen) = point_world(0.0);
        let controller = e.mem.alloc(0x500);
        e.register_double(0x0093_06d0, move |_, _| int(controller));
        e.register(0x005c_0880, |_, _| int(1));
        e.mem.set_f32(point + 8, 1000.0);
        e.call(0x008b_2f90, &args![actor, point, 5.0f32, false, false]);
        assert_eq!(f32::from_bits(seen.borrow()[0][2]), 0.0);
        // Forcing the window leaves it (1000 is far above the window).
        e.call(0x008b_2f90, &args![actor, point, 5.0f32, false, true]);
        assert_eq!(f32::from_bits(seen.borrow()[1][2]), -950.0);
    }

    #[test]
    fn the_actors_radius_is_added_to_the_distance() {
        let (mut e, actor, point, seen) = point_world(0.0);
        // No controller: plus 32.
        e.call(0x008b_2f90, &args![actor, point, 5.0f32, true, true]);
        assert_eq!(f32::from_bits(seen.borrow()[0][3]), 37.0);
        let controller = e.mem.alloc(0x500);
        e.register_double(0x0093_06d0, move |_, _| int(controller));
        e.register(0x005c_0880, |_, _| int(0));
        e.register(0x00c6_e280, |_, _| float(40.0));
        e.register(0x0045_87d0, |_, a| float(f32::from_bits(a[0]) as f64 + 1.0));
        e.register_double(0x008a_5170, move |_, a| {
            assert_eq!(a[0], controller + 0x410);
            int(0)
        });
        e.call(0x008b_2f90, &args![actor, point, 5.0f32, true, true]);
        assert_eq!(f32::from_bits(seen.borrow()[1][3]), 46.0);
        e.register(0x008a_5170, |_, _| int(1));
        e.call(0x008b_2f90, &args![actor, point, 5.0f32, true, true]);
        assert_eq!(f32::from_bits(seen.borrow()[2][3]), 87.0);
        // Below 32 the minimum applies.
        e.register(0x00c6_e280, |_, _| float(3.0));
        e.register(0x008a_5170, |_, _| int(0));
        e.call(0x008b_2f90, &args![actor, point, 5.0f32, true, true]);
        assert_eq!(f32::from_bits(seen.borrow()[3][3]), 37.0);
    }

    #[test]
    fn the_distance_comparison_decides_the_result() {
        let (mut e, actor, point, _) = point_world(0.0);
        for (answer, expected) in [(-1i32, true), (0, true), (1, false)] {
            e.register_double(0x004b_5470, move |_, _| int(answer as u32));
            assert_eq!(
                e.call(0x008b_2f90, &args![actor, point, 1.0f32, false, true])
                    .bool(),
                expected
            );
        }
    }

    fn mover_world() -> (World, Ptr<Actor>, Ptr, SharedLog) {
        let mut w = world();
        let (this, _) = w.actor();
        let mover = object_with_slots(&mut w.e, 0x100, &[(0, 0x0f30_0000), (0x14, 0x0f20_0014)]);
        w.e.set(this, Actor::pActorMover, mover);
        w.e.register(0x0f30_0000, |_, _| Ret::default());
        let seen = Rc::new(std::cell::RefCell::new(vec![]));
        let log = seen.clone();
        // Every ActorMover function this part calls records its arguments.
        for address in [
            0x009d_ba30u32,
            0x009d_ba50,
            0x009d_bb10,
            0x009d_bb60,
            0x009d_bc30,
            0x009d_c950,
            0x006d_61e0,
        ] {
            let log = log.clone();
            w.e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        (w, this, mover, seen)
    }

    #[test]
    fn the_actor_mover_is_created_destroyed_and_fed() {
        let (mut w, this, mover, log) = mover_world();
        // Created: the new mover replaces the old pointer.
        w.e.register(OPERATOR_NEW, |e, a| {
            assert_eq!(a[0], 0x88);
            int(e.mem.alloc(a[0]))
        });
        w.e.register(0x009d_ad00, |_, a| int(a[0]));
        w.e.call(0x008b_30f0, &args![this]);
        let created = w.e.get(this, Actor::pActorMover);
        assert_ne!(created, mover);
        assert!(!created.is_null());
        // A failed allocation stores null.
        w.e.register(OPERATOR_NEW, |_, _| int(0));
        w.e.call(0x008b_30f0, &args![this]);
        assert!(w.e.get(this, Actor::pActorMover).is_null());
        // Value passed on.
        w.e.set(this, Actor::pActorMover, mover);
        w.e.call(0x008b_3200, &args![this, 0x1234u32]);
        assert_eq!(log.borrow()[0], (0x009d_c950, vec![mover.addr(), 0x1234]));
        w.e.set(this, Actor::pActorMover, Ptr::NULL);
        w.e.call(0x008b_3200, &args![this, 1u32]);
        assert_eq!(log.borrow().len(), 1);
    }

    #[test]
    fn destroying_the_actor_mover_clears_inventory_references_first() {
        let (mut w, this, mover, _) = mover_world();
        let destroyed = Rc::new(Cell::new(0));
        let seen = destroyed.clone();
        w.e.register_double(0x0f30_0000, move |_, a| {
            assert_eq!(a[1], 1);
            seen.set(a[0]);
            Ret::default()
        });
        w.e.register(0x005d_43c0, |_, a| int(a[0] + 0x44));
        w.e.register(0x0041_8520, |_, a| int(a[0] + 1));
        w.e.register(0x0047_d0b0, |_, a| {
            assert!(a.is_empty());
            int(0x0e10_0000)
        });
        let cleared = Rc::new(Cell::new(0));
        let seen = cleared.clone();
        w.e.register_double(0x006e_bd10, move |_, a| {
            assert_eq!(a[0], 0x0e10_0000);
            seen.set(a[1]);
            Ret::default()
        });
        w.e.call(0x008b_3180, &args![this]);
        assert_eq!(cleared.get(), this.addr() + 0x44 + 1);
        assert_eq!(destroyed.get(), mover.addr());
        assert!(w.e.get(this, Actor::pActorMover).is_null());
        // Without a mover nothing is destroyed.
        destroyed.set(0);
        w.e.call(0x008b_3180, &args![this]);
        assert_eq!(destroyed.get(), 0);
        // Without extra data the path manager is left alone.
        w.e.register(0x005d_43c0, |_, _| int(0));
        w.e.register(0x006e_bd10, |_, _| panic!("no extra data"));
        w.e.call(0x008b_3180, &args![this]);
    }

    #[test]
    fn pathfinding_goals_go_to_the_mover_with_the_default_from_the_process() {
        let (mut w, this, mover, log) = mover_world();
        let (t, m) = (this.addr(), mover.addr());
        let (_, process) = {
            let p = w.e.get(this, Actor::pCurrentProcess);
            ((), p)
        };
        w.answer(process.addr(), 0x254, 0x5555);
        w.e.register(0x0069_e390, |_, a| int((a[0] == 0x77) as u32));
        // A goal that fails 0069e390 first gets the process value.
        w.e.call(0x008b_3630, &args![this, 0x66u32]);
        w.e.call(0x008b_3630, &args![this, 0x77u32]);
        w.e.call(0x008b_3630, &args![this, 0u32]);
        w.e.call(0x008b_3690, &args![this, 1u32, 2u32, 3u32, 4.5f32, 0u32]);
        w.e.call(0x008b_3690, &args![this, 1u32, 2u32, 3u32, 4.5f32, 9u32]);
        w.e.call(0x008b_36f0, &args![this, 1u32, 2.5f32, 0u32]);
        w.e.call(
            0x008b_3750,
            &args![this, 1u32, 2u32, 3u32, 4.5f32, 5.5f32, 0u32],
        );
        w.e.call(0x008b_37c0, &args![this, 1u32, 2.5f32, 8u32]);
        let log = log.borrow();
        assert_eq!(
            *log,
            vec![
                (0x006d_61e0, vec![0x66, 0x5555]),
                (0x009d_ba30, vec![m, 0x66]),
                (0x009d_ba30, vec![m, 0x77]),
                (0x009d_ba30, vec![m, 0]),
                (0x009d_ba50, vec![m, 1, 2, 3, 4.5f32.to_bits(), 0x5555]),
                (0x009d_ba50, vec![m, 1, 2, 3, 4.5f32.to_bits(), 9]),
                (0x009d_bb10, vec![m, 1, 2.5f32.to_bits(), 0x5555]),
                (
                    0x009d_bb60,
                    vec![m, 1, 2, 3, 4.5f32.to_bits(), 5.5f32.to_bits(), 0x5555]
                ),
                (0x009d_bc30, vec![m, 1, 2.5f32.to_bits(), 8]),
            ]
        );
        let _ = t;
    }

    /// An actor and everything `fn_008b3230` calls; slot answers in
    /// the `World`, knobs through cells.
    struct FrameKnobs {
        playing: Cell<bool>,
        valid: Cell<bool>,
        play_flag: Cell<bool>,
    }

    fn frame_world() -> (World, Ptr<Actor>, Ptr, Rc<FrameKnobs>) {
        let mut w = world();
        let (this, _) = w.actor();
        let mover = object_with_slots(&mut w.e, 0x100, &[(0x14, 0x0f20_0014)]);
        w.e.set(this, Actor::pActorMover, mover);
        let magic = w.e.mem.alloc(0x800);
        w.e.mem.set_u32(magic + 8, 0x0f20_0008);
        w.e.mem.set_u32(this.addr() + 0xa4, magic);
        w.e.map(0x011f_6000, 0x1000);
        w.e.mem.set_f32(0x011f_63a0, 0.25);
        for address in [0x0040_4eb0u32, 0x0040_4ee0] {
            w.e.register(address, |_, _| Ret::default());
        }
        w.e.register(0x0093_1850, |_, _| int(0));
        w.e.register(0x0044_ddc0, |e, a| int(e.mem.u32(a[0] + 8)));
        w.e.register(0x0084_d030, |e, a| float(e.mem.f32(a[0] + 0xc) as f64));
        w.e.register(0x009c_8d60, |_, _| float(2.0));
        w.e.register(0x004e_3d00, |_, _| float(3.0));
        let knobs = Rc::new(FrameKnobs {
            playing: Cell::new(false),
            valid: Cell::new(false),
            play_flag: Cell::new(false),
        });
        (w, this, mover, knobs)
    }

    fn mover_calls(w: &World, mover: Ptr) -> Vec<(u32, Vec<u32>)> {
        w.calls_on(mover.addr())
    }

    #[test]
    fn the_player_is_not_updated() {
        let (mut w, _, _, _) = frame_world();
        let (player, _) = w.actor();
        w.e.set_global(PLAYER_CHARACTER, player.addr());
        w.e.call_log = Some(vec![]);
        w.e.call(0x008b_3230, &args![player, 0.5f32]);
        let log = w.e.call_log.take().unwrap();
        let addresses: Vec<u32> = log.iter().map(|(address, _)| *address).collect();
        assert_eq!(addresses, vec![0x008b_3230, 0x0040_4eb0, 0x0040_4ee0]);
        assert_eq!(log[1].1[1..], [0x2d, 1, 0x0108_4918, 0x5d06]);
    }

    #[test]
    fn the_elapsed_time_goes_to_the_mover() {
        let (mut w, this, mover, _) = frame_world();
        w.answer(this.addr(), 0x21c, 0);
        w.e.call(0x008b_3230, &args![this, 0.5f32]);
        assert_eq!(mover_calls(&w, mover), vec![(0x14, vec![0.5f32.to_bits()])]);
        // Slot 8 of the magic target answering 0x33 > 0 with an animation scales it.
        w.log.borrow_mut().clear();
        w.answer(this.addr() + 0xa4, 8, 1);
        w.answer(this.addr(), 0x1e4, 0xa11);
        w.e.call(0x008b_3230, &args![this, 0.5f32]);
        assert_eq!(mover_calls(&w, mover), vec![(0x14, vec![1.5f32.to_bits()])]);
    }

    #[test]
    fn a_zero_elapsed_time_is_replaced_by_the_setting() {
        let (mut w, this, mover, _) = frame_world();
        w.e.call(0x008b_3230, &args![this, 0.0f32]);
        assert_eq!(
            mover_calls(&w, mover),
            vec![(0x14, vec![0.25f32.to_bits()])]
        );
        // In VATS mode (4) the VATS target gets the update multiplier.
        w.log.borrow_mut().clear();
        w.e.mem.set_u32(0x011f_2250 + 8, 4);
        w.e.mem.set_u32(0x011f_21cc, this.addr());
        w.e.call(0x008b_3230, &args![this, 0.0f32]);
        assert_eq!(mover_calls(&w, mover), vec![(0x14, vec![0.5f32.to_bits()])]);
        // A nonzero process type keeps the zero.
        w.log.borrow_mut().clear();
        w.e.register(0x0093_1850, |_, _| int(1));
        w.e.call(0x008b_3230, &args![this, 0.0f32]);
        assert_eq!(mover_calls(&w, mover), vec![(0x14, vec![0])]);
    }

    /// Doubles for the creature sound part; `playing`/`valid` are read from
    /// the knobs, and every handle call is logged by address.
    fn sound_world() -> (World, Ptr<Actor>, Rc<FrameKnobs>) {
        let (mut w, this, _, knobs) = frame_world();
        w.answer(this.addr(), 0x21c, 1);
        w.answer(this.addr(), 0x1d0, 0x1111);
        w.e.register(0x007a_f430, |_, _| int(0x2222));
        w.e.register(0x0059_f3a0, |_, _| int(6));
        w.e.register(0x005f_92d0, |_, a| {
            assert_eq!((a[0], a[1]), (0x2222, 10));
            int(0x3333)
        });
        w.e.register(0x0049_38e0, |_, _| int(1));
        w.e.register(0x008a_3b30, |_, _| int(0));
        w.e.register(0x0041_a250, |_, _| Ret::default());
        w.e.register(0x005d_43c0, |_, a| int(a[0] + 0x44));
        w.e.register(0x0041_8980, |_, _| Ret::default());
        w.e.register(0x0041_a280, |_, _| Ret::default());
        let k = knobs.clone();
        w.e.register_double(0x00ad_8930, move |_, _| int(k.playing.get() as u32));
        let k = knobs.clone();
        w.e.register_double(0x00ad_8ce0, move |_, _| int(k.valid.get() as u32));
        w.e.register(0x004e_75d0, |_, a| int(a[1]));
        w.e.register(0x005e_39b0, |_, _| int(0x10));
        w.e.register(0x0051_1840, |_, _| int(0x4444));
        w.e.register(0x0045_3a70, |_, _| int(0x5555));
        w.e.register(0x00ad_7480, |_, a| int(a[1]));
        for address in [
            0x0041_8900u32,
            0x0048_3710,
            0x004f_15a0,
            0x00ad_8f20,
            0x0068_a7d0,
            0x00ad_8830,
            0x00ad_88f0,
            0x00ad_8d10,
        ] {
            w.e.register(address, |_, _| Ret::default());
        }
        let _ = knobs.play_flag.get();
        (w, this, knobs)
    }

    fn handle_calls(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter()
            .map(|(address, _)| *address)
            .filter(|address| {
                [
                    0x0041_8900,
                    0x004f_15a0,
                    0x00ad_8f20,
                    0x0068_a7d0,
                    0x00ad_8830,
                    0x00ad_88f0,
                    0x00ad_8d10,
                    0x0041_a280,
                    0x0041_8980,
                ]
                .contains(address)
            })
            .collect()
    }

    #[test]
    fn a_creature_starts_its_sound_when_it_should_and_nothing_plays() {
        let (mut w, this, knobs) = sound_world();
        knobs.valid.set(false);
        w.e.call_log = Some(vec![]);
        let frame = next_frame(&mut w.e, 0x84);
        w.e.call(0x008b_3230, &args![this, 0.5f32]);
        // The 0x84-byte frame: the scope at +0x74, the handle at +0x54, the sound data at +0x30,
        // the temporary at +0x24.
        let early = w.e.call_log.clone().unwrap();
        assert_eq!(args_of(&early, 0x0040_4eb0)[0][0], frame + 0x74);
        assert_eq!(args_of(&early, 0x0041_a250), vec![vec![frame + 0x54]]);
        assert_eq!(args_of(&early, 0x004e_75d0)[0][1], frame + 0x30);
        assert_eq!(args_of(&early, 0x00ad_7480)[0][1], frame + 0x24);
        let log = w.e.call_log.take().unwrap();
        // Load the handle, create it, store it, follow, play, store it.
        assert_eq!(
            handle_calls(&log),
            vec![
                0x0041_8980,
                0x0041_8900,
                0x004f_15a0,
                0x0041_a280,
                0x00ad_8f20,
                0x0068_a7d0,
                0x00ad_8830,
                0x0041_a280
            ]
        );
        let flags = log
            .iter()
            .find(|(address, _)| *address == 0x00ad_7480)
            .unwrap();
        // (audio, handle out, file name, flags 0x10 | 2, sound)
        assert_eq!(flags.1[0], 0x5555);
        assert_eq!(flags.1[2..], [0x4444, 0x12, 0x3333]);
        let follow = log
            .iter()
            .find(|(address, _)| *address == 0x00ad_8f20)
            .unwrap();
        assert_eq!(follow.1[1], 0x1111);
    }

    #[test]
    fn a_valid_handle_is_only_played() {
        let (mut w, this, knobs) = sound_world();
        knobs.valid.set(true);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008b_3230, &args![this, 0.5f32]);
        let log = w.e.call_log.take().unwrap();
        assert_eq!(
            handle_calls(&log),
            vec![0x0041_8980, 0x0068_a7d0, 0x00ad_8830, 0x0041_a280]
        );
    }

    #[test]
    fn a_playing_sound_is_left_alone_while_it_should_play() {
        let (mut w, this, knobs) = sound_world();
        knobs.playing.set(true);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008b_3230, &args![this, 0.5f32]);
        let log = w.e.call_log.take().unwrap();
        assert_eq!(handle_calls(&log), vec![0x0041_8980]);
    }

    #[test]
    fn a_sound_that_should_not_play_is_stopped() {
        let (mut w, this, knobs) = sound_world();
        w.e.register(0x0049_38e0, |_, _| int(0));
        knobs.playing.set(true);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008b_3230, &args![this, 0.5f32]);
        let log = w.e.call_log.take().unwrap();
        assert_eq!(
            handle_calls(&log),
            vec![0x0041_8980, 0x00ad_88f0, 0x0041_a280]
        );
        // Slot 0x22c answering true while playing stops, releases and stores it.
        let (mut w, this, knobs) = sound_world();
        knobs.playing.set(true);
        w.answer(this.addr(), 0x22c, 1);
        w.e.call_log = Some(vec![]);
        w.e.call(0x008b_3230, &args![this, 0.5f32]);
        let log = w.e.call_log.take().unwrap();
        assert_eq!(
            handle_calls(&log),
            vec![
                0x0041_8980,
                0x00ad_88f0,
                0x0041_a280,
                0x00ad_88f0,
                0x00ad_8d10,
                0x0041_a280
            ]
        );
    }

    #[test]
    fn creatures_need_a_sound_a_follow_target_and_the_right_form_kind() {
        let (mut w, this, _) = sound_world();
        w.e.register(0x0059_f3a0, |_, _| int(5));
        w.e.register(0x0041_a250, |_, _| panic!("no handle for other kinds"));
        w.e.call(0x008b_3230, &args![this, 0.5f32]);
        w.e.register(0x0059_f3a0, |_, _| int(7));
        w.e.register(0x005f_92d0, |_, _| int(0));
        w.e.call(0x008b_3230, &args![this, 0.5f32]);
        w.e.register(0x005f_92d0, |_, _| int(0x3333));
        w.answer(this.addr(), 0x1d0, 0);
        w.e.call(0x008b_3230, &args![this, 0.5f32]);
        w.answer(this.addr(), 0x21c, 0);
        w.e.call(0x008b_3230, &args![this, 0.5f32]);
    }

    // ---- 008b3820 .. 008b4cd0 ------------------------------------------

    type Calls = Vec<(u32, Vec<u32>)>;
    type SlotCalls = Rc<std::cell::RefCell<Vec<(u32, u32, Vec<u32>)>>>;
    type SlotAnswers = Rc<std::cell::RefCell<std::collections::HashMap<(u32, u32), u32>>>;

    /// An engine whose objects share one vtable: each of `slots` points at a
    /// double that logs `(object, slot, arguments)` and answers what was set
    /// with [`Scene::answer`] (default 0).
    struct Scene {
        e: Engine,
        calls: SlotCalls,
        answers: SlotAnswers,
        vtable: u32,
    }

    fn scene(slots: &[u32]) -> Scene {
        let (mut e, _) = engine();
        let calls: SlotCalls = Default::default();
        let answers: SlotAnswers = Default::default();
        let vtable = e.mem.alloc(0x800);
        for &slot in slots {
            let (calls, answers) = (calls.clone(), answers.clone());
            let target = 0x0f30_0000 + slot;
            e.mem.set_u32(vtable + slot, target);
            e.register_double(target, move |_, a| {
                calls.borrow_mut().push((a[0], slot, a[1..].to_vec()));
                int(answers.borrow().get(&(a[0], slot)).copied().unwrap_or(0))
            });
        }
        Scene {
            e,
            calls,
            answers,
            vtable,
        }
    }

    impl Scene {
        fn object(&mut self, size: u32) -> u32 {
            let object = self.e.mem.alloc(size);
            self.e.mem.set_u32(object, self.vtable);
            object
        }

        fn actor(&mut self) -> Ptr<Actor> {
            let actor = self.object(0x1b4);
            self.e.mem.set_u32(actor + 0xa4, self.vtable);
            Ptr::new(actor)
        }

        fn answer(&self, object: u32, slot: u32, value: u32) {
            self.answers.borrow_mut().insert((object, slot), value);
        }

        fn slot_calls(&self) -> Vec<(u32, u32, Vec<u32>)> {
            self.calls.borrow().clone()
        }
    }

    /// Runs `address` on an actor whose mover is a plain block; the
    /// callee `target` answers 0x1234. Checks that `target` receives the
    /// mover and `words` and that its answer is returned.
    fn assert_mover_thunk(address: u32, target: u32, words: &[u32]) {
        let (mut e, _) = engine();
        let actor = e.new_object::<Actor>();
        let mover = e.mem.alloc(0xa8);
        e.mem.set_u32(actor.addr() + 0x190, mover);
        e.register(target, |_, _| int(0x1234));
        e.call_log = Some(vec![]);
        let mut all = vec![actor.addr()];
        all.extend_from_slice(words);
        let result = e.call(address, &all).u32();
        let log = e.call_log.take().unwrap();
        let mut expected = vec![mover];
        expected.extend_from_slice(words);
        assert_eq!(log[1], (target, expected));
        assert_eq!(log.len(), 2);
        assert_eq!(result, 0x1234);
    }

    /// Like [`assert_mover_thunk`] for a thunk of virtual slot `slot` of the mover.
    fn assert_mover_slot_thunk(address: u32, slot: u32, words: &[u32]) {
        let mut s = scene(&[slot]);
        let actor = s.actor();
        let mover = s.object(0xa8);
        s.e.mem.set_u32(actor.addr() + 0x190, mover);
        s.answer(mover, slot, 0x4321);
        let mut all = vec![actor.addr()];
        all.extend_from_slice(words);
        assert_eq!(s.e.call(address, &all).u32(), 0x4321);
        assert_eq!(s.slot_calls(), vec![(mover, slot, words.to_vec())]);
    }

    #[test]
    fn get_current_pathing_request_asks_the_mover() {
        assert_mover_thunk(0x008b_3820, 0x0068_38b0, &[]);
    }

    #[test]
    fn get_current_pathfinding_goal_passes_its_argument() {
        assert_mover_thunk(0x008b_3840, 0x009d_e2b0, &[0x77]);
    }

    #[test]
    fn thunk_008b3860_passes_its_argument() {
        assert_mover_thunk(0x008b_3860, 0x009d_e330, &[0x78]);
    }

    #[test]
    fn build_request_follows_up_only_in_combat() {
        for combat in [0u32, 1] {
            let (mut e, _) = engine();
            let actor = e.new_object::<Actor>();
            e.register(0x009d_bc90, |_, _| int(0));
            e.register_double(ACTOR_IS_IN_COMBAT, move |_, _| int(combat));
            e.register(0x008a_d8f0, |_, _| int(0));
            e.call_log = Some(vec![]);
            e.call(
                0x008b_3880,
                &args![actor, 0x11u32, 0x22u32, 0x33u32, 0x44u32, 1.5f32, 0x66u32],
            );
            let log = e.call_log.take().unwrap();
            assert_eq!(
                log[1],
                (
                    0x009d_bc90,
                    vec![actor.addr(), 0x11, 0x22, 0x33, 0x44, 1.5f32.to_bits(), 0x66]
                )
            );
            assert_eq!(log[2].0, ACTOR_IS_IN_COMBAT);
            if combat == 1 {
                assert_eq!(log[3], (0x008a_d8f0, vec![0x11, 0]));
            } else {
                assert_eq!(log.len(), 3);
            }
        }
    }

    #[test]
    fn build_request_ov2_picks_cell_or_world_space() {
        // (place, expected first place, expected second place)
        for (place, first, second) in [
            (0u32, 0xce11u32, 0x3du32),
            (0x1001, 0x1001, 0),
            (0x1002, 0, 0x1002),
            (0x1003, 0, 0),
        ] {
            let (mut e, _) = engine();
            let actor = e.new_object::<Actor>();
            e.register(0x009d_bc90, |_, _| int(0));
            e.register(ACTOR_IS_IN_COMBAT, |_, _| int(0));
            e.register(0x008d_6f30, |_, _| int(0xce11));
            e.register(0x0057_5d70, |_, _| int(0x3d));
            e.register(0x0040_1170, |_, a| {
                int(match a[0] {
                    0x1001 => 0x39,
                    0x1002 => 0x41,
                    _ => 0x40,
                })
            });
            e.call_log = Some(vec![]);
            e.call(
                0x008b_38d0,
                &args![actor, 0x11u32, 0x22u32, place, 2.5f32, 0x66u32],
            );
            let log = e.call_log.take().unwrap();
            let request = log.iter().find(|c| c.0 == 0x009d_bc90).unwrap();
            assert_eq!(
                request.1,
                vec![
                    actor.addr(),
                    0x11,
                    0x22,
                    first,
                    second,
                    2.5f32.to_bits(),
                    0x66
                ]
            );
        }
    }

    #[test]
    fn is_path_valid_asks_the_mover() {
        assert_mover_thunk(0x008b_3960, 0x009d_e3e0, &[]);
    }

    #[test]
    fn thunk_008b3980_asks_the_mover() {
        assert_mover_thunk(0x008b_3980, 0x009d_cab0, &[]);
    }

    #[test]
    fn thunk_008b39a0_passes_both_arguments() {
        assert_mover_thunk(0x008b_39a0, 0x009d_e110, &[5, 6]);
    }

    #[test]
    fn get_failed_path_destination_asks_the_mover() {
        assert_mover_thunk(0x008b_39d0, 0x009d_e460, &[]);
    }

    #[test]
    fn set_move_mode_uses_slot_4() {
        assert_mover_slot_thunk(0x008b_39f0, 0x4, &[3]);
    }

    #[test]
    fn force_move_mode_uses_slot_c() {
        assert_mover_slot_thunk(0x008b_3a20, 0xc, &[3]);
    }

    #[test]
    fn clear_forced_move_mode_uses_slot_10() {
        assert_mover_slot_thunk(0x008b_3a50, 0x10, &[]);
    }

    #[test]
    fn clear_move_mode_uses_slot_8() {
        assert_mover_slot_thunk(0x008b_3a80, 0x8, &[9]);
    }

    #[test]
    fn stop_moving_asks_the_mover() {
        assert_mover_thunk(0x008b_3ab0, 0x009d_d0a0, &[]);
    }

    #[test]
    fn force_stop_moving_asks_the_mover() {
        assert_mover_thunk(0x008b_3ad0, 0x009d_d0c0, &[]);
    }

    #[test]
    fn mover_stop_flag_is_cleared() {
        let (mut e, _) = engine();
        let actor = e.new_object::<Actor>();
        let mover = e.mem.alloc(0xa8);
        e.mem.set_u8(mover + 0x72, 1);
        e.mem.set_u32(actor.addr() + 0x190, mover);
        e.call(0x008b_3af0, &args![actor]);
        assert_eq!(e.mem.u8(mover + 0x72), 0);
        e.mem.set_u8(mover + 0x72, 1);
        e.call(0x008b_3b10, &args![Ptr::<()>::new(mover)]);
        assert_eq!(e.mem.u8(mover + 0x72), 0);
    }

    #[test]
    fn vector_comes_from_the_actor_or_the_mover() {
        // Zero or negative: the actor's own vector.
        for value in [0.0f32, -2.0] {
            let (mut e, _) = engine();
            let actor = e.new_object::<Actor>();
            let source = e.mem.alloc(12);
            for (i, bits) in [1.0f32, 2.0, 3.0].iter().enumerate() {
                e.mem.set_f32(source + 4 * i as u32, *bits);
            }
            e.register_double(0x0043_6aa0, move |_, _| int(source));
            e.register(0x009d_cc80, |_, _| panic!("mover is not asked"));
            let out = e.mem.alloc(12);
            let result = e.call(0x008b_3b30, &args![actor, out, value]).u32();
            assert_eq!(result, out);
            assert_eq!(
                [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)],
                [1.0, 2.0, 3.0]
            );
        }
        // Positive: the mover fills it.
        let (mut e, _) = engine();
        let actor = e.new_object::<Actor>();
        let mover = e.mem.alloc(0xa8);
        e.mem.set_u32(actor.addr() + 0x190, mover);
        e.register(0x0043_6aa0, |_, _| panic!("actor is not asked"));
        e.register(0x009d_cc80, |_, _| int(0));
        let out = e.mem.alloc(12);
        e.call_log = Some(vec![]);
        let result = e.call(0x008b_3b30, &args![actor, out, 0.5f32]).u32();
        assert_eq!(result, out);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x009d_cc80, vec![mover, out, 0.5f32.to_bits()]));
    }

    #[test]
    fn thunk_008b3b90_asks_the_mover() {
        assert_mover_thunk(0x008b_3b90, 0x0097_f2f0, &[]);
    }

    #[test]
    fn is_pathing_complete_asks_the_mover() {
        assert_mover_thunk(0x008b_3bb0, 0x009d_cd40, &[]);
    }

    #[test]
    fn is_pathing_asks_the_mover() {
        assert_mover_thunk(0x008b_3bd0, 0x009d_cdb0, &[]);
    }

    #[test]
    fn mover_flags_are_bytes() {
        let (mut e, _) = engine();
        let actor = e.new_object::<Actor>();
        let mover = e.mem.alloc(0xa8);
        e.mem.set_u32(actor.addr() + 0x190, mover);
        e.mem.set_u8(mover + 0x72, 0x11);
        e.mem.set_u8(mover + 0x73, 0x22);
        e.mem.set_u8(mover + 0x74, 0x33);
        let mover_ptr = Ptr::<()>::new(mover);
        assert_eq!(e.call(0x008b_3bf0, &args![actor]).u8(), 0x33);
        assert_eq!(e.call(0x008b_3c10, &args![mover_ptr]).u8(), 0x33);
        assert_eq!(e.call(0x008b_3c70, &args![actor]).u8(), 0x11);
        assert_eq!(e.call(0x008b_3c90, &args![mover_ptr]).u8(), 0x11);
        assert_eq!(e.call(0x008b_3d10, &args![mover_ptr]).u8(), 0x22);
    }

    #[test]
    fn is_rotating_asks_the_mover() {
        assert_mover_thunk(0x008b_3c30, 0x009d_db50, &[]);
    }

    #[test]
    fn thunk_008b3c50_asks_the_mover() {
        assert_mover_thunk(0x008b_3c50, 0x009d_cb30, &[]);
    }

    #[test]
    fn thunk_008b3cb0_passes_its_argument() {
        assert_mover_thunk(0x008b_3cb0, 0x009d_e3a0, &[0x44]);
    }

    #[test]
    fn set_look_at_target_passes_the_three_floats() {
        assert_mover_thunk(
            0x008b_3cd0,
            0x009d_e160,
            &[1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()],
        );
    }

    #[test]
    fn clear_look_at_target_asks_the_mover() {
        assert_mover_thunk(0x008b_3d30, 0x009d_e230, &[]);
    }

    #[test]
    fn thunk_008b3d50_asks_the_mover() {
        assert_mover_thunk(0x008b_3d50, 0x0046_0140, &[]);
    }

    /// Doubles for the vector helpers of `008b3d70`: subtraction, scaling
    /// and normalisation are done on the actual floats.
    fn ray_engine() -> (Engine, Ptr<Actor>) {
        let (mut e, actor) = engine();
        for page in [0x0102_3000, 0x0101_6000, 0x0102_2000] {
            e.map(page, 0x1000);
        }
        e.set_global::<f32>(0x0102_31a0, 512.0);
        e.set_global::<f32>(0x0101_6248, 0.5);
        e.set_global::<f32>(0x0102_226c, 7.0);
        e.register(0x0043_fcd0, |_, _| int(0xa01d));
        e.register(0x0043_9ef0, |e, a| {
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) - e.mem.f32(a[2] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            int(a[1])
        });
        e.register(0x0052_5340, |e, a| {
            let v: Vec<f32> = (0..3).map(|i| e.mem.f32(a[0] + 4 * i)).collect();
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            for i in 0..3 {
                e.mem.set_f32(a[0] + 4 * i, v[i as usize] / length);
            }
            Ret::default()
        });
        e.register(0x0045_bb20, |e, a| {
            let factor = f32::from_bits(a[2]);
            for i in 0..3 {
                let value = e.mem.f32(a[0] + 4 * i) * factor;
                e.mem.set_f32(a[1] + 4 * i, value);
            }
            int(a[1])
        });
        (e, actor)
    }

    #[test]
    fn ray_hit_releases_through_slot_c() {
        let (mut e, actor) = ray_engine();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0xc, 0x0f31_000c);
        let hit = e.mem.alloc(0x10);
        e.mem.set_u32(hit, vtable);
        e.register(0x0f31_000c, |_, _| int(0x55));
        e.register_double(0x008c_fbc0, move |e, a| {
            // world, start (3), direction (3), length pointer, 0
            assert_eq!(a[0], 0xa01d);
            assert_eq!(f32::from_bits(a[1]), -253.0);
            assert_eq!(f32::from_bits(a[4]), 1.0);
            assert_eq!(e.mem.f32(a[7]), 512.0);
            assert_eq!(a[8], 0);
            int(hit)
        });
        e.register(0x0062_40d0, |_, _| panic!("no second test after a hit"));
        let result = e.call(
            0x008b_3d70,
            &args![actor, 0f32, 0f32, 0f32, 3.0f32, 0f32, 0f32],
        );
        assert_eq!(result.u32(), 0x55);
    }

    #[test]
    fn ray_without_hit_tries_the_second_test() {
        let (mut e, actor) = ray_engine();
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(vtable + 0xc, 0x0f31_000c);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.register(0x0f31_000c, |_, _| int(0x66));
        e.register(0x008c_fbc0, |_, _| int(0));
        e.register(0x0062_40d0, |_, _| Ret::default());
        e.register(0x0068_15c0, |_, _| Ret::default());
        e.register(0x004a_3e00, |_, _| Ret::default());
        e.register(0x008a_50f0, |_, _| Ret::default());
        e.register(0x008a_50d0, |_, a| {
            assert_eq!(f32::from_bits(a[1]), 7.0);
            Ret::default()
        });
        e.register(0x00c8_06b0, |_, a| {
            assert_eq!(a[0], 0xa01d);
            int(0x1111)
        });
        e.register(0x004a_e750, |_, a| {
            assert_eq!(a[0], 0x1111);
            int(0x2222)
        });
        e.register(0x004b_5a20, |_, a| {
            assert_eq!(a[0], 0x2222);
            int(0x3333)
        });
        e.register_double(0x0044_ddc0, move |_, a| {
            assert_eq!(a[0], 0x3333);
            int(object)
        });
        e.call_log = Some(vec![]);
        let result = e.call(
            0x008b_3d70,
            &args![actor, 0f32, 0f32, 0f32, 3.0f32, 0f32, 0f32],
        );
        assert_eq!(result.u32(), 0x66);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            &order[order.len() - 10..],
            [
                0x0062_40d0,
                0x0068_15c0,
                0x004a_3e00,
                0x008a_50f0,
                0x008a_50d0,
                0x00c8_06b0,
                0x004a_e750,
                0x004b_5a20,
                0x0044_ddc0,
                0x0f31_000c
            ]
        );
    }

    #[test]
    fn ray_with_no_object_returns_zero() {
        let (mut e, actor) = ray_engine();
        e.register(0x008c_fbc0, |_, _| int(0));
        e.register(0x0062_40d0, |_, _| Ret::default());
        e.register(0x0068_15c0, |_, _| Ret::default());
        e.register(0x004a_3e00, |_, _| Ret::default());
        e.register(0x008a_50f0, |_, _| Ret::default());
        e.register(0x008a_50d0, |_, _| Ret::default());
        e.register(0x00c8_06b0, |_, _| int(0));
        let result = e.call(
            0x008b_3d70,
            &args![actor, 0f32, 0f32, 0f32, 3.0f32, 0f32, 0f32],
        );
        assert_eq!(result.u32(), 0);
    }

    #[test]
    fn body_part_index_walks_up_the_parents() {
        let mut s = scene(&[0x1d0, 0x1e8, 0x190, 0x6d0]);
        let actor = s.actor();
        let process = s.object(0x800);
        s.e.mem.set_u32(actor.addr() + 0x68, process);
        let stop = 0x5555;
        s.answer(actor.addr(), 0x1d0, stop);
        s.answer(actor.addr(), 0x1e8, 0x9);
        s.answer(process, 0x190, 0x200);
        // Parent chain: 100 -> 150 -> 0.
        s.e.register(0x0096_11e0, |_, a| {
            int(match a[0] {
                100 => 150,
                _ => 0,
            })
        });
        // Index 7 of the process holds node 150.
        s.e.register_double(0x0f30_06d0, move |_, a| {
            int(if a[1] == 7 { 150 } else { 0 })
        });
        assert_eq!(s.e.call(0x008b_3ef0, &args![actor, 100u32]).i32(), 7);
        // A parent equal to the process's answer for the actor's slot 0x1E8 gives 0xE.
        s.answer(process, 0x190, 150);
        assert_eq!(s.e.call(0x008b_3ef0, &args![actor, 100u32]).i32(), 0xe);
        // Reaching the actor's own object, an empty node, or no process: -1.
        assert_eq!(s.e.call(0x008b_3ef0, &args![actor, stop]).i32(), -1);
        assert_eq!(s.e.call(0x008b_3ef0, &args![actor, 0u32]).i32(), -1);
        s.e.mem.set_u32(actor.addr() + 0x68, 0);
        assert_eq!(s.e.call(0x008b_3ef0, &args![actor, 100u32]).i32(), -1);
    }

    #[test]
    fn transform_inverse_inverts_scale_and_translation() {
        let (mut e, _) = engine();
        let this = e.mem.alloc(0x34);
        let dest = e.mem.alloc(0x34);
        e.mem.set_f32(this + 0x30, 4.0);
        let negated = e.mem.alloc(12);
        let rotated = e.mem.alloc(12);
        let scaled = e.mem.alloc(12);
        for (i, v) in [7.0f32, 8.0, 9.0].iter().enumerate() {
            e.mem.set_f32(scaled + 4 * i as u32, *v);
        }
        e.register(0x004b_46a0, |_, _| Ret::default());
        e.register_double(0x004a_0bd0, move |_, a| {
            assert_eq!(a[0], this + 0x24);
            int(negated)
        });
        e.register_double(0x004b_4500, move |_, a| {
            assert_eq!((a[0], a[2]), (dest, negated));
            int(rotated)
        });
        e.register_double(0x004a_3760, move |_, a| {
            assert_eq!(f32::from_bits(a[1]), 0.25);
            assert_eq!(a[2], rotated);
            int(scaled)
        });
        e.call_log = Some(vec![]);
        e.call(
            0x008b_42e0,
            &args![Ptr::<()>::new(this), Ptr::<()>::new(dest)],
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (0x004b_46a0, vec![this, dest]));
        assert_eq!(e.mem.f32(dest + 0x30), 0.25);
        assert_eq!(
            [
                e.mem.f32(dest + 0x24),
                e.mem.f32(dest + 0x28),
                e.mem.f32(dest + 0x2c)
            ],
            [7.0, 8.0, 9.0]
        );
    }

    #[test]
    fn body_part_flags_and_byte() {
        let (mut e, _) = engine();
        let part = e.mem.alloc(0x80);
        let part_ptr = Ptr::<()>::new(part);
        assert!(!e.call(0x008b_4360, &args![part_ptr]).bool());
        assert!(!e.call(0x008b_4cd0, &args![part_ptr]).bool());
        e.mem.set_u8(part + 0x60, 0x08);
        assert!(e.call(0x008b_4360, &args![part_ptr]).bool());
        assert!(!e.call(0x008b_4cd0, &args![part_ptr]).bool());
        e.mem.set_u8(part + 0x60, 0x40);
        assert!(!e.call(0x008b_4360, &args![part_ptr]).bool());
        assert!(e.call(0x008b_4cd0, &args![part_ptr]).bool());
        e.mem.set_u8(part + 0x66, 0x5a);
        assert_eq!(e.call(0x008b_4380, &args![part_ptr]).u8(), 0x5a);
    }

    /// The world of the `008b3fe0` tests: every callee is a double.
    struct Spawn {
        s: Scene,
        actor: Ptr<Actor>,
        part: u32,
        reference: u32,
        impact: u32,
        clone: u32,
    }

    fn spawn_world() -> Spawn {
        let mut s = scene(&[0x180, 0x1d0, 0x14]);
        let actor = s.actor();
        let form = s.object(0x20);
        let owner = s.object(0x100);
        let path_owner = s.object(0x20);
        s.answer(form, 0x180, 0xb0d1);
        s.answer(actor.addr(), 0x1d0, owner);
        s.answer(path_owner, 0x14, 0x9a7);
        let part = s.e.mem.alloc(0x80);
        s.e.mem.set_u8(part + 0x60, 0x08);
        s.e.mem.set_u8(part + 0x66, 3);
        let reference = s.e.mem.alloc(0x100);
        let impact = s.e.mem.alloc(0x40);
        for i in 0..9 {
            s.e.mem.set_u32(reference + 0x68 + 4 * i, 0x100 + i);
        }
        for i in 0..3 {
            s.e.mem.set_u32(impact + 0x24 + 4 * i, 0x200 + i);
        }
        let e = &mut s.e;
        e.map(0x011c_3000, 0x1000);
        e.set_global::<u32>(MODEL_LOADER, 0xa0ad);
        e.register_double(0x0041_81e0, move |_, _| int(form));
        e.register(0x008d_6f30, |_, _| int(0xce11));
        e.register_double(0x005e_50f0, move |_, a| {
            assert_eq!(a[0], 0xb0d1);
            int(part)
        });
        e.register(0x005e_3fa0, |_, _| int(0x77));
        e.register(0x0046_1130, |_, a| int(a[0] + 0x68));
        e.register(0x009a_c9c0, |_, _| Ret::default());
        e.register(ACTOR_GET_PROCESS, |_, _| int(0xb1));
        e.register(0x0056_7400, |_, _| float(2.0));
        e.register(0x009b_0bd0, |_, _| float(3.0));
        e.register(0x004f_a6f0, |_, _| Ret::default());
        e.register_double(0x005d_8a70, move |_, _| int(path_owner));
        e.register(0x0048_cee0, |_, _| int(1));
        e.register(0x0044_7080, |_, _| int(0x30d1));
        e.register(0x004a_d050, |_, _| Ret::default());
        e.register(0x00a5_d2c0, |_, _| int(0xc10e));
        e.register(0x0047_6a80, |_, _| Ret::default());
        e.register(0x004b_46a0, |_, _| Ret::default());
        e.register(0x004a_0bd0, |_, _| int(0x1000));
        e.register(0x004b_4500, |_, _| int(0x2000));
        let scaled_buffer = e.mem.alloc(12);
        e.register_double(0x004a_3760, move |_, _| int(scaled_buffer));
        let composed = e.mem.alloc(0x34);
        for i in 0..13 {
            e.mem.set_u32(composed + 4 * i, 0x300 + i);
        }
        e.register_double(0x0062_c250, move |_, _| int(composed));
        e.register(0x0044_0460, |_, _| Ret::default());
        e.register(0x0043_fa80, |_, _| Ret::default());
        e.register(0x009b_88c0, |_, _| float(0.5));
        e.register(0x0044_0490, |_, _| Ret::default());
        e.register(0x004a_d270, |_, _| Ret::default());
        e.register(0x0045_a5e0, |_, _| Ret::default());
        Spawn {
            s,
            actor,
            part,
            reference,
            impact,
            clone: 0xc10e,
        }
    }

    #[test]
    fn spawn_part_model_runs_the_whole_chain() {
        let mut w = spawn_world();
        let e = &mut w.s.e;
        e.call_log = Some(vec![]);
        let result = e
            .call(
                0x008b_3fe0,
                &args![w.actor, 5u32, w.reference, w.impact, true],
            )
            .u32();
        assert_eq!(result, w.clone);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log.iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x008b_3fe0,
                0x0041_81e0,
                0x0f30_0180,
                0x008d_6f30,
                0x005e_50f0,
                0x005e_3fa0,
                0x0046_1130,
                0x008d_6f30,
                0x005e_3fa0,
                0x009a_c9c0,
                ACTOR_GET_PROCESS,
                0x0056_7400,
                0x009b_0bd0,
                0x008d_6f30,
                ACTOR_GET_PROCESS,
                0x004f_a6f0,
                0x005d_8a70,
                0x0048_cee0,
                0x005d_8a70,
                0x0f30_0014,
                0x0044_7080,
                0x004a_d050,
                0x00a5_d2c0,
                0x0047_6a80,
                0x0047_6a80,
                0x0f30_01d0,
                0x0046_1130,
                0x004b_46a0,
                0x004a_0bd0,
                0x004b_4500,
                0x004a_3760,
                0x0046_1130,
                0x0062_c250,
                0x0044_0460,
                0x0043_fa80,
                0x009b_88c0,
                0x0044_0490,
                0x004a_d270,
                0x005d_8a70,
                0x0f30_0014,
                0x0045_a5e0,
            ]
        );
        let explosion = &log[order.iter().position(|a| *a == 0x009a_c9c0).unwrap()].1;
        let mut expected = vec![0x77, 0, 0, 0xce11, 0x200, 0x201, 0x202];
        expected.extend((0..9).map(|i| 0x100 + i));
        assert_eq!(explosion, &expected);
        let placed = &log[order.iter().position(|a| *a == 0x004f_a6f0).unwrap()].1;
        assert_eq!(
            placed,
            &vec![0xb1, 0xce11, w.impact + 0x24, 3, 6.0f32.to_bits()]
        );
        let clone_scale = &log[order.iter().position(|a| *a == 0x0044_0490).unwrap()].1;
        assert_eq!(clone_scale, &vec![w.clone, 0.5f32.to_bits()]);
        let _ = w.part;
    }

    #[test]
    fn spawn_part_model_stops_early() {
        // No effects and no reference: the model is not touched.
        let mut w = spawn_world();
        let e = &mut w.s.e;
        e.register(0x0044_7080, |_, _| panic!("no model without a reference"));
        e.call_log = Some(vec![]);
        let result = e
            .call(0x008b_3fe0, &args![w.actor, 5u32, 0u32, w.impact, false])
            .u32();
        assert_eq!(result, 0);
        let order: Vec<u32> = e.call_log.take().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            vec![
                0x008b_3fe0,
                0x0041_81e0,
                0x0f30_0180,
                0x008d_6f30,
                0x005e_50f0,
                0x005d_8a70,
                0x0048_cee0
            ]
        );
        // A part without flag 0x08 stops before anything else.
        e.mem.set_u8(w.part + 0x60, 0);
        e.call_log = Some(vec![]);
        e.call(
            0x008b_3fe0,
            &args![w.actor, 5u32, w.reference, w.impact, true],
        );
        assert_eq!(e.call_log.take().unwrap().len(), 5);
        // No cell: nothing after the body part data.
        e.register(0x008d_6f30, |_, _| int(0));
        e.call_log = Some(vec![]);
        e.call(
            0x008b_3fe0,
            &args![w.actor, 5u32, w.reference, w.impact, true],
        );
        assert_eq!(e.call_log.take().unwrap().len(), 4);
    }

    /// The world of the `008b43a0` tests.
    struct Dismember {
        s: Scene,
        actor: Ptr<Actor>,
        hit: u32,
        parts: Rc<std::cell::RefCell<std::collections::HashMap<u32, u32>>>,
    }

    /// Settings (by the object `0043d4d0` is given) and their values.
    const DISMEMBER_SETTINGS: [(u32, u32); 7] = [
        (DISMEMBER_LIMIT_SETTING, 10),
        (DISMEMBER_SETTING_LIMB_CHANCE, 50),
        (DISMEMBER_SETTING_FIRST_PART, 40),
        (DISMEMBER_SETTING_OTHER_PART, 41),
        (DISMEMBER_SETTING_ROLL, 60),
        (DISMEMBER_SETTING_ROLLED_PART, 42),
        (DISMEMBER_BLOCK_SETTING, 0),
    ];

    fn dismember_world() -> Dismember {
        let mut s = scene(&[0x218, 0x180, 0x220, 8, 0x1d0]);
        let actor = s.actor();
        let form = s.object(0x20);
        s.answer(form, 0x180, 0xb0d1);
        let hit = s.e.mem.alloc(0x20);
        let e = &mut s.e;
        e.map(0x011e_0000, 0x1000);
        e.set_global::<u32>(DISMEMBER_COUNTER, 0);
        let mut pointers = std::collections::HashMap::new();
        for (setting, value) in DISMEMBER_SETTINGS {
            let cell = e.mem.alloc(4);
            e.mem.set_u32(cell, value);
            pointers.insert(setting, cell);
        }
        e.register_double(0x0043_d4d0, move |_, a| int(pointers[&a[0]]));
        e.register_double(0x0041_81e0, move |_, _| int(form));
        e.register(0x00ec_43fb, |_, _| int(0));
        e.register(0x005f_bf20, |_, _| int(0));
        e.register(0x0040_8d60, |_, a| int(a[0]));
        let parts: Rc<std::cell::RefCell<std::collections::HashMap<u32, u32>>> = Default::default();
        let by_key = parts.clone();
        e.register_double(0x005e_50f0, move |_, a| {
            int(by_key.borrow().get(&a[1]).copied().unwrap_or(0))
        });
        let by_value = parts.clone();
        e.register_double(0x005e_5130, move |_, a| {
            int(by_value
                .borrow()
                .get(&(0x1000 + a[1]))
                .copied()
                .unwrap_or(0))
        });
        // Part kind: the byte at +0x61 is the actor value, the dword at +0x70 its body part.
        e.register(0x005e_5190, |e, a| int(e.mem.u8(a[0] + 0x61) as u32));
        e.register(0x005e_5300, |e, a| int(e.mem.u32(a[0] + 0x70)));
        e.register(0x008b_4cf0, |e, a| int(e.mem.u8(a[0] + 0x62) as u32));
        e.register(0x0058_cba0, |_, _| int(0));
        e.register(0x0048_7f50, |_, _| int(250));
        e.register(0x0066_ec10, |_, a| {
            assert_eq!(a[0], 4);
            int(10 + a[1])
        });
        e.register(0x0040_7e00, |_, _| Ret::default());
        e.register(ACTOR_DISMEMBER, |_, _| Ret::default());
        Dismember {
            s,
            actor,
            hit,
            parts,
        }
    }

    impl Dismember {
        /// A body part with the given actor value byte, part number and 4cf0 byte.
        fn part(&mut self, value: u8, number: u32, severable: u8, flags: u8) -> u32 {
            let part = self.s.e.mem.alloc(0x80);
            self.s.e.mem.set_u8(part + 0x60, flags);
            self.s.e.mem.set_u8(part + 0x61, value);
            self.s.e.mem.set_u8(part + 0x62, severable);
            self.s.e.mem.set_u32(part + 0x70, number);
            part
        }

        fn run(&mut self) -> Calls {
            self.s.e.call_log = Some(vec![]);
            let actor = self.actor;
            let hit = self.hit;
            self.s
                .e
                .call(0x008b_43a0, &args![actor, hit, 0xaau32, 0xbbu32]);
            self.s.e.call_log.take().unwrap()
        }
    }

    fn dismember_calls(log: &Calls) -> Calls {
        log.iter()
            .filter(|c| c.0 == ACTOR_DISMEMBER || c.0 == 0x0040_7e00)
            .cloned()
            .collect()
    }

    #[test]
    fn dismember_stops_early_and_counts() {
        let mut w = dismember_world();
        // A creature that cannot be dismembered while the setting says so.
        w.s.answer(w.actor.addr(), 0x218, 1);
        let blocked = w.s.e.mem.alloc(4);
        w.s.e.mem.set_u8(blocked, 1);
        w.s.e.register_double(0x0040_8d60, move |_, _| int(blocked));
        let log = w.run();
        assert_eq!(log.last().unwrap().0, 0x0040_8d60);
        assert_eq!(w.s.e.global::<u32>(DISMEMBER_COUNTER), 0);
        // No body part data.
        let mut w = dismember_world();
        let form = w.s.object(0x20);
        w.s.e.register_double(0x0041_81e0, move |_, _| int(form));
        let log = w.run();
        assert_eq!(log.last().unwrap().0, 0x0f30_0180);
        // Without a hit.
        let mut w = dismember_world();
        w.hit = 0;
        let log = w.run();
        assert_eq!(log.last().unwrap().0, 0x0f30_0180);
        // The counter has reached the limit.
        let mut w = dismember_world();
        w.s.e.set_global::<u32>(DISMEMBER_COUNTER, 10);
        let log = w.run();
        assert!(dismember_calls(&log).is_empty());
        assert_eq!(w.s.e.global::<u32>(DISMEMBER_COUNTER), 10);
        // Otherwise it counts up.
        let mut w = dismember_world();
        w.run();
        assert_eq!(w.s.e.global::<u32>(DISMEMBER_COUNTER), 1);
    }

    #[test]
    fn dismember_without_candidates_dismembers_nothing_specific() {
        let mut w = dismember_world();
        let log = w.run();
        assert_eq!(
            dismember_calls(&log),
            vec![(
                ACTOR_DISMEMBER,
                vec![
                    w.actor.addr(),
                    w.hit,
                    u32::MAX,
                    u32::MAX,
                    0xaa,
                    0xbb,
                    0,
                    0,
                    0
                ]
            )]
        );
    }

    #[test]
    fn dismember_single_part_of_the_hit() {
        let mut w = dismember_world();
        let part = w.part(9, 0x33, 0, 0);
        w.parts.borrow_mut().insert(3, part);
        w.s.e.mem.set_u32(w.hit + 0x10, 3);
        let log = w.run();
        assert_eq!(
            dismember_calls(&log),
            vec![(
                ACTOR_DISMEMBER,
                vec![w.actor.addr(), w.hit, 9, 3, 0xaa, 0xbb, 0, 0, 0]
            )]
        );
        // The attacker's value already above zero: nothing is added.
        w.s.answer(w.actor.addr() + 0xa4, 8, 1);
        let log = w.run();
        assert_eq!(
            dismember_calls(&log)[0].1[2..4],
            [u32::MAX, u32::MAX],
            "no candidate, so the general call"
        );
    }

    #[test]
    fn dismember_with_a_strong_attacker_picks_the_hit_part_and_another() {
        let mut w = dismember_world();
        let attacker = w.s.object(0x100);
        w.s.answer(attacker + 0xa4, 8, 5);
        // Slot 8 is also the actor's value lookup; the attacker's and the
        // actor's answers are keyed by object.
        w.s.e.mem.set_u32(attacker + 0xa4, w.s.vtable);
        w.s.e.mem.set_u32(w.hit, attacker);
        // The hit's part: not severable; actor value 5.
        let hit_part = w.part(5, 0x51, 0, 0);
        w.parts.borrow_mut().insert(3, hit_part);
        w.s.e.mem.set_u32(w.hit + 0x10, 3);
        // Another part for the actor value 11 (the second index).
        let other = w.part(0, 0x52, 0, 0);
        w.parts.borrow_mut().insert(0x1000 + 11, other);
        let log = w.run();
        assert_eq!(
            dismember_calls(&log),
            vec![
                (
                    ACTOR_DISMEMBER,
                    vec![w.actor.addr(), w.hit, 5, 3, 0xaa, 0xbb, 100, 40, 0]
                ),
                (
                    ACTOR_DISMEMBER,
                    vec![w.actor.addr(), w.hit, 11, 0x52, 0xaa, 0xbb, 100, 41, 0]
                ),
            ]
        );
    }

    #[test]
    fn dismember_head_roll_sets_the_flag_around_the_call() {
        let mut w = dismember_world();
        let attacker = w.s.object(0x100);
        w.s.answer(attacker + 0xa4, 8, 5);
        let vtable = w.s.vtable;
        w.s.e.mem.set_u32(attacker + 0xa4, vtable);
        w.s.e.mem.set_u32(w.hit, attacker);
        // The hit's part is the head (value 0x1a) and severable (flags 0x08 | 0x40, byte >= 100).
        let head = w.part(0x1a, 0x61, 100, 0x48);
        w.parts.borrow_mut().insert(3, head);
        w.s.e.mem.set_u32(w.hit + 0x10, 3);
        let log = w.run();
        assert_eq!(
            dismember_calls(&log),
            vec![
                (0x0040_7e00, vec![w.hit, 0x40, 1]),
                (
                    ACTOR_DISMEMBER,
                    vec![w.actor.addr(), w.hit, 0x1a, 0x61, 0xaa, 0xbb, 100, 100, 1]
                ),
                (0x0040_7e00, vec![w.hit, 0x40, 0]),
            ]
        );
    }

    #[test]
    fn dismember_from_a_part_list() {
        let mut w = dismember_world();
        let owner = w.s.object(0x40);
        w.s.answer(owner, 0x220, 1);
        w.s.e.mem.set_u32(w.hit + 8, owner);
        let part = w.part(7, 0x71, 0, 0);
        w.parts.borrow_mut().insert(0x1234, part);
        let entry = w.s.e.mem.alloc(4);
        w.s.e.mem.set_u32(entry, 0x1234);
        let slot = w.s.e.mem.alloc(4);
        w.s.e.mem.set_u32(slot, entry);
        w.s.e.register(0x009b_1720, |_, _| int(0xa1));
        w.s.e.register(0x0082_56d0, |_, _| int(0));
        w.s.e.register_double(0x0068_15c0, move |_, _| int(slot));
        w.s.e.register(0x0072_6070, |_, _| int(0));
        let log = w.run();
        assert_eq!(
            dismember_calls(&log),
            vec![(
                ACTOR_DISMEMBER,
                vec![w.actor.addr(), w.hit, 7, 0x1234, 0xaa, 0xbb, 0, 0, 0]
            )]
        );
    }

    // ---- 008b4cf0 .. 008b6df0: dismemberment ----------------------------

    /// Registers doubles that answer 0 at every address in `addresses`.
    fn zeros(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// The arguments of every logged call to `address`.
    fn args_of(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|call| call.0 == address)
            .map(|call| call.1.clone())
            .collect()
    }

    /// Runs `f` with call logging on and returns its result and the log.
    fn logged<R>(e: &mut Engine, f: impl FnOnce(&mut Engine) -> R) -> (R, Vec<(u32, Vec<u32>)>) {
        e.call_log = Some(vec![]);
        let result = f(e);
        (result, e.call_log.take().unwrap())
    }

    #[test]
    fn body_part_chance_byte_of_008b4cf0() {
        let (mut e, _) = engine();
        let part = e.mem.alloc(0x100);
        e.mem.set_u8(part + 0x65, 77);
        assert_eq!(e.call(0x008b_4cf0, &args![part]).u8(), 77);
    }

    #[test]
    fn body_part_listed_flag_of_008b5190() {
        let (mut e, _) = engine();
        let part = e.mem.alloc(0x100);
        assert!(!e.call(0x008b_5190, &args![part]).bool());
        e.mem.set_u8(part + 0x60, 0x03);
        assert!(e.call(0x008b_5190, &args![part]).bool());
        e.mem.set_u8(part + 0x60, 0xfe);
        assert!(!e.call(0x008b_5190, &args![part]).bool());
    }

    /// The pattern `62c250` writes into the transform it composes: 13 words
    /// `0x1000 + n`.
    const COMPOSED: u32 = 0x1000;
    /// The part (body part index) the limb stage answers for.
    const STAGE_PART_INDEX: u32 = 4;

    /// A world for `fn_008b52a0` and its callers: an actor with 3D (a root
    /// node with one child), a process that has one limb node, a base form
    /// with body part data and one body part (index 4) whose actor value is
    /// 0x1a. Every callee outside the file is a double (default answer 0).
    struct LimbStage {
        s: Scene,
        actor: Ptr<Actor>,
        body_parts: u32,
        part: u32,
        process: u32,
        root: u32,
        limb: u32,
        clone: u32,
        group: u32,
        havok: u32,
        child: u32,
    }

    fn limb_stage() -> LimbStage {
        let mut s = scene(&[
            0x218, 0x1d0, 0x1e8, 0x9c, 0xdc, 0xb8, 0xd0, 0xc, 0x6cc, 0x6d0, 0x73c, 0x14, 0x58,
            0x180, 0x1a0, 0x18, 0x10, 0xe8, 0x468, 0xf0, 0x6c8, 0x28c, 0x1e4, 0x230, 0x22c, 0x3ac,
        ]);
        let actor = s.actor();
        let vtable = s.vtable;
        let form = s.object(0x40);
        s.e.mem.set_u32(form + 0x30, vtable);
        let body_parts = s.e.mem.alloc(0x40);
        s.answer(form, 0x180, body_parts);
        let process = s.object(0x40);
        let root = s.object(0x100);
        let limb = s.object(0x100);
        let clone = s.object(0x100);
        let group = s.object(0xac);
        let havok = s.object(0x40);
        let child = s.object(0x100);
        let part = s.e.mem.alloc(0x100);
        s.e.mem.set_u8(part + 0x61, 0x1a);
        s.e.set(actor, Actor::pCurrentProcess, Ptr::new(process));
        s.answer(process, 0x6d0, limb);
        s.answer(actor.addr(), 0x1d0, root);
        s.answer(clone, 0xc, 0xc10c);
        let bound = s.e.mem.alloc(0x10);
        for (word, value) in [1.0f32, 2.0, 3.0, 4.0].into_iter().enumerate() {
            s.e.mem.set_f32(bound + 4 * word as u32, value);
        }
        let factor = s.e.mem.alloc(8);
        s.e.mem.set_f32(factor, 2.0);
        let e = &mut s.e;
        for page in [
            0x0102_3000,
            0x0119_6000,
            0x011c_a000,
            0x011d_f000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global::<f64>(DEGREES_TO_RADIANS, f64::from_bits(0x3f91_df46_a000_0000));
        zeros(
            e,
            &[
                0x00ec_43fb,
                0x005f_bf20,
                0x0043_8170,
                0x0041_3f40,
                0x009a_3830,
                0x0043_81b0,
                0x00a5_b950,
                0x0043_d410,
                0x0055_9450,
                0x0047_6a80,
                0x00a5_9540,
                0x004a_d050,
                0x004a_d240,
                0x004a_de00,
                0x0096_11e0,
                0x0040_7840,
                0x0072_6070,
                0x0050_0940,
                0x004a_af30,
                0x0044_0460,
                0x0043_fa80,
                0x0044_0490,
                0x004f_b070,
                0x009a_c9c0,
                0x0056_7400,
                0x0064_47b0,
                0x004f_a6f0,
                0x004a_d270,
                0x0045_0f90,
                0x0068_38b0,
                0x0045_3470,
                0x00c6_9ee0,
                0x00c6_a200,
                0x004b_5260,
                0x006f_a820,
                0x00c8_ec60,
                0x00c6_bd00,
                0x00a5_9c60,
                0x00c6_a350,
                0x005e_3fc0,
                0x004f_d400,
                0x0048_cee0,
                0x004b_45b0,
                0x0068_9210,
                0x0067_33e0,
                0x0068_a8d0,
                0x0040_1000,
                0x004a_2c70,
                0x004a_1a50,
                0x0044_ddc0,
                0x009c_71c0,
                0x0044_edb0,
                0x0063_9b40,
                0x0058_cbc0,
                0x0058_ccf0,
                0x00a5_a040,
                0x0093_1ed0,
                0x004a_3a20,
                0x00ca_2ad0,
                0x0043_9ef0,
                0x004a_0c10,
                0x0045_bb20,
                0x0062_b8d0,
                0x0049_10d0,
                0x005e_4730,
                0x005e_55e0,
                0x005e_4970,
                0x005e_5750,
                0x005e_4890,
                0x004a_de40,
                0x0043_fad0,
                0x0055_85e0,
                0x0080_41a0,
                0x0082_5c00,
                0x0040_3d30,
                0x0055_d520,
                0x005b_5e40,
                0x005d_8a70,
                0x005e_3fa0,
                0x0058_e9d0,
                0x0057_2fc0,
                0x0057_3090,
                0x0087_f3d0,
                0x0058_cba0,
                0x0040_1170,
                0x0045_6610,
                0x008c_7aa0,
                0x0087_b440,
                0x0064_6e50,
                0x0043_b480,
                0x0043_b4a0,
                0x0042_e8e0,
                0x00c6_a270,
                0x0097_4290,
                0x0057_3050,
                0x0042_e8c0,
                0x005d_43c0,
                0x0045_64f0,
                0x00ec_a6d3,
                0x0043_b1b0,
                ACTOR_GET_ANIMATION,
            ],
        );
        zeros(e, &[0x004b_46a0, 0x004a_0bd0, 0x004b_4500]);
        let triple = e.mem.alloc(0x10);
        e.register_double(0x004a_3760, move |_, _| int(triple));
        e.register_double(0x0041_81e0, move |_, _| int(form));
        e.register(0x0040_8d60, |_, a| int(a[0]));
        e.register_double(0x0043_fcd0, move |_, _| int(root));
        e.register(0x008d_6f30, |_, _| int(0x5000));
        e.register_double(0x0045_43c0, move |_, _| int(havok));
        e.register(ACTOR_GET_PROCESS, |e, a| int(e.mem.u32(a[0] + 0x68)));
        e.register_double(0x005e_50f0, move |_, a| {
            int(if a[1] == STAGE_PART_INDEX { part } else { 0 })
        });
        e.register(0x005e_5190, |e, a| int(u32::from(e.mem.u8(a[0] + 0x61))));
        e.register_double(0x0043_b480, move |_, a| int(u32::from(a[0] == root)));
        e.register_double(0x0043_b4a0, move |_, a| {
            int(if a[0] == root { child } else { 0 })
        });
        e.register_double(0x00aa_13e0, move |_, _| int(group));
        e.register(0x00a5_ecb0, |_, a| int(a[0]));
        e.register(0x0046_1130, |_, a| int(a[0] + 0x10));
        e.register(0x0062_c250, |e, a| {
            for word in 0..13 {
                e.mem.set_u32(a[1] + 4 * word, COMPOSED + word);
            }
            int(a[1])
        });
        e.register(0x009f_8300, |e, a| {
            for (word, value) in [10.0f32, 20.0, 30.0].into_iter().enumerate() {
                e.mem.set_f32(a[1] + 4 * word as u32, value);
            }
            int(a[1])
        });
        e.register(0x0041_6870, |e, a| {
            for word in 0..3 {
                e.mem.set_u32(a[0] + 4 * word, a[1 + word as usize]);
            }
            Ret::default()
        });
        e.register_double(0x00a5_d2c0, move |_, _| int(clone));
        e.register(0x0089_1350, |_, _| int(7));
        e.register(0x0068_15c0, |_, a| int(a[0]));
        e.register_double(0x0043_d450, move |_, _| int(bound));
        e.register(0x0045_bb80, |_, a| int(a[0] + 0x8c));
        e.register_double(0x0040_3e20, move |_, _| int(factor));
        LimbStage {
            s,
            actor,
            body_parts,
            part,
            process,
            root,
            limb,
            clone,
            group,
            havok,
            child,
        }
    }

    impl LimbStage {
        fn dismember(&mut self, value: i32, part_index: u32, hit: u32) -> Vec<(u32, Vec<u32>)> {
            let actor = self.actor;
            logged(&mut self.s.e, |e| {
                e.call(
                    0x008b_4d10,
                    &args![actor, hit, value, part_index, 0xe1u32, 0xe2u32, 0xe3u32, 0xe4u32, 0u32],
                )
            })
            .1
        }

        fn slot_calls_on(&self, object: u32) -> Vec<(u32, Vec<u32>)> {
            self.s
                .slot_calls()
                .into_iter()
                .filter(|call| call.0 == object)
                .map(|call| (call.1, call.2))
                .collect()
        }
    }

    #[test]
    fn dismember_with_a_value_outside_the_limb_range_only_marks_the_part() {
        let mut w = limb_stage();
        let actor = w.actor.addr();
        let log = w.dismember(0x10, 4, 0);
        assert_eq!(
            args_of(&log, 0x0057_2fc0),
            vec![vec![actor, u32::MAX, 0xe1, 0xe2, 4, 0]]
        );
        assert!(args_of(&log, 0x00a5_a040).is_empty());
        // With a hit, the hit's part index (+0x10) is used instead.
        let hit = w.s.e.mem.alloc(0x40);
        w.s.e.mem.set_u32(hit + 0x10, 9);
        let log = w.dismember(0x18, 4, hit);
        assert_eq!(
            args_of(&log, 0x0057_2fc0),
            vec![vec![actor, u32::MAX, 0xe1, 0xe2, 9, 0]]
        );
        let log = w.dismember(0x20, 4, 0);
        assert_eq!(args_of(&log, 0x0057_2fc0).len(), 1);
    }

    #[test]
    fn dismember_stops_for_blocked_creatures_dismembered_parts_and_essential_actors() {
        let mut w = limb_stage();
        w.s.e.mem.set_u8(DISMEMBER_BLOCK_SETTING, 1);
        w.s.answer(w.actor.addr(), 0x218, 1);
        let log = w.dismember(0x1a, 4, 0);
        assert!(args_of(&log, 0x0057_2fc0).is_empty());
        assert!(args_of(&log, 0x0057_3090).is_empty());

        for stopper in [0x0057_3090, 0x0087_f3d0] {
            let mut w = limb_stage();
            w.s.e.register(stopper, |_, _| int(1));
            let log = w.dismember(0x1a, 4, 0);
            assert!(args_of(&log, 0x0064_6e50).is_empty(), "{stopper:08x}");
            assert!(args_of(&log, 0x00a5_a040).is_empty());
        }
        let mut w = limb_stage();
        w.s.answer(w.actor.addr(), 0x1a0, 1);
        let log = w.dismember(0x1a, 4, 0);
        assert!(args_of(&log, 0x0064_6e50).is_empty());
        assert!(args_of(&log, 0x00a5_a040).is_empty());
    }

    #[test]
    fn dismember_asks_the_outcome_routine_and_stops_when_it_sets_nothing() {
        let mut w = limb_stage();
        // Part flags 0x40 and 0x08; chance byte 100 makes it severable.
        w.s.e.mem.set_u8(w.part + 0x60, 0x48);
        w.s.e.mem.set_u8(w.part + 0x65, 100);
        let log = w.dismember(0x1a, 4, 0);
        // Severable: the outcome is set at once and 00646e50 is not asked.
        assert!(args_of(&log, 0x0064_6e50).is_empty());
        assert_eq!(args_of(&log, 0x00a5_a040).len(), 1);

        let mut w = limb_stage();
        let actor = w.actor.addr();
        w.s.e.mem.set_u8(w.part + 0x60, 0x01);
        w.s.e.mem.set_u8(w.part + 0x65, 33);
        let log = w.dismember(0x1a, 4, 0);
        let asked = args_of(&log, 0x0064_6e50);
        assert_eq!(asked.len(), 1);
        // (magic target, source, chance byte, chance, limit, part flag, two byte pointers):
        // the chance byte is only read for a part with flag 0x08.
        assert_eq!(asked[0][..6], [actor + 0xa4, 0, 0, 0xe3, 0xe4, 0]);
        assert!(args_of(&log, 0x00a5_a040).is_empty());
    }

    #[test]
    fn dismember_takes_the_source_from_the_extra_word_or_the_hit() {
        let mut w = limb_stage();
        w.s.e.mem.set_u8(w.part + 0x60, 0x01);
        w.s.e.register(0x0040_1170, |_, _| int(0x28));
        let log = w.dismember(0x1a, 4, 0);
        assert_eq!(args_of(&log, 0x0064_6e50)[0][1], 0xe1);

        let mut w = limb_stage();
        w.s.e.mem.set_u8(w.part + 0x60, 0x01);
        w.s.e.set_global::<u32>(0x011c_a278, 0x5150);
        let hit = w.s.e.mem.alloc(0x40);
        w.s.e.mem.set_u32(hit + 0x10, 4);
        w.s.e.mem.set_u32(hit + 0x30, 0x4242);
        let log = w.dismember(0x1a, 4, hit);
        assert_eq!(args_of(&log, 0x0064_6e50)[0][1], 0x4242);
        w.s.e.mem.set_u32(hit + 0x30, 0);
        w.s.e.mem.set_u32(hit + 0xc, 0x2d);
        let log = w.dismember(0x1a, 4, hit);
        assert_eq!(args_of(&log, 0x0064_6e50)[0][1], 0x5150);
        // A hit on another part gives no source.
        w.s.e.mem.set_u32(hit + 0x10, 5);
        let log = w.dismember(0x1a, 4, hit);
        assert_eq!(args_of(&log, 0x0064_6e50)[0][1], 0);
    }

    #[test]
    fn dismember_makes_the_piece_for_the_part_with_the_value() {
        let mut w = limb_stage();
        let actor = w.actor.addr();
        let (limb, group) = (w.limb, w.group);
        // Listed (0x01) and severable (0x08, 0x40, chance 100): the outcome stays
        // set, so the piece is only hidden (`hide_only`): no clone is made and
        // slot 0x73c of the process gets the limb.
        w.s.e.mem.set_u8(w.part + 0x60, 0x49);
        w.s.e.mem.set_u8(w.part + 0x65, 100);
        let log = w.dismember(0x1a, 4, 0);
        assert_eq!(
            args_of(&log, 0x0057_2fc0),
            vec![vec![actor, 4, 0xe1, 0xe2, 4, 1]]
        );
        assert!(args_of(&log, 0x00a5_d2c0).is_empty());
        assert!(w
            .slot_calls_on(w.process)
            .contains(&(0x73c, vec![4, limb, group, 1])));
        // The outcome byte is cleared for a part that is not severable, so a clone is made.
        let mut w = limb_stage();
        w.s.e.mem.set_u8(w.part + 0x60, 0x01);
        w.s.e.register(0x0064_6e50, |e, a| {
            e.mem.set_u8(a[7], 1);
            Ret::default()
        });
        let frame = next_frame(&mut w.s.e, 0x324);
        let log = w.dismember(0x1a, 4, 0);
        // The 0x324-byte frame of 008b52a0: the name at +0x314, the cloning process at +0x248, the
        // transforms at +0x2c4, +0x264, +0x1dc and +0x210.
        assert_eq!(args_of(&log, 0x0043_8170)[0][0], frame + 0x314);
        assert_eq!(args_of(&log, 0x004a_d050)[0][0], frame + 0x248);
        let transforms: Vec<u32> = args_of(&log, 0x0047_6a80).iter().map(|c| c[0]).collect();
        assert_eq!(
            transforms,
            vec![frame + 0x2c4, frame + 0x264, frame + 0x1dc, frame + 0x210]
        );
        assert_eq!(args_of(&log, 0x00a5_d2c0).len(), 1);
        assert!(w
            .slot_calls_on(w.process)
            .contains(&(0x6cc, vec![4, 0xc10c])));
        // A part with another actor value is skipped.
        let mut w = limb_stage();
        w.s.e.mem.set_u8(w.part + 0x60, 0x49);
        w.s.e.mem.set_u8(w.part + 0x65, 100);
        w.s.e.mem.set_u8(w.part + 0x61, 0x1b);
        let log = w.dismember(0x1a, 4, 0);
        assert!(args_of(&log, 0x00a5_a040).is_empty());
        // A limb the check rejects is skipped too.
        let mut w = limb_stage();
        w.s.e.mem.set_u8(w.part + 0x60, 0x49);
        w.s.e.mem.set_u8(w.part + 0x65, 100);
        w.s.e.register(0x0045_6610, |_, _| int(1));
        let log = w.dismember(0x1a, 4, 0);
        assert!(args_of(&log, 0x00a5_a040).is_empty());
    }

    #[test]
    fn dismember_goes_through_the_task_queue_when_asked_to() {
        let mut w = limb_stage();
        let actor = w.actor.addr();
        w.s.e.mem.set_u8(w.part + 0x60, 0x49);
        w.s.e.mem.set_u8(w.part + 0x65, 100);
        w.s.e.register(0x008c_7aa0, |_, _| int(1));
        w.s.e.register(0x0045_37b0, |_, _| int(0x7777));
        let log = w.dismember(0x1a, 4, 0);
        assert!(args_of(&log, 0x00a5_a040).is_empty());
        assert_eq!(
            args_of(&log, 0x0087_b440),
            vec![vec![0x7777, actor, 0x1a, 4, 1, 1]]
        );
    }

    #[test]
    fn dismember_with_the_single_part_setting_stops_after_one_part() {
        let mut w = limb_stage();
        w.s.e.mem.set_u8(w.part + 0x60, 0x49);
        w.s.e.mem.set_u8(w.part + 0x65, 100);
        w.s.e.mem.set_u8(DISMEMBER_SINGLE_PART_SETTING, 1);
        let log = w.dismember(0x1a, 4, 0);
        assert_eq!(args_of(&log, 0x00a5_a040).len(), 1);
        assert_eq!(args_of(&log, 0x0057_2fc0).len(), 1);
    }

    #[test]
    fn script_dismember_asks_the_value_and_dismembers_without_a_hit() {
        let mut w = limb_stage();
        let actor = w.actor;
        let a = actor.addr();
        w.s.e.mem.set_u8(w.part + 0x61, 0x1c);
        let seen: Rc<std::cell::RefCell<Vec<Vec<u32>>>> = Default::default();
        let sink = seen.clone();
        w.s.e.register_double(0x0f30_0000 + 0xc, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            float(2.5)
        });
        let part = w.part;
        w.s.e.register_double(0x005e_50f0, move |_, _| int(part));
        let ((), log) = logged(&mut w.s.e, |e| {
            e.call(0x008b_51b0, &args![actor, 0x77u32, 3u32, 0x66u32]);
        });
        // The slot gets twice the answer.
        assert_eq!(seen.borrow()[0], vec![a + 0xa4, 0x1c]);
        assert!(w
            .s
            .slot_calls()
            .contains(&(a, 0x3ac, vec![0x1c, 5.0f32.to_bits(), 0x66])));
        // Dismember: no hit, the value, the part 3, chance 0 and limit 1000.
        assert_eq!(args_of(&log, 0x0057_3090), vec![vec![a, 3]]);
        assert_eq!(
            args_of(&log, 0x0064_6e50)[0][..6],
            [a + 0xa4, 0, 0, 0, 1000, 0]
        );
        // Without a part the value is -1 and the part is only marked.
        w.s.e.register(0x005e_50f0, |_, _| int(0));
        let ((), log) = logged(&mut w.s.e, |e| {
            e.call(0x008b_51b0, &args![actor, 0x77u32, 0u32, 0x66u32]);
        });
        assert_eq!(
            args_of(&log, 0x0057_2fc0),
            vec![vec![a, u32::MAX, 0, 0x77, 0, 0]]
        );
    }

    #[test]
    fn script_dismember_needs_3d() {
        let mut w = limb_stage();
        let actor = w.actor;
        w.s.answer(actor.addr(), 0x1d0, 0);
        let ((), log) = logged(&mut w.s.e, |e| {
            e.call(0x008b_51b0, &args![actor, 0u32, 3u32, 0u32]);
        });
        assert_eq!(log.len(), 2);
    }

    fn limb_piece_run(
        w: &mut LimbStage,
        hide_only: u8,
        flag_14: u8,
        flag_18: u8,
        flag_1c: u8,
    ) -> Vec<(u32, Vec<u32>)> {
        let actor = w.actor;
        logged(&mut w.s.e, |e| {
            e.call(
                0x008b_52a0,
                &args![
                    actor,
                    0x1au32,
                    STAGE_PART_INDEX,
                    hide_only,
                    flag_14,
                    flag_18,
                    flag_1c
                ],
            );
        })
        .1
    }

    #[test]
    fn limb_piece_is_skipped_for_blocked_creatures() {
        let mut w = limb_stage();
        w.s.e.mem.set_u8(DISMEMBER_BLOCK_SETTING, 1);
        w.s.answer(w.actor.addr(), 0x218, 1);
        let log = limb_piece_run(&mut w, 0, 0, 0, 0);
        assert!(args_of(&log, 0x00a5_a040).is_empty());
        assert!(args_of(&log, 0x0043_fcd0).is_empty());
    }

    #[test]
    fn limb_piece_clones_the_limb_and_throws_it() {
        let mut w = limb_stage();
        let (root, limb, clone, group, havok) = (w.root, w.limb, w.clone, w.group, w.havok);
        w.s.e
            .set_global::<u32>(LIMB_NAME_TABLE + 4 * STAGE_PART_INDEX, 0x4e4e);
        let log = limb_piece_run(&mut w, 0, 0, 0, 0);

        // The name of the part is built from the name table, the group node is
        // attached to the root and placed at the origin.
        assert_eq!(args_of(&log, 0x0043_8170)[0][1], 0x4e4e);
        let name = args_of(&log, 0x0043_8170)[0][0];
        assert_eq!(args_of(&log, 0x00a5_b950), vec![vec![group, name]]);
        assert!(w.slot_calls_on(root).contains(&(0xdc, vec![group, 1])));
        assert_eq!(w.slot_calls_on(group)[0].0, 0xb8);

        // The euler angles of the body part data (degrees) become radians.
        let factor = f64::from_bits(0x3f91_df46_a000_0000);
        let radians = |degrees: f32| ((f64::from(degrees) * factor) as f32).to_bits();
        let matrix_calls = args_of(&log, 0x00a5_9540);
        assert_eq!(matrix_calls.len(), 1);
        assert_eq!(
            matrix_calls[0][1..],
            [radians(10.0), radians(20.0), radians(30.0)]
        );
        assert_eq!(args_of(&log, 0x0041_6870).len(), 1);

        // The limb is cloned and the clone placed from the composed transform.
        let cloning = args_of(&log, 0x00a5_d2c0);
        assert_eq!(cloning.len(), 1);
        assert_eq!(cloning[0][0], limb);
        assert_eq!(args_of(&log, 0x004a_de00), vec![vec![clone, 7]]);
        let translate = args_of(&log, 0x0044_0460);
        assert_eq!(translate[0][0], clone);
        assert_eq!(w.s.e.mem.u32(translate[0][1]), COMPOSED + 9);
        let rotate = args_of(&log, 0x0043_fa80);
        assert_eq!(rotate[0][0], clone);
        assert_eq!(w.s.e.mem.u32(rotate[0][1]), COMPOSED);
        assert_eq!(args_of(&log, 0x0044_0490), vec![vec![clone, COMPOSED + 12]]);
        assert!(w
            .slot_calls_on(w.process)
            .contains(&(0x6cc, vec![STAGE_PART_INDEX, 0xc10c])));

        // The clone joins the group, gets collision, motion and a velocity.
        assert!(w.slot_calls_on(group).contains(&(0xdc, vec![clone, 1])));
        assert_eq!(args_of(&log, 0x00c6_bd00), vec![vec![clone, 1]]);
        assert_eq!(args_of(&log, 0x00c6_a350), vec![vec![clone, 1, 1, 1, 1]]);
        assert!(w
            .slot_calls_on(havok)
            .contains(&(0xd0, vec![clone, 1, 0, 0, 1])));
        let velocity = args_of(&log, 0x0062_b8d0);
        assert_eq!(velocity.len(), 1);
        assert_eq!(velocity[0][0], clone);
        assert_eq!(velocity[0][2], 0);
        let scaling = args_of(&log, 0x0045_bb20);
        assert_eq!(scaling[0][1], velocity[0][1]);
        assert_eq!(scaling[0][2], 2.0f32.to_bits());
        let direction = args_of(&log, 0x0043_9ef0);
        assert_eq!(direction[0][0], clone + 0x8c);
        assert_eq!(w.s.e.mem.f32(direction[0][2]), 1.0);
        assert_eq!(args_of(&log, 0x004a_0c10), vec![vec![direction[0][1]]]);

        // The limb is hidden and the property update runs; the name is released.
        assert_eq!(args_of(&log, 0x0045_0f90).last(), Some(&vec![limb, 1]));
        assert_eq!(args_of(&log, 0x00a5_a040), vec![vec![root]]);
        let released = args_of(&log, 0x0043_81b0);
        assert_eq!(released.len(), 2);
        assert_eq!(released[1], vec![name]);
    }

    #[test]
    fn limb_piece_places_the_clone_with_the_parent_node_when_the_root_has_the_part() {
        let mut w = limb_stage();
        let root = w.root;
        w.s.answer(root, 0x9c, 0x9a9a);
        let log = limb_piece_run(&mut w, 0, 0, 0, 0);
        // The parent's transform (46 1130 of the node found by name) is the composing side.
        assert_eq!(args_of(&log, 0x0046_1130)[0], vec![0x9a9a]);
    }

    #[test]
    fn limb_piece_plays_the_effects_of_the_part() {
        let mut w = limb_stage();
        let (limb, clone) = (w.limb, w.clone);
        w.s.e.mem.set_u8(w.part + 0x78, 3);
        w.s.e.mem.set_u8(w.part + 0xa8, 5);
        w.s.e.register(0x004f_b070, |_, _| int(0xef));
        w.s.e.register(0x0040_7840, |_, _| int(0xb1));
        w.s.e.register(0x0056_7400, |_, _| float(1.5));
        w.s.e.register(0x0064_47b0, |_, _| float(2.0));
        // Debris objects: a sound (with a vtable at +0x18 for the list), a selector.
        let sound = w.s.object(0x40);
        let vtable = w.s.vtable;
        w.s.e.mem.set_u32(sound + 0x18, vtable);
        w.s.answer(sound + 0x18, 0x14, 0x7a7a);
        w.s.e.register(0x004f_d400, |_, _| int(0x77));
        w.s.e.register_double(0x0058_e9d0, move |_, _| int(sound));
        w.s.e.register(0x0048_cee0, |_, _| int(1));
        w.s.e.register(0x0067_33e0, |_, _| int(1));
        w.s.e.register(0x0068_9210, |_, _| int(0xc4c4));
        w.s.e.register(0x0040_1000, |_, _| int(0xb10c));
        w.s.e.register(0x004a_2c70, |_, _| int(0x0b1e));
        w.s.e.register(0x004b_45b0, |e, a| {
            for word in 0..9 {
                e.mem.set_u32(a[1] + 4 * word, 0x2000 + word);
            }
            int(a[1])
        });
        let log = limb_piece_run(&mut w, 0, 1, 1, 0);

        // Explosion of the part: effect, 0, 0, the cell, the translation, the rotation.
        let mut explosion = vec![0xef, 0, 0, 0x5000];
        explosion.extend((9..12).map(|word| COMPOSED + word));
        explosion.extend((0..9).map(|word| COMPOSED + word));
        assert_eq!(args_of(&log, 0x009a_c9c0), vec![explosion]);
        // The model object: scale (actor 1.5 * part 2.0) and the count byte (3).
        let model = args_of(&log, 0x004f_a6f0);
        assert_eq!(model.len(), 1);
        assert_eq!(model[0][0], 0xb1);
        assert_eq!(model[0][1], 0x5000);
        assert_eq!(model[0][3], 3);
        assert_eq!(model[0][4], 3.0f32.to_bits());

        // Debris, for the parent (hide_only is clear: so also for the clone, inverted).
        let debris = args_of(&log, 0x0068_9210);
        assert_eq!(debris.len(), 2);
        let mut expected = vec![0x5000, 1.0f32.to_bits(), 0x7a7a];
        expected.extend((0..9).map(|word| COMPOSED + word));
        expected.extend((9..12).map(|word| COMPOSED + word));
        expected.extend([1.5f32.to_bits(), 7, limb]);
        assert_eq!(debris[0], expected);
        let mut expected = vec![0x5000, 1.0f32.to_bits(), 0x7a7a];
        expected.extend((0..9).map(|word| 0x2000 + word));
        expected.extend((9..12).map(|word| COMPOSED + word));
        expected.extend([1.5f32.to_bits(), 7, clone]);
        assert_eq!(debris[1], expected);
        assert_eq!(args_of(&log, 0x0068_a8d0), vec![vec![0xc4c4, sound]; 2]);
        assert_eq!(args_of(&log, 0x0040_1000), vec![vec![0x10]; 2]);
        assert_eq!(
            args_of(&log, 0x004a_2c70),
            vec![vec![0xb10c, 0xc4c4, sound, 5]; 2]
        );
        assert_eq!(args_of(&log, 0x004a_1a50), vec![vec![0x0b1e]; 2]);
    }

    #[test]
    fn limb_piece_skips_the_debris_when_the_actor_has_the_blocking_word() {
        let mut w = limb_stage();
        let actor = w.actor.addr();
        let sound = w.s.object(0x40);
        let vtable = w.s.vtable;
        w.s.e.mem.set_u32(sound + 0x18, vtable);
        w.s.e.register(0x004f_d400, |_, _| int(0x77));
        w.s.e.register_double(0x0058_e9d0, move |_, _| int(sound));
        w.s.e.register(0x0048_cee0, |_, _| int(1));
        w.s.e.mem.set_u32(actor + 0x10c, 1);
        let log = limb_piece_run(&mut w, 0, 1, 1, 0);
        assert!(args_of(&log, 0x0068_9210).is_empty());
        w.s.e.mem.set_u32(actor + 0x10c, 0);
        w.s.e.register(0x0048_cee0, |_, _| int(0));
        let log = limb_piece_run(&mut w, 0, 1, 1, 0);
        assert!(args_of(&log, 0x0068_9210).is_empty());
    }

    #[test]
    fn limb_piece_hides_the_node_found_by_name() {
        let mut w = limb_stage();
        let (limb, child, root) = (w.limb, w.child, w.root);
        // The bone table holds the limb (it is filled before in a real run).
        w.s.e.set_global::<u32>(LIMB_BONE_COUNT, 1);
        w.s.e.set_global::<u32>(LIMB_BONES, limb);
        w.s.answer(child, 0x18, 1);
        w.s.e.register(0x0041_3f40, |_, a| int(a[0]));
        w.s.e
            .register_double(0x009a_3830, move |_, a| int(u32::from(a[0] == child)));
        w.s.e.register(0x0049_10d0, |_, _| int(1));
        w.s.e.register(0x0043_fad0, |_, _| int(0x5c1));
        w.s.e.register(0x0055_85e0, |_, _| int(0xb01));
        w.s.e.register(0x0080_41a0, |_, _| int(1));
        let list = w.s.e.mem.alloc(8);
        w.s.e.mem.set_u32(list, limb);
        w.s.e.register_double(0x0082_5c00, move |_, _| int(list));
        let body_parts = w.body_parts;
        let log = limb_piece_run(&mut w, 1, 0, 0, 0);
        // `actor_hide_dismembered_limb` hides it: 005e4730 then 005e55e0.
        assert_eq!(
            args_of(&log, 0x005e_4730),
            vec![vec![STAGE_PART_INDEX, child]]
        );
        assert_eq!(
            args_of(&log, 0x005e_55e0),
            vec![vec![body_parts, limb, root, STAGE_PART_INDEX, child]]
        );
        // The function ends there: nothing is created.
        assert!(args_of(&log, 0x00aa_13e0).is_empty());
        assert!(args_of(&log, 0x00a5_a040).is_empty());
        assert_eq!(args_of(&log, 0x0043_81b0).len(), 1);
    }

    #[test]
    fn limb_piece_without_a_match_uses_the_process_slot_or_the_hide_function() {
        let mut w = limb_stage();
        let (limb, group) = (w.limb, w.group);
        let log = limb_piece_run(&mut w, 1, 0, 0, 0);
        assert!(w
            .slot_calls_on(w.process)
            .contains(&(0x73c, vec![STAGE_PART_INDEX, limb, group, 1])));
        assert_eq!(args_of(&log, 0x00a5_a040).len(), 1);
        // The group node is the piece, nothing is cloned.
        assert!(args_of(&log, 0x00a5_d2c0).is_empty());
        // With the third option the hide function is used instead.
        let mut w = limb_stage();
        w.s.e.register(0x0049_10d0, |_, _| int(0));
        let _ = limb_piece_run(&mut w, 1, 0, 0, 1);
        assert!(w
            .slot_calls_on(w.process)
            .iter()
            .all(|call| call.0 != 0x73c));
    }

    #[test]
    fn limb_piece_tells_vats_and_the_sibling() {
        let mut w = limb_stage();
        let (clone, root) = (w.clone, w.root);
        let actor = w.actor.addr();
        let target = w.s.e.mem.alloc(0x40);
        let action = w.s.e.mem.alloc(0x40);
        w.s.e.mem.set_u32(action + 0x10, 0x1a);
        w.s.e.register(0x0044_ddc0, |_, a| {
            int(if a[0] == VATS_INSTANCE { 4 } else { 0 })
        });
        w.s.e.register_double(0x009c_71c0, move |_, _| int(action));
        w.s.e.register_double(0x0044_edb0, move |_, _| int(target));
        w.s.e.register(0x0063_9b40, |_, _| int(2));
        w.s.e.register_double(0x008d_6f30, move |_, a| {
            int(if a[0] == target { 2 } else { 0x5000 })
        });
        let sibling = w.s.e.mem.alloc(0x40);
        w.s.e.mem.set_u32(actor + 0xb0, sibling);
        w.s.e.register(0x0093_1ed0, |_, _| int(0x4d4d));
        w.s.e.register(0x004a_3a20, |_, a| int(a[0] + 1));
        let log = limb_piece_run(&mut w, 0, 0, 0, 0);
        assert_eq!(args_of(&log, 0x0058_cbc0), vec![vec![target, clone]]);
        assert_eq!(args_of(&log, 0x0058_ccf0), vec![vec![target, clone]]);
        assert_eq!(
            args_of(&log, 0x00ca_2ad0),
            vec![vec![sibling, root, 0x4d4e]]
        );
        // Another action's attacker, or another VATS state, is left alone.
        w.s.e.mem.set_u32(action + 0x10, 0x1b);
        let log = limb_piece_run(&mut w, 0, 0, 0, 0);
        assert!(args_of(&log, 0x0058_cbc0).is_empty());
    }

    #[test]
    fn limb_piece_removes_the_collision_of_the_limb_children_and_resets_the_clone() {
        let mut w = limb_stage();
        let (limb, clone) = (w.limb, w.clone);
        let (root, child) = (w.root, w.child);
        w.s.e.register(0x0068_38b0, |_, _| int(1));
        w.s.e.register(0x0045_3470, |_, _| int(1));
        w.s.e.register_double(0x0043_b480, move |_, a| {
            int(u32::from(a[0] == root || a[0] == limb))
        });
        w.s.e.register_double(0x0043_b4a0, move |_, a| {
            int(if a[0] == root || a[0] == limb {
                child
            } else {
                0
            })
        });
        w.s.e.register(0x004b_5260, |_, _| int(0xc011));
        w.s.e.register(0x006f_a820, |_, _| int(0xb0d1));
        let log = limb_piece_run(&mut w, 0, 0, 0, 0);
        assert!(args_of(&log, 0x00c6_9ee0).contains(&vec![child, 1, 1]));
        assert!(args_of(&log, 0x00c6_a200).contains(&vec![child, 1, 1, 1]));
        assert_eq!(args_of(&log, 0x004b_5260), vec![vec![clone]]);
        assert_eq!(args_of(&log, 0x006f_a820), vec![vec![0xc011]]);
        assert_eq!(args_of(&log, 0x00c8_ec60), vec![vec![0xb0d1]]);
    }

    /// Writes `text` as a C string in fresh memory and returns its address.
    fn cstr(e: &mut Engine, text: &str) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(at, text.as_bytes());
        at
    }

    /// The bone table holds the limb, and `child` is skinned to it.
    fn make_child_skinned(w: &mut LimbStage) {
        let (limb, child) = (w.limb, w.child);
        w.s.e.set_global::<u32>(LIMB_BONE_COUNT, 1);
        w.s.e.set_global::<u32>(LIMB_BONES, limb);
        w.s.answer(child, 0x18, 1);
        w.s.e.register(0x0049_10d0, |_, _| int(1));
        w.s.e.register(0x0043_fad0, |_, _| int(0x5c1));
        w.s.e.register(0x0055_85e0, |_, _| int(0xb01));
        w.s.e.register(0x0080_41a0, |_, _| int(1));
        let list = w.s.e.mem.alloc(8);
        w.s.e.mem.set_u32(list, limb);
        w.s.e.register_double(0x0082_5c00, move |_, _| int(list));
    }

    #[test]
    fn body_part_data_readers_of_008b6200_to_008b6260() {
        let (mut e, _) = engine();
        let entry = e.mem.alloc(0x100);
        e.register(0x0055_9450, |_, a| int(a[0] + 1));
        assert_eq!(e.call(0x008b_6200, &args![entry]).u32(), entry + 0x24 + 1);
        e.mem.set_u8(entry + 0x78, 7);
        e.mem.set_u8(entry + 0xa8, 9);
        assert_eq!(e.call(0x008b_6220, &args![entry]).u8(), 7);
        assert_eq!(e.call(0x008b_6240, &args![entry]).u8(), 9);
        for (word, value) in [0x11u32, 0x22, 0x33].into_iter().enumerate() {
            e.mem.set_u32(entry + 0x88 + 4 * word as u32, value);
        }
        let out = e.mem.alloc(0x10);
        assert_eq!(e.call(0x008b_6260, &args![entry, out]).u32(), out);
        assert_eq!(
            [e.mem.u32(out), e.mem.u32(out + 4), e.mem.u32(out + 8)],
            [0x11, 0x22, 0x33]
        );
    }

    #[test]
    fn dismemberment_extra_data_count_of_008b6800() {
        let (mut e, _) = engine();
        e.register(0x0044_ddc0, |_, a| int(a[0] + 5));
        assert_eq!(
            e.call(0x008b_6800, &args![0x1000u32]).u32(),
            0x1000 + 0x20 + 5
        );
    }

    #[test]
    fn hide_dismembered_limb_hides_the_skinned_nodes() {
        let mut w = limb_stage();
        make_child_skinned(&mut w);
        let (actor, limb, child, root, body_parts) =
            (w.actor, w.limb, w.child, w.root, w.body_parts);
        let (_, log) = logged(&mut w.s.e, |e| {
            e.call(
                0x008b_6290,
                &args![actor, STAGE_PART_INDEX, limb, 0xbe1u32, 1u32],
            )
        });
        assert_eq!(
            args_of(&log, 0x005e_4730),
            vec![vec![STAGE_PART_INDEX, child]]
        );
        assert_eq!(
            args_of(&log, 0x005e_55e0),
            vec![vec![body_parts, limb, root, STAGE_PART_INDEX, child]]
        );
        // Part 0 hides the node itself instead.
        w.s.e
            .register_double(0x005e_50f0, move |_, _| int(body_parts));
        let (_, log) = logged(&mut w.s.e, |e| {
            e.call(0x008b_6290, &args![actor, 0u32, limb, 0xbe1u32, 1u32])
        });
        assert_eq!(args_of(&log, 0x005e_4730), vec![vec![0, child]]);
        assert_eq!(args_of(&log, 0x0045_0f90), vec![vec![child, 1]]);
        assert!(args_of(&log, 0x005e_55e0).is_empty());
    }

    #[test]
    fn hide_dismembered_limb_clones_and_attaches_the_node_when_not_hiding_only() {
        let mut w = limb_stage();
        make_child_skinned(&mut w);
        let (actor, limb, child, root, body_parts, clone) =
            (w.actor, w.limb, w.child, w.root, w.body_parts, w.clone);
        let owner = w.s.object(0x40);
        let world = w.s.object(0x40);
        w.s.answer(owner, 0xc, 0x5ce1);
        w.s.e.register_double(0x0096_11e0, move |_, a| {
            int(if a[0] == owner { world } else { 0 })
        });
        let (_, log) = logged(&mut w.s.e, |e| {
            e.call(
                0x008b_6290,
                &args![actor, STAGE_PART_INDEX, limb, owner, 0u32],
            )
        });
        assert_eq!(args_of(&log, 0x00a5_d2c0)[0][0], child);
        assert_eq!(
            args_of(&log, 0x005e_4730),
            vec![vec![STAGE_PART_INDEX, child]]
        );
        assert_eq!(
            args_of(&log, 0x005e_55e0),
            vec![vec![body_parts, limb, root, STAGE_PART_INDEX, child]]
        );
        assert_eq!(
            args_of(&log, 0x005e_4970),
            vec![vec![STAGE_PART_INDEX, clone]]
        );
        assert_eq!(
            args_of(&log, 0x005e_5750),
            vec![vec![body_parts, limb, root, STAGE_PART_INDEX, clone]]
        );
        assert!(w.slot_calls_on(world).contains(&(0xdc, vec![clone, 1])));
        assert_eq!(args_of(&log, 0x004a_de40), vec![vec![0x5ce1, clone, 0, 0]]);
        assert_eq!(args_of(&log, 0x004a_d270).len(), 1);
    }

    #[test]
    fn hide_dismembered_limb_needs_the_part_and_the_limb() {
        let mut w = limb_stage();
        make_child_skinned(&mut w);
        let (actor, limb) = (w.actor, w.limb);
        let (_, log) = logged(&mut w.s.e, |e| {
            e.call(0x008b_6290, &args![actor, 3u32, limb, 0xbe1u32, 1u32])
        });
        assert!(args_of(&log, 0x0049_10d0).is_empty());
        let (_, log) = logged(&mut w.s.e, |e| {
            e.call(
                0x008b_6290,
                &args![actor, STAGE_PART_INDEX, 0u32, 0xbe1u32, 1u32],
            )
        });
        assert!(args_of(&log, 0x0049_10d0).is_empty());
    }

    #[test]
    fn unhide_dismembered_limb_shows_the_skinned_nodes() {
        let mut w = limb_stage();
        make_child_skinned(&mut w);
        let (actor, limb, child, root, body_parts) =
            (w.actor, w.limb, w.child, w.root, w.body_parts);
        let (_, log) = logged(&mut w.s.e, |e| {
            e.call(0x008b_64e0, &args![actor, STAGE_PART_INDEX, limb])
        });
        assert_eq!(
            args_of(&log, 0x005e_4890),
            vec![vec![STAGE_PART_INDEX, child]]
        );
        assert_eq!(
            args_of(&log, 0x005e_5750),
            vec![vec![body_parts, limb, root, STAGE_PART_INDEX, child]]
        );
        assert!(args_of(&log, 0x0045_0f90).is_empty());
        // Part 0 shows the node first.
        w.s.e
            .register_double(0x005e_50f0, move |_, _| int(body_parts));
        let (_, log) = logged(&mut w.s.e, |e| {
            e.call(0x008b_64e0, &args![actor, 0u32, limb])
        });
        assert_eq!(args_of(&log, 0x0045_0f90), vec![vec![child, 0]]);
        // No limb: nothing.
        let (_, log) = logged(&mut w.s.e, |e| {
            e.call(0x008b_64e0, &args![actor, 0u32, 0u32])
        });
        assert!(args_of(&log, 0x005e_5750).is_empty());
    }

    #[test]
    fn dismembered_limbs_are_thrown_one_by_one() {
        let mut w = limb_stage();
        let (actor, root) = (w.actor, w.root);
        let nodes = w.s.object(0x40);
        w.s.answer(root, 0xc, nodes);
        let (n0, n1) = (w.s.object(0x100), w.s.object(0x100));
        let record = w.s.e.mem.alloc(0x40);
        let entries = [w.s.e.mem.alloc(8), w.s.e.mem.alloc(8)];
        w.s.e.mem.set_u8(entries[0], 1);
        w.s.e.mem.set_u8(entries[1], 2);
        w.s.e.set_global::<u32>(LIMB_NAME_TABLE + 4, 0x1111);
        w.s.e.set_global::<u32>(LIMB_NAME_TABLE + 8, 0x2222);
        w.s.e.register(0x005d_43c0, |_, a| int(a[0]));
        w.s.e.register_double(0x0042_e8c0, move |_, _| int(record));
        w.s.e.register(0x0044_ddc0, |_, _| int(2));
        w.s.e
            .register_double(0x0044_1420, move |_, a| int(entries[a[1] as usize]));
        w.s.e.register_double(0x0043_b480, move |_, a| {
            int(if a[0] == nodes { 2 } else { 0 })
        });
        w.s.e.register_double(0x0043_b4a0, move |_, a| {
            int(if a[1] == 0 { n0 } else { n1 })
        });
        w.s.e.register(0x0041_3f40, |_, a| int(a[0]));
        w.s.e
            .register_double(0x009a_3830, move |_, a| int(u32::from(a[0] == n1)));
        let frame = next_frame(&mut w.s.e, 0x70);
        let (_, log) = logged(&mut w.s.e, |e| e.call(0x008b_65f0, &args![actor]));
        // The 0x70-byte frame: the name at +0x4c, the bound copy at +0x24, the direction at +0xc,
        // the velocity at +0x18.
        assert_eq!(args_of(&log, 0x0043_8170)[0][0], frame + 0x4c);
        assert_eq!(args_of(&log, 0x0068_15c0)[0][0], frame + 0x24);
        assert_eq!(
            args_of(&log, 0x0045_bb20)[0][..2],
            [frame + 0xc, frame + 0x18]
        );
        assert_eq!(args_of(&log, 0x0062_b8d0)[0][1], frame + 0x18);
        assert_eq!(args_of(&log, 0x00c6_a270), vec![vec![nodes, 1, 1, 0]]);
        let names = args_of(&log, 0x0043_8170);
        assert_eq!(
            names.iter().map(|call| call[1]).collect::<Vec<_>>(),
            [0x1111, 0x2222]
        );
        assert_eq!(args_of(&log, 0x009a_3830).len(), 4);
        let thrown = args_of(&log, 0x0062_b8d0);
        assert_eq!(
            thrown.iter().map(|call| call[0]).collect::<Vec<_>>(),
            [n1, n1]
        );
        assert_eq!(args_of(&log, 0x0043_81b0).len(), 2);
        // No matching node: nothing is thrown.
        w.s.e.register(0x009a_3830, |_, _| int(0));
        let (_, log) = logged(&mut w.s.e, |e| e.call(0x008b_65f0, &args![actor]));
        assert!(args_of(&log, 0x0062_b8d0).is_empty());
        // Without the extra data nothing happens.
        w.s.e.register(0x0042_e8c0, |_, _| int(0));
        let (_, log) = logged(&mut w.s.e, |e| e.call(0x008b_65f0, &args![actor]));
        assert_eq!(log.len(), 3);
    }

    #[test]
    fn clearing_dismembered_limbs_restores_the_flagged_parts() {
        let mut w = limb_stage();
        let (actor, root, limb, child, havok, process, body_parts) = (
            w.actor,
            w.root,
            w.limb,
            w.child,
            w.havok,
            w.process,
            w.body_parts,
        );
        w.s.e.map(0x0108_4000, 0x1000);
        w.s.e
            .mem
            .write(DISMEMBERED_LIMB_PREFIX, b"DismemberedLimb\0");
        let nodes = w.s.object(0x40);
        w.s.answer(root, 0xc, nodes);
        let (n0, n1, n2) = (w.s.object(0x100), w.s.object(0x100), w.s.object(0x100));
        let text0 = cstr(&mut w.s.e, "DismemberedLimb2_Arm");
        let text1 = cstr(&mut w.s.e, "Something");
        w.s.e.register(0x005d_43c0, |_, a| int(a[0]));
        w.s.e.register(0x0041_3f40, |_, a| int(a[0]));
        w.s.e.register_double(0x0043_b1b0, move |_, a| {
            int(match a[0] {
                x if x == n0 => text0,
                x if x == n1 => text1,
                _ => 0,
            })
        });
        w.s.e.register(0x0045_64f0, |e, a| {
            let mut head = [0u8; 15];
            e.mem.read(a[0], &mut head);
            int(u32::from(&head != b"DismemberedLimb"))
        });
        w.s.e.register(0x00ec_a6d3, |_, _| int(2));
        w.s.e.register_double(0x0043_b480, move |_, a| {
            int(if a[0] == nodes {
                3
            } else if a[0] == limb {
                1
            } else {
                0
            })
        });
        w.s.e.register_double(0x0043_b4a0, move |_, a| {
            int(if a[0] == nodes {
                [n0, n1, n2][a[1] as usize]
            } else if a[0] == limb {
                child
            } else {
                0
            })
        });
        w.s.e.register(0x0068_38b0, |_, _| int(1));
        w.s.e.register(0x0045_3470, |_, _| int(1));
        let (_, log) = logged(&mut w.s.e, |e| e.call(0x008b_6820, &args![actor]));
        // Only the first node has the prefix: its objects are removed and it leaves the list.
        assert_eq!(args_of(&log, 0x00c6_9ee0), vec![vec![n0, 1, 1]]);
        assert_eq!(args_of(&log, 0x0045_64f0).len(), 2);
        assert!(w.slot_calls_on(nodes).contains(&(0xf0, vec![0])));
        // Part 2 is restored: process slot 0x6c8, bones, show, collision on again.
        assert!(w
            .slot_calls_on(process)
            .contains(&(0x6c8, vec![root, body_parts])));
        assert!(args_of(&log, 0x0045_0f90).contains(&vec![limb, 0]));
        assert_eq!(args_of(&log, 0x00c6_a200), vec![vec![child, 0, 1, 1]]);
        assert!(w
            .slot_calls_on(havok)
            .contains(&(0xd0, vec![child, 1, 0, 0, 1])));
    }

    #[test]
    fn clearing_dismembered_limbs_does_nothing_without_a_flagged_node() {
        let mut w = limb_stage();
        let (actor, root) = (w.actor, w.root);
        let nodes = w.s.object(0x40);
        w.s.answer(root, 0xc, nodes);
        w.s.e.register(0x005d_43c0, |_, a| int(a[0]));
        let (_, log) = logged(&mut w.s.e, |e| e.call(0x008b_6820, &args![actor]));
        assert!(args_of(&log, 0x00c6_9ee0).is_empty());
        assert!(args_of(&log, 0x0045_0f90).is_empty());
        // The extra data is removed first in any case.
        assert_eq!(args_of(&log, 0x0042_e8e0).len(), 1);
    }

    /// The world of the `008b6ae0` tests: one entry (part 4) whose bytes are
    /// the three flags, a creature-kind actor.
    fn create_stage(skip: u8, wants_removal: u8) -> (LimbStage, u32, u32) {
        let mut w = limb_stage();
        let record = w.s.e.mem.alloc(0x40);
        w.s.e.mem.set_u16(record + 0xc, 1);
        let entry = w.s.e.mem.alloc(0x40);
        w.s.e.mem.set_u8(entry, STAGE_PART_INDEX as u8);
        w.s.e.mem.set_u8(entry + 1, 9);
        w.s.e.mem.set_u8(entry + 2, skip);
        w.s.e.mem.set_u8(entry + 3, wants_removal);
        w.s.e.register(0x005d_43c0, |_, a| int(a[0]));
        w.s.e.register_double(0x0042_e8c0, move |_, _| int(record));
        w.s.e.register(0x0044_ddc0, |_, _| int(1));
        w.s.e.register_double(0x0044_1420, move |_, _| int(entry));
        w.s.e.register(0x0041_3f40, |_, a| int(a[0]));
        w.s.e.register(0x0096_11e0, |_, a| int(a[0] + 8));
        zeros(
            &mut w.s.e,
            &[
                ACTOR_GET_ANIMATION,
                0x0042_f570,
                0x0042_ff80,
                0x0043_87b0,
                0x0060_5fc0,
                0x0060_5e70,
                0x0057_3050,
            ],
        );
        (w, record, entry)
    }

    #[test]
    fn creating_dismembered_limbs_rebuilds_the_equipment_and_the_pieces() {
        let (mut w, _, entry) = create_stage(0, 1);
        let (actor, process) = (w.actor, w.process);
        let a = actor.addr();
        w.s.answer(a, 0x218, 1);
        w.s.answer(a, 0x1e8, 0x77);
        let form = w.s.e.call(0x0041_81e0, &args![a]).u32();
        let frame = next_frame(&mut w.s.e, 0x78);
        let (_, log) = logged(&mut w.s.e, |e| e.call(0x008b_6ae0, &args![actor]));
        // The 0x78-byte frame: the equipment at +0x24.
        assert_eq!(args_of(&log, 0x0042_f570), vec![vec![frame + 0x24]]);
        // The worn equipment is built for the comparison, then applied because they differ.
        let compared = args_of(&log, 0x0043_87b0);
        assert_eq!(compared.len(), 1);
        assert_eq!(compared[0][1], entry + 4);
        assert_eq!(
            args_of(&log, 0x0060_5fc0),
            vec![vec![form, a, 0x77, compared[0][0]]]
        );
        assert_eq!(
            args_of(&log, 0x0060_5e70),
            vec![vec![form, a, 0x77, entry + 4]]
        );
        // The piece is made (hide only: the entry's second byte), the removal is asked.
        assert_eq!(
            args_of(&log, 0x0057_3050),
            vec![vec![a, STAGE_PART_INDEX, 1]]
        );
        assert_eq!(args_of(&log, 0x00a5_a040).len(), 1);
        // The equipment changed: the targets are reloaded and the process told.
        assert!(w.slot_calls_on(process).contains(&(0x468, vec![1])));
        assert!(!args_of(&log, ACTOR_GET_ANIMATION).is_empty());
    }

    #[test]
    fn creating_dismembered_limbs_skips_the_equipment_for_the_skip_byte_and_non_creatures() {
        let (mut w, _, _) = create_stage(1, 0);
        let (actor, process) = (w.actor, w.process);
        w.s.answer(actor.addr(), 0x218, 1);
        let (_, log) = logged(&mut w.s.e, |e| e.call(0x008b_6ae0, &args![actor]));
        assert!(args_of(&log, 0x0060_5fc0).is_empty());
        assert!(args_of(&log, 0x0057_3050).is_empty());
        assert!(!w.slot_calls_on(process).iter().any(|call| call.0 == 0x468));
        let (mut w, _, _) = create_stage(0, 0);
        let actor = w.actor;
        let (_, log) = logged(&mut w.s.e, |e| e.call(0x008b_6ae0, &args![actor]));
        assert!(args_of(&log, 0x0060_5fc0).is_empty());
        assert_eq!(args_of(&log, 0x00a5_a040).len(), 1);
    }

    #[test]
    fn creating_dismembered_limbs_leaves_parts_whose_limb_exists() {
        let (mut w, _, _) = create_stage(0, 1);
        let actor = w.actor;
        w.s.e.register(0x009a_3830, |_, _| int(1));
        let (_, log) = logged(&mut w.s.e, |e| e.call(0x008b_6ae0, &args![actor]));
        assert!(args_of(&log, 0x00a5_a040).is_empty());
        assert!(args_of(&log, 0x0057_3050).is_empty());
        // A record without entries (word +0xc zero) does nothing at all.
        let (mut w, record, _) = create_stage(0, 1);
        let actor = w.actor;
        w.s.e.mem.set_u16(record + 0xc, 0);
        let (_, log) = logged(&mut w.s.e, |e| e.call(0x008b_6ae0, &args![actor]));
        assert!(args_of(&log, 0x0044_1420).is_empty());
    }

    #[test]
    fn removing_a_limb_from_the_world() {
        let mut w = limb_stage();
        let (actor, limb) = (w.actor, w.limb);
        w.s.e.register(0x0096_11e0, |_, a| int(a[0] + 8));
        let (removed, log) = logged(&mut w.s.e, |e| {
            e.call(0x008b_6df0, &args![actor, STAGE_PART_INDEX]).bool()
        });
        assert!(removed);
        assert_eq!(
            args_of(&log, 0x0057_3050),
            vec![vec![actor.addr(), STAGE_PART_INDEX, 1]]
        );
        assert_eq!(args_of(&log, 0x0045_0f90), vec![vec![limb + 8, 1]]);
        assert_eq!(args_of(&log, 0x00c6_9ee0), vec![vec![limb + 8, 1, 1]]);
        assert_eq!(
            args_of(&log, 0x0097_4290),
            vec![vec![PROCESS_LISTS_INSTANCE, limb]]
        );
        // Without a world node, or without a limb, nothing is removed.
        w.s.e.register(0x0096_11e0, |_, _| int(0));
        let (removed, _) = logged(&mut w.s.e, |e| {
            e.call(0x008b_6df0, &args![actor, STAGE_PART_INDEX]).bool()
        });
        assert!(!removed);
        let process = w.process;
        w.s.answer(process, 0x6d0, 0);
        let (removed, _) = logged(&mut w.s.e, |e| {
            e.call(0x008b_6df0, &args![actor, STAGE_PART_INDEX]).bool()
        });
        assert!(!removed);
    }

    // ---- 008b6e80 .. 008b7b70: bones, animation sets ---------------------

    /// An engine for the bone table and node tests: nodes are objects of a
    /// scene whose slots `0x18`, `0xc` and `0x10` are the ones used.
    fn bone_scene() -> Scene {
        let mut s = scene(&[0x18, 0xc, 0x10, 0x1b8]);
        s.e.map(0x011d_f000, 0x1000);
        zeros(
            &mut s.e,
            &[
                0x0043_fad0,
                0x0055_85e0,
                0x0080_41a0,
                0x0082_5c00,
                0x0043_b480,
                0x0043_b4a0,
            ],
        );
        s
    }

    #[test]
    fn skinned_nodes_are_found_in_the_bone_table() {
        let mut s = bone_scene();
        let actor = s.actor();
        let node = s.object(0x40);
        s.answer(node, 0x18, 1);
        s.e.register(0x0043_fad0, |_, _| int(0x5c1));
        s.e.register(0x0055_85e0, |_, _| int(0xb01));
        s.e.register(0x0080_41a0, |_, _| int(2));
        let list = s.e.mem.alloc(8);
        s.e.mem.set_u32(list, 0xb0aa);
        s.e.mem.set_u32(list + 4, 0xb0bb);
        s.e.register_double(0x0082_5c00, move |_, _| int(list));
        s.e.set_global::<u32>(LIMB_BONE_COUNT, 2);
        s.e.set_global::<u32>(LIMB_BONES, 0xb0cc);
        s.e.set_global::<u32>(LIMB_BONES + 4, 0xb0bb);
        assert!(s.e.call(0x008b_6e80, &args![actor, node, 3u32]).bool());
        // No match in the table.
        s.e.set_global::<u32>(LIMB_BONES + 4, 0xb0dd);
        assert!(!s.e.call(0x008b_6e80, &args![actor, node, 3u32]).bool());
        // A node without skin data, or a null node.
        s.e.register(0x0043_fad0, |_, _| int(0));
        assert!(!s.e.call(0x008b_6e80, &args![actor, node, 3u32]).bool());
        assert!(!s.e.call(0x008b_6e80, &args![actor, 0u32, 3u32]).bool());
    }

    #[test]
    fn skinned_children_count_for_their_parent() {
        let mut s = bone_scene();
        let actor = s.actor();
        let (parent, children, skinned_child, plain_child) = (
            s.object(0x40),
            s.object(0x40),
            s.object(0x40),
            s.object(0x40),
        );
        s.answer(parent, 0xc, children);
        s.answer(skinned_child, 0x18, 1);
        s.e.register_double(0x0043_b480, move |_, a| {
            int(if a[0] == children { 2 } else { 0 })
        });
        s.e.register_double(0x0043_b4a0, move |_, a| {
            int(if a[1] == 0 {
                plain_child
            } else {
                skinned_child
            })
        });
        s.e.register(0x0043_fad0, |_, _| int(0x5c1));
        s.e.register(0x0055_85e0, |_, _| int(0xb01));
        s.e.register(0x0080_41a0, |_, _| int(1));
        let list = s.e.mem.alloc(8);
        s.e.mem.set_u32(list, 0xb0aa);
        s.e.register_double(0x0082_5c00, move |_, _| int(list));
        s.e.set_global::<u32>(LIMB_BONE_COUNT, 1);
        s.e.set_global::<u32>(LIMB_BONES, 0xb0aa);
        assert!(s.e.call(0x008b_6e80, &args![actor, parent, 3u32]).bool());
        // Slot 0x10 stops the descent.
        s.answer(parent, 0x10, 1);
        assert!(!s.e.call(0x008b_6e80, &args![actor, parent, 3u32]).bool());
    }

    #[test]
    fn the_bone_table_collects_the_nodes_and_their_children() {
        let mut s = bone_scene();
        let actor = s.actor();
        let (node, child, other) = (s.object(0x40), s.object(0x40), s.object(0x40));
        s.answer(node, 0xc, 1);
        s.answer(child, 0xc, 1);
        s.e.register_double(0x0043_b480, move |_, a| int(u32::from(a[0] == node)));
        s.e.register_double(0x0043_b4a0, move |_, _| int(child));
        s.e.set_global::<u32>(LIMB_BONE_COUNT, 7);
        let (_, log) = logged(&mut s.e, |e| e.call(0x008b_6ff0, &args![actor, node, 0u32]));
        assert_eq!(args_of(&log, 0x0040_3d30), vec![vec![LIMB_BONES, 0, 0x400]]);
        assert_eq!(s.e.global::<u32>(LIMB_BONE_COUNT), 2);
        assert_eq!(s.e.global::<u32>(LIMB_BONES), node);
        assert_eq!(s.e.global::<u32>(LIMB_BONES + 4), child);
        // Kept: appended; a node without slot 0xc is not.
        s.e.call(0x008b_6ff0, &args![actor, other, 1u32]);
        assert_eq!(s.e.global::<u32>(LIMB_BONE_COUNT), 2);
        s.e.call(0x008b_6ff0, &args![actor, 0u32, 1u32]);
        assert_eq!(s.e.global::<u32>(LIMB_BONE_COUNT), 2);
    }

    #[test]
    fn a_full_bone_table_logs_a_message() {
        let mut s = bone_scene();
        let actor = s.actor();
        let node = s.object(0x40);
        s.answer(node, 0xc, 1);
        s.e.register(0x0055_d520, |_, _| int(0x4e4e));
        s.e.set_global::<u32>(LIMB_BONE_COUNT, 0x100);
        let (_, log) = logged(&mut s.e, |e| e.call(0x008b_6ff0, &args![actor, node, 1u32]));
        assert_eq!(
            args_of(&log, 0x005b_5e40),
            vec![vec![LIMB_BONES_MESSAGE, 0x4e4e, 0x100, 0x100]]
        );
        assert_eq!(s.e.global::<u32>(LIMB_BONE_COUNT), 0x100);
    }

    #[test]
    fn the_animation_of_an_actor_comes_from_its_process() {
        let mut s = bone_scene();
        let actor = s.actor();
        assert_eq!(s.e.call(0x008b_70d0, &args![actor]).u32(), 0);
        let process = s.object(0x40);
        s.answer(process, 0x1b8, 0xa11);
        s.e.set(actor, Actor::pCurrentProcess, Ptr::new(process));
        assert_eq!(s.e.call(0x008b_70d0, &args![actor]).u32(), 0xa11);
    }

    /// The scene of the `008b7100` tests: an actor with 3D and visible flags,
    /// a process answering slot `0x28c`, and doubles for the callees.
    fn position_scene() -> (Scene, Ptr<Actor>, u32) {
        let mut s = scene(&[0x1d0, 0x230, 0x22c, 0x28c, 0x1e4]);
        let actor = s.actor();
        let process = s.object(0x40);
        s.e.set(actor, Actor::pCurrentProcess, Ptr::new(process));
        s.e.set(actor, Actor::iVisFlags, 7);
        s.answer(actor.addr(), 0x1d0, 0x3d3d);
        let e = &mut s.e;
        e.map(0x011f_4000, 0x1000);
        for (word, value) in [1.5f32, 2.5, 3.5].into_iter().enumerate() {
            e.mem.set_f32(DEFAULT_LOCATION + 4 * word as u32, value);
        }
        e.register(ACTOR_GET_PROCESS, |e, a| int(e.mem.u32(a[0] + 0x68)));
        zeros(
            e,
            &[
                0x0094_df60,
                0x0043_fcd0,
                0x00c8_21b0,
                0x0043_0830,
                0x0068_15c0,
                0x004a_0c90,
                0x004b_3ae0,
                0x0050_d4a0,
                0x0049_4390,
            ],
        );
        (s, actor, process)
    }

    fn position_of(e: &mut Engine, actor: Ptr<Actor>) -> (bool, [f32; 3]) {
        let out = e.mem.alloc(0x10);
        let result = e.call(0x008b_7100, &args![actor, out]).bool();
        (
            result,
            [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)],
        )
    }

    #[test]
    fn position_defaults_without_3d_visibility_or_while_the_player_sleeps() {
        let (mut s, actor, _) = position_scene();
        s.answer(actor.addr(), 0x1d0, 0);
        assert_eq!(position_of(&mut s.e, actor), (false, [1.5, 2.5, 3.5]));
        s.answer(actor.addr(), 0x1d0, 0x3d3d);
        s.e.set(actor, Actor::iVisFlags, 5);
        assert_eq!(position_of(&mut s.e, actor), (false, [1.5, 2.5, 3.5]));
        s.e.set(actor, Actor::iVisFlags, 7);
        s.e.register(0x0094_df60, |_, _| int(1));
        assert_eq!(position_of(&mut s.e, actor), (false, [1.5, 2.5, 3.5]));
    }

    #[test]
    fn position_from_the_bip_transform_is_rotated_by_the_height() {
        let (mut s, actor, _) = position_scene();
        s.answer(actor.addr(), 0x230, 1);
        s.e.register(0x0043_fcd0, |_, _| int(0x7007));
        let place = s.e.mem.alloc(0x10);
        s.e.mem.set_f32(place + 8, 9.0);
        s.e.register_double(0x0043_0830, move |_, _| int(place));
        s.e.register(0x004b_3ae0, |e, a| {
            for word in 0..3 {
                e.mem.set_f32(a[0] + 4 * word, 10.0 + word as f32);
            }
            int(a[0])
        });
        let out = s.e.mem.alloc(0x10);
        let (result, log) = logged(&mut s.e, |e| e.call(0x008b_7100, &args![actor, out]).bool());
        assert!(result);
        // Slot 0x22c (0) answered false: the flag is 1.
        assert_eq!(args_of(&log, 0x00c8_21b0), vec![vec![0x7007, out, 1]]);
        assert_eq!(args_of(&log, 0x004a_0c90)[0][1], 9.0f32.to_bits());
        assert_eq!(
            [s.e.mem.f32(out), s.e.mem.f32(out + 4), s.e.mem.f32(out + 8)],
            [10.0, 11.0, 12.0]
        );
        // Slot 0x22c true instead: the flag is 0.
        s.answer(actor.addr(), 0x230, 0);
        s.answer(actor.addr(), 0x22c, 1);
        let (_, log) = logged(&mut s.e, |e| e.call(0x008b_7100, &args![actor, out]));
        assert_eq!(args_of(&log, 0x00c8_21b0), vec![vec![0x7007, out, 0]]);
    }

    #[test]
    fn position_without_the_blocking_state_comes_from_the_1e4_object() {
        let (mut s, actor, process) = position_scene();
        let controller = s.e.mem.alloc(0x500);
        s.answer(process, 0x28c, controller);
        // A state that is set and non-empty gives the default location.
        s.e.mem.set_u8(controller + 0x410 + 0x69, 1);
        s.e.mem.set_u32(controller + 0x410 + 0x6c, 3);
        assert_eq!(position_of(&mut s.e, actor), (true, [1.5, 2.5, 3.5]));
        s.e.mem.set_u32(controller + 0x410 + 0x6c, 0);
        // No slot 0x1e4 object: default again.
        assert_eq!(position_of(&mut s.e, actor), (true, [1.5, 2.5, 3.5]));
        // With the object, 00494390 fills the position: the two flags depend on 0050d4a0.
        let object = s.object(0x40);
        s.answer(actor.addr(), 0x1e4, object);
        s.e.register(0x0049_4390, |e, a| {
            e.mem.set_f32(a[1], 7.0);
            Ret::default()
        });
        let out = s.e.mem.alloc(0x10);
        let (result, log) = logged(&mut s.e, |e| e.call(0x008b_7100, &args![actor, out]).bool());
        assert!(result);
        assert_eq!(
            args_of(&log, 0x0049_4390),
            vec![vec![object, out, actor.addr(), 0, 1]]
        );
        assert_eq!(s.e.mem.f32(out), 7.0);
        s.e.register(0x0050_d4a0, |_, _| int(1));
        let (_, log) = logged(&mut s.e, |e| e.call(0x008b_7100, &args![actor, out]));
        assert_eq!(
            args_of(&log, 0x0049_4390),
            vec![vec![object, out, actor.addr(), 1, 0]]
        );
    }

    #[test]
    fn controller_state_of_008b7320_needs_the_byte_and_the_word() {
        let (mut e, _) = engine();
        let state = e.mem.alloc(0x100);
        assert!(!e.call(0x008b_7320, &args![state]).bool());
        e.mem.set_u8(state + 0x69, 1);
        assert!(!e.call(0x008b_7320, &args![state]).bool());
        e.mem.set_u32(state + 0x6c, 4);
        assert!(e.call(0x008b_7320, &args![state]).bool());
        e.mem.set_u8(state + 0x69, 0);
        assert!(!e.call(0x008b_7320, &args![state]).bool());
    }

    /// Exe strings the animation code compares with or builds from.
    const ANIMATION_STRINGS: [(u32, &str); 9] = [
        (0x0101_7114, "\\Locomotion\\"),
        (0x0101_70f8, "Hurt\\"),
        (0x0108_4f20, "Toddler\\"),
        (0x0101_70ec, "IdleAnims"),
        (0x0108_4f0c, "\\Locomotion\\Child\\"),
        (0x0108_4ef8, "\\Locomotion\\Female\\"),
        (0x0108_4ee4, "\\Locomotion\\Male\\"),
        (0x0108_4f34, "Child\\"),
        (0x0108_4f2c, "Female\\"),
    ];

    /// The text of the C string at `at`.
    fn text_at(e: &Engine, at: u32) -> String {
        let mut bytes = vec![];
        while e.mem.u8(at + bytes.len() as u32) != 0 {
            bytes.push(e.mem.u8(at + bytes.len() as u32));
        }
        String::from_utf8(bytes).unwrap()
    }

    /// The `(path, [loader, a, b, options])` of every list built.
    type PathLog = Rc<std::cell::RefCell<Vec<(String, Vec<u32>)>>>;

    /// The scene of the animation-list tests: string routines work on memory,
    /// `00447330` records the path it is given and returns a list when
    /// `lists` says so.
    struct Animations {
        s: Scene,
        actor: Ptr<Actor>,
        process: u32,
        paths: PathLog,
    }

    fn animations() -> Animations {
        let mut s = scene(&[0x148, 0x390, 0x1a0, 0xc, 0x1b8, 0x1d0, 0x10]);
        let actor = s.actor();
        let process = s.object(0x40);
        s.e.set(actor, Actor::pCurrentProcess, Ptr::new(process));
        let e = &mut s.e;
        for page in [0x0101_7000, 0x0108_4000, 0x0118_a000, 0x011c_3000] {
            e.map(page, 0x1000);
        }
        for (at, text) in ANIMATION_STRINGS {
            e.mem.write(at, text.as_bytes());
        }
        e.set_global::<u32>(MODEL_LOADER, 0x10ad);
        e.register(ACTOR_GET_PROCESS, |e, a| int(e.mem.u32(a[0] + 0x68)));
        e.register(0x0040_6d30, |e, a| {
            let text = text_at(e, a[2]);
            e.mem.write(a[0], text.as_bytes());
            e.mem.set_u8(a[0] + text.len() as u32, 0);
            int(a[0])
        });
        e.register(0x0040_6d50, |e, a| {
            let (head, tail) = (text_at(e, a[0]), text_at(e, a[2]));
            let at = a[0] + head.len() as u32;
            e.mem.write(at, tail.as_bytes());
            e.mem.set_u8(at + tail.len() as u32, 0);
            int(a[0])
        });
        e.register(0x0040_ab30, |e, a| {
            let text = text_at(e, a[0]);
            int(text
                .rfind(a[1] as u8 as char)
                .map_or(0, |at| a[0] + at as u32))
        });
        e.register(0x0057_15d0, |e, _| {
            let path = cstr(e, "Meshes\\Chars\\skel.nif");
            int(path)
        });
        let paths: PathLog = Default::default();
        let log = paths.clone();
        e.register_double(0x0044_7330, move |e, a| {
            log.borrow_mut()
                .push((text_at(e, a[1]), vec![a[0], a[2], a[3], a[4]]));
            int(0x5757)
        });
        zeros(
            e,
            &[
                0x0044_5430,
                0x0044_65f0,
                0x0044_6500,
                0x0047_02f0,
                0x0043_cc60,
                0x0083_97d0,
                0x0087_f4c0,
                0x008a_6970,
                0x0045_cd60,
                0x0044_ddc0,
                0x0044_6390,
                0x0049_7280,
                0x0049_c390,
                0x0041_3f40,
                0x0043_b1b0,
                0x00ec_7750,
                0x0040_3df0,
                0x0046_4f30,
            ],
        );
        e.register(PLAYER_GET_ANIMATION, |_, a| {
            int(if a[1] == 0 { 0xa100 } else { 0xa200 })
        });
        Animations {
            s,
            actor,
            process,
            paths,
        }
    }

    impl Animations {
        fn load(
            &mut self,
            hurt: u8,
            animation: u32,
            path: u32,
            replacement: u32,
        ) -> Vec<(u32, Vec<u32>)> {
            let actor = self.actor;
            logged(&mut self.s.e, |e| {
                e.call(
                    0x008b_73f0,
                    &args![actor, hurt, animation, path, replacement],
                )
            })
            .1
        }
    }

    #[test]
    fn locomotion_lists_use_the_model_path_and_replace_the_animation() {
        let mut w = animations();
        let log = w.load(0, 0xa1, 0, 0);
        assert_eq!(
            w.paths.borrow()[0],
            (
                "Meshes\\Chars\\Locomotion\\IdleAnims".to_string(),
                vec![0x10ad, 0, 1, 0]
            )
        );
        // The old list is cancelled and the new one installed for the animation, then released.
        assert_eq!(args_of(&log, 0x0044_5430), vec![vec![0x10ad, 0xa1]]);
        assert_eq!(
            args_of(&log, 0x0044_65f0),
            vec![vec![0x10ad, 0x5757, 0xa1, 5, 0, 0]]
        );
        assert_eq!(args_of(&log, 0x0047_02f0), vec![vec![0x5757, 1]]);
        // Slot 0x390 is false: no second list.
        assert_eq!(w.paths.borrow().len(), 1);
    }

    #[test]
    fn locomotion_lists_for_hurt_actors_and_given_model_paths() {
        let mut w = animations();
        let path = cstr(&mut w.s.e, "Meshes\\Other\\body.nif");
        w.load(1, 0xa1, path, 0);
        assert_eq!(
            w.paths.borrow()[0].0,
            "Meshes\\Other\\Locomotion\\Hurt\\IdleAnims"
        );
        // Only one list for hurt actors, even with slot 0x390.
        w.s.answer(w.actor.addr(), 0x390, 1);
        w.paths.borrow_mut().clear();
        w.load(1, 0xa1, path, 0);
        assert_eq!(w.paths.borrow().len(), 1);
    }

    #[test]
    fn locomotion_lists_add_the_child_female_or_male_set() {
        let mut w = animations();
        w.s.answer(w.actor.addr(), 0x390, 1);
        w.load(0, 0xa1, 0, 0);
        assert_eq!(
            w.paths.borrow()[1].0,
            "Meshes\\Chars\\Locomotion\\Male\\IdleAnims"
        );
        assert_eq!(w.paths.borrow()[1].1, vec![0x10ad, 0, 0, 0]);
        w.s.e.register(0x0087_f4c0, |_, _| int(1));
        w.paths.borrow_mut().clear();
        w.load(0, 0xa1, 0, 0);
        assert_eq!(
            w.paths.borrow()[1].0,
            "Meshes\\Chars\\Locomotion\\Female\\IdleAnims"
        );
        w.s.answer(w.actor.addr(), 0x1a0, 1);
        w.paths.borrow_mut().clear();
        w.load(0, 0xa1, 0, 0);
        assert_eq!(
            w.paths.borrow()[1].0,
            "Meshes\\Chars\\Locomotion\\Child\\IdleAnims"
        );
        // The second list does not cancel the first one's replacement again.
        let log = w.load(0, 0xa1, 0, 0);
        assert_eq!(args_of(&log, 0x0044_5430).len(), 1);
        assert_eq!(args_of(&log, 0x0044_65f0).len(), 2);
    }

    #[test]
    fn locomotion_lists_fill_a_replacement_when_given() {
        let mut w = animations();
        w.s.e.register(0x0043_cc60, |_, _| int(0xc1c1));
        let log = w.load(0, 0xa1, 0, 0x7e7e);
        assert_eq!(
            args_of(&log, 0x0044_6500),
            vec![vec![0x10ad, 0x5757, 0xc1c1, 0x7e7e, 0]]
        );
        assert!(args_of(&log, 0x0044_5430).is_empty());
        // Without an animation or a replacement nothing is built; the actor's own is used.
        let log = w.load(0, 0, 0, 0);
        assert!(args_of(&log, 0x0040_6d30).is_empty());
        let process = w.process;
        w.s.answer(process, 0x1b8, 0xa77);
        let log = w.load(0, 0, 0, 0);
        assert_eq!(
            args_of(&log, 0x0044_65f0),
            vec![vec![0x10ad, 0x5757, 0xa77, 5, 0, 0]]
        );
        // A path without a backslash ends it.
        let flat = cstr(&mut w.s.e, "flat.nif");
        let log = w.load(0, 0xa1, flat, 0);
        assert!(args_of(&log, 0x0044_7330).is_empty());
    }

    #[test]
    fn locomotion_options_come_from_the_process_or_the_player() {
        let mut w = animations();
        let process = w.process;
        // A process whose kind (0045cd60) is 0 or 1 and whose slot 0x148 gives a source.
        w.s.e.register(0x0045_cd60, |_, _| int(1));
        let source = w.s.object(0x40);
        w.s.answer(process, 0x148, source);
        w.s.e.register(0x0044_ddc0, |_, _| int(0x44));
        w.s.e.register(0x0044_6390, |_, _| int(3));
        w.s.e.set_global::<u32>(0x0118_a838 + 12, 0x0d0d);
        w.load(0, 0xa1, 0, 0);
        assert_eq!(w.paths.borrow()[0].1, vec![0x10ad, 0, 1, 0x0d0d]);
        // No source: option 1 when 008a6970 says so.
        w.s.answer(process, 0x148, 0);
        w.s.e.register(0x008a_6970, |_, _| int(1));
        w.load(0, 0xa1, 0, 0);
        assert_eq!(w.paths.borrow()[1].1[3], 1);
        // A process kind outside 0..=1: 0.
        w.s.e.register(0x0045_cd60, |_, _| int(2));
        w.load(0, 0xa1, 0, 0);
        assert_eq!(w.paths.borrow()[2].1[3], 0);
        // The player: 0xc, and the Toddler set with 008397d0 and a model path.
        let player = w.s.e.global::<u32>(PLAYER_CHARACTER);
        let vtable = w.s.vtable;
        w.s.e.mem.set_u32(player, vtable);
        w.s.e.register(0x0083_97d0, |_, _| int(1));
        let path = cstr(&mut w.s.e, "Meshes\\Chars\\skel.nif");
        w.paths.borrow_mut().clear();
        w.s.e
            .call(0x008b_73f0, &args![player, 0u8, 0xa1u32, path, 0u32]);
        assert_eq!(
            w.paths.borrow()[0],
            (
                "Meshes\\Chars\\Locomotion\\Toddler\\IdleAnims".to_string(),
                vec![0x10ad, 0, 1, 0xc]
            )
        );
    }

    #[test]
    fn the_players_locomotion_is_reloaded_for_both_animations() {
        let mut w = animations();
        let player = w.s.e.global::<u32>(PLAYER_CHARACTER);
        w.s.e.register(0x0040_3df0, |_, _| int(0x00a0));
        w.s.e.register(0x0046_4f30, |e, _| {
            let path = cstr(e, "Meshes\\Fp\\arms.nif");
            int(path)
        });
        let (_, log) = logged(&mut w.s.e, |e| {
            e.call(0x008b_7360, &args![player, 1u32, 0u32])
        });
        let paths = w.paths.borrow().clone();
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0].0, "Meshes\\Chars\\Locomotion\\Hurt\\IdleAnims");
        assert_eq!(paths[1].0, "Meshes\\Fp\\Locomotion\\Hurt\\IdleAnims");
        assert_eq!(
            args_of(&log, PLAYER_GET_ANIMATION),
            vec![vec![player, 0], vec![player, 1]]
        );
        assert_eq!(args_of(&log, 0x0040_3df0), vec![vec![0x011c_dd78]]);
        // Another actor: its own animation, one list.
        w.paths.borrow_mut().clear();
        let process = w.process;
        w.s.answer(process, 0x1b8, 0xa77);
        let actor = w.actor;
        let (_, log) = logged(&mut w.s.e, |e| {
            e.call(0x008b_7360, &args![actor, 0u32, 0u32])
        });
        assert_eq!(w.paths.borrow().len(), 1);
        assert_eq!(args_of(&log, 0x0044_65f0)[0][2], 0xa77);
    }

    /// The scene of `008b78c0`: the animation's sequence name is `name`; the
    /// actor values (magic target slot `0xc`) are 0 except those in `values`.
    fn reload_scene(name: &str, values: &'static [(u32, f64)]) -> Animations {
        let mut w = animations();
        let process = w.process;
        w.s.answer(process, 0x1b8, 0xa77);
        let sequences = w.s.object(0x40);
        let sequence_holder = w.s.object(0x40);
        let text = cstr(&mut w.s.e, name);
        w.s.e.register(0x0049_7280, |_, _| int(0x5eb));
        w.s.e.register_double(0x0049_c390, move |e, a| {
            e.mem.set_u32(a[2], sequences);
            int(1)
        });
        w.s.answer(sequences, 0x10, sequence_holder);
        w.s.e.register(0x0041_3f40, |_, a| int(a[0]));
        w.s.e.register_double(0x0043_b1b0, move |_, _| int(text));
        w.s.e.register(0x00ec_7750, |e, a| {
            let (haystack, needle) = (text_at(e, a[0]), text_at(e, a[1]));
            int(haystack.find(&needle).map_or(0, |at| a[0] + at as u32))
        });
        w.s.e.register_double(0x0f30_0000 + 0xc, move |_, a| {
            float(values.iter().find(|v| v.0 == a[1]).map_or(0.0, |v| v.1))
        });
        w
    }

    #[test]
    fn the_locomotion_is_reloaded_when_the_wanted_kinds_differ() {
        // Not hurt, but the loaded set has "Hurt\": reload.
        let mut w = reload_scene(
            "Meshes\\Locomotion\\Hurt\\IdleAnims",
            &[(0x1d, 1.0), (0x1e, 1.0)],
        );
        let actor = w.actor;
        w.s.e.call(0x008b_78c0, &args![actor, 0u32]);
        assert_eq!(w.paths.borrow().len(), 1);
        // The state matches the loaded set (hurt: 0x1d is zero and 0x48 is zero): left alone.
        let mut w = reload_scene("Meshes\\Locomotion\\Hurt\\IdleAnims", &[]);
        let actor = w.actor;
        w.s.e.call(0x008b_78c0, &args![actor, 0u32]);
        assert!(w.paths.borrow().is_empty());
        // `force` reloads anyway.
        w.s.e.call(0x008b_78c0, &args![actor, 1u32]);
        assert_eq!(w.paths.borrow().len(), 1);
        // A child set that matches a child actor (not hurt on both sides): left alone.
        let mut w = reload_scene("x\\Child\\Female\\y", &[(0x1d, 1.0), (0x1e, 1.0)]);
        let actor = w.actor;
        w.s.answer(actor.addr(), 0x1a0, 1);
        w.s.e.register(0x0087_f4c0, |_, _| int(1));
        w.s.e.call(0x008b_78c0, &args![actor, 0u32]);
        assert!(w.paths.borrow().is_empty());
        // The same set for an actor that is not a child: reload.
        w.s.answer(actor.addr(), 0x1a0, 0);
        w.s.e.call(0x008b_78c0, &args![actor, 0u32]);
        assert_eq!(w.paths.borrow().len(), 1);
    }

    #[test]
    fn the_locomotion_reload_needs_the_animation_sequence_and_name() {
        let mut w = reload_scene("x", &[]);
        let actor = w.actor;
        let process = w.process;
        w.s.answer(process, 0x1b8, 0);
        w.s.e.call(0x008b_78c0, &args![actor, 1u32]);
        assert!(w.paths.borrow().is_empty());
        w.s.answer(process, 0x1b8, 0xa77);
        w.s.e.register(0x0049_7280, |_, _| int(0));
        w.s.e.call(0x008b_78c0, &args![actor, 1u32]);
        assert!(w.paths.borrow().is_empty());
        w.s.e.register(0x0049_7280, |_, _| int(0x5eb));
        w.s.e.register(0x0049_c390, |_, _| int(0));
        w.s.e.call(0x008b_78c0, &args![actor, 1u32]);
        assert!(w.paths.borrow().is_empty());
        // A sequence that is 0, or one without a name, ends it too.
        let object = w.s.object(0x40);
        w.s.e.register_double(0x0049_c390, move |e, a| {
            e.mem.set_u32(a[2], object);
            int(1)
        });
        w.s.e.call(0x008b_78c0, &args![actor, 1u32]);
        assert!(w.paths.borrow().is_empty());
        w.s.answer(object, 0x10, 0x5e9);
        w.s.e.register(0x0043_b1b0, |_, _| int(0));
        w.s.e.call(0x008b_78c0, &args![actor, 1u32]);
        assert!(w.paths.borrow().is_empty());
        // The player needs both animations.
        let mut w = reload_scene("x", &[]);
        let player = w.s.e.global::<u32>(PLAYER_CHARACTER);
        w.s.e.register(PLAYER_GET_ANIMATION, |_, a| {
            int(if a[1] == 0 { 0xa100 } else { 0 })
        });
        w.s.e.call(0x008b_78c0, &args![player, 1u32]);
        assert!(w.paths.borrow().is_empty());
    }

    #[test]
    fn hurt_is_both_pairs_of_actor_values_being_zero_in_the_right_way() {
        let mut s = scene(&[0xc]);
        let actor = s.actor();
        s.e.map(0x0101_2000, 0x1000);
        let table: Rc<std::cell::RefCell<std::collections::HashMap<u32, f64>>> = Default::default();
        let values = table.clone();
        s.e.register_double(0x0f30_0000 + 0xc, move |_, a| {
            float(values.borrow().get(&a[1]).copied().unwrap_or(0.0))
        });
        let hurt = |s: &mut Scene, one: f64, two: f64, three: f64| {
            *table.borrow_mut() = [(0x1d, one), (0x1e, two), (0x48, three)]
                .into_iter()
                .collect();
            s.e.call(0x008b_7b70, &args![actor]).bool()
        };
        assert!(hurt(&mut s, 0.0, 5.0, 0.0));
        assert!(hurt(&mut s, 2.0, 0.0, 0.0));
        assert!(!hurt(&mut s, 2.0, 3.0, 0.0));
        assert!(!hurt(&mut s, 0.0, 0.0, 1.0));
        // The second value is only asked when the first is not zero.
        s.e.call_log = Some(vec![]);
        hurt(&mut s, 0.0, 3.0, 0.0);
        let log = s.e.call_log.take().unwrap();
        let slots: Vec<u32> = log
            .iter()
            .map(|call| call.1.get(1).copied().unwrap_or(0))
            .collect();
        assert_eq!(slots, vec![0, 0x1d, 0x48]);
    }

    // ---- 008b7b70 .. 008b8e20: factions -----------------------------------

    /// The scene of the faction tests: an actor keeps the head of its form's
    /// faction chain at `+0x30` and its extra faction changes (a block whose
    /// `+0xc` is another chain head) at `+0x38`; chain nodes are `[entry,
    /// next]` and entries `[faction, rank byte at +4]`.
    fn faction_stage() -> Scene {
        let mut s = scene(&[0x48]);
        let e = &mut s.e;
        e.register(0x0041_81e0, |_, a| int(a[0]));
        e.register(0x005d_8a70, |e, a| int(e.mem.u32(a[0])));
        e.register(0x005d_43c0, |_, a| int(a[0]));
        e.register(0x0042_e800, |e, a| int(e.mem.u32(a[0] + 0x38)));
        e.register(0x0068_15c0, |_, a| int(a[0]));
        e.register(0x0072_6070, |e, a| int(e.mem.u32(a[0] + 4)));
        zeros(e, &[0x0084_e3a0, 0x0055_d520, 0x005b_5e40]);
        s
    }

    /// A chain of `(faction, rank)` entries; returns the first node.
    fn faction_chain(e: &mut Engine, entries: &[(u32, i8)]) -> u32 {
        let mut next = 0;
        for &(faction, rank) in entries.iter().rev() {
            let entry = e.mem.alloc(8);
            e.mem.set_u32(entry, faction);
            e.mem.set_u8(entry + 4, rank as u8);
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, entry);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    /// Gives `actor` the factions `base` and, when given, the extra `changes`.
    fn give_factions(
        s: &mut Scene,
        actor: Ptr<Actor>,
        base: &[(u32, i8)],
        changes: Option<&[(u32, i8)]>,
    ) {
        let head = faction_chain(&mut s.e, base);
        s.e.mem.set_u32(actor.addr() + 0x30, head);
        let block = match changes {
            Some(extra) => {
                let chain = faction_chain(&mut s.e, extra);
                let block = s.e.mem.alloc(0x20);
                s.e.mem.set_u32(block + 0xc, chain);
                block
            }
            None => 0,
        };
        s.e.mem.set_u32(actor.addr() + 0x38, block);
    }

    /// A faction object with the word at `+0x34` set to `flags`.
    fn faction_with(e: &mut Engine, flags: u32) -> u32 {
        let faction = e.mem.alloc(0x80);
        e.mem.set_u32(faction + 0x34, flags);
        faction
    }

    #[test]
    fn faction_lists_merge_the_base_and_the_changes() {
        let mut s = faction_stage();
        let actor = s.actor();
        give_factions(
            &mut s,
            actor,
            &[(0xf1, 0), (0xf2, 5), (0xf3, -1)],
            Some(&[(0xf2, -1), (0xf4, 3), (0xf5, -1), (0xf1, 2)]),
        );
        let head = s.e.mem.u32(actor.addr() + 0x30);
        let block = s.e.mem.u32(actor.addr() + 0x38);
        let extra = s.e.mem.u32(block + 0xc);
        let buffer = s.e.mem.alloc(0x200);
        let count =
            s.e.call(0x008b_8ca0, &args![actor, buffer, 0x80u32, head, extra])
                .u32();
        // 0xf3 (rank -1) is left out, 0xf2 is removed by the changes, 0xf4 is added,
        // 0xf5 (rank -1, not present) is not, and 0xf1 stays.
        assert_eq!(count, 3);
        assert_eq!(
            [
                s.e.mem.u32(buffer),
                s.e.mem.u32(buffer + 4),
                s.e.mem.u32(buffer + 8)
            ],
            [0xf1, 0, 0xf4]
        );
        // Without the second chain only the first one counts.
        let count =
            s.e.call(0x008b_8ca0, &args![actor, buffer, 0x80u32, head, 0u32])
                .u32();
        assert_eq!(count, 2);
        assert_eq!(
            s.e.call(0x008b_8ca0, &args![actor, buffer, 0x80u32, 0u32, 0u32])
                .u32(),
            0
        );
    }

    #[test]
    fn faction_lists_stop_at_the_capacity_with_a_message() {
        let mut s = faction_stage();
        let actor = s.actor();
        s.e.register(0x0084_e3a0, |_, _| int(0x1d));
        s.e.register(0x0055_d520, |_, _| int(0x4e4e));
        give_factions(
            &mut s,
            actor,
            &[(0xf1, 0), (0xf2, 0), (0xf3, 0)],
            Some(&[(0xf4, 0)]),
        );
        let head = s.e.mem.u32(actor.addr() + 0x30);
        let block = s.e.mem.u32(actor.addr() + 0x38);
        let extra = s.e.mem.u32(block + 0xc);
        let buffer = s.e.mem.alloc(0x200);
        let (count, log) = logged(&mut s.e, |e| {
            e.call(0x008b_8ca0, &args![actor, buffer, 2u32, head, extra])
                .u32()
        });
        assert_eq!(count, 2);
        assert_eq!(
            args_of(&log, 0x005b_5e40),
            vec![vec![3, INTEGRATE_OVERFLOW_MESSAGE, 2, 0x4e4e, 0x1d]]
        );
        // The second chain can overflow too.
        let (count, log) = logged(&mut s.e, |e| {
            e.call(0x008b_8ca0, &args![actor, buffer, 2u32, 0u32, head])
                .u32()
        });
        assert_eq!(count, 2);
        assert_eq!(args_of(&log, 0x005b_5e40).len(), 1);
    }

    #[test]
    fn crime_flag_of_a_faction() {
        let (mut e, _) = engine();
        let faction = e.mem.alloc(0x80);
        assert!(!e.call(0x008b_7d00, &args![faction]).bool());
        e.mem.set_u32(faction + 0x34, 0x100);
        assert!(e.call(0x008b_7d00, &args![faction]).bool());
        e.mem.set_u32(faction + 0x34, 0xfeff);
        assert!(!e.call(0x008b_7d00, &args![faction]).bool());
    }

    #[test]
    fn crimes_are_added_to_the_factions_that_care() {
        let mut s = faction_stage();
        let actor = s.actor();
        let (caring, indifferent) = (faction_with(&mut s.e, 0x100), faction_with(&mut s.e, 0));
        give_factions(&mut s, actor, &[(caring, 0), (indifferent, 0)], None);
        zeros(&mut s.e, &[0x005f_da00, 0x005f_da50]);
        let (_, log) = logged(&mut s.e, |e| {
            e.call(0x008b_7c00, &args![actor, 0x55u32, 9u32])
        });
        assert_eq!(args_of(&log, 0x005f_da00), vec![vec![caring, 0x55, 9]]);
        assert!(args_of(&log, 0x005f_da50).is_empty());
        let (_, log) = logged(&mut s.e, |e| {
            e.call(0x008b_7d20, &args![actor, 0x66u32, 7u32])
        });
        assert_eq!(args_of(&log, 0x005f_da50), vec![vec![caring, 0x66, 7]]);
        assert!(args_of(&log, 0x005f_da00).is_empty());
    }

    #[test]
    fn faction_sums_of_008b7e20_and_008b7f00() {
        let mut s = faction_stage();
        let actor = s.actor();
        give_factions(&mut s, actor, &[(3, 0), (4, 0), (5, -1)], Some(&[(6, 1)]));
        s.e.register(0x0067_33e0, |_, a| int(a[0] * 10));
        s.e.register(0x0080_41a0, |_, a| int(a[0] + 100));
        assert_eq!(s.e.call(0x008b_7e20, &args![actor]).i32(), 30 + 40 + 60);
        assert_eq!(s.e.call(0x008b_7f00, &args![actor]).i32(), 103 + 104 + 106);
    }

    /// Two actors with the given base factions and a relation table for `0048bf50`.
    fn relation_scene(
        first: &[(u32, i8)],
        second: &[(u32, i8)],
        relations: &'static [((u32, u32), i32)],
    ) -> (Scene, Ptr<Actor>, Ptr<Actor>) {
        let mut s = faction_stage();
        let (a, b) = (s.actor(), s.actor());
        give_factions(&mut s, a, first, None);
        give_factions(&mut s, b, second, None);
        s.e.register(0x0048_bf50, |_, args| {
            let key = (args[0] - 0x24, args[1]);
            int(RELATIONS.with(|table| {
                table
                    .borrow()
                    .iter()
                    .find(|entry| entry.0 == key)
                    .map_or(0, |entry| entry.1)
            }) as u32)
        });
        RELATIONS.with(|table| *table.borrow_mut() = relations);
        (s, a, b)
    }

    thread_local! {
        static RELATIONS: std::cell::RefCell<&'static [((u32, u32), i32)]> =
            const { std::cell::RefCell::new(&[]) };
    }

    #[test]
    fn the_best_relation_between_two_actors_prefers_shared_factions() {
        let (mut s, a, b) = relation_scene(
            &[(0x100, 0), (0x200, 0)],
            &[(0x200, 0), (0x300, 0)],
            &[
                ((0x100, 0x200), 7),
                ((0x100, 0x300), 3),
                ((0x200, 0x200), 10),
                ((0x200, 0x300), 99),
            ],
        );
        let out = s.e.mem.alloc(0x10);
        let result =
            s.e.call(0x008b_7fe0, &args![a, b, out, out + 4, out + 8])
                .i32();
        // The lowest relation of unshared pairs is overridden by the shared faction.
        assert_eq!(result, 10);
        assert_eq!(
            [s.e.mem.u32(out), s.e.mem.u32(out + 4), s.e.mem.u32(out + 8)],
            [1, 0x200, 0x200]
        );
    }

    #[test]
    fn the_lowest_relation_between_unshared_factions_is_taken() {
        let (mut s, a, b) = relation_scene(
            &[(0x100, 0), (0x400, 0)],
            &[(0x200, 0), (0x300, 0)],
            &[
                ((0x100, 0x200), 7),
                ((0x100, 0x300), 3),
                ((0x400, 0x200), 1),
                ((0x400, 0x300), 5),
            ],
        );
        let out = s.e.mem.alloc(0x10);
        let result =
            s.e.call(0x008b_7fe0, &args![a, b, out, out + 4, out + 8])
                .i32();
        assert_eq!(result, 1);
        assert_eq!(
            [s.e.mem.u32(out), s.e.mem.u32(out + 4), s.e.mem.u32(out + 8)],
            [1, 0x400, 0x200]
        );
        // No factions: nothing is written and the result is 0.
        let (mut s, a, b) = relation_scene(&[], &[], &[]);
        let out = s.e.mem.alloc(0x10);
        assert_eq!(
            s.e.call(0x008b_7fe0, &args![a, b, out, out + 4, out + 8])
                .i32(),
            0
        );
        assert_eq!(s.e.mem.u32(out), 0);
    }

    #[test]
    fn the_rank_of_a_faction_is_one_when_the_actor_has_it() {
        let mut s = faction_stage();
        let actor = s.actor();
        give_factions(&mut s, actor, &[(3, 0), (4, 0)], Some(&[(5, 2), (4, -1)]));
        assert_eq!(s.e.call(0x008b_8290, &args![actor, 3u32, 0u32]).i32(), 1);
        assert_eq!(s.e.call(0x008b_8290, &args![actor, 5u32, 0u32]).i32(), 1);
        assert_eq!(s.e.call(0x008b_8290, &args![actor, 4u32, 0u32]).i32(), -1);
        assert_eq!(s.e.call(0x008b_8290, &args![actor, 9u32, 0u32]).i32(), -1);
    }

    #[test]
    fn factions_that_care_about_crime_are_recorded() {
        let mut s = faction_stage();
        let actor = s.actor();
        let (caring, indifferent, absent) = (
            faction_with(&mut s.e, 0x100),
            faction_with(&mut s.e, 0),
            faction_with(&mut s.e, 0x100),
        );
        give_factions(
            &mut s,
            actor,
            &[(caring, 0), (indifferent, 0), (absent, 0)],
            None,
        );
        zeros(&mut s.e, &[0x0047_eb90, 0x009e_bae0]);
        // The other object is in the `caring` faction only.
        s.e.register_double(0x008b_8e90, move |_, a| int(u32::from(a[1] == caring)));
        let (result, log) = logged(&mut s.e, |e| {
            e.call(0x008b_8360, &args![actor, 0x0b0b_u32, 0x0c0cu32])
                .u8()
        });
        assert_eq!(result, 1);
        assert_eq!(args_of(&log, 0x0047_eb90), vec![vec![caring, 1]]);
        assert_eq!(args_of(&log, 0x009e_bae0), vec![vec![0x0c0c, caring]]);
        assert_eq!(args_of(&log, 0x008b_8e90).len(), 2);
        s.e.register(0x008b_8e90, |_, _| int(0));
        assert_eq!(s.e.call(0x008b_8360, &args![actor, 1u32, 2u32]).u8(), 0);
    }

    #[test]
    fn an_actor_has_a_faction_that_cares_about_crime() {
        let mut s = faction_stage();
        let actor = s.actor();
        let indifferent = faction_with(&mut s.e, 0);
        give_factions(&mut s, actor, &[(indifferent, 0)], None);
        assert_eq!(s.e.call(0x008b_8490, &args![actor]).u8(), 0);
        let caring = faction_with(&mut s.e, 0x100);
        give_factions(&mut s, actor, &[(indifferent, 0), (caring, 0)], None);
        assert_eq!(s.e.call(0x008b_8490, &args![actor]).u8(), 1);
    }

    #[test]
    fn setting_a_faction_rank_changes_removes_or_adds() {
        let mut s = faction_stage();
        let actor = s.actor();
        let a = actor.addr();
        give_factions(&mut s, actor, &[(3, 0)], Some(&[(4, 1)]));
        let changes = s.e.mem.u32(a + 0x38);
        zeros(
            &mut s.e,
            &[0x0043_6e10, 0x0043_6f20, 0x0043_6f80, 0x0042_e760],
        );
        // A faction the actor has: removed with rank -1, else set.
        let (_, log) = logged(&mut s.e, |e| {
            e.call(0x008b_8580, &args![actor, 3u32, 0xffu8])
        });
        assert_eq!(args_of(&log, 0x0043_6e10), vec![vec![changes, 3]]);
        let (_, log) = logged(&mut s.e, |e| e.call(0x008b_8580, &args![actor, 4u32, 2u8]));
        assert_eq!(args_of(&log, 0x0043_6f20), vec![vec![changes, 4, 2]]);
        // A new faction is added; the rank is sign extended.
        let (_, log) = logged(&mut s.e, |e| {
            e.call(0x008b_8580, &args![actor, 9u32, 0xf0u8])
        });
        assert_eq!(
            args_of(&log, 0x0043_6f80),
            vec![vec![changes, 9, 0xffff_fff0]]
        );
        // Slot 0x48 is asked first; faction 0 does nothing.
        assert!(s.slot_calls().contains(&(a, 0x48, vec![0x8000_0000])));
        let (_, log) = logged(&mut s.e, |e| e.call(0x008b_8580, &args![actor, 0u32, 1u8]));
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn setting_a_faction_rank_creates_the_changes_when_missing() {
        let mut s = faction_stage();
        let actor = s.actor();
        let a = actor.addr();
        give_factions(&mut s, actor, &[(3, 0)], None);
        let block = s.e.mem.alloc(0x20);
        s.e.register_double(0x0042_e760, move |e, _| {
            e.mem.set_u32(a + 0x38, block);
            Ret::default()
        });
        zeros(&mut s.e, &[0x0043_6f80]);
        let (_, log) = logged(&mut s.e, |e| e.call(0x008b_8580, &args![actor, 9u32, 1u8]));
        assert_eq!(args_of(&log, 0x0043_6f80), vec![vec![block, 9, 1]]);
        // The same through 008b8e20, which always stores.
        s.e.mem.set_u32(a + 0x38, 0);
        let (_, log) = logged(&mut s.e, |e| e.call(0x008b_8e20, &args![actor, 8u32, 6u32]));
        assert_eq!(args_of(&log, 0x0043_6f80), vec![vec![block, 8, 6]]);
        // Without changes even after the creation, nothing is stored.
        s.e.register(0x0042_e760, |_, _| Ret::default());
        s.e.mem.set_u32(a + 0x38, 0);
        let (_, log) = logged(&mut s.e, |e| e.call(0x008b_8e20, &args![actor, 8u32, 6u32]));
        assert!(args_of(&log, 0x0043_6f80).is_empty());
    }

    #[test]
    fn changing_a_faction_rank_clamps_at_zero_and_needs_the_faction() {
        let mut s = faction_stage();
        let actor = s.actor();
        give_factions(&mut s, actor, &[(3, 0)], Some(&[(4, 1)]));
        let changes = s.e.mem.u32(actor.addr() + 0x38);
        zeros(&mut s.e, &[0x0043_6e10, 0x0043_6f20, 0x0043_6f80]);
        let (_, log) = logged(&mut s.e, |e| {
            e.call(0x008b_86e0, &args![actor, 3u32, 0xfdu8, 0u8])
        });
        // The rank of a held faction is 1; 1 - 3 is clamped at 0.
        assert_eq!(args_of(&log, 0x0043_6f20), vec![vec![changes, 3, 0]]);
        let (_, log) = logged(&mut s.e, |e| {
            e.call(0x008b_86e0, &args![actor, 3u32, 4u8, 0u8])
        });
        assert_eq!(args_of(&log, 0x0043_6f20), vec![vec![changes, 3, 5]]);
        // A faction the actor does not have, or none at all: nothing.
        let (_, log) = logged(&mut s.e, |e| {
            e.call(0x008b_86e0, &args![actor, 9u32, 4u8, 0u8])
        });
        assert!(args_of(&log, 0x0043_6f20).is_empty() && args_of(&log, 0x0043_6f80).is_empty());
        let (_, log) = logged(&mut s.e, |e| {
            e.call(0x008b_86e0, &args![actor, 0u32, 4u8, 0u8])
        });
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn faction_reactions_are_combined_by_008b8740() {
        let (mut e, _) = engine();
        e.register(0x0048_c1b0, |_, a| int(a[1]));
        let combine = |e: &mut Engine, running: i32, new: u32| {
            e.call(0x008b_8740, &args![running, 0x1000u32, new]).i32()
        };
        assert_eq!(combine(&mut e, 0, 3), 3);
        assert_eq!(combine(&mut e, 1, 0), 1);
        assert_eq!(combine(&mut e, 1, 3), 3);
        assert_eq!(combine(&mut e, 3, 2), 2);
        assert_eq!(combine(&mut e, 3, 1), 3);
        assert_eq!(combine(&mut e, 2, 3), 2);
        // The relation is asked of the faction's member at +0x24.
        e.call_log = Some(vec![]);
        combine(&mut e, 0, 5);
        assert_eq!(
            args_of(&e.call_log.take().unwrap(), 0x0048_c1b0),
            vec![vec![0x1024, 5]]
        );
    }

    /// The scene of the `008b87a0` tests: relations by `(faction, other)` come
    /// from `0048c1b0` through the thread-local table, `005a2270` accepts
    /// `friendly`.
    fn reaction_scene(
        relations: &'static [((u32, u32), u32)],
        friendly: u32,
    ) -> (Scene, Ptr<Actor>, Ptr<Actor>, u32) {
        let mut s = faction_stage();
        let (a, b) = (s.actor(), s.actor());
        REACTIONS.with(|table| *table.borrow_mut() = relations);
        s.e.register(0x0048_c1b0, |_, args| {
            let key = (args[0] - 0x24, args[1]);
            int(REACTIONS.with(|table| {
                table
                    .borrow()
                    .iter()
                    .find(|entry| entry.0 == key)
                    .map_or(0, |entry| entry.1)
            }))
        });
        s.e.register_double(0x005a_2270, move |_, a| int(u32::from(a[0] == friendly)));
        let flag = s.e.mem.alloc(8);
        (s, a, b, flag)
    }

    thread_local! {
        static REACTIONS: std::cell::RefCell<&'static [((u32, u32), u32)]> =
            const { std::cell::RefCell::new(&[]) };
    }

    #[test]
    fn the_fight_reaction_of_plain_factions_is_folded_pair_by_pair() {
        let (mut s, a, b, flag) = reaction_scene(
            &[((0x10, 0x30), 1), ((0x20, 0x30), 0), ((0x10, 0x40), 3)],
            0,
        );
        give_factions(&mut s, a, &[(0x10, 0), (0x20, 0)], None);
        give_factions(&mut s, b, &[(0x30, 0), (0x40, 0)], None);
        // 0 -> 1 (0x10 vs 0x30), then 1 -> 3 (0x10 vs 0x40); the rest are 0 and keep it.
        assert_eq!(s.e.call(0x008b_87a0, &args![a, b, flag]).i32(), 3);
        // A reaction of 2 ends the search at once.
        let (mut s, a, b, flag) = reaction_scene(&[((0x10, 0x30), 2)], 0);
        give_factions(&mut s, a, &[(0x10, 0), (0x20, 0)], None);
        give_factions(&mut s, b, &[(0x30, 0), (0x40, 0)], None);
        let (result, log) = logged(&mut s.e, |e| e.call(0x008b_87a0, &args![a, b, flag]).i32());
        assert_eq!(result, 2);
        assert_eq!(args_of(&log, 0x0048_c1b0).len(), 1);
        // Entries with a negative rank are ignored.
        let (mut s, a, b, flag) = reaction_scene(&[((0x10, 0x30), 2)], 0);
        give_factions(&mut s, a, &[(0x10, -1)], None);
        give_factions(&mut s, b, &[(0x30, 0)], None);
        assert_eq!(s.e.call(0x008b_87a0, &args![a, b, flag]).i32(), 0);
    }

    #[test]
    fn the_fight_reaction_towards_the_player_is_friendly_for_accepted_factions() {
        let (mut s, a, _, flag) = reaction_scene(&[((0x10, 0x30), 2)], 0x10);
        let player: Ptr<Actor> = Ptr::new(s.e.global::<u32>(PLAYER_CHARACTER));
        give_factions(&mut s, a, &[(0x10, 0), (0x20, 0)], None);
        give_factions(&mut s, player, &[(0x30, 0)], None);
        s.e.mem.set_u8(flag, 0);
        assert_eq!(s.e.call(0x008b_87a0, &args![a, player, flag]).i32(), 1);
        assert_eq!(s.e.mem.u8(flag), 1);
    }

    #[test]
    fn the_fight_reaction_with_changes_on_either_side() {
        // Changes on the first actor only: its merged list against the other's chain.
        let (mut s, a, b, flag) = reaction_scene(&[((0x11, 0x30), 2), ((0x10, 0x30), 1)], 0);
        give_factions(&mut s, a, &[(0x10, 0)], Some(&[(0x11, 0)]));
        give_factions(&mut s, b, &[(0x30, 0)], None);
        assert_eq!(s.e.call(0x008b_87a0, &args![a, b, flag]).i32(), 2);
        // Changes on the second actor only.
        let (mut s, a, b, flag) = reaction_scene(&[((0x10, 0x31), 2)], 0);
        give_factions(&mut s, a, &[(0x10, 0)], None);
        give_factions(&mut s, b, &[(0x30, 0)], Some(&[(0x31, 0)]));
        assert_eq!(s.e.call(0x008b_87a0, &args![a, b, flag]).i32(), 2);
        // Changes on both.
        let (mut s, a, b, flag) = reaction_scene(&[((0x11, 0x31), 1)], 0);
        give_factions(&mut s, a, &[(0x10, 0)], Some(&[(0x11, 0)]));
        give_factions(&mut s, b, &[(0x30, 0)], Some(&[(0x31, 0)]));
        assert_eq!(s.e.call(0x008b_87a0, &args![a, b, flag]).i32(), 1);
        // The player as the other actor with changes: friendly, flag set (not for both-sides).
        let (mut s, a, _, flag) = reaction_scene(&[], 0x10);
        let player: Ptr<Actor> = Ptr::new(s.e.global::<u32>(PLAYER_CHARACTER));
        give_factions(&mut s, a, &[(0x10, 0)], Some(&[(0x11, 0)]));
        give_factions(&mut s, player, &[(0x30, 0)], None);
        assert_eq!(s.e.call(0x008b_87a0, &args![a, player, flag]).i32(), 1);
        assert_eq!(s.e.mem.u8(flag), 1);
        give_factions(&mut s, a, &[(0x10, 0)], None);
        give_factions(&mut s, player, &[(0x30, 0)], Some(&[(0x31, 0)]));
        s.e.mem.set_u8(flag, 0);
        assert_eq!(s.e.call(0x008b_87a0, &args![a, player, flag]).i32(), 1);
        assert_eq!(s.e.mem.u8(flag), 1);
        give_factions(&mut s, a, &[(0x10, 0)], Some(&[(0x11, 0)]));
        s.e.mem.set_u8(flag, 0);
        assert_eq!(s.e.call(0x008b_87a0, &args![a, player, flag]).i32(), 1);
        assert_eq!(s.e.mem.u8(flag), 0);
    }

    #[test]
    fn pathfinding_goals_answer_the_movers_value() {
        let (mut w, this, _mover, _log) = mover_world();
        let process = w.e.get(this, Actor::pCurrentProcess);
        w.answer(process.addr(), 0x254, 0x5555);
        w.e.register(0x0069_e390, |_, _| int(1));
        for address in [0x009d_ba30u32, 0x009d_ba50, 0x009d_bb10, 0x009d_bb60] {
            w.e.register(address, |_, _| int(1));
        }
        assert!(w.e.call(0x008b_3630, &args![this, 0x66u32]).bool());
        assert!(w
            .e
            .call(0x008b_3690, &args![this, 1u32, 2u32, 3u32, 4.5f32, 0u32])
            .bool());
        assert!(w
            .e
            .call(0x008b_36f0, &args![this, 1u32, 2.5f32, 0u32])
            .bool());
        assert!(w
            .e
            .call(
                0x008b_3750,
                &args![this, 1u32, 2u32, 3u32, 4.5f32, 5.5f32, 0u32]
            )
            .bool());
        w.e.register(0x009d_ba30, |_, _| int(0));
        assert!(!w.e.call(0x008b_3630, &args![this, 0x66u32]).bool());
    }

    // ---- the exe's stack frames ----

    /// The local the next `with_stack(size)` on this thread takes: the frame of the function the
    /// test calls next, taken outside any other local of it (nothing takes its place again
    /// before the function returns, so its words can still be read after the call).
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        e.next_stack_local(size)
    }

    #[test]
    fn the_faction_list_buffers_sit_at_the_exe_frame_offsets() {
        // (function, its arguments after the actor, frame size, buffer offset)
        let cases: [(u32, Vec<u32>, u32, u32); 8] = [
            (0x008b_7c00, vec![0x55, 9], 0x21c, 0xc),
            (0x008b_7d20, vec![0x55, 9], 0x21c, 0xc),
            (0x008b_7e20, vec![], 0x21c, 0xc),
            (0x008b_7f00, vec![], 0x21c, 0xc),
            (0x008b_8290, vec![0x99, 0], 0x21c, 0xc),
            (0x008b_8360, vec![0, 0], 0x224, 0x14),
            (0x008b_8490, vec![], 0x224, 0x14),
            (0x008b_8580, vec![0x77, 1], 0x224, 0x14),
        ];
        for (function, rest, size, offset) in cases {
            let mut s = faction_stage();
            let actor = s.actor();
            let (first, second) = (faction_with(&mut s.e, 0), faction_with(&mut s.e, 0));
            give_factions(&mut s, actor, &[(first, 0), (second, 0)], None);
            zeros(
                &mut s.e,
                &[
                    0x005f_da00,
                    0x005f_da50,
                    0x0067_33e0,
                    0x0080_41a0,
                    0x008b_8e90,
                    0x0043_6f80,
                    0x0043_6f20,
                    0x0043_6e10,
                    0x0042_e760,
                ],
            );
            let mut args = vec![actor.addr()];
            args.extend(rest);
            let frame = next_frame(&mut s.e, size);
            s.e.call(function, &args);
            assert_eq!(
                [s.e.mem.u32(frame + offset), s.e.mem.u32(frame + offset + 4)],
                [first, second],
                "{function:08x}"
            );
        }
    }

    #[test]
    fn the_relation_buffers_sit_at_the_exe_frame_offsets() {
        let (mut s, a, b) = relation_scene(&[(0x100, 0)], &[(0x200, 0)], &[((0x100, 0x200), 7)]);
        let out = s.e.mem.alloc(0x10);
        let frame = next_frame(&mut s.e, 0x444);
        s.e.call(0x008b_7fe0, &args![a, b, out, out + 4, out + 8]);
        assert_eq!(s.e.mem.u32(frame + 0x1c), 0x100);
        assert_eq!(s.e.mem.u32(frame + 0x22c), 0x200);
    }

    #[test]
    fn the_fight_reaction_buffers_sit_at_the_exe_frame_offsets() {
        // Changes on the first actor only: +0x644.
        let (mut s, a, b, flag) = reaction_scene(&[], 0);
        give_factions(&mut s, a, &[(0x10, 0)], Some(&[(0x11, 0)]));
        give_factions(&mut s, b, &[(0x30, 0)], None);
        let frame = next_frame(&mut s.e, 0x86c);
        s.e.call(0x008b_87a0, &args![a, b, flag]);
        assert_eq!(s.e.mem.u32(frame + 0x644), 0x10);
        // Changes on the second actor only: +0x42c.
        let (mut s, a, b, flag) = reaction_scene(&[], 0);
        give_factions(&mut s, a, &[(0x10, 0)], None);
        give_factions(&mut s, b, &[(0x30, 0)], Some(&[(0x31, 0)]));
        let frame = next_frame(&mut s.e, 0x86c);
        s.e.call(0x008b_87a0, &args![a, b, flag]);
        assert_eq!(s.e.mem.u32(frame + 0x42c), 0x30);
        // Changes on both: +0x14 and +0x21c.
        let (mut s, a, b, flag) = reaction_scene(&[], 0);
        give_factions(&mut s, a, &[(0x10, 0)], Some(&[(0x11, 0)]));
        give_factions(&mut s, b, &[(0x30, 0)], Some(&[(0x31, 0)]));
        let frame = next_frame(&mut s.e, 0x86c);
        s.e.call(0x008b_87a0, &args![a, b, flag]);
        assert_eq!(s.e.mem.u32(frame + 0x14), 0x10);
        assert_eq!(s.e.mem.u32(frame + 0x21c), 0x30);
    }

    #[test]
    fn a_conversation_keeps_its_package_locations_in_the_frame() {
        let (mut w, this, this_process, partner, _) = talk_world();
        w.e.register(0x0083_b9a0, |_, _| int(1));
        let built = Rc::new(Cell::new(0));
        let seen = built.clone();
        w.e.register_double(DIALOGUE_PACKAGE_CONSTRUCTOR, move |_, a| {
            seen.set(a[0]);
            int(a[0])
        });
        let accepted = built.clone();
        w.e.register_double(0x0093_44a0, move |_, _| int(accepted.get()));
        w.answer(this_process.addr(), 0x4bc, 0);
        // The 0xc4-byte frame of 008b2170: the locations at +0x90 and +0x9c.
        let frame = next_frame(&mut w.e, 0xc4);
        w.e.call_log = Some(vec![]);
        assert!(talk(
            &mut w,
            this,
            partner,
            [true, true, false, false, false]
        ));
        let log = w.e.call_log.take().unwrap();
        let locations: Vec<u32> = args_of(&log, PACKAGE_LOCATION_CONSTRUCTOR)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(locations, vec![frame + 0x90, frame + 0x9c]);
        let destroyed: Vec<u32> = args_of(&log, PACKAGE_LOCATION_DESTRUCTOR)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(destroyed, vec![frame + 0x9c, frame + 0x90]);
    }
}
