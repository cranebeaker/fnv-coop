//! `fallout/magic/run_0080c890` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `ActiveEffect` constructor `00803d30` (`thiscall`, three words).
const ACTIVE_EFFECT_CONSTRUCTOR: u32 = 0x0080_3d30;
/// The `DemoralizeEffect` vtable (`01079ddc`, RTTI `.?AVDemoralizeEffect@@`).
const DEMORALIZE_EFFECT_VTABLE: u32 = 0x0107_9ddc;

// Translated from 0080c890 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `DemoralizeEffect` constructor (by the vtable it stores; unnamed in the engine map;
/// `thiscall`, `RET 0xc`): runs the `ActiveEffect` constructor `00803d30` with the three
/// words, stores the vtable and clears `bTurned` (+0x48, Xbox PDB). Returns `this`.
pub fn fn_0080c890(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(
        ACTIVE_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, effect],
    );
    e.mem.set_u32(this.addr(), DEMORALIZE_EFFECT_VTABLE);
    e.mem.set_u8(this.addr() + 0x48, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0080_c890, fn_0080c890(Ptr, u32, u32, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_the_base_sets_the_vtable_and_clears_the_flag() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0] + 0x48, 0xffff_ffff);
            a[0].into_ret()
        });
        let this = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_c890, &[this, 1, 2, 3]).u32(), this);
        let log = e.call_log.take().unwrap();
        let base: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == ACTIVE_EFFECT_CONSTRUCTOR)
            .collect();
        assert_eq!(base[0].1, vec![this, 1, 2, 3]);
        assert_eq!(e.mem.u32(this), DEMORALIZE_EFFECT_VTABLE);
        // Only the byte at +0x48 is cleared.
        assert_eq!(e.mem.u32(this + 0x48), 0xffff_ff00);
    }
}
