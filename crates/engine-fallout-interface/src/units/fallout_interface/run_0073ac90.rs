//! `fallout/interface/run_0073ac90` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0073ac90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the word at `this` plus 8 (the
/// address 8 bytes into the object `this` points to; nothing is read there).
pub fn fn_0073ac90(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr()).wrapping_add(8)
}

/// The `Menu` constructor (`00a1c4a0`) and `BookMenu`'s vtable (`01070ecc`, RTTI
/// `.?AVBookMenu@@`).
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
const BOOK_MENU_VTABLE: u32 = 0x0107_0ecc;

// Translated from 0073acb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `BookMenu` constructor (unnamed in the engine map; `thiscall`): the `Menu`
/// constructor, the vtable, the words at +0x28 and +0x2c cleared. Answers `this`.
pub fn fn_0073acb0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MENU_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), BOOK_MENU_VTABLE);
    e.mem.set_u32(this.addr() + 0x28, 0);
    e.mem.set_u32(this.addr() + 0x2c, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0073_ac90, fn_0073ac90(Ptr) -> u32),
        entry!(0x0073_acb0, fn_0073acb0(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answers_the_pointed_to_address_plus_8() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x1234_5678);
        assert_eq!(e.call(0x0073_ac90, &[this]).u32(), 0x1234_5680);
        e.mem.set_u32(this, 0xffff_fffc);
        assert_eq!(e.call(0x0073_ac90, &[this]).u32(), 4);
    }
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn book_menu_constructor() {
        let mut e = Engine::new();
        e.register(MENU_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x3c);
        e.mem.set_u32(this + 0x28, 5);
        e.mem.set_u32(this + 0x2c, 6);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_acb0, &args![this]).u32(), this);
        assert_eq!(
            [
                e.mem.u32(this),
                e.mem.u32(this + 0x28),
                e.mem.u32(this + 0x2c)
            ],
            [BOOK_MENU_VTABLE, 0, 0]
        );
        assert!(e.call_log.unwrap().contains(&(MENU_CONSTRUCT, vec![this])));
    }
}
