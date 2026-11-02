//! `BSHavok/bhkpcollisionobject.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkPCollisionObject` (0x14 bytes, the same fields as `bhkNiCollisionObject`) links a scene
//! object (`+8`) with a world object (`+0x10`) whose Havok object (at `+8` of it) has the bounding
//! box and the transform the two conversion functions move between.

#[allow(unused_imports)]
use crate::prelude::*;

use super::bhknicollisionobject::{enter_context, push_viewer_string, BhkNiCollisionObject};

/// The stream version (`stream + 4`) from which the collision object loads and links itself;
/// older streams use the `NiCollisionObject` versions.
const FIRST_NEW_STREAM_VERSION: u32 = 0x15;
/// `bhkNiCollisionObject::LoadBinary` (`00c6c330`) and `LinkObject` (`00c6bb70`), and the
/// `NiCollisionObject` ones (`00a80fd0`, `00a80ff0`) that old streams get.
const NI_COLLISION_OBJECT_LOAD_BINARY: u32 = 0x00a8_0fd0;
const NI_COLLISION_OBJECT_LINK_OBJECT: u32 = 0x00a8_0ff0;
const BHK_NI_COLLISION_OBJECT_LOAD_BINARY: u32 = 0x00c6_c330;
const BHK_NI_COLLISION_OBJECT_LINK_OBJECT: u32 = 0x00c6_bb70;
/// `bhkNiCollisionObject`'s constructor body (`00c6b910`), `CopyMembers` (`00c6ba30`) and
/// `AddViewerStrings` (`00c6c430`); `operator new` (`00aa13e0`) and the class's vtable.
const BASE_CONSTRUCTOR: u32 = 0x00c6_b910;
const BASE_COPY_MEMBERS: u32 = 0x00c6_ba30;
const BASE_ADD_VIEWER_STRINGS: u32 = 0x00c6_c430;
const OPERATOR_NEW: u32 = 0x00aa_13e0;
const VTABLE: u32 = 0x0106_6b94;
/// The `NiRTTI` pointer word (`01268168`) that `AddViewerStrings` shows as the class name, and the
/// name builder (`00a7d330`, cdecl).
const CLASS_RTTI: u32 = 0x0126_8168;
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
/// The floats the box conversions use: 0.5 (`01016248`), the Havok scale (`01268170`) and the
/// factor `01012054`.
const HALF: u32 = 0x0101_6248;
const HAVOK_SCALE: u32 = 0x0126_8170;
const BOX_FACTOR: u32 = 0x0101_2054;
/// `NiAVObject` helper that refreshes the 0x34-byte transform at `+0x68` (`00a68bf0`), the
/// vector conversion that scales three lanes into three floats (`00458620`, cdecl `(out,
/// vector)`), the reentrant lock at `01267b80` (acquire `0040fbf0(lock, 0)`; the count is at
/// `+4`, the owner at `+0`), an empty hook the exe calls around the box update (`00a29680`) and
/// `hkpAabbPhantom::setAabb` (`00cf24a0`).
const REFRESH_LOCAL_TRANSFORM: u32 = 0x00a6_8bf0;
const SCALE_VECTOR: u32 = 0x0045_8620;
const LOCK: u32 = 0x0126_7b80;
const LOCK_ACQUIRE: u32 = 0x0040_fbf0;
const BOX_HOOK: u32 = 0x00a2_9680;
const SET_AABB: u32 = 0x00cf_24a0;

/// Reads the four lanes of the 16-byte vector at `address`.
fn read_lanes(e: &Engine, address: u32) -> [f32; 4] {
    [0, 4, 8, 12].map(|offset| e.mem.f32(address + offset))
}

/// The lanes `f(a, b)` of two vectors.
fn combine(a: [f32; 4], b: [f32; 4], f: impl Fn(f32, f32) -> f32) -> [f32; 4] {
    std::array::from_fn(|lane| f(a[lane], b[lane]))
}

/// Writes four lanes to the 16-byte vector at `address`, which a `MOVAPS` needs to be aligned.
fn write_lanes(e: &mut Engine, address: u32, lanes: [f32; 4]) {
    assert_eq!(
        address & 0xf,
        0,
        "MOVAPS address {address:#010x} is not 16-byte aligned"
    );
    for (index, lane) in lanes.into_iter().enumerate() {
        e.mem.set_f32(address + 4 * index as u32, lane);
    }
}

// Translated from 00c87f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPCollisionObject::LoadBinary` (engine map): a jump on the stream's version (`stream + 4`):
/// below 0x15 to `00a80fd0` (the `NiCollisionObject` one), otherwise to `00c6c330`
/// (`bhkNiCollisionObject`); `this` and the stream are passed on.
pub fn bhk_p_collision_object_load_binary(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    stream: Ptr,
) {
    let target = if e.mem.u32(stream.addr() + 4) < FIRST_NEW_STREAM_VERSION {
        NI_COLLISION_OBJECT_LOAD_BINARY
    } else {
        BHK_NI_COLLISION_OBJECT_LOAD_BINARY
    };
    e.call(target, &args![this, stream]);
}

// Translated from 00c87f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkPCollisionObject::LinkObject` (engine map): like `00c87f60`, below version 0x15 to
/// `00a80ff0`, otherwise to `00c6bb70`.
pub fn bhk_p_collision_object_link_object(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    stream: Ptr,
) {
    let target = if e.mem.u32(stream.addr() + 4) < FIRST_NEW_STREAM_VERSION {
        NI_COLLISION_OBJECT_LINK_OBJECT
    } else {
        BHK_NI_COLLISION_OBJECT_LINK_OBJECT
    };
    e.call(target, &args![this, stream]);
}

// Translated from 00c88020 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c88020 - the /GS stack cookie check (00ec408c) is not translated.
/// `bhkPCollisionObject::hktoNiTransform` (engine map): when there is a world object whose Havok
/// object (`+8` of it) exists, builds an identity `NiTransform` (rotation, then translation, then
/// scale 1.0), asks the Havok object for its bounding box (virtual slot `+0x18`, `(this, &box)`,
/// into a 16-byte aligned box of two vectors), makes the second vector half of the difference
/// of the two (`0.5 * (max - min)`, lane by lane) and the first that plus the old first, converts
/// the second vector into the transform's translation (`00458620(&translation, &vector)`) and
/// hands the transform to the object's virtual slot `+0xB8`.
pub fn bhk_p_collision_object_hk_to_ni_transform(e: &mut Engine, this: Ptr<BhkNiCollisionObject>) {
    let world = e.get(this, BhkNiCollisionObject::spWorldObject).addr();
    if world == 0 {
        return;
    }
    let havok = e.mem.u32(world + 8);
    if havok == 0 {
        return;
    }
    // The transform (0x34 bytes) sits right below the box, which is aligned to 16.
    e.with_stack(0x70, |e, frame| {
        let aabb = (frame.addr() + 0x34 + 0xf) & !0xf;
        let transform = aabb - 0x34;
        for (offset, value) in [(0u32, 1.0f32), (0x10, 1.0), (0x20, 1.0), (0x30, 1.0)] {
            e.mem.set_f32(transform + offset, value);
        }
        for offset in [4u32, 8, 0xc, 0x14, 0x18, 0x1c, 0x24] {
            e.mem.set_f32(transform + offset, 0.0);
        }
        e.vcall(havok, 0x18, &args![aabb]);
        let min = read_lanes(e, aabb);
        let max = read_lanes(e, aabb + 0x10);
        let half = e.mem.f32(HALF);
        let half_extent = combine(max, min, |high, low| half * (high - low));
        let center = combine(half_extent, min, |h, low| h + low);
        write_lanes(e, aabb + 0x10, half_extent);
        write_lanes(e, aabb, center);
        e.call(SCALE_VECTOR, &args![transform + 0x24, aabb + 0x10]);
        e.vcall(this.addr(), 0xb8, &args![transform]);
    });
}

// Translated from 00c880f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c880f0 - the /GS stack cookie check (00ec408c) is not translated.
/// `bhkPCollisionObject::NitohkTransform` (engine map): when there is a scene object (`+8`),
/// refreshes its local transform (`00a68bf0`); then when there is a world object (`+0x10`) whose
/// Havok object (`+8` of it) exists, copies the scene object's 0x34-byte transform at `+0x68`
/// aside, asks the Havok object for its bounding box (virtual slot `+0x18`), and makes the box
/// the translation of the transform, scaled by the Havok scale (`float` at `01268170`) and
/// widened by the half extents: with `h = 0.5 * (max - min)` and `t` the translation
/// `(x, y, z, 0)` times the scale, the first vector becomes `k * h + t` (`k` the float at
/// `01012054`) and the second `t + h`. Takes the reentrant lock at `01267b80` (`0040fbf0(lock,
/// 0)`) for the update; if the Havok object is still present, calls the empty hook `00a29680`
/// on the world object, `setAabb` (`00cf24a0`) with the box and the hook again; releases the
/// lock (decrement of the count at `01267b84`, clearing the owner at `01267b80` at zero).
pub fn bhk_p_collision_object_ni_to_hk_transform(e: &mut Engine, this: Ptr<BhkNiCollisionObject>) {
    let world = e.get(this, BhkNiCollisionObject::spWorldObject).addr();
    let scene = e.get(this, BhkNiCollisionObject::m_pkSceneObject).addr();
    if scene == 0 {
        return;
    }
    e.call(REFRESH_LOCAL_TRANSFORM, &args![scene]);
    if world == 0 {
        return;
    }
    let havok = e.mem.u32(world + 8);
    if havok == 0 {
        return;
    }
    // The copy of the transform (0x34 bytes) is followed by the vector `t` (16-byte aligned) and
    // the box (two vectors), as on the exe's frame.
    e.with_stack(0xa0, |e, frame| {
        let vector = (frame.addr() + 0x34 + 0xf) & !0xf;
        let local = vector - 0x34;
        for word in 0..13 {
            let value = e.mem.u32(scene + 0x68 + 4 * word);
            e.mem.set_u32(local + 4 * word, value);
        }
        let aabb = vector + 0x10;
        e.vcall(havok, 0x18, &args![aabb]);
        let min = read_lanes(e, aabb);
        let max = read_lanes(e, aabb + 0x10);
        let half = e.mem.f32(HALF);
        let half_extent = combine(max, min, |high, low| half * (high - low));
        let scale = e.mem.f32(HAVOK_SCALE);
        let factor = e.mem.f32(BOX_FACTOR);
        let translation = [
            scale * e.mem.f32(local + 0x24),
            scale * e.mem.f32(local + 0x28),
            scale * e.mem.f32(local + 0x2c),
            0.0,
        ];
        let first = combine(half_extent, translation, |h, t| factor * h + t);
        let second = combine(translation, half_extent, |t, h| t + h);
        write_lanes(e, vector, translation);
        write_lanes(e, aabb, first);
        write_lanes(e, aabb + 0x10, second);
        e.call(LOCK_ACQUIRE, &args![LOCK, 0u32]);
        let havok = e.mem.u32(world + 8);
        if havok != 0 {
            e.call(BOX_HOOK, &args![world]);
            e.call(SET_AABB, &args![havok, aabb]);
            e.call(BOX_HOOK, &args![world]);
        }
        let count = e.mem.u32(LOCK + 4).wrapping_sub(1);
        e.mem.set_u32(LOCK + 4, count);
        if count == 0 {
            e.mem.set_u32(LOCK, 0);
        }
    });
}

// Translated from 00c88210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c88210 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPCollisionObject::CreateClone` (engine map): with the thread block's `+0x2b4` word at
/// 0x10, takes 0x14 bytes from `operator new`; when that worked runs the base constructor body
/// (`00c6b910`) on them and stores the class's vtable (`01066b94`). Runs `CopyMembers`
/// (`00c6ba30`) of the original with the clone (null when the allocation failed) and `process`.
/// Returns the clone.
pub fn bhk_p_collision_object_create_clone(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    process: Ptr,
) -> Ptr<BhkNiCollisionObject> {
    let (word, saved) = enter_context(e);
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    if block != 0 {
        e.call(BASE_CONSTRUCTOR, &args![block]);
        e.mem.set_u32(block, VTABLE);
    }
    e.call(BASE_COPY_MEMBERS, &args![this, block, process]);
    e.mem.set_u32(word, saved);
    Ptr::new(block)
}

// Translated from 00c882c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c882c0 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkPCollisionObject::AddViewerStrings` (engine map): with the thread block's `+0x2b4` word at
/// 0x10, appends to `strings` the base class strings (`00c6c430`) and the class name (`00a7d330`
/// on the `NiRTTI` pointer at `01268168`).
pub fn bhk_p_collision_object_add_viewer_strings(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    strings: Ptr,
) {
    let (word, saved) = enter_context(e);
    e.call(BASE_ADD_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(CLASS_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, strings.addr(), text);
    e.mem.set_u32(word, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c87f60,
            bhk_p_collision_object_load_binary(Ptr<BhkNiCollisionObject>, Ptr)
        ),
        entry!(
            0x00c87f80,
            bhk_p_collision_object_link_object(Ptr<BhkNiCollisionObject>, Ptr)
        ),
        entry!(
            0x00c88020,
            bhk_p_collision_object_hk_to_ni_transform(Ptr<BhkNiCollisionObject>)
        ),
        entry!(
            0x00c880f0,
            bhk_p_collision_object_ni_to_hk_transform(Ptr<BhkNiCollisionObject>)
        ),
        entry!(
            0x00c88210,
            bhk_p_collision_object_create_clone(
                Ptr<BhkNiCollisionObject>,
                Ptr,
            ) -> Ptr<BhkNiCollisionObject>
        ),
        entry!(
            0x00c882c0,
            bhk_p_collision_object_add_viewer_strings(Ptr<BhkNiCollisionObject>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Words = Vec<u32>;

    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Words> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine with this unit's globals mapped and the call log on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(HALF, 4);
        e.map(HAVOK_SCALE, 4);
        e.map(BOX_FACTOR, 4);
        e.map(LOCK, 8);
        e.map(CLASS_RTTI, 4);
        e.mem.set_f32(HALF, 0.5);
        e.mem.set_f32(HAVOK_SCALE, 0.25);
        e.mem.set_f32(BOX_FACTOR, -1.0);
        e.call_log = Some(vec![]);
        e
    }

    /// An object of `size` bytes whose vtable (a zeroed block) holds the `(slot offset, target)`
    /// pairs.
    fn object_with_slots(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        object
    }

    #[test]
    fn load_binary_and_link_object_pick_their_target_by_the_stream_version() {
        for (function, old, new) in [
            (
                0x00c87f60,
                NI_COLLISION_OBJECT_LOAD_BINARY,
                BHK_NI_COLLISION_OBJECT_LOAD_BINARY,
            ),
            (
                0x00c87f80,
                NI_COLLISION_OBJECT_LINK_OBJECT,
                BHK_NI_COLLISION_OBJECT_LINK_OBJECT,
            ),
        ] {
            let mut e = engine();
            for target in [old, new] {
                e.register(target, |_, _| Ret::default());
            }
            let stream = e.mem.alloc(0x20);
            for (version, expected) in [(0x14, old), (0x15, new), (0x14_0000_u32, new), (0, old)] {
                e.mem.set_u32(stream + 4, version);
                e.call(function, &args![0x7000u32, stream]);
                assert_eq!(
                    calls_to(&e, expected).last(),
                    Some(&vec![0x7000, stream]),
                    "version {version:#x}"
                );
            }
            assert_eq!(calls_to(&e, old).len(), 2);
            assert_eq!(calls_to(&e, new).len(), 2);
        }
    }

    fn collision_object(e: &mut Engine, slots: &[(u32, u32)]) -> Ptr<BhkNiCollisionObject> {
        Ptr::new(object_with_slots(e, 0x14, slots))
    }

    #[test]
    fn hk_to_ni_transform_needs_a_world_object_with_a_havok_object() {
        let mut e = engine();
        e.register(0x7000_00b8, |_, _| Ret::default());
        let this = collision_object(&mut e, &[(0xb8, 0x7000_00b8)]);
        e.call(0x00c88020, &args![this]);
        let world = e.mem.alloc(0x10);
        e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(world));
        e.call(0x00c88020, &args![this]);
        assert!(calls_to(&e, 0x7000_00b8).is_empty());
    }

    #[test]
    fn hk_to_ni_transform_builds_the_transform_from_the_bounding_box() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let taken = seen.clone();
        e.register_double(0x7000_0018, |e, a| {
            for (lane, (min, max)) in [(1.0f32, 5.0f32), (-2.0, 2.0), (0.5, 8.5), (7.0, 3.0)]
                .into_iter()
                .enumerate()
            {
                e.mem.set_f32(a[1] + 4 * lane as u32, min);
                e.mem.set_f32(a[1] + 0x10 + 4 * lane as u32, max);
            }
            ret(0)
        });
        e.register_double(SCALE_VECTOR, move |e, a| {
            // The vector it receives is the second one of the box: half the difference.
            *taken.borrow_mut() = (0..4).map(|lane| e.mem.f32(a[1] + 4 * lane)).collect();
            for lane in 0..3 {
                e.mem.set_f32(a[0] + 4 * lane, 10.0 + lane as f32);
            }
            ret(a[0])
        });
        let transform_seen = Rc::new(RefCell::new(Vec::new()));
        let sink = transform_seen.clone();
        e.register_double(0x7000_00b8, move |e, a| {
            *sink.borrow_mut() = (0..13).map(|word| e.mem.u32(a[1] + 4 * word)).collect();
            Ret::default()
        });
        let this = collision_object(&mut e, &[(0xb8, 0x7000_00b8)]);
        let havok = object_with_slots(&mut e, 0x10, &[(0x18, 0x7000_0018)]);
        let world = e.mem.alloc(0x10);
        e.mem.set_u32(world + 8, havok);
        e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(world));

        e.call(0x00c88020, &args![this]);

        // 0.5 * (max - min) per lane.
        assert_eq!(*seen.borrow(), [2.0, 2.0, 4.0, -2.0]);
        let one = 1.0f32.to_bits();
        let ten = 10.0f32.to_bits();
        assert_eq!(
            *transform_seen.borrow(),
            [
                one,
                0,
                0,
                0,
                one,
                0,
                0,
                0,
                one,
                ten,
                11.0f32.to_bits(),
                12.0f32.to_bits(),
                one
            ]
        );
        assert_eq!(calls_to(&e, 0x7000_00b8).len(), 1);
        // The box passed to the Havok object and the vector conversion are 16-byte aligned, and
        // the translation lies in the transform.
        let get_aabb = calls_to(&e, 0x7000_0018);
        assert_eq!(get_aabb[0][0], havok);
        assert_eq!(get_aabb[0][1] & 0xf, 0);
        let scale = &calls_to(&e, SCALE_VECTOR)[0];
        assert_eq!(scale[1], get_aabb[0][1] + 0x10);
        assert_eq!(scale[0], calls_to(&e, 0x7000_00b8)[0][1] + 0x24);
    }

    #[test]
    fn ni_to_hk_transform_does_nothing_without_a_scene_object() {
        let mut e = engine();
        e.register(REFRESH_LOCAL_TRANSFORM, |_, _| Ret::default());
        let this = collision_object(&mut e, &[]);
        e.call(0x00c880f0, &args![this]);
        assert!(calls_to(&e, REFRESH_LOCAL_TRANSFORM).is_empty());
    }

    #[test]
    fn ni_to_hk_transform_refreshes_the_scene_object_even_without_a_world_object() {
        let mut e = engine();
        e.register(REFRESH_LOCAL_TRANSFORM, |_, _| Ret::default());
        let this = collision_object(&mut e, &[]);
        e.set(
            this,
            BhkNiCollisionObject::m_pkSceneObject,
            Ptr::new(0x5000),
        );
        e.call(0x00c880f0, &args![this]);
        assert_eq!(calls_to(&e, REFRESH_LOCAL_TRANSFORM), vec![vec![0x5000]]);
        assert!(calls_to(&e, LOCK_ACQUIRE).is_empty());
        // A world object without a Havok object does not lock either.
        let world = e.mem.alloc(0x10);
        e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(world));
        e.call(0x00c880f0, &args![this]);
        assert!(calls_to(&e, LOCK_ACQUIRE).is_empty());
    }

    /// Everything `NitohkTransform` calls, as doubles; the Havok object answers the box
    /// `[(1, 5), (-2, 2), (0.5, 8.5), (7, 3)]` (min, max per lane), and the hook, `setAabb` and
    /// the lock record what they see.
    #[test]
    fn ni_to_hk_transform_stores_the_widened_box_under_the_lock() {
        let mut e = engine();
        e.register(REFRESH_LOCAL_TRANSFORM, |_, _| Ret::default());
        e.register(0x7000_0018, |e, a| {
            for (lane, (min, max)) in [(1.0f32, 5.0f32), (-2.0, 2.0), (0.5, 8.5), (7.0, 3.0)]
                .into_iter()
                .enumerate()
            {
                e.mem.set_f32(a[1] + 4 * lane as u32, min);
                e.mem.set_f32(a[1] + 0x10 + 4 * lane as u32, max);
            }
            ret(0)
        });
        let lock_state = Rc::new(RefCell::new(Vec::new()));
        let acquired = lock_state.clone();
        e.register_double(LOCK_ACQUIRE, move |e, _| {
            acquired.borrow_mut().push(e.mem.u32(LOCK + 4));
            e.mem.set_u32(LOCK, 0x1234);
            e.mem.set_u32(LOCK + 4, 1);
            Ret::default()
        });
        e.register(BOX_HOOK, |_, _| Ret::default());
        let stored = Rc::new(RefCell::new(Vec::new()));
        let sink = stored.clone();
        e.register_double(SET_AABB, move |e, a| {
            *sink.borrow_mut() = (0..8).map(|lane| e.mem.f32(a[1] + 4 * lane)).collect();
            Ret::default()
        });
        let scene = e.mem.alloc(0x100);
        // The local transform at +0x68: translation (4, 8, 12) after nine rotation words and
        // the scale.
        for word in 0..13 {
            e.mem.set_u32(scene + 0x68 + 4 * word, 0x100 + word);
        }
        e.mem.set_f32(scene + 0x68 + 0x24, 4.0);
        e.mem.set_f32(scene + 0x68 + 0x28, 8.0);
        e.mem.set_f32(scene + 0x68 + 0x2c, 12.0);
        let havok = object_with_slots(&mut e, 0x10, &[(0x18, 0x7000_0018)]);
        let world = e.mem.alloc(0x10);
        e.mem.set_u32(world + 8, havok);
        let this = collision_object(&mut e, &[]);
        e.set(this, BhkNiCollisionObject::m_pkSceneObject, Ptr::new(scene));
        e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(world));
        e.mem.set_u32(LOCK, 0x1234);
        e.mem.set_u32(LOCK + 4, 2);

        e.call(0x00c880f0, &args![this]);

        // h = 0.5 * (max - min) = (2, 2, 4, -2); t = 0.25 * (4, 8, 12) = (1, 2, 3, 0);
        // first = -h + t, second = t + h.
        assert_eq!(
            *stored.borrow(),
            [-1.0, 0.0, -1.0, 2.0, 3.0, 4.0, 7.0, -2.0]
        );
        assert_eq!(*lock_state.borrow(), [2]);
        assert_eq!(calls_to(&e, LOCK_ACQUIRE), vec![vec![LOCK, 0]]);
        assert_eq!(calls_to(&e, BOX_HOOK), vec![vec![world]; 2]);
        // The acquire raised the count to 1; the release took it to 0 and cleared the owner.
        assert_eq!(e.mem.u32(LOCK + 4), 0);
        assert_eq!(e.mem.u32(LOCK), 0);
    }

    #[test]
    fn ni_to_hk_transform_keeps_the_owner_while_the_lock_is_held_again() {
        let mut e = engine();
        e.register(REFRESH_LOCAL_TRANSFORM, |_, _| Ret::default());
        e.register(0x7000_0018, |_, _| Ret::default());
        e.register(LOCK_ACQUIRE, |e, _| {
            let count = e.mem.u32(LOCK + 4);
            e.mem.set_u32(LOCK + 4, count + 1);
            Ret::default()
        });
        e.register(BOX_HOOK, |_, _| Ret::default());
        e.register(SET_AABB, |_, _| Ret::default());
        let scene = e.mem.alloc(0x100);
        let havok = object_with_slots(&mut e, 0x10, &[(0x18, 0x7000_0018)]);
        let world = e.mem.alloc(0x10);
        e.mem.set_u32(world + 8, havok);
        let this = collision_object(&mut e, &[]);
        e.set(this, BhkNiCollisionObject::m_pkSceneObject, Ptr::new(scene));
        e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(world));
        e.mem.set_u32(LOCK, 0x1234);
        e.mem.set_u32(LOCK + 4, 1);
        e.call(0x00c880f0, &args![this]);
        // 1 + 1 - 1: still held once, so the owner stays.
        assert_eq!(e.mem.u32(LOCK + 4), 1);
        assert_eq!(e.mem.u32(LOCK), 0x1234);
    }

    #[test]
    fn create_clone_constructs_in_the_new_block_and_copies_members() {
        let mut e = engine();
        e.register(BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(BASE_COPY_MEMBERS, |_, _| Ret::default());
        let block = e.mem.alloc(0x14);
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);
        let this = collision_object(&mut e, &[]);
        let clone = e.call(0x00c88210, &args![this, 0x6000u32]).u32();
        assert_eq!(clone, block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(calls_to(&e, BASE_CONSTRUCTOR), vec![vec![block]]);
        assert_eq!(
            calls_to(&e, BASE_COPY_MEMBERS),
            vec![vec![this.addr(), block, 0x6000]]
        );
        assert_eq!(e.mem.u32(word), 0x42);

        // A failed allocation: no construction, CopyMembers with a null clone.
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x00c88210, &args![this, 0x6000u32]).u32(), 0);
        assert_eq!(calls_to(&e, BASE_CONSTRUCTOR).len(), 1);
        assert_eq!(calls_to(&e, BASE_COPY_MEMBERS)[1], [this.addr(), 0, 0x6000]);
    }

    #[test]
    fn add_viewer_strings_appends_the_class_name_after_the_base_strings() {
        let mut e = engine();
        e.mem.set_u32(CLASS_RTTI, 0x5151_0001);
        e.register(BASE_ADD_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(NI_GET_VIEWER_STRING, |_, _| ret(0xa000));
        e.register(STRINGS_GROW, |_, _| Ret::default());
        let stored = Rc::new(RefCell::new(Vec::new()));
        let sink = stored.clone();
        e.register_double(STRINGS_SET_AT, move |e, a| {
            sink.borrow_mut().push((a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        let this = collision_object(&mut e, &[]);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xa, 1);
        let word = e.tls() + 0x2b4;
        e.mem.set_u32(word, 0x42);

        e.call(0x00c882c0, &args![this, strings]);

        assert_eq!(
            calls_to(&e, BASE_ADD_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x5151_0001]]);
        assert_eq!(*stored.borrow(), vec![(1, 0xa000)]);
        assert!(calls_to(&e, STRINGS_GROW).is_empty());
        assert_eq!(e.mem.u32(word), 0x42);
    }
}
