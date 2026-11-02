//! `Havok SDK/run_00e31740` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::{ARRAY_RESERVE_MORE, CAPACITY_MASK, MEMORY_ROUTER};

/// Per kind: the alignment mask (`010eb3e8`) the padding of `00e318a0` fills up to.
const MASK_TABLE: u32 = 0x010e_b3e8;

/// Appends one byte to the byte `hkArray` at `array` the way the exe inlines it: the memory
/// router is asked first; when the size has reached the capacity (masked with `0x3fffffff`) the
/// array grows through `00c90940` (allocator, array, element size 1); then the byte is stored
/// at data + size and the size goes up by one.
fn push_byte(e: &mut Engine, array: u32, value: u8) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
    if e.mem.u32(array + 4) == capacity {
        let allocator = e.mem.u32(router + 0x10);
        e.call(ARRAY_RESERVE_MORE, &args![allocator, array, 1u32]);
    }
    let data = e.mem.u32(array);
    let size = e.mem.u32(array + 4);
    e.mem.set_u8(data.wrapping_add(size), value);
    e.mem.set_u32(array + 4, size.wrapping_add(1));
}

// Translated from 00e31740 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: walks a field with a length block. Does nothing when the low byte
/// of `flags` is zero. Otherwise `00e31570(flags, cursor)` reads the length, `00e315a0(flags,
/// length, extra, cursor)` steps over the selected fields, and the offset at `cursor` is rounded
/// up to 4. The function takes three stack words and pops none (`RET`) and returns nothing.
pub fn fn_00e31740(e: &mut Engine, flags: u8, extra: u32, cursor: Ptr) {
    if flags == 0 {
        return;
    }
    let length = e.call(0x00e3_1570, &args![flags, cursor]).u32();
    e.call(0x00e3_15a0, &args![flags, length, extra, cursor]);
    let at = e.mem.u32(cursor.addr());
    e.mem
        .set_u32(cursor.addr(), at.wrapping_add(3) & 0xffff_fffc);
}

// Translated from 00e31780 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the same as `00e31740` with `00e31010(flags, length, extra,
/// cursor)` (`extra` is the kind) in place of `00e315a0`. The function takes three stack words
/// and pops none (`RET`) and returns nothing.
pub fn fn_00e31780(e: &mut Engine, flags: u8, extra: u32, cursor: Ptr) {
    if flags == 0 {
        return;
    }
    let length = e.call(0x00e3_1570, &args![flags, cursor]).u32();
    e.call(0x00e3_1010, &args![flags, length, extra, cursor]);
    let at = e.mem.u32(cursor.addr());
    e.mem
        .set_u32(cursor.addr(), at.wrapping_add(3) & 0xffff_fffc);
}

// Translated from 00e317c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: appends the byte `value` to the byte `hkArray` (data, size,
/// capacity and flags) at `array`, growing it through `00c90940` when the size equals the
/// capacity. The function takes two stack words and pops none (`RET`) and returns nothing.
pub fn fn_00e317c0(e: &mut Engine, value: u8, array: Ptr) {
    push_byte(e, array.addr(), value);
}

// Translated from 00e31800 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: appends `count` zero bytes to the byte `hkArray` at `array` (one
/// at a time as `00e317c0` does); nothing when `count` is not above zero. The function takes two
/// stack words and pops none (`RET`) and returns nothing.
pub fn fn_00e31800(e: &mut Engine, count: i32, array: Ptr) {
    if count <= 0 {
        return;
    }
    let mut remaining = count;
    loop {
        push_byte(e, array.addr(), 0);
        remaining = remaining.wrapping_sub(1);
        if remaining == 0 {
            break;
        }
    }
}

// Translated from 00e31850 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: pads the byte `hkArray` at `array` with zero bytes until its size
/// is a multiple of `alignment` (`size & (alignment - 1)` is zero). The function takes two stack
/// words and pops none (`RET`) and returns nothing.
pub fn fn_00e31850(e: &mut Engine, alignment: u32, array: Ptr) {
    let mask = alignment.wrapping_sub(1);
    while e.mem.u32(array.addr() + 4) & mask != 0 {
        push_byte(e, array.addr(), 0);
    }
}

// Translated from 00e318a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: pads the byte `hkArray` at `array` with zero bytes until `size &
/// mask` is zero, the mask being entry `kind` of the table at `010eb3e8` (read again after every
/// byte). The function takes two stack words and pops none (`RET`) and returns nothing.
pub fn fn_00e318a0(e: &mut Engine, kind: u32, array: Ptr) {
    let entry = MASK_TABLE.wrapping_add(kind.wrapping_mul(4));
    while e.mem.u32(array.addr() + 4) & e.mem.u32(entry) != 0 {
        push_byte(e, array.addr(), 0);
    }
}

// Translated from 00e31900 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: byte-swaps (and steps over) the fields of a packed buffer
/// described by compact records (`this` in `ECX`). Does nothing when the word at `this + 0x80`
/// is zero or the count at `this + 0x2c` is not above zero. For each of the `[this + 0x2c]`
/// entries: a pointer into the buffer is `[[this + 0x44] + 4 * entry] + [this + 0x74]`, and a
/// stack cursor starts at that pointer plus `[this + 0x34]`; then for each of `[this + 0x10]`
/// four-byte records `b0 b1 b2 b3`: `00e31740(b1, b0 & 3, cursor)`, `00e31780(b2, (b0 >> 2) &
/// 0xf, cursor)` and `00e31740(b3, b0 >> 6, cursor)`; then for each of `[this + 0x14]` one-byte
/// records `b`: `00e31740(b & 0xf9, (b >> 1) & 3, cursor)`. The class is unconfirmed. The
/// function takes no stack words and returns nothing.
pub fn fn_00e31900(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x80) == 0 {
        return;
    }
    if (e.mem.u32(base + 0x2c) as i32) <= 0 {
        return;
    }
    e.with_stack(4, |e, cursor| {
        let mut entry: i32 = 0;
        loop {
            let table = e.mem.u32(base + 0x44);
            let offset = e
                .mem
                .u32(table.wrapping_add((entry as u32).wrapping_mul(4)));
            let mut at = offset.wrapping_add(e.mem.u32(base + 0x74));
            let start = at.wrapping_add(e.mem.u32(base + 0x34));
            e.mem.set_u32(cursor.addr(), start);
            if (e.mem.u32(base + 0x10) as i32) > 0 {
                let mut record: i32 = 0;
                loop {
                    let packed = e.mem.u8(at);
                    let first = e.mem.u8(at.wrapping_add(1));
                    fn_00e31740(e, first, u32::from(packed & 3), cursor);
                    let second = e.mem.u8(at.wrapping_add(2));
                    fn_00e31780(e, second, u32::from((packed >> 2) & 0xf), cursor);
                    let third = e.mem.u8(at.wrapping_add(3));
                    fn_00e31740(e, third, u32::from(packed >> 6), cursor);
                    record = record.wrapping_add(1);
                    at = at.wrapping_add(4);
                    if record >= e.mem.u32(base + 0x10) as i32 {
                        break;
                    }
                }
            }
            if (e.mem.u32(base + 0x14) as i32) > 0 {
                let mut record: i32 = 0;
                loop {
                    let packed = e.mem.u8(at);
                    at = at.wrapping_add(1);
                    fn_00e31740(e, packed & 0xf9, u32::from((packed >> 1) & 3), cursor);
                    record = record.wrapping_add(1);
                    if record >= e.mem.u32(base + 0x14) as i32 {
                        break;
                    }
                }
            }
            entry = entry.wrapping_add(1);
            if entry >= e.mem.u32(base + 0x2c) as i32 {
                break;
            }
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e3_1740, fn_00e31740(u8, u32, Ptr)),
        entry!(0x00e3_1780, fn_00e31780(u8, u32, Ptr)),
        entry!(0x00e3_17c0, fn_00e317c0(u8, Ptr)),
        entry!(0x00e3_1800, fn_00e31800(i32, Ptr)),
        entry!(0x00e3_1850, fn_00e31850(u32, Ptr)),
        entry!(0x00e3_18a0, fn_00e318a0(u32, Ptr)),
        entry!(0x00e3_1900, fn_00e31900(Ptr)),
    ]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// Doubles over `addresses` that log (address, words); `00c8_5750` answers a router
    /// whose allocator is `allocator`.
    fn engine(addresses: &[u32]) -> (Engine, Log, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        for &address in addresses {
            let calls = Rc::clone(&log);
            e.register_double(address, move |_, a| {
                calls.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        let allocator = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (e, log, allocator)
    }

    /// A byte array with `capacity` bytes of storage and `size` bytes used; the growth double
    /// `00c90940` makes it room for 64.
    fn byte_array(e: &mut Engine, size: u32, capacity: u32) -> u32 {
        let array = e.mem.alloc(0x10);
        let storage = e.mem.alloc(0x80);
        e.mem.set_u32(array, storage);
        e.mem.set_u32(array + 4, size);
        e.mem.set_u32(array + 8, capacity);
        array
    }

    fn contents(e: &Engine, array: u32) -> Vec<u8> {
        let data = e.mem.u32(array);
        (0..e.mem.u32(array + 4))
            .map(|i| e.mem.u8(data + i))
            .collect()
    }

    /// Bytes are appended; at full capacity the growth function runs first.
    #[test]
    fn fn_00e317c0_appends_and_grows_when_full() {
        let (mut e, log, allocator) = engine(&[]);
        let calls = Rc::clone(&log);
        e.register_double(0x00c9_0940, move |e, a| {
            calls.borrow_mut().push((0x00c9_0940, a.to_vec()));
            e.mem.set_u32(a[1] + 8, 64);
            Ret::default()
        });
        let array = byte_array(&mut e, 0, 2);
        e.call(0x00e3_17c0, &args![0x11u32, array]);
        e.call(0x00e3_17c0, &args![0x22u32, array]);
        assert!(log.borrow().is_empty());
        e.call(0x00e3_17c0, &args![0x33u32, array]);
        assert_eq!(
            *log.borrow(),
            vec![(0x00c9_0940, vec![allocator, array, 1])]
        );
        assert_eq!(contents(&e, array), vec![0x11, 0x22, 0x33]);
    }

    /// A positive count appends that many zero bytes; zero and negative counts nothing.
    #[test]
    fn fn_00e31800_appends_zero_bytes() {
        let (mut e, _, _) = engine(&[]);
        let array = byte_array(&mut e, 1, 64);
        let storage = e.mem.u32(array);
        e.mem.set_u8(storage, 0x55);
        e.mem.set_u8(storage + 1, 0x7f);
        e.call(0x00e3_1800, &args![0i32, array]);
        e.call(0x00e3_1800, &args![-3i32, array]);
        assert_eq!(e.mem.u32(array + 4), 1);
        e.call(0x00e3_1800, &args![3i32, array]);
        assert_eq!(contents(&e, array), vec![0x55, 0, 0, 0]);
    }

    /// Zero bytes up to a multiple of the alignment; nothing when already aligned.
    #[test]
    fn fn_00e31850_pads_to_the_alignment() {
        let (mut e, _, _) = engine(&[]);
        let array = byte_array(&mut e, 5, 64);
        e.call(0x00e3_1850, &args![4u32, array]);
        assert_eq!(e.mem.u32(array + 4), 8);
        e.call(0x00e3_1850, &args![4u32, array]);
        assert_eq!(e.mem.u32(array + 4), 8);
        e.call(0x00e3_1850, &args![16u32, array]);
        assert_eq!(e.mem.u32(array + 4), 16);
    }

    /// The mask comes from the table entry of the kind.
    #[test]
    fn fn_00e318a0_pads_with_the_table_mask() {
        let (mut e, _, _) = engine(&[]);
        e.mem.set_u32(0x010e_b3e8 + 4 * 2, 7);
        let array = byte_array(&mut e, 3, 64);
        e.call(0x00e3_18a0, &args![2u32, array]);
        assert_eq!(e.mem.u32(array + 4), 8);
    }

    /// A zero flags byte does nothing; otherwise the length comes from `00e31570`, the fields
    /// from `00e315a0`, and the cursor ends up aligned to 4.
    #[test]
    fn fn_00e31740_walks_a_field() {
        let (mut e, log, _) = engine(&[0x00e3_1570, 0x00e3_15a0]);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, 0x100);
        e.call(0x00e3_1740, &args![0u32, 5u32, cursor]);
        assert!(log.borrow().is_empty());
        e.register_double(0x00e3_1570, |_, _| Ret {
            eax: 9,
            ..Ret::default()
        });
        e.register_double(0x00e3_15a0, move |e, a| {
            assert_eq!(a[1..3], [9, 5]);
            e.mem.set_u32(a[3], 0x105);
            Ret::default()
        });
        e.call(0x00e3_1740, &args![0x11u32, 5u32, cursor]);
        assert_eq!(e.mem.u32(cursor), 0x108);
    }

    /// The second variant hands over to `00e31010`.
    #[test]
    fn fn_00e31780_walks_a_field_through_00e31010() {
        let (mut e, log, _) = engine(&[0x00e3_1570, 0x00e3_1010]);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, 0x100);
        e.call(0x00e3_1780, &args![0u32, 5u32, cursor]);
        assert!(log.borrow().is_empty());
        e.call(0x00e3_1780, &args![0x11u32, 5u32, cursor]);
        let log = log.borrow();
        assert_eq!(log[0], (0x00e3_1570, vec![0x11, cursor]));
        assert_eq!(log[1], (0x00e3_1010, vec![0x11, 0, 5, cursor]));
        assert_eq!(e.mem.u32(cursor), 0x100);
    }

    /// The descriptor: two entries, each with one four-byte record and one one-byte record.
    /// The calls come in the exe's order with the unpacked fields.
    #[test]
    fn fn_00e31900_unpacks_the_records() {
        let (mut e, log, _) = engine(&[0x00e3_1570, 0x00e3_15a0, 0x00e3_1010]);
        // Flags bytes chosen so that every call passes the first guard (non-zero).
        let data = e.mem.alloc(0x100);
        // Entry 0 at data + 0x10: record (b0 = 0b11_0101_10, 1, 2, 3), one-byte record 0x0f.
        e.mem.write(data + 0x10, &[0b1101_0110, 1, 2, 3, 0x0f]);
        // Entry 1 at data + 0x20: record (b0 = 0b01_0010_01, 4, 5, 6), one-byte record 0xf8.
        e.mem.write(data + 0x20, &[0b0100_1001, 4, 5, 6, 0xf8]);
        let offsets = e.mem.alloc(8);
        e.mem.set_u32(offsets, 0x10);
        e.mem.set_u32(offsets + 4, 0x20);
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x80, 1);
        e.mem.set_u32(this + 0x2c, 2);
        e.mem.set_u32(this + 0x44, offsets);
        e.mem.set_u32(this + 0x74, data);
        e.mem.set_u32(this + 0x34, 0x40);
        e.mem.set_u32(this + 0x10, 1);
        e.mem.set_u32(this + 0x14, 1);
        e.call(0x00e3_1900, &args![this]);
        // Four walked fields per entry: the flags byte of each is what `00e31570` sees.
        let flags: Vec<u32> = log
            .borrow()
            .iter()
            .filter(|(address, _)| *address == 0x00e3_1570)
            .map(|(_, words)| words[0])
            .collect();
        assert_eq!(flags, vec![1, 2, 3, 0x09, 4, 5, 6, 0xf8]);
        let list = log.borrow();
        let extras: Vec<(u32, u32)> = list
            .iter()
            .filter(|(address, _)| *address == 0x00e3_15a0 || *address == 0x00e3_1010)
            .map(|(address, words)| (*address, words[2]))
            .collect();
        assert_eq!(
            extras,
            vec![
                (0x00e3_15a0, 2),
                (0x00e3_1010, 5),
                (0x00e3_15a0, 3),
                (0x00e3_15a0, 3),
                (0x00e3_15a0, 1),
                (0x00e3_1010, 2),
                (0x00e3_15a0, 1),
                (0x00e3_15a0, 0)
            ]
        );
    }

    /// With the flag word zero, or no entries, nothing is walked.
    #[test]
    fn fn_00e31900_does_nothing_without_a_flag_or_entries() {
        let (mut e, log, _) = engine(&[0x00e3_1570]);
        let this = e.mem.alloc(0x90);
        e.mem.set_u32(this + 0x2c, 3);
        e.call(0x00e3_1900, &args![this]);
        e.mem.set_u32(this + 0x80, 1);
        e.mem.set_u32(this + 0x2c, 0);
        e.call(0x00e3_1900, &args![this]);
        assert!(log.borrow().is_empty());
    }
}
