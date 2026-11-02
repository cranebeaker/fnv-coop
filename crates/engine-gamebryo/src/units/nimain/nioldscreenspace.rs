//! `NiMain/nioldscreenspace.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::NiTArray;

/// `NiScreenGeometry::ms_RTTI` (the class's `NiRTTI` record).
const NI_SCREEN_GEOMETRY_RTTI: u32 = 0x011f_5e5c;

/// `NiScreenPolygon::ms_RTTI` (the class's `NiRTTI` record).
const NI_SCREEN_POLYGON_RTTI: u32 = 0x011f_5e64;

/// `NiScreenGeometryData::ms_RTTI` (the class's `NiRTTI` record).
const NI_SCREEN_GEOMETRY_DATA_RTTI: u32 = 0x011f_5e54;

/// `NiScreenSpaceCamera::ms_RTTI` (the class's `NiRTTI` record).
const NI_SCREEN_SPACE_CAMERA_RTTI: u32 = 0x011f_5e6c;

// Translated from 00a8ff70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometry::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_screen_geometry_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SCREEN_GEOMETRY_RTTI)
}

// Translated from 00a8ffb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenPolygon::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_screen_polygon_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SCREEN_POLYGON_RTTI)
}

// Translated from 00a90ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometryData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_screen_geometry_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SCREEN_GEOMETRY_DATA_RTTI)
}

// Translated from 00a91690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenSpaceCamera::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_screen_space_camera_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SCREEN_SPACE_CAMERA_RTTI)
}

/// `00a98ea0`: the jump target of `00a877d0`.
const JUMP_TARGET_00A877D0: u32 = 0x00a9_8ea0;

/// `00a71d10`: the jump target of `00a877e0`.
const JUMP_TARGET_00A877E0: u32 = 0x00a7_1d10;

/// `00a71450`: the jump target of `00a90000`.
const JUMP_TARGET_00A90000: u32 = 0x00a7_1450;

/// `00a716c0`: the jump target of `00a90020`.
const JUMP_TARGET_00A90020: u32 = 0x00a7_16c0;

/// `00a6ff20`: the jump target of `00a90030`.
const JUMP_TARGET_00A90030: u32 = 0x00a6_ff20;

// Translated from 00a877d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometry::LoadBinary` (Xbox PDB): a 5-byte jump to `00a98ea0` (engine map: `NiParticleMeshes::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiScreenGeometry@@UAAXAAVNiStream@@@Z`.
pub fn ni_screen_geometry_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A877D0, &args![this, stream]);
}

// Translated from 00a877e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometry::SaveBinary` (Xbox PDB): a 5-byte jump to `00a71d10` (engine map: `NiParticleMeshes::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiScreenGeometry@@UAAXAAVNiStream@@@Z`.
pub fn ni_screen_geometry_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A877E0, &args![this, stream]);
}

// Translated from 00a90000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenSpaceCamera::LoadBinary` (Xbox PDB): a 5-byte jump to `00a71450` (engine map: `NiCamera::LoadBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?LoadBinary@NiScreenSpaceCamera@@UAAXAAVNiStream@@@Z`.
pub fn ni_screen_space_camera_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A90000, &args![this, stream]);
}

// Translated from 00a90020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenSpaceCamera::SaveBinary` (Xbox PDB): a 5-byte jump to `00a716c0` (engine map: `NiCamera::SaveBinary`), passing on `this` and its one argument. Return type (void): Xbox PDB public name `?SaveBinary@NiScreenSpaceCamera@@UAAXAAVNiStream@@@Z`.
pub fn ni_screen_space_camera_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(JUMP_TARGET_00A90020, &args![this, stream]);
}

// Translated from 00a90030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenSpaceCamera::IsEqual` (Xbox PDB): a 5-byte jump to `00a6ff20` (engine map: `NiCamera::IsEqual`), passing on `this` and its one argument; returns the target result. Return type (bool): Xbox PDB public name `?IsEqual@NiScreenSpaceCamera@@UAA_NPAVNiObject@@@Z`.
pub fn ni_screen_space_camera_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    e.call(JUMP_TARGET_00A90030, &args![this, other]).bool()
}

/// `00aa1070` and `00aa10f0`: the `cdecl` allocate (size) and free (block).
const ALLOCATE: u32 = 0x00aa_1070;
const FREE: u32 = 0x00aa_10f0;

// Translated from 00a90180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `SetSize` of an `NiTArray`-shaped array of pointers
/// (`thiscall`, one stack word; the layout is [`NiTArray`]: 16-bit capacity,
/// used size and non-null count; the stream's group array is one). Does nothing when
/// `new_max` is the current capacity. When it is below the used size, the
/// elements from `new_max` on are cleared (the non-null count goes down for
/// each non-null one) and the used size becomes `new_max`. Then new storage of
/// `new_max & 0xffff` pointers (`00aa1070`) takes the used elements and its
/// other slots are zeroed, the old storage is freed (`00aa10f0`), and the
/// capacity is the low 16 bits of `new_max`; for `new_max` 0 the pointer is
/// null. All counts are 16-bit and wrap as the exe's do.
pub fn fn_00a90180(e: &mut Engine, this: Ptr<NiTArray>, new_max: u32) {
    if new_max == e.get(this, NiTArray::m_usMaxSize) as u32 {
        return;
    }
    let low = new_max as u16;
    if new_max < e.get(this, NiTArray::m_usSize) as u32 {
        let mut index = low;
        if index < e.get(this, NiTArray::m_usSize) {
            loop {
                let base = e.get(this, NiTArray::m_pBase);
                let slot = base.wrapping_add(index as u32 * 4);
                if e.mem.u32(slot) != 0 {
                    e.mem.set_u32(slot, 0);
                    let present = e.get(this, NiTArray::m_usESize);
                    e.set(this, NiTArray::m_usESize, present.wrapping_sub(1));
                }
                index = index.wrapping_add(1);
                if index >= e.get(this, NiTArray::m_usSize) {
                    break;
                }
            }
        }
        e.set(this, NiTArray::m_usSize, low);
    }
    let old = e.get(this, NiTArray::m_pBase);
    e.set(this, NiTArray::m_usMaxSize, low);
    if new_max != 0 {
        let storage = e.call(ALLOCATE, &args![(low as u32) * 4]).u32();
        e.set(this, NiTArray::m_pBase, storage);
        let size = e.get(this, NiTArray::m_usSize);
        if size != 0 {
            let mut index = 0u16;
            loop {
                let base = e.get(this, NiTArray::m_pBase);
                let value = e.mem.u32(old.wrapping_add(index as u32 * 4));
                e.mem.set_u32(base.wrapping_add(index as u32 * 4), value);
                index = index.wrapping_add(1);
                if index >= e.get(this, NiTArray::m_usSize) {
                    break;
                }
            }
        }
        let mut index = e.get(this, NiTArray::m_usSize);
        if index < e.get(this, NiTArray::m_usMaxSize) {
            loop {
                let base = e.get(this, NiTArray::m_pBase);
                e.mem.set_u32(base.wrapping_add(index as u32 * 4), 0);
                index = index.wrapping_add(1);
                if index >= e.get(this, NiTArray::m_usMaxSize) {
                    break;
                }
            }
        }
    } else {
        e.set(this, NiTArray::m_pBase, 0);
    }
    e.call(FREE, &args![old]);
}

/// `011f5e50`: the pointer `00a91290` stores the object it builds in.
const SCREEN_SPACE_OBJECT_SLOT: u32 = 0x011f_5e50;
/// The memory manager (`011f6238`), the `this` of its allocate `00aa3e40`.
const MEMORY_MANAGER_OBJECT: u32 = 0x011f_6238;
/// `00aa3e40` (`thiscall`, one stack word): the memory manager's allocate.
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;

// Translated from 00a91290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nioldscreenspace.cpp`; run by the start-up
/// `00a89cc0`, no arguments): allocates 0x18 bytes from the memory manager, zeroes the
/// first four words, sets the words at `+0x10` and `+0x14` to 8 and stores the block in
/// `011f5e50` (null when the allocation fails).
pub fn fn_00a91290(e: &mut Engine) {
    let block = e
        .call(
            MEMORY_MANAGER_ALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, 0x18u32],
        )
        .u32();
    if block == 0 {
        e.mem.set_u32(SCREEN_SPACE_OBJECT_SLOT, 0);
        return;
    }
    e.mem.set_u32(block, 0);
    e.mem.set_u32(block + 4, 0);
    e.mem.set_u32(block + 8, 0);
    e.mem.set_u32(block + 0xc, 0);
    e.mem.set_u32(block + 0x10, 8);
    e.mem.set_u32(block + 0x14, 8);
    e.mem.set_u32(SCREEN_SPACE_OBJECT_SLOT, block);
}

/// `00aa4060` (`thiscall` on the memory manager, one stack word): the memory manager's
/// deallocate.
const MEMORY_MANAGER_DEALLOCATE: u32 = 0x00aa_4060;
/// `00e7f900` (`thiscall`, one stack word; unit `NiXenonRenderer/nid3dpass.cpp`).
const D3D_PASS_RELEASE: u32 = 0x00e7_f900;

// Translated from 00a90ac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nioldscreenspace.cpp`, `thiscall`, no stack word): the
/// tear-down of the 0x18-byte object `00a91290` builds. Clears the word at +8; when the
/// pointer at +0xc is set, that block's first word is disposed of (when the word in front of
/// it is 0 the block from that word on goes back to the memory manager, otherwise its vtable
/// slot 0 runs with 3), `00e7f900(1)` runs on its third word when that is set, and the block
/// goes back to the memory manager (`00aa4060`). Last, the storage at +0 is freed
/// (`00aa10f0`).
pub fn fn_00a90ac0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let inner = e.mem.u32(base + 0xc);
    e.mem.set_u32(base + 8, 0);
    if inner != 0 {
        let first = e.mem.u32(inner);
        if first != 0 {
            let header = first.wrapping_sub(4);
            if e.mem.u32(header) == 0 {
                e.call(
                    MEMORY_MANAGER_DEALLOCATE,
                    &args![MEMORY_MANAGER_OBJECT, header],
                );
            } else {
                e.vcall(first, 0, &args![3u32]);
            }
        }
        let third = e.mem.u32(inner + 8);
        if third != 0 {
            e.call(D3D_PASS_RELEASE, &args![third, 1u32]);
        }
        e.call(
            MEMORY_MANAGER_DEALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, inner],
        );
    }
    let storage = e.mem.u32(base);
    e.call(FREE, &args![storage]);
}

// Translated from 00a91930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nioldscreenspace.cpp`, no arguments; one of the `NiMain`
/// shutdown steps of `00a8a280`): when `011f5e50` holds the object `00a91290` built, tears it
/// down ([`fn_00a90ac0`]) and gives it back to the memory manager. The pointer is not
/// cleared.
pub fn fn_00a91930(e: &mut Engine) {
    let object = e.mem.u32(SCREEN_SPACE_OBJECT_SLOT);
    if object != 0 {
        fn_00a90ac0(e, Ptr::new(object));
        e.call(
            MEMORY_MANAGER_DEALLOCATE,
            &args![MEMORY_MANAGER_OBJECT, object],
        );
    }
}

/// The Ni allocator `operator new` (`00aa13e0`, `cdecl`, the size) and the sized delete
/// (`00aa1460`, `cdecl`: pointer, size).
const NI_NEW: u32 = 0x00aa_13e0;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// Import slots of `InterlockedIncrement` / `InterlockedDecrement` (`stdcall`, one word).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// `_atexit` (`00ec658f`, `cdecl`, the function).
const ATEXIT: u32 = 0x00ec_658f;
/// The compiler's `vector constructor iterator` (`00401050`, `stdcall`: block, element size,
/// count, constructor), the screen element's constructor (`00a8ff50`) and `NiColorA`'s
/// default constructor (`004a7800`) it is given.
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
const SCREEN_ELEMENT_CONSTRUCT: u32 = 0x00a8_ff50;
const COLOR_CONSTRUCT: u32 = 0x004a_7800;

/// Vtables: `NiScreenGeometry` (`0109f81c`), `NiScreenPolygon` (`0109f914`),
/// `NiScreenGeometryData::ScreenElement` (`0109f814`), `NiScreenGeometryData` (`0109f9c4`),
/// `NiScreenSpaceCamera` (`0109fab4`); the element arrays `NiTArray<ScreenElement *,
/// NiTMallocInterface<..>>` (`0109f9a4`) and `NiTPrimitiveArray<ScreenElement *>`
/// (`0109f9ac`); the camera's arrays `NiTArray<NiPointer<NiScreenPolygon>,
/// NiTNewInterface<..>>` (`0109f9b4`), `NiTArray<NiPointer<NiScreenTexture>, ..>`
/// (`0109f9bc`), `NiTObjectArray<NiPointer<NiScreenPolygon>>` (`0109fa7c`) and
/// `NiTObjectArray<NiPointer<NiScreenTexture>>` (`0109fa84`) (all by their RTTI).
const NI_SCREEN_GEOMETRY_VTABLE: u32 = 0x0109_f81c;
const NI_SCREEN_POLYGON_VTABLE: u32 = 0x0109_f914;
const SCREEN_ELEMENT_VTABLE: u32 = 0x0109_f814;
const NI_SCREEN_GEOMETRY_DATA_VTABLE: u32 = 0x0109_f9c4;
const NI_SCREEN_SPACE_CAMERA_VTABLE: u32 = 0x0109_fab4;
const ELEMENT_ARRAY_VTABLE: u32 = 0x0109_f9a4;
const ELEMENT_PRIMITIVE_ARRAY_VTABLE: u32 = 0x0109_f9ac;
const POLYGON_ARRAY_VTABLE: u32 = 0x0109_f9b4;
const TEXTURE_ARRAY_VTABLE: u32 = 0x0109_f9bc;
const POLYGON_OBJECT_ARRAY_VTABLE: u32 = 0x0109_fa7c;
const TEXTURE_OBJECT_ARRAY_VTABLE: u32 = 0x0109_fa84;

/// The base classes' functions these units start or end with (`thiscall`): the triangle
/// shape's constructor (`00a744a0`), member copy (`00a744c0`, engine map
/// `NiParticleMeshes::CopyMembers`, folded) and destructor body (`00a89760`);
/// `NiObject`'s constructor (`00a5d3a0`), destructor body (`00a5d3d0`), `IsEqual`
/// (`00a5d470`), `RegisterStreamables` (`00a5d2f0`), clone recording (`00a5d450`),
/// `LoadBinary` (`00a5d610`) and `GetViewerStrings` (`00a5d6e0`); the empty `00a89af0`;
/// `NiAVObject::ProcessClone` (`00a59ec0`).
const SHAPE_CONSTRUCT: u32 = 0x00a7_44a0;
const SHAPE_COPY_MEMBERS: u32 = 0x00a7_44c0;
const SHAPE_DESTRUCT: u32 = 0x00a8_9760;
const NI_OBJECT_CONSTRUCT: u32 = 0x00a5_d3a0;
const NI_OBJECT_DESTRUCT: u32 = 0x00a5_d3d0;
const NI_OBJECT_IS_EQUAL: u32 = 0x00a5_d470;
const NI_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_d2f0;
const NI_OBJECT_RECORD_CLONE: u32 = 0x00a5_d450;
const NI_OBJECT_LOAD_BINARY: u32 = 0x00a5_d610;
const NI_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_d6e0;
const EMPTY_BASE: u32 = 0x00a8_9af0;
const AV_PROCESS_CLONE: u32 = 0x00a5_9ec0;
/// `NiTriShapeData` (`thiscall`): its constructor `NiTriShapeData_ov2` (`00a7b690`),
/// `IsEqual` (`00a7b7e0`), destructor (`00a7b8c0`), `LoadBinary` (`00a7b940`) and
/// `SaveBinary` (`00a7bbe0`); `NiGeometryData::LoadBinary` (`00a67ea0`) and
/// `GetViewerStrings` (`00a688b0`); `NiTriShape::GetViewerStrings` (`00a74690`).
const TRI_SHAPE_DATA_CONSTRUCT: u32 = 0x00a7_b690;
const TRI_SHAPE_DATA_IS_EQUAL: u32 = 0x00a7_b7e0;
const TRI_SHAPE_DATA_DESTRUCT: u32 = 0x00a7_b8c0;
const TRI_SHAPE_DATA_LOAD_BINARY: u32 = 0x00a7_b940;
const TRI_SHAPE_DATA_SAVE_BINARY: u32 = 0x00a7_bbe0;
const GEOMETRY_DATA_LOAD_BINARY: u32 = 0x00a6_7ea0;
const GEOMETRY_DATA_GET_VIEWER_STRINGS: u32 = 0x00a6_88b0;
const TRI_SHAPE_GET_VIEWER_STRINGS: u32 = 0x00a7_4690;
/// `NiCamera` (`thiscall`): constructor (`00a712f0`), member copy (`00a70f60`),
/// `UpdateWorldData` (`00a711b0`), `UpdateWorldBound` (`00a70b40`), `LinkObject`
/// (`00a711d0`), `GetViewerStrings` (`00a71920`) and destructor body (`00a6fae0`).
const CAMERA_CONSTRUCT: u32 = 0x00a7_12f0;
const CAMERA_COPY_MEMBERS: u32 = 0x00a7_0f60;
const CAMERA_UPDATE_WORLD_DATA: u32 = 0x00a7_11b0;
const CAMERA_UPDATE_WORLD_BOUND: u32 = 0x00a7_0b40;
const CAMERA_LINK_OBJECT: u32 = 0x00a7_11d0;
const CAMERA_GET_VIEWER_STRINGS: u32 = 0x00a7_1920;
const CAMERA_DESTRUCT: u32 = 0x00a6_fae0;
/// Helpers of the camera's pointer arrays: `00a47770` (`thiscall` on an array, the new size;
/// engine map `NiTArray<NiPointer<NiControllerSequence>,..>::SetSize`, folded), `00a2e620`
/// (`thiscall` on an array, no stack words), `00a47600` (`cdecl`, an element block: the
/// array delete) and `00c1d790` (`cdecl`, a count: the array new).
const POINTER_ARRAY_SET_SIZE: u32 = 0x00a4_7770;
const POINTER_ARRAY_00A2E620: u32 = 0x00a2_e620;
const POINTER_ARRAY_DELETE: u32 = 0x00a4_7600;
const POINTER_ARRAY_NEW: u32 = 0x00c1_d790;
/// `00ba3aa0` (`thiscall` on the element array: index, address of the element): stores an
/// element.
const ELEMENT_ARRAY_SET_AT: u32 = 0x00ba_3aa0;
/// `00e7ec00` (`thiscall` on the element pool at `011f5e50`, the address of the element;
/// engine map `NiControllerSequence::SetState`, folded): gives an element back to the pool.
const POOL_RELEASE: u32 = 0x00e7_ec00;
/// `00a311e0` (`thiscall` on the element pool's free array, the new capacity; engine map
/// `NiTSet<NiNode *,..>::Realloc`, folded).
const FREE_LIST_REALLOC: u32 = 0x00a3_11e0;
/// The `SetAt` of one of the camera's object arrays (`00a908b0`, `00a90980`).
type SetAt = fn(&mut Engine, Ptr, u32, Ptr);
/// `NiPropertyState`'s copy constructor (`00a89810`, `thiscall`, the state).
const PROPERTY_STATE_COPY: u32 = 0x00a8_9810;
/// Points and colors (`thiscall` on the value, the stream): `NiColor::LoadBinary`
/// (`00a59be0`) / `SaveBinary` (`00a69780`) (three floats, used for the polygon's
/// vertices), `NiColorA::LoadBinary` (`00a69800`) / `SaveBinary` (`00a698a0`), and the
/// two-float load `00a68c80` / save `00a68ce0`.
const POINT3_LOAD_BINARY: u32 = 0x00a5_9be0;
const POINT3_SAVE_BINARY: u32 = 0x00a6_9780;
const COLOR_A_LOAD_BINARY: u32 = 0x00a6_9800;
const COLOR_A_SAVE_BINARY: u32 = 0x00a6_98a0;
const POINT2_LOAD_BINARY: u32 = 0x00a6_8c80;
const POINT2_SAVE_BINARY: u32 = 0x00a6_8ce0;
/// `0051c330` and `004a4c90` (`thiscall` on a value, the other): true when two points / two
/// colors differ.
const POINT2_DIFFERS: u32 = 0x0051_c330;
const COLOR_A_DIFFERS: u32 = 0x004a_4c90;
/// `NiStream` (`thiscall`): `GetNumberOfLinkIDs` (`00a63460`), `GetObjectFromLinkID`
/// (`00a63430`), `ReadMultipleLinkIDs` (`00a64670`); its file version (+0xd8), input
/// (+0x24c) and output (+0x250); its `SaveLinkID` (vtable +0x30).
const STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
const STREAM_FILE_VERSION: u32 = 0xd8;
const STREAM_SAVE_LINK_ID_SLOT: u32 = 0x30;
/// Vtable slots: `DeleteThis` (+4), `GetRTTI` (+8), `CreateClone` (+0x48),
/// `RegisterStreamables` (+0x54), `ProcessClone` (+0x68), `NiProperty::Type` (+0x8c).
const SLOT_DELETE_THIS: u32 = 0x4;
const SLOT_GET_RTTI: u32 = 0x8;
const SLOT_CREATE_CLONE: u32 = 0x48;
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
const SLOT_PROCESS_CLONE: u32 = 0x68;
const SLOT_PROPERTY_TYPE: u32 = 0x8c;
/// Viewer string builders (`cdecl`): the class name (`00a7d330`), a flag (`00a7d370`), a
/// 16-bit value (`00a7d490`), an unsigned value (`00a7d510`) and a pointer (`00a7d5a0`).
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
const VIEWER_STRING_SHORT: u32 = 0x00a7_d490;
const VIEWER_STRING_UNSIGNED: u32 = 0x00a7_d510;
const VIEWER_STRING_POINTER: u32 = 0x00a7_d5a0;
/// `NiTArray<char *>` resize (`0096ad30`) and `SetAt` (`0096ae90`).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The labels: `"m_bPixelAccurate"` (`0109fa98`), `"Num Elements"` (`0109fa88`),
/// `"m_usVertices"` (`0109c14c`), `"m_pkVertex"` (`0109c140`), `"m_pkTexture"`
/// (`0109c0e4`), `"m_pkColor"` (`0109c104`).
const TEXT_PIXEL_ACCURATE: u32 = 0x0109_fa98;
const TEXT_NUM_ELEMENTS: u32 = 0x0109_fa88;
const TEXT_VERTICES: u32 = 0x0109_c14c;
const TEXT_VERTEX: u32 = 0x0109_c140;
const TEXT_TEXTURE: u32 = 0x0109_c0e4;
const TEXT_COLOR: u32 = 0x0109_c104;
/// The null-pointer statics the two `SetAt` instances compare with (`NiPointer` values with
/// their guard words and `atexit` destructors): polygons `011f5e74` (guard `011f5e78`,
/// `00fdb140`), textures `011f5e7c` (guard `011f5e80`, `00fdb110`).
const POLYGON_NULL: u32 = 0x011f_5e74;
const POLYGON_NULL_GUARD: u32 = 0x011f_5e78;
const POLYGON_NULL_DESTRUCT: u32 = 0x00fd_b140;
const TEXTURE_NULL: u32 = 0x011f_5e7c;
const TEXTURE_NULL_GUARD: u32 = 0x011f_5e80;
const TEXTURE_NULL_DESTRUCT: u32 = 0x00fd_b110;

/// `NiScreenGeometryData` (Xbox PDB) fields at their PC offsets: `m_bPixelAccurate` (+0x58),
/// `m_bUnorderedElements` (+0x59), `m_usActiveVerts` (+0x5c) and the element array
/// `m_kScreenElements` (+0x60: element block +0x64, capacity +0x68, used size +0x6a,
/// count +0x6c, grow-by +0x6e, all 16-bit); `NiGeometryData`'s vertex count (+8, 16-bit) and
/// `NiTriShapeData`'s triangle count (+0x40, 16-bit), list length (+0x44) and list (+0x48).
const PIXEL_ACCURATE: u32 = 0x58;
const ELEMENTS: u32 = 0x60;
/// A screen element (`NiScreenGeometryData::ScreenElement`, Xbox PDB, 0x14 bytes):
/// `m_usNumVerts` (+4), `m_usNumTexCoordSets` (+6), `m_pkVerts` (+8, two floats each),
/// `m_pkColors` (+0xc, four floats each), `m_pkTexCoords` (+0x10, two floats each).
const ELEMENT_SIZE: u32 = 0x14;
/// `NiScreenPolygon` (Xbox PDB): `m_spPropertyState` (+8, seven property pointers),
/// `m_usVertices` (+0xc), `m_pkVertex` (+0x10, three floats each), `m_pkTexture` (+0x14,
/// two floats each), `m_pkColor` (+0x18, four floats each).
const POLYGON_PROPERTIES: u32 = 0x8;
const POLYGON_VERTEX_COUNT: u32 = 0xc;
const POLYGON_VERTICES: u32 = 0x10;
const POLYGON_TEXTURE: u32 = 0x14;
const POLYGON_COLORS: u32 = 0x18;
/// `NiScreenSpaceCamera`'s two pointer arrays as the PC code reads them: the polygons at
/// +0x114 and the textures at +0x124 (each: element block +4, capacity +8, used size +0xa,
/// count +0xc, grow-by +0xe, 16-bit).
const CAMERA_POLYGONS: u32 = 0x114;
const CAMERA_TEXTURES: u32 = 0x124;

/// `InterlockedDecrement` of the count at `object + 4`; `DeleteThis` at zero.
fn release(e: &mut Engine, object: u32) {
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, SLOT_DELETE_THIS, &args![]);
    }
}

fn acquire(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

fn memory_allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_MANAGER_ALLOCATE, &args![MEMORY_MANAGER_OBJECT, size])
        .u32()
}

fn memory_deallocate(e: &mut Engine, block: u32) {
    e.call(
        MEMORY_MANAGER_DEALLOCATE,
        &args![MEMORY_MANAGER_OBJECT, block],
    );
}

/// `count * size` as the code sizes a block: `0xffffffff` when the product overflows.
fn block_bytes(count: u32, size: u32) -> u32 {
    count.checked_mul(size).unwrap_or(u32::MAX)
}

/// One read from the stream's input (`NiStream` +0x24c, read function +8, position +4):
/// `size` bytes into `destination`, the component-size word at `component` set first.
fn read(e: &mut Engine, stream: u32, destination: u32, size: u32, component: u32, unit: u32) {
    let input = e.mem.u32(stream + 0x24c);
    let function = e.mem.u32(input + 8);
    e.mem.set_u32(component, unit);
    let done = e
        .call(function, &args![input, destination, size, component, 1u32])
        .u32();
    let position = e.mem.u32(input + 4).wrapping_add(done);
    e.mem.set_u32(input + 4, position);
}

/// One write to the stream's output (`NiStream` +0x250, write function +0xc, position +4).
fn write(e: &mut Engine, stream: u32, source: u32, size: u32, component: u32, unit: u32) {
    let output = e.mem.u32(stream + 0x250);
    let function = e.mem.u32(output + 0xc);
    e.mem.set_u32(component, unit);
    let done = e
        .call(function, &args![output, source, size, component, 1u32])
        .u32();
    let position = e.mem.u32(output + 4).wrapping_add(done);
    e.mem.set_u32(output + 4, position);
}

/// Adds the string in the word at `slot` to the viewer strings: the array grown by its
/// grow-by count when full (`0096ad30`), then `SetAt` (`0096ae90`) at the old size.
fn add_viewer_string(e: &mut Engine, strings: u32, slot: u32) {
    let index = u32::from(e.mem.u16(strings + 0xa));
    if index >= u32::from(e.mem.u16(strings + 8)) {
        let grow = u32::from(e.mem.u16(strings + 0xe));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

/// A viewer string added through the word at `slot`.
fn add_text(e: &mut Engine, strings: u32, slot: u32, text: u32) {
    e.mem.set_u32(slot, text);
    add_viewer_string(e, strings, slot);
}

// Translated from 00a8ff80 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiScreenGeometry`): allocates 0xc4 bytes (`00aa13e0`) and, when that
/// succeeds, runs the base constructor (`00a744a0`) and sets `NiScreenGeometry`'s vtable.
/// Answers the object, or null.
pub fn fn_00a8ff80(e: &mut Engine) -> Ptr {
    let object = e.call(NI_NEW, &args![0xc4u32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(SHAPE_CONSTRUCT, &args![object]);
    e.mem.set_u32(object, NI_SCREEN_GEOMETRY_VTABLE);
    Ptr::new(object)
}

// Translated from 00a8ffc0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiScreenPolygon`): allocates 0x1c bytes (`00aa13e0`) and, when that
/// succeeds, runs `NiObject`'s constructor, clears the vertex count and the three arrays and
/// sets `NiScreenPolygon`'s vtable. Answers the object, or null.
pub fn fn_00a8ffc0(e: &mut Engine) -> Ptr {
    let object = e.call(NI_NEW, &args![0x1cu32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    e.call(NI_OBJECT_CONSTRUCT, &args![object]);
    e.mem.set_u16(object + POLYGON_VERTEX_COUNT, 0);
    e.mem.set_u32(object + POLYGON_VERTICES, 0);
    e.mem.set_u32(object + POLYGON_TEXTURE, 0);
    e.mem.set_u32(object + POLYGON_COLORS, 0);
    e.mem.set_u32(object, NI_SCREEN_POLYGON_VTABLE);
    Ptr::new(object)
}

// Translated from 00a90040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometry::_scalar_deleting_destructor_` (Xbox PDB): the vtable, the base
/// destructor body (`00a89760`), then the sized delete of 0xc4 bytes when bit 0 of `flags`
/// is set. Answers `this`.
pub fn ni_screen_geometry_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_SCREEN_GEOMETRY_VTABLE);
    e.call(SHAPE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xc4u32]);
    }
    this
}

// Translated from 00a90070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometry::CreateClone` (Xbox PDB): a new object as `00a8ff80` makes it (0xc4
/// bytes, the base constructor, the vtable), then the base member copy (`00a744c0`) from
/// `this` into it with the cloning process; when the allocation failed the copy still runs,
/// with a null clone. Answers the clone.
pub fn ni_screen_geometry_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = e.call(NI_NEW, &args![0xc4u32]).u32();
    if clone != 0 {
        e.call(SHAPE_CONSTRUCT, &args![clone]);
        e.mem.set_u32(clone, NI_SCREEN_GEOMETRY_VTABLE);
    }
    e.call(SHAPE_COPY_MEMBERS, &args![this, clone, cloning]);
    Ptr::new(clone)
}

// Translated from 00a900c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenPolygon::RegisterStreamables` (Xbox PDB): false when
/// `NiObject::RegisterStreamables` is; otherwise each of the seven properties of the
/// property state (+8) registers itself (vtable +0x54; none is checked for null), and true.
pub fn ni_screen_polygon_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if !e
        .call(NI_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
    {
        return false;
    }
    for offset in (0..0x1c).step_by(4) {
        let state = e.mem.u32(this.addr() + POLYGON_PROPERTIES);
        let property = e.mem.u32(state.wrapping_add(offset));
        e.vcall(property, SLOT_REGISTER_STREAMABLES, &args![stream]);
    }
    true
}

/// Calls `call` on each non-null pointer of the 16-bit-sized array at `array` (element
/// block +4, used size +0xa read again after each one).
fn for_each_pointer(e: &mut Engine, array: u32, call: &mut dyn FnMut(&mut Engine, u32)) {
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(array + 0xa)) {
        let base = e.mem.u32(array + 4);
        let object = e.mem.u32(base.wrapping_add(4 * index));
        if object != 0 {
            call(e, object);
        }
        index += 1;
    }
}

// Translated from 00a90100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenSpaceCamera::ProcessClone` (Xbox PDB): `NiAVObject::ProcessClone`, then every
/// polygon and every texture of the camera's two arrays runs its `ProcessClone` (vtable
/// +0x68) with the cloning process.
pub fn ni_screen_space_camera_process_clone(e: &mut Engine, this: Ptr, cloning: Ptr) {
    e.call(AV_PROCESS_CLONE, &args![this, cloning]);
    for array in [CAMERA_POLYGONS, CAMERA_TEXTURES] {
        for_each_pointer(e, this.addr() + array, &mut |e, object| {
            e.vcall(object, SLOT_PROCESS_CLONE, &args![cloning]);
        });
    }
}

// Translated from 00a90260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the scalar deleting destructor of `NiTArray<ScreenElement *,
/// NiTMallocInterface<..>>` by the vtable it sets, `0109f9a4`): the vtable, the element block
/// freed (`00aa10f0`), and for bit 0 of `flags` the array given back to the memory manager.
/// Answers `this`.
pub fn fn_00a90260(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let block = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(this.addr(), ELEMENT_ARRAY_VTABLE);
    e.call(FREE, &args![block]);
    if flags & 1 != 0 {
        memory_deallocate(e, this.addr());
    }
    this
}

// Translated from 00a90290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, the count; by its body the array `new` of screen
/// elements): a block of `count` 0x14-byte elements behind a count word, from the memory
/// manager (the size `count * 0x14 + 4`, `0xffffffff` on overflow), each element constructed
/// by `00a8ff50` through the vector constructor iterator. Answers the first element, or null.
pub fn fn_00a90290(e: &mut Engine, count: u32) -> Ptr {
    let bytes = block_bytes(count, ELEMENT_SIZE);
    let size = bytes.checked_add(4).unwrap_or(u32::MAX);
    let block = memory_allocate(e, size);
    if block == 0 {
        return Ptr::NULL;
    }
    let first = block + 4;
    e.mem.set_u32(block, count);
    e.call(
        VECTOR_CONSTRUCT,
        &args![first, ELEMENT_SIZE, count, SCREEN_ELEMENT_CONSTRUCT],
    );
    Ptr::new(first)
}

/// The inlined screen element destructor: its vtable, then its vertex, color and
/// texture-coordinate blocks given back to the memory manager (each word read when used).
fn destroy_element(e: &mut Engine, element: u32) {
    let vertices = e.mem.u32(element + 8);
    e.mem.set_u32(element, SCREEN_ELEMENT_VTABLE);
    memory_deallocate(e, vertices);
    let colors = e.mem.u32(element + 0xc);
    memory_deallocate(e, colors);
    let coordinates = e.mem.u32(element + 0x10);
    memory_deallocate(e, coordinates);
}

// Translated from 00a902e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometryData::ScreenElement::_vector_deleting_destructor_` (Xbox PDB): with bit
/// 1 of `flags` an array: the count word before `this`, the elements destroyed last first
/// (while the count minus one, as a signed number, stays at or above 0), and with bit 0 the
/// block (from the count word) given back to the memory manager; answers the block. Without
/// bit 1 one element destroyed and, with bit 0, given back; answers `this`.
pub fn screen_element_vector_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    if flags & 2 != 0 {
        let block = this.addr().wrapping_sub(4);
        let count = e.mem.u32(block);
        let mut end = this
            .addr()
            .wrapping_add(count.wrapping_mul(5).wrapping_mul(4));
        let mut remaining = (count as i32).wrapping_sub(1);
        while remaining >= 0 {
            end = end.wrapping_sub(ELEMENT_SIZE);
            destroy_element(e, end);
            remaining = remaining.wrapping_sub(1);
        }
        if flags & 1 != 0 {
            memory_deallocate(e, block);
        }
        return Ptr::new(block);
    }
    destroy_element(e, this.addr());
    if flags & 1 != 0 {
        memory_deallocate(e, this.addr());
    }
    this
}

// Translated from 00a903a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometryData::IsEqual` (Xbox PDB): `NiTriShapeData::IsEqual`, the same pixel
/// accuracy byte, element count (+0x6c) and used size (+0x6a), then element by element
/// (over the count): both null or both present with the same vertex and texture-set counts,
/// and per vertex the same point (`0051c330`), color (`004a4c90`) and texture coordinates.
pub fn ni_screen_geometry_data_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let (a, b) = (this.addr(), other.addr());
    if !e.call(TRI_SHAPE_DATA_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    if e.mem.u8(b + PIXEL_ACCURATE) != e.mem.u8(a + PIXEL_ACCURATE) {
        return false;
    }
    let count = u32::from(e.mem.u16(b + 0x6c));
    if count != u32::from(e.mem.u16(a + 0x6c)) {
        return false;
    }
    if e.mem.u16(b + 0x6a) != e.mem.u16(a + 0x6a) {
        return false;
    }
    let mine_base = e.mem.u32(a + 0x64);
    let theirs_offset = e.mem.u32(b + 0x64).wrapping_sub(mine_base);
    for index in 0..count {
        let slot = mine_base.wrapping_add(4 * index);
        let theirs = e.mem.u32(slot.wrapping_add(theirs_offset));
        let mine = e.mem.u32(slot);
        if theirs == 0 {
            if mine != 0 {
                return false;
            }
            continue;
        }
        if mine == 0 {
            return false;
        }
        let vertices = u32::from(e.mem.u16(theirs + 4));
        let sets = u32::from(e.mem.u16(theirs + 6));
        if vertices != u32::from(e.mem.u16(mine + 4)) || sets != u32::from(e.mem.u16(mine + 6)) {
            return false;
        }
        let (mut point, other_point) = (e.mem.u32(theirs + 8), e.mem.u32(mine + 8));
        let (mut color, other_color) = (e.mem.u32(theirs + 0xc), e.mem.u32(mine + 0xc));
        let (mut coordinates, other_coordinates) =
            (e.mem.u32(theirs + 0x10), e.mem.u32(mine + 0x10));
        let point_offset = other_point.wrapping_sub(point);
        let color_offset = other_color.wrapping_sub(color);
        let coordinate_offset = other_coordinates.wrapping_sub(coordinates);
        for _ in 0..vertices {
            let pair = point.wrapping_add(point_offset);
            if e.call(POINT2_DIFFERS, &args![point, pair]).bool() {
                return false;
            }
            let pair = color.wrapping_add(color_offset);
            if e.call(COLOR_A_DIFFERS, &args![color, pair]).bool() {
                return false;
            }
            let mut coordinate = coordinates;
            for _ in 0..sets {
                let pair = coordinate.wrapping_add(coordinate_offset);
                if e.call(POINT2_DIFFERS, &args![coordinate, pair]).bool() {
                    return false;
                }
                coordinate = coordinate.wrapping_add(8);
            }
            point = point.wrapping_add(8);
            color = color.wrapping_add(0x10);
            coordinates = coordinates.wrapping_add(sets.wrapping_mul(8));
        }
    }
    true
}

// Translated from 00a90560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenPolygon::_scalar_deleting_destructor_` (Xbox PDB): the vtable, the vertex,
/// texture and color blocks given back to the memory manager, `NiObject`'s destructor body,
/// then the sized delete of 0x1c bytes when bit 0 of `flags` is set. Answers `this`.
pub fn ni_screen_polygon_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let polygon = this.addr();
    let vertices = e.mem.u32(polygon + POLYGON_VERTICES);
    e.mem.set_u32(polygon, NI_SCREEN_POLYGON_VTABLE);
    memory_deallocate(e, vertices);
    for field in [POLYGON_TEXTURE, POLYGON_COLORS] {
        let block = e.mem.u32(polygon + field);
        memory_deallocate(e, block);
    }
    e.call(NI_OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x1cu32]);
    }
    this
}

// Translated from 00a905c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenPolygon::CreateClone` (Xbox PDB): a new, empty polygon as `00a8ffc0` makes it
/// (null when the allocation fails), recorded in the cloning process (`00a5d450`), and its
/// property state a copy of this one's (0x1c bytes from the memory manager,
/// `NiPropertyState`'s copy constructor `00a89810`), or null when that allocation fails.
/// Answers the clone.
pub fn ni_screen_polygon_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let clone = e.call(NI_NEW, &args![0x1cu32]).u32();
    if clone != 0 {
        e.call(NI_OBJECT_CONSTRUCT, &args![clone]);
        e.mem.set_u32(clone, NI_SCREEN_POLYGON_VTABLE);
        e.mem.set_u16(clone + POLYGON_VERTEX_COUNT, 0);
        e.mem.set_u32(clone + POLYGON_VERTICES, 0);
        e.mem.set_u32(clone + POLYGON_TEXTURE, 0);
        e.mem.set_u32(clone + POLYGON_COLORS, 0);
    }
    e.call(NI_OBJECT_RECORD_CLONE, &args![this, clone, cloning]);
    let block = memory_allocate(e, 0x1c);
    let state = if block != 0 {
        let original = e.mem.u32(this.addr() + POLYGON_PROPERTIES);
        e.call(PROPERTY_STATE_COPY, &args![block, original]).u32()
    } else {
        0
    };
    e.mem.set_u32(clone.wrapping_add(POLYGON_PROPERTIES), state);
    Ptr::new(clone)
}

// Translated from 00a90640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenPolygon::LinkObject` (Xbox PDB): the empty base `00a89af0`, then for each of
/// the stream's link ids (`GetNumberOfLinkIDs`, counting down) the linked object
/// (`GetObjectFromLinkID`); a property whose type (vtable +0x8c) is at most 7 replaces the
/// property state's entry of that type (the old one released, the new one counted).
pub fn ni_screen_polygon_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(EMPTY_BASE, &args![this, stream]);
    let mut remaining = e.call(STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    while remaining != 0 {
        remaining -= 1;
        let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        let state = e.mem.u32(this.addr() + POLYGON_PROPERTIES);
        if object == 0 {
            continue;
        }
        if e.vcall(object, SLOT_PROPERTY_TYPE, &args![]).i32() > 7 {
            continue;
        }
        let kind = e.vcall(object, SLOT_PROPERTY_TYPE, &args![]).u32();
        let slot = state.wrapping_add(kind.wrapping_mul(4));
        let old = e.mem.u32(slot);
        if old == object {
            continue;
        }
        if old != 0 {
            release(e, old);
        }
        e.mem.set_u32(slot, object);
        acquire(e, object);
    }
}

/// The floats at `a` and `b` compared as `FLD a; FLD b; FUCOMPP` with an equality test.
fn same_float(e: &Engine, a: u32, b: u32) -> bool {
    x87_load32(e.mem.u32(b)) == x87_load32(e.mem.u32(a))
}

/// Whether two blocks of `count` entries of `floats` floats are equal (both null counts as
/// equal; one null as not), as `00a906e0` compares the optional arrays.
fn same_optional_floats(e: &Engine, mine: u32, theirs: u32, count: u32, floats: u32) -> bool {
    if (mine != 0) != (theirs != 0) {
        return false;
    }
    if mine == 0 {
        return true;
    }
    (0..count).all(|entry| {
        let base = entry * 4 * floats;
        (0..floats).all(|f| same_float(e, mine + base + 4 * f, theirs + base + 4 * f))
    })
}

// Translated from 00a906e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenPolygon::IsEqual` (Xbox PDB): `NiObject::IsEqual`, the same vertex count, the
/// same vertices (three floats each), texture coordinates (two floats, both absent or both
/// present) and colors (four floats, likewise) as x87 compares (a NaN is never equal), and
/// the same seven property pointers.
pub fn ni_screen_polygon_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let (a, b) = (this.addr(), other.addr());
    if !e.call(NI_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    let count = e.mem.u16(a + POLYGON_VERTEX_COUNT);
    if count != e.mem.u16(b + POLYGON_VERTEX_COUNT) {
        return false;
    }
    let count = u32::from(count);
    let (theirs, mine) = (
        e.mem.u32(b + POLYGON_VERTICES),
        e.mem.u32(a + POLYGON_VERTICES),
    );
    let vertices_equal = (0..count)
        .all(|v| (0..3).all(|f| same_float(e, mine + 0xc * v + 4 * f, theirs + 0xc * v + 4 * f)));
    if !vertices_equal {
        return false;
    }
    let (mine, theirs) = (
        e.mem.u32(a + POLYGON_TEXTURE),
        e.mem.u32(b + POLYGON_TEXTURE),
    );
    if !same_optional_floats(e, mine, theirs, count, 2) {
        return false;
    }
    let (mine, theirs) = (e.mem.u32(a + POLYGON_COLORS), e.mem.u32(b + POLYGON_COLORS));
    if !same_optional_floats(e, mine, theirs, count, 4) {
        return false;
    }
    let (theirs, mine) = (
        e.mem.u32(b + POLYGON_PROPERTIES),
        e.mem.u32(a + POLYGON_PROPERTIES),
    );
    (0..7).all(|i| e.mem.u32(mine + 4 * i) == e.mem.u32(theirs + 4 * i))
}

/// The inlined `SetAt` of the camera's two `NiTObjectArray<NiPointer<..>>` instances
/// (`00a908b0`, `00a90980`): the function-local null pointer static (`null`, initialized
/// once under its guard bit with its destructor given to `_atexit`) decides the count;
/// beyond the used size the size becomes `index + 1` (and the count goes up for a non-null
/// value); inside it the count goes up or down as a null entry is filled or emptied; then
/// the entry is replaced with the value (the old one released, the new one counted) unless
/// it is the same.
fn object_array_set_at(
    e: &mut Engine,
    array: u32,
    index: u32,
    value: Ptr,
    null: u32,
    guard: u32,
    destructor: u32,
) {
    if e.mem.u8(guard) & 1 == 0 {
        let word = e.mem.u32(guard);
        e.mem.set_u32(guard, word | 1);
        e.mem.set_u32(null, 0);
        e.call(ATEXIT, &args![destructor]);
    }
    let size = u32::from(e.mem.u16(array + 0xa));
    let new = e.mem.u32(value.addr());
    let empty = e.mem.u32(null);
    if index >= size {
        e.mem.set_u16(array + 0xa, index.wrapping_add(1) as u16);
        if new != empty {
            let count = e.mem.u16(array + 0xc);
            e.mem.set_u16(array + 0xc, count.wrapping_add(1));
        }
    } else {
        let base = e.mem.u32(array + 4);
        let entry = e.mem.u32(base.wrapping_add(4 * index));
        if new != empty {
            if entry == empty {
                let count = e.mem.u16(array + 0xc);
                e.mem.set_u16(array + 0xc, count.wrapping_add(1));
            }
        } else if entry != empty {
            let count = e.mem.u16(array + 0xc);
            e.mem.set_u16(array + 0xc, count.wrapping_add(0xffff));
        }
    }
    let slot = e.mem.u32(array + 4).wrapping_add(index.wrapping_mul(4));
    let old = e.mem.u32(slot);
    if old == e.mem.u32(value.addr()) {
        return;
    }
    if old != 0 {
        release(e, old);
    }
    let new = e.mem.u32(value.addr());
    e.mem.set_u32(slot, new);
    if new != 0 {
        acquire(e, new);
    }
}

// Translated from 00a908b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the camera's polygon array, (index, address of
/// the pointer)): `NiTObjectArray<NiPointer<NiScreenPolygon>>`'s `SetAt` with its null static
/// at `011f5e74` (see [`object_array_set_at`]).
pub fn fn_00a908b0(e: &mut Engine, this: Ptr, index: u32, value: Ptr) {
    object_array_set_at(
        e,
        this.addr(),
        index,
        value,
        POLYGON_NULL,
        POLYGON_NULL_GUARD,
        POLYGON_NULL_DESTRUCT,
    );
}

// Translated from 00a90980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the camera's texture array, (index, address of
/// the pointer)): `NiTObjectArray<NiPointer<NiScreenTexture>>`'s `SetAt` with its null static
/// at `011f5e7c` (see [`object_array_set_at`]).
pub fn fn_00a90980(e: &mut Engine, this: Ptr, index: u32, value: Ptr) {
    object_array_set_at(
        e,
        this.addr(),
        index,
        value,
        TEXTURE_NULL,
        TEXTURE_NULL_GUARD,
        TEXTURE_NULL_DESTRUCT,
    );
}

// Translated from 00a90a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; by its body
/// `NiScreenGeometryData`'s default constructor): `NiTriShapeData`'s constructor, pixel
/// accuracy and unordered flags cleared, no active vertices, the vtable, and the element
/// array (capacity 4, grow-by 4, empty, a 0x10-byte element block from `00aa1070`) with its
/// two vtables in turn. Answers `this`.
pub fn fn_00a90a50(e: &mut Engine, this: Ptr) -> Ptr {
    let data = this.addr();
    e.call(TRI_SHAPE_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u8(data + PIXEL_ACCURATE, 0);
    e.mem.set_u8(data + 0x59, 0);
    e.mem.set_u32(data + 0x5c, 0);
    e.mem.set_u32(data, NI_SCREEN_GEOMETRY_DATA_VTABLE);
    e.mem.set_u16(data + 0x68, 4);
    e.mem.set_u32(data + ELEMENTS, ELEMENT_ARRAY_VTABLE);
    e.mem.set_u16(data + 0x6e, 4);
    e.mem.set_u16(data + 0x6a, 0);
    e.mem.set_u16(data + 0x6c, 0);
    let block = e.call(ALLOCATE, &args![0x10u32]).u32();
    e.mem.set_u32(data + 0x64, block);
    e.mem
        .set_u32(data + ELEMENTS, ELEMENT_PRIMITIVE_ARRAY_VTABLE);
    this
}

// Translated from 00a90b20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiScreenGeometryData`): allocates 0x70 bytes (`00aa13e0`) and, when that
/// succeeds, jumps to the constructor (`00a90a50`). Answers the object, or null.
pub fn fn_00a90b20(e: &mut Engine) -> Ptr {
    let object = e.call(NI_NEW, &args![0x70u32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    fn_00a90a50(e, Ptr::new(object))
}

// Translated from 00a90b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometryData::SaveBinary` (Xbox PDB): `NiTriShapeData::SaveBinary`, the pixel
/// accuracy byte, the element used size (4 bytes), then per element its vertex count (2
/// bytes; 0 for a null element) and, when not 0: a byte for whether it has vertices and
/// the vertices (two floats each, `00a68ce0`), a byte for whether it has colors and the
/// colors (`NiColorA::SaveBinary`), the texture-set count (2 bytes) and all its texture
/// coordinates (`00a68ce0`).
pub fn ni_screen_geometry_data_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let data = this.addr();
    let s = stream.addr();
    e.call(TRI_SHAPE_DATA_SAVE_BINARY, &args![this, stream]);
    e.with_stack(0x10, |e, frame| {
        let (component, byte, count, vertices) = (
            frame.addr(),
            frame.addr() + 4,
            frame.addr() + 8,
            frame.addr() + 0xc,
        );
        let accurate = e.mem.u8(data + PIXEL_ACCURATE);
        e.mem.set_u8(byte, accurate);
        write(e, s, byte, 1, component, 1);
        let elements = u32::from(e.mem.u16(data + 0x6a));
        e.mem.set_u32(count, elements);
        write(e, s, count, 4, component, 4);
        for index in 0..elements {
            let base = e.mem.u32(data + 0x64);
            let element = e.mem.u32(base.wrapping_add(4 * index));
            let vertex_count = if element != 0 {
                u32::from(e.mem.u16(element + 4))
            } else {
                0
            };
            e.mem.set_u32(vertices, vertex_count);
            write(e, s, vertices, 2, component, 2);
            if vertex_count as u16 == 0 {
                continue;
            }
            let coordinates = e.mem.u32(element + 0x10);
            let sets = u32::from(e.mem.u16(element + 6));
            let colors = e.mem.u32(element + 0xc);
            let points = e.mem.u32(element + 8);
            e.mem.set_u8(byte, u8::from(points != 0));
            write(e, s, byte, 1, component, 1);
            if e.mem.u8(byte) != 0 {
                for v in 0..u32::from(vertex_count as u16) {
                    e.call(POINT2_SAVE_BINARY, &args![points + 8 * v, stream]);
                }
            }
            e.mem.set_u8(byte, u8::from(colors != 0));
            write(e, s, byte, 1, component, 1);
            if e.mem.u8(byte) != 0 {
                for v in 0..u32::from(vertex_count as u16) {
                    e.call(COLOR_A_SAVE_BINARY, &args![colors + 0x10 * v, stream]);
                }
            }
            e.mem.set_u32(byte, sets);
            write(e, s, byte, 2, component, 2);
            let sets = e.mem.u32(byte);
            if sets as u16 != 0 {
                let total = u32::from(sets as u16).wrapping_mul(u32::from(vertex_count as u16));
                for c in 0..total {
                    e.call(POINT2_SAVE_BINARY, &args![coordinates + 8 * c, stream]);
                }
            }
        }
    });
}

// Translated from 00a90d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenPolygon::LoadBinary` (Xbox PDB): `NiObject::LoadBinary`, the vertex count (2
/// bytes), the vertices (a block of 0xc bytes each from the memory manager, three floats
/// each through `NiColor::LoadBinary`); then whether there are texture coordinates and
/// colors: before file version 0x04010000 each is a 4-byte word read into its own field and
/// tested for not 0, from that version on a byte. Present texture coordinates get a block of
/// 8 bytes each (`00a68c80` per vertex), present colors a block of 0x10 bytes each with every
/// float zeroed (null when the allocation fails) and `NiColorA::LoadBinary` per vertex. Last,
/// the property links (`NiStream::ReadMultipleLinkIDs`).
pub fn ni_screen_polygon_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let polygon = this.addr();
    let s = stream.addr();
    e.call(NI_OBJECT_LOAD_BINARY, &args![this, stream]);
    e.with_stack(0xc, |e, frame| {
        let (component, texture_flag, color_flag) =
            (frame.addr(), frame.addr() + 4, frame.addr() + 8);
        read(e, s, polygon + POLYGON_VERTEX_COUNT, 2, component, 2);
        let count = |e: &Engine| u32::from(e.mem.u16(polygon + POLYGON_VERTEX_COUNT));
        let size = block_bytes(count(e), 0xc);
        let block = memory_allocate(e, size);
        e.mem.set_u32(polygon + POLYGON_VERTICES, block);
        let mut index = 0u32;
        while index < count(e) {
            let base = e.mem.u32(polygon + POLYGON_VERTICES);
            e.call(POINT3_LOAD_BINARY, &args![base + 0xc * index, stream]);
            index += 1;
        }
        let old_format = e.mem.u32(s + STREAM_FILE_VERSION) < 0x0401_0000;
        if old_format {
            read(e, s, polygon + POLYGON_TEXTURE, 4, component, 4);
            let present = e.mem.u32(polygon + POLYGON_TEXTURE) != 0;
            e.mem.set_u8(texture_flag, u8::from(present));
        } else {
            read(e, s, texture_flag, 1, component, 1);
        }
        if e.mem.u8(texture_flag) != 0 {
            let size = block_bytes(count(e), 8);
            let block = memory_allocate(e, size);
            e.mem.set_u32(polygon + POLYGON_TEXTURE, block);
            let mut index = 0u32;
            while index < count(e) {
                let base = e.mem.u32(polygon + POLYGON_TEXTURE);
                e.call(POINT2_LOAD_BINARY, &args![base + 8 * index, stream]);
                index += 1;
            }
        }
        let old_format = e.mem.u32(s + STREAM_FILE_VERSION) < 0x0401_0000;
        if old_format {
            read(e, s, polygon + POLYGON_COLORS, 4, component, 4);
            let present = e.mem.u32(polygon + POLYGON_COLORS) != 0;
            e.mem.set_u8(color_flag, u8::from(present));
        } else {
            read(e, s, color_flag, 1, component, 1);
        }
        if e.mem.u8(color_flag) != 0 {
            let entries = count(e);
            let block = memory_allocate(e, block_bytes(entries, 0x10));
            if block != 0 {
                let zero = x87_store32(0.0);
                let mut remaining = (entries as i32).wrapping_sub(1);
                let mut at = block;
                while remaining >= 0 {
                    for word in 0..4 {
                        e.mem.set_u32(at + 4 * word, zero);
                    }
                    at = at.wrapping_add(0x10);
                    remaining = remaining.wrapping_sub(1);
                }
            }
            e.mem.set_u32(polygon + POLYGON_COLORS, block);
            let mut index = 0u32;
            while index < count(e) {
                let base = e.mem.u32(polygon + POLYGON_COLORS);
                e.call(COLOR_A_LOAD_BINARY, &args![base + 0x10 * index, stream]);
                index += 1;
            }
        }
    });
    e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
}

// Translated from 00a90f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenPolygon::SaveBinary` (Xbox PDB): the empty base `00a89af0`, the vertex count (2
/// bytes), the vertices (`NiColor::SaveBinary`), a byte for whether there are texture
/// coordinates and those (`00a68ce0`), a byte for whether there are colors and those
/// (`NiColorA::SaveBinary`), then the property count 7 (4 bytes) and the seven property
/// links (`NiStream::SaveLinkID`, vtable +0x30).
pub fn ni_screen_polygon_save_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let polygon = this.addr();
    let s = stream.addr();
    e.call(EMPTY_BASE, &args![this, stream]);
    e.with_stack(0xc, |e, frame| {
        let (component, byte, properties) = (frame.addr(), frame.addr() + 4, frame.addr() + 8);
        write(e, s, polygon + POLYGON_VERTEX_COUNT, 2, component, 2);
        let count = |e: &Engine| u32::from(e.mem.u16(polygon + POLYGON_VERTEX_COUNT));
        let mut index = 0u32;
        while index < count(e) {
            let base = e.mem.u32(polygon + POLYGON_VERTICES);
            e.call(POINT3_SAVE_BINARY, &args![base + 0xc * index, stream]);
            index += 1;
        }
        for (field, stride, save) in [
            (POLYGON_TEXTURE, 8u32, POINT2_SAVE_BINARY),
            (POLYGON_COLORS, 0x10, COLOR_A_SAVE_BINARY),
        ] {
            let present = e.mem.u32(polygon + field) != 0;
            e.mem.set_u8(byte, u8::from(present));
            write(e, s, byte, 1, component, 1);
            if e.mem.u8(byte) != 0 {
                let mut index = 0u32;
                while index < count(e) {
                    let base = e.mem.u32(polygon + field);
                    e.call(save, &args![base + stride * index, stream]);
                    index += 1;
                }
            }
        }
        e.mem.set_u32(properties, 7);
        write(e, s, properties, 4, component, 4);
        let mut index = 0u32;
        while index < e.mem.u32(properties) {
            let state = e.mem.u32(polygon + POLYGON_PROPERTIES);
            let property = e.mem.u32(state.wrapping_add(4 * index));
            e.vcall(s, STREAM_SAVE_LINK_ID_SLOT, &args![property]);
            index = (index + 1) & 0xffff;
        }
    });
}

// Translated from 00a910f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the scalar deleting destructor of `NiTArray<NiPointer<
/// NiScreenTexture>, NiTNewInterface<..>>` by the vtable it sets, `0109f9bc`): the vtable,
/// the element block deleted (`00a47600`), and for bit 0 of `flags` the array given back to
/// the memory manager. Answers `this`.
pub fn fn_00a910f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let block = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(this.addr(), TEXTURE_ARRAY_VTABLE);
    e.call(POINTER_ARRAY_DELETE, &args![block]);
    if flags & 1 != 0 {
        memory_deallocate(e, this.addr());
    }
    this
}

// Translated from 00a91120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the screen element pool `00a91290` builds, the
/// number of elements): adds a chunk of `count` elements. The chunk header (0xc bytes from
/// the memory manager) holds the element block (`00a90290`, null for no elements) and
/// `count`; each element (null past the header's count) is pushed on the pool's free array
/// (+0, capacity +4, used +8), grown first (`00a311e0`, to twice its capacity, or 1) when
/// full; then the header is linked in front of the pool's chunk list (+0xc).
pub fn fn_00a91120(e: &mut Engine, this: Ptr, count: u32) {
    let pool = this.addr();
    let header = memory_allocate(e, 0xc);
    if header != 0 {
        e.mem.set_u32(header + 4, count);
        e.mem.set_u32(header + 8, 0);
        let block = if count > 0 {
            fn_00a90290(e, count).addr()
        } else {
            0
        };
        e.mem.set_u32(header, block);
    }
    for index in 0..count {
        let element = if index < e.mem.u32(header + 4) {
            e.mem
                .u32(header)
                .wrapping_add(index.wrapping_mul(ELEMENT_SIZE))
        } else {
            0
        };
        let capacity = e.mem.u32(pool + 4);
        if e.mem.u32(pool + 8) == capacity {
            let grown = if capacity > 0 {
                capacity.wrapping_add(capacity)
            } else {
                1
            };
            e.call(FREE_LIST_REALLOC, &args![pool, grown]);
        }
        let used = e.mem.u32(pool + 8);
        let free = e.mem.u32(pool);
        e.mem
            .set_u32(free.wrapping_add(used.wrapping_mul(4)), element);
        let used = e.mem.u32(pool + 8).wrapping_add(1);
        e.mem.set_u32(pool + 8, used);
    }
    let chunks = e.mem.u32(pool + 0xc);
    e.mem.set_u32(header.wrapping_add(8), chunks);
    e.mem.set_u32(pool + 0xc, header);
}

// Translated from 00a911d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometryData::~NiScreenGeometryData` (Xbox PDB): the vtable; every element of
/// the element array is taken out (the count goes down for a non-null entry, the entry
/// cleared), its vertex, color and texture blocks given back to the memory manager and the
/// element returned to the pool (`00e7ec00` on `011f5e50`); then the array is emptied
/// (`00a90180` with 0), its vtable set back to `0109f9a4` and its block freed, and
/// `NiTriShapeData`'s destructor runs.
pub fn ni_screen_geometry_data_destruct(e: &mut Engine, this: Ptr) {
    let data = this.addr();
    e.mem.set_u32(data, NI_SCREEN_GEOMETRY_DATA_VTABLE);
    e.with_stack(4, |e, local| {
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(data + 0x6a)) {
            let size = u32::from(e.mem.u16(data + 0x6a));
            let base = e.mem.u32(data + 0x64);
            let element = e.mem.u32(base.wrapping_add(4 * index));
            if index >= size {
                e.mem.set_u16(data + 0x6a, index.wrapping_add(1) as u16);
            } else if e.mem.u32(base.wrapping_add(4 * index)) != 0 {
                let count = e.mem.u16(data + 0x6c);
                e.mem.set_u16(data + 0x6c, count.wrapping_add(0xffff));
            }
            let base = e.mem.u32(data + 0x64);
            e.mem.set_u32(base.wrapping_add(4 * index), 0);
            if element != 0 {
                for field in [8u32, 0xc, 0x10] {
                    let block = e.mem.u32(element + field);
                    memory_deallocate(e, block);
                }
                let pool = e.mem.u32(SCREEN_SPACE_OBJECT_SLOT);
                e.mem.set_u32(local.addr(), element);
                e.call(POOL_RELEASE, &args![pool, local]);
            }
            index += 1;
        }
    });
    fn_00a90180(e, Ptr::new(data + ELEMENTS), 0);
    let block = e.mem.u32(data + ELEMENTS + 4);
    e.mem.set_u32(data + ELEMENTS, ELEMENT_ARRAY_VTABLE);
    e.call(FREE, &args![block]);
    e.call(TRI_SHAPE_DATA_DESTRUCT, &args![this]);
}

// Translated from 00a912d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometryData::GetViewerStrings` (Xbox PDB): after
/// `NiGeometryData::GetViewerStrings`, adds the class name, `"m_bPixelAccurate"` and
/// `"Num Elements"` (the element count, +0x6c).
pub fn ni_screen_geometry_data_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let data = this.addr();
    let list = strings.addr();
    e.call(GEOMETRY_DATA_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let name = e.mem.u32(NI_SCREEN_GEOMETRY_DATA_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![name]).u32();
        add_text(e, list, slot.addr(), text);
        let accurate = u32::from(e.mem.u8(data + PIXEL_ACCURATE));
        let text = e
            .call(VIEWER_STRING_BOOL, &args![TEXT_PIXEL_ACCURATE, accurate])
            .u32();
        add_text(e, list, slot.addr(), text);
        let count = u32::from(e.mem.u16(data + 0x6c));
        let text = e
            .call(VIEWER_STRING_UNSIGNED, &args![TEXT_NUM_ELEMENTS, count])
            .u32();
        add_text(e, list, slot.addr(), text);
    });
}

// Translated from 00a913a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometry::GetViewerStrings` (Xbox PDB): after `NiTriShape::GetViewerStrings`,
/// adds the class name.
pub fn ni_screen_geometry_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(TRI_SHAPE_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let name = e.mem.u32(NI_SCREEN_GEOMETRY_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![name]).u32();
        add_text(e, strings.addr(), slot.addr(), text);
    });
}

// Translated from 00a913f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenPolygon::GetViewerStrings` (Xbox PDB): after `NiObject::GetViewerStrings`,
/// adds the class name, `"m_usVertices"`, `"m_pkVertex"`, `"m_pkTexture"`, `"m_pkColor"`,
/// then for each of the seven properties a string with its class name (`GetRTTI`) as the
/// label and its address, stored by an inlined `SetAt`: at the used size (grown first when
/// full), which grows the used size by one (and the count for a non-null string).
pub fn ni_screen_polygon_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    let polygon = this.addr();
    let list = strings.addr();
    e.call(NI_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let name = e.mem.u32(NI_SCREEN_POLYGON_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![name]).u32();
        add_text(e, list, slot.addr(), text);
        let count = u32::from(e.mem.u16(polygon + POLYGON_VERTEX_COUNT));
        let text = e
            .call(VIEWER_STRING_SHORT, &args![TEXT_VERTICES, count])
            .u32();
        add_text(e, list, slot.addr(), text);
        for (label, field) in [
            (TEXT_VERTEX, POLYGON_VERTICES),
            (TEXT_TEXTURE, POLYGON_TEXTURE),
            (TEXT_COLOR, POLYGON_COLORS),
        ] {
            let value = e.mem.u32(polygon + field);
            let text = e.call(VIEWER_STRING_POINTER, &args![label, value]).u32();
            add_text(e, list, slot.addr(), text);
        }
    });
    for offset in (0..0x1c).step_by(4) {
        let state = e.mem.u32(polygon + POLYGON_PROPERTIES);
        let property = e.mem.u32(state.wrapping_add(offset));
        let rtti = e.vcall(property, SLOT_GET_RTTI, &args![]).u32();
        let label = e.mem.u32(rtti);
        let text = e.call(VIEWER_STRING_POINTER, &args![label, property]).u32();
        let index = u32::from(e.mem.u16(list + 0xa));
        if index >= u32::from(e.mem.u16(list + 8)) {
            let grow = u32::from(e.mem.u16(list + 0xe));
            e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index]);
        }
        let size = u32::from(e.mem.u16(list + 0xa));
        if index >= size {
            e.mem.set_u16(list + 0xa, index.wrapping_add(1) as u16);
            if text != 0 {
                let count = e.mem.u16(list + 0xc);
                e.mem.set_u16(list + 0xc, count.wrapping_add(1));
            }
        } else {
            let base = e.mem.u32(list + 4);
            let entry = e.mem.u32(base.wrapping_add(4 * index));
            if text != 0 {
                if entry == 0 {
                    let count = e.mem.u16(list + 0xc);
                    e.mem.set_u16(list + 0xc, count.wrapping_add(1));
                }
            } else if entry != 0 {
                let count = e.mem.u16(list + 0xc);
                e.mem.set_u16(list + 0xc, count.wrapping_add(0xffff));
            }
        }
        let base = e.mem.u32(list + 4);
        e.mem.set_u32(base.wrapping_add(4 * index), text);
    }
}

// Translated from 00a915d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; by its body the destructor of
/// `NiScreenSpaceCamera`): the vtable; every polygon entry set to null (`00a908b0`), the
/// polygon array's `00a2e620`; the same for the textures (`00a90980`); then both arrays
/// get their base vtables and have their blocks deleted (`00a47600`), textures first, and
/// `NiCamera`'s destructor body (`00a6fae0`) runs.
pub fn fn_00a915d0(e: &mut Engine, this: Ptr) {
    let camera = this.addr();
    e.mem.set_u32(camera, NI_SCREEN_SPACE_CAMERA_VTABLE);
    e.with_stack(4, |e, null| {
        let polygons = camera + CAMERA_POLYGONS;
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(polygons + 0xa)) {
            fn_00a908b0(e, Ptr::new(polygons), index, null);
            index += 1;
        }
        e.call(POINTER_ARRAY_00A2E620, &args![polygons]);
        let textures = camera + CAMERA_TEXTURES;
        let mut index = 0u32;
        while index < u32::from(e.mem.u16(textures + 0xa)) {
            fn_00a90980(e, Ptr::new(textures), index, null);
            index += 1;
        }
        e.call(POINTER_ARRAY_00A2E620, &args![textures]);
    });
    let textures = camera + CAMERA_TEXTURES;
    let block = e.mem.u32(textures + 4);
    e.mem.set_u32(textures, TEXTURE_ARRAY_VTABLE);
    e.call(POINTER_ARRAY_DELETE, &args![block]);
    let polygons = camera + CAMERA_POLYGONS;
    let block = e.mem.u32(polygons + 4);
    e.mem.set_u32(polygons, POLYGON_ARRAY_VTABLE);
    e.call(POINTER_ARRAY_DELETE, &args![block]);
    e.call(CAMERA_DESTRUCT, &args![this]);
}

/// One of the camera's pointer arrays copied by `00a916a0`: `this`'s array at `offset`
/// into the clone's, the clone's resized (`00a47770`) to this one's used size and given its
/// grow-by, then each entry cloned (`CreateClone`, vtable +0x48, with the cloning process;
/// counted while it is in use) and stored by `set_at`, or null for a null entry.
fn copy_pointer_array(
    e: &mut Engine,
    this: u32,
    clone: u32,
    cloning: Ptr,
    offset: u32,
    set_at: SetAt,
    temporary: Ptr,
) {
    let (source, target) = (this + offset, clone.wrapping_add(offset));
    let size = u32::from(e.mem.u16(source + 0xa));
    e.call(POINTER_ARRAY_SET_SIZE, &args![target, size]);
    let grow = e.mem.u16(source + 0xe);
    e.mem.set_u16(target + 0xe, grow);
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(source + 0xa)) {
        let base = e.mem.u32(source + 4);
        let original = e.mem.u32(base.wrapping_add(4 * index));
        if original == 0 {
            e.mem.set_u32(temporary.addr(), 0);
            set_at(e, Ptr::new(target), index, temporary);
        } else {
            acquire(e, original);
            let copy = e.vcall(original, SLOT_CREATE_CLONE, &args![cloning]).u32();
            e.mem.set_u32(temporary.addr(), copy);
            if copy != 0 {
                acquire(e, copy);
            }
            set_at(e, Ptr::new(target), index, temporary);
            if copy != 0 {
                release(e, copy);
            }
            release(e, original);
        }
        index += 1;
    }
}

// Translated from 00a916a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, (clone, cloning process); by its body
/// `NiScreenSpaceCamera`'s member copy): `NiCamera`'s member copy (`00a70f60`), then the
/// polygon and texture arrays copied into the clone with every entry cloned (see
/// [`copy_pointer_array`]).
pub fn fn_00a916a0(e: &mut Engine, this: Ptr, clone: Ptr, cloning: Ptr) {
    e.call(CAMERA_COPY_MEMBERS, &args![this, clone, cloning]);
    e.with_stack(4, |e, temporary| {
        copy_pointer_array(
            e,
            this.addr(),
            clone.addr(),
            cloning,
            CAMERA_POLYGONS,
            fn_00a908b0,
            temporary,
        );
        copy_pointer_array(
            e,
            this.addr(),
            clone.addr(),
            cloning,
            CAMERA_TEXTURES,
            fn_00a90980,
            temporary,
        );
    });
}

// Translated from 00a91890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenSpaceCamera::GetViewerStrings` (Xbox PDB): after `NiCamera::GetViewerStrings`,
/// adds the class name.
pub fn ni_screen_space_camera_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr) {
    e.call(CAMERA_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let name = e.mem.u32(NI_SCREEN_SPACE_CAMERA_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![name]).u32();
        add_text(e, strings.addr(), slot.addr(), text);
    });
}

// Translated from 00a918e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (the scalar deleting destructor of `NiTArray<NiPointer<
/// NiScreenPolygon>, NiTNewInterface<..>>` by the vtable it sets, `0109f9b4`): the vtable,
/// the element block deleted (`00a47600`), and for bit 0 of `flags` the array given back to
/// the memory manager. Answers `this`.
pub fn fn_00a918e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let block = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(this.addr(), POLYGON_ARRAY_VTABLE);
    e.call(POINTER_ARRAY_DELETE, &args![block]);
    if flags & 1 != 0 {
        memory_deallocate(e, this.addr());
    }
    this
}

// Translated from 00a91910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometryData::_scalar_deleting_destructor_` (Xbox PDB): the destructor
/// (`00a911d0`), then the sized delete of 0x70 bytes when bit 0 of `flags` is set. Answers
/// `this`.
pub fn ni_screen_geometry_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    ni_screen_geometry_data_destruct(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x70u32]);
    }
    this
}

// Translated from 00a91950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenGeometryData::LoadBinary` (Xbox PDB): before file version 0x0a00010f
/// `NiGeometryData::LoadBinary`, the pixel accuracy byte, the vertex count (+8) and
/// triangle count (+0x40, 2 bytes each) and the triangle list (its 2-byte length into +0x44,
/// then that many 16-bit indices into a block from `00aa1070` at +0x48); from that version
/// on `NiTriShapeData::LoadBinary` and the pixel accuracy byte only. Then the element count
/// (4 bytes): the element array is sized to it (`00a90180`) and each element read: its
/// vertex count (2 bytes; 0 skips the element), a byte and the vertices (8 bytes each), a
/// byte and the colors (0x10 bytes each, constructed by `NiColorA`'s constructor), the
/// texture-set count and the texture coordinates (8 bytes each); the blocks come from the
/// memory manager. An element is taken from the pool at `011f5e50` (grown by its block size,
/// `00a91120`, which then doubles, when empty), its old blocks given back, the new ones and
/// counts stored, and it goes into the array at its index (`00ba3aa0`).
pub fn ni_screen_geometry_data_load_binary(e: &mut Engine, this: Ptr, stream: Ptr) {
    let data = this.addr();
    let s = stream.addr();
    let old_format = |e: &Engine| e.mem.u32(s + STREAM_FILE_VERSION) < 0x0a00_010f;
    if old_format(e) {
        e.call(GEOMETRY_DATA_LOAD_BINARY, &args![this, stream]);
    } else {
        e.call(TRI_SHAPE_DATA_LOAD_BINARY, &args![this, stream]);
    }
    e.with_stack(0x20, |e, frame| {
        let f = frame.addr();
        let (component, byte, word, count) = (f, f + 4, f + 8, f + 0xc);
        let (vertices, sets, element_slot) = (f + 0x10, f + 0x14, f + 0x18);
        read(e, s, byte, 1, component, 1);
        let accurate = e.mem.u8(byte) != 0;
        e.mem.set_u8(data + PIXEL_ACCURATE, u8::from(accurate));
        if old_format(e) {
            read(e, s, word, 2, component, 2);
            let vertex_count = e.mem.u16(word);
            e.mem.set_u16(data + 8, vertex_count);
            read(e, s, data + 0x40, 2, component, 2);
        }
        if old_format(e) {
            read(e, s, word, 2, component, 2);
            let length = e.mem.u16(word);
            e.mem.set_u32(data + 0x44, u32::from(length));
            if length > 0 {
                let list = e.call(ALLOCATE, &args![u32::from(length) * 2]).u32();
                let bytes = u32::from(e.mem.u16(word)) * 2;
                e.mem.set_u32(data + 0x48, list);
                read(e, s, list, bytes, component, 2);
            }
        }
        e.mem.set_u32(count, 0);
        read(e, s, count, 4, component, 4);
        let elements = e.mem.u32(count);
        if elements == 0 {
            return;
        }
        let array = data + ELEMENTS;
        fn_00a90180(e, Ptr::new(array), elements);
        for index in 0..elements {
            e.mem.set_u32(vertices, 0);
            read(e, s, vertices, 2, component, 2);
            let vertex_count = u32::from(e.mem.u16(vertices));
            if vertex_count == 0 {
                continue;
            }
            e.mem.set_u32(sets, 0);
            let mut points = 0u32;
            let mut colors = 0u32;
            read(e, s, byte, 1, component, 1);
            if e.mem.u8(byte) != 0 {
                points = memory_allocate(e, block_bytes(vertex_count, 8));
                read(e, s, points, vertex_count.wrapping_mul(8), component, 8);
            }
            read(e, s, byte, 1, component, 1);
            if e.mem.u8(byte) != 0 {
                let block = memory_allocate(e, block_bytes(vertex_count, 0x10));
                if block != 0 {
                    e.call(
                        VECTOR_CONSTRUCT,
                        &args![block, 0x10u32, vertex_count, COLOR_CONSTRUCT],
                    );
                }
                colors = block;
                read(e, s, colors, vertex_count << 4, component, 0x10);
            }
            read(e, s, sets, 2, component, 2);
            let set_count = e.mem.u32(sets);
            let mut coordinates = 0u32;
            if set_count as u16 != 0 {
                let total = u32::from(set_count as u16).wrapping_mul(vertex_count);
                coordinates = memory_allocate(e, block_bytes(total, 8));
                read(e, s, coordinates, total.wrapping_mul(8), component, 8);
            }
            // Take an element from the pool.
            let pool = e.mem.u32(SCREEN_SPACE_OBJECT_SLOT);
            if e.mem.u32(pool + 8) == 0 {
                let chunk = e.mem.u32(pool + 0x10);
                fn_00a91120(e, Ptr::new(pool), chunk);
                let doubled = e.mem.u32(pool + 0x10).wrapping_add(e.mem.u32(pool + 0x10));
                e.mem.set_u32(pool + 0x10, doubled);
            }
            let free = e.mem.u32(pool);
            let element = e.mem.u32(free);
            let used = e.mem.u32(pool + 8).wrapping_sub(1);
            e.mem.set_u32(pool + 8, used);
            let last = e.mem.u32(free.wrapping_add(used.wrapping_mul(4)));
            e.mem.set_u32(free, last);
            let old = e.mem.u32(element + 8);
            e.mem.set_u32(element_slot, element);
            memory_deallocate(e, old);
            e.mem.set_u16(element + 4, vertex_count as u16);
            e.mem.set_u32(element + 8, points);
            let old = e.mem.u32(element + 0xc);
            memory_deallocate(e, old);
            e.mem.set_u32(element + 0xc, colors);
            let old = e.mem.u32(element + 0x10);
            memory_deallocate(e, old);
            e.mem.set_u32(element + 0x10, coordinates);
            e.mem.set_u16(element + 6, set_count as u16);
            e.call(ELEMENT_ARRAY_SET_AT, &args![array, index, element_slot]);
        }
    });
}

// Translated from 00a91d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenSpaceCamera::NiScreenSpaceCamera` (Xbox PDB; `thiscall`, no stack words):
/// `NiCamera`'s constructor, the vtable, the polygon and texture arrays (capacity 5, grow-by
/// 5, empty, a block of 5 pointers from `00c1d790`, base then object-array vtable each), the
/// byte at +0xf4 set to 1, then `UpdateWorldData` with an update data of time 0.0 and all
/// flags clear and `UpdateWorldBound`. Answers `this`.
pub fn ni_screen_space_camera_ni_screen_space_camera(e: &mut Engine, this: Ptr) -> Ptr {
    let camera = this.addr();
    e.call(CAMERA_CONSTRUCT, &args![this]);
    e.mem.set_u32(camera, NI_SCREEN_SPACE_CAMERA_VTABLE);
    for (array, base_vtable, vtable) in [
        (
            CAMERA_POLYGONS,
            POLYGON_ARRAY_VTABLE,
            POLYGON_OBJECT_ARRAY_VTABLE,
        ),
        (
            CAMERA_TEXTURES,
            TEXTURE_ARRAY_VTABLE,
            TEXTURE_OBJECT_ARRAY_VTABLE,
        ),
    ] {
        let at = camera + array;
        e.mem.set_u16(at + 8, 5);
        e.mem.set_u32(at, base_vtable);
        e.mem.set_u16(at + 0xe, 5);
        e.mem.set_u16(at + 0xa, 0);
        e.mem.set_u16(at + 0xc, 0);
        let block = e.call(POINTER_ARRAY_NEW, &args![5u32]).u32();
        e.mem.set_u32(at + 4, block);
        e.mem.set_u32(at, vtable);
    }
    // `NiUpdateData` (Xbox PDB, 0xc bytes): `fTime` 0.0 and the five flags clear (the three
    // padding bytes are not written; here they are 0).
    e.with_stack(0xc, |e, update| {
        e.mem.set_u32(update.addr(), x87_store32(0.0));
        e.mem.set_u8(camera + 0xf4, 1);
        for flag in 4..9 {
            e.mem.set_u8(update.addr() + flag, 0);
        }
        e.call(CAMERA_UPDATE_WORLD_DATA, &args![this, update]);
    });
    e.call(CAMERA_UPDATE_WORLD_BOUND, &args![this]);
    this
}

// Translated from 00a91e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenSpaceCamera::_scalar_deleting_destructor_` (Xbox PDB): the destructor
/// (`00a915d0`), then the sized delete of 0x134 bytes when bit 0 of `flags` is set. Answers
/// `this`.
pub fn ni_screen_space_camera_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00a915d0(e, this);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x134u32]);
    }
    this
}

// Translated from 00a91e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenSpaceCamera::CreateClone` (Xbox PDB): 0x134 bytes
/// (`00aa13e0`), constructed (`00a91d80`), then the member copy (`00a916a0`) into it; when
/// the allocation failed the copy still runs with a null clone. Answers the clone.
pub fn ni_screen_space_camera_create_clone(e: &mut Engine, this: Ptr, cloning: Ptr) -> Ptr {
    let block = e.call(NI_NEW, &args![0x134u32]).u32();
    let clone = if block != 0 {
        ni_screen_space_camera_ni_screen_space_camera(e, Ptr::new(block))
    } else {
        Ptr::NULL
    };
    fn_00a916a0(e, this, clone, cloning);
    clone
}

// Translated from 00a91ee0 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; by its body the creation
/// function of `NiScreenSpaceCamera`): allocates 0x134 bytes (`00aa13e0`) and, when that
/// succeeds, jumps to the constructor (`00a91d80`). Answers the object, or null.
pub fn fn_00a91ee0(e: &mut Engine) -> Ptr {
    let object = e.call(NI_NEW, &args![0x134u32]).u32();
    if object == 0 {
        return Ptr::NULL;
    }
    ni_screen_space_camera_ni_screen_space_camera(e, Ptr::new(object))
}

// Translated from 00a91f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenSpaceCamera::LinkObject` (Xbox PDB): `NiCamera::LinkObject`, then for each of
/// the polygon array's entries (its used size read once) the entry's address is taken, the
/// array grown by its grow-by count (`00a47770`) when the used size has reached the capacity,
/// and the pointer at that address stored at the used size (`00a908b0`; the address is the
/// one taken before growing); then `00a2e620`. The same for the textures (`00a90980`).
pub fn ni_screen_space_camera_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    let camera = this.addr();
    e.call(CAMERA_LINK_OBJECT, &args![this, stream]);
    let arrays: [(u32, SetAt); 2] = [
        (CAMERA_POLYGONS, fn_00a908b0),
        (CAMERA_TEXTURES, fn_00a90980),
    ];
    for (offset, set_at) in arrays {
        let array = camera + offset;
        let count = u32::from(e.mem.u16(array + 0xa));
        for index in 0..count {
            let size = u32::from(e.mem.u16(array + 0xa));
            let capacity = u32::from(e.mem.u16(array + 8));
            let entry = e.mem.u32(array + 4).wrapping_add(4 * index);
            if size >= capacity {
                let grow = u32::from(e.mem.u16(array + 0xe));
                e.call(POINTER_ARRAY_SET_SIZE, &args![array, grow + size]);
            }
            set_at(e, Ptr::new(array), size, Ptr::new(entry));
        }
        e.call(POINTER_ARRAY_00A2E620, &args![array]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a8ff70, ni_screen_geometry_get_rtti() -> Ptr),
        entry!(0x00a8ffb0, ni_screen_polygon_get_rtti() -> Ptr),
        entry!(0x00a90ab0, ni_screen_geometry_data_get_rtti() -> Ptr),
        entry!(0x00a91690, ni_screen_space_camera_get_rtti() -> Ptr),
        entry!(0x00a8_77d0, ni_screen_geometry_load_binary(Ptr, Ptr)),
        entry!(0x00a8_77e0, ni_screen_geometry_save_binary(Ptr, Ptr)),
        entry!(0x00a9_0000, ni_screen_space_camera_load_binary(Ptr, Ptr)),
        entry!(0x00a9_0020, ni_screen_space_camera_save_binary(Ptr, Ptr)),
        entry!(0x00a9_0030, ni_screen_space_camera_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_0180, fn_00a90180(Ptr<NiTArray>, u32)),
        entry!(0x00a9_1290, fn_00a91290()),
        entry!(0x00a9_0ac0, fn_00a90ac0(Ptr)),
        entry!(0x00a9_1930, fn_00a91930()),
        entry!(0x00a8_ff80, fn_00a8ff80() -> Ptr),
        entry!(0x00a8_ffc0, fn_00a8ffc0() -> Ptr),
        entry!(
            0x00a9_0040,
            ni_screen_geometry_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a9_0070, ni_screen_geometry_create_clone(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a9_00c0,
            ni_screen_polygon_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a9_0100, ni_screen_space_camera_process_clone(Ptr, Ptr)),
        entry!(0x00a9_0260, fn_00a90260(Ptr, u32) -> Ptr),
        entry!(0x00a9_0290, fn_00a90290(u32) -> Ptr),
        entry!(
            0x00a9_02e0,
            screen_element_vector_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a9_03a0, ni_screen_geometry_data_is_equal(Ptr, Ptr) -> bool),
        entry!(
            0x00a9_0560,
            ni_screen_polygon_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a9_05c0, ni_screen_polygon_create_clone(Ptr, Ptr) -> Ptr),
        entry!(0x00a9_0640, ni_screen_polygon_link_object(Ptr, Ptr)),
        entry!(0x00a9_06e0, ni_screen_polygon_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_08b0, fn_00a908b0(Ptr, u32, Ptr)),
        entry!(0x00a9_0980, fn_00a90980(Ptr, u32, Ptr)),
        entry!(0x00a9_0a50, fn_00a90a50(Ptr) -> Ptr),
        entry!(0x00a9_0b20, fn_00a90b20() -> Ptr),
        entry!(0x00a9_0b40, ni_screen_geometry_data_save_binary(Ptr, Ptr)),
        entry!(0x00a9_0d70, ni_screen_polygon_load_binary(Ptr, Ptr)),
        entry!(0x00a9_0f70, ni_screen_polygon_save_binary(Ptr, Ptr)),
        entry!(0x00a9_10f0, fn_00a910f0(Ptr, u32) -> Ptr),
        entry!(0x00a9_1120, fn_00a91120(Ptr, u32)),
        entry!(0x00a9_11d0, ni_screen_geometry_data_destruct(Ptr)),
        entry!(
            0x00a9_12d0,
            ni_screen_geometry_data_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a9_13a0, ni_screen_geometry_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00a9_13f0, ni_screen_polygon_get_viewer_strings(Ptr, Ptr)),
        entry!(0x00a9_15d0, fn_00a915d0(Ptr)),
        entry!(0x00a9_16a0, fn_00a916a0(Ptr, Ptr, Ptr)),
        entry!(
            0x00a9_1890,
            ni_screen_space_camera_get_viewer_strings(Ptr, Ptr)
        ),
        entry!(0x00a9_18e0, fn_00a918e0(Ptr, u32) -> Ptr),
        entry!(
            0x00a9_1910,
            ni_screen_geometry_data_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a9_1950, ni_screen_geometry_data_load_binary(Ptr, Ptr)),
        entry!(
            0x00a9_1d80,
            ni_screen_space_camera_ni_screen_space_camera(Ptr) -> Ptr
        ),
        entry!(
            0x00a9_1e60,
            ni_screen_space_camera_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00a9_1e90,
            ni_screen_space_camera_create_clone(Ptr, Ptr) -> Ptr
        ),
        entry!(0x00a9_1ee0, fn_00a91ee0() -> Ptr),
        entry!(0x00a9_1f00, ni_screen_space_camera_link_object(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_screen_geometry_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8ff70, &args![]).u32(), NI_SCREEN_GEOMETRY_RTTI);
    }

    #[test]
    fn ni_screen_polygon_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a8ffb0, &args![]).u32(), NI_SCREEN_POLYGON_RTTI);
    }

    #[test]
    fn ni_screen_geometry_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a90ab0, &args![]).u32(),
            NI_SCREEN_GEOMETRY_DATA_RTTI
        );
    }

    #[test]
    fn ni_screen_space_camera_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(
            e.call(0x00a91690, &args![]).u32(),
            NI_SCREEN_SPACE_CAMERA_RTTI
        );
    }

    #[test]
    fn thunk_00a877d0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a8_77d0;
        const TARGET: u32 = 0x00a9_8ea0;
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
    fn thunk_00a877e0_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a8_77e0;
        const TARGET: u32 = 0x00a7_1d10;
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
    fn thunk_00a90000_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_0000;
        const TARGET: u32 = 0x00a7_1450;
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
    fn thunk_00a90020_forwards_its_words_to_the_jump_target() {
        const THUNK: u32 = 0x00a9_0020;
        const TARGET: u32 = 0x00a7_16c0;
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
    fn thunk_00a90030_forwards_its_words_and_returns_the_target_result() {
        const THUNK: u32 = 0x00a9_0030;
        const TARGET: u32 = 0x00a6_ff20;
        let mut e = Engine::new();
        e.register(TARGET, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log.iter().filter(|call| call.0 == TARGET).collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, args![0x1000u32, 0x2000u32]);
        e.register(TARGET, |_, _| Ret::default());
        assert_eq!(e.call(THUNK, &args![0x1000u32, 0x2000u32]).u32(), 0);
    }

    /// An array with room for `max` pointers holding `items`.
    fn array_fixture(max: u16, items: &[u32]) -> (Engine, Ptr<NiTArray>, u32) {
        let mut e = Engine::new();
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FREE, |_, _| Ret::default());
        let array = e.new_object::<NiTArray>();
        let base = e.mem.alloc(max as u32 * 4 + 4);
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(base + i as u32 * 4, *item);
        }
        e.set(array, NiTArray::m_pBase, base);
        e.set(array, NiTArray::m_usMaxSize, max);
        e.set(array, NiTArray::m_usSize, items.len() as u16);
        let present = items.iter().filter(|item| **item != 0).count();
        e.set(array, NiTArray::m_usESize, present as u16);
        (e, array, base)
    }

    #[test]
    fn fn_00a90180_does_nothing_for_the_current_capacity() {
        let (mut e, array, base) = array_fixture(4, &[0x10]);
        e.call_log = Some(vec![]);
        e.call(0x00a9_0180, &args![array, 4u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 1);
        assert_eq!(e.get(array, NiTArray::m_pBase), base);
    }

    #[test]
    fn fn_00a90180_grows_copies_and_zeroes_the_rest() {
        let (mut e, array, base) = array_fixture(3, &[0x10, 0, 0x30]);
        e.call_log = Some(vec![]);
        e.call(0x00a9_0180, &args![array, 6u32]);
        let log = e.call_log.take().unwrap();
        let allocations: Vec<_> = log.iter().filter(|call| call.0 == ALLOCATE).collect();
        assert_eq!(allocations.len(), 1);
        assert_eq!(allocations[0].1, args![24u32]);
        let frees: Vec<_> = log.iter().filter(|call| call.0 == FREE).collect();
        assert_eq!(frees.len(), 1);
        assert_eq!(frees[0].1, args![base]);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 6);
        assert_eq!(e.get(array, NiTArray::m_usSize), 3);
        assert_eq!(e.get(array, NiTArray::m_usESize), 2);
        let new = e.get(array, NiTArray::m_pBase);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(new + i * 4)).collect();
        assert_eq!(words, vec![0x10, 0, 0x30, 0, 0, 0]);
    }

    #[test]
    fn fn_00a90180_below_the_used_size_clears_the_cut_elements() {
        let (mut e, array, base) = array_fixture(4, &[0x10, 0x20, 0, 0x40]);
        e.call(0x00a9_0180, &args![array, 2u32]);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 2);
        assert_eq!(e.get(array, NiTArray::m_usSize), 2);
        assert_eq!(e.get(array, NiTArray::m_usESize), 2);
        // The cut slots of the old block were cleared before the copy.
        assert_eq!(e.mem.u32(base + 12), 0);
        let new = e.get(array, NiTArray::m_pBase);
        assert_eq!([e.mem.u32(new), e.mem.u32(new + 4)], [0x10, 0x20]);
    }

    #[test]
    fn fn_00a90180_to_zero_drops_the_storage() {
        let (mut e, array, base) = array_fixture(2, &[0x10]);
        e.call_log = Some(vec![]);
        e.call(0x00a9_0180, &args![array, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|call| call.0 != ALLOCATE));
        let frees: Vec<_> = log.iter().filter(|call| call.0 == FREE).collect();
        assert_eq!(frees[0].1, args![base]);
        assert_eq!(e.get(array, NiTArray::m_pBase), 0);
        assert_eq!(e.get(array, NiTArray::m_usMaxSize), 0);
        assert_eq!(e.get(array, NiTArray::m_usSize), 0);
        assert_eq!(e.get(array, NiTArray::m_usESize), 0);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;

    #[test]
    fn builds_the_block_with_two_sizes_of_eight() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        let block = e.mem.alloc(0x18);
        e.mem.write(block, &[0xaa; 0x18]);
        e.register_double(MEMORY_MANAGER_ALLOCATE, move |_, a| {
            assert_eq!(a, &[MEMORY_MANAGER_OBJECT, 0x18]);
            block.into_ret()
        });
        e.call(0x00a9_1290, &args![]);
        assert_eq!(e.mem.u32(SCREEN_SPACE_OBJECT_SLOT), block);
        assert_eq!(e.mem.bytes(block, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.u32(block + 0x10), 8);
        assert_eq!(e.mem.u32(block + 0x14), 8);
    }

    #[test]
    fn a_failed_allocation_stores_null() {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(SCREEN_SPACE_OBJECT_SLOT, 0x1234);
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        e.call(0x00a9_1290, &args![]);
        assert_eq!(e.mem.u32(SCREEN_SPACE_OBJECT_SLOT), 0);
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    fn calls(e: &mut Engine, top: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|call| call.0 != top)
            .collect()
    }

    fn fixture() -> Engine {
        let mut e = Engine::new();
        e.register(MEMORY_MANAGER_DEALLOCATE, |_, _| Ret::default());
        e.register(D3D_PASS_RELEASE, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        e.put_vtable(0x0150_0000, &[0x0150_1000]);
        e.register(0x0150_1000, |_, _| Ret::default());
        e
    }

    #[test]
    fn fn_00a90ac0_tears_down_the_inner_block_and_frees_the_storage() {
        let mut e = fixture();
        let this = e.mem.alloc(0x18);
        let inner = e.mem.alloc(0xc);
        let counted = e.mem.alloc(8);
        e.mem.set_u32(counted, 2);
        e.mem.set_u32(counted + 4, 0x0150_0000);
        e.mem.set_u32(this, 0x7000);
        e.mem.set_u32(this + 8, 5);
        e.mem.set_u32(this + 0xc, inner);
        e.mem.set_u32(inner, counted + 4);
        e.mem.set_u32(inner + 8, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x00a9_0ac0, &args![this]);
        assert_eq!(
            calls(&mut e, 0x00a9_0ac0),
            vec![
                (0x0150_1000, vec![counted + 4, 3]),
                (D3D_PASS_RELEASE, vec![0x6000, 1]),
                (
                    MEMORY_MANAGER_DEALLOCATE,
                    vec![MEMORY_MANAGER_OBJECT, inner]
                ),
                (FREE, vec![0x7000]),
            ]
        );
        assert_eq!(e.mem.u32(this + 8), 0);

        // A first word with a zero header is freed from the header; no third word.
        e.mem.set_u32(counted, 0);
        e.mem.set_u32(inner + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a9_0ac0, &args![this]);
        assert_eq!(
            calls(&mut e, 0x00a9_0ac0),
            vec![
                (
                    MEMORY_MANAGER_DEALLOCATE,
                    vec![MEMORY_MANAGER_OBJECT, counted]
                ),
                (
                    MEMORY_MANAGER_DEALLOCATE,
                    vec![MEMORY_MANAGER_OBJECT, inner]
                ),
                (FREE, vec![0x7000]),
            ]
        );

        // No inner block: only the storage is freed.
        e.mem.set_u32(this + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x00a9_0ac0, &args![this]);
        assert_eq!(calls(&mut e, 0x00a9_0ac0), vec![(FREE, vec![0x7000])]);
    }

    #[test]
    fn fn_00a91930_frees_the_built_object_when_there_is_one() {
        let mut e = fixture();
        e.map(SCREEN_SPACE_OBJECT_SLOT & !0xfff, 0x1000);
        e.call_log = Some(vec![]);
        e.call(0x00a9_1930, &args![]);
        assert!(calls(&mut e, 0x00a9_1930).is_empty());
        let object = e.mem.alloc(0x18);
        e.mem.set_u32(object, 0x7100);
        e.mem.set_u32(SCREEN_SPACE_OBJECT_SLOT, object);
        e.call_log = Some(vec![]);
        e.call(0x00a9_1930, &args![]);
        assert_eq!(
            calls(&mut e, 0x00a9_1930),
            vec![
                (FREE, vec![0x7100]),
                (
                    MEMORY_MANAGER_DEALLOCATE,
                    vec![MEMORY_MANAGER_OBJECT, object]
                ),
            ]
        );
        assert_eq!(e.mem.u32(SCREEN_SPACE_OBJECT_SLOT), object);
    }
}

#[cfg(test)]
mod a06_tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    /// Every slot of the test vtable points at `TARGETS + slot`.
    const VTABLE: u32 = 0x0320_0000;
    const TARGETS: u32 = 0x0330_0000;
    /// The test stream's read and write functions.
    const READ: u32 = 0x0340_0000;
    const WRITE: u32 = 0x0340_0010;

    type Written = Rc<RefCell<Vec<Vec<u8>>>>;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// The globals page, the test vtable, counting reference doubles, a memory manager
    /// that hands out fresh zeroed blocks, and the call log on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_5000, 0x1000);
        let slots: Vec<u32> = (0..0x50).map(|i| TARGETS + 4 * i).collect();
        e.put_vtable(VTABLE, &slots);
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            let size = if a[1] == u32::MAX { 4 } else { a[1] + 4 };
            e.mem.alloc(size).into_ret()
        });
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0] + 4).into_ret());
        quiet(
            &mut e,
            &[MEMORY_MANAGER_DEALLOCATE, FREE, SIZED_DELETE, TARGETS + 4],
        );
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, size: u32) -> u32 {
        let p = e.mem.alloc(size);
        e.mem.set_u32(p, VTABLE);
        p
    }

    /// A 16-bit sized pointer array (element block +4, capacity +8, used size +0xa, count
    /// +0xc, grow-by +0xe) at `array` holding `entries`, with room for `extra` more.
    fn fill_array(e: &mut Engine, array: u32, entries: &[u32], extra: u16) {
        let capacity = entries.len() as u16 + extra;
        let block = e.mem.alloc(4 * u32::from(capacity) + 8);
        for (i, v) in entries.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *v);
        }
        e.mem.set_u32(array + 4, block);
        e.mem.set_u16(array + 8, capacity);
        e.mem.set_u16(array + 0xa, entries.len() as u16);
        let count = entries.iter().filter(|v| **v != 0).count() as u16;
        e.mem.set_u16(array + 0xc, count);
        e.mem.set_u16(array + 0xe, 2);
    }

    /// A stream of the file version `version` that reads from `input` and records what is
    /// written; its vtable is the test vtable (`SaveLinkID` at `TARGETS + 0x30`).
    fn stream(e: &mut Engine, version: u32, input: &[u8]) -> (u32, Written) {
        let s = object(e, 0x260);
        let io = e.mem.alloc(0x10);
        e.mem.set_u32(s + STREAM_FILE_VERSION, version);
        e.mem.set_u32(s + 0x24c, io);
        e.mem.set_u32(s + 0x250, io);
        e.mem.set_u32(io + 8, READ);
        e.mem.set_u32(io + 0xc, WRITE);
        let queue: Rc<RefCell<VecDeque<u8>>> =
            Rc::new(RefCell::new(input.iter().copied().collect()));
        e.register_double(READ, move |e, a| {
            let bytes: Vec<u8> = (0..a[2])
                .map(|_| queue.borrow_mut().pop_front().unwrap())
                .collect();
            e.mem.write(a[1], &bytes);
            a[2].into_ret()
        });
        let written = Written::default();
        let w = written.clone();
        e.register_double(WRITE, move |e, a| {
            w.borrow_mut().push(e.mem.bytes(a[1], a[2]));
            a[2].into_ret()
        });
        quiet(e, &[TARGETS + STREAM_SAVE_LINK_ID_SLOT]);
        (s, written)
    }

    #[test]
    fn create_geometry_constructs_or_answers_null() {
        let mut e = engine();
        quiet(&mut e, &[SHAPE_CONSTRUCT]);
        let p = e.call(0x00a8_ff80, &args![]).u32();
        assert_eq!(e.mem.u32(p), NI_SCREEN_GEOMETRY_VTABLE);
        assert_eq!(calls(&e, SHAPE_CONSTRUCT), vec![vec![p]]);
        assert_eq!(calls(&e, NI_NEW), vec![vec![0xc4]]);
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a8_ff80, &args![]).u32(), 0);
    }

    #[test]
    fn create_polygon_clears_the_fields() {
        let mut e = engine();
        quiet(&mut e, &[NI_OBJECT_CONSTRUCT]);
        e.register(NI_NEW, |e, a| {
            let p = e.mem.alloc(a[0]);
            e.mem.write(p, &vec![0xcc; a[0] as usize]);
            p.into_ret()
        });
        let p = e.call(0x00a8_ffc0, &args![]).u32();
        assert_eq!(e.mem.u32(p), NI_SCREEN_POLYGON_VTABLE);
        assert_eq!(e.mem.u16(p + POLYGON_VERTEX_COUNT), 0);
        for field in [POLYGON_VERTICES, POLYGON_TEXTURE, POLYGON_COLORS] {
            assert_eq!(e.mem.u32(p + field), 0);
        }
        assert_eq!(e.mem.u32(p + POLYGON_PROPERTIES), 0xcccc_cccc);
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a8_ffc0, &args![]).u32(), 0);
    }

    #[test]
    fn geometry_destructor_deletes_on_bit_0() {
        let mut e = engine();
        quiet(&mut e, &[SHAPE_DESTRUCT]);
        let p = object(&mut e, 0xc4);
        assert_eq!(e.call(0x00a9_0040, &args![p, 0u32]).u32(), p);
        assert_eq!(e.mem.u32(p), NI_SCREEN_GEOMETRY_VTABLE);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        e.call(0x00a9_0040, &args![p, 1u32]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![p, 0xc4]]);
        assert_eq!(calls(&e, SHAPE_DESTRUCT).len(), 2);
    }

    #[test]
    fn geometry_clone_copies_into_the_new_object_or_null() {
        let mut e = engine();
        quiet(&mut e, &[SHAPE_CONSTRUCT, SHAPE_COPY_MEMBERS]);
        let this = object(&mut e, 0xc4);
        let clone = e.call(0x00a9_0070, &args![this, 0x77u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_SCREEN_GEOMETRY_VTABLE);
        assert_eq!(calls(&e, SHAPE_COPY_MEMBERS), vec![vec![this, clone, 0x77]]);
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_0070, &args![this, 0x77u32]).u32(), 0);
        assert_eq!(calls(&e, SHAPE_COPY_MEMBERS)[1], vec![this, 0, 0x77]);
    }

    /// A polygon with a property state of seven test objects.
    fn polygon(e: &mut Engine) -> (u32, Vec<u32>) {
        let p = object(e, 0x1c);
        let state = e.mem.alloc(0x1c);
        let properties: Vec<u32> = (0..7).map(|_| object(e, 8)).collect();
        for (i, q) in properties.iter().enumerate() {
            e.mem.set_u32(state + 4 * i as u32, *q);
        }
        e.mem.set_u32(p + POLYGON_PROPERTIES, state);
        (p, properties)
    }

    #[test]
    fn polygon_register_streamables_registers_the_seven_properties() {
        let mut e = engine();
        let (p, properties) = polygon(&mut e);
        quiet(&mut e, &[TARGETS + SLOT_REGISTER_STREAMABLES]);
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| 0u32.into_ret());
        assert!(!e.call(0x00a9_00c0, &args![p, 0x55u32]).bool());
        assert!(calls(&e, TARGETS + SLOT_REGISTER_STREAMABLES).is_empty());
        e.register(NI_OBJECT_REGISTER_STREAMABLES, |_, _| 1u32.into_ret());
        assert!(e.call(0x00a9_00c0, &args![p, 0x55u32]).bool());
        let got: Vec<u32> = calls(&e, TARGETS + SLOT_REGISTER_STREAMABLES)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(got, properties);
    }

    #[test]
    fn camera_process_clone_walks_both_arrays() {
        let mut e = engine();
        quiet(&mut e, &[AV_PROCESS_CLONE, TARGETS + SLOT_PROCESS_CLONE]);
        let camera = object(&mut e, 0x134);
        let (a, b) = (object(&mut e, 8), object(&mut e, 8));
        fill_array(&mut e, camera + CAMERA_POLYGONS, &[a, 0], 0);
        fill_array(&mut e, camera + CAMERA_TEXTURES, &[0, b], 0);
        e.call(0x00a9_0100, &args![camera, 0x88u32]);
        assert_eq!(calls(&e, AV_PROCESS_CLONE), vec![vec![camera, 0x88]]);
        assert_eq!(
            calls(&e, TARGETS + SLOT_PROCESS_CLONE),
            vec![vec![a, 0x88], vec![b, 0x88]]
        );
    }

    #[test]
    fn element_array_destructors_free_and_delete_on_bit_0() {
        let mut e = engine();
        quiet(&mut e, &[POINTER_ARRAY_DELETE]);
        for (addr, vtable, free) in [
            (0x00a9_0260u32, ELEMENT_ARRAY_VTABLE, FREE),
            (0x00a9_10f0, TEXTURE_ARRAY_VTABLE, POINTER_ARRAY_DELETE),
            (0x00a9_18e0, POLYGON_ARRAY_VTABLE, POINTER_ARRAY_DELETE),
        ] {
            let array = e.mem.alloc(0x10);
            e.mem.set_u32(array + 4, 0x0123_0000);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &args![array, 0u32]).u32(), array);
            assert_eq!(e.mem.u32(array), vtable);
            assert_eq!(calls(&e, free), vec![vec![0x0123_0000]]);
            assert!(calls(&e, MEMORY_MANAGER_DEALLOCATE).is_empty());
            e.call(addr, &args![array, 1u32]);
            assert_eq!(
                calls(&e, MEMORY_MANAGER_DEALLOCATE),
                vec![vec![MEMORY_MANAGER_OBJECT, array]]
            );
        }
    }

    #[test]
    fn element_array_new_counts_and_constructs() {
        let mut e = engine();
        quiet(&mut e, &[VECTOR_CONSTRUCT]);
        let first = e.call(0x00a9_0290, &args![3u32]).u32();
        assert_eq!(e.mem.u32(first - 4), 3);
        assert_eq!(
            calls(&e, MEMORY_MANAGER_ALLOCATE),
            vec![vec![MEMORY_MANAGER_OBJECT, 3 * 0x14 + 4]]
        );
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![vec![first, 0x14, 3, SCREEN_ELEMENT_CONSTRUCT]]
        );
        // An overflowing size asks for 0xffffffff bytes; a failed allocation is null.
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_0290, &args![0x1000_0000u32]).u32(), 0);
        assert_eq!(calls(&e, MEMORY_MANAGER_ALLOCATE)[1][1], u32::MAX);
    }

    #[test]
    fn element_vector_destructor_handles_arrays_and_single_elements() {
        let mut e = engine();
        let block = e.mem.alloc(4 + 2 * 0x14);
        e.mem.set_u32(block, 2);
        let first = block + 4;
        for i in 0..2u32 {
            for (k, field) in [8u32, 0xc, 0x10].iter().enumerate() {
                e.mem
                    .set_u32(first + 0x14 * i + field, 0x100 * (i + 1) + k as u32);
            }
        }
        assert_eq!(e.call(0x00a9_02e0, &args![first, 3u32]).u32(), block);
        let freed: Vec<u32> = calls(&e, MEMORY_MANAGER_DEALLOCATE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(freed, vec![0x200, 0x201, 0x202, 0x100, 0x101, 0x102, block]);
        assert_eq!(e.mem.u32(first), SCREEN_ELEMENT_VTABLE);
        // One element, kept.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_02e0, &args![first, 0u32]).u32(), first);
        assert_eq!(calls(&e, MEMORY_MANAGER_DEALLOCATE).len(), 3);
    }

    /// A screen element with `vertices` vertices and one texture set, its blocks filled
    /// from `seed`.
    fn element(e: &mut Engine, vertices: u32, seed: f32) -> u32 {
        let el = e.mem.alloc(ELEMENT_SIZE);
        e.mem.set_u16(el + 4, vertices as u16);
        e.mem.set_u16(el + 6, 1);
        for (field, floats) in [(8u32, 2u32), (0xc, 4), (0x10, 2)] {
            let block = e.mem.alloc(4 * floats * vertices);
            for i in 0..floats * vertices {
                e.mem.set_f32(block + 4 * i, seed + i as f32);
            }
            e.mem.set_u32(el + field, block);
        }
        el
    }

    fn geometry_data(e: &mut Engine, elements: &[u32]) -> u32 {
        let data = object(e, 0x70);
        fill_array(e, data + ELEMENTS, elements, 0);
        data
    }

    #[test]
    fn geometry_data_is_equal_compares_each_element() {
        let mut e = engine();
        e.register(TRI_SHAPE_DATA_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(POINT2_DIFFERS, |e, a| {
            u32::from(e.mem.bytes(a[0], 8) != e.mem.bytes(a[1], 8)).into_ret()
        });
        e.register(COLOR_A_DIFFERS, |e, a| {
            u32::from(e.mem.bytes(a[0], 16) != e.mem.bytes(a[1], 16)).into_ret()
        });
        let elements: Vec<u32> = [1.0, 1.0, 5.0, 1.0]
            .iter()
            .map(|seed| element(&mut e, 2, *seed))
            .collect();
        let a = geometry_data(&mut e, &[elements[0], 0]);
        let b = geometry_data(&mut e, &[elements[1], 0]);
        let c = geometry_data(&mut e, &[elements[2], 0]);
        let d = geometry_data(&mut e, &[0, elements[3]]);
        let eq = |e: &mut Engine, x: u32, y: u32| e.call(0x00a9_03a0, &args![x, y]).bool();
        assert!(eq(&mut e, a, b));
        assert_eq!(calls(&e, POINT2_DIFFERS).len(), 4);
        assert_eq!(calls(&e, COLOR_A_DIFFERS).len(), 2);
        assert!(!eq(&mut e, a, c));
        assert!(!eq(&mut e, a, d));
        e.mem.set_u8(b + PIXEL_ACCURATE, 1);
        assert!(!eq(&mut e, a, b));
        e.mem.set_u8(b + PIXEL_ACCURATE, 0);
        e.register(TRI_SHAPE_DATA_IS_EQUAL, |_, _| 0u32.into_ret());
        assert!(!eq(&mut e, a, b));
    }

    #[test]
    fn polygon_destructor_gives_back_the_blocks_and_deletes_on_bit_0() {
        let mut e = engine();
        quiet(&mut e, &[NI_OBJECT_DESTRUCT]);
        let p = object(&mut e, 0x1c);
        for (i, field) in [POLYGON_VERTICES, POLYGON_TEXTURE, POLYGON_COLORS]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(p + field, 0x10 + i as u32);
        }
        assert_eq!(e.call(0x00a9_0560, &args![p, 0u32]).u32(), p);
        let freed: Vec<u32> = calls(&e, MEMORY_MANAGER_DEALLOCATE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(freed, vec![0x10, 0x11, 0x12]);
        assert_eq!(e.mem.u32(p), NI_SCREEN_POLYGON_VTABLE);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        e.call(0x00a9_0560, &args![p, 1u32]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![p, 0x1c]]);
    }

    #[test]
    fn polygon_clone_records_and_copies_the_property_state() {
        let mut e = engine();
        quiet(&mut e, &[NI_OBJECT_CONSTRUCT, NI_OBJECT_RECORD_CLONE]);
        e.register(PROPERTY_STATE_COPY, |_, a| a[0].into_ret());
        let (p, _) = polygon(&mut e);
        let clone = e.call(0x00a9_05c0, &args![p, 0x66u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_SCREEN_POLYGON_VTABLE);
        assert_eq!(
            calls(&e, NI_OBJECT_RECORD_CLONE),
            vec![vec![p, clone, 0x66]]
        );
        let copy = calls(&e, PROPERTY_STATE_COPY);
        assert_eq!(copy[0][1], e.mem.u32(p + POLYGON_PROPERTIES));
        assert_eq!(e.mem.u32(clone + POLYGON_PROPERTIES), copy[0][0]);
        // Without memory for the state the clone gets none.
        e.register(MEMORY_MANAGER_ALLOCATE, |_, _| 0u32.into_ret());
        let clone = e.call(0x00a9_05c0, &args![p, 0x66u32]).u32();
        assert_eq!(e.mem.u32(clone + POLYGON_PROPERTIES), 0);
    }

    #[test]
    fn polygon_link_object_stores_properties_by_type() {
        let mut e = engine();
        quiet(&mut e, &[EMPTY_BASE]);
        let (p, properties) = polygon(&mut e);
        let replacement = object(&mut e, 8);
        let ignored = object(&mut e, 8);
        e.mem.set_u32(properties[2] + 4, 1);
        let kinds = vec![(replacement, 2u32), (ignored, 8), (properties[3], 3)];
        e.register_double(TARGETS + SLOT_PROPERTY_TYPE, move |_, a| {
            let kind = kinds.iter().find(|(o, _)| *o == a[0]).unwrap().1;
            kind.into_ret()
        });
        let links = Rc::new(RefCell::new(vec![properties[3], ignored, 0, replacement]));
        let l = links.clone();
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            l.borrow_mut().pop().unwrap().into_ret()
        });
        e.register(STREAM_GET_NUMBER_OF_LINK_IDS, |_, _| 4u32.into_ret());
        e.call(0x00a9_0640, &args![p, 0x99u32]);
        let state = e.mem.u32(p + POLYGON_PROPERTIES);
        assert_eq!(e.mem.u32(state + 8), replacement);
        assert_eq!(e.mem.u32(replacement + 4), 1);
        // The old type-2 property was released (its count reached 0: deleted).
        assert_eq!(e.mem.u32(properties[2] + 4), 0);
        assert_eq!(calls(&e, TARGETS + 4), vec![vec![properties[2]]]);
        // The same object at its own slot changes nothing; type 8 is ignored.
        assert_eq!(e.mem.u32(state + 0xc), properties[3]);
        assert_eq!(e.mem.u32(properties[3] + 4), 0);
    }

    /// A polygon with `count` vertices whose floats start at `seed`, texture coordinates
    /// and colors only when asked for.
    fn shaped_polygon(e: &mut Engine, count: u32, seed: f32, texture: bool, colors: bool) -> u32 {
        let (p, _) = polygon(e);
        e.mem.set_u16(p + POLYGON_VERTEX_COUNT, count as u16);
        for (field, floats, wanted) in [
            (POLYGON_VERTICES, 3u32, true),
            (POLYGON_TEXTURE, 2, texture),
            (POLYGON_COLORS, 4, colors),
        ] {
            if wanted {
                let block = e.mem.alloc(4 * floats * count);
                for i in 0..floats * count {
                    e.mem.set_f32(block + 4 * i, seed + i as f32);
                }
                e.mem.set_u32(p + field, block);
            }
        }
        p
    }

    #[test]
    fn polygon_is_equal_compares_floats_arrays_and_properties() {
        let mut e = engine();
        e.register(NI_OBJECT_IS_EQUAL, |_, _| 1u32.into_ret());
        let a = shaped_polygon(&mut e, 2, 1.0, true, false);
        let b = shaped_polygon(&mut e, 2, 1.0, true, false);
        let eq = |e: &mut Engine, x: u32, y: u32| e.call(0x00a9_06e0, &args![x, y]).bool();
        // Different property objects.
        assert!(!eq(&mut e, a, b));
        let state = e.mem.u32(a + POLYGON_PROPERTIES);
        e.mem.set_u32(b + POLYGON_PROPERTIES, state);
        assert!(eq(&mut e, a, b));
        let vertices = e.mem.u32(b + POLYGON_VERTICES);
        e.mem.set_f32(vertices + 0x14, f32::NAN);
        assert!(!eq(&mut e, a, b));
        e.mem.set_f32(vertices + 0x14, 6.0);
        assert!(eq(&mut e, a, b));
        let c = shaped_polygon(&mut e, 2, 1.0, false, false);
        e.mem.set_u32(c + POLYGON_PROPERTIES, state);
        assert!(!eq(&mut e, a, c));
        let d = shaped_polygon(&mut e, 2, 1.0, true, true);
        e.mem.set_u32(d + POLYGON_PROPERTIES, state);
        assert!(!eq(&mut e, a, d));
        let shorter = shaped_polygon(&mut e, 1, 1.0, true, false);
        assert!(!eq(&mut e, a, shorter));
    }

    #[test]
    fn polygon_set_at_initializes_its_null_and_counts() {
        let mut e = engine();
        e.register(ATEXIT, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        let (a, b) = (object(&mut e, 8), object(&mut e, 8));
        e.mem.set_u32(a + 4, 1);
        fill_array(&mut e, array, &[a, 0], 2);
        let value = e.mem.alloc(4);
        // Beyond the used size: the size grows, the count too for a non-null value.
        e.mem.set_u32(value, b);
        e.call(0x00a9_08b0, &args![array, 3u32, value]);
        assert_eq!(calls(&e, ATEXIT), vec![vec![POLYGON_NULL_DESTRUCT]]);
        assert_eq!(e.mem.u8(POLYGON_NULL_GUARD) & 1, 1);
        assert_eq!((e.mem.u16(array + 0xa), e.mem.u16(array + 0xc)), (4, 2));
        assert_eq!(e.mem.u32(b + 4), 1);
        // Inside it: filling an empty entry counts up, emptying one counts down.
        e.call(0x00a9_08b0, &args![array, 1u32, value]);
        assert_eq!(e.mem.u16(array + 0xc), 3);
        e.mem.set_u32(value, 0);
        e.call(0x00a9_08b0, &args![array, 0u32, value]);
        assert_eq!(e.mem.u16(array + 0xc), 2);
        // The old entry was released and deleted at zero.
        assert_eq!(calls(&e, TARGETS + 4), vec![vec![a]]);
        // The guard is set: no second registration.
        assert_eq!(calls(&e, ATEXIT).len(), 1);
    }

    #[test]
    fn texture_set_at_uses_its_own_null() {
        let mut e = engine();
        e.register(ATEXIT, |_, _| Ret::default());
        let array = e.mem.alloc(0x10);
        fill_array(&mut e, array, &[], 2);
        let b = object(&mut e, 8);
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, b);
        e.call(0x00a9_0980, &args![array, 0u32, value]);
        assert_eq!(calls(&e, ATEXIT), vec![vec![TEXTURE_NULL_DESTRUCT]]);
        assert_eq!(e.mem.u8(TEXTURE_NULL_GUARD) & 1, 1);
        assert_eq!((e.mem.u16(array + 0xa), e.mem.u16(array + 0xc)), (1, 1));
        assert_eq!(e.mem.u32(e.mem.u32(array + 4)), b);
    }

    #[test]
    fn geometry_data_constructor_sets_up_the_element_array() {
        let mut e = engine();
        quiet(&mut e, &[TRI_SHAPE_DATA_CONSTRUCT]);
        let data = e.mem.alloc(0x70);
        e.mem.write(data, &[0xcc; 0x70]);
        assert_eq!(e.call(0x00a9_0a50, &args![data]).u32(), data);
        assert_eq!(e.mem.u32(data), NI_SCREEN_GEOMETRY_DATA_VTABLE);
        assert_eq!(e.mem.u32(data + ELEMENTS), ELEMENT_PRIMITIVE_ARRAY_VTABLE);
        assert_eq!(
            (
                e.mem.u16(data + 0x68),
                e.mem.u16(data + 0x6a),
                e.mem.u16(data + 0x6c),
                e.mem.u16(data + 0x6e)
            ),
            (4, 0, 0, 4)
        );
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x10]]);
        assert_ne!(e.mem.u32(data + 0x64), 0);
        assert_eq!(
            (
                e.mem.u8(data + 0x58),
                e.mem.u8(data + 0x59),
                e.mem.u32(data + 0x5c)
            ),
            (0, 0, 0)
        );
    }

    #[test]
    fn create_geometry_data_runs_the_constructor_or_answers_null() {
        let mut e = engine();
        quiet(&mut e, &[TRI_SHAPE_DATA_CONSTRUCT]);
        let p = e.call(0x00a9_0b20, &args![]).u32();
        assert_eq!(e.mem.u32(p), NI_SCREEN_GEOMETRY_DATA_VTABLE);
        assert_eq!(calls(&e, NI_NEW), vec![vec![0x70]]);
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_0b20, &args![]).u32(), 0);
    }

    #[test]
    fn geometry_data_save_binary_writes_each_element() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                TRI_SHAPE_DATA_SAVE_BINARY,
                POINT2_SAVE_BINARY,
                COLOR_A_SAVE_BINARY,
            ],
        );
        let el = element(&mut e, 2, 1.0);
        let data = geometry_data(&mut e, &[el, 0]);
        e.mem.set_u8(data + PIXEL_ACCURATE, 1);
        let (s, written) = stream(&mut e, 0x1402_0000, &[]);
        e.call(0x00a9_0b40, &args![data, s]);
        let written = written.borrow();
        assert_eq!(
            *written,
            vec![
                vec![1],
                2u32.to_le_bytes().to_vec(),
                vec![2, 0],
                vec![1],
                vec![1],
                vec![1, 0],
                vec![0, 0],
            ]
        );
        let points = e.mem.u32(el + 8);
        let coordinates = e.mem.u32(el + 0x10);
        let saved: Vec<u32> = calls(&e, POINT2_SAVE_BINARY).iter().map(|c| c[0]).collect();
        assert_eq!(
            saved,
            vec![points, points + 8, coordinates, coordinates + 8]
        );
        assert_eq!(calls(&e, COLOR_A_SAVE_BINARY).len(), 2);
    }

    #[test]
    fn polygon_load_binary_reads_old_and_new_formats() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                NI_OBJECT_LOAD_BINARY,
                POINT3_LOAD_BINARY,
                POINT2_LOAD_BINARY,
                COLOR_A_LOAD_BINARY,
                STREAM_READ_MULTIPLE_LINK_IDS,
            ],
        );
        // New format: two vertices, a texture flag 1, a color flag 0.
        let (s, _) = stream(&mut e, 0x0401_0000, &[2, 0, 1, 0]);
        let p = object(&mut e, 0x1c);
        e.call(0x00a9_0d70, &args![p, s]);
        assert_eq!(e.mem.u16(p + POLYGON_VERTEX_COUNT), 2);
        assert_ne!(e.mem.u32(p + POLYGON_VERTICES), 0);
        assert_ne!(e.mem.u32(p + POLYGON_TEXTURE), 0);
        assert_eq!(e.mem.u32(p + POLYGON_COLORS), 0);
        assert_eq!(calls(&e, POINT3_LOAD_BINARY).len(), 2);
        assert_eq!(calls(&e, POINT2_LOAD_BINARY).len(), 2);
        assert!(calls(&e, COLOR_A_LOAD_BINARY).is_empty());
        assert_eq!(calls(&e, STREAM_READ_MULTIPLE_LINK_IDS), vec![vec![s]]);
        // Old format: 4-byte words read into the fields; the colors get zeroed floats.
        e.call_log = Some(vec![]);
        let (s, _) = stream(&mut e, 0x0400_0000, &[1, 0, 0, 0, 0, 0, 9, 0, 0, 0]);
        let p = object(&mut e, 0x1c);
        e.call(0x00a9_0d70, &args![p, s]);
        assert_eq!(e.mem.u32(p + POLYGON_TEXTURE), 0);
        let colors = e.mem.u32(p + POLYGON_COLORS);
        assert_ne!(colors, 9);
        assert_eq!(e.mem.bytes(colors, 16), vec![0; 16]);
        assert_eq!(calls(&e, COLOR_A_LOAD_BINARY), vec![vec![colors, s]]);
        let sizes: Vec<u32> = calls(&e, MEMORY_MANAGER_ALLOCATE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(sizes, vec![0xc, 0x10]);
    }

    #[test]
    fn polygon_save_binary_writes_the_arrays_and_seven_links() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                EMPTY_BASE,
                POINT3_SAVE_BINARY,
                POINT2_SAVE_BINARY,
                COLOR_A_SAVE_BINARY,
            ],
        );
        let p = shaped_polygon(&mut e, 2, 1.0, false, true);
        let properties: Vec<u32> = {
            let state = e.mem.u32(p + POLYGON_PROPERTIES);
            (0..7).map(|i| e.mem.u32(state + 4 * i)).collect()
        };
        let (s, written) = stream(&mut e, 0x1402_0000, &[]);
        e.call(0x00a9_0f70, &args![p, s]);
        assert_eq!(
            *written.borrow(),
            vec![vec![2, 0], vec![0], vec![1], 7u32.to_le_bytes().to_vec()]
        );
        assert_eq!(calls(&e, POINT3_SAVE_BINARY).len(), 2);
        assert!(calls(&e, POINT2_SAVE_BINARY).is_empty());
        assert_eq!(calls(&e, COLOR_A_SAVE_BINARY).len(), 2);
        let links: Vec<u32> = calls(&e, TARGETS + STREAM_SAVE_LINK_ID_SLOT)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(links, properties);
    }

    /// The element pool `00a91290` builds, with room for `capacity` free elements.
    fn pool(e: &mut Engine, capacity: u32) -> u32 {
        let pool = e.mem.alloc(0x18);
        let free = e.mem.alloc(4 * capacity + 4);
        e.mem.set_u32(pool, free);
        e.mem.set_u32(pool + 4, capacity);
        e.mem.set_u32(pool + 0x10, 2);
        e.mem.set_u32(SCREEN_SPACE_OBJECT_SLOT, pool);
        pool
    }

    #[test]
    fn pool_chunks_push_their_elements_and_grow_the_free_array() {
        let mut e = engine();
        quiet(&mut e, &[VECTOR_CONSTRUCT]);
        let pool = pool(&mut e, 1);
        e.register(FREE_LIST_REALLOC, |e, a| {
            let bigger = e.mem.alloc(4 * a[1] + 4);
            let used = e.mem.u32(a[0] + 8);
            let old = e.mem.u32(a[0]);
            let bytes = e.mem.bytes(old, 4 * used);
            e.mem.write(bigger, &bytes);
            e.mem.set_u32(a[0], bigger);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.call(0x00a9_1120, &args![pool, 3u32]);
        let header = e.mem.u32(pool + 0xc);
        let first = e.mem.u32(header);
        assert_eq!(e.mem.u32(header + 4), 3);
        assert_eq!(e.mem.u32(header + 8), 0);
        assert_eq!(e.mem.u32(pool + 8), 3);
        let free = e.mem.u32(pool);
        let elements: Vec<u32> = (0..3).map(|i| e.mem.u32(free + 4 * i)).collect();
        assert_eq!(elements, vec![first, first + 0x14, first + 0x28]);
        // Grown from 1 to 2, then to 4.
        let grown: Vec<u32> = calls(&e, FREE_LIST_REALLOC).iter().map(|c| c[1]).collect();
        assert_eq!(grown, vec![2, 4]);
        // A second chunk goes in front of the first.
        e.call(0x00a9_1120, &args![pool, 0u32]);
        let second = e.mem.u32(pool + 0xc);
        assert_eq!((e.mem.u32(second), e.mem.u32(second + 8)), (0, header));
    }

    #[test]
    fn geometry_data_destructor_returns_elements_to_the_pool() {
        let mut e = engine();
        quiet(&mut e, &[POOL_RELEASE, TRI_SHAPE_DATA_DESTRUCT]);
        pool(&mut e, 4);
        let el = element(&mut e, 1, 1.0);
        let data = geometry_data(&mut e, &[el, 0]);
        let blocks: Vec<u32> = [8u32, 0xc, 0x10]
            .iter()
            .map(|f| e.mem.u32(el + f))
            .collect();
        let old_block = e.mem.u32(data + 0x64);
        let l = Rc::new(RefCell::new(vec![]));
        let seen = l.clone();
        e.register_double(POOL_RELEASE, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        e.call(0x00a9_11d0, &args![data]);
        assert_eq!(e.mem.u32(data), NI_SCREEN_GEOMETRY_DATA_VTABLE);
        assert_eq!(e.mem.u32(data + ELEMENTS), ELEMENT_ARRAY_VTABLE);
        let freed: Vec<u32> = calls(&e, MEMORY_MANAGER_DEALLOCATE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(freed, blocks);
        let pool = e.mem.u32(SCREEN_SPACE_OBJECT_SLOT);
        assert_eq!(*l.borrow(), vec![(pool, el)]);
        // Emptied through 00a90180 (to no storage), the old block freed there and again.
        assert_eq!(e.mem.u16(data + 0x6a), 0);
        assert_eq!(calls(&e, FREE)[0], vec![old_block]);
        assert_eq!(calls(&e, TRI_SHAPE_DATA_DESTRUCT), vec![vec![data]]);
    }

    /// A viewer-string list with room for `capacity` strings and a `SetAt` that counts.
    fn strings(e: &mut Engine, capacity: u16) -> u32 {
        let list = e.mem.alloc(0x10);
        // Room for more strings than the capacity says: the inline adds store past it.
        let block = e.mem.alloc(0x100);
        e.mem.set_u32(list + 4, block);
        e.mem.set_u16(list + 8, capacity);
        e.mem.set_u16(list + 0xe, 4);
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            Ret::default()
        });
        quiet(e, &[STRING_ARRAY_SET_SIZE]);
        for (addr, eax) in [
            (VIEWER_STRING_NAME, 0x61u32),
            (VIEWER_STRING_BOOL, 0x62),
            (VIEWER_STRING_SHORT, 0x63),
            (VIEWER_STRING_UNSIGNED, 0x64),
            (VIEWER_STRING_POINTER, 0x65),
        ] {
            e.register_double(addr, move |_, _| eax.into_ret());
        }
        list
    }

    #[test]
    fn geometry_data_viewer_strings_add_name_accuracy_and_count() {
        let mut e = engine();
        quiet(&mut e, &[GEOMETRY_DATA_GET_VIEWER_STRINGS]);
        e.mem.set_u32(NI_SCREEN_GEOMETRY_DATA_RTTI, 0x0abc_0000);
        let data = geometry_data(&mut e, &[1, 0, 2]);
        e.mem.set_u8(data + PIXEL_ACCURATE, 1);
        let list = strings(&mut e, 8);
        e.call(0x00a9_12d0, &args![data, list]);
        assert_eq!(calls(&e, VIEWER_STRING_NAME), vec![vec![0x0abc_0000]]);
        assert_eq!(
            calls(&e, VIEWER_STRING_BOOL),
            vec![vec![TEXT_PIXEL_ACCURATE, 1]]
        );
        assert_eq!(
            calls(&e, VIEWER_STRING_UNSIGNED),
            vec![vec![TEXT_NUM_ELEMENTS, 2]]
        );
        assert_eq!(e.mem.u16(list + 0xa), 3);
    }

    #[test]
    fn geometry_and_camera_viewer_strings_add_the_class_name() {
        for (addr, base, rtti) in [
            (
                0x00a9_13a0u32,
                TRI_SHAPE_GET_VIEWER_STRINGS,
                NI_SCREEN_GEOMETRY_RTTI,
            ),
            (
                0x00a9_1890,
                CAMERA_GET_VIEWER_STRINGS,
                NI_SCREEN_SPACE_CAMERA_RTTI,
            ),
        ] {
            let mut e = engine();
            quiet(&mut e, &[base]);
            e.mem.set_u32(rtti, 0x0abc_0010);
            let list = strings(&mut e, 0);
            let this = object(&mut e, 0x134);
            e.call(addr, &args![this, list]);
            assert_eq!(calls(&e, base), vec![vec![this, list]]);
            assert_eq!(calls(&e, VIEWER_STRING_NAME), vec![vec![0x0abc_0010]]);
            // The full list grew by its grow-by count first.
            assert_eq!(calls(&e, STRING_ARRAY_SET_SIZE), vec![vec![list, 4]]);
        }
    }

    #[test]
    fn polygon_viewer_strings_add_fields_and_the_properties_inline() {
        let mut e = engine();
        quiet(&mut e, &[NI_OBJECT_GET_VIEWER_STRINGS]);
        e.mem.set_u32(NI_SCREEN_POLYGON_RTTI, 0x0abc_0020);
        let p = shaped_polygon(&mut e, 3, 1.0, false, false);
        let rtti = e.mem.alloc(8);
        e.mem.set_u32(rtti, 0x0abc_0030);
        e.register_double(TARGETS + SLOT_GET_RTTI, move |_, _| rtti.into_ret());
        let list = strings(&mut e, 4);
        e.call(0x00a9_13f0, &args![p, list]);
        assert_eq!(calls(&e, VIEWER_STRING_SHORT), vec![vec![TEXT_VERTICES, 3]]);
        let pointers = calls(&e, VIEWER_STRING_POINTER);
        assert_eq!(pointers.len(), 3 + 7);
        assert_eq!(
            pointers[0],
            vec![TEXT_VERTEX, e.mem.u32(p + POLYGON_VERTICES)]
        );
        assert_eq!(pointers[1], vec![TEXT_TEXTURE, 0]);
        let state = e.mem.u32(p + POLYGON_PROPERTIES);
        assert_eq!(pointers[3], vec![0x0abc_0030, e.mem.u32(state)]);
        // Five through SetAt, seven inline: size 12, count 7 (the inline ones count).
        assert_eq!(e.mem.u16(list + 0xa), 12);
        assert_eq!(e.mem.u16(list + 0xc), 7);
        let block = e.mem.u32(list + 4);
        assert_eq!(e.mem.u32(block + 4 * 11), 0x65);
    }

    #[test]
    fn camera_destructor_empties_both_arrays_and_runs_the_base() {
        let mut e = engine();
        e.register(ATEXIT, |_, _| Ret::default());
        quiet(
            &mut e,
            &[
                POINTER_ARRAY_00A2E620,
                POINTER_ARRAY_DELETE,
                CAMERA_DESTRUCT,
            ],
        );
        let camera = object(&mut e, 0x134);
        let (a, b) = (object(&mut e, 8), object(&mut e, 8));
        e.mem.set_u32(a + 4, 1);
        e.mem.set_u32(b + 4, 2);
        fill_array(&mut e, camera + CAMERA_POLYGONS, &[a], 0);
        fill_array(&mut e, camera + CAMERA_TEXTURES, &[0, b], 0);
        let (polygons, textures) = (
            e.mem.u32(camera + CAMERA_POLYGONS + 4),
            e.mem.u32(camera + CAMERA_TEXTURES + 4),
        );
        e.call(0x00a9_15d0, &args![camera]);
        assert_eq!(e.mem.u32(camera), NI_SCREEN_SPACE_CAMERA_VTABLE);
        assert_eq!((e.mem.u32(polygons), e.mem.u32(textures + 4)), (0, 0));
        assert_eq!(calls(&e, TARGETS + 4), vec![vec![a]]);
        assert_eq!(e.mem.u32(b + 4), 1);
        assert_eq!(
            calls(&e, POINTER_ARRAY_DELETE),
            vec![vec![textures], vec![polygons]]
        );
        assert_eq!(e.mem.u32(camera + CAMERA_POLYGONS), POLYGON_ARRAY_VTABLE);
        assert_eq!(e.mem.u32(camera + CAMERA_TEXTURES), TEXTURE_ARRAY_VTABLE);
        assert_eq!(calls(&e, CAMERA_DESTRUCT), vec![vec![camera]]);
    }

    #[test]
    fn camera_member_copy_clones_every_entry() {
        let mut e = engine();
        e.register(ATEXIT, |_, _| Ret::default());
        quiet(&mut e, &[CAMERA_COPY_MEMBERS]);
        e.register(POINTER_ARRAY_SET_SIZE, |e, a| {
            let block = e.mem.alloc(4 * a[1] + 4);
            e.mem.set_u32(a[0] + 4, block);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        let copies = Rc::new(RefCell::new(vec![]));
        let c = copies.clone();
        e.register_double(TARGETS + SLOT_CREATE_CLONE, move |e, a| {
            let copy = e.mem.alloc(8);
            e.mem.set_u32(copy, VTABLE);
            c.borrow_mut().push((a[0], copy));
            copy.into_ret()
        });
        let this = object(&mut e, 0x134);
        let clone = object(&mut e, 0x134);
        let a = object(&mut e, 8);
        e.mem.set_u32(a + 4, 1);
        fill_array(&mut e, this + CAMERA_POLYGONS, &[a, 0], 0);
        fill_array(&mut e, this + CAMERA_TEXTURES, &[], 0);
        e.call(0x00a9_16a0, &args![this, clone, 0x44u32]);
        assert_eq!(
            calls(&e, CAMERA_COPY_MEMBERS),
            vec![vec![this, clone, 0x44]]
        );
        assert_eq!(calls(&e, TARGETS + SLOT_CREATE_CLONE), vec![vec![a, 0x44]]);
        let copy = copies.borrow()[0].1;
        let target = clone + CAMERA_POLYGONS;
        let block = e.mem.u32(target + 4);
        assert_eq!((e.mem.u32(block), e.mem.u32(block + 4)), (copy, 0));
        assert_eq!(e.mem.u16(target + 0xa), 2);
        assert_eq!(e.mem.u16(target + 0xe), 2);
        assert_eq!(e.mem.u32(copy + 4), 1);
        assert_eq!(e.mem.u32(a + 4), 1);
        assert_eq!(
            calls(&e, POINTER_ARRAY_SET_SIZE),
            vec![
                vec![clone + CAMERA_POLYGONS, 2],
                vec![clone + CAMERA_TEXTURES, 0]
            ]
        );
    }

    #[test]
    fn geometry_data_scalar_destructor_destroys_and_deletes_on_bit_0() {
        let mut e = engine();
        quiet(&mut e, &[TRI_SHAPE_DATA_DESTRUCT]);
        pool(&mut e, 2);
        let data = geometry_data(&mut e, &[]);
        assert_eq!(e.call(0x00a9_1910, &args![data, 0u32]).u32(), data);
        assert_eq!(calls(&e, TRI_SHAPE_DATA_DESTRUCT), vec![vec![data]]);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        let data = geometry_data(&mut e, &[]);
        e.call(0x00a9_1910, &args![data, 1u32]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![data, 0x70]]);
    }

    #[test]
    fn geometry_data_load_binary_reads_the_list_and_elements_through_the_pool() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                GEOMETRY_DATA_LOAD_BINARY,
                TRI_SHAPE_DATA_LOAD_BINARY,
                VECTOR_CONSTRUCT,
            ],
        );
        let pool = pool(&mut e, 4);
        // One pooled element with old blocks.
        let spare = e.mem.alloc(ELEMENT_SIZE);
        for (field, old) in [(8u32, 0x81u32), (0xc, 0x82), (0x10, 0x83)] {
            e.mem.set_u32(spare + field, old);
        }
        let free = e.mem.u32(pool);
        e.mem.set_u32(free, spare);
        e.mem.set_u32(pool + 8, 1);
        let stored = Rc::new(RefCell::new(vec![]));
        let st = stored.clone();
        e.register_double(ELEMENT_ARRAY_SET_AT, move |e, a| {
            st.borrow_mut().push((a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        // Old format: accuracy 1, 3 vertices, 1 triangle, a list of 3 indices, 2 elements:
        // the first empty, the second with 1 vertex, a point, no colors, one texture set.
        let mut input = vec![1, 3, 0, 1, 0, 3, 0, 0, 0, 1, 0, 2, 0];
        input.extend(2u32.to_le_bytes());
        input.extend([0, 0]);
        input.extend([1, 0, 1]);
        input.extend([0u8; 8]);
        input.extend([0, 1, 0]);
        input.extend([0u8; 8]);
        let (s, _) = stream(&mut e, 0x0a00_0100, &input);
        let data = geometry_data(&mut e, &[]);
        e.call(0x00a9_1950, &args![data, s]);
        assert_eq!(calls(&e, GEOMETRY_DATA_LOAD_BINARY), vec![vec![data, s]]);
        assert_eq!(e.mem.u8(data + PIXEL_ACCURATE), 1);
        assert_eq!((e.mem.u16(data + 8), e.mem.u16(data + 0x40)), (3, 1));
        assert_eq!(e.mem.u32(data + 0x44), 3);
        let list = e.mem.u32(data + 0x48);
        assert_eq!(e.mem.bytes(list, 6), vec![0, 0, 1, 0, 2, 0]);
        assert_eq!(*stored.borrow(), vec![(1, spare)]);
        assert_eq!(e.mem.u16(spare + 4), 1);
        assert_eq!(e.mem.u16(spare + 6), 1);
        assert_ne!(e.mem.u32(spare + 8), 0);
        assert_eq!(e.mem.u32(spare + 0xc), 0);
        assert_ne!(e.mem.u32(spare + 0x10), 0);
        let given_back: Vec<u32> = calls(&e, MEMORY_MANAGER_DEALLOCATE)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(given_back, vec![0x81, 0x82, 0x83]);
        assert_eq!(e.mem.u32(pool + 8), 0);
        // The new format starts with NiTriShapeData's load and has no lists.
        e.call_log = Some(vec![]);
        let mut input = vec![0];
        input.extend(0u32.to_le_bytes());
        let (s, _) = stream(&mut e, 0x0a00_010f, &input);
        let data = geometry_data(&mut e, &[]);
        e.call(0x00a9_1950, &args![data, s]);
        assert_eq!(calls(&e, TRI_SHAPE_DATA_LOAD_BINARY), vec![vec![data, s]]);
        assert!(calls(&e, ALLOCATE).is_empty());
    }

    #[test]
    fn camera_constructor_sets_up_both_arrays_and_updates() {
        let mut e = engine();
        quiet(&mut e, &[CAMERA_CONSTRUCT, CAMERA_UPDATE_WORLD_BOUND]);
        e.register(POINTER_ARRAY_NEW, |e, a| e.mem.alloc(4 * a[0]).into_ret());
        let seen = Rc::new(RefCell::new(vec![]));
        let s = seen.clone();
        e.register_double(CAMERA_UPDATE_WORLD_DATA, move |e, a| {
            s.borrow_mut().push(e.mem.bytes(a[1], 9));
            Ret::default()
        });
        let camera = e.mem.alloc(0x134);
        assert_eq!(e.call(0x00a9_1d80, &args![camera]).u32(), camera);
        assert_eq!(e.mem.u32(camera), NI_SCREEN_SPACE_CAMERA_VTABLE);
        for (array, vtable) in [
            (CAMERA_POLYGONS, POLYGON_OBJECT_ARRAY_VTABLE),
            (CAMERA_TEXTURES, TEXTURE_OBJECT_ARRAY_VTABLE),
        ] {
            let at = camera + array;
            assert_eq!(e.mem.u32(at), vtable);
            assert_eq!(
                (
                    e.mem.u16(at + 8),
                    e.mem.u16(at + 0xa),
                    e.mem.u16(at + 0xc),
                    e.mem.u16(at + 0xe)
                ),
                (5, 0, 0, 5)
            );
            assert_ne!(e.mem.u32(at + 4), 0);
        }
        assert_eq!(e.mem.u8(camera + 0xf4), 1);
        assert_eq!(*seen.borrow(), vec![vec![0; 9]]);
        assert_eq!(calls(&e, CAMERA_UPDATE_WORLD_BOUND), vec![vec![camera]]);
    }

    #[test]
    fn camera_scalar_destructor_destroys_and_deletes_on_bit_0() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                POINTER_ARRAY_00A2E620,
                POINTER_ARRAY_DELETE,
                CAMERA_DESTRUCT,
            ],
        );
        let camera = object(&mut e, 0x134);
        assert_eq!(e.call(0x00a9_1e60, &args![camera, 0u32]).u32(), camera);
        assert_eq!(calls(&e, CAMERA_DESTRUCT), vec![vec![camera]]);
        assert!(calls(&e, SIZED_DELETE).is_empty());
        e.call(0x00a9_1e60, &args![camera, 1u32]);
        assert_eq!(calls(&e, SIZED_DELETE), vec![vec![camera, 0x134]]);
    }

    #[test]
    fn camera_create_and_clone_construct_a_camera() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                CAMERA_CONSTRUCT,
                CAMERA_UPDATE_WORLD_DATA,
                CAMERA_UPDATE_WORLD_BOUND,
                CAMERA_COPY_MEMBERS,
            ],
        );
        e.register(POINTER_ARRAY_NEW, |e, a| e.mem.alloc(4 * a[0]).into_ret());
        e.register(POINTER_ARRAY_SET_SIZE, |_, _| Ret::default());
        let created = e.call(0x00a9_1ee0, &args![]).u32();
        assert_eq!(e.mem.u32(created), NI_SCREEN_SPACE_CAMERA_VTABLE);
        assert_eq!(calls(&e, NI_NEW), vec![vec![0x134]]);
        let this = object(&mut e, 0x134);
        let clone = e.call(0x00a9_1e90, &args![this, 0x22u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_SCREEN_SPACE_CAMERA_VTABLE);
        assert_eq!(
            calls(&e, CAMERA_COPY_MEMBERS),
            vec![vec![this, clone, 0x22]]
        );
        // (A failed allocation in `CreateClone` makes the member copy write through a null
        // clone, as the game does; that path is not run here.)
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_1ee0, &args![]).u32(), 0);
    }

    #[test]
    fn camera_link_object_appends_each_entry_and_compacts() {
        let mut e = engine();
        e.register(ATEXIT, |_, _| Ret::default());
        quiet(&mut e, &[CAMERA_LINK_OBJECT, POINTER_ARRAY_00A2E620]);
        e.register(POINTER_ARRAY_SET_SIZE, |e, a| {
            let old = e.mem.u32(a[0] + 4);
            let used = u32::from(e.mem.u16(a[0] + 0xa));
            let bytes = e.mem.bytes(old, 4 * used);
            let block = e.mem.alloc(4 * a[1] + 4);
            e.mem.write(block, &bytes);
            e.mem.set_u32(a[0] + 4, block);
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        let camera = object(&mut e, 0x134);
        let a = object(&mut e, 8);
        e.mem.set_u32(a + 4, 1);
        fill_array(&mut e, camera + CAMERA_POLYGONS, &[a], 1);
        fill_array(&mut e, camera + CAMERA_TEXTURES, &[], 0);
        e.call(0x00a9_1f00, &args![camera, 0x33u32]);
        assert_eq!(calls(&e, CAMERA_LINK_OBJECT), vec![vec![camera, 0x33]]);
        let array = camera + CAMERA_POLYGONS;
        let block = e.mem.u32(array + 4);
        assert_eq!((e.mem.u32(block), e.mem.u32(block + 4)), (a, a));
        assert_eq!(e.mem.u16(array + 0xa), 2);
        assert_eq!(e.mem.u32(a + 4), 2);
        assert!(calls(&e, POINTER_ARRAY_SET_SIZE).is_empty());
        assert_eq!(
            calls(&e, POINTER_ARRAY_00A2E620),
            vec![vec![array], vec![camera + CAMERA_TEXTURES]]
        );
    }
}
