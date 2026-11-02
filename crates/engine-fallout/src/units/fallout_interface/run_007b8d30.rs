//! `fallout/interface/run_007b8d30` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `Menu` constructor (`00a1c4a0`), `RouletteMenu`'s vtable (`01075f7c`, RTTI
/// `.?AVRouletteMenu@@`), `` `eh vector constructor iterator' `` (`00ec782f`, stdcall)
/// with the element constructor `007b8fa0` and destructor `007b9000` (0x9f elements of
/// 0x64 bytes at +0x5c), the `NiPointer` constructor (`00633c90`, `RET 4`), the trait id
/// lookup by name (`00a00940`, cdecl: the name and -1) of `0106ed18` and `"_X"`
/// (`01075f74`) into `011da870` and `011da86c`, the allocator (`00401000`) and the list
/// constructor (`0096a2d0`), and the player (`011dea3c`).
const MENU_CONSTRUCT: u32 = 0x00a1_c4a0;
const ROULETTE_MENU_VTABLE: u32 = 0x0107_5f7c;
const VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const ELEMENT_CONSTRUCT_007B8FA0: u32 = 0x007b_8fa0;
const ELEMENT_DESTRUCT_007B9000: u32 = 0x007b_9000;
const NI_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const TRAIT_BY_NAME: u32 = 0x00a0_0940;
const LABEL_TRAIT_NAME: u32 = 0x0106_ed18;
const X_TRAIT_NAME: u32 = 0x0107_5f74;
const LABEL_TRAIT_011DA870: u32 = 0x011d_a870;
const X_TRAIT_011DA86C: u32 = 0x011d_a86c;
const ALLOCATE: u32 = 0x0040_1000;
const LIST_CONSTRUCT_0096A2D0: u32 = 0x0096_a2d0;
const PLAYER: u32 = 0x011d_ea3c;

// Translated from 007b8d30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007b8d30 - the C++ exception-unwinding frame (FS:[0] around the base, the elements, the pointer and the list) is not translated.
/// The `RouletteMenu` constructor (unnamed in the engine map; `thiscall`): the `Menu`
/// constructor, the vtable, the 0x9f elements at +0x5c, an empty `NiPointer` at
/// +0x3f7c, the two trait ids, a new list (8 bytes) at +0x3e78; state (+0x28) 4, +0x3f70,
/// the byte +0x400d and +0x58 cleared, the floats +0x3fd4..+0x3fdc and +0x3f90, +0x3f94
/// 0.0, the words +0x3fe0..+0x3ffc cleared, and +0x4010 the player's `007b8f10`.
/// Answers `this`.
pub fn fn_007b8d30(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(MENU_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, ROULETTE_MENU_VTABLE);
    e.call(
        VECTOR_CONSTRUCTOR_ITERATOR,
        &args![
            base + 0x5c,
            0x64u32,
            0x9fu32,
            ELEMENT_CONSTRUCT_007B8FA0,
            ELEMENT_DESTRUCT_007B9000
        ],
    );
    e.call(NI_POINTER_CONSTRUCT, &args![base + 0x3f7c, 0u32]);
    let label = e
        .call(TRAIT_BY_NAME, &args![LABEL_TRAIT_NAME, 0xffff_ffffu32])
        .u32();
    e.set_global(LABEL_TRAIT_011DA870, label);
    let x = e
        .call(TRAIT_BY_NAME, &args![X_TRAIT_NAME, 0xffff_ffffu32])
        .u32();
    e.set_global(X_TRAIT_011DA86C, x);
    let block = e.call(ALLOCATE, &args![8u32]).u32();
    let list = if block != 0 {
        e.call(LIST_CONSTRUCT_0096A2D0, &args![block]).u32()
    } else {
        0
    };
    e.mem.set_u32(base + 0x3e78, list);
    e.mem.set_u32(base + 0x28, 4);
    e.mem.set_u32(base + 0x3f70, 0);
    e.mem.set_u8(base + 0x400d, 0);
    e.mem.set_u32(base + 0x58, 0);
    let zero = x87_store32(0.0);
    for offset in [0x3fd4u32, 0x3fd8, 0x3fdc] {
        e.mem.set_u32(base + offset, zero);
    }
    for offset in [
        0x3fe0u32, 0x3fe4, 0x3fe8, 0x3ffc, 0x3fec, 0x3ff0, 0x3ff4, 0x3ff8,
    ] {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x3f90, zero);
    e.mem.set_u32(base + 0x3f94, zero);
    let player = e.global::<u32>(PLAYER);
    let word = fn_007b8f10(e, Ptr::new(player));
    e.mem.set_u32(base + 0x4010, word);
    this
}

// Translated from 007b8f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`thiscall`; on the player): the word at +0x630.
pub fn fn_007b8f10(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x630)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x007b_8d30, fn_007b8d30(Ptr) -> Ptr),
        entry!(0x007b_8f10, fn_007b8f10(Ptr) -> u32),
    ]
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    #[test]
    fn roulette_menu_constructor() {
        let mut e = Engine::new();
        e.map(0x011d_a000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        for address in [
            MENU_CONSTRUCT,
            VECTOR_CONSTRUCTOR_ITERATOR,
            NI_POINTER_CONSTRUCT,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(TRAIT_BY_NAME, |_, a| (a[0] & 0xffff).into_ret());
        e.register(ALLOCATE, |_, _| 0x8800u32.into_ret());
        e.register(LIST_CONSTRUCT_0096A2D0, |_, a| (a[0] + 1).into_ret());
        let player = e.mem.alloc(0x640);
        e.mem.set_u32(player + 0x630, 0x6300);
        e.set_global(PLAYER, player);
        let this = e.mem.alloc(0x4014);
        e.mem.set_u32(this + 0x3fec, 5);
        e.mem.set_u32(this + 0x3f94, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x007b_8d30, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), ROULETTE_MENU_VTABLE);
        assert_eq!(e.global::<u32>(X_TRAIT_011DA86C), 0x5f74);
        assert_eq!(e.mem.u32(this + 0x3e78), 0x8801);
        assert_eq!(e.mem.u32(this + 0x28), 4);
        assert_eq!(e.mem.u32(this + 0x3fec), 0);
        assert_eq!(e.mem.u32(this + 0x3f94), 0);
        assert_eq!(e.mem.u32(this + 0x4010), 0x6300);
        assert!(e.call_log.unwrap().contains(&(
            VECTOR_CONSTRUCTOR_ITERATOR,
            vec![
                this + 0x5c,
                0x64,
                0x9f,
                ELEMENT_CONSTRUCT_007B8FA0,
                ELEMENT_DESTRUCT_007B9000
            ]
        )));
    }
}
