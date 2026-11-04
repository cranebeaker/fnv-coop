//! `fallout/magic/run_0080a900` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `0082ba60` (unnamed in the engine map; `thiscall`, three words).
const BASE_CONSTRUCTOR: u32 = 0x0082_ba60;
/// `CalmEffect`'s vtable (`0107989c`, RTTI `.?AVCalmEffect@@`).
const CALM_EFFECT_VTABLE: u32 = 0x0107_989c;

// Translated from 0080a900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CalmEffect` constructor (by the vtable it stores; unnamed in the engine map;
/// `thiscall`, `RET 0xc`): runs the base constructor `0082ba60` with the three words, then
/// stores the vtable. Returns `this`.
pub fn fn_0080a900(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(BASE_CONSTRUCTOR, &args![this, caster, spell, effect]);
    e.mem.set_u32(this.addr(), CALM_EFFECT_VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080_a900, fn_0080a900(Ptr, u32, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_the_base_then_sets_the_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0xdead);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_a900, &[this, 1, 2, 3]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log.iter().filter(|(a, _)| *a == BASE_CONSTRUCTOR).collect();
        assert_eq!(base[0].1, vec![this, 1, 2, 3]);
        assert_eq!(e.mem.u32(this), CALM_EFFECT_VTABLE);
    }
}
