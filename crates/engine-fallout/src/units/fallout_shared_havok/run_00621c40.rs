//! `fallout shared/havok/run_00621c40` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00437c10` (unnamed in the engine map): `thiscall`, plain `RET`.
const BASE_RESET: u32 = 0x0043_7c10;
/// `004a4840` (unnamed in the engine map): `thiscall` on the address of a byte, `RET 4`.
const BYTE_HOLDER_OF: u32 = 0x004a_4840;
/// `00626870` (`run_00626790.rs`): `thiscall`, plain `RET`.
const ARRAY_CLEAR: u32 = 0x0062_6870;
/// `0062a100` (`run_0062a0b0.rs`): `thiscall`, plain `RET`; answers the element count that was
/// reserved.
const ARRAY_RESERVED: u32 = 0x0062_a100;
/// `004968b0` (unnamed in the engine map): `thiscall`, plain `RET`; the base constructor of
/// `CellMopp`.
const CELL_MOPP_BASE_CONSTRUCT: u32 = 0x0049_68b0;
/// `006227e0` (`run_006227e0.rs`): `thiscall`, `RET 8`; the constructor of the pointer array.
const POINTER_ARRAY_CONSTRUCT: u32 = 0x0062_27e0;
/// `CellMopp`'s vtable (RTTI `.?AVCellMopp@@`).
const CELL_MOPP_VTABLE: u32 = 0x0104_dab4;
/// The live-instance counter `CellMopp`'s constructor increments.
const CELL_MOPP_INSTANCE_COUNT: u32 = 0x011c_c1c8;
/// Size of one element of the array `00621c90` releases.
const ELEMENT_SIZE: u32 = 0x70;
/// Vtable slot of the allocator's release function.
const ALLOCATOR_RELEASE_SLOT: u32 = 0x10;

// Translated from 00621c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: runs `00437c10` on `this` and answers `this`.
pub fn fn_00621c40(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_RESET, &args![this]);
    this
}

// Translated from 00621ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 8`. On the allocator `this`, calls the vtable
/// slot `+0x10` with `block` and `count * 0x70`: the release of an array of 0x70-byte elements.
pub fn fn_00621ce0(e: &mut Engine, this: Ptr, block: u32, count: u32) {
    e.vcall(
        this.addr(),
        ALLOCATOR_RELEASE_SLOT,
        &args![block, count.wrapping_mul(ELEMENT_SIZE)],
    );
}

// Translated from 00621c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. Clears the array `this` (`00626870`); unless
/// bit 31 of the word at `+8` is set (the block belongs to someone else), releases the block at
/// `+0` through the allocator `allocator` (`00621ce0`, with the reserved count `0062a100`
/// answers). Then clears `+0` and sets the word at `+8` to `0x80000000`.
pub fn fn_00621c90(e: &mut Engine, this: Ptr, allocator: Ptr) {
    e.call(ARRAY_CLEAR, &args![this]);
    if e.mem.u32(this.addr().wrapping_add(8)) & 0x8000_0000 == 0 {
        let reserved = e.call(ARRAY_RESERVED, &args![this]).u32();
        let block = e.mem.u32(this.addr());
        fn_00621ce0(e, allocator, block, reserved);
    }
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr().wrapping_add(8), 0x8000_0000);
}

// Translated from 00621c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`. With a zeroed byte on the stack as the holder, asks
/// `004a4840` for the allocator of `this` and releases the array `this` through it
/// (`00621c90`).
pub fn fn_00621c60(e: &mut Engine, this: Ptr) {
    // The byte at `EBP-1` of the exe's 8-byte frame (`SUB ESP,8`).
    e.with_stack(8, |e, frame| {
        let byte = frame.addr() + 7;
        e.mem.set_u8(byte, 0);
        let allocator = e.call(BYTE_HOLDER_OF, &args![byte, this]).u32();
        fn_00621c90(e, this, Ptr::new(allocator));
    });
}

// Translated from 00621d10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00621d10 - C++ exception unwinding (FS:[0] frame) not translated
/// `CellMopp`'s constructor (RTTI `.?AVCellMopp@@`, vtable `0104dab4`). `fastcall`, plain `RET`.
/// Runs the base constructor (`004968b0`), stores the vtable, constructs the pointer array at
/// `+8` with 0 and 1 (`006227e0`) and counts the instance. Answers `this`.
pub fn fn_00621d10(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(CELL_MOPP_BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), CELL_MOPP_VTABLE);
    e.call(
        POINTER_ARRAY_CONSTRUCT,
        &args![this.addr().wrapping_add(8), 0u32, 1u32],
    );
    let count = e.mem.u32(CELL_MOPP_INSTANCE_COUNT);
    e.mem
        .set_u32(CELL_MOPP_INSTANCE_COUNT, count.wrapping_add(1));
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_1c40, fn_00621c40(Ptr) -> Ptr),
        entry!(0x0062_1c60, fn_00621c60(Ptr)),
        entry!(0x0062_1c90, fn_00621c90(Ptr, Ptr)),
        entry!(0x0062_1ce0, fn_00621ce0(Ptr, u32, u32)),
        entry!(0x0062_1d10, fn_00621d10(Ptr) -> Ptr),
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
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An allocator whose vtable slot `+0x10` is a double at `0x0700_0010`.
    fn allocator(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x10, 0x0700_0010);
        e.register(0x0700_0010, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn at_00621c40_runs_the_base_and_answers_this() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(BASE_RESET, |_, _| Ret::default());
        assert_eq!(e.call(0x0062_1c40, &args![0x4000u32]).u32(), 0x4000);
        assert_eq!(calls(&e, BASE_RESET), vec![vec![0x4000]]);
    }

    #[test]
    fn at_00621ce0_releases_through_slot_0x10_with_the_byte_size() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let allocator = allocator(&mut e);
        e.call(0x0062_1ce0, &args![allocator, 0x9000u32, 3u32]);
        assert_eq!(calls(&e, 0x0700_0010), vec![vec![allocator, 0x9000, 0x150]]);
        // The multiplication wraps like the 32-bit IMUL.
        e.call(0x0062_1ce0, &args![allocator, 0x9000u32, 0x0200_0000u32]);
        assert_eq!(
            calls(&e, 0x0700_0010)[1],
            vec![allocator, 0x9000, 0xe000_0000]
        );
    }

    #[test]
    fn at_00621c90_releases_an_owned_block_and_marks_the_array() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let allocator = allocator(&mut e);
        e.register(ARRAY_CLEAR, |_, _| Ret::default());
        e.register(ARRAY_RESERVED, |_, _| 4u32.into_ret());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x9000);
        e.mem.set_u32(this + 8, 4);
        e.call(0x0062_1c90, &args![this, allocator]);
        assert_eq!(calls(&e, ARRAY_CLEAR), vec![vec![this]]);
        assert_eq!(calls(&e, 0x0700_0010), vec![vec![allocator, 0x9000, 0x1c0]]);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 8), 0x8000_0000);
    }

    #[test]
    fn at_00621c90_keeps_a_block_it_does_not_own() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let allocator = allocator(&mut e);
        e.register(ARRAY_CLEAR, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x9000);
        e.mem.set_u32(this + 8, 0x8000_0004);
        e.call(0x0062_1c90, &args![this, allocator]);
        assert!(calls(&e, 0x0700_0010).is_empty());
        assert!(calls(&e, ARRAY_RESERVED).is_empty());
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 8), 0x8000_0000);
    }

    #[test]
    fn at_00621c60_releases_through_the_allocator_the_holder_answers() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let allocator = allocator(&mut e);
        e.register_double(BYTE_HOLDER_OF, move |e, a| {
            // The byte holder is handed a zeroed byte.
            assert_eq!(e.mem.u8(a[0]), 0);
            allocator.into_ret()
        });
        e.register(ARRAY_CLEAR, |_, _| Ret::default());
        e.register(ARRAY_RESERVED, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x9000);
        e.call(0x0062_1c60, &args![this]);
        let holder = calls(&e, BYTE_HOLDER_OF);
        assert_eq!(holder.len(), 1);
        assert_eq!(holder[0][1], this);
        assert_eq!(calls(&e, 0x0700_0010), vec![vec![allocator, 0x9000, 0x70]]);
    }

    #[test]
    fn at_00621d10_constructs_a_cell_mopp_and_counts_it() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(CELL_MOPP_INSTANCE_COUNT & !0xfff, 0x1000);
        e.set_global(CELL_MOPP_INSTANCE_COUNT, 7u32);
        e.register(CELL_MOPP_BASE_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0x1111);
            Ret::default()
        });
        e.register(POINTER_ARRAY_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0062_1d10, &args![this]).u32(), this);
        assert_eq!(
            e.mem.u32(this),
            CELL_MOPP_VTABLE,
            "the vtable follows the base"
        );
        assert_eq!(
            calls(&e, POINTER_ARRAY_CONSTRUCT),
            vec![vec![this + 8, 0, 1]]
        );
        assert_eq!(e.global::<u32>(CELL_MOPP_INSTANCE_COUNT), 8);
    }

    #[test]
    fn at_00621c60_hands_over_the_byte_at_frame_offset_7() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let allocator = allocator(&mut e);
        e.register_double(BYTE_HOLDER_OF, move |_, _| allocator.into_ret());
        e.register(ARRAY_CLEAR, |_, _| Ret::default());
        e.register(ARRAY_RESERVED, |_, _| 1u32.into_ret());
        let this = e.mem.alloc(0x10);
        let frame = e.next_stack_local(8);
        e.call(0x0062_1c60, &args![this]);
        assert_eq!(calls(&e, BYTE_HOLDER_OF), vec![vec![frame + 7, this]]);
    }
}
