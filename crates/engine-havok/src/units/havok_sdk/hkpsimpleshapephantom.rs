//! `Havok SDK/hkpsimpleshapephantom.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::havok_sdk::hkpsphereshape::{
    copy_vec, load_bits, load_vec, maxps, minps, store_bits, store_vec, sum3, zip_with,
};

/// The memory router (`0x00c8_5750`): its field `+0x10` is the allocator the block is freed through.
const MEMORY_ROUTER: u32 = 0x00c8_5750;

// Translated from 00d1f9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deleting destructor: runs the base destructor (`0x00d1_f780`) on `this` and, when bit 0 of
/// `flags` is set, returns the block to the memory router's allocator (slot `8`) with the
/// 16-bit size kept at `this + 4`. Returns `this`. The class is unconfirmed.
pub fn deleting_destructor_00d1f9c0(e: &mut Engine, this: Ptr, flags: u8) -> Ptr {
    e.call(0x00d1_f780, &args![this]);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this.addr() + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// `hkArrayUtil::_reserveMore` (`00c90940`, cdecl: allocator, the array, element size).
const ARRAY_RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkpPhantom::fireCollidableAdded` (`00ca94c0`, thiscall, one stack word) and
/// `fireCollidableRemoved` (`00ca9520`, thiscall, two stack words: the collidable and a
/// `hkBool` pushed as a whole word).
const FIRE_COLLIDABLE_ADDED: u32 = 0x00ca_94c0;
const FIRE_COLLIDABLE_REMOVED: u32 = 0x00ca_9520;
/// `hkpWorldObject::copyProperties` (`00ca92a0`, thiscall, one stack word: the source).
const COPY_PROPERTIES: u32 = 0x00ca_92a0;
/// `hkpPhantom::updateBroadPhase` (`00d07ec0`, thiscall, one stack word: the AABB).
const UPDATE_BROAD_PHASE: u32 = 0x00d0_7ec0;
/// `hkpPhantom::calcContentStatistics` (`00d07cf0`, thiscall, collector and class record) and
/// the class record `hkpSimpleShapePhantom` passes (`0126902c`).
const PHANTOM_CALC_CONTENT_STATISTICS: u32 = 0x00d0_7cf0;
const PHANTOM_CLASS: u32 = 0x0126_902c;
/// `hkpPhantom::~hkpPhantom` (`00d08130`, thiscall).
const PHANTOM_DESTRUCTOR: u32 = 0x00d0_8130;
/// `hkStatisticsCollector::getAllocatedSize` (`00d06f80`, cdecl: block, size).
const GET_ALLOCATED_SIZE: u32 = 0x00d0_6f80;
/// The constructor of `hkpShapePhantom` (`00c9ea40`, thiscall, two stack words) and its
/// `deallocateInternalArrays` (`00c9e820`, thiscall).
const SHAPE_PHANTOM_CONSTRUCT: u32 = 0x00c9_ea40;
const SHAPE_PHANTOM_DEALLOCATE_INTERNAL_ARRAYS: u32 = 0x00c9_e820;
/// The vtable of `hkpSimpleShapePhantom`.
const SIMPLE_SHAPE_PHANTOM_VTABLE: u32 = 0x010c_e15c;
/// The name `calcContentStatistics` reports its array under: "OverlapPtr".
const OVERLAP_PTR_NAME: u32 = 0x010c_e148;
/// `0.5` as a `double` (the tolerance scale of the linear casts) and the tiny `float` the
/// linear cast record starts with (`1.1920929e-7`, overwritten before it is used).
const TOLERANCE_SCALE: u32 = 0x0101_1588;
const CAST_RECORD_EPSILON: u32 = 0x0104_da4c;

// Translated from 00cf2ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkAlgorithm::quickSortRecursive<hkpSimpleShapePhantom::CollisionDetail,
/// hkpSimpleShapePhantom::OrderByUid>` (engine map), cdecl `(array, low, high, order)`: sorts
/// the pointers in `array[low..=high]` by the unsigned word at +0x14 of what they point to,
/// with the middle element as the pivot, recursing into the lower part and looping on the
/// upper. `order` (the empty comparator, passed as a word) is only handed on to the
/// recursive call.
pub fn hk_algorithm_quick_sort_recursive_collision_detail_order_by_uid(
    e: &mut Engine,
    array: Ptr,
    low: i32,
    high: i32,
    order: u32,
) {
    let table = array.addr();
    let mut low = low;
    loop {
        let mut left = low;
        let mut right = high;
        let pivot = item_at(e, table, low.wrapping_add(high) >> 1);
        loop {
            let pivot_uid = e.mem.u32(pivot + 0x14);
            while uid_at(e, table, left) < pivot_uid {
                left = left.wrapping_add(1);
            }
            while pivot_uid < uid_at(e, table, right) {
                right = right.wrapping_sub(1);
            }
            if right < left {
                break;
            }
            if right != left {
                let first = item_at(e, table, left);
                let second = item_at(e, table, right);
                e.mem.set_u32(item_address(table, right), first);
                e.mem.set_u32(item_address(table, left), second);
            }
            right = right.wrapping_sub(1);
            left = left.wrapping_add(1);
            if left > right {
                break;
            }
        }
        if low < right {
            hk_algorithm_quick_sort_recursive_collision_detail_order_by_uid(
                e, array, low, right, order,
            );
        }
        if left >= high {
            break;
        }
        low = left;
    }
}

fn item_address(table: u32, index: i32) -> u32 {
    table.wrapping_add((index as u32).wrapping_mul(4))
}

fn item_at(e: &Engine, table: u32, index: i32) -> u32 {
    e.mem.u32(item_address(table, index))
}

fn uid_at(e: &Engine, table: u32, index: i32) -> u32 {
    let record = item_at(e, table, index);
    e.mem.u32(record + 0x14)
}

// Translated from 00d1f220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleShapePhantom::getType` (Xbox PDB): the phantom type `1`. Register value first:
/// ECX (`this`) is never read.
pub fn hkp_simple_shape_phantom_get_type(_e: &mut Engine, _this: Ptr) -> u32 {
    1
}

// Translated from 00d1f230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleShapePhantom::isOverlappingCollidableAdded` (Xbox PDB), `(this, result,
/// collidable)`: stores `1` in the byte `result` points to when the pointer array at +0x160
/// (size +0x164) holds `collidable`, else `0`. Returns `result` (the exe leaves it in EAX).
pub fn hkp_simple_shape_phantom_is_overlapping_collidable_added(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    collidable: u32,
) -> Ptr {
    let base = this.addr();
    let size = e.mem.i32(base + 0x164);
    let table = e.mem.u32(base + 0x160);
    let found = (0..size).any(|index| e.mem.u32(item_address(table, index)) == collidable);
    e.mem.set_u8(result.addr(), u8::from(found));
    result
}

// Translated from 00d1f270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleShapePhantom::setPositionAndLinearCast` (Xbox PDB), `(this, position, input,
/// cast_collector, start_collector)`, `RET 0x10`. Copies the 16 bytes at `position` to
/// `this + 0xe0`; asks the shape (the word at +0x10) for its AABB through slot `0x1c`
/// (`transform = this + 0xb0`, `tolerance`, `aabb` on the stack) with `tolerance =
/// float(float at (world->+0x78)+8 * 0.5) + float at input+0x14` (each step rounded to
/// `float`), widens the AABB by the path `input[0..4] - position` (`min(0, path)` added to
/// the minimum, `max(0, path)` to the maximum) and calls `hkpPhantom::updateBroadPhase` with
/// it. Then builds the cast record on the stack: the words +0 .. +0x14 and the vectors
/// +0x20, +0x30, +0x40 of the world's collision input (the word at `world + 0x78`; the
/// word at +8 is then replaced by the float at `input + 0x14`), the path at +0x50, the float
/// at `input + 0x10` at +0x60, the path's length at +0x64 and the word at `collision input +
/// 0x70` at +0x68. Finally, for each overlapping collidable (the array at +0x160, size
/// +0x164) from the last to the first, looks up the linear cast function for the pair of
/// shape types in the world's dispatcher (`world + 0x80`: the byte at `((type A + 0xd) * 32 +
/// type B)` selects the five-word entry at `+0x9ac`) and calls it with `(this + 0x10, the
/// other collidable, the record, cast_collector, start_collector)`, cdecl.
pub fn hkp_simple_shape_phantom_set_position_and_linear_cast(
    e: &mut Engine,
    this: Ptr,
    position: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    let base = this.addr();
    let position_bits = load_bits(e, position.addr());
    let shape = e.mem.u32(base + 0x10);
    let world = e.mem.u32(base + 8);
    store_bits(e, base + 0xe0, position_bits);
    let collision_input = e.mem.u32(world + 0x78);
    let scale = x87_quiet(e.mem.f32(collision_input + 8));
    let half: f64 = e.global(TOLERANCE_SCALE);
    let scaled = (f64::from(scale) * half) as f32;
    let extra = x87_quiet(e.mem.f32(input.addr() + 0x14));
    let tolerance = x87_quiet((f64::from(scaled) + f64::from(extra)) as f32);
    e.with_stack(0xb0, |e, frame| {
        let aabb = frame.addr();
        let record = aabb + 0x20;
        let path_slot = aabb + 0x90;
        e.vcall(shape, 0x1c, &args![base + 0xb0, tolerance, aabb]);
        let start = load_vec(e, position.addr());
        let end = load_vec(e, input.addr());
        let aabb_min = load_vec(e, aabb);
        let path = zip_with(end, start, |end, start| end - start);
        let lower = path.map(|lane| minps(0.0, lane));
        let upper = path.map(|lane| maxps(0.0, lane));
        store_vec(e, path_slot, path);
        let aabb_max = load_vec(e, aabb + 0x10);
        let widened_min = zip_with(lower, aabb_min, |lower, minimum| lower + minimum);
        let widened_max = zip_with(upper, aabb_max, |upper, maximum| upper + maximum);
        store_vec(e, aabb, widened_min);
        store_vec(e, aabb + 0x10, widened_max);
        e.call(UPDATE_BROAD_PHASE, &args![this, aabb]);
        let epsilon = x87_quiet(e.global::<f32>(CAST_RECORD_EPSILON));
        e.mem.set_f32(record + 0x60, epsilon);
        let world = e.mem.u32(base + 8);
        e.mem.set_u32(record + 4, 0);
        let collision_input = e.mem.u32(world + 0x78);
        for offset in [0u32, 4, 8, 0xc, 0x10, 0x14] {
            let word = e.mem.u32(collision_input + offset);
            e.mem.set_u32(record + offset, word);
        }
        for offset in [0x20u32, 0x30, 0x40] {
            copy_vec(e, collision_input + offset, record + offset);
        }
        let last_word = e.mem.u32(collision_input + 0x70);
        let path = load_vec(e, path_slot);
        store_vec(e, record + 0x50, path);
        e.mem.set_u32(record + 0x68, last_word);
        let length = sum3(path.map(|lane| lane * lane)).sqrt();
        e.mem
            .set_u32(record + 0x64, x87_quiet_bits(length.to_bits()));
        let extra_word = x87_quiet_bits(e.mem.u32(input.addr() + 0x14));
        e.mem.set_u32(record + 8, extra_word);
        let cast_word = x87_quiet_bits(e.mem.u32(input.addr() + 0x10));
        e.mem.set_u32(record + 0x60, cast_word);
        let mut index = e.mem.i32(base + 0x164).wrapping_sub(1);
        let dispatcher = e.mem.u32(world + 0x80);
        while index >= 0 {
            let (function, other) = collision_function(e, base, index, dispatcher, 0x9ac);
            e.call(
                function,
                &args![base + 0x10, other, record, cast_collector, start_collector],
            );
            index = index.wrapping_sub(1);
        }
    });
}

/// The function the dispatcher `dispatcher` holds at `offset` (`0x9a4`, `0x9a8` or `0x9ac`) for
/// this phantom's shape type and the shape type of the collidable at `index` of the array at
/// +0x160: answers `(function, other collidable)`.
fn collision_function(
    e: &Engine,
    base: u32,
    index: i32,
    dispatcher: u32,
    offset: u32,
) -> (u32, u32) {
    let own_shape = e.mem.u32(base + 0x10);
    let own_type = e.mem.u32(own_shape + 0xc);
    let other = item_at(e, e.mem.u32(base + 0x160), index);
    let other_shape = e.mem.u32(other);
    let other_type = e.mem.u32(other_shape + 0xc);
    let cell = own_type
        .wrapping_add(0xd)
        .wrapping_shl(5)
        .wrapping_add(other_type);
    let kind = u32::from(e.mem.u8(dispatcher.wrapping_add(cell)));
    let function = e.mem.u32(
        dispatcher
            .wrapping_add(kind.wrapping_mul(20))
            .wrapping_add(offset),
    );
    (function, other)
}

// Translated from 00d1f450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleShapePhantom::setTransformAndLinearCast` (Xbox PDB), `(this, transform, input,
/// cast_collector, start_collector)`, `RET 0x10`: copies the three 16-byte vectors at
/// `transform`, `+0x10` and `+0x20` to `this + 0xb0`, `+0xc0` and `+0xd0`, then runs
/// `setPositionAndLinearCast` with the position at `transform + 0x30`.
pub fn hkp_simple_shape_phantom_set_transform_and_linear_cast(
    e: &mut Engine,
    this: Ptr,
    transform: Ptr,
    input: Ptr,
    cast_collector: u32,
    start_collector: u32,
) {
    let base = this.addr();
    for offset in [0u32, 0x10, 0x20] {
        copy_vec(e, transform.addr() + offset, base + 0xb0 + offset);
    }
    hkp_simple_shape_phantom_set_position_and_linear_cast(
        e,
        this,
        Ptr::new(transform.addr() + 0x30),
        input,
        cast_collector,
        start_collector,
    );
}

// Translated from 00d1f4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleShapePhantom::getClosestPoints` (Xbox PDB), `(this, collector, input)`, `RET 8`.
/// A null `input` becomes the world's collision input (the word at `world + 0x78`). For each
/// overlapping collidable (the array at +0x160, size +0x164) from the last to the first,
/// looks up the closest-points function for the pair of shape types in the dispatcher (the
/// first word of `input`: the byte at `((type A + 0xd) * 32 + type B)` selects the
/// five-word entry at `+0x9a8`) and calls it with `(this + 0x10, the other collidable, input,
/// collector)`, cdecl.
pub fn hkp_simple_shape_phantom_get_closest_points(
    e: &mut Engine,
    this: Ptr,
    collector: u32,
    input: u32,
) {
    let base = this.addr();
    let mut input = input;
    if input == 0 {
        let world = e.mem.u32(base + 8);
        input = e.mem.u32(world + 0x78);
    }
    let mut index = e.mem.i32(base + 0x164).wrapping_sub(1);
    let dispatcher = e.mem.u32(input);
    while index >= 0 {
        let (function, other) = collision_function(e, base, index, dispatcher, 0x9a8);
        e.call(function, &args![base + 0x10, other, input, collector]);
        index = index.wrapping_sub(1);
    }
}

// Translated from 00d1f520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleShapePhantom::getPenetrations` (Xbox PDB), `(this, collector, input)`, `RET 8`.
/// As `getClosestPoints` (null `input` becomes the world's collision input), with the entry at
/// `+0x9a4` and the call `(this + 0x10, the other collidable, input, collector)`; the loop
/// also ends as soon as the byte at `collector + 4` is not zero after a call.
pub fn hkp_simple_shape_phantom_get_penetrations(
    e: &mut Engine,
    this: Ptr,
    collector: u32,
    input: u32,
) {
    let base = this.addr();
    let mut input = input;
    if input == 0 {
        let world = e.mem.u32(base + 8);
        input = e.mem.u32(world + 0x78);
    }
    let mut index = e.mem.i32(base + 0x164).wrapping_sub(1);
    let dispatcher = e.mem.u32(input);
    while index >= 0 {
        let (function, other) = collision_function(e, base, index, dispatcher, 0x9a4);
        e.call(function, &args![base + 0x10, other, input, collector]);
        if e.mem.u8(collector + 4) != 0 {
            break;
        }
        index = index.wrapping_sub(1);
    }
}

// Translated from 00d1f5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleShapePhantom::removeOverlappingCollidable` (Xbox PDB), `(this, collidable)`:
/// nothing happens when the first word of `collidable` is zero. Otherwise looks for it in the
/// pointer array at +0x160 (size +0x164) from the last entry down; when it is not there,
/// `fireCollidableRemoved(collidable, flag 0)` is called; when it is, the same with flag `1`,
/// the size is decremented, the last entry moves into the freed place (unless it was the
/// last) and the byte at +0x16c is set. (The flag is pushed as a whole word whose low byte
/// is the `hkBool`: the other three bytes are those of `collidable`.)
pub fn hkp_simple_shape_phantom_remove_overlapping_collidable(
    e: &mut Engine,
    this: Ptr,
    collidable: u32,
) {
    let base = this.addr();
    if e.mem.u32(collidable) == 0 {
        return;
    }
    let table = e.mem.u32(base + 0x160);
    let mut index = e.mem.i32(base + 0x164).wrapping_sub(1);
    let mut found = false;
    while index >= 0 {
        if e.mem.u32(item_address(table, index)) == collidable {
            found = true;
            break;
        }
        index = index.wrapping_sub(1);
    }
    let flag = (collidable & 0xffff_ff00) | u32::from(found);
    e.call(FIRE_COLLIDABLE_REMOVED, &args![this, collidable, flag]);
    if !found {
        return;
    }
    let size = e.mem.u32(base + 0x164).wrapping_sub(1);
    e.mem.set_u32(base + 0x164, size);
    if size as i32 != index {
        let table = e.mem.u32(base + 0x160);
        let last = e.mem.u32(item_address(table, size as i32));
        e.mem.set_u32(item_address(table, index), last);
    }
    e.mem.set_u8(base + 0x16c, 1);
}

// Translated from 00d1f620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleShapePhantom::calcContentStatistics` (Xbox PDB), `(this, collector,
/// _unused_1)`, `RET 8`: runs `hkpPhantom::calcContentStatistics(collector, 0126902c)`, then,
/// when the capacity word at +0x168 is not negative or the byte at `collector + 0x10` is set,
/// reports the pointer array at +0x160 through the collector's slot `0x14` as `(3,
/// "OverlapPtr", data, size * 4, getAllocatedSize(data, (capacity & 0x3fffffff) * 4))`.
pub fn hkp_simple_shape_phantom_calc_content_statistics(
    e: &mut Engine,
    this: Ptr,
    collector: u32,
    _unused_1: u32,
) {
    let base = this.addr();
    e.call(
        PHANTOM_CALC_CONTENT_STATISTICS,
        &args![this, collector, PHANTOM_CLASS],
    );
    let capacity = e.mem.u32(base + 0x168);
    if (capacity as i32) < 0 && e.mem.u8(collector + 0x10) == 0 {
        return;
    }
    let data = e.mem.u32(base + 0x160);
    let allocated = e
        .call(
            GET_ALLOCATED_SIZE,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
        )
        .u32();
    let data = e.mem.u32(base + 0x160);
    let used = e.mem.u32(base + 0x164).wrapping_mul(4);
    e.vcall(
        collector,
        0x14,
        &args![3u32, OVERLAP_PTR_NAME, data, used, allocated],
    );
}

// Translated from 00d1f690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleShapePhantom::addOverlappingCollidable` (Xbox PDB), `(this, collidable)`: nothing
/// happens when the first word of `collidable` is zero or when
/// `hkpPhantom::fireCollidableAdded(collidable)` answers non-zero. Otherwise appends
/// `collidable` to the pointer array at +0x160 (size +0x164, capacity word +0x168), growing it
/// through `00c90940(allocator, array, 4)` when the size equals the capacity, and sets the
/// byte at +0x16c.
pub fn hkp_simple_shape_phantom_add_overlapping_collidable(
    e: &mut Engine,
    this: Ptr,
    collidable: u32,
) {
    let base = this.addr();
    if e.mem.u32(collidable) == 0 {
        return;
    }
    if e.call(FIRE_COLLIDABLE_ADDED, &args![this, collidable])
        .u32()
        != 0
    {
        return;
    }
    let array = base + 0x160;
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 4u32]);
    }
    let size = e.mem.u32(array + 4);
    let data = e.mem.u32(array);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
    e.mem
        .set_u32(data.wrapping_add(size.wrapping_mul(4)), collidable);
    e.mem.set_u8(base + 0x16c, 1);
}

// Translated from 00d1f6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleShapePhantom::ensureDeterministicOrder` (Xbox PDB): when the byte at +0x16c is
/// set, sorts the pointer array at +0x160 (size +0x164) by uid with the quick sort when it
/// holds more than one entry (the comparator word is `this` with its low byte cleared: the
/// exe pushes `ECX` and zeroes the low byte of the slot), then clears the byte.
pub fn hkp_simple_shape_phantom_ensure_deterministic_order(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u8(base + 0x16c) == 0 {
        return;
    }
    let size = e.mem.i32(base + 0x164);
    if size > 1 {
        let table = e.mem.u32(base + 0x160);
        hk_algorithm_quick_sort_recursive_collision_detail_order_by_uid(
            e,
            Ptr::new(table),
            0,
            size - 1,
            base & 0xffff_ff00,
        );
    }
    e.mem.set_u8(base + 0x16c, 0);
}

// Translated from 00d1f730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleShapePhantom::hkpSimpleShapePhantom` (Xbox PDB, unnamed in the map), `(this,
/// shape, transform, collision_filter_info)`, `RET 0xc`: runs the `hkpShapePhantom`
/// constructor (`00c9ea40`, `(shape, transform)`), then sets the vtable `010ce15c`, the empty
/// pointer array at +0x160 (data 0, size 0, capacity word `0x80000000`), clears the byte at
/// +0x16c and stores the last word at +0x2c. Returns `this`.
pub fn fn_00d1f730(
    e: &mut Engine,
    this: Ptr,
    shape: u32,
    transform: u32,
    collision_filter_info: u32,
) -> Ptr {
    let base = this.addr();
    e.call(SHAPE_PHANTOM_CONSTRUCT, &args![this, shape, transform]);
    e.mem.set_u32(base, SIMPLE_SHAPE_PHANTOM_VTABLE);
    e.mem.set_u32(base + 0x160, 0);
    e.mem.set_u32(base + 0x164, 0);
    e.mem.set_u32(base + 0x168, 0x8000_0000);
    e.mem.set_u8(base + 0x16c, 0);
    e.mem.set_u32(base + 0x2c, collision_filter_info);
    this
}

// Translated from 00d1f780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleShapePhantom::~hkpSimpleShapePhantom` (Xbox PDB): sets the vtable `010ce15c`,
/// empties the pointer array at +0x160 (size 0; when the capacity word +0x168 is not negative
/// the data goes back to the allocator, slot `0x10`, `(capacity & 0x3fffffff) * 4` bytes;
/// data cleared, capacity word `0x80000000`), then jumps to `hkpPhantom::~hkpPhantom`
/// (`00d08130`).
pub fn hkp_simple_shape_phantom_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, SIMPLE_SHAPE_PHANTOM_VTABLE);
    free_pointer_array(e, base + 0x160);
    e.call(PHANTOM_DESTRUCTOR, &args![this]);
}

/// The common tail of the destructor and `deallocateInternalArrays`: through the router's
/// allocator, empties the array `array` (data +0, size +4, capacity word +8).
fn free_pointer_array(e: &mut Engine, array: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(array + 8);
    e.mem.set_u32(array + 4, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(array);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(array, 0);
    e.mem.set_u32(array + 8, 0x8000_0000);
}

// Translated from 00d1f7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleShapePhantom::clone` (Xbox PDB): takes 0x170 bytes from the allocator (slot 4),
/// stores the size at +4, builds the copy with the constructor `(shape word at +0x10,
/// this + 0xb0, word at +0x2c)`, copies the two pointer arrays at +0x8c and +0x98 into it
/// (each grown first when the copy's capacity is below the source's size: the old block is
/// freed when its capacity word is not negative, a new one comes from allocator slot `0xc`
/// asking for `size * 4` bytes, and the capacity word becomes the granted byte count divided
/// by 4) and calls `hkpWorldObject::copyProperties(copy, this)` (`00ca92a0`). Returns the
/// copy.
pub fn hkp_simple_shape_phantom_clone(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![0x170u32]).u32();
    e.mem.set_u16(block + 4, 0x170);
    let shape = e.mem.u32(base + 0x10);
    let filter_info = e.mem.u32(base + 0x2c);
    let copy = fn_00d1f730(e, Ptr::new(block), shape, base + 0xb0, filter_info);
    copy_pointer_array(e, base + 0x8c, copy.addr() + 0x8c);
    copy_pointer_array(e, base + 0x98, copy.addr() + 0x98);
    e.call(COPY_PROPERTIES, &args![copy, this]);
    copy
}

/// Copies the pointer array `from` (data +0, size +4) into `to`, growing `to` when its
/// capacity (the capacity word at +8 masked with `0x3fffffff`, compared signed) is below the
/// source's size.
fn copy_pointer_array(e: &mut Engine, from: u32, to: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity_word = e.mem.u32(to + 8);
    let capacity = capacity_word & 0x3fff_ffff;
    let count = e.mem.u32(from + 4);
    if (capacity as i32) < (count as i32) {
        if (capacity_word as i32) >= 0 {
            let old = e.mem.u32(to);
            e.vcall(allocator, 0x10, &args![old, capacity.wrapping_mul(4)]);
        }
        let bytes = count.wrapping_mul(4);
        e.with_stack(4, |e, request| {
            e.mem.set_u32(request.addr(), bytes);
            let data = e.vcall(allocator, 0xc, &args![request]).u32();
            e.mem.set_u32(to, data);
            let granted = e.mem.i32(request.addr());
            e.mem.set_u32(to + 8, (granted / 4) as u32);
        });
    }
    let count = e.mem.u32(from + 4);
    let data = e.mem.u32(to);
    e.mem.set_u32(to + 4, count);
    let source = e.mem.u32(from);
    if (count as i32) > 0 {
        for index in 0..count {
            let word = e.mem.u32(source.wrapping_add(4 * index));
            e.mem.set_u32(data.wrapping_add(4 * index), word);
        }
    }
}

// Translated from 00d1f960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpSimpleShapePhantom::deallocateInternalArrays` (Xbox PDB): when the size at +0x164 is
/// zero, empties the pointer array at +0x160 as the destructor does (allocator slot `0x10`
/// when the capacity word is not negative, data cleared, capacity word `0x80000000`). Then
/// jumps to `hkpShapePhantom::deallocateInternalArrays` (`00c9e820`).
pub fn hkp_simple_shape_phantom_deallocate_internal_arrays(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x164) == 0 {
        free_pointer_array(e, base + 0x160);
    }
    e.call(SHAPE_PHANTOM_DEALLOCATE_INTERNAL_ARRAYS, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00cf_2ed0,
            hk_algorithm_quick_sort_recursive_collision_detail_order_by_uid(Ptr, i32, i32, u32)
        ),
        entry!(0x00d1_f220, hkp_simple_shape_phantom_get_type(Ptr) -> u32),
        entry!(
            0x00d1_f230,
            hkp_simple_shape_phantom_is_overlapping_collidable_added(Ptr, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00d1_f270,
            hkp_simple_shape_phantom_set_position_and_linear_cast(Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00d1_f450,
            hkp_simple_shape_phantom_set_transform_and_linear_cast(Ptr, Ptr, Ptr, u32, u32)
        ),
        entry!(
            0x00d1_f4a0,
            hkp_simple_shape_phantom_get_closest_points(Ptr, u32, u32)
        ),
        entry!(
            0x00d1_f520,
            hkp_simple_shape_phantom_get_penetrations(Ptr, u32, u32)
        ),
        entry!(
            0x00d1_f5a0,
            hkp_simple_shape_phantom_remove_overlapping_collidable(Ptr, u32)
        ),
        entry!(
            0x00d1_f620,
            hkp_simple_shape_phantom_calc_content_statistics(Ptr, u32, u32)
        ),
        entry!(
            0x00d1_f690,
            hkp_simple_shape_phantom_add_overlapping_collidable(Ptr, u32)
        ),
        entry!(
            0x00d1_f6f0,
            hkp_simple_shape_phantom_ensure_deterministic_order(Ptr)
        ),
        entry!(0x00d1_f730, fn_00d1f730(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00d1_f780, hkp_simple_shape_phantom_destructor(Ptr)),
        entry!(0x00d1_f7e0, hkp_simple_shape_phantom_clone(Ptr) -> Ptr),
        entry!(
            0x00d1_f960,
            hkp_simple_shape_phantom_deallocate_internal_arrays(Ptr)
        ),
        entry!(0x00d1_f9c0, deleting_destructor_00d1f9c0(Ptr, u8) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    /// Installs a base-destructor double at `base` and a memory router whose allocator
    /// frees through a double; both push their arguments (tagged 1 and 2) on the log.
    fn install(e: &mut Engine, base: u32) -> Rc<RefCell<Vec<u32>>> {
        let log = Rc::new(RefCell::new(Vec::new()));
        let base_log = Rc::clone(&log);
        e.register_double(base, move |_, a| {
            base_log.borrow_mut().extend([1, a[0]]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().extend([2, a[1], a[2]]);
            Ret::default()
        });
        log
    }

    /// Without bit 0 only the base destructor runs; with it the block is also freed with
    /// its stored size. Either way `this` is returned.
    fn check(address: u32, base: u32) {
        let mut e = engine();
        let log = install(&mut e, base);
        let this = e.mem.alloc(0x40);
        e.mem.set_u16(this + 4, 0x1234);
        let got = e.call(address, &args![this, 0xffff_fffeu32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this]);
        log.borrow_mut().clear();
        let got = e.call(address, &args![this, 1u32]).u32();
        assert_eq!(got, this);
        assert_eq!(*log.borrow(), vec![1, this, 2, this, 0x1234]);
    }

    #[test]
    fn deleting_destructor_00d1f9c0_frees_only_when_bit_zero_is_set() {
        check(0x00d1_f9c0, 0x00d1_f780);
    }

    type EventLog = Rc<RefCell<Vec<Vec<u32>>>>;

    fn new_log() -> EventLog {
        Rc::new(RefCell::new(Vec::new()))
    }

    /// A double at `address` that logs `[address, arguments...]` and answers `answer`.
    fn log_calls(e: &mut Engine, address: u32, log: &EventLog, answer: u32) {
        let log = Rc::clone(log);
        e.register_double(address, move |_, a| {
            let mut entry = vec![address];
            entry.extend_from_slice(a);
            log.borrow_mut().push(entry);
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    fn entries(log: &EventLog, address: u32) -> Vec<Vec<u32>> {
        log.borrow()
            .iter()
            .filter(|entry| entry[0] == address)
            .cloned()
            .collect()
    }

    const ALLOCATOR_VTABLE: u32 = 0x0900_0500;
    const ALLOCATE_SLOT: u32 = 0x0900_0504;
    const CHUNK_SLOT: u32 = 0x0900_050c;
    const FREE_SLOT: u32 = 0x0900_0510;

    /// A memory router whose allocator logs `[4, size]` (slot 4, answering a fresh block),
    /// `[0xc, requested]` (slot `0xc`, granting 8 bytes more) and `[0x10, data, size]`
    /// (slot `0x10`); answers the allocator.
    fn install_allocator(e: &mut Engine, log: &EventLog) -> u32 {
        e.put_vtable(
            ALLOCATOR_VTABLE,
            &[0, ALLOCATE_SLOT, 0, CHUNK_SLOT, FREE_SLOT],
        );
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let allocate_log = Rc::clone(log);
        e.register_double(ALLOCATE_SLOT, move |e, a| {
            allocate_log.borrow_mut().push(vec![4, a[1]]);
            Ret {
                eax: e.mem.alloc(a[1]),
                ..Ret::default()
            }
        });
        let chunk_log = Rc::clone(log);
        e.register_double(CHUNK_SLOT, move |e, a| {
            let requested = e.mem.u32(a[1]);
            chunk_log.borrow_mut().push(vec![0xc, requested]);
            e.mem.set_u32(a[1], requested + 8);
            Ret {
                eax: e.mem.alloc(requested + 8),
                ..Ret::default()
            }
        });
        let free_log = Rc::clone(log);
        e.register_double(FREE_SLOT, move |_, a| {
            free_log.borrow_mut().push(vec![0x10, a[1], a[2]]);
            Ret::default()
        });
        allocator
    }

    /// Records with the given uids at +0x14.
    fn records(e: &mut Engine, uids: &[u32]) -> Vec<u32> {
        uids.iter()
            .map(|uid| {
                let record = e.mem.alloc(0x20);
                e.mem.set_u32(record + 0x14, *uid);
                record
            })
            .collect()
    }

    /// A pointer array holding `items`.
    fn pointer_array(e: &mut Engine, items: &[u32]) -> u32 {
        let array = e.mem.alloc(4 * items.len() as u32 + 4);
        for (index, item) in items.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *item);
        }
        array
    }

    fn uids_of(e: &Engine, array: u32, count: u32) -> Vec<u32> {
        (0..count)
            .map(|index| e.mem.u32(e.mem.u32(array + 4 * index) + 0x14))
            .collect()
    }

    #[test]
    fn the_quick_sort_orders_by_uid_and_leaves_the_rest() {
        let mut e = engine();
        let uids = [5u32, 3, 9, 3, 1, 7, 7, 0, 8, 2];
        let items = records(&mut e, &uids);
        let array = pointer_array(&mut e, &items);
        e.call(0x00cf_2ed0, &args![array, 0i32, 9i32, 0u32]);
        let mut sorted = uids.to_vec();
        sorted.sort_unstable();
        assert_eq!(uids_of(&e, array, 10), sorted);
        // The same records are still there, each once.
        let mut kept: Vec<u32> = (0..10).map(|index| e.mem.u32(array + 4 * index)).collect();
        kept.sort_unstable();
        let mut expected = items.clone();
        expected.sort_unstable();
        assert_eq!(kept, expected);
        // A sub-range only touches itself.
        let items = records(&mut e, &[9, 8, 7, 6, 5, 4, 3]);
        let array = pointer_array(&mut e, &items);
        e.call(0x00cf_2ed0, &args![array, 2i32, 5i32, 0u32]);
        assert_eq!(uids_of(&e, array, 7), vec![9, 8, 4, 5, 6, 7, 3]);
    }

    #[test]
    fn the_phantom_type_is_one() {
        let mut e = engine();
        assert_eq!(e.call(0x00d1_f220, &args![0x1234u32]).u32(), 1);
    }

    #[test]
    fn an_added_collidable_is_found_in_the_array() {
        let mut e = engine();
        let this = e.mem.alloc(0x180);
        let array = pointer_array(&mut e, &[0x100, 0x200, 0x300]);
        e.mem.set_u32(this + 0x160, array);
        e.mem.set_u32(this + 0x164, 3);
        let result = e.mem.alloc(0x10);
        let got = e.call(0x00d1_f230, &args![this, result, 0x300u32]).u32();
        assert_eq!(got, result);
        assert_eq!(e.mem.u8(result), 1);
        e.call(0x00d1_f230, &args![this, result, 0x400u32]);
        assert_eq!(e.mem.u8(result), 0);
        e.mem.set_u32(this + 0x164, 0);
        e.mem.set_u8(result, 1);
        e.call(0x00d1_f230, &args![this, result, 0x100u32]);
        assert_eq!(e.mem.u8(result), 0);
    }

    const SHAPE_VTABLE: u32 = 0x0900_3100;
    const GET_AABB_DOUBLE: u32 = 0x0900_311c;
    /// The functions the dispatcher offers, `FUNCTIONS + kind * 0x10 + {0: penetrations,
    /// 1: closest points, 2: linear cast}`.
    const FUNCTIONS: u32 = 0x0900_4000;

    /// A phantom of shape type 1 overlapping two collidables of shape types 2 and 3 (the
    /// dispatcher maps them to functions of kinds 1 and 2), with a world whose collision
    /// input holds recognisable words. Returns `(this, collision input, dispatcher, others)`.
    fn phantom_scene(e: &mut Engine, log: &EventLog) -> (u32, u32, u32, [u32; 2]) {
        e.set_global(TOLERANCE_SCALE, 0.5f64);
        e.set_global(CAST_RECORD_EPSILON, 1.0e-7f32);
        let this = e.mem.alloc(0x180);
        let world = e.mem.alloc(0x100);
        let collision_input = e.mem.alloc(0x80);
        let dispatcher = e.mem.alloc(0xa00);
        e.mem.set_u32(this + 8, world);
        e.mem.set_u32(world + 0x78, collision_input);
        e.mem.set_u32(world + 0x80, dispatcher);
        e.mem.set_u32(collision_input, dispatcher);
        for (offset, word) in [
            (4u32, 0x11u32),
            (0xc, 0x13),
            (0x10, 0x14),
            (0x14, 0x15),
            (0x70, 0x77),
        ] {
            e.mem.set_u32(collision_input + offset, word);
        }
        e.mem.set_f32(collision_input + 8, 4.0);
        for lane in 0..4u32 {
            e.mem
                .set_f32(collision_input + 0x20 + 4 * lane, 1.0 + lane as f32);
            e.mem
                .set_f32(collision_input + 0x30 + 4 * lane, 5.0 + lane as f32);
            e.mem
                .set_f32(collision_input + 0x40 + 4 * lane, 9.0 + lane as f32);
        }
        // The shape: type 1, slot 0x1c fills the AABB.
        let shape = e.mem.alloc(0x20);
        e.put_vtable(SHAPE_VTABLE, &[0, 0, 0, 0, 0, 0, 0, GET_AABB_DOUBLE]);
        e.mem.set_u32(shape, SHAPE_VTABLE);
        e.mem.set_u32(shape + 0xc, 1);
        e.mem.set_u32(this + 0x10, shape);
        let get_aabb_log = Rc::clone(log);
        e.register_double(GET_AABB_DOUBLE, move |e, a| {
            get_aabb_log
                .borrow_mut()
                .push(vec![GET_AABB_DOUBLE, a[0], a[1], a[2]]);
            store_vec(e, a[3], [-1.0, -1.0, -1.0, 0.0]);
            store_vec(e, a[3] + 0x10, [1.0, 1.0, 1.0, 0.0]);
            Ret::default()
        });
        // The overlapping collidables, whose shapes have types 2 and 3.
        let mut others = [0u32; 2];
        for (index, shape_type) in [2u32, 3].into_iter().enumerate() {
            let other_shape = e.mem.alloc(0x20);
            e.mem.set_u32(other_shape + 0xc, shape_type);
            let other = e.mem.alloc(0x20);
            e.mem.set_u32(other, other_shape);
            others[index] = other;
        }
        let array = pointer_array(e, &others);
        e.mem.set_u32(this + 0x160, array);
        e.mem.set_u32(this + 0x164, 2);
        // Types (1, 2) select kind 1 and (1, 3) kind 2: cell = ((1 + 13) << 5) + type.
        e.mem.set_u8(dispatcher + 450, 1);
        e.mem.set_u8(dispatcher + 451, 2);
        for kind in 1..3u32 {
            for (slot, offset) in [(0u32, 0x9a4u32), (1, 0x9a8), (2, 0x9ac)] {
                let function = FUNCTIONS + kind * 0x10 + slot;
                e.mem.set_u32(dispatcher + kind * 20 + offset, function);
                log_calls(e, function, log, 0);
            }
        }
        (this, collision_input, dispatcher, others)
    }

    #[test]
    fn the_position_cast_widens_the_aabb_and_calls_the_cast_functions_back_to_front() {
        let mut e = engine();
        let log = new_log();
        let (this, collision_input, dispatcher, others) = phantom_scene(&mut e, &log);
        let aabb_log = Rc::clone(&log);
        e.register_double(UPDATE_BROAD_PHASE, move |e, a| {
            let mut entry = vec![UPDATE_BROAD_PHASE, a[0]];
            for vector in [a[1], a[1] + 0x10] {
                entry.extend(load_bits(e, vector));
            }
            aabb_log.borrow_mut().push(entry);
            Ret::default()
        });
        let position = e.mem.alloc(0x10);
        store_vec(&mut e, position, [1.0, 2.0, 3.0, 0.0]);
        let input = e.mem.alloc(0x20);
        store_vec(&mut e, input, [4.0, 2.0, 0.0, 0.0]);
        e.mem.set_f32(input + 0x10, 0.75);
        e.mem.set_f32(input + 0x14, 0.25);
        // The cast functions copy the record they are given.
        let record_log = new_log();
        for kind in 1..3u32 {
            let function = FUNCTIONS + kind * 0x10 + 2;
            let record_log = Rc::clone(&record_log);
            e.register_double(function, move |e, a| {
                let mut entry = vec![function, a[0], a[1], a[3], a[4]];
                entry.extend((0..0x1cu32).map(|word| e.mem.u32(a[2] + 4 * word)));
                record_log.borrow_mut().push(entry);
                Ret::default()
            });
        }
        e.call(
            0x00d1_f270,
            &args![this, position, input, 0xc0c0u32, 0x5757u32],
        );
        assert_eq!(load_vec(&e, this + 0xe0), [1.0, 2.0, 3.0, 0.0]);
        // Tolerance: 4.0 * 0.5 + 0.25 = 2.25, asked of the shape with this + 0xb0.
        let shape = e.mem.u32(this + 0x10);
        let aabb_requests = entries(&log, GET_AABB_DOUBLE);
        assert_eq!(aabb_requests.len(), 1);
        assert_eq!(
            aabb_requests[0][1..4],
            [shape, this + 0xb0, 2.25f32.to_bits()]
        );
        // The path (3, 0, -3, 0) widens the box (-1..1) to (-1, -1, -4)..(4, 1, 1).
        let broad = &entries(&log, UPDATE_BROAD_PHASE)[0];
        assert_eq!(broad[1], this);
        let widened: Vec<f32> = broad[2..10]
            .iter()
            .map(|bits| f32::from_bits(*bits))
            .collect();
        assert_eq!(widened, vec![-1.0, -1.0, -4.0, 0.0, 4.0, 1.0, 1.0, 0.0]);
        // The functions run for the last collidable first (kind 2), then the first (kind 1).
        let records = record_log.borrow();
        assert_eq!(records.len(), 2);
        assert_eq!(
            records[0][..5],
            [FUNCTIONS + 0x22, this + 0x10, others[1], 0xc0c0, 0x5757]
        );
        assert_eq!(
            records[1][..5],
            [FUNCTIONS + 0x12, this + 0x10, others[0], 0xc0c0, 0x5757]
        );
        let record = &records[0][5..];
        assert_eq!(record[..2], [dispatcher, 0x11]);
        // The word at +8 is the float at input + 0x14, not the collision input's.
        assert_eq!(f32::from_bits(record[2]), 0.25);
        assert_eq!(record[3..6], [0x13, 0x14, 0x15]);
        assert_eq!(f32::from_bits(record[8]), 1.0);
        assert_eq!(f32::from_bits(record[12]), 5.0);
        assert_eq!(f32::from_bits(record[16]), 9.0);
        let path: Vec<f32> = record[0x14..0x18]
            .iter()
            .map(|bits| f32::from_bits(*bits))
            .collect();
        assert_eq!(path, vec![3.0, 0.0, -3.0, 0.0]);
        assert_eq!(f32::from_bits(record[0x18]), 0.75);
        assert!((f32::from_bits(record[0x19]) - 18.0f32.sqrt()).abs() < 1e-6);
        assert_eq!(record[0x1a], 0x77);
        assert_eq!(e.mem.u32(collision_input + 8), 4.0f32.to_bits());
    }

    #[test]
    fn the_transform_cast_stores_the_transform_and_casts_to_its_position() {
        let mut e = engine();
        let log = new_log();
        let (this, _, _, _) = phantom_scene(&mut e, &log);
        e.mem.set_u32(this + 0x164, 0);
        log_calls(&mut e, UPDATE_BROAD_PHASE, &log, 0);
        let transform = e.mem.alloc(0x40);
        for row in 0..4u32 {
            store_vec(&mut e, transform + 0x10 * row, [row as f32, 1.0, 2.0, 3.0]);
        }
        let input = e.mem.alloc(0x20);
        store_vec(&mut e, input, [1.0, 2.0, 3.0, 0.0]);
        e.call(0x00d1_f450, &args![this, transform, input, 0x1u32, 0x2u32]);
        for row in 0..3u32 {
            assert_eq!(
                load_bits(&e, this + 0xb0 + 0x10 * row),
                load_bits(&e, transform + 0x10 * row)
            );
        }
        // The position is the fourth row.
        assert_eq!(load_vec(&e, this + 0xe0), [3.0, 1.0, 2.0, 3.0]);
        assert_eq!(entries(&log, GET_AABB_DOUBLE).len(), 1);
    }

    #[test]
    fn closest_points_and_penetrations_use_their_own_dispatcher_entries() {
        let mut e = engine();
        let log = new_log();
        let (this, collision_input, _, others) = phantom_scene(&mut e, &log);
        // With a null input the world's collision input is used.
        e.call(0x00d1_f4a0, &args![this, 0xc011u32, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![
                    FUNCTIONS + 0x21,
                    this + 0x10,
                    others[1],
                    collision_input,
                    0xc011
                ],
                vec![
                    FUNCTIONS + 0x11,
                    this + 0x10,
                    others[0],
                    collision_input,
                    0xc011
                ],
            ]
        );
        log.borrow_mut().clear();
        // Penetrations: the byte at collector + 4 ends the loop after the first call.
        let collector = e.mem.alloc(0x10);
        let stop = Rc::new(std::cell::Cell::new(1u8));
        for kind in 1..3u32 {
            let function = FUNCTIONS + kind * 0x10;
            let function_log = Rc::clone(&log);
            let stop = Rc::clone(&stop);
            e.register_double(function, move |e, a| {
                function_log
                    .borrow_mut()
                    .push(vec![function, a[0], a[1], a[2], a[3]]);
                e.mem.set_u8(a[3] + 4, stop.get());
                Ret::default()
            });
        }
        e.call(0x00d1_f520, &args![this, collector, collision_input]);
        assert_eq!(
            *log.borrow(),
            vec![vec![
                FUNCTIONS + 0x20,
                this + 0x10,
                others[1],
                collision_input,
                collector
            ]]
        );
        log.borrow_mut().clear();
        stop.set(0);
        e.mem.set_u8(collector + 4, 0);
        e.call(0x00d1_f520, &args![this, collector, 0u32]);
        assert_eq!(log.borrow().len(), 2);
    }

    #[test]
    fn removing_a_collidable_fires_the_event_and_swaps_in_the_last() {
        let mut e = engine();
        let log = new_log();
        log_calls(&mut e, FIRE_COLLIDABLE_REMOVED, &log, 0);
        let this = e.mem.alloc(0x180);
        let collidables: Vec<u32> = (0..4)
            .map(|_| {
                let collidable = e.mem.alloc(0x10);
                e.mem.set_u32(collidable, 0x7000);
                collidable
            })
            .collect();
        let array = pointer_array(&mut e, &collidables[..3]);
        e.mem.set_u32(this + 0x160, array);
        e.mem.set_u32(this + 0x164, 3);
        // A collidable whose first word is zero is ignored.
        let blank = e.mem.alloc(0x10);
        e.call(0x00d1_f5a0, &args![this, blank]);
        assert!(log.borrow().is_empty());
        // One that is not in the array is reported with flag 0.
        e.call(0x00d1_f5a0, &args![this, collidables[3]]);
        assert_eq!(
            *log.borrow(),
            vec![vec![
                FIRE_COLLIDABLE_REMOVED,
                this,
                collidables[3],
                collidables[3] & 0xffff_ff00
            ]]
        );
        assert_eq!(e.mem.u32(this + 0x164), 3);
        assert_eq!(e.mem.u8(this + 0x16c), 0);
        log.borrow_mut().clear();
        // The first entry is replaced by the last.
        e.call(0x00d1_f5a0, &args![this, collidables[0]]);
        assert_eq!(
            *log.borrow(),
            vec![vec![
                FIRE_COLLIDABLE_REMOVED,
                this,
                collidables[0],
                (collidables[0] & 0xffff_ff00) | 1
            ]]
        );
        assert_eq!(e.mem.u32(this + 0x164), 2);
        assert_eq!(e.mem.u32(array), collidables[2]);
        assert_eq!(e.mem.u32(array + 4), collidables[1]);
        assert_eq!(e.mem.u8(this + 0x16c), 1);
        // The last entry is just dropped.
        e.call(0x00d1_f5a0, &args![this, collidables[1]]);
        assert_eq!(e.mem.u32(this + 0x164), 1);
        assert_eq!(e.mem.u32(array), collidables[2]);
    }

    #[test]
    fn the_statistics_report_the_overlap_array_when_it_owns_memory_or_verbose() {
        let mut e = engine();
        let log = new_log();
        log_calls(&mut e, PHANTOM_CALC_CONTENT_STATISTICS, &log, 0);
        e.register_double(GET_ALLOCATED_SIZE, |_, a| Ret {
            eax: 0x1000 + a[1],
            ..Ret::default()
        });
        e.put_vtable(0x0900_3200, &[0, 0, 0, 0, 0, 0x0900_3214]);
        let report_log = Rc::clone(&log);
        e.register_double(0x0900_3214, move |_, a| {
            let mut entry = vec![0x14];
            entry.extend_from_slice(a);
            report_log.borrow_mut().push(entry);
            Ret::default()
        });
        let collector = e.mem.alloc(0x20);
        e.mem.set_u32(collector, 0x0900_3200);
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this + 0x160, 0x5000);
        e.mem.set_u32(this + 0x164, 3);
        e.mem.set_u32(this + 0x168, 0x4000_0005);
        e.call(0x00d1_f620, &args![this, collector, 0u32]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![
                    PHANTOM_CALC_CONTENT_STATISTICS,
                    this,
                    collector,
                    PHANTOM_CLASS
                ],
                vec![
                    0x14,
                    collector,
                    3,
                    OVERLAP_PTR_NAME,
                    0x5000,
                    12,
                    0x1000 + 20
                ],
            ]
        );
        // A capacity word with the top bit set is skipped unless the collector is verbose.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x168, 0x8000_0005);
        e.call(0x00d1_f620, &args![this, collector, 0u32]);
        assert_eq!(log.borrow().len(), 1);
        e.mem.set_u8(collector + 0x10, 1);
        e.call(0x00d1_f620, &args![this, collector, 0u32]);
        assert_eq!(
            log.borrow().last().unwrap(),
            &vec![
                0x14,
                collector,
                3,
                OVERLAP_PTR_NAME,
                0x5000,
                12,
                0x1000 + 20
            ]
        );
    }

    #[test]
    fn adding_a_collidable_appends_it_unless_the_event_vetoes() {
        let mut e = engine();
        let log = new_log();
        let allocator = install_allocator(&mut e, &log);
        let veto = Rc::new(std::cell::Cell::new(1u32));
        let event_veto = Rc::clone(&veto);
        e.register_double(FIRE_COLLIDABLE_ADDED, move |_, _| Ret {
            eax: event_veto.get(),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x180);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0x160, data);
        e.mem.set_u32(this + 0x168, 0x4000_0002);
        let collidable = e.mem.alloc(0x10);
        e.mem.set_u32(collidable, 0x7000);
        let blank = e.mem.alloc(0x10);
        e.call(0x00d1_f690, &args![this, blank]);
        e.call(0x00d1_f690, &args![this, collidable]);
        assert_eq!(e.mem.u32(this + 0x164), 0);
        assert_eq!(e.mem.u8(this + 0x16c), 0);
        veto.set(0);
        e.call(0x00d1_f690, &args![this, collidable]);
        assert_eq!(e.mem.u32(this + 0x164), 1);
        assert_eq!(e.mem.u32(data), collidable);
        assert_eq!(e.mem.u8(this + 0x16c), 1);
        // A full array is grown first.
        e.mem.set_u32(this + 0x164, 2);
        let grown = e.mem.alloc(0x40);
        let reserve_log = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE_MORE, move |e, a| {
            reserve_log
                .borrow_mut()
                .push(vec![ARRAY_RESERVE_MORE, a[0], a[1], a[2]]);
            e.mem.set_u32(a[1], grown);
            e.mem.set_u32(a[1] + 8, 0x4000_0008);
            Ret::default()
        });
        e.call(0x00d1_f690, &args![this, collidable]);
        assert_eq!(
            entries(&log, ARRAY_RESERVE_MORE),
            vec![vec![ARRAY_RESERVE_MORE, allocator, this + 0x160, 4]]
        );
        assert_eq!(e.mem.u32(this + 0x164), 3);
        assert_eq!(e.mem.u32(grown + 8), collidable);
    }

    #[test]
    fn deterministic_order_sorts_once_when_marked() {
        let mut e = engine();
        let this = e.mem.alloc(0x180);
        let items = records(&mut e, &[30, 10, 20]);
        let array = pointer_array(&mut e, &items);
        e.mem.set_u32(this + 0x160, array);
        e.mem.set_u32(this + 0x164, 3);
        e.call(0x00d1_f6f0, &args![this]);
        assert_eq!(uids_of(&e, array, 3), vec![30, 10, 20]);
        e.mem.set_u8(this + 0x16c, 1);
        e.call(0x00d1_f6f0, &args![this]);
        assert_eq!(uids_of(&e, array, 3), vec![10, 20, 30]);
        assert_eq!(e.mem.u8(this + 0x16c), 0);
        // A single entry is not sorted but the mark is cleared.
        e.mem.set_u8(this + 0x16c, 1);
        e.mem.set_u32(this + 0x164, 1);
        e.call(0x00d1_f6f0, &args![this]);
        assert_eq!(e.mem.u8(this + 0x16c), 0);
    }

    #[test]
    fn the_constructor_builds_the_shape_phantom_then_the_empty_array() {
        let mut e = engine();
        let log = new_log();
        log_calls(&mut e, SHAPE_PHANTOM_CONSTRUCT, &log, 0);
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this + 0x164, 9);
        e.mem.set_u8(this + 0x16c, 1);
        let got = e
            .call(0x00d1_f730, &args![this, 0x11u32, 0x22u32, 0x33u32])
            .u32();
        assert_eq!(got, this);
        assert_eq!(
            *log.borrow(),
            vec![vec![SHAPE_PHANTOM_CONSTRUCT, this, 0x11, 0x22]]
        );
        assert_eq!(e.mem.u32(this), SIMPLE_SHAPE_PHANTOM_VTABLE);
        assert_eq!(e.mem.u32(this + 0x160), 0);
        assert_eq!(e.mem.u32(this + 0x164), 0);
        assert_eq!(e.mem.u32(this + 0x168), 0x8000_0000);
        assert_eq!(e.mem.u8(this + 0x16c), 0);
        assert_eq!(e.mem.u32(this + 0x2c), 0x33);
    }

    #[test]
    fn the_destructor_frees_an_owned_array_and_runs_the_phantom_destructor() {
        let mut e = engine();
        let log = new_log();
        install_allocator(&mut e, &log);
        log_calls(&mut e, PHANTOM_DESTRUCTOR, &log, 0);
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this + 0x160, 0x5000);
        e.mem.set_u32(this + 0x164, 3);
        e.mem.set_u32(this + 0x168, 0x4000_0006);
        e.call(0x00d1_f780, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![vec![0x10, 0x5000, 24], vec![PHANTOM_DESTRUCTOR, this]]
        );
        assert_eq!(e.mem.u32(this), SIMPLE_SHAPE_PHANTOM_VTABLE);
        assert_eq!(e.mem.u32(this + 0x160), 0);
        assert_eq!(e.mem.u32(this + 0x164), 0);
        assert_eq!(e.mem.u32(this + 0x168), 0x8000_0000);
        // Memory that is not owned is not freed.
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x160, 0x5000);
        e.mem.set_u32(this + 0x168, 0x8000_0006);
        e.call(0x00d1_f780, &args![this]);
        assert_eq!(*log.borrow(), vec![vec![PHANTOM_DESTRUCTOR, this]]);
    }

    #[test]
    fn cloning_copies_the_two_listener_arrays() {
        let mut e = engine();
        let log = new_log();
        install_allocator(&mut e, &log);
        log_calls(&mut e, COPY_PROPERTIES, &log, 0);
        // The constructor leaves the copy's first array without memory of its own and its
        // second one with room for a few words.
        let spare = e.mem.alloc(0x20);
        let construct_log = Rc::clone(&log);
        e.register_double(SHAPE_PHANTOM_CONSTRUCT, move |e, a| {
            construct_log
                .borrow_mut()
                .push(vec![SHAPE_PHANTOM_CONSTRUCT, a[0], a[1], a[2]]);
            e.mem.set_u32(a[0] + 0x94, 0x8000_0000);
            e.mem.set_u32(a[0] + 0x98, spare);
            e.mem.set_u32(a[0] + 0xa0, 0x4000_0008);
            Ret::default()
        });
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this + 0x10, 0xabc0);
        e.mem.set_u32(this + 0x2c, 0x1234);
        let first = pointer_array(&mut e, &[0x11, 0x22]);
        e.mem.set_u32(this + 0x8c, first);
        e.mem.set_u32(this + 0x90, 2);
        let second = pointer_array(&mut e, &[0x33]);
        e.mem.set_u32(this + 0x98, second);
        e.mem.set_u32(this + 0x9c, 1);
        let copy = e.call(0x00d1_f7e0, &args![this]).u32();
        let calls = log.borrow();
        assert_eq!(calls[0], vec![4, 0x170]);
        assert_eq!(
            calls[1],
            vec![SHAPE_PHANTOM_CONSTRUCT, copy, 0xabc0, this + 0xb0]
        );
        assert_eq!(e.mem.u16(copy + 4), 0x170);
        // First array: no memory of its own to free, 8 bytes asked for and 16 granted.
        assert_eq!(calls[2], vec![0xc, 8]);
        assert_eq!(e.mem.u32(copy + 0x94), 4);
        assert_eq!(e.mem.u32(copy + 0x90), 2);
        let data = e.mem.u32(copy + 0x8c);
        assert_eq!(e.mem.u32(data), 0x11);
        assert_eq!(e.mem.u32(data + 4), 0x22);
        // Second array: it fits, so it is only filled.
        assert_eq!(e.mem.u32(copy + 0x98), spare);
        assert_eq!(e.mem.u32(copy + 0x9c), 1);
        assert_eq!(e.mem.u32(spare), 0x33);
        assert_eq!(calls[3], vec![COPY_PROPERTIES, copy, this]);
        assert_eq!(calls.len(), 4);
    }

    #[test]
    fn a_cloned_array_with_an_owned_small_buffer_frees_it_first() {
        let mut e = engine();
        let log = new_log();
        install_allocator(&mut e, &log);
        log_calls(&mut e, COPY_PROPERTIES, &log, 0);
        e.register_double(SHAPE_PHANTOM_CONSTRUCT, move |e, a| {
            e.mem.set_u32(a[0] + 0x8c, 0x6000);
            e.mem.set_u32(a[0] + 0x94, 0x4000_0001);
            Ret::default()
        });
        let this = e.mem.alloc(0x180);
        let first = pointer_array(&mut e, &[0x11, 0x22, 0x33]);
        e.mem.set_u32(this + 0x8c, first);
        e.mem.set_u32(this + 0x90, 3);
        let copy = e.call(0x00d1_f7e0, &args![this]).u32();
        assert_eq!(entries(&log, 0x10), vec![vec![0x10, 0x6000, 4]]);
        assert_eq!(e.mem.u32(copy + 0x94), 5);
    }

    #[test]
    fn deallocating_only_empties_the_array_when_it_holds_nothing() {
        let mut e = engine();
        let log = new_log();
        install_allocator(&mut e, &log);
        log_calls(&mut e, SHAPE_PHANTOM_DEALLOCATE_INTERNAL_ARRAYS, &log, 0);
        let this = e.mem.alloc(0x180);
        e.mem.set_u32(this + 0x160, 0x5000);
        e.mem.set_u32(this + 0x164, 2);
        e.mem.set_u32(this + 0x168, 0x4000_0004);
        e.call(0x00d1_f960, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![vec![SHAPE_PHANTOM_DEALLOCATE_INTERNAL_ARRAYS, this]]
        );
        assert_eq!(e.mem.u32(this + 0x160), 0x5000);
        log.borrow_mut().clear();
        e.mem.set_u32(this + 0x164, 0);
        e.call(0x00d1_f960, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                vec![0x10, 0x5000, 16],
                vec![SHAPE_PHANTOM_DEALLOCATE_INTERNAL_ARRAYS, this]
            ]
        );
        assert_eq!(e.mem.u32(this + 0x160), 0);
        assert_eq!(e.mem.u32(this + 0x168), 0x8000_0000);
    }
}
