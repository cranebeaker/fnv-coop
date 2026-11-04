//! `fallout shared/gameplayformulas.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0043d4d0` (`thiscall` on an integer `Setting`, no stack words): answers the address of
/// the setting's value. `00403e20` (`thiscall` on a float `Setting`): the same for a float.
const INT_SETTING_VALUE: u32 = 0x0043_d4d0;
const FLOAT_SETTING_VALUE: u32 = 0x0040_3e20;
/// The settings, by the names their static initializers give them.
const I_XP_BASE: u32 = 0x011d_03e8;
const I_XP_BUMP_BASE: u32 = 0x011d_0ae4;
const F_ACTOR_SWIM_BREATH_BASE: u32 = 0x011d_0868;
const F_ACTOR_SWIM_BREATH_MULT: u32 = 0x011d_0e28;
const F_LOCK_SKILL_BASE: u32 = 0x011d_0550;
/// The two doubles `fn_00648c10` scales and offsets by (0.5 and 10.0 in the exe).
const SCALE_DOUBLE: u32 = 0x0101_1588;
const OFFSET_DOUBLE: u32 = 0x0102_0758;
/// `00406cc0` (`cdecl`, one float, answers `ST0`; no engine-map name).
const FLOAT_FUNCTION: u32 = 0x0040_6cc0;
/// `_ftol2` (`00ec62c0`, the value in `ST0`).
const FTOL2: u32 = 0x00ec_62c0;
/// `004a8f20` and `00647b70` (`cdecl`, two words each; no engine-map names).
const COMBINE_FIRST: u32 = 0x004a_8f20;
const COMBINE_SECOND: u32 = 0x0064_7b70;

// Translated from 00648b50 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `GamePlayFormulas::GetRequiredExperiencePoints` (Xbox PDB; `cdecl`, the level): 0 for a
/// level of 1 or less; otherwise `iXPBase` plus, for each further level below `level`, a
/// step that starts at `iXPBase` and grows by `iXPBumpBase` each time (wrapping 32-bit
/// arithmetic, as the exe's `ADD`).
pub fn game_play_formulas_get_required_experience_points(e: &mut Engine, level: i32) -> i32 {
    if level <= 1 {
        return 0;
    }
    let base = e.call(INT_SETTING_VALUE, &args![I_XP_BASE]).u32();
    let mut total = e.mem.i32(base);
    let mut step = total;
    let bump = e.call(INT_SETTING_VALUE, &args![I_XP_BUMP_BASE]).u32();
    let bump = e.mem.i32(bump);
    let mut i = 1i32;
    while i < level.wrapping_sub(1) {
        step = step.wrapping_add(bump);
        total = total.wrapping_add(step);
        i += 1;
    }
    total
}

// Translated from 00648c10 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words): `_ftol2(00406cc0(first * 0.5 + 10.0))`
/// (the doubles at `01011588` and `01020758`, the sum rounded to a float), plus one when
/// `second` is even and `first` odd and positive (the C remainders `% 2`).
pub fn fn_00648c10(e: &mut Engine, first: i32, second: i32) -> i32 {
    let scaled = x87_mul(f64::from(first), e.mem.f64(SCALE_DOUBLE));
    let sum = x87_store32(x87_add(scaled, e.mem.f64(OFFSET_DOUBLE)));
    let value = e.call(FLOAT_FUNCTION, &args![x87_quiet_bits(sum)]).f64();
    let mut result = e.call(FTOL2, &args![value]).i32();
    if second % 2 == 0 && first % 2 == 1 {
        result = result.wrapping_add(1);
    }
    result
}

// Translated from 00648a10 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `GamePlayFormulas::SwimBreathTime` (Xbox PDB; `cdecl`, one word; answers `ST0`):
/// `fActorSwimBreathMult * value + fActorSwimBreathBase`, rounded to a float.
pub fn game_play_formulas_swim_breath_time(e: &mut Engine, value: i32) -> f32 {
    let base = e
        .call(FLOAT_SETTING_VALUE, &args![F_ACTOR_SWIM_BREATH_BASE])
        .u32();
    let mult = e
        .call(FLOAT_SETTING_VALUE, &args![F_ACTOR_SWIM_BREATH_MULT])
        .u32();
    let product = x87_mul(x87_load32(e.mem.u32(mult)), f64::from(value));
    let sum = x87_add(product, x87_m32(e.mem.u32(base)));
    f32::from_bits(x87_quiet_bits(x87_store32(sum)))
}

// Translated from 00648d30 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// `GamePlayFormulas::GetForceLockChance` (Xbox PDB; `cdecl`, two words):
/// `00647b70(004a8f20(first - second + _ftol2(<the float setting at 011d0550>), 100), 0)`
/// (the `PUSH 0` and `PUSH 0x64` at the start are those calls' second words).
pub fn game_play_formulas_get_force_lock_chance(e: &mut Engine, first: i32, second: i32) -> u32 {
    let difference = first.wrapping_sub(second);
    let setting = e.call(FLOAT_SETTING_VALUE, &args![F_LOCK_SKILL_BASE]).u32();
    let base = e.call(FTOL2, &args![x87_load32(e.mem.u32(setting))]).i32();
    let value = difference.wrapping_add(base);
    let limited = e.call(COMBINE_FIRST, &args![value, 0x64u32]).u32();
    e.call(COMBINE_SECOND, &args![limited, 0u32]).u32()
}

/// `00404010` and `0040ebd0` (`cdecl`, two floats each, answer `ST0`; no engine-map names).
const FLOAT_PAIR_00404010: u32 = 0x0040_4010;
const FLOAT_PAIR_0040EBD0: u32 = 0x0040_ebd0;

// Translated from 00648e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three floats; answers `ST0`): with
/// `ratio = (second - third) / second` (rounded to a float), answers
/// `first * 0040ebd0(1.0, 00404010(0.0, ratio))`, each call's result rounded to a float
/// and the product rounded to a float.
pub fn fn_00648e50(e: &mut Engine, first: f32, second: f32, third: f32) -> f32 {
    let difference = x87_sub(x87_load32(second.to_bits()), x87_m32(third.to_bits()));
    let ratio = x87_store32(x87_div(difference, x87_m32(second.to_bits())));
    let inner = e
        .call(
            FLOAT_PAIR_00404010,
            &args![0.0f32, f32::from_bits(x87_quiet_bits(ratio))],
        )
        .st0;
    let inner = x87_store32(inner);
    let outer = e
        .call(FLOAT_PAIR_0040EBD0, &args![1.0f32, f32::from_bits(inner)])
        .st0;
    let outer = x87_store32(outer);
    let product = x87_store32(x87_mul(x87_load32(first.to_bits()), x87_m32(outer)));
    f32::from_bits(x87_quiet_bits(product))
}

/// The exe's `double` constants these formulas read: 100.0 (`01017a40`),
/// 10.0 (`01020758`), 0.5 (`01011588`), 1.0 (`01012070`), 0.1
/// (`0101ffa0`), 0.2 (`01018a90`), 0.7 (`0104ee98`), 3600.0 (`01012640`)
/// and 60.0 (`01012638`), read from memory as the code does.
const HUNDRED: u32 = 0x0101_7a40;
const TEN: u32 = OFFSET_DOUBLE;
const HALF: u32 = SCALE_DOUBLE;
const ONE: u32 = 0x0101_2070;
const TENTH: u32 = 0x0101_ffa0;
const FIFTH: u32 = 0x0101_8a90;
const SEVEN_TENTHS: u32 = 0x0104_ee98;
const HOUR_SECONDS: u32 = 0x0101_2640;
const SIXTY: u32 = 0x0101_2638;
/// `004bd510` (cdecl, two floats, the answer in `ST0`), `004dd130` (the
/// same) and `00476b90` (`RET 8`, two floats, `ST0`) on the object
/// `00476c00` answers.
const FLOAT_PAIR_004BD510: u32 = 0x004b_d510;
const FLOAT_PAIR_004DD130: u32 = 0x004d_d130;
const FLOAT_PAIR_00476B90: u32 = 0x0047_6b90;
const FLOAT_PAIR_OWNER: u32 = 0x0047_6c00;

/// The bits of a float setting's value (`00403e20` answers its address).
fn setting_bits(e: &mut Engine, setting: u32) -> u32 {
    let at = e.call(FLOAT_SETTING_VALUE, &args![setting]).u32();
    e.mem.u32(at)
}

/// `*out = factor * setting * *out + *out` as `FLD factor; FMUL [setting];
/// FMUL [out]; FADD [out]; FSTP [out]`.
fn grow_factor_first(e: &mut Engine, out: u32, factor: u32, setting: u32) {
    let value = setting_bits(e, setting);
    let product = x87_mul(x87_load32(factor), x87_m32(value));
    let product = x87_mul(product, x87_m32(e.mem.u32(out)));
    let sum = x87_add(product, x87_m32(e.mem.u32(out)));
    e.mem.set_u32(out, x87_store32(sum));
}

/// `*out = setting * (*out * factor) + *out` as `FLD [out]; FMUL factor;
/// FSTP double; FLD [setting]; FMUL double; FADD [out]; FSTP [out]`.
fn grow_factor_last(e: &mut Engine, out: u32, factor: u32, setting: u32) {
    let product = x87_mul(x87_load32(e.mem.u32(out)), x87_m32(factor));
    let value = setting_bits(e, setting);
    let product = x87_mul(x87_load32(value), product);
    let sum = x87_add(product, x87_m32(e.mem.u32(out)));
    e.mem.set_u32(out, x87_store32(sum));
}

/// `setting * (*from * factor)` as `FLD [from]; FMUL factor; FSTP double;
/// FLD [setting]; FMUL double`, unrounded.
fn scaled_term(e: &mut Engine, from: u32, factor: u32, setting: u32) -> f64 {
    let product = x87_mul(x87_load32(e.mem.u32(from)), x87_m32(factor));
    let value = setting_bits(e, setting);
    x87_mul(x87_load32(value), product)
}

/// `factor * setting * *from` as `FLD factor; FMUL [setting]; FMUL
/// [from]`, unrounded.
fn factor_term(e: &mut Engine, from: u32, factor: u32, setting: u32) -> f64 {
    let value = setting_bits(e, setting);
    let product = x87_mul(x87_load32(factor), x87_m32(value));
    x87_mul(product, x87_m32(e.mem.u32(from)))
}

/// `*out = second + first + *out` (`FADD double; FADD [out]; FSTP [out]`).
fn add_terms(e: &mut Engine, out: u32, first: f64, second: f64) {
    let sum = x87_add(second, first);
    let sum = x87_add(sum, x87_m32(e.mem.u32(out)));
    e.mem.set_u32(out, x87_store32(sum));
}

// Translated from 00648090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GamePlayFormulas::GetRepairedCondition` (Xbox PDB; cdecl: the skill,
/// two conditions, an optional out word; the answer in `ST0`). With both
/// conditions divided by 10: the condition is the larger (`00404010`) plus
/// the setting at `011d0880`, plus the skill's share of the difference of
/// the settings at `011cffa0` and `011d0880` (over 100), plus the smaller
/// (`0040ebd0`) times the setting at `011d0748`. With an out word, when the
/// condition's percentage is no more than the larger's (both through
/// `004bd510` with 1.0 and `_ftol2`), the out word gets
/// `_ftol2((larger + 0.1 - s0ff0) * 100 / (s018c - s0ff0))` (the settings
/// at `011d0ff0` and `011d018c`). The answer is the condition times 10. A
/// fourth pair of setting reads computes a value the function never uses.
pub fn game_play_formulas_get_repaired_condition(
    e: &mut Engine,
    skill: i32,
    first: f32,
    second: f32,
    out: Ptr,
) -> f32 {
    let ten = e.global::<f64>(TEN);
    let hundred = e.global::<f64>(HUNDRED);
    let first = x87_store32(x87_div(x87_load32(first.to_bits()), ten));
    let second = x87_store32(x87_div(x87_load32(second.to_bits()), ten));
    let high = setting_bits(e, 0x011c_ffa0);
    let low = setting_bits(e, 0x011d_0880);
    let difference = x87_sub(x87_load32(high), x87_m32(low));
    let skill_part = x87_store32(x87_div(x87_mul(difference, f64::from(skill)), hundred));
    let mult = setting_bits(e, 0x011d_0748);
    let pair = args![x87_quiet_bits(first), x87_quiet_bits(second)];
    let lesser = e.call(FLOAT_PAIR_0040EBD0, &pair).f64();
    let lesser_part = x87_store32(x87_mul(lesser, x87_m32(mult)));
    let unused_base = setting_bits(e, 0x011d_0ff0);
    let unused_high = setting_bits(e, 0x011d_018c);
    let unused_low = setting_bits(e, 0x011d_0ff0);
    let unused = x87_sub(x87_load32(unused_high), x87_m32(unused_low));
    let unused = x87_div(x87_mul(unused, f64::from(skill)), hundred);
    // FSTP to a local nothing reads.
    let _ = x87_store32(x87_add(unused, x87_m32(unused_base)));
    let greater = x87_store32(e.call(FLOAT_PAIR_00404010, &pair).f64());
    let base = setting_bits(e, 0x011d_0880);
    let condition = x87_add(x87_load32(greater), x87_m32(base));
    let condition = x87_add(condition, x87_m32(skill_part));
    let condition = x87_store32(x87_add(condition, x87_m32(lesser_part)));
    if !out.is_null() {
        let one = 1.0f32.to_bits();
        let scaled = x87_store32(x87_mul(x87_load32(condition), hundred));
        let value = e
            .call(FLOAT_PAIR_004BD510, &args![x87_quiet_bits(scaled), one])
            .f64();
        let condition_percent = e.call(FTOL2, &args![value]).i32();
        let scaled = x87_store32(x87_mul(x87_load32(greater), hundred));
        let value = e
            .call(FLOAT_PAIR_004BD510, &args![x87_quiet_bits(scaled), one])
            .f64();
        let greater_percent = e.call(FTOL2, &args![value]).i32();
        if condition_percent <= greater_percent {
            let raised = x87_add(x87_load32(greater), e.global::<f64>(TENTH));
            let low = setting_bits(e, 0x011d_0ff0);
            // FSUBR double: the raised value minus the setting.
            let numerator = x87_mul(x87_sub(raised, x87_load32(low)), hundred);
            let high = setting_bits(e, 0x011d_018c);
            let low = setting_bits(e, 0x011d_0ff0);
            let denominator = x87_sub(x87_load32(high), x87_m32(low));
            // FDIVR double: the numerator over the difference.
            let ratio = x87_div(numerator, denominator);
            let word = e.call(FTOL2, &args![ratio]).u32();
            e.mem.set_u32(out.addr(), word);
        }
    }
    let answer = x87_store32(x87_mul(x87_load32(condition), ten));
    f32::from_bits(x87_quiet_bits(answer))
}

// Translated from 00648230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GamePlayFormulas::GetRepairCost` (Xbox PDB; cdecl: the skill, a
/// condition and a value): the setting at `011d1158` times the value times
/// `(s0ff0 + skill * (s018c - s0ff0) / 100 - condition / 10) / 10` (the
/// settings at `011d0ff0` and `011d018c`); above 0.5 the answer is
/// `00647b70(_ftol2(004bd510(cost, 1.0)), 1)`, otherwise 0.
pub fn game_play_formulas_get_repair_cost(
    e: &mut Engine,
    skill: i32,
    condition: f32,
    value: f32,
) -> i32 {
    let ten = e.global::<f64>(TEN);
    let hundred = e.global::<f64>(HUNDRED);
    let base = setting_bits(e, 0x011d_0ff0);
    let high = setting_bits(e, 0x011d_018c);
    let low = setting_bits(e, 0x011d_0ff0);
    let skill_part = x87_sub(x87_load32(high), x87_m32(low));
    let skill_part = x87_div(x87_mul(skill_part, f64::from(skill)), hundred);
    let skill_part = x87_store32(x87_add(skill_part, x87_m32(base)));
    let condition = x87_store32(x87_div(x87_load32(condition.to_bits()), ten));
    let mult = setting_bits(e, 0x011d_1158);
    let scale = x87_mul(x87_load32(mult), x87_m32(value.to_bits()));
    let share = x87_sub(x87_load32(skill_part), x87_m32(condition));
    let share = x87_div(share, ten);
    // FMULP (DE C9).
    let cost = x87_store32(x87_mul(scale, share));
    // FCOMP; FNSTSW; TEST AH,0x41; JNZ: only above 0.5.
    if x87_load32(cost) > e.global::<f64>(HALF) {
        let rounded = e
            .call(
                FLOAT_PAIR_004BD510,
                &args![x87_quiet_bits(cost), 1.0f32.to_bits()],
            )
            .f64();
        let whole = e.call(FTOL2, &args![rounded]).u32();
        return e.call(COMBINE_SECOND, &args![whole, 1u32]).i32();
    }
    0
}

// Translated from 006482f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A two-value formula (cdecl, ten words: two out floats, two floats, a
/// mode, a flag, three factors and a second flag; the engine map has no
/// name). The first out is `004dd130(fourth * s00e4 / third,
/// 1 / (s198c + 1))` and the second the first over `s00e4` and over
/// `s1998`. Then by mode (the exe's jump table): 1 grows both by the first
/// factor (settings `011d06b8`, `011d0568`) when neither flag is set, then
/// as 2; 2 grows both by the second factor (`011d02b0`, `011d009c`) when
/// only the first flag is set; 3 grows both by the third factor
/// (`011d022c`); 4 with neither flag adds the first-factor term and a
/// third-factor term (the first out's from the second out's value, as the
/// exe has it), otherwise grows both by the third factor; 5 with only the
/// first flag adds second- and third-factor terms, otherwise grows both by
/// the third factor, then as 3.
#[allow(clippy::too_many_arguments)]
pub fn fn_006482f0(
    e: &mut Engine,
    first_out: Ptr,
    second_out: Ptr,
    third: f32,
    fourth: f32,
    mode: u32,
    flag: bool,
    first_factor: f32,
    second_factor: f32,
    third_factor: f32,
    other_flag: bool,
) {
    let one = e.global::<f64>(ONE);
    let (a, b) = (first_out.addr(), second_out.addr());
    let (f1, f2, f3) = (
        first_factor.to_bits(),
        second_factor.to_bits(),
        third_factor.to_bits(),
    );
    let base = setting_bits(e, 0x011d_198c);
    // FLD1; FDIVRP (DE F1): one over the sum.
    let inverse = x87_store32(x87_div(1.0, x87_add(x87_load32(base), one)));
    let scale = setting_bits(e, 0x011d_00e4);
    let value = x87_mul(x87_load32(fourth.to_bits()), x87_m32(scale));
    let value = x87_store32(x87_div(value, x87_m32(third.to_bits())));
    let result = e
        .call(
            FLOAT_PAIR_004DD130,
            &args![x87_quiet_bits(value), x87_quiet_bits(inverse)],
        )
        .f64();
    e.mem.set_u32(a, x87_store32(result));
    let scale = setting_bits(e, 0x011d_00e4);
    let ratio = x87_div(x87_load32(e.mem.u32(a)), x87_m32(scale));
    let divisor = setting_bits(e, 0x011d_1998);
    // FDIVR double: the ratio over the setting.
    e.mem
        .set_u32(b, x87_store32(x87_div(ratio, x87_load32(divisor))));
    let mut third_case = false;
    match mode.wrapping_sub(1) {
        0 | 1 => {
            if mode == 1 && !flag && !other_flag {
                grow_factor_first(e, a, f1, 0x011d_06b8);
                grow_factor_first(e, b, f1, 0x011d_0568);
            }
            if flag && !other_flag {
                grow_factor_first(e, a, f2, 0x011d_02b0);
                grow_factor_first(e, b, f2, 0x011d_009c);
            }
        }
        2 => third_case = true,
        3 => {
            if !flag && !other_flag {
                let first = scaled_term(e, a, f1, 0x011d_06b8);
                let second = scaled_term(e, b, f3, 0x011d_022c);
                add_terms(e, a, first, second);
                let first = scaled_term(e, b, f1, 0x011d_0568);
                let second = scaled_term(e, b, f3, 0x011d_022c);
                add_terms(e, b, first, second);
            } else {
                grow_factor_first(e, a, f3, 0x011d_022c);
                grow_factor_first(e, b, f3, 0x011d_022c);
            }
        }
        4 => {
            if flag && !other_flag {
                let first = factor_term(e, a, f2, 0x011d_02b0);
                let second = factor_term(e, a, f3, 0x011d_022c);
                add_terms(e, a, first, second);
                let first = factor_term(e, b, f2, 0x011d_009c);
                let second = factor_term(e, b, f3, 0x011d_022c);
                add_terms(e, b, first, second);
            } else {
                grow_factor_first(e, a, f3, 0x011d_022c);
                grow_factor_first(e, b, f3, 0x011d_022c);
            }
            third_case = true;
        }
        _ => {}
    }
    if third_case {
        grow_factor_first(e, a, f3, 0x011d_022c);
        grow_factor_first(e, b, f3, 0x011d_022c);
    }
}

// Translated from 00648620 (decompiled, FalloutNV.exe 1.4.0.525)
/// A one-value formula (cdecl, nine words: the out float, two floats, a
/// mode, a flag, three factors and a second flag; the engine map has no
/// name): the out is `third / second / s1998`; then by mode (the exe's
/// jump table): 1 grows it by the first factor (`011d0394`) when neither
/// flag is set, then as 2; 2 grows it by the second factor (`011d14e8`)
/// when only the first flag is set; 3 grows it by the third factor
/// (`011d1524`); 4 with neither flag adds first- and third-factor terms,
/// otherwise grows it by the third factor; 5 with only the first flag adds
/// second- and third-factor terms, otherwise grows it by the third factor.
#[allow(clippy::too_many_arguments)]
pub fn fn_00648620(
    e: &mut Engine,
    out: Ptr,
    second: f32,
    third: f32,
    mode: u32,
    flag: bool,
    first_factor: f32,
    second_factor: f32,
    third_factor: f32,
    other_flag: bool,
) {
    let a = out.addr();
    let (f1, f2, f3) = (
        first_factor.to_bits(),
        second_factor.to_bits(),
        third_factor.to_bits(),
    );
    let ratio = x87_div(x87_load32(third.to_bits()), x87_m32(second.to_bits()));
    let divisor = setting_bits(e, 0x011d_1998);
    e.mem
        .set_u32(a, x87_store32(x87_div(ratio, x87_load32(divisor))));
    match mode.wrapping_sub(1) {
        0 | 1 => {
            if mode == 1 && !flag && !other_flag {
                grow_factor_last(e, a, f1, 0x011d_0394);
            }
            if flag && !other_flag {
                grow_factor_last(e, a, f2, 0x011d_14e8);
            }
        }
        2 => grow_factor_last(e, a, f3, 0x011d_1524),
        3 => {
            if !flag && !other_flag {
                let first = scaled_term(e, a, f1, 0x011d_0394);
                let second = scaled_term(e, a, f3, 0x011d_1524);
                add_terms(e, a, first, second);
            } else {
                grow_factor_last(e, a, f3, 0x011d_1524);
            }
        }
        4 => {
            if flag && !other_flag {
                let first = scaled_term(e, a, f2, 0x011d_14e8);
                let second = scaled_term(e, a, f3, 0x011d_1524);
                add_terms(e, a, first, second);
            } else {
                grow_factor_last(e, a, f3, 0x011d_1524);
            }
        }
        _ => {}
    }
}

// Translated from 00648810 (decompiled, FalloutNV.exe 1.4.0.525)
/// The one-value formula of [`fn_00648620`] through `004dd130` (cdecl,
/// nine words, the same order; the engine map has no name): the out is
/// `004dd130(third / second / s1998, 1 / s198c)`; then by mode: 1 grows it
/// by the first factor (`011d0f3c`) when neither flag is set, then as 2; 2
/// grows it by the second factor (`011cff40`) when only the first flag is
/// set; 3 grows it by the third factor (`011d0108`); 4 with neither flag
/// adds the product of the first-factor term, the third factor and
/// `s0108`, otherwise grows it by the third factor; 5 the same with the
/// second factor when only the first flag is set.
#[allow(clippy::too_many_arguments)]
pub fn fn_00648810(
    e: &mut Engine,
    out: Ptr,
    second: f32,
    third: f32,
    mode: u32,
    flag: bool,
    first_factor: f32,
    second_factor: f32,
    third_factor: f32,
    other_flag: bool,
) {
    let a = out.addr();
    let (f1, f2, f3) = (
        first_factor.to_bits(),
        second_factor.to_bits(),
        third_factor.to_bits(),
    );
    let base = setting_bits(e, 0x011d_198c);
    // FLD1; FDIVRP (DE F1): one over the setting.
    let inverse = x87_store32(x87_div(1.0, x87_load32(base)));
    let ratio = x87_div(x87_load32(third.to_bits()), x87_m32(second.to_bits()));
    let divisor = setting_bits(e, 0x011d_1998);
    let value = x87_store32(x87_div(ratio, x87_load32(divisor)));
    let result = e
        .call(
            FLOAT_PAIR_004DD130,
            &args![x87_quiet_bits(value), x87_quiet_bits(inverse)],
        )
        .f64();
    e.mem.set_u32(a, x87_store32(result));
    // `setting * (*out * factor) * third_factor` through `s0108`, added.
    let chained = |e: &mut Engine, factor: u32, setting: u32| {
        let term = scaled_term(e, a, factor, setting);
        let term = x87_mul(term, x87_m32(f3));
        let value = setting_bits(e, 0x011d_0108);
        let term = x87_mul(x87_load32(value), term);
        let sum = x87_add(term, x87_m32(e.mem.u32(a)));
        e.mem.set_u32(a, x87_store32(sum));
    };
    match mode.wrapping_sub(1) {
        0 | 1 => {
            if mode == 1 && !flag && !other_flag {
                grow_factor_last(e, a, f1, 0x011d_0f3c);
            }
            if flag && !other_flag {
                grow_factor_last(e, a, f2, 0x011c_ff40);
            }
        }
        2 => grow_factor_last(e, a, f3, 0x011d_0108),
        3 => {
            if !flag && !other_flag {
                chained(e, f1, 0x011d_0f3c);
            } else {
                grow_factor_last(e, a, f3, 0x011d_0108);
            }
        }
        4 => {
            if flag && !other_flag {
                chained(e, f2, 0x011c_ff40);
            } else {
                grow_factor_last(e, a, f3, 0x011d_0108);
            }
        }
        _ => {}
    }
}

// Translated from 00648a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The setting at `011d117c` times a whole number (cdecl, one word; the
/// answer in `ST0`, rounded to a float; the engine map has no name).
pub fn fn_00648a50(e: &mut Engine, value: i32) -> f32 {
    let setting = setting_bits(e, 0x011d_117c);
    let product = x87_store32(x87_mul(x87_load32(setting), f64::from(value)));
    f32::from_bits(x87_quiet_bits(product))
}

// Translated from 00648a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// A falloff (cdecl, one float; the answer in `ST0`; the engine map has no
/// name): 0 when the setting at `011d111c` is below the value; otherwise
/// (NaN included) `s061c + (s08a4 - s061c) * (1 - value / s111c)`.
pub fn fn_00648a80(e: &mut Engine, value: f32) -> f32 {
    let limit = setting_bits(e, 0x011d_111c);
    // FLD value; FLD limit; FCOMPP; TEST AH,5; JP: the formula unless the
    // limit is below the value.
    if x87_load32(limit) < x87_load32(value.to_bits()) {
        return 0.0;
    }
    let limit = setting_bits(e, 0x011d_111c);
    let share = x87_div(x87_load32(value.to_bits()), x87_m32(limit));
    // FLD1; FSUBRP (DE E1): one minus the share.
    let rest = x87_store32(x87_sub(1.0, share));
    let low = setting_bits(e, 0x011d_061c);
    let high = setting_bits(e, 0x011d_08a4);
    let low_again = setting_bits(e, 0x011d_061c);
    let span = x87_sub(x87_load32(high), x87_m32(low_again));
    let result = x87_add(x87_mul(span, x87_m32(rest)), x87_m32(low));
    f32::from_bits(x87_quiet_bits(x87_store32(result)))
}

// Translated from 00648b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Seconds to hours (cdecl: a float and a flag; the answer in `ST0`; the
/// engine map has no name): `value / 3600`, and with the flag
/// `value / 3600 * s1164 + s01fc / 3600` (the settings at `011d1164`,
/// `011d01fc`).
pub fn fn_00648b00(e: &mut Engine, value: f32, flag: bool) -> f32 {
    let hour = e.global::<f64>(HOUR_SECONDS);
    let mut hours = x87_store32(x87_div(x87_load32(value.to_bits()), hour));
    if flag {
        let offset = setting_bits(e, 0x011d_01fc);
        let offset = x87_div(x87_load32(offset), hour);
        let mult = setting_bits(e, 0x011d_1164);
        let scaled = x87_mul(x87_load32(hours), x87_m32(mult));
        hours = x87_store32(x87_add(scaled, offset));
    }
    f32::from_bits(x87_quiet_bits(hours))
}

// Translated from 00648bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Skill points for a level (cdecl, one word: an `ActorValueOwner`; the
/// engine map has no name): `fn_00648c10(004a8f20(<vtable +0x1C>(9), 10),
/// <vtable +0x28>())` (the low 16 bits of the +0x28 answer). The exe pushes
/// the +0x28 answer and 10 ahead of the +0x1C call, which takes one word
/// (`RET 4` on the player's `ActorValueOwner` vtable `0108a974`, the only
/// caller's object): the 10 is `004a8f20`'s second word and the +0x28
/// answer `fn_00648c10`'s.
pub fn fn_00648bc0(e: &mut Engine, owner: u32) -> i32 {
    let level = e.vcall(owner, 0x28, &args![]).u32() & 0xffff;
    let value = e.vcall(owner, 0x1c, &args![9u32]).u32();
    let limited = e.call(COMBINE_FIRST, &args![value, 10u32]).i32();
    fn_00648c10(e, limited, level as i32)
}

// Translated from 00648c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// A value times the type-0x18 multiplier of its kind (cdecl: a float and
/// the kind; the answer in `ST0`; the engine map has no name):
/// `value * fn_00648cb0(kind, 0x18, 1)`, rounded to a float.
pub fn fn_00648c80(e: &mut Engine, value: f32, kind: u32) -> f32 {
    let value = x87_quiet_bits(value.to_bits());
    // FSTP float of the answer (already a float).
    let mult = fn_00648cb0(e, kind, 0x18, true).to_bits();
    let product = x87_store32(x87_mul(x87_load32(value), x87_m32(mult)));
    f32::from_bits(x87_quiet_bits(product))
}

// Translated from 00648cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A multiplier by kind and type (cdecl: the kind, the type, a flag; the
/// answer in `ST0`; the engine map has no name): 1.0, or for type 0x10 the
/// setting the table at `0119b310` (flag set) or `0119b324` gives for the
/// kind, for type 0x18 the one at `0119b338`.
pub fn fn_00648cb0(e: &mut Engine, kind: u32, kind_type: u32, flag: bool) -> f32 {
    let table = match kind_type {
        0x10 if flag => 0x0119_b310u32,
        0x10 => 0x0119_b324,
        0x18 => 0x0119_b338,
        _ => return 1.0,
    };
    let setting = e.global::<u32>(table.wrapping_add(kind.wrapping_mul(4)));
    f32::from_bits(x87_quiet_bits(setting_bits(e, setting)))
}

// Translated from 00648d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Three weights (cdecl: three out floats, two floats and a percentage;
/// the engine map has no name): the first and third are
/// `percent / 100 * 0.7 + (1 - first) * 0.2 + (1 - second) * 0.1`, the
/// second `(100 - percent) / 100 * 0.7 + first * 0.2 + second * 0.1`.
pub fn fn_00648d70(
    e: &mut Engine,
    first_out: Ptr,
    second_out: Ptr,
    third_out: Ptr,
    first: f32,
    second: f32,
    percent: i32,
) {
    let hundred = e.global::<f64>(HUNDRED);
    let seven = e.global::<f64>(SEVEN_TENTHS);
    let fifth = e.global::<f64>(FIFTH);
    let tenth = e.global::<f64>(TENTH);
    let (first, second) = (first.to_bits(), second.to_bits());
    let low = |percent_part: f64| {
        let total = x87_mul(percent_part, seven);
        // FLD x; FLD1; FSUBRP (DE E1): one minus x.
        let term = x87_mul(x87_sub(1.0, x87_load32(first)), fifth);
        let total = x87_add(total, term);
        let term = x87_mul(x87_sub(1.0, x87_load32(second)), tenth);
        x87_store32(x87_add(total, term))
    };
    let value = low(x87_div(f64::from(percent), hundred));
    e.mem.set_u32(first_out.addr(), value);
    // FSUBR double: 100 minus the percentage.
    let rest = x87_div(x87_sub(hundred, f64::from(percent)), hundred);
    let total = x87_mul(rest, seven);
    let total = x87_add(total, x87_mul(x87_load32(first), fifth));
    let total = x87_add(total, x87_mul(x87_load32(second), tenth));
    e.mem.set_u32(second_out.addr(), x87_store32(total));
    let value = low(x87_div(f64::from(percent), hundred));
    e.mem.set_u32(third_out.addr(), value);
}

// Translated from 00648e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GamePlayFormulas::PainChangePerSecond` (Xbox PDB; cdecl, one float;
/// `ST0`): the value times 60 (`double` at `01012638`), rounded to a float.
pub fn game_play_formulas_pain_change_per_second(e: &mut Engine, value: f32) -> f32 {
    let product = x87_mul(x87_load32(value.to_bits()), e.global::<f64>(SIXTY));
    f32::from_bits(x87_quiet_bits(x87_store32(product)))
}

// Translated from 00648e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GamePlayFormulas::GetPainCausedByWound` (Xbox PDB; cdecl, one float;
/// `ST0`): the value times 0.5 (`double` at `01011588`), rounded to a
/// float.
pub fn game_play_formulas_get_pain_caused_by_wound(e: &mut Engine, value: f32) -> f32 {
    let product = x87_mul(x87_load32(value.to_bits()), e.global::<f64>(HALF));
    f32::from_bits(x87_quiet_bits(x87_store32(product)))
}

// Translated from 00648ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A chance in percent (cdecl: two floats and a whole number; the engine
/// map has no name): `((first - s07c0) * s0c90 / 100 + 1) * ((second -
/// s0a24) * s1110 / 100 + 1) * (100 - third) / 100`, each factor rounded to
/// a float; 100 when the setting at `011d00f0` is below it, otherwise (NaN
/// included) `_ftol2(chance * 100 + 0.5)`.
pub fn fn_00648ea0(e: &mut Engine, first: f32, second: f32, third: i32) -> i32 {
    let hundred = e.global::<f64>(HUNDRED);
    let scale = setting_bits(e, 0x011d_0c90);
    let offset = setting_bits(e, 0x011d_07c0);
    let term = x87_sub(x87_load32(first.to_bits()), x87_m32(offset));
    let term = x87_div(x87_mul(term, x87_m32(scale)), hundred);
    // FLD1; FADDP.
    let first_factor = x87_store32(x87_add(term, 1.0));
    let scale = setting_bits(e, 0x011d_1110);
    let offset = setting_bits(e, 0x011d_0a24);
    let term = x87_sub(x87_load32(second.to_bits()), x87_m32(offset));
    let term = x87_div(x87_mul(term, x87_m32(scale)), hundred);
    let second_factor = x87_store32(x87_add(term, 1.0));
    let rest = 100i32.wrapping_sub(third);
    let third_factor = x87_store32(x87_div(f64::from(rest), hundred));
    let chance = x87_mul(x87_load32(first_factor), x87_m32(second_factor));
    let chance = x87_store32(x87_mul(chance, x87_m32(third_factor)));
    let limit = setting_bits(e, 0x011d_00f0);
    // FLD chance; FLD limit; FCOMPP; TEST AH,5; JP: rounded unless the
    // limit is below the chance.
    if x87_load32(limit) < x87_load32(chance) {
        return 100;
    }
    let value = x87_mul(x87_load32(chance), hundred);
    let value = x87_add(value, e.global::<f64>(HALF));
    e.call(FTOL2, &args![value]).i32()
}

// Translated from 00648f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A penalty (cdecl, one whole number; `ST0`; the engine map has no name):
/// `-s1f1c - (1 - value / 100) * s14d0` (the settings at `011cff1c` and
/// `011d14d0`), rounded to a float.
pub fn fn_00648f60(e: &mut Engine, value: i32) -> f32 {
    let base = setting_bits(e, 0x011c_ff1c);
    // FLD; FCHS; FSTP double.
    let negated = -x87_load32(base);
    let mult = setting_bits(e, 0x011d_14d0);
    let share = x87_div(f64::from(value), e.global::<f64>(HUNDRED));
    // FLD1; FSUBRP (DE E1): one minus the share.
    let rest = x87_mul(x87_sub(1.0, share), x87_m32(mult));
    // FSUBR double: the negated base minus the product.
    let result = x87_store32(x87_sub(negated, rest));
    f32::from_bits(x87_quiet_bits(result))
}

// Translated from 00648fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GamePlayFormulas::GetSurgeryAutoAttemptPercentages` (Xbox PDB; cdecl:
/// two out floats, a whole number, a byte and a float): the first out is
/// `00476b90(0, value * 100 / 100 * (byte * 0.5 + 1))` and the second
/// `00476b90(0, (value - 100) / 100 * (float - 100) / 100 * 100 *
/// (1 - byte * 0.5))`, each on the object `00476c00` answers.
pub fn game_play_formulas_get_surgery_auto_attempt_percentages(
    e: &mut Engine,
    first_out: Ptr,
    second_out: Ptr,
    value: i32,
    level: u8,
    other: f32,
) {
    let hundred = e.global::<f64>(HUNDRED);
    let half = e.global::<f64>(HALF);
    let share = x87_div(x87_mul(f64::from(value), hundred), hundred);
    // FLD1; FADDP; FMULP.
    let bonus = x87_add(x87_mul(f64::from(level), half), 1.0);
    let first = x87_store32(x87_mul(share, bonus));
    let owner = e.call(FLOAT_PAIR_OWNER, &args![]).u32();
    let result = e
        .call(
            FLOAT_PAIR_00476B90,
            &args![owner, 0.0f32.to_bits(), x87_quiet_bits(first)],
        )
        .f64();
    e.mem.set_u32(first_out.addr(), x87_store32(result));
    let part = x87_div(x87_sub(f64::from(value), hundred), hundred);
    let other_part = x87_div(x87_sub(x87_load32(other.to_bits()), hundred), hundred);
    let product = x87_mul(x87_mul(part, other_part), hundred);
    // FLD1; FSUBRP (DE E1): one minus the half byte; FMULP.
    let malus = x87_sub(1.0, x87_mul(f64::from(level), half));
    let second = x87_store32(x87_mul(product, malus));
    let owner = e.call(FLOAT_PAIR_OWNER, &args![]).u32();
    let result = e
        .call(
            FLOAT_PAIR_00476B90,
            &args![owner, 0.0f32.to_bits(), x87_quiet_bits(second)],
        )
        .f64();
    e.mem.set_u32(second_out.addr(), x87_store32(result));
}

// Translated from 00649050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GamePlayFormulas::GetBarterMultipliers` (Xbox PDB; cdecl: a whole
/// number and two out floats): `value * s040c / 100 + s03c4` and
/// `value * s0370 / 100 + s0760` (the settings at `011d040c`, `011d03c4`,
/// `011d0370`, `011d0760`).
pub fn game_play_formulas_get_barter_multipliers(e: &mut Engine, value: i32, buy: Ptr, sell: Ptr) {
    let hundred = e.global::<f64>(HUNDRED);
    for (out, base, mult) in [
        (buy.addr(), 0x011d_03c4u32, 0x011d_040cu32),
        (sell.addr(), 0x011d_0760, 0x011d_0370),
    ] {
        let base = setting_bits(e, base);
        let mult = setting_bits(e, mult);
        let product = x87_div(x87_mul(f64::from(value), x87_m32(mult)), hundred);
        let sum = x87_add(product, x87_m32(base));
        e.mem.set_u32(out, x87_store32(sum));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0064_8090,
            game_play_formulas_get_repaired_condition(i32, f32, f32, Ptr) -> f32
        ),
        entry!(
            0x0064_8230,
            game_play_formulas_get_repair_cost(i32, f32, f32) -> i32
        ),
        entry!(
            0x0064_82f0,
            fn_006482f0(Ptr, Ptr, f32, f32, u32, bool, f32, f32, f32, bool)
        ),
        entry!(
            0x0064_8620,
            fn_00648620(Ptr, f32, f32, u32, bool, f32, f32, f32, bool)
        ),
        entry!(
            0x0064_8810,
            fn_00648810(Ptr, f32, f32, u32, bool, f32, f32, f32, bool)
        ),
        entry!(0x0064_8a50, fn_00648a50(i32) -> f32),
        entry!(0x0064_8a80, fn_00648a80(f32) -> f32),
        entry!(0x0064_8b00, fn_00648b00(f32, bool) -> f32),
        entry!(0x0064_8bc0, fn_00648bc0(u32) -> i32),
        entry!(0x0064_8c80, fn_00648c80(f32, u32) -> f32),
        entry!(0x0064_8cb0, fn_00648cb0(u32, u32, bool) -> f32),
        entry!(0x0064_8d70, fn_00648d70(Ptr, Ptr, Ptr, f32, f32, i32)),
        entry!(
            0x0064_8e10,
            game_play_formulas_pain_change_per_second(f32) -> f32
        ),
        entry!(
            0x0064_8e30,
            game_play_formulas_get_pain_caused_by_wound(f32) -> f32
        ),
        entry!(0x0064_8ea0, fn_00648ea0(f32, f32, i32) -> i32),
        entry!(0x0064_8f60, fn_00648f60(i32) -> f32),
        entry!(
            0x0064_8fa0,
            game_play_formulas_get_surgery_auto_attempt_percentages(Ptr, Ptr, i32, u8, f32)
        ),
        entry!(
            0x0064_9050,
            game_play_formulas_get_barter_multipliers(i32, Ptr, Ptr)
        ),
        entry!(0x0064_8e50, fn_00648e50(f32, f32, f32) -> f32),
        entry!(
            0x0064_8b50,
            game_play_formulas_get_required_experience_points(i32) -> i32
        ),
        entry!(0x0064_8c10, fn_00648c10(i32, i32) -> i32),
        entry!(0x0064_8a10, game_play_formulas_swim_breath_time(i32) -> f32),
        entry!(
            0x0064_8d30,
            game_play_formulas_get_force_lock_chance(i32, i32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Settings whose value sits at +4 (the double answers `this + 4`).
    fn settings(e: &mut Engine) {
        e.map(0x011d_0000, 0x1000);
        e.register(INT_SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(FLOAT_SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
    }

    #[test]
    fn required_experience_grows_by_the_bump() {
        let mut e = Engine::new();
        settings(&mut e);
        e.mem.set_u32(I_XP_BASE + 4, 200);
        e.mem.set_u32(I_XP_BUMP_BASE + 4, 150);
        let at = |e: &mut Engine, level: i32| e.call(0x0064_8b50, &[level as u32]).i32();
        assert_eq!(at(&mut e, 1), 0);
        assert_eq!(at(&mut e, -5), 0);
        assert_eq!(at(&mut e, 2), 200);
        assert_eq!(at(&mut e, 3), 200 + 350);
        assert_eq!(at(&mut e, 4), 200 + 350 + 500);
    }

    #[test]
    fn fn_00648c10_rounds_and_adds_one_for_the_parities() {
        let mut e = Engine::new();
        e.map(SCALE_DOUBLE & !0xfff, 0x1000);
        e.map(OFFSET_DOUBLE & !0xfff, 0x1000);
        e.mem.set_f64(SCALE_DOUBLE, 0.5);
        e.mem.set_f64(OFFSET_DOUBLE, 10.0);
        e.register(FLOAT_FUNCTION, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])),
            ..Ret::default()
        });
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        let at = |e: &mut Engine, a: i32, b: i32| e.call(0x0064_8c10, &[a as u32, b as u32]).i32();
        e.call_log = Some(vec![]);
        // 6 * 0.5 + 10 = 13.
        assert_eq!(at(&mut e, 6, 2), 13);
        assert_eq!(calls_to(&e, FLOAT_FUNCTION), vec![vec![13.0f32.to_bits()]]);
        // 5: 12.5 truncated to 12, odd first and even second: 13.
        assert_eq!(at(&mut e, 5, 4), 13);
        assert_eq!(at(&mut e, 5, 3), 12);
        // -5 % 2 is -1: no extra point.
        assert_eq!(at(&mut e, -5, 0), 7);
    }

    #[test]
    fn swim_breath_time_is_mult_times_value_plus_base() {
        let mut e = Engine::new();
        settings(&mut e);
        e.mem.set_f32(F_ACTOR_SWIM_BREATH_BASE + 4, 10.0);
        e.mem.set_f32(F_ACTOR_SWIM_BREATH_MULT + 4, 1.5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_8a10, &[4]).f64(), 16.0);
        assert_eq!(
            calls_to(&e, FLOAT_SETTING_VALUE),
            vec![
                vec![F_ACTOR_SWIM_BREATH_BASE],
                vec![F_ACTOR_SWIM_BREATH_MULT]
            ]
        );
    }

    #[test]
    fn force_lock_chance_combines_the_difference_and_the_base() {
        let mut e = Engine::new();
        settings(&mut e);
        e.mem.set_f32(F_LOCK_SKILL_BASE + 4, 20.75);
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        e.register(COMBINE_FIRST, |_, a| {
            (a[0] as i32).min(a[1] as i32).into_ret()
        });
        e.register(COMBINE_SECOND, |_, a| {
            (a[0] as i32).max(a[1] as i32).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_8d30, &[50, 30]).u32(), 40);
        assert_eq!(calls_to(&e, COMBINE_FIRST), vec![vec![40, 100]]);
        assert_eq!(calls_to(&e, COMBINE_SECOND), vec![vec![40, 0]]);
    }
}

#[cfg(test)]
mod la1_tests {
    use super::*;

    #[test]
    fn fn_00648e50_scales_by_the_clamped_ratio() {
        let mut e = Engine::new();
        // Doubles: the larger and the smaller of the two floats.
        e.register(FLOAT_PAIR_00404010, |_, a| {
            f32::from_bits(a[0]).max(f32::from_bits(a[1])).into_ret()
        });
        e.register(FLOAT_PAIR_0040EBD0, |_, a| {
            f32::from_bits(a[0]).min(f32::from_bits(a[1])).into_ret()
        });
        e.call_log = Some(vec![]);
        // (8 - 2) / 8 = 0.75, times 10.
        let value = e.call(0x0064_8e50, &args![10.0f32, 8.0f32, 2.0f32]).f32();
        assert_eq!(value, 7.5);
        let log = e.call_log.take().unwrap();
        let inner = log.iter().find(|(a, _)| *a == FLOAT_PAIR_00404010).unwrap();
        assert_eq!(inner.1, vec![0.0f32.to_bits(), 0.75f32.to_bits()]);
        let outer = log.iter().find(|(a, _)| *a == FLOAT_PAIR_0040EBD0).unwrap();
        assert_eq!(outer.1, vec![1.0f32.to_bits(), 0.75f32.to_bits()]);
        // A third above the second: the ratio is negative, clamped to 0 by the doubles.
        let value = e.call(0x0064_8e50, &args![10.0f32, 4.0f32, 6.0f32]).f32();
        assert_eq!(value, 0.0);
        // The ratio is rounded to a float before the calls: 1/3.
        e.call_log = Some(vec![]);
        e.call(0x0064_8e50, &args![1.0f32, 3.0f32, 2.0f32]);
        let log = e.call_log.take().unwrap();
        let inner = log.iter().find(|(a, _)| *a == FLOAT_PAIR_00404010).unwrap();
        assert_eq!(inner.1[1], ((1.0f64 / 3.0) as f32).to_bits());
    }
}

#[cfg(test)]
mod n085_tests {
    use super::*;

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The exe's constants, settings whose value sits at +4, `_ftol2`
    /// truncating.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x0101_7000,
            0x0101_8000,
            0x0101_f000,
            0x0102_0000,
            0x0104_e000,
            0x0119_b000,
            0x011c_f000,
            0x011d_0000,
            0x011d_1000,
        ] {
            e.map(page, 0x1000);
        }
        for (at, value) in [
            (HUNDRED, 100.0f64),
            (TEN, 10.0),
            (HALF, 0.5),
            (ONE, 1.0),
            (TENTH, f64::from_bits(0x3fb9_9999_a000_0000)),
            (FIFTH, f64::from_bits(0x3fc9_9999_a000_0000)),
            (SEVEN_TENTHS, f64::from_bits(0x3fe6_6666_6000_0000)),
            (HOUR_SECONDS, 3600.0),
            (SIXTY, 60.0),
        ] {
            e.mem.set_f64(at, value);
        }
        e.register(FLOAT_SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(FTOL2, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        e
    }

    fn set(e: &mut Engine, setting: u32, value: f32) {
        e.mem.set_f32(setting + 4, value);
    }

    fn f(word: u32) -> f32 {
        f32::from_bits(word)
    }

    #[test]
    fn repaired_condition_adds_the_parts_and_fills_the_out_word() {
        let mut e = engine();
        set(&mut e, 0x011c_ffa0, 0.6);
        set(&mut e, 0x011d_0880, 0.1);
        set(&mut e, 0x011d_0748, 0.5);
        set(&mut e, 0x011d_0ff0, 0.2);
        set(&mut e, 0x011d_018c, 1.0);
        // The pair doubles: the smaller and the larger.
        e.register(FLOAT_PAIR_0040EBD0, |_, a| {
            st0(f64::from(f(a[0]).min(f(a[1]))))
        });
        e.register(FLOAT_PAIR_00404010, |_, a| {
            st0(f64::from(f(a[0]).max(f(a[1]))))
        });
        e.register(FLOAT_PAIR_004BD510, |_, a| st0(f64::from(f(a[0]))));
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0xdead);
        e.call_log = Some(vec![]);
        let value = e
            .call(0x0064_8090, &args![50i32, 4.0f32, 2.0f32, out])
            .f32();
        // 0.4 + 0.1 + 50 * 0.5 / 100 + 0.2 * 0.5.
        let lesser = (f64::from(0.2f32) * 0.5) as f32;
        let skill = ((f64::from(0.6f32) - f64::from(0.1f32)) * 50.0 / 100.0) as f32;
        let greater = 0.4f32;
        let condition = (((f64::from(greater) + f64::from(0.1f32)) + f64::from(skill))
            + f64::from(lesser)) as f32;
        assert_eq!(value, (f64::from(condition) * 10.0) as f32);
        assert_eq!(
            calls(&e, FLOAT_PAIR_0040EBD0),
            vec![vec![0.4f32.to_bits(), 0.2f32.to_bits()]]
        );
        let percents = calls(&e, FLOAT_PAIR_004BD510);
        assert_eq!(percents[1], vec![40.0f32.to_bits(), 1.0f32.to_bits()]);
        // The condition's percentage (about 85) is above the larger's (40):
        // the out word stays.
        assert_eq!(e.mem.u32(out), 0xdead);

        // A low skill: the condition is no more than the larger's percentage.
        set(&mut e, 0x011d_0880, -0.5);
        set(&mut e, 0x011d_0748, 0.0);
        let _ = e.call(0x0064_8090, &args![0i32, 4.0f32, 2.0f32, out]);
        let raised = f64::from(0.4f32) + f64::from_bits(0x3fb9_9999_a000_0000);
        let numerator = (raised - f64::from(0.2f32)) * 100.0;
        let expected = (numerator / (1.0 - f64::from(0.2f32))) as i32;
        assert_eq!(e.mem.u32(out), expected as u32);

        // Without an out word, no percentages.
        e.call_log = Some(vec![]);
        let _ = e.call(0x0064_8090, &args![0i32, 4.0f32, 2.0f32, 0u32]);
        assert!(calls(&e, FLOAT_PAIR_004BD510).is_empty());
    }

    #[test]
    fn repair_cost_is_zero_up_to_one_half() {
        let mut e = engine();
        set(&mut e, 0x011d_0ff0, 0.5);
        set(&mut e, 0x011d_018c, 1.5);
        set(&mut e, 0x011d_1158, 2.0);
        e.register(FLOAT_PAIR_004BD510, |_, a| st0(f64::from(f(a[0]))));
        e.register(COMBINE_SECOND, |_, a| (a[0] * 10 + a[1]).into_ret());
        e.call_log = Some(vec![]);
        // (0.5 + 50 * 1.0 / 100 - 2 / 10) / 10 * 2 * 100 = 16.
        let cost = e.call(0x0064_8230, &args![50i32, 2.0f32, 100.0f32]).i32();
        let share = ((0.5f64 + 0.5) as f32 as f64 - f64::from(0.2f32)) / 10.0;
        let expected = (2.0 * 100.0 * share) as f32;
        assert_eq!(
            calls(&e, FLOAT_PAIR_004BD510),
            vec![vec![expected.to_bits(), 1.0f32.to_bits()]]
        );
        assert_eq!(cost, expected as i32 * 10 + 1);
        // A tiny value: no cost.
        assert_eq!(e.call(0x0064_8230, &args![50i32, 2.0f32, 0.1f32]).i32(), 0);
    }

    /// Settings for the out formulas: each at 0.5 unless named.
    fn formula_engine(settings: &[u32]) -> Engine {
        let mut e = engine();
        for &s in settings {
            set(&mut e, s, 0.5);
        }
        e.register(FLOAT_PAIR_004DD130, |_, a| st0(f64::from(f(a[0]))));
        e
    }

    const SETTINGS_82F0: [u32; 9] = [
        0x011d_198c,
        0x011d_00e4,
        0x011d_1998,
        0x011d_06b8,
        0x011d_0568,
        0x011d_02b0,
        0x011d_009c,
        0x011d_022c,
        0x011c_ff40,
    ];

    fn run_82f0(mode: u32, flag: bool, other: bool) -> (f32, f32, Vec<u32>) {
        let mut e = formula_engine(&SETTINGS_82F0);
        let (a, b) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call_log = Some(vec![]);
        e.call(
            0x0064_82f0,
            &args![a, b, 2.0f32, 4.0f32, mode, flag, 1.0f32, 2.0f32, 4.0f32, other],
        );
        let used: Vec<u32> = calls(&e, FLOAT_SETTING_VALUE)
            .iter()
            .map(|w| w[0])
            .skip(4)
            .collect();
        (e.mem.f32(a), e.mem.f32(b), used)
    }

    #[test]
    fn two_value_formula_follows_the_jump_table() {
        // 004dd130 echoes its first float: 4 * 0.5 / 2 = 1; the second out
        // 1 / 0.5 / 0.5 = 4.
        let (a, b, used) = run_82f0(9, false, false);
        assert_eq!((a, b), (1.0, 4.0));
        assert!(used.is_empty());
        // Mode 1, no flags: grown by the first factor (1 * 0.5): 1.5, 6.
        let (a, b, used) = run_82f0(1, false, false);
        assert_eq!((a, b), (1.5, 6.0));
        assert_eq!(used, vec![0x011d_06b8, 0x011d_0568]);
        // Mode 1 with the first flag falls into mode 2's growth (2 * 0.5).
        let (a, b, used) = run_82f0(1, true, false);
        assert_eq!((a, b), (2.0, 8.0));
        assert_eq!(used, vec![0x011d_02b0, 0x011d_009c]);
        // The other flag stops both.
        let (a, b, _) = run_82f0(2, true, true);
        assert_eq!((a, b), (1.0, 4.0));
        // Mode 3: the third factor (4 * 0.5).
        let (a, b, _) = run_82f0(3, true, true);
        assert_eq!((a, b), (3.0, 12.0));
        // Mode 4, no flags: the first out takes 0.5 * 1 * 1 + 0.5 * 4 * 4
        // (from the second out) + 1 = 9.5; the second 0.5*4 + 0.5*16 + 4.
        let (a, b, used) = run_82f0(4, false, false);
        assert_eq!((a, b), (9.5, 14.0));
        assert_eq!(
            used,
            vec![0x011d_06b8, 0x011d_022c, 0x011d_0568, 0x011d_022c]
        );
        // Mode 4 with a flag: the third factor once.
        let (a, b, _) = run_82f0(4, true, false);
        assert_eq!((a, b), (3.0, 12.0));
        // Mode 5 with only the first flag: second- and third-factor terms
        // (1 + 1 + 2 = 4, 4 + 8 + 4 = 16), then mode 3 (12, 48).
        let (a, b, used) = run_82f0(5, true, false);
        assert_eq!((a, b), (12.0, 48.0));
        assert_eq!(used.len(), 6);
        // Mode 5 otherwise: the third factor twice.
        let (a, b, _) = run_82f0(5, false, false);
        assert_eq!((a, b), (9.0, 36.0));
    }

    fn run_one(addr: u32, mode: u32, flag: bool, other: bool) -> (f32, Vec<u32>) {
        let mut e = formula_engine(&[
            0x011d_198c,
            0x011d_1998,
            0x011d_0394,
            0x011d_14e8,
            0x011d_1524,
            0x011d_0f3c,
            0x011c_ff40,
            0x011d_0108,
        ]);
        let a = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(
            addr,
            &args![a, 2.0f32, 4.0f32, mode, flag, 1.0f32, 2.0f32, 4.0f32, other],
        );
        let used = calls(&e, FLOAT_SETTING_VALUE)
            .iter()
            .map(|w| w[0])
            .collect();
        (e.mem.f32(a), used)
    }

    #[test]
    fn one_value_formula_follows_the_jump_table() {
        // 4 / 2 / 0.5 = 4.
        assert_eq!(run_one(0x0064_8620, 0, false, false).0, 4.0);
        // Mode 1: 0.5 * (4 * 1) + 4.
        let (value, used) = run_one(0x0064_8620, 1, false, false);
        assert_eq!(value, 6.0);
        assert_eq!(used, vec![0x011d_1998, 0x011d_0394]);
        // Mode 2 with the flag: 0.5 * (4 * 2) + 4.
        assert_eq!(run_one(0x0064_8620, 2, true, false).0, 8.0);
        // Mode 3: 0.5 * 16 + 4.
        assert_eq!(run_one(0x0064_8620, 3, true, true).0, 12.0);
        // Mode 4, no flags: 0.5*4 + 0.5*16 + 4.
        assert_eq!(run_one(0x0064_8620, 4, false, false).0, 14.0);
        assert_eq!(run_one(0x0064_8620, 4, true, false).0, 12.0);
        // Mode 5 with the flag: 0.5*8 + 0.5*16 + 4.
        let (value, used) = run_one(0x0064_8620, 5, true, false);
        assert_eq!(value, 16.0);
        assert_eq!(used, vec![0x011d_1998, 0x011d_14e8, 0x011d_1524]);
        assert_eq!(run_one(0x0064_8620, 5, false, false).0, 12.0);
    }

    #[test]
    fn one_value_formula_through_004dd130_follows_the_jump_table() {
        // 004dd130 echoes 4 / 2 / 0.5 = 4; its second float is 1 / 0.5.
        let mut e = formula_engine(&[0x011d_198c, 0x011d_1998]);
        let a = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(
            0x0064_8810,
            &args![a, 2.0f32, 4.0f32, 0u32, false, 1.0f32, 2.0f32, 4.0f32, false],
        );
        assert_eq!(
            calls(&e, FLOAT_PAIR_004DD130),
            vec![vec![4.0f32.to_bits(), 2.0f32.to_bits()]]
        );
        assert_eq!(e.mem.f32(a), 4.0);
        // Mode 1: 0.5 * 4 + 4; mode 2 with the flag: 0.5 * 8 + 4.
        assert_eq!(run_one(0x0064_8810, 1, false, false).0, 6.0);
        let (value, used) = run_one(0x0064_8810, 2, true, false);
        assert_eq!(value, 8.0);
        assert_eq!(used.last(), Some(&0x011c_ff40));
        // Mode 3: 0.5 * 16 + 4.
        assert_eq!(run_one(0x0064_8810, 3, false, false).0, 12.0);
        // Mode 4, no flags: 0.5 * (0.5 * 4 * 4) + 4 = 8.
        let (value, used) = run_one(0x0064_8810, 4, false, false);
        assert_eq!(value, 8.0);
        assert_eq!(&used[2..], &[0x011d_0f3c, 0x011d_0108]);
        // Mode 5 with the flag: 0.5 * (0.5 * 8 * 4) + 4 = 12.
        assert_eq!(run_one(0x0064_8810, 5, true, false).0, 12.0);
        assert_eq!(run_one(0x0064_8810, 5, false, false).0, 12.0);
    }

    #[test]
    fn small_formulas() {
        let mut e = engine();
        set(&mut e, 0x011d_117c, 1.5);
        assert_eq!(e.call(0x0064_8a50, &args![-4i32]).f32(), -6.0);

        // The falloff: 0 above the limit, the blend below it.
        set(&mut e, 0x011d_111c, 10.0);
        set(&mut e, 0x011d_061c, 2.0);
        set(&mut e, 0x011d_08a4, 6.0);
        assert_eq!(e.call(0x0064_8a80, &args![11.0f32]).f32(), 0.0);
        assert_eq!(e.call(0x0064_8a80, &args![5.0f32]).f32(), 4.0);
        assert_eq!(e.call(0x0064_8a80, &args![10.0f32]).f32(), 2.0);
        assert!(e.call(0x0064_8a80, &args![f32::NAN]).f32().is_nan());

        // Seconds to hours, with the flag's scale and offset.
        set(&mut e, 0x011d_01fc, 7200.0);
        set(&mut e, 0x011d_1164, 3.0);
        assert_eq!(e.call(0x0064_8b00, &args![1800.0f32, false]).f32(), 0.5);
        assert_eq!(e.call(0x0064_8b00, &args![1800.0f32, true]).f32(), 3.5);

        assert_eq!(e.call(0x0064_8e10, &args![0.5f32]).f32(), 30.0);
        assert_eq!(e.call(0x0064_8e30, &args![3.0f32]).f32(), 1.5);

        // -2 - (1 - 25 / 100) * 4 = -5.
        set(&mut e, 0x011c_ff1c, 2.0);
        set(&mut e, 0x011d_14d0, 4.0);
        assert_eq!(e.call(0x0064_8f60, &args![25i32]).f32(), -5.0);
    }

    #[test]
    fn multipliers_by_kind_and_type() {
        let mut e = engine();
        for (table, setting, value) in [
            (0x0119_b310u32, 0x011d_0100u32, 2.0f32),
            (0x0119_b324, 0x011d_0200, 3.0),
            (0x0119_b338, 0x011d_0300, 4.0),
        ] {
            e.mem.set_u32(table + 8, setting);
            set(&mut e, setting, value);
        }
        let at = |e: &mut Engine, kind_type: u32, flag: bool| {
            e.call(0x0064_8cb0, &args![2u32, kind_type, flag]).f32()
        };
        assert_eq!(at(&mut e, 0x10, true), 2.0);
        assert_eq!(at(&mut e, 0x10, false), 3.0);
        assert_eq!(at(&mut e, 0x18, false), 4.0);
        assert_eq!(at(&mut e, 0x11, true), 1.0);
        assert_eq!(e.call(0x0064_8c80, &args![1.5f32, 2u32]).f32(), 6.0);
    }

    #[test]
    fn skill_points_take_intelligence_capped_at_ten_and_the_level() {
        let mut e = engine();
        let owner = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x30);
        e.mem.set_u32(owner, vtable);
        e.mem.set_u32(vtable + 0x1c, 0x7700_001c);
        e.mem.set_u32(vtable + 0x28, 0x7700_0028);
        e.register(0x7700_001c, |_, _| 12u32.into_ret());
        e.register(0x7700_0028, |_, _| 0xffff_0003u32.into_ret());
        e.register(COMBINE_FIRST, |_, a| {
            (a[0] as i32).min(a[1] as i32).into_ret()
        });
        e.register(FLOAT_FUNCTION, |_, a| st0(f64::from(f(a[0]))));
        e.mem.set_f64(SCALE_DOUBLE, 0.5);
        e.mem.set_f64(OFFSET_DOUBLE, 10.0);
        e.call_log = Some(vec![]);
        // min(12, 10) = 10: 10 * 0.5 + 10 = 15 (level 3 is odd: no extra).
        assert_eq!(e.call(0x0064_8bc0, &args![owner]).i32(), 15);
        assert_eq!(calls(&e, 0x7700_001c), vec![vec![owner, 9]]);
        assert_eq!(calls(&e, COMBINE_FIRST), vec![vec![12, 10]]);
    }

    #[test]
    fn three_weights() {
        let mut e = engine();
        let (a, b, c) = (e.mem.alloc(4), e.mem.alloc(4), e.mem.alloc(4));
        e.call(0x0064_8d70, &args![a, b, c, 0.5f32, 0.25f32, 40i32]);
        let seven = f64::from_bits(0x3fe6_6666_6000_0000);
        let fifth = f64::from_bits(0x3fc9_9999_a000_0000);
        let tenth = f64::from_bits(0x3fb9_9999_a000_0000);
        let low = (0.4 * seven + 0.5 * fifth + 0.75 * tenth) as f32;
        let high = (0.6 * seven + 0.5 * fifth + 0.25 * tenth) as f32;
        assert_eq!(e.mem.f32(a), low);
        assert_eq!(e.mem.f32(b), high);
        assert_eq!(e.mem.f32(c), low);
    }

    #[test]
    fn chance_is_capped_at_one_hundred() {
        let mut e = engine();
        set(&mut e, 0x011d_0c90, 10.0);
        set(&mut e, 0x011d_07c0, 5.0);
        set(&mut e, 0x011d_1110, 20.0);
        set(&mut e, 0x011d_0a24, 0.0);
        set(&mut e, 0x011d_00f0, 1.0);
        // (1 + 5 * 10 / 100) * (1 + 0) * 0.5 = 0.75: 75.
        assert_eq!(
            e.call(0x0064_8ea0, &args![10.0f32, 0.0f32, 50i32]).i32(),
            75
        );
        // (1.5) * (1 + 10 * 20 / 100) * 0.5 = 2.25: above the limit.
        assert_eq!(
            e.call(0x0064_8ea0, &args![10.0f32, 10.0f32, 50i32]).i32(),
            100
        );
    }

    #[test]
    fn surgery_percentages_and_barter_multipliers() {
        let mut e = engine();
        e.register(FLOAT_PAIR_OWNER, |_, _| 0x4400u32.into_ret());
        e.register(FLOAT_PAIR_00476B90, |_, a| st0(f64::from(f(a[2]) + 1.0)));
        let (a, b) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call_log = Some(vec![]);
        e.call(0x0064_8fa0, &args![a, b, 50i32, 1u8, 80.0f32]);
        // 50 * 1.5 = 75; (-0.5) * (-0.2) * 100 * 0.5 = 5.
        let pairs = calls(&e, FLOAT_PAIR_00476B90);
        assert_eq!(pairs[0], vec![0x4400, 0, 75.0f32.to_bits()]);
        assert_eq!(e.mem.f32(a), 76.0);
        let second = ((-0.5f64) * ((f64::from(80.0f32) - 100.0) / 100.0) * 100.0 * 0.5) as f32;
        assert_eq!(pairs[1][2], second.to_bits());

        set(&mut e, 0x011d_03c4, 1.0);
        set(&mut e, 0x011d_040c, 2.0);
        set(&mut e, 0x011d_0760, 3.0);
        set(&mut e, 0x011d_0370, 4.0);
        e.call(0x0064_9050, &args![50i32, a, b]);
        assert_eq!(e.mem.f32(a), 2.0);
        assert_eq!(e.mem.f32(b), 5.0);
    }
}
