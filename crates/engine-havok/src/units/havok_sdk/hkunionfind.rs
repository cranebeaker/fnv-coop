//! `Havok SDK/hkunionfind.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): word +0xc is a chunk allocator, word +0x10 the heap
/// allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`, cdecl `(allocator, array, count,
/// elementSize)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;

// Translated from 00d282c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkUnionFind::reindex` (Xbox PDB). `this` (ECX) is a record whose first word points to a block whose first word is the data pointer, with the count at +4;
/// `map` points to a block whose first word is the int data pointer; `count` is the size of
/// the output; `output` is an int array `{data, count, capacity}`. Replaces every element `x`
/// of `this` with `map[x]`; then, with a zeroed scratch block of `count` words taken from the
/// router's chunk allocator (vtable offset 0xc, which takes the byte size by reference and may
/// round it up), adds `output[i]` to `scratch[map[i]]` for every `i` below the output's
/// count, grows `output` through `_reserve` when its capacity (low 30 bits) is below `count`
/// (to the larger of `count` and double the capacity, 4-byte elements), sets its count to
/// `count`, copies the scratch block into it and frees the block with vtable offset 0x10 of the chunk
/// allocator, unless the block was never allocated (`count` zero).
pub fn hk_union_find_reindex(e: &mut Engine, this: Ptr, map: Ptr, count: u32, output: Ptr) {
    let this = this.addr();
    let map_data = |e: &Engine| e.mem.u32(map.addr());
    let mut i = 0u32;
    while (i as i32) < e.mem.u32(this + 4) as i32 {
        let data = e.mem.u32(e.mem.u32(this));
        let value = e.mem.u32(data + i * 4);
        let mapped = e.mem.u32(map_data(e) + value.wrapping_mul(4));
        e.mem.set_u32(data + i * 4, mapped);
        i += 1;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let chunk_allocator = e.mem.u32(router + 0xc);
    let (scratch, words) = if count != 0 {
        e.with_stack(4, |e, size| {
            e.mem.set_u32(size.addr(), count.wrapping_mul(4));
            let block = e.vcall(chunk_allocator, 0xc, &args![size]).u32();
            let bytes = e.mem.u32(size.addr()) as i32;
            let words = (bytes + (((bytes >> 31) as u32 & 3) as i32)) >> 2;
            (
                block,
                if words == 0 {
                    0x8000_0000
                } else {
                    words as u32
                },
            )
        })
    } else {
        (0, 0x8000_0000)
    };
    if (count as i32) > 0 {
        for k in 0..count {
            e.mem.set_u32(scratch + k * 4, 0);
        }
    }
    let output_count = e.mem.u32(output.addr() + 4) as i32;
    let mut i = 0u32;
    if output_count > 0 {
        loop {
            let slot = e.mem.u32(map_data(e) + i * 4);
            let add = e.mem.u32(e.mem.u32(output.addr()) + i * 4);
            let at = scratch.wrapping_add(slot.wrapping_mul(4));
            let sum = e.mem.u32(at).wrapping_add(add);
            e.mem.set_u32(at, sum);
            i += 1;
            if i as i32 >= e.mem.u32(output.addr() + 4) as i32 {
                break;
            }
        }
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let heap = e.mem.u32(router + 0x10);
    let capacity = (e.mem.u32(output.addr() + 8) & 0x3fff_ffff) as i32;
    if capacity < count as i32 {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if (count as i32) < doubled {
            doubled
        } else {
            count as i32
        };
        e.call(ARRAY_RESERVE, &args![heap, output, wanted as u32, 4u32]);
    }
    e.mem.set_u32(output.addr() + 4, count);
    let mut i = 0u32;
    while (i as i32) < (count as i32) {
        let value = e.mem.u32(scratch + i * 4);
        let data = e.mem.u32(output.addr());
        e.mem.set_u32(data + i * 4, value);
        i += 1;
    }
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let chunk_allocator = e.mem.u32(router + 0xc);
    if (words as i32) >= 0 {
        let bytes = (words & 0x3fff_ffff).wrapping_mul(4);
        e.vcall(chunk_allocator, 0x10, &args![scratch, bytes]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00d2_82c0,
        hk_union_find_reindex(Ptr, Ptr, u32, Ptr)
    )]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    #[test]
    fn reindex_maps_the_items_and_sums_into_the_output() {
        let mut e = test_engine();
        let log: Rc<RefCell<Vec<(&'static str, Vec<u32>)>>> = Rc::new(RefCell::new(Vec::new()));
        let heap = e.mem.alloc(0x20);
        let sink = Rc::clone(&log);
        e.register_double(0x0900_0003, move |e, a| {
            sink.borrow_mut().push(("alloc", vec![e.mem.u32(a[1])]));
            Ret {
                eax: e.mem.alloc(64),
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(0x0900_0004, move |_, a| {
            sink.borrow_mut().push(("free", vec![a[2]]));
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0xc, 0x0900_0003);
        e.mem.set_u32(vtable + 0x10, 0x0900_0004);
        let chunk = e.mem.alloc(8);
        e.mem.set_u32(chunk, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0xc, chunk);
        e.mem.set_u32(router + 0x10, heap);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let sink = Rc::clone(&log);
        e.register_double(0x00c9_08c0, move |e, a| {
            sink.borrow_mut().push(("reserve", vec![a[2], a[3]]));
            let data = e.mem.alloc(64);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        // this -> block -> data [1, 0, 2]; map data [5, 1, 0] -> becomes [1, 5, 0]
        let data = e.mem.alloc(16);
        for (i, v) in [1u32, 0, 2].into_iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, v);
        }
        let block = e.mem.alloc(8);
        e.mem.set_u32(block, data);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, block);
        e.mem.set_u32(this + 4, 3);
        let map_data = e.mem.alloc(16);
        for (i, v) in [0u32, 1, 1].into_iter().enumerate() {
            e.mem.set_u32(map_data + 4 * i as u32, v);
        }
        let map = e.mem.alloc(4);
        e.mem.set_u32(map, map_data);
        // output: two items [10, 20], capacity 0 so it must grow to 2.
        let out_data = e.mem.alloc(16);
        e.mem.set_u32(out_data, 10);
        e.mem.set_u32(out_data + 4, 20);
        let output = e.mem.alloc(12);
        e.mem.set_u32(output, out_data);
        e.mem.set_u32(output + 4, 2);
        e.call(0x00d2_82c0, &args![this, map, 2u32, output]);
        // items mapped through the map: 1 -> 1, 0 -> 0, 2 -> 1
        assert_eq!(e.mem.u32(data), 1);
        assert_eq!(e.mem.u32(data + 4), 0);
        assert_eq!(e.mem.u32(data + 8), 1);
        // scratch[map[0]=0] += 10, scratch[map[1]=1] += 20
        let new_data = e.mem.u32(output);
        assert_eq!(e.mem.u32(output + 4), 2);
        assert_eq!(e.mem.u32(new_data), 10);
        assert_eq!(e.mem.u32(new_data + 4), 20);
        let log = log.borrow();
        assert_eq!(log[0], ("alloc", vec![8]));
        assert_eq!(log[1], ("reserve", vec![2, 4]));
        assert_eq!(log[2], ("free", vec![8]));
    }
}
