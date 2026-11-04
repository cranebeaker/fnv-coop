//! `fallout/misc/appmemory_xenon.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The game's memory layout: which heaps `MemoryManager` makes. The
//! manager's own unit is `BSCore/memorymanager.cpp`; its fields are written
//! at their PC offsets (as `00aa3880` does): `iNumHeaps` +2,
//! `iNumPhysicalHeaps` +4, `ppHeaps` +8, `pHeapsByContextA` (66 pointers)
//! from +0xc.

#[allow(unused_imports)]
use crate::prelude::*;

/// Fills `count` bytes at `dest` with `value` (`cdecl(dest, value,
/// count)`, `00403d30`).
const FILL_BYTES: u32 = 0x0040_3d30;
/// `operator new(size)` (cdecl).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// The 0x20f4-byte heap's constructor (`thiscall(heap, size, ?, name,
/// 0)`, `BSCore` `00aa78a0`).
const HEAP_CONSTRUCT: u32 = 0x00aa_78a0;
/// The 0x40-byte heap's constructor (`thiscall(heap, size, name)`,
/// `00aa7400`).
const SMALL_HEAP_CONSTRUCT: u32 = 0x00aa_7400;
/// `"Default Heap"`, `"Static Heap"`, `"File Heap"`.
const DEFAULT_HEAP_NAME: u32 = 0x0108_24fc;
const STATIC_HEAP_NAME: u32 = 0x0108_24f0;
const FILE_HEAP_NAME: u32 = 0x0108_24e4;

/// `new` of `size` bytes, then `construct` on it unless it is null (the
/// result, or 0).
fn new_then(e: &mut Engine, size: u32, construct: impl FnOnce(&mut Engine, u32) -> u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        0
    } else {
        construct(e, block)
    }
}

// Translated from 00866e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MemoryManager::SpecifyMemoryLayout` (Xbox PDB): clears the 0x108 bytes
/// of `pHeapsByContextA`, sets `iNumHeaps` to 3 and allocates `ppHeaps`
/// (`iNumHeaps * 4` bytes, all ones when that overflows), then makes three
/// heaps: "Default Heap" (0x20f4 bytes, `00aa78a0(0xc800000, 0x5500000,
/// name, 0)`), the default for context slot 0x41 (+0x110); "Static Heap"
/// (0x40 bytes, `00aa7400(0x183800, name)`) for context slots 0, 3 and 9
/// (+0xc, +0x18, +0x30); "File Heap" (`00aa78a0(0x4000000, 0x200000,
/// name, 0)`) for slot 0x17 (+0x68). Ends with `iNumPhysicalHeaps` = 0.
// Translation incomplete: 00866e00 - the FS:[0] exception frame (frees each new block if its constructor throws) is not translated
pub fn memory_manager_specify_memory_layout(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    e.call(FILL_BYTES, &args![at + 0xc, 0u32, 0x108u32]);
    e.mem.set_u16(at + 2, 3);
    let count = e.mem.u16(at + 2) as u32;
    // `MUL` by 4; `SETO`/`NEG`/`OR` make an overflowing size all ones.
    let size = count.checked_mul(4).unwrap_or(u32::MAX);
    let heaps = e.call(OPERATOR_NEW, &args![size]).u32();
    e.mem.set_u32(at + 8, heaps);

    let default_heap = new_then(e, 0x20f4, |e, block| {
        e.call(
            HEAP_CONSTRUCT,
            &args![
                block,
                0x0c80_0000u32,
                0x0550_0000u32,
                DEFAULT_HEAP_NAME,
                0u32
            ],
        )
        .u32()
    });
    let heaps = e.mem.u32(at + 8);
    e.mem.set_u32(heaps, default_heap);
    let heaps = e.mem.u32(at + 8);
    let first = e.mem.u32(heaps);
    e.mem.set_u32(at + 0x110, first);

    let static_heap = new_then(e, 0x40, |e, block| {
        e.call(
            SMALL_HEAP_CONSTRUCT,
            &args![block, 0x0018_3800u32, STATIC_HEAP_NAME],
        )
        .u32()
    });
    let heaps = e.mem.u32(at + 8);
    e.mem.set_u32(heaps + 4, static_heap);
    for slot in [0xc, 0x30, 0x18] {
        let heaps = e.mem.u32(at + 8);
        let second = e.mem.u32(heaps + 4);
        e.mem.set_u32(at + slot, second);
    }

    let file_heap = new_then(e, 0x20f4, |e, block| {
        e.call(
            HEAP_CONSTRUCT,
            &args![block, 0x0400_0000u32, 0x0020_0000u32, FILE_HEAP_NAME, 0u32],
        )
        .u32()
    });
    let heaps = e.mem.u32(at + 8);
    e.mem.set_u32(heaps + 8, file_heap);
    let heaps = e.mem.u32(at + 8);
    let third = e.mem.u32(heaps + 8);
    e.mem.set_u32(at + 0x68, third);
    e.mem.set_u16(at + 4, 0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x00866e00,
        memory_manager_specify_memory_layout(Ptr)
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_three_heaps_and_fills_the_context_slots() {
        let mut e = Engine::new();
        e.register(FILL_BYTES, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        // The constructors answer their block.
        e.register(HEAP_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SMALL_HEAP_CONSTRUCT, |_, a| a[0].into_ret());
        let this = e.mem.alloc(0x140);
        e.mem.set_u16(this + 4, 7);
        e.call_log = Some(vec![]);
        e.call(0x0086_6e00, &args![this]);
        let log = e.call_log.take().unwrap();
        let news: Vec<u32> = log
            .iter()
            .filter(|(a, _)| *a == OPERATOR_NEW)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(news, vec![12, 0x20f4, 0x40, 0x20f4]);
        let fill = log.iter().find(|(a, _)| *a == FILL_BYTES).unwrap();
        assert_eq!(fill.1, vec![this + 0xc, 0, 0x108]);
        let heaps: Vec<&Vec<u32>> = log
            .iter()
            .filter(|(a, _)| *a == HEAP_CONSTRUCT)
            .map(|(_, w)| w)
            .collect();
        assert_eq!(
            heaps[0][1..],
            [0x0c80_0000, 0x0550_0000, DEFAULT_HEAP_NAME, 0]
        );
        assert_eq!(heaps[1][1..], [0x0400_0000, 0x0020_0000, FILE_HEAP_NAME, 0]);
        let small = log
            .iter()
            .find(|(a, _)| *a == SMALL_HEAP_CONSTRUCT)
            .unwrap();
        assert_eq!(small.1[1..], [0x0018_3800, STATIC_HEAP_NAME]);

        assert_eq!(e.mem.u16(this + 2), 3);
        assert_eq!(e.mem.u16(this + 4), 0);
        let table = e.mem.u32(this + 8);
        let (default_heap, static_heap, file_heap) = (heaps[0][0], small.1[0], heaps[1][0]);
        assert_eq!(e.mem.u32(table), default_heap);
        assert_eq!(e.mem.u32(table + 4), static_heap);
        assert_eq!(e.mem.u32(table + 8), file_heap);
        assert_eq!(e.mem.u32(this + 0x110), default_heap);
        for slot in [0xc, 0x18, 0x30] {
            assert_eq!(e.mem.u32(this + slot), static_heap);
        }
        assert_eq!(e.mem.u32(this + 0x68), file_heap);
    }

    #[test]
    fn a_failed_allocation_leaves_a_null_heap() {
        let mut e = Engine::new();
        e.register(FILL_BYTES, |_, _| Ret::default());
        // Only the table allocation succeeds.
        e.register(OPERATOR_NEW, |e, a| {
            if a[0] == 12 {
                e.mem.alloc(12).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let this = e.mem.alloc(0x140);
        e.mem.set_u32(this + 0x110, 5);
        e.call_log = Some(vec![]);
        e.call(0x0086_6e00, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == HEAP_CONSTRUCT));
        assert_eq!(e.mem.u32(this + 0x110), 0);
        let table = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(table + 4), 0);
    }
}
