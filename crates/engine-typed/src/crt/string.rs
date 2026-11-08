//! The string routines: lengths, compares (plain, counted and case-blind in the C locale), copies,
//! `strcat_s`, `strncpy_s`, `strcspn`, the case conversions, and three small helpers of the runtime's
//! type information (`type_info::operator==`, `FindCompleteObject`, `PMDtoOffset`).
//!
//! The scans read whole chunks (never past the page of a byte the exe reads) and look for the byte
//! that ends them, where the exe reads a byte or an aligned dword at a time: the bytes compared and
//! the answer are the same.

use super::{chunk, low_byte, low_half, move_up, pair_room, signed, small, FAMILY, SCAN};
use engine_core::{args, typed_call, Engine, Ptr};

/// `__locale_changed` (`0126fdf4`): 0 while the C locale is the global one.
const LOCALE_CHANGED: u32 = 0x0126_fdf4;
/// The invalid-parameter report (`00ec7c30`), `__tolower_l` (`00ec6695`), `__toupper_l`
/// (`00eca6de`), `__stricmp_l` (`00ec680f`) and `__strnicmp_l` (`00ec7dce`).
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
const TOLOWER_L: u32 = 0x00ec_6695;
const TOUPPER_L: u32 = 0x00ec_a6de;
const STRICMP_L: u32 = 0x00ec_680f;
const STRNICMP_L: u32 = 0x00ec_7dce;
/// `EINVAL`, `ERANGE` and `STRUNCATE`.
const EINVAL: u32 = 0x16;
const ERANGE: u32 = 0x22;
const STRUNCATE: u32 = 0x50;

/// `*_errno() = code`, then the invalid-parameter report with five zeros.
fn report(e: &mut Engine, code: u32) {
    let slot: Ptr = typed_call!(FAMILY, 0x00ec_85e3, super::math::errno(e));
    e.mem.set_u32(slot.addr(), code);
    e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
}

/// `A` to `Z` folded to lower case (`c - 0x41 <= 0x19` unsigned, plus 0x20).
#[inline(always)]
fn fold(c: u8) -> u8 {
    if c.wrapping_sub(0x41) <= 0x19 {
        c + 0x20
    } else {
        c
    }
}

/// The length of the zero-terminated bytes at `text`.
fn length(e: &Engine, text: u32) -> u32 {
    let mut buf = [0u8; SCAN];
    let mut at = text;
    loop {
        let run = chunk(e, at, u32::MAX, &mut buf);
        if let Some(i) = run.iter().position(|&c| c == 0) {
            return at.wrapping_add(small(i)).wrapping_sub(text);
        }
        at = at.wrapping_add(small(run.len()));
    }
}

// Typed from 00ec6130
/// `strlen`, cdecl: the bytes before the terminating zero.
pub fn strlen(e: &mut Engine, text: u32) -> u32 {
    length(e, text)
}

// Typed from 00ec6da0
/// `strcmp`, cdecl: 0 for equal texts, else -1 or 1 by the first differing byte (unsigned).
pub fn strcmp(e: &mut Engine, a: u32, b: u32) -> i32 {
    let (mut x, mut y) = ([0u8; SCAN], [0u8; SCAN]);
    let mut done = 0u32;
    loop {
        let (pa, pb) = (a.wrapping_add(done), b.wrapping_add(done));
        let n = pair_room(pa, pb, u32::MAX);
        e.mem.read(pa, &mut x[..n]);
        e.mem.read(pb, &mut y[..n]);
        for i in 0..n {
            if x[i] != y[i] {
                return if x[i] < y[i] { -1 } else { 1 };
            }
            if x[i] == 0 {
                return 0;
            }
        }
        done = done.wrapping_add(small(n));
    }
}

// Typed from 00ec6370
/// `strcpy`, cdecl: destination, source; the source's bytes and their zero copied (`strcat`'s copy,
/// `00ec63e5`), the destination answered. The exe copies bytes until the source is 4-aligned, then
/// dwords, storing exactly the string and its zero: one run gives the same unless the destination is
/// above the source inside the string, where the exe reads dwords it has just written; there its
/// steps are kept ([`word_copy`]).
pub fn strcpy(e: &mut Engine, dst: u32, src: u32) -> u32 {
    let n = length(e, src).wrapping_add(1);
    if !(dst > src && dst < src.wrapping_add(n)) {
        move_up(e, dst, src, n);
    } else {
        word_copy(e, dst, src);
    }
    dst
}

/// `00ec63e5`'s steps: bytes until the source is 4-aligned (a zero is stored and ends it), then a
/// dword at a time, stored whole unless the zero-byte test (`((x + 0x7efefeff) ^ !x) & 0x81010100`)
/// flags it; a flagged dword ends with its first zero (1, 2, 3 or 4 bytes stored) or, without one,
/// is stored whole.
fn word_copy(e: &Engine, dst: u32, src: u32) {
    let (mut d, mut s) = (dst, src);
    while s & 3 != 0 {
        let b = e.mem.u8(s);
        s = s.wrapping_add(1);
        e.mem.set_u8(d, b);
        if b == 0 {
            return;
        }
        d = d.wrapping_add(1);
    }
    loop {
        let x = e.mem.u32(s);
        s = s.wrapping_add(4);
        if (x.wrapping_add(0x7efe_feff) ^ !x) & 0x8101_0100 != 0 {
            if x & 0xff == 0 {
                e.mem.set_u8(d, 0);
                return;
            }
            if x & 0xff00 == 0 {
                e.mem.set_u16(d, low_half(x));
                return;
            }
            if x & 0xff_0000 == 0 {
                e.mem.set_u16(d, low_half(x));
                e.mem.set_u8(d.wrapping_add(2), 0);
                return;
            }
            if x & 0xff00_0000 == 0 {
                e.mem.set_u32(d, x);
                return;
            }
        }
        e.mem.set_u32(d, x);
        d = d.wrapping_add(4);
    }
}

// Typed from 00ec7690
/// `strchr`, cdecl: text, character; the first byte equal to the character's low byte, checked
/// before the terminator (so 0 finds the terminator), or null.
pub fn strchr(e: &mut Engine, text: u32, character: u32) -> u32 {
    let wanted = low_byte(character);
    let mut buf = [0u8; SCAN];
    let mut at = text;
    loop {
        let run = chunk(e, at, u32::MAX, &mut buf);
        for (i, &c) in run.iter().enumerate() {
            if c == wanted {
                return at.wrapping_add(small(i));
            }
            if c == 0 {
                return 0;
            }
        }
        at = at.wrapping_add(small(run.len()));
    }
}

// Typed from 00ec8a19
/// `strncmp`, cdecl: two strings and a count; the difference of the first pair of bytes (unsigned)
/// that differ or where the first string ends, within the count, else 0.
pub fn strncmp(e: &mut Engine, a: u32, b: u32, count: u32) -> i32 {
    let (mut x, mut y) = ([0u8; SCAN], [0u8; SCAN]);
    let mut done = 0u32;
    while done < count {
        let (pa, pb) = (a.wrapping_add(done), b.wrapping_add(done));
        let n = pair_room(pa, pb, count - done);
        e.mem.read(pa, &mut x[..n]);
        e.mem.read(pb, &mut y[..n]);
        for i in 0..n {
            if x[i] == 0 || x[i] != y[i] {
                return i32::from(x[i]) - i32::from(y[i]);
            }
        }
        done += small(n);
    }
    0
}

// Typed from 00ec67aa
/// `tolower`, cdecl: in the C locale `A` to `Z` get 0x20 added; otherwise `__tolower_l(c, NULL)`.
pub fn tolower(e: &mut Engine, c: u32) -> u32 {
    if e.mem.u32(LOCALE_CHANGED) != 0 {
        return e.call(TOLOWER_L, &args![c, 0u32]).u32();
    }
    if c.wrapping_sub(0x41) <= 0x19 {
        c.wrapping_add(0x20)
    } else {
        c
    }
}

// Typed from 00eca7f4
/// `toupper`, cdecl: in the C locale `a` to `z` lose 0x20; otherwise `__toupper_l(c, NULL)`.
pub fn toupper(e: &mut Engine, c: u32) -> u32 {
    if e.mem.u32(LOCALE_CHANGED) != 0 {
        return e.call(TOUPPER_L, &args![c, 0u32]).u32();
    }
    if c.wrapping_sub(0x61) <= 0x19 {
        c.wrapping_sub(0x20)
    } else {
        c
    }
}

// Typed from 00ec68e4
/// `_stricmp`, cdecl: with a changed locale `__stricmp_l(a, b, NULL)`; a null string is `EINVAL`
/// with the report and `0x7fffffff`; otherwise `___ascii_stricmp` (a jump: its answer).
pub fn stricmp(e: &mut Engine, a: u32, b: u32) -> i32 {
    if e.mem.u32(LOCALE_CHANGED) != 0 {
        return e.call(STRICMP_L, &args![a, b, 0u32]).i32();
    }
    if a == 0 || b == 0 {
        report(e, EINVAL);
        return 0x7fff_ffff;
    }
    typed_call!(
        FAMILY,
        0x00ec_67d6,
        ascii_stricmp(e, Ptr::new(a), Ptr::new(b))
    )
}

// Typed from 00ec67d6
/// `___ascii_stricmp`, cdecl: byte by byte with `A` to `Z` folded to lower case, to the first
/// difference or the first string's end; the difference of the two folded bytes.
pub fn ascii_stricmp(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    let (a, b) = (first.addr(), second.addr());
    let (mut x, mut y) = ([0u8; SCAN], [0u8; SCAN]);
    let mut done = 0u32;
    loop {
        let (pa, pb) = (a.wrapping_add(done), b.wrapping_add(done));
        let n = pair_room(pa, pb, u32::MAX);
        e.mem.read(pa, &mut x[..n]);
        e.mem.read(pb, &mut y[..n]);
        for i in 0..n {
            let (p, q) = (fold(x[i]), fold(y[i]));
            if p == 0 || p != q {
                return i32::from(p) - i32::from(q);
            }
        }
        done = done.wrapping_add(small(n));
    }
}

// Typed from 00ec7ec0
/// `_strnicmp`, cdecl: with a changed locale `__strnicmp_l(a, b, count, NULL)`; a null string or a
/// count above `0x7fffffff` is `EINVAL` with the report and `0x7fffffff`; else `___ascii_strnicmp`.
pub fn strnicmp(e: &mut Engine, a: u32, b: u32, count: u32) -> i32 {
    if e.mem.u32(LOCALE_CHANGED) != 0 {
        return e.call(STRNICMP_L, &args![a, b, count, 0u32]).i32();
    }
    if a == 0 || b == 0 || count > 0x7fff_ffff {
        report(e, EINVAL);
        return 0x7fff_ffff;
    }
    typed_call!(FAMILY, 0x00ed_5bc0, ascii_strnicmp(e, a, b, count))
}

// Typed from 00ed5bc0
/// `___ascii_strnicmp`, cdecl: two strings and a count (0 answers 0). Byte by byte, `A` to `Z`
/// folded: the first difference answers -1 or 1 (unsigned); a zero byte in either ends it, the bytes
/// then compared raw; `count` equal bytes answer 0.
pub fn ascii_strnicmp(e: &mut Engine, first: u32, second: u32, count: u32) -> i32 {
    let order = |p: u8, q: u8| match p.cmp(&q) {
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Greater => 1,
    };
    let (mut x, mut y) = ([0u8; SCAN], [0u8; SCAN]);
    let mut done = 0u32;
    while done < count {
        let (pa, pb) = (first.wrapping_add(done), second.wrapping_add(done));
        let n = pair_room(pa, pb, count - done);
        e.mem.read(pa, &mut x[..n]);
        e.mem.read(pb, &mut y[..n]);
        for i in 0..n {
            if x[i] == 0 || y[i] == 0 {
                return order(x[i], y[i]);
            }
            let (p, q) = (fold(x[i]), fold(y[i]));
            if p != q {
                return order(p, q);
            }
        }
        done += small(n);
    }
    0
}

// Typed from 00ec6bd2
/// `strcat_s`, cdecl: destination, size, source; 0 or an error. A null destination or size 0 is
/// `EINVAL` with the report; a null source, or no end within the size, empties the destination and
/// is `EINVAL`. The source and its zero are copied while room is left; running out of room empties
/// the destination and is `ERANGE` with the report.
pub fn strcat_s(e: &mut Engine, dst: u32, size: u32, src: u32) -> u32 {
    if dst != 0 && size != 0 {
        if src != 0 {
            let mut at = dst;
            let mut room = size;
            loop {
                if e.mem.u8(at) == 0 {
                    break;
                }
                at = at.wrapping_add(1);
                room -= 1;
                if room == 0 {
                    break;
                }
            }
            if room != 0 {
                let mut from = src;
                loop {
                    let b = e.mem.u8(from);
                    e.mem.set_u8(at, b);
                    at = at.wrapping_add(1);
                    from = from.wrapping_add(1);
                    if b == 0 {
                        break;
                    }
                    room -= 1;
                    if room == 0 {
                        break;
                    }
                }
                if room != 0 {
                    return 0;
                }
                e.mem.set_u8(dst, 0);
                report(e, ERANGE);
                return ERANGE;
            }
        }
        e.mem.set_u8(dst, 0);
    }
    report(e, EINVAL);
    EINVAL
}

// Typed from 00ec8d5f
/// `strncpy_s`, cdecl: destination, size, source, count; 0, `STRUNCATE` or an error. A zero count
/// with a null destination and size 0 answers 0; a null destination or size 0 is `EINVAL` with the
/// report; a zero count empties the destination; a null source empties it and is `EINVAL`. With
/// `_TRUNCATE` (-1) the source and its zero while room is left, else at most `count` bytes and a zero
/// after them when the count ran out. Out of room: with `_TRUNCATE` the last byte becomes the zero
/// (`STRUNCATE`), else the destination is emptied (`ERANGE` with the report).
pub fn strncpy_s(e: &mut Engine, dst: u32, size: u32, src: u32, count: u32) -> u32 {
    if count == 0 && dst == 0 && size == 0 {
        return 0;
    }
    if dst != 0 && size != 0 {
        if count == 0 {
            e.mem.set_u8(dst, 0);
            return 0;
        }
        if src != 0 {
            let truncate = count == 0xffff_ffff;
            let (mut at, mut from, mut room, mut left) = (dst, src, size, count);
            loop {
                let b = e.mem.u8(from);
                e.mem.set_u8(at, b);
                at = at.wrapping_add(1);
                from = from.wrapping_add(1);
                if b == 0 {
                    break;
                }
                room -= 1;
                if room == 0 {
                    break;
                }
                if !truncate {
                    left -= 1;
                    if left == 0 {
                        break;
                    }
                }
            }
            if !truncate && left == 0 {
                e.mem.set_u8(at, 0);
            }
            if room != 0 {
                return 0;
            }
            if truncate {
                e.mem.set_u8(dst.wrapping_add(size).wrapping_sub(1), 0);
                return STRUNCATE;
            }
            e.mem.set_u8(dst, 0);
            report(e, ERANGE);
            return ERANGE;
        }
        e.mem.set_u8(dst, 0);
    }
    report(e, EINVAL);
    EINVAL
}

// Typed from 00ecdbb0
/// `strcspn`, cdecl: text, control; the length of the text's first run of bytes not in the control
/// set (a 256-bit map; the terminator is never in it).
pub fn strcspn(e: &mut Engine, text: u32, control: u32) -> u32 {
    let mut map = [0u8; 32];
    let mut at = control;
    loop {
        let c = e.mem.u8(at);
        if c == 0 {
            break;
        }
        at = at.wrapping_add(1);
        map[usize::from(c >> 3)] |= 1 << (c & 7);
    }
    let mut buf = [0u8; SCAN];
    let mut at = text;
    loop {
        let run = chunk(e, at, u32::MAX, &mut buf);
        for (i, &c) in run.iter().enumerate() {
            if c == 0 || map[usize::from(c >> 3)] & (1 << (c & 7)) != 0 {
                return at.wrapping_add(small(i)).wrapping_sub(text);
            }
        }
        at = at.wrapping_add(small(run.len()));
    }
}

// Typed from 00ec3bad
/// `_byteswap_ulong`, cdecl: the bytes reversed (the exe's adds of shifted parts carry nothing).
pub fn byteswap_ulong(_e: &mut Engine, value: u32) -> u32 {
    value.swap_bytes()
}

// Typed from 00ec40cc
/// `type_info::operator==` (thiscall, the other `type_info`): 1 when `strcmp` of the decorated names
/// (`+9`, the other's first) is 0.
pub fn type_info_equal(e: &mut Engine, this: u32, other: u32) -> u32 {
    let order: i32 = typed_call!(
        FAMILY,
        0x00ec_6da0,
        strcmp(e, other.wrapping_add(9), this.wrapping_add(9))
    );
    u32::from(order == 0)
}

// Typed from 00ec40ec
/// `FindCompleteObject`, the object in `ECX`: the object less its complete object locator's offset
/// (`+4`), and with a constructor displacement (`+8`) less the word at the object minus it too.
pub fn find_complete_object(e: &mut Engine, object: u32) -> u32 {
    let vtable = e.mem.u32(object);
    let locator = e.mem.u32(vtable.wrapping_sub(4));
    let mut complete = object.wrapping_sub(e.mem.u32(locator.wrapping_add(4)));
    let displacement = e.mem.u32(locator.wrapping_add(8));
    if displacement != 0 {
        complete = complete.wrapping_sub(e.mem.u32(object.wrapping_sub(displacement)));
    }
    complete
}

// Typed from 00ec4179
/// `PMDtoOffset`, the `PMD` in `EAX`, the complete object on the stack: `mdisp`, plus, when `pdisp`
/// (`+4`) is not negative, `pdisp` and the word at `vdisp` (`+8`) in the virtual base table there.
pub fn pmd_to_offset(e: &mut Engine, pmd: u32, object: u32) -> u32 {
    let pdisp = e.mem.u32(pmd.wrapping_add(4));
    let mut offset = 0u32;
    if signed(pdisp) >= 0 {
        let table = e.mem.u32(pdisp.wrapping_add(object));
        let vdisp = e.mem.u32(pmd.wrapping_add(8));
        offset = pdisp.wrapping_add(e.mem.u32(table.wrapping_add(vdisp)));
    }
    e.mem.u32(pmd).wrapping_add(offset)
}
