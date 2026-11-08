//! xNVSE's math commands (`Commands_Math.cpp` (xNVSE)), batch X19 of ADR-0016's build order.
//!
//! xNVSE reads every number into a `float` (or a `UInt32`) and calls the C++ overloads for
//! `float` (`expf`, `sinf`, `powf`, ...; `floor(x/n)` is `floorf`), so the results are single
//! precision widened to the result double. The real game with xNVSE 6.4.9 gave the values the
//! tests pin (`measure/extenders/real-results/batch1.txt`, each printed times 2^40, which is
//! exact for a float).

use super::Read;
use crate::args::f64_to_i32;
use crate::table::{Command, Param, Returns};
use crate::{handled, CommandArgs, Nvse};
use engine_core::prelude::*;

const P_FLOAT: Param = Param {
    name: "float",
    kind: 0x02,
    optional: false,
};
const P_INT: Param = Param {
    name: "int",
    kind: 0x01,
    optional: false,
};
/// `kParams_OneFloat`, `kParams_TwoFloats`, `kParams_OneInt`, `kParams_TwoInts` (xNVSE).
const ONE_FLOAT: &[Param] = &[P_FLOAT];
const TWO_FLOATS: &[Param] = &[P_FLOAT, P_FLOAT];
const ONE_INT: &[Param] = &[P_INT];
const TWO_INTS: &[Param] = &[P_INT, P_INT];
/// `kParams_TwoInts_OneOptInt` (xNVSE).
const TWO_INTS_ONE_OPT_INT: &[Param] = &[
    P_INT,
    P_INT,
    Param {
        name: "int",
        kind: 0x01,
        optional: true,
    },
];
/// `kParams_Fmod` (xNVSE).
const FMOD_PARAMS: &[Param] = &[
    Param {
        name: "x",
        kind: 0x02,
        optional: false,
    },
    Param {
        name: "n",
        kind: 0x02,
        optional: false,
    },
    Param {
        name: "offset",
        kind: 0x02,
        optional: true,
    },
];

/// A plain command: no reference, no condition eval, a number.
const fn cmd(
    opcode: u32,
    name: &'static str,
    short: &'static str,
    help: &'static str,
    params: &'static [Param],
    execute: AbiFn,
) -> Command {
    Command {
        opcode,
        name,
        short,
        help,
        needs_ref: false,
        params,
        execute,
        eval: None,
        returns: Returns::Default,
    }
}

// `DEFINE_COMMAND` puts its description through `#description`, so a description written in
// quotes keeps them (xNVSE's trig commands): the help texts below are as the records hold them.
pub static COMMANDS: &[Command] = &[
    cmd(
        0x14c9,
        "Exp",
        "",
        "calculates the exponential of a value",
        ONE_FLOAT,
        exp,
    ),
    cmd(
        0x14ca,
        "Log10",
        "",
        "calculates the base 10 log of a value",
        ONE_FLOAT,
        log10,
    ),
    cmd(
        0x14cb,
        "Floor",
        "flr",
        "Returns the nearest whole number that is less than a float",
        ONE_FLOAT,
        floor,
    ),
    cmd(
        0x14cc,
        "Ceil",
        "",
        "Returns the nearest whole number that is greater than a float",
        ONE_FLOAT,
        ceil,
    ),
    cmd(
        0x14cd,
        "LeftShift",
        "",
        "Shifts a 32-bit integer left the specified number of bits",
        TWO_INTS,
        left_shift,
    ),
    cmd(
        0x14ce,
        "RightShift",
        "",
        "Shifts a 32-bit integer right the specified number of bits",
        TWO_INTS,
        right_shift,
    ),
    cmd(
        0x14cf,
        "LogicalAnd",
        "",
        "Performs a logical AND between two 32-bit integers.",
        TWO_INTS,
        logical_and,
    ),
    cmd(
        0x14d0,
        "LogicalOr",
        "",
        "Performs a logical OR between two 32-bit integers.",
        TWO_INTS,
        logical_or,
    ),
    cmd(
        0x14d1,
        "LogicalXor",
        "",
        "Performs a logical XOR between two 32-bit integers.",
        TWO_INTS,
        logical_xor,
    ),
    cmd(
        0x14d2,
        "LogicalNot",
        "",
        "Performs a logical NOT on a 32-bit integer.",
        ONE_INT,
        logical_not,
    ),
    cmd(
        0x14d3,
        "Pow",
        "",
        "Calculates one value raised to the power of another",
        TWO_FLOATS,
        pow,
    ),
    cmd(
        0x14d4,
        "Fmod",
        "",
        "returns the result of a floating point modulous of the passed floats",
        FMOD_PARAMS,
        fmod,
    ),
    cmd(
        0x14d5,
        "Rand",
        "r",
        "Returns a float between min and max",
        TWO_FLOATS,
        rand,
    ),
    cmd(
        0x14f0,
        "ClearBit",
        "",
        "Clear a single bit specified by lsb-first index",
        TWO_INTS,
        clear_bit,
    ),
    cmd(
        0x14f1,
        "SetBit",
        "",
        "Sets a single bit specified by lsb-first index",
        TWO_INTS_ONE_OPT_INT,
        set_bit,
    ),
    cmd(
        0x1607,
        "GetBit",
        "",
        "Gets a single bit specified by lsb-first index",
        TWO_INTS,
        get_bit,
    ),
    cmd(
        0x160a,
        "ATan2",
        "",
        "\"Calculates the arccos of two values in radians\"",
        TWO_FLOATS,
        atan2,
    ),
    cmd(
        0x160b,
        "Sinh",
        "",
        "\"Calculates the hyperbolic sin of a value in radians\"",
        ONE_FLOAT,
        sinh,
    ),
    cmd(
        0x160c,
        "Cosh",
        "",
        "\"Calculates the hyperbolic cos of a value in radians\"",
        ONE_FLOAT,
        cosh,
    ),
    cmd(
        0x160d,
        "Tanh",
        "",
        "\"Calculates the hyperbolic tan of a value in radians\"",
        ONE_FLOAT,
        tanh,
    ),
    cmd(
        0x160e,
        "dSin",
        "",
        "\"Calculates the sin of a value in degrees\"",
        ONE_FLOAT,
        d_sin,
    ),
    cmd(
        0x160f,
        "dCos",
        "",
        "\"Calculates the cos of a value in degrees\"",
        ONE_FLOAT,
        d_cos,
    ),
    cmd(
        0x1610,
        "dTan",
        "",
        "\"Calculates the tan of a value in degrees\"",
        ONE_FLOAT,
        d_tan,
    ),
    cmd(
        0x1611,
        "dASin",
        "",
        "\"Calculates the arcsin of a value in degrees\"",
        ONE_FLOAT,
        d_asin,
    ),
    cmd(
        0x1612,
        "dACos",
        "",
        "\"Calculates the arccos of a value in degrees\"",
        ONE_FLOAT,
        d_acos,
    ),
    cmd(
        0x1613,
        "dATan",
        "",
        "\"Calculates the arctan of a value in degrees\"",
        ONE_FLOAT,
        d_atan,
    ),
    cmd(
        0x1614,
        "dATan2",
        "",
        "\"Calculates the arctan of two values in degrees\"",
        TWO_FLOATS,
        d_atan2,
    ),
    cmd(
        0x1615,
        "dSinh",
        "",
        "\"Calculates the hyperbolic sin of a value in degrees\"",
        ONE_FLOAT,
        d_sinh,
    ),
    cmd(
        0x1616,
        "dCosh",
        "",
        "\"Calculates the hyperbolic cos of a value in degrees\"",
        ONE_FLOAT,
        d_cosh,
    ),
    cmd(
        0x1617,
        "dTanh",
        "",
        "\"Calculates the hyperbolic tan of a value in degrees\"",
        ONE_FLOAT,
        d_tanh,
    ),
];

/// `DEGTORAD` (xNVSE), a `float` (the source's literal).
#[allow(clippy::excessive_precision)]
const DEG_TO_RAD: f32 = 0.017_453_292_52;

/// A one-float command (`*result = 0`, then `f(arg)` when the read succeeds).
fn one_float(e: &mut Engine, w: &[u32], f: fn(f32) -> f32) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if read.ok {
        let v = f(read.float(0, 0.0));
        a.set_result(e, f64::from(v));
    }
    handled(true)
}

/// A two-float command.
fn two_floats(e: &mut Engine, w: &[u32], f: fn(f32, f32) -> f32) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if read.ok {
        let v = f(read.float(0, 0.0), read.float(1, 0.0));
        a.set_result(e, f64::from(v));
    }
    handled(true)
}

/// A two-integer command (`UInt32` operands, a `UInt32` result).
fn two_ints(e: &mut Engine, w: &[u32], f: fn(u32, u32) -> u32) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if read.ok {
        let v = f(read.int(0, 0), read.int(1, 0));
        a.set_result(e, f64::from(v));
    }
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Math.cpp: Cmd_Exp_Execute.
fn exp(e: &mut Engine, w: &[u32]) -> Ret {
    one_float(e, w, f32::exp)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Math.cpp: Cmd_Log10_Execute.
fn log10(e: &mut Engine, w: &[u32]) -> Ret {
    one_float(e, w, f32::log10)
}

/// MSVC's SSE2 `cvttss2si`: truncation; `0x80000000` for NaN and out of range.
fn cvtt(v: f32) -> i32 {
    f64_to_i32(f64::from(v))
}

/// `cvtss2si` under a rounding mode: `round` gives the whole number, then the range check.
fn cvt_rounded(v: f32, round: fn(f64) -> f64) -> i32 {
    f64_to_i32(round(f64::from(v)))
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), utility.cpp: ifloor.
/// A non-negative sign bit truncates (`cvttss2si`); a negative one converts with the rounding
/// mode set to down (MXCSR `0x3FA0`).
pub fn ifloor(v: f32) -> i32 {
    if v.to_bits() & 0x8000_0000 == 0 {
        cvtt(v)
    } else {
        cvt_rounded(v, f64::floor)
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), utility.cpp: iceil.
/// A negative sign bit truncates; otherwise the rounding mode is up (MXCSR `0x5FA0`).
pub fn iceil(v: f32) -> i32 {
    if v.to_bits() & 0x8000_0000 != 0 {
        cvtt(v)
    } else {
        cvt_rounded(v, f64::ceil)
    }
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Math.cpp: Cmd_Floor_Execute.
fn floor(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = Read::of(e, &a);
    let v = if read.ok {
        f64::from(ifloor(read.float(0, 0.0)))
    } else {
        0.0
    };
    a.set_result(e, v);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Math.cpp: Cmd_Ceil_Execute.
fn ceil(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    let read = Read::of(e, &a);
    let v = if read.ok {
        f64::from(iceil(read.float(0, 0.0)))
    } else {
        0.0
    };
    a.set_result(e, v);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Math.cpp:
// Cmd_LeftShift_Execute, Cmd_RightShift_Execute.
fn left_shift(e: &mut Engine, w: &[u32]) -> Ret {
    two_ints(e, w, |l, r| if r >= 32 { 0 } else { l << r })
}

fn right_shift(e: &mut Engine, w: &[u32]) -> Ret {
    two_ints(e, w, |l, r| if r >= 32 { 0 } else { l >> r })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Math.cpp: Cmd_LogicalAnd_Execute,
// Cmd_LogicalOr_Execute, Cmd_LogicalXor_Execute, Cmd_LogicalNot_Execute.
fn logical_and(e: &mut Engine, w: &[u32]) -> Ret {
    two_ints(e, w, |l, r| l & r)
}

fn logical_or(e: &mut Engine, w: &[u32]) -> Ret {
    two_ints(e, w, |l, r| l | r)
}

fn logical_xor(e: &mut Engine, w: &[u32]) -> Ret {
    two_ints(e, w, |l, r| l ^ r)
}

fn logical_not(e: &mut Engine, w: &[u32]) -> Ret {
    two_ints(e, w, |l, _| !l)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Math.cpp: Cmd_Pow_Execute.
fn pow(e: &mut Engine, w: &[u32]) -> Ret {
    two_floats(e, w, f32::powf)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Math.cpp: Cmd_Fmod_Execute.
/// The read's result is not looked at: whatever it stored is used, the rest stays 0.
fn fmod(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    let (x, n, offset) = (read.float(0, 0.0), read.float(1, 0.0), read.float(2, 0.0));
    let mut answer = x - n * (x / n).floor();
    if offset != 0.0 {
        let bump = n * (offset / n).floor();
        let bound = offset - bump;
        answer += bump;
        let bigger = n > 0.0;
        if (bigger && answer < bound) || (!bigger && answer > bound) {
            answer += n;
        }
    }
    a.set_result(e, f64::from(answer));
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Math.cpp: Cmd_Rand_Execute.
/// `min + genrand_real2() * (max - min)`, the range a `float`, the rest in double.
fn rand(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if !read.ok {
        return handled(true);
    }
    let (mut min, mut max) = (read.float(0, 0.0), read.float(1, 0.0));
    if max < min {
        std::mem::swap(&mut min, &mut max);
    }
    let range = max - min;
    let value = e.ext::<Nvse>().mt.genrand_real2() * f64::from(range) + f64::from(min);
    a.set_result(e, value);
    handled(true)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Math.cpp: Cmd_SetBit_Execute,
// Cmd_ClearBit_Execute, Cmd_GetBit_Execute.
/// `1 << idx` is an x86 shift: the count is taken modulo 32.
fn set_bit(e: &mut Engine, w: &[u32]) -> Ret {
    let a = CommandArgs::new(w);
    a.set_result(e, 0.0);
    let read = Read::of(e, &a);
    if read.ok {
        let (data, idx, set) = (read.int(0, 0), read.int(1, 0), read.int(2, 1));
        let bit = 1u32.wrapping_shl(idx);
        let v = if set != 0 { data | bit } else { data & !bit };
        a.set_result(e, f64::from(v));
    }
    handled(true)
}

fn clear_bit(e: &mut Engine, w: &[u32]) -> Ret {
    two_ints(e, w, |data, idx| data & !1u32.wrapping_shl(idx))
}

fn get_bit(e: &mut Engine, w: &[u32]) -> Ret {
    two_ints(e, w, |data, idx| {
        (data & 1u32.wrapping_shl(idx)).wrapping_shr(idx)
    })
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Math.cpp: Cmd_ATan2_Execute,
// Cmd_Sinh_Execute, Cmd_Cosh_Execute, Cmd_Tanh_Execute.
fn atan2(e: &mut Engine, w: &[u32]) -> Ret {
    two_floats(e, w, f32::atan2)
}

fn sinh(e: &mut Engine, w: &[u32]) -> Ret {
    one_float(e, w, f32::sinh)
}

fn cosh(e: &mut Engine, w: &[u32]) -> Ret {
    one_float(e, w, f32::cosh)
}

fn tanh(e: &mut Engine, w: &[u32]) -> Ret {
    one_float(e, w, f32::tanh)
}

// Translated from xNVSE 6.4.9 (xNVSE/NVSE at 0ccd23a), Commands_Math.cpp: the degree commands
// (Cmd_dSin_Execute ... Cmd_dTanh_Execute): the argument times `DEGTORAD`, or the result
// divided by it, in `float`.
fn d_sin(e: &mut Engine, w: &[u32]) -> Ret {
    one_float(e, w, |v| (v * DEG_TO_RAD).sin())
}

fn d_cos(e: &mut Engine, w: &[u32]) -> Ret {
    one_float(e, w, |v| (v * DEG_TO_RAD).cos())
}

fn d_tan(e: &mut Engine, w: &[u32]) -> Ret {
    one_float(e, w, |v| (v * DEG_TO_RAD).tan())
}

fn d_asin(e: &mut Engine, w: &[u32]) -> Ret {
    one_float(e, w, |v| v.asin() / DEG_TO_RAD)
}

fn d_acos(e: &mut Engine, w: &[u32]) -> Ret {
    one_float(e, w, |v| v.acos() / DEG_TO_RAD)
}

fn d_atan(e: &mut Engine, w: &[u32]) -> Ret {
    one_float(e, w, |v| v.atan() / DEG_TO_RAD)
}

fn d_atan2(e: &mut Engine, w: &[u32]) -> Ret {
    two_floats(e, w, |y, x| y.atan2(x) / DEG_TO_RAD)
}

fn d_sinh(e: &mut Engine, w: &[u32]) -> Ret {
    one_float(e, w, |v| (v * DEG_TO_RAD).sinh())
}

fn d_cosh(e: &mut Engine, w: &[u32]) -> Ret {
    one_float(e, w, |v| (v * DEG_TO_RAD).cosh())
}

fn d_tanh(e: &mut Engine, w: &[u32]) -> Ret {
    one_float(e, w, |v| (v * DEG_TO_RAD).tanh())
}

#[cfg(test)]
mod tests {
    use super::super::test_support::*;

    fn f(e: &mut Engine, opcode: u32, args: &[Vec<u8>]) -> f64 {
        let (ok, v) = call(e, opcode, 0, args);
        assert_eq!(ok, 1);
        v
    }
    use engine_core::prelude::*;

    /// Each case as the real game with xNVSE 6.4.9 answered it (`a_math` in
    /// `real-results/batch1.txt`, the second number: the result times 2^40).
    #[test]
    fn the_results_match_the_real_game_bit_for_bit() {
        let mut e = engine_with_table();
        let cases: &[(&str, u32, Vec<Vec<u8>>, f64)] = &[
            ("Exp1.5", 0x14c9, vec![z(1.5)], 4927669141504.0),
            ("Exp-3", 0x14c9, vec![z(-3.0)], 54741458944.0),
            ("Log10_7", 0x14ca, vec![z(7.0)], 929195098112.0),
            ("Floor-2.5", 0x14cb, vec![z(-2.5)], -3298534883328.0),
            ("Floor2.9", 0x14cb, vec![z(2.9)], 2199023255552.0),
            ("Floor3e9", 0x14cb, vec![z(3e9)], -2361183241434822606848.0),
            ("Ceil2.1", 0x14cc, vec![z(2.1)], 3298534883328.0),
            ("Ceil-2.1", 0x14cc, vec![z(-2.1)], -2199023255552.0),
            ("Pow2_0.5", 0x14d3, vec![z(2.0), z(0.5)], 1554944229376.0),
            (
                "Pow10_20",
                0x14d3,
                vec![z(10.0), z(20.0)],
                109951164981117766915322462339072.0,
            ),
            ("Fmod7.5_2", 0x14d4, vec![z(7.5), z(2.0)], 1649267441664.0),
            ("Fmod-7.5_2", 0x14d4, vec![z(-7.5), z(2.0)], 549755813888.0),
            (
                "Fmod7.5_2_10",
                0x14d4,
                vec![z(7.5), z(2.0), z(10.0)],
                12644383719424.0,
            ),
            (
                "Fmod7.5_-2_3",
                0x14d4,
                vec![z(7.5), z(-2.0), z(3.0)],
                1649267441664.0,
            ),
            ("dSin30", 0x160e, vec![z(30.0)], 549755813888.0),
            ("dCos60", 0x160f, vec![z(60.0)], 549755781120.0),
            ("dTan45", 0x1610, vec![z(45.0)], 1099511627776.0),
            ("dTan90", 0x1610, vec![z(90.0)], -25153892546491973632.0),
            ("dASin0.5", 0x1611, vec![z(0.5)], 32985350930432.0),
            ("dACos0.5", 0x1612, vec![z(0.5)], 65970701860864.0),
            ("dATan1", 0x1613, vec![z(1.0)], 49478023249920.0),
            ("dATan2_1_2", 0x1614, vec![z(1.0), z(2.0)], 29208583602176.0),
            ("ATan2_1_2", 0x160a, vec![z(1.0), z(2.0)], 509785931776.0),
            ("Sinh0.5", 0x160b, vec![z(0.5)], 572950315008.0),
            ("Cosh0.5", 0x160c, vec![z(0.5)], 1239837835264.0),
            ("Tanh0.5", 0x160d, vec![z(0.5)], 508103196672.0),
            ("dSinh30", 0x1615, vec![z(30.0)], 602371260416.0),
            ("dCosh30", 0x1616, vec![z(30.0)], 1253705252864.0),
            ("dTanh30", 0x1617, vec![z(30.0)], 528285433856.0),
            (
                "LeftShift1_31",
                0x14cd,
                vec![n(1), n(31)],
                2361183241434822606848.0,
            ),
            (
                "LeftShift3_31",
                0x14cd,
                vec![n(3), n(31)],
                2361183241434822606848.0,
            ),
            ("LeftShift1_32", 0x14cd, vec![n(1), n(32)], 0.0),
            (
                "RightShift-1_4",
                0x14ce,
                vec![n(-1), n(4)],
                295147904079841198080.0,
            ),
            // The compiler stores 4000000000 as 0x7FFFFFFF (the real game's answer shows it).
            (
                "RightShift4e9_1",
                0x14ce,
                vec![n(0x7fff_ffff), n(1)],
                1180591619617899675648.0,
            ),
            ("And12_10", 0x14cf, vec![n(12), n(10)], 8796093022208.0),
            ("Or12_10", 0x14d0, vec![n(12), n(10)], 15393162788864.0),
            ("Xor12_10", 0x14d1, vec![n(12), n(10)], 6597069766656.0),
            ("Not0", 0x14d2, vec![n(0)], 4722366481770133585920.0),
            ("Not-1", 0x14d2, vec![n(-1)], 0.0),
            (
                "SetBit0_31",
                0x14f1,
                vec![n(0), n(31)],
                2361183241434822606848.0,
            ),
            (
                "SetBit7_1_0",
                0x14f1,
                vec![n(7), n(1), n(0)],
                5497558138880.0,
            ),
            (
                "ClearBit-1_0",
                0x14f0,
                vec![n(-1), n(0)],
                4722366480670621958144.0,
            ),
            ("GetBit5_2", 0x1607, vec![n(5), n(2)], 1099511627776.0),
            ("GetBit5_1", 0x1607, vec![n(5), n(1)], 0.0),
            ("GetBit-1_31", 0x1607, vec![n(-1), n(31)], 1099511627776.0),
        ];
        let mut wrong = Vec::new();
        for (label, opcode, args, real) in cases {
            let got = f(&mut e, *opcode, args);
            if got.to_bits() != scaled(*real).to_bits() {
                wrong.push(format!("{label}: port {got:e}, real {:e}", scaled(*real)));
            }
        }
        assert!(wrong.is_empty(), "{wrong:#?}");
    }

    /// The real game's NaN and infinity cases: `-inf` for log10 0, NaN for log10 -1, a negative
    /// base to a fractional power, `Fmod 5 0` and `dASin 2`.
    #[test]
    fn nan_and_infinity_cases() {
        let mut e = engine_with_table();
        assert_eq!(f(&mut e, 0x14ca, &[z(0.0)]), f64::NEG_INFINITY);
        assert!(f(&mut e, 0x14ca, &[z(-1.0)]).is_nan());
        assert!(f(&mut e, 0x14d3, &[z(-8.0), z(0.333)]).is_nan());
        assert!(f(&mut e, 0x14d4, &[z(5.0), z(0.0)]).is_nan());
        assert!(f(&mut e, 0x1611, &[z(2.0)]).is_nan());
    }

    /// `Rand` draws from the engine's generator: with the same seed the same numbers, inside the
    /// range, the bounds in either order.
    #[test]
    fn rand_draws_from_mt19937_in_the_range() {
        let mut e = engine_with_table();
        e.ext::<crate::Nvse>().mt.init_genrand(5489);
        let first = f(&mut e, 0x14d5, &[z(10.0), z(20.0)]);
        // 3499211612 / 2^32 * 10 + 10.
        assert_eq!(first, 3_499_211_612.0 / 4_294_967_296.0 * 10.0 + 10.0);
        let second = f(&mut e, 0x14d5, &[z(20.0), z(10.0)]);
        assert_eq!(second, 581_869_302.0 / 4_294_967_296.0 * 10.0 + 10.0);
    }

    /// `ifloor` and `iceil` at the edges: the sign bit chooses truncation.
    #[test]
    fn ifloor_and_iceil() {
        use super::{iceil, ifloor};
        assert_eq!(ifloor(-0.0), 0);
        assert_eq!(ifloor(-0.5), -1);
        assert_eq!(ifloor(f32::NAN), i32::MIN);
        assert_eq!(iceil(0.5), 1);
        assert_eq!(iceil(-0.5), 0);
        assert_eq!(iceil(3e9), i32::MIN);
    }
}
