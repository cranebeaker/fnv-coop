//! The C runtime's lower-casing of a string in place (MSVC 2008 LIBCMT, statically linked into
//! FalloutNV.exe 1.4.0.525), translated from the exe: `_strlwr_s_l_stat` (`00ec8e34`), the
//! body of `_strlwr_s` (`00ec8feb`) and `__strlwr_s_l` (`00ec8fb7`), and the two functions it
//! calls that nothing had translated: `strnlen` (`00ed5d55`) and `strcpy_s` (`00ec65a6`).
//! `__crtLCMapStringA` (`00ed0883`) is in `crt/stat.rs`; `_errno`, the invalid-parameter
//! handler, `malloc` and `_freea` are in `crt.rs`. Every call is made by address, as the exe
//! makes it, so a test can double each one.
//!
//! `strcpy_s` (`00ec65a6`) replaces the platform stand-in `crt.rs` had for it (taken out).

use super::*;

/// `_errno` (`00ec85e3`, cdecl), the invalid-parameter handler (`00ec7c30`, cdecl, five
/// words), `__crtLCMapStringA` (`00ed0883`, cdecl, nine words), `_malloc` (`00ecd1c7`, cdecl),
/// `__freea` (`00ec8e14`, cdecl) and `strcpy_s` (`00ec65a6`, cdecl: destination, size,
/// source). Names from Ghidra's function ID matches (engine map).
const ERRNO_ADDR: u32 = 0x00ec_85e3;
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
const CRT_LC_MAP_STRING_A: u32 = 0x00ed_0883;
const MALLOC_ADDR: u32 = 0x00ec_d1c7;
const FREEA_ADDR: u32 = 0x00ec_8e14;
const STRCPY_S: u32 = 0x00ec_65a6;
/// `LCMAP_LOWERCASE`, the mapping `_strlwr_s_l_stat` asks for.
const LCMAP_LOWERCASE: u32 = 0x100;
/// `EINVAL`, `ERANGE`, `EILSEQ` and `ENOMEM`.
const EINVAL: u32 = 0x16;
const ERANGE: u32 = 0x22;
const EILSEQ: u32 = 0x2a;
const ENOMEM: u32 = 0xc;

/// `errno` set through `_errno`.
fn store_errno(e: &mut Engine, code: u32) {
    let slot = e.call(ERRNO_ADDR, &args![]).u32();
    e.mem.set_u32(slot, code);
}

/// `errno` set through `_errno`, then the invalid-parameter handler with five zero words;
/// answers the code, as the exe's `MOV EAX,ESI` after the handler.
fn invalid(e: &mut Engine, code: u32) -> i32 {
    store_errno(e, code);
    e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
    code as i32
}

// Translated from 00ec8e34 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_strlwr_s_l_stat` (LIBCMT/strlwr.c, engine map), cdecl: (string, size, locale object,
/// the `_LocaleUpdate` whose first word is the locale information). A null string, or one
/// with no terminator within `size` bytes (`strnlen(string, size) >= size`; the first byte
/// is then cleared, even for a size of 0), sets `errno` `EINVAL`, calls the invalid-parameter
/// handler and answers `EINVAL`.
///
/// In the "C" locale (no `LC_CTYPE` handle, locale information +0x14) each byte `A`..`Z`
/// (a signed compare, so bytes from 0x80 stay) is made lower case in place, answering 0.
///
/// Otherwise `__crtLCMapStringA(locale, handle, LCMAP_LOWERCASE, string, -1, NULL, 0,
/// code page (+4), TRUE)` gives the size the mapped string needs. 0 sets `errno` `EILSEQ` and
/// answers `errno`. A size above `size` clears the first byte, sets `errno` `ERANGE`, calls the
/// handler and answers `ERANGE`. A buffer of that size and 8 more is taken: on the stack
/// (`_alloca`, marked 0xcccc) up to 0x400 bytes, else from `malloc` (marked 0xdddd); the
/// buffer starts after the 8-byte mark. A size that is not positive or for which 0xffffffe0
/// divided by it is below 1, or a failed `malloc`, sets `errno` `ENOMEM` and answers `errno`.
/// Then the string is mapped into the buffer (the handle and code page read again) and
/// copied back with `strcpy_s(string, size, buffer)`, whose result is the answer; a failed
/// mapping sets `errno` `EILSEQ` and answers it. `_freea(buffer)` follows either way.
///
/// The `_alloca` block is made with `with_stack`, as `crt.rs` does at its other `_alloca`
/// sites: the probe (`__alloca_probe_16` `00ec5ec0`, which rounds the block to 16 bytes and
/// goes on to `__alloca_probe` `00ec8660`) only touches the stack's pages and moves `ESP`, and
/// is left out.
// Translation incomplete: 00ec8e34 - the /GS stack cookie check (00ec408c) is not translated.
pub fn strlwr_s_l_stat(e: &mut Engine, string: Ptr, size: u32, locale: u32) -> i32 {
    let text = string.addr();
    if text == 0 {
        return invalid(e, EINVAL);
    }
    if strnlen(e, string, size) >= size {
        e.mem.set_u8(text, 0);
        return invalid(e, EINVAL);
    }
    let info = e.mem.u32(locale);
    let handle = e.mem.u32(info + 0x14);
    if handle == 0 {
        // 00ec8e8e: MOV ECX,EDI; CMP byte [EDI],0; JZ.
        let mut at = text;
        if e.mem.u8(at) != 0 {
            loop {
                // CMP AL,0x41; JL; CMP AL,0x5a; JG: signed byte compares.
                let c = e.mem.u8(at) as i8;
                if (0x41..=0x5a).contains(&c) {
                    e.mem.set_u8(at, (c + 0x20) as u8);
                }
                at = at.wrapping_add(1);
                if e.mem.u8(at) == 0 {
                    break;
                }
            }
        }
        return 0;
    }
    let code_page = e.mem.u32(info + 4);
    let count = e
        .call(
            CRT_LC_MAP_STRING_A,
            &args![
                locale,
                handle,
                LCMAP_LOWERCASE,
                text,
                0xffff_ffffu32,
                0u32,
                0u32,
                code_page,
                1u32
            ],
        )
        .u32();
    if count == 0 {
        store_errno(e, EILSEQ);
        // 00ec8edc: CALL _errno; MOV EAX,[EAX].
        let slot = e.call(ERRNO_ADDR, &args![]).u32();
        return e.mem.u32(slot) as i32;
    }
    // CMP [EBP+0xc],ECX; JNC: unsigned.
    if size < count {
        e.mem.set_u8(text, 0);
        return invalid(e, ERANGE);
    }
    // CMP ECX,0; JLE, then the unsigned DIV of 0xffffffe0 by the count and JC below 1.
    let fits = (count as i32) > 0 && 0xffff_ffe0u32 / count >= 1;
    let block_size = count.wrapping_add(8);
    if fits && block_size <= 0x400 {
        // CALL 00ec5ec0; MOV EAX,ESP: the block on the stack.
        e.with_stack(block_size, |e, block| {
            let block = block.addr();
            // CMP EAX,EBX; JZ 00ec8f3c: never taken (the stack is not at address 0); the buffer
            // would be 0, as below.
            let buffer = if block == 0 {
                0
            } else {
                e.mem.set_u32(block, 0xcccc);
                block + 8
            };
            lower_into(e, text, size, locale, count, buffer)
        })
    } else {
        let mut buffer = 0;
        if fits {
            let block = e.call(MALLOC_ADDR, &args![block_size]).u32();
            if block != 0 {
                e.mem.set_u32(block, 0xdddd);
                buffer = block + 8;
            }
        }
        lower_into(e, text, size, locale, count, buffer)
    }
}

/// `_strlwr_s_l_stat` from `00ec8f47` on: the buffer chosen (0 if none).
fn lower_into(e: &mut Engine, text: u32, size: u32, locale: u32, count: u32, buffer: u32) -> i32 {
    if buffer == 0 {
        store_errno(e, ENOMEM);
        // 00ec8edc: CALL _errno; MOV EAX,[EAX].
        let slot = e.call(ERRNO_ADDR, &args![]).u32();
        return e.mem.u32(slot) as i32;
    }
    // 00ec8f59: the locale information read again.
    let info = e.mem.u32(locale);
    let handle = e.mem.u32(info + 0x14);
    let code_page = e.mem.u32(info + 4);
    let mapped = e
        .call(
            CRT_LC_MAP_STRING_A,
            &args![
                locale,
                handle,
                LCMAP_LOWERCASE,
                text,
                0xffff_ffffu32,
                buffer,
                count,
                code_page,
                1u32
            ],
        )
        .u32();
    let result = if mapped != 0 {
        e.call(STRCPY_S, &args![text, size, buffer]).i32()
    } else {
        store_errno(e, EILSEQ);
        EILSEQ as i32
    };
    e.call(FREEA_ADDR, &args![buffer]);
    result
}

// Translated from 00ed5d55 (disassembled, FalloutNV.exe 1.4.0.525)
/// `strnlen` (LIBCMT/strnlen.c, engine map), cdecl: (string, most): the bytes before the
/// terminator, at most `most`. A `most` of 0 answers 0 without reading the string.
pub fn strnlen(e: &mut Engine, string: Ptr, most: u32) -> u32 {
    let mut length = 0u32;
    // CMP [EBP+0xc],EAX; JBE: unsigned.
    if most > length {
        let mut at = string.addr();
        while e.mem.u8(at) != 0 {
            length = length.wrapping_add(1);
            at = at.wrapping_add(1);
            if length >= most {
                break;
            }
        }
    }
    length
}

// Translated from 00ec65a6 (disassembled, FalloutNV.exe 1.4.0.525)
/// `strcpy_s` (LIBCMT/strcpy_s.c, engine map), cdecl: (destination, size, source). A null
/// destination or a size of 0 sets `errno` `EINVAL`, calls the invalid-parameter handler and
/// answers `EINVAL`; a null source clears the destination's first byte first. Otherwise the
/// source is copied byte by byte up to and including its terminator, at most `size` bytes;
/// answers 0. A source that does not fit (no terminator among the first `size` bytes copied)
/// clears the destination's first byte (the other bytes copied stay), sets `errno` `ERANGE`,
/// calls the handler and answers `ERANGE`.
///
/// Not in `funcs()`: `crt.rs`'s platform stand-in holds the address (see the module comment).
pub fn strcpy_s(e: &mut Engine, destination: Ptr, size: u32, source: Ptr) -> i32 {
    let to = destination.addr();
    // CMP ECX,EBX; JZ, then CMP EDI,EBX; JA: unsigned.
    if to == 0 || size == 0 {
        return invalid(e, EINVAL);
    }
    let mut from = source.addr();
    if from == 0 {
        e.mem.set_u8(to, 0);
        return invalid(e, EINVAL);
    }
    let mut at = to;
    let mut left = size;
    loop {
        let c = e.mem.u8(from);
        e.mem.set_u8(at, c);
        at = at.wrapping_add(1);
        from = from.wrapping_add(1);
        if c == 0 {
            break;
        }
        left = left.wrapping_sub(1);
        if left == 0 {
            break;
        }
    }
    if left == 0 {
        e.mem.set_u8(to, 0);
        return invalid(e, ERANGE);
    }
    0
}

/// `_getptd` (`00ed11de`, cdecl): the thread's data, whose `_token` (`+0x18`) `strtok` keeps.
const GETPTD: u32 = 0x00ed_11de;
const PTD_TOKEN: u32 = 0x18;

// Translated from 00ecc6e6 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecc6e6 - the /GS stack cookie check (00ec408c) is not translated.
/// `strtok` (LIBCMT/strtok.c, Ghidra's function ID), cdecl: (string, delimiters). The
/// delimiters, their terminating zero included, are set in a 32-byte bit map on the stack (bit
/// `c & 7` of byte `c >> 3`). A null string goes on from the thread's `_token` (`_getptd()`
/// +0x18). Leading delimiters are skipped (the zero, always in the map, ends the skip); the
/// token runs to the next delimiter, which is overwritten with a zero and stepped over, or to
/// the string's end. `_token` keeps where it stopped. Answers the token's start, or null when
/// it is empty.
pub fn strtok(e: &mut Engine, string: Ptr, delimiters: Ptr) -> Ptr {
    let ptd = e.call(GETPTD, &args![]).u32();
    let mut map = [0u8; 32];
    let mut at = delimiters.addr();
    loop {
        let c = e.mem.u8(at);
        map[(c >> 3) as usize] |= 1 << (c & 7);
        at = at.wrapping_add(1);
        if c == 0 {
            break;
        }
    }
    let delimiter = |c: u8| map[(c >> 3) as usize] & (1 << (c & 7)) != 0;
    let mut p = string.addr();
    if p == 0 {
        p = e.mem.u32(ptd.wrapping_add(PTD_TOKEN));
    }
    // 00ecc748: skip delimiters, stopping at the zero.
    loop {
        let c = e.mem.u8(p);
        if !delimiter(c) || c == 0 {
            break;
        }
        p = p.wrapping_add(1);
    }
    let start = p;
    // 00ecc77d: the token.
    while e.mem.u8(p) != 0 {
        if delimiter(e.mem.u8(p)) {
            e.mem.set_u8(p, 0);
            p = p.wrapping_add(1);
            break;
        }
        p = p.wrapping_add(1);
    }
    e.mem.set_u32(ptd.wrapping_add(PTD_TOKEN), p);
    // SUB EAX,EDX; NEG; SBB; AND EBX: the start unless it is where the scan stopped.
    if start != p {
        Ptr::new(start)
    } else {
        Ptr::new(0)
    }
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_8e34, strlwr_s_l_stat(Ptr, u32, u32) -> i32),
        entry!(0x00ed_5d55, strnlen(Ptr, u32) -> u32),
        entry!(0x00ec_65a6, strcpy_s(Ptr, u32, Ptr) -> i32),
        entry!(0x00ec_c6e6, strtok(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The mark word `_freea` found before each buffer it was given.
    type Marks = Rc<RefCell<Vec<u32>>>;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An engine with `_errno` doubled to answer one slot, the invalid-parameter handler
    /// doubled to do nothing, and the call log on. Answers the engine and the slot.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        e.register_double(ERRNO_ADDR, move |_, _| slot.into_ret());
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, slot)
    }

    /// `bytes` and a terminator in engine memory, with room after; answers the address.
    fn text(e: &mut Engine, bytes: &[u8]) -> u32 {
        let at = e.mem.alloc(bytes.len() as u32 + 0x10);
        e.mem.write(at, bytes);
        e.mem.set_u8(at + bytes.len() as u32, 0);
        at
    }

    /// A `_LocaleUpdate` object whose locale information has the `LC_CTYPE` handle and code
    /// page given; answers the object's address.
    fn locale(e: &mut Engine, handle: u32, code_page: u32) -> u32 {
        let info = e.mem.alloc(0x100);
        e.mem.set_u32(info + 0x14, handle);
        e.mem.set_u32(info + 4, code_page);
        let update = e.mem.alloc(0x10);
        e.mem.set_u32(update, info);
        update
    }

    #[test]
    fn strlwr_s_l_stat_lowers_in_the_c_locale_or_through_lcmapstring() {
        let (mut e, slot) = engine();
        // The "C" locale: A..Z only, bytes from 0x80 stay.
        let c_locale = locale(&mut e, 0, 0);
        let s = text(&mut e, b"AbZ@[\xc1Q");
        assert_eq!(e.call(0x00ec_8e34, &args![s, 0x10u32, c_locale]).i32(), 0);
        assert_eq!(e.mem.cstr(s), b"abz@[\xc1q".to_vec());
        let empty = text(&mut e, b"");
        assert_eq!(e.call(0x00ec_8e34, &args![empty, 1u32, c_locale]).i32(), 0);
        assert!(calls(&e, CRT_LC_MAP_STRING_A).is_empty());
        // A null string, or no terminator within the size (the first byte cleared, also for a
        // size of 0): EINVAL with the handler.
        assert_eq!(
            e.call(0x00ec_8e34, &args![0u32, 0x10u32, c_locale]).i32(),
            0x16
        );
        assert_eq!(e.mem.u32(slot), 0x16);
        assert_eq!(calls(&e, INVALID_PARAMETER), vec![vec![0, 0, 0, 0, 0]]);
        let full = text(&mut e, b"ABC");
        assert_eq!(
            e.call(0x00ec_8e34, &args![full, 3u32, c_locale]).i32(),
            0x16
        );
        assert_eq!(e.mem.bytes(full, 3), b"\0BC".to_vec());
        let zero = text(&mut e, b"X");
        assert_eq!(
            e.call(0x00ec_8e34, &args![zero, 0u32, c_locale]).i32(),
            0x16
        );
        assert_eq!(e.mem.u8(zero), 0);
        assert_eq!(calls(&e, INVALID_PARAMETER).len(), 3);

        // A locale with a handle: LCMapStringA asked for the size, then into the buffer.
        let update = locale(&mut e, 0x409, 1252);
        e.register(CRT_LC_MAP_STRING_A, |e, a| {
            let source = e.mem.cstr(a[3]);
            if a[5] != 0 {
                e.mem.write(a[5], &source.to_ascii_lowercase());
                e.mem.set_u8(a[5] + source.len() as u32, 0);
            }
            (source.len() as u32 + 1).into_ret()
        });
        e.register(STRCPY_S, |e, a| {
            let source = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &source);
            0u32.into_ret()
        });
        let marks: Marks = Rc::default();
        let seen = marks.clone();
        e.register_double(FREEA_ADDR, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[0] - 8));
            Ret::default()
        });
        let s = text(&mut e, b"HeLLo");
        assert_eq!(e.call(0x00ec_8e34, &args![s, 0x20u32, update]).i32(), 0);
        assert_eq!(e.mem.cstr(s), b"hello".to_vec());
        let mapped = calls(&e, CRT_LC_MAP_STRING_A);
        assert_eq!(
            mapped[0],
            vec![update, 0x409, 0x100, s, 0xffff_ffff, 0, 0, 1252, 1]
        );
        let buffer = mapped[1][5];
        assert_eq!(
            mapped[1],
            vec![update, 0x409, 0x100, s, 0xffff_ffff, buffer, 6, 1252, 1]
        );
        assert_eq!(calls(&e, STRCPY_S), vec![vec![s, 0x20, buffer]]);
        assert_eq!(calls(&e, FREEA_ADDR), vec![vec![buffer]]);
        // On the stack: marked 0xcccc, no malloc.
        assert_eq!(*marks.borrow(), vec![0xcccc]);
        assert!(calls(&e, MALLOC_ADDR).is_empty());
        // strcpy_s's answer is the answer.
        e.register(STRCPY_S, |_, _| 0x22u32.into_ret());
        assert_eq!(e.call(0x00ec_8e34, &args![s, 0x20u32, update]).i32(), 0x22);
        e.register(STRCPY_S, |e, a| {
            let source = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &source);
            0u32.into_ret()
        });

        // 0x3f9 bytes needed: 0x401 with the mark, from malloc (marked 0xdddd).
        e.register(MALLOC_ADDR, |e, a| e.mem.alloc(a[0]).into_ret());
        let long = text(&mut e, &[b'A'; 0x3f8]);
        assert_eq!(e.call(0x00ec_8e34, &args![long, 0x400u32, update]).i32(), 0);
        assert_eq!(e.mem.cstr(long), vec![b'a'; 0x3f8]);
        assert_eq!(calls(&e, MALLOC_ADDR), vec![vec![0x401]]);
        assert_eq!(marks.borrow().last(), Some(&0xdddd));
        // 0x3f8 needed is still on the stack (0x400 with the mark).
        let edge = text(&mut e, &[b'B'; 0x3f7]);
        assert_eq!(e.call(0x00ec_8e34, &args![edge, 0x3f8u32, update]).i32(), 0);
        assert_eq!(calls(&e, MALLOC_ADDR).len(), 1);
        assert_eq!(marks.borrow().last(), Some(&0xcccc));
        // malloc failing: ENOMEM, answered from errno, nothing mapped or freed.
        e.register(MALLOC_ADDR, |_, _| 0u32.into_ret());
        let frees = calls(&e, FREEA_ADDR).len();
        let maps = calls(&e, CRT_LC_MAP_STRING_A).len();
        assert_eq!(
            e.call(0x00ec_8e34, &args![long, 0x400u32, update]).i32(),
            0xc
        );
        assert_eq!(e.mem.u32(slot), 0xc);
        assert_eq!(calls(&e, FREEA_ADDR).len(), frees);
        assert_eq!(calls(&e, CRT_LC_MAP_STRING_A).len(), maps + 1);

        // More needed than the size: the first byte cleared, ERANGE with the handler.
        let handlers = calls(&e, INVALID_PARAMETER).len();
        let s = text(&mut e, b"ABC");
        assert_eq!(e.call(0x00ec_8e34, &args![s, 4u32, update]).i32(), 0);
        assert_eq!(e.mem.cstr(s), b"abc".to_vec());
        e.register(CRT_LC_MAP_STRING_A, |_, _| 5u32.into_ret());
        let s = text(&mut e, b"ABC");
        assert_eq!(e.call(0x00ec_8e34, &args![s, 4u32, update]).i32(), 0x22);
        assert_eq!(e.mem.bytes(s, 3), b"\0BC".to_vec());
        assert_eq!(e.mem.u32(slot), 0x22);
        assert_eq!(calls(&e, INVALID_PARAMETER).len(), handlers + 1);
        // A size that is not positive: ENOMEM.
        e.register(CRT_LC_MAP_STRING_A, |_, _| 0x8000_0000u32.into_ret());
        assert_eq!(
            e.call(0x00ec_8e34, &args![s, 0xffff_ffffu32, update]).i32(),
            0xc
        );
        // The size asked for failing: EILSEQ, answered from errno.
        e.register(CRT_LC_MAP_STRING_A, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ec_8e34, &args![s, 4u32, update]).i32(), 0x2a);
        assert_eq!(e.mem.u32(slot), 0x2a);
        // The mapping failing: EILSEQ, no copy, the buffer still freed.
        e.register(CRT_LC_MAP_STRING_A, |_, a| u32::from(a[5] == 0).into_ret());
        let copies = calls(&e, STRCPY_S).len();
        let frees = calls(&e, FREEA_ADDR).len();
        e.mem.set_u32(slot, 0);
        assert_eq!(e.call(0x00ec_8e34, &args![s, 4u32, update]).i32(), 0x2a);
        assert_eq!(e.mem.u32(slot), 0x2a);
        assert_eq!(calls(&e, STRCPY_S).len(), copies);
        assert_eq!(calls(&e, FREEA_ADDR).len(), frees + 1);
    }

    #[test]
    fn strnlen_counts_to_the_terminator_or_the_limit() {
        let (mut e, _) = engine();
        let s = text(&mut e, b"abc");
        assert_eq!(e.call(0x00ed_5d55, &args![s, 10u32]).u32(), 3);
        assert_eq!(e.call(0x00ed_5d55, &args![s, 3u32]).u32(), 3);
        assert_eq!(e.call(0x00ed_5d55, &args![s, 2u32]).u32(), 2);
        let empty = text(&mut e, b"");
        assert_eq!(e.call(0x00ed_5d55, &args![empty, 10u32]).u32(), 0);
        // A limit of 0 does not read the string (a null one here).
        assert_eq!(e.call(0x00ed_5d55, &args![0u32, 0u32]).u32(), 0);
    }

    #[test]
    fn strcpy_s_copies_or_reports_einval_and_erange() {
        let (mut e, slot) = engine();
        let source = text(&mut e, b"ab");
        let to = text(&mut e, b"xxxxxx");
        assert_eq!(strcpy_s(&mut e, Ptr::new(to), 3, Ptr::new(source)), 0);
        assert_eq!(e.mem.bytes(to, 5), b"ab\0xx".to_vec());
        assert!(calls(&e, INVALID_PARAMETER).is_empty());
        // A null destination or a size of 0: EINVAL with the handler, nothing written.
        assert_eq!(strcpy_s(&mut e, Ptr::new(0), 3, Ptr::new(source)), 0x16);
        assert_eq!(e.mem.u32(slot), 0x16);
        let to = text(&mut e, b"xxxxxx");
        assert_eq!(strcpy_s(&mut e, Ptr::new(to), 0, Ptr::new(source)), 0x16);
        assert_eq!(e.mem.u8(to), b'x');
        assert_eq!(calls(&e, INVALID_PARAMETER), vec![vec![0, 0, 0, 0, 0]; 2]);
        // A null source: the first byte cleared, EINVAL.
        assert_eq!(strcpy_s(&mut e, Ptr::new(to), 3, Ptr::new(0)), 0x16);
        assert_eq!(e.mem.bytes(to, 2), b"\0x".to_vec());
        // Too long: the bytes copied stay but the first, ERANGE with the handler.
        e.mem.set_u32(slot, 0);
        let long = text(&mut e, b"abcd");
        let to = text(&mut e, b"xxxxxx");
        assert_eq!(strcpy_s(&mut e, Ptr::new(to), 3, Ptr::new(long)), 0x22);
        assert_eq!(e.mem.bytes(to, 5), b"\0bcxx".to_vec());
        assert_eq!(e.mem.u32(slot), 0x22);
        assert_eq!(calls(&e, INVALID_PARAMETER).len(), 4);
        // Exactly the size with its terminator fits.
        let to = text(&mut e, b"xxxxxx");
        assert_eq!(strcpy_s(&mut e, Ptr::new(to), 4, Ptr::new(long)), 0x22);
        assert_eq!(strcpy_s(&mut e, Ptr::new(to), 5, Ptr::new(long)), 0);
        assert_eq!(e.mem.bytes(to, 6), b"abcd\0x".to_vec());
    }

    #[test]
    fn strtok_splits_on_the_delimiters_and_keeps_the_rest_in_the_thread_data() {
        let mut e = Engine::new();
        let ptd = e.mem.alloc(0x100);
        e.register_double(GETPTD, move |_, _| ptd.into_ret());
        let s = text(&mut e, b", ab,,c d ,");
        let delimiters = text(&mut e, b" ,");
        let next = |e: &mut Engine, from: u32| -> u32 {
            e.call(0x00ec_c6e6, &args![from, delimiters]).u32()
        };
        let first = next(&mut e, s);
        assert_eq!(first, s + 2);
        assert_eq!(e.mem.cstr(first), b"ab".to_vec());
        assert_eq!(e.mem.u32(ptd + 0x18), s + 5);
        let second = next(&mut e, 0);
        assert_eq!(e.mem.cstr(second), b"c".to_vec());
        let third = next(&mut e, 0);
        assert_eq!(e.mem.cstr(third), b"d".to_vec());
        // Only delimiters left: null, the token at the string's end.
        assert_eq!(next(&mut e, 0), 0);
        assert_eq!(e.mem.u32(ptd + 0x18), s + 11);
        assert_eq!(next(&mut e, 0), 0);
        // No delimiters: the whole string, the token at its zero.
        let plain = text(&mut e, b"plain");
        let none = text(&mut e, b"");
        assert_eq!(e.call(0x00ec_c6e6, &args![plain, none]).u32(), plain);
        assert_eq!(e.mem.u32(ptd + 0x18), plain + 5);
        // A delimiter byte from 0x80 up is in the map as well.
        let high = text(&mut e, b"x\xffy");
        let hd = text(&mut e, b"\xff");
        assert_eq!(e.call(0x00ec_c6e6, &args![high, hd]).u32(), high);
        assert_eq!(e.mem.cstr(high), b"x".to_vec());
        assert_eq!(e.mem.u32(ptd + 0x18), high + 2);
    }
}
