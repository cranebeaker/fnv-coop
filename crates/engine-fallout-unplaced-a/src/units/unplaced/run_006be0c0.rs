//! `(unplaced)/run_006be0c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006be0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, no arguments; on an edge record): true when its first
/// 16-bit slot (+0x08) is free (0xFFFF); when the first is taken and the second (+0x0A) is
/// free, true unless the word at +0x10 is 1, 3 or 4; false when both are taken.
pub fn fn_006be0c0(e: &mut Engine, this: Ptr) -> bool {
    let at = this.addr();
    if e.mem.u16(at.wrapping_add(8)) == 0xffff {
        return true;
    }
    if e.mem.u16(at.wrapping_add(0xa)) != 0xffff {
        return false;
    }
    let kind = e.mem.u32(at.wrapping_add(0x10));
    !(kind == 1 || kind == 3 || kind == 4)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006be0c0, fn_006be0c0(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_006be0c0_checks_the_free_slots_and_kind() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        e.mem.set_u16(this + 8, 0xffff);
        assert!(e.call(0x006be0c0, &args![this]).bool());
        e.mem.set_u16(this + 8, 1);
        assert!(!e.call(0x006be0c0, &args![this]).bool());
        e.mem.set_u16(this + 0xa, 0xffff);
        for (kind, free) in [
            (0u32, true),
            (1, false),
            (2, true),
            (3, false),
            (4, false),
            (5, true),
        ] {
            e.mem.set_u32(this + 0x10, kind);
            assert_eq!(e.call(0x006be0c0, &args![this]).bool(), free);
        }
    }
}
