//! `(unplaced)/static_init_irregular_i` (no source unit: the compiler's dynamic initializers),
//! subsystem `compiler: static init/exit`: 25 float initializers of FalloutNV.exe 1.4.0.525
//! that `research/initgen` refused and start-up skipped. Part 9 of the list. Decoded from
//! `read_memory` bytes (Ghidra made no function of them), every instruction checked:
//!
//! - 10 roots and a cosine: `PUSH ECX; FLD double ptr [a]; CALL _CIsqrt (00ec6040)` or
//!   `_CIcos (00ec9f70)` (the argument and the answer in `ST0`), the answer rounded through
//!   the local (`FSTP float ptr [ESP]; FLD float ptr [ESP]`), then either stored as it is, or
//!   divided into a double (`FDIVR double ptr [b]`), or multiplied by a double, divided into
//!   a double and multiplied by a double (`FMUL`, `FDIVR`, `FMUL`, all double memory
//!   operands), and stored as a float (`FSTP float ptr [g]`).
//! - 6 zeroed arrays: `FLDZ; MOV ECX,n; MOV EAX,base`, then a loop (`FST [EAX-8]; ADD EAX,0x10;
//!   SUB ECX,1; FST [EAX-0x14]; FST [EAX-0x10]; FST [EAX-0xc]; JNS`), `n + 1` rounds of four
//!   zero floats from `base - 8`; `FSTP ST0`.
//! - 9 runs of zero floats: `FLDZ` and `FST`s to ascending adjacent floats, the last `FSTP`.
//!
//! No frame, no `/GS` check. They are cdecl functions with no parameters; `_initterm` ignores
//! `EAX`.

#[allow(unused_imports)]
use crate::prelude::*;

/// `_CIsqrt` (`00ec6040`): the square root of `ST0` (a leading `f64` argument; `ST0` answers).
const CI_SQRT: u32 = 0x00ec_6040;
/// `_CIcos` (`00ec9f70`): the cosine of `ST0` (a leading `f64` argument; `ST0` answers).
const CI_COS: u32 = 0x00ec_9f70;

/// What follows the rounded answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Then {
    /// Stored as it is.
    Store,
    /// `FDIVR double ptr [b]`.
    DivInto(u32),
    /// `FMUL double ptr [m]; FDIVR double ptr [b]; FMUL double ptr [s]`.
    MulDivMul(u32, u32, u32),
}

/// One root or cosine initializer.
#[derive(Clone, Copy, Debug)]
struct Root {
    function: u32,
    arg: u32,
    then: Then,
    global: u32,
}

fn root(e: &mut Engine, r: &Root) {
    let x = x87_load64(e.global::<u64>(r.arg));
    let answer = e.call(r.function, &args![x]).st0;
    // FSTP float ptr [ESP]; FLD float ptr [ESP].
    let v = x87_load32(x87_store32(answer));
    let out = match r.then {
        Then::Store => v,
        Then::DivInto(b) => x87_div(e.global::<f64>(b), v),
        Then::MulDivMul(m, b, s) => {
            let p = x87_mul(v, e.global::<f64>(m));
            let q = x87_div(e.global::<f64>(b), p);
            x87_mul(q, e.global::<f64>(s))
        }
    };
    e.set_global::<u32>(r.global, x87_store32(out));
}

/// The zeroed-array loop: `n + 1` rounds of four zero floats from `base - 8`, in the code's
/// store order.
fn zero_loop(e: &mut Engine, n: u32, base: u32) {
    let mut ecx = n as i32;
    let mut eax = base;
    loop {
        e.set_global::<u32>(eax.wrapping_sub(8), 0);
        eax = eax.wrapping_add(0x10);
        ecx = ecx.wrapping_sub(1);
        e.set_global::<u32>(eax.wrapping_sub(0x14), 0);
        e.set_global::<u32>(eax.wrapping_sub(0x10), 0);
        e.set_global::<u32>(eax.wrapping_sub(0xc), 0);
        // JNS: again while the count is not negative.
        if ecx < 0 {
            break;
        }
    }
}

/// `count` zero floats from `first` upwards (`FST` each, `FSTP` the last).
fn zero_run(e: &mut Engine, first: u32, count: u32) {
    for k in 0..count {
        e.set_global::<u32>(first + 4 * k, 0);
    }
}

// Translated from 00fb51c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float at `011f3ff0`: `_CIsqrt` of the double at `01099680`, rounded to a float, divided into the double at `010996d0`.
pub fn fn_00fb51c0(e: &mut Engine) {
    root(e, &ROOT_00FB51C0);
}

/// `00fb51c0`.
const ROOT_00FB51C0: Root = Root {
    function: CI_SQRT,
    arg: 0x0109_9680,
    then: Then::DivInto(0x0109_96d0),
    global: 0x011f_3ff0,
};

// Translated from 00fb51e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float at `011f3fec`: `_CIsqrt` of the double at `01099680`, rounded to a float, times the double at `01099680`, divided into the double at `01011598`, times the double at `010996d0`.
pub fn fn_00fb51e0(e: &mut Engine) {
    root(e, &ROOT_00FB51E0);
}

/// `00fb51e0`.
const ROOT_00FB51E0: Root = Root {
    function: CI_SQRT,
    arg: 0x0109_9680,
    then: Then::MulDivMul(0x0109_9680, 0x0101_1598, 0x0109_96d0),
    global: 0x011f_3fec,
};

// Translated from 00fb5270 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float at `011f4010`: `_CIsqrt` of the double at `01099680`, rounded to a float, divided into the double at `010996d0`.
pub fn fn_00fb5270(e: &mut Engine) {
    root(e, &ROOT_00FB5270);
}

/// `00fb5270`.
const ROOT_00FB5270: Root = Root {
    function: CI_SQRT,
    arg: 0x0109_9680,
    then: Then::DivInto(0x0109_96d0),
    global: 0x011f_4010,
};

// Translated from 00fb5290 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float at `011f400c`: `_CIsqrt` of the double at `01099680`, rounded to a float, times the double at `01099680`, divided into the double at `01011598`, times the double at `010996d0`.
pub fn fn_00fb5290(e: &mut Engine) {
    root(e, &ROOT_00FB5290);
}

/// `00fb5290`.
const ROOT_00FB5290: Root = Root {
    function: CI_SQRT,
    arg: 0x0109_9680,
    then: Then::MulDivMul(0x0109_9680, 0x0101_1598, 0x0109_96d0),
    global: 0x011f_400c,
};

// Translated from 00fb5a50 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float at `011f4a00`: `_CIsqrt` of the double at `01099680`, rounded to a float, divided into the double at `010996d0`.
pub fn fn_00fb5a50(e: &mut Engine) {
    root(e, &ROOT_00FB5A50);
}

/// `00fb5a50`.
const ROOT_00FB5A50: Root = Root {
    function: CI_SQRT,
    arg: 0x0109_9680,
    then: Then::DivInto(0x0109_96d0),
    global: 0x011f_4a00,
};

// Translated from 00fb5a70 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float at `011f49fc`: `_CIsqrt` of the double at `01099680`, rounded to a float, times the double at `01099680`, divided into the double at `01011598`, times the double at `010996d0`.
pub fn fn_00fb5a70(e: &mut Engine) {
    root(e, &ROOT_00FB5A70);
}

/// `00fb5a70`.
const ROOT_00FB5A70: Root = Root {
    function: CI_SQRT,
    arg: 0x0109_9680,
    then: Then::MulDivMul(0x0109_9680, 0x0101_1598, 0x0109_96d0),
    global: 0x011f_49fc,
};

// Translated from 00fbe640 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float at `012686dc`: `_CIcos` of the double at `010cb310`, rounded to a float, as it is.
pub fn fn_00fbe640(e: &mut Engine) {
    root(e, &ROOT_00FBE640);
}

/// `00fbe640`.
const ROOT_00FBE640: Root = Root {
    function: CI_COS,
    arg: 0x010c_b310,
    then: Then::Store,
    global: 0x0126_86dc,
};

// Translated from 00fbfe90 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float at `01269b24`: `_CIsqrt` of the double at `01099680`, rounded to a float, divided into the double at `010996d0`.
pub fn fn_00fbfe90(e: &mut Engine) {
    root(e, &ROOT_00FBFE90);
}

/// `00fbfe90`.
const ROOT_00FBFE90: Root = Root {
    function: CI_SQRT,
    arg: 0x0109_9680,
    then: Then::DivInto(0x0109_96d0),
    global: 0x0126_9b24,
};

// Translated from 00fbfeb0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float at `01269b1c`: `_CIsqrt` of the double at `01099680`, rounded to a float, times the double at `01099680`, divided into the double at `01011598`, times the double at `010996d0`.
pub fn fn_00fbfeb0(e: &mut Engine) {
    root(e, &ROOT_00FBFEB0);
}

/// `00fbfeb0`.
const ROOT_00FBFEB0: Root = Root {
    function: CI_SQRT,
    arg: 0x0109_9680,
    then: Then::MulDivMul(0x0109_9680, 0x0101_1598, 0x0109_96d0),
    global: 0x0126_9b1c,
};

// Translated from 00fc7580 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float at `0126f9c4`: `_CIsqrt` of the double at `010231b0`, rounded to a float, as it is.
pub fn fn_00fc7580(e: &mut Engine) {
    root(e, &ROOT_00FC7580);
}

/// `00fc7580`.
const ROOT_00FC7580: Root = Root {
    function: CI_SQRT,
    arg: 0x0102_31b0,
    then: Then::Store,
    global: 0x0126_f9c4,
};

// Translated from 00fb99f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zeroes 37 rounds of four floats from `011fa0c8` - 8 ([`zero_loop`], `ECX` = 0x24).
pub fn fn_00fb99f0(e: &mut Engine) {
    zero_loop(e, 0x24, 0x011f_a0c8);
}

// Translated from 00fb9d40 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zeroes 11 rounds of four floats from `011ff4d0` - 8 ([`zero_loop`], `ECX` = 0xa).
pub fn fn_00fb9d40(e: &mut Engine) {
    zero_loop(e, 0xa, 0x011f_f4d0);
}

// Translated from 00fb9d70 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zeroes 31 rounds of four floats from `011ff580` - 8 ([`zero_loop`], `ECX` = 0x1e).
pub fn fn_00fb9d70(e: &mut Engine) {
    zero_loop(e, 0x1e, 0x011f_f580);
}

// Translated from 00fba8c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zeroes 164 rounds of four floats from `012010f8` - 8 ([`zero_loop`], `ECX` = 0xa3).
pub fn fn_00fba8c0(e: &mut Engine) {
    zero_loop(e, 0xa3, 0x0120_10f8);
}

// Translated from 00fba8f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zeroes 63 rounds of four floats from `01201b38` - 8 ([`zero_loop`], `ECX` = 0x3e).
pub fn fn_00fba8f0(e: &mut Engine) {
    zero_loop(e, 0x3e, 0x0120_1b38);
}

// Translated from 00fbab80 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zeroes 16 rounds of four floats from `01202050` - 8 ([`zero_loop`], `ECX` = 0xf).
pub fn fn_00fbab80(e: &mut Engine) {
    zero_loop(e, 0xf, 0x0120_2050);
}

// Translated from 00fb9a60 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zeroes the 40 floats from `011fd9a8` (`FLDZ`, `FST` each, `FSTP` the last).
pub fn fn_00fb9a60(e: &mut Engine) {
    zero_run(e, 0x011f_d9a8, 40);
}

// Translated from 00fb9da0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zeroes the 32 floats from `011ff768` (`FLDZ`, `FST` each, `FSTP` the last).
pub fn fn_00fb9da0(e: &mut Engine) {
    zero_run(e, 0x011f_f768, 32);
}

// Translated from 00fb9e70 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zeroes the 32 floats from `011ff7e8` (`FLDZ`, `FST` each, `FSTP` the last).
pub fn fn_00fb9e70(e: &mut Engine) {
    zero_run(e, 0x011f_f7e8, 32);
}

// Translated from 00fb9f60 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zeroes the 12 floats from `011ff8e0` (`FLDZ`, `FST` each, `FSTP` the last).
pub fn fn_00fb9f60(e: &mut Engine) {
    zero_run(e, 0x011f_f8e0, 12);
}

// Translated from 00fb9fb0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zeroes the 12 floats from `011ff910` (`FLDZ`, `FST` each, `FSTP` the last).
pub fn fn_00fb9fb0(e: &mut Engine) {
    zero_run(e, 0x011f_f910, 12);
}

// Translated from 00fba1f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zeroes the 36 floats from `01200510` (`FLDZ`, `FST` each, `FSTP` the last).
pub fn fn_00fba1f0(e: &mut Engine) {
    zero_run(e, 0x0120_0510, 36);
}

// Translated from 00fba550 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zeroes the 24 floats from `01200618` (`FLDZ`, `FST` each, `FSTP` the last).
pub fn fn_00fba550(e: &mut Engine) {
    zero_run(e, 0x0120_0618, 24);
}

// Translated from 00fbabb0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zeroes the 8 floats from `01202148` (`FLDZ`, `FST` each, `FSTP` the last).
pub fn fn_00fbabb0(e: &mut Engine) {
    zero_run(e, 0x0120_2148, 8);
}

// Translated from 00fbabf0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zeroes the 8 floats from `01202168` (`FLDZ`, `FST` each, `FSTP` the last).
pub fn fn_00fbabf0(e: &mut Engine) {
    zero_run(e, 0x0120_2168, 8);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00fb_51c0, fn_00fb51c0()),
        entry!(0x00fb_51e0, fn_00fb51e0()),
        entry!(0x00fb_5270, fn_00fb5270()),
        entry!(0x00fb_5290, fn_00fb5290()),
        entry!(0x00fb_5a50, fn_00fb5a50()),
        entry!(0x00fb_5a70, fn_00fb5a70()),
        entry!(0x00fb_e640, fn_00fbe640()),
        entry!(0x00fb_fe90, fn_00fbfe90()),
        entry!(0x00fb_feb0, fn_00fbfeb0()),
        entry!(0x00fc_7580, fn_00fc7580()),
        entry!(0x00fb_99f0, fn_00fb99f0()),
        entry!(0x00fb_9d40, fn_00fb9d40()),
        entry!(0x00fb_9d70, fn_00fb9d70()),
        entry!(0x00fb_a8c0, fn_00fba8c0()),
        entry!(0x00fb_a8f0, fn_00fba8f0()),
        entry!(0x00fb_ab80, fn_00fbab80()),
        entry!(0x00fb_9a60, fn_00fb9a60()),
        entry!(0x00fb_9da0, fn_00fb9da0()),
        entry!(0x00fb_9e70, fn_00fb9e70()),
        entry!(0x00fb_9f60, fn_00fb9f60()),
        entry!(0x00fb_9fb0, fn_00fb9fb0()),
        entry!(0x00fb_a1f0, fn_00fba1f0()),
        entry!(0x00fb_a550, fn_00fba550()),
        entry!(0x00fb_abb0, fn_00fbabb0()),
        entry!(0x00fb_abf0, fn_00fbabf0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROOTS: &[(u32, Root)] = &[
        (0x00fb_51c0, ROOT_00FB51C0),
        (0x00fb_51e0, ROOT_00FB51E0),
        (0x00fb_5270, ROOT_00FB5270),
        (0x00fb_5290, ROOT_00FB5290),
        (0x00fb_5a50, ROOT_00FB5A50),
        (0x00fb_5a70, ROOT_00FB5A70),
        (0x00fb_e640, ROOT_00FBE640),
        (0x00fb_fe90, ROOT_00FBFE90),
        (0x00fb_feb0, ROOT_00FBFEB0),
        (0x00fc_7580, ROOT_00FC7580),
    ];
    const LOOPS: &[(u32, u32, u32)] = &[
        (0x00fb_99f0, 0x24, 0x011f_a0c8),
        (0x00fb_9d40, 0xa, 0x011f_f4d0),
        (0x00fb_9d70, 0x1e, 0x011f_f580),
        (0x00fb_a8c0, 0xa3, 0x0120_10f8),
        (0x00fb_a8f0, 0x3e, 0x0120_1b38),
        (0x00fb_ab80, 0xf, 0x0120_2050),
    ];
    const RUNS: &[(u32, u32, u32)] = &[
        (0x00fb_9a60, 0x011f_d9a8, 40),
        (0x00fb_9da0, 0x011f_f768, 32),
        (0x00fb_9e70, 0x011f_f7e8, 32),
        (0x00fb_9f60, 0x011f_f8e0, 12),
        (0x00fb_9fb0, 0x011f_f910, 12),
        (0x00fb_a1f0, 0x0120_0510, 36),
        (0x00fb_a550, 0x0120_0618, 24),
        (0x00fb_abb0, 0x0120_2148, 8),
        (0x00fb_abf0, 0x0120_2168, 8),
    ];

    /// The double (two stack words, low first) a call was given.
    fn f64_arg(a: &[u32]) -> f64 {
        f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32))
    }

    #[test]
    fn roots_and_the_cosine_compute_in_the_codes_order() {
        for &(entry, r) in ROOTS {
            let mut e = Engine::new();
            for (a, v) in [
                (0x0109_9680u32, 4.0f64),
                (0x0109_96d0, 3.0),
                (0x0101_1598, 16.0),
                (0x0102_31b0, 9.0),
                (0x010c_b310, 0.0),
            ] {
                e.map(a, 8);
                e.mem.set_f64(a, v);
            }
            e.map(r.global, 4);
            e.register_double(CI_SQRT, |_, a| Ret {
                st0: f64_arg(a).sqrt(),
                ..Ret::default()
            });
            e.register_double(CI_COS, |_, a| Ret {
                st0: f64_arg(a).cos(),
                ..Ret::default()
            });
            e.call(entry, &args![]);
            let want: f32 = match (r.function, r.then) {
                (CI_COS, _) => 1.0,
                (_, Then::Store) => 3.0,
                (_, Then::DivInto(_)) => 1.5,
                (_, Then::MulDivMul(..)) => 6.0,
            };
            assert_eq!(e.mem.u32(r.global), want.to_bits(), "{entry:08x}");
        }
        assert_eq!(ROOTS.len(), 10);
    }

    #[test]
    fn zero_loops_clear_n_plus_one_rounds_of_four_floats() {
        for &(entry, n, base) in LOOPS {
            let mut e = Engine::new();
            let start = base - 8;
            let len = 16 * (n + 1);
            e.map(start - 4, len + 8);
            for k in 0..(len + 8) / 4 {
                e.mem.set_u32(start - 4 + 4 * k, 0xffff_ffff);
            }
            e.call(entry, &args![]);
            assert_eq!(e.mem.u32(start - 4), 0xffff_ffff, "{entry:08x}");
            for k in 0..len / 4 {
                assert_eq!(e.mem.u32(start + 4 * k), 0, "{entry:08x}");
            }
            assert_eq!(e.mem.u32(start + len), 0xffff_ffff, "{entry:08x}");
        }
    }

    #[test]
    fn zero_runs_clear_their_floats() {
        for &(entry, first, count) in RUNS {
            let mut e = Engine::new();
            e.map(first, 4 * count + 4);
            for k in 0..=count {
                e.mem.set_u32(first + 4 * k, 0xffff_ffff);
            }
            e.call(entry, &args![]);
            for k in 0..count {
                assert_eq!(e.mem.u32(first + 4 * k), 0, "{entry:08x}");
            }
            assert_eq!(e.mem.u32(first + 4 * count), 0xffff_ffff, "{entry:08x}");
        }
        assert_eq!(funcs().len(), ROOTS.len() + LOOPS.len() + RUNS.len());
    }
}
