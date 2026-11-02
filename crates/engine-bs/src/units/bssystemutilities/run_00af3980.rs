//! `BSSystemUtilities/run_00af3980` (the engine map's run of unnamed functions from
//! `00af3980`), subsystem `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525,
//! translated (docs/ENGINE_CRATE.md). The game data utility's constructor and destructor body.

#[allow(unused_imports)]
use crate::prelude::*;

/// The game data utility's vtable (`010a42fc`).
const GAME_DATA_VTABLE: u32 = 0x010a_42fc;

// Translated from 00af3980 (disassembled, FalloutNV.exe 1.4.0.525)
/// The game data utility's constructor (no label; `thiscall`, plain `RET`): the vtable
/// `010a42fc`, the bytes at +4 and +5 and the words at +8 and +0x84c cleared. Answers `this`.
pub fn fn_00af3980(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, GAME_DATA_VTABLE);
    e.mem.set_u8(this.wrapping_add(4), 0);
    e.mem.set_u8(this.wrapping_add(5), 0);
    e.mem.set_u32(this.wrapping_add(8), 0);
    e.mem.set_u32(this.wrapping_add(0x84c), 0);
    this
}

// Translated from 00af39c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The game data utility's destructor body (no label; `thiscall`, plain `RET`; the scalar
/// deleting destructor `00af3a10` calls it): stores the vtable `010a42fc` again and does
/// nothing else.
pub fn fn_00af39c0(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, GAME_DATA_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_3980, fn_00af3980(u32) -> u32),
        entry!(0x00af_39c0, fn_00af39c0(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_constructor_and_destructor() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x850);
        for offset in (0..0x850).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        assert_eq!(e.call(0x00af_3980, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), GAME_DATA_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0xffff_0000);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x848), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x84c), 0);
        e.mem.set_u32(this, 0);
        e.call(0x00af_39c0, &args![this]);
        assert_eq!(e.mem.u32(this), GAME_DATA_VTABLE);
    }
}
