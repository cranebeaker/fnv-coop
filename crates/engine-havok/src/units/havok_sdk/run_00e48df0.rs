//! `Havok SDK/run_00e48df0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{load_vec, maxps, minps, store_vec};

/// `00c85750` (hkMemoryRouter lookup): no arguments, the router in `EAX`; its `+0x10` is the heap
/// allocator object.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserveMore` (Xbox PDB, `00c90940`): cdecl, `(allocator, array, element size)`.
const RESERVE_MORE: u32 = 0x00c9_0940;
/// `hkMapBase<unsigned long, unsigned long>::reserve` (Xbox PDB, `00cb1fe0`): thiscall on the
/// map, `(allocator, count)`.
const MAP_RESERVE: u32 = 0x00cb_1fe0;
/// `hkMapBase<..>::findKey` (Xbox PDB, `00cb18f0`): thiscall on the map, one word (the key);
/// the cell index, or `hashMod + 1` when the key is not in the map.
const MAP_FIND_KEY: u32 = 0x00cb_18f0;
/// `00cb1870`: thiscall on the map, `(allocator, key, value)`, inserts a new cell.
const MAP_INSERT: u32 = 0x00cb_1870;
/// `hkMapBase<..>::clearAndDeallocate` (Xbox PDB, `00cb1a90`): thiscall on the map, one word
/// (the allocator).
const MAP_CLEAR_AND_DEALLOCATE: u32 = 0x00cb_1a90;
/// `00a29680`: thiscall on the map, no stack words, does nothing (its destructor).
const MAP_DESTRUCTOR: u32 = 0x00a2_9680;
/// `hkReferencedObject::removeReference` (Xbox PDB, `00c905b0`): thiscall, `this` only.
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `00e607c0`: thiscall on the buffer to build (`this` returned), one `float`, `RET 4`.
const BUILD_SHAPE_OBJECT: u32 = 0x00e6_07c0;
/// The `float`s every lane of the box minimum and maximum start from (`0104da48`, `0104e148`) and
/// the `float` in lane 3 (`01011d78`).
const BOX_MINIMUM_START: u32 = 0x0104_da48;
const BOX_MAXIMUM_START: u32 = 0x0104_e148;
const LANE_THREE: u32 = 0x0101_1d78;

/// The heap allocator object of the memory router (`00c85750`, then `+0x10`).
fn heap_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    e.mem.u32(router.wrapping_add(0x10))
}

/// Appends one element of `element_size` bytes to the `hkArray` at `array` (`data`, `size`,
/// `capacity | flags`): the router is asked first, a full array (size equal to the capacity
/// bits) is grown through `00c90940`, and the size is incremented. Returns the address of the new
/// element.
fn push_slot(e: &mut Engine, array: u32, element_size: u32) -> u32 {
    let allocator = heap_allocator(e);
    let capacity = e.mem.u32(array.wrapping_add(8)) & 0x3fff_ffff;
    if e.mem.u32(array.wrapping_add(4)) == capacity {
        e.call(RESERVE_MORE, &args![allocator, array, element_size]);
    }
    let size = e.mem.u32(array.wrapping_add(4));
    let slot = e
        .mem
        .u32(array)
        .wrapping_add(size.wrapping_mul(element_size));
    e.mem.set_u32(array.wrapping_add(4), size.wrapping_add(1));
    slot
}

// Translated from 00e48df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 8`: appends a record. The byte array at `this + 0x14` (`data`, `size`,
/// `capacity | flags`) gets the low byte of `count`; then the `u16` array at `this + 8` gets the
/// low 16 bits of the first word of each of the `count` pairs of words at `items` (the element
/// stride is 4 bytes). Both arrays grow through `00c90940` when full.
pub fn fn_00e48df0(e: &mut Engine, this: Ptr, items: u32, count: i32) {
    let this = this.addr();
    let slot = push_slot(e, this.wrapping_add(0x14), 1);
    e.mem.set_u8(slot, count as u8);
    for index in 0..count.max(0) as u32 {
        let value = e.mem.u16(items.wrapping_add(index.wrapping_mul(4)));
        let slot = push_slot(e, this.wrapping_add(8), 2);
        e.mem.set_u16(slot, value);
    }
}

/// The edge check of `00e48e80` over the polygons of `this` and the map at `map`: `false` as soon
/// as an edge is seen twice in the same direction.
fn edges_are_paired(e: &mut Engine, this: u32, map: u32) -> bool {
    let count = e.mem.i32(this.wrapping_add(0x18));
    let mut offset = 0u32;
    for polygon in 0..count.max(0) as u32 {
        let length = u32::from(
            e.mem
                .u8(e.mem.u32(this.wrapping_add(0x14)).wrapping_add(polygon)),
        );
        let base = e
            .mem
            .u32(this.wrapping_add(8))
            .wrapping_add(offset.wrapping_mul(2));
        let mut previous = u32::from(
            e.mem
                .u16(base.wrapping_add(length.wrapping_mul(2)).wrapping_sub(2)),
        );
        for vertex in 0..length {
            let current = u32::from(e.mem.u16(base.wrapping_add(vertex.wrapping_mul(2))));
            let (key, direction) = if previous < current {
                ((previous << 16) | current, 2)
            } else {
                ((current << 16) | previous, 1)
            };
            let key = key.wrapping_add(1);
            let index = e.call(MAP_FIND_KEY, &args![map, key]).u32() as i32;
            if index <= e.mem.i32(map.wrapping_add(8)) {
                let cell = e.mem.u32(map).wrapping_add((index as u32).wrapping_mul(8));
                let value = e.mem.u32(cell.wrapping_add(4));
                if direction & value != 0 {
                    return false;
                }
                e.mem.set_u32(cell.wrapping_add(4), value | direction);
            } else {
                let allocator = heap_allocator(e);
                e.call(MAP_INSERT, &args![map, allocator, key, direction]);
            }
            previous = current;
        }
        offset = offset.wrapping_add(length);
    }
    let hash_mod = e.mem.i32(map.wrapping_add(8));
    if hash_mod < 0 {
        return true;
    }
    let data = e.mem.u32(map);
    let key_at =
        |e: &Engine, index: i32| e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(8)));
    let mut index = 0i32;
    while index <= hash_mod && key_at(e, index) == u32::MAX {
        index += 1;
    }
    loop {
        if index > hash_mod {
            return true;
        }
        let cell = data.wrapping_add((index as u32).wrapping_mul(8));
        if e.mem.u32(cell.wrapping_add(4)) != 3 {
            return false;
        }
        index += 1;
        while index <= hash_mod && key_at(e, index) == u32::MAX {
            index += 1;
        }
    }
}

// Translated from 00e48e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4`: writes a byte to `out` and returns `out`. Checks that the polygons of
/// `this` (`[+0x18]` of them; the byte at `*(this + 0x14) + i` is polygon `i`'s vertex count and
/// the `u16` vertices follow one after the other at `*(this + 8)`) pair up every edge: an edge's
/// key is `(low << 16 | high) + 1` of its two vertices, and an `hkMap` on the stack collects a
/// direction flag per edge (2 when the edge runs from the lower to the higher vertex, else 1;
/// the polygon's first edge runs from its last vertex). The byte is 0 when an edge is seen twice
/// in the same direction or when, at the end, an edge's flags are not 3; otherwise 1. The map is
/// then cleared through the allocator and its destructor (`00a29680`) runs.
pub fn fn_00e48e80(e: &mut Engine, this: Ptr, out: u32) -> u32 {
    let this = this.addr();
    e.with_stack(0x10, |e, map| {
        let map = map.addr();
        e.mem.set_u32(map, 0);
        e.mem.set_u32(map.wrapping_add(4), 0);
        e.mem.set_u32(map.wrapping_add(8), u32::MAX);
        let allocator = heap_allocator(e);
        e.call(MAP_RESERVE, &args![map, allocator, 0u32]);
        let paired = edges_are_paired(e, this, map);
        e.mem.set_u8(out, u8::from(paired));
        let allocator = heap_allocator(e);
        e.call(MAP_CLEAR_AND_DEALLOCATE, &args![map, allocator]);
        e.call(MAP_DESTRUCTOR, &args![map]);
    });
    out
}

// Translated from 00e49030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Virtual, no stack words: returns 0.
pub fn fn_00e49030(_e: &mut Engine, _this: Ptr) -> u32 {
    0
}

// Translated from 00e49040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall (`this` is not read), `RET 0xc`: writes to `out` (a byte) whether bit `index` is set
/// in a bit field of 31 bits per word: with `bits` pointing at `{ n, ... }`, the word is at
/// `bits + 4 * (n + index / 31) + 4` and the bit is `index % 31`.
/// Answers `out` (`EAX` at the `RET`; the caller tests the flag through it, 00e491ed).
pub fn fn_00e49040(e: &mut Engine, _this: Ptr, out: u32, bits: u32, index: u32) -> u32 {
    let word_index = e.mem.u32(bits).wrapping_add(index / 31);
    let bit = index % 31;
    let word = e.mem.u32(
        bits.wrapping_add(word_index.wrapping_mul(4))
            .wrapping_add(4),
    );
    e.mem.set_u8(out, u8::from(word & (1 << bit) != 0));
    out
}

// Translated from 00e490d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall destructor: stores the vtables `010e8e94` at `+0` and `010e8e74` at `+0x10`, drops
/// the references on the objects at `+0x1c` and `+0x18` (`00c905b0`, with no null check: the exe
/// faults on a null pointer), then stores `010c6fb0` at `+0x10` and `0102e388` at `+0`.
pub fn fn_00e490d0(e: &mut Engine, this: Ptr) {
    let this = this.addr();
    let first = e.mem.u32(this.wrapping_add(0x1c));
    e.mem.set_u32(this, 0x010e_8e94);
    e.mem.set_u32(this.wrapping_add(0x10), 0x010e_8e74);
    e.call(REMOVE_REFERENCE, &args![first]);
    let second = e.mem.u32(this.wrapping_add(0x18));
    e.call(REMOVE_REFERENCE, &args![second]);
    e.mem.set_u32(this.wrapping_add(0x10), 0x010c_6fb0);
    e.mem.set_u32(this, 0x0102_e388);
}

// Translated from 00e49100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4` (virtual): the next index, `index + 1`, or `-1` when that is not below the
/// sum of the words at `+0x18` and `+0x24` of the object at `*(this + 8)`.
pub fn fn_00e49100(e: &mut Engine, this: Ptr, index: i32) -> i32 {
    let owner = e.mem.u32(this.addr().wrapping_add(8));
    let total = e
        .mem
        .i32(owner.wrapping_add(0x24))
        .wrapping_add(e.mem.i32(owner.wrapping_add(0x18)));
    let next = index.wrapping_add(1);
    if next >= total {
        -1
    } else {
        next
    }
}

// Translated from 00e49120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 8` (virtual): builds the shape of entry `index` in the buffer `buffer` and
/// returns it (`00e607c0` makes it, with the `float` at `this + 0x10`; a null buffer makes the
/// exe store through address 0x20, which the memory model stops). With `owner` the object at
/// `*(this + 8)`: `+0x20` of the shape gets the word at `this + 0xc`. The vertices go to the
/// 16-byte aligned address `(buffer + 0x3b) & !0xf` (stored at `+0x18`). An `index` below the
/// polygon count `[owner + 0x18]` names the record `[owner + 0x14][index]` of the words at
/// `*(owner + 8)`: `{ n, n items, bit words }`; for each item (`+0x28` is `n`, `+0x24` the
/// item list, `+0x1c` counts the vertices kept) the object at `*(this + 0xc)` is asked
/// (virtual slot `0x14` of its sub-object at `+0x10`, with the item and a 0x200-byte buffer) for
/// a triangle whose three vertices (at `+0x20`) are kept when the matching bit (`3 * i + j`,
/// `00e49040`) of the record is set. A larger `index` names item `index - count` of
/// `*(owner + 0x20)` and keeps all three vertices (`+0x28` is 1, `+0x1c` is 3).
pub fn fn_00e49120(e: &mut Engine, this: Ptr, index: u32, buffer: u32) -> u32 {
    let this = this.addr();
    let shape = if buffer == 0 {
        0
    } else {
        let radius = x87_quiet(e.mem.f32(this.wrapping_add(0x10)));
        e.call(BUILD_SHAPE_OBJECT, &args![buffer, radius]).u32()
    };
    let source = e.mem.u32(this.wrapping_add(0xc));
    e.mem.set_u32(shape.wrapping_add(0x20), source);
    let owner = e.mem.u32(this.wrapping_add(8));
    let sub_object = source.wrapping_add(0x10);
    let polygons = e.mem.u32(owner.wrapping_add(0x18));
    e.with_stack(0x200, |e, scratch| {
        let scratch = scratch.addr();
        if index < polygons {
            let numbers = e.mem.u32(owner.wrapping_add(0x14));
            let number = e.mem.u32(numbers.wrapping_add(index.wrapping_mul(4)));
            let words = e.mem.u32(owner.wrapping_add(8));
            let record = words.wrapping_add(number.wrapping_mul(4));
            let item_count = e.mem.i32(record);
            e.mem
                .set_u32(shape.wrapping_add(0x24), record.wrapping_add(4));
            let mut cursor = buffer.wrapping_add(0x3b) & 0xffff_fff0;
            e.mem.set_u32(shape.wrapping_add(0x18), cursor);
            e.mem.set_i32(shape.wrapping_add(0x28), item_count);
            e.mem.set_u32(shape.wrapping_add(0x1c), 0);
            let mut bit_base = 0u32;
            for item in 0..item_count.max(0) as u32 {
                let items = e.mem.u32(shape.wrapping_add(0x24));
                let id = e.mem.u32(items.wrapping_add(item.wrapping_mul(4)));
                let object = e.vcall(sub_object, 0x14, &args![id, scratch]).u32();
                for vertex in 0..3u32 {
                    let kept = e.with_stack(4, |e, flag| {
                        fn_00e49040(
                            e,
                            Ptr::new(cursor.wrapping_sub(0x10)),
                            flag.addr(),
                            record,
                            bit_base.wrapping_add(vertex),
                        );
                        e.mem.u8(flag.addr()) != 0
                    });
                    if kept {
                        let point =
                            load_vec(e, object.wrapping_add(0x20).wrapping_add(16 * vertex));
                        store_vec(e, cursor, point);
                        cursor = cursor.wrapping_add(0x10);
                        let kept_count = e.mem.u32(shape.wrapping_add(0x1c));
                        e.mem
                            .set_u32(shape.wrapping_add(0x1c), kept_count.wrapping_add(1));
                    }
                }
                bit_base = bit_base.wrapping_add(3);
            }
        } else {
            e.mem.set_u32(shape.wrapping_add(0x28), 1);
            let extra = e.mem.u32(owner.wrapping_add(0x20));
            let item_slot = extra.wrapping_add(index.wrapping_sub(polygons).wrapping_mul(4));
            let cursor = buffer.wrapping_add(0x3b) & 0xffff_fff0;
            e.mem.set_u32(shape.wrapping_add(0x24), item_slot);
            e.mem.set_u32(shape.wrapping_add(0x18), cursor);
            e.mem.set_u32(shape.wrapping_add(0x1c), 3);
            let id = e.mem.u32(item_slot);
            let object = e.vcall(sub_object, 0x14, &args![id, scratch]).u32();
            for vertex in 0..3u32 {
                let point = load_vec(e, object.wrapping_add(0x20).wrapping_add(16 * vertex));
                store_vec(e, cursor.wrapping_add(16 * vertex), point);
            }
        }
    });
    shape
}

// Translated from 00e49290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4` (virtual): a tail call. Finds the item of entry `index` the way
/// `00e49120` does (below the polygon count: the first item of the record, `record[1]`; else
/// `*(owner + 0x20)[index - count]`, with `owner` the object at `*(this + 8)`) and calls virtual
/// slot `0x10` of the sub-object at `*(this + 0xc) + 0x10` with it, returning its result.
pub fn fn_00e49290(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    let this = this.addr();
    let owner = e.mem.u32(this.wrapping_add(8));
    let polygons = e.mem.u32(owner.wrapping_add(0x18));
    let item = if index < polygons {
        let numbers = e.mem.u32(owner.wrapping_add(0x14));
        let number = e.mem.u32(numbers.wrapping_add(index.wrapping_mul(4)));
        let words = e.mem.u32(owner.wrapping_add(8));
        e.mem
            .u32(words.wrapping_add(number.wrapping_mul(4)).wrapping_add(4))
    } else {
        let extra = e.mem.u32(owner.wrapping_add(0x20));
        e.mem
            .u32(extra.wrapping_add(index.wrapping_sub(polygons).wrapping_mul(4)))
    };
    let source = e.mem.u32(this.wrapping_add(0xc));
    e.vcall(source.wrapping_add(0x10), 0x10, &args![item]).u32()
}

// Translated from 00e492d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc` (virtual): the bounding box of every entry of the iterator interface at
/// `this + 0x10`. `out` (two vectors) starts as `(m, m, m, c)` and `(M, M, M, c)` with `m` the
/// `float` at `0104da48`, `M` the one at `0104e148` and `c` the one at `01011d78`. Then, for each
/// index from the iterator (virtual slot `8` gives the first, slot `0xc` the next after one; `-1`
/// ends), the iterator's slot `0x14` (index and a 0x200-byte buffer) gives an object whose
/// virtual slot `0x1c` is called with `first`, `scale` (a `float` through the x87 stack) and a
/// local pair of vectors; the pair is merged into `out` with `MINPS` and `MAXPS`.
pub fn fn_00e492d0(e: &mut Engine, this: Ptr, first: u32, scale: f32, out: u32) {
    let iterator = this.addr().wrapping_add(0x10);
    let minimum = f32::from_bits(e.mem.u32(BOX_MINIMUM_START));
    let lane_three = f32::from_bits(e.mem.u32(LANE_THREE));
    store_vec(e, out, [minimum, minimum, minimum, lane_three]);
    let maximum = f32::from_bits(e.mem.u32(BOX_MAXIMUM_START));
    store_vec(
        e,
        out.wrapping_add(0x10),
        [maximum, maximum, maximum, lane_three],
    );
    e.with_stack(0x240, |e, frame| {
        let frame = frame.addr();
        let (local, buffer) = (frame.wrapping_add(0x10), frame.wrapping_add(0x30));
        let mut index = e.vcall(iterator, 8, &[]).u32();
        while index != u32::MAX {
            let object = e.vcall(iterator, 0x14, &args![index, buffer]).u32();
            e.vcall(object, 0x1c, &args![first, x87_quiet(scale), local]);
            let merged = minps(load_vec(e, out), load_vec(e, local));
            store_vec(e, out, merged);
            let merged = maxps(
                load_vec(e, out.wrapping_add(0x10)),
                load_vec(e, local.wrapping_add(0x10)),
            );
            store_vec(e, out.wrapping_add(0x10), merged);
            index = e.vcall(iterator, 0xc, &args![index]).u32();
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e4_8df0, fn_00e48df0(Ptr, u32, i32)),
        entry!(0x00e4_8e80, fn_00e48e80(Ptr, u32) -> u32),
        entry!(0x00e4_9030, fn_00e49030(Ptr) -> u32),
        entry!(0x00e4_9040, fn_00e49040(Ptr, u32, u32, u32) -> u32),
        entry!(0x00e4_90d0, fn_00e490d0(Ptr)),
        entry!(0x00e4_9100, fn_00e49100(Ptr, i32) -> i32),
        entry!(0x00e4_9120, fn_00e49120(Ptr, u32, u32) -> u32),
        entry!(0x00e4_9290, fn_00e49290(Ptr, u32) -> u32),
        entry!(0x00e4_92d0, fn_00e492d0(Ptr, u32, f32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::run_00d6bf60::Vec4;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The argument words each double saw, tagged with the address.
    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.mem.set_u32(0x0126_0010, 0x4444);
        e.register(MEMORY_ROUTER, |_, _| Ret {
            eax: 0x0126_0000,
            ..Ret::default()
        });
        e
    }

    fn record(e: &mut Engine, addresses: &[u32]) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in addresses {
            let address = *address;
            let log = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        calls
    }

    fn put_u32s(e: &mut Engine, address: u32, values: &[(u32, u32)]) {
        for (offset, value) in values {
            e.mem.set_u32(address + offset, *value);
        }
    }

    /// An `hkArray` header with room for `capacity` elements of `element_size` bytes.
    fn array(e: &mut Engine, at: u32, capacity: u32, element_size: u32) {
        let data = e.mem.alloc(capacity * element_size + 8);
        put_u32s(e, at, &[(0, data), (4, 0), (8, capacity)]);
    }

    #[test]
    fn record_is_appended_to_both_arrays() {
        let mut e = engine();
        let this = e.mem.alloc(0x40);
        array(&mut e, this + 0x14, 4, 1);
        array(&mut e, this + 8, 8, 2);
        let items = e.mem.alloc(16);
        for (index, value) in [0x1_0007u32, 0x2_0008, 0x3_0009].iter().enumerate() {
            e.mem.set_u32(items + 4 * index as u32, *value);
        }
        e.call(0x00e4_8df0, &args![this, items, 3i32]);
        assert_eq!(e.mem.u32(this + 0x18), 1);
        assert_eq!(e.mem.u8(e.mem.u32(this + 0x14)), 3);
        assert_eq!(e.mem.u32(this + 0xc), 3);
        let data = e.mem.u32(this + 8);
        let values: Vec<u16> = (0..3).map(|index| e.mem.u16(data + 2 * index)).collect();
        assert_eq!(values, vec![7, 8, 9]);
        // A second record appends after the first.
        e.call(0x00e4_8df0, &args![this, items, 1i32]);
        assert_eq!(e.mem.u32(this + 0x18), 2);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        // No items: only the count byte.
        e.call(0x00e4_8df0, &args![this, items, 0i32]);
        assert_eq!(e.mem.u32(this + 0x18), 3);
        assert_eq!(e.mem.u32(this + 0xc), 4);
    }

    #[test]
    fn full_arrays_grow_before_the_append() {
        let mut e = engine();
        let calls = record(&mut e, &[]);
        let log = Rc::clone(&calls);
        e.register_double(RESERVE_MORE, move |e, a| {
            log.borrow_mut().push((RESERVE_MORE, a.to_vec()));
            let data = e.mem.alloc(64);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, 16);
            Ret::default()
        });
        let this = e.mem.alloc(0x40);
        array(&mut e, this + 0x14, 0, 1);
        array(&mut e, this + 8, 0, 2);
        let items = e.mem.alloc(8);
        e.call(0x00e4_8df0, &args![this, items, 1i32]);
        assert_eq!(
            *calls.borrow(),
            vec![
                (RESERVE_MORE, vec![0x4444, this + 0x14, 1]),
                (RESERVE_MORE, vec![0x4444, this + 8, 2])
            ]
        );
    }

    /// Doubles that keep an `hkMap` of 16 cells in game memory (key, value), the empty key being
    /// `0xffffffff`; `hashMod` is 15 once the first cell is inserted.
    fn map_doubles(e: &mut Engine) -> Calls {
        let calls = record(e, &[MAP_RESERVE, MAP_CLEAR_AND_DEALLOCATE, MAP_DESTRUCTOR]);
        e.register(MAP_FIND_KEY, |e, a| {
            let hash_mod = e.mem.i32(a[0] + 8);
            if hash_mod < 0 {
                return Ret::default();
            }
            let data = e.mem.u32(a[0]);
            for index in 0..=hash_mod as u32 {
                if e.mem.u32(data + 8 * index) == a[1] {
                    return Ret {
                        eax: index,
                        ..Ret::default()
                    };
                }
            }
            Ret {
                eax: hash_mod as u32 + 1,
                ..Ret::default()
            }
        });
        e.register(MAP_INSERT, |e, a| {
            if e.mem.i32(a[0] + 8) < 0 {
                let data = e.mem.alloc(16 * 8);
                for index in 0..16 {
                    e.mem.set_u32(data + 8 * index, u32::MAX);
                }
                e.mem.set_u32(a[0], data);
                e.mem.set_u32(a[0] + 8, 15);
            }
            let data = e.mem.u32(a[0]);
            let mut index = 0;
            while e.mem.u32(data + 8 * index) != u32::MAX {
                index += 1;
            }
            e.mem.set_u32(data + 8 * index, a[2]);
            e.mem.set_u32(data + 8 * index + 4, a[3]);
            Ret::default()
        });
        calls
    }

    /// An object holding polygons with the given vertex lists.
    fn polygons(e: &mut Engine, lists: &[&[u16]]) -> u32 {
        let this = e.mem.alloc(0x40);
        let lengths = e.mem.alloc(lists.len() as u32 + 4);
        let vertices = e.mem.alloc(64);
        let mut cursor = 0;
        for (index, list) in lists.iter().enumerate() {
            e.mem.set_u8(lengths + index as u32, list.len() as u8);
            for vertex in *list {
                e.mem.set_u16(vertices + 2 * cursor, *vertex);
                cursor += 1;
            }
        }
        put_u32s(
            e,
            this,
            &[(8, vertices), (0x14, lengths), (0x18, lists.len() as u32)],
        );
        this
    }

    fn pairing(e: &mut Engine, lists: &[&[u16]]) -> (u8, Calls) {
        let calls = map_doubles(e);
        let this = polygons(e, lists);
        let out = e.mem.alloc(8);
        e.mem.set_u8(out, 0xee);
        let result = e.call(0x00e4_8e80, &args![this, out]).u32();
        assert_eq!(result, out);
        assert_eq!(e.mem.u8(out + 1), 0);
        (e.mem.u8(out), calls)
    }

    #[test]
    fn two_sided_triangle_pairs_every_edge() {
        let mut e = engine();
        let (answer, calls) = pairing(&mut e, &[&[0, 1, 2], &[0, 2, 1]]);
        assert_eq!(answer, 1);
        let addresses: Vec<u32> = calls.borrow().iter().map(|(address, _)| *address).collect();
        assert_eq!(addresses[0], MAP_RESERVE);
        assert_eq!(
            addresses[addresses.len() - 2..],
            [MAP_CLEAR_AND_DEALLOCATE, MAP_DESTRUCTOR]
        );
        assert_eq!(calls.borrow()[0].1[2], 0);
    }

    #[test]
    fn single_triangle_leaves_unpaired_edges() {
        let mut e = engine();
        assert_eq!(pairing(&mut e, &[&[0, 1, 2]]).0, 0);
    }

    #[test]
    fn same_direction_twice_fails_at_once() {
        let mut e = engine();
        assert_eq!(pairing(&mut e, &[&[0, 1, 2], &[0, 1, 2]]).0, 0);
    }

    #[test]
    fn no_polygons_pass() {
        let mut e = engine();
        assert_eq!(pairing(&mut e, &[]).0, 1);
    }

    #[test]
    fn bit_field_has_31_bits_per_word() {
        let mut e = engine();
        let bits = e.mem.alloc(16);
        put_u32s(&mut e, bits, &[(0, 0), (4, 1 << 3), (8, 1)]);
        for (index, expected) in [(3u32, 1u8), (4, 0), (30, 0), (31, 1), (32, 0)] {
            let out = e.mem.alloc(8);
            e.call(0x00e4_9040, &args![0u32, out, bits, index]);
            assert_eq!(e.mem.u8(out), expected, "bit {index}");
            assert_eq!(e.mem.u8(out + 1), 0);
        }
        // The first word index is the header word.
        e.mem.set_u32(bits, 1);
        let out = e.mem.alloc(8);
        e.call(0x00e4_9040, &args![0u32, out, bits, 0u32]);
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn bit_test_answers_its_out_pointer() {
        let mut e = engine();
        let bits = e.mem.alloc(16);
        let out = e.mem.alloc(8);
        // EAX is `out` at the RET; the caller tests the flag through it (00e491ed).
        assert_eq!(
            e.call(0x00e4_9040, &args![0u32, out, bits, 3u32]).u32(),
            out
        );
    }

    #[test]
    fn destructor_drops_both_references_in_order() {
        let mut e = engine();
        let calls = record(&mut e, &[REMOVE_REFERENCE]);
        let this = e.mem.alloc(0x40);
        put_u32s(&mut e, this, &[(0x18, 0x2222), (0x1c, 0x1111)]);
        e.call(0x00e4_90d0, &args![this]);
        assert_eq!(
            *calls.borrow(),
            vec![
                (REMOVE_REFERENCE, vec![0x1111]),
                (REMOVE_REFERENCE, vec![0x2222])
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 0x10), 0x010c_6fb0);
    }

    #[test]
    fn constant_virtuals() {
        let mut e = engine();
        assert_eq!(e.call(0x00e4_9030, &args![0u32]).u32(), 0);
        let this = e.mem.alloc(0x20);
        let owner = e.mem.alloc(0x40);
        put_u32s(&mut e, owner, &[(0x18, 3), (0x24, 2)]);
        e.mem.set_u32(this + 8, owner);
        for (index, expected) in [(0i32, 1i32), (3, 4), (4, -1), (-1, 0)] {
            assert_eq!(e.call(0x00e4_9100, &args![this, index]).i32(), expected);
        }
    }

    /// A source object whose sub-object (at `+0x10`) has virtual slot `0x14` returning an object
    /// whose vertices (at `+0x20`) are `(id * 10 + j)` in every lane, and slot `0x10` answering
    /// `id + 100`; the recorded calls hold `(slot, words)`.
    fn source_object(e: &mut Engine) -> (u32, Calls) {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&calls);
        e.register_double(0x0ff0_0014, move |e, a| {
            log.borrow_mut().push((0x14, a.to_vec()));
            let triangle = e.mem.alloc(0x60);
            for vertex in 0..3u32 {
                store_vec(
                    e,
                    triangle + 0x20 + 16 * vertex,
                    [(a[1] * 10 + vertex) as f32; 4],
                );
            }
            Ret {
                eax: triangle,
                ..Ret::default()
            }
        });
        let log = Rc::clone(&calls);
        e.register_double(0x0ff0_0010, move |_, a| {
            log.borrow_mut().push((0x10, a.to_vec()));
            Ret {
                eax: a[1] + 100,
                ..Ret::default()
            }
        });
        e.put_vtable(0x0126_0400, &[0, 0, 0, 0, 0x0ff0_0010, 0x0ff0_0014]);
        let source = e.mem.alloc(0x40);
        e.mem.set_u32(source + 0x10, 0x0126_0400);
        (source, calls)
    }

    /// The owner object: two polygons, record numbers `[0, 3]`, the words `B`, extra items.
    fn owner(e: &mut Engine) -> u32 {
        let owner = e.mem.alloc(0x40);
        let numbers = e.mem.alloc(8);
        put_u32s(e, numbers, &[(0, 0), (4, 4)]);
        let words = e.mem.alloc(0x40);
        // Record 0: n = 2, items 5 and 6, then the bit word 0b10101 (vertices 0, 2 and 4).
        put_u32s(e, words, &[(0, 2), (4, 5), (8, 6), (0xc, 0b10101)]);
        // Record 4: n = 1, item 7, bit word 0b111.
        put_u32s(e, words, &[(0x10, 1), (0x14, 7), (0x18, 0b111)]);
        let extra = e.mem.alloc(16);
        put_u32s(e, extra, &[(0, 8), (4, 9)]);
        put_u32s(
            e,
            owner,
            &[
                (8, words),
                (0x14, numbers),
                (0x18, 2),
                (0x20, extra),
                (0x24, 2),
            ],
        );
        owner
    }

    fn shape_builder(e: &mut Engine) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&calls);
        e.register_double(BUILD_SHAPE_OBJECT, move |_, a| {
            log.borrow_mut().push((BUILD_SHAPE_OBJECT, a.to_vec()));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        calls
    }

    fn lanes(e: &Engine, address: u32) -> Vec4 {
        load_vec(e, address)
    }

    #[test]
    fn polygon_entry_keeps_the_vertices_with_set_bits() {
        let mut e = engine();
        let built = shape_builder(&mut e);
        let (source, calls) = source_object(&mut e);
        let owner = owner(&mut e);
        let this = e.mem.alloc(0x40);
        put_u32s(
            &mut e,
            this,
            &[(8, owner), (0xc, source), (0x10, 0.5f32.to_bits())],
        );
        let buffer = e.mem.alloc(0x200);
        let shape = e.call(0x00e4_9120, &args![this, 0u32, buffer]).u32();
        assert_eq!(shape, buffer);
        assert_eq!(
            *built.borrow(),
            vec![(BUILD_SHAPE_OBJECT, vec![buffer, 0.5f32.to_bits()])]
        );
        assert_eq!(e.mem.u32(shape + 0x20), source);
        let cursor = (buffer + 0x3b) & !0xf;
        assert_eq!(e.mem.u32(shape + 0x18), cursor);
        assert_eq!(e.mem.u32(shape + 0x1c), 3);
        assert_eq!(e.mem.u32(shape + 0x28), 2);
        assert_eq!(e.mem.u32(shape + 0x24), e.mem.u32(owner + 8) + 4);
        // Item 5 keeps vertices 0 and 2, item 6 keeps vertex 1 (bit 4).
        assert_eq!(lanes(&e, cursor), [50.0; 4]);
        assert_eq!(lanes(&e, cursor + 16), [52.0; 4]);
        assert_eq!(lanes(&e, cursor + 32), [61.0; 4]);
        let asked: Vec<u32> = calls.borrow().iter().map(|(_, words)| words[1]).collect();
        assert_eq!(asked, vec![5, 6]);
        assert_eq!(calls.borrow()[0].1[0], source + 0x10);
    }

    #[test]
    fn extra_entry_keeps_all_three_vertices() {
        let mut e = engine();
        shape_builder(&mut e);
        let (source, _) = source_object(&mut e);
        let owner = owner(&mut e);
        let this = e.mem.alloc(0x40);
        put_u32s(&mut e, this, &[(8, owner), (0xc, source)]);
        let buffer = e.mem.alloc(0x200);
        let shape = e.call(0x00e4_9120, &args![this, 3u32, buffer]).u32();
        let cursor = (buffer + 0x3b) & !0xf;
        assert_eq!(e.mem.u32(shape + 0x28), 1);
        assert_eq!(e.mem.u32(shape + 0x1c), 3);
        let extra = e.mem.u32(owner + 0x20);
        assert_eq!(e.mem.u32(shape + 0x24), extra + 4);
        for vertex in 0..3 {
            assert_eq!(lanes(&e, cursor + 16 * vertex), [(90 + vertex) as f32; 4]);
        }
    }

    #[test]
    fn second_record_of_a_polygon_entry_uses_its_own_bits() {
        let mut e = engine();
        shape_builder(&mut e);
        let (source, _) = source_object(&mut e);
        let owner = owner(&mut e);
        let this = e.mem.alloc(0x40);
        put_u32s(&mut e, this, &[(8, owner), (0xc, source)]);
        let buffer = e.mem.alloc(0x200);
        let shape = e.call(0x00e4_9120, &args![this, 1u32, buffer]).u32();
        let cursor = (buffer + 0x3b) & !0xf;
        // Record number 4: one item (7), all three bits set.
        assert_eq!(e.mem.u32(shape + 0x1c), 3);
        assert_eq!(e.mem.u32(shape + 0x28), 1);
        assert_eq!(lanes(&e, cursor + 32), [72.0; 4]);
    }

    #[test]
    fn item_tail_call_picks_the_first_item_or_the_extra_one() {
        let mut e = engine();
        let (source, calls) = source_object(&mut e);
        let owner = owner(&mut e);
        let this = e.mem.alloc(0x40);
        put_u32s(&mut e, this, &[(8, owner), (0xc, source)]);
        assert_eq!(e.call(0x00e4_9290, &args![this, 0u32]).u32(), 105);
        assert_eq!(e.call(0x00e4_9290, &args![this, 1u32]).u32(), 107);
        assert_eq!(e.call(0x00e4_9290, &args![this, 3u32]).u32(), 109);
        assert_eq!(calls.borrow()[0].1[0], source + 0x10);
    }

    #[test]
    fn iterator_boxes_are_merged_into_the_output() {
        let mut e = engine();
        e.mem.set_f32(BOX_MINIMUM_START, 1.0e30);
        e.mem.set_f32(BOX_MAXIMUM_START, -1.0e30);
        e.mem.set_f32(LANE_THREE, 4.0);
        // Iterator vtable: slot 2 first index, slot 3 next, slot 5 object.
        let calls = record(&mut e, &[]);
        let log = Rc::clone(&calls);
        e.register_double(0x0ff0_0008, move |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register_double(0x0ff0_000c, move |_, a| Ret {
            eax: if a[1] == 3 { 5 } else { u32::MAX },
            ..Ret::default()
        });
        let query = e.mem.alloc(8);
        e.put_vtable(0x0126_0500, &[0, 0, 0, 0, 0, 0, 0, 0x0ff0_001c]);
        e.mem.set_u32(query, 0x0126_0500);
        e.register_double(0x0ff0_0014, move |_, a| Ret {
            eax: if a[1] == 3 { query } else { query + 4 },
            ..Ret::default()
        });
        e.mem.set_u32(query + 4, 0x0126_0500);
        e.register_double(0x0ff0_001c, move |e, a| {
            log.borrow_mut().push((0x1c, a.to_vec()));
            let base = if a[0] == query { 10.0 } else { 20.0 };
            store_vec(e, a[3], [base, base, base, 0.0]);
            store_vec(e, a[3] + 0x10, [base + 5.0, base + 5.0, base + 5.0, 0.0]);
            Ret::default()
        });
        e.put_vtable(
            0x0126_0600,
            &[0, 0, 0x0ff0_0008, 0x0ff0_000c, 0, 0x0ff0_0014],
        );
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x10, 0x0126_0600);
        let out = e.mem.alloc(0x20);
        e.call(0x00e4_92d0, &args![this, 0x77u32, 2.0f32, out]);
        assert_eq!(lanes(&e, out), [10.0, 10.0, 10.0, 0.0]);
        assert_eq!(lanes(&e, out + 16), [25.0, 25.0, 25.0, 4.0]);
        assert_eq!(calls.borrow().len(), 2);
        assert_eq!(calls.borrow()[0].1[1], 0x77);
        assert_eq!(calls.borrow()[0].1[2], 2.0f32.to_bits());
    }

    #[test]
    fn empty_iterator_leaves_the_starting_box() {
        let mut e = engine();
        e.mem.set_f32(BOX_MINIMUM_START, 1.0e30);
        e.mem.set_f32(BOX_MAXIMUM_START, -1.0e30);
        e.mem.set_f32(LANE_THREE, 4.0);
        e.register_double(0x0ff0_0008, |_, _| Ret {
            eax: u32::MAX,
            ..Ret::default()
        });
        e.put_vtable(0x0126_0600, &[0, 0, 0x0ff0_0008]);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x10, 0x0126_0600);
        let out = e.mem.alloc(0x20);
        e.call(0x00e4_92d0, &args![this, 0u32, 0.0f32, out]);
        assert_eq!(lanes(&e, out), [1.0e30, 1.0e30, 1.0e30, 4.0]);
        assert_eq!(lanes(&e, out + 16), [-1.0e30, -1.0e30, -1.0e30, 4.0]);
    }
}
