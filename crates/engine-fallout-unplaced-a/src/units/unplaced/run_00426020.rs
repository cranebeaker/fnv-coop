//! `(unplaced)/run_00426020` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable delta, source type, target type,
/// is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors `.?AVTESForm@@`, `.?AVTESObjectREFR@@` and `.?AVActor@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_REFERENCE: u32 = 0x0118_41cc;
const TYPE_ACTOR: u32 = 0x0118_46d4;
/// `004839c0` (`cdecl`, one word): the form of a form id, or null.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `BaseExtraList::GetExtraData` (`00410220`, `ECX`, one stack word: the type).
const GET_EXTRA_DATA: u32 = 0x0041_0220;
/// `0084e3a0` (`ECX`): a word of the object (here handed to `00469860`).
const RECORD_ID_0084E3A0: u32 = 0x0084_e3a0;
/// `00469860` (`tesdatahandler.cpp`; `ECX` = the data handler, one stack word), `AL`.
const DATA_HANDLER_TEST_00469860: u32 = 0x0046_9860;
/// The `TESDataHandler` singleton pointer.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `TESPackage::CalculateProcedureType` (engine map, `006777b0`, `ECX`, one stack word).
const CALCULATE_PROCEDURE_TYPE: u32 = 0x0067_77b0;
/// `00422700` (`extradatalist.cpp`; `ECX` = the list): an extra data whose +0xc is a list.
const ACTOR_LIST_EXTRA_00422700: u32 = 0x0042_2700;
/// `008256d0` (`ECX` = list entry): whether the entry is empty (`AL`).
const ENTRY_EMPTY_008256D0: u32 = 0x0082_56d0;
/// `006815c0` (`ECX` = list entry): the address of the entry's item word.
const ITEM_ADDRESS_006815C0: u32 = 0x0068_15c0;
/// `0063f7b0` (`ECX` = list entry): removes the entry's item (the walk stays on the entry).
const REMOVE_ITEM_0063F7B0: u32 = 0x0063_f7b0;
/// `00726c60` (`ECX` = list entry, one stack word: the address of the new item).
const SET_ITEM_00726C60: u32 = 0x0072_6c60;
/// `00726070` (`ECX` = list entry): the next entry.
const NEXT_ENTRY_00726070: u32 = 0x0072_6070;
/// `00905330` (`ECX` = list entry, one stack word: the address of an item word): removes the
/// entry after it.
const REMOVE_NEXT_00905330: u32 = 0x0090_5330;
/// `00418460` (`ECX` = the list) and `0043a9f0` (`teleportdata.cpp`; `ECX` = its result).
const TELEPORT_EXTRA_00418460: u32 = 0x0041_8460;
const TELEPORT_RESOLVE_0043A9F0: u32 = 0x0043_a9f0;
/// The pointer handed to `008df040`, and `008df040` itself (`ECX`; a version byte in `AL`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const VERSION_008DF040: u32 = 0x008d_f040;
/// `ExtraDataList::GetDroppedItemList` (engine map, `0041df90`, `ECX`).
const GET_DROPPED_ITEM_LIST: u32 = 0x0041_df90;
/// `005d43c0` (`ECX` = a reference, no stack words): the extra list it answers.
const REFERENCE_EXTRA_LIST_005D43C0: u32 = 0x005d_43c0;
/// `ExtraDataList::AddDroppedItem` (engine map, `0041de40`, `ECX`, one stack word).
const ADD_DROPPED_ITEM: u32 = 0x0041_de40;
/// `ExtraDataList::GetItemDropper` (engine map, `0041de00`, `ECX`).
const GET_ITEM_DROPPER: u32 = 0x0041_de00;
/// `0041e000` (`extradatalist.cpp`; `ECX`, one stack word).
const CALLEE_0041E000: u32 = 0x0041_e000;
/// `ExtraDataList::GetHeadTrackTargetExtra` (engine map, `00419ea0`, `ECX`).
const GET_HEAD_TRACK_TARGET: u32 = 0x0041_9ea0;
/// `ExtraDataList::SetItemDropper` (engine map, `00419dc0`, `ECX`, one stack word).
const SET_ITEM_DROPPER: u32 = 0x0041_9dc0;

/// The form a form id names, cast from `TESForm` to `class` (`004839c0`, then
/// `__RTDynamicCast`).
fn resolve(e: &mut Engine, id: u32, class: u32) -> u32 {
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    e.call(DYNAMIC_CAST, &args![form, 0u32, TYPE_TES_FORM, class, 0u32])
        .u32()
}

// Translated from 00426020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`ECX` = an extra-data list, four stack words, `RET 0x10`;
/// called by `004bed00` and `0055f5f0`): turns the form ids the list's extra datas hold
/// after loading into the forms they name. The engine map's `ExtraDataList` names of the
/// callees are kept as names only.
///
/// When `reference` is not null and its vtable slot +0x100 answers true: the type-0x19
/// extra's object (+0xc) gets slot +0x158 when `00469860` answers true for its
/// `0084e3a0` word, its +0x14 is resolved to a `TESObjectREFR` when not null, and
/// `TESPackage::CalculateProcedureType` runs on the object with it; then each entry of the
/// list at +0xc of `00422700`'s extra is resolved to an `Actor` (`00726c60`), or removed
/// (`0063f7b0`, staying on the entry) when that fails. Otherwise, with `flags & 0x20000`,
/// `0043a9f0` on `00418460`'s result.
///
/// Then, while `008df040`'s byte is below 0x46, each entry of the dropped-item list is
/// resolved to a `TESObjectREFR` (stored by `00726c60`, its extra list given `reference`
/// through `0041de40`), or removed: the first entry through `0063f7b0` (the walk ends when
/// the entry is then empty), a later one through `00905330` on the entry before it. From
/// 0x46 on, the item dropper (`0041de00`) is resolved, handed to `0041de40`, and when not
/// null its extra list gets `reference` through `0041e000`. Last, the head-track target
/// (`00419ea0`), when not null, is resolved and handed to `00419dc0`.
///
/// Six blocks are guarded by `flags & 0` (the compiler folded the mask to zero: extra types
/// 0xd, 0x1c, 0x3f, 0x1a, 0x32 and 0x33, the last two only without a resolved `reference`)
/// and never run; they are left out. `_unused_2` is never read, and `extra` is read only by
/// one of those blocks.
pub fn fn_00426020(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
    _unused_2: u32,
    reference: Ptr,
    _extra: u32,
) {
    let list = this.addr();
    let resolved = !reference.is_null() && e.vcall(reference.addr(), 0x100, &args![]).u8() != 0;
    if resolved {
        let package = e.call(GET_EXTRA_DATA, &args![list, 0x19u32]).u32();
        if package != 0 {
            let object = e.mem.u32(package + 0xc);
            let word = e.call(RECORD_ID_0084E3A0, &args![object]).u32();
            let handler: u32 = e.global(DATA_HANDLER);
            if e.call(DATA_HANDLER_TEST_00469860, &args![handler, word])
                .u8()
                != 0
            {
                let object = e.mem.u32(package + 0xc);
                e.vcall(object, 0x158, &args![]);
            }
            let target = e.mem.u32(package + 0x14);
            if target != 0 {
                let form = resolve(e, target, TYPE_REFERENCE);
                e.mem.set_u32(package + 0x14, form);
            }
            let target = e.mem.u32(package + 0x14);
            let object = e.mem.u32(package + 0xc);
            e.call(CALCULATE_PROCEDURE_TYPE, &args![object, target]);
        }
        let holder = e.call(ACTOR_LIST_EXTRA_00422700, &args![list]).u32();
        if holder != 0 {
            let mut entry = e.mem.u32(holder + 0xc);
            e.with_stack(4, |e, cast| {
                while entry != 0 && e.call(ENTRY_EMPTY_008256D0, &args![entry]).u8() == 0 {
                    e.mem.set_u32(cast.addr(), 0);
                    let item_address = e.call(ITEM_ADDRESS_006815C0, &args![entry]).u32();
                    let item = e.mem.u32(item_address);
                    if item != 0 {
                        let actor = resolve(e, item, TYPE_ACTOR);
                        e.mem.set_u32(cast.addr(), actor);
                    }
                    if e.mem.u32(cast.addr()) == 0 {
                        e.call(REMOVE_ITEM_0063F7B0, &args![entry]);
                    } else {
                        e.call(SET_ITEM_00726C60, &args![entry, cast]);
                        entry = e.call(NEXT_ENTRY_00726070, &args![entry]).u32();
                    }
                }
            });
        }
    } else if flags & 0x0002_0000 != 0 {
        let teleport = e.call(TELEPORT_EXTRA_00418460, &args![list]).u32();
        e.call(TELEPORT_RESOLVE_0043A9F0, &args![teleport]);
    }

    let save_load_game: u32 = e.global(SAVE_LOAD_GAME);
    if e.call(VERSION_008DF040, &args![save_load_game]).u8() < 0x46 {
        let mut entry = e.call(GET_DROPPED_ITEM_LIST, &args![list]).u32();
        let mut previous = 0u32;
        e.with_stack(8, |e, slots| {
            let cast = slots;
            let removed = slots.byte_add(4);
            while entry != 0 && e.call(ENTRY_EMPTY_008256D0, &args![entry]).u8() == 0 {
                let item_address = e.call(ITEM_ADDRESS_006815C0, &args![entry]).u32();
                let item = e.mem.u32(item_address);
                e.mem.set_u32(cast.addr(), item);
                if item != 0 {
                    let form = resolve(e, item, TYPE_REFERENCE);
                    e.mem.set_u32(cast.addr(), form);
                }
                if e.mem.u32(cast.addr()) == 0 {
                    if previous == 0 {
                        e.call(REMOVE_ITEM_0063F7B0, &args![entry]);
                        if e.call(ENTRY_EMPTY_008256D0, &args![entry]).u8() != 0 {
                            entry = 0;
                        }
                    } else {
                        e.mem.set_u32(removed.addr(), item);
                        e.call(REMOVE_NEXT_00905330, &args![previous, removed]);
                        entry = e.call(NEXT_ENTRY_00726070, &args![previous]).u32();
                    }
                } else {
                    e.call(SET_ITEM_00726C60, &args![entry, cast]);
                    let form = e.mem.u32(cast.addr());
                    let extra_list = e.call(REFERENCE_EXTRA_LIST_005D43C0, &args![form]).u32();
                    e.call(ADD_DROPPED_ITEM, &args![extra_list, reference]);
                    previous = entry;
                    entry = e.call(NEXT_ENTRY_00726070, &args![entry]).u32();
                }
            }
        });
    }

    let save_load_game: u32 = e.global(SAVE_LOAD_GAME);
    if e.call(VERSION_008DF040, &args![save_load_game]).u8() >= 0x46 {
        let dropper = e.call(GET_ITEM_DROPPER, &args![list]).u32();
        if dropper != 0 {
            let form = resolve(e, dropper, TYPE_REFERENCE);
            e.call(ADD_DROPPED_ITEM, &args![list, form]);
            if form != 0 {
                let extra_list = e.call(REFERENCE_EXTRA_LIST_005D43C0, &args![form]).u32();
                e.call(CALLEE_0041E000, &args![extra_list, reference]);
            }
        }
    }

    let target = e.call(GET_HEAD_TRACK_TARGET, &args![list]).u32();
    if target != 0 {
        let form = resolve(e, target, TYPE_REFERENCE);
        e.call(SET_ITEM_DROPPER, &args![list, form]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0042_6020, fn_00426020(Ptr, u32, u32, Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    /// The extra list at 0x0050_0000; `version` is `008df040`'s byte; `resolved` is the
    /// reference's slot +0x100 answer. Form ids resolve to `id + 0x1000` except id 0x66 (null).
    fn engine(version: u32, resolved: bool) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global(DATA_HANDLER, 0x0090_0000u32);
        e.set_global(SAVE_LOAD_GAME, 0x0091_0000u32);
        e.register_double(VERSION_008DF040, move |_, _| version.into_ret());
        e.register(LOOKUP_FORM, |_, a| a[0].into_ret());
        e.register(DYNAMIC_CAST, |_, a| {
            (if a[0] == 0x66 { 0u32 } else { a[0] + 0x1000 }).into_ret()
        });
        e.register(
            0x0060_0100,
            if resolved {
                |_, _| 1u32.into_ret()
            } else {
                |_, _| 0x100u32.into_ret()
            },
        );
        let mut slots = vec![0u32; 0x57];
        slots[0x100 / 4] = 0x0060_0100;
        slots[0x158 / 4] = 0x0060_0158;
        e.put_vtable(0x0070_0000, &slots);
        e.register(0x0060_0158, |_, _| Ret::default());
        for callee in [
            CALCULATE_PROCEDURE_TYPE,
            REMOVE_ITEM_0063F7B0,
            REMOVE_NEXT_00905330,
            TELEPORT_RESOLVE_0043A9F0,
            ADD_DROPPED_ITEM,
            CALLEE_0041E000,
            SET_ITEM_DROPPER,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.register(TELEPORT_EXTRA_00418460, |_, _| 0x0077_0000u32.into_ret());
        e.register(REFERENCE_EXTRA_LIST_005D43C0, |_, a| {
            (a[0] + 0x20).into_ret()
        });
        e.register(RECORD_ID_0084E3A0, |_, a| (a[0] + 1).into_ret());
        e.register(DATA_HANDLER_TEST_00469860, |_, _| 1u32.into_ret());
        // Lists: entries with the item word at +0 and the next entry at +4.
        e.register(ITEM_ADDRESS_006815C0, |_, a| a[0].into_ret());
        e.register(NEXT_ENTRY_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(ENTRY_EMPTY_008256D0, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0).into_ret()
        });
        e.register(SET_ITEM_00726C60, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            Ret::default()
        });
        e.register(GET_EXTRA_DATA, |_, _| 0u32.into_ret());
        e.register(ACTOR_LIST_EXTRA_00422700, |_, _| 0u32.into_ret());
        e.register(GET_DROPPED_ITEM_LIST, |_, _| 0u32.into_ret());
        e.register(GET_ITEM_DROPPER, |_, _| 0u32.into_ret());
        e.register(GET_HEAD_TRACK_TARGET, |_, _| 0u32.into_ret());
        e
    }

    fn reference(e: &mut Engine) -> u32 {
        let reference = e.mem.alloc(8);
        e.mem.set_u32(reference, 0x0070_0000);
        reference
    }

    /// A list of entries holding `items`; answers the first entry.
    fn list(e: &mut Engine, items: &[u32]) -> u32 {
        let entries: Vec<u32> = items.iter().map(|_| e.mem.alloc(8)).collect();
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(entries[i], *item);
            e.mem
                .set_u32(entries[i] + 4, entries.get(i + 1).copied().unwrap_or(0));
        }
        entries[0]
    }

    fn calls(e: &mut Engine, callee: u32) -> Vec<Vec<u32>> {
        let log: Log = e.call_log.take().unwrap();
        let out = log
            .iter()
            .filter(|(a, _)| *a == callee)
            .map(|(_, w)| w.clone())
            .collect();
        e.call_log = Some(log);
        out
    }

    #[test]
    fn resolves_the_package_extra_and_the_actor_list_for_a_resolved_reference() {
        let mut e = engine(0x46, true);
        let reference = reference(&mut e);
        let package = e.mem.alloc(0x18);
        e.mem.set_u32(package + 0xc, reference);
        e.mem.set_u32(package + 0x14, 0x55);
        e.register_double(GET_EXTRA_DATA, move |_, a| {
            (if a[1] == 0x19 { package } else { 0 }).into_ret()
        });
        let entries = list(&mut e, &[0x66, 0x44]);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(holder + 0xc, entries);
        e.register_double(ACTOR_LIST_EXTRA_00422700, move |_, _| holder.into_ret());
        // Removing the failed first item makes the walk see the next one in its place.
        let removed = Rc::new(RefCell::new(0));
        let count = removed.clone();
        e.register_double(REMOVE_ITEM_0063F7B0, move |e, a| {
            *count.borrow_mut() += 1;
            e.mem.set_u32(a[0], 0x44);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(
            0x0042_6020,
            &args![0x0050_0000u32, 0x2_0000u32, 0u32, reference, 0u32],
        );
        assert_eq!(e.mem.u32(package + 0x14), 0x1055);
        assert_eq!(calls(&mut e, 0x0060_0158), vec![vec![reference]]);
        assert_eq!(
            calls(&mut e, CALCULATE_PROCEDURE_TYPE),
            vec![vec![reference, 0x1055]]
        );
        assert_eq!(*removed.borrow(), 1);
        assert_eq!(e.mem.u32(entries), 0x1044);
        // The teleport branch is the other side of the resolved test.
        assert!(calls(&mut e, TELEPORT_EXTRA_00418460).is_empty());
    }

    #[test]
    fn an_unresolved_reference_resolves_the_teleport_extra_with_flag_0x20000() {
        let mut e = engine(0x46, false);
        let reference = reference(&mut e);
        e.call_log = Some(vec![]);
        e.call(
            0x0042_6020,
            &args![0x0050_0000u32, 0x2_0000u32, 0u32, reference, 0u32],
        );
        assert_eq!(
            calls(&mut e, TELEPORT_RESOLVE_0043A9F0),
            vec![vec![0x0077_0000]]
        );
        assert!(calls(&mut e, GET_EXTRA_DATA).is_empty());
        let mut e = engine(0x46, false);
        e.call_log = Some(vec![]);
        e.call(
            0x0042_6020,
            &args![0x0050_0000u32, !0x2_0000u32, 0u32, 0u32, 0u32],
        );
        assert!(calls(&mut e, TELEPORT_EXTRA_00418460).is_empty());
    }

    #[test]
    fn old_saves_resolve_the_dropped_item_list() {
        let mut e = engine(0x45, false);
        // Items: 0x11 resolves, 0x66 fails after it (removed through the previous entry).
        let entries = list(&mut e, &[0x11, 0x66]);
        e.register_double(GET_DROPPED_ITEM_LIST, move |_, _| entries.into_ret());
        e.register(REMOVE_NEXT_00905330, |e, a| {
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        let reference = reference(&mut e);
        e.call_log = Some(vec![]);
        e.call(
            0x0042_6020,
            &args![0x0050_0000u32, 0u32, 0u32, reference, 0u32],
        );
        assert_eq!(e.mem.u32(entries), 0x1011);
        assert_eq!(
            calls(&mut e, ADD_DROPPED_ITEM),
            vec![vec![0x1031, reference]]
        );
        let removes = calls(&mut e, REMOVE_NEXT_00905330);
        assert_eq!(removes.len(), 1);
        assert_eq!(removes[0][0], entries);
        assert!(calls(&mut e, GET_ITEM_DROPPER).is_empty());
    }

    #[test]
    fn a_failed_first_dropped_item_is_removed_until_the_entry_is_empty() {
        let mut e = engine(0x10, false);
        let entries = list(&mut e, &[0x66]);
        e.register_double(GET_DROPPED_ITEM_LIST, move |_, _| entries.into_ret());
        e.register(REMOVE_ITEM_0063F7B0, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0042_6020, &args![0x0050_0000u32, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(calls(&mut e, REMOVE_ITEM_0063F7B0), vec![vec![entries]]);
    }

    #[test]
    fn new_saves_resolve_the_item_dropper_and_every_save_the_head_track_target() {
        let mut e = engine(0x46, false);
        e.register(GET_ITEM_DROPPER, |_, _| 0x22u32.into_ret());
        e.register(GET_HEAD_TRACK_TARGET, |_, _| 0x33u32.into_ret());
        let reference = reference(&mut e);
        e.call_log = Some(vec![]);
        e.call(
            0x0042_6020,
            &args![0x0050_0000u32, 0u32, 0u32, reference, 0u32],
        );
        assert_eq!(
            calls(&mut e, ADD_DROPPED_ITEM),
            vec![vec![0x0050_0000, 0x1022]]
        );
        assert_eq!(
            calls(&mut e, CALLEE_0041E000),
            vec![vec![0x1042, reference]]
        );
        assert_eq!(
            calls(&mut e, SET_ITEM_DROPPER),
            vec![vec![0x0050_0000, 0x1033]]
        );
        assert!(calls(&mut e, GET_DROPPED_ITEM_LIST).is_empty());
        // A dropper that does not resolve is still handed on, without the extra list call.
        let mut e = engine(0x46, false);
        e.register(GET_ITEM_DROPPER, |_, _| 0x66u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0042_6020, &args![0x0050_0000u32, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(calls(&mut e, ADD_DROPPED_ITEM), vec![vec![0x0050_0000, 0]]);
        assert!(calls(&mut e, CALLEE_0041E000).is_empty());
    }
}
