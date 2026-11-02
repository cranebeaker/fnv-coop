//! `BSHavok/bhkspcollisionobject.obj` (Xbox PDB source unit), subsystem `BSHavok`: its
//! functions in FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkSPCollisionObject` (vtable `01066acc`) is a `bhkCollisionObject` (0x14 bytes: the scene
//! node at `+8`, the Havok world object at `+0x10`) whose world object is a shape phantom. Its
//! stream functions are jumps to the `bhkCollisionObject` ones.
//!
//! The functions marked `Translation incomplete` carry an `FS:[0]` exception-unwinding frame,
//! which is not modeled.

#[allow(unused_imports)]
use crate::prelude::*;

/// The jump targets of the stream functions (engine map names): `bhkPCollisionObject::LoadBinary`
/// (`00c87f60`), `bhkPCollisionObject::LinkObject` (`00c87f80`),
/// `bhkCollisionObject::SaveBinary` (`00c65820`) and `bhkCollisionObject::IsEqual` (`00c87fb0`).
const COLLISION_OBJECT_LOAD_BINARY: u32 = 0x00c8_7f60;
const COLLISION_OBJECT_LINK_OBJECT: u32 = 0x00c8_7f80;
const COLLISION_OBJECT_SAVE_BINARY: u32 = 0x00c6_5820;
const COLLISION_OBJECT_IS_EQUAL: u32 = 0x00c8_7fb0;
/// The base constructor body of the collision object (`00c6b910`; the engine map does not
/// name it), the clone's member copy (`00c657f0`, thiscall: clone, process; the engine map does
/// not name it) and the heap allocator (`00aa13e0`, cdecl: size).
const BASE_CONSTRUCT: u32 = 0x00c6_b910;
const COPY_MEMBERS: u32 = 0x00c6_57f0;
const HEAP_ALLOCATE: u32 = 0x00aa_13e0;
/// `bhkShapePhantom::GetTransform` (`00c9ec60`) and `bhkShapePhantom::SetTransform`
/// (`0056df20`), engine map names (thiscall on the phantom, one stack word: the transform).
const SHAPE_PHANTOM_GET_TRANSFORM: u32 = 0x00c9_ec60;
const SHAPE_PHANTOM_SET_TRANSFORM: u32 = 0x0056_df20;
/// The scene node's local-to-world refresh (`00a68bf0`, thiscall, no stack words).
const REFRESH_WORLD_TRANSFORM: u32 = 0x00a6_8bf0;
/// The recursive lock (`01267b80`: owner at `+0`, count at `+4`) and its acquire (`0040fbf0`,
/// thiscall: one unread stack word). Releasing is done in line: the count is lowered and the
/// owner cleared when it reaches 0.
const LOCK: u32 = 0x0126_7b80;
const LOCK_ACQUIRE: u32 = 0x0040_fbf0;
/// `bhkPCollisionObject::AddViewerStrings` (`00c882c0`, engine map name), `NiGetViewerString`
/// (`00a7d330`, cdecl: name) and the string array's resize (`0096ad30`) and store
/// (`0096ae90`).
const P_COLLISION_OBJECT_ADD_VIEWER_STRINGS: u32 = 0x00c8_82c0;
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The vtable stored in a new object (`01066acc`).
const OBJECT_VTABLE: u32 = 0x0106_6acc;
/// The class name word (`012683d0`) `AddViewerStrings` passes to `NiGetViewerString`.
const CLASS_RTTI: u32 = 0x0126_83d0;
/// Per-thread word the Havok-allocating functions set to 0x10 (`TLS + 0x2b4`).
const SCOPE_WORD: u32 = 0x2b4;

/// `SetTransform` (virtual `+0xb8`), taking the `NiTransform`.
const VSLOT_SET_TRANSFORM: u32 = 0xb8;

/// The size of a `NiTransform` (nine rotation floats, the translation and the scale).
const NI_TRANSFORM_SIZE: u32 = 0x34;
/// The `NiTransform` of a scene node (`+0x68`).
const NODE_WORLD_TRANSFORM: u32 = 0x68;

layout! {
    /// `bhkSPCollisionObject` (0x14 bytes): a `bhkCollisionObject`.
    pub struct BhkSPCollisionObject: 0x14 {
        /// The scene node.
        0x08 sceneObject: Ptr,
        /// The Havok world object (a shape phantom).
        0x10 worldObject: Ptr,
    }
}

// Translated from 00cb75f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSPCollisionObject::LoadBinary` (engine map name), `(stream)`: jumps to `00c87f60`.
pub fn bhk_sp_collision_object_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(COLLISION_OBJECT_LOAD_BINARY, &args![this, stream]);
}

// Translated from 00cb7600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSPCollisionObject::LinkObject` (engine map name), `(stream)`: jumps to `00c87f80`.
pub fn bhk_sp_collision_object_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(COLLISION_OBJECT_LINK_OBJECT, &args![this, stream]);
}

// Translated from 00cb7620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSPCollisionObject::SaveBinary` (engine map name), `(stream)`: jumps to `00c65820`.
pub fn bhk_sp_collision_object_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(COLLISION_OBJECT_SAVE_BINARY, &args![this, stream]);
}

// Translated from 00cb7630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSPCollisionObject::IsEqual` (engine map name), `(other)`: jumps to `00c87fb0` and
/// answers what it answers (a bool in the low byte).
pub fn bhk_sp_collision_object_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    e.call(COLLISION_OBJECT_IS_EQUAL, &args![this, other]).u32()
}

// Translated from 00cb7640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb7640 - the FS:[0] exception-unwinding frame is not modeled.
/// No map name (the map places it in `bhkspcollisionobject.obj`). Allocates 0x14 bytes
/// (`operator new`, `00aa13e0`), runs the base constructor body (`00c6b910`) on them and
/// stores the vtable (`01066acc`). Answers the object, or 0 when the allocation failed.
pub fn fn_00cb7640(e: &mut Engine) -> Ptr {
    let block = e.call(HEAP_ALLOCATE, &args![0x14u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    e.call(BASE_CONSTRUCT, &args![block]);
    e.mem.set_u32(block, OBJECT_VTABLE);
    Ptr::new(block)
}

// Translated from 00cb76a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb76a0 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSPCollisionObject::CreateClone` (engine map name), `(process)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, makes a new object as `00cb7640` does (0 when the allocation
/// fails) and runs the member copy (`00c657f0`) on this object with the clone and `process`.
/// Answers the clone.
pub fn bhk_sp_collision_object_create_clone(e: &mut Engine, this: Ptr, process: Ptr) -> Ptr {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    let block = e.call(HEAP_ALLOCATE, &args![0x14u32]).u32();
    let mut clone = 0;
    if block != 0 {
        e.call(BASE_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, OBJECT_VTABLE);
        clone = block;
    }
    e.call(COPY_MEMBERS, &args![this, clone, process]);
    e.mem.set_u32(thread + SCOPE_WORD, saved);
    Ptr::new(clone)
}

// Translated from 00cb7750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSPCollisionObject::hktoNiTransform` (engine map name): when there is a world object
/// (`+0x10`) with a Havok object (`+8` of it), builds the phantom's transform
/// (`bhkShapePhantom::GetTransform`, `00c9ec60`) in a local `NiTransform` and hands it to the
/// object's `SetTransform` (virtual `+0xb8`).
pub fn bhk_sp_collision_object_hk_to_ni_transform(e: &mut Engine, this: Ptr<BhkSPCollisionObject>) {
    let world = e.get(this, BhkSPCollisionObject::worldObject).addr();
    if world == 0 || e.mem.u32(world + 8) == 0 {
        return;
    }
    e.with_stack(NI_TRANSFORM_SIZE, |e, transform| {
        e.call(SHAPE_PHANTOM_GET_TRANSFORM, &args![world, transform]);
        e.vcall(this.addr(), VSLOT_SET_TRANSFORM, &args![transform]);
    });
}

// Translated from 00cb7790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkSPCollisionObject::NitohkTransform` (engine map name): when there is a scene object
/// (`+8`), refreshes its transform (`00a68bf0`); then, when there is a world object (`+0x10`)
/// with a Havok object (`+8` of it), copies the scene object's `NiTransform` (0x34 bytes at
/// `+0x68`) aside and, under the lock at `01267b80` (`0040fbf0(lock, 0)`), hands it to
/// `bhkShapePhantom::SetTransform` (`0056df20`) of the world object; then releases the lock
/// (the count at `01267b84` is lowered and the owner at `01267b80` cleared at zero).
pub fn bhk_sp_collision_object_ni_to_hk_transform(e: &mut Engine, this: Ptr<BhkSPCollisionObject>) {
    let world = e.get(this, BhkSPCollisionObject::worldObject).addr();
    let scene = e.get(this, BhkSPCollisionObject::sceneObject).addr();
    if scene == 0 {
        return;
    }
    e.call(REFRESH_WORLD_TRANSFORM, &args![scene]);
    if world == 0 || e.mem.u32(world + 8) == 0 {
        return;
    }
    e.with_stack(NI_TRANSFORM_SIZE, |e, transform| {
        for word in 0..NI_TRANSFORM_SIZE / 4 {
            let value = e.mem.u32(scene + NODE_WORLD_TRANSFORM + 4 * word);
            e.mem.set_u32(transform.addr() + 4 * word, value);
        }
        e.call(LOCK_ACQUIRE, &args![LOCK, 0u32]);
        e.call(SHAPE_PHANTOM_SET_TRANSFORM, &args![world, transform]);
    });
    let count = e.mem.u32(LOCK + 4).wrapping_sub(1);
    e.mem.set_u32(LOCK + 4, count);
    if count == 0 {
        e.mem.set_u32(LOCK, 0);
    }
}

// Translated from 00cb7800 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00cb7800 - the FS:[0] exception-unwinding frame is not modeled.
/// `bhkSPCollisionObject::AddViewerStrings` (engine map name), `(strings)`: inside the
/// `TLS + 0x2b4 = 0x10` scope, the base strings (`00c882c0`) and the text `NiGetViewerString`
/// makes of the word at `012683d0`, stored in the string array (grown by `0096ad30` when its
/// size, `+0xa`, has reached its capacity, `+8`; stored by `0096ae90`).
pub fn bhk_sp_collision_object_add_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let thread = e.tls();
    let saved = e.mem.u32(thread + SCOPE_WORD);
    e.mem.set_u32(thread + SCOPE_WORD, 0x10);
    e.call(P_COLLISION_OBJECT_ADD_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(CLASS_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![name]).u32();
    let array = strings.addr();
    let size = u32::from(e.mem.u16(array + 0xa));
    if size >= u32::from(e.mem.u16(array + 8)) {
        let grow_by = u32::from(e.mem.u16(array + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![array, grow_by + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRING_ARRAY_SET_AT, &args![array, size, slot]);
    });
    e.mem.set_u32(thread + SCOPE_WORD, saved);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb75f0, bhk_sp_collision_object_load_binary(Ptr, Ptr)),
        entry!(0x00cb7600, bhk_sp_collision_object_link_object(Ptr, Ptr)),
        entry!(0x00cb7620, bhk_sp_collision_object_save_binary(Ptr, Ptr)),
        entry!(0x00cb7630, bhk_sp_collision_object_is_equal(Ptr, Ptr) -> u32),
        entry!(0x00cb7640, fn_00cb7640() -> Ptr),
        entry!(
            0x00cb76a0,
            bhk_sp_collision_object_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(
            0x00cb7750,
            bhk_sp_collision_object_hk_to_ni_transform(Ptr<BhkSPCollisionObject>)
        ),
        entry!(
            0x00cb7790,
            bhk_sp_collision_object_ni_to_hk_transform(Ptr<BhkSPCollisionObject>)
        ),
        entry!(
            0x00cb7800,
            bhk_sp_collision_object_add_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SET_TRANSFORM_SLOT: u32 = 0x7000_00b8;
    const OBJECT_TABLE: u32 = 0x7001_1000;

    fn logged_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order_of(e: &Engine, addresses: &[u32]) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|c| c.0)
            .filter(|a| addresses.contains(a))
            .collect()
    }

    /// A collision object with the scene node and world object given.
    fn object(e: &mut Engine, scene: u32, world: u32) -> u32 {
        let mut slots = vec![0u32; 0x30];
        slots[0xb8 / 4] = SET_TRANSFORM_SLOT;
        e.put_vtable(OBJECT_TABLE, &slots);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this, OBJECT_TABLE);
        e.mem.set_u32(this + 8, scene);
        e.mem.set_u32(this + 0x10, world);
        this
    }

    #[test]
    fn the_stream_functions_jump_to_the_collision_object_ones() {
        let mut e = logged_engine();
        e.register(COLLISION_OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(COLLISION_OBJECT_LINK_OBJECT, |_, _| Ret::default());
        e.register(COLLISION_OBJECT_SAVE_BINARY, |_, _| Ret::default());
        e.call(0x00cb_75f0, &args![0x10u32, 0x20u32]);
        e.call(0x00cb_7600, &args![0x11u32, 0x21u32]);
        e.call(0x00cb_7620, &args![0x12u32, 0x22u32]);
        assert_eq!(
            calls_to(&e, COLLISION_OBJECT_LOAD_BINARY),
            vec![vec![0x10, 0x20]]
        );
        assert_eq!(
            calls_to(&e, COLLISION_OBJECT_LINK_OBJECT),
            vec![vec![0x11, 0x21]]
        );
        assert_eq!(
            calls_to(&e, COLLISION_OBJECT_SAVE_BINARY),
            vec![vec![0x12, 0x22]]
        );
    }

    #[test]
    fn bhk_sp_collision_object_is_equal_answers_what_the_jump_target_answers() {
        let mut e = logged_engine();
        e.register(COLLISION_OBJECT_IS_EQUAL, |_, a| (a[1] + 0x100).into_ret());
        assert_eq!(e.call(0x00cb_7630, &args![0x10u32, 0x21u32]).u32(), 0x121);
        assert_eq!(
            calls_to(&e, COLLISION_OBJECT_IS_EQUAL),
            vec![vec![0x10, 0x21]]
        );
    }

    #[test]
    fn fn_00cb7640_constructs_the_object_and_stores_the_vtable() {
        let mut e = logged_engine();
        e.register(HEAP_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        let object = e.call(0x00cb_7640, &args![]).u32();
        assert_eq!(calls_to(&e, HEAP_ALLOCATE), vec![vec![0x14]]);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![object]]);
        assert_eq!(e.mem.u32(object), OBJECT_VTABLE);
    }

    #[test]
    fn fn_00cb7640_answers_zero_when_the_allocation_fails() {
        let mut e = logged_engine();
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        assert_eq!(e.call(0x00cb_7640, &args![]).u32(), 0);
        assert!(calls_to(&e, BASE_CONSTRUCT).is_empty());
    }

    #[test]
    fn bhk_sp_collision_object_create_clone_copies_the_members_into_the_clone() {
        let mut e = logged_engine();
        e.register(HEAP_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        let clone = e.call(0x00cb_76a0, &args![this, 0x9999u32]).u32();
        assert_eq!(e.mem.u32(clone), OBJECT_VTABLE);
        assert_eq!(calls_to(&e, BASE_CONSTRUCT), vec![vec![clone]]);
        assert_eq!(calls_to(&e, COPY_MEMBERS), vec![vec![this, clone, 0x9999]]);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }

    #[test]
    fn bhk_sp_collision_object_create_clone_copies_into_null_when_the_allocation_fails() {
        let mut e = logged_engine();
        e.register(HEAP_ALLOCATE, |_, _| Ret::default());
        e.register(COPY_MEMBERS, |_, _| Ret::default());
        let this = e.mem.alloc(0x14);
        let clone = e.call(0x00cb_76a0, &args![this, 0x9999u32]).u32();
        assert_eq!(clone, 0);
        assert_eq!(calls_to(&e, COPY_MEMBERS), vec![vec![this, 0, 0x9999]]);
    }

    #[test]
    fn bhk_sp_collision_object_hk_to_ni_transform_sets_the_phantom_transform() {
        let mut e = logged_engine();
        e.register(SHAPE_PHANTOM_GET_TRANSFORM, |e, a| {
            e.mem.set_u32(a[1], 0xfeed);
            Ret::default()
        });
        e.register_double(SET_TRANSFORM_SLOT, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0xfeed);
            Ret::default()
        });
        let world = e.mem.alloc(0x20);
        e.mem.set_u32(world + 8, 0x4000);
        let this = object(&mut e, 0, world);
        e.call(0x00cb_7750, &args![this]);
        let get = calls_to(&e, SHAPE_PHANTOM_GET_TRANSFORM);
        assert_eq!(get.len(), 1);
        assert_eq!(get[0][0], world);
        assert_eq!(
            calls_to(&e, SET_TRANSFORM_SLOT),
            vec![vec![this, get[0][1]]]
        );
    }

    #[test]
    fn bhk_sp_collision_object_hk_to_ni_transform_needs_a_world_object_with_a_havok_object() {
        let mut e = logged_engine();
        e.register(SHAPE_PHANTOM_GET_TRANSFORM, |_, _| Ret::default());
        e.register(SET_TRANSFORM_SLOT, |_, _| Ret::default());
        let world = e.mem.alloc(0x20);
        let without_world = object(&mut e, 0, 0);
        let without_havok = object(&mut e, 0, world);
        e.call(0x00cb_7750, &args![without_world]);
        e.call(0x00cb_7750, &args![without_havok]);
        assert!(calls_to(&e, SHAPE_PHANTOM_GET_TRANSFORM).is_empty());
        assert!(calls_to(&e, SET_TRANSFORM_SLOT).is_empty());
    }

    #[test]
    fn bhk_sp_collision_object_ni_to_hk_transform_sets_the_phantom_transform_under_the_lock() {
        let mut e = logged_engine();
        e.register(REFRESH_WORLD_TRANSFORM, |_, _| Ret::default());
        e.register(LOCK_ACQUIRE, |e, _| {
            e.mem.set_u32(LOCK, 0x1234);
            e.mem.set_u32(LOCK + 4, 1);
            Ret::default()
        });
        e.register_double(SHAPE_PHANTOM_SET_TRANSFORM, |e, a| {
            let words: Vec<u32> = (0..13).map(|n| e.mem.u32(a[1] + 4 * n)).collect();
            assert_eq!(words, (0..13).map(|n| 0x100 + n).collect::<Vec<u32>>());
            assert_eq!(e.mem.u32(LOCK + 4), 1);
            Ret::default()
        });
        let scene = e.mem.alloc(0x100);
        for n in 0..13u32 {
            e.mem.set_u32(scene + 0x68 + 4 * n, 0x100 + n);
        }
        let world = e.mem.alloc(0x20);
        e.mem.set_u32(world + 8, 0x4000);
        let this = object(&mut e, scene, world);
        e.call(0x00cb_7790, &args![this]);
        assert_eq!(calls_to(&e, REFRESH_WORLD_TRANSFORM), vec![vec![scene]]);
        assert_eq!(calls_to(&e, LOCK_ACQUIRE), vec![vec![LOCK, 0]]);
        assert_eq!(calls_to(&e, SHAPE_PHANTOM_SET_TRANSFORM).len(), 1);
        assert_eq!(e.mem.u32(LOCK + 4), 0);
        assert_eq!(e.mem.u32(LOCK), 0);
        assert_eq!(
            order_of(
                &e,
                &[
                    REFRESH_WORLD_TRANSFORM,
                    LOCK_ACQUIRE,
                    SHAPE_PHANTOM_SET_TRANSFORM
                ]
            ),
            vec![
                REFRESH_WORLD_TRANSFORM,
                LOCK_ACQUIRE,
                SHAPE_PHANTOM_SET_TRANSFORM
            ]
        );
    }

    #[test]
    fn bhk_sp_collision_object_ni_to_hk_transform_keeps_the_owner_of_a_nested_lock() {
        let mut e = logged_engine();
        e.register(REFRESH_WORLD_TRANSFORM, |_, _| Ret::default());
        e.register(LOCK_ACQUIRE, |e, _| {
            e.mem.set_u32(LOCK, 0x1234);
            e.mem.set_u32(LOCK + 4, 2);
            Ret::default()
        });
        e.register(SHAPE_PHANTOM_SET_TRANSFORM, |_, _| Ret::default());
        let scene = e.mem.alloc(0x100);
        let world = e.mem.alloc(0x20);
        e.mem.set_u32(world + 8, 0x4000);
        let this = object(&mut e, scene, world);
        e.call(0x00cb_7790, &args![this]);
        assert_eq!(e.mem.u32(LOCK + 4), 1);
        assert_eq!(e.mem.u32(LOCK), 0x1234);
    }

    #[test]
    fn bhk_sp_collision_object_ni_to_hk_transform_stops_early_without_the_objects() {
        let mut e = logged_engine();
        e.register(REFRESH_WORLD_TRANSFORM, |_, _| Ret::default());
        e.register(LOCK_ACQUIRE, |_, _| Ret::default());
        e.register(SHAPE_PHANTOM_SET_TRANSFORM, |_, _| Ret::default());
        let scene = e.mem.alloc(0x100);
        let world = e.mem.alloc(0x20);
        let without_scene = object(&mut e, 0, world);
        let without_world = object(&mut e, scene, 0);
        let without_havok = object(&mut e, scene, world);
        e.mem.set_u32(LOCK + 4, 5);
        e.call(0x00cb_7790, &args![without_scene]);
        assert!(calls_to(&e, REFRESH_WORLD_TRANSFORM).is_empty());
        e.call(0x00cb_7790, &args![without_world]);
        e.call(0x00cb_7790, &args![without_havok]);
        assert_eq!(calls_to(&e, REFRESH_WORLD_TRANSFORM).len(), 2);
        assert!(calls_to(&e, LOCK_ACQUIRE).is_empty());
        assert!(calls_to(&e, SHAPE_PHANTOM_SET_TRANSFORM).is_empty());
        assert_eq!(e.mem.u32(LOCK + 4), 5);
    }

    #[test]
    fn bhk_sp_collision_object_add_viewer_strings_stores_the_class_name() {
        let mut e = logged_engine();
        e.register(P_COLLISION_OBJECT_ADD_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x1111u32.into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            Ret::default()
        });
        e.mem.set_u32(CLASS_RTTI, 0x4242);
        let this = e.mem.alloc(0x14);
        let array = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(array + 4, base);
        e.mem.set_u16(array + 8, 0);
        e.mem.set_u16(array + 0xe, 4);
        let thread = e.tls();
        e.mem.set_u32(thread + SCOPE_WORD, 0x77);
        e.call(0x00cb_7800, &args![this, array]);
        assert_eq!(
            calls_to(&e, P_COLLISION_OBJECT_ADD_VIEWER_STRINGS),
            vec![vec![this, array]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x4242]]);
        assert_eq!(calls_to(&e, STRING_ARRAY_SET_SIZE), vec![vec![array, 4]]);
        assert_eq!(e.mem.u32(base), 0x1111);
        assert_eq!(e.mem.u32(thread + SCOPE_WORD), 0x77);
    }
}
