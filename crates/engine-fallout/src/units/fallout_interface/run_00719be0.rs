//! `fallout/interface/run_00719be0` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The word `Setting` base constructor (`00404920`), the vtable of the
/// interface settings and their collection (`004de490`).
const SETTING_CONSTRUCT_WORD: u32 = 0x0040_4920;
const SETTING_VTABLE: u32 = 0x0102_2738;
const COLLECTION: u32 = 0x004d_e490;
/// Slot of a settings collection's vtable that adds a setting.
const COLLECTION_ADD_SLOT: u32 = 0x4;

// Translated from 00719c00 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// The constructor of a word setting of the `004de490` collection
/// (`thiscall`, `RET 8`): the word base constructor (`00404920`), the vtable
/// `01022738`, then adds itself to the collection (vtable slot +4). Returns
/// `this`. The exception-unwinding frame is not translated.
pub fn fn_00719c00(e: &mut Engine, this: Ptr, name: u32, value: u32) -> Ptr {
    e.call(SETTING_CONSTRUCT_WORD, &args![this, name, value]);
    e.mem.set_u32(this.addr(), SETTING_VTABLE);
    let collection = e.call(COLLECTION, &[]).u32();
    e.vcall(collection, COLLECTION_ADD_SLOT, &args![this]);
    this
}

/// The vtable `0106f798` (RTTI `.?AV?$BSSimpleArray@PAVTile@@$0EAA@@@`).
const TILE_ARRAY_VTABLE: u32 = 0x0106_f798;
/// `008454f0` (`thiscall`, one word): the array's clear.
const ARRAY_CLEAR_008454F0: u32 = 0x0084_54f0;
/// List-box helpers (`thiscall`): `007a1950` (two words), `007242c0` (one word, answers a
/// byte), `ListBox<ItemChange_P>::AddItem` (`0071a320`, four words) and
/// `ListBox<ItemChange_P>::Synchronize` (`0071a670`).
const LIST_SET_007A1950: u32 = 0x007a_1950;
const LIST_STATE_007242C0: u32 = 0x0072_42c0;
const LIST_ADD_ITEM: u32 = 0x0071_a320;
const LIST_SYNCHRONIZE: u32 = 0x0071_a670;
/// `00575510` (`thiscall`, three words): the reference's inventory iterator and changes.
const INVENTORY_00575510: u32 = 0x0057_5510;
/// `InventoryChanges::GetNextOEIFastInventoryItem` (`004ca330`, `thiscall`, one word).
const NEXT_ITEM: u32 = 0x004c_a330;
/// The iterator's scalar deleting destructor (`004bf630`, `thiscall`, one word).
const ITERATOR_DELETE: u32 = 0x004b_f630;
/// Item helpers (`thiscall`): `00726070` (+4, the count), `0044ddc0` (+8, the form),
/// `ItemChange::GetModSlots` (`004bd820`), `ItemChange::ItemChange_ov3` (`0076b630`),
/// `004bc810` (one word: copies an item) and `006ecd40` (one word: sets the count).
const ITEM_COUNT_00726070: u32 = 0x0072_6070;
const ITEM_FORM_0044DDC0: u32 = 0x0044_ddc0;
const ITEM_HAS_MODS: u32 = 0x004b_d820;
const ITEM_CONSTRUCTOR: u32 = 0x0076_b630;
const ITEM_COPY_004BC810: u32 = 0x004b_c810;
const ITEM_SET_COUNT_006ECD40: u32 = 0x006e_cd40;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five words) from `01183108` to `01183158`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const CAST_SOURCE_01183108: u32 = 0x0118_3108;
const CAST_TARGET_01183158: u32 = 0x0118_3158;
/// `MapMarkerData::GetLocationName` (engine map name for the name getter `00408da0`).
const FORM_NAME_00408DA0: u32 = 0x0040_8da0;
/// `0044a670` (`cdecl`, one word): the string's length.
const STRING_LENGTH_0044A670: u32 = 0x0044_a670;
/// `00406d30` and `00406d50` (`cdecl`, three words: buffer, size, string): copy and
/// append; the appended string at `010345e8`.
const STRING_COPY_00406D30: u32 = 0x0040_6d30;
const STRING_APPEND_00406D50: u32 = 0x0040_6d50;
const MODDED_SUFFIX_010345E8: u32 = 0x0103_45e8;
/// The allocator `00401000` (`cdecl`, one word).
const ALLOCATE: u32 = 0x0040_1000;

// Translated from 00719be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of a `BSSimpleArray<Tile *>` (RTTI of the vtable it stores, `0106f798`;
/// unnamed in the engine map; `thiscall`): stores the vtable and clears the array
/// (`008454f0` with 1).
pub fn fn_00719be0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TILE_ARRAY_VTABLE);
    e.call(ARRAY_CLEAR_008454F0, &args![this, 1u32]);
}

// Translated from 00719ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills an item list box from a reference's inventory (unnamed in the engine map;
/// `thiscall`, `RET 0x18`: the reference, a filter function, a word passed to `AddItem`, a
/// callback function, a split flag, an inventory flag byte). Needs the reference and the
/// list's words +0xc and +0x18. The list is reset (its slot +0x1c, `007a1950(1, 1)`, its
/// `007242c0(0)` byte kept, `007a1950(0, 0)`); then each inventory item (`00575510` with
/// the flag byte, `004ca330`) that the filter (`cdecl`, when given) does not take is added
/// (`0071a320` with the item, its form's name in a 128-byte buffer with the suffix at
/// `010345e8` when it has mods, the word and 0) and given to the callback (`cdecl`, entry
/// and item, when given). With the split flag an item of several gets one entry per unit:
/// after the first a new 0xc-byte item copies it (`004bc810`), each entry with a count of
/// 1 (`006ecd40`). The iterator is deleted, a list given the word is synchronized, and the
/// list gets `007a1950(0, kept byte)`.
///
/// The exception frame (`FS:[0]`) and the /GS stack cookie check (`00ec408c`) are not
/// translated.
// Translation incomplete: 00719ef0 - the FS:[0] exception frame and the /GS stack cookie check (00ec408c) are not translated
#[allow(clippy::too_many_arguments)]
pub fn fn_00719ef0(
    e: &mut Engine,
    this: Ptr,
    reference: u32,
    filter: u32,
    word: u32,
    callback: u32,
    split: u8,
    flags: u8,
) {
    let base = this.addr();
    if reference == 0 || e.mem.u32(base + 0xc) == 0 || e.mem.u32(base + 0x18) == 0 {
        return;
    }
    e.vcall(base, 0x1c, &args![]);
    e.call(LIST_SET_007A1950, &args![this, 1u32, 1u32]);
    let kept = e.call(LIST_STATE_007242C0, &args![this, 0u32]).u8();
    e.call(LIST_SET_007A1950, &args![this, 0u32, 0u32]);
    e.with_stack(0x88, |e, locals| {
        let iterator_slot = locals.addr();
        let changes_slot = locals.addr() + 4;
        let buffer = locals.addr() + 8;
        e.call(
            INVENTORY_00575510,
            &args![reference, u32::from(flags), iterator_slot, changes_slot],
        );
        let iterator = e.mem.u32(iterator_slot);
        let changes = e.mem.u32(changes_slot);
        if iterator == 0 || changes == 0 {
            return;
        }
        let mut item = e.call(NEXT_ITEM, &args![changes, iterator]).u32();
        while item != 0 {
            let skipped = filter != 0 && e.call(filter, &args![item]).u8() != 0;
            if !skipped {
                let mut copies = 0i32;
                let count = e.call(ITEM_COUNT_00726070, &args![item]).i32();
                loop {
                    let form = e.call(ITEM_FORM_0044DDC0, &args![item]).u32();
                    let form = e
                        .call(
                            DYNAMIC_CAST,
                            &args![form, 0u32, CAST_SOURCE_01183108, CAST_TARGET_01183158, 0u32],
                        )
                        .u32();
                    let name = e.call(FORM_NAME_00408DA0, &args![form]).u32();
                    let length = e.call(STRING_LENGTH_0044A670, &args![name]).u32();
                    let name = e.call(FORM_NAME_00408DA0, &args![form]).u32();
                    e.call(
                        STRING_COPY_00406D30,
                        &args![buffer, length.wrapping_add(1), name],
                    );
                    if e.call(ITEM_HAS_MODS, &args![item]).u8() != 0 {
                        e.call(
                            STRING_APPEND_00406D50,
                            &args![buffer, length.wrapping_add(2), MODDED_SUFFIX_010345E8],
                        );
                    }
                    if split != 0 {
                        if copies > 0 {
                            let block = e.call(ALLOCATE, &args![0xcu32]).u32();
                            let copy = if block != 0 {
                                e.call(ITEM_CONSTRUCTOR, &args![block]).u32()
                            } else {
                                0
                            };
                            e.call(ITEM_COPY_004BC810, &args![copy, item]);
                            item = copy;
                            e.call(ITEM_SET_COUNT_006ECD40, &args![item, 1u32]);
                        } else if count > 1 {
                            e.call(ITEM_SET_COUNT_006ECD40, &args![item, 1u32]);
                        }
                        copies += 1;
                    }
                    let entry = e
                        .call(LIST_ADD_ITEM, &args![this, item, buffer, word, 0u32])
                        .u32();
                    if callback != 0 {
                        e.call(callback, &args![entry, item]);
                    }
                    if !(split != 0 && copies < count) {
                        break;
                    }
                }
            }
            item = e.call(NEXT_ITEM, &args![changes, iterator]).u32();
        }
        let iterator = e.mem.u32(iterator_slot);
        if iterator != 0 {
            e.call(ITERATOR_DELETE, &args![iterator, 1u32]);
        }
    });
    if word != 0 {
        e.call(LIST_SYNCHRONIZE, &args![this]);
    }
    e.call(LIST_SET_007A1950, &args![this, 0u32, u32::from(kept)]);
}

/// `ItemChange::GetFullName` (`004be2d0`, `thiscall`), the name comparison `00469880`
/// (cdecl, two names), the form's type byte (`00401170`, `thiscall`: the byte at +4) and
/// the `TESHealthForm` type descriptor (`01186c3c`) the cast targets.
const ITEM_FULL_NAME: u32 = 0x004b_e2d0;
const COMPARE_NAMES_00469880: u32 = 0x0046_9880;
const FORM_TYPE_00401170: u32 = 0x0040_1170;
const HEALTH_FORM_TYPE_01186C3C: u32 = 0x0118_6c3c;

// Translated from 00719c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, two items; a list sort's comparison): `00469880` of the two items'
/// full names (`ItemChange::GetFullName`, the second item asked first).
pub fn fn_00719c80(e: &mut Engine, first: u32, second: u32) -> i32 {
    let second_name = e.call(ITEM_FULL_NAME, &args![second]).u32();
    let first_name = e.call(ITEM_FULL_NAME, &args![first]).u32();
    e.call(COMPARE_NAMES_00469880, &args![first_name, second_name])
        .i32()
}

// Translated from 00719ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, two items; a list sort's comparison): -1, 1 or 0 as the first item's
/// count (`00726070`) is below, above or equal to the second's.
pub fn fn_00719ca0(e: &mut Engine, first: u32, second: u32) -> i32 {
    let a = e.call(ITEM_COUNT_00726070, &args![first]).i32();
    let b = e.call(ITEM_COUNT_00726070, &args![second]).i32();
    if a < b {
        return -1;
    }
    let a = e.call(ITEM_COUNT_00726070, &args![first]).i32();
    let b = e.call(ITEM_COUNT_00726070, &args![second]).i32();
    if a > b {
        1
    } else {
        0
    }
}

/// The item's form cast to `TESHealthForm` (`__RTDynamicCast(form, 0, 01183108,
/// 01186c3c, 0)`).
fn health_form(e: &mut Engine, item: u32) -> u32 {
    let form = e.call(ITEM_FORM_0044DDC0, &args![item]).u32();
    e.call(
        DYNAMIC_CAST,
        &args![
            form,
            0u32,
            CAST_SOURCE_01183108,
            HEALTH_FORM_TYPE_01186C3C,
            0u32
        ],
    )
    .u32()
}

// Translated from 00719cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, two items; a list sort's comparison): -1, 1 or 0 as the first item's
/// health (its form as a `TESHealthForm`, slot +0x10) is below, above or equal to the
/// second's, compared unsigned.
pub fn fn_00719cf0(e: &mut Engine, first: u32, second: u32) -> i32 {
    let first = health_form(e, first);
    let second = health_form(e, second);
    let a = e.vcall(first, 0x10, &args![]).u32();
    let b = e.vcall(second, 0x10, &args![]).u32();
    if a < b {
        return -1;
    }
    let a = e.vcall(first, 0x10, &args![]).u32();
    let b = e.vcall(second, 0x10, &args![]).u32();
    if a > b {
        1
    } else {
        0
    }
}

/// Whether the item's form type (`00401170` of `0044ddc0`) is one of `types`, asked once
/// per type in order until one matches; false without an item.
fn item_form_type_is(e: &mut Engine, item: u32, types: &[u32]) -> bool {
    if item == 0 {
        return false;
    }
    for &kind in types {
        let form = e.call(ITEM_FORM_0044DDC0, &args![item]).u32();
        if e.call(FORM_TYPE_00401170, &args![form]).u32() == kind {
            return true;
        }
    }
    false
}

// Translated from 00719da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one item; a list filter): whether there is an item and its form type
/// is 0x28.
pub fn fn_00719da0(e: &mut Engine, item: u32) -> bool {
    item_form_type_is(e, item, &[0x28])
}

// Translated from 00719de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one item; a list filter): whether there is an item and its form type
/// is 0x1a or 0x18.
pub fn fn_00719de0(e: &mut Engine, item: u32) -> bool {
    item_form_type_is(e, item, &[0x1a, 0x18])
}

// Translated from 00719e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, one item; a list filter): whether there is an item and its form type
/// is one of 0x1f, 0x29, 0x67, 0x32, 0x6c, 0x74, 0x73.
pub fn fn_00719e30(e: &mut Engine, item: u32) -> bool {
    item_form_type_is(e, item, &[0x1f, 0x29, 0x67, 0x32, 0x6c, 0x74, 0x73])
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00719c00, fn_00719c00(Ptr, u32, u32) -> Ptr),
        entry!(0x0071_9be0, fn_00719be0(Ptr)),
        entry!(0x0071_9ef0, fn_00719ef0(Ptr, u32, u32, u32, u32, u8, u8)),
        entry!(0x0071_9c80, fn_00719c80(u32, u32) -> i32),
        entry!(0x0071_9ca0, fn_00719ca0(u32, u32) -> i32),
        entry!(0x0071_9cf0, fn_00719cf0(u32, u32) -> i32),
        entry!(0x0071_9da0, fn_00719da0(u32) -> bool),
        entry!(0x0071_9de0, fn_00719de0(u32) -> bool),
        entry!(0x0071_9e30, fn_00719e30(u32) -> bool),
    ]
}

#[cfg(test)]
mod list_box_tests {
    use super::*;

    fn called(e: &Engine, target: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == target)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn tile_array_destructor_stores_the_vtable_and_clears() {
        let mut e = Engine::new();
        e.register(ARRAY_CLEAR_008454F0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0071_9be0, &[this]);
        assert_eq!(e.mem.u32(this), TILE_ARRAY_VTABLE);
        assert_eq!(called(&e, ARRAY_CLEAR_008454F0), vec![vec![this, 1]]);
    }

    const FILTER: u32 = 0x0000_7100;
    const CALLBACK: u32 = 0x0000_7200;
    const RESET_SLOT: u32 = 0x0000_701c;

    /// A list box over two items: 0x100 (count 3, modded) and 0x200 (count 1); the filter
    /// takes 0x200 when `filter_takes`.
    fn engine(filter_takes: bool) -> (Engine, u32) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x1c, RESET_SLOT);
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0xc, 1);
        e.mem.set_u32(this + 0x18, 1);
        for address in [
            RESET_SLOT,
            LIST_SET_007A1950,
            LIST_SYNCHRONIZE,
            ITERATOR_DELETE,
            STRING_COPY_00406D30,
            STRING_APPEND_00406D50,
            ITEM_COPY_004BC810,
            ITEM_SET_COUNT_006ECD40,
            CALLBACK,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(LIST_STATE_007242C0, |_, _| 0x101u32.into_ret());
        e.register(INVENTORY_00575510, |e, a| {
            e.mem.set_u32(a[2], 0x9000);
            e.mem.set_u32(a[3], 0x9100);
            Ret::default()
        });
        e.register_double(NEXT_ITEM, {
            let mut items = vec![0u32, 0x200, 0x100];
            move |_, _| items.pop().unwrap_or(0).into_ret()
        });
        e.register_double(FILTER, move |_, a| {
            (filter_takes && a[0] == 0x200).into_ret()
        });
        e.register(ITEM_COUNT_00726070, |_, a| {
            if a[0] == 0x100 { 3u32 } else { 1 }.into_ret()
        });
        e.register(ITEM_FORM_0044DDC0, |_, a| (a[0] + 8).into_ret());
        e.register(DYNAMIC_CAST, |_, a| (a[0] + 0x1000).into_ret());
        e.register(FORM_NAME_00408DA0, |_, a| (a[0] + 0x10).into_ret());
        e.register(STRING_LENGTH_0044A670, |_, _| 5u32.into_ret());
        e.register(ITEM_HAS_MODS, |_, a| (a[0] == 0x100).into_ret());
        e.register(ALLOCATE, |_, _| 0x8800u32.into_ret());
        e.register(ITEM_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(LIST_ADD_ITEM, |_, a| (a[1] + 0x10_0000).into_ret());
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fills_one_entry_per_item() {
        let (mut e, this) = engine(false);
        e.call(0x0071_9ef0, &[this, 0x5000, FILTER, 0x77, CALLBACK, 0, 4]);
        assert_eq!(called(&e, RESET_SLOT), vec![vec![this]]);
        assert_eq!(
            called(&e, LIST_SET_007A1950),
            vec![vec![this, 1, 1], vec![this, 0, 0], vec![this, 0, 1]]
        );
        assert_eq!(called(&e, INVENTORY_00575510)[0][..2], [0x5000, 4]);
        let added = called(&e, LIST_ADD_ITEM);
        let items: Vec<u32> = added.iter().map(|w| w[1]).collect();
        assert_eq!(items, vec![0x100, 0x200]);
        assert_eq!(added[0][3..], [0x77, 0]);
        let copies = called(&e, STRING_COPY_00406D30);
        assert_eq!(copies[0][1..], [6, 0x1118]);
        assert_eq!(copies[0][0], added[0][2]);
        assert_eq!(
            called(&e, STRING_APPEND_00406D50),
            vec![vec![copies[0][0], 7, MODDED_SUFFIX_010345E8]]
        );
        assert_eq!(
            called(&e, CALLBACK),
            vec![vec![0x10_0100, 0x100], vec![0x10_0200, 0x200]]
        );
        assert_eq!(called(&e, ITERATOR_DELETE), vec![vec![0x9000, 1]]);
        assert_eq!(called(&e, LIST_SYNCHRONIZE), vec![vec![this]]);
        assert!(called(&e, ITEM_SET_COUNT_006ECD40).is_empty());
    }

    #[test]
    fn split_makes_an_entry_per_unit_and_the_filter_skips() {
        let (mut e, this) = engine(true);
        e.call(0x0071_9ef0, &[this, 0x5000, FILTER, 0, 0, 1, 0]);
        let items: Vec<u32> = called(&e, LIST_ADD_ITEM).iter().map(|w| w[1]).collect();
        // Three entries for the count-3 item (two copies), none for the filtered one.
        assert_eq!(items, vec![0x100, 0x8800, 0x8800]);
        assert_eq!(
            called(&e, ITEM_COPY_004BC810),
            vec![vec![0x8800, 0x100], vec![0x8800, 0x8800]]
        );
        assert_eq!(called(&e, ITEM_SET_COUNT_006ECD40).len(), 3);
        assert!(called(&e, LIST_SYNCHRONIZE).is_empty());
    }

    #[test]
    fn needs_the_reference_and_the_list() {
        let (mut e, this) = engine(false);
        e.call(0x0071_9ef0, &[this, 0, 0, 0, 0, 0, 0]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.mem.set_u32(this + 0x18, 0);
        e.call(0x0071_9ef0, &[this, 0x5000, 0, 0, 0, 0, 0]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        // No iterator: no item, no delete, but the list is still restored.
        let (mut e, this) = engine(false);
        e.register(INVENTORY_00575510, |_, _| Ret::default());
        e.call(0x0071_9ef0, &[this, 0x5000, 0, 0, 0, 0, 0]);
        assert!(called(&e, NEXT_ITEM).is_empty());
        assert!(called(&e, ITERATOR_DELETE).is_empty());
        assert_eq!(called(&e, LIST_SET_007A1950).len(), 3);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_word_setting_joins_its_collection() {
        let mut e = Engine::new();
        let c = e.mem.alloc(8);
        e.put_vtable(0x0090_0000, &[0, 0x0090_1000]);
        e.mem.set_u32(c, 0x0090_0000);
        e.register_double(COLLECTION, move |_, _| c.into_ret());
        e.register(0x0090_1000, |_, _| Ret::default());
        let s = e.mem.alloc(0xc);
        e.call(0x00719c00, &[s, 0x40, 0x1234]);
        assert_eq!(e.mem.u32(s), SETTING_VTABLE);
        assert_eq!(e.mem.u32(s + 4), 0x1234);
    }
}

#[cfg(test)]
mod n059_tests {
    use super::*;

    const VTABLE: u32 = 0x0f00_0000;

    #[test]
    fn comparisons_order_by_name_count_and_health() {
        let mut e = Engine::new();
        e.register(ITEM_FULL_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(COMPARE_NAMES_00469880, |_, a| {
            (a[0] as i32 - a[1] as i32).into_ret()
        });
        assert_eq!(e.call(0x0071_9c80, &args![0x10u32, 0x30u32]).i32(), -0x20);
        e.register(ITEM_COUNT_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let item = |e: &mut Engine, count: u32| {
            let block = e.mem.alloc(0xc);
            e.mem.set_u32(block + 4, count);
            block
        };
        let (two, five) = (item(&mut e, 2), item(&mut e, 5));
        let compare = |e: &mut Engine, f: u32, a: u32, b: u32| e.call(f, &args![a, b]).i32();
        assert_eq!(compare(&mut e, 0x0071_9ca0, two, five), -1);
        assert_eq!(compare(&mut e, 0x0071_9ca0, five, two), 1);
        assert_eq!(compare(&mut e, 0x0071_9ca0, two, two), 0);
        // Health: the form (+8) is its own health form; slot +0x10 answers the word
        // after the vtable (unsigned: 0xffffffff is the highest).
        e.register(ITEM_FORM_0044DDC0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(0x0e00_0010, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.put_vtable(VTABLE, &[0, 0, 0, 0, 0x0e00_0010]);
        let health = |e: &mut Engine, value: u32| {
            let form = e.mem.alloc(8);
            e.mem.set_u32(form, VTABLE);
            e.mem.set_u32(form + 4, value);
            let block = e.mem.alloc(0xc);
            e.mem.set_u32(block + 8, form);
            block
        };
        let (low, high) = (health(&mut e, 3), health(&mut e, 0xffff_ffff));
        assert_eq!(compare(&mut e, 0x0071_9cf0, low, high), -1);
        assert_eq!(compare(&mut e, 0x0071_9cf0, high, low), 1);
        assert_eq!(compare(&mut e, 0x0071_9cf0, low, low), 0);
    }

    #[test]
    fn filters_test_the_form_type() {
        let mut e = Engine::new();
        e.register(ITEM_FORM_0044DDC0, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(FORM_TYPE_00401170, |_, a| a[0].into_ret());
        let item = |e: &mut Engine, kind: u32| {
            let block = e.mem.alloc(0xc);
            e.mem.set_u32(block + 8, kind);
            block
        };
        let filter = |e: &mut Engine, f: u32, item: u32| e.call(f, &args![item]).bool();
        assert!(!filter(&mut e, 0x0071_9da0, 0));
        let weapon = item(&mut e, 0x28);
        assert!(filter(&mut e, 0x0071_9da0, weapon));
        assert!(!filter(&mut e, 0x0071_9de0, weapon));
        let armor = item(&mut e, 0x18);
        assert!(filter(&mut e, 0x0071_9de0, armor));
        assert!(!filter(&mut e, 0x0071_9e30, armor));
        for kind in [0x1f, 0x29, 0x67, 0x32, 0x6c, 0x74, 0x73] {
            let thing = item(&mut e, kind);
            assert!(filter(&mut e, 0x0071_9e30, thing));
        }
        assert!(!filter(&mut e, 0x0071_9e30, 0));
    }
}
