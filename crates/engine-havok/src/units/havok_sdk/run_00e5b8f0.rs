//! `Havok SDK/run_00e5b8f0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::havok_sdk::run_00d6bf60::{load_vec, store_vec};

/// `00c85750` (hkMemoryRouter lookup): no arguments, the router in `EAX`; its `+0x10` is the heap
/// allocator object.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkArrayUtil::_reserve` (Xbox PDB, `00c908c0`): cdecl, `(allocator, array, count, element size)`.
const RESERVE: u32 = 0x00c9_08c0;
/// `hkMemUtil::memMove` (Xbox PDB, `00d2cc70`): cdecl, `(destination, source, byte count)`.
const MEM_MOVE: u32 = 0x00d2_cc70;
/// `hkReferencedObject::addReference` (Xbox PDB, `00c90510`): thiscall, `this` only.
const ADD_REFERENCE: u32 = 0x00c9_0510;
/// `hkpConstraintUtils::getConstraintPivots` (Xbox PDB, `00e5ad40`): cdecl `(constraint, first,
/// second)`, 0 when it filled the two vectors.
const GET_CONSTRAINT_PIVOTS: u32 = 0x00e5_ad40;
/// `00dd04d0`: thiscall on the world, one word (a body), `RET 4`.
const ADD_BODY: u32 = 0x00dd_04d0;
/// `00d53d30`: thiscall on the target, `(vector, vector)`, `RET 8`.
const PASS_VECTORS: u32 = 0x00d5_3d30;
/// `00d10710`: thiscall on a pointer array, `(allocator, index, pointer to the item)`, `RET 0xc`.
const INSERT_POINTER: u32 = 0x00d1_0710;

/// The heap allocator object of the memory router (`00c85750`, then `+0x10`).
fn heap_allocator(e: &mut Engine) -> u32 {
    let router = e.call(MEMORY_ROUTER, &[]).u32();
    e.mem.u32(router.wrapping_add(0x10))
}

// Translated from 00e5b8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, `(constraint, world, target)`: returns 0 when the constraint's two bodies (`+0x14`,
/// `+0x18`) are dealt with, else 1. `00e5ad40` gives the two pivot vectors (`A`, `B`); when it
/// fails (nonzero) the result is 1. An empty body array of `world` (`+0x38`, count `+0x3c`) first
/// gets the first body (`00dd04d0`). The last body of the array is compared with the two bodies:
/// the first one gives `00d53d30` on `target` with `(A, B)` and adds the second body to the world;
/// the second gives `(B, A)` and adds the first body. Otherwise the first body of the array is
/// compared the same way: when it is the constraint's first body the pair `(B, A)` (two vectors),
/// when it is the second the pair `(A, B)`, is inserted at index 0 of the 32-byte element array
/// at `target + 0x18` (`00e5c440`) and the other body is added to the world's array
/// (`00e5c460`). Anything else returns 1.
pub fn fn_00e5b8f0(e: &mut Engine, constraint: u32, world: u32, target: u32) -> u32 {
    e.with_stack(0x40, |e, frame| {
        let first = frame.addr();
        let second = first.wrapping_add(0x10);
        let pair = first.wrapping_add(0x20);
        let failed = e
            .call(GET_CONSTRAINT_PIVOTS, &args![constraint, first, second])
            .u32();
        if failed != 0 {
            return 1;
        }
        if e.mem.u32(world.wrapping_add(0x3c)) == 0 {
            let body = e.mem.u32(constraint.wrapping_add(0x14));
            e.call(ADD_BODY, &args![world, body]);
        }
        let bodies = e.mem.u32(world.wrapping_add(0x38));
        let count = e.mem.u32(world.wrapping_add(0x3c));
        let last = e
            .mem
            .u32(bodies.wrapping_add(count.wrapping_mul(4)).wrapping_sub(4));
        if last == e.mem.u32(constraint.wrapping_add(0x14)) {
            e.call(PASS_VECTORS, &args![target, first, second]);
            let body = e.mem.u32(constraint.wrapping_add(0x18));
            e.call(ADD_BODY, &args![world, body]);
            return 0;
        }
        let second_body = e.mem.u32(constraint.wrapping_add(0x18));
        if last == second_body {
            e.call(PASS_VECTORS, &args![target, second, first]);
            let body = e.mem.u32(constraint.wrapping_add(0x14));
            e.call(ADD_BODY, &args![world, body]);
            return 0;
        }
        let front = e.mem.u32(bodies);
        let (vectors, other) = if front == e.mem.u32(constraint.wrapping_add(0x14)) {
            ((second, first), second_body)
        } else if front == second_body {
            ((first, second), e.mem.u32(constraint.wrapping_add(0x14)))
        } else {
            return 1;
        };
        let (lower, upper) = (load_vec(e, vectors.0), load_vec(e, vectors.1));
        store_vec(e, pair, lower);
        store_vec(e, pair.wrapping_add(0x10), upper);
        let allocator = heap_allocator(e);
        fn_00e5c440(e, Ptr::new(target.wrapping_add(0x18)), allocator, 0, pair);
        fn_00e5c460(e, Ptr::new(world), other);
        0
    })
}

// Translated from 00e5c370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0x14`: as `00e4aaa0` for elements of 32 bytes: replaces `remove` elements at
/// `index` of the `hkArray` at `this` (`data`, `size`, `capacity | flags`) by the `insert`
/// elements at `items`. The array is reserved (`00c908c0`, the larger of twice the capacity and
/// the new size, element size 0x20) when the capacity bits are below the new size; the tail
/// moves with `memMove` (`00d2cc70`) and the new elements are copied in as two vectors each
/// (skipped for a null destination); the size is stored.
pub fn fn_00e5c370(
    e: &mut Engine,
    this: Ptr,
    allocator: u32,
    index: i32,
    remove: i32,
    items: u32,
    insert: i32,
) {
    let this = this.addr();
    let size = e.mem.i32(this.wrapping_add(4));
    let new_size = size.wrapping_sub(remove).wrapping_add(insert);
    let tail = size.wrapping_sub(index).wrapping_sub(remove);
    let capacity = (e.mem.u32(this.wrapping_add(8)) & 0x3fff_ffff) as i32;
    if capacity < new_size {
        let doubled = capacity.wrapping_add(capacity);
        let wanted = if new_size < doubled {
            doubled
        } else {
            new_size
        };
        e.call(RESERVE, &args![allocator, this, wanted, 0x20u32]);
    }
    let data = e.mem.u32(this);
    let source = (remove.wrapping_add(index) as u32)
        .wrapping_shl(5)
        .wrapping_add(data);
    let destination = (index.wrapping_add(insert) as u32)
        .wrapping_shl(5)
        .wrapping_add(data);
    e.call(
        MEM_MOVE,
        &args![destination, source, (tail as u32).wrapping_shl(5)],
    );
    let mut cursor = (index as u32).wrapping_shl(5).wrapping_add(e.mem.u32(this));
    for element in 0..insert.max(0) as u32 {
        if cursor != 0 {
            let from = items.wrapping_add(32 * element);
            let low = load_vec(e, from);
            store_vec(e, cursor, low);
            let high = load_vec(e, from.wrapping_add(0x10));
            store_vec(e, cursor.wrapping_add(0x10), high);
        }
        cursor = cursor.wrapping_add(32);
    }
    e.mem.set_i32(this.wrapping_add(4), new_size);
}

// Translated from 00e5c420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0x10`: inserts `count` elements from `items` at `index`: `00e5c370` with a
/// remove count of 0.
pub fn fn_00e5c420(e: &mut Engine, this: Ptr, allocator: u32, index: u32, items: u32, count: u32) {
    fn_00e5c370(e, this, allocator, index as i32, 0, items, count as i32);
}

// Translated from 00e5c440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 0xc`: inserts the one element at `items` at `index`: `00e5c420` with a count of 1.
pub fn fn_00e5c440(e: &mut Engine, this: Ptr, allocator: u32, index: u32, items: u32) {
    fn_00e5c420(e, this, allocator, index, items, 1);
}

// Translated from 00e5c460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall, `RET 4`: inserts `object` at index 0 of the pointer array at `this + 0x38`
/// (`00d10710`, with the allocator and the address of a stack word holding the pointer) and takes a
/// reference on it (`00c90510`, with no null check).
pub fn fn_00e5c460(e: &mut Engine, this: Ptr, object: u32) {
    let this = this.addr();
    let allocator = heap_allocator(e);
    let object = e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, object);
        e.call(
            INSERT_POINTER,
            &args![this.wrapping_add(0x38), allocator, 0u32, slot],
        );
        e.mem.u32(slot)
    });
    e.call(ADD_REFERENCE, &args![object]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00e5_b8f0, fn_00e5b8f0(u32, u32, u32) -> u32),
        entry!(0x00e5_c370, fn_00e5c370(Ptr, u32, i32, i32, u32, i32)),
        entry!(0x00e5_c420, fn_00e5c420(Ptr, u32, u32, u32, u32)),
        entry!(0x00e5_c440, fn_00e5c440(Ptr, u32, u32, u32)),
        entry!(0x00e5_c460, fn_00e5c460(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;
    /// What the vector double saw: the target and the two vectors.
    type VectorCalls = Rc<RefCell<Vec<(u32, [f32; 4], [f32; 4])>>>;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.register(MEMORY_ROUTER, |_, _| Ret {
            eax: 0x0126_0000,
            ..Ret::default()
        });
        e.mem.set_u32(0x0126_0010, 0x4444);
        e.register(MEM_MOVE, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e
    }

    fn record(e: &mut Engine, addresses: &[u32]) -> Calls {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in addresses {
            let address = *address;
            let log = Rc::clone(&calls);
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        calls
    }

    /// An array of 32-byte elements whose element `k` has the words `k * 100 + 1 ..= 8`.
    fn array32(e: &mut Engine, size: u32, capacity: u32) -> u32 {
        let array = e.mem.alloc(12);
        let data = e.mem.alloc(32 * capacity.max(size) + 32);
        for element in 0..size {
            for word in 0..8 {
                e.mem
                    .set_u32(data + 32 * element + 4 * word, element * 100 + word + 1);
            }
        }
        e.mem.set_u32(array, data);
        e.mem.set_u32(array + 4, size);
        e.mem.set_u32(array + 8, capacity);
        array
    }

    fn element(e: &Engine, array: u32, element: u32) -> Vec<u32> {
        let data = e.mem.u32(array);
        (0..8)
            .map(|word| e.mem.u32(data + 32 * element + 4 * word))
            .collect()
    }

    #[test]
    fn insert_shifts_the_tail_by_32_byte_elements() {
        let mut e = engine();
        let array = array32(&mut e, 2, 8);
        let items = array32(&mut e, 1, 1);
        let new_item = e.mem.u32(items);
        // Insert the element (1..=8) at index 1.
        e.call(
            0x00e5_c370,
            &args![array, 0x99u32, 1i32, 0i32, new_item, 1i32],
        );
        assert_eq!(e.mem.u32(array + 4), 3);
        assert_eq!(element(&e, array, 0), vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(element(&e, array, 1), vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(
            element(&e, array, 2),
            vec![101, 102, 103, 104, 105, 106, 107, 108]
        );
    }

    #[test]
    fn replacing_grows_the_array_first() {
        let mut e = engine();
        let reserved = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&reserved);
        e.register_double(RESERVE, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            let data = e.mem.alloc(32 * a[2] + 32);
            let old = e.mem.u32(a[1]);
            let bytes = e.mem.bytes(old, 64);
            e.mem.write(data, &bytes);
            e.mem.set_u32(a[1], data);
            e.mem.set_u32(a[1] + 8, a[2]);
            Ret::default()
        });
        let array = array32(&mut e, 2, 2);
        let items = array32(&mut e, 2, 2);
        let new_items = e.mem.u32(items);
        // Replace element 0 by two elements: the new size 3 exceeds the capacity 2, so 4 is asked.
        e.call(
            0x00e5_c370,
            &args![array, 0x99u32, 0i32, 1i32, new_items, 2i32],
        );
        assert_eq!(*reserved.borrow(), vec![vec![0x99, array, 4, 0x20]]);
        assert_eq!(e.mem.u32(array + 4), 3);
        assert_eq!(element(&e, array, 0), vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(
            element(&e, array, 1),
            vec![101, 102, 103, 104, 105, 106, 107, 108]
        );
        assert_eq!(
            element(&e, array, 2),
            vec![101, 102, 103, 104, 105, 106, 107, 108]
        );
    }

    #[test]
    fn the_wrappers_insert_with_the_right_counts() {
        let mut e = engine();
        let array = array32(&mut e, 1, 4);
        let items = array32(&mut e, 2, 2);
        let new_items = e.mem.u32(items);
        e.call(0x00e5_c420, &args![array, 0x99u32, 1u32, new_items, 2u32]);
        assert_eq!(e.mem.u32(array + 4), 3);
        assert_eq!(
            element(&e, array, 2),
            vec![101, 102, 103, 104, 105, 106, 107, 108]
        );
        e.call(0x00e5_c440, &args![array, 0x99u32, 0u32, new_items]);
        assert_eq!(e.mem.u32(array + 4), 4);
        assert_eq!(element(&e, array, 0), vec![1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(element(&e, array, 1), vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn a_pointer_is_inserted_and_referenced() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(INSERT_POINTER, move |e, a| {
            log.borrow_mut().push((a.to_vec(), e.mem.u32(a[3])));
            Ret::default()
        });
        let calls = record(&mut e, &[ADD_REFERENCE]);
        e.call(0x00e5_c460, &args![0x1000u32, 0x7777u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(&seen[0].0[..3], &[0x1038, 0x4444, 0]);
        assert_eq!(seen[0].1, 0x7777);
        assert_eq!(*calls.borrow(), vec![(ADD_REFERENCE, vec![0x7777])]);
    }

    /// A constraint with the bodies `1111` and `2222`, a world whose body array holds `bodies`
    /// and a target with an empty 32-byte array at `+0x18`.
    fn setup(e: &mut Engine, bodies: &[u32]) -> (u32, u32, u32) {
        let constraint = e.mem.alloc(0x30);
        e.mem.set_u32(constraint + 0x14, 0x1111);
        e.mem.set_u32(constraint + 0x18, 0x2222);
        let world = e.mem.alloc(0x50);
        let list = e.mem.alloc(4 * bodies.len() as u32 + 8);
        for (index, body) in bodies.iter().enumerate() {
            e.mem.set_u32(list + 4 * index as u32, *body);
        }
        e.mem.set_u32(world + 0x38, list);
        e.mem.set_u32(world + 0x3c, bodies.len() as u32);
        let target = e.mem.alloc(0x50);
        let data = e.mem.alloc(0x100);
        e.mem.set_u32(target + 0x18, data);
        e.mem.set_u32(target + 0x1c, 0);
        e.mem.set_u32(target + 0x20, 8);
        (constraint, world, target)
    }

    /// The pivot double: writes `A = (1, ...)` and `B = (2, ...)` and answers `answer`.
    fn pivots(e: &mut Engine, answer: u32) {
        e.register_double(GET_CONSTRAINT_PIVOTS, move |e, a| {
            store_vec(e, a[1], [1.0; 4]);
            store_vec(e, a[2], [2.0; 4]);
            Ret {
                eax: answer,
                ..Ret::default()
            }
        });
    }

    /// `00d53d30` records the vectors it is given.
    fn pass_double(e: &mut Engine) -> VectorCalls {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(PASS_VECTORS, move |e, a| {
            log.borrow_mut()
                .push((a[0], load_vec(e, a[1]), load_vec(e, a[2])));
            Ret::default()
        });
        seen
    }

    #[test]
    fn failed_pivots_return_one() {
        let mut e = engine();
        pivots(&mut e, 1);
        let calls = record(&mut e, &[ADD_BODY]);
        let (constraint, world, target) = setup(&mut e, &[0x1111]);
        assert_eq!(
            e.call(0x00e5_b8f0, &args![constraint, world, target]).u32(),
            1
        );
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn last_body_equal_to_the_first_passes_a_then_b() {
        let mut e = engine();
        pivots(&mut e, 0);
        let seen = pass_double(&mut e);
        let calls = record(&mut e, &[ADD_BODY]);
        let (constraint, world, target) = setup(&mut e, &[0x1111]);
        assert_eq!(
            e.call(0x00e5_b8f0, &args![constraint, world, target]).u32(),
            0
        );
        assert_eq!(*seen.borrow(), vec![(target, [1.0; 4], [2.0; 4])]);
        assert_eq!(*calls.borrow(), vec![(ADD_BODY, vec![world, 0x2222])]);
    }

    #[test]
    fn last_body_equal_to_the_second_passes_b_then_a() {
        let mut e = engine();
        pivots(&mut e, 0);
        let seen = pass_double(&mut e);
        let calls = record(&mut e, &[ADD_BODY]);
        let (constraint, world, target) = setup(&mut e, &[0x9999, 0x2222]);
        assert_eq!(
            e.call(0x00e5_b8f0, &args![constraint, world, target]).u32(),
            0
        );
        assert_eq!(*seen.borrow(), vec![(target, [2.0; 4], [1.0; 4])]);
        assert_eq!(*calls.borrow(), vec![(ADD_BODY, vec![world, 0x1111])]);
    }

    #[test]
    fn empty_world_gets_the_first_body_before_the_comparison() {
        let mut e = engine();
        pivots(&mut e, 0);
        pass_double(&mut e);
        let calls = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&calls);
        e.register_double(ADD_BODY, move |e, a| {
            log.borrow_mut().push(a.to_vec());
            // The world's array gains the body.
            let list = e.mem.u32(a[0] + 0x38);
            e.mem.set_u32(list, a[1]);
            e.mem.set_u32(a[0] + 0x3c, 1);
            Ret::default()
        });
        let (constraint, world, target) = setup(&mut e, &[]);
        assert_eq!(
            e.call(0x00e5_b8f0, &args![constraint, world, target]).u32(),
            0
        );
        // First the first body is added; then the last body (1111) matches and 2222 is added.
        assert_eq!(
            *calls.borrow(),
            vec![vec![world, 0x1111], vec![world, 0x2222]]
        );
    }

    #[test]
    fn first_body_of_the_array_inserts_the_pair_and_the_other_body() {
        let mut e = engine();
        pivots(&mut e, 0);
        let pointer_calls = record(&mut e, &[INSERT_POINTER, ADD_REFERENCE]);
        // The array holds the constraint's first body at the front and another one last.
        let (constraint, world, target) = setup(&mut e, &[0x1111, 0x9999]);
        assert_eq!(
            e.call(0x00e5_b8f0, &args![constraint, world, target]).u32(),
            0
        );
        // The pair (B, A) is element 0 of the target's array.
        let data = e.mem.u32(target + 0x18);
        assert_eq!(e.mem.u32(target + 0x1c), 1);
        assert_eq!(load_vec(&e, data), [2.0; 4]);
        assert_eq!(load_vec(&e, data + 0x10), [1.0; 4]);
        let calls = pointer_calls.borrow();
        assert_eq!(calls[0].0, INSERT_POINTER);
        assert_eq!(calls[0].1[0], world + 0x38);
        assert_eq!(calls[1], (ADD_REFERENCE, vec![0x2222]));
    }

    #[test]
    fn second_body_at_the_front_inserts_a_then_b() {
        let mut e = engine();
        pivots(&mut e, 0);
        let pointer_calls = record(&mut e, &[INSERT_POINTER, ADD_REFERENCE]);
        let (constraint, world, target) = setup(&mut e, &[0x2222, 0x9999]);
        assert_eq!(
            e.call(0x00e5_b8f0, &args![constraint, world, target]).u32(),
            0
        );
        let data = e.mem.u32(target + 0x18);
        assert_eq!(load_vec(&e, data), [1.0; 4]);
        assert_eq!(load_vec(&e, data + 0x10), [2.0; 4]);
        assert_eq!(pointer_calls.borrow()[1], (ADD_REFERENCE, vec![0x1111]));
    }

    #[test]
    fn unrelated_bodies_return_one() {
        let mut e = engine();
        pivots(&mut e, 0);
        let (constraint, world, target) = setup(&mut e, &[0x8888, 0x9999]);
        assert_eq!(
            e.call(0x00e5_b8f0, &args![constraint, world, target]).u32(),
            1
        );
    }
}
