//! `(unplaced)/run_0070ecb0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The dividend and the divisor (`00706e20` and `00715da0`, no arguments,
/// each a value in ST0).
const DIVIDEND_00706E20: u32 = 0x0070_6e20;
const DIVISOR_00715DA0: u32 = 0x0071_5da0;

// Translated from 0070ecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `00706e20()` (stored as a `double`) divided by `00715da0()` (`FDIVR`),
/// rounded to a `float` (unnamed in the engine map; no arguments).
pub fn fn_0070ecb0(e: &mut Engine) -> f32 {
    let dividend = e.call(DIVIDEND_00706E20, &args![]).f64();
    let divisor = e.call(DIVISOR_00715DA0, &args![]).f64();
    f32::from_bits(x87_store32(x87_div(dividend, divisor)))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0070_ecb0, fn_0070ecb0() -> f32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn divides_the_first_value_by_the_second() {
        let mut e = Engine::new();
        e.register(DIVIDEND_00706E20, |_, _| Ret {
            st0: 1280.0,
            ..Ret::default()
        });
        e.register(DIVISOR_00715DA0, |_, _| Ret {
            st0: 720.0,
            ..Ret::default()
        });
        let ratio = e.call(0x0070_ecb0, &args![]).f64();
        assert_eq!(ratio, f64::from((1280.0f64 / 720.0) as f32));
        e.register(DIVISOR_00715DA0, |_, _| Ret::default());
        assert_eq!(e.call(0x0070_ecb0, &args![]).f64(), f64::INFINITY);
    }
}
