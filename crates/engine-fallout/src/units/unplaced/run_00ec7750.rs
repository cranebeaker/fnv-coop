//! `(unplaced)/run_00ec7750` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A run of the statically linked C/C++ runtime (LIBCMT): `strstr`, the
//! 64-bit shift helpers, `_CxxThrowException` and three `std::exception`
//! constructors.
//!
//! `00ec787c`, the `__finally` block of the `eh vector constructor iterator`
//! (`00ec782f`), is not a function of its own: it reads the iterator's locals and
//! arguments through the iterator's `EBP`, so its uniform form takes that `EBP` as
//! its one word ([`fn_00ec787c`]).

#[allow(unused_imports)]
use crate::prelude::*;

/// `strchr(string, character)` (LIBCMT, `cdecl`); `strstr` jumps into its
/// body for a one-character pattern.
const STRCHR: u32 = 0x00ec_7690;
/// The import slot of `RaiseException(code, flags, count, arguments)`
/// (KERNEL32, `stdcall`).
const RAISE_EXCEPTION: u32 = 0x00fd_f154;
/// The eight-word exception record template `_CxxThrowException` copies
/// (code, flags, record, address, parameter count, magic number, object,
/// throw information).
const THROW_RECORD_TEMPLATE: u32 = 0x010f_1f80;
/// The magic number `_CxxThrowException` stores when the throw information
/// has bit 3 set (a pure managed exception, in the MSVC runtime's terms).
const PURE_MAGIC_NUMBER: u32 = 0x0199_4000;
/// The `std::exception` vtable (its RTTI complete object locator, at
/// `0x0113e8d0`, names `.?AVexception@std@@`).
const EXCEPTION_VTABLE: u32 = 0x010f_1fa4;

layout! {
    /// `std::exception` (Xbox PDB), 0xc bytes; the PC code uses the same
    /// offsets.
    pub struct StdException: 0xc {
        /// `_Mywhat` (Xbox PDB): the message, `const char*`.
        0x04 _Mywhat: Ptr,
        /// `_Mydofree` (Xbox PDB): whether the message is an owned copy; the
        /// PC code reads and writes it as a word.
        0x08 _Mydofree: u32,
    }
}

// Translated from 00ec7750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `strstr(string, pattern)` (LIBCMT, `cdecl`): the first place `pattern`
/// occurs in `string`, or null. An empty pattern answers `string`; a
/// one-character pattern is `strchr`'s answer (the code jumps into
/// `strchr`'s body with the character). Longer patterns are compared two
/// characters at a time after the first two match; on a mismatch the scan
/// starts again at the second character of the candidate.
pub fn strstr(e: &mut Engine, string: Ptr, pattern: Ptr) -> Ptr {
    let first = e.mem.u8(pattern.addr());
    if first == 0 {
        return string;
    }
    let second = e.mem.u8(pattern.addr().wrapping_add(1));
    if second == 0 {
        return e.call(STRCHR, &args![string, first as u32]).ptr();
    }
    // `EDI`: where the next scan for the first character starts.
    let mut start = string.addr();
    'restart: loop {
        let mut cursor = start;
        let mut character = e.mem.u8(cursor);
        cursor = cursor.wrapping_add(1);
        loop {
            if character == first {
                character = e.mem.u8(cursor);
                cursor = cursor.wrapping_add(1);
                if character == second {
                    break;
                }
                // The character after a first-character match is checked
                // again as a first character.
                continue;
            }
            if character == 0 {
                return Ptr::new(0);
            }
            character = e.mem.u8(cursor);
            cursor = cursor.wrapping_add(1);
        }
        // `cursor` is just past the second character; the match starts one
        // before `start`.
        start = cursor.wrapping_sub(1);
        let mut pattern_at = pattern.addr();
        loop {
            let wanted = e.mem.u8(pattern_at.wrapping_add(2));
            if wanted == 0 {
                return Ptr::new(start.wrapping_sub(1));
            }
            let found = e.mem.u8(cursor);
            cursor = cursor.wrapping_add(2);
            if found != wanted {
                continue 'restart;
            }
            let wanted = e.mem.u8(pattern_at.wrapping_add(3));
            if wanted == 0 {
                return Ptr::new(start.wrapping_sub(1));
            }
            let found = e.mem.u8(cursor.wrapping_sub(1));
            pattern_at = pattern_at.wrapping_add(2);
            if found != wanted {
                continue 'restart;
            }
        }
    }
}

// Translated from 00ec77e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__allshr` (MSVC runtime helper): the signed 64-bit `value` (`EDX:EAX`)
/// shifted right by `count` (`CL`), the sign kept; a count of 64 or more
/// leaves only the sign. The uniform form takes the value as two words (low
/// first) and then the count; the result is in `EDX:EAX`.
pub fn allshr(_e: &mut Engine, value: u64, count: u8) -> u64 {
    let value = value as i64;
    if count >= 0x40 {
        return (value >> 63) as u64;
    }
    (value >> count) as u64
}

// Translated from 00ec7810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__allshl` (MSVC runtime helper): the 64-bit `value` (`EDX:EAX`) shifted
/// left by `count` (`CL`); a count of 64 or more gives 0. The uniform form
/// takes the value as two words (low first) and then the count; the result
/// is in `EDX:EAX`.
pub fn allshl(_e: &mut Engine, value: u64, count: u8) -> u64 {
    if count >= 0x40 {
        return 0;
    }
    value << count
}

// Translated from 00ec7894 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_CxxThrowException(object, throw_info)` (MSVC runtime, `stdcall`): copies
/// the exception record template (`0x010f1f80`, eight words) to the stack,
/// puts the thrown object and its throw information in the last two words,
/// stores the magic number `0x01994000` in the sixth when the throw
/// information is not null and has bit 3 set, and raises the exception
/// (`RaiseException(code, flags, parameter count, &magic number)`).
pub fn cxx_throw_exception(e: &mut Engine, object: u32, throw_info: Ptr) {
    e.with_stack(0x20, |e, record| {
        let record = record.addr();
        for i in 0..8 {
            let word = e.mem.u32(THROW_RECORD_TEMPLATE + 4 * i);
            e.mem.set_u32(record + 4 * i, word);
        }
        e.mem.set_u32(record + 0x18, object);
        e.mem.set_u32(record + 0x1c, throw_info.addr());
        if !throw_info.is_null() && e.mem.u8(throw_info.addr()) & 8 != 0 {
            e.mem.set_u32(record + 0x14, PURE_MAGIC_NUMBER);
        }
        let code = e.mem.u32(record);
        let flags = e.mem.u32(record + 4);
        let count = e.mem.u32(record + 0x10);
        e.call(RAISE_EXCEPTION, &args![code, flags, count, record + 0x14]);
    });
}

// Translated from 00ec78e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `std::exception` constructor without a message (unnamed in the
/// engine map; the body stores the `std::exception` vtable): no message,
/// nothing to free. Returns `this`.
pub fn fn_00ec78e0(e: &mut Engine, this: Ptr<StdException>) -> Ptr<StdException> {
    e.set(this, StdException::_Mywhat, Ptr::new(0));
    e.set(this, StdException::_Mydofree, 0);
    e.mem.set_u32(this.addr(), EXCEPTION_VTABLE);
    this
}

/// `__ArrayUnwind` (`00ec5f70`, four words: array, element size, count, destructor).
const ARRAY_UNWIND: u32 = 0x00ec_5f70;

// Translated from 00ec787c (decompiled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of the `eh vector constructor iterator` (`00ec782f`): it runs on the
/// iterator's frame, so the uniform form takes that frame's `EBP` as its one word. When the
/// completion flag (`EBP-0x20`) is 0 (the construction did not finish) the constructed elements
/// are destroyed: `__ArrayUnwind` with the array (`EBP+8`), the element size (`EBP+0xc`), the
/// count constructed (`EBP-0x1c`) and the destructor (`EBP+0x18`).
pub fn fn_00ec787c(e: &mut Engine, frame: u32) {
    if e.mem.u32(frame.wrapping_sub(0x20)) != 0 {
        return;
    }
    let destructor = e.mem.u32(frame.wrapping_add(0x18));
    let count = e.mem.u32(frame.wrapping_sub(0x1c));
    let size = e.mem.u32(frame.wrapping_add(0xc));
    let array = e.mem.u32(frame.wrapping_add(8));
    e.call(ARRAY_UNWIND, &args![array, size, count, destructor]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec7750, strstr(Ptr, Ptr) -> Ptr),
        entry!(0x00ec77e0, allshr(u64, u8) -> u64),
        entry!(0x00ec7810, allshl(u64, u8) -> u64),
        entry!(0x00ec7894, cxx_throw_exception(u32, Ptr)),
        entry!(0x00ec_787c, fn_00ec787c(u32)),
        entry!(
            0x00ec78e0,
            fn_00ec78e0(Ptr<StdException>) -> Ptr<StdException>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn string(e: &mut Engine, text: &[u8]) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(at, text);
        at
    }

    fn find(e: &mut Engine, haystack: u32, needle: &[u8]) -> u32 {
        let needle = string(e, needle);
        e.call(0x00ec_7750, &args![haystack, needle]).u32()
    }

    #[test]
    fn strstr_finds_patterns_and_restarts_after_partial_matches() {
        let mut e = Engine::new();
        e.register(STRCHR, |_, _| panic!("strchr is only for one character"));
        let haystack = string(&mut e, b"aababcabcdabcde");
        assert_eq!(find(&mut e, haystack, b""), haystack);
        assert_eq!(find(&mut e, haystack, b"ab"), haystack + 1);
        assert_eq!(find(&mut e, haystack, b"abc"), haystack + 3);
        assert_eq!(find(&mut e, haystack, b"abcd"), haystack + 6);
        assert_eq!(find(&mut e, haystack, b"abcde"), haystack + 10);
        assert_eq!(find(&mut e, haystack, b"abcdef"), 0);
        assert_eq!(find(&mut e, haystack, b"xy"), 0);
        // A first character directly after a first-character match.
        let doubled = string(&mut e, b"xxxy");
        assert_eq!(find(&mut e, doubled, b"xy"), doubled + 2);
        assert_eq!(find(&mut e, doubled, b"xxy"), doubled + 1);
    }

    #[test]
    fn strstr_hands_one_character_patterns_to_strchr() {
        let mut e = Engine::new();
        e.register(STRCHR, |_, a| Ret {
            eax: a[0] + a[1],
            ..Ret::default()
        });
        let haystack = string(&mut e, b"hello");
        assert_eq!(find(&mut e, haystack, b"l"), haystack + u32::from(b'l'));
    }

    #[test]
    fn shifts_handle_small_large_and_out_of_range_counts() {
        let mut e = Engine::new();
        let value = 0x8000_0001_0000_0010u64;
        let mut shift = |address: u32, count: u8| e.call(address, &args![value, count]).u64();
        assert_eq!(shift(0x00ec_77e0, 4), 0xf800_0000_1000_0001);
        assert_eq!(shift(0x00ec_77e0, 36), 0xffff_ffff_f800_0000);
        assert_eq!(shift(0x00ec_77e0, 64), u64::MAX);
        assert_eq!(shift(0x00ec_7810, 4), 0x0000_0010_0000_0100);
        assert_eq!(shift(0x00ec_7810, 36), 0x0000_0100_0000_0000);
        assert_eq!(shift(0x00ec_7810, 0x80), 0);
        assert_eq!(e.call(0x00ec_77e0, &args![0x10u64, 0x40u8]).u64(), 0);
    }

    #[test]
    fn throw_raises_the_template_record_with_the_magic_number() {
        for (flags, expected_magic) in [(8u8, PURE_MAGIC_NUMBER), (0, 0x1993_0520)] {
            let mut e = Engine::new();
            e.map(THROW_RECORD_TEMPLATE, 0x20);
            for (i, word) in [0xe06d_7363u32, 1, 0, 0, 3, 0x1993_0520, 0, 0]
                .iter()
                .enumerate()
            {
                e.mem.set_u32(THROW_RECORD_TEMPLATE + 4 * i as u32, *word);
            }
            let info = e.mem.alloc(4);
            e.mem.set_u8(info, flags);
            e.register_double(RAISE_EXCEPTION, move |e, a| {
                assert_eq!(&a[..3], &[0xe06d_7363, 1, 3]);
                assert_eq!(e.mem.u32(a[3]), expected_magic);
                assert_eq!(e.mem.u32(a[3] + 4), 0x1234);
                assert_eq!(e.mem.u32(a[3] + 8), info);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            e.call(0x00ec_7894, &args![0x1234u32, info]);
            let log = e.call_log.take().unwrap();
            assert_eq!(log.iter().filter(|c| c.0 == RAISE_EXCEPTION).count(), 1);
        }
    }

    #[test]
    fn throw_without_information_keeps_the_template_magic() {
        let mut e = Engine::new();
        e.map(THROW_RECORD_TEMPLATE, 0x20);
        e.mem.set_u32(THROW_RECORD_TEMPLATE + 0x14, 0x1993_0520);
        e.register(RAISE_EXCEPTION, |e, a| {
            assert_eq!(e.mem.u32(a[3]), 0x1993_0520);
            assert_eq!(e.mem.u32(a[3] + 8), 0);
            Ret::default()
        });
        e.call(0x00ec_7894, &args![7u32, 0u32]);
    }

    #[test]
    fn default_constructor_clears_the_message() {
        let mut e = Engine::new();
        let this = e.new_object::<StdException>();
        e.mem.set_u32(this.addr() + 4, 5);
        e.mem.set_u32(this.addr() + 8, 6);
        let r = e.call(0x00ec_78e0, &args![this]).u32();
        assert_eq!(r, this.addr());
        assert_eq!(e.mem.u32(this.addr()), EXCEPTION_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 4), 0);
        assert_eq!(e.mem.u32(this.addr() + 8), 0);
    }
}

#[cfg(test)]
mod n089_tests {
    use super::*;

    #[test]
    fn n089_00ec787c_unwinds_an_unfinished_array() {
        let mut e = Engine::new();
        e.register(ARRAY_UNWIND, |_, _| Ret::default());
        let frame = e.mem.alloc(0x40) + 0x20;
        e.mem.set_u32(frame + 8, 0xa000);
        e.mem.set_u32(frame + 0xc, 0x10);
        e.mem.set_u32(frame - 0x1c, 3);
        e.mem.set_u32(frame + 0x18, 0x00d0_0000);
        e.call_log = Some(vec![]);
        e.call(0x00ec_787c, &args![frame]);
        let unwinds = |e: &Engine| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == ARRAY_UNWIND)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(unwinds(&e), vec![vec![0xa000, 0x10, 3, 0x00d0_0000]]);
        // Finished: nothing to unwind.
        e.mem.set_u32(frame - 0x20, 1);
        e.call_log = Some(vec![]);
        e.call(0x00ec_787c, &args![frame]);
        assert!(unwinds(&e).is_empty());
    }
}
