//! `fallout/interface/run_007315e0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Menu` constructor (`00a1c4a0`), `BlackJackMenu`'s vtable (`010708fc`),
/// `` `eh vector constructor iterator' `` (`00ec782f`, stdcall: the array, the element
/// size, the count, the constructor and the destructor) with the card's constructor
/// (`00731710`) and destructor (`00914ae0`).
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
const BLACKJACK_MENU_VTABLE: u32 = 0x0107_08fc;
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const CARD_CONSTRUCTOR: u32 = 0x0073_1710;
const CARD_DESTRUCTOR: u32 = 0x0091_4ae0;
/// The trait id lookup by name (`00a00940`, cdecl: the name and -1) of `0106ed18` and
/// `0106ec90`, kept in the statics `011d9044` and `011d9048`.
const TRAIT_BY_NAME: u32 = 0x00a0_0940;
const LABEL_TRAIT_NAME: u32 = 0x0106_ed18;
const VALUE_TRAIT_NAME: u32 = 0x0106_ec90;
const LABEL_TRAIT_011D9044: u32 = 0x011d_9044;
const VALUE_TRAIT_011D9048: u32 = 0x011d_9048;
/// The casino (`011d9058`), the player (`011dea3c`) and
/// `Actor::GetItemCountinContainer` (`00575610`, `RET 4`); `004839c0` (cdecl, one word).
const CASINO_011D9058: u32 = 0x011d_9058;
const PLAYER: u32 = 0x011d_ea3c;
const GET_ITEM_COUNT: u32 = 0x0057_5610;
const FORM_004839C0: u32 = 0x0048_39c0;

// Translated from 007315e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007315e0 - the C++ exception-unwinding frame (FS:[0] around the base and the cards) is not translated.
/// The `BlackJackMenu` constructor (unnamed in the engine map; `thiscall`): the `Menu`
/// constructor, the vtable, the 0xd0 cards at +0x90 (0x10 bytes each), the two trait ids
/// looked up by name into their statics, state (+0x28) 10, no last tick (+0x7c), the
/// chips held (+0x74, the player's count of the casino's chip, `007316d0`) when the casino
/// has a chip, and the flags cleared (`007316f0`). Answers `this`.
pub fn fn_007315e0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, BLACKJACK_MENU_VTABLE);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            base + 0x90,
            0x10u32,
            0xd0u32,
            CARD_CONSTRUCTOR,
            CARD_DESTRUCTOR
        ],
    );
    let label = e
        .call(TRAIT_BY_NAME, &args![LABEL_TRAIT_NAME, 0xffff_ffffu32])
        .u32();
    e.set_global(LABEL_TRAIT_011D9044, label);
    let value = e
        .call(TRAIT_BY_NAME, &args![VALUE_TRAIT_NAME, 0xffff_ffffu32])
        .u32();
    e.set_global(VALUE_TRAIT_011D9048, value);
    e.mem.set_u32(base + 0x28, 0xa);
    e.mem.set_u32(base + 0x7c, 0);
    let casino = e.global::<u32>(CASINO_011D9058);
    if fn_007316d0(e, Ptr::new(casino)) != 0 {
        let casino = e.global::<u32>(CASINO_011D9058);
        let chip = fn_007316d0(e, Ptr::new(casino));
        let player = e.global::<u32>(PLAYER);
        let chips = e.call(GET_ITEM_COUNT, &args![player, chip]).u32();
        e.mem.set_u32(base + 0x74, chips);
    }
    fn_007316f0(e, this);
    this
}

// Translated from 007316d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, on a casino; its chip): `004839c0` of the word at +0x214.
pub fn fn_007316d0(e: &mut Engine, this: Ptr) -> u32 {
    let word = e.mem.u32(this.addr() + 0x214);
    e.call(FORM_004839C0, &args![word]).u32()
}

// Translated from 007316f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`, on the blackjack menu): clears the flags word (+0xdb0).
pub fn fn_007316f0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0xdb0, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0073_15e0, fn_007315e0(Ptr) -> Ptr),
        entry!(0x0073_16d0, fn_007316d0(Ptr) -> u32),
        entry!(0x0073_16f0, fn_007316f0(Ptr)),
    ]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn constructor_sets_up_the_table() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        for address in [MENU_CONSTRUCT, VECTOR_CONSTRUCTOR_ITERATOR, GET_ITEM_COUNT] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(TRAIT_BY_NAME, |_, a| (a[0] & 0xffff).into_ret());
        e.register(FORM_004839C0, |_, a| a[0].into_ret());
        e.register(GET_ITEM_COUNT, |_, a| (a[1] + 1).into_ret());
        let casino = e.mem.alloc(0x220);
        e.set_global(CASINO_011D9058, casino);
        e.set_global(PLAYER, 0x9000u32);
        let this = e.mem.alloc(0xe80);
        e.mem.set_u32(this + 0xdb0, 0xffff);
        e.mem.set_u32(this + 0x74, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0073_15e0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), BLACKJACK_MENU_VTABLE);
        assert_eq!(e.global::<u32>(LABEL_TRAIT_011D9044), 0xed18);
        assert_eq!(e.global::<u32>(VALUE_TRAIT_011D9048), 0xec90);
        assert_eq!(e.mem.u32(this + 0x28), 0xa);
        assert_eq!(e.mem.u32(this + 0xdb0), 0);
        // The casino has no chip: the count is left alone.
        assert_eq!(e.mem.u32(this + 0x74), 0x77);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|(a, w)| *a == VECTOR_CONSTRUCTOR_ITERATOR
            && *w == vec![this + 0x90, 0x10, 0xd0, CARD_CONSTRUCTOR, CARD_DESTRUCTOR]));
        e.mem.set_u32(casino + 0x214, 0x4100);
        e.call(0x0073_15e0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x74), 0x4101);
    }
}
