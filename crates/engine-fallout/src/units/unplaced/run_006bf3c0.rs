//! `(unplaced)/run_006bf3c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `ScrapHeap::Allocate` (`thiscall` on the scrap heap at `this + 0xC`, `(size,
/// alignment)`), the alignment word it is given (`010a2720`), the placement step `006e6da0`
/// (`cdecl`, `(size, block)`, answers the block or null), the record test `006bf1d0`
/// (`thiscall`, `(&record)`), and the edge's member constructors `00633c90` (`thiscall`,
/// `(0)`) and `006240d0` (`thiscall`, no arguments).
const SCRAP_HEAP_ALLOCATE: u32 = 0x00aa_54a0;
const SCRAP_ALIGNMENT: u32 = 0x010a_2720;
const PLACE_006E6DA0: u32 = 0x006e_6da0;
const RECORD_TEST_006BF1D0: u32 = 0x006b_f1d0;
const MEMBER_CONSTRUCTOR_00633C90: u32 = 0x0063_3c90;
const MEMBER_CONSTRUCTOR_006240D0: u32 = 0x0062_40d0;

/// A `size`-byte block from the scrap heap at `this + 0xC` and the placement step on it:
/// (block, placed).
fn scrap_block(e: &mut Engine, this: Ptr, size: u32) -> (u32, u32) {
    let alignment = e.global::<u32>(SCRAP_ALIGNMENT);
    let block = e
        .call(
            SCRAP_HEAP_ALLOCATE,
            &args![this.addr() + 0xc, size, alignment],
        )
        .u32();
    let placed = e.call(PLACE_006E6DA0, &args![size, block]).u32();
    (block, placed)
}

// Translated from 006bf480 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 8-byte vertex record constructor (unnamed in the map; `thiscall`, `(index, flag 2,
/// flag 3, flag 4)`): the 16-bit index, the three flag bytes at +2..+4, and the bytes at +5
/// and +6 cleared. Answers `this`.
pub fn fn_006bf480(
    e: &mut Engine,
    this: Ptr,
    index: u16,
    flag_2: u8,
    flag_3: u8,
    flag_4: u8,
) -> Ptr {
    let at = this.addr();
    e.mem.set_u16(at, index);
    e.mem.set_u8(at + 2, flag_2);
    e.mem.set_u8(at + 3, flag_3);
    e.mem.set_u8(at + 4, flag_4);
    e.mem.set_u8(at + 5, 0);
    e.mem.set_u8(at + 6, 0);
    this
}

// Translated from 006bf3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `(index, flag 2, flag 3, flag 4)`): an 8-byte block from
/// the scrap heap at `this + 0xC`, constructed as a vertex record (`fn_006bf480`) when the
/// placement step answers it, and its byte at +5 set to `006bf1d0`'s answer for it. Answers
/// the block.
// Translation incomplete: 006bf3c0 - SEH exception unwinding is unsupported.
pub fn fn_006bf3c0(
    e: &mut Engine,
    this: Ptr,
    index: u16,
    flag_2: u8,
    flag_3: u8,
    flag_4: u8,
) -> Ptr {
    let (block, placed) = scrap_block(e, this, 8);
    if placed != 0 {
        fn_006bf480(e, Ptr::new(placed), index, flag_2, flag_3, flag_4);
    }
    let free = e.call(RECORD_TEST_006BF1D0, &args![this, block]).u8();
    e.mem.set_u8(block.wrapping_add(5), free);
    Ptr::new(block)
}

// Translated from 006bf570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x30-byte edge record constructor (unnamed in the map; `thiscall`, three words): the
/// members at +0x1C (`00633c90(0)`) and +0x20 (`006240d0`) constructed, the first two words
/// at +0 and +4, the four 16-bit slots at +0x08..+0x0E 0xFFFF, the third word at +0x10, the
/// 16-bit +0x16 and the bytes +0x18, +0x19 cleared, +0x14 0xFFFF, +0x20 cleared, +0x24
/// 0xFFFF, +0x28 -1, +0x2C 0xFFFF and the byte +0x2E cleared. Answers `this`.
// Translation incomplete: 006bf570 - SEH exception unwinding is unsupported.
pub fn fn_006bf570(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) -> Ptr {
    let at = this.addr();
    e.call(MEMBER_CONSTRUCTOR_00633C90, &args![at + 0x1c, 0u32]);
    e.call(MEMBER_CONSTRUCTOR_006240D0, &args![at + 0x20]);
    e.mem.set_u32(at, first);
    e.mem.set_u32(at + 4, second);
    for offset in [8u32, 0xa, 0xc, 0xe] {
        e.mem.set_u16(at + offset, 0xffff);
    }
    e.mem.set_u32(at + 0x10, third);
    e.mem.set_u16(at + 0x16, 0);
    e.mem.set_u8(at + 0x18, 0);
    e.mem.set_u8(at + 0x19, 0);
    e.mem.set_u16(at + 0x14, 0xffff);
    e.mem.set_u32(at + 0x20, 0);
    e.mem.set_u16(at + 0x24, 0xffff);
    e.mem.set_u32(at + 0x28, u32::MAX);
    e.mem.set_u16(at + 0x2c, 0xffff);
    e.mem.set_u8(at + 0x2e, 0);
    this
}

// Translated from 006bf4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, three words): a 0x30-byte block from the scrap heap at
/// `this + 0xC`, constructed as an edge record (`fn_006bf570`) when the placement step
/// answers it. Answers the block.
// Translation incomplete: 006bf4d0 - SEH exception unwinding is unsupported.
pub fn fn_006bf4d0(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) -> Ptr {
    let (block, placed) = scrap_block(e, this, 0x30);
    if placed != 0 {
        fn_006bf570(e, Ptr::new(placed), first, second, third);
    }
    Ptr::new(block)
}

// Translated from 006bf670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`thiscall`, `RET 4`; called from `UndoNavMeshFromObstacleUndoData`
/// and others): the address of element `index` of a `BSSimpleArray<PortalEdgeSwap,1024>`
/// (Xbox PDB; 0x1C-byte elements, `pBuffer` at +4): `pBuffer + index * 0x1C`.
pub fn fn_006bf670(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    index
        .wrapping_mul(0x1c)
        .wrapping_add(e.mem.u32(this.addr() + 4))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x006bf480, fn_006bf480(Ptr, u16, u8, u8, u8) -> Ptr),
        entry!(0x006bf3c0, fn_006bf3c0(Ptr, u16, u8, u8, u8) -> Ptr),
        entry!(0x006bf570, fn_006bf570(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006bf4d0, fn_006bf4d0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x006b_f670, fn_006bf670(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn scrap_engine() -> Engine {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(SCRAP_ALIGNMENT, 4);
        e.set_global::<u32>(SCRAP_ALIGNMENT, 4);
        e.register(SCRAP_HEAP_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(PLACE_006E6DA0, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        e
    }

    #[test]
    fn vertex_record_constructor_sets_index_and_flags() {
        let mut e = Engine::new();
        let r = e.mem.alloc(8);
        e.mem.set_u8(r + 5, 9);
        e.mem.set_u8(r + 6, 9);
        assert_eq!(
            e.call(0x006bf480, &args![r, 0x1234u32, 1u32, 2u32, 3u32])
                .u32(),
            r
        );
        assert_eq!(e.mem.u16(r), 0x1234);
        assert_eq!([2u32, 3, 4, 5, 6].map(|o| e.mem.u8(r + o)), [1, 2, 3, 0, 0]);
    }

    #[test]
    fn new_vertex_record_comes_from_the_scrap_heap() {
        let mut e = scrap_engine();
        e.register(RECORD_TEST_006BF1D0, |_, _| Ret {
            eax: 0x101,
            ..Ret::default()
        });
        let this = e.mem.alloc(0x40);
        let r = e
            .call(0x006bf3c0, &args![this, 7u32, 1u32, 0u32, 1u32])
            .u32();
        assert_eq!(e.mem.u16(r), 7);
        assert_eq!(e.mem.u8(r + 4), 1);
        assert_eq!(e.mem.u8(r + 5), 1);
        assert_eq!(
            calls_at(&e, SCRAP_HEAP_ALLOCATE),
            vec![vec![this + 0xc, 8, 4]]
        );
        assert_eq!(calls_at(&e, RECORD_TEST_006BF1D0), vec![vec![this, r]]);
        // A refused placement leaves the block unconstructed.
        e.register(PLACE_006E6DA0, |_, _| Ret::default());
        let r = e
            .call(0x006bf3c0, &args![this, 7u32, 1u32, 0u32, 1u32])
            .u32();
        assert_eq!(e.mem.u16(r), 0);
        assert_eq!(e.mem.u8(r + 5), 1);
    }

    #[test]
    fn edge_record_constructor_and_allocation() {
        let mut e = scrap_engine();
        e.register(MEMBER_CONSTRUCTOR_00633C90, |_, _| Ret::default());
        e.register(MEMBER_CONSTRUCTOR_006240D0, |_, _| Ret::default());
        let this = e.mem.alloc(0x40);
        let r = e
            .call(0x006bf4d0, &args![this, 0x11u32, 0x22u32, 0x33u32])
            .u32();
        assert_eq!(
            calls_at(&e, SCRAP_HEAP_ALLOCATE),
            vec![vec![this + 0xc, 0x30, 4]]
        );
        assert_eq!(
            [0u32, 4, 0x10].map(|o| e.mem.u32(r + o)),
            [0x11, 0x22, 0x33]
        );
        assert_eq!(
            [8u32, 0xe, 0x14, 0x24, 0x2c].map(|o| e.mem.u16(r + o)),
            [0xffff; 5]
        );
        assert_eq!(e.mem.u32(r + 0x28), u32::MAX);
        assert_eq!(
            calls_at(&e, MEMBER_CONSTRUCTOR_00633C90),
            vec![vec![r + 0x1c, 0]]
        );
        assert_eq!(
            calls_at(&e, MEMBER_CONSTRUCTOR_006240D0),
            vec![vec![r + 0x20]]
        );
        e.register(PLACE_006E6DA0, |_, _| Ret::default());
        let r = e
            .call(0x006bf4d0, &args![this, 0x11u32, 0x22u32, 0x33u32])
            .u32();
        assert_eq!(e.mem.u32(r), 0);
        assert_eq!(calls_at(&e, MEMBER_CONSTRUCTOR_006240D0).len(), 1);
        // The constructor on its own answers its block.
        let block = e.mem.alloc(0x30);
        assert_eq!(
            e.call(0x006bf570, &args![block, 1u32, 2u32, 3u32]).u32(),
            block
        );
    }
}

#[cfg(test)]
mod n076_tests {
    use super::*;

    #[test]
    fn element_address_steps_by_0x1c() {
        let mut e = Engine::new();
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x4000);
        assert_eq!(e.call(0x006b_f670, &args![array, 3u32]).u32(), 0x4054);
    }
}
