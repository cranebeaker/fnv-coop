//! `Havok SDK/run_00d61f30` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`, cdecl: the allocator, the array, the capacity,
/// the element size), `hkMemUtil::memMove` (Xbox PDB, `00d2cc70`, cdecl: destination, source,
/// byte count) and `hkStringBuf::operator=(const char*)` (Xbox PDB name `hkStringBuf::operator_`,
/// `00d61c10`, thiscall, one argument).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
const MEM_MOVE: u32 = 0x00d2_cc70;
const STRING_ASSIGN: u32 = 0x00d6_1c10;
/// The two string buffer routines the join goes through: `00d61cc0` (thiscall, no arguments:
/// makes the buffer the empty string, growing it to one byte when it holds none) and
/// `hkStringBuf::appendJoin` (Xbox PDB, `00d61e30`, thiscall, six words).
const STRING_CLEAR: u32 = 0x00d6_1cc0;
const STRING_APPEND_JOIN: u32 = 0x00d6_1e30;
/// The low 30 bits of an `hkArray` capacity word are the capacity.
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// Translated from 00d61f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the string buffer `this` (`00d61cc0`) and appends the join of the six words given
/// (`hkStringBuf::appendJoin`, Xbox PDB name of `00d61e30`; the six words are passed on as they
/// came), so the buffer holds exactly the joined text. The class of this function is not
/// confirmed (it behaves as a `join`).
// The exe takes the six join words as separate stack words.
#[allow(clippy::too_many_arguments)]
pub fn fn_00d61f30(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
) {
    e.call(STRING_CLEAR, &args![this]);
    e.call(
        STRING_APPEND_JOIN,
        &args![this, first, second, third, fourth, fifth, sixth],
    );
}

// Translated from 00d623a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs a string buffer from a pointer-with-a-flag word (`source` points at a word whose
/// bit 0 is a flag): the data pointer is the buffer's own inline storage at `this + 0xc`, the
/// size is 0 and the capacity word is `0x80000080` (128 bytes, storage not owned), then the text
/// the word points at (with bit 0 cleared) is assigned (`hkStringBuf::operator=`, `00d61c10`).
/// Answers `this`.
pub fn fn_00d623a0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let at = this.addr();
    e.mem.set_u32(at, at.wrapping_add(0xc));
    e.mem.set_u32(at.wrapping_add(4), 0);
    e.mem.set_u32(at.wrapping_add(8), 0x8000_0080);
    let word = e.mem.u32(source.addr());
    e.call(STRING_ASSIGN, &args![this, word & 0xffff_fffe]);
    this
}

// Translated from 00d625a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces `removed` characters at `start` of the string buffer `this` by `inserted_count`
/// characters read from `inserted`: the new size is `size - removed + inserted_count`; when it is
/// above the capacity (low 30 bits, signed compare) the array is reserved (`hkArrayUtil::_reserve`
/// with `allocator`, the larger of twice the capacity and the new size, element size 1); the
/// tail behind the removed part (`size - start - removed` bytes) moves to
/// `start + inserted_count` (`hkMemUtil::memMove`); the new characters are copied forward one
/// at a time to `start`; the size is stored.
pub fn fn_00d625a0(
    e: &mut Engine,
    this: Ptr,
    allocator: u32,
    start: i32,
    removed: i32,
    inserted: u32,
    inserted_count: i32,
) {
    let at = this.addr();
    let size = e.mem.i32(at.wrapping_add(4));
    let tail = size.wrapping_sub(start).wrapping_sub(removed);
    let new_size = size.wrapping_sub(removed).wrapping_add(inserted_count);
    let capacity = (e.mem.u32(at.wrapping_add(8)) & CAPACITY_MASK) as i32;
    if capacity < new_size {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = doubled.max(new_size);
        e.call(ARRAY_RESERVE, &args![allocator, this, wanted, 1u32]);
    }
    let data = e.mem.u32(at).wrapping_add(start as u32);
    e.call(
        MEM_MOVE,
        &args![
            data.wrapping_add(inserted_count as u32),
            data.wrapping_add(removed as u32),
            tail
        ],
    );
    let target = e.mem.u32(at).wrapping_add(start as u32);
    if inserted_count > 0 {
        for index in 0..inserted_count as u32 {
            let byte = e.mem.u8(inserted.wrapping_add(index));
            e.mem.set_u8(target.wrapping_add(index), byte);
        }
    }
    e.mem.set_u32(at.wrapping_add(4), new_size as u32);
}

// Translated from 00d62080 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a string buffer `append`): `this` in `ECX` is a string buffer whose
/// first three words are an `hkArray` of characters (data pointer, size including the terminating
/// 0, capacity word with the flag bits in the top two) and two stack words (`RET 8`): `text` and
/// `length`. A negative `length` is replaced by the length of `text` (`strlen`, `00cb10d0`). The size
/// becomes `size + length` (`size - 1` characters are kept, `length` are added, plus the 0). When
/// the capacity (the low 30 bits) is below that, `00c908c0` (`hkArrayUtil::_reserve`) grows the
/// array to the larger of twice the capacity and the new size, with the allocator at +0xc of the
/// memory router's structure (`00c85750`) and elements of 1 byte. Then the new terminating 0 is
/// written, the size stored and `length` bytes of `text` copied (`00d2caa0`) after the old last
/// character. Returns nothing. Ghidra made no function here; it is called from 00d6220e.
pub fn fn_00d62080(e: &mut Engine, this: u32, text: u32, length: i32) {
    let mut length = length;
    if length < 0 {
        length = e.call(0x00cb_10d0, &args![text]).i32();
    }
    let kept = e.mem.i32(this + 4).wrapping_sub(1);
    let last = kept.wrapping_add(length);
    let new_size = last.wrapping_add(1);
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0xc);
    let capacity = (e.mem.u32(this + 8) & 0x3fff_ffff) as i32;
    if capacity < new_size {
        let mut wanted = capacity.wrapping_add(capacity);
        if new_size >= wanted {
            wanted = new_size;
        }
        e.call(0x00c9_08c0, &args![allocator, this, wanted as u32, 1u32]);
    }
    let data = e.mem.u32(this);
    e.mem.set_u32(this + 4, new_size as u32);
    e.mem.set_u8(data.wrapping_add(last as u32), 0);
    let data = e.mem.u32(this);
    e.call(
        0x00d2_caa0,
        &args![data.wrapping_add(kept as u32), text, length as u32],
    );
}
// Translated from 00d62190 (decompiled, FalloutNV.exe 1.4.0.525)
/// No name in the engine map (a string buffer `replace`): `this` in `ECX` is a string buffer (the
/// layout of `00d62080`) and three stack words (`RET 0xc`): `from`, `to` (C strings) and `all`.
/// Replaces the first (or, when `all` is not 0, every) occurrence of `from` by `to`, searching with
/// `00d61480(from, start, 0x7fffffff)` (the index of the match, -1 when there is none).
///
/// When `to` is longer than `from` the work goes through a temporary string buffer (140 bytes: the
/// array words and 128 bytes of inline storage): `00d61b80` copies the buffer into it, `00d61cc0`
/// empties the buffer, and for each match the text before it and then `to` are appended with
/// `00d62080`, the rest of the text after the last match last; the temporary's storage is freed
/// through the allocator at +0xc of the memory router's structure (virtual function at +0x10, with
/// the data pointer and the capacity in bytes) when it is owned. The answer is 1 when a match
/// was found, else 0. Otherwise the buffer is rewritten in place, reading and writing the same
/// character array: the text before each match, `to` and finally the tail are moved down, the
/// terminating 0 is written, the size set to the written length plus 1 (growing the capacity
/// through `00c908c0` first when it is below that) and the answer is always 0. Ghidra made no function here; it is called from 00d2b38c.
pub fn fn_00d62190(e: &mut Engine, this: u32, from: u32, to: u32, all: u32) -> u32 {
    let from_length = e.call(0x00cb_10d0, &args![from]).i32();
    let to_length = e.call(0x00cb_10d0, &args![to]).i32();
    if to_length > from_length {
        return e.with_stack(0x8c, |e, temp| {
            let temp = temp.addr();
            e.call(0x00d6_1b80, &args![temp, this]);
            e.call(0x00d6_1cc0, &args![this]);
            let mut found = 0u32;
            let mut start = 0i32;
            let mut index = e
                .call(
                    0x00d6_1480,
                    &args![temp, from, start as u32, 0x7fff_ffffu32],
                )
                .i32();
            if index >= 0 {
                loop {
                    found = 1;
                    let data = e.mem.u32(temp);
                    fn_00d62080(
                        e,
                        this,
                        data.wrapping_add(start as u32),
                        index.wrapping_sub(start),
                    );
                    fn_00d62080(e, this, to, to_length);
                    start = index.wrapping_add(from_length);
                    if all == 0 {
                        break;
                    }
                    index = e
                        .call(
                            0x00d6_1480,
                            &args![temp, from, start as u32, 0x7fff_ffffu32],
                        )
                        .i32();
                    if index < 0 {
                        break;
                    }
                }
            }
            let size = e.mem.i32(temp + 4);
            let data = e.mem.u32(temp);
            let rest = size.wrapping_sub(start).wrapping_sub(1);
            fn_00d62080(e, this, data.wrapping_add(start as u32), rest);
            let router = e.call(0x00c8_5750, &args![]).u32();
            let allocator = e.mem.u32(router + 0xc);
            let capacity = e.mem.u32(temp + 8);
            e.mem.set_u32(temp + 4, 0);
            if (capacity as i32) >= 0 {
                let storage = e.mem.u32(temp);
                e.vcall(allocator, 0x10, &args![storage, capacity & 0x3fff_ffff]);
            }
            found
        });
    }
    // In place: `data` is read once, before the first search.
    let data = e.mem.u32(this);
    let mut read = 0i32;
    let mut write = 0i32;
    let mut index = e
        .call(0x00d6_1480, &args![this, from, 0u32, 0x7fff_ffffu32])
        .i32();
    if index != -1 {
        loop {
            while read < index {
                let byte = e.mem.u8(data.wrapping_add(read as u32));
                e.mem.set_u8(data.wrapping_add(write as u32), byte);
                read += 1;
                write += 1;
            }
            if to_length > 0 {
                let mut n = 0i32;
                loop {
                    let byte = e.mem.u8(to.wrapping_add(n as u32));
                    e.mem.set_u8(data.wrapping_add(write as u32), byte);
                    n += 1;
                    write += 1;
                    if n >= to_length {
                        break;
                    }
                }
            }
            read = read.wrapping_add(from_length);
            if all == 0 {
                break;
            }
            let start = index.wrapping_add(from_length);
            index = e
                .call(
                    0x00d6_1480,
                    &args![this, from, start as u32, 0x7fff_ffffu32],
                )
                .i32();
            if index == -1 {
                break;
            }
        }
    }
    if read < e.mem.i32(this + 4).wrapping_sub(1) {
        loop {
            let byte = e.mem.u8(data.wrapping_add(read as u32));
            e.mem.set_u8(data.wrapping_add(write as u32), byte);
            read += 1;
            write += 1;
            if read >= e.mem.i32(this + 4).wrapping_sub(1) {
                break;
            }
        }
    }
    e.mem.set_u8(data.wrapping_add(write as u32), 0);
    let new_size = write.wrapping_add(1);
    let router = e.call(0x00c8_5750, &args![]).u32();
    let allocator = e.mem.u32(router + 0xc);
    let capacity = (e.mem.u32(this + 8) & 0x3fff_ffff) as i32;
    if capacity < new_size {
        let mut wanted = capacity.wrapping_add(capacity);
        if new_size >= wanted {
            wanted = new_size;
        }
        e.call(0x00c9_08c0, &args![allocator, this, wanted as u32, 1u32]);
    }
    let data = e.mem.u32(this);
    e.mem.set_u32(this + 4, new_size as u32);
    e.mem.set_u8(data.wrapping_add(write as u32), 0);
    0
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d6_1f30, fn_00d61f30(Ptr, u32, u32, u32, u32, u32, u32)),
        entry!(0x00d6_23a0, fn_00d623a0(Ptr, Ptr) -> Ptr),
        entry!(0x00d6_25a0, fn_00d625a0(Ptr, u32, i32, i32, u32, i32)),
        entry!(0x00d6_2080, fn_00d62080(u32, u32, i32)),
        entry!(0x00d6_2190, fn_00d62190(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod string_edit_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn record(e: &mut Engine, log: &Log, address: u32) {
        let seen = Rc::clone(log);
        e.register_double(address, move |_, a| {
            seen.borrow_mut().push((address, a.to_vec()));
            Ret::default()
        });
    }

    #[test]
    fn fn_00d61f30_clears_then_appends_the_join() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        record(&mut e, &log, STRING_CLEAR);
        record(&mut e, &log, STRING_APPEND_JOIN);
        e.call(
            0x00d6_1f30,
            &args![0x100u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32],
        );
        assert_eq!(
            *log.borrow(),
            vec![
                (STRING_CLEAR, vec![0x100]),
                (STRING_APPEND_JOIN, vec![0x100, 1, 2, 3, 4, 5, 6]),
            ]
        );
    }

    #[test]
    fn fn_00d623a0_sets_up_inline_storage_and_assigns_the_text() {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        record(&mut e, &log, STRING_ASSIGN);
        let this = e.mem.alloc(0x100);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 0x1235);
        let got = e.call(0x00d6_23a0, &args![this, source]).ptr::<()>();
        assert_eq!(got.addr(), this);
        assert_eq!(e.mem.u32(this), this + 0xc);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0x8000_0080);
        assert_eq!(*log.borrow(), vec![(STRING_ASSIGN, vec![this, 0x1234])]);
    }

    /// A string buffer of `text` (size counts the terminator) in `capacity` bytes, and a
    /// double for `memMove` that copies like `memmove`, one for `_reserve` that grows the
    /// storage.
    fn setup(text: &[u8], capacity: u32) -> (Engine, u32, Log) {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let this = e.mem.alloc(0x10);
        let data = e.mem.alloc(capacity.max(text.len() as u32 + 1));
        e.mem.write(data, text);
        e.mem.set_u32(this, data);
        e.mem.set_u32(this + 4, text.len() as u32 + 1);
        e.mem.set_u32(this + 8, capacity);
        let seen = Rc::clone(&log);
        e.register_double(MEM_MOVE, move |e, a| {
            seen.borrow_mut().push((MEM_MOVE, a.to_vec()));
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            seen.borrow_mut().push((ARRAY_RESERVE, a.to_vec()));
            let array = a[1];
            let bytes = e.mem.bytes(e.mem.u32(array), e.mem.u32(array + 4));
            let data = e.mem.alloc(a[2]);
            e.mem.write(data, &bytes);
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, a[2]);
            Ret::default()
        });
        (e, this, log)
    }

    fn contents(e: &Engine, this: u32) -> Vec<u8> {
        e.mem
            .bytes(e.mem.u32(this), e.mem.u32(this + 4).saturating_sub(1))
    }

    fn put_text(e: &mut Engine, text: &[u8]) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(at, text);
        at
    }

    #[test]
    fn fn_00d625a0_replaces_a_range_within_the_capacity() {
        let (mut e, this, log) = setup(b"hello world", 32);
        let new = put_text(&mut e, b"big");
        // "world" (5 characters at 6) becomes "big": only the terminator is moved.
        e.call(0x00d6_25a0, &args![this, 0x77u32, 6i32, 5i32, new, 3i32]);
        assert_eq!(contents(&e, this), b"hello big");
        assert_eq!(e.mem.u32(this + 4), 10);
        assert!(log.borrow().iter().all(|(a, _)| *a != ARRAY_RESERVE));
        // Replace two characters at 3 of "hello" by four:
        // the terminator moves up.
        let (mut e, this, log) = setup(b"hello", 32);
        let new = put_text(&mut e, b"p!!!");
        e.call(0x00d6_25a0, &args![this, 0x77u32, 3i32, 2i32, new, 4i32]);
        assert_eq!(contents(&e, this), b"help!!!");
        let data = e.mem.u32(this);
        // The tail (size 6 - start 3 - removed 2 = 1 byte, the terminator) moves from
        // start + removed to start + inserted.
        assert_eq!(
            *log.borrow(),
            vec![(MEM_MOVE, vec![data + 3 + 4, data + 3 + 2, 1])]
        );
    }

    #[test]
    fn fn_00d625a0_grows_to_the_larger_of_double_and_the_new_size() {
        // Capacity 4, size 4: doubling gives 8; the new size 4 + 10 = 14 is larger.
        let (mut e, this, log) = setup(b"abc", 4);
        let new = put_text(&mut e, b"0123456789");
        e.call(0x00d6_25a0, &args![this, 0x77u32, 3i32, 0i32, new, 10i32]);
        assert_eq!(log.borrow()[0], (ARRAY_RESERVE, vec![0x77, this, 14, 1]));
        assert_eq!(contents(&e, this), b"abc0123456789");
        // Capacity 5, size 6: doubling gives 10; the new size 8 is smaller, so 10 is asked for.
        let (mut e, this, log) = setup(b"abcde", 5);
        let new = put_text(&mut e, b"xy");
        e.call(0x00d6_25a0, &args![this, 0x78u32, 5i32, 0i32, new, 2i32]);
        assert_eq!(log.borrow()[0], (ARRAY_RESERVE, vec![0x78, this, 10, 1]));
        assert_eq!(contents(&e, this), b"abcdexy");
    }

    #[test]
    fn fn_00d625a0_inserting_nothing_only_closes_the_gap() {
        let (mut e, this, _) = setup(b"abcdef", 16);
        e.call(0x00d6_25a0, &args![this, 0u32, 1i32, 2i32, 0u32, 0i32]);
        assert_eq!(contents(&e, this), b"adef");
        assert_eq!(e.mem.u32(this + 4), 5);
    }
}

#[cfg(test)]
mod tests_a01 {
    use crate::prelude::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::fn_00d62080;

    /// A one-byte-element reserve double (`00c908c0`: allocator, array, capacity, element size) that
    /// moves the contents to a new block and logs its arguments, a memory router whose structure has
    /// an allocator word at +0xc, and the C string helpers as doubles.
    type ReserveLog = Rc<RefCell<Vec<Vec<u32>>>>;

    fn string_engine() -> (Engine, u32, ReserveLog) {
        let mut e = Engine::new();
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0xc, 0xa110c);
        e.register_double(0x00c8_5750, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let reserves: ReserveLog = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&reserves);
        e.register_double(0x00c9_08c0, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            let block = e.mem.alloc(a[2]);
            let bytes = e.mem.bytes(old, size);
            e.mem.write(block, &bytes);
            e.mem.set_u32(a[1], block);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        e.register_double(0x00cb_10d0, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        e.register_double(0x00d2_caa0, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        (e, router, reserves)
    }

    /// A string buffer holding `text` (terminator included in the size) with room for `capacity` bytes
    /// and the capacity word `flags | capacity`.
    fn buffer_with(e: &mut Engine, text: &[u8], capacity: u32, flags: u32) -> u32 {
        let buffer = e.mem.alloc(0x10);
        let data = e.mem.alloc(capacity.max(1));
        e.mem.write(data, text);
        e.mem.set_u8(data + text.len() as u32, 0);
        e.mem.set_u32(buffer, data);
        e.mem.set_u32(buffer + 4, text.len() as u32 + 1);
        e.mem.set_u32(buffer + 8, flags | capacity);
        buffer
    }

    fn text_of(e: &Engine, buffer: u32) -> Vec<u8> {
        e.mem.cstr(e.mem.u32(buffer))
    }

    /// With room, nothing is reserved and the characters are appended in place.
    #[test]
    fn fn_00d62080_appends_in_place_when_there_is_room() {
        let (mut e, _, reserves) = string_engine();
        let buffer = buffer_with(&mut e, b"abc", 16, 0x8000_0000);
        let text = e.mem.alloc(8);
        e.mem.set_cstr(text, b"xyz");
        fn_00d62080(&mut e, buffer, text, 2);
        assert!(reserves.borrow().is_empty());
        assert_eq!(text_of(&e, buffer), b"abcxy");
        assert_eq!(e.mem.u32(buffer + 4), 6);
    }

    /// A negative length means the length of the text; a full buffer is reserved at twice the capacity
    /// (or the new size when that is larger) through the allocator at +0xc.
    #[test]
    fn fn_00d62080_reserves_twice_the_capacity_or_the_new_size() {
        let (mut e, _, reserves) = string_engine();
        let buffer = buffer_with(&mut e, b"abc", 4, 0);
        let text = e.mem.alloc(16);
        e.mem.set_cstr(text, b"defg");
        e.call(0x00d6_2080, &args![buffer, text, -1i32]);
        assert_eq!(text_of(&e, buffer), b"abcdefg");
        assert_eq!(*reserves.borrow(), vec![vec![0xa110c, buffer, 8, 1]]);
        // Twice the capacity (8) is below the new size 12: the new size is asked for.
        e.mem.set_cstr(text, b"hijklmnop");
        e.call(0x00d6_2080, &args![buffer, text, 9i32]);
        assert_eq!(text_of(&e, buffer), b"abcdefghijklmnop");
        assert_eq!(reserves.borrow()[1][2], 17);
    }
    /// Doubles for the search (`strstr` from `start`), the copy constructor of the temporary buffer
    /// (inline storage of 128 bytes) and the emptying function.
    fn replace_engine() -> Engine {
        let (mut e, _, _) = string_engine();
        e.register_double(0x00d6_1480, |e, a| {
            let text = e.mem.cstr(e.mem.u32(a[0]));
            let needle = e.mem.cstr(a[1]);
            let start = a[2] as usize;
            let hit = text.get(start..).and_then(|tail| {
                tail.windows(needle.len())
                    .position(|w| w == needle.as_slice())
            });
            Ret {
                eax: hit.map_or(u32::MAX, |p| (p + start) as u32),
                ..Ret::default()
            }
        });
        e.register_double(0x00d6_1b80, |e, a| {
            // temp: data = temp + 0xc (inline), the source's size and bytes, capacity word 0x80000080.
            let size = e.mem.u32(a[1] + 4);
            let bytes = e.mem.bytes(e.mem.u32(a[1]), size);
            e.mem.write(a[0] + 0xc, &bytes);
            e.mem.set_u32(a[0], a[0] + 0xc);
            e.mem.set_u32(a[0] + 4, size);
            e.mem.set_u32(a[0] + 8, 0x8000_0080);
            Ret::default()
        });
        e.register_double(0x00d6_1cc0, |e, a| {
            let data = e.mem.u32(a[0]);
            e.mem.set_u8(data, 0);
            e.mem.set_u32(a[0] + 4, 1);
            Ret::default()
        });
        e
    }

    fn replace(e: &mut Engine, buffer: u32, from: &[u8], to: &[u8], all: u32) -> u32 {
        let words = e.mem.alloc(0x40);
        e.mem.set_cstr(words, from);
        e.mem.set_cstr(words + 0x20, to);
        e.call(0x00d6_2190, &args![buffer, words, words + 0x20, all])
            .u32()
    }

    /// A longer replacement goes through the temporary buffer: first match only, or all of them.
    #[test]
    fn fn_00d62190_replaces_with_a_longer_text() {
        let mut e = replace_engine();
        let buffer = buffer_with(&mut e, b"a-b-c", 64, 0x8000_0000);
        assert_eq!(replace(&mut e, buffer, b"-", b"123", 0), 1);
        assert_eq!(text_of(&e, buffer), b"a123b-c");
        let buffer = buffer_with(&mut e, b"a-b-c", 64, 0x8000_0000);
        assert_eq!(replace(&mut e, buffer, b"-", b"123", 1), 1);
        assert_eq!(text_of(&e, buffer), b"a123b123c");
        assert_eq!(e.mem.u32(buffer + 4), 10);
    }

    /// No match: the text comes back unchanged and the answer is 0.
    #[test]
    fn fn_00d62190_without_a_match_keeps_the_text() {
        let mut e = replace_engine();
        let buffer = buffer_with(&mut e, b"abc", 16, 0x8000_0000);
        assert_eq!(replace(&mut e, buffer, b"zz", b"1234", 1), 0);
        assert_eq!(text_of(&e, buffer), b"abc");
    }

    /// An equal or shorter replacement is done in place and always answers 0.
    #[test]
    fn fn_00d62190_replaces_in_place_with_a_shorter_or_equal_text() {
        let mut e = replace_engine();
        let buffer = buffer_with(&mut e, b"a--b--c", 16, 0x8000_0000);
        assert_eq!(replace(&mut e, buffer, b"--", b"+", 1), 0);
        assert_eq!(text_of(&e, buffer), b"a+b+c");
        assert_eq!(e.mem.u32(buffer + 4), 6);
        let buffer = buffer_with(&mut e, b"a--b--c", 16, 0x8000_0000);
        assert_eq!(replace(&mut e, buffer, b"--", b"", 0), 0);
        assert_eq!(text_of(&e, buffer), b"ab--c");
        let buffer = buffer_with(&mut e, b"a-b-c", 16, 0x8000_0000);
        assert_eq!(replace(&mut e, buffer, b"-", b"/", 1), 0);
        assert_eq!(text_of(&e, buffer), b"a/b/c");
    }

    /// No match in place: the whole text is moved onto itself.
    #[test]
    fn fn_00d62190_in_place_without_a_match_keeps_the_text() {
        let mut e = replace_engine();
        let buffer = buffer_with(&mut e, b"abc", 16, 0x8000_0000);
        assert_eq!(replace(&mut e, buffer, b"zz", b"y", 1), 0);
        assert_eq!(text_of(&e, buffer), b"abc");
        assert_eq!(e.mem.u32(buffer + 4), 4);
    }
}
