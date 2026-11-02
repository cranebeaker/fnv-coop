//! `BSHavok/hknitristripsshape.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

//!
//! The shape keeps an `NiTLargeArray<hkNiTriStripsData>` at `+0x94`: a vtable, the data pointer
//! (`+4`), the capacity (`+8`), the used size (`+0xC`), the count of non-empty entries (`+0x10`)
//! and the growth step (`+0x14`). Its entries are 8 bytes: a counted pointer to the strips data
//! (reference count at `+4` of the object) and a second word. The functions that carry an
//! `FS:[0]` exception frame are marked `Translation incomplete`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhkentity::{release_ni_object, INTERLOCKED_INCREMENT, MEMORY_MANAGER};

/// `NiTriStripsData::GetStripLists` (`00a75f10`, `this` = the strips data): answers the pointer
/// to the 16-bit strip indices.
const GET_STRIP_LISTS: u32 = 0x00a7_5f10;
/// The game heap: raw allocate (`00aa1070`, cdecl, a byte count) and free (`00aa10f0`, cdecl,
/// a pointer); the memory manager's `Deallocate` (`00aa4060`, `this` = the manager, a
/// pointer); the `operator delete` (`00401030`, cdecl, a pointer).
const RAW_ALLOCATE: u32 = 0x00aa_1070;
const FREE_BUFFER: u32 = 0x00aa_10f0;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The shape base constructor (`00d21410`, `this` plus two words).
const SHAPE_BASE_CONSTRUCT: u32 = 0x00d2_1410;
/// The three vtables the shape stores while it is built or destroyed, the vtable of the
/// array, the vtable of the base `hkpShapeContainer` and the vtable the destructor ends with.
const SHAPE_VTABLE: u32 = 0x010c_75c4;
const CONTAINER_VTABLE: u32 = 0x010c_75a4;
const ARRAY_VTABLE: u32 = 0x010c_7600;
const ARRAY_ENDED_VTABLE: u32 = 0x010c_759c;
const SHAPE_ENDED_VTABLE: u32 = 0x0102_e388;
const BASE_VTABLE: u32 = 0x010c_6fb0;
/// The floats the shape's bounding corners start from (`+0x60` and `+0x70`): the largest and
/// the smallest value, and the fourth lane both get.
const BOX_START_LOW: u32 = 0x0104_da48;
const BOX_START_HIGH: u32 = 0x0104_e148;
const BOX_FOURTH_LANE: u32 = 0x0101_1d78;

// Translated from 00ca4520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the triangle `key` of the shape is degenerate. The key's high 12 bits pick the
/// strips data (an array of 8-byte entries at `this + 0x98`), its low 20 bits the triangle:
/// the strips (lengths at data `+0x48`, count at `+0x44`) are walked, each strip of length
/// `n` holding `n - 2` triangles, until the triangle falls in one; the result is true when
/// two of the three consecutive indices of that triangle are equal.
pub fn fn_00ca4520(e: &mut Engine, this: Ptr, key: u32) -> bool {
    let table = e.mem.u32(this.addr() + 0x98);
    let data = e.mem.u32(table + (key >> 20) * 8);
    let strip_count = e.mem.u16(data + 0x44);
    let mut indices = e.call(GET_STRIP_LISTS, &args![data]).u32();
    let lengths = e.mem.u32(data + 0x48);
    let mut triangle = key & 0xfffff;
    let mut strip: u16 = 0;
    if strip_count != 0 {
        loop {
            let length = u32::from(e.mem.u16(lengths + u32::from(strip) * 2));
            if triangle < length.wrapping_sub(2) {
                break;
            }
            triangle = triangle.wrapping_add(2u32.wrapping_sub(length));
            indices = indices.wrapping_add(length * 2);
            strip = strip.wrapping_add(1);
            if strip >= strip_count {
                break;
            }
        }
    }
    let at = indices.wrapping_add(triangle.wrapping_mul(2));
    let first = e.mem.u16(at);
    let second = e.mem.u16(at + 2);
    let third = e.mem.u16(at + 4);
    first == second || first == third || second == third
}

// Translated from 00ca4ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The key of the next triangle after `key` that is not degenerate, or -1 when there is none.
/// `this` is the sub-object 0x10 bytes into the shape that `00ca4520` takes. Each step adds 1;
/// when the low 20 bits reach the strips data's count (the word at `+0x40` of the entry at
/// `this + 0x88`), the key moves to the start of the next data (the high 12 bits plus 1); a key
/// whose data index reaches the count at `this + 0x90` ends the search.
pub fn fn_00ca4ac0(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let mut key = key;
    loop {
        let table = e.mem.u32(this.addr() + 0x88);
        key = key.wrapping_add(1);
        let data_index = key >> 20;
        let data = e.mem.u32(table + data_index * 8);
        let count = i32::from(e.mem.u16(data + 0x40));
        if ((key & 0xfffff) as i32) >= count {
            key = (data_index + 1) << 20;
        }
        if ((key >> 20) as i32) >= e.mem.i32(this.addr() + 0x90) {
            return u32::MAX;
        }
        let degenerate = fn_00ca4520(e, Ptr::new(this.addr().wrapping_sub(0x10)), key);
        if !degenerate {
            return key;
        }
    }
}

/// Whether the 8-byte entry at `entry` is empty: both of its words are 0.
fn entry_is_empty(e: &Engine, entry: u32) -> bool {
    e.mem.u32(entry) == 0 && e.mem.u32(entry + 4) == 0
}

/// Assigns an empty entry (a local with both words set to 0) over the entry at `entry`.
fn assign_empty(e: &mut Engine, entry: u32) {
    e.with_stack(8, |e, empty| {
        e.mem.set_u32(empty.addr(), 0);
        e.mem.set_u32(empty.addr() + 4, 0);
        fn_00ca47a0(e, entry, empty.addr());
    });
}

// Translated from 00c9d890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpShapeContainer::_scalar_deleting_destructor_` (Xbox PDB name from the map; the body
/// only stores the vtable 0x010c6fb0 and, when bit 0 of `flags` is set, runs the `operator
/// delete` `00401030` on the object): answers `this`.
pub fn hkp_shape_container_scalar_deleting_destructor(
    e: &mut Engine,
    this: u32,
    flags: u32,
) -> u32 {
    e.mem.set_u32(this, BASE_VTABLE);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00ca4490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca4490 - the FS:[0] exception frame is not modeled.
/// `fn_00ca4490` (unnamed in the Xbox PDB): the destructor body of the shape. Stores the
/// shape vtable and the container vtable, frees the data block of the array (`+0x98`,
/// `00aa10f0`) after storing the array's ended vtable at `+0x94`, then the base vtable at
/// `+0x10` and the final vtable 0x0102e388 at `+0`.
pub fn fn_00ca4490(e: &mut Engine, this: u32) {
    e.mem.set_u32(this, SHAPE_VTABLE);
    e.mem.set_u32(this + 0x10, CONTAINER_VTABLE);
    let block = e.mem.u32(this + 0x98);
    e.mem.set_u32(this + 0x94, ARRAY_ENDED_VTABLE);
    e.call(FREE_BUFFER, &args![block]);
    e.mem.set_u32(this + 0x10, BASE_VTABLE);
    e.mem.set_u32(this, SHAPE_ENDED_VTABLE);
}

// Translated from 00ca47a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00ca47a0` (unnamed in the Xbox PDB): assigns the 8-byte entry at `source` to the one at
/// `this`. When the counted pointers differ the old one is released (decrement of the count at
/// `+4`, virtual slot 4 when it reaches 0) and the new one is counted; the second word is
/// always copied. Answers `this`.
pub fn fn_00ca47a0(e: &mut Engine, this: u32, source: u32) -> u32 {
    let old = e.mem.u32(this);
    if old != e.mem.u32(source) {
        if old != 0 {
            release_ni_object(e, old);
        }
        let new = e.mem.u32(source);
        e.mem.set_u32(this, new);
        if new != 0 {
            e.call(INTERLOCKED_INCREMENT, &args![new + 4]);
        }
    }
    let extra = e.mem.u32(source + 4);
    e.mem.set_u32(this + 4, extra);
    this
}

// Translated from 00ca47f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca47f0 - the FS:[0] exception frame is not modeled, and the new block's slots are read before they are written (the exe's allocator is not shown to clear them; the engine's heap gives zeros).
/// `NiTLargeArray<hkNiTriStripsData>::SetSize` (Xbox PDB): takes the new capacity. Does nothing
/// when it equals the capacity (`+8`). When it is below the used size (`+0xC`) the entries
/// from the new capacity on are emptied (an entry that is not empty is assigned an empty
/// entry through `fn_00ca47a0` and counted out of `+0x10`) and the used size becomes the new
/// capacity. The capacity is set; for 0 the data pointer becomes 0, otherwise a block of 8
/// bytes per entry is allocated (`00aa1070`), the used entries are assigned over from the old
/// block and the other slots are emptied. The old block is freed (`00aa10f0`).
pub fn ni_t_large_array_hk_ni_tri_strips_data_set_size(e: &mut Engine, this: u32, new_size: u32) {
    if new_size == e.mem.u32(this + 8) {
        return;
    }
    if new_size < e.mem.u32(this + 0xc) {
        let mut index = new_size;
        loop {
            let entry = e.mem.u32(this + 4).wrapping_add(index.wrapping_mul(8));
            if !entry_is_empty(e, entry) {
                assign_empty(e, entry);
                let count = e.mem.u32(this + 0x10).wrapping_sub(1);
                e.mem.set_u32(this + 0x10, count);
            }
            index = index.wrapping_add(1);
            if index >= e.mem.u32(this + 0xc) {
                break;
            }
        }
        e.mem.set_u32(this + 0xc, new_size);
    }
    let old = e.mem.u32(this + 4);
    e.mem.set_u32(this + 8, new_size);
    if new_size == 0 {
        e.mem.set_u32(this + 4, 0);
    } else {
        let fresh = e.call(RAW_ALLOCATE, &args![new_size.wrapping_mul(8)]).u32();
        e.mem.set_u32(this + 4, fresh);
        if e.mem.u32(this + 0xc) != 0 {
            let mut index = 0u32;
            loop {
                let destination = e.mem.u32(this + 4).wrapping_add(index * 8);
                fn_00ca47a0(e, destination, old.wrapping_add(index * 8));
                index += 1;
                if index >= e.mem.u32(this + 0xc) {
                    break;
                }
            }
        }
        let mut index = e.mem.u32(this + 0xc);
        if index < e.mem.u32(this + 8) {
            loop {
                let entry = e.mem.u32(this + 4).wrapping_add(index.wrapping_mul(8));
                assign_empty(e, entry);
                index += 1;
                if index >= e.mem.u32(this + 8) {
                    break;
                }
            }
        }
    }
    e.call(FREE_BUFFER, &args![old]);
}

// Translated from 00ca49a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTLargeArray<hkNiTriStripsData>::SetAt` (Xbox PDB): stores the entry at `source` at
/// `index`. At or past the used size (`+0xC`) the used size becomes `index + 1` and the count
/// of non-empty entries (`+0x10`) grows when `source` is not empty; below it the count changes
/// by the difference between the old and the new entry being empty (+1 when an empty entry is
/// replaced by a non-empty one, -1 for the reverse). The entry is then assigned as in
/// `fn_00ca47a0`.
pub fn ni_t_large_array_hk_ni_tri_strips_data_set_at(
    e: &mut Engine,
    this: u32,
    index: u32,
    source: u32,
) {
    let source_filled = !entry_is_empty(e, source);
    if index >= e.mem.u32(this + 0xc) {
        e.mem.set_u32(this + 0xc, index.wrapping_add(1));
        if source_filled {
            let count = e.mem.u32(this + 0x10).wrapping_add(1);
            e.mem.set_u32(this + 0x10, count);
        }
    } else {
        let slot = e.mem.u32(this + 4).wrapping_add(index.wrapping_mul(8));
        let slot_filled = !entry_is_empty(e, slot);
        if source_filled && !slot_filled {
            let count = e.mem.u32(this + 0x10).wrapping_add(1);
            e.mem.set_u32(this + 0x10, count);
        } else if !source_filled && slot_filled {
            let count = e.mem.u32(this + 0x10).wrapping_sub(1);
            e.mem.set_u32(this + 0x10, count);
        }
    }
    let slot = e.mem.u32(this + 4).wrapping_add(index.wrapping_mul(8));
    fn_00ca47a0(e, slot, source);
}

// Translated from 00ca4a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00ca4a90` (unnamed in the Xbox PDB; it has the shape of the shape's scalar deleting
/// destructor, unconfirmed): runs the destructor body `fn_00ca4490` and, when bit 0 of `flags`
/// is set and `this` is not 0, frees the block the object lives in: the byte before the
/// object is the distance back to the block, handed to `MemoryManager::Deallocate`
/// (`00aa4060`). Answers `this`.
pub fn fn_00ca4a90(e: &mut Engine, this: u32, flags: u32) -> u32 {
    fn_00ca4490(e, this);
    if flags & 1 != 0 && this != 0 {
        let distance = u32::from(e.mem.u8(this.wrapping_sub(1)));
        e.call(
            MEMORY_DEALLOCATE,
            &args![MEMORY_MANAGER, this.wrapping_sub(distance)],
        );
    }
    this
}

// Translated from 00ca4b20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca4b20 - the FS:[0] exception frame is not modeled.
/// `NiTLargeArray<hkNiTriStripsData>::RemoveAll` (Xbox PDB): for each of the used entries
/// (`+0xC`) releases the counted pointer, if any (decrement of the count at `+4`, virtual slot
/// 4 when it reaches 0), and clears both words of the entry; then clears the count of
/// non-empty entries (`+0x10`) and the used size.
pub fn ni_t_large_array_hk_ni_tri_strips_data_remove_all(e: &mut Engine, this: u32) {
    let mut index = 0u32;
    while index < e.mem.u32(this + 0xc) {
        let entry = e.mem.u32(this + 4).wrapping_add(index.wrapping_mul(8));
        let object = e.mem.u32(entry);
        if object != 0 {
            release_ni_object(e, object);
            e.mem.set_u32(entry, 0);
        }
        e.mem.set_u32(entry + 4, 0);
        index += 1;
    }
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this + 0xc, 0);
}

// Translated from 00ca4bc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ca4bc0 - the FS:[0] exception frame is not modeled.
/// `hkNiTriStripsShape::hkNiTriStripsShape` (Xbox PDB): takes the construction info. Runs the
/// base constructor (`00d21410`) on `(this, 0x1e, 3)`, stores the shape and container vtables,
/// the info's float at `+4` (through the x87 stack) at `+0x90`, an empty array at `+0x94`
/// (growth step 1, vtable 0x010c7600), the info's four floats at `+0x20..+0x30` at `+0x80`
/// and then every entry of the info (the count at `+0x14`, signed; entries from the pointer at
/// `+0xC`) into the array: growing it by its step plus the used size first whenever the used
/// size has reached the capacity. The bounding corners at `+0x60` and `+0x70` start from the
/// largest and the smallest float (the fourth lane both get is the float at 0x01011d78), and
/// the byte at `+0x14` becomes 1. Answers `this`.
pub fn hk_ni_tri_strips_shape_hk_ni_tri_strips_shape(e: &mut Engine, this: u32, info: u32) -> u32 {
    e.call(SHAPE_BASE_CONSTRUCT, &args![this, 0x1eu32, 3u32]);
    e.mem.set_u32(this, SHAPE_VTABLE);
    e.mem.set_u32(this + 0x10, CONTAINER_VTABLE);
    let float = x87_quiet_bits(e.mem.u32(info + 4));
    e.mem.set_u32(this + 0x90, float);
    let array = this + 0x94;
    for offset in [8, 0xc, 0x10, 4] {
        e.mem.set_u32(array + offset, 0);
    }
    e.mem.set_u32(array + 0x14, 1);
    e.mem.set_u32(array, ARRAY_VTABLE);
    let float = x87_quiet_bits(e.mem.u32(info + 4));
    e.mem.set_u32(this + 0x90, float);
    for lane in 0..4 {
        let value = e.mem.u32(info + 0x20 + 4 * lane);
        e.mem.set_u32(this + 0x80 + 4 * lane, value);
    }
    let mut index = 0i32;
    while index < e.mem.i32(info + 0x14) {
        let entry = e.mem.u32(info + 0xc).wrapping_add(index as u32 * 8);
        let used = e.mem.u32(array + 0xc);
        if used >= e.mem.u32(array + 8) {
            let step = e.mem.u32(array + 0x14);
            ni_t_large_array_hk_ni_tri_strips_data_set_size(e, array, step.wrapping_add(used));
        }
        ni_t_large_array_hk_ni_tri_strips_data_set_at(e, array, used, entry);
        index += 1;
    }
    let low = e.mem.u32(BOX_START_LOW);
    let high = e.mem.u32(BOX_START_HIGH);
    let fourth = e.mem.u32(BOX_FOURTH_LANE);
    for (offset, value) in [(0x60, low), (0x70, high)] {
        for lane in 0..3 {
            e.mem.set_u32(this + offset + 4 * lane, value);
        }
        e.mem.set_u32(this + offset + 12, fourth);
    }
    e.mem.set_u8(this + 0x14, 1);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c9d890,
            hkp_shape_container_scalar_deleting_destructor(u32, u32) -> u32
        ),
        entry!(0x00ca4490, fn_00ca4490(u32)),
        entry!(0x00ca_4520, fn_00ca4520(Ptr, u32) -> bool),
        entry!(0x00ca47a0, fn_00ca47a0(u32, u32) -> u32),
        entry!(
            0x00ca47f0,
            ni_t_large_array_hk_ni_tri_strips_data_set_size(u32, u32)
        ),
        entry!(
            0x00ca49a0,
            ni_t_large_array_hk_ni_tri_strips_data_set_at(u32, u32, u32)
        ),
        entry!(0x00ca4a90, fn_00ca4a90(u32, u32) -> u32),
        entry!(0x00ca_4ac0, fn_00ca4ac0(Ptr, u32) -> u32),
        entry!(
            0x00ca4b20,
            ni_t_large_array_hk_ni_tri_strips_data_remove_all(u32)
        ),
        entry!(
            0x00ca4bc0,
            hk_ni_tri_strips_shape_hk_ni_tri_strips_shape(u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// A shape with one strips data of two strips (lengths 5 and 4): the indices
    /// `0 1 2 3 4` and `5 5 6 7`. Returns the shape (`+0x98` = table) and the sub-object 0x10
    /// bytes into it (`+0x88` = table, `+0x90` = 1 data; the shape fields are `+0x98` and `+0xa0`).
    fn rig() -> (Engine, u32, u32) {
        let mut e = test_engine();
        let indices = e.mem.alloc(0x20);
        for (i, v) in [0u16, 1, 2, 3, 4, 5, 5, 6, 7].iter().enumerate() {
            e.mem.set_u16(indices + 2 * i as u32, *v);
        }
        let lengths = e.mem.alloc(8);
        e.mem.set_u16(lengths, 5);
        e.mem.set_u16(lengths + 2, 4);
        let data = e.mem.alloc(0x60);
        e.mem.set_u16(data + 0x40, 5);
        e.mem.set_u16(data + 0x44, 2);
        e.mem.set_u32(data + 0x48, lengths);
        e.register_double(GET_STRIP_LISTS, move |_, a| {
            assert_eq!(a[0], data);
            Ret {
                eax: indices,
                ..Ret::default()
            }
        });
        let table = e.mem.alloc(8);
        e.mem.set_u32(table, data);
        let shape = e.mem.alloc(0xb0);
        e.mem.set_u32(shape + 0x98, table);
        e.mem.set_u32(shape + 0xa0, 1);
        (e, shape, shape + 0x10)
    }

    #[test]
    fn a_triangle_is_degenerate_when_two_indices_match() {
        let (mut e, shape, _) = rig();
        let degenerate = |e: &mut Engine, key: u32| e.call(0x00ca_4520, &args![shape, key]).bool();
        assert!(!degenerate(&mut e, 0));
        assert!(!degenerate(&mut e, 2));
        // The first triangle of the second strip is 5, 5, 6.
        assert!(degenerate(&mut e, 3));
        assert!(!degenerate(&mut e, 4));
    }

    #[test]
    fn the_next_valid_triangle_skips_degenerate_ones() {
        let (mut e, _, sub) = rig();
        assert_eq!(e.call(0x00ca_4ac0, &args![sub, 2u32]).u32(), 4);
        assert_eq!(e.call(0x00ca_4ac0, &args![sub, 0u32]).u32(), 1);
        // Past the last triangle the key moves to the next data, which does not exist.
        assert_eq!(e.call(0x00ca_4ac0, &args![sub, 4u32]).u32(), u32::MAX);
    }
}

#[cfg(test)]
mod array_tests {
    use super::*;
    use crate::units::bshavok::bhkentity::testing::{
        calls_to, interlocked_doubles, logging_engine, ni_object,
    };
    use crate::units::bshavok::bhkentity::INTERLOCKED_DECREMENT;

    /// The double that stands for virtual slot 4 (the delete) of the strips data objects.
    const DELETE_SLOT: u32 = 0x0900_0004;

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// An engine with the interlocked imports, the delete slot and the heap routines doubled
    /// (the allocator answers zeroed blocks).
    fn engine() -> Engine {
        let mut e = logging_engine();
        interlocked_doubles(&mut e);
        e.register(DELETE_SLOT, nothing);
        e.register(RAW_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0]);
            for offset in (0..a[0]).step_by(4) {
                e.mem.set_u32(block + offset, 0);
            }
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(FREE_BUFFER, nothing);
        e.register(MEMORY_DEALLOCATE, nothing);
        e.register(OPERATOR_DELETE, nothing);
        e
    }

    /// An array with room for `capacity` entries holding `entries` (pointer, extra word).
    fn array(e: &mut Engine, capacity: u32, entries: &[(u32, u32)]) -> u32 {
        let array = e.mem.alloc(0x20);
        let data = e.mem.alloc(8 * capacity.max(1));
        e.mem.set_u32(array + 4, data);
        e.mem.set_u32(array + 8, capacity);
        e.mem.set_u32(array + 0xc, entries.len() as u32);
        let filled = entries.iter().filter(|p| **p != (0, 0)).count();
        e.mem.set_u32(array + 0x10, filled as u32);
        e.mem.set_u32(array + 0x14, 3);
        for (i, (pointer, extra)) in entries.iter().enumerate() {
            e.mem.set_u32(data + 8 * i as u32, *pointer);
            e.mem.set_u32(data + 8 * i as u32 + 4, *extra);
        }
        array
    }

    fn entry_at(e: &Engine, array: u32, index: u32) -> (u32, u32) {
        let data = e.mem.u32(array + 4);
        (e.mem.u32(data + 8 * index), e.mem.u32(data + 8 * index + 4))
    }

    fn pair(e: &mut Engine, pointer: u32, extra: u32) -> u32 {
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, pointer);
        e.mem.set_u32(pair + 4, extra);
        pair
    }

    #[test]
    fn the_container_destructor_deletes_on_request() {
        let mut e = engine();
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x00c9_d890, &args![this, 0u32]).u32(), this);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.call(0x00c9_d890, &args![this, 1u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), [[this]]);
    }

    #[test]
    fn the_destructor_body_frees_the_array_block_and_ends_with_its_vtables() {
        let mut e = engine();
        let this = e.mem.alloc(0xc0);
        e.mem.set_u32(this + 0x98, 0x5000);
        e.call(0x00ca_4490, &args![this]);
        assert_eq!(calls_to(&e, FREE_BUFFER), [[0x5000]]);
        assert_eq!(e.mem.u32(this), SHAPE_ENDED_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x94), ARRAY_ENDED_VTABLE);
    }

    #[test]
    fn assigning_an_entry_swaps_the_counted_pointers() {
        let mut e = engine();
        let old = ni_object(&mut e, 1, DELETE_SLOT);
        let new = ni_object(&mut e, 4, DELETE_SLOT);
        let target = pair(&mut e, old, 7);
        let source = pair(&mut e, new, 9);
        assert_eq!(e.call(0x00ca_47a0, &args![target, source]).u32(), target);
        assert_eq!(e.mem.u32(target), new);
        assert_eq!(e.mem.u32(target + 4), 9);
        assert_eq!(e.mem.u32(new + 4), 5);
        // The old object count reached 0, so its delete slot ran.
        assert_eq!(e.mem.u32(old + 4), 0);
        assert_eq!(calls_to(&e, DELETE_SLOT), [[old]]);
    }

    #[test]
    fn assigning_the_same_pointer_only_copies_the_second_word() {
        let mut e = engine();
        let object = ni_object(&mut e, 2, DELETE_SLOT);
        let target = pair(&mut e, object, 7);
        let source = pair(&mut e, object, 9);
        e.call(0x00ca_47a0, &args![target, source]);
        assert_eq!(e.mem.u32(object + 4), 2);
        assert_eq!(e.mem.u32(target + 4), 9);
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
        assert!(calls_to(&e, INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn assigning_an_empty_entry_releases_without_counting() {
        let mut e = engine();
        let old = ni_object(&mut e, 2, DELETE_SLOT);
        let target = pair(&mut e, old, 7);
        let source = pair(&mut e, 0, 0);
        e.call(0x00ca_47a0, &args![target, source]);
        assert_eq!(e.mem.u32(old + 4), 1);
        assert_eq!((e.mem.u32(target), e.mem.u32(target + 4)), (0, 0));
        assert!(calls_to(&e, DELETE_SLOT).is_empty());
        assert!(calls_to(&e, INTERLOCKED_INCREMENT).is_empty());
    }

    #[test]
    fn set_size_to_the_same_capacity_does_nothing() {
        let mut e = engine();
        let array = array(&mut e, 4, &[]);
        e.call(0x00ca_47f0, &args![array, 4u32]);
        assert!(calls_to(&e, RAW_ALLOCATE).is_empty());
        assert!(calls_to(&e, FREE_BUFFER).is_empty());
    }

    #[test]
    fn growing_copies_the_entries_into_a_new_block() {
        let mut e = engine();
        let object = ni_object(&mut e, 1, DELETE_SLOT);
        let array = array(&mut e, 2, &[(object, 5), (0, 0)]);
        let old = e.mem.u32(array + 4);
        e.call(0x00ca_47f0, &args![array, 5u32]);
        assert_eq!(calls_to(&e, RAW_ALLOCATE), [[40]]);
        assert_eq!(e.mem.u32(array + 8), 5);
        assert_ne!(e.mem.u32(array + 4), old);
        assert_eq!(entry_at(&e, array, 0), (object, 5));
        assert_eq!(entry_at(&e, array, 1), (0, 0));
        assert_eq!(entry_at(&e, array, 4), (0, 0));
        // The copy counted the object a second time; the old block was freed.
        assert_eq!(e.mem.u32(object + 4), 2);
        assert_eq!(calls_to(&e, FREE_BUFFER), [[old]]);
    }

    #[test]
    fn shrinking_empties_the_entries_past_the_new_capacity() {
        let mut e = engine();
        let first = ni_object(&mut e, 2, DELETE_SLOT);
        let second = ni_object(&mut e, 1, DELETE_SLOT);
        let array = array(&mut e, 4, &[(first, 1), (second, 2), (0, 0)]);
        e.call(0x00ca_47f0, &args![array, 1u32]);
        assert_eq!(e.mem.u32(array + 0xc), 1);
        assert_eq!(e.mem.u32(array + 8), 1);
        assert_eq!(e.mem.u32(array + 0x10), 1);
        assert_eq!(entry_at(&e, array, 0), (first, 1));
        // The second entry was released and deleted on the way.
        assert_eq!(calls_to(&e, DELETE_SLOT), [[second]]);
    }

    #[test]
    fn shrinking_to_zero_drops_the_block() {
        let mut e = engine();
        let array = array(&mut e, 2, &[(0, 0)]);
        let old = e.mem.u32(array + 4);
        e.call(0x00ca_47f0, &args![array, 0u32]);
        assert_eq!(e.mem.u32(array + 4), 0);
        assert_eq!(e.mem.u32(array + 8), 0);
        assert!(calls_to(&e, RAW_ALLOCATE).is_empty());
        assert_eq!(calls_to(&e, FREE_BUFFER), [[old]]);
    }

    #[test]
    fn set_at_counts_entries_that_become_filled_or_empty() {
        let mut e = engine();
        let object = ni_object(&mut e, 1, DELETE_SLOT);
        let array = array(&mut e, 8, &[(0, 0), (object, 3)]);
        let filled = pair(&mut e, object, 8);
        let empty = pair(&mut e, 0, 0);
        // Past the used size: used size 3, count up.
        e.call(0x00ca_49a0, &args![array, 2u32, filled]);
        assert_eq!(e.mem.u32(array + 0xc), 3);
        assert_eq!(e.mem.u32(array + 0x10), 2);
        // Below the used size: an empty slot receiving a filled entry counts up.
        e.call(0x00ca_49a0, &args![array, 0u32, filled]);
        assert_eq!(e.mem.u32(array + 0x10), 3);
        // A filled slot receiving an empty entry counts down.
        e.call(0x00ca_49a0, &args![array, 1u32, empty]);
        assert_eq!(e.mem.u32(array + 0x10), 2);
        // Filled over filled, and empty over empty, leave the count alone.
        e.call(0x00ca_49a0, &args![array, 0u32, filled]);
        e.call(0x00ca_49a0, &args![array, 1u32, empty]);
        assert_eq!(e.mem.u32(array + 0x10), 2);
        assert_eq!(entry_at(&e, array, 0), (object, 8));
        assert_eq!(entry_at(&e, array, 1), (0, 0));
        // An empty entry past the used size does not count.
        e.call(0x00ca_49a0, &args![array, 6u32, empty]);
        assert_eq!(e.mem.u32(array + 0xc), 7);
        assert_eq!(e.mem.u32(array + 0x10), 2);
    }

    #[test]
    fn the_scalar_deleting_destructor_frees_the_aligned_block() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0) + 0x10;
        e.mem.set_u8(this - 1, 6);
        assert_eq!(e.call(0x00ca_4a90, &args![this, 0u32]).u32(), this);
        assert!(calls_to(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00ca_4a90, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            [[MEMORY_MANAGER, this - 6]]
        );
        assert_eq!(e.mem.u32(this), SHAPE_ENDED_VTABLE);
    }

    #[test]
    fn remove_all_releases_and_clears_every_used_entry() {
        let mut e = engine();
        let first = ni_object(&mut e, 1, DELETE_SLOT);
        let second = ni_object(&mut e, 2, DELETE_SLOT);
        let array = array(&mut e, 4, &[(first, 1), (0, 5), (second, 2)]);
        e.call(0x00ca_4b20, &args![array]);
        assert_eq!(e.mem.u32(array + 0xc), 0);
        assert_eq!(e.mem.u32(array + 0x10), 0);
        for index in 0..3 {
            assert_eq!(entry_at(&e, array, index), (0, 0));
        }
        assert_eq!(calls_to(&e, DELETE_SLOT), [[first]]);
        assert_eq!(e.mem.u32(second + 4), 1);
    }

    #[test]
    fn the_constructor_copies_the_entries_into_a_growing_array() {
        let mut e = engine();
        e.register(SHAPE_BASE_CONSTRUCT, nothing);
        for (address, bits) in [
            (BOX_START_LOW, 0x7f7f_ffeeu32),
            (BOX_START_HIGH, 0xff7f_ffee),
            (BOX_FOURTH_LANE, 0x3f80_0000),
        ] {
            e.map(address, 4);
            e.mem.set_u32(address, bits);
        }
        let first = ni_object(&mut e, 1, DELETE_SLOT);
        let second = ni_object(&mut e, 1, DELETE_SLOT);
        let info = e.mem.alloc(0x40);
        let entries = e.mem.alloc(16);
        e.mem.set_u32(entries, first);
        e.mem.set_u32(entries + 4, 10);
        e.mem.set_u32(entries + 8, second);
        e.mem.set_u32(entries + 12, 20);
        e.mem.set_u32(info + 4, 0x7f80_0001);
        e.mem.set_u32(info + 0xc, entries);
        e.mem.set_u32(info + 0x14, 2);
        for lane in 0..4 {
            e.mem.set_u32(info + 0x20 + 4 * lane, 0x100 + lane);
        }
        let this = e.mem.alloc(0xc0);
        assert_eq!(e.call(0x00ca_4bc0, &args![this, info]).u32(), this);
        assert_eq!(calls_to(&e, SHAPE_BASE_CONSTRUCT), [[this, 0x1e, 3]]);
        assert_eq!(e.mem.u32(this), SHAPE_VTABLE);
        assert_eq!(e.mem.u32(this + 0x10), CONTAINER_VTABLE);
        assert_eq!(e.mem.u32(this + 0x90), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 0x94), ARRAY_VTABLE);
        assert_eq!(e.mem.u32(this + 0x80 + 8), 0x102);
        let array = this + 0x94;
        assert_eq!(e.mem.u32(array + 0xc), 2);
        assert_eq!(e.mem.u32(array + 0x10), 2);
        assert_eq!(entry_at(&e, array, 0), (first, 10));
        assert_eq!(entry_at(&e, array, 1), (second, 20));
        // The array grew from empty by its step 1 plus the used size, twice.
        assert_eq!(calls_to(&e, RAW_ALLOCATE), [[8], [16]]);
        assert_eq!(e.mem.u32(this + 0x60), 0x7f7f_ffee);
        assert_eq!(e.mem.u32(this + 0x6c), 0x3f80_0000);
        assert_eq!(e.mem.u32(this + 0x70), 0xff7f_ffee);
        assert_eq!(e.mem.u8(this + 0x14), 1);
    }
}
