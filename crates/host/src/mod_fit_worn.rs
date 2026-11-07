//! The worn question after fitting a weapon mod (`NV_MOD_WORN`), a fix of the `fixes` preset
//! ([`crate::fixes`], docs/FIXES.md): **not 1:1 with FalloutNV.exe**, and in the `vanilla` preset
//! nothing here runs.
//!
//! **What the original does.** Fitting a mod in the item mod menu (`ItemModMenu::DoClick`,
//! `007838a0`, the list's click) fits it to the weapon (`ItemModItem`), removes one of the mod
//! from the player (the player's vtable `+0x17c`, `007839ca`), plays the sound, takes the entry
//! out of the menu's list and rebuilds the menu, and only then asks `ItemChange::GetWorn`
//! (`00783a3f`) of the *mod's* list entry, not the weapon's: a slip. That entry is a copy of the
//! inventory's whose extra list is the mod's own, and the removal deletes that list when the last
//! of the mod goes, so the question reads a released block. In the original it nearly always
//! answers "no" (the list's flag bytes were cleared before the delete and a freed 32-byte pool
//! block keeps them), and then nothing more happens. A "yes" makes the player update its equipment
//! (vtable `+0x3c4`) and rebuilds the inventory list (`InventoryMenu::UpdateInventoryMenu`
//! `00782a90`), which frees the weapon entry the mod menu keeps (`011d9f58`); closing the menu
//! then re-equips the weapon through that freed entry and the game stops
//! (measure/weapon-mod-crash, 2026-11-07).
//!
//! **What this does.** The list's click runs as the original's, step for step, except that the
//! worn question is asked of the same entry just before the removal, while its extra list still
//! exists, and its answer is used where the original asks. A mod is never worn, so the answer is
//! the original's usual "no", and no released memory is read. Every other click is the
//! translation's own.

use crate::fixes::Fix;
use engine::prelude::*;
use engine::units::fallout_interface::itemmodmenu::{
    item_mod_menu_do_click, item_mod_menu_init_item_mod_menu, item_mod_menu_item_mod_item,
};

/// `ItemModMenu::DoClick` (Xbox PDB).
pub const DO_CLICK: u32 = 0x0078_38a0;

/// The row in [`crate::fixes::FIXES`].
pub const FIX: Fix = Fix {
    name: "mod fit worn question",
    env: "NV_MOD_WORN",
    addresses: &[DO_CLICK],
    summary: "fitting a weapon mod asks whether the mod is worn before removing it, not after",
    install,
};

/// The menu's item list (`lbItemModList`, Xbox PDB) and the list's id.
const MOD_LIST: u32 = 0x60;
const LIST_ID: i32 = 0xf;
/// `ItemModMenu`'s item being modified (the weapon's entry) and the player.
const MOD_ITEM: u32 = 0x011d_9f58;
const PLAYER: u32 = 0x011d_ea3c;
/// The word at +4 used on a tile + 4, `Tile::GetFloatTrait` (`this`, trait; ST0), the trait the
/// entry's alpha is in and the alpha below which an entry can't be chosen (`double`).
const WORD_AT_FOUR_007B5370: u32 = 0x007b_5370;
const GET_FLOAT_TRAIT: u32 = 0x00a0_11b0;
const TRAIT_ALPHA: u32 = 0xfa9;
const ENABLED_ALPHA: u32 = 0x0101_e568;
/// `ListBox<ItemChange *>`'s highlighted item and `RemoveItem` (Xbox PDB).
const LIST_SELECTED_ITEM: u32 = 0x007a_1910;
const LIST_REMOVE_ITEM: u32 = 0x0071_a5c0;
/// The player's vtable slots: remove an item, update the equipment.
const REMOVE_ITEM_SLOT: u32 = 0x17c;
const UPDATE_EQUIPMENT_SLOT: u32 = 0x3c4;
/// `BSAudio::QInstance`, `BSAudio::GetSoundHandleByName`, `BSSoundHandle::Play` (Xbox PDB), the
/// handle's destructor body and the sound of a fitted mod.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
const AUDIO_GET_SOUND: u32 = 0x00ad_7550;
const SOUND_PLAY: u32 = 0x00ad_8830;
const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
const MOD_SOUND: u32 = 0x0107_3c58;
/// `ItemChange::GetWorn` and `InventoryMenu::UpdateInventoryMenu` (Xbox PDB).
const ITEM_GET_WORN: u32 = 0x004b_ddd0;
const UPDATE_INVENTORY_MENU: u32 = 0x0078_2a90;

/// Registers the click at `007838a0`.
pub fn install(e: &mut Engine) {
    e.register(DO_CLICK, do_click_entry);
}

fn do_click_entry(e: &mut Engine, a: &[u32]) -> Ret {
    let (this, id, tile) = (Ptr::new(a[0]), a[1] as i32, a[2]);
    if id == LIST_ID {
        fit_clicked_mod(e, this, tile);
    } else {
        item_mod_menu_do_click(e, this, id, tile);
    }
    Ret::default()
}

/// `DoClick`'s list case as the translation has it, with the worn question moved before the
/// removal (module documentation).
fn fit_clicked_mod(e: &mut Engine, this: Ptr, tile: u32) {
    let base = this.addr();
    let slot = e
        .call(WORD_AT_FOUR_007B5370, &args![tile.wrapping_add(4)])
        .u32();
    let target = e.mem.u32(slot);
    let alpha = e.call(GET_FLOAT_TRAIT, &args![target, TRAIT_ALPHA]).f64();
    if alpha < e.global::<f64>(ENABLED_ALPHA) {
        return;
    }
    let mod_item = e.global::<u32>(MOD_ITEM);
    let selected = e.call(LIST_SELECTED_ITEM, &args![base + MOD_LIST]).u32();
    if selected == mod_item {
        return;
    }
    let item = e.call(LIST_SELECTED_ITEM, &args![base + MOD_LIST]).u32();
    let extra = first_extra_list(e, item);
    if !item_mod_menu_item_mod_item(e, this, item) {
        return;
    }
    let object = typed_call!(leaf::FAMILY, 0x0044_ddc0, leaf::word_at_8(e, item));
    // The fix: asked while the mod's extra list still exists.
    let worn = e.call(ITEM_GET_WORN, &args![item, 0u32]).bool();
    let player = e.global::<u32>(PLAYER);
    e.vcall(
        player,
        REMOVE_ITEM_SLOT,
        &args![object, extra, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
    );
    e.with_stack(0xc, |e, handle| {
        let audio = e.call(AUDIO_INSTANCE, &args![]).u32();
        let sound = e
            .call(AUDIO_GET_SOUND, &args![audio, handle, MOD_SOUND, 0x121u32])
            .u32();
        e.call(SOUND_PLAY, &args![sound, 0u32]);
        e.call(SOUND_HANDLE_DESTRUCTOR, &args![handle]);
    });
    let highlighted = e.vcall(base + MOD_LIST, 4, &args![]).u32();
    e.call(LIST_REMOVE_ITEM, &args![base + MOD_LIST, highlighted]);
    let mod_item = e.global::<u32>(MOD_ITEM);
    item_mod_menu_init_item_mod_menu(e, mod_item, 0);
    if worn {
        let player = e.global::<u32>(PLAYER);
        e.vcall(player, UPDATE_EQUIPMENT_SLOT, &args![]);
        e.call(UPDATE_INVENTORY_MENU, &args![]);
    }
}

/// The first extra data list of an item, or 0 without one.
fn first_extra_list(e: &mut Engine, item: u32) -> u32 {
    if typed_call!(leaf::FAMILY, 0x0055_9450, leaf::word_at_0(e, item)) == 0 {
        return 0;
    }
    let list = typed_call!(leaf::FAMILY, 0x0055_9450, leaf::word_at_0(e, item));
    let slot = typed_call!(leaf::FAMILY, 0x0068_15c0, leaf::same_word(e, list));
    e.mem.u32(slot)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// `ItemModMenu::pMe`.
    const ITEM_MOD_MENU: u32 = 0x011d_9f54;
    /// What the menu's rebuild (`InitItemModMenu` with its `Update`) calls, answered 0: the
    /// item's health, tile traits, the form's type, settings' strings, the armour rating and its
    /// rounding, the icon, the mod slots, the mod group list, the string comparison, the sort,
    /// the gamepad test, the interface calls and the tutorial.
    const REBUILD_STUBS: [u32; 22] = [
        0x004b_cdb0,
        0x00a0_12d0,
        0x0070_0320,
        0x0040_1170,
        0x0040_3df0,
        0x00a0_1350,
        0x004b_e060,
        0x00ec_62c0,
        0x0064_6360,
        0x0047_6b20,
        0x004b_e200,
        0x004b_d820,
        0x0096_a2d0,
        0x004d_4ca0,
        0x0040_8b20,
        0x0046_ffb0,
        0x00a0_3da0,
        0x0073_14c0,
        0x004b_71d0,
        0x0070_46f0,
        0x004b_7210,
        0x0071_8630,
    ];
    /// The menu's and its list's virtual calls in the rebuild (test vtables): the menu's `+0x10`,
    /// the list's `+0x1c` and `+0x20`.
    const MENU_SLOT_10: u32 = 0x00c4_0010;
    const LIST_SLOT_1C: u32 = 0x00c3_001c;
    const LIST_SLOT_20: u32 = 0x00c3_0020;
    /// `100.0`, the full health the rebuild divides by.
    const FULL_HEALTH: u32 = 0x0101_7a40;
    type Log = Vec<(u32, Vec<u32>)>;
    /// What `ItemModItem` calls on the way to fitting a mod to a single weapon (the translation's
    /// own test, `fitting_a_mod_to_a_single_weapon`).
    const ITEM_OBJECT: u32 = 0x0044_ddc0;
    const WORD_AT_ZERO: u32 = 0x0055_9450;
    const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
    const GET_OBJECT_IN_LIST: u32 = 0x004b_fba0;
    const OPERATOR_NEW: u32 = 0x0040_1000;
    const EXTRA_CONSTRUCTOR: u32 = 0x0041_0360;
    const EXTRA_GET_COUNT: u32 = 0x0041_8770;
    const EXTRA_GET_MOD_FLAGS: u32 = 0x0042_e560;
    const WEAPON_MOD: u32 = 0x004b_d570;
    const WEAPON_MOD_EFFECT: u32 = 0x004b_d880;
    const ITEM_MOD_STUBS: [u32; 9] = [
        0x0041_2380, // DuplicateExtraListForContainer
        0x0041_9ad0, // SetCount
        0x0063_f7b0, // the simple list's head removal
        0x0042_e380, // SetWeaponModSlot
        0x005a_e3d0, // the simple list's add
        0x0090_5330, // the simple list's removal
        0x0041_9f80, // the extra list's clear
        0x00c5_0000, // the new extra list's deleting destructor (test vtable)
        0x004d_5c60, // MiscStatManager::Increment
    ];
    /// The player's removal and equipment update, and the list's highlighted item (test vtables).
    const PLAYER_REMOVE: u32 = 0x00c2_0000;
    const PLAYER_UPDATE_EQUIPMENT: u32 = 0x00c2_1000;
    const LIST_HIGHLIGHTED: u32 = 0x00c3_0000;

    fn stub(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    fn answer(e: &mut Engine, address: u32, eax: u32) {
        e.register_double(address, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn calls_to(e: &Engine, callee: u32) -> usize {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .count()
    }

    /// Clicks an enabled mod entry (`0x8000`) of the menu modifying a single weapon (`0x7000`).
    /// The removal deletes the mod's extra list; `GetWorn` reads it, and after the removal answers
    /// "yes" (a released block taken again), or always `always` when given. Answers, for each worn
    /// question, whether it read the released block, the calls of the equipment update and the
    /// inventory rebuild, and every call the click made, in order.
    fn click(fixes: bool, always: Option<bool>) -> (Vec<bool>, usize, usize, Log) {
        let mut e = Engine::new();
        if fixes {
            install(&mut e);
        }
        for page in [ITEM_MOD_MENU, PLAYER, ENABLED_ALPHA, FULL_HEALTH] {
            e.map(page & !0xfff, 0x1000);
        }
        e.mem.set_f64(FULL_HEALTH, 100.0);
        let this = e.mem.alloc(0x90);
        let menu_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this, menu_vtable);
        e.mem.set_u32(menu_vtable + 0x10, MENU_SLOT_10);
        e.set_global(ITEM_MOD_MENU, this);
        let list_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(this + MOD_LIST, list_vtable);
        e.mem.set_u32(list_vtable + 4, LIST_HIGHLIGHTED);
        e.mem.set_u32(list_vtable + 0x1c, LIST_SLOT_1C);
        e.mem.set_u32(list_vtable + 0x20, LIST_SLOT_20);
        stub(&mut e, &REBUILD_STUBS);
        stub(&mut e, &[MENU_SLOT_10, LIST_SLOT_1C, LIST_SLOT_20]);
        let player = e.mem.alloc(0x10);
        let player_vtable = e.mem.alloc(0x400);
        e.mem.set_u32(player, player_vtable);
        e.mem
            .set_u32(player_vtable + REMOVE_ITEM_SLOT, PLAYER_REMOVE);
        e.mem.set_u32(
            player_vtable + UPDATE_EQUIPMENT_SLOT,
            PLAYER_UPDATE_EQUIPMENT,
        );
        e.set_global(PLAYER, player);
        e.set_global(MOD_ITEM, 0x7000u32);
        // An enabled entry, the mod's.
        let slot = e.mem.alloc(4);
        answer(&mut e, WORD_AT_FOUR_007B5370, slot);
        e.register_double(GET_FLOAT_TRAIT, |_, _| Ret {
            st0: 255.0,
            ..Ret::default()
        });
        answer(&mut e, LIST_SELECTED_ITEM, 0x8000);
        // ItemModItem fits it (the translation's test of a single weapon).
        e.register(ITEM_OBJECT, |_, a| (a[0] + 0x100).into_ret());
        answer(&mut e, GET_INVENTORY_CHANGES, 0x9000);
        answer(&mut e, GET_OBJECT_IN_LIST, 0x9100);
        answer(&mut e, WORD_AT_ZERO, 0);
        answer(&mut e, OPERATOR_NEW, 0x9200);
        answer(&mut e, EXTRA_GET_COUNT, 1);
        answer(&mut e, EXTRA_GET_MOD_FLAGS, 1);
        e.register(WEAPON_MOD, |_, a| {
            (if a[1] == 2 { 0x8100u32 } else { 0 }).into_ret()
        });
        answer(&mut e, WEAPON_MOD_EFFECT, 0);
        let new_extra = e.mem.alloc(8);
        let new_extra_vtable = e.mem.alloc(8);
        e.mem.set_u32(new_extra, new_extra_vtable);
        e.mem.set_u32(new_extra_vtable, 0x00c5_0000);
        answer(&mut e, EXTRA_CONSTRUCTOR, new_extra);
        stub(&mut e, &ITEM_MOD_STUBS);
        // The removal releases the mod's list; the worn question reads it.
        let released = Rc::new(Cell::new(false));
        let reads = Rc::new(RefCell::new(Vec::new()));
        let r = released.clone();
        e.register_double(PLAYER_REMOVE, move |_, _| {
            r.set(true);
            Ret::default()
        });
        let (r, log) = (released.clone(), reads.clone());
        e.register_double(ITEM_GET_WORN, move |_, a| {
            assert_eq!(a[..2], [0x8000, 0]);
            log.borrow_mut().push(r.get());
            Ret {
                eax: u32::from(always.unwrap_or(r.get())),
                ..Ret::default()
            }
        });
        stub(
            &mut e,
            &[
                AUDIO_INSTANCE,
                AUDIO_GET_SOUND,
                SOUND_PLAY,
                SOUND_HANDLE_DESTRUCTOR,
                LIST_HIGHLIGHTED,
                LIST_REMOVE_ITEM,
                PLAYER_UPDATE_EQUIPMENT,
                UPDATE_INVENTORY_MENU,
            ],
        );
        e.call_log = Some(vec![]);
        e.call(DO_CLICK, &args![this, LIST_ID, 0u32]);
        assert_eq!(calls_to(&e, PLAYER_REMOVE), 1, "the mod is removed");
        assert_eq!(
            calls_to(&e, LIST_REMOVE_ITEM),
            1,
            "its entry leaves the list"
        );
        let reads = reads.borrow().clone();
        (
            reads,
            calls_to(&e, PLAYER_UPDATE_EQUIPMENT),
            calls_to(&e, UPDATE_INVENTORY_MENU),
            e.call_log.take().unwrap(),
        )
    }

    fn fit(fixes: bool) -> (Vec<bool>, usize, usize) {
        let (reads, updates, rebuilds, _) = click(fixes, None);
        (reads, updates, rebuilds)
    }

    /// The fixes preset asks before the removal: no released block is read, and the answer, the
    /// original's usual "no", rebuilds nothing.
    #[test]
    fn fixes_ask_before_the_removal_and_rebuild_nothing() {
        assert_eq!(fit(true), (vec![false], 0, 0));
    }

    /// Vanilla is the translation: asked after the removal, through the released block, whose
    /// "yes" updates the equipment and rebuilds the inventory list (the crash's first half).
    #[test]
    fn vanilla_asks_after_the_removal() {
        assert_eq!(fit(false), (vec![true], 1, 1));
    }

    /// The fix's copy of the click makes the translation's calls, addresses and arguments in
    /// order (the lookups, the fitting, the removal, the sound, the list removal, the menu's
    /// rebuild and, on a "yes", the equipment update and the inventory rebuild); only the worn
    /// question moves, to just before the removal.
    #[test]
    fn fixes_make_the_translations_calls_with_only_the_question_moved() {
        let at = |log: &Log, callee: u32| log.iter().position(|(a, _)| *a == callee).unwrap();
        for worn in [false, true] {
            let (_, _, _, fixed) = click(true, Some(worn));
            let (_, _, _, vanilla) = click(false, Some(worn));
            assert_eq!(at(&fixed, ITEM_GET_WORN) + 1, at(&fixed, PLAYER_REMOVE));
            assert!(at(&vanilla, ITEM_GET_WORN) > at(&vanilla, LIST_REMOVE_ITEM));
            let without_question = |log: &Log| {
                let mut log = log.clone();
                log.remove(at(&log, ITEM_GET_WORN));
                log
            };
            assert_eq!(without_question(&fixed), without_question(&vanilla));
            // The whole click is in the sequence.
            for callee in [
                WORD_AT_FOUR_007B5370,
                GET_FLOAT_TRAIT,
                LIST_SELECTED_ITEM,
                GET_INVENTORY_CHANGES,
                PLAYER_REMOVE,
                SOUND_PLAY,
                LIST_REMOVE_ITEM,
                0x0073_14c0,
                0x0071_8630,
            ] {
                at(&vanilla, callee);
            }
            assert_eq!(
                vanilla.iter().any(|(a, _)| *a == UPDATE_INVENTORY_MENU),
                worn
            );
        }
    }

    /// The other clicks are the translation's: the close button without a weapon to re-equip
    /// returns to the inventory.
    #[test]
    fn other_clicks_run_the_translation() {
        const PLAY_MENU_SOUND: u32 = 0x0071_7280;
        const SET_INVENTORY_MENU_VISIBLE: u32 = 0x0070_48f0;
        let mut e = Engine::new();
        install(&mut e);
        e.map(ITEM_MOD_MENU & !0xfff, 0x1000);
        let this = e.mem.alloc(0x90);
        stub(&mut e, &[PLAY_MENU_SOUND, SET_INVENTORY_MENU_VISIBLE]);
        e.call_log = Some(vec![]);
        e.call(DO_CLICK, &args![this, 0xci32, 0u32]);
        assert_eq!(calls_to(&e, SET_INVENTORY_MENU_VISIBLE), 1);
        assert_eq!(calls_to(&e, ITEM_GET_WORN), 0);
    }
}
