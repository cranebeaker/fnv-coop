//! `fallout/magic/associateditemeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `AssociatedItemEffect` (Xbox PDB): an `ActiveEffect` (0x48 bytes) with the
    /// associated item; 0x4c bytes (the PC `Copy` allocates 0x4c).
    pub struct AssociatedItemEffect: 0x4c {
        /// `pAssociatedItem` (Xbox PDB): `TESForm*`.
        0x48 pAssociatedItem: u32,
    }
}

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00806c20` (thiscall on a new 0x4c-byte block: caster, spell, effect item; unnamed):
/// the `AssociatedItemEffect` constructor `Copy` uses.
const CONSTRUCTOR_00806C20: u32 = 0x0080_6c20;
/// `ActiveEffect::CopyData` (`00804090`, thiscall: the target effect).
const ACTIVE_EFFECT_COPY_DATA: u32 = 0x0080_4090;
/// `ActiveEffect::SaveGame` (`008050d0`, thiscall: one word).
const ACTIVE_EFFECT_SAVE_GAME: u32 = 0x0080_50d0;
/// `ActiveEffect::LoadGame` (`00805330`, thiscall: one word).
const ACTIVE_EFFECT_LOAD_GAME: u32 = 0x0080_5330;
/// `ActiveEffect::InitLoadGame` (`00805670`, thiscall: one word).
const ACTIVE_EFFECT_INIT_LOAD_GAME: u32 = 0x0080_5670;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, vfDelta, source type, target type, isReference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `ActiveEffect`'s RTTI type descriptor (`.?AVActiveEffect@@`).
const ACTIVE_EFFECT_TYPE: u32 = 0x011a_0d1c;
/// `AssociatedItemEffect`'s RTTI type descriptor (`.?AVAssociatedItemEffect@@`).
const ASSOCIATED_ITEM_EFFECT_TYPE: u32 = 0x011a_0d60;
/// `0084e3a0` (thiscall, returns a word; unnamed getter, here of the item).
const GETTER_0084E3A0: u32 = 0x0084_e3a0;
/// The `TESSaveLoadGame` pointer (`011de45c`).
const SAVE_LOAD_GAME_POINTER: u32 = 0x011d_e45c;
/// `TESSaveLoadGame::SaveNumericID` (`00857a10`, address, size).
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
/// `TESSaveLoadGame::LoadNumericID` (`00857aa0`, address, size).
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
/// `004839c0` (cdecl, one word: a form id; returns the form).
const LOOKUP_FORM_004839C0: u32 = 0x0048_39c0;

// Translated from 00806ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00806ca0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `AssociatedItemEffect::Copy` (Xbox PDB): a new 0x4c-byte effect (`operator new`, then
/// `00806c20` with the caster +0x28, spell +8 and effect item +0xc; null when the
/// allocation fails), given this effect's data through `CopyData` (slot +0x44); returns it.
pub fn associated_item_effect_copy(e: &mut Engine, this: Ptr<AssociatedItemEffect>) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let base = this.addr();
        let item = e.mem.u32(base + 0xc);
        let spell = e.mem.u32(base + 8);
        let caster = e.mem.u32(base + 0x28);
        e.call(CONSTRUCTOR_00806C20, &args![block, caster, spell, item])
            .u32()
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

// Translated from 00806d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AssociatedItemEffect::CopyData` (Xbox PDB): `ActiveEffect::CopyData`, then, when
/// `target` is an `AssociatedItemEffect` (`__RTDynamicCast` from `ActiveEffect`), the
/// associated item.
pub fn associated_item_effect_copy_data(
    e: &mut Engine,
    this: Ptr<AssociatedItemEffect>,
    target: u32,
) {
    e.call(ACTIVE_EFFECT_COPY_DATA, &args![this, target]);
    let cast = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                target,
                0u32,
                ACTIVE_EFFECT_TYPE,
                ASSOCIATED_ITEM_EFFECT_TYPE,
                0u32
            ],
        )
        .u32();
    if cast != 0 {
        let item = e.get(this, AssociatedItemEffect::pAssociatedItem);
        e.mem.set_u32(cast + 0x48, item);
    }
}

// Translated from 00806de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AssociatedItemEffect::SaveGame` (Xbox PDB): `ActiveEffect::SaveGame`, then the item's
/// `0084e3a0` word (0 without an item) through `SaveNumericID`.
pub fn associated_item_effect_save_game(
    e: &mut Engine,
    this: Ptr<AssociatedItemEffect>,
    argument: u32,
) {
    e.call(ACTIVE_EFFECT_SAVE_GAME, &args![this, argument]);
    e.with_stack(4, |e, id| {
        let item = e.get(this, AssociatedItemEffect::pAssociatedItem);
        let value = if item == 0 {
            0
        } else {
            e.call(GETTER_0084E3A0, &args![item]).u32()
        };
        e.mem.set_u32(id.addr(), value);
        let game = e.global::<u32>(SAVE_LOAD_GAME_POINTER);
        e.call(SAVE_NUMERIC_ID, &args![game, id, 4u32]);
    });
}

// Translated from 00806e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AssociatedItemEffect::LoadGame` (Xbox PDB): `ActiveEffect::LoadGame`, then the item's
/// id (`LoadNumericID`) into the associated item.
pub fn associated_item_effect_load_game(
    e: &mut Engine,
    this: Ptr<AssociatedItemEffect>,
    argument: u32,
) {
    e.call(ACTIVE_EFFECT_LOAD_GAME, &args![this, argument]);
    e.with_stack(4, |e, id| {
        let game = e.global::<u32>(SAVE_LOAD_GAME_POINTER);
        e.call(LOAD_NUMERIC_ID, &args![game, id, 4u32]);
        let value = e.mem.u32(id.addr());
        e.set(this, AssociatedItemEffect::pAssociatedItem, value);
    });
}

// Translated from 00806e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `AssociatedItemEffect::InitLoadGame` (Xbox PDB): `ActiveEffect::InitLoadGame`, then a
/// set item id becomes its form (`004839c0`).
pub fn associated_item_effect_init_load_game(
    e: &mut Engine,
    this: Ptr<AssociatedItemEffect>,
    argument: u32,
) {
    e.call(ACTIVE_EFFECT_INIT_LOAD_GAME, &args![this, argument]);
    let id = e.get(this, AssociatedItemEffect::pAssociatedItem);
    if id != 0 {
        let form = e.call(LOOKUP_FORM_004839C0, &args![id]).u32();
        e.set(this, AssociatedItemEffect::pAssociatedItem, form);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0080_6ca0,
            associated_item_effect_copy(Ptr<AssociatedItemEffect>) -> u32
        ),
        entry!(
            0x0080_6d50,
            associated_item_effect_copy_data(Ptr<AssociatedItemEffect>, u32)
        ),
        entry!(
            0x0080_6de0,
            associated_item_effect_save_game(Ptr<AssociatedItemEffect>, u32)
        ),
        entry!(
            0x0080_6e30,
            associated_item_effect_load_game(Ptr<AssociatedItemEffect>, u32)
        ),
        entry!(
            0x0080_6e70,
            associated_item_effect_init_load_game(Ptr<AssociatedItemEffect>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    /// An effect whose vtable slot +0x44 is a double at `0x0300_0044`; the base-class
    /// callees are doubles.
    fn rig() -> (Engine, Ptr<AssociatedItemEffect>) {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.set_global(SAVE_LOAD_GAME_POINTER, 0x0500_0000u32);
        let vtable = e.mem.alloc(0x60);
        e.mem.set_u32(vtable + 0x44, 0x0300_0044);
        answer(&mut e, 0x0300_0044, 0);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this, vtable);
        for addr in [
            ACTIVE_EFFECT_COPY_DATA,
            ACTIVE_EFFECT_SAVE_GAME,
            ACTIVE_EFFECT_LOAD_GAME,
            ACTIVE_EFFECT_INIT_LOAD_GAME,
        ] {
            answer(&mut e, addr, 0);
        }
        e.call_log = Some(vec![]);
        (e, Ptr::new(this))
    }

    #[test]
    fn copy_allocates_constructs_and_copies_data() {
        let (mut e, this) = rig();
        let base = this.addr();
        e.mem.set_u32(base + 8, 0x08);
        e.mem.set_u32(base + 0xc, 0x0c);
        e.mem.set_u32(base + 0x28, 0x28);
        answer(&mut e, OPERATOR_NEW, 0x7000);
        answer(&mut e, CONSTRUCTOR_00806C20, 0x7004);
        assert_eq!(e.call(0x0080_6ca0, &args![this]).u32(), 0x7004);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x4c]]);
        assert_eq!(
            calls(&e, CONSTRUCTOR_00806C20),
            vec![vec![0x7000, 0x28, 0x08, 0x0c]]
        );
        assert_eq!(calls(&e, 0x0300_0044), vec![vec![base, 0x7004]]);
        answer(&mut e, OPERATOR_NEW, 0);
        assert_eq!(e.call(0x0080_6ca0, &args![this]).u32(), 0);
        assert_eq!(calls(&e, CONSTRUCTOR_00806C20).len(), 1);
        assert_eq!(calls(&e, 0x0300_0044)[1], vec![base, 0]);
    }

    #[test]
    fn copy_data_copies_the_item_only_into_an_associated_item_effect() {
        for cast in [false, true] {
            let (mut e, this) = rig();
            e.set(this, AssociatedItemEffect::pAssociatedItem, 0x4848);
            let target = e.mem.alloc(0x4c);
            answer(&mut e, RT_DYNAMIC_CAST, if cast { target } else { 0 });
            e.call(0x0080_6d50, &args![this, target]);
            assert_eq!(
                calls(&e, ACTIVE_EFFECT_COPY_DATA),
                vec![vec![this.addr(), target]]
            );
            assert_eq!(
                calls(&e, RT_DYNAMIC_CAST),
                vec![vec![
                    target,
                    0,
                    ACTIVE_EFFECT_TYPE,
                    ASSOCIATED_ITEM_EFFECT_TYPE,
                    0
                ]]
            );
            let expected = if cast { 0x4848 } else { 0 };
            assert_eq!(e.mem.u32(target + 0x48), expected);
        }
    }

    #[test]
    fn save_game_writes_the_item_id_after_the_effect() {
        for (item, expected) in [(0u32, 0u32), (0x4848, 0x99)] {
            let (mut e, this) = rig();
            e.set(this, AssociatedItemEffect::pAssociatedItem, item);
            answer(&mut e, GETTER_0084E3A0, 0x99);
            let saved = Rc::new(Cell::new(0xffff_ffffu32));
            let seen = Rc::clone(&saved);
            e.register_double(SAVE_NUMERIC_ID, move |e, a| {
                assert_eq!((a[0], a[2]), (0x0500_0000, 4));
                seen.set(e.mem.u32(a[1]));
                Ret::default()
            });
            e.call(0x0080_6de0, &args![this, 0x77u32]);
            assert_eq!(saved.get(), expected);
            let log = e.call_log.take().unwrap();
            let base = log
                .iter()
                .position(|c| c.0 == ACTIVE_EFFECT_SAVE_GAME)
                .unwrap();
            let save = log.iter().position(|c| c.0 == SAVE_NUMERIC_ID).unwrap();
            assert!(base < save);
            assert_eq!(log[base].1, vec![this.addr(), 0x77]);
        }
    }

    #[test]
    fn load_game_reads_the_item_id() {
        let (mut e, this) = rig();
        e.register(LOAD_NUMERIC_ID, |e, a| {
            e.mem.set_u32(a[1], 0x4242);
            Ret::default()
        });
        e.call(0x0080_6e30, &args![this, 0x77u32]);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_LOAD_GAME),
            vec![vec![this.addr(), 0x77]]
        );
        assert_eq!(e.get(this, AssociatedItemEffect::pAssociatedItem), 0x4242);
    }

    #[test]
    fn init_load_game_resolves_a_set_item() {
        for (id, expected) in [(0u32, 0u32), (0x4242, 0x9000)] {
            let (mut e, this) = rig();
            e.set(this, AssociatedItemEffect::pAssociatedItem, id);
            answer(&mut e, LOOKUP_FORM_004839C0, 0x9000);
            e.call(0x0080_6e70, &args![this, 0x77u32]);
            assert_eq!(
                calls(&e, ACTIVE_EFFECT_INIT_LOAD_GAME),
                vec![vec![this.addr(), 0x77]]
            );
            assert_eq!(e.get(this, AssociatedItemEffect::pAssociatedItem), expected);
        }
    }
}
