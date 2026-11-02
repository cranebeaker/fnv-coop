//! `fallout/magic/run_0080ba90` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `CommandEffect` constructor `0080b560` (`thiscall`, three words).
const BASE_CONSTRUCTOR: u32 = 0x0080_b560;
/// `CommandHumanoidEffect`'s vtable (`01079b3c`, RTTI `.?AVCommandHumanoidEffect@@`).
const VTABLE: u32 = 0x0107_9b3c;

// Translated from 0080ba90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CommandHumanoidEffect` constructor (by the vtable it stores; unnamed in the engine map;
/// `thiscall`, `RET 0xc`): runs the base constructor `0080b560` with the three words, then
/// stores the vtable. Returns `this`.
pub fn fn_0080ba90(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(BASE_CONSTRUCTOR, &args![this, caster, spell, effect]);
    e.mem.set_u32(this.addr(), VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080_ba90, fn_0080ba90(Ptr, u32, u32, u32) -> Ptr)]
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
        let this = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_ba90, &[this, 1, 2, 3]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log.iter().filter(|(a, _)| *a == BASE_CONSTRUCTOR).collect();
        assert_eq!(base[0].1, vec![this, 1, 2, 3]);
        assert_eq!(e.mem.u32(this), VTABLE);
    }
}
