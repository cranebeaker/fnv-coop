//! `BSSystemUtilities/run_00af3b20` (the engine map's run of unnamed functions from
//! `00af3b20`), subsystem `BSSystemUtilities`: its functions in FalloutNV.exe 1.4.0.525,
//! translated (docs/ENGINE_CRATE.md). The system info utility's constructor and destructor
//! body.

#[allow(unused_imports)]
use crate::prelude::*;

/// The system info utility's vtable (`010a4328`).
const SYS_INFO_VTABLE: u32 = 0x010a_4328;

// Translated from 00af3b20 (disassembled, FalloutNV.exe 1.4.0.525)
/// The system info utility's constructor (no label; `thiscall`, plain `RET`): the vtable
/// `010a4328` and 0x10 at +4. Answers `this`.
pub fn fn_00af3b20(e: &mut Engine, this: u32) -> u32 {
    e.mem.set_u32(this, SYS_INFO_VTABLE);
    e.mem.set_u32(this.wrapping_add(4), 0x10);
    this
}

// Translated from 00af3b50 (disassembled, FalloutNV.exe 1.4.0.525)
/// The system info utility's destructor body (no label; `thiscall`, plain `RET`; the scalar
/// deleting destructor `00af3ba0` calls it): stores the vtable `010a4328` again and does
/// nothing else.
pub fn fn_00af3b50(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, SYS_INFO_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00af_3b20, fn_00af3b20(u32) -> u32),
        entry!(0x00af_3b50, fn_00af3b50(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_constructor_and_destructor() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        assert_eq!(e.call(0x00af_3b20, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SYS_INFO_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x10);
        e.mem.set_u32(this, 0);
        e.call(0x00af_3b50, &args![this]);
        assert_eq!(e.mem.u32(this), SYS_INFO_VTABLE);
        assert_eq!(e.mem.u32(this + 4), 0x10);
    }
}
