//! `___get_qualified_locale` (`00ee0ff8`) and its helpers (MSVC 2008 LIBCMT, statically
//! linked into FalloutNV.exe 1.4.0.525), translated from the exe: `TranslateName`
//! (`00ee0a3d`), `ProcessCodePage` (`00ee0a9f`), `TestDefaultCountry` (`00ee0b43`),
//! `LcidFromHexString` (`00ee0b67`), `GetPrimaryLen` (`00ee0b9b`), the `EnumSystemLocalesA`
//! callbacks `CountryEnumProc` (`00ee0bb6`), `LangCountryEnumProc` (`00ee0cc2`) and
//! `LanguageEnumProc` (`00ee0e94`), `TestDefaultLanguage` (`00ee0c4e`),
//! `GetLcidFromLangCountry` (`00ee0f55`), `GetLcidFromLanguage` (`00ee0fbc`), and the
//! number-to-text pair `xtoa_s` (`00ed0e10`) and `_itoa_s` (`00ed0eed`).
//!
//! The search state is the thread's `_setloc_struct` (`_getptd` + 0x9c): the language text
//! (+0x0), the country text (+0x4), the flags (+0x8: 0x4 found, 0x1/0x2 partial country
//! matches, 0x100/0x200 language and country agree), the primary length (+0xc), whether
//! the language and country are three-letter abbreviations (+0x10, +0x14), and the
//! language and country `LCID`s (+0x18, +0x1c).

use super::*;

/// `_getptd` (`00ed11de`), `errno` (`00ec85e3`), the invalid-parameter report (`00ec7c30`),
/// `__invoke_watson` (`00ec7b08`), `strcpy_s` (`00ec65a6`), `strcmp` (`00ec6da0`),
/// `strlen` (`00ec6130`), `_stricmp` (`00ec68e4`), `_strnicmp` (`00ec7ec0`) and `atol`'s
/// body (`00eca6bd`).
const GETPTD: u32 = 0x00ed_11de;
const ERRNO: u32 = 0x00ec_85e3;
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
const INVOKE_WATSON: u32 = 0x00ec_7b08;
const STRCPY_S: u32 = 0x00ec_65a6;
const STRCMP: u32 = 0x00ec_6da0;
const STRLEN: u32 = 0x00ec_6130;
const STRICMP: u32 = 0x00ec_68e4;
const STRNICMP: u32 = 0x00ec_7ec0;
const ATOL: u32 = 0x00ec_a6bd;
/// The import slots of KERNEL32 `GetLocaleInfoA` (`00fdf268`), `GetACP` (`00fdf1e4`),
/// `EnumSystemLocalesA` (`00fdf25c`), `GetUserDefaultLCID` (`00fdf264`), `IsValidCodePage`
/// (`00fdf1f0`) and `IsValidLocale` (`00fdf238`).
const GET_LOCALE_INFO_A: u32 = 0x00fd_f268;
const GET_ACP: u32 = 0x00fd_f1e4;
const ENUM_SYSTEM_LOCALES_A: u32 = 0x00fd_f25c;
const GET_USER_DEFAULT_LCID: u32 = 0x00fd_f264;
const IS_VALID_CODE_PAGE: u32 = 0x00fd_f1f0;
const IS_VALID_LOCALE: u32 = 0x00fd_f238;
/// The functions of this file that call each other.
const TRANSLATE_NAME: u32 = 0x00ee_0a3d;
const PROCESS_CODE_PAGE: u32 = 0x00ee_0a9f;
const TEST_DEFAULT_COUNTRY: u32 = 0x00ee_0b43;
const LCID_FROM_HEX_STRING: u32 = 0x00ee_0b67;
const GET_PRIMARY_LEN: u32 = 0x00ee_0b9b;
const COUNTRY_ENUM_PROC: u32 = 0x00ee_0bb6;
const TEST_DEFAULT_LANGUAGE: u32 = 0x00ee_0c4e;
const LANG_COUNTRY_ENUM_PROC: u32 = 0x00ee_0cc2;
const LANGUAGE_ENUM_PROC: u32 = 0x00ee_0e94;
const GET_LCID_FROM_LANG_COUNTRY: u32 = 0x00ee_0f55;
const GET_LCID_FROM_LANGUAGE: u32 = 0x00ee_0fbc;
const XTOA_S: u32 = 0x00ed_0e10;
const ITOA_S: u32 = 0x00ed_0eed;
/// The country-name table (`010fe810`, highest index 0x16) and language-name table
/// (`010fe608`, highest index 0x40), eight bytes per entry: the name, then a four-byte
/// abbreviation; the ten default-country `LANGID`s (`010fe8c8`); the texts `"ACP"`
/// (`010fe8e0`), `"OCP"` (`010fe8dc`), Norwegian-Nynorsk's language name (`010fe8e4`) and
/// the code-page text `ProcessCodePage` compares with (`01020760`).
const COUNTRY_TABLE: u32 = 0x010f_e810;
const LANGUAGE_TABLE: u32 = 0x010f_e608;
const DEFAULT_COUNTRIES: u32 = 0x010f_e8c8;
const ACP_TEXT: u32 = 0x010f_e8e0;
const OCP_TEXT: u32 = 0x010f_e8dc;
const NYNORSK_TEXT: u32 = 0x010f_e8e4;
const ZERO_CODE_PAGE_TEXT: u32 = 0x0102_0760;

fn setloc(e: &mut Engine) -> u32 {
    e.call(GETPTD, &args![]).u32().wrapping_add(0x9c)
}

fn get_locale_info(e: &mut Engine, lcid: u32, kind: u32, buffer: u32, size: u32) -> u32 {
    e.call(GET_LOCALE_INFO_A, &args![lcid, kind, buffer, size])
        .u32()
}

fn report(e: &mut Engine, code: u32) -> u32 {
    let slot = e.call(ERRNO, &args![]).u32();
    e.mem.set_u32(slot, code);
    e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
    code
}

// Translated from 00ed0e10 (disassembled, FalloutNV.exe 1.4.0.525)
/// `xtoa_s` (Ghidra's function ID), `EAX` the value, `ECX` the target, then (callee-popped)
/// the size, the radix and whether the value is negative. A null target or a size of 0
/// sets `errno` to `EINVAL`, reports and answers it. The target is emptied; a size not
/// above the room for a sign and one digit is `ERANGE`, a radix outside 2..36 `EINVAL`.
/// A negative value gets `-` and is negated; the digits (`a`.. above 9) are written lowest
/// first while the value stays above 0 and the count stays below the size, then reversed.
/// Running out of room empties the target again and is `ERANGE`; otherwise 0.
pub fn xtoa_s(
    e: &mut Engine,
    value: u32,
    target: u32,
    size: u32,
    radix: u32,
    negative: u32,
) -> u32 {
    if target == 0 || size == 0 {
        return report(e, 0x16);
    }
    e.mem.set_u8(target, 0);
    let need = u32::from(negative != 0) + 1;
    if size <= need {
        return report(e, 0x22);
    }
    if radix.wrapping_sub(2) > 0x22 {
        return report(e, 0x16);
    }
    let mut count = 0u32;
    let mut at = target;
    let mut value = value;
    if negative != 0 {
        e.mem.set_u8(target, b'-');
        at = target.wrapping_add(1);
        count = 1;
        value = value.wrapping_neg();
    }
    let mut first = at;
    loop {
        let digit = value % radix;
        value /= radix;
        let byte = if digit > 9 {
            digit as u8 + 0x57
        } else {
            digit as u8 + 0x30
        };
        e.mem.set_u8(at, byte);
        at = at.wrapping_add(1);
        count += 1;
        if value == 0 || count >= size {
            break;
        }
    }
    if count >= size {
        e.mem.set_u8(target, 0);
        return report(e, 0x22);
    }
    e.mem.set_u8(at, 0);
    let mut last = at.wrapping_sub(1);
    loop {
        let low = e.mem.u8(first);
        let high = e.mem.u8(last);
        e.mem.set_u8(last, low);
        last = last.wrapping_sub(1);
        e.mem.set_u8(first, high);
        first = first.wrapping_add(1);
        if first >= last {
            break;
        }
    }
    0
}

// Translated from 00ed0eed (disassembled, FalloutNV.exe 1.4.0.525)
/// `_itoa_s` (Ghidra's function ID), cdecl: value, target, size, radix. A negative value in
/// radix 10 is written with its sign (`xtoa_s` with the flag set); everything else as
/// unsigned.
pub fn itoa_s(e: &mut Engine, value: u32, target: u32, size: u32, radix: u32) -> u32 {
    let (radix, negative) = if radix == 10 && (value as i32) < 0 {
        (10u32, 1u32)
    } else {
        (radix, 0u32)
    };
    e.call(XTOA_S, &args![value, target, size, radix, negative])
        .u32()
}

// Translated from 00ee0a3d (disassembled, FalloutNV.exe 1.4.0.525)
/// `TranslateName` (Ghidra's function ID), cdecl: a sorted table of (name, four-byte
/// abbreviation) entries, its highest index, and the address of a text pointer. A binary
/// search (`_stricmp` of the text against each middle name; signed halving) that, on a
/// match, points the text at the entry's abbreviation and answers 1; no match answers 0.
pub fn translate_name(e: &mut Engine, table: u32, high: u32, name: u32) -> u32 {
    let mut result = 1i32;
    let mut low = 0i32;
    let mut high = high as i32;
    if high < 0 {
        return 0;
    }
    while result != 0 && low <= high {
        let middle = (high.wrapping_add(low)) / 2;
        let entry = table.wrapping_add((middle as u32).wrapping_mul(8));
        let text = e.mem.u32(name);
        let label = e.mem.u32(entry);
        result = e.call(STRICMP, &args![text, label]).i32();
        if result == 0 {
            e.mem.set_u32(name, entry.wrapping_add(4));
        } else if result < 0 {
            high = middle - 1;
        } else {
            low = middle + 1;
        }
    }
    u32::from(result == 0)
}

// Translated from 00ee0a9f (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee0a9f - the /GS stack cookie check (00ec408c) is not translated
/// `ProcessCodePage` (Ghidra's function ID), `ECX` the code-page text, `EDI` the search state.
/// No text, an empty one or `"ACP"`: the country `LCID`'s default ANSI code page
/// (`GetLocaleInfoA` 0x1004 into eight bytes; failing answers 0), or `GetACP` when that
/// text equals `01020760`'s. `"OCP"`: the default OEM code page (0xb). Each text is
/// converted with `atol`'s body.
pub fn process_code_page(e: &mut Engine, text: u32, state: u32) -> u32 {
    e.with_stack(8, |e, buffer| {
        let buffer = buffer.addr();
        let country = e.mem.u32(state.wrapping_add(0x1c));
        let ansi =
            text == 0 || e.mem.u8(text) == 0 || e.call(STRCMP, &args![text, ACP_TEXT]).u32() == 0;
        let number = if ansi {
            if get_locale_info(e, country, 0x1004, buffer, 8) == 0 {
                return 0;
            }
            if e.call(STRCMP, &args![buffer, ZERO_CODE_PAGE_TEXT]).u32() == 0 {
                return e.call(GET_ACP, &args![]).u32();
            }
            buffer
        } else if e.call(STRCMP, &args![text, OCP_TEXT]).u32() == 0 {
            if get_locale_info(e, country, 0xb, buffer, 8) == 0 {
                return 0;
            }
            buffer
        } else {
            text
        };
        e.call(ATOL, &args![number]).u32()
    })
}

// Translated from 00ee0b43 (disassembled, FalloutNV.exe 1.4.0.525)
/// `TestDefaultCountry` (Ghidra's function ID), cdecl, an `LCID`: 0 when its low word is one
/// of the ten default-country `LANGID`s (`010fe8c8`), 1 otherwise.
pub fn test_default_country(e: &mut Engine, lcid: u32) -> u32 {
    let word = lcid as u16;
    for i in 0..10u32 {
        if e.mem.u16(DEFAULT_COUNTRIES.wrapping_add(i * 2)) == word {
            return 0;
        }
    }
    1
}

// Translated from 00ee0b67 (disassembled, FalloutNV.exe 1.4.0.525)
/// `LcidFromHexString` (Ghidra's function ID), `EDX` the text: its hexadecimal digits
/// (`a`-`f` and `A`-`F` moved next to the decimal ones, every byte otherwise taken as a
/// digit, sign-extended, less `'0'`) accumulated four bits at a time.
pub fn lcid_from_hex_string(e: &mut Engine, text: u32) -> u32 {
    let mut value = 0u32;
    let mut at = text;
    loop {
        let mut byte = e.mem.u8(at);
        if byte == 0 {
            return value;
        }
        at = at.wrapping_add(1);
        if byte.wrapping_sub(0x61) <= 5 {
            byte = byte.wrapping_add(0xd9);
        } else if byte.wrapping_sub(0x41) <= 5 {
            byte = byte.wrapping_add(0xf9);
        }
        let digit = byte as i8 as i32 as u32;
        value = value.wrapping_shl(4).wrapping_add(digit).wrapping_sub(0x30);
    }
}

// Translated from 00ee0b9b (disassembled, FalloutNV.exe 1.4.0.525)
/// `GetPrimaryLen` (Ghidra's function ID), `EDX` the text: the number of leading ASCII
/// letters.
pub fn get_primary_len(e: &mut Engine, text: u32) -> u32 {
    let mut count = 0u32;
    loop {
        let byte = e.mem.u8(text.wrapping_add(count));
        let upper = (byte as i8) >= 0x41 && (byte as i8) <= 0x5a;
        if !upper && byte.wrapping_sub(0x61) > 0x19 {
            return count;
        }
        count += 1;
    }
}

/// The `LCTYPE` for a name: `abbreviated` (a set word) picks the abbreviation (`NEG`,
/// `SBB`, `AND`, `ADD` in the exe).
fn name_kind(abbreviated: u32, short: u32, long: u32) -> u32 {
    if abbreviated != 0 {
        short
    } else {
        long
    }
}

/// The callbacks' answer: keep enumerating while the "found" flag (0x4) is clear.
fn keep_going(e: &mut Engine, state: u32) -> u32 {
    !(e.mem.u32(state.wrapping_add(8)) >> 2) & 1
}

// Translated from 00ee0bb6 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee0bb6 - the /GS stack cookie check (00ec408c) is not translated
/// `CountryEnumProc` (Ghidra's function ID), stdcall, a locale's hexadecimal `LCID` text.
/// Its country name (abbreviated (7) or English (0x1002) as the country text is, 0x78
/// bytes) failing clears the flags and answers 1. A name equal to the country text
/// (`_stricmp`) for a country that is not a default one sets "found" and both `LCID`s.
pub fn country_enum_proc(e: &mut Engine, text: u32) -> u32 {
    let state = setloc(e);
    let lcid = e.call(LCID_FROM_HEX_STRING, &args![text]).u32();
    e.with_stack(0x78, |e, buffer| {
        let buffer = buffer.addr();
        let kind = name_kind(e.mem.u32(state.wrapping_add(0x14)), 7, 0x1002);
        if get_locale_info(e, lcid, kind, buffer, 0x78) == 0 {
            e.mem.set_u32(state.wrapping_add(8), 0);
            return 1;
        }
        let country = e.mem.u32(state.wrapping_add(4));
        if e.call(STRICMP, &args![country, buffer]).u32() == 0
            && e.call(TEST_DEFAULT_COUNTRY, &args![lcid]).u32() != 0
        {
            let flags = e.mem.u32(state.wrapping_add(8));
            e.mem.set_u32(state.wrapping_add(8), flags | 4);
            e.mem.set_u32(state.wrapping_add(0x1c), lcid);
            e.mem.set_u32(state.wrapping_add(0x18), lcid);
        }
        keep_going(e, state)
    })
}

// Translated from 00ee0c4e (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee0c4e - the /GS stack cookie check (00ec408c) is not translated
/// `TestDefaultLanguage` (Ghidra's function ID), `ECX` the search state, then an `LCID` and
/// whether to check the primary length. The primary language's `LANGID` text
/// (`GetLocaleInfoA` of the `LCID`'s primary language with sublanguage 1, `LOCALE_ILANGUAGE`;
/// failing answers 0) naming this `LCID` answers 1; so does no check. Otherwise 0 when the
/// language text is all letters (`GetPrimaryLen` equal to `strlen`), 1 when not.
pub fn test_default_language(e: &mut Engine, state: u32, lcid: u32, check_primary: u32) -> u32 {
    e.with_stack(0x78, |e, buffer| {
        let buffer = buffer.addr();
        let primary = (lcid & 0x3ff) | 0x400;
        if get_locale_info(e, primary, 1, buffer, 0x78) == 0 {
            return 0;
        }
        let named = e.call(LCID_FROM_HEX_STRING, &args![buffer]).u32();
        if lcid == named || check_primary == 0 {
            return 1;
        }
        let language = e.mem.u32(state);
        let letters = e.call(GET_PRIMARY_LEN, &args![language]).u32();
        let len = e.call(STRLEN, &args![language]).u32();
        u32::from(letters != len)
    })
}

// Translated from 00ee0cc2 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee0cc2 - the /GS stack cookie check (00ec408c) is not translated
/// `LangCountryEnumProc` (Ghidra's function ID), stdcall, a locale's hexadecimal `LCID` text.
/// A failing `GetLocaleInfoA` anywhere clears the flags and answers 1. When the locale's
/// country name matches: a matching language name sets 0x304 and both `LCID`s; otherwise,
/// without flag 0x2, a primary-length match (`_strnicmp`) sets 0x2 and the country `LCID`
/// (and the language one when the text is exactly that long); otherwise, without flag 0x1,
/// a non-default country sets 0x1 and the country `LCID`. Then, unless 0x300 are both
/// set, a matching language name sets 0x200 and (for an abbreviation, or after
/// `TestDefaultLanguage` agrees for an all-letter text or one with no primary length) 0x100,
/// with the language `LCID` when none is set yet; a non-matching one whose text is not an
/// abbreviation and has a primary length asks `_stricmp` again and `TestDefaultLanguage`
/// without the primary check before doing the same.
pub fn lang_country_enum_proc(e: &mut Engine, text: u32) -> u32 {
    let state = setloc(e);
    let lcid = e.call(LCID_FROM_HEX_STRING, &args![text]).u32();
    let flags_at = state.wrapping_add(8);
    e.with_stack(0x78, |e, buffer| {
        let buffer = buffer.addr();
        let fail = |e: &mut Engine| {
            e.mem.set_u32(flags_at, 0);
            1u32
        };
        let kind = name_kind(e.mem.u32(state.wrapping_add(0x14)), 7, 0x1002);
        if get_locale_info(e, lcid, kind, buffer, 0x78) == 0 {
            return fail(e);
        }
        let country = e.mem.u32(state.wrapping_add(4));
        if e.call(STRICMP, &args![country, buffer]).u32() == 0 {
            let kind = name_kind(e.mem.u32(state.wrapping_add(0x10)), 3, 0x1001);
            if get_locale_info(e, lcid, kind, buffer, 0x78) == 0 {
                return fail(e);
            }
            let language = e.mem.u32(state);
            if e.call(STRICMP, &args![language, buffer]).u32() == 0 {
                let flags = e.mem.u32(flags_at);
                e.mem.set_u32(flags_at, flags | 0x304);
                e.mem.set_u32(state.wrapping_add(0x18), lcid);
                e.mem.set_u32(state.wrapping_add(0x1c), lcid);
            } else if e.mem.u8(flags_at) & 2 == 0 {
                let primary = e.mem.u32(state.wrapping_add(0xc));
                let partial =
                    primary != 0 && e.call(STRNICMP, &args![language, buffer, primary]).u32() == 0;
                if partial {
                    let flags = e.mem.u32(flags_at);
                    e.mem.set_u32(flags_at, flags | 2);
                    e.mem.set_u32(state.wrapping_add(0x1c), lcid);
                    let language = e.mem.u32(state);
                    if e.call(STRLEN, &args![language]).u32() == e.mem.u32(state.wrapping_add(0xc))
                    {
                        e.mem.set_u32(state.wrapping_add(0x18), lcid);
                    }
                } else {
                    let flags = e.mem.u32(flags_at);
                    if flags & 1 == 0 && e.call(TEST_DEFAULT_COUNTRY, &args![lcid]).u32() != 0 {
                        e.mem.set_u32(flags_at, flags | 1);
                        e.mem.set_u32(state.wrapping_add(0x1c), lcid);
                    }
                }
            }
        }
        if e.mem.u32(flags_at) & 0x300 == 0x300 {
            return keep_going(e, state);
        }
        let abbreviated = e.mem.u32(state.wrapping_add(0x10));
        let kind = name_kind(abbreviated, 3, 0x1001);
        if get_locale_info(e, lcid, kind, buffer, 0x78) == 0 {
            return fail(e);
        }
        let language = e.mem.u32(state);
        let agrees = if e.call(STRICMP, &args![language, buffer]).u32() == 0 {
            let flags = e.mem.u32(flags_at) | 0x200;
            e.mem.set_u32(flags_at, flags);
            if e.mem.u32(state.wrapping_add(0x10)) != 0 {
                true
            } else {
                let primary = e.mem.u32(state.wrapping_add(0xc));
                if primary == 0 {
                    true
                } else {
                    let language = e.mem.u32(state);
                    if e.call(STRLEN, &args![language]).u32() != e.mem.u32(state.wrapping_add(0xc))
                    {
                        true
                    } else {
                        e.call(TEST_DEFAULT_LANGUAGE, &args![state, lcid, 1u32])
                            .u32()
                            != 0
                    }
                }
            }
        } else {
            if e.mem.u32(state.wrapping_add(0x10)) != 0 || e.mem.u32(state.wrapping_add(0xc)) == 0 {
                return keep_going(e, state);
            }
            let language = e.mem.u32(state);
            if e.call(STRICMP, &args![language, buffer]).u32() != 0 {
                return keep_going(e, state);
            }
            e.call(TEST_DEFAULT_LANGUAGE, &args![state, lcid, 0u32])
                .u32()
                != 0
        };
        if agrees {
            let flags = e.mem.u32(flags_at);
            e.mem.set_u32(flags_at, flags | 0x100);
            if e.mem.u32(state.wrapping_add(0x18)) == 0 {
                e.mem.set_u32(state.wrapping_add(0x18), lcid);
            }
        }
        keep_going(e, state)
    })
}

// Translated from 00ee0e94 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee0e94 - the /GS stack cookie check (00ec408c) is not translated
/// `LanguageEnumProc` (Ghidra's function ID), stdcall, a locale's hexadecimal `LCID` text.
/// Its language name (abbreviated (3) or English (0x1001)) failing clears the flags and
/// answers 1. A matching name sets "found" and both `LCID`s directly for an abbreviation,
/// or when `TestDefaultLanguage` (with the primary check) agrees. A non-matching name, for
/// a text that is not an abbreviation and has a primary length, is compared again
/// (`_stricmp`) and, matching, tried with `TestDefaultLanguage` without the check.
pub fn language_enum_proc(e: &mut Engine, text: u32) -> u32 {
    let state = setloc(e);
    let lcid = e.call(LCID_FROM_HEX_STRING, &args![text]).u32();
    e.with_stack(0x78, |e, buffer| {
        let buffer = buffer.addr();
        let abbreviated = e.mem.u32(state.wrapping_add(0x10));
        let kind = name_kind(abbreviated, 3, 0x1001);
        if get_locale_info(e, lcid, kind, buffer, 0x78) == 0 {
            e.mem.set_u32(state.wrapping_add(8), 0);
            return 1;
        }
        let language = e.mem.u32(state);
        let found = if e.call(STRICMP, &args![language, buffer]).u32() == 0 {
            e.mem.u32(state.wrapping_add(0x10)) != 0
                || e.call(TEST_DEFAULT_LANGUAGE, &args![state, lcid, 1u32])
                    .u32()
                    != 0
        } else {
            e.mem.u32(state.wrapping_add(0x10)) == 0
                && e.mem.u32(state.wrapping_add(0xc)) != 0
                && {
                    let language = e.mem.u32(state);
                    e.call(STRICMP, &args![language, buffer]).u32() == 0
                }
                && e.call(TEST_DEFAULT_LANGUAGE, &args![state, lcid, 0u32])
                    .u32()
                    != 0
        };
        if found {
            let flags = e.mem.u32(state.wrapping_add(8));
            e.mem.set_u32(state.wrapping_add(8), flags | 4);
            e.mem.set_u32(state.wrapping_add(0x18), lcid);
            e.mem.set_u32(state.wrapping_add(0x1c), lcid);
        }
        keep_going(e, state)
    })
}

// Translated from 00ee0f55 (disassembled, FalloutNV.exe 1.4.0.525)
/// `GetLcidFromLangCountry` (Ghidra's function ID), `ESI` the search state: the language and
/// country abbreviation words (`strlen` 3), no `LCID` yet, the primary length (2 for an
/// abbreviation, else `GetPrimaryLen`), then `EnumSystemLocalesA` with
/// `LangCountryEnumProc` (`LCID_INSTALLED`). Unless 0x100, 0x200 and one of 0x7 are all
/// set, the flags are cleared.
pub fn get_lcid_from_lang_country(e: &mut Engine, state: u32) {
    let language = e.mem.u32(state);
    let three = u32::from(e.call(STRLEN, &args![language]).u32() == 3);
    e.mem.set_u32(state.wrapping_add(0x10), three);
    let country = e.mem.u32(state.wrapping_add(4));
    let three = u32::from(e.call(STRLEN, &args![country]).u32() == 3);
    e.mem.set_u32(state.wrapping_add(0x18), 0);
    e.mem.set_u32(state.wrapping_add(0x14), three);
    let primary = if e.mem.u32(state.wrapping_add(0x10)) != 0 {
        2
    } else {
        let language = e.mem.u32(state);
        e.call(GET_PRIMARY_LEN, &args![language]).u32()
    };
    e.mem.set_u32(state.wrapping_add(0xc), primary);
    e.call(ENUM_SYSTEM_LOCALES_A, &args![LANG_COUNTRY_ENUM_PROC, 1u32]);
    let flags = e.mem.u32(state.wrapping_add(8));
    if !(flags & 0x100 != 0 && flags & 0x200 != 0 && flags & 7 != 0) {
        e.mem.set_u32(state.wrapping_add(8), 0);
    }
}

// Translated from 00ee0fbc (disassembled, FalloutNV.exe 1.4.0.525)
/// `GetLcidFromLanguage` (Ghidra's function ID), `ESI` the search state: the language
/// abbreviation word, the primary length as above, `EnumSystemLocalesA` with
/// `LanguageEnumProc`; without "found" the flags are cleared.
pub fn get_lcid_from_language(e: &mut Engine, state: u32) {
    let language = e.mem.u32(state);
    let three = u32::from(e.call(STRLEN, &args![language]).u32() == 3);
    e.mem.set_u32(state.wrapping_add(0x10), three);
    let primary = if three != 0 {
        2
    } else {
        let language = e.mem.u32(state);
        e.call(GET_PRIMARY_LEN, &args![language]).u32()
    };
    e.mem.set_u32(state.wrapping_add(0xc), primary);
    e.call(ENUM_SYSTEM_LOCALES_A, &args![LANGUAGE_ENUM_PROC, 1u32]);
    if e.mem.u8(state.wrapping_add(8)) & 4 == 0 {
        e.mem.set_u32(state.wrapping_add(8), 0);
    }
}

// Translated from 00ee0ff8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___get_qualified_locale` (Ghidra's function ID), cdecl: the input `LC_STRINGS` (may be
/// null), the `LC_ID` out (may be null), the output `LC_STRINGS` (may be null). Null input
/// takes the user default `LCID` for both (flags 0x104). Otherwise the country text is
/// first translated through the country table; a language looks up the `LCID`
/// (`GetLcidFromLangCountry` with a country, `GetLcidFromLanguage` without) and, not found,
/// is translated through the language table and looked up again; a country alone
/// enumerates with `CountryEnumProc`; neither takes the user default. Nothing found answers
/// 0. The code page (`ProcessCodePage` on the input's code-page text) must not be 0 or
/// UTF-7/UTF-8 and must be valid (`IsValidCodePage`), the language `LCID` valid
/// (`IsValidLocale`, `LCID_INSTALLED`), or 0 is answered. The `LC_ID` gets both `LCID`s'
/// low words and the code page; the output gets the English language name (Norwegian-
/// Nynorsk's for `LANGID` 0x814, read from the `LC_ID`'s first word), the English country
/// name and the code page in decimal (`_itoa_s`); a failing name answers 0. Otherwise 1.
pub fn get_qualified_locale(e: &mut Engine, input: u32, id_out: u32, output: u32) -> u32 {
    let state = setloc(e);
    let flags_at = state.wrapping_add(8);
    if input == 0 {
        let flags = e.mem.u32(flags_at);
        e.mem.set_u32(flags_at, flags | 0x104);
        user_default(e, state);
    } else {
        let country_slot = state.wrapping_add(4);
        e.mem.set_u32(state, input);
        let country = input.wrapping_add(0x40);
        e.mem.set_u32(country_slot, country);
        if country != 0 && e.mem.u8(country) != 0 {
            e.call(TRANSLATE_NAME, &args![COUNTRY_TABLE, 0x16u32, country_slot]);
        }
        let language = e.mem.u32(state);
        e.mem.set_u32(flags_at, 0);
        let has_country = |e: &mut Engine| {
            let country = e.mem.u32(country_slot);
            country != 0 && e.mem.u8(country) != 0
        };
        if language != 0 && e.mem.u8(language) != 0 {
            if has_country(e) {
                e.call(GET_LCID_FROM_LANG_COUNTRY, &args![state]);
            } else {
                e.call(GET_LCID_FROM_LANGUAGE, &args![state]);
            }
            if e.mem.u32(flags_at) == 0
                && e.call(TRANSLATE_NAME, &args![LANGUAGE_TABLE, 0x40u32, state])
                    .u32()
                    != 0
            {
                if has_country(e) {
                    e.call(GET_LCID_FROM_LANG_COUNTRY, &args![state]);
                } else {
                    e.call(GET_LCID_FROM_LANGUAGE, &args![state]);
                }
            }
        } else if has_country(e) {
            let country = e.mem.u32(country_slot);
            let three = u32::from(e.call(STRLEN, &args![country]).u32() == 3);
            e.mem.set_u32(state.wrapping_add(0x14), three);
            e.call(ENUM_SYSTEM_LOCALES_A, &args![COUNTRY_ENUM_PROC, 1u32]);
            if e.mem.u8(flags_at) & 4 == 0 {
                e.mem.set_u32(flags_at, 0);
            }
        } else {
            e.mem.set_u32(flags_at, 0x104);
            user_default(e, state);
        }
    }
    if e.mem.u32(flags_at) == 0 {
        return 0;
    }
    let code_page_text = if input != 0 {
        input.wrapping_add(0x80)
    } else {
        0
    };
    let code_page = e
        .call(PROCESS_CODE_PAGE, &args![code_page_text, state])
        .u32();
    if code_page == 0 || code_page == 0xfde8 || code_page == 0xfde9 {
        return 0;
    }
    if e.call(IS_VALID_CODE_PAGE, &args![code_page & 0xffff]).u32() == 0 {
        return 0;
    }
    let language_lcid = e.mem.u32(state.wrapping_add(0x18));
    if e.call(IS_VALID_LOCALE, &args![language_lcid, 1u32]).u32() == 0 {
        return 0;
    }
    if id_out != 0 {
        let language = e.mem.u16(state.wrapping_add(0x18));
        e.mem.set_u16(id_out, language);
        let country = e.mem.u16(state.wrapping_add(0x1c));
        e.mem.set_u16(id_out.wrapping_add(2), country);
        e.mem.set_u16(id_out.wrapping_add(4), code_page as u16);
    }
    if output == 0 {
        return 1;
    }
    // The exe reads the LC_ID's first word here whether or not one was given.
    if e.mem.u16(id_out) == 0x814 {
        if e.call(STRCPY_S, &args![output, 0x40u32, NYNORSK_TEXT])
            .u32()
            != 0
        {
            e.call(INVOKE_WATSON, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        }
    } else {
        let language_lcid = e.mem.u32(state.wrapping_add(0x18));
        if get_locale_info(e, language_lcid, 0x1001, output, 0x40) == 0 {
            return 0;
        }
    }
    let country_lcid = e.mem.u32(state.wrapping_add(0x1c));
    if get_locale_info(e, country_lcid, 0x1002, output.wrapping_add(0x40), 0x40) == 0 {
        return 0;
    }
    e.call(
        ITOA_S,
        &args![code_page, output.wrapping_add(0x80), 0x10u32, 10u32],
    );
    1
}

/// `___get_qualified_locale`'s `00ee10dd`: both `LCID`s from `GetUserDefaultLCID`.
fn user_default(e: &mut Engine, state: u32) {
    let lcid = e.call(GET_USER_DEFAULT_LCID, &args![]).u32();
    e.mem.set_u32(state.wrapping_add(0x18), lcid);
    e.mem.set_u32(state.wrapping_add(0x1c), lcid);
}

/// The registry entries of the functions that take registers.
fn xtoa_s_entry(e: &mut Engine, words: &[u32]) -> Ret {
    xtoa_s(e, words[0], words[1], words[2], words[3], words[4]).into_ret()
}

fn process_code_page_entry(e: &mut Engine, words: &[u32]) -> Ret {
    process_code_page(e, words[0], words[1]).into_ret()
}

fn lcid_from_hex_string_entry(e: &mut Engine, words: &[u32]) -> Ret {
    lcid_from_hex_string(e, words[0]).into_ret()
}

fn get_primary_len_entry(e: &mut Engine, words: &[u32]) -> Ret {
    get_primary_len(e, words[0]).into_ret()
}

fn test_default_language_entry(e: &mut Engine, words: &[u32]) -> Ret {
    test_default_language(e, words[0], words[1], words[2]).into_ret()
}

fn get_lcid_from_lang_country_entry(e: &mut Engine, words: &[u32]) -> Ret {
    get_lcid_from_lang_country(e, words[0]);
    Ret::default()
}

fn get_lcid_from_language_entry(e: &mut Engine, words: &[u32]) -> Ret {
    get_lcid_from_language(e, words[0]);
    Ret::default()
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00ed_0e10, xtoa_s_entry as AbiFn),
        entry!(0x00ed_0eed, itoa_s(u32, u32, u32, u32) -> u32),
        entry!(0x00ee_0a3d, translate_name(u32, u32, u32) -> u32),
        (0x00ee_0a9f, process_code_page_entry as AbiFn),
        entry!(0x00ee_0b43, test_default_country(u32) -> u32),
        (0x00ee_0b67, lcid_from_hex_string_entry as AbiFn),
        (0x00ee_0b9b, get_primary_len_entry as AbiFn),
        entry!(0x00ee_0bb6, country_enum_proc(u32) -> u32),
        (0x00ee_0c4e, test_default_language_entry as AbiFn),
        entry!(0x00ee_0cc2, lang_country_enum_proc(u32) -> u32),
        entry!(0x00ee_0e94, language_enum_proc(u32) -> u32),
        (0x00ee_0f55, get_lcid_from_lang_country_entry as AbiFn),
        (0x00ee_0fbc, get_lcid_from_language_entry as AbiFn),
        entry!(0x00ee_0ff8, get_qualified_locale(u32, u32, u32) -> u32),
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

    fn put(e: &mut Engine, addr: u32, t: &[u8]) {
        e.mem.map(addr, t.len() as u32 + 1);
        e.mem.set_cstr(addr, t);
    }

    fn order(a: Vec<u8>, b: Vec<u8>) -> u32 {
        match a.cmp(&b) {
            std::cmp::Ordering::Less => 0xffff_ffff,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => 1,
        }
    }

    fn lower(t: Vec<u8>) -> Vec<u8> {
        t.to_ascii_lowercase()
    }

    /// What the doubled `GetLocaleInfoA` knows: (LCID, LCTYPE, text).
    const LOCALE_INFO: &[(u32, u32, &[u8])] = &[
        (0x409, 0x1001, b"English"),
        (0x409, 3, b"ENU"),
        (0x409, 0x1002, b"United States"),
        (0x409, 7, b"USA"),
        (0x409, 1, b"0409"),
        (0x409, 0x1004, b"1252"),
        (0x409, 0xb, b"437"),
        (0x407, 0x1001, b"German"),
        (0x407, 3, b"DEU"),
        (0x407, 0x1002, b"Germany"),
        (0x407, 7, b"DEU"),
        (0x407, 1, b"0407"),
        (0x407, 0x1004, b"1252"),
        (0x807, 0x1001, b"German"),
        (0x807, 3, b"DES"),
        (0x807, 0x1002, b"Switzerland"),
        (0x807, 7, b"CHE"),
        (0x807, 0x1004, b"1252"),
    ];

    /// An engine with the thread's data, the KERNEL32 locale imports, the string compares,
    /// `atol` and the report doubled; three installed locales (0409, 0407, 0807); the name
    /// tables with `american` (`ENU`) first and `britain` (`GBR`) first.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let ptd = e.mem.alloc(0x300);
        e.register_double(GETPTD, move |_, _| ptd.into_ret());
        e.register_double(GET_LOCALE_INFO_A, |e, a| {
            for &(lcid, kind, value) in LOCALE_INFO {
                if lcid == a[0] && kind == a[1] {
                    e.mem.set_cstr(a[2], value);
                    return (value.len() as u32 + 1).into_ret();
                }
            }
            0u32.into_ret()
        });
        e.register_double(ENUM_SYSTEM_LOCALES_A, |e, a| {
            for lcid in [b"00000409", b"00000407", b"00000807"] {
                let at = e.mem.alloc(9);
                e.mem.set_cstr(at, lcid);
                if e.call(a[0], &args![at]).u32() == 0 {
                    break;
                }
            }
            1u32.into_ret()
        });
        e.register_double(GET_USER_DEFAULT_LCID, |_, _| 0x409u32.into_ret());
        e.register_double(GET_ACP, |_, _| 1252u32.into_ret());
        e.register_double(IS_VALID_CODE_PAGE, |_, _| 1u32.into_ret());
        e.register_double(IS_VALID_LOCALE, |_, _| 1u32.into_ret());
        e.register_double(STRICMP, |e, a| {
            order(lower(e.mem.cstr(a[0])), lower(e.mem.cstr(a[1]))).into_ret()
        });
        e.register_double(STRNICMP, |e, a| {
            let n = a[2] as usize;
            let mut x = lower(e.mem.cstr(a[0]));
            let mut y = lower(e.mem.cstr(a[1]));
            x.truncate(n);
            y.truncate(n);
            order(x, y).into_ret()
        });
        e.register_double(STRCMP, |e, a| {
            order(e.mem.cstr(a[0]), e.mem.cstr(a[1])).into_ret()
        });
        e.register_double(ATOL, |e, a| {
            let t = String::from_utf8(e.mem.cstr(a[0])).unwrap();
            t.parse::<u32>().unwrap_or(0).into_ret()
        });
        let slot = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| slot.into_ret());
        e.register_double(INVALID_PARAMETER, |_, _| Ret::default());
        put(&mut e, ACP_TEXT, b"ACP");
        put(&mut e, OCP_TEXT, b"OCP");
        put(&mut e, ZERO_CODE_PAGE_TEXT, b"0");
        e.mem.map(DEFAULT_COUNTRIES, 20);
        e.mem.set_u16(DEFAULT_COUNTRIES, 0x409);
        table(&mut e, COUNTRY_TABLE, 0x17, b"britain", b"GBR");
        table(&mut e, LANGUAGE_TABLE, 0x41, b"american", b"ENU");
        e.call_log = Some(vec![]);
        (e, ptd)
    }

    /// A sorted name table of `count` entries: `first` (with its abbreviation), then
    /// `zz00`, `zz01`, ...
    fn table(e: &mut Engine, at: u32, count: u32, first: &[u8], abbreviation: &[u8]) {
        e.mem.map(at, count * 8);
        for i in 0..count {
            let name = if i == 0 {
                first.to_vec()
            } else {
                format!("zz{i:02}").into_bytes()
            };
            let name = text(e, &name);
            e.mem.set_u32(at + i * 8, name);
        }
        e.mem.set_cstr(at + 4, abbreviation);
    }

    fn names(e: &mut Engine, language: &[u8], country: &[u8], code_page: &[u8]) -> u32 {
        let at = e.mem.alloc(0x90);
        e.mem.set_cstr(at, language);
        e.mem.set_cstr(at + 0x40, country);
        e.mem.set_cstr(at + 0x80, code_page);
        at
    }

    #[test]
    fn xtoa_s_and_itoa_s_write_digits_or_report() {
        let (mut e, _) = engine();
        let target = e.mem.alloc(16);
        assert_eq!(itoa_s(&mut e, (-123i32) as u32, target, 16, 10), 0);
        assert_eq!(e.mem.cstr(target), b"-123".to_vec());
        assert_eq!(
            e.call(ITOA_S, &args![255u32, target, 16u32, 16u32]).u32(),
            0
        );
        assert_eq!(e.mem.cstr(target), b"ff".to_vec());
        assert_eq!(itoa_s(&mut e, 0xffff_ffff, target, 16, 16), 0);
        assert_eq!(e.mem.cstr(target), b"ffffffff".to_vec());
        assert_eq!(itoa_s(&mut e, 0, target, 16, 2), 0);
        assert_eq!(e.mem.cstr(target), b"0".to_vec());
        // Exactly the room for the digits and the zero.
        assert_eq!(itoa_s(&mut e, 1234, target, 5, 10), 0);
        // One byte short: emptied, ERANGE.
        assert_eq!(itoa_s(&mut e, 12345, target, 5, 10), 0x22);
        assert_eq!(e.mem.u8(target), 0);
        assert_eq!(itoa_s(&mut e, (-1i32) as u32, target, 2, 10), 0x22);
        assert_eq!(itoa_s(&mut e, 5, target, 16, 1), 0x16);
        assert_eq!(itoa_s(&mut e, 5, target, 16, 37), 0x16);
        assert_eq!(itoa_s(&mut e, 5, 0, 16, 10), 0x16);
        assert_eq!(itoa_s(&mut e, 5, target, 0, 10), 0x16);
        assert_eq!(calls(&e, INVALID_PARAMETER).len(), 6);
    }

    #[test]
    fn translate_name_searches_the_sorted_table() {
        let (mut e, _) = engine();
        let slot = e.mem.alloc(4);
        let american = text(&mut e, b"AMERICAN");
        e.mem.set_u32(slot, american);
        assert_eq!(translate_name(&mut e, LANGUAGE_TABLE, 0x40, slot), 1);
        assert_eq!(e.mem.cstr(e.mem.u32(slot)), b"ENU".to_vec());
        let zz = text(&mut e, b"zz17");
        e.mem.set_u32(slot, zz);
        assert_eq!(
            e.call(TRANSLATE_NAME, &args![LANGUAGE_TABLE, 0x40u32, slot])
                .u32(),
            1
        );
        assert_eq!(e.mem.u32(slot), LANGUAGE_TABLE + 17 * 8 + 4);
        let none = text(&mut e, b"klingon");
        e.mem.set_u32(slot, none);
        assert_eq!(translate_name(&mut e, LANGUAGE_TABLE, 0x40, slot), 0);
        assert_eq!(e.mem.u32(slot), none);
        assert_eq!(translate_name(&mut e, LANGUAGE_TABLE, 0xffff_ffff, slot), 0);
    }

    #[test]
    fn small_helpers_read_hex_letters_and_defaults() {
        let (mut e, _) = engine();
        let hex = text(&mut e, b"00000409");
        assert_eq!(e.call(LCID_FROM_HEX_STRING, &args![hex]).u32(), 0x409);
        let hex = text(&mut e, b"1aF");
        assert_eq!(lcid_from_hex_string(&mut e, hex), 0x1af);
        let name = text(&mut e, b"English_US");
        assert_eq!(e.call(GET_PRIMARY_LEN, &args![name]).u32(), 7);
        let name = text(&mut e, b"Zz@");
        assert_eq!(get_primary_len(&mut e, name), 2);
        assert_eq!(test_default_country(&mut e, 0x0001_0409), 0);
        assert_eq!(e.call(TEST_DEFAULT_COUNTRY, &args![0x807u32]).u32(), 1);
    }

    #[test]
    fn process_code_page_reads_the_default_or_the_number() {
        let (mut e, ptd) = engine();
        let state = ptd + 0x9c;
        e.mem.set_u32(state + 0x1c, 0x409);
        assert_eq!(e.call(PROCESS_CODE_PAGE, &args![0u32, state]).u32(), 1252);
        let acp = text(&mut e, b"ACP");
        assert_eq!(process_code_page(&mut e, acp, state), 1252);
        let ocp = text(&mut e, b"OCP");
        assert_eq!(process_code_page(&mut e, ocp, state), 437);
        let number = text(&mut e, b"850");
        assert_eq!(process_code_page(&mut e, number, state), 850);
        // A default of "0" is GetACP's.
        put(&mut e, ZERO_CODE_PAGE_TEXT, b"1252");
        assert_eq!(process_code_page(&mut e, 0, state), 1252);
        assert_eq!(calls(&e, GET_ACP).len(), 1);
        // No information: 0.
        e.mem.set_u32(state + 0x1c, 0x123);
        assert_eq!(process_code_page(&mut e, 0, state), 0);
    }

    #[test]
    fn test_default_language_checks_the_primary_language() {
        let (mut e, ptd) = engine();
        let state = ptd + 0x9c;
        let english = text(&mut e, b"English");
        e.mem.set_u32(state, english);
        assert_eq!(test_default_language(&mut e, state, 0x409, 1), 1);
        assert_eq!(
            e.call(TEST_DEFAULT_LANGUAGE, &args![state, 0x809u32, 0u32])
                .u32(),
            1
        );
        assert_eq!(test_default_language(&mut e, state, 0x809, 1), 0);
        let partial = text(&mut e, b"Eng-x");
        e.mem.set_u32(state, partial);
        assert_eq!(test_default_language(&mut e, state, 0x809, 1), 1);
        assert_eq!(test_default_language(&mut e, state, 0x123, 1), 0);
    }

    #[test]
    fn qualified_language_and_country() {
        let (mut e, ptd) = engine();
        let input = names(&mut e, b"English", b"United States", b"");
        let id = e.mem.alloc(8);
        let output = e.mem.alloc(0x90);
        let answer = e.call(0x00ee_0ff8, &args![input, id, output]).u32();
        assert_eq!(answer, 1);
        assert_eq!(
            (e.mem.u16(id), e.mem.u16(id + 2), e.mem.u16(id + 4)),
            (0x409, 0x409, 1252)
        );
        assert_eq!(e.mem.cstr(output), b"English".to_vec());
        assert_eq!(e.mem.cstr(output + 0x40), b"United States".to_vec());
        assert_eq!(e.mem.cstr(output + 0x80), b"1252".to_vec());
        assert_eq!(e.mem.u32(ptd + 0x9c + 8), 0x304);
        // The enumeration stopped at the first locale.
        assert_eq!(calls(&e, LANG_COUNTRY_ENUM_PROC).len(), 1);
    }

    #[test]
    fn qualified_language_through_the_table() {
        let (mut e, _) = engine();
        let input = names(&mut e, b"american", b"", b"");
        let id = e.mem.alloc(8);
        assert_eq!(get_qualified_locale(&mut e, input, id, 0), 1);
        assert_eq!(e.mem.u16(id), 0x409);
        // The first search found nothing; the abbreviation found 0409.
        assert_eq!(calls(&e, LANGUAGE_ENUM_PROC).len(), 4);
        // A language nobody knows: 0.
        let input = names(&mut e, b"Klingon", b"", b"");
        assert_eq!(get_qualified_locale(&mut e, input, id, 0), 0);
    }

    #[test]
    fn qualified_country_alone_and_defaults() {
        let (mut e, ptd) = engine();
        let input = names(&mut e, b"", b"Switzerland", b"850");
        let id = e.mem.alloc(8);
        assert_eq!(get_qualified_locale(&mut e, input, id, 0), 1);
        assert_eq!(
            (e.mem.u16(id), e.mem.u16(id + 2), e.mem.u16(id + 4)),
            (0x807, 0x807, 850)
        );
        assert_eq!(calls(&e, COUNTRY_ENUM_PROC).len(), 3);
        // Nothing at all, or no input: the user default.
        let input = names(&mut e, b"", b"", b"");
        assert_eq!(get_qualified_locale(&mut e, input, id, 0), 1);
        assert_eq!(e.mem.u32(ptd + 0x9c + 8), 0x104);
        assert_eq!(get_qualified_locale(&mut e, 0, id, 0), 1);
        assert_eq!(e.mem.u16(id + 4), 1252);
        // UTF-8 is refused.
        let input = names(&mut e, b"", b"", b"65001");
        assert_eq!(get_qualified_locale(&mut e, input, id, 0), 0);
        // An invalid locale is refused.
        e.register_double(IS_VALID_LOCALE, |_, _| 0u32.into_ret());
        assert_eq!(get_qualified_locale(&mut e, 0, id, 0), 0);
    }

    #[test]
    fn lang_country_enum_proc_records_partial_matches() {
        let (mut e, ptd) = engine();
        let state = ptd + 0x9c;
        let language = text(&mut e, b"Germ");
        let country = text(&mut e, b"Switzerland");
        e.mem.set_u32(state, language);
        e.mem.set_u32(state + 4, country);
        e.mem.set_u32(state + 8, 0);
        e.mem.set_u32(state + 0xc, 4);
        e.mem.set_u32(state + 0x10, 0);
        e.mem.set_u32(state + 0x14, 0);
        e.mem.set_u32(state + 0x18, 0);
        let lcid = text(&mut e, b"00000807");
        // Country matches; "Germ" is the first four letters of "German" and the whole text.
        assert_eq!(lang_country_enum_proc(&mut e, lcid), 1);
        assert_eq!(e.mem.u32(state + 8), 2);
        assert_eq!(e.mem.u32(state + 0x1c), 0x807);
        assert_eq!(e.mem.u32(state + 0x18), 0x807);
        // A locale with no information clears the flags and goes on.
        let unknown = text(&mut e, b"00000123");
        assert_eq!(e.call(LANG_COUNTRY_ENUM_PROC, &args![unknown]).u32(), 1);
        assert_eq!(e.mem.u32(state + 8), 0);
    }
}
