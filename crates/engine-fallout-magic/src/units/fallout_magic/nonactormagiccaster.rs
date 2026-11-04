//! `fallout/magic/nonactormagiccaster.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0041fd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NonActorMagicCaster::SetCurrentSpell` (Xbox PDB; the map takes the name
/// from that class's vtable): stores `value` in the word at `+0xC` of `this`
/// (`__thiscall`, `RET 4`). The linker shares this body with every other
/// one-word setter at `+0xC` (`GridCellArray`'s constructor calls it to set
/// `GridArray::iDimension`, for example), so callers of other classes use it
/// too.
pub fn non_actor_magic_caster_set_current_spell(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0xc, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0041fd00,
        non_actor_magic_caster_set_current_spell(Ptr, u32)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_the_word_at_offset_0xc() {
        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(0x14));
        e.mem.set_u32(this.addr() + 8, 0x77);
        e.mem.set_u32(this.addr() + 0x10, 0x88);
        e.call(0x0041_fd00, &args![this, 0xdead_beefu32]);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0xdead_beef);
        assert_eq!(e.mem.u32(this.addr() + 8), 0x77);
        assert_eq!(e.mem.u32(this.addr() + 0x10), 0x88);
    }
}
