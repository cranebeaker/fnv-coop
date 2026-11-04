//! `fallout shared/run_004a8b00` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Seven functions that sit between the `bgssceneinfo.cpp` code and the
//! `BGSTextureUseMap` members (`run_004aa340`) and have no names in the Xbox
//! PDB map: the scene-data report builder `004a8bb0`, its helpers, and a
//! walk callback. What identifies them: `004a8bb0` is called by the cell
//! test functions (`TestAllCells`, `TestCode`) and builds a
//! `BGSSceneInfo` (0x48 bytes, `pAssociatedForm` at +0x44) for the interior
//! cell or for each exterior cell around the player, collects its scene
//! data and texture use (`BGSTextureUseMap`, 0x14 bytes, built by
//! `004a8f50`), and adds up the counters; `004a8b40` is the callback that
//! `BGSSceneInfo::CollectSceneDataRecursive` (`004a82d0`) receives as a
//! function pointer. The names of these functions stay `fn_<address>`.
//!
//! Table at `01188694`: sixteen entries of five words (stride 0x14), one per
//! scene data counter index; the entry's meaning is not named in the Xbox
//! PDB, so the words are called by their offsets: +0 and +4 are passed to
//! a virtual function (slot 0x14) of the object `0044f560` returns, +8 is a
//! pointer to a record with a `float` factor at +4 (null when the counter has
//! no factor) and +0xC is a byte flag.
//!
//! x87 note: the floats are added and multiplied in `f64` and rounded to
//! `f32` at each store, as ENGINE_CRATE.md prescribes; the 32-bit counters
//! are loaded with `FILD` of a zero-extended 64-bit value.

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (cdecl, size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `BGSSceneInfo` constructor (`ECX` = block of 0x48 bytes): returns `this`.
const SCENE_INFO_CONSTRUCT: u32 = 0x004a_7c40;
/// Stores its argument in `BGSSceneInfo::pAssociatedForm` (+0x44); the map
/// names this body `LowProcess::SetGenericLocation`, a folded name.
const SCENE_INFO_SET_ASSOCIATED_FORM: u32 = 0x008d_8040;
/// `BGSSceneInfo::CollectSceneDataFromNode` (Xbox PDB): `this`, node,
/// `with_addons`, `descend_anyway`.
const COLLECT_SCENE_DATA_FROM_NODE: u32 = 0x004a_8230;
/// Counter `index` of the scene data (`this`, index); the totals use 9 for
/// the total size.
const SCENE_INFO_COUNTER: u32 = 0x004a_7cf0;
/// Completes the scene info for a cell (`this`, cell); the Xbox PDB map has
/// no name for it.
const SCENE_INFO_FINISH: u32 = 0x004a_7d20;
/// `TESObjectCELL::Get3D` (Xbox PDB): the cell's 3D node (`ECX` = cell, no
/// stack arguments).
const CELL_GET_3D: u32 = 0x0054_5cb0;
/// Collects the textures used below a node (`this` = the texture use map,
/// node, default user, flag).
const TEXTURE_USE_COLLECT: u32 = 0x004a_a3a0;
/// The total of the texture sizes of the texture use map in `ECX`.
const TEXTURE_USE_TOTAL_SIZE: u32 = 0x004a_a890;
/// Destructor body of the texture use map in `ECX`.
const TEXTURE_USE_DESTRUCT: u32 = 0x004a_a340;
/// `NiTMap` constructor body (`ECX` = map, the bucket count).
const MAP_CONSTRUCT: u32 = 0x004a_95d0;
/// Removes every entry of the map in `ECX`.
const MAP_REMOVE_ALL: u32 = 0x0043_8af0;
/// Lookup in the hash table in `ECX`: key, byte out-parameter; true when
/// the key is there.
const TABLE_LOOKUP: u32 = 0x0057_c850;
/// Called on the same table with the key and 1 once a walked object has been
/// counted (its body is `bgssaveloadglobaldata.cpp` code in the map; the
/// role is not identified).
const TABLE_MARK: u32 = 0x0084_d310;
/// Constructor of the two-word list head (`ECX` = list).
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// Appends the value whose address is passed to the list in `ECX`.
const LIST_ADD: u32 = 0x005a_e3d0;
/// Whether the list in `ECX` is empty.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// Returns the item slot of the list node in `ECX` (the node itself).
const LIST_ITEM_SLOT: u32 = 0x0068_15c0;
/// Removes the first node of the list in `ECX`.
const LIST_POP_FRONT: u32 = 0x0063_f7b0;
/// The list destructor (`ECX` = list).
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
/// Returns the pointer at +0x34 of the object in `ECX` (the map names it
/// `ActorMover::GetPreferredMoveMode`, a folded name; on the object in
/// `011dea10` the Xbox PDB's `TES` class has `pInteriorCell` at +0x34).
const INTERIOR_CELL_GETTER: u32 = 0x005f_36f0;
/// Returns the address of the word at +4 of the object in `ECX`, or of a
/// static zero when there is none (the setting value of the grid size).
const WORD_AT_FOUR_ADDRESS: u32 = 0x0043_d4d0;
/// Returns the address of the cell pointer of the grid cell (x, y) of the
/// grid at +8 of the object in `ECX`.
const GRID_CELL_SLOT: u32 = 0x0045_7050;
/// Returns the address of the float at +4 of the object in `ECX`, or of a
/// static zero when it is null.
const FLOAT_AT_FOUR_ADDRESS: u32 = 0x0040_3e20;
/// Returns a singleton (the object whose virtual slot `0x14` the table's first
/// two words go to).
const TABLE_WORDS_RECEIVER: u32 = 0x0044_f560;
/// Returns the pointer that `ECX + 0x1c` holds (a pointer getter).
const POINTER_GETTER: u32 = 0x0068_38b0;

/// The global pointer of the object whose +0x34 is the interior cell and
/// whose +8 is the grid of exterior cells (`011dea10`).
const CELL_OWNER: u32 = 0x011d_ea10;
/// The object whose +4 word is the grid size (`011c63cc`); the report covers
/// the cells within half of it, at most two cells around the centre.
const GRID_SETTING: u32 = 0x011c_63cc;
/// The global map whose entries the walk callback counts (`011c5c40`).
const WALKED_TABLE: u32 = 0x011c_5c40;
/// Start of the 16-entry table of five words, and its stride.
const COUNTER_TABLE: u32 = 0x0118_8694;
const COUNTER_ENTRY_SIZE: u32 = 0x14;
const COUNTER_ENTRY_COUNT: i32 = 16;
/// Offsets inside a table entry.
const ENTRY_FIRST_WORD: u32 = 0;
const ENTRY_SECOND_WORD: u32 = 4;
const ENTRY_FACTOR_RECORD: u32 = 8;
const ENTRY_SKIP_FLAG: u32 = 0xc;

/// The counter index whose value is added to the second total.
const TOTAL_COUNTER: u32 = 9;
/// The bucket count of the texture use map's `NiTMap`.
const TEXTURE_MAP_BUCKETS: u32 = 0x25;
/// Size of a `BGSSceneInfo`, of a `BGSTextureUseMap` and of a list head.
const SCENE_INFO_SIZE: u32 = 0x48;
const TEXTURE_USE_MAP_SIZE: u32 = 0x14;
const LIST_HEAD_SIZE: u32 = 8;
/// Virtual slot that the table's first two words are passed to.
const SLOT_TABLE_WORD_RECEIVER: u32 = 0x14;
/// Virtual slot of the object `004a8b00` looks at.
const SLOT_QUERY: u32 = 0x38;

// Translated from 004a8b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the pointer `arg + 0x1c` holds (through the getter `006838b0`) and,
/// when it is not null, returns the result of its virtual function at vtable
/// offset `0x38` (no arguments); 0 otherwise. The only argument is a stack
/// word, the function is `cdecl`.
pub fn fn_004a8b00(e: &mut Engine, arg: Ptr) -> u32 {
    let object = e.call(POINTER_GETTER, &args![arg]).u32();
    if object == 0 {
        0
    } else {
        e.vcall(object, SLOT_QUERY, &args![]).u32()
    }
}

// Translated from 004a8b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The callback `CollectSceneDataRecursive` calls for an object `key`
/// (`cdecl`): unless the global table already holds `key` with its byte set,
/// adds `delta` to the word that `*counter_slot` points to and marks the key
/// (`TABLE_MARK` with 1). (The compiler left a block of dead flag tests,
/// both always zero, between the lookup and the addition; it is not
/// translated.)
pub fn fn_004a8b40(e: &mut Engine, key: u32, delta: u32, counter_slot: Ptr) {
    let (found, flag) = e.with_stack(1, |e, flag| {
        let found = e.call(TABLE_LOOKUP, &args![WALKED_TABLE, key, flag]).bool();
        (found, e.mem.u8(flag.addr()))
    });
    if found && flag != 0 {
        return;
    }
    let counter = e.mem.u32(counter_slot.addr());
    let total = e.mem.u32(counter).wrapping_add(delta);
    e.mem.set_u32(counter, total);
    e.call(TABLE_MARK, &args![WALKED_TABLE, key, 1]);
}

// Translated from 004a8f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The signed minimum of two integers (`cdecl`).
pub fn fn_004a8f20(_e: &mut Engine, first: i32, second: i32) -> i32 {
    first.min(second)
}

// Translated from 004a8f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the 0x14-byte `BGSTextureUseMap`-sized object: builds
/// the `NiTMap` at +4 with 0x25 buckets. Returns `this`.
pub fn fn_004a8f50(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MAP_CONSTRUCT, &args![this.addr() + 4, TEXTURE_MAP_BUCKETS]);
    this
}

// Translated from 004a8f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scene info's counter `index` times the table entry's factor, as a
/// `float`; 0.0 for an index outside 0 to 15 or for an entry without a
/// factor record. `this` is the `BGSSceneInfo`.
pub fn fn_004a8f70(e: &mut Engine, this: Ptr, index: i32) -> f32 {
    if !(0..COUNTER_ENTRY_COUNT).contains(&index) {
        return 0.0;
    }
    let entry = COUNTER_TABLE + index as u32 * COUNTER_ENTRY_SIZE;
    if e.mem.u32(entry + ENTRY_FACTOR_RECORD) == 0 {
        return 0.0;
    }
    let count = e.call(SCENE_INFO_COUNTER, &args![this, index]).u32();
    let record = e.mem.u32(entry + ENTRY_FACTOR_RECORD);
    let factor = e.call(FLOAT_AT_FOUR_ADDRESS, &args![record]).u32();
    let factor = e.mem.f32(factor);
    (factor as f64 * count as f64) as f32
}

// Translated from 004a8fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For each of the sixteen table entries calls the virtual function at slot
/// `0x14` of the object `0044f560` returns, first with the entry's first
/// word and then (the object is fetched again) with its second word.
pub fn fn_004a8fe0(e: &mut Engine) {
    for index in 0..COUNTER_ENTRY_COUNT as u32 {
        let entry = COUNTER_TABLE + index * COUNTER_ENTRY_SIZE;
        let receiver = e.call(TABLE_WORDS_RECEIVER, &args![]).u32();
        let word = e.mem.u32(entry + ENTRY_FIRST_WORD);
        e.vcall(receiver, SLOT_TABLE_WORD_RECEIVER, &args![word]);
        let receiver = e.call(TABLE_WORDS_RECEIVER, &args![]).u32();
        let word = e.mem.u32(entry + ENTRY_SECOND_WORD);
        e.vcall(receiver, SLOT_TABLE_WORD_RECEIVER, &args![word]);
    }
}

/// Builds the scene info of one cell and queues it: a new `BGSSceneInfo`
/// (null when the allocation fails), its associated form, the scene data
/// collected from the cell's 3D node, the finish step and the texture use of
/// the node, and the pointer appended to `list`. The interior path finishes
/// the scene info before the texture use, the exterior path after
/// (`finish_first`).
fn queue_cell(
    e: &mut Engine,
    texture_map: Ptr,
    list: Ptr,
    cell: u32,
    with_addons: u8,
    finish_first: bool,
) {
    let block = e.call(OPERATOR_NEW, &args![SCENE_INFO_SIZE]).u32();
    let scene_info = if block == 0 {
        0
    } else {
        e.call(SCENE_INFO_CONSTRUCT, &args![block]).u32()
    };
    e.call(SCENE_INFO_SET_ASSOCIATED_FORM, &args![scene_info, cell]);
    let node = e.call(CELL_GET_3D, &args![cell]).u32();
    e.call(
        COLLECT_SCENE_DATA_FROM_NODE,
        &args![scene_info, node, with_addons as u32, 0],
    );
    if finish_first {
        e.call(SCENE_INFO_FINISH, &args![scene_info, cell]);
    }
    let node = e.call(CELL_GET_3D, &args![cell]).u32();
    e.call(TEXTURE_USE_COLLECT, &args![texture_map, node, cell, 0]);
    if !finish_first {
        e.call(SCENE_INFO_FINISH, &args![scene_info, cell]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), scene_info);
        e.call(LIST_ADD, &args![list, slot]);
    });
}

// Translated from 004a8bb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004a8bb0 - C++ exception-unwinding frame (FS:[0] handler chain) not translated
/// Builds the scene-data report of the cells around the player (`cdecl`).
/// `total_size` and `total_count` are `float` totals, set to 0.0 first:
/// `total_size` receives, for every scene info, the sum over the table
/// entries whose skip byte (+0xC) is zero of [`fn_004a8f70`], and
/// `total_count` the counter 9 of every scene info (plus, when
/// `add_textures` is not zero, the total texture size).
///
/// The scene infos are made for the interior cell when `005f36f0` of the
/// object in `011dea10` is not null, otherwise for every loaded exterior
/// cell within two cells (at most half the grid size) of the centre,
/// scanning the grid with the outer index first. Each scene info is queued in
/// a list, and the list is then drained: each scene info is added to the
/// totals and deleted through its virtual destructor with 1. At the end the
/// global table is emptied, the list and the texture use map are destroyed.
pub fn fn_004a8bb0(
    e: &mut Engine,
    total_size: Ptr,
    total_count: Ptr,
    add_textures: u8,
    with_addons: u8,
) {
    e.mem.set_f32(total_size.addr(), 0.0);
    e.mem.set_f32(total_count.addr(), 0.0);
    fn_004a8fe0(e);
    let texture_map = Ptr::new(e.mem.alloc(TEXTURE_USE_MAP_SIZE));
    fn_004a8f50(e, texture_map);
    e.with_stack(LIST_HEAD_SIZE, |e, list| {
        e.call(LIST_CONSTRUCT, &args![list]);
        let owner = e.global::<u32>(CELL_OWNER);
        if e.call(INTERIOR_CELL_GETTER, &args![owner]).u32() != 0 {
            let owner = e.global::<u32>(CELL_OWNER);
            let cell = e.call(INTERIOR_CELL_GETTER, &args![owner]).u32();
            queue_cell(e, texture_map, list, cell, with_addons, true);
        } else {
            let grid = e.call(WORD_AT_FOUR_ADDRESS, &args![GRID_SETTING]).u32();
            let half = e.mem.u32(grid) >> 1;
            let reach = fn_004a8f20(e, half as i32, 2);
            let first = half.wrapping_sub(reach as u32);
            let reach = fn_004a8f20(e, half as i32, 2);
            let last = (reach as u32).wrapping_add(half);
            let mut outer = first;
            while outer <= last {
                let mut inner = first;
                while inner <= last {
                    let owner = e.global::<u32>(CELL_OWNER);
                    let slot = e.call(GRID_CELL_SLOT, &args![owner, outer, inner]).u32();
                    let cell = e.mem.u32(slot);
                    if cell != 0 {
                        queue_cell(e, texture_map, list, cell, with_addons, false);
                    }
                    inner = inner.wrapping_add(1);
                }
                outer = outer.wrapping_add(1);
            }
        }
        while !e.call(LIST_IS_EMPTY, &args![list]).bool() {
            let slot = e.call(LIST_ITEM_SLOT, &args![list]).u32();
            let scene_info = e.mem.u32(slot);
            e.call(LIST_POP_FRONT, &args![list]);
            let count = e
                .call(SCENE_INFO_COUNTER, &args![scene_info, TOTAL_COUNTER])
                .u32();
            let sum = count as f64 + e.mem.f32(total_count.addr()) as f64;
            e.mem.set_f32(total_count.addr(), sum as f32);
            for index in 0..COUNTER_ENTRY_COUNT {
                let skip = e
                    .mem
                    .u8(COUNTER_TABLE + index as u32 * COUNTER_ENTRY_SIZE + ENTRY_SKIP_FLAG);
                if skip == 0 {
                    let part = fn_004a8f70(e, Ptr::new(scene_info), index);
                    let sum = part as f64 + e.mem.f32(total_size.addr()) as f64;
                    e.mem.set_f32(total_size.addr(), sum as f32);
                }
            }
            if scene_info != 0 {
                e.vcall(scene_info, 0, &args![1]);
            }
        }
        if add_textures != 0 {
            let size = e.call(TEXTURE_USE_TOTAL_SIZE, &args![texture_map]).u32();
            let sum = size as f64 + e.mem.f32(total_count.addr()) as f64;
            e.mem.set_f32(total_count.addr(), sum as f32);
        }
        e.call(MAP_REMOVE_ALL, &args![WALKED_TABLE]);
        e.call(LIST_DESTRUCT, &args![list]);
    });
    e.call(TEXTURE_USE_DESTRUCT, &args![texture_map]);
    e.mem.free(texture_map.addr());
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x004a8b00, fn_004a8b00(Ptr) -> u32),
        entry!(0x004a8b40, fn_004a8b40(u32, u32, Ptr)),
        entry!(0x004a8bb0, fn_004a8bb0(Ptr, Ptr, u8, u8)),
        entry!(0x004a8f20, fn_004a8f20(i32, i32) -> i32),
        entry!(0x004a8f50, fn_004a8f50(Ptr) -> Ptr),
        entry!(0x004a8f70, fn_004a8f70(Ptr, i32) -> f32),
        entry!(0x004a8fe0, fn_004a8fe0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A vtable address used for fake objects, and the addresses of the
    /// functions its slots hold.
    const FAKE_VTABLE: u32 = 0x0300_0000;
    const FAKE_DESTRUCTOR: u32 = 0x0abc_0001;
    const FAKE_RECEIVER_VTABLE: u32 = 0x0300_1000;
    const FAKE_RECEIVER_METHOD: u32 = 0x0abc_0002;
    const FAKE_QUERY: u32 = 0x0abc_0003;

    /// Maps the pages the code reads globals and the table from.
    fn engine_with_memory() -> Engine {
        let mut e = Engine::new();
        e.map(0x0118_8000, 0x1000);
        e.map(0x011c_5000, 0x2000);
        e.map(0x011d_e000, 0x1000);
        e.put_vtable(FAKE_VTABLE, &[FAKE_DESTRUCTOR]);
        e.put_vtable(
            FAKE_RECEIVER_VTABLE,
            &[0, 0, 0, 0, 0, FAKE_RECEIVER_METHOD, 0, 0, 0],
        );
        e
    }

    #[test]
    fn minimum_is_signed() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x004a_8f20, &args![3, 7]).i32(), 3);
        assert_eq!(e.call(0x004a_8f20, &args![7, 3]).i32(), 3);
        assert_eq!(e.call(0x004a_8f20, &args![-1, 2]).i32(), -1);
        assert_eq!(e.call(0x004a_8f20, &args![4, 4]).i32(), 4);
        assert_eq!(e.call(0x004a_8f20, &args![i32::MIN, 0]).i32(), i32::MIN);
    }

    #[test]
    fn query_calls_slot_0x38_of_the_object_the_getter_returns() {
        let mut e = engine_with_memory();
        e.put_vtable(0x0300_2000, &[0; 0x0f]);
        e.mem.set_u32(0x0300_2000 + SLOT_QUERY, FAKE_QUERY);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0300_2000);
        e.register_double(POINTER_GETTER, move |_, a| {
            assert_eq!(a[0], 0x1234);
            Ret {
                eax: object,
                ..Ret::default()
            }
        });
        e.register(FAKE_QUERY, |_, a| Ret {
            eax: a[0] + 5,
            ..Ret::default()
        });
        assert_eq!(e.call(0x004a_8b00, &args![0x1234u32]).u32(), object + 5);
        e.register(POINTER_GETTER, |_, _| Ret::default());
        assert_eq!(e.call(0x004a_8b00, &args![0x1234u32]).u32(), 0);
    }

    #[test]
    fn walk_callback_counts_unless_the_key_is_already_marked() {
        let mut e = engine_with_memory();
        let counter = e.mem.alloc(4);
        e.mem.set_u32(counter, 0xffff_fff0);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, counter);
        // Found with its byte set: nothing happens.
        e.register(TABLE_LOOKUP, |e, a| {
            assert_eq!(a[0], WALKED_TABLE);
            e.mem.set_u8(a[2], 1);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register(TABLE_MARK, |_, _| panic!("already marked"));
        e.call(0x004a_8b40, &args![0x77u32, 5u32, slot]);
        assert_eq!(e.mem.u32(counter), 0xffff_fff0);

        // Found, byte clear: counted (the sum wraps), the key is marked.
        e.register(TABLE_LOOKUP, |e, a| {
            e.mem.set_u8(a[2], 0);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register(TABLE_MARK, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004a_8b40, &args![0x77u32, 0x20u32, slot]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(counter), 0x10);
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == TABLE_MARK && a[..] == [WALKED_TABLE, 0x77, 1]));

        // Not found (the byte is whatever the lookup left): counted.
        e.register(TABLE_LOOKUP, |e, a| {
            e.mem.set_u8(a[2], 1);
            Ret::default()
        });
        e.call(0x004a_8b40, &args![0x78u32, 1u32, slot]);
        assert_eq!(e.mem.u32(counter), 0x11);
    }

    #[test]
    fn texture_map_constructor_builds_the_map_at_four() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x14);
        e.register(MAP_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004a_8f50, &args![block]).u32(), block);
        let log = e.call_log.take().unwrap();
        assert!(log
            .iter()
            .any(|(addr, a)| *addr == MAP_CONSTRUCT && a[..] == [block + 4, 0x25]));
    }

    /// Fills the table: entry `i` has a factor record (a 0.5 factor at +4)
    /// except entry 7 (none), and its skip byte is set for 3 and 5.
    fn fill_table(e: &mut Engine) {
        for index in 0..16u32 {
            let entry = COUNTER_TABLE + index * COUNTER_ENTRY_SIZE;
            e.mem.set_u32(entry, 0x1000 + index);
            e.mem.set_u32(entry + 4, 0x2000 + index);
            let record = if index == 7 {
                0
            } else {
                let record = e.mem.alloc(8);
                e.mem.set_f32(record + 4, 0.5);
                record
            };
            e.mem.set_u32(entry + 8, record);
            e.mem.set_u8(entry + 0xc, (index == 3 || index == 5) as u8);
        }
    }

    #[test]
    fn counter_times_factor_with_the_range_and_record_checks() {
        let mut e = engine_with_memory();
        fill_table(&mut e);
        e.register(FLOAT_AT_FOUR_ADDRESS, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.register(SCENE_INFO_COUNTER, |_, a| Ret {
            eax: a[1] * 10 + 3,
            ..Ret::default()
        });
        let scene_info = Ptr::<()>::new(0x5000);
        // Counter 2 is 23: 23 * 0.5.
        assert_eq!(e.call(0x004a_8f70, &args![scene_info, 2]).f32(), 11.5);
        // Out of range, or no record: 0.0 without asking for the counter.
        e.register(SCENE_INFO_COUNTER, |_, _| panic!("no counter needed"));
        assert_eq!(e.call(0x004a_8f70, &args![scene_info, -1]).f32(), 0.0);
        assert_eq!(e.call(0x004a_8f70, &args![scene_info, 16]).f32(), 0.0);
        assert_eq!(e.call(0x004a_8f70, &args![scene_info, 7]).f32(), 0.0);
        // A large counter is zero-extended and the product rounded to f32.
        e.mem.set_f32(
            e.mem.u32(COUNTER_TABLE + 12 * COUNTER_ENTRY_SIZE + 8) + 4,
            1.0,
        );
        e.register(SCENE_INFO_COUNTER, |_, _| Ret {
            eax: u32::MAX,
            ..Ret::default()
        });
        assert_eq!(
            e.call(0x004a_8f70, &args![scene_info, 12]).f32(),
            u32::MAX as f64 as f32
        );
    }

    #[test]
    fn table_walk_calls_the_receiver_twice_per_entry() {
        let mut e = engine_with_memory();
        fill_table(&mut e);
        let receiver = e.mem.alloc(8);
        e.mem.set_u32(receiver, FAKE_RECEIVER_VTABLE);
        e.register_double(TABLE_WORDS_RECEIVER, move |_, _| Ret {
            eax: receiver,
            ..Ret::default()
        });
        e.register(FAKE_RECEIVER_METHOD, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x004a_8fe0, &args![]);
        let log = e.call_log.take().unwrap();
        let words: Vec<u32> = log
            .iter()
            .filter(|(addr, _)| *addr == FAKE_RECEIVER_METHOD)
            .map(|(_, a)| a[1])
            .collect();
        assert_eq!(words.len(), 32);
        assert_eq!(words[..4], [0x1000, 0x2000, 0x1001, 0x2001]);
        assert_eq!(words[30..], [0x100f, 0x200f]);
        // `this` of each call is the receiver; the receiver is fetched per call.
        assert!(log
            .iter()
            .filter(|(addr, _)| *addr == FAKE_RECEIVER_METHOD)
            .all(|(_, a)| a[0] == receiver));
        assert_eq!(
            log.iter()
                .filter(|(addr, _)| *addr == TABLE_WORDS_RECEIVER)
                .count(),
            32
        );
    }

    /// The grid coordinates the double of the cell lookup was asked for.
    type AskedCells = Rc<RefCell<Vec<(u32, u32)>>>;

    /// Doubles for everything `004a8bb0` calls outside this file. The list
    /// is a queue kept on the Rust side; the cells of the grid are
    /// `cells`, `(x, y, cell)`.
    fn report_engine(cells: Vec<(u32, u32, u32)>) -> (Engine, AskedCells) {
        let mut e = engine_with_memory();
        fill_table(&mut e);
        let receiver = e.mem.alloc(8);
        e.mem.set_u32(receiver, FAKE_RECEIVER_VTABLE);
        e.register_double(TABLE_WORDS_RECEIVER, move |_, _| Ret {
            eax: receiver,
            ..Ret::default()
        });
        e.register(FAKE_RECEIVER_METHOD, |_, _| Ret::default());
        e.register(FAKE_DESTRUCTOR, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(SCENE_INFO_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], FAKE_VTABLE);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(SCENE_INFO_SET_ASSOCIATED_FORM, |_, _| Ret::default());
        e.register(CELL_GET_3D, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        e.register(COLLECT_SCENE_DATA_FROM_NODE, |_, _| Ret::default());
        e.register(SCENE_INFO_FINISH, |_, _| Ret::default());
        e.register(TEXTURE_USE_COLLECT, |_, _| Ret::default());
        e.register(TEXTURE_USE_TOTAL_SIZE, |_, _| Ret {
            eax: 77,
            ..Ret::default()
        });
        e.register(TEXTURE_USE_DESTRUCT, |_, _| Ret::default());
        e.register(MAP_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(MAP_REMOVE_ALL, |_, _| Ret::default());
        e.register(LIST_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        // Counter k of any scene info is k + 1; the factor getter is the
        // plain one.
        e.register(SCENE_INFO_COUNTER, |_, a| Ret {
            eax: a[1] + 1,
            ..Ret::default()
        });
        e.register(FLOAT_AT_FOUR_ADDRESS, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        // The list: a queue of scene info pointers.
        let queue = Rc::new(RefCell::new(Vec::<u32>::new()));
        let scratch = e.mem.alloc(4);
        let items = queue.clone();
        e.register_double(LIST_ADD, move |e, a| {
            items.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        let items = queue.clone();
        e.register_double(LIST_IS_EMPTY, move |_, _| Ret {
            eax: items.borrow().is_empty() as u32,
            ..Ret::default()
        });
        let items = queue.clone();
        e.register_double(LIST_ITEM_SLOT, move |e, _| {
            e.mem.set_u32(scratch, items.borrow()[0]);
            Ret {
                eax: scratch,
                ..Ret::default()
            }
        });
        let items = queue;
        e.register_double(LIST_POP_FRONT, move |_, _| {
            items.borrow_mut().remove(0);
            Ret::default()
        });
        // The grid.
        e.mem.set_u32(0x011c_63d0, 5);
        e.register(WORD_AT_FOUR_ADDRESS, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        let asked = Rc::new(RefCell::new(Vec::new()));
        let record = asked.clone();
        let cell_slot = e.mem.alloc(4);
        e.register_double(GRID_CELL_SLOT, move |e, a| {
            record.borrow_mut().push((a[1], a[2]));
            let cell = cells
                .iter()
                .find(|(x, y, _)| (*x, *y) == (a[1], a[2]))
                .map_or(0, |(_, _, cell)| *cell);
            e.mem.set_u32(cell_slot, cell);
            Ret {
                eax: cell_slot,
                ..Ret::default()
            }
        });
        (e, asked)
    }

    fn address_of(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(callee, _)| *callee == addr)
            .map(|(_, a)| a.clone())
            .collect()
    }

    #[test]
    fn report_for_an_interior_cell_finishes_before_the_textures() {
        let (mut e, asked) = report_engine(vec![]);
        let owner = e.mem.alloc(0x40);
        let cell = 0x0a00_0042;
        e.mem.set_u32(owner + 0x34, cell);
        e.set_global::<u32>(CELL_OWNER, owner);
        e.register(INTERIOR_CELL_GETTER, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x34),
            ..Ret::default()
        });
        let totals = e.mem.alloc(8);
        e.mem.set_f32(totals, 7.0);
        e.mem.set_f32(totals + 4, 7.0);
        e.call_log = Some(vec![]);
        e.call(0x004a_8bb0, &args![totals, totals + 4, 0, 1]);
        let log = e.call_log.take().unwrap();
        assert!(asked.borrow().is_empty());
        // Entries 0..16 except the skipped 3 and 5 and the factor-less 7:
        // 0.5 * (index + 1) each; counter 9 is 10.
        assert_eq!(e.mem.f32(totals), 59.0);
        assert_eq!(e.mem.f32(totals + 4), 10.0);
        let scene_info = address_of(&log, SCENE_INFO_SET_ASSOCIATED_FORM)[0][0];
        assert_eq!(
            address_of(&log, SCENE_INFO_SET_ASSOCIATED_FORM),
            [vec![scene_info, cell]]
        );
        assert_eq!(
            address_of(&log, COLLECT_SCENE_DATA_FROM_NODE),
            [vec![scene_info, cell + 0x100, 1, 0]]
        );
        let map = address_of(&log, MAP_CONSTRUCT)[0][0] - 4;
        assert_eq!(
            address_of(&log, TEXTURE_USE_COLLECT),
            [vec![map, cell + 0x100, cell, 0]]
        );
        // Interior order: finish, then the texture use.
        let finish = log
            .iter()
            .position(|(addr, _)| *addr == SCENE_INFO_FINISH)
            .unwrap();
        let textures = log
            .iter()
            .position(|(addr, _)| *addr == TEXTURE_USE_COLLECT)
            .unwrap();
        assert!(finish < textures);
        assert_eq!(
            address_of(&log, SCENE_INFO_FINISH),
            [vec![scene_info, cell]]
        );
        // The owner's interior getter is asked twice; the grid is not used.
        assert_eq!(address_of(&log, INTERIOR_CELL_GETTER).len(), 2);
        assert!(address_of(&log, TEXTURE_USE_TOTAL_SIZE).is_empty());
        // The scene info is deleted through its virtual destructor.
        assert_eq!(address_of(&log, FAKE_DESTRUCTOR), [vec![scene_info, 1]]);
        // Teardown order: global table emptied, list, texture map.
        let tail: Vec<u32> = log
            .iter()
            .map(|(addr, _)| *addr)
            .filter(|addr| [MAP_REMOVE_ALL, LIST_DESTRUCT, TEXTURE_USE_DESTRUCT].contains(addr))
            .collect();
        assert_eq!(tail, [MAP_REMOVE_ALL, LIST_DESTRUCT, TEXTURE_USE_DESTRUCT]);
        assert_eq!(address_of(&log, MAP_REMOVE_ALL), [vec![WALKED_TABLE]]);
    }

    #[test]
    fn report_for_the_exterior_scans_the_grid_and_adds_textures() {
        let cell_a = 0x0a00_0001;
        let cell_b = 0x0a00_0002;
        let (mut e, asked) = report_engine(vec![(1, 2, cell_a), (3, 4, cell_b)]);
        let owner = e.mem.alloc(0x40);
        e.set_global::<u32>(CELL_OWNER, owner);
        e.register(INTERIOR_CELL_GETTER, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x34),
            ..Ret::default()
        });
        let totals = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x004a_8bb0, &args![totals, totals + 4, 1, 0]);
        let log = e.call_log.take().unwrap();
        // The grid size 5 gives half 2, reach 2: coordinates 0 to 4, outer
        // index first.
        let asked = asked.borrow();
        assert_eq!(asked.len(), 25);
        assert_eq!(asked[..3], [(0, 0), (0, 1), (0, 2)]);
        assert_eq!(asked[24], (4, 4));
        // Two scene infos: 2 * 59 and 2 * 10 + the texture total 77.
        assert_eq!(e.mem.f32(totals), 118.0);
        assert_eq!(e.mem.f32(totals + 4), 97.0);
        let forms = address_of(&log, SCENE_INFO_SET_ASSOCIATED_FORM);
        assert_eq!(forms.len(), 2);
        assert_eq!([forms[0][1], forms[1][1]], [cell_a, cell_b]);
        // with_addons is 0 here.
        assert!(address_of(&log, COLLECT_SCENE_DATA_FROM_NODE)
            .iter()
            .all(|a| a[2] == 0 && a[3] == 0));
        // Exterior order: the texture use, then the finish step.
        let textures = log
            .iter()
            .position(|(addr, _)| *addr == TEXTURE_USE_COLLECT)
            .unwrap();
        let finish = log
            .iter()
            .position(|(addr, _)| *addr == SCENE_INFO_FINISH)
            .unwrap();
        assert!(textures < finish);
        assert_eq!(address_of(&log, FAKE_DESTRUCTOR).len(), 2);
        assert_eq!(address_of(&log, TEXTURE_USE_TOTAL_SIZE).len(), 1);
        // The interior getter was asked once (it returned null).
        assert_eq!(address_of(&log, INTERIOR_CELL_GETTER).len(), 1);
    }

    #[test]
    fn report_for_a_small_grid_scans_a_smaller_window() {
        let (mut e, asked) = report_engine(vec![]);
        let owner = e.mem.alloc(0x40);
        e.set_global::<u32>(CELL_OWNER, owner);
        e.register(INTERIOR_CELL_GETTER, |_, _| Ret::default());
        // Grid size 3: half 1, reach min(1, 2) = 1: coordinates 0 to 2.
        e.mem.set_u32(0x011c_63d0, 3);
        let totals = e.mem.alloc(8);
        e.call(0x004a_8bb0, &args![totals, totals + 4, 0, 0]);
        assert_eq!(asked.borrow().len(), 9);
        assert_eq!(asked.borrow()[8], (2, 2));
        assert_eq!(e.mem.f32(totals), 0.0);
        assert_eq!(e.mem.f32(totals + 4), 0.0);
    }
}
