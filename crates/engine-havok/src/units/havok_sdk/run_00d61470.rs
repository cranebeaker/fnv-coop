//! `Havok SDK/run_00d61470` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::run_00d51760::push_word_with;

/// The memory router (`00c85750`, no arguments) and `hkString::strLen` (`00cb10d0`, cdecl, one
/// pointer; answers the length).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
const STRING_LENGTH: u32 = 0x00cb_10d0;
/// The router field the string buffer functions take their allocator from (`+0xc`, not the
/// `+0x10` the array push-backs of other units use), and the low 30 bits of a capacity word.
const STRING_ALLOCATOR_SLOT: u32 = 0x0c;
const CAPACITY_MASK: u32 = 0x3fff_ffff;

// The functions of this unit work on a string buffer: a word for the data pointer at `+0`, the
// size at `+4` (the characters plus the terminating zero) and a capacity word at `+8` (the
// capacity in the low 30 bits, bit 31 set when the storage is not owned). The class
// (`hkStringBuf` in the Xbox PDB, which names the neighbouring functions) is not confirmed for
// these.

// Translated from 00d61470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Answers `this` (`MOV EAX,ECX`).
pub fn fn_00d61470(_e: &mut Engine, this: Ptr) -> Ptr {
    this
}

// Translated from 00d61600 (decompiled, FalloutNV.exe 1.4.0.525)
/// The index of the first character equal to `wanted` from `start` up to `end` (signed; the
/// end is cut to `size - 1` so the terminator is never looked at), or -1.
pub fn fn_00d61600(e: &mut Engine, this: Ptr, wanted: u8, start: i32, end: i32) -> i32 {
    let last = e.mem.i32(this.addr().wrapping_add(4)).wrapping_sub(1);
    let limit = end.min(last);
    if start >= limit {
        return -1;
    }
    let data = e.mem.u32(this.addr());
    let mut index = start;
    loop {
        if e.mem.u8(data.wrapping_add(index as u32)) == wanted {
            return index;
        }
        index = index.wrapping_add(1);
        if index >= limit {
            return -1;
        }
    }
}

// Translated from 00d61700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the buffer's text ends with the zero-terminated `suffix` (`hkString::strLen`
/// first; a suffix longer than the text gives 0, an empty one 1).
pub fn fn_00d61700(e: &mut Engine, this: Ptr, suffix: u32) -> bool {
    let length = e.call(STRING_LENGTH, &args![suffix]).i32();
    let size = e.mem.i32(this.addr().wrapping_add(4));
    if length > size.wrapping_sub(1) {
        return false;
    }
    let offset = size.wrapping_sub(length).wrapping_sub(1);
    if length > 0 {
        let data = e.mem.u32(this.addr()).wrapping_add(offset as u32);
        for index in 0..length as u32 {
            let ours = e.mem.u8(data.wrapping_add(index));
            let theirs = e.mem.u8(suffix.wrapping_add(index));
            if ours != theirs {
                return false;
            }
        }
    }
    true
}

// Translated from 00d61840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the characters equal to `find` by `replacement`, over the text without its
/// terminator: only the first one when `all` is zero, every one otherwise. Answers whether
/// anything was replaced.
pub fn fn_00d61840(e: &mut Engine, this: Ptr, find: u8, replacement: u8, all: u32) -> bool {
    let mut replaced = false;
    if e.mem.i32(this.addr().wrapping_add(4)).wrapping_sub(1) <= 0 {
        return false;
    }
    let mut index = 0u32;
    loop {
        let at = e.mem.u32(this.addr()).wrapping_add(index);
        if e.mem.u8(at) == find {
            e.mem.set_u8(at, replacement);
            replaced = true;
            if all == 0 {
                return true;
            }
        }
        index = index.wrapping_add(1);
        let last = e.mem.i32(this.addr().wrapping_add(4)).wrapping_sub(1);
        if (index as i32) >= last {
            return replaced;
        }
    }
}

/// The shared loop of `00d61890` and `00d618d0`: every character of the text (terminator
/// excluded) whose value minus `first` is at most 0x19 (an unsigned byte compare) moves by
/// `delta`.
fn shift_letters(e: &mut Engine, this: Ptr, first: u8, delta: i32) {
    if e.mem.i32(this.addr().wrapping_add(4)).wrapping_sub(1) <= 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let data = e.mem.u32(this.addr());
        let at = data.wrapping_add(index);
        let character = e.mem.i8(at);
        let mut value = i32::from(character);
        if (character as u8).wrapping_sub(first) <= 0x19 {
            value = value.wrapping_add(delta);
        }
        e.mem.set_u8(at, value as u8);
        index = index.wrapping_add(1);
        let last = e.mem.i32(this.addr().wrapping_add(4)).wrapping_sub(1);
        if (index as i32) >= last {
            return;
        }
    }
}

// Translated from 00d61890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lower-cases the text: every `A` to `Z` (the character minus 0x41 is at most 0x19, unsigned
/// byte compare) gets 0x20 added; the terminator is left alone.
pub fn fn_00d61890(e: &mut Engine, this: Ptr) {
    shift_letters(e, this, 0x41, 0x20);
}

// Translated from 00d618d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Upper-cases the text: every `a` to `z` gets 0x20 taken away; the terminator is left alone.
pub fn fn_00d618d0(e: &mut Engine, this: Ptr) {
    shift_letters(e, this, 0x61, -0x20);
}

// Translated from 00d61910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Splits the text at `separator`: pushes the data pointer on the pointer array `pieces` (data,
/// size, capacity word; a push-back that grows through the router's allocator at `+0xc`), then
/// for each separator found from index 1 on (`00d61600` with the end 0x7fffffff) zeroes it and
/// pushes the address after it. Answers the array's size at the end.
pub fn fn_00d61910(e: &mut Engine, this: Ptr, separator: u8, pieces: Ptr) -> i32 {
    let data = e.mem.u32(this.addr());
    push_word_with(e, pieces.addr(), data, STRING_ALLOCATOR_SLOT);
    let mut count = e.mem.i32(pieces.addr().wrapping_add(4));
    let mut found = fn_00d61600(e, this, separator, 1, 0x7fff_ffff);
    while found >= 0 {
        let data = e.mem.u32(this.addr());
        e.mem.set_u8(data.wrapping_add(found as u32), 0);
        let data = e.mem.u32(this.addr());
        let next = data.wrapping_add(found as u32).wrapping_add(1);
        push_word_with(e, pieces.addr(), next, STRING_ALLOCATOR_SLOT);
        count = e.mem.i32(pieces.addr().wrapping_add(4));
        found = fn_00d61600(e, this, separator, found.wrapping_add(1), 0x7fff_ffff);
    }
    count
}

// Translated from 00d619d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the text of the buffer `other` into this one: when this capacity (low 30 bits,
/// signed compare) is below `other`'s size, frees the storage if it is owned (capacity word not
/// negative; allocator slot `0x10`, the data and the capacity) and asks the allocator (slot
/// `0xc`) for `other`'s size in bytes, through a stack word that comes back with the size
/// granted (kept as the new capacity word); then sets the size and copies the bytes forward one
/// at a time. The allocator is the router's field at `+0xc`. Answers `this`.
pub fn fn_00d619d0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router.wrapping_add(STRING_ALLOCATOR_SLOT));
    let capacity = e.mem.u32(this.addr().wrapping_add(8));
    let masked = capacity & CAPACITY_MASK;
    if (masked as i32) < e.mem.i32(other.addr().wrapping_add(4)) {
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(this.addr());
            e.vcall(allocator, 0x10, &args![data, masked]);
        }
        let wanted = e.mem.u32(other.addr().wrapping_add(4));
        let (data, granted) = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), wanted);
            let data = e.vcall(allocator, 0xc, &args![slot]).u32();
            (data, e.mem.u32(slot.addr()))
        });
        e.mem.set_u32(this.addr(), data);
        e.mem.set_u32(this.addr().wrapping_add(8), granted);
    }
    let count = e.mem.u32(other.addr().wrapping_add(4));
    let target = e.mem.u32(this.addr());
    e.mem.set_u32(this.addr().wrapping_add(4), count);
    let source = e.mem.u32(other.addr());
    if (count as i32) > 0 {
        for index in 0..count {
            let byte = e.mem.u8(source.wrapping_add(index));
            e.mem.set_u8(target.wrapping_add(index), byte);
        }
    }
    this
}

// Translated from 00d61480 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, three stack words `needle`, `offset`, `_unused_2`, `RET 0xc`; no
/// direct caller; descriptions, not confirmed): `this` holds a string pointer in its first word.
/// Looks for `needle` in the string from `offset` with the CRT search `00cb1130` (cdecl,
/// `(string + offset, needle)`; a character search). Answers the position found, counted from the
/// start of the string, or -1 when the search answers null.
pub fn fn_00d61480(e: &mut Engine, this: Ptr, needle: u32, offset: u32, _unused_2: u32) -> i32 {
    let text = e.mem.u32(this.addr());
    let found = e
        .call(0x00cb_1130, &args![text.wrapping_add(offset), needle])
        .u32();
    if found == 0 {
        return -1;
    }
    found.wrapping_sub(e.mem.u32(this.addr())) as i32
}

// Translated from 00d61560 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one stack word `other`, `RET 4`; no direct caller; descriptions, not
/// confirmed): `this` holds a string pointer in its first word. Compares that string with the
/// string at `other` while neither has ended: a difference answers 0. When the loop ends it
/// answers 1 if `other` has ended (it is equal to the start of the string, or all of it), else
/// 0.
pub fn fn_00d61560(e: &mut Engine, this: Ptr, other: u32) -> u32 {
    let text = e.mem.u32(this.addr());
    let mut index = 0u32;
    if e.mem.u8(text) != 0 {
        loop {
            let wanted = e.mem.u8(other.wrapping_add(index));
            if wanted == 0 {
                break;
            }
            if e.mem.u8(text.wrapping_add(index)) != wanted {
                return 0;
            }
            index += 1;
            if e.mem.u8(text.wrapping_add(index)) == 0 {
                break;
            }
        }
    }
    u32::from(e.mem.u8(other.wrapping_add(index)) == 0)
}

// Translated from 00d617d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (`thiscall`, one stack word `count`, `RET 4`; no direct caller; descriptions, not
/// confirmed): `this` holds a string pointer in its first word and the length in the word at
/// `+4`. Nothing happens for a `count` of 0 or less (signed). Otherwise the length drops by
/// `count` and, when the new length is above 0 (signed), that many bytes are moved down from
/// `count` bytes later, a byte at a time from the front.
pub fn fn_00d617d0(e: &mut Engine, this: Ptr, count: i32) {
    if count <= 0 {
        return;
    }
    let length = e.mem.i32(this.addr().wrapping_add(4)).wrapping_sub(count);
    e.mem.set_i32(this.addr().wrapping_add(4), length);
    let text = e.mem.u32(this.addr());
    if length <= 0 {
        return;
    }
    let mut destination = text;
    for _ in 0..length {
        let byte = e.mem.u8(destination.wrapping_add(count as u32));
        e.mem.set_u8(destination, byte);
        destination = destination.wrapping_add(1);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d61480, fn_00d61480(Ptr, u32, u32, u32) -> i32),
        entry!(0x00d61560, fn_00d61560(Ptr, u32) -> u32),
        entry!(0x00d617d0, fn_00d617d0(Ptr, i32)),
        entry!(0x00d6_1470, fn_00d61470(Ptr) -> Ptr),
        entry!(0x00d6_1600, fn_00d61600(Ptr, u8, i32, i32) -> i32),
        entry!(0x00d6_1700, fn_00d61700(Ptr, u32) -> bool),
        entry!(0x00d6_1840, fn_00d61840(Ptr, u8, u8, u32) -> bool),
        entry!(0x00d6_1890, fn_00d61890(Ptr)),
        entry!(0x00d6_18d0, fn_00d618d0(Ptr)),
        entry!(0x00d6_1910, fn_00d61910(Ptr, u8, Ptr) -> i32),
        entry!(0x00d6_19d0, fn_00d619d0(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod string_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A string object: the first word points to `text` (zero ended), the word at `+4` is its
    /// length.
    fn string_object_00d61470(e: &mut Engine, text: &[u8]) -> (u32, u32) {
        let buffer = e.mem.alloc(text.len() as u32 + 8);
        e.mem.write(buffer, text);
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, buffer);
        e.mem.set_u32(object + 4, text.len() as u32);
        (object, buffer)
    }

    #[test]
    fn fn_00d61480_answers_the_position_counted_from_the_start() {
        let mut e = Engine::new();
        let (object, buffer) = string_object_00d61470(&mut e, b"hello world");
        e.register(0x00cb_1130, |_, a| Ret {
            eax: a[0] + 4,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let answer = e
            .call(0x00d6_1480, &args![object, 0x6fu32, 3u32, 0u32])
            .i32();
        assert_eq!(answer, 7);
        let seen: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00cb_1130)
            .cloned()
            .collect();
        assert_eq!(seen, vec![(0x00cb_1130, vec![buffer + 3, 0x6f])]);
    }

    #[test]
    fn fn_00d61480_answers_minus_one_when_the_search_finds_nothing() {
        let mut e = Engine::new();
        let (object, _) = string_object_00d61470(&mut e, b"hello");
        e.register(0x00cb_1130, |_, _| Ret::default());
        let answer = e
            .call(0x00d6_1480, &args![object, 0x7au32, 0u32, 0u32])
            .i32();
        assert_eq!(answer, -1);
    }

    #[test]
    fn fn_00d61560_tells_whether_the_other_string_starts_the_text() {
        let mut e = Engine::new();
        let (object, _) = string_object_00d61470(&mut e, b"hello");
        let (_, prefix) = string_object_00d61470(&mut e, b"hel");
        let (_, whole) = string_object_00d61470(&mut e, b"hello");
        let (_, longer) = string_object_00d61470(&mut e, b"hello!");
        let (_, different) = string_object_00d61470(&mut e, b"help");
        let (_, empty) = string_object_00d61470(&mut e, b"");
        assert_eq!(e.call(0x00d6_1560, &args![object, prefix]).u32(), 1);
        assert_eq!(e.call(0x00d6_1560, &args![object, whole]).u32(), 1);
        assert_eq!(e.call(0x00d6_1560, &args![object, longer]).u32(), 0);
        assert_eq!(e.call(0x00d6_1560, &args![object, different]).u32(), 0);
        assert_eq!(e.call(0x00d6_1560, &args![object, empty]).u32(), 1);
    }

    #[test]
    fn fn_00d61560_with_an_empty_text_is_true_only_for_an_empty_other() {
        let mut e = Engine::new();
        let (object, _) = string_object_00d61470(&mut e, b"");
        let (_, empty) = string_object_00d61470(&mut e, b"");
        let (_, some) = string_object_00d61470(&mut e, b"a");
        assert_eq!(e.call(0x00d6_1560, &args![object, empty]).u32(), 1);
        assert_eq!(e.call(0x00d6_1560, &args![object, some]).u32(), 0);
    }

    #[test]
    fn fn_00d617d0_removes_bytes_from_the_front() {
        let mut e = Engine::new();
        let (object, buffer) = string_object_00d61470(&mut e, b"abcdef");
        e.call(0x00d6_17d0, &args![object, 2i32]);
        assert_eq!(e.mem.u32(object + 4), 4);
        assert_eq!(e.mem.bytes(buffer, 4), b"cdef");
    }

    #[test]
    fn fn_00d617d0_ignores_a_count_of_zero_or_less() {
        let mut e = Engine::new();
        let (object, buffer) = string_object_00d61470(&mut e, b"abcdef");
        e.call(0x00d6_17d0, &args![object, 0i32]);
        e.call(0x00d6_17d0, &args![object, -3i32]);
        assert_eq!(e.mem.u32(object + 4), 6);
        assert_eq!(e.mem.bytes(buffer, 6), b"abcdef");
    }

    #[test]
    fn fn_00d617d0_empties_the_length_without_moving_when_everything_is_removed() {
        let mut e = Engine::new();
        let (object, buffer) = string_object_00d61470(&mut e, b"abc");
        e.call(0x00d6_17d0, &args![object, 5i32]);
        assert_eq!(e.mem.i32(object + 4), -2);
        assert_eq!(e.mem.bytes(buffer, 3), b"abc");
    }

    const ALLOCATOR_FREE: u32 = 0x0900_0001;
    const ALLOCATOR_ALLOCATE: u32 = 0x0900_0002;
    const ALLOCATOR_VTABLE: u32 = 0x0900_0100;
    const ROUTER: u32 = 0x0900_0200;
    const ALLOCATOR: u32 = 0x0900_0300;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine with the router (allocator at `+0xc`, whose vtable slot `0xc` hands out a new
    /// block and rounds the request up by 8, and slot `0x10` records frees), the string-length
    /// function and the array growth.
    fn engine() -> (Engine, Log) {
        let mut e = Engine::new();
        e.map(0x0900_0000, 0x1000);
        e.mem.set_u32(ROUTER + 0xc, ALLOCATOR);
        e.mem.set_u32(ALLOCATOR, ALLOCATOR_VTABLE);
        e.mem.set_u32(ALLOCATOR_VTABLE + 0xc, ALLOCATOR_ALLOCATE);
        e.mem.set_u32(ALLOCATOR_VTABLE + 0x10, ALLOCATOR_FREE);
        e.register(MEMORY_ROUTER, |_, _| Ret {
            eax: ROUTER,
            ..Ret::default()
        });
        e.register(STRING_LENGTH, |e, a| Ret {
            eax: e.mem.cstr(a[0]).len() as u32,
            ..Ret::default()
        });
        let log: Log = Rc::default();
        let seen = Rc::clone(&log);
        e.register_double(ALLOCATOR_FREE, move |_, a| {
            seen.borrow_mut().push((ALLOCATOR_FREE, a.to_vec()));
            Ret::default()
        });
        e.register(ALLOCATOR_ALLOCATE, |e, a| {
            let requested = e.mem.u32(a[1]);
            let granted = requested + 8;
            e.mem.set_u32(a[1], granted);
            Ret {
                eax: e.mem.alloc(granted),
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(0x00c9_0940, move |e, a| {
            seen.borrow_mut().push((0x00c9_0940, a.to_vec()));
            let (array, element) = (a[1], a[2]);
            let data = e.mem.alloc(8 * element);
            let size = e.mem.u32(array + 4);
            if size > 0 {
                let bytes = e.mem.bytes(e.mem.u32(array), size * element);
                e.mem.write(data, &bytes);
            }
            e.mem.set_u32(array, data);
            e.mem.set_u32(array + 8, 8);
            Ret::default()
        });
        (e, log)
    }

    /// A buffer holding `text` and its terminator in storage of `capacity` bytes.
    fn buffer(e: &mut Engine, text: &[u8], capacity: u32) -> Ptr {
        let this = Ptr::new(e.mem.alloc(0x10));
        let data = e
            .mem
            .alloc((capacity & 0x3fff_ffff).max(text.len() as u32 + 1));
        e.mem.write(data, text);
        e.mem.set_u32(this.addr(), data);
        e.mem.set_u32(this.addr() + 4, text.len() as u32 + 1);
        e.mem.set_u32(this.addr() + 8, capacity);
        this
    }

    fn text(e: &Engine, this: Ptr) -> Vec<u8> {
        let data = e.mem.u32(this.addr());
        let size = e.mem.u32(this.addr() + 4);
        e.mem.bytes(data, size)
    }

    #[test]
    fn fn_00d61470_answers_this() {
        let (mut e, _) = engine();
        assert_eq!(e.call(0x00d6_1470, &args![0x1234u32]).u32(), 0x1234);
    }

    #[test]
    fn fn_00d61600_finds_a_character_below_the_end() {
        let (mut e, _) = engine();
        let this = buffer(&mut e, b"a.b.c", 8);
        let find = |e: &mut Engine, start: i32, end: i32| {
            e.call(0x00d6_1600, &args![this, b'.' as u32, start, end])
                .i32()
        };
        assert_eq!(find(&mut e, 0, 100), 1);
        assert_eq!(find(&mut e, 2, 100), 3);
        // The end is exclusive and cut to the size minus 1 (the terminator).
        assert_eq!(find(&mut e, 2, 3), -1);
        assert_eq!(find(&mut e, 4, i32::MAX), -1);
        assert_eq!(find(&mut e, 5, i32::MAX), -1);
        // The terminator itself is never looked at.
        let nul = e
            .call(0x00d6_1600, &args![this, 0u32, 0i32, i32::MAX])
            .i32();
        assert_eq!(nul, -1);
    }

    #[test]
    fn fn_00d61700_compares_the_end_of_the_text() {
        let (mut e, _) = engine();
        let this = buffer(&mut e, b"hello", 8);
        let probe = |e: &mut Engine, suffix: &[u8]| {
            let at = e.mem.alloc(16);
            e.mem.set_cstr(at, suffix);
            e.call(0x00d6_1700, &args![this, at]).bool()
        };
        assert!(probe(&mut e, b"llo"));
        assert!(probe(&mut e, b"hello"));
        assert!(probe(&mut e, b""));
        assert!(!probe(&mut e, b"lo!"));
        assert!(!probe(&mut e, b"xhello"));
        assert!(!probe(&mut e, b"hallo"));
    }

    #[test]
    fn fn_00d61840_replaces_the_first_or_every_match() {
        let (mut e, _) = engine();
        let this = buffer(&mut e, b"a-b-c", 8);
        let got = e.call(0x00d6_1840, &args![this, b'-' as u32, b'_' as u32, 0u32]);
        assert!(got.bool());
        assert_eq!(text(&e, this), b"a_b-c\0");
        let got = e.call(0x00d6_1840, &args![this, b'-' as u32, b'+' as u32, 1u32]);
        assert!(got.bool());
        assert_eq!(text(&e, this), b"a_b+c\0");
        let got = e.call(0x00d6_1840, &args![this, b'z' as u32, b'+' as u32, 1u32]);
        assert!(!got.bool());
        // The terminator is outside the range, and an empty text is not looked at.
        let got = e.call(0x00d6_1840, &args![this, 0u32, b'!' as u32, 1u32]);
        assert!(!got.bool());
        let empty = buffer(&mut e, b"", 8);
        let got = e.call(0x00d6_1840, &args![empty, 0u32, b'!' as u32, 1u32]);
        assert!(!got.bool());
        assert_eq!(text(&e, empty), b"\0");
    }

    #[test]
    fn fn_00d61890_lower_cases_letters_only() {
        let (mut e, _) = engine();
        let this = buffer(&mut e, b"Az@[`{ \xc1Z", 16);
        e.call(0x00d6_1890, &args![this]);
        assert_eq!(text(&e, this), b"az@[`{ \xc1z\0");
        let empty = buffer(&mut e, b"", 4);
        e.call(0x00d6_1890, &args![empty]);
        assert_eq!(text(&e, empty), b"\0");
    }

    #[test]
    fn fn_00d618d0_upper_cases_letters_only() {
        let (mut e, _) = engine();
        let this = buffer(&mut e, b"aZ@[`{ \xe1z", 16);
        e.call(0x00d6_18d0, &args![this]);
        assert_eq!(text(&e, this), b"AZ@[`{ \xe1Z\0");
        let empty = buffer(&mut e, b"", 4);
        e.call(0x00d6_18d0, &args![empty]);
        assert_eq!(text(&e, empty), b"\0");
    }

    #[test]
    fn fn_00d61910_splits_at_the_separator() {
        let (mut e, log) = engine();
        let this = buffer(&mut e, b"ab,cd,,e", 16);
        let data = e.mem.u32(this.addr());
        // An empty pointer array: the first push grows it through the router's allocator.
        let pieces = e.mem.alloc(0xc);
        let got = e.call(0x00d6_1910, &args![this, b',' as u32, pieces]).i32();
        assert_eq!(got, 4);
        let array = e.mem.u32(pieces);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(array + 4 * i)).collect();
        assert_eq!(words, vec![data, data + 3, data + 6, data + 7]);
        assert_eq!(text(&e, this), b"ab\0cd\0\0e\0");
        let grown = log.borrow();
        assert_eq!(grown[0], (0x00c9_0940, vec![ALLOCATOR, pieces, 4]));
        // A separator at index 0 is not looked for (the search starts at 1).
        drop(grown);
        let this = buffer(&mut e, b",x", 8);
        let pieces = e.mem.alloc(0xc);
        let got = e.call(0x00d6_1910, &args![this, b',' as u32, pieces]).i32();
        assert_eq!(got, 1);
        assert_eq!(text(&e, this), b",x\0");
    }

    #[test]
    fn fn_00d619d0_copies_into_a_buffer_that_is_large_enough() {
        let (mut e, log) = engine();
        let other = buffer(&mut e, b"hello", 8);
        let this = buffer(&mut e, b"ab", 16);
        let data = e.mem.u32(this.addr());
        let got = e.call(0x00d6_19d0, &args![this, other]).ptr::<()>();
        assert_eq!(got, this);
        assert_eq!(e.mem.u32(this.addr()), data);
        assert_eq!(e.mem.u32(this.addr() + 8), 16);
        assert_eq!(text(&e, this), b"hello\0");
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn fn_00d619d0_reallocates_a_buffer_that_is_too_small() {
        let (mut e, log) = engine();
        let other = buffer(&mut e, b"hello", 8);
        let this = buffer(&mut e, b"ab", 2);
        let old = e.mem.u32(this.addr());
        e.call(0x00d6_19d0, &args![this, other]);
        // The old storage is freed with its capacity; the new one was asked for 6 bytes and
        // the allocator granted 14, which becomes the capacity word.
        assert_eq!(
            *log.borrow(),
            vec![(ALLOCATOR_FREE, vec![ALLOCATOR, old, 2])]
        );
        assert_ne!(e.mem.u32(this.addr()), old);
        assert_eq!(e.mem.u32(this.addr() + 8), 14);
        assert_eq!(text(&e, this), b"hello\0");
        // Storage that is not owned (bit 31 of the capacity) is not freed.
        let borrowed = buffer(&mut e, b"", 0x8000_0001);
        log.borrow_mut().clear();
        e.call(0x00d6_19d0, &args![borrowed, other]);
        assert!(log.borrow().is_empty());
        assert_eq!(text(&e, borrowed), b"hello\0");
    }
}
