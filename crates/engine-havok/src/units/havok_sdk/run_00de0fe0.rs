//! `Havok SDK/run_00de0fe0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `hkpShapeDisplayBuilder::hkpShapeDisplayBuilderEnvironment` constructor (`00ddfb70`, thiscall
/// on a two-word record).
const ENVIRONMENT_CONSTRUCTOR: u32 = 0x00dd_fb70;
/// `hkpShapeDisplayBuilder` constructor (`00ddfb80`, thiscall, `RET 4`: the environment).
const BUILDER_CONSTRUCTOR: u32 = 0x00dd_fb80;
/// `hkpShapeDisplayBuilder::buildDisplayGeometries` (`00de0f90`, thiscall, `RET 8`: the shape and
/// the array of geometries).
const BUILD_DISPLAY_GEOMETRIES: u32 = 0x00de_0f90;
/// The routine `00c983e0` (thiscall on the object whose pointer is at `012681e0`, `RET 0x14`: the
/// array, two words of the caller, 0 and the shape).
const PROCESS_GEOMETRIES: u32 = 0x00c9_83e0;
/// The word at `012681e0`: the pointer to the object that `00c983e0` runs on.
const DISPLAY_OBJECT: u32 = 0x0126_81e0;
/// The memory router (`00c85750`; its word at `+0x10` is the allocator freed through).
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// The vtable stored by the destructor below (its RTTI name is `hkBaseObject`).
const BASE_OBJECT_VTABLE: u32 = 0x0102_e388;

// Translated from 00de0fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: `shape`, `second`, `third`). The code keeps one 0x20-byte
/// frame: an environment record (two words, `00ddfb70`) at its start, a display builder
/// (`00ddfb80` with the environment) at `+0xc`, and, on top of the environment once the builder
/// has copied it, an `hkArray` of geometries (data 0, size 0, capacity word `0x80000000`: not
/// owned). It fills the array with `buildDisplayGeometries(shape, array)` (`00de0f90`), runs
/// `00c983e0` on the object whose pointer is at `012681e0` with `(array, second, third, 0,
/// shape)`, then destroys the array: while the size is not 0 it calls the first element's
/// virtual function 0 with 1 (when the element is not 0), lowers the size by one and, while the
/// size is still not 0, moves the last element into the first place. Finally it sets the size to
/// 0 and, unless the capacity word's sign bit is set, frees the array's storage through the
/// router's allocator (virtual function `+0x10`: data, capacity times 4).
pub fn fn_00de0fe0(e: &mut Engine, shape: u32, second: u32, third: u32) {
    // The exe's frame (`SUB ESP,0x20`, one push): 0x24 bytes, the environment and later the
    // geometry array at +0x4 (00de0fe4, 00de0ffd) and the builder at +0x10 (00de0ff0).
    e.with_stack(0x24, |e, frame| {
        let frame = frame.addr();
        let environment = frame + 4;
        let builder = frame + 0x10;
        let array = frame + 4;
        e.call(ENVIRONMENT_CONSTRUCTOR, &args![environment]);
        e.call(BUILDER_CONSTRUCTOR, &args![builder, environment]);
        e.mem.set_u32(array, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, 0x8000_0000);
        e.call(BUILD_DISPLAY_GEOMETRIES, &args![builder, shape, array]);
        let target = e.mem.u32(DISPLAY_OBJECT);
        e.call(
            PROCESS_GEOMETRIES,
            &args![target, array, second, third, 0u32, shape],
        );
        if e.mem.u32(array + 4) != 0 {
            loop {
                let data = e.mem.u32(array);
                let first = e.mem.u32(data);
                if first != 0 {
                    e.vcall(first, 0, &args![1u32]);
                }
                let data = e.mem.u32(array);
                let size = e.mem.u32(array + 4).wrapping_sub(1);
                e.mem.set_u32(array + 4, size);
                if size == 0 {
                    break;
                }
                let last = e.mem.u32(data.wrapping_add(size.wrapping_mul(4)));
                e.mem.set_u32(data, last);
            }
        }
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let capacity = e.mem.u32(array + 8);
        e.mem.set_u32(array + 4, 0);
        if (capacity as i32) >= 0 {
            let data = e.mem.u32(array);
            e.vcall(allocator, 0x10, &args![data, (capacity & 0x3fff_ffff) << 2]);
        }
    });
}

// Translated from 00de10b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: `this`, `flags`): the scalar deleting
/// destructor of the object whose vtable is at `0102e388` (RTTI `hkBaseObject`). Stores that
/// vtable at `this` and, when bit 0 of `flags` is set, frees `this` through the router's
/// allocator (virtual function `+8`: `this` and the zero-extended 16-bit word at `this + 4`).
/// Answers `this`.
pub fn fn_00de10b0(e: &mut Engine, this: u32, flags: u32) -> u32 {
    e.mem.set_u32(this, BASE_OBJECT_VTABLE);
    if flags & 1 != 0 {
        let router = e.call(MEMORY_ROUTER, &args![]).u32();
        let allocator = e.mem.u32(router + 0x10);
        let size = u32::from(e.mem.u16(this + 4));
        e.vcall(allocator, 8, &args![this, size]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00de0fe0, fn_00de0fe0(u32, u32, u32)),
        entry!(0x00de10b0, fn_00de10b0(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const DESTRUCTOR: u32 = 0x7000_0001;
    const FREE_ARRAY: u32 = 0x7000_0002;
    const FREE_OBJECT: u32 = 0x7000_0003;

    type Log = Rc<RefCell<Vec<(&'static str, Vec<u32>)>>>;

    /// What a run of `fn_00de0fe0` left behind.
    struct Outcome {
        log: Log,
        /// The frame address the environment constructor was given.
        frame: u32,
        /// The three element objects (numbers 1 to 3 in `run`).
        objects: Vec<u32>,
        /// The array size word as the free function saw it.
        size_when_freed: Vec<u32>,
        /// The block the build double allocated for the array.
        block: u32,
    }

    /// Runs `fn_00de0fe0` with doubles for every callee. The build double fills the array with
    /// the objects numbered by `numbers` (0 is a null element) and the capacity word `capacity`.
    fn run(numbers: &[u32], capacity: u32) -> Outcome {
        let mut e = Engine::new();
        let log: Log = Rc::default();
        let frame = Rc::new(Cell::new(0u32));
        let block = Rc::new(Cell::new(0u32));
        let size_when_freed: Rc<RefCell<Vec<u32>>> = Rc::default();

        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_OBJECT);
        e.mem.set_u32(vtable + 0x10, FREE_ARRAY);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });

        let element_vtable = e.mem.alloc(8);
        e.mem.set_u32(element_vtable, DESTRUCTOR);
        let objects: Vec<u32> = (0..3).map(|_| e.mem.alloc(8)).collect();
        for object in &objects {
            e.mem.set_u32(*object, element_vtable);
        }
        e.map(DISPLAY_OBJECT, 4);
        e.mem.set_u32(DISPLAY_OBJECT, 0x5555);

        let seen = Rc::clone(&log);
        let frame_seen = Rc::clone(&frame);
        e.register_double(ENVIRONMENT_CONSTRUCTOR, move |e, a| {
            frame_seen.set(a[0]);
            seen.borrow_mut().push(("environment", a.to_vec()));
            e.mem.set_u32(a[0], 8);
            e.mem.set_u32(a[0] + 4, 8);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        let seen = Rc::clone(&log);
        e.register_double(BUILDER_CONSTRUCTOR, move |e, a| {
            let (first, second) = (e.mem.u32(a[1]), e.mem.u32(a[1] + 4));
            seen.borrow_mut()
                .push(("builder", vec![a[0], a[1], first, second]));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        let block_seen = Rc::clone(&block);
        let numbers = numbers.to_vec();
        let element_objects = objects.clone();
        e.register_double(BUILD_DISPLAY_GEOMETRIES, move |e, a| {
            let initial = vec![e.mem.u32(a[2]), e.mem.u32(a[2] + 4), e.mem.u32(a[2] + 8)];
            seen.borrow_mut()
                .push(("build", [vec![a[0], a[1], a[2]], initial].concat()));
            let at = e.mem.alloc(4 * numbers.len().max(1) as u32);
            block_seen.set(at);
            for (k, number) in numbers.iter().enumerate() {
                let object = match *number {
                    0 => 0,
                    n => element_objects[n as usize - 1],
                };
                e.mem.set_u32(at + 4 * k as u32, object);
            }
            e.mem.set_u32(a[2], at);
            e.mem.set_u32(a[2] + 4, numbers.len() as u32);
            e.mem.set_u32(a[2] + 8, capacity);
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(PROCESS_GEOMETRIES, move |e, a| {
            let mut words = a.to_vec();
            words.push(e.mem.u32(a[1] + 4));
            seen.borrow_mut().push(("process", words));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(DESTRUCTOR, move |_, a| {
            seen.borrow_mut().push(("destroy", a.to_vec()));
            Ret::default()
        });
        let seen = Rc::clone(&log);
        let frame_seen = Rc::clone(&frame);
        let sizes = Rc::clone(&size_when_freed);
        e.register_double(FREE_ARRAY, move |e, a| {
            sizes.borrow_mut().push(e.mem.u32(frame_seen.get() + 4));
            seen.borrow_mut().push(("free array", a[1..].to_vec()));
            Ret::default()
        });
        // The environment and the array sit at +0x4 of the exe's 0x24-byte frame.
        let pinned = e.next_stack_local(0x24);
        e.call(0x00de_0fe0, &args![0x1111u32, 0x2222u32, 0x3333u32]);
        assert_eq!(frame.get(), pinned + 4);
        let size_when_freed = size_when_freed.borrow().clone();
        Outcome {
            log,
            frame: frame.get(),
            objects,
            size_when_freed,
            block: block.get(),
        }
    }

    #[test]
    fn the_array_is_built_processed_and_destroyed_in_order() {
        let o = run(&[1, 2], 4);
        let [a, b] = [o.objects[0], o.objects[1]];
        let frame = o.frame;
        assert_eq!(
            *o.log.borrow(),
            vec![
                ("environment", vec![frame]),
                ("builder", vec![frame + 0xc, frame, 8, 8]),
                ("build", vec![frame + 0xc, 0x1111, frame, 0, 0, 0x8000_0000]),
                ("process", vec![0x5555, frame, 0x2222, 0x3333, 0, 0x1111, 2]),
                ("destroy", vec![a, 1]),
                ("destroy", vec![b, 1]),
                ("free array", vec![o.block, 16]),
            ]
        );
        assert_eq!(o.size_when_freed, vec![0]);
    }

    #[test]
    fn a_capacity_word_with_the_sign_bit_set_is_not_freed() {
        let o = run(&[3], 0x8000_0001);
        let names: Vec<&str> = o.log.borrow().iter().map(|entry| entry.0).collect();
        assert_eq!(
            names,
            ["environment", "builder", "build", "process", "destroy"]
        );
        assert!(o.size_when_freed.is_empty());
    }

    #[test]
    fn an_empty_array_destroys_nothing_but_is_still_freed() {
        let o = run(&[], 0x4000_0003);
        let log = o.log.borrow();
        assert_eq!(log[3].0, "process");
        assert_eq!(
            log[4],
            ("free array", vec![o.block, 12]),
            "capacity is the low 30 bits"
        );
        assert_eq!(log.len(), 5);
    }

    #[test]
    fn a_null_first_element_is_skipped_and_the_last_one_moves_forward() {
        let o = run(&[0, 2, 3], 3);
        let destroyed: Vec<Vec<u32>> = o
            .log
            .borrow()
            .iter()
            .filter(|entry| entry.0 == "destroy")
            .map(|entry| entry.1.clone())
            .collect();
        // [0, B, C]: skip 0, last (C) moves to the front: [C, B]; destroy C, then B moves to
        // the front: [B]; destroy B.
        assert_eq!(
            destroyed,
            vec![vec![o.objects[2], 1], vec![o.objects[1], 1]]
        );
    }

    #[test]
    fn the_scalar_deleting_destructor_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::default();
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 8, FREE_OBJECT);
        let allocator = e.mem.alloc(0x10);
        e.mem.set_u32(allocator, vtable);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.register_double(MEMORY_ROUTER, move |_, _| Ret {
            eax: router,
            ..Ret::default()
        });
        let seen = Rc::clone(&log);
        e.register_double(FREE_OBJECT, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this + 4, 0xffff_1234);
        assert_eq!(
            e.call(0x00de_10b0, &args![this, 0xffff_fffeu32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
        assert!(log.borrow().is_empty());
        e.mem.set_u32(this, 0);
        assert_eq!(e.call(0x00de_10b0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), BASE_OBJECT_VTABLE);
        // The allocator is `this` of the call; the block and its 16-bit size follow.
        assert_eq!(*log.borrow(), vec![vec![allocator, this, 0x1234]]);
    }
}
