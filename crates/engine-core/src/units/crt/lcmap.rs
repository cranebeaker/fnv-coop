//! `__crtLCMapStringA_stat` (`00ed04de`, MSVC 2008 LIBCMT, statically linked into
//! FalloutNV.exe 1.4.0.525), translated from the exe.

use super::*;

/// `__ansicp` (`00ee13e5`), `__convertcp` (`00ee142e`), the heap allocation `_malloca`
/// falls back on (`00ecd1c7`), `__freea` (`00ec8e14`), `free` (`00ecd291`), `memset`
/// (`00ec61c0`).
const ANSICP: u32 = 0x00ee_13e5;
const CONVERTCP: u32 = 0x00ee_142e;
const MALLOC: u32 = 0x00ec_d1c7;
const FREEA: u32 = 0x00ec_8e14;
const FREE_FN: u32 = 0x00ec_d291;
const MEMSET: u32 = 0x00ec_61c0;
/// The import slots of KERNEL32 `LCMapStringW` (`00fdf1d4`), `LCMapStringA` (`00fdf1ac`),
/// `MultiByteToWideChar` (`00fdf128`), `WideCharToMultiByte` (`00fdf170`) and
/// `GetLastError` (`00fdf094`).
const LC_MAP_STRING_W: u32 = 0x00fd_f1d4;
const LC_MAP_STRING_A: u32 = 0x00fd_f1ac;
const MULTI_BYTE_TO_WIDE_CHAR: u32 = 0x00fd_f128;
const WIDE_CHAR_TO_MULTI_BYTE: u32 = 0x00fd_f170;
const GET_LAST_ERROR: u32 = 0x00fd_f094;
/// Which `LCMapString` works (`0126fdf8`: 0 not known yet, 1 the wide one, 2 only the
/// ANSI one) and the empty wide text it is probed with (`01011d78`).
const LC_MAP_MODE: u32 = 0x0126_fdf8;
const EMPTY_WIDE: u32 = 0x0101_1d78;

/// A `_malloca` block: below 0x400 bytes (header included) on the stack (marked 0xcccc),
/// above from the heap (marked 0xdddd); the 8-byte header is skipped. `None` when the heap
/// has none. The stack blocks are given back when the frame ends ([`release`]).
struct Malloca {
    buffer: u32,
    stack: Option<u32>,
}

fn malloca(e: &mut Engine, size: u32) -> Option<Malloca> {
    if size <= 0x400 {
        // `_alloca` through the probe `00ec5ec0`: a stack block here.
        let block = e.mem.alloc(size);
        e.mem.set_u32(block, 0xcccc);
        Some(Malloca {
            buffer: block + 8,
            stack: Some(block),
        })
    } else {
        let block = e.call(MALLOC, &args![size]).u32();
        if block == 0 {
            return None;
        }
        e.mem.set_u32(block, 0xdddd);
        Some(Malloca {
            buffer: block + 8,
            stack: None,
        })
    }
}

/// The end of the frame: the stack blocks of `_malloca` go away with it.
fn release(e: &mut Engine, blocks: &[&Option<Malloca>]) {
    for block in blocks.iter().copied().flatten() {
        if let Some(stack) = block.stack {
            e.mem.free(stack);
        }
    }
}

/// A buffer of `count` wide characters as the exe sizes it: none for a count of 0 or below
/// (signed), or one whose doubled size would not fit (`0xffffffe0 / count < 2`).
fn wide_buffer(e: &mut Engine, count: u32) -> Option<Malloca> {
    if (count as i32) <= 0 || 0xffff_ffe0 / count < 2 {
        return None;
    }
    malloca(e, count.wrapping_mul(2).wrapping_add(8))
}

// Translated from 00ed04de (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed04de - the /GS stack cookie check (00ec408c) is not translated
/// `__crtLCMapStringA_stat` (Ghidra's function ID), `ECX` the `_locale_t` (its first word
/// the locale information), then LCID, flags, source, source length, target, target size,
/// code page, "fail on invalid characters". The first call probes `LCMapStringW` (works:
/// mode 1; `ERROR_CALL_NOT_IMPLEMENTED`: mode 2). A positive length is cut after the
/// source's first zero (counting it). Mode 1: the source becomes wide
/// (`MultiByteToWideChar`, its length first, then into a `_malloca` buffer), is mapped
/// with `LCMapStringW` (a sort key straight into the target when it fits, answering the
/// key's size; otherwise into a second buffer and back with `WideCharToMultiByte`), and
/// the length answered (0 on any failure). Modes 0 and 2: when the LCID's ANSI code page
/// (`__ansicp`; -1 answers 0) is the code page, `LCMapStringA` straight; otherwise the
/// source is converted to the LCID's code page (`__convertcp`), mapped into a cleared
/// buffer and converted back into the target, freeing what was converted. Any other mode
/// answers 0.
#[allow(clippy::too_many_arguments)]
pub fn crt_lc_map_string_a_stat(
    e: &mut Engine,
    locale: u32,
    lcid: u32,
    flags: u32,
    source: u32,
    length: u32,
    target: u32,
    size: u32,
    code_page: u32,
    fail_invalid: u32,
) -> u32 {
    if e.mem.u32(LC_MAP_MODE) == 0 {
        let probed = e
            .call(
                LC_MAP_STRING_W,
                &args![0u32, 0x100u32, EMPTY_WIDE, 1u32, 0u32, 0u32],
            )
            .u32();
        if probed != 0 {
            e.mem.set_u32(LC_MAP_MODE, 1);
        } else if e.call(GET_LAST_ERROR, &args![]).u32() == 0x78 {
            e.mem.set_u32(LC_MAP_MODE, 2);
        }
    }
    let mut length = length;
    if (length as i32) > 0 {
        let mut left = length;
        let mut at = source;
        loop {
            left = left.wrapping_sub(1);
            if e.mem.u8(at) == 0 {
                break;
            }
            at = at.wrapping_add(1);
            if left == 0 {
                left = 0xffff_ffff;
                break;
            }
        }
        let mut cut = length.wrapping_sub(left).wrapping_sub(1);
        if (cut as i32) < (length as i32) {
            cut = cut.wrapping_add(1);
        }
        length = cut;
    }
    match e.mem.u32(LC_MAP_MODE) {
        0 | 2 => ansi_path(
            e, locale, lcid, flags, source, length, target, size, code_page,
        ),
        1 => wide_path(
            e,
            locale,
            lcid,
            flags,
            source,
            length,
            target,
            size,
            code_page,
            fail_invalid,
        ),
        _ => 0,
    }
}

/// Mode 1 (`00ed057d`..`00ed0713`).
#[allow(clippy::too_many_arguments)]
fn wide_path(
    e: &mut Engine,
    locale: u32,
    lcid: u32,
    flags: u32,
    source: u32,
    length: u32,
    target: u32,
    size: u32,
    code_page: u32,
    fail_invalid: u32,
) -> u32 {
    let code_page = if code_page == 0 {
        let info = e.mem.u32(locale);
        e.mem.u32(info.wrapping_add(4))
    } else {
        code_page
    };
    let to_wide = if fail_invalid != 0 { 9u32 } else { 1 };
    let wide_count = e
        .call(
            MULTI_BYTE_TO_WIDE_CHAR,
            &args![code_page, to_wide, source, length, 0u32, 0u32],
        )
        .u32();
    if wide_count == 0 {
        return 0;
    }
    let input = wide_buffer(e, wide_count);
    let Some(input_block) = &input else {
        return 0;
    };
    let wide = input_block.buffer;
    let mut result = 0;
    let converted = e
        .call(
            MULTI_BYTE_TO_WIDE_CHAR,
            &args![code_page, 1u32, source, length, wide, wide_count],
        )
        .u32();
    let mut output = None;
    if converted != 0 {
        let mapped = e
            .call(
                LC_MAP_STRING_W,
                &args![lcid, flags, wide, wide_count, 0u32, 0u32],
            )
            .u32();
        result = mapped;
        if mapped != 0 {
            if flags & 0x400 != 0 {
                if size != 0 && (mapped as i32) <= (size as i32) {
                    e.call(
                        LC_MAP_STRING_W,
                        &args![lcid, flags, wide, wide_count, target, size],
                    );
                }
            } else {
                output = wide_buffer(e, mapped);
                if let Some(out) = &output {
                    let buffer = out.buffer;
                    let done = e
                        .call(
                            LC_MAP_STRING_W,
                            &args![lcid, flags, wide, wide_count, buffer, mapped],
                        )
                        .u32();
                    if done != 0 {
                        let (to, room) = if size == 0 { (0, 0) } else { (target, size) };
                        result = e
                            .call(
                                WIDE_CHAR_TO_MULTI_BYTE,
                                &args![code_page, 0u32, buffer, mapped, to, room, 0u32, 0u32],
                            )
                            .u32();
                    }
                    e.call(FREEA, &args![buffer]);
                }
            }
        }
    }
    e.call(FREEA, &args![wide]);
    release(e, &[&input, &output]);
    result
}

/// Modes 0 and 2 (`00ed0718`..`00ed086f`).
#[allow(clippy::too_many_arguments)]
fn ansi_path(
    e: &mut Engine,
    locale: u32,
    lcid: u32,
    flags: u32,
    source: u32,
    length: u32,
    target: u32,
    size: u32,
    code_page: u32,
) -> u32 {
    let lcid = if lcid == 0 {
        let info = e.mem.u32(locale);
        e.mem.u32(info.wrapping_add(0x14))
    } else {
        lcid
    };
    let code_page = if code_page == 0 {
        let info = e.mem.u32(locale);
        e.mem.u32(info.wrapping_add(4))
    } else {
        code_page
    };
    let lcid_code_page = e.call(ANSICP, &args![lcid]).u32();
    if lcid_code_page == 0xffff_ffff {
        return 0;
    }
    if lcid_code_page == code_page {
        return e
            .call(
                LC_MAP_STRING_A,
                &args![lcid, flags, source, length, target, size],
            )
            .u32();
    }
    // The frame words __convertcp writes through: the source length (EBP+0x14) and the
    // mapped length (EBP-0x8).
    e.with_stack(8, |e, words| {
        let length_word = words.addr();
        let mapped_word = length_word + 4;
        e.mem.set_u32(length_word, length);
        let converted = e
            .call(
                CONVERTCP,
                &args![code_page, lcid_code_page, source, length_word, 0u32, 0u32],
            )
            .u32();
        if converted == 0 {
            return 0;
        }
        let mut output = 0;
        let count = e.mem.u32(length_word);
        let mapped = e
            .call(
                LC_MAP_STRING_A,
                &args![lcid, flags, converted, count, 0u32, 0u32],
            )
            .u32();
        let mut result = 0;
        if mapped != 0 {
            let block = if (mapped as i32) > 0 && mapped <= 0xffff_ffe0 {
                malloca(e, mapped.wrapping_add(8))
            } else {
                None
            };
            if let Some(buffer) = &block {
                let buffer_at = buffer.buffer;
                e.call(MEMSET, &args![buffer_at, 0u32, mapped]);
                let count = e.mem.u32(length_word);
                let done = e
                    .call(
                        LC_MAP_STRING_A,
                        &args![lcid, flags, converted, count, buffer_at, mapped],
                    )
                    .u32();
                e.mem.set_u32(mapped_word, done);
                if done != 0 {
                    output = e
                        .call(
                            CONVERTCP,
                            &args![
                                lcid_code_page,
                                code_page,
                                buffer_at,
                                mapped_word,
                                target,
                                size
                            ],
                        )
                        .u32();
                    result = if output != 0 {
                        e.mem.u32(mapped_word)
                    } else {
                        0
                    };
                }
                e.call(FREEA, &args![buffer_at]);
            }
            release(e, &[&block]);
        }
        e.call(FREE_FN, &args![converted]);
        if output != 0 && output != target {
            e.call(FREE_FN, &args![output]);
        }
        result
    })
}

/// The registry entry: `ECX` first.
fn crt_lc_map_string_a_stat_entry(e: &mut Engine, words: &[u32]) -> Ret {
    crt_lc_map_string_a_stat(
        e, words[0], words[1], words[2], words[3], words[4], words[5], words[6], words[7], words[8],
    )
    .into_ret()
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![(0x00ed_04de, crt_lc_map_string_a_stat_entry as AbiFn)]
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

    /// An engine with a locale (code page 1252, LCID 0x409) and doubled conversions: bytes
    /// to words and back one for one, `LCMapString` upper-casing ASCII letters.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let info = e.mem.alloc(0xd8);
        e.mem.set_u32(info + 4, 1252);
        e.mem.set_u32(info + 0x14, 0x409);
        let locale = e.mem.alloc(8);
        e.mem.set_u32(locale, info);
        e.register_double(MULTI_BYTE_TO_WIDE_CHAR, |e, a| {
            if a[4] != 0 {
                for i in 0..a[3] {
                    let b = e.mem.u8(a[2] + i);
                    e.mem.set_u16(a[4] + 2 * i, u16::from(b));
                }
            }
            a[3].into_ret()
        });
        e.register_double(WIDE_CHAR_TO_MULTI_BYTE, |e, a| {
            if a[4] != 0 {
                for i in 0..a[3] {
                    let w = e.mem.u16(a[2] + 2 * i);
                    e.mem.set_u8(a[4] + i, w as u8);
                }
            }
            a[3].into_ret()
        });
        e.register_double(LC_MAP_STRING_W, |e, a| {
            if a[4] != 0 {
                for i in 0..a[3] {
                    let w = e.mem.u16(a[2] + 2 * i);
                    e.mem
                        .set_u16(a[4] + 2 * i, (w as u8).to_ascii_uppercase().into());
                }
            }
            a[3].into_ret()
        });
        e.register_double(LC_MAP_STRING_A, |e, a| {
            if a[4] != 0 {
                for i in 0..a[3] {
                    let b = e.mem.u8(a[2] + i);
                    e.mem.set_u8(a[4] + i, b.to_ascii_uppercase());
                }
            }
            a[3].into_ret()
        });
        e.call_log = Some(vec![]);
        (e, locale)
    }

    fn text(e: &mut Engine, t: &[u8]) -> u32 {
        let at = e.mem.alloc(t.len() as u32 + 1);
        e.mem.set_cstr(at, t);
        at
    }

    #[test]
    fn the_wide_form_maps_through_wide_buffers() {
        let (mut e, locale) = engine();
        let source = text(&mut e, b"vault");
        let target = e.mem.alloc(16);
        let n = e
            .call(
                0x00ed_04de,
                &args![locale, 0x409u32, 0x200u32, source, 16u32, target, 16u32, 0u32, 0u32],
            )
            .u32();
        // The probe found the wide form; the length was cut after the zero (6).
        assert_eq!(e.mem.u32(LC_MAP_MODE), 1);
        assert_eq!(n, 6);
        assert_eq!(e.mem.cstr(target), b"VAULT".to_vec());
        assert_eq!(
            calls(&e, MULTI_BYTE_TO_WIDE_CHAR)[0][..4],
            [1252, 1, source, 6]
        );
        // A sort key goes straight into the target, and only when it fits.
        let n = crt_lc_map_string_a_stat(&mut e, locale, 0x409, 0x400, source, 5, target, 4, 0, 1);
        assert_eq!(n, 5);
        // "Fail on invalid characters" asks the length query for MB_ERR_INVALID_CHARS (9).
        let to_wide = calls(&e, MULTI_BYTE_TO_WIDE_CHAR);
        assert_eq!((to_wide[2][1], to_wide[3][1]), (9, 1));
        let sort_calls = calls(&e, LC_MAP_STRING_W).len();
        crt_lc_map_string_a_stat(&mut e, locale, 0x409, 0x400, source, 5, target, 8, 0, 0);
        assert_eq!(calls(&e, LC_MAP_STRING_W).len(), sort_calls + 2);
        // A failed conversion answers 0.
        e.register_double(MULTI_BYTE_TO_WIDE_CHAR, |_, _| 0u32.into_ret());
        assert_eq!(
            crt_lc_map_string_a_stat(&mut e, locale, 0x409, 0x200, source, 5, target, 8, 0, 0),
            0
        );
    }

    #[test]
    fn the_ansi_form_maps_directly_or_through_the_lcid_code_page() {
        let (mut e, locale) = engine();
        e.register_double(LC_MAP_STRING_W, |_, _| 0u32.into_ret());
        e.register_double(GET_LAST_ERROR, |_, _| 0x78u32.into_ret());
        e.register_double(ANSICP, |_, a| {
            (if a[0] == 0x409 { 1252u32 } else { 932 }).into_ret()
        });
        let source = text(&mut e, b"ncr");
        let target = e.mem.alloc(16);
        let n = crt_lc_map_string_a_stat(&mut e, locale, 0, 0x200, source, 3, target, 16, 0, 0);
        assert_eq!(e.mem.u32(LC_MAP_MODE), 2);
        assert_eq!(n, 3);
        assert_eq!(e.mem.cstr(target), b"NCR".to_vec());
        assert_eq!(calls(&e, LC_MAP_STRING_A)[0][..2], [0x409, 0x200]);
        // Another LCID's code page: converted there and back, the conversions freed.
        e.register_double(CONVERTCP, |e, a| {
            let n = e.mem.u32(a[3]);
            let to = if a[4] != 0 { a[4] } else { e.mem.alloc(n + 1) };
            for i in 0..n {
                let b = e.mem.u8(a[2] + i);
                e.mem.set_u8(to + i, b);
            }
            to.into_ret()
        });
        e.register_double(FREE_FN, |_, _| Ret::default());
        let target = e.mem.alloc(16);
        let n = crt_lc_map_string_a_stat(&mut e, locale, 0x411, 0x200, source, 3, target, 16, 0, 0);
        assert_eq!(n, 3);
        assert_eq!(e.mem.cstr(target), b"NCR".to_vec());
        let converts = calls(&e, CONVERTCP);
        assert_eq!((converts[0][0], converts[0][1]), (1252, 932));
        assert_eq!((converts[1][0], converts[1][1]), (932, 1252));
        assert_eq!(calls(&e, FREE_FN).len(), 1);
        // An unknown LCID code page answers 0.
        e.register_double(ANSICP, |_, _| 0xffff_ffffu32.into_ret());
        assert_eq!(
            crt_lc_map_string_a_stat(&mut e, locale, 0x411, 0x200, source, 3, target, 16, 0, 0),
            0
        );
    }
}
