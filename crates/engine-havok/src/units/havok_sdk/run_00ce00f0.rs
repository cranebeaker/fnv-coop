//! `Havok SDK/run_00ce00f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkMath::atan2fApproximation` (`00cbff80`, cdecl, two floats, result in `ST0`; named by
/// Ghidra from the Xbox PDB).
const ATAN2_APPROXIMATION: u32 = 0x00cb_ff80;

// Translated from 00ce00f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// cdecl `(first, second)`: passes both floats to `hkMath::atan2fApproximation` in the same
/// order, each copied through the x87 stack (`FLD`/`FSTP`, so a signalling NaN is quieted),
/// and returns its `ST0` result unchanged.
pub fn fn_00ce00f0(e: &mut Engine, first: f32, second: f32) -> f32 {
    let first = x87_quiet_bits(first.to_bits());
    let second = x87_quiet_bits(second.to_bits());
    e.call(ATAN2_APPROXIMATION, &args![first, second]).f32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00ce_00f0, fn_00ce00f0(f32, f32) -> f32)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Both floats reach the callee in order as bit patterns (a signalling NaN quieted) and its
    /// result comes back.
    #[test]
    fn passes_both_floats_on_and_returns_the_result() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&seen);
        e.register_double(ATAN2_APPROXIMATION, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                st0: 0.75,
                ..Ret::default()
            }
        });
        let got = e.call(0x00ce_00f0, &args![1.5f32, -2.0f32]);
        assert_eq!(got.f32(), 0.75);
        e.call(0x00ce_00f0, &args![f32::from_bits(0x7f80_0001), 3.0f32]);
        assert_eq!(
            *seen.borrow(),
            vec![
                vec![1.5f32.to_bits(), (-2.0f32).to_bits()],
                vec![0x7fc0_0001, 3.0f32.to_bits()]
            ]
        );
    }
}
