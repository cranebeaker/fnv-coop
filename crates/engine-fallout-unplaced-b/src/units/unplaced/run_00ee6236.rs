//! `(unplaced)/run_00ee6236` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The slot `00ee6d44` jumps through.
const IMPORT_SLOT_00EE6D44: u32 = 0x00fd_f33c;

// Translated from 00ee6d44 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf33c]` (the import slot; Ghidra labels the import `closesocket`, unconfirmed beyond that label). stdcall (the callee pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d44(e: &mut Engine, s: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D44, &args![s]).i32()
}

/// The slot `00ee6d4a` jumps through.
const IMPORT_SLOT_00EE6D4A: u32 = 0x00fd_f340;

// Translated from 00ee6d4a (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf340]` (the import slot; Ghidra labels the import `socket`, unconfirmed beyond that label). stdcall (the callee pops its words): the 3 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d4a(e: &mut Engine, af: u32, socket_type: u32, protocol: u32) -> u32 {
    e.call(IMPORT_SLOT_00EE6D4A, &args![af, socket_type, protocol])
        .u32()
}

/// The slot `00ee6d50` jumps through.
const IMPORT_SLOT_00EE6D50: u32 = 0x00fd_f344;

// Translated from 00ee6d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf344]` (the import slot; Ghidra labels the import `WSAGetLastError`, unconfirmed beyond that label). stdcall (the callee pops its words): the 0 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d50(e: &mut Engine) -> i32 {
    e.call(IMPORT_SLOT_00EE6D50, &args![]).i32()
}

/// The slot `00ee6d56` jumps through.
const IMPORT_SLOT_00EE6D56: u32 = 0x00fd_f348;

// Translated from 00ee6d56 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf348]` (the import slot; Ghidra labels the import `recv`, unconfirmed beyond that label). stdcall (the callee pops its words): the 4 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d56(e: &mut Engine, s: u32, buf: Ptr, len: u32, flags: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D56, &args![s, buf, len, flags])
        .i32()
}

/// The slot `00ee6d5c` jumps through.
const IMPORT_SLOT_00EE6D5C: u32 = 0x00fd_f34c;

// Translated from 00ee6d5c (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf34c]` (the import slot; Ghidra labels the import `send`, unconfirmed beyond that label). stdcall (the callee pops its words): the 4 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d5c(e: &mut Engine, s: u32, buf: Ptr, len: u32, flags: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D5C, &args![s, buf, len, flags])
        .i32()
}

/// The slot `00ee6d62` jumps through.
const IMPORT_SLOT_00EE6D62: u32 = 0x00fd_f350;

// Translated from 00ee6d62 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf350]` (the import slot; Ghidra labels the import `connect`, unconfirmed beyond that label). stdcall (the callee pops its words): the 3 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d62(e: &mut Engine, s: u32, name: Ptr, namelen: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D62, &args![s, name, namelen]).i32()
}

/// The slot `00ee6d68` jumps through.
const IMPORT_SLOT_00EE6D68: u32 = 0x00fd_f354;

// Translated from 00ee6d68 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf354]` (the import slot; Ghidra labels the import `gethostbyname`, unconfirmed beyond that label). stdcall (the callee pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d68(e: &mut Engine, name: Ptr) -> Ptr {
    e.call(IMPORT_SLOT_00EE6D68, &args![name]).ptr()
}

/// The slot `00ee6d6e` jumps through.
const IMPORT_SLOT_00EE6D6E: u32 = 0x00fd_f358;

// Translated from 00ee6d6e (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf358]` (the import slot; Ghidra labels the import `inet_addr`, unconfirmed beyond that label). stdcall (the callee pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d6e(e: &mut Engine, cp: Ptr) -> u32 {
    e.call(IMPORT_SLOT_00EE6D6E, &args![cp]).u32()
}

/// The slot `00ee6d74` jumps through.
const IMPORT_SLOT_00EE6D74: u32 = 0x00fd_f35c;

// Translated from 00ee6d74 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf35c]` (the import slot; Ghidra labels the import `htons`, unconfirmed beyond that label). stdcall (the callee pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra; the result is the low 16 bits of `EAX`.
pub fn fn_00ee6d74(e: &mut Engine, hostshort: u16) -> u16 {
    e.call(IMPORT_SLOT_00EE6D74, &args![hostshort]).u16()
}

/// The slot `00ee6d7a` jumps through.
const IMPORT_SLOT_00EE6D7A: u32 = 0x00fd_f38c;

// Translated from 00ee6d7a (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf38c]` (the import slot; Ghidra labels the import `WSAAsyncSelect`, unconfirmed beyond that label). stdcall (the callee pops its words): the 4 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d7a(e: &mut Engine, s: u32, hwnd: u32, msg: u32, event: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D7A, &args![s, hwnd, msg, event])
        .i32()
}

/// The slot `00ee6d86` jumps through.
const IMPORT_SLOT_00EE6D86: u32 = 0x00fd_f364;

// Translated from 00ee6d86 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf364]` (the import slot; Ghidra labels the import `WSAStartup`, unconfirmed beyond that label). stdcall (the callee pops its words): the 2 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d86(e: &mut Engine, version_requested: u16, wsa_data: Ptr) -> i32 {
    e.call(IMPORT_SLOT_00EE6D86, &args![version_requested, wsa_data])
        .i32()
}

/// The slot `00ee6d8c` jumps through.
const IMPORT_SLOT_00EE6D8C: u32 = 0x00fd_f368;

// Translated from 00ee6d8c (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf368]` (the import slot; Ghidra labels the import `gethostname`, unconfirmed beyond that label). stdcall (the callee pops its words): the 2 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d8c(e: &mut Engine, name: Ptr, namelen: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D8C, &args![name, namelen]).i32()
}

/// The slot `00ee6d92` jumps through.
const IMPORT_SLOT_00EE6D92: u32 = 0x00fd_f36c;

// Translated from 00ee6d92 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf36c]` (the import slot; Ghidra labels the import `listen`, unconfirmed beyond that label). stdcall (the callee pops its words): the 2 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d92(e: &mut Engine, s: u32, backlog: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D92, &args![s, backlog]).i32()
}

/// The slot `00ee6d98` jumps through.
const IMPORT_SLOT_00EE6D98: u32 = 0x00fd_f370;

// Translated from 00ee6d98 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf370]` (the import slot; Ghidra labels the import `bind`, unconfirmed beyond that label). stdcall (the callee pops its words): the 3 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d98(e: &mut Engine, s: u32, addr: Ptr, namelen: u32) -> i32 {
    e.call(IMPORT_SLOT_00EE6D98, &args![s, addr, namelen]).i32()
}

/// The slot `00ee6d9e` jumps through.
const IMPORT_SLOT_00EE6D9E: u32 = 0x00fd_f374;

// Translated from 00ee6d9e (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf374]` (the import slot; Ghidra labels the import `setsockopt`, unconfirmed beyond that label). stdcall (the callee pops its words): the 5 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6d9e(
    e: &mut Engine,
    s: u32,
    level: u32,
    optname: u32,
    optval: Ptr,
    optlen: u32,
) -> i32 {
    e.call(
        IMPORT_SLOT_00EE6D9E,
        &args![s, level, optname, optval, optlen],
    )
    .i32()
}

/// The slot `00ee6da4` jumps through.
const IMPORT_SLOT_00EE6DA4: u32 = 0x00fd_f378;

// Translated from 00ee6da4 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf378]` (the import slot; Ghidra labels the import `inet_ntoa`, unconfirmed beyond that label). stdcall (the callee pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra (the `in_addr` is one word).
pub fn fn_00ee6da4(e: &mut Engine, in_addr: u32) -> Ptr {
    e.call(IMPORT_SLOT_00EE6DA4, &args![in_addr]).ptr()
}

/// The slot `00ee6daa` jumps through.
const IMPORT_SLOT_00EE6DAA: u32 = 0x00fd_f388;

// Translated from 00ee6daa (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf388]` (the import slot; Ghidra labels the import `ntohs`, unconfirmed beyond that label). stdcall (the callee pops its words): the 1 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra; the result is the low 16 bits of `EAX`.
pub fn fn_00ee6daa(e: &mut Engine, netshort: u16) -> u16 {
    e.call(IMPORT_SLOT_00EE6DAA, &args![netshort]).u16()
}

/// The slot `00ee6db0` jumps through.
const IMPORT_SLOT_00EE6DB0: u32 = 0x00fd_f384;

// Translated from 00ee6db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf384]` (the import slot; Ghidra labels the import `accept`, unconfirmed beyond that label). stdcall (the callee pops its words): the 3 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6db0(e: &mut Engine, s: u32, addr: Ptr, addrlen: Ptr) -> u32 {
    e.call(IMPORT_SLOT_00EE6DB0, &args![s, addr, addrlen]).u32()
}

/// The slot `00ee6db6` jumps through.
const IMPORT_SLOT_00EE6DB6: u32 = 0x00fd_f380;

// Translated from 00ee6db6 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf380]` (the import slot; Ghidra labels the import `__WSAFDIsSet`, unconfirmed beyond that label). stdcall (the callee pops its words): the 2 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra (its parameters are unnamed there).
pub fn fn_00ee6db6(e: &mut Engine, s: u32, set: Ptr) -> i32 {
    e.call(IMPORT_SLOT_00EE6DB6, &args![s, set]).i32()
}

/// The slot `00ee6dbc` jumps through.
const IMPORT_SLOT_00EE6DBC: u32 = 0x00fd_f37c;

// Translated from 00ee6dbc (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf37c]` (the import slot; Ghidra labels the import `select`, unconfirmed beyond that label). stdcall (the callee pops its words): the 5 stack word(s) go unchanged to the import and its result comes back. Signature from Ghidra.
pub fn fn_00ee6dbc(
    e: &mut Engine,
    nfds: u32,
    readfds: Ptr,
    writefds: Ptr,
    exceptfds: Ptr,
    timeout: Ptr,
) -> i32 {
    e.call(
        IMPORT_SLOT_00EE6DBC,
        &args![nfds, readfds, writefds, exceptfds, timeout],
    )
    .i32()
}

/// The slot `00ee6dc2` jumps through.
const IMPORT_SLOT_00EE6DC2: u32 = 0x00fd_f3d4;

// Translated from 00ee6dc2 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf3d4]` (the import slot; Ghidra labels the import `D3DXSaveTextureToFileA`, unconfirmed beyond that label). stdcall (the callee pops its words): the 4 stack word(s) go unchanged to the import and its result comes back. Argument count from the only caller at 007a0797: four words (the first pushed is 0, then a texture word from a virtual call with no stack argument, 3, and a path buffer); the result is not used there.
pub fn fn_00ee6dc2(e: &mut Engine, word_0: Ptr, word_1: u32, word_2: u32, word_3: u32) -> u32 {
    e.call(IMPORT_SLOT_00EE6DC2, &args![word_0, word_1, word_2, word_3])
        .u32()
}

/// The slot `00ee6dc8` jumps through.
const IMPORT_SLOT_00EE6DC8: u32 = 0x00fd_f418;

// Translated from 00ee6dc8 (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf418]` (the import slot; Ghidra labels the import `D3DXPlaneTransform`, unconfirmed beyond that label). stdcall (the callee pops its words): the 3 stack word(s) go unchanged to the import and its result comes back. Argument count from the callers (three address pushes at 004ed0b1; the result is not used there).
pub fn fn_00ee6dc8(e: &mut Engine, word_0: Ptr, word_1: Ptr, word_2: Ptr) -> Ptr {
    e.call(IMPORT_SLOT_00EE6DC8, &args![word_0, word_1, word_2])
        .ptr()
}

/// The slot `00ee6dce` jumps through.
const IMPORT_SLOT_00EE6DCE: u32 = 0x00fd_f430;

// Translated from 00ee6dce (decompiled, FalloutNV.exe 1.4.0.525)
/// Import thunk `JMP dword ptr [0x00fdf430]` (the import slot; Ghidra labels the import `D3DXPlaneNormalize`, unconfirmed beyond that label). stdcall (the callee pops its words): the 2 stack word(s) go unchanged to the import and its result comes back. Argument count from the callers (two address pushes at 004ed097; the result is not used there).
pub fn fn_00ee6dce(e: &mut Engine, word_0: Ptr, word_1: Ptr) -> Ptr {
    e.call(IMPORT_SLOT_00EE6DCE, &args![word_0, word_1]).ptr()
}

/// The import slot of KERNEL32 `CreateFileA` (`00fdf088`, stdcall, seven words).
const CREATE_FILE_A: u32 = 0x00fd_f088;
/// `"CONOUT$"` in `.rdata`, the file name `___initconout` opens.
const CONOUT_NAME: u32 = 0x0110_08b4;
/// The CRT's console output handle, written by `___initconout`.
const CONSOLE_OUTPUT_HANDLE: u32 = 0x011c_2f74;

// Translated from 00ee626f (decompiled, FalloutNV.exe 1.4.0.525)
/// `___initconout` (Ghidra label from its function ID; `__putwch_nolock` calls it,
/// `00ee46d5`): opens the console output device and keeps the handle. It calls
/// `CreateFileA("CONOUT$", GENERIC_WRITE (0x40000000), FILE_SHARE_READ | FILE_SHARE_WRITE (3),
/// NULL, OPEN_EXISTING (3), 0, NULL)` and stores the result (`INVALID_HANDLE_VALUE` when
/// there is no console) in `011c2f74`. No arguments, nothing returned.
pub fn fn_00ee626f(e: &mut Engine) {
    let handle = e
        .call(
            CREATE_FILE_A,
            &args![CONOUT_NAME, 0x4000_0000u32, 3u32, 0u32, 3u32, 0u32, 0u32],
        )
        .u32();
    e.set_global(CONSOLE_OUTPUT_HANDLE, handle);
}

/// Import slot of KERNEL32 `CloseHandle` (stdcall, one word).
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
/// The CRT's second console handle, the word before [`CONSOLE_OUTPUT_HANDLE`].
const CONSOLE_SECOND_HANDLE: u32 = 0x011c_2f70;

// Translated from 00ee628e (decompiled, FalloutNV.exe 1.4.0.525)
/// The exit-time partner of `___initconout` (no Ghidra function; the CRT
/// pre-terminator table entry at `010113f8`, disassembled from its bytes):
/// closes the console handle in `011c2f74` and then the one in `011c2f70`
/// with `CloseHandle`, each only when it is neither -1 nor -2. The globals
/// keep their values. No arguments, nothing returned.
pub fn fn_00ee628e(e: &mut Engine) {
    for slot in [CONSOLE_OUTPUT_HANDLE, CONSOLE_SECOND_HANDLE] {
        let handle = e.global::<u32>(slot);
        if handle != u32::MAX && handle != 0xffff_fffe {
            e.call(CLOSE_HANDLE, &args![handle]);
        }
    }
}

/// `_errno` (`00ec85e3`): the address of the thread's `errno`.
const ERRNO: u32 = 0x00ec_85e3;
/// `_invalid_parameter` (`00ec7c30`, five words, all zero here).
const INVALID_PARAMETER: u32 = 0x00ec_7c30;
/// The CRT's `_fmode` (`012708dc`), the default file translation mode.
const FMODE: u32 = 0x0127_08dc;

// Translated from 00ee6236 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__get_fmode` (Ghidra library match; cdecl, one word): with a null `mode`
/// sets `errno` to `0x16` (`EINVAL`), calls `_invalid_parameter(0, 0, 0, 0,
/// 0)` and returns `0x16`; otherwise stores `_fmode` (`012708dc`) in `*mode`
/// and returns 0.
pub fn get_fmode(e: &mut Engine, mode: Ptr) -> u32 {
    if mode.is_null() {
        let error = e.call(ERRNO, &args![]).u32();
        e.mem.set_u32(error, 0x16);
        e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
        return 0x16;
    }
    let value = e.global::<u32>(FMODE);
    e.mem.set_u32(mode.addr(), value);
    0
}

/// `_LocaleUpdate::_LocaleUpdate` (`00ec660e`, thiscall, one word: the `_locale_t`).
const LOCALE_UPDATE: u32 = 0x00ec_660e;
/// The `_LocaleUpdate` local (0x10 bytes): `locinfo` at +0, `mbcinfo` at +4,
/// the thread data at +8 and the "updated" byte at +0xc.
const LOCALE_UPDATE_SIZE: u32 = 0x10;
const LOCALE_UPDATE_MBCINFO: u32 = 0x4;
const LOCALE_UPDATE_THREAD_DATA: u32 = 0x8;
const LOCALE_UPDATE_UPDATED: u32 = 0xc;
/// `___crtCompareStringA` (`00ee67e8`, cdecl, eight words).
const CRT_COMPARE_STRING_A: u32 = 0x00ee_67e8;
/// `__strnicmp_l` (`00ec7dce`, cdecl, four words).
const STRNICMP_L: u32 = 0x00ec_7dce;
/// `NORM_IGNORECASE | SORT_STRINGSORT`, the compare flags both functions pass.
const IGNORE_CASE_STRING_SORT: u32 = 0x1001;

/// The end of the `_LocaleUpdate` scope: when it changed the thread's locale
/// (its byte at +0xc), clears bit 1 of the thread data's word at +0x70.
fn end_locale_update(e: &mut Engine, update: u32) {
    if e.mem.u8(update + LOCALE_UPDATE_UPDATED) != 0 {
        let thread_data = e.mem.u32(update + LOCALE_UPDATE_THREAD_DATA);
        let flags = e.mem.u32(thread_data + 0x70);
        e.mem.set_u32(thread_data + 0x70, flags & 0xffff_fffd);
    }
}

/// The shared argument check of `_mbsnbicoll_l` and `_strnicoll_l`, after the
/// `_LocaleUpdate`: `None` to go on, or the result to return (0 for a count
/// of 0; `0x7fffffff` after `errno = EINVAL` and `_invalid_parameter` for a
/// null string). The count check above `0x7fffffff` is the caller's (its
/// error path differs in what follows).
fn check_collate_arguments(
    e: &mut Engine,
    update: u32,
    first: Ptr,
    second: Ptr,
    count: u32,
) -> Option<i32> {
    if count == 0 {
        end_locale_update(e, update);
        return Some(0);
    }
    if first.is_null() || second.is_null() {
        invalid_parameter(e);
        end_locale_update(e, update);
        return Some(0x7fff_ffff);
    }
    None
}

/// `errno = EINVAL (0x16)` and `_invalid_parameter(0, 0, 0, 0, 0)`.
fn invalid_parameter(e: &mut Engine) {
    let error = e.call(ERRNO, &args![]).u32();
    e.mem.set_u32(error, 0x16);
    e.call(INVALID_PARAMETER, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
}

// Translated from 00ee62bb (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mbsnbicoll_l` (Ghidra library match; cdecl, four words): with a
/// `_LocaleUpdate` for `locale` on the stack, 0 for a `count` of 0;
/// `0x7fffffff` (after `errno = EINVAL` and `_invalid_parameter`) for a null
/// string or a `count` above `0x7fffffff`. When the multibyte info
/// (`mbcinfo`) says the code page is not multibyte (`ismbcodepage`, +8, is
/// 0) the result is `__strnicoll_l(first, second, count, locale)`;
/// otherwise `___crtCompareStringA(&update, <mbcinfo +0xc>, 0x1001, first,
/// count, second, count, <mbcinfo +4, the code page>)` less 2, or
/// `0x7fffffff` when that returns 0 (no `errno` then).
pub fn mbsnbicoll_l(e: &mut Engine, first: Ptr, second: Ptr, count: u32, locale: u32) -> i32 {
    e.with_stack(LOCALE_UPDATE_SIZE, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        if let Some(result) = check_collate_arguments(e, update, first, second, count) {
            return result;
        }
        if count > 0x7fff_ffff {
            invalid_parameter(e);
            end_locale_update(e, update);
            return 0x7fff_ffff;
        }
        let mbcinfo = e.mem.u32(update + LOCALE_UPDATE_MBCINFO);
        let result = if e.mem.u32(mbcinfo + 8) == 0 {
            strnicoll_l(e, first, second, count, locale)
        } else {
            let name = e.mem.u32(mbcinfo + 0xc);
            let code_page = e.mem.u32(mbcinfo + 4);
            let compared = e
                .call(
                    CRT_COMPARE_STRING_A,
                    &args![
                        update,
                        name,
                        IGNORE_CASE_STRING_SORT,
                        first,
                        count,
                        second,
                        count,
                        code_page
                    ],
                )
                .i32();
            if compared == 0 {
                end_locale_update(e, update);
                return 0x7fff_ffff;
            }
            compared.wrapping_sub(2)
        };
        end_locale_update(e, update);
        result
    })
}

// Translated from 00ee63a9 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mbsnbicoll` (Ghidra library match; cdecl, three words):
/// `__mbsnbicoll_l(first, second, count, NULL)`.
pub fn mbsnbicoll(e: &mut Engine, first: Ptr, second: Ptr, count: u32) -> i32 {
    mbsnbicoll_l(e, first, second, count, 0)
}

// Translated from 00ee682a (decompiled, FalloutNV.exe 1.4.0.525)
/// `__strnicoll_l` (Ghidra library match; cdecl, four words): with a
/// `_LocaleUpdate` for `locale` on the stack, 0 for a `count` of 0;
/// `0x7fffffff` (after `errno = EINVAL` and `_invalid_parameter`) for a null
/// string or a `count` above `0x7fffffff`. When the locale's collate name
/// (`locinfo + 0x10`) is null the result is `__strnicmp_l(first, second,
/// count, &update)`; otherwise `___crtCompareStringA(&update, name, 0x1001,
/// first, count, second, count, <locinfo +8, the collate code page>)` less
/// 2, or `0x7fffffff` with `errno = EINVAL` (no `_invalid_parameter`) when
/// that returns 0.
pub fn strnicoll_l(e: &mut Engine, first: Ptr, second: Ptr, count: u32, locale: u32) -> i32 {
    e.with_stack(LOCALE_UPDATE_SIZE, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        if let Some(result) = check_collate_arguments(e, update, first, second, count) {
            return result;
        }
        if count > 0x7fff_ffff {
            invalid_parameter(e);
            end_locale_update(e, update);
            return 0x7fff_ffff;
        }
        let locinfo = e.mem.u32(update);
        let name = e.mem.u32(locinfo + 0x10);
        let result = if name == 0 {
            e.call(STRNICMP_L, &args![first, second, count, update])
                .i32()
        } else {
            let code_page = e.mem.u32(locinfo + 8);
            let compared = e
                .call(
                    CRT_COMPARE_STRING_A,
                    &args![
                        update,
                        name,
                        IGNORE_CASE_STRING_SORT,
                        first,
                        count,
                        second,
                        count,
                        code_page
                    ],
                )
                .i32();
            if compared == 0 {
                let error = e.call(ERRNO, &args![]).u32();
                e.mem.set_u32(error, 0x16);
                end_locale_update(e, update);
                return 0x7fff_ffff;
            }
            compared.wrapping_sub(2)
        };
        end_locale_update(e, update);
        result
    })
}

/// `free` (`00ecd291`, cdecl, one word), `__calloc_crt` (`00ed0d24`, cdecl: count, size),
/// `__malloc_crt` (`00ed0cdf`, the map's `malloc`; cdecl, one word), `__recalloc_crt`
/// (`00ed0dbe`, cdecl: block, count, size) and `malloc` (`00ecd1c7`, cdecl, one word).
const FREE: u32 = 0x00ec_d291;
const CALLOC_CRT: u32 = 0x00ed_0d24;
const MALLOC_CRT: u32 = 0x00ed_0cdf;
const RECALLOC_CRT: u32 = 0x00ed_0dbe;
const MALLOC: u32 = 0x00ec_d1c7;
/// `strlen` (`00ec6130`), `strcpy_s` (`00ec65a6`, cdecl: destination, size, source), `strchr`
/// (`00ec7690`, cdecl: string, character) and `__invoke_watson` (`00ec7b08`, five words).
const STRLEN: u32 = 0x00ec_6130;
const STRCPY_S: u32 = 0x00ec_65a6;
const STRCHR: u32 = 0x00ec_7690;
const INVOKE_WATSON: u32 = 0x00ec_7b08;
/// `_amsg_exit` (`00eccc55`, cdecl, one word: the run-time error number).
const AMSG_EXIT: u32 = 0x00ec_cc55;
/// The CRT's `_environ` (`0126fdc8`), `__initenv` (`0126fdcc`) and `_wenviron` (`0126fdd0`).
const ENVIRON: u32 = 0x0126_fdc8;
const INITIAL_ENVIRON: u32 = 0x0126_fdcc;
const WIDE_ENVIRON: u32 = 0x0126_fdd0;
/// Import slots of KERNEL32 `WideCharToMultiByte` (eight words), `MultiByteToWideChar` (six),
/// `GetCPInfo` (two), `CompareStringW` (six), `GetLastError` (none), `CompareStringA` (six)
/// and `SetEnvironmentVariableA` (two); all stdcall.
const WIDE_CHAR_TO_MULTI_BYTE: u32 = 0x00fd_f170;
const MULTI_BYTE_TO_WIDE_CHAR: u32 = 0x00fd_f128;
const GET_CP_INFO: u32 = 0x00fd_f1d8;
const COMPARE_STRING_W: u32 = 0x00fd_f1f8;
const GET_LAST_ERROR: u32 = 0x00fd_f094;
const COMPARE_STRING_A_IMPORT: u32 = 0x00fd_f208;
const SET_ENVIRONMENT_VARIABLE_A: u32 = 0x00fd_f1f4;
/// `___ansicp` (`00ee13e5`, cdecl, one word: the LCID), `___convertcp` (`00ee142e`, cdecl,
/// six words) and `__freea` (`00ec8e14`, cdecl, one word).
const ANSICP: u32 = 0x00ee_13e5;
const CONVERTCP: u32 = 0x00ee_142e;
const FREEA: u32 = 0x00ec_8e14;
/// `__crtCompareStringA_stat`'s static choice of API (`012708e0`): 0 not decided yet, 1
/// `CompareStringW` works, 2 it is not implemented (`ERROR_CALL_NOT_IMPLEMENTED`, 0x78).
const COMPARE_STRING_MODE: u32 = 0x0127_08e0;
/// The one-character wide string `CompareStringW` is tried on (`01011d78`).
const COMPARE_STRING_PROBE: u32 = 0x0101_1d78;

// Translated from 00ee645a (decompiled, FalloutNV.exe 1.4.0.525)
/// `strncnt` (Ghidra library match; register argument `string` in `EAX`, `count` the one stack
/// word): the number of bytes of `string` before its terminator, at most `count`.
pub fn strncnt(e: &mut Engine, string: Ptr, count: i32) -> i32 {
    let mut left = count;
    let mut at = string.addr();
    loop {
        if left == 0 {
            left = left.wrapping_sub(1);
            break;
        }
        left = left.wrapping_sub(1);
        if e.mem.u8(at) == 0 {
            break;
        }
        at = at.wrapping_add(1);
    }
    count.wrapping_sub(left).wrapping_sub(1)
}

/// The `_alloca`/`malloc` buffer of `__crtCompareStringA_stat` for `count` wide characters:
/// none (address 0) unless `count` is positive and `0xffffffe0 / count` is at least 2; a
/// stack block (`_alloca` through the probe `00ec5ec0`, its first word `0xcccc`) when
/// `count * 2 + 8` is at most 0x400, else `malloc` (first word `0xdddd`, nothing when it
/// fails); `run` gets the address 8 bytes in, or 0.
fn with_wide_buffer(e: &mut Engine, count: u32, run: impl FnOnce(&mut Engine, u32) -> i32) -> i32 {
    if (count as i32) <= 0 || 0xffff_ffe0u32 / count < 2 {
        return run(e, 0);
    }
    let size = count.wrapping_mul(2).wrapping_add(8);
    if size <= 0x400 {
        e.with_stack(size, |e, block| {
            let block = block.addr();
            e.mem.set_u32(block, 0xcccc);
            run(e, block.wrapping_add(8))
        })
    } else {
        let block = e.call(MALLOC, &args![size]).u32();
        if block == 0 {
            return run(e, 0);
        }
        e.mem.set_u32(block, 0xdddd);
        run(e, block.wrapping_add(8))
    }
}

/// The lead-byte test of `__crtCompareStringA_stat` for a one-byte side: whether the first
/// byte of `string` falls in one of the `LeadByte` ranges of the `CPINFO` at `info` (pairs
/// from +6, ended by a zero byte in either place).
fn is_lead_byte(e: &mut Engine, info: u32, string: u32) -> bool {
    let mut range = info.wrapping_add(6);
    if e.mem.u8(range) == 0 {
        return false;
    }
    loop {
        let high = e.mem.u8(range.wrapping_add(1));
        if high == 0 {
            return false;
        }
        let byte = e.mem.u8(string);
        if byte >= e.mem.u8(range) && byte <= high {
            return true;
        }
        range = range.wrapping_add(2);
        if e.mem.u8(range) == 0 {
            return false;
        }
    }
}

// Translation incomplete: 00ee6478 - the /GS security-cookie check (00ec408c) is not translated; the _alloca buffers (probe 00ec5ec0) are with_stack blocks of the exact size.
// Translated from 00ee6478 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__crtCompareStringA_stat` (Ghidra library match; register arguments: `update`, the
/// `_LocaleUpdate`, in `ECX` and `string1` in `EDX`, then six stack words, cdecl). On its
/// first use it tries `CompareStringW(0, 0, L"..", 1, L"..", 1)` (`01011d78`) and keeps in
/// `012708e0` 1 when that works, or 2 when it fails with `ERROR_CALL_NOT_IMPLEMENTED` (0x78).
/// Positive counts are cut to the strings' lengths (`strncnt`); a count below -1 answers 0.
/// With mode 1: a zero code page is the locale's (`locinfo +4`); when either count is 0 it
/// answers 2 for equal counts, 1 when the second is above 1, 3 when the first is, and
/// otherwise looks at the code page (`GetCPInfo`, 0 when that fails): a one-character first
/// string answers 3 (2 when it is a lead byte), a one-character second string 1 (2 for a
/// lead byte); else (and when neither count is 0) both strings are widened
/// (`MultiByteToWideChar`, flags 9 to size, 1 to convert) into `_alloca`/`malloc` buffers
/// and compared by `CompareStringW(locale, flags, ..)`; 0 when any step fails. With mode 0
/// or 2: a zero LCID is the locale's collate one (`locinfo +0x14`), a zero code page the
/// locale's; the strings are converted to the LCID's ANSI code page (`___ansicp`,
/// `___convertcp`) when it differs, and compared by `CompareStringA`. Any other mode answers
/// 0.
#[allow(clippy::too_many_arguments)]
pub fn crt_compare_string_a_stat(
    e: &mut Engine,
    update: Ptr,
    string1: Ptr,
    locale: u32,
    flags: u32,
    count1: i32,
    string2: Ptr,
    count2: i32,
    code_page: u32,
) -> i32 {
    let update = update.addr();
    let (string1, string2) = (string1.addr(), string2.addr());
    if e.mem.u32(COMPARE_STRING_MODE) == 0 {
        let probe = e
            .call(
                COMPARE_STRING_W,
                &args![
                    0u32,
                    0u32,
                    COMPARE_STRING_PROBE,
                    1u32,
                    COMPARE_STRING_PROBE,
                    1u32
                ],
            )
            .u32();
        if probe != 0 {
            e.mem.set_u32(COMPARE_STRING_MODE, 1);
        } else if e.call(GET_LAST_ERROR, &args![]).u32() == 0x78 {
            e.mem.set_u32(COMPARE_STRING_MODE, 2);
        }
    }
    let mut count1 = count1;
    let mut count2 = count2;
    if count1 > 0 {
        count1 = strncnt(e, Ptr::new(string1), count1);
    } else if count1 < -1 {
        return 0;
    }
    if count2 > 0 {
        count2 = strncnt(e, Ptr::new(string2), count2);
    } else if count2 < -1 {
        return 0;
    }
    let mode = e.mem.u32(COMPARE_STRING_MODE);
    if mode == 2 || mode == 0 {
        return compare_ansi(
            e, update, string1, locale, flags, count1, string2, count2, code_page,
        );
    }
    if mode != 1 {
        return 0;
    }
    let code_page = if code_page == 0 {
        let locinfo = e.mem.u32(update);
        e.mem.u32(locinfo.wrapping_add(4))
    } else {
        code_page
    };
    if count1 == 0 || count2 == 0 {
        if count1 == count2 {
            return 2;
        }
        if count2 > 1 {
            return 1;
        }
        if count1 > 1 {
            return 3;
        }
        let answer = e.with_stack(0x14, |e, info| {
            let info = info.addr();
            if e.call(GET_CP_INFO, &args![code_page, info]).u32() == 0 {
                return Some(0);
            }
            if count1 > 0 {
                if e.mem.u32(info) < 2 || !is_lead_byte(e, info, string1) {
                    return Some(3);
                }
                return Some(2);
            }
            if count2 > 0 {
                if e.mem.u32(info) < 2 || !is_lead_byte(e, info, string2) {
                    return Some(1);
                }
                return Some(2);
            }
            None
        });
        if let Some(answer) = answer {
            return answer;
        }
    }
    let wide1 = e
        .call(
            MULTI_BYTE_TO_WIDE_CHAR,
            &args![code_page, 9u32, string1, count1, 0u32, 0u32],
        )
        .u32();
    if wide1 == 0 {
        return 0;
    }
    with_wide_buffer(e, wide1, |e, buffer1| {
        if buffer1 == 0 {
            return 0;
        }
        let mut result = 0;
        let converted = e
            .call(
                MULTI_BYTE_TO_WIDE_CHAR,
                &args![code_page, 1u32, string1, count1, buffer1, wide1],
            )
            .u32();
        if converted != 0 {
            let wide2 = e
                .call(
                    MULTI_BYTE_TO_WIDE_CHAR,
                    &args![code_page, 9u32, string2, count2, 0u32, 0u32],
                )
                .u32();
            if wide2 != 0 {
                result = with_wide_buffer(e, wide2, |e, buffer2| {
                    if buffer2 == 0 {
                        return 0;
                    }
                    let mut result = 0;
                    let converted = e
                        .call(
                            MULTI_BYTE_TO_WIDE_CHAR,
                            &args![code_page, 1u32, string2, count2, buffer2, wide2],
                        )
                        .u32();
                    if converted != 0 {
                        result = e
                            .call(
                                COMPARE_STRING_W,
                                &args![locale, flags, buffer1, wide1, buffer2, wide2],
                            )
                            .i32();
                    }
                    e.call(FREEA, &args![buffer2]);
                    result
                });
            }
        }
        e.call(FREEA, &args![buffer1]);
        result
    })
}

/// `__crtCompareStringA_stat`'s `CompareStringA` path (modes 0 and 2). The counts the
/// conversions update are the exe's own argument slots: a stack block here.
#[allow(clippy::too_many_arguments)]
fn compare_ansi(
    e: &mut Engine,
    update: u32,
    string1: u32,
    locale: u32,
    flags: u32,
    count1: i32,
    string2: u32,
    count2: i32,
    code_page: u32,
) -> i32 {
    let locale = if locale == 0 {
        let locinfo = e.mem.u32(update);
        e.mem.u32(locinfo.wrapping_add(0x14))
    } else {
        locale
    };
    let code_page = if code_page == 0 {
        let locinfo = e.mem.u32(update);
        e.mem.u32(locinfo.wrapping_add(4))
    } else {
        code_page
    };
    let ansi = e.call(ANSICP, &args![locale]).u32();
    if ansi == u32::MAX {
        return 0;
    }
    e.with_stack(8, |e, counts| {
        let counts = counts.addr();
        e.mem.set_u32(counts, count1 as u32);
        e.mem.set_u32(counts.wrapping_add(4), count2 as u32);
        let (mut first, mut second) = (string1, string2);
        let (mut converted1, mut converted2) = (0u32, 0u32);
        if ansi != code_page {
            converted1 = e
                .call(
                    CONVERTCP,
                    &args![code_page, ansi, string1, counts, 0u32, 0u32],
                )
                .u32();
            if converted1 == 0 {
                return 0;
            }
            converted2 = e
                .call(
                    CONVERTCP,
                    &args![code_page, ansi, string2, counts.wrapping_add(4), 0u32, 0u32],
                )
                .u32();
            if converted2 == 0 {
                e.call(FREE, &args![converted1]);
                return 0;
            }
            first = converted1;
            second = converted2;
        }
        let count1 = e.mem.u32(counts);
        let count2 = e.mem.u32(counts.wrapping_add(4));
        let result = e
            .call(
                COMPARE_STRING_A_IMPORT,
                &args![locale, flags, first, count1, second, count2],
            )
            .i32();
        if converted1 != 0 {
            e.call(FREE, &args![converted1]);
            e.call(FREE, &args![converted2]);
        }
        result
    })
}

// Translated from 00ee67e8 (decompiled, FalloutNV.exe 1.4.0.525)
/// `___crtCompareStringA` (Ghidra library match; cdecl, eight words): with a `_LocaleUpdate`
/// for `locale_info` on the stack, answers `__crtCompareStringA_stat(&update, string1,
/// locale, flags, count1, string2, count2, code_page)`, then ends the update (clears bit 1 of
/// the thread data's word +0x70 when it changed the locale).
#[allow(clippy::too_many_arguments)]
pub fn crt_compare_string_a(
    e: &mut Engine,
    locale_info: u32,
    locale: u32,
    flags: u32,
    string1: Ptr,
    count1: i32,
    string2: Ptr,
    count2: i32,
    code_page: u32,
) -> i32 {
    e.with_stack(LOCALE_UPDATE_SIZE, |e, update| {
        e.call(LOCALE_UPDATE, &args![update, locale_info]);
        let result = crt_compare_string_a_stat(
            e, update, string1, locale, flags, count1, string2, count2, code_page,
        );
        end_locale_update(e, update.addr());
        result
    })
}

// Translated from 00ee6924 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_findenv` (Ghidra library match; register argument `length` in `EDI`, `name` the one
/// stack word): the index in `_environ` of the first entry whose first `length` bytes match
/// `name` without case (`__mbsnbicoll`) and that has `=` or its end right after them; when
/// none does, minus the number of entries.
pub fn findenv(e: &mut Engine, length: u32, name: Ptr) -> i32 {
    let mut slot = e.mem.u32(ENVIRON);
    loop {
        let entry = e.mem.u32(slot);
        if entry == 0 {
            let start = e.mem.u32(ENVIRON);
            return ((slot.wrapping_sub(start) as i32) >> 2).wrapping_neg();
        }
        if mbsnbicoll(e, name, Ptr::new(entry), length) == 0 {
            let next = e.mem.u8(length.wrapping_add(e.mem.u32(slot)));
            if next == b'=' || next == 0 {
                let start = e.mem.u32(ENVIRON);
                return (slot.wrapping_sub(start) as i32) >> 2;
            }
        }
        slot = slot.wrapping_add(4);
    }
}

// Translated from 00ee6976 (decompiled, FalloutNV.exe 1.4.0.525)
/// `_copy_environ` (Ghidra library match; register argument `environment` in `EAX`): null
/// for a null array; otherwise a new array (`__calloc_crt(count + 1, 4)`; `_amsg_exit(9)`
/// when that fails, after which the copy goes on at address 0 as the exe's code does) of
/// `_strdup` copies of the entries, ended by a null word. Answers the new array.
pub fn copy_environ(e: &mut Engine, environment: Ptr) -> Ptr {
    let mut source = environment.addr();
    if source == 0 {
        return Ptr::NULL;
    }
    let mut count = 0u32;
    if e.mem.u32(source) != 0 {
        let mut at = source;
        loop {
            at = at.wrapping_add(4);
            count = count.wrapping_add(1);
            if e.mem.u32(at) == 0 {
                break;
            }
        }
    }
    let copy = e
        .call(CALLOC_CRT, &args![count.wrapping_add(1), 4u32])
        .u32();
    if copy == 0 {
        e.call(AMSG_EXIT, &args![9u32]);
    }
    let mut target = copy;
    loop {
        let entry = e.mem.u32(source);
        if entry == 0 {
            break;
        }
        let duplicate = strdup(e, Ptr::new(entry)).addr();
        e.mem.set_u32(target, duplicate);
        target = target.wrapping_add(4);
        source = source.wrapping_add(4);
    }
    e.mem.set_u32(target, 0);
    Ptr::new(copy)
}

// Translated from 00ee63c3 (decompiled, FalloutNV.exe 1.4.0.525)
/// `___wtomb_environ` (Ghidra library match; cdecl, no arguments): converts each entry of
/// `_wenviron` with `WideCharToMultiByte` (code page 0, sized first, into a
/// `__calloc_crt(size, 1)` block) and sets it with `__crtsetenv(&entry, 0)`; an entry the
/// call refuses is freed unless it was taken. Answers 0, or -1 when a size, block or
/// conversion fails (the block freed on a failed conversion).
pub fn wtomb_environ(e: &mut Engine) -> i32 {
    let mut wide = e.mem.u32(WIDE_ENVIRON);
    e.with_stack(4, |e, entry| {
        e.mem.set_u32(entry.addr(), 0);
        loop {
            let source = e.mem.u32(wide);
            if source == 0 {
                return 0;
            }
            let size = e
                .call(
                    WIDE_CHAR_TO_MULTI_BYTE,
                    &args![0u32, 0u32, source, u32::MAX, 0u32, 0u32, 0u32, 0u32],
                )
                .u32();
            if size == 0 {
                return -1;
            }
            let block = e.call(CALLOC_CRT, &args![size, 1u32]).u32();
            e.mem.set_u32(entry.addr(), block);
            if block == 0 {
                return -1;
            }
            let source = e.mem.u32(wide);
            let converted = e
                .call(
                    WIDE_CHAR_TO_MULTI_BYTE,
                    &args![0u32, 0u32, source, u32::MAX, block, size, 0u32, 0u32],
                )
                .u32();
            if converted == 0 {
                let block = e.mem.u32(entry.addr());
                e.call(FREE, &args![block]);
                return -1;
            }
            if crtsetenv(e, entry, 0) < 0 && e.mem.u32(entry.addr()) != 0 {
                let block = e.mem.u32(entry.addr());
                e.call(FREE, &args![block]);
                e.mem.set_u32(entry.addr(), 0);
            }
            wide = wide.wrapping_add(4);
        }
    })
}

/// `errno = EINVAL (0x16)` and -1, the common failure of `__crtsetenv`.
fn setenv_invalid(e: &mut Engine) -> i32 {
    let error = e.call(ERRNO, &args![]).u32();
    e.mem.set_u32(error, 0x16);
    -1
}

// Translated from 00ee69d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `___crtsetenv` (Ghidra library match; cdecl, two words): sets or (with nothing after the
/// `=`) removes the `name=value` string `*option`, which it takes over (`*option` cleared
/// whenever it is stored or freed). A null `option` is `_invalid_parameter` and -1; a null
/// string, one without `=` or starting with it is `errno = EINVAL` and -1. `_environ` is first
/// copied when it is still `__initenv` (`_copy_environ`); without one, a `primary` call with
/// a `_wenviron` builds it (`___wtomb_environ`), a removal answers 0, and otherwise empty
/// `_environ` and (if missing) `_wenviron` arrays are made (`__malloc_crt(4)`). A found entry
/// (`_findenv`) is freed and replaced, or removed with the array shrunk
/// (`__recalloc_crt(_environ, count, 4)` when that count is below 0x3fffffff); a new entry
/// grows the array (`__recalloc_crt(_environ, 4, count + 2)`, as the exe pushes them) and is
/// appended. A `primary` call also sets the process variable (`SetEnvironmentVariableA` on a
/// `__calloc_crt` copy, the value null for a removal; -1 and `errno = EILSEQ (0x2a)` when it
/// fails). Answers 0, or -1 when an allocation fails.
pub fn crtsetenv(e: &mut Engine, option: Ptr, primary: u32) -> i32 {
    let option_slot = option.addr();
    if option_slot == 0 {
        invalid_parameter(e);
        return -1;
    }
    let mut result = 0;
    let string = e.mem.u32(option_slot);
    if string == 0 {
        return setenv_invalid(e);
    }
    let equal = mbschr(e, Ptr::new(string), u32::from(b'=')).addr();
    if equal == 0 || string == equal {
        return setenv_invalid(e);
    }
    let remove = e.mem.u8(equal.wrapping_add(1)) == 0;
    let mut environment = e.mem.u32(ENVIRON);
    if environment == e.mem.u32(INITIAL_ENVIRON) {
        environment = copy_environ(e, Ptr::new(environment)).addr();
        e.mem.set_u32(ENVIRON, environment);
    }
    if environment == 0 {
        if primary != 0 && e.mem.u32(WIDE_ENVIRON) != 0 {
            if wtomb_environ(e) != 0 {
                return setenv_invalid(e);
            }
        } else {
            if remove {
                return 0;
            }
            let array = e.call(MALLOC_CRT, &args![4u32]).u32();
            e.mem.set_u32(ENVIRON, array);
            if array == 0 {
                return -1;
            }
            e.mem.set_u32(array, 0);
            if e.mem.u32(WIDE_ENVIRON) == 0 {
                let array = e.call(MALLOC_CRT, &args![4u32]).u32();
                e.mem.set_u32(WIDE_ENVIRON, array);
                if array == 0 {
                    return -1;
                }
                e.mem.set_u32(array, 0);
            }
        }
    }
    let environment = e.mem.u32(ENVIRON);
    if environment == 0 {
        return -1;
    }
    let mut index = findenv(e, equal.wrapping_sub(string), Ptr::new(string));
    if index >= 0 && e.mem.u32(environment) != 0 {
        let mut slot = environment.wrapping_add((index as u32).wrapping_mul(4));
        let old = e.mem.u32(slot);
        e.call(FREE, &args![old]);
        if !remove {
            e.mem.set_u32(slot, string);
            e.mem.set_u32(option_slot, 0);
        } else {
            while e.mem.u32(slot) != 0 {
                let next = e.mem.u32(slot.wrapping_add(4));
                e.mem.set_u32(slot, next);
                index = index.wrapping_add(1);
                slot = environment.wrapping_add((index as u32).wrapping_mul(4));
            }
            if (index as u32) < 0x3fff_ffff {
                let current = e.mem.u32(ENVIRON);
                let array = e.call(RECALLOC_CRT, &args![current, index, 4u32]).u32();
                if array != 0 {
                    e.mem.set_u32(ENVIRON, array);
                }
            }
        }
    } else {
        if remove {
            e.call(FREE, &args![string]);
            e.mem.set_u32(option_slot, 0);
            return 0;
        }
        if index < 0 {
            index = index.wrapping_neg();
        }
        let size = index.wrapping_add(2);
        if size < index || size as u32 >= 0x3fff_ffff {
            return -1;
        }
        let current = e.mem.u32(ENVIRON);
        let array = e.call(RECALLOC_CRT, &args![current, 4u32, size]).u32();
        if array == 0 {
            return -1;
        }
        let slot = array.wrapping_add((index as u32).wrapping_mul(4));
        e.mem.set_u32(slot, string);
        e.mem.set_u32(slot.wrapping_add(4), 0);
        e.mem.set_u32(option_slot, 0);
        e.mem.set_u32(ENVIRON, array);
    }
    if primary != 0 {
        let length = e.call(STRLEN, &args![string]).u32();
        let copy = e
            .call(CALLOC_CRT, &args![length.wrapping_add(2), 1u32])
            .u32();
        if copy != 0 {
            let length = e.call(STRLEN, &args![string]).u32();
            let failed = e
                .call(STRCPY_S, &args![copy, length.wrapping_add(2), string])
                .u32();
            if failed != 0 {
                e.call(INVOKE_WATSON, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
            }
            let split = copy.wrapping_sub(string).wrapping_add(equal);
            e.mem.set_u8(split, 0);
            let value = if remove { 0 } else { split.wrapping_add(1) };
            if e.call(SET_ENVIRONMENT_VARIABLE_A, &args![copy, value])
                .u32()
                == 0
            {
                result = -1;
                let error = e.call(ERRNO, &args![]).u32();
                e.mem.set_u32(error, 0x2a);
            }
            e.call(FREE, &args![copy]);
        }
    }
    if remove {
        e.call(FREE, &args![string]);
        e.mem.set_u32(option_slot, 0);
    }
    result
}

// Translated from 00ee6c1b (decompiled, FalloutNV.exe 1.4.0.525)
/// `__strdup` (Ghidra library match; cdecl, one word): null for a null `source`; otherwise a
/// `malloc(strlen + 1)` copy made with `strcpy_s` (`__invoke_watson(0, 0, 0, 0, 0)` when that
/// fails; the code after the call answers the copy), or null when `malloc` fails.
pub fn strdup(e: &mut Engine, source: Ptr) -> Ptr {
    if source.is_null() {
        return Ptr::NULL;
    }
    let size = e.call(STRLEN, &args![source]).u32().wrapping_add(1);
    let copy = e.call(MALLOC, &args![size]).u32();
    if copy == 0 {
        return Ptr::NULL;
    }
    if e.call(STRCPY_S, &args![copy, size, source]).u32() != 0 {
        e.call(INVOKE_WATSON, &args![0u32, 0u32, 0u32, 0u32, 0u32]);
    }
    Ptr::new(copy)
}

// Translated from 00ee6c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mbschr_l` (Ghidra library match; cdecl, three words): with a `_LocaleUpdate` for
/// `locale` on the stack, a null `string` is `errno = EINVAL`, `_invalid_parameter` and
/// null. For a code page that is not multibyte (`mbcinfo +8` is 0) it answers `strchr(string,
/// character)`. Otherwise it walks the string: a lead byte (bit 2 of `mbcinfo`'s type byte at
/// +0x1d + byte) joins the next byte into one character (null when that byte is the end),
/// and the first character equal to `character` (the terminator too, for 0) answers its
/// address; null when there is none.
pub fn mbschr_l(e: &mut Engine, string: Ptr, character: u32, locale: u32) -> Ptr {
    e.with_stack(LOCALE_UPDATE_SIZE, |e, update| {
        let update = update.addr();
        e.call(LOCALE_UPDATE, &args![update, locale]);
        let mut at = string.addr();
        if at == 0 {
            invalid_parameter(e);
            end_locale_update(e, update);
            return Ptr::NULL;
        }
        let mbcinfo = e.mem.u32(update + LOCALE_UPDATE_MBCINFO);
        if e.mem.u32(mbcinfo.wrapping_add(8)) == 0 {
            let found = e.call(STRCHR, &args![at, character]).u32();
            end_locale_update(e, update);
            return Ptr::new(found);
        }
        loop {
            let byte = u32::from(e.mem.u8(at));
            if byte == 0 {
                if character == byte {
                    end_locale_update(e, update);
                    return Ptr::new(at);
                }
                end_locale_update(e, update);
                return Ptr::NULL;
            }
            let kind = e.mem.u8(byte.wrapping_add(mbcinfo).wrapping_add(0x1d));
            if kind & 4 != 0 {
                at = at.wrapping_add(1);
                let trail = u32::from(e.mem.u8(at));
                if trail == 0 {
                    end_locale_update(e, update);
                    return Ptr::NULL;
                }
                if character == (byte << 8 | trail) {
                    end_locale_update(e, update);
                    return Ptr::new(at.wrapping_sub(1));
                }
            } else if character == byte {
                end_locale_update(e, update);
                return Ptr::new(at);
            }
            at = at.wrapping_add(1);
        }
    })
}

// Translated from 00ee6d2c (decompiled, FalloutNV.exe 1.4.0.525)
/// `__mbschr` (Ghidra library match; cdecl, two words): `__mbschr_l(string, character,
/// NULL)`.
pub fn mbschr(e: &mut Engine, string: Ptr, character: u32) -> Ptr {
    mbschr_l(e, string, character, 0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ee_645a, strncnt(Ptr, i32) -> i32),
        entry!(0x00ee_6478, crt_compare_string_a_stat(Ptr, Ptr, u32, u32, i32, Ptr, i32, u32) -> i32),
        entry!(0x00ee_67e8, crt_compare_string_a(u32, u32, u32, Ptr, i32, Ptr, i32, u32) -> i32),
        entry!(0x00ee_6924, findenv(u32, Ptr) -> i32),
        entry!(0x00ee_6976, copy_environ(Ptr) -> Ptr),
        entry!(0x00ee_63c3, wtomb_environ() -> i32),
        entry!(0x00ee_69d0, crtsetenv(Ptr, u32) -> i32),
        entry!(0x00ee_6c1b, strdup(Ptr) -> Ptr),
        entry!(0x00ee_6c70, mbschr_l(Ptr, u32, u32) -> Ptr),
        entry!(0x00ee_6d2c, mbschr(Ptr, u32) -> Ptr),
        entry!(0x00ee_6236, get_fmode(Ptr) -> u32),
        entry!(0x00ee_628e, fn_00ee628e()),
        entry!(0x00ee_62bb, mbsnbicoll_l(Ptr, Ptr, u32, u32) -> i32),
        entry!(0x00ee_63a9, mbsnbicoll(Ptr, Ptr, u32) -> i32),
        entry!(0x00ee_682a, strnicoll_l(Ptr, Ptr, u32, u32) -> i32),
        entry!(0x00ee_626f, fn_00ee626f()),
        entry!(0x00ee_6d44, fn_00ee6d44(u32) -> i32),
        entry!(0x00ee_6d4a, fn_00ee6d4a(u32, u32, u32) -> u32),
        entry!(0x00ee_6d50, fn_00ee6d50() -> i32),
        entry!(0x00ee_6d56, fn_00ee6d56(u32, Ptr, u32, u32) -> i32),
        entry!(0x00ee_6d5c, fn_00ee6d5c(u32, Ptr, u32, u32) -> i32),
        entry!(0x00ee_6d62, fn_00ee6d62(u32, Ptr, u32) -> i32),
        entry!(0x00ee_6d68, fn_00ee6d68(Ptr) -> Ptr),
        entry!(0x00ee_6d6e, fn_00ee6d6e(Ptr) -> u32),
        entry!(0x00ee_6d74, fn_00ee6d74(u16) -> u16),
        entry!(0x00ee_6d7a, fn_00ee6d7a(u32, u32, u32, u32) -> i32),
        entry!(0x00ee_6d86, fn_00ee6d86(u16, Ptr) -> i32),
        entry!(0x00ee_6d8c, fn_00ee6d8c(Ptr, u32) -> i32),
        entry!(0x00ee_6d92, fn_00ee6d92(u32, u32) -> i32),
        entry!(0x00ee_6d98, fn_00ee6d98(u32, Ptr, u32) -> i32),
        entry!(0x00ee_6d9e, fn_00ee6d9e(u32, u32, u32, Ptr, u32) -> i32),
        entry!(0x00ee_6da4, fn_00ee6da4(u32) -> Ptr),
        entry!(0x00ee_6daa, fn_00ee6daa(u16) -> u16),
        entry!(0x00ee_6db0, fn_00ee6db0(u32, Ptr, Ptr) -> u32),
        entry!(0x00ee_6db6, fn_00ee6db6(u32, Ptr) -> i32),
        entry!(0x00ee_6dbc, fn_00ee6dbc(u32, Ptr, Ptr, Ptr, Ptr) -> i32),
        entry!(0x00ee_6dc2, fn_00ee6dc2(Ptr, u32, u32, u32) -> u32),
        entry!(0x00ee_6dc8, fn_00ee6dc8(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00ee_6dce, fn_00ee6dce(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Calls `thunk` with `words`, with a double over the import `slot` that
    /// returns `0xdeadbeef`; checks that the double ran once with exactly
    /// those words and that the thunk returned `expected`.
    fn import(thunk: u32, slot: u32, words: &[u32], expected: u32) {
        let mut e = Engine::new();
        e.register(slot, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, words);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == slot).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, words);
        assert_eq!(result.u32(), expected);
    }

    #[test]
    fn thunk_00ee6d44() {
        import(0x00ee_6d44, IMPORT_SLOT_00EE6D44, &[0x1001], 0xdead_beef);
    }

    #[test]
    fn thunk_00ee6d4a() {
        import(
            0x00ee_6d4a,
            IMPORT_SLOT_00EE6D4A,
            &[0x1001, 0x1002, 0x1003],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d50() {
        import(0x00ee_6d50, IMPORT_SLOT_00EE6D50, &[], 0xdead_beef);
    }

    #[test]
    fn thunk_00ee6d56() {
        import(
            0x00ee_6d56,
            IMPORT_SLOT_00EE6D56,
            &[0x1001, 0x1002, 0x1003, 0x1004],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d5c() {
        import(
            0x00ee_6d5c,
            IMPORT_SLOT_00EE6D5C,
            &[0x1001, 0x1002, 0x1003, 0x1004],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d62() {
        import(
            0x00ee_6d62,
            IMPORT_SLOT_00EE6D62,
            &[0x1001, 0x1002, 0x1003],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d68() {
        import(0x00ee_6d68, IMPORT_SLOT_00EE6D68, &[0x1001], 0xdead_beef);
    }

    #[test]
    fn thunk_00ee6d6e() {
        import(0x00ee_6d6e, IMPORT_SLOT_00EE6D6E, &[0x1001], 0xdead_beef);
    }

    #[test]
    fn thunk_00ee6d74() {
        import(0x00ee_6d74, IMPORT_SLOT_00EE6D74, &[0x1001], 0xbeef);
    }

    #[test]
    fn thunk_00ee6d7a() {
        import(
            0x00ee_6d7a,
            IMPORT_SLOT_00EE6D7A,
            &[0x1001, 0x1002, 0x1003, 0x1004],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d86() {
        import(
            0x00ee_6d86,
            IMPORT_SLOT_00EE6D86,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d8c() {
        import(
            0x00ee_6d8c,
            IMPORT_SLOT_00EE6D8C,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d92() {
        import(
            0x00ee_6d92,
            IMPORT_SLOT_00EE6D92,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d98() {
        import(
            0x00ee_6d98,
            IMPORT_SLOT_00EE6D98,
            &[0x1001, 0x1002, 0x1003],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6d9e() {
        import(
            0x00ee_6d9e,
            IMPORT_SLOT_00EE6D9E,
            &[0x1001, 0x1002, 0x1003, 0x1004, 0x1005],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6da4() {
        import(0x00ee_6da4, IMPORT_SLOT_00EE6DA4, &[0x1001], 0xdead_beef);
    }

    #[test]
    fn thunk_00ee6daa() {
        import(0x00ee_6daa, IMPORT_SLOT_00EE6DAA, &[0x1001], 0xbeef);
    }

    #[test]
    fn thunk_00ee6db0() {
        import(
            0x00ee_6db0,
            IMPORT_SLOT_00EE6DB0,
            &[0x1001, 0x1002, 0x1003],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6db6() {
        import(
            0x00ee_6db6,
            IMPORT_SLOT_00EE6DB6,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6dbc() {
        import(
            0x00ee_6dbc,
            IMPORT_SLOT_00EE6DBC,
            &[0x1001, 0x1002, 0x1003, 0x1004, 0x1005],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6dc2() {
        import(
            0x00ee_6dc2,
            IMPORT_SLOT_00EE6DC2,
            &[0x1001, 0x1002, 0x1003, 0x1004],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6dc8() {
        import(
            0x00ee_6dc8,
            IMPORT_SLOT_00EE6DC8,
            &[0x1001, 0x1002, 0x1003],
            0xdead_beef,
        );
    }

    #[test]
    fn thunk_00ee6dce() {
        import(
            0x00ee_6dce,
            IMPORT_SLOT_00EE6DCE,
            &[0x1001, 0x1002],
            0xdead_beef,
        );
    }
}

#[cfg(test)]
mod physics_step_tests {
    use super::*;

    #[test]
    fn initconout_opens_conout_and_keeps_the_handle() {
        let mut e = Engine::new();
        e.map(CONSOLE_OUTPUT_HANDLE, 4);
        e.register(CREATE_FILE_A, |_, _| Ret {
            eax: 0x1234,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00ee_626f, &args![]);
        let log = e.call_log.take().unwrap();
        let opens: Vec<_> = log.iter().filter(|c| c.0 == CREATE_FILE_A).collect();
        assert_eq!(opens.len(), 1);
        assert_eq!(
            opens[0].1,
            vec![0x0110_08b4, 0x4000_0000, 3, 0, 3, 0, 0],
            "name, GENERIC_WRITE, share read|write, no security, OPEN_EXISTING, no flags, no template"
        );
        assert_eq!(e.global::<u32>(CONSOLE_OUTPUT_HANDLE), 0x1234);
    }

    #[test]
    fn initconout_keeps_an_invalid_handle_too() {
        let mut e = Engine::new();
        e.map(CONSOLE_OUTPUT_HANDLE, 4);
        e.set_global(CONSOLE_OUTPUT_HANDLE, 0xfffe_fffeu32);
        e.register(CREATE_FILE_A, |_, _| Ret {
            eax: 0xffff_ffff,
            ..Ret::default()
        });
        e.call(0x00ee_626f, &args![]);
        assert_eq!(e.global::<u32>(CONSOLE_OUTPUT_HANDLE), 0xffff_ffff);
    }
}

#[cfg(test)]
mod b006_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn termconout_closes_only_real_handles() {
        let mut e = Engine::new();
        e.map(0x011c_2000, 0x1000);
        e.register(CLOSE_HANDLE, |_, _| 1u32.into_ret());
        e.set_global(CONSOLE_OUTPUT_HANDLE, 0x44u32);
        e.set_global(CONSOLE_SECOND_HANDLE, 0x48u32);
        e.call_log = Some(vec![]);
        e.call(0x00ee_628e, &args![]);
        assert_eq!(calls_to(&e, CLOSE_HANDLE), vec![vec![0x44], vec![0x48]]);
        assert_eq!(e.global::<u32>(CONSOLE_OUTPUT_HANDLE), 0x44);
        for (output, second) in [(u32::MAX, 0xffff_fffeu32), (0xffff_fffe, u32::MAX)] {
            e.set_global(CONSOLE_OUTPUT_HANDLE, output);
            e.set_global(CONSOLE_SECOND_HANDLE, second);
            e.call_log = Some(vec![]);
            e.call(0x00ee_628e, &args![]);
            assert!(calls_to(&e, CLOSE_HANDLE).is_empty());
        }
    }

    /// Doubles for the CRT callees; `errno` lives at a fresh word.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        let error = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| error.into_ret());
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        (e, error)
    }

    #[test]
    fn get_fmode_stores_the_mode_or_rejects_null() {
        let (mut e, error) = engine();
        e.map(0x0127_0000, 0x1000);
        e.set_global(FMODE, 0x8000u32);
        let mode = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_6236, &args![mode]).u32(), 0);
        assert_eq!(e.mem.u32(mode), 0x8000);
        assert!(calls_to(&e, INVALID_PARAMETER).is_empty());
        assert_eq!(e.call(0x00ee_6236, &args![0u32]).u32(), 0x16);
        assert_eq!(e.mem.u32(error), 0x16);
        assert_eq!(calls_to(&e, INVALID_PARAMETER), vec![vec![0; 5]]);
    }

    /// A `_LocaleUpdate` double that fills the local with `locinfo`,
    /// `mbcinfo`, a thread data block and the updated byte.
    fn locale(e: &mut Engine, locinfo: u32, mbcinfo: u32, updated: bool) -> u32 {
        let thread_data = e.mem.alloc(0x80);
        e.mem.set_u32(thread_data + 0x70, 0xff);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            e.mem.set_u32(a[0], locinfo);
            e.mem.set_u32(a[0] + 4, mbcinfo);
            e.mem.set_u32(a[0] + 8, thread_data);
            e.mem.set_u8(a[0] + 0xc, updated as u8);
            Ret::default()
        });
        thread_data
    }

    #[test]
    fn strnicoll_l_checks_its_arguments() {
        let (mut e, error) = engine();
        let locinfo = e.mem.alloc(0x20);
        let thread_data = locale(&mut e, locinfo, 0, true);
        let (a, b) = (e.mem.alloc(4), e.mem.alloc(4));
        assert_eq!(e.call(0x00ee_682a, &args![a, b, 0u32, 0u32]).i32(), 0);
        // The updated byte makes every path clear bit 1 of the thread flags.
        assert_eq!(e.mem.u32(thread_data + 0x70), 0xfd);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ee_682a, &args![0u32, b, 3u32, 0u32]).i32(),
            0x7fff_ffff
        );
        assert_eq!(
            e.call(0x00ee_682a, &args![a, 0u32, 3u32, 0u32]).i32(),
            0x7fff_ffff
        );
        assert_eq!(
            e.call(0x00ee_682a, &args![a, b, 0x8000_0000u32, 0u32])
                .i32(),
            0x7fff_ffff
        );
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 3);
        assert_eq!(e.mem.u32(error), 0x16);
    }

    #[test]
    fn strnicoll_l_without_a_collate_name_uses_strnicmp() {
        let (mut e, _) = engine();
        let locinfo = e.mem.alloc(0x20);
        locale(&mut e, locinfo, 0, false);
        e.register(STRNICMP_L, |_, _| (-5i32).into_ret());
        let (a, b) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_682a, &args![a, b, 3u32, 9u32]).i32(), -5);
        let calls = calls_to(&e, STRNICMP_L);
        assert_eq!(calls.len(), 1);
        assert_eq!(&calls[0][..3], &[a, b, 3]);
        // The locale passed on is the `_LocaleUpdate` local, not `locale`.
        assert_eq!(calls_to(&e, LOCALE_UPDATE)[0][0], calls[0][3]);
    }

    #[test]
    fn strnicoll_l_with_a_collate_name_compares_strings() {
        let (mut e, error) = engine();
        let locinfo = e.mem.alloc(0x20);
        e.mem.set_u32(locinfo + 8, 1252);
        e.mem.set_u32(locinfo + 0x10, 0x4321);
        locale(&mut e, locinfo, 0, false);
        let answer = std::rc::Rc::new(std::cell::Cell::new(3i32));
        let given = answer.clone();
        e.register_double(CRT_COMPARE_STRING_A, move |_, _| given.get().into_ret());
        let (a, b) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_682a, &args![a, b, 5u32, 0u32]).i32(), 1);
        let calls = calls_to(&e, CRT_COMPARE_STRING_A);
        assert_eq!(&calls[0][1..], &[0x4321, 0x1001, a, 5, b, 5, 1252]);
        answer.set(0);
        assert_eq!(
            e.call(0x00ee_682a, &args![a, b, 5u32, 0u32]).i32(),
            0x7fff_ffff
        );
        assert_eq!(e.mem.u32(error), 0x16);
        assert!(calls_to(&e, INVALID_PARAMETER).is_empty());
    }

    #[test]
    fn mbsnbicoll_l_single_byte_code_page_goes_through_strnicoll_l() {
        let (mut e, _) = engine();
        let locinfo = e.mem.alloc(0x20);
        let mbcinfo = e.mem.alloc(0x20);
        locale(&mut e, locinfo, mbcinfo, false);
        e.register(STRNICMP_L, |_, _| 7i32.into_ret());
        let (a, b) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_62bb, &args![a, b, 3u32, 0u32]).i32(), 7);
        // `_mbsnbicoll` passes a null locale.
        assert_eq!(e.call(0x00ee_63a9, &args![a, b, 3u32]).i32(), 7);
        assert!(calls_to(&e, LOCALE_UPDATE).iter().all(|c| c[1] == 0));
        assert_eq!(e.call(0x00ee_62bb, &args![a, b, 0u32, 0u32]).i32(), 0);
        assert_eq!(
            e.call(0x00ee_62bb, &args![a, 0u32, 1u32, 0u32]).i32(),
            0x7fff_ffff
        );
        assert_eq!(
            e.call(0x00ee_62bb, &args![a, b, 0x8000_0000u32, 0u32])
                .i32(),
            0x7fff_ffff
        );
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 2);
    }

    #[test]
    fn mbsnbicoll_l_multibyte_code_page_compares_strings() {
        let (mut e, error) = engine();
        let locinfo = e.mem.alloc(0x20);
        let mbcinfo = e.mem.alloc(0x20);
        e.mem.set_u32(mbcinfo + 4, 932);
        e.mem.set_u32(mbcinfo + 8, 1);
        e.mem.set_u32(mbcinfo + 0xc, 0x411);
        locale(&mut e, locinfo, mbcinfo, false);
        let answer = std::rc::Rc::new(std::cell::Cell::new(1i32));
        let given = answer.clone();
        e.register_double(CRT_COMPARE_STRING_A, move |_, _| given.get().into_ret());
        let (a, b) = (e.mem.alloc(4), e.mem.alloc(4));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_62bb, &args![a, b, 2u32, 0u32]).i32(), -1);
        let calls = calls_to(&e, CRT_COMPARE_STRING_A);
        assert_eq!(&calls[0][1..], &[0x411, 0x1001, a, 2, b, 2, 932]);
        answer.set(0);
        assert_eq!(
            e.call(0x00ee_62bb, &args![a, b, 2u32, 0u32]).i32(),
            0x7fff_ffff
        );
        // No `errno` on this failure.
        assert_eq!(e.mem.u32(error), 0);
    }
}

#[cfg(test)]
mod n082_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type CallLog = Rc<RefCell<Vec<Vec<u32>>>>;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn put_string(e: &mut Engine, text: &[u8]) -> u32 {
        let at = e.mem.alloc(text.len() as u32 + 1);
        for (i, byte) in text.iter().enumerate() {
            e.mem.set_u8(at + i as u32, *byte);
        }
        e.mem.set_u8(at + text.len() as u32, 0);
        at
    }

    fn read_string(e: &Engine, at: u32) -> Vec<u8> {
        let mut out = vec![];
        let mut p = at;
        while e.mem.u8(p) != 0 {
            out.push(e.mem.u8(p));
            p += 1;
        }
        out
    }

    /// The CRT doubles: `errno` at a fresh word, `_invalid_parameter`, a `_LocaleUpdate` that
    /// fills the local with `locinfo` and `mbcinfo` (updated byte set, thread flags 0xff), and
    /// working `strlen`, `strcpy_s`, `strchr`, `malloc`, `__calloc_crt`, `__malloc_crt` and a
    /// case-blind `__strnicmp_l`; `free` only does nothing. Answers the engine, `errno`'s word
    /// and the thread data block.
    fn crt(locinfo: u32, mbcinfo: u32) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let error = e.mem.alloc(4);
        e.register_double(ERRNO, move |_, _| error.into_ret());
        e.register(INVALID_PARAMETER, |_, _| Ret::default());
        let thread_data = e.mem.alloc(0x80);
        e.mem.set_u32(thread_data + 0x70, 0xff);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            let locinfo = if locinfo == 0 {
                e.mem.alloc(0x20)
            } else {
                locinfo
            };
            let mbcinfo = if mbcinfo == 0 {
                e.mem.alloc(0x120)
            } else {
                mbcinfo
            };
            e.mem.set_u32(a[0], locinfo);
            e.mem.set_u32(a[0] + 4, mbcinfo);
            e.mem.set_u32(a[0] + 8, thread_data);
            e.mem.set_u8(a[0] + 0xc, 1);
            Ret::default()
        });
        e.register(STRLEN, |e, a| {
            let mut n = 0;
            while e.mem.u8(a[0] + n) != 0 {
                n += 1;
            }
            n.into_ret()
        });
        e.register(STRCPY_S, |e, a| {
            let mut i = 0;
            loop {
                let byte = e.mem.u8(a[2] + i);
                e.mem.set_u8(a[0] + i, byte);
                if byte == 0 {
                    break;
                }
                i += 1;
            }
            0u32.into_ret()
        });
        e.register(STRCHR, |e, a| {
            let mut p = a[0];
            loop {
                let byte = u32::from(e.mem.u8(p));
                if byte == a[1] & 0xff {
                    return p.into_ret();
                }
                if byte == 0 {
                    return 0u32.into_ret();
                }
                p += 1;
            }
        });
        e.register(MALLOC, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(MALLOC_CRT, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CALLOC_CRT, |e, a| e.mem.alloc(a[0] * a[1]).into_ret());
        e.register(FREE, |_, _| Ret::default());
        e.register(STRNICMP_L, |e, a| {
            for i in 0..a[2] {
                let x = e.mem.u8(a[0] + i).to_ascii_lowercase();
                let y = e.mem.u8(a[1] + i).to_ascii_lowercase();
                if x != y {
                    return (i32::from(x) - i32::from(y)).into_ret();
                }
                if x == 0 {
                    break;
                }
            }
            0i32.into_ret()
        });
        (e, error, thread_data)
    }

    #[test]
    fn strncnt_counts_up_to_the_end_or_the_limit() {
        let mut e = Engine::new();
        let text = put_string(&mut e, b"ab");
        assert_eq!(e.call(0x00ee_645a, &args![text, 3i32]).i32(), 2);
        assert_eq!(e.call(0x00ee_645a, &args![text, 1i32]).i32(), 1);
        assert_eq!(e.call(0x00ee_645a, &args![text, 0i32]).i32(), 0);
        let long = put_string(&mut e, b"abcd");
        assert_eq!(e.call(0x00ee_645a, &args![long, 3i32]).i32(), 3);
    }

    /// The engine for `__crtCompareStringA_stat`: the CRT doubles, the mode global mapped and
    /// set to `mode`, and an update local whose locale info has code page 1252 (+4) and LCID
    /// 0x409 (+0x14).
    fn compare_engine(mode: u32) -> (Engine, u32) {
        let (mut e, _, _) = crt(0, 0);
        e.map(COMPARE_STRING_MODE & !0xfff, 0x1000);
        e.mem.set_u32(COMPARE_STRING_MODE, mode);
        let locinfo = e.mem.alloc(0x20);
        e.mem.set_u32(locinfo + 4, 1252);
        e.mem.set_u32(locinfo + 0x14, 0x409);
        let update = e.mem.alloc(0x10);
        e.mem.set_u32(update, locinfo);
        e.register(FREEA, |_, _| Ret::default());
        (e, update)
    }

    #[test]
    fn crt_compare_string_a_stat_decides_its_api_once() {
        let (mut e, update) = compare_engine(0);
        e.register(COMPARE_STRING_W, |_, _| 1u32.into_ret());
        e.register(MULTI_BYTE_TO_WIDE_CHAR, |_, _| 0u32.into_ret());
        let a = put_string(&mut e, b"x");
        e.call_log = Some(vec![]);
        e.call(
            0x00ee_6478,
            &args![update, a, 0u32, 0u32, 1i32, a, 1i32, 0u32],
        );
        assert_eq!(e.mem.u32(COMPARE_STRING_MODE), 1);
        assert_eq!(
            calls_to(&e, COMPARE_STRING_W),
            vec![vec![0, 0, COMPARE_STRING_PROBE, 1, COMPARE_STRING_PROBE, 1]]
        );
        // Not implemented: mode 2; any other error leaves it undecided.
        let (mut e, update) = compare_engine(0);
        e.register(COMPARE_STRING_W, |_, _| 0u32.into_ret());
        e.register(GET_LAST_ERROR, |_, _| 0x78u32.into_ret());
        e.register(ANSICP, |_, _| u32::MAX.into_ret());
        e.call(
            0x00ee_6478,
            &args![update, a, 0u32, 0u32, 1i32, a, 1i32, 0u32],
        );
        assert_eq!(e.mem.u32(COMPARE_STRING_MODE), 2);
        let (mut e, update) = compare_engine(0);
        e.register(COMPARE_STRING_W, |_, _| 0u32.into_ret());
        e.register(GET_LAST_ERROR, |_, _| 5u32.into_ret());
        e.register(ANSICP, |_, _| u32::MAX.into_ret());
        e.call(
            0x00ee_6478,
            &args![update, a, 0u32, 0u32, 1i32, a, 1i32, 0u32],
        );
        assert_eq!(e.mem.u32(COMPARE_STRING_MODE), 0);
        // An unknown mode answers 0; a count below -1 too.
        let (mut e, update) = compare_engine(3);
        assert_eq!(
            e.call(
                0x00ee_6478,
                &args![update, a, 0u32, 0u32, 1i32, a, 1i32, 0u32]
            )
            .i32(),
            0
        );
        assert_eq!(
            e.call(
                0x00ee_6478,
                &args![update, a, 0u32, 0u32, -2i32, a, 1i32, 0u32]
            )
            .i32(),
            0
        );
        assert_eq!(
            e.call(
                0x00ee_6478,
                &args![update, a, 0u32, 0u32, 1i32, a, -2i32, 0u32]
            )
            .i32(),
            0
        );
    }

    #[test]
    fn crt_compare_string_a_stat_answers_empty_sides_from_the_counts_and_lead_bytes() {
        let (mut e, update) = compare_engine(1);
        let lead = Rc::new(RefCell::new((2u32, 0x81u8, 0x9fu8)));
        let given = lead.clone();
        e.register_double(GET_CP_INFO, move |e, a| {
            let (size, low, high) = *given.borrow();
            if size == 0 {
                return 0u32.into_ret();
            }
            e.mem.set_u32(a[1], size);
            e.mem.set_u8(a[1] + 6, low);
            e.mem.set_u8(a[1] + 7, high);
            e.mem.set_u8(a[1] + 8, 0);
            e.mem.set_u8(a[1] + 9, 0);
            1u32.into_ret()
        });
        let empty = put_string(&mut e, b"");
        let lead_char = put_string(&mut e, &[0x85]);
        let plain = put_string(&mut e, b"a");
        let run = |e: &mut Engine, s1: u32, c1: i32, s2: u32, c2: i32| {
            e.call(
                0x00ee_6478,
                &args![update, s1, 0u32, 0u32, c1, s2, c2, 0u32],
            )
            .i32()
        };
        assert_eq!(run(&mut e, empty, 0, empty, 0), 2);
        let two = put_string(&mut e, b"ab");
        assert_eq!(run(&mut e, empty, 0, two, 2), 1);
        assert_eq!(run(&mut e, two, 2, empty, 0), 3);
        e.call_log = Some(vec![]);
        assert_eq!(run(&mut e, lead_char, 1, empty, 0), 2);
        // The zero code page is the locale's.
        assert_eq!(calls_to(&e, GET_CP_INFO)[0][0], 1252);
        assert_eq!(run(&mut e, plain, 1, empty, 0), 3);
        assert_eq!(run(&mut e, empty, 0, lead_char, 1), 2);
        assert_eq!(run(&mut e, empty, 0, plain, 1), 1);
        *lead.borrow_mut() = (1, 0x81, 0x9f);
        assert_eq!(run(&mut e, lead_char, 1, empty, 0), 3);
        assert_eq!(run(&mut e, empty, 0, lead_char, 1), 1);
        *lead.borrow_mut() = (0, 0, 0);
        assert_eq!(run(&mut e, plain, 1, empty, 0), 0);
    }

    #[test]
    fn crt_compare_string_a_stat_widens_both_strings_for_compare_string_w() {
        let (mut e, update) = compare_engine(1);
        // Sizes: 3 wide characters for each (stack buffers), or 0x300 (heap buffers).
        let size = Rc::new(RefCell::new(3u32));
        let given = size.clone();
        e.register_double(MULTI_BYTE_TO_WIDE_CHAR, move |_, a| {
            if a[1] == 9 {
                (*given.borrow()).into_ret()
            } else {
                a[5].into_ret()
            }
        });
        e.register(COMPARE_STRING_W, |_, _| 3u32.into_ret());
        let a = put_string(&mut e, b"abc");
        let b = put_string(&mut e, b"abd");
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(
                0x00ee_6478,
                &args![update, a, 0x411u32, 0x1001u32, 5i32, b, -1i32, 932u32]
            )
            .i32(),
            3
        );
        let widen = calls_to(&e, MULTI_BYTE_TO_WIDE_CHAR);
        assert_eq!(widen.len(), 4);
        assert_eq!(widen[0], vec![932, 9, a, 3, 0, 0]);
        assert_eq!(widen[2], vec![932, 9, b, u32::MAX, 0, 0]);
        let compare = calls_to(&e, COMPARE_STRING_W);
        assert_eq!(compare.len(), 1);
        assert_eq!(&compare[0][..2], &[0x411, 0x1001]);
        assert_eq!(compare[0][3], 3);
        let (first, second) = (compare[0][2], compare[0][4]);
        assert_eq!(widen[1], vec![932, 1, a, 3, first, 3]);
        assert_eq!(widen[3], vec![932, 1, b, u32::MAX, second, 3]);
        assert_eq!(calls_to(&e, FREEA), vec![vec![second], vec![first]]);
        // Large buffers come from malloc, marked 0xdddd.
        *size.borrow_mut() = 0x300;
        e.call_log = Some(vec![]);
        e.call(
            0x00ee_6478,
            &args![update, a, 0u32, 0u32, 3i32, b, 3i32, 0u32],
        );
        let mallocs = calls_to(&e, MALLOC);
        assert_eq!(mallocs, vec![vec![0x608], vec![0x608]]);
        let compare = calls_to(&e, COMPARE_STRING_W);
        assert_eq!(e.mem.u32(compare[0][2] - 8), 0xdddd);
        assert_eq!(e.mem.u32(compare[0][4] - 8), 0xdddd);
        // A failed conversion of the second string frees only the first buffer and answers 0.
        *size.borrow_mut() = 3;
        e.register_double(MULTI_BYTE_TO_WIDE_CHAR, move |_, a| {
            if a[2] == b && a[1] == 1 {
                0u32.into_ret()
            } else if a[1] == 9 {
                3u32.into_ret()
            } else {
                a[5].into_ret()
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(
                0x00ee_6478,
                &args![update, a, 0u32, 0u32, 3i32, b, 3i32, 0u32]
            )
            .i32(),
            0
        );
        assert!(calls_to(&e, COMPARE_STRING_W).is_empty());
        assert_eq!(calls_to(&e, FREEA).len(), 2);
        // A zero size answers 0 at once.
        e.register(MULTI_BYTE_TO_WIDE_CHAR, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(
                0x00ee_6478,
                &args![update, a, 0u32, 0u32, 3i32, b, 3i32, 0u32]
            )
            .i32(),
            0
        );
        assert!(calls_to(&e, FREEA).is_empty());
    }

    #[test]
    fn crt_compare_string_a_stat_converts_to_the_ansi_code_page_for_compare_string_a() {
        let (mut e, update) = compare_engine(2);
        let ansi = Rc::new(RefCell::new(1252u32));
        let given = ansi.clone();
        e.register_double(ANSICP, move |_, _| (*given.borrow()).into_ret());
        let blocks: CallLog = Rc::new(RefCell::new(vec![]));
        let seen = blocks.clone();
        e.register_double(CONVERTCP, move |e, a| {
            // Converted strings are 2 long.
            e.mem.set_u32(a[3], 2);
            let block = e.mem.alloc(4);
            seen.borrow_mut().push(vec![block]);
            block.into_ret()
        });
        e.register(COMPARE_STRING_A_IMPORT, |_, _| 1u32.into_ret());
        let a = put_string(&mut e, b"abc");
        let b = put_string(&mut e, b"xy");
        // Same code page: no conversion; LCID and code page from the locale.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(
                0x00ee_6478,
                &args![update, a, 0u32, 7u32, 5i32, b, 2i32, 0u32]
            )
            .i32(),
            1
        );
        assert_eq!(calls_to(&e, ANSICP), vec![vec![0x409]]);
        assert_eq!(
            calls_to(&e, COMPARE_STRING_A_IMPORT),
            vec![vec![0x409, 7, a, 3, b, 2]]
        );
        assert!(calls_to(&e, FREE).is_empty());
        // Another code page: both converted, compared, freed.
        e.call_log = Some(vec![]);
        e.call(
            0x00ee_6478,
            &args![update, a, 0x411u32, 7u32, 3i32, b, 2i32, 932u32],
        );
        let converted: Vec<u32> = blocks.borrow().iter().map(|b| b[0]).collect();
        let convert = calls_to(&e, CONVERTCP);
        assert_eq!(&convert[0][..3], &[932, 1252, a]);
        assert_eq!(&convert[0][4..], &[0, 0]);
        assert_eq!(&convert[1][..3], &[932, 1252, b]);
        assert_eq!(
            calls_to(&e, COMPARE_STRING_A_IMPORT),
            vec![vec![0x411, 7, converted[0], 2, converted[1], 2]]
        );
        assert_eq!(
            calls_to(&e, FREE),
            vec![vec![converted[0]], vec![converted[1]]]
        );
        // The second conversion failing frees the first and answers 0.
        e.register(CONVERTCP, |e, a| {
            if a[2] == 0 {
                0u32.into_ret()
            } else {
                e.mem.alloc(4).into_ret()
            }
        });
        let null = 0u32;
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(
                0x00ee_6478,
                &args![update, a, 0x411u32, 7u32, 3i32, null, -1i32, 932u32]
            )
            .i32(),
            0
        );
        assert_eq!(calls_to(&e, FREE).len(), 1);
        assert!(calls_to(&e, COMPARE_STRING_A_IMPORT).is_empty());
        // An unknown LCID answers 0.
        *ansi.borrow_mut() = u32::MAX;
        assert_eq!(
            e.call(
                0x00ee_6478,
                &args![update, a, 0x411u32, 7u32, 3i32, b, 2i32, 932u32]
            )
            .i32(),
            0
        );
    }

    #[test]
    fn crt_compare_string_a_passes_the_update_on_and_ends_it() {
        let (mut e, _, thread_data) = crt(0, 0);
        e.map(COMPARE_STRING_MODE & !0xfff, 0x1000);
        e.mem.set_u32(COMPARE_STRING_MODE, 2);
        e.register(ANSICP, |_, a| a[0].into_ret());
        e.register(COMPARE_STRING_A_IMPORT, |_, _| 2u32.into_ret());
        let a = put_string(&mut e, b"ab");
        let b = put_string(&mut e, b"ac");
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(
                0x00ee_67e8,
                &args![0x55u32, 0x409u32, 0x1001u32, a, 2i32, b, 2i32, 0x409u32]
            )
            .i32(),
            2
        );
        assert_eq!(calls_to(&e, LOCALE_UPDATE)[0][1], 0x55);
        assert_eq!(
            calls_to(&e, COMPARE_STRING_A_IMPORT),
            vec![vec![0x409, 0x1001, a, 2, b, 2]]
        );
        assert_eq!(e.mem.u32(thread_data + 0x70), 0xfd);
    }

    /// Maps the CRT's environment globals and sets `_environ` to an array of `entries`.
    fn environ(e: &mut Engine, entries: &[&[u8]]) -> u32 {
        e.map(ENVIRON & !0xfff, 0x1000);
        let array = e.mem.alloc(4 * entries.len() as u32 + 4);
        for (i, entry) in entries.iter().enumerate() {
            let text = put_string(e, entry);
            e.mem.set_u32(array + 4 * i as u32, text);
        }
        e.mem.set_u32(array + 4 * entries.len() as u32, 0);
        e.mem.set_u32(ENVIRON, array);
        array
    }

    #[test]
    fn findenv_finds_a_name_followed_by_its_equal_sign() {
        let (mut e, _, _) = crt(0, 0);
        environ(&mut e, &[b"PATH=x", b"PATHEXT=y", b"Home=z", b"TEMP"]);
        let name = put_string(&mut e, b"home=1");
        assert_eq!(e.call(0x00ee_6924, &args![4u32, name]).i32(), 2);
        let name = put_string(&mut e, b"PATHEXT");
        assert_eq!(e.call(0x00ee_6924, &args![7u32, name]).i32(), 1);
        // A name ending where the entry ends matches too.
        let name = put_string(&mut e, b"temp");
        assert_eq!(e.call(0x00ee_6924, &args![4u32, name]).i32(), 3);
        // Not found: minus the number of entries.
        let name = put_string(&mut e, b"PAT");
        assert_eq!(e.call(0x00ee_6924, &args![3u32, name]).i32(), -4);
    }

    #[test]
    fn copy_environ_duplicates_every_entry() {
        let (mut e, _, _) = crt(0, 0);
        assert_eq!(e.call(0x00ee_6976, &args![0u32]).u32(), 0);
        let source = environ(&mut e, &[b"A=1", b"BB=2"]);
        e.call_log = Some(vec![]);
        let copy = e.call(0x00ee_6976, &args![source]).u32();
        assert_eq!(calls_to(&e, CALLOC_CRT), vec![vec![3, 4]]);
        assert_ne!(e.mem.u32(copy), e.mem.u32(source));
        assert_eq!(read_string(&e, e.mem.u32(copy)), b"A=1");
        assert_eq!(read_string(&e, e.mem.u32(copy + 4)), b"BB=2");
        assert_eq!(e.mem.u32(copy + 8), 0);
        // An empty array gets one null word; a failed allocation is `_amsg_exit(9)`.
        let empty = environ(&mut e, &[]);
        let copy = e.call(0x00ee_6976, &args![empty]).u32();
        assert_eq!(e.mem.u32(copy), 0);
        e.register(CALLOC_CRT, |_, _| 0u32.into_ret());
        e.register(AMSG_EXIT, |_, _| Ret::default());
        e.map(0, 0x1000);
        e.mem.set_u32(0, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_6976, &args![empty]).u32(), 0);
        assert_eq!(calls_to(&e, AMSG_EXIT), vec![vec![9]]);
        assert_eq!(e.mem.u32(0), 0);
    }

    #[test]
    fn strdup_copies_or_answers_null() {
        let (mut e, _, _) = crt(0, 0);
        assert_eq!(e.call(0x00ee_6c1b, &args![0u32]).u32(), 0);
        let text = put_string(&mut e, b"abc");
        e.call_log = Some(vec![]);
        let copy = e.call(0x00ee_6c1b, &args![text]).u32();
        assert_eq!(calls_to(&e, MALLOC), vec![vec![4]]);
        assert_eq!(read_string(&e, copy), b"abc");
        e.register(STRCPY_S, |_, _| 0x22u32.into_ret());
        e.register(INVOKE_WATSON, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let copy = e.call(0x00ee_6c1b, &args![text]).u32();
        assert_ne!(copy, 0);
        assert_eq!(calls_to(&e, INVOKE_WATSON), vec![vec![0; 5]]);
        e.register(MALLOC, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ee_6c1b, &args![text]).u32(), 0);
    }

    #[test]
    fn mbschr_l_single_byte_code_page_uses_strchr() {
        let (mut e, error, thread_data) = crt(0, 0);
        let text = put_string(&mut e, b"a=b");
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ee_6c70, &args![text, 0x3du32, 0x66u32]).u32(),
            text + 1
        );
        assert_eq!(calls_to(&e, LOCALE_UPDATE)[0][1], 0x66);
        assert_eq!(calls_to(&e, STRCHR), vec![vec![text, 0x3d]]);
        assert_eq!(e.mem.u32(thread_data + 0x70), 0xfd);
        assert_eq!(e.call(0x00ee_6c70, &args![0u32, 0x3du32, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(error), 0x16);
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
        // `_mbschr` passes a null locale.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_6d2c, &args![text, 0x62u32]).u32(), text + 2);
        assert_eq!(calls_to(&e, LOCALE_UPDATE)[0][1], 0);
    }

    #[test]
    fn mbschr_l_multibyte_code_page_skips_trail_bytes() {
        let (mut e, _, thread_data) = crt(0, 0);
        let mbcinfo = e.mem.alloc(0x120);
        e.mem.set_u32(mbcinfo + 8, 1);
        // 0x81 is a lead byte.
        e.mem.set_u8(mbcinfo + 0x1d + 0x81, 4);
        e.register_double(LOCALE_UPDATE, move |e, a| {
            let locinfo = e.mem.alloc(0x20);
            e.mem.set_u32(a[0], locinfo);
            e.mem.set_u32(a[0] + 4, mbcinfo);
            e.mem.set_u32(a[0] + 8, thread_data);
            e.mem.set_u8(a[0] + 0xc, 0);
            Ret::default()
        });
        // "x" 0x81 '=' "=" : the first '=' is a trail byte.
        let text = put_string(&mut e, &[b'x', 0x81, b'=', b'=']);
        assert_eq!(
            e.call(0x00ee_6c70, &args![text, 0x3du32, 0u32]).u32(),
            text + 3
        );
        assert_eq!(
            e.call(0x00ee_6c70, &args![text, 0x813du32, 0u32]).u32(),
            text + 1
        );
        assert_eq!(
            e.call(0x00ee_6c70, &args![text, 0u32, 0u32]).u32(),
            text + 4
        );
        assert_eq!(e.call(0x00ee_6c70, &args![text, 0x79u32, 0u32]).u32(), 0);
        // A lead byte at the end answers null.
        let cut = put_string(&mut e, &[b'x', 0x81]);
        assert_eq!(e.call(0x00ee_6c70, &args![cut, 0u32, 0u32]).u32(), 0);
    }

    /// The engine for `__crtsetenv`: the CRT doubles, `_environ` set to `entries` (and
    /// `__initenv` elsewhere), a `_wenviron`, `__recalloc_crt` growing or shrinking a copy and
    /// `SetEnvironmentVariableA` recording the name and value it is given.
    fn setenv_engine(entries: &[&[u8]]) -> (Engine, u32, CallLog) {
        let (mut e, error, _) = crt(0, 0);
        environ(&mut e, entries);
        e.mem.set_u32(INITIAL_ENVIRON, 0x1234);
        let wide = e.mem.alloc(4);
        e.mem.set_u32(WIDE_ENVIRON, wide);
        e.register(RECALLOC_CRT, |e, a| {
            let count = if a[2] == 4 { a[1] } else { a[2] };
            let block = e.mem.alloc(4 * count);
            for i in 0..count {
                let word = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(block + 4 * i, word);
                if word == 0 {
                    break;
                }
            }
            block.into_ret()
        });
        let set: CallLog = Rc::new(RefCell::new(vec![]));
        let seen = set.clone();
        e.register_double(SET_ENVIRONMENT_VARIABLE_A, move |e, a| {
            let name = read_string(e, a[0]).iter().map(|b| u32::from(*b)).collect();
            let value = if a[1] == 0 {
                vec![u32::MAX]
            } else {
                read_string(e, a[1]).iter().map(|b| u32::from(*b)).collect()
            };
            seen.borrow_mut().push(name);
            seen.borrow_mut().push(value);
            1u32.into_ret()
        });
        (e, error, set)
    }

    fn entries(e: &Engine) -> Vec<Vec<u8>> {
        let mut out = vec![];
        let mut slot = e.mem.u32(ENVIRON);
        while e.mem.u32(slot) != 0 {
            out.push(read_string(e, e.mem.u32(slot)));
            slot += 4;
        }
        out
    }

    fn option(e: &mut Engine, text: &[u8]) -> (u32, u32) {
        let string = put_string(e, text);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, string);
        (slot, string)
    }

    #[test]
    fn crtsetenv_rejects_bad_options() {
        let (mut e, error, _) = setenv_engine(&[b"A=1"]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_69d0, &args![0u32, 0u32]).i32(), -1);
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
        assert_eq!(e.mem.u32(error), 0x16);
        e.mem.set_u32(error, 0);
        let (slot, _) = option(&mut e, b"NOEQUAL");
        assert_eq!(e.call(0x00ee_69d0, &args![slot, 1u32]).i32(), -1);
        assert_eq!(e.mem.u32(error), 0x16);
        e.mem.set_u32(error, 0);
        let (slot, _) = option(&mut e, b"=x");
        assert_eq!(e.call(0x00ee_69d0, &args![slot, 1u32]).i32(), -1);
        assert_eq!(e.mem.u32(error), 0x16);
        let empty = e.mem.alloc(4);
        assert_eq!(e.call(0x00ee_69d0, &args![empty, 1u32]).i32(), -1);
        assert_eq!(calls_to(&e, INVALID_PARAMETER).len(), 1);
    }

    #[test]
    fn crtsetenv_replaces_an_entry_and_sets_the_process_variable() {
        let (mut e, _, set) = setenv_engine(&[b"A=1", b"Bee=2"]);
        let old = e.mem.u32(e.mem.u32(ENVIRON) + 4);
        let (slot, string) = option(&mut e, b"bee=3");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_69d0, &args![slot, 1u32]).i32(), 0);
        assert_eq!(e.mem.u32(e.mem.u32(ENVIRON) + 4), string);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(entries(&e), vec![b"A=1".to_vec(), b"bee=3".to_vec()]);
        let frees = calls_to(&e, FREE);
        assert_eq!(frees[0], vec![old]);
        let as_words = |text: &[u8]| text.iter().map(|b| u32::from(*b)).collect::<Vec<_>>();
        assert_eq!(*set.borrow(), vec![as_words(b"bee"), as_words(b"3")]);
        // The copy given to the process is freed, the option is not.
        assert!(!frees.iter().any(|f| f[0] == string));
        // Without `primary` nothing goes to the process.
        let (slot, _) = option(&mut e, b"A=9");
        e.call(0x00ee_69d0, &args![slot, 0u32]);
        assert_eq!(set.borrow().len(), 2);
        assert_eq!(entries(&e)[0], b"A=9");
    }

    #[test]
    fn crtsetenv_removes_an_entry_and_shrinks_the_array() {
        let (mut e, _, set) = setenv_engine(&[b"A=1", b"B=2", b"C=3"]);
        let array = e.mem.u32(ENVIRON);
        let (slot, string) = option(&mut e, b"B=");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_69d0, &args![slot, 1u32]).i32(), 0);
        assert_eq!(calls_to(&e, RECALLOC_CRT), vec![vec![array, 3, 4]]);
        assert_eq!(entries(&e), vec![b"A=1".to_vec(), b"C=3".to_vec()]);
        // The option is freed after the process variable is removed (null value).
        assert_eq!(calls_to(&e, FREE).last().unwrap(), &vec![string]);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(set.borrow()[1], vec![u32::MAX]);
        // Removing a name that is not there frees the option and answers 0.
        let (slot, string) = option(&mut e, b"Z=");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_69d0, &args![slot, 1u32]).i32(), 0);
        assert_eq!(calls_to(&e, FREE), vec![vec![string]]);
        assert_eq!(set.borrow().len(), 2);
    }

    #[test]
    fn crtsetenv_appends_a_new_entry() {
        let (mut e, error, _) = setenv_engine(&[b"A=1", b"B=2"]);
        let array = e.mem.u32(ENVIRON);
        let (slot, string) = option(&mut e, b"C=3");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_69d0, &args![slot, 1u32]).i32(), 0);
        assert_eq!(calls_to(&e, RECALLOC_CRT), vec![vec![array, 4, 4]]);
        assert_eq!(e.mem.u32(e.mem.u32(ENVIRON) + 8), string);
        assert_eq!(entries(&e).len(), 3);
        assert_eq!(e.mem.u32(slot), 0);
        // A refused process variable answers -1 with `errno = EILSEQ`.
        e.register(SET_ENVIRONMENT_VARIABLE_A, |_, _| 0u32.into_ret());
        let (slot, _) = option(&mut e, b"D=4");
        assert_eq!(e.call(0x00ee_69d0, &args![slot, 1u32]).i32(), -1);
        assert_eq!(e.mem.u32(error), 0x2a);
        // A failed reallocation answers -1 and keeps the option.
        e.register(RECALLOC_CRT, |_, _| 0u32.into_ret());
        let (slot, string) = option(&mut e, b"E=5");
        assert_eq!(e.call(0x00ee_69d0, &args![slot, 1u32]).i32(), -1);
        assert_eq!(e.mem.u32(slot), string);
    }

    #[test]
    fn crtsetenv_copies_the_initial_environment_and_builds_a_missing_one() {
        let (mut e, _, _) = setenv_engine(&[b"A=1"]);
        let first = e.mem.u32(ENVIRON);
        e.mem.set_u32(INITIAL_ENVIRON, first);
        let (slot, _) = option(&mut e, b"A=2");
        assert_eq!(e.call(0x00ee_69d0, &args![slot, 0u32]).i32(), 0);
        assert_ne!(e.mem.u32(ENVIRON), first);
        assert_eq!(read_string(&e, e.mem.u32(first)), b"A=1");
        assert_eq!(entries(&e), vec![b"A=2".to_vec()]);
        // No `_environ`: a removal answers 0, an addition makes both arrays.
        e.mem.set_u32(ENVIRON, 0);
        e.mem.set_u32(WIDE_ENVIRON, 0);
        let (slot, _) = option(&mut e, b"B=");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_69d0, &args![slot, 0u32]).i32(), 0);
        assert!(calls_to(&e, MALLOC_CRT).is_empty());
        let (slot, _) = option(&mut e, b"B=1");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_69d0, &args![slot, 0u32]).i32(), 0);
        assert_eq!(calls_to(&e, MALLOC_CRT), vec![vec![4], vec![4]]);
        assert_eq!(entries(&e), vec![b"B=1".to_vec()]);
        assert_eq!(e.mem.u32(e.mem.u32(WIDE_ENVIRON)), 0);
        // A failed allocation answers -1.
        e.mem.set_u32(ENVIRON, 0);
        e.register(MALLOC_CRT, |_, _| 0u32.into_ret());
        let (slot, _) = option(&mut e, b"C=1");
        assert_eq!(e.call(0x00ee_69d0, &args![slot, 0u32]).i32(), -1);
        // A primary call with a `_wenviron` and no `_environ` builds it from the wide one.
        let wide = e.mem.alloc(4);
        e.mem.set_u32(WIDE_ENVIRON, wide);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_69d0, &args![slot, 1u32]).i32(), -1);
        assert!(calls_to(&e, WIDE_CHAR_TO_MULTI_BYTE).is_empty());
    }

    #[test]
    fn wtomb_environ_converts_and_sets_each_wide_entry() {
        let (mut e, error, _) = setenv_engine(&[b"A=1"]);
        let wide = e.mem.alloc(12);
        e.mem.set_u32(wide, 0x7000);
        e.mem.set_u32(wide + 4, 0x7100);
        e.mem.set_u32(wide + 8, 0);
        e.mem.set_u32(WIDE_ENVIRON, wide);
        e.register(WIDE_CHAR_TO_MULTI_BYTE, |e, a| {
            let text: &[u8] = if a[2] == 0x7000 { b"B=2" } else { b"BAD" };
            if a[4] == 0 {
                return (text.len() as u32 + 1).into_ret();
            }
            for (i, byte) in text.iter().enumerate() {
                e.mem.set_u8(a[4] + i as u32, *byte);
            }
            e.mem.set_u8(a[4] + text.len() as u32, 0);
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_63c3, &args![]).i32(), 0);
        let sizes = calls_to(&e, WIDE_CHAR_TO_MULTI_BYTE);
        assert_eq!(sizes[0], vec![0, 0, 0x7000, u32::MAX, 0, 0, 0, 0]);
        assert_eq!(calls_to(&e, CALLOC_CRT)[0], vec![4, 1]);
        assert_eq!(entries(&e), vec![b"A=1".to_vec(), b"B=2".to_vec()]);
        // "BAD" has no `=`: refused, so its block is freed.
        assert_eq!(e.mem.u32(error), 0x16);
        let freed = calls_to(&e, FREE).last().unwrap()[0];
        assert_eq!(read_string(&e, freed), b"BAD");
        // Failures answer -1.
        e.register(WIDE_CHAR_TO_MULTI_BYTE, |_, a| {
            if a[4] == 0 {
                4u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ee_63c3, &args![]).i32(), -1);
        assert_eq!(calls_to(&e, FREE).len(), 1);
        e.register(CALLOC_CRT, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ee_63c3, &args![]).i32(), -1);
        e.register(WIDE_CHAR_TO_MULTI_BYTE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ee_63c3, &args![]).i32(), -1);
    }
}
