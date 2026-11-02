//! `fallout shared/aiformulas.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UFollowerDist@@$0EAA@@` (read at the word before the vtable: its complete object locator, then the type descriptor).
const ARRAY_VTABLE_0104F3E8: u32 = 0x0104_f3e8;
/// `008454f0` (unnamed in the engine map): `thiscall`, `this` plus one stack word (`RET 4`). When the
/// buffer pointer (+4) is set it runs the instance's element destroy step; with the word non-zero it
/// then calls the array's virtual slot +8 on the buffer and clears the buffer pointer and the reserved
/// size (+0xc); finally it clears the size (+8).
const ARRAY_RELEASE_008454F0: u32 = 0x0084_54f0;

// Translated from 006442e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of the `BSSimpleArray` instance with RTTI name `.?AV?$BSSimpleArray@UFollowerDist@@$0EAA@@` (`thiscall`): stores the
/// instance's vtable at `this`, then calls `008454f0(this, 1)`.
pub fn fn_006442e0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0104F3E8);
    e.call(ARRAY_RELEASE_008454F0, &args![this, 1u32]);
}

/// `00403e20` (`thiscall` on a setting): the address of its value (+4), or of
/// a zeroed scratch `float` without a setting.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `00406ce0` (`cdecl`, one `float`, result in ST0; no Xbox PDB name).
const FLOAT_STEP_00406CE0: u32 = 0x0040_6ce0;
/// `004e4450` (`cdecl`, one `float`, result in ST0; no Xbox PDB name).
const FLOAT_STEP_004E4450: u32 = 0x004e_4450;
/// `0047f060` (`cdecl`, the actor value; returns a `bool` in AL; no Xbox
/// PDB name).
const ACCEPTS_ACTOR_VALUE_0047F060: u32 = 0x0047_f060;
/// `00643bf0` (`cdecl`: actor value, `int*`, `float*`; unit
/// `(unplaced)/run_00642860`): a word from the table at `0119b144` and the value of the
/// setting from the table at `0119b17c`, by actor value.
const ENTRY_00643BF0: u32 = 0x0064_3bf0;
/// `ActorValueOwner::GetClampedActorFloatValue` (`0066ef50`, engine map;
/// `thiscall`, one word; result in ST0).
const GET_CLAMPED_ACTOR_FLOAT_VALUE: u32 = 0x0066_ef50;
/// `ActorValue::ClampActorValue` (`0066f190`, engine map; `cdecl`: actor
/// value, `float`; result in ST0).
const CLAMP_ACTOR_VALUE: u32 = 0x0066_f190;
/// `Actor::GetClass` (`00884350`, engine map; `thiscall`, no stack words).
const ACTOR_GET_CLASS: u32 = 0x0088_4350;
/// `005a5f40` (`thiscall` on the class, one word: the actor value; returns
/// a `bool` in AL; no Xbox PDB name).
const CLASS_TEST_005A5F40: u32 = 0x005a_5f40;

/// The address of a setting's value (`00403e20`).
fn setting(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_VALUE, &args![setting]).u32()
}

/// `FLD dword [address]`.
fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

// Translated from 00643c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AiFormulas::CalculateDerivedSkillValue` (Xbox PDB, matched by the engine
/// map; `cdecl`, result in ST0): `base + 00406ce0(first * s[11cdb30]) +
/// 004e4450(second * s[11cdd84])`, each sum stored as a `float` (`s[..]` is
/// the value of the setting at that address).
pub fn ai_formulas_calculate_derived_skill_value(
    e: &mut Engine,
    base: f32,
    first: f32,
    second: f32,
) -> f32 {
    let mut total = x87_quiet_bits(base.to_bits());
    let scale = setting(e, 0x011c_db30);
    let term = x87_store32(x87_mul(
        x87_load32(first.to_bits()),
        x87_m32(e.mem.u32(scale)),
    ));
    let step = e
        .call(FLOAT_STEP_00406CE0, &args![x87_quiet_bits(term)])
        .f64();
    total = x87_store32(x87_add(step, x87_m32(total)));
    let scale = setting(e, 0x011c_dd84);
    let term = x87_store32(x87_mul(
        x87_load32(second.to_bits()),
        x87_m32(e.mem.u32(scale)),
    ));
    let step = e
        .call(FLOAT_STEP_004E4450, &args![x87_quiet_bits(term)])
        .f64();
    total = x87_store32(x87_add(step, x87_m32(total)));
    f32::from_bits(x87_quiet_bits(total))
}

// Translated from 00643c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AiFormulas::AVDeriveSkill` (Xbox PDB, matched by the engine map;
/// `cdecl`, result in ST0): 0 unless `0047f060` accepts the actor value.
/// Otherwise the actor value's entry (`00643bf0`: a word, initially -1, and
/// a base `float`); when the word is not -1 it is read as an actor value and
/// the result is [`ai_formulas_calculate_derived_skill_value`] of the base,
/// that value and actor value 0xB, read clamped (`0066ef50`) or, with
/// `use_virtual` set, as the owner's vtable +0x04 plus +0x18 values clamped
/// by `ActorValue::ClampActorValue`; then the setting at `011ccf5c` is added
/// when the owner's vtable +0x24 object answers its +0xF0 and +0x100 slots
/// and its class (`Actor::GetClass`, engine map) answers `005a5f40(index)`. Slot
/// meanings are not confirmed; they are described by offset.
pub fn ai_formulas_av_derive_skill(e: &mut Engine, owner: u32, index: u32, use_virtual: u8) -> f32 {
    e.with_stack(8, |e, locals| {
        let entry_word = locals.addr();
        let result = locals.addr() + 4;
        e.mem.set_u32(result, x87_store32(0.0));
        if !e.call(ACCEPTS_ACTOR_VALUE_0047F060, &args![index]).bool() {
            return f32::from_bits(e.mem.u32(result));
        }
        e.mem.set_u32(entry_word, 0xffff_ffff);
        e.call(ENTRY_00643BF0, &args![index, entry_word, result]);
        let other_index = e.mem.u32(entry_word);
        if other_index != 0xffff_ffff {
            let (other_value, skill) = if use_virtual == 0 {
                let skill = e
                    .call(GET_CLAMPED_ACTOR_FLOAT_VALUE, &args![owner, 0xbu32])
                    .f64();
                let skill = x87_store32(skill);
                let other_value = e
                    .call(GET_CLAMPED_ACTOR_FLOAT_VALUE, &args![owner, other_index])
                    .f64();
                (x87_store32(other_value), skill)
            } else {
                let skill = clamped_sum(e, owner, 0xb);
                let other_value = clamped_sum(e, owner, other_index);
                (other_value, skill)
            };
            let base = f32::from_bits(x87_quiet_bits(e.mem.u32(result)));
            let value = ai_formulas_calculate_derived_skill_value(
                e,
                base,
                f32::from_bits(other_value),
                f32::from_bits(skill),
            );
            e.mem.set_u32(result, x87_store32(f64::from(value)));
        }
        let object = e.vcall(owner, 0x24, &args![]).u32();
        if object != 0
            && e.vcall(object, 0xf0, &args![]).bool()
            && e.vcall(object, 0x100, &args![]).bool()
            && e.call(ACTOR_GET_CLASS, &args![object]).u32() != 0
        {
            let class = e.call(ACTOR_GET_CLASS, &args![object]).u32();
            if e.call(CLASS_TEST_005A5F40, &args![class, index]).bool() {
                let bonus = setting(e, 0x011c_cf5c);
                let sum = x87_add(x87_load32(e.mem.u32(result)), x87_m32(e.mem.u32(bonus)));
                e.mem.set_u32(result, x87_store32(sum));
            }
        }
        f32::from_bits(x87_quiet_bits(e.mem.u32(result)))
    })
}

/// The owner's vtable +0x04 and +0x18 values of `index` added and clamped
/// (`ActorValue::ClampActorValue`), as the `float` word the exe leaves in
/// the argument slot.
fn clamped_sum(e: &mut Engine, owner: u32, index: u32) -> u32 {
    let first = e.vcall(owner, 0x4, &args![index]).f64();
    let second = e.vcall(owner, 0x18, &args![index]).f64();
    let sum = x87_store32(x87_add(second, first));
    let clamped = e
        .call(CLAMP_ACTOR_VALUE, &args![index, x87_quiet_bits(sum)])
        .f64();
    x87_store32(clamped)
}

/// `(clamped value 7 + s[add]) * s[scale]` as a `float` result.
fn scaled_value_7(e: &mut Engine, owner: u32, add: u32, scale: u32) -> u32 {
    let value = e
        .call(GET_CLAMPED_ACTOR_FLOAT_VALUE, &args![owner, 7u32])
        .f64();
    let add = setting(e, add);
    let sum = x87_add(load(e, add), value);
    let scale = setting(e, scale);
    x87_store32(x87_mul(load(e, scale), sum))
}

// Translated from 00643e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Actor value derivation callback (`cdecl`: owner, actor value; result in
/// ST0; no Xbox PDB name): for actor value 0x13 `(clamped value 7 +
/// s[11cdf78]) * s[11cdf44]`, for 0x14 `(clamped value 7 + s[11cde50]) *
/// s[11cdce8]`, else 0.
pub fn fn_00643e40(e: &mut Engine, owner: u32, index: u32) -> f32 {
    let result = match index {
        0x13 => scaled_value_7(e, owner, 0x011c_df78, 0x011c_df44),
        0x14 => scaled_value_7(e, owner, 0x011c_de50, 0x011c_dce8),
        _ => x87_store32(0.0),
    };
    f32::from_bits(x87_quiet_bits(result))
}

// Translated from 00643ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Actor value derivation callback (`cdecl`: owner, actor value; result in
/// ST0; no Xbox PDB name): for actor value 0x38 `s[11cd3a8] + owner vtable
/// +0x0C(0x2D) * s[11cdaac]`, else 0.
pub fn fn_00643ed0(e: &mut Engine, owner: u32, index: u32) -> f32 {
    let mut result = x87_store32(0.0);
    if index == 0x38 {
        let scale = setting(e, 0x011c_daac);
        let value = e.vcall(owner, 0xc, &args![0x2du32]).f64();
        let term = x87_mul(value, x87_m32(e.mem.u32(scale)));
        let add = setting(e, 0x011c_d3a8);
        result = x87_store32(x87_add(load(e, add), term));
    }
    f32::from_bits(x87_quiet_bits(result))
}

/// Settings by the names their static initializers give them:
/// `fFollowSlowdownZoneWidth`, `fFollowStopZoneMinMult`,
/// `fFollowWalkZoneMult`, `fFollowMatchSpeedZoneWidth`,
/// `fFollowSpaceBetweenFollowers` and `iAIDistanceRadiusMinLocation`.
const SETTING_FOLLOW_SLOWDOWN_ZONE_WIDTH: u32 = 0x011c_de74;
const SETTING_FOLLOW_STOP_ZONE_MIN_MULT: u32 = 0x011c_d420;
const SETTING_FOLLOW_WALK_ZONE_MULT: u32 = 0x011c_e00c;
const SETTING_FOLLOW_MATCH_SPEED_ZONE_WIDTH: u32 = 0x011c_d85c;
const SETTING_FOLLOW_SPACE_BETWEEN_FOLLOWERS: u32 = 0x011c_d65c;
const SETTING_DISTANCE_RADIUS_MIN_LOCATION: u32 = 0x011c_de98;
/// `00404010` (`cdecl`, two `float`s, result in ST0; no Xbox PDB name).
const FLOAT_PAIR_00404010: u32 = 0x0040_4010;
/// The `double` 0.5 at `01011588`.
const HALF_01011588: u32 = 0x0101_1588;
/// `00558dd0` (`thiscall`, one word): the address of a `BSSimpleArray`
/// element by index.
const ARRAY_ELEMENT: u32 = 0x0055_8dd0;

// Translated from 00643f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one `float`): `00404010` of
/// `radius * fFollowStopZoneMinMult` and `radius -
/// fFollowSlowdownZoneWidth` (each stored as a `float` and pushed; the
/// difference is computed first), its `ST0` passed on.
pub fn fn_00643f20(e: &mut Engine, radius: f32) -> f64 {
    let width = setting(e, SETTING_FOLLOW_SLOWDOWN_ZONE_WIDTH);
    let difference = x87_store32(x87_sub(
        x87_load32(radius.to_bits()),
        x87_m32(e.mem.u32(width)),
    ));
    let multiplier = setting(e, SETTING_FOLLOW_STOP_ZONE_MIN_MULT);
    let product = x87_store32(x87_mul(
        x87_load32(radius.to_bits()),
        x87_m32(e.mem.u32(multiplier)),
    ));
    e.call(FLOAT_PAIR_00404010, &args![product, difference])
        .f64()
}

// Translated from 00643f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AiFormulas::GetFollowRadiusWalk` (engine map), cdecl: `radius *
/// fFollowWalkZoneMult`, rounded to a `float`.
pub fn ai_formulas_get_follow_radius_walk(e: &mut Engine, radius: f32) -> f32 {
    let multiplier = setting(e, SETTING_FOLLOW_WALK_ZONE_MULT);
    let product = x87_mul(x87_load32(radius.to_bits()), x87_m32(e.mem.u32(multiplier)));
    f32::from_bits(x87_store32(product))
}

// Translated from 00643f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): its argument, loaded (`FLD dword`).
pub fn fn_00643f90(_e: &mut Engine, radius: f32) -> f64 {
    x87_load32(radius.to_bits())
}

// Translated from 00643fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl): `fFollowMatchSpeedZoneWidth * 0.5
/// (the double at 01011588) + radius`, rounded to a `float`.
pub fn fn_00643fa0(e: &mut Engine, radius: f32) -> f32 {
    let width = setting(e, SETTING_FOLLOW_MATCH_SPEED_ZONE_WIDTH);
    let half = f64::from_bits(e.mem.u64(HALF_01011588));
    let product = x87_mul(load(e, width), half);
    let sum = x87_add(product, x87_m32(radius.to_bits()));
    f32::from_bits(x87_store32(sum))
}

// Translated from 00643fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AiFormulas::GetFollowRadiusMatchSpeed` (engine map), cdecl: `radius +
/// fFollowMatchSpeedZoneWidth`, rounded to a `float`.
pub fn ai_formulas_get_follow_radius_match_speed(e: &mut Engine, radius: f32) -> f32 {
    let width = setting(e, SETTING_FOLLOW_MATCH_SPEED_ZONE_WIDTH);
    let sum = x87_add(x87_load32(radius.to_bits()), x87_m32(e.mem.u32(width)));
    f32::from_bits(x87_store32(sum))
}

/// The followers counted by
/// [`ai_formulas_get_follow_radius_fudge_for_multiple_followers`]: each
/// actor of the list's nodes (from the word at +0xC) whose current package
/// (`009344a0`) answers 1 to `0041ca90`. Calls `each` with the actor and
/// its package.
fn for_each_follower(e: &mut Engine, list: u32, mut each: impl FnMut(&mut Engine, u32, u32)) {
    let mut node = e.mem.u32(list + 0xc);
    while node != 0 {
        let item = e.call(0x0068_15c0, &args![node]).u32();
        let actor = e.mem.u32(item);
        let package = e.call(0x0093_44a0, &args![actor]).u32();
        if package != 0 && e.call(0x0041_ca90, &args![package]).u32() == 1 {
            each(e, actor, package);
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
}

// Translated from 00643ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00643ff0 - the exception-unwinding frame (FS:[0]) and the security-cookie word it pushes are not translated.
/// `AiFormulas::GetFollowRadiusFudgeForMultipleFollowers` (engine map),
/// cdecl: 0.0 unless `leader` has a follower list (`00422700` of
/// `005d43c0`) whose nodes (from +0xC; none when the first is empty,
/// `008256d0`) hold followers (see [`for_each_follower`]). Then a stack
/// `BSSimpleArray<FollowerDist>` ([`fn_006442b0`], 0x10 bytes) gets for
/// each follower its actor, its distance to the leader (`004a7290` of
/// `00439ef0` on both slot-125 (+0x1F4) answers, into a 12-byte stack
/// temporary), `iAIDistanceRadiusMinLocation` (overwritten by the
/// package's radius, `006787e0` with `actor` and 0) and its own radius
/// (`008be1b0`); it is sorted (`006c8980` with [`fn_00644300`]). Walking it,
/// the running sum adds from the second element on the previous and
/// current radius (+0xC, summed and stored as a `double`) plus
/// `fFollowSpaceBetweenFollowers`, then becomes `00404010` of itself and
/// the element's +0x8; at `actor`'s element the answer is that sum minus
/// its +0x8. The array is destroyed ([`fn_006442e0`]).
pub fn ai_formulas_get_follow_radius_fudge_for_multiple_followers(
    e: &mut Engine,
    actor: u32,
    leader: u32,
) -> f32 {
    let mut answer = 0u32;
    let list = if leader != 0 {
        let process = e.call(0x005d_43c0, &args![leader]).u32();
        e.call(0x0042_2700, &args![process]).u32()
    } else {
        0
    };
    if list == 0 {
        return f32::from_bits(answer);
    }
    let mut count = 0u32;
    let first = e.mem.u32(list + 0xc);
    if !e.call(0x0082_56d0, &args![first]).bool() {
        for_each_follower(e, list, |_, _, _| count += 1);
    }
    if count == 0 {
        return f32::from_bits(answer);
    }
    e.with_stack(0x10, |e, array| {
        fn_006442b0(e, array, count);
        let mut index = 0u32;
        for_each_follower(e, list, |e, follower, package| {
            let element = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
            e.mem.set_u32(element, follower);
            let follower_position = e.vcall(follower, 0x1f4, &args![]).u32();
            let leader_position = e.vcall(leader, 0x1f4, &args![]).u32();
            let distance = e.with_stack(0xc, |e, temporary| {
                let difference = e
                    .call(
                        0x0043_9ef0,
                        &args![leader_position, temporary, follower_position],
                    )
                    .u32();
                e.call(0x004a_7290, &args![difference]).f64()
            });
            e.mem.set_u32(element + 4, x87_store32(distance));
            let minimum = e
                .call(0x0043_d4d0, &args![SETTING_DISTANCE_RADIUS_MIN_LOCATION])
                .u32();
            let minimum = e.mem.u32(minimum) as i32;
            e.mem.set_u32(element + 8, x87_store32(f64::from(minimum)));
            let own = e.call(0x008b_e1b0, &args![follower]).f64();
            e.mem.set_u32(element + 0xc, x87_store32(own));
            let radius = e.call(0x0067_87e0, &args![package, actor, 0u32]).f64();
            e.mem.set_u32(element + 8, x87_store32(radius));
            index += 1;
        });
        e.call(0x006c_8980, &args![array, 0x0064_4300u32]);
        let mut sum = 0u32;
        let mut index = 0u32;
        while index < e.call(0x0044_ddc0, &args![array]).u32() {
            let element = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
            if index > 0 {
                let previous = e.call(ARRAY_ELEMENT, &args![array, index - 1]).u32();
                let pair = x87_add(load(e, previous + 0xc), x87_m32(e.mem.u32(element + 0xc)));
                let spacing = setting(e, SETTING_FOLLOW_SPACE_BETWEEN_FOLLOWERS);
                let total = x87_add(load(e, spacing), pair);
                sum = x87_store32(x87_add(total, x87_m32(sum)));
            }
            let radius = x87_quiet_bits(e.mem.u32(element + 8));
            let running = x87_quiet_bits(sum);
            sum = x87_store32(e.call(FLOAT_PAIR_00404010, &args![running, radius]).f64());
            let element = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
            if e.mem.u32(element) == actor {
                let element = e.call(ARRAY_ELEMENT, &args![array, index]).u32();
                answer = x87_store32(x87_sub(x87_load32(sum), x87_m32(e.mem.u32(element + 8))));
                break;
            }
            index += 1;
        }
        fn_006442e0(e, array);
    });
    f32::from_bits(answer)
}

// Translated from 006442b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the `BSSimpleArray<FollowerDist>` instance (`RET 4`):
/// its vtable (`0104f3e8`), then `00644400` with `count` twice. Returns
/// `this`.
pub fn fn_006442b0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    e.mem.set_u32(this.addr(), ARRAY_VTABLE_0104F3E8);
    e.call(0x0064_4400, &args![this, count, count]);
    this
}

// Translated from 00644300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the sort comparator (cdecl, two element
/// addresses) [`fn_00644350`] gives: 1 when `first` goes before `second`,
/// -1 when `second` goes before `first`, else 0.
pub fn fn_00644300(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    if fn_00644350(e, first, second) {
        1
    } else if fn_00644350(e, second, first) {
        -1
    } else {
        0
    }
}

// Translated from 00644350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 4`), on two `FollowerDist` elements:
/// with equal +0x8 (`FUCOMPP`), whether `other`'s +0x4 is below this one's;
/// otherwise (NaN included) whether `other`'s +0x8 is below this one's
/// (`FCOMPP`; false on NaN).
pub fn fn_00644350(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let mine = load(e, this.addr() + 8);
    let theirs = load(e, other.addr() + 8);
    if theirs == mine {
        load(e, other.addr() + 4) < load(e, this.addr() + 4)
    } else {
        load(e, other.addr() + 8) < load(e, this.addr() + 8)
    }
}

// Translated from 006443d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<FollowerDist,1024>` scalar deleting destructor (engine
/// map): [`fn_006442e0`], then frees `this` (`00401030`) when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_006443d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_006442e0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0064_42e0, fn_006442e0(Ptr)),
        entry!(
            0x0064_3c20,
            ai_formulas_calculate_derived_skill_value(f32, f32, f32) -> f32
        ),
        entry!(0x0064_3c90, ai_formulas_av_derive_skill(u32, u32, u8) -> f32),
        entry!(0x0064_3e40, fn_00643e40(u32, u32) -> f32),
        entry!(0x0064_3ed0, fn_00643ed0(u32, u32) -> f32),
        entry!(0x0064_3f20, fn_00643f20(f32) -> f64),
        entry!(0x0064_3f70, ai_formulas_get_follow_radius_walk(f32) -> f32),
        entry!(0x0064_3f90, fn_00643f90(f32) -> f64),
        entry!(0x0064_3fa0, fn_00643fa0(f32) -> f32),
        entry!(
            0x0064_3fd0,
            ai_formulas_get_follow_radius_match_speed(f32) -> f32
        ),
        entry!(
            0x0064_3ff0,
            ai_formulas_get_follow_radius_fudge_for_multiple_followers(u32, u32) -> f32
        ),
        entry!(0x0064_42b0, fn_006442b0(Ptr, u32) -> Ptr),
        entry!(0x0064_4300, fn_00644300(Ptr, Ptr) -> i32),
        entry!(0x0064_4350, fn_00644350(Ptr, Ptr) -> bool),
        entry!(0x0064_43d0, fn_006443d0(Ptr, u32) -> Ptr),
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
    fn fn_006442e0_sets_the_vtable_then_releases_the_buffer() {
        check_array_destructor(0x0064_42e0, ARRAY_VTABLE_0104F3E8, ARRAY_RELEASE_008454F0);
    }
}

#[cfg(test)]
mod derivation_tests {
    use super::*;

    const SLOT_A: u32 = 0x0001_0010;
    const SLOT_B: u32 = 0x0001_0020;
    const SLOT_C: u32 = 0x0001_0030;
    const SLOT_D: u32 = 0x0001_0040;
    const SLOT_E: u32 = 0x0001_0050;

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn word(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_c000, 0x3000);
        e.register(SETTING_VALUE, |_, a| word(a[0] + 4));
        e
    }

    fn set(e: &mut Engine, setting: u32, value: f32) {
        e.mem.set_f32(setting + 4, value);
    }

    fn arg_f32(a: &[u32], i: usize) -> f32 {
        f32::from_bits(a[i])
    }

    fn set_skill_steps(e: &mut Engine) {
        set(e, 0x011c_db30, 2.0);
        set(e, 0x011c_dd84, 0.5);
        e.register(FLOAT_STEP_00406CE0, |_, a| {
            st0(f64::from(arg_f32(a, 0)) + 1.0)
        });
        e.register(FLOAT_STEP_004E4450, |_, a| {
            st0(f64::from(arg_f32(a, 0)) * 2.0)
        });
    }

    #[test]
    fn calculate_derived_skill_value_adds_both_steps() {
        let mut e = engine();
        set_skill_steps(&mut e);
        e.call_log = Some(vec![]);
        let r = e.call(0x0064_3c20, &args![1.0f32, 3.0f32, 4.0f32]).f32();
        // 1 + (3 * 2 + 1) + (4 * 0.5 * 2).
        assert_eq!(r, 12.0);
        let log = e.call_log.take().unwrap();
        let steps: Vec<_> = log
            .iter()
            .filter(|c| c.0 == FLOAT_STEP_00406CE0 || c.0 == FLOAT_STEP_004E4450)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(steps, vec![vec![6.0f32.to_bits()], vec![2.0f32.to_bits()]]);
    }

    /// An owner whose vtable +0x04 answers the index, +0x18 answers 1 and
    /// +0x24 the object `SLOT_C` returns; the object's +0xF0 and +0x100 are
    /// `SLOT_D` and `SLOT_E`.
    fn owner(e: &mut Engine) -> (u32, u32) {
        let owner = e.mem.alloc(0x10);
        let object = e.mem.alloc(0x10);
        let mut slots = vec![0u32; 0x10];
        slots[1] = SLOT_A;
        slots[0x18 / 4] = SLOT_B;
        slots[0x24 / 4] = SLOT_C;
        e.put_vtable(0x0002_0000, &slots);
        e.mem.set_u32(owner, 0x0002_0000);
        let mut slots = vec![0u32; 0x48];
        slots[0xf0 / 4] = SLOT_D;
        slots[0x100 / 4] = SLOT_E;
        e.put_vtable(0x0002_1000, &slots);
        e.mem.set_u32(object, 0x0002_1000);
        e.register(SLOT_A, |_, a| st0(f64::from(a[1])));
        e.register(SLOT_B, |_, _| st0(1.0));
        e.register(SLOT_C, |_, _| word(0));
        e.register(SLOT_D, |_, _| word(1));
        e.register(SLOT_E, |_, _| word(1));
        (owner, object)
    }

    #[test]
    fn av_derive_skill_takes_each_branch() {
        let mut e = engine();
        set_skill_steps(&mut e);
        let (owner, object) = owner(&mut e);
        e.register(ACCEPTS_ACTOR_VALUE_0047F060, |_, _| word(0));
        assert_eq!(e.call(0x0064_3c90, &args![owner, 0x20u32, 0u32]).f32(), 0.0);

        e.register(ACCEPTS_ACTOR_VALUE_0047F060, |_, a| {
            assert_eq!(a, &[0x20]);
            word(1)
        });
        e.register(ENTRY_00643BF0, |e, a| {
            assert_eq!(a[0], 0x20);
            e.mem.set_u32(a[1], 5);
            e.mem.set_f32(a[2], 10.0);
            Ret::default()
        });
        e.register(GET_CLAMPED_ACTOR_FLOAT_VALUE, |_, a| {
            st0(if a[1] == 0xb { 2.0 } else { 3.0 })
        });
        // 10 + (3 * 2 + 1) + (2 * 0.5 * 2).
        assert_eq!(
            e.call(0x0064_3c90, &args![owner, 0x20u32, 0u32]).f32(),
            19.0
        );

        // Through the vtable: 0xB gives 12, 5 gives 6; the class bonus.
        e.register(CLAMP_ACTOR_VALUE, |_, a| st0(f64::from(arg_f32(a, 1))));
        e.register_double(SLOT_C, move |_, _| word(object));
        e.register(ACTOR_GET_CLASS, |_, _| word(0x9000));
        e.register(CLASS_TEST_005A5F40, |_, a| {
            assert_eq!(a, &[0x9000, 0x20]);
            word(1)
        });
        set(&mut e, 0x011c_cf5c, 0.5);
        e.call_log = Some(vec![]);
        // 10 + (6 * 2 + 1) + (12 * 0.5 * 2) + 0.5.
        assert_eq!(
            e.call(0x0064_3c90, &args![owner, 0x20u32, 1u32]).f32(),
            35.5
        );
        let clamps: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 == CLAMP_ACTOR_VALUE)
            .map(|c| c.1)
            .collect();
        assert_eq!(
            clamps,
            vec![vec![0xb, 12.0f32.to_bits()], vec![5, 6.0f32.to_bits()]]
        );

        // A -1 entry word keeps the base value; a class without the skill
        // adds nothing.
        e.register(ENTRY_00643BF0, |e, a| {
            e.mem.set_f32(a[2], 10.0);
            Ret::default()
        });
        e.register(CLASS_TEST_005A5F40, |_, _| word(0));
        assert_eq!(
            e.call(0x0064_3c90, &args![owner, 0x20u32, 1u32]).f32(),
            10.0
        );
    }

    #[test]
    fn fn_00643e40_scales_actor_value_7_for_0x13_and_0x14() {
        let mut e = engine();
        e.register(GET_CLAMPED_ACTOR_FLOAT_VALUE, |_, a| {
            assert_eq!(a, &[0x4000, 7]);
            st0(4.0)
        });
        set(&mut e, 0x011c_df78, 1.0);
        set(&mut e, 0x011c_df44, 2.0);
        set(&mut e, 0x011c_de50, 2.0);
        set(&mut e, 0x011c_dce8, 0.5);
        assert_eq!(e.call(0x0064_3e40, &args![0x4000u32, 0x13u32]).f32(), 10.0);
        assert_eq!(e.call(0x0064_3e40, &args![0x4000u32, 0x14u32]).f32(), 3.0);
        assert_eq!(e.call(0x0064_3e40, &args![0x4000u32, 0x15u32]).f32(), 0.0);
    }

    #[test]
    fn fn_00643ed0_scales_the_owner_value_for_0x38() {
        let mut e = engine();
        let owner = e.mem.alloc(0x10);
        let mut slots = vec![0u32; 4];
        slots[3] = SLOT_A;
        e.put_vtable(0x0002_0000, &slots);
        e.mem.set_u32(owner, 0x0002_0000);
        e.register(SLOT_A, |_, a| {
            assert_eq!(a[1], 0x2d);
            st0(4.0)
        });
        set(&mut e, 0x011c_daac, 0.5);
        set(&mut e, 0x011c_d3a8, 1.0);
        assert_eq!(e.call(0x0064_3ed0, &args![owner, 0x38u32]).f32(), 3.0);
        assert_eq!(e.call(0x0064_3ed0, &args![owner, 0x37u32]).f32(), 0.0);
    }
}

#[cfg(test)]
mod n036_tests {
    use super::*;

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// Settings at their addresses, `00403e20`/`0043d4d0` answering +4.
    fn settings_engine(values: &[(u32, f32)]) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_d000, 0x2000);
        e.map(0x0101_1000, 0x1000);
        e.mem.set_u64(HALF_01011588, 0.5f64.to_bits());
        for (address, value) in values {
            e.mem.set_f32(address + 4, *value);
        }
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(0x0043_d4d0, |_, a| (a[0] + 4).into_ret());
        e
    }

    #[test]
    fn the_follow_radii_use_their_settings() {
        let mut e = settings_engine(&[
            (SETTING_FOLLOW_SLOWDOWN_ZONE_WIDTH, 50.0),
            (SETTING_FOLLOW_STOP_ZONE_MIN_MULT, 0.25),
            (SETTING_FOLLOW_WALK_ZONE_MULT, 2.0),
            (SETTING_FOLLOW_MATCH_SPEED_ZONE_WIDTH, 30.0),
        ]);
        e.register(FLOAT_PAIR_00404010, |_, a| {
            f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))).into_ret()
        });
        let radius = 200.0f32.to_bits();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_3f20, &args![radius]).f64(), 150.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FLOAT_PAIR_00404010),
            vec![vec![50.0f32.to_bits(), 150.0f32.to_bits()]]
        );
        assert_eq!(e.call(0x0064_3f70, &args![radius]).f64(), 400.0);
        assert_eq!(e.call(0x0064_3f90, &args![radius]).f64(), 200.0);
        assert_eq!(e.call(0x0064_3fa0, &args![radius]).f64(), 215.0);
        assert_eq!(e.call(0x0064_3fd0, &args![radius]).f64(), 230.0);
        // FLD quiets a signalling NaN.
        let quiet = e.call(0x0064_3f90, &args![0x7f80_0001u32]).f64();
        assert_eq!((quiet as f32).to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn the_comparator_orders_by_radius_then_distance() {
        let mut e = Engine::new();
        let a = e.mem.alloc(0x10);
        let b = e.mem.alloc(0x10);
        let set = |e: &mut Engine, p: u32, distance: f32, radius: f32| {
            e.mem.set_f32(p + 4, distance);
            e.mem.set_f32(p + 8, radius);
        };
        set(&mut e, a, 1.0, 5.0);
        set(&mut e, b, 2.0, 3.0);
        assert!(e.call(0x0064_4350, &args![a, b]).bool());
        assert_eq!(e.call(0x0064_4300, &args![a, b]).i32(), 1);
        assert_eq!(e.call(0x0064_4300, &args![b, a]).i32(), -1);
        set(&mut e, b, 2.0, 5.0);
        assert_eq!(e.call(0x0064_4300, &args![a, b]).i32(), -1);
        set(&mut e, b, 1.0, 5.0);
        assert_eq!(e.call(0x0064_4300, &args![a, b]).i32(), 0);
        set(&mut e, b, 1.0, f32::NAN);
        assert_eq!(e.call(0x0064_4300, &args![a, b]).i32(), 0);
    }

    #[test]
    fn the_array_constructor_and_deleting_destructor() {
        let mut e = Engine::new();
        for callee in [0x0064_4400, ARRAY_RELEASE_008454F0, 0x0040_1030] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_42b0, &args![this, 3u32]).u32(), this);
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE_0104F3E8);
        e.mem.set_u32(this, 0);
        assert_eq!(e.call(0x0064_43d0, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), ARRAY_VTABLE_0104F3E8);
        e.call(0x0064_43d0, &args![this, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0064_4400), vec![vec![this, 3, 3]]);
        assert_eq!(calls_to(&log, 0x0040_1030), vec![vec![this]]);
        assert_eq!(calls_to(&log, ARRAY_RELEASE_008454F0).len(), 2);
    }

    #[test]
    fn the_fudge_sums_the_spacing_up_to_the_actor() {
        let mut e = settings_engine(&[(SETTING_FOLLOW_SPACE_BETWEEN_FOLLOWERS, 10.0)]);
        e.mem.set_u32(SETTING_DISTANCE_RADIUS_MIN_LOCATION + 4, 7);
        // The leader's follower list: two followers, their packages 1.
        let list = e.mem.alloc(0x10);
        let node2 = e.mem.alloc(8);
        let node1 = e.mem.alloc(8);
        let leader = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(list + 0xc, node1);
        e.mem.set_u32(node1, first);
        e.mem.set_u32(node1 + 4, node2);
        e.mem.set_u32(node2, second);
        e.put_vtable(0x0206_0000, &[0; 0x80]);
        e.mem.set_u32(0x0206_0000 + 0x1f4, 0x0090_01f4);
        e.register(0x0090_01f4, |_, a| a[0].into_ret());
        for p in [leader, first, second] {
            e.mem.set_u32(p, 0x0206_0000);
        }
        e.register(0x005d_43c0, |_, a| a[0].into_ret());
        e.register_double(0x0042_2700, move |_, _| list.into_ret());
        e.register(0x0082_56d0, |e, a| (e.mem.u32(a[0]) == 0).into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x0072_6070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0093_44a0, |_, a| (a[0] + 1).into_ret());
        e.register(0x0041_ca90, |_, _| 1u32.into_ret());
        e.register(0x0043_9ef0, |_, a| a[1].into_ret());
        e.register(0x004a_7290, |_, _| 100.0f64.into_ret());
        e.register(0x008b_e1b0, |_, _| 20.0f64.into_ret());
        e.register(0x0067_87e0, |_, a| f64::from(a[0] & 0xff).into_ret());
        // A plain array: buffer at +4, size at +8.
        e.register(0x0064_4400, |e, a| {
            let buffer = e.mem.alloc(a[1] * 0x10);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(ARRAY_ELEMENT, |e, a| {
            (e.mem.u32(a[0] + 4) + a[1] * 0x10).into_ret()
        });
        e.register(0x0044_ddc0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x006c_8980, |_, _| Ret::default());
        e.register(ARRAY_RELEASE_008454F0, |_, _| Ret::default());
        e.register(FLOAT_PAIR_00404010, |_, a| {
            f64::from(f32::from_bits(a[0]).max(f32::from_bits(a[1]))).into_ret()
        });
        let radius_of = |p: u32| ((p + 1) & 0xff) as f32;
        e.call_log = Some(vec![]);
        let answer = e.call(0x0064_3ff0, &args![second, leader]).f64();
        // First: max(0, r1) = r1. Second: r1 + 20 + 20 + 10, max with r2,
        // minus r2.
        let r1 = radius_of(first);
        let r2 = radius_of(second);
        let want = (r1 + 50.0).max(r2) - r2;
        assert_eq!(answer as f32, want);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, 0x006c_8980),
            vec![vec![calls_to(&log, 0x0064_4400)[0][0], 0x0064_4300]]
        );
        assert_eq!(calls_to(&log, 0x0067_87e0)[0][1..], [second, 0]);
        assert_eq!(calls_to(&log, ARRAY_RELEASE_008454F0).len(), 1);

        // No leader: 0.0 without a call.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_3ff0, &args![second, 0u32]).f64(), 0.0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
