//! `setlocale`'s machinery (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe 1.4.0.525),
//! translated from the exe: `__copytlocinfo_nolock` (`00ecf6de`), `sync_legacy_variables_lk`
//! (`00ecf7b8`), `_free_locale` (`00ecf80b`) with its `__finally` blocks, `__strcats`
//! (`00ecf8cd`), `___lc_strtolc` (`00ecf90d`), `___lc_lctostr` (`00ecfa36`),
//! `__setlocale_get_all` (`00ecfaa3`), `__expandlocale` (`00ecfc20`), `__setlocale_set_cat`
//! (`00ecfe51`), `__setlocale_nolock` (`00ed0153`), and the `LC_TIME` and `LC_NUMERIC` init
//! callbacks `___init_time` (`00ee045e`) and `___init_numeric` (`00ee051c`).
//!
//! The locale information (`threadlocinfo`, 0xd8 bytes) is addressed by offset: the reference
//! count (+0x0), the code page (+0x4), the collation code page (+0x8), the `lc_handle` words
//! (+0xc), the `LC_ID` triples (+0x24, six bytes each), the category records (+0x48, 0x10
//! bytes each: name, wide name, reference count, wide reference count), `mb_cur_max` (+0xa8).

use super::*;

/// `_getptd` (`00ed11de`), `_malloc_crt` (`00ed0cdf`), `free` (`00ecd291`), `_lock`
/// (`00eddf39`), `_unlock` (`00edde5f`), `__invoke_watson` (`00ec7b08`, five words),
/// `strcpy_s` (`00ec65a6`), `strcat_s` (`00ec6bd2`), `strncpy_s` (`00ec8d5f`), `strcmp`
/// (`00ec6da0`), `strncmp` (`00ec8a19`), `strlen` (`00ec6130`), `strcspn` (`00ecdbb0`),
/// `strpbrk` (`00ecbf20`), `memset` (`00ec61c0`), `memcpy` (`00ec44d0`), `memcmp`
/// (`00ec4835`).
const GETPTD: u32 = 0x00ed_11de;
const MALLOC_CRT: u32 = 0x00ed_0cdf;
const FREE_FN: u32 = 0x00ec_d291;
const LOCK: u32 = 0x00ed_df39;
const UNLOCK: u32 = 0x00ed_de5f;
const INVOKE_WATSON: u32 = 0x00ec_7b08;
const STRCPY_S: u32 = 0x00ec_65a6;
const STRCAT_S: u32 = 0x00ec_6bd2;
const STRNCPY_S: u32 = 0x00ec_8d5f;
const STRCMP: u32 = 0x00ec_6da0;
const STRNCMP: u32 = 0x00ec_8a19;
const STRLEN: u32 = 0x00ec_6130;
const STRCSPN: u32 = 0x00ec_dbb0;
const STRPBRK: u32 = 0x00ec_bf20;
const MEMSET: u32 = 0x00ec_61c0;
const MEMCPY: u32 = 0x00ec_44d0;
const MEMCMP: u32 = 0x00ec_4835;
/// `__addlocaleref` (`00ecf5b6`), `__removelocaleref` (`00ecf645`), `__freetlocinfo`
/// (`00ecf46d`), `___get_qualified_locale` (`00ee0ff8`), `___crtGetStringTypeA`
/// (`00ee13a3`, eight words).
const ADDLOCALEREF: u32 = 0x00ec_f5b6;
const REMOVELOCALEREF: u32 = 0x00ec_f645;
const FREETLOCINFO: u32 = 0x00ec_f46d;
const GET_QUALIFIED_LOCALE: u32 = 0x00ee_0ff8;
const CRT_GET_STRING_TYPE_A: u32 = 0x00ee_13a3;
/// The import slot of KERNEL32 `InterlockedDecrement` (`00fdf1c4`).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The functions of this file that call each other.
const FREE_LOCALE_FINALLY_MBC: u32 = 0x00ec_f8b8;
const FREE_LOCALE_FINALLY_INFO: u32 = 0x00ec_f8c4;
const STRCATS: u32 = 0x00ec_f8cd;
const LC_STRTOLC: u32 = 0x00ec_f90d;
const LC_LCTOSTR: u32 = 0x00ec_fa36;
const SETLOCALE_GET_ALL: u32 = 0x00ec_faa3;
const EXPANDLOCALE: u32 = 0x00ec_fc20;
const SETLOCALE_SET_CAT: u32 = 0x00ec_fe51;

/// The global locale (`__ptlocinfo`, `011c23f8`) and the legacy copies
/// `sync_legacy_variables_lk` writes: `__lc_codepage` (`0127049c`), `__lc_collate_cp`
/// (`012704a0`), `__mb_cur_max` (`011c23fc`), `_pctype` (`011c2b50`), `_pwctype`
/// (`011c1a04`), `__lc_clike`-style `ctype1` (`011c1cb4`) and `__lc_time_curr` (`011c2b54`).
const PTLOCINFO_WORD: u32 = 0x011c_23f8;
const LEGACY_CODEPAGE: u32 = 0x0127_049c;
const LEGACY_COLLATE_CP: u32 = 0x0127_04a0;
const LEGACY_MB_CUR_MAX: u32 = 0x011c_23fc;
const LEGACY_PCTYPE: u32 = 0x011c_2b50;
const LEGACY_PWCTYPE: u32 = 0x011c_1a04;
const LEGACY_CTYPE1: u32 = 0x011c_1cb4;
const LEGACY_TIME_CURR: u32 = 0x011c_2b54;
/// The initial multibyte information (`011c1df0`) and locale information (`011c2320`),
/// which are never freed; the C locale's name (`011c2318`, `"C"`).
const INITIAL_MBC: u32 = 0x011c_1df0;
const INITIAL_INFO: u32 = 0x011c_2320;
const C_NAME_STATIC: u32 = 0x011c_2318;
/// The category table (`010f2108`, 0xc bytes each: the init callback, then the name of the
/// category; the name of category 1 at `010f210c`, the last at `010f213c`).
const CATEGORY_TABLE: u32 = 0x010f_2108;
const CATEGORY_NAMES: u32 = 0x010f_210c;
const CATEGORY_NAMES_LAST: u32 = 0x010f_213c;
/// Texts: `"_.,"` (`010f21c8`), `"=;"` (`010f21cc`), `";"` (`01063d7c`), `"="`
/// (`01063ec8`), `"_"` (`01063a04`), `"."` (`01044c28`), `"C"` (`01063d10`), `""`
/// (`01011584`); the 0x7f characters 1..0x7f (`010f2148`) and the C locale's character
/// types (the word at `011c1cb0`).
const LOCALE_SEPARATORS: u32 = 0x010f_21c8;
const CATEGORY_SEPARATORS: u32 = 0x010f_21cc;
const SEMICOLON: u32 = 0x0106_3d7c;
const EQUALS: u32 = 0x0106_3ec8;
const UNDERSCORE: u32 = 0x0106_3a04;
const DOT: u32 = 0x0104_4c28;
const C_TEXT: u32 = 0x0106_3d10;
const EMPTY_TEXT: u32 = 0x0101_1584;
const ASCII_CHARS: u32 = 0x010f_2148;
const C_CTYPE_WORD: u32 = 0x011c_1cb0;

fn watson(e: &mut Engine) {
    e.call(INVOKE_WATSON, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
}

fn interlocked_decrement(e: &mut Engine, word: u32) -> u32 {
    e.call(INTERLOCKED_DECREMENT, &args![word]).u32()
}

// Translated from 00ecf6de (disassembled, FalloutNV.exe 1.4.0.525)
/// `__copytlocinfo_nolock` (Ghidra's function ID), `EAX` the new information, `ECX` the one
/// to copy: when both are set and differ, its 0x36 words are copied (`REP MOVSD`), the copy's
/// reference count cleared and `__addlocaleref` run on it.
pub fn copytlocinfo_nolock(e: &mut Engine, target: u32, source: u32) {
    if source == 0 || target == 0 || target == source {
        return;
    }
    for i in 0..0x36u32 {
        let word = e.mem.u32(source.wrapping_add(i * 4));
        e.mem.set_u32(target.wrapping_add(i * 4), word);
    }
    e.mem.set_u32(target, 0);
    e.call(ADDLOCALEREF, &args![target]);
}

// Translated from 00ecf7b8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `sync_legacy_variables_lk` (Ghidra's function ID), no arguments: the global locale's code
/// page (+0x4), collation code page (+0x8), `mb_cur_max` (+0xa8), `pctype` (+0xd4),
/// `pwctype` (+0xbc), `ctype1` (+0xc8) and `lc_time_curr` (+0xac) copied to the legacy words.
pub fn sync_legacy_variables_lk(e: &mut Engine) {
    let info = e.mem.u32(PTLOCINFO_WORD);
    for (offset, legacy) in [
        (0x4, LEGACY_CODEPAGE),
        (0x8, LEGACY_COLLATE_CP),
        (0xa8, LEGACY_MB_CUR_MAX),
        (0xd4, LEGACY_PCTYPE),
        (0xbc, LEGACY_PWCTYPE),
        (0xc8, LEGACY_CTYPE1),
        (0xac, LEGACY_TIME_CURR),
    ] {
        let word = e.mem.u32(info.wrapping_add(offset));
        e.mem.set_u32(legacy, word);
    }
}

// Translated from 00ecf80b (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecf80b - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 01180198) is not translated; its __finally blocks (00ecf8b8, 00ecf8c4) are run as the exe runs them on the way out of each guarded part.
/// `_free_locale` (no label), cdecl, a `_locale_t` (two words: the locale information, the
/// multibyte information). Null does nothing. Under lock 0xd the multibyte information loses
/// a reference (`InterlockedDecrement`) and, at none left and not the initial one
/// (`011c1df0`), is freed. Under lock 0xc a set locale information loses a reference
/// (`__removelocaleref`) and, at none left and not the initial one (`011c2320`), is freed
/// (`__freetlocinfo`). Both words become 0xbaadf00d and the pair is freed.
pub fn free_locale(e: &mut Engine, locale: u32) {
    if locale == 0 {
        return;
    }
    e.call(LOCK, &args![0xdu32]);
    let mbc = e.mem.u32(locale.wrapping_add(4));
    if mbc != 0 && interlocked_decrement(e, mbc) == 0 {
        let mbc = e.mem.u32(locale.wrapping_add(4));
        if mbc != INITIAL_MBC {
            e.call(FREE_FN, &args![mbc]);
        }
    }
    e.call(FREE_LOCALE_FINALLY_MBC, &args![]);
    if e.mem.u32(locale) != 0 {
        e.call(LOCK, &args![0xcu32]);
        let info = e.mem.u32(locale);
        e.call(REMOVELOCALEREF, &args![info]);
        let info = e.mem.u32(locale);
        if info != 0 && e.mem.u32(info) == 0 && info != INITIAL_INFO {
            e.call(FREETLOCINFO, &args![info]);
        }
        e.call(FREE_LOCALE_FINALLY_INFO, &args![]);
    }
    e.mem.set_u32(locale, 0xbaad_f00d);
    e.mem.set_u32(locale.wrapping_add(4), 0xbaad_f00d);
    e.call(FREE_FN, &args![locale]);
}

// Translated from 00ecf8b8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_free_locale`'s first `__finally` block: `_unlock(0xd)`.
pub fn free_locale_finally_mbc(e: &mut Engine) {
    e.call(UNLOCK, &args![0xdu32]);
}

// Translated from 00ecf8c4 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_free_locale`'s second `__finally` block: `_unlock(0xc)`.
pub fn free_locale_finally_info(e: &mut Engine) {
    e.call(UNLOCK, &args![0xcu32]);
}

// Translated from 00ecf8cd (disassembled, FalloutNV.exe 1.4.0.525)
/// `__strcats` (no label), cdecl with a variable list: target, its size, a count, then that
/// many texts, each appended with `strcat_s` (a failure calls `__invoke_watson`). A count of
/// 0 or below (signed) appends nothing.
pub fn strcats(e: &mut Engine, target: u32, size: u32, texts: &[u32]) {
    for &text in texts {
        if e.call(STRCAT_S, &args![target, size, text]).u32() != 0 {
            watson(e);
        }
    }
}

// Translated from 00ecf90d (disassembled, FalloutNV.exe 1.4.0.525)
/// `___lc_strtolc` (no label), cdecl: a `LC_STRINGS` (0x90 bytes: language at +0, country at
/// +0x40, code page at +0x80, 0x10 bytes) and a locale text. The strings are cleared
/// (`memset`); an empty text answers 0. `".cp"` copies the code page alone (at most 0xf
/// characters, the last byte cleared) and answers 0. Otherwise the text is split at `"_.,"`
/// (`strcspn`): the language (shorter than 0x40, not ended by `.`), the country (shorter than
/// 0x40, not ended by `_`) and the code page (shorter than 0x10, ended by the end or `,`),
/// each copied with `strncpy_s`; an end or `,` after a piece answers 0, an empty piece or a
/// piece that breaks the rules answers -1.
pub fn lc_strtolc(e: &mut Engine, names: u32, text: u32) -> i32 {
    e.call(MEMSET, &args![names, 0u32, 0x90u32]);
    let first = e.mem.u8(text);
    if first == 0 {
        return 0;
    }
    if first == b'.' && e.mem.u8(text.wrapping_add(1)) != 0 {
        let code_page = names.wrapping_add(0x80);
        if e.call(
            STRNCPY_S,
            &args![code_page, 0x10u32, text.wrapping_add(1), 0xfu32],
        )
        .u32()
            != 0
        {
            watson(e);
        }
        e.mem.set_u8(names.wrapping_add(0x8f), 0);
        return 0;
    }
    let mut field = 0u32;
    let mut piece = text;
    let mut len = e.call(STRCSPN, &args![piece, LOCALE_SEPARATORS]).u32();
    while len != 0 {
        let end = e.mem.u8(len.wrapping_add(piece));
        let (target, size) = match field {
            0 if len < 0x40 && end != b'.' => (names, 0x40u32),
            1 if len < 0x40 && end != b'_' => (names.wrapping_add(0x40), 0x40),
            2 if len < 0x10 && (end == 0 || end == b',') => (names.wrapping_add(0x80), 0x10),
            _ => return -1,
        };
        if e.call(STRNCPY_S, &args![target, size, piece, len]).u32() != 0 {
            watson(e);
        }
        if end == b',' || end == 0 {
            return 0;
        }
        field += 1;
        piece = len.wrapping_add(piece).wrapping_add(1);
        len = e.call(STRCSPN, &args![piece, LOCALE_SEPARATORS]).u32();
    }
    -1
}

// Translated from 00ecfa36 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___lc_lctostr` (no label), cdecl: target, its size, a `LC_STRINGS`. The language is copied
/// (`strcpy_s`), then `"_"` and the country and `"."` and the code page appended
/// (`__strcats`) when set; a failed copy calls `__invoke_watson`.
pub fn lc_lctostr(e: &mut Engine, target: u32, size: u32, names: u32) {
    if e.call(STRCPY_S, &args![target, size, names]).u32() != 0 {
        watson(e);
    }
    let country = names.wrapping_add(0x40);
    if e.mem.u8(country) != 0 {
        e.call(STRCATS, &args![target, size, 2u32, UNDERSCORE, country]);
    }
    let code_page = names.wrapping_add(0x80);
    if e.mem.u8(code_page) != 0 {
        e.call(STRCATS, &args![target, size, 2u32, DOT, code_page]);
    }
}

/// Releases a category record's reference-counted word (at `info + offset`): when set and its
/// count falls to 0 (`InterlockedDecrement`), it is freed.
fn release_word(e: &mut Engine, info: u32, offset: u32) {
    let word = e.mem.u32(info.wrapping_add(offset));
    if word != 0 && interlocked_decrement(e, word) == 0 {
        let word = e.mem.u32(info.wrapping_add(offset));
        e.call(FREE_FN, &args![word]);
    }
}

// Translated from 00ecfaa3 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__setlocale_get_all` (no label), `ESI` the locale information. A 0x355-byte block from
/// `_malloc_crt` (none: 0) gets a reference count of 1 and, after it, the text
/// `"LC_COLLATE=..;LC_CTYPE=..;..."` of categories 1..5 (`__strcats`, `strcat_s` for the
/// `";"`, size 0x351). When the names differ (`strcmp` of each with the next) the old
/// `LC_ALL` name and wide name (+0x50, +0x54) lose a reference, the block becomes the
/// `LC_ALL` record (name +0x48, count +0x50; +0x4c and +0x54 cleared) and its text is
/// answered. When all agree the block is freed, the old words released the same way, the
/// record cleared and category 1's name (+0x68) answered.
pub fn setlocale_get_all(e: &mut Engine, info: u32) -> u32 {
    let mut same = true;
    let block = e.call(MALLOC_CRT, &args![0x355u32]).u32();
    if block == 0 {
        return 0;
    }
    let text = block.wrapping_add(4);
    e.mem.set_u8(text, 0);
    e.mem.set_u32(block, 1);
    let mut category = 1u32;
    let mut name_slot = info.wrapping_add(0x58);
    let first = e.mem.u32(name_slot);
    let label = e.mem.u32(CATEGORY_NAMES);
    e.call(STRCATS, &args![text, 0x351u32, 3u32, label, EQUALS, first]);
    let mut entry = CATEGORY_NAMES;
    loop {
        if e.call(STRCAT_S, &args![text, 0x351u32, SEMICOLON]).u32() != 0 {
            watson(e);
        }
        let record = category.wrapping_mul(0x10).wrapping_add(info);
        let this = e.mem.u32(name_slot);
        let next = e.mem.u32(record.wrapping_add(0x58));
        if e.call(STRCMP, &args![this, next]).u32() != 0 {
            same = false;
        }
        category += 1;
        entry = entry.wrapping_add(0xc);
        let record = category.wrapping_mul(0x10).wrapping_add(info);
        name_slot = record.wrapping_add(0x48);
        let name = e.mem.u32(name_slot);
        let label = e.mem.u32(entry);
        e.call(STRCATS, &args![text, 0x351u32, 3u32, label, EQUALS, name]);
        if (entry as i32) >= (CATEGORY_NAMES_LAST as i32) {
            break;
        }
    }
    if !same {
        release_word(e, info, 0x50);
        release_word(e, info, 0x54);
        e.mem.set_u32(info.wrapping_add(0x54), 0);
        e.mem.set_u32(info.wrapping_add(0x4c), 0);
        e.mem.set_u32(info.wrapping_add(0x50), block);
        e.mem.set_u32(info.wrapping_add(0x48), text);
        return text;
    }
    e.call(FREE_FN, &args![block]);
    release_word(e, info, 0x50);
    release_word(e, info, 0x54);
    let answer = e.mem.u32(info.wrapping_add(0x68));
    for offset in [0x54, 0x4c, 0x50, 0x48] {
        e.mem.set_u32(info.wrapping_add(offset), 0);
    }
    answer
}

// Translated from 00ecfc20 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecfc20 - the /GS stack cookie check (00ec408c) is not translated
/// `__expandlocale` (no label), cdecl: a locale text, the target, its size, an `LC_ID` out
/// (six bytes, may be null), a code page out (may be null); callers push a sixth word it does
/// not read. The thread's cache (`_getptd` + 0x9c: the `LC_ID` at +0x20, the code page at
/// +0x28, the last text at +0x2c, its expansion at +0xaf) answers repeated texts.
/// Null text, target or size: 0. `"C"` copies `"C"` (`strcpy_s`), clears the outs and
/// answers the target. A text shorter than 0x83 that equals the cached expansion or text
/// is taken from the cache; otherwise it is split (`___lc_strtolc`), qualified
/// (`___get_qualified_locale`; either failing answers 0), the cache's code page set from the
/// `LC_ID`'s third word, its expansion rebuilt (`___lc_lctostr`) and the text remembered
/// (`strncpy_s`; an empty or too long text as `""`). The outs are copied from the cache, the
/// expansion copied to the target, and the cache's expansion answered.
pub fn expandlocale(
    e: &mut Engine,
    locale: u32,
    target: u32,
    size: u32,
    id_out: u32,
    code_page_out: u32,
) -> u32 {
    let cache = e.call(GETPTD, &args![]).u32().wrapping_add(0x9c);
    let code_page = cache.wrapping_add(0x28);
    let last_text = cache.wrapping_add(0x2c);
    let id = cache.wrapping_add(0x20);
    let expansion = cache.wrapping_add(0xaf);
    if locale == 0 || target == 0 || size == 0 {
        return 0;
    }
    if e.mem.u8(locale) == b'C' && e.mem.u8(locale.wrapping_add(1)) == 0 {
        if e.call(STRCPY_S, &args![target, size, C_TEXT]).u32() != 0 {
            watson(e);
        }
        if id_out != 0 {
            e.mem.set_u16(id_out, 0);
            e.mem.set_u16(id_out.wrapping_add(2), 0);
            e.mem.set_u16(id_out.wrapping_add(4), 0);
        }
        if code_page_out != 0 {
            e.mem.set_u32(code_page_out, 0);
        }
        return target;
    }
    let len = e.call(STRLEN, &args![locale]).u32();
    let cached = len < 0x83
        && (e.call(STRCMP, &args![expansion, locale]).u32() == 0
            || e.call(STRCMP, &args![last_text, locale]).u32() == 0);
    if !cached {
        let ok = e.with_stack(0x90, |e, names| {
            let names = names.addr();
            if e.call(LC_STRTOLC, &args![names, locale]).u32() != 0 {
                return false;
            }
            if e.call(GET_QUALIFIED_LOCALE, &args![names, id, names]).u32() == 0 {
                return false;
            }
            let wide = u32::from(e.mem.u16(id.wrapping_add(4)));
            e.mem.set_u32(code_page, wide);
            e.call(LC_LCTOSTR, &args![expansion, 0x83u32, names]);
            true
        });
        if !ok {
            return 0;
        }
        let (text, count) = if e.mem.u8(locale) == 0 || len >= 0x83 {
            (EMPTY_TEXT, 1u32)
        } else {
            (locale, len.wrapping_add(1))
        };
        if e.call(STRNCPY_S, &args![last_text, 0x83u32, text, count])
            .u32()
            != 0
        {
            watson(e);
        }
    }
    if id_out != 0 {
        e.call(MEMCPY, &args![id_out, id, 6u32]);
    }
    if code_page_out != 0 {
        e.call(MEMCPY, &args![code_page_out, code_page, 4u32]);
    }
    if e.call(STRCPY_S, &args![target, size, expansion]).u32() != 0 {
        watson(e);
    }
    expansion
}

// Translated from 00ecfe51 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecfe51 - the /GS stack cookie check (00ec408c) is not translated
/// `__setlocale_set_cat` (no label), `ESI` the locale information, `ECX` the locale text, the
/// category on the stack. The text is expanded (`__expandlocale`, with its `LC_ID` and code
/// page; failing answers 0); the category's current name unchanged answers it. Otherwise a
/// block of its length + 5 (`_malloc_crt`; none answers 0) takes the new name after a
/// reference count; the old name, `lc_handle` word (+0xc + 4 * category), `LC_ID` and code
/// page are kept, the new name, handle (the language word) and `LC_ID` set. For `LC_CTYPE`
/// the code page is set and looked up in the thread's five-entry cache (`_getptd` + 0x1d0,
/// pairs of code page and "C-like" flag; a hit moves to the front); a miss asks
/// `___crtGetStringTypeA` for the types of 1..0x7f and compares them (masked to 0x1ff) with
/// the C locale's (`memcmp`, 0xfe bytes), the answer stored at the front. The flag becomes
/// +0xa8. For `LC_COLLATE` the code page becomes +0x8. The category's init callback
/// (`010f2108` + 0xc * category) runs: failing restores the name, frees the block, restores
/// the handle and code page (not the `LC_ID`) and answers 0. Succeeding releases the old
/// name's count (unless the static `"C"`: at 0 the name block and wide name are freed and
/// the wide count cleared), counts the block once, stores it and answers the new name.
pub fn setlocale_set_cat(e: &mut Engine, info: u32, locale: u32, category: u32) -> u32 {
    let cache = e.call(GETPTD, &args![]).u32().wrapping_add(0x1d0);
    // The frame, EBP-0x1c4 up to EBP: the saved LC_ID (EBP-0x1c4), the new LC_ID
    // (EBP-0x1b0), its code page (EBP-0x198), the character types (EBP-0x188) and the
    // expanded name (EBP-0x88).
    e.with_stack(0x1c4, |e, frame| {
        let f = frame.addr();
        let saved_id = f;
        let new_id = f.wrapping_add(0x14);
        let new_code_page = f.wrapping_add(0x2c);
        let types = f.wrapping_add(0x3c);
        let name = f.wrapping_add(0x13c);
        if e.call(
            EXPANDLOCALE,
            &args![locale, name, 0x83u32, new_id, new_code_page, category],
        )
        .u32()
            == 0
        {
            return 0;
        }
        let record = category.wrapping_shl(4).wrapping_add(info);
        let current = e.mem.u32(record.wrapping_add(0x48));
        if e.call(STRCMP, &args![name, current]).u32() == 0 {
            return e.mem.u32(record.wrapping_add(0x48));
        }
        let size = e.call(STRLEN, &args![name]).u32().wrapping_add(5);
        let block = e.call(MALLOC_CRT, &args![size]).u32();
        if block == 0 {
            return 0;
        }
        let old_name = e.mem.u32(record.wrapping_add(0x48));
        let handle_slot = info
            .wrapping_add(category.wrapping_mul(4))
            .wrapping_add(0xc);
        let old_handle = e.mem.u32(handle_slot);
        let id_slot = category.wrapping_add(6).wrapping_mul(6).wrapping_add(info);
        e.call(MEMCPY, &args![saved_id, id_slot, 6u32]);
        let old_code_page = e.mem.u32(info.wrapping_add(4));
        if e.call(
            STRCPY_S,
            &args![block.wrapping_add(4), size.wrapping_sub(4), name],
        )
        .u32()
            != 0
        {
            watson(e);
        }
        e.mem
            .set_u32(record.wrapping_add(0x48), block.wrapping_add(4));
        let language = u32::from(e.mem.u16(new_id));
        e.mem.set_u32(handle_slot, language);
        e.call(MEMCPY, &args![id_slot, new_id, 6u32]);
        if category == 2 {
            let code_page = e.mem.u32(new_code_page);
            e.mem.set_u32(info.wrapping_add(4), code_page);
            ctype_cache(e, info, cache, types);
            let flag = e.mem.u32(cache.wrapping_add(4));
            e.mem.set_u32(info.wrapping_add(0xa8), flag);
        }
        if category == 1 {
            let code_page = e.mem.u32(new_code_page);
            e.mem.set_u32(info.wrapping_add(8), code_page);
        }
        let callback = e
            .mem
            .u32(category.wrapping_mul(0xc).wrapping_add(CATEGORY_TABLE));
        if e.call(callback, &args![info]).u32() != 0 {
            e.mem.set_u32(record.wrapping_add(0x48), old_name);
            e.call(FREE_FN, &args![block]);
            e.mem.set_u32(handle_slot, old_handle);
            e.mem.set_u32(info.wrapping_add(4), old_code_page);
            return 0;
        }
        if old_name != C_NAME_STATIC {
            let count_slot = category.wrapping_add(5).wrapping_shl(4).wrapping_add(info);
            let count = e.mem.u32(count_slot);
            if interlocked_decrement(e, count) == 0 {
                let count = e.mem.u32(count_slot);
                e.call(FREE_FN, &args![count]);
                let wide = e.mem.u32(record.wrapping_add(0x54));
                e.call(FREE_FN, &args![wide]);
                e.mem.set_u32(record.wrapping_add(0x4c), 0);
            }
        }
        e.mem.set_u32(block, 1);
        let count_slot = category.wrapping_add(5).wrapping_shl(4).wrapping_add(info);
        e.mem.set_u32(count_slot, block);
        e.mem.u32(record.wrapping_add(0x48))
    })
}

/// `__setlocale_set_cat`'s `LC_CTYPE` code-page cache (`00ecffa8`..`00ed0095`): five pairs of
/// (code page, flag) at `cache`, searched for the locale's code page (+0x4) while the pairs
/// before it shift up one place; a hit is swapped to the front, a miss (all five passed)
/// fills the front from `___crtGetStringTypeA`.
fn ctype_cache(e: &mut Engine, info: u32, cache: u32, types: u32) {
    let mut passed = 0u32;
    let mut carry_page = e.mem.u32(cache.wrapping_add(0x20));
    let mut carry_flag = e.mem.u32(cache.wrapping_add(0x24));
    let mut slot = cache;
    loop {
        if e.mem.u32(info.wrapping_add(4)) == e.mem.u32(slot) {
            if passed != 0 {
                let hit = cache.wrapping_add(passed.wrapping_mul(8));
                let page = e.mem.u32(hit);
                e.mem.set_u32(cache, page);
                let flag = e.mem.u32(hit.wrapping_add(4));
                e.mem.set_u32(cache.wrapping_add(4), flag);
                e.mem.set_u32(hit, carry_page);
                e.mem.set_u32(hit.wrapping_add(4), carry_flag);
            }
            break;
        }
        let page = e.mem.u32(slot);
        passed += 1;
        e.mem.set_u32(slot, carry_page);
        let flag = e.mem.u32(slot.wrapping_add(4));
        e.mem.set_u32(slot.wrapping_add(4), carry_flag);
        carry_page = page;
        carry_flag = flag;
        slot = slot.wrapping_add(8);
        if (passed as i32) >= 5 {
            break;
        }
    }
    if passed != 5 {
        return;
    }
    let code_page = e.mem.u32(info.wrapping_add(4));
    let lcid = e.mem.u32(info.wrapping_add(0x14));
    let ok = e
        .call(
            CRT_GET_STRING_TYPE_A,
            &args![
                0u32,
                1u32,
                ASCII_CHARS,
                0x7fu32,
                types,
                code_page,
                lcid,
                1u32
            ],
        )
        .u32();
    if ok != 0 {
        for i in 0..0x7fu32 {
            let at = types.wrapping_add(i * 2);
            let word = e.mem.u16(at);
            e.mem.set_u16(at, word & 0x1ff);
        }
        let c_types = e.mem.u32(C_CTYPE_WORD);
        let differ = e.call(MEMCMP, &args![types, c_types, 0xfeu32]).u32();
        e.mem.set_u32(cache.wrapping_add(4), u32::from(differ == 0));
    } else {
        e.mem.set_u32(cache.wrapping_add(4), 0);
    }
    let code_page = e.mem.u32(info.wrapping_add(4));
    e.mem.set_u32(cache, code_page);
}

// Translated from 00ed0153 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed0153 - the /GS stack cookie check (00ec408c) is not translated
/// `__setlocale_nolock` (Ghidra's function ID), `EDX` the locale information, `ECX` the
/// locale text, the category on the stack. A single category with a text sets it
/// (`__setlocale_set_cat`); without one its name (+0x48 + 0x10 * category) is answered.
/// `LC_ALL` without a text answers the combined name (`__setlocale_get_all`). An
/// `"LC_..."` text is taken apart at `"=;"` (`strpbrk`): each `NAME=value` names a category
/// (`strncmp` and `strlen` against the table's names; an empty name or a `;` there answers
/// 0) and its value (to `;`, `strcspn`; an empty one not ended by `;` answers 0) is copied
/// (`strncpy_s`, 0x83) and set; with no category set it answers 0, otherwise the combined
/// name. Any other text is expanded (`__expandlocale`; failing answers 0) and set for each
/// category 1..5 whose name differs (`strcmp`); one failing clears the "all agree" flag,
/// each set or unchanged one counts. The combined name is answered unless the flag is
/// clear and none counted (0).
pub fn setlocale_nolock(e: &mut Engine, info: u32, locale: u32, category: u32) -> u32 {
    if category != 0 {
        if locale != 0 {
            return e
                .call(SETLOCALE_SET_CAT, &args![info, locale, category])
                .u32();
        }
        return e.mem.u32(
            category
                .wrapping_shl(4)
                .wrapping_add(info)
                .wrapping_add(0x48),
        );
    }
    if locale == 0 {
        return e.call(SETLOCALE_GET_ALL, &args![info]).u32();
    }
    // The frame's name buffer (EBP-0x88, 0x83 bytes).
    e.with_stack(0x84, |e, buffer| {
        let buffer = buffer.addr();
        let mut all = 1u32;
        let mut counted = 0u32;
        if e.mem.u8(locale) == b'L'
            && e.mem.u8(locale.wrapping_add(1)) == b'C'
            && e.mem.u8(locale.wrapping_add(2)) == b'_'
        {
            let mut at = locale;
            loop {
                let mark = e.call(STRPBRK, &args![at, CATEGORY_SEPARATORS]).u32();
                if mark == 0 {
                    return 0;
                }
                let len = mark.wrapping_sub(at);
                if len == 0 || e.mem.u8(mark) == b';' {
                    return 0;
                }
                let mut index = 1u32;
                let mut entry = CATEGORY_NAMES;
                loop {
                    let label = e.mem.u32(entry);
                    if e.call(STRNCMP, &args![label, at, len]).u32() == 0 {
                        let label = e.mem.u32(entry);
                        if e.call(STRLEN, &args![label]).u32() == len {
                            break;
                        }
                    }
                    index += 1;
                    entry = entry.wrapping_add(0xc);
                    if (entry as i32) > (CATEGORY_NAMES_LAST as i32) {
                        break;
                    }
                }
                let value = mark.wrapping_add(1);
                let value_len = e.call(STRCSPN, &args![value, SEMICOLON]).u32();
                if value_len == 0 && e.mem.u8(value) != b';' {
                    return 0;
                }
                if (index as i32) <= 5 {
                    if e.call(STRNCPY_S, &args![buffer, 0x83u32, value, value_len])
                        .u32()
                        != 0
                    {
                        watson(e);
                    }
                    e.mem.set_u8(buffer.wrapping_add(value_len), 0);
                    if e.call(SETLOCALE_SET_CAT, &args![info, buffer, index]).u32() != 0 {
                        counted += 1;
                    }
                }
                at = value_len.wrapping_add(value);
                if e.mem.u8(at) == 0 {
                    break;
                }
                at = at.wrapping_add(1);
                if e.mem.u8(at) == 0 {
                    break;
                }
            }
            if counted == 0 {
                return 0;
            }
            return e.call(SETLOCALE_GET_ALL, &args![info]).u32();
        }
        if e.call(
            EXPANDLOCALE,
            &args![locale, buffer, 0x83u32, 0u32, 0u32, 0u32],
        )
        .u32()
            == 0
        {
            return 0;
        }
        for index in 1..=5u32 {
            let name = e
                .mem
                .u32(index.wrapping_shl(4).wrapping_add(info).wrapping_add(0x48));
            if e.call(STRCMP, &args![buffer, name]).u32() != 0 {
                if e.call(SETLOCALE_SET_CAT, &args![info, buffer, index]).u32() == 0 {
                    all = 0;
                } else {
                    counted += 1;
                }
            } else {
                counted += 1;
            }
        }
        if all == 0 && counted == 0 {
            return 0;
        }
        e.call(SETLOCALE_GET_ALL, &args![info]).u32()
    })
}

/// `__calloc_crt` (`00ed0d24`), `__get_lc_time` (`00edfeae`; `EAX` the locale information,
/// `ESI` the block), `___free_lc_time` (`00ee02c9`) and the C locale's time block
/// (`__lc_time_c`, `011c2a98`).
const CALLOC_CRT: u32 = 0x00ed_0d24;
const GET_LC_TIME: u32 = 0x00ed_feae;
const FREE_LC_TIME: u32 = 0x00ee_02c9;
const C_LC_TIME: u32 = 0x011c_2a98;

// Translated from 00ee045e (disassembled, FalloutNV.exe 1.4.0.525)
/// `___init_time` (Ghidra's function ID; `LC_TIME`'s init callback), cdecl, the locale
/// information. With a `LC_TIME` handle (+0x20) a 0xb8-byte block (`__calloc_crt`; none
/// answers 1) is filled (`__get_lc_time`; failing frees it with `___free_lc_time` and `free`
/// and answers 1) and counted once (+0xb4); without one the C locale's block is used. The
/// old block (+0xd4) loses a reference unless it is the C one, the new one is stored and 0
/// answered.
pub fn init_time(e: &mut Engine, info: u32) -> u32 {
    let block = if e.mem.u32(info.wrapping_add(0x20)) != 0 {
        let block = e.call(CALLOC_CRT, &args![1u32, 0xb8u32]).u32();
        if block == 0 {
            return 1;
        }
        if e.call(GET_LC_TIME, &args![info, block]).u32() != 0 {
            e.call(FREE_LC_TIME, &args![block]);
            e.call(FREE_FN, &args![block]);
            return 1;
        }
        e.mem.set_u32(block.wrapping_add(0xb4), 1);
        block
    } else {
        C_LC_TIME
    };
    let old = e.mem.u32(info.wrapping_add(0xd4));
    if old != C_LC_TIME {
        interlocked_decrement(e, old.wrapping_add(0xb4));
    }
    e.mem.set_u32(info.wrapping_add(0xd4), block);
    0
}

/// `_malloc_crt` (`00ed0cdf`), `__getlocaleinfo` (`00ed2af0`; locale object, type, LCID,
/// field, out), `__free_lconv_num` (`00ee04d7`) and the C locale's `lconv` (`__lconv_c`,
/// `011c19d4`).
const MALLOC_CRT_FN: u32 = 0x00ed_0cdf;
const GETLOCALEINFO: u32 = 0x00ed_2af0;
const FREE_LCONV_NUM: u32 = 0x00ee_04d7;
const C_LCONV: u32 = 0x011c_19d4;

// Translated from 00ee051c (disassembled, FalloutNV.exe 1.4.0.525)
/// `___init_numeric` (Ghidra's function ID; `LC_NUMERIC`'s init callback), cdecl, the
/// locale information. Without `LC_MONETARY` and `LC_NUMERIC` handles (+0x18, +0x1c) the C
/// locale's `lconv` is taken with no counts. Otherwise a copy of the current `lconv` (+0xbc,
/// 0x30 bytes, `__calloc_crt`) with a reference count (`_malloc_crt`) is made (either
/// failing answers 1). With a `LC_NUMERIC` handle a second count is made and the decimal
/// point, thousands separator and grouping are read (`__getlocaleinfo` with the
/// category's language, `LOCALE_SDECIMAL`/`STHOUSAND`/`SGROUPING`); any failure frees them
/// (`__free_lconv_num`) and the copy and answers -1; the grouping's digits become numbers
/// and its `;`s are removed. Without the handle the C locale's three words are copied.
/// The counts start at 1; the old counts lose a reference (the old `lconv` and its count
/// freed at 0) and the new `lconv` and counts are stored. 0 is answered.
pub fn init_numeric(e: &mut Engine, info: u32) -> u32 {
    let (lconv, count, numeric_count) =
        if e.mem.u32(info.wrapping_add(0x1c)) == 0 && e.mem.u32(info.wrapping_add(0x18)) == 0 {
            (C_LCONV, 0, 0)
        } else {
            let lconv = e.call(CALLOC_CRT, &args![1u32, 0x30u32]).u32();
            if lconv == 0 {
                return 1;
            }
            let current = e.mem.u32(info.wrapping_add(0xbc));
            for i in 0..0xcu32 {
                let word = e.mem.u32(current.wrapping_add(i * 4));
                e.mem.set_u32(lconv.wrapping_add(i * 4), word);
            }
            let count = e.call(MALLOC_CRT_FN, &args![4u32]).u32();
            if count == 0 {
                e.call(FREE_FN, &args![lconv]);
                return 1;
            }
            e.mem.set_u32(count, 0);
            let numeric_count = if e.mem.u32(info.wrapping_add(0x1c)) != 0 {
                let numeric_count = e.call(MALLOC_CRT_FN, &args![4u32]).u32();
                if numeric_count == 0 {
                    e.call(FREE_FN, &args![lconv]);
                    e.call(FREE_FN, &args![count]);
                    return 1;
                }
                e.mem.set_u32(numeric_count, 0);
                let lcid = u32::from(e.mem.u16(info.wrapping_add(0x3e)));
                // The locale object `__getlocaleinfo` takes (EBP-0x18): this information and no
                // multibyte information.
                let failed = e.with_stack(8, |e, object| {
                    let object = object.addr();
                    e.mem.set_u32(object, info);
                    e.mem.set_u32(object + 4, 0);
                    let mut failed = 0;
                    for (field, offset) in [(0xeu32, 0u32), (0xf, 4), (0x10, 8)] {
                        failed |= e
                            .call(
                                GETLOCALEINFO,
                                &args![object, 1u32, lcid, field, lconv.wrapping_add(offset)],
                            )
                            .u32();
                    }
                    failed
                });
                if failed != 0 {
                    e.call(FREE_LCONV_NUM, &args![lconv]);
                    e.call(FREE_FN, &args![lconv]);
                    e.call(FREE_FN, &args![count]);
                    return 0xffff_ffff;
                }
                let mut at = e.mem.u32(lconv.wrapping_add(8));
                while e.mem.u8(at) != 0 {
                    let c = e.mem.u8(at);
                    if (c as i8) >= 0x30 && (c as i8) <= 0x39 {
                        e.mem.set_u8(at, c - 0x30);
                        at = at.wrapping_add(1);
                    } else if c == b';' {
                        let mut to = at;
                        loop {
                            let next = e.mem.u8(to.wrapping_add(1));
                            e.mem.set_u8(to, next);
                            to = to.wrapping_add(1);
                            if e.mem.u8(to) == 0 {
                                break;
                            }
                        }
                    } else {
                        at = at.wrapping_add(1);
                    }
                }
                numeric_count
            } else {
                for i in 0..3u32 {
                    let word = e.mem.u32(C_LCONV.wrapping_add(i * 4));
                    e.mem.set_u32(lconv.wrapping_add(i * 4), word);
                }
                0
            };
            e.mem.set_u32(count, 1);
            if numeric_count != 0 {
                e.mem.set_u32(numeric_count, 1);
            }
            (lconv, count, numeric_count)
        };
    let old_numeric = e.mem.u32(info.wrapping_add(0xb4));
    if old_numeric != 0 {
        interlocked_decrement(e, old_numeric);
    }
    let old_count = e.mem.u32(info.wrapping_add(0xb0));
    if old_count != 0 && interlocked_decrement(e, old_count) == 0 {
        let old_count = e.mem.u32(info.wrapping_add(0xb0));
        e.call(FREE_FN, &args![old_count]);
        let old_lconv = e.mem.u32(info.wrapping_add(0xbc));
        e.call(FREE_FN, &args![old_lconv]);
    }
    e.mem.set_u32(info.wrapping_add(0xb4), numeric_count);
    e.mem.set_u32(info.wrapping_add(0xb0), count);
    e.mem.set_u32(info.wrapping_add(0xbc), lconv);
    0
}

/// The registry entries of the functions that take registers or a variable list.
fn copytlocinfo_entry(e: &mut Engine, words: &[u32]) -> Ret {
    copytlocinfo_nolock(e, words[0], words[1]);
    Ret::default()
}

fn strcats_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let count = words[2] as i32;
    let texts: Vec<u32> = if count > 0 {
        words[3..3 + count as usize].to_vec()
    } else {
        Vec::new()
    };
    strcats(e, words[0], words[1], &texts);
    Ret::default()
}

fn setlocale_get_all_entry(e: &mut Engine, words: &[u32]) -> Ret {
    setlocale_get_all(e, words[0]).into_ret()
}

fn setlocale_set_cat_entry(e: &mut Engine, words: &[u32]) -> Ret {
    setlocale_set_cat(e, words[0], words[1], words[2]).into_ret()
}

fn setlocale_nolock_entry(e: &mut Engine, words: &[u32]) -> Ret {
    setlocale_nolock(e, words[0], words[1], words[2]).into_ret()
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00ec_f6de, copytlocinfo_entry as AbiFn),
        entry!(0x00ec_f7b8, sync_legacy_variables_lk()),
        entry!(0x00ec_f80b, free_locale(u32)),
        entry!(0x00ec_f8b8, free_locale_finally_mbc()),
        entry!(0x00ec_f8c4, free_locale_finally_info()),
        (0x00ec_f8cd, strcats_entry as AbiFn),
        entry!(0x00ec_f90d, lc_strtolc(u32, u32) -> i32),
        entry!(0x00ec_fa36, lc_lctostr(u32, u32, u32)),
        (0x00ec_faa3, setlocale_get_all_entry as AbiFn),
        entry!(0x00ec_fc20, expandlocale(u32, u32, u32, u32, u32) -> u32),
        (0x00ec_fe51, setlocale_set_cat_entry as AbiFn),
        (0x00ed_0153, setlocale_nolock_entry as AbiFn),
        entry!(0x00ee_045e, init_time(u32) -> u32),
        entry!(0x00ee_051c, init_numeric(u32) -> u32),
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

    fn text(e: &mut Engine, t: &[u8]) -> u32 {
        let at = e.mem.alloc(t.len() as u32 + 1);
        e.mem.set_cstr(at, t);
        at
    }

    /// Writes a text at a fixed image address (mapping it first).
    fn put(e: &mut Engine, addr: u32, t: &[u8]) {
        e.mem.map(addr, t.len() as u32 + 1);
        e.mem.set_cstr(addr, t);
    }

    fn compare(e: &Engine, a: u32, b: u32) -> u32 {
        match e.mem.cstr(a).cmp(&e.mem.cstr(b)) {
            std::cmp::Ordering::Less => 0xffff_ffff,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }

    /// An engine with `strcmp`, `_malloc_crt`, `free` and `__invoke_watson` doubled, the
    /// thread's data at a fresh block, and the call log on.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let ptd = e.mem.alloc(0x300);
        e.register_double(GETPTD, move |_, _| ptd.into_ret());
        e.register_double(STRCMP, |e, a| compare(e, a[0], a[1]).into_ret());
        e.register_double(MALLOC_CRT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register_double(FREE_FN, |_, _| Ret::default());
        e.register_double(INVOKE_WATSON, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, ptd)
    }

    #[test]
    fn copytlocinfo_copies_the_words_and_takes_a_reference() {
        let (mut e, _) = engine();
        e.register_double(ADDLOCALEREF, |_, _| Ret::default());
        let source = e.mem.alloc(0xd8);
        for i in 0..0x36 {
            e.mem.set_u32(source + i * 4, 0x100 + i);
        }
        let target = e.mem.alloc(0xd8);
        e.call(0x00ec_f6de, &args![target, source]);
        assert_eq!(e.mem.u32(target), 0);
        assert_eq!(e.mem.u32(target + 4), 0x101);
        assert_eq!(e.mem.u32(target + 0xd4), 0x135);
        assert_eq!(calls(&e, ADDLOCALEREF), vec![vec![target]]);
        // The same block, or a null one, does nothing.
        e.call(0x00ec_f6de, &args![target, target]);
        e.call(0x00ec_f6de, &args![0u32, source]);
        e.call(0x00ec_f6de, &args![target, 0u32]);
        assert_eq!(calls(&e, ADDLOCALEREF).len(), 1);
    }

    #[test]
    fn sync_legacy_variables_copies_the_global_locale() {
        let (mut e, _) = engine();
        let info = e.mem.alloc(0xd8);
        for (offset, value) in [
            (4, 1252),
            (8, 1251),
            (0xa8, 1),
            (0xd4, 0x11),
            (0xbc, 0x22),
            (0xc8, 0x33),
            (0xac, 0x44),
        ] {
            e.mem.set_u32(info + offset, value);
        }
        for addr in [
            PTLOCINFO_WORD,
            LEGACY_CODEPAGE,
            LEGACY_COLLATE_CP,
            LEGACY_MB_CUR_MAX,
            LEGACY_PCTYPE,
            LEGACY_PWCTYPE,
            LEGACY_CTYPE1,
            LEGACY_TIME_CURR,
        ] {
            e.mem.map(addr, 4);
        }
        e.mem.set_u32(PTLOCINFO_WORD, info);
        e.call(0x00ec_f7b8, &args![]);
        assert_eq!(e.mem.u32(LEGACY_CODEPAGE), 1252);
        assert_eq!(e.mem.u32(LEGACY_COLLATE_CP), 1251);
        assert_eq!(e.mem.u32(LEGACY_MB_CUR_MAX), 1);
        assert_eq!(e.mem.u32(LEGACY_PCTYPE), 0x11);
        assert_eq!(e.mem.u32(LEGACY_PWCTYPE), 0x22);
        assert_eq!(e.mem.u32(LEGACY_CTYPE1), 0x33);
        assert_eq!(e.mem.u32(LEGACY_TIME_CURR), 0x44);
    }

    #[test]
    fn free_locale_releases_both_parts_under_their_locks() {
        let (mut e, _) = engine();
        for addr in [LOCK, UNLOCK, REMOVELOCALEREF, FREETLOCINFO] {
            e.register_double(addr, |_, _| Ret::default());
        }
        e.register_double(INTERLOCKED_DECREMENT, |_, _| 0u32.into_ret());
        let mbc = e.mem.alloc(8);
        let info = e.mem.alloc(8);
        let pair = e.mem.alloc(8);
        e.mem.set_u32(pair, info);
        e.mem.set_u32(pair + 4, mbc);
        e.call(0x00ec_f80b, &args![pair]);
        assert_eq!(calls(&e, LOCK), vec![vec![0xd], vec![0xc]]);
        assert_eq!(calls(&e, UNLOCK), vec![vec![0xd], vec![0xc]]);
        assert_eq!(calls(&e, REMOVELOCALEREF), vec![vec![info]]);
        assert_eq!(calls(&e, FREETLOCINFO), vec![vec![info]]);
        assert_eq!(calls(&e, FREE_FN), vec![vec![mbc], vec![pair]]);
        assert_eq!(e.mem.u32(pair), 0xbaad_f00d);
        assert_eq!(e.mem.u32(pair + 4), 0xbaad_f00d);
        // The initial parts are never freed.
        e.call_log = Some(vec![]);
        e.mem.map(INITIAL_INFO, 4);
        e.mem.set_u32(pair, INITIAL_INFO);
        e.mem.set_u32(pair + 4, INITIAL_MBC);
        e.call(0x00ec_f80b, &args![pair]);
        assert_eq!(calls(&e, FREE_FN), vec![vec![pair]]);
        assert!(calls(&e, FREETLOCINFO).is_empty());
        e.call(0x00ec_f80b, &args![0u32]);
        assert_eq!(calls(&e, LOCK).len(), 2);
    }

    #[test]
    fn strcats_appends_each_text() {
        let (mut e, _) = engine();
        let target = e.mem.alloc(16);
        e.mem.set_cstr(target, b"Pr");
        let a = text(&mut e, b"im");
        let b = text(&mut e, b"m");
        e.call(STRCATS, &args![target, 16u32, 2u32, a, b]);
        assert_eq!(e.mem.cstr(target), b"Primm".to_vec());
        e.call(STRCATS, &args![target, 16u32, 0u32]);
        assert_eq!(e.mem.cstr(target), b"Primm".to_vec());
    }

    #[test]
    fn lc_strtolc_splits_language_country_and_code_page() {
        let (mut e, _) = engine();
        put(&mut e, LOCALE_SEPARATORS, b"_.,");
        let names = e.mem.alloc(0x90);
        let full = text(&mut e, b"English_United States.1252");
        assert_eq!(lc_strtolc(&mut e, names, full), 0);
        assert_eq!(e.mem.cstr(names), b"English".to_vec());
        assert_eq!(e.mem.cstr(names + 0x40), b"United States".to_vec());
        assert_eq!(e.mem.cstr(names + 0x80), b"1252".to_vec());
        let page = text(&mut e, b".932");
        assert_eq!(e.call(LC_STRTOLC, &args![names, page]).i32(), 0);
        assert_eq!(e.mem.cstr(names), b"".to_vec());
        assert_eq!(e.mem.cstr(names + 0x80), b"932".to_vec());
        let empty = text(&mut e, b"");
        assert_eq!(lc_strtolc(&mut e, names, empty), 0);
        // A language ended by '.', an empty piece, a too long code page: -1.
        let bad = text(&mut e, b"English.1252");
        assert_eq!(lc_strtolc(&mut e, names, bad), -1);
        let bad = text(&mut e, b"_x");
        assert_eq!(lc_strtolc(&mut e, names, bad), -1);
        let bad = text(&mut e, b"a_b.12345678901234567");
        assert_eq!(lc_strtolc(&mut e, names, bad), -1);
        let comma = text(&mut e, b"German,rest");
        assert_eq!(lc_strtolc(&mut e, names, comma), 0);
        assert_eq!(e.mem.cstr(names), b"German".to_vec());
    }

    #[test]
    fn lc_lctostr_joins_the_set_parts() {
        let (mut e, _) = engine();
        put(&mut e, UNDERSCORE, b"_");
        put(&mut e, DOT, b".");
        let names = e.mem.alloc(0x90);
        e.mem.set_cstr(names, b"French");
        e.mem.set_cstr(names + 0x40, b"France");
        e.mem.set_cstr(names + 0x80, b"1252");
        let target = e.mem.alloc(0x83);
        e.call(LC_LCTOSTR, &args![target, 0x83u32, names]);
        assert_eq!(e.mem.cstr(target), b"French_France.1252".to_vec());
        e.mem.set_u8(names + 0x40, 0);
        lc_lctostr(&mut e, target, 0x83, names);
        assert_eq!(e.mem.cstr(target), b"French.1252".to_vec());
    }

    /// A locale information whose categories 1..5 are named by `names`, with the category
    /// table's labels in place.
    fn info_with(e: &mut Engine, names: [&[u8]; 5]) -> u32 {
        put(e, EQUALS, b"=");
        put(e, SEMICOLON, b";");
        e.mem.map(CATEGORY_TABLE, 0x40);
        let labels: [&[u8]; 5] = [
            b"LC_COLLATE",
            b"LC_CTYPE",
            b"LC_MONETARY",
            b"LC_NUMERIC",
            b"LC_TIME",
        ];
        for (i, label) in labels.into_iter().enumerate() {
            let at = text(e, label);
            e.mem.set_u32(CATEGORY_NAMES + i as u32 * 0xc, at);
        }
        let info = e.mem.alloc(0xd8);
        for (i, name) in names.into_iter().enumerate() {
            let at = text(e, name);
            e.mem.set_u32(info + 0x58 + i as u32 * 0x10, at);
        }
        info
    }

    #[test]
    fn setlocale_get_all_names_mixed_categories_together() {
        let (mut e, _) = engine();
        e.register_double(INTERLOCKED_DECREMENT, |_, _| 0u32.into_ret());
        let info = info_with(&mut e, [b"C", b"German_Germany.1252", b"C", b"C", b"C"]);
        let old = e.mem.alloc(8);
        e.mem.set_u32(info + 0x50, old);
        let all = e.call(SETLOCALE_GET_ALL, &args![info]).u32();
        assert_eq!(
            e.mem.cstr(all),
            b"LC_COLLATE=C;LC_CTYPE=German_Germany.1252;LC_MONETARY=C;LC_NUMERIC=C;LC_TIME=C"
                .to_vec()
        );
        assert_eq!(e.mem.u32(info + 0x48), all);
        assert_eq!(e.mem.u32(info + 0x50), all - 4);
        assert_eq!(e.mem.u32(all - 4), 1);
        assert_eq!(calls(&e, FREE_FN), vec![vec![old]]);
    }

    #[test]
    fn setlocale_get_all_answers_the_shared_name() {
        let (mut e, _) = engine();
        let info = info_with(&mut e, [b"C"; 5]);
        e.mem.set_u32(info + 0x48, 0x1234);
        let all = setlocale_get_all(&mut e, info);
        assert_eq!(all, e.mem.u32(info + 0x68));
        assert_eq!(e.mem.u32(info + 0x48), 0);
        assert_eq!(e.mem.u32(info + 0x50), 0);
        assert_eq!(calls(&e, FREE_FN).len(), 1);
        // No block: 0.
        e.register_double(MALLOC_CRT, |_, _| 0u32.into_ret());
        assert_eq!(setlocale_get_all(&mut e, info), 0);
    }

    #[test]
    fn expandlocale_answers_c_the_cache_or_a_qualified_name() {
        let (mut e, ptd) = engine();
        put(&mut e, C_TEXT, b"C");
        put(&mut e, EMPTY_TEXT, b"");
        let cache = ptd + 0x9c;
        let target = e.mem.alloc(0x83);
        let id = e.mem.alloc(8);
        let code_page = e.mem.alloc(4);
        e.mem.set_u32(code_page, 7);
        let c = text(&mut e, b"C");
        assert_eq!(expandlocale(&mut e, c, target, 0x83, id, code_page), target);
        assert_eq!(e.mem.cstr(target), b"C".to_vec());
        assert_eq!(e.mem.u32(code_page), 0);
        assert_eq!(expandlocale(&mut e, 0, target, 0x83, id, code_page), 0);
        // A new text: split, qualified, rebuilt and remembered.
        e.register_double(LC_STRTOLC, |_, _| 0u32.into_ret());
        e.register_double(GET_QUALIFIED_LOCALE, |e, a| {
            e.mem.set_u16(a[1], 0x407);
            e.mem.set_u16(a[1] + 2, 0x407);
            e.mem.set_u16(a[1] + 4, 1252);
            1u32.into_ret()
        });
        e.register_double(LC_LCTOSTR, |e, a| {
            e.mem.set_cstr(a[0], b"German_Germany.1252");
            Ret::default()
        });
        let german = text(&mut e, b"german");
        let answer = e
            .call(
                EXPANDLOCALE,
                &args![german, target, 0x83u32, id, code_page, 2u32],
            )
            .u32();
        assert_eq!(answer, cache + 0xaf);
        assert_eq!(e.mem.cstr(target), b"German_Germany.1252".to_vec());
        assert_eq!(e.mem.cstr(cache + 0x2c), b"german".to_vec());
        assert_eq!(e.mem.u32(code_page), 1252);
        assert_eq!(e.mem.u16(id), 0x407);
        assert_eq!(e.mem.u16(id + 4), 1252);
        // The same text again comes from the cache.
        expandlocale(&mut e, german, target, 0x83, 0, 0);
        assert_eq!(calls(&e, LC_STRTOLC).len(), 1);
        // A failed qualification answers 0.
        e.register_double(GET_QUALIFIED_LOCALE, |_, _| 0u32.into_ret());
        let other = text(&mut e, b"french");
        assert_eq!(expandlocale(&mut e, other, target, 0x83, 0, 0), 0);
    }

    /// A locale information for `__setlocale_set_cat`: the C locale's names, code page 0.
    fn set_cat_engine() -> (Engine, u32, u32) {
        let (mut e, ptd) = engine();
        let info = info_with(&mut e, [b"C"; 5]);
        e.mem.map(C_NAME_STATIC, 4);
        e.mem.map(ASCII_CHARS, 0x80);
        e.mem.map(C_CTYPE_WORD, 4);
        e.mem.set_u32(info + 0x14, 0x409);
        e.register_double(INTERLOCKED_DECREMENT, |_, _| 1u32.into_ret());
        e.register_double(EXPANDLOCALE, |e, a| {
            e.mem.set_cstr(a[1], b"German_Germany.1252");
            e.mem.set_u16(a[3], 0x407);
            e.mem.set_u16(a[3] + 2, 0x407);
            e.mem.set_u16(a[3] + 4, 1252);
            e.mem.set_u32(a[4], 1252);
            1u32.into_ret()
        });
        (e, ptd, info)
    }

    #[test]
    fn setlocale_set_cat_sets_ctype_and_fills_the_cache() {
        let (mut e, ptd, info) = set_cat_engine();
        e.register_double(CRT_GET_STRING_TYPE_A, |_, _| 1u32.into_ret());
        e.register_double(MEMCMP, |_, _| 0u32.into_ret());
        let callback = 0x0040_1000;
        e.register_double(callback, |_, _| 0u32.into_ret());
        e.mem.set_u32(CATEGORY_TABLE + 2 * 0xc, callback);
        let cache = ptd + 0x1d0;
        e.mem.set_u32(cache, 437);
        e.mem.set_u32(cache + 4, 9);
        let locale = text(&mut e, b"de");
        let name = e.call(SETLOCALE_SET_CAT, &args![info, locale, 2u32]).u32();
        assert_eq!(e.mem.cstr(name), b"German_Germany.1252".to_vec());
        assert_eq!(e.mem.u32(info + 0x68), name);
        assert_eq!(e.mem.u32(info + 0x70), name - 4);
        assert_eq!(e.mem.u32(name - 4), 1);
        assert_eq!(e.mem.u32(info + 4), 1252);
        assert_eq!(e.mem.u32(info + 0xc + 8), 0x407);
        assert_eq!(e.mem.u16(info + 8 * 6), 0x407);
        // A cache miss: the old front moved down, the new page "C-like" at the front.
        assert_eq!(e.mem.u32(cache), 1252);
        assert_eq!(e.mem.u32(cache + 4), 1);
        assert_eq!(e.mem.u32(cache + 8), 437);
        assert_eq!(e.mem.u32(cache + 0xc), 9);
        assert_eq!(e.mem.u32(info + 0xa8), 1);
        assert_eq!(calls(&e, callback), vec![vec![info]]);
        assert_eq!(
            calls(&e, CRT_GET_STRING_TYPE_A)[0][..4],
            [0, 1, ASCII_CHARS, 0x7f]
        );
        // The same name again changes nothing.
        let again = setlocale_set_cat(&mut e, info, locale, 2);
        assert_eq!(again, name);
        assert_eq!(calls(&e, callback).len(), 1);
    }

    #[test]
    fn setlocale_set_cat_finds_a_cached_code_page() {
        let (mut e, ptd, info) = set_cat_engine();
        let callback = 0x0040_1000;
        e.register_double(callback, |_, _| 0u32.into_ret());
        e.mem.set_u32(CATEGORY_TABLE + 2 * 0xc, callback);
        let cache = ptd + 0x1d0;
        for (i, page) in [437u32, 850, 1252, 932, 949].into_iter().enumerate() {
            e.mem.set_u32(cache + i as u32 * 8, page);
            e.mem.set_u32(cache + i as u32 * 8 + 4, i as u32 + 10);
        }
        let locale = text(&mut e, b"de");
        setlocale_set_cat(&mut e, info, locale, 2);
        // 1252 swapped to the front; the pairs before it moved down one place.
        let pairs: Vec<u32> = (0..10).map(|i| e.mem.u32(cache + i * 4)).collect();
        assert_eq!(pairs, vec![1252, 12, 437, 10, 850, 11, 932, 13, 949, 14]);
        assert_eq!(e.mem.u32(info + 0xa8), 12);
    }

    #[test]
    fn setlocale_set_cat_restores_on_a_failed_callback() {
        let (mut e, _, info) = set_cat_engine();
        let callback = 0x0040_1000;
        e.register_double(callback, |_, _| 1u32.into_ret());
        e.mem.set_u32(CATEGORY_TABLE + 0xc, callback);
        e.mem.set_u32(info + 4, 850);
        e.mem.set_u32(info + 0xc + 4, 0x409);
        let old_name = e.mem.u32(info + 0x58);
        let locale = text(&mut e, b"de");
        assert_eq!(setlocale_set_cat(&mut e, info, locale, 1), 0);
        assert_eq!(e.mem.u32(info + 0x58), old_name);
        assert_eq!(e.mem.u32(info + 0xc + 4), 0x409);
        assert_eq!(e.mem.u32(info + 4), 850);
        // The collation code page was set before the callback and stays.
        assert_eq!(e.mem.u32(info + 8), 1252);
        assert_eq!(calls(&e, FREE_FN).len(), 1);
    }

    #[test]
    fn setlocale_nolock_routes_each_form() {
        let (mut e, _) = engine();
        put(&mut e, CATEGORY_SEPARATORS, b"=;");
        put(&mut e, SEMICOLON, b";");
        let info = info_with(&mut e, [b"C"; 5]);
        e.register_double(SETLOCALE_SET_CAT, |e, a| {
            if e.mem.cstr(a[1]) == b"bad".to_vec() {
                0u32.into_ret()
            } else {
                0x77u32.into_ret()
            }
        });
        e.register_double(SETLOCALE_GET_ALL, |_, _| 0x99u32.into_ret());
        e.register_double(STRPBRK, |e, a| {
            let mut at = a[0];
            loop {
                let c = e.mem.u8(at);
                if c == 0 {
                    return 0u32.into_ret();
                }
                if c == b'=' || c == b';' {
                    return at.into_ret();
                }
                at += 1;
            }
        });
        let de = text(&mut e, b"de");
        // One category: set, or named.
        assert_eq!(e.call(0x00ed_0153, &args![info, de, 3u32]).u32(), 0x77);
        assert_eq!(setlocale_nolock(&mut e, info, 0, 5), e.mem.u32(info + 0x98));
        assert_eq!(setlocale_nolock(&mut e, info, 0, 0), 0x99);
        // "LC_..=..;": each named category set, then the combined name.
        let list = text(&mut e, b"LC_CTYPE=German;LC_TIME=French");
        assert_eq!(setlocale_nolock(&mut e, info, list, 0), 0x99);
        let sets = calls(&e, SETLOCALE_SET_CAT);
        assert_eq!(sets.len(), 3);
        assert_eq!((sets[1][0], sets[1][2]), (info, 2));
        assert_eq!((sets[2][0], sets[2][2]), (info, 5));
        let broken = text(&mut e, b"LC_CTYPE");
        assert_eq!(setlocale_nolock(&mut e, info, broken, 0), 0);
        // A plain text: expanded and set where the names differ.
        e.register_double(EXPANDLOCALE, |e, a| {
            let name = e.mem.cstr(a[0]);
            e.mem.set_cstr(a[1], &name);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(setlocale_nolock(&mut e, info, de, 0), 0x99);
        assert_eq!(calls(&e, SETLOCALE_SET_CAT).len(), 5);
        let c = text(&mut e, b"C");
        e.call_log = Some(vec![]);
        assert_eq!(setlocale_nolock(&mut e, info, c, 0), 0x99);
        assert!(calls(&e, SETLOCALE_SET_CAT).is_empty());
        let bad = text(&mut e, b"bad");
        assert_eq!(setlocale_nolock(&mut e, info, bad, 0), 0);
    }

    #[test]
    fn init_time_takes_a_new_block_or_the_c_one() {
        let (mut e, _) = engine();
        e.register_double(CALLOC_CRT, |e, a| e.mem.alloc(a[0] * a[1]).into_ret());
        e.register_double(GET_LC_TIME, |_, _| 0u32.into_ret());
        e.register_double(INTERLOCKED_DECREMENT, |_, _| 0u32.into_ret());
        e.mem.map(C_LC_TIME, 4);
        let info = e.mem.alloc(0xd8);
        e.mem.set_u32(info + 0xd4, C_LC_TIME);
        assert_eq!(e.call(0x00ee_045e, &args![info]).u32(), 0);
        assert_eq!(e.mem.u32(info + 0xd4), C_LC_TIME);
        e.mem.set_u32(info + 0x20, 0x407);
        assert_eq!(init_time(&mut e, info), 0);
        let block = e.mem.u32(info + 0xd4);
        assert_eq!(e.mem.u32(block + 0xb4), 1);
        assert!(calls(&e, INTERLOCKED_DECREMENT).is_empty());
        // A second one releases the first; a failed read frees the new block.
        assert_eq!(init_time(&mut e, info), 0);
        assert_eq!(calls(&e, INTERLOCKED_DECREMENT), vec![vec![block + 0xb4]]);
        e.register_double(GET_LC_TIME, |_, _| 1u32.into_ret());
        e.register_double(FREE_LC_TIME, |_, _| Ret::default());
        assert_eq!(init_time(&mut e, info), 1);
        assert_eq!(calls(&e, FREE_LC_TIME).len(), 1);
    }

    #[test]
    fn init_numeric_reads_the_separators_and_the_grouping() {
        let (mut e, _) = engine();
        e.register_double(CALLOC_CRT, |e, a| e.mem.alloc(a[0] * a[1]).into_ret());
        e.register_double(INTERLOCKED_DECREMENT, |_, _| 0u32.into_ret());
        e.mem.map(C_LCONV, 0x30);
        let info = e.mem.alloc(0xd8);
        let current = e.mem.alloc(0x30);
        e.mem.set_u32(current + 0xc, 0x77);
        e.mem.set_u32(info + 0xbc, current);
        // No handles: the C lconv.
        assert_eq!(e.call(0x00ee_051c, &args![info]).u32(), 0);
        assert_eq!(e.mem.u32(info + 0xbc), C_LCONV);
        assert_eq!(e.mem.u32(info + 0xb0), 0);
        // A numeric locale: the three fields read, the grouping turned into numbers.
        e.mem.set_u32(info + 0xbc, current);
        e.mem.set_u32(info + 0x1c, 0x407);
        e.mem.set_u16(info + 0x3e, 0x407);
        e.register_double(GETLOCALEINFO, |e, a| {
            let text: &[u8] = match a[3] {
                0xe => b",",
                0xf => b".",
                _ => b"3;2;0",
            };
            let at = e.mem.alloc(8);
            e.mem.set_cstr(at, text);
            e.mem.set_u32(a[4], at);
            0u32.into_ret()
        });
        let old_count = e.mem.alloc(4);
        e.mem.set_u32(info + 0xb0, old_count);
        assert_eq!(init_numeric(&mut e, info), 0);
        let lconv = e.mem.u32(info + 0xbc);
        assert_eq!(e.mem.cstr(e.mem.u32(lconv)), b",".to_vec());
        assert_eq!(e.mem.bytes(e.mem.u32(lconv + 8), 4), vec![3, 2, 0, 0]);
        assert_eq!(e.mem.u32(lconv + 0xc), 0x77);
        assert_eq!(e.mem.u32(e.mem.u32(info + 0xb0)), 1);
        assert_eq!(e.mem.u32(e.mem.u32(info + 0xb4)), 1);
        assert_eq!(calls(&e, FREE_FN), vec![vec![old_count], vec![current]]);
        // A failed read frees everything and answers -1.
        e.register_double(GETLOCALEINFO, |_, _| 1u32.into_ret());
        e.register_double(FREE_LCONV_NUM, |_, _| Ret::default());
        assert_eq!(init_numeric(&mut e, info), 0xffff_ffff);
        assert_eq!(calls(&e, FREE_LCONV_NUM).len(), 1);
    }
}
