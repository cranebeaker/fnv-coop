//! `Havok SDK/run_00e02f30` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::hkaabbtree::{allocator, ARRAY_RESERVE_MORE, CAPACITY_MASK};

// Translated from 00e030c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: reads the word at `base + [cursor]`, advances the offset stored at
/// `cursor` by 4 and returns the word. The function takes two stack words and pops none
/// (`RET`).
pub fn fn_00e030c0(e: &mut Engine, base: u32, cursor: Ptr) -> u32 {
    let offset = e.mem.u32(cursor.addr());
    e.mem.set_u32(cursor.addr(), offset.wrapping_add(4));
    e.mem.u32(offset.wrapping_add(base))
}

// Translated from 00e02f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: skips a string in a packed table. The string starts at
/// `base + [cursor]`; the offset stored at `cursor` grows by its length plus the terminator,
/// rounded up to a multiple of 4 (the terminator always counts: a length of 3 adds 4, 4 adds
/// 8). Returns the address of the string. The function takes two stack words and pops none
/// (`RET`).
pub fn fn_00e02f30(e: &mut Engine, base: u32, cursor: Ptr) -> u32 {
    let offset = e.mem.u32(cursor.addr());
    let string = base.wrapping_add(offset);
    let mut length: u32 = 0;
    while e.mem.u8(string.wrapping_add(length)) != 0 {
        length = length.wrapping_add(1);
    }
    length = length.wrapping_add(1);
    while length & 3 != 0 {
        length = length.wrapping_add(1);
    }
    e.mem.set_u32(cursor.addr(), offset.wrapping_add(length));
    string
}

/// The loop of `00e02f60` and `00e03010`: walks the packed table that starts at
/// `[this + start]` (`start` is the offset of the field; the field after it is the end) read
/// through `00e030c0` and `00e02f30`, appending a pair (string address, value + `shift`) to the
/// 8-byte-element array at `array` for every entry until the offset reaches the table's size or
/// `00e030c0` answers `-1`. The array grows by `00c90940` when its size equals its capacity.
fn collect_strings(e: &mut Engine, this: u32, start: u32, shift: u32, array: u32) {
    let first = e.mem.u32(this + start);
    let size = e.mem.u32(this + start + 4).wrapping_sub(first) as i32;
    let table = first.wrapping_add(shift);
    if size <= 0 {
        return;
    }
    e.with_stack(4, |e, cursor_slot| {
        e.mem.set_u32(cursor_slot.addr(), 0);
        loop {
            let value = fn_00e030c0(e, table, cursor_slot);
            if value == u32::MAX {
                break;
            }
            let string = fn_00e02f30(e, table, cursor_slot);
            let heap = allocator(e);
            let capacity = e.mem.u32(array + 8) & CAPACITY_MASK;
            if e.mem.u32(array + 4) == capacity {
                e.call(ARRAY_RESERVE_MORE, &args![heap, array, 8u32]);
            }
            let count = e.mem.u32(array + 4);
            let slot = e.mem.u32(array).wrapping_add(count.wrapping_mul(8));
            e.mem.set_u32(array + 4, count.wrapping_add(1));
            e.mem.set_u32(slot, string);
            e.mem.set_u32(slot + 4, value.wrapping_add(shift));
            let cursor = e.mem.u32(cursor_slot.addr());
            let first = e.mem.u32(this + start);
            let remaining = e.mem.u32(this + start + 4).wrapping_sub(first) as i32;
            if (cursor as i32) >= remaining {
                break;
            }
        }
    });
}

// Translated from 00e02f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: reads a packed table of `(value, string)` entries between the
/// pointers at `this + 0x24` and `this + 0x28` (data base `shift` added to the first pointer) and
/// appends `(string address, value + shift)` pairs to the 8-byte-element `hkArray`-shaped array
/// at `array` (see the loop in `collect_strings`). Stops when the cursor passes the table or a
/// value of `-1` is read. The class is unconfirmed.
pub fn fn_00e02f60(e: &mut Engine, this: Ptr, shift: u32, array: Ptr) {
    collect_strings(e, this.addr(), 0x24, shift, array.addr());
}

// Translated from 00e03010 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map: the same as `00e02f60` for the table between the pointers at
/// `this + 0x28` and `this + 0x2c`. The class is unconfirmed.
pub fn fn_00e03010(e: &mut Engine, this: Ptr, shift: u32, array: Ptr) {
    collect_strings(e, this.addr(), 0x28, shift, array.addr());
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e0_2f30, fn_00e02f30(u32, Ptr) -> u32),
        entry!(0x00e0_2f60, fn_00e02f60(Ptr, u32, Ptr)),
        entry!(0x00e0_3010, fn_00e03010(Ptr, u32, Ptr)),
        entry!(0x00e0_30c0, fn_00e030c0(u32, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod tests_a05 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const ROUTER: u32 = 0x00c8_5750;
    const RESERVE_MORE: u32 = 0x00c9_0940;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A word is read at `base + offset`; the offset moves on by 4.
    #[test]
    fn fn_00e030c0_reads_a_word_and_advances() {
        let mut e = Engine::new();
        let base = e.mem.alloc(0x20);
        e.mem.set_u32(base + 8, 0xfeed_beef);
        let cursor = e.mem.alloc(4);
        e.mem.set_u32(cursor, 8);
        assert_eq!(e.call(0x00e0_30c0, &args![base, cursor]).u32(), 0xfeed_beef);
        assert_eq!(e.mem.u32(cursor), 12);
    }

    /// The offset grows by the length plus the terminator, rounded up to a multiple of 4; the
    /// string's address comes back.
    #[test]
    fn fn_00e02f30_skips_a_padded_string() {
        let mut e = Engine::new();
        let base = e.mem.alloc(0x40);
        let cursor = e.mem.alloc(4);
        // (text, expected step): the terminator always counts.
        for (text, step) in [
            (&b""[..], 4u32),
            (&b"a"[..], 4),
            (&b"abc"[..], 4),
            (&b"abcd"[..], 8),
            (&b"abcdefg"[..], 8),
            (&b"abcdefgh"[..], 12),
        ] {
            e.mem.write(base + 4, &[0xff; 0x20]);
            e.mem.set_cstr(base + 4, text);
            e.mem.set_u32(cursor, 4);
            assert_eq!(e.call(0x00e0_2f30, &args![base, cursor]).u32(), base + 4);
            assert_eq!(e.mem.u32(cursor), 4 + step, "{text:?}");
        }
    }

    struct Table {
        e: Engine,
        this: u32,
        array: u32,
        data: u32,
    }

    /// An engine with a table object at `this`: the table pointers at `+0x24`/`+0x28` (and the
    /// same data again at `+0x28`/`+0x2c` when `second`), an empty owned array with room for
    /// `room` entries, a router whose allocator is a double, and a `00c90940` double that logs.
    fn table(entries: &[(u32, &str)], second: bool, room: u32, shift: u32) -> (Table, Log) {
        let mut e = Engine::new();
        let data = e.mem.alloc(0x200);
        let mut at = 0u32;
        for (value, text) in entries {
            e.mem.set_u32(data + shift + at, *value);
            e.mem.set_cstr(data + shift + at + 4, text.as_bytes());
            at += 4 + ((text.len() as u32 + 1 + 3) & !3);
        }
        // The word after the last entry ends a walk that the shifted size would let go on.
        e.mem.set_u32(data + shift + at, u32::MAX);
        let this = e.mem.alloc(0x40);
        let (start, end) = if second { (0x28, 0x2c) } else { (0x24, 0x28) };
        e.mem.set_u32(this + start, data);
        e.mem.set_u32(this + end, data + shift + at);
        let array = e.mem.alloc(0x10);
        let storage = e.mem.alloc(0x100);
        e.mem.set_u32(array, storage);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, room);
        let allocator = e.mem.alloc(0x10);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let reserve_log = Rc::clone(&log);
        e.register_double(RESERVE_MORE, move |e, a| {
            reserve_log.borrow_mut().push(a.to_vec());
            // Make room for 8 more entries in place.
            e.mem.set_u32(a[1] + 8, 8);
            Ret::default()
        });
        (
            Table {
                e,
                this,
                array,
                data,
            },
            log,
        )
    }

    fn pairs(t: &Table) -> Vec<(Vec<u8>, u32)> {
        let count = t.e.mem.u32(t.array + 4);
        let data = t.e.mem.u32(t.array);
        (0..count)
            .map(|i| {
                let string = t.e.mem.u32(data + 8 * i);
                (t.e.mem.cstr(string), t.e.mem.u32(data + 8 * i + 4))
            })
            .collect()
    }

    /// Two entries are collected with the shift added to the values; the strings point into
    /// the table.
    #[test]
    fn fn_00e02f60_collects_the_entries() {
        let (mut t, log) = table(&[(0x10, "alpha"), (0x20, "be")], false, 4, 0x20);
        t.e.call(0x00e0_2f60, &args![t.this, 0x20u32, t.array]);
        assert_eq!(
            pairs(&t),
            vec![(b"alpha".to_vec(), 0x30), (b"be".to_vec(), 0x40)]
        );
        let first_string = t.e.mem.u32(t.e.mem.u32(t.array));
        assert_eq!(first_string, t.data + 0x20 + 4);
        assert!(log.borrow().is_empty());
    }

    /// A value of -1 stops the walk before its string is read.
    #[test]
    fn fn_00e02f60_stops_at_minus_one() {
        let (mut t, _) = table(
            &[(0x10, "alpha"), (u32::MAX, "zz"), (0x30, "c")],
            false,
            4,
            0,
        );
        t.e.call(0x00e0_2f60, &args![t.this, 0u32, t.array]);
        assert_eq!(pairs(&t), vec![(b"alpha".to_vec(), 0x10)]);
    }

    /// An empty table does nothing.
    #[test]
    fn fn_00e02f60_empty_table_does_nothing() {
        let (mut t, log) = table(&[], false, 4, 0);
        t.e.call(0x00e0_2f60, &args![t.this, 0u32, t.array]);
        assert_eq!(t.e.mem.u32(t.array + 4), 0);
        assert!(log.borrow().is_empty());
    }

    /// A full array is grown through `00c90940` (allocator, array, 8) before the pair is
    /// written.
    #[test]
    fn fn_00e02f60_grows_a_full_array() {
        let (mut t, log) = table(&[(0x10, "alpha"), (0x20, "be")], false, 1, 0);
        let allocator = {
            let router = t.e.call(ROUTER, &args![]).u32();
            t.e.mem.u32(router + 0x10)
        };
        t.e.call(0x00e0_2f60, &args![t.this, 0u32, t.array]);
        assert_eq!(*log.borrow(), vec![vec![allocator, t.array, 8]]);
        assert_eq!(pairs(&t).len(), 2);
    }

    /// The second variant reads the table between +0x28 and +0x2c.
    #[test]
    fn fn_00e03010_collects_the_entries_of_the_second_table() {
        let (mut t, _) = table(&[(0x10, "alpha"), (0x20, "be")], true, 4, 0x40);
        t.e.call(0x00e0_3010, &args![t.this, 0x40u32, t.array]);
        assert_eq!(
            pairs(&t),
            vec![(b"alpha".to_vec(), 0x50), (b"be".to_vec(), 0x60)]
        );
    }
}
