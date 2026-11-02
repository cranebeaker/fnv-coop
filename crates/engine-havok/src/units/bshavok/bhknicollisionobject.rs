//! `BSHavok/bhknicollisionobject.obj` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `bhkNiCollisionObject` joins a scene graph node (`m_pkSceneObject`) with a Havok world object
//! (`spWorldObject`). `m_uFlags` bits, named by the viewer strings `AddViewerStrings` writes:
//! bit 0 `bActive`, bit 2 `bNotify`, bit 3 `bSetLocal`, bit 6 `bReset`; bit 4 says that the debug
//! display is shown (`SetDebugDisplay`).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00a810b0`: the jump target of `00c6b5e0`.
const JUMP_TARGET_00C6B5E0: u32 = 0x00a8_10b0;

/// `bhkNiCollisionObject`'s vtable (`010c4314`).
const VTABLE: u32 = 0x010c_4314;
/// `bhkNiCollisionObject::iCount` (Xbox PDB static): the number of live collision objects.
const OBJECT_COUNT: u32 = 0x0126_7b60;
/// `pTransChangeFunc` (Xbox PDB static): the function `SetTransform` calls (cdecl, `(object,
/// change_flags)`) when it is set.
const TRANSFORM_CHANGE_HOOK: u32 = 0x0126_7b68;
/// A byte that, when set, makes `SetTransform` take the new transform without comparing it.
const ALWAYS_REPORT_TRANSFORM: u32 = 0x0126_7b6c;
/// A word whose value `AddViewerStrings` shows as the class name (an `NiRTTI` pointer). Its
/// address is also the key of the property `SetWorldObject` puts on the Havok object.
const CLASS_RTTI: u32 = 0x0126_7b70;
/// The pooled empty `NiFixedString` handle (it has no reference count to release).
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// `"bhkColDisp"`: the name of the debug display node.
const DEBUG_DISPLAY_NAME: u32 = 0x010c_43d8;
/// The rotation tolerance (`float`) `SetTransform` passes to the matrix comparison.
const ROTATION_EPSILON: u32 = 0x0101_7d00;
/// The translation tolerance (`float`) `SetTransform` passes to the row comparison.
const TRANSLATION_EPSILON: u32 = 0x0101_3ea4;

/// `KERNEL32` imports (slots).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// `NiCollisionObject` methods the class builds on.
const BASE_CONSTRUCTOR: u32 = 0x00a8_0fa0;
const BASE_CONSTRUCTOR_WITH_SCENE_OBJECT: u32 = 0x00a8_10d0;
const BASE_DESTRUCTOR_BODY: u32 = 0x00a8_0fc0;
const BASE_REGISTER_STREAMABLES: u32 = 0x00a8_1010;
const BASE_IS_EQUAL: u32 = 0x00a8_1060;
const BASE_LINK_OBJECT: u32 = 0x00a8_0ff0;
const BASE_LOAD_BINARY: u32 = 0x00a8_0fd0;
const BASE_SAVE_BINARY: u32 = 0x00a8_1040;
const BASE_ADD_VIEWER_STRINGS: u32 = 0x00a8_1160;
/// `00aa1000(this, destination, cloning_process)`: `NiObject::CopyMembers` through a jump.
const BASE_COPY_MEMBERS: u32 = 0x00aa_1000;

/// `hkpWorldObject::removeProperty(this, out, key)` (`00cd2a50`) and `addProperty(this, key,
/// value_a, value_b)` (`00cd2be0`).
const REMOVE_PROPERTY: u32 = 0x00cd_2a50;
const ADD_PROPERTY: u32 = 0x00cd_2be0;
/// `00a29680(object)`: a lock call the PC build compiled to a bare `RET`.
const EMPTY_LOCK: u32 = 0x00a2_9680;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, thiscall, no stack arguments) and the stream's
/// post-load step (`00a645b0`, thiscall).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_FINISH_BINARY: u32 = 0x00a6_45b0;
/// `NiTPointerMap::GetAt(this, key, out)` (`00a53c30`) on the cloning process's map.
const CLONE_MAP_GET_AT: u32 = 0x00a5_3c30;
/// `NiPointer::operator=(this, object)` (`00a5f860`; `this` is the pointer's address).
const NI_POINTER_ASSIGN: u32 = 0x00a5_f860;
/// `NiPointer` destructor (`0045cec0`; `this` is the pointer's address).
const NI_POINTER_RELEASE: u32 = 0x0045_cec0;
/// `NiFixedString::NiFixedString(this, const char*)` as the exe inlines it (`00438170`) and the
/// `NiFixedString` creation (`00a5b690`, cdecl, returns the handle).
const FIXED_STRING_CONSTRUCTOR: u32 = 0x0043_8170;
const FIXED_STRING_CREATE: u32 = 0x00a5_b690;
/// `NiObjectNET::SetName(this, const NiFixedString&)` (`00a5b950`).
const SET_NAME: u32 = 0x00a5_b950;
/// `NiAVObject::GetObjectByName(this, NiFixedString*)` (`00a59e20`).
const NI_AV_OBJECT_GET_OBJECT_BY_NAME: u32 = 0x00a5_9e20;
/// `NiAVObject::Update(this, NiUpdateData&)` (`00a59c60`) and `UpdateProperties` (`00a5a040`).
const NI_AV_OBJECT_UPDATE: u32 = 0x00a5_9c60;
const NI_AV_OBJECT_UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// The step `SynchronizeRec` runs on a node before its `UpdateWorldData` (`00a68bf0`, thiscall).
const NI_AV_OBJECT_PREPARE_UPDATE: u32 = 0x00a6_8bf0;
/// `NiNode` constructor (`00a5ecb0`, `(this, capacity)`) and `operator new` (`00aa13e0`, cdecl).
const NI_NODE_CONSTRUCTOR: u32 = 0x00a5_ecb0;
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiTransform::InvertNonUniform(this, out)` (`004b4880`) and the transform multiply of
/// `TESHavokUtilities` (`0062c250`, `(this, out, other)`, returns `out`).
const TRANSFORM_INVERT_NON_UNIFORM: u32 = 0x004b_4880;
const TRANSFORM_MULTIPLY: u32 = 0x0062_c250;
/// `0049e2f0(row_a, row_b, epsilon)` (cdecl): compares two rows of three floats.
const ROW_NEARLY_EQUAL: u32 = 0x0049_e2f0;
/// `00c85d20(world_object, flag)`: a `bhkWorldObject` method.
const WORLD_OBJECT_SET_FLAG: u32 = 0x00c8_5d20;
/// `00653290(address)` (`thiscall` on the world object, RET 4): an accessor that returns an
/// object for the static at `address`.
const STATIC_ACCESSOR: u32 = 0x0065_3290;
/// The static `SetRigidBodyName` hands to the accessor.
const RIGID_BODY_NAME_STATIC: u32 = 0x0126_81c0;
/// `hkStringPtr::operator=(this, NiFixedString handle)` (`00cb0c10`).
const STRING_PTR_ASSIGN: u32 = 0x00cb_0c10;
/// The sized delete (`00aa1460`, cdecl `(block, size)`).
const SIZED_DELETE: u32 = 0x00aa_1460;

/// The viewer-string helpers: `NiGetViewerString(NiRTTI*)` (`00a7d330`), the `(name, text)`
/// (`00a7d5e0`) and `(name, bool)` (`00a7d370`) overloads, and the string array's grow
/// (`0096ad30`) and set-at (`0096ae90`).
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
const NI_GET_VIEWER_STRING_TEXT: u32 = 0x00a7_d5e0;
const NI_GET_VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The formatter (`00406d00`, cdecl: `(buffer, size, format, ...)`).
const FORMAT_TEXT: u32 = 0x0040_6d00;
/// Viewer-string texts: the format `"0x%8X"`, then `"bhkWorldObject"`, `"bActive"`, `"bReset"`,
/// `"bNotify"`, `"bSetLocal"`, `"bKeyframe"` and `"bFaceDown"`.
const FORMAT_WORLD_OBJECT: u32 = 0x010c_4430;
const NAME_WORLD_OBJECT: u32 = 0x010c_4420;
const NAME_ACTIVE: u32 = 0x010c_4418;
const NAME_RESET: u32 = 0x010c_4410;
const NAME_NOTIFY: u32 = 0x010c_4408;
const NAME_SET_LOCAL: u32 = 0x010c_43fc;
const NAME_KEYFRAME: u32 = 0x010c_43f0;
const NAME_FACE_DOWN: u32 = 0x010c_43e4;

/// The thread block (`e.tls()`) word that debug display and viewer strings set to 0x10 while they
/// run, and the word `Synchronize` sets to its second argument while it runs.
const THREAD_CONTEXT: u32 = 0x2b4;
const THREAD_CONTEXT_VALUE: u32 = 0x10;
const THREAD_SYNCHRONIZE_ARGUMENT: u32 = 0x2c0;

layout! {
    /// `bhkNiCollisionObject` (Xbox PDB), 0x14 bytes.
    pub struct BhkNiCollisionObject: 0x14 {
        /// `m_pkSceneObject` (Xbox PDB, `NiCollisionObject`): the `NiAVObject*` it is attached to.
        0x08 m_pkSceneObject: Ptr,
        /// `m_uFlags` (Xbox PDB).
        0x0c m_uFlags: u16,
        /// `spWorldObject` (Xbox PDB): the `bhkWorldObject` (a `NiPointer`).
        0x10 spWorldObject: Ptr,
    }
}

/// `NiPointer` release: `InterlockedDecrement(object + 4)`, and the object's virtual delete
/// (slot `+4`) when the count reaches zero. Nothing for a null pointer.
pub(crate) fn release_reference(e: &mut Engine, object: u32) {
    if object != 0
        && e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// Releases an `NiFixedString` handle: `InterlockedDecrement(handle - 8)` unless it is the pooled
/// empty string.
pub(crate) fn release_fixed_string(e: &mut Engine, handle: u32) {
    if handle != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// Appends the viewer string `text` to the array `strings` (size and capacity are 16-bit fields
/// at `+0xA` and `+8`; `+0xE` is the grow step): grows it when full, then sets the element at the
/// old size through `0096ae90`.
pub(crate) fn push_viewer_string(e: &mut Engine, strings: u32, text: u32) {
    let size = u32::from(e.mem.u16(strings + 0xa));
    if size >= u32::from(e.mem.u16(strings + 8)) {
        let step = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

/// Sets the thread block's `+0x2b4` word to 0x10; returns its address and the previous value.
pub(crate) fn enter_context(e: &mut Engine) -> (u32, u32) {
    let word = e.tls() + THREAD_CONTEXT;
    let previous = e.mem.u32(word);
    e.mem.set_u32(word, THREAD_CONTEXT_VALUE);
    (word, previous)
}

/// Runs `f` with an `NiUpdateData` as the exe builds it for these calls: `fTime = 0.0` and five
/// zero bytes after it (the three padding bytes that follow are not written by the exe; they are
/// zero here).
fn with_zero_update_data<R>(e: &mut Engine, f: impl FnOnce(&mut Engine, Ptr) -> R) -> R {
    e.with_stack(0xc, f)
}

/// Copies `count` words from `from` to `to`, raw (a `REP MOVSD`).
pub(crate) fn copy_words(e: &mut Engine, from: u32, to: u32, count: u32) {
    for word in 0..count {
        let value = e.mem.u32(from + 4 * word);
        e.mem.set_u32(to + 4 * word, value);
    }
}

/// The number of children of the `NiNode` at `node` (`u16` at `+0xa6`).
fn child_count(e: &Engine, node: u32) -> u32 {
    u32::from(e.mem.u16(node + 0xa6))
}

/// The child at `index` of `node`'s child array (the array pointer at `+0xa0` is read again for
/// every element, as the exe does).
fn child_at(e: &Engine, node: u32, index: u32) -> u32 {
    let array = e.mem.u32(node + 0xa0);
    e.mem.u32(array + 4 * index)
}

// Translated from 00c6b5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::SetSceneGraphObject` (Xbox PDB): a 5-byte jump to `00a810b0` (engine map: `NiCollisionObject::SetSceneGraphObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SetSceneGraphObject@bhkNiCollisionObject@@UAAXPAVNiAVObject@@@Z`.
pub fn bhk_ni_collision_object_set_scene_graph_object(e: &mut Engine, this: Ptr, arg: Ptr) {
    e.call(JUMP_TARGET_00C6B5E0, &args![this, arg]);
}

// Translated from 00c6b5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares two 3x3 float matrices (three rows of three floats, 12 bytes apart) row by row: each
/// row is copied (float load and store) to a local pair and handed to `0049e2f0(row_a, row_b,
/// epsilon)`. True when every row passes, false at the first row that does not. cdecl.
pub fn fn_00c6b5f0(e: &mut Engine, a: Ptr, b: Ptr, epsilon: f32) -> bool {
    let epsilon_bits = x87_quiet_bits(epsilon.to_bits());
    e.with_stack(0x18, |e, rows| {
        let row_b = rows.addr();
        let row_a = rows.addr() + 0xc;
        for row in 0..3u32 {
            for lane in 0..3u32 {
                let from_a = x87_quiet_bits(e.mem.u32(a.addr() + 12 * row + 4 * lane));
                e.mem.set_u32(row_a + 4 * lane, from_a);
                let from_b = x87_quiet_bits(e.mem.u32(b.addr() + 12 * row + 4 * lane));
                e.mem.set_u32(row_b + 4 * lane, from_b);
            }
            if !e
                .call(ROW_NEARLY_EQUAL, &args![row_a, row_b, epsilon_bits])
                .bool()
            {
                return false;
            }
        }
        true
    })
}

// Translated from 00c6b730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::RegisterStreamables` (Xbox PDB): registers the world object with the
/// stream (its virtual slot `+0x54`) when there is one, then runs the base class
/// (`NiCollisionObject::RegisterStreamables`, `00a81010`) and returns its result.
pub fn bhk_ni_collision_object_register_streamables(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    stream: Ptr,
) -> bool {
    let body = e.get(this, BhkNiCollisionObject::spWorldObject);
    if !body.is_null() {
        e.vcall(body.addr(), 0x54, &args![stream]);
    }
    e.call(BASE_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
}

// Translated from 00c6b760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::IsEqual` (Xbox PDB): the base comparison (`00a81060`) must pass; then
/// the world objects are compared through virtual slot `+0x5c` (or must both be missing), and the
/// `m_uFlags` words must match. With a world object the result is the AND of the slot's byte and
/// the flags comparison.
pub fn bhk_ni_collision_object_is_equal(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    other: Ptr<BhkNiCollisionObject>,
) -> bool {
    if !e.call(BASE_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let body = e.get(this, BhkNiCollisionObject::spWorldObject);
    if body.is_null() {
        let both_missing = e.get(other, BhkNiCollisionObject::spWorldObject).is_null();
        let same_flags = e.get(other, BhkNiCollisionObject::m_uFlags)
            == e.get(this, BhkNiCollisionObject::m_uFlags);
        both_missing && same_flags
    } else {
        let other_body = e.get(other, BhkNiCollisionObject::spWorldObject);
        let bodies_equal = e.vcall(body.addr(), 0x5c, &args![other_body]).u8();
        let same_flags = e.get(other, BhkNiCollisionObject::m_uFlags)
            == e.get(this, BhkNiCollisionObject::m_uFlags);
        bodies_equal & u8::from(same_flags) != 0
    }
}

/// The float at `offset` of `node` (or of the object `node` finds by `name`, when `name` is not
/// null and the lookup succeeds) is greater than zero. The shared body of `00c6b7b0` and
/// `00c6b860`, which differ only in the offset. A null `node` gives false.
///
/// The name is turned into an `NiFixedString` (`00a5b690`), looked up through the node's virtual
/// slot `+0x9c` (`NiNode::GetObjectByName`) and the temporary released afterwards.
fn named_object_float_is_positive(e: &mut Engine, node: Ptr, name: u32, offset: u32) -> bool {
    if node.is_null() {
        return false;
    }
    let mut target = node.addr();
    if name != 0 {
        let handle = e.call(FIXED_STRING_CREATE, &args![name]).u32();
        let found = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), handle);
            e.vcall(node.addr(), 0x9c, &args![slot]).u32()
        });
        release_fixed_string(e, handle);
        if found != 0 {
            target = found;
        }
    }
    0.0 < e.mem.f32(target + offset)
}

// Translated from 00c6b7b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6b7b0 - C++ exception unwinding (SEH frame state) of the NiFixedString temporary is not translated.
/// `bhkNiCollisionObject::GetFaceUp` (engine map name, unconfirmed): cdecl `(node, name)`. False
/// for a null node; otherwise looks up `name` below `node` when it is not null (the node itself
/// when the name is null or not found) and returns whether the float at `+0x84` of that object
/// (the 8th element of its world rotation matrix at `+0x68`) is greater than zero; NaN gives false.
pub fn bhk_ni_collision_object_get_face_up(e: &mut Engine, node: Ptr, name: u32) -> bool {
    named_object_float_is_positive(e, node, name, 0x84)
}

// Translated from 00c6b860 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6b860 - C++ exception unwinding (SEH frame state) of the NiFixedString temporary is not translated.
/// Unnamed in the engine map; the same as `00c6b7b0` but for the float at `+0x88` (the last
/// element of the world rotation matrix): cdecl `(node, name)`.
pub fn fn_00c6b860(e: &mut Engine, node: Ptr, name: u32) -> bool {
    named_object_float_is_positive(e, node, name, 0x88)
}

/// What both constructors do after the base class: vtable, no world object, the object count and
/// the initial flags.
fn finish_construction(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
) -> Ptr<BhkNiCollisionObject> {
    e.mem.set_u32(this.addr(), VTABLE);
    e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::NULL);
    let count = e.mem.u32(OBJECT_COUNT);
    e.mem.set_u32(OBJECT_COUNT, count.wrapping_add(1));
    e.set(this, BhkNiCollisionObject::m_uFlags, 0x41);
    this
}

// Translated from 00c6b910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkNiCollisionObject` constructor without arguments (unnamed in the engine map): runs the
/// `NiCollisionObject` constructor (`00a80fa0`), sets the vtable, clears the world object,
/// counts the object (`iCount`) and sets `m_uFlags` to 0x41. Returns `this`.
pub fn fn_00c6b910(e: &mut Engine, this: Ptr<BhkNiCollisionObject>) -> Ptr<BhkNiCollisionObject> {
    e.call(BASE_CONSTRUCTOR, &args![this]);
    finish_construction(e, this)
}

// Translated from 00c6b950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bhkNiCollisionObject` constructor with the scene object (unnamed in the engine map): the
/// `NiCollisionObject` constructor `00a810d0(this, scene_object)`, then as `00c6b910`. Returns
/// `this`.
pub fn fn_00c6b950(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    scene_object: Ptr,
) -> Ptr<BhkNiCollisionObject> {
    e.call(
        BASE_CONSTRUCTOR_WITH_SCENE_OBJECT,
        &args![this, scene_object],
    );
    finish_construction(e, this)
}

// Translated from 00c6b980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::SetWorldObject` (engine map name): when `body` differs from the current
/// world object, removes the property keyed by `01267b70` from the old one's Havok object
/// (`[body + 8]`, `hkpWorldObject::removeProperty`), replaces the `NiPointer` (releasing the old
/// object, taking a reference on the new one), and adds the property (key, this object, 0) to the
/// new one's Havok object. The empty lock call (`00a29680`) brackets each property change.
pub fn bhk_ni_collision_object_set_world_object(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    body: Ptr,
) {
    let current = e.get(this, BhkNiCollisionObject::spWorldObject);
    if current == body {
        return;
    }
    if !current.is_null() {
        let havok_object = e.mem.u32(current.addr() + 8);
        if havok_object != 0 {
            e.call(EMPTY_LOCK, &args![current]);
            e.with_stack(8, |e, removed| {
                e.call(REMOVE_PROPERTY, &args![havok_object, removed, CLASS_RTTI]);
            });
            e.call(EMPTY_LOCK, &args![current]);
        }
    }
    let old = e.get(this, BhkNiCollisionObject::spWorldObject);
    if old != body {
        release_reference(e, old.addr());
        e.set(this, BhkNiCollisionObject::spWorldObject, body);
        if !body.is_null() {
            e.call(INTERLOCKED_INCREMENT, &args![body.addr() + 4]);
        }
    }
    let attached = e.get(this, BhkNiCollisionObject::spWorldObject);
    if !attached.is_null() {
        let havok_object = e.mem.u32(attached.addr() + 8);
        if havok_object != 0 {
            e.call(EMPTY_LOCK, &args![attached]);
            e.call(ADD_PROPERTY, &args![havok_object, CLASS_RTTI, this, 0u32]);
            e.call(EMPTY_LOCK, &args![attached]);
        }
    }
}

// Translated from 00c6ba30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6ba30 - C++ exception unwinding (SEH frame state) of the NiPointer temporaries is not translated.
/// The clone step of `bhkNiCollisionObject` (unnamed in the engine map, cdecl-free `thiscall`
/// `(this, destination, process)`): runs `NiObject::CopyMembers` (`00aa1000`) on `destination`,
/// copies `m_uFlags`, and gives the destination a clone of the world object: the cloning process's
/// map (`[process]`) is asked for an existing clone of the world object (`00a53c30`), otherwise
/// the world object's virtual slot `+0x48` (`CreateClone`) makes one; it is held in a temporary
/// `NiPointer` and set with `SetWorldObject`. The references taken on the way are released.
pub fn fn_00c6ba30(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    destination: Ptr<BhkNiCollisionObject>,
    process: Ptr,
) {
    e.call(BASE_COPY_MEMBERS, &args![this, destination, process]);
    let flags = e.get(this, BhkNiCollisionObject::m_uFlags);
    e.set(destination, BhkNiCollisionObject::m_uFlags, flags);
    let body = e.get(this, BhkNiCollisionObject::spWorldObject).addr();
    if body != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![body + 4]);
        let map = e.mem.u32(process.addr());
        // The out slot is the stack word that held the `destination` argument.
        let clone = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), destination.addr());
            if e.call(CLONE_MAP_GET_AT, &args![map, body, slot]).bool() {
                e.mem.u32(slot.addr())
            } else {
                e.vcall(body, 0x48, &args![process]).u32()
            }
        });
        e.with_stack(4, |e, held| {
            e.call(NI_POINTER_ASSIGN, &args![held, clone]);
            let held_object = e.mem.u32(held.addr());
            bhk_ni_collision_object_set_world_object(e, destination, Ptr::new(held_object));
            release_reference(e, held_object);
        });
        release_reference(e, body);
    }
}

// Translated from 00c6bb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::LinkObject` (Xbox PDB): runs the base `LinkObject` (`00a80ff0`); marks
/// the scene object's flags word (`+0x30`) with `(flags & ~0x10) | 6`; reads the next link id from
/// the stream (`00a63430`) and sets the world object to it.
pub fn bhk_ni_collision_object_link_object(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    stream: Ptr,
) {
    e.call(BASE_LINK_OBJECT, &args![this, stream]);
    let scene = e.get(this, BhkNiCollisionObject::m_pkSceneObject);
    if !scene.is_null() {
        let flags = e.mem.u32(scene.addr() + 0x30);
        e.mem
            .set_u32(scene.addr() + 0x30, (flags & 0xffff_ffef) | 6);
    }
    let body = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).ptr();
    bhk_ni_collision_object_set_world_object(e, this, body);
}

// Translated from 00c6bbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::SetTransform` (Xbox PDB): `transform` is an `NiTransform` (rotation
/// matrix at +0, translation at +0x24). Does nothing without a scene object. When `m_uFlags` bit 3
/// (`bSetLocal`) is set the transform is first taken into the parent's space: the parent's world
/// rotation (`+0x68`) is inverted (`004b4880`) and multiplied with it (`0062c250`), and the
/// resulting rotation and translation become the scene object's local transform (`+0x34` and
/// `+0x58`; the scale is not copied). Then the world transform (`+0x68`, `+0x8c`) is replaced
/// unless the byte at `01267b6c` is clear and both the rotation (within `01017d00`) and the
/// translation (within `01013ea4`) already match. The change flags are 3 when the byte is set,
/// otherwise 2 for a rotation change plus 1 for a translation change. With bit 2 (`bNotify`) set
/// and the hook at `01267b68` set, the hook is called with `(this, change_flags)`.
pub fn bhk_ni_collision_object_set_transform(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    transform: Ptr,
) {
    let scene = e.get(this, BhkNiCollisionObject::m_pkSceneObject);
    if scene.is_null() {
        return;
    }
    let scene = scene.addr();
    if (e.mem.u8(this.addr() + 0xc) >> 3) & 1 != 0 {
        let parent = e.mem.u32(scene + 0x18);
        e.with_stack(0x68, |e, area| {
            let local = area.addr();
            let product = area.addr() + 0x34;
            let source = if parent != 0 {
                e.call(TRANSFORM_INVERT_NON_UNIFORM, &args![parent + 0x68, local]);
                e.call(TRANSFORM_MULTIPLY, &args![local, product, transform])
                    .u32()
            } else {
                transform.addr()
            };
            copy_words(e, source, local, 0xd);
            copy_words(e, local, scene + 0x34, 9);
            copy_words(e, local + 0x24, scene + 0x58, 3);
        });
    }
    let mut change_flags = 0u32;
    if e.mem.u8(ALWAYS_REPORT_TRANSFORM) != 0 {
        change_flags = 3;
    } else {
        let epsilon = e.mem.f32(ROTATION_EPSILON);
        if !fn_00c6b5f0(e, Ptr::new(scene + 0x68), transform, epsilon) {
            change_flags = 2;
        }
        let epsilon_bits = x87_quiet_bits(e.mem.u32(TRANSLATION_EPSILON));
        if !e
            .call(
                ROW_NEARLY_EQUAL,
                &args![scene + 0x8c, transform.addr() + 0x24, epsilon_bits],
            )
            .bool()
        {
            change_flags |= 1;
        }
        if change_flags == 0 {
            return;
        }
    }
    copy_words(e, transform.addr(), scene + 0x68, 9);
    copy_words(e, transform.addr() + 0x24, scene + 0x8c, 3);
    let hook = e.mem.u32(TRANSFORM_CHANGE_HOOK);
    if hook != 0 && (e.mem.u8(this.addr() + 0xc) >> 2) & 1 != 0 {
        e.call(hook, &args![this, change_flags]);
    }
}

// Translated from 00c6bd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::ResetSim` (engine map name), cdecl `(node, recurse)`: nothing for a null
/// node. Sets bit 0x40 (`bReset`) of `m_uFlags` of the node's collision object (`[node + 0x1c]`)
/// when that object's virtual slot `+0x38` (`IsBhkNiCollisionObject`) answers non-null. When
/// `recurse` is set, repeats for every child of the node (the node's own virtual slot `+0xc`
/// answers `this` for an `NiNode`), last child first.
pub fn bhk_ni_collision_object_reset_sim(e: &mut Engine, node: Ptr, recurse: bool) {
    if node.is_null() {
        return;
    }
    let collision = e.mem.u32(node.addr() + 0x1c);
    if collision != 0 {
        let as_collision = e.vcall(collision, 0x38, &args![]).u32();
        if as_collision != 0 {
            let flags = e.mem.u16(as_collision + 0xc);
            e.mem.set_u16(as_collision + 0xc, flags | 0x40);
        }
    }
    if recurse {
        let as_node = e.vcall(node.addr(), 0xc, &args![]).u32();
        if as_node != 0 {
            let mut remaining = child_count(e, as_node);
            while remaining != 0 {
                let child = child_at(e, as_node, remaining - 1);
                remaining -= 1;
                bhk_ni_collision_object_reset_sim(e, Ptr::new(child), true);
            }
        }
    }
}

// Translated from 00c6bd60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6bd60 - C++ exception unwinding (SEH frame state) is not translated.
/// `bhkNiCollisionObject::SetDebugDisplay` (Xbox PDB): shows or hides the collision shapes as a
/// node named `bhkColDisp` below the scene object's node. `m_uFlags` bit 4 says that the display
/// is shown. Runs with the thread block's `+0x2b4` set to 0x10.
///
/// Hide (`enable` false, shown): the world object's virtual slot `+0xd0`
/// (`DestroyDisplayGeometry`, Xbox PDB) runs; then every child of the scene object's node (or of
/// its parent when the scene object is not a node) that has an object named `bhkColDisp`
/// (`NiAVObject::GetObjectByName`) detaches it (`NiNode` slot `+0xe4`, `DetachChild`). Bit 4 is
/// cleared.
///
/// Show (`enable` true, not shown, a world object whose Havok object's collision-filter layer, the
/// low 7 bits of the word at `+0x2c`, is not 0x11): creates an `NiNode`, names it `bhkColDisp`,
/// gives it the scale `|1 / scale|` of the node (`+0x98`), attaches it to the node (`NiNode` slot
/// `+0xdc`, `AttachChild`) and updates it; asks the world object to build its display geometry for
/// the new node (slot `+0xcc`, `CreateDisplayGeometry`); when geometry came back it is updated
/// (`UpdateProperties`, `Update`), otherwise the new node is detached again. Bit 4 is set.
///
/// When the scene object is null on a path that needs its parent, the exe reads `[0x18]` (and
/// faults); that read happens here too.
pub fn bhk_ni_collision_object_set_debug_display(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    enable: bool,
) {
    let (context, previous) = enter_context(e);
    let shown = (e.mem.u8(this.addr() + 0xc) >> 4) & 1 != 0;
    if !enable {
        if shown {
            hide_debug_display(e, this);
        }
    } else if !shown {
        show_debug_display(e, this);
    }
    e.mem.set_u32(context, previous);
}

/// The hide half of `SetDebugDisplay`.
fn hide_debug_display(e: &mut Engine, this: Ptr<BhkNiCollisionObject>) {
    let scene = e.get(this, BhkNiCollisionObject::m_pkSceneObject).addr();
    let mut node = if scene != 0 {
        e.vcall(scene, 0xc, &args![]).u32()
    } else {
        0
    };
    let body = e.get(this, BhkNiCollisionObject::spWorldObject);
    if !body.is_null() {
        e.vcall(body.addr(), 0xd0, &args![]);
    }
    if node == 0 {
        node = e.mem.u32(scene + 0x18);
    }
    if node != 0 {
        let count = child_count(e, node);
        for index in 0..count {
            let child = child_at(e, node, index);
            let mut found = 0;
            if child != 0 {
                let handle = e
                    .call(FIXED_STRING_CREATE, &args![DEBUG_DISPLAY_NAME])
                    .u32();
                found = e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), handle);
                    e.call(NI_AV_OBJECT_GET_OBJECT_BY_NAME, &args![child, slot])
                        .u32()
                });
                release_fixed_string(e, handle);
            }
            if found != 0 {
                let detached = e.with_stack(4, |e, out| {
                    e.vcall(node, 0xe4, &args![found, out]);
                    e.mem.u32(out.addr())
                });
                release_reference(e, detached);
            }
        }
    }
    let flags = e.get(this, BhkNiCollisionObject::m_uFlags);
    e.set(this, BhkNiCollisionObject::m_uFlags, flags & 0xffef);
}

/// The show half of `SetDebugDisplay`.
fn show_debug_display(e: &mut Engine, this: Ptr<BhkNiCollisionObject>) {
    let body = e.get(this, BhkNiCollisionObject::spWorldObject).addr();
    if body == 0 {
        return;
    }
    let havok_object = e.mem.u32(body + 8);
    let filter_word = if havok_object != 0 {
        e.mem.u32(havok_object + 0x2c)
    } else {
        0
    };
    if filter_word & 0x7f == 0x11 {
        return;
    }
    let scene = e.get(this, BhkNiCollisionObject::m_pkSceneObject).addr();
    let mut node = if scene != 0 {
        e.vcall(scene, 0xc, &args![]).u32()
    } else {
        0
    };
    if node == 0 {
        node = e.mem.u32(scene + 0x18);
    }
    let memory = e.call(OPERATOR_NEW, &args![0xacu32]).u32();
    let display = if memory != 0 {
        e.call(NI_NODE_CONSTRUCTOR, &args![memory, 0u32]).u32()
    } else {
        0
    };
    e.with_stack(4, |e, name| {
        e.call(FIXED_STRING_CONSTRUCTOR, &args![name, DEBUG_DISPLAY_NAME]);
        e.call(SET_NAME, &args![display, name]);
        let handle = e.mem.u32(name.addr());
        release_fixed_string(e, handle);
    });
    if node != 0 {
        let scale = e.mem.f32(node + 0x98);
        let inverse = if scale.is_nan() {
            x87_quiet(scale)
        } else {
            (1.0f64 / f64::from(scale)) as f32
        };
        e.mem.set_f32(display + 0x64, inverse.abs());
        e.vcall(node, 0xdc, &args![display, 1u32]);
        with_zero_update_data(e, |e, data| {
            e.call(NI_AV_OBJECT_UPDATE, &args![display, data]);
        });
    }
    let geometry = e.vcall(body, 0xcc, &args![display]).u32();
    let held = e.mem.alloc(4);
    e.mem.set_u32(held, geometry);
    if geometry != 0 {
        e.call(INTERLOCKED_INCREMENT, &args![geometry + 4]);
        if node != 0 {
            e.call(NI_AV_OBJECT_UPDATE_PROPERTIES, &args![geometry]);
            with_zero_update_data(e, |e, data| {
                e.call(NI_AV_OBJECT_UPDATE, &args![geometry, data]);
            });
        }
    } else if node != 0 && display != 0 {
        e.with_stack(4, |e, out| {
            e.vcall(node, 0xe4, &args![display, out]);
            e.call(NI_POINTER_RELEASE, &args![out]);
        });
    }
    let flags = e.get(this, BhkNiCollisionObject::m_uFlags);
    e.set(this, BhkNiCollisionObject::m_uFlags, flags | 0x10);
    e.call(NI_POINTER_RELEASE, &args![held]);
    e.mem.free(held);
}

// Translated from 00c6c0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl `(node, flag)`): for a node whose collision object
/// (`[node + 0x1c]`) answers non-null to its virtual slot `+0x38` (`IsBhkNiCollisionObject`) and
/// has a world object (`+0x10`), calls the world object's method `00c85d20(flag)`.
pub fn fn_00c6c0e0(e: &mut Engine, node: Ptr, flag: u32) {
    let collision = e.mem.u32(node.addr() + 0x1c);
    if collision != 0 {
        let as_collision = e.vcall(collision, 0x38, &args![]).u32();
        if as_collision != 0 {
            let body = e.mem.u32(as_collision + 0x10);
            if body != 0 {
                e.call(WORLD_OBJECT_SET_FLAG, &args![body, flag]);
            }
        }
    }
}

// Translated from 00c6c110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::SynchronizeRec` (engine map name), cdecl `(node, flag)`: nothing for a
/// null node. When the node's collision object is a `bhkNiCollisionObject` (its virtual slot
/// `+0x38` answers non-null) or the low byte of `flag` is set, prepares the node (`00a68bf0`),
/// runs the node's virtual slot `+0xb8` (`UpdateWorldData`) with a zeroed `NiUpdateData`, and sets
/// the low byte of `flag` to 1. Then calls itself on every child with the (possibly changed)
/// `flag`, first to last, and ends by running the node's virtual slot `+0xbc`
/// (`UpdateWorldBound`).
pub fn bhk_ni_collision_object_synchronize_rec(e: &mut Engine, node: Ptr, flag: u32) {
    if node.is_null() {
        return;
    }
    let mut flag = flag;
    let collision = e.mem.u32(node.addr() + 0x1c);
    let collision_object = collision != 0 && e.vcall(collision, 0x38, &args![]).u32() != 0;
    if collision_object || flag as u8 != 0 {
        e.call(NI_AV_OBJECT_PREPARE_UPDATE, &args![node]);
        with_zero_update_data(e, |e, data| {
            e.vcall(node.addr(), 0xb8, &args![data]);
        });
        flag = (flag & !0xff) | 1;
    }
    let as_node = e.vcall(node.addr(), 0xc, &args![]).u32();
    if as_node != 0 && child_count(e, as_node) != 0 {
        let mut index = 0;
        loop {
            let child = child_at(e, as_node, index);
            bhk_ni_collision_object_synchronize_rec(e, Ptr::new(child), flag);
            let count = child_count(e, as_node);
            index += 1;
            if index >= count {
                break;
            }
        }
    }
    e.vcall(node.addr(), 0xbc, &args![]);
}

// Translated from 00c6c1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::QKeyframe` (Xbox PDB): true without a world object. With one, takes the
/// Havok object at `[world object + 8]`, its embedded part at `+0x10`, and when the byte at `+0x18`
/// of that is 1 the owner it points to (`+0x10`, a signed byte offset from the part): the owner's
/// byte at `+0xe8` must then be 4 or 5. True when there is no such owner. (With a null Havok
/// object the exe reads address `0x18`, as this does.)
pub fn bhk_ni_collision_object_q_keyframe(e: &mut Engine, this: Ptr<BhkNiCollisionObject>) -> bool {
    let body = e.get(this, BhkNiCollisionObject::spWorldObject);
    if body.is_null() {
        return true;
    }
    let havok_object = e.mem.u32(body.addr() + 8);
    let part = if havok_object != 0 {
        havok_object + 0x10
    } else {
        0
    };
    let owner = if e.mem.u8(part + 0x18) == 1 {
        part.wrapping_add(e.mem.i8(part + 0x10) as u32)
    } else {
        0
    };
    if owner == 0 {
        return true;
    }
    let kind = e.mem.u8(owner + 0xe8);
    kind == 4 || kind == 5
}

// Translated from 00c6c210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::SetRigidBodyName` (engine map name), cdecl `(node)`: nothing for a null
/// node. When the node's collision object (`[node + 0x1c]`) answers non-null to its virtual slot
/// `+0x38` and has a world object, asks the accessor `00653290(01268 1c0)` for an object and, when
/// there is one, passes the node's name (`[node + 8]`) to the string assignment `00cb0c10` on the
/// field at `+0x74` of that object's `+8` part. Then repeats for every child of the node.
pub fn bhk_ni_collision_object_set_rigid_body_name(e: &mut Engine, node: Ptr) {
    if node.is_null() {
        return;
    }
    let collision = e.mem.u32(node.addr() + 0x1c);
    if collision != 0 {
        let as_collision = e.vcall(collision, 0x38, &args![]).u32();
        if as_collision != 0 && e.mem.u32(as_collision + 0x10) != 0 {
            // 00c6c237: `00653290` (RET 4) is a thiscall on the world object at +0x10
            // (ECX from 00c6c22b).
            let world_object = e.mem.u32(as_collision + 0x10);
            let accessor = e
                .call(
                    STATIC_ACCESSOR,
                    &args![world_object, RIGID_BODY_NAME_STATIC],
                )
                .u32();
            if accessor != 0 {
                let name = e.mem.u32(node.addr() + 8);
                let target = e.mem.u32(accessor + 8) + 0x74;
                e.call(STRING_PTR_ASSIGN, &args![target, name]);
            }
        }
    }
    let as_node = e.vcall(node.addr(), 0xc, &args![]).u32();
    if as_node != 0 {
        let count = child_count(e, as_node);
        for index in 0..count {
            let child = child_at(e, as_node, index);
            bhk_ni_collision_object_set_rigid_body_name(e, Ptr::new(child));
        }
    }
}

// Translated from 00c6c290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6c290 - C++ exception unwinding (SEH frame state) is not translated.
/// The `bhkNiCollisionObject` destructor body (the engine map has no name): sets the vtable, drops
/// the object count, lets the world object's virtual slot `+0xa0` (`Remove`, Xbox PDB) run, clears
/// the world object (`SetWorldObject(null)`), releases whatever `NiPointer` is left and runs the
/// base destructor body (`00a80fc0`).
pub fn fn_00c6c290(e: &mut Engine, this: Ptr<BhkNiCollisionObject>) {
    e.mem.set_u32(this.addr(), VTABLE);
    let count = e.mem.u32(OBJECT_COUNT);
    e.mem.set_u32(OBJECT_COUNT, count.wrapping_sub(1));
    let body = e.get(this, BhkNiCollisionObject::spWorldObject);
    if !body.is_null() {
        e.vcall(body.addr(), 0xa0, &args![]);
    }
    bhk_ni_collision_object_set_world_object(e, this, Ptr::NULL);
    let remaining = e.get(this, BhkNiCollisionObject::spWorldObject).addr();
    release_reference(e, remaining);
    e.call(BASE_DESTRUCTOR_BODY, &args![this]);
}

/// Moves the 16-bit `m_uFlags` through the stream's read or write function the way the
/// `NiStream` binary load and save are inlined: calls the function at `stream_io +
/// function_slot` (cdecl) with `(stream_io, this + 0xc, 2, &2, 1)` and adds its result to the byte
/// counter at `stream_io + 4`.
fn stream_flags(
    e: &mut Engine,
    stream_io: u32,
    function_slot: u32,
    this: Ptr<BhkNiCollisionObject>,
) {
    let function = e.mem.u32(stream_io + function_slot);
    let moved = e.with_stack(4, |e, size| {
        e.mem.set_u32(size.addr(), 2);
        e.call(
            function,
            &args![stream_io, this.addr() + 0xc, 2u32, size, 1u32],
        )
        .u32()
    });
    let total = e.mem.u32(stream_io + 4);
    e.mem.set_u32(stream_io + 4, total.wrapping_add(moved));
}

// Translated from 00c6c330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::LoadBinary` (Xbox PDB): runs the base `LoadBinary` (`00a80fd0`), then
/// reads the 16-bit `m_uFlags` through the stream's read function (the pointer at `+8` of the
/// structure at `stream + 0x24c`), adding the bytes read to the structure's counter at `+4`, and
/// finishes with the stream's `00a645b0`.
pub fn bhk_ni_collision_object_load_binary(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    stream: Ptr,
) {
    e.call(BASE_LOAD_BINARY, &args![this, stream]);
    let stream_io = e.mem.u32(stream.addr() + 0x24c);
    stream_flags(e, stream_io, 8, this);
    e.call(STREAM_FINISH_BINARY, &args![stream]);
}

// Translated from 00c6c380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::SaveBinary` (Xbox PDB): runs the base `SaveBinary` (`00a81040`), writes
/// the 16-bit `m_uFlags` through the stream's write function (the pointer at `+0xc` of the
/// structure at `stream + 0x250`), adding the bytes written to the counter at `+4`, then saves the
/// link to the world object with the stream's virtual slot `+0x30`.
pub fn bhk_ni_collision_object_save_binary(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    stream: Ptr,
) {
    e.call(BASE_SAVE_BINARY, &args![this, stream]);
    let stream_io = e.mem.u32(stream.addr() + 0x250);
    stream_flags(e, stream_io, 0xc, this);
    let body = e.get(this, BhkNiCollisionObject::spWorldObject);
    e.vcall(stream.addr(), 0x30, &args![body]);
}

// Translated from 00c6c3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::Synchronize` (engine map name), cdecl `(node, argument)`: runs
/// `SynchronizeRec(node, 0)` with the thread block's word at `+0x2c0` set to `argument`, then
/// restores it.
pub fn bhk_ni_collision_object_synchronize(e: &mut Engine, node: Ptr, argument: u32) {
    let word = e.tls() + THREAD_SYNCHRONIZE_ARGUMENT;
    let previous = e.mem.u32(word);
    e.mem.set_u32(word, argument);
    bhk_ni_collision_object_synchronize_rec(e, node, 0);
    e.mem.set_u32(word, previous);
}

// Translated from 00c6c410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body
/// (`00c6c290`), then the sized delete (`00aa1460`, `(this, size)`) of 0x14 bytes when bit 0 of
/// `flags` is set. Returns `this`.
pub fn bhk_ni_collision_object_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    flags: u32,
) -> Ptr<BhkNiCollisionObject> {
    fn_00c6c290(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x14u32]);
    }
    this
}

// Translated from 00c6c430 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c6c430 - C++ exception unwinding (SEH frame state) and the /GS stack cookie check (00ec408c) are not translated.
/// `bhkNiCollisionObject::AddViewerStrings` (Xbox PDB): with the thread block's `+0x2b4` set to
/// 0x10, appends to `strings` the base class strings (`00a81160`), the class name (the `NiRTTI`
/// pointer at `01267b70`), the world object's address formatted with `"0x%8X"` into a 0x20-byte
/// buffer and shown as `bhkWorldObject`, then true/false strings for `m_uFlags` bit 0
/// (`bActive`), bit 6 (`bReset`), bit 2 (`bNotify`) and bit 3 (`bSetLocal`), for the result of
/// virtual slot `+0xb4` (`QKeyframe`, shown as `bKeyframe`), and for whether the float at `+0x80`
/// of the scene object is negative (`bFaceDown`).
pub fn bhk_ni_collision_object_add_viewer_strings(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    strings: Ptr,
) {
    let (context, previous) = enter_context(e);
    let array = strings.addr();
    e.call(BASE_ADD_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.mem.u32(CLASS_RTTI);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    push_viewer_string(e, array, text);
    let body = e.get(this, BhkNiCollisionObject::spWorldObject);
    let text = e.with_stack(0x20, |e, buffer| {
        e.call(
            FORMAT_TEXT,
            &args![buffer, 0x20u32, FORMAT_WORLD_OBJECT, body],
        );
        e.call(NI_GET_VIEWER_STRING_TEXT, &args![NAME_WORLD_OBJECT, buffer])
            .u32()
    });
    push_viewer_string(e, array, text);
    let flags = e.mem.u8(this.addr() + 0xc);
    for (name, bit) in [
        (NAME_ACTIVE, 0),
        (NAME_RESET, 6),
        (NAME_NOTIFY, 2),
        (NAME_SET_LOCAL, 3),
    ] {
        let text = e
            .call(
                NI_GET_VIEWER_STRING_BOOL,
                &args![name, u32::from((flags >> bit) & 1)],
            )
            .u32();
        push_viewer_string(e, array, text);
    }
    let keyframe = e.vcall(this.addr(), 0xb4, &args![]).u8();
    let text = e
        .call(
            NI_GET_VIEWER_STRING_BOOL,
            &args![NAME_KEYFRAME, u32::from(keyframe)],
        )
        .u32();
    push_viewer_string(e, array, text);
    let scene = e.get(this, BhkNiCollisionObject::m_pkSceneObject).addr();
    let face_down = scene != 0 && e.mem.f32(scene + 0x80) < 0.0;
    let text = e
        .call(
            NI_GET_VIEWER_STRING_BOOL,
            &args![NAME_FACE_DOWN, u32::from(face_down)],
        )
        .u32();
    push_viewer_string(e, array, text);
    e.mem.set_u32(context, previous);
}

// Translated from 00c6b940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0126_7b70.
pub fn bhk_ni_collision_object_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0126_7b70
}

// Translated from 00c6b670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `bhkNiCollisionObject::UpdateWorldData` (Xbox PDB; the vtable `010c4314` lists this address at
/// slot `0x90`, where the Xbox PDB has `UpdateWorldData`; Ghidra never created the function and
/// holds it in the body of `00c65890`, ghidra_gaps.tsv). Thiscall `(this)` with one stack word that
/// is never read (`RET 4`). Reads the word at `+0x2c0` of the thread block (`e.tls()`; the one
/// `Synchronize` sets) and works on `m_uFlags`:
///
/// - Mode 3: skips straight to the final step.
/// - Bit 6 (`bReset`) set: unless the mode is 2, calls virtual slots `0xac` and `0xa8` (the Xbox
///   PDB lists `ClearVelocities` and `NitohkTransform`) and clears bit 6.
/// - Otherwise, bit 0 (`bActive`) set: asks slot `0xb4` (`QKeyframe`); when it answers true and
///   the mode (read again) is not 2, calls slot `0xa8`; when it answers false and the mode (read
///   again) is not 1, calls slot `0xa4` (`hktoNiTransform`).
/// - Anything that did not return above ends with `00a68bf0` (thiscall) on the scene object
///   (`m_pkSceneObject`) when it is not null.
pub fn bhk_ni_collision_object_update_world_data(
    e: &mut Engine,
    this: Ptr<BhkNiCollisionObject>,
    _unused_1: u32,
) {
    let thread_word = e.tls() + THREAD_SYNCHRONIZE_ARGUMENT;
    let mode = e.mem.u32(thread_word);
    if mode != 3 {
        let flags = e.get(this, BhkNiCollisionObject::m_uFlags);
        if flags as u8 & 0x40 != 0 {
            if mode != 2 {
                e.vcall(this.addr(), 0xac, &args![]);
                e.vcall(this.addr(), 0xa8, &args![]);
                let flags = e.get(this, BhkNiCollisionObject::m_uFlags);
                e.set(this, BhkNiCollisionObject::m_uFlags, flags & 0xffbf);
                return;
            }
        } else if e.get(this, BhkNiCollisionObject::m_uFlags) as u8 & 1 != 0 {
            if e.vcall(this.addr(), 0xb4, &args![]).bool() {
                if e.mem.u32(thread_word) != 2 {
                    e.vcall(this.addr(), 0xa8, &args![]);
                    return;
                }
            } else if e.mem.u32(thread_word) != 1 {
                e.vcall(this.addr(), 0xa4, &args![]);
                return;
            }
        }
    }
    let scene_object = e.get(this, BhkNiCollisionObject::m_pkSceneObject);
    if !scene_object.is_null() {
        e.call(NI_AV_OBJECT_PREPARE_UPDATE, &args![scene_object]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c6b940, bhk_ni_collision_object_get_r_t_t_i() -> u32),
        entry!(
            0x00c6_b5e0,
            bhk_ni_collision_object_set_scene_graph_object(Ptr, Ptr)
        ),
        entry!(0x00c6b5f0, fn_00c6b5f0(Ptr, Ptr, f32) -> bool),
        entry!(
            0x00c6b730,
            bhk_ni_collision_object_register_streamables(Ptr<BhkNiCollisionObject>, Ptr) -> bool
        ),
        entry!(
            0x00c6b760,
            bhk_ni_collision_object_is_equal(
                Ptr<BhkNiCollisionObject>,
                Ptr<BhkNiCollisionObject>,
            ) -> bool
        ),
        entry!(0x00c6b7b0, bhk_ni_collision_object_get_face_up(Ptr, u32) -> bool),
        entry!(0x00c6b860, fn_00c6b860(Ptr, u32) -> bool),
        entry!(
            0x00c6b910,
            fn_00c6b910(Ptr<BhkNiCollisionObject>) -> Ptr<BhkNiCollisionObject>
        ),
        entry!(
            0x00c6b950,
            fn_00c6b950(Ptr<BhkNiCollisionObject>, Ptr) -> Ptr<BhkNiCollisionObject>
        ),
        entry!(
            0x00c6b980,
            bhk_ni_collision_object_set_world_object(Ptr<BhkNiCollisionObject>, Ptr)
        ),
        entry!(
            0x00c6ba30,
            fn_00c6ba30(Ptr<BhkNiCollisionObject>, Ptr<BhkNiCollisionObject>, Ptr)
        ),
        entry!(
            0x00c6bb70,
            bhk_ni_collision_object_link_object(Ptr<BhkNiCollisionObject>, Ptr)
        ),
        entry!(
            0x00c6bbb0,
            bhk_ni_collision_object_set_transform(Ptr<BhkNiCollisionObject>, Ptr)
        ),
        entry!(0x00c6bd00, bhk_ni_collision_object_reset_sim(Ptr, bool)),
        entry!(
            0x00c6bd60,
            bhk_ni_collision_object_set_debug_display(Ptr<BhkNiCollisionObject>, bool)
        ),
        entry!(0x00c6c0e0, fn_00c6c0e0(Ptr, u32)),
        entry!(
            0x00c6c110,
            bhk_ni_collision_object_synchronize_rec(Ptr, u32)
        ),
        entry!(
            0x00c6c1d0,
            bhk_ni_collision_object_q_keyframe(Ptr<BhkNiCollisionObject>) -> bool
        ),
        entry!(0x00c6c210, bhk_ni_collision_object_set_rigid_body_name(Ptr)),
        entry!(0x00c6c290, fn_00c6c290(Ptr<BhkNiCollisionObject>)),
        entry!(
            0x00c6c330,
            bhk_ni_collision_object_load_binary(Ptr<BhkNiCollisionObject>, Ptr)
        ),
        entry!(
            0x00c6c380,
            bhk_ni_collision_object_save_binary(Ptr<BhkNiCollisionObject>, Ptr)
        ),
        entry!(0x00c6c3d0, bhk_ni_collision_object_synchronize(Ptr, u32)),
        entry!(
            0x00c6c410,
            bhk_ni_collision_object_scalar_deleting_destructor(
                Ptr<BhkNiCollisionObject>,
                u32,
            )
                -> Ptr<BhkNiCollisionObject>
        ),
        entry!(
            0x00c6c430,
            bhk_ni_collision_object_add_viewer_strings(Ptr<BhkNiCollisionObject>, Ptr)
        ),
        entry!(
            0x00c6_b670,
            bhk_ni_collision_object_update_world_data(Ptr<BhkNiCollisionObject>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
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

    /// An engine with this unit's globals mapped, the interlocked imports working on memory and
    /// the call log on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(OBJECT_COUNT, 0x20);
        e.map(EMPTY_FIXED_STRING, 4);
        e.map(ROTATION_EPSILON, 4);
        e.map(TRANSLATION_EPSILON, 4);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            ret(value)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            ret(value)
        });
        e.call_log = Some(vec![]);
        e
    }

    /// An object of `size` bytes whose vtable (a zeroed block) holds the `(slot offset, target)` pairs.
    fn object_with_slots(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let table = e.mem.alloc(0x100);
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, table);
        object
    }

    fn collision_object(e: &mut Engine) -> Ptr<BhkNiCollisionObject> {
        e.new_object::<BhkNiCollisionObject>()
    }

    const DELETE: u32 = 0x7000_0004;

    #[test]
    fn matrix_compare_passes_every_row_or_stops_at_the_first_failure() {
        let mut e = engine();
        let a = e.mem.alloc(0x24);
        let b = e.mem.alloc(0x24);
        for word in 0..9 {
            e.mem.set_u32(a + 4 * word, 0x3f80_0000 + word);
            e.mem.set_u32(b + 4 * word, 0x4000_0000 + word);
        }
        e.mem.set_u32(a + 12, 0x7f80_0001); // a signalling NaN, row 1 lane 0
        let rows = Rc::new(RefCell::new(Vec::new()));
        let seen = rows.clone();
        e.register_double(ROW_NEARLY_EQUAL, move |e, args| {
            let mut words = Vec::new();
            for lane in 0..3 {
                words.push(e.mem.u32(args[0] + 4 * lane));
                words.push(e.mem.u32(args[1] + 4 * lane));
            }
            words.push(args[2]);
            seen.borrow_mut().push(words);
            ret(u32::from(seen.borrow().len() < 2))
        });
        // The second row fails: two calls only.
        let result = e.call(0x00c6b5f0, &args![a, b, f32::from_bits(0x7f80_0002)]);
        assert!(!result.bool());
        let rows = rows.borrow();
        assert_eq!(rows.len(), 2);
        assert_eq!(
            rows[0],
            vec![
                0x3f80_0000,
                0x4000_0000,
                0x3f80_0001,
                0x4000_0001,
                0x3f80_0002,
                0x4000_0002,
                0x7fc0_0002
            ]
        );
        // The signalling NaN is quieted by the float copy.
        assert_eq!(rows[1][0], 0x7fc0_0001);
    }

    #[test]
    fn matrix_compare_is_true_when_all_three_rows_match() {
        let mut e = engine();
        let a = e.mem.alloc(0x24);
        let b = e.mem.alloc(0x24);
        e.register(ROW_NEARLY_EQUAL, |_, _| ret(1));
        assert!(e.call(0x00c6b5f0, &args![a, b, 0.5f32]).bool());
        assert_eq!(calls_to(&e, ROW_NEARLY_EQUAL).len(), 3);
    }

    #[test]
    fn register_streamables_registers_the_world_object_first() {
        let mut e = engine();
        e.register(0x7000_0054, |_, _| ret(0));
        e.register(BASE_REGISTER_STREAMABLES, |_, _| ret(1));
        let body = object_with_slots(&mut e, 0x20, &[(0x54, 0x7000_0054)]);
        let this = collision_object(&mut e);
        e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(body));
        assert!(e.call(0x00c6b730, &args![this, 0x1234u32]).bool());
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(order, [0x00c6b730, 0x7000_0054, BASE_REGISTER_STREAMABLES]);
        assert_eq!(calls_to(&e, 0x7000_0054), vec![vec![body, 0x1234]]);
        assert_eq!(
            calls_to(&e, BASE_REGISTER_STREAMABLES),
            vec![vec![this.addr(), 0x1234]]
        );

        // Without a world object only the base runs, and its answer is the result.
        let mut e = engine();
        e.register(BASE_REGISTER_STREAMABLES, |_, _| ret(0));
        let this = collision_object(&mut e);
        assert!(!e.call(0x00c6b730, &args![this, 0x1234u32]).bool());
        assert_eq!(calls_to(&e, BASE_REGISTER_STREAMABLES).len(), 1);
    }

    #[test]
    fn is_equal_needs_the_base_the_world_objects_and_the_flags() {
        let mut e = engine();
        e.register(BASE_IS_EQUAL, |_, _| ret(0));
        let this = collision_object(&mut e);
        let other = collision_object(&mut e);
        assert!(!e.call(0x00c6b760, &args![this, other]).bool());

        // Both without world objects: only the flags matter.
        e.register(BASE_IS_EQUAL, |_, _| ret(1));
        e.set(this, BhkNiCollisionObject::m_uFlags, 0x41);
        e.set(other, BhkNiCollisionObject::m_uFlags, 0x41);
        assert!(e.call(0x00c6b760, &args![this, other]).bool());
        e.set(other, BhkNiCollisionObject::m_uFlags, 0x45);
        assert!(!e.call(0x00c6b760, &args![this, other]).bool());
        // The other has a world object while this has none.
        e.set(other, BhkNiCollisionObject::m_uFlags, 0x41);
        e.set(other, BhkNiCollisionObject::spWorldObject, Ptr::new(0x40));
        assert!(!e.call(0x00c6b760, &args![this, other]).bool());

        // With a world object the slot at +0x5c decides, together with the flags.
        e.register(0x7000_005c, |_, a| ret(u32::from(a[1] == 0x40)));
        let body = object_with_slots(&mut e, 0x20, &[(0x5c, 0x7000_005c)]);
        e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(body));
        assert!(e.call(0x00c6b760, &args![this, other]).bool());
        e.set(other, BhkNiCollisionObject::m_uFlags, 0x43);
        assert!(!e.call(0x00c6b760, &args![this, other]).bool());
        e.set(other, BhkNiCollisionObject::m_uFlags, 0x41);
        e.set(other, BhkNiCollisionObject::spWorldObject, Ptr::new(0x50));
        assert!(!e.call(0x00c6b760, &args![this, other]).bool());
    }

    /// A mapped fixed-string block: the reference count at +0, the handle at +8.
    const STRING_BLOCK: u32 = 0x7800_0000;
    const STRING_HANDLE: u32 = STRING_BLOCK + 8;

    #[test]
    fn face_up_reads_the_float_of_the_node_or_of_the_named_object() {
        let mut e = engine();
        e.map(STRING_BLOCK, 0x20);
        e.mem.set_u32(STRING_BLOCK, 5);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x9000);
        e.register(FIXED_STRING_CREATE, |_, _| ret(STRING_HANDLE));
        // The lookup answers the word at +0x24 of the node.
        e.register(0x7000_009c, |e, a| {
            assert_eq!(e.mem.u32(a[1]), STRING_HANDLE);
            ret(e.mem.u32(a[0] + 0x24))
        });
        let node = object_with_slots(&mut e, 0x100, &[(0x9c, 0x7000_009c)]);
        let named = e.mem.alloc(0x100);
        e.mem.set_f32(named + 0x84, 2.0);
        e.mem.set_f32(node + 0x84, -1.0);
        // No name: the node's own value, and no string is made.
        assert!(!e.call(0x00c6b7b0, &args![node, 0u32]).bool());
        assert!(calls_to(&e, FIXED_STRING_CREATE).is_empty());
        e.mem.set_f32(node + 0x84, 1.0);
        assert!(e.call(0x00c6b7b0, &args![node, 0u32]).bool());
        // A name that is not found: the node itself.
        e.mem.set_f32(node + 0x84, -1.0);
        assert!(!e.call(0x00c6b7b0, &args![node, 0x5000u32]).bool());
        // A name that is found: the found object's value.
        e.mem.set_u32(node + 0x24, named);
        assert!(e.call(0x00c6b7b0, &args![node, 0x5000u32]).bool());
        // NaN, and a null node.
        e.mem.set_u32(named + 0x84, 0x7fc0_0000);
        assert!(!e.call(0x00c6b7b0, &args![node, 0x5000u32]).bool());
        assert!(!e.call(0x00c6b7b0, &args![0u32, 0x5000u32]).bool());
        // Each of the three lookups released its handle.
        assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT).len(), 3);
        assert_eq!(e.mem.u32(STRING_BLOCK), 2);
    }

    #[test]
    fn the_second_lookup_reads_offset_0x88() {
        let mut e = engine();
        let node = e.mem.alloc(0x100);
        e.mem.set_f32(node + 0x84, -3.0);
        e.mem.set_f32(node + 0x88, 3.0);
        assert!(e.call(0x00c6b860, &args![node, 0u32]).bool());
        assert!(!e.call(0x00c6b7b0, &args![node, 0u32]).bool());
        e.mem.set_f32(node + 0x88, 0.0);
        assert!(!e.call(0x00c6b860, &args![node, 0u32]).bool());
        assert!(!e.call(0x00c6b860, &args![0u32, 0u32]).bool());
    }

    #[test]
    fn both_constructors_set_vtable_flags_and_count() {
        let mut e = engine();
        e.register(BASE_CONSTRUCTOR, |_, _| ret(0));
        e.register(BASE_CONSTRUCTOR_WITH_SCENE_OBJECT, |_, _| ret(0));
        e.mem.set_u32(OBJECT_COUNT, 5);
        let this = collision_object(&mut e);
        e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(0x77));
        assert_eq!(e.call(0x00c6b910, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert!(e.get(this, BhkNiCollisionObject::spWorldObject).is_null());
        assert_eq!(e.get(this, BhkNiCollisionObject::m_uFlags), 0x41);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 6);
        assert_eq!(calls_to(&e, BASE_CONSTRUCTOR), vec![vec![this.addr()]]);

        let other = collision_object(&mut e);
        assert_eq!(
            e.call(0x00c6b950, &args![other, 0x1234u32]).u32(),
            other.addr()
        );
        assert_eq!(e.mem.u32(OBJECT_COUNT), 7);
        assert_eq!(e.get(other, BhkNiCollisionObject::m_uFlags), 0x41);
        assert_eq!(
            calls_to(&e, BASE_CONSTRUCTOR_WITH_SCENE_OBJECT),
            vec![vec![other.addr(), 0x1234]]
        );
    }

    #[test]
    fn set_world_object_moves_the_property_and_the_references() {
        let mut e = engine();
        e.register(EMPTY_LOCK, |_, _| ret(0));
        e.register(REMOVE_PROPERTY, |_, _| ret(0));
        e.register(ADD_PROPERTY, |_, _| ret(0));
        e.register(DELETE, |_, _| ret(0));
        let old_havok = e.mem.alloc(0x20);
        let new_havok = e.mem.alloc(0x20);
        let old = object_with_slots(&mut e, 0x20, &[(4, DELETE)]);
        let new = object_with_slots(&mut e, 0x20, &[(4, DELETE)]);
        e.mem.set_u32(old + 4, 1);
        e.mem.set_u32(old + 8, old_havok);
        e.mem.set_u32(new + 8, new_havok);
        let this = collision_object(&mut e);
        e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(old));

        // The same object again: nothing happens.
        e.call(0x00c6b980, &args![this, old]);
        assert!(calls_to(&e, EMPTY_LOCK).is_empty());

        e.call(0x00c6b980, &args![this, new]);
        let removed = calls_to(&e, REMOVE_PROPERTY);
        assert_eq!(removed.len(), 1);
        assert_eq!((removed[0][0], removed[0][2]), (old_havok, CLASS_RTTI));
        assert_eq!(calls_to(&e, DELETE), vec![vec![old]]);
        assert_eq!(e.get(this, BhkNiCollisionObject::spWorldObject).addr(), new);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(
            calls_to(&e, ADD_PROPERTY),
            vec![vec![new_havok, CLASS_RTTI, this.addr(), 0]]
        );
        let locks: Vec<u32> = calls_to(&e, EMPTY_LOCK).iter().map(|c| c[0]).collect();
        assert_eq!(locks, [old, old, new, new]);

        // Clearing: the property is removed, nothing is added.
        e.call(0x00c6b980, &args![this, 0u32]);
        assert_eq!(calls_to(&e, REMOVE_PROPERTY).len(), 2);
        assert_eq!(calls_to(&e, ADD_PROPERTY).len(), 1);
        assert!(e.get(this, BhkNiCollisionObject::spWorldObject).is_null());
        assert_eq!(e.mem.u32(new + 4), 0);
        assert_eq!(calls_to(&e, DELETE).len(), 2);
    }

    #[test]
    fn copy_members_uses_the_existing_clone_or_makes_one() {
        for found in [true, false] {
            let mut e = engine();
            e.register(BASE_COPY_MEMBERS, |_, _| ret(0));
            e.register(EMPTY_LOCK, |_, _| ret(0));
            e.register(REMOVE_PROPERTY, |_, _| ret(0));
            e.register(ADD_PROPERTY, |_, _| ret(0));
            e.register(DELETE, |_, _| ret(0));
            e.register(0x7000_0048, |e, _| ret(e.mem.u32(0x7777_0000)));
            e.map(0x7777_0000, 4);
            let existing = e.mem.alloc(0x20);
            let created = e.mem.alloc(0x20);
            e.mem.set_u32(0x7777_0000, created);
            e.register_double(CLONE_MAP_GET_AT, move |e, a| {
                if found {
                    e.mem.set_u32(a[2], existing);
                }
                ret(u32::from(found))
            });
            e.register(NI_POINTER_ASSIGN, |e, a| {
                e.mem.set_u32(a[0], a[1]);
                e.call(INTERLOCKED_INCREMENT, &args![a[1] + 4]);
                ret(a[0])
            });
            let body = object_with_slots(&mut e, 0x20, &[(4, DELETE), (0x48, 0x7000_0048)]);
            e.mem.set_u32(body + 4, 1);
            let this = collision_object(&mut e);
            e.set(this, BhkNiCollisionObject::m_uFlags, 0x55);
            e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(body));
            let destination = collision_object(&mut e);
            let map = 0x5151_0000u32;
            let process = e.mem.alloc(8);
            e.mem.set_u32(process, map);
            e.call(0x00c6ba30, &args![this, destination, process]);
            let clone = if found { existing } else { created };
            assert_eq!(
                e.get(destination, BhkNiCollisionObject::spWorldObject)
                    .addr(),
                clone
            );
            assert_eq!(e.get(destination, BhkNiCollisionObject::m_uFlags), 0x55);
            assert_eq!(e.mem.u32(clone + 4), 1);
            assert_eq!(e.mem.u32(body + 4), 1);
            assert_eq!(calls_to(&e, 0x7000_0048).len(), usize::from(!found));
            assert_eq!(
                calls_to(&e, BASE_COPY_MEMBERS),
                vec![vec![this.addr(), destination.addr(), process]]
            );
            let lookup = calls_to(&e, CLONE_MAP_GET_AT);
            assert_eq!((lookup[0][0], lookup[0][1]), (map, body));
        }
    }

    #[test]
    fn copy_members_without_a_world_object_only_copies_the_flags() {
        let mut e = engine();
        e.register(BASE_COPY_MEMBERS, |_, _| ret(0));
        let this = collision_object(&mut e);
        e.set(this, BhkNiCollisionObject::m_uFlags, 0x61);
        let destination = collision_object(&mut e);
        e.call(0x00c6ba30, &args![this, destination, 0x1000u32]);
        assert_eq!(e.get(destination, BhkNiCollisionObject::m_uFlags), 0x61);
        assert!(calls_to(&e, CLONE_MAP_GET_AT).is_empty());
    }

    #[test]
    fn link_object_marks_the_scene_object_and_takes_the_linked_world_object() {
        let mut e = engine();
        e.register(BASE_LINK_OBJECT, |_, _| ret(0));
        let linked = e.mem.alloc(0x20);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| ret(linked));
        let scene = e.mem.alloc(0x40);
        e.mem.set_u32(scene + 0x30, 0xffff_ffff);
        let this = collision_object(&mut e);
        e.set(this, BhkNiCollisionObject::m_pkSceneObject, Ptr::new(scene));
        e.call(0x00c6bb70, &args![this, 0x2000u32]);
        assert_eq!(e.mem.u32(scene + 0x30), 0xffff_ffef | 6);
        assert_eq!(
            e.get(this, BhkNiCollisionObject::spWorldObject).addr(),
            linked
        );
        assert_eq!(e.mem.u32(linked + 4), 1);
        assert_eq!(
            calls_to(&e, BASE_LINK_OBJECT),
            vec![vec![this.addr(), 0x2000]]
        );
        assert_eq!(
            calls_to(&e, STREAM_GET_OBJECT_FROM_LINK_ID),
            vec![vec![0x2000]]
        );
    }

    /// A scene object and collision object for the `SetTransform` tests, with the transform given.
    fn transform_fixture(e: &mut Engine, flags: u16) -> (Ptr<BhkNiCollisionObject>, u32, u32) {
        let scene = e.mem.alloc(0xb0);
        let this = collision_object(e);
        e.set(this, BhkNiCollisionObject::m_pkSceneObject, Ptr::new(scene));
        e.set(this, BhkNiCollisionObject::m_uFlags, flags);
        let transform = e.mem.alloc(0x34);
        for word in 0..13 {
            e.mem.set_u32(transform + 4 * word, 0x100 + word);
        }
        (this, scene, transform)
    }

    #[test]
    fn set_transform_without_a_scene_object_does_nothing() {
        let mut e = engine();
        let this = collision_object(&mut e);
        e.call(0x00c6bbb0, &args![this, 0x3000u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn set_transform_copies_and_reports_when_the_always_byte_is_set() {
        let mut e = engine();
        e.mem.set_u8(ALWAYS_REPORT_TRANSFORM, 1);
        e.register(0x7000_1111, |_, _| ret(0));
        e.mem.set_u32(TRANSFORM_CHANGE_HOOK, 0x7000_1111);
        let (this, scene, transform) = transform_fixture(&mut e, 0x04);
        e.call(0x00c6bbb0, &args![this, transform]);
        for word in 0..9 {
            assert_eq!(e.mem.u32(scene + 0x68 + 4 * word), 0x100 + word);
        }
        for word in 0..3 {
            assert_eq!(e.mem.u32(scene + 0x8c + 4 * word), 0x109 + word);
        }
        // No comparisons were made.
        assert!(calls_to(&e, ROW_NEARLY_EQUAL).is_empty());
        assert_eq!(calls_to(&e, 0x7000_1111), vec![vec![this.addr(), 3]]);

        // Without flag bit 2 the hook is not called.
        let (other, _, transform) = transform_fixture(&mut e, 0x00);
        e.call(0x00c6bbb0, &args![other, transform]);
        assert_eq!(calls_to(&e, 0x7000_1111).len(), 1);
    }

    #[test]
    fn set_transform_reports_which_part_changed() {
        // (rotation matches, translation matches) -> (expected change flags, or none)
        for (rotation_matches, translation_matches, expected) in [
            (true, true, None),
            (false, true, Some(2)),
            (true, false, Some(1)),
            (false, false, Some(3)),
        ] {
            let mut e = engine();
            e.mem.set_u32(ROTATION_EPSILON, 0x3a83_126f);
            e.mem.set_u32(TRANSLATION_EPSILON, 0x3c23_d70a);
            e.register(0x7000_2222, |_, _| ret(0));
            e.mem.set_u32(TRANSFORM_CHANGE_HOOK, 0x7000_2222);
            let (this, scene, transform) = transform_fixture(&mut e, 0x04);
            // The comparison double answers by the tolerance it is given.
            e.register_double(ROW_NEARLY_EQUAL, move |_, a| {
                let rotation_row = a[2] == 0x3a83_126f;
                ret(u32::from(if rotation_row {
                    rotation_matches
                } else {
                    translation_matches
                }))
            });
            e.call(0x00c6bbb0, &args![this, transform]);
            match expected {
                None => {
                    assert!(calls_to(&e, 0x7000_2222).is_empty());
                    assert_eq!(e.mem.u32(scene + 0x68), 0);
                }
                Some(flags) => {
                    assert_eq!(calls_to(&e, 0x7000_2222), vec![vec![this.addr(), flags]]);
                    assert_eq!(e.mem.u32(scene + 0x68), 0x100);
                    assert_eq!(e.mem.u32(scene + 0x8c), 0x109);
                }
            }
            // The translation comparison is given the world translation and the new one.
            let last = calls_to(&e, ROW_NEARLY_EQUAL).pop().unwrap();
            assert_eq!((last[0], last[1]), (scene + 0x8c, transform + 0x24));
        }
    }

    #[test]
    fn set_transform_with_bit_3_sets_the_local_transform_in_the_parent_space() {
        let mut e = engine();
        e.mem.set_u8(ALWAYS_REPORT_TRANSFORM, 1);
        let (this, scene, transform) = transform_fixture(&mut e, 0x08);
        let parent = e.mem.alloc(0xb0);
        e.mem.set_u32(scene + 0x18, parent);
        let inverted = Rc::new(RefCell::new(0));
        let seen = inverted.clone();
        e.register_double(TRANSFORM_INVERT_NON_UNIFORM, move |_, a| {
            *seen.borrow_mut() = a[0];
            ret(0)
        });
        e.register(TRANSFORM_MULTIPLY, |e, a| {
            for word in 0..13 {
                e.mem.set_u32(a[1] + 4 * word, 0x900 + word);
            }
            ret(a[1])
        });
        e.call(0x00c6bbb0, &args![this, transform]);
        assert_eq!(*inverted.borrow(), parent + 0x68);
        // The local rotation and translation are the product's; scale is not copied.
        for word in 0..9 {
            assert_eq!(e.mem.u32(scene + 0x34 + 4 * word), 0x900 + word);
        }
        for word in 0..3 {
            assert_eq!(e.mem.u32(scene + 0x58 + 4 * word), 0x909 + word);
        }
        assert_eq!(e.mem.u32(scene + 0x64), 0);
        let multiply = calls_to(&e, TRANSFORM_MULTIPLY);
        assert_eq!(multiply[0][2], transform);
        // The world transform still takes the transform given.
        assert_eq!(e.mem.u32(scene + 0x68), 0x100);

        // Without a parent the transform itself is copied.
        let (other, scene, transform) = transform_fixture(&mut e, 0x08);
        e.call(0x00c6bbb0, &args![other, transform]);
        assert_eq!(e.mem.u32(scene + 0x34), 0x100);
        assert_eq!(e.mem.u32(scene + 0x58), 0x109);
        assert_eq!(calls_to(&e, TRANSFORM_MULTIPLY).len(), 1);
    }

    #[test]
    fn reset_sim_marks_collision_objects_and_walks_the_children_last_first() {
        let mut e = engine();
        e.register(0x7000_0038, |_, a| ret(a[0]));
        e.register(0x7000_000c, |_, a| ret(a[0]));
        let collision = object_with_slots(&mut e, 0x20, &[(0x38, 0x7000_0038)]);
        // A node with two children, each with its own collision object.
        let first = object_with_slots(&mut e, 0xc0, &[(0xc, 0x7000_000c)]);
        let second = object_with_slots(&mut e, 0xc0, &[(0xc, 0x7000_000c)]);
        let first_collision = object_with_slots(&mut e, 0x20, &[(0x38, 0x7000_0038)]);
        let second_collision = object_with_slots(&mut e, 0x20, &[(0x38, 0x7000_0038)]);
        e.mem.set_u32(first + 0x1c, first_collision);
        e.mem.set_u32(second + 0x1c, second_collision);
        let parent = object_with_slots(&mut e, 0xc0, &[(0xc, 0x7000_000c)]);
        e.mem.set_u32(parent + 0x1c, collision);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, first);
        e.mem.set_u32(array + 4, second);
        e.mem.set_u32(parent + 0xa0, array);
        e.mem.set_u16(parent + 0xa6, 2);

        e.call(0x00c6bd00, &args![parent, false]);
        assert_eq!(e.mem.u16(collision + 0xc), 0x40);
        assert_eq!(e.mem.u16(first_collision + 0xc), 0);
        assert_eq!(calls_to(&e, 0x7000_000c).len(), 0);

        e.call(0x00c6bd00, &args![parent, true]);
        assert_eq!(e.mem.u16(first_collision + 0xc), 0x40);
        assert_eq!(e.mem.u16(second_collision + 0xc), 0x40);
        let walked: Vec<u32> = calls_to(&e, 0x7000_000c).iter().map(|c| c[0]).collect();
        assert_eq!(walked, [parent, second, first]);
        // A null node is skipped.
        e.call(0x00c6bd00, &args![0u32, true]);
    }

    #[test]
    fn set_debug_display_does_nothing_when_the_state_already_matches() {
        let mut e = engine();
        let context = e.tls() + THREAD_CONTEXT;
        e.mem.set_u32(context, 0x33);
        let this = collision_object(&mut e);
        // Hide while not shown, and show while shown.
        e.call(0x00c6bd60, &args![this, false]);
        e.set(this, BhkNiCollisionObject::m_uFlags, 0x10);
        e.call(0x00c6bd60, &args![this, true]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
        assert_eq!(e.get(this, BhkNiCollisionObject::m_uFlags), 0x10);
        assert_eq!(e.mem.u32(context), 0x33);
    }

    #[test]
    fn set_debug_display_shows_a_scaled_node_with_the_display_geometry() {
        for geometry_made in [true, false] {
            let mut e = engine();
            let context = e.tls() + THREAD_CONTEXT;
            e.mem.set_u32(context, 0x33);
            e.mem.set_u32(EMPTY_FIXED_STRING, 0x9000);
            // Slots: node +0xc answers itself; +0xdc attach; +0xe4 detach. World object +0xcc.
            e.register(0x7000_000c, |_, a| ret(a[0]));
            e.register(0x7000_00dc, |_, _| ret(0));
            e.register(0x7000_00e4, |_, _| ret(0));
            let geometry = e.mem.alloc(0x20);
            e.register_double(0x7000_00cc, move |_, _| {
                ret(if geometry_made { geometry } else { 0 })
            });
            e.map(STRING_BLOCK, 0x20);
            e.mem.set_u32(STRING_BLOCK, 5);
            let display = e.mem.alloc(0xb0);
            e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
            e.register_double(NI_NODE_CONSTRUCTOR, move |_, _| ret(display));
            e.register(FIXED_STRING_CONSTRUCTOR, |e, a| {
                e.mem.set_u32(a[0], STRING_HANDLE);
                ret(a[0])
            });
            e.register(SET_NAME, |_, _| ret(0));
            e.register(NI_AV_OBJECT_UPDATE, |_, _| ret(0));
            e.register(NI_AV_OBJECT_UPDATE_PROPERTIES, |_, _| ret(0));
            e.register(NI_POINTER_RELEASE, |_, _| ret(0));
            let node = object_with_slots(
                &mut e,
                0xc0,
                &[(0xc, 0x7000_000c), (0xdc, 0x7000_00dc), (0xe4, 0x7000_00e4)],
            );
            e.mem.set_f32(node + 0x98, -4.0);
            let havok = e.mem.alloc(0x40);
            e.mem.set_u32(havok + 0x2c, 0x1_0000 | 0x05);
            let body = object_with_slots(&mut e, 0x20, &[(0xcc, 0x7000_00cc)]);
            e.mem.set_u32(body + 8, havok);
            let this = collision_object(&mut e);
            e.set(this, BhkNiCollisionObject::m_pkSceneObject, Ptr::new(node));
            e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(body));

            e.call(0x00c6bd60, &args![this, true]);

            assert_eq!(e.get(this, BhkNiCollisionObject::m_uFlags), 0x10);
            assert_eq!(e.mem.u32(context), 0x33);
            // The scale is |1 / -4|.
            assert_eq!(e.mem.f32(display + 0x64), 0.25);
            assert_eq!(calls_to(&e, 0x7000_00dc), vec![vec![node, display, 1]]);
            assert_eq!(calls_to(&e, 0x7000_00cc), vec![vec![body, display]]);
            assert_eq!(calls_to(&e, SET_NAME).len(), 1);
            assert_eq!(calls_to(&e, NI_AV_OBJECT_UPDATE)[0][0], display);
            if geometry_made {
                assert_eq!(
                    calls_to(&e, NI_AV_OBJECT_UPDATE_PROPERTIES),
                    vec![vec![geometry]]
                );
                assert_eq!(calls_to(&e, NI_AV_OBJECT_UPDATE).len(), 2);
                assert_eq!(e.mem.u32(geometry + 4), 1);
                assert!(calls_to(&e, 0x7000_00e4).is_empty());
            } else {
                assert!(calls_to(&e, NI_AV_OBJECT_UPDATE_PROPERTIES).is_empty());
                assert_eq!(calls_to(&e, NI_AV_OBJECT_UPDATE).len(), 1);
                assert_eq!(calls_to(&e, 0x7000_00e4)[0][..2], [node, display]);
            }
            // The name handle is released through the handle - 8 convention.
            assert_eq!(calls_to(&e, INTERLOCKED_DECREMENT)[0], vec![STRING_BLOCK]);
        }
    }

    #[test]
    fn set_debug_display_does_not_show_for_a_layer_17_or_without_a_world_object() {
        let mut e = engine();
        let this = collision_object(&mut e);
        e.call(0x00c6bd60, &args![this, true]);
        assert_eq!(e.get(this, BhkNiCollisionObject::m_uFlags), 0);
        let havok = e.mem.alloc(0x40);
        e.mem.set_u32(havok + 0x2c, 0x8000_0000 | 0x11);
        let body = e.mem.alloc(0x20);
        e.mem.set_u32(body + 8, havok);
        e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(body));
        e.call(0x00c6bd60, &args![this, true]);
        assert_eq!(e.get(this, BhkNiCollisionObject::m_uFlags), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn set_debug_display_hides_by_detaching_the_named_children() {
        let mut e = engine();
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x9000);
        e.register(0x7000_000c, |_, a| ret(a[0]));
        e.register(0x7000_00d0, |_, _| ret(0));
        e.map(STRING_BLOCK, 0x20);
        e.mem.set_u32(STRING_BLOCK, 5);
        e.register(FIXED_STRING_CREATE, |_, _| ret(STRING_HANDLE));
        e.register(DELETE, |_, _| ret(0));
        // GetObjectByName answers the child's own "found" word at +0x24.
        e.register(NI_AV_OBJECT_GET_OBJECT_BY_NAME, |e, a| {
            assert_eq!(e.mem.u32(a[1]), STRING_HANDLE);
            ret(e.mem.u32(a[0] + 0x24))
        });
        // Detach hands the detached object back through the out pointer.
        e.register(0x7000_00e4, |e, a| {
            e.mem.set_u32(a[2], a[1]);
            ret(0)
        });
        let node = object_with_slots(&mut e, 0xc0, &[(0xc, 0x7000_000c), (0xe4, 0x7000_00e4)]);
        let detached = object_with_slots(&mut e, 0x20, &[(4, DELETE)]);
        e.mem.set_u32(detached + 4, 1);
        let with_display = e.mem.alloc(0x40);
        e.mem.set_u32(with_display + 0x24, detached);
        let without_display = e.mem.alloc(0x40);
        let array = e.mem.alloc(12);
        e.mem.set_u32(array, with_display);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array + 8, without_display);
        e.mem.set_u32(node + 0xa0, array);
        e.mem.set_u16(node + 0xa6, 3);
        let body = object_with_slots(&mut e, 0x20, &[(0xd0, 0x7000_00d0)]);
        let this = collision_object(&mut e);
        e.set(this, BhkNiCollisionObject::m_pkSceneObject, Ptr::new(node));
        e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(body));
        e.set(this, BhkNiCollisionObject::m_uFlags, 0x51);

        e.call(0x00c6bd60, &args![this, false]);

        assert_eq!(e.get(this, BhkNiCollisionObject::m_uFlags), 0x41);
        assert_eq!(calls_to(&e, 0x7000_00d0), vec![vec![body]]);
        assert_eq!(calls_to(&e, 0x7000_00e4).len(), 1);
        // The detached object's last reference is released (its delete slot runs).
        assert_eq!(calls_to(&e, DELETE), vec![vec![detached]]);
        // Two names were created (the null child is skipped) and each handle released.
        assert_eq!(calls_to(&e, FIXED_STRING_CREATE).len(), 2);
    }

    #[test]
    fn world_object_flag_call_needs_a_collision_object_with_a_world_object() {
        let mut e = engine();
        e.register(0x7000_0038, |e, a| {
            ret(if e.mem.u32(a[0] + 0x10) == 0xdead {
                0
            } else {
                a[0]
            })
        });
        e.register(WORLD_OBJECT_SET_FLAG, |_, _| ret(0));
        let collision = object_with_slots(&mut e, 0x20, &[(0x38, 0x7000_0038)]);
        let node = e.mem.alloc(0x40);
        e.mem.set_u32(node + 0x1c, collision);
        // No world object: nothing.
        e.call(0x00c6c0e0, &args![node, 1u32]);
        assert!(calls_to(&e, WORLD_OBJECT_SET_FLAG).is_empty());
        e.mem.set_u32(collision + 0x10, 0x4444);
        e.call(0x00c6c0e0, &args![node, 1u32]);
        assert_eq!(calls_to(&e, WORLD_OBJECT_SET_FLAG), vec![vec![0x4444, 1]]);
        // The cast answers null.
        e.mem.set_u32(collision + 0x10, 0xdead);
        e.call(0x00c6c0e0, &args![node, 1u32]);
        assert_eq!(calls_to(&e, WORLD_OBJECT_SET_FLAG).len(), 1);
        // No collision object.
        e.mem.set_u32(node + 0x1c, 0);
        e.call(0x00c6c0e0, &args![node, 1u32]);
        assert_eq!(calls_to(&e, WORLD_OBJECT_SET_FLAG).len(), 1);
    }

    #[test]
    fn synchronize_rec_updates_marked_nodes_and_their_children() {
        let mut e = engine();
        e.register(0x7000_0038, |_, a| ret(a[0]));
        e.register(0x7000_000c, |_, a| ret(a[0]));
        e.register(0x7000_00b8, |_, _| ret(0));
        e.register(0x7000_00bc, |_, _| ret(0));
        e.register(NI_AV_OBJECT_PREPARE_UPDATE, |_, _| ret(0));
        let slots = [(0xc, 0x7000_000c), (0xb8, 0x7000_00b8), (0xbc, 0x7000_00bc)];
        let collision = object_with_slots(&mut e, 0x20, &[(0x38, 0x7000_0038)]);
        let child = object_with_slots(&mut e, 0xc0, &slots);
        let parent = object_with_slots(&mut e, 0xc0, &slots);
        e.mem.set_u32(parent + 0x1c, collision);
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, child);
        e.mem.set_u32(parent + 0xa0, array);
        e.mem.set_u16(parent + 0xa6, 1);

        e.call(0x00c6c110, &args![parent, 0u32]);

        // The parent updates (it has a collision object), and its update turns the flag on for
        // the child, which updates too; each ends with its slot +0xbc.
        let updated: Vec<u32> = calls_to(&e, 0x7000_00b8).iter().map(|c| c[0]).collect();
        assert_eq!(updated, [parent, child]);
        assert_eq!(
            calls_to(&e, NI_AV_OBJECT_PREPARE_UPDATE),
            vec![vec![parent], vec![child]]
        );
        let finished: Vec<u32> = calls_to(&e, 0x7000_00bc).iter().map(|c| c[0]).collect();
        assert_eq!(finished, [child, parent]);

        // A node without a collision object and a zero flag is not updated, but a flag of 0x100
        // (low byte zero) is still "off"; 0x101 is on.
        let plain = object_with_slots(&mut e, 0xc0, &slots);
        e.call(0x00c6c110, &args![plain, 0x100u32]);
        assert_eq!(calls_to(&e, 0x7000_00b8).len(), 2);
        e.call(0x00c6c110, &args![plain, 0x101u32]);
        assert_eq!(calls_to(&e, 0x7000_00b8).len(), 3);
        e.call(0x00c6c110, &args![0u32, 1u32]);
    }

    #[test]
    fn synchronize_sets_the_thread_word_around_the_walk() {
        let mut e = engine();
        let word = e.tls() + THREAD_SYNCHRONIZE_ARGUMENT;
        e.mem.set_u32(word, 0x11);
        let seen = Rc::new(RefCell::new(0));
        let during = seen.clone();
        e.register(0x7000_000c, |_, _| ret(0));
        e.register_double(0x7000_00bc, move |e, _| {
            *during.borrow_mut() = e.mem.u32(word);
            ret(0)
        });
        let node = object_with_slots(&mut e, 0xc0, &[(0xc, 0x7000_000c), (0xbc, 0x7000_00bc)]);
        e.call(0x00c6c3d0, &args![node, 0x77u32]);
        assert_eq!(*seen.borrow(), 0x77);
        assert_eq!(e.mem.u32(word), 0x11);
        assert_eq!(calls_to(&e, 0x7000_00bc), vec![vec![node]]);
    }

    #[test]
    fn q_keyframe_follows_the_owner_to_its_motion_kind() {
        let mut e = engine();
        let this = collision_object(&mut e);
        // No world object: true.
        assert!(e.call(0x00c6c1d0, &args![this]).bool());
        let body = e.mem.alloc(0x20);
        e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(body));
        // The part is at +0x10 of the Havok object; its byte +0x18 is not 1: true.
        let block = e.mem.alloc(0x200);
        e.mem.set_u32(body + 8, block);
        assert!(e.call(0x00c6c1d0, &args![this]).bool());
        // It is 1: the owner is the part plus the signed byte at the part's +0x10.
        let part = block + 0x10;
        e.mem.set_u8(part + 0x18, 1);
        e.mem.set_i8(part + 0x10, 0x70);
        let owner = part + 0x70;
        for (kind, expected) in [(4, true), (5, true), (1, false), (6, false)] {
            e.mem.set_u8(owner + 0xe8, kind);
            assert_eq!(e.call(0x00c6c1d0, &args![this]).bool(), expected);
        }
        // A negative offset: the owner lies before the part.
        e.mem.set_i8(part + 0x10, -8);
        e.mem.set_u8(part - 8 + 0xe8, 3);
        assert!(!e.call(0x00c6c1d0, &args![this]).bool());
        // An offset of zero points at the part itself (non-zero): its +0xe8 byte is read.
        e.mem.set_i8(part + 0x10, 0);
        e.mem.set_u8(part + 0xe8, 5);
        assert!(e.call(0x00c6c1d0, &args![this]).bool());
    }

    #[test]
    fn set_rigid_body_name_assigns_the_node_name_through_the_accessor() {
        let mut e = engine();
        e.register(0x7000_0038, |_, a| ret(a[0]));
        e.register(0x7000_000c, |_, a| ret(a[0]));
        e.register(STRING_PTR_ASSIGN, |_, _| ret(0));
        let owner_part = e.mem.alloc(0x100);
        let accessor = e.mem.alloc(0x10);
        e.mem.set_u32(accessor + 8, owner_part);
        e.register_double(STATIC_ACCESSOR, move |_, _| ret(accessor));
        let collision = object_with_slots(&mut e, 0x20, &[(0x38, 0x7000_0038)]);
        e.mem.set_u32(collision + 0x10, 0x4444);
        let child = object_with_slots(&mut e, 0xc0, &[(0xc, 0x7000_000c)]);
        e.mem.set_u32(child + 8, 0x6666);
        e.mem.set_u32(child + 0x1c, collision);
        let node = object_with_slots(&mut e, 0xc0, &[(0xc, 0x7000_000c)]);
        e.mem.set_u32(node + 8, 0x5555);
        let array = e.mem.alloc(4);
        e.mem.set_u32(array, child);
        e.mem.set_u32(node + 0xa0, array);
        e.mem.set_u16(node + 0xa6, 1);

        e.call(0x00c6c210, &args![node]);

        assert_eq!(
            calls_to(&e, STATIC_ACCESSOR),
            vec![vec![0x4444, RIGID_BODY_NAME_STATIC]]
        );
        assert_eq!(
            calls_to(&e, STRING_PTR_ASSIGN),
            vec![vec![owner_part + 0x74, 0x6666]]
        );

        // When the accessor answers nothing, no assignment happens.
        e.register(STATIC_ACCESSOR, |_, _| ret(0));
        e.call(0x00c6c210, &args![node]);
        assert_eq!(calls_to(&e, STRING_PTR_ASSIGN).len(), 1);
        e.call(0x00c6c210, &args![0u32]);
    }

    #[test]
    fn destructor_body_clears_the_world_object_and_the_count() {
        let mut e = engine();
        e.register(EMPTY_LOCK, |_, _| ret(0));
        e.register(REMOVE_PROPERTY, |_, _| ret(0));
        e.register(DELETE, |_, _| ret(0));
        e.register(0x7000_00a0, |_, _| ret(0));
        e.register(BASE_DESTRUCTOR_BODY, |_, _| ret(0));
        e.mem.set_u32(OBJECT_COUNT, 3);
        let havok = e.mem.alloc(0x20);
        let body = object_with_slots(&mut e, 0x20, &[(4, DELETE), (0xa0, 0x7000_00a0)]);
        e.mem.set_u32(body + 4, 1);
        e.mem.set_u32(body + 8, havok);
        let this = collision_object(&mut e);
        e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(body));
        e.call(0x00c6c290, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.mem.u32(OBJECT_COUNT), 2);
        assert_eq!(calls_to(&e, 0x7000_00a0), vec![vec![body]]);
        assert!(e.get(this, BhkNiCollisionObject::spWorldObject).is_null());
        assert_eq!(calls_to(&e, DELETE), vec![vec![body]]);
        assert_eq!(calls_to(&e, BASE_DESTRUCTOR_BODY), vec![vec![this.addr()]]);
    }

    #[test]
    fn scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = engine();
        e.register(BASE_DESTRUCTOR_BODY, |_, _| ret(0));
        e.register(SIZED_DELETE, |_, _| ret(0));
        let this = collision_object(&mut e);
        assert_eq!(
            e.call(0x00c6c410, &args![this, 0xfffeu32]).u32(),
            this.addr()
        );
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        assert_eq!(e.call(0x00c6c410, &args![this, 1u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this.addr(), 0x14]]);
        assert_eq!(calls_to(&e, BASE_DESTRUCTOR_BODY).len(), 2);
    }

    /// A stream object with its read (`+0x24c`) and write (`+0x250`) structures; each holds a
    /// byte counter at +4 and the function at +8 / +0xc.
    fn stream_fixture(e: &mut Engine) -> (u32, u32) {
        let stream = object_with_slots(e, 0x300, &[(0x30, 0x7000_0030)]);
        let io = e.mem.alloc(0x20);
        e.mem.set_u32(stream + 0x24c, io);
        e.mem.set_u32(stream + 0x250, io);
        e.mem.set_u32(io + 4, 10);
        e.mem.set_u32(io + 8, 0x7000_0008);
        e.mem.set_u32(io + 0xc, 0x7000_000c);
        (stream, io)
    }

    #[test]
    fn load_binary_reads_the_flags_through_the_stream_function() {
        let mut e = engine();
        e.register(BASE_LOAD_BINARY, |_, _| ret(0));
        e.register(STREAM_FINISH_BINARY, |_, _| ret(0));
        e.register(0x7000_0008, |e, a| {
            // (stream_io, destination, 2, &2, 1): the size cell holds 2.
            assert_eq!((a[2], a[4]), (2, 1));
            assert_eq!(e.mem.u32(a[3]), 2);
            e.mem.set_u16(a[1], 0x1234);
            ret(2)
        });
        let (stream, io) = stream_fixture(&mut e);
        let this = collision_object(&mut e);
        e.call(0x00c6c330, &args![this, stream]);
        assert_eq!(e.get(this, BhkNiCollisionObject::m_uFlags), 0x1234);
        assert_eq!(e.mem.u32(io + 4), 12);
        assert_eq!(calls_to(&e, 0x7000_0008)[0][..2], [io, this.addr() + 0xc]);
        assert_eq!(calls_to(&e, STREAM_FINISH_BINARY), vec![vec![stream]]);
        assert_eq!(
            calls_to(&e, BASE_LOAD_BINARY),
            vec![vec![this.addr(), stream]]
        );
    }

    #[test]
    fn save_binary_writes_the_flags_and_the_world_object_link() {
        let mut e = engine();
        e.register(BASE_SAVE_BINARY, |_, _| ret(0));
        e.register(0x7000_000c, |_, _| ret(2));
        e.register(0x7000_0030, |_, _| ret(0));
        let (stream, io) = stream_fixture(&mut e);
        let this = collision_object(&mut e);
        e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(0x4242));
        e.call(0x00c6c380, &args![this, stream]);
        assert_eq!(e.mem.u32(io + 4), 12);
        assert_eq!(calls_to(&e, 0x7000_000c)[0][..2], [io, this.addr() + 0xc]);
        assert_eq!(calls_to(&e, 0x7000_0030), vec![vec![stream, 0x4242]]);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [0x00c6c380, BASE_SAVE_BINARY, 0x7000_000c, 0x7000_0030]
        );
    }

    #[test]
    fn add_viewer_strings_lists_the_flags_and_the_world_object() {
        let mut e = engine();
        let context = e.tls() + THREAD_CONTEXT;
        e.mem.set_u32(context, 0x31);
        e.mem.set_u32(CLASS_RTTI, 0x5151_0001);
        e.register(BASE_ADD_VIEWER_STRINGS, |_, _| ret(0));
        e.register(NI_GET_VIEWER_STRING, |_, _| ret(0xa000));
        e.register(FORMAT_TEXT, |_, _| ret(0));
        e.register(NI_GET_VIEWER_STRING_TEXT, |_, _| ret(0xb000));
        // The text of a boolean string is its name here.
        e.register(NI_GET_VIEWER_STRING_BOOL, |_, a| ret(a[0]));
        e.register(STRINGS_GROW, |_, _| ret(0));
        let stored = Rc::new(RefCell::new(Vec::new()));
        let seen = stored.clone();
        e.register_double(STRINGS_SET_AT, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[2]));
            ret(0)
        });
        e.register(0x7000_00b4, |_, _| ret(1));
        let scene = e.mem.alloc(0x100);
        e.mem.set_f32(scene + 0x80, -0.5);
        let this = object_with_slots(&mut e, 0x14, &[(0xb4, 0x7000_00b4)]);
        let this = Ptr::<BhkNiCollisionObject>::new(this);
        e.set(this, BhkNiCollisionObject::m_uFlags, 0x45);
        e.set(this, BhkNiCollisionObject::m_pkSceneObject, Ptr::new(scene));
        e.set(this, BhkNiCollisionObject::spWorldObject, Ptr::new(0x6000));
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 20);

        e.call(0x00c6c430, &args![this, strings]);

        assert_eq!(
            *stored.borrow(),
            [
                0xa000,
                0xb000,
                NAME_ACTIVE,
                NAME_RESET,
                NAME_NOTIFY,
                NAME_SET_LOCAL,
                NAME_KEYFRAME,
                NAME_FACE_DOWN
            ]
        );
        // The flags 0x45: bits 0, 6 and 2 set, bit 3 clear; keyframe 1; face down.
        let bools: Vec<u32> = calls_to(&e, NI_GET_VIEWER_STRING_BOOL)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(bools, [1, 1, 1, 0, 1, 1]);
        assert_eq!(calls_to(&e, NI_GET_VIEWER_STRING), vec![vec![0x5151_0001]]);
        let format = calls_to(&e, FORMAT_TEXT);
        assert_eq!(format[0][1..], [0x20, FORMAT_WORLD_OBJECT, 0x6000]);
        assert_eq!(
            calls_to(&e, NI_GET_VIEWER_STRING_TEXT)[0][0],
            NAME_WORLD_OBJECT
        );
        assert_eq!(e.mem.u32(context), 0x31);
    }

    #[test]
    fn push_viewer_string_grows_a_full_array_first() {
        let mut e = engine();
        e.register(STRINGS_GROW, |_, _| ret(0));
        e.register(STRINGS_SET_AT, |_, _| ret(0));
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 3);
        e.mem.set_u16(strings + 0xa, 3);
        e.mem.set_u16(strings + 0xe, 5);
        push_viewer_string(&mut e, strings, 0x99);
        assert_eq!(calls_to(&e, STRINGS_GROW), vec![vec![strings, 8]]);
        assert_eq!(calls_to(&e, STRINGS_SET_AT)[0][..2], [strings, 3]);
        e.mem.set_u16(strings + 8, 30);
        push_viewer_string(&mut e, strings, 0x99);
        assert_eq!(calls_to(&e, STRINGS_GROW).len(), 1);
    }

    #[test]
    fn thunk_00c6b5e0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00c6_b5e0;
        const TARGET: u32 = 0x00a8_10b0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }

    #[test]
    fn bhk_ni_collision_object_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c6_b940, &args![]).u32(), 0x0126_7b70);
    }
}

#[cfg(test)]
mod a05_tests {
    use super::*;

    /// Calls slot-by-slot: returns the addresses called, in order, of the slots and of `00a68bf0`.
    fn update_world_data_calls(
        flags: u16,
        mode: u32,
        keyframe: u32,
        scene_object: u32,
    ) -> (Vec<u32>, u16) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x14);
        let table = e.mem.alloc(0x100);
        for slot in [0xa4u32, 0xa8, 0xac, 0xb4] {
            e.mem.set_u32(table + slot, 0x7000_0000 + slot);
            e.register(0x7000_0000 + slot, |_, _| Ret::default());
        }
        e.register_double(0x7000_00b4, move |_, _| Ret {
            eax: keyframe,
            ..Ret::default()
        });
        e.register(NI_AV_OBJECT_PREPARE_UPDATE, |_, _| Ret::default());
        e.mem.set_u32(this, table);
        e.mem.set_u32(this + 8, scene_object);
        e.mem.set_u16(this + 0xc, flags);
        let word = e.tls() + THREAD_SYNCHRONIZE_ARGUMENT;
        e.mem.set_u32(word, mode);
        e.call_log = Some(vec![]);
        e.call(0x00c6_b670, &args![this, 0u32]);
        let called = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .skip(1)
            .map(|(address, _)| address)
            .collect();
        (called, e.mem.u16(this + 0xc))
    }

    const PREPARE: u32 = NI_AV_OBJECT_PREPARE_UPDATE;

    #[test]
    fn update_world_data_with_the_reset_flag_clears_it_after_the_velocity_and_transform_slots() {
        let (called, flags) = update_world_data_calls(0x0041, 1, 0, 0x5000);
        assert_eq!(called, vec![0x7000_00ac, 0x7000_00a8]);
        assert_eq!(flags, 0x0001);
    }

    #[test]
    fn update_world_data_with_the_reset_flag_in_mode_2_or_3_only_prepares_the_scene_object() {
        for mode in [2, 3] {
            let (called, flags) = update_world_data_calls(0x0041, mode, 0, 0x5000);
            assert_eq!(called, vec![PREPARE]);
            assert_eq!(flags, 0x0041);
        }
    }

    #[test]
    fn update_world_data_for_an_active_keyframed_object_calls_slot_a8_unless_mode_2() {
        let (called, _) = update_world_data_calls(0x0001, 1, 1, 0x5000);
        assert_eq!(called, vec![0x7000_00b4, 0x7000_00a8]);
        let (called, _) = update_world_data_calls(0x0001, 2, 1, 0x5000);
        assert_eq!(called, vec![0x7000_00b4, PREPARE]);
        let (called, _) = update_world_data_calls(0x0001, 3, 1, 0x5000);
        assert_eq!(called, vec![PREPARE]);
    }

    #[test]
    fn update_world_data_for_an_active_free_object_calls_slot_a4_unless_mode_1() {
        let (called, _) = update_world_data_calls(0x0001, 0, 0, 0x5000);
        assert_eq!(called, vec![0x7000_00b4, 0x7000_00a4]);
        let (called, _) = update_world_data_calls(0x0001, 1, 0, 0x5000);
        assert_eq!(called, vec![0x7000_00b4, PREPARE]);
    }

    #[test]
    fn update_world_data_for_an_inactive_object_only_prepares_a_scene_object() {
        let (called, _) = update_world_data_calls(0x0004, 0, 0, 0x5000);
        assert_eq!(called, vec![PREPARE]);
        let (called, _) = update_world_data_calls(0x0004, 0, 0, 0);
        assert!(called.is_empty());
    }
}
