//! `(unplaced)/run_00425fd0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00425fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the byte at `+0x24` of `this` is set (a plain flag getter).
pub fn fn_00425fd0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x24) & 1 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00425fd0, fn_00425fd0(Ptr) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_bit_zero_of_the_byte_at_0x24() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x30));
        assert!(!e.call(0x0042_5fd0, &args![object]).bool());
        // Only bit 0 counts.
        e.mem.set_u8(object.addr() + 0x24, 0xfe);
        assert!(!e.call(0x0042_5fd0, &args![object]).bool());
        e.mem.set_u8(object.addr() + 0x24, 0x01);
        assert!(e.call(0x0042_5fd0, &args![object]).bool());
    }
}
