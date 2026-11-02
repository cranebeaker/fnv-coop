//! `fallout shared/magic/magicformulas.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The value of a game setting (`00403e20`, `ECX` = the setting): the
/// address of its value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// Game settings (names from their static initializers):
/// `fMagicAreaBaseCostMult`, `fMagicDurMagBaseCostMult`,
/// `fMagicCostScale`, `fMagicRangeTargetCostMult`.
const AREA_BASE_COST_MULT: u32 = 0x011d_195c;
const DUR_MAG_BASE_COST_MULT: u32 = 0x011d_1998;
const COST_SCALE: u32 = 0x011d_198c;
const RANGE_TARGET_COST_MULT: u32 = 0x011d_1950;
/// The two-float function applied to the magnitude term (`00404010`,
/// cdecl: value, 1.0), result in `ST0`.
const MAGNITUDE_TERM: u32 = 0x0040_4010;
/// The two-float function applied to the duration (`004dd130`, cdecl:
/// duration, `fMagicCostScale`), result in `ST0`.
const DURATION_TERM: u32 = 0x004d_d130;

fn setting(e: &mut Engine, setting: u32) -> u32 {
    let p = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.u32(p)
}

// Translated from 006490b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicFormulas::GetMagicEffectCost` (Xbox PDB, cdecl): `range factor *
/// (00404010(magnitude * fMagicAreaBaseCostMult, 1.0) * (base cost *
/// fMagicDurMagBaseCostMult * area) * duration factor)`, where an area of
/// 0 counts as 1, the duration factor is `004dd130(duration,
/// fMagicCostScale)` (1.0 for 0 duration) and the range factor is
/// `fMagicRangeTargetCostMult` with `target` set (else 1.0). Each step is
/// the exe's x87 operation and `float` store; the result is in `ST0`.
pub fn magic_formulas_get_magic_effect_cost(
    e: &mut Engine,
    base_cost: f32,
    magnitude: i32,
    area: i32,
    duration: i32,
    target: u8,
) -> f64 {
    let one = 1.0f32.to_bits();
    let mult = setting(e, AREA_BASE_COST_MULT);
    let scaled = x87_store32(x87_mul(f64::from(magnitude), x87_m32(mult)));
    let magnitude_term = x87_store32(e.call(MAGNITUDE_TERM, &args![scaled, one]).f64());
    let area_factor = if area == 0 {
        one
    } else {
        x87_store32(f64::from(area))
    };
    let mult = setting(e, DUR_MAG_BASE_COST_MULT);
    let base = x87_store32(x87_mul(
        x87_mul(x87_m32(base_cost.to_bits()), x87_m32(mult)),
        x87_m32(area_factor),
    ));
    let duration_factor = if duration == 0 {
        one
    } else {
        let scale = x87_quiet_bits(setting(e, COST_SCALE));
        let duration = x87_store32(f64::from(duration));
        x87_store32(e.call(DURATION_TERM, &args![duration, scale]).f64())
    };
    let duration_factor = x87_quiet_bits(duration_factor);
    let cost = x87_store32(x87_mul(
        x87_mul(x87_m32(magnitude_term), x87_m32(base)),
        x87_m32(duration_factor),
    ));
    let range_factor = if target != 0 {
        x87_quiet_bits(setting(e, RANGE_TARGET_COST_MULT))
    } else {
        one
    };
    let result = x87_store32(x87_mul(x87_m32(range_factor), x87_m32(cost)));
    x87_load32(result)
}

/// `100.0` (`double`).
const HUNDRED_DOUBLE: u32 = 0x0101_7a40;
/// `12.0` and `4.0` (`float`).
const TWELVE_FLOAT: u32 = 0x0101_8f7c;
const FOUR_FLOAT: u32 = 0x0101_e570;
/// The settings the formulas below read (their values at the address
/// `00403e20` gives), by address.
const SETTING_011D16E0: u32 = 0x011d_16e0;
const SETTING_011D1794: u32 = 0x011d_1794;
const SETTING_011D158C: u32 = 0x011d_158c;
const SETTING_011D1B98: u32 = 0x011d_1b98;
/// The four thresholds `006491d0` compares with, in order.
const SETTING_011D19C0: u32 = 0x011d_19c0;
const SETTING_011D1CC4: u32 = 0x011d_1cc4;
const SETTING_011D1D0C: u32 = 0x011d_1d0c;
const SETTING_011D1B80: u32 = 0x011d_1b80;
const SETTING_011D1CE8: u32 = 0x011d_1ce8;
const SETTING_011D1BB0: u32 = 0x011d_1bb0;
const SETTING_011D1CD0: u32 = 0x011d_1cd0;
const SETTING_011D1938: u32 = 0x011d_1938;
const SETTING_011D1B68: u32 = 0x011d_1b68;
const SETTING_011D164C: u32 = 0x011d_164c;
const SETTING_011D1764: u32 = 0x011d_1764;
const SETTING_011D1704: u32 = 0x011d_1704;
/// Thiscall on the object at `011d1a14` (`0043d4d0`, `modelloader.cpp`
/// range): the address of an integer `006494d0` compares with.
const INTEGER_ADDRESS_0043D4D0: u32 = 0x0043_d4d0;
const OBJECT_011D1A14: u32 = 0x011d_1a14;

/// The address of a setting's value (`00403e20`).
fn setting_address(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_VALUE, &args![setting]).u32()
}

// Translated from 00649180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicFormulas::AdjustMagicSpellCost` (engine map; cdecl, three words, a
/// float in `ST0`): `cost * (setting 011d1794 * (1 - skill / 100) + setting
/// 011d16e0)`, each `float` store as the exe makes it (`1 - skill / 100`
/// kept as a `double`). The third word is not read.
pub fn magic_formulas_adjust_magic_spell_cost(
    e: &mut Engine,
    cost: f32,
    skill: i32,
    _unused_2: u32,
) -> f64 {
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    let fraction = x87_store32(x87_div(f64::from(skill), hundred));
    let base = setting_address(e, SETTING_011D16E0);
    let remaining = x87_sub(1.0, x87_load32(fraction));
    let mult = setting_address(e, SETTING_011D1794);
    let factor = x87_mul(x87_load32(e.mem.u32(mult)), remaining);
    let factor = x87_store32(x87_add(factor, x87_m32(e.mem.u32(base))));
    let result = x87_store32(x87_mul(x87_load32(cost.to_bits()), x87_m32(factor)));
    x87_load32(result)
}

// Translated from 006491d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one float): `value * setting 011d1b98
/// + setting 011d158c` (stored as a `float`) placed among four thresholds:
/// 0 when it is below setting `011d19c0`, else 1 below `011d1cc4`, 2 below
/// `011d1d0c`, 3 below `011d1b80`, else 4 (a NaN falls through to 4).
pub fn fn_006491d0(e: &mut Engine, value: f32) -> u32 {
    let base = setting_address(e, SETTING_011D158C);
    let mult = setting_address(e, SETTING_011D1B98);
    let scaled = x87_mul(x87_load32(e.mem.u32(mult)), x87_m32(value.to_bits()));
    let scaled = x87_store32(x87_add(scaled, x87_m32(e.mem.u32(base))));
    let thresholds = [
        SETTING_011D19C0,
        SETTING_011D1CC4,
        SETTING_011D1D0C,
        SETTING_011D1B80,
    ];
    for (category, threshold) in (0u32..).zip(thresholds) {
        let limit = setting_address(e, threshold);
        // `FCOMPP; TEST AH,0x41; JNZ`: this category when the threshold is
        // above the value (ordered).
        if x87_load32(e.mem.u32(limit)) > x87_load32(scaled) {
            return category;
        }
    }
    4
}

// Translated from 006492a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two floats, a float in `ST0`): 0.0
/// unless both are below 100 (or NaN); otherwise, with `a = first / 100`
/// and `b = second / 100` stored as `float`s, `1 - ((1 - a) * b + a)`.
pub fn fn_006492a0(e: &mut Engine, first: f32, second: f32) -> f64 {
    let hundred: f64 = e.global(HUNDRED_DOUBLE);
    // `FCOMP; TEST AH,1`: C0 is set below 100 and for a NaN.
    if !(x87_load32(first.to_bits()) >= hundred) && !(x87_load32(second.to_bits()) >= hundred) {
        let a = x87_store32(x87_div(x87_load32(first.to_bits()), hundred));
        let b = x87_store32(x87_div(x87_load32(second.to_bits()), hundred));
        let value = x87_sub(1.0, x87_load32(a));
        let value = x87_mul(value, x87_m32(b));
        let value = x87_add(value, x87_m32(a));
        let result = x87_store32(x87_sub(1.0, value));
        x87_load32(result)
    } else {
        0.0
    }
}

// Translated from 00649300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, four words, a float in `ST0`): with
/// `fall = 1 - (distance / range)^2` for a positive `range` (1.0
/// otherwise; each step stored as a `float`), the result is `setting
/// 011d1bb0 + setting 011d1ce8 * fall * base - count * setting 011d1cd0`
/// (the middle term kept as a `double`, the `count` converted to a `float`
/// first).
pub fn fn_00649300(e: &mut Engine, base: i32, range: i32, distance: f32, count: i32) -> f64 {
    let fall = if range > 0 {
        let ratio = x87_store32(x87_div(x87_m32(distance.to_bits()), f64::from(range)));
        let squared = x87_store32(x87_mul(x87_load32(ratio), x87_m32(ratio)));
        x87_store32(x87_sub(1.0, x87_load32(squared)))
    } else {
        1.0f32.to_bits()
    };
    let mult = setting_address(e, SETTING_011D1CE8);
    let term = x87_mul(x87_load32(e.mem.u32(mult)), x87_m32(fall));
    let term = x87_mul(term, f64::from(base));
    let offset = setting_address(e, SETTING_011D1BB0);
    let value = x87_store32(x87_add(x87_load32(e.mem.u32(offset)), term));
    let count = x87_store32(f64::from(count));
    let per = setting_address(e, SETTING_011D1CD0);
    let less = x87_mul(x87_load32(count), x87_m32(e.mem.u32(per)));
    let result = x87_store32(x87_sub(x87_m32(value), less));
    x87_load32(result)
}

// Translated from 00649390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, seven words, a float in `ST0`): 0.0
/// when `limit_a` is above `value` or `limit_b` below it (ordered). Otherwise
/// the falloff `f = 1 - 006494b0((12 - value) / (12 - limit), 4.0)`, where
/// `limit` is `limit_a` unless `value` is above 12 (then `limit_b`), and the
/// result is `setting 011d1764 * scale * f * (setting 011d1938 + (1 -
/// setting 011d1938) * weight) * (setting 011d164c when `use_setting_b`
/// else 1.0) * (1.0 when `skip_setting_a` else setting 011d1b68)`, each
/// step as the exe stores it.
#[allow(clippy::too_many_arguments)]
pub fn fn_00649390(
    e: &mut Engine,
    scale: f32,
    value: f32,
    limit_a: f32,
    limit_b: f32,
    skip_setting_a: u8,
    weight: f32,
    use_setting_b: u8,
) -> f64 {
    let value_x = x87_load32(value.to_bits());
    // `FCOMPP; TEST AH,0x41; JZ`: zero when `limit_a` is above `value`.
    if x87_load32(limit_a.to_bits()) > value_x {
        return 0.0;
    }
    // `FCOMPP; TEST AH,5; JP`: zero when `limit_b` is below `value`.
    if x87_load32(limit_b.to_bits()) < value_x {
        return 0.0;
    }
    let twelve = x87_store32(x87_load32(e.mem.u32(TWELVE_FLOAT)));
    // `TEST AH,5; JP`: the `limit_a` form unless 12 is below `value`.
    let limit = if x87_load32(twelve) < value_x {
        limit_b
    } else {
        limit_a
    };
    let numerator = x87_sub(x87_load32(twelve), x87_m32(value.to_bits()));
    let denominator = x87_sub(x87_load32(twelve), x87_m32(limit.to_bits()));
    let ratio = x87_store32(x87_div(numerator, denominator));
    let four = x87_quiet_bits(e.mem.u32(FOUR_FLOAT));
    let curve = fn_006494b0(
        e,
        f32::from_bits(x87_quiet_bits(ratio)),
        f32::from_bits(four),
    );
    let falloff = x87_store32(x87_sub(1.0, curve));
    let floor = setting_address(e, SETTING_011D1938);
    let term = x87_sub(1.0, x87_load32(e.mem.u32(floor)));
    let term = x87_mul(term, x87_m32(weight.to_bits()));
    let floor = setting_address(e, SETTING_011D1938);
    let weighted = x87_store32(x87_add(x87_load32(e.mem.u32(floor)), term));
    let factor_a = if skip_setting_a == 0 {
        let a = setting_address(e, SETTING_011D1B68);
        x87_store32(x87_load32(e.mem.u32(a)))
    } else {
        1.0f32.to_bits()
    };
    let factor_b = if use_setting_b != 0 {
        let b = setting_address(e, SETTING_011D164C);
        x87_store32(x87_load32(e.mem.u32(b)))
    } else {
        1.0f32.to_bits()
    };
    let mult = setting_address(e, SETTING_011D1764);
    let result = x87_mul(x87_load32(e.mem.u32(mult)), x87_m32(scale.to_bits()));
    let result = x87_mul(result, x87_m32(falloff));
    let result = x87_mul(result, x87_m32(weighted));
    let result = x87_mul(result, x87_m32(factor_b));
    let result = x87_mul(result, x87_m32(factor_a));
    x87_load32(x87_store32(result))
}

// Translated from 006494b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two floats, a float in `ST0`): passes
/// both (through `FLD`/`FSTP`) to `004dd130` and answers its result.
pub fn fn_006494b0(e: &mut Engine, first: f32, second: f32) -> f64 {
    e.call(
        DURATION_TERM,
        &args![
            x87_quiet_bits(first.to_bits()),
            x87_quiet_bits(second.to_bits())
        ],
    )
    .f64()
}

// Translated from 006494d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicFormulas::CheckTargetLevelMagnitude` (engine map; cdecl, two
/// words): with `scaled = magnitude * setting 011d1704` stored as a
/// `float`, true when it is not below the integer `0043d4d0` gives for
/// `011d1a14`; otherwise whether it is not below `level` (a NaN is below).
pub fn magic_formulas_check_target_level_magnitude(
    e: &mut Engine,
    level: i32,
    magnitude: f32,
) -> bool {
    let mult = setting_address(e, SETTING_011D1704);
    let scaled = x87_store32(x87_mul(
        x87_load32(magnitude.to_bits()),
        x87_m32(e.mem.u32(mult)),
    ));
    let cap = e
        .call(INTEGER_ADDRESS_0043D4D0, &args![OBJECT_011D1A14])
        .u32();
    let cap = e.mem.u32(cap) as i32;
    // `FCOMPP; TEST AH,1`: C0 is set below (and for a NaN).
    if x87_load32(scaled) >= f64::from(cap) {
        return true;
    }
    x87_load32(scaled) >= f64::from(level)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0064_90b0,
            magic_formulas_get_magic_effect_cost(f32, i32, i32, i32, u8) -> f64
        ),
        entry!(
            0x0064_9180,
            magic_formulas_adjust_magic_spell_cost(f32, i32, u32) -> f64
        ),
        entry!(0x0064_91d0, fn_006491d0(f32) -> u32),
        entry!(0x0064_92a0, fn_006492a0(f32, f32) -> f64),
        entry!(0x0064_9300, fn_00649300(i32, i32, f32, i32) -> f64),
        entry!(
            0x0064_9390,
            fn_00649390(f32, f32, f32, f32, u8, f32, u8) -> f64
        ),
        entry!(0x0064_94b0, fn_006494b0(f32, f32) -> f64),
        entry!(
            0x0064_94d0,
            magic_formulas_check_target_level_magnitude(i32, f32) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Settings with their values at `setting + 4` (as `00403e20` gives
    /// them), and doubles that multiply their two floats.
    fn cost_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_1000, 0x1000);
        e.set_global(AREA_BASE_COST_MULT + 4, 0.5f32);
        e.set_global(DUR_MAG_BASE_COST_MULT + 4, 2.0f32);
        e.set_global(COST_SCALE + 4, 3.0f32);
        e.set_global(RANGE_TARGET_COST_MULT + 4, 1.5f32);
        e.register(SETTING_VALUE, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        for f in [MAGNITUDE_TERM, DURATION_TERM] {
            e.register(f, |_, a| Ret {
                st0: f64::from(f32::from_bits(a[0])) * f64::from(f32::from_bits(a[1])),
                ..Ret::default()
            });
        }
        e
    }

    #[test]
    fn cost_multiplies_the_terms() {
        let mut e = cost_engine();
        e.call_log = Some(vec![]);
        // (10 * 0.5) * 1.0 = 5; 4 * 2 * 3 = 24; 2 * 3 = 6; 5 * 24 * 6 = 720;
        // times 1.5 for a target.
        let words = args![4.0f32, 10i32, 3i32, 2i32, 1u32];
        let cost = e.call(0x0064_90b0, &words).f64();
        assert_eq!(cost, 1080.0);
        assert_eq!(
            calls_to(&e, MAGNITUDE_TERM),
            vec![vec![5.0f32.to_bits(), 1.0f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, DURATION_TERM),
            vec![vec![2.0f32.to_bits(), 3.0f32.to_bits()]]
        );
    }

    #[test]
    fn zero_area_and_duration_count_as_one() {
        let mut e = cost_engine();
        e.call_log = Some(vec![]);
        let cost = magic_formulas_get_magic_effect_cost(&mut e, 4.0, 10, 0, 0, 0);
        // 5 * (4 * 2 * 1) * 1 * 1 = 40.
        assert_eq!(cost, 40.0);
        assert!(calls_to(&e, DURATION_TERM).is_empty());
    }
}

#[cfg(test)]
mod formula_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// Settings with their values at `setting + 4`, the constants mapped.
    fn engine(settings: &[(u32, f32)]) -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_1000, 0x1000);
        e.map(0x0101_7000, 0x8000);
        e.set_global(HUNDRED_DOUBLE, 100.0f64);
        e.set_global(TWELVE_FLOAT, 12.0f32);
        e.set_global(FOUR_FLOAT, 4.0f32);
        for (setting, value) in settings {
            e.set_global(setting + 4, *value);
        }
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e
    }

    #[test]
    fn spell_cost_adjusted_by_skill() {
        let mut e = engine(&[(SETTING_011D16E0, 0.5), (SETTING_011D1794, 2.0)]);
        // 10 * (2 * (1 - 0.25) + 0.5) = 20.
        let words = args![10.0f32, 25i32, 99u32];
        assert_eq!(e.call(0x0064_9180, &words).f64(), 20.0);
        // A skill of 100 leaves the base: 4 * 0.5.
        assert_eq!(
            magic_formulas_adjust_magic_spell_cost(&mut e, 4.0, 100, 0),
            2.0
        );
    }

    #[test]
    fn category_by_thresholds() {
        let mut e = engine(&[
            (SETTING_011D158C, 1.0),
            (SETTING_011D1B98, 2.0),
            (SETTING_011D19C0, 5.0),
            (SETTING_011D1CC4, 10.0),
            (SETTING_011D1D0C, 20.0),
            (SETTING_011D1B80, 40.0),
        ]);
        // value * 2 + 1 against 5, 10, 20, 40.
        for (value, category) in [
            (1.0f32, 0u32),
            (2.0, 1),
            (4.5, 2),
            (9.5, 3),
            (19.5, 4),
            (30.0, 4),
        ] {
            assert_eq!(
                e.call(0x0064_91d0, &args![value]).u32(),
                category,
                "{value}"
            );
        }
        assert_eq!(fn_006491d0(&mut e, f32::NAN), 4);
    }

    #[test]
    fn two_chances_combined() {
        let mut e = engine(&[]);
        // a = 0.5, b = 0.5: 1 - (0.5 * 0.5 + 0.5) = 0.25.
        assert_eq!(e.call(0x0064_92a0, &args![50.0f32, 50.0f32]).f64(), 0.25);
        assert_eq!(fn_006492a0(&mut e, 100.0, 50.0), 0.0);
        assert_eq!(fn_006492a0(&mut e, 50.0, 100.0), 0.0);
        assert!(fn_006492a0(&mut e, f32::NAN, 50.0).is_nan());
    }

    #[test]
    fn falloff_with_distance_and_count() {
        let mut e = engine(&[
            (SETTING_011D1CE8, 2.0),
            (SETTING_011D1BB0, 1.0),
            (SETTING_011D1CD0, 0.5),
        ]);
        // fall = 1 - (5 / 10)^2 = 0.75; 1 + 2 * 0.75 * 4 - 3 * 0.5 = 5.5.
        let words = args![4i32, 10i32, 5.0f32, 3i32];
        assert_eq!(e.call(0x0064_9300, &words).f64(), 5.5);
        // No range: fall is 1.0; 1 + 2 * 4 - 0 = 9.
        assert_eq!(fn_00649300(&mut e, 4, 0, 5.0, 0), 9.0);
        assert_eq!(fn_00649300(&mut e, 4, -1, 5.0, 2), 8.0);
    }

    #[test]
    fn falloff_between_limits() {
        let mut e = engine(&[
            (SETTING_011D1938, 0.5),
            (SETTING_011D1B68, 3.0),
            (SETTING_011D164C, 5.0),
            (SETTING_011D1764, 2.0),
        ]);
        // 004dd130 doubled as a product.
        e.register(DURATION_TERM, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) * f64::from(f32::from_bits(a[1])),
            ..Ret::default()
        });
        // value 8, limit_a 4: (12 - 8) / (12 - 4) = 0.5; f = 1 - 0.5 * 4 = -1;
        // weighted = 0.5 + 0.5 * 1 = 1; 2 * 3 * -1 * 1 * 1 * 3 = -18.
        e.call_log = Some(vec![]);
        let words = args![3.0f32, 8.0f32, 4.0f32, 9.0f32, 0u32, 1.0f32, 0u32];
        assert_eq!(e.call(0x0064_9390, &words).f64(), -18.0);
        assert_eq!(
            calls_to(&e, DURATION_TERM),
            vec![vec![0.5f32.to_bits(), 4.0f32.to_bits()]]
        );
        // Value above 12 uses limit_b: (12 - 13) / (12 - 14) = 0.5 again;
        // skip setting a, use setting b: 2 * 3 * -1 * 1 * 5 * 1 = -30.
        assert_eq!(fn_00649390(&mut e, 3.0, 13.0, 4.0, 14.0, 1, 1.0, 1), -30.0);
        // limit_a above the value, or limit_b below it: zero.
        assert_eq!(fn_00649390(&mut e, 3.0, 8.0, 9.0, 9.0, 0, 1.0, 0), 0.0);
        assert_eq!(fn_00649390(&mut e, 3.0, 8.0, 4.0, 7.0, 0, 1.0, 0), 0.0);
    }

    #[test]
    fn curve_passes_both_floats() {
        let mut e = engine(&[]);
        e.register(DURATION_TERM, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) - f64::from(f32::from_bits(a[1])),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_94b0, &args![5.0f32, 2.0f32]).f64(), 3.0);
        fn_006494b0(&mut e, f32::from_bits(0x7f80_0001), 1.0);
        let passed = calls_to(&e, DURATION_TERM);
        assert_eq!(passed[1][0], 0x7fc0_0001);
    }

    #[test]
    fn target_level_against_magnitude() {
        let mut e = engine(&[(SETTING_011D1704, 2.0)]);
        let cap = e.mem.alloc(4);
        e.mem.set_u32(cap, 30);
        e.register_double(INTEGER_ADDRESS_0043D4D0, move |_, _| cap.into_ret());
        // scaled = magnitude * 2.
        assert!(e.call(0x0064_94d0, &args![100i32, 15.0f32]).bool());
        assert!(magic_formulas_check_target_level_magnitude(&mut e, 10, 5.0));
        assert!(!magic_formulas_check_target_level_magnitude(
            &mut e, 11, 5.0
        ));
        assert!(!magic_formulas_check_target_level_magnitude(
            &mut e,
            0,
            f32::NAN
        ));
    }
}
