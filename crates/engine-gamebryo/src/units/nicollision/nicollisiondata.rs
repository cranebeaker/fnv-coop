//! `NiCollision/nicollisiondata.cpp` (Xbox PDB source unit), subsystem `NiCollision`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00ead0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::_scalar_deleting_destructor_` (Xbox PDB): runs the destructor body at `00eacde0`,
/// then the sized delete (`00aa1460`, `(this, size)`) of 0x50 bytes when bit 0 of
/// `flags` is set. Answers `this`.
pub fn ni_collision_data_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00ea_cde0;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x50u32]);
    }
    this
}

layout! {
    /// `NiCollisionData` (Xbox PDB), 0x50 bytes on PC as on the Xbox: an `NiCollisionObject`
    /// (vtable, reference count, `m_pkSceneObject`) and the fields below. The two velocities
    /// are `NiPoint3`s, declared as three words each because the code only moves them.
    pub struct NiCollisionData: 0x50 {
        /// `m_pkSceneObject` (Xbox PDB): `NiAVObject*`.
        0x08 m_pkSceneObject: u32,
        /// `m_kLocalVelocity` (Xbox PDB), x.
        0x0C m_kLocalVelocity_x: u32,
        /// `m_kLocalVelocity` (Xbox PDB), y.
        0x10 m_kLocalVelocity_y: u32,
        /// `m_kLocalVelocity` (Xbox PDB), z.
        0x14 m_kLocalVelocity_z: u32,
        /// `m_kWorldVelocity` (Xbox PDB), x.
        0x18 m_kWorldVelocity_x: u32,
        /// `m_kWorldVelocity` (Xbox PDB), y.
        0x1C m_kWorldVelocity_y: u32,
        /// `m_kWorldVelocity` (Xbox PDB), z.
        0x20 m_kWorldVelocity_z: u32,
        /// `m_ePropagationMode` (Xbox PDB).
        0x24 m_ePropagationMode: u32,
        /// `m_eCollisionMode` (Xbox PDB).
        0x28 m_eCollisionMode: u32,
        /// `m_pkModelABV` (Xbox PDB): `NiBoundingVolume*`.
        0x2C m_pkModelABV: u32,
        /// `m_pkWorldABV` (Xbox PDB): `NiBoundingVolume*`.
        0x30 m_pkWorldABV: u32,
        /// `m_pkOBBRoot` (Xbox PDB): `NiOBBRoot*`.
        0x34 m_pkOBBRoot: u32,
        /// `m_pfnCollideCallback` (Xbox PDB).
        0x38 m_pfnCollideCallback: u32,
        /// `m_pvCollideCallbackData` (Xbox PDB).
        0x3C m_pvCollideCallbackData: u32,
        /// `m_pkWorldVertex` (Xbox PDB): `NiPoint3*`.
        0x40 m_pkWorldVertex: u32,
        /// `m_pkWorldNormal` (Xbox PDB): `NiPoint3*`.
        0x44 m_pkWorldNormal: u32,
        /// `m_bWorldVerticesNeedUpdate` (Xbox PDB).
        0x48 m_bWorldVerticesNeedUpdate: u8,
        /// `m_bWorldNormalsNeedUpdate` (Xbox PDB).
        0x49 m_bWorldNormalsNeedUpdate: u8,
        /// `m_usNumVertices` (Xbox PDB).
        0x4A m_usNumVertices: u16,
        /// `m_usNumTriangles` (Xbox PDB).
        0x4C m_usNumTriangles: u16,
        /// `m_bAuxCallbacks` (Xbox PDB).
        0x4E m_bAuxCallbacks: u8,
    }
}

/// `NiCollisionData::ms_RTTI` (Xbox PDB): the `NiRTTI` record, whose first word is the class name.
const NI_COLLISION_DATA_RTTI: u32 = 0x0126_fab4;
/// The vtable stored at +0 (`010f18bc`; its slot 0 is the scalar deleting destructor `00ead0b0`).
const NI_COLLISION_DATA_VTABLE: u32 = 0x010f_18bc;
/// `NiCollisionData::ms_bComputeWorldVelocity` (Xbox PDB).
const COMPUTE_WORLD_VELOCITY: u32 = 0x0126_fab0;
/// The three words at `011f426c` that the constructor and `GetVelocity` copy as the default
/// velocity.
const DEFAULT_VELOCITY_011F426C: u32 = 0x011f_426c;
/// The memory manager instance (`this` of its allocate and deallocate).
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (`00aa3e40`, thiscall on [`MEMORY_MANAGER`], size).
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
/// `MemoryManager::Deallocate` (`00aa4060`, thiscall on [`MEMORY_MANAGER`], block).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;
/// The allocator `00aa1070` (cdecl, size): returns the block.
const ALLOCATE: u32 = 0x00aa_1070;
/// `operator new` (`00aa13e0`, cdecl, size).
const OPERATOR_NEW: u32 = 0x00aa_13e0;
/// `NiSprintf` (`00a305b0`, cdecl: buffer, size, format, then the format's arguments).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `NiGetViewerString` (`00a7d330`, cdecl: the name), the class line of the viewer strings.
const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
/// `NiPoint3::GetViewerString` (`00a59af0`, thiscall on the point, the name).
const NI_POINT3_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
/// `NiCollisionObject::NiCollisionObject` (`00a810d0`, thiscall, the scene object).
const NI_COLLISION_OBJECT_CONSTRUCTOR: u32 = 0x00a8_10d0;
/// The body of `NiCollisionObject`'s destructor (`00a80fc0`, thiscall, no arguments).
const NI_COLLISION_OBJECT_DESTRUCTOR_BODY: u32 = 0x00a8_0fc0;
/// `NiCollisionObject::LoadBinary` (`00a80fd0`, thiscall, the stream).
const NI_COLLISION_OBJECT_LOAD_BINARY: u32 = 0x00a8_0fd0;
/// `NiCollisionObject::LinkObject` (`00a80ff0`, thiscall, the stream).
const NI_COLLISION_OBJECT_LINK_OBJECT: u32 = 0x00a8_0ff0;
/// `NiCollisionObject::RegisterStreamables` (`00a81010`, thiscall, the stream).
const NI_COLLISION_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a8_1010;
/// `NiCollisionObject::SaveBinary` (`00a81040`, thiscall, the stream).
const NI_COLLISION_OBJECT_SAVE_BINARY: u32 = 0x00a8_1040;
/// `NiCollisionObject::IsEqual` (`00a81060`, thiscall, the other object).
const NI_COLLISION_OBJECT_IS_EQUAL: u32 = 0x00a8_1060;
/// `NiCollisionObject::SetSceneGraphObject` (`00a810b0`, thiscall, the scene object).
const NI_COLLISION_OBJECT_SET_SCENE_GRAPH_OBJECT: u32 = 0x00a8_10b0;
/// `NiCollisionObject::GetViewerStrings` (`00a81100`, thiscall, the strings).
const NI_COLLISION_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a8_1100;
/// `NiCollisionObject::AddViewerStrings` (`00a81160`, thiscall, the strings).
const NI_COLLISION_OBJECT_ADD_VIEWER_STRINGS: u32 = 0x00a8_1160;
/// `NiObject::CopyMembers` (`00aa1000`, thiscall: the destination, the cloning process).
const NI_OBJECT_COPY_MEMBERS: u32 = 0x00aa_1000;
/// `NiBoundingVolume::CreateFromStream` (`00e9cab0`, cdecl, one word).
const NI_BOUNDING_VOLUME_CREATE_FROM_STREAM: u32 = 0x00e9_cab0;
/// `NiTriShapeData::GetTriList` (`00a7b850`, thiscall).
const NI_TRI_SHAPE_DATA_GET_TRI_LIST: u32 = 0x00a7_b850;
/// `NiOBBRoot`'s constructor (`00eb5f10`, thiscall: triangle count, the index list, the vertices,
/// the world vertices, one more word).
const NI_OBB_ROOT_CONSTRUCTOR: u32 = 0x00eb_5f10;
/// `NiNode::Transform * NiPoint3` (`004b4500`, thiscall on the transform): out point, in point;
/// answers the out point.
const TRANSFORM_POINT: u32 = 0x004b_4500;
/// The function pointer `011aa050` (a variable in `.data`): transforms an array of points
/// (cdecl: count, source points, destination points, transform).
const TRANSFORM_POINTS_POINTER: u32 = 0x011a_a050;
/// The `NiRTTI` record the first scan of [`fn_00ead380`] looks for in the scene object's class
/// chain.
const RTTI_TRIANGLES_TRI_LIST: u32 = 0x011f_4a40;
/// The `NiRTTI` record the second scan of [`fn_00ead380`] looks for.
const RTTI_TRIANGLES_INDEXED: u32 = 0x011f_4a20;
/// The viewer string names.
const PROPAGATION_MODE_NAME: u32 = 0x010f_1a68;
const COLLISION_MODE_NAME: u32 = 0x010f_1a54;
const MODEL_ABV_TYPE_NAME: u32 = 0x010f_1a40;
const LOCAL_VELOCITY_NAME: u32 = 0x010f_1a2c;
const WORLD_VELOCITY_NAME: u32 = 0x010f_1a18;
/// `"%s = UNKNOWN!!!"`.
const UNKNOWN_FORMAT: u32 = 0x0109_b2ec;
/// `GetViewerString`'s formats, by collision mode: `"%s = USE_OBB"`, `USE_TRI`, `USE_ABV`,
/// `NOTEST`, `USE_NIBOUND`.
const COLLISION_MODE_FORMATS: [u32; 5] = [
    0x010f_19a0,
    0x010f_1990,
    0x010f_1980,
    0x010f_1974,
    0x010f_1960,
];
/// `GetViewerString`'s second overload formats, by propagation mode: `PROPAGATE_ON_SUCCESS`,
/// `PROPAGATE_ON_FAILURE`, `PROPAGATE_ALWAYS`, `PROPAGATE_NEVER`.
const PROPAGATION_MODE_FORMATS: [u32; 4] = [0x010f_19fc, 0x010f_19e0, 0x010f_19c8, 0x010f_19b0];
/// `NiTArray<char*>::SetSize`-shaped grow of the viewer strings array (`0096ad30`, thiscall: the
/// new size).
pub(crate) const VIEWER_STRINGS_GROW: u32 = 0x0096_ad30;
/// The viewer strings array's `SetAt` (`0096ae90`, thiscall: index, pointer to the value).
pub(crate) const VIEWER_STRINGS_SET_AT: u32 = 0x0096_ae90;

/// The length of the zero-terminated string at `text` (the compiler's counting loop).
pub(crate) fn c_string_length(e: &Engine, text: u32) -> u32 {
    let mut length = 0u32;
    while e.mem.u8(text.wrapping_add(length)) != 0 {
        length = length.wrapping_add(1);
    }
    length
}

/// `FLD dword ptr [address]`: the float at `address` as the x87 stack holds it (a signalling
/// NaN quieted). Shared by the bounding volume units.
pub(crate) fn load_float(e: &Engine, address: u32) -> f64 {
    x87_load32(e.mem.u32(address))
}

/// A `float` memory operand of an x87 arithmetic instruction (`FMUL dword ptr [address]` and the
/// like). Shared by the bounding volume units.
pub(crate) fn float_operand(e: &Engine, address: u32) -> f64 {
    x87_m32(e.mem.u32(address))
}

/// Appends the string pointer `string` to the viewer strings array at `strings`, the sequence
/// the compiler inlined in every `GetViewerStrings`/`AddViewerStrings` (16-bit size at +0xa and
/// capacity at +8, grow-by at +0xe): when the size has reached the capacity the array is grown
/// to `size + grow-by` (`0096ad30`), then `SetAt` (`0096ae90`) stores the value at the old size.
pub(crate) fn add_viewer_string(e: &mut Engine, strings: Ptr, string: u32) {
    let base = strings.addr();
    let size = e.mem.u16(base.wrapping_add(0xa)) as u32;
    let capacity = e.mem.u16(base.wrapping_add(8)) as u32;
    if size >= capacity {
        let grow_by = e.mem.u16(base.wrapping_add(0xe)) as u32;
        e.call(
            VIEWER_STRINGS_GROW,
            &args![strings, grow_by.wrapping_add(size)],
        );
    }
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), string);
        e.call(VIEWER_STRINGS_SET_AT, &args![strings, size, value]);
    });
}

/// Whether `rtti` or one of its base classes (the parent pointer at +4) is `target`, starting
/// from the object's own `GetRTTI` (virtual, slot +8).
fn class_chain_contains(e: &mut Engine, object: u32, target: u32) -> bool {
    let mut rtti = e.vcall(object, 8, &args![]).u32();
    while rtti != 0 {
        if rtti == target {
            return true;
        }
        rtti = e.mem.u32(rtti.wrapping_add(4));
    }
    false
}

/// One read or write the exe inlines through a stream's function pointer (`+8` reads, `+0xc`
/// writes): `function(stream, buffer, size, &size, 1)` (cdecl), then the stream's byte counter
/// at +4 grows by the result. `size` is passed again by address in a local that starts at it.
pub(crate) fn stream_transfer(
    e: &mut Engine,
    stream: u32,
    function_offset: u32,
    buffer: u32,
    size: u32,
) {
    e.with_stack(4, |e, size_variable| {
        e.mem.set_u32(size_variable.addr(), size);
        let function = e.mem.u32(stream.wrapping_add(function_offset));
        let done = e
            .call(function, &args![stream, buffer, size, size_variable, 1u32])
            .u32();
        let position = e.mem.u32(stream.wrapping_add(4));
        e.mem
            .set_u32(stream.wrapping_add(4), position.wrapping_add(done));
    });
}

// Translated from 00eacd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::RegisterStreamables` (Xbox PDB): `NiCollisionObject`'s registration of the
/// stream, as a `bool`.
pub fn ni_collision_data_register_streamables(
    e: &mut Engine,
    this: Ptr<NiCollisionData>,
    stream: Ptr,
) -> bool {
    e.call(
        NI_COLLISION_OBJECT_REGISTER_STREAMABLES,
        &args![this, stream],
    )
    .bool()
}

// Translated from 00eacd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_collision_data_get_rtti(_e: &mut Engine, _this: Ptr<NiCollisionData>) -> u32 {
    NI_COLLISION_DATA_RTTI
}

// Translated from 00eacd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::GetVelocity` (Xbox PDB): writes the three words at `011f426c` to `out`
/// (the hidden return value) and answers `out`; the object is not read.
pub fn ni_collision_data_get_velocity(
    e: &mut Engine,
    _this: Ptr<NiCollisionData>,
    out: Ptr,
) -> Ptr {
    let x = e.mem.u32(DEFAULT_VELOCITY_011F426C);
    let y = e.mem.u32(DEFAULT_VELOCITY_011F426C + 4);
    e.mem.set_u32(out.addr(), x);
    let z = e.mem.u32(DEFAULT_VELOCITY_011F426C + 8);
    e.mem.set_u32(out.addr().wrapping_add(4), y);
    e.mem.set_u32(out.addr().wrapping_add(8), z);
    out
}

// Translated from 00eacd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiCollisionData` constructor. Runs
/// `NiCollisionObject`'s constructor with a null scene object, clears the world vertices and
/// normals and their two update flags, sets the vtable `010f18bc`, copies the three words at
/// `011f426c` to both velocities, clears the triangle count, the model and world bounding
/// volumes, the OBB root, the two callback fields and `m_bAuxCallbacks`, and sets the
/// propagation mode to 2 and the collision mode to 3. `m_usNumVertices` (+0x4a) is not written.
pub fn fn_00eacd60(e: &mut Engine, this: Ptr<NiCollisionData>) -> Ptr<NiCollisionData> {
    e.call(NI_COLLISION_OBJECT_CONSTRUCTOR, &args![this, 0u32]);
    e.set(this, NiCollisionData::m_pkWorldVertex, 0);
    e.set(this, NiCollisionData::m_pkWorldNormal, 0);
    e.set(this, NiCollisionData::m_bWorldVerticesNeedUpdate, 0);
    e.set(this, NiCollisionData::m_bWorldNormalsNeedUpdate, 0);
    e.mem.set_u32(this.addr(), NI_COLLISION_DATA_VTABLE);
    let x = e.mem.u32(DEFAULT_VELOCITY_011F426C);
    e.set(this, NiCollisionData::m_kLocalVelocity_x, x);
    let y = e.mem.u32(DEFAULT_VELOCITY_011F426C + 4);
    e.set(this, NiCollisionData::m_kLocalVelocity_y, y);
    let z = e.mem.u32(DEFAULT_VELOCITY_011F426C + 8);
    e.set(this, NiCollisionData::m_kLocalVelocity_z, z);
    let x = e.mem.u32(DEFAULT_VELOCITY_011F426C);
    e.set(this, NiCollisionData::m_kWorldVelocity_x, x);
    let y = e.mem.u32(DEFAULT_VELOCITY_011F426C + 4);
    e.set(this, NiCollisionData::m_kWorldVelocity_y, y);
    let z = e.mem.u32(DEFAULT_VELOCITY_011F426C + 8);
    e.set(this, NiCollisionData::m_usNumTriangles, 0);
    e.set(this, NiCollisionData::m_pkModelABV, 0);
    e.set(this, NiCollisionData::m_pkWorldABV, 0);
    e.set(this, NiCollisionData::m_pkOBBRoot, 0);
    e.set(this, NiCollisionData::m_pfnCollideCallback, 0);
    e.set(this, NiCollisionData::m_pvCollideCallbackData, 0);
    e.set(this, NiCollisionData::m_bAuxCallbacks, 0);
    e.set(this, NiCollisionData::m_kWorldVelocity_z, z);
    e.set(this, NiCollisionData::m_ePropagationMode, 2);
    e.set(this, NiCollisionData::m_eCollisionMode, 3);
    this
}

// Translated from 00ead090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, and not a function Ghidra created: allocates 0x50 bytes with
/// `operator new` (`00aa13e0`) and, when that gives a block, runs the constructor
/// ([`fn_00eacd60`], a tail jump) on it. Answers the object, or null.
pub fn fn_00ead090(e: &mut Engine) -> Ptr<NiCollisionData> {
    let block = e.call(OPERATOR_NEW, &args![0x50u32]).u32();
    if block != 0 {
        return fn_00eacd60(e, Ptr::new(block));
    }
    Ptr::NULL
}

// Translated from 00eacde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the destructor body. Sets the vtable `010f18bc`, frees the world
/// vertex and world normal arrays (`MemoryManager::Deallocate`), deletes the model and the world
/// bounding volumes through their scalar deleting destructor (virtual slot +8, flag 1) and the
/// OBB root through its slot 0 (flag 1), clears the OBB root pointer, and ends in
/// `NiCollisionObject`'s destructor body (`00a80fc0`, a tail jump).
pub fn fn_00eacde0(e: &mut Engine, this: Ptr<NiCollisionData>) {
    e.mem.set_u32(this.addr(), NI_COLLISION_DATA_VTABLE);
    let world_vertex = e.get(this, NiCollisionData::m_pkWorldVertex);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, world_vertex]);
    let world_normal = e.get(this, NiCollisionData::m_pkWorldNormal);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, world_normal]);
    let model = e.get(this, NiCollisionData::m_pkModelABV);
    if model != 0 {
        e.vcall(model, 8, &args![1u32]);
    }
    let world = e.get(this, NiCollisionData::m_pkWorldABV);
    if world != 0 {
        e.vcall(world, 8, &args![1u32]);
    }
    let root = e.get(this, NiCollisionData::m_pkOBBRoot);
    if root != 0 {
        e.vcall(root, 0, &args![1u32]);
    }
    e.set(this, NiCollisionData::m_pkOBBRoot, 0);
    e.call(NI_COLLISION_OBJECT_DESTRUCTOR_BODY, &args![this]);
}

// Translated from 00eace50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::SetModelSpaceABV` (Xbox PDB): deletes the old model and world bounding
/// volumes (virtual slot +8, flag 1), stores `volume` as the model one and, when it is not null,
/// makes the world one with its `Create` (slot +0x18) and updates that from the model volume and
/// the scene object's world transform (slot +0x14, the transform at the scene object + 0x68)
/// when there is a scene object; a null `volume` clears the world volume.
pub fn ni_collision_data_set_model_space_abv(
    e: &mut Engine,
    this: Ptr<NiCollisionData>,
    volume: Ptr,
) {
    let model = e.get(this, NiCollisionData::m_pkModelABV);
    if model != 0 {
        e.vcall(model, 8, &args![1u32]);
    }
    let world = e.get(this, NiCollisionData::m_pkWorldABV);
    if world != 0 {
        e.vcall(world, 8, &args![1u32]);
    }
    e.set(this, NiCollisionData::m_pkModelABV, volume.addr());
    if volume.is_null() {
        e.set(this, NiCollisionData::m_pkWorldABV, 0);
        return;
    }
    let created = e.vcall(volume.addr(), 0x18, &args![]).u32();
    let scene = e.get(this, NiCollisionData::m_pkSceneObject);
    e.set(this, NiCollisionData::m_pkWorldABV, created);
    if scene != 0 {
        let model = e.get(this, NiCollisionData::m_pkModelABV);
        e.vcall(created, 0x14, &args![model, scene.wrapping_add(0x68)]);
    }
}

// Translated from 00eaceb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::Initialize` (Xbox PDB): `init` points at the initialization block of four
/// words. Sets the scene object through the virtual `SetSceneGraphObject` (slot +0x8c) with the
/// word at +0, takes the propagation mode from +4, and when the word at +8 is not 0 makes the
/// model bounding volume from the word at +0xc (`NiBoundingVolume::CreateFromStream`,
/// `00e9cab0`) and installs it with [`ni_collision_data_set_model_space_abv`].
pub fn ni_collision_data_initialize(e: &mut Engine, this: Ptr<NiCollisionData>, init: Ptr) {
    let scene = e.mem.u32(init.addr());
    e.vcall(this.addr(), 0x8c, &args![scene]);
    let propagation = e.mem.u32(init.addr().wrapping_add(4));
    e.set(this, NiCollisionData::m_ePropagationMode, propagation);
    if e.mem.u32(init.addr().wrapping_add(8)) != 0 {
        let stream = e.mem.u32(init.addr().wrapping_add(0xc));
        let volume = e
            .call(NI_BOUNDING_VOLUME_CREATE_FROM_STREAM, &args![stream])
            .u32();
        ni_collision_data_set_model_space_abv(e, this, Ptr::new(volume));
    }
}

// Translated from 00eacef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::GetViewerString` (Xbox PDB), a static: formats `"<name> = <mode>"` for a
/// collision mode into a block of `strlen(name) + 0x11` bytes (`00aa1070`) with `NiSprintf`
/// (`00a305b0`) and answers the block. The format depends on the mode: 0 `USE_OBB`, 1 `USE_TRI`,
/// 2 `USE_ABV`, 3 `NOTEST`, 4 `USE_NIBOUND`, anything else `UNKNOWN!!!`.
pub fn ni_collision_data_get_viewer_string(e: &mut Engine, name: Ptr, mode: u32) -> Ptr {
    let size = c_string_length(e, name.addr()).wrapping_add(0x11);
    let block = e.call(ALLOCATE, &args![size]).u32();
    let format = match COLLISION_MODE_FORMATS.get(mode as usize) {
        Some(format) => *format,
        None => UNKNOWN_FORMAT,
    };
    e.call(NI_SPRINTF, &args![block, size, format, name]);
    Ptr::new(block)
}

// Translated from 00eacfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::GetViewerString_ov2` (Xbox PDB), a static: as
/// [`ni_collision_data_get_viewer_string`] for a propagation mode, into a block of
/// `strlen(name) + 0x20` bytes. The format: 0 `PROPAGATE_ON_SUCCESS`, 1 `PROPAGATE_ON_FAILURE`,
/// 2 `PROPAGATE_ALWAYS`, 3 `PROPAGATE_NEVER`, anything else `UNKNOWN!!!`.
pub fn ni_collision_data_get_viewer_string_ov2(e: &mut Engine, name: Ptr, mode: u32) -> Ptr {
    let size = c_string_length(e, name.addr()).wrapping_add(0x20);
    let block = e.call(ALLOCATE, &args![size]).u32();
    let format = match PROPAGATION_MODE_FORMATS.get(mode as usize) {
        Some(format) => *format,
        None => UNKNOWN_FORMAT,
    };
    e.call(NI_SPRINTF, &args![block, size, format, name]);
    Ptr::new(block)
}

// Translated from 00ead0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the first ancestor of `node` (following the parent pointer at
/// +0x18) that has a collision object (the word at +0x1c), or null; null for a null `node`. The
/// object is not read.
pub fn fn_00ead0d0(e: &mut Engine, _this: Ptr<NiCollisionData>, node: Ptr) -> Ptr {
    let mut current = node.addr();
    if current == 0 {
        return Ptr::NULL;
    }
    loop {
        current = e.mem.u32(current.wrapping_add(0x18));
        if current == 0 {
            return Ptr::NULL;
        }
        if e.mem.u32(current.wrapping_add(0x1c)) != 0 {
            return Ptr::new(current);
        }
    }
}

// Translated from 00ead0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::UpdateWorldData` (Xbox PDB; the one stack word, the time, is not read).
/// Updates the world bounding volume from the model one and the scene object's world transform
/// (slot +0x14 of the world volume, transform at the scene object + 0x68) when there is a model
/// volume. When `ms_bComputeWorldVelocity` (`0126fab0`) is set: a scene object without a parent
/// (+0x18) takes the local velocity as the world velocity; otherwise the local velocity is
/// transformed by the parent's world transform (`004b4500`) and, when the parent or its first
/// ancestor with a collision object ([`fn_00ead0d0`], tried when the parent has none) has one,
/// the world velocity of that collision object (+0x18) is added to it (x87 additions rounded to
/// floats); with no such collision object the transformed velocity alone is the world velocity.
/// Both update flags (+0x48, +0x49) are set at the end on every path.
pub fn ni_collision_data_update_world_data(
    e: &mut Engine,
    this: Ptr<NiCollisionData>,
    _unused_1: u32,
) {
    let model = e.get(this, NiCollisionData::m_pkModelABV);
    if model != 0 {
        let scene = e.get(this, NiCollisionData::m_pkSceneObject);
        let world = e.get(this, NiCollisionData::m_pkWorldABV);
        e.vcall(world, 0x14, &args![model, scene.wrapping_add(0x68)]);
    }
    if e.mem.u8(COMPUTE_WORLD_VELOCITY) != 0 {
        let scene = e.get(this, NiCollisionData::m_pkSceneObject);
        let parent = e.mem.u32(scene.wrapping_add(0x18));
        if parent == 0 {
            let x = e.get(this, NiCollisionData::m_kLocalVelocity_x);
            let y = e.get(this, NiCollisionData::m_kLocalVelocity_y);
            let z = e.get(this, NiCollisionData::m_kLocalVelocity_z);
            e.set(this, NiCollisionData::m_kWorldVelocity_x, x);
            e.set(this, NiCollisionData::m_kWorldVelocity_y, y);
            e.set(this, NiCollisionData::m_kWorldVelocity_z, z);
        } else {
            // The exe also tests the scene object (read just above, so never null here) and
            // the parent (not null in this branch) before the ancestor search.
            let mut owner = if scene == 0 { 0 } else { parent };
            if owner != 0 && e.mem.u32(owner.wrapping_add(0x1c)) == 0 {
                owner = fn_00ead0d0(e, this, Ptr::new(owner)).addr();
            }
            let ancestor_data = if owner == 0 {
                0
            } else {
                e.mem.u32(owner.wrapping_add(0x1c))
            };
            let local = this.addr().wrapping_add(0xc);
            e.with_stack(12, |e, out| {
                // The transformed velocity is read back through the pointer the call answers.
                let result = e
                    .call(
                        TRANSFORM_POINT,
                        &args![parent.wrapping_add(0x68), out, local],
                    )
                    .u32();
                let (x, y, z) = if ancestor_data != 0 {
                    let mut sums = [0u32; 3];
                    for (axis, sum) in sums.iter_mut().enumerate() {
                        let offset = 4 * axis as u32;
                        let ancestor = e.mem.u32(ancestor_data.wrapping_add(0x18 + offset));
                        let moved = e.mem.u32(result.wrapping_add(offset));
                        *sum = x87_store32(x87_add(x87_load32(ancestor), x87_m32(moved)));
                    }
                    (sums[0], sums[1], sums[2])
                } else {
                    (
                        e.mem.u32(result),
                        e.mem.u32(result.wrapping_add(4)),
                        e.mem.u32(result.wrapping_add(8)),
                    )
                };
                e.set(this, NiCollisionData::m_kWorldVelocity_x, x);
                e.set(this, NiCollisionData::m_kWorldVelocity_y, y);
                e.set(this, NiCollisionData::m_kWorldVelocity_z, z);
            });
        }
    }
    e.set(this, NiCollisionData::m_bWorldNormalsNeedUpdate, 1);
    e.set(this, NiCollisionData::m_bWorldVerticesNeedUpdate, 1);
}

// Translated from 00ead1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: makes the world vertex array. With a scene object that is a
/// triangle-based geometry (virtual slot +0x1c answers it, the PDB's `IsTriBasedGeom`), no world
/// vertex array yet, and a geometry data (+0xb8) with a non-zero vertex count (16-bit at +8):
/// stores the count at +0x4a, allocates `count * 12` bytes (all ones on carry) from the memory
/// manager into +0x40, flags the world vertices as needing an update, and stores the data's
/// triangle count (16-bit at +0x40) at +0x4c.
pub fn fn_00ead1e0(e: &mut Engine, this: Ptr<NiCollisionData>) {
    let scene = e.get(this, NiCollisionData::m_pkSceneObject);
    if scene == 0 {
        return;
    }
    let geometry = e.vcall(scene, 0x1c, &args![]).u32();
    if geometry == 0 {
        return;
    }
    if e.get(this, NiCollisionData::m_pkWorldVertex) != 0 {
        return;
    }
    let data = e.mem.u32(geometry.wrapping_add(0xb8));
    let count = e.mem.u16(data.wrapping_add(8));
    if count == 0 {
        return;
    }
    e.set(this, NiCollisionData::m_usNumVertices, count);
    let size = (count as u32).checked_mul(12).unwrap_or(u32::MAX);
    let block = e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    e.set(this, NiCollisionData::m_pkWorldVertex, block);
    e.set(this, NiCollisionData::m_bWorldVerticesNeedUpdate, 1);
    let data = e.mem.u32(geometry.wrapping_add(0xb8));
    let triangles = e.mem.u16(data.wrapping_add(0x40));
    e.set(this, NiCollisionData::m_usNumTriangles, triangles);
}

// Translated from 00ead250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: refreshes the world vertices. With a scene object that is a
/// geometry (slot +0x18), a world vertex array and the update flag set: the geometry data's
/// virtual slot +0x90 answers the active vertex count (16-bit), and the function pointer at
/// `011aa050` is called (cdecl) with the count, the data's vertices (+0x20), the world vertex
/// array and the geometry's world transform (+0x68); then the flag is cleared.
pub fn fn_00ead250(e: &mut Engine, this: Ptr<NiCollisionData>) {
    let scene = e.get(this, NiCollisionData::m_pkSceneObject);
    if scene == 0 {
        return;
    }
    let geometry = e.vcall(scene, 0x18, &args![]).u32();
    if geometry == 0 {
        return;
    }
    if e.get(this, NiCollisionData::m_pkWorldVertex) == 0 {
        return;
    }
    if e.get(this, NiCollisionData::m_bWorldVerticesNeedUpdate) == 0 {
        return;
    }
    let data = e.mem.u32(geometry.wrapping_add(0xb8));
    let vertices = e.mem.u32(data.wrapping_add(0x20));
    let count = e.vcall(data, 0x90, &args![]).u16() as u32;
    let world_vertices = e.get(this, NiCollisionData::m_pkWorldVertex);
    let function = e.mem.u32(TRANSFORM_POINTS_POINTER);
    e.call(
        function,
        &args![count, vertices, world_vertices, geometry.wrapping_add(0x68)],
    );
    e.set(this, NiCollisionData::m_bWorldVerticesNeedUpdate, 0);
}

// Translated from 00ead2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::IsEqual` (Xbox PDB): `NiCollisionObject::IsEqual` must hold, `other` must be
/// non-null and have this class in its class chain (its virtual `GetRTTI` and the parent links at
/// +4), the collision modes (+0x28) and propagation modes (+0x24) must match, and the model
/// bounding volumes must match: when both exist their virtual `Type` (slot +0xc) must agree, and
/// otherwise the two pointers must be equal.
pub fn ni_collision_data_is_equal(
    e: &mut Engine,
    this: Ptr<NiCollisionData>,
    other: Ptr<NiCollisionData>,
) -> bool {
    let base_equal = e
        .call(NI_COLLISION_OBJECT_IS_EQUAL, &args![this, other])
        .bool();
    if !base_equal || other.is_null() {
        return false;
    }
    if !class_chain_contains(e, other.addr(), NI_COLLISION_DATA_RTTI) {
        return false;
    }
    if e.get(other, NiCollisionData::m_eCollisionMode)
        != e.get(this, NiCollisionData::m_eCollisionMode)
    {
        return false;
    }
    if e.get(other, NiCollisionData::m_ePropagationMode)
        != e.get(this, NiCollisionData::m_ePropagationMode)
    {
        return false;
    }
    let other_volume = e.get(other, NiCollisionData::m_pkModelABV);
    if other_volume != 0 {
        let this_volume = e.get(this, NiCollisionData::m_pkModelABV);
        if this_volume != 0 {
            let other_type = e.vcall(other_volume, 0xc, &args![]).u32();
            let this_type = e.vcall(this_volume, 0xc, &args![]).u32();
            return other_type == this_type;
        }
    }
    other_volume == e.get(this, NiCollisionData::m_pkModelABV)
}

// Translated from 00ead330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::SetSceneGraphObject` (Xbox PDB): `NiCollisionObject`'s setter, then, when the
/// collision mode (+0x28) is 0 or 1 and the new scene object is a geometry (slot +0x18) with a
/// geometry data (+0xb8), sets bits `0x33` in the data's flag byte at +0x38.
pub fn ni_collision_data_set_scene_graph_object(
    e: &mut Engine,
    this: Ptr<NiCollisionData>,
    scene: Ptr,
) {
    e.call(
        NI_COLLISION_OBJECT_SET_SCENE_GRAPH_OBJECT,
        &args![this, scene],
    );
    let mode = e.get(this, NiCollisionData::m_eCollisionMode);
    let scene = e.get(this, NiCollisionData::m_pkSceneObject);
    set_geometry_data_flags(e, scene, mode);
}

/// The tail `SetSceneGraphObject`, `Convert` and `LinkObject` share: for a scene object that is a
/// geometry (slot +0x18) with a geometry data (+0xb8), when `mode` is 0 or 1, sets bits `0x33` of
/// the data's flag byte at +0x38.
fn set_geometry_data_flags(e: &mut Engine, scene: u32, mode: u32) {
    if scene == 0 {
        return;
    }
    if e.vcall(scene, 0x18, &args![]).u32() == 0 {
        return;
    }
    let data = e.mem.u32(scene.wrapping_add(0xb8));
    if data == 0 {
        return;
    }
    if mode == 0 || mode == 1 {
        let flags = e.mem.u8(data.wrapping_add(0x38));
        e.mem.set_u8(data.wrapping_add(0x38), flags | 0x33);
    }
}

// Translated from 00ead380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: makes the OBB root (+0x34) when there is none, from the scene
/// object's triangles; `argument` is handed on to the OBB root constructor (`00eb5f10`, the last
/// of its five words, with the 0x54-byte block from the memory manager). A scene object with the
/// `NiRTTI` record `011f4a40` in its class chain uses the data's triangle list (`00a7b850`);
/// otherwise one with `011f4a20` builds a compacted index list from the data's triangle indices
/// (virtual slot +0xac of the data, one call per triangle), keeping only the triangles whose
/// three indices differ, and frees that list afterwards. Both first make the world vertices
/// ([`fn_00ead1e0`], [`fn_00ead250`]) when there are none. A failed block allocation leaves the
/// root null.
pub fn fn_00ead380(e: &mut Engine, this: Ptr<NiCollisionData>, argument: u32) {
    let scene = e.get(this, NiCollisionData::m_pkSceneObject);
    if scene != 0 && class_chain_contains(e, scene, RTTI_TRIANGLES_TRI_LIST) {
        if e.get(this, NiCollisionData::m_pkOBBRoot) != 0 {
            return;
        }
        if e.get(this, NiCollisionData::m_pkWorldVertex) == 0 {
            fn_00ead1e0(e, this);
            fn_00ead250(e, this);
        }
        let scene = e.get(this, NiCollisionData::m_pkSceneObject);
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x54u32])
            .u32();
        if block == 0 {
            e.set(this, NiCollisionData::m_pkOBBRoot, 0);
            return;
        }
        let data = e.mem.u32(scene.wrapping_add(0xb8));
        let vertices = e.mem.u32(data.wrapping_add(0x20));
        let triangle_list = e.call(NI_TRI_SHAPE_DATA_GET_TRI_LIST, &args![data]).u32();
        let data = e.mem.u32(scene.wrapping_add(0xb8));
        let triangles = e.mem.u16(data.wrapping_add(0x40)) as u32;
        let world_vertices = e.get(this, NiCollisionData::m_pkWorldVertex);
        let root = e
            .call(
                NI_OBB_ROOT_CONSTRUCTOR,
                &args![
                    block,
                    triangles,
                    triangle_list,
                    vertices,
                    world_vertices,
                    argument
                ],
            )
            .u32();
        e.set(this, NiCollisionData::m_pkOBBRoot, root);
        return;
    }
    let scene = e.get(this, NiCollisionData::m_pkSceneObject);
    if scene == 0 || !class_chain_contains(e, scene, RTTI_TRIANGLES_INDEXED) {
        return;
    }
    if e.get(this, NiCollisionData::m_pkOBBRoot) != 0 {
        return;
    }
    if e.get(this, NiCollisionData::m_pkWorldVertex) == 0 {
        fn_00ead1e0(e, this);
        fn_00ead250(e, this);
    }
    let scene = e.get(this, NiCollisionData::m_pkSceneObject);
    let data = e.mem.u32(scene.wrapping_add(0xb8));
    let triangles = e.mem.u16(data.wrapping_add(0x40)) as u32;
    // Three 16-bit indices per triangle: `ESI + ESI * 2`, doubled.
    let list_size = triangles.wrapping_mul(3).wrapping_mul(2);
    let list = e.call(ALLOCATE, &args![list_size]).u32();
    let mut valid = 0u32;
    // The write cursor sits on the second index of the next kept triangle.
    let mut cursor = list.wrapping_add(2);
    for index in 0..triangles {
        let data = e.mem.u32(scene.wrapping_add(0xb8));
        e.vcall(
            data,
            0xac,
            &args![
                index,
                cursor.wrapping_sub(2),
                cursor,
                cursor.wrapping_add(2)
            ],
        );
        let first = e.mem.u16(cursor.wrapping_sub(2));
        let second = e.mem.u16(cursor);
        let third = e.mem.u16(cursor.wrapping_add(2));
        if first != second && first != third && second != third {
            cursor = cursor.wrapping_add(6);
            valid += 1;
        }
    }
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x54u32])
        .u32();
    let root = if block != 0 {
        let data = e.mem.u32(scene.wrapping_add(0xb8));
        let vertices = e.mem.u32(data.wrapping_add(0x20));
        let world_vertices = e.get(this, NiCollisionData::m_pkWorldVertex);
        e.call(
            NI_OBB_ROOT_CONSTRUCTOR,
            &args![block, valid, list, vertices, world_vertices, argument],
        )
        .u32()
    } else {
        0
    };
    e.set(this, NiCollisionData::m_pkOBBRoot, root);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, list]);
}

// Translated from 00ead550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::Convert` (Xbox PDB): converts the modes of a file older than version
/// `0x0a000106` (`version`; the second stack word is not read). The propagation mode (+0x24) is
/// rewritten by an old value: 0 becomes 3, 1 becomes 3 and nothing else happens, 2 becomes 0, 3
/// becomes 2, 4 becomes 2 and nothing else happens (other values are left). For 0, 2 and 3: with
/// a model bounding volume the collision mode (+0x28) becomes 2 and the scene object is asked
/// `IsGeometry` (slot +0x18, the answer unused); without one the collision mode becomes 0 and a
/// geometry data of the scene object gets bits `0x33` set in its flag byte at +0x38.
pub fn ni_collision_data_convert(
    e: &mut Engine,
    this: Ptr<NiCollisionData>,
    version: u32,
    _unused_2: u32,
) {
    if version >= 0x0a00_0106 {
        return;
    }
    match e.get(this, NiCollisionData::m_ePropagationMode) {
        0 => e.set(this, NiCollisionData::m_ePropagationMode, 3),
        1 => {
            e.set(this, NiCollisionData::m_ePropagationMode, 3);
            return;
        }
        2 => e.set(this, NiCollisionData::m_ePropagationMode, 0),
        3 => e.set(this, NiCollisionData::m_ePropagationMode, 2),
        4 => {
            e.set(this, NiCollisionData::m_ePropagationMode, 2);
            return;
        }
        _ => return,
    }
    if e.get(this, NiCollisionData::m_pkModelABV) == 0 {
        let scene = e.get(this, NiCollisionData::m_pkSceneObject);
        e.set(this, NiCollisionData::m_eCollisionMode, 0);
        // The geometry data's bits are set whatever the (now 0) collision mode.
        set_geometry_data_flags(e, scene, 0);
    } else {
        e.set(this, NiCollisionData::m_eCollisionMode, 2);
        let scene = e.get(this, NiCollisionData::m_pkSceneObject);
        if scene != 0 {
            e.vcall(scene, 0x18, &args![]);
        }
    }
}

// Translated from 00ead610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: copies the members of this collision data to `dest`.
/// `00aa1000` copies the base members (destination, cloning process); then the propagation and
/// collision modes are copied, a model bounding volume is cloned (virtual slot +0x1c) into
/// `dest` with [`ni_collision_data_set_model_space_abv`], and when this one has an OBB root
/// `dest` makes the world vertices ([`fn_00ead1e0`], [`fn_00ead250`]) if it has none and then its
/// own root ([`fn_00ead380`] with 1).
pub fn fn_00ead610(
    e: &mut Engine,
    this: Ptr<NiCollisionData>,
    dest: Ptr<NiCollisionData>,
    clone_process: u32,
) {
    e.call(NI_OBJECT_COPY_MEMBERS, &args![this, dest, clone_process]);
    let propagation = e.get(this, NiCollisionData::m_ePropagationMode);
    e.set(dest, NiCollisionData::m_ePropagationMode, propagation);
    let collision = e.get(this, NiCollisionData::m_eCollisionMode);
    e.set(dest, NiCollisionData::m_eCollisionMode, collision);
    let model = e.get(this, NiCollisionData::m_pkModelABV);
    if model != 0 {
        let clone = e.vcall(model, 0x1c, &args![]).u32();
        ni_collision_data_set_model_space_abv(e, dest, Ptr::new(clone));
    }
    if e.get(this, NiCollisionData::m_pkOBBRoot) != 0 {
        if e.get(dest, NiCollisionData::m_pkWorldVertex) == 0 {
            fn_00ead1e0(e, dest);
            fn_00ead250(e, dest);
        }
        fn_00ead380(e, dest, 1);
    }
}

// Translated from 00ead670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::LoadBinary` (Xbox PDB): `NiCollisionObject::LoadBinary`, then reads through
/// the stream's input object (+0x24c) and its read function (+8, with the byte counter at +4):
/// a file older than version `0x0a000106` (+0xd8) has one 4-byte mode, stored at +0x24; a newer
/// one has two, stored at +0x24 and +0x28. A 1-byte flag follows; when it is not 0 the model
/// bounding volume is made from the stream (`NiBoundingVolume::CreateFromStream`) and its world
/// bounding volume with the model's virtual `Create` (slot +0x18).
pub fn ni_collision_data_load_binary(e: &mut Engine, this: Ptr<NiCollisionData>, stream: Ptr) {
    e.call(NI_COLLISION_OBJECT_LOAD_BINARY, &args![this, stream]);
    let version = e.mem.u32(stream.addr().wrapping_add(0xd8));
    let reader = e.mem.u32(stream.addr().wrapping_add(0x24c));
    e.with_stack(8, |e, buffer| {
        if version < 0x0a00_0106 {
            stream_transfer(e, reader, 8, buffer.addr(), 4);
            let mode = e.mem.u32(buffer.addr());
            e.set(this, NiCollisionData::m_ePropagationMode, mode);
        } else {
            stream_transfer(e, reader, 8, buffer.addr(), 4);
            let propagation = e.mem.u32(buffer.addr());
            e.set(this, NiCollisionData::m_ePropagationMode, propagation);
            let reader = e.mem.u32(stream.addr().wrapping_add(0x24c));
            stream_transfer(e, reader, 8, buffer.addr(), 4);
            let collision = e.mem.u32(buffer.addr());
            e.set(this, NiCollisionData::m_eCollisionMode, collision);
        }
        let reader = e.mem.u32(stream.addr().wrapping_add(0x24c));
        stream_transfer(e, reader, 8, buffer.addr(), 1);
        if e.mem.u8(buffer.addr()) != 0 {
            let volume = e
                .call(NI_BOUNDING_VOLUME_CREATE_FROM_STREAM, &args![stream])
                .u32();
            e.set(this, NiCollisionData::m_pkModelABV, volume);
            let world = e.vcall(volume, 0x18, &args![]).u32();
            e.set(this, NiCollisionData::m_pkWorldABV, world);
        }
    });
}

// Translated from 00ead760 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::LinkObject` (Xbox PDB): `NiCollisionObject::LinkObject`, then, for a stream
/// of a version older than `0x0a000110` (+0xd8), the same geometry data flag fix as
/// [`ni_collision_data_set_scene_graph_object`] (bits `0x33` of the data's byte at +0x38 when the
/// collision mode is 0 or 1).
pub fn ni_collision_data_link_object(e: &mut Engine, this: Ptr<NiCollisionData>, stream: Ptr) {
    e.call(NI_COLLISION_OBJECT_LINK_OBJECT, &args![this, stream]);
    if e.mem.u32(stream.addr().wrapping_add(0xd8)) < 0x0a00_0110 {
        let mode = e.get(this, NiCollisionData::m_eCollisionMode);
        let scene = e.get(this, NiCollisionData::m_pkSceneObject);
        set_geometry_data_flags(e, scene, mode);
    }
}

// Translated from 00ead7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::SaveBinary` (Xbox PDB): `NiCollisionObject::SaveBinary`, then writes through
/// the stream's output object (+0x250) and its write function (+0xc, with the byte counter at
/// +4): the propagation mode and the collision mode (4 bytes each, from +0x24 and +0x28), then a
/// 1-byte flag (1 with a model bounding volume, else 0) and, with one, the volume's own virtual
/// `SaveBinary` (slot +4).
pub fn ni_collision_data_save_binary(e: &mut Engine, this: Ptr<NiCollisionData>, stream: Ptr) {
    e.call(NI_COLLISION_OBJECT_SAVE_BINARY, &args![this, stream]);
    let writer = e.mem.u32(stream.addr().wrapping_add(0x250));
    stream_transfer(e, writer, 0xc, this.addr().wrapping_add(0x24), 4);
    let writer = e.mem.u32(stream.addr().wrapping_add(0x250));
    stream_transfer(e, writer, 0xc, this.addr().wrapping_add(0x28), 4);
    let volume = e.get(this, NiCollisionData::m_pkModelABV);
    e.with_stack(4, |e, flag| {
        if volume != 0 {
            let writer = e.mem.u32(stream.addr().wrapping_add(0x250));
            e.mem.set_u8(flag.addr(), 1);
            stream_transfer(e, writer, 0xc, flag.addr(), 1);
            e.vcall(volume, 4, &args![stream]);
        } else {
            let writer = e.mem.u32(stream.addr().wrapping_add(0x250));
            e.mem.set_u8(flag.addr(), 0);
            stream_transfer(e, writer, 0xc, flag.addr(), 1);
        }
    });
}

// Translated from 00ead870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::GetViewerStrings` (Xbox PDB): `NiCollisionObject::GetViewerStrings`, then
/// appends the class line (`NiGetViewerString` of the first word of the class's `NiRTTI` record)
/// to the strings array and calls the virtual `AddViewerStrings` (slot +0x64).
pub fn ni_collision_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiCollisionData>,
    strings: Ptr,
) {
    e.call(
        NI_COLLISION_OBJECT_GET_VIEWER_STRINGS,
        &args![this, strings],
    );
    let class_name = e.mem.u32(NI_COLLISION_DATA_RTTI);
    let line = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    add_viewer_string(e, strings, line);
    e.vcall(this.addr(), 0x64, &args![strings]);
}

// Translated from 00ead8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::AddViewerStrings` (Xbox PDB): `NiCollisionObject::AddViewerStrings`, then
/// appends to the strings array the class line, the propagation mode
/// ([`ni_collision_data_get_viewer_string_ov2`] with `m_ePropagationMode`), the collision mode
/// ([`ni_collision_data_get_viewer_string`] with `m_eCollisionMode`), the model bounding volume's
/// own strings when there is one (its virtual slot +0x2c with `m_pkModelABV->Type`), and the local
/// and world velocities (`NiPoint3::GetViewerString`).
pub fn ni_collision_data_add_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiCollisionData>,
    strings: Ptr,
) {
    e.call(
        NI_COLLISION_OBJECT_ADD_VIEWER_STRINGS,
        &args![this, strings],
    );
    let class_name = e.mem.u32(NI_COLLISION_DATA_RTTI);
    let line = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    add_viewer_string(e, strings, line);
    let propagation = e.get(this, NiCollisionData::m_ePropagationMode);
    let line =
        ni_collision_data_get_viewer_string_ov2(e, Ptr::new(PROPAGATION_MODE_NAME), propagation);
    add_viewer_string(e, strings, line.addr());
    let collision = e.get(this, NiCollisionData::m_eCollisionMode);
    let line = ni_collision_data_get_viewer_string(e, Ptr::new(COLLISION_MODE_NAME), collision);
    add_viewer_string(e, strings, line.addr());
    let volume = e.get(this, NiCollisionData::m_pkModelABV);
    if volume != 0 {
        e.vcall(volume, 0x2c, &args![MODEL_ABV_TYPE_NAME, strings]);
    }
    let line = e
        .call(
            NI_POINT3_GET_VIEWER_STRING,
            &args![this.addr().wrapping_add(0xc), LOCAL_VELOCITY_NAME],
        )
        .u32();
    add_viewer_string(e, strings, line);
    let line = e
        .call(
            NI_POINT3_GET_VIEWER_STRING,
            &args![this.addr().wrapping_add(0x18), WORLD_VELOCITY_NAME],
        )
        .u32();
    add_viewer_string(e, strings, line);
}

// Translated from 00eada20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCollisionData::CreateClone` (Xbox PDB): allocates 0x50 bytes with `operator new`
/// (`00aa13e0`), constructs a collision data in it ([`fn_00eacd60`]) and copies this one's members
/// into it ([`fn_00ead610`]); answers the clone. When the allocation fails the copy still runs
/// with a null destination and the answer is null.
pub fn ni_collision_data_create_clone(
    e: &mut Engine,
    this: Ptr<NiCollisionData>,
    clone_process: u32,
) -> Ptr<NiCollisionData> {
    let block = e.call(OPERATOR_NEW, &args![0x50u32]).u32();
    if block != 0 {
        let clone = fn_00eacd60(e, Ptr::new(block));
        fn_00ead610(e, this, clone, clone_process);
        return clone;
    }
    fn_00ead610(e, this, Ptr::NULL, clone_process);
    Ptr::NULL
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ead0b0, ni_collision_data_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(
            0x00eacd00,
            ni_collision_data_register_streamables(Ptr<NiCollisionData>, Ptr) -> bool
        ),
        entry!(
            0x00eacd20,
            ni_collision_data_get_rtti(Ptr<NiCollisionData>) -> u32
        ),
        entry!(
            0x00eacd30,
            ni_collision_data_get_velocity(Ptr<NiCollisionData>, Ptr) -> Ptr
        ),
        entry!(
            0x00eacd60,
            fn_00eacd60(Ptr<NiCollisionData>) -> Ptr<NiCollisionData>
        ),
        entry!(0x00eacde0, fn_00eacde0(Ptr<NiCollisionData>)),
        entry!(
            0x00eace50,
            ni_collision_data_set_model_space_abv(Ptr<NiCollisionData>, Ptr)
        ),
        entry!(
            0x00eaceb0,
            ni_collision_data_initialize(Ptr<NiCollisionData>, Ptr)
        ),
        entry!(0x00eacef0, ni_collision_data_get_viewer_string(Ptr, u32) -> Ptr),
        entry!(0x00eacfd0, ni_collision_data_get_viewer_string_ov2(Ptr, u32) -> Ptr),
        entry!(0x00ead090, fn_00ead090() -> Ptr<NiCollisionData>),
        entry!(0x00ead0d0, fn_00ead0d0(Ptr<NiCollisionData>, Ptr) -> Ptr),
        entry!(
            0x00ead0f0,
            ni_collision_data_update_world_data(Ptr<NiCollisionData>, u32)
        ),
        entry!(0x00ead1e0, fn_00ead1e0(Ptr<NiCollisionData>)),
        entry!(0x00ead250, fn_00ead250(Ptr<NiCollisionData>)),
        entry!(
            0x00ead2b0,
            ni_collision_data_is_equal(Ptr<NiCollisionData>, Ptr<NiCollisionData>) -> bool
        ),
        entry!(
            0x00ead330,
            ni_collision_data_set_scene_graph_object(Ptr<NiCollisionData>, Ptr)
        ),
        entry!(0x00ead380, fn_00ead380(Ptr<NiCollisionData>, u32)),
        entry!(
            0x00ead550,
            ni_collision_data_convert(Ptr<NiCollisionData>, u32, u32)
        ),
        entry!(
            0x00ead610,
            fn_00ead610(Ptr<NiCollisionData>, Ptr<NiCollisionData>, u32)
        ),
        entry!(
            0x00ead670,
            ni_collision_data_load_binary(Ptr<NiCollisionData>, Ptr)
        ),
        entry!(
            0x00ead760,
            ni_collision_data_link_object(Ptr<NiCollisionData>, Ptr)
        ),
        entry!(
            0x00ead7b0,
            ni_collision_data_save_binary(Ptr<NiCollisionData>, Ptr)
        ),
        entry!(
            0x00ead870,
            ni_collision_data_get_viewer_strings(Ptr<NiCollisionData>, Ptr)
        ),
        entry!(
            0x00ead8d0,
            ni_collision_data_add_viewer_strings(Ptr<NiCollisionData>, Ptr)
        ),
        entry!(
            0x00eada20,
            ni_collision_data_create_clone(Ptr<NiCollisionData>, u32) -> Ptr<NiCollisionData>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_collision_data_scalar_deleting_destructor_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00ea_cde0, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00ead0b0, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00ead0b0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0x50]]);
        assert_eq!(calls_to(&e, 0x00ea_cde0), vec![vec![this]; 2]);
    }

    /// The argument words of every logged call to `addr`.
    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|call| call.0 == addr)
            .map(|call| call.1.clone())
            .collect()
    }

    fn nothing(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// An object whose vtable (at `vtable`) holds the given (byte offset, function) slots.
    fn vtable_object(e: &mut Engine, vtable: u32, slots: &[(u32, u32)]) -> u32 {
        e.map(vtable, 0x200);
        for (offset, function) in slots {
            e.mem.set_u32(vtable + offset, *function);
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    /// An engine with the memory manager's allocation and the data pages the unit reads mapped.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x1000);
        e.map(0x011a_a000, 0x1000);
        e.map(0x011f_4000, 0x1000);
        e.register(MEMORY_ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[1]),
            ..Ret::default()
        });
        e.register(MEMORY_DEALLOCATE, nothing);
        e.call_log = Some(vec![]);
        e
    }

    fn collision_data(e: &mut Engine) -> Ptr<NiCollisionData> {
        e.new_object::<NiCollisionData>()
    }

    #[test]
    fn register_streamables_answers_the_low_byte_of_the_base_result() {
        let mut e = engine();
        e.register(NI_COLLISION_OBJECT_REGISTER_STREAMABLES, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        assert!(e
            .call(0x00ea_cd00, &args![0x5000u32, 0x1234_5601u32])
            .bool());
        assert!(!e
            .call(0x00ea_cd00, &args![0x5000u32, 0x1234_5600u32])
            .bool());
        assert_eq!(
            logged(&e, NI_COLLISION_OBJECT_REGISTER_STREAMABLES),
            vec![vec![0x5000, 0x1234_5601], vec![0x5000, 0x1234_5600]]
        );
    }

    #[test]
    fn get_rtti_answers_the_class_record() {
        let mut e = engine();
        assert_eq!(e.call(0x00ea_cd20, &args![0x5000u32]).u32(), 0x0126_fab4);
    }

    #[test]
    fn get_velocity_copies_the_default_velocity_to_the_out_point() {
        let mut e = engine();
        for (i, value) in [1.5f32, -2.0, 3.25].iter().enumerate() {
            e.mem
                .set_f32(DEFAULT_VELOCITY_011F426C + 4 * i as u32, *value);
        }
        let out = e.mem.alloc(12);
        assert_eq!(e.call(0x00ea_cd30, &args![0x5000u32, out]).u32(), out);
        assert_eq!(e.mem.f32(out), 1.5);
        assert_eq!(e.mem.f32(out + 4), -2.0);
        assert_eq!(e.mem.f32(out + 8), 3.25);
    }

    #[test]
    fn the_constructor_sets_the_defaults_and_leaves_the_vertex_count() {
        let mut e = engine();
        e.register(NI_COLLISION_OBJECT_CONSTRUCTOR, nothing);
        e.mem.set_u32(DEFAULT_VELOCITY_011F426C, 0x1111);
        e.mem.set_u32(DEFAULT_VELOCITY_011F426C + 4, 0x2222);
        e.mem.set_u32(DEFAULT_VELOCITY_011F426C + 8, 0x3333);
        let this = e.mem.alloc(0x50);
        e.mem.write(this, &[0xaa; 0x50]);
        assert_eq!(e.call(0x00ea_cd60, &args![this]).u32(), this);
        assert_eq!(
            logged(&e, NI_COLLISION_OBJECT_CONSTRUCTOR),
            vec![vec![this, 0]]
        );
        assert_eq!(e.mem.u32(this), 0x010f_18bc);
        let word = |e: &Engine, offset: u32| e.mem.u32(this + offset);
        assert_eq!(word(&e, 0x0c), 0x1111);
        assert_eq!(word(&e, 0x10), 0x2222);
        assert_eq!(word(&e, 0x14), 0x3333);
        assert_eq!(word(&e, 0x18), 0x1111);
        assert_eq!(word(&e, 0x1c), 0x2222);
        assert_eq!(word(&e, 0x20), 0x3333);
        assert_eq!(word(&e, 0x24), 2);
        assert_eq!(word(&e, 0x28), 3);
        for offset in [0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40, 0x44] {
            assert_eq!(word(&e, offset), 0, "offset {offset:#x}");
        }
        assert_eq!(e.mem.u8(this + 0x48), 0);
        assert_eq!(e.mem.u8(this + 0x49), 0);
        assert_eq!(e.mem.u16(this + 0x4a), 0xaaaa);
        assert_eq!(e.mem.u16(this + 0x4c), 0);
        assert_eq!(e.mem.u8(this + 0x4e), 0);
    }

    #[test]
    fn fn_00ead090_constructs_in_a_new_block_or_answers_null() {
        let mut e = engine();
        e.register(NI_COLLISION_OBJECT_CONSTRUCTOR, nothing);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        let made = e.call(0x00ea_d090, &args![]).u32();
        assert_ne!(made, 0);
        assert_eq!(e.mem.u32(made), 0x010f_18bc);
        assert_eq!(e.mem.u32(made + 0x24), 2);
        assert_eq!(logged(&e, OPERATOR_NEW), vec![vec![0x50]]);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00ea_d090, &args![]).u32(), 0);
    }

    #[test]
    fn the_destructor_body_frees_and_deletes_what_the_object_owns() {
        let mut e = engine();
        e.register(NI_COLLISION_OBJECT_DESTRUCTOR_BODY, nothing);
        e.register(0x0300_0001, nothing);
        e.register(0x0300_0002, nothing);
        e.register(0x0300_0003, nothing);
        let model = vtable_object(&mut e, 0x0200_0000, &[(8, 0x0300_0001)]);
        let world = vtable_object(&mut e, 0x0200_1000, &[(8, 0x0300_0002)]);
        let root = vtable_object(&mut e, 0x0200_2000, &[(0, 0x0300_0003)]);
        let this = collision_data(&mut e);
        e.set(this, NiCollisionData::m_pkWorldVertex, 0x7100);
        e.set(this, NiCollisionData::m_pkWorldNormal, 0x7200);
        e.set(this, NiCollisionData::m_pkModelABV, model);
        e.set(this, NiCollisionData::m_pkWorldABV, world);
        e.set(this, NiCollisionData::m_pkOBBRoot, root);
        e.call(0x00ea_cde0, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), 0x010f_18bc);
        assert_eq!(
            logged(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x7100], vec![MEMORY_MANAGER, 0x7200]]
        );
        assert_eq!(logged(&e, 0x0300_0001), vec![vec![model, 1]]);
        assert_eq!(logged(&e, 0x0300_0002), vec![vec![world, 1]]);
        assert_eq!(logged(&e, 0x0300_0003), vec![vec![root, 1]]);
        assert_eq!(e.get(this, NiCollisionData::m_pkOBBRoot), 0);
        assert_eq!(
            logged(&e, NI_COLLISION_OBJECT_DESTRUCTOR_BODY),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn the_destructor_body_skips_what_is_null() {
        let mut e = engine();
        e.register(NI_COLLISION_OBJECT_DESTRUCTOR_BODY, nothing);
        let this = collision_data(&mut e);
        e.call(0x00ea_cde0, &args![this]);
        // The call itself, two deallocations and the base destructor body.
        assert_eq!(e.call_log.as_ref().unwrap().len(), 4);
        assert_eq!(logged(&e, MEMORY_DEALLOCATE).len(), 2);
    }

    #[test]
    fn set_model_space_abv_replaces_the_volumes() {
        let mut e = engine();
        e.register(0x0300_0001, nothing);
        e.register(0x0300_0002, nothing);
        e.register(0x0300_0003, |e, _| Ret {
            eax: e.mem.u32(0x0600_0000),
            ..Ret::default()
        });
        e.register(0x0300_0004, nothing);
        let old_model = vtable_object(&mut e, 0x0200_0000, &[(8, 0x0300_0001)]);
        let old_world = vtable_object(&mut e, 0x0200_1000, &[(8, 0x0300_0002)]);
        let world = vtable_object(&mut e, 0x0200_2000, &[(0x14, 0x0300_0004)]);
        let model = vtable_object(&mut e, 0x0200_3000, &[(0x18, 0x0300_0003)]);
        // `Create` answers the word at 06000000: the new world volume.
        e.map(0x0600_0000, 0x1000);
        e.mem.set_u32(0x0600_0000, world);
        let scene = e.mem.alloc(0x100);
        let this = collision_data(&mut e);
        e.set(this, NiCollisionData::m_pkSceneObject, scene);
        e.set(this, NiCollisionData::m_pkModelABV, old_model);
        e.set(this, NiCollisionData::m_pkWorldABV, old_world);
        e.call(0x00ea_ce50, &args![this, model]);
        assert_eq!(logged(&e, 0x0300_0001), vec![vec![old_model, 1]]);
        assert_eq!(logged(&e, 0x0300_0002), vec![vec![old_world, 1]]);
        assert_eq!(e.get(this, NiCollisionData::m_pkModelABV), model);
        assert_eq!(e.get(this, NiCollisionData::m_pkWorldABV), world);
        assert_eq!(
            logged(&e, 0x0300_0004),
            vec![vec![world, model, scene + 0x68]]
        );
    }

    #[test]
    fn set_model_space_abv_without_a_scene_object_does_not_update() {
        let mut e = engine();
        e.register(0x0300_0003, |_, _| Ret {
            eax: 0x0600_0000,
            ..Ret::default()
        });
        let model = vtable_object(&mut e, 0x0200_3000, &[(0x18, 0x0300_0003)]);
        let this = collision_data(&mut e);
        e.call(0x00ea_ce50, &args![this, model]);
        assert_eq!(e.get(this, NiCollisionData::m_pkWorldABV), 0x0600_0000);
        // The call itself and the `Create`, no update.
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn set_model_space_abv_with_null_clears_the_world_volume() {
        let mut e = engine();
        e.register(0x0300_0001, nothing);
        let old_world = vtable_object(&mut e, 0x0200_1000, &[(8, 0x0300_0001)]);
        let this = collision_data(&mut e);
        e.set(this, NiCollisionData::m_pkWorldABV, old_world);
        e.call(0x00ea_ce50, &args![this, 0u32]);
        assert_eq!(logged(&e, 0x0300_0001), vec![vec![old_world, 1]]);
        assert_eq!(e.get(this, NiCollisionData::m_pkModelABV), 0);
        assert_eq!(e.get(this, NiCollisionData::m_pkWorldABV), 0);
    }

    #[test]
    fn initialize_sets_the_scene_object_the_mode_and_a_volume_from_the_stream() {
        let mut e = engine();
        e.register(0x0300_0001, nothing);
        e.register(0x0300_0002, |e, _| Ret {
            eax: e.mem.u32(0x0600_0000),
            ..Ret::default()
        });
        e.register(NI_BOUNDING_VOLUME_CREATE_FROM_STREAM, |e, _| Ret {
            eax: e.mem.u32(0x0600_0004),
            ..Ret::default()
        });
        let world = vtable_object(&mut e, 0x0200_6000, &[(0x14, 0x0300_0001)]);
        let volume = vtable_object(&mut e, 0x0200_5000, &[(0x18, 0x0300_0002)]);
        e.map(0x0600_0000, 0x1000);
        e.mem.set_u32(0x0600_0000, world);
        e.mem.set_u32(0x0600_0004, volume);
        let this = collision_data(&mut e);
        let this_vtable = 0x0200_0000;
        e.map(this_vtable, 0x200);
        e.mem.set_u32(this_vtable + 0x8c, 0x0300_0001);
        e.mem.set_u32(this.addr(), this_vtable);
        let init = e.mem.alloc(16);
        e.mem.set_u32(init, 0x4400);
        e.mem.set_u32(init + 4, 3);
        e.mem.set_u32(init + 8, 1);
        e.mem.set_u32(init + 12, 0x5500);
        e.call(0x00ea_ceb0, &args![this, init]);
        assert_eq!(logged(&e, 0x0300_0001)[0], vec![this.addr(), 0x4400]);
        assert_eq!(e.get(this, NiCollisionData::m_ePropagationMode), 3);
        assert_eq!(
            logged(&e, NI_BOUNDING_VOLUME_CREATE_FROM_STREAM),
            vec![vec![0x5500]]
        );
        assert_eq!(e.get(this, NiCollisionData::m_pkModelABV), volume);
        assert_eq!(e.get(this, NiCollisionData::m_pkWorldABV), world);
    }

    #[test]
    fn initialize_without_the_flag_makes_no_volume() {
        let mut e = engine();
        e.register(0x0300_0001, nothing);
        let this = collision_data(&mut e);
        e.map(0x0200_0000, 0x200);
        e.mem.set_u32(0x0200_0000 + 0x8c, 0x0300_0001);
        e.mem.set_u32(this.addr(), 0x0200_0000);
        let init = e.mem.alloc(16);
        e.mem.set_u32(init + 4, 1);
        e.call(0x00ea_ceb0, &args![this, init]);
        assert_eq!(e.get(this, NiCollisionData::m_ePropagationMode), 1);
        assert!(logged(&e, NI_BOUNDING_VOLUME_CREATE_FROM_STREAM).is_empty());
    }

    fn text(e: &mut Engine, string: &str) -> u32 {
        let address = e.mem.alloc(string.len() as u32 + 1);
        e.mem.set_cstr(address, string.as_bytes());
        address
    }

    fn formatting_engine() -> Engine {
        let mut e = engine();
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(NI_SPRINTF, nothing);
        e
    }

    #[test]
    fn get_viewer_string_picks_the_format_by_collision_mode() {
        let mut e = formatting_engine();
        let name = text(&mut e, "m_eCollisionMode");
        for (mode, format) in [
            (0u32, 0x010f_19a0u32),
            (1, 0x010f_1990),
            (2, 0x010f_1980),
            (3, 0x010f_1974),
            (4, 0x010f_1960),
            (5, 0x0109_b2ec),
            (0xffff_ffff, 0x0109_b2ec),
        ] {
            e.call_log = Some(vec![]);
            let block = e.call(0x00ea_cef0, &args![name, mode]).u32();
            assert_eq!(logged(&e, ALLOCATE), vec![vec![16 + 0x11]]);
            assert_eq!(
                logged(&e, NI_SPRINTF),
                vec![vec![block, 16 + 0x11, format, name]]
            );
        }
    }

    #[test]
    fn get_viewer_string_ov2_picks_the_format_by_propagation_mode() {
        let mut e = formatting_engine();
        let name = text(&mut e, "m_ePropagationMode");
        for (mode, format) in [
            (0u32, 0x010f_19fcu32),
            (1, 0x010f_19e0),
            (2, 0x010f_19c8),
            (3, 0x010f_19b0),
            (4, 0x0109_b2ec),
        ] {
            e.call_log = Some(vec![]);
            let block = e.call(0x00ea_cfd0, &args![name, mode]).u32();
            assert_eq!(logged(&e, ALLOCATE), vec![vec![18 + 0x20]]);
            assert_eq!(
                logged(&e, NI_SPRINTF),
                vec![vec![block, 18 + 0x20, format, name]]
            );
        }
    }

    #[test]
    fn fn_00ead0d0_finds_the_first_ancestor_with_a_collision_object() {
        let mut e = engine();
        let node = |e: &mut Engine, parent: u32, collision: u32| {
            let object = e.mem.alloc(0x40);
            e.mem.set_u32(object + 0x18, parent);
            e.mem.set_u32(object + 0x1c, collision);
            object
        };
        let root = node(&mut e, 0, 0);
        let owner = node(&mut e, root, 0x77);
        let middle = node(&mut e, owner, 0);
        let leaf = node(&mut e, middle, 0x55);
        assert_eq!(e.call(0x00ea_d0d0, &args![0x5000u32, leaf]).u32(), owner);
        assert_eq!(e.call(0x00ea_d0d0, &args![0x5000u32, owner]).u32(), 0);
        assert_eq!(e.call(0x00ea_d0d0, &args![0x5000u32, 0u32]).u32(), 0);
    }

    /// A collision data with a scene object whose parent is `parent` (+0x18), a transform
    /// double that doubles the local velocity, and the compute flag set to `flag`.
    fn velocity_fixture(parent: u32, flag: u8) -> (Engine, Ptr<NiCollisionData>, u32) {
        let mut e = engine();
        e.register(TRANSFORM_POINT, |e, a| {
            for axis in 0..3 {
                let value = e.mem.f32(a[2] + 4 * axis) * 2.0;
                e.mem.set_f32(a[1] + 4 * axis, value);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.mem.set_u8(COMPUTE_WORLD_VELOCITY, flag);
        let scene = e.mem.alloc(0x100);
        e.mem.set_u32(scene + 0x18, parent);
        let this = collision_data(&mut e);
        e.set(this, NiCollisionData::m_pkSceneObject, scene);
        for (axis, value) in [1.0f32, 2.0, 4.0].iter().enumerate() {
            e.mem.set_f32(this.addr() + 0xc + 4 * axis as u32, *value);
        }
        (e, this, scene)
    }

    fn world_velocity(e: &Engine, this: Ptr<NiCollisionData>) -> [f32; 3] {
        [
            f32::from_bits(e.get(this, NiCollisionData::m_kWorldVelocity_x)),
            f32::from_bits(e.get(this, NiCollisionData::m_kWorldVelocity_y)),
            f32::from_bits(e.get(this, NiCollisionData::m_kWorldVelocity_z)),
        ]
    }

    #[test]
    fn update_world_data_updates_the_world_volume_from_the_model_volume() {
        let (mut e, this, scene) = velocity_fixture(0, 0);
        e.register(0x0300_0001, nothing);
        let world = vtable_object(&mut e, 0x0200_0000, &[(0x14, 0x0300_0001)]);
        e.set(this, NiCollisionData::m_pkModelABV, 0x4000);
        e.set(this, NiCollisionData::m_pkWorldABV, world);
        e.call(0x00ea_d0f0, &args![this, 0x3f80_0000u32]);
        assert_eq!(
            logged(&e, 0x0300_0001),
            vec![vec![world, 0x4000, scene + 0x68]]
        );
        assert_eq!(e.get(this, NiCollisionData::m_bWorldNormalsNeedUpdate), 1);
        assert_eq!(e.get(this, NiCollisionData::m_bWorldVerticesNeedUpdate), 1);
    }

    #[test]
    fn update_world_data_leaves_the_velocity_when_the_flag_is_clear() {
        let (mut e, this, _) = velocity_fixture(0, 0);
        e.call(0x00ea_d0f0, &args![this, 0u32]);
        assert_eq!(world_velocity(&e, this), [0.0; 3]);
        assert_eq!(e.get(this, NiCollisionData::m_bWorldVerticesNeedUpdate), 1);
        assert_eq!(e.get(this, NiCollisionData::m_bWorldNormalsNeedUpdate), 1);
    }

    #[test]
    fn update_world_data_without_a_parent_copies_the_local_velocity() {
        let (mut e, this, _) = velocity_fixture(0, 1);
        e.call(0x00ea_d0f0, &args![this, 0u32]);
        assert_eq!(world_velocity(&e, this), [1.0, 2.0, 4.0]);
        assert!(logged(&e, TRANSFORM_POINT).is_empty());
    }

    #[test]
    fn update_world_data_adds_the_parents_world_velocity() {
        let (mut e, this, scene) = velocity_fixture(0, 1);
        let ancestor_data = e.mem.alloc(0x50);
        for (axis, value) in [0.5f32, 0.25, 8.0].iter().enumerate() {
            e.mem
                .set_f32(ancestor_data + 0x18 + 4 * axis as u32, *value);
        }
        let parent = e.mem.alloc(0x100);
        e.mem.set_u32(parent + 0x1c, ancestor_data);
        e.mem.set_u32(scene + 0x18, parent);
        e.call(0x00ea_d0f0, &args![this, 0u32]);
        assert_eq!(world_velocity(&e, this), [2.5, 4.25, 16.0]);
        let calls = logged(&e, TRANSFORM_POINT);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], parent + 0x68);
        assert_eq!(calls[0][2], this.addr() + 0xc);
    }

    #[test]
    fn update_world_data_searches_the_ancestors_when_the_parent_has_no_collision_object() {
        let (mut e, this, scene) = velocity_fixture(0, 1);
        let ancestor_data = e.mem.alloc(0x50);
        for axis in 0..3 {
            e.mem.set_f32(ancestor_data + 0x18 + 4 * axis, 1.0);
        }
        let grandparent = e.mem.alloc(0x100);
        e.mem.set_u32(grandparent + 0x1c, ancestor_data);
        let parent = e.mem.alloc(0x100);
        e.mem.set_u32(parent + 0x18, grandparent);
        e.mem.set_u32(scene + 0x18, parent);
        e.call(0x00ea_d0f0, &args![this, 0u32]);
        assert_eq!(world_velocity(&e, this), [3.0, 5.0, 9.0]);
        // The transform used is still the parent's own.
        assert_eq!(logged(&e, TRANSFORM_POINT)[0][0], parent + 0x68);
    }

    #[test]
    fn update_world_data_without_any_collision_ancestor_uses_the_transformed_velocity() {
        let (mut e, this, scene) = velocity_fixture(0, 1);
        let parent = e.mem.alloc(0x100);
        e.mem.set_u32(scene + 0x18, parent);
        e.call(0x00ea_d0f0, &args![this, 0u32]);
        assert_eq!(world_velocity(&e, this), [2.0, 4.0, 8.0]);
    }

    #[test]
    fn update_world_data_adds_with_the_x87_choice_of_nan() {
        let (mut e, this, scene) = velocity_fixture(0, 1);
        let ancestor_data = e.mem.alloc(0x50);
        // A signalling NaN in the ancestor's x: FLD quiets it, the sum keeps it quiet.
        e.mem.set_u32(ancestor_data + 0x18, 0x7f80_0001);
        let parent = e.mem.alloc(0x100);
        e.mem.set_u32(parent + 0x1c, ancestor_data);
        e.mem.set_u32(scene + 0x18, parent);
        e.call(0x00ea_d0f0, &args![this, 0u32]);
        assert_eq!(
            e.get(this, NiCollisionData::m_kWorldVelocity_x),
            0x7fc0_0001
        );
    }

    /// A scene object that is a geometry: its slots 0x18 and 0x1c answer itself, and it carries
    /// (+0xb8) a geometry data with `vertices` vertices and `triangles` triangles. Answers (scene
    /// object, geometry (the same object), data).
    fn geometry_scene(e: &mut Engine, vertices: u16, triangles: u16) -> (u32, u32, u32) {
        e.register(0x0300_0018, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        let data = e.mem.alloc(0x100);
        e.mem.set_u16(data + 8, vertices);
        e.mem.set_u16(data + 0x40, triangles);
        let scene = vtable_object(e, 0x0200_0000, &[(0x18, 0x0300_0018), (0x1c, 0x0300_0018)]);
        e.mem.set_u32(scene + 0xb8, data);
        (scene, scene, data)
    }

    #[test]
    fn fn_00ead1e0_allocates_the_world_vertices() {
        let mut e = engine();
        let (scene, _, _) = geometry_scene(&mut e, 5, 3);
        let this = collision_data(&mut e);
        e.set(this, NiCollisionData::m_pkSceneObject, scene);
        e.call(0x00ea_d1e0, &args![this]);
        assert_eq!(e.get(this, NiCollisionData::m_usNumVertices), 5);
        assert_eq!(e.get(this, NiCollisionData::m_usNumTriangles), 3);
        assert_eq!(e.get(this, NiCollisionData::m_bWorldVerticesNeedUpdate), 1);
        assert_ne!(e.get(this, NiCollisionData::m_pkWorldVertex), 0);
        assert_eq!(logged(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 60]]);
    }

    #[test]
    fn fn_00ead1e0_does_nothing_when_a_condition_fails() {
        let mut e = engine();
        let this = collision_data(&mut e);
        // No scene object.
        e.call(0x00ea_d1e0, &args![this]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        // The scene object is not a triangle-based geometry.
        e.register(0x0300_0001, nothing);
        let scene = vtable_object(&mut e, 0x0200_8000, &[(0x1c, 0x0300_0001)]);
        e.set(this, NiCollisionData::m_pkSceneObject, scene);
        e.call(0x00ea_d1e0, &args![this]);
        assert!(logged(&e, MEMORY_ALLOCATE).is_empty());
        // Already has world vertices.
        let (scene, _, _) = geometry_scene(&mut e, 5, 3);
        e.set(this, NiCollisionData::m_pkSceneObject, scene);
        e.set(this, NiCollisionData::m_pkWorldVertex, 0x9000);
        e.call(0x00ea_d1e0, &args![this]);
        assert!(logged(&e, MEMORY_ALLOCATE).is_empty());
        // No vertices.
        let mut e = engine();
        let (scene, _, _) = geometry_scene(&mut e, 0, 3);
        let this = collision_data(&mut e);
        e.set(this, NiCollisionData::m_pkSceneObject, scene);
        e.call(0x00ea_d1e0, &args![this]);
        assert!(logged(&e, MEMORY_ALLOCATE).is_empty());
        assert_eq!(e.get(this, NiCollisionData::m_usNumVertices), 0);
    }

    /// The function pointer `011aa050` (a double that records its call), the geometry data's
    /// slot 0x90 answering `count`.
    fn refresh_fixture(count: u16) -> (Engine, Ptr<NiCollisionData>, u32, u32) {
        let mut e = engine();
        e.register(0x0300_0001, nothing);
        e.register_double(0x0300_0002, move |_, _| Ret {
            eax: 0xabcd_0000 | count as u32,
            ..Ret::default()
        });
        e.mem.set_u32(TRANSFORM_POINTS_POINTER, 0x0300_0001);
        let data = vtable_object(&mut e, 0x0200_9000, &[(0x90, 0x0300_0002)]);
        e.mem.set_u32(data + 0x20, 0x6600);
        let (scene, geometry, _) = geometry_scene(&mut e, 1, 1);
        e.mem.set_u32(geometry + 0xb8, data);
        let this = collision_data(&mut e);
        e.set(this, NiCollisionData::m_pkSceneObject, scene);
        e.set(this, NiCollisionData::m_pkWorldVertex, 0x7700);
        e.set(this, NiCollisionData::m_bWorldVerticesNeedUpdate, 1);
        (e, this, geometry, data)
    }

    #[test]
    fn fn_00ead250_transforms_the_vertices_and_clears_the_flag() {
        let (mut e, this, geometry, _) = refresh_fixture(9);
        e.call(0x00ea_d250, &args![this]);
        assert_eq!(
            logged(&e, 0x0300_0001),
            vec![vec![9, 0x6600, 0x7700, geometry + 0x68]]
        );
        assert_eq!(e.get(this, NiCollisionData::m_bWorldVerticesNeedUpdate), 0);
    }

    #[test]
    fn fn_00ead250_does_nothing_without_the_flag_or_the_array() {
        let (mut e, this, _, _) = refresh_fixture(9);
        e.set(this, NiCollisionData::m_bWorldVerticesNeedUpdate, 0);
        e.call(0x00ea_d250, &args![this]);
        assert!(logged(&e, 0x0300_0001).is_empty());
        e.set(this, NiCollisionData::m_bWorldVerticesNeedUpdate, 1);
        e.set(this, NiCollisionData::m_pkWorldVertex, 0);
        e.call(0x00ea_d250, &args![this]);
        assert!(logged(&e, 0x0300_0001).is_empty());
        e.set(this, NiCollisionData::m_pkSceneObject, 0);
        e.call(0x00ea_d250, &args![this]);
        assert!(logged(&e, 0x0300_0001).is_empty());
    }

    /// Two collision data objects sharing the class record `0126fab4` in the class chain of
    /// their `GetRTTI` (a fake record whose parent is the real one, or the real one itself).
    fn equality_fixture() -> (Engine, Ptr<NiCollisionData>, Ptr<NiCollisionData>) {
        let mut e = engine();
        e.register(NI_COLLISION_OBJECT_IS_EQUAL, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(0x0300_0001, |_, _| Ret {
            eax: 0x0500_0000,
            ..Ret::default()
        });
        e.register(0x0300_0002, |_, _| Ret {
            eax: 0x0500_0100,
            ..Ret::default()
        });
        e.map(0x0500_0000, 0x1000);
        e.mem.set_u32(0x0500_0000 + 4, NI_COLLISION_DATA_RTTI);
        e.mem.set_u32(0x0500_0100 + 4, 0x0500_0200);
        e.mem.set_u32(NI_COLLISION_DATA_RTTI + 4, 0);
        let this = e.new_object::<NiCollisionData>();
        let other = e.new_object::<NiCollisionData>();
        e.mem.set_u32(other.addr(), 0x0200_a000);
        e.map(0x0200_a000, 0x200);
        e.mem.set_u32(0x0200_a000 + 8, 0x0300_0001);
        (e, this, other)
    }

    #[test]
    fn is_equal_compares_the_modes_and_the_volume_types() {
        let (mut e, this, other) = equality_fixture();
        assert!(e.call(0x00ea_d2b0, &args![this, other]).bool());
        e.set(other, NiCollisionData::m_eCollisionMode, 2);
        assert!(!e.call(0x00ea_d2b0, &args![this, other]).bool());
        e.set(this, NiCollisionData::m_eCollisionMode, 2);
        e.set(other, NiCollisionData::m_ePropagationMode, 1);
        assert!(!e.call(0x00ea_d2b0, &args![this, other]).bool());
        e.set(this, NiCollisionData::m_ePropagationMode, 1);
        assert!(e.call(0x00ea_d2b0, &args![this, other]).bool());
        // Both have a model volume: their `Type` (slot 0xc) must agree.
        e.register(0x0300_0003, |_, _| Ret {
            eax: 4,
            ..Ret::default()
        });
        e.register(0x0300_0004, |_, _| Ret {
            eax: 4,
            ..Ret::default()
        });
        e.register(0x0300_0005, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        let same_a = vtable_object(&mut e, 0x0200_b000, &[(0xc, 0x0300_0003)]);
        let same_b = vtable_object(&mut e, 0x0200_c000, &[(0xc, 0x0300_0004)]);
        let different = vtable_object(&mut e, 0x0200_d000, &[(0xc, 0x0300_0005)]);
        e.set(other, NiCollisionData::m_pkModelABV, same_a);
        e.set(this, NiCollisionData::m_pkModelABV, same_b);
        assert!(e.call(0x00ea_d2b0, &args![this, other]).bool());
        e.set(this, NiCollisionData::m_pkModelABV, different);
        assert!(!e.call(0x00ea_d2b0, &args![this, other]).bool());
        // One without a volume: equal only when the pointers are.
        e.set(this, NiCollisionData::m_pkModelABV, 0);
        assert!(!e.call(0x00ea_d2b0, &args![this, other]).bool());
        e.set(other, NiCollisionData::m_pkModelABV, 0);
        assert!(e.call(0x00ea_d2b0, &args![this, other]).bool());
        e.set(this, NiCollisionData::m_pkModelABV, same_a);
        assert!(!e.call(0x00ea_d2b0, &args![this, other]).bool());
    }

    #[test]
    fn is_equal_needs_the_base_the_object_and_the_class() {
        let (mut e, this, other) = equality_fixture();
        assert!(!e.call(0x00ea_d2b0, &args![this, 0u32]).bool());
        // The class chain of the other object (a record that never reaches this class).
        e.mem.set_u32(0x0200_a000 + 8, 0x0300_0002);
        e.map(0x0500_0200, 0x100);
        assert!(!e.call(0x00ea_d2b0, &args![this, other]).bool());
        // The base comparison fails.
        e.mem.set_u32(0x0200_a000 + 8, 0x0300_0001);
        e.register(NI_COLLISION_OBJECT_IS_EQUAL, |_, _| Ret::default());
        assert!(!e.call(0x00ea_d2b0, &args![this, other]).bool());
        assert_eq!(
            logged(&e, NI_COLLISION_OBJECT_IS_EQUAL)[0],
            vec![this.addr(), 0]
        );
    }

    #[test]
    fn set_scene_graph_object_marks_the_geometry_data_for_collision_modes_0_and_1() {
        for (mode, expected) in [(0u32, 0x37u8), (1, 0x37), (2, 0x04), (3, 0x04)] {
            let mut e = engine();
            e.register(NI_COLLISION_OBJECT_SET_SCENE_GRAPH_OBJECT, |e, a| {
                e.mem.set_u32(a[0] + 8, a[1]);
                Ret::default()
            });
            let (scene, _, data) = geometry_scene(&mut e, 1, 1);
            e.mem.set_u8(data + 0x38, 0x04);
            let this = collision_data(&mut e);
            e.set(this, NiCollisionData::m_eCollisionMode, mode);
            e.call(0x00ea_d330, &args![this, scene]);
            assert_eq!(e.mem.u8(data + 0x38), expected, "mode {mode}");
            assert_eq!(
                logged(&e, NI_COLLISION_OBJECT_SET_SCENE_GRAPH_OBJECT),
                vec![vec![this.addr(), scene]]
            );
        }
    }

    #[test]
    fn set_scene_graph_object_skips_a_non_geometry_and_a_null_scene() {
        let mut e = engine();
        e.register(NI_COLLISION_OBJECT_SET_SCENE_GRAPH_OBJECT, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(0x0300_0001, nothing);
        let scene = vtable_object(&mut e, 0x0200_8000, &[(0x18, 0x0300_0001)]);
        let this = collision_data(&mut e);
        e.call(0x00ea_d330, &args![this, 0u32]);
        e.call(0x00ea_d330, &args![this, scene]);
        assert_eq!(logged(&e, 0x0300_0001), vec![vec![scene]]);
        // A geometry without a data.
        let (scene, geometry, _) = geometry_scene(&mut e, 1, 1);
        e.mem.set_u32(geometry + 0xb8, 0);
        e.call(0x00ea_d330, &args![this, scene]);
    }

    /// The class record chains the OBB root builder searches: a record whose parent is the
    /// record at `011f4a40`, one whose parent is the record at `011f4a20`, and one reaching
    /// neither.
    const CHAIN_TO_TRI_LIST: u32 = 0x0500_0000;
    const CHAIN_TO_INDEXED: u32 = 0x0500_0100;
    const CHAIN_TO_NOTHING: u32 = 0x0500_0200;

    /// A scene object whose `GetRTTI` (slot 8) answers `chain`, whose slots 0x18 and 0x1c answer
    /// null, and whose geometry data (+0xb8) has 0x6600 as its vertices (+0x20) and `triangles`
    /// triangles (+0x40); a collision data holding it and already having world vertices.
    fn obb_fixture(chain: u32, triangles: u16) -> (Engine, Ptr<NiCollisionData>, u32) {
        let mut e = engine();
        e.map(0x0500_0000, 0x1000);
        e.mem
            .set_u32(CHAIN_TO_TRI_LIST + 4, RTTI_TRIANGLES_TRI_LIST);
        e.mem.set_u32(CHAIN_TO_INDEXED + 4, RTTI_TRIANGLES_INDEXED);
        e.mem.set_u32(CHAIN_TO_NOTHING + 4, 0x0500_0300);
        e.register(0x0300_0001, |e, a| Ret {
            eax: e.mem.u32(a[0] + 0x84),
            ..Ret::default()
        });
        e.register(0x0300_0002, |_, _| Ret::default());
        e.register(NI_OBB_ROOT_CONSTRUCTOR, |_, _| Ret {
            eax: 0x8888,
            ..Ret::default()
        });
        e.register(NI_TRI_SHAPE_DATA_GET_TRI_LIST, |_, _| Ret {
            eax: 0x4321,
            ..Ret::default()
        });
        e.register(ALLOCATE, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        let scene = vtable_object(
            &mut e,
            0x0200_0000,
            &[(8, 0x0300_0001), (0x18, 0x0300_0002), (0x1c, 0x0300_0002)],
        );
        e.mem.set_u32(scene + 0x84, chain);
        let data = e.mem.alloc(0x100);
        e.mem.set_u32(data + 0x20, 0x6600);
        e.mem.set_u16(data + 0x40, triangles);
        e.mem.set_u32(scene + 0xb8, data);
        let this = collision_data(&mut e);
        e.set(this, NiCollisionData::m_pkSceneObject, scene);
        e.set(this, NiCollisionData::m_pkWorldVertex, 0x7700);
        (e, this, data)
    }

    #[test]
    fn fn_00ead380_builds_the_root_from_the_triangle_list() {
        let (mut e, this, _) = obb_fixture(CHAIN_TO_TRI_LIST, 3);
        e.call(0x00ea_d380, &args![this, 0x55u32]);
        assert_eq!(
            logged(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x54]]
        );
        let calls = logged(&e, NI_OBB_ROOT_CONSTRUCTOR);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][1..], [3, 0x4321, 0x6600, 0x7700, 0x55]);
        assert_eq!(e.get(this, NiCollisionData::m_pkOBBRoot), 0x8888);
    }

    #[test]
    fn fn_00ead380_makes_the_world_vertices_first_when_there_are_none() {
        let (mut e, this, _) = obb_fixture(CHAIN_TO_TRI_LIST, 3);
        e.set(this, NiCollisionData::m_pkWorldVertex, 0);
        e.call(0x00ea_d380, &args![this, 0x55u32]);
        // `IsTriBasedGeom` (slot 0x1c) then `IsGeometry` (slot 0x18), both answering null.
        assert_eq!(logged(&e, 0x0300_0002).len(), 2);
        assert_eq!(logged(&e, NI_OBB_ROOT_CONSTRUCTOR)[0][4], 0);
    }

    #[test]
    fn fn_00ead380_leaves_a_failed_allocation_as_a_null_root() {
        let (mut e, this, _) = obb_fixture(CHAIN_TO_TRI_LIST, 3);
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        e.call(0x00ea_d380, &args![this, 0x55u32]);
        assert!(logged(&e, NI_OBB_ROOT_CONSTRUCTOR).is_empty());
        assert_eq!(e.get(this, NiCollisionData::m_pkOBBRoot), 0);
    }

    #[test]
    fn fn_00ead380_does_nothing_with_a_root_without_a_scene_or_with_another_class() {
        let (mut e, this, _) = obb_fixture(CHAIN_TO_TRI_LIST, 3);
        e.set(this, NiCollisionData::m_pkOBBRoot, 0x9999);
        e.call(0x00ea_d380, &args![this, 0x55u32]);
        assert!(logged(&e, MEMORY_ALLOCATE).is_empty());
        let (mut e, this, _) = obb_fixture(CHAIN_TO_INDEXED, 3);
        e.set(this, NiCollisionData::m_pkOBBRoot, 0x9999);
        e.call(0x00ea_d380, &args![this, 0x55u32]);
        assert!(logged(&e, MEMORY_ALLOCATE).is_empty());
        assert!(logged(&e, ALLOCATE).is_empty());
        let (mut e, this, _) = obb_fixture(CHAIN_TO_NOTHING, 3);
        e.call(0x00ea_d380, &args![this, 0x55u32]);
        assert!(logged(&e, MEMORY_ALLOCATE).is_empty());
        assert_eq!(e.get(this, NiCollisionData::m_pkOBBRoot), 0);
        e.set(this, NiCollisionData::m_pkSceneObject, 0);
        e.call(0x00ea_d380, &args![this, 0x55u32]);
        assert!(logged(&e, MEMORY_ALLOCATE).is_empty());
    }

    /// The data's slot 0xac double writes the next of `triples` through its three pointers.
    fn index_fixture(
        triples: Vec<[u16; 3]>,
    ) -> (
        Engine,
        Ptr<NiCollisionData>,
        std::rc::Rc<std::cell::RefCell<Vec<u16>>>,
    ) {
        let (mut e, this, data) = obb_fixture(CHAIN_TO_INDEXED, triples.len() as u16);
        let triangles = triples.len() as u16;
        let _ = triangles;
        e.register_double(0x0300_0003, move |e, a| {
            for (slot, pointer) in a[2..5].iter().enumerate() {
                e.mem.set_u16(*pointer, triples[a[1] as usize][slot]);
            }
            Ret::default()
        });
        let data_object = vtable_object(&mut e, 0x0200_1000, &[(0xac, 0x0300_0003)]);
        e.mem.set_u32(data_object + 0x20, 0x6600);
        e.mem.set_u16(data_object + 0x40, e.mem.u16(data + 0x40));
        let scene = e.get(this, NiCollisionData::m_pkSceneObject);
        e.mem.set_u32(scene + 0xb8, data_object);
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let record = seen.clone();
        e.register_double(NI_OBB_ROOT_CONSTRUCTOR, move |e, a| {
            for index in 0..a[1] * 3 {
                record.borrow_mut().push(e.mem.u16(a[2] + 2 * index));
            }
            Ret {
                eax: 0x8888,
                ..Ret::default()
            }
        });
        (e, this, seen)
    }

    #[test]
    fn fn_00ead380_keeps_only_the_triangles_with_three_different_indices() {
        let triples = vec![[0, 1, 2], [3, 3, 4], [5, 6, 7], [8, 9, 8], [1, 2, 2]];
        let (mut e, this, seen) = index_fixture(triples);
        e.call(0x00ea_d380, &args![this, 0x55u32]);
        assert_eq!(logged(&e, ALLOCATE), vec![vec![5 * 6]]);
        let calls = logged(&e, NI_OBB_ROOT_CONSTRUCTOR);
        assert_eq!(calls.len(), 1);
        let list = calls[0][2];
        assert_eq!(calls[0][1], 2);
        assert_eq!(calls[0][3..], [0x6600, 0x7700, 0x55]);
        assert_eq!(*seen.borrow(), vec![0, 1, 2, 5, 6, 7]);
        assert_eq!(e.get(this, NiCollisionData::m_pkOBBRoot), 0x8888);
        assert_eq!(
            logged(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, list]]
        );
        assert_eq!(
            logged(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x54]]
        );
    }

    #[test]
    fn fn_00ead380_builds_an_empty_root_for_no_triangles_and_frees_the_list() {
        let (mut e, this, _) = index_fixture(vec![]);
        e.call(0x00ea_d380, &args![this, 0x55u32]);
        assert_eq!(logged(&e, ALLOCATE), vec![vec![0]]);
        assert_eq!(logged(&e, NI_OBB_ROOT_CONSTRUCTOR)[0][1], 0);
        assert_eq!(logged(&e, MEMORY_DEALLOCATE).len(), 1);
    }

    #[test]
    fn fn_00ead380_frees_the_list_even_when_the_root_block_fails() {
        let (mut e, this, _) = index_fixture(vec![[1, 2, 3]]);
        e.register(MEMORY_ALLOCATE, |_, _| Ret::default());
        e.call(0x00ea_d380, &args![this, 0x55u32]);
        assert_eq!(e.get(this, NiCollisionData::m_pkOBBRoot), 0);
        assert_eq!(logged(&e, MEMORY_DEALLOCATE).len(), 1);
    }

    #[test]
    fn convert_rewrites_the_modes_of_old_files() {
        // (old propagation mode, model volume?, new propagation mode, new collision mode,
        //  scene object asked, data flags set)
        for (old, volume, new_mode, collision, asked, flagged) in [
            (0u32, false, 3u32, 0u32, 1, true),
            (0, true, 3, 2, 1, false),
            (1, false, 3, 9, 0, false),
            (1, true, 3, 9, 0, false),
            (2, false, 0, 0, 1, true),
            (2, true, 0, 2, 1, false),
            (3, false, 2, 0, 1, true),
            (3, true, 2, 2, 1, false),
            (4, false, 2, 9, 0, false),
            (7, true, 7, 9, 0, false),
        ] {
            let mut e = engine();
            let (scene, _, data) = geometry_scene(&mut e, 1, 1);
            let this = collision_data(&mut e);
            e.set(this, NiCollisionData::m_pkSceneObject, scene);
            e.set(this, NiCollisionData::m_ePropagationMode, old);
            e.set(this, NiCollisionData::m_eCollisionMode, 9);
            e.set(
                this,
                NiCollisionData::m_pkModelABV,
                if volume { 0x4000 } else { 0 },
            );
            e.call(0x00ea_d550, &args![this, 0x0a00_0105u32, 0u32]);
            let label = format!("old mode {old}, volume {volume}");
            assert_eq!(
                e.get(this, NiCollisionData::m_ePropagationMode),
                new_mode,
                "{label}"
            );
            assert_eq!(
                e.get(this, NiCollisionData::m_eCollisionMode),
                collision,
                "{label}"
            );
            assert_eq!(logged(&e, 0x0300_0018).len(), asked, "{label}");
            assert_eq!(e.mem.u8(data + 0x38) == 0x33, flagged, "{label}");
        }
    }

    #[test]
    fn convert_leaves_files_of_the_new_version_alone() {
        let mut e = engine();
        let this = collision_data(&mut e);
        e.set(this, NiCollisionData::m_ePropagationMode, 0);
        e.call(0x00ea_d550, &args![this, 0x0a00_0106u32, 0u32]);
        assert_eq!(e.get(this, NiCollisionData::m_ePropagationMode), 0);
    }

    #[test]
    fn fn_00ead610_copies_the_modes_and_clones_the_volume() {
        let mut e = engine();
        e.register(NI_OBJECT_COPY_MEMBERS, nothing);
        e.register(0x0300_0001, |e, _| Ret {
            eax: e.mem.u32(0x0700_0000),
            ..Ret::default()
        });
        e.register(0x0300_0002, |_, _| Ret {
            eax: 0x0700_0100,
            ..Ret::default()
        });
        e.map(0x0700_0000, 0x1000);
        let model = vtable_object(&mut e, 0x0200_0000, &[(0x1c, 0x0300_0001)]);
        // The clone's `Create` answers a world volume (never updated: the copy has no scene).
        let clone = vtable_object(&mut e, 0x0200_1000, &[(0x18, 0x0300_0002)]);
        e.mem.set_u32(0x0700_0000, clone);
        let this = collision_data(&mut e);
        let dest = collision_data(&mut e);
        e.set(this, NiCollisionData::m_ePropagationMode, 1);
        e.set(this, NiCollisionData::m_eCollisionMode, 0);
        e.set(this, NiCollisionData::m_pkModelABV, model);
        e.call(0x00ea_d610, &args![this, dest, 0x1234u32]);
        assert_eq!(
            logged(&e, NI_OBJECT_COPY_MEMBERS),
            vec![vec![this.addr(), dest.addr(), 0x1234]]
        );
        assert_eq!(e.get(dest, NiCollisionData::m_ePropagationMode), 1);
        assert_eq!(e.get(dest, NiCollisionData::m_eCollisionMode), 0);
        assert_eq!(e.get(dest, NiCollisionData::m_pkModelABV), clone);
        assert_eq!(e.get(dest, NiCollisionData::m_pkWorldABV), 0x0700_0100);
    }

    #[test]
    fn fn_00ead610_makes_the_destinations_root_when_this_has_one() {
        let (mut e, this, _) = obb_fixture(CHAIN_TO_TRI_LIST, 3);
        e.register(NI_OBJECT_COPY_MEMBERS, nothing);
        e.set(this, NiCollisionData::m_pkOBBRoot, 0x9999);
        let dest = collision_data(&mut e);
        let scene = e.get(this, NiCollisionData::m_pkSceneObject);
        e.set(dest, NiCollisionData::m_pkSceneObject, scene);
        e.set(dest, NiCollisionData::m_pkWorldVertex, 0x7700);
        e.call(0x00ea_d610, &args![this, dest, 0u32]);
        let calls = logged(&e, NI_OBB_ROOT_CONSTRUCTOR);
        // The triangle list branch, with 1 as the last word.
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][1..], [3, 0x4321, 0x6600, 0x7700, 1]);
        assert_eq!(e.get(dest, NiCollisionData::m_pkOBBRoot), 0x8888);
    }

    #[test]
    fn fn_00ead610_makes_the_destinations_world_vertices_before_its_root() {
        let (mut e, this, _) = obb_fixture(CHAIN_TO_TRI_LIST, 3);
        e.register(NI_OBJECT_COPY_MEMBERS, nothing);
        e.set(this, NiCollisionData::m_pkOBBRoot, 0x9999);
        let dest = collision_data(&mut e);
        let scene = e.get(this, NiCollisionData::m_pkSceneObject);
        e.set(dest, NiCollisionData::m_pkSceneObject, scene);
        e.call(0x00ea_d610, &args![this, dest, 0u32]);
        // The geometry questions of both refreshes, in the copy and again in the root builder.
        assert_eq!(logged(&e, 0x0300_0002).len(), 4);
    }

    /// A stream object of the given file `version` whose input (+0x24c) and output (+0x250)
    /// objects are one object with the read function (+8) `0x03000010` and the write function
    /// (+0xc) `0x03000011`. Answers (stream, io object).
    fn stream_fixture(e: &mut Engine, version: u32) -> (u32, u32) {
        let stream = e.mem.alloc(0x300);
        let io = e.mem.alloc(0x20);
        e.mem.set_u32(stream + 0xd8, version);
        e.mem.set_u32(stream + 0x24c, io);
        e.mem.set_u32(stream + 0x250, io);
        e.mem.set_u32(io + 8, 0x0300_0010);
        e.mem.set_u32(io + 0xc, 0x0300_0011);
        (stream, io)
    }

    /// A read function double handing out `values` (each written as `size` bytes) and noting the
    /// size words it was given.
    fn reader(
        e: &mut Engine,
        values: Vec<u32>,
    ) -> std::rc::Rc<std::cell::RefCell<Vec<(u32, u32)>>> {
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let record = seen.clone();
        let mut queue = std::collections::VecDeque::from(values);
        e.register_double(0x0300_0010, move |e, a| {
            let value = queue.pop_front().unwrap();
            e.mem.write(a[1], &value.to_le_bytes()[..a[2] as usize]);
            record.borrow_mut().push((a[2], e.mem.u32(a[3])));
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        seen
    }

    fn loading_fixture(version: u32, values: Vec<u32>) -> (Engine, Ptr<NiCollisionData>, u32, u32) {
        let mut e = engine();
        e.register(NI_COLLISION_OBJECT_LOAD_BINARY, nothing);
        let (stream, io) = stream_fixture(&mut e, version);
        let seen = reader(&mut e, values);
        let this = collision_data(&mut e);
        e.set(this, NiCollisionData::m_ePropagationMode, 77);
        e.set(this, NiCollisionData::m_eCollisionMode, 88);
        let _ = seen;
        (e, this, stream, io)
    }

    #[test]
    fn load_binary_of_a_new_file_reads_two_modes_and_a_volume() {
        let (mut e, this, stream, io) = loading_fixture(0x0a00_0106, vec![3, 2, 1]);
        e.register(0x0300_0002, |e, _| Ret {
            eax: e.mem.u32(0x0600_0000),
            ..Ret::default()
        });
        e.register(NI_BOUNDING_VOLUME_CREATE_FROM_STREAM, |e, _| Ret {
            eax: e.mem.u32(0x0600_0004),
            ..Ret::default()
        });
        let world = 0x0600_1000;
        let volume = vtable_object(&mut e, 0x0200_0000, &[(0x18, 0x0300_0002)]);
        e.map(0x0600_0000, 0x2000);
        e.mem.set_u32(0x0600_0000, world);
        e.mem.set_u32(0x0600_0004, volume);
        e.call(0x00ea_d670, &args![this, stream]);
        assert_eq!(
            logged(&e, NI_COLLISION_OBJECT_LOAD_BINARY),
            vec![vec![this.addr(), stream]]
        );
        assert_eq!(e.get(this, NiCollisionData::m_ePropagationMode), 3);
        assert_eq!(e.get(this, NiCollisionData::m_eCollisionMode), 2);
        assert_eq!(e.get(this, NiCollisionData::m_pkModelABV), volume);
        assert_eq!(e.get(this, NiCollisionData::m_pkWorldABV), world);
        assert_eq!(logged(&e, 0x0300_0010).len(), 3);
        assert_eq!(logged(&e, 0x0300_0010)[0][0], io);
        // Byte counter: 4 + 4 + 1.
        assert_eq!(e.mem.u32(io + 4), 9);
        assert_eq!(
            logged(&e, NI_BOUNDING_VOLUME_CREATE_FROM_STREAM),
            vec![vec![stream]]
        );
    }

    #[test]
    fn load_binary_of_an_old_file_reads_one_mode_and_stops_without_the_flag() {
        let (mut e, this, stream, io) = loading_fixture(0x0a00_0105, vec![4, 0]);
        e.call(0x00ea_d670, &args![this, stream]);
        assert_eq!(e.get(this, NiCollisionData::m_ePropagationMode), 4);
        assert_eq!(e.get(this, NiCollisionData::m_eCollisionMode), 88);
        assert_eq!(e.get(this, NiCollisionData::m_pkModelABV), 0);
        assert_eq!(e.mem.u32(io + 4), 5);
        assert!(logged(&e, NI_BOUNDING_VOLUME_CREATE_FROM_STREAM).is_empty());
    }

    #[test]
    fn link_object_marks_the_geometry_data_for_old_streams_only() {
        for (version, mode, expected) in [
            (0x0a00_010fu32, 0u32, 0x33u8),
            (0x0a00_010f, 1, 0x33),
            (0x0a00_010f, 2, 0),
            (0x0a00_0110, 0, 0),
        ] {
            let mut e = engine();
            e.register(NI_COLLISION_OBJECT_LINK_OBJECT, nothing);
            let (stream, _) = stream_fixture(&mut e, version);
            let (scene, _, data) = geometry_scene(&mut e, 1, 1);
            let this = collision_data(&mut e);
            e.set(this, NiCollisionData::m_pkSceneObject, scene);
            e.set(this, NiCollisionData::m_eCollisionMode, mode);
            e.call(0x00ea_d760, &args![this, stream]);
            assert_eq!(e.mem.u8(data + 0x38), expected, "{version:#x} mode {mode}");
            assert_eq!(
                logged(&e, NI_COLLISION_OBJECT_LINK_OBJECT),
                vec![vec![this.addr(), stream]]
            );
        }
    }

    /// A write function double that notes the bytes of every write.
    fn writer(e: &mut Engine) -> std::rc::Rc<std::cell::RefCell<Vec<Vec<u8>>>> {
        let seen = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let record = seen.clone();
        e.register_double(0x0300_0011, move |e, a| {
            record.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            Ret {
                eax: a[2],
                ..Ret::default()
            }
        });
        seen
    }

    #[test]
    fn save_binary_writes_the_modes_and_the_volume() {
        let mut e = engine();
        e.register(NI_COLLISION_OBJECT_SAVE_BINARY, nothing);
        e.register(0x0300_0004, nothing);
        let (stream, io) = stream_fixture(&mut e, 0x0a00_0106);
        let written = writer(&mut e);
        let volume = vtable_object(&mut e, 0x0200_0000, &[(4, 0x0300_0004)]);
        let this = collision_data(&mut e);
        e.set(this, NiCollisionData::m_ePropagationMode, 0x0102_0304);
        e.set(this, NiCollisionData::m_eCollisionMode, 5);
        e.set(this, NiCollisionData::m_pkModelABV, volume);
        e.call(0x00ea_d7b0, &args![this, stream]);
        assert_eq!(
            *written.borrow(),
            vec![vec![4, 3, 2, 1], vec![5, 0, 0, 0], vec![1]]
        );
        assert_eq!(e.mem.u32(io + 4), 9);
        assert_eq!(logged(&e, 0x0300_0004), vec![vec![volume, stream]]);
        assert_eq!(
            logged(&e, NI_COLLISION_OBJECT_SAVE_BINARY),
            vec![vec![this.addr(), stream]]
        );
    }

    #[test]
    fn save_binary_writes_a_zero_flag_without_a_volume() {
        let mut e = engine();
        e.register(NI_COLLISION_OBJECT_SAVE_BINARY, nothing);
        let (stream, io) = stream_fixture(&mut e, 0x0a00_0106);
        let written = writer(&mut e);
        let this = collision_data(&mut e);
        e.set(this, NiCollisionData::m_ePropagationMode, 2);
        e.set(this, NiCollisionData::m_eCollisionMode, 3);
        e.call(0x00ea_d7b0, &args![this, stream]);
        assert_eq!(
            *written.borrow(),
            vec![vec![2, 0, 0, 0], vec![3, 0, 0, 0], vec![0]]
        );
        assert_eq!(e.mem.u32(io + 4), 9);
    }

    /// A viewer strings array (capacity at +8, size at +0xa, grow-by at +0xe) with the grow and
    /// `SetAt` doubles: `SetAt` stores the value at `06000000 + 4 * index` and counts it.
    fn strings_fixture(e: &mut Engine, capacity: u16, size: u16) -> u32 {
        e.map(0x0600_0000, 0x1000);
        e.register(VIEWER_STRINGS_GROW, nothing);
        e.register(VIEWER_STRINGS_SET_AT, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(0x0600_0000 + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x20);
        e.mem.set_u16(strings + 8, capacity);
        e.mem.set_u16(strings + 0xa, size);
        e.mem.set_u16(strings + 0xe, 4);
        strings
    }

    #[test]
    fn get_viewer_strings_appends_the_class_line_and_adds_the_rest() {
        let mut e = engine();
        e.register(NI_COLLISION_OBJECT_GET_VIEWER_STRINGS, nothing);
        e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
            eax: 0x6001,
            ..Ret::default()
        });
        e.register(0x0300_0001, nothing);
        e.mem.set_u32(NI_COLLISION_DATA_RTTI, 0x5aa5);
        let strings = strings_fixture(&mut e, 4, 1);
        let this = collision_data(&mut e);
        let vtable = 0x0200_0000;
        e.map(vtable, 0x200);
        e.mem.set_u32(vtable + 0x64, 0x0300_0001);
        e.mem.set_u32(this.addr(), vtable);
        e.call(0x00ea_d870, &args![this, strings]);
        assert_eq!(
            logged(&e, NI_COLLISION_OBJECT_GET_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        assert_eq!(logged(&e, NI_GET_VIEWER_STRING), vec![vec![0x5aa5]]);
        assert_eq!(e.mem.u32(0x0600_0000 + 4), 0x6001);
        assert!(logged(&e, VIEWER_STRINGS_GROW).is_empty());
        assert_eq!(logged(&e, 0x0300_0001), vec![vec![this.addr(), strings]]);
    }

    #[test]
    fn get_viewer_strings_grows_a_full_array_first() {
        let mut e = engine();
        e.register(NI_COLLISION_OBJECT_GET_VIEWER_STRINGS, nothing);
        e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
            eax: 0x6001,
            ..Ret::default()
        });
        e.register(0x0300_0001, nothing);
        let strings = strings_fixture(&mut e, 3, 3);
        let this = collision_data(&mut e);
        e.map(0x0200_0000, 0x200);
        e.mem.set_u32(0x0200_0000 + 0x64, 0x0300_0001);
        e.mem.set_u32(this.addr(), 0x0200_0000);
        e.call(0x00ea_d870, &args![this, strings]);
        assert_eq!(logged(&e, VIEWER_STRINGS_GROW), vec![vec![strings, 7]]);
        assert_eq!(logged(&e, VIEWER_STRINGS_SET_AT).len(), 1);
        assert_eq!(logged(&e, VIEWER_STRINGS_SET_AT)[0][..2], [strings, 3]);
    }

    #[test]
    fn add_viewer_strings_appends_the_class_modes_volume_and_velocities() {
        let mut e = formatting_engine();
        e.register(NI_COLLISION_OBJECT_ADD_VIEWER_STRINGS, nothing);
        e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
            eax: 0x1000,
            ..Ret::default()
        });
        e.register(NI_POINT3_GET_VIEWER_STRING, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x0300_0001, nothing);
        e.map(0x010f_1000, 0x1000);
        e.mem.set_cstr(PROPAGATION_MODE_NAME, b"m_ePropagationMode");
        e.mem.set_cstr(COLLISION_MODE_NAME, b"m_eCollisionMode");
        let strings = strings_fixture(&mut e, 16, 0);
        let volume = vtable_object(&mut e, 0x0200_0000, &[(0x2c, 0x0300_0001)]);
        let this = collision_data(&mut e);
        e.set(this, NiCollisionData::m_pkModelABV, volume);
        e.set(this, NiCollisionData::m_ePropagationMode, 2);
        e.set(this, NiCollisionData::m_eCollisionMode, 1);
        e.call(0x00ea_d8d0, &args![this, strings]);
        assert_eq!(
            logged(&e, NI_COLLISION_OBJECT_ADD_VIEWER_STRINGS),
            vec![vec![this.addr(), strings]]
        );
        let sprintf = logged(&e, NI_SPRINTF);
        assert_eq!(sprintf.len(), 2);
        assert_eq!(
            sprintf[0][1..],
            [18 + 0x20, 0x010f_19c8, PROPAGATION_MODE_NAME]
        );
        assert_eq!(
            sprintf[1][1..],
            [16 + 0x11, 0x010f_1990, COLLISION_MODE_NAME]
        );
        let stored: Vec<u32> = (0..5).map(|i| e.mem.u32(0x0600_0000 + 4 * i)).collect();
        assert_eq!(
            stored,
            vec![
                0x1000,
                sprintf[0][0],
                sprintf[1][0],
                this.addr() + 0xc,
                this.addr() + 0x18
            ]
        );
        assert_eq!(
            logged(&e, 0x0300_0001),
            vec![vec![volume, MODEL_ABV_TYPE_NAME, strings]]
        );
        assert_eq!(
            logged(&e, NI_POINT3_GET_VIEWER_STRING),
            vec![
                vec![this.addr() + 0xc, LOCAL_VELOCITY_NAME],
                vec![this.addr() + 0x18, WORLD_VELOCITY_NAME]
            ]
        );
        // The volume's strings come between the collision mode line and the velocities.
        let order: Vec<u32> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|call| call.0)
            .filter(|address| {
                [
                    VIEWER_STRINGS_SET_AT,
                    0x0300_0001,
                    NI_POINT3_GET_VIEWER_STRING,
                ]
                .contains(address)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                VIEWER_STRINGS_SET_AT,
                VIEWER_STRINGS_SET_AT,
                VIEWER_STRINGS_SET_AT,
                0x0300_0001,
                NI_POINT3_GET_VIEWER_STRING,
                VIEWER_STRINGS_SET_AT,
                NI_POINT3_GET_VIEWER_STRING,
                VIEWER_STRINGS_SET_AT
            ]
        );
    }

    #[test]
    fn add_viewer_strings_without_a_volume_skips_its_strings() {
        let mut e = formatting_engine();
        e.register(NI_COLLISION_OBJECT_ADD_VIEWER_STRINGS, nothing);
        e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
            eax: 0x1000,
            ..Ret::default()
        });
        e.register(NI_POINT3_GET_VIEWER_STRING, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.map(0x010f_1000, 0x1000);
        e.mem.set_cstr(PROPAGATION_MODE_NAME, b"m_ePropagationMode");
        e.mem.set_cstr(COLLISION_MODE_NAME, b"m_eCollisionMode");
        let strings = strings_fixture(&mut e, 16, 0);
        let this = collision_data(&mut e);
        e.call(0x00ea_d8d0, &args![this, strings]);
        assert_eq!(logged(&e, VIEWER_STRINGS_SET_AT).len(), 5);
    }

    #[test]
    fn create_clone_constructs_a_new_object_with_the_same_members() {
        let mut e = engine();
        e.register(NI_COLLISION_OBJECT_CONSTRUCTOR, nothing);
        e.register(NI_OBJECT_COPY_MEMBERS, nothing);
        e.register(OPERATOR_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        let this = collision_data(&mut e);
        e.set(this, NiCollisionData::m_ePropagationMode, 1);
        e.set(this, NiCollisionData::m_eCollisionMode, 0);
        let clone = e.call(0x00ea_da20, &args![this, 0x77u32]).u32();
        assert_ne!(clone, 0);
        assert_ne!(clone, this.addr());
        assert_eq!(logged(&e, OPERATOR_NEW), vec![vec![0x50]]);
        assert_eq!(e.mem.u32(clone), 0x010f_18bc);
        assert_eq!(e.mem.u32(clone + 0x24), 1);
        assert_eq!(e.mem.u32(clone + 0x28), 0);
        assert_eq!(
            logged(&e, NI_OBJECT_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x77]]
        );
    }

    #[test]
    fn create_clone_copies_into_null_when_the_allocation_fails() {
        let mut e = engine();
        // The copy writes the modes through the null destination: give page 0 some memory.
        e.map(0, 0x1000);
        e.register(NI_OBJECT_COPY_MEMBERS, nothing);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        let this = collision_data(&mut e);
        assert_eq!(e.call(0x00ea_da20, &args![this, 0x77u32]).u32(), 0);
        assert_eq!(
            logged(&e, NI_OBJECT_COPY_MEMBERS),
            vec![vec![this.addr(), 0, 0x77]]
        );
    }
}
