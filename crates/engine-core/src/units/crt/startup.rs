//! The C runtime's start-up of its handle and stream tables (MSVC 2008 LIBCMT, statically
//! linked into FalloutNV.exe 1.4.0.525), translated from the exe: `_RTC_Initialize`
//! (`00eddb1e`) and `_ioinit` (`00ed7e18`), which `mainCRTStartup` runs after `_mtinit`, and
//! `__initstdio` (`00ecdf4a`), one of the C initializers `_cinit` runs through `_initterm_e`
//! (`010113c4..010113e8`). Without them no `FILE` stream can be opened (`_getstream` finds no
//! `__piob` table) and no handle has its `ioinfo`.

use super::*;

/// KERNEL32 import slots: `GetStartupInfoA`, `GetStdHandle`, `GetFileType`, `SetHandleCount`.
const IMPORT_GET_STARTUP_INFO_A: u32 = 0x00fd_f190;
const IMPORT_GET_STD_HANDLE_SLOT: u32 = 0x00fd_f1e0;
const IMPORT_GET_FILE_TYPE_SLOT: u32 = 0x00fd_f200;
const IMPORT_SET_HANDLE_COUNT: u32 = 0x00fd_f1fc;
/// `__calloc_crt` (`00ed0d24`, cdecl: count, size) and `__crtInitCritSecAndSpinCount`
/// (`00ede19c`, cdecl: section, spin count).
const CALLOC_CRT: u32 = 0x00ed_0d24;
const INIT_CRIT_SEC_AND_SPIN_COUNT: u32 = 0x00ed_e19c;
/// `_nstream` (`01271a80`), `__piob` (`01270a70`) and `_iob` (`011c1a08`, 20 streams of 0x20
/// bytes to `011c1c88`).
const NSTREAM_WORD: u32 = 0x0127_1a80;
const PIOB_WORD: u32 = 0x0127_0a70;
const IOB: u32 = 0x011c_1a08;
const IOB_END: u32 = 0x011c_1c88;
/// The `.CRT$XI`-like table `_RTC_Initialize` runs (`0113ec34..0113ec34`: empty in this exe).
const RTC_TABLE: (u32, u32) = (0x0113_ec34, 0x0113_ec34);

/// KERNEL32 `HeapCreate`'s import slot, and `__active_heap` (`01270948`).
const IMPORT_HEAP_CREATE: u32 = 0x00fd_f254;
const ACTIVE_HEAP: u32 = 0x0127_0948;

// Translated from 00eddb6a (disassembled, FalloutNV.exe 1.4.0.525)
/// `_heap_init` (LIBCMT/heapinit.c, engine map), cdecl: the multi-thread flag
/// `mainCRTStartup` passes (1). `_crtheap` (`012705bc`) is `HeapCreate(flag == 0, 0x1000, 0)`
/// (`HEAP_NO_SERIALIZE` only for a single-threaded start); none answers 0, else
/// `__active_heap` (`01270948`) becomes 1 (the system heap) and 1 is answered.
pub fn heap_init(e: &mut Engine, multithreaded: u32) -> u32 {
    let options = u32::from(multithreaded == 0);
    let heap = e
        .call(IMPORT_HEAP_CREATE, &args![options, 0x1000u32, 0u32])
        .u32();
    e.mem.set_u32(LOCKS_READY, heap);
    if heap == 0 {
        return 0;
    }
    e.mem.set_u32(ACTIVE_HEAP, 1);
    1
}

// Translated from 00eddb1e (disassembled, FalloutNV.exe 1.4.0.525)
/// `_RTC_Initialize` (LIBCMT/initsect.cpp, engine map), no arguments: calls each non-null
/// function of its table (`0113ec34` to `0113ec34`, so none in this exe).
pub fn rtc_initialize(e: &mut Engine) {
    let mut at = RTC_TABLE.0;
    while at < RTC_TABLE.1 {
        let function = e.mem.u32(at);
        if function != 0 {
            e.call(function, &args![]);
        }
        at += 4;
    }
}

/// An `ioinfo` entry as `_ioinit` starts it: no handle (-1), `osfile` 0, the pipe and UTF-8
/// look-ahead bytes 10 (+5, +0x25, +0x26), the lock flag (+8) and +0x38 0, +0x34 0; `+0x24`
/// cleared (`first`) or only its bit 7 kept.
fn clear_entry(e: &mut Engine, entry: u32, first: bool) {
    e.mem.set_u8(entry + 4, 0);
    e.mem.set_u32(entry, 0xffff_ffff);
    e.mem.set_u8(entry + 5, 0xa);
    e.mem.set_u32(entry + 8, 0);
    let byte = if first {
        0
    } else {
        e.mem.u8(entry + 0x24) & 0x80
    };
    e.mem.set_u8(entry + 0x24, byte);
    e.mem.set_u8(entry + 0x25, 0xa);
    e.mem.set_u8(entry + 0x26, 0xa);
    e.mem.set_u32(entry + 0x38, 0);
    e.mem.set_u8(entry + 0x34, 0);
}

// Translated from 00ed7e18 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed7e18 - the SEH frame (__SEH_prolog4/__SEH_epilog4, 00ed1714/00ed1759) around GetStartupInfoA is not translated.
/// `_ioinit` (LIBCMT/ioinit.c, engine map), no arguments: 0, or -1 when an allocation or a
/// lock's set-up fails. `GetStartupInfoA`; the first block of 32 `ioinfo` entries
/// (`__calloc_crt(0x20, 0x40)`) goes to `__pioinfo[0]` (`01270960`) with `_nhandle` 32, each
/// entry started (no handle). Handles a parent passed (`cbReserved2` and `lpReserved2`: a
/// count, the `osfile` bytes, the handles; at most 0x800) get more blocks as needed (stopping
/// at the first failed allocation) and, when open (`osfile` bit 0) and a pipe or a known file
/// type (`GetFileType`), their entries with a lock. Then handles 0..2: one passed stays and
/// is text (`FTEXT` 0x80); otherwise it is `FOPEN|FTEXT` with `GetStdHandle(-10/-11/-12)`:
/// a handle with a file type is kept (`FDEV` 0x40 for a character device, `FPIPE` 8 for a
/// pipe) with a lock, else it is `FDEV` with no handle (-2). Finally
/// `SetHandleCount(_nhandle)`.
pub fn ioinit(e: &mut Engine) -> i32 {
    // STARTUPINFOA (0x44 bytes) at EBP-0x64: `cbReserved2` at +0x32, `lpReserved2` at +0x34.
    e.with_stack(0x44, |e, info| {
        let info = info.addr();
        e.call(IMPORT_GET_STARTUP_INFO_A, &args![info]);
        let block = e.call(CALLOC_CRT, &args![0x20u32, 0x40u32]).u32();
        if block == 0 {
            return -1;
        }
        e.mem.set_u32(PIOINFO, block);
        e.mem.set_u32(NHANDLE, 0x20);
        let mut entry = block;
        while entry < e.mem.u32(PIOINFO).wrapping_add(0x800) {
            clear_entry(e, entry, true);
            entry += 0x40;
        }
        let reserved = e.mem.u32(info + 0x34);
        if e.mem.u16(info + 0x32) != 0 && reserved != 0 {
            let mut count = e.mem.u32(reserved) as i32;
            let mut flags = reserved + 4;
            let mut handles = flags.wrapping_add(count as u32);
            if count >= 0x800 {
                count = 0x800;
            }
            let mut index = 1u32;
            while (e.mem.u32(NHANDLE) as i32) < count {
                let more = e.call(CALLOC_CRT, &args![0x20u32, 0x40u32]).u32();
                if more == 0 {
                    count = e.mem.u32(NHANDLE) as i32;
                    break;
                }
                let slot = PIOINFO + 4 * index;
                e.mem.set_u32(slot, more);
                let handles_now = e.mem.u32(NHANDLE).wrapping_add(0x20);
                e.mem.set_u32(NHANDLE, handles_now);
                let mut entry = more;
                while entry < e.mem.u32(slot).wrapping_add(0x800) {
                    clear_entry(e, entry, false);
                    entry += 0x40;
                }
                index += 1;
            }
            let mut i = 0i32;
            while i < count {
                let handle = e.mem.u32(handles);
                let flag = e.mem.u8(flags);
                let open = handle != 0xffff_ffff
                    && handle != 0xffff_fffe
                    && flag & 1 != 0
                    && (flag & 8 != 0
                        || e.call(IMPORT_GET_FILE_TYPE_SLOT, &args![handle]).u32() != 0);
                if open {
                    let entry = ioinfo(e, i as u32);
                    e.mem.set_u32(entry, e.mem.u32(handles));
                    e.mem.set_u8(entry + 4, e.mem.u8(flags));
                    if e.call(INIT_CRIT_SEC_AND_SPIN_COUNT, &args![entry + 0xc, 0xfa0u32])
                        .u32()
                        == 0
                    {
                        return -1;
                    }
                    let locks = e.mem.u32(entry + 8).wrapping_add(1);
                    e.mem.set_u32(entry + 8, locks);
                }
                i += 1;
                flags = flags.wrapping_add(1);
                handles = handles.wrapping_add(4);
            }
        }
        for i in 0..3u32 {
            let entry = e.mem.u32(PIOINFO).wrapping_add(i << 6);
            let handle = e.mem.u32(entry);
            if handle != 0xffff_ffff && handle != 0xffff_fffe {
                let flag = e.mem.u8(entry + 4) | 0x80;
                e.mem.set_u8(entry + 4, flag);
                continue;
            }
            e.mem.set_u8(entry + 4, 0x81);
            let which = match i {
                0 => 0xffff_fff6u32,
                1 => 0xffff_fff5,
                _ => 0xffff_fff4,
            };
            let handle = e.call(IMPORT_GET_STD_HANDLE_SLOT, &args![which]).u32();
            let kind = if handle != 0xffff_ffff && handle != 0 {
                e.call(IMPORT_GET_FILE_TYPE_SLOT, &args![handle]).u32()
            } else {
                0
            };
            if kind != 0 {
                e.mem.set_u32(entry, handle);
                let kind = kind & 0xff;
                if kind == 2 {
                    let flag = e.mem.u8(entry + 4) | 0x40;
                    e.mem.set_u8(entry + 4, flag);
                } else if kind == 3 {
                    let flag = e.mem.u8(entry + 4) | 8;
                    e.mem.set_u8(entry + 4, flag);
                }
                if e.call(INIT_CRIT_SEC_AND_SPIN_COUNT, &args![entry + 0xc, 0xfa0u32])
                    .u32()
                    == 0
                {
                    return -1;
                }
                let locks = e.mem.u32(entry + 8).wrapping_add(1);
                e.mem.set_u32(entry + 8, locks);
            } else {
                let flag = e.mem.u8(entry + 4) | 0x40;
                e.mem.set_u8(entry + 4, flag);
                e.mem.set_u32(entry, 0xffff_fffe);
            }
        }
        let handles = e.mem.u32(NHANDLE);
        e.call(IMPORT_SET_HANDLE_COUNT, &args![handles]);
        0
    })
}

// Translated from 00ecdf4a (disassembled, FalloutNV.exe 1.4.0.525)
/// `__initstdio` (LIBCMT/_file.c, engine map), a C initializer, no arguments: `_nstream`
/// (`01271a80`) becomes 512 when 0 and at least 20; `__piob` (`01270a70`) is
/// `__calloc_crt(_nstream, 4)`, or with `_nstream` 20 `__calloc_crt(20, 4)` when that fails
/// (`ENOMEM`-like answer 0x1A when that fails too). The first 20 slots hold the `_iob`
/// streams (`011c1a08`, 0x20 bytes each); `stdin`, `stdout` and `stderr` whose handle
/// (`ioinfo` +0) is -1, -2 or 0 get `_file` -2. Answers 0.
pub fn initstdio(e: &mut Engine) -> u32 {
    let mut count = e.mem.u32(NSTREAM_WORD) as i32;
    if count == 0 {
        count = 0x200;
        e.mem.set_u32(NSTREAM_WORD, count as u32);
    } else if count < 0x14 {
        count = 0x14;
        e.mem.set_u32(NSTREAM_WORD, count as u32);
    }
    let mut table = e.call(CALLOC_CRT, &args![count as u32, 4u32]).u32();
    e.mem.set_u32(PIOB_WORD, table);
    if table == 0 {
        e.mem.set_u32(NSTREAM_WORD, 0x14);
        table = e.call(CALLOC_CRT, &args![0x14u32, 4u32]).u32();
        e.mem.set_u32(PIOB_WORD, table);
        if table == 0 {
            return 0x1a;
        }
    }
    let mut offset = 0u32;
    let mut stream = IOB;
    while stream < IOB_END {
        let table = e.mem.u32(PIOB_WORD);
        e.mem.set_u32(table.wrapping_add(offset), stream);
        stream += 0x20;
        offset += 4;
    }
    let mut index = 0u32;
    let mut file = IOB + 0x10;
    while file < IOB + 0x70 {
        let block = e.mem.u32(PIOINFO + 4 * (((index as i32) >> 5) as u32));
        let handle = e.mem.u32(block.wrapping_add((index & 0x1f) << 6));
        if handle == 0xffff_ffff || handle == 0xffff_fffe || handle == 0 {
            e.mem.set_u32(file, 0xffff_fffe);
        }
        file += 0x20;
        index += 1;
    }
    0
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ed_db6a, heap_init(u32) -> u32),
        entry!(0x00ed_db1e, rtc_initialize()),
        entry!(0x00ed_7e18, ioinit() -> i32),
        entry!(0x00ec_df4a, initstdio() -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine whose heap answers `__calloc_crt`, with the start-up info a GUI process got
    /// (no handles passed), no standard handles (`GetStdHandle` 0, as measured) and the locks'
    /// set-up doubled.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0127_0000, 0x2000);
        e.map(0x011c_1000, 0x1000);
        e.register(CALLOC_CRT, |e, a| {
            let block = e.mem.alloc(a[0] * a[1]);
            e.mem.write(block, &vec![0; (a[0] * a[1]) as usize]);
            block.into_ret()
        });
        e.register(IMPORT_GET_STARTUP_INFO_A, |e, a| {
            e.mem.write(a[0], &[0; 0x44]);
            e.mem.set_u32(a[0], 0x44);
            Ret::default()
        });
        e.register(IMPORT_GET_STD_HANDLE_SLOT, |_, _| 0u32.into_ret());
        e.register(IMPORT_SET_HANDLE_COUNT, |_, a| a[0].into_ret());
        e.register(INIT_CRIT_SEC_AND_SPIN_COUNT, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn ioinit_starts_32_handles_with_no_standard_ones() {
        let mut e = engine();
        assert_eq!(e.call(0x00ed_7e18, &args![]).i32(), 0);
        let block = e.mem.u32(PIOINFO);
        assert_ne!(block, 0);
        assert_eq!(e.mem.u32(NHANDLE), 0x20);
        for i in 0..3 {
            let entry = block + 0x40 * i;
            assert_eq!(e.mem.u32(entry), 0xffff_fffe);
            assert_eq!(e.mem.u8(entry + 4), 0xc1);
            assert_eq!(e.mem.u8(entry + 0x25), 0xa);
        }
        assert_eq!(e.mem.u32(block + 0x40 * 5), 0xffff_ffff);
        assert_eq!(e.mem.u8(block + 0x40 * 5 + 4), 0);
        let counts: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == IMPORT_SET_HANDLE_COUNT)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(counts, [vec![0x20]]);
    }

    #[test]
    fn initstdio_makes_the_stream_table_and_closes_the_missing_standard_streams() {
        let mut e = engine();
        e.call(0x00ed_7e18, &args![]);
        assert_eq!(e.call(0x00ec_df4a, &args![]).u32(), 0);
        assert_eq!(e.mem.u32(NSTREAM_WORD), 0x200);
        let table = e.mem.u32(PIOB_WORD);
        assert_eq!(e.mem.u32(table), IOB);
        assert_eq!(e.mem.u32(table + 4 * 19), IOB + 0x20 * 19);
        assert_eq!(e.mem.u32(table + 4 * 20), 0);
        for i in 0..3 {
            assert_eq!(e.mem.u32(IOB + 0x20 * i + 0x10), 0xffff_fffe);
        }
        // A small `_nstream` is raised to 20.
        e.mem.set_u32(NSTREAM_WORD, 5);
        e.call(0x00ec_df4a, &args![]);
        assert_eq!(e.mem.u32(NSTREAM_WORD), 0x14);
    }

    #[test]
    fn heap_init_keeps_the_heap_and_picks_the_system_heap() {
        let mut e = engine();
        e.register(IMPORT_HEAP_CREATE, |_, a| (0x0e50_0000 + a[0]).into_ret());
        assert_eq!(e.call(0x00ed_db6a, &args![1u32]).u32(), 1);
        assert_eq!(e.mem.u32(LOCKS_READY), 0x0e50_0000);
        assert_eq!(e.mem.u32(ACTIVE_HEAP), 1);
        // Single-threaded: HEAP_NO_SERIALIZE; no heap answers 0 and leaves the choice.
        e.mem.set_u32(ACTIVE_HEAP, 0);
        e.register(IMPORT_HEAP_CREATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00ed_db6a, &args![0u32]).u32(), 0);
        assert_eq!(e.mem.u32(LOCKS_READY), 0);
        assert_eq!(e.mem.u32(ACTIVE_HEAP), 0);
    }
}
