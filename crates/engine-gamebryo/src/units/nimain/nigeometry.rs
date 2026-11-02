//! `NiMain/nigeometry.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::nimain::nimainsdm::{
    assign_ref_counted, interlocked_decrement, interlocked_increment, EMPTY_FIXED_STRING,
};

/// `NiGeometry::ms_RTTI` (the class's `NiRTTI` record).
const NI_GEOMETRY_RTTI: u32 = 0x011f_4acc;

// Translated from 00a80530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_geometry_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_GEOMETRY_RTTI)
}

/// `00a59dc0`: the jump target of `00a7fdb0`.
const JUMP_TARGET_00A7FDB0: u32 = 0x00a5_9dc0;

/// `00a5c6a0`: the jump target of `00a7fdc0`.
const JUMP_TARGET_00A7FDC0: u32 = 0x00a5_c6a0;

// Translated from 00a7fdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::SetBound` (Xbox PDB): a 5-byte jump to `00a59dc0` (engine map: `NiAVObject::SetBound`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SetBound@NiGeometry@@UAAXPAVBSNiBound@@@Z`.
pub fn ni_geometry_set_bound(e: &mut Engine, this: Ptr, arg: Ptr) {
    e.call(JUMP_TARGET_00A7FDB0, &args![this, arg]);
}

// Translated from 00a7fdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::PostLinkObject` (Xbox PDB): a 5-byte jump to `00a5c6a0` (engine map: `NiObjectNET::PostLinkObject`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?PostLinkObject@NiGeometry@@UAAXAAVNiStream@@@Z`.
pub fn ni_geometry_post_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A7FDC0, &args![this, stream]);
}

/// `NiAVObject::NiAVObject` (engine map, `00a5aa20`, `thiscall`, no stack words).
const NI_AV_OBJECT_CONSTRUCT: u32 = 0x00a5_aa20;
/// `NiPropertyState::NiPropertyState` (engine map, `00a89ab0`, `thiscall`, no stack words).
const NI_PROPERTY_STATE_CONSTRUCT: u32 = 0x00a8_9ab0;
/// Imported `InterlockedIncrement` (import slot).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `NiGeometry`'s vtable (`0109e04c`; its RTTI type descriptor reads `.?AVNiGeometry@@`).
const NI_GEOMETRY_VTABLE: u32 = 0x0109_e04c;

// Translated from 00a804e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::NiGeometry` (Xbox PDB), `thiscall` with one stack word (the geometry data).
/// Runs the `NiAVObject` constructor (`00a5aa20`), stores the vtable `0109e04c`, constructs
/// `m_PropertyState` at +0x9c (`00a89ab0`), keeps the data in `m_spModelData` (+0xb8; its
/// reference count at +4 incremented through `InterlockedIncrement` when it is not null) and
/// clears `m_spSkinInstance` (+0xbc) and `m_spShader` (+0xc0). PC offsets are the Xbox PDB's
/// less 0x24 (the PC `NiAVObject` is 0x9c bytes). Answers `this`.
pub fn ni_geometry_ni_geometry(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_AV_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, NI_GEOMETRY_VTABLE);
    e.call(NI_PROPERTY_STATE_CONSTRUCT, &args![base + 0x9c]);
    e.mem.set_u32(base + 0xb8, data.addr());
    if data.addr() != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![data.addr() + 4]);
    }
    e.mem.set_u32(base + 0xbc, 0);
    e.mem.set_u32(base + 0xc0, 0);
    this
}

/// The spin lock at `011f42a0` (owner word, then the entry count at `011f42a4`) the property
/// walks hold: entered through `0040fbf0` (`thiscall`, `RET 4`, here with a spin
/// limit of 0), left inline by a plain decrement of the count that clears the owner at zero.
const PROPERTY_LOCK: u32 = 0x011f_42a0;
const PROPERTY_LOCK_COUNT: u32 = 0x011f_42a4;
const SPIN_LOCK_ENTER_0040FBF0: u32 = 0x0040_fbf0;
/// The memory manager (`011f6238`) and its allocate `00aa3e40` (`thiscall`, one stack word).
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// The default bound `UpdateWorldBound` reads when +0x20 is null (`011f4288`, 16 bytes) and
/// the centre it stores into a new bound (`011f426c`, three words).
const DEFAULT_BOUND_011F4288: u32 = 0x011f_4288;
const BOUND_CENTER_011F426C: u32 = 0x011f_426c;
/// Engine-map names: `NiAVObject::UpdatePropertiesDownward` (`00a5a140`, `RET 4`),
/// `NiAVObject::SetSelectiveUpdateFlags` (`00a5a450`, `RET 0xc`), `NiAVObject::LoadBinary`
/// (`00a5a510`, `RET 4`), `NiAVObject::LinkObject` (`00a5a7f0`, `RET 4`),
/// `NiAVObject::ProcessClone` (`00a59ec0`, `RET 4`), `NiPropertyState::~NiPropertyState`
/// (`00a89890`), `NiBound::Update` (`00a7fdd0`, `RET 8`), `NiStream::LoadFixedString`
/// (`00a64df0`, `RET 4`), `NiStream::LoadCStringAsFixedString` (`00a64ea0`, `RET 4`),
/// `NiStream::GetObjectFromLinkID` (`00a63430`), `NiAVObject::SetCollisionObject`
/// (`0062bc90`, `RET 4`) and `NiTPointerListBase<..NiPointer<NiProperty>>::AddHead`
/// (`00439fd0`, `RET 4`).
const AV_OBJECT_UPDATE_PROPERTIES_DOWNWARD: u32 = 0x00a5_a140;
const AV_OBJECT_SET_SELECTIVE_UPDATE_FLAGS: u32 = 0x00a5_a450;
const AV_OBJECT_LOAD_BINARY: u32 = 0x00a5_a510;
const AV_OBJECT_LINK_OBJECT: u32 = 0x00a5_a7f0;
const AV_OBJECT_PROCESS_CLONE: u32 = 0x00a5_9ec0;
const PROPERTY_STATE_DESTRUCT: u32 = 0x00a8_9890;
const BOUND_UPDATE: u32 = 0x00a7_fdd0;
const STREAM_LOAD_FIXED_STRING: u32 = 0x00a6_4df0;
const STREAM_LOAD_C_STRING_AS_FIXED_STRING: u32 = 0x00a6_4ea0;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const AV_OBJECT_SET_COLLISION_OBJECT: u32 = 0x0062_bc90;
const PROPERTY_LIST_ADD_HEAD: u32 = 0x0043_9fd0;
/// Unnamed callees: the `NiAVObject` destructor body `00a5a390`, the property-state copy
/// `00a89900` (`RET 4`), the `NiObjectNET` member copy `00a5c290` (`RET 8`), the object
/// clone `00a5d3f0` (`RET 4`), the skin instance's bound step `00a86530` (`RET 4`), the
/// bound steps `00a7fea0` (`RET 4`) and `00a92660` (`RET 4`, a byte answer), the stream's
/// link-id read `00a645b0`, the `cdecl` stream read `00a26df0` (stream, out), the clone
/// map's lookup `00a53c30` (`RET 8`), the `NiPointer` assignment `0066b0d0` (`RET 4`), the
/// allocator `00aa13e0` (`cdecl`), the constructor `00a8f440`, the getter `00a62660` (its
/// engine-map name is a folded one), the name setter `00a5b950` (`RET 4`),
/// `NiD3DShaderProgram::GetName` (engine map, `00cc0760`) and `00ba3760` (`RET 4`).
const AV_OBJECT_DESTRUCT_00A5A390: u32 = 0x00a5_a390;
const PROPERTY_STATE_COPY_00A89900: u32 = 0x00a8_9900;
const OBJECT_NET_COPY_MEMBERS_00A5C290: u32 = 0x00a5_c290;
const OBJECT_CLONE_00A5D3F0: u32 = 0x00a5_d3f0;
const SKIN_INSTANCE_BOUND_00A86530: u32 = 0x00a8_6530;
const BOUND_STEP_00A7FEA0: u32 = 0x00a7_fea0;
const BOUND_COMPARE_00A92660: u32 = 0x00a9_2660;
const STREAM_READ_LINK_ID_00A645B0: u32 = 0x00a6_45b0;
const STREAM_READ_00A26DF0: u32 = 0x00a2_6df0;
const CLONE_MAP_LOOKUP_00A53C30: u32 = 0x00a5_3c30;
const POINTER_ASSIGN_0066B0D0: u32 = 0x0066_b0d0;
const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const CONSTRUCT_00A8F440: u32 = 0x00a8_f440;
const GETTER_00A62660: u32 = 0x00a6_2660;
const SET_NAME_00A5B950: u32 = 0x00a5_b950;
const SHADER_PROGRAM_GET_NAME: u32 = 0x00cc_0760;
const SETTER_00BA3760: u32 = 0x00ba_3760;
/// The `NiRTTI` record `ProcessClone` compares the shader's `GetRTTI` with.
const SHADER_RTTI_011F5D08: u32 = 0x011f_5d08;
/// `NiStream`: `m_uiNifFileVersion` (+0xd8) and `m_pkIstr` (+0x24c); `NiBinaryStream`: the
/// position (+4) and `m_pfnRead` (+8) (Xbox PDB names).
const STREAM_VERSION: u32 = 0xd8;
const STREAM_INPUT: u32 = 0x24c;

/// Enters the property lock (`0040fbf0` with 0).
fn enter_property_lock(e: &mut Engine) {
    e.call(SPIN_LOCK_ENTER_0040FBF0, &args![PROPERTY_LOCK, 0u32]);
}

/// Leaves the property lock: the count drops by one and the owner is cleared at zero.
fn leave_property_lock(e: &mut Engine) {
    let count = e.mem.u32(PROPERTY_LOCK_COUNT).wrapping_sub(1);
    e.mem.set_u32(PROPERTY_LOCK_COUNT, count);
    if count == 0 {
        e.mem.set_u32(PROPERTY_LOCK, 0);
    }
}

/// Releases a counted object (`InterlockedDecrement` on +4, its slot +4 at zero).
fn release(e: &mut Engine, object: u32) {
    if interlocked_decrement(e, object.wrapping_add(4)) == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// One read through the stream's `m_pfnRead` into `buffer`, the element size held in
/// `size_slot`, the position advanced by the result.
fn stream_read(e: &mut Engine, stream: u32, buffer: u32, bytes: u32, size_slot: u32, size: u32) {
    let binary = e.mem.u32(stream + STREAM_INPUT);
    let function = e.mem.u32(binary.wrapping_add(8));
    e.mem.set_u32(size_slot, size);
    let moved = e
        .call(function, &args![binary, buffer, bytes, size_slot, 1u32])
        .u32();
    let position = e.mem.u32(binary.wrapping_add(4)).wrapping_add(moved);
    e.mem.set_u32(binary.wrapping_add(4), position);
}

// Translated from 00a7fd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::OnVisible` (Xbox PDB), `thiscall`, `RET 4`: hands `this` to the culler's
/// slot +0x4c (a tail jump with `this` in the argument slot).
pub fn ni_geometry_on_visible(e: &mut Engine, this: Ptr, culler: Ptr) {
    e.vcall(culler.addr(), 0x4c, &args![this]);
}

// Translated from 00a80540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeometry.cpp`): the `NiGeometry` constructor without
/// data (`thiscall`, no stack words). Runs the `NiAVObject` constructor (`00a5aa20`), stores
/// the vtable `0109e04c`, constructs `m_PropertyState` at +0x9c (`00a89ab0`) and clears
/// `m_spModelData` (+0xb8), `m_spSkinInstance` (+0xbc) and `m_spShader` (+0xc0). Answers
/// `this`.
pub fn fn_00a80540(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(NI_AV_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, NI_GEOMETRY_VTABLE);
    e.call(NI_PROPERTY_STATE_CONSTRUCT, &args![base + 0x9c]);
    e.mem.set_u32(base + 0xb8, 0);
    e.mem.set_u32(base + 0xbc, 0);
    e.mem.set_u32(base + 0xc0, 0);
    this
}

// Translated from 00a80610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::RenderImmediate` (Xbox PDB), `thiscall`, `RET 4`: stores the address of
/// `m_PropertyState` (+0x9c) at +0xc of `renderer`, then calls slot +0xa8 of each object in
/// the chain that starts at `m_spControllers` (+0xc) and goes on through +0x30.
pub fn ni_geometry_render_immediate(e: &mut Engine, this: Ptr, renderer: Ptr) {
    let base = this.addr();
    e.mem.set_u32(renderer.addr() + 0xc, base + 0x9c);
    let mut controller = e.mem.u32(base + 0xc);
    while controller != 0 {
        e.vcall(controller, 0xa8, &args![]);
        controller = e.mem.u32(controller + 0x30);
    }
}

// Translated from 00a808f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::SetSelectiveUpdateFlags` (Xbox PDB), `thiscall`, `RET 0xc`: runs
/// `NiAVObject::SetSelectiveUpdateFlags` (`00a5a450`) with the same words; with a skin
/// instance (+0xbc) it then sets bit 4 of the flags (+0x30), the byte at `selective_update`
/// to 1, bit 2 of the flags and the byte at `rigid` to 0.
pub fn ni_geometry_set_selective_update_flags(
    e: &mut Engine,
    this: Ptr,
    selective_update: Ptr,
    selective_update_transforms: u32,
    rigid: Ptr,
) {
    let base = this.addr();
    e.call(
        AV_OBJECT_SET_SELECTIVE_UPDATE_FLAGS,
        &args![this, selective_update, selective_update_transforms, rigid],
    );
    if e.mem.u32(base + 0xbc) != 0 {
        let flags = e.mem.u32(base + 0x30) | 4;
        e.mem.set_u32(base + 0x30, flags);
        e.mem.set_u8(selective_update.addr(), 1);
        let flags = e.mem.u32(base + 0x30) | 2;
        e.mem.set_u32(base + 0x30, flags);
        e.mem.set_u8(rigid.addr(), 0);
    }
}

// Translated from 00a5abe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nigeometry.cpp`; `thiscall`, `RET 0xc`: the copy, the
/// property list, the cloning process). Under the property lock, walks `list` from its tail
/// (+4) back through each node's +4; for each node's property (+8) whose type (slot +0x8c)
/// is below 7, clones it (`00a5d3f0`) and, when the clone's type is below 7 too, holds it in
/// the `list` argument's own slot (counted) while `AddHead` (`00439fd0`) puts it on the
/// copy's list (+0x24), then releases that hold.
pub fn fn_00a5abe0(e: &mut Engine, _unused_this: Ptr, copy: Ptr, list: Ptr, cloning: Ptr) {
    enter_property_lock(e);
    let mut node = e.mem.u32(list.addr() + 4);
    while node != 0 {
        let property = e.mem.u32(node + 8);
        node = e.mem.u32(node + 4);
        if property == 0 || e.vcall(property, 0x8c, &args![]).i32() >= 7 {
            continue;
        }
        let clone = e
            .call(OBJECT_CLONE_00A5D3F0, &args![property, cloning])
            .u32();
        if e.vcall(clone, 0x8c, &args![]).i32() >= 7 {
            continue;
        }
        interlocked_increment(e, clone.wrapping_add(4));
        e.with_stack(4, |e, held| {
            e.mem.set_u32(held.addr(), clone);
            e.call(
                PROPERTY_LIST_ADD_HEAD,
                &args![copy.addr() + 0x24, held.addr()],
            );
        });
        release(e, clone);
    }
    leave_property_lock(e);
}

// Translated from 00a5ac90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names this `NiGeometry::CopyMembers`, but the body touches only
/// `NiAVObject` fields (`thiscall`, `RET 8`: the copy, the cloning process): the
/// `NiObjectNET` copy `00a5c290`, the flags (+0x30) with bit 0x80000 set, the 13 words of
/// the local transform (+0x34), the properties (`00a5abe0` with the list at +0x24) when the
/// list's count (+0x2c) is not zero, and a clone (slot +0x48) of the collision object (+0x1c)
/// given to `SetCollisionObject` (`0062bc90`).
pub fn fn_00a5ac90(e: &mut Engine, this: Ptr, copy: Ptr, cloning: Ptr) {
    let base = this.addr();
    let dest = copy.addr();
    e.call(
        OBJECT_NET_COPY_MEMBERS_00A5C290,
        &args![this, copy, cloning],
    );
    let flags = e.mem.u32(base + 0x30) | 0x80000;
    e.mem.set_u32(dest + 0x30, flags);
    for index in 0..0xdu32 {
        let word = e.mem.u32(base + 0x34 + 4 * index);
        e.mem.set_u32(dest + 0x34 + 4 * index, word);
    }
    if e.mem.u32(base + 0x2c) != 0 {
        fn_00a5abe0(e, this, copy, Ptr::new(base + 0x24), cloning);
    }
    let collision = e.mem.u32(base + 0x1c);
    if collision != 0 {
        let clone = e.vcall(collision, 0x48, &args![cloning]).u32();
        e.call(AV_OBJECT_SET_COLLISION_OBJECT, &args![copy, clone]);
    }
}

// Translated from 00a80580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::~NiGeometry` (Xbox PDB; `thiscall`, no stack words): stores the vtable
/// `0109e04c`, releases `m_spModelData` (+0xb8) and clears it, then the member destructors:
/// `m_spSkinInstance` (+0xbc) and `m_spModelData` again (each released when set),
/// `m_PropertyState` (`00a89890` on +0x9c), and ends in the `NiAVObject` destructor body
/// `00a5a390`.
pub fn ni_geometry_destructor(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, NI_GEOMETRY_VTABLE);
    let data = e.mem.u32(base + 0xb8);
    if data != 0 {
        release(e, data);
        e.mem.set_u32(base + 0xb8, 0);
    }
    for offset in [0xbcu32, 0xb8] {
        let object = e.mem.u32(base + offset);
        if object != 0 {
            release(e, object);
        }
    }
    e.call(PROPERTY_STATE_DESTRUCT, &args![base + 0x9c]);
    e.call(AV_OBJECT_DESTRUCT_00A5A390, &args![this]);
}

// Translated from 00a80680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::SetModelData` (Xbox PDB), `thiscall`, `RET 4`: nothing for null data.
/// Otherwise, with a collision object (+0x1c): its slot +0x94, then its slot +0x90 with a
/// local of a `float` 0.0 and five zero bytes; then `data` becomes `m_spModelData` (+0xb8,
/// the old one released, the new one counted, nothing when it is already there).
pub fn ni_geometry_set_model_data(e: &mut Engine, this: Ptr, data: Ptr) {
    let base = this.addr();
    if data.addr() == 0 {
        return;
    }
    if e.mem.u32(base + 0x1c) != 0 {
        let collision = e.mem.u32(base + 0x1c);
        e.vcall(collision, 0x94, &args![]);
        let collision = e.mem.u32(base + 0x1c);
        e.with_stack(0xc, |e, local| {
            e.mem.set_u32(local.addr(), 0.0f32.to_bits());
            for offset in 4..9u32 {
                e.mem.set_u8(local.addr() + offset, 0);
            }
            e.vcall(collision, 0x90, &args![local.addr()]);
        });
    }
    assign_ref_counted(e, base + 0xb8, data.addr());
}

// Translated from 00a80710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::UpdateWorldBound` (Xbox PDB; `thiscall`, no stack words). Copies the world
/// bound (+0x20, or the default at `011f4288` when null) into a local; unless bit 19 of the
/// flags (+0x30) is set with bit 13 clear, goes on only when that radius (+0xc) is exactly
/// 0.0. Without a bound one is allocated (16 bytes) and set through slot +0x78, with the
/// centre at `011f426c` and radius 0.0. Then with a skin instance (+0xbc) `00a86530` and
/// `00a7fea0` with the world transform (+0x68), else `NiBound::Update` from the model
/// data's bound (+0x10) and the world transform. When bit 22 of the flags is set, it stays
/// set only if `00a92660` answers true for the old bound.
pub fn ni_geometry_update_world_bound(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let bound = e.mem.u32(base + 0x20);
    let source = if bound == 0 {
        DEFAULT_BOUND_011F4288
    } else {
        bound
    };
    e.with_stack(0x10, |e, old| {
        for offset in [0u32, 4, 8, 0xc] {
            let word = e.mem.u32(source + offset);
            e.mem.set_u32(old.addr() + offset, word);
        }
        let flags = e.mem.u32(base + 0x30);
        let forced = (flags >> 19) & 1 != 0 && (flags >> 13) & 1 == 0;
        if !forced {
            let radius = e.mem.f32(old.addr() + 0xc);
            // FLDZ; FCOMP radius; TEST AH,0x44; JP: on only for an ordered equal (a NaN
            // radius returns).
            if radius != 0.0 {
                return;
            }
        }
        if bound == 0 {
            let block = e
                .call(
                    MEMORY_MANAGER_ALLOCATE,
                    &args![MEMORY_MANAGER_OBJECT, 0x10u32],
                )
                .u32();
            e.vcall(base, 0x78, &args![block]);
            let bound = e.mem.u32(base + 0x20);
            for offset in [0u32, 4, 8] {
                let word = e.mem.u32(BOUND_CENTER_011F426C + offset);
                e.mem.set_u32(bound + offset, word);
            }
            e.mem.set_u32(bound + 0xc, 0.0f32.to_bits());
        }
        let skin = e.mem.u32(base + 0xbc);
        if skin != 0 {
            let bound = e.mem.u32(base + 0x20);
            e.call(SKIN_INSTANCE_BOUND_00A86530, &args![skin, bound]);
            let bound = e.mem.u32(base + 0x20);
            e.call(BOUND_STEP_00A7FEA0, &args![bound, base + 0x68]);
        } else {
            let data = e.mem.u32(base + 0xb8);
            let bound = e.mem.u32(base + 0x20);
            e.call(
                BOUND_UPDATE,
                &args![bound, data.wrapping_add(0x10), base + 0x68],
            );
        }
        let flags = e.mem.u32(base + 0x30);
        if (flags >> 22) & 1 != 0 {
            let bound = e.mem.u32(base + 0x20);
            if e.call(BOUND_COMPARE_00A92660, &args![bound, old.addr()])
                .bool()
            {
                e.mem.set_u32(base + 0x30, flags | 0x40_0000);
            } else {
                e.mem.set_u32(base + 0x30, flags & 0xffbf_ffff);
            }
        }
    });
}

// Translated from 00a80820 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::UpdatePropertiesDownward` (Xbox PDB), `thiscall`, `RET 4`:
/// `NiAVObject::UpdatePropertiesDownward` (`00a5a140`), then under the property lock the
/// state is copied into `m_PropertyState` (`00a89900` on +0x9c) and each property of the
/// list (head at +0x24, next at +0 of each node, the property at +8) whose type (slot +0x8c)
/// is at most 7 is put in the state's slot for that type (+0x9c + 4 * type, counted); then
/// slot +0x94 (`SetMaterialNeedsUpdate`) with 1.
pub fn ni_geometry_update_properties_downward(e: &mut Engine, this: Ptr, state: Ptr) {
    let base = this.addr();
    e.call(AV_OBJECT_UPDATE_PROPERTIES_DOWNWARD, &args![this, state]);
    enter_property_lock(e);
    e.call(PROPERTY_STATE_COPY_00A89900, &args![base + 0x9c, state]);
    let mut node = e.mem.u32(base + 0x24);
    while node != 0 {
        let property = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if property == 0 || e.vcall(property, 0x8c, &args![]).i32() > 7 {
            continue;
        }
        let kind = e.vcall(property, 0x8c, &args![]).u32();
        let slot = base.wrapping_add(kind.wrapping_mul(4)).wrapping_add(0x9c);
        assign_ref_counted(e, slot, property);
    }
    leave_property_lock(e);
    e.vcall(base, 0x94, &args![1u32]);
}

// Translated from 00a80a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::ProcessClone` (Xbox PDB), `thiscall`, `RET 4`. After
/// `NiAVObject::ProcessClone` the clone of `this` is looked up in the cloning process's map
/// (`00a53c30`, the result in the argument's own slot). A skin instance (+0xbc) already
/// cloned is given to the clone's +0xbc (`0066b0d0`); otherwise its clone (slot +0x48) is,
/// and its slot +0x68 (`ProcessClone`) runs. A shader (+0xc0) whose `GetRTTI` (slot +8) is
/// `011f5d08` gets a new 0x14-byte object (`00aa13e0`, `00a8f440`) named like it
/// (`00a62660`, `00a5b950`) with its program name (`00cc0760`, `00ba3760`) as the clone's
/// shader; any other shader is shared: stored in the clone, given the clone through its slot
/// +0x50, and under the property lock the clone's slot +0xb0 gets `m_PropertyState` (+0x9c).
pub fn ni_geometry_process_clone(e: &mut Engine, this: Ptr, cloning: Ptr) {
    let base = this.addr();
    e.call(AV_OBJECT_PROCESS_CLONE, &args![this, cloning]);
    e.with_stack(4, |e, slot| {
        let map = e.mem.u32(cloning.addr());
        e.call(CLONE_MAP_LOOKUP_00A53C30, &args![map, base, slot.addr()]);
        let clone = e.mem.u32(slot.addr());
        let skin = e.mem.u32(base + 0xbc);
        if skin != 0 {
            let map = e.mem.u32(cloning.addr());
            if e.call(CLONE_MAP_LOOKUP_00A53C30, &args![map, skin, slot.addr()])
                .bool()
            {
                let found = e.mem.u32(slot.addr());
                e.call(POINTER_ASSIGN_0066B0D0, &args![clone + 0xbc, found]);
            } else {
                let skin = e.mem.u32(base + 0xbc);
                let copy = e.vcall(skin, 0x48, &args![cloning]).u32();
                e.call(POINTER_ASSIGN_0066B0D0, &args![clone + 0xbc, copy]);
                let skin = e.mem.u32(base + 0xbc);
                e.vcall(skin, 0x68, &args![cloning]);
            }
        }
        let shader = e.mem.u32(base + 0xc0);
        if shader == 0 {
            return;
        }
        if e.vcall(shader, 8, &args![]).u32() == SHADER_RTTI_011F5D08 {
            let block = e.call(ALLOCATE_00AA13E0, &args![0x14u32]).u32();
            let made = if block != 0 {
                e.call(CONSTRUCT_00A8F440, &args![block]).u32()
            } else {
                0
            };
            let shader = e.mem.u32(base + 0xc0);
            let name = e.call(GETTER_00A62660, &args![shader]).u32();
            e.call(SET_NAME_00A5B950, &args![made, name]);
            let shader = e.mem.u32(base + 0xc0);
            let program = e.call(SHADER_PROGRAM_GET_NAME, &args![shader]).u32();
            e.call(SETTER_00BA3760, &args![made, program]);
            e.mem.set_u32(clone + 0xc0, made);
        } else {
            let shader = e.mem.u32(base + 0xc0);
            e.mem.set_u32(clone + 0xc0, shader);
            let shader = e.mem.u32(base + 0xc0);
            e.vcall(shader, 0x50, &args![clone]);
            enter_property_lock(e);
            e.vcall(clone, 0xb0, &args![base + 0x9c]);
            leave_property_lock(e);
        }
    });
}

// Translated from 00a80b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::LinkObject` (Xbox PDB), `thiscall`, `RET 4`: `NiAVObject::LinkObject`, then
/// the objects of the next two link ids (`GetObjectFromLinkID`) become `m_spModelData`
/// (+0xb8) and `m_spSkinInstance` (+0xbc), each counted.
pub fn ni_geometry_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    let base = this.addr();
    e.call(AV_OBJECT_LINK_OBJECT, &args![this, stream]);
    for offset in [0xb8u32, 0xbc] {
        let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        assign_ref_counted(e, base + offset, object);
    }
}

/// Releases a fixed-string handle held in a local unless it is the empty string's.
fn release_local_string(e: &mut Engine, handle: u32) {
    if handle != e.mem.u32(EMPTY_FIXED_STRING) {
        interlocked_decrement(e, handle.wrapping_sub(8));
    }
}

// Translated from 00a80bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::LoadBinary` (Xbox PDB), `thiscall`, `RET 4`: `NiAVObject::LoadBinary`, two
/// link ids (`00a645b0`), then what the file has and the game drops: from 20.2.0.5 a count
/// (into the stream argument's slot, zeroed first) of fixed strings (`LoadFixedString` into
/// a local starting as the empty handle) each followed by a word, and one more word; from
/// 5.0.0.21 to 20.2.0.4 a byte that, when set, is followed by a string (`LoadFixedString`
/// from 20.1.0.1, `LoadCStringAsFixedString` before) and a value read by `00a26df0`. The
/// strings read are released. From 20.2.0.7 one more byte.
pub fn ni_geometry_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let stream = stream.addr();
    e.call(AV_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.call(STREAM_READ_LINK_ID_00A645B0, &args![stream]);
    e.call(STREAM_READ_LINK_ID_00A645B0, &args![stream]);
    // Locals: +0 the string handle, +4 the element size, +8 and +0xc words read and
    // dropped, +0x10 the stream argument's own slot.
    e.with_stack(0x14, |e, locals| {
        let l = locals.addr();
        let argument = l + 0x10;
        e.mem.set_u32(argument, stream);
        let version = e.mem.u32(stream + STREAM_VERSION);
        if version >= 0x1402_0005 {
            e.mem.set_u32(argument, 0);
            stream_read(e, stream, argument, 4, l, 4);
            let mut index = 0u32;
            while index < e.mem.u32(argument) {
                let empty = e.mem.u32(EMPTY_FIXED_STRING);
                e.mem.set_u32(l, empty);
                e.call(STREAM_LOAD_FIXED_STRING, &args![stream, l]);
                stream_read(e, stream, l + 8, 4, l + 4, 4);
                let handle = e.mem.u32(l);
                release_local_string(e, handle);
                index += 1;
            }
            stream_read(e, stream, l + 0xc, 4, l + 4, 4);
        } else if version >= 0x0500_0015 {
            e.mem.set_u8(argument, 0);
            stream_read(e, stream, argument, 1, l + 4, 1);
            if e.mem.u8(argument) != 0 {
                let empty = e.mem.u32(EMPTY_FIXED_STRING);
                e.mem.set_u32(l, empty);
                if e.mem.u32(stream + STREAM_VERSION) < 0x1401_0001 {
                    e.call(STREAM_LOAD_C_STRING_AS_FIXED_STRING, &args![stream, l]);
                } else {
                    e.call(STREAM_LOAD_FIXED_STRING, &args![stream, l]);
                }
                e.call(STREAM_READ_00A26DF0, &args![stream, l + 0xc]);
                let handle = e.mem.u32(l);
                release_local_string(e, handle);
            }
        }
        if e.mem.u32(stream + STREAM_VERSION) >= 0x1402_0007 {
            e.mem.set_u8(argument, 0);
            stream_read(e, stream, argument, 1, l + 4, 1);
        }
    });
}

/// `NiAVObject::RegisterStreamables` (`00a59f00`, the stream), `NiAVObject::IsEqual`
/// (`00a5a260`, the other object), `NiAVObject::PurgeRendererData` (`00a59e50`, the
/// renderer), `NiAVObject::SaveBinary` (`00a5a900`, the stream) and
/// `NiAVObject::GetViewerStrings` (`00a5ad00`, the string array), all `thiscall`.
const AV_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_9f00;
const AV_OBJECT_IS_EQUAL: u32 = 0x00a5_a260;
const AV_OBJECT_PURGE_RENDERER_DATA: u32 = 0x00a5_9e50;
const AV_OBJECT_SAVE_BINARY: u32 = 0x00a5_a900;
const AV_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_ad00;
/// `NiGeometryData::SetConsistency` (`00a67050`, `thiscall`, the flags).
const GEOMETRY_DATA_SET_CONSISTENCY: u32 = 0x00a6_7050;
/// The light listener (`011f4748`) whose slot +0xa0 `CalculateConsistency` asks.
const LIGHT_LISTENER_011F4748: u32 = 0x011f_4748;
/// `NiMatrix3::operator*` (`0043f8d0`; `thiscall` on the left matrix: the out matrix and
/// the right one), `NiMatrix3::TransposeTimes` (`00a588a0`; `thiscall`: the out matrix and
/// the other one), `NiMatrix3::Inverse_ov2` (`004b46a0`; `thiscall`: the out matrix) and
/// `NiBound::ComputeFromData` (`00a7ee30`; `thiscall` on the bound: the count and the
/// vertices).
const MATRIX_PRODUCT: u32 = 0x0043_f8d0;
const MATRIX_TRANSPOSE_TIMES: u32 = 0x00a5_88a0;
const MATRIX_INVERSE: u32 = 0x004b_46a0;
const BOUND_COMPUTE_FROM_DATA: u32 = 0x00a7_ee30;
/// The sized delete (`00aa1460`, `cdecl`: block, size).
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiGetViewerString` (`00a7d330`, the class name), `NiGetViewerString_ov8` (`00a7d510`,
/// a name and an unsigned word), `NiGetViewerString_ov10` (`00a7d5a0`, a name and an
/// object) and `NiGetViewerString_ov11` (`00a7d5e0`, a name and a fixed-string handle), all
/// `cdecl`, answering a new string.
const VIEWER_STRING_CLASS: u32 = 0x00a7_d330;
const VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const VIEWER_STRING_OBJECT: u32 = 0x00a7_d5a0;
const VIEWER_STRING_FIXED_STRING: u32 = 0x00a7_d5e0;
/// The string array's grow (`0096ad30`, `thiscall`, the new size) and set-at (`0096ae90`,
/// `thiscall`, (index, address of the string)), both unnamed in the engine map.
const STRINGS_GROW: u32 = 0x0096_ad30;
const STRINGS_SET_AT: u32 = 0x0096_ae90;
/// The literals `"GeomData"`, `"shader"` and `"skin partitions"`.
const TEXT_GEOM_DATA: u32 = 0x0109_e154;
const TEXT_SHADER: u32 = 0x0109_e14c;
const TEXT_SKIN_PARTITIONS: u32 = 0x0109_e13c;
/// `NiGeometry` (PC offsets): `m_spModelData` (+0xb8), `m_spSkinInstance` (+0xbc),
/// `m_spShader` (+0xc0); `NiAVObject`'s world transform: rotation (nine floats at +0x34),
/// translation (+0x58) and scale (+0x64); `NiObjectNET::m_spControllers` (+0xc, controllers
/// chained through their +0x30).
const MODEL_DATA: u32 = 0xb8;
const SKIN_INSTANCE: u32 = 0xbc;
const SHADER: u32 = 0xc0;
const WORLD_ROTATE: u32 = 0x34;
const WORLD_TRANSLATE: u32 = 0x58;
const WORLD_SCALE: u32 = 0x64;
const CONTROLLERS: u32 = 0xc;
const NEXT_CONTROLLER: u32 = 0x30;

/// Appends `text` to a viewer-string array as the exe's inlined `Add` does: when the 16-bit
/// size (+0xa) has reached the capacity (+0x8) the array grows to its step (+0xe) plus the
/// size, then the string is stored at the old size (through a stack word holding it).
fn push_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
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

// Translated from 00a7ff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::CalculateNormals` (Xbox PDB): a tail jump into the model data's (+0xb8)
/// virtual slot +0xa0.
pub fn ni_geometry_calculate_normals(e: &mut Engine, this: Ptr) {
    let data = e.mem.u32(this.addr() + MODEL_DATA);
    e.vcall(data, 0xa0, &args![]);
}

/// The x87 loads and stores `ApplyTransform` makes: `FLD dword` (`load`), a `float` memory
/// operand (`operand`) and `FSTP dword` (`store`).
fn load(e: &Engine, at: u32) -> f64 {
    x87_load32(e.mem.u32(at))
}
fn operand(e: &Engine, at: u32) -> f64 {
    x87_m32(e.mem.u32(at))
}
fn store(e: &mut Engine, at: u32, value: f64) {
    e.mem.set_u32(at, x87_store32(value));
}

/// The three rows of `matrix` (nine floats) times the vector at `vector`, each as the exe
/// computes it: `(row[1] * y + row[0] * x) + row[2] * z` with the matrix words loaded and
/// the vector words as memory operands.
fn rows_times(e: &Engine, matrix: u32, vector: u32) -> [f64; 3] {
    let mut out = [0.0; 3];
    for (row, slot) in out.iter_mut().enumerate() {
        let m = matrix + 12 * row as u32;
        let a = x87_mul(load(e, m + 4), operand(e, vector + 4));
        let b = x87_mul(load(e, m), operand(e, vector));
        let sum = x87_add(a, b);
        *slot = x87_add(sum, x87_mul(load(e, m + 8), operand(e, vector + 8)));
    }
    out
}

/// The normal loop both paths share (after `NiMatrix3::Inverse_ov2` into the matrix at
/// `inverse`): each normal becomes the inverse's columns times it, with the inverse's
/// words 4 to 8 held on the x87 stack. The two paths load different operands for the
/// first products: `loads_normal` (the model-space path) loads the normal's word and
/// multiplies by the matrix word in memory; otherwise the matrix word is loaded and the
/// normal's word is the memory operand.
fn transform_normals(
    e: &mut Engine,
    normals: u32,
    count: u32,
    inverse: u32,
    locals: u32,
    loads_normal: bool,
) {
    let b = |e: &Engine, i: u32| load(e, inverse + 4 * i);
    let (b4, b5, b6, b7, b8) = (b(e, 4), b(e, 5), b(e, 6), b(e, 7), b(e, 8));
    // `word * matrix[i]` as the path's instruction has it.
    let by_memory = |e: &Engine, word: u32, i: u32| {
        if loads_normal {
            x87_mul(load(e, word), operand(e, inverse + 4 * i))
        } else {
            x87_mul(load(e, inverse + 4 * i), operand(e, word))
        }
    };
    // `word * held` (a matrix word on the x87 stack) as the path's instruction has it.
    let by_held = |e: &Engine, word: u32, held: f64| {
        if loads_normal {
            x87_mul(load(e, word), held)
        } else {
            x87_mul(held, operand(e, word))
        }
    };
    for index in 0..count {
        let n = normals + 12 * index;
        let (x, y, z) = (n, n + 4, n + 8);
        let p = by_memory(e, y, 3);
        let q = by_memory(e, x, 0);
        let s = x87_add(p, q);
        let first = x87_add(s, x87_mul(b6, operand(e, z)));
        store(e, locals + 0x34, first);
        let p = by_memory(e, x, 1);
        let s = x87_add(p, by_held(e, y, b4));
        let second = x87_add(s, x87_mul(b7, operand(e, z)));
        store(e, locals + 0x38, second);
        let p = by_memory(e, x, 2);
        let s = x87_add(p, by_held(e, y, b5));
        let last = x87_mul(b8, operand(e, z));
        let word = e.mem.u32(locals + 0x34);
        e.mem.set_u32(x, word);
        let word = e.mem.u32(locals + 0x38);
        e.mem.set_u32(y, word);
        store(e, locals + 0x3c, x87_add(s, last));
        let word = e.mem.u32(locals + 0x3c);
        e.mem.set_u32(z, word);
    }
}

// Translated from 00a7ff70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::ApplyTransform` (Xbox PDB; `thiscall`: a matrix, a translation, a byte).
/// The model data's slot +0x90 gives the vertex count (16 bits); its vertices (+0x20) and
/// normals (+0x24) are read after it. With the byte 0 each vertex becomes matrix * vertex
/// + translation and, when there are normals, each normal the columns of the matrix's
/// `Inverse_ov2` (into a local) times it. Otherwise the transform is applied in this
/// object's model space: `M1 = matrix * world rotation` (`NiMatrix3::operator*`),
/// `M2 = world rotationᵀ * M1` (`TransposeTimes`), the offset `(world rotationᵀ *
/// (matrix * world translation + translation - world translation)) / world scale`, each
/// vertex `M2 * vertex + offset` and each normal by `Inverse_ov2` of `M2`. Every product
/// and sum is the exe's x87 instruction in its order (rounded at each `float` store).
/// Finally the data's bound (+0x10) is computed from the vertices
/// (`NiBound::ComputeFromData`).
pub fn ni_geometry_apply_transform(
    e: &mut Engine,
    this: Ptr,
    matrix: Ptr,
    translation: Ptr,
    model_space: u32,
) {
    let t = this.addr();
    let data = e.mem.u32(t + MODEL_DATA);
    let count = u32::from(e.vcall(data, 0x90, &args![]).u32() as u16);
    let data = e.mem.u32(t + MODEL_DATA);
    let vertices = e.mem.u32(data + 0x20);
    let normals = e.mem.u32(data + 0x24);
    let m = matrix.addr();
    let v = translation.addr();
    // The frame's locals: +0x1c..+0x3c scratch floats, +0x40 M2, +0x64 M1 (and the inverse).
    e.with_stack(0x88, |e, frame| {
        let f = frame.addr();
        if model_space as u8 == 0 {
            if vertices != 0 {
                for index in 0..count {
                    let p = vertices + 12 * index;
                    let (x, y, z) = (p, p + 4, p + 8);
                    let s = x87_add(
                        x87_mul(load(e, m + 4), operand(e, y)),
                        x87_mul(load(e, m), operand(e, x)),
                    );
                    let a = x87_add(s, x87_mul(load(e, z), operand(e, m + 8)));
                    let s = x87_add(
                        x87_mul(load(e, m + 0x10), operand(e, y)),
                        x87_mul(load(e, x), operand(e, m + 0xc)),
                    );
                    let b = x87_add(s, x87_mul(load(e, z), operand(e, m + 0x14)));
                    let s = x87_add(
                        x87_mul(load(e, m + 0x18), operand(e, x)),
                        x87_mul(load(e, y), operand(e, m + 0x1c)),
                    );
                    let c = x87_add(s, x87_mul(load(e, z), operand(e, m + 0x20)));
                    store(e, f + 0x34, a);
                    store(e, f + 0x38, b);
                    store(e, f + 0x3c, c);
                    let x = x87_add(load(e, v), operand(e, f + 0x34));
                    store(e, f + 0x28, x);
                    let y = x87_add(load(e, f + 0x38), operand(e, v + 4));
                    store(e, f + 0x2c, y);
                    let z = x87_add(load(e, f + 0x3c), operand(e, v + 8));
                    for (offset, local) in [(0u32, 0x28u32), (4, 0x2c)] {
                        let word = e.mem.u32(f + local);
                        e.mem.set_u32(p + offset, word);
                    }
                    store(e, f + 0x30, z);
                    let word = e.mem.u32(f + 0x30);
                    e.mem.set_u32(p + 8, word);
                }
            }
            if normals != 0 {
                e.call(MATRIX_INVERSE, &args![m, f + 0x64]);
                transform_normals(e, normals, count, f + 0x64, f, false);
            }
        } else {
            let rotate = t + WORLD_ROTATE;
            let product = e.call(MATRIX_PRODUCT, &args![m, f + 0x64, rotate]).u32();
            e.call(MATRIX_TRANSPOSE_TIMES, &args![rotate, f + 0x40, product]);
            let moved = rows_times(e, m, t + WORLD_TRANSLATE);
            for (i, value) in moved.into_iter().enumerate() {
                store(e, f + 0x1c + 4 * i as u32, value);
            }
            for i in 0..3u32 {
                let sum = x87_add(load(e, v + 4 * i), operand(e, f + 0x1c + 4 * i));
                store(e, f + 0x28 + 4 * i, sum);
            }
            for i in 0..3u32 {
                let offset = x87_sub(
                    load(e, f + 0x28 + 4 * i),
                    operand(e, t + WORLD_TRANSLATE + 4 * i),
                );
                store(e, f + 0x1c + 4 * i, offset);
            }
            let r = |e: &Engine, i: u32| load(e, rotate + 4 * i);
            let (d0, d1, d2) = (load(e, f + 0x1c), load(e, f + 0x20), load(e, f + 0x24));
            let p1 = x87_mul(r(e, 3), d1);
            let p0 = x87_mul(r(e, 0), d0);
            let s = x87_add(p0, p1);
            let p2 = x87_mul(r(e, 6), d2);
            store(e, f + 0x28, x87_add(p2, s));
            let q1 = x87_mul(r(e, 4), d1);
            let q0 = x87_mul(r(e, 1), d0);
            let s = x87_add(q1, q0);
            let q2 = x87_mul(r(e, 7), d2);
            store(e, f + 0x2c, x87_add(s, q2));
            let w1 = x87_mul(d1, r(e, 5));
            let w0 = x87_mul(d0, r(e, 2));
            let s = x87_add(w0, w1);
            let w2 = x87_mul(d2, operand(e, rotate + 0x20));
            store(e, f + 0x30, x87_add(s, w2));
            let reciprocal = x87_div(1.0, load(e, t + WORLD_SCALE));
            store(e, f + 0x18, reciprocal);
            let k = load(e, f + 0x18);
            store(e, f + 0x1c, x87_mul(k, operand(e, f + 0x28)));
            store(e, f + 0x20, x87_mul(load(e, f + 0x2c), k));
            store(e, f + 0x24, x87_mul(k, operand(e, f + 0x30)));
            if vertices != 0 && count != 0 {
                let a = |e: &Engine, i: u32| load(e, f + 0x40 + 4 * i);
                let (a4, a5, a6, a7, a8) = (a(e, 4), a(e, 5), a(e, 6), a(e, 7), a(e, 8));
                for index in 0..count {
                    let p = vertices + 12 * index;
                    let (x, y, z) = (p, p + 4, p + 8);
                    let s = x87_add(
                        x87_mul(load(e, x), operand(e, f + 0x40)),
                        x87_mul(load(e, y), operand(e, f + 0x44)),
                    );
                    let first = x87_add(s, x87_mul(load(e, z), operand(e, f + 0x48)));
                    store(e, f + 0x28, first);
                    let s = x87_add(
                        x87_mul(load(e, x), operand(e, f + 0x4c)),
                        x87_mul(load(e, y), a4),
                    );
                    store(e, f + 0x2c, x87_add(s, x87_mul(load(e, z), a5)));
                    let s = x87_add(x87_mul(load(e, x), a6), x87_mul(load(e, y), a7));
                    store(e, f + 0x30, x87_add(s, x87_mul(load(e, z), a8)));
                    for i in 0..3u32 {
                        let sum = x87_add(load(e, f + 0x28 + 4 * i), operand(e, f + 0x1c + 4 * i));
                        store(e, f + 0x34 + 4 * i, sum);
                        let word = e.mem.u32(f + 0x34 + 4 * i);
                        e.mem.set_u32(p + 4 * i, word);
                    }
                }
            }
            if normals != 0 {
                e.call(MATRIX_INVERSE, &args![f + 0x40, f + 0x64]);
                transform_normals(e, normals, count, f + 0x64, f, true);
            }
        }
    });
    let data = e.mem.u32(t + MODEL_DATA);
    e.call(
        BOUND_COMPUTE_FROM_DATA,
        &args![data.wrapping_add(0x10), count, vertices],
    );
}

// Translated from 00a80430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::RegisterStreamables` (Xbox PDB): false when
/// `NiAVObject::RegisterStreamables` is false; otherwise the model data and, when present,
/// the skin instance register themselves (their slot +0x54 with the stream) and the answer
/// is true.
pub fn ni_geometry_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if !e
        .call(AV_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    let data = e.mem.u32(this.addr() + MODEL_DATA);
    e.vcall(data, 0x54, &args![stream]);
    let skin = e.mem.u32(this.addr() + SKIN_INSTANCE);
    if skin != 0 {
        e.vcall(skin, 0x54, &args![stream]);
    }
    true
}

// Translated from 00a80470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::IsEqual` (Xbox PDB): `NiAVObject::IsEqual`; then false when only one side
/// has a skin instance or the skin instances differ (this one's slot +0x5c with the
/// other's); then what the model data's slot +0x5c answers for the other's data.
pub fn ni_geometry_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    if !e.call(AV_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let mine = e.mem.u32(this.addr() + SKIN_INSTANCE);
    let theirs = e.mem.u32(other.addr().wrapping_add(SKIN_INSTANCE));
    if (theirs == 0) != (mine == 0) {
        return false;
    }
    if mine != 0 && !e.vcall(mine, 0x5c, &args![theirs]).bool() {
        return false;
    }
    let data = e.mem.u32(this.addr() + MODEL_DATA);
    let other_data = e.mem.u32(other.addr().wrapping_add(MODEL_DATA));
    e.vcall(data, 0x5c, &args![other_data]).bool()
}

// Translated from 00a80650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::OnlyRenderImmediate` (engine map; `thiscall`, one word): stores the address
/// of this object's property state (+0x9c) at the argument's +0xc, then runs each
/// controller's (+0xc, chained through +0x30) virtual slot +0xa8.
pub fn ni_geometry_only_render_immediate(e: &mut Engine, this: Ptr, target: Ptr) {
    e.mem
        .set_u32(target.addr().wrapping_add(0xc), this.addr() + 0x9c);
    let mut controller = e.mem.u32(this.addr() + CONTROLLERS);
    while controller != 0 {
        e.vcall(controller, 0xa8, &args![]);
        controller = e.mem.u32(controller + NEXT_CONTROLLER);
    }
}

// Translated from 00a80930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::CalculateConsistency` (Xbox PDB; `thiscall`, a bool word): the consistency
/// starts as 0x4000 (static) and becomes 0 when a controller's slot +0xa0 answers true (the
/// walk stops there). With a skin instance: when the flag's byte is set, a skin instance
/// without a word at +0xc makes it 0; when it is clear, a skin instance without that word
/// makes it 0, and one with it makes it 0 unless the light listener (`011f4748`) is null or
/// its slot +0xa0 answers bit 1. The model data takes it (`SetConsistency`).
pub fn ni_geometry_calculate_consistency(e: &mut Engine, this: Ptr, force: u32) {
    let t = this.addr();
    let mut consistency = 0x4000u32;
    let mut controller = e.mem.u32(t + CONTROLLERS);
    while controller != 0 {
        if e.vcall(controller, 0xa0, &args![]).bool() {
            consistency = 0;
            break;
        }
        controller = e.mem.u32(controller + NEXT_CONTROLLER);
    }
    let skin = e.mem.u32(t + SKIN_INSTANCE);
    if skin != 0 {
        let partitions = e.mem.u32(skin + 0xc);
        if force as u8 != 0 {
            if partitions == 0 {
                consistency = 0;
            }
        } else if partitions == 0 {
            consistency = 0;
        } else {
            let listener = e.mem.u32(LIGHT_LISTENER_011F4748);
            if listener != 0 && e.vcall(listener, 0xa0, &args![]).u32() & 2 == 0 {
                consistency = 0;
            }
        }
    }
    let data = e.mem.u32(t + MODEL_DATA);
    e.call(GEOMETRY_DATA_SET_CONSISTENCY, &args![data, consistency]);
}

// Translated from 00a809b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::PurgeRendererData` (Xbox PDB): `NiAVObject::PurgeRendererData`, the
/// renderer's slot +0xf0 with the model data and, with a skin instance, its slot +0x104
/// with the skin instance and +0x100 with the skin instance's word at +0xc.
pub fn ni_geometry_purge_renderer_data(e: &mut Engine, this: Ptr, renderer: Ptr) {
    let t = this.addr();
    let r = renderer.addr();
    e.call(AV_OBJECT_PURGE_RENDERER_DATA, &args![this, renderer]);
    let data = e.mem.u32(t + MODEL_DATA);
    e.vcall(r, 0xf0, &args![data]);
    let skin = e.mem.u32(t + SKIN_INSTANCE);
    if skin != 0 {
        e.vcall(r, 0x104, &args![skin]);
        let skin = e.mem.u32(t + SKIN_INSTANCE);
        let partitions = e.mem.u32(skin + 0xc);
        e.vcall(r, 0x100, &args![partitions]);
    }
}

// Translated from 00a80db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::SaveBinary` (Xbox PDB): `NiAVObject::SaveBinary`, the links of the model
/// data and of the skin instance (the stream's slot +0x30 with each), then through the
/// stream's output (`m_pkOstr`, its write function, the position advanced by each result)
/// two 4-byte zeros and one zero byte (written from the stream argument's stack word whose
/// low byte the exe clears), each from the exe's locals with its component size word.
pub fn ni_geometry_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let t = this.addr();
    let s = stream.addr();
    e.call(AV_OBJECT_SAVE_BINARY, &args![this, stream]);
    let data = e.mem.u32(t + MODEL_DATA);
    e.vcall(s, 0x30, &args![data]);
    let skin = e.mem.u32(t + SKIN_INSTANCE);
    e.vcall(s, 0x30, &args![skin]);
    e.with_stack(0x10, |e, locals| {
        // +0 the size word, +4 and +8 the two words written, +0xc the argument's word.
        let l = locals.addr();
        e.mem.set_u32(l + 0xc, s);
        for (data, size_word, value, bytes) in [(l + 4, l, 0u32, 4u32), (l + 8, l, 0, 4)] {
            e.mem.set_u32(data, value);
            write(e, s, data, bytes, size_word);
        }
        e.mem.set_u8(l + 0xc, 0);
        write(e, s, l + 0xc, 1, l + 8);
    });
}

/// One write through the stream's output (`m_pkOstr` +0x250, its `m_pfnWrite` +0xc,
/// `cdecl`: (output, data, bytes, address of the size word, 1)) with the size word set to
/// `bytes`; the position (+4) advanced by the result.
fn write(e: &mut Engine, stream: u32, data: u32, bytes: u32, size_word: u32) {
    let output = e.mem.u32(stream + 0x250);
    let function = e.mem.u32(output + 0xc);
    e.mem.set_u32(size_word, bytes);
    let done = e
        .call(function, &args![output, data, bytes, size_word, 1u32])
        .u32();
    let position = e.mem.u32(output + 4).wrapping_add(done);
    e.mem.set_u32(output + 4, position);
}

// Translated from 00a80e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::GetViewerStrings` (Xbox PDB): `NiAVObject::GetViewerStrings`, the class
/// name, `"GeomData"` with the model data; with a shader, `"shader"` with the handle its
/// slot +8 points at; with a skin instance whose word at +0xc is not null, `"skin
/// partitions"` with that object's word at +8. Each appended to the array.
pub fn ni_geometry_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let t = this.addr();
    e.call(AV_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let name = e.mem.u32(NI_GEOMETRY_RTTI);
    let text = e.call(VIEWER_STRING_CLASS, &args![name]).u32();
    push_viewer_string(e, strings, text);
    let data = e.mem.u32(t + MODEL_DATA);
    let text = e
        .call(VIEWER_STRING_OBJECT, &args![TEXT_GEOM_DATA, data])
        .u32();
    push_viewer_string(e, strings, text);
    let shader = e.mem.u32(t + SHADER);
    if shader != 0 {
        let handle = e.vcall(shader, 8, &args![]).u32();
        let handle = e.mem.u32(handle);
        let text = e
            .call(VIEWER_STRING_FIXED_STRING, &args![TEXT_SHADER, handle])
            .u32();
        push_viewer_string(e, strings, text);
    }
    let skin = e.mem.u32(t + SKIN_INSTANCE);
    if skin != 0 {
        let partitions = e.mem.u32(skin + 0xc);
        if partitions != 0 {
            let count = e.mem.u32(partitions + 8);
            let text = e
                .call(VIEWER_STRING_UNSIGNED, &args![TEXT_SKIN_PARTITIONS, count])
                .u32();
            push_viewer_string(e, strings, text);
        }
    }
}

// Translated from 00a9b1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiGeometry::_scalar_deleting_destructor_` (Xbox PDB): the destructor (`00a80580`),
/// then the sized delete of 0xc4 bytes when bit 0 of `flags` is set. Answers `this`.
pub fn ni_geometry_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    ni_geometry_destructor(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xc4u32]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_fd90, ni_geometry_on_visible(Ptr, Ptr)),
        entry!(0x00a8_0540, fn_00a80540(Ptr) -> Ptr),
        entry!(0x00a8_0610, ni_geometry_render_immediate(Ptr, Ptr)),
        entry!(
            0x00a8_08f0,
            ni_geometry_set_selective_update_flags(Ptr, Ptr, u32, Ptr)
        ),
        entry!(0x00a5_abe0, fn_00a5abe0(Ptr, Ptr, Ptr, Ptr)),
        entry!(0x00a5_ac90, fn_00a5ac90(Ptr, Ptr, Ptr)),
        entry!(0x00a8_0580, ni_geometry_destructor(Ptr)),
        entry!(0x00a8_0680, ni_geometry_set_model_data(Ptr, Ptr)),
        entry!(0x00a8_0710, ni_geometry_update_world_bound(Ptr)),
        entry!(
            0x00a8_0820,
            ni_geometry_update_properties_downward(Ptr, Ptr)
        ),
        entry!(0x00a8_0a10, ni_geometry_process_clone(Ptr, Ptr)),
        entry!(0x00a8_0b50, ni_geometry_link_object(Ptr, Ptr)),
        entry!(0x00a8_0bf0, ni_geometry_load_binary(Ptr, Ptr)),
        entry!(0x00a8_04e0, ni_geometry_ni_geometry(Ptr, Ptr) -> Ptr),
        entry!(0x00a80530, ni_geometry_get_rtti() -> Ptr),
        entry!(0x00a7_fdb0, ni_geometry_set_bound(Ptr, Ptr)),
        entry!(0x00a7_fdc0, ni_geometry_post_link_object(Ptr, Ptr)),
        entry!(0x00a7_ff60, ni_geometry_calculate_normals(Ptr)),
        entry!(0x00a7_ff70, ni_geometry_apply_transform(Ptr, Ptr, Ptr, u32)),
        entry!(
            0x00a8_0430,
            ni_geometry_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a8_0470, ni_geometry_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a8_0650, ni_geometry_only_render_immediate(Ptr, Ptr)),
        entry!(0x00a8_0930, ni_geometry_calculate_consistency(Ptr, u32)),
        entry!(0x00a8_09b0, ni_geometry_purge_renderer_data(Ptr, Ptr)),
        entry!(0x00a8_0db0, ni_geometry_save_binary(Ptr, Ptr)),
        entry!(0x00a8_0e70, ni_geometry_get_viewer_strings(Ptr, Ptr)),
        entry!(
            0x00a9_b1c0,
            ni_geometry_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_geometry_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a80530, &args![]).u32(), NI_GEOMETRY_RTTI);
    }

    #[test]
    fn thunk_00a7fdb0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_fdb0;
        const TARGET: u32 = 0x00a5_9dc0;
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
    fn thunk_00a7fdc0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a7_fdc0;
        const TARGET: u32 = 0x00a5_c6a0;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(THUNK, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
    }
}

#[cfg(test)]
mod constructor_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NI_AV_OBJECT_CONSTRUCT, |_, _| Ret::default());
        e.register(NI_PROPERTY_STATE_CONSTRUCT, |_, _| Ret::default());
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn ni_geometry_ni_geometry_keeps_the_data_and_counts_a_reference() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        e.mem.set_u32(this + 0xbc, 9);
        e.mem.set_u32(this + 0xc0, 9);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data + 4, 2);
        assert_eq!(e.call(0x00a8_04e0, &args![this, data]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_GEOMETRY_VTABLE);
        assert_eq!(e.mem.u32(this + 0xb8), data);
        assert_eq!(e.mem.u32(data + 4), 3);
        assert_eq!(e.mem.u32(this + 0xbc), 0);
        assert_eq!(e.mem.u32(this + 0xc0), 0);
        assert_eq!(calls(&e, NI_AV_OBJECT_CONSTRUCT), vec![vec![this]]);
        assert_eq!(
            calls(&e, NI_PROPERTY_STATE_CONSTRUCT),
            vec![vec![this + 0x9c]]
        );
    }

    #[test]
    fn ni_geometry_ni_geometry_without_data_counts_nothing() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        e.call(0x00a8_04e0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 0xb8), 0);
        assert!(calls(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
    }
}

#[cfg(test)]
mod batch_b011_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};
    use std::cell::RefCell;
    use std::rc::Rc;

    /// One vtable for every object of these tests: each slot is a double at
    /// `SLOT_DOUBLES + slot` answering the word stored at `ANSWERS + slot` (0 by default).
    const VTABLE: u32 = 0x0320_0000;
    const SLOT_DOUBLES: u32 = 0x0321_0000;
    const ANSWERS: u32 = 0x0322_0000;
    const READ_DOUBLE: u32 = 0x0323_0000;

    type Log = Rc<RefCell<Vec<(u32, u32)>>>;

    fn slot(offset: u32) -> u32 {
        SLOT_DOUBLES + offset
    }

    fn engine() -> Engine {
        let mut e = engine_with_counting();
        e.map(ANSWERS, 0x1000);
        let slots: Vec<u32> = (0..0x40u32).map(|i| slot(4 * i)).collect();
        e.put_vtable(VTABLE, &slots);
        for i in 0..0x40u32 {
            e.register(slot(4 * i), |e, _| {
                // The double's own address is not passed; every slot answers the same word,
                // so tests that need different answers register their own double.
                e.mem.u32(ANSWERS).into_ret()
            });
        }
        for double in [
            NI_AV_OBJECT_CONSTRUCT,
            NI_PROPERTY_STATE_CONSTRUCT,
            SPIN_LOCK_ENTER_0040FBF0,
            AV_OBJECT_UPDATE_PROPERTIES_DOWNWARD,
            AV_OBJECT_SET_SELECTIVE_UPDATE_FLAGS,
            AV_OBJECT_LOAD_BINARY,
            AV_OBJECT_LINK_OBJECT,
            AV_OBJECT_PROCESS_CLONE,
            PROPERTY_STATE_DESTRUCT,
            BOUND_UPDATE,
            STREAM_LOAD_FIXED_STRING,
            STREAM_LOAD_C_STRING_AS_FIXED_STRING,
            AV_OBJECT_SET_COLLISION_OBJECT,
            PROPERTY_LIST_ADD_HEAD,
            AV_OBJECT_DESTRUCT_00A5A390,
            PROPERTY_STATE_COPY_00A89900,
            OBJECT_NET_COPY_MEMBERS_00A5C290,
            SKIN_INSTANCE_BOUND_00A86530,
            BOUND_STEP_00A7FEA0,
            STREAM_READ_LINK_ID_00A645B0,
            STREAM_READ_00A26DF0,
            POINTER_ASSIGN_0066B0D0,
            SET_NAME_00A5B950,
            SETTER_00BA3760,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(CONSTRUCT_00A8F440, |_, a| a[0].into_ret());
        e.register(GETTER_00A62660, |_, _| 0x6666u32.into_ret());
        e.register(SHADER_PROGRAM_GET_NAME, |_, _| 0x7777u32.into_ret());
        e.register(BOUND_COMPARE_00A92660, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, size: u32, count: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, VTABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    #[test]
    fn on_visible_hands_this_to_the_culler() {
        let mut e = engine();
        let culler = object(&mut e, 8, 0);
        e.call(0x00a7_fd90, &args![0x1234u32, culler]);
        assert_eq!(calls_to(&e, slot(0x4c)), vec![vec![culler, 0x1234]]);
    }

    #[test]
    fn default_constructor_clears_the_pointers() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        e.mem.write(this + 0xb8, &[0xcc; 0xc]);
        assert_eq!(e.call(0x00a8_0540, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_GEOMETRY_VTABLE);
        assert_eq!(
            calls_to(&e, NI_PROPERTY_STATE_CONSTRUCT),
            vec![vec![this + 0x9c]]
        );
        for offset in [0xb8u32, 0xbc, 0xc0] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }

    #[test]
    fn render_immediate_sets_the_state_and_walks_the_controllers() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        let renderer = e.mem.alloc(0x10);
        let first = object(&mut e, 0x40, 0);
        let second = object(&mut e, 0x40, 0);
        e.mem.set_u32(this + 0xc, first);
        e.mem.set_u32(first + 0x30, second);
        e.call(0x00a8_0610, &args![this, renderer]);
        assert_eq!(e.mem.u32(renderer + 0xc), this + 0x9c);
        assert_eq!(calls_to(&e, slot(0xa8)), vec![vec![first], vec![second]]);
    }

    #[test]
    fn selective_update_flags_mark_skinned_geometry() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        let selective = e.mem.alloc(4);
        let rigid = e.mem.alloc(4);
        e.mem.set_u8(rigid, 1);
        e.call(0x00a8_08f0, &args![this, selective, 1u32, rigid]);
        assert_eq!(
            calls_to(&e, AV_OBJECT_SET_SELECTIVE_UPDATE_FLAGS),
            vec![vec![this, selective, 1, rigid]]
        );
        assert_eq!((e.mem.u8(selective), e.mem.u8(rigid)), (0, 1));
        e.mem.set_u32(this + 0xbc, 9);
        e.mem.set_u32(this + 0x30, 0x100);
        e.call(0x00a8_08f0, &args![this, selective, 1u32, rigid]);
        assert_eq!((e.mem.u8(selective), e.mem.u8(rigid)), (1, 0));
        assert_eq!(e.mem.u32(this + 0x30), 0x106);
    }

    #[test]
    fn copy_properties_clones_from_the_tail() {
        let mut e = engine();
        let property = object(&mut e, 0x20, 1);
        let ignored = object(&mut e, 0x20, 1);
        let clone = object(&mut e, 0x20, 0);
        // Node layout: +0 next, +4 previous, +8 property; the list's tail at +4.
        let tail = e.mem.alloc(0xc);
        let before = e.mem.alloc(0xc);
        e.mem.set_u32(tail + 4, before);
        e.mem.set_u32(tail + 8, property);
        e.mem.set_u32(before + 8, ignored);
        let list = e.mem.alloc(0xc);
        e.mem.set_u32(list + 4, tail);
        // Types: the property 3, the ignored one 7, the clone 3.
        e.register_double(slot(0x8c), move |_, a| {
            (if a[0] == ignored { 7u32 } else { 3 }).into_ret()
        });
        e.register_double(OBJECT_CLONE_00A5D3F0, move |_, _| clone.into_ret());
        let held: Rc<RefCell<Vec<u32>>> = Rc::default();
        let seen = held.clone();
        e.register_double(PROPERTY_LIST_ADD_HEAD, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[1]));
            seen.borrow_mut().push(e.mem.u32(clone + 4));
            Ret::default()
        });
        let copy = e.mem.alloc(0xd0);
        e.mem.set_u32(PROPERTY_LOCK_COUNT, 1);
        e.mem.set_u32(PROPERTY_LOCK, 0x55);
        e.call(0x00a5_abe0, &args![0x9999u32, copy, list, 0x4444u32]);
        assert_eq!(
            calls_to(&e, OBJECT_CLONE_00A5D3F0),
            vec![vec![property, 0x4444]]
        );
        // Held (counted once) while added, released after: count back to 0, deleted.
        assert_eq!(*held.borrow(), vec![clone, 1]);
        assert_eq!(e.mem.u32(clone + 4), 0);
        assert_eq!(calls_to(&e, slot(4)), vec![vec![clone]]);
        assert_eq!(calls_to(&e, PROPERTY_LIST_ADD_HEAD)[0][0], copy + 0x24);
        assert_eq!(
            calls_to(&e, SPIN_LOCK_ENTER_0040FBF0),
            vec![vec![PROPERTY_LOCK, 0]]
        );
        assert_eq!(e.mem.u32(PROPERTY_LOCK_COUNT), 0);
        assert_eq!(e.mem.u32(PROPERTY_LOCK), 0);
    }

    #[test]
    fn copy_members_copies_flags_transform_and_collision() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        let copy = e.mem.alloc(0xd0);
        e.mem.set_u32(this + 0x30, 0x11);
        for index in 0..0xdu32 {
            e.mem.set_u32(this + 0x34 + 4 * index, index + 1);
        }
        let collision = object(&mut e, 0x20, 0);
        e.mem.set_u32(this + 0x1c, collision);
        e.mem.set_u32(ANSWERS, 0xc0ffee);
        e.mem.set_u32(PROPERTY_LOCK_COUNT, 1);
        e.call(0x00a5_ac90, &args![this, copy, 0x4444u32]);
        assert_eq!(e.mem.u32(copy + 0x30), 0x80011);
        for index in 0..0xdu32 {
            assert_eq!(e.mem.u32(copy + 0x34 + 4 * index), index + 1);
        }
        assert_eq!(e.mem.u32(copy + 0x68), 0);
        // An empty property list (count +0x2c zero): no property copy.
        assert!(calls_to(&e, SPIN_LOCK_ENTER_0040FBF0).is_empty());
        assert_eq!(calls_to(&e, slot(0x48)), vec![vec![collision, 0x4444]]);
        assert_eq!(
            calls_to(&e, AV_OBJECT_SET_COLLISION_OBJECT),
            vec![vec![copy, 0xc0ffee]]
        );
        // With a property count the list walk runs (an empty list here).
        e.mem.set_u32(this + 0x2c, 1);
        e.call(0x00a5_ac90, &args![this, copy, 0x4444u32]);
        assert_eq!(calls_to(&e, SPIN_LOCK_ENTER_0040FBF0).len(), 1);
    }

    #[test]
    fn destructor_releases_data_and_skin() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        let data = object(&mut e, 0x40, 1);
        let skin = object(&mut e, 0x40, 2);
        e.mem.set_u32(this + 0xb8, data);
        e.mem.set_u32(this + 0xbc, skin);
        e.call(0x00a8_0580, &args![this]);
        assert_eq!(e.mem.u32(this), NI_GEOMETRY_VTABLE);
        assert_eq!(e.mem.u32(this + 0xb8), 0);
        assert_eq!(e.mem.u32(data + 4), 0);
        assert_eq!(e.mem.u32(skin + 4), 1);
        assert_eq!(calls_to(&e, slot(4)), vec![vec![data]]);
        assert_eq!(
            calls_to(&e, PROPERTY_STATE_DESTRUCT),
            vec![vec![this + 0x9c]]
        );
        assert_eq!(calls_to(&e, AV_OBJECT_DESTRUCT_00A5A390), vec![vec![this]]);
    }

    #[test]
    fn set_model_data_tells_the_collision_object() {
        let mut e = engine();
        let this = e.mem.alloc(0xd0);
        let data = object(&mut e, 0x40, 0);
        // Null data: nothing.
        e.call(0x00a8_0680, &args![this, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        let collision = object(&mut e, 0x20, 0);
        e.mem.set_u32(this + 0x1c, collision);
        let seen: Rc<RefCell<Vec<u8>>> = Rc::default();
        let local = seen.clone();
        e.register_double(slot(0x90), move |e, a| {
            local.borrow_mut().extend(e.mem.bytes(a[1], 9));
            Ret::default()
        });
        e.call(0x00a8_0680, &args![this, data]);
        assert_eq!(calls_to(&e, slot(0x94)), vec![vec![collision]]);
        assert_eq!(*seen.borrow(), vec![0; 9]);
        assert_eq!(e.mem.u32(this + 0xb8), data);
        assert_eq!(e.mem.u32(data + 4), 1);
    }

    #[test]
    fn update_world_bound_runs_only_for_a_zero_radius_or_forced() {
        let mut e = engine();
        let this = object(&mut e, 0xd0, 0);
        let bound = e.mem.alloc(0x10);
        e.mem.set_f32(bound + 0xc, 2.0);
        e.mem.set_u32(this + 0x20, bound);
        e.call(0x00a8_0710, &args![this]);
        assert!(calls_to(&e, BOUND_UPDATE).is_empty());
        // A NaN radius stops too.
        e.mem.set_u32(bound + 0xc, 0x7fc0_0000);
        e.call(0x00a8_0710, &args![this]);
        assert!(calls_to(&e, BOUND_UPDATE).is_empty());
        // Bit 19 set, bit 13 clear: forced.
        e.mem.set_u32(this + 0x30, 1 << 19);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0xb8, data);
        e.call(0x00a8_0710, &args![this]);
        assert_eq!(
            calls_to(&e, BOUND_UPDATE),
            vec![vec![bound, data + 0x10, this + 0x68]]
        );
        // Bit 13 too: back to the radius test.
        e.mem.set_u32(this + 0x30, (1 << 19) | (1 << 13));
        e.call(0x00a8_0710, &args![this]);
        assert_eq!(calls_to(&e, BOUND_UPDATE).len(), 1);
    }

    #[test]
    fn update_world_bound_makes_a_bound_and_tracks_changes() {
        let mut e = engine();
        e.mem.set_u32(BOUND_CENTER_011F426C, 1);
        e.mem.set_u32(BOUND_CENTER_011F426C + 4, 2);
        e.mem.set_u32(BOUND_CENTER_011F426C + 8, 3);
        let this = object(&mut e, 0xd0, 0);
        e.register(slot(0x78), |e, a| {
            e.mem.set_u32(a[0] + 0x20, a[1]);
            Ret::default()
        });
        e.mem.set_u32(this + 0xbc, 0x5150);
        e.mem.set_u32(this + 0x30, 1 << 22);
        e.register(BOUND_COMPARE_00A92660, |_, _| 0x100u32.into_ret());
        e.call(0x00a8_0710, &args![this]);
        let bound = e.mem.u32(this + 0x20);
        assert_ne!(bound, 0);
        assert_eq!(
            e.mem.bytes(bound, 12),
            vec![1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]
        );
        assert_eq!(e.mem.u32(bound + 0xc), 0);
        assert_eq!(
            calls_to(&e, SKIN_INSTANCE_BOUND_00A86530),
            vec![vec![0x5150, bound]]
        );
        assert_eq!(
            calls_to(&e, BOUND_STEP_00A7FEA0),
            vec![vec![bound, this + 0x68]]
        );
        // The compare answered false: bit 22 cleared.
        assert_eq!(e.mem.u32(this + 0x30), 0);
        e.mem.set_u32(this + 0x30, 1 << 22);
        e.register(BOUND_COMPARE_00A92660, |_, _| 1u32.into_ret());
        e.call(0x00a8_0710, &args![this]);
        assert_eq!(e.mem.u32(this + 0x30), 1 << 22);
    }

    #[test]
    fn update_properties_downward_fills_the_state() {
        let mut e = engine();
        let this = object(&mut e, 0xd0, 0);
        let alpha = object(&mut e, 0x20, 0);
        let other = object(&mut e, 0x20, 0);
        let old = object(&mut e, 0x20, 1);
        e.register_double(slot(0x8c), move |_, a| {
            (if a[0] == other { 8u32 } else { 2 }).into_ret()
        });
        let first = e.mem.alloc(0xc);
        let second = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, alpha);
        e.mem.set_u32(second + 8, other);
        e.mem.set_u32(this + 0x24, first);
        e.mem.set_u32(this + 0x9c + 8, old);
        e.mem.set_u32(PROPERTY_LOCK_COUNT, 1);
        e.call(0x00a8_0820, &args![this, 0x3000u32]);
        assert_eq!(e.mem.u32(this + 0x9c + 8), alpha);
        assert_eq!(e.mem.u32(alpha + 4), 1);
        assert_eq!(calls_to(&e, slot(4)), vec![vec![old]]);
        assert_eq!(
            calls_to(&e, PROPERTY_STATE_COPY_00A89900),
            vec![vec![this + 0x9c, 0x3000]]
        );
        assert_eq!(calls_to(&e, slot(0x94)), vec![vec![this, 1]]);
        assert_eq!(e.mem.u32(PROPERTY_LOCK_COUNT), 0);
    }

    #[test]
    fn process_clone_shares_or_rebuilds_the_shader() {
        let mut e = engine();
        let this = object(&mut e, 0xd0, 0);
        let clone = object(&mut e, 0xd0, 0);
        let cloning = e.mem.alloc(8);
        e.mem.set_u32(cloning, 0x8888);
        e.register_double(CLONE_MAP_LOOKUP_00A53C30, move |e, a| {
            if a[1] == this {
                e.mem.set_u32(a[2], clone);
                1u32.into_ret()
            } else {
                0u32.into_ret()
            }
        });
        let skin = object(&mut e, 0x40, 0);
        e.mem.set_u32(this + 0xbc, skin);
        let shader = object(&mut e, 0x20, 0);
        e.mem.set_u32(this + 0xc0, shader);
        e.mem.set_u32(ANSWERS, 0x1357);
        e.mem.set_u32(PROPERTY_LOCK_COUNT, 1);
        e.call(0x00a8_0a10, &args![this, cloning]);
        // The skin was not in the map: its clone (slot +0x48) is given and processed.
        assert_eq!(
            calls_to(&e, POINTER_ASSIGN_0066B0D0),
            vec![vec![clone + 0xbc, 0x1357]]
        );
        assert_eq!(calls_to(&e, slot(0x68)), vec![vec![skin, cloning]]);
        // A shared shader.
        assert_eq!(e.mem.u32(clone + 0xc0), shader);
        assert_eq!(calls_to(&e, slot(0x50)), vec![vec![shader, clone]]);
        assert_eq!(calls_to(&e, slot(0xb0)), vec![vec![clone, this + 0x9c]]);
        assert_eq!(e.mem.u32(PROPERTY_LOCK_COUNT), 0);

        // The shader of RTTI 011f5d08 is rebuilt.
        e.mem.set_u32(ANSWERS, SHADER_RTTI_011F5D08);
        e.mem.set_u32(this + 0xbc, 0);
        e.call(0x00a8_0a10, &args![this, cloning]);
        let made = e.mem.u32(clone + 0xc0);
        assert_ne!(made, shader);
        assert_eq!(calls_to(&e, SET_NAME_00A5B950), vec![vec![made, 0x6666]]);
        assert_eq!(calls_to(&e, SETTER_00BA3760), vec![vec![made, 0x7777]]);
    }

    #[test]
    fn link_object_takes_data_and_skin() {
        let mut e = engine();
        let this = object(&mut e, 0xd0, 0);
        let data = object(&mut e, 0x40, 0);
        let mut next = vec![data, 0];
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            next.remove(0).into_ret()
        });
        e.call(0x00a8_0b50, &args![this, 0x500u32]);
        assert_eq!(e.mem.u32(this + 0xb8), data);
        assert_eq!(e.mem.u32(this + 0xbc), 0);
        assert_eq!(e.mem.u32(data + 4), 1);
    }

    /// A stream whose reads answer the given chunks in order and log (bytes, size).
    fn make_stream(e: &mut Engine, version: u32, chunks: Vec<Vec<u8>>) -> (u32, Log) {
        let stream = e.mem.alloc(0x5c4);
        e.mem.set_u32(stream + STREAM_VERSION, version);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 8, READ_DOUBLE);
        e.mem.set_u32(stream + STREAM_INPUT, binary);
        let log: Log = Rc::default();
        let reads = log.clone();
        let mut chunks = chunks;
        e.register_double(READ_DOUBLE, move |e, a| {
            reads.borrow_mut().push((a[2], e.mem.u32(a[3])));
            let chunk = chunks.remove(0);
            e.mem.write(a[1], &chunk);
            a[2].into_ret()
        });
        (stream, log)
    }

    #[test]
    fn load_binary_drops_the_shader_strings_of_new_files() {
        let mut e = engine();
        let handle = e.mem.alloc(0x20) + 8;
        e.mem.set_u32(handle - 8, 5);
        e.register_double(STREAM_LOAD_FIXED_STRING, move |e, a| {
            e.mem.set_u32(a[1], handle);
            Ret::default()
        });
        let chunks = vec![
            vec![2, 0, 0, 0],
            vec![0; 4],
            vec![0; 4],
            vec![0; 4],
            vec![1],
        ];
        let (stream, log) = make_stream(&mut e, 0x1402_0007, chunks);
        e.call(0x00a8_0bf0, &args![0x7000u32, stream]);
        assert_eq!(*log.borrow(), vec![(4, 4), (4, 4), (4, 4), (4, 4), (1, 1)]);
        assert_eq!(calls_to(&e, STREAM_LOAD_FIXED_STRING).len(), 2);
        assert_eq!(e.mem.u32(handle - 8), 3);
        assert_eq!(calls_to(&e, STREAM_READ_LINK_ID_00A645B0).len(), 2);
        assert_eq!(
            calls_to(&e, AV_OBJECT_LOAD_BINARY),
            vec![vec![0x7000, stream]]
        );
    }

    #[test]
    fn load_binary_of_older_files_reads_an_optional_string() {
        let mut e = engine();
        // 5.0.0.21 to 20.1.0.0: a set byte, then a C string and a value.
        let (stream, log) = make_stream(&mut e, 0x1401_0000, vec![vec![1]]);
        e.call(0x00a8_0bf0, &args![0x7000u32, stream]);
        assert_eq!(*log.borrow(), vec![(1, 1)]);
        assert_eq!(calls_to(&e, STREAM_LOAD_C_STRING_AS_FIXED_STRING).len(), 1);
        assert_eq!(calls_to(&e, STREAM_READ_00A26DF0)[0][0], stream);
        // 20.1.0.1: the fixed-string form.
        let (stream, _) = make_stream(&mut e, 0x1401_0001, vec![vec![1]]);
        e.call(0x00a8_0bf0, &args![0x7000u32, stream]);
        assert_eq!(calls_to(&e, STREAM_LOAD_FIXED_STRING).len(), 1);
        // A clear byte: nothing more; before 5.0.0.21 nothing at all.
        let (stream, _) = make_stream(&mut e, 0x1401_0001, vec![vec![0]]);
        e.call(0x00a8_0bf0, &args![0x7000u32, stream]);
        assert_eq!(calls_to(&e, STREAM_READ_00A26DF0).len(), 2);
        let (stream, log) = make_stream(&mut e, 0x0500_0014, vec![]);
        e.call(0x00a8_0bf0, &args![0x7000u32, stream]);
        assert!(log.borrow().is_empty());
    }
}

#[cfg(test)]
mod n016_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};

    /// Slot doubles of the test vtable `0x0e10_0000` (slot `n` at `0x0e00_0000 + n`).
    const VTABLE: u32 = 0x0e10_0000;
    fn slot(offset: u32) -> u32 {
        0x0e00_0000 + offset
    }

    /// An engine whose test vtable has recording doubles answering 1 in every slot, and a
    /// geometry with model data (vertex count `count` from slot +0x90) using that vtable.
    fn rig(count: u32) -> (Engine, u32, u32) {
        let mut e = engine_with_counting();
        let slots: Vec<u32> = (0..0x48).map(|i| slot(4 * i)).collect();
        e.put_vtable(VTABLE, &slots);
        for i in 0..0x48u32 {
            e.register(slot(4 * i), |_, _| 1u32.into_ret());
        }
        e.register_double(slot(0x90), move |_, _| (0xffff_0000 | count).into_ret());
        let this = e.mem.alloc(0xc4);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(data, VTABLE);
        e.mem.set_u32(this + MODEL_DATA, data);
        e.call_log = Some(vec![]);
        (e, this, data)
    }

    fn floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn read(e: &Engine, at: u32, n: u32) -> Vec<f32> {
        (0..n).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// The inverse double: writes 1..9 into its out matrix.
    fn inverse_writes_one_to_nine(e: &mut Engine) {
        e.register(MATRIX_INVERSE, |e, a| {
            for i in 0..9u32 {
                e.mem.set_f32(a[1] + 4 * i, (i + 1) as f32);
            }
            1u32.into_ret()
        });
    }

    #[test]
    fn calculate_normals_jumps_into_the_data() {
        let (mut e, this, data) = rig(0);
        e.call(0x00a7_ff60, &args![this]);
        assert_eq!(calls_to(&e, slot(0xa0)), vec![vec![data]]);
    }

    #[test]
    fn apply_transform_with_byte_0_transforms_the_data_directly() {
        let (mut e, this, data) = rig(1);
        e.register(BOUND_COMPUTE_FROM_DATA, |_, _| Ret::default());
        inverse_writes_one_to_nine(&mut e);
        let vertices = e.mem.alloc(12);
        let normals = e.mem.alloc(12);
        floats(&mut e, vertices, &[1.0, 1.0, 1.0]);
        floats(&mut e, normals, &[1.0, 2.0, 3.0]);
        e.mem.set_u32(data + 0x20, vertices);
        e.mem.set_u32(data + 0x24, normals);
        let matrix = e.mem.alloc(0x24);
        floats(
            &mut e,
            matrix,
            &[1.0, 2.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 2.0],
        );
        let translation = e.mem.alloc(12);
        floats(&mut e, translation, &[1.0, 2.0, 3.0]);
        e.call(0x00a7_ff70, &args![this, matrix, translation, 0x100u32]);
        assert_eq!(read(&e, vertices, 3), vec![4.0, 3.0, 5.0]);
        // The columns of the inverse (1..9) times (1, 2, 3).
        assert_eq!(read(&e, normals, 3), vec![30.0, 36.0, 42.0]);
        let inverse = calls_to(&e, MATRIX_INVERSE);
        assert_eq!(inverse[0][0], matrix);
        assert_eq!(
            calls_to(&e, BOUND_COMPUTE_FROM_DATA),
            vec![vec![data + 0x10, 1, vertices]]
        );
        assert!(calls_to(&e, MATRIX_PRODUCT).is_empty());
    }

    #[test]
    fn apply_transform_with_the_byte_set_goes_through_the_world_transform() {
        let (mut e, this, data) = rig(1);
        e.register(BOUND_COMPUTE_FROM_DATA, |_, _| Ret::default());
        inverse_writes_one_to_nine(&mut e);
        // operator* copies the left matrix out; TransposeTimes copies its argument out.
        e.register(MATRIX_PRODUCT, |e, a| {
            let words = e.mem.bytes(a[0], 0x24);
            e.mem.write(a[1], &words);
            a[1].into_ret()
        });
        e.register(MATRIX_TRANSPOSE_TIMES, |e, a| {
            let words = e.mem.bytes(a[2], 0x24);
            e.mem.write(a[1], &words);
            a[1].into_ret()
        });
        floats(
            &mut e,
            this + WORLD_ROTATE,
            &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        );
        floats(&mut e, this + WORLD_TRANSLATE, &[10.0, 0.0, 0.0, 2.0]);
        let vertices = e.mem.alloc(12);
        let normals = e.mem.alloc(12);
        floats(&mut e, vertices, &[1.0, 1.0, 1.0]);
        floats(&mut e, normals, &[1.0, 2.0, 3.0]);
        e.mem.set_u32(data + 0x20, vertices);
        e.mem.set_u32(data + 0x24, normals);
        let matrix = e.mem.alloc(0x24);
        floats(
            &mut e,
            matrix,
            &[2.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 2.0],
        );
        let translation = e.mem.alloc(12);
        floats(&mut e, translation, &[4.0, 0.0, 0.0]);
        e.call(0x00a7_ff70, &args![this, matrix, translation, 1u32]);
        // offset = (2 * 10 + 4 - 10) / 2 = 7 on x.
        assert_eq!(read(&e, vertices, 3), vec![9.0, 2.0, 2.0]);
        assert_eq!(read(&e, normals, 3), vec![30.0, 36.0, 42.0]);
        let product = calls_to(&e, MATRIX_PRODUCT);
        assert_eq!(
            (product[0][0], product[0][2]),
            (matrix, this + WORLD_ROTATE)
        );
        let transposed = calls_to(&e, MATRIX_TRANSPOSE_TIMES);
        assert_eq!(transposed[0][0], this + WORLD_ROTATE);
        assert_eq!(transposed[0][2], product[0][1]);
        assert_eq!(calls_to(&e, MATRIX_INVERSE)[0][0], transposed[0][1]);
        // No vertices and no normals: only the bound.
        e.mem.set_u32(data + 0x20, 0);
        e.mem.set_u32(data + 0x24, 0);
        e.call(0x00a7_ff70, &args![this, matrix, translation, 1u32]);
        assert_eq!(calls_to(&e, MATRIX_INVERSE).len(), 1);
        assert_eq!(
            calls_to(&e, BOUND_COMPUTE_FROM_DATA)[1],
            vec![data + 0x10, 1, 0]
        );
    }

    #[test]
    fn register_streamables_registers_data_and_skin() {
        let (mut e, this, data) = rig(0);
        e.register(AV_OBJECT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        assert!(e.call(0x00a8_0430, &args![this, 0x77u32]).bool());
        assert_eq!(calls_to(&e, slot(0x54)), vec![vec![data, 0x77]]);
        let skin = e.mem.alloc(0x10);
        e.mem.set_u32(skin, VTABLE);
        e.mem.set_u32(this + SKIN_INSTANCE, skin);
        e.call(0x00a8_0430, &args![this, 0x77u32]);
        assert_eq!(calls_to(&e, slot(0x54))[2], vec![skin, 0x77]);
        e.register(AV_OBJECT_REGISTER_STREAMABLES, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a8_0430, &args![this, 0x77u32]).bool());
        assert_eq!(calls_to(&e, slot(0x54)).len(), 3);
    }

    #[test]
    fn is_equal_checks_skins_then_data() {
        let (mut e, this, data) = rig(0);
        e.register(AV_OBJECT_IS_EQUAL, |_, _| 1u32.into_ret());
        let other = e.mem.alloc(0xc4);
        let other_data = e.mem.alloc(0x10);
        e.mem.set_u32(other + MODEL_DATA, other_data);
        assert!(e.call(0x00a8_0470, &args![this, other]).bool());
        assert_eq!(calls_to(&e, slot(0x5c)), vec![vec![data, other_data]]);
        let skin = e.mem.alloc(0x10);
        e.mem.set_u32(skin, VTABLE);
        e.mem.set_u32(this + SKIN_INSTANCE, skin);
        assert!(!e.call(0x00a8_0470, &args![this, other]).bool());
        e.mem.set_u32(other + SKIN_INSTANCE, 0x0e70_0000);
        assert!(e.call(0x00a8_0470, &args![this, other]).bool());
        assert_eq!(calls_to(&e, slot(0x5c))[1], vec![skin, 0x0e70_0000]);
        e.register(slot(0x5c), |_, a| u32::from(a[1] != 0x0e70_0000).into_ret());
        assert!(!e.call(0x00a8_0470, &args![this, other]).bool());
        e.register(AV_OBJECT_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a8_0470, &args![this, other]).bool());
    }

    #[test]
    fn only_render_immediate_points_at_the_state_and_runs_controllers() {
        let (mut e, this, _) = rig(0);
        let first = e.mem.alloc(0x34);
        let second = e.mem.alloc(0x34);
        for controller in [first, second] {
            e.mem.set_u32(controller, VTABLE);
        }
        e.mem.set_u32(first + NEXT_CONTROLLER, second);
        e.mem.set_u32(this + CONTROLLERS, first);
        let target = e.mem.alloc(0x10);
        e.call(0x00a8_0650, &args![this, target]);
        assert_eq!(e.mem.u32(target + 0xc), this + 0x9c);
        assert_eq!(calls_to(&e, slot(0xa8)), vec![vec![first], vec![second]]);
    }

    #[test]
    fn calculate_consistency_picks_static_or_mutable() {
        let (mut e, this, data) = rig(0);
        e.register(GEOMETRY_DATA_SET_CONSISTENCY, |_, _| Ret::default());
        let consistency = |e: &mut Engine, force: u32| {
            e.call(0x00a8_0930, &args![this, force]);
            calls_to(e, GEOMETRY_DATA_SET_CONSISTENCY)
                .last()
                .unwrap()
                .clone()
        };
        // No controllers, no skin: static.
        assert_eq!(consistency(&mut e, 0), vec![data, 0x4000]);
        // A controller answering true: mutable.
        let controller = e.mem.alloc(0x34);
        e.mem.set_u32(controller, VTABLE);
        e.mem.set_u32(this + CONTROLLERS, controller);
        assert_eq!(consistency(&mut e, 0)[1], 0);
        e.register(slot(0xa0), |_, _| 0u32.into_ret());
        e.mem.set_u32(this + CONTROLLERS, 0);
        assert_eq!(consistency(&mut e, 0)[1], 0x4000);
        // A skin without partitions: mutable either way.
        let skin = e.mem.alloc(0x10);
        e.mem.set_u32(this + SKIN_INSTANCE, skin);
        assert_eq!(consistency(&mut e, 1)[1], 0);
        assert_eq!(consistency(&mut e, 0)[1], 0);
        // With partitions: forced stays static; otherwise the listener decides.
        e.mem.set_u32(skin + 0xc, 0x0e70_0000);
        assert_eq!(consistency(&mut e, 1)[1], 0x4000);
        assert_eq!(consistency(&mut e, 0)[1], 0x4000);
        let listener = e.mem.alloc(4);
        e.mem.set_u32(listener, VTABLE);
        e.mem.set_u32(LIGHT_LISTENER_011F4748, listener);
        assert_eq!(consistency(&mut e, 0)[1], 0);
        e.register(slot(0xa0), |_, _| 2u32.into_ret());
        assert_eq!(consistency(&mut e, 0)[1], 0x4000);
    }

    #[test]
    fn purge_renderer_data_releases_data_and_skin() {
        let (mut e, this, data) = rig(0);
        e.register(AV_OBJECT_PURGE_RENDERER_DATA, |_, _| Ret::default());
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, VTABLE);
        e.call(0x00a8_09b0, &args![this, renderer]);
        assert_eq!(
            calls_to(&e, AV_OBJECT_PURGE_RENDERER_DATA),
            vec![vec![this, renderer]]
        );
        assert_eq!(calls_to(&e, slot(0xf0)), vec![vec![renderer, data]]);
        assert!(calls_to(&e, slot(0x104)).is_empty());
        let skin = e.mem.alloc(0x10);
        e.mem.set_u32(skin + 0xc, 0x0e70_0000);
        e.mem.set_u32(this + SKIN_INSTANCE, skin);
        e.call(0x00a8_09b0, &args![this, renderer]);
        assert_eq!(calls_to(&e, slot(0x104)), vec![vec![renderer, skin]]);
        assert_eq!(calls_to(&e, slot(0x100)), vec![vec![renderer, 0x0e70_0000]]);
    }

    #[test]
    fn save_binary_links_then_writes_zeros() {
        let (mut e, this, data) = rig(0);
        e.register(AV_OBJECT_SAVE_BINARY, |_, _| Ret::default());
        e.register(0x0300_0000, |e, a| {
            assert_eq!((e.mem.u32(a[3]), a[4]), (a[2], 1));
            let value = if a[2] == 1 {
                u32::from(e.mem.u8(a[1]))
            } else {
                e.mem.u32(a[1])
            };
            assert_eq!(value, 0);
            a[2].into_ret()
        });
        let stream = e.mem.alloc(0x260);
        let output = e.mem.alloc(0x10);
        e.mem.set_u32(stream, VTABLE);
        e.mem.set_u32(stream + 0x250, output);
        e.mem.set_u32(output + 0xc, 0x0300_0000);
        e.mem.set_u32(this + SKIN_INSTANCE, 0x0e70_0000);
        e.call(0x00a8_0db0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, AV_OBJECT_SAVE_BINARY),
            vec![vec![this, stream]]
        );
        assert_eq!(
            calls_to(&e, slot(0x30)),
            vec![vec![stream, data], vec![stream, 0x0e70_0000]]
        );
        let sizes: Vec<u32> = calls_to(&e, 0x0300_0000).iter().map(|c| c[2]).collect();
        assert_eq!(sizes, vec![4, 4, 1]);
        assert_eq!(e.mem.u32(output + 4), 9);
    }

    #[test]
    fn get_viewer_strings_lists_data_shader_and_partitions() {
        let (mut e, this, data) = rig(0);
        e.mem.set_u32(NI_GEOMETRY_RTTI, 0x0123_0000);
        for double in [AV_OBJECT_GET_VIEWER_STRINGS, STRINGS_GROW, STRINGS_SET_AT] {
            e.register(double, |_, _| Ret::default());
        }
        for text in [
            VIEWER_STRING_CLASS,
            VIEWER_STRING_OBJECT,
            VIEWER_STRING_FIXED_STRING,
            VIEWER_STRING_UNSIGNED,
        ] {
            e.register(text, |_, _| 0x0e60_0000u32.into_ret());
        }
        let strings = e.mem.alloc(0x10);
        e.mem.set_u16(strings + 8, 8);
        e.call(0x00a8_0e70, &args![this, strings]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_OBJECT),
            vec![vec![TEXT_GEOM_DATA, data]]
        );
        assert_eq!(calls_to(&e, STRINGS_SET_AT).len(), 2);
        // A shader (its slot +8 points at the handle) and a skin with partitions.
        let handle = e.mem.alloc(4);
        e.mem.set_u32(handle, 0x0e50_0000);
        e.register_double(slot(8), move |_, _| handle.into_ret());
        let shader = e.mem.alloc(4);
        e.mem.set_u32(shader, VTABLE);
        e.mem.set_u32(this + SHADER, shader);
        let skin = e.mem.alloc(0x10);
        let partitions = e.mem.alloc(0x10);
        e.mem.set_u32(partitions + 8, 6);
        e.mem.set_u32(skin + 0xc, partitions);
        e.mem.set_u32(this + SKIN_INSTANCE, skin);
        e.call(0x00a8_0e70, &args![this, strings]);
        assert_eq!(
            calls_to(&e, VIEWER_STRING_FIXED_STRING),
            vec![vec![TEXT_SHADER, 0x0e50_0000]]
        );
        assert_eq!(
            calls_to(&e, VIEWER_STRING_UNSIGNED),
            vec![vec![TEXT_SKIN_PARTITIONS, 6]]
        );
        assert_eq!(calls_to(&e, STRINGS_SET_AT).len(), 6);
        // A skin without partitions adds nothing.
        e.mem.set_u32(skin + 0xc, 0);
        e.call(0x00a8_0e70, &args![this, strings]);
        assert_eq!(calls_to(&e, VIEWER_STRING_UNSIGNED).len(), 1);
    }

    #[test]
    fn scalar_deleting_destructor_runs_the_destructor_and_deletes_on_bit_0() {
        let (mut e, this, _) = rig(0);
        for double in [
            PROPERTY_STATE_DESTRUCT,
            AV_OBJECT_DESTRUCT_00A5A390,
            SIZED_DELETE,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.mem.set_u32(this + MODEL_DATA, 0);
        assert_eq!(e.call(0x00a9_b1c0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_GEOMETRY_VTABLE);
        assert_eq!(
            calls_to(&e, PROPERTY_STATE_DESTRUCT),
            vec![vec![this + 0x9c]]
        );
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0xc4]]);
        e.call(0x00a9_b1c0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
    }
}
