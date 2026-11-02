//! `NiParticle/nipsysmeshupdatemodifier.cpp` (Xbox PDB source unit), subsystem `NiParticle`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00c21840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshUpdateModifier::GetViewerStrings` (Xbox PDB): runs the parent class's `GetViewerStrings`
/// (`00c319e0`) on `strings`, then appends the class name (the first word of the
/// `NiRTTI` record at `01202530`, passed through `NiGetViewerString`) to the viewer-string
/// array: when the 16-bit size (`+0xA`) has reached the capacity (`+0x8`) the array is
/// grown by the step (`+0xE`) plus the size first, then the string is stored at the old size.
pub fn ni_p_sys_mesh_update_modifier_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    const PARENT_GET_VIEWER_STRINGS: u32 = 0x00c3_19e0;
    const CLASS_NAME_POINTER: u32 = 0x0120_2530;
    const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
    const STRINGS_GROW: u32 = 0x0096_ad30;
    const STRINGS_SET_AT: u32 = 0x0096_ae90;
    e.call(PARENT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(CLASS_NAME_POINTER);
    let text = e.call(NI_GET_VIEWER_STRING, &args![class_name]).u32();
    let size = u32::from(e.mem.u16(strings.addr() + 0xa));
    if size >= u32::from(e.mem.u16(strings.addr() + 8)) {
        let step = u32::from(e.mem.u16(strings.addr() + 0xe));
        e.call(STRINGS_GROW, &args![strings, step + size]);
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), text);
        e.call(STRINGS_SET_AT, &args![strings, size, slot]);
    });
}

// Translated from 00c21720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshUpdateModifier::GetRTTI` (Xbox PDB): takes no arguments and returns the constant 0x0120_2530.
pub fn ni_p_sys_mesh_update_modifier_get_r_t_t_i(_e: &mut Engine) -> u32 {
    0x0120_2530
}

use crate::types::NiTArray;
use crate::units::niparticle::nipsysmodifier::{
    stream_write, NiPSysModifier, INTERLOCKED_DECREMENT, INTERLOCKED_INCREMENT,
    STREAM_SAVE_LINK_ID, STREAM_VERSION,
};

layout! {
    /// `NiPSysMeshUpdateModifier` (Xbox PDB), 0x28 bytes; the PC code uses
    /// the same offsets.
    pub struct NiPSysMeshUpdateModifier: 0x28 {
        /// `m_kMeshes` (Xbox PDB): an `NiTObjectArray<NiPointer<NiAVObject>>`
        /// (its vtable; the rest is `crate::types::NiTArray` at +0x18).
        0x18 m_kMeshes: u32,
    }
}

/// The vtable of `NiPSysMeshUpdateModifier` (RTTI
/// `.?AVNiPSysMeshUpdateModifier@@`).
const VTABLE: u32 = 0x010b_dc4c;
/// The vtables of `NiTObjectArray<NiPointer<NiAVObject>>` (`0101fbd4`) and
/// of its base `NiTArray<NiPointer<NiAVObject>, NiTNewInterface<..>>`
/// (`0101fbcc`), by their RTTI.
const OBJECT_ARRAY_VTABLE: u32 = 0x0101_fbd4;
const ARRAY_VTABLE: u32 = 0x0101_fbcc;
/// The order a loaded mesh update modifier gets (`0x1964`).
const LOADED_ORDER: u32 = 0x1964;
/// `00aa13e0` (`cdecl`, one word): the Gamebryo allocation; `00aa1460`
/// (`cdecl`, `(block, size)`): the sized delete.
const NI_NEW: u32 = 0x00aa_13e0;
const NI_DELETE: u32 = 0x00aa_1460;
/// `NiPSysModifier`'s functions this unit's call (`thiscall`): the
/// constructor (`00c31720`), destructor body (`00c31750`), member copy
/// (`00c31780`, `(clone, process)`), `LinkObject` (`00c31650`),
/// `RegisterStreamables` (`00c31670`), `ProcessClone` (`00c317e0`),
/// `IsEqual` (`00c31830`), `LoadBinary` (`00c318b0`) and `SaveBinary`
/// (`00c31950`) (one word each where not said).
const MODIFIER_CONSTRUCTOR: u32 = 0x00c3_1720;
const MODIFIER_DESTRUCTOR: u32 = 0x00c3_1750;
const MODIFIER_COPY_MEMBERS: u32 = 0x00c3_1780;
const MODIFIER_LINK_OBJECT: u32 = 0x00c3_1650;
const MODIFIER_REGISTER_STREAMABLES: u32 = 0x00c3_1670;
const MODIFIER_PROCESS_CLONE: u32 = 0x00c3_17e0;
const MODIFIER_IS_EQUAL: u32 = 0x00c3_1830;
const MODIFIER_LOAD_BINARY: u32 = 0x00c3_18b0;
const MODIFIER_SAVE_BINARY: u32 = 0x00c3_1950;
/// `00a89af0` (3 bytes, `thiscall`, one word): the base post-link step.
const BASE_POST_LINK_OBJECT: u32 = 0x00a8_9af0;
/// `NiStream::ReadMultipleLinkIDs` (`00a64670`, no stack words),
/// `NiStream::GetNumberOfLinkIDs` (`00a63460`, no stack words) and
/// `NiStream::GetObjectFromLinkID` (`00a63430`, no stack words).
const READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
const GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// The `NiTArray<NiPointer<..>>` helpers (`thiscall` on the array):
/// `0060bef0` resizes it (one word, the new capacity) and `004b0310` sets
/// an element (`(index, &pointer)`). `004b0220` (`cdecl`, the element
/// block) deletes the element block.
const ARRAY_SET_SIZE: u32 = 0x0060_bef0;
const ARRAY_SET_AT: u32 = 0x004b_0310;
const ARRAY_DELETE_ELEMENTS: u32 = 0x004b_0220;
/// `NiObject::Clone_ov2` (`00a5d680`, `thiscall`, no stack words).
const OBJECT_CLONE: u32 = 0x00a5_d680;
/// `00a6d390` (unnamed, `NiMain/nitimecontroller.cpp`; `cdecl`: an
/// object and a `float` time): run on a mesh taken for a particle with
/// time 0.
const START_CONTROLLERS: u32 = 0x00a6_d390;
/// `NiMeshPSysData::SetNumGenerations` (`00c30f90`, `thiscall`, one word).
const SET_NUM_GENERATIONS: u32 = 0x00c3_0f90;
/// `NiPointer` assignment (`006e5cc0`, `thiscall` on the destination, the
/// address of the source pointer; the engine map names it after another
/// template instance).
const POINTER_ASSIGN: u32 = 0x006e_5cc0;
/// `004168a0` (unnamed; `thiscall` on an `NiMatrix3`: angle, axis x, y,
/// z as floats): the rotation matrix by an angle about an axis.
const MATRIX_MAKE_ROTATION: u32 = 0x0041_68a0;
/// The stream version from which the mesh pools are filled
/// (`CMP [stream+0xd8], 0x0a010000; JA`).
const VERSION_POOLS: u32 = 0x0a01_0000;
/// Vtable slots: `NiObject::CreateClone` (+0x48), `RegisterStreamables`
/// (+0x54), `IsEqual` (+0x5c), `ProcessClone` (+0x68), `DeleteThis` (+4),
/// the `NiAVObject` update the meshes get (+0xa4, `(&update data,
/// flags)`), the call on the target given to a taken mesh (+0xb0, one
/// word), and the particle-mesh node's slot +0xf8 (`(index, mesh)`).
const CREATE_CLONE_SLOT: u32 = 0x48;
const REGISTER_STREAMABLES_SLOT: u32 = 0x54;
const IS_EQUAL_SLOT: u32 = 0x5c;
const PROCESS_CLONE_SLOT: u32 = 0x68;
const DELETE_THIS_SLOT: u32 = 0x4;
const AV_OBJECT_UPDATE_SLOT: u32 = 0xa4;
const MESH_TARGET_SLOT: u32 = 0xb0;
const MESHES_SET_AT_SLOT: u32 = 0xf8;
/// `NiPSysData` fields as the PC code reads them: positions (+0x20),
/// radii (+0x4c), active count (+0x50, `u16`), sizes (+0x54), rotation
/// angles (+0x5c), rotation axes (+0x60), particle infos (+0x70; 0x1c
/// bytes each: age +0xc, generation +0x18 `u16`); `NiMeshPSysData`
/// (Xbox PDB): `m_spParticleMeshes` (+0x7c), `m_bFillPoolsOnLoad`
/// (+0x80) and `m_kPools` (+0x88: element block +0x8c, size +0x92).
const DATA_VERTICES: u32 = 0x20;
const DATA_RADII: u32 = 0x4c;
const DATA_ACTIVE: u32 = 0x50;
const DATA_SIZES: u32 = 0x54;
const DATA_ROTATION_ANGLES: u32 = 0x5c;
const DATA_ROTATION_AXES: u32 = 0x60;
const DATA_INFOS: u32 = 0x70;
const INFO_SIZE: u32 = 0x1c;
const INFO_AGE: u32 = 0xc;
const INFO_GENERATION: u32 = 0x18;
const DATA_PARTICLE_MESHES: u32 = 0x7c;
const DATA_FILL_POOLS_ON_LOAD: u32 = 0x80;
const DATA_POOLS: u32 = 0x8c;
const DATA_POOL_COUNT: u32 = 0x92;
/// `NiNode::m_kChildren`'s element block (+0xa0) and `NiAVObject`'s flags
/// (+0x30), local rotation (+0x34), translation (+0x58) and scale (+0x64),
/// as the PC code reads them.
const NODE_CHILDREN: u32 = 0xa0;
const OBJECT_FLAGS: u32 = 0x30;
const OBJECT_ROTATION: u32 = 0x34;
const OBJECT_TRANSLATION: u32 = 0x58;
const OBJECT_SCALE: u32 = 0x64;
/// The target system's word +0x9c that a taken mesh's slot +0xb0 gets the
/// address of.
const SYSTEM_MESH_TARGET: u32 = 0x9c;

/// The meshes array (`m_kMeshes`, +0x18).
fn meshes(this: Ptr<NiPSysMeshUpdateModifier>) -> Ptr<NiTArray> {
    Ptr::new(this.addr() + NiPSysMeshUpdateModifier::m_kMeshes.off)
}

/// `InterlockedIncrement` on a reference-counted object's count (+4).
fn hold(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

/// `InterlockedDecrement` on the count (+4); at zero `DeleteThis` (slot +4).
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, DELETE_THIS_SLOT, &args![]);
    }
}

/// The empty `NiTObjectArray<NiPointer<NiAVObject>>` the constructor and
/// `CreateClone` build at +0x18: no elements, grow by 1.
fn init_meshes(e: &mut Engine, this: Ptr<NiPSysMeshUpdateModifier>) {
    let array = meshes(this);
    e.set(array, NiTArray::m_usGrowBy, 1);
    e.set(array, NiTArray::m_usMaxSize, 0);
    e.set(array, NiTArray::m_usSize, 0);
    e.set(array, NiTArray::m_usESize, 0);
    e.set(array, NiTArray::m_pBase, 0);
    e.mem.set_u32(array.addr(), OBJECT_ARRAY_VTABLE);
}

// Translated from 00c20f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshUpdateModifier::LoadBinary` (Xbox PDB): `NiPSysModifier`'s
/// `LoadBinary`, the order set to `0x1964`, then the meshes' link ids
/// (`NiStream::ReadMultipleLinkIDs`).
pub fn ni_p_sys_mesh_update_modifier_load_binary(
    e: &mut Engine,
    this: Ptr<NiPSysMeshUpdateModifier>,
    stream: Ptr,
) {
    e.call(MODIFIER_LOAD_BINARY, &args![this, stream]);
    e.set(
        this.cast::<NiPSysModifier>(),
        NiPSysModifier::m_uiOrder,
        LOADED_ORDER,
    );
    e.call(READ_MULTIPLE_LINK_IDS, &args![stream]);
}

// Translated from 00c20f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshUpdateModifier::ProcessClone` (Xbox PDB): `NiPSysModifier`'s
/// `ProcessClone`, then each non-null mesh's (slot +0x68).
pub fn ni_p_sys_mesh_update_modifier_process_clone(
    e: &mut Engine,
    this: Ptr<NiPSysMeshUpdateModifier>,
    process: Ptr,
) {
    e.call(MODIFIER_PROCESS_CLONE, &args![this, process]);
    let array = meshes(this);
    let mut i = 0u32;
    while i < u32::from(e.get(array, NiTArray::m_usSize)) {
        let mesh = e.mem.u32(e.get(array, NiTArray::m_pBase) + 4 * i);
        if mesh != 0 {
            e.vcall(mesh, PROCESS_CLONE_SLOT, &args![process]);
        }
        i += 1;
    }
}

// Translated from 00c20f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshUpdateModifier::RegisterStreamables` (Xbox PDB):
/// `NiPSysModifier`'s `RegisterStreamables`; when it answers false, false.
/// Otherwise each non-null mesh's (slot +0x54), and true.
pub fn ni_p_sys_mesh_update_modifier_register_streamables(
    e: &mut Engine,
    this: Ptr<NiPSysMeshUpdateModifier>,
    stream: Ptr,
) -> bool {
    if !e
        .call(MODIFIER_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let array = meshes(this);
    let mut i = 0u32;
    while i < u32::from(e.get(array, NiTArray::m_usSize)) {
        let mesh = e.mem.u32(e.get(array, NiTArray::m_pBase) + 4 * i);
        if mesh != 0 {
            e.vcall(mesh, REGISTER_STREAMABLES_SLOT, &args![stream]);
        }
        i += 1;
    }
    true
}

// Translated from 00c20fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiTObjectArray<NiPointer<..>>` removal
/// of the last element (`thiscall` on the array, `RET 4`: the address of
/// the `NiPointer` result). An empty array gives null. Otherwise the size
/// drops by one, the element there is held for the result, released from
/// the array (deleted through slot +4 at zero) and its slot cleared, the
/// non-null count drops for a non-null element, and the result (held, the
/// local reference released) is stored. Returns the result's address.
pub fn fn_00c20fd0(e: &mut Engine, this: Ptr<NiTArray>, result: Ptr) -> Ptr {
    let size = e.get(this, NiTArray::m_usSize);
    if size == 0 {
        e.mem.set_u32(result.addr(), 0);
        return result;
    }
    let size = size - 1;
    e.set(this, NiTArray::m_usSize, size);
    let element = e
        .mem
        .u32(e.get(this, NiTArray::m_pBase) + 4 * u32::from(size));
    if element != 0 {
        hold(e, element);
    }
    let size = e.get(this, NiTArray::m_usSize);
    let slot = e.get(this, NiTArray::m_pBase) + 4 * u32::from(size);
    let old = e.mem.u32(slot);
    if old != 0 {
        release(e, old);
        e.mem.set_u32(slot, 0);
    }
    if element != 0 {
        let count = e.get(this, NiTArray::m_usESize);
        e.set(this, NiTArray::m_usESize, count.wrapping_add(0xffff));
    }
    e.mem.set_u32(result.addr(), element);
    if element != 0 {
        hold(e, element);
        release(e, element);
    }
    result
}

// Translated from 00c21080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshUpdateModifier::Update` (Xbox PDB; the time is not read):
/// for each active particle, the particle-mesh node's child of that index
/// gets the particle's position as its translation; with rotation angles
/// and axes, its rotation becomes the angle about the axis (`004168a0`);
/// its scale is `|size * radius|` (rounded to a float); then its update
/// (slot +0xa4) runs with update data whose time is the particle's age and
/// whose first flag is set (the other four bytes 0), and flags 0. The loop
/// counter lives in the stack word of the data argument.
pub fn ni_p_sys_mesh_update_modifier_update(
    e: &mut Engine,
    _this: Ptr<NiPSysMeshUpdateModifier>,
    _time: f32,
    data: Ptr,
) {
    let d = data.addr();
    e.with_stack(0x60, |e, frame| {
        let at = |x: u32| frame.addr() + x;
        let angles = e.mem.u32(d + DATA_ROTATION_ANGLES);
        let axes = e.mem.u32(d + DATA_ROTATION_AXES);
        e.mem.set_u32(at(0x10), angles);
        e.mem.set_u32(at(0x14), axes);
        // The data argument's stack word, now the loop counter.
        let counter = at(0x58);
        e.mem.set_u32(counter, 0);
        if e.mem.u16(d + DATA_ACTIVE) == 0 {
            return;
        }
        loop {
            let i = u32::from(e.mem.u32(counter) as u16);
            let info = e.mem.u32(d + DATA_INFOS).wrapping_add(i * INFO_SIZE);
            let node = e.mem.u32(d + DATA_PARTICLE_MESHES);
            let children = e.mem.u32(node.wrapping_add(NODE_CHILDREN));
            let mesh = e.mem.u32(children.wrapping_add(4 * i));
            let position = e.mem.u32(d + DATA_VERTICES).wrapping_add(12 * i);
            e.mem.set_u32(at(0x1c), info);
            for k in 0..3 {
                let word = e.mem.u32(position + 4 * k);
                e.mem
                    .set_u32(mesh.wrapping_add(OBJECT_TRANSLATION + 4 * k), word);
            }
            e.mem.set_u32(at(0x18), mesh);
            let angles = e.mem.u32(at(0x10));
            let axes = e.mem.u32(at(0x14));
            if angles != 0 && axes != 0 {
                let axis = axes.wrapping_add(12 * i);
                let z = x87_quiet_bits(e.mem.u32(axis + 8));
                let y = x87_quiet_bits(e.mem.u32(axis + 4));
                let x = x87_quiet_bits(e.mem.u32(axis));
                let angle = x87_quiet_bits(e.mem.u32(angles + 4 * i));
                e.call(MATRIX_MAKE_ROTATION, &args![at(0x2c), angle, x, y, z]);
                for k in 0..9 {
                    let word = e.mem.u32(at(0x2c + 4 * k));
                    e.mem
                        .set_u32(mesh.wrapping_add(OBJECT_ROTATION + 4 * k), word);
                }
            }
            let size = x87_load32(e.mem.u32(e.mem.u32(d + DATA_SIZES) + 4 * i));
            let radius = x87_m32(e.mem.u32(e.mem.u32(d + DATA_RADII) + 4 * i));
            e.mem.set_u32(at(0x18), x87_store32(x87_mul(size, radius)));
            let magnitude = x87_store32(x87_load32(e.mem.u32(at(0x18))).abs());
            e.mem.set_u32(at(0x18), magnitude);
            let scale = x87_quiet_bits(e.mem.u32(at(0x18)));
            e.mem.set_u32(mesh.wrapping_add(OBJECT_SCALE), scale);
            let age = x87_quiet_bits(e.mem.u32(info + INFO_AGE));
            e.mem.set_u32(at(0x20), age);
            e.mem.set_u8(at(0x24), 1);
            for k in 0..4 {
                e.mem.set_u8(at(0x25 + k), 0);
            }
            e.vcall(mesh, AV_OBJECT_UPDATE_SLOT, &args![at(0x20), 0u32]);
            let next = e.mem.u32(counter).wrapping_add(1);
            e.mem.set_u32(counter, next);
            if next as u16 >= e.mem.u16(d + DATA_ACTIVE) {
                break;
            }
        }
    });
}

// Translated from 00c211c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshUpdateModifier::SaveBinary` (Xbox PDB): `NiPSysModifier`'s
/// `SaveBinary`, the mesh count (`m_usSize`, as 4 bytes written from the
/// stack word that held the stream argument; the component size in the
/// function's own stack word), then each mesh's link id (stream slot
/// +0x30), nulls included.
pub fn ni_p_sys_mesh_update_modifier_save_binary(
    e: &mut Engine,
    this: Ptr<NiPSysMeshUpdateModifier>,
    stream: Ptr,
) {
    let s = stream.addr();
    e.call(MODIFIER_SAVE_BINARY, &args![this, s]);
    let array = meshes(this);
    e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let argument_slot = frame.addr() + 4;
        let count = u32::from(e.get(array, NiTArray::m_usSize));
        e.mem.set_u32(argument_slot, count);
        stream_write(e, s, argument_slot, 4, 4, local);
        let mut i = 0u32;
        while i < e.mem.u32(argument_slot) {
            let mesh = e.mem.u32(e.get(array, NiTArray::m_pBase) + 4 * i);
            e.vcall(s, STREAM_SAVE_LINK_ID, &args![mesh]);
            i += 1;
        }
    });
}

// Translated from 00c21230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshUpdateModifier::IsEqual` (Xbox PDB): `NiPSysModifier::IsEqual`,
/// the same mesh count, and for each index both meshes null or both
/// non-null and equal (the first's `IsEqual`, slot +0x5c; an index past the
/// other's count reads as null).
pub fn ni_p_sys_mesh_update_modifier_is_equal(
    e: &mut Engine,
    this: Ptr<NiPSysMeshUpdateModifier>,
    other: Ptr<NiPSysMeshUpdateModifier>,
) -> bool {
    if !e.call(MODIFIER_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let (mine, theirs) = (meshes(this), meshes(other));
    let count = e.get(mine, NiTArray::m_usSize);
    if count != e.get(theirs, NiTArray::m_usSize) {
        return false;
    }
    let mut i = 0u32;
    if count == 0 {
        return true;
    }
    loop {
        let other_count = u32::from(e.get(theirs, NiTArray::m_usSize));
        let a = e.mem.u32(e.get(mine, NiTArray::m_pBase) + 4 * i);
        let b = if i < other_count {
            e.mem.u32(e.get(theirs, NiTArray::m_pBase) + 4 * i)
        } else {
            0
        };
        if a == 0 {
            if b != 0 {
                return false;
            }
        } else if b == 0 || !e.vcall(a, IS_EQUAL_SLOT, &args![b]).bool() {
            return false;
        }
        i += 1;
        if i >= u32::from(e.get(mine, NiTArray::m_usSize)) {
            return true;
        }
    }
}

// Translated from 00c212b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::FillPool` (Xbox PDB; `thiscall` on the mesh data,
/// `RET 8`: the pool index, the mesh): with a pool at that index, each of
/// its slots up to its capacity (+8, re-read each time) gets a clone of the
/// mesh (`NiObject::Clone_ov2`, through a `NiPointer` kept in the stack
/// word of the index argument; set with `004b0310`).
pub fn ni_mesh_p_sys_data_fill_pool(e: &mut Engine, this: Ptr, index: u32, mesh: Ptr) {
    let d = this.addr();
    if index >= u32::from(e.mem.u16(d + DATA_POOL_COUNT)) {
        return;
    }
    let pool = e.mem.u32(e.mem.u32(d + DATA_POOLS) + 4 * index);
    if pool == 0 {
        return;
    }
    let mut i = 0u32;
    if e.mem.u16(pool + 8) == 0 {
        return;
    }
    e.with_stack(4, |e, slot| loop {
        let clone = e.call(OBJECT_CLONE, &args![mesh]).u32();
        e.mem.set_u32(slot.addr(), clone);
        if clone != 0 {
            hold(e, clone);
        }
        e.call(ARRAY_SET_AT, &args![pool, i, slot]);
        if clone != 0 {
            release(e, clone);
        }
        i += 1;
        if i >= u32::from(e.mem.u16(pool + 8)) {
            break;
        }
    });
}

/// `NiPointer` assignment of null to the pointer at `pointer` (the old one
/// released unless null), as `RemoveFromPool` inlines it.
fn clear_pointer(e: &mut Engine, pointer: u32) {
    let old = e.mem.u32(pointer);
    if old != 0 {
        release(e, old);
        e.mem.set_u32(pointer, 0);
    }
}

// Translated from 00c21340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMeshPSysData::RemoveFromPool` (Xbox PDB; `thiscall` on the mesh data,
/// `RET 8`: the pool index, the address of the `NiPointer` result): without
/// a pool at that index the result is cleared; otherwise the pool's last
/// element is removed (`00c20fd0`, into the stack word of the index
/// argument), assigned to the result (`006e5cc0`) and the temporary
/// released.
pub fn ni_mesh_p_sys_data_remove_from_pool(e: &mut Engine, this: Ptr, index: u32, result: Ptr) {
    let d = this.addr();
    if index >= u32::from(e.mem.u16(d + DATA_POOL_COUNT)) {
        clear_pointer(e, result.addr());
        return;
    }
    let pool = e.mem.u32(e.mem.u32(d + DATA_POOLS) + 4 * index);
    if pool == 0 {
        clear_pointer(e, result.addr());
        return;
    }
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), index);
        let removed = fn_00c20fd0(e, Ptr::new(pool), slot);
        e.call(POINTER_ASSIGN, &args![result, removed]);
        let temporary = e.mem.u32(slot.addr());
        if temporary != 0 {
            release(e, temporary);
        }
    });
}

// Translated from 00c21400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshUpdateModifier::Initialize` (Xbox PDB; `RET 8`: the particle
/// data, the particle index as a 16-bit value): the particle's generation
/// (capped to the last mesh); a mesh from that generation's pool
/// (`RemoveFromPool`, into the stack word of the index argument), else a
/// clone of the generation's mesh (held); its flags get bit 0, `00a6d390`
/// runs on it with time 0, the particle-mesh node takes it at the
/// particle's index (slot +0xf8), its slot +0xb0 gets the address of the
/// target system's +0x9c, and the reference is released.
pub fn ni_p_sys_mesh_update_modifier_initialize(
    e: &mut Engine,
    this: Ptr<NiPSysMeshUpdateModifier>,
    data: Ptr,
    index: u32,
) {
    let d = data.addr();
    let particle = index & 0xffff;
    let info = e.mem.u32(d + DATA_INFOS).wrapping_add(particle * INFO_SIZE);
    let mut generation = e.mem.u16(info + INFO_GENERATION);
    let count = e.get(meshes(this), NiTArray::m_usSize);
    if generation >= count {
        generation = count.wrapping_sub(1);
    }
    let generation = u32::from(generation);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        ni_mesh_p_sys_data_remove_from_pool(e, data, generation, slot);
        let mut mesh = e.mem.u32(slot.addr());
        if mesh == 0 {
            let source = e
                .mem
                .u32(e.get(meshes(this), NiTArray::m_pBase) + 4 * generation);
            let clone = e.call(OBJECT_CLONE, &args![source]).u32();
            if clone != 0 {
                mesh = clone;
                hold(e, clone);
            }
        }
        let flags = e.mem.u32(mesh.wrapping_add(OBJECT_FLAGS));
        e.mem.set_u32(mesh.wrapping_add(OBJECT_FLAGS), flags | 1);
        e.call(START_CONTROLLERS, &args![mesh, 0u32]);
        let node = e.mem.u32(d + DATA_PARTICLE_MESHES);
        e.vcall(node, MESHES_SET_AT_SLOT, &args![particle, mesh]);
        let system = e.get(this.cast::<NiPSysModifier>(), NiPSysModifier::m_pkTarget);
        e.vcall(
            mesh,
            MESH_TARGET_SLOT,
            &args![system.wrapping_add(SYSTEM_MESH_TARGET)],
        );
        release(e, mesh);
    });
}

// Translated from 00c214c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshUpdateModifier::LinkObject` (Xbox PDB): `NiPSysModifier`'s
/// `LinkObject`, the meshes array sized to the stream's number of link ids
/// (`0060bef0`), then each linked object set at its index (`004b0310`,
/// through a held `NiPointer` in the function's own stack word).
pub fn ni_p_sys_mesh_update_modifier_link_object(
    e: &mut Engine,
    this: Ptr<NiPSysMeshUpdateModifier>,
    stream: Ptr,
) {
    e.call(MODIFIER_LINK_OBJECT, &args![this, stream]);
    let count = e.call(GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    let array = meshes(this);
    e.call(ARRAY_SET_SIZE, &args![array, count]);
    e.with_stack(4, |e, slot| {
        for i in 0..count {
            let object = e.call(GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
            e.mem.set_u32(slot.addr(), object);
            if object != 0 {
                hold(e, object);
            }
            e.call(ARRAY_SET_AT, &args![array, i, slot]);
            if object != 0 {
                release(e, object);
            }
        }
    });
}

// Translated from 00c21550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshUpdateModifier::PostLinkObject` (Xbox PDB): the base
/// post-link step; for files up to 10.1.0.0 the target system's mesh data
/// (+0xb8) gets the mesh count as its number of generations
/// (`SetNumGenerations`); for later files, when the mesh data fills pools
/// on load, each pool index below the pool count whose mesh exists (an
/// index below the meshes' capacity with a non-null element) is filled
/// (`FillPool`).
pub fn ni_p_sys_mesh_update_modifier_post_link_object(
    e: &mut Engine,
    this: Ptr<NiPSysMeshUpdateModifier>,
    stream: Ptr,
) {
    e.call(BASE_POST_LINK_OBJECT, &args![this, stream]);
    let system = e.get(this.cast::<NiPSysModifier>(), NiPSysModifier::m_pkTarget);
    let mesh_data = e.mem.u32(system.wrapping_add(0xb8));
    let array = meshes(this);
    if e.mem.u32(stream.addr() + STREAM_VERSION) <= VERSION_POOLS {
        let count = u32::from(e.get(array, NiTArray::m_usSize));
        e.call(SET_NUM_GENERATIONS, &args![mesh_data, count]);
        return;
    }
    if e.mem.u8(mesh_data.wrapping_add(DATA_FILL_POOLS_ON_LOAD)) == 0 {
        return;
    }
    let mut i = 0u32;
    while i < u32::from(e.mem.u16(mesh_data.wrapping_add(DATA_POOL_COUNT))) {
        if i < u32::from(e.get(array, NiTArray::m_usMaxSize)) {
            let mesh = e.mem.u32(e.get(array, NiTArray::m_pBase) + 4 * i);
            if mesh != 0 {
                ni_mesh_p_sys_data_fill_pool(e, Ptr::new(mesh_data), i, Ptr::new(mesh));
            }
        }
        i += 1;
    }
}

// Translated from 00c215d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets mesh `index` of the modifier to `mesh`
/// and hands back the one it replaces (`thiscall` on the modifier,
/// `RET 0xc`: index, mesh, the address of the `NiPointer` result). Below the
/// mesh count the old element is held, the new one set (`004b0310` through
/// a held `NiPointer` in the stack word of the mesh argument) and the
/// result assigned the old one; otherwise the array grows first when the
/// index is past its capacity (`0060bef0` to the grow step plus the index),
/// the new one is set and the result cleared.
pub fn fn_00c215d0(
    e: &mut Engine,
    this: Ptr<NiPSysMeshUpdateModifier>,
    index: u32,
    mesh: Ptr,
    result: Ptr,
) {
    let array = meshes(this);
    let new = mesh.addr();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), new);
        if index < u32::from(e.get(array, NiTArray::m_usSize)) {
            let old = e.mem.u32(e.get(array, NiTArray::m_pBase) + 4 * index);
            if old != 0 {
                hold(e, old);
            }
            if new != 0 {
                hold(e, new);
            }
            e.call(ARRAY_SET_AT, &args![array, index, slot]);
            if new != 0 {
                release(e, new);
            }
            let current = e.mem.u32(result.addr());
            if current != old {
                if current != 0 {
                    release(e, current);
                }
                e.mem.set_u32(result.addr(), old);
                if old == 0 {
                    return;
                }
                hold(e, old);
            }
            if old != 0 {
                release(e, old);
            }
        } else {
            if new != 0 {
                hold(e, new);
            }
            if index >= u32::from(e.get(array, NiTArray::m_usMaxSize)) {
                let grow = u32::from(e.get(array, NiTArray::m_usGrowBy));
                e.call(ARRAY_SET_SIZE, &args![array, grow + index]);
            }
            e.call(ARRAY_SET_AT, &args![array, index, slot]);
            if new != 0 {
                release(e, new);
            }
            clear_pointer(e, result.addr());
        }
    });
}

// Translated from 00c21730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshUpdateModifier::_scalar_deleting_destructor_` (Xbox PDB): the
/// meshes array's destructor inlined (its base vtable `0101fbcc`, the
/// element block deleted with `004b0220`), `NiPSysModifier`'s destructor
/// body, then, with bit 0 of `flags`, the sized delete of 0x28 bytes.
/// Returns `this`.
pub fn ni_p_sys_mesh_update_modifier_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiPSysMeshUpdateModifier>,
    flags: u32,
) -> Ptr<NiPSysMeshUpdateModifier> {
    let array = meshes(this);
    let elements = e.get(array, NiTArray::m_pBase);
    e.mem.set_u32(array.addr(), ARRAY_VTABLE);
    e.call(ARRAY_DELETE_ELEMENTS, &args![elements]);
    e.call(MODIFIER_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_DELETE, &args![this, 0x28u32]);
    }
    this
}

// Translated from 00c21770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the `NiPSysMeshUpdateModifier` member copy
/// into `clone` (`RET 8`: clone, process). `NiPSysModifier`'s member copy,
/// then each non-null mesh's clone (its `CreateClone`, slot +0x48) set at
/// the same index of the clone (`00c215d0`, the replaced one handed back
/// into the stack word of the process argument and released).
pub fn fn_00c21770(
    e: &mut Engine,
    this: Ptr<NiPSysMeshUpdateModifier>,
    clone: Ptr<NiPSysMeshUpdateModifier>,
    process: Ptr,
) {
    e.call(MODIFIER_COPY_MEMBERS, &args![this, clone, process]);
    let array = meshes(this);
    e.with_stack(4, |e, slot| {
        let mut i = 0u32;
        while i < u32::from(e.get(array, NiTArray::m_usSize)) {
            let mesh = e.mem.u32(e.get(array, NiTArray::m_pBase) + 4 * i);
            if mesh != 0 {
                let copy = e.vcall(mesh, CREATE_CLONE_SLOT, &args![process]).u32();
                e.mem.set_u32(slot.addr(), 0);
                fn_00c215d0(e, clone, i, Ptr::new(copy), slot);
                let replaced = e.mem.u32(slot.addr());
                if replaced != 0 {
                    release(e, replaced);
                }
            }
            i += 1;
        }
    });
}

// Translated from 00c217f0 (bytes disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): the `NiPSysMeshUpdateModifier`
/// factory. A 0x28-byte object from `00aa13e0` through `NiPSysModifier`'s
/// constructor, this class's vtable and an empty meshes array; 0 when the
/// allocation fails.
pub fn fn_00c217f0(e: &mut Engine) -> Ptr<NiPSysMeshUpdateModifier> {
    let block = e.call(NI_NEW, &args![0x28u32]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    let this = Ptr::<NiPSysMeshUpdateModifier>::new(block);
    e.call(MODIFIER_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(block, VTABLE);
    init_meshes(e, this);
    this
}

// Translated from 00c21890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiPSysMeshUpdateModifier::CreateClone` (Xbox PDB): a 0x28-byte object
/// through `NiPSysModifier`'s constructor, this class's vtable and an empty
/// meshes array (0 when the allocation fails), then the member copy
/// (`00c21770`) into it, which still runs for a null clone. Returns the
/// clone.
pub fn ni_p_sys_mesh_update_modifier_create_clone(
    e: &mut Engine,
    this: Ptr<NiPSysMeshUpdateModifier>,
    process: Ptr,
) -> Ptr<NiPSysMeshUpdateModifier> {
    let block = e.call(NI_NEW, &args![0x28u32]).u32();
    let clone = Ptr::<NiPSysMeshUpdateModifier>::new(block);
    if block != 0 {
        e.call(MODIFIER_CONSTRUCTOR, &args![clone]);
        e.mem.set_u32(block, VTABLE);
        init_meshes(e, clone);
    }
    fn_00c21770(e, this, clone, process);
    clone
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00c2_0f00,
            ni_p_sys_mesh_update_modifier_load_binary(Ptr<NiPSysMeshUpdateModifier>, Ptr)
        ),
        entry!(
            0x00c2_0f30,
            ni_p_sys_mesh_update_modifier_process_clone(Ptr<NiPSysMeshUpdateModifier>, Ptr)
        ),
        entry!(
            0x00c2_0f80,
            ni_p_sys_mesh_update_modifier_register_streamables(
                Ptr<NiPSysMeshUpdateModifier>,
                Ptr,
            ) -> bool
        ),
        entry!(0x00c2_0fd0, fn_00c20fd0(Ptr<NiTArray>, Ptr) -> Ptr),
        entry!(
            0x00c2_1080,
            ni_p_sys_mesh_update_modifier_update(Ptr<NiPSysMeshUpdateModifier>, f32, Ptr)
        ),
        entry!(
            0x00c2_11c0,
            ni_p_sys_mesh_update_modifier_save_binary(Ptr<NiPSysMeshUpdateModifier>, Ptr)
        ),
        entry!(
            0x00c2_1230,
            ni_p_sys_mesh_update_modifier_is_equal(
                Ptr<NiPSysMeshUpdateModifier>,
                Ptr<NiPSysMeshUpdateModifier>,
            ) -> bool
        ),
        entry!(0x00c2_12b0, ni_mesh_p_sys_data_fill_pool(Ptr, u32, Ptr)),
        entry!(
            0x00c2_1340,
            ni_mesh_p_sys_data_remove_from_pool(Ptr, u32, Ptr)
        ),
        entry!(
            0x00c2_1400,
            ni_p_sys_mesh_update_modifier_initialize(Ptr<NiPSysMeshUpdateModifier>, Ptr, u32)
        ),
        entry!(
            0x00c2_14c0,
            ni_p_sys_mesh_update_modifier_link_object(Ptr<NiPSysMeshUpdateModifier>, Ptr)
        ),
        entry!(
            0x00c2_1550,
            ni_p_sys_mesh_update_modifier_post_link_object(Ptr<NiPSysMeshUpdateModifier>, Ptr)
        ),
        entry!(
            0x00c2_15d0,
            fn_00c215d0(Ptr<NiPSysMeshUpdateModifier>, u32, Ptr, Ptr)
        ),
        entry!(
            0x00c2_1730,
            ni_p_sys_mesh_update_modifier_scalar_deleting_destructor(
                Ptr<NiPSysMeshUpdateModifier>,
                u32,
            ) -> Ptr<
                NiPSysMeshUpdateModifier,
            >
        ),
        entry!(
            0x00c2_1770,
            fn_00c21770(Ptr<NiPSysMeshUpdateModifier>, Ptr<NiPSysMeshUpdateModifier>, Ptr)
        ),
        entry!(0x00c2_17f0, fn_00c217f0() -> Ptr<NiPSysMeshUpdateModifier>),
        entry!(
            0x00c2_1890,
            ni_p_sys_mesh_update_modifier_create_clone(
                Ptr<NiPSysMeshUpdateModifier>,
                Ptr,
            )
                -> Ptr<NiPSysMeshUpdateModifier>
        ),
        entry!(0x00c21720, ni_p_sys_mesh_update_modifier_get_r_t_t_i() -> u32),
        entry!(
            0x00c21840,
            ni_p_sys_mesh_update_modifier_get_viewer_strings(Ptr, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    #[test]
    fn ni_p_sys_mesh_update_modifier_get_viewer_strings_appends_the_class_name_growing_a_full_array(
    ) {
        const PARENT: u32 = 0x00c3_19e0;
        const NAME_POINTER: u32 = 0x0120_2530;
        const NI_GET_VIEWER_STRING: u32 = 0x00a7_d330;
        const GROW: u32 = 0x0096_ad30;
        const SET_AT: u32 = 0x0096_ae90;
        // (capacity, size): room left, then a full array.
        for (capacity, size) in [(4u16, 2u16), (3, 3)] {
            let mut e = Engine::new();
            e.map(NAME_POINTER, 4);
            e.set_global(NAME_POINTER, 0x5151_0001u32);
            let this = Ptr::new(e.mem.alloc(0x40));
            let strings = e.mem.alloc(0x10);
            e.mem.set_u16(strings + 8, capacity);
            e.mem.set_u16(strings + 0xa, size);
            e.mem.set_u16(strings + 0xe, 5);
            e.register(PARENT, |_, _| Ret::default());
            e.register(NI_GET_VIEWER_STRING, |_, _| Ret {
                eax: 0x7777_0001,
                ..Ret::default()
            });
            e.register(GROW, |_, _| Ret::default());
            let stored = Rc::new(Cell::new(0u32));
            let seen = stored.clone();
            e.register_double(SET_AT, move |e, a| {
                seen.set(e.mem.u32(a[2]));
                Ret::default()
            });
            e.call_log = Some(vec![]);
            ni_p_sys_mesh_update_modifier_get_viewer_strings(&mut e, this, Ptr::new(strings));
            let log = e.call_log.take().unwrap();
            let order: Vec<u32> = log.iter().map(|c| c.0).collect();
            let size = u32::from(size);
            if capacity > 3 {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, SET_AT]);
            } else {
                assert_eq!(order, [PARENT, NI_GET_VIEWER_STRING, GROW, SET_AT]);
                let grow = log.iter().find(|c| c.0 == GROW).unwrap();
                assert_eq!(grow.1, [strings, 5 + size]);
            }
            let parent = log.iter().find(|c| c.0 == PARENT).unwrap();
            assert_eq!(parent.1, [this.addr(), strings]);
            let name = log.iter().find(|c| c.0 == NI_GET_VIEWER_STRING).unwrap();
            assert_eq!(name.1, [0x5151_0001]);
            let set = log.iter().find(|c| c.0 == SET_AT).unwrap();
            assert_eq!(&set.1[..2], [strings, size]);
            assert_eq!(stored.get(), 0x7777_0001);
        }
    }

    #[test]
    fn ni_p_sys_mesh_update_modifier_get_r_t_t_i_returns_the_constant() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00c2_1720, &args![]).u32(), 0x0120_2530);
    }
}

#[cfg(test)]
mod mesh_tests {
    use super::*;
    use crate::units::niparticle::nipsysmodifier::test_stream;
    use crate::units::niparticle::nipsysmodifier::test_stream::calls_to;

    /// Test slots for the meshes' vtable.
    const DELETE_THIS: u32 = 0x0f80_0000;
    const MESH_SLOT: u32 = 0x0f80_0010;
    const MESH_IS_EQUAL: u32 = 0x0f80_0020;

    /// A mesh (reference count `count`) whose vtable routes every slot used
    /// here to `MESH_SLOT`, `IsEqual` to `MESH_IS_EQUAL` and `DeleteThis` to
    /// `DELETE_THIS`.
    fn mesh(e: &mut Engine, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for slot in [
            CREATE_CLONE_SLOT,
            REGISTER_STREAMABLES_SLOT,
            PROCESS_CLONE_SLOT,
            AV_OBJECT_UPDATE_SLOT,
            MESH_TARGET_SLOT,
            MESHES_SET_AT_SLOT,
        ] {
            e.mem.set_u32(vtable + slot, MESH_SLOT);
        }
        e.mem.set_u32(vtable + IS_EQUAL_SLOT, MESH_IS_EQUAL);
        e.mem.set_u32(vtable + DELETE_THIS_SLOT, DELETE_THIS);
        let object = e.mem.alloc(0x70);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn doubles(e: &mut Engine) {
        test_stream::interlocked(e);
        e.register(DELETE_THIS, |_, _| Ret::default());
        e.register(MESH_SLOT, |_, _| Ret::default());
    }

    /// A modifier whose meshes array holds `elements`.
    fn modifier(e: &mut Engine, elements: &[u32]) -> Ptr<NiPSysMeshUpdateModifier> {
        let this = e.new_object::<NiPSysMeshUpdateModifier>();
        let base = e.mem.alloc(4 * elements.len() as u32 + 4);
        for (i, element) in elements.iter().enumerate() {
            e.mem.set_u32(base + 4 * i as u32, *element);
        }
        let array = meshes(this);
        e.set(array, NiTArray::m_pBase, base);
        e.set(array, NiTArray::m_usMaxSize, elements.len() as u16);
        e.set(array, NiTArray::m_usSize, elements.len() as u16);
        e.set(array, NiTArray::m_usGrowBy, 1);
        this
    }

    #[test]
    fn load_binary_sets_the_order_and_reads_the_links() {
        let mut e = Engine::new();
        e.register(MODIFIER_LOAD_BINARY, |_, _| Ret::default());
        e.register(READ_MULTIPLE_LINK_IDS, |_, _| Ret::default());
        let this = modifier(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00c2_0f00, &args![this, 0x7000u32]);
        assert_eq!(e.mem.u32(this.addr() + 0xc), 0x1964);
        assert_eq!(calls_to(&e, READ_MULTIPLE_LINK_IDS), vec![vec![0x7000]]);
    }

    #[test]
    fn process_clone_and_register_streamables_visit_the_meshes() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(MODIFIER_PROCESS_CLONE, |_, _| Ret::default());
        e.register(MODIFIER_REGISTER_STREAMABLES, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let m = mesh(&mut e, 1);
        let this = modifier(&mut e, &[m, 0, m]);
        e.call_log = Some(vec![]);
        e.call(0x00c2_0f30, &args![this, 0x6000u32]);
        assert_eq!(calls_to(&e, MESH_SLOT), vec![vec![m, 0x6000]; 2]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c2_0f80, &args![this, 0x7000u32]).bool());
        assert_eq!(calls_to(&e, MESH_SLOT).len(), 2);
        e.register(MODIFIER_REGISTER_STREAMABLES, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c2_0f80, &args![this, 0x7000u32]).bool());
        assert!(calls_to(&e, MESH_SLOT).is_empty());
    }

    #[test]
    fn remove_last_hands_out_the_last_element() {
        let mut e = Engine::new();
        doubles(&mut e);
        let m = mesh(&mut e, 1);
        let this = modifier(&mut e, &[0, m]);
        let array = meshes(this);
        e.set(array, NiTArray::m_usESize, 1);
        let result = e.mem.alloc(4);
        assert_eq!(e.call(0x00c2_0fd0, &args![array, result]).u32(), result);
        assert_eq!(e.mem.u32(result), m);
        assert_eq!(e.get(array, NiTArray::m_usSize), 1);
        assert_eq!(e.get(array, NiTArray::m_usESize), 0);
        assert_eq!(e.mem.u32(e.get(array, NiTArray::m_pBase) + 4), 0);
        // Held by the result only.
        assert_eq!(e.mem.u32(m + 4), 1);
        // A null last element, then an empty array.
        e.call(0x00c2_0fd0, &args![array, result]);
        assert_eq!(e.mem.u32(result), 0);
        assert_eq!(e.get(array, NiTArray::m_usSize), 0);
        e.mem.set_u32(result, 0x55);
        e.call(0x00c2_0fd0, &args![array, result]);
        assert_eq!(e.mem.u32(result), 0);
    }

    #[test]
    fn update_places_rotates_scales_and_updates_each_particle_mesh() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(MATRIX_MAKE_ROTATION, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        let m = mesh(&mut e, 1);
        let data = e.mem.alloc(0x98);
        let node = e.mem.alloc(0xb0);
        let children = e.mem.alloc(8);
        e.mem.set_u32(children, m);
        e.mem.set_u32(node + NODE_CHILDREN, children);
        e.mem.set_u32(data + DATA_PARTICLE_MESHES, node);
        let infos = e.mem.alloc(INFO_SIZE);
        e.mem.set_u32(infos + INFO_AGE, 3.0f32.to_bits());
        e.mem.set_u32(data + DATA_INFOS, infos);
        let vertices = e.mem.alloc(12);
        e.mem.set_u32(vertices + 8, 5.0f32.to_bits());
        e.mem.set_u32(data + DATA_VERTICES, vertices);
        let sizes = e.mem.alloc(4);
        e.mem.set_u32(sizes, (-2.0f32).to_bits());
        e.mem.set_u32(data + DATA_SIZES, sizes);
        let radii = e.mem.alloc(4);
        e.mem.set_u32(radii, 1.5f32.to_bits());
        e.mem.set_u32(data + DATA_RADII, radii);
        let angles = e.mem.alloc(4);
        e.mem.set_u32(angles, 0.25f32.to_bits());
        let axes = e.mem.alloc(12);
        e.mem.set_u16(data + DATA_ACTIVE, 1);
        let this = modifier(&mut e, &[]);
        e.register_double(MESH_SLOT, move |e, a| {
            assert_eq!(e.mem.u32(a[1]), 3.0f32.to_bits());
            assert_eq!(e.mem.u8(a[1] + 4), 1);
            assert_eq!(a[2], 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c2_1080, &args![this, 0.0f32, data]);
        assert_eq!(e.mem.u32(m + OBJECT_TRANSLATION + 8), 5.0f32.to_bits());
        assert_eq!(e.mem.u32(m + OBJECT_SCALE), 3.0f32.to_bits());
        assert!(calls_to(&e, MATRIX_MAKE_ROTATION).is_empty());
        assert_eq!(calls_to(&e, MESH_SLOT).len(), 1);
        // With angles and axes the rotation is set.
        e.mem.set_u32(data + DATA_ROTATION_ANGLES, angles);
        e.mem.set_u32(data + DATA_ROTATION_AXES, axes);
        e.call_log = Some(vec![]);
        e.call(0x00c2_1080, &args![this, 0.0f32, data]);
        assert_eq!(calls_to(&e, MATRIX_MAKE_ROTATION)[0][1], 0.25f32.to_bits());
        assert_eq!(e.mem.u32(m + OBJECT_ROTATION), 0.25f32.to_bits());
    }

    #[test]
    fn save_binary_writes_the_count_and_links() {
        const SAVE_LINK: u32 = 0x0f80_0030;
        let mut e = Engine::new();
        e.register(MODIFIER_SAVE_BINARY, |_, _| Ret::default());
        e.register(SAVE_LINK, |_, _| Ret::default());
        let writes = test_stream::writes(&mut e);
        let stream = test_stream::new_output(&mut e);
        let vtable = e.mem.alloc(0x40);
        e.mem.set_u32(stream, vtable);
        e.mem.set_u32(vtable + STREAM_SAVE_LINK_ID, SAVE_LINK);
        let this = modifier(&mut e, &[0x11, 0, 0x33]);
        e.call_log = Some(vec![]);
        e.call(0x00c2_11c0, &args![this, stream]);
        assert_eq!(*writes.borrow(), vec![(vec![3, 0, 0, 0], 4)]);
        assert_eq!(
            calls_to(&e, SAVE_LINK),
            vec![vec![stream, 0x11], vec![stream, 0], vec![stream, 0x33]]
        );
    }

    #[test]
    fn is_equal_compares_the_meshes_pairwise() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(MODIFIER_IS_EQUAL, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register(MESH_IS_EQUAL, |e, a| Ret {
            eax: u32::from(e.mem.u32(a[0] + 8) == e.mem.u32(a[1] + 8)),
            ..Ret::default()
        });
        let m1 = mesh(&mut e, 1);
        let m2 = mesh(&mut e, 1);
        let a = modifier(&mut e, &[m1, 0]);
        let b = modifier(&mut e, &[m2, 0]);
        assert!(e.call(0x00c2_1230, &args![a, b]).bool());
        e.mem.set_u32(m2 + 8, 1);
        assert!(!e.call(0x00c2_1230, &args![a, b]).bool());
        let c = modifier(&mut e, &[m1, m1]);
        assert!(!e.call(0x00c2_1230, &args![a, c]).bool());
        assert!(!e.call(0x00c2_1230, &args![c, a]).bool());
        let d = modifier(&mut e, &[m1]);
        assert!(!e.call(0x00c2_1230, &args![a, d]).bool());
        let empty = modifier(&mut e, &[]);
        assert!(e.call(0x00c2_1230, &args![empty, empty]).bool());
        e.register(MODIFIER_IS_EQUAL, |_, _| Ret::default());
        assert!(!e.call(0x00c2_1230, &args![empty, empty]).bool());
    }

    /// Mesh data with one pool (an array of capacity 2) at index 0.
    fn pooled_data(e: &mut Engine) -> (u32, u32) {
        let data = e.mem.alloc(0x98);
        let pool = e.mem.alloc(0x10);
        let elements = e.mem.alloc(8);
        e.mem.set_u32(pool + 4, elements);
        e.mem.set_u16(pool + 8, 2);
        let pools = e.mem.alloc(4);
        e.mem.set_u32(pools, pool);
        e.mem.set_u32(data + DATA_POOLS, pools);
        e.mem.set_u16(data + DATA_POOL_COUNT, 1);
        (data, pool)
    }

    #[test]
    fn fill_pool_clones_the_mesh_into_every_slot() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (data, pool) = pooled_data(&mut e);
        let m = mesh(&mut e, 1);
        let clone = mesh(&mut e, 0);
        e.register_double(OBJECT_CLONE, move |_, _| Ret {
            eax: clone,
            ..Ret::default()
        });
        e.register(ARRAY_SET_AT, |e, a| {
            let element = e.mem.u32(a[2]);
            let base = e.mem.u32(a[0] + 4);
            e.mem.set_u32(base + 4 * a[1], element);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00c2_12b0, &args![data, 0u32, m]);
        let sets = calls_to(&e, ARRAY_SET_AT);
        assert_eq!(sets.len(), 2);
        assert_eq!(sets[1][..2], [pool, 1]);
        assert_eq!(e.mem.u32(e.mem.u32(pool + 4) + 4), clone);
        assert_eq!(calls_to(&e, OBJECT_CLONE), vec![vec![m]; 2]);
        // Past the pool count nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x00c2_12b0, &args![data, 1u32, m]);
        assert!(calls_to(&e, OBJECT_CLONE).is_empty());
    }

    #[test]
    fn remove_from_pool_assigns_the_pool_last_or_clears() {
        let mut e = Engine::new();
        doubles(&mut e);
        let (data, pool) = pooled_data(&mut e);
        let m = mesh(&mut e, 1);
        e.mem.set_u32(e.mem.u32(pool + 4), m);
        e.mem.set_u16(pool + 0xa, 1);
        e.register(POINTER_ASSIGN, |e, a| {
            let value = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], value);
            if value != 0 {
                let count = e.mem.u32(value + 4);
                e.mem.set_u32(value + 4, count + 1);
            }
            Ret::default()
        });
        let result = e.mem.alloc(4);
        e.call(0x00c2_1340, &args![data, 0u32, result]);
        assert_eq!(e.mem.u32(result), m);
        assert_eq!(e.mem.u16(pool + 0xa), 0);
        // Index past the pools: the result is released and cleared.
        e.call_log = Some(vec![]);
        e.call(0x00c2_1340, &args![data, 3u32, result]);
        assert_eq!(e.mem.u32(result), 0);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![m]]);
    }

    #[test]
    fn initialize_takes_a_pooled_or_cloned_mesh() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(START_CONTROLLERS, |_, _| Ret::default());
        let source = mesh(&mut e, 1);
        let clone = mesh(&mut e, 0);
        e.register_double(OBJECT_CLONE, move |_, _| Ret {
            eax: clone,
            ..Ret::default()
        });
        let this = modifier(&mut e, &[source]);
        let system = e.mem.alloc(0x100);
        e.mem.set_u32(this.addr() + 0x10, system);
        let data = e.mem.alloc(0x98);
        let node = mesh(&mut e, 1);
        e.mem.set_u32(data + DATA_PARTICLE_MESHES, node);
        let infos = e.mem.alloc(2 * INFO_SIZE);
        // Generation 5 is capped to the last mesh, 0.
        e.mem.set_u16(infos + INFO_SIZE + INFO_GENERATION, 5);
        e.mem.set_u32(data + DATA_INFOS, infos);
        e.call_log = Some(vec![]);
        e.call(0x00c2_1400, &args![this, data, 0x0007_0001u32]);
        assert_eq!(calls_to(&e, OBJECT_CLONE), vec![vec![source]]);
        assert_eq!(e.mem.u32(clone + OBJECT_FLAGS) & 1, 1);
        assert_eq!(calls_to(&e, START_CONTROLLERS), vec![vec![clone, 0]]);
        assert_eq!(
            calls_to(&e, MESH_SLOT),
            vec![vec![node, 1, clone], vec![clone, system + 0x9c]]
        );
        // Held once for the clone and released at the end.
        assert_eq!(e.mem.u32(clone + 4), 0);
        assert_eq!(calls_to(&e, DELETE_THIS), vec![vec![clone]]);
    }

    #[test]
    fn link_object_sizes_and_fills_the_meshes() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(MODIFIER_LINK_OBJECT, |_, _| Ret::default());
        e.register(GET_NUMBER_OF_LINK_IDS, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        let m = mesh(&mut e, 0);
        e.register_double(GET_OBJECT_FROM_LINK_ID, move |_, _| Ret {
            eax: m,
            ..Ret::default()
        });
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |e, a| {
            let element = e.mem.u32(a[2]);
            let count = e.mem.u32(element + 4);
            e.mem.set_u32(element + 4, count + 1);
            Ret::default()
        });
        let this = modifier(&mut e, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00c2_14c0, &args![this, 0x7000u32]);
        let array = meshes(this).addr();
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![array, 2]]);
        let sets = calls_to(&e, ARRAY_SET_AT);
        assert_eq!((sets[0][1], sets[1][1]), (0, 1));
        assert_eq!(e.mem.u32(m + 4), 2);
    }

    #[test]
    fn post_link_object_sets_generations_or_fills_pools() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(BASE_POST_LINK_OBJECT, |_, _| Ret::default());
        e.register(SET_NUM_GENERATIONS, |_, _| Ret::default());
        e.register(OBJECT_CLONE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |_, _| Ret::default());
        let (data, _) = pooled_data(&mut e);
        let system = e.mem.alloc(0x100);
        e.mem.set_u32(system + 0xb8, data);
        let m = mesh(&mut e, 1);
        let this = modifier(&mut e, &[m]);
        e.mem.set_u32(this.addr() + 0x10, system);
        let stream = e.mem.alloc(0x100);
        e.mem.set_u32(stream + STREAM_VERSION, VERSION_POOLS);
        e.call_log = Some(vec![]);
        e.call(0x00c2_1550, &args![this, stream]);
        assert_eq!(calls_to(&e, SET_NUM_GENERATIONS), vec![vec![data, 1]]);
        e.mem.set_u32(stream + STREAM_VERSION, VERSION_POOLS + 1);
        e.call_log = Some(vec![]);
        e.call(0x00c2_1550, &args![this, stream]);
        assert!(calls_to(&e, OBJECT_CLONE).is_empty());
        e.mem.set_u8(data + DATA_FILL_POOLS_ON_LOAD, 1);
        e.call(0x00c2_1550, &args![this, stream]);
        assert_eq!(calls_to(&e, OBJECT_CLONE), vec![vec![m]; 2]);
    }

    #[test]
    fn set_mesh_hands_back_the_replaced_one_or_grows() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(ARRAY_SET_AT, |e, a| {
            if a[1] < u32::from(e.mem.u16(a[0] + 8)) {
                let element = e.mem.u32(a[2]);
                let base = e.mem.u32(a[0] + 4);
                e.mem.set_u32(base + 4 * a[1], element);
            }
            Ret::default()
        });
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        let old = mesh(&mut e, 1);
        let new = mesh(&mut e, 0);
        let this = modifier(&mut e, &[old]);
        let result = e.mem.alloc(4);
        e.call(0x00c2_15d0, &args![this, 0u32, new, result]);
        assert_eq!(e.mem.u32(result), old);
        assert_eq!(e.mem.u32(old + 4), 2);
        assert_eq!(e.mem.u32(new + 4), 0);
        // Past the capacity: grown by the step plus the index, result null.
        e.call_log = Some(vec![]);
        e.call(0x00c2_15d0, &args![this, 3u32, 0u32, result]);
        assert_eq!(
            calls_to(&e, ARRAY_SET_SIZE),
            vec![vec![meshes(this).addr(), 4]]
        );
        assert_eq!(e.mem.u32(result), 0);
        assert_eq!(e.mem.u32(old + 4), 1);
    }

    #[test]
    fn destructor_deletes_the_elements_and_with_bit_0_the_object() {
        for flags in [0u32, 1] {
            let mut e = Engine::new();
            e.register(ARRAY_DELETE_ELEMENTS, |_, _| Ret::default());
            e.register(MODIFIER_DESTRUCTOR, |_, _| Ret::default());
            e.register(NI_DELETE, |_, _| Ret::default());
            let this = modifier(&mut e, &[0]);
            let base = e.get(meshes(this), NiTArray::m_pBase);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00c2_1730, &args![this, flags]).u32(), this.addr());
            assert_eq!(e.mem.u32(this.addr() + 0x18), ARRAY_VTABLE);
            assert_eq!(calls_to(&e, ARRAY_DELETE_ELEMENTS), vec![vec![base]]);
            assert_eq!(calls_to(&e, NI_DELETE).len(), flags as usize);
        }
    }

    #[test]
    fn clone_factory_and_copy_build_the_meshes() {
        let mut e = Engine::new();
        doubles(&mut e);
        e.register(NI_NEW, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(MODIFIER_CONSTRUCTOR, |_, _| Ret::default());
        e.register(MODIFIER_COPY_MEMBERS, |_, _| Ret::default());
        e.register(ARRAY_SET_SIZE, |e, a| {
            let base = e.mem.alloc(4 * a[1]);
            e.mem.set_u32(a[0] + 4, base);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(ARRAY_SET_AT, |e, a| {
            let element = e.mem.u32(a[2]);
            let base = e.mem.u32(a[0] + 4);
            e.mem.set_u32(base + 4 * a[1], element);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let copy = mesh(&mut e, 0);
        e.register_double(MESH_SLOT, move |_, _| Ret {
            eax: copy,
            ..Ret::default()
        });
        let made = e.call(0x00c2_17f0, &args![]).u32();
        assert_eq!(e.mem.u32(made), VTABLE);
        assert_eq!(e.mem.u32(made + 0x18), OBJECT_ARRAY_VTABLE);
        assert_eq!(e.mem.u16(made + 0x26), 1);
        let source = mesh(&mut e, 1);
        let this = modifier(&mut e, &[source]);
        e.call_log = Some(vec![]);
        let clone = e.call(0x00c2_1890, &args![this, 0x6000u32]).u32();
        assert_eq!(e.mem.u32(clone), VTABLE);
        assert_eq!(
            calls_to(&e, MODIFIER_COPY_MEMBERS),
            vec![vec![this.addr(), clone, 0x6000]]
        );
        assert_eq!(calls_to(&e, MESH_SLOT), vec![vec![source, 0x6000]]);
        let base = e.mem.u32(clone + 0x1c);
        assert_eq!(e.mem.u32(base), copy);
        e.register(NI_NEW, |_, _| Ret::default());
        assert_eq!(e.call(0x00c2_17f0, &args![]).u32(), 0);
    }
}
