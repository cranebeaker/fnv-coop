//! `fallout/misc/run_00845230` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// `00845b30(this, word)` (unit `bgssaveloadchangesmap`): the base
/// constructor `00845550` runs with `0x13af`.
const BASE_CONSTRUCTOR: u32 = 0x0084_5b30;
/// The vtable `00845550` stores.
const VTABLE: u32 = 0x0107_ed68;

/// `006e6da0(size, address)` (cdecl): placement allocation, returns `address`.
const PLACEMENT: u32 = 0x006e_6da0;
/// `0096a2d0(this)`: zeroes the two words of an 8-byte element (thiscall,
/// returns `this`).
const ELEMENT_INITIALIZER: u32 = 0x0096_a2d0;

/// `BSSimpleArray<OffsetData,1024>`'s vtable (RTTI
/// `.?AV?$BSSimpleArray@UOffsetData@@$0EAA@@@`), stored by `00845430`.
const SIMPLE_ARRAY_VTABLE: u32 = 0x0107_ed50;
/// `operator new` (`00401000`, cdecl `(size)`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (`00401030`, cdecl `(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00844ff0` (thiscall, no stack words): the destructor the scalar deleting
/// destructor `00845230` runs (it stores the `BSScrapArray<OffsetData,1024>`
/// vtable `0107ed3c` and calls `008454f0(this, 1)`).
const SCRAP_ARRAY_DESTRUCTOR: u32 = 0x0084_4ff0;
/// `007f6900` (thiscall, no stack words, returns `this`): runs `0096a2d0` on
/// `this + 8`.
const PAIR_INITIALIZER: u32 = 0x007f_6900;
/// `00438b90` (thiscall): whether the array is full (`iSize ==
/// iReservedSize`), byte result.
const IS_FULL: u32 = 0x0043_8b90;
/// `009a3910` (thiscall): the grown capacity (`iReservedSize` doubled while
/// it is at most 0x400, otherwise plus 0x400).
const GROWN_CAPACITY: u32 = 0x009a_3910;
/// `006a2770` (thiscall `(array, capacity, count)`, `RET 8`): called with
/// the grown capacity and `iSize` when a full array grows.
const RESIZE: u32 = 0x006a_2770;
/// `0072ba80` (thiscall `(array, buffer, count)`, `RET 8`): the per-element
/// destruction loop (empty for this element type).
const DESTROY_ELEMENTS: u32 = 0x0072_ba80;
/// `006a8500` (thiscall): frees the buffer through vtable slot +8 and clears
/// `pBuffer`.
const FREE_BUFFER: u32 = 0x006a_8500;

// Translated from 00845230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor in slot 0 of
/// the `BSScrapArray<OffsetData,1024>` vtable (`0107ed3c`, RTTI). Runs the
/// destructor `00844ff0` and frees the object if bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_00845230(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(SCRAP_ARRAY_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00845260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: allocates a 0x10-byte object and constructs it
/// with `008452e0`; returns it, or 0 if the allocation failed. `this` (ECX)
/// is saved but never read.
// Translation incomplete: 00845260 - FS:[0] exception frame (unwind of the allocation if the constructor throws) is not translated.
pub fn fn_00845260(e: &mut Engine, _this: Ptr) -> Ptr {
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    fn_008452e0(e, Ptr::new(block))
}

// Translated from 008452e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `007f6900` on `this`; returns `this`.
pub fn fn_008452e0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(PAIR_INITIALIZER, &args![this]);
    this
}

// Translated from 00845430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<OffsetData,1024>`
/// constructor: stores its vtable (`0107ed50`, RTTI) and runs
/// `008453a0(this, 0, 0)`. Returns `this`.
pub fn fn_00845430(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), SIMPLE_ARRAY_VTABLE);
    fn_008453a0(e, this, 0, 0);
    this
}

// Translated from 00845460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: reserves one more element of a
/// `BSSimpleArray` and returns its index. When the array is full
/// (`00438b90`): an array without capacity gets 4 elements through vtable
/// slot +4, otherwise it is resized (`006a2770`) to the grown capacity
/// (`009a3910`). Then `iSize` is incremented and its old value returned.
pub fn fn_00845460(e: &mut Engine, this: Ptr<BSSimpleArray>) -> u32 {
    let t = this.addr();
    if e.call(IS_FULL, &args![this]).u8() != 0 {
        if e.get(this, BSSimpleArray::iReservedSize) == 0 {
            let capacity = 4u32;
            let block = e.vcall(t, 4, &args![capacity]).u32();
            e.set(this, BSSimpleArray::pBuffer, block);
            e.set(this, BSSimpleArray::iReservedSize, capacity);
        } else {
            let capacity = e.call(GROWN_CAPACITY, &args![this]).u32();
            let count = e.get(this, BSSimpleArray::iSize);
            e.call(RESIZE, &args![this, capacity, count]);
            e.set(this, BSSimpleArray::iReservedSize, capacity);
        }
    }
    let count = e.get(this, BSSimpleArray::iSize).wrapping_add(1);
    e.set(this, BSSimpleArray::iSize, count);
    count.wrapping_sub(1)
}

// Translated from 008454f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears a `BSSimpleArray`. With a buffer: runs
/// `0072ba80(pBuffer, iSize)`, and if `free` is set frees the buffer
/// (`006a8500`) and clears `pBuffer` and `iReservedSize`; then `iSize` is 0.
/// Without a buffer nothing happens. `RET 4`.
pub fn fn_008454f0(e: &mut Engine, this: Ptr<BSSimpleArray>, free: u8) {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    if buffer == 0 {
        return;
    }
    let count = e.get(this, BSSimpleArray::iSize);
    e.call(DESTROY_ELEMENTS, &args![this, buffer, count]);
    if free != 0 {
        e.call(FREE_BUFFER, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.set(this, BSSimpleArray::iReservedSize, 0);
    }
    e.set(this, BSSimpleArray::iSize, 0);
}

// Translated from 00845300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: initializes `count` 8-byte elements starting at
/// `array` (`006e6da0(8, element)`, then `0096a2d0` unless that returned 0).
/// `thiscall` with `RET 8`: `this` (ECX) is never read.
// Translation incomplete: 00845300 - FS:[0] exception frame and security cookie (011c16bc) are not translated.
pub fn fn_00845300(e: &mut Engine, _this: Ptr, array: Ptr, count: u32) {
    for index in 0..count {
        let element = e
            .call(
                PLACEMENT,
                &args![8u32, array.addr().wrapping_add(index.wrapping_mul(8))],
            )
            .u32();
        if element != 0 {
            e.call(ELEMENT_INITIALIZER, &args![element]);
        }
    }
}

// Translated from 008453a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the pointer (+4), count (+8) and
/// capacity (+0xc), then reserves `max(capacity, count)` elements through
/// vtable slot +4 (one argument, the element count; the result is stored at
/// +4, the capacity at +0xc) and, if `count` is not zero, initializes the
/// first `count` elements with `00845300` and stores `count` at +8. `RET 8`.
pub fn fn_008453a0(e: &mut Engine, this: Ptr, capacity: u32, count: u32) {
    let t = this.addr();
    e.mem.set_u32(t + 4, 0);
    e.mem.set_u32(t + 8, 0);
    e.mem.set_u32(t + 0xc, 0);
    let capacity = capacity.max(count);
    if capacity != 0 {
        let block = e.vcall(t, 4, &args![capacity]).u32();
        e.mem.set_u32(t + 4, block);
        e.mem.set_u32(t + 0xc, capacity);
    }
    if count != 0 {
        let array = e.mem.u32(t + 4);
        fn_00845300(e, this, Ptr::new(array), count);
        e.mem.set_u32(t + 8, count);
    }
}

// Translated from 00845550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: runs `00845b30(this, 0x13af)` and stores the
/// vtable `0107ed68` over the one it set. Returns `this`.
pub fn fn_00845550(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_CONSTRUCTOR, &args![this, 0x13afu32]);
    e.mem.set_u32(this.addr(), VTABLE);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00845230, fn_00845230(Ptr, u32) -> Ptr),
        entry!(0x00845260, fn_00845260(Ptr) -> Ptr),
        entry!(0x008452e0, fn_008452e0(Ptr) -> Ptr),
        entry!(0x00845300, fn_00845300(Ptr, Ptr, u32)),
        entry!(0x00845430, fn_00845430(Ptr) -> Ptr),
        entry!(0x00845460, fn_00845460(Ptr<BSSimpleArray>) -> u32),
        entry!(0x008454f0, fn_008454f0(Ptr<BSSimpleArray>, u8)),
        entry!(0x008453a0, fn_008453a0(Ptr, u32, u32)),
        entry!(0x00845550, fn_00845550(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn array_doubles(e: &mut Engine) {
        e.register(PLACEMENT, |_, a| a[1].into_ret());
        e.register(ELEMENT_INITIALIZER, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            a[0].into_ret()
        });
    }

    #[test]
    fn fn_00845300_initializes_each_element_and_skips_null() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        let array = e.mem.alloc(24);
        for i in 0..6 {
            e.mem.set_u32(array + 4 * i, 0xaaaa_aaaa);
        }
        e.call_log = Some(vec![]);
        e.call(0x00845300, &args![0x1111u32, array, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (PLACEMENT, vec![8, array]));
        assert_eq!(log[2], (ELEMENT_INITIALIZER, vec![array]));
        assert_eq!(log[5], (PLACEMENT, vec![8, array + 16]));
        assert_eq!(log.len(), 7);
        assert_eq!(e.mem.bytes(array, 24), vec![0; 24]);
        // A placement result of 0 skips the element initializer.
        e.register(PLACEMENT, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00845300, &args![0u32, array, 2u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
        e.call_log = Some(vec![]);
        e.call(0x00845300, &args![0u32, array, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    /// An object whose vtable slot +4 hands out blocks.
    fn array_object(e: &mut Engine) -> u32 {
        array_doubles(e);
        let vtable = e.mem.alloc(16);
        e.register(0x7000_0004, |e, a| e.mem.alloc(a[1] * 8).into_ret());
        e.mem.set_u32(vtable + 4, 0x7000_0004);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        for off in [4, 8, 0xc] {
            e.mem.set_u32(this + off, 0x99);
        }
        this
    }

    #[test]
    fn fn_008453a0_reserves_the_larger_of_capacity_and_count() {
        for (capacity, count, reserved) in [
            (0u32, 0u32, 0u32),
            (4, 0, 4),
            (2, 5, 5),
            (7, 3, 7),
            (0, 2, 2),
        ] {
            let mut e = Engine::new();
            let this = array_object(&mut e);
            e.call_log = Some(vec![]);
            e.call(0x008453a0, &args![this, capacity, count]);
            let log = e.call_log.take().unwrap();
            if reserved == 0 {
                assert_eq!(log.len(), 1, "no reservation, no initialization");
                assert_eq!(e.mem.u32(this + 4), 0);
                assert_eq!(e.mem.u32(this + 0xc), 0);
            } else {
                assert_eq!(log[1], (0x7000_0004, vec![this, reserved]));
                assert_ne!(e.mem.u32(this + 4), 0);
                assert_eq!(e.mem.u32(this + 0xc), reserved);
            }
            assert_eq!(e.mem.u32(this + 8), count);
            let initialized = log.iter().filter(|c| c.0 == ELEMENT_INITIALIZER).count();
            assert_eq!(initialized as u32, count);
        }
    }

    #[test]
    fn fn_00845230_destroys_and_frees_on_bit_zero() {
        let mut e = Engine::new();
        e.register(SCRAP_ARRAY_DESTRUCTOR, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        for (flags, frees) in [(1u32, true), (0, false), (3, true), (2, false)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00845230, &args![this, flags]).u32(), this);
            let log = e.call_log.take().unwrap();
            assert_eq!(log[1], (SCRAP_ARRAY_DESTRUCTOR, vec![this]));
            let deletes: Vec<_> = log.iter().filter(|c| c.0 == OPERATOR_DELETE).collect();
            assert_eq!(deletes.len(), usize::from(frees));
        }
    }

    #[test]
    fn fn_00845260_allocates_and_constructs() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(PAIR_INITIALIZER, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let object = e.call(0x00845260, &args![0x4444u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_ne!(object, 0);
        assert_eq!(log[1], (OPERATOR_NEW, vec![0x10]));
        assert_eq!(log[2], (PAIR_INITIALIZER, vec![object]));
        // A failed allocation returns 0 without constructing.
        e.register(OPERATOR_NEW, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00845260, &args![0x4444u32]).u32(), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn fn_008452e0_returns_this() {
        let mut e = Engine::new();
        e.register(PAIR_INITIALIZER, |_, _| 0x5555u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x008452e0, &args![0x1230u32]).u32(), 0x1230);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (PAIR_INITIALIZER, vec![0x1230]));
    }

    #[test]
    fn fn_00845430_stores_the_vtable_and_empties() {
        let mut e = Engine::new();
        array_doubles(&mut e);
        let this = e.mem.alloc(0x10);
        for off in [4, 8, 0xc] {
            e.mem.set_u32(this + off, 0x99);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00845430, &args![this]).u32(), this);
        assert_eq!(e.call_log.take().unwrap().len(), 1, "no reservation");
        assert_eq!(e.mem.u32(this), SIMPLE_ARRAY_VTABLE);
        assert_eq!(e.mem.bytes(this + 4, 12), vec![0; 12]);
    }

    fn grow_doubles(e: &mut Engine) {
        e.register(IS_FULL, |e, a| {
            (0xff00 | u32::from(e.mem.u32(a[0] + 8) == e.mem.u32(a[0] + 0xc))).into_ret()
        });
        e.register(GROWN_CAPACITY, |e, a| {
            (e.mem.u32(a[0] + 0xc) * 2).into_ret()
        });
        e.register(RESIZE, |_, _| Ret::default());
    }

    #[test]
    fn fn_00845460_grows_when_full_and_returns_the_index() {
        let mut e = Engine::new();
        grow_doubles(&mut e);
        let this = array_object(&mut e);
        // Empty, no capacity: 4 elements through slot +4.
        for off in [4, 8, 0xc] {
            e.mem.set_u32(this + off, 0);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00845460, &args![this]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x7000_0004, vec![this, 4])));
        assert_ne!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 0xc), 4);
        assert_eq!(e.mem.u32(this + 8), 1);
        // Not full: only the count moves.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00845460, &args![this]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == RESIZE || c.0 == 0x7000_0004));
        assert_eq!(e.mem.u32(this + 8), 2);
        // Full with capacity: resized to the grown capacity.
        e.mem.set_u32(this + 8, 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00845460, &args![this]).u32(), 4);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(RESIZE, vec![this, 8, 4])));
        assert_eq!(e.mem.u32(this + 0xc), 8);
        assert_eq!(e.mem.u32(this + 8), 5);
    }

    #[test]
    fn fn_008454f0_clears_and_optionally_frees() {
        let mut e = Engine::new();
        e.register(DESTROY_ELEMENTS, |_, _| Ret::default());
        e.register(FREE_BUFFER, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        for (free, frees) in [(0u32, false), (1, true), (0x100, false)] {
            e.mem.set_u32(this + 4, 0x800);
            e.mem.set_u32(this + 8, 3);
            e.mem.set_u32(this + 0xc, 6);
            e.call_log = Some(vec![]);
            e.call(0x008454f0, &args![this, free]);
            let log = e.call_log.take().unwrap();
            assert_eq!(log[1], (DESTROY_ELEMENTS, vec![this, 0x800, 3]));
            assert_eq!(log.iter().any(|c| c.0 == FREE_BUFFER), frees);
            assert_eq!(e.mem.u32(this + 8), 0);
            let kept = if frees { (0, 0) } else { (0x800, 6) };
            assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 0xc)), kept);
        }
        // Without a buffer nothing happens.
        e.mem.set_u32(this + 4, 0);
        e.mem.set_u32(this + 8, 3);
        e.call_log = Some(vec![]);
        e.call(0x008454f0, &args![this, 1u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(this + 8), 3);
    }

    #[test]
    fn constructs_the_base_then_stores_its_vtable() {
        let mut e = Engine::new();
        e.register(BASE_CONSTRUCTOR, |e, a| {
            e.mem.set_u32(a[0], 0x1234);
            Ret::default()
        });
        let this = e.mem.alloc(16);
        e.call_log = Some(vec![]);
        let r = e.call(0x00845550, &args![this]);
        assert_eq!(r.u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (BASE_CONSTRUCTOR, vec![this, 0x13af]));
        assert_eq!(e.mem.u32(this), VTABLE, "the vtable replaces the base's");
    }
}
