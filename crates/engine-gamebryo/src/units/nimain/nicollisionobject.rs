//! `NiMain/nicollisionobject.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiCollisionObject::ms_RTTI` (the class's `NiRTTI` record).
const NI_COLLISION_OBJECT_RTTI: u32 = 0x011f_4ad4;

// Translated from 00a810a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionObject::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_collision_object_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_COLLISION_OBJECT_RTTI)
}

/// `NiObject::LoadBinary` (`00a5d610`, `this` and the stream).
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
/// `NiStream` reader of a link id into the stream's link list (`00a645b0`,
/// the stream only; unnamed in the engine map).
const NI_STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// The base `LinkObject` (`00a89af0`, `this` and the stream; its body is a
/// bare `RET 4`).
const NI_OBJECT_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, the stream only).
const NI_STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `NiAVObject::SetCollisionObject` (`0062bc90`, the scene object and the
/// collision object).
const NI_AV_OBJECT_SET_COLLISION_OBJECT: u32 = 0x0062_bc90;
/// `NiObject`'s constructor (`00a5d3a0`, `this` only).
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
/// The `NiCollisionObject` vtable (its RTTI locator names
/// `.?AVNiCollisionObject@@`).
const NI_COLLISION_OBJECT_VTABLE: u32 = 0x0109_e174;
/// `NiCollisionObject::m_pkSceneObject` (Xbox PDB) +0x08.
const SCENE_OBJECT: u32 = 0x08;
/// `NiAVObject::m_spCollisionObject` (Xbox PDB) +0x1C.
const AV_OBJECT_COLLISION_OBJECT: u32 = 0x1c;

// Translated from 00a80fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionObject::LoadBinary` (Xbox PDB): the `NiObject` part, then
/// the stream reads the scene object's link id (`00a645b0`).
pub fn ni_collision_object_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.call(NI_STREAM_READ_LINK_ID, &args![stream]);
}

// Translated from 00a80ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionObject::LinkObject` (Xbox PDB): the base part, then the scene
/// object is the stream's object for the next link id.
pub fn ni_collision_object_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_LINK_OBJECT, &args![this, stream]);
    let object = e
        .call(NI_STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream])
        .u32();
    e.mem
        .set_u32(this.addr().wrapping_add(SCENE_OBJECT), object);
}

/// Stores `object` as the scene object and, when it is not null and does
/// not already hold `this` as its collision object, gives it `this`
/// (`NiAVObject::SetCollisionObject`).
fn set_scene_object(e: &mut Engine, this: Ptr, object: Ptr) {
    e.mem
        .set_u32(this.addr().wrapping_add(SCENE_OBJECT), object.addr());
    if !object.is_null()
        && e.mem
            .u32(object.addr().wrapping_add(AV_OBJECT_COLLISION_OBJECT))
            != this.addr()
    {
        e.call(NI_AV_OBJECT_SET_COLLISION_OBJECT, &args![object, this]);
    }
}

// Translated from 00a810b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionObject::SetSceneGraphObject` (Xbox PDB): stores the scene
/// object and, when it is not null and its collision object is not already
/// `this`, sets `this` as its collision object (a tail jump).
pub fn ni_collision_object_set_scene_graph_object(e: &mut Engine, this: Ptr, object: Ptr) {
    set_scene_object(e, this, object);
}

// Translated from 00a810d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiCollisionObject` constructor taking the scene object: runs the
/// `NiObject` constructor, stores the class's vtable, then the scene object
/// as `SetSceneGraphObject` does. Returns `this`.
pub fn fn_00a810d0(e: &mut Engine, this: Ptr, object: Ptr) -> Ptr {
    e.call(NI_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_COLLISION_OBJECT_VTABLE);
    set_scene_object(e, this, object);
    this
}

/// `NiObject::RegisterStreamables` (`00a5d2f0`), `00a89af0` (the base's `SaveBinary`, the body
/// the base's `LinkObject` shares), `NiObject::IsEqual` (`00a5d470`) and
/// `NiObject::GetViewerStrings` (`00a5d6e0`), each `thiscall` with one word.
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
const NI_OBJECT_SAVE_BINARY: u32 = 0x00a8_9af0;
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const NI_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
/// `NiGetViewerString` of a class name (`00a7d330`) and of an object pointer (`00a7d5a0`,
/// `cdecl`, name and pointer); the member name `0109e214`.
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const GET_VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
const NAME_SCENE_OBJECT: u32 = 0x0109_e214;
/// `0096ad30` grows an `NiTArray`; `0096ae90` stores an element (both `thiscall`).
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;

// Translated from 00a81010 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionObject::RegisterStreamables` (Xbox PDB): `NiObject::RegisterStreamables`
/// (false stops), then the scene object's (+8, when set) `RegisterStreamables` (its slot
/// +0x54, the answer ignored); true.
pub fn ni_collision_object_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if !e
        .call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let object = e.mem.u32(this.addr() + SCENE_OBJECT);
    if object != 0 {
        e.vcall(object, 0x54, &args![stream]);
    }
    true
}

// Translated from 00a81040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionObject::SaveBinary` (Xbox PDB): the base's `SaveBinary` (`00a89af0`), then the
/// scene object's (+8) link id through the stream's slot +0x30.
pub fn ni_collision_object_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(NI_OBJECT_SAVE_BINARY, &args![this, stream]);
    let object = e.mem.u32(this.addr() + SCENE_OBJECT);
    e.vcall(stream.addr(), 0x30, &args![object]);
}

// Translated from 00a81060 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionObject::IsEqual` (Xbox PDB): `NiObject::IsEqual`, then whether this one having a
/// scene object (+8) matches the other object pointer being non-null (the exe tests the other
/// pointer itself, not its scene object).
pub fn ni_collision_object_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let object = e.mem.u32(this.addr() + SCENE_OBJECT);
    (object != 0) == (other.addr() != 0)
}

/// Appends the string pointer held in the word at `slot` to the viewer-string array (grown to
/// count + `m_usGrowBy` by `0096ad30` when full, stored by `0096ae90`).
fn append_viewer_string(e: &mut Engine, strings: Ptr, slot: u32) {
    let count = e.mem.u16(strings.addr() + 0xa) as u32;
    let capacity = e.mem.u16(strings.addr() + 8) as u32;
    if count >= capacity {
        let grow_by = e.mem.u16(strings.addr() + 0xe) as u32;
        e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
    }
    e.call(ARRAY_SET_AT, &args![strings, count, slot]);
}

/// Appends the class name (`NiGetViewerString` of the first word of the `NiRTTI` at
/// `011f4ad4`).
fn append_class_name(e: &mut Engine, strings: Ptr) {
    let class_name = e.global::<u32>(NI_COLLISION_OBJECT_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        append_viewer_string(e, strings, slot.addr());
    });
}

// Translated from 00a81100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionObject::GetViewerStrings` (Xbox PDB): `NiObject::GetViewerStrings`, the class
/// name, then the object's `AddViewerStrings` (its slot +0x64).
pub fn ni_collision_object_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(NI_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    append_class_name(e, strings);
    e.vcall(this.addr(), 0x64, &args![strings]);
}

// Translated from 00a81160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionObject::AddViewerStrings` (Xbox PDB): the class name, then the scene object
/// (+8) under the name at `0109e214` (`00a7d5a0`).
pub fn ni_collision_object_add_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    append_class_name(e, strings);
    let object = e.mem.u32(this.addr() + SCENE_OBJECT);
    let text = e
        .call(GET_VIEWER_STRING_POINTER, &args![NAME_SCENE_OBJECT, object])
        .u32();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        append_viewer_string(e, strings, slot.addr());
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a810a0, ni_collision_object_get_rtti() -> Ptr),
        entry!(0x00a80fd0, ni_collision_object_load_binary(Ptr, Ptr)),
        entry!(0x00a80ff0, ni_collision_object_link_object(Ptr, Ptr)),
        entry!(
            0x00a810b0,
            ni_collision_object_set_scene_graph_object(Ptr, Ptr)
        ),
        entry!(0x00a810d0, fn_00a810d0(Ptr, Ptr) -> Ptr),
        entry!(0x00a8_1010, ni_collision_object_register_streamables(Ptr, Ptr) -> bool),
        entry!(0x00a8_1040, ni_collision_object_save_binary(Ptr, Ptr)),
        entry!(0x00a8_1060, ni_collision_object_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a8_1100,
            ni_collision_object_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(
            0x00a8_1160,
            ni_collision_object_add_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_collision_object_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a810a0, &args![]).u32(), NI_COLLISION_OBJECT_RTTI);
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .iter()
            .flatten()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    #[test]
    fn load_binary_loads_the_base_then_reads_the_link_id() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_LOAD_BINARY, nothing);
        e.register(NI_STREAM_READ_LINK_ID, nothing);
        e.call_log = Some(vec![]);
        e.call(0x00a8_0fd0, &args![0x100u32, 0x200u32]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_LOAD_BINARY),
            vec![vec![0x100, 0x200]]
        );
        assert_eq!(calls_to(&e, NI_STREAM_READ_LINK_ID), vec![vec![0x200]]);
    }

    #[test]
    fn link_object_stores_the_linked_scene_object() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_LINK_OBJECT, nothing);
        e.register(NI_STREAM_GET_OBJECT_FROM_LINK_ID, |_, _| {
            0x4444u32.into_ret()
        });
        let this = e.mem.alloc(0x0c);
        e.call_log = Some(vec![]);
        e.call(0x00a8_0ff0, &args![this, 0x200u32]);
        assert_eq!(calls_to(&e, NI_OBJECT_LINK_OBJECT), vec![vec![this, 0x200]]);
        assert_eq!(
            calls_to(&e, NI_STREAM_GET_OBJECT_FROM_LINK_ID),
            vec![vec![0x200]]
        );
        assert_eq!(e.mem.u32(this + SCENE_OBJECT), 0x4444);
    }

    #[test]
    fn set_scene_graph_object_links_back_only_when_needed() {
        let mut e = Engine::new();
        e.register(NI_AV_OBJECT_SET_COLLISION_OBJECT, nothing);
        let this = e.mem.alloc(0x0c);
        let object = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        // Null object: stored, nothing else.
        e.mem.set_u32(this + SCENE_OBJECT, 7);
        e.call(0x00a8_10b0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + SCENE_OBJECT), 0);
        // The object already holds `this`.
        e.mem.set_u32(object + AV_OBJECT_COLLISION_OBJECT, this);
        e.call(0x00a8_10b0, &args![this, object]);
        assert_eq!(e.mem.u32(this + SCENE_OBJECT), object);
        assert!(calls_to(&e, NI_AV_OBJECT_SET_COLLISION_OBJECT).is_empty());
        // Another collision object: gives it `this`.
        e.mem.set_u32(object + AV_OBJECT_COLLISION_OBJECT, 0);
        e.call(0x00a8_10b0, &args![this, object]);
        assert_eq!(
            calls_to(&e, NI_AV_OBJECT_SET_COLLISION_OBJECT),
            vec![vec![object, this]]
        );
    }

    #[test]
    fn constructor_with_scene_object_sets_vtable_and_links() {
        let mut e = Engine::new();
        e.register(NI_OBJECT_CONSTRUCT, nothing);
        e.register(NI_AV_OBJECT_SET_COLLISION_OBJECT, nothing);
        let this = e.mem.alloc(0x0c);
        let object = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a8_10d0, &args![this, object]).u32(), this);
        assert_eq!(calls_to(&e, NI_OBJECT_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), NI_COLLISION_OBJECT_VTABLE);
        assert_eq!(e.mem.u32(this + SCENE_OBJECT), object);
        assert_eq!(
            calls_to(&e, NI_AV_OBJECT_SET_COLLISION_OBJECT),
            vec![vec![object, this]]
        );
        let other = e.mem.alloc(0x0c);
        e.call(0x00a8_10d0, &args![other, 0u32]);
        assert_eq!(e.mem.u32(other + SCENE_OBJECT), 0);
        assert_eq!(calls_to(&e, NI_AV_OBJECT_SET_COLLISION_OBJECT).len(), 1);
    }
}

#[cfg(test)]
mod n015_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::calls_to;

    const SLOT: u32 = 0x0300_0000;

    /// An object whose vtable slots +0x30, +0x54 and +0x64 are the recording double `SLOT`.
    fn object(e: &mut Engine) -> u32 {
        let vtable = e.mem.alloc(0x80);
        for slot in [0x30, 0x54, 0x64] {
            e.mem.set_u32(vtable + slot, SLOT);
        }
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        object
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(SLOT, |_, _| Ret::default());
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        e.register(NI_OBJECT_SAVE_BINARY, |_, _| Ret::default());
        e.register(NI_OBJECT_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(NI_OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| 0x7000u32.into_ret());
        e.register(GET_VIEWER_STRING_POINTER, |_, _| 0x8000u32.into_ret());
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |e, a| {
            let count = e.mem.u16(a[0] + 0xa) + 1;
            e.mem.set_u16(a[0] + 0xa, count);
            Ret::default()
        });
        e.map(0x011f_4000, 0x1000);
        e.mem.set_u32(NI_COLLISION_OBJECT_RTTI, 0x0102_0000);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn register_streamables_passes_on_to_the_scene_object() {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        let scene = object(&mut e);
        assert!(e.call(0x00a8_1010, &args![this, 0x55u32]).bool());
        assert!(calls_to(&e, SLOT).is_empty());
        e.mem.set_u32(this + 8, scene);
        assert!(e.call(0x00a8_1010, &args![this, 0x55u32]).bool());
        assert_eq!(calls_to(&e, SLOT), vec![vec![scene, 0x55]]);
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a8_1010, &args![this, 0x55u32]).bool());
        assert_eq!(calls_to(&e, SLOT).len(), 1);
    }

    #[test]
    fn save_binary_saves_the_scene_link() {
        let mut e = engine();
        let stream = object(&mut e);
        let this = e.mem.alloc(0x14);
        e.mem.set_u32(this + 8, 0x4444);
        e.call(0x00a8_1040, &args![this, stream]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(calls_to(&e, SLOT), vec![vec![stream, 0x4444]]);
    }

    #[test]
    fn is_equal_tests_scene_against_the_other_pointer() {
        let mut e = engine();
        let this = e.mem.alloc(0x14);
        let other = e.mem.alloc(0x14);
        assert!(!e.call(0x00a8_1060, &args![this, other]).bool());
        e.mem.set_u32(this + 8, 0x4444);
        assert!(e.call(0x00a8_1060, &args![this, other]).bool());
        assert!(!e.call(0x00a8_1060, &args![this, 0u32]).bool());
        e.mem.set_u32(this + 8, 0);
        assert!(e.call(0x00a8_1060, &args![this, 0u32]).bool());
        e.register(NI_OBJECT_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a8_1060, &args![this, 0u32]).bool());
    }

    #[test]
    fn viewer_strings_add_class_then_the_object_strings() {
        let mut e = engine();
        let this = object(&mut e);
        e.mem.set_u32(this + 8, 0x4444);
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 8);
        e.call(0x00a8_1100, &args![this, strings]);
        assert_eq!(
            calls_to(&e, NI_OBJECT_GET_VIEWER_STRINGS),
            vec![vec![this, strings]]
        );
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x0102_0000]]);
        assert_eq!(calls_to(&e, SLOT), vec![vec![this, strings]]);
        assert_eq!(e.mem.u16(strings + 0xa), 1);
        e.call(0x00a8_1160, &args![this, strings]);
        assert_eq!(
            calls_to(&e, GET_VIEWER_STRING_POINTER),
            vec![vec![NAME_SCENE_OBJECT, 0x4444]]
        );
        assert_eq!(e.mem.u16(strings + 0xa), 3);
        assert!(calls_to(&e, ARRAY_SET_SIZE).is_empty());
    }
}
