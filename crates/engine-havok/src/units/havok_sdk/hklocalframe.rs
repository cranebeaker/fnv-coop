//! `Havok SDK/hklocalframe.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `hkSimpleLocalFrame` keeps a frame's local transform and the frames below it. The Xbox PDB
//! has no type for it, so its layout is described from the code: `+0` virtual table; `+0x10`
//! the local transform (four 16-byte columns: three of rotation, the translation at `+0x40`);
//! `+0x50`, `+0x54`, `+0x58` the array of child frames (data, size, capacity word whose top bit
//! means "storage not owned"); `+0x60` the frame group (a reference-counted object); `+0x64` the
//! name (an `hkStringPtr`). Virtual slots used: `0xc` writes the local transform to a 64-byte
//! buffer, `0x14` the local position to a 16-byte one, `0x18` collects the frames near a point,
//! `0x20` answers the parent frame (0 for none). The 16-byte copies the exe does with `MOVAPS`
//! are four words each.

#[allow(unused_imports)]
use crate::prelude::*;

/// The memory router (`00c85750`): its field `+0x10` is the heap allocator.
const MEMORY_ROUTER: u32 = 0x00c8_5750;
/// `hkReferencedObject::addReference` (`00c90510`) and `removeReference` (`00c905b0`), `this`
/// only.
const ADD_REFERENCE: u32 = 0x00c9_0510;
const REMOVE_REFERENCE: u32 = 0x00c9_05b0;
/// `hkVector4::setTransformedInversePos` (`00c7f690`, `this` the result, `(transform, point)`)
/// and `setTransformedPos` (`00c7f640`, the same arguments).
const SET_TRANSFORMED_INVERSE_POS: u32 = 0x00c7_f690;
const SET_TRANSFORMED_POS: u32 = 0x00c7_f640;
/// `hkTransform::setMul` (`00cb2240`, `this` the result, `(left, right)`).
const TRANSFORM_SET_MUL: u32 = 0x00cb_2240;
/// `hkStringPtr::~hkStringPtr` (`00cb0c00`).
const STRING_PTR_DESTRUCT: u32 = 0x00cb_0c00;
/// The frame's virtual table while it is being destroyed (`010d461c`, then `0102e388`).
const VTABLE_DURING_DESTRUCTION: u32 = 0x010d_461c;
const BASE_VTABLE: u32 = 0x0102_e388;

/// Copies 16 bytes (a `MOVAPS` pair).
fn copy_16(e: &mut Engine, destination: u32, source: u32) {
    for word in 0..4 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(destination.wrapping_add(4 * word), value);
    }
}

// Translated from 00df6550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleLocalFrame::setGroup` (Xbox PDB), `RET 4`: takes a reference on the new group
/// (when not null), releases the old one (when not null) and stores the new one at `+0x60`.
pub fn hk_simple_localframe_set_group(e: &mut Engine, this: Ptr, group: Ptr) {
    if group.addr() != 0 {
        e.call(ADD_REFERENCE, &args![group]);
    }
    let old = e.mem.u32(this.addr() + 0x60);
    if old != 0 {
        e.call(REMOVE_REFERENCE, &args![old]);
    }
    e.mem.set_u32(this.addr() + 0x60, group.addr());
}

// Translated from 00df6580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleLocalFrame::setLocalTransform` (Xbox PDB), `RET 4`: copies the four 16-byte columns
/// of the transform at `transform` to `+0x10`.
pub fn hk_simple_localframe_set_local_transform(e: &mut Engine, this: Ptr, transform: Ptr) {
    for column in 0..4 {
        copy_16(
            e,
            this.addr() + 0x10 + 0x10 * column,
            transform.addr().wrapping_add(0x10 * column),
        );
    }
}

// Translated from 00df65b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleLocalFrame::getLocalPosition` (Xbox PDB), `RET 4`: copies the translation column
/// (`+0x40`) to the vector at `out`.
pub fn hk_simple_localframe_get_local_position(e: &mut Engine, this: Ptr, out: Ptr) {
    copy_16(e, out.addr(), this.addr() + 0x40);
}

// Translated from 00df65d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleLocalFrame::getNearbyFrames` (Xbox PDB), `RET 0xc` `(position, radius,
/// collector)`: works out the distance from `position` to the frame's translation (SSE:
/// squared differences added as `z² + (y² + x²)`, then `SQRTSS`); when it is within `radius`
/// (compared with `FCOMP`: not above, and not a NaN) calls the collector's slot `0xc` with the
/// frame and the distance. Then, when the frame has children, puts `position` into the frame's
/// own space (`hkVector4::setTransformedInversePos` with the local transform at `+0x10`) and
/// asks each child (slot `0x18`, `(that position, radius, collector)`).
pub fn hk_simple_localframe_get_nearby_frames(
    e: &mut Engine,
    this: Ptr,
    position: Ptr,
    radius: f32,
    collector: Ptr,
) {
    let base = this.addr();
    let delta: [f32; 3] = std::array::from_fn(|lane| {
        e.mem.f32(position.addr() + 4 * lane as u32) - e.mem.f32(base + 0x40 + 4 * lane as u32)
    });
    let squares = delta.map(|d| d * d);
    let inner = squares[1] + squares[0];
    let distance = (squares[2] + inner).sqrt();
    // `FLD` of the stored distance and of the radius, `FCOMP`: skip when radius < distance or
    // unordered.
    if radius >= distance {
        let bits = x87_quiet_bits(distance.to_bits());
        e.vcall(collector.addr(), 0xc, &args![this, bits]);
    }
    if e.mem.i32(base + 0x54) > 0 {
        e.with_stack(0x10, |e, local| {
            e.call(
                SET_TRANSFORMED_INVERSE_POS,
                &args![local, base + 0x10, position],
            );
            let mut index = 0i32;
            if e.mem.i32(base + 0x54) > 0 {
                loop {
                    let children = e.mem.u32(base + 0x50);
                    let child = e
                        .mem
                        .u32(children.wrapping_add((index as u32).wrapping_mul(4)));
                    let bits = x87_quiet_bits(radius.to_bits());
                    e.vcall(child, 0x18, &args![local, bits, collector]);
                    index += 1;
                    if index >= e.mem.i32(base + 0x54) {
                        break;
                    }
                }
            }
        });
    }
}

// Translated from 00df6680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleLocalFrame::getNumChildFrames` (Xbox PDB): the size of the child array (`+0x54`).
pub fn hk_simple_localframe_get_num_child_frames(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.i32(this.addr() + 0x54)
}

// Translated from 00df6690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleLocalFrame::getChildFrame` (Xbox PDB), `RET 4`: child number `index` of the array at
/// `+0x50`.
pub fn hk_simple_localframe_get_child_frame(e: &mut Engine, this: Ptr, index: i32) -> u32 {
    let children = e.mem.u32(this.addr() + 0x50);
    e.mem
        .u32(children.wrapping_add((index as u32).wrapping_mul(4)))
}

// Translated from 00df66a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the frame's transform in the root frame's
/// space, written to the 64 bytes at `out`: with a parent frame (slot `0x20`) the parent's own
/// such transform (this function, recursively) multiplied by the frame's local transform
/// (slot `0xc`, `hkTransform::setMul`); without one, the local transform itself. (The exe asks
/// slot `0x20` twice, the second time with the address of the parent's buffer pushed, which that
/// slot ignores and the recursive call then takes as its argument.)
pub fn fn_00df66a0(e: &mut Engine, this: Ptr, out: Ptr) {
    let parent = e.vcall(this.addr(), 0x20, &args![]).u32();
    if parent != 0 {
        e.with_stack(0x80, |e, buffers| {
            let local = buffers.addr();
            let parents = buffers.addr() + 0x40;
            let parent = e.vcall(this.addr(), 0x20, &args![]).u32();
            fn_00df66a0(e, Ptr::new(parent), Ptr::new(parents));
            e.vcall(this.addr(), 0xc, &args![local]);
            e.call(TRANSFORM_SET_MUL, &args![out, parents, local]);
        });
    } else {
        e.vcall(this.addr(), 0xc, &args![out]);
    }
}

// Translated from 00df6710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the frame's position in the root frame's
/// space, written to the vector at `position`: first the local position (slot `0x14`, into
/// `position`), then for each ancestor from the parent up (slot `0x20`) the vector is replaced by
/// the ancestor's local transform (slot `0xc`) applied to it (`hkVector4::setTransformedPos`).
pub fn fn_00df6710(e: &mut Engine, this: Ptr, position: Ptr) {
    e.vcall(this.addr(), 0x14, &args![position]);
    let mut frame = e.vcall(this.addr(), 0x20, &args![]).u32();
    if frame != 0 {
        e.with_stack(0x40, |e, transform| {
            while frame != 0 {
                e.vcall(frame, 0xc, &args![transform]);
                e.call(SET_TRANSFORMED_POS, &args![position, transform, position]);
                frame = e.vcall(frame, 0x20, &args![]).u32();
            }
        });
    }
}

// Translated from 00df6770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the translation column of the frame's
/// local transform as the virtual slot `0xc` gives it (a 64-byte buffer), copied to the vector
/// at `out`.
pub fn fn_00df6770(e: &mut Engine, this: Ptr, out: Ptr) {
    e.with_stack(0x40, |e, transform| {
        e.vcall(this.addr(), 0xc, &args![transform]);
        copy_16(e, out.addr(), transform.addr() + 0x30);
    });
}

// Translated from 00df67a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleLocalFrame::getLocalTransform` (Xbox PDB), `RET 4`: copies the four columns at
/// `+0x10` to the 64 bytes at `out`.
pub fn hk_simple_localframe_get_local_transform(e: &mut Engine, this: Ptr, out: Ptr) {
    for column in 0..4 {
        copy_16(
            e,
            out.addr().wrapping_add(0x10 * column),
            this.addr() + 0x10 + 0x10 * column,
        );
    }
}

// Translated from 00df67d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkSimpleLocalFrame::~hkSimpleLocalFrame` (Xbox PDB), `fastcall`: stores the destruction
/// virtual table (`010d461c`), releases every child (`00c905b0`) and the group, destroys the name
/// (`hkStringPtr`, at `+0x64`), empties the child array (freeing its storage through the heap
/// allocator's slot `0x10` when it owns it) and stores the base virtual table (`0102e388`).
pub fn fn_00df67d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let count = e.mem.i32(base + 0x54);
    e.mem.set_u32(base, VTABLE_DURING_DESTRUCTION);
    let mut index = 0i32;
    if count > 0 {
        loop {
            let children = e.mem.u32(base + 0x50);
            let child = e
                .mem
                .u32(children.wrapping_add((index as u32).wrapping_mul(4)));
            e.call(REMOVE_REFERENCE, &args![child]);
            index += 1;
            if index >= count {
                break;
            }
        }
    }
    let group = e.mem.u32(base + 0x60);
    if group != 0 {
        e.call(REMOVE_REFERENCE, &args![group]);
    }
    e.call(STRING_PTR_DESTRUCT, &args![base + 0x64]);
    let router = e.call(MEMORY_ROUTER, &args![]).u32();
    let allocator = e.mem.u32(router + 0x10);
    let capacity = e.mem.u32(base + 0x58);
    e.mem.set_u32(base + 0x54, 0);
    if (capacity as i32) >= 0 {
        let data = e.mem.u32(base + 0x50);
        e.vcall(
            allocator,
            0x10,
            &args![data, (capacity & 0x3fff_ffff).wrapping_mul(4)],
        );
    }
    e.mem.set_u32(base + 0x50, 0);
    e.mem.set_u32(base + 0x58, 0x8000_0000);
    e.mem.set_u32(base, BASE_VTABLE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df6550, hk_simple_localframe_set_group(Ptr, Ptr)),
        entry!(
            0x00df6580,
            hk_simple_localframe_set_local_transform(Ptr, Ptr)
        ),
        entry!(
            0x00df65b0,
            hk_simple_localframe_get_local_position(Ptr, Ptr)
        ),
        entry!(
            0x00df65d0,
            hk_simple_localframe_get_nearby_frames(Ptr, Ptr, f32, Ptr)
        ),
        entry!(
            0x00df6680,
            hk_simple_localframe_get_num_child_frames(Ptr) -> i32
        ),
        entry!(
            0x00df6690,
            hk_simple_localframe_get_child_frame(Ptr, i32) -> u32
        ),
        entry!(0x00df66a0, fn_00df66a0(Ptr, Ptr)),
        entry!(0x00df6710, fn_00df6710(Ptr, Ptr)),
        entry!(0x00df6770, fn_00df6770(Ptr, Ptr)),
        entry!(
            0x00df67a0,
            hk_simple_localframe_get_local_transform(Ptr, Ptr)
        ),
        entry!(0x00df67d0, fn_00df67d0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROUTER_CELL: u32 = 0x0127_0800;
    const ALLOCATOR_VTABLE: u32 = 0x0128_4000;
    const FREE_BUFFER: u32 = 0x0900_5001;
    const FRAME_VTABLE: u32 = 0x0128_4100;
    const GET_LOCAL_TRANSFORM: u32 = 0x0900_5002;
    const GET_LOCAL_POSITION: u32 = 0x0900_5003;
    const GET_NEARBY_FRAMES: u32 = 0x0900_5004;
    const GET_PARENT: u32 = 0x0900_5005;
    const COLLECTOR_VTABLE: u32 = 0x0128_4200;
    const COLLECT: u32 = 0x0900_5006;

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0127_0000, 0x1000);
        e.put_vtable(ALLOCATOR_VTABLE, &[0, 0, 0, 0, FREE_BUFFER]);
        let allocator = e.mem.alloc(8);
        e.mem.set_u32(allocator, ALLOCATOR_VTABLE);
        let router = e.mem.alloc(0x20);
        e.mem.set_u32(router + 0x10, allocator);
        e.mem.set_u32(ROUTER_CELL, router);
        e.register(MEMORY_ROUTER, |e, _| e.mem.u32(ROUTER_CELL).into_ret());
        e.register(FREE_BUFFER, noop);
        // A frame's virtual table: slot 0xc writes a transform whose first word is the frame's
        // id (the word at +0x84) and whose translation column is (id, 1, 2, 3); slot 0x14
        // writes the position (id, 0, 0, 0); slot 0x20 answers the parent (the word at +0x80).
        e.put_vtable(
            FRAME_VTABLE,
            &[
                0,
                0,
                0,
                GET_LOCAL_TRANSFORM,
                0,
                GET_LOCAL_POSITION,
                GET_NEARBY_FRAMES,
                0,
                GET_PARENT,
            ],
        );
        e.register(GET_LOCAL_TRANSFORM, |e, a| {
            let id = e.mem.u32(a[0] + 0x84);
            e.mem.set_u32(a[1], id);
            for (lane, value) in [id, 1, 2, 3].into_iter().enumerate() {
                e.mem.set_u32(a[1] + 0x30 + 4 * lane as u32, value);
            }
            Ret::default()
        });
        e.register(GET_LOCAL_POSITION, |e, a| {
            let id = e.mem.u32(a[0] + 0x84);
            e.mem.set_u32(a[1], id);
            Ret::default()
        });
        e.register(GET_NEARBY_FRAMES, noop);
        e.register(GET_PARENT, |e, a| e.mem.u32(a[0] + 0x80).into_ret());
        e.put_vtable(COLLECTOR_VTABLE, &[0, 0, 0, COLLECT]);
        e.register(COLLECT, noop);
        for address in [
            ADD_REFERENCE,
            REMOVE_REFERENCE,
            STRING_PTR_DESTRUCT,
            SET_TRANSFORMED_INVERSE_POS,
        ] {
            e.register(address, noop);
        }
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A frame object with the virtual table, an id (+0x84) and a parent (+0x80).
    fn frame(e: &mut Engine, id: u32, parent: u32) -> u32 {
        let frame = e.mem.alloc(0x100);
        e.mem.set_u32(frame, FRAME_VTABLE);
        e.mem.set_u32(frame + 0x80, parent);
        e.mem.set_u32(frame + 0x84, id);
        frame
    }

    #[test]
    fn set_group_takes_the_new_reference_first_and_releases_the_old() {
        let mut e = engine();
        let frame = e.mem.alloc(0x100);
        e.call(0x00df_6550, &args![frame, 0x1111u32]);
        assert_eq!(e.mem.u32(frame + 0x60), 0x1111);
        assert_eq!(calls(&e, ADD_REFERENCE), vec![vec![0x1111]]);
        assert!(calls(&e, REMOVE_REFERENCE).is_empty());
        e.call(0x00df_6550, &args![frame, 0x2222u32]);
        let log: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            log[log.len() - 3..],
            [0x00df_6550, ADD_REFERENCE, REMOVE_REFERENCE]
        );
        assert_eq!(calls(&e, REMOVE_REFERENCE), vec![vec![0x1111]]);
        // Clearing the group only releases.
        e.call(0x00df_6550, &args![frame, 0u32]);
        assert_eq!(e.mem.u32(frame + 0x60), 0);
        assert_eq!(calls(&e, ADD_REFERENCE).len(), 2);
        assert_eq!(calls(&e, REMOVE_REFERENCE).len(), 2);
    }

    #[test]
    fn the_transform_accessors_copy_the_four_columns() {
        let mut e = engine();
        let frame = e.mem.alloc(0x100);
        let source = e.mem.alloc(0x40);
        for word in 0..16 {
            e.mem.set_u32(source + 4 * word, 100 + word);
        }
        e.call(0x00df_6580, &args![frame, source]);
        for word in 0..16 {
            assert_eq!(e.mem.u32(frame + 0x10 + 4 * word), 100 + word);
        }
        // Nothing around the transform was touched.
        assert_eq!(e.mem.u32(frame + 0x0c), 0);
        assert_eq!(e.mem.u32(frame + 0x50), 0);
        let out = e.mem.alloc(0x40);
        e.call(0x00df_67a0, &args![frame, out]);
        for word in 0..16 {
            assert_eq!(e.mem.u32(out + 4 * word), 100 + word);
        }
        let position = e.mem.alloc(0x10);
        e.call(0x00df_65b0, &args![frame, position]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(position + 4 * i)).collect();
        assert_eq!(words, vec![112, 113, 114, 115]);
    }

    #[test]
    fn the_child_accessors_read_the_child_array() {
        let mut e = engine();
        let frame = e.mem.alloc(0x100);
        let children = e.mem.alloc(16);
        e.mem.set_u32(children, 0xa1);
        e.mem.set_u32(children + 4, 0xa2);
        e.mem.set_u32(frame + 0x50, children);
        e.mem.set_u32(frame + 0x54, 2);
        assert_eq!(e.call(0x00df_6680, &args![frame]).i32(), 2);
        assert_eq!(e.call(0x00df_6690, &args![frame, 1u32]).u32(), 0xa2);
    }

    /// A frame at (1, 2, 3) with the collector and one child; `position` (4, 6, 3).
    fn nearby(e: &mut Engine, radius: f32, with_child: bool) -> (u32, u32, u32) {
        let frame = e.mem.alloc(0x100);
        for (lane, value) in [1.0f32, 2.0, 3.0, 0.0].into_iter().enumerate() {
            e.mem.set_f32(frame + 0x40 + 4 * lane as u32, value);
        }
        let collector = e.mem.alloc(8);
        e.mem.set_u32(collector, COLLECTOR_VTABLE);
        let position = e.mem.alloc(0x10);
        for (lane, value) in [4.0f32, 6.0, 3.0, 9.0].into_iter().enumerate() {
            e.mem.set_f32(position + 4 * lane as u32, value);
        }
        if with_child {
            let child = frame_object(e);
            let children = e.mem.alloc(8);
            e.mem.set_u32(children, child);
            e.mem.set_u32(frame + 0x50, children);
            e.mem.set_u32(frame + 0x54, 1);
        }
        e.call(0x00df_65d0, &args![frame, position, radius, collector]);
        (frame, collector, position)
    }

    fn frame_object(e: &mut Engine) -> u32 {
        let child = e.mem.alloc(0x100);
        e.mem.set_u32(child, FRAME_VTABLE);
        child
    }

    #[test]
    fn get_nearby_frames_reports_the_frame_within_the_radius() {
        let mut e = engine();
        // The distance from (1, 2, 3) to (4, 6, 3) is 5.
        let (frame, collector, _) = nearby(&mut e, 5.0, false);
        assert_eq!(
            calls(&e, COLLECT),
            vec![vec![collector, frame, 5.0f32.to_bits()]]
        );
        assert!(calls(&e, SET_TRANSFORMED_INVERSE_POS).is_empty());
        // Just under it: not reported.
        let mut e = engine();
        nearby(&mut e, 4.99, false);
        assert!(calls(&e, COLLECT).is_empty());
        // A NaN radius is not "within" either.
        let mut e = engine();
        nearby(&mut e, f32::NAN, false);
        assert!(calls(&e, COLLECT).is_empty());
    }

    #[test]
    fn get_nearby_frames_asks_each_child_with_the_position_in_its_own_space() {
        let mut e = engine();
        let (frame, collector, position) = nearby(&mut e, 1.0, true);
        // Too far to report itself, but the children are still asked.
        assert!(calls(&e, COLLECT).is_empty());
        let transform = calls(&e, SET_TRANSFORMED_INVERSE_POS);
        assert_eq!(transform.len(), 1);
        // (result vector, the frame's transform at +0x10, the position)
        assert_eq!(transform[0][1..], [frame + 0x10, position]);
        let asked = calls(&e, GET_NEARBY_FRAMES);
        assert_eq!(asked.len(), 1);
        // (child, the vector, radius bits, collector)
        assert_eq!(asked[0][1], transform[0][0]);
        assert_eq!(asked[0][2..], [1.0f32.to_bits(), collector]);
    }

    #[test]
    fn the_world_transform_multiplies_the_parents_by_the_local_one() {
        let mut e = engine();
        e.register(TRANSFORM_SET_MUL, |e, a| {
            // out[0] = parent[0] * 100 + local[0]
            let value = e.mem.u32(a[1]) * 100 + e.mem.u32(a[2]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        let grandparent = frame(&mut e, 3, 0);
        let parent = frame(&mut e, 2, grandparent);
        let child = frame(&mut e, 1, parent);
        let out = e.mem.alloc(0x40);
        // Without a parent the frame's own local transform.
        e.call(0x00df_66a0, &args![grandparent, out]);
        assert_eq!(e.mem.u32(out), 3);
        // One parent: parent * local.
        e.call(0x00df_66a0, &args![parent, out]);
        assert_eq!(e.mem.u32(out), 3 * 100 + 2);
        // Two: (grandparent * parent) * local.
        e.call(0x00df_66a0, &args![child, out]);
        assert_eq!(e.mem.u32(out), (3 * 100 + 2) * 100 + 1);
        assert_eq!(calls(&e, TRANSFORM_SET_MUL).len(), 1 + 2);
    }

    #[test]
    fn the_world_position_applies_each_ancestors_transform() {
        let mut e = engine();
        e.register(SET_TRANSFORMED_POS, |e, a| {
            // out = out + 10 * transform[0] (a[0] is the result, a[2] the point: the same)
            let value = e.mem.u32(a[2]) + 10 * e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        let grandparent = frame(&mut e, 3, 0);
        let parent = frame(&mut e, 2, grandparent);
        let child = frame(&mut e, 1, parent);
        let position = e.mem.alloc(0x10);
        e.call(0x00df_6710, &args![child, position]);
        // The local position (id 1), then the parent's and the grandparent's transforms.
        assert_eq!(e.mem.u32(position), 1 + 10 * 2 + 10 * 3);
        let applied = calls(&e, SET_TRANSFORMED_POS);
        assert_eq!(applied.len(), 2);
        assert_eq!(applied[0][0], position);
        assert_eq!(applied[0][2], position);
        // A frame without a parent is just its local position.
        e.call(0x00df_6710, &args![grandparent, position]);
        assert_eq!(e.mem.u32(position), 3);
        assert_eq!(calls(&e, SET_TRANSFORMED_POS).len(), 2);
    }

    #[test]
    fn fn_00df6770_copies_the_translation_column_of_the_local_transform() {
        let mut e = engine();
        let child = frame(&mut e, 7, 0);
        let out = e.mem.alloc(0x10);
        e.call(0x00df_6770, &args![child, out]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(out + 4 * i)).collect();
        assert_eq!(words, vec![7, 1, 2, 3]);
    }

    #[test]
    fn the_destructor_body_releases_the_children_group_and_storage() {
        let mut e = engine();
        let frame_object = e.mem.alloc(0x100);
        let children = e.mem.alloc(16);
        e.mem.set_u32(children, 0xa1);
        e.mem.set_u32(children + 4, 0xa2);
        e.mem.set_u32(frame_object + 0x50, children);
        e.mem.set_u32(frame_object + 0x54, 2);
        e.mem.set_u32(frame_object + 0x58, 2);
        e.mem.set_u32(frame_object + 0x60, 0xb1);
        e.call(0x00df_67d0, &args![frame_object]);
        assert_eq!(
            calls(&e, REMOVE_REFERENCE),
            vec![vec![0xa1], vec![0xa2], vec![0xb1]]
        );
        assert_eq!(
            calls(&e, STRING_PTR_DESTRUCT),
            vec![vec![frame_object + 0x64]]
        );
        // The owned storage goes back (capacity 2 words of 4 bytes); the array is empty.
        assert_eq!(calls(&e, FREE_BUFFER).len(), 1);
        assert_eq!(calls(&e, FREE_BUFFER)[0][1..], [children, 8]);
        assert_eq!(e.mem.u32(frame_object + 0x50), 0);
        assert_eq!(e.mem.u32(frame_object + 0x54), 0);
        assert_eq!(e.mem.u32(frame_object + 0x58), 0x8000_0000);
        assert_eq!(e.mem.u32(frame_object), BASE_VTABLE);
        // Storage that is not owned is not freed, and no group means one release fewer.
        let other = e.mem.alloc(0x100);
        e.mem.set_u32(other + 0x58, 0x8000_0000);
        e.call(0x00df_67d0, &args![other]);
        assert_eq!(calls(&e, FREE_BUFFER).len(), 1);
        assert_eq!(calls(&e, REMOVE_REFERENCE).len(), 3);
    }
}
