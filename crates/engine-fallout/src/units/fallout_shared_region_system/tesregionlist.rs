//! `fallout shared/region system/tesregionlist.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0096a2d0` (`ECX` = list): clears the list's two words (the constructor
/// of the `BSSimpleList<TESRegion *>` base at `+4`).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `TESRegionList`'s vtable (RTTI `.?AVTESRegionList@@`).
const VTABLE: u32 = 0x0102_40a0;
/// How many lists of region data the constructor discards, once, when the
/// first region list that owns its memory is made (9 pairs of pointers).
const DISCARDED_PAIRS: u32 = 9;
/// The table of those pointers: pair `i` is at `011c7f80 + i * 8`, its two
/// words at `+0` and `+4`.
const POINTER_PAIRS: u32 = 0x011c_7f80;
/// The count of region lists made so far that own their memory.
const OWNING_LISTS: u32 = 0x011c_7fc8;

layout! {
    /// `TESRegionList` (Xbox PDB), 0x10 bytes.
    pub struct TESRegionList: 0x10 {
        /// `bOwnsRegionMemory` (Xbox PDB).
        0x0c bOwnsRegionMemory: u8,
    }
}

/// Deletes the object whose address is in the table slot at `slot`, if any:
/// the slot is cleared, then the object's scalar deleting destructor (vtable
/// slot 0, argument 1) is called.
fn discard(e: &mut Engine, slot: u32) {
    let object = e.mem.u32(slot);
    if object != 0 {
        e.mem.set_u32(slot, 0);
        e.vcall(object, 0, &args![1u32]);
    }
}

// Translated from 004f6320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f6320 - the exception-unwinding frame (FS:[0]) is not translated.
/// `TESRegionList::TESRegionList` (Xbox PDB; `thiscall`, `RET 4`): runs the
/// list constructor on the base at `+4`, stores the vtable and
/// `bOwnsRegionMemory` (`owns_region_memory`, a byte). A list that owns its
/// region memory, when none has been made before (the count at `011c7fc8` is
/// 0), first discards the nine pairs of objects in the table at `011c7f80`
/// (each non-null one is cleared in the table and deleted through its
/// virtual slot 0 with argument 1); it then counts itself in. Returns `this`.
/// The exception-unwinding frame is not translated.
pub fn tes_region_list_tes_region_list(
    e: &mut Engine,
    this: Ptr<TESRegionList>,
    owns_region_memory: u8,
) -> Ptr<TESRegionList> {
    e.call(LIST_CONSTRUCT, &args![this.byte_add(4)]);
    e.mem.set_u32(this.addr(), VTABLE);
    e.set(this, TESRegionList::bOwnsRegionMemory, owns_region_memory);
    if e.get(this, TESRegionList::bOwnsRegionMemory) != 0 {
        if e.global::<u32>(OWNING_LISTS) == 0 {
            for pair in 0..DISCARDED_PAIRS {
                discard(e, POINTER_PAIRS + pair * 8);
                discard(e, POINTER_PAIRS + pair * 8 + 4);
            }
        }
        let count: u32 = e.global(OWNING_LISTS);
        e.set_global(OWNING_LISTS, count.wrapping_add(1));
    }
    this
}

// Translated from 004f6480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionList` scalar deleting destructor (engine map name): runs the destructor body `004f64b0` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn tes_region_list_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x004f_64b0, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// List helpers: the item pointer's address of a node (`006815c0`) and the
/// next node (`00726070`).
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// `0063f7b0` (thiscall on the list base, no stack words); `004f6640` runs
/// it once per first item until the first item is null, so it removes the
/// head.
const LIST_REMOVE_HEAD_0063F7B0: u32 = 0x0063_f7b0;
/// Clears the list base (`00470470`, thiscall, no stack words).
const LIST_CLEAR_00470470: u32 = 0x0047_0470;
/// The `BSSimpleList` destructor of the base at +4 (`0046ffb0`).
const LIST_DESTRUCT_0046FFB0: u32 = 0x0046_ffb0;
/// Whether the list holds an item (`005f65d0`, thiscall on the list base,
/// the item's address) and the list's add (`005ae3d0`, the same shape).
const LIST_CONTAINS_005F65D0: u32 = 0x005f_65d0;
const LIST_ADD_005AE3D0: u32 = 0x005a_e3d0;
/// The list's item count (`005ae380`, engine map `VATS::GetCount`, folded).
const LIST_COUNT_005AE380: u32 = 0x005a_e380;
/// A form's id (`0084e3a0`, the word at +0x0c).
const FORM_ID: u32 = 0x0084_e3a0;
/// The game's allocator (`00401000`, cdecl, the size).
const ALLOCATE: u32 = 0x0040_1000;
/// Region queries (thiscall on the region): a skip test (`00440d80`, a
/// `bool`), the region's worldspace word (`007af430`), its data list
/// (`009611e0`), its point lists (`00441110`).
const REGION_SKIP_00440D80: u32 = 0x0044_0d80;
const REGION_WORLDSPACE_007AF430: u32 = 0x007a_f430;
const REGION_DATA_LIST_009611E0: u32 = 0x0096_11e0;
const REGION_POINT_LISTS_00441110: u32 = 0x0044_1110;
/// `TESRegionDataList::Find` (`004f35b0`, thiscall on the data list, the
/// data type).
const REGION_DATA_LIST_FIND: u32 = 0x004f_35b0;
/// Data queries (thiscall on a region data): ignored (`004f6de0`, a
/// `bool`), overriding (`004f1540`, a `bool`) and its priority (`005bb4d0`,
/// a byte).
const DATA_IGNORED_004F6DE0: u32 = 0x004f_6de0;
const DATA_OVERRIDE_004F1540: u32 = 0x004f_1540;
const DATA_PRIORITY_005BB4D0: u32 = 0x005b_b4d0;
/// A region point list's test of a point (`004f8360`, thiscall, the point).
const POINT_LIST_CONTAINS_004F8360: u32 = 0x004f_8360;
/// Region data virtual slots: a copy (+0x14, no stack words) and a merge
/// into a copy (+0x1c: the data and the list's count).
const DATA_COPY_SLOT: u32 = 0x14;
const DATA_MERGE_SLOT: u32 = 0x1c;
/// `TESRegionPoint` (`004f6fc0` constructs one) and its two coordinate
/// setters (`0052ce20`, `004f5d90`; a `float` each).
const REGION_POINT_CONSTRUCT: u32 = 0x004f_6fc0;
const REGION_POINT_SET_X_0052CE20: u32 = 0x0052_ce20;
const REGION_POINT_SET_Y_004F5D90: u32 = 0x004f_5d90;

/// The table pair `kind` (`011c7f80 + kind * 8`): the derived data at +0,
/// the region list it was derived from at +4.
fn pair_slot(kind: u32) -> u32 {
    POINTER_PAIRS.wrapping_add(kind.wrapping_mul(8))
}

/// `this + 4` for a non-null list, else 0 (the code's null-adjusted base).
fn list_base(list: u32) -> u32 {
    if list == 0 {
        0
    } else {
        list + 4
    }
}

/// The item a list node holds (`*006815c0(node)`).
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_ITEM, &args![node]).u32();
    e.mem.u32(slot)
}

// Translated from 004f64b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f64b0 - the function's C++ exception frame (FS:[0], unwind state of the list base) is not translated
/// `TESRegionList`'s destructor body (by the vtable it stores; unnamed in
/// the engine map): the vtable, its regions removed (`004f6640`); a list
/// that owns its region memory takes itself off the count at `011c7fc8`,
/// and the last one discards the nine pairs of the table at `011c7f80`
/// (each non-null object cleared in the table and deleted through its
/// virtual slot 0 with argument 1). Then the list base's destructor
/// (`0046ffb0` on +4).
pub fn fn_004f64b0(e: &mut Engine, this: Ptr<TESRegionList>) {
    e.mem.set_u32(this.addr(), VTABLE);
    fn_004f6640(e, this);
    if e.get(this, TESRegionList::bOwnsRegionMemory) != 0 {
        let count = e.global::<u32>(OWNING_LISTS).wrapping_sub(1);
        e.set_global(OWNING_LISTS, count);
        if count == 0 {
            for pair in 0..DISCARDED_PAIRS {
                discard(e, POINTER_PAIRS + pair * 8);
                discard(e, POINTER_PAIRS + pair * 8 + 4);
            }
        }
    }
    e.call(LIST_DESTRUCT_0046FFB0, &args![this.addr() + 4]);
}

// Translated from 004f6600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `region` to the list unless it is already there (thiscall,
/// `RET 4`; unnamed in the engine map): null gives false; otherwise
/// `005f65d0(base, &region)` and, when that answers false,
/// `005ae3d0(base, &region)`, with the address of the argument word.
/// Returns true for any non-null region.
pub fn fn_004f6600(e: &mut Engine, this: Ptr<TESRegionList>, region: u32) -> bool {
    if region == 0 {
        return false;
    }
    let base = this.addr() + 4;
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), region);
        if e.call(LIST_CONTAINS_005F65D0, &args![base, slot]).u32() & 0xff == 0 {
            e.call(LIST_ADD_005AE3D0, &args![base, slot]);
        }
    });
    true
}

// Translated from 004f6640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the list (unnamed in the engine map): while its first item is
/// non-null, removes the head (`0063f7b0`) and, when the list owns its
/// region memory, deletes that region through its virtual slot +0x10 with
/// argument 1.
pub fn fn_004f6640(e: &mut Engine, this: Ptr<TESRegionList>) {
    let base = this.addr() + 4;
    let mut region = node_item(e, base);
    while region != 0 {
        e.call(LIST_REMOVE_HEAD_0063F7B0, &args![base]);
        if e.get(this, TESRegionList::bOwnsRegionMemory) != 0 && region != 0 {
            e.vcall(region, 0x10, &args![1u32]);
        }
        region = node_item(e, base);
    }
}

// Translated from 004f66c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The list's region whose form id (`0084e3a0`) is `form_id`, or null
/// (thiscall, `RET 4`; unnamed in the engine map). Null items are skipped.
pub fn fn_004f66c0(e: &mut Engine, this: Ptr<TESRegionList>, form_id: u32) -> u32 {
    let mut node = list_base(this.addr());
    while node != 0 {
        if node_item(e, node) != 0 {
            let region = node_item(e, node);
            if e.call(FORM_ID, &args![region]).u32() == form_id {
                return node_item(e, node);
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    0
}

// Translated from 004f6730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether two region lists hold the same items in the same order
/// (thiscall, `RET 4`; unnamed in the engine map). A null list on either
/// side gives false.
pub fn fn_004f6730(e: &mut Engine, this: Ptr<TESRegionList>, other: u32) -> bool {
    let mut mine = list_base(this.addr());
    let mut theirs = list_base(other);
    if mine == 0 || theirs == 0 {
        return false;
    }
    while mine != 0 {
        let my_slot = e.call(NODE_ITEM, &args![mine]).u32();
        let their_slot = e.call(NODE_ITEM, &args![theirs]).u32();
        if e.mem.u32(my_slot) != e.mem.u32(their_slot) {
            return false;
        }
        mine = e.call(NODE_NEXT, &args![mine]).u32();
        theirs = e.call(NODE_NEXT, &args![theirs]).u32();
        if mine == 0 && theirs == 0 {
            return true;
        }
        if (mine == 0 && theirs != 0) || (mine != 0 && theirs == 0) {
            return false;
        }
    }
    true
}

// Translated from 004f6800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionList::GetDerivedData` (engine map; thiscall, `RET 0xc`): a
/// `TESRegionPoint` on the stack (`004f6fc0`) set from the two `float`s at
/// `point` (`0052ce20`, `004f5d90`; `FLD`/`FSTP` copies), then the derived
/// data of type `kind` for that point and `worldspace` (`004f6850`).
pub fn tes_region_list_get_derived_data(
    e: &mut Engine,
    this: Ptr<TESRegionList>,
    kind: u32,
    point: Ptr,
    worldspace: u32,
) -> u32 {
    e.with_stack(8, |e, local| {
        e.call(REGION_POINT_CONSTRUCT, &args![local]);
        let x = x87_quiet_bits(e.mem.u32(point.addr()));
        e.call(REGION_POINT_SET_X_0052CE20, &args![local, x]);
        let y = x87_quiet_bits(e.mem.u32(point.addr().wrapping_add(4)));
        e.call(REGION_POINT_SET_Y_004F5D90, &args![local, y]);
        fn_004f6850(e, this, kind, local.addr(), worldspace)
    })
}

// Translated from 004f6850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f6850 - the function's C++ exception frame (FS:[0], unwind state of the new TESRegionList) is not translated
/// The derived region data of type `kind` (thiscall, `RET 0xc`; unnamed in
/// the engine map). A new `TESRegionList` (0x10 bytes, `00401000`, not
/// owning) collects this list's regions that are not skipped
/// (`00440d80`), match `worldspace` when it is non-zero (`007af430`),
/// have data of the type (`TESRegionDataList::Find` on `009611e0`) that is
/// not ignored (`004f6de0`), and, for a non-null `point`, have a point list
/// containing it (`00441110`, `004f8360`). When the table pair `kind`
/// (`011c7f80`) has data and a list equal to the new one (`004f6730`), the
/// new list is cleared and deleted and the cached data returned. Otherwise
/// both cached objects are discarded and the data of the collected regions
/// combined: data that overrides (`004f1540`) drops what was built and the
/// highest priority one (`005bb4d0`) is copied at the end (slot +0x14);
/// before any override the first data is copied and later ones merged
/// into it (slot +0x1c with the list's count). A result is cached with the
/// new list (replacing, clearing and deleting the old ones); otherwise the
/// new list is cleared and deleted. Returns the result (null when none).
pub fn fn_004f6850(
    e: &mut Engine,
    this: Ptr<TESRegionList>,
    kind: u32,
    point: u32,
    worldspace: u32,
) -> u32 {
    let mut overriding = false;
    let mut best_priority: u16 = 0;
    let mut best_data = 0u32;
    let mut result = 0u32;
    let mut node = list_base(this.addr());
    let memory = e.call(ALLOCATE, &args![0x10u32]).u32();
    let mut new_list = if memory == 0 {
        0
    } else {
        tes_region_list_tes_region_list(e, Ptr::new(memory), 0).addr()
    };

    while node != 0 && node_item(e, node) != 0 {
        let region = node_item(e, node);
        if e.call(REGION_SKIP_00440D80, &args![region]).u32() & 0xff != 0 {
            node = e.call(NODE_NEXT, &args![node]).u32();
            continue;
        }
        if worldspace != 0 && worldspace != e.call(REGION_WORLDSPACE_007AF430, &args![region]).u32()
        {
            node = e.call(NODE_NEXT, &args![node]).u32();
            continue;
        }
        let data_list = e.call(REGION_DATA_LIST_009611E0, &args![region]).u32();
        let data = e.call(REGION_DATA_LIST_FIND, &args![data_list, kind]).u32();
        if data == 0 || e.call(DATA_IGNORED_004F6DE0, &args![data]).u32() & 0xff != 0 {
            node = e.call(NODE_NEXT, &args![node]).u32();
            continue;
        }
        if point != 0 {
            let mut lists = e.call(REGION_POINT_LISTS_00441110, &args![region]).u32();
            while lists != 0 && node_item(e, lists) != 0 {
                let list = node_item(e, lists);
                if e.call(POINT_LIST_CONTAINS_004F8360, &args![list, point])
                    .u32()
                    & 0xff
                    != 0
                {
                    fn_004f6600(e, Ptr::new(new_list), region);
                    break;
                }
                lists = e.call(NODE_NEXT, &args![lists]).u32();
            }
        } else {
            fn_004f6600(e, Ptr::new(new_list), region);
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }

    let slot = pair_slot(kind);
    let cached_list = e.mem.u32(slot + 4);
    if e.mem.u32(slot) != 0 && cached_list != 0 && fn_004f6730(e, Ptr::new(cached_list), new_list) {
        e.call(LIST_CLEAR_00470470, &args![new_list.wrapping_add(4)]);
        if new_list != 0 {
            e.vcall(new_list, 0, &args![1u32]);
        }
        return e.mem.u32(slot);
    }

    discard(e, slot);
    discard(e, slot + 4);
    node = list_base(new_list);
    while node != 0 && node_item(e, node) != 0 {
        let region = node_item(e, node);
        let data_list = e.call(REGION_DATA_LIST_009611E0, &args![region]).u32();
        let data = e.call(REGION_DATA_LIST_FIND, &args![data_list, kind]).u32();
        if data == 0 {
            node = e.call(NODE_NEXT, &args![node]).u32();
            continue;
        }
        if e.call(DATA_OVERRIDE_004F1540, &args![data]).u32() & 0xff != 0 {
            overriding = true;
            if result != 0 {
                e.vcall(result, 0, &args![1u32]);
                result = 0;
            }
            let priority = e.call(DATA_PRIORITY_005BB4D0, &args![data]).u32() & 0xff;
            if priority as i32 > i32::from(best_priority) {
                best_data = data;
                let priority = e.call(DATA_PRIORITY_005BB4D0, &args![data]).u32() & 0xff;
                best_priority = priority as u16;
            }
        } else if !overriding {
            if result != 0 {
                let count = e
                    .call(LIST_COUNT_005AE380, &args![new_list.wrapping_add(4)])
                    .u32();
                e.vcall(result, DATA_MERGE_SLOT, &args![data, count]);
            } else {
                result = e.vcall(data, DATA_COPY_SLOT, &args![]).u32();
            }
        }
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    if overriding {
        result = e.vcall(best_data, DATA_COPY_SLOT, &args![]).u32();
    }
    if result != 0 {
        let cached = e.mem.u32(slot);
        if cached != 0 {
            e.vcall(cached, 0, &args![1u32]);
        }
        e.mem.set_u32(slot, result);
        let old_list = e.mem.u32(slot + 4);
        if old_list != 0 {
            e.call(LIST_CLEAR_00470470, &args![old_list + 4]);
            let old_list = e.mem.u32(slot + 4);
            if old_list != 0 {
                e.vcall(old_list, 0, &args![1u32]);
            }
        }
        e.mem.set_u32(slot + 4, new_list);
        new_list = 0;
    }
    if new_list != 0 {
        e.call(LIST_CLEAR_00470470, &args![new_list + 4]);
        e.vcall(new_list, 0, &args![1u32]);
    }
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004f6320,
            tes_region_list_tes_region_list(Ptr<TESRegionList>, u8) -> Ptr<TESRegionList>
        ),
        entry!(0x004f_6480, tes_region_list_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x004f_64b0, fn_004f64b0(Ptr<TESRegionList>)),
        entry!(0x004f_6600, fn_004f6600(Ptr<TESRegionList>, u32) -> bool),
        entry!(0x004f_6640, fn_004f6640(Ptr<TESRegionList>)),
        entry!(0x004f_66c0, fn_004f66c0(Ptr<TESRegionList>, u32) -> u32),
        entry!(0x004f_6730, fn_004f6730(Ptr<TESRegionList>, u32) -> bool),
        entry!(
            0x004f_6800,
            tes_region_list_get_derived_data(Ptr<TESRegionList>, u32, Ptr, u32) -> u32
        ),
        entry!(
            0x004f_6850,
            fn_004f6850(Ptr<TESRegionList>, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(POINTER_PAIRS, 0x80);
        e.map(OWNING_LISTS, 4);
        e.register(LIST_CONSTRUCT, |_, _| Ret::default());
        e
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn a_list_that_does_not_own_its_memory_only_stores_its_fields() {
        let mut e = engine();
        e.mem.set_u32(POINTER_PAIRS, 0x0800_0000);
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xcd; 0x10]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f6320, &args![this, 0u32]).u32(), this);
        assert_eq!(calls_to(&e, LIST_CONSTRUCT), vec![vec![this + 4]]);
        assert_eq!(e.mem.u32(this), VTABLE);
        assert_eq!(e.mem.u8(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 4), 0xcdcd_cdcd);
        assert_eq!(e.mem.u32(this + 8), 0xcdcd_cdcd);
        assert_eq!(e.mem.bytes(this + 0xd, 3), [0xcd; 3]);
        assert_eq!(e.global::<u32>(OWNING_LISTS), 0);
        assert_eq!(e.mem.u32(POINTER_PAIRS), 0x0800_0000);
    }

    #[test]
    fn the_first_owning_list_deletes_the_tabled_objects() {
        let mut e = engine();
        // Two objects share one vtable whose slot 0 is a deleting destructor.
        e.put_vtable(0x0700_0000, &[0x0050_0000]);
        for object in [0x0800_0000, 0x0800_0100, 0x0800_0200] {
            e.map(object, 8);
            e.mem.set_u32(object, 0x0700_0000);
        }
        e.mem.set_u32(POINTER_PAIRS, 0x0800_0000);
        e.mem.set_u32(POINTER_PAIRS + 4 + 8 * 3, 0x0800_0100);
        e.mem.set_u32(POINTER_PAIRS + 8 * 8 + 4, 0x0800_0200);
        e.register(0x0050_0000, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f6320, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        assert_eq!(
            calls_to(&e, 0x0050_0000),
            vec![
                vec![0x0800_0000, 1],
                vec![0x0800_0100, 1],
                vec![0x0800_0200, 1],
            ]
        );
        for pair in 0..18 {
            assert_eq!(e.mem.u32(POINTER_PAIRS + pair * 4), 0, "word {pair}");
        }
        assert_eq!(e.global::<u32>(OWNING_LISTS), 1);
    }

    #[test]
    fn a_later_owning_list_only_counts_itself() {
        let mut e = engine();
        e.set_global(OWNING_LISTS, 2u32);
        e.mem.set_u32(POINTER_PAIRS, 0x0800_0000);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x004f6320, &args![this, 0x101u32]);
        assert_eq!(e.mem.u8(this + 0xc), 1);
        assert_eq!(e.global::<u32>(OWNING_LISTS), 3);
        assert_eq!(e.mem.u32(POINTER_PAIRS), 0x0800_0000);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }
    #[test]
    fn at_004f6480_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x004f_64b0, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x004f_6480, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x004f_6480)
                .collect();
            let mut expected = vec![(0x004f_64b0, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod region_list_tests {
    use super::*;

    /// One shared deleter for every vtable slot 0 (records `this`).
    const DELETER: u32 = 0x0051_0000;
    /// Region vtable (slot +0x10 deletes), data vtable (slot 0 deletes,
    /// +0x14 copies, +0x1c merges).
    const REGION_VTABLE: u32 = 0x0710_0000;
    const DATA_VTABLE: u32 = 0x0711_0000;
    const REGION_DELETE: u32 = 0x0052_0000;
    const DATA_COPY: u32 = 0x0053_0000;
    const DATA_MERGE: u32 = 0x0054_0000;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// A list base (`{item, next}` at the base, more nodes linked by
    /// `next`), walked and changed by doubles of the game's helpers.
    fn list_doubles(e: &mut Engine) {
        e.map(POINTER_PAIRS, 0x80);
        e.map(OWNING_LISTS, 4);
        e.register(NODE_ITEM, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(LIST_CLEAR_00470470, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(LIST_CONTAINS_005F65D0, |e, a| {
            let wanted = e.mem.u32(a[1]);
            let mut node = a[0];
            while node != 0 {
                if e.mem.u32(node) == wanted {
                    return 1u32.into_ret();
                }
                node = e.mem.u32(node + 4);
            }
            0u32.into_ret()
        });
        e.register(LIST_ADD_005AE3D0, |e, a| {
            let item = e.mem.u32(a[1]);
            if e.mem.u32(a[0]) == 0 {
                e.mem.set_u32(a[0], item);
                return Ret::default();
            }
            let mut tail = a[0];
            while e.mem.u32(tail + 4) != 0 {
                tail = e.mem.u32(tail + 4);
            }
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, item);
            e.mem.set_u32(tail + 4, node);
            Ret::default()
        });
        e.register(LIST_COUNT_005AE380, |e, a| {
            let mut count = 0u32;
            let mut node = a[0];
            while node != 0 && e.mem.u32(node) != 0 {
                count += 1;
                node = e.mem.u32(node + 4);
            }
            count.into_ret()
        });
        e.register(LIST_REMOVE_HEAD_0063F7B0, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            }
            Ret::default()
        });
        e.register(LIST_DESTRUCT_0046FFB0, |_, _| Ret::default());
        e.register(DELETER, |_, _| Ret::default());
        e.register(REGION_DELETE, |_, _| Ret::default());
        e.put_vtable(VTABLE, &[DELETER]);
        e.put_vtable(REGION_VTABLE, &[0, 0, 0, 0, REGION_DELETE]);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
    }

    /// A region list holding `items` (not owning unless `owns`).
    fn region_list(e: &mut Engine, items: &[u32], owns: bool) -> u32 {
        let list = e.mem.alloc(0x10);
        e.mem.set_u32(list, VTABLE);
        e.mem.set_u8(list + 0xc, u8::from(owns));
        let mut next = 0;
        for (i, item) in items.iter().enumerate().rev() {
            if i == 0 {
                e.mem.set_u32(list + 4, *item);
                e.mem.set_u32(list + 8, next);
            } else {
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, *item);
                e.mem.set_u32(node + 4, next);
                next = node;
            }
        }
        list
    }

    fn items(e: &Engine, list: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut node = list + 4;
        while node != 0 && e.mem.u32(node) != 0 {
            out.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        out
    }

    #[test]
    fn fn_004f6600_adds_a_region_once_and_refuses_null() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let list = region_list(&mut e, &[], false);
        assert_eq!(e.call(0x004f_6600, &args![list, 0u32]).u32() & 0xff, 0);
        assert_eq!(e.call(0x004f_6600, &args![list, 0x0a00u32]).u32() & 0xff, 1);
        assert_eq!(e.call(0x004f_6600, &args![list, 0x0b00u32]).u32() & 0xff, 1);
        assert_eq!(e.call(0x004f_6600, &args![list, 0x0a00u32]).u32() & 0xff, 1);
        assert_eq!(items(&e, list), vec![0x0a00, 0x0b00]);
    }

    #[test]
    fn fn_004f6640_empties_the_list_deleting_owned_regions() {
        for owns in [false, true] {
            let mut e = Engine::new();
            list_doubles(&mut e);
            let mut regions = vec![];
            for _ in 0..3 {
                let region = e.mem.alloc(4);
                e.mem.set_u32(region, REGION_VTABLE);
                regions.push(region);
            }
            let list = region_list(&mut e, &regions, owns);
            e.call_log = Some(vec![]);
            e.call(0x004f_6640, &args![list]);
            assert!(items(&e, list).is_empty());
            assert_eq!(calls_to(&e, LIST_REMOVE_HEAD_0063F7B0).len(), 3);
            let deleted = calls_to(&e, REGION_DELETE);
            if owns {
                assert_eq!(
                    deleted,
                    regions.iter().map(|r| vec![*r, 1]).collect::<Vec<_>>()
                );
            } else {
                assert!(deleted.is_empty());
            }
        }
    }

    #[test]
    fn fn_004f64b0_uncounts_an_owning_list_and_the_last_discards_the_table() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let cached = e.mem.alloc(4);
        e.mem.set_u32(cached, VTABLE);
        e.mem.set_u32(POINTER_PAIRS + 8 * 2 + 4, cached);
        e.set_global(OWNING_LISTS, 2u32);
        let list = region_list(&mut e, &[], true);
        e.call_log = Some(vec![]);
        e.call(0x004f_64b0, &args![list]);
        assert_eq!(e.mem.u32(list), VTABLE);
        assert_eq!(e.global::<u32>(OWNING_LISTS), 1);
        assert_eq!(e.mem.u32(POINTER_PAIRS + 8 * 2 + 4), cached);
        assert_eq!(calls_to(&e, LIST_DESTRUCT_0046FFB0), vec![vec![list + 4]]);
        // The last owning list discards the cached objects.
        e.call_log = Some(vec![]);
        e.call(0x004f_64b0, &args![list]);
        assert_eq!(e.global::<u32>(OWNING_LISTS), 0);
        assert_eq!(e.mem.u32(POINTER_PAIRS + 8 * 2 + 4), 0);
        assert_eq!(calls_to(&e, DELETER), vec![vec![cached, 1]]);
        // A list that does not own its memory leaves the count alone.
        let other = region_list(&mut e, &[], false);
        e.call(0x004f_64b0, &args![other]);
        assert_eq!(e.global::<u32>(OWNING_LISTS), 0);
    }

    #[test]
    fn fn_004f66c0_finds_a_region_by_form_id() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        e.register(FORM_ID, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        let first = e.mem.alloc(0x10);
        e.mem.set_u32(first + 0xc, 0x100);
        let second = e.mem.alloc(0x10);
        e.mem.set_u32(second + 0xc, 0x200);
        let list = region_list(&mut e, &[first, second], false);
        assert_eq!(e.call(0x004f_66c0, &args![list, 0x200u32]).u32(), second);
        assert_eq!(e.call(0x004f_66c0, &args![list, 0x100u32]).u32(), first);
        assert_eq!(e.call(0x004f_66c0, &args![list, 0x300u32]).u32(), 0);
        // A null list gives null.
        assert_eq!(e.call(0x004f_66c0, &args![0u32, 0x100u32]).u32(), 0);
    }

    #[test]
    fn fn_004f6730_compares_lists_item_by_item() {
        let mut e = Engine::new();
        list_doubles(&mut e);
        let a = region_list(&mut e, &[1, 2, 3], false);
        let same = region_list(&mut e, &[1, 2, 3], false);
        let shorter = region_list(&mut e, &[1, 2], false);
        let different = region_list(&mut e, &[1, 5, 3], false);
        let check = |e: &mut Engine, x: u32, y: u32| e.call(0x004f_6730, &args![x, y]).u32() & 0xff;
        assert_eq!(check(&mut e, a, same), 1);
        assert_eq!(check(&mut e, a, shorter), 0);
        assert_eq!(check(&mut e, shorter, a), 0);
        assert_eq!(check(&mut e, a, different), 0);
        assert_eq!(check(&mut e, a, 0), 0);
        assert_eq!(check(&mut e, 0, a), 0);
    }

    /// Region: +0 vtable, +4 skip byte, +8 worldspace, +0xc its data (the
    /// data list `009611e0` gives is `region + 0xc`), +0x10 point lists.
    fn region(e: &mut Engine, worldspace: u32, data: u32, points: u32) -> u32 {
        let region = e.mem.alloc(0x14);
        e.mem.set_u32(region, REGION_VTABLE);
        e.mem.set_u32(region + 8, worldspace);
        e.mem.set_u32(region + 0xc, data);
        e.mem.set_u32(region + 0x10, points);
        region
    }

    /// Data: +0 vtable, +4 ignored, +8 overrides, +0xc priority; copies
    /// keep their source at +0x10.
    fn data(e: &mut Engine, overrides: bool, priority: u32) -> u32 {
        let data = e.mem.alloc(0x14);
        e.mem.set_u32(data, DATA_VTABLE);
        e.mem.set_u32(data + 8, u32::from(overrides));
        e.mem.set_u32(data + 0xc, priority);
        data
    }

    fn derive_doubles(e: &mut Engine) {
        list_doubles(e);
        e.put_vtable(
            DATA_VTABLE,
            &[DELETER, 0, 0, 0, 0, DATA_COPY, 0, DATA_MERGE],
        );
        e.register(REGION_SKIP_00440D80, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(REGION_WORLDSPACE_007AF430, |e, a| {
            e.mem.u32(a[0] + 8).into_ret()
        });
        e.register(REGION_DATA_LIST_009611E0, |_, a| (a[0] + 0xc).into_ret());
        e.register(REGION_DATA_LIST_FIND, |e, a| {
            assert_eq!(a[1], 3);
            e.mem.u32(a[0]).into_ret()
        });
        e.register(REGION_POINT_LISTS_00441110, |e, a| {
            e.mem.u32(a[0] + 0x10).into_ret()
        });
        // A point list contains the point when the list's item equals it.
        e.register(POINT_LIST_CONTAINS_004F8360, |_, a| {
            u32::from(a[0] == a[1]).into_ret()
        });
        e.register(DATA_IGNORED_004F6DE0, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(DATA_OVERRIDE_004F1540, |e, a| {
            e.mem.u32(a[0] + 8).into_ret()
        });
        e.register(DATA_PRIORITY_005BB4D0, |e, a| {
            (0xffff_ff00 | e.mem.u32(a[0] + 0xc)).into_ret()
        });
        e.register(DATA_COPY, |e, a| {
            let copy = e.mem.alloc(0x14);
            e.mem.set_u32(copy, DATA_VTABLE);
            e.mem.set_u32(copy + 0x10, a[0]);
            copy.into_ret()
        });
        e.register(DATA_MERGE, |_, _| Ret::default());
    }

    #[test]
    fn fn_004f6850_merges_the_data_of_the_matching_regions_and_caches_it() {
        let mut e = Engine::new();
        derive_doubles(&mut e);
        let data_a = data(&mut e, false, 0);
        let data_b = data(&mut e, false, 0);
        let ignored = data(&mut e, false, 0);
        e.mem.set_u32(ignored + 4, 1);
        let region_a = region(&mut e, 7, data_a, 0);
        let region_b = region(&mut e, 7, data_b, 0);
        let other_world = region(&mut e, 8, data_a, 0);
        let skipped = region(&mut e, 7, data_a, 0);
        e.mem.set_u32(skipped + 4, 1);
        let no_data = region(&mut e, 7, 0, 0);
        let ignoring = region(&mut e, 7, ignored, 0);
        let this = region_list(
            &mut e,
            &[region_a, other_world, skipped, no_data, ignoring, region_b],
            false,
        );
        // Old cached objects of the type are discarded.
        let old_data = data(&mut e, false, 0);
        let old_list = region_list(&mut e, &[region_a], false);
        e.mem.set_u32(POINTER_PAIRS + 24, old_data);
        e.mem.set_u32(POINTER_PAIRS + 28, old_list);
        e.call_log = Some(vec![]);
        let result = e.call(0x004f_6850, &args![this, 3u32, 0u32, 7u32]).u32();
        assert_ne!(result, 0);
        assert_eq!(e.mem.u32(result + 0x10), data_a);
        assert_eq!(calls_to(&e, DATA_MERGE), vec![vec![result, data_b, 2]]);
        let new_list = e.mem.u32(POINTER_PAIRS + 28);
        assert_eq!(e.mem.u32(POINTER_PAIRS + 24), result);
        assert_eq!(items(&e, new_list), vec![region_a, region_b]);
        assert_eq!(e.mem.u32(new_list), VTABLE);
        assert_eq!(e.mem.u8(new_list + 0xc), 0);
        assert_eq!(
            calls_to(&e, DELETER),
            vec![vec![old_data, 1], vec![old_list, 1]]
        );

        // The same regions again: the cached data comes back and the new
        // list is cleared and deleted.
        e.call_log = Some(vec![]);
        let again = e.call(0x004f_6850, &args![this, 3u32, 0u32, 7u32]).u32();
        assert_eq!(again, result);
        let deleted = calls_to(&e, DELETER);
        assert_eq!(deleted.len(), 1);
        assert_ne!(deleted[0][0], new_list);
        assert_eq!(e.mem.u32(POINTER_PAIRS + 28), new_list);
        assert!(calls_to(&e, DATA_COPY).is_empty());
    }

    #[test]
    fn fn_004f6850_takes_the_highest_priority_override_and_filters_by_point() {
        let mut e = Engine::new();
        derive_doubles(&mut e);
        let plain = data(&mut e, false, 0);
        let low = data(&mut e, true, 2);
        let high = data(&mut e, true, 5);
        let equal = data(&mut e, true, 5);
        // The point lists: one node whose item is the point 0x0900_0000.
        let points = e.mem.alloc(8);
        e.mem.set_u32(points, 0x0900_0000);
        let elsewhere = e.mem.alloc(8);
        e.mem.set_u32(elsewhere, 0x0901_0000);
        let r_plain = region(&mut e, 0, plain, points);
        let r_low = region(&mut e, 0, low, points);
        let r_high = region(&mut e, 0, high, points);
        let r_equal = region(&mut e, 0, equal, points);
        let r_far = region(&mut e, 0, high, elsewhere);
        let this = region_list(&mut e, &[r_plain, r_low, r_far, r_high, r_equal], false);
        e.call_log = Some(vec![]);
        let result = e
            .call(0x004f_6850, &args![this, 3u32, 0x0900_0000u32, 0u32])
            .u32();
        // The plain data's copy is dropped by the first override; the copy
        // made at the end is of the first highest priority data.
        let copies = calls_to(&e, DATA_COPY);
        assert_eq!(copies, vec![vec![plain], vec![high]]);
        assert_eq!(e.mem.u32(result + 0x10), high);
        assert!(calls_to(&e, DATA_MERGE).is_empty());
        let new_list = e.mem.u32(POINTER_PAIRS + 28);
        assert_eq!(items(&e, new_list), vec![r_plain, r_low, r_high, r_equal]);
    }

    #[test]
    fn fn_004f6850_without_data_deletes_its_list_and_gives_null() {
        let mut e = Engine::new();
        derive_doubles(&mut e);
        let this = region_list(&mut e, &[], false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_6850, &args![this, 3u32, 0u32, 0u32]).u32(), 0);
        let deleted = calls_to(&e, DELETER);
        assert_eq!(deleted.len(), 1);
        assert_eq!(
            calls_to(&e, LIST_CLEAR_00470470),
            vec![vec![deleted[0][0] + 4]]
        );
        assert_eq!(e.mem.u32(POINTER_PAIRS + 28), 0);
    }

    #[test]
    fn get_derived_data_builds_the_point_then_derives() {
        let mut e = Engine::new();
        derive_doubles(&mut e);
        e.register(REGION_POINT_CONSTRUCT, |_, _| Ret::default());
        e.register(REGION_POINT_SET_X_0052CE20, |_, _| Ret::default());
        e.register(REGION_POINT_SET_Y_004F5D90, |_, _| Ret::default());
        let this = region_list(&mut e, &[], false);
        let point = e.mem.alloc(8);
        e.mem.set_u32(point, 1.5f32.to_bits());
        e.mem.set_u32(point + 4, 0x7f80_0001);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004f_6800, &args![this, 3u32, point, 0u32]).u32(),
            0
        );
        let local = calls_to(&e, REGION_POINT_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&e, REGION_POINT_SET_X_0052CE20),
            vec![vec![local, 1.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&e, REGION_POINT_SET_Y_004F5D90),
            vec![vec![local, 0x7fc0_0001]]
        );
        // The derivation ran (its new list was made and deleted).
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x10]]);
    }
}
