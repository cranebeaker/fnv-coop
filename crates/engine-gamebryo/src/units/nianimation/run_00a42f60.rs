//! `NiAnimation/run_00a42f60` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00a36890` (`thiscall`; the engine map has no
/// name) and `BSBlendTreadTransfInterpolator`'s vtable (`01097904`).
const BASE_CONSTRUCTOR: u32 = 0x00a3_6890;
const BLEND_TREAD_VTABLE: u32 = 0x0109_7904;
/// `BSBlendTreadTransfInterpolator::fCurrLerp` (Xbox PDB) +0x30.
const CURR_LERP: u32 = 0x30;

// Translated from 00a42f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BSBlendTreadTransfInterpolator` constructor (the engine map has no
/// name; it writes that class's vtable): the base constructor `00a36890`,
/// `fCurrLerp` 0.0, the vtable. Answers `this`.
pub fn fn_00a42f60(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_f32(this.addr() + CURR_LERP, 0.0);
    e.mem.set_u32(this.addr(), BLEND_TREAD_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a4_2f60, fn_00a42f60(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blend_tread_constructor_runs_the_base_then_clears_the_lerp() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |e, a| {
            e.mem.set_f32(a[0] + CURR_LERP, 9.0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x34);
        assert_eq!(e.call(0x00a4_2f60, &args![this]).u32(), this);
        assert_eq!(e.mem.f32(this + CURR_LERP), 0.0);
        assert_eq!(e.mem.u32(this), BLEND_TREAD_VTABLE);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|call| call.0 == BASE_CONSTRUCTOR && call.1 == vec![this]));
    }
}
