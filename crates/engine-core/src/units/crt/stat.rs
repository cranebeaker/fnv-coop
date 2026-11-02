//! The C runtime's `_stat64i32` (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe
//! 1.4.0.525), translated from the exe, and the functions it calls: the multibyte helpers
//! `_mbspbrk`, `_mbctolower` and `_mbsrchr` (each with its `_l` body), `_ismbblead` (and
//! `_ismbbtype_l`, which `_splitpath_s` reaches through it), `__getdrive`,
//! `__dtoxmode` and `_IsRootUNCName`. The archive constructor (`00af7eb0`) takes each archive's
//! file time from it.

use super::*;

/// KERNEL32 import slots: `FindFirstFileA`, `FindClose`, `GetDriveTypeA`,
/// `FileTimeToLocalFileTime`, `FileTimeToSystemTime`, `GetCurrentDirectoryA`.
const IMPORT_FIND_FIRST_FILE_A: u32 = 0x00fd_f070;
const IMPORT_FIND_CLOSE: u32 = 0x00fd_f064;
const IMPORT_GET_DRIVE_TYPE_A: u32 = 0x00fd_f0e8;
const IMPORT_FILE_TIME_TO_LOCAL_FILE_TIME: u32 = 0x00fd_f09c;
const IMPORT_FILE_TIME_TO_SYSTEM_TIME: u32 = 0x00fd_f098;
const IMPORT_GET_CURRENT_DIRECTORY_A: u32 = 0x00fd_f0c0;
/// CRT functions (cdecl) by address: `_mbspbrk`, `_mbspbrk_l`, `_mbctolower`,
/// `_mbctolower_l`, `_mbsrchr`, `_mbsrchr_l`, `__getdrive`, `__fullpath_helper`, `strlen`,
/// `_IsRootUNCName` (its path in `ESI`), `free`, `__loctotime64_t`, `__dtoxmode`,
/// `_dosmaperr`, `__calloc_crt`, `toupper`, `strpbrk`, `strrchr`, `_mbsicmp`,
/// `__crtLCMapStringA`.
const MBSPBRK: u32 = 0x00ed_76fc;
const MBSPBRK_L: u32 = 0x00ed_762f;
const MBCTOLOWER: u32 = 0x00ed_761c;
const MBCTOLOWER_L: u32 = 0x00ed_7576;
const MBSRCHR: u32 = 0x00ee_248c;
const MBSRCHR_L: u32 = 0x00ee_23d0;
/// `_ismbbtype_l` (cdecl: locale, character, ctype mask, mbctype mask).
const ISMBBTYPE_L: u32 = 0x00ed_5c21;
#[cfg(test)]
const ISMBBLEAD: u32 = 0x00ed_5c8d;
/// `__strlwr_s_l` and its body `_strlwr_s_l_stat` (cdecl: string, size, locale / `_LocaleUpdate`).
const STRLWR_S_L: u32 = 0x00ec_8fb7;
const STRLWR_S_L_STAT: u32 = 0x00ec_8e34;
/// `__mbsdec_l` (cdecl: start, position, locale) and `_mbsdec`.
const MBSDEC_L: u32 = 0x00ed_5ca5;
#[cfg(test)]
const MBSDEC: u32 = 0x00ed_5d3e;
const GETDRIVE: u32 = 0x00ed_74ad;
const FULLPATH_HELPER: u32 = 0x00ec_91dc;
const STRLEN: u32 = 0x00ec_6130;
const IS_ROOT_UNC_NAME: u32 = 0x00ec_9162;
const FREE: u32 = 0x00ec_d291;
const LOCTOTIME64: u32 = 0x00ed_7242;
const DTOXMODE: u32 = 0x00ed_7185;
const DOSMAPERR: u32 = 0x00ec_8609;
const CALLOC_CRT: u32 = 0x00ed_0d24;
const TOUPPER: u32 = 0x00ec_a7f4;
const STRPBRK: u32 = 0x00ec_bf20;
const STRRCHR: u32 = 0x00ec_6e30;
const MBSICMP: u32 = 0x00ec_858a;
const CRT_LC_MAP_STRING_A: u32 = 0x00ed_0883;
/// `"?*"`, `"./\"` and the four extensions `__dtoxmode` marks executable (`.exe`, `.cmd`,
/// `.bat`, `.com`), in `.rdata`.
const TEXT_WILDCARDS: u32 = 0x010f_1fe8;
const TEXT_PATH_MARKS: u32 = 0x010f_1fe4;
const TEXT_EXE: u32 = 0x010f_3428;
const TEXT_CMD: u32 = 0x010f_3420;
const TEXT_BAT: u32 = 0x010f_3418;
const TEXT_COM: u32 = 0x010f_3410;

/// The multibyte information of a `_LocaleUpdate` object (+4) and its lead-byte test: bit 2
/// of `mbctype` (+0x1D) for the byte.
fn lead_byte(e: &Engine, mbcinfo: u32, byte: u8) -> bool {
    e.mem.u8(mbcinfo + 0x1d + u32::from(byte)) & 4 != 0
}

// Translated from 00ed76fc (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbspbrk` (LIBCMT/mbspbrk.c, engine map), cdecl: (string, control): `_mbspbrk_l` with no
/// locale.
pub fn mbspbrk(e: &mut Engine, string: Ptr, control: Ptr) -> Ptr {
    e.call(MBSPBRK_L, &args![string, control, 0u32]).ptr()
}

// Translated from 00ed762f (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbspbrk_l` (LIBCMT/mbspbrk.c, engine map), cdecl: (string, control, locale). With the
/// locale taken (`_LocaleUpdate`): a single-byte code page (`ismbcodepage` +8 clear) is
/// `strpbrk`. Otherwise a null string or control gives `errno` `EINVAL`, the invalid-parameter
/// handler and null. Else the first character of the string found in the control, comparing
/// two-byte characters (lead bytes, `mbctype` bit 2) as pairs; a lead byte at the very end of
/// the control matches any character, as the exe's scan stops there. Answers that character,
/// or null.
pub fn mbspbrk_l(e: &mut Engine, string: Ptr, control: Ptr, locale: u32) -> Ptr {
    e.with_stack(0x10, |e, frame| {
        let update = frame.addr();
        e.call(LOCALE_UPDATE, &[update, locale]);
        let mbcinfo = e.mem.u32(update + 4);
        if e.mem.u32(mbcinfo + 8) == 0 {
            let found = e.call(STRPBRK, &args![string, control]).u32();
            locale_update_release(e, update);
            return Ptr::new(found);
        }
        let mut at = string.addr();
        if at == 0 || control.addr() == 0 {
            set_errno(e, 0x16);
            invalid_parameter_zeros(e);
            locale_update_release(e, update);
            return Ptr::new(0);
        }
        if e.mem.u8(at) != 0 {
            loop {
                let mut c = control.addr();
                while e.mem.u8(c) != 0 {
                    let byte = e.mem.u8(c);
                    if lead_byte(e, mbcinfo, byte) {
                        if byte == e.mem.u8(at) && e.mem.u8(c + 1) == e.mem.u8(at + 1) {
                            break;
                        }
                        let next = c.wrapping_add(1);
                        if e.mem.u8(next) == 0 {
                            break;
                        }
                        c = next;
                    } else if byte == e.mem.u8(at) {
                        break;
                    }
                    c = c.wrapping_add(1);
                }
                if e.mem.u8(c) != 0 {
                    break;
                }
                if lead_byte(e, mbcinfo, e.mem.u8(at)) {
                    at = at.wrapping_add(1);
                    if e.mem.u8(at) == 0 {
                        break;
                    }
                }
                at = at.wrapping_add(1);
                if e.mem.u8(at) == 0 {
                    break;
                }
            }
        }
        let found = if e.mem.u8(at) != 0 { at } else { 0 };
        locale_update_release(e, update);
        Ptr::new(found)
    })
}

// Translated from 00ed761c (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbctolower` (LIBCMT/mbctolow.c, engine map), cdecl: the character: `_mbctolower_l` with
/// no locale.
pub fn mbctolower(e: &mut Engine, c: u32) -> u32 {
    e.call(MBCTOLOWER_L, &args![c, 0u32]).u32()
}

// Translated from 00ed7576 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbctolower_l` (LIBCMT/mbctolow.c, engine map), cdecl: (character, locale). With the
/// locale taken (`_LocaleUpdate`): a single byte that is upper case (`mbctype` +0x1D bit 4)
/// becomes its `mbcasemap` (+0x11D) entry, others stay. A two-byte character whose high byte is
/// not a lead byte stays; otherwise it is mapped with `__crtLCMapStringA(update, its locale
/// name (+0xC), LCMAP_LOWERCASE, the two bytes, 2, out, 2, its code page (+4), 1)`, the
/// character staying when that answers 0, else the two bytes out (high first).
pub fn mbctolower_l(e: &mut Engine, c: u32, locale: u32) -> u32 {
    e.with_stack(0x18, |e, frame| {
        let update = frame.addr();
        let source = update + 0x14;
        let out = update + 0x10;
        e.call(LOCALE_UPDATE, &[update, locale]);
        let mbcinfo = e.mem.u32(update + 4);
        let result = if c <= 0xff {
            let entry = mbcinfo.wrapping_add(c);
            if e.mem.u8(entry + 0x1d) & 0x10 != 0 {
                u32::from(e.mem.u8(entry + 0x11d))
            } else {
                c
            }
        } else {
            let high = (c >> 8) as u8;
            e.mem.set_u8(source, high);
            e.mem.set_u8(source + 1, c as u8);
            if !lead_byte(e, mbcinfo, high) {
                c
            } else {
                let name = e.mem.u32(mbcinfo + 0xc);
                let code_page = e.mem.u32(mbcinfo + 4);
                let mapped = e
                    .call(
                        CRT_LC_MAP_STRING_A,
                        &args![update, name, 0x100u32, source, 2u32, out, 2u32, code_page, 1u32],
                    )
                    .u32();
                if mapped == 0 {
                    c
                } else {
                    (u32::from(e.mem.u8(out)) << 8).wrapping_add(u32::from(e.mem.u8(out + 1)))
                }
            }
        };
        locale_update_release(e, update);
        result
    })
}

// Translated from 00ed5c21 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_ismbbtype_l` (LIBCMT/ismbbyte.c, Ghidra library match), cdecl: (locale, character,
/// ctype mask, mbctype mask). With the locale taken (`_LocaleUpdate`): 1 when the character's
/// low byte has a bit of the mbctype mask in the multibyte table (`mbctype` +0x1D), or, with a
/// nonzero ctype mask, a bit of it in the locale's `pctype` word table (locale info +0xC8);
/// otherwise 0.
pub fn ismbbtype_l(e: &mut Engine, locale: u32, c: u32, ctype_mask: u32, mbctype_mask: u32) -> u32 {
    e.with_stack(0x10, |e, frame| {
        let update = frame.addr();
        e.call(LOCALE_UPDATE, &[update, locale]);
        let byte = c & 0xff;
        let mbcinfo = e.mem.u32(update + 4);
        let result =
            if e.mem.u8(mbcinfo.wrapping_add(0x1d).wrapping_add(byte)) & mbctype_mask as u8 != 0 {
                1
            } else {
                let ctype = if ctype_mask == 0 {
                    0
                } else {
                    let table = e.mem.u32(e.mem.u32(update).wrapping_add(0xc8));
                    u32::from(e.mem.u16(table.wrapping_add(byte * 2))) & ctype_mask
                };
                u32::from(ctype != 0)
            };
        locale_update_release(e, update);
        result
    })
}

// Translated from 00ed5c8d (disassembled, FalloutNV.exe 1.4.0.525)
/// `_ismbblead` (LIBCMT/ismbbyte.c), cdecl: (character): `_ismbbtype_l(NULL, character, 0,
/// _MLEAD (4))`.
pub fn ismbblead(e: &mut Engine, c: u32) -> u32 {
    e.call(ISMBBTYPE_L, &args![0u32, c, 0u32, 4u32]).u32()
}

// Translated from 00ed5ca5 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__mbsdec_l` (LIBCMT/mbsdec.c, Ghidra library match), cdecl: (start, position, locale):
/// the character before `position`. A null start or position gives `errno` `EINVAL`, the
/// invalid-parameter handler and null; a position not after the start gives null (neither
/// with the locale taken). With the locale taken (`_LocaleUpdate`): on a single-byte code page
/// `position - 1`; otherwise it counts back over the lead bytes before `position - 1` (down to
/// the start) and answers `position - 1`, or `position - 2` when an odd number of bytes lay
/// between (so a two-byte character is stepped over whole).
pub fn mbsdec_l(e: &mut Engine, start: Ptr, position: Ptr, locale: u32) -> Ptr {
    let (start, position) = (start.addr(), position.addr());
    if start == 0 || position == 0 {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return Ptr::new(0);
    }
    if start >= position {
        return Ptr::new(0);
    }
    e.with_stack(0x10, |e, frame| {
        let update = frame.addr();
        e.call(LOCALE_UPDATE, &[update, locale]);
        let mbcinfo = e.mem.u32(update + 4);
        let result = if e.mem.u32(mbcinfo + 8) == 0 {
            position - 1
        } else {
            let mut at = position - 1;
            loop {
                at = at.wrapping_sub(1);
                if start > at || !lead_byte(e, mbcinfo, e.mem.u8(at)) {
                    break;
                }
            }
            position - 1 - (position.wrapping_sub(at) & 1)
        };
        locale_update_release(e, update);
        Ptr::new(result)
    })
}

// Translated from 00ed5d3e (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbsdec` (LIBCMT/mbsdec.c), cdecl: (start, position): `__mbsdec_l` with no locale.
pub fn mbsdec(e: &mut Engine, start: Ptr, position: Ptr) -> Ptr {
    e.call(MBSDEC_L, &args![start, position, 0u32]).ptr()
}

// Translated from 00ec8fb7 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__strlwr_s_l` (LIBCMT/strlwr.c, Ghidra library match), cdecl: (string, size, locale): with
/// the locale taken (`_LocaleUpdate`), `_strlwr_s_l_stat(string, size, the update)`'s answer.
pub fn strlwr_s_l(e: &mut Engine, string: Ptr, size: u32, locale: u32) -> u32 {
    e.with_stack(0x10, |e, frame| {
        let update = frame.addr();
        e.call(LOCALE_UPDATE, &[update, locale]);
        let result = e.call(STRLWR_S_L_STAT, &args![string, size, update]).u32();
        locale_update_release(e, update);
        result
    })
}

// Translated from 00ec8feb (disassembled, FalloutNV.exe 1.4.0.525)
/// `_strlwr_s` (LIBCMT/strlwr.c), cdecl: (string, size): `__strlwr_s_l` with no locale.
pub fn strlwr_s(e: &mut Engine, string: Ptr, size: u32) -> u32 {
    e.call(STRLWR_S_L, &args![string, size, 0u32]).u32()
}

// Translated from 00ee248c (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbsrchr` (LIBCMT/mbsrchr.c, engine map), cdecl: (string, character): `_mbsrchr_l` with
/// no locale.
pub fn mbsrchr(e: &mut Engine, string: Ptr, c: u32) -> Ptr {
    e.call(MBSRCHR_L, &args![string, c, 0u32]).ptr()
}

// Translated from 00ee23d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbsrchr_l` (LIBCMT/mbsrchr.c, engine map), cdecl: (string, character, locale). With the
/// locale taken (`_LocaleUpdate`): a null string gives `errno` `EINVAL`, the invalid-parameter
/// handler and null; a single-byte code page is `strrchr`. Otherwise the last place the
/// character is, two-byte characters read as one (the lead byte's place answered); the
/// terminating zero counts as a character, and a lead byte just before it leaves the place of
/// that zero when nothing was found before.
pub fn mbsrchr_l(e: &mut Engine, string: Ptr, c: u32, locale: u32) -> Ptr {
    e.with_stack(0x14, |e, frame| {
        let update = frame.addr();
        e.call(LOCALE_UPDATE, &[update, locale]);
        let mut at = string.addr();
        if at == 0 {
            set_errno(e, 0x16);
            invalid_parameter_zeros(e);
            locale_update_release(e, update);
            return Ptr::new(0);
        }
        let mbcinfo = e.mem.u32(update + 4);
        if e.mem.u32(mbcinfo + 8) == 0 {
            let found = e.call(STRRCHR, &args![at, c]).u32();
            locale_update_release(e, update);
            return Ptr::new(found);
        }
        let mut found = 0u32;
        loop {
            let mut byte = e.mem.u8(at);
            let mut character = u32::from(byte);
            if lead_byte(e, mbcinfo, byte) {
                at = at.wrapping_add(1);
                byte = e.mem.u8(at);
                if byte == 0 {
                    if found == 0 {
                        found = at;
                    }
                } else {
                    character = (character << 8) | u32::from(byte);
                    if c == character {
                        found = at.wrapping_sub(1);
                    }
                }
            } else if c == character {
                found = at;
            }
            at = at.wrapping_add(1);
            if byte == 0 {
                break;
            }
        }
        locale_update_release(e, update);
        Ptr::new(found)
    })
}

// Translated from 00ed7185 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__dtoxmode` (LIBCMT/stat.c, engine map), cdecl: (attributes byte, path): the `st_mode`
/// for a file with those attributes. A root (`\` or `/` alone, after any drive), a directory
/// (`FILE_ATTRIBUTE_DIRECTORY` 0x10) or an empty name is `_S_IFDIR | _S_IEXEC` (0x4040), else
/// `_S_IFREG` (0x8000); `_S_IREAD` (0x100) always, `_S_IWRITE` (0x80) unless read-only (bit 0);
/// `_S_IEXEC` (0x40) for a name ending in `.exe`, `.cmd`, `.bat` or `.com` (`_mbsrchr`,
/// `_mbsicmp`). The owner bits are copied to the group and other bits.
pub fn dtoxmode(e: &mut Engine, attributes: u8, path: Ptr) -> u32 {
    let path = path.addr();
    let name = if e.mem.u8(path + 1) == b':' {
        path + 2
    } else {
        path
    };
    let first = e.mem.u8(name);
    let mut mode = if ((first == b'\\' || first == b'/') && e.mem.u8(name + 1) == 0)
        || attributes & 0x10 != 0
        || first == 0
    {
        0x4040
    } else {
        0x8000
    };
    mode |= (!(u32::from(attributes) << 7) & 0x80) | 0x100;
    let dot = e.call(MBSRCHR, &args![path, 0x2eu32]).u32();
    if dot != 0 {
        let executable = [TEXT_EXE, TEXT_CMD, TEXT_BAT, TEXT_COM]
            .into_iter()
            .any(|text| e.call(MBSICMP, &args![dot, text]).u32() == 0);
        if executable {
            mode |= 0x40;
        }
    }
    mode |= (mode >> 3) & 0x38;
    ((mode >> 6) & 7) | mode
}

// Translated from 00ec9162 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_IsRootUNCName` (LIBCMT/stat.c, engine map); its path comes in `ESI` (the first argument
/// word here): whether the path is `\\server\share` with at most one separator after the
/// share. Fewer than 5 characters, no two leading separators (`\` or `/`), a third separator,
/// or no share answer 0.
pub fn is_root_unc_name(e: &mut Engine, path: Ptr) -> u32 {
    let path = path.addr();
    let slash = |b: u8| b == b'\\' || b == b'/';
    if e.call(STRLEN, &args![path]).u32() < 5 {
        return 0;
    }
    if !slash(e.mem.u8(path)) || !slash(e.mem.u8(path + 1)) || slash(e.mem.u8(path + 2)) {
        return 0;
    }
    let mut at = path + 3;
    if e.mem.u8(at) == 0 {
        return 0;
    }
    while !slash(e.mem.u8(at)) {
        at += 1;
        if e.mem.u8(at) == 0 {
            break;
        }
    }
    if e.mem.u8(at) == 0 {
        return 0;
    }
    at += 1;
    if e.mem.u8(at) == 0 {
        return 0;
    }
    while !slash(e.mem.u8(at)) {
        at += 1;
        if e.mem.u8(at) == 0 {
            break;
        }
    }
    if e.mem.u8(at) == 0 || e.mem.u8(at + 1) == 0 {
        1
    } else {
        0
    }
}

// Translated from 00ed74ad (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed74ad - the /GS stack cookie check (00ec408c) is not translated.
/// `__getdrive` (LIBCMT/getdrive.c, engine map), cdecl, no arguments: the current drive
/// (1 for A:). `GetCurrentDirectoryA(0x105, buffer)`; a longer directory is read again into
/// `__calloc_crt(length + 1, 1)` (`ENOMEM` and none when that fails). No directory gives
/// `errno` `ENOMEM` (0xC) and 0; a directory without a drive letter 0; else `toupper` of the
/// letter less 0x40. The allocated buffer is freed.
pub fn getdrive(e: &mut Engine) -> u32 {
    e.with_stack(0x110, |e, frame| {
        let mut buffer = frame.addr();
        let mut allocated = false;
        let mut length = e
            .call(IMPORT_GET_CURRENT_DIRECTORY_A, &args![0x105u32, buffer])
            .u32();
        if (length as i32) > 0x104 {
            let size = length.wrapping_add(1);
            let block = e.call(CALLOC_CRT, &args![size, 1u32]).u32();
            if block == 0 {
                set_errno(e, 0xc);
                length = 0;
            } else {
                buffer = block;
                allocated = true;
                if length != 0 {
                    length = e
                        .call(IMPORT_GET_CURRENT_DIRECTORY_A, &args![size, buffer])
                        .u32();
                }
            }
        }
        let mut drive = 0u32;
        if length == 0 {
            set_errno(e, 0xc);
        } else if e.mem.u8(buffer + 1) == b':' {
            let letter = u32::from(e.mem.u8(buffer));
            drive = e.call(TOUPPER, &args![letter]).u32().wrapping_sub(0x40);
        }
        if allocated {
            e.call(FREE, &args![buffer]);
        }
        drive
    })
}

/// `__loctotime64_t(year, month, day, hour, minute, second, dst)` (`00ed7242`) of the
/// `SYSTEMTIME` at `system` (its `wYear`, `wMonth`, `wDay`, `wHour`, `wMinute`, `wSecond`
/// words), with `dst` -1, as `_stat64i32` calls it three times.
fn system_time_to_time64(e: &mut Engine, system: u32) -> u64 {
    let word = |e: &Engine, offset: u32| u32::from(e.mem.u16(system + offset));
    let (year, month, day) = (word(e, 0), word(e, 2), word(e, 6));
    let (hour, minute, second) = (word(e, 8), word(e, 0xa), word(e, 0xc));
    e.call(
        LOCTOTIME64,
        &args![year, month, day, hour, minute, second, 0xffff_ffffu32],
    )
    .u64()
}

fn set_u64(e: &mut Engine, at: u32, value: u64) {
    e.mem.set_u32(at, value as u32);
    e.mem.set_u32(at + 4, (value >> 32) as u32);
}

// Translated from 00ec923c (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec923c - the /GS stack cookie check (00ec408c) is not translated.
/// `_stat64i32` (LIBCMT/stat.c, engine map), cdecl: (path, out `struct _stat64i32`): 0, or -1
/// with `errno` set. A null path or out gives `_doserrno` 0, `errno` `EINVAL` (0x16) and the
/// invalid-parameter handler. A path with `?` or `*`, or a drive letter alone (`X:`), is
/// `ENOENT` (2, `_doserrno` 2). The drive is the letter's (`_mbctolower` less 0x60) or the
/// current one (`__getdrive`). `FindFirstFileA` describes the file; when it finds nothing a
/// path with `.`, `/` or `\` that is a root (its full path, `__fullpath_helper`, is 3
/// characters or a root UNC name, `_IsRootUNCName`, and `GetDriveTypeA` above 1) is a
/// directory dated 1980-01-01 00:00 local; anything else is `ENOENT`. A found file's
/// last-write time (0 when none) is `st_mtime` through `FileTimeToLocalFileTime`,
/// `FileTimeToSystemTime` and `__loctotime64_t(.., -1)`; the access and creation times the same,
/// or `st_mtime` when not kept; a failed conversion maps `GetLastError` (`_dosmaperr`), closes
/// the search and answers -1. Then the search is closed (`FindClose`), `st_mode` is
/// `__dtoxmode(attributes, path)`, `st_nlink` 1, `st_size` the low size word, `st_ino`,
/// `st_uid` and `st_gid` 0, and `st_dev` and `st_rdev` the drive less 1.
pub fn stat64i32(e: &mut Engine, path: Ptr, out: Ptr) -> i32 {
    let path = path.addr();
    let out = out.addr();
    if path == 0 || out == 0 {
        set_doserrno(e, 0);
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return -1;
    }
    let not_found = |e: &mut Engine| {
        set_errno(e, 2);
        set_doserrno(e, 2);
        -1
    };
    if e.call(MBSPBRK, &args![path, TEXT_WILDCARDS]).u32() != 0 {
        return not_found(e);
    }
    let drive = if e.mem.u8(path + 1) == b':' {
        let letter = e.mem.u8(path);
        if letter != 0 && e.mem.u8(path + 2) == 0 {
            return not_found(e);
        }
        let letter = letter as i8 as i32 as u32;
        e.call(MBCTOLOWER, &args![letter]).u32().wrapping_sub(0x60)
    } else {
        e.call(GETDRIVE, &args![]).u32()
    };
    // The frame below EBP-0x268: the drive, the local FILETIME (+4), the SYSTEMTIME (+0xC), the
    // search handle (+0x1C), the WIN32_FIND_DATAA (+0x20) and the full-path buffer (+0x160).
    e.with_stack(0x268, |e, frame| {
        let local = frame.addr() + 4;
        let system = frame.addr() + 0xc;
        let handle_slot = frame.addr() + 0x1c;
        let data = frame.addr() + 0x20;
        let full = frame.addr() + 0x160;
        let handle = e.call(IMPORT_FIND_FIRST_FILE_A, &args![path, data]).u32();
        e.mem.set_u32(handle_slot, handle);
        if handle == 0xffff_ffff {
            e.mem.set_u32(handle_slot, 0);
            if e.call(MBSPBRK, &args![path, TEXT_PATH_MARKS]).u32() == 0 {
                return not_found(e);
            }
            let name = e
                .call(FULLPATH_HELPER, &args![full, path, 0x104u32, handle_slot])
                .u32();
            let root = name != 0
                && (e.call(STRLEN, &args![name]).u32() == 3
                    || e.call(IS_ROOT_UNC_NAME, &args![name]).u32() != 0)
                && e.call(IMPORT_GET_DRIVE_TYPE_A, &args![name]).u32() > 1;
            if !root {
                let block = e.mem.u32(handle_slot);
                if block != 0 {
                    e.call(FREE, &args![block]);
                }
                return not_found(e);
            }
            let block = e.mem.u32(handle_slot);
            if block != 0 {
                e.call(FREE, &args![block]);
            }
            e.mem.set_u32(data, 0x10);
            e.mem.set_u32(data + 0x1c, 0);
            e.mem.set_u32(data + 0x20, 0);
            e.mem.set_u8(data + 0x2c, 0);
            let time = e
                .call(
                    LOCTOTIME64,
                    &args![0x7bcu32, 1u32, 1u32, 0u32, 0u32, 0u32, 0xffff_ffffu32],
                )
                .u64();
            set_u64(e, out + 0x20, time);
            set_u64(e, out + 0x18, time);
            set_u64(e, out + 0x28, time);
        } else {
            // A conversion that failed: the error mapped, the search closed, -1.
            let failed = |e: &mut Engine| {
                let error = e.call(IMPORT_GET_LAST_ERROR, &args![]).u32();
                e.call(DOSMAPERR, &args![error]);
                let handle = e.mem.u32(handle_slot);
                e.call(IMPORT_FIND_CLOSE, &args![handle]);
                -1
            };
            // The FILETIME at `data + offset` as a time (0 when it is zero), or `None` when a
            // conversion failed.
            let convert = |e: &mut Engine, offset: u32| -> Option<Option<u64>> {
                let time = data + offset;
                if e.mem.u32(time) == 0 && e.mem.u32(time + 4) == 0 {
                    return Some(None);
                }
                if e.call(IMPORT_FILE_TIME_TO_LOCAL_FILE_TIME, &args![time, local])
                    .u32()
                    == 0
                {
                    return None;
                }
                if e.call(IMPORT_FILE_TIME_TO_SYSTEM_TIME, &args![local, system])
                    .u32()
                    == 0
                {
                    return None;
                }
                Some(Some(system_time_to_time64(e, system)))
            };
            match convert(e, 0x14) {
                None => return failed(e),
                Some(time) => set_u64(e, out + 0x20, time.unwrap_or(0)),
            }
            match convert(e, 0xc) {
                None => return failed(e),
                Some(Some(time)) => set_u64(e, out + 0x18, time),
                Some(None) => {
                    let low = e.mem.u32(out + 0x20);
                    let high = e.mem.u32(out + 0x24);
                    e.mem.set_u32(out + 0x18, low);
                    e.mem.set_u32(out + 0x1c, high);
                }
            }
            match convert(e, 4) {
                None => return failed(e),
                Some(Some(time)) => set_u64(e, out + 0x28, time),
                Some(None) => {
                    let low = e.mem.u32(out + 0x20);
                    let high = e.mem.u32(out + 0x24);
                    e.mem.set_u32(out + 0x28, low);
                    e.mem.set_u32(out + 0x2c, high);
                }
            }
            let handle = e.mem.u32(handle_slot);
            e.call(IMPORT_FIND_CLOSE, &args![handle]);
        }
        let attributes = e.mem.u32(data);
        let mode = e.call(DTOXMODE, &args![attributes, path]).u32();
        e.mem.set_u16(out + 6, mode as u16);
        e.mem.set_u16(out + 8, 1);
        let size = e.mem.u32(data + 0x20);
        e.mem.set_u32(out + 0x14, size);
        e.mem.set_u16(out + 4, 0);
        e.mem.set_u16(out + 0xc, 0);
        e.mem.set_u16(out + 0xa, 0);
        let device = drive.wrapping_sub(1);
        e.mem.set_u32(out, device);
        e.mem.set_u32(out + 0x10, device);
        0
    })
}

/// `__isleadbyte_l` (`00ed419c`, cdecl: byte, locale object) and `__crtLCMapStringA_stat`
/// (`00ed04de`; its locale object comes in `ECX`, the first word here, then eight pushed words).
const ISLEADBYTE_L: u32 = 0x00ed_419c;
const CRT_LC_MAP_STRING_A_STAT: u32 = 0x00ed_04de;

// Translated from 00ec67d6 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___ascii_stricmp` (LIBCMT/stricmp.c, engine map), cdecl: (first, second): compares the
/// strings byte by byte with `A`..`Z` folded to lower case (`c - 0x41 <= 0x19` unsigned, plus
/// 0x20), stopping at the first difference or at the first string's end; answers the
/// difference of the two folded bytes.
pub fn ascii_stricmp(e: &mut Engine, first: Ptr, second: Ptr) -> i32 {
    let (mut a, mut b) = (first.addr(), second.addr());
    loop {
        let mut x = u32::from(e.mem.u8(a));
        a = a.wrapping_add(1);
        if x.wrapping_sub(0x41) <= 0x19 {
            x += 0x20;
        }
        let mut y = u32::from(e.mem.u8(b));
        b = b.wrapping_add(1);
        if y.wrapping_sub(0x41) <= 0x19 {
            y += 0x20;
        }
        if x == 0 || x != y {
            return x.wrapping_sub(y) as i32;
        }
    }
}

// Translated from 00ec680f (disassembled, FalloutNV.exe 1.4.0.525)
/// `__stricmp_l` (LIBCMT/stricmp.c, engine map), cdecl: (first, second, locale). With the
/// locale taken (`_LocaleUpdate`): a null string (the first checked first) gives `errno`
/// `EINVAL` (0x16), the invalid-parameter handler and `0x7fffffff`. A locale whose `LC_CTYPE`
/// handle (`lc_handle[LC_CTYPE]`, `locinfo` +0x14) is 0 (the C locale) compares with
/// `___ascii_stricmp`; otherwise both strings go through `__tolower_l` (with the update object
/// as the locale) one byte at a time until the first string's character is 0 or they differ,
/// answering the difference.
pub fn stricmp_l(e: &mut Engine, first: Ptr, second: Ptr, locale: u32) -> i32 {
    e.with_stack(0x10, |e, frame| {
        let update = frame.addr();
        e.call(LOCALE_UPDATE, &[update, locale]);
        if first.is_null() || second.is_null() {
            set_errno(e, 0x16);
            invalid_parameter_zeros(e);
            locale_update_release(e, update);
            return 0x7fff_ffff;
        }
        let info = e.mem.u32(update);
        let result = if e.mem.u32(info + 0x14) == 0 {
            ascii_stricmp(e, first, second)
        } else {
            let (mut a, mut b) = (first.addr(), second.addr());
            loop {
                let x = tolower_l(e, u32::from(e.mem.u8(a)), update);
                a = a.wrapping_add(1);
                let y = tolower_l(e, u32::from(e.mem.u8(b)), update);
                b = b.wrapping_add(1);
                if x == 0 || x != y {
                    break x.wrapping_sub(y) as i32;
                }
            }
        };
        locale_update_release(e, update);
        result
    })
}

// Translated from 00ec6695 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__tolower_l` (LIBCMT/tolower.c, engine map), cdecl: (character, locale). With the locale
/// taken (`_LocaleUpdate`): a character below 0x100 that is upper case (`_UPPER`, bit 1: through
/// `__isctype_l` when the locale's `mb_cur_max` (+0xAC) is above 1, else its ctype table entry
/// at +0xC8) becomes its entry in the lower-case map (+0xCC); other characters below 0x100 stay.
/// A larger one is two bytes (high, low) when `mb_cur_max` is above 1 and its high byte (`SAR
/// 8`) is a lead byte (`__isleadbyte_l`); otherwise `errno` becomes `EILSEQ` (0x2A) and only its
/// low byte is mapped. The bytes go through `__crtLCMapStringA(update, its LC_CTYPE handle
/// (+0x14), LCMAP_LOWERCASE, bytes, count, out, 3, code page (+4), 1)`: 0 keeps the character,
/// 1 answers the first byte out, more the first two (high first).
pub fn tolower_l(e: &mut Engine, c: u32, locale: u32) -> u32 {
    e.with_stack(0x18, |e, frame| {
        let update = frame.addr();
        let out = update + 0x10;
        let source = update + 0x14;
        e.call(LOCALE_UPDATE, &[update, locale]);
        let info = e.mem.u32(update);
        let mapped = if c < 0x100 {
            let upper = if e.mem.u32(info + 0xac) as i32 > 1 {
                e.call(ISCTYPE_L, &[c, 1, update]).u32()
            } else {
                let table = e.mem.u32(info + 0xc8);
                u32::from(e.mem.u16(table.wrapping_add(c.wrapping_mul(2)))) & 1
            };
            if upper != 0 {
                let info = e.mem.u32(update);
                let map = e.mem.u32(info + 0xcc);
                Some(u32::from(e.mem.u8(map.wrapping_add(c))))
            } else {
                None
            }
        } else {
            let high = ((c as i32) >> 8) as u32;
            let lead = e.mem.u32(info + 0xac) as i32 > 1
                && e.call(ISLEADBYTE_L, &[high & 0xff, update]).u32() != 0;
            let count = if lead {
                e.mem.set_u8(source, high as u8);
                e.mem.set_u8(source + 1, c as u8);
                e.mem.set_u8(source + 2, 0);
                2u32
            } else {
                set_errno(e, 0x2a);
                e.mem.set_u8(source, c as u8);
                e.mem.set_u8(source + 1, 0);
                1u32
            };
            let info = e.mem.u32(update);
            let lcid = e.mem.u32(info + 0x14);
            let code_page = e.mem.u32(info + 4);
            let written = e
                .call(
                    CRT_LC_MAP_STRING_A,
                    &args![update, lcid, 0x100u32, source, count, out, 3u32, code_page, 1u32],
                )
                .u32();
            match written {
                0 => None,
                1 => Some(u32::from(e.mem.u8(out))),
                _ => Some((u32::from(e.mem.u8(out)) << 8) | u32::from(e.mem.u8(out + 1))),
            }
        };
        locale_update_release(e, update);
        mapped.unwrap_or(c)
    })
}

// Translated from 00ed0883 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___crtLCMapStringA` (LIBCMT/a_map.c, engine map), cdecl, nine words: takes the locale
/// (`_LocaleUpdate`) and passes the update object (in `ECX`) and the eight other words to
/// `__crtLCMapStringA_stat` (`00ed04de`), answering its result.
#[allow(clippy::too_many_arguments)]
pub fn crt_lc_map_string_a(
    e: &mut Engine,
    locale: u32,
    lcid: u32,
    flags: u32,
    source: u32,
    source_count: u32,
    dest: u32,
    dest_count: u32,
    code_page: u32,
    error: u32,
) -> u32 {
    e.with_stack(0x10, |e, frame| {
        let update = frame.addr();
        e.call(LOCALE_UPDATE, &[update, locale]);
        let result = e
            .call(
                CRT_LC_MAP_STRING_A_STAT,
                &[
                    update,
                    lcid,
                    flags,
                    source,
                    source_count,
                    dest,
                    dest_count,
                    code_page,
                    error,
                ],
            )
            .u32();
        locale_update_release(e, update);
        result
    })
}

/// `_mbstok_l` (`00ece21d`), `_mbstok_s_l` (`00edfa19`), `__ismbblead_l` (`00ed5c74`),
/// `_getptd` (`00ed11de`) and `strtok_s` (`00ecc5f8`), cdecl.
const MBSTOK_L: u32 = 0x00ec_e21d;
const MBSTOK_S_L: u32 = 0x00ed_fa19;
const ISMBBLEAD_L: u32 = 0x00ed_5c74;
const GETPTD_ADDR: u32 = 0x00ed_11de;
const STRTOK_S: u32 = 0x00ec_c5f8;

// Translated from 00ece23e (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbstok` (LIBCMT/mbstok.c, engine map), cdecl: (string, delimiters): `_mbstok_l` with
/// no locale.
pub fn mbstok(e: &mut Engine, string: Ptr, delimiters: Ptr) -> Ptr {
    e.call(MBSTOK_L, &args![string, delimiters, 0u32]).ptr()
}

// Translated from 00ece21d (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbstok_l` (LIBCMT/mbstok.c, engine map), cdecl: (string, delimiters, locale):
/// `_mbstok_s_l` with the thread's token position (`_getptd()` + 0x20, `_mtoken`).
pub fn mbstok_l(e: &mut Engine, string: Ptr, delimiters: Ptr, locale: u32) -> Ptr {
    let ptd = e.call(GETPTD_ADDR, &args![]).u32();
    let context = ptd.wrapping_add(0x20);
    e.call(MBSTOK_S_L, &args![string, delimiters, context, locale])
        .ptr()
}

// Translated from 00ed5c74 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__ismbblead_l` (LIBCMT/ismbbyte.c, engine map), cdecl: (character, locale):
/// `_ismbbtype_l(locale, character, 0, _MLEAD (4))`.
pub fn ismbblead_l(e: &mut Engine, c: u32, locale: u32) -> u32 {
    e.call(ISMBBTYPE_L, &args![locale, c, 0u32, 4u32]).u32()
}

/// `_mbstok_s_l`'s delimiter scan: walks the delimiters from the start for the character at
/// `at` (a two-byte delimiter, lead byte by `__ismbblead_l` on the `_LocaleUpdate` object,
/// matches the pair). Answers where the scan stopped (on the delimiter that matched, or on
/// the terminating zero), whether a lead byte at the end of the delimiters stopped it, and
/// whether a pair matched.
fn mbstok_scan(e: &mut Engine, delimiters: u32, at: u32, update: u32) -> (u32, bool, bool) {
    let mut d = delimiters;
    while e.mem.u8(d) != 0 {
        let byte = e.mem.u8(d);
        if e.call(ISMBBLEAD_L, &args![u32::from(byte), update]).u32() != 0 {
            let next = d.wrapping_add(1);
            let second = e.mem.u8(next);
            if second == 0 {
                return (next, true, false);
            }
            if e.mem.u8(d) == e.mem.u8(at) && second == e.mem.u8(at.wrapping_add(1)) {
                return (d, false, true);
            }
            d = next;
        } else if e.mem.u8(d) == e.mem.u8(at) {
            return (d, false, false);
        }
        d = d.wrapping_add(1);
    }
    (d, false, false)
}

// Translated from 00edfa19 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_mbstok_s_l` (LIBCMT/mbstok_s.c, engine map), cdecl: (string, delimiters, context,
/// locale). A null context or delimiters, or a null string with a null `*context`, give
/// `errno` `EINVAL`, the invalid-parameter handler and null. With the locale taken
/// (`_LocaleUpdate`): a single-byte code page (`ismbcodepage` +8 clear) is `strtok_s`.
/// Otherwise, from the string (or `*context` when it is null), leading delimiters are
/// skipped (two-byte characters compared as pairs; a lead byte at the end of the
/// delimiters, or a lead byte ending the string, sets `errno` `EILSEQ` (0x2A) and ends the
/// skip); then the token runs to the next delimiter, which is overwritten with zeros (both
/// bytes of a two-byte one) and stepped over, or to the end of the string (a lead byte
/// before the end is zeroed). `*context` gets where the scan stopped; the answer is the
/// token's start, or null when the token is empty.
pub fn mbstok_s_l(e: &mut Engine, string: Ptr, delimiters: Ptr, context: Ptr, locale: u32) -> Ptr {
    let (mut at, delimiters, context) = (string.addr(), delimiters.addr(), context.addr());
    if context == 0 || delimiters == 0 {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return Ptr::new(0);
    }
    if at == 0 && e.mem.u32(context) == 0 {
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        return Ptr::new(0);
    }
    e.with_stack(0x10, |e, frame| {
        let update = frame.addr();
        e.call(LOCALE_UPDATE, &[update, locale]);
        let mbcinfo = e.mem.u32(update + 4);
        if e.mem.u32(mbcinfo + 8) == 0 {
            let token = e.call(STRTOK_S, &args![string, delimiters, context]).u32();
            locale_update_release(e, update);
            return Ptr::new(token);
        }
        if at == 0 {
            at = e.mem.u32(context);
        }
        // Skip the leading delimiters.
        while e.mem.u8(at) != 0 {
            let (d, lead_at_end, _) = mbstok_scan(e, delimiters, at, update);
            if lead_at_end {
                set_errno(e, 0x2a);
            }
            if e.mem.u8(d) == 0 {
                break;
            }
            let byte = e.mem.u8(at);
            if e.call(ISMBBLEAD_L, &args![u32::from(byte), update]).u32() != 0 {
                at = at.wrapping_add(1);
                if e.mem.u8(at) == 0 {
                    set_errno(e, 0x2a);
                    break;
                }
            }
            at = at.wrapping_add(1);
        }
        let start = at;
        // The token.
        while e.mem.u8(at) != 0 {
            let (d, _, pair) = mbstok_scan(e, delimiters, at, update);
            if e.mem.u8(d) != 0 {
                e.mem.set_u8(at, 0);
                at = at.wrapping_add(1);
                if pair {
                    e.mem.set_u8(at, 0);
                    at = at.wrapping_add(1);
                }
                break;
            }
            let byte = e.mem.u8(at);
            if e.call(ISMBBLEAD_L, &args![u32::from(byte), update]).u32() != 0 {
                let next = at.wrapping_add(1);
                if e.mem.u8(next) == 0 {
                    e.mem.set_u8(at, 0);
                    break;
                }
                at = next;
            }
            at = at.wrapping_add(1);
        }
        e.mem.set_u32(context, at);
        let token = if start == at { 0 } else { start };
        locale_update_release(e, update);
        Ptr::new(token)
    })
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_67d6, ascii_stricmp(Ptr, Ptr) -> i32),
        entry!(0x00ec_680f, stricmp_l(Ptr, Ptr, u32) -> i32),
        entry!(0x00ec_6695, tolower_l(u32, u32) -> u32),
        entry!(
            0x00ed_0883,
            crt_lc_map_string_a(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00ed_76fc, mbspbrk(Ptr, Ptr) -> Ptr),
        entry!(0x00ed_762f, mbspbrk_l(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00ed_761c, mbctolower(u32) -> u32),
        entry!(0x00ed_7576, mbctolower_l(u32, u32) -> u32),
        entry!(0x00ed_5c21, ismbbtype_l(u32, u32, u32, u32) -> u32),
        entry!(0x00ed_5c8d, ismbblead(u32) -> u32),
        entry!(0x00ec_8fb7, strlwr_s_l(Ptr, u32, u32) -> u32),
        entry!(0x00ec_8feb, strlwr_s(Ptr, u32) -> u32),
        entry!(0x00ed_5ca5, mbsdec_l(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00ed_5d3e, mbsdec(Ptr, Ptr) -> Ptr),
        entry!(0x00ee_248c, mbsrchr(Ptr, u32) -> Ptr),
        entry!(0x00ee_23d0, mbsrchr_l(Ptr, u32, u32) -> Ptr),
        entry!(0x00ed_7185, dtoxmode(u8, Ptr) -> u32),
        entry!(0x00ec_9162, is_root_unc_name(Ptr) -> u32),
        entry!(0x00ed_74ad, getdrive() -> u32),
        entry!(0x00ec_923c, stat64i32(Ptr, Ptr) -> i32),
        entry!(0x00ec_e23e, mbstok(Ptr, Ptr) -> Ptr),
        entry!(0x00ec_e21d, mbstok_l(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00ed_5c74, ismbblead_l(u32, u32) -> u32),
        entry!(0x00ed_fa19, mbstok_s_l(Ptr, Ptr, Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The multibyte information the doubled `_LocaleUpdate` hands out: `ismbcodepage` set and
    /// 0x81 a lead byte.
    const MBCINFO: u32 = 0x0e40_0000;

    /// An engine with `errno`'s thread block, the invalid-parameter handler and `_LocaleUpdate`
    /// doubled; answers it and the thread block.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        let block = e.mem.alloc(0x80);
        e.register_double(GETPTD_NOEXIT, move |_, _| block.into_ret());
        e.register(DECODE_POINTER, |_, _| 0x0e30_0000u32.into_ret());
        e.register(0x0e30_0000, |_, _| Ret::default());
        e.map(MBCINFO, 0x1000);
        e.mem.set_u32(MBCINFO + 8, 1);
        e.mem.set_u8(MBCINFO + 0x1d + 0x81, 4);
        e.register(LOCALE_UPDATE, |e, a| {
            e.mem.set_u32(a[0] + 4, MBCINFO);
            e.mem.set_u8(a[0] + 0xc, 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        (e, block)
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let at = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(at, s);
        at
    }

    #[test]
    fn mbsdec_steps_back_over_a_two_byte_character_whole() {
        let (mut e, _) = engine();
        // TEST DATA: 0x81 is the doubled locale's lead byte.
        let s = text(&mut e, b"ab\x81\x82c");
        assert_eq!(
            e.call(MBSDEC, &args![s, s + 4]).u32(),
            s + 2,
            "before c: the pair"
        );
        assert_eq!(e.call(MBSDEC, &args![s, s + 2]).u32(), s + 1);
        assert_eq!(e.call(MBSDEC, &args![s, s]).u32(), 0);
    }

    #[test]
    fn the_multibyte_searches_read_two_byte_characters_as_one() {
        let (mut e, _) = engine();
        let s = text(&mut e, b"a\x81.b.c");
        // The `.` inside the two-byte character is not one; the last `.` is.
        assert_eq!(e.call(MBSRCHR, &args![s, 0x2eu32]).u32(), s + 4);
        assert_eq!(e.call(MBSRCHR, &args![s, 0x812eu32]).u32(), s + 1);
        assert_eq!(e.call(MBSRCHR, &args![s, 0x7au32]).u32(), 0);
        let control = text(&mut e, b"?*");
        let wild = text(&mut e, br"Data\*.bsa");
        assert_eq!(e.call(MBSPBRK, &args![s, control]).u32(), 0);
        assert_eq!(e.call(MBSPBRK, &args![wild, control]).u32(), wild + 5);
        let marks = text(&mut e, b".");
        assert_eq!(e.call(MBSPBRK, &args![s, marks]).u32(), s + 4);
        // Upper-case single bytes map through `mbcasemap`.
        e.mem.set_u8(MBCINFO + 0x1d + 0x43, 0x10);
        e.mem.set_u8(MBCINFO + 0x11d + 0x43, 0x63);
        assert_eq!(e.call(MBCTOLOWER, &args![0x43u32]).u32(), 0x63);
        assert_eq!(e.call(MBCTOLOWER, &args![0x63u32]).u32(), 0x63);
    }

    #[test]
    fn dtoxmode_marks_directories_read_only_and_programs() {
        let (mut e, _) = engine();
        e.register(MBSICMP, |e, a| {
            let (x, y) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            u32::from(!x.eq_ignore_ascii_case(&y)).into_ret()
        });
        e.map(0x010f_3000, 0x1000);
        for (at, s) in [
            (TEXT_EXE, b".exe"),
            (TEXT_CMD, b".cmd"),
            (TEXT_BAT, b".bat"),
            (TEXT_COM, b".com"),
        ] {
            e.mem.set_cstr(at, s);
        }
        let archive = text(&mut e, br"C:\Data\Fallout - Meshes.bsa");
        assert_eq!(e.call(DTOXMODE, &args![0x20u32, archive]).u32(), 0x81b6);
        assert_eq!(e.call(DTOXMODE, &args![0x21u32, archive]).u32(), 0x8124);
        let program = text(&mut e, br"C:\FalloutNV.EXE");
        assert_eq!(e.call(DTOXMODE, &args![0x20u32, program]).u32(), 0x81ff);
        let root = text(&mut e, br"C:\");
        assert_eq!(e.call(DTOXMODE, &args![0u32, root]).u32(), 0x41ff);
    }

    #[test]
    fn root_unc_names_are_server_and_share() {
        let (mut e, _) = engine();
        for (s, root) in [
            (&br"\\server\share"[..], 1u32),
            (br"\\server\share\", 1),
            (b"//server/share", 1),
            (br"\\server\share\dir", 0),
            (br"\\server", 0),
            (br"C:\dir\x", 0),
            (br"\\\x\y", 0),
        ] {
            let at = text(&mut e, s);
            assert_eq!(e.call(IS_ROOT_UNC_NAME, &args![at]).u32(), root, "{s:?}");
        }
    }

    #[test]
    fn stat_describes_a_found_file() {
        let (mut e, _) = engine();
        e.register(MBSPBRK, |_, _| 0u32.into_ret());
        e.register(GETDRIVE, |_, _| 3u32.into_ret());
        e.register(IMPORT_FIND_FIRST_FILE_A, |e, a| {
            let d = a[1];
            e.mem.set_u32(d, 0x20);
            // Created and written; never accessed.
            e.mem.set_u32(d + 4, 0x1111);
            e.mem.set_u32(d + 0xc, 0);
            e.mem.set_u32(d + 0x10, 0);
            e.mem.set_u32(d + 0x14, 0x3333);
            e.mem.set_u32(d + 0x18, 0x4444);
            e.mem.set_u32(d + 0x1c, 0);
            e.mem.set_u32(d + 0x20, 0x3f47_1aab);
            0x700u32.into_ret()
        });
        e.register(IMPORT_FILE_TIME_TO_LOCAL_FILE_TIME, |e, a| {
            let low = e.mem.u32(a[0]);
            e.mem.set_u32(a[1], low);
            1u32.into_ret()
        });
        e.register(IMPORT_FILE_TIME_TO_SYSTEM_TIME, |e, a| {
            // The year carries the FILETIME's low word, to tell the times apart.
            let low = e.mem.u32(a[0]);
            for (offset, value) in [(0u32, low), (2, 6), (6, 15), (8, 1), (0xa, 2), (0xc, 3)] {
                e.mem.set_u16(a[1] + offset, value as u16);
            }
            1u32.into_ret()
        });
        e.register(LOCTOTIME64, |_, a| {
            ((u64::from(a[0]) << 32) | u64::from(a[1] * 100 + a[2])).into_ret()
        });
        e.register(DTOXMODE, |_, _| 0x81b6u32.into_ret());
        e.register(IMPORT_FIND_CLOSE, |_, _| 1u32.into_ret());
        let path = text(&mut e, br"Data\Fallout - Meshes.bsa");
        let out = e.mem.alloc(0x30);
        assert_eq!(e.call(0x00ec_923c, &args![path, out]).i32(), 0);
        let time =
            |e: &Engine, at: u32| u64::from(e.mem.u32(at)) | (u64::from(e.mem.u32(at + 4)) << 32);
        assert_eq!(time(&e, out + 0x20), (0x3333 << 32) | 615);
        assert_eq!(time(&e, out + 0x18), (0x3333 << 32) | 615);
        assert_eq!(time(&e, out + 0x28), (0x1111 << 32) | 615);
        assert_eq!(e.mem.u16(out + 6), 0x81b6);
        assert_eq!(e.mem.u16(out + 8), 1);
        assert_eq!(e.mem.u32(out + 0x14), 0x3f47_1aab);
        assert_eq!(e.mem.u32(out), 2);
        assert_eq!(e.mem.u32(out + 0x10), 2);
        let log = e.call_log.as_ref().unwrap();
        let closes: Vec<_> = log.iter().filter(|c| c.0 == IMPORT_FIND_CLOSE).collect();
        assert_eq!(closes.len(), 1);
        assert_eq!(closes[0].1, vec![0x700]);
    }

    #[test]
    fn stat_answers_enoent_for_wildcards_and_missing_files() {
        let (mut e, block) = engine();
        e.map(0x010f_1000, 0x1000);
        e.mem.set_cstr(TEXT_WILDCARDS, b"?*");
        e.mem.set_cstr(TEXT_PATH_MARKS, b"./\\");
        let path = text(&mut e, br"Data\*.bsa");
        let out = e.mem.alloc(0x30);
        e.register(MBSPBRK, |e, a| {
            let s = e.mem.cstr(a[0]);
            let set = e.mem.cstr(a[1]);
            let at = s.iter().position(|c| set.contains(c));
            at.map_or(0, |i| a[0] + i as u32).into_ret()
        });
        assert_eq!(e.call(0x00ec_923c, &args![path, out]).i32(), -1);
        assert_eq!(e.mem.u32(block + 8), 2);
        e.mem.set_u32(block + 8, 0);
        e.register(GETDRIVE, |_, _| 3u32.into_ret());
        e.register(IMPORT_FIND_FIRST_FILE_A, |_, _| 0xffff_ffffu32.into_ret());
        let missing = text(&mut e, b"NoSuch");
        assert_eq!(e.call(0x00ec_923c, &args![missing, out]).i32(), -1);
        assert_eq!(e.mem.u32(block + 8), 2);
        assert_eq!(e.mem.u32(block + 0xc), 2);
        // A drive letter alone; a null out.
        let drive = text(&mut e, b"C:");
        assert_eq!(e.call(0x00ec_923c, &args![drive, out]).i32(), -1);
        assert_eq!(e.call(0x00ec_923c, &args![drive, 0u32]).i32(), -1);
        assert_eq!(e.mem.u32(block + 8), 0x16);
    }

    /// The locale information the doubled `_LocaleUpdate` (`locale_info`) hands out at +0: an
    /// `LC_CTYPE` handle (+0x14), `mb_cur_max` (+0xAC), a ctype table (+0xC8) marking `A`..`Z`
    /// upper case and a lower-case map (+0xCC).
    const LOCINFO: u32 = 0x0e50_0000;

    fn locale_info(e: &mut Engine, handle: u32, mb_cur_max: u32) {
        e.map(LOCINFO, 0x2000);
        e.mem.set_u32(LOCINFO + 0x14, handle);
        e.mem.set_u32(LOCINFO + 0xac, mb_cur_max);
        e.mem.set_u32(LOCINFO + 0xc8, LOCINFO + 0x400);
        e.mem.set_u32(LOCINFO + 0xcc, LOCINFO + 0x800);
        for c in 0..0x100u32 {
            let upper = u16::from((0x41..=0x5a).contains(&c));
            e.mem.set_u16(LOCINFO + 0x400 + c * 2, upper);
            let lower = if upper != 0 { c + 0x20 } else { c };
            e.mem.set_u8(LOCINFO + 0x800 + c, lower as u8);
        }
        e.register(LOCALE_UPDATE, |e, a| {
            e.mem.set_u32(a[0], LOCINFO);
            e.mem.set_u32(a[0] + 4, MBCINFO);
            e.mem.set_u8(a[0] + 0xc, 0);
            Ret::default()
        });
    }

    #[test]
    fn ascii_stricmp_folds_only_capital_letters() {
        let (mut e, _) = engine();
        let a = text(&mut e, b"Data.BSA");
        let b = text(&mut e, b"data.bsa");
        assert_eq!(e.call(0x00ec_67d6, &args![a, b]).i32(), 0);
        let a = text(&mut e, b"a");
        let b = text(&mut e, b"B");
        assert_eq!(e.call(0x00ec_67d6, &args![a, b]).i32(), -1);
        // `[` (0x5B) is not folded; `a` is 0x61.
        let a = text(&mut e, b"[");
        let b = text(&mut e, b"A");
        assert_eq!(e.call(0x00ec_67d6, &args![a, b]).i32(), -6);
        let a = text(&mut e, b"abc");
        let b = text(&mut e, b"AB");
        assert_eq!(e.call(0x00ec_67d6, &args![a, b]).i32(), 0x63);
    }

    #[test]
    fn stricmp_l_uses_the_ascii_compare_in_the_c_locale_and_tolower_otherwise() {
        let (mut e, block) = engine();
        locale_info(&mut e, 0, 1);
        let a = text(&mut e, b".EXE");
        let b = text(&mut e, b".exe");
        assert_eq!(e.call(0x00ec_680f, &args![a, b, 0u32]).i32(), 0);
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|c| c.0 == 0x00ec_6695));
        // A null string: EINVAL and 0x7fffffff.
        assert_eq!(
            e.call(0x00ec_680f, &args![a, 0u32, 0u32]).i32(),
            0x7fff_ffff
        );
        assert_eq!(e.mem.u32(block + 8), 0x16);
        e.mem.set_u32(block + 8, 0);
        assert_eq!(
            e.call(0x00ec_680f, &args![0u32, b, 0u32]).i32(),
            0x7fff_ffff
        );
        assert_eq!(e.mem.u32(block + 8), 0x16);
        // Another locale: through the ctype table and the lower-case map.
        locale_info(&mut e, 0x409, 1);
        let a = text(&mut e, b"ABc");
        let b = text(&mut e, b"abD");
        assert_eq!(e.call(0x00ec_680f, &args![a, b, 0u32]).i32(), -1);
        let b = text(&mut e, b"abC");
        assert_eq!(e.call(0x00ec_680f, &args![a, b, 0u32]).i32(), 0);
    }

    #[test]
    fn tolower_l_maps_bytes_and_sends_wider_characters_to_lcmapstring() {
        let (mut e, block) = engine();
        locale_info(&mut e, 0x409, 1);
        assert_eq!(e.call(0x00ec_6695, &args![0x51u32, 0u32]).u32(), 0x71);
        assert_eq!(e.call(0x00ec_6695, &args![0x71u32, 0u32]).u32(), 0x71);
        // mb_cur_max above 1: the upper-case test goes through `__isctype_l`.
        locale_info(&mut e, 0x409, 2);
        e.register(ISCTYPE_L, |_, a| {
            u32::from(a[0] == 0x51 && a[1] == 1).into_ret()
        });
        assert_eq!(e.call(0x00ec_6695, &args![0x51u32, 0u32]).u32(), 0x71);
        // A wider character whose high byte is a lead byte: two bytes to LCMapString.
        e.register(ISLEADBYTE_L, |_, a| u32::from(a[0] == 0x81).into_ret());
        e.register(CRT_LC_MAP_STRING_A, |e, a| {
            assert_eq!((a[1], a[2], a[6], a[8]), (0x409, 0x100, 3, 1));
            let bytes = e.mem.cstr(a[3]);
            e.mem.set_u8(a[5], b'x');
            e.mem.set_u8(a[5] + 1, b'y');
            (if bytes == [0x81, 0x41] { 2u32 } else { 1 }).into_ret()
        });
        assert_eq!(e.call(0x00ec_6695, &args![0x8141u32, 0u32]).u32(), 0x7879);
        assert_eq!(e.mem.u32(block + 8), 0);
        // Not a lead byte: EILSEQ and the low byte alone.
        assert_eq!(e.call(0x00ec_6695, &args![0x4241u32, 0u32]).u32(), 0x78);
        assert_eq!(e.mem.u32(block + 8), 0x2a);
        // No mapping: the character stays.
        e.register(CRT_LC_MAP_STRING_A, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ec_6695, &args![0x8141u32, 0u32]).u32(), 0x8141);
    }

    #[test]
    fn crt_lc_map_string_a_passes_the_update_object_and_its_words() {
        let (mut e, _) = engine();
        locale_info(&mut e, 0x409, 1);
        e.register(CRT_LC_MAP_STRING_A_STAT, |e, a| {
            assert_eq!(e.mem.u32(a[0]), LOCINFO);
            assert_eq!(&a[1..], &[2, 3, 4, 5, 6, 7, 8, 9]);
            0x55u32.into_ret()
        });
        let r = e.call(
            0x00ed_0883,
            &args![0u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32, 9u32],
        );
        assert_eq!(r.u32(), 0x55);
    }
}

#[cfg(test)]
mod tests_ismbb {
    use super::*;

    #[test]
    fn lead_bytes_and_ctype_bits_come_from_the_locale_tables() {
        let mut e = Engine::new();
        // TEST DATA: a multibyte table with 0x81 a lead byte; a ctype table with 0x41 marked 0x100.
        let mbcinfo = e.mem.alloc(0x200);
        e.mem.set_u8(mbcinfo + 0x1d + 0x81, 4);
        let ctype = e.mem.alloc(0x200);
        e.mem.set_u16(ctype + 2 * 0x41, 0x100);
        let locinfo = e.mem.alloc(0x100);
        e.mem.set_u32(locinfo + 0xc8, ctype);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0], locinfo);
            e.mem.set_u32(a[0] + 4, mbcinfo);
            e.mem.set_u8(a[0] + 0xc, 0);
            Ret::default()
        });
        assert_eq!(e.call(ISMBBLEAD, &args![0x81u32]).u32(), 1);
        assert_eq!(
            e.call(ISMBBLEAD, &args![0x4181u32]).u32(),
            1,
            "the low byte counts"
        );
        assert_eq!(e.call(ISMBBLEAD, &args![0x41u32]).u32(), 0);
        assert_eq!(
            e.call(ISMBBTYPE_L, &args![0u32, 0x41u32, 0x100u32, 4u32])
                .u32(),
            1
        );
        assert_eq!(
            e.call(ISMBBTYPE_L, &args![0u32, 0x41u32, 0x200u32, 4u32])
                .u32(),
            0
        );
    }
}

#[cfg(test)]
mod tests_strlwr {
    use super::*;

    #[test]
    fn strlwr_s_takes_the_locale_and_hands_on_to_the_body() {
        let mut e = Engine::new();
        let update_seen = std::rc::Rc::new(std::cell::Cell::new(0u32));
        let seen = update_seen.clone();
        e.register_double(LOCALE_UPDATE, move |e, a| {
            seen.set(a[0]);
            e.mem.set_u8(a[0] + 0xc, 0);
            Ret::default()
        });
        let calls = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let record = calls.clone();
        e.register_double(STRLWR_S_L_STAT, move |_, a| {
            record.borrow_mut().push(a.to_vec());
            0x22u32.into_ret()
        });
        assert_eq!(e.call(0x00ec_8feb, &args![0x1000u32, 9u32]).u32(), 0x22);
        let calls = calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(&calls[0][..2], &[0x1000, 9]);
        assert_eq!(calls[0][2], update_seen.get());
    }
}

#[cfg(test)]
mod mbstok_tests {
    use super::*;

    /// The multibyte information the doubled `_LocaleUpdate` hands out: `ismbcodepage` set
    /// and 0x81 a lead byte.
    const MBCINFO: u32 = 0x0e40_0000;
    /// The thread data block `_getptd` answers (the token position at +0x20).
    const THREAD: u32 = 0x0e50_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(THREAD, 0x100);
        e.register(GETPTD_NOEXIT, |_, _| THREAD.into_ret());
        e.register(GETPTD_ADDR, |_, _| THREAD.into_ret());
        e.register(DECODE_POINTER, |_, _| 0x0e30_0000u32.into_ret());
        e.register(0x0e30_0000, |_, _| Ret::default());
        e.map(MBCINFO, 0x1000);
        e.mem.set_u32(MBCINFO + 8, 1);
        e.mem.set_u8(MBCINFO + 0x1d + 0x81, 4);
        e.register(LOCALE_UPDATE, |e, a| {
            e.mem.set_u32(a[0] + 4, MBCINFO);
            e.mem.set_u8(a[0] + 0xc, 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn text(e: &mut Engine, s: &[u8]) -> u32 {
        let at = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(at, s);
        at
    }

    fn errno_now(e: &mut Engine) -> u32 {
        let slot = errno(e);
        e.mem.u32(slot.addr())
    }

    #[test]
    fn mbstok_splits_on_one_and_two_byte_delimiters() {
        let mut e = engine();
        // TEST DATA: delimiters are the comma and the two-byte character 0x81 0x82.
        let s = text(&mut e, b",ab,\x81\x82cd\x81\x82e");
        let delimiters = text(&mut e, b",\x81\x82");
        assert_eq!(e.call(0x00ec_e23e, &args![s, delimiters]).u32(), s + 1);
        assert_eq!(e.mem.cstr(s + 1), b"ab");
        assert_eq!(e.mem.u32(THREAD + 0x20), s + 4);
        assert_eq!(e.call(0x00ec_e23e, &args![0u32, delimiters]).u32(), s + 6);
        assert_eq!(e.mem.cstr(s + 6), b"cd");
        assert_eq!(&e.mem.bytes(s + 8, 2)[..], &[0, 0]);
        assert_eq!(e.mem.u32(THREAD + 0x20), s + 10);
        assert_eq!(e.call(0x00ec_e23e, &args![0u32, delimiters]).u32(), s + 10);
        assert_eq!(e.mem.cstr(s + 10), b"e");
        assert_eq!(e.call(0x00ec_e23e, &args![0u32, delimiters]).u32(), 0);
        assert_eq!(e.mem.u32(THREAD + 0x20), s + 11);
        // Each call reached `_mbstok_s_l` with the thread's token position.
        let log = e.call_log.as_ref().unwrap();
        let contexts: Vec<u32> = log
            .iter()
            .filter(|c| c.0 == MBSTOK_S_L)
            .map(|c| c.1[2])
            .collect();
        assert_eq!(contexts, vec![THREAD + 0x20; 4]);
    }

    #[test]
    fn a_two_byte_character_inside_a_token_is_not_split() {
        let mut e = engine();
        // 0x81 0x2c is one character, though its second byte is a comma.
        let s = text(&mut e, b"a\x81,b,c");
        let delimiters = text(&mut e, b",");
        let context = e.mem.alloc(4);
        let words = |s: u32| args![s, delimiters, context, 0u32];
        assert_eq!(e.call(0x00ed_fa19, &words(s)).u32(), s);
        assert_eq!(e.mem.cstr(s), b"a\x81,b");
        assert_eq!(e.call(0x00ed_fa19, &words(0)).u32(), s + 5);
        assert_eq!(e.mem.cstr(s + 5), b"c");
    }

    #[test]
    fn lead_bytes_at_the_ends_set_eilseq_or_end_the_token() {
        let mut e = engine();
        let context = e.mem.alloc(4);
        // A lead byte ending the delimiters: EILSEQ, and the skip ends.
        let s = text(&mut e, b"a");
        let delimiters = text(&mut e, b",\x81");
        assert_eq!(
            e.call(0x00ed_fa19, &args![s, delimiters, context, 0u32])
                .u32(),
            s
        );
        assert_eq!(errno_now(&mut e), 0x2a);
        // A lead byte ending the string inside a token is zeroed.
        let slot = errno(&mut e).addr();
        e.mem.set_u32(slot, 0);
        let s = text(&mut e, b"ab\x81");
        let delimiters = text(&mut e, b",");
        assert_eq!(
            e.call(0x00ed_fa19, &args![s, delimiters, context, 0u32])
                .u32(),
            s
        );
        assert_eq!(e.mem.u8(s + 2), 0);
        assert_eq!(e.mem.u32(context), s + 2);
        assert_eq!(errno_now(&mut e), 0);
        // A lone lead byte after a delimiter: zeroed, an empty token.
        let s = text(&mut e, b",\x81");
        assert_eq!(
            e.call(0x00ed_fa19, &args![s, delimiters, context, 0u32])
                .u32(),
            0
        );
        assert_eq!(e.mem.u8(s + 1), 0);
        assert_eq!(e.mem.u32(context), s + 1);
    }

    #[test]
    fn bad_arguments_and_single_byte_code_pages() {
        let mut e = engine();
        let s = text(&mut e, b"a,b");
        let delimiters = text(&mut e, b",");
        let context = e.mem.alloc(4);
        e.mem.set_u32(context, 0);
        for words in [
            args![s, delimiters, 0u32, 0u32],
            args![s, 0u32, context, 0u32],
            args![0u32, delimiters, context, 0u32],
        ] {
            let slot = errno(&mut e).addr();
            e.mem.set_u32(slot, 0);
            assert_eq!(e.call(0x00ed_fa19, &words).u32(), 0);
            assert_eq!(errno_now(&mut e), 0x16);
        }
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .all(|c| c.0 != LOCALE_UPDATE));
        // A single-byte code page hands the work to `strtok_s`.
        e.mem.set_u32(MBCINFO + 8, 0);
        e.register(STRTOK_S, |_, a| (a[0] + 0x100).into_ret());
        assert_eq!(
            e.call(0x00ed_fa19, &args![s, delimiters, context, 0u32])
                .u32(),
            s + 0x100
        );
        let calls: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == STRTOK_S)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(calls, vec![vec![s, delimiters, context]]);
    }

    #[test]
    fn ismbblead_l_asks_the_lead_byte_type() {
        let mut e = engine();
        assert_eq!(e.call(0x00ed_5c74, &args![0x81u32, 0u32]).u32(), 1);
        assert_eq!(e.call(0x00ed_5c74, &args![0x41u32, 0u32]).u32(), 0);
        let calls: Vec<Vec<u32>> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == ISMBBTYPE_L)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(calls, vec![vec![0, 0x81, 0, 4], vec![0, 0x41, 0, 4]]);
    }
}
