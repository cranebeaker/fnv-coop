//! More C runtime pieces (MSVC 2008 LIBCMT, statically linked into FalloutNV.exe 1.4.0.525),
//! translated from the exe: the start-up's `___security_init_cookie` (`00eddb9a`),
//! `__tmainCRTStartup` (`00ecc35d`), `___crtGetEnvironmentStringsA` (`00edd9e7`) and
//! `__setargv` (`00edd92c`), and the wide `__tsopen_nolock` (`00ee50ef`), which differs from the
//! narrow one (`00ee311d`, `crt.rs`) only in calling `CreateFileW` and in reading the last
//! character into a word for the Ctrl-Z test.
//!
//! Every call is made by address, as the exe makes it, so a test can double each one.

use super::{
    alloc_osfhnd, close_nolock, doserrno_value, dosmaperr, errno_value, free_osfhnd,
    invalid_parameter_zeros, ioinfo, lseek_nolock, set_doserrno, set_errno, set_osfhnd,
    tsopen_mark, OpenStep, CHSIZE_NOLOCK, GET_FMODE, IMPORT_CLOSE_HANDLE, IMPORT_GET_FILE_TYPE,
    IMPORT_GET_LAST_ERROR, INVOKE_WATSON_ADDR, READ_NOLOCK, UMASK,
};
use crate::prelude::*;

/// The security cookie (`011c16bc`) and its complement (`011c16c0`); the default cookie.
const SECURITY_COOKIE: u32 = 0x011c_16bc;
const SECURITY_COOKIE_COMPLEMENT: u32 = 0x011c_16c0;
const DEFAULT_COOKIE: u32 = 0xbb40_e64e;
/// The KERNEL32 import slots (`stdcall`): `GetSystemTimeAsFileTime` (`00fdf184`),
/// `GetCurrentProcessId` (`00fdf258`), `GetCurrentThreadId` (`00fdf1d0`), `GetTickCount`
/// (`00fdf060`), `QueryPerformanceCounter` (`00fdf0a0`), `GetStartupInfoA` (`00fdf190`),
/// `GetCommandLineA` (`00fdf18c`), `GetModuleFileNameA` (`00fdf150`),
/// `GetEnvironmentStringsW` (`00fdf250`), `FreeEnvironmentStringsW` (`00fdf24c`),
/// `GetEnvironmentStrings` (`00fdf248`), `FreeEnvironmentStringsA` (`00fdf244`),
/// `WideCharToMultiByte` (`00fdf170`) and `CreateFileW` (`00fdf214`).
const IMPORT_GET_SYSTEM_TIME_AS_FILE_TIME: u32 = 0x00fd_f184;
const IMPORT_GET_CURRENT_PROCESS_ID: u32 = 0x00fd_f258;
const IMPORT_GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
const IMPORT_GET_TICK_COUNT: u32 = 0x00fd_f060;
const IMPORT_QUERY_PERFORMANCE_COUNTER: u32 = 0x00fd_f0a0;
const IMPORT_GET_STARTUP_INFO_A: u32 = 0x00fd_f190;
const IMPORT_GET_COMMAND_LINE_A: u32 = 0x00fd_f18c;
const IMPORT_GET_MODULE_FILE_NAME_A: u32 = 0x00fd_f150;
const IMPORT_GET_ENVIRONMENT_STRINGS_W: u32 = 0x00fd_f250;
const IMPORT_FREE_ENVIRONMENT_STRINGS_W: u32 = 0x00fd_f24c;
const IMPORT_GET_ENVIRONMENT_STRINGS: u32 = 0x00fd_f248;
const IMPORT_FREE_ENVIRONMENT_STRINGS_A: u32 = 0x00fd_f244;
const IMPORT_WIDE_CHAR_TO_MULTI_BYTE: u32 = 0x00fd_f170;
const IMPORT_CREATE_FILE_W: u32 = 0x00fd_f214;
/// The C runtime's own functions (`cdecl`): `_heap_init` (`00eddb6a`), `fast_error_exit`
/// (`00ecc334`), `__mtinit` (`00ed1327`), `_RTC_Initialize` (`00eddb1e`), `_ioinit`
/// (`00ed7e18`), `_amsg_exit` (`00eccc55`), `_setenvp` (`00edd6b4`), `_cinit` (`00eccd14`),
/// `_wincmdln` (`00edd655`), `exit` (`00eccec5`), `_cexit` (`00eccef1`), `___initmbctable`
/// (`00ed417e`), `parse_cmdline` (`00edd792`: the command line and the character count's
/// address as its register words, then `argv`, the text buffer and the argument count's
/// address), `_malloc_crt` (`00ed0cdf`), `free` (`00ecd291`) and `memcpy` (`00ec44d0`); the
/// game's `WinMain` (`0086a850`, `stdcall`, four words).
const HEAP_INIT: u32 = 0x00ed_db6a;
const FAST_ERROR_EXIT: u32 = 0x00ec_c334;
const MTINIT: u32 = 0x00ed_1327;
const RTC_INITIALIZE: u32 = 0x00ed_db1e;
const IOINIT: u32 = 0x00ed_7e18;
const AMSG_EXIT: u32 = 0x00ec_cc55;
const SETENVP: u32 = 0x00ed_d6b4;
const CINIT: u32 = 0x00ec_cd14;
const WINCMDLN: u32 = 0x00ed_d655;
const EXIT: u32 = 0x00ec_cec5;
const CEXIT: u32 = 0x00ec_cef1;
const INITMBCTABLE: u32 = 0x00ed_417e;
const PARSE_CMDLINE: u32 = 0x00ed_d792;
const MALLOC_CRT: u32 = 0x00ed_0cdf;
const FREE: u32 = 0x00ec_d291;
const MEMCPY: u32 = 0x00ec_44d0;
const WIN_MAIN: u32 = 0x0086_a850;
/// `___crtGetEnvironmentStringsA` (`00edd9e7`) and `__setargv` (`00edd92c`), for
/// `__tmainCRTStartup`'s calls by address.
const CRT_GET_ENVIRONMENT_STRINGS_A: u32 = 0x00ed_d9e7;
const SETARGV: u32 = 0x00ed_d92c;
/// The image base (where the PE headers are) and the globals the start-up sets: `_acmdln`
/// (`01271a98`), `_aenvptr` (`0126fdac`), `__mbctype_initialized` (`01271a90`), `_pgmname`
/// (`012704b0`, 0x104 bytes) and its last byte's neighbour `012705b4`, `_pgmptr` (`0126fdd8`),
/// `__argc` (`0126fdbc`), `__argv` (`0126fdc0`) and the environment kind
/// `___crtGetEnvironmentStringsA` keeps (`012705b8`: 1 wide, 2 ANSI).
const IMAGE_BASE: u32 = 0x0040_0000;
const ACMDLN: u32 = 0x0127_1a98;
const AENVPTR: u32 = 0x0126_fdac;
const MBCTYPE_INITIALIZED: u32 = 0x0127_1a90;
const PGMNAME: u32 = 0x0127_04b0;
const PGMNAME_END: u32 = 0x0127_05b4;
const PGMPTR: u32 = 0x0126_fdd8;
const ARGC: u32 = 0x0126_fdbc;
const ARGV: u32 = 0x0126_fdc0;
const ENVIRONMENT_KIND: u32 = 0x0127_05b8;

// Translated from 00eddb9a (disassembled, FalloutNV.exe 1.4.0.525)
/// `___security_init_cookie` (LIBCMT/gs_support.c; no arguments). A cookie that is neither the
/// default (`bb40e64e`) nor without high bits is kept; otherwise it becomes the XOR of the
/// system time's two halves (`GetSystemTimeAsFileTime` into a zeroed local), the process id,
/// the thread id, the tick count and the performance counter's two halves (in that call
/// order); a result equal to the default becomes `bb40e64f`, one without high bits gets its low
/// half copied into them. The complement goes to `011c16c0` either way.
pub fn security_init_cookie(e: &mut Engine) {
    let cookie = e.mem.u32(SECURITY_COOKIE);
    if cookie != DEFAULT_COOKIE && cookie & 0xffff_0000 != 0 {
        e.mem.set_u32(SECURITY_COOKIE_COMPLEMENT, !cookie);
        return;
    }
    let mut value = e.with_stack(8, |e, time| {
        e.mem.set_u32(time.addr(), 0);
        e.mem.set_u32(time.addr() + 4, 0);
        e.call(IMPORT_GET_SYSTEM_TIME_AS_FILE_TIME, &args![time.addr()]);
        e.mem.u32(time.addr() + 4) ^ e.mem.u32(time.addr())
    });
    value ^= e.call(IMPORT_GET_CURRENT_PROCESS_ID, &args![]).u32();
    value ^= e.call(IMPORT_GET_CURRENT_THREAD_ID, &args![]).u32();
    value ^= e.call(IMPORT_GET_TICK_COUNT, &args![]).u32();
    value ^= e.with_stack(8, |e, counter| {
        e.call(IMPORT_QUERY_PERFORMANCE_COUNTER, &args![counter.addr()]);
        e.mem.u32(counter.addr() + 4) ^ e.mem.u32(counter.addr())
    });
    if value == DEFAULT_COOKIE {
        value = 0xbb40_e64f;
    } else if value & 0xffff_0000 == 0 {
        value |= value << 16;
    }
    e.mem.set_u32(SECURITY_COOKIE, value);
    e.mem.set_u32(SECURITY_COOKIE_COMPLEMENT, !value);
}

// Translated from 00ecc35d (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ecc35d - the SEH frame (__SEH_prolog4/__SEH_epilog4, its state words and the __except handlers 00ecc49c/00ecc4c6) is not translated.
/// `__tmainCRTStartup` (LIBCMT/crt0.c, the GUI start-up; no arguments). Takes the
/// `STARTUPINFOA` (`GetStartupInfoA`, a local) and whether the image is managed (the PE headers
/// at `00400000`: `MZ`, the `PE` signature, the optional header's magic 0x10B, more than 14
/// data directories and a non-zero CLR directory); runs `_heap_init(1)` (0: `fast_error_exit`
/// 0x1C), `__mtinit` (0: `fast_error_exit` 0x10), `_RTC_Initialize`, `_ioinit` (negative:
/// `_amsg_exit` 0x1B), keeps `GetCommandLineA` in `_acmdln` and `___crtGetEnvironmentStringsA`
/// in `_aenvptr`, runs `__setargv` (negative: `_amsg_exit` 8), `_setenvp` (negative:
/// `_amsg_exit` 9) and `_cinit(1)` (non-zero: `_amsg_exit` with it); then calls
/// `WinMain(00400000, 0, _wincmdln(), show)` with the start-up's `wShowWindow` when its
/// `dwFlags` has `STARTF_USESHOWWINDOW`, else `SW_SHOWDEFAULT` (10). An unmanaged image then
/// calls `exit` with `WinMain`'s answer; `_cexit` follows (the exe goes on after each of these
/// calls as after any, so the translation does too). Answers `WinMain`'s answer.
pub fn tmain_crt_startup(e: &mut Engine) -> u32 {
    e.with_stack(0x44, |e, startup| {
        let startup = startup.addr();
        e.call(IMPORT_GET_STARTUP_INFO_A, &args![startup]);
        let managed = if e.mem.u16(IMAGE_BASE) == 0x5a4d {
            let header = e.mem.u32(IMAGE_BASE + 0x3c);
            let at = |offset: u32| header.wrapping_add(IMAGE_BASE).wrapping_add(offset);
            if e.mem.u32(at(0)) == 0x4550
                && e.mem.u16(at(0x18)) == 0x10b
                && e.mem.u32(at(0x74)) > 0xe
            {
                u32::from(e.mem.u32(at(0xe8)) != 0)
            } else {
                0
            }
        } else {
            0
        };
        if e.call(HEAP_INIT, &args![1u32]).u32() == 0 {
            e.call(FAST_ERROR_EXIT, &args![0x1cu32]);
        }
        if e.call(MTINIT, &args![]).u32() == 0 {
            e.call(FAST_ERROR_EXIT, &args![0x10u32]);
        }
        e.call(RTC_INITIALIZE, &args![]);
        if (e.call(IOINIT, &args![]).u32() as i32) < 0 {
            e.call(AMSG_EXIT, &args![0x1bu32]);
        }
        let command_line = e.call(IMPORT_GET_COMMAND_LINE_A, &args![]).u32();
        e.mem.set_u32(ACMDLN, command_line);
        let environment = e.call(CRT_GET_ENVIRONMENT_STRINGS_A, &args![]).u32();
        e.mem.set_u32(AENVPTR, environment);
        if (e.call(SETARGV, &args![]).u32() as i32) < 0 {
            e.call(AMSG_EXIT, &args![8u32]);
        }
        if (e.call(SETENVP, &args![]).u32() as i32) < 0 {
            e.call(AMSG_EXIT, &args![9u32]);
        }
        let initialized = e.call(CINIT, &args![1u32]).u32();
        if initialized != 0 {
            e.call(AMSG_EXIT, &args![initialized]);
        }
        let arguments = e.call(WINCMDLN, &args![]).u32();
        let show = if e.mem.u8(startup + 0x2c) & 1 != 0 {
            u32::from(e.mem.u16(startup + 0x30))
        } else {
            10
        };
        let answer = e
            .call(WIN_MAIN, &args![IMAGE_BASE, 0u32, arguments, show])
            .u32();
        if managed == 0 {
            e.call(EXIT, &args![answer]);
        }
        e.call(CEXIT, &args![]);
        answer
    })
}

/// The end of a block of zero-terminated strings that ends with an empty one: from `start`,
/// as the exe steps (`unit` bytes a character): when the first character is not zero, it
/// steps over characters until a zero, then once more, and goes on while that character is
/// not zero. Answers the address of the last character read (the block's final zero, or
/// `start` for an empty block).
fn block_end(e: &Engine, start: u32, unit: u32) -> u32 {
    let read = |at: u32| {
        if unit == 2 {
            u32::from(e.mem.u16(at))
        } else {
            u32::from(e.mem.u8(at))
        }
    };
    let mut at = start;
    if read(at) == 0 {
        return at;
    }
    loop {
        at = at.wrapping_add(unit);
        if read(at) != 0 {
            continue;
        }
        at = at.wrapping_add(unit);
        if read(at) == 0 {
            return at;
        }
    }
}

// Translated from 00edd9e7 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___crtGetEnvironmentStringsA` (LIBCMT/aw_env.c; no arguments): the environment as an ANSI
/// block of the C runtime's heap (`_malloc_crt`). The kind (`012705b8`) is found on the first
/// call: `GetEnvironmentStringsW` answering a block makes it 1 (wide); a null answer with
/// `GetLastError` 0x78 makes it 2 (ANSI); any other error leaves it 0. Wide (kind 1): the wide
/// block (taken again when not taken yet; null: 0 answered) is measured through its final zero,
/// sized with `WideCharToMultiByte(CP_ACP, 0, block, count, NULL, 0, NULL, NULL)`, converted
/// into a block of that size (a failed conversion frees it: 0), and the wide block freed
/// (`FreeEnvironmentStringsW`); answers the block or 0. Kind 0 or 2: the ANSI block
/// (`GetEnvironmentStrings`; null: 0) is copied whole (through its final zero) into a new block
/// and freed (`FreeEnvironmentStringsA`); a failed allocation answers 0. Another kind answers 0.
pub fn crt_get_environment_strings_a(e: &mut Engine) -> u32 {
    let mut kind = e.mem.u32(ENVIRONMENT_KIND);
    let mut wide = 0u32;
    if kind == 0 {
        wide = e.call(IMPORT_GET_ENVIRONMENT_STRINGS_W, &args![]).u32();
        if wide != 0 {
            e.mem.set_u32(ENVIRONMENT_KIND, 1);
            kind = 1;
        } else if e.call(IMPORT_GET_LAST_ERROR, &args![]).u32() == 0x78 {
            e.mem.set_u32(ENVIRONMENT_KIND, 2);
            kind = 2;
        } else {
            kind = e.mem.u32(ENVIRONMENT_KIND);
        }
    }
    if kind == 1 {
        if wide == 0 {
            wide = e.call(IMPORT_GET_ENVIRONMENT_STRINGS_W, &args![]).u32();
            if wide == 0 {
                return 0;
            }
        }
        let end = block_end(e, wide, 2);
        let count = ((end.wrapping_sub(wide) as i32) >> 1).wrapping_add(1) as u32;
        let size = e
            .call(
                IMPORT_WIDE_CHAR_TO_MULTI_BYTE,
                &args![0u32, 0u32, wide, count, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        let mut result = 0u32;
        if size != 0 {
            let mut block = e.call(MALLOC_CRT, &args![size]).u32();
            if block != 0 {
                let converted = e
                    .call(
                        IMPORT_WIDE_CHAR_TO_MULTI_BYTE,
                        &args![0u32, 0u32, wide, count, block, size, 0u32, 0u32],
                    )
                    .u32();
                if converted == 0 {
                    e.call(FREE, &args![block]);
                    block = 0;
                }
                result = block;
            }
        }
        e.call(IMPORT_FREE_ENVIRONMENT_STRINGS_W, &args![wide]);
        return result;
    }
    if kind != 2 && kind != 0 {
        return 0;
    }
    let ansi = e.call(IMPORT_GET_ENVIRONMENT_STRINGS, &args![]).u32();
    if ansi == 0 {
        return 0;
    }
    let size = block_end(e, ansi, 1).wrapping_sub(ansi).wrapping_add(1);
    let block = e.call(MALLOC_CRT, &args![size]).u32();
    if block == 0 {
        e.call(IMPORT_FREE_ENVIRONMENT_STRINGS_A, &args![ansi]);
        return 0;
    }
    e.call(MEMCPY, &args![block, ansi, size]);
    e.call(IMPORT_FREE_ENVIRONMENT_STRINGS_A, &args![ansi]);
    block
}

// Translated from 00edd92c (disassembled, FalloutNV.exe 1.4.0.525)
/// `__setargv` (LIBCMT/stdargv.c; no arguments): makes the multibyte table first when it is not
/// made (`___initmbctable`), clears `012705b4`, puts the program's path into `_pgmname`
/// (`GetModuleFileNameA(NULL, 012704b0, 0x104)`) and points `_pgmptr` at it; the command line is
/// `_acmdln`, or `_pgmname` when that is null or empty. `parse_cmdline` counts the arguments and
/// characters; an argument count of 0x3FFFFFFF or more, a character count of -1, or a size
/// (count * 4 + characters) that wraps answers -1; so does a failed `_malloc_crt` of that size.
/// Otherwise `parse_cmdline` fills the block (`argv` first, the text after it), `__argc` takes
/// the count less one and `__argv` the block; answers 0.
pub fn setargv(e: &mut Engine) -> u32 {
    if e.mem.u32(MBCTYPE_INITIALIZED) == 0 {
        e.call(INITMBCTABLE, &args![]);
    }
    e.mem.set_u8(PGMNAME_END, 0);
    e.call(
        IMPORT_GET_MODULE_FILE_NAME_A,
        &args![0u32, PGMNAME, 0x104u32],
    );
    e.mem.set_u32(PGMPTR, PGMNAME);
    let acmdln = e.mem.u32(ACMDLN);
    let line = if acmdln == 0 || e.mem.u8(acmdln) == 0 {
        PGMNAME
    } else {
        acmdln
    };
    e.with_stack(8, |e, counts| {
        let arguments = counts.addr();
        let characters = counts.addr() + 4;
        e.call(
            PARSE_CMDLINE,
            &args![line, characters, 0u32, 0u32, arguments],
        );
        let count = e.mem.u32(arguments);
        if count >= 0x3fff_ffff {
            return 0xffff_ffff;
        }
        let chars = e.mem.u32(characters);
        if chars == 0xffff_ffff {
            return 0xffff_ffff;
        }
        let table = count << 2;
        let size = table.wrapping_add(chars);
        if size < chars {
            return 0xffff_ffff;
        }
        let block = e.call(MALLOC_CRT, &args![size]).u32();
        if block == 0 {
            return 0xffff_ffff;
        }
        e.call(
            PARSE_CMDLINE,
            &args![
                line,
                characters,
                block,
                table.wrapping_add(block),
                arguments
            ],
        );
        let count = e.mem.u32(arguments);
        e.mem.set_u32(ARGC, count.wrapping_sub(1));
        e.mem.set_u32(ARGV, block);
        0
    })
}

// Translated from 00ee50ef (disassembled, FalloutNV.exe 1.4.0.525)
/// `__tsopen_nolock`, the wide one (LIBCMT/open.c built for `wchar_t`, called by
/// `_wsopen_helper` `00ee580f`): `cdecl` with the result handle's address in `EAX` (the first
/// word here) and five stack words: the address of the caller's unlock flag, the wide path,
/// the open flags, the share flag and the permission mode. Answers 0 or an `errno`. The same
/// steps as the narrow `__tsopen_nolock` (`crt.rs`, `00ee311d`; its instructions match this
/// one's except for the import and the Ctrl-Z read) with `CreateFileW` for both opens.
pub fn wtsopen_nolock(
    e: &mut Engine,
    handle_out: Ptr,
    unlock_flag: Ptr,
    path: Ptr,
    flags: u32,
    share: u32,
    mode: u32,
) -> u32 {
    e.with_stack(0xc, |e, security| {
        wtsopen_body(
            e,
            handle_out.addr(),
            unlock_flag.addr(),
            path.addr(),
            flags,
            share,
            mode,
            security.addr(),
        )
    })
}

/// `__tsopen_nolock` (wide) with its `SECURITY_ATTRIBUTES` stack block.
fn wtsopen_body(
    e: &mut Engine,
    handle_out: u32,
    unlock_flag: u32,
    path: u32,
    flags: u32,
    share: u32,
    mode: u32,
    security: u32,
) -> u32 {
    let mut flags = flags;
    let mut encoding: u8 = 0;
    e.mem.set_u32(security, 0xc);
    e.mem.set_u32(security + 4, 0);
    let mut osfile: u8 = if flags & 0x80 != 0 {
        e.mem.set_u32(security + 8, 0);
        0x10
    } else {
        e.mem.set_u32(security + 8, 1);
        0
    };
    let file_mode = e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), 0);
        if e.call(GET_FMODE, &args![word]).u32() != 0 {
            e.call(INVOKE_WATSON_ADDR, &[0, 0, 0, 0, 0]);
        }
        e.mem.u32(word.addr())
    });
    if flags & 0x8000 == 0 && (flags & 0x74000 != 0 || file_mode != 0x8000) {
        osfile |= 0x80;
    }
    let invalid = |e: &mut Engine| -> u32 {
        set_doserrno(e, 0);
        e.mem.set_u32(handle_out, 0xffff_ffff);
        set_errno(e, 0x16);
        invalid_parameter_zeros(e);
        0x16
    };
    let mut access = match flags & 3 {
        0 => 0x8000_0000u32,
        1 => {
            if flags & 8 != 0 && flags & 0x70000 != 0 {
                0xc000_0000
            } else {
                0x4000_0000
            }
        }
        2 => 0xc000_0000,
        _ => return invalid(e),
    };
    let mut share_mode = match share {
        0x10 => 0u32,
        0x20 => 1,
        0x30 => 2,
        0x40 => 3,
        0x80 => u32::from(access == 0x8000_0000),
        _ => return invalid(e),
    };
    let creation = match flags & 0x700 {
        0 | 0x400 => 3u32,
        0x100 => 4,
        0x200 | 0x600 => 5,
        0x300 => 2,
        0x500 | 0x700 => 1,
        _ => return invalid(e),
    };
    let mut attributes = 0x80u32;
    if flags & 0x100 != 0 && (!e.mem.u32(UMASK) & mode) as u8 & 0x80 == 0 {
        attributes = 1;
    }
    if flags & 0x40 != 0 {
        attributes |= 0x400_0000;
        access |= 0x1_0000;
        share_mode |= 4;
    }
    if flags & 0x1000 != 0 {
        attributes |= 0x100;
    }
    if flags & 0x20 != 0 {
        attributes |= 0x800_0000;
    } else if flags & 0x10 != 0 {
        attributes |= 0x1000_0000;
    }
    let handle = alloc_osfhnd(e);
    e.mem.set_u32(handle_out, handle);
    if handle == 0xffff_ffff {
        set_doserrno(e, 0);
        e.mem.set_u32(handle_out, 0xffff_ffff);
        set_errno(e, 0x18);
        return errno_value(e);
    }
    e.mem.set_u32(unlock_flag, 1);
    let open = |e: &mut Engine, access: u32, creation: u32| -> u32 {
        e.call(
            IMPORT_CREATE_FILE_W,
            &args![path, access, share_mode, security, creation, attributes, 0u32],
        )
        .u32()
    };
    let mut os_handle = open(e, access, creation);
    if os_handle == 0xffff_ffff && access & 0xc000_0000 == 0xc000_0000 && flags & 1 != 0 {
        access &= 0x7fff_ffff;
        os_handle = open(e, access, creation);
    }
    let clear_open = |e: &mut Engine, handle: u32| {
        let entry = ioinfo(e, handle);
        let file = e.mem.u8(entry + 4) & 0xfe;
        e.mem.set_u8(entry + 4, file);
    };
    if os_handle == 0xffff_ffff {
        clear_open(e, handle);
        let error = e.call(IMPORT_GET_LAST_ERROR, &args![]).u32();
        dosmaperr(e, error);
        return errno_value(e);
    }
    let kind = e.call(IMPORT_GET_FILE_TYPE, &args![os_handle]).u32();
    if kind == 0 {
        clear_open(e, handle);
        let error = e.call(IMPORT_GET_LAST_ERROR, &args![]).u32();
        dosmaperr(e, error);
        e.call(IMPORT_CLOSE_HANDLE, &args![os_handle]);
        if error == 0 {
            set_errno(e, 0xd);
        }
        return errno_value(e);
    }
    if kind == 2 {
        osfile |= 0x40;
    } else if kind == 3 {
        osfile |= 8;
    }
    set_osfhnd(e, handle, os_handle);
    osfile |= 1;
    let entry = ioinfo(e, handle);
    e.mem.set_u8(entry + 4, osfile);
    let modes = e.mem.u8(entry + 0x24) & 0x80;
    e.mem.set_u8(entry + 0x24, modes);
    let device_or_pipe = osfile & 0x48;
    let mut step = OpenStep::Done;
    if device_or_pipe == 0 && osfile & 0x80 != 0 && flags & 2 != 0 {
        step = wtsopen_cut_ctrl_z(e, handle);
    }
    if matches!(step, OpenStep::Done) && osfile & 0x80 != 0 {
        if flags & 0x74000 == 0 {
            let default = file_mode & 0x74000;
            if default == 0 {
                flags |= 0x4000;
            } else {
                flags |= default;
            }
        }
        match flags & 0x74000 {
            0x4000 => encoding = 0,
            0x10000 | 0x14000 => {
                if flags & 0x301 == 0x301 {
                    encoding = 2;
                }
            }
            0x20000 | 0x24000 => encoding = 2,
            0x40000 | 0x44000 => encoding = 1,
            _ => {}
        }
        if flags & 0x70000 != 0 && osfile & 0x40 == 0 {
            step = tsopen_mark(e, handle, access, creation, &mut encoding);
        }
    }
    match step {
        OpenStep::Done => {}
        OpenStep::Failed => {
            close_nolock(e, handle);
            return errno_value(e);
        }
        OpenStep::Invalid => return 0x16,
    }
    let entry = ioinfo(e, handle);
    let modes = e.mem.u8(entry + 0x24);
    e.mem
        .set_u8(entry + 0x24, modes ^ ((modes ^ encoding) & 0x7f));
    let modes = e.mem.u8(entry + 0x24);
    e.mem
        .set_u8(entry + 0x24, (((flags >> 16) as u8) << 7) | (modes & 0x7f));
    if device_or_pipe == 0 && flags & 8 != 0 {
        let file = e.mem.u8(entry + 4) | 0x20;
        e.mem.set_u8(entry + 4, file);
    }
    if access & 0xc000_0000 == 0xc000_0000 && flags & 1 != 0 {
        e.call(IMPORT_CLOSE_HANDLE, &args![os_handle]);
        let reopened = open(e, access & 0x7fff_ffff, 3);
        if reopened == 0xffff_ffff {
            let error = e.call(IMPORT_GET_LAST_ERROR, &args![]).u32();
            dosmaperr(e, error);
            clear_open(e, handle);
            free_osfhnd(e, handle);
            return errno_value(e);
        }
        let entry = ioinfo(e, handle);
        e.mem.set_u32(entry, reopened);
    }
    0
}

/// A trailing Ctrl-Z (0x1a) cut off a text file opened read-write: seek to the last byte
/// (`_lseek_nolock(-1, SEEK_END)`; a failure other than `ERROR_NEGATIVE_SEEK` (0x83) in
/// `_doserrno` fails), read it (one byte into a zeroed word, which is compared as a 16-bit word), cut the file there when it is 0x1a (`_chsize_nolock` with the
/// sign-extended position), seek back to 0.
fn wtsopen_cut_ctrl_z(e: &mut Engine, handle: u32) -> OpenStep {
    let end = lseek_nolock(e, handle, 0xffff_ffff, 2);
    if end == 0xffff_ffff {
        if doserrno_value(e) == 0x83 {
            return OpenStep::Done;
        }
        return OpenStep::Failed;
    }
    let (read, last) = e.with_stack(4, |e, byte| {
        e.mem.set_u32(byte.addr(), 0);
        let read = e.call(READ_NOLOCK, &args![handle, byte, 1u32]).u32();
        (read, e.mem.u16(byte.addr()))
    });
    if read == 0 && last == 0x1a {
        let size = end as i32 as i64 as u64;
        let cut = e
            .call(
                CHSIZE_NOLOCK,
                &args![handle, size as u32, (size >> 32) as u32],
            )
            .u32();
        if cut == 0xffff_ffff {
            return OpenStep::Failed;
        }
    }
    if lseek_nolock(e, handle, 0, 0) == 0xffff_ffff {
        return OpenStep::Failed;
    }
    OpenStep::Done
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ed_db9a, security_init_cookie()),
        entry!(0x00ec_c35d, tmain_crt_startup() -> u32),
        entry!(0x00ed_d9e7, crt_get_environment_strings_a() -> u32),
        entry!(0x00ed_d92c, setargv() -> u32),
        entry!(0x00ee_50ef, wtsopen_nolock(Ptr, Ptr, Ptr, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::{
        DECODE_POINTER, ERROR_TABLE, GETPTD_NOEXIT, IMPORT_ENTER_CRITICAL_SECTION,
        IMPORT_INIT_CRITICAL_SECTION_SPIN, IMPORT_LEAVE_CRITICAL_SECTION, IMPORT_SET_STD_HANDLE,
        LOCKS_READY, LOCK_TABLE, NHANDLE, PIOINFO,
    };
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The order the logged calls to `addrs` were made in.
    fn order(e: &Engine, addrs: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addrs.contains(a))
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, value: u32) {
        e.register_double(addr, move |_, _| value.into_ret());
    }

    #[test]
    fn the_cookie_is_made_from_the_clocks_and_ids() {
        let mut e = Engine::new();
        e.map(0x011c_1000, 0x1000);
        e.register(IMPORT_GET_SYSTEM_TIME_AS_FILE_TIME, |e, a| {
            e.mem.set_u32(a[0], 0x1111_0000);
            e.mem.set_u32(a[0] + 4, 0x0000_2222);
            Ret::default()
        });
        answer(&mut e, IMPORT_GET_CURRENT_PROCESS_ID, 0x10);
        answer(&mut e, IMPORT_GET_CURRENT_THREAD_ID, 0x200);
        answer(&mut e, IMPORT_GET_TICK_COUNT, 0x3000);
        e.register(IMPORT_QUERY_PERFORMANCE_COUNTER, |e, a| {
            e.mem.set_u32(a[0], 0x0004_0000);
            e.mem.set_u32(a[0] + 4, 0x0050_0000);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e.mem.set_u32(SECURITY_COOKIE, DEFAULT_COOKIE);
        e.call(0x00ed_db9a, &args![]);
        let value = 0x1111_2222 ^ 0x10 ^ 0x200 ^ 0x3000 ^ 0x0054_0000;
        assert_eq!(e.mem.u32(SECURITY_COOKIE), value);
        assert_eq!(e.mem.u32(SECURITY_COOKIE_COMPLEMENT), !value);
        assert_eq!(
            order(
                &e,
                &[
                    IMPORT_GET_SYSTEM_TIME_AS_FILE_TIME,
                    IMPORT_GET_CURRENT_PROCESS_ID,
                    IMPORT_GET_CURRENT_THREAD_ID,
                    IMPORT_GET_TICK_COUNT,
                    IMPORT_QUERY_PERFORMANCE_COUNTER
                ]
            ),
            [
                IMPORT_GET_SYSTEM_TIME_AS_FILE_TIME,
                IMPORT_GET_CURRENT_PROCESS_ID,
                IMPORT_GET_CURRENT_THREAD_ID,
                IMPORT_GET_TICK_COUNT,
                IMPORT_QUERY_PERFORMANCE_COUNTER
            ]
        );
        // A set cookie is kept; a cookie without high bits is made again and widened.
        e.mem.set_u32(SECURITY_COOKIE, 0x1234_5678);
        security_init_cookie(&mut e);
        assert_eq!(e.mem.u32(SECURITY_COOKIE), 0x1234_5678);
        assert_eq!(e.mem.u32(SECURITY_COOKIE_COMPLEMENT), !0x1234_5678);
        e.register(IMPORT_GET_SYSTEM_TIME_AS_FILE_TIME, |_, _| Ret::default());
        e.register(IMPORT_QUERY_PERFORMANCE_COUNTER, |_, _| 1u32.into_ret());
        e.mem.set_u32(SECURITY_COOKIE, 0x5678);
        security_init_cookie(&mut e);
        let low = 0x10 ^ 0x200 ^ 0x3000;
        assert_eq!(e.mem.u32(SECURITY_COOKIE), low | (low << 16));
        // A result equal to the default becomes bb40e64f.
        answer(&mut e, IMPORT_GET_TICK_COUNT, DEFAULT_COOKIE ^ 0x10 ^ 0x200);
        e.mem.set_u32(SECURITY_COOKIE, 0);
        security_init_cookie(&mut e);
        assert_eq!(e.mem.u32(SECURITY_COOKIE), 0xbb40_e64f);
    }

    const STEPS: [u32; 12] = [
        IMPORT_GET_STARTUP_INFO_A,
        HEAP_INIT,
        MTINIT,
        RTC_INITIALIZE,
        IOINIT,
        IMPORT_GET_COMMAND_LINE_A,
        CRT_GET_ENVIRONMENT_STRINGS_A,
        SETARGV,
        SETENVP,
        CINIT,
        WINCMDLN,
        WIN_MAIN,
    ];

    fn start_up(show: Option<u16>, managed: bool) -> Engine {
        let mut e = Engine::new();
        e.map(IMAGE_BASE, 0x200);
        e.map(0x0126_f000, 0x3000);
        e.mem.set_u16(IMAGE_BASE, 0x5a4d);
        e.mem.set_u32(IMAGE_BASE + 0x3c, 0x80);
        e.mem.set_u32(IMAGE_BASE + 0x80, 0x4550);
        e.mem.set_u16(IMAGE_BASE + 0x98, 0x10b);
        e.mem.set_u32(IMAGE_BASE + 0xf4, 0x10);
        e.mem.set_u32(IMAGE_BASE + 0x168, u32::from(managed));
        e.register(IMPORT_GET_STARTUP_INFO_A, move |e, a| {
            e.mem.write(a[0], &[0; 0x44]);
            Ret::default()
        });
        if let Some(show) = show {
            e.register_double(IMPORT_GET_STARTUP_INFO_A, move |e, a| {
                e.mem.write(a[0], &[0; 0x44]);
                e.mem.set_u32(a[0] + 0x2c, 1);
                e.mem.set_u16(a[0] + 0x30, show);
                Ret::default()
            });
        }
        for (addr, value) in [
            (HEAP_INIT, 1u32),
            (MTINIT, 1),
            (IOINIT, 0),
            (IMPORT_GET_COMMAND_LINE_A, 0x0e00_0000),
            (CRT_GET_ENVIRONMENT_STRINGS_A, 0x0e00_1000),
            (SETARGV, 0),
            (SETENVP, 0),
            (CINIT, 0),
            (WINCMDLN, 0x0e00_0010),
            (WIN_MAIN, 7),
        ] {
            answer(&mut e, addr, value);
        }
        for addr in [RTC_INITIALIZE, FAST_ERROR_EXIT, AMSG_EXIT, EXIT, CEXIT] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn the_start_up_runs_its_steps_and_calls_win_main() {
        let mut e = start_up(None, false);
        assert_eq!(e.call(0x00ec_c35d, &args![]).u32(), 7);
        assert_eq!(order(&e, &STEPS), STEPS);
        assert_eq!(e.mem.u32(ACMDLN), 0x0e00_0000);
        assert_eq!(e.mem.u32(AENVPTR), 0x0e00_1000);
        assert_eq!(
            calls(&e, WIN_MAIN),
            vec![vec![IMAGE_BASE, 0, 0x0e00_0010, 10]]
        );
        assert_eq!(calls(&e, HEAP_INIT), vec![vec![1]]);
        assert_eq!(calls(&e, CINIT), vec![vec![1]]);
        assert_eq!(calls(&e, EXIT), vec![vec![7]]);
        assert_eq!(calls(&e, CEXIT).len(), 1);
        assert!(calls(&e, AMSG_EXIT).is_empty());
        // The start-up's show command, and a managed image skips exit.
        let mut e = start_up(Some(3), true);
        tmain_crt_startup(&mut e);
        assert_eq!(calls(&e, WIN_MAIN)[0][3], 3);
        assert!(calls(&e, EXIT).is_empty());
    }

    #[test]
    fn failed_steps_report_and_go_on() {
        let mut e = start_up(None, false);
        for (addr, value) in [
            (HEAP_INIT, 0u32),
            (MTINIT, 0),
            (IOINIT, 0xffff_ffff),
            (SETARGV, 0xffff_ffff),
            (SETENVP, 0xffff_ffff),
            (CINIT, 0x18),
        ] {
            answer(&mut e, addr, value);
        }
        tmain_crt_startup(&mut e);
        assert_eq!(calls(&e, FAST_ERROR_EXIT), vec![vec![0x1c], vec![0x10]]);
        assert_eq!(
            calls(&e, AMSG_EXIT),
            vec![vec![0x1b], vec![8], vec![9], vec![0x18]]
        );
        assert_eq!(calls(&e, WIN_MAIN).len(), 1);
    }

    fn environment_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0127_0000, 0x2000);
        e.register(MALLOC_CRT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FREE, |_, _| Ret::default());
        e.register(IMPORT_FREE_ENVIRONMENT_STRINGS_W, |_, _| 1u32.into_ret());
        e.register(IMPORT_FREE_ENVIRONMENT_STRINGS_A, |_, _| 1u32.into_ret());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        // WideCharToMultiByte for ASCII: the size, or the bytes.
        e.register(IMPORT_WIDE_CHAR_TO_MULTI_BYTE, |e, a| {
            if a[5] == 0 {
                return a[3].into_ret();
            }
            for i in 0..a[3] {
                let unit = e.mem.u16(a[2] + 2 * i) as u8;
                e.mem.set_u8(a[4] + i, unit);
            }
            a[3].into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn the_wide_environment_is_converted_and_freed() {
        let mut e = environment_engine();
        let units: Vec<u16> = "A=1\0BC=2\0\0".encode_utf16().collect();
        let wide = e.mem.alloc(2 * units.len() as u32);
        for (i, u) in units.iter().enumerate() {
            e.mem.set_u16(wide + 2 * i as u32, *u);
        }
        answer(&mut e, IMPORT_GET_ENVIRONMENT_STRINGS_W, wide);
        let block = e.call(0x00ed_d9e7, &args![]).u32();
        assert_eq!(e.mem.bytes(block, 10), b"A=1\0BC=2\0\0".to_vec());
        assert_eq!(e.mem.u32(ENVIRONMENT_KIND), 1);
        assert_eq!(calls(&e, IMPORT_WIDE_CHAR_TO_MULTI_BYTE)[0][3], 10);
        assert_eq!(
            calls(&e, IMPORT_FREE_ENVIRONMENT_STRINGS_W),
            vec![vec![wide]]
        );
        // A failed conversion frees the block and answers 0.
        e.register(IMPORT_WIDE_CHAR_TO_MULTI_BYTE, |_, a| {
            if a[5] == 0 { 4u32 } else { 0 }.into_ret()
        });
        assert_eq!(crt_get_environment_strings_a(&mut e), 0);
        assert_eq!(calls(&e, FREE).len(), 1);
    }

    #[test]
    fn without_the_wide_call_the_ansi_block_is_copied() {
        let mut e = environment_engine();
        answer(&mut e, IMPORT_GET_ENVIRONMENT_STRINGS_W, 0);
        answer(&mut e, IMPORT_GET_LAST_ERROR, 0x78);
        let ansi = e.mem.alloc(8);
        e.mem.write(ansi, b"X=y\0\0");
        answer(&mut e, IMPORT_GET_ENVIRONMENT_STRINGS, ansi);
        let block = crt_get_environment_strings_a(&mut e);
        assert_eq!(e.mem.bytes(block, 5), b"X=y\0\0".to_vec());
        assert_eq!(calls(&e, MEMCPY)[0][2], 5);
        assert_eq!(e.mem.u32(ENVIRONMENT_KIND), 2);
        assert_eq!(
            calls(&e, IMPORT_FREE_ENVIRONMENT_STRINGS_A),
            vec![vec![ansi]]
        );
        // A kind other than 0, 1 and 2 answers 0.
        e.mem.set_u32(ENVIRONMENT_KIND, 3);
        assert_eq!(crt_get_environment_strings_a(&mut e), 0);
    }

    #[test]
    fn setargv_parses_the_command_line_into_one_block() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x3000);
        e.map(0x0127_0000, 0x2000);
        e.mem.set_u32(MBCTYPE_INITIALIZED, 1);
        e.register(IMPORT_GET_MODULE_FILE_NAME_A, |e, a| {
            e.mem.set_cstr(a[1], b"C:\\G\\FalloutNV.exe");
            18u32.into_ret()
        });
        // parse_cmdline's lead-byte test (`_ismbblead`) and the platform malloc.
        e.register(0x00ed_5c8d, |_, _| 0u32.into_ret());
        let line = e.mem.alloc(0x40);
        e.mem.set_cstr(line, b"\"C:\\G\\FalloutNV.exe\" -a b");
        e.mem.set_u32(ACMDLN, line);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_d92c, &args![]).u32(), 0);
        assert_eq!(e.mem.u32(ARGC), 3);
        let argv = e.mem.u32(ARGV);
        let words: Vec<Vec<u8>> = (0..3)
            .map(|i| e.mem.cstr(e.mem.u32(argv + 4 * i)))
            .collect();
        assert_eq!(
            words,
            [
                b"C:\\G\\FalloutNV.exe".to_vec(),
                b"-a".to_vec(),
                b"b".to_vec()
            ]
        );
        assert_eq!(e.mem.u32(argv + 12), 0);
        assert_eq!(e.mem.u32(PGMPTR), PGMNAME);
        assert_eq!(
            calls(&e, IMPORT_GET_MODULE_FILE_NAME_A),
            vec![vec![0, PGMNAME, 0x104]]
        );
        // With no command line the program's name is parsed.
        e.mem.set_u32(ACMDLN, 0);
        assert_eq!(setargv(&mut e), 0);
        assert_eq!(e.mem.u32(ARGC), 1);
    }

    #[test]
    fn the_wide_open_calls_create_file_w_and_maps_its_failure() {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        e.map(0x011c_1000, 0x2000);
        let block = e.mem.alloc(0x40);
        e.register_double(GETPTD_NOEXIT, move |_, _| block.into_ret());
        e.register(DECODE_POINTER, |_, _| 0x0e30_0000u32.into_ret());
        e.mem.set_u32(LOCKS_READY, 1);
        for number in [1u32, 0xa, 0xb] {
            let section = e.mem.alloc(0x18);
            e.mem.set_u32(LOCK_TABLE + 8 * number, section);
        }
        let entries = e.mem.alloc(0x800);
        for index in 0..0x20 {
            e.mem.set_u32(entries + 0x40 * index, 0xffff_ffff);
        }
        e.mem.set_u32(PIOINFO, entries);
        e.mem.set_u32(NHANDLE, 0x20);
        for target in [
            0x0e30_0000,
            IMPORT_ENTER_CRITICAL_SECTION,
            IMPORT_LEAVE_CRITICAL_SECTION,
            IMPORT_SET_STD_HANDLE,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(IMPORT_INIT_CRITICAL_SECTION_SPIN, |_, _| 1u32.into_ret());
        e.register(IMPORT_CREATE_FILE_W, |_, _| 0x500u32.into_ret());
        e.register(IMPORT_GET_FILE_TYPE, |_, _| 1u32.into_ret());
        e.register(IMPORT_CLOSE_HANDLE, |_, _| 1u32.into_ret());
        e.register(GET_FMODE, |e, a| {
            e.mem.set_u32(a[0], 0);
            0u32.into_ret()
        });
        e.register(IMPORT_GET_LAST_ERROR, |_, _| 2u32.into_ret());
        e.call_log = Some(vec![]);
        let path = e.mem.alloc(8);
        let handle = e.mem.alloc(4);
        let unlock = e.mem.alloc(4);
        // _O_RDONLY | _O_BINARY, _SH_DENYNO.
        assert_eq!(
            e.call(
                0x00ee_50ef,
                &args![handle, unlock, path, 0x8000u32, 0x40u32, 0x180u32]
            )
            .u32(),
            0
        );
        assert_eq!(e.mem.u32(handle), 0);
        assert_eq!(e.mem.u32(unlock), 1);
        assert_eq!(e.mem.u32(entries), 0x500);
        let create = &calls(&e, IMPORT_CREATE_FILE_W)[0];
        assert_eq!(create[..3], [path, 0x8000_0000, 3]);
        assert_eq!(create[4..], [3, 0x80, 0]);
        // A failed CreateFileW: the error mapped through the table.
        e.register(IMPORT_CREATE_FILE_W, |_, _| 0xffff_ffffu32.into_ret());
        e.mem.set_u32(ERROR_TABLE, 2);
        e.mem.set_u32(ERROR_TABLE + 4, 2);
        assert_eq!(
            wtsopen_nolock(
                &mut e,
                Ptr::new(handle),
                Ptr::new(unlock),
                Ptr::new(path),
                0x8000,
                0x40,
                0x180
            ),
            2
        );
        assert_eq!(e.mem.u8(entries + 0x44), 0);
    }
}
