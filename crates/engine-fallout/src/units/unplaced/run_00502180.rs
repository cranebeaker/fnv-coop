//! `(unplaced)/run_00502180` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00502180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x20 of the byte at +0x24 of `this` is set (no name in the
/// engine map; `SETNZ AL`).
pub fn fn_00502180(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr().wrapping_add(0x24)) & 0x20 != 0
}

// Translated from 005021a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0 of the byte at +8 of `this` is set, as 0 or 1 in all of
/// `EAX` (no name in the engine map).
pub fn fn_005021a0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.i8(this.addr().wrapping_add(8)) & 1 != 0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0050_2180, fn_00502180(Ptr) -> bool),
        entry!(0x0050_21a0, fn_005021a0(Ptr) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00502180_tests_bit_0x20_at_0x24() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x28);
        e.mem.set_u8(this + 0x24, 0xdf);
        assert!(!e.call(0x0050_2180, &args![this]).bool());
        e.mem.set_u8(this + 0x24, 0x20);
        assert!(e.call(0x0050_2180, &args![this]).bool());
    }

    #[test]
    fn fn_005021a0_tests_bit_0_at_8() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.set_u8(this + 8, 0xff);
        assert_eq!(e.call(0x0050_21a0, &args![this]).u32(), 1);
        e.mem.set_u8(this + 8, 0xfe);
        assert_eq!(e.call(0x0050_21a0, &args![this]).u32(), 0);
    }
}
