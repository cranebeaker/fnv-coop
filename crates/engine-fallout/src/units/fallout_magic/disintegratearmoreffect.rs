//! `fallout/magic/disintegratearmoreffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::activeeffect::{load_bytes, save_bytes, save_version};

layout! {
    /// `DisintegrateArmorEffect` (Xbox PDB), 0x4c bytes: an `ActiveEffect` (0x48) and the
    /// armor it damages.
    pub struct DisintegrateArmorEffect: 0x4c {
        /// `pArmor` (Xbox PDB): `ItemChange*`.
        0x48 pArmor: u32,
    }
}

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (`00401030`, cdecl: the block).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `DisintegrateArmorEffect`'s vtable (stored by its destructor).
const DISINTEGRATE_ARMOR_EFFECT_VTABLE: u32 = 0x0107_9f2c;
/// `0080d1a0` (thiscall on a new 0x4c-byte block: caster, spell, effect item; unnamed):
/// the `DisintegrateArmorEffect` constructor `Copy` uses.
const CONSTRUCTOR_0080D1A0: u32 = 0x0080_d1a0;
/// `00803ed0` (thiscall; unnamed): the `ActiveEffect` destructor part.
const ACTIVE_EFFECT_DESTRUCTOR_00803ED0: u32 = 0x0080_3ed0;
/// `ActiveEffect::CopyData` (`00804090`, thiscall: the target effect).
const ACTIVE_EFFECT_COPY_DATA: u32 = 0x0080_4090;
/// `ActiveEffect::GetSaveSize` (`00804f80`, thiscall: one word; returns `AX`).
const ACTIVE_EFFECT_GET_SAVE_SIZE: u32 = 0x0080_4f80;
/// `ActiveEffect::SaveGame` (`008050d0`, thiscall: one word).
const ACTIVE_EFFECT_SAVE_GAME: u32 = 0x0080_50d0;
/// `ActiveEffect::LoadGame` (`00805330`, thiscall: one word).
const ACTIVE_EFFECT_LOAD_GAME: u32 = 0x0080_5330;
/// `004459e0` (thiscall: one word; with 1 here, an item change's deleting destructor).
const DELETE_004459E0: u32 = 0x0044_59e0;
/// `MagicTarget::GetTargetAsActor` (`00822b40`).
const MAGIC_TARGET_GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `0059bb30` (thiscall): the effect's target (+0x24).
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `Actor::GetRandomWornArmor` (`0089d8b0`, thiscall: one word).
const ACTOR_GET_RANDOM_WORN_ARMOR: u32 = 0x0089_d8b0;
/// `00495460` (thiscall on the effect, returns `ST0`; unnamed): its magnitude.
const EFFECT_MAGNITUDE_00495460: u32 = 0x0049_5460;
/// `005b6f70` (no arguments, `AL`; unnamed, `tesscriptfunctions.cpp`).
const TEST_005B6F70: u32 = 0x005b_6f70;
/// `ItemChange::GetFullName` (`004be2d0`).
const ITEM_CHANGE_GET_FULL_NAME: u32 = 0x004b_e2d0;
/// `0055d520` (thiscall on the actor, no stack words; unnamed, `tesobjectrefr.cpp`): its
/// name.
const ACTOR_NAME_0055D520: u32 = 0x0055_d520;
/// `00703c00` (cdecl, variable arguments, a format first; unnamed, `interface.cpp`).
const MESSAGE_00703C00: u32 = 0x0070_3c00;
/// `"%s's %s takes %0.2f disintegrate armor damage!"` (`01079f88`).
const ARMOR_DAMAGE_FORMAT: u32 = 0x0107_9f88;
/// `0044ddc0` (thiscall): the word at +8 (an item change's form).
const WORD_AT_8_0044DDC0: u32 = 0x0044_ddc0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, vfDelta, source type, target type, isReference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TESBoundObject`'s RTTI type descriptor (`.?AVTESBoundObject@@`).
const TES_BOUND_OBJECT_TYPE: u32 = 0x0118_3108;
/// `TESObjectARMO`'s RTTI type descriptor (`.?AVTESObjectARMO@@`).
const TES_OBJECT_ARMO_TYPE: u32 = 0x0118_3a34;
/// `TESBipedModelForm::FillsBipedSlot` (`00480af0`, thiscall: three words).
const TES_BIPED_MODEL_FORM_FILLS_BIPED_SLOT: u32 = 0x0048_0af0;
/// `005d43c0` (thiscall on an actor; unnamed): its extra-data list.
const EXTRA_LIST_005D43C0: u32 = 0x005d_43c0;
/// `ExtraDataList::GetContainerChanges` (`00418520`).
const EXTRA_DATA_LIST_GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
/// `InventoryChanges::GetWornItem` (`004c8c10`, thiscall: slot, flag).
const INVENTORY_CHANGES_GET_WORN_ITEM: u32 = 0x004c_8c10;

// Translated from 0080d1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DisintegrateArmorEffect::__scalar_deleting_destructor` (Xbox PDB): the destructor
/// (`fn_0080d210`), then `operator delete` when bit 0 of `flags` is set; returns `this`.
pub fn disintegrate_armor_effect_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<DisintegrateArmorEffect>,
    flags: u32,
) -> u32 {
    fn_0080d210(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this.addr()
}

// Translated from 0080d210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080d210 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (`disintegratearmoreffect.cpp`): the `DisintegrateArmorEffect` destructor: its
/// vtable, the armor's item change deleted (`004459e0` with 1) when set, then the
/// `ActiveEffect` part (`00803ed0`).
pub fn fn_0080d210(e: &mut Engine, this: Ptr<DisintegrateArmorEffect>) {
    e.mem.set_u32(this.addr(), DISINTEGRATE_ARMOR_EFFECT_VTABLE);
    let armor = e.get(this, DisintegrateArmorEffect::pArmor);
    if armor != 0 {
        e.call(DELETE_004459E0, &args![armor, 1u32]);
    }
    e.call(ACTIVE_EFFECT_DESTRUCTOR_00803ED0, &args![this]);
}

// Translated from 0080d2a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080d2a0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `DisintegrateArmorEffect::Copy` (Xbox PDB): a new 0x4c-byte effect (`operator new`,
/// then `0080d1a0` with the caster +0x28, spell +8 and effect item +0xc; null when the
/// allocation fails), given this effect's data through `ActiveEffect::CopyData` (called
/// directly); returns it.
pub fn disintegrate_armor_effect_copy(e: &mut Engine, this: Ptr<DisintegrateArmorEffect>) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let base = this.addr();
        let item = e.mem.u32(base + 0xc);
        let spell = e.mem.u32(base + 8);
        let caster = e.mem.u32(base + 0x28);
        e.call(CONSTRUCTOR_0080D1A0, &args![block, caster, spell, item])
            .u32()
    };
    e.call(ACTIVE_EFFECT_COPY_DATA, &args![this, copy]);
    copy
}

// Translated from 0080d340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DisintegrateArmorEffect::Start` (Xbox PDB): for a target actor, a random worn armor
/// (`GetRandomWornArmor` with 0) becomes the armor; with one, the actor's slot +0x3c8
/// damages it by the magnitude (a `float`, then 1); when `005b6f70` is true the damage is
/// reported (`00703c00` with the format, the actor's name, the item's name and the
/// magnitude as a `double`); then the item change is deleted (the pointer stays).
pub fn disintegrate_armor_effect_start(e: &mut Engine, this: Ptr<DisintegrateArmorEffect>) {
    let actor = if e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() == 0 {
        0
    } else {
        let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
        e.call(MAGIC_TARGET_GET_TARGET_AS_ACTOR, &args![target])
            .u32()
    };
    if actor == 0 {
        return;
    }
    let armor = e
        .call(ACTOR_GET_RANDOM_WORN_ARMOR, &args![actor, 0u32])
        .u32();
    e.set(this, DisintegrateArmorEffect::pArmor, armor);
    if e.get(this, DisintegrateArmorEffect::pArmor) == 0 {
        return;
    }
    let magnitude = x87_store32(e.call(EFFECT_MAGNITUDE_00495460, &args![this]).f64());
    let armor = e.get(this, DisintegrateArmorEffect::pArmor);
    e.vcall(actor, 0x3c8, &args![armor, magnitude, 1u32]);
    if e.call(TEST_005B6F70, &[]).bool() {
        let magnitude = e.call(EFFECT_MAGNITUDE_00495460, &args![this]).f64();
        let armor = e.get(this, DisintegrateArmorEffect::pArmor);
        let item_name = e.call(ITEM_CHANGE_GET_FULL_NAME, &args![armor]).u32();
        let actor_name = e.call(ACTOR_NAME_0055D520, &args![actor]).u32();
        e.call(
            MESSAGE_00703C00,
            &args![ARMOR_DAMAGE_FORMAT, actor_name, item_name, magnitude],
        );
    }
    let armor = e.get(this, DisintegrateArmorEffect::pArmor);
    if armor != 0 {
        e.call(DELETE_004459E0, &args![armor, 1u32]);
    }
}

// Translated from 0080d4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DisintegrateArmorEffect::GetSaveSize` (Xbox PDB): `ActiveEffect::GetSaveSize`, 4 more
/// from save version 0x37. Returns `AX`.
pub fn disintegrate_armor_effect_get_save_size(
    e: &mut Engine,
    this: Ptr<DisintegrateArmorEffect>,
    argument: u32,
) -> u16 {
    let mut size = 0u16;
    let base = e
        .call(ACTIVE_EFFECT_GET_SAVE_SIZE, &args![this, argument])
        .u16();
    size = size.wrapping_add(base);
    if save_version(e) >= 0x37 {
        size = size.wrapping_add(4);
    }
    size
}

// Translated from 0080d540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DisintegrateArmorEffect::SaveGame` (Xbox PDB): `ActiveEffect::SaveGame`, then (save
/// version 0x37 or later) the first biped slot the armor (a `TESObjectARMO`) fills, -1
/// without one, as 4 bytes.
pub fn disintegrate_armor_effect_save_game(
    e: &mut Engine,
    this: Ptr<DisintegrateArmorEffect>,
    argument: u32,
) {
    e.call(ACTIVE_EFFECT_SAVE_GAME, &args![this, argument]);
    if save_version(e) < 0x37 {
        return;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0xffff_ffff);
        let change = e.get(this, DisintegrateArmorEffect::pArmor);
        if change != 0 && e.call(WORD_AT_8_0044DDC0, &args![change]).u32() != 0 {
            let change = e.get(this, DisintegrateArmorEffect::pArmor);
            let form = e.call(WORD_AT_8_0044DDC0, &args![change]).u32();
            let armor = e
                .call(
                    RT_DYNAMIC_CAST,
                    &args![
                        form,
                        0u32,
                        TES_BOUND_OBJECT_TYPE,
                        TES_OBJECT_ARMO_TYPE,
                        0u32
                    ],
                )
                .u32();
            if armor != 0 {
                for index in 0..20u32 {
                    if e.call(
                        TES_BIPED_MODEL_FORM_FILLS_BIPED_SLOT,
                        &args![armor + 0x70, index, 0u32, 0u32],
                    )
                    .bool()
                    {
                        e.mem.set_u32(slot.addr(), index);
                        break;
                    }
                }
            }
        }
        save_bytes(e, slot.addr(), 4);
    });
}

// Translated from 0080d610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DisintegrateArmorEffect::LoadGame` (Xbox PDB; the word is passed on and used as the
/// actor): `ActiveEffect::LoadGame`, then (save version 0x37 or later) the saved slot;
/// unless it is -1, the armor becomes the worn item of that slot (`GetWornItem` with 1) of
/// the actor's container changes when there are some, and the armor's item change is then
/// deleted (the pointer stays).
pub fn disintegrate_armor_effect_load_game(
    e: &mut Engine,
    this: Ptr<DisintegrateArmorEffect>,
    argument: u32,
) {
    e.call(ACTIVE_EFFECT_LOAD_GAME, &args![this, argument]);
    if save_version(e) < 0x37 {
        return;
    }
    e.with_stack(4, |e, slot| {
        load_bytes(e, slot.addr(), 4);
        let index = e.mem.u32(slot.addr());
        if index == 0xffff_ffff {
            return;
        }
        let list = e.call(EXTRA_LIST_005D43C0, &args![argument]).u32();
        let changes = e
            .call(EXTRA_DATA_LIST_GET_CONTAINER_CHANGES, &args![list])
            .u32();
        if changes != 0 {
            let worn = e
                .call(
                    INVENTORY_CHANGES_GET_WORN_ITEM,
                    &args![changes, index, 1u32],
                )
                .u32();
            e.set(this, DisintegrateArmorEffect::pArmor, worn);
        }
        let armor = e.get(this, DisintegrateArmorEffect::pArmor);
        if armor != 0 {
            e.call(DELETE_004459E0, &args![armor, 1u32]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0080_d1e0,
            disintegrate_armor_effect_scalar_deleting_destructor(
                Ptr<DisintegrateArmorEffect>,
                u32,
            ) -> u32
        ),
        entry!(0x0080_d210, fn_0080d210(Ptr<DisintegrateArmorEffect>)),
        entry!(
            0x0080_d2a0,
            disintegrate_armor_effect_copy(Ptr<DisintegrateArmorEffect>) -> u32
        ),
        entry!(
            0x0080_d340,
            disintegrate_armor_effect_start(Ptr<DisintegrateArmorEffect>)
        ),
        entry!(
            0x0080_d4f0,
            disintegrate_armor_effect_get_save_size(Ptr<DisintegrateArmorEffect>, u32) -> u16
        ),
        entry!(
            0x0080_d540,
            disintegrate_armor_effect_save_game(Ptr<DisintegrateArmorEffect>, u32)
        ),
        entry!(
            0x0080_d610,
            disintegrate_armor_effect_load_game(Ptr<DisintegrateArmorEffect>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const SAVE_VERSION: u32 = 0x008d_f040;
    const SAVE_BYTES: u32 = 0x0085_79b0;
    const LOAD_BYTES: u32 = 0x0085_79e0;

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

    fn rig(version: u32) -> (Engine, Ptr<DisintegrateArmorEffect>) {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let game = e.mem.alloc(0x100);
        e.set_global(0x011d_e45c, game);
        answer(&mut e, SAVE_VERSION, version);
        for addr in [
            DELETE_004459E0,
            OPERATOR_DELETE,
            ACTIVE_EFFECT_DESTRUCTOR_00803ED0,
            ACTIVE_EFFECT_COPY_DATA,
            ACTIVE_EFFECT_SAVE_GAME,
            ACTIVE_EFFECT_LOAD_GAME,
            MESSAGE_00703C00,
        ] {
            answer(&mut e, addr, 0);
        }
        let this: Ptr<DisintegrateArmorEffect> = e.new_object();
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn destructors_delete_the_armor() {
        for (flags, deletes) in [(1u32, 1usize), (0, 0)] {
            let (mut e, this) = rig(0);
            e.set(this, DisintegrateArmorEffect::pArmor, 0x4800);
            assert_eq!(e.call(0x0080_d1e0, &args![this, flags]).u32(), this.addr());
            assert_eq!(e.mem.u32(this.addr()), DISINTEGRATE_ARMOR_EFFECT_VTABLE);
            assert_eq!(calls(&e, DELETE_004459E0), vec![vec![0x4800, 1]]);
            assert_eq!(
                calls(&e, ACTIVE_EFFECT_DESTRUCTOR_00803ED0),
                vec![vec![this.addr()]]
            );
            assert_eq!(calls(&e, OPERATOR_DELETE).len(), deletes);
        }
        let (mut e, this) = rig(0);
        e.call(0x0080_d210, &args![this]);
        assert!(calls(&e, DELETE_004459E0).is_empty());
    }

    #[test]
    fn copy_constructs_and_copies_through_active_effect() {
        let (mut e, this) = rig(0);
        e.mem.set_u32(this.addr() + 8, 0x08);
        e.mem.set_u32(this.addr() + 0xc, 0x0c);
        e.mem.set_u32(this.addr() + 0x28, 0x28);
        answer(&mut e, OPERATOR_NEW, 0x9000);
        answer(&mut e, CONSTRUCTOR_0080D1A0, 0x9004);
        assert_eq!(e.call(0x0080_d2a0, &args![this]).u32(), 0x9004);
        assert_eq!(
            calls(&e, CONSTRUCTOR_0080D1A0),
            vec![vec![0x9000, 0x28, 0x08, 0x0c]]
        );
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_COPY_DATA),
            vec![vec![this.addr(), 0x9004]]
        );
        answer(&mut e, OPERATOR_NEW, 0);
        assert_eq!(e.call(0x0080_d2a0, &args![this]).u32(), 0);
    }

    #[test]
    fn start_damages_a_worn_armor_and_reports_it() {
        let (mut e, this) = rig(0);
        let vtable = e.mem.alloc(0x400);
        e.mem.set_u32(vtable + 0x3c8, vtable + 0x3c8);
        answer(&mut e, vtable + 0x3c8, 0);
        let actor = e.mem.alloc(0x10);
        e.mem.set_u32(actor, vtable);
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        answer(&mut e, ACTOR_GET_RANDOM_WORN_ARMOR, 0x4800);
        e.register(EFFECT_MAGNITUDE_00495460, |_, _| Ret {
            st0: 7.5,
            ..Ret::default()
        });
        answer(&mut e, TEST_005B6F70, 1);
        answer(&mut e, ITEM_CHANGE_GET_FULL_NAME, 0x6100);
        answer(&mut e, ACTOR_NAME_0055D520, 0x6200);
        e.call(0x0080_d340, &args![this]);
        assert_eq!(calls(&e, ACTOR_GET_RANDOM_WORN_ARMOR), vec![vec![actor, 0]]);
        assert_eq!(
            calls(&e, vtable + 0x3c8),
            vec![vec![actor, 0x4800, 7.5f32.to_bits(), 1]]
        );
        let bits = 7.5f64.to_bits();
        assert_eq!(
            calls(&e, MESSAGE_00703C00),
            vec![vec![
                ARMOR_DAMAGE_FORMAT,
                0x6200,
                0x6100,
                bits as u32,
                (bits >> 32) as u32
            ]]
        );
        assert_eq!(calls(&e, DELETE_004459E0), vec![vec![0x4800, 1]]);
        assert_eq!(e.get(this, DisintegrateArmorEffect::pArmor), 0x4800);
        // No worn armor: nothing more.
        let (mut e, this) = rig(0);
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        answer(&mut e, ACTOR_GET_RANDOM_WORN_ARMOR, 0);
        e.call(0x0080_d340, &args![this]);
        assert!(calls(&e, DELETE_004459E0).is_empty());
    }

    #[test]
    fn save_size_adds_the_slot_from_version_0x37() {
        for (version, expected) in [(0x37u32, 0x24u16), (0x36, 0x20)] {
            let (mut e, this) = rig(version);
            answer(&mut e, ACTIVE_EFFECT_GET_SAVE_SIZE, 0x0001_0020);
            assert_eq!(e.call(0x0080_d4f0, &args![this, 0x77u32]).u16(), expected);
            assert_eq!(
                calls(&e, ACTIVE_EFFECT_GET_SAVE_SIZE),
                vec![vec![this.addr(), 0x77]]
            );
        }
    }

    #[test]
    fn save_game_writes_the_first_filled_slot() {
        let (mut e, this) = rig(0x37);
        let writes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&writes);
        e.register_double(SAVE_BYTES, move |e, a| {
            log.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        let change = e.mem.alloc(0xc);
        e.mem.set_u32(change + 8, 0x6000);
        e.set(this, DisintegrateArmorEffect::pArmor, change);
        e.register(WORD_AT_8_0044DDC0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        answer(&mut e, RT_DYNAMIC_CAST, 0x4200);
        e.register(TES_BIPED_MODEL_FORM_FILLS_BIPED_SLOT, |_, a| {
            u8::from(a[1] >= 3).into_ret()
        });
        e.call(0x0080_d540, &args![this, 0x77u32]);
        assert_eq!(*writes.borrow(), vec![3]);
        assert_eq!(
            calls(&e, TES_BIPED_MODEL_FORM_FILLS_BIPED_SLOT)[0],
            vec![0x4270, 0, 0, 0]
        );
        // Without an armor: -1.
        let (mut e, this) = rig(0x37);
        let writes = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&writes);
        e.register_double(SAVE_BYTES, move |e, a| {
            log.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        e.call(0x0080_d540, &args![this, 0x77u32]);
        assert_eq!(*writes.borrow(), vec![0xffff_ffff]);
    }

    #[test]
    fn load_game_finds_the_worn_item_of_the_slot() {
        let (mut e, this) = rig(0x37);
        e.register(LOAD_BYTES, |e, a| {
            e.mem.set_u32(a[1], 5);
            Ret::default()
        });
        answer(&mut e, EXTRA_LIST_005D43C0, 0x3000);
        answer(&mut e, EXTRA_DATA_LIST_GET_CONTAINER_CHANGES, 0x3100);
        answer(&mut e, INVENTORY_CHANGES_GET_WORN_ITEM, 0x4900);
        e.call(0x0080_d610, &args![this, 0x7100u32]);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_LOAD_GAME),
            vec![vec![this.addr(), 0x7100]]
        );
        assert_eq!(calls(&e, EXTRA_LIST_005D43C0), vec![vec![0x7100]]);
        assert_eq!(
            calls(&e, INVENTORY_CHANGES_GET_WORN_ITEM),
            vec![vec![0x3100, 5, 1]]
        );
        assert_eq!(e.get(this, DisintegrateArmorEffect::pArmor), 0x4900);
        assert_eq!(calls(&e, DELETE_004459E0), vec![vec![0x4900, 1]]);
        // -1: nothing more.
        let (mut e, this) = rig(0x37);
        e.register(LOAD_BYTES, |e, a| {
            e.mem.set_u32(a[1], 0xffff_ffff);
            Ret::default()
        });
        e.call(0x0080_d610, &args![this, 0x7100u32]);
        assert!(calls(&e, EXTRA_LIST_005D43C0).is_empty());
    }
}
