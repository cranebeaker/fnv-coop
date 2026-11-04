//! `fallout/ai/combat/hitdata.cpp` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVBGSBodyPart@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0108F9E8: u32 = 0x0108_f9e8;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 009b7970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@PAVBGSBodyPart@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_009b7970(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108F9E8);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `00761540` (unnamed in the engine map, `creditsmenu.cpp`): reserves the next slot of a
/// `BSSimpleArray`, growing the buffer when it is full, and returns the new slot's index.
const ARRAY_RESERVE_SLOT: u32 = 0x0076_1540;
/// `00726bf0` (unnamed in the engine map): `thiscall`, `RET 8`; constructs `count` four-byte
/// elements starting at the address it is given.
const ARRAY_CONSTRUCT_ELEMENTS: u32 = 0x0072_6bf0;

// Translated from 007cb2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BGSBodyPart *, 1024>::AddUninitialized` (name from the engine map; the body
/// reserves a slot through `00761540`, constructs one element in it through `00726bf0`, then
/// copies `*item` into the slot, which is what an `Add` of one pointer does, so the name is
/// unconfirmed). `thiscall`, `RET 4`; `item` points to the pointer to store. Returns the slot's
/// index. The buffer pointer is read again after each call, as the exe does.
pub fn fn_007cb2e0(e: &mut Engine, this: Ptr<BSSimpleArray>, item: Ptr) -> u32 {
    let index = e.call(ARRAY_RESERVE_SLOT, &args![this]).u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let slot = buffer.wrapping_add(index.wrapping_mul(4));
    e.call(ARRAY_CONSTRUCT_ELEMENTS, &args![this, slot, 1u32]);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let value = e.mem.u32(item.addr());
    e.mem
        .set_u32(buffer.wrapping_add(index.wrapping_mul(4)), value);
    index
}

layout! {
    /// `HitData` (Xbox PDB), 0x64 bytes on PC as on the Xbox (the offsets below are the ones the
    /// PC code uses).
    pub struct HitData: 0x64 {
        /// `pAggressor` (Xbox PDB): `Actor *`.
        0x00 pAggressor: u32,
        /// `pTarget` (Xbox PDB): `Actor *`.
        0x04 pTarget: u32,
        /// `pSourceRef` (Xbox PDB): `TESObjectREFR *` (the explosion, for explosion hits).
        0x08 pSourceRef: u32,
        /// `eAttackSkill` (Xbox PDB): `ActorValue::Index`.
        0x0C eAttackSkill: u32,
        /// `eDamageLimb` (Xbox PDB): `BGSBodyPart::LIMB_ENUM`.
        0x10 eDamageLimb: u32,
        /// `fHealthDamage` (Xbox PDB).
        0x14 fHealthDamage: u32,
        /// `fTotalDamage` (Xbox PDB).
        0x18 fTotalDamage: u32,
        /// `fFatigueDamage` (Xbox PDB).
        0x1C fFatigueDamage: u32,
        /// `fTargetedLimbDamage` (Xbox PDB).
        0x20 fTargetedLimbDamage: u32,
        /// `fPercentBlocked` (Xbox PDB).
        0x24 fPercentBlocked: u32,
        /// `fArmorDamage` (Xbox PDB).
        0x28 fArmorDamage: u32,
        /// `fDamageToWeapon` (Xbox PDB).
        0x2C fDamageToWeapon: u32,
        /// `pWeapon` (Xbox PDB): `TESObjectWEAP *`.
        0x30 pWeapon: u32,
        /// `fWeaponHealth` (Xbox PDB).
        0x34 fWeaponHealth: u32,
        /// `pCriticalEffect` (Xbox PDB): `SpellItem *`.
        0x50 pCriticalEffect: u32,
        /// `pVATSCommand` (Xbox PDB): `VATS_COMMAND *`.
        0x54 pVATSCommand: u32,
        /// `uiFlags` (Xbox PDB).
        0x58 uiFlags: u32,
        /// `fBonusMult` (Xbox PDB).
        0x5C fBonusMult: u32,
        /// `iRefCount` (Xbox PDB).
        0x60 iRefCount: u32,
    }
}

/// `HitData::kHitLocation` (Xbox PDB), an `NiPoint3` at `+0x38`.
const HIT_LOCATION: u32 = 0x38;
/// `HitData::kHitDirection` (Xbox PDB), an `NiPoint3` at `+0x44`.
const HIT_DIRECTION: u32 = 0x44;
/// `00407e00` (unnamed in the engine map): `thiscall`, `RET 8`; sets (`value` non-zero) or
/// clears the bits `mask` of `uiFlags`.
const SET_FLAG: u32 = 0x0040_7e00;
/// `0058cba0` (unnamed in the engine map): `thiscall`, `RET 4`; whether any bit of `mask` is set.
const IS_FLAG_SET: u32 = 0x0058_cba0;
/// `00403e20` (unnamed in the engine map, `effectitem.cpp`): on a game setting, the address of
/// its value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `BGSEntryPoint::HandleEntryPoint` (`005e58f0`, `cdecl`, five words here).
const HANDLE_ENTRY_POINT: u32 = 0x005e_58f0;
/// The default weapon (the unarmed weapon form) at `011ca278`.
const DEFAULT_WEAPON_011CA278: u32 = 0x011c_a278;
/// The player character.
const PLAYER_011DEA3C: u32 = 0x011d_ea3c;
/// `0.0` (`double`).
const ZERO_01012060: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE_01012070: u32 = 0x0101_2070;
/// `100.0` (`double`).
const HUNDRED_01017A40: u32 = 0x0101_7a40;
/// `100.0` (`float`).
const HUNDRED_01016410: u32 = 0x0101_6410;
/// `10.0` (`double`).
const TEN_01020758: u32 = 0x0102_0758;
/// `0.05` (`double`, the `float` 0.05 widened).
const FIVE_HUNDREDTHS_0103C7C8: u32 = 0x0103_c7c8;
/// `_ftol2` (`00ec62c0`): the value in `ST0` (an `f64` argument) truncated, in `EAX`.
const FTOL: u32 = 0x00ec_62c0;

/// The x87 condition bits `FCOM` leaves in `AH` (after `FNSTSW AX`) for `st0` compared with
/// `st1`: C0 (0x01) for less, C3 (0x40) for equal, C3|C2|C0 (0x45) unordered, none for greater.
fn fpu_compare(st0: f64, st1: f64) -> u8 {
    if st0.is_nan() || st1.is_nan() {
        0x45
    } else if st0 < st1 {
        0x01
    } else if st0 == st1 {
        0x40
    } else {
        0
    }
}

/// The parity flag of `TEST AH,mask`: set when the masked byte has an even number of bits.
fn parity_even(value: u8) -> bool {
    value.count_ones() % 2 == 0
}

/// `FCOMP` against 0 then `TEST AH,0x41` / `JNZ`: the branch is skipped unless `value > 0`.
fn above_zero(e: &Engine, value: f64) -> bool {
    fpu_compare(value, double_at(e, ZERO_01012060)) & 0x41 == 0
}

/// `FCOMP` against 0 then `TEST AH,0x41` / `JP`: taken (not skipped) when `value <= 0`.
fn at_most_zero(e: &Engine, value: f64) -> bool {
    !parity_even(fpu_compare(value, double_at(e, ZERO_01012060)) & 0x41)
}

fn double_at(e: &Engine, address: u32) -> f64 {
    f64::from_bits(e.mem.u64(address))
}

/// The `float` value of the game setting `setting` (through `00403e20`), as its bits.
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.u32(value)
}

fn field(e: &Engine, this: Ptr<HitData>, offset: u32) -> u32 {
    e.mem.u32(this.addr().wrapping_add(offset))
}

fn set_field(e: &mut Engine, this: Ptr<HitData>, offset: u32, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(offset), value);
}

fn set_flag(e: &mut Engine, this: Ptr<HitData>, mask: u32, value: u32) {
    e.call(SET_FLAG, &args![this, mask, value]);
}

/// The weapon, or the default weapon (`011ca278`) when the hit has none.
fn weapon_or_default(e: &Engine, this: Ptr<HitData>) -> u32 {
    let weapon = e.get(this, HitData::pWeapon);
    if weapon != 0 {
        weapon
    } else {
        e.mem.u32(DEFAULT_WEAPON_011CA278)
    }
}

/// The weapon's current ammunition's effect list for the aggressor
/// (`TESObjectWEAP::GetCurrentAmmo(weapon, aggressor)`, `00525980`, then `00923000` on it), or
/// 0 without ammunition.
fn ammo_effects(e: &mut Engine, this: Ptr<HitData>) -> u32 {
    let weapon = e.get(this, HitData::pWeapon);
    let aggressor = e.get(this, HitData::pAggressor);
    if e.call(0x0052_5980, &args![weapon, aggressor]).u32() == 0 {
        return 0;
    }
    let weapon = e.get(this, HitData::pWeapon);
    let aggressor = e.get(this, HitData::pAggressor);
    let ammo = e.call(0x0052_5980, &args![weapon, aggressor]).u32();
    e.call(0x0092_3000, &args![ammo]).u32()
}

/// `0059a030(kind, effects, value)` (`cdecl`): the ammunition effects of `kind` applied to the
/// `float` `value`, stored as a `float`.
fn apply_ammo_effects(e: &mut Engine, kind: u32, effects: u32, value: u32) -> u32 {
    x87_store32(
        e.call(0x0059_a030, &args![kind, effects, x87_quiet_bits(value)])
            .f64(),
    )
}

/// The `1 + 0.05 * GetClampedActorFloatValue(player, 8)` factor of `ReduceDamage` (the player's
/// `ActorValueOwner` at `+0xa4`), both steps stored as `float`s.
fn player_factor(e: &mut Engine) -> u32 {
    let player = e.mem.u32(PLAYER_011DEA3C);
    let value = x87_store32(
        e.call(0x0066_ef50, &args![player.wrapping_add(0xa4), 8u32])
            .f64(),
    );
    let scaled = x87_mul(x87_load32(value), double_at(e, FIVE_HUNDREDTHS_0103C7C8));
    x87_store32(x87_add(scaled, 1.0))
}

/// The target's actor value `index` through its `ActorValueOwner` (`+0xa4`) vtable slot `0xc`
/// (`ST0`).
fn target_actor_value(e: &mut Engine, target: u32, index: u32) -> f64 {
    e.vcall(target.wrapping_add(0xa4), 0xc, &args![index]).f64()
}

// Translated from 009a4320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BGSBodyPart *,1024>::_MoveItems` (Xbox PDB), `RET 8`: drops `count` items
/// at `index`: the items after them (`0042f5a0(size - index - count, count)` of them) are
/// destroyed (`0072ba80(this, buffer + index, count)`) and moved down (`0042fb60(this, buffer +
/// index, buffer + size - moved, moved)`), and the size (`+8`) shrinks by `count`.
pub fn bs_simple_array_body_part_move_items(e: &mut Engine, this: Ptr, index: u32, count: u32) {
    let base = this.addr();
    let size = e.mem.u32(base + 8);
    let moved = e
        .call(
            0x0042_f5a0,
            &args![size.wrapping_sub(index).wrapping_sub(count), count],
        )
        .u32();
    let buffer = e.mem.u32(base + 4);
    e.call(
        0x0072_ba80,
        &args![this, buffer.wrapping_add(index.wrapping_mul(4)), count],
    );
    let size = e.mem.u32(base + 8);
    let buffer = e.mem.u32(base + 4);
    let source = buffer
        .wrapping_add(size.wrapping_mul(4))
        .wrapping_sub(moved.wrapping_shl(2));
    let destination = buffer.wrapping_add(index.wrapping_mul(4));
    e.call(0x0042_fb60, &args![this, destination, source, moved]);
    let size = e.mem.u32(base + 8);
    e.mem.set_u32(base + 8, size.wrapping_sub(count));
}

// Translated from 009b4d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::HitData` (Xbox PDB): runs `006815c0` on the two points (`+0x38`, `+0x44`; the
/// engine map's name for that folded body does not fit) and `HitData::ClearData`. Returns
/// `this`.
pub fn hit_data_constructor(e: &mut Engine, this: Ptr<HitData>) -> Ptr<HitData> {
    e.call(0x0068_15c0, &args![this.addr().wrapping_add(HIT_LOCATION)]);
    e.call(0x0068_15c0, &args![this.addr().wrapping_add(HIT_DIRECTION)]);
    hit_data_clear_data(e, this);
    this
}

// Translated from 009b4dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::ClearData` (Xbox PDB): clears every field; `eAttackSkill` becomes 0x4d and
/// `eDamageLimb` -1, both points the three words at `011f426c`. `iRefCount` is cleared too.
pub fn hit_data_clear_data(e: &mut Engine, this: Ptr<HitData>) {
    e.set(this, HitData::pAggressor, 0);
    e.set(this, HitData::pTarget, 0);
    e.set(this, HitData::pSourceRef, 0);
    e.set(this, HitData::eAttackSkill, 0x4d);
    e.set(this, HitData::eDamageLimb, u32::MAX);
    for offset in (0x14..=0x2c).step_by(4) {
        set_field(e, this, offset, 0);
    }
    for point in [HIT_LOCATION, HIT_DIRECTION] {
        for word in 0..3 {
            let value = e.mem.u32(0x011f_426c + 4 * word);
            set_field(e, this, point + 4 * word, value);
        }
    }
    e.set(this, HitData::pWeapon, 0);
    e.set(this, HitData::fWeaponHealth, 0);
    e.set(this, HitData::pCriticalEffect, 0);
    e.set(this, HitData::pVATSCommand, 0);
    e.set(this, HitData::uiFlags, 0);
    e.set(this, HitData::iRefCount, 0);
    e.set(this, HitData::fBonusMult, 0);
}

// Translated from 009b4ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::Copy` (Xbox PDB), `RET 4`: copies every field of `other` but `iRefCount` (the
/// `float`s through `FLD`/`FSTP`, which quiets a signalling NaN).
pub fn hit_data_copy(e: &mut Engine, this: Ptr<HitData>, other: Ptr<HitData>) {
    for offset in [0u32, 4, 8, 0xc, 0x10] {
        let value = field(e, other, offset);
        set_field(e, this, offset, value);
    }
    for offset in (0x14..=0x2c).step_by(4) {
        let value = x87_quiet_bits(field(e, other, offset));
        set_field(e, this, offset, value);
    }
    for offset in (HIT_LOCATION..HIT_LOCATION + 0x18).step_by(4) {
        let value = field(e, other, offset);
        set_field(e, this, offset, value);
    }
    let weapon = field(e, other, 0x30);
    set_field(e, this, 0x30, weapon);
    let health = x87_quiet_bits(field(e, other, 0x34));
    set_field(e, this, 0x34, health);
    for offset in [0x50u32, 0x54, 0x58] {
        let value = field(e, other, offset);
        set_field(e, this, offset, value);
    }
    let bonus = x87_quiet_bits(field(e, other, 0x5c));
    set_field(e, this, 0x5c, bonus);
}

// Translated from 009b5770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::InitializeExplosionData` (Xbox PDB), `cdecl` (`hit`, `aggressor`, `target`,
/// `explosion`, `damage`). Nothing for a null `hit`. Stores the aggressor and target and sets
/// flag 0x2000. An explosion (vtable slot `0x220` true) gives the weapon (`009b0900`) with
/// health 1.0; when the explosion's owner (`009acce0`) is a character (vtable slot `0x100`)
/// holding that weapon (`Actor::GetCurrentWeapon`), the health is
/// `ItemChange::GetItemHealth(item, 1)` of the owner's process's vtable slot `0x148` item (1.0
/// without one). Then the source is the explosion, both damages the `damage` and the hit runs
/// `ReduceDamage(1)`, `CheckCriticalStrike` and `DetermineOutcome`.
pub fn hit_data_initialize_explosion_data(
    e: &mut Engine,
    hit: Ptr<HitData>,
    aggressor: u32,
    target: u32,
    explosion: u32,
    damage: f32,
) {
    if hit.is_null() {
        return;
    }
    e.set(hit, HitData::pAggressor, aggressor);
    e.set(hit, HitData::pTarget, target);
    set_flag(e, hit, 0x2000, 1);
    if explosion != 0 && e.vcall(explosion, 0x220, &args![]).bool() {
        let weapon = e.call(0x009b_0900, &args![explosion]).u32();
        e.set(hit, HitData::pWeapon, weapon);
        e.set(hit, HitData::fWeaponHealth, 1.0f32.to_bits());
        if e.call(0x009a_cce0, &args![explosion]).u32() != 0 {
            let owner = e.call(0x009a_cce0, &args![explosion]).u32();
            if e.vcall(owner, 0x100, &args![]).bool() {
                let actor = e.call(0x009a_cce0, &args![explosion]).u32();
                if e.call(0x008a_1710, &args![actor]).u32() == e.get(hit, HitData::pWeapon) {
                    let item = if e.call(0x008d_8520, &args![actor]).u32() != 0 {
                        let process = e.call(0x008d_8520, &args![actor]).u32();
                        e.vcall(process, 0x148, &args![]).u32()
                    } else {
                        0
                    };
                    let health = if item != 0 {
                        x87_store32(e.call(0x004b_cdb0, &args![item, 1u32]).f64())
                    } else {
                        1.0f32.to_bits()
                    };
                    e.set(hit, HitData::fWeaponHealth, x87_quiet_bits(health));
                }
            }
        }
    }
    e.set(hit, HitData::pSourceRef, explosion);
    e.set(
        hit,
        HitData::fHealthDamage,
        x87_quiet_bits(damage.to_bits()),
    );
    let health_damage = e.get(hit, HitData::fHealthDamage);
    e.set(hit, HitData::fTotalDamage, x87_quiet_bits(health_damage));
    hit_data_reduce_damage(e, hit, 1);
    hit_data_check_critical_strike(e, hit);
    hit_data_determine_outcome(e, hit);
}

// Translated from 009b5a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::ReduceDamage` (Xbox PDB), `RET 4` (`ignore_block`, its low byte). Only for a
/// character target (vtable slot `0x100`); a target whose `+0x94` object's vtable slot `0x10`
/// is true takes no damage (total, health and fatigue damage cleared). Otherwise the total
/// damage is the health damage, the minimum damage `health damage * setting 011ced4c`, and:
/// - the fatigue damage goes through the ammunition effects of kind 5;
/// - damage resistance: `min(actor value 0x12, 100) / 100`, scaled by the player's `1 + 0.05 *
///   value 8` factor when `00566950(target)`, through the ammunition effects of kind 1 and
///   capped at `setting 011d1020 / 100`; the armor damage is `006462e0(damage, resistance)`
///   unless the fatigue damage is above 0 (then 0), and the health damage is scaled by `1 -
///   resistance`;
/// - damage threshold: `max(actor value 0x4c, 0)`, the same factor, kept unmodified for the
///   test below and through the ammunition effects of kind 2;
/// - a blocked hit (unless `ignore_block`; `Actor::GetBlocked` when vtable slot `0x234` is
///   false) that `009a6ae0(target, source or aggressor, 0)` accepts sets flag 1 (the player's
///   block runs `00944380` with setting `011d0be4`), computes the percentage blocked from the
///   skill (`0x26` for a weapon of type 1 or 2 held, else `0x2d` when unarmed or with type 0;
///   `006463a0`), 0 for an unarmed block of a weapon hit or a hit with a source, adds it to the
///   threshold and sets flag 2 with whether the target's weapon is drawn;
/// - entry points 0x3a (the aggressor's, with the weapon, or `004839c0(500)` stored when
///   there is none) and 0x38 (the target's) lower and raise the threshold, which is then not
///   below 0, the "threshold ate everything" marker being set when the original threshold was
///   above 0 and the result at most 0; a weapon with mod effect 0xc (`ItemChange::
///   HasModEffectActive_ov2`) divides it by `GetNumProjectiles`;
/// - the threshold is subtracted, or for `00524b40`/`006450c0` weapons applied as a fraction of
///   `max(vtable slot 0x10 of the weapon's +0x9c object, 1)`;
/// - the ammunition effects of kind 0 and entry point 0 adjust the health damage; with no
///   fatigue damage the armor damage gains `setting 011cf0fc * min(threshold, damage -
///   minimum)`;
/// - a damage below the minimum is raised to it, with the hit-marker UI (`00709c60` for the
///   player as target, `00709c80` for the player as aggressor unless slot `0x22c`) and a
///   follower bark (`FollowerBarks::TriggerFollowerBark(aggressor, 9)` when `00566950`); the
///   UI also runs for the marker alone;
/// - a weapon enchantment's actor value (`009b6600`, not -1) between 0 and 100% scales the
///   damage again; a positive fatigue damage follows the health damage's ratio to the total;
///   for the player as target with `00525430(011f2250)` true every damage is scaled by setting
///   `011ce9f0`.
pub fn hit_data_reduce_damage(e: &mut Engine, this: Ptr<HitData>, ignore_block: u32) {
    let target = e.get(this, HitData::pTarget);
    if target == 0 || !e.vcall(target, 0x100, &args![]).bool() {
        return;
    }
    let target = e.get(this, HitData::pTarget);
    if e.vcall(target.wrapping_add(0x94), 0x10, &args![]).bool() {
        e.set(this, HitData::fTotalDamage, 0);
        e.set(this, HitData::fHealthDamage, 0);
        e.set(this, HitData::fFatigueDamage, 0);
        return;
    }
    let damage = e.get(this, HitData::fHealthDamage);
    e.set(this, HitData::fTotalDamage, x87_quiet_bits(damage));
    let minimum_scale = setting_bits(e, 0x011c_ed4c);
    let damage = e.get(this, HitData::fHealthDamage);
    let minimum = x87_store32(x87_mul(x87_load32(damage), x87_m32(minimum_scale)));
    if e.get(this, HitData::pWeapon) != 0 {
        let effects = ammo_effects(e, this);
        let fatigue = e.get(this, HitData::fFatigueDamage);
        let fatigue = apply_ammo_effects(e, 5, effects, fatigue);
        e.set(this, HitData::fFatigueDamage, fatigue);
    }
    let hundred = x87_quiet_bits(e.mem.u32(HUNDRED_01016410));
    let target = e.get(this, HitData::pTarget);
    let resistance_value = x87_store32(target_actor_value(e, target, 0x12));
    let clamped = e.call(0x0040_ebd0, &args![resistance_value, hundred]).f64();
    let mut resistance = x87_store32(x87_div(clamped, double_at(e, HUNDRED_01017A40)));
    let target = e.get(this, HitData::pTarget);
    if e.call(0x0056_6950, &args![target]).bool() {
        let factor = player_factor(e);
        resistance = x87_store32(x87_mul(x87_load32(resistance), x87_m32(factor)));
    }
    if e.get(this, HitData::pWeapon) != 0 {
        let effects = ammo_effects(e, this);
        resistance = apply_ammo_effects(e, 1, effects, resistance);
    }
    let cap = setting_bits(e, 0x011d_1020);
    let cap = x87_store32(x87_div(x87_load32(cap), double_at(e, HUNDRED_01017A40)));
    if !parity_even(fpu_compare(x87_load32(cap), x87_load32(resistance)) & 0x05) {
        resistance = x87_quiet_bits(cap);
    }
    let fatigue = e.get(this, HitData::fFatigueDamage);
    let armor_damage = if above_zero(e, x87_load32(fatigue)) {
        0
    } else {
        let damage = x87_quiet_bits(e.get(this, HitData::fHealthDamage));
        x87_store32(
            e.call(0x0064_62e0, &args![damage, x87_quiet_bits(resistance)])
                .f64(),
        )
    };
    e.set(this, HitData::fArmorDamage, x87_quiet_bits(armor_damage));
    let damage = e.get(this, HitData::fHealthDamage);
    let kept = x87_sub(1.0, x87_load32(resistance));
    e.set(
        this,
        HitData::fHealthDamage,
        x87_store32(x87_mul(kept, x87_m32(damage))),
    );
    let target = e.get(this, HitData::pTarget);
    let threshold_value = x87_store32(target_actor_value(e, target, 0x4c));
    let mut threshold = x87_store32(e.call(0x0040_4010, &args![threshold_value, 0.0f32]).f64());
    let target = e.get(this, HitData::pTarget);
    if e.call(0x0056_6950, &args![target]).bool() {
        let factor = player_factor(e);
        threshold = x87_store32(x87_mul(x87_load32(threshold), x87_m32(factor)));
    }
    let original_threshold = x87_quiet_bits(threshold);
    if e.get(this, HitData::pWeapon) != 0 {
        let effects = ammo_effects(e, this);
        threshold = apply_ammo_effects(e, 2, effects, threshold);
    }
    if ignore_block as u8 == 0 {
        threshold = reduce_damage_block(e, this, threshold);
    }
    threshold = reduce_damage_entry_points(e, this, threshold);
    let marker =
        above_zero(e, x87_load32(original_threshold)) && at_most_zero(e, x87_load32(threshold));
    threshold = x87_store32(
        e.call(0x0040_4010, &args![0.0f32, x87_quiet_bits(threshold)])
            .f64(),
    );
    threshold = reduce_damage_projectiles(e, this, threshold);
    let weapon = e.get(this, HitData::pWeapon);
    if weapon != 0
        && e.call(0x0052_4b40, &args![weapon]).bool()
        && e.call(0x0064_50c0, &args![weapon]).bool()
    {
        let weapon = e.get(this, HitData::pWeapon);
        let value = e.vcall(weapon.wrapping_add(0x9c), 0x10, &args![]).u16();
        let divisor = e.call(0x009b_65d0, &args![u32::from(value), 1u32]).i32();
        let divisor = x87_store32(f64::from(divisor));
        let fraction = x87_div(x87_load32(threshold), x87_m32(divisor));
        let kept = x87_store32(x87_sub(1.0, fraction));
        let kept = x87_store32(
            e.call(0x0040_4010, &args![x87_quiet_bits(kept), 0.0f32])
                .f64(),
        );
        let damage = e.get(this, HitData::fHealthDamage);
        e.set(
            this,
            HitData::fHealthDamage,
            x87_store32(x87_mul(x87_load32(damage), x87_m32(kept))),
        );
    } else {
        let damage = e.get(this, HitData::fHealthDamage);
        e.set(
            this,
            HitData::fHealthDamage,
            x87_store32(x87_sub(x87_load32(damage), x87_m32(threshold))),
        );
    }
    reduce_damage_finish(e, this, minimum, threshold, marker);
}

/// The block part of `ReduceDamage` (`009b5a30`); returns the threshold.
fn reduce_damage_block(e: &mut Engine, this: Ptr<HitData>, threshold: u32) -> u32 {
    let mut threshold = threshold;
    let target = e.get(this, HitData::pTarget);
    let blocked = if e.vcall(target, 0x234, &args![]).bool() {
        false
    } else {
        let target = e.get(this, HitData::pTarget);
        e.call(0x0089_4d60, &args![target]).bool()
    };
    let target = e.get(this, HitData::pTarget);
    let process = e.call(0x008d_8520, &args![target]).u32();
    let unarmed_block = e.vcall(process, 0x148, &args![]).u32() == 0 && blocked;
    if !blocked {
        return threshold;
    }
    let source = match e.get(this, HitData::pSourceRef) {
        0 => e.get(this, HitData::pAggressor),
        source => source,
    };
    let target = e.get(this, HitData::pTarget);
    if !e.call(0x009a_6ae0, &args![target, source, 0u32]).bool() {
        return threshold;
    }
    let target = e.get(this, HitData::pTarget);
    let process = e.call(0x008d_8520, &args![target]).u32();
    let held = if e.vcall(process, 0x148, &args![]).u32() != 0 {
        let target = e.get(this, HitData::pTarget);
        let process = e.call(0x008d_8520, &args![target]).u32();
        let item = e.vcall(process, 0x148, &args![]).u32();
        e.call(0x0044_ddc0, &args![item]).u32()
    } else {
        0
    };
    set_flag(e, this, 1, 1);
    let player = e.mem.u32(PLAYER_011DEA3C);
    if player == e.get(this, HitData::pTarget) {
        let value = x87_quiet_bits(setting_bits(e, 0x011d_0be4));
        let player = e.mem.u32(PLAYER_011DEA3C);
        e.call(0x0094_4380, &args![player, value]);
    }
    let mut drawn = 0u32;
    if held != 0 {
        let target = e.get(this, HitData::pTarget);
        if e.call(0x008a_16d0, &args![target]).bool() {
            drawn = 1;
        }
    }
    let skill_index = if e.get(this, HitData::pWeapon) != 0
        && held != 0
        && (e.call(0x0044_6390, &args![held]).u32() == 2
            || e.call(0x0044_6390, &args![held]).u32() == 1)
    {
        Some(0x26u32)
    } else if e.get(this, HitData::pWeapon) == 0
        || (held != 0 && e.call(0x0044_6390, &args![held]).u32() == 0)
    {
        Some(0x2d)
    } else {
        None
    };
    let skill = match skill_index {
        Some(index) => {
            let target = e.get(this, HitData::pTarget);
            e.vcall(target.wrapping_add(0xa4), 8, &args![index]).u32()
        }
        None => 0,
    };
    let weapon = e.get(this, HitData::pWeapon);
    let no_block = unarmed_block
        && ((weapon != 0 && !e.call(0x0064_50c0, &args![weapon]).bool())
            || e.get(this, HitData::pSourceRef) != 0);
    let percent = if no_block {
        0
    } else {
        x87_store32(e.call(0x0064_63a0, &args![skill]).f64())
    };
    e.set(this, HitData::fPercentBlocked, percent);
    let percent = e.get(this, HitData::fPercentBlocked);
    if above_zero(e, x87_load32(percent)) {
        threshold = x87_store32(x87_add(x87_load32(threshold), x87_m32(percent)));
    }
    set_flag(e, this, 2, drawn);
    threshold
}

/// The entry-point part of `ReduceDamage` (`009b5a30`); returns the threshold.
fn reduce_damage_entry_points(e: &mut Engine, this: Ptr<HitData>, threshold: u32) -> u32 {
    e.with_stack(8, |e, locals| {
        let lowered = locals.addr();
        let raised = lowered + 4;
        e.mem.set_u32(lowered, 0);
        if e.get(this, HitData::pAggressor) != 0 {
            if e.get(this, HitData::pWeapon) == 0 {
                let weapon = e.call(0x0048_39c0, &args![0x1f4u32]).u32();
                e.set(this, HitData::pWeapon, weapon);
            }
            let aggressor = e.get(this, HitData::pAggressor);
            let weapon = e.get(this, HitData::pWeapon);
            let target = e.get(this, HitData::pTarget);
            e.call(
                HANDLE_ENTRY_POINT,
                &args![0x3au32, aggressor, weapon, target, lowered],
            );
        }
        e.mem.set_u32(raised, 0);
        if e.get(this, HitData::pTarget) != 0 {
            let weapon = weapon_or_default(e, this);
            let target = e.get(this, HitData::pTarget);
            let aggressor = e.get(this, HitData::pAggressor);
            e.call(
                HANDLE_ENTRY_POINT,
                &args![0x38u32, target, aggressor, weapon, raised],
            );
        }
        let lowered = e.mem.u32(lowered);
        let raised = e.mem.u32(raised);
        let value = x87_sub(x87_load32(threshold), x87_m32(lowered));
        x87_store32(x87_add(value, x87_m32(raised)))
    })
}

/// The projectile split of `ReduceDamage` (`009b5a30`); returns the threshold.
fn reduce_damage_projectiles(e: &mut Engine, this: Ptr<HitData>, threshold: u32) -> u32 {
    let aggressor = e.get(this, HitData::pAggressor);
    if aggressor == 0 || e.call(0x008d_8520, &args![aggressor]).u32() == 0 {
        return threshold;
    }
    let aggressor = e.get(this, HitData::pAggressor);
    let process = e.call(0x008d_8520, &args![aggressor]).u32();
    if e.vcall(process, 0x148, &args![]).u32() == 0 {
        return threshold;
    }
    let aggressor = e.get(this, HitData::pAggressor);
    let process = e.call(0x008d_8520, &args![aggressor]).u32();
    let item = e.vcall(process, 0x148, &args![]).u32();
    if !e.call(0x004b_da70, &args![item, 0xcu32]).bool() {
        return threshold;
    }
    if !above_zero(e, x87_load32(threshold)) {
        return threshold;
    }
    let weapon = e.get(this, HitData::pWeapon);
    let aggressor = e.get(this, HitData::pAggressor);
    let projectiles = e
        .call(0x0052_5b20, &args![weapon, 1u32, 0u32, aggressor])
        .u8();
    x87_store32(x87_div(x87_m32(threshold), f64::from(projectiles)))
}

/// Shows the hit marker of `ReduceDamage` (`009b5a30`): `00709c60(marker)` for the player as
/// target, else `00709c80(marker)` for the player as aggressor unless the target's vtable slot
/// `0x22c` (with 0) is true.
fn reduce_damage_marker(e: &mut Engine, this: Ptr<HitData>, marker: bool) {
    let player = e.mem.u32(PLAYER_011DEA3C);
    if e.get(this, HitData::pTarget) == player {
        e.call(0x0070_9c60, &args![u32::from(marker)]);
        return;
    }
    let player = e.mem.u32(PLAYER_011DEA3C);
    if e.get(this, HitData::pAggressor) != player {
        return;
    }
    let target = e.get(this, HitData::pTarget);
    if !e.vcall(target, 0x22c, &args![0u32]).bool() {
        e.call(0x0070_9c80, &args![u32::from(marker)]);
    }
}

/// The end of `ReduceDamage` (`009b5a30`), from the ammunition effects of kind 0 on.
fn reduce_damage_finish(
    e: &mut Engine,
    this: Ptr<HitData>,
    minimum: u32,
    threshold: u32,
    marker: bool,
) {
    if e.get(this, HitData::pWeapon) != 0 {
        let effects = ammo_effects(e, this);
        let damage = e.get(this, HitData::fHealthDamage);
        let damage = apply_ammo_effects(e, 0, effects, damage);
        e.set(this, HitData::fHealthDamage, damage);
    }
    if e.get(this, HitData::pWeapon) != 0 {
        let aggressor = e.get(this, HitData::pAggressor);
        let weapon = e.get(this, HitData::pWeapon);
        let target = e.get(this, HitData::pTarget);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![0u32, aggressor, weapon, target, this.addr() + 0x14],
        );
    }
    let fatigue = e.get(this, HitData::fFatigueDamage);
    if at_most_zero(e, x87_load32(fatigue)) {
        let damage = e.get(this, HitData::fHealthDamage);
        let over = x87_store32(x87_sub(x87_load32(damage), x87_m32(minimum)));
        let least = e
            .call(
                0x0040_ebd0,
                &args![x87_quiet_bits(threshold), x87_quiet_bits(over)],
            )
            .f64();
        let scale = setting_bits(e, 0x011c_f0fc);
        let extra = x87_store32(x87_mul(x87_load32(scale), least));
        let armor = e.get(this, HitData::fArmorDamage);
        e.set(
            this,
            HitData::fArmorDamage,
            x87_store32(x87_add(x87_load32(armor), x87_m32(extra))),
        );
    }
    let damage = e.get(this, HitData::fHealthDamage);
    if fpu_compare(x87_load32(minimum), x87_load32(damage)) & 0x41 == 0 {
        e.set(this, HitData::fHealthDamage, x87_quiet_bits(minimum));
        reduce_damage_marker(e, this, marker);
        let aggressor = e.get(this, HitData::pAggressor);
        if aggressor != 0 && e.call(0x0056_6950, &args![aggressor]).bool() {
            let aggressor = e.get(this, HitData::pAggressor);
            e.call(0x008d_5cb0, &args![aggressor, 9u32]);
        }
    } else if marker {
        reduce_damage_marker(e, this, marker);
    }
    let weapon = e.get(this, HitData::pWeapon);
    if weapon != 0 && e.call(0x009b_6600, &args![weapon]).i32() != -1 {
        let hundred = x87_quiet_bits(e.mem.u32(HUNDRED_01016410));
        let weapon = e.get(this, HitData::pWeapon);
        let index = e.call(0x009b_6600, &args![weapon]).u32();
        let target = e.get(this, HitData::pTarget);
        let value = x87_store32(target_actor_value(e, target, index));
        let clamped = e.call(0x0040_ebd0, &args![value, hundred]).f64();
        let fraction = x87_store32(x87_div(clamped, double_at(e, HUNDRED_01017A40)));
        let fraction_value = x87_load32(fraction);
        if above_zero(e, fraction_value)
            && !parity_even(fpu_compare(fraction_value, double_at(e, ONE_01012070)) & 0x41)
        {
            let damage = e.get(this, HitData::fHealthDamage);
            let kept = x87_sub(1.0, fraction_value);
            e.set(
                this,
                HitData::fHealthDamage,
                x87_store32(x87_mul(kept, x87_m32(damage))),
            );
        }
    }
    let fatigue = e.get(this, HitData::fFatigueDamage);
    let total = e.get(this, HitData::fTotalDamage);
    if above_zero(e, x87_load32(fatigue)) && above_zero(e, x87_load32(total)) {
        let damage = e.get(this, HitData::fHealthDamage);
        let total = e.get(this, HitData::fTotalDamage);
        let ratio = x87_store32(x87_div(x87_load32(damage), x87_m32(total)));
        let fatigue = e.get(this, HitData::fFatigueDamage);
        e.set(
            this,
            HitData::fFatigueDamage,
            x87_store32(x87_mul(x87_load32(fatigue), x87_m32(ratio))),
        );
    }
    let player = e.mem.u32(PLAYER_011DEA3C);
    if e.get(this, HitData::pTarget) == player && e.call(0x0052_5430, &args![0x011f_2250u32]).bool()
    {
        for offset in [0x14u32, 0x18, 0x1c, 0x20, 0x28, 0x2c] {
            let scale = setting_bits(e, 0x011c_e9f0);
            let value = field(e, this, offset);
            set_field(
                e,
                this,
                offset,
                x87_store32(x87_mul(x87_load32(value), x87_m32(scale))),
            );
        }
    }
}

// Translated from 009b7010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`): copies the three words at `this + 0x104` (a point) to
/// `result` and returns `result`.
pub fn fn_009b7010(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    for word in 0..3 {
        let value = e.mem.u32(this.addr().wrapping_add(0x104 + 4 * word));
        e.mem.set_u32(result.addr().wrapping_add(4 * word), value);
    }
    result
}

// Translated from 009b7060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::CheckCriticalStrike` (Xbox PDB). Nothing when the target's `+0x94` object's
/// vtable slot `0x10` is true or flag 0x800 is set. The chance is `00646d80(aggressor's
/// ActorValueOwner or 0, weapon or default)`, adjusted by entry points 1 (the aggressor's) and
/// 0x24 (the target's), plus setting `011ce46c` for a VATS hit, times 10; a sneak attack (the
/// aggressor's vtable slot `0x360`, a character target that is not slot `0x22c`, `004997b0` on
/// the aggressor, not already in combat with it, with no detection
/// (`Actor::GetDetectionLevelAgainstActor` after `Actor::LineOfSight`) multiplies it by
/// setting `011ce858` and sets flag 0x400. A roll `00487f50() % 1000` below the chance
/// (`_ftol2`) is a critical: flag 4, and with a weapon or the default weapon the critical
/// damage (the weapon's `00645c20`, or the health damage times setting `011ce57c` without
/// one) goes through entry point 2 and, when positive, is added to the health damage; flag 8
/// becomes `009b73b0(weapon)` and the critical effect `0051f4d0(weapon)`.
pub fn hit_data_check_critical_strike(e: &mut Engine, this: Ptr<HitData>) {
    let target = e.get(this, HitData::pTarget);
    if target != 0 && e.vcall(target.wrapping_add(0x94), 0x10, &args![]).bool() {
        return;
    }
    if e.call(IS_FLAG_SET, &args![this, 0x800u32]).bool() {
        return;
    }
    e.with_stack(8, |e, locals| {
        let chance_slot = locals.addr();
        let bonus_slot = chance_slot + 4;
        e.mem.set_u32(bonus_slot, 0);
        let weapon = weapon_or_default(e, this);
        let aggressor = e.get(this, HitData::pAggressor);
        let owner = if aggressor != 0 {
            aggressor.wrapping_add(0xa4)
        } else {
            0
        };
        let chance = x87_store32(e.call(0x0064_6d80, &args![owner, weapon]).f64());
        e.mem.set_u32(chance_slot, chance);
        let weapon = weapon_or_default(e, this);
        let aggressor = e.get(this, HitData::pAggressor);
        let target = e.get(this, HitData::pTarget);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![1u32, aggressor, weapon, target, chance_slot],
        );
        let weapon = weapon_or_default(e, this);
        let target = e.get(this, HitData::pTarget);
        let aggressor = e.get(this, HitData::pAggressor);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![0x24u32, target, weapon, aggressor, chance_slot],
        );
        let mut chance = e.mem.u32(chance_slot);
        if e.get(this, HitData::pVATSCommand) != 0 {
            let bonus = setting_bits(e, 0x011c_e46c);
            chance = x87_store32(x87_add(x87_load32(chance), x87_m32(bonus)));
        }
        chance = x87_store32(x87_mul(x87_load32(chance), double_at(e, TEN_01020758)));
        if critical_sneak_attack(e, this) {
            let scale = setting_bits(e, 0x011c_e858);
            chance = x87_store32(x87_mul(x87_load32(chance), x87_m32(scale)));
            set_flag(e, this, 0x400, 1);
        }
        let roll = e.call(0x0048_7f50, &args![]).u32() % 1000;
        let limit = e.call(FTOL, &args![x87_load32(chance)]).u32() as i32;
        if roll as i32 >= limit {
            return;
        }
        set_flag(e, this, 4, 1);
        let weapon = weapon_or_default(e, this);
        if weapon == 0 {
            return;
        }
        let damage = if e.get(this, HitData::pWeapon) != 0 {
            let own = e.get(this, HitData::pWeapon);
            f64::from(e.call(0x0064_5c20, &args![own]).u16())
        } else {
            let scale = setting_bits(e, 0x011c_e57c);
            let health = e.get(this, HitData::fHealthDamage);
            x87_mul(x87_load32(health), x87_m32(scale))
        };
        e.mem.set_u32(bonus_slot, x87_store32(damage));
        let aggressor = e.get(this, HitData::pAggressor);
        let target = e.get(this, HitData::pTarget);
        e.call(
            HANDLE_ENTRY_POINT,
            &args![2u32, aggressor, weapon, target, bonus_slot],
        );
        let bonus = e.mem.u32(bonus_slot);
        if above_zero(e, x87_load32(bonus)) {
            let health = e.get(this, HitData::fHealthDamage);
            e.set(
                this,
                HitData::fHealthDamage,
                x87_store32(x87_add(x87_load32(health), x87_m32(bonus))),
            );
        }
        let critical = e.call(0x009b_73b0, &args![weapon]).u8();
        set_flag(e, this, 8, u32::from(critical));
        let effect = e.call(0x0051_f4d0, &args![weapon]).u32();
        e.set(this, HitData::pCriticalEffect, effect);
    });
}

/// The sneak-attack test of `CheckCriticalStrike` (`009b7060`).
fn critical_sneak_attack(e: &mut Engine, this: Ptr<HitData>) -> bool {
    let aggressor = e.get(this, HitData::pAggressor);
    if aggressor == 0 || !e.vcall(aggressor, 0x360, &args![]).bool() {
        return false;
    }
    let target = e.get(this, HitData::pTarget);
    if target == 0 || !e.vcall(target, 0x100, &args![]).bool() {
        return false;
    }
    let target = e.get(this, HitData::pTarget);
    if e.vcall(target, 0x22c, &args![0u32]).bool() {
        return false;
    }
    let aggressor = e.get(this, HitData::pAggressor);
    if !e.call(0x0049_97b0, &args![aggressor]).bool() {
        return false;
    }
    let target = e.get(this, HitData::pTarget);
    let aggressor = e.get(this, HitData::pAggressor);
    if e.call(0x008b_c700, &args![target, aggressor]).bool() {
        return false;
    }
    e.with_stack(4, |e, sight| {
        let target = e.get(this, HitData::pTarget);
        let aggressor = e.get(this, HitData::pAggressor);
        let seen = e
            .call(
                0x0088_b880,
                &args![target, 0u32, aggressor, 1u32, 0u32, 0u32],
            )
            .u8();
        e.mem.set_u8(sight.addr(), seen);
        let target = e.get(this, HitData::pTarget);
        let aggressor = e.get(this, HitData::pAggressor);
        let level = e
            .call(
                0x008a_0d10,
                &args![target, 0u32, aggressor, sight, 0u32, 0u32, 0u32, 0u32],
            )
            .i32();
        level <= 0
    })
}

// Translated from 009b73d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::DetermineOutcome` (Xbox PDB). Only for a character target (vtable slot `0x100`)
/// whose `+0x94` object's slot `0x10` is false. A positive bonus multiplier applies to the
/// health, total and (for a non-`0x21c` aggressor with skill 0x2d) fatigue damage, first scaled
/// by setting `011ceae0` (no weapon or a `006450c0` weapon) or `011cf794` when it is at least 1
/// and flag 0x400 is set. A VATS command with more than one shot multiplies the damage for the
/// kill test. A hit on limb 0xe (the weapon): when the target's item (`0044ddc0` of its process's
/// slot `0x148` item) would break (`ItemChange::GetItemHealth(item, 0)` minus the limb damage at
/// most 0), flag 0x200 (slot `0x21c` true or `0046e8c0`) or 0x100. Another limb (not -1) of the
/// target's body part data (`004181e0(target)` slot `0x180`, `BGSBodyPartData::GetBodyPart`) that
/// is `008b4360`: the dismember chance (`008b4cf0`), explode flag (`008b4cd0`), and a crippled
/// limb (the limb's actor value, `005e5190`, above 0 and the limb damage taking it to at most 0)
/// sets flag 0x80. When the health (actor value 0x10) minus the scaled damage is at most 0: flag
/// 0x10, and for a limb the dismember / explode outcome (`00646e50`, unless the chance is 100 and
/// the limb explodes) sets flags 0x20 and 0x40.
pub fn hit_data_determine_outcome(e: &mut Engine, this: Ptr<HitData>) {
    let target = e.get(this, HitData::pTarget);
    if target == 0 || !e.vcall(target, 0x100, &args![]).bool() {
        return;
    }
    let target = e.get(this, HitData::pTarget);
    if e.vcall(target.wrapping_add(0x94), 0x10, &args![]).bool() {
        return;
    }
    let bonus = e.get(this, HitData::fBonusMult);
    if above_zero(e, x87_load32(bonus)) {
        apply_bonus(e, this);
    }
    let mut shots = 1.0f32.to_bits();
    let command = e.get(this, HitData::pVATSCommand);
    if command != 0 && i32::from(e.mem.u8(command.wrapping_add(8))) > 1 {
        let count = e.mem.u8(command.wrapping_add(8));
        shots = x87_store32(f64::from(count));
    }
    let mut chance = 0u32;
    let mut sure = false;
    let mut explodes = 0u8;
    let limb = e.get(this, HitData::eDamageLimb);
    if limb == 0xe {
        outcome_weapon_limb(e, this);
    } else if limb != u32::MAX {
        if let Some((part_chance, part_explodes)) = outcome_body_part(e, this) {
            chance = part_chance;
            explodes = part_explodes;
            sure = explodes != 0 && chance as i32 >= 100;
        }
    }
    let target = e.get(this, HitData::pTarget);
    let health = x87_store32(target_actor_value(e, target, 0x10));
    let damage = e.get(this, HitData::fHealthDamage);
    let scaled = x87_mul(x87_load32(damage), x87_m32(shots));
    let left = x87_sub(x87_m32(health), scaled);
    if !at_most_zero(e, left) {
        return;
    }
    set_flag(e, this, 0x10, 1);
    if e.get(this, HitData::eDamageLimb) == u32::MAX {
        return;
    }
    e.with_stack(4, |e, flags| {
        let dismember = flags.addr();
        let explode = dismember + 1;
        e.mem.set_u8(dismember, 0);
        e.mem.set_u8(explode, u8::from(sure));
        if !sure {
            let weapon = match e.get(this, HitData::pWeapon) {
                0 => {
                    if e.get(this, HitData::eAttackSkill) == 0x2d {
                        e.mem.u32(DEFAULT_WEAPON_011CA278)
                    } else {
                        0
                    }
                }
                weapon => weapon,
            };
            let target = e.get(this, HitData::pTarget);
            let owner = if target != 0 {
                target.wrapping_add(0xa4)
            } else {
                0
            };
            e.call(
                0x0064_6e50,
                &args![
                    owner,
                    weapon,
                    chance,
                    0u32,
                    0u32,
                    u32::from(explodes),
                    dismember,
                    explode
                ],
            );
        }
        if e.mem.u8(dismember) != 0 {
            set_flag(e, this, 0x20, 1);
        }
        if e.mem.u8(explode) != 0 {
            set_flag(e, this, 0x40, 1);
        }
    });
}

/// The bonus-multiplier part of `DetermineOutcome` (`009b73d0`).
fn apply_bonus(e: &mut Engine, this: Ptr<HitData>) {
    let bonus = e.get(this, HitData::fBonusMult);
    let at_least_one = fpu_compare(x87_load32(bonus), double_at(e, ONE_01012070)) & 0x01 == 0;
    if at_least_one && e.call(IS_FLAG_SET, &args![this, 0x400u32]).bool() {
        let weapon = e.get(this, HitData::pWeapon);
        let setting = if weapon == 0 || e.call(0x0064_50c0, &args![weapon]).bool() {
            0x011c_eae0
        } else {
            0x011c_f794
        };
        let scale = setting_bits(e, setting);
        let bonus = e.get(this, HitData::fBonusMult);
        e.set(
            this,
            HitData::fBonusMult,
            x87_store32(x87_mul(x87_load32(bonus), x87_m32(scale))),
        );
    }
    let scale_field = |e: &mut Engine, offset: u32| {
        let value = field(e, this, offset);
        let bonus = e.get(this, HitData::fBonusMult);
        set_field(
            e,
            this,
            offset,
            x87_store32(x87_mul(x87_load32(value), x87_m32(bonus))),
        );
    };
    scale_field(e, 0x14);
    let aggressor = e.get(this, HitData::pAggressor);
    if aggressor != 0
        && !e.vcall(aggressor, 0x21c, &args![]).bool()
        && e.get(this, HitData::eAttackSkill) == 0x2d
    {
        scale_field(e, 0x1c);
    }
    scale_field(e, 0x18);
}

/// The weapon-limb (0xe) part of `DetermineOutcome` (`009b73d0`).
fn outcome_weapon_limb(e: &mut Engine, this: Ptr<HitData>) {
    let target = e.get(this, HitData::pTarget);
    let process = e.call(0x008d_8520, &args![target]).u32();
    let item = e.vcall(process, 0x148, &args![]).u32();
    let form = if item != 0 {
        e.call(0x0044_ddc0, &args![item]).u32()
    } else {
        0
    };
    if form == 0 {
        return;
    }
    let health = x87_store32(e.call(0x004b_cdb0, &args![item, 0u32]).f64());
    let limb_damage = e.get(this, HitData::fTargetedLimbDamage);
    if !at_most_zero(e, x87_sub(x87_load32(health), x87_m32(limb_damage))) {
        return;
    }
    let target = e.get(this, HitData::pTarget);
    if e.vcall(target, 0x21c, &args![]).bool() || e.call(0x0046_e8c0, &args![form]).bool() {
        set_flag(e, this, 0x200, 1);
    } else {
        set_flag(e, this, 0x100, 1);
    }
}

/// The body-part part of `DetermineOutcome` (`009b73d0`): the dismember chance and explode flag
/// when the target's body part for the limb is `008b4360`.
fn outcome_body_part(e: &mut Engine, this: Ptr<HitData>) -> Option<(u32, u8)> {
    let target = e.get(this, HitData::pTarget);
    let data = e.call(0x0041_81e0, &args![target]).u32();
    let parts = e.vcall(data, 0x180, &args![]).u32();
    if parts == 0 {
        return None;
    }
    let limb = e.get(this, HitData::eDamageLimb);
    let part = e.call(0x005e_50f0, &args![parts, limb]).u32();
    if part == 0 || !e.call(0x008b_4360, &args![part]).bool() {
        return None;
    }
    let chance = u32::from(e.call(0x008b_4cf0, &args![part]).u8());
    let explodes = e.call(0x008b_4cd0, &args![part]).u8();
    let index = e.call(0x005e_5190, &args![part]).u8() as i8 as i32 as u32;
    let target = e.get(this, HitData::pTarget);
    let value = x87_store32(target_actor_value(e, target, index));
    if above_zero(e, x87_load32(value)) {
        let limb_damage = e.get(this, HitData::fTargetedLimbDamage);
        if at_most_zero(e, x87_sub(x87_load32(value), x87_m32(limb_damage))) {
            set_flag(e, this, 0x80, 1);
        }
    }
    Some((chance, explodes))
}

/// `0066de30` (`ActionPoints::IsMeleeAttackAction`, engine map; cdecl(action)).
const IS_MELEE_ATTACK_ACTION: u32 = 0x0066_de30;
/// `008d8520` (thiscall): the actor's process.
const GET_PROCESS: u32 = 0x008d_8520;
/// `00476c90` (thiscall): a projectile's weapon.
const PROJECTILE_WEAPON: u32 = 0x0047_6c90;
/// `ItemChange::GetItemHealth` (`004bcdb0`, thiscall(byte), ST0).
const GET_ITEM_HEALTH: u32 = 0x004b_cdb0;
/// `00598040` (thiscall on the aggressor, ST0): the aggressor's damage
/// factor.
const AGGRESSOR_DAMAGE_FACTOR: u32 = 0x0059_8040;
/// `GetClampedActorValue` (`0066ef20`, thiscall on an `ActorValueOwner`).
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
/// `CombatFormulas::GetPowerAttackDamageMultiplier` (`00644520`, cdecl(value,
/// animation type), ST0).
const POWER_ATTACK_MULTIPLIER: u32 = 0x0064_4520;
/// `004997b0` (thiscall on the aggressor, byte; no Xbox PDB name).
const AGGRESSOR_TEST_004997B0: u32 = 0x0049_97b0;
/// `006450c0` (thiscall on the weapon, byte; no Xbox PDB name).
const WEAPON_TEST_006450C0: u32 = 0x0064_50c0;
/// `BSSimpleArray` constructor body `006b3eb0` (thiscall(0, 0)).
const ARRAY_INITIALIZE: u32 = 0x006b_3eb0;
/// `operator delete` (`00401030`, cdecl).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The body-part data's part for a limb (`005e50f0`, thiscall(limb)).
const GET_BODY_PART: u32 = 0x005e_50f0;
/// A body part's limb (`005e5300`, thiscall).
const BODY_PART_LIMB: u32 = 0x005e_5300;
/// The target's limb for a node (`008b3ef0`, thiscall(node)).
const LIMB_FOR_NODE: u32 = 0x008b_3ef0;
/// `NiMatrix`-free point copy (`00458620`, cdecl(dest, source)).
const POINT_COPY: u32 = 0x0045_8620;
/// `006815c0` (thiscall): returns `this` (a point's or a hit's own address).
const SAME_ADDRESS: u32 = 0x0068_15c0;
/// `200.0` (`float`): the limb ray's length.
const LIMB_RAY_LENGTH: u32 = 0x0102_226c;
/// `6.0` (`float`): the divisor of the fallback hit location's offset.
const LIMB_FALLBACK_SCALE: u32 = 0x0105_0c48;
/// `Actor::GetWeaponPosition` (`008a5340`, thiscall(out, byte)).
const GET_WEAPON_POSITION: u32 = 0x008a_5340;
/// `NiPoint3` difference (`00439ef0`, thiscall(out, other): `this - other`).
const POINT_SUBTRACT: u32 = 0x0043_9ef0;

// Translated from 009b4fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The health damage a VATS command would do (cdecl(`command`), a `float`
/// in ST0; no Xbox PDB name): 0 without a command; otherwise a stack
/// `HitData` (constructed, flags 0x800 and 0x1000 set) runs
/// [`hit_data_initialize_hit_data`] with the command, whose byte +4 is 1
/// for the call and restored after, and its `fHealthDamage` is the result.
pub fn fn_009b4fe0(e: &mut Engine, command: Ptr) -> f32 {
    if command.is_null() {
        return 0.0;
    }
    e.with_stack(0x70, |e, frame| {
        let hit: Ptr<HitData> = Ptr::new(frame.addr());
        hit_data_constructor(e, hit);
        set_flag(e, hit, 0x800, 1);
        set_flag(e, hit, 0x1000, 1);
        let saved = e.mem.u8(command.addr() + 4);
        e.mem.set_u8(command.addr() + 4, 1);
        hit_data_initialize_hit_data(e, hit, command);
        e.mem.set_u8(command.addr() + 4, saved);
        f32::from_bits(x87_quiet_bits(e.get(hit, HitData::fHealthDamage)))
    })
}

// Translated from 009b5050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::InitializeHitData` (Xbox PDB; cdecl(`hit`, `command`)): for a
/// hit and a VATS command with a target (+0xC), stores the command
/// (`pVATSCommand`) and runs [`hit_data_initialize_hit_data_ov3`] with the
/// player as aggressor, the target when it is a character (virtual +0x100)
/// else 0, the player's process's virtual +0x148 item, and whether the
/// command's action is a melee attack (`IsMeleeAttackAction`) with its byte
/// +4 set.
pub fn hit_data_initialize_hit_data(e: &mut Engine, hit: Ptr<HitData>, command: Ptr) {
    if hit.is_null() || command.is_null() || e.mem.u32(command.addr() + 0xc) == 0 {
        return;
    }
    e.set(hit, HitData::pVATSCommand, command.addr());
    let action = e.mem.u32(command.addr());
    let melee =
        e.call(IS_MELEE_ATTACK_ACTION, &args![action]).bool() && e.mem.u8(command.addr() + 4) != 0;
    let target = e.mem.u32(command.addr() + 0xc);
    let character = if e.vcall(target, 0x100, &args![]).bool() {
        e.mem.u32(command.addr() + 0xc)
    } else {
        0
    };
    let player = e.mem.u32(PLAYER_011DEA3C);
    let process = e.call(GET_PROCESS, &args![player]).u32();
    let item = e.vcall(process, 0x148, &args![]).u32();
    let player = e.mem.u32(PLAYER_011DEA3C);
    hit_data_initialize_hit_data_ov3(e, hit, player, character, item, u32::from(melee));
}

// Translated from 009b5120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::InitializeHitData_ov2` (Xbox PDB; cdecl(`hit`, `aggressor`,
/// `target`, `item`, `power_attack`, `projectile`)):
/// [`hit_data_initialize_hit_data_ov4`] for a projectile, else
/// [`hit_data_initialize_hit_data_ov3`] with the power-attack byte.
pub fn hit_data_initialize_hit_data_ov2(
    e: &mut Engine,
    hit: Ptr<HitData>,
    aggressor: u32,
    target: u32,
    item: u32,
    power_attack: u32,
    projectile: u32,
) {
    if projectile != 0 {
        hit_data_initialize_hit_data_ov4(e, hit, aggressor, target, item, projectile);
    } else {
        hit_data_initialize_hit_data_ov3(e, hit, aggressor, target, item, power_attack & 0xff);
    }
}

// Translated from 009b5170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::InitializeHitData_ov3` (Xbox PDB; cdecl(`hit`, `aggressor`,
/// `target`, `item`, `power_attack` (its low byte))): the melee and unarmed
/// hit. Stores aggressor and target; the weapon is the item's object when
/// it is of form type 0x28 (`0044ddc0`, `00401170`), its health the item's
/// (`GetItemHealth(1)`, 1.0 without an item). The hit counts (`ok`) without
/// a VATS command, with one whose byte +4 is set, or with one whose byte +4
/// is clear when there is no weapon or `006450c0` answers true. The
/// aggressor's animation type comes from `005f2440(004301b0(virtual
/// +0x1E4, 4))`.
///
/// When it counts: with a weapon, the attack skill is `008d85e0(weapon)`,
/// the power-attack multiplier (unless the aggressor's virtual +0x21C or
/// `004997b0` answers true) `GetPowerAttackDamageMultiplier(skill value,
/// animation type)`, the damage `GetModifiedAttackDamage(owner, multiplier,
/// 0)` of the item, scaled by `command +8 / 00524b60(weapon)` (integer
/// division) for a VATS hit with flag 0x1000 and more than one shot, by the
/// setting at `011cf5d0` for `006450c0` and `00524b40`, and by the action's
/// factor (0x10: `008d9140(weapon)`, 0x11/0x14/0x15: the settings at
/// `011d0424`, `011d12d8`, `011d07b4`). Without a weapon: the aggressor's
/// virtual +0x444 as the damage when its virtual +0x21C answers true, else
/// `CombatFormulas::CalcHandToHandDamage` (`00646310`; the target's
/// virtual +0x230 byte) into the health and fatigue damage, skill 0x2D, and
/// the power-attack multiplier raises the bonus (`009b7840`) unless
/// `004997b0`. Then the aggressor's factor (`00598040`), the weapon damage
/// (`00646260(aggressor)` into +0x2C, for an aggressor whose virtual +0x360
/// answers true with a weapon `0046e8c0` says no to), the player factor for
/// `00566950(aggressor)`, `CheckCriticalStrike` and `ReduceDamage(0)`.
/// Always [`hit_data_set_targeted_limb`] with no projectile, and
/// `DetermineOutcome` when the hit counts.
pub fn hit_data_initialize_hit_data_ov3(
    e: &mut Engine,
    hit: Ptr<HitData>,
    aggressor: u32,
    target: u32,
    item: u32,
    power_attack: u32,
) {
    if hit.is_null() {
        return;
    }
    let power_attack = power_attack as u8;
    e.set(hit, HitData::pAggressor, aggressor);
    e.set(hit, HitData::pTarget, target);
    let mut weapon = 0;
    if item != 0 {
        let object = e.call(0x0044_ddc0, &args![item]).u32();
        if e.call(0x0040_1170, &args![object]).u32() == 0x28 {
            weapon = e.call(0x0044_ddc0, &args![item]).u32();
        }
    }
    e.set(hit, HitData::pWeapon, weapon);
    let health = if item != 0 {
        x87_store32(e.call(GET_ITEM_HEALTH, &args![item, 1u32]).f64())
    } else {
        x87_store32(1.0)
    };
    e.set(hit, HitData::fWeaponHealth, x87_quiet_bits(health));
    let command = e.get(hit, HitData::pVATSCommand);
    let mut counts = if command != 0 {
        e.mem.u8(command.wrapping_add(4)) != 0
    } else {
        true
    };
    let command = e.get(hit, HitData::pVATSCommand);
    if command != 0 && e.mem.u8(command.wrapping_add(4)) == 0 {
        let weapon = e.get(hit, HitData::pWeapon);
        if weapon == 0 || e.call(WEAPON_TEST_006450C0, &args![weapon]).bool() {
            counts = true;
        }
    }
    let animation = e.vcall(aggressor, 0x1e4, &args![]).u32();
    let group = e.call(0x0043_01b0, &args![animation, 4u32]).u16();
    let animation_type = e.call(0x005f_2440, &args![u32::from(group)]).u32();
    if counts {
        if e.get(hit, HitData::pWeapon) != 0 {
            weapon_damage(e, hit, aggressor, item, power_attack, animation_type);
        } else {
            unarmed_damage(e, hit, aggressor, target, power_attack, animation_type);
        }
        if aggressor != 0 {
            let factor = e.call(AGGRESSOR_DAMAGE_FACTOR, &args![aggressor]).f64();
            let damage = e.get(hit, HitData::fHealthDamage);
            let damage = x87_store32(x87_mul(factor, x87_m32(damage)));
            e.set(hit, HitData::fHealthDamage, damage);
        }
        if aggressor != 0
            && e.vcall(aggressor, 0x360, &args![]).bool()
            && e.get(hit, HitData::pWeapon) != 0
        {
            let weapon = e.get(hit, HitData::pWeapon);
            if !e.call(0x0046_e8c0, &args![weapon]).bool() {
                let aggressor = e.get(hit, HitData::pAggressor);
                let damage = e.call(0x0064_6260, &args![aggressor]).f64();
                e.set(hit, HitData::fDamageToWeapon, x87_store32(damage));
            }
        }
        if e.call(0x0056_6950, &args![aggressor]).bool() {
            let factor = player_factor(e);
            let damage = e.get(hit, HitData::fHealthDamage);
            let damage = x87_store32(x87_mul(x87_load32(damage), x87_m32(factor)));
            e.set(hit, HitData::fHealthDamage, damage);
        }
        hit_data_check_critical_strike(e, hit);
        hit_data_reduce_damage(e, hit, 0);
    }
    hit_data_set_targeted_limb(e, hit, 0);
    if counts {
        hit_data_determine_outcome(e, hit);
    }
}

/// The power-attack multiplier of [`hit_data_initialize_hit_data_ov3`]:
/// `GetPowerAttackDamageMultiplier(GetClampedActorValue(skill),
/// animation type)` stored as a `float`, or 1.0 without a power attack.
fn power_multiplier(
    e: &mut Engine,
    hit: Ptr<HitData>,
    aggressor: u32,
    power_attack: u8,
    animation_type: u32,
) -> u32 {
    if power_attack != 0 {
        let skill = e.get(hit, HitData::eAttackSkill);
        let value = e
            .call(
                GET_CLAMPED_ACTOR_VALUE,
                &args![aggressor.wrapping_add(0xa4), skill],
            )
            .u32();
        let multiplier = e
            .call(POWER_ATTACK_MULTIPLIER, &args![value, animation_type])
            .f64();
        x87_store32(multiplier)
    } else {
        x87_store32(1.0)
    }
}

/// The armed branch of [`hit_data_initialize_hit_data_ov3`].
fn weapon_damage(
    e: &mut Engine,
    hit: Ptr<HitData>,
    aggressor: u32,
    item: u32,
    power_attack: u8,
    animation_type: u32,
) {
    let mut multiplier = x87_store32(1.0);
    let weapon = e.get(hit, HitData::pWeapon);
    let skill = e.call(0x008d_85e0, &args![weapon]).u32();
    e.set(hit, HitData::eAttackSkill, skill);
    if !e.vcall(aggressor, 0x21c, &args![]).bool()
        && !e.call(AGGRESSOR_TEST_004997B0, &args![aggressor]).bool()
    {
        let value = power_multiplier(e, hit, aggressor, power_attack, animation_type);
        multiplier = x87_quiet_bits(value);
    }
    let owner = if aggressor != 0 {
        aggressor.wrapping_add(0xa4)
    } else {
        0
    };
    let damage = e
        .call(
            0x004b_df00,
            &args![item, owner, x87_quiet_bits(multiplier), 0u32],
        )
        .f64();
    e.set(hit, HitData::fHealthDamage, x87_store32(damage));
    let command = e.get(hit, HitData::pVATSCommand);
    if command != 0
        && i32::from(e.mem.u8(command.wrapping_add(8))) > 1
        && e.call(IS_FLAG_SET, &args![hit, 0x1000u32]).bool()
    {
        let command = e.get(hit, HitData::pVATSCommand);
        let shots = i32::from(e.mem.u8(command.wrapping_add(8)));
        let weapon = e.get(hit, HitData::pWeapon);
        let per_shot = i32::from(e.call(0x0052_4b60, &args![weapon]).u8());
        // `CDQ`/`IDIV`: signed division (a zero divisor faults in the exe).
        let ratio = shots / per_shot;
        let damage = e.get(hit, HitData::fHealthDamage);
        let damage = x87_store32(x87_mul(f64::from(ratio), x87_m32(damage)));
        e.set(hit, HitData::fHealthDamage, damage);
    }
    if e.get(hit, HitData::pVATSCommand) != 0 {
        let weapon = e.get(hit, HitData::pWeapon);
        if e.call(WEAPON_TEST_006450C0, &args![weapon]).bool() {
            let weapon = e.get(hit, HitData::pWeapon);
            if e.call(0x0052_4b40, &args![weapon]).bool() {
                scale_health_damage(e, hit, 0x011c_f5d0);
            }
        }
    }
    let command = e.get(hit, HitData::pVATSCommand);
    if command != 0 {
        match e.mem.u32(command) {
            0x10 => {
                let weapon = e.get(hit, HitData::pWeapon);
                let factor = e.call(0x008d_9140, &args![weapon]).f64();
                let damage = e.get(hit, HitData::fHealthDamage);
                let damage = x87_store32(x87_mul(factor, x87_m32(damage)));
                e.set(hit, HitData::fHealthDamage, damage);
            }
            0x11 => scale_health_damage(e, hit, 0x011d_0424),
            0x14 => scale_health_damage(e, hit, 0x011d_12d8),
            0x15 => scale_health_damage(e, hit, 0x011d_07b4),
            _ => {}
        }
    }
}

/// `fHealthDamage *= setting` (`FLD` the damage, `FMUL` the setting's
/// `float`, `FSTP`).
fn scale_health_damage(e: &mut Engine, hit: Ptr<HitData>, setting: u32) {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    let damage = e.get(hit, HitData::fHealthDamage);
    let damage = x87_store32(x87_mul(x87_load32(damage), x87_m32(e.mem.u32(value))));
    e.set(hit, HitData::fHealthDamage, damage);
}

/// The unarmed branch of [`hit_data_initialize_hit_data_ov3`].
fn unarmed_damage(
    e: &mut Engine,
    hit: Ptr<HitData>,
    aggressor: u32,
    target: u32,
    power_attack: u8,
    animation_type: u32,
) {
    if e.vcall(aggressor, 0x21c, &args![]).bool() {
        let damage = e.vcall(aggressor, 0x444, &args![]).i32();
        e.set(hit, HitData::fHealthDamage, x87_store32(f64::from(damage)));
        return;
    }
    let blocked = if target != 0 {
        e.vcall(target, 0x230, &args![]).u8()
    } else {
        0
    };
    let owner = if aggressor != 0 {
        aggressor.wrapping_add(0xa4)
    } else {
        0
    };
    e.call(
        0x0064_6310,
        &args![
            owner,
            u32::from(blocked),
            hit.addr() + 0x14,
            hit.addr() + 0x1c
        ],
    );
    e.set(hit, HitData::eAttackSkill, 0x2d);
    let multiplier = power_multiplier(e, hit, aggressor, power_attack, animation_type);
    let multiplier = x87_quiet_bits(multiplier);
    if !e.call(AGGRESSOR_TEST_004997B0, &args![aggressor]).bool() {
        fn_009b7840(e, hit, f32::from_bits(x87_quiet_bits(multiplier)));
    }
}

// Translated from 009b5650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::InitializeHitData_ov4` (Xbox PDB; cdecl(`hit`, `aggressor`,
/// `target`, `item`, `projectile`)): the projectile hit. Stores aggressor
/// and target, the projectile's weapon (`00476c90`, 0 without a
/// projectile), the item's health (`GetItemHealth(1)`, 1.0 without one),
/// the projectile as source, the projectile's damage (`00885d70`) times
/// the aggressor's factor (`00598040`, with an aggressor) as health and
/// total damage; then `CheckCriticalStrike`, `ReduceDamage(1)`,
/// [`hit_data_set_targeted_limb`] with the projectile and
/// `DetermineOutcome`.
pub fn hit_data_initialize_hit_data_ov4(
    e: &mut Engine,
    hit: Ptr<HitData>,
    aggressor: u32,
    target: u32,
    item: u32,
    projectile: u32,
) {
    if hit.is_null() {
        return;
    }
    e.set(hit, HitData::pAggressor, aggressor);
    e.set(hit, HitData::pTarget, target);
    let weapon = if projectile != 0 {
        e.call(PROJECTILE_WEAPON, &args![projectile]).u32()
    } else {
        0
    };
    e.set(hit, HitData::pWeapon, weapon);
    let health = if item != 0 {
        x87_store32(e.call(GET_ITEM_HEALTH, &args![item, 1u32]).f64())
    } else {
        x87_store32(1.0)
    };
    e.set(hit, HitData::fWeaponHealth, x87_quiet_bits(health));
    e.set(hit, HitData::pSourceRef, projectile);
    let damage = e.call(0x0088_5d70, &args![projectile]).f64();
    e.set(hit, HitData::fHealthDamage, x87_store32(damage));
    if aggressor != 0 {
        let factor = e.call(AGGRESSOR_DAMAGE_FACTOR, &args![aggressor]).f64();
        let damage = e.get(hit, HitData::fHealthDamage);
        let damage = x87_store32(x87_mul(factor, x87_m32(damage)));
        e.set(hit, HitData::fHealthDamage, damage);
    }
    let damage = e.get(hit, HitData::fHealthDamage);
    e.set(hit, HitData::fTotalDamage, x87_quiet_bits(damage));
    hit_data_check_critical_strike(e, hit);
    hit_data_reduce_damage(e, hit, 1);
    hit_data_set_targeted_limb(e, hit, projectile);
    hit_data_determine_outcome(e, hit);
}

// Translated from 009b5720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::InitializeHitData_ov5` (Xbox PDB; cdecl(`hit`, `aggressor`,
/// `target`, `damage`)): aggressor, target, the aggressor as source, the
/// damage as health and total damage, then `ReduceDamage(0)` and
/// `DetermineOutcome`.
pub fn hit_data_initialize_hit_data_ov5(
    e: &mut Engine,
    hit: Ptr<HitData>,
    aggressor: u32,
    target: u32,
    damage: f32,
) {
    if hit.is_null() {
        return;
    }
    e.set(hit, HitData::pAggressor, aggressor);
    e.set(hit, HitData::pTarget, target);
    e.set(hit, HitData::pSourceRef, aggressor);
    let damage = x87_quiet_bits(damage.to_bits());
    e.set(hit, HitData::fHealthDamage, damage);
    e.set(hit, HitData::fTotalDamage, damage);
    hit_data_reduce_damage(e, hit, 0);
    hit_data_determine_outcome(e, hit);
}

// Translated from 009b58d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HitData::InitializeImpactData` (Xbox PDB; cdecl(`hit`, a word it does
/// not read, `target`, `damage`, `impact`)): no aggressor, the target, the
/// damage as health and total damage. The hit location is the target's
/// virtual +0x1F4 point moved a quarter of the way toward the impact point
/// (`impact +4`; `00439ef0`, `004a3760(0.25)`, `0063c8a0`) with the impact's
/// z; the direction is the impact's (+0x10). With a target, the limb is
/// `008b3ef0(008b3d70(impact point - direction, impact point))`. Then
/// `ReduceDamage(1)` and `DetermineOutcome`.
pub fn hit_data_initialize_impact_data(
    e: &mut Engine,
    hit: Ptr<HitData>,
    _unused_1: u32,
    target: u32,
    damage: f32,
    impact: u32,
) {
    if hit.is_null() {
        return;
    }
    e.with_stack(0x38, |e, frame| {
        let ebp = frame.addr() + 0x38;
        let location = ebp - 0xc;
        e.set(hit, HitData::pAggressor, 0);
        e.set(hit, HitData::pTarget, target);
        let damage = x87_quiet_bits(damage.to_bits());
        e.set(hit, HitData::fHealthDamage, damage);
        e.set(hit, HitData::fTotalDamage, damage);
        let point = e.vcall(target, 0x1f4, &args![]).u32();
        for word in 0..3 {
            let value = e.mem.u32(point.wrapping_add(4 * word));
            e.mem.set_u32(location + 4 * word, value);
        }
        let point = e.vcall(target, 0x1f4, &args![]).u32();
        let offset = e
            .call(
                POINT_SUBTRACT,
                &args![impact.wrapping_add(4), ebp - 0x1c, point],
            )
            .u32();
        let quarter = x87_quiet_bits(e.mem.u32(0x0101_622c));
        let scaled = e
            .call(0x004a_3760, &args![ebp - 0x28, quarter, offset])
            .u32();
        e.call(0x0063_c8a0, &args![location, scaled]);
        let z = x87_quiet_bits(e.mem.u32(impact.wrapping_add(0xc)));
        e.mem.set_u32(location + 8, z);
        for word in 0..3 {
            let value = e.mem.u32(location + 4 * word);
            set_field(e, hit, HIT_LOCATION + 4 * word, value);
            let value = e.mem.u32(impact.wrapping_add(0x10 + 4 * word));
            set_field(e, hit, HIT_DIRECTION + 4 * word, value);
        }
        if target != 0 {
            let start: Vec<u32> = (0..3)
                .map(|word| e.mem.u32(impact.wrapping_add(4 + 4 * word)))
                .collect();
            let back = e
                .call(
                    POINT_SUBTRACT,
                    &args![
                        impact.wrapping_add(4),
                        ebp - 0x34,
                        impact.wrapping_add(0x10)
                    ],
                )
                .u32();
            let end: Vec<u32> = (0..3).map(|word| e.mem.u32(back + 4 * word)).collect();
            let node = e
                .call(
                    0x008b_3d70,
                    &args![target, end[0], end[1], end[2], start[0], start[1], start[2]],
                )
                .u32();
            let limb = e.call(LIMB_FOR_NODE, &args![target, node]).u32();
            e.set(hit, HitData::eDamageLimb, limb);
        }
    });
    hit_data_reduce_damage(e, hit, 1);
    hit_data_determine_outcome(e, hit);
}

// Translated from 009b65d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The larger of a 16-bit value (zero-extended) and an integer (cdecl;
/// signed compare, `JLE`): `max(value, minimum)`.
pub fn fn_009b65d0(_e: &mut Engine, value: u16, minimum: i32) -> i32 {
    if i32::from(value) > minimum {
        i32::from(value)
    } else {
        minimum
    }
}

// Translated from 009b6620 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b6620 - C++ exception unwinding (FS:[0] frame state) and the /GS security-cookie check (00ec408c) are not translated.
/// `HitData::SetTargetedLimb` (Xbox PDB; thiscall(`projectile`), `RET 4`).
/// Only for a character target (virtual +0x100). The limb comes from, in
/// order: a VATS command's body part (+0x10; `005e5130` on the target's
/// body-part data, the base object's virtual +0x180; 0xE without a part)
/// or, for one without a part, 0 for a melee action with flag 0x800, else
/// a ray (melee) or 0xE; a projectile's impact on the target (`009c3ee0`:
/// its limb +0x24 and part, the hit location +4, the direction
/// `009b7010` + the impact normal, normalized); otherwise a ray.
///
/// The ray: from the aggressor's eye (`008aff80`) along its eye vector
/// (200 units, `0045bb20`) through a hit caster (`008d6f30`, `00626dd0`,
/// `00626d20`, `00626b30`, `00621210`, `005609f0`); each hit whose
/// reference is the target (`004b5820`, `0056f930`) and whose body
/// `004b59f0` accepts moves the weapon position (`00458620`), sets the hit
/// location and direction once, and adds its limb's body part (not 0xE)
/// to a weighted random (`009b7990`, weight `005dc980`) unless it is
/// there (`009b79d0`). Without such a hit, with both 3D roots: the hit
/// location is the target's bound center plus (weapon position - center)
/// / 6 and the direction center - aggressor center, and a pick
/// (`00c806b0`, 200 units) adds its node's part the same way (limb -1
/// excepted). With a hit, the limb is the picked part's (`00822650`) or
/// -1.
///
/// Then a limb 0xE moves the health damage to the targeted limb damage
/// (times the setting at `011ce5d0` for VATS) and clears it; a part gives
/// the limb and the limb damage `00647a30(target owner, part, health
/// damage)` (times the setting at `011cfa40` when the target's virtual
/// +0x360 says so, times `009b7040(weapon)` with a weapon) and raises the
/// bonus to the part's `00639b20` (1.0 without a weapon or for
/// `006450c0`).
pub fn hit_data_set_targeted_limb(e: &mut Engine, hit: Ptr<HitData>, projectile: u32) {
    let target = e.get(hit, HitData::pTarget);
    if target == 0 || !e.vcall(target, 0x100, &args![]).bool() {
        return;
    }
    e.with_stack(0x200, |e, frame| {
        let ebp = frame.addr() + 0x200;
        let target = e.get(hit, HitData::pTarget);
        let base = e.call(0x0041_81e0, &args![target]).u32();
        let parts = e.vcall(base, 0x180, &args![]).u32();
        let mut part = 0u32;
        let mut cast_ray = false;
        let command = e.get(hit, HitData::pVATSCommand);
        if command != 0 {
            let wanted = e.mem.u32(command.wrapping_add(0x10));
            if wanted != u32::MAX {
                if parts != 0 {
                    part = e.call(0x005e_5130, &args![parts, wanted]).u32();
                    let limb = if part != 0 {
                        e.call(BODY_PART_LIMB, &args![part]).u32()
                    } else {
                        0xe
                    };
                    e.set(hit, HitData::eDamageLimb, limb);
                }
            } else {
                let action = e.mem.u32(command);
                if e.call(IS_MELEE_ATTACK_ACTION, &args![action]).bool() {
                    if e.call(IS_FLAG_SET, &args![hit, 0x800u32]).bool() {
                        e.set(hit, HitData::eDamageLimb, 0);
                    } else {
                        cast_ray = true;
                    }
                } else {
                    e.set(hit, HitData::eDamageLimb, 0xe);
                }
            }
        } else if projectile != 0 {
            let target = e.get(hit, HitData::pTarget);
            let impact = e.call(0x009c_3ee0, &args![projectile, target]).u32();
            if impact != 0 {
                let limb = e.mem.u32(impact.wrapping_add(0x24));
                e.set(hit, HitData::eDamageLimb, limb);
                part = e.call(GET_BODY_PART, &args![parts, limb]).u32();
                for word in 0..3 {
                    let value = e.mem.u32(impact.wrapping_add(4 + 4 * word));
                    set_field(e, hit, HIT_LOCATION + 4 * word, value);
                }
                let direction = fn_009b7010(e, Ptr::new(projectile), Ptr::new(ebp - 0x3c));
                for word in 0..3 {
                    let value = e.mem.u32(direction.addr() + 4 * word);
                    set_field(e, hit, HIT_DIRECTION + 4 * word, value);
                }
                let direction = hit.addr() + HIT_DIRECTION;
                e.call(0x004a_0c10, &args![direction]);
                let normal = e
                    .call(0x004a_0bd0, &args![impact.wrapping_add(0x10), ebp - 0x48])
                    .u32();
                e.call(0x0063_c8a0, &args![direction, normal]);
                e.call(0x004a_0c10, &args![direction]);
            }
        } else {
            cast_ray = true;
        }
        if cast_ray {
            if let Some(picked) = limb_ray(e, hit, parts, ebp) {
                part = picked;
            }
        }
        apply_limb(e, hit, part);
    });
}

/// The ray of [`hit_data_set_targeted_limb`] on its frame (`ebp`): `None`
/// when nothing was hit (the limb stays), else the picked body part (0
/// for none, the limb then -1).
fn limb_ray(e: &mut Engine, hit: Ptr<HitData>, parts: u32, ebp: u32) -> Option<u32> {
    let weapon_position = ebp - 0x54;
    let eye = ebp - 0x60;
    let eye_vector = ebp - 0x6c;
    let start = ebp - 0x78;
    let random = Ptr::new(ebp - 0xb0);
    let aggressor = e.get(hit, HitData::pAggressor);
    e.call(
        GET_WEAPON_POSITION,
        &args![aggressor, weapon_position, 1u32],
    );
    e.call(SAME_ADDRESS, &args![eye]);
    e.call(SAME_ADDRESS, &args![eye_vector]);
    let aggressor = e.get(hit, HitData::pAggressor);
    e.call(0x008a_ff80, &args![aggressor, eye, eye_vector, 0u32]);
    for word in 0..3 {
        let value = e.mem.u32(eye + 4 * word);
        e.mem.set_u32(start + 4 * word, value);
    }
    let length = x87_quiet_bits(e.mem.u32(LIMB_RAY_LENGTH));
    e.call(0x0045_bb20, &args![eye_vector, ebp - 0x84, length]);
    let length = x87_quiet_bits(e.mem.u32(LIMB_RAY_LENGTH));
    let mut found = false;
    fn_009b7870(e, random);
    let aggressor = e.get(hit, HitData::pAggressor);
    let caster = e.call(0x008d_6f30, &args![aggressor]).u32();
    if caster != 0 {
        let aggressor = e.get(hit, HitData::pAggressor);
        let not_player = u32::from(aggressor != e.mem.u32(PLAYER_011DEA3C));
        let cast = e.call(0x0062_6dd0, &args![not_player, 1u32]).u32();
        let source = e.call(0x0045_43c0, &args![caster]).u32();
        e.call(0x0062_6d20, &args![cast, source]);
        e.call(0x0062_6b30, &args![cast, start, eye_vector, length]);
        let collector = e.call(SAME_ADDRESS, &args![cast]).u32();
        let result = e.call(0x0062_1210, &args![collector, ebp - 0xbd]).u32();
        if e.call(0x0056_09f0, &args![result]).bool() {
            let mut index = 0i32;
            loop {
                let hits = e.call(0x0046_0140, &args![collector]).u32();
                let count = e.call(0x0072_6070, &args![hits]).i32();
                if index >= count {
                    break;
                }
                let hits = e.call(0x0046_0140, &args![collector]).u32();
                let entry = e.call(0x0080_7770, &args![hits, index as u32]).u32();
                let local = ebp - 0x140;
                e.call(0x0063_2d40, &args![local, entry]);
                let collidable = e.mem.u32(ebp - 0xf8);
                let body = e.call(0x004b_5820, &args![collidable]).u32();
                let reference = e.call(0x0056_f930, &args![body]).u32();
                if reference != 0 && reference == e.get(hit, HitData::pTarget) {
                    let point = e.call(SAME_ADDRESS, &args![local]).u32();
                    e.call(POINT_COPY, &args![weapon_position, point]);
                    let collidable = e.mem.u32(ebp - 0xf8);
                    if e.call(0x004b_59f0, &args![collidable]).u32() != 0 {
                        if !found {
                            found = true;
                            let point = e.call(SAME_ADDRESS, &args![local]).u32();
                            e.call(POINT_COPY, &args![hit.addr() + HIT_LOCATION, point]);
                            for word in 0..3 {
                                let value = e.mem.u32(eye_vector + 4 * word);
                                set_field(e, hit, HIT_DIRECTION + 4 * word, value);
                            }
                        }
                        let limb = node_limb(e, hit, body);
                        if limb != u32::MAX && limb != 0xe {
                            add_part(e, random, parts, limb);
                        }
                    }
                }
                index += 1;
            }
        }
    }
    if !found {
        let target = e.get(hit, HitData::pTarget);
        let target_root = e.vcall(target, 0x1d0, &args![]).u32() != 0;
        if target_root {
            let aggressor = e.get(hit, HitData::pAggressor);
            if e.vcall(aggressor, 0x1d0, &args![]).u32() != 0 {
                found = pick_limb(e, hit, parts, random, ebp);
            }
        }
    }
    let picked = if found {
        let part = e.call(0x0082_2650, &args![random]).u32();
        let limb = if part != 0 {
            e.call(BODY_PART_LIMB, &args![part]).u32()
        } else {
            u32::MAX
        };
        e.set(hit, HitData::eDamageLimb, limb);
        Some(part)
    } else {
        None
    };
    fn_009b78e0(e, random);
    picked
}

/// The target's limb for a collision body: its node (virtual +0xC) or,
/// without one, `009611e0(body)`, through `008b3ef0`.
fn node_limb(e: &mut Engine, hit: Ptr<HitData>, body: u32) -> u32 {
    let node = if e.vcall(body, 0xc, &args![]).u32() != 0 {
        e.vcall(body, 0xc, &args![]).u32()
    } else {
        e.call(0x0096_11e0, &args![body]).u32()
    };
    let target = e.get(hit, HitData::pTarget);
    e.call(LIMB_FOR_NODE, &args![target, node]).u32()
}

/// Adds the limb's body part to the weighted random once (weight
/// `005dc980`).
fn add_part(e: &mut Engine, random: Ptr, parts: u32, limb: u32) {
    let part = e.call(GET_BODY_PART, &args![parts, limb]).u32();
    if part != 0 && !fn_009b79d0(e, random, part) {
        let weight = e.call(0x005d_c980, &args![part]).u8();
        fn_009b7990(e, random, part, u32::from(weight));
    }
}

/// The fallback of the limb ray: the hit location and direction from the
/// two bound centers and a pick from the weapon position.
fn pick_limb(e: &mut Engine, hit: Ptr<HitData>, parts: u32, random: Ptr, ebp: u32) -> bool {
    let target_center = ebp - 0x164;
    let aggressor_center = ebp - 0x170;
    for (actor, out) in [
        (e.get(hit, HitData::pTarget), target_center),
        (e.get(hit, HitData::pAggressor), aggressor_center),
    ] {
        let root = e.vcall(actor, 0x1d0, &args![]).u32();
        let bound = e.call(0x0043_d450, &args![root]).u32();
        let center = e.call(SAME_ADDRESS, &args![bound]).u32();
        for word in 0..3 {
            let value = e.mem.u32(center.wrapping_add(4 * word));
            e.mem.set_u32(out + 4 * word, value);
        }
    }
    let aggressor = e.get(hit, HitData::pAggressor);
    let position = e
        .call(GET_WEAPON_POSITION, &args![aggressor, ebp - 0x194, 1u32])
        .u32();
    let offset = e
        .call(POINT_SUBTRACT, &args![position, ebp - 0x188, target_center])
        .u32();
    let scale = x87_quiet_bits(e.mem.u32(LIMB_FALLBACK_SCALE));
    let scaled = e
        .call(0x0053_d280, &args![offset, ebp - 0x17c, scale])
        .u32();
    let location = e
        .call(0x0043_9e90, &args![target_center, ebp - 0x1a0, scaled])
        .u32();
    for word in 0..3 {
        let value = e.mem.u32(location + 4 * word);
        set_field(e, hit, HIT_LOCATION + 4 * word, value);
    }
    let direction = e
        .call(
            POINT_SUBTRACT,
            &args![target_center, ebp - 0x1ac, aggressor_center],
        )
        .u32();
    for word in 0..3 {
        let value = e.mem.u32(direction + 4 * word);
        set_field(e, hit, HIT_DIRECTION + 4 * word, value);
    }
    let ray = ebp - 0x1c0;
    let origin = ebp - 0x1d0;
    e.call(0x0062_40d0, &args![ray]);
    e.call(SAME_ADDRESS, &args![origin]);
    e.call(0x004a_3e00, &args![origin, ebp - 0x54]);
    e.call(0x008a_50f0, &args![ray, origin]);
    let length = x87_quiet_bits(e.mem.u32(LIMB_RAY_LENGTH));
    e.call(0x008a_50d0, &args![ray, length]);
    let target = e.get(hit, HitData::pTarget);
    let root = e.vcall(target, 0x1d0, &args![]).u32();
    let picked = e.call(0x00c8_06b0, &args![root, ray, 0u32]).u32();
    if picked == 0 {
        return false;
    }
    let collidable = e.call(0x0043_b560, &args![picked]).u32();
    let body = e.call(0x004b_5820, &args![collidable]).u32();
    if body != 0 {
        let limb = node_limb(e, hit, body);
        if limb != u32::MAX {
            add_part(e, random, parts, limb);
        }
    }
    true
}

/// The damage part of [`hit_data_set_targeted_limb`].
fn apply_limb(e: &mut Engine, hit: Ptr<HitData>, part: u32) {
    if e.get(hit, HitData::eDamageLimb) == 0xe {
        let damage = x87_quiet_bits(e.get(hit, HitData::fHealthDamage));
        e.set(hit, HitData::fTargetedLimbDamage, damage);
        if e.get(hit, HitData::pVATSCommand) != 0 {
            let value = e.call(SETTING_VALUE, &args![0x011c_e5d0u32]).u32();
            let damage = e.get(hit, HitData::fTargetedLimbDamage);
            let damage = x87_store32(x87_mul(x87_load32(damage), x87_m32(e.mem.u32(value))));
            e.set(hit, HitData::fTargetedLimbDamage, damage);
        }
        e.set(hit, HitData::fHealthDamage, x87_store32(0.0));
        return;
    }
    if part == 0 {
        return;
    }
    // The exe tests the part again here.
    let limb = if part != 0 {
        e.call(BODY_PART_LIMB, &args![part]).u32()
    } else {
        u32::MAX
    };
    e.set(hit, HitData::eDamageLimb, limb);
    let target = e.get(hit, HitData::pTarget);
    let owner = if target != 0 {
        target.wrapping_add(0xa4)
    } else {
        0
    };
    let damage = x87_quiet_bits(e.get(hit, HitData::fHealthDamage));
    let limb_damage = e.call(0x0064_7a30, &args![owner, part, damage]).f64();
    e.set(hit, HitData::fTargetedLimbDamage, x87_store32(limb_damage));
    let target = e.get(hit, HitData::pTarget);
    if e.vcall(target, 0x360, &args![]).bool() {
        let value = e.call(SETTING_VALUE, &args![0x011c_fa40u32]).u32();
        let damage = e.get(hit, HitData::fTargetedLimbDamage);
        let damage = x87_store32(x87_mul(x87_load32(damage), x87_m32(e.mem.u32(value))));
        e.set(hit, HitData::fTargetedLimbDamage, damage);
    }
    let mut bonus = x87_store32(e.call(0x0063_9b20, &args![part]).f64());
    let weapon = e.get(hit, HitData::pWeapon);
    if weapon == 0 || e.call(WEAPON_TEST_006450C0, &args![weapon]).bool() {
        bonus = x87_store32(1.0);
    }
    let weapon = e.get(hit, HitData::pWeapon);
    if weapon != 0 {
        let factor = fn_009b7040(e, Ptr::new(weapon));
        let damage = e.get(hit, HitData::fTargetedLimbDamage);
        let damage = x87_store32(x87_mul(f64::from(factor), x87_m32(damage)));
        e.set(hit, HitData::fTargetedLimbDamage, damage);
    }
    fn_009b7840(e, hit, f32::from_bits(x87_quiet_bits(bonus)));
}

// Translated from 009b7040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0x168 of `this` (thiscall, ST0; the hit code calls it on
/// the weapon).
pub fn fn_009b7040(e: &mut Engine, this: Ptr) -> f32 {
    f32::from_bits(x87_quiet_bits(e.mem.u32(this.addr().wrapping_add(0x168))))
}

// Translated from 009b73b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x1C8 of `this` (thiscall; the hit code calls it on the
/// weapon).
pub fn fn_009b73b0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x1c8))
}

// Translated from 009b7840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Raises `fBonusMult` to `value` (thiscall(`value`), `RET 4`): `FCOMPP` of
/// the bonus with `value`, `TEST AH,5`/`JP`; only an ordered "below"
/// stores `value`.
pub fn fn_009b7840(e: &mut Engine, this: Ptr<HitData>, value: f32) {
    let bonus = e.get(this, HitData::fBonusMult);
    if x87_load32(bonus) < x87_load32(value.to_bits()) {
        e.set(this, HitData::fBonusMult, x87_quiet_bits(value.to_bits()));
    }
}

// Translated from 009b7870 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b7870 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The `BSWeightedRandom<BGSBodyPart*>` constructor (thiscall; no Xbox PDB
/// name): the results array ([`fn_009b7940`]), the weights array at +0x10
/// (`00438660`) and the total weight +0x20 = 0. Returns `this`.
pub fn fn_009b7870(e: &mut Engine, this: Ptr) -> Ptr {
    fn_009b7940(e, this);
    e.call(0x0043_8660, &args![this.addr() + 0x10]);
    e.mem.set_u32(this.addr() + 0x20, 0);
    this
}

// Translated from 009b78e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 009b78e0 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// The `BSWeightedRandom<BGSBodyPart*>` destructor (thiscall; the engine
/// map's name on this folded body is unrelated): the weights array's
/// destructor (`00438690` on +0x10), then the results array's
/// ([`fn_009b7970`]).
pub fn fn_009b78e0(e: &mut Engine, this: Ptr) {
    e.call(0x0043_8690, &args![this.addr() + 0x10]);
    fn_009b7970(e, this);
}

// Translated from 009b7940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSSimpleArray<BGSBodyPart*,1024>` constructor (thiscall): its
/// vtable, then `006b3eb0(this, 0, 0)`. Returns `this`.
pub fn fn_009b7940(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0108F9E8);
    e.call(ARRAY_INITIALIZE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 009b7990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWeightedRandom<BGSBodyPart*>::AddWeightedResult` (Xbox PDB;
/// thiscall(`result`, `weight`), `RET 8`): adds `weight` to the total
/// (+0x20), then appends the weight to the weights array (+0x10) and the
/// result to the results array ([`fn_007cb2e0`], each given the address of
/// its stack argument).
pub fn fn_009b7990(e: &mut Engine, this: Ptr, result: u32, weight: u32) {
    let total = e.mem.u32(this.addr() + 0x20).wrapping_add(weight);
    e.mem.set_u32(this.addr() + 0x20, total);
    e.with_stack(8, |e, arguments| {
        e.mem.set_u32(arguments.addr(), result);
        e.mem.set_u32(arguments.addr() + 4, weight);
        fn_007cb2e0(
            e,
            Ptr::new(this.addr() + 0x10),
            Ptr::new(arguments.addr() + 4),
        );
        fn_007cb2e0(e, Ptr::new(this.addr()), arguments);
    });
}

// Translated from 009b79d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the weighted random already holds `result` (thiscall(`result`),
/// `RET 4`): `009962f0(this, &result)`.
pub fn fn_009b79d0(e: &mut Engine, this: Ptr, result: u32) -> bool {
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), result);
        e.call(0x0099_62f0, &args![this, argument]).bool()
    })
}

// Translated from 009b79f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<BGSBodyPart*,1024>::scalar deleting destructor` (Xbox
/// PDB): the destructor ([`fn_009b7970`]), then `operator delete(this)` when
/// bit 0 of `flags` is set. Returns `this`.
pub fn bs_simple_array_body_part_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_009b7970(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007c_b2e0, fn_007cb2e0(Ptr<BSSimpleArray>, Ptr) -> u32),
        entry!(0x009b_7970, fn_009b7970(Ptr)),
        entry!(
            0x009a_4320,
            bs_simple_array_body_part_move_items(Ptr, u32, u32)
        ),
        entry!(
            0x009b_4d90,
            hit_data_constructor(Ptr<HitData>) -> Ptr<HitData>
        ),
        entry!(0x009b_4dc0, hit_data_clear_data(Ptr<HitData>)),
        entry!(0x009b_4ec0, hit_data_copy(Ptr<HitData>, Ptr<HitData>)),
        entry!(
            0x009b_5770,
            hit_data_initialize_explosion_data(Ptr<HitData>, u32, u32, u32, f32)
        ),
        entry!(0x009b_5a30, hit_data_reduce_damage(Ptr<HitData>, u32)),
        entry!(0x009b_7010, fn_009b7010(Ptr, Ptr) -> Ptr),
        entry!(0x009b_7060, hit_data_check_critical_strike(Ptr<HitData>)),
        entry!(0x009b_73d0, hit_data_determine_outcome(Ptr<HitData>)),
        entry!(0x009b_4fe0, fn_009b4fe0(Ptr) -> f32),
        entry!(0x009b_5050, hit_data_initialize_hit_data(Ptr<HitData>, Ptr)),
        entry!(
            0x009b_5120,
            hit_data_initialize_hit_data_ov2(Ptr<HitData>, u32, u32, u32, u32, u32)
        ),
        entry!(
            0x009b_5170,
            hit_data_initialize_hit_data_ov3(Ptr<HitData>, u32, u32, u32, u32)
        ),
        entry!(
            0x009b_5650,
            hit_data_initialize_hit_data_ov4(Ptr<HitData>, u32, u32, u32, u32)
        ),
        entry!(
            0x009b_5720,
            hit_data_initialize_hit_data_ov5(Ptr<HitData>, u32, u32, f32)
        ),
        entry!(
            0x009b_58d0,
            hit_data_initialize_impact_data(Ptr<HitData>, u32, u32, f32, u32)
        ),
        entry!(0x009b_65d0, fn_009b65d0(u16, i32) -> i32),
        entry!(0x009b_6620, hit_data_set_targeted_limb(Ptr<HitData>, u32)),
        entry!(0x009b_7040, fn_009b7040(Ptr) -> f32),
        entry!(0x009b_73b0, fn_009b73b0(Ptr) -> u8),
        entry!(0x009b_7840, fn_009b7840(Ptr<HitData>, f32)),
        entry!(0x009b_7870, fn_009b7870(Ptr) -> Ptr),
        entry!(0x009b_78e0, fn_009b78e0(Ptr)),
        entry!(0x009b_7940, fn_009b7940(Ptr) -> Ptr),
        entry!(0x009b_7990, fn_009b7990(Ptr, u32, u32)),
        entry!(0x009b_79d0, fn_009b79d0(Ptr, u32) -> bool),
        entry!(
            0x009b_79f0,
            bs_simple_array_body_part_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod array_destructor_tests {
    use super::*;

    fn check_array_destructor(address: u32, vtable: u32, release: u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.register(release, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), vtable);
        let hits: Vec<_> = log.iter().filter(|c| c.0 == release).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![this, 1]);
    }

    #[test]
    fn fn_009b7970_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x009b_7970, ARRAY_VTABLE_0108F9E8, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;

    #[test]
    fn at_007cb2e0_reserves_a_slot_constructs_it_and_stores_the_pointer() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let array = e.mem.alloc(0x10);
        let buffer = e.mem.alloc(0x20);
        e.mem.set_u32(array + 4, buffer);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0xabcd_0001);
        e.register(0x0076_1540, |_, _| 3u32.into_ret());
        e.register(0x0072_6bf0, |_, _| Ret::default());
        let index = e.call(0x007c_b2e0, &args![array, item]).u32();
        assert_eq!(index, 3);
        assert_eq!(e.mem.u32(buffer + 12), 0xabcd_0001);
        let log = e.call_log.take().unwrap();
        let reserve: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == 0x0076_1540)
            .map(|c| &c.1)
            .collect();
        assert_eq!(reserve, vec![&vec![array]]);
        let construct: Vec<&Vec<u32>> = log
            .iter()
            .filter(|c| c.0 == 0x0072_6bf0)
            .map(|c| &c.1)
            .collect();
        assert_eq!(construct, vec![&vec![array, buffer + 12, 1]]);
    }

    #[test]
    fn at_007cb2e0_stores_into_the_buffer_the_reservation_moved_to() {
        let mut e = Engine::new();
        let array = e.mem.alloc(0x10);
        let old_buffer = e.mem.alloc(0x20);
        let new_buffer = e.mem.alloc(0x20);
        e.mem.set_u32(array + 4, old_buffer);
        let item = e.mem.alloc(4);
        e.mem.set_u32(item, 0x5555_aaaa);
        e.register_double(0x0076_1540, move |e, a| {
            e.mem.set_u32(a[0] + 4, new_buffer);
            0u32.into_ret()
        });
        e.register(0x0072_6bf0, |_, _| Ret::default());
        assert_eq!(e.call(0x007c_b2e0, &args![array, item]).u32(), 0);
        assert_eq!(e.mem.u32(new_buffer), 0x5555_aaaa);
        assert_eq!(e.mem.u32(old_buffer), 0);
    }
}

#[cfg(test)]
mod b035_tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;

    fn calls(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    const SLOT_BASE: u32 = 0x7100_0000;

    type Answers = Rc<RefCell<HashMap<(u32, u32), Ret>>>;

    /// Test objects sharing one vtable whose slots answer from a table by (object, slot).
    struct World {
        vtable: u32,
        answers: Answers,
    }

    impl World {
        fn new(e: &mut Engine) -> World {
            let vtable = e.mem.alloc(0x400);
            let answers: Answers = Default::default();
            for slot in (0..0x400).step_by(4) {
                e.mem.set_u32(vtable + slot, SLOT_BASE + slot);
                let table = answers.clone();
                e.register_double(SLOT_BASE + slot, move |_, a| {
                    table
                        .borrow()
                        .get(&(a[0], slot))
                        .copied()
                        .unwrap_or_default()
                });
            }
            World { vtable, answers }
        }

        /// An object with a vtable at `+0`, `+0x94` and `+0xa4`.
        fn object(&self, e: &mut Engine) -> u32 {
            let object = e.mem.alloc(0x100);
            for offset in [0, 0x94, 0xa4] {
                e.mem.set_u32(object + offset, self.vtable);
            }
            object
        }

        fn answer(&self, object: u32, slot: u32, value: u32) {
            self.answers.borrow_mut().insert(
                (object, slot),
                Ret {
                    eax: value,
                    ..Ret::default()
                },
            );
        }

        fn answer_float(&self, object: u32, slot: u32, value: f64) {
            self.answers.borrow_mut().insert(
                (object, slot),
                Ret {
                    st0: value,
                    ..Ret::default()
                },
            );
        }
    }

    fn float_ret(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn constants(e: &mut Engine) {
        for page in [
            0x0101_2000u32,
            0x0101_6000,
            0x0101_7000,
            0x0102_0000,
            0x0103_c000,
            0x011c_a000,
            0x011d_e000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.set_u64(ZERO_01012060, 0f64.to_bits());
        e.mem.set_u64(ONE_01012070, 1f64.to_bits());
        e.mem.set_u64(HUNDRED_01017A40, 100f64.to_bits());
        e.mem.set_u32(HUNDRED_01016410, 100f32.to_bits());
        e.mem.set_u64(TEN_01020758, 10f64.to_bits());
        e.mem
            .set_u64(FIVE_HUNDREDTHS_0103C7C8, f64::from(0.05f32).to_bits());
    }

    /// Settings: `00403e20` answers the address of a float holding the value set here.
    fn settings(e: &mut Engine, values: &[(u32, f32)]) {
        let table: HashMap<u32, u32> = values
            .iter()
            .map(|(setting, value)| {
                let slot = e.mem.alloc(4);
                e.mem.set_u32(slot, value.to_bits());
                (*setting, slot)
            })
            .collect();
        e.register_double(SETTING_VALUE, move |_, a| table[&a[0]].into_ret());
    }

    #[test]
    fn move_items_destroys_and_moves_the_tail() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0x4000);
        e.mem.set_u32(this + 8, 10);
        e.register(0x0042_f5a0, |_, a| a[0].min(a[1]).into_ret());
        quiet(&mut e, &[0x0072_ba80, 0x0042_fb60]);
        e.call_log = Some(vec![]);
        e.call(0x009a_4320, &args![this, 2u32, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x0042_f5a0), vec![vec![5, 3]]);
        assert_eq!(calls(&log, 0x0072_ba80), vec![vec![this, 0x4008, 3]]);
        assert_eq!(
            calls(&log, 0x0042_fb60),
            vec![vec![this, 0x4008, 0x401c, 3]]
        );
        assert_eq!(e.mem.u32(this + 8), 7);
    }

    #[test]
    fn fn_009b7010_copies_the_point() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x110);
        for word in 0..3 {
            e.mem.set_u32(this + 0x104 + 4 * word, 0x10 + word);
        }
        let result = e.mem.alloc(0xc);
        assert_eq!(e.call(0x009b_7010, &args![this, result]).u32(), result);
        assert_eq!(
            (
                e.mem.u32(result),
                e.mem.u32(result + 4),
                e.mem.u32(result + 8)
            ),
            (0x10, 0x11, 0x12)
        );
    }

    #[test]
    fn constructor_clears_and_copy_copies() {
        let mut e = Engine::new();
        constants(&mut e);
        for word in 0..3 {
            e.mem.set_u32(0x011f_426c + 4 * word, 0x100 + word);
        }
        quiet(&mut e, &[0x0068_15c0]);
        let hit = e.mem.alloc(0x64);
        for offset in (0..0x64).step_by(4) {
            e.mem.set_u32(hit + offset, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009b_4d90, &args![hit]).u32(), hit);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, 0x0068_15c0),
            vec![vec![hit + 0x38], vec![hit + 0x44]]
        );
        let words: Vec<u32> = (0..0x64)
            .step_by(4)
            .map(|offset| e.mem.u32(hit + offset))
            .collect();
        let mut expected = vec![0u32; 25];
        expected[3] = 0x4d;
        expected[4] = u32::MAX;
        for word in 0..3 {
            expected[14 + word] = 0x100 + word as u32;
            expected[17 + word] = 0x100 + word as u32;
        }
        assert_eq!(words, expected);
        let other = e.mem.alloc(0x64);
        for offset in (0..0x64).step_by(4) {
            e.mem.set_u32(other + offset, 0x1000 + offset);
        }
        e.mem.set_u32(other + 0x14, 0x7f80_0001);
        e.call(0x009b_4ec0, &args![hit, other]);
        for offset in (0..0x60).step_by(4) {
            let expected = if offset == 0x14 {
                0x7fc0_0001
            } else {
                0x1000 + offset
            };
            assert_eq!(e.mem.u32(hit + offset), expected, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u32(hit + 0x60), 0);
    }

    #[test]
    fn initialize_explosion_data_fills_the_hit_and_runs_the_steps() {
        let mut e = Engine::new();
        constants(&mut e);
        let world = World::new(&mut e);
        let explosion = world.object(&mut e);
        world.answer(explosion, 0x220, 1);
        e.register(0x009b_0900, |_, _| 0x600u32.into_ret());
        let owner = world.object(&mut e);
        world.answer(owner, 0x100, 1);
        e.register_double(0x009a_cce0, move |_, _| owner.into_ret());
        e.register(0x008a_1710, |_, _| 0x600u32.into_ret());
        let process = world.object(&mut e);
        e.register_double(0x008d_8520, move |_, _| process.into_ret());
        world.answer(process, 0x148, 0x700);
        e.register(0x004b_cdb0, |_, _| float_ret(0.75));
        // The critical check stops at flag 0x800; with no target the other steps return.
        e.register(IS_FLAG_SET, |_, _| 1u32.into_ret());
        quiet(&mut e, &[SET_FLAG]);
        let hit = e.mem.alloc(0x64);
        e.call_log = Some(vec![]);
        e.call(0x009b_5770, &args![hit, 0x10u32, 0u32, explosion, 12.5f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SET_FLAG), vec![vec![hit, 0x2000, 1]]);
        assert_eq!(calls(&log, 0x004b_cdb0), vec![vec![0x700, 1]]);
        let hit = Ptr::<HitData>::new(hit);
        assert_eq!(e.get(hit, HitData::pAggressor), 0x10);
        assert_eq!(e.get(hit, HitData::pWeapon), 0x600);
        assert_eq!(e.get(hit, HitData::fWeaponHealth), 0.75f32.to_bits());
        assert_eq!(e.get(hit, HitData::pSourceRef), explosion);
        assert_eq!(e.get(hit, HitData::fHealthDamage), 12.5f32.to_bits());
        assert_eq!(e.get(hit, HitData::fTotalDamage), 12.5f32.to_bits());
        // A null hit does nothing.
        e.call_log = Some(vec![]);
        e.call(0x009b_5770, &args![0u32, 0x10u32, 0u32, explosion, 12.5f32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    /// A hit of 100 damage by `aggressor` on `target` (both world objects).
    fn reduce_world(e: &mut Engine, world: &World) -> (Ptr<HitData>, u32, u32) {
        constants(e);
        let aggressor = world.object(e);
        let target = world.object(e);
        world.answer(target, 0x100, 1);
        let hit = Ptr::<HitData>::new(e.mem.alloc(0x64));
        e.set(hit, HitData::pAggressor, aggressor);
        e.set(hit, HitData::pTarget, target);
        e.set(hit, HitData::fHealthDamage, 100.0f32.to_bits());
        settings(
            e,
            &[
                (0x011c_ed4c, 0.2),
                (0x011d_1020, 85.0),
                (0x011c_f0fc, 0.5),
                (0x011c_e9f0, 1.0),
            ],
        );
        e.register(0x0040_ebd0, |_, a| {
            float_ret(f64::from(f32::from_bits(a[0]).min(f32::from_bits(a[1]))))
        });
        e.register(0x0040_4010, |_, a| {
            float_ret(f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))))
        });
        e.register(0x0064_62e0, |_, _| float_ret(7.0));
        let process = world.object(e);
        e.register_double(0x008d_8520, move |_, _| process.into_ret());
        quiet(
            e,
            &[
                0x0056_6950,
                0x0089_4d60,
                HANDLE_ENTRY_POINT,
                0x0070_9c60,
                0x0070_9c80,
                0x0048_39c0,
                0x0052_5430,
            ],
        );
        e.register(0x009b_6600, |_, _| u32::MAX.into_ret());
        (hit, aggressor, target)
    }

    #[test]
    fn reduce_damage_applies_resistance_and_threshold() {
        let mut e = Engine::new();
        let world = World::new(&mut e);
        let (hit, aggressor, target) = reduce_world(&mut e, &world);
        // Resistance 30 (0.3), threshold 10: 100 * 0.7 - 10 = 60.
        let values = Rc::new(RefCell::new(HashMap::from([
            (0x12u32, 30.0f64),
            (0x4c, 10.0),
        ])));
        let table = values.clone();
        e.register_double(SLOT_BASE + 0xc, move |_, a| {
            float_ret(table.borrow().get(&a[1]).copied().unwrap_or(0.0))
        });
        e.call_log = Some(vec![]);
        e.call(0x009b_5a30, &args![hit, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(hit, HitData::fHealthDamage), 60.0f32.to_bits());
        assert_eq!(e.get(hit, HitData::fTotalDamage), 100.0f32.to_bits());
        // Armor damage: 7 from 006462e0, plus 0.5 * min(10, 60 - 20).
        assert_eq!(e.get(hit, HitData::fArmorDamage), 12.0f32.to_bits());
        assert_eq!(
            calls(&log, 0x0064_62e0),
            vec![vec![100.0f32.to_bits(), 0.3f32.to_bits()]]
        );
        // Entry points 0x3a (with the default 004839c0(500) weapon) and 0x38.
        let entries = calls(&log, HANDLE_ENTRY_POINT);
        assert_eq!(entries[0][..4], [0x3a, aggressor, 0, target]);
        assert_eq!(entries[1][..4], [0x38, target, aggressor, 0]);
        assert!(calls(&log, 0x0070_9c60).is_empty());
        // A huge threshold leaves the minimum (0.2 * 100); the player as target sees the hit
        // marker (0: the threshold was not used up by the entry points).
        values.borrow_mut().insert(0x4c, 500.0);
        let player = target;
        e.mem.set_u32(PLAYER_011DEA3C, player);
        e.set(hit, HitData::fHealthDamage, 100.0f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x009b_5a30, &args![hit, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(hit, HitData::fHealthDamage), 20.0f32.to_bits());
        assert_eq!(calls(&log, 0x0070_9c60), vec![vec![0]]);
    }

    #[test]
    fn reduce_damage_spares_an_immune_target() {
        let mut e = Engine::new();
        let world = World::new(&mut e);
        let (hit, _, target) = reduce_world(&mut e, &world);
        e.mem.set_u32(target + 0x94, world.vtable);
        // The +0x94 object is the target plus 0x94: answer its slot 0x10.
        world.answer(target + 0x94, 0x10, 1);
        e.set(hit, HitData::fFatigueDamage, 5.0f32.to_bits());
        e.call(0x009b_5a30, &args![hit, 0u32]);
        assert_eq!(e.get(hit, HitData::fHealthDamage), 0);
        assert_eq!(e.get(hit, HitData::fTotalDamage), 0);
        assert_eq!(e.get(hit, HitData::fFatigueDamage), 0);
    }

    #[test]
    fn check_critical_strike_rolls_against_the_chance() {
        let mut e = Engine::new();
        constants(&mut e);
        let world = World::new(&mut e);
        let aggressor = world.object(&mut e);
        let target = world.object(&mut e);
        let hit = Ptr::<HitData>::new(e.mem.alloc(0x64));
        e.set(hit, HitData::pAggressor, aggressor);
        e.set(hit, HitData::pTarget, target);
        e.set(hit, HitData::pWeapon, 0x600);
        e.set(hit, HitData::fHealthDamage, 10.0f32.to_bits());
        e.register(IS_FLAG_SET, |_, _| 0u32.into_ret());
        // Chance 5% -> 50 out of 1000.
        e.register(0x0064_6d80, |_, _| float_ret(5.0));
        quiet(&mut e, &[HANDLE_ENTRY_POINT, SET_FLAG]);
        e.register(0x0064_5c20, |_, _| 15u32.into_ret());
        e.register(0x009b_73b0, |_, _| 1u32.into_ret());
        e.register(0x0051_f4d0, |_, _| 0x900u32.into_ret());
        e.register(0x0048_7f50, |_, _| 1049u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009b_7060, &args![hit]);
        let log = e.call_log.take().unwrap();
        // 1049 % 1000 = 49 < 50: a critical with 15 damage added.
        assert_eq!(
            calls(&log, 0x0064_6d80),
            vec![vec![aggressor + 0xa4, 0x600]]
        );
        assert_eq!(
            calls(&log, SET_FLAG),
            vec![vec![hit.addr(), 4, 1], vec![hit.addr(), 8, 1]]
        );
        assert_eq!(e.get(hit, HitData::fHealthDamage), 25.0f32.to_bits());
        assert_eq!(e.get(hit, HitData::pCriticalEffect), 0x900);
        let entries = calls(&log, HANDLE_ENTRY_POINT);
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[1][..4], [0x24, target, 0x600, aggressor]);
        assert_eq!(entries[2][..4], [2, aggressor, 0x600, target]);
        // 50 is not below 50.
        e.register(0x0048_7f50, |_, _| 50u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009b_7060, &args![hit]);
        assert!(calls(&e.call_log.take().unwrap(), SET_FLAG).is_empty());
        // Flag 0x800 skips everything.
        e.register(IS_FLAG_SET, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009b_7060, &args![hit]);
        assert!(calls(&e.call_log.take().unwrap(), 0x0064_6d80).is_empty());
    }

    #[test]
    fn determine_outcome_flags_a_kill_and_its_limb() {
        let mut e = Engine::new();
        constants(&mut e);
        let world = World::new(&mut e);
        let target = world.object(&mut e);
        world.answer(target, 0x100, 1);
        let hit = Ptr::<HitData>::new(e.mem.alloc(0x64));
        e.set(hit, HitData::pTarget, target);
        e.set(hit, HitData::fHealthDamage, 30.0f32.to_bits());
        e.set(hit, HitData::fTargetedLimbDamage, 30.0f32.to_bits());
        e.set(hit, HitData::eDamageLimb, 2);
        // Health 20 and the limb's value 25: both go to at most 0.
        let values = HashMap::from([(0x10u32, 20.0f64), (0x1a, 25.0)]);
        e.register_double(SLOT_BASE + 0xc, move |_, a| {
            float_ret(values.get(&a[1]).copied().unwrap_or(0.0))
        });
        let data = world.object(&mut e);
        e.register_double(0x0041_81e0, move |_, _| data.into_ret());
        world.answer(data, 0x180, 0x800);
        e.register(0x005e_50f0, |_, _| 0x810u32.into_ret());
        e.register(0x008b_4360, |_, _| 1u32.into_ret());
        e.register(0x008b_4cf0, |_, _| 40u32.into_ret());
        e.register(0x008b_4cd0, |_, _| 0u32.into_ret());
        e.register(0x005e_5190, |_, _| 0x1au32.into_ret());
        e.register(0x0064_6e50, |e, a| {
            e.mem.set_u8(a[6], 1);
            Ret::default()
        });
        quiet(&mut e, &[SET_FLAG]);
        e.call_log = Some(vec![]);
        e.call(0x009b_73d0, &args![hit]);
        let log = e.call_log.take().unwrap();
        let flags: Vec<u32> = calls(&log, SET_FLAG).iter().map(|a| a[1]).collect();
        assert_eq!(flags, vec![0x80, 0x10, 0x20]);
        let outcome = &calls(&log, 0x0064_6e50)[0];
        assert_eq!(outcome[..6], [target + 0xa4, 0, 40, 0, 0, 0]);
        // A non-character target is skipped.
        world.answer(target, 0x100, 0);
        e.call_log = Some(vec![]);
        e.call(0x009b_73d0, &args![hit]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn determine_outcome_scales_by_the_bonus() {
        let mut e = Engine::new();
        constants(&mut e);
        let world = World::new(&mut e);
        let target = world.object(&mut e);
        world.answer(target, 0x100, 1);
        world.answer_float(target + 0xa4, 0xc, 1000.0);
        let hit = Ptr::<HitData>::new(e.mem.alloc(0x64));
        e.set(hit, HitData::pTarget, target);
        e.set(hit, HitData::eDamageLimb, u32::MAX);
        e.set(hit, HitData::fHealthDamage, 10.0f32.to_bits());
        e.set(hit, HitData::fTotalDamage, 12.0f32.to_bits());
        e.set(hit, HitData::fFatigueDamage, 3.0f32.to_bits());
        e.set(hit, HitData::fBonusMult, 2.0f32.to_bits());
        e.register(IS_FLAG_SET, |_, _| 1u32.into_ret());
        settings(&mut e, &[(0x011c_eae0, 1.5)]);
        quiet(&mut e, &[SET_FLAG]);
        e.call(0x009b_73d0, &args![hit]);
        assert_eq!(e.get(hit, HitData::fBonusMult), 3.0f32.to_bits());
        assert_eq!(e.get(hit, HitData::fHealthDamage), 30.0f32.to_bits());
        assert_eq!(e.get(hit, HitData::fTotalDamage), 36.0f32.to_bits());
        // No aggressor: the fatigue damage stays.
        assert_eq!(e.get(hit, HitData::fFatigueDamage), 3.0f32.to_bits());
    }
}

#[cfg(test)]
mod n026_tests {
    use super::*;

    type CallLog = [(u32, Vec<u32>)];
    type Logged = Vec<(u32, Vec<u32>)>;

    fn calls(log: &CallLog, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addresses: &[u32]) {
        for address in addresses {
            e.register(*address, |_, _| Ret::default());
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn f(bits: u32) -> f32 {
        f32::from_bits(bits)
    }

    /// An object whose vtable slots answer the given words (each slot a
    /// double at `vtable + 0x10_0000 + slot`).
    fn object(e: &mut Engine, vtable: u32, slots: &[(u32, Ret)]) -> u32 {
        let size = slots.iter().map(|(s, _)| *s).max().unwrap_or(0) / 4 + 1;
        let mut words = vec![0u32; size as usize];
        for (slot, ret) in slots {
            let target = vtable + 0x10_0000 + slot;
            words[*slot as usize / 4] = target;
            let ret = *ret;
            e.register_double(target, move |_, _| ret);
        }
        e.put_vtable(vtable, &words);
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object, vtable);
        object
    }

    fn word(value: u32) -> Ret {
        value.into_ret()
    }

    /// A target: `character` answers its virtual +0x100; its +0x94 object
    /// is immune (virtual +0x10), so `CheckCriticalStrike` stops at once.
    fn make_target(e: &mut Engine, character: bool, extra: &[(u32, Ret)]) -> u32 {
        let mut slots = vec![(0x100, word(u32::from(character)))];
        slots.extend_from_slice(extra);
        let target = object(e, 0x0a00_0000, &slots);
        let immune = object(e, 0x0a20_0000, &[(0x10, word(1))]);
        let vtable = e.mem.u32(immune);
        e.mem.set_u32(target + 0x94, vtable);
        target
    }

    fn settings(e: &mut Engine) {
        e.map(0x011c_e000, 0x4000);
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.map(PLAYER_011DEA3C & !0xfff, 0x1000);
        e.map(FIVE_HUNDREDTHS_0103C7C8 & !0xfff, 0x1000);
        e.set_global(FIVE_HUNDREDTHS_0103C7C8, f64::from(0.05f32));
        e.register(SET_FLAG, |e, a| {
            let flags = e.mem.u32(a[0] + 0x58);
            let flags = if a[2] != 0 {
                flags | a[1]
            } else {
                flags & !a[1]
            };
            e.mem.set_u32(a[0] + 0x58, flags);
            Ret::default()
        });
        e.register(IS_FLAG_SET, |e, a| {
            u32::from(e.mem.u32(a[0] + 0x58) & a[1] != 0).into_ret()
        });
    }

    fn setting(e: &mut Engine, address: u32, value: f32) {
        e.mem.set_f32(address + 4, value);
    }

    #[test]
    fn small_helpers() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x009b_65d0, &args![0xffff_0005u32, 3u32]).i32(), 5);
        assert_eq!(e.call(0x009b_65d0, &args![2u32, 3u32]).i32(), 3);
        assert_eq!(e.call(0x009b_65d0, &args![0xffffu32, -1i32]).i32(), 0xffff);
        let weapon = e.mem.alloc(0x200);
        e.mem.set_u32(weapon + 0x168, 0x7f80_0001);
        e.mem.set_u8(weapon + 0x1c8, 9);
        assert_eq!(
            e.call(0x009b_7040, &args![weapon]).f32().to_bits(),
            0x7fc0_0001
        );
        assert_eq!(e.call(0x009b_73b0, &args![weapon]).u8(), 9);
        let hit = e.new_object::<HitData>();
        e.set(hit, HitData::fBonusMult, 1.5f32.to_bits());
        e.call(0x009b_7840, &args![hit, 1.0f32]);
        assert_eq!(f(e.get(hit, HitData::fBonusMult)), 1.5);
        e.call(0x009b_7840, &args![hit, 2.5f32]);
        assert_eq!(f(e.get(hit, HitData::fBonusMult)), 2.5);
        e.call(0x009b_7840, &args![hit, f32::NAN]);
        assert_eq!(f(e.get(hit, HitData::fBonusMult)), 2.5);
    }

    /// Arrays whose constructors give them a 0x40-byte buffer and whose
    /// slot reservation counts up the size at +8.
    fn arrays(e: &mut Engine) {
        e.register(ARRAY_INITIALIZE, |e, a| {
            let buffer = e.mem.alloc(0x40);
            e.mem.set_u32(a[0] + 4, buffer);
            Ret::default()
        });
        e.register(0x0043_8660, |e, a| {
            let buffer = e.mem.alloc(0x40);
            e.mem.set_u32(a[0] + 4, buffer);
            a[0].into_ret()
        });
        e.register(ARRAY_RESERVE_SLOT, |e, a| {
            let size = e.mem.u32(a[0] + 8);
            e.mem.set_u32(a[0] + 8, size + 1);
            size.into_ret()
        });
        quiet(
            e,
            &[
                ARRAY_CONSTRUCT_ELEMENTS,
                0x0043_8690,
                ARRAY_RELEASE_008454F0,
            ],
        );
    }

    #[test]
    fn weighted_random_builds_adds_and_goes() {
        let mut e = Engine::new();
        arrays(&mut e);
        quiet(&mut e, &[OPERATOR_DELETE]);
        e.register(0x0099_62f0, |e, a| {
            u32::from(e.mem.u32(a[1]) == 0x77).into_ret()
        });
        let random = e.mem.alloc(0x24);
        e.mem.set_u32(random + 0x20, 0xffff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009b_7870, &args![random]).u32(), random);
        assert_eq!(e.mem.u32(random), ARRAY_VTABLE_0108F9E8);
        assert_eq!(e.mem.u32(random + 0x20), 0);
        e.call(0x009b_7990, &args![random, 0x55u32, 3u32]);
        e.call(0x009b_7990, &args![random, 0x66u32, 4u32]);
        assert_eq!(e.mem.u32(random + 0x20), 7);
        let results = e.mem.u32(random + 4);
        let weights = e.mem.u32(random + 0x14);
        assert_eq!([e.mem.u32(results), e.mem.u32(results + 4)], [0x55, 0x66]);
        assert_eq!([e.mem.u32(weights), e.mem.u32(weights + 4)], [3, 4]);
        assert!(e.call(0x009b_79d0, &args![random, 0x77u32]).bool());
        assert!(!e.call(0x009b_79d0, &args![random, 0x55u32]).bool());
        e.call(0x009b_78e0, &args![random]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, ARRAY_INITIALIZE), vec![vec![random, 0, 0]]);
        assert_eq!(calls(&log, 0x0043_8690), vec![vec![random + 0x10]]);
        assert_eq!(calls(&log, ARRAY_RELEASE_008454F0), vec![vec![random, 1]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009b_79f0, &args![random, 1u32]).u32(), random);
        assert_eq!(e.call(0x009b_79f0, &args![random, 2u32]).u32(), random);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, OPERATOR_DELETE), vec![vec![random]]);
    }

    #[test]
    fn plain_hit_and_projectile_hit() {
        let mut e = Engine::new();
        settings(&mut e);
        let target = make_target(&mut e, false, &[]);
        let hit = e.new_object::<HitData>();
        e.call(0x009b_5720, &args![0u32, 1u32, 2u32, 3u32]);
        e.call(0x009b_5720, &args![hit, 0x11u32, target, 0x7f80_0001u32]);
        assert_eq!(e.get(hit, HitData::pAggressor), 0x11);
        assert_eq!(e.get(hit, HitData::pSourceRef), 0x11);
        assert_eq!(e.get(hit, HitData::pTarget), target);
        assert_eq!(e.get(hit, HitData::fHealthDamage), 0x7fc0_0001);
        assert_eq!(e.get(hit, HitData::fTotalDamage), 0x7fc0_0001);

        let aggressor = 0x0b00_0000u32;
        e.register(PROJECTILE_WEAPON, |_, _| 0x0c00_0000u32.into_ret());
        e.register(GET_ITEM_HEALTH, |_, a| {
            assert_eq!(a[1], 1);
            st0(0.75)
        });
        e.register(0x0088_5d70, |_, _| st0(5.0));
        e.register(AGGRESSOR_DAMAGE_FACTOR, |_, _| st0(1.5));
        let hit = e.new_object::<HitData>();
        e.call(
            0x009b_5650,
            &args![hit, aggressor, target, 0x0d00_0000u32, 0x0e00_0000u32],
        );
        assert_eq!(e.get(hit, HitData::pWeapon), 0x0c00_0000);
        assert_eq!(f(e.get(hit, HitData::fWeaponHealth)), 0.75);
        assert_eq!(e.get(hit, HitData::pSourceRef), 0x0e00_0000);
        assert_eq!(f(e.get(hit, HitData::fHealthDamage)), 7.5);
        assert_eq!(f(e.get(hit, HitData::fTotalDamage)), 7.5);
        // No aggressor, item or projectile.
        let hit = e.new_object::<HitData>();
        e.call(0x009b_5650, &args![hit, 0u32, target, 0u32, 0u32]);
        assert_eq!(e.get(hit, HitData::pWeapon), 0);
        assert_eq!(f(e.get(hit, HitData::fWeaponHealth)), 1.0);
        assert_eq!(f(e.get(hit, HitData::fHealthDamage)), 5.0);
    }

    struct Melee {
        item: bool,
        command: Option<(u32, u8, u8)>,
        power: u32,
        hand_to_hand_answer: u32,
        flags: u32,
        weapon_test: u32,
        force_update: u32,
        projectile: u32,
    }

    fn melee(world: Melee) -> (Engine, Ptr<HitData>, Logged) {
        let mut e = Engine::new();
        settings(&mut e);
        let player = e.mem.alloc(0x200);
        e.set_global(PLAYER_011DEA3C, player);
        e.register(0x0066_ef50, |_, _| st0(10.0));
        let target = make_target(&mut e, false, &[(0x230, word(1))]);
        let aggressor = object(
            &mut e,
            0x0a40_0000,
            &[
                (0x1e4, word(0x5000)),
                (0x21c, word(world.hand_to_hand_answer)),
                (0x360, word(1)),
                (0x444, word(7)),
            ],
        );
        e.register(0x0043_01b0, |_, a| {
            assert_eq!(a[1], 4);
            0xabcd_0003u32.into_ret()
        });
        e.register(0x005f_2440, |_, a| {
            assert_eq!(a[0], 3);
            2u32.into_ret()
        });
        e.register(0x0044_ddc0, |_, _| 0x0700_0000u32.into_ret());
        e.register(0x0040_1170, |_, _| 0x28u32.into_ret());
        e.register(GET_ITEM_HEALTH, |_, _| st0(0.5));
        e.register(0x008d_85e0, |_, _| 0x20u32.into_ret());
        e.register(AGGRESSOR_TEST_004997B0, |_, _| Ret::default());
        e.register(GET_CLAMPED_ACTOR_VALUE, |_, _| 50u32.into_ret());
        e.register(POWER_ATTACK_MULTIPLIER, |_, a| {
            assert_eq!(a, [50, 2]);
            st0(1.5)
        });
        e.register(0x004b_df00, |_, _| st0(10.0));
        e.register(0x0052_4b60, |_, _| 1u32.into_ret());
        let weapon_test = world.weapon_test;
        e.register_double(WEAPON_TEST_006450C0, move |_, _| weapon_test.into_ret());
        e.register(0x0052_4b40, |_, _| 1u32.into_ret());
        e.register(0x008d_9140, |_, _| st0(4.0));
        e.register(AGGRESSOR_DAMAGE_FACTOR, |_, _| st0(2.0));
        e.register(0x0046_e8c0, |_, _| Ret::default());
        e.register(0x0064_6260, |_, _| st0(3.0));
        let force = world.force_update;
        e.register_double(0x0056_6950, move |_, _| force.into_ret());
        e.register(0x0064_6310, |e, a| {
            e.mem.set_f32(a[2], 4.0);
            e.mem.set_f32(a[3], 1.0);
            Ret::default()
        });
        e.register(PROJECTILE_WEAPON, |_, _| 0x0c00_0000u32.into_ret());
        e.register(0x0088_5d70, |_, _| st0(5.0));
        setting(&mut e, 0x011c_f5d0, 2.0);
        setting(&mut e, 0x011d_0424, 0.5);
        setting(&mut e, 0x011d_12d8, 0.25);
        setting(&mut e, 0x011d_07b4, 0.125);
        let hit = e.new_object::<HitData>();
        e.set(hit, HitData::uiFlags, world.flags);
        if let Some((kind, ready, shots)) = world.command {
            let command = e.mem.alloc(0x20);
            e.mem.set_u32(command, kind);
            e.mem.set_u8(command + 4, ready);
            e.mem.set_u8(command + 8, shots);
            e.set(hit, HitData::pVATSCommand, command);
        }
        let item = if world.item { 0x0600_0000u32 } else { 0 };
        e.call_log = Some(vec![]);
        if world.projectile != 0 {
            e.call(
                0x009b_5120,
                &args![hit, aggressor, target, item, world.power, world.projectile],
            );
        } else {
            e.call(
                0x009b_5120,
                &args![hit, aggressor, target, item, world.power, 0u32],
            );
        }
        let log = e.call_log.take().unwrap();
        (e, hit, log)
    }

    #[test]
    fn melee_weapon_hit_with_power_attack() {
        let (e, hit, log) = melee(Melee {
            item: true,
            command: None,
            power: 0x101,
            hand_to_hand_answer: 0,
            flags: 0,
            weapon_test: 0,
            force_update: 1,
            projectile: 0,
        });
        assert_eq!(e.get(hit, HitData::pWeapon), 0x0700_0000);
        assert_eq!(f(e.get(hit, HitData::fWeaponHealth)), 0.5);
        assert_eq!(e.get(hit, HitData::eAttackSkill), 0x20);
        let damage = calls(&log, 0x004b_df00);
        assert_eq!(damage[0][3], 0);
        assert_eq!(f(damage[0][2]), 1.5);
        // 10 x 2 (aggressor) x (1 + 10 x 0.05) (player).
        let factor = x87_store32(x87_add(x87_mul(10.0, f64::from(0.05f32)), 1.0));
        let expected = x87_store32(x87_mul(20.0, x87_m32(factor)));
        assert_eq!(e.get(hit, HitData::fHealthDamage), expected);
        assert_eq!(f(e.get(hit, HitData::fDamageToWeapon)), 3.0);
        assert!(calls(&log, 0x0088_5d70).is_empty());
    }

    #[test]
    fn vats_weapon_hit_scales_by_shots_and_action() {
        let (e, hit, log) = melee(Melee {
            item: true,
            command: Some((0x11, 1, 3)),
            power: 0,
            hand_to_hand_answer: 0,
            flags: 0x1000,
            weapon_test: 1,
            force_update: 0,
            projectile: 0,
        });
        assert_eq!(f(calls(&log, 0x004b_df00)[0][2]), 1.0);
        // 10 x 3 shots x 2.0 x 0.5, then x 2 (aggressor).
        assert_eq!(f(e.get(hit, HitData::fHealthDamage)), 60.0);
        let (e, hit, _) = melee(Melee {
            item: true,
            command: Some((0x10, 1, 1)),
            power: 0,
            hand_to_hand_answer: 0,
            flags: 0,
            weapon_test: 0,
            force_update: 0,
            projectile: 0,
        });
        // One shot, no 006450c0: 10 x 4.0 (008d9140) x 2.
        assert_eq!(f(e.get(hit, HitData::fHealthDamage)), 80.0);
        let (e, hit, _) = melee(Melee {
            item: true,
            command: Some((0x15, 1, 1)),
            power: 0,
            hand_to_hand_answer: 0,
            flags: 0,
            weapon_test: 0,
            force_update: 0,
            projectile: 0,
        });
        assert_eq!(f(e.get(hit, HitData::fHealthDamage)), 2.5);
    }

    #[test]
    fn unarmed_hits() {
        let (e, hit, log) = melee(Melee {
            item: false,
            command: None,
            power: 0,
            hand_to_hand_answer: 1,
            flags: 0,
            weapon_test: 0,
            force_update: 0,
            projectile: 0,
        });
        // The aggressor's virtual +0x444 (7) as the damage, x 2.
        assert_eq!(f(e.get(hit, HitData::fHealthDamage)), 14.0);
        assert!(calls(&log, 0x0064_6310).is_empty());
        let (e, hit, log) = melee(Melee {
            item: false,
            command: None,
            power: 1,
            hand_to_hand_answer: 0,
            flags: 0,
            weapon_test: 0,
            force_update: 0,
            projectile: 0,
        });
        let hand = calls(&log, 0x0064_6310);
        assert_eq!(hand[0][1], 1);
        assert_eq!(hand[0][2..], [hit.addr() + 0x14, hit.addr() + 0x1c]);
        assert_eq!(e.get(hit, HitData::eAttackSkill), 0x2d);
        assert_eq!(f(e.get(hit, HitData::fBonusMult)), 1.5);
        assert_eq!(f(e.get(hit, HitData::fFatigueDamage)), 1.0);
        assert_eq!(f(e.get(hit, HitData::fHealthDamage)), 8.0);
    }

    #[test]
    fn a_vats_hit_that_does_not_count_and_the_projectile_dispatch() {
        let (e, hit, log) = melee(Melee {
            item: true,
            command: Some((0x11, 0, 1)),
            power: 0,
            hand_to_hand_answer: 0,
            flags: 0,
            weapon_test: 0,
            force_update: 0,
            projectile: 0,
        });
        assert!(calls(&log, 0x004b_df00).is_empty());
        assert!(calls(&log, AGGRESSOR_DAMAGE_FACTOR).is_empty());
        assert_eq!(e.get(hit, HitData::fHealthDamage), 0);
        let (e, hit, log) = melee(Melee {
            item: false,
            command: None,
            power: 1,
            hand_to_hand_answer: 0,
            flags: 0,
            weapon_test: 0,
            force_update: 0,
            projectile: 0x0e00_0000,
        });
        assert_eq!(calls(&log, 0x0088_5d70), vec![vec![0x0e00_0000]]);
        assert!(calls(&log, 0x0043_01b0).is_empty());
        assert_eq!(f(e.get(hit, HitData::fHealthDamage)), 10.0);
    }

    #[test]
    fn vats_command_initialization() {
        let mut e = Engine::new();
        settings(&mut e);
        let player = object(&mut e, 0x0a60_0000, &[(0x1e4, word(0x5000))]);
        e.set_global(PLAYER_011DEA3C, player);
        let process = object(&mut e, 0x0a70_0000, &[(0x148, word(0x0600_0000))]);
        e.register_double(GET_PROCESS, move |_, _| process.into_ret());
        e.register(IS_MELEE_ATTACK_ACTION, |_, _| 1u32.into_ret());
        e.register(0x0043_01b0, |_, _| 3u32.into_ret());
        e.register(0x005f_2440, |_, _| 2u32.into_ret());
        e.register(0x0044_ddc0, |_, _| 0x0700_0000u32.into_ret());
        e.register(0x0040_1170, |_, _| 0x28u32.into_ret());
        e.register(GET_ITEM_HEALTH, |_, _| st0(0.5));
        // A weapon `006450c0` rejects with byte +4 clear: the hit does not count.
        e.register(WEAPON_TEST_006450C0, |_, _| Ret::default());
        let target = make_target(&mut e, false, &[]);
        let hit = e.new_object::<HitData>();
        let command = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x009b_5050, &args![hit, command]);
        assert_eq!(e.get(hit, HitData::pVATSCommand), 0);
        e.mem.set_u32(command + 0xc, target);
        e.call(0x009b_5050, &args![0u32, command]);
        e.call(0x009b_5050, &args![hit, command]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(hit, HitData::pVATSCommand), command);
        assert_eq!(e.get(hit, HitData::pAggressor), player);
        // Not a character: the target is 0.
        assert_eq!(e.get(hit, HitData::pTarget), 0);
        assert_eq!(e.get(hit, HitData::pWeapon), 0x0700_0000);
        assert_eq!(calls(&log, IS_MELEE_ATTACK_ACTION).len(), 1);
        assert!(calls(&log, 0x004b_df00).is_empty());

        // The damage estimate: no command, or a stack hit with the
        // command's byte +4 set for the call and restored after.
        assert_eq!(e.call(0x009b_4fe0, &args![0u32]).f32(), 0.0);
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u8(command + 4, 0);
        e.mem.set_u32(command + 0xc, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x009b_4fe0, &args![command]).f32(), 0.0);
        let log = e.call_log.take().unwrap();
        let flags: Vec<_> = calls(&log, SET_FLAG).iter().map(|w| w[1]).collect();
        assert_eq!(flags, [0x800, 0x1000]);
        assert_eq!(e.mem.u8(command + 4), 0);
    }

    #[test]
    fn impact_data_places_the_hit_and_finds_the_limb() {
        let mut e = Engine::new();
        settings(&mut e);
        e.map(0x0101_6000, 0x1000);
        e.mem.set_f32(0x0101_622c, 0.25);
        let point = e.mem.alloc(12);
        e.mem.set_u32(point, 0x1111);
        e.mem.set_u32(point + 4, 0x2222);
        e.mem.set_u32(point + 8, 0x3333);
        let target = make_target(&mut e, false, &[(0x1f4, word(point))]);
        let impact = e.mem.alloc(0x30);
        for i in 0..6 {
            e.mem.set_u32(impact + 4 + 4 * i, 0x100 + i);
        }
        e.register(POINT_SUBTRACT, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[1] + 4 * i, 0x5000 + i);
            }
            a[1].into_ret()
        });
        e.register(0x004a_3760, |_, a| a[0].into_ret());
        e.register(0x0063_c8a0, |e, a| {
            e.mem.set_u32(a[0], 0x4444);
            a[0].into_ret()
        });
        e.register(0x008b_3d70, |_, _| 0x9999u32.into_ret());
        e.register(LIMB_FOR_NODE, |_, a| {
            assert_eq!(a[1], 0x9999);
            6u32.into_ret()
        });
        let hit = e.new_object::<HitData>();
        e.set(hit, HitData::pAggressor, 0x77);
        e.call_log = Some(vec![]);
        e.call(0x009b_58d0, &args![hit, 0xdeadu32, target, 2.0f32, impact]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(hit, HitData::pAggressor), 0);
        assert_eq!(f(e.get(hit, HitData::fTotalDamage)), 2.0);
        // x moved by 0063c8a0, y kept, z the impact's +0xC.
        assert_eq!(
            (0..3)
                .map(|i| field(&e, hit, HIT_LOCATION + 4 * i))
                .collect::<Vec<_>>(),
            vec![0x4444, 0x2222, 0x102]
        );
        assert_eq!(
            (0..3)
                .map(|i| field(&e, hit, HIT_DIRECTION + 4 * i))
                .collect::<Vec<_>>(),
            vec![0x103, 0x104, 0x105]
        );
        let scaled = calls(&log, 0x004a_3760);
        assert_eq!(f(scaled[0][1]), 0.25);
        let limb = calls(&log, 0x008b_3d70);
        assert_eq!(
            limb[0],
            vec![target, 0x5000, 0x5001, 0x5002, 0x100, 0x101, 0x102]
        );
        assert_eq!(e.get(hit, HitData::eDamageLimb), 6);
    }

    /// A character target with body-part data 0x9000 (base object
    /// 0x0900_0000's virtual +0x180) and its virtual +0x360 answering
    /// `armored`.
    fn limb_world(e: &mut Engine, armored: u32, roots: u32) -> (Ptr<HitData>, u32) {
        settings(e);
        let target = make_target(e, true, &[(0x360, word(armored)), (0x1d0, word(roots))]);
        let base = object(e, 0x0a80_0000, &[(0x180, word(0x9000))]);
        e.register_double(0x0041_81e0, move |_, _| base.into_ret());
        e.register(BODY_PART_LIMB, |_, a| (a[0] & 0xff).into_ret());
        e.register(0x0064_7a30, |_, a| st0(f64::from(f(a[2])) * 2.0));
        e.register(0x0063_9b20, |_, _| st0(3.0));
        e.register(WEAPON_TEST_006450C0, |_, _| Ret::default());
        setting(e, 0x011c_fa40, 0.5);
        setting(e, 0x011c_e5d0, 0.25);
        let hit = e.new_object::<HitData>();
        e.set(hit, HitData::pTarget, target);
        e.set(hit, HitData::fHealthDamage, 6.0f32.to_bits());
        (hit, target)
    }

    #[test]
    fn vats_limbs() {
        let mut e = Engine::new();
        let (hit, target) = limb_world(&mut e, 1, 0);
        e.register(0x005e_5130, |_, a| {
            assert_eq!(a, [0x9000, 3]);
            0x9105u32.into_ret()
        });
        let weapon = e.mem.alloc(0x200);
        e.mem.set_f32(weapon + 0x168, 0.5);
        e.set(hit, HitData::pWeapon, weapon);
        let command = e.mem.alloc(0x20);
        e.mem.set_u32(command + 0x10, 3);
        e.set(hit, HitData::pVATSCommand, command);
        e.call_log = Some(vec![]);
        e.call(0x009b_6620, &args![hit, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(hit, HitData::eDamageLimb), 5);
        assert_eq!(calls(&log, 0x0064_7a30)[0][..2], [target + 0xa4, 0x9105]);
        // 6 x 2, x 0.5 (armored), x 0.5 (the weapon's +0x168).
        assert_eq!(f(e.get(hit, HitData::fTargetedLimbDamage)), 3.0);
        assert_eq!(f(e.get(hit, HitData::fBonusMult)), 3.0);

        // A command without a part on a non-melee action: limb 0xE.
        let mut e = Engine::new();
        let (hit, _) = limb_world(&mut e, 0, 0);
        e.register(IS_MELEE_ATTACK_ACTION, |_, _| Ret::default());
        let command = e.mem.alloc(0x20);
        e.mem.set_u32(command + 0x10, u32::MAX);
        e.set(hit, HitData::pVATSCommand, command);
        e.call(0x009b_6620, &args![hit, 0u32]);
        assert_eq!(e.get(hit, HitData::eDamageLimb), 0xe);
        assert_eq!(f(e.get(hit, HitData::fTargetedLimbDamage)), 1.5);
        assert_eq!(e.get(hit, HitData::fHealthDamage), 0);

        // A melee action with flag 0x800: limb 0, nothing else.
        let mut e = Engine::new();
        let (hit, _) = limb_world(&mut e, 0, 0);
        e.register(IS_MELEE_ATTACK_ACTION, |_, _| 1u32.into_ret());
        let command = e.mem.alloc(0x20);
        e.mem.set_u32(command + 0x10, u32::MAX);
        e.set(hit, HitData::pVATSCommand, command);
        e.set(hit, HitData::uiFlags, 0x800);
        e.set(hit, HitData::eDamageLimb, 9);
        e.call(0x009b_6620, &args![hit, 0u32]);
        assert_eq!(e.get(hit, HitData::eDamageLimb), 0);
        assert_eq!(e.get(hit, HitData::fTargetedLimbDamage), 0);

        // Not a character: nothing.
        let mut e = Engine::new();
        settings(&mut e);
        let target = make_target(&mut e, false, &[]);
        let hit = e.new_object::<HitData>();
        e.set(hit, HitData::pTarget, target);
        e.set(hit, HitData::eDamageLimb, 9);
        e.call(0x009b_6620, &args![hit, 0u32]);
        assert_eq!(e.get(hit, HitData::eDamageLimb), 9);
    }

    #[test]
    fn projectile_impact_limb() {
        let mut e = Engine::new();
        let (hit, target) = limb_world(&mut e, 0, 0);
        let impact = e.mem.alloc(0x30);
        for i in 0..3 {
            e.mem.set_u32(impact + 4 + 4 * i, 0x200 + i);
        }
        e.mem.set_u32(impact + 0x24, 2);
        e.register_double(0x009c_3ee0, move |_, a| {
            assert_eq!(a[1], target);
            impact.into_ret()
        });
        e.register(GET_BODY_PART, |_, a| (0x9100 | a[1]).into_ret());
        let projectile = e.mem.alloc(0x200);
        for i in 0..3 {
            e.mem.set_u32(projectile + 0x104 + 4 * i, 0x300 + i);
        }
        quiet(&mut e, &[0x004a_0c10, 0x0063_c8a0]);
        e.register(0x004a_0bd0, |_, a| a[1].into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009b_6620, &args![hit, projectile]);
        let log = e.call_log.take().unwrap();
        assert_eq!(field(&e, hit, HIT_LOCATION), 0x200);
        assert_eq!(field(&e, hit, HIT_DIRECTION + 8), 0x302);
        assert_eq!(calls(&log, 0x004a_0c10).len(), 2);
        assert_eq!(calls(&log, 0x004a_0bd0)[0][0], impact + 0x10);
        // The part 0x9102 gives limb 2 again; 6 x 2 limb damage.
        assert_eq!(e.get(hit, HitData::eDamageLimb), 2);
        assert_eq!(f(e.get(hit, HitData::fTargetedLimbDamage)), 12.0);
        // No weapon: the bonus is 1.0.
        assert_eq!(f(e.get(hit, HitData::fBonusMult)), 1.0);
    }

    fn ray_world(e: &mut Engine, caster: u32, roots: u32) -> (Ptr<HitData>, u32) {
        let (hit, target) = limb_world(e, 0, roots);
        arrays(e);
        let aggressor = object(e, 0x0a90_0000, &[(0x1d0, word(roots))]);
        e.set(hit, HitData::pAggressor, aggressor);
        e.map(PLAYER_011DEA3C & !0xfff, 0x1000);
        e.set_global(PLAYER_011DEA3C, aggressor);
        e.register(GET_WEAPON_POSITION, |_, a| a[1].into_ret());
        e.register(SAME_ADDRESS, |_, a| a[0].into_ret());
        e.register(0x008a_ff80, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[1] + 4 * i, 0x400 + i);
                e.mem.set_u32(a[2] + 4 * i, 0x500 + i);
            }
            Ret::default()
        });
        e.map(0x0102_2000, 0x1000);
        e.mem.set_f32(LIMB_RAY_LENGTH, 200.0);
        e.map(0x0105_0000, 0x1000);
        e.mem.set_f32(LIMB_FALLBACK_SCALE, 6.0);
        quiet(
            e,
            &[
                0x0045_bb20,
                0x0062_6d20,
                0x0062_6b30,
                POINT_COPY,
                0x0062_40d0,
                0x004a_3e00,
            ],
        );
        quiet(e, &[0x008a_50f0, 0x008a_50d0, 0x0045_43c0]);
        e.register_double(0x008d_6f30, move |_, _| caster.into_ret());
        e.register(0x0062_6dd0, |_, _| 0x0f00_0000u32.into_ret());
        e.register(0x0062_1210, |_, _| 0x0f10_0000u32.into_ret());
        e.register(0x0056_09f0, |_, _| 1u32.into_ret());
        e.register(0x0046_0140, |_, _| 0x0f20_0000u32.into_ret());
        e.register(0x0072_6070, |_, _| 3u32.into_ret());
        // Hit 0 is another reference; hits 1 and 2 are the target's bodies
        // with a node (limb 4) and without one (009611e0: limb 0xE).
        let bodies = [
            object(e, 0x0aa0_0000, &[(0xc, word(0x0100))]),
            object(e, 0x0ab0_0000, &[(0xc, word(0))]),
        ];
        e.register(0x0080_7770, |_, a| a[1].into_ret());
        e.register(0x0063_2d40, |e, a| {
            e.mem.set_u32(a[0] + 0x48, a[1]);
            Ret::default()
        });
        e.register_double(0x004b_5820, move |_, a| {
            let body = match a[0] {
                1 => bodies[0],
                2 => bodies[1],
                _ => 0x77,
            };
            body.into_ret()
        });
        e.register_double(0x0056_f930, move |_, a| {
            let reference = if a[0] == 0x77 { 0x1234 } else { target };
            reference.into_ret()
        });
        e.register(0x004b_59f0, |_, _| 1u32.into_ret());
        e.register(0x0096_11e0, |_, _| 0x0200u32.into_ret());
        e.register(LIMB_FOR_NODE, |_, a| {
            let limb: u32 = if a[1] == 0x0100 { 4 } else { 0xe };
            limb.into_ret()
        });
        e.register(GET_BODY_PART, |_, a| (0x9100 | a[1]).into_ret());
        e.register(0x0099_62f0, |_, _| Ret::default());
        e.register(0x005d_c980, |_, _| 0x0107u32.into_ret());
        e.register(0x0082_2650, |e, a| {
            let results = e.mem.u32(a[0] + 4);
            e.mem.u32(results).into_ret()
        });
        quiet(e, &[0x0043_8690, ARRAY_RELEASE_008454F0]);
        (hit, target)
    }

    #[test]
    fn ray_limb_from_the_hit_caster() {
        let mut e = Engine::new();
        let (hit, _) = ray_world(&mut e, 0x0e00_0000, 0);
        e.call_log = Some(vec![]);
        e.call(0x009b_6620, &args![hit, 0u32]);
        let log = e.call_log.take().unwrap();
        // The player casts: 0 for "not the player", then the ray.
        assert_eq!(calls(&log, 0x0062_6dd0), vec![vec![0, 1]]);
        let cast = calls(&log, 0x0062_6b30);
        assert_eq!(f(cast[0][3]), 200.0);
        // Two target hits move the weapon position; the location once.
        let copies = calls(&log, POINT_COPY);
        assert_eq!(copies.len(), 3);
        assert_eq!(copies[1][0], hit.addr() + HIT_LOCATION);
        assert_eq!(field(&e, hit, HIT_DIRECTION + 4), 0x501);
        // Only limb 4 is added, with weight 7.
        let added = calls(&log, 0x0099_62f0);
        assert_eq!(added.len(), 1);
        assert_eq!(e.get(hit, HitData::eDamageLimb), 4);
        assert_eq!(f(e.get(hit, HitData::fTargetedLimbDamage)), 12.0);
        assert!(calls(&log, 0x00c8_06b0).is_empty());
        assert_eq!(calls(&log, 0x0043_8690).len(), 1);
    }

    #[test]
    fn ray_limb_falls_back_to_a_pick() {
        let mut e = Engine::new();
        let (hit, target) = ray_world(&mut e, 0, 0x0c00_0000);
        let centers = e.mem.alloc(0x20);
        e.register_double(0x0043_d450, move |_, a| {
            (if a[0] == 0x0c00_0000 {
                centers
            } else {
                centers + 0x10
            })
            .into_ret()
        });
        e.register(POINT_SUBTRACT, |_, a| a[1].into_ret());
        e.register(0x0053_d280, |_, a| a[1].into_ret());
        e.register(0x0043_9e90, |e, a| {
            for i in 0..3 {
                e.mem.set_u32(a[1] + 4 * i, 0x600 + i);
            }
            a[1].into_ret()
        });
        e.register(0x00c8_06b0, |_, _| 0x0d10_0000u32.into_ret());
        e.register(0x0043_b560, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x009b_6620, &args![hit, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(field(&e, hit, HIT_LOCATION + 8), 0x602);
        let scaled = calls(&log, 0x0053_d280);
        assert_eq!(f(scaled[0][2]), 6.0);
        let pick = calls(&log, 0x00c8_06b0);
        assert_eq!(pick[0][0], 0x0c00_0000);
        assert_eq!(pick[0][2], 0);
        assert_eq!(f(calls(&log, 0x008a_50d0)[0][1]), 200.0);
        assert_eq!(e.get(hit, HitData::eDamageLimb), 4);
        assert_eq!(calls(&log, 0x0043_d450).len(), 2);
        assert_eq!(calls(&log, LIMB_FOR_NODE)[0][0], target);
    }
}
