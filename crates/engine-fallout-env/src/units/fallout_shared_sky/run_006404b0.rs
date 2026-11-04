//! `fallout shared/sky/run_006404b0` (Xbox PDB source unit), subsystem `fallout shared/sky`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `004168a0` (`fallout shared/extradatalist.cpp`, `thiscall`, four `float` words, `RET 0x10`).
const CALLEE_004168A0: u32 = 0x0041_68a0;

// Translated from 006404b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `thiscall` with four `float`s (`RET 0x10`); passes them on, in
/// the same order, to `004168a0` on the member at `this + 0x34`. Each float goes through
/// `FLD`/`FSTP` on the way (a signalling NaN arrives quiet).
pub fn fn_006404b0(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32, d: u32) {
    let member = this.addr().wrapping_add(0x34);
    let (a, b, c, d) = (
        x87_quiet_bits(a),
        x87_quiet_bits(b),
        x87_quiet_bits(c),
        x87_quiet_bits(d),
    );
    e.call(CALLEE_004168A0, &args![member, a, b, c, d]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0064_04b0, fn_006404b0(Ptr, u32, u32, u32, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_006404b0_passes_the_floats_to_the_member() {
        let mut e = Engine::new();
        e.register(CALLEE_004168A0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(
            0x0064_04b0,
            &args![0x4000u32, 1.5f32, 0x7f80_0001u32, -2.0f32, 0.25f32],
        );
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_004168A0).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(
            calls[0].1,
            vec![
                0x4034,
                1.5f32.to_bits(),
                0x7fc0_0001,
                (-2.0f32).to_bits(),
                0.25f32.to_bits()
            ]
        );
    }
}
