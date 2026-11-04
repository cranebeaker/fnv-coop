//! `(unplaced)/run_005f3710` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005f3710 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name: the address of 16-byte element `index` of the array whose
/// count is at `this + 0x34` and whose elements start at `[this + 0x38]`;
/// 0 when `index` is not below the count (unsigned).
pub fn fn_005f3710(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let base = this.addr();
    if index < e.mem.u32(base + 0x34) {
        (index << 4).wrapping_add(e.mem.u32(base + 0x38))
    } else {
        0
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005f_3710, fn_005f3710(Ptr, u32) -> u32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_005f3710_indexes_within_the_count() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x34, 3);
        e.mem.set_u32(this + 0x38, 0x9000);
        assert_eq!(e.call(0x005f_3710, &args![this, 2u32]).u32(), 0x9020);
        assert_eq!(e.call(0x005f_3710, &args![this, 3u32]).u32(), 0);
        assert_eq!(e.call(0x005f_3710, &args![this, 0xffff_ffffu32]).u32(), 0);
    }
}
