//! `(unplaced)/run_00aa3880` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The `MemoryManager` constructor (the static initializer `00fb66a0` runs
//! it on the singleton; the host runs its allocator stand-in at that point
//! instead, docs/ENGINE_CRATE.md). `MemoryManager`'s own unit is
//! `BSCore/memorymanager.cpp`, so its fields are written at their PC
//! offsets here, each named after the Xbox PDB member the offsets line up
//! with (the PC class lacks the Xbox build's leading `RemoteLog` and context
//! tracker).

#[allow(unused_imports)]
use crate::prelude::*;

/// The constructor of the thread-to-scrap-heap map at `+0x114`
/// (`thiscall(map, 0x25)`).
const SCRAP_HEAP_MAP_CONSTRUCT: u32 = 0x00aa_47e0;
/// The vtable the constructor then installs in that map.
const SCRAP_HEAP_MAP_VTABLE: u32 = 0x010a_25f8;
/// `memset(dest, value, count)` (LIBCMT, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;
/// The import slots of `GlobalMemoryStatusEx(&status)` and
/// `MessageBoxA(window, text, caption, type)`.
const GLOBAL_MEMORY_STATUS_EX: u32 = 0x00fd_f14c;
const MESSAGE_BOX_A: u32 = 0x00fd_f2f4;
/// `"Not enough memory to run application."` and `"Memory Error"`.
const NOT_ENOUGH_MEMORY_TEXT: u32 = 0x010a_2590;
const MEMORY_ERROR_CAPTION: u32 = 0x010a_2580;
/// `_exit(code)` (LIBCMT).
const EXIT: u32 = 0x00ec_cec5;
/// `MemoryManager::Initialize` (engine map, `thiscall`).
const MEMORY_MANAGER_INITIALIZE: u32 = 0x00aa_39c0;
/// The allocator start-up `00866690` (`thiscall`).
const ALLOCATOR_START_UP: u32 = 0x0086_6690;
/// The scrap-heap set-up `00aa58d0` (`BSCore/scrapheap.cpp` by the map; no
/// arguments).
const SCRAP_HEAP_SET_UP: u32 = 0x00aa_58d0;

// Translated from 00aa3880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryManager::MemoryManager` (the class by the fields it writes and
/// `MemoryManager::Initialize`, which it calls): clears `bInitialized`
/// (+0), `iNumHeaps` (+2), `iNumPhysicalHeaps` (+4) and `ppHeaps` (+8);
/// constructs `kThreadScrapHeapMap` (+0x114; `00aa47e0` with 0x25, then
/// its vtable); clears `ppPhysicalHeaps` (+0x124), sets `bSpecialHeaps`
/// and `bAllowPoolUse` (+0x128, +0x129), clears the words at +0x12c,
/// +0x130 and +0x134 and the 0x108 bytes of `pHeapsByContextA` (+0xc).
/// When `GlobalMemoryStatusEx` succeeds with less than 0x20000000 bytes of
/// `ullAvailPageFile`, shows "Not enough memory to run application." and
/// exits. Then `Initialize`, the allocator start-up `00866690` and
/// `00aa58d0`. Returns `this`.
// Translation incomplete: 00aa3880 - the FS:[0] exception frame (destroys kThreadScrapHeapMap if a later step throws) is not translated
pub fn memory_manager_construct(e: &mut Engine, this: Ptr) -> Ptr {
    let at = this.addr();
    e.mem.set_u8(at, 0);
    e.mem.set_u16(at + 2, 0);
    e.mem.set_u16(at + 4, 0);
    e.mem.set_u32(at + 8, 0);
    e.call(SCRAP_HEAP_MAP_CONSTRUCT, &args![at + 0x114, 0x25u32]);
    e.mem.set_u32(at + 0x114, SCRAP_HEAP_MAP_VTABLE);
    e.mem.set_u32(at + 0x124, 0);
    e.mem.set_u8(at + 0x128, 1);
    e.mem.set_u8(at + 0x129, 1);
    e.mem.set_u32(at + 0x12c, 0);
    e.mem.set_u32(at + 0x130, 0);
    e.mem.set_u32(at + 0x134, 0);
    e.call(MEMSET, &args![at + 0xc, 0u32, 0x108u32]);
    // `MEMORYSTATUSEX` on the stack, `dwLength` = 0x40; `ullAvailPageFile`
    // is its quadword at +0x20.
    let low_on_memory = e.with_stack(0x40, |e, status| {
        e.mem.set_u32(status.addr(), 0x40);
        let ok = e.call(GLOBAL_MEMORY_STATUS_EX, &args![status]).u32() != 0;
        ok && e.mem.u64(status.addr() + 0x20) < 0x2000_0000
    });
    if low_on_memory {
        e.call(
            MESSAGE_BOX_A,
            &args![0u32, NOT_ENOUGH_MEMORY_TEXT, MEMORY_ERROR_CAPTION, 0x10u32],
        );
        e.call(EXIT, &args![0u32]);
    }
    e.call(MEMORY_MANAGER_INITIALIZE, &args![this]);
    e.call(ALLOCATOR_START_UP, &args![this]);
    e.call(SCRAP_HEAP_SET_UP, &[]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00aa3880, memory_manager_construct(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine(avail_page_file: u64) -> Engine {
        let mut e = Engine::new();
        for addr in [
            SCRAP_HEAP_MAP_CONSTRUCT,
            MEMSET,
            MESSAGE_BOX_A,
            EXIT,
            MEMORY_MANAGER_INITIALIZE,
            ALLOCATOR_START_UP,
            SCRAP_HEAP_SET_UP,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register_double(GLOBAL_MEMORY_STATUS_EX, move |e, a| {
            assert_eq!(e.mem.u32(a[0]), 0x40);
            e.mem.set_u64(a[0] + 0x20, avail_page_file);
            1u32.into_ret()
        });
        e
    }

    fn called(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn constructs_the_fields_and_starts_up() {
        let mut e = engine(0x1_0000_0000);
        let this = e.mem.alloc(0x140);
        for i in 0..0x50 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        let back = e.call(0x00aa_3880, &args![this]).u32();
        assert_eq!(back, this);
        assert_eq!(e.mem.u8(this), 0);
        assert_eq!(e.mem.u16(this + 2), 0);
        assert_eq!(e.mem.u16(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0x114), SCRAP_HEAP_MAP_VTABLE);
        assert_eq!(e.mem.u32(this + 0x124), 0);
        assert_eq!(e.mem.u8(this + 0x128), 1);
        assert_eq!(e.mem.u8(this + 0x129), 1);
        assert_eq!(e.mem.u32(this + 0x12c), 0);
        assert_eq!(e.mem.u32(this + 0x134), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            called(&log, SCRAP_HEAP_MAP_CONSTRUCT),
            vec![vec![this + 0x114, 0x25]]
        );
        assert_eq!(called(&log, MEMSET), vec![vec![this + 0xc, 0, 0x108]]);
        assert!(called(&log, MESSAGE_BOX_A).is_empty());
        assert!(called(&log, EXIT).is_empty());
        let order: Vec<u32> = log
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| {
                [
                    MEMORY_MANAGER_INITIALIZE,
                    ALLOCATOR_START_UP,
                    SCRAP_HEAP_SET_UP,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                MEMORY_MANAGER_INITIALIZE,
                ALLOCATOR_START_UP,
                SCRAP_HEAP_SET_UP
            ]
        );
        assert_eq!(called(&log, ALLOCATOR_START_UP), vec![vec![this]]);
    }

    #[test]
    fn low_page_file_shows_the_message_and_exits() {
        for (avail, warns) in [(0x1fff_ffffu64, true), (0x2000_0000, false)] {
            let mut e = engine(avail);
            let this = e.mem.alloc(0x140);
            e.call_log = Some(vec![]);
            e.call(0x00aa_3880, &args![this]);
            let log = e.call_log.take().unwrap();
            if warns {
                assert_eq!(
                    called(&log, MESSAGE_BOX_A),
                    vec![vec![0, NOT_ENOUGH_MEMORY_TEXT, MEMORY_ERROR_CAPTION, 0x10]]
                );
                assert_eq!(called(&log, EXIT), vec![vec![0]]);
            } else {
                assert!(called(&log, EXIT).is_empty());
            }
        }
        // A failed GlobalMemoryStatusEx never warns.
        let mut e = engine(0);
        e.register(GLOBAL_MEMORY_STATUS_EX, |_, _| 0u32.into_ret());
        let this = e.mem.alloc(0x140);
        e.call_log = Some(vec![]);
        e.call(0x00aa_3880, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(called(&log, MESSAGE_BOX_A).is_empty());
    }
}
