//! `(unplaced)/run_004b5510` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The CRT `double` function `004b5530` forwards to (`00ec9610`, cdecl, a
/// `double`, the result in ST0; Ghidra's library match names it `cos`
/// with a conflict, not confirmed).
const DOUBLE_FUNCTION_00EC9610: u32 = 0x00ec_9610;

// Translated from 004b5510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards `value` (an x87 `FLD`/`FSTP` copy) to `004b5530` and returns
/// its `float` (cdecl; unnamed in the engine map).
pub fn fn_004b5510(e: &mut Engine, value: f32) -> f32 {
    fn_004b5530(e, x87_quiet(value))
}

// Translated from 004b5530 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `float` form of a CRT `double` function (cdecl; Ghidra's library
/// match is `_logf`, its callee's `cos`, both unconfirmed): widens
/// `value` to a `double` (exact), calls `00ec9610` and rounds its result
/// to a `float` (`FSTP float`).
pub fn fn_004b5530(e: &mut Engine, value: f32) -> f32 {
    let widened = x87_load32(value.to_bits());
    let result = e.call(DOUBLE_FUNCTION_00EC9610, &args![widened]).f64();
    f32::from_bits(x87_store32(result))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004b_5510, fn_004b5510(f32) -> f32),
        entry!(0x004b_5530, fn_004b5530(f32) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn double_function(e: &mut Engine) {
        // Returns the argument plus a quarter, as `double`.
        e.register(DOUBLE_FUNCTION_00EC9610, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            Ret {
                st0: x + 0.25,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn fn_004b5530_widens_calls_and_rounds_to_float() {
        let mut e = Engine::new();
        double_function(&mut e);
        e.call_log = Some(vec![]);
        let result = e.call(0x004b_5530, &args![1.5f32.to_bits()]).f64();
        assert_eq!(result, 1.75);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == DOUBLE_FUNCTION_00EC9610)
            .collect();
        let bits = 1.5f64.to_bits();
        assert_eq!(calls[0].1, vec![bits as u32, (bits >> 32) as u32]);
        // The result is rounded to a float.
        let tiny = f32::from_bits(1);
        let rounded = e.call(0x004b_5530, &args![tiny.to_bits()]).f64();
        assert_eq!(rounded, 0.25);
    }

    #[test]
    fn fn_004b5510_forwards_a_quieted_copy() {
        let mut e = Engine::new();
        double_function(&mut e);
        assert_eq!(e.call(0x004b_5510, &args![2.0f32.to_bits()]).f64(), 2.25);
        e.call_log = Some(vec![]);
        let nan = e.call(0x004b_5510, &args![0x7f80_0001u32]).f64();
        assert!(nan.is_nan());
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == DOUBLE_FUNCTION_00EC9610)
            .collect();
        // The quiet NaN 7fc00001 widened.
        let widened = x87_load32(0x7fc0_0001).to_bits();
        assert_eq!(calls[0].1, vec![widened as u32, (widened >> 32) as u32]);
    }
}
