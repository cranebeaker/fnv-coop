//! `NiSystem/bsniallocator.cpp` (Xbox PDB source unit), subsystem `NiSystem`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00aa2240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNiAllocator::Allocate` (Xbox PDB). Kind 7 reserves the byte alignment
/// in addition to the requested size, adjusts the returned address using
/// byte arithmetic and stores that adjustment immediately before it. Other
/// kinds forward the size and final three context words to `00aa4030`.
/// Both paths reload the size after allocation and zero that many bytes.
#[allow(clippy::too_many_arguments)] // Native receiver plus seven stack words.
pub fn allocate(
    e: &mut Engine,
    _this: Ptr,
    size: Ptr<u32>,
    alignment: Ptr<u8>,
    kind: u32,
    _unused_4: u32,
    context_1: u32,
    context_2: u32,
    context_3: u32,
) -> Ptr {
    let buffer = if kind == 7 {
        let alignment = e.mem.u8(alignment.addr());
        let bytes = e.mem.u32(size.addr()).wrapping_add(alignment as u32);
        let raw = e.call(0x00aa_3e40, &args![0x011f_6238u32, bytes]).u32();
        let adjustment = alignment.wrapping_sub(alignment.wrapping_sub(1) & raw as u8);
        let aligned = raw.wrapping_add(adjustment as u32);
        e.mem.set_u8(aligned.wrapping_sub(1), adjustment);
        aligned
    } else {
        let bytes = e.mem.u32(size.addr());
        e.call(
            0x00aa_4030,
            &args![0x011f_6238u32, bytes, context_1, context_2, context_3],
        )
        .u32()
    };
    let bytes = e.mem.u32(size.addr());
    e.call(0x00ec_61c0, &args![buffer, 0u32, bytes]);
    Ptr::new(buffer)
}

// Translated from 00aa22b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNiAllocator::Deallocate` (engine map; thiscall, `RET 0xc`, `this` and the third word
/// unread): frees `memory` through the memory manager (`00aa4060` on `011f6238`); for kind 10
/// the block starts the byte stored just before `memory` earlier (and a null `memory` frees
/// nothing).
pub fn deallocate(e: &mut Engine, _this: Ptr, memory: Ptr, kind: u32, _unused_3: u32) {
    if kind == 10 {
        if memory.addr() == 0 {
            return;
        }
        let adjustment = u32::from(e.mem.u8(memory.addr().wrapping_sub(1)));
        let block = memory.addr().wrapping_sub(adjustment);
        e.call(0x00aa_4060, &args![0x011f_6238u32, block]);
        return;
    }
    e.call(0x00aa_4060, &args![0x011f_6238u32, memory]);
}

/// The memory manager singleton and its functions (thiscall on `011f6238`): `Allocate`
/// (`00aa3e40`: size), `Deallocate` (`00aa4060`: block), the reallocation (`00aa4150`:
/// block, size), the tracked allocation (`00aa4030`: size and three context words) and the
/// tracked reallocation (`00aa4200`: block, size and three context words).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
const MEMORY_REALLOCATE: u32 = 0x00aa_4150;
const MEMORY_ALLOCATE_TRACKED: u32 = 0x00aa_4030;
const MEMORY_REALLOCATE_TRACKED: u32 = 0x00aa_4200;
/// `memset` (`00ec61c0`, cdecl).
const MEMSET: u32 = 0x00ec_61c0;

/// The aligned block of `raw`: it moves up by `alignment - ((alignment - 1) & raw)` (byte
/// arithmetic), and that adjustment is stored in the byte before it.
fn align_block(e: &mut Engine, raw: u32, alignment: u8) -> u32 {
    let adjustment = alignment.wrapping_sub(alignment.wrapping_sub(1) & raw as u8);
    let aligned = raw.wrapping_add(u32::from(adjustment));
    e.mem.set_u8(aligned.wrapping_sub(1), adjustment);
    aligned
}

/// The reallocation `Reallocate` (kind 8) and `ReallocateExternal` (kind 0xE) share: for the
/// aligned kind, a null `memory` answers 0, otherwise the block (`memory` less its stored
/// adjustment) is reallocated (`00aa4150`) to `size + alignment` bytes and aligned again;
/// other kinds go to the tracked reallocation (`00aa4200`) with `*size` and the three
/// context words, whose answer is passed on.
#[allow(clippy::too_many_arguments)] // The block, its sizes, the kind and three context words.
fn reallocate_kind(
    e: &mut Engine,
    aligned_kind: u32,
    memory: u32,
    size: Ptr<u32>,
    alignment: Ptr<u8>,
    kind: u32,
    context: [u32; 3],
) -> u32 {
    if kind != aligned_kind {
        let bytes = e.mem.u32(size.addr());
        return e
            .call(
                MEMORY_REALLOCATE_TRACKED,
                &args![
                    MEMORY_MANAGER,
                    memory,
                    bytes,
                    context[0],
                    context[1],
                    context[2]
                ],
            )
            .u32();
    }
    let bytes = e.mem.u32(size.addr());
    let alignment = e.mem.u8(alignment.addr());
    if memory == 0 {
        return 0;
    }
    let block = memory.wrapping_sub(u32::from(e.mem.u8(memory.wrapping_sub(1))));
    let raw = e
        .call(
            MEMORY_REALLOCATE,
            &args![
                MEMORY_MANAGER,
                block,
                u32::from(alignment).wrapping_add(bytes)
            ],
        )
        .u32();
    align_block(e, raw, alignment)
}

// Translated from 00aa22f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNiAllocator::Reallocate` (engine map; thiscall, nine stack words, `this` and the fifth
/// and sixth unread): kind 8 is the aligned reallocation, other kinds the tracked one (see
/// [`reallocate_kind`]).
#[allow(clippy::too_many_arguments)] // Native receiver plus nine stack words.
pub fn bs_ni_allocator_reallocate(
    e: &mut Engine,
    _this: Ptr,
    memory: u32,
    size: Ptr<u32>,
    alignment: Ptr<u8>,
    kind: u32,
    _unused_5: u32,
    _unused_6: u32,
    context_1: u32,
    context_2: u32,
    context_3: u32,
) -> u32 {
    reallocate_kind(
        e,
        8,
        memory,
        size,
        alignment,
        kind,
        [context_1, context_2, context_3],
    )
}

// Translated from 00aa2370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNiAllocator::AllocateExternal` (engine map; thiscall, seven stack words, `this` and the
/// fourth unread): kind 0xD reserves the alignment besides the size (`Allocate`) and aligns
/// the block (the adjustment in the byte before it); other kinds use the tracked allocation
/// (`00aa4030`) with the last three words. Both clear `*size` bytes of the answer.
#[allow(clippy::too_many_arguments)] // Native receiver plus seven stack words.
pub fn bs_ni_allocator_allocate_external(
    e: &mut Engine,
    _this: Ptr,
    size: Ptr<u32>,
    alignment: Ptr<u8>,
    kind: u32,
    _unused_4: u32,
    context_1: u32,
    context_2: u32,
    context_3: u32,
) -> u32 {
    let buffer = if kind == 0xd {
        let alignment = e.mem.u8(alignment.addr());
        let bytes = u32::from(alignment).wrapping_add(e.mem.u32(size.addr()));
        let raw = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
        align_block(e, raw, alignment)
    } else {
        let bytes = e.mem.u32(size.addr());
        e.call(
            MEMORY_ALLOCATE_TRACKED,
            &args![MEMORY_MANAGER, bytes, context_1, context_2, context_3],
        )
        .u32()
    };
    let bytes = e.mem.u32(size.addr());
    e.call(MEMSET, &args![buffer, 0u32, bytes]);
    buffer
}

// Translated from 00aa23e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNiAllocator::DeallocateExternal` (engine map; thiscall, three stack words, `this` and
/// the third unread): frees `memory` (`Deallocate`); for kind 0x10 the block starts the
/// stored adjustment before it and a null `memory` frees nothing (answering it). The
/// memory manager's answer is passed on.
pub fn bs_ni_allocator_deallocate_external(
    e: &mut Engine,
    _this: Ptr,
    memory: u32,
    kind: u32,
    _unused_3: u32,
) -> u32 {
    if kind == 0x10 {
        if memory == 0 {
            return memory;
        }
        let block = memory.wrapping_sub(u32::from(e.mem.u8(memory.wrapping_sub(1))));
        return e
            .call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, block])
            .u32();
    }
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, memory])
        .u32()
}

// Translated from 00aa2420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSNiAllocator::ReallocateExternal` (engine map; thiscall, nine stack words, `this` and
/// the fifth and sixth unread): as [`bs_ni_allocator_reallocate`] with kind 0xE aligned.
#[allow(clippy::too_many_arguments)] // Native receiver plus nine stack words.
pub fn bs_ni_allocator_reallocate_external(
    e: &mut Engine,
    _this: Ptr,
    memory: u32,
    size: Ptr<u32>,
    alignment: Ptr<u8>,
    kind: u32,
    _unused_5: u32,
    _unused_6: u32,
    context_1: u32,
    context_2: u32,
    context_3: u32,
) -> u32 {
    reallocate_kind(
        e,
        0xe,
        memory,
        size,
        alignment,
        kind,
        [context_1, context_2, context_3],
    )
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00aa_22f0,
            bs_ni_allocator_reallocate(
                Ptr,
                u32,
                Ptr<u32>,
                Ptr<u8>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> u32
        ),
        entry!(
            0x00aa_2370,
            bs_ni_allocator_allocate_external(
                Ptr,
                Ptr<u32>,
                Ptr<u8>,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> u32
        ),
        entry!(
            0x00aa_23e0,
            bs_ni_allocator_deallocate_external(Ptr, u32, u32, u32) -> u32
        ),
        entry!(
            0x00aa_2420,
            bs_ni_allocator_reallocate_external(
                Ptr,
                u32,
                Ptr<u32>,
                Ptr<u8>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
            ) -> u32
        ),
        entry!(
            0x00aa2240,
            allocate(Ptr, Ptr<u32>, Ptr<u8>, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00aa_22b0, deallocate(Ptr, Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordinary_allocation_forwards_context_and_reloads_size_before_zeroing() {
        let mut e = Engine::new();
        let size = e.mem.alloc(4);
        let buffer = e.mem.alloc(16);
        e.mem.set_u32(size, 4);
        e.mem.write(buffer, &[0xcc; 16]);
        e.register_double(0x00aa4030, move |e, args| {
            assert_eq!(args, &[0x011f6238, 4, 12, 13, 14]);
            e.mem.set_u32(size, 8);
            buffer.into_ret()
        });
        // Alignment is unread outside kind 7; null is intentional.
        let result = e.call(0x00aa2240, &[0, size, 0, 5, 11, 12, 13, 14]).u32();
        assert_eq!(result, buffer);
        assert_eq!(e.mem.bytes(buffer, 8), [0; 8]);
        assert_eq!(e.mem.bytes(buffer + 8, 8), [0xcc; 8]);
    }

    #[test]
    fn aligned_allocation_uses_low_alignment_byte_and_preserves_guards() {
        for raw_offset in [0u32, 3, 8] {
            let mut e = Engine::new();
            let size = e.mem.alloc(4);
            let alignment = e.mem.alloc(4);
            let backing = e.mem.alloc(64);
            e.mem.set_u32(size, 4);
            e.mem.set_u32(alignment, 0xaaaaaa10);
            e.mem.write(backing, &[0xcc; 64]);
            let raw = backing + raw_offset;
            e.register_double(0x00aa3e40, move |e, args| {
                assert_eq!(args, &[0x011f6238, 20]);
                // PC caches alignment before this call, but reloads size.
                e.mem.set_u8(alignment, 8);
                e.mem.set_u32(size, 5);
                raw.into_ret()
            });
            let result = e
                .call(0x00aa2240, &[0, size, alignment, 7, 11, 12, 13, 14])
                .u32();
            assert_eq!(result & 15, 0);
            assert!(result > raw);
            let adjustment = result - raw;
            assert_eq!(e.mem.u8(result - 1), adjustment as u8);
            assert_eq!(e.mem.u8(result - 2), 0xcc);
            assert_eq!(e.mem.bytes(result, 5), [0; 5]);
            assert_eq!(e.mem.u8(result + 5), 0xcc);
        }
    }
}

#[cfg(test)]
mod b004_tests {
    use super::*;

    fn frees(e: &mut Engine, memory: u32, kind: u32) -> Vec<Vec<u32>> {
        e.register(0x00aa_4060, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00aa_22b0, &args![0x1234u32, memory, kind, 0x40u32]);
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x00aa_4060)
            .map(|(_, w)| w)
            .collect()
    }

    #[test]
    fn deallocate_frees_through_the_memory_manager() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x20);
        assert_eq!(frees(&mut e, block, 3), vec![vec![0x011f_6238, block]]);
        assert_eq!(frees(&mut e, 0, 3), vec![vec![0x011f_6238, 0]]);
        // Kind 10: the adjustment byte before the memory gives the block's start.
        e.mem.set_u8(block + 5, 6);
        assert_eq!(frees(&mut e, block + 6, 10), vec![vec![0x011f_6238, block]]);
        assert!(frees(&mut e, 0, 10).is_empty());
    }
}

#[cfg(test)]
mod n083_tests {
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

    fn words(e: &mut Engine, size: u32, alignment: u8) -> (u32, u32) {
        let at = e.mem.alloc(8);
        e.mem.set_u32(at, size);
        e.mem.set_u8(at + 4, alignment);
        (at, at + 4)
    }

    #[test]
    fn reallocations_align_their_kind_and_track_the_rest() {
        for (addr, aligned) in [(0x00aa_22f0u32, 8u32), (0x00aa_2420, 0xe)] {
            let mut e = Engine::new();
            let raw = e.mem.alloc(0x80);
            e.register_double(MEMORY_REALLOCATE, move |_, _| (raw + 3).into_ret());
            e.register(MEMORY_REALLOCATE_TRACKED, |_, _| 0x7777u32.into_ret());
            let (size, alignment) = words(&mut e, 0x20, 16);
            let old = e.mem.alloc(0x40) + 8;
            e.mem.set_u8(old - 1, 5);
            e.call_log = Some(vec![]);
            let r = e
                .call(
                    addr,
                    &args![0u32, old, size, alignment, aligned, 0u32, 0u32, 1u32, 2u32, 3u32],
                )
                .u32();
            assert_eq!(
                calls_to(&e, MEMORY_REALLOCATE),
                vec![vec![MEMORY_MANAGER, old - 5, 0x30]]
            );
            let up = 16 - (15 & (raw + 3) as u8);
            assert_eq!(r, raw + 3 + u32::from(up));
            assert_eq!(e.mem.u8(r - 1), up);
            let none = e.call(
                addr,
                &args![0u32, 0u32, size, alignment, aligned, 0u32, 0u32, 1u32, 2u32, 3u32],
            );
            assert_eq!(none.u32(), 0);
            let tracked = e.call(
                addr,
                &args![0u32, old, size, alignment, 7u32, 0u32, 0u32, 1u32, 2u32, 3u32],
            );
            assert_eq!(tracked.u32(), 0x7777);
            assert_eq!(
                calls_to(&e, MEMORY_REALLOCATE_TRACKED),
                vec![vec![MEMORY_MANAGER, old, 0x20, 1, 2, 3]]
            );
        }
    }

    #[test]
    fn external_allocation_aligns_kind_13_and_clears() {
        let mut e = Engine::new();
        let raw = e.mem.alloc(0x80);
        e.register_double(MEMORY_ALLOCATE, move |_, _| (raw + 1).into_ret());
        e.register(MEMORY_ALLOCATE_TRACKED, |_, _| 0x6000u32.into_ret());
        e.register(MEMSET, |_, _| Ret::default());
        let (size, alignment) = words(&mut e, 0x20, 4);
        e.call_log = Some(vec![]);
        let r = e
            .call(
                0x00aa_2370,
                &args![0u32, size, alignment, 0xdu32, 0u32, 1u32, 2u32, 3u32],
            )
            .u32();
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x24]]
        );
        let up = 4 - (3 & (raw + 1) as u8);
        assert_eq!(r, raw + 1 + u32::from(up));
        assert_eq!(e.mem.u8(r - 1), up);
        assert_eq!(calls_to(&e, MEMSET), vec![vec![r, 0, 0x20]]);
        let r = e
            .call(
                0x00aa_2370,
                &args![0u32, size, alignment, 1u32, 0u32, 1u32, 2u32, 3u32],
            )
            .u32();
        assert_eq!(r, 0x6000);
        assert_eq!(
            calls_to(&e, MEMORY_ALLOCATE_TRACKED),
            vec![vec![MEMORY_MANAGER, 0x20, 1, 2, 3]]
        );
        assert_eq!(calls_to(&e, MEMSET)[1], [0x6000, 0, 0x20]);
    }

    #[test]
    fn external_deallocation_undoes_the_alignment() {
        let mut e = Engine::new();
        e.register(MEMORY_DEALLOCATE, |_, _| 0x11u32.into_ret());
        let block = e.mem.alloc(0x10) + 4;
        e.mem.set_u8(block - 1, 4);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_23e0, &args![0u32, block, 0x10u32, 0u32])
                .u32(),
            0x11
        );
        assert_eq!(
            e.call(0x00aa_23e0, &args![0u32, 0u32, 0x10u32, 0u32]).u32(),
            0
        );
        assert_eq!(
            e.call(0x00aa_23e0, &args![0u32, block, 2u32, 0u32]).u32(),
            0x11
        );
        assert_eq!(
            calls_to(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, block - 4], vec![MEMORY_MANAGER, block]]
        );
    }
}
