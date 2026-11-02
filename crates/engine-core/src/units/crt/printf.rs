//! The C runtime's string `printf` family and its floating-point helpers (MSVC 2008 LIBCMT,
//! statically linked into FalloutNV.exe 1.4.0.525), translated from the exe: `sprintf`
//! (`00ec623a`), `vsprintf` (`00ec6cc4`) and `_vsprintf_l` (`00ec6c46`), `vsprintf_s`
//! (`00ec6bb5`), `_vsprintf_s_l` (`00ec6b2d`) and `_vsnprintf_helper` (`00ec6a61`), `_snprintf`
//! (`00eca38d`), `_vsnprintf_l` (`00ecdd3e`), the secure output core `_output_s_l`
//! (`00ed4685`), and the floating-point helpers `_output_l`/`_output_s_l` call through the
//! encoded `_cfltcvt_tab`: `__cfltcvt_l` (`00ed23e3`) with `__cfltcvt` (`00ed246b`), `__cftog_l`
//! (`00ed22e9`), `__forcdecpt_l` (`00ed1996`) with `__forcdecpt` (`00ed1b22`), `__cropzeros_l`
//! (`00ed1a09`) with `__cropzeros` (`00ed1b35`), `__positive` (`00ed1a8b`) and `__fassign`
//! (`00ed1ae9`). The names are Ghidra's function-ID labels.
//!
//! The string functions write through a `FILE` on their own frame (`_IOWRT | _IOSTRG`, 0x42;
//! `_ptr` and `_base` the buffer, `_cnt` the room capped at `0x7fffffff`), as the exe does;
//! the `printf` core (`_output_l`, `00ed2f21`, in `crt.rs`) and `_flsbuf` (`00ed2d64`) do the
//! rest. They replace the platform stand-ins that formatted with host code.

use super::*;

/// `_errno` (`00ec85e3`) and the invalid-parameter report (`00ec7c30`, five words), cdecl.
const ERRNO_ADDR: u32 = 0x00ec_85e3;
const INVALID_PARAMETER_ADDR: u32 = 0x00ec_7c30;
/// `_output_l` (`00ed2f21`) and `_output_s_l` (`00ed4685`), cdecl: stream, format, locale,
/// argument list.
const OUTPUT_L_ADDR: u32 = 0x00ed_2f21;
const OUTPUT_S_L_ADDR: u32 = 0x00ed_4685;
/// `_vsnprintf_helper` (`00ec6a61`), `_vsprintf_l` (`00ec6c46`), `_vsprintf_s_l` (`00ec6b2d`),
/// `__cfltcvt_l` (`00ed23e3`), `__forcdecpt_l` (`00ed1996`) and `__cropzeros_l` (`00ed1a09`).
const VSNPRINTF_HELPER_ADDR: u32 = 0x00ec_6a61;
const VSPRINTF_L_ADDR: u32 = 0x00ec_6c46;
const VSPRINTF_S_L_ADDR: u32 = 0x00ec_6b2d;
const CFLTCVT_L_ADDR: u32 = 0x00ed_23e3;
const FORCDECPT_L_ADDR: u32 = 0x00ed_1996;
const CROPZEROS_L_ADDR: u32 = 0x00ed_1a09;
/// The conversions `__cfltcvt_l` picks between: `__cftoe_l` (`00ed1cb7`), `__cftof_l`
/// (`00ed2211`), `__cftoa_l` (`00ed1da7`, not translated: `%a`) and `__cftog_l` (`00ed22e9`);
/// `__cftog_l`'s helpers `__fltout2` (`00edf36b`), `__fptostr` (`00ee18cb`), `__cftof2_l`
/// (`00ed211a`; `EAX` the `_strflt`, `ECX` the buffer) and `__cftoe2_l` (`00ed1b48`; `EAX` the
/// buffer); `__fassign_l` (`00ed1aa7`).
const CFTOE_L_ADDR: u32 = 0x00ed_1cb7;
const CFTOF_L_ADDR: u32 = 0x00ed_2211;
const CFTOA_L_ADDR: u32 = 0x00ed_1da7;
const CFTOG_L_ADDR: u32 = 0x00ed_22e9;
const FLTOUT2_ADDR: u32 = 0x00ed_f36b;
const FPTOSTR_ADDR: u32 = 0x00ee_18cb;
const CFTOF2_L_ADDR: u32 = 0x00ed_211a;
const CFTOE2_L_ADDR: u32 = 0x00ed_1b48;
const FASSIGN_L_ADDR: u32 = 0x00ed_1aa7;
/// `tolower` (`00ec67aa`) and `isdigit` (`00ec8860`), cdecl.
const TOLOWER_ADDR: u32 = 0x00ec_67aa;
const ISDIGIT_ADDR: u32 = 0x00ec_8860;

/// The string stream's frame `FILE` (0x20 bytes at `EBP-0x20`): `_ptr` +0, `_cnt` +4, `_base`
/// +8, `_flag` +0xc (the rest left as the frame holds it).
const FILE_SIZE: u32 = 0x20;
/// `_IOWRT | _IOSTRG`.
const STRING_STREAM: u32 = 0x42;
/// The room a string stream gets at most.
const ROOM_MAX: u32 = 0x7fff_ffff;

/// `*_errno() = code` then the invalid-parameter report with five zeros, the exe's order.
fn report(e: &mut Engine, code: u32) {
    let slot = e.call(ERRNO_ADDR, &args![]).u32();
    e.mem.set_u32(slot, code);
    e.call(INVALID_PARAMETER_ADDR, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
}

/// Makes the frame's string stream: `_ptr` and `_base` the buffer, `_cnt` the room, `_flag`
/// 0x42 (in the exe's store order the values are the same).
fn string_stream(e: &mut Engine, file: u32, buffer: u32, room: u32) {
    e.mem.set_u32(file + 4, room);
    e.mem.set_u32(file + 0xc, STRING_STREAM);
    e.mem.set_u32(file + 8, buffer);
    e.mem.set_u32(file, buffer);
}

/// The terminating zero the string functions put at the stream's end: `_cnt` counted down;
/// still non-negative, a zero stored at `_ptr`; else `_flsbuf(0, stream)`, whose answer is
/// given (`None` when the zero was stored).
fn put_terminator(e: &mut Engine, file: u32) -> Option<u32> {
    let left = e.mem.i32(file + 4).wrapping_sub(1);
    e.mem.set_i32(file + 4, left);
    if left >= 0 {
        let at = e.mem.u32(file);
        e.mem.set_u8(at, 0);
        None
    } else {
        Some(e.call(FLSBUF, &args![0u32, file]).u32())
    }
}

/// Copies variable-argument words to game memory (as the caller pushed them) and runs `f`
/// with the address of the first: the `va_list` of a cdecl variadic function.
fn with_arguments<R>(e: &mut Engine, words: &[u32], f: impl FnOnce(&mut Engine, u32) -> R) -> R {
    let size = 4 * (words.len() as u32).max(1);
    e.with_stack(size, |e, list| {
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(list.addr() + 4 * i as u32, *word);
        }
        f(e, list.addr())
    })
}

// Translated from 00ec623a (disassembled, FalloutNV.exe 1.4.0.525)
/// `sprintf` (Ghidra's function ID), cdecl: buffer, format, then the variable arguments (their
/// words here). A null format, then a null buffer, sets `errno` to `EINVAL`, reports and
/// answers -1. Otherwise `_output_l` writes into the frame's string stream (room
/// `0x7fffffff`) with a null locale and the arguments' address as the list; the terminating
/// zero is put after the text ([`put_terminator`]), and `_output_l`'s answer given.
pub fn sprintf(e: &mut Engine, buffer: u32, format: u32, words: &[u32]) -> i32 {
    if format == 0 || buffer == 0 {
        report(e, 0x16);
        return -1;
    }
    with_arguments(e, words, |e, list| {
        e.with_stack(FILE_SIZE, |e, file| {
            let file = file.addr();
            string_stream(e, file, buffer, ROOM_MAX);
            let result = e
                .call(OUTPUT_L_ADDR, &args![file, format, 0u32, list])
                .i32();
            put_terminator(e, file);
            result
        })
    })
}

/// The registry entry of [`sprintf`]: buffer, format, then the variable arguments.
fn sprintf_entry(e: &mut Engine, words: &[u32]) -> Ret {
    sprintf(e, words[0], words[1], &words[2..]).into_ret()
}

// Translated from 00ec6c46 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_vsprintf_l` (Ghidra's function ID), cdecl: buffer, format, locale, argument list. As
/// [`sprintf`] with the locale and list given: a null format, then a null buffer, is
/// `EINVAL` with the report and -1; otherwise `_output_l` into the string stream (room
/// `0x7fffffff`), the terminating zero, and `_output_l`'s answer.
pub fn vsprintf_l(e: &mut Engine, buffer: u32, format: u32, locale: u32, list: u32) -> i32 {
    if format == 0 || buffer == 0 {
        report(e, 0x16);
        return -1;
    }
    e.with_stack(FILE_SIZE, |e, file| {
        let file = file.addr();
        string_stream(e, file, buffer, ROOM_MAX);
        let result = e
            .call(OUTPUT_L_ADDR, &args![file, format, locale, list])
            .i32();
        put_terminator(e, file);
        result
    })
}

// Translated from 00ec6cc4 (disassembled, FalloutNV.exe 1.4.0.525)
/// `vsprintf` (Ghidra's function ID), cdecl: buffer, format, argument list; `_vsprintf_l` with
/// a null locale.
pub fn vsprintf(e: &mut Engine, buffer: u32, format: u32, list: u32) -> i32 {
    e.call(VSPRINTF_L_ADDR, &args![buffer, format, 0u32, list])
        .i32()
}

// Translated from 00ec6a61 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_vsnprintf_helper` (Ghidra's function ID), cdecl: the output function, buffer, count,
/// format, locale, argument list. A null format, or a null buffer with a nonzero count, is
/// `EINVAL` with the report and -1. Otherwise the output function writes into the frame's
/// string stream (room the count, at most `0x7fffffff`). With no buffer its answer is given.
/// With a non-negative answer the terminating zero is put ([`put_terminator`]) and the answer
/// given, unless `_flsbuf` answered -1 for it. Then (and for a negative answer) the last byte
/// of the buffer (`buffer[count - 1]`) is cleared and -1 answered when the room is not
/// negative, -2 when it is (the text did not fit).
pub fn vsnprintf_helper(
    e: &mut Engine,
    output: u32,
    buffer: u32,
    count: u32,
    format: u32,
    locale: u32,
    list: u32,
) -> i32 {
    if format == 0 || (count != 0 && buffer == 0) {
        report(e, 0x16);
        return -1;
    }
    e.with_stack(FILE_SIZE, |e, file| {
        let file = file.addr();
        let room = if count > ROOM_MAX { ROOM_MAX } else { count };
        string_stream(e, file, buffer, room);
        let result = e.call(output, &args![file, format, locale, list]).i32();
        if buffer == 0 {
            return result;
        }
        if result >= 0 {
            match put_terminator(e, file) {
                None => return result,
                Some(answer) if answer != 0xffff_ffff => return result,
                Some(_) => {}
            }
        }
        // 00ec6b1a: CMP [cnt], 0 before the store; SETGE; DEC; DEC.
        let fits = e.mem.i32(file + 4) >= 0;
        e.mem.set_u8(buffer.wrapping_add(count).wrapping_sub(1), 0);
        if fits {
            -1
        } else {
            -2
        }
    })
}

// Translated from 00ec6b2d (disassembled, FalloutNV.exe 1.4.0.525)
/// `_vsprintf_s_l` (Ghidra's function ID), cdecl: buffer, size, format, locale, argument list.
/// A null format, then a null buffer or a size of 0, is `EINVAL` with the report and -1.
/// Otherwise `_vsnprintf_helper(_output_s_l, ...)`; a negative answer clears the buffer's
/// first byte, and -2 (it did not fit) sets `errno` to `ERANGE` (0x22), reports and answers
/// -1. Any other answer is given.
pub fn vsprintf_s_l(
    e: &mut Engine,
    buffer: u32,
    size: u32,
    format: u32,
    locale: u32,
    list: u32,
) -> i32 {
    if format == 0 || buffer == 0 || size == 0 {
        report(e, 0x16);
        return -1;
    }
    let result = e
        .call(
            VSNPRINTF_HELPER_ADDR,
            &args![OUTPUT_S_L_ADDR, buffer, size, format, locale, list],
        )
        .i32();
    if result < 0 {
        e.mem.set_u8(buffer, 0);
    }
    if result == -2 {
        report(e, 0x22);
        return -1;
    }
    result
}

// Translated from 00ec6bb5 (disassembled, FalloutNV.exe 1.4.0.525)
/// `vsprintf_s` (Ghidra's function ID), cdecl: buffer, size, format, argument list;
/// `_vsprintf_s_l` with a null locale.
pub fn vsprintf_s(e: &mut Engine, buffer: u32, size: u32, format: u32, list: u32) -> i32 {
    e.call(VSPRINTF_S_L_ADDR, &args![buffer, size, format, 0u32, list])
        .i32()
}

// Translated from 00eca38d (disassembled, FalloutNV.exe 1.4.0.525)
/// `_snprintf` (Ghidra's function ID), cdecl: buffer, count, format, then the variable
/// arguments (their words here). A null format, or a null buffer with a nonzero count, is
/// `EINVAL` with the report and -1. Otherwise `_output_l` writes into the frame's string
/// stream (room the count, at most `0x7fffffff`) with a null locale; with a buffer the
/// terminating zero is put ([`put_terminator`]: a full buffer goes to `_flsbuf`, which writes
/// nothing). `_output_l`'s answer is given (-1 when the text did not fit).
pub fn snprintf(e: &mut Engine, buffer: u32, count: u32, format: u32, words: &[u32]) -> i32 {
    if format == 0 || (count != 0 && buffer == 0) {
        report(e, 0x16);
        return -1;
    }
    with_arguments(e, words, |e, list| {
        e.with_stack(FILE_SIZE, |e, file| {
            let file = file.addr();
            let room = if count > ROOM_MAX { ROOM_MAX } else { count };
            string_stream(e, file, buffer, room);
            let result = e
                .call(OUTPUT_L_ADDR, &args![file, format, 0u32, list])
                .i32();
            if buffer != 0 {
                put_terminator(e, file);
            }
            result
        })
    })
}

/// The registry entry of [`snprintf`]: buffer, count, format, then the variable arguments.
fn snprintf_entry(e: &mut Engine, words: &[u32]) -> Ret {
    snprintf(e, words[0], words[1], words[2], &words[3..]).into_ret()
}

// Translated from 00ecdd3e (disassembled, FalloutNV.exe 1.4.0.525)
/// `_vsnprintf_l` (Ghidra's function ID), cdecl: buffer, count, format, locale, argument list.
/// As [`snprintf`] with the locale and list given.
pub fn vsnprintf_l(
    e: &mut Engine,
    buffer: u32,
    count: u32,
    format: u32,
    locale: u32,
    list: u32,
) -> i32 {
    if format == 0 || (count != 0 && buffer == 0) {
        report(e, 0x16);
        return -1;
    }
    e.with_stack(FILE_SIZE, |e, file| {
        let file = file.addr();
        let room = if count > ROOM_MAX { ROOM_MAX } else { count };
        string_stream(e, file, buffer, room);
        let result = e
            .call(OUTPUT_L_ADDR, &args![file, format, locale, list])
            .i32();
        if buffer != 0 {
            put_terminator(e, file);
        }
        result
    })
}

// ---- Floating-point helpers ---------------------------------------------------------------

// Translated from 00ed23e3 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__cfltcvt_l` (Ghidra's function ID), cdecl: the double's address, buffer, size, the format
/// character, precision, capitals, locale. `'e'`/`'E'`: `__cftoe_l(value, buffer, size,
/// precision, capitals, locale)`; `'f'`: `__cftof_l(value, buffer, size, precision, locale)`;
/// `'a'`/`'A'`: `__cftoa_l` with the `%e` words; anything else `__cftog_l` with them. The
/// conversion's answer is given.
#[allow(clippy::too_many_arguments)]
pub fn cfltcvt_l(
    e: &mut Engine,
    value: u32,
    buffer: u32,
    size: u32,
    format: i32,
    precision: u32,
    capitals: u32,
    locale: u32,
) -> i32 {
    let target = match format {
        0x65 | 0x45 => CFTOE_L_ADDR,
        0x66 => {
            return e
                .call(CFTOF_L_ADDR, &args![value, buffer, size, precision, locale])
                .i32();
        }
        0x61 | 0x41 => CFTOA_L_ADDR,
        _ => CFTOG_L_ADDR,
    };
    e.call(
        target,
        &args![value, buffer, size, precision, capitals, locale],
    )
    .i32()
}

// Translated from 00ed246b (disassembled, FalloutNV.exe 1.4.0.525)
/// `__cfltcvt` (Ghidra's function ID), cdecl: `__cfltcvt_l` with a null locale.
pub fn cfltcvt(
    e: &mut Engine,
    value: u32,
    buffer: u32,
    size: u32,
    format: i32,
    precision: u32,
    capitals: u32,
) -> i32 {
    e.call(
        CFLTCVT_L_ADDR,
        &args![value, buffer, size, format, precision, capitals, 0u32],
    )
    .i32()
}

/// `__cftog_l`'s frame (`SUB ESP,0x30`): the `_strflt` at `EBP-0x30` (sign, `decpt`, flag,
/// digits' address), the magnitude before rounding at `EBP-0x20` and the 0x16 digit bytes at
/// `EBP-0x1c`, as offsets from the frame's bottom.
const CFTOG_FRAME: u32 = 0x30;
const CFTOG_MAGNITUDE: u32 = 0x10;
const CFTOG_DIGITS: u32 = 0x14;

// Translated from 00ed22e9 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed22e9 - the /GS stack cookie check (00ec408c) is not translated.
/// `__cftog_l` (Ghidra's function ID), cdecl: the double's address, buffer, size, precision,
/// capitals, locale. `__fltout2` fills the `_strflt` and its digits; then a null buffer or a
/// size of 0 is `EINVAL` (0x16) with the report and answered. The magnitude is `decpt - 1`;
/// `__fptostr` rounds to `precision` digits into the buffer after the sign's place (the size
/// less that place, unless it is -1); its error clears the buffer's first byte and is
/// answered. With the new magnitude (`decpt - 1` after rounding) from -4 up to below the
/// precision (signed) the `%f` layout is taken: when rounding raised the magnitude the last
/// digit is dropped (the byte before the digits' end cleared), then `__cftof2_l(flt, buffer,
/// size, precision, 1, locale)`; otherwise `__cftoe2_l(buffer, size, precision, capitals, flt,
/// 1, locale)`. Their answer is given.
pub fn cftog_l(
    e: &mut Engine,
    value: u32,
    buffer: u32,
    size: u32,
    precision: i32,
    capitals: u32,
    locale: u32,
) -> i32 {
    e.with_stack(CFTOG_FRAME, |e, frame| {
        let flt = frame.addr();
        let magnitude_at = flt + CFTOG_MAGNITUDE;
        let digits = flt + CFTOG_DIGITS;
        let low = e.mem.u32(value);
        let high = e.mem.u32(value.wrapping_add(4));
        e.call(FLTOUT2_ADDR, &args![low, high, flt, digits, 0x16u32]);
        if buffer == 0 || size == 0 {
            report(e, 0x16);
            return 0x16;
        }
        let decpt = e.mem.u32(flt + 4);
        e.mem.set_u32(magnitude_at, decpt.wrapping_sub(1));
        let negative = u32::from(e.mem.u32(flt) == 0x2d);
        let mut place = negative.wrapping_add(buffer);
        let room = if size == 0xffff_ffff {
            size
        } else {
            size.wrapping_sub(negative)
        };
        let error = e
            .call(FPTOSTR_ADDR, &args![place, room, precision as u32, flt])
            .u32();
        if error != 0 {
            e.mem.set_u8(buffer, 0);
            return error as i32;
        }
        let magnitude = e.mem.u32(flt + 4).wrapping_sub(1) as i32;
        let raised = (e.mem.u32(magnitude_at) as i32) < magnitude;
        if magnitude < -4 || magnitude >= precision {
            return e
                .call(
                    CFTOE2_L_ADDR,
                    &args![buffer, size, precision as u32, capitals, flt, 1u32, locale],
                )
                .i32();
        }
        if raised {
            // 00ed2394: to one past the digits' zero, then the byte before the zero cleared.
            loop {
                let byte = e.mem.u8(place);
                place = place.wrapping_add(1);
                if byte == 0 {
                    break;
                }
            }
            e.mem.set_u8(place.wrapping_sub(2), 0);
        }
        e.call(
            CFTOF2_L_ADDR,
            &args![flt, buffer, size, precision as u32, 1u32, locale],
        )
        .i32()
    })
}

// Translated from 00ed1996 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__forcdecpt_l` (Ghidra's function ID), cdecl: buffer, locale. Takes the locale
/// (`_LocaleUpdate`, `EBP-0x10`). Unless the first character is an exponent (`tolower` of it is
/// `'e'`) the digits after it are skipped (`isdigit`); a following `'x'` (`tolower`) skips two
/// more. The locale's decimal point is put there and the rest of the text moves up by one, its
/// zero included.
pub fn forcdecpt_l(e: &mut Engine, buffer: u32, locale: u32) {
    e.with_stack(0x10, |e, frame| {
        let update = frame.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let mut at = buffer;
        let first = i32::from(e.mem.u8(at) as i8) as u32;
        if e.call(TOLOWER_ADDR, &args![first]).u32() != 0x65 {
            loop {
                at = at.wrapping_add(1);
                let byte = u32::from(e.mem.u8(at));
                if e.call(ISDIGIT_ADDR, &args![byte]).u32() == 0 {
                    break;
                }
            }
        }
        let current = i32::from(e.mem.u8(at) as i8) as u32;
        if e.call(TOLOWER_ADDR, &args![current]).u32() == 0x78 {
            at = at.wrapping_add(2);
        }
        let info = e.mem.u32(update);
        let point = e.mem.u8(e.mem.u32(e.mem.u32(info.wrapping_add(0xbc))));
        let mut held = e.mem.u8(at);
        e.mem.set_u8(at, point);
        loop {
            at = at.wrapping_add(1);
            let next = e.mem.u8(at);
            e.mem.set_u8(at, held);
            held = next;
            if e.mem.u8(at) == 0 {
                break;
            }
        }
        locale_update_release(e, update);
    })
}

// Translated from 00ed1a09 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__cropzeros_l` (Ghidra's function ID), cdecl: buffer, locale. Takes the locale
/// (`_LocaleUpdate`, `EBP-0x10`). The text is searched for the locale's decimal point; with
/// none nothing changes. After it, up to an `'e'`, `'E'` or the end, the trailing `'0'`s are
/// dropped, and the point too when only it is left before them; the rest (the exponent and
/// the zero) moves down over them.
pub fn cropzeros_l(e: &mut Engine, buffer: u32, locale: u32) {
    e.with_stack(0x10, |e, frame| {
        let update = frame.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let info = e.mem.u32(update);
        let mut at = buffer;
        let mut byte = e.mem.u8(at);
        if byte != 0 {
            // DL: the decimal point, read once.
            let point = e.mem.u8(e.mem.u32(e.mem.u32(info.wrapping_add(0xbc))));
            while byte != point {
                at = at.wrapping_add(1);
                byte = e.mem.u8(at);
                if byte == 0 {
                    break;
                }
            }
        }
        let found = e.mem.u8(at);
        at = at.wrapping_add(1);
        if found != 0 {
            loop {
                let c = e.mem.u8(at);
                if c == 0 || c == b'e' || c == b'E' {
                    break;
                }
                at = at.wrapping_add(1);
            }
            let mut from = at;
            loop {
                at = at.wrapping_sub(1);
                if e.mem.u8(at) != b'0' {
                    break;
                }
            }
            let point = e.mem.u8(e.mem.u32(e.mem.u32(info.wrapping_add(0xbc))));
            if e.mem.u8(at) == point {
                at = at.wrapping_sub(1);
            }
            loop {
                let c = e.mem.u8(from);
                at = at.wrapping_add(1);
                from = from.wrapping_add(1);
                e.mem.set_u8(at, c);
                if c == 0 {
                    break;
                }
            }
        }
        locale_update_release(e, update);
    })
}

// Translated from 00ed1a8b (disassembled, FalloutNV.exe 1.4.0.525)
/// `__positive` (Ghidra's function ID), cdecl: the double's address. `FLDZ; FCOMP` with it and
/// `TEST AH, 0x41; JP`: 1 when the double is at or above zero (C0 or C3 alone), 0 below zero
/// or unordered (a NaN).
pub fn positive(e: &mut Engine, value: u32) -> u32 {
    let x = f64::from_bits(e.mem.u64(value));
    // 0 compared with x: C0 for 0 < x, C3 for equal, both clear for 0 > x, all set unordered.
    let c0 = 0.0 < x || x.is_nan();
    let c3 = 0.0 == x || x.is_nan();
    u32::from(c0 != c3)
}

// Translated from 00ed1ae9 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__fassign` (Ghidra's function ID), cdecl: flag, destination, text; `__fassign_l` with a
/// null locale.
pub fn fassign(e: &mut Engine, flag: u32, destination: u32, text: u32) {
    e.call(FASSIGN_L_ADDR, &args![flag, destination, text, 0u32]);
}

// Translated from 00ed1b22 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__forcdecpt` (Ghidra's function ID), cdecl: `__forcdecpt_l(buffer, NULL)`.
pub fn forcdecpt(e: &mut Engine, buffer: u32) {
    e.call(FORCDECPT_L_ADDR, &args![buffer, 0u32]);
}

// Translated from 00ed1b35 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__cropzeros` (Ghidra's function ID), cdecl: `__cropzeros_l(buffer, NULL)`.
pub fn cropzeros(e: &mut Engine, buffer: u32) {
    e.call(CROPZEROS_L_ADDR, &args![buffer, 0u32]);
}

// ---- The secure output core -----------------------------------------------------------------

/// `_output_s_l`'s state table (`__lookuptable_s`): each character's class in the low nibble
/// of the byte at `010f3190 + character`, each (class, state)'s next state in the high nibble
/// of the byte at `010f31b0 + class * 9 + state` (nine states: state 8 is invalid).
const LOOKUP_S_CLASSES: u32 = 0x010f_3190;
const LOOKUP_S_STATES: u32 = 0x010f_31b0;

/// `_output_s_l`'s frame below its frame pointer (`SUB ESP,0x278`) and the locals it passes by
/// address, by their offset below it.
const OUTPUT_S_FRAME: u32 = 0x278;
const OUTPUT_S_LOCALE: u32 = 0x250;
const OUTPUT_S_COUNT: u32 = 0x228;
const OUTPUT_S_TEXT_LENGTH: u32 = 0x220;
const OUTPUT_S_BUFFER: u32 = 0x20c;
const OUTPUT_S_BUFFER_SIZE: u32 = 0x200;
const OUTPUT_S_DIGITS_END: u32 = 0xd;
const OUTPUT_S_PREFIX: u32 = 0x22c;
const OUTPUT_S_DOUBLE: u32 = 0x278;
const OUTPUT_S_WIDE_SIZE: u32 = 0x270;
const OUTPUT_S_MULTIBYTE: u32 = 0xc;

// Translated from 00ed4685 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed4685 - the /GS stack cookie check (00ec408c) is not translated.
/// `__output_s_l` (Ghidra's function ID), the secure `printf` core, cdecl: stream, format,
/// locale, argument list. Answers the number of characters written, or -1.
///
/// It is `_output_l` (`00ed2f21`, `crt.rs`) with these differences, read from the code: the
/// character classes and states come from its own table (`010f3190`, `010f31b0`; the next
/// state is `class * 9 + state`'s byte shifted right by 4, unsigned); the state 8 (an invalid
/// sequence) refuses the format at once (`errno` `EINVAL`, the invalid-parameter report, the
/// locale released, -1); a normal character (state 0, also the `I` written as one) clears the
/// wide-text flag first; and when the loop ends (the format's end, or the count turning
/// negative) in a state other than 0 or 7 (inside a conversion) the format is refused the same
/// way. Everything else (the stream and format checks, flags, width and precision, the size
/// prefixes, each conversion and the output with its padding) is the same code on its own
/// frame offsets.
pub fn output_s_l(e: &mut Engine, stream: Ptr, format: Ptr, locale: u32, args: u32) -> i32 {
    e.with_stack(OUTPUT_S_FRAME, |e, frame| {
        let at = |offset: u32| frame.addr() + OUTPUT_S_FRAME - offset;
        let stream = stream.addr();
        let stream_ptr = Ptr::new(stream);
        let count = Ptr::new(at(OUTPUT_S_COUNT));
        let update = at(OUTPUT_S_LOCALE);
        let text_length = at(OUTPUT_S_TEXT_LENGTH);
        let mut argument = args;
        let mut hex_add = 0u32;
        let mut flags = 0u32;
        let mut width = 0i32;
        let mut precision = 0i32;
        let mut prefix_length = 0i32;
        let mut no_output = 0u32;
        let mut text_is_wide = 0u32;
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let fail = |e: &mut Engine| {
            let error = errno(e);
            e.mem.set_u32(error.addr(), 0x16);
            fn_00ec7c30(e, 0, 0, 0, 0, 0);
            locale_update_release(e, update);
            -1
        };
        if stream == 0 {
            return fail(e);
        }
        if e.mem.u8(stream + 0xc) & 0x40 == 0 {
            let handle = e.call(FILENO, &args![stream]).u32();
            let entry = |e: &Engine| {
                if handle == 0xffff_ffff || handle == 0xffff_fffe {
                    BAD_IOINFO
                } else {
                    ioinfo(e, handle)
                }
            };
            if e.mem.u8(entry(e) + 0x24) & 0x7f != 0 || e.mem.u8(entry(e) + 0x24) & 0x80 != 0 {
                return fail(e);
            }
        }
        let mut cursor = format.addr();
        if cursor == 0 {
            return fail(e);
        }
        let mut ch = e.mem.u8(cursor);
        e.mem.set_u32(count.addr(), 0);
        e.mem.set_u32(text_length, 0);
        let mut heap_buffer = 0u32;
        let mut state = 0u32;
        let mut cap_exponent = 0u32;
        let mut text = 0u32;
        while ch != 0 {
            cursor = cursor.wrapping_add(1);
            if e.mem.i32(count.addr()) < 0 {
                break;
            }
            let class = if ch.wrapping_sub(0x20) <= 0x58 {
                // MOVZX of the byte at 010f3190 + MOVSX(ch).
                let entry = LOOKUP_S_CLASSES.wrapping_add(i32::from(ch as i8) as u32);
                u32::from(e.mem.u8(entry)) & 0xf
            } else {
                0
            };
            // IMUL by 9 and MOVZX; SHR 4: the next state, 0 to 15.
            let entry = LOOKUP_S_STATES.wrapping_add(class * 9 + state);
            state = u32::from(e.mem.u8(entry)) >> 4;
            if state == 8 {
                // The invalid state: the format is refused.
                return fail(e);
            }
            let mut step = OutputStep::Next;
            match state {
                0 => {
                    text_is_wide = 0;
                    if output_normal_character(e, ch, &mut cursor, stream_ptr, count, update) {
                        return fail(e);
                    }
                }
                1 => {
                    precision = -1;
                    cap_exponent = 0;
                    no_output = 0;
                    width = 0;
                    prefix_length = 0;
                    flags = 0;
                    text_is_wide = 0;
                }
                2 => match ch {
                    0x20 => flags |= FL_SIGNSP,
                    0x23 => flags |= FL_ALTERNATE,
                    0x2b => flags |= FL_SIGN,
                    0x2d => flags |= FL_LEFT,
                    0x30 => flags |= FL_LEADZERO,
                    _ => {}
                },
                3 => {
                    if ch == b'*' {
                        argument = argument.wrapping_add(4);
                        width = e.mem.i32(argument.wrapping_sub(4));
                        if width < 0 {
                            flags |= FL_LEFT;
                            width = width.wrapping_neg();
                        }
                    } else {
                        width = width
                            .wrapping_mul(10)
                            .wrapping_add(i32::from(ch as i8))
                            .wrapping_sub(0x30);
                    }
                }
                4 => precision = 0,
                5 => {
                    if ch == b'*' {
                        argument = argument.wrapping_add(4);
                        precision = e.mem.i32(argument.wrapping_sub(4));
                        if precision < 0 {
                            precision = -1;
                        }
                    } else {
                        precision = precision
                            .wrapping_mul(10)
                            .wrapping_add(i32::from(ch as i8))
                            .wrapping_sub(0x30);
                    }
                }
                6 => match ch {
                    b'I' => {
                        let next = e.mem.u8(cursor);
                        if next == b'6' && e.mem.u8(cursor.wrapping_add(1)) == b'4' {
                            cursor = cursor.wrapping_add(2);
                            flags |= FL_I64;
                        } else if next == b'3' && e.mem.u8(cursor.wrapping_add(1)) == b'2' {
                            cursor = cursor.wrapping_add(2);
                            flags &= !FL_I64;
                        } else if !matches!(next, b'd' | b'i' | b'o' | b'u' | b'x' | b'X') {
                            state = 0;
                            text_is_wide = 0;
                            let stop = output_normal_character(
                                e,
                                ch,
                                &mut cursor,
                                stream_ptr,
                                count,
                                update,
                            );
                            if stop {
                                return fail(e);
                            }
                        }
                    }
                    b'h' => flags |= FL_SHORT,
                    b'l' => {
                        if e.mem.u8(cursor) == b'l' {
                            cursor = cursor.wrapping_add(1);
                            flags |= FL_LONGLONG;
                        } else {
                            flags |= FL_LONG;
                        }
                    }
                    b'w' => flags |= FL_WIDECHAR,
                    _ => {}
                },
                7 => match ch {
                    b'A' | b'E' | b'G' | b'a' | b'e' | b'f' | b'g' => {
                        if matches!(ch, b'A' | b'E' | b'G') {
                            ch = ch.wrapping_add(0x20);
                            cap_exponent = 1;
                        }
                        flags |= FL_SIGNED;
                        let mut buffer = at(OUTPUT_S_BUFFER);
                        text = buffer;
                        let mut buffer_size = OUTPUT_S_BUFFER_SIZE;
                        match precision.cmp(&0) {
                            std::cmp::Ordering::Less => precision = 6,
                            std::cmp::Ordering::Equal => {
                                if ch == b'g' {
                                    precision = 1;
                                }
                            }
                            std::cmp::Ordering::Greater => {
                                if precision > 0x200 {
                                    precision = 0x200;
                                }
                                if precision > 0xa3 {
                                    let size = (precision as u32).wrapping_add(0x15d);
                                    heap_buffer = e.call(MALLOC_CRT, &args![size]).u32();
                                    if heap_buffer != 0 {
                                        text = heap_buffer;
                                        buffer_size = size;
                                        buffer = heap_buffer;
                                    } else {
                                        precision = 0xa3;
                                    }
                                }
                            }
                        }
                        let value = at(OUTPUT_S_DOUBLE);
                        let low = e.mem.u32(argument);
                        e.mem.set_u32(value, low);
                        let high = e.mem.u32(argument.wrapping_add(4));
                        e.mem.set_u32(value + 4, high);
                        argument = argument.wrapping_add(8);
                        let encoded = e.mem.u32(CFLTCVT);
                        let convert = e.call(DECODE_POINTER, &args![encoded]).u32();
                        e.call(
                            convert,
                            &args![
                                value,
                                buffer,
                                buffer_size,
                                i32::from(ch as i8) as u32,
                                precision,
                                cap_exponent,
                                update
                            ],
                        );
                        let alternate = flags & FL_ALTERNATE;
                        if alternate != 0 && precision == 0 {
                            let encoded = e.mem.u32(FORCDECPT);
                            let force = e.call(DECODE_POINTER, &args![encoded]).u32();
                            e.call(force, &args![buffer, update]);
                        }
                        if ch == b'g' && alternate == 0 {
                            let encoded = e.mem.u32(CROPZEROS);
                            let crop = e.call(DECODE_POINTER, &args![encoded]).u32();
                            e.call(crop, &args![buffer, update]);
                        }
                        if e.mem.u8(buffer) == b'-' {
                            flags |= FL_NEGATIVE;
                            buffer = buffer.wrapping_add(1);
                            text = buffer;
                        }
                        let length = e.call(STRLEN_ADDR, &args![buffer]).u32();
                        e.mem.set_u32(text_length, length);
                        step = OutputStep::Output;
                    }
                    b'C' | b'c' => {
                        if ch == b'C' && flags & 0x830 == 0 {
                            flags |= FL_WIDECHAR;
                        }
                        argument = argument.wrapping_add(4);
                        if flags & 0x810 != 0 {
                            let character = u32::from(e.mem.u16(argument.wrapping_sub(4)));
                            let failed = e
                                .call(
                                    WCTOMB_S,
                                    &args![
                                        text_length,
                                        at(OUTPUT_S_BUFFER),
                                        OUTPUT_S_BUFFER_SIZE,
                                        character
                                    ],
                                )
                                .u32();
                            if failed != 0 {
                                no_output = 1;
                            }
                        } else {
                            let byte = e.mem.u8(argument.wrapping_sub(4));
                            e.mem.set_u8(at(OUTPUT_S_BUFFER), byte);
                            e.mem.set_u32(text_length, 1);
                        }
                        text = at(OUTPUT_S_BUFFER);
                        step = OutputStep::Output;
                    }
                    b'S' | b's' => {
                        if ch == b'S' && flags & 0x830 == 0 {
                            flags |= FL_WIDECHAR;
                        }
                        let mut limit = precision as u32;
                        if precision == -1 {
                            limit = 0x7fff_ffff;
                        }
                        argument = argument.wrapping_add(4);
                        let pointer = e.mem.u32(argument.wrapping_sub(4));
                        text = pointer;
                        if flags & 0x810 == 0 {
                            if pointer == 0 {
                                text = e.mem.u32(NULL_STRING);
                            }
                            let mut walk = text;
                            while limit != 0 {
                                limit -= 1;
                                if e.mem.u8(walk) == 0 {
                                    break;
                                }
                                walk = walk.wrapping_add(1);
                            }
                            e.mem.set_u32(text_length, walk.wrapping_sub(text));
                        } else {
                            if pointer == 0 {
                                text = e.mem.u32(NULL_WIDE_STRING);
                            }
                            text_is_wide = 1;
                            let mut walk = text;
                            while limit != 0 {
                                limit -= 1;
                                if e.mem.u16(walk) == 0 {
                                    break;
                                }
                                walk = walk.wrapping_add(2);
                            }
                            let length = (walk.wrapping_sub(text) as i32) >> 1;
                            e.mem.set_i32(text_length, length);
                        }
                        step = OutputStep::Output;
                    }
                    b'Z' => {
                        let counted = e.mem.u32(argument);
                        argument = argument.wrapping_add(4);
                        let buffer = if counted == 0 {
                            0
                        } else {
                            e.mem.u32(counted.wrapping_add(4))
                        };
                        if buffer == 0 {
                            text = e.mem.u32(NULL_STRING);
                            let length = e.call(STRLEN_ADDR, &args![text]).u32();
                            e.mem.set_u32(text_length, length);
                        } else {
                            let length = i32::from(e.mem.i16(counted));
                            text = buffer;
                            if flags & FL_WIDECHAR != 0 {
                                e.mem.set_i32(text_length, length / 2);
                                text_is_wide = 1;
                            } else {
                                text_is_wide = 0;
                                e.mem.set_i32(text_length, length);
                            }
                        }
                        step = OutputStep::Output;
                    }
                    b'n' => {
                        let target = e.mem.u32(argument);
                        argument = argument.wrapping_add(4);
                        if e.call(GET_PRINTF_COUNT_OUTPUT, &args![]).u32() == 0 {
                            return fail(e);
                        }
                        let written = e.mem.u32(count.addr());
                        if flags & FL_SHORT != 0 {
                            e.mem.set_u16(target, written as u16);
                        } else {
                            e.mem.set_u32(target, written);
                        }
                        no_output = 1;
                        step = OutputStep::Release;
                    }
                    b'd' | b'i' | b'u' => {
                        if ch != b'u' {
                            flags |= FL_SIGNED;
                        }
                        e.mem.set_u32(text_length, 10);
                        step = OutputStep::Integer;
                    }
                    b'o' => {
                        e.mem.set_u32(text_length, 8);
                        if flags & FL_ALTERNATE != 0 {
                            flags |= FL_FORCEOCTAL;
                        }
                        step = OutputStep::Integer;
                    }
                    b'p' | b'X' | b'x' => {
                        if ch == b'p' {
                            precision = 8;
                        }
                        hex_add = if ch == b'x' { 0x27 } else { 7 };
                        e.mem.set_u32(text_length, 0x10);
                        if flags & FL_ALTERNATE != 0 {
                            e.mem.set_u8(at(OUTPUT_S_PREFIX), b'0');
                            let letter = (hex_add as u8).wrapping_add(0x51);
                            e.mem.set_u8(at(OUTPUT_S_PREFIX) + 1, letter);
                            prefix_length = 2;
                        }
                        step = OutputStep::Integer;
                    }
                    _ => step = OutputStep::Output,
                },
                _ => {}
            }

            if step == OutputStep::Integer {
                let mut value: u64 = if flags & (FL_I64 | FL_LONGLONG) != 0 {
                    let low = e.mem.u32(argument);
                    let high = e.mem.u32(argument.wrapping_add(4));
                    argument = argument.wrapping_add(8);
                    u64::from(low) | u64::from(high) << 32
                } else {
                    argument = argument.wrapping_add(4);
                    if flags & FL_SHORT != 0 {
                        let word = e.mem.u16(argument.wrapping_sub(4));
                        if flags & FL_SIGNED != 0 {
                            i64::from(word as i16) as u64
                        } else {
                            u64::from(word)
                        }
                    } else {
                        let word = e.mem.u32(argument.wrapping_sub(4));
                        if flags & FL_SIGNED != 0 {
                            i64::from(word as i32) as u64
                        } else {
                            u64::from(word)
                        }
                    }
                };
                if flags & FL_SIGNED != 0 && (value as i64) < 0 {
                    value = value.wrapping_neg();
                    flags |= FL_NEGATIVE;
                }
                if flags & (FL_I64 | FL_LONGLONG) == 0 {
                    value &= 0xffff_ffff;
                }
                if precision < 0 {
                    precision = 1;
                } else {
                    flags &= !FL_LEADZERO;
                    if precision > 0x200 {
                        precision = 0x200;
                    }
                }
                if value == 0 {
                    prefix_length = 0;
                }
                let radix = i64::from(e.mem.i32(text_length)) as u64;
                let end = at(OUTPUT_S_DIGITS_END);
                let mut digit_at = end;
                loop {
                    let before = precision;
                    precision = precision.wrapping_sub(1);
                    if before <= 0 && value == 0 {
                        break;
                    }
                    let (quotient, remainder) = aulldvrm(e, value, radix);
                    let mut digit = (remainder as u32).wrapping_add(0x30);
                    value = quotient;
                    if (digit as i32) > 0x39 {
                        digit = digit.wrapping_add(hex_add);
                    }
                    e.mem.set_u8(digit_at, digit as u8);
                    digit_at = digit_at.wrapping_sub(1);
                }
                let mut length = end.wrapping_sub(digit_at);
                text = digit_at.wrapping_add(1);
                e.mem.set_u32(text_length, length);
                if flags & FL_FORCEOCTAL != 0 && (length == 0 || e.mem.u8(text) != b'0') {
                    text = text.wrapping_sub(1);
                    e.mem.set_u8(text, b'0');
                    length = length.wrapping_add(1);
                    e.mem.set_u32(text_length, length);
                }
                step = OutputStep::Output;
            }

            if step == OutputStep::Output {
                if no_output == 0 {
                    if flags & FL_SIGNED != 0 {
                        let sign = if flags & FL_NEGATIVE != 0 {
                            Some(b'-')
                        } else if flags & FL_SIGN != 0 {
                            Some(b'+')
                        } else if flags & FL_SIGNSP != 0 {
                            Some(b' ')
                        } else {
                            None
                        };
                        if let Some(sign) = sign {
                            e.mem.set_u8(at(OUTPUT_S_PREFIX), sign);
                            prefix_length = 1;
                        }
                    }
                    let length = e.mem.i32(text_length);
                    let padding = width.wrapping_sub(length).wrapping_sub(prefix_length);
                    if flags & (FL_LEFT | FL_LEADZERO) == 0 {
                        write_multi_char(e, 0x20, padding, stream_ptr, count);
                    }
                    let prefix = Ptr::new(at(OUTPUT_S_PREFIX));
                    write_string(e, count, prefix, stream_ptr, prefix_length);
                    if flags & FL_LEADZERO != 0 && flags & FL_LEFT == 0 {
                        write_multi_char(e, 0x30, padding, stream_ptr, count);
                    }
                    let length = e.mem.i32(text_length);
                    if text_is_wide != 0 && length > 0 {
                        let mut walk = text;
                        let mut remaining = length;
                        loop {
                            let character = u32::from(e.mem.u16(walk));
                            remaining -= 1;
                            walk = walk.wrapping_add(2);
                            let size_out = at(OUTPUT_S_WIDE_SIZE);
                            let multibyte = at(OUTPUT_S_MULTIBYTE);
                            let failed = e
                                .call(WCTOMB_S, &args![size_out, multibyte, 6u32, character])
                                .u32();
                            let size = e.mem.i32(size_out);
                            if failed != 0 || size == 0 {
                                e.mem.set_u32(count.addr(), 0xffff_ffff);
                                break;
                            }
                            write_string(e, count, Ptr::new(multibyte), stream_ptr, size);
                            if remaining == 0 {
                                break;
                            }
                        }
                    } else {
                        write_string(e, count, Ptr::new(text), stream_ptr, length);
                    }
                    if e.mem.i32(count.addr()) >= 0 && flags & FL_LEFT != 0 {
                        write_multi_char(e, 0x20, padding, stream_ptr, count);
                    }
                }
                step = OutputStep::Release;
            }

            if step == OutputStep::Release && heap_buffer != 0 {
                e.call(FREE_CRT, &args![heap_buffer]);
                heap_buffer = 0;
            }
            ch = e.mem.u8(cursor);
        }
        // 00ed51e3: the format must end in the normal state or after a conversion.
        if state != 0 && state != 7 {
            return fail(e);
        }
        locale_update_release(e, update);
        e.mem.i32(count.addr())
    })
}

/// The registry entry of [`output_s_l`]: stream, format, locale, argument list.
fn output_s_l_entry(e: &mut Engine, words: &[u32]) -> Ret {
    output_s_l(
        e,
        Ptr::new(words[0]),
        Ptr::new(words[1]),
        words[2],
        words[3],
    )
    .into_ret()
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00ec_623a, sprintf_entry as AbiFn),
        entry!(0x00ec_6c46, vsprintf_l(u32, u32, u32, u32) -> i32),
        entry!(0x00ec_6cc4, vsprintf(u32, u32, u32) -> i32),
        entry!(
            0x00ec_6a61,
            vsnprintf_helper(u32, u32, u32, u32, u32, u32) -> i32
        ),
        entry!(0x00ec_6b2d, vsprintf_s_l(u32, u32, u32, u32, u32) -> i32),
        entry!(0x00ec_6bb5, vsprintf_s(u32, u32, u32, u32) -> i32),
        (0x00ec_a38d, snprintf_entry as AbiFn),
        entry!(0x00ec_dd3e, vsnprintf_l(u32, u32, u32, u32, u32) -> i32),
        (OUTPUT_S_L_ADDR, output_s_l_entry as AbiFn),
        entry!(
            0x00ed_23e3,
            cfltcvt_l(u32, u32, u32, i32, u32, u32, u32) -> i32
        ),
        entry!(0x00ed_246b, cfltcvt(u32, u32, u32, i32, u32, u32) -> i32),
        entry!(0x00ed_22e9, cftog_l(u32, u32, u32, i32, u32, u32) -> i32),
        entry!(0x00ed_1996, forcdecpt_l(u32, u32)),
        entry!(0x00ed_1a09, cropzeros_l(u32, u32)),
        entry!(0x00ed_1a8b, positive(u32) -> u32),
        entry!(0x00ed_1ae9, fassign(u32, u32, u32)),
        entry!(0x00ed_1b22, forcdecpt(u32)),
        entry!(0x00ed_1b35, cropzeros(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;
    type Row = [u8; 9];

    /// The handler the invalid-parameter report calls (doubled), and the float helpers'
    /// doubles behind the encoded table.
    const HANDLER: u32 = 0x0e30_0000;
    const CONVERT: u32 = 0x0e31_0000;
    const FORCE_POINT: u32 = 0x0e32_0000;
    const CROP: u32 = 0x0e33_0000;

    fn calls(e: &Engine, addr: u32) -> Calls {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn text(e: &mut Engine, t: &[u8]) -> u32 {
        let at = e.mem.alloc(t.len() as u32 + 1);
        e.mem.set_cstr(at, t);
        at
    }

    /// An engine with `errno` in a doubled thread block, the invalid-parameter report going to
    /// a doubled handler, and the call log on. Returns the engine and the `errno` slot.
    fn base_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        e.map(0x011c_1000, 0x2000);
        e.map(SECURITY_COOKIE, 4);
        let block = e.mem.alloc(0x80);
        e.register_double(GETPTD_NOEXIT, move |_, _| block.into_ret());
        e.register(DECODE_POINTER, |_, a| a[0].into_ret());
        e.mem.set_u32(INVALID_PARAMETER_HANDLER, HANDLER);
        e.register(HANDLER, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, block + 8)
    }

    /// `_output_l` and `_output_s_l` doubled to write their format's bytes literally through
    /// the stream (with the real `write_char`, so the room and `_flsbuf` are the exe's) and
    /// answer the count.
    fn literal_output(e: &mut Engine) {
        for addr in [OUTPUT_L_ADDR, OUTPUT_S_L_ADDR] {
            e.register(addr, |e, a| {
                let bytes = e.mem.cstr(a[1]);
                let count = e.mem.alloc(4);
                e.mem.set_u32(count, 0);
                for b in bytes {
                    write_char(e, u32::from(b), Ptr::new(a[0]), Ptr::new(count));
                    if e.mem.i32(count) < 0 {
                        break;
                    }
                }
                e.mem.u32(count).into_ret()
            });
        }
    }

    #[test]
    fn sprintf_writes_through_a_string_stream_and_terminates() {
        let (mut e, _) = base_engine();
        literal_output(&mut e);
        let buffer = e.mem.alloc(16);
        e.mem.write(buffer, &[0xaa; 16]);
        let f = text(&mut e, b"Form");
        assert_eq!(sprintf(&mut e, buffer, f, &[7, 8]), 4);
        assert_eq!(e.mem.bytes(buffer, 6), b"Form\0\xaa".to_vec());
        // The list is the variable words, in order; the locale null.
        let c = calls(&e, OUTPUT_L_ADDR);
        assert_eq!((c[0][1], c[0][2]), (f, 0));
        assert_eq!((e.mem.u32(c[0][3]), e.mem.u32(c[0][3] + 4)), (7, 8));
        // Through the entry, and `vsprintf` through `_vsprintf_l`.
        assert_eq!(e.call(0x00ec_623a, &args![buffer, f]).i32(), 4);
        let list = e.mem.alloc(4);
        assert_eq!(vsprintf(&mut e, buffer, f, list), 4);
        assert_eq!(calls(&e, VSPRINTF_L_ADDR), vec![vec![buffer, f, 0, list]]);
        assert_eq!(e.mem.cstr(buffer), b"Form".to_vec());
    }

    #[test]
    fn string_functions_refuse_null_arguments() {
        let (mut e, slot) = base_engine();
        literal_output(&mut e);
        let f = text(&mut e, b"x");
        let buffer = e.mem.alloc(8);
        let results = [
            sprintf(&mut e, 0, f, &[]),
            sprintf(&mut e, buffer, 0, &[]),
            vsprintf_l(&mut e, 0, f, 0, 0),
            snprintf(&mut e, 0, 4, f, &[]),
            snprintf(&mut e, buffer, 4, 0, &[]),
            vsnprintf_l(&mut e, 0, 4, f, 0, 0),
            vsnprintf_helper(&mut e, OUTPUT_S_L_ADDR, 0, 4, f, 0, 0),
            vsprintf_s_l(&mut e, buffer, 0, f, 0, 0),
            vsprintf_s_l(&mut e, 0, 4, f, 0, 0),
            vsprintf_s_l(&mut e, buffer, 4, 0, 0, 0),
        ];
        assert_eq!(results, [-1; 10]);
        assert_eq!(calls(&e, HANDLER).len(), 10);
        assert!(calls(&e, OUTPUT_L_ADDR).is_empty());
        assert_eq!(e.mem.u32(slot), 0x16);
    }

    #[test]
    fn snprintf_fits_fills_and_overflows() {
        let (mut e, _) = base_engine();
        literal_output(&mut e);
        let buffer = e.mem.alloc(16);
        let f = text(&mut e, b"abcd");
        let fill = |e: &mut Engine| e.mem.write(buffer, &[0xaa; 16]);
        // Room to spare: the text and its zero.
        fill(&mut e);
        assert_eq!(snprintf(&mut e, buffer, 8, f, &[]), 4);
        assert_eq!(e.mem.bytes(buffer, 6), b"abcd\0\xaa".to_vec());
        // Exactly full: `_flsbuf` takes the zero (nothing written).
        fill(&mut e);
        assert_eq!(snprintf(&mut e, buffer, 4, f, &[]), 4);
        assert_eq!(e.mem.bytes(buffer, 5), b"abcd\xaa".to_vec());
        // Too long: `_output_l` gets -1 from the full stream.
        fill(&mut e);
        assert_eq!(snprintf(&mut e, buffer, 2, f, &[]), -1);
        assert_eq!(e.mem.bytes(buffer, 3), b"ab\xaa".to_vec());
        // A null buffer with no room only counts (no terminator).
        assert_eq!(snprintf(&mut e, 0, 0, f, &[]), 4);
        assert_eq!(e.call(0x00ec_a38d, &args![buffer, 8u32, f]).i32(), 4);
        // `_vsnprintf_l` passes its locale and list.
        e.call_log = Some(vec![]);
        assert_eq!(vsnprintf_l(&mut e, buffer, 8, f, 0x1234, 0x5678), 4);
        let c = calls(&e, OUTPUT_L_ADDR);
        assert_eq!((c[0][1], c[0][2], c[0][3]), (f, 0x1234, 0x5678));
        assert_eq!(e.mem.cstr(buffer), b"abcd".to_vec());
    }

    #[test]
    fn vsnprintf_helper_answers_minus_two_when_the_text_does_not_fit() {
        let (mut e, _) = base_engine();
        literal_output(&mut e);
        let buffer = e.mem.alloc(16);
        let f = text(&mut e, b"abcd");
        let helper = |e: &mut Engine, count: u32| {
            e.mem.write(buffer, &[0xaa; 16]);
            vsnprintf_helper(e, OUTPUT_S_L_ADDR, buffer, count, f, 0, 0)
        };
        assert_eq!(helper(&mut e, 8), 4);
        assert_eq!(e.mem.bytes(buffer, 6), b"abcd\0\xaa".to_vec());
        // Exactly full: the zero does not fit, the last byte is cleared, -2.
        assert_eq!(helper(&mut e, 4), -2);
        assert_eq!(e.mem.bytes(buffer, 5), b"abc\0\xaa".to_vec());
        // Too long: `_output_s_l` answers -1, the last byte is cleared, -2.
        assert_eq!(helper(&mut e, 2), -2);
        assert_eq!(e.mem.bytes(buffer, 3), b"a\0\xaa".to_vec());
        // No buffer and no room: the count.
        assert_eq!(vsnprintf_helper(&mut e, OUTPUT_S_L_ADDR, 0, 0, f, 0, 0), 4);
        // A negative answer with room left: -1 and the last byte cleared.
        e.register(OUTPUT_S_L_ADDR, |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(helper(&mut e, 4), -1);
        assert_eq!(e.mem.bytes(buffer, 5), b"\xaa\xaa\xaa\0\xaa".to_vec());
    }

    #[test]
    fn vsprintf_s_reports_erange_and_empties_the_buffer() {
        let (mut e, slot) = base_engine();
        let buffer = e.mem.alloc(8);
        let f = text(&mut e, b"x");
        e.mem.write(buffer, b"zzzz");
        e.register(VSNPRINTF_HELPER_ADDR, |_, _| 3u32.into_ret());
        assert_eq!(vsprintf_s(&mut e, buffer, 8, f, 0x40), 3);
        assert_eq!(
            calls(&e, VSNPRINTF_HELPER_ADDR),
            vec![vec![OUTPUT_S_L_ADDR, buffer, 8, f, 0, 0x40]]
        );
        assert_eq!(e.mem.u8(buffer), b'z');
        e.register(VSNPRINTF_HELPER_ADDR, |_, _| 0xffff_fffeu32.into_ret());
        assert_eq!(vsprintf_s(&mut e, buffer, 8, f, 0x40), -1);
        assert_eq!(e.mem.u8(buffer), 0);
        assert_eq!(e.mem.u32(slot), 0x22);
        assert_eq!(calls(&e, HANDLER).len(), 1);
        e.mem.set_u8(buffer, b'z');
        e.register(VSNPRINTF_HELPER_ADDR, |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(vsprintf_s(&mut e, buffer, 8, f, 0x40), -1);
        assert_eq!(e.mem.u8(buffer), 0);
        assert_eq!(calls(&e, HANDLER).len(), 1, "-1 is not reported again");
    }

    #[test]
    fn cfltcvt_picks_the_conversion_by_the_format_character() {
        let (mut e, _) = base_engine();
        for addr in [CFTOE_L_ADDR, CFTOF_L_ADDR, CFTOA_L_ADDR, CFTOG_L_ADDR] {
            e.register_double(addr, move |_, _| addr.into_ret());
        }
        let pick = |e: &mut Engine, c: u8| cfltcvt_l(e, 1, 2, 3, i32::from(c), 4, 5, 6) as u32;
        assert_eq!(pick(&mut e, b'e'), CFTOE_L_ADDR);
        assert_eq!(pick(&mut e, b'E'), CFTOE_L_ADDR);
        assert_eq!(pick(&mut e, b'f'), CFTOF_L_ADDR);
        assert_eq!(pick(&mut e, b'a'), CFTOA_L_ADDR);
        assert_eq!(pick(&mut e, b'A'), CFTOA_L_ADDR);
        assert_eq!(pick(&mut e, b'g'), CFTOG_L_ADDR);
        assert_eq!(pick(&mut e, b'G'), CFTOG_L_ADDR);
        assert_eq!(calls(&e, CFTOF_L_ADDR), vec![vec![1, 2, 3, 4, 6]]);
        assert_eq!(calls(&e, CFTOE_L_ADDR)[0], vec![1, 2, 3, 4, 5, 6]);
        e.register(CFLTCVT_L_ADDR, |_, _| 9u32.into_ret());
        assert_eq!(cfltcvt(&mut e, 1, 2, 3, 0x67, 4, 5), 9);
        assert_eq!(
            calls(&e, CFLTCVT_L_ADDR),
            vec![vec![1, 2, 3, 0x67, 4, 5, 0]]
        );
    }

    /// `__cftog_l` with its helpers doubled: `__fltout2` fills the `_strflt` with the sign and
    /// `decpt`, `__fptostr` writes `digits` at the place it is given and sets `decpt` to
    /// `rounded`, and the layouts record their words. Answers which layout ran and the buffer.
    fn cftog(
        negative: bool,
        decpt: i32,
        digits: &'static [u8],
        rounded: i32,
        precision: i32,
    ) -> (u32, Vec<u8>, Calls) {
        let (mut e, _) = base_engine();
        e.register_double(FLTOUT2_ADDR, move |e, a| {
            e.mem.set_u32(a[2], if negative { 0x2d } else { 0x20 });
            e.mem.set_u32(a[2] + 4, decpt as u32);
            Ret::default()
        });
        e.register_double(FPTOSTR_ADDR, move |e, a| {
            e.mem.set_cstr(a[0], digits);
            e.mem.set_u32(a[3] + 4, rounded as u32);
            0u32.into_ret()
        });
        e.register(CFTOF2_L_ADDR, |_, _| 0xfu32.into_ret());
        e.register(CFTOE2_L_ADDR, |_, _| 0xeu32.into_ret());
        let value = e.mem.alloc(8);
        let buffer = e.mem.alloc(32);
        e.mem.write(buffer, &[b'#'; 32]);
        let which = cftog_l(&mut e, value, buffer, 32, precision, 1, 0x77) as u32;
        let mut log = calls(&e, CFTOF2_L_ADDR);
        log.extend(calls(&e, CFTOE2_L_ADDR));
        log.extend(calls(&e, FPTOSTR_ADDR));
        (which, e.mem.cstr(buffer), log)
    }

    #[test]
    fn cftog_takes_f_or_e_by_the_rounded_magnitude() {
        // 1.5 with 6 digits: %f, the digits after the sign's place.
        let (which, text, log) = cftog(false, 1, b"150000", 1, 6);
        assert_eq!(which, 0xf);
        assert_eq!(text, b"150000".to_vec());
        assert_eq!(
            (log[0][2], log[0][3], log[0][4], log[0][5]),
            (32, 6, 1, 0x77)
        );
        // A negative number: the digits one place on, the room one less.
        let (_, text, log) = cftog(true, 1, b"15", 1, 2);
        assert_eq!(text, b"#15".to_vec());
        assert_eq!((log[1][1], log[1][2]), (31, 2));
        // Magnitude 6 with precision 6 (1e6): %e with the flag.
        let (which, _, log) = cftog(false, 7, b"100000", 7, 6);
        assert_eq!(which, 0xe);
        assert_eq!(
            (log[0][1], log[0][2], log[0][3], log[0][5], log[0][6]),
            (32, 6, 1, 1, 0x77)
        );
        // Magnitude -5: %e; -4: %f.
        assert_eq!(cftog(false, -4, b"1", -4, 6).0, 0xe);
        assert_eq!(cftog(false, -3, b"1", -3, 6).0, 0xf);
        // Rounding raised the magnitude (9.99 to 10.0 with 3 digits): the last digit dropped.
        let (which, text, _) = cftog(false, 1, b"100", 2, 3);
        assert_eq!(which, 0xf);
        assert_eq!(text, b"10".to_vec());
    }

    #[test]
    fn cftog_refuses_a_null_buffer_and_passes_fptostr_errors() {
        let (mut e, slot) = base_engine();
        e.register(FLTOUT2_ADDR, |_, _| Ret::default());
        let value = e.mem.alloc(8);
        assert_eq!(cftog_l(&mut e, value, 0, 8, 6, 0, 0), 0x16);
        let buffer = e.mem.alloc(8);
        assert_eq!(cftog_l(&mut e, value, buffer, 0, 6, 0, 0), 0x16);
        assert_eq!(e.mem.u32(slot), 0x16);
        e.register(FPTOSTR_ADDR, |_, _| 0x22u32.into_ret());
        e.mem.set_u8(buffer, b'x');
        assert_eq!(cftog_l(&mut e, value, buffer, 8, 6, 0, 0), 0x22);
        assert_eq!(e.mem.u8(buffer), 0);
        // A size of -1 is passed on whole.
        e.call_log = Some(vec![]);
        cftog_l(&mut e, value, buffer, 0xffff_ffff, 6, 0, 0);
        assert_eq!(calls(&e, FPTOSTR_ADDR)[0][1], 0xffff_ffff);
    }

    /// Doubles for `_LocaleUpdate` (a locale whose decimal point is `point`), `tolower` and
    /// `isdigit` (ASCII).
    fn locale_engine(point: u8) -> Engine {
        let (mut e, _) = base_engine();
        let info = e.mem.alloc(0x100);
        let lconv = e.mem.alloc(8);
        let text_point = e.mem.alloc(2);
        e.mem.set_u8(text_point, point);
        e.mem.set_u32(lconv, text_point);
        e.mem.set_u32(info + 0xbc, lconv);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0], info);
            e.mem.set_u8(a[0] + 0xc, 0);
            a[0].into_ret()
        });
        e.register(TOLOWER_ADDR, |_, a| {
            (a[0] as u8).to_ascii_lowercase().into_ret()
        });
        e.register(ISDIGIT_ADDR, |_, a| {
            u32::from((a[0] as u8).is_ascii_digit()).into_ret()
        });
        e
    }

    fn rewrite(point: u8, input: &[u8], f: fn(&mut Engine, u32, u32)) -> Vec<u8> {
        let mut e = locale_engine(point);
        let buffer = e.mem.alloc(32);
        e.mem.set_cstr(buffer, input);
        f(&mut e, buffer, 0);
        e.mem.cstr(buffer)
    }

    #[test]
    fn forcdecpt_puts_the_point_after_the_digits() {
        assert_eq!(rewrite(b'.', b"12", forcdecpt_l), b"12.".to_vec());
        assert_eq!(rewrite(b'.', b"1e+005", forcdecpt_l), b"1.e+005".to_vec());
        assert_eq!(rewrite(b',', b"7E-001", forcdecpt_l), b"7,E-001".to_vec());
        // An exponent first: the point goes before it.
        assert_eq!(rewrite(b'.', b"e5", forcdecpt_l), b".e5".to_vec());
        // A hexadecimal "0x1p" form skips the "x" and the next character.
        assert_eq!(rewrite(b'.', b"0x1p+0", forcdecpt_l), b"0x1.p+0".to_vec());
    }

    #[test]
    fn cropzeros_drops_trailing_zeros_and_a_lone_point() {
        assert_eq!(rewrite(b'.', b"1.500000", cropzeros_l), b"1.5".to_vec());
        assert_eq!(rewrite(b'.', b"2.000000", cropzeros_l), b"2".to_vec());
        assert_eq!(
            rewrite(b'.', b"1.500000e+006", cropzeros_l),
            b"1.5e+006".to_vec()
        );
        assert_eq!(
            rewrite(b'.', b"1.000000E+006", cropzeros_l),
            b"1E+006".to_vec()
        );
        assert_eq!(rewrite(b'.', b"100", cropzeros_l), b"100".to_vec());
        assert_eq!(rewrite(b',', b"3,250", cropzeros_l), b"3,25".to_vec());
        // The locale's point only: a '.' is not one.
        assert_eq!(rewrite(b',', b"3.250", cropzeros_l), b"3.250".to_vec());
        assert_eq!(rewrite(b'.', b"", cropzeros_l), b"".to_vec());
    }

    #[test]
    fn positive_is_at_or_above_zero_and_not_a_nan() {
        let (mut e, _) = base_engine();
        let at = e.mem.alloc(8);
        let cases: [(u64, u32); 8] = [
            (1.5f64.to_bits(), 1),
            (0, 1),
            ((-0.0f64).to_bits(), 1),
            ((-2.0f64).to_bits(), 0),
            (f64::INFINITY.to_bits(), 1),
            (f64::NEG_INFINITY.to_bits(), 0),
            (0x7ff8_0000_0000_0000, 0),
            (0x7ff0_0000_0000_0001, 0),
        ];
        for (bits, want) in cases {
            e.mem.set_u64(at, bits);
            assert_eq!(positive(&mut e, at), want, "{bits:016x}");
        }
    }

    #[test]
    fn locale_free_wrappers_pass_a_null_locale() {
        let (mut e, _) = base_engine();
        for addr in [FASSIGN_L_ADDR, FORCDECPT_L_ADDR, CROPZEROS_L_ADDR] {
            e.register(addr, |_, _| Ret::default());
        }
        fassign(&mut e, 1, 2, 3);
        forcdecpt(&mut e, 4);
        cropzeros(&mut e, 5);
        assert_eq!(calls(&e, FASSIGN_L_ADDR), vec![vec![1, 2, 3, 0]]);
        assert_eq!(calls(&e, FORCDECPT_L_ADDR), vec![vec![4, 0]]);
        assert_eq!(calls(&e, CROPZEROS_L_ADDR), vec![vec![5, 0]]);
    }

    /// `__lookuptable_s` as the secure `printf` defines it: each character's class in the low
    /// nibble at `010f3190 + character`, each (class, state)'s next state in the high nibble
    /// at `010f31b0 + class * 9 + state`; state 8 is the invalid one.
    fn lookup_table_s(e: &mut Engine) {
        e.map(LOOKUP_S_CLASSES, 0x100);
        let classes: &[(&[u8], u8)] = &[
            (b" #+-", 6),
            (b"%", 1),
            (b"*", 3),
            (b".", 2),
            (b"0", 4),
            (b"123456789", 5),
            (b"FILNhlw", 7),
            (b"ACEGSXZacdefginopsux", 8),
        ];
        for (characters, class) in classes {
            for c in characters.iter() {
                let at = LOOKUP_S_CLASSES + u32::from(*c);
                let byte = e.mem.u8(at) | class;
                e.mem.set_u8(at, byte);
            }
        }
        // Rows: classes 0 (other) to 8 (type); columns: states 0 (normal) to 8 (invalid).
        let next: [Row; 9] = [
            [0, 8, 8, 8, 8, 8, 8, 0, 0],
            [1, 0, 8, 8, 8, 8, 8, 1, 0],
            [0, 4, 4, 4, 8, 8, 8, 0, 0],
            [0, 3, 3, 8, 5, 8, 8, 0, 0],
            [0, 2, 2, 3, 5, 5, 8, 0, 0],
            [0, 3, 3, 3, 5, 5, 8, 0, 0],
            [0, 2, 2, 8, 8, 8, 8, 0, 0],
            [0, 6, 6, 6, 6, 6, 6, 0, 0],
            [0, 7, 7, 7, 7, 7, 7, 0, 0],
        ];
        for (class, row) in next.iter().enumerate() {
            for (state, value) in row.iter().enumerate() {
                let at = LOOKUP_S_STATES + (class * 9 + state) as u32;
                let byte = e.mem.u8(at) | (value << 4);
                e.mem.set_u8(at, byte);
            }
        }
    }

    /// An engine for `_output_s_l`: the base engine, the secure table, the null strings, and
    /// doubles for the locale, lead bytes, `wctomb_s`, the count switch, the heap and the
    /// float helpers (`_cfltcvt` writes "-1.50" for 'f' and "2.5e+000" otherwise).
    fn output_engine() -> (Engine, u32) {
        let (mut e, slot) = base_engine();
        e.register(LOCALE_UPDATE, |e, a| {
            e.mem.set_u8(a[0] + 0xc, 0);
            a[0].into_ret()
        });
        e.register(ISLEADBYTE_L, |_, _| 0u32.into_ret());
        e.register(WCTOMB_S, |e, a| {
            e.mem.set_u8(a[1], a[3] as u8);
            e.mem.set_u32(a[0], 1);
            0u32.into_ret()
        });
        e.register(GET_PRINTF_COUNT_OUTPUT, |_, _| 1u32.into_ret());
        e.register(MALLOC_CRT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FREE_CRT, |_, _| Ret::default());
        e.mem.set_u32(CFLTCVT, CONVERT);
        e.mem.set_u32(FORCDECPT, FORCE_POINT);
        e.mem.set_u32(CROPZEROS, CROP);
        e.register(CONVERT, |e, a| {
            let text: &[u8] = if a[3] == u32::from(b'f') {
                b"-1.50"
            } else {
                b"2.5e+000"
            };
            e.mem.set_cstr(a[1], text);
            Ret::default()
        });
        e.register(FORCE_POINT, |_, _| Ret::default());
        e.register(CROP, |_, _| Ret::default());
        let null = e.mem.alloc(8);
        e.mem.set_cstr(null, b"(null)");
        e.mem.set_u32(NULL_STRING, null);
        lookup_table_s(&mut e);
        (e, slot)
    }

    /// Runs `_output_s_l` on `format` with `words` as the argument list into a string stream;
    /// answers the result and the text written.
    fn run(e: &mut Engine, format: &[u8], words: &[u32]) -> (i32, Vec<u8>) {
        let buffer = e.mem.alloc(0x100);
        let stream = e.mem.alloc(0x20);
        string_stream(e, stream, buffer, 0x100);
        let f = text(e, format);
        let list = e.mem.alloc(4 * words.len() as u32 + 4);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *word);
        }
        let result = e.call(OUTPUT_S_L_ADDR, &args![stream, f, 0u32, list]).i32();
        let written = e.mem.u32(stream).wrapping_sub(buffer);
        (result, e.mem.bytes(buffer, written))
    }

    #[test]
    fn output_s_formats_as_output_l_does() {
        let (mut e, _) = output_engine();
        let hi = text(&mut e, b"hi");
        let (result, written) = run(
            &mut e,
            b"a%5d|%-4s|%#x|%c%%|%X|%s|%.2f|%g",
            &[42, hi, 255, u32::from(b'z'), 0xbeef, 0, 0, 0, 0, 0],
        );
        assert_eq!(
            written,
            b"a   42|hi  |0xff|z%|BEEF|(null)|-1.50|2.5e+000".to_vec()
        );
        assert_eq!(result, written.len() as i32);
        let (result, written) = run(
            &mut e,
            b"%I64x|%hd|%+i|%05d|%o|%#o",
            &[0, 1, 0x1_0005, 3, 7, 8, 8],
        );
        assert_eq!(written, b"100000000|5|+3|00007|10|010".to_vec());
        assert_eq!(result, written.len() as i32);
        let count = e.mem.alloc(4);
        let (result, written) = run(&mut e, b"ab%n", &[count]);
        assert_eq!((result, written), (2, b"ab".to_vec()));
        assert_eq!(e.mem.u32(count), 2);
    }

    #[test]
    fn output_s_refuses_invalid_and_unfinished_formats() {
        let (mut e, slot) = output_engine();
        // A character that is no conversion after '%' is the invalid state.
        let (result, written) = run(&mut e, b"ab%q", &[]);
        assert_eq!((result, written), (-1, b"ab".to_vec()));
        assert_eq!(e.mem.u32(slot), 0x16);
        assert_eq!(calls(&e, HANDLER).len(), 1);
        // A flag after the width is invalid too ("%5-d").
        assert_eq!(run(&mut e, b"%5-d", &[1]).0, -1);
        // A format ending inside a conversion is refused at the end.
        let (result, written) = run(&mut e, b"xy%", &[]);
        assert_eq!((result, written), (-1, b"xy".to_vec()));
        let (result, written) = run(&mut e, b"%-5", &[]);
        assert_eq!((result, written), (-1, Vec::new()));
        assert_eq!(calls(&e, HANDLER).len(), 4);
        // "%%" and plain text end in the normal state; a conversion ends in state 7.
        assert_eq!(run(&mut e, b"100%%", &[]), (4, b"100%".to_vec()));
        assert_eq!(run(&mut e, b"%d", &[9]), (1, b"9".to_vec()));
        assert_eq!(run(&mut e, b"", &[]), (0, Vec::new()));
        // An 'I' not before a size or an integer conversion is written as text.
        assert_eq!(run(&mut e, b"%Iq", &[]).1, b"Iq".to_vec());
        assert_eq!(calls(&e, HANDLER).len(), 4);
    }

    #[test]
    fn output_s_refuses_null_streams_and_formats() {
        let (mut e, _) = output_engine();
        let f = text(&mut e, b"x");
        assert_eq!(output_s_l(&mut e, Ptr::new(0), Ptr::new(f), 0, 0), -1);
        let stream = e.mem.alloc(0x20);
        let buffer = e.mem.alloc(8);
        string_stream(&mut e, stream, buffer, 8);
        assert_eq!(output_s_l(&mut e, Ptr::new(stream), Ptr::new(0), 0, 0), -1);
        assert_eq!(calls(&e, HANDLER).len(), 2);
    }

    #[test]
    fn printf_functions_are_registered() {
        let table = funcs();
        for addr in [
            0x00ec_623au32,
            0x00ec_6c46,
            0x00ec_6cc4,
            0x00ec_6a61,
            0x00ec_6b2d,
            0x00ec_6bb5,
            0x00ec_a38d,
            0x00ec_dd3e,
            0x00ed_4685,
            0x00ed_23e3,
            0x00ed_246b,
            0x00ed_22e9,
            0x00ed_1996,
            0x00ed_1a09,
            0x00ed_1a8b,
            0x00ed_1ae9,
            0x00ed_1b22,
            0x00ed_1b35,
        ] {
            assert!(table.iter().any(|f| f.0 == addr), "{addr:08x}");
        }
    }
}

#[cfg(test)]
mod exe_tests {
    use super::*;

    type Expect<'a> = (&'a [u8], &'a [u32], &'a str);
    type Float<'a> = (&'a [u8], f64, &'a str);

    /// An engine on the exe's data (`NV_EXE`, the installed FalloutNV.exe 1.4.0.525), with the
    /// thread block doubled to one on the global locale, pointer decoding and encoding the
    /// identity (as `EncodePointer` is here) and the invalid-parameter handler doubled; `None`
    /// without the variable.
    fn exe_engine() -> Option<Engine> {
        let path = std::env::var("NV_EXE").ok().filter(|p| !p.is_empty())?;
        let mut e = Engine::with_exe(std::path::Path::new(&path)).expect("NV_EXE");
        let block = e.mem.alloc(0x400);
        let locale = e.mem.u32(GLOBAL_LOCALE_INFO);
        e.mem.set_u32(block + 0x6c, locale);
        e.register_double(GETPTD_NOEXIT, move |_, _| block.into_ret());
        e.register_double(0x00ed_11de, move |_, _| block.into_ret());
        // `_LocaleUpdate` on the global locale and multibyte info (`__ptlocinfo`,
        // `__ptmbcinfo`), the thread block, not updated.
        e.register_double(LOCALE_UPDATE, move |e, a| {
            let mbc = e.mem.u32(GLOBAL_MBC_INFO);
            let info = e.mem.u32(GLOBAL_LOCALE_INFO);
            e.mem.set_u32(a[0], info);
            e.mem.set_u32(a[0] + 4, mbc);
            e.mem.set_u32(a[0] + 8, block);
            e.mem.set_u8(a[0] + 0xc, 0);
            a[0].into_ret()
        });
        e.register(DECODE_POINTER, |_, a| a[0].into_ret());
        e.register(0x00ed_0f17, |_, a| a[0].into_ret());
        e.mem.set_u32(INVALID_PARAMETER_HANDLER, 0x0e30_0000);
        e.register(0x0e30_0000, |_, _| Ret::default());
        // `__cfltcvt_init` (`00ec5eed`, a C initializer): the float helpers' table.
        e.call(0x00ec_5eed, &args![]);
        Some(e)
    }

    /// `vsprintf(format, words)` through the registered entry (the exe's `_output_l` and float
    /// conversions on the exe's tables).
    fn vsp(e: &mut Engine, format: &[u8], words: &[u32]) -> String {
        let f = e.mem.alloc(format.len() as u32 + 1);
        e.mem.set_cstr(f, format);
        let list = e.mem.alloc(4 * words.len().max(1) as u32);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *w);
        }
        let dest = e.mem.alloc(512);
        let n = e.call(0x00ec_6cc4, &args![dest, f, list]).i32();
        let s = e.mem.cstr(dest);
        assert_eq!(n as usize, s.len());
        String::from_utf8(s).unwrap()
    }

    fn dbl(v: f64) -> [u32; 2] {
        let b = v.to_bits();
        [b as u32, (b >> 32) as u32]
    }

    #[test]
    fn the_exes_vsprintf_formats_integers_strings_and_floats() {
        let Some(mut e) = exe_engine() else {
            return;
        };
        let s = e.mem.alloc(16);
        e.mem.set_cstr(s, b"Doc Mitchell");
        let cases: [Expect; 8] = [
            (
                b"%d|%i|%u",
                &[(-5i32) as u32, 42, (-1i32) as u32],
                "-5|42|4294967295",
            ),
            (
                b"%08X|%x|%X|%o|%#x|%#o",
                &[0xbeef, 255, 255, 8, 255, 8],
                "0000BEEF|ff|FF|10|0xff|010",
            ),
            (
                b"%03i|%10i|%-6d|%+d|% d",
                &[7, 7, 7, 7, 7],
                "007|         7|7     |+7| 7",
            ),
            (b"%.3d|%5.3d|%-5.3d|%.0d", &[5, 5, 5, 0], "005|  005|005  |"),
            (
                b"%s|%.4s|%10s|%-10s|%s",
                &[s, s, s, s, 0],
                "Doc Mitchell|Doc |Doc Mitchell|Doc Mitchell|(null)",
            ),
            (
                b"%c%c|100%%|%p|%hd|%lu",
                &[b'N' as u32, b'V' as u32, 0x1234, 0x1_0005, 9],
                "NV|100%|00001234|5|9",
            ),
            (b"%*d|%-*d|%.*s", &[4, 9, 3, 9, 2, s], "   9|9  |Do"),
            (
                b"%I64d|%llx",
                &[0xffff_ffff, 1, 0, 3],
                "8589934591|300000000",
            ),
        ];
        for (format, words, want) in cases {
            assert_eq!(vsp(&mut e, format, words), want);
        }
        let floats: [Float; 27] = [
            (b"%f", 1.5, "1.500000"),
            (b"%0.2f", 3.45678, "3.46"),
            (b"%.2f", 2.675, "2.67"),
            (b"%.0f", 0.5, "1"),
            (b"%.0f", 2.5, "3"),
            (b"%5.3f|", -0.0004, "-0.000|"),
            (b"%.2f", 0.006, "0.01"),
            (b"%.2f", 9.999, "10.00"),
            (b"%f", 0.0, "0.000000"),
            (b"%08.2f", -1.5, "-0001.50"),
            (b"%+.1f", 1.25, "+1.3"),
            (b"%.20f", 0.1, "0.10000000000000001000"),
            (b"%e", 1.0, "1.000000e+000"),
            (b"%E", 12345.678, "1.234568E+004"),
            (b"%.2e", 0.000123, "1.23e-004"),
            (b"%e", 0.0, "0.000000e+000"),
            (b"%g", 100000.0, "100000"),
            (b"%g", 1000000.0, "1e+006"),
            (b"%g", 0.0001, "0.0001"),
            (b"%g", 0.00001, "1e-005"),
            (b"%g", 1.5, "1.5"),
            (b"%g", 0.0, "0"),
            (b"%G", 1.5e20, "1.5E+020"),
            (b"%#g", 1.5, "1.50000"),
            (b"%.3g", 1234.5, "1.23e+003"),
            (b"%10.1f|", 2.25, "       2.3|"),
            (b"%-10.1f|", 2.25, "2.3       |"),
        ];
        for (format, value, want) in floats {
            assert_eq!(vsp(&mut e, format, &dbl(value)), want, "{value}");
        }
        let specials: [(u64, &str); 4] = [
            (f64::INFINITY.to_bits(), "1.#INF00"),
            (f64::NEG_INFINITY.to_bits(), "-1.#INF00"),
            (0xfff8_0000_0000_0000, "-1.#IND00"),
            (0x7ff8_0000_0000_0001, "1.#QNAN0"),
        ];
        for (bits, want) in specials {
            assert_eq!(vsp(&mut e, b"%f", &dbl(f64::from_bits(bits))), want);
        }
        assert_eq!(vsp(&mut e, b"%g", &dbl(f64::INFINITY)), "1.#INF");
        assert_eq!(
            vsp(
                &mut e,
                b"Script %s line %i: form %08X, %.2f",
                &[s, 12, 0x0001_f2c0, 0, 0x4000_0000]
            ),
            "Script Doc Mitchell line 12: form 0001F2C0, 2.00"
        );
    }

    #[test]
    fn the_exes_secure_and_counted_forms_on_the_exes_tables() {
        let Some(mut e) = exe_engine() else {
            return;
        };
        let f = e.mem.alloc(16);
        e.mem.set_cstr(f, b"%s %d");
        let s = e.mem.alloc(8);
        e.mem.set_cstr(s, b"level");
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, s);
        e.mem.set_u32(list + 4, 2);
        let dest = e.mem.alloc(16);
        assert_eq!(e.call(0x00ec_6bb5, &args![dest, 16u32, f, list]).i32(), 7);
        assert_eq!(e.mem.cstr(dest), b"level 2".to_vec());
        // Seven characters and the zero don't fit in seven bytes: empty, ERANGE, -1.
        assert_eq!(e.call(0x00ec_6bb5, &args![dest, 7u32, f, list]).i32(), -1);
        assert_eq!(e.mem.u8(dest), 0);
        // The secure form refuses an unknown conversion.
        let bad = e.mem.alloc(8);
        e.mem.set_cstr(bad, b"%q");
        assert_eq!(
            e.call(0x00ec_6bb5, &args![dest, 16u32, bad, list]).i32(),
            -1
        );
        // _snprintf: fits, fills exactly, overflows.
        let t = e.mem.alloc(16);
        e.mem.set_cstr(t, b"abc%d");
        let buffer = e.mem.alloc(16);
        e.mem.write(buffer, &[0xaa; 16]);
        assert_eq!(e.call(0x00ec_a38d, &args![buffer, 8u32, t, 7u32]).i32(), 4);
        assert_eq!(e.mem.bytes(buffer, 6), b"abc7\0\xaa".to_vec());
        e.mem.write(buffer, &[0xaa; 16]);
        assert_eq!(e.call(0x00ec_a38d, &args![buffer, 4u32, t, 7u32]).i32(), 4);
        assert_eq!(e.mem.bytes(buffer, 5), b"abc7\xaa".to_vec());
        e.mem.write(buffer, &[0xaa; 16]);
        assert_eq!(e.call(0x00ec_a38d, &args![buffer, 3u32, t, 7u32]).i32(), -1);
        assert_eq!(e.mem.bytes(buffer, 4), b"abc\xaa".to_vec());
        // sprintf writes the text and its zero.
        e.mem.write(buffer, &[0xaa; 16]);
        assert_eq!(e.call(0x00ec_623a, &args![buffer, t, 12u32]).i32(), 5);
        assert_eq!(e.mem.bytes(buffer, 7), b"abc12\0\xaa".to_vec());
    }
}
