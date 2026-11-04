//! `fallout/interface/run_00784a40` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Menu` constructor (`00a1c4a0`), `LevelUpMenu`'s vtable (`01073cdc`, RTTI
/// `.?AVLevelUpMenu@@`), the constructors of its members at +0x64 (`00721060`), +0x94
/// (`00786bb0`) and +0xc4 (`0096a2d0`), and the trait id lookup by name (`00a00940`,
/// cdecl: the name and -1).
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
const LEVEL_UP_MENU_VTABLE: u32 = 0x0107_3cdc;
const MEMBER_CONSTRUCT_00721060: u32 = 0x0072_1060;
const MEMBER_CONSTRUCT_00786BB0: u32 = 0x0078_6bb0;
const MEMBER_CONSTRUCT_0096A2D0: u32 = 0x0096_a2d0;
const TRAIT_BY_NAME: u32 = 0x00a0_0940;

/// The trait names looked up (`"_BaseValue"`, `"_ExtraValue"`, `"_AddedValue"`,
/// `"_CurrentPage"`, `"_CurrPoints"`, `"_MaxPoints"`, `"_Rank"`, `"_selected"`,
/// `"_TextAlpha"`) and the statics their ids are kept in, in the code's order.
const LEVEL_UP_TRAITS: [(u32, u32); 9] = [
    (0x0107_1ba0, 0x011d_9fe4),
    (0x0107_1b94, 0x011d_9fec),
    (0x0107_1b88, 0x011d_9ff0),
    (0x0107_3cc8, 0x011d_9fe8),
    (0x0107_1b7c, 0x011d_9fd8),
    (0x0107_1b70, 0x011d_9fd4),
    (0x0107_3cc0, 0x011d_9ff8),
    (0x0107_1b64, 0x011d_9fe0),
    (0x0107_3cb4, 0x011d_9ff4),
];

// Translated from 00784a40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00784a40 - the C++ exception-unwinding frame (FS:[0] around the base and the members) is not translated.
/// The `LevelUpMenu` constructor (unnamed in the engine map; `thiscall`): the `Menu`
/// constructor, the vtable, the members at +0x64, +0x94 and +0xc4 constructed, and the
/// nine trait ids looked up by name into their statics. Answers `this`.
pub fn fn_00784a40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, LEVEL_UP_MENU_VTABLE);
    e.call(MEMBER_CONSTRUCT_00721060, &args![base + 0x64]);
    e.call(MEMBER_CONSTRUCT_00786BB0, &args![base + 0x94]);
    e.call(MEMBER_CONSTRUCT_0096A2D0, &args![base + 0xc4]);
    for (name, holder) in LEVEL_UP_TRAITS {
        let id = e.call(TRAIT_BY_NAME, &args![name, 0xffff_ffffu32]).u32();
        e.set_global(holder, id);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0078_4a40, fn_00784a40(Ptr) -> Ptr)]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn level_up_menu_constructor() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        for address in [
            MENU_CONSTRUCT,
            MEMBER_CONSTRUCT_00721060,
            MEMBER_CONSTRUCT_00786BB0,
            MEMBER_CONSTRUCT_0096A2D0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(TRAIT_BY_NAME, |_, a| (a[0] & 0xffff).into_ret());
        let this = e.mem.alloc(0xcc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0078_4a40, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), LEVEL_UP_MENU_VTABLE);
        assert_eq!(e.global::<u32>(0x011d_9fe4), 0x1ba0);
        assert_eq!(e.global::<u32>(0x011d_9ff4), 0x3cb4);
        let log = e.call_log.unwrap();
        assert!(log.contains(&(MEMBER_CONSTRUCT_00786BB0, vec![this + 0x94])));
        assert!(log.contains(&(TRAIT_BY_NAME, vec![0x0107_3cc0, 0xffff_ffff])));
    }
}
