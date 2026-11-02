//! `Havok SDK/run_00d217d0` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two unrelated pieces of Havok code the engine map could not place in a source file.
//!
//! `00d217d0` sits in slot `0x24` (`castRayWithCollectorImpl`, Xbox PDB `hkpShape`) of the
//! vtables of many shape collections (`010c7580`, `010cc388`, `010d0580`, ...): it casts a
//! ray against every child of the collection through the shape container interface at
//! `this + 0x10` (`hkpShapeContainer`, Xbox PDB: slot `0x08` `getFirstKey`, `0x0C`
//! `getNextKey`, `0x14` `getChildShape(key, buffer)`). It builds a 16-byte `hkpCdBody` on
//! the stack for each child (shape, shape key, the parent's motion at +8, the parent at
//! +0xC) and calls the child's own slot `0x24` with `(input, that body, collector)`. When
//! the input carries a filter object (+0x24, an `hkpRayShapeCollectionFilter` in Havok),
//! the filter's slot 0 decides per key whether the child is cast against.
//!
//! `00d21930` is a constructor: the `hkpThinBoxMotion` vtable `010c6e34` (RTTI
//! `.?AVhkpThinBoxMotion@@` confirmed) goes in after the `hkpBoxMotion` constructor.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TlsGetValue` import slot and the TLS index of the Havok monitor stream.
const TLS_GET_VALUE: u32 = 0x00fd_f044;
const MONITOR_SLOT: u32 = 0x0126_835c;
/// Profiling labels: `"TtrcShpCollect"` and the end label.
const LABEL_RAY_CAST_COLLECT: u32 = 0x010c_e1f8;
const LABEL_END: u32 = 0x010c_41f8;
/// `hkpBoxMotion` constructor (`00d1e690`, `this` and two arguments).
const BOX_MOTION_CONSTRUCT: u32 = 0x00d1_e690;
/// The vtable of `hkpThinBoxMotion`.
const THIN_BOX_MOTION_VTABLE: u32 = 0x010c_6e34;
/// `hkpMotion::m_type` (Xbox PDB) at +8 of a motion, and the value for `MOTION_THIN_BOX_INERTIA`.
const MOTION_TYPE: u32 = 0x08;
const MOTION_THIN_BOX_INERTIA: u8 = 6;
/// The size of the child shape buffer (`hkpShapeBuffer`) the exe keeps on its stack.
const SHAPE_BUFFER_SIZE: u32 = 0x200;

// Translated from 00d21960 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (cdecl, one word `object`; the names are descriptions, not confirmed): the depth of
/// a hierarchy. Slot `0x10` of `object` answers an iterator (none: the result is 0). With an
/// iterator, slot 8 gives the first index and slot `0xc` the next, until `0xffffffff`; slot `0x14`
/// turns an index (with a 0x200-byte buffer on the stack) into a child object, whose depth is this
/// function's result for it, compared as a byte. The result is the largest of those bytes
/// (unsigned) plus one, as the whole register (so up to 256).
pub fn fn_00d21960(e: &mut Engine, object: Ptr) -> u32 {
    let iterator = e.vcall(object.addr(), 0x10, &args![]).u32();
    if iterator == 0 {
        return 0;
    }
    // The 0x200-byte buffer is a local of the whole frame (stack offset -0x210): the child slot 0x14
    // answers can be built in it, and the recursion below reads that child, so the buffer lives
    // until the function returns. (Scoped to the slot call alone, it was freed before the
    // recursion, whose own buffer then took and cleared the same block under the child.)
    e.with_stack(0x200, |e, buffer| {
        let mut deepest = 0u8;
        let mut index = e.vcall(iterator, 8, &args![]).u32();
        while index != 0xffff_ffff {
            let child = e.vcall(iterator, 0x14, &args![index, buffer]).u32();
            let depth = fn_00d21960(e, Ptr::new(child)) as u8;
            if depth > deepest {
                deepest = depth;
            }
            index = e.vcall(iterator, 0xc, &args![index]).u32();
        }
        u32::from(deepest) + 1
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00d217d0, fn_00d217d0(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00d21930, fn_00d21930(Ptr, u32, u32) -> Ptr),
        entry!(0x00d2_1960, fn_00d21960(Ptr) -> u32),
    ]
}

/// Low word of the time-stamp counter (`RDTSC`); the host clock stands in.
fn time_stamp_low() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_nanos() as u32)
        .unwrap_or(0)
}

/// Appends a 12-byte record `(label, time stamp)` to the thread's monitor stream while its
/// cursor (+4) is below its limit (+0xC).
fn profile_record(e: &mut Engine, label: u32) {
    let index: u32 = e.global(MONITOR_SLOT);
    let stream = e.call(TLS_GET_VALUE, &args![index]).u32();
    let cursor = e.mem.u32(stream + 4);
    if cursor < e.mem.u32(stream + 0xc) {
        e.mem.set_u32(cursor, label);
        e.mem.set_u32(cursor + 4, time_stamp_low());
        e.mem.set_u32(stream + 4, cursor + 12);
    }
}

// Translated from 00d217d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Slot `0x24` of the shape collections' vtables (`RET 0xC`): `(input, parent body,
/// collector)` with `this` the collection. Casts the ray of `input` against every child (see
/// the module comment), between `"TtrcShpCollect"` profiling records.
pub fn fn_00d217d0(e: &mut Engine, this: Ptr, input: Ptr, parent_body: Ptr, collector: Ptr) {
    profile_record(e, LABEL_RAY_CAST_COLLECT);
    let container = this.addr() + 0x10;
    let filter = e.mem.u32(input.addr() + 0x24);
    // The stack frame: the child body (16 bytes), the filter's answer byte, the shape buffer.
    e.with_stack(0x20 + SHAPE_BUFFER_SIZE, |e, frame| {
        let body = frame.addr();
        let answer = frame.addr() + 0x10;
        let buffer = frame.addr() + 0x20;
        let mut key = e.vcall(container, 0x8, &args![]).i32();
        while key != -1 {
            let wanted = if filter == 0 {
                true
            } else {
                let verdict = e
                    .vcall(filter, 0, &args![answer, input, this, container, key])
                    .u32();
                e.mem.u8(verdict) != 0
            };
            if wanted {
                let child = e.vcall(container, 0x14, &args![key, buffer]).u32();
                let parent_motion = e.mem.u32(parent_body.addr() + 8);
                e.mem.set_u32(body, child);
                e.mem.set_i32(body + 4, key);
                e.mem.set_u32(body + 8, parent_motion);
                e.mem.set_u32(body + 0xc, parent_body.addr());
                e.vcall(child, 0x24, &args![input, body, collector]);
            }
            key = e.vcall(container, 0xc, &args![key]).i32();
        }
    });
    profile_record(e, LABEL_END);
}

// Translated from 00d21930 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `hkpThinBoxMotion` constructor (`RET 8`): `hkpBoxMotion`'s constructor (`00d1e690`)
/// with the two arguments, then the thin box vtable and motion type 6
/// (`MOTION_THIN_BOX_INERTIA`, Xbox PDB `hkpMotion::MotionType`). Returns `this`.
pub fn fn_00d21930(e: &mut Engine, this: Ptr, first: u32, second: u32) -> Ptr {
    e.call(BOX_MOTION_CONSTRUCT, &args![this, first, second]);
    e.mem.set_u32(this.addr(), THIN_BOX_MOTION_VTABLE);
    e.mem
        .set_u8(this.addr() + MOTION_TYPE, MOTION_THIN_BOX_INERTIA);
    this
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// The child bodies seen by the child casts: child, key, parent motion, parent.
    type Bodies = Rc<RefCell<Vec<[u32; 4]>>>;

    const CONTAINER_VTABLE: u32 = 0x0200_0000;
    const FIRST_KEY: u32 = 0x0200_0100;
    const NEXT_KEY: u32 = 0x0200_0104;
    const GET_CHILD: u32 = 0x0200_0108;
    const FILTER_VTABLE: u32 = 0x0200_0040;
    const FILTER: u32 = 0x0200_0200;
    const CHILD_VTABLE: u32 = 0x0200_0080;
    const CHILD_CAST: u32 = 0x0200_0300;

    struct Rig {
        e: Engine,
        collection: Ptr,
        input: Ptr,
        parent: Ptr,
        stream: u32,
        bodies: Bodies,
    }

    /// A collection with three keys (4, 6, 8) whose children share one vtable; the filter
    /// (when installed) accepts only key 6.
    fn rig(with_filter: bool) -> Rig {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(Vec::new());
        e.put_vtable(CONTAINER_VTABLE, &[0, 0, FIRST_KEY, NEXT_KEY, 0, GET_CHILD]);
        e.put_vtable(FILTER_VTABLE, &[FILTER]);
        e.put_vtable(CHILD_VTABLE, &[0, 0, 0, 0, 0, 0, 0, 0, 0, CHILD_CAST]);
        e.register(FIRST_KEY, |_, _| 4u32.into_ret());
        e.register(NEXT_KEY, |_, a| {
            let next = match a[1] {
                4 => 6u32,
                6 => 8,
                _ => u32::MAX,
            };
            next.into_ret()
        });
        // The child for a key is an object whose second word records the key.
        e.register(GET_CHILD, |e, a| {
            let child = e.mem.alloc(8);
            e.mem.set_u32(child, CHILD_VTABLE);
            e.mem.set_u32(child + 4, a[1]);
            child.into_ret()
        });
        let bodies = Bodies::default();
        let seen = Rc::clone(&bodies);
        e.register_double(CHILD_CAST, move |e, a| {
            let body = [0, 4, 8, 12].map(|off| e.mem.u32(a[2] + off));
            seen.borrow_mut().push(body);
            Ret::default()
        });
        e.register(FILTER, |e, a| {
            // `(answer, input, collection, container, key)`: only key 6 is wanted.
            e.mem.set_u8(a[1], u8::from(a[5] == 6));
            a[1].into_ret()
        });
        let collection = e.mem.alloc(0x20);
        e.mem.set_u32(collection + 0x10, CONTAINER_VTABLE);
        let input = e.mem.alloc(0x40);
        if with_filter {
            let filter = e.mem.alloc(4);
            e.mem.set_u32(filter, FILTER_VTABLE);
            e.mem.set_u32(input + 0x24, filter);
        }
        let parent = e.mem.alloc(0x10);
        e.mem.set_u32(parent + 8, 0x4242);
        let stream = e.mem.alloc(0x200);
        e.mem.set_u32(stream + 4, stream + 0x40);
        e.mem.set_u32(stream + 0xc, stream + 0x1f0);
        e.mem.set_u32(MONITOR_SLOT, 5);
        e.register_double(TLS_GET_VALUE, move |_, _| stream.into_ret());
        Rig {
            e,
            collection: Ptr::new(collection),
            input: Ptr::new(input),
            parent: Ptr::new(parent),
            stream,
            bodies,
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
    fn without_a_filter_every_child_is_cast_with_a_body_on_the_parent() {
        let mut r = rig(false);
        let collector = 0x9000u32;
        r.e.call(
            0x00d2_17d0,
            &args![r.collection, r.input, r.parent, Ptr::<()>::new(collector)],
        );
        let casts = calls(&r.e, CHILD_CAST);
        assert_eq!(casts.len(), 3);
        // (child, input, body, collector): the body is {child, key, parent motion, parent}.
        // The body is {child, key, parent motion, parent}, as seen when the child was cast.
        let bodies = r.bodies.borrow();
        assert_eq!(bodies.len(), 3);
        assert_eq!(bodies[1], [casts[1][0], 6, 0x4242, r.parent.addr()]);
        assert!(calls(&r.e, FILTER).is_empty());
        // The start and end profiling records.
        assert_eq!(r.e.mem.u32(r.stream + 4), r.stream + 0x40 + 24);
        assert_eq!(r.e.mem.u32(r.stream + 0x40), LABEL_RAY_CAST_COLLECT);
        assert_eq!(r.e.mem.u32(r.stream + 0x4c), LABEL_END);
    }

    #[test]
    fn a_filter_selects_the_children() {
        let mut r = rig(true);
        r.e.call(
            0x00d2_17d0,
            &args![r.collection, r.input, r.parent, Ptr::<()>::new(0x9000)],
        );
        assert_eq!(calls(&r.e, FILTER).len(), 3);
        let first = &calls(&r.e, FILTER)[0];
        assert_eq!(first[2], r.input.addr());
        assert_eq!(first[3], r.collection.addr());
        assert_eq!(first[4], r.collection.addr() + 0x10);
        assert_eq!(first[5], 4);
        // Only the child of key 6 is asked for and cast.
        assert_eq!(calls(&r.e, GET_CHILD).len(), 1);
        assert_eq!(calls(&r.e, CHILD_CAST).len(), 1);
    }

    #[test]
    fn thin_box_motion_constructor_sets_vtable_and_type() {
        let mut e = Engine::new();
        e.call_log = Some(Vec::new());
        e.register(BOX_MOTION_CONSTRUCT, |e, a| {
            e.mem.set_u8(a[0] + 8, 3);
            a[0].into_ret()
        });
        let motion = e.mem.alloc(0x100);
        let result = e.call(0x00d2_1930, &args![motion, 0x11u32, 0x22u32]).u32();
        assert_eq!(result, motion);
        assert_eq!(
            calls(&e, BOX_MOTION_CONSTRUCT),
            vec![vec![motion, 0x11, 0x22]]
        );
        assert_eq!(e.mem.u32(motion), THIN_BOX_MOTION_VTABLE);
        assert_eq!(e.mem.u8(motion + 8), 6);
    }
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    /// An object whose slot `0x10` answers the iterator stored at `+4`; an iterator keeps its
    /// children as a count at `+4` and pointers from `+8`.
    fn build(e: &mut Engine) -> (u32, u32) {
        let object_vtable = e.mem.alloc(0x20);
        e.mem.set_u32(object_vtable + 0x10, 0x0900_0031);
        e.register(0x0900_0031, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        let iterator_vtable = e.mem.alloc(0x20);
        for (slot, address) in [(8, 0x0900_0032), (0xc, 0x0900_0033), (0x14, 0x0900_0034)] {
            e.mem.set_u32(iterator_vtable + slot, address);
        }
        e.register(0x0900_0032, |e, a| Ret {
            eax: if e.mem.u32(a[0] + 4) > 0 {
                0
            } else {
                0xffff_ffff
            },
            ..Ret::default()
        });
        e.register(0x0900_0033, |e, a| Ret {
            eax: if a[1] + 1 < e.mem.u32(a[0] + 4) {
                a[1] + 1
            } else {
                0xffff_ffff
            },
            ..Ret::default()
        });
        e.register(0x0900_0034, |e, a| {
            assert_ne!(a[2], 0, "a buffer is given");
            Ret {
                eax: e.mem.u32(a[0] + 8 + a[1] * 4),
                ..Ret::default()
            }
        });
        (object_vtable, iterator_vtable)
    }

    fn object(e: &mut Engine, vtable: u32, iterator: u32) -> u32 {
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, iterator);
        object
    }

    fn iterator(e: &mut Engine, vtable: u32, children: &[u32]) -> u32 {
        let iterator = e.mem.alloc(0x20);
        e.mem.set_u32(iterator, vtable);
        e.mem.set_u32(iterator + 4, children.len() as u32);
        for (index, child) in children.iter().enumerate() {
            e.mem.set_u32(iterator + 8 + index as u32 * 4, *child);
        }
        iterator
    }

    #[test]
    fn fn_00d21960_is_the_depth_of_the_hierarchy() {
        let mut e = Engine::new();
        let (object_vtable, iterator_vtable) = build(&mut e);
        // No iterator: 0. An iterator without children: 1.
        let leaf = object(&mut e, object_vtable, 0);
        assert_eq!(e.call(0x00d2_1960, &args![leaf]).u32(), 0);
        let empty = iterator(&mut e, iterator_vtable, &[]);
        let childless = object(&mut e, object_vtable, empty);
        assert_eq!(e.call(0x00d2_1960, &args![childless]).u32(), 1);
        // root -> (leaf, middle -> (leaf)): the middle is 1, the root 2.
        let inner = iterator(&mut e, iterator_vtable, &[leaf]);
        let middle = object(&mut e, object_vtable, inner);
        let top = iterator(&mut e, iterator_vtable, &[leaf, middle, leaf]);
        let root = object(&mut e, object_vtable, top);
        assert_eq!(e.call(0x00d2_1960, &args![middle]).u32(), 1);
        assert_eq!(e.call(0x00d2_1960, &args![root]).u32(), 2);
    }

    /// A child built in the 0x200-byte buffer (slot 0x14 answers a pointer into it) is still
    /// there when the recursion reads it: the buffer is the frame's, not the slot call's. With
    /// the freed-block check a buffer freed early would fault on the child's vtable.
    #[test]
    fn fn_00d21960_reads_a_child_built_in_its_buffer() {
        let mut e = Engine::new();
        e.mem.enable_freed_check(true);
        let (object_vtable, iterator_vtable) = build(&mut e);
        let in_buffer = e.mem.alloc(0x20);
        for slot in (0..0x20).step_by(4) {
            let word = e.mem.u32(iterator_vtable + slot);
            e.mem.set_u32(in_buffer + slot, word);
        }
        e.mem.set_u32(in_buffer + 0x14, 0x0900_0035);
        e.register_double(0x0900_0035, move |e, a| {
            // A leaf made in place: the object table, no iterator.
            e.mem.set_u32(a[2], object_vtable);
            e.mem.set_u32(a[2] + 4, 0);
            a[2].into_ret()
        });
        let one = iterator(&mut e, in_buffer, &[0]);
        let root = object(&mut e, object_vtable, one);
        assert_eq!(e.call(0x00d2_1960, &args![root]).u32(), 1);
    }
}
