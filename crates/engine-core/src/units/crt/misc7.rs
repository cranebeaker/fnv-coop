//! More C runtime pieces (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe 1.4.0.525),
//! translated from the exe: the small-block heap's `__sbh_find_block` (`00ede233`) and
//! `__sbh_alloc_new_group` (`00ede629`), `_ValidateRead` (`00edfbe8`), and the wide open
//! `_wsopen_s` (`00ee58db`) with `_wsopen_helper` (`00ee580f`) and its `__finally` block
//! (`00ee58ad`), the stream opener `_wopenfile` (`00edfbfa`), and the locale readers
//! `__get_lc_time` (`00edfeae`) and `__init_monetary` (`00ee0774`).
//!
//! Every call is made by address, as the exe makes it, so a test can double each one.

use crate::prelude::*;

/// `__sbh_cntHeaderList` (`01270930`) and `__sbh_pHeaderList` (`01270934`): the small-block
/// heap's region headers, 0x14 bytes each (`+0x0C` the region's base, `+0x10` its group data).
const SBH_HEADER_COUNT: u32 = 0x0127_0930;
const SBH_HEADER_LIST: u32 = 0x0127_0934;
/// The `VirtualAlloc` import slot (`00fdf114`, stdcall: address, size, type, protection).
const VIRTUAL_ALLOC: u32 = 0x00fd_f114;
/// `_errno` (`00ec85e3`), `_invalid_parameter` (`00ec7c30`, five words), `_tsopen_nolock`
/// (`00ee50ef`; the handle's address in `EAX`, then the unlock flag's address, name, open
/// flags, share flags, permission), `_unlock_fhandle` (`00ee2f5c`, cdecl, one word), the
/// `__finally` block of `_wsopen_helper` (`00ee58ad`) and `_wsopen_helper` (`00ee580f`).
const ERRNO: u32 = 0x00ec_85e3;
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
const TSOPEN_NOLOCK: u32 = 0x00ee_50ef;
const UNLOCK_FHANDLE: u32 = 0x00ee_2f5c;
const WSOPEN_HELPER_FINALLY: u32 = 0x00ee_58ad;
const WSOPEN_HELPER: u32 = 0x00ee_580f;
/// `__pioinfo` (`01270960`, 32 blocks of 0x40-byte entries).
const PIOINFO: u32 = 0x0127_0960;

// Translated from 00ede233 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__sbh_find_block` (LIBCMT/sbheap.c; cdecl: the block): the first region header whose base
/// (+0x0C) lies within 1 MB below the block (`block - base < 0x100000`, unsigned), or 0.
pub fn sbh_find_block(e: &mut Engine, block: u32) -> u32 {
    let end = e
        .mem
        .u32(SBH_HEADER_COUNT)
        .wrapping_mul(0x14)
        .wrapping_add(e.mem.u32(SBH_HEADER_LIST));
    let mut header = e.mem.u32(SBH_HEADER_LIST);
    while header < end {
        if block.wrapping_sub(e.mem.u32(header.wrapping_add(0xc))) < 0x10_0000 {
            return header;
        }
        header = header.wrapping_add(0x14);
    }
    0
}

// Translated from 00ede629 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__sbh_alloc_new_group` (LIBCMT/sbheap.c; cdecl: the region header). The group is the
/// first one free in the header's group mask (+8, the highest set bit; the exe's search never
/// ends for an empty mask, as here). Its 63 list heads (0x204 bytes per group from region data
/// +0x144) are made empty, its eight pages (32 KB at base + group * 0x8000) committed
/// (`VirtualAlloc`, `MEM_COMMIT`, `PAGE_READWRITE`; -1 when that fails) and each made one free
/// block of 0xFF0 bytes with end markers, all of them linked on the group's last list
/// (+0x1F8). The group's bit is set in the region's last-list masks (`+0x44` cleared, `+0xC4`
/// 1), its count of used groups (`+0x43`) goes up (setting bit 0 of the header's first mask,
/// +4, when it was 0) and its bit leaves the header's group mask. Answers the group index.
pub fn sbh_alloc_new_group(e: &mut Engine, header: u32) -> i32 {
    let mut mask = e.mem.u32(header.wrapping_add(8));
    let region = e.mem.u32(header.wrapping_add(0x10));
    let mut index = 0u32;
    while (mask as i32) >= 0 {
        mask = mask.wrapping_add(mask);
        index = index.wrapping_add(1);
    }
    let group = index
        .wrapping_mul(0x204)
        .wrapping_add(region)
        .wrapping_add(0x144);
    let mut head = group;
    for _ in 0..0x3f {
        e.mem.set_u32(head.wrapping_add(8), head);
        e.mem.set_u32(head.wrapping_add(4), head);
        head = head.wrapping_add(8);
    }
    let first = (index << 15).wrapping_add(e.mem.u32(header.wrapping_add(0xc)));
    if e.call(VIRTUAL_ALLOC, &args![first, 0x8000u32, 0x1000u32, 4u32])
        .u32()
        == 0
    {
        return -1;
    }
    let last = first.wrapping_add(0x7000);
    if first <= last {
        let mut pages = (last.wrapping_sub(first) >> 12).wrapping_add(1);
        let mut at = first.wrapping_add(0x10);
        loop {
            e.mem.set_u32(at.wrapping_sub(8), u32::MAX);
            e.mem.set_u32(at.wrapping_add(0xfec), u32::MAX);
            e.mem.set_u32(at, at.wrapping_add(0xffc));
            e.mem.set_u32(at.wrapping_sub(4), 0xff0);
            e.mem.set_u32(at.wrapping_add(4), at.wrapping_sub(0x1004));
            e.mem.set_u32(at.wrapping_add(0xfe8), 0xff0);
            at = at.wrapping_add(0x1000);
            pages = pages.wrapping_sub(1);
            if pages == 0 {
                break;
            }
        }
    }
    let list = group.wrapping_add(0x1f8);
    let first_block = first.wrapping_add(0xc);
    e.mem.set_u32(list.wrapping_add(4), first_block);
    e.mem.set_u32(first_block.wrapping_add(8), list);
    let last_block = last.wrapping_add(0xc);
    e.mem.set_u32(list.wrapping_add(8), last_block);
    e.mem.set_u32(last_block.wrapping_add(4), list);
    let slot = region.wrapping_add(index.wrapping_mul(4));
    e.mem.set_u32(slot.wrapping_add(0x44), 0);
    e.mem.set_u32(slot.wrapping_add(0xc4), 1);
    let used = e.mem.u8(region.wrapping_add(0x43));
    e.mem
        .set_u8(region.wrapping_add(0x43), used.wrapping_add(1));
    if used == 0 {
        let word = e.mem.u32(header.wrapping_add(4)) | 1;
        e.mem.set_u32(header.wrapping_add(4), word);
    }
    // SHR EDX, CL: the shift count is the index's low five bits.
    let bit = 0x8000_0000u32 >> (index & 0x1f);
    let groups = e.mem.u32(header.wrapping_add(8)) & !bit;
    e.mem.set_u32(header.wrapping_add(8), groups);
    index as i32
}

// Translated from 00edfbe8 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_ValidateRead` (LIBCMT/eh3valid.c; cdecl: address, size): 1 for a non-null address, else
/// 0. The size is not read.
pub fn validate_read(_e: &mut Engine, address: u32, _size: u32) -> u32 {
    u32::from(address != 0)
}

// Translated from 00ee58db (disassembled, FalloutNV.exe 1.4.0.525)
/// `_wsopen_s` (LIBCMT/wopen.c; cdecl: handle out, name, open flags, share flags,
/// permission): `_wsopen_helper(name, flags, share, permission, handle, 1)`, whose `EAX` is
/// answered.
pub fn wsopen_s(e: &mut Engine, handle: u32, name: u32, flags: u32, share: u32, mode: u32) -> u32 {
    e.call(
        WSOPEN_HELPER,
        &args![name, flags, share, mode, handle, 1u32],
    )
    .u32()
}

// Translated from 00ee580f (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee580f - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 01180610) is not translated; its __finally block (00ee58ad) is called as the exe calls it on the way out.
/// `_wsopen_helper` (LIBCMT/wopen.c; cdecl: name, open flags, share flags, permission, handle
/// out, secure). A null handle address, a null name, or for a secure call a permission with
/// bits other than `_S_IREAD | _S_IWRITE` (0x180) sets `errno` to `EINVAL` and answers it
/// (0x16); the handle is set to -1 first once its address is known. Otherwise
/// `_tsopen_nolock` opens it with a local unlock flag (0 at first); the `__finally` block
/// releases the handle, and a non-zero result sets the handle to -1 again. Answers that result.
pub fn wsopen_helper(
    e: &mut Engine,
    name: u32,
    flags: u32,
    share: u32,
    mode: u32,
    handle: u32,
    secure: u32,
) -> u32 {
    let refuse = |e: &mut Engine| {
        let errno = e.call(ERRNO, &args![]).u32();
        e.mem.set_u32(errno, 0x16);
        e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        0x16u32
    };
    if handle == 0 {
        return refuse(e);
    }
    e.mem.set_u32(handle, u32::MAX);
    if name == 0 || (secure != 0 && mode & 0xffff_fe7f != 0) {
        return refuse(e);
    }
    e.with_stack(4, |e, unlock| {
        let unlock = unlock.addr();
        e.mem.set_u32(unlock, 0);
        let result = e
            .call(
                TSOPEN_NOLOCK,
                &args![handle, unlock, name, flags, share, mode],
            )
            .u32();
        let flag = e.mem.u32(unlock);
        e.call(WSOPEN_HELPER_FINALLY, &args![flag, result, handle]);
        if result != 0 {
            e.mem.set_u32(handle, u32::MAX);
        }
        result
    })
}

// Translated from 00ee58ad (disassembled, FalloutNV.exe 1.4.0.525)
/// The `__finally` block of `_wsopen_helper` (`00ee580f`): its frame words are the arguments,
/// the unlock flag (`[EBP-0x1C]`), the open's result (`[EBP-0x20]`) and the handle's address
/// (`EDI`); `ESI` is 0 in that frame. With the flag set, a failed open clears the handle's
/// `FOPEN` bit (bit 0 of `osfile`, entry +4) and the handle is unlocked (`_unlock_fhandle`).
pub fn wsopen_helper_finally(e: &mut Engine, unlock: u32, result: u32, handle: u32) {
    if unlock == 0 {
        return;
    }
    if result != 0 {
        let fh = e.mem.u32(handle);
        let block = e.mem.u32(
            (((fh as i32) >> 5) as u32)
                .wrapping_mul(4)
                .wrapping_add(PIOINFO),
        );
        let osfile = block.wrapping_add((fh & 0x1f) << 6).wrapping_add(4);
        let byte = e.mem.u8(osfile) & 0xfe;
        e.mem.set_u8(osfile, byte);
    }
    let fh = e.mem.u32(handle);
    e.call(UNLOCK_FHANDLE, &args![fh]);
}

/// `_commode` (`012708d8`), `_cflush` (`0126fdf0`), `wcsncmp` (`00ee5a8b`),
/// `_wcsnicmp` (`00ee59e5`) and `_wsopen_s` (`00ee58db`), cdecl; the wide texts `_wopenfile`
/// compares the `ccs=` part with: `L"ccs"` (`010fe050`), `L"UTF-8"` (`010fe058`),
/// `L"UTF-16LE"` (`010fe064`) and `L"UNICODE"` (`010fe078`).
const COMMODE: u32 = 0x0127_08d8;
const CFLUSH: u32 = 0x0126_fdf0;
const WCSNCMP: u32 = 0x00ee_5a8b;
const WCSNICMP: u32 = 0x00ee_59e5;
const WSOPEN_S: u32 = 0x00ee_58db;
const CCS_TEXT: u32 = 0x010f_e050;
const UTF8_TEXT: u32 = 0x010f_e058;
const UTF16LE_TEXT: u32 = 0x010f_e064;
const UNICODE_TEXT: u32 = 0x010f_e078;

/// `_wopenfile`'s skip of wide spaces (0x20) from `at`.
fn skip_spaces(e: &Engine, mut at: u32) -> u32 {
    while e.mem.u16(at) == 0x20 {
        at = at.wrapping_add(2);
    }
    at
}

// Translated from 00edfbfa (disassembled, FalloutNV.exe 1.4.0.525)
/// `_wopenfile` (LIBCMT/_wopen.c; cdecl: name, mode text, share flags, stream). After leading
/// spaces the mode starts with `r` (`_O_RDONLY`, stream flag `_IOREAD`), `w` (0x301:
/// `_O_WRONLY | _O_CREAT | _O_TRUNC`) or `a` (0x109: `_O_WRONLY | _O_CREAT | _O_APPEND`), both
/// `_IOWRT`; the stream flags start from `_commode`. The modifiers follow until the text ends
/// or one repeats or conflicts (that one is passed over and the rest left): `+` read and
/// write (`_O_RDWR`, `_IORW`), `t`/`b` text or binary, `c`/`n` commit or not, `S`/`R`
/// sequential or random, `T` short-lived, `D` temporary, `N` no inheritance, a space nothing,
/// and `,` which asks for `ccs=UTF-8`, `ccs=UTF-16LE` or `ccs=UNICODE` (case-blind, spaces
/// around `=`) after it. Anything else, or text left over, sets `errno` to `EINVAL` and
/// answers 0. `_wsopen_s(handle, name, flags, share, _S_IREAD | _S_IWRITE)` opens it (0 when
/// that fails); `_cflush` is counted up and the stream set up (flags, pointer, count, base,
/// buffer size cleared, the handle). Answers the stream.
pub fn wopenfile(e: &mut Engine, name: u32, mode: u32, share: u32, stream: u32) -> u32 {
    let invalid = |e: &mut Engine| {
        let errno = e.call(ERRNO, &args![]).u32();
        e.mem.set_u32(errno, 0x16);
        e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        0u32
    };
    let mut stream_flags = e.mem.u32(COMMODE);
    let mut commit_seen = false;
    let mut order_seen = false;
    let mut ccs = false;
    let mut p = skip_spaces(e, mode);
    let mut flags = match e.mem.u16(p) {
        0x61 => {
            stream_flags |= 2;
            0x109u32
        }
        0x72 => {
            stream_flags |= 1;
            0
        }
        0x77 => {
            stream_flags |= 2;
            0x301
        }
        _ => return invalid(e),
    };
    let mut going = true;
    p = p.wrapping_add(2);
    let mut c = e.mem.u16(p);
    if c != 0 {
        while going {
            match c {
                0x20 => {}
                0x2b => {
                    if flags & 2 != 0 {
                        going = false;
                    } else {
                        flags = (flags & 0xffff_fffe) | 2;
                        stream_flags = (stream_flags & 0xffff_fffc) | 0x80;
                    }
                }
                0x2c => {
                    ccs = true;
                    going = false;
                }
                0x44 => {
                    if flags & 0x40 != 0 {
                        going = false;
                    } else {
                        flags |= 0x40;
                    }
                }
                0x4e => flags |= 0x80,
                0x52 | 0x53 => {
                    if order_seen {
                        going = false;
                    } else {
                        order_seen = true;
                        flags |= if c == 0x52 { 0x10 } else { 0x20 };
                    }
                }
                0x54 => {
                    if flags & 0x1000 != 0 {
                        going = false;
                    } else {
                        flags |= 0x1000;
                    }
                }
                0x62 | 0x74 => {
                    if flags & 0xc000 != 0 {
                        going = false;
                    } else {
                        flags |= if c == 0x62 { 0x8000 } else { 0x4000 };
                    }
                }
                0x63 | 0x6e => {
                    if commit_seen {
                        going = false;
                    } else {
                        if c == 0x63 {
                            stream_flags |= 0x4000;
                        } else {
                            stream_flags &= 0xffff_bfff;
                        }
                        commit_seen = true;
                    }
                }
                _ => return invalid(e),
            }
            p = p.wrapping_add(2);
            c = e.mem.u16(p);
            if c == 0 {
                break;
            }
        }
        if ccs {
            p = skip_spaces(e, p);
            if e.call(WCSNCMP, &args![CCS_TEXT, p, 3u32]).u32() != 0 {
                return invalid(e);
            }
            p = skip_spaces(e, p.wrapping_add(6));
            if e.mem.u16(p) != 0x3d {
                return invalid(e);
            }
            p = skip_spaces(e, p.wrapping_add(2));
            if e.call(WCSNICMP, &args![p, UTF8_TEXT, 5u32]).u32() == 0 {
                p = p.wrapping_add(0xa);
                flags |= 0x4_0000;
            } else if e.call(WCSNICMP, &args![p, UTF16LE_TEXT, 8u32]).u32() == 0 {
                p = p.wrapping_add(0x10);
                flags |= 0x2_0000;
            } else if e.call(WCSNICMP, &args![p, UNICODE_TEXT, 7u32]).u32() == 0 {
                p = p.wrapping_add(0xe);
                flags |= 0x1_0000;
            } else {
                return invalid(e);
            }
        }
    }
    p = skip_spaces(e, p);
    if e.mem.u16(p) != 0 {
        return invalid(e);
    }
    e.with_stack(4, |e, handle| {
        let handle = handle.addr();
        e.mem.set_u32(handle, mode);
        if e.call(WSOPEN_S, &args![handle, name, flags, share, 0x180u32])
            .u32()
            != 0
        {
            return 0;
        }
        let opened = e.mem.u32(CFLUSH).wrapping_add(1);
        e.mem.set_u32(CFLUSH, opened);
        e.mem.set_u32(stream.wrapping_add(0xc), stream_flags);
        let fh = e.mem.u32(handle);
        e.mem.set_u32(stream.wrapping_add(4), 0);
        e.mem.set_u32(stream, 0);
        e.mem.set_u32(stream.wrapping_add(8), 0);
        e.mem.set_u32(stream.wrapping_add(0x1c), 0);
        e.mem.set_u32(stream.wrapping_add(0x10), fh);
        stream
    })
}

/// `__getlocaleinfo` (`00ed2af0`, cdecl: locale object, kind, LCID, LCTYPE, destination).
const GETLOCALEINFO: u32 = 0x00ed_2af0;

/// The `LC_TIME` fields `__get_lc_time` fills, in its order: the field's offset in the
/// `__lc_time_data`, the `LCTYPE` asked, and whether the `LCID` is the information's word
/// at +0x44 rather than the one at +0x42. All are asked as text (kind 1).
const LC_TIME_FIELDS: [(u32, u32, bool); 43] = [
    (0x04, 0x31, false),
    (0x08, 0x32, false),
    (0x0c, 0x33, false),
    (0x10, 0x34, false),
    (0x14, 0x35, false),
    (0x18, 0x36, false),
    (0x00, 0x37, false),
    (0x20, 0x2a, false),
    (0x24, 0x2b, false),
    (0x28, 0x2c, false),
    (0x2c, 0x2d, false),
    (0x30, 0x2e, false),
    (0x34, 0x2f, false),
    (0x1c, 0x30, false),
    (0x38, 0x44, false),
    (0x3c, 0x45, false),
    (0x40, 0x46, false),
    (0x44, 0x47, false),
    (0x48, 0x48, false),
    (0x4c, 0x49, false),
    (0x50, 0x4a, false),
    (0x54, 0x4b, false),
    (0x58, 0x4c, false),
    (0x5c, 0x4d, false),
    (0x60, 0x4e, false),
    (0x64, 0x4f, false),
    (0x68, 0x38, false),
    (0x6c, 0x39, false),
    (0x70, 0x3a, false),
    (0x74, 0x3b, false),
    (0x78, 0x3c, false),
    (0x7c, 0x3d, false),
    (0x80, 0x3e, false),
    (0x84, 0x3f, false),
    (0x88, 0x40, false),
    (0x8c, 0x41, false),
    (0x90, 0x42, false),
    (0x94, 0x43, false),
    (0x98, 0x28, false),
    (0x9c, 0x29, false),
    (0xa0, 0x1f, true),
    (0xa4, 0x20, true),
    (0xa8, 0x1003, true),
];

// Translated from 00edfeae (disassembled, FalloutNV.exe 1.4.0.525)
/// `__get_lc_time` (LIBCMT/inittime.c; the locale information in `EAX`, the `__lc_time_data`
/// in `ESI`): -1 for a null `__lc_time_data`. Otherwise, through a local locale object (the
/// information, a null multibyte pointer), the day and month names, `AM`/`PM` and the date
/// and time formats are read as text (`__getlocaleinfo` kind 1, with the `LCID`s at +0x42 and
/// +0x44 of the information; see [`LC_TIME_FIELDS`]), then `LOCALE_ICALENDARTYPE` (0x1009) as
/// a number into +0xB0, and the `LCID` stored at +0xAC. Answers the results ORed together.
pub fn get_lc_time(e: &mut Engine, info: u32, time: u32) -> i32 {
    let ctype_lcid = u32::from(e.mem.u16(info.wrapping_add(0x42)));
    let time_lcid = u32::from(e.mem.u16(info.wrapping_add(0x44)));
    if time == 0 {
        return -1;
    }
    e.with_stack(8, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local.wrapping_add(4), 0);
        e.mem.set_u32(local, info);
        let mut result = 0u32;
        for (offset, lctype, time_side) in LC_TIME_FIELDS {
            let lcid = if time_side { time_lcid } else { ctype_lcid };
            result |= e
                .call(
                    GETLOCALEINFO,
                    &args![local, 1u32, lcid, lctype, time.wrapping_add(offset)],
                )
                .u32();
        }
        result |= e
            .call(
                GETLOCALEINFO,
                &args![local, 0u32, time_lcid, 0x1009u32, time.wrapping_add(0xb0)],
            )
            .u32();
        e.mem.set_u32(time.wrapping_add(0xac), time_lcid);
        result as i32
    })
}

/// `__lconv_c` (`011c19d4`, the C locale's `lconv`, 0x30 bytes), `_calloc_crt` (`00ed0d24`,
/// cdecl: count, size), `_malloc_crt` (`00ed0cdf`), `free` (`00ecd291`), `__free_lconv_mon`
/// (`00ee06e6`), cdecl, one word; the `InterlockedDecrement` import slot (`00fdf1c4`).
const LCONV_C: u32 = 0x011c_19d4;
const CALLOC_CRT: u32 = 0x00ed_0d24;
const MALLOC_CRT: u32 = 0x00ed_0cdf;
const FREE: u32 = 0x00ec_d291;
const FREE_LCONV_MON: u32 = 0x00ee_06e6;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// The monetary fields `__init_monetary` reads: the offset in the new `lconv`, the `LCTYPE`
/// and the `__getlocaleinfo` kind (1 text, 0 a number into one byte).
const MONETARY_FIELDS: [(u32, u32, u32); 15] = [
    (0x0c, 0x15, 1),
    (0x10, 0x14, 1),
    (0x14, 0x16, 1),
    (0x18, 0x17, 1),
    (0x1c, 0x18, 1),
    (0x20, 0x50, 1),
    (0x24, 0x51, 1),
    (0x28, 0x1a, 0),
    (0x29, 0x19, 0),
    (0x2a, 0x54, 0),
    (0x2b, 0x55, 0),
    (0x2c, 0x56, 0),
    (0x2d, 0x57, 0),
    (0x2e, 0x52, 0),
    (0x2f, 0x53, 0),
];

// Translated from 00ee0774 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__init_monetary` (LIBCMT/initmon.c; cdecl: the locale information). For the C locale
/// (no `LC_MONETARY` or `LC_NUMERIC` handle, +0x18 and +0x1C) the `lconv` is `__lconv_c`
/// with no counts. Otherwise a new `lconv` (`_calloc_crt(1, 0x30)`) and a numeric count word
/// are made (failures free what was made and answer 1); without a monetary handle the
/// `lconv` is a copy of `__lconv_c`, else a monetary count word is made too and the 15
/// monetary fields read with the information's `LCID` at +0x38 (any failure frees all,
/// with `__free_lconv_mon`, and answers 1); the grouping text's digits become numbers and its
/// `;` are taken out. The decimal point, thousands separator and grouping are taken from the
/// current `lconv` (+0xBC) and both counts set to 1. Then the old counts (+0xB8 monetary,
/// +0xB0 numeric) are released (`InterlockedDecrement`; a numeric count reaching 0 frees the
/// old `lconv` and the count), the new ones and the `lconv` stored, and 0 answered.
pub fn init_monetary(e: &mut Engine, info: u32) -> u32 {
    let (lconv, numeric_count, monetary_count) =
        if e.mem.u32(info.wrapping_add(0x18)) == 0 && e.mem.u32(info.wrapping_add(0x1c)) == 0 {
            (LCONV_C, 0u32, 0u32)
        } else {
            let mut monetary_count = 0u32;
            let lconv = e.call(CALLOC_CRT, &args![1u32, 0x30u32]).u32();
            if lconv == 0 {
                return 1;
            }
            let numeric_count = e.call(MALLOC_CRT, &args![4u32]).u32();
            if numeric_count == 0 {
                e.call(FREE, &args![lconv]);
                return 1;
            }
            e.mem.set_u32(numeric_count, 0);
            if e.mem.u32(info.wrapping_add(0x18)) == 0 {
                // REP MOVSD: the 12 words of __lconv_c.
                for i in 0..12u32 {
                    let word = e.mem.u32(LCONV_C.wrapping_add(i * 4));
                    e.mem.set_u32(lconv.wrapping_add(i * 4), word);
                }
            } else {
                monetary_count = e.call(MALLOC_CRT, &args![4u32]).u32();
                if monetary_count == 0 {
                    e.call(FREE, &args![lconv]);
                    e.call(FREE, &args![numeric_count]);
                    return 1;
                }
                e.mem.set_u32(monetary_count, 0);
                let lcid = u32::from(e.mem.u16(info.wrapping_add(0x38)));
                let failed = e.with_stack(8, |e, local| {
                    let local = local.addr();
                    e.mem.set_u32(local, info);
                    e.mem.set_u32(local.wrapping_add(4), 0);
                    let mut result = 0u32;
                    for (offset, lctype, kind) in MONETARY_FIELDS {
                        result |= e
                            .call(
                                GETLOCALEINFO,
                                &args![local, kind, lcid, lctype, lconv.wrapping_add(offset)],
                            )
                            .u32();
                    }
                    result
                });
                if failed != 0 {
                    e.call(FREE_LCONV_MON, &args![lconv]);
                    e.call(FREE, &args![lconv]);
                    e.call(FREE, &args![numeric_count]);
                    e.call(FREE, &args![monetary_count]);
                    return 1;
                }
                let mut p = e.mem.u32(lconv.wrapping_add(0x1c));
                while e.mem.u8(p) != 0 {
                    let c = e.mem.u8(p) as i8;
                    if (0x30..=0x39).contains(&c) {
                        e.mem.set_u8(p, (c - 0x30) as u8);
                        p = p.wrapping_add(1);
                    } else if c == 0x3b {
                        let mut at = p;
                        loop {
                            let next = e.mem.u8(at.wrapping_add(1));
                            e.mem.set_u8(at, next);
                            at = at.wrapping_add(1);
                            if e.mem.u8(at) == 0 {
                                break;
                            }
                        }
                    } else {
                        p = p.wrapping_add(1);
                    }
                }
            }
            for i in 0..3u32 {
                let word = e
                    .mem
                    .u32(e.mem.u32(info.wrapping_add(0xbc)).wrapping_add(i * 4));
                e.mem.set_u32(lconv.wrapping_add(i * 4), word);
            }
            e.mem.set_u32(numeric_count, 1);
            if monetary_count != 0 {
                e.mem.set_u32(monetary_count, 1);
            }
            (lconv, numeric_count, monetary_count)
        };
    let old = e.mem.u32(info.wrapping_add(0xb8));
    if old != 0 {
        e.call(INTERLOCKED_DECREMENT, &args![old]);
    }
    let old = e.mem.u32(info.wrapping_add(0xb0));
    if old != 0 && e.call(INTERLOCKED_DECREMENT, &args![old]).u32() == 0 {
        let old_lconv = e.mem.u32(info.wrapping_add(0xbc));
        e.call(FREE, &args![old_lconv]);
        let old = e.mem.u32(info.wrapping_add(0xb0));
        e.call(FREE, &args![old]);
    }
    e.mem.set_u32(info.wrapping_add(0xb8), monetary_count);
    e.mem.set_u32(info.wrapping_add(0xb0), numeric_count);
    e.mem.set_u32(info.wrapping_add(0xbc), lconv);
    0
}

/// This file's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ed_e233, sbh_find_block(u32) -> u32),
        entry!(0x00ed_e629, sbh_alloc_new_group(u32) -> i32),
        entry!(0x00ed_fbe8, validate_read(u32, u32) -> u32),
        entry!(0x00ee_58db, wsopen_s(u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ee_580f, wsopen_helper(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ee_58ad, wsopen_helper_finally(u32, u32, u32)),
        entry!(0x00ed_fbfa, wopenfile(u32, u32, u32, u32) -> u32),
        entry!(0x00ed_feae, get_lc_time(u32, u32) -> i32),
        entry!(0x00ee_0774, init_monetary(u32) -> u32),
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

    fn nothing(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    #[test]
    fn sbh_find_block_takes_the_region_within_a_megabyte() {
        let mut e = Engine::new();
        e.map(0x0127_0000, 0x1000);
        let list = e.mem.alloc(0x14 * 3);
        for (i, base) in [0x1000_0000u32, 0x2000_0000, 0x3000_0000]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(list + i as u32 * 0x14 + 0xc, *base);
        }
        e.mem.set_u32(SBH_HEADER_LIST, list);
        e.mem.set_u32(SBH_HEADER_COUNT, 3);
        assert_eq!(e.call(0x00ed_e233, &[0x2000_0010]).u32(), list + 0x14);
        assert_eq!(e.call(0x00ed_e233, &[0x300f_ffff]).u32(), list + 0x28);
        assert_eq!(e.call(0x00ed_e233, &[0x3010_0000]).u32(), 0);
        assert_eq!(e.call(0x00ed_e233, &[0x0fff_ffff]).u32(), 0);
        e.mem.set_u32(SBH_HEADER_COUNT, 2);
        assert_eq!(e.call(0x00ed_e233, &[0x3000_0000]).u32(), 0);
    }

    #[test]
    fn sbh_alloc_new_group_commits_and_links_eight_pages() {
        let mut e = Engine::new();
        let base = 0x0400_0000u32;
        e.map(base, 0x10_0000);
        e.register(VIRTUAL_ALLOC, |_, a| a[0].into_ret());
        let region = e.mem.alloc(0x41c4 + 0x204 * 32);
        let header = e.mem.alloc(0x14);
        e.mem.set_u32(header + 4, 0);
        e.mem.set_u32(header + 8, 0x3fff_ffff);
        e.mem.set_u32(header + 0xc, base);
        e.mem.set_u32(header + 0x10, region);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_e629, &[header]).i32(), 2);
        let first = base + 0x1_0000;
        assert_eq!(
            calls(&e, VIRTUAL_ALLOC),
            vec![vec![first, 0x8000, 0x1000, 4]]
        );
        let group = region + 2 * 0x204 + 0x144;
        // The empty heads point at themselves (all but the last, linked below).
        assert_eq!(e.mem.u32(group + 4), group);
        assert_eq!(e.mem.u32(group + 8), group);
        // Each page: markers, one free block of 0xff0 linked to its neighbours.
        for page in 0..8 {
            let p = first + page * 0x1000;
            assert_eq!(e.mem.u32(p + 8), u32::MAX);
            assert_eq!(e.mem.u32(p + 0xffc), u32::MAX);
            assert_eq!(e.mem.u32(p + 0xc), 0xff0);
            assert_eq!(e.mem.u32(p + 0xff8), 0xff0);
            // The first block's back link and the last one's forward link go to the list.
            if page != 7 {
                assert_eq!(e.mem.u32(p + 0x10), p + 0x100c);
            }
            if page != 0 {
                assert_eq!(e.mem.u32(p + 0x14), p.wrapping_sub(0xff4));
            }
        }
        let list = group + 0x1f8;
        assert_eq!(e.mem.u32(list + 4), first + 0xc);
        assert_eq!(e.mem.u32(first + 0x14), list);
        assert_eq!(e.mem.u32(list + 8), first + 0x700c);
        assert_eq!(e.mem.u32(first + 0x7010), list);
        assert_eq!(e.mem.u32(region + 0x44 + 8), 0);
        assert_eq!(e.mem.u32(region + 0xc4 + 8), 1);
        assert_eq!(e.mem.u8(region + 0x43), 1);
        assert_eq!(e.mem.u32(header + 4), 1);
        assert_eq!(e.mem.u32(header + 8), 0x1fff_ffff);
        // A second group: the count goes on, the first mask is left alone.
        e.mem.set_u32(header + 4, 0);
        assert_eq!(e.call(0x00ed_e629, &[header]).i32(), 3);
        assert_eq!(e.mem.u8(region + 0x43), 2);
        assert_eq!(e.mem.u32(header + 4), 0);
        assert_eq!(e.mem.u32(header + 8), 0x0fff_ffff);
        // A failed commit.
        e.register(VIRTUAL_ALLOC, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ed_e629, &[header]).i32(), -1);
        assert_eq!(e.mem.u32(header + 8), 0x0fff_ffff);
    }

    #[test]
    fn validate_read_checks_only_the_address() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ed_fbe8, &[0, 4]).u32(), 0);
        assert_eq!(e.call(0x00ed_fbe8, &[1, 0]).u32(), 1);
    }

    #[test]
    fn wsopen_s_and_wsopen_helper_check_then_open() {
        let mut e = Engine::new();
        let errno = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| errno.into_ret());
        nothing(&mut e, &[INVALID_PARAMETER, UNLOCK_FHANDLE]);
        e.register(TSOPEN_NOLOCK, |e, a| {
            e.mem.set_u32(a[1], 1);
            e.mem.set_u32(a[0], 0x21);
            u32::from(a[3] == 0xdead).into_ret()
        });
        let handle = e.mem.alloc(4);
        let name = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ee_58db, &[handle, name, 0x8000, 0x40, 0x180])
                .u32(),
            0
        );
        assert_eq!(e.mem.u32(handle), 0x21);
        let open = calls(&e, TSOPEN_NOLOCK);
        assert_eq!(open.len(), 1);
        assert_eq!(
            (open[0][0], &open[0][2..]),
            (handle, &[name, 0x8000, 0x40, 0x180][..])
        );
        assert_eq!(calls(&e, WSOPEN_HELPER_FINALLY), vec![vec![1, 0, handle]]);
        assert_eq!(calls(&e, UNLOCK_FHANDLE), vec![vec![0x21]]);
        // A failed open: the handle back to -1 and its FOPEN bit cleared.
        e.map(0x0127_0000, 0x1000);
        let block = e.mem.alloc(0x800);
        e.mem.set_u32(PIOINFO + 4, block);
        e.mem.set_u8(block + 0x40 + 4, 0x81);
        assert_eq!(
            e.call(0x00ee_58db, &[handle, name, 0xdead, 0x40, 0]).u32(),
            1
        );
        assert_eq!(e.mem.u32(handle), u32::MAX);
        assert_eq!(e.mem.u8(block + 0x44), 0x80);
        // Refusals.
        assert_eq!(e.call(0x00ee_58db, &[0, name, 0, 0x40, 0]).u32(), 0x16);
        e.mem.set_u32(handle, 5);
        assert_eq!(e.call(0x00ee_58db, &[handle, 0, 0, 0x40, 0]).u32(), 0x16);
        assert_eq!(e.mem.u32(handle), u32::MAX);
        assert_eq!(
            e.call(0x00ee_58db, &[handle, name, 0, 0x40, 0x181]).u32(),
            0x16
        );
        assert_eq!(e.mem.u32(errno), 0x16);
        // Not secure: any permission.
        assert_eq!(
            e.call(0x00ee_580f, &[name, 0, 0x40, 0x181, handle, 0])
                .u32(),
            0
        );
        assert_eq!(calls(&e, TSOPEN_NOLOCK).len(), 3);
    }

    #[test]
    fn wsopen_helper_finally_does_nothing_without_the_flag() {
        let mut e = Engine::new();
        nothing(&mut e, &[UNLOCK_FHANDLE]);
        e.call_log = Some(vec![]);
        e.call(0x00ee_58ad, &[0, 1, 0]);
        assert!(calls(&e, UNLOCK_FHANDLE).is_empty());
    }
    fn wide(e: &mut Engine, s: &str) -> u32 {
        let units: Vec<u16> = s.encode_utf16().collect();
        let at = e.mem.alloc(units.len() as u32 * 2 + 2);
        for (i, u) in units.iter().enumerate() {
            e.mem.set_u16(at + i as u32 * 2, *u);
        }
        at
    }

    /// An engine for `_wopenfile`: `_wsopen_s` answers handle 7 (or 0x18 for flags 0xdead),
    /// the wide comparisons are the real ones, the `ccs` texts are in place.
    fn open_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let errno = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| errno.into_ret());
        nothing(&mut e, &[INVALID_PARAMETER]);
        e.map(0x0126_f000, 0x2000);
        e.map(0x010f_e000, 0x1000);
        for (at, s) in [
            (CCS_TEXT, "ccs"),
            (UTF8_TEXT, "UTF-8"),
            (UTF16LE_TEXT, "UTF-16LE"),
            (UNICODE_TEXT, "UNICODE"),
        ] {
            for (i, u) in s.encode_utf16().chain([0]).enumerate() {
                e.mem.set_u16(at + i as u32 * 2, u);
            }
        }
        e.mem.set_u32(0x0126_fdf4, 0);
        e.register(WSOPEN_S, |e, a| {
            e.mem.set_u32(a[0], 7);
            u32::from(a[4] != 0x180).into_ret()
        });
        (e, errno)
    }

    /// Opens with `mode`; answers the flags `_wsopen_s` got and the stream's flags, or `None`
    /// when it answered 0.
    fn open_with(e: &mut Engine, mode: &str) -> Option<(u32, u32)> {
        let m = wide(e, mode);
        let stream = e.mem.alloc(0x20);
        e.mem.set_u32(stream + 8, 0x55);
        e.call_log = Some(vec![]);
        let r = e.call(0x00ed_fbfa, &[0x0600_0000, m, 0x40, stream]).u32();
        if r == 0 {
            return None;
        }
        assert_eq!(r, stream);
        assert_eq!(e.mem.u32(stream + 0x10), 7);
        assert_eq!(e.mem.u32(stream + 8), 0);
        let open = calls(e, WSOPEN_S);
        assert_eq!(
            (open[0][1], open[0][3], open[0][4]),
            (0x0600_0000, 0x40, 0x180)
        );
        Some((open[0][2], e.mem.u32(stream + 0xc)))
    }

    #[test]
    fn wopenfile_reads_the_mode_text() {
        let (mut e, errno) = open_engine();
        e.mem.set_u32(COMMODE, 0);
        assert_eq!(open_with(&mut e, "r"), Some((0, 1)));
        assert_eq!(open_with(&mut e, "  w"), Some((0x301, 2)));
        assert_eq!(open_with(&mut e, "a+b"), Some((0x10a | 0x8000, 0x80)));
        assert_eq!(
            open_with(&mut e, "rtTDNS"),
            Some((0x4000 | 0x1000 | 0x40 | 0x80 | 0x20, 1))
        );
        assert_eq!(open_with(&mut e, "rR c"), Some((0x10, 0x4001)));
        e.mem.set_u32(COMMODE, 0x4000);
        assert_eq!(open_with(&mut e, "rn"), Some((0, 1)));
        assert_eq!(open_with(&mut e, "r"), Some((0, 0x4001)));
        e.mem.set_u32(COMMODE, 0);
        // A repeated or conflicting modifier is passed over and ends the reading.
        assert_eq!(open_with(&mut e, "rbt"), Some((0x8000, 1)));
        assert_eq!(open_with(&mut e, "r++"), Some((2, 0x80)));
        // ccs.
        assert_eq!(open_with(&mut e, "r, ccs = utf-8 "), Some((0x4_0000, 1)));
        assert_eq!(open_with(&mut e, "w,ccs=UTF-16le"), Some((0x2_0301, 2)));
        assert_eq!(open_with(&mut e, "w,ccs=unicode"), Some((0x1_0301, 2)));
        // Refused.
        for mode in [
            "x",
            "rq",
            "r,ccs",
            "r,ccs=UTF-7",
            "r,cc=UTF-8",
            "r,ccs=UTF-8x",
            "",
        ] {
            e.mem.set_u32(errno, 0);
            assert_eq!(open_with(&mut e, mode), None, "{mode}");
            assert_eq!(e.mem.u32(errno), 0x16, "{mode}");
        }
        assert_eq!(e.mem.u32(CFLUSH), 12);
    }
    #[test]
    fn get_lc_time_asks_every_field_and_ors_the_results() {
        let mut e = Engine::new();
        e.register(GETLOCALEINFO, |e, a| {
            assert_eq!(e.mem.u32(a[0] + 4), 0);
            u32::from(a[3] == 0x2c).into_ret()
        });
        let info = e.mem.alloc(0x100);
        e.mem.set_u16(info + 0x42, 0x409);
        e.mem.set_u16(info + 0x44, 0x407);
        let time = e.mem.alloc(0xb4);
        assert_eq!(e.call(0x00ed_feae, &[info, 0]).i32(), -1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_feae, &[info, time]).i32(), 1);
        let asked = calls(&e, GETLOCALEINFO);
        assert_eq!(asked.len(), 44);
        assert_eq!(&asked[0][1..], &[1, 0x409, 0x31, time + 4]);
        assert_eq!(&asked[6][1..], &[1, 0x409, 0x37, time]);
        assert_eq!(&asked[40][1..], &[1, 0x407, 0x1f, time + 0xa0]);
        assert_eq!(&asked[43][1..], &[0, 0x407, 0x1009, time + 0xb0]);
        assert_eq!(e.mem.u32(time + 0xac), 0x407);
        // Every field from +0 to +0xA8 once.
        let mut offsets: Vec<u32> = asked[..43].iter().map(|a| a[4] - time).collect();
        offsets.sort_unstable();
        assert_eq!(offsets, (0..43).map(|i| i * 4).collect::<Vec<u32>>());
    }
    /// An engine for `__init_monetary`: the allocators from the engine's memory, `free` and
    /// `__free_lconv_mon` quiet, `InterlockedDecrement` decrementing, `__getlocaleinfo`
    /// answering `fail` for the `LCTYPE` 0x18 (and setting the grouping text "3;2;0" there).
    fn monetary_engine(fail: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_1000, 0x1000);
        for i in 0..12u32 {
            e.mem.set_u32(LCONV_C + i * 4, 0x100 + i);
        }
        e.register(CALLOC_CRT, |e, a| e.mem.alloc(a[0] * a[1]).into_ret());
        e.register(MALLOC_CRT, |e, a| e.mem.alloc(a[0]).into_ret());
        nothing(&mut e, &[FREE, FREE_LCONV_MON]);
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register_double(GETLOCALEINFO, move |e, a| {
            if a[3] == 0x18 {
                let text = e.mem.alloc(8);
                e.mem.write(text, b"3;2;0\0");
                e.mem.set_u32(a[4], text);
                return fail.into_ret();
            }
            0u32.into_ret()
        });
        e
    }

    fn locale_info(e: &mut Engine, monetary: u32, numeric: u32) -> u32 {
        let info = e.mem.alloc(0xc0);
        e.mem.set_u32(info + 0x18, monetary);
        e.mem.set_u32(info + 0x1c, numeric);
        e.mem.set_u16(info + 0x38, 0x409);
        let current = e.mem.alloc(0x30);
        e.mem.set_u32(current, 0x2e);
        e.mem.set_u32(current + 4, 0x2c);
        e.mem.set_u32(current + 8, 0x77);
        e.mem.set_u32(info + 0xbc, current);
        info
    }

    #[test]
    fn init_monetary_reads_the_fields_and_swaps_the_counts() {
        let mut e = monetary_engine(0);
        let info = locale_info(&mut e, 1, 1);
        let old_monetary = e.mem.alloc(4);
        let old_numeric = e.mem.alloc(4);
        e.mem.set_u32(old_monetary, 2);
        e.mem.set_u32(old_numeric, 1);
        e.mem.set_u32(info + 0xb8, old_monetary);
        e.mem.set_u32(info + 0xb0, old_numeric);
        let old_lconv = e.mem.u32(info + 0xbc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_0774, &[info]).u32(), 0);
        let lconv = e.mem.u32(info + 0xbc);
        assert_ne!(lconv, old_lconv);
        let asked = calls(&e, GETLOCALEINFO);
        assert_eq!(asked.len(), 15);
        assert_eq!(&asked[0][1..], &[1, 0x409, 0x15, lconv + 0xc]);
        assert_eq!(&asked[14][1..], &[0, 0x409, 0x53, lconv + 0x2f]);
        // The grouping digits as numbers, the separators gone.
        let grouping = e.mem.u32(lconv + 0x1c);
        assert_eq!(e.mem.cstr(grouping), &[3, 2]);
        assert_eq!(e.mem.u8(grouping + 2), 0);
        // The current lconv's first three words.
        assert_eq!(
            (e.mem.u32(lconv), e.mem.u32(lconv + 4), e.mem.u32(lconv + 8)),
            (0x2e, 0x2c, 0x77)
        );
        assert_eq!(e.mem.u32(e.mem.u32(info + 0xb8)), 1);
        assert_eq!(e.mem.u32(e.mem.u32(info + 0xb0)), 1);
        // The old numeric count reached 0: the old lconv and the count are freed.
        assert_eq!(e.mem.u32(old_monetary), 1);
        assert_eq!(calls(&e, FREE), vec![vec![old_lconv], vec![old_numeric]]);
    }

    #[test]
    fn init_monetary_for_the_c_locale_and_without_a_monetary_handle() {
        let mut e = monetary_engine(0);
        let info = locale_info(&mut e, 0, 0);
        assert_eq!(e.call(0x00ee_0774, &[info]).u32(), 0);
        assert_eq!(e.mem.u32(info + 0xbc), LCONV_C);
        assert_eq!((e.mem.u32(info + 0xb0), e.mem.u32(info + 0xb8)), (0, 0));
        let info = locale_info(&mut e, 0, 1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_0774, &[info]).u32(), 0);
        let lconv = e.mem.u32(info + 0xbc);
        assert_eq!(e.mem.u32(lconv), 0x2e);
        assert_eq!(e.mem.u32(lconv + 0xc), 0x103);
        assert_eq!(e.mem.u32(lconv + 0x2c), 0x10b);
        assert_eq!(e.mem.u32(info + 0xb8), 0);
        assert_eq!(e.mem.u32(e.mem.u32(info + 0xb0)), 1);
        assert!(calls(&e, GETLOCALEINFO).is_empty());
    }

    #[test]
    fn init_monetary_frees_everything_when_a_field_fails() {
        let mut e = monetary_engine(1);
        let info = locale_info(&mut e, 1, 1);
        let old_lconv = e.mem.u32(info + 0xbc);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_0774, &[info]).u32(), 1);
        assert_eq!(e.mem.u32(info + 0xbc), old_lconv);
        assert_eq!(calls(&e, FREE_LCONV_MON).len(), 1);
        assert_eq!(calls(&e, FREE).len(), 3);
        // The lconv block itself cannot be had.
        e.register(CALLOC_CRT, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ee_0774, &[info]).u32(), 1);
    }
}
