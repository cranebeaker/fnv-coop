//! `Havok SDK/hkdisplaygeometry.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The display shapes (`hkDisplayCone`, `hkDisplayConvex`, `hkDisplayCylinder`,
//! `hkDisplayPlane`) share this base: a 16-bit count at `+6`, a pointer at `+8` to the built
//! geometry (an `hkGeometry`: two `hkArray`s of 16-byte elements, vertices at `+0` and index records at `+0xc`,
//! in a 0x18-byte block), a 4x4 transform at `+0x10` and a shape kind word at `+0x50`. The
//! helpers below are the pieces of that layout the derived classes repeat.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): word +0x10 is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`, cdecl `(allocator, array, capacity, element size)`).
const ARRAY_RESERVE: u32 = 0x00c9_08c0;
/// `hkGeometry::~hkGeometry` (Xbox PDB, `00cce530`, `this` only): the destructor of the object the
/// geometry pointer at `+8` points to.
const GEOMETRY_DESTRUCTOR: u32 = 0x00cc_e530;
/// The vtable the destructor writes first (this class's own, as a destructor does) and the one
/// it writes last (the base class's; its identity is unconfirmed).
const OWN_VTABLE: u32 = 0x010c_e2b4;
const BASE_VTABLE: u32 = 0x0102_e388;
/// Size of the geometry block the destructor returns to the allocator.
const GEOMETRY_BLOCK_SIZE: u32 = 0x18;

/// Writes the transform every display shape starts with: the four rows at `+0x10`, `+0x20`,
/// `+0x30` and `+0x40` zeroed, then `1.0` (from `FLD1`) at `+0x10`, `+0x24` and `+0x38`, the
/// diagonal of the 3x3 part.
pub(crate) fn reset_transform(e: &mut Engine, this: u32) {
    for row in [0x10, 0x20, 0x30, 0x40] {
        for lane in 0..4 {
            e.mem.set_u32(this + row + 4 * lane, 0);
        }
    }
    for diagonal in [0x10, 0x24, 0x38] {
        e.mem.set_f32(this + diagonal, 1.0);
    }
}

/// Appends one 16-byte element to an `hkArray` the way the display code inlines it, answering
/// the address of the new element (its contents are not written). The memory router is asked
/// for its allocator first; when the size `+4` has reached the capacity `+8` (masked with
/// `0x3fffffff`) the array is reserved to the larger of `size + 1` and twice the capacity
/// (`00c908c0`, element size 0x10; signed compares); the size goes up by one.
pub(crate) fn push_back_slot(e: &mut Engine, array: u32) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let size = e.mem.u32(array + 4);
    let capacity = e.mem.u32(array + 8) & 0x3fff_ffff;
    let wanted = size.wrapping_add(1);
    if (capacity as i32) < (wanted as i32) {
        let doubled = capacity.wrapping_add(capacity);
        let new_capacity = (wanted as i32).max(doubled as i32) as u32;
        e.call(
            ARRAY_RESERVE,
            &args![allocator, array, new_capacity, 0x10u32],
        );
    }
    let counted = e.mem.u32(array + 4).wrapping_add(1);
    e.mem.set_u32(array + 4, counted);
    size.wrapping_mul(16).wrapping_add(e.mem.u32(array))
}

/// Allocates the geometry block a display shape builds into: the router's allocator (slot `4`,
/// one argument, the 0x18 bytes) hands the block out and, unless it answers null, the two empty
/// `hkArray`s are written (`size 0`, capacity word `0x80000000`: no capacity, memory not owned)
/// at `+0` and `+0xc`. Answers the block, or null.
pub(crate) fn allocate_geometry_block(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let block = e.vcall(allocator, 4, &args![GEOMETRY_BLOCK_SIZE]).u32();
    if block != 0 {
        for array in [0, 0xc] {
            e.mem.set_u32(block + array, 0);
            e.mem.set_u32(block + array + 4, 0);
            e.mem.set_u32(block + array + 8, 0x8000_0000);
        }
    }
    block
}

// Translated from 00da2560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkDisplayGeometry::~hkDisplayGeometry` (Xbox PDB): writes this class's vtable, and when the
/// geometry block at `+8` exists runs its destructor (`00cce530`), returns the block (0x18
/// bytes) to the router's allocator (slot `8`, `(block, size)`) and clears `+8`; then writes
/// the base class's vtable.
pub fn hk_display_geometry_destructor(e: &mut Engine, this: Ptr) {
    let block = e.mem.u32(this.addr() + 8);
    e.mem.set_u32(this.addr(), OWN_VTABLE);
    if block != 0 {
        e.call(GEOMETRY_DESTRUCTOR, &args![block]);
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        e.vcall(allocator, 8, &args![block, GEOMETRY_BLOCK_SIZE]);
        e.mem.set_u32(this.addr() + 8, 0);
    }
    e.mem.set_u32(this.addr(), BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00da_2560, hk_display_geometry_destructor(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    const FREE_SLOT_DOUBLE: u32 = 0x0900_0011;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;

    /// A router whose allocator frees through a double; the double logs `(block, size)`.
    fn install_router(e: &mut Engine) -> (u32, Log) {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let free_log = Rc::clone(&log);
        e.register_double(FREE_SLOT_DOUBLE, move |_, a| {
            free_log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        (allocator, log)
    }

    #[test]
    fn hk_display_geometry_destructor_without_a_block_only_swaps_the_vtables() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x00da_2560, &args![this]);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert!(e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .all(|c| c.0 == 0x00da_2560));
    }

    #[test]
    fn hk_display_geometry_destructor_destroys_and_frees_the_block() {
        let mut e = Engine::new();
        let (allocator, frees) = install_router(&mut e);
        let seen_vtable = Rc::new(RefCell::new(Vec::new()));
        let record = Rc::clone(&seen_vtable);
        let this = e.mem.alloc(0x20);
        let block = e.mem.alloc(0x18);
        e.mem.set_u32(this + 8, block);
        e.register_double(GEOMETRY_DESTRUCTOR, move |e, a| {
            record.borrow_mut().push((a.to_vec(), e.mem.u32(this)));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00da_2560, &args![this]);
        // The inner destructor ran after this class's vtable was written, with the block only.
        assert_eq!(*seen_vtable.borrow(), vec![(vec![block], OWN_VTABLE)]);
        assert_eq!(*frees.borrow(), vec![vec![allocator, block, 0x18]]);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this), BASE_VTABLE);
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| [GEOMETRY_DESTRUCTOR, MEMORY_ROUTER, FREE_SLOT_DOUBLE].contains(a))
            .collect();
        assert_eq!(
            order,
            vec![GEOMETRY_DESTRUCTOR, MEMORY_ROUTER, FREE_SLOT_DOUBLE]
        );
    }

    #[test]
    fn reset_transform_zeroes_four_rows_and_sets_the_diagonal() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        for offset in 0..0x58 {
            e.mem.set_u8(this + offset, 0xaa);
        }
        reset_transform(&mut e, this);
        for lane in 0..16u32 {
            let expected = if [0, 5, 10].contains(&lane) { 1.0 } else { 0.0 };
            assert_eq!(e.mem.f32(this + 0x10 + 4 * lane), expected, "lane {lane}");
        }
        // Words outside the four rows are untouched.
        assert_eq!(e.mem.u8(this + 0x0f), 0xaa);
        assert_eq!(e.mem.u8(this + 0x50), 0xaa);
    }

    /// A reserve double that moves the array to a fresh buffer of `capacity` elements
    /// (clearing the flag bits) and logs `(allocator, capacity, element size)`.
    fn install_reserve(e: &mut Engine) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let reserve_log = Rc::clone(&log);
        e.register_double(ARRAY_RESERVE, move |e, a| {
            reserve_log.borrow_mut().push(vec![a[0], a[2], a[3]]);
            let old = e.mem.u32(a[1]);
            let size = e.mem.u32(a[1] + 4);
            let fresh = e.mem.alloc(a[2] * a[3]);
            let bytes = if size == 0 {
                vec![]
            } else {
                e.mem.bytes(old, size * a[3])
            };
            e.mem.write(fresh, &bytes);
            e.mem.set_u32(a[1], fresh);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        log
    }

    #[test]
    fn push_back_slot_reserves_to_the_larger_of_one_more_and_double() {
        let mut e = Engine::new();
        let (allocator, _) = install_router(&mut e);
        let reserves = install_reserve(&mut e);
        let array = e.mem.alloc(0x10);
        // Empty array with capacity 0 (not owning its data): size + 1 = 1 beats double (0).
        e.mem.set_u32(array + 8, 0x8000_0000);
        let first = push_back_slot(&mut e, array);
        assert_eq!(*reserves.borrow(), vec![vec![allocator, 1, 0x10]]);
        assert_eq!(e.mem.u32(array + 4), 1);
        assert_eq!(first, e.mem.u32(array));
        // Full at capacity 1: size + 1 and double are both 2.
        let second = push_back_slot(&mut e, array);
        assert_eq!(reserves.borrow()[1], vec![allocator, 2, 0x10]);
        assert_eq!(second, e.mem.u32(array) + 0x10);
        // Full at capacity 2 (size 2): double is 4.
        let third = push_back_slot(&mut e, array);
        assert_eq!(reserves.borrow()[2], vec![allocator, 4, 0x10]);
        assert_eq!(third, e.mem.u32(array) + 0x20);
        // Room left: no reserve, the next slot follows.
        let fourth = push_back_slot(&mut e, array);
        assert_eq!(reserves.borrow().len(), 3);
        assert_eq!(fourth, e.mem.u32(array) + 0x30);
        assert_eq!(e.mem.u32(array + 4), 4);
    }

    #[test]
    fn push_back_slot_takes_one_more_when_double_is_smaller() {
        let mut e = Engine::new();
        let (allocator, _) = install_router(&mut e);
        let reserves = install_reserve(&mut e);
        let array = e.mem.alloc(0x10);
        let data = e.mem.alloc(0x100);
        // Size 5 in a capacity of 2 (corrupt but possible): size + 1 = 6 beats double = 4.
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, 5);
        e.mem.set_u32(array + 8, 2);
        push_back_slot(&mut e, array);
        assert_eq!(*reserves.borrow(), vec![vec![allocator, 6, 0x10]]);
    }

    /// Installs a router whose allocator (slot 4) logs the size asked and hands out a block
    /// of the given content, or null.
    fn install_allocating_router(e: &mut Engine, block: u32, log: &Log) {
        const ALLOCATE_SLOT_DOUBLE: u32 = 0x0900_0021;
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 4, ALLOCATE_SLOT_DOUBLE);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let allocate_log = Rc::clone(log);
        e.register_double(ALLOCATE_SLOT_DOUBLE, move |_, a| {
            allocate_log.borrow_mut().push(a.to_vec());
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn allocate_geometry_block_writes_two_empty_arrays() {
        let mut e = Engine::new();
        let block = e.mem.alloc(0x18);
        for offset in (0..0x18).step_by(4) {
            e.mem.set_u32(block + offset, 0xdead_beef);
        }
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        install_allocating_router(&mut e, block, &log);
        let got = allocate_geometry_block(&mut e);
        assert_eq!(got, block);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(log.borrow()[0][1..], [0x18]);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(block + 4 * i)).collect();
        assert_eq!(words, vec![0, 0, 0x8000_0000, 0, 0, 0x8000_0000]);
    }

    #[test]
    fn allocate_geometry_block_answers_null_without_touching_memory() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        install_allocating_router(&mut e, 0, &log);
        assert_eq!(allocate_geometry_block(&mut e), 0);
        assert_eq!(log.borrow().len(), 1);
    }
}
