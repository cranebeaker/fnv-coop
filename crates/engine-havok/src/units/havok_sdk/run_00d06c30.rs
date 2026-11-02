//! `Havok SDK/run_00d06c30` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): word +0x10 is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The C runtime `memcpy` (`00ec44d0`, cdecl `(destination, source, bytes)`).
const MEM_COPY: u32 = 0x00ec_44d0;

/// Moves a small array (data at +0, 16-bit size at +4, 16-bit capacity and flags at +6)
/// into a block of `capacity` elements from the heap allocator (slot 4), copies the
/// elements over and frees the old block through slot 8 when the array owned it (bit 15 of
/// the capacity word clear). The capacity word becomes the new capacity with bit 14 kept and
/// bit 15 cleared.
fn move_small_array(e: &mut Engine, array: Ptr, capacity: u32, element_size: u32) {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e
        .vcall(allocator, 4, &args![capacity.wrapping_mul(element_size)])
        .u32();
    let data = e.mem.u32(array.addr());
    let size = u32::from(e.mem.u16(array.addr() + 4));
    e.call(
        MEM_COPY,
        &args![block, data, size.wrapping_mul(element_size)],
    );
    let old_capacity = u32::from(e.mem.u16(array.addr() + 6));
    if old_capacity & 0x8000 == 0 {
        let bytes = (old_capacity & 0x3fff).wrapping_mul(element_size);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![data, bytes]);
    }
    let flags = (e.mem.u16(array.addr() + 6) & 0x4000) | capacity as u16;
    e.mem.set_u32(array.addr(), block);
    e.mem.set_u16(array.addr() + 6, flags);
}

// Translated from 00d06c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Grows a small array (16-bit size and capacity) to room for `capacity` elements of
/// `element_size` bytes; see the shared routine above for the exact steps. The class is
/// unconfirmed (it is a template instance of the Havok small array).
pub fn fn_00d06c30(e: &mut Engine, array: Ptr, capacity: u32, element_size: u32) {
    move_small_array(e, array, capacity, element_size);
}

// Translated from 00d06cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Doubles a small array's room (1 element when it holds none; otherwise twice its current
/// size, not its capacity) with `element_size`-byte elements, as `00d06c30` does. The class
/// is unconfirmed.
pub fn fn_00d06cb0(e: &mut Engine, array: Ptr, element_size: u32) {
    let size = u32::from(e.mem.u16(array.addr() + 4));
    let capacity = if size == 0 { 1 } else { size + size };
    move_small_array(e, array, capacity, element_size);
}

// Translated from 00d06d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks an object up through two virtual methods (the class is unconfirmed): with a null
/// `object` the answer is 0. Otherwise virtual method `+4` of `object` is called; a non-zero
/// answer is returned. Else, when the word at `this + 0xc` is not 0, virtual method `+0x10` of the
/// object it points to is called with `object` and a non-zero answer is returned. Otherwise 0.
pub fn fn_00d06d50(e: &mut Engine, this: Ptr, object: Ptr) -> u32 {
    if object.is_null() {
        return 0;
    }
    let answer = e.vcall(object.addr(), 4, &args![]).u32();
    if answer != 0 {
        return answer;
    }
    let next = e.mem.u32(this.addr() + 0xc);
    if next == 0 {
        return 0;
    }
    e.vcall(next, 0x10, &args![object]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d0_6c30, fn_00d06c30(Ptr, u32, u32)),
        entry!(0x00d0_6cb0, fn_00d06cb0(Ptr, u32)),
        entry!(0x00d0_6d50, fn_00d06d50(Ptr, Ptr) -> u32),
    ]
}

#[cfg(test)]
mod movement_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// An engine with the exe's data range and the import page mapped, as the tests run without the exe.
    fn test_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.map(0x00fd_f000, 0x1000);
        e
    }

    type Frees = Rc<RefCell<Vec<(u32, u32)>>>;

    /// An allocator whose slot 4 hands out fresh blocks (logging the size) and whose slot 8
    /// logs `(pointer, bytes)`; a copying `memcpy`.
    fn rig() -> (Engine, Frees, Rc<RefCell<Vec<u32>>>) {
        let mut e = test_engine();
        let frees: Frees = Rc::new(RefCell::new(Vec::new()));
        let allocations = Rc::new(RefCell::new(Vec::new()));
        let sizes = Rc::clone(&allocations);
        e.register_double(0x0900_0004, move |e, a| {
            sizes.borrow_mut().push(a[1]);
            Ret {
                eax: e.mem.alloc(a[1].max(4)),
                ..Ret::default()
            }
        });
        let freed = Rc::clone(&frees);
        e.register_double(0x0900_0008, move |_, a| {
            freed.borrow_mut().push((a[1], a[2]));
            Ret::default()
        });
        e.register(MEM_COPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, 0x0900_0004);
        e.mem.set_u32(vtable + 8, 0x0900_0008);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        (e, frees, allocations)
    }

    fn array(e: &mut Engine, size: u16, capacity_word: u16, element: u32) -> (u32, u32) {
        let header = e.mem.alloc(8);
        let data = e.mem.alloc(0x40 * element);
        for i in 0..u32::from(size) * element {
            e.mem.set_u8(data + i, (i % 251) as u8 + 1);
        }
        e.mem.set_u32(header, data);
        e.mem.set_u16(header + 4, size);
        e.mem.set_u16(header + 6, capacity_word);
        (header, data)
    }

    #[test]
    fn grow_moves_the_elements_and_frees_an_owned_block() {
        let (mut e, frees, allocations) = rig();
        let (header, data) = array(&mut e, 3, 0x4005, 0x30);
        let before = e.mem.bytes(data, 3 * 0x30);
        e.call(0x00d0_6c30, &args![header, 10u32, 0x30u32]);
        assert_eq!(*allocations.borrow(), vec![10 * 0x30]);
        let moved = e.mem.u32(header);
        assert_ne!(moved, data);
        assert_eq!(e.mem.bytes(moved, 3 * 0x30), before);
        // The old block (capacity 5 x 0x30 bytes) is freed; bit 14 is kept, bit 15 clear.
        assert_eq!(*frees.borrow(), vec![(data, 5 * 0x30)]);
        assert_eq!(e.mem.u16(header + 6), 0x4000 | 10);
    }

    #[test]
    fn grow_keeps_a_block_the_array_did_not_own() {
        let (mut e, frees, _) = rig();
        let (header, _) = array(&mut e, 2, 0x8004, 4);
        e.call(0x00d0_6c30, &args![header, 6u32, 4u32]);
        assert!(frees.borrow().is_empty());
        assert_eq!(e.mem.u16(header + 6), 6);
    }

    #[test]
    fn doubling_uses_the_size_or_one() {
        let (mut e, frees, allocations) = rig();
        let (header, data) = array(&mut e, 5, 0x0005, 8);
        e.call(0x00d0_6cb0, &args![header, 8u32]);
        assert_eq!(*allocations.borrow(), vec![10 * 8]);
        assert_eq!(e.mem.u16(header + 6), 10);
        assert_eq!(*frees.borrow(), vec![(data, 5 * 8)]);
        let (empty, _) = array(&mut e, 0, 0x8000, 8);
        e.call(0x00d0_6cb0, &args![empty, 8u32]);
        assert_eq!(allocations.borrow()[1], 8);
        assert_eq!(e.mem.u16(empty + 6), 1);
    }
}

#[cfg(test)]
mod tests_00d06d50 {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FIRST_SLOT_DOUBLE: u32 = 0x0900_0501;
    const SECOND_SLOT_DOUBLE: u32 = 0x0900_0511;

    /// An object whose slot 4 answers `first` and a holder (`this`) whose `+0xc` object has a slot
    /// `0x10` that answers `second`; returns the engine, `this`, the object and a call log.
    fn rig(
        first: u32,
        second: u32,
        has_next: bool,
    ) -> (Engine, u32, u32, Rc<RefCell<Vec<(u32, Vec<u32>)>>>) {
        let mut e = Engine::new();
        let log = Rc::new(RefCell::new(Vec::new()));
        let sink = Rc::clone(&log);
        e.register_double(FIRST_SLOT_DOUBLE, move |_, a| {
            sink.borrow_mut().push((1, a.to_vec()));
            Ret {
                eax: first,
                ..Ret::default()
            }
        });
        let sink = Rc::clone(&log);
        e.register_double(SECOND_SLOT_DOUBLE, move |_, a| {
            sink.borrow_mut().push((2, a.to_vec()));
            Ret {
                eax: second,
                ..Ret::default()
            }
        });
        let object_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(object_vtable + 4, FIRST_SLOT_DOUBLE);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, object_vtable);
        let next_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(next_vtable + 0x10, SECOND_SLOT_DOUBLE);
        let next = e.mem.alloc(0x10);
        e.mem.set_u32(next, next_vtable);
        let this = e.mem.alloc(0x20);
        if has_next {
            e.mem.set_u32(this + 0xc, next);
        }
        (e, this, object, log)
    }

    /// A null object gives 0 with no calls; a first answer wins; otherwise the next holder is
    /// asked with the object; no next holder gives 0.
    #[test]
    fn the_first_non_zero_answer_is_returned() {
        let (mut e, this, _, log) = rig(5, 6, true);
        assert_eq!(e.call(0x00d0_6d50, &args![this, 0u32]).u32(), 0);
        assert!(log.borrow().is_empty());
        let (mut e, this, object, log) = rig(5, 6, true);
        assert_eq!(e.call(0x00d0_6d50, &args![this, object]).u32(), 5);
        assert_eq!(*log.borrow(), vec![(1, vec![object])]);
        let (mut e, this, object, log) = rig(0, 6, true);
        assert_eq!(e.call(0x00d0_6d50, &args![this, object]).u32(), 6);
        let next = e.mem.u32(this + 0xc);
        assert_eq!(
            *log.borrow(),
            vec![(1, vec![object]), (2, vec![next, object])]
        );
        let (mut e, this, object, _) = rig(0, 6, false);
        assert_eq!(e.call(0x00d0_6d50, &args![this, object]).u32(), 0);
        let (mut e, this, object, log) = rig(0, 0, true);
        assert_eq!(e.call(0x00d0_6d50, &args![this, object]).u32(), 0);
        assert_eq!(log.borrow().len(), 2);
    }
}
