//! `fallout/interface/run_00753230` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `Menu`'s constructor (`00a1c4a0`, thiscall).
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
/// The list box constructor run on `+0x58` (`00721060`, thiscall).
const LIST_BOX_CONSTRUCT: u32 = 0x0072_1060;
/// `Tile::AddUserTrait` (engine map), cdecl (name, -1): the trait id.
const ADD_USER_TRAIT: u32 = 0x00a0_0940;
/// `CharGenMenu`'s vtable (RTTI `.?AVCharGenMenu@@`).
const CHAR_GEN_MENU_VTABLE: u32 = 0x0107_1bb4;
/// The float copied to `fPrevHighlightIndex` (`01012054`).
const INITIAL_HIGHLIGHT: u32 = 0x0101_2054;

/// The user traits the constructor registers: the name's address and the
/// `CharGenMenu::iUT_*` static (Xbox PDB) that keeps the id, in the exe's order.
const USER_TRAITS: [(u32, u32); 6] = [
    // "_BaseValue" → iUT_BaseValue
    (0x0107_1ba0, 0x011d_9210),
    // "_ExtraValue" → iUT_ExtraValue
    (0x0107_1b94, 0x011d_9214),
    // "_AddedValue" → iUT_AddedValue
    (0x0107_1b88, 0x011d_9224),
    // "_CurrPoints" → iUT_CurrPoints
    (0x0107_1b7c, 0x011d_9218),
    // "_MaxPoints" → iUT_MaxPoints
    (0x0107_1b70, 0x011d_921c),
    // "_selected" → iUT_Selected
    (0x0107_1b64, 0x011d_9220),
];

// Translated from 00753230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00753230 - the C++ exception-unwinding frame (FS:[0]) that destroys the base when a member constructor throws is not translated
/// `CharGenMenu::CharGenMenu` (the vtable it sets is `CharGenMenu`'s by
/// its RTTI; unnamed in the engine map), thiscall without stack words:
/// constructs the `Menu` base, sets the vtable, constructs the list box
/// `lbItemList` (+0x58), registers the six user traits (`_BaseValue`,
/// `_ExtraValue`, `_AddedValue`, `_CurrPoints`, `_MaxPoints`,
/// `_selected`) into the class's statics, clears `iCurrPoints` (+0x48),
/// copies the float at `01012054` to `fPrevHighlightIndex` (+0x50;
/// `FLD`/`FSTP`: a signalling NaN comes out quiet) and clears
/// `bShowTaggedSkills` (+0x55) and `bOnlyAddValues` (+0x54). Answers `this`.
pub fn fn_00753230(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, CHAR_GEN_MENU_VTABLE);
    e.call(LIST_BOX_CONSTRUCT, &args![base + 0x58]);
    for (name, slot) in USER_TRAITS {
        let id = e.call(ADD_USER_TRAIT, &args![name, 0xffff_ffffu32]).u32();
        e.set_global(slot, id);
    }
    e.mem.set_u32(base + 0x48, 0);
    let highlight = x87_quiet_bits(e.global::<u32>(INITIAL_HIGHLIGHT));
    e.mem.set_u32(base + 0x50, highlight);
    e.mem.set_u8(base + 0x55, 0);
    e.mem.set_u8(base + 0x54, 0);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00753230, fn_00753230(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructs_the_menu_and_registers_its_traits() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x011d_9000, 0x1000);
        e.set_global(INITIAL_HIGHLIGHT, 0xff80_0001u32);
        e.register(MENU_CONSTRUCT, |_, _| Ret::default());
        e.register(LIST_BOX_CONSTRUCT, |_, _| Ret::default());
        e.register(ADD_USER_TRAIT, |_, a| Ret {
            eax: a[0] & 0xff,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x88);
        e.mem.set_u32(this + 0x48, 7);
        e.mem.set_u16(this + 0x54, 0xffff);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0075_3230, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        let of = |callee: u32| -> Vec<Vec<u32>> {
            log.iter()
                .filter(|c| c.0 == callee)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(of(MENU_CONSTRUCT), vec![vec![this]]);
        assert_eq!(of(LIST_BOX_CONSTRUCT), vec![vec![this + 0x58]]);
        let traits = of(ADD_USER_TRAIT);
        assert_eq!(traits.len(), 6);
        assert_eq!(traits[0], vec![0x0107_1ba0, 0xffff_ffff]);
        assert_eq!(traits[5], vec![0x0107_1b64, 0xffff_ffff]);
        assert_eq!(e.global::<u32>(0x011d_9210), 0xa0);
        assert_eq!(e.global::<u32>(0x011d_9224), 0x88);
        assert_eq!(e.global::<u32>(0x011d_9220), 0x64);
        assert_eq!(e.mem.u32(this), CHAR_GEN_MENU_VTABLE);
        assert_eq!(e.mem.u32(this + 0x48), 0);
        assert_eq!(e.mem.u32(this + 0x50), 0xffc0_0001);
        assert_eq!(e.mem.u16(this + 0x54), 0);
    }
}
