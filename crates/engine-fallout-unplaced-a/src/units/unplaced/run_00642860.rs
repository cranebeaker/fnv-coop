//! `(unplaced)/run_00642860` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A run of game-setting formulas without Xbox PDB names (the AI and actor
//! value code calls them): most read game settings through `00403e20` (the
//! address of a setting's value, `thiscall` on the `Setting`), compute in x87
//! and store `float`s. The x87 arithmetic goes through `crate::x87`, one call
//! per instruction in the exe's order. `s[xxxxxxxx]` below is the value of
//! the setting at that address.

#[allow(unused_imports)]
use crate::prelude::*;

/// `00403e20` (`thiscall` on a setting): the address of its value (+4), or of
/// a zeroed scratch `float` at `01202800` without a setting.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `0043d4d0` (`thiscall` on a setting): the address of its integer value
/// (+4), or of a zeroed scratch word without a setting.
const SETTING_INT_VALUE: u32 = 0x0043_d4d0;
/// `_ftol2` (`00ec62c0`): the `f64` in ST0 truncated to an integer in EAX
/// (the value is a leading `f64` argument).
const FTOL: u32 = 0x00ec_62c0;
/// `RandomFloat` (`00476b70`, engine map; `cdecl`, two `float`s, result in
/// ST0).
const RANDOM_FLOAT: u32 = 0x0047_6b70;
/// `00404010` (`cdecl`, two `float`s, result in ST0; no Xbox PDB name).
const FLOAT_PAIR_00404010: u32 = 0x0040_4010;
/// `_pow` (`00ec9ac0`, engine map; `cdecl`, two `double`s, result in ST0).
const POW: u32 = 0x00ec_9ac0;
/// `00559450` (`thiscall`, no stack words; no Xbox PDB name): the object a
/// smart-pointer member holds.
const POINTER_GET_00559450: u32 = 0x0055_9450;
/// `NiAVObject::GetProperty` (`00a59d30`, engine map; `thiscall`, one word:
/// the property type).
const NI_AV_OBJECT_GET_PROPERTY: u32 = 0x00a5_9d30;
/// `00441110` (`thiscall`, no stack words): the engine map's name
/// (`PathingLocation::GetWorldspace`) is a folded body; its result is
/// compared with 0xD.
const PROPERTY_KIND_00441110: u32 = 0x0044_1110;
/// `00b64d60` (`thiscall`, no stack words; no Xbox PDB name).
const OBJECT_STEP_00B64D60: u32 = 0x00b6_4d60;
/// `00653270` (`cdecl`, two words; no Xbox PDB name).
const LOOKUP_00653270: u32 = 0x0065_3270;
/// `006348e0` (`thiscall`, one word; no Xbox PDB name).
const STORE_006348E0: u32 = 0x0063_48e0;
/// `00634900` (`thiscall`, one word; no Xbox PDB name).
const SET_POINTER_00634900: u32 = 0x0063_4900;
/// `00647b70` (`cdecl`, two words; no Xbox PDB name).
const INT_PAIR_00647B70: u32 = 0x0064_7b70;
/// `ActorValue::ClampActorValue` (`0066f190`, engine map; `cdecl`: actor
/// value, `float`; result in ST0).
const CLAMP_ACTOR_VALUE: u32 = 0x0066_f190;
/// `ActorValueOwner::GetClampedActorValue` (`0066ef20`, engine map;
/// `thiscall`, one word: the actor value).
const GET_CLAMPED_ACTOR_VALUE: u32 = 0x0066_ef20;
/// `ActorValueOwner::GetClampedActorFloatValue` (`0066ef50`, engine map;
/// `thiscall`, one word; result in ST0).
const GET_CLAMPED_ACTOR_FLOAT_VALUE: u32 = 0x0066_ef50;
/// `0087f4e0` (`thiscall`, no stack words; returns a `bool` in AL; no Xbox
/// PDB name).
const ACTOR_TEST_0087F4E0: u32 = 0x0087_f4e0;
/// `004181e0` (`thiscall`, no stack words; returns an object; no Xbox PDB
/// name).
const ACTOR_GET_004181E0: u32 = 0x0041_81e0;
/// `00603fc0` (`thiscall`, one word; returns a word; no Xbox PDB name).
const COUNT_00603FC0: u32 = 0x0060_3fc0;
/// `006040d0` (`thiscall`, one word; returns a word; no Xbox PDB name).
const COUNT_006040D0: u32 = 0x0060_40d0;
/// `AiFormulas::AVDeriveSkill` (`00643c90`, engine map; `cdecl`, three
/// words; result in ST0).
const AV_DERIVE_SKILL: u32 = 0x0064_3c90;

/// The first argument [`fn_00642940`] passes to `00653270`.
const LOOKUP_KEY_012005B8: u32 = 0x0120_05b8;
/// The pointer [`fn_00642940`] passes to `00634900`.
const POINTER_ARGUMENT_011F49E0: u32 = 0x011f_49e0;
/// `24.0` (`double`).
const HOURS_PER_DAY: u32 = 0x0103_56d8;
/// `0.0` (`double`).
const DOUBLE_ZERO: u32 = 0x0101_2060;
/// `1.0` (`double`).
const DOUBLE_ONE: u32 = 0x0101_2070;
/// The `double` [`fn_006434f0`] divides by and [`fn_00642ed0`] compares its
/// mode word with.
const DIVISOR_01011590: u32 = 0x0101_1590;
/// The `double` [`fn_00642ed0`] divides its first word by.
const TENTHS_DIVISOR_01020758: u32 = 0x0102_0758;
/// [`fn_00643540`]'s table: 16-byte rows of four setting pointers.
const RANGE_TABLE_011CD680: u32 = 0x011c_d680;
/// [`fn_00643bf0`]'s tables: a word per index, and a setting per index.
const WORD_TABLE_0119B144: u32 = 0x0119_b144;
const SETTING_TABLE_0119B17C: u32 = 0x0119_b17c;
/// [`fn_006435c0`]'s two tables of setting pointers, by index.
const SETTING_TABLE_0119B194: u32 = 0x0119_b194;
const SETTING_TABLE_0119B1AC: u32 = 0x0119_b1ac;

/// The address of a setting's value (`00403e20`).
fn setting(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_VALUE, &args![setting]).u32()
}

/// The address of a setting's integer value (`0043d4d0`).
fn int_setting(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_INT_VALUE, &args![setting]).u32()
}

/// `_ftol2` on the value in ST0.
fn ftol(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

/// `FLD dword [address]`.
fn load(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

/// A `float` memory operand at `address` (`FADD`/`FMUL`/... `dword`).
fn operand(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

/// The `float` result of an `FSTP dword` of `value`, as the next `FLD`
/// sees it.
fn round_float(value: f64) -> f64 {
    x87_load32(x87_store32(value))
}

/// The value an `FLD dword` of `bits` returns in ST0 as a `float` result.
fn float_result(bits: u32) -> f32 {
    f32::from_bits(x87_quiet_bits(bits))
}

// Translated from 00642860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at +0x24 and, when the smart pointer at +0x28 holds an
/// object, runs `00b64d60` on it (`thiscall`; no Xbox PDB name).
pub fn fn_00642860(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x24, 1);
    let object = e
        .call(POINTER_GET_00559450, &args![this.addr() + 0x28])
        .u32();
    if object != 0 {
        e.call(OBJECT_STEP_00B64D60, &args![object]);
    }
}

// Translated from 00642890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the byte at +0x24 (`thiscall`; no Xbox PDB name).
pub fn fn_00642890(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x24, 0);
}

/// Property 3 of the object the smart pointer at `member` holds.
fn property_3(e: &mut Engine, member: u32) -> u32 {
    let object = e.call(POINTER_GET_00559450, &args![member]).u32();
    e.call(NI_AV_OBJECT_GET_PROPERTY, &args![object, 3u32])
        .u32()
}

// Translated from 006428b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` at +0x6C of property 3 of the object held at +0x14 when that
/// property exists and `00441110` on it answers 0xD, else 0 (`thiscall`,
/// result in ST0; no Xbox PDB name). The property is fetched again for each
/// use, as the exe does.
pub fn fn_006428b0(e: &mut Engine, this: Ptr) -> f32 {
    let member = this.addr() + 0x14;
    let matches = if property_3(e, member) == 0 {
        false
    } else {
        let property = property_3(e, member);
        e.call(PROPERTY_KIND_00441110, &args![property]).u32() == 0xd
    };
    let property = if matches { property_3(e, member) } else { 0 };
    if property != 0 {
        float_result(e.mem.u32(property + 0x6c))
    } else {
        0.0
    }
}

// Translated from 00642940 (decompiled, FalloutNV.exe 1.4.0.525)
/// For property 3 of the objects held at +0x10 and then +0x14: looks it up
/// with `00653270(012005b8, property)` and, when that finds something, runs
/// `006348e0(found, 0)` and `00634900(found, 011f49e0)` (`thiscall`; no Xbox
/// PDB name).
pub fn fn_00642940(e: &mut Engine, this: Ptr) {
    for member in [this.addr() + 0x10, this.addr() + 0x14] {
        let property = property_3(e, member);
        let found = e
            .call(LOOKUP_00653270, &args![LOOKUP_KEY_012005B8, property])
            .u32();
        if found != 0 {
            e.call(STORE_006348E0, &args![found, 0u32]);
            e.call(
                SET_POINTER_00634900,
                &args![found, POINTER_ARGUMENT_011F49E0],
            );
        }
    }
}

// Translated from 006429e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A random hour between `from` and `to` across midnight (`cdecl`, result
/// in ST0; no Xbox PDB name): when `to < from` the range's end is moved a
/// day (24, the `double` at `010356d8`) later; `RandomFloat(from, end)`
/// less 24 when that is not negative, else the random value itself.
pub fn fn_006429e0(e: &mut Engine, from: f32, to: f32) -> f32 {
    let start = x87_load32(from.to_bits());
    let end = x87_load32(to.to_bits());
    let day: f64 = e.global(HOURS_PER_DAY);
    // FCOMPP of `to` against `from`, TEST AH,5 / JP: the add runs only for
    // `to < from` (unordered jumps).
    let end = if end < start { x87_add(end, day) } else { end };
    let end_bits = x87_store32(end);
    let random = e
        .call(
            RANDOM_FLOAT,
            &args![x87_quiet_bits(from.to_bits()), x87_quiet_bits(end_bits)],
        )
        .f64();
    let random = round_float(random);
    let wrapped = round_float(x87_sub(random, day));
    let zero: f64 = e.global(DOUBLE_ZERO);
    // FCOMP with the `double` 0, TEST AH,1 / JNZ: below zero or unordered
    // keeps the random value.
    let result = if wrapped >= zero { wrapped } else { random };
    float_result(x87_store32(result))
}

// Translated from 00642a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// An integer sum of setting-weighted terms (`cdecl`, eleven words; no Xbox
/// PDB name): `base`, `(level - s[11cd014]) * s[11cdf38] + s[11cddec]`,
/// unless `skip` is set `(count * s[11cdc28] + s[11cd228]) * (scaled *
/// s[11ccfe4])`, `s[11cdfa8] * factor`, `(second * s[11cddc4] + s[11cd890]) *
/// (first * s[11cddc4])`, unless `skip` is set `s[11cd2d0] * weight`,
/// `s[11cd438] * other` and `extra`, each product truncated by `_ftol2`.
/// Argument names describe their use only.
#[allow(clippy::too_many_arguments)]
pub fn fn_00642a60(
    e: &mut Engine,
    base: i32,
    level: i32,
    count: i32,
    scaled: i32,
    factor: i32,
    first: i32,
    second: i32,
    weight: i32,
    other: i32,
    extra: i32,
    skip: u8,
) -> i32 {
    let offset = setting(e, 0x011c_ddec);
    let scale = setting(e, 0x011c_df38);
    let level_value = f64::from(level);
    let level_base = setting(e, 0x011c_d014);
    let term = x87_sub(level_value, load(e, level_base));
    let term = x87_mul(term, operand(e, scale));
    let term = x87_add(term, operand(e, offset));
    let level_term = ftol(e, term);

    let mut count_term = 0;
    if skip == 0 {
        let per = setting(e, 0x011c_cfe4);
        let product = x87_mul(f64::from(scaled), operand(e, per));
        let add = setting(e, 0x011c_d228);
        let mul = setting(e, 0x011c_dc28);
        let term = x87_mul(f64::from(count), operand(e, mul));
        let term = x87_add(term, operand(e, add));
        let term = x87_mul(term, product);
        count_term = ftol(e, term);
    }

    let factor_setting = setting(e, 0x011c_dfa8);
    let factor_term = ftol(e, load(e, factor_setting)).wrapping_mul(factor);

    let first_value = f64::from(first);
    let first_setting = setting(e, 0x011c_ddc4);
    let first_product = x87_mul(load(e, first_setting), first_value);
    let add = setting(e, 0x011c_d890);
    let second_value = f64::from(second);
    let second_setting = setting(e, 0x011c_ddc4);
    let term = x87_mul(load(e, second_setting), second_value);
    let term = x87_add(term, operand(e, add));
    let term = x87_mul(term, first_product);
    let pair_term = ftol(e, term);

    let mut weight_term = 0;
    if skip == 0 {
        let weight_setting = setting(e, 0x011c_d2d0);
        weight_term = ftol(e, load(e, weight_setting)).wrapping_mul(weight);
    }

    let other_setting = setting(e, 0x011c_d438);
    let other_term = ftol(e, load(e, other_setting)).wrapping_mul(other);

    base.wrapping_add(level_term)
        .wrapping_add(count_term)
        .wrapping_add(factor_term)
        .wrapping_add(pair_term)
        .wrapping_add(weight_term)
        .wrapping_add(other_term)
        .wrapping_add(extra)
}

// Translated from 00642e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `value / s[11cd240]` truncated when the setting is below `value`, else 1
/// (`cdecl`; no Xbox PDB name).
pub fn fn_00642e80(e: &mut Engine, value: f32) -> i32 {
    let mut result = 1;
    let limit = setting(e, 0x011c_d240);
    // FCOMPP of the setting against `value`, TEST AH,5 / JP: only
    // `setting < value` divides (unordered jumps).
    if load(e, limit) < x87_load32(value.to_bits()) {
        let divisor = setting(e, 0x011c_d240);
        let quotient = x87_div(x87_load32(value.to_bits()), operand(e, divisor));
        result = ftol(e, quotient);
    }
    result
}

// Translated from 00642ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A detection formula (`cdecl`, 25 words; no Xbox PDB name; its caller is
/// `GetDetectionLevelAgainstActor` per the engine map): computes three
/// `float` terms from settings and the arguments, stores them at `out_first`,
/// `out_second` and `out_third`, and returns their sum with the setting at
/// `011cd7e4` as an integer: 1 when the sum lies strictly between 0 and 1,
/// else the sum truncated by `_ftol2`. Argument names describe their use
/// only.
#[allow(clippy::too_many_arguments)]
pub fn fn_00642ed0(
    e: &mut Engine,
    tenths: i32,
    base: i32,
    sign_word: i32,
    multiplier: i32,
    distance: f32,
    added: i32,
    percent: i32,
    zero_multiplier: u8,
    scaled: i32,
    flag_a: u8,
    flag_b: u8,
    flag_c: u8,
    flag_d: u8,
    flag_e: u8,
    product_factor: i32,
    count: i32,
    long_range: u8,
    flag_f: u8,
    mode: i32,
    out_first: Ptr,
    out_second: Ptr,
    out_third: Ptr,
    subtract_a: i32,
    subtract_b: i32,
    subtract_c: i32,
) -> i32 {
    let total_setting = setting(e, 0x011c_d7e4);
    let total = x87_store32(load(e, total_setting));

    let mut offset = base;
    let start = int_setting(e, 0x011c_d8c0);
    let mut limit = e.mem.u32(start) as i32;
    let per = int_setting(e, 0x011c_d13c);
    offset = subtract_b
        .wrapping_sub(subtract_a)
        .wrapping_mul(e.mem.u32(per) as i32)
        .wrapping_add(offset);
    let per = int_setting(e, 0x011c_d4c0);
    limit = limit.wrapping_sub(subtract_b.wrapping_mul(e.mem.u32(per) as i32));
    let pair = e.call(INT_PAIR_00647B70, &args![limit, 0u32]).i32();
    offset = pair.wrapping_add(offset);
    offset = offset.wrapping_sub(subtract_c);

    let flag_e_word = i32::from(flag_e != 0);
    let flag_d_word = i32::from(flag_d != 0);
    let flag_f_word = i32::from(flag_f != 0);

    let mut mode_bonus = x87_store32(0.0);
    if mode == 1 {
        let bonus = setting(e, 0x011c_dbf8);
        mode_bonus = x87_quiet_bits(e.mem.u32(bonus));
    } else {
        let compare: f64 = e.global(DIVISOR_01011590);
        // FCOMP, TEST AH,0x44 / JP: only an equal value takes the bonus.
        if f64::from(mode) == compare {
            let bonus = setting(e, 0x011c_d1e0);
            mode_bonus = x87_quiet_bits(e.mem.u32(bonus));
        }
    }

    let low = setting(e, 0x011c_df04);
    let high = setting(e, 0x011c_dd9c);
    let low_again = setting(e, 0x011c_df04);
    let span = x87_sub(load(e, high), operand(e, low_again));
    let tenths_divisor: f64 = e.global(TENTHS_DIVISOR_01020758);
    let fraction = x87_div(f64::from(tenths), tenths_divisor);
    let base_term = x87_mul(span, fraction);
    let base_term = x87_add(base_term, operand(e, low));
    let base_term = x87_store32(base_term);

    let flag_e_value = f64::from(flag_e_word);
    let weight = setting(e, 0x011c_dfc0);
    let sum = x87_mul(load(e, weight), flag_e_value);
    let sum = x87_add(sum, 1.0);
    let flag_d_value = f64::from(flag_d_word);
    let weight = setting(e, 0x011c_d774);
    let product = x87_mul(load(e, weight), flag_d_value);
    let sum = x87_add(product, sum);
    let flag_f_value = f64::from(flag_f_word);
    let weight = setting(e, 0x011c_dbd4);
    let product = x87_mul(load(e, weight), flag_f_value);
    let sum = x87_add(product, sum);
    let sum = x87_add(sum, x87_m32(mode_bonus));
    let sum = x87_mul(sum, x87_m32(base_term));
    let base_bits = x87_store32(sum);

    let held = if zero_multiplier != 0 { 0 } else { multiplier };
    let positive = i32::from(sign_word > 0);
    let flag_b_word = i32::from(flag_b != 0);
    let flag_c_word = i32::from(flag_c != 0);
    let flag_a_word = i32::from(flag_a != 0);

    let range = if long_range != 0 {
        let scale = setting(e, 0x011c_dcbc);
        let range = setting(e, 0x011c_d7d8);
        x87_mul(load(e, scale), operand(e, range))
    } else {
        let range = setting(e, 0x011c_d7d8);
        load(e, range)
    };
    let range_bits = x87_store32(range);
    let ratio = x87_sub(x87_load32(range_bits), x87_m32(distance.to_bits()));
    let ratio = x87_div(ratio, x87_m32(range_bits));
    let ratio_bits = x87_quiet_bits(x87_store32(ratio));
    let clamped = e
        .call(FLOAT_PAIR_00404010, &args![ratio_bits, x87_store32(0.0)])
        .f64();
    let clamped_bits = x87_store32(clamped);
    let exponent = setting(e, 0x011c_d304);
    let exponent_value = load(e, exponent);
    let power = e
        .call(POW, &args![x87_load32(clamped_bits), exponent_value])
        .f64();
    let power_bits = x87_store32(power);
    let mut scaled_power = x87_quiet_bits(power_bits);
    if positive == 0 && mode != 1 {
        let factor = setting(e, 0x011c_d2e8);
        scaled_power = x87_store32(x87_mul(x87_load32(power_bits), operand(e, factor)));
    }

    let flag_a_value = f64::from(flag_a_word);
    let add = setting(e, 0x011c_d258);
    let per = setting(e, 0x011c_dc80);
    let term = x87_mul(f64::from(scaled), operand(e, per));
    let term = x87_add(term, operand(e, add));
    let term = x87_mul(term, flag_a_value);
    let scaled_bits = x87_store32(term);
    let mut factor_bits = x87_store32(1.0);
    if flag_c_word != 0 {
        let factor = setting(e, 0x011c_d148);
        factor_bits = x87_quiet_bits(e.mem.u32(factor));
    }
    let scaled_bits = x87_store32(x87_mul(x87_load32(scaled_bits), x87_m32(factor_bits)));
    let per = setting(e, 0x011c_d080);
    let product_term = ftol(e, load(e, per)).wrapping_mul(product_factor);
    let weight = setting(e, 0x011c_e024);
    let left = x87_mul(x87_load32(scaled_power), operand(e, weight));
    let right = x87_add(f64::from(product_term), x87_m32(scaled_bits));
    let first = x87_store32(x87_mul(left, right));
    let first = e
        .call(
            FLOAT_PAIR_00404010,
            &args![x87_quiet_bits(first), x87_store32(0.0)],
        )
        .f64();
    let first_bits = x87_store32(first);
    e.mem.set_u32(out_first.addr(), x87_quiet_bits(first_bits));

    let flag_a_value = f64::from(flag_a_word);
    let weight = setting(e, 0x011c_dd18);
    let sum = x87_mul(load(e, weight), flag_a_value);
    let sum = x87_add(sum, 1.0);
    let flag_c_value = f64::from(flag_c_word);
    let weight = setting(e, 0x011c_d2dc);
    let product = x87_mul(load(e, weight), flag_c_value);
    let sum_bits = x87_store32(x87_add(product, sum));
    let add = setting(e, 0x011c_db84);
    let added_term = x87_add(f64::from(added), operand(e, add));
    let scale = setting(e, 0x011c_d08c);
    let added_bits = x87_store32(x87_mul(load(e, scale), added_term));
    let percent_word = 100i32.wrapping_sub(percent) / 100;
    let percent_bits = x87_store32(f64::from(percent_word));
    let count_factor = if count <= 1 {
        1.0
    } else {
        let one: f64 = e.global(DOUBLE_ONE);
        let extra = x87_sub(f64::from(count), one);
        let weight = setting(e, 0x011c_deec);
        let term = x87_mul(load(e, weight), extra);
        x87_add(term, 1.0)
    };
    let count_bits = x87_store32(count_factor);
    let second = x87_mul(f64::from(held), x87_m32(power_bits));
    let second = x87_mul(second, x87_m32(sum_bits));
    let second = x87_mul(second, x87_m32(added_bits));
    let second = x87_mul(second, x87_m32(percent_bits));
    let second = x87_mul(second, x87_m32(count_bits));
    let second = x87_store32(second);
    let second = e
        .call(
            FLOAT_PAIR_00404010,
            &args![x87_quiet_bits(second), x87_store32(0.0)],
        )
        .f64();
    let mut second_bits = x87_store32(second);
    if zero_multiplier != 0 {
        let factor = setting(e, 0x011c_d310);
        second_bits = x87_store32(x87_mul(x87_load32(second_bits), operand(e, factor)));
    }
    e.mem
        .set_u32(out_second.addr(), x87_quiet_bits(second_bits));

    let offset_bits = x87_store32(f64::from(offset.wrapping_mul(flag_b_word)));
    let distance_term = x87_store32(x87_mul(x87_load32(base_bits), x87_m32(power_bits)));
    let weight = setting(e, 0x011c_da04);
    let third = x87_sub(x87_load32(distance_term), x87_m32(offset_bits));
    let third_bits = x87_store32(x87_mul(third, operand(e, weight)));
    e.mem.set_u32(out_third.addr(), x87_quiet_bits(third_bits));

    let sum = x87_add(x87_load32(total), x87_m32(first_bits));
    let sum = x87_add(sum, x87_m32(second_bits));
    let sum = x87_add(sum, x87_m32(third_bits));
    let value = x87_load32(x87_store32(sum));
    let one: f64 = e.global(DOUBLE_ONE);
    let zero: f64 = e.global(DOUBLE_ZERO);
    // FCOMP with 1.0, TEST AH,5 / JP: below one goes on; FCOMP with 0.0,
    // TEST AH,0x41 / JNZ: above zero answers 1. Everything else (NaN
    // included) is truncated.
    if value < one && value > zero {
        1
    } else {
        ftol(e, value)
    }
}

// Translated from 006434f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `(s[11cdb0c] - (level * s[11cd7f0] + s[11cd550])) / d[01011590]` as a
/// `float` (`cdecl`, two words, the second unused; result in ST0; no Xbox
/// PDB name).
pub fn fn_006434f0(e: &mut Engine, level: i32, _unused_2: u32) -> f32 {
    let add = setting(e, 0x011c_d550);
    let per = setting(e, 0x011c_d7f0);
    let term = x87_mul(f64::from(level), operand(e, per));
    let term = x87_store32(x87_add(term, operand(e, add)));
    let top = setting(e, 0x011c_db0c);
    let difference = x87_sub(load(e, top), x87_m32(term));
    let divisor: f64 = e.global(DIVISOR_01011590);
    let result = x87_store32(x87_div(difference, divisor));
    float_result(result)
}

// Translated from 00643540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Picks two random values from row `index` of the setting table at
/// `011cd680` (`cdecl`; no Xbox PDB name): an hour between the settings of
/// words 0 and 1 ([`fn_006429e0`]) into `*hour` and `RandomFloat` between
/// the settings of words 2 and 3 into `*value`.
pub fn fn_00643540(e: &mut Engine, index: i32, hour: Ptr, value: Ptr) {
    let row = (index as u32)
        .wrapping_shl(4)
        .wrapping_add(RANGE_TABLE_011CD680);
    let to_setting = e.mem.u32(row + 4);
    let to = setting(e, to_setting);
    let to = float_result(e.mem.u32(to));
    let from_setting = e.mem.u32(row);
    let from = setting(e, from_setting);
    let from = float_result(e.mem.u32(from));
    let picked = fn_006429e0(e, from, to);
    e.mem.set_u32(hour.addr(), x87_store32(f64::from(picked)));
    let high_setting = e.mem.u32(row + 0xc);
    let high = setting(e, high_setting);
    let high = x87_quiet_bits(e.mem.u32(high));
    let low_setting = e.mem.u32(row + 8);
    let low = setting(e, low_setting);
    let low = x87_quiet_bits(e.mem.u32(low));
    let random = e.call(RANDOM_FLOAT, &args![low, high]).f64();
    e.mem.set_u32(value.addr(), x87_store32(random));
}

// Translated from 006435c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A random amount around `(level * s[11cdc10] * s[table2[kind]] + 1) *
/// s[11cdc74] * s[table1[kind]]`, between `(1 - s[11cd1d4])` and `(1 +
/// s[11cd1d4])` times it (`cdecl`; result in ST0; no Xbox PDB name; the
/// tables of setting pointers are at `0119b194` and `0119b1ac`).
pub fn fn_006435c0(e: &mut Engine, level: i32, kind: i32) -> f32 {
    let level_bits = x87_store32(f64::from(level));
    let base = setting(e, 0x011c_dc74);
    let entry = e
        .mem
        .u32(SETTING_TABLE_0119B194.wrapping_add((kind as u32).wrapping_mul(4)));
    let factor = setting(e, entry);
    let amount_bits = x87_store32(x87_mul(load(e, base), operand(e, factor)));
    let base = setting(e, 0x011c_dc10);
    let entry = e
        .mem
        .u32(SETTING_TABLE_0119B1AC.wrapping_add((kind as u32).wrapping_mul(4)));
    let factor = setting(e, entry);
    let scale_bits = x87_store32(x87_mul(load(e, base), operand(e, factor)));
    let term = x87_mul(x87_load32(level_bits), x87_m32(scale_bits));
    let term = x87_add(term, 1.0);
    let amount_bits = x87_store32(x87_mul(term, x87_m32(amount_bits)));
    let spread = setting(e, 0x011c_d1d4);
    // FLD1; FSUBRP (DE E1): ST(1) = ST(0) - ST(1), so 1 - setting.
    let low = x87_sub(1.0, load(e, spread));
    let low = x87_store32(x87_mul(low, x87_m32(amount_bits)));
    let spread = setting(e, 0x011c_d1d4);
    let high = x87_add(load(e, spread), 1.0);
    let high = x87_store32(x87_mul(high, x87_m32(amount_bits)));
    let random = e
        .call(
            RANDOM_FLOAT,
            &args![x87_quiet_bits(low), x87_quiet_bits(high)],
        )
        .f64();
    float_result(x87_store32(random))
}

// Translated from 00643670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Actor value derivation callback (`cdecl`: owner, actor value; the actor
/// value word is not read; result in ST0; no Xbox PDB name): actor value 7
/// of the owner (vtable +0x20), clamped (`ActorValue::ClampActorValue(7,
/// value)`), through [`fn_006436c0`].
pub fn fn_00643670(e: &mut Engine, owner: u32, _unused_2: u32) -> f32 {
    let value = e.vcall(owner, 0x20, &args![7u32]).f64();
    let value = x87_store32(value);
    let clamped = e
        .call(CLAMP_ACTOR_VALUE, &args![7u32, x87_quiet_bits(value)])
        .f64();
    let clamped = float_result(x87_store32(clamped));
    fn_006436c0(e, owner, clamped)
}

/// The actor of an actor value owner at +0xA4 (null stays null).
fn owner_actor(owner: u32) -> u32 {
    if owner != 0 {
        owner.wrapping_sub(0xa4)
    } else {
        0
    }
}

// Translated from 006436c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `float` from an actor value owner (`cdecl`; result in ST0; no Xbox PDB
/// name): for the actor 0xA4 before `owner`, with its vtable +0x360 answer
/// `(value + s[11cd120]) * s[11cd270]`, else with its vtable +0x218 answer
/// `(value + s[11cd27c]) * s[11cd544]`, else 0; then when `0087f4e0` and the
/// +0x218 answer hold but not the +0x360 one, the count `00603fc0(0)` of
/// the actor's `004181e0` object instead, or with the +0x360 answer and a
/// positive count from the vtable +0x28 of the object at actor +0xA4,
/// `(count - 1) * s[11cdea4]` added.
pub fn fn_006436c0(e: &mut Engine, owner: u32, value: f32) -> f32 {
    let mut result = x87_store32(0.0);
    let actor = owner_actor(owner);
    let first = e.vcall(actor, 0x218, &args![]).bool();
    let second = e.vcall(actor, 0x360, &args![]).bool();
    if second {
        let add = setting(e, 0x011c_d120);
        let sum = x87_add(x87_load32(value.to_bits()), operand(e, add));
        let scale = setting(e, 0x011c_d270);
        result = x87_store32(x87_mul(load(e, scale), sum));
    } else if first {
        let add = setting(e, 0x011c_d27c);
        let sum = x87_add(x87_load32(value.to_bits()), operand(e, add));
        let scale = setting(e, 0x011c_d544);
        result = x87_store32(x87_mul(load(e, scale), sum));
    }
    if e.call(ACTOR_TEST_0087F4E0, &args![actor]).bool() && first && !second {
        let object = e.call(ACTOR_GET_004181E0, &args![actor]).u32();
        let count = e.call(COUNT_00603FC0, &args![object, 0u32]).u32();
        result = x87_store32(f64::from(count));
    } else if second {
        let holder = actor.wrapping_add(0xa4);
        if e.vcall(holder, 0x28, &args![]).u16() > 0 {
            let count = i32::from(e.vcall(holder, 0x28, &args![]).u16()) - 1;
            let weight = setting(e, 0x011c_dea4);
            let term = x87_mul(load(e, weight), f64::from(count));
            result = x87_store32(x87_add(term, x87_m32(result)));
        }
    }
    float_result(result)
}

// Translated from 00643820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Actor value derivation callback (`cdecl`: owner, actor value; the actor
/// value word is not read; result in ST0; no Xbox PDB name): actor value 7
/// of the owner (vtable +0x20), clamped, through [`fn_00643870`].
pub fn fn_00643820(e: &mut Engine, owner: u32, _unused_2: u32) -> f32 {
    let value = e.vcall(owner, 0x20, &args![7u32]).f64();
    let value = x87_store32(value);
    let clamped = e
        .call(CLAMP_ACTOR_VALUE, &args![7u32, x87_quiet_bits(value)])
        .f64();
    let clamped = float_result(x87_store32(clamped));
    fn_00643870(e, owner, clamped)
}

// Translated from 00643870 (decompiled, FalloutNV.exe 1.4.0.525)
/// The companion of [`fn_006436c0`] (`cdecl`; result in ST0; no Xbox PDB
/// name): with the vtable +0x360 answer `value * s[11cd7b0] + s[11cd638]`,
/// else with the +0x218 answer `value * s[11ce018] + s[11cd974]`, else 0;
/// then the count `006040d0(1)` or `(count - 1) * s[11cdadc]` added, under
/// the same conditions.
pub fn fn_00643870(e: &mut Engine, owner: u32, value: f32) -> f32 {
    let mut result = x87_store32(0.0);
    let actor = owner_actor(owner);
    let first = e.vcall(actor, 0x218, &args![]).bool();
    let second = e.vcall(actor, 0x360, &args![]).bool();
    if second {
        let add = setting(e, 0x011c_d638);
        let scale = setting(e, 0x011c_d7b0);
        let term = x87_mul(x87_load32(value.to_bits()), operand(e, scale));
        result = x87_store32(x87_add(term, operand(e, add)));
    } else if first {
        let add = setting(e, 0x011c_d974);
        let scale = setting(e, 0x011c_e018);
        let term = x87_mul(x87_load32(value.to_bits()), operand(e, scale));
        result = x87_store32(x87_add(term, operand(e, add)));
    }
    if e.call(ACTOR_TEST_0087F4E0, &args![actor]).bool() && first && !second {
        let object = e.call(ACTOR_GET_004181E0, &args![actor]).u32();
        let count = e.call(COUNT_006040D0, &args![object, 1u32]).u32();
        result = x87_store32(f64::from(count));
    } else if second {
        let holder = actor.wrapping_add(0xa4);
        if e.vcall(holder, 0x28, &args![]).u16() > 0 {
            let count = i32::from(e.vcall(holder, 0x28, &args![]).u16()) - 1;
            let weight = setting(e, 0x011c_dadc);
            let term = x87_mul(load(e, weight), f64::from(count));
            result = x87_store32(x87_add(term, x87_m32(result)));
        }
    }
    float_result(result)
}

/// `s[add] + clamped float actor value * s[scale]` for a derivation
/// callback, the actor value read first.
fn clamped_linear(e: &mut Engine, owner: u32, index: u32, scale: u32, add: u32) -> f32 {
    let value = e
        .call(GET_CLAMPED_ACTOR_FLOAT_VALUE, &args![owner, index])
        .f64();
    let scale = setting(e, scale);
    let term = x87_mul(load(e, scale), value);
    let add = setting(e, add);
    let result = x87_store32(x87_add(load(e, add), term));
    float_result(result)
}

// Translated from 006439d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Actor value derivation callback (`cdecl`: owner, actor value; the actor
/// value word is not read; result in ST0; no Xbox PDB name):
/// `s[11cd5a4] + clamped value 0xA * s[11cd7bc]`.
pub fn fn_006439d0(e: &mut Engine, owner: u32, _unused_2: u32) -> f32 {
    clamped_linear(e, owner, 0xa, 0x011c_d7bc, 0x011c_d5a4)
}

// Translated from 00643a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Actor value derivation callback (`cdecl`: owner, actor value; the actor
/// value word is not read; result in ST0; no Xbox PDB name):
/// `s[11cde08] + clamped value 5 * s[11cdd00]`.
pub fn fn_00643a10(e: &mut Engine, owner: u32, _unused_2: u32) -> f32 {
    clamped_linear(e, owner, 5, 0x011c_dd00, 0x011c_de08)
}

// Translated from 00643a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Actor value derivation callback (`cdecl`: owner, actor value; neither
/// word is read; result in ST0; no Xbox PDB name): the setting at
/// `011cda70`.
pub fn fn_00643a50(e: &mut Engine, _unused_1: u32, _unused_2: u32) -> f32 {
    let value = setting(e, 0x011c_da70);
    float_result(e.mem.u32(value))
}

// Translated from 00643a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Actor value derivation callback (`cdecl`: owner, actor value; the actor
/// value word is not read; result in ST0; no Xbox PDB name): by the owner's
/// clamped actor value 7, from 6 to 10 the setting at `011cd4d8`,
/// `011cd360`, `011cda10`, `011cd6e4` or `011cdd54`, else 0. (The exe's
/// jump table at `00643afc` holds the five cases in that order.)
pub fn fn_00643a70(e: &mut Engine, owner: u32, _unused_2: u32) -> f32 {
    let mut result = x87_store32(0.0);
    let value = e.call(GET_CLAMPED_ACTOR_VALUE, &args![owner, 7u32]).u32();
    let chosen = match value.wrapping_sub(6) {
        0 => Some(0x011c_d4d8),
        1 => Some(0x011c_d360),
        2 => Some(0x011c_da10),
        3 => Some(0x011c_d6e4),
        4 => Some(0x011c_dd54),
        _ => None,
    };
    if let Some(chosen) = chosen {
        let value = setting(e, chosen);
        result = x87_quiet_bits(e.mem.u32(value));
    }
    float_result(result)
}

// Translated from 00643b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Actor value derivation callback (`cdecl`: owner, actor value; the actor
/// value word is not read; result in ST0; no Xbox PDB name): clamped value
/// 0xB `* s[11cdb48] + s[11cdee0]` (the added setting is fetched first).
pub fn fn_00643b10(e: &mut Engine, owner: u32, _unused_2: u32) -> f32 {
    let add = setting(e, 0x011c_dee0);
    let value = e
        .call(GET_CLAMPED_ACTOR_FLOAT_VALUE, &args![owner, 0xbu32])
        .f64();
    let scale = setting(e, 0x011c_db48);
    let term = x87_mul(load(e, scale), value);
    let result = x87_store32(x87_add(term, operand(e, add)));
    float_result(result)
}

// Translated from 00643bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Actor value derivation callback (`cdecl`: owner, actor value; result in
/// ST0; no Xbox PDB name): `AiFormulas::AVDeriveSkill(owner, index, 0)` as a
/// `float`.
pub fn fn_00643bd0(e: &mut Engine, owner: u32, index: u32) -> f32 {
    let value = e.call(AV_DERIVE_SKILL, &args![owner, index, 0u32]).f64();
    float_result(x87_store32(value))
}

// Translated from 00643bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Row `index` of two tables (`cdecl`; no Xbox PDB name): the word at
/// `0119b144` into `*word` and the value of the setting at `0119b17c` into
/// `*value`.
pub fn fn_00643bf0(e: &mut Engine, index: i32, word: Ptr, value: Ptr) {
    let offset = (index as u32).wrapping_mul(4);
    let entry = e.mem.u32(WORD_TABLE_0119B144.wrapping_add(offset));
    e.mem.set_u32(word.addr(), entry);
    let entry = e.mem.u32(SETTING_TABLE_0119B17C.wrapping_add(offset));
    let address = setting(e, entry);
    let bits = x87_quiet_bits(e.mem.u32(address));
    e.mem.set_u32(value.addr(), bits);
}

// Translated from 00642bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A detection formula of the family of [`fn_00642ed0`] (`cdecl`, eleven words; no Xbox
/// PDB name; called twice by `RunDetection`): a base term from `tenths` (the span of the
/// settings `011cdd9c` over `011cdf04` times `tenths / 10`, plus `011cdf04`) weighted by
/// `1 + s[11cdfc0] * flag_b + s[11cd774] * flag_a + s[11cdbd4] * flag_c` (each flag 0 or 1;
/// the product is stored but not used further); the range ratio
/// `(range - distance) / range` (`range` the setting `011cd7d8`, times `011cdcbc` with
/// `long_range`) clamped by `00404010(ratio, 0)` and raised to the setting `011cd304`
/// (`pow`), scaled by `011cd2e8` unless `sign_word > 0`; a first term
/// `00404010(power' * s[11ce024] * (ftol(s[11cd080]) * product_factor), 0)`; a second term
/// `00404010(held * power * ((added + s[11cdb84]) * s[11cd08c]) * ((100 - percent) / 100),
/// 0)`. With the setting `011cd7e4` added to both, the sum answers 1 when strictly between
/// 0 and 1, else its `_ftol2` truncation (NaN included). Every step is x87 in the exe's
/// order, rounded at each store. Argument names describe their use only.
#[allow(clippy::too_many_arguments)]
pub fn fn_00642bc0(
    e: &mut Engine,
    tenths: i32,
    sign_word: i32,
    distance: f32,
    percent: i32,
    added: i32,
    flag_a: u8,
    flag_b: u8,
    flag_c: u8,
    product_factor: i32,
    held: i32,
    long_range: u8,
) -> i32 {
    let total_setting = setting(e, 0x011c_d7e4);
    let total = x87_store32(load(e, total_setting));
    let flag_b_word = i32::from(flag_b != 0);
    let flag_a_word = i32::from(flag_a != 0);
    let flag_c_word = i32::from(flag_c != 0);

    let low = setting(e, 0x011c_df04);
    let high = setting(e, 0x011c_dd9c);
    let low_again = setting(e, 0x011c_df04);
    let span = x87_sub(load(e, high), operand(e, low_again));
    let tenths_divisor: f64 = e.global(TENTHS_DIVISOR_01020758);
    let fraction = x87_div(f64::from(tenths), tenths_divisor);
    let base_term = x87_mul(span, fraction);
    let base_term = x87_add(base_term, operand(e, low));
    let base_bits = x87_store32(base_term);

    let flag_b_value = f64::from(flag_b_word);
    let weight = setting(e, 0x011c_dfc0);
    let sum = x87_mul(load(e, weight), flag_b_value);
    let sum = x87_add(sum, 1.0);
    let flag_a_value = f64::from(flag_a_word);
    let weight = setting(e, 0x011c_d774);
    let product = x87_mul(load(e, weight), flag_a_value);
    let sum = x87_add(product, sum);
    let flag_c_value = f64::from(flag_c_word);
    let weight = setting(e, 0x011c_dbd4);
    let product = x87_mul(load(e, weight), flag_c_value);
    let sum = x87_add(product, sum);
    let sum = x87_mul(sum, x87_m32(base_bits));
    let base_bits = x87_store32(sum);

    let positive = i32::from(sign_word > 0);
    let range = if long_range != 0 {
        let scale = setting(e, 0x011c_dcbc);
        let range = setting(e, 0x011c_d7d8);
        x87_mul(load(e, scale), operand(e, range))
    } else {
        let range = setting(e, 0x011c_d7d8);
        load(e, range)
    };
    let range_bits = x87_store32(range);
    let ratio = x87_sub(x87_load32(range_bits), x87_m32(distance.to_bits()));
    let ratio = x87_div(ratio, x87_m32(range_bits));
    let ratio_bits = x87_quiet_bits(x87_store32(ratio));
    let clamped = e
        .call(FLOAT_PAIR_00404010, &args![ratio_bits, x87_store32(0.0)])
        .f64();
    let clamped_bits = x87_store32(clamped);
    let exponent = setting(e, 0x011c_d304);
    let exponent_value = load(e, exponent);
    let power = e
        .call(POW, &args![x87_load32(clamped_bits), exponent_value])
        .f64();
    let power_bits = x87_store32(power);
    let mut scaled_power = x87_quiet_bits(power_bits);
    if positive == 0 {
        let factor = setting(e, 0x011c_d2e8);
        scaled_power = x87_store32(x87_mul(x87_load32(power_bits), operand(e, factor)));
    }

    let per = setting(e, 0x011c_d080);
    let product_term = ftol(e, load(e, per)).wrapping_mul(product_factor);
    let weight = setting(e, 0x011c_e024);
    let first = x87_mul(x87_load32(scaled_power), operand(e, weight));
    let first = x87_store32(x87_mul(first, f64::from(product_term)));
    let first = e
        .call(
            FLOAT_PAIR_00404010,
            &args![x87_quiet_bits(first), x87_store32(0.0)],
        )
        .f64();
    let first_bits = x87_store32(first);

    let add = setting(e, 0x011c_db84);
    let added_term = x87_add(f64::from(added), operand(e, add));
    let scale = setting(e, 0x011c_d08c);
    let added_bits = x87_store32(x87_mul(load(e, scale), added_term));
    let percent_word = 100i32.wrapping_sub(percent) / 100;
    let percent_bits = x87_store32(f64::from(percent_word));
    let second = x87_mul(f64::from(held), x87_m32(power_bits));
    let second = x87_mul(second, x87_m32(added_bits));
    let second = x87_mul(second, x87_m32(percent_bits));
    let second = x87_store32(second);
    let second = e
        .call(
            FLOAT_PAIR_00404010,
            &args![x87_quiet_bits(second), x87_store32(0.0)],
        )
        .f64();
    let second_bits = x87_store32(second);
    // The exe also stores the base term times the power into a local it never reads.
    let _base_times_power = x87_store32(x87_mul(x87_load32(base_bits), x87_m32(power_bits)));

    let sum = x87_add(x87_load32(total), x87_m32(first_bits));
    let sum = x87_add(sum, x87_m32(second_bits));
    let value = x87_load32(x87_store32(sum));
    let one: f64 = e.global(DOUBLE_ONE);
    let zero: f64 = e.global(DOUBLE_ZERO);
    // FCOMP with 1.0, TEST AH,5 / JP: below one goes on; FCOMP with 0.0, TEST AH,0x41 /
    // JNZ: above zero answers 1. Everything else (NaN included) is truncated.
    if value < one && value > zero {
        1
    } else {
        ftol(e, value)
    }
}

/// The settings [`fn_00643400`] weighs its three terms with, and its two limits.
const PICKPOCKET_FIRST: (u32, u32) = (0x011c_def8, 0x011c_d8e4);
const PICKPOCKET_SECOND: (u32, u32) = (0x011c_d48c, 0x011c_dec8);
const PICKPOCKET_THIRD: (u32, u32) = (0x011c_da88, 0x011c_d8a8);
const PICKPOCKET_MINIMUM: u32 = 0x011c_dffc;
const PICKPOCKET_MAXIMUM: u32 = 0x011c_da40;

// Translated from 00643400 (decompiled, FalloutNV.exe 1.4.0.525)
/// A chance formula (`cdecl`, three words; no Xbox PDB name; called by
/// `PickpocketCheck`): `first * s[11cd8e4] + s[11cdef8]` plus
/// `second * s[11cdec8] + s[11cd48c]` plus `s[11cd8a8] * third + s[11cda88]` (each stored
/// as a `float`, the integers loaded by `FILD`), truncated by `_ftol2`; raised to the
/// truncated setting `011cdffc` when that is above it, else lowered to the truncated
/// setting `011cda40` when that is below it (NaN keeps the sum). Argument names describe
/// their use only.
pub fn fn_00643400(e: &mut Engine, first: i32, second: i32, third: f32) -> i32 {
    let add = setting(e, PICKPOCKET_FIRST.0);
    let scale = setting(e, PICKPOCKET_FIRST.1);
    let term = x87_mul(f64::from(first), operand(e, scale));
    let first_bits = x87_store32(x87_add(term, operand(e, add)));
    let add = setting(e, PICKPOCKET_SECOND.0);
    let scale = setting(e, PICKPOCKET_SECOND.1);
    let term = x87_mul(f64::from(second), operand(e, scale));
    let second_bits = x87_store32(x87_add(term, operand(e, add)));
    let add = setting(e, PICKPOCKET_THIRD.0);
    let scale = setting(e, PICKPOCKET_THIRD.1);
    let term = x87_mul(load(e, scale), x87_m32(third.to_bits()));
    let third_bits = x87_store32(x87_add(term, operand(e, add)));
    let sum = x87_add(x87_load32(first_bits), x87_m32(second_bits));
    let sum = x87_add(sum, x87_m32(third_bits));
    let mut result = ftol(e, sum);
    let value = f64::from(result);
    let minimum = setting(e, PICKPOCKET_MINIMUM);
    // `FCOMP`, `TEST AH,0x41`, `JNZ`: the minimum only when it is above the sum.
    if load(e, minimum) > value {
        let minimum = setting(e, PICKPOCKET_MINIMUM);
        result = ftol(e, load(e, minimum));
    } else {
        let value = f64::from(result);
        let maximum = setting(e, PICKPOCKET_MAXIMUM);
        // `FCOMP`, `TEST AH,5`, `JP`: the maximum only when it is below the sum.
        if load(e, maximum) < value {
            let maximum = setting(e, PICKPOCKET_MAXIMUM);
            result = ftol(e, load(e, maximum));
        }
    }
    result
}

// Translated from 00643b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// A derived actor value (`cdecl`, two words; no Xbox PDB name; registered by
/// `RegisterActorValues`), in ST0: 0.0 unless `index` is 0x11; then the owner's clamped
/// actor value 5 (`0066ef50`) plus the setting `011cd89c`, times the setting `011cd12c`
/// (stored as a `float`), through `00406ce0`; a result above 0.0 is added to the 0.0 start
/// (NaN not above).
pub fn fn_00643b50(e: &mut Engine, owner: u32, index: i32) -> f32 {
    let mut result = x87_store32(0.0);
    if index == 0x11 {
        let value = e
            .call(GET_CLAMPED_ACTOR_FLOAT_VALUE, &args![owner, 5u32])
            .f64();
        let add = setting(e, 0x011c_d89c);
        let sum = x87_add(load(e, add), value);
        let scale = setting(e, 0x011c_d12c);
        let product = x87_store32(x87_mul(load(e, scale), sum));
        let rounded = e.call(0x0040_6ce0, &args![x87_quiet_bits(product)]).f64();
        let rounded = x87_store32(rounded);
        let zero: f64 = e.global(DOUBLE_ZERO);
        // `FCOMP`, `TEST AH,0x41`, `JNZ`: added only when above zero.
        if x87_load32(rounded) > zero {
            result = x87_store32(x87_add(x87_load32(result), x87_m32(rounded)));
        }
    }
    float_result(result)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0064_2bc0,
            fn_00642bc0(i32, i32, f32, i32, i32, u8, u8, u8, i32, i32, u8) -> i32
        ),
        entry!(0x0064_3400, fn_00643400(i32, i32, f32) -> i32),
        entry!(0x0064_3b50, fn_00643b50(u32, i32) -> f32),
        entry!(0x0064_2860, fn_00642860(Ptr)),
        entry!(0x0064_2890, fn_00642890(Ptr)),
        entry!(0x0064_28b0, fn_006428b0(Ptr) -> f32),
        entry!(0x0064_2940, fn_00642940(Ptr)),
        entry!(0x0064_29e0, fn_006429e0(f32, f32) -> f32),
        entry!(
            0x0064_2a60,
            fn_00642a60(i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, u8) -> i32
        ),
        entry!(0x0064_2e80, fn_00642e80(f32) -> i32),
        entry!(
            0x0064_2ed0,
            fn_00642ed0(
                i32, i32, i32, i32, f32, i32, i32, u8, i32, u8, u8, u8, u8, u8, i32, i32, u8, u8,
                i32, Ptr, Ptr, Ptr, i32, i32, i32
            ) -> i32
        ),
        entry!(0x0064_34f0, fn_006434f0(i32, u32) -> f32),
        entry!(0x0064_3540, fn_00643540(i32, Ptr, Ptr)),
        entry!(0x0064_35c0, fn_006435c0(i32, i32) -> f32),
        entry!(0x0064_3670, fn_00643670(u32, u32) -> f32),
        entry!(0x0064_36c0, fn_006436c0(u32, f32) -> f32),
        entry!(0x0064_3820, fn_00643820(u32, u32) -> f32),
        entry!(0x0064_3870, fn_00643870(u32, f32) -> f32),
        entry!(0x0064_39d0, fn_006439d0(u32, u32) -> f32),
        entry!(0x0064_3a10, fn_00643a10(u32, u32) -> f32),
        entry!(0x0064_3a50, fn_00643a50(u32, u32) -> f32),
        entry!(0x0064_3a70, fn_00643a70(u32, u32) -> f32),
        entry!(0x0064_3b10, fn_00643b10(u32, u32) -> f32),
        entry!(0x0064_3bd0, fn_00643bd0(u32, u32) -> f32),
        entry!(0x0064_3bf0, fn_00643bf0(i32, Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Doubles at fake addresses for vtable slots.
    const SLOT_A: u32 = 0x0001_0010;
    const SLOT_B: u32 = 0x0001_0020;
    const SLOT_C: u32 = 0x0001_0030;
    const SLOT_D: u32 = 0x0001_0040;

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
        e.map(0x0119_b000, 0x1000);
        e.map(0x0101_1000, 0x2000);
        e.map(0x0102_0000, 0x1000);
        e.map(0x0103_5000, 0x1000);
        e.set_global(HOURS_PER_DAY, 24.0f64);
        e.set_global(DOUBLE_ZERO, 0.0f64);
        e.set_global(DOUBLE_ONE, 1.0f64);
        e.register(SETTING_VALUE, |_, a| word(a[0] + 4));
        e.register(SETTING_INT_VALUE, |_, a| word(a[0] + 4));
        e.register(FTOL, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            word(v as i32 as u32)
        });
        e
    }

    fn set(e: &mut Engine, setting: u32, value: f32) {
        e.mem.set_f32(setting + 4, value);
    }

    fn arg_f32(a: &[u32], i: usize) -> f32 {
        f32::from_bits(a[i])
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An actor with a vtable whose +0x218, +0x360 and +0x20 slots are the
    /// doubles `SLOT_A`, `SLOT_B` and `SLOT_C`; the object at +0xA4 (the
    /// actor value owner) has a vtable whose +0x28 is `SLOT_D` and +0x20 is
    /// `SLOT_C`. Returns (actor, owner).
    fn actor(e: &mut Engine) -> (u32, u32) {
        let actor = e.mem.alloc(0x200);
        let mut slots = vec![0u32; 0x100];
        slots[0x218 / 4] = SLOT_A;
        slots[0x360 / 4] = SLOT_B;
        e.put_vtable(0x0002_0000, &slots);
        e.mem.set_u32(actor, 0x0002_0000);
        let mut owner_slots = vec![0u32; 0x10];
        owner_slots[0x20 / 4] = SLOT_C;
        owner_slots[0x28 / 4] = SLOT_D;
        e.put_vtable(0x0002_1000, &owner_slots);
        e.mem.set_u32(actor + 0xa4, 0x0002_1000);
        (actor, actor + 0xa4)
    }

    #[test]
    fn fn_00642860_sets_the_flag_and_steps_the_held_object() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.register(POINTER_GET_00559450, |_, _| word(0x1234));
        e.register(OBJECT_STEP_00B64D60, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0064_2860, &args![this]);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert_eq!(calls(&e, POINTER_GET_00559450), vec![vec![this + 0x28]]);
        assert_eq!(calls(&e, OBJECT_STEP_00B64D60), vec![vec![0x1234]]);

        e.register(POINTER_GET_00559450, |_, _| word(0));
        e.call_log = Some(vec![]);
        e.call(0x0064_2860, &args![this]);
        assert!(calls(&e, OBJECT_STEP_00B64D60).is_empty());
    }

    #[test]
    fn fn_00642890_clears_the_flag() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.mem.set_u8(this + 0x24, 1);
        e.call(0x0064_2890, &args![this]);
        assert_eq!(e.mem.u8(this + 0x24), 0);
    }

    #[test]
    fn fn_006428b0_reads_the_property_value_only_for_type_0xd() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        let property = e.mem.alloc(0x80);
        e.mem.set_f32(property + 0x6c, 2.5);
        e.register(POINTER_GET_00559450, |_, _| word(0x5000));
        e.register_double(NI_AV_OBJECT_GET_PROPERTY, move |_, a| {
            assert_eq!(a, &[0x5000, 3]);
            word(property)
        });
        e.register(PROPERTY_KIND_00441110, |_, _| word(0xd));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_28b0, &args![this]).f32(), 2.5);
        assert_eq!(calls(&e, NI_AV_OBJECT_GET_PROPERTY).len(), 3);
        assert_eq!(calls(&e, POINTER_GET_00559450)[0], vec![this + 0x14]);

        e.register(PROPERTY_KIND_00441110, |_, _| word(7));
        assert_eq!(e.call(0x0064_28b0, &args![this]).f32(), 0.0);

        e.register(NI_AV_OBJECT_GET_PROPERTY, |_, _| word(0));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_28b0, &args![this]).f32(), 0.0);
        assert!(calls(&e, PROPERTY_KIND_00441110).is_empty());
    }

    #[test]
    fn fn_00642940_updates_what_each_property_finds() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        e.register(POINTER_GET_00559450, |_, a| word(a[0] + 0x100));
        e.register(NI_AV_OBJECT_GET_PROPERTY, |_, a| word(a[0] + 1));
        // Only the +0x14 member's property finds something.
        e.register_double(LOOKUP_00653270, move |_, a| {
            word(if a[1] == this + 0x115 { 0x7000 } else { 0 })
        });
        e.register(STORE_006348E0, |_, _| Ret::default());
        e.register(SET_POINTER_00634900, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0064_2940, &args![this]);
        assert_eq!(
            calls(&e, LOOKUP_00653270),
            vec![
                vec![LOOKUP_KEY_012005B8, this + 0x111],
                vec![LOOKUP_KEY_012005B8, this + 0x115]
            ]
        );
        assert_eq!(calls(&e, STORE_006348E0), vec![vec![0x7000, 0]]);
        assert_eq!(
            calls(&e, SET_POINTER_00634900),
            vec![vec![0x7000, POINTER_ARGUMENT_011F49E0]]
        );
    }

    #[test]
    fn fn_006429e0_wraps_the_hour_across_midnight() {
        let mut e = engine();
        // The double returns the range's end.
        e.register(RANDOM_FLOAT, |_, a| st0(f64::from(arg_f32(a, 1))));
        e.call_log = Some(vec![]);
        let r = e.call(0x0064_29e0, &args![22.0f32, 2.0f32]).f32();
        assert_eq!(r, 2.0);
        assert_eq!(
            calls(&e, RANDOM_FLOAT),
            vec![vec![22.0f32.to_bits(), 26.0f32.to_bits()]]
        );
        // Without wrapping the value less 24 is negative: kept as is.
        let r = e.call(0x0064_29e0, &args![1.0f32, 5.0f32]).f32();
        assert_eq!(r, 5.0);
    }

    fn set_00642a60(e: &mut Engine) {
        set(e, 0x011c_ddec, 1.0);
        set(e, 0x011c_df38, 2.0);
        set(e, 0x011c_d014, 3.0);
        set(e, 0x011c_cfe4, 2.0);
        set(e, 0x011c_d228, 1.0);
        set(e, 0x011c_dc28, 3.0);
        set(e, 0x011c_dfa8, 2.0);
        set(e, 0x011c_ddc4, 0.5);
        set(e, 0x011c_d890, 1.0);
        set(e, 0x011c_d2d0, 3.0);
        set(e, 0x011c_d438, 4.0);
    }

    #[test]
    fn fn_00642a60_sums_the_weighted_terms() {
        let mut e = engine();
        set_00642a60(&mut e);
        let words = |skip: u32| {
            args![100i32, 5i32, 1i32, 2i32, 3i32, 4i32, 6i32, 2i32, 5i32, 1000i32, skip].to_vec()
        };
        // 100 + 5 + 16 + 6 + 8 + 6 + 20 + 1000.
        assert_eq!(e.call(0x0064_2a60, &words(0)).i32(), 1161);
        // Without the two skipped terms.
        assert_eq!(e.call(0x0064_2a60, &words(1)).i32(), 1139);
    }

    #[test]
    fn fn_00642e80_divides_only_above_the_setting() {
        let mut e = engine();
        set(&mut e, 0x011c_d240, 2.0);
        assert_eq!(e.call(0x0064_2e80, &args![7.0f32]).i32(), 3);
        assert_eq!(e.call(0x0064_2e80, &args![1.0f32]).i32(), 1);
        assert_eq!(e.call(0x0064_2e80, &args![f32::NAN]).i32(), 1);
    }

    fn set_00642ed0(e: &mut Engine) {
        e.set_global(DIVISOR_01011590, 2.0f64);
        e.set_global(TENTHS_DIVISOR_01020758, 10.0f64);
        e.mem.set_u32(0x011c_d8c0 + 4, 10);
        e.mem.set_u32(0x011c_d13c + 4, 1);
        e.mem.set_u32(0x011c_d4c0 + 4, 2);
        for (s, v) in [
            (0x011c_d7e4, 0.25f32),
            (0x011c_dbf8, 0.5),
            (0x011c_d1e0, 0.5),
            (0x011c_df04, 1.0),
            (0x011c_dd9c, 3.0),
            (0x011c_dfc0, 1.0),
            (0x011c_d774, 7.0),
            (0x011c_dbd4, 0.5),
            (0x011c_dcbc, 2.0),
            (0x011c_d7d8, 10.0),
            (0x011c_d304, 2.0),
            (0x011c_d2e8, 0.5),
            (0x011c_d258, 1.0),
            (0x011c_dc80, 0.5),
            (0x011c_d148, 2.0),
            (0x011c_d080, 1.0),
            (0x011c_e024, 4.0),
            (0x011c_dd18, 1.0),
            (0x011c_d2dc, 0.5),
            (0x011c_db84, 1.0),
            (0x011c_d08c, 0.5),
            (0x011c_deec, 0.5),
            (0x011c_d310, 3.0),
            (0x011c_da04, 2.0),
        ] {
            set(e, s, v);
        }
        e.register(INT_PAIR_00647B70, |_, a| word(a[0].wrapping_add(a[1])));
        e.register(FLOAT_PAIR_00404010, |_, a| {
            st0(f64::from(arg_f32(a, 0).max(arg_f32(a, 1))))
        });
        e.register(POW, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            let y = f64::from_bits(u64::from(a[2]) | u64::from(a[3]) << 32);
            st0(x.powf(y))
        });
    }

    fn words_00642ed0(sign_word: i32, zero_multiplier: u32, mode: i32, outs: [u32; 3]) -> Vec<u32> {
        args![
            5i32,
            5i32,
            sign_word,
            2i32,
            5.0f32,
            3i32,
            0i32,
            zero_multiplier,
            4i32,
            1u32,
            1u32,
            1u32,
            0u32,
            1u32,
            2i32,
            3i32,
            0u32,
            1u32,
            mode,
            outs[0],
            outs[1],
            outs[2],
            1i32,
            3i32,
            4i32
        ]
        .to_vec()
    }

    #[test]
    fn fn_00642ed0_computes_the_three_terms_and_their_sum() {
        let mut e = engine();
        set_00642ed0(&mut e);
        let outs = [e.mem.alloc(4), e.mem.alloc(4), e.mem.alloc(4)];
        e.call_log = Some(vec![]);
        let r = e.call(0x0064_2ed0, &words_00642ed0(1, 0, 1, outs)).i32();
        assert_eq!(e.mem.f32(outs[0]), 8.0);
        assert_eq!(e.mem.f32(outs[1]), 5.0);
        assert_eq!(e.mem.f32(outs[2]), -11.0);
        // 0.25 + 8 + 5 - 11.
        assert_eq!(r, 2);
        assert_eq!(calls(&e, INT_PAIR_00647B70), vec![vec![4, 0]]);
        assert_eq!(
            calls(&e, POW),
            vec![args![0.5f64, 2.0f64].to_vec()],
            "the clamped ratio to the setting's power"
        );

        // A sum strictly between 0 and 1 answers 1.
        set(&mut e, 0x011c_d7e4, -1.75);
        assert_eq!(e.call(0x0064_2ed0, &words_00642ed0(1, 0, 1, outs)).i32(), 1);

        // Mode 2 (equal to the double), a non-positive sign word and the
        // zero multiplier.
        set(&mut e, 0x011c_d7e4, 0.25);
        let r = e.call(0x0064_2ed0, &words_00642ed0(0, 1, 2, outs)).i32();
        assert_eq!(e.mem.f32(outs[0]), 4.0);
        assert_eq!(e.mem.f32(outs[1]), 0.0);
        assert_eq!(e.mem.f32(outs[2]), -11.0);
        assert_eq!(r, -6);
    }

    #[test]
    fn fn_006434f0_scales_the_level_against_the_top() {
        let mut e = engine();
        e.set_global(DIVISOR_01011590, 2.0f64);
        set(&mut e, 0x011c_d550, 1.0);
        set(&mut e, 0x011c_d7f0, 2.0);
        set(&mut e, 0x011c_db0c, 10.0);
        assert_eq!(e.call(0x0064_34f0, &args![3i32, 0u32]).f32(), 1.5);
    }

    #[test]
    fn fn_00643540_picks_an_hour_and_a_value_from_the_row() {
        let mut e = engine();
        for (i, (s, v)) in [
            (0x011c_e100u32, 22.0f32),
            (0x011c_e110, 2.0),
            (0x011c_e120, 1.0),
            (0x011c_e130, 3.0),
        ]
        .into_iter()
        .enumerate()
        {
            e.mem.set_u32(RANGE_TABLE_011CD680 + 0x10 + 4 * i as u32, s);
            set(&mut e, s, v);
        }
        e.register(RANDOM_FLOAT, |_, a| st0(f64::from(arg_f32(a, 1))));
        let hour = e.mem.alloc(4);
        let value = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x0064_3540, &args![1i32, hour, value]);
        assert_eq!(e.mem.f32(hour), 2.0);
        assert_eq!(e.mem.f32(value), 3.0);
        assert_eq!(
            calls(&e, RANDOM_FLOAT)[1],
            vec![1.0f32.to_bits(), 3.0f32.to_bits()]
        );
    }

    #[test]
    fn fn_006435c0_spreads_the_amount_around_its_value() {
        let mut e = engine();
        e.mem.set_u32(SETTING_TABLE_0119B194 + 4, 0x011c_e200);
        e.mem.set_u32(SETTING_TABLE_0119B1AC + 4, 0x011c_e210);
        set(&mut e, 0x011c_e200, 3.0);
        set(&mut e, 0x011c_e210, 2.0);
        set(&mut e, 0x011c_dc74, 2.0);
        set(&mut e, 0x011c_dc10, 0.5);
        set(&mut e, 0x011c_d1d4, 0.25);
        e.register(RANDOM_FLOAT, |_, a| {
            st0((f64::from(arg_f32(a, 0)) + f64::from(arg_f32(a, 1))) / 2.0)
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_35c0, &args![3i32, 1i32]).f32(), 24.0);
        assert_eq!(
            calls(&e, RANDOM_FLOAT),
            vec![vec![18.0f32.to_bits(), 30.0f32.to_bits()]]
        );
    }

    #[test]
    fn fn_00643670_derives_from_the_clamped_actor_value_7() {
        let mut e = engine();
        let (_, owner) = actor(&mut e);
        e.register(SLOT_C, |_, a| {
            assert_eq!(a[1], 7);
            st0(5.0)
        });
        e.register(CLAMP_ACTOR_VALUE, |_, a| {
            assert_eq!(a[0], 7);
            st0(f64::from(arg_f32(a, 1)) + 1.0)
        });
        e.register(SLOT_A, |_, _| word(1));
        e.register(SLOT_B, |_, _| word(0));
        e.register(ACTOR_TEST_0087F4E0, |_, _| word(0));
        set(&mut e, 0x011c_d27c, 2.0);
        set(&mut e, 0x011c_d544, 0.5);
        // (6 + 2) * 0.5.
        assert_eq!(e.call(0x0064_3670, &args![owner, 0u32]).f32(), 4.0);
    }

    #[test]
    fn fn_006436c0_takes_each_branch() {
        let mut e = engine();
        let (actor, owner) = actor(&mut e);
        set(&mut e, 0x011c_d120, 1.0);
        set(&mut e, 0x011c_d270, 2.0);
        set(&mut e, 0x011c_dea4, 0.5);
        e.register(SLOT_A, |_, _| word(0));
        e.register(SLOT_B, |_, _| word(1));
        e.register(SLOT_D, |_, _| word(0xffff_0003));
        e.register(ACTOR_TEST_0087F4E0, |_, _| word(0));
        e.call_log = Some(vec![]);
        // (1 + 1) * 2 + (3 - 1) * 0.5.
        assert_eq!(e.call(0x0064_36c0, &args![owner, 1.0f32]).f32(), 5.0);
        assert_eq!(calls(&e, SLOT_D), vec![vec![owner], vec![owner]]);
        assert_eq!(calls(&e, ACTOR_TEST_0087F4E0), vec![vec![actor]]);

        // Only the +0x218 answer, with 0087f4e0: the count.
        e.register(SLOT_A, |_, _| word(1));
        e.register(SLOT_B, |_, _| word(0));
        e.register(ACTOR_TEST_0087F4E0, |_, _| word(1));
        e.register(ACTOR_GET_004181E0, |_, _| word(0x9000));
        e.register(COUNT_00603FC0, |_, a| {
            assert_eq!(a, &[0x9000, 0]);
            word(7)
        });
        assert_eq!(e.call(0x0064_36c0, &args![owner, 1.0f32]).f32(), 7.0);

        // Neither answer: 0.
        e.register(SLOT_A, |_, _| word(0));
        assert_eq!(e.call(0x0064_36c0, &args![owner, 1.0f32]).f32(), 0.0);
    }

    #[test]
    fn fn_00643820_derives_through_the_companion() {
        let mut e = engine();
        let (_, owner) = actor(&mut e);
        e.register(SLOT_C, |_, _| st0(5.0));
        e.register(
            CLAMP_ACTOR_VALUE,
            |_, a| st0(f64::from(arg_f32(a, 1)) + 1.0),
        );
        e.register(SLOT_A, |_, _| word(0));
        e.register(SLOT_B, |_, _| word(1));
        e.register(SLOT_D, |_, _| word(0));
        e.register(ACTOR_TEST_0087F4E0, |_, _| word(0));
        set(&mut e, 0x011c_d638, 1.0);
        set(&mut e, 0x011c_d7b0, 0.5);
        // 6 * 0.5 + 1.
        assert_eq!(e.call(0x0064_3820, &args![owner, 0u32]).f32(), 4.0);
    }

    #[test]
    fn fn_00643870_takes_each_branch() {
        let mut e = engine();
        let (_, owner) = actor(&mut e);
        set(&mut e, 0x011c_d974, 1.0);
        set(&mut e, 0x011c_e018, 3.0);
        e.register(SLOT_A, |_, _| word(1));
        e.register(SLOT_B, |_, _| word(0));
        e.register(ACTOR_TEST_0087F4E0, |_, _| word(0));
        assert_eq!(e.call(0x0064_3870, &args![owner, 2.0f32]).f32(), 7.0);

        e.register(ACTOR_TEST_0087F4E0, |_, _| word(1));
        e.register(ACTOR_GET_004181E0, |_, _| word(0x9000));
        e.register(COUNT_006040D0, |_, a| {
            assert_eq!(a, &[0x9000, 1]);
            word(9)
        });
        assert_eq!(e.call(0x0064_3870, &args![owner, 2.0f32]).f32(), 9.0);

        e.register(SLOT_A, |_, _| word(0));
        e.register(SLOT_B, |_, _| word(1));
        e.register(SLOT_D, |_, _| word(2));
        set(&mut e, 0x011c_d638, 1.0);
        set(&mut e, 0x011c_d7b0, 0.5);
        set(&mut e, 0x011c_dadc, 4.0);
        // 2 * 0.5 + 1, then (2 - 1) * 4 added.
        assert_eq!(e.call(0x0064_3870, &args![owner, 2.0f32]).f32(), 6.0);
    }

    #[test]
    fn fn_006439d0_scales_actor_value_0xa() {
        let mut e = engine();
        e.register(GET_CLAMPED_ACTOR_FLOAT_VALUE, |_, a| {
            assert_eq!(a, &[0x4000, 0xa]);
            st0(4.0)
        });
        set(&mut e, 0x011c_d7bc, 0.5);
        set(&mut e, 0x011c_d5a4, 1.0);
        assert_eq!(e.call(0x0064_39d0, &args![0x4000u32, 9u32]).f32(), 3.0);
    }

    #[test]
    fn fn_00643a10_scales_actor_value_5() {
        let mut e = engine();
        e.register(GET_CLAMPED_ACTOR_FLOAT_VALUE, |_, a| {
            assert_eq!(a, &[0x4000, 5]);
            st0(4.0)
        });
        set(&mut e, 0x011c_dd00, 2.0);
        set(&mut e, 0x011c_de08, 0.5);
        assert_eq!(e.call(0x0064_3a10, &args![0x4000u32, 9u32]).f32(), 8.5);
    }

    #[test]
    fn fn_00643a50_returns_its_setting() {
        let mut e = engine();
        set(&mut e, 0x011c_da70, 1.5);
        assert_eq!(e.call(0x0064_3a50, &args![0u32, 0u32]).f32(), 1.5);
    }

    #[test]
    fn fn_00643a70_picks_the_setting_by_actor_value_7() {
        let mut e = engine();
        let settings = [
            0x011c_d4d8u32,
            0x011c_d360,
            0x011c_da10,
            0x011c_d6e4,
            0x011c_dd54,
        ];
        for (i, s) in settings.iter().enumerate() {
            set(&mut e, *s, 10.0 + i as f32);
        }
        for value in [5u32, 6, 7, 8, 9, 10, 11] {
            e.register_double(GET_CLAMPED_ACTOR_VALUE, move |_, a| {
                assert_eq!(a, &[0x4000, 7]);
                word(value)
            });
            let r = e.call(0x0064_3a70, &args![0x4000u32, 0u32]).f32();
            let expected = if (6..=10).contains(&value) {
                10.0 + (value - 6) as f32
            } else {
                0.0
            };
            assert_eq!(r, expected, "actor value {value}");
        }
    }

    #[test]
    fn fn_00643b10_scales_actor_value_0xb() {
        let mut e = engine();
        e.register(GET_CLAMPED_ACTOR_FLOAT_VALUE, |_, a| {
            assert_eq!(a, &[0x4000, 0xb]);
            st0(4.0)
        });
        set(&mut e, 0x011c_dee0, 1.0);
        set(&mut e, 0x011c_db48, 0.25);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_3b10, &args![0x4000u32, 0u32]).f32(), 2.0);
        assert_eq!(calls(&e, SETTING_VALUE)[0], vec![0x011c_dee0]);
    }

    #[test]
    fn fn_00643bd0_derives_the_skill() {
        let mut e = engine();
        e.register(AV_DERIVE_SKILL, |_, a| {
            assert_eq!(a, &[0x4000, 0x20, 0]);
            st0(2.5)
        });
        assert_eq!(e.call(0x0064_3bd0, &args![0x4000u32, 0x20u32]).f32(), 2.5);
    }

    #[test]
    fn fn_00643bf0_reads_both_tables() {
        let mut e = engine();
        e.mem.set_u32(WORD_TABLE_0119B144 + 8, 0x77);
        e.mem.set_u32(SETTING_TABLE_0119B17C + 8, 0x011c_e300);
        set(&mut e, 0x011c_e300, 6.5);
        let w = e.mem.alloc(4);
        let v = e.mem.alloc(4);
        e.call(0x0064_3bf0, &args![2i32, w, v]);
        assert_eq!(e.mem.u32(w), 0x77);
        assert_eq!(e.mem.f32(v), 6.5);
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// Settings in a mapped block (value at +4, as `00403e20` answers); every setting
    /// starts at 0.0.
    fn engine(settings: &[(u32, f32)]) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_c000, 0x3000);
        e.map(0x0101_0000, 0x2_0000);
        for (setting, value) in settings {
            e.mem.set_f32(setting + 4, *value);
        }
        e.set_global::<f64>(DOUBLE_ONE, 1.0);
        e.set_global::<f64>(DOUBLE_ZERO, 0.0);
        e.set_global::<f64>(TENTHS_DIVISOR_01020758, 10.0);
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(FTOL, |_, a| {
            let v = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (v as i32).into_ret()
        });
        e.register(FLOAT_PAIR_00404010, |_, a| {
            st0(f64::from(f32::from_bits(a[0])))
        });
        e.register(POW, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            let y = f64::from_bits(u64::from(a[2]) | u64::from(a[3]) << 32);
            st0(x.powf(y))
        });
        e.call_log = Some(vec![]);
        e
    }

    fn ftol_calls(e: &mut Engine) -> usize {
        let log: Log = e.call_log.take().unwrap();
        log.iter().filter(|(a, _)| *a == FTOL).count()
    }

    fn detection(total: f32, held: i32) -> (i32, usize) {
        let mut e = engine(&[
            (0x011c_d7e4, total),
            (0x011c_d7d8, 10.0),
            (0x011c_d304, 1.0),
            (0x011c_d080, 2.0),
            (0x011c_e024, 0.25),
            (0x011c_db84, 1.0),
            (0x011c_d08c, 0.5),
        ]);
        let words =
            args![0u32, 1u32, 5.0f32, 0u32, 1u32, 0u32, 0u32, 0u32, 3u32, held, 0u32].to_vec();
        let result = e.call(0x0064_2bc0, &words).i32();
        (result, ftol_calls(&mut e))
    }

    #[test]
    fn fn_00642bc0_sums_the_terms() {
        // 0.5 + 0.75 + 1.0 truncated; the first ftol is the d080 setting.
        assert_eq!(detection(0.5, 2), (2, 2));
        // -0.5 + 0.75 + 0.5 lies in (0, 1): 1 without a second ftol.
        assert_eq!(detection(-0.5, 1), (1, 1));
    }

    #[test]
    fn fn_00643400_clamps_the_chance() {
        let settings = [
            (0x011c_d8e4u32, 2.0f32),
            (0x011c_def8, 1.0),
            (0x011c_dec8, 3.0),
            (0x011c_d8a8, 0.5),
            (0x011c_dffc, 0.0),
            (0x011c_da40, 100.0),
        ];
        let mut e = engine(&settings);
        // 2*2+1 + 3*3 + 0.5*4 = 16.
        assert_eq!(e.call(0x0064_3400, &args![2u32, 3u32, 4.0f32]).i32(), 16);
        let mut e = engine(&[(0x011c_dffc, 20.0), (0x011c_da40, 100.0)]);
        assert_eq!(e.call(0x0064_3400, &args![2u32, 3u32, 4.0f32]).i32(), 20);
        let mut e = engine(&[(0x011c_def8, 50.0), (0x011c_dffc, 0.0), (0x011c_da40, 10.0)]);
        assert_eq!(e.call(0x0064_3400, &args![0u32, 0u32, 0.0f32]).i32(), 10);
    }

    #[test]
    fn fn_00643b50_derives_value_0x11() {
        let mut e = engine(&[(0x011c_d89c, 1.0), (0x011c_d12c, 2.0)]);
        e.register(GET_CLAMPED_ACTOR_FLOAT_VALUE, |_, a| {
            assert_eq!(a[1], 5);
            st0(3.0)
        });
        e.register(0x0040_6ce0, |_, a| st0(f64::from(f32::from_bits(a[0]))));
        assert_eq!(e.call(0x0064_3b50, &args![0x1000u32, 0x11u32]).f64(), 8.0);
        assert_eq!(e.call(0x0064_3b50, &args![0x1000u32, 0x10u32]).f64(), 0.0);
        e.register(0x0040_6ce0, |_, _| st0(-1.0));
        assert_eq!(e.call(0x0064_3b50, &args![0x1000u32, 0x11u32]).f64(), 0.0);
    }
}
