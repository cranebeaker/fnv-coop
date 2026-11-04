//! `fallout/interface/run_007c06f0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Menu` constructor (`00a1c4a0`), `SlotMachineMenu`'s vtable (`010764dc`, RTTI
/// `.?AVSlotMachineMenu@@`), `` `eh vector constructor iterator' `` (`00ec782f`,
/// stdcall) with the `NiPointer`-like element constructor `006694e0` and destructor
/// `0045cec0` (8 elements of 4 bytes at +0x114), the trait id lookup by name (`00a00940`)
/// of `0106ed18` and `0106ec90` into `011da99c` and `011da998`, the player
/// (`011dea3c`) and its `007b8f10`, the slot machine's casino (`011da9ac`), its chip
/// (`007316d0`) and `Actor::GetItemCountinContainer` (`00575610`, `RET 4`).
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
const SLOT_MACHINE_MENU_VTABLE: u32 = 0x0107_64dc;
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const ELEMENT_CONSTRUCT_006694E0: u32 = 0x0066_94e0;
const ELEMENT_DESTRUCT_0045CEC0: u32 = 0x0045_cec0;
const TRAIT_BY_NAME: u32 = 0x00a0_0940;
const LABEL_TRAIT_NAME: u32 = 0x0106_ed18;
const VALUE_TRAIT_NAME: u32 = 0x0106_ec90;
const LABEL_TRAIT_011DA99C: u32 = 0x011d_a99c;
const VALUE_TRAIT_011DA998: u32 = 0x011d_a998;
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_007B8F10: u32 = 0x007b_8f10;
const CASINO_011DA9AC: u32 = 0x011d_a9ac;
const CASINO_CHIP_007316D0: u32 = 0x0073_16d0;
const GET_ITEM_COUNT: u32 = 0x0057_5610;

// Translated from 007c06f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007c06f0 - the C++ exception-unwinding frame (FS:[0] around the base and the elements) is not translated.
/// The `SlotMachineMenu` constructor (unnamed in the engine map; `thiscall`): the `Menu`
/// constructor, the vtable, the 8 pointers at +0x114, the two trait ids; +0x28 and +0xf4
/// cleared, +0x154 the player's `007b8f10`, +0xfc 1, and the chips held (+0xd4) when the
/// casino has a chip. Answers `this`.
pub fn fn_007c06f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, SLOT_MACHINE_MENU_VTABLE);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            base + 0x114,
            4u32,
            8u32,
            ELEMENT_CONSTRUCT_006694E0,
            ELEMENT_DESTRUCT_0045CEC0
        ],
    );
    let label = e
        .call(TRAIT_BY_NAME, &args![LABEL_TRAIT_NAME, 0xffff_ffffu32])
        .u32();
    e.set_global(LABEL_TRAIT_011DA99C, label);
    let value = e
        .call(TRAIT_BY_NAME, &args![VALUE_TRAIT_NAME, 0xffff_ffffu32])
        .u32();
    e.set_global(VALUE_TRAIT_011DA998, value);
    e.mem.set_u32(base + 0x28, 0);
    e.mem.set_u32(base + 0xf4, 0);
    let player = e.global::<u32>(PLAYER);
    let word = e.call(PLAYER_007B8F10, &args![player]).u32();
    e.mem.set_u32(base + 0x154, word);
    e.mem.set_u32(base + 0xfc, 1);
    let casino = e.global::<u32>(CASINO_011DA9AC);
    if e.call(CASINO_CHIP_007316D0, &args![casino]).u32() != 0 {
        let casino = e.global::<u32>(CASINO_011DA9AC);
        let chip = e.call(CASINO_CHIP_007316D0, &args![casino]).u32();
        let player = e.global::<u32>(PLAYER);
        let chips = e.call(GET_ITEM_COUNT, &args![player, chip]).u32();
        e.mem.set_u32(base + 0xd4, chips);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x007c_06f0, fn_007c06f0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn slot_machine_menu_constructor() {
        let mut e = Engine::new();
        e.map(0x011d_a000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.register(MENU_CONSTRUCT, |_, _| Ret::default());
        e.register(VECTOR_CONSTRUCTOR_ITERATOR, |_, _| Ret::default());
        e.register(TRAIT_BY_NAME, |_, a| (a[0] & 0xffff).into_ret());
        e.register(PLAYER_007B8F10, |_, _| 0x6300u32.into_ret());
        e.register(CASINO_CHIP_007316D0, |_, a| a[0].into_ret());
        e.register(GET_ITEM_COUNT, |_, a| (a[1] + 2).into_ret());
        let this = e.mem.alloc(0x158);
        e.mem.set_u32(this + 0xd4, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007c_06f0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), SLOT_MACHINE_MENU_VTABLE);
        assert_eq!(e.global::<u32>(VALUE_TRAIT_011DA998), 0xec90);
        assert_eq!(e.mem.u32(this + 0x154), 0x6300);
        assert_eq!(e.mem.u32(this + 0xfc), 1);
        assert_eq!(e.mem.u32(this + 0xd4), 0x77);
        e.set_global(CASINO_011DA9AC, 0x4100u32);
        e.call(0x007c_06f0, &args![this]);
        assert_eq!(e.mem.u32(this + 0xd4), 0x4102);
    }
}
