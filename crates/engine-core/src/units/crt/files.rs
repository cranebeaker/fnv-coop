//! File-handle and user-interface pieces of the C runtime (MSVC 2008 LIBCMT, statically
//! linked into FalloutNV.exe 1.4.0.525), translated from the exe: `__fcloseall`
//! (`00edf400`), `_setmode_nolock` (`00ee617a`), `_chsize_nolock` (`00ee5fc4`) and
//! `___crtMessageBoxA` (`00ee1c43`).

use super::*;

/// `_lock` (`00eddf39`), `__fcloseall`'s `__finally` block (`00edf496`), `fclose`
/// (`00ec9907`), `free` (`00ecd291`), `errno` (`00ec85e3`), `__doserrno` (`00ec85f6`),
/// `_lseeki64_nolock` (`00ee1fa7`; handle, offset low, high, origin: `EDX:EAX`),
/// `_write_nolock` (`00edc010`), `__get_osfhandle` (`00ee2e45`), `__encoded_null`
/// (`00ed0f89`), `__encode_pointer` (`00ed0f17`), `__decode_pointer` (`00ed0f92`).
const LOCK: u32 = 0x00ed_df39;
const FCLOSEALL_FINALLY: u32 = 0x00ed_f496;
const FCLOSE: u32 = 0x00ec_9907;
const FREE_FN: u32 = 0x00ec_d291;
const ERRNO: u32 = 0x00ec_85e3;
const DOSERRNO: u32 = 0x00ec_85f6;
const LSEEKI64_NOLOCK: u32 = 0x00ee_1fa7;
const WRITE_NOLOCK: u32 = 0x00ed_c010;
const GET_OSFHANDLE: u32 = 0x00ee_2e45;
const SETMODE_NOLOCK: u32 = 0x00ee_617a;
const ENCODED_NULL: u32 = 0x00ed_0f89;
const ENCODE_POINTER: u32 = 0x00ed_0f17;
const DECODE_POINTER: u32 = 0x00ed_0f92;
/// The import slots of KERNEL32 `DeleteCriticalSection` (`00fdf058`), `GetProcessHeap`
/// (`00fdf20c`), `HeapAlloc` (`00fdf1a0`), `HeapFree` (`00fdf1a4`), `SetEndOfFile`
/// (`00fdf210`), `GetLastError` (`00fdf094`), `LoadLibraryA` (`00fdf0b0`) and
/// `GetProcAddress` (`00fdf0ac`).
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
const GET_PROCESS_HEAP: u32 = 0x00fd_f20c;
const HEAP_ALLOC: u32 = 0x00fd_f1a0;
const HEAP_FREE: u32 = 0x00fd_f1a4;
const SET_END_OF_FILE: u32 = 0x00fd_f210;
const GET_LAST_ERROR: u32 = 0x00fd_f094;
const LOAD_LIBRARY_A: u32 = 0x00fd_f0b0;
const GET_PROC_ADDRESS: u32 = 0x00fd_f0ac;
/// The stream table (`__piob`, `01270a70`) and its length (`_nstream`, `01271a80`); the
/// handle table (`__pioinfo`, `01270960`).
const PIOB: u32 = 0x0127_0a70;
const NSTREAM: u32 = 0x0127_1a80;
const PIOINFO: u32 = 0x0127_0960;
/// The encoded USER32 functions `___crtMessageBoxA` keeps: `MessageBoxA` (`01270804`),
/// `GetActiveWindow` (`01270808`), `GetLastActivePopup` (`0127080c`),
/// `GetProcessWindowStation` (`01270810`), `GetUserObjectInformationA` (`01270814`); the
/// names it loads them by (`.rdata`).
const MESSAGE_BOX: u32 = 0x0127_0804;
const ACTIVE_WINDOW: u32 = 0x0127_0808;
const LAST_ACTIVE_POPUP: u32 = 0x0127_080c;
const WINDOW_STATION: u32 = 0x0127_0810;
const USER_OBJECT_INFORMATION: u32 = 0x0127_0814;
const USER32_NAME: u32 = 0x010f_f034;
const MESSAGE_BOX_NAME: u32 = 0x010f_f028;
const ACTIVE_WINDOW_NAME: u32 = 0x010f_f018;
const LAST_ACTIVE_POPUP_NAME: u32 = 0x010f_f004;
const USER_OBJECT_INFORMATION_NAME: u32 = 0x010f_efe8;
const WINDOW_STATION_NAME: u32 = 0x010f_efd0;

// Translated from 00edf400 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00edf400 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759, scope table 011804c8) is not translated; its __finally block (00edf496) is called as the exe calls it on the way out.
/// `__fcloseall` (Ghidra's function ID), no arguments: under lock 1, every stream from the
/// fourth on (`__piob`, `_nstream` of them) that is in use (flags 0x83) is closed
/// (`fclose`; each success counted), and the dynamically made ones (index 20 on) lose
/// their critical section (`DeleteCriticalSection`, +0x20), are freed and their slot
/// cleared. The count is answered.
pub fn fcloseall(e: &mut Engine) -> u32 {
    let mut closed = 0u32;
    e.call(LOCK, &args![1u32]);
    let mut index = 3u32;
    while (index as i32) < (e.mem.u32(NSTREAM) as i32) {
        let offset = index.wrapping_mul(4);
        let table = e.mem.u32(PIOB);
        let stream = e.mem.u32(table.wrapping_add(offset));
        if stream != 0 {
            if e.mem.u8(stream.wrapping_add(0xc)) & 0x83 != 0
                && e.call(FCLOSE, &args![stream]).u32() != 0xffff_ffff
            {
                closed = closed.wrapping_add(1);
            }
            if (index as i32) >= 0x14 {
                let table = e.mem.u32(PIOB);
                let stream = e.mem.u32(table.wrapping_add(offset));
                e.call(DELETE_CRITICAL_SECTION, &args![stream.wrapping_add(0x20)]);
                let table = e.mem.u32(PIOB);
                let stream = e.mem.u32(table.wrapping_add(offset));
                e.call(FREE_FN, &args![stream]);
                let table = e.mem.u32(PIOB);
                e.mem.set_u32(table.wrapping_add(offset), 0);
            }
        }
        index = index.wrapping_add(1);
    }
    e.call(FCLOSEALL_FINALLY, &args![]);
    closed
}

// Translated from 00ee617a (disassembled, FalloutNV.exe 1.4.0.525)
/// `_setmode_nolock` (Ghidra's function ID), cdecl: handle, mode. The handle's entry
/// (`__pioinfo`) changes: `_O_TEXT` (0x4000) sets `FTEXT` (0x80 of `osfile`, +0x4) and the
/// ANSI text mode (+0x24, keeping bit 0x80); `_O_BINARY` (0x8000) clears `FTEXT`;
/// `_O_WTEXT`/`_O_U16TEXT` (0x10000, 0x20000) set it with UTF-16 (2), `_O_U8TEXT`
/// (0x40000) with UTF-8 (1); any other mode changes nothing. The old mode is answered:
/// `_O_BINARY` without `FTEXT`, else `_O_WTEXT` for a Unicode text mode or `_O_TEXT`.
pub fn setmode_nolock(e: &mut Engine, handle: u32, mode: u32) -> u32 {
    let block_slot = PIOINFO.wrapping_add((((handle as i32) >> 5) as u32).wrapping_mul(4));
    let offset = (handle & 0x1f) << 6;
    let entry = e.mem.u32(block_slot).wrapping_add(offset);
    let text_mode = ((e
        .mem
        .u8(entry.wrapping_add(0x24))
        .wrapping_add(e.mem.u8(entry.wrapping_add(0x24)))) as i8
        >> 1) as i32;
    let was_text = e.mem.u8(entry.wrapping_add(4)) & 0x80 != 0;
    let set_text = |e: &mut Engine, keep: u8, or: u8| {
        let osfile = e.mem.u8(entry.wrapping_add(4));
        e.mem.set_u8(entry.wrapping_add(4), osfile | 0x80);
        let at = e
            .mem
            .u32(block_slot)
            .wrapping_add(offset)
            .wrapping_add(0x24);
        let byte = e.mem.u8(at);
        e.mem.set_u8(at, (byte & keep) | or);
    };
    match mode {
        0x4000 => set_text(e, 0x80, 0),
        0x8000 => {
            let osfile = e.mem.u8(entry.wrapping_add(4));
            e.mem.set_u8(entry.wrapping_add(4), osfile & 0x7f);
        }
        0x1_0000 | 0x2_0000 => set_text(e, 0x82, 2),
        0x4_0000 => set_text(e, 0x81, 1),
        _ => {}
    }
    if !was_text {
        0x8000
    } else if text_mode != 0 {
        0x1_0000
    } else {
        0x4000
    }
}

/// `errno`'s value (`MOV EAX, [errno]`).
fn errno_value(e: &mut Engine) -> u32 {
    let slot = e.call(ERRNO, &args![]).u32();
    e.mem.u32(slot)
}

fn seek(e: &mut Engine, handle: u32, offset: u64, origin: u32) -> u64 {
    e.call(
        LSEEKI64_NOLOCK,
        &args![handle, offset as u32, (offset >> 32) as u32, origin],
    )
    .u64()
}

// Translated from 00ee5fc4 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_chsize_nolock` (Ghidra's function ID), cdecl: handle, new size (64-bit). The position
/// and the end are found (`_lseeki64_nolock`; a failure answers `errno`). Growing writes
/// zeros from a 0x1000-byte zeroed heap block (none: `ENOMEM`) in binary mode, at most
/// 0x1000 bytes a time (`_write_nolock`; a failure with `ERROR_ACCESS_DENIED` is `EACCES`
/// and stops with -1); the mode is restored and the block freed. Shrinking seeks to the size
/// and ends the file there (`SetEndOfFile`; a failure is `EACCES` with the system error in
/// `_doserrno`). A failure answers `errno`; otherwise the position is restored (its failure
/// also answers `errno`) and 0 answered.
pub fn chsize_nolock(e: &mut Engine, handle: u32, size_low: u32, size_high: u32) -> u32 {
    let size = u64::from(size_low) | u64::from(size_high) << 32;
    let mut result: u64 = 0;
    let position = seek(e, handle, 0, 1);
    if position == u64::MAX {
        return errno_value(e);
    }
    let end = seek(e, handle, 0, 2);
    if end == u64::MAX {
        return errno_value(e);
    }
    let mut extend = size.wrapping_sub(end) as i64;
    if extend > 0 {
        let heap = e.call(GET_PROCESS_HEAP, &args![]).u32();
        let block = e.call(HEAP_ALLOC, &args![heap, 8u32, 0x1000u32]).u32();
        if block == 0 {
            let slot = e.call(ERRNO, &args![]).u32();
            e.mem.set_u32(slot, 0xc);
            return errno_value(e);
        }
        let old_mode = e.call(SETMODE_NOLOCK, &args![handle, 0x8000u32]).u32();
        loop {
            let count = if extend >= 0x1000 {
                0x1000
            } else {
                extend as u32
            };
            let written = e.call(WRITE_NOLOCK, &args![handle, block, count]).u32();
            if written == 0xffff_ffff {
                let slot = e.call(DOSERRNO, &args![]).u32();
                if e.mem.u32(slot) == 5 {
                    let slot = e.call(ERRNO, &args![]).u32();
                    e.mem.set_u32(slot, 0xd);
                }
                result = u64::MAX;
                break;
            }
            extend = extend.wrapping_sub(i64::from(written as i32));
            if extend <= 0 {
                break;
            }
        }
        e.call(SETMODE_NOLOCK, &args![handle, old_mode]);
        let heap = e.call(GET_PROCESS_HEAP, &args![]).u32();
        e.call(HEAP_FREE, &args![heap, 0u32, block]);
        if result == u64::MAX {
            return errno_value(e);
        }
    } else if extend < 0 {
        if seek(e, handle, size, 0) == u64::MAX {
            return errno_value(e);
        }
        let os_handle = e.call(GET_OSFHANDLE, &args![handle]).u32();
        let ended = e.call(SET_END_OF_FILE, &args![os_handle]).u32();
        result = if ended != 0 { 0 } else { u64::MAX };
        if result == u64::MAX {
            let slot = e.call(ERRNO, &args![]).u32();
            e.mem.set_u32(slot, 0xd);
            let slot = e.call(DOSERRNO, &args![]).u32();
            let error = e.call(GET_LAST_ERROR, &args![]).u32();
            e.mem.set_u32(slot, error);
            return errno_value(e);
        }
    }
    if seek(e, handle, position, 0) == u64::MAX {
        return errno_value(e);
    }
    0
}

/// A USER32 function by name, kept encoded at `slot`.
fn keep_encoded(e: &mut Engine, library: u32, name: u32, slot: u32) -> u32 {
    let function = e.call(GET_PROC_ADDRESS, &args![library, name]).u32();
    let encoded = e.call(ENCODE_POINTER, &args![function]).u32();
    e.mem.set_u32(slot, encoded);
    encoded
}

// Translated from 00ee1c43 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___crtMessageBoxA` (Ghidra's function ID), cdecl: text, caption, type. On first use
/// USER32 is loaded (`LoadLibraryA`; none, or no `MessageBoxA`, answers 0) and its
/// functions kept encoded (`GetProcessWindowStation` only when
/// `GetUserObjectInformationA` is there). When the process's window station is not
/// visible (`UOI_FLAGS` without `WSF_VISIBLE`, or either call failing) the box is a service
/// notification (`MB_SERVICE_NOTIFICATION`, 0x200000) without an owner; otherwise the owner
/// is the active window's last active pop-up. `MessageBoxA(owner, text, caption, type)` is
/// answered.
pub fn crt_message_box_a(e: &mut Engine, text: u32, caption: u32, kind: u32) -> u32 {
    let null = e.call(ENCODED_NULL, &args![]).u32();
    let mut owner = 0u32;
    let mut kind = kind;
    if e.mem.u32(MESSAGE_BOX) == 0 {
        let library = e.call(LOAD_LIBRARY_A, &args![USER32_NAME]).u32();
        if library == 0 {
            return 0;
        }
        let message_box = e
            .call(GET_PROC_ADDRESS, &args![library, MESSAGE_BOX_NAME])
            .u32();
        if message_box == 0 {
            return 0;
        }
        let encoded = e.call(ENCODE_POINTER, &args![message_box]).u32();
        e.mem.set_u32(MESSAGE_BOX, encoded);
        keep_encoded(e, library, ACTIVE_WINDOW_NAME, ACTIVE_WINDOW);
        keep_encoded(e, library, LAST_ACTIVE_POPUP_NAME, LAST_ACTIVE_POPUP);
        let info = keep_encoded(
            e,
            library,
            USER_OBJECT_INFORMATION_NAME,
            USER_OBJECT_INFORMATION,
        );
        if info != 0 {
            keep_encoded(e, library, WINDOW_STATION_NAME, WINDOW_STATION);
        }
    }
    let mut find_owner = true;
    let station_word = e.mem.u32(WINDOW_STATION);
    if station_word != null && e.mem.u32(USER_OBJECT_INFORMATION) != null {
        let station_function = e.call(DECODE_POINTER, &args![station_word]).u32();
        let encoded = e.mem.u32(USER_OBJECT_INFORMATION);
        let information = e.call(DECODE_POINTER, &args![encoded]).u32();
        if station_function != 0 && information != 0 {
            let visible = e.with_stack(0x10, |e, frame| {
                // USEROBJECTFLAGS (EBP-0x14, 0xc bytes) and the length out (EBP-0x8).
                let flags = frame.addr();
                let needed = flags + 0xc;
                let station = e.call(station_function, &args![]).u32();
                station != 0
                    && e.call(information, &args![station, 1u32, flags, 0xcu32, needed])
                        .u32()
                        != 0
                    && e.mem.u8(flags + 8) & 1 != 0
            });
            if !visible {
                kind |= 0x20_0000;
                find_owner = false;
            }
        }
    }
    if find_owner {
        let encoded = e.mem.u32(ACTIVE_WINDOW);
        if encoded != null {
            let active = e.call(DECODE_POINTER, &args![encoded]).u32();
            if active != 0 {
                owner = e.call(active, &args![]).u32();
                let encoded = e.mem.u32(LAST_ACTIVE_POPUP);
                if owner != 0 && encoded != null {
                    let popup = e.call(DECODE_POINTER, &args![encoded]).u32();
                    if popup != 0 {
                        owner = e.call(popup, &args![owner]).u32();
                    }
                }
            }
        }
    }
    let encoded = e.mem.u32(MESSAGE_BOX);
    let message_box = e.call(DECODE_POINTER, &args![encoded]).u32();
    if message_box == 0 {
        return 0;
    }
    e.call(message_box, &args![owner, text, caption, kind])
        .u32()
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ed_f400, fcloseall() -> u32),
        entry!(0x00ee_617a, setmode_nolock(u32, u32) -> u32),
        entry!(0x00ee_5fc4, chsize_nolock(u32, u32, u32) -> u32),
        entry!(0x00ee_1c43, crt_message_box_a(u32, u32, u32) -> u32),
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

    /// An engine with one block of handle entries (handle 3 in use) and the given entry's
    /// `osfile` and text-mode bytes.
    fn handles(osfile: u8, text_mode: u8) -> (Engine, u32) {
        let mut e = Engine::new();
        e.mem.map(PIOINFO, 4);
        let block = e.mem.alloc(0x40 * 32);
        e.mem.set_u32(PIOINFO, block);
        let entry = block + 3 * 0x40;
        e.mem.set_u8(entry + 4, osfile);
        e.mem.set_u8(entry + 0x24, text_mode);
        e.call_log = Some(vec![]);
        (e, entry)
    }

    #[test]
    fn setmode_nolock_switches_and_answers_the_old_mode() {
        let (mut e, entry) = handles(0x81, 0x80);
        assert_eq!(e.call(0x00ee_617a, &args![3u32, 0x8000u32]).u32(), 0x4000);
        assert_eq!(e.mem.u8(entry + 4), 0x01);
        assert_eq!(setmode_nolock(&mut e, 3, 0x2_0000), 0x8000);
        assert_eq!((e.mem.u8(entry + 4), e.mem.u8(entry + 0x24)), (0x81, 0x82));
        assert_eq!(setmode_nolock(&mut e, 3, 0x4_0000), 0x1_0000);
        assert_eq!(e.mem.u8(entry + 0x24), 0x81);
        assert_eq!(setmode_nolock(&mut e, 3, 0x4000), 0x1_0000);
        assert_eq!(e.mem.u8(entry + 0x24), 0x80);
        assert_eq!(setmode_nolock(&mut e, 3, 0x1234), 0x4000);
    }

    #[test]
    fn fcloseall_closes_streams_and_frees_the_made_ones() {
        let mut e = Engine::new();
        e.mem.map(PIOB, 4);
        e.mem.map(NSTREAM, 4);
        let table = e.mem.alloc(4 * 22);
        e.mem.set_u32(PIOB, table);
        e.mem.set_u32(NSTREAM, 22);
        let mut streams = vec![];
        for i in [0u32, 4, 5, 21] {
            let s = e.mem.alloc(0x40);
            e.mem.set_u32(s + 0xc, if i == 5 { 0 } else { 1 });
            e.mem.set_u32(table + i * 4, s);
            streams.push(s);
        }
        for addr in [LOCK, FCLOSEALL_FINALLY, DELETE_CRITICAL_SECTION, FREE_FN] {
            e.register_double(addr, |_, _| Ret::default());
        }
        e.register_double(FCLOSE, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ed_f400, &args![]).u32(), 2);
        assert_eq!(calls(&e, FCLOSE), vec![vec![streams[1]], vec![streams[3]]]);
        assert_eq!(
            calls(&e, DELETE_CRITICAL_SECTION),
            vec![vec![streams[3] + 0x20]]
        );
        assert_eq!(calls(&e, FREE_FN), vec![vec![streams[3]]]);
        assert_eq!(e.mem.u32(table + 21 * 4), 0);
        assert_eq!(e.mem.u32(table + 4 * 4), streams[1]);
    }

    /// A file of `length` bytes at position 7, through doubles of the low-level calls.
    fn file_engine(length: u64) -> (Engine, std::rc::Rc<std::cell::Cell<u64>>, u32) {
        let mut e = Engine::new();
        let len = std::rc::Rc::new(std::cell::Cell::new(length));
        let pos = std::rc::Rc::new(std::cell::Cell::new(7u64));
        let (l, p) = (len.clone(), pos.clone());
        e.register_double(LSEEKI64_NOLOCK, move |_, a| {
            let offset = u64::from(a[1]) | u64::from(a[2]) << 32;
            let at = match a[3] {
                0 => offset,
                1 => p.get() + offset,
                _ => l.get() + offset,
            };
            p.set(at);
            at.into_ret()
        });
        let (l, p) = (len.clone(), pos.clone());
        e.register_double(WRITE_NOLOCK, move |_, a| {
            p.set(p.get() + u64::from(a[2]));
            l.set(l.get().max(p.get()));
            a[2].into_ret()
        });
        let l = len.clone();
        let p = pos.clone();
        e.register_double(SET_END_OF_FILE, move |_, _| {
            l.set(p.get());
            1u32.into_ret()
        });
        e.register_double(GET_OSFHANDLE, |_, a| a[0].into_ret());
        e.register_double(GET_PROCESS_HEAP, |_, _| 0x99u32.into_ret());
        e.register_double(HEAP_ALLOC, |e, a| e.mem.alloc(a[2]).into_ret());
        e.register_double(HEAP_FREE, |_, _| 1u32.into_ret());
        e.register_double(SETMODE_NOLOCK, |_, a| (a[1] ^ 0xc000).into_ret());
        let slot = e.mem.alloc(8);
        e.register_double(ERRNO, move |_, _| slot.into_ret());
        e.register_double(DOSERRNO, move |_, _| (slot + 4).into_ret());
        e.call_log = Some(vec![]);
        let _ = pos;
        (e, len, slot)
    }

    #[test]
    fn chsize_nolock_grows_with_zeros_and_shrinks_at_the_end() {
        let (mut e, len, slot) = file_engine(0x10);
        assert_eq!(e.call(0x00ee_5fc4, &args![3u32, 0x2010u32, 0u32]).u32(), 0);
        assert_eq!(len.get(), 0x2010);
        let writes: Vec<u32> = calls(&e, WRITE_NOLOCK).iter().map(|c| c[2]).collect();
        assert_eq!(writes, vec![0x1000, 0x1000]);
        // Binary mode for the writes, the old mode back.
        assert_eq!(
            calls(&e, SETMODE_NOLOCK),
            vec![vec![3, 0x8000], vec![3, 0x4000]]
        );
        assert_eq!(chsize_nolock(&mut e, 3, 0x20, 0), 0);
        assert_eq!(len.get(), 0x20);
        assert_eq!(chsize_nolock(&mut e, 3, 0x20, 0), 0);
        // A refused write is EACCES.
        e.register_double(WRITE_NOLOCK, |_, _| 0xffff_ffffu32.into_ret());
        e.mem.set_u32(slot + 4, 5);
        assert_eq!(chsize_nolock(&mut e, 3, 0x40, 0), 0xd);
    }

    #[test]
    fn crt_message_box_a_loads_user32_and_finds_the_owner() {
        let mut e = Engine::new();
        e.mem.map(MESSAGE_BOX, 0x14);
        e.register_double(ENCODED_NULL, |_, _| 0x55u32.into_ret());
        e.register_double(ENCODE_POINTER, |_, a| (a[0] ^ 0x55).into_ret());
        e.register_double(DECODE_POINTER, |_, a| (a[0] ^ 0x55).into_ret());
        e.register_double(LOAD_LIBRARY_A, |_, _| 0x7000_0000u32.into_ret());
        e.register_double(GET_PROC_ADDRESS, |_, a| {
            (0x0040_8000 + (a[1] & 0xff)).into_ret()
        });
        let message_box = 0x0040_8000 + (MESSAGE_BOX_NAME & 0xff);
        let active = 0x0040_8000 + (ACTIVE_WINDOW_NAME & 0xff);
        let popup = 0x0040_8000 + (LAST_ACTIVE_POPUP_NAME & 0xff);
        let station = 0x0040_8000 + (WINDOW_STATION_NAME & 0xff);
        let info = 0x0040_8000 + (USER_OBJECT_INFORMATION_NAME & 0xff);
        e.register_double(message_box, |_, a| (a[0] + a[3]).into_ret());
        e.register_double(active, |_, _| 0x100u32.into_ret());
        e.register_double(popup, |_, a| (a[0] + 1).into_ret());
        e.register_double(station, |_, _| 0x200u32.into_ret());
        let visible = std::rc::Rc::new(std::cell::Cell::new(1u8));
        let v = visible.clone();
        e.register_double(info, move |e, a| {
            e.mem.set_u8(a[2] + 8, v.get());
            1u32.into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ee_1c43, &args![1u32, 2u32, 0x10u32]).u32(),
            0x111
        );
        assert_eq!(e.mem.u32(MESSAGE_BOX), message_box ^ 0x55);
        // An invisible window station: a service notification without an owner.
        visible.set(0);
        assert_eq!(crt_message_box_a(&mut e, 1, 2, 0x10), 0x20_0010);
        assert_eq!(calls(&e, LOAD_LIBRARY_A).len(), 1);
        e.mem.set_u32(MESSAGE_BOX, 0);
        e.register_double(LOAD_LIBRARY_A, |_, _| 0u32.into_ret());
        assert_eq!(crt_message_box_a(&mut e, 1, 2, 0), 0);
    }
}
