//! `Havok SDK/run_00e3a1e0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::{
    allocator, release_array, ARRAY_RESERVE_MORE, CAPACITY_MASK, MEMORY_ROUTER,
};

/// `hkReferencedObject::removeReference` (`00c905b0`, `this` = the object).
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;

// Translated from 00e3a1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: sets `[this + 0xc]` to `count` and `[this + 8]` to 0, then appends
/// `count` zeroed 32-byte elements to the `hkArray`-shaped array at `this + 0x10` (data, size,
/// capacity and flags). Each element is built in a 32-byte stack temporary cleared by `00e39e50`;
/// the memory router is asked, the array grows through `00c90940` (allocator, array, 0x20) when
/// its size equals its capacity, and the 8 words are copied to data + size * 32 unless that
/// address is null; the size goes up by one. Nothing is appended when `count` is not above zero
/// (the counter `[this + 0xc]` is read again after each element). The function takes one stack
/// word and pops it (`RET 4`) and returns nothing. The class is unconfirmed.
pub fn fn_00e3a1e0(e: &mut Engine, this: Ptr, count: i32) {
    let base = this.addr();
    e.mem.set_u32(base + 0xc, count as u32);
    e.mem.set_u32(base + 8, 0);
    if count <= 0 {
        return;
    }
    let array = base + 0x10;
    let mut index: i32 = 0;
    e.with_stack(0x20, |e, element| loop {
        e.call(0x00e3_9e50, &args![element]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
        if e.mem.u32(array + 4) == capacity {
            let heap = e.mem.u32(router + 0x10);
            e.call(ARRAY_RESERVE_MORE, &args![heap, array, 0x20u32]);
        }
        let slot = e
            .mem
            .u32(array + 4)
            .wrapping_shl(5)
            .wrapping_add(e.mem.u32(array));
        if slot != 0 {
            for word in 0..8u32 {
                let value = e.mem.u32(element.addr() + 4 * word);
                e.mem.set_u32(slot + 4 * word, value);
            }
        }
        let size = e.mem.u32(array + 4);
        e.mem.set_u32(array + 4, size.wrapping_add(1));
        index = index.wrapping_add(1);
        if index >= e.mem.u32(base + 0xc) as i32 {
            break;
        }
    });
}

// Translated from 00e3a360 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a destructor (`this` in `ECX`). It writes the vtable word
/// `010e00dc`, releases a reference on every object pointer in the array at `this + 0x10`
/// (`00c905b0`; no null check; the size at `+0x14` is read again after each), empties the array
/// (4-byte elements: size cleared, storage returned through the allocator's slot `0x10` unless
/// the capacity word at `+0x18` is negative, data cleared, capacity word `0x80000000`) and
/// writes the vtable word `0102e388`. The class is unconfirmed.
pub fn fn_00e3a360(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010e_00dc);
    if (e.mem.u32(base + 0x14) as i32) > 0 {
        let mut index: i32 = 0;
        loop {
            let data = e.mem.u32(base + 0x10);
            let object = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            e.call(REMOVE_REFERENCE, &args![object]);
            index = index.wrapping_add(1);
            if index >= e.mem.u32(base + 0x14) as i32 {
                break;
            }
        }
    }
    release_array(e, base + 0x10, 4);
    e.mem.set_u32(base, 0x0102_e388);
}

// Translated from 00e3a3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a destructor (`this` in `ECX`). It writes the vtable word
/// `010e04ac`, releases a reference (`00c905b0`) on the object pointer at the start of each of
/// the `[this + 0xc]` elements of 0x50 bytes in the array at `this + 8` (the count is read once),
/// empties the array (element size 0x50, as in `hkArray::clearAndDeallocate`) and writes the
/// vtable word `0102e388`. The class is unconfirmed.
pub fn fn_00e3a3d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let count = e.mem.u32(base + 0xc) as i32;
    e.mem.set_u32(base, 0x010e_04ac);
    if count > 0 {
        let mut offset: u32 = 0;
        let mut remaining = count;
        loop {
            let data = e.mem.u32(base + 8);
            let object = e.mem.u32(data.wrapping_add(offset));
            e.call(REMOVE_REFERENCE, &args![object]);
            offset = offset.wrapping_add(0x50);
            remaining = remaining.wrapping_sub(1);
            if remaining == 0 {
                break;
            }
        }
    }
    release_array(e, base + 8, 0x50);
    e.mem.set_u32(base, 0x0102_e388);
}

/// The slot `index` of a table of 12-byte records (`first`, `count`, one more word) at
/// `[this + 8]`.
fn record_address(e: &Engine, this: u32, index: i32) -> u32 {
    e.mem
        .u32(this + 8)
        .wrapping_add((index as u32).wrapping_mul(12))
}

// Translated from 00e3a4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: copies the 16-byte vector at `vector` into the vector at offset
/// `0x20` of every slot a record names. Record `index` (12 bytes at `[this + 8]`) gives `first`
/// and `count`; entry `first + i` of the table at `[this + 0x14]` (8 bytes: an object pointer and
/// a slot number) names the destination `[object + 0x18] + slot * 0x50 + 0x20`. The vector is
/// loaded again for every entry. The function takes two stack words and pops them (`RET 8`) and
/// returns nothing. The class is unconfirmed.
pub fn fn_00e3a4d0(e: &mut Engine, this: Ptr, index: i32, vector: Ptr) {
    let record = record_address(e, this.addr(), index);
    if (e.mem.u32(record + 4) as i32) <= 0 {
        return;
    }
    let mut entry_index: i32 = 0;
    loop {
        let first = e.mem.u32(record);
        let table = e.mem.u32(this.addr() + 0x14);
        let words: [u32; 4] =
            std::array::from_fn(|word| e.mem.u32(vector.addr() + 4 * word as u32));
        let entry = table.wrapping_add(first.wrapping_add(entry_index as u32).wrapping_mul(8));
        let slot = e.mem.u32(entry + 4);
        let object = e.mem.u32(entry);
        let storage = e.mem.u32(object + 0x18);
        let target = slot
            .wrapping_mul(0x50)
            .wrapping_add(storage)
            .wrapping_add(0x20);
        for (word, value) in words.into_iter().enumerate() {
            e.mem.set_u32(target + 4 * word as u32, value);
        }
        entry_index = entry_index.wrapping_add(1);
        if entry_index >= e.mem.u32(record + 4) as i32 {
            break;
        }
    }
}

// Translated from 00e3a520 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: collects the non-null pointers a record points at. For every
/// entry of record `index` (as in `00e3a4d0`) the word at `[object + 0x18] + 0x40 + 4 * field +
/// slot * 0x50` is read; when it is not zero it is appended to the pointer array at `array`
/// (data, size, capacity and flags): the memory router is asked, the array grows through
/// `00c90940` (allocator, array, 4) when its size equals its capacity, the word is stored at
/// data + size * 4 and the size goes up by one. The function takes three stack words and pops
/// them (`RET 0xc`) and returns nothing. The class is unconfirmed.
pub fn fn_00e3a520(e: &mut Engine, this: Ptr, index: i32, field: u32, array: Ptr) {
    let record = record_address(e, this.addr(), index);
    if (e.mem.u32(record + 4) as i32) <= 0 {
        return;
    }
    let mut entry_index: i32 = 0;
    loop {
        let first = e.mem.u32(record);
        let table = e.mem.u32(this.addr() + 0x14);
        let entry = table.wrapping_add(first.wrapping_add(entry_index as u32).wrapping_mul(8));
        let slot = e.mem.u32(entry + 4);
        let object = e.mem.u32(entry);
        let storage = e.mem.u32(object + 0x18);
        let offset = field.wrapping_mul(4).wrapping_add(slot.wrapping_mul(0x50));
        let value = e.mem.u32(storage.wrapping_add(0x40).wrapping_add(offset));
        if value != 0 {
            let router = e.call(MEMORY_ROUTER, &args![]).u32();
            let capacity = e.mem.u32(array.addr() + 8) & CAPACITY_MASK;
            if e.mem.u32(array.addr() + 4) == capacity {
                let heap = e.mem.u32(router + 0x10);
                e.call(ARRAY_RESERVE_MORE, &args![heap, array, 4u32]);
            }
            let size = e.mem.u32(array.addr() + 4);
            let data = e.mem.u32(array.addr());
            e.mem
                .set_u32(data.wrapping_add(size.wrapping_mul(4)), value);
            e.mem.set_u32(array.addr() + 4, size.wrapping_add(1));
        }
        entry_index = entry_index.wrapping_add(1);
        if entry_index >= e.mem.u32(record + 4) as i32 {
            break;
        }
    }
}

// Translated from 00e3ad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: a destructor (`this` in `ECX`). It writes the vtable word
/// `010e0654`, releases a reference (`00c905b0`) on the non-null object pointer at `+8` of each
/// 12-byte element of the array at `this + 8` (count at `+0xc`), then on every pointer of the
/// array at `this + 0x20` (count at `+0x24`; no null check), empties the arrays at `+0x20`
/// (4-byte elements), `+0x14` (8-byte elements) and `+8` (12-byte elements) in that order, and
/// writes the vtable word `0102e388`. The class is unconfirmed.
pub fn fn_00e3ad80(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010e_0654);
    if (e.mem.u32(base + 0xc) as i32) > 0 {
        let mut index: i32 = 0;
        loop {
            let data = e.mem.u32(base + 8);
            let object = e
                .mem
                .u32(data.wrapping_add((index as u32).wrapping_mul(12)) + 8);
            if object != 0 {
                e.call(REMOVE_REFERENCE, &args![object]);
            }
            index = index.wrapping_add(1);
            if index >= e.mem.u32(base + 0xc) as i32 {
                break;
            }
        }
    }
    if (e.mem.u32(base + 0x24) as i32) > 0 {
        let mut index: i32 = 0;
        loop {
            let data = e.mem.u32(base + 0x20);
            let object = e.mem.u32(data.wrapping_add((index as u32).wrapping_mul(4)));
            e.call(REMOVE_REFERENCE, &args![object]);
            index = index.wrapping_add(1);
            if index >= e.mem.u32(base + 0x24) as i32 {
                break;
            }
        }
    }
    release_array(e, base + 0x20, 4);
    release_array(e, base + 0x14, 8);
    release_array(e, base + 8, 12);
    e.mem.set_u32(base, 0x0102_e388);
}

// Translated from 00e3ae70 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: averages two floats over the pointers `00e3a520` collects and
/// stores the averages in each pointed-to object (`this` in `ECX`; four stack words, `RET
/// 0x10`). A three-word pointer array on the stack (`{0, 0, 0x80000000}`) is filled by `00e3a520`
/// (`this`, `index`, `field`, the array). When it holds `n` pointers (`n` not zero),
/// `average_a = first / n` and `average_b = second / n` are computed on the x87 stack (`n` is
/// loaded as an integer and stored as a float first; the quotients are stored as floats), and
/// when `n` is above zero every pointed-to object gets `average_a` at `+0xc` and `average_b` at
/// `+0x10`. Then the router is asked and the array's size cleared; unless the capacity word is
/// negative its storage is returned through the allocator's slot `0x10` (capacity times 4
/// bytes). The class is unconfirmed.
pub fn fn_00e3ae70(e: &mut Engine, this: Ptr, index: i32, field: u32, first: f32, second: f32) {
    e.with_stack(12, |e, list| {
        e.mem.set_u32(list.addr(), 0);
        e.mem.set_u32(list.addr() + 4, 0);
        e.mem.set_u32(list.addr() + 8, 0x8000_0000);
        fn_00e3a520(e, this, index, field, list);
        let count = e.mem.u32(list.addr() + 4) as i32;
        if count != 0 {
            let divisor = x87_load32(x87_store32(f64::from(count)));
            let average_first = x87_store32(x87_div(x87_load32(first.to_bits()), divisor));
            let average_second = x87_store32(x87_div(x87_m32(second.to_bits()), divisor));
            if count > 0 {
                let mut position: i32 = 0;
                loop {
                    let data = e.mem.u32(list.addr());
                    let object = e
                        .mem
                        .u32(data.wrapping_add((position as u32).wrapping_mul(4)));
                    e.mem.set_u32(object + 0xc, x87_quiet_bits(average_first));
                    position = position.wrapping_add(1);
                    e.mem.set_u32(object + 0x10, x87_quiet_bits(average_second));
                    if position >= e.mem.u32(list.addr() + 4) as i32 {
                        break;
                    }
                }
            }
        }
        let heap = allocator(e);
        let capacity = e.mem.u32(list.addr() + 8);
        e.mem.set_u32(list.addr() + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(list.addr());
            e.vcall(
                heap,
                0x10,
                &args![data, (capacity & CAPACITY_MASK).wrapping_mul(4)],
            );
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_a1e0, fn_00e3a1e0(Ptr, i32)),
        entry!(0x00e3_a360, fn_00e3a360(Ptr)),
        entry!(0x00e3_a3d0, fn_00e3a3d0(Ptr)),
        entry!(0x00e3_a4d0, fn_00e3a4d0(Ptr, i32, Ptr)),
        entry!(0x00e3_a520, fn_00e3a520(Ptr, i32, u32, Ptr)),
        entry!(0x00e3_ad80, fn_00e3ad80(Ptr)),
        entry!(0x00e3_ae70, fn_00e3ae70(Ptr, i32, u32, f32, f32)),
    ]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT: u32 = 0x0900_0011;
    const RELEASE: u32 = 0x00c9_05b0;
    const GROW: u32 = 0x00c9_0940;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A router whose allocator's slot 0x10 logs `(0x10, [allocator, data, bytes])`, a release
    /// double and a growth double (both log) that gives the array room for 8 elements of
    /// `element` bytes in fresh storage.
    fn engine() -> (Engine, Log, u32) {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, FREE_SLOT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        for address in [FREE_SLOT, RELEASE] {
            let calls = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                calls.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let calls = Rc::clone(&log);
        e.register_double(GROW, move |e, a| {
            calls.borrow_mut().push((GROW, a.to_vec()));
            let storage = e.mem.alloc(0x200);
            e.mem.set_u32(a[1], storage);
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        (e, log, allocator)
    }

    /// Three elements are appended (zero, through the `00e39e50` double that writes a marker
    /// first): the growth double runs once, for the first element.
    #[test]
    fn fn_00e3a1e0_appends_cleared_elements() {
        let (mut e, log, allocator) = engine();
        let calls = Rc::clone(&log);
        e.register_double(0x00e3_9e50, move |e, a| {
            calls.borrow_mut().push((0x00e3_9e50, a.to_vec()));
            for word in 0..8 {
                e.mem.set_u32(a[0] + 4 * word, 0x100 + word);
            }
            Ret::default()
        });
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x18, 0x8000_0000);
        e.call(0x00e3_a1e0, &args![this, 3i32]);
        assert_eq!(e.mem.u32(this + 0xc), 3);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x14), 3);
        let data = e.mem.u32(this + 0x10);
        for element in 0..3u32 {
            for word in 0..8u32 {
                assert_eq!(e.mem.u32(data + 32 * element + 4 * word), 0x100 + word);
            }
        }
        let grows: Vec<_> = log
            .borrow()
            .iter()
            .filter(|(a, _)| *a == GROW)
            .cloned()
            .collect();
        assert_eq!(grows, vec![(GROW, vec![allocator, this + 0x10, 0x20])]);
        let clears = log
            .borrow()
            .iter()
            .filter(|(a, _)| *a == 0x00e3_9e50)
            .count();
        assert_eq!(clears, 3);
    }

    /// A count that is not above zero only sets the two fields.
    #[test]
    fn fn_00e3a1e0_appends_nothing_for_a_non_positive_count() {
        let (mut e, log, _) = engine();
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 8, 5);
        e.call(0x00e3_a1e0, &args![this, 0i32]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert!(log.borrow().is_empty());
    }

    /// References are released on every pointer (nulls included), then the array's storage is
    /// freed (capacity 5 times 4 bytes) and emptied.
    #[test]
    fn fn_00e3a360_releases_the_objects_and_the_array() {
        let (mut e, log, allocator) = engine();
        let this = e.mem.alloc(0x30);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, 0xa1);
        e.mem.set_u32(data + 4, 0);
        e.mem.set_u32(data + 8, 0xa3);
        e.mem.set_u32(this + 0x10, data);
        e.mem.set_u32(this + 0x14, 3);
        e.mem.set_u32(this + 0x18, 5);
        e.call(0x00e3_a360, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                (RELEASE, vec![0xa1]),
                (RELEASE, vec![0]),
                (RELEASE, vec![0xa3]),
                (FREE_SLOT, vec![allocator, data, 20])
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        assert_eq!(e.mem.u32(this + 0x18), 0x8000_0000);
    }

    /// The first word of each 0x50-byte element is released; the storage is freed with the
    /// capacity times 0x50 bytes.
    #[test]
    fn fn_00e3a3d0_releases_the_leading_pointers() {
        let (mut e, log, allocator) = engine();
        let this = e.mem.alloc(0x30);
        let data = e.mem.alloc(0x200);
        e.mem.set_u32(data, 0xb1);
        e.mem.set_u32(data + 0x50, 0xb2);
        e.mem.set_u32(this + 8, data);
        e.mem.set_u32(this + 0xc, 2);
        e.mem.set_u32(this + 0x10, 3);
        e.call(0x00e3_a3d0, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                (RELEASE, vec![0xb1]),
                (RELEASE, vec![0xb2]),
                (FREE_SLOT, vec![allocator, data, 0xf0])
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        assert_eq!(e.mem.u32(this + 0x10), 0x8000_0000);
    }

    /// A scene: `this + 8` holds two 12-byte records ((first 0, count 1) and (first 1, count
    /// 2)), `this + 0x14` three entries (object, slot) with slots 0, 1 and 2 of two objects,
    /// whose storage (`[object + 0x18]`) holds the pointer words the functions read.
    struct Scene {
        e: Engine,
        this: u32,
        storage: [u32; 2],
    }

    fn scene() -> (Scene, Log, u32) {
        let (mut e, log, allocator) = engine();
        let this = e.mem.alloc(0x30);
        let records = e.mem.alloc(0x18);
        for (index, (first, count)) in [(0u32, 1u32), (1, 2)].into_iter().enumerate() {
            e.mem.set_u32(records + 12 * index as u32, first);
            e.mem.set_u32(records + 12 * index as u32 + 4, count);
        }
        e.mem.set_u32(this + 8, records);
        let objects = [e.mem.alloc(0x20), e.mem.alloc(0x20)];
        let storage = [e.mem.alloc(0x200), e.mem.alloc(0x200)];
        for (object, storage) in objects.iter().zip(storage) {
            e.mem.set_u32(object + 0x18, storage);
        }
        let entries = e.mem.alloc(0x18);
        for (index, (object, slot)) in [(0usize, 0u32), (0, 1), (1, 2)].into_iter().enumerate() {
            e.mem.set_u32(entries + 8 * index as u32, objects[object]);
            e.mem.set_u32(entries + 8 * index as u32 + 4, slot);
        }
        e.mem.set_u32(this + 0x14, entries);
        (Scene { e, this, storage }, log, allocator)
    }

    /// Record 1 names entries 1 and 2: slot 1 of object 0 and slot 2 of object 1.
    #[test]
    fn fn_00e3a4d0_copies_the_vector_into_every_named_slot() {
        let (mut s, _, _) = scene();
        let vector = s.e.mem.alloc(0x10);
        for word in 0..4u32 {
            s.e.mem.set_u32(vector + 4 * word, 0x700 + word);
        }
        s.e.call(0x00e3_a4d0, &args![s.this, 1i32, vector]);
        for word in 0..4u32 {
            assert_eq!(
                s.e.mem.u32(s.storage[0] + 0x50 + 0x20 + 4 * word),
                0x700 + word
            );
            assert_eq!(
                s.e.mem.u32(s.storage[1] + 0xa0 + 0x20 + 4 * word),
                0x700 + word
            );
            assert_eq!(s.e.mem.u32(s.storage[0] + 0x20 + 4 * word), 0);
        }
    }

    /// A record without entries does nothing.
    #[test]
    fn fn_00e3a4d0_empty_record_does_nothing() {
        let (mut s, _, _) = scene();
        let records = s.e.mem.u32(s.this + 8);
        s.e.mem.set_u32(records + 4, 0);
        let vector = s.e.mem.alloc(0x10);
        s.e.mem.set_u32(vector, 0x1234);
        s.e.call(0x00e3_a4d0, &args![s.this, 0i32, vector]);
        assert_eq!(s.e.mem.u32(s.storage[0] + 0x20), 0);
    }

    /// Of record 1's two words (field 3: slot 1 of object 0 at `0x40 + 12 + 0x50`, slot 2 of
    /// object 1 at `0x40 + 12 + 0xa0`) only the non-zero one is appended.
    #[test]
    fn fn_00e3a520_appends_the_non_null_words() {
        let (mut s, log, allocator) = scene();
        s.e.mem.set_u32(s.storage[0] + 0x40 + 12 + 0x50, 0);
        s.e.mem.set_u32(s.storage[1] + 0x40 + 12 + 0xa0, 0xcafe);
        let array = s.e.mem.alloc(0x10);
        let data = s.e.mem.alloc(0x40);
        s.e.mem.set_u32(array, data);
        s.e.mem.set_u32(array + 8, 4);
        s.e.call(0x00e3_a520, &args![s.this, 1i32, 3u32, array]);
        assert_eq!(s.e.mem.u32(array + 4), 1);
        assert_eq!(s.e.mem.u32(data), 0xcafe);
        assert!(log.borrow().is_empty());
        // Both words set, and a full array: the growth double runs for the second word.
        s.e.mem.set_u32(s.storage[0] + 0x40 + 12 + 0x50, 0xbeef);
        s.e.mem.set_u32(array + 4, 4);
        s.e.call(0x00e3_a520, &args![s.this, 1i32, 3u32, array]);
        assert_eq!(*log.borrow(), vec![(GROW, vec![allocator, array, 4])]);
        let grown = s.e.mem.u32(array);
        assert_eq!(s.e.mem.u32(array + 4), 6);
        assert_eq!(s.e.mem.u32(grown + 16), 0xbeef);
        assert_eq!(s.e.mem.u32(grown + 20), 0xcafe);
    }

    /// The destructor releases the non-null record objects, all array pointers and the three
    /// arrays, in the exe's order.
    #[test]
    fn fn_00e3ad80_releases_everything_in_order() {
        let (mut e, log, allocator) = engine();
        let this = e.mem.alloc(0x40);
        let records = e.mem.alloc(0x30);
        e.mem.set_u32(records + 8, 0xa1);
        e.mem.set_u32(records + 12 + 8, 0);
        e.mem.set_u32(this + 8, records);
        e.mem.set_u32(this + 0xc, 2);
        e.mem.set_u32(this + 0x10, 3);
        let pairs = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0x14, pairs);
        e.mem.set_u32(this + 0x18, 1);
        e.mem.set_u32(this + 0x1c, 2);
        let pointers = e.mem.alloc(0x10);
        e.mem.set_u32(pointers, 0xb1);
        e.mem.set_u32(pointers + 4, 0xb2);
        e.mem.set_u32(this + 0x20, pointers);
        e.mem.set_u32(this + 0x24, 2);
        e.mem.set_u32(this + 0x28, 4);
        e.call(0x00e3_ad80, &args![this]);
        assert_eq!(
            *log.borrow(),
            vec![
                (RELEASE, vec![0xa1]),
                (RELEASE, vec![0xb1]),
                (RELEASE, vec![0xb2]),
                (FREE_SLOT, vec![allocator, pointers, 16]),
                (FREE_SLOT, vec![allocator, pairs, 16]),
                (FREE_SLOT, vec![allocator, records, 36])
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0102_e388);
        for (data, size, capacity) in [(8, 0xc, 0x10), (0x14, 0x18, 0x1c), (0x20, 0x24, 0x28)] {
            assert_eq!(e.mem.u32(this + data), 0);
            assert_eq!(e.mem.u32(this + size), 0);
            assert_eq!(e.mem.u32(this + capacity), 0x8000_0000);
        }
    }

    /// Record 1 gives two objects (slot 1 of object 0, slot 2 of object 1) whose pointer words
    /// at field 3 are two objects of their own; each gets the averages `6 / 2` and `1 / 2`, and the
    /// collected array is freed (capacity 8 times 4 bytes).
    #[test]
    fn fn_00e3ae70_stores_the_averages() {
        let (mut s, log, allocator) = scene();
        let first = s.e.mem.alloc(0x20);
        let second = s.e.mem.alloc(0x20);
        s.e.mem.set_u32(s.storage[0] + 0x40 + 12 + 0x50, first);
        s.e.mem.set_u32(s.storage[1] + 0x40 + 12 + 0xa0, second);
        s.e.call(0x00e3_ae70, &args![s.this, 1i32, 3u32, 6.0f32, 1.0f32]);
        for object in [first, second] {
            assert_eq!(s.e.mem.f32(object + 0xc), 3.0);
            assert_eq!(s.e.mem.f32(object + 0x10), 0.5);
        }
        let frees: Vec<_> = log
            .borrow()
            .iter()
            .filter(|(a, _)| *a == FREE_SLOT)
            .cloned()
            .collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1[0], allocator);
        assert_eq!(frees[0].1[2], 32);
    }

    /// With no pointers collected nothing is divided or stored, and nothing is freed.
    #[test]
    fn fn_00e3ae70_without_pointers_does_nothing() {
        let (mut s, log, _) = scene();
        let marker = s.e.mem.alloc(0x20);
        s.e.mem.write(marker, &[0x33; 0x20]);
        s.e.call(0x00e3_ae70, &args![s.this, 0i32, 3u32, 6.0f32, 1.0f32]);
        assert_eq!(s.e.mem.u32(marker + 0xc), 0x3333_3333);
        assert!(log.borrow().iter().all(|(a, _)| *a != FREE_SLOT));
    }
}
