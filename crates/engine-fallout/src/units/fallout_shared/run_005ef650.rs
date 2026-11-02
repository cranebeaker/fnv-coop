//! `fallout shared/run_005ef650` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSSimpleArray` template members: the `unsigned short` array's element
//! construction and resize (`005ef650`, `005ef6a0`) and the foot IK leg
//! data array's start (`005ef910`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleArray;

/// Places one element of `size` bytes at an address (`006e6da0`, cdecl:
/// size, address).
const PLACE_ELEMENT: u32 = 0x006e_6da0;
/// The leg data array's element construction (`005ef750`, `RET 8`:
/// address, count).
const CONSTRUCT_LEG_ELEMENTS: u32 = 0x005e_f750;

// Translated from 005ef650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs `count` `unsigned short` elements from `elements`
/// (`006e6da0(2, address)` for each; `this` is not read).
pub fn fn_005ef650(e: &mut Engine, _this: Ptr, elements: u32, count: u32) {
    let mut i = 0u32;
    while i < count {
        e.call(
            PLACE_ELEMENT,
            &args![2u32, elements.wrapping_add(i.wrapping_mul(2))],
        );
        i = i.wrapping_add(1);
    }
}

// Translated from 005ef6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resizes the `unsigned short` array's buffer to `capacity` keeping
/// `count` elements (`RET 8`): without a buffer, allocates one (vtable +4)
/// and sets the capacity; with `count` equal to the capacity, reallocates
/// in place (vtable +0xC with the buffer and `capacity`; the capacity
/// field is left); otherwise allocates a new buffer, moves the elements
/// (`005efdd0`), frees the old one (`006a8500`) and keeps the new one.
pub fn fn_005ef6a0(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, count: u32) {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    if buffer == 0 {
        let new = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, new);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    } else if count == e.get(this, BSSimpleArray::iReservedSize) {
        let new = e.vcall(this.addr(), 0xc, &args![buffer, capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, new);
    } else {
        let new = e.vcall(this.addr(), 4, &args![capacity]).u32();
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(0x005e_fdd0, &args![this, new, old, count]);
        e.call(0x006a_8500, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, new);
    }
}

// Translated from 005ef910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Starts the foot IK leg data array (`RET 8`): empty, then with a
/// capacity of `capacity` (at least `count`; allocated through vtable +4
/// when not 0) and `count` constructed elements.
pub fn fn_005ef910(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, count: u32) {
    e.set(this, BSSimpleArray::pBuffer, 0);
    e.set(this, BSSimpleArray::iSize, 0);
    e.set(this, BSSimpleArray::iReservedSize, 0);
    let capacity = if count > capacity { count } else { capacity };
    if capacity > 0 {
        let buffer = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    }
    if count > 0 {
        let buffer = e.get(this, BSSimpleArray::pBuffer);
        e.call(CONSTRUCT_LEG_ELEMENTS, &args![this, buffer, count]);
        e.set(this, BSSimpleArray::iSize, count);
    }
}

/// The shape `005ef860` and `005efa80` share (`005ef6a0`'s with another
/// mover): resizes the buffer to `capacity` keeping `count` elements;
/// without a buffer allocates one (vtable +4) and sets the capacity; with
/// `count` equal to the capacity reallocates in place (vtable +0xC);
/// otherwise allocates a new buffer, moves the elements
/// (`mover(this, new, old, count)`), frees the old one (`006a8500`) and
/// keeps the new one.
pub(crate) fn resize_with(
    e: &mut Engine,
    this: Ptr<BSSimpleArray>,
    capacity: u32,
    count: u32,
    mover: u32,
) {
    let buffer = e.get(this, BSSimpleArray::pBuffer);
    if buffer == 0 {
        let new = e.vcall(this.addr(), 4, &args![capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, new);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
    } else if count == e.get(this, BSSimpleArray::iReservedSize) {
        let new = e.vcall(this.addr(), 0xc, &args![buffer, capacity]).u32();
        e.set(this, BSSimpleArray::pBuffer, new);
    } else {
        let new = e.vcall(this.addr(), 4, &args![capacity]).u32();
        let old = e.get(this, BSSimpleArray::pBuffer);
        e.call(mover, &args![this, new, old, count]);
        e.call(0x006a_8500, &args![this]);
        e.set(this, BSSimpleArray::pBuffer, new);
    }
}

/// Constructs `count` elements of `size` bytes from `elements`: each
/// placed by `006e6da0(size, address)` and, when that answers an address,
/// constructed by `construct`.
pub(crate) fn construct_elements(
    e: &mut Engine,
    elements: u32,
    count: u32,
    size: u32,
    construct: fn(&mut Engine, Ptr) -> Ptr,
) {
    let mut i = 0u32;
    while i < count {
        let place = e
            .call(
                PLACE_ELEMENT,
                &args![size, i.wrapping_mul(size).wrapping_add(elements)],
            )
            .u32();
        if place != 0 {
            construct(e, Ptr::new(place));
        }
        i = i.wrapping_add(1);
    }
}

// Translated from 005ef750 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ef750 - the exception-unwinding frame (FS:[0]) around each element's construction is not translated.
/// Constructs `count` 0x38-byte foot IK entries from `elements` (`RET 8`;
/// `this` is not read; the engine map has no name): each placed by
/// `006e6da0(0x38, address)` and constructed by [`fn_005ef7f0`].
pub fn fn_005ef750(e: &mut Engine, _this: Ptr, elements: u32, count: u32) {
    construct_elements(e, elements, count, 0x38, fn_005ef7f0);
}

// Translated from 005ef7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a 0x38-byte foot IK entry (the engine map has no name):
/// the three `u16` at +0, +2, +4 set to 0xFFFF, the byte at +6 cleared and
/// the `float`s at +8 .. +0x1C set to 0.0 (the words after +0x20 are not
/// written). Returns `this`.
pub fn fn_005ef7f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    for offset in [0u32, 2, 4] {
        e.mem.set_u16(base + offset, 0xffff);
    }
    e.mem.set_u8(base + 6, 0);
    for offset in [8u32, 0xc, 0x10, 0x14, 0x18, 0x1c] {
        e.mem.set_f32(base + offset, 0.0);
    }
    this
}

// Translated from 005ef860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resizes the foot IK entry array's buffer (`RET 8`: capacity, count;
/// the engine map has no name), moving the elements with `005efe70`.
pub fn fn_005ef860(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, count: u32) {
    resize_with(e, this, capacity, count, 0x005e_fe70);
}

// Translated from 005ef9a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005ef9a0 - the exception-unwinding frame (FS:[0]) around each element's construction is not translated.
/// Constructs `count` 0x28-byte look IK entries from `elements` (`RET 8`;
/// `this` is not read; the engine map has no name): each placed by
/// `006e6da0(0x28, address)` and constructed by [`fn_005efa40`].
pub fn fn_005ef9a0(e: &mut Engine, _this: Ptr, elements: u32, count: u32) {
    construct_elements(e, elements, count, 0x28, fn_005efa40);
}

// Translated from 005efa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of a 0x28-byte look IK entry (the engine map has no name):
/// the `u16` at +0 and +2 set to 0xFFFF and the `float`s at +0x1C, +0x20,
/// +0x24 set to 0.0. Returns `this`.
pub fn fn_005efa40(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u16(base, 0xffff);
    e.mem.set_u16(base + 2, 0xffff);
    for offset in [0x1cu32, 0x20, 0x24] {
        e.mem.set_f32(base + offset, 0.0);
    }
    this
}

// Translated from 005efa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resizes the look IK entry array's buffer (`RET 8`: capacity, count;
/// the engine map has no name), moving the elements with `006928a0`.
pub fn fn_005efa80(e: &mut Engine, this: Ptr<BSSimpleArray>, capacity: u32, count: u32) {
    resize_with(e, this, capacity, count, 0x0069_28a0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x005e_f750, fn_005ef750(Ptr, u32, u32)),
        entry!(0x005e_f7f0, fn_005ef7f0(Ptr) -> Ptr),
        entry!(0x005e_f860, fn_005ef860(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x005e_f9a0, fn_005ef9a0(Ptr, u32, u32)),
        entry!(0x005e_fa40, fn_005efa40(Ptr) -> Ptr),
        entry!(0x005e_fa80, fn_005efa80(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x005e_f650, fn_005ef650(Ptr, u32, u32)),
        entry!(0x005e_f6a0, fn_005ef6a0(Ptr<BSSimpleArray>, u32, u32)),
        entry!(0x005e_f910, fn_005ef910(Ptr<BSSimpleArray>, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An array whose vtable +4 allocates 0x9000 and +0xC reallocates to
    /// 0xa000.
    fn array(e: &mut Engine, buffer: u32, count: u32, capacity: u32) -> Ptr<BSSimpleArray> {
        let this: Ptr<BSSimpleArray> = e.new_object();
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 4, 0x7000_0004);
        e.mem.set_u32(vtable + 0xc, 0x7000_000c);
        e.mem.set_u32(this.addr(), vtable);
        e.set(this, BSSimpleArray::pBuffer, buffer);
        e.set(this, BSSimpleArray::iSize, count);
        e.set(this, BSSimpleArray::iReservedSize, capacity);
        e.register(0x7000_0004, |_, _| ret(0x9000));
        e.register(0x7000_000c, |_, _| ret(0xa000));
        for addr in [
            0x005e_fdd0u32,
            0x006a_8500,
            CONSTRUCT_LEG_ELEMENTS,
            PLACE_ELEMENT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        this
    }

    fn state(e: &Engine, this: Ptr<BSSimpleArray>) -> (u32, u32, u32) {
        (
            e.get(this, BSSimpleArray::pBuffer),
            e.get(this, BSSimpleArray::iSize),
            e.get(this, BSSimpleArray::iReservedSize),
        )
    }

    #[test]
    fn fn_005ef650_places_each_element() {
        let mut e = Engine::new();
        let this = array(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_f650, &args![this, 0x5000u32, 3u32]);
        assert_eq!(
            calls(&e, PLACE_ELEMENT),
            vec![vec![2, 0x5000], vec![2, 0x5002], vec![2, 0x5004]]
        );
        e.call_log = Some(vec![]);
        e.call(0x005e_f650, &args![this, 0x5000u32, 0u32]);
        assert!(calls(&e, PLACE_ELEMENT).is_empty());
    }

    #[test]
    fn fn_005ef6a0_allocates_reallocates_or_moves() {
        let mut e = Engine::new();
        let this = array(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x005e_f6a0, &args![this, 8u32, 0u32]);
        assert_eq!(state(&e, this), (0x9000, 0, 8));
        assert_eq!(calls(&e, 0x7000_0004), vec![vec![this.addr(), 8]]);

        let mut e = Engine::new();
        let this = array(&mut e, 0x5000, 4, 4);
        e.call_log = Some(vec![]);
        e.call(0x005e_f6a0, &args![this, 8u32, 4u32]);
        assert_eq!(calls(&e, 0x7000_000c), vec![vec![this.addr(), 0x5000, 8]]);
        assert_eq!(state(&e, this), (0xa000, 4, 4));

        let mut e = Engine::new();
        let this = array(&mut e, 0x5000, 3, 4);
        e.call_log = Some(vec![]);
        e.call(0x005e_f6a0, &args![this, 8u32, 3u32]);
        assert_eq!(
            calls(&e, 0x005e_fdd0),
            vec![vec![this.addr(), 0x9000, 0x5000, 3]]
        );
        assert_eq!(calls(&e, 0x006a_8500), vec![vec![this.addr()]]);
        assert_eq!(state(&e, this), (0x9000, 3, 4));
    }

    #[test]
    fn fn_005ef910_starts_with_capacity_and_count() {
        let mut e = Engine::new();
        let this = array(&mut e, 0x5000, 3, 4);
        e.call_log = Some(vec![]);
        e.call(0x005e_f910, &args![this, 0u32, 0u32]);
        assert_eq!(state(&e, this), (0, 0, 0));
        assert!(calls(&e, 0x7000_0004).is_empty());

        // The count raises the capacity.
        e.call_log = Some(vec![]);
        e.call(0x005e_f910, &args![this, 2u32, 5u32]);
        assert_eq!(calls(&e, 0x7000_0004), vec![vec![this.addr(), 5]]);
        assert_eq!(
            calls(&e, CONSTRUCT_LEG_ELEMENTS),
            vec![vec![this.addr(), 0x9000, 5]]
        );
        assert_eq!(state(&e, this), (0x9000, 5, 5));

        e.call_log = Some(vec![]);
        e.call(0x005e_f910, &args![this, 6u32, 0u32]);
        assert_eq!(state(&e, this), (0x9000, 0, 6));
        assert!(calls(&e, CONSTRUCT_LEG_ELEMENTS).is_empty());
    }
}

#[cfg(test)]
mod n041_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn entry_constructors_fill_their_defaults() {
        let mut e = Engine::new();
        let entry = e.mem.alloc(0x38);
        for i in 0..0xe {
            e.mem.set_u32(entry + 4 * i, 0x1234_5678);
        }
        assert_eq!(e.call(0x005e_f7f0, &args![entry]).u32(), entry);
        assert_eq!(e.mem.u32(entry), 0xffff_ffff);
        assert_eq!(e.mem.u32(entry + 4), 0x1200_ffff);
        for offset in (8..0x20).step_by(4) {
            assert_eq!(e.mem.u32(entry + offset), 0);
        }
        assert_eq!(e.mem.u32(entry + 0x20), 0x1234_5678);

        let entry = e.mem.alloc(0x28);
        for i in 0..0xa {
            e.mem.set_u32(entry + 4 * i, 0x1234_5678);
        }
        assert_eq!(e.call(0x005e_fa40, &args![entry]).u32(), entry);
        assert_eq!(e.mem.u32(entry), 0xffff_ffff);
        assert_eq!(e.mem.u32(entry + 4), 0x1234_5678);
        assert_eq!(e.mem.u32(entry + 0x18), 0x1234_5678);
        for offset in [0x1cu32, 0x20, 0x24] {
            assert_eq!(e.mem.u32(entry + offset), 0);
        }
    }

    #[test]
    fn element_construction_skips_a_null_place() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x38 * 3);
        // The second place answers null.
        e.register_double(PLACE_ELEMENT, move |_, a| {
            ret(if a[1] == block + 0x38 { 0 } else { a[1] })
        });
        e.call_log = Some(vec![]);
        e.call(0x005e_f750, &args![0u32, block, 3u32]);
        assert_eq!(
            calls(&e, PLACE_ELEMENT),
            vec![
                vec![0x38, block],
                vec![0x38, block + 0x38],
                vec![0x38, block + 0x70]
            ]
        );
        assert_eq!(e.mem.u16(block), 0xffff);
        assert_eq!(e.mem.u16(block + 0x38), 0);
        assert_eq!(e.mem.u16(block + 0x70), 0xffff);
        e.register(PLACE_ELEMENT, |_, a| ret(a[1]));
        let block = e.mem.alloc(0x28 * 2);
        e.call(0x005e_f9a0, &args![0u32, block, 2u32]);
        assert_eq!(e.mem.u16(block), 0xffff);
        assert_eq!(e.mem.u16(block + 0x28), 0xffff);
        e.register(PLACE_ELEMENT, |_, _| ret(0));
        let block = e.mem.alloc(0x28);
        e.call(0x005e_f9a0, &args![0u32, block, 1u32]);
        assert_eq!(e.mem.u16(block), 0);
    }

    #[test]
    fn resizes_allocate_reallocate_or_move() {
        for (addr, mover) in [(0x005e_f860u32, 0x005e_fe70u32), (0x005e_fa80, 0x0069_28a0)] {
            let mut e = Engine::new();
            let vtable = e.mem.alloc(0x10);
            e.mem.set_u32(vtable + 4, 0x7000_0004);
            e.mem.set_u32(vtable + 0xc, 0x7000_000c);
            e.register(0x7000_0004, |_, a| ret(0x9000 + a[1]));
            e.register(0x7000_000c, |_, a| ret(a[1] + a[2]));
            e.register(mover, |_, _| Ret::default());
            e.register(0x006a_8500, |_, _| Ret::default());
            let this = e.mem.alloc(0x10);
            e.mem.set_u32(this, vtable);
            // No buffer: allocate and set the capacity.
            e.call(addr, &args![this, 4u32, 0u32]);
            assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 0xc)), (0x9004, 4));
            // Count equal to the capacity: in place, the capacity kept.
            e.call(addr, &args![this, 8u32, 4u32]);
            assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 0xc)), (0x900c, 4));
            // Otherwise: a new buffer, the elements moved, the old freed.
            e.call_log = Some(vec![]);
            e.call(addr, &args![this, 6u32, 2u32]);
            assert_eq!(calls(&e, mover), vec![vec![this, 0x9006, 0x900c, 2]]);
            assert_eq!(calls(&e, 0x006a_8500), vec![vec![this]]);
            assert_eq!((e.mem.u32(this + 4), e.mem.u32(this + 0xc)), (0x9006, 4));
        }
    }
}
