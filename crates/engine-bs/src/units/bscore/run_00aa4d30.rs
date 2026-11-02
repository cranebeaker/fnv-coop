//! `BSCore/run_00aa4d30` (Xbox PDB source unit), subsystem `BSCore`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `QueryPerformanceFrequency` import slot (`kernel32`, stdcall: pointer to a 64-bit result).
const QUERY_PERFORMANCE_FREQUENCY: u32 = 0x00fd_f0a4;
/// A `double` divisor (`.rdata`).
const DIVISOR_01017B70: u32 = 0x0101_7b70;
/// The `float` global that receives the quotient (`.data`).
const RESULT_011AC39C: u32 = 0x011a_c39c;

// Translated from 00aa4d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: no parameters. Reads the performance-counter frequency into a
/// 64-bit local, converts it as a signed integer to floating point, divides by the `double` at
/// `01017b70` and stores the quotient as a `float` at `011ac39c`. Computed in `f64`, rounded
/// to `f32` at the store.
pub fn fn_00aa4d50(e: &mut Engine) {
    let frequency = e.with_stack(8, |e, local| {
        e.call(QUERY_PERFORMANCE_FREQUENCY, &args![local]);
        e.mem.u64(local.addr()) as i64
    });
    let quotient = frequency as f64 / e.global::<f64>(DIVISOR_01017B70);
    e.set_global(RESULT_011AC39C, quotient as f32);
}

// Translated from 00aa4d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns 0 (thiscall, `RET 0x14`: `this` and five stack words, none of them read).
pub fn fn_00aa4d40(
    _e: &mut Engine,
    _this: Ptr,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    0
}

// Translated from 00aa4d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0xc of `BSCoreMessage`'s virtual table (`010a2708`;
/// `Assert` in the Xbox PDB's order, not confirmed); no stack words, `this` not read:
/// answers 0.
pub fn fn_00aa4d30(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00aa_4d30, fn_00aa4d30(Ptr) -> u32),
        entry!(0x00aa_4d50, fn_00aa4d50()),
        entry!(0x00aa_4d40, fn_00aa4d40(Ptr, u32, u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod static_init_callee_tests {
    use super::*;

    fn calls_to(e: &mut Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    #[test]
    fn stores_frequency_over_divisor_as_a_float() {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.map(0x011a_c000, 0x1000);
        e.set_global(DIVISOR_01017B70, 1000.0f64);
        e.register(QUERY_PERFORMANCE_FREQUENCY, |e, a| {
            e.mem.set_u64(a[0], 2_500_000);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00aa_4d50, &args![]);
        assert_eq!(calls_to(&mut e, QUERY_PERFORMANCE_FREQUENCY).len(), 1);
        assert_eq!(e.global::<f32>(RESULT_011AC39C), 2500.0);
    }

    #[test]
    fn a_negative_frequency_is_converted_as_signed() {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.map(0x011a_c000, 0x1000);
        e.set_global(DIVISOR_01017B70, 4.0f64);
        e.register(QUERY_PERFORMANCE_FREQUENCY, |e, a| {
            e.mem.set_u64(a[0], (-8i64) as u64);
            Ret::default()
        });
        e.call(0x00aa_4d50, &args![]);
        assert_eq!(e.global::<f32>(RESULT_011AC39C), -2.0);
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    #[test]
    fn returns_zero() {
        let mut e = Engine::new();
        let r = e.call(0x00aa_4d40, &args![1u32, 2u32, 3u32, 4u32, 5u32, 6u32]);
        assert_eq!(r.u32(), 0);
    }
}

#[cfg(test)]
mod n082_tests {
    use super::*;

    #[test]
    fn fn_00aa4d30_answers_zero() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00aa_4d30, &args![0x5000u32]).u32(), 0);
    }
}
