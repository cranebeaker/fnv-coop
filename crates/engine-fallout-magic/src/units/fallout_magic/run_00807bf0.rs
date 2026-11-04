//! `fallout/magic/run_00807bf0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00807d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00807d60`, which, when bit 0x8000 of the
/// half-word at `+6` is clear, calls `00807da0` with 0x18 and passes the result and the word
/// at `+0` to `00807dc0`, and returns nothing. Parameters: one word, `this`; the disassembly
/// of the function ends in a plain `RET` and passes only `ECX` on. `00807d60` computes no
/// result on either path (the skipped path loads nothing, the other ends `ADD ESP,0xC`);
/// Ghidra types it void.
pub fn fn_00807d40(e: &mut Engine, this: Ptr) {
    e.call(0x0080_7d60, &args![this]);
}

// Translated from 00807df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x4` (`SUB ECX,0x4`) and jumps to `008078e0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00807df0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0080_78e0, &args![this.addr().wrapping_sub(0x4), flags])
        .u32()
}

// Translated from 00807e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `008078e0`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00807e00(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0080_78e0, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

/// `00c85750` (no arguments): the thread's Havok memory router (`TlsGetValue` of the slot
/// at `01268108`).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `0044edb0` (`thiscall`, no stack words): the word at +0x10 of the router, its heap
/// allocator (`hkMemoryRouter::m_heap`, Xbox PDB; the engine map's name for the folded body,
/// `BaseProcess::GetCurrentProcedureIndex`, does not fit here).
const ROUTER_HEAP: u32 = 0x0044_edb0;
/// `hkMemoryAllocator::blockAlloc` (Xbox PDB vtable slot +4; one word: the size).
const SLOT_BLOCK_ALLOC: u32 = 0x4;
/// `hkMemoryAllocator::blockFree` (Xbox PDB vtable slot +8; two words: block, size).
const SLOT_BLOCK_FREE: u32 = 0x8;
/// `00c9c140`, `00c9c670`, `00c9c0f0` (`hkpentity.cpp` in the engine map; `thiscall`, one
/// stack word each).
const ENTITY_STEP_1: u32 = 0x00c9_c140;
const ENTITY_STEP_2: u32 = 0x00c9_c670;
const ENTITY_STEP_3: u32 = 0x00c9_c0f0;

/// The heap allocator of the thread's memory router.
fn router_heap(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    e.call(ROUTER_HEAP, &args![router]).u32()
}

// Translated from 00807bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): allocates `size` bytes from the heap
/// allocator of the thread's Havok memory router (vtable +4, `blockAlloc`) and returns the
/// block.
pub fn fn_00807bf0(e: &mut Engine, size: u32) -> u32 {
    let heap = router_heap(e);
    e.vcall(heap, SLOT_BLOCK_ALLOC, &args![size]).u32()
}

// Translated from 00807c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): when `block` is not null, gives it back
/// to the heap allocator of the thread's Havok memory router as a 0x10-byte block (vtable
/// +8, `blockFree`).
pub fn fn_00807c20(e: &mut Engine, block: u32) {
    if block != 0 {
        let heap = router_heap(e);
        e.vcall(heap, SLOT_BLOCK_FREE, &args![block, 0x10u32]);
    }
}

// Translated from 00807c50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00807c50 - the FS:[0] exception frame (unwinding the first header when the second one's constructor throws) is not translated
/// Unnamed in the engine map (`thiscall`, plain `RET`): initialises the two array headers
/// at `this` and `this + 8` (`00807d10`) and returns `this`.
pub fn fn_00807c50(e: &mut Engine, this: Ptr) -> Ptr {
    fn_00807d10(e, this);
    fn_00807d10(e, this.byte_add(8));
    this
}

// Translated from 00807cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, two words): calls `00c9c140` on `entity` with
/// `object`, then `00c9c670` with `object + 4` and `00c9c0f0` with `object + 8` (each 0
/// when `object` is null).
pub fn fn_00807cb0(e: &mut Engine, object: u32, entity: u32) {
    e.call(ENTITY_STEP_1, &args![entity, object]);
    let second = if object == 0 {
        0
    } else {
        object.wrapping_add(4)
    };
    e.call(ENTITY_STEP_2, &args![entity, second]);
    let third = if object == 0 {
        0
    } else {
        object.wrapping_add(8)
    };
    e.call(ENTITY_STEP_3, &args![entity, third]);
}

// Translated from 00807d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): an empty array header: the data word at +0 and
/// the half-word at +4 cleared, the half-word at +6 set to 0x8000 (the flag `00807d60`
/// tests: nothing to free). Returns `this`.
pub fn fn_00807d10(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u16(this.addr() + 4, 0);
    e.mem.set_u16(this.addr() + 6, 0x8000);
    this
}

// Translated from 00807d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): when bit 0x8000 of the half-word at +6 is clear,
/// frees the array's data (`00807dc0` with the word at +0 and the capacity `00807da0`
/// answers). The exe pushes 0x18 before calling `00807da0`, which takes no stack word; the
/// word stays on the stack as the third word of the `cdecl` call to `00807dc0` (its caller
/// cleans 0xC bytes), so it is passed there.
pub fn fn_00807d60(e: &mut Engine, this: Ptr) {
    if e.mem.u16(this.addr() + 6) & 0x8000 == 0 {
        let capacity = fn_00807da0(e, this);
        let data = e.mem.u32(this.addr());
        fn_00807dc0(e, data, capacity, 0x18);
    }
}

// Translated from 00807da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the capacity, the low 14 bits of the half-word
/// at +6.
pub fn fn_00807da0(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u16(this.addr() + 6) & 0x3fff)
}

// Translated from 00807dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`; its one caller `00807d60` cleans three words, the
/// third of which it never reads): gives `data` back to the heap allocator of the thread's
/// Havok memory router as `count * 4` bytes (vtable +8, `blockFree`).
pub fn fn_00807dc0(e: &mut Engine, data: u32, count: u32, _unused_2: u32) {
    let heap = router_heap(e);
    e.vcall(heap, SLOT_BLOCK_FREE, &args![data, count << 2]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_7bf0, fn_00807bf0(u32) -> u32),
        entry!(0x0080_7c20, fn_00807c20(u32)),
        entry!(0x0080_7c50, fn_00807c50(Ptr) -> Ptr),
        entry!(0x0080_7cb0, fn_00807cb0(u32, u32)),
        entry!(0x0080_7d10, fn_00807d10(Ptr) -> Ptr),
        entry!(0x0080_7d60, fn_00807d60(Ptr)),
        entry!(0x0080_7da0, fn_00807da0(Ptr) -> u32),
        entry!(0x0080_7dc0, fn_00807dc0(u32, u32, u32)),
        entry!(0x0080_7d40, fn_00807d40(Ptr)),
        entry!(0x0080_7df0, fn_00807df0(Ptr, u32) -> u32),
        entry!(0x0080_7e00, fn_00807e00(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod thunk_tests {
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
    fn fwd_a03_00807d40_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0080_7d60, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0080_7d40, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0080_7d60), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00807df0() {
        assert_eq!(check(0x0080_7df0, 0x0080_78e0, 0x4), 0xdead_beef);
    }

    #[test]
    fn thunk_00807e00() {
        assert_eq!(check(0x0080_7e00, 0x0080_78e0, 0x8), 0xdead_beef);
    }
}

#[cfg(test)]
mod n049_tests {
    use super::*;

    const SLOT_DOUBLE: u32 = 0x0e20_0000;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A router whose heap (+0x10) is an allocator with `SLOT_DOUBLE` in every slot.
    fn engine_with_heap() -> (Engine, u32) {
        let mut e = Engine::new();
        let vtable = e.mem.alloc(0x10);
        for slot in 0..4 {
            e.mem.set_u32(vtable + 4 * slot, SLOT_DOUBLE);
        }
        let heap = e.mem.alloc(8);
        e.mem.set_u32(heap, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, heap);
        e.register_double(MEMORY_ROUTER, move |_, _| router.into_ret());
        e.register(ROUTER_HEAP, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(SLOT_DOUBLE, |_, _| 0x7700u32.into_ret());
        e.call_log = Some(vec![]);
        (e, heap)
    }

    #[test]
    fn fn_00807bf0_allocates_from_the_router_heap() {
        let (mut e, heap) = engine_with_heap();
        assert_eq!(e.call(0x0080_7bf0, &[0x30]).u32(), 0x7700);
        assert_eq!(calls(&e, SLOT_DOUBLE), vec![vec![heap, 0x30]]);
    }

    #[test]
    fn fn_00807c20_frees_only_a_block() {
        let (mut e, heap) = engine_with_heap();
        e.call(0x0080_7c20, &[0]);
        assert!(calls(&e, SLOT_DOUBLE).is_empty());
        e.call(0x0080_7c20, &[0x4440]);
        assert_eq!(calls(&e, SLOT_DOUBLE), vec![vec![heap, 0x4440, 0x10]]);
    }

    #[test]
    fn fn_00807c50_initialises_both_headers() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        for i in 0..4 {
            e.mem.set_u32(this + 4 * i, 0xffff_ffff);
        }
        assert_eq!(e.call(0x0080_7c50, &[this]).u32(), this);
        for base in [this, this + 8] {
            assert_eq!(e.mem.u32(base), 0);
            assert_eq!(e.mem.u16(base + 4), 0);
            assert_eq!(e.mem.u16(base + 6), 0x8000);
        }
    }

    #[test]
    fn fn_00807cb0_passes_the_object_parts_or_null() {
        let mut e = Engine::new();
        for callee in [ENTITY_STEP_1, ENTITY_STEP_2, ENTITY_STEP_3] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0080_7cb0, &[0x100, 0x900]);
        e.call(0x0080_7cb0, &[0, 0x900]);
        assert_eq!(
            calls(&e, ENTITY_STEP_1),
            vec![vec![0x900, 0x100], vec![0x900, 0]]
        );
        assert_eq!(
            calls(&e, ENTITY_STEP_2),
            vec![vec![0x900, 0x104], vec![0x900, 0]]
        );
        assert_eq!(
            calls(&e, ENTITY_STEP_3),
            vec![vec![0x900, 0x108], vec![0x900, 0]]
        );
    }

    #[test]
    fn fn_00807d10_and_00807da0_read_and_write_the_header() {
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x1234);
        e.mem.set_u16(this + 4, 7);
        e.mem.set_u16(this + 6, 0xc005);
        assert_eq!(e.call(0x0080_7da0, &[this]).u32(), 5);
        assert_eq!(e.call(0x0080_7d10, &[this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u16(this + 4), 0);
        assert_eq!(e.call(0x0080_7da0, &[this]).u32(), 0);
        assert_eq!(e.mem.u16(this + 6), 0x8000);
    }

    #[test]
    fn fn_00807d60_frees_unless_flagged() {
        let (mut e, heap) = engine_with_heap();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x5550);
        e.mem.set_u16(this + 6, 0x8003);
        e.call(0x0080_7d60, &[this]);
        assert!(calls(&e, SLOT_DOUBLE).is_empty());
        e.mem.set_u16(this + 6, 0x4003);
        e.call(0x0080_7d60, &[this]);
        assert_eq!(calls(&e, SLOT_DOUBLE), vec![vec![heap, 0x5550, 0xc]]);
    }

    #[test]
    fn fn_00807dc0_frees_four_bytes_per_entry() {
        let (mut e, heap) = engine_with_heap();
        e.call(0x0080_7dc0, &[0x6000, 9, 0x18]);
        assert_eq!(calls(&e, SLOT_DOUBLE), vec![vec![heap, 0x6000, 36]]);
    }
}
