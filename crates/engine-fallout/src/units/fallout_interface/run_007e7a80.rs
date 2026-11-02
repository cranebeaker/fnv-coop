//! `fallout/interface/run_007e7a80` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Menu` constructor (`00a1c4a0`) and `TraitSelectMenu`'s vtable (`01077abc`, RTTI
/// `.?AVTraitSelectMenu@@`).
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
const TRAIT_SELECT_MENU_VTABLE: u32 = 0x0107_7abc;

// Translated from 007e7a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TraitSelectMenu` constructor (unnamed in the engine map; `thiscall`): the
/// `Menu` constructor, the vtable, and -1 at +0x78 and +0x7c. Answers `this`.
pub fn fn_007e7a80(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MENU_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), TRAIT_SELECT_MENU_VTABLE);
    e.mem.set_u32(this.addr() + 0x78, 0xffff_ffff);
    e.mem.set_u32(this.addr() + 0x7c, 0xffff_ffff);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007e_7a80, fn_007e7a80(Ptr) -> Ptr)]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn trait_select_menu_constructor() {
        let mut e = Engine::new();
        e.register(MENU_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x88);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007e_7a80, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), TRAIT_SELECT_MENU_VTABLE);
        assert_eq!(e.mem.u32(this + 0x78), 0xffff_ffff);
        assert_eq!(e.mem.u32(this + 0x7c), 0xffff_ffff);
        assert!(e.call_log.unwrap().contains(&(MENU_CONSTRUCT, vec![this])));
    }
}
