//! The C runtime's start-up as far as the exit table, for bare-engine tests (no executable
//! mapped): the KERNEL32 imports the start-up reaches are registered at their import-table
//! slots (the platform's implementations, as `Engine::register_imports` does for the real
//! executable), the image words and texts the start-up reads are written with their image
//! values, then `__mtinit` (`00ed1327`: the FLS functions, `__init_pointers`, the locks,
//! the first thread block with its `EncodePointer`/`DecodePointer`) and the first C
//! initializer (`00ec6522`: the exit table from `__calloc_crt(32, 4)`, its encoded address
//! as start and end) run as the exe runs them. Afterwards `_atexit`, `__onexit` and
//! `doexit` work on the table as in the game.

use crate::prelude::*;

/// The KERNEL32 imports the start-up and the exit-table code call (import-table slot,
/// name), from the executable's import table.
const KERNEL32_IMPORTS: [(u32, &str); 18] = [
    (0x00fd_f044, "TlsGetValue"),
    (0x00fd_f048, "TlsAlloc"),
    (0x00fd_f04c, "TlsSetValue"),
    (0x00fd_f050, "TlsFree"),
    (0x00fd_f058, "DeleteCriticalSection"),
    (0x00fd_f05c, "EnterCriticalSection"),
    (0x00fd_f094, "GetLastError"),
    (0x00fd_f0ac, "GetProcAddress"),
    (0x00fd_f100, "LeaveCriticalSection"),
    (0x00fd_f104, "Sleep"),
    (0x00fd_f130, "InitializeCriticalSectionAndSpinCount"),
    (0x00fd_f198, "GetModuleHandleW"),
    (0x00fd_f1a0, "HeapAlloc"),
    (0x00fd_f1c0, "InterlockedIncrement"),
    (0x00fd_f1c4, "InterlockedDecrement"),
    (0x00fd_f1d0, "GetCurrentThreadId"),
    (0x00fd_f1dc, "SetLastError"),
    (0x00fd_f1c8, "WaitForSingleObject"),
];

/// The image pages the start-up reads or writes: the import table, the texts, the CRT's
/// `.data` words (locks, TLS/FLS indexes, encoded pointers, the exit table's words).
const PAGES: [u32; 8] = [
    0x00fd_f000,
    0x010f_2000,
    0x0101_1000,
    0x011c_1000,
    0x011c_2000,
    0x0126_f000,
    0x0127_0000,
    0x0127_1000,
];

/// The texts `__mtinit` looks up (`.rdata`): `L"KERNEL32.DLL"` and the function names.
const WIDE_KERNEL32: (u32, &str) = (0x010f_29e8, "KERNEL32.DLL");
const NAMES: [(u32, &str); 6] = [
    (0x010f_29d8, "EncodePointer"),
    (0x010f_2a04, "DecodePointer"),
    (0x010f_2a14, "FlsFree"),
    (0x010f_2a1c, "FlsSetValue"),
    (0x010f_2a28, "FlsGetValue"),
    (0x010f_2a34, "FlsAlloc"),
];

/// The lock table (`_locktable`, `011c28a8`, 0x24 pairs of section and kind): the numbers
/// whose kind is 1 in the image (statically allocated sections).
const LOCK_TABLE: u32 = 0x011c_28a8;
const STATIC_LOCKS: [u32; 14] = [0, 1, 3, 4, 6, 7, 8, 10, 12, 13, 14, 16, 17, 18];
/// `__flsindex` (`011c1cbc`) and `__tlsindex` (`011c1cc0`): -1 in the image.
const FLS_INDEX: u32 = 0x011c_1cbc;
const TLS_INDEX: u32 = 0x011c_1cc0;
/// The global locale (`__ptlocinfo`, `011c23f8`) and the initial locale information it
/// points at (`__initiallocinfo`, `011c2320`, 0xd8 bytes): the image's nonzero words
/// (offset, value); the first thread block takes a reference on it.
const PTLOCINFO: u32 = 0x011c_23f8;
const INITIAL_LOCALE_INFO: u32 = 0x011c_2320;
const INITIAL_LOCALE_WORDS: [(u32, u32); 13] = [
    (0x00, 1),
    (0x58, 0x011c_2318),
    (0x68, 0x011c_2318),
    (0x78, 0x011c_2318),
    (0x88, 0x011c_2318),
    (0x98, 0x011c_2318),
    (0xa8, 1),
    (0xac, 1),
    (0xbc, 0x011c_19d4),
    (0xc8, 0x010f_22d0),
    (0xcc, 0x010f_2758),
    (0xd0, 0x010f_28d8),
    (0xd4, 0x011c_2a98),
];
/// `__mtinit` and the first C initializer.
const MTINIT: u32 = 0x00ed_1327;
const ONEXIT_INIT: u32 = 0x00ec_6522;
/// The pointer set-up `__init_pointers` calls third (`00ec7af9`).
const INIT_POINTER_0126FD9C: u32 = 0x00ec_7af9;

/// Runs the start-up as far as the exit table on a bare engine (see the module notes).
pub fn exit_table(e: &mut Engine) {
    for page in PAGES {
        e.map(page, 0x1000);
    }
    let imports: Vec<crate::exe::Import> = KERNEL32_IMPORTS
        .iter()
        .map(|&(slot, name)| crate::exe::Import {
            slot,
            dll: "KERNEL32.dll".to_string(),
            name: name.to_string(),
        })
        .collect();
    e.register_imports(&imports);
    let (at, text) = WIDE_KERNEL32;
    for (i, unit) in text.encode_utf16().chain([0]).enumerate() {
        e.mem.set_u16(at + 2 * i as u32, unit);
    }
    for (at, name) in NAMES {
        e.mem.set_cstr(at, name.as_bytes());
    }
    for number in STATIC_LOCKS {
        e.mem.set_u32(LOCK_TABLE + number * 8 + 4, 1);
    }
    e.mem.set_u32(FLS_INDEX, u32::MAX);
    e.mem.set_u32(TLS_INDEX, u32::MAX);
    e.mem.set_u32(PTLOCINFO, INITIAL_LOCALE_INFO);
    for (offset, value) in INITIAL_LOCALE_WORDS {
        e.mem.set_u32(INITIAL_LOCALE_INFO + offset, value);
    }
    // `__init_pointers` stores the encoded null through `00ec7af9`, whose translation is in
    // engine-fallout (`unplaced/run_00ec7ad2.rs`); without that crate the same store stands
    // in for it.
    if !e.is_registered(INIT_POINTER_0126FD9C) {
        e.register(INIT_POINTER_0126FD9C, |e, a| {
            e.mem.set_u32(0x0126_fd9c, a[0]);
            Ret::default()
        });
    }
    assert_eq!(e.call(MTINIT, &args![]).u32(), 1, "__mtinit failed");
    assert_eq!(
        e.call(ONEXIT_INIT, &args![]).u32(),
        0,
        "__onexitinit failed"
    );
}
