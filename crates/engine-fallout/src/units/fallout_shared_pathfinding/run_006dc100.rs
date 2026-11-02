//! `fallout shared/pathfinding/run_006dc100` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSSimpleArray<T,1024>` template code emitted next to the pathfinding
//! code: constructors, element initialization loops, the "add one slot"
//! helpers, a reallocation and two element move loops. The linker shares
//! these bodies between arrays of the same element size, so the functions
//! are named by address and described by what their bodies do.
//!
//! The vtables the constructors store (RTTI read from the exe):
//! `0106cc14` `.?AV?$BSSimpleArray@PAVPathingNode@@$0EAA@@@`,
//! `0106cc64` `.?AV?$BSSimpleArray@M$0EAA@@@` (`float`),
//! `0106cc8c` `.?AV?$BSSimpleArray@UCrossedTriangle@@$0EAA@@@`.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// `BSSimpleArray<PathingNode*,1024>`'s vtable (RTTI).
const PATHING_NODE_ARRAY_VTABLE: u32 = 0x0106_cc14;
/// `BSSimpleArray<float,1024>`'s vtable (RTTI).
const FLOAT_ARRAY_VTABLE: u32 = 0x0106_cc64;
/// `BSSimpleArray<CrossedTriangle,1024>`'s vtable (RTTI).
const CROSSED_TRIANGLE_ARRAY_VTABLE: u32 = 0x0106_cc8c;

/// `006b3eb0` (thiscall `(array, capacity, count)`, `RET 8`): the
/// initializer the 4-byte-element array constructors run (engine map unit
/// `pathsmootherpov.cpp`).
const WORD_ARRAY_INITIALIZE: u32 = 0x006b_3eb0;
/// `006e6da0(size, address)` (cdecl): placement allocation; the loops below
/// run the element constructor on its result unless it is 0.
const PLACEMENT: u32 = 0x006e_6da0;
/// `006d4550` (thiscall, no stack words): the 0x20-byte element constructor.
const WIDE_ELEMENT_CONSTRUCTOR: u32 = 0x006d_4550;
/// `006815c0` (thiscall, no stack words): the 8-byte element constructor.
const PAIR_ELEMENT_CONSTRUCTOR: u32 = 0x0068_15c0;
/// `00438b90` (thiscall): whether the array is full, byte result.
const IS_FULL: u32 = 0x0043_8b90;
/// `009a3910` (thiscall): the grown capacity of a full array.
const GROWN_CAPACITY: u32 = 0x009a_3910;
/// `006d0680` (thiscall `(array, capacity, count)`, `RET 8`): the resize
/// [`fn_006dc260`] uses.
const RESIZE_006D0680: u32 = 0x006d_0680;
/// `006dbfc0` (thiscall `(array, capacity, count)`, `RET 8`): the resize
/// [`fn_006dc500`] uses.
const RESIZE_006DBFC0: u32 = 0x006d_bfc0;
/// `00761540` (thiscall): reserves one more slot of the array and returns
/// its index.
const ADD_SLOT: u32 = 0x0076_1540;
/// `00726bf0` (thiscall `(array, address, count)`, `RET 8`): the 4-byte
/// element initialization loop.
const WORD_ELEMENTS_INITIALIZE: u32 = 0x0072_6bf0;
/// `006d32c0` (thiscall, no stack words): the destructor [`fn_006dc4d0`]
/// runs.
const DESTRUCTOR_006D32C0: u32 = 0x006d_32c0;
/// `operator delete` (`00401030`, cdecl `(block)`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `006a8500` (thiscall): frees the array's buffer.
const FREE_BUFFER: u32 = 0x006a_8500;
/// `memmove` (`00ec7230`, cdecl `(dest, src, size)`).
const MEMMOVE: u32 = 0x00ec_7230;

/// Runs the placement and the element constructor on `count` elements of
/// `size` bytes from `array`.
fn construct_elements(e: &mut Engine, array: u32, count: u32, size: u32, constructor: u32) {
    let mut index = 0u32;
    while index < count {
        let address = array.wrapping_add(index.wrapping_mul(size));
        let element = e.call(PLACEMENT, &args![size, address]).u32();
        if element != 0 {
            e.call(constructor, &args![element]);
        }
        index += 1;
    }
}

/// The body shared by [`fn_006dc1d0`] and [`fn_006dc440`]: clears the
/// array, reserves `max(capacity, count)` elements through vtable slot +4
/// and, when `count` is not zero, constructs the first `count` elements.
fn initialize(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    capacity: u32,
    count: u32,
    construct: fn(&mut Engine, Ptr, Ptr, u32),
) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let capacity = if count > capacity { count } else { capacity };
    if capacity != 0 {
        let buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    }
    if count != 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        construct(e, this.cast(), Ptr::new(buffer), count);
        e.set(this, BSSimpleArray::iSize, count);
    }
}

/// The body shared by [`fn_006dc260`] and [`fn_006dc500`]: when the array
/// is full, an array without capacity gets 4 elements through vtable slot
/// +4, otherwise it is resized to the grown capacity; then `iSize` grows by
/// one and its old value is returned.
fn add_slot(e: &mut Engine, this: Ptr<BSSimpleArray>, resize: u32) -> u32 {
    if e.call(IS_FULL, &args![this]).u8() != 0 {
        if e.get(this, BSSimpleArray::iReservedSize) == 0 {
            let capacity = 4u32;
            let buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
            e.set(this, BSSimpleArray::pBuffer, buffer);
            e.set(this, BSSimpleArray::iReservedSize, capacity);
        } else {
            let capacity = e.call(GROWN_CAPACITY, &args![this]).u32();
            let count = e.get(this, BSSimpleArray::iSize);
            e.call(resize, &args![this, capacity, count]);
            e.set(this, BSSimpleArray::iReservedSize, capacity);
        }
    }
    let size = e.get(this, BSSimpleArray::iSize).wrapping_add(1);
    e.set(this, BSSimpleArray::iSize, size);
    size.wrapping_sub(1)
}

/// The body shared by [`fn_006dc640`] and [`fn_006dc6e0`]: moves `count`
/// elements of `size` bytes from `source` to `dest` one `memmove` each,
/// front to back when `dest < source`, back to front when `dest > source`
/// (the backward index is compared as a signed integer), nothing when they
/// are equal.
fn move_elements(e: &mut Engine, dest: u32, source: u32, count: u32, size: u32) {
    if count == 0 {
        return;
    }
    if dest < source {
        let mut index = 0u32;
        while index < count {
            let offset = index.wrapping_mul(size);
            e.call(
                MEMMOVE,
                &args![dest.wrapping_add(offset), source.wrapping_add(offset), size],
            );
            index += 1;
        }
    } else if dest > source {
        let mut index = count.wrapping_sub(1) as i32;
        while index >= 0 {
            let offset = (index as u32).wrapping_mul(size);
            e.call(
                MEMMOVE,
                &args![dest.wrapping_add(offset), source.wrapping_add(offset), size],
            );
            index -= 1;
        }
    }
}

// Translated from 006dc100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<PathingNode*,1024>`
/// constructor: stores its vtable (`0106cc14`) and runs
/// `006b3eb0(this, 0, 0)`. Returns `this`.
pub fn fn_006dc100(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), PATHING_NODE_ARRAY_VTABLE);
    e.call(WORD_ARRAY_INITIALIZE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006dc130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: constructs `count` 0x20-byte elements from
/// `array` (`006e6da0(0x20, element)`, then `006d4550` unless that returned
/// 0). `thiscall` with `RET 8`; `this` (ECX) is never read.
// Translation incomplete: 006dc130 - FS:[0] exception frame and security cookie (011c16bc) are not translated.
pub fn fn_006dc130(e: &mut Engine, _this: Ptr, array: Ptr, count: u32) {
    construct_elements(e, array.addr(), count, 0x20, WIDE_ELEMENT_CONSTRUCTOR);
}

// Translated from 006dc1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the array and reserves
/// `max(capacity, count)` 0x20-byte elements through vtable slot +4; when
/// `count` is not zero, constructs that many with [`fn_006dc130`] and sets
/// `iSize` to it. `RET 8`.
pub fn fn_006dc1d0(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, count: u32) {
    initialize(e, this, capacity, count, fn_006dc130);
}

// Translated from 006dc260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: reserves one more element and returns its
/// index. When full (`00438b90`), an array without capacity gets 4 elements
/// through vtable slot +4, otherwise `006d0680(this, grown, iSize)` resizes
/// it to the grown capacity (`009a3910`).
pub fn fn_006dc260(e: &mut Engine, this: Ptr<BSSimpleArray>) -> u32 {
    add_slot(e, this, RESIZE_006D0680)
}

// Translated from 006dc2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<float,1024>` constructor:
/// stores its vtable (`0106cc64`) and runs `006b3eb0(this, 0, 0)`. Returns
/// `this`.
pub fn fn_006dc2f0(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), FLOAT_ARRAY_VTABLE);
    e.call(WORD_ARRAY_INITIALIZE, &args![this, 0u32, 0u32]);
    this
}

// Translated from 006dc320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: appends the 4-byte value at `value` to the
/// array and returns its index. The slot comes from `00761540`, is
/// initialized by `00726bf0(this, &pBuffer[index], 1)`, then the value is
/// copied with `FLD`/`FSTP` (a signalling NaN comes out quiet). `RET 4`.
pub fn fn_006dc320(e: &mut Engine, this: Ptr<BSSimpleArray>, value: Ptr) -> u32 {
    let index = e.call(ADD_SLOT, &args![this]).u32();
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let slot = buffer.wrapping_add(index.wrapping_mul(4));
    e.call(WORD_ELEMENTS_INITIALIZE, &args![this, slot, 1u32]);
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    let bits = x87_quiet_bits(e.mem.u32(value.addr()));
    e.mem
        .set_u32(buffer.wrapping_add(index.wrapping_mul(4)), bits);
    index
}

// Translated from 006dc370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `BSSimpleArray<CrossedTriangle,1024>`
/// constructor: stores its vtable (`0106cc8c`) and runs
/// [`fn_006dc440`]`(this, 0, 0)`. Returns `this`.
pub fn fn_006dc370(e: &mut Engine, this: Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray> {
    e.mem.set_u32(this.addr(), CROSSED_TRIANGLE_ARRAY_VTABLE);
    fn_006dc440(e, this, 0, 0);
    this
}

// Translated from 006dc3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: constructs `count` 8-byte elements from
/// `array` (`006e6da0(8, element)`, then `006815c0` unless that returned 0).
/// `thiscall` with `RET 8`; `this` (ECX) is never read.
// Translation incomplete: 006dc3a0 - FS:[0] exception frame and security cookie (011c16bc) are not translated.
pub fn fn_006dc3a0(e: &mut Engine, _this: Ptr, array: Ptr, count: u32) {
    construct_elements(e, array.addr(), count, 8, PAIR_ELEMENT_CONSTRUCTOR);
}

// Translated from 006dc440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears the array and reserves
/// `max(capacity, count)` 8-byte elements through vtable slot +4; when
/// `count` is not zero, constructs that many with [`fn_006dc3a0`] and sets
/// `iSize` to it. `RET 8`.
pub fn fn_006dc440(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, count: u32) {
    initialize(e, this, capacity, count, fn_006dc3a0);
}

// Translated from 006dc4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a scalar deleting destructor: runs
/// `006d32c0` on `this` and frees it with `operator delete` when bit 0 of
/// `flags` is set. Returns `this`. `RET 4`.
pub fn fn_006dc4d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCTOR_006D32C0, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 006dc500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: reserves one more element and returns its
/// index, like [`fn_006dc260`] but resizing a full array with
/// `006dbfc0(this, grown, iSize)`.
pub fn fn_006dc500(e: &mut Engine, this: Ptr<BSSimpleArray>) -> u32 {
    add_slot(e, this, RESIZE_006DBFC0)
}

// Translated from 006dc590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: reallocates a 12-byte-element array to
/// `capacity`. Without a buffer: a new buffer from vtable slot +4 and
/// `iReservedSize = capacity`. When `count` equals `iReservedSize`: the
/// buffer is replaced by vtable slot +0xc `(pBuffer, capacity)`. Otherwise
/// a new buffer from slot +4, the `count` old elements moved into it
/// ([`fn_006dc6e0`]), the old buffer freed (`006a8500`) and the new one
/// stored. Only the first case writes `iReservedSize`. `RET 8`.
pub fn fn_006dc590(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, count: u32) {
    let t = this.addr();
    if e.get(this, BSSimpleArray::pBuffer) == 0 {
        let buffer = e.vcall(t, 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    } else if count == e.get(this, BSSimpleArray::iReservedSize) {
        let old = e.get(this, BSSimpleArray::pBuffer);
        let buffer = e.vcall(t, 0xc, &args![old, capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
    } else {
        let buffer = e.vcall(t, 4, &args![capacity]).u32();
        let old = e.get(this, BSSimpleArray::pBuffer);
        fn_006dc6e0(e, this.cast(), buffer, old, count);
        e.call(FREE_BUFFER, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, buffer);
    }
}

// Translated from 006dc640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: moves `count` 0x20-byte elements from
/// `source` to `dest` with one `memmove` per element, in the direction that
/// keeps overlapping ranges intact. `thiscall` with `RET 0xc`; `this` (ECX)
/// is never read.
pub fn fn_006dc640(e: &mut Engine, _this: Ptr, dest: u32, source: u32, count: u32) {
    move_elements(e, dest, source, count, 0x20);
}

// Translated from 006dc6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: moves `count` 12-byte elements from `source`
/// to `dest` with one `memmove` per element, in the direction that keeps
/// overlapping ranges intact. `thiscall` with `RET 0xc`; `this` (ECX) is
/// never read.
pub fn fn_006dc6e0(e: &mut Engine, _this: Ptr, dest: u32, source: u32, count: u32) {
    move_elements(e, dest, source, count, 0xc);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x006dc100,
            fn_006dc100(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x006dc130, fn_006dc130(Ptr, Ptr, u32)),
        entry!(0x006dc1d0, fn_006dc1d0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x006dc260, fn_006dc260(Ptr<BSSimpleArray>) -> u32),
        entry!(
            0x006dc2f0,
            fn_006dc2f0(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x006dc320, fn_006dc320(Ptr<BSSimpleArray>, Ptr) -> u32),
        entry!(
            0x006dc370,
            fn_006dc370(Ptr<BSSimpleArray>) -> Ptr<BSSimpleArray>
        ),
        entry!(0x006dc3a0, fn_006dc3a0(Ptr, Ptr, u32)),
        entry!(0x006dc440, fn_006dc440(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x006dc4d0, fn_006dc4d0(Ptr, u32) -> Ptr),
        entry!(0x006dc500, fn_006dc500(Ptr<BSSimpleArray>) -> u32),
        entry!(0x006dc590, fn_006dc590(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x006dc640, fn_006dc640(Ptr, u32, u32, u32)),
        entry!(0x006dc6e0, fn_006dc6e0(Ptr, u32, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Slot +4 hands out 0x100-byte blocks, slot +0xc a fixed block.
    fn array_object(e: &mut Engine) -> Ptr<BSSimpleArray> {
        let vtable = e.mem.alloc(16);
        e.register(0x7000_0004, |e, _| e.mem.alloc(0x100).into_ret());
        e.register(0x7000_000c, |_, _| 0x5000u32.into_ret());
        e.mem.set_u32(vtable + 4, 0x7000_0004);
        e.mem.set_u32(vtable + 0xc, 0x7000_000c);
        let this: Ptr<BSSimpleArray> = e.new_object();
        e.mem.set_u32(this.addr(), vtable);
        for off in [4, 8, 0xc] {
            e.mem.set_u32(this.addr() + off, 0x99);
        }
        this
    }

    #[test]
    fn constructors_store_vtable_and_initialize() {
        let mut e = Engine::new();
        e.register(WORD_ARRAY_INITIALIZE, |_, _| Ret::default());
        for (addr, vtable) in [
            (0x006dc100u32, PATHING_NODE_ARRAY_VTABLE),
            (0x006dc2f0, FLOAT_ARRAY_VTABLE),
        ] {
            let this = e.mem.alloc(0x10);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![this]).u32(), this);
            assert_eq!(e.mem.u32(this), vtable);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, WORD_ARRAY_INITIALIZE), vec![vec![this, 0, 0]]);
        }
    }

    #[test]
    fn crossed_triangle_constructor_clears_the_array() {
        let mut e = Engine::new();
        let this: Ptr<BSSimpleArray> = e.new_object();
        for off in [4, 8, 0xc] {
            e.mem.set_u32(this.addr() + off, 0x99);
        }
        assert_eq!(e.call(0x006dc370, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), CROSSED_TRIANGLE_ARRAY_VTABLE);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0);
        assert_eq!(e.get(this, BSSimpleArray::iSize), 0);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 0);
    }

    #[test]
    fn element_loops_construct_unless_placement_fails() {
        for (addr, size, constructor) in [
            (0x006dc130u32, 0x20u32, WIDE_ELEMENT_CONSTRUCTOR),
            (0x006dc3a0, 8, PAIR_ELEMENT_CONSTRUCTOR),
        ] {
            let mut e = Engine::new();
            e.register(PLACEMENT, |_, a| a[1].into_ret());
            e.register(constructor, |_, a| a[0].into_ret());
            e.call_log = Some(vec![]);
            e.call(addr, &args![0x1111u32, 0x6000u32, 3u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls(&log, PLACEMENT),
                vec![
                    vec![size, 0x6000],
                    vec![size, 0x6000 + size],
                    vec![size, 0x6000 + 2 * size]
                ]
            );
            assert_eq!(calls(&log, constructor).len(), 3);
            assert_eq!(calls(&log, constructor)[2], vec![0x6000 + 2 * size]);
            e.register(PLACEMENT, |_, _| 0u32.into_ret());
            e.call_log = Some(vec![]);
            e.call(addr, &args![0u32, 0x6000u32, 2u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, PLACEMENT).len(), 2);
            assert!(calls(&log, constructor).is_empty());
        }
    }

    #[test]
    fn initializers_reserve_the_larger_and_construct_count() {
        for (addr, constructor) in [
            (0x006dc1d0u32, WIDE_ELEMENT_CONSTRUCTOR),
            (0x006dc440, PAIR_ELEMENT_CONSTRUCTOR),
        ] {
            for (capacity, count, reserved) in [(0u32, 0u32, 0u32), (4, 0, 4), (2, 5, 5), (7, 3, 7)]
            {
                let mut e = Engine::new();
                e.register(PLACEMENT, |_, a| a[1].into_ret());
                e.register(constructor, |_, a| a[0].into_ret());
                let this = array_object(&mut e);
                e.call_log = Some(vec![]);
                e.call(addr, &args![this, capacity, count]);
                let log = e.call_log.take().unwrap();
                let reservations = calls(&log, 0x7000_0004);
                if reserved == 0 {
                    assert!(reservations.is_empty());
                    assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0);
                } else {
                    assert_eq!(reservations, vec![vec![this.addr(), reserved]]);
                    assert_ne!(e.get(this, BSSimpleArray::pBuffer), 0);
                }
                assert_eq!(e.get(this, BSSimpleArray::iReservedSize), reserved);
                assert_eq!(e.get(this, BSSimpleArray::iSize), count);
                assert_eq!(calls(&log, constructor).len() as u32, count);
            }
        }
    }

    #[test]
    fn add_slot_grows_full_arrays() {
        for (addr, resize) in [
            (0x006dc260u32, RESIZE_006D0680),
            (0x006dc500, RESIZE_006DBFC0),
        ] {
            let mut e = Engine::new();
            e.register(GROWN_CAPACITY, |_, _| 16u32.into_ret());
            e.register(resize, |_, _| Ret::default());
            let this = array_object(&mut e);

            // Not full: only the size grows.
            e.register(IS_FULL, |_, _| 0u32.into_ret());
            e.set(this, BSSimpleArray::iSize, 2);
            e.set(this, BSSimpleArray::iReservedSize, 8);
            assert_eq!(e.call(addr, &args![this]).u32(), 2);
            assert_eq!(e.get(this, BSSimpleArray::iSize), 3);

            // Full without capacity: 4 elements from slot +4.
            e.register(IS_FULL, |_, _| 1u32.into_ret());
            e.set(this, BSSimpleArray::iSize, 0);
            e.set(this, BSSimpleArray::iReservedSize, 0);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![this]).u32(), 0);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, 0x7000_0004), vec![vec![this.addr(), 4]]);
            assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 4);
            assert_eq!(e.get(this, BSSimpleArray::iSize), 1);

            // Full with capacity: resized to the grown capacity.
            e.set(this, BSSimpleArray::iSize, 8);
            e.set(this, BSSimpleArray::iReservedSize, 8);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![this]).u32(), 8);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, resize), vec![vec![this.addr(), 16, 8]]);
            assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 16);
            assert_eq!(e.get(this, BSSimpleArray::iSize), 9);
        }
    }

    #[test]
    fn append_value_copies_through_x87() {
        let mut e = Engine::new();
        let this: Ptr<BSSimpleArray> = e.new_object();
        let buffer = e.mem.alloc(0x20);
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.register(ADD_SLOT, |_, _| 3u32.into_ret());
        e.register(WORD_ELEMENTS_INITIALIZE, |_, _| Ret::default());
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0x7f80_0001);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x006dc320, &args![this, value]).u32(), 3);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, WORD_ELEMENTS_INITIALIZE),
            vec![vec![this.addr(), buffer + 12, 1]]
        );
        // The signalling NaN comes out quiet.
        assert_eq!(e.mem.u32(buffer + 12), 0x7fc0_0001);
        e.mem.set_u32(value, 0x4048_0000);
        e.call(0x006dc320, &args![this, value]);
        assert_eq!(e.mem.f32(buffer + 12), 3.125);
    }

    #[test]
    fn scalar_deleting_destructor_frees_on_bit_zero() {
        let mut e = Engine::new();
        e.register(DESTRUCTOR_006D32C0, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        for (flags, frees) in [(1u32, 1usize), (0, 0), (3, 1), (2, 0)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x006dc4d0, &args![0x4000u32, flags]).u32(), 0x4000);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls(&log, DESTRUCTOR_006D32C0), vec![vec![0x4000]]);
            assert_eq!(calls(&log, OPERATOR_DELETE).len(), frees);
        }
    }

    #[test]
    fn reallocate_covers_its_three_cases() {
        let mut e = Engine::new();
        e.register(MEMMOVE, |_, _| Ret::default());
        e.register(FREE_BUFFER, |_, _| Ret::default());
        let this = array_object(&mut e);

        // No buffer: a new one, capacity stored.
        e.set(this, BSSimpleArray::pBuffer, 0);
        e.call(0x006dc590, &args![this, 10u32, 0u32]);
        let first = e.get(this, BSSimpleArray::pBuffer);
        assert_ne!(first, 0);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 10);

        // Count equal to the capacity: slot +0xc reallocates in place.
        e.call_log = Some(vec![]);
        e.call(0x006dc590, &args![this, 20u32, 10u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x7000_000c), vec![vec![this.addr(), first, 20]]);
        assert_eq!(e.get(this, BSSimpleArray::pBuffer), 0x5000);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 10);

        // Otherwise: new buffer, elements moved, old buffer freed.
        e.call_log = Some(vec![]);
        e.call(0x006dc590, &args![this, 20u32, 2u32]);
        let log = e.call_log.take().unwrap();
        let fresh = e.get(this, BSSimpleArray::pBuffer);
        // The heap block lies above 0x5000, so the move runs back to front.
        assert!(fresh > 0x5000);
        assert_eq!(
            calls(&log, MEMMOVE),
            vec![vec![fresh + 12, 0x500c, 12], vec![fresh, 0x5000, 12]]
        );
        assert_eq!(calls(&log, FREE_BUFFER), vec![vec![this.addr()]]);
        assert_eq!(e.get(this, BSSimpleArray::iReservedSize), 10);
    }

    #[test]
    fn move_loops_pick_the_direction() {
        for (addr, size) in [(0x006dc640u32, 0x20u32), (0x006dc6e0, 0xc)] {
            let mut e = Engine::new();
            e.register(MEMMOVE, |_, _| Ret::default());
            // Forward when dest < source.
            e.call_log = Some(vec![]);
            e.call(addr, &args![0u32, 0x1000u32, 0x2000u32, 2u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls(&log, MEMMOVE),
                vec![
                    vec![0x1000, 0x2000, size],
                    vec![0x1000 + size, 0x2000 + size, size]
                ]
            );
            // Backward when dest > source.
            e.call_log = Some(vec![]);
            e.call(addr, &args![0u32, 0x2000u32, 0x1000u32, 2u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(
                calls(&log, MEMMOVE),
                vec![
                    vec![0x2000 + size, 0x1000 + size, size],
                    vec![0x2000, 0x1000, size]
                ]
            );
            // Equal ranges, a zero count, or a count whose last index is
            // negative as a signed integer: nothing.
            for (dest, source, count) in [
                (0x1000u32, 0x1000u32, 2u32),
                (0x1000, 0x2000, 0),
                (0x2000, 0x1000, 0x8000_0001),
            ] {
                e.call_log = Some(vec![]);
                e.call(addr, &args![0u32, dest, source, count]);
                let log = e.call_log.take().unwrap();
                assert!(calls(&log, MEMMOVE).is_empty());
            }
        }
    }
}
