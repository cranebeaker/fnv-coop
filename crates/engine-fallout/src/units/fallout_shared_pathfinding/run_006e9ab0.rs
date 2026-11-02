//! `fallout shared/pathfinding/run_006e9ab0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The curve segment's parameter setter (`006e7c20`, `pathingsolution.cpp`;
/// thiscall, one `float`).
const SET_PARAMETER: u32 = 0x006e_7c20;
/// The curve segment's blend of two position-and-tangent blocks into a
/// point (`006e7d60`, `pathingsolution.cpp`; thiscall: out, the two blocks).
const BLEND_POINT: u32 = 0x006e_7d60;
/// The segment's two position-and-tangent blocks (+0x14 and +0x2c, as the
/// code passes them).
const START_BLOCK: u32 = 0x14;
const END_BLOCK: u32 = 0x2c;

// Translated from 006e9ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The curve segment's point at `parameter` (thiscall, `RET 8`; unnamed in
/// the engine map): sets the parameter (`006e7c20`, the `float` copied with
/// `FLD`/`FSTP`), then blends the blocks at +0x14 and +0x2c into `out`
/// (`006e7d60`). Returns `out`.
pub fn fn_006e9ab0(e: &mut Engine, this: Ptr, out: Ptr, parameter: f32) -> Ptr {
    e.call(SET_PARAMETER, &args![this, x87_quiet(parameter).to_bits()]);
    let base = this.addr();
    e.call(
        BLEND_POINT,
        &args![this, out, base + START_BLOCK, base + END_BLOCK],
    );
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006e_9ab0, fn_006e9ab0(Ptr, Ptr, f32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn sets_the_parameter_then_blends_into_out() {
        let mut e = Engine::new();
        e.register(SET_PARAMETER, |_, _| Ret::default());
        e.register(BLEND_POINT, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x44);
        let out = e.mem.alloc(0xc);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x006e_9ab0, &args![this, out, 0.25f32.to_bits()])
            .u32();
        assert_eq!(result, out);
        assert_eq!(
            calls_to(&e, SET_PARAMETER),
            vec![vec![this, 0.25f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, BLEND_POINT),
            vec![vec![this, out, this + 0x14, this + 0x2c]]
        );
        // A signalling NaN parameter arrives quieted.
        e.call_log = Some(vec![]);
        e.call(0x006e_9ab0, &args![this, out, 0x7f80_0001u32]);
        assert_eq!(calls_to(&e, SET_PARAMETER), vec![vec![this, 0x7fc0_0001]]);
    }
}
