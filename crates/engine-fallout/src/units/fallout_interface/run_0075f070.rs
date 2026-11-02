//! `fallout/interface/run_0075f070` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Menu` constructor (`00a1c4a0`), `CreditsMenu`'s vtable (`0107235c`, RTTI
/// `.?AVCreditsMenu@@`), the `NiPointer` constructor (`00633c90`, `RET 4`) and
/// assignment (`0066b0d0`, `RET 4`), the four-float set (`00414430`, `RET 0x10`), the
/// constructor of the object at +0x60 (`00761470`), the color constant `011a9be0` (four
/// words) and the static `011d9474`.
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
const CREDITS_MENU_VTABLE: u32 = 0x0107_235c;
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const FOUR_FLOATS_00414430: u32 = 0x0041_4430;
const FIELD_CONSTRUCT_00761470: u32 = 0x0076_1470;
const COLOR_011A9BE0: u32 = 0x011a_9be0;
const STATIC_011D9474: u32 = 0x011d_9474;

// Translated from 0075f070 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0075f070 - the C++ exception-unwinding frame (FS:[0] around the base and the pointer) is not translated.
/// The `CreditsMenu` constructor (unnamed in the engine map; `thiscall`): the `Menu`
/// constructor, the vtable, an empty `NiPointer` at +0x30, four zero floats at +0x50, the
/// object at +0x60 constructed (`00761470`); +0x28, +0x34, +0x38, +0x3c cleared, the
/// pointer at +0x30 cleared again, +0x40 and +0x44 set to 1, +0x48 and +0x4c cleared, the
/// four words at +0x50 copied from `011a9be0`, +0x2c and +0x70 cleared, the float at
/// +0x78 0.0, +0x7c cleared and the static `011d9474` cleared. Answers `this`.
pub fn fn_0075f070(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, CREDITS_MENU_VTABLE);
    e.call(NI_POINTER_CONSTRUCT, &args![base + 0x30, 0u32]);
    let zero = x87_store32(0.0);
    e.call(
        FOUR_FLOATS_00414430,
        &args![base + 0x50, zero, zero, zero, zero],
    );
    e.call(FIELD_CONSTRUCT_00761470, &args![base + 0x60]);
    for offset in [0x28u32, 0x34, 0x38, 0x3c] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(NI_POINTER_ASSIGN, &args![base + 0x30, 0u32]);
    e.mem.set_u32(base + 0x40, 1);
    e.mem.set_u32(base + 0x44, 1);
    e.mem.set_u32(base + 0x48, 0);
    e.mem.set_u32(base + 0x4c, 0);
    for i in 0..4 {
        let word = e.global::<u32>(COLOR_011A9BE0 + 4 * i);
        e.mem.set_u32(base + 0x50 + 4 * i, word);
    }
    e.mem.set_u32(base + 0x2c, 0);
    e.mem.set_u32(base + 0x70, 0);
    e.mem.set_u32(base + 0x78, zero);
    e.mem.set_u32(base + 0x7c, 0);
    e.set_global(STATIC_011D9474, 0u32);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0075_f070, fn_0075f070(Ptr) -> Ptr)]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn credits_menu_constructor() {
        let mut e = Engine::new();
        e.map(0x011a_9000, 0x1000);
        e.map(0x011d_9000, 0x1000);
        for address in [
            MENU_CONSTRUCT,
            NI_POINTER_CONSTRUCT,
            NI_POINTER_ASSIGN,
            FOUR_FLOATS_00414430,
            FIELD_CONSTRUCT_00761470,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        for i in 0..4 {
            e.set_global(COLOR_011A9BE0 + 4 * i, 0x100 + i);
        }
        e.set_global(STATIC_011D9474, 9u32);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x44, 7);
        e.mem.set_u32(this + 0x78, 0xffff_ffff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0075_f070, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), CREDITS_MENU_VTABLE);
        assert_eq!([e.mem.u32(this + 0x40), e.mem.u32(this + 0x44)], [1, 1]);
        assert_eq!(e.mem.u32(this + 0x58), 0x102);
        assert_eq!(e.mem.u32(this + 0x78), 0);
        assert_eq!(e.global::<u32>(STATIC_011D9474), 0);
        let log = e.call_log.unwrap();
        assert!(log.contains(&(NI_POINTER_ASSIGN, vec![this + 0x30, 0])));
        assert!(log.contains(&(FIELD_CONSTRUCT_00761470, vec![this + 0x60])));
    }
}
