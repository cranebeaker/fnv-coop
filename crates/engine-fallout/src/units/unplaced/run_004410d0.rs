//! `(unplaced)/run_004410d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004410d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores the low byte of the argument
/// at `this + 0x38`.
pub fn fn_004410d0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x38, value);
}

// Translated from 004410f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores the low byte of the argument
/// at `this + 0x39`.
pub fn fn_004410f0(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x39, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044_10d0, fn_004410d0(Ptr, u8)),
        entry!(0x0044_10f0, fn_004410f0(Ptr, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_the_low_byte() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        e.call(0x0044_10d0, &args![this, 0x1234u32]);
        e.call(0x0044_10f0, &args![this, 0x0156u32]);
        assert_eq!((e.mem.u8(this + 0x38), e.mem.u8(this + 0x39)), (0x34, 0x56));
        assert_eq!(e.mem.u8(this + 0x3a), 0);
    }
}
