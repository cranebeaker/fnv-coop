//! `(unplaced)/run_0051f5f0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The bytes the two getters return (+0x11d and +0x101 of `this`).
const BYTE_11D: u32 = 0x11d;
const BYTE_101: u32 = 0x101;

// Translated from 0051f5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x11d of `this` (zero-extended; unnamed in the engine map).
pub fn fn_0051f5f0(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u8(this.addr() + BYTE_11D))
}

// Translated from 0051f610 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at +0x101 of `this` (zero-extended; unnamed in the engine map).
pub fn fn_0051f610(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u8(this.addr() + BYTE_101))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_f5f0, fn_0051f5f0(Ptr) -> u32),
        entry!(0x0051_f610, fn_0051f610(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getters_zero_extend_their_bytes() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x120);
        for offset in 0..0x120 {
            e.mem.set_u8(this + offset, 0xee);
        }
        e.mem.set_u8(this + BYTE_11D, 0x81);
        e.mem.set_u8(this + BYTE_101, 0x7f);
        assert_eq!(e.call(0x0051_f5f0, &args![this]).u32(), 0x81);
        assert_eq!(e.call(0x0051_f610, &args![this]).u32(), 0x7f);
    }
}
