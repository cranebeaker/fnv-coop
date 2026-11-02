//! `fallout shared/pathfinding/run_006bcfe0` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 006bcfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(first, second)`, two 16-bit words): fills the first
/// free pair of 16-bit slots: when the word at +0x08 is 0xFFFF, `first` goes there and
/// `second` to +0x0C; else when the word at +0x0A is 0xFFFF, `first` goes there and `second`
/// to +0x0E; else nothing.
pub fn fn_006bcfe0(e: &mut Engine, this: Ptr, first: u16, second: u16) {
    let this = this.addr();
    if e.mem.u16(this.wrapping_add(8)) == 0xffff {
        e.mem.set_u16(this.wrapping_add(8), first);
        e.mem.set_u16(this.wrapping_add(0xc), second);
    } else if e.mem.u16(this.wrapping_add(0xa)) == 0xffff {
        e.mem.set_u16(this.wrapping_add(0xa), first);
        e.mem.set_u16(this.wrapping_add(0xe), second);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006bcfe0, fn_006bcfe0(Ptr, u16, u16))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_006bcfe0_fills_the_first_free_pair() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u16(this + 8, 0xffff);
        e.mem.set_u16(this + 0xa, 0xffff);
        e.call(0x006bcfe0, &args![this, 1u32, 2u32]);
        e.call(0x006bcfe0, &args![this, 3u32, 4u32]);
        e.call(0x006bcfe0, &args![this, 5u32, 6u32]);
        assert_eq!(
            [8u32, 0xa, 0xc, 0xe].map(|o| e.mem.u16(this + o)),
            [1, 3, 2, 4]
        );
    }
}
