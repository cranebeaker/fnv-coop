//! `fallout/ai/run_00982f10` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00982f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the two bytes at `source` to +0x14 and +0x15 (`RET 4`).
pub fn fn_00982f10(e: &mut Engine, this: Ptr, source: Ptr) {
    let first = e.mem.u8(source.addr());
    e.mem.set_u8(this.addr() + 0x14, first);
    let second = e.mem.u8(source.addr() + 1);
    e.mem.set_u8(this.addr() + 0x15, second);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0098_2f10, fn_00982f10(Ptr, Ptr))]
}

#[cfg(test)]
mod n028_tests {
    use super::*;

    #[test]
    fn fn_00982f10_copies_the_two_bytes() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x18);
        let source = e.mem.alloc(2);
        e.mem.write(source, &[0x12, 0x34]);
        e.call(0x0098_2f10, &[this, source]);
        assert_eq!(e.mem.bytes(this + 0x14, 2), vec![0x12, 0x34]);
    }
}
