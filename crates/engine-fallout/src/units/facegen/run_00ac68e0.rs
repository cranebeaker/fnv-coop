//! `FaceGen/run_00ac68e0` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::facegen::binaryfile::{
    string_chars, with_string_arg, StringArg, INVALID_PARAMETER, STRING_ASSIGN, STRING_LENGTH,
    STRING_NPOS, STRING_TIDY,
};

/// The error word `futSetLastError` sets (`011f6c88`), `futPrintLastError` (Xbox PDB,
/// `00ac6850`) and `_exit` (`00eccec5`, cdecl).
const LAST_ERROR: u32 = 0x011f_6c88;
const PRINT_LAST_ERROR: u32 = 0x00ac_6850;
const EXIT: u32 = 0x00ec_cec5;
/// `fr2Assert` (`00aafb90`, cdecl: source file name, line) and `algs.cpp`'s name.
const FR2_ASSERT: u32 = 0x00aa_fb90;
const ALGS_SOURCE: u32 = 0x010a_2f50;
/// `strlen` (`00ec6130`), `strchr` (`00ec7690`), `strpbrk` (`00ecbf20`), `fopen`
/// (`00ec9a47`), `fclose` (`00ec9907`), `sprintf` (`00ec623a`) and `_ftol2` (`00ec62c0`, the
/// value in `ST0`).
const STRLEN: u32 = 0x00ec_6130;
const STRCHR: u32 = 0x00ec_7690;
const STRPBRK: u32 = 0x00ec_bf20;
const FOPEN: u32 = 0x00ec_9a47;
const FCLOSE: u32 = 0x00ec_9907;
const SPRINTF: u32 = 0x00ec_623a;
const FTOL2: u32 = 0x00ec_62c0;
/// The texts the path functions use: `"\\"`, `"/"`, `":"`, `"."`, `"\\/"`, `"r"`, `"%d"`, `""`.
const TEXT_BACKSLASH: u32 = 0x0101_3444;
const TEXT_SLASH: u32 = 0x0106_3cbc;
const TEXT_COLON: u32 = 0x0106_3a14;
const TEXT_DOT: u32 = 0x0104_4c28;
const TEXT_SEPARATORS: u32 = 0x010a_2f88;
const TEXT_READ_MODE: u32 = 0x0108_2cb8;
const TEXT_DECIMAL: u32 = 0x0102_0764;
const TEXT_EMPTY: u32 = 0x0101_1584;
/// The `std::string` members used: `rfind(text, position, count)` (`00a0c300`), `substr`
/// (`007ab000`: result, position, count), the construction from text (`0044a4c0`), the append
/// of text (`00a0c430`: text, count), and the concatenations (`00ad4210`: result, string,
/// text; `00ad42e0`: result, string, string; cdecl).
const STRING_RFIND: u32 = 0x00a0_c300;
const STRING_SUBSTR: u32 = 0x007a_b000;
const STRING_FROM_TEXT: u32 = 0x0044_a4c0;
const STRING_APPEND_TEXT: u32 = 0x00a0_c430;
const STRING_PLUS_TEXT: u32 = 0x00ad_4210;
const STRING_PLUS_STRING: u32 = 0x00ad_42e0;
/// The character container the hex functions fill: its clear (`00acae90`), the append of a
/// character (`00acad60`: the character's address) and the address of an element of the byte
/// vector (`00acad30`: the index); `thiscall`.
const CHARS_CLEAR: u32 = 0x00ac_ae90;
const CHARS_APPEND: u32 = 0x00ac_ad60;
const BYTES_AT: u32 = 0x00ac_ad30;
/// `-2147483648.0` and `2147483647.0` (`double`s).
const INT_MINIMUM: u32 = 0x0109_4ba8;
const INT_MAXIMUM: u32 = 0x010a_2f90;

/// The vector's first (+0xC) and last (+0x10) pointers.
const VECTOR_FIRST: u32 = 0xc;
const VECTOR_LAST: u32 = 0x10;

fn check(e: &mut Engine, ok: bool) {
    if !ok {
        e.call(INVALID_PARAMETER, &[]);
    }
}

/// The `std::string` destruction (`0044a730(1, 0)`).
fn destroy_string(e: &mut Engine, string: u32) {
    e.call(STRING_TIDY, &args![string, 1u32, 0u32]);
}

/// `assign(source, 0, npos)` of the string at `to`.
fn assign_string(e: &mut Engine, to: u32, source: u32) {
    let npos: u32 = e.global(STRING_NPOS);
    e.call(STRING_ASSIGN, &args![to, source, 0u32, npos]);
}

// Translated from 00ac68e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, no arguments): with a FaceGen error set (`011f6c88`),
/// prints it (`futPrintLastError`) and ends the process (`_exit(-1)`, which does not return
/// in the game).
pub fn fn_00ac68e0(e: &mut Engine) {
    if e.mem.u32(LAST_ERROR) != 0 {
        e.call(PRINT_LAST_ERROR, &[]);
        e.call(EXIT, &args![u32::MAX]);
    }
}

// Translated from 00ac6900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): the index of the highest set bit of `value`
/// (asserting it is not 0, `fr2Assert(algs.cpp, 0x27)`; then the answer is 0).
pub fn fn_00ac6900(e: &mut Engine, value: u32) -> u32 {
    if value == 0 {
        e.call(FR2_ASSERT, &args![ALGS_SOURCE, 0x27u32]);
    }
    let mut value = value;
    let mut bits = 0u32;
    loop {
        value >>= 1;
        bits = bits.wrapping_add(1);
        if value == 0 {
            break;
        }
    }
    bits.wrapping_sub(1)
}

// Translated from 00ac6950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, one word): the number of bits needed for `value` rounded
/// up to a power of two: `fn_00ac6900`, plus one unless `value` is that power (the shift by
/// the low five bits of the index, as `SHL` does).
pub fn fn_00ac6950(e: &mut Engine, value: u32) -> u32 {
    let bit = fn_00ac6900(e, value);
    if value == 1u32.wrapping_shl(bit & 0x1f) {
        fn_00ac6900(e, value)
    } else {
        fn_00ac6900(e, value).wrapping_add(1)
    }
}

// Translated from 00ac6990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `n`, `k`, two word pointers): the pair `(i, j)` with
/// `i < j < n` that is `k`-th in row order, written through the pointers (asserting `n` is not
/// 0, line 0x53). Running out of pairs asserts (line 0x60); the jump around that assert is
/// guarded by a register the code has just cleared (`XOR ECX,ECX; JZ`), so it is never taken.
pub fn fn_00ac6990(e: &mut Engine, n: u32, k: u32, first: u32, second: u32) {
    if n == 0 {
        e.call(FR2_ASSERT, &args![ALGS_SOURCE, 0x53u32]);
    }
    let mut index = 0u32;
    e.mem.set_u32(first, 0);
    while e.mem.u32(first) < n {
        let next = e.mem.u32(first).wrapping_add(1);
        e.mem.set_u32(second, next);
        while e.mem.u32(second) < n {
            if k == index {
                return;
            }
            index = index.wrapping_add(1);
            let next = e.mem.u32(second).wrapping_add(1);
            e.mem.set_u32(second, next);
        }
        let next = e.mem.u32(first).wrapping_add(1);
        e.mem.set_u32(first, next);
    }
    e.call(FR2_ASSERT, &args![ALGS_SOURCE, 0x60u32]);
}

/// `rfind(text)` of the string at `string` from `npos`, with the text's `strlen`; `npos`
/// (the global) when absent.
fn rfind(e: &mut Engine, string: u32, text: u32) -> u32 {
    let npos: u32 = e.global(STRING_NPOS);
    let count = e.call(STRLEN, &args![text]).u32();
    e.call(STRING_RFIND, &args![string, text, npos, count])
        .u32()
}

/// A found position plus one, or 0 for `npos`.
fn after(e: &Engine, position: u32) -> u32 {
    if position == e.global::<u32>(STRING_NPOS) {
        0
    } else {
        position.wrapping_add(1)
    }
}

/// `substr(position, count)` of the string at `string` assigned to `to` (through a temporary
/// destroyed after).
fn assign_part(e: &mut Engine, string: u32, to: u32, position: u32, count: u32) {
    e.with_stack(0x1c, |e, temporary| {
        let part = e
            .call(STRING_SUBSTR, &args![string, temporary, position, count])
            .u32();
        assign_string(e, to, part);
        destroy_string(e, temporary.addr());
    });
}

// Translated from 00ac6a30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac6a30 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (cdecl: a path string by value, then the directory, name and
/// extension strings to fill): splits the path. The name starts after the last `\`, `/` or
/// `:` (`rfind`); a `:` after a separator, a path ending there or a last `.` right there
/// refuse (false). The extension starts after the last `.` (none: the path's end; a `.`
/// before the name start also counts as none), asserting it lies past the name start
/// (`algs.cpp`, 0xaa). The three parts are `substr`s assigned to the out strings; true. The
/// path argument is destroyed on every path.
pub fn fn_00ac6a30(
    e: &mut Engine,
    path: StringArg,
    directory: u32,
    name: u32,
    extension: u32,
) -> bool {
    with_string_arg(e, path, |e, path| {
        let length = e.mem.u32(path + STRING_LENGTH);
        let backslash = rfind(e, path, TEXT_BACKSLASH);
        let slash = rfind(e, path, TEXT_SLASH);
        let colon = rfind(e, path, TEXT_COLON);
        let backslash = after(e, backslash);
        let slash = after(e, slash);
        let colon = after(e, colon);
        let mut start = if backslash < slash { slash } else { backslash };
        if start > 0 && colon > start {
            destroy_string(e, path);
            return false;
        }
        start = if start < colon { colon } else { start };
        if start == length {
            destroy_string(e, path);
            return false;
        }
        let mut dot = rfind(e, path, TEXT_DOT);
        let extension_start = if dot == e.global::<u32>(STRING_NPOS) {
            dot = length;
            length
        } else {
            dot.wrapping_add(1)
        };
        if dot == start {
            destroy_string(e, path);
            return false;
        }
        if dot < start {
            dot = length;
        }
        if start >= dot {
            e.call(FR2_ASSERT, &args![ALGS_SOURCE, 0xaau32]);
        }
        assign_part(e, path, directory, 0, start);
        assign_part(e, path, name, start, dot.wrapping_sub(start));
        assign_part(
            e,
            path,
            extension,
            extension_start,
            length.wrapping_sub(extension_start),
        );
        destroy_string(e, path);
        true
    })
}

// Translated from 00ac6e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, the address of a string): whether the file it names can
/// be opened for reading (`fopen` `"r"`, closed again).
pub fn fn_00ac6e10(e: &mut Engine, name: u32) -> bool {
    let path = string_chars(e, name);
    let file = e.call(FOPEN, &args![path, TEXT_READ_MODE]).u32();
    if file == 0 {
        return false;
    }
    e.call(FCLOSE, &args![file]);
    true
}

/// Whether the byte at `at` is `\` or `/` (`MOVSX`, compared).
fn is_separator(e: &Engine, at: u32) -> bool {
    let c = i32::from(e.mem.i8(at));
    c == 0x5c || c == 0x2f
}

// Translated from 00ac6e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, a C string): the part of a path after its root: null for
/// none or an empty path; after a drive's `:` not followed by a separator, the rest; otherwise
/// after the first separator (`strpbrk "\\/"`), and for a path starting with two separators
/// (`\\server\share\`), after the two following ones. An empty rest is null.
pub fn fn_00ac6e70(e: &mut Engine, path: u32) -> u32 {
    let mut rest = 0u32;
    if path == 0 {
        return rest;
    }
    if e.mem.i8(path) == 0 {
        return rest;
    }
    let mut from = path;
    let colon = e.call(STRCHR, &args![path, 0x3au32]).u32();
    if colon != 0 {
        from = colon.wrapping_add(1);
        if !is_separator(e, from) {
            return from;
        }
    }
    let separator = e.call(STRPBRK, &args![from, TEXT_SEPARATORS]).u32();
    if separator != 0 {
        rest = separator.wrapping_add(1);
        if rest == path.wrapping_add(1) {
            let next = e.call(STRPBRK, &args![rest, TEXT_SEPARATORS]).u32();
            if next == rest {
                rest = rest.wrapping_add(1);
                rest = e.call(STRPBRK, &args![rest, TEXT_SEPARATORS]).u32();
                if rest != 0 {
                    rest = rest.wrapping_add(1);
                    rest = e.call(STRPBRK, &args![rest, TEXT_SEPARATORS]).u32();
                }
                if rest != 0 {
                    rest = rest.wrapping_add(1);
                }
            }
        }
    }
    if rest != 0 && e.mem.i8(rest) == 0 {
        rest = 0;
    }
    rest
}

// Translated from 00ac6fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac6fa0 - the compiler's exception frame (FS:[0] chain) and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map (cdecl: the result string, then the directory, name and
/// extension strings): a file name that does not exist yet: `name + "." + extension`, then
/// while `directory + candidate` exists (`fn_00ac6e10`), `name + n + "." + extension` for
/// n = 1, 2, ... (`sprintf "%d"`). The result is made empty and assigned the candidate;
/// returns it.
pub fn fn_00ac6fa0(e: &mut Engine, result: u32, directory: u32, name: u32, extension: u32) -> u32 {
    e.with_stack(0x100, |e, locals| {
        let base = locals.addr();
        let candidate = base;
        let number = base + 0x20;
        let temporary = base + 0x30;
        let with_dot = e
            .call(STRING_PLUS_TEXT, &args![temporary, name, TEXT_DOT])
            .u32();
        e.call(STRING_PLUS_STRING, &args![candidate, with_dot, extension]);
        destroy_string(e, temporary);
        let mut n = 0u32;
        loop {
            let full = base + 0x50;
            let path = e
                .call(STRING_PLUS_STRING, &args![full, directory, candidate])
                .u32();
            let exists = fn_00ac6e10(e, path);
            destroy_string(e, full);
            if !exists {
                break;
            }
            n = n.wrapping_add(1);
            e.call(SPRINTF, &args![number, TEXT_DECIMAL, n]);
            let first = base + 0x70;
            let second = base + 0x90;
            let third = base + 0xb0;
            let numbered = e.call(STRING_PLUS_TEXT, &args![first, name, number]).u32();
            let dotted = e
                .call(STRING_PLUS_TEXT, &args![second, numbered, TEXT_DOT])
                .u32();
            let next = e
                .call(STRING_PLUS_STRING, &args![third, dotted, extension])
                .u32();
            assign_string(e, candidate, next);
            destroy_string(e, third);
            destroy_string(e, second);
            destroy_string(e, first);
        }
        e.call(STRING_TIDY, &args![result, 0u32, 0u32]);
        assign_string(e, result, candidate);
        destroy_string(e, candidate);
    });
    result
}

// Translated from 00ac71f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac71f0 - the compiler's exception frame (FS:[0] chain) and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map (cdecl: the result string, the source string): the source
/// without the characters `*?"<>|:\/` and spaces (each kept one appended as a one-character
/// text, `00a0c430`). The result is made empty and assigned it; returns it.
pub fn fn_00ac71f0(e: &mut Engine, result: u32, source: u32) -> u32 {
    e.with_stack(0x24, |e, locals| {
        let kept = locals.addr();
        let character = locals.addr() + 0x20;
        e.call(STRING_FROM_TEXT, &args![kept, TEXT_EMPTY]);
        e.mem.set_u8(character, 0);
        e.mem.set_u8(character + 1, 0);
        let mut i = 0u32;
        while i < e.mem.u32(source + STRING_LENGTH) {
            check(e, i <= e.mem.u32(source + STRING_LENGTH));
            let chars = string_chars(e, source);
            let c = e.mem.u8(chars.wrapping_add(i));
            e.mem.set_u8(character, c);
            let skipped = matches!(
                c,
                b'*' | b'?' | b'"' | b'<' | b'>' | b'|' | b':' | b'\\' | b'/' | b' '
            );
            if !skipped {
                let count = e.call(STRLEN, &args![character]).u32();
                e.call(STRING_APPEND_TEXT, &args![kept, character, count]);
            }
            i += 1;
        }
        e.call(STRING_TIDY, &args![result, 0u32, 0u32]);
        assign_string(e, result, kept);
        destroy_string(e, kept);
    });
    result
}

/// The byte `index` of the byte vector at `vector`, after the library's bounds check.
fn byte_at(e: &mut Engine, vector: u32, index: u32) -> u32 {
    let size = e
        .mem
        .u32(vector + VECTOR_LAST)
        .wrapping_sub(e.mem.u32(vector + VECTOR_FIRST));
    check(e, index < size);
    e.mem.u32(vector + VECTOR_FIRST).wrapping_add(index)
}

/// Appends the character `c` (through a stack byte) to the container at `out`.
fn append_char(e: &mut Engine, out: u32, c: u8) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u8(slot.addr(), c);
        e.call(CHARS_APPEND, &args![out, slot]);
    });
}

/// A hex digit's character (`0`-`9`, `A`-`F`).
fn hex_char(digit: u8) -> u8 {
    if i32::from(digit) < 10 {
        digit.wrapping_add(0x30)
    } else {
        digit.wrapping_add(0x37)
    }
}

// Translated from 00ac7380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: a byte vector, the character container to fill): clears
/// the container (`00acae90`) and appends each byte as two upper-case hex digits (`00acad60`),
/// with a space before every second byte after the first.
pub fn fn_00ac7380(e: &mut Engine, bytes: u32, out: u32) {
    e.call(CHARS_CLEAR, &args![out]);
    let size = |e: &Engine| {
        e.mem
            .u32(bytes + VECTOR_LAST)
            .wrapping_sub(e.mem.u32(bytes + VECTOR_FIRST))
    };
    let mut i = 0u32;
    while i < size(e) {
        if (i as i32) > 0 && (i as i32).wrapping_rem(2) == 0 {
            append_char(e, out, 0x20);
        }
        let at = byte_at(e, bytes, i);
        let high = (u32::from(e.mem.u8(at)) >> 4) as u8;
        let at = byte_at(e, bytes, i);
        let low = (i32::from(e.mem.i8(at)) & 0xf) as u8;
        let high = hex_char(high);
        let low = hex_char(low);
        append_char(e, out, high);
        append_char(e, out, low);
        i += 1;
    }
}

/// A hex digit's value added to `base` as the code does it: `A`-`F`, `a`-`f`, then `0`-`9`
/// (signed compares); anything else leaves `base`. With `through_at` the decimal test and
/// value read the character through `00acad30` (the second digit), otherwise inline.
fn hex_value(e: &mut Engine, text: u32, index: u32, through_at: bool, base: u8) -> u8 {
    let c = |e: &mut Engine| {
        let at = byte_at(e, text, index);
        i32::from(e.mem.i8(at))
    };
    if c(e) >= 0x41 && c(e) <= 0x46 {
        return (i32::from(base) + c(e) - 0x37) as u8;
    }
    if c(e) >= 0x61 && c(e) <= 0x66 {
        return (i32::from(base) + c(e) - 0x57) as u8;
    }
    if c(e) < 0x30 {
        return base;
    }
    let digit = |e: &mut Engine| {
        if through_at {
            let at = e.call(BYTES_AT, &args![text, index]).u32();
            i32::from(e.mem.i8(at))
        } else {
            c(e)
        }
    };
    if digit(e) > 0x39 {
        return base;
    }
    (i32::from(base) + digit(e) - 0x30) as u8
}

// Translated from 00ac74a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: a text byte vector, the character container to fill):
/// clears the container (`00acae90`) and appends a byte (`00acad60`) for each pair of hex
/// digits, skipping spaces; it stops at the end, a 0 in either digit, or a single digit left.
/// A character that is no hex digit counts as 0.
pub fn fn_00ac74a0(e: &mut Engine, text: u32, out: u32) {
    e.call(CHARS_CLEAR, &args![out]);
    let count = e
        .mem
        .u32(text + VECTOR_LAST)
        .wrapping_sub(e.mem.u32(text + VECTOR_FIRST)) as i32;
    let mut i = 0u32;
    while (i as i32) < count {
        if (i.wrapping_add(1) as i32) >= count {
            return;
        }
        let at = byte_at(e, text, i);
        if e.mem.i8(at) == 0x20 {
            i += 1;
            continue;
        }
        let at = byte_at(e, text, i);
        if e.mem.i8(at) == 0 {
            return;
        }
        let at = byte_at(e, text, i.wrapping_add(1));
        if e.mem.i8(at) == 0 {
            return;
        }
        let high = hex_value(e, text, i, false, 0);
        let value = high.wrapping_shl(4);
        i += 1;
        let value = hex_value(e, text, i, true, value);
        append_char(e, out, value);
        i += 1;
    }
}

// Translated from 00ac7870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two bytes): their sum, at most 255 (`AL`).
pub fn fn_00ac7870(_e: &mut Engine, a: u8, b: u8) -> u8 {
    let sum = i32::from(a) + i32::from(b);
    let clamped = if 0xff >= sum { sum } else { 0xff };
    clamped as u8
}

// Translated from 00ac78b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two signed bytes): their sum, clamped to -128..127
/// (`AL`).
pub fn fn_00ac78b0(_e: &mut Engine, a: i8, b: i8) -> u8 {
    let sum = i32::from(a) + i32::from(b);
    let high = if 0x7f >= sum { sum } else { 0x7f };
    let clamped = if -0x80 >= high { -0x80 } else { high };
    clamped as u8
}

// Translated from 00ac7920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two `short`s): their sum, clamped to -32768..32767, but
/// answered as its low byte sign-extended (`MOVSX AX, byte`), in `AX`.
pub fn fn_00ac7920(_e: &mut Engine, a: i16, b: i16) -> i16 {
    let sum = i32::from(a) + i32::from(b);
    let high = if 0x7fff >= sum { sum } else { 0x7fff };
    let clamped = if -0x8000 >= high { -0x8000 } else { high };
    i16::from(clamped as u8 as i8)
}

// Translated from 00ac7990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl, two `int`s): their sum (`FILD`, `FIADD`, kept as a
/// `double`), `0x80000000` below -2147483648, `0x7fffffff` above 2147483647 (a NaN passes both
/// tests), else converted by `_ftol2`.
pub fn fn_00ac7990(e: &mut Engine, a: i32, b: i32) -> u32 {
    let sum = x87_add(f64::from(a), f64::from(b));
    let minimum: f64 = e.global(INT_MINIMUM);
    // FLD minimum; FCOMP sum; TEST AH,0x41; JNZ: the low clamp only when above the sum.
    if minimum > sum {
        return 0x8000_0000;
    }
    let maximum: f64 = e.global(INT_MAXIMUM);
    // FLD maximum; FCOMP sum; TEST AH,5; JP: the high clamp only when below the sum.
    if maximum < sum {
        return 0x7fff_ffff;
    }
    let bits = sum.to_bits();
    e.call(FTOL2, &args![bits as u32, (bits >> 32) as u32])
        .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ac_68e0, fn_00ac68e0()),
        entry!(0x00ac_6900, fn_00ac6900(u32) -> u32),
        entry!(0x00ac_6950, fn_00ac6950(u32) -> u32),
        entry!(0x00ac_6990, fn_00ac6990(u32, u32, u32, u32)),
        entry!(0x00ac_6a30, fn_00ac6a30(StringArg, u32, u32, u32) -> bool),
        entry!(0x00ac_6e10, fn_00ac6e10(u32) -> bool),
        entry!(0x00ac_6e70, fn_00ac6e70(u32) -> u32),
        entry!(0x00ac_6fa0, fn_00ac6fa0(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_71f0, fn_00ac71f0(u32, u32) -> u32),
        entry!(0x00ac_7380, fn_00ac7380(u32, u32)),
        entry!(0x00ac_74a0, fn_00ac74a0(u32, u32)),
        entry!(0x00ac_7870, fn_00ac7870(u8, u8) -> u8),
        entry!(0x00ac_78b0, fn_00ac78b0(i8, i8) -> u8),
        entry!(0x00ac_7920, fn_00ac7920(i16, i16) -> i16),
        entry!(0x00ac_7990, fn_00ac7990(i32, i32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// A string at `at` holding `text` inline (at most 15 characters).
    fn set_string(e: &mut Engine, at: u32, text: &[u8]) {
        e.mem.write(at + 4, &[0u8; 16]);
        e.mem.write(at + 4, text);
        e.mem.set_u32(at + STRING_LENGTH, text.len() as u32);
        e.mem.set_u32(at + 0x18, 0xf);
    }

    fn string_text(e: &Engine, at: u32) -> Vec<u8> {
        let length = e.mem.u32(at + STRING_LENGTH);
        e.mem.bytes(at + 4, length)
    }

    fn new_string(e: &mut Engine, text: &[u8]) -> u32 {
        let at = e.mem.alloc(0x1c);
        set_string(e, at, text);
        at
    }

    /// Strings as inline text: assign, rfind, substr, the concatenations, the text append and
    /// the construction from text; the C string functions; quiet asserts and checks.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global(STRING_NPOS, u32::MAX);
        e.register(STRING_ASSIGN, |e, a| {
            let text = string_text(e, a[1]);
            set_string(e, a[0], &text);
            Ret::default()
        });
        e.register(STRING_RFIND, |e, a| {
            let text = string_text(e, a[0]);
            let needle = e.mem.bytes(a[1], a[3]);
            let found = text.windows(needle.len()).rposition(|w| w == needle);
            found.map_or(u32::MAX, |i| i as u32).into_ret()
        });
        e.register(STRING_SUBSTR, |e, a| {
            let text = string_text(e, a[0]);
            let part = text[a[2] as usize..(a[2] + a[3]) as usize].to_vec();
            set_string(e, a[1], &part);
            a[1].into_ret()
        });
        e.register(STRING_PLUS_TEXT, |e, a| {
            let mut text = string_text(e, a[1]);
            text.extend(e.mem.cstr(a[2]));
            set_string(e, a[0], &text);
            a[0].into_ret()
        });
        e.register(STRING_PLUS_STRING, |e, a| {
            let mut text = string_text(e, a[1]);
            text.extend(string_text(e, a[2]));
            set_string(e, a[0], &text);
            a[0].into_ret()
        });
        e.register(STRING_FROM_TEXT, |e, a| {
            let text = e.mem.cstr(a[1]);
            set_string(e, a[0], &text);
            a[0].into_ret()
        });
        e.register(STRING_APPEND_TEXT, |e, a| {
            let mut text = string_text(e, a[0]);
            text.extend(e.mem.bytes(a[1], a[2]));
            set_string(e, a[0], &text);
            Ret::default()
        });
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(STRCHR, |e, a| {
            let text = e.mem.cstr(a[0]);
            let found = text.iter().position(|&c| u32::from(c) == a[1]);
            found.map_or(0, |i| a[0] + i as u32).into_ret()
        });
        e.register(STRPBRK, |e, a| {
            let text = e.mem.cstr(a[0]);
            let set = e.mem.cstr(a[1]);
            let found = text.iter().position(|c| set.contains(c));
            found.map_or(0, |i| a[0] + i as u32).into_ret()
        });
        e.register(SPRINTF, |e, a| {
            let text = format!("{}", a[2]);
            e.mem.set_cstr(a[0], text.as_bytes());
            Ret::default()
        });
        for (addr, text) in [
            (TEXT_BACKSLASH, &b"\\"[..]),
            (TEXT_SLASH, b"/"),
            (TEXT_COLON, b":"),
            (TEXT_DOT, b"."),
            (TEXT_SEPARATORS, b"\\/"),
            (TEXT_EMPTY, b""),
        ] {
            e.map(addr, 4);
            e.mem.set_cstr(addr, text);
        }
        quiet(
            &mut e,
            &[STRING_TIDY, FR2_ASSERT, INVALID_PARAMETER, FCLOSE],
        );
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn last_error_ends_the_process() {
        let mut e = engine();
        e.map(LAST_ERROR, 4);
        quiet(&mut e, &[PRINT_LAST_ERROR, EXIT]);
        e.call(0x00ac_68e0, &[]);
        assert!(calls(&e, EXIT).is_empty());
        e.mem.set_u32(LAST_ERROR, 3);
        e.call(0x00ac_68e0, &[]);
        assert_eq!(calls(&e, PRINT_LAST_ERROR).len(), 1);
        assert_eq!(calls(&e, EXIT), vec![vec![u32::MAX]]);
    }

    #[test]
    fn highest_bit() {
        let mut e = engine();
        assert_eq!(e.call(0x00ac_6900, &args![1u32]).u32(), 0);
        assert_eq!(e.call(0x00ac_6900, &args![8u32]).u32(), 3);
        assert_eq!(e.call(0x00ac_6900, &args![0x8000_0001u32]).u32(), 31);
        assert!(calls(&e, FR2_ASSERT).is_empty());
        assert_eq!(e.call(0x00ac_6900, &args![0u32]).u32(), 0);
        assert_eq!(calls(&e, FR2_ASSERT), vec![vec![ALGS_SOURCE, 0x27]]);
    }

    #[test]
    fn bits_rounded_up() {
        let mut e = engine();
        assert_eq!(e.call(0x00ac_6950, &args![8u32]).u32(), 3);
        assert_eq!(e.call(0x00ac_6950, &args![9u32]).u32(), 4);
        assert_eq!(e.call(0x00ac_6950, &args![1u32]).u32(), 0);
    }

    #[test]
    fn kth_pair() {
        let mut e = engine();
        let i = e.mem.alloc(4);
        let j = e.mem.alloc(4);
        e.call(0x00ac_6990, &args![3u32, 2u32, i, j]);
        assert_eq!((e.mem.u32(i), e.mem.u32(j)), (1, 2));
        e.call(0x00ac_6990, &args![3u32, 0u32, i, j]);
        assert_eq!((e.mem.u32(i), e.mem.u32(j)), (0, 1));
        assert!(calls(&e, FR2_ASSERT).is_empty());
        e.call(0x00ac_6990, &args![3u32, 3u32, i, j]);
        assert_eq!(calls(&e, FR2_ASSERT), vec![vec![ALGS_SOURCE, 0x60]]);
        assert_eq!(e.mem.u32(i), 3);
    }

    /// Splits `path`; answers the result and the three parts.
    fn split(e: &mut Engine, path: &[u8]) -> (bool, Vec<u8>, Vec<u8>, Vec<u8>) {
        let source = new_string(e, path);
        let words = StringArg::load(e, source);
        let parts = [
            new_string(e, b"?"),
            new_string(e, b"?"),
            new_string(e, b"?"),
        ];
        let mut a = words.0.to_vec();
        a.extend(parts);
        let ok = e.call(0x00ac_6a30, &a).bool();
        (
            ok,
            string_text(e, parts[0]),
            string_text(e, parts[1]),
            string_text(e, parts[2]),
        )
    }

    #[test]
    fn path_split() {
        let mut e = engine();
        assert_eq!(
            split(&mut e, b"a\\b.c"),
            (true, b"a\\".to_vec(), b"b".to_vec(), b"c".to_vec())
        );
        assert_eq!(
            split(&mut e, b"a/b"),
            (true, b"a/".to_vec(), b"b".to_vec(), vec![])
        );
        // A colon after the separator, a path ending at the name start, a dot right there.
        assert!(!split(&mut e, b"a\\b:c").0);
        assert!(!split(&mut e, b"c:").0);
        assert!(!split(&mut e, b"a\\.c").0);
        // A dot before the name: the name runs to the end, the extension from after the dot.
        assert_eq!(
            split(&mut e, b"a.b\\c"),
            (true, b"a.b\\".to_vec(), b"c".to_vec(), b"b\\c".to_vec())
        );
        assert!(calls(&e, FR2_ASSERT).is_empty());
        // The path argument destroyed every time.
        assert_eq!(
            calls(&e, STRING_TIDY)
                .iter()
                .filter(|c| c[1..] == [1, 0])
                .count(),
            3 * 4 + 3
        );
    }

    #[test]
    fn file_exists() {
        let mut e = engine();
        let name = new_string(&mut e, b"f");
        e.register(FOPEN, |_, _| 0x5000u32.into_ret());
        assert!(e.call(0x00ac_6e10, &args![name]).bool());
        assert_eq!(calls(&e, FOPEN), vec![vec![name + 4, TEXT_READ_MODE]]);
        assert_eq!(calls(&e, FCLOSE), vec![vec![0x5000]]);
        e.register(FOPEN, |_, _| Ret::default());
        assert!(!e.call(0x00ac_6e10, &args![name]).bool());
        assert_eq!(calls(&e, FCLOSE).len(), 1);
    }

    fn rest(e: &mut Engine, path: &[u8]) -> Option<Vec<u8>> {
        let at = e.mem.alloc(0x40);
        e.mem.set_cstr(at, path);
        let r = e.call(0x00ac_6e70, &args![at]).u32();
        (r != 0).then(|| e.mem.cstr(r))
    }

    #[test]
    fn path_after_its_root() {
        let mut e = engine();
        assert_eq!(rest(&mut e, b"C:\\dir\\x"), Some(b"dir\\x".to_vec()));
        assert_eq!(rest(&mut e, b"C:x"), Some(b"x".to_vec()));
        assert_eq!(rest(&mut e, b"\\\\srv\\share\\f"), Some(b"f".to_vec()));
        assert_eq!(rest(&mut e, b"dir\\file"), Some(b"file".to_vec()));
        assert_eq!(rest(&mut e, b"dir\\"), None);
        assert_eq!(rest(&mut e, b"file"), None);
        assert_eq!(rest(&mut e, b""), None);
        assert_eq!(e.call(0x00ac_6e70, &args![0u32]).u32(), 0);
    }

    #[test]
    fn unused_file_name() {
        let mut e = engine();
        // "d/n.e" and "d/n1.e" exist, "d/n2.e" does not.
        e.register(FOPEN, |e, a| {
            let name = e.mem.cstr(a[0]);
            u32::from(name != b"d/n2.e").into_ret()
        });
        let directory = new_string(&mut e, b"d/");
        let name = new_string(&mut e, b"n");
        let extension = new_string(&mut e, b"e");
        let result = new_string(&mut e, b"?");
        let r = e.call(0x00ac_6fa0, &args![result, directory, name, extension]);
        assert_eq!(r.u32(), result);
        assert_eq!(string_text(&e, result), b"n2.e");
        assert_eq!(calls(&e, FOPEN).len(), 3);
    }

    #[test]
    fn file_name_without_reserved_characters() {
        let mut e = engine();
        let source = new_string(&mut e, b"a*b c:d/e?");
        let result = new_string(&mut e, b"?");
        assert_eq!(e.call(0x00ac_71f0, &args![result, source]).u32(), result);
        assert_eq!(string_text(&e, result), b"abcde");
    }

    /// A byte vector over `bytes`, and a character container whose appends land in a buffer
    /// (its count at +0, the characters from +4).
    fn hex_engine(bytes: &[u8]) -> (Engine, u32, u32) {
        let mut e = engine();
        let data = e.mem.alloc(bytes.len().max(1) as u32);
        e.mem.write(data, bytes);
        let vector = e.mem.alloc(0x18);
        e.mem.set_u32(vector + VECTOR_FIRST, data);
        e.mem
            .set_u32(vector + VECTOR_LAST, data + bytes.len() as u32);
        let out = e.mem.alloc(0x40);
        e.register(CHARS_CLEAR, |e, a| {
            e.mem.set_u32(a[0], 0);
            Ret::default()
        });
        e.register(CHARS_APPEND, |e, a| {
            let n = e.mem.u32(a[0]);
            let c = e.mem.u8(a[1]);
            e.mem.set_u8(a[0] + 4 + n, c);
            e.mem.set_u32(a[0], n + 1);
            Ret::default()
        });
        e.register(BYTES_AT, |e, a| {
            (e.mem.u32(a[0] + VECTOR_FIRST) + a[1]).into_ret()
        });
        (e, vector, out)
    }

    fn appended(e: &Engine, out: u32) -> Vec<u8> {
        let n = e.mem.u32(out);
        e.mem.bytes(out + 4, n)
    }

    #[test]
    fn bytes_to_hex() {
        let (mut e, vector, out) = hex_engine(&[0x1f, 0xa0, 0x05]);
        e.call(0x00ac_7380, &args![vector, out]);
        assert_eq!(appended(&e, out), b"1FA0 05");
        assert_eq!(calls(&e, INVALID_PARAMETER).len(), 0);
    }

    #[test]
    fn hex_to_bytes() {
        let (mut e, vector, out) = hex_engine(b"1f A0 9z 5");
        e.call(0x00ac_74a0, &args![vector, out]);
        // "1f", "A0", "9z" (z counts as nothing: 0x90), then a single digit left.
        assert_eq!(appended(&e, out), vec![0x1f, 0xa0, 0x90]);
        assert_eq!(calls(&e, BYTES_AT).len(), 3);
        // A 0 stops it.
        let (mut e, vector, out) = hex_engine(b"12\x0034");
        e.call(0x00ac_74a0, &args![vector, out]);
        assert_eq!(appended(&e, out), vec![0x12]);
    }

    #[test]
    fn byte_sum_clamps() {
        let mut e = engine();
        assert_eq!(
            e.call(0x00ac_7870, &args![200u32, 100u32]).u32() & 0xff,
            0xff
        );
        assert_eq!(e.call(0x00ac_7870, &args![20u32, 100u32]).u32() & 0xff, 120);
    }

    #[test]
    fn signed_byte_sum_clamps() {
        let mut e = engine();
        assert_eq!(
            e.call(0x00ac_78b0, &args![100u32, 100u32]).u32() & 0xff,
            0x7f
        );
        assert_eq!(
            e.call(0x00ac_78b0, &args![0x9cu32, 0x9cu32]).u32() & 0xff,
            0x80
        );
        assert_eq!(e.call(0x00ac_78b0, &args![0xfeu32, 3u32]).u32() & 0xff, 1);
    }

    #[test]
    fn short_sum_answers_its_low_byte() {
        let mut e = engine();
        // 0x7fff clamped, low byte 0xff sign-extended.
        assert_eq!(
            e.call(0x00ac_7920, &args![0x7000u32, 0x7000u32]).u32() & 0xffff,
            0xffff
        );
        // 0x0110: the low byte 0x10.
        assert_eq!(
            e.call(0x00ac_7920, &args![0x100u32, 0x10u32]).u32() & 0xffff,
            0x10
        );
        // -0x8000 clamped: low byte 0.
        assert_eq!(
            e.call(0x00ac_7920, &args![0x8000u32, 0x8000u32]).u32() & 0xffff,
            0
        );
    }

    #[test]
    fn int_sum_clamps_or_converts() {
        let mut e = engine();
        for (addr, value) in [
            (INT_MINIMUM, -2_147_483_648.0f64),
            (INT_MAXIMUM, 2_147_483_647.0),
        ] {
            e.map(addr, 8);
            e.mem.set_f64(addr, value);
        }
        e.register(FTOL2, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32));
            (value as i32 as u32).into_ret()
        });
        assert_eq!(e.call(0x00ac_7990, &args![5i32, 7i32]).u32(), 12);
        assert_eq!(
            e.call(0x00ac_7990, &args![i32::MAX, 1i32]).u32(),
            0x7fff_ffff
        );
        assert_eq!(
            e.call(0x00ac_7990, &args![i32::MIN, -1i32]).u32(),
            0x8000_0000
        );
        assert_eq!(calls(&e, FTOL2).len(), 1);
    }
}
