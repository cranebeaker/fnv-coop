//! `BSSystemUtilities/run_00af3a40` (the engine map's run of unnamed functions from
//! `00af3a40`), subsystem `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525,
//! translated (docs/ENGINE_CRATE.md). The disc boot utility's constructor and destructor body.

#[allow(unused_imports)]
use crate::prelude::*;

/// The disc boot utility's vtable (`010a4318`).
const DISC_BOOT_VTABLE: u32 = 0x010a_4318;

// Translated from 00af3a40 (disassembled, FalloutNV.exe 1.4.0.525)
/// The disc boot utility's constructor (no label; `thiscall`, plain `RET`): the vtable
/// `010a4318`, the word at +0x44, the byte at +0x48, the word at +0x4c and the byte at +4
/// (the name's first character) cleared, in that order. Answers `this`.
pub fn fn_00af3a40(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, DISC_BOOT_VTABLE);
    e.mem.set_u32(this.wrapping_add(0x44), 0);
    e.mem.set_u8(this.wrapping_add(0x48), 0);
    e.mem.set_u32(this.wrapping_add(0x4c), 0);
    e.mem.set_u8(this.wrapping_add(4), 0);
    this
}

// Translated from 00af3a80 (disassembled, FalloutNV.exe 1.4.0.525)
/// The disc boot utility's destructor body (no label; `thiscall`, plain `RET`; the scalar
/// deleting destructor `00af3af0` calls it): stores the vtable `010a4318` again and does
/// nothing else.
pub fn fn_00af3a80(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, DISC_BOOT_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_3a40, fn_00af3a40(u32) -> u32),
        entry!(0x00af_3a80, fn_00af3a80(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_constructor_and_destructor() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x50);
        for offset in (0..0x50).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00af_3a40, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), DISC_BOOT_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0xffff_ff00);
        assert_eq!(e.mem.u32(this + 8), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x44), 0);
        assert_eq!(e.mem.u32(this + 0x48), 0xffff_ff00);
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        e.mem.set_u32(this, 0);
        e.call(0x00af_3a80, &args![this]);
        assert_eq!(e.mem.u32(this), DISC_BOOT_VTABLE);
    }
}
