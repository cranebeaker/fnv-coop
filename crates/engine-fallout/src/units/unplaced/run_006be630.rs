//! `(unplaced)/run_006be630` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006be630 (decompiled, FalloutNV.exe 1.4.0.525)
/// A three-word 16-bit setter (unnamed in the map; `thiscall`, `(first, second, third)`):
/// the three values at +0, +2 and +4.
pub fn fn_006be630(e: &mut Engine, this: Ptr, first: u16, second: u16, third: u16) {
    let at = this.addr();
    e.mem.set_u16(at, first);
    e.mem.set_u16(at.wrapping_add(2), second);
    e.mem.set_u16(at.wrapping_add(4), third);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006be630, fn_006be630(Ptr, u16, u16, u16))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_006be630_sets_three_halfwords() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.call(0x006be630, &args![this, 0x1_0001u32, 2u32, 3u32]);
        assert_eq!([0u32, 2, 4, 6].map(|o| e.mem.u16(this + o)), [1, 2, 3, 0]);
    }
}
