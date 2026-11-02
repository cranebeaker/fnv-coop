//! `fallout shared/facegen/bsfacegenmodel.cpp` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00659b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenModel` scalar deleting destructor (Xbox PDB name from the engine map; identical-code folding can share this body): runs the
/// destructor body at `00659b50` and frees 0x10 bytes only when `flags & 1` is
/// set. Returns `this`.
pub fn bs_face_gen_model_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0065_9b50, &args![this]);
    if flags & 1 != 0 {
        e.call(0x00aa_1460, &args![this, 0x10u32]);
    }
    this
}

// ---- callees and data of the functions below ----

/// `NiPointer<T>::get` (`00559450`), `NiPointer<T>::operator=` (`0066b0d0`, one word), the
/// `NiPointer` constructor from a raw pointer (`00633c90`, one word) and destructor
/// (`0045cec0`) (`thiscall`).
const NI_POINTER_GET: u32 = 0x0055_9450;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_FROM_RAW: u32 = 0x0063_3c90;
const NI_POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `operator new` (`00401000`) and `operator delete` (`00401030`) (`cdecl`), the Ni
/// allocation (`00aa13e0`, `cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `BSStringT<char>`: constructor (`004037b0`), destructor (`004037d0`), set (`004037f0`:
/// text, length word) (`thiscall`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;
const STRING_SET: u32 = 0x0040_37f0;
/// The scope timer (`00404eb0`: kind, 1, file, line; `00404ee0` ends it) and the file name
/// it is given (`01065b00`).
const TIMER_SCOPE_CONSTRUCT: u32 = 0x0040_4eb0;
const TIMER_SCOPE_DESTROY: u32 = 0x0040_4ee0;
const FACE_GEN_MODEL_SOURCE_FILE: u32 = 0x0106_5b00;
/// The scalar deleting destructors of the EGM data (`006599d0`) and the EGT data
/// (`00659ab0`) (`thiscall`, one word: the flags).
const EGM_DATA_DELETE: u32 = 0x0065_99d0;
const EGT_DATA_DELETE: u32 = 0x0065_9ab0;
/// `BSFaceGenModel`'s vtable (`01065af4`) and the `NiRefObject` destructor body it ends with
/// (`00496910`).
const FACE_GEN_MODEL_VTABLE: u32 = 0x0106_5af4;
const REF_OBJECT_DESTRUCT: u32 = 0x0049_6910;
/// The byte setting `0043faf0` answers (`cdecl`, no words; it reads the setting at
/// `011d5a44`).
const FACE_GEN_ENABLED: u32 = 0x0043_faf0;
/// `NiGeometryData::SetConsistency` (`00a67050`, `thiscall`, one word).
const SET_CONSISTENCY: u32 = 0x00a6_7050;
/// The tri file data's vectors (`thiscall`): the vertex vector's size (`0065c700`, 12-byte
/// elements), element address (`0065c720`, one word), the morph vector's size (`0065cc30`,
/// 0x38-byte elements) and element address (`00662890`, one word).
const TRI_VERTEX_COUNT: u32 = 0x0065_c700;
const TRI_VERTEX: u32 = 0x0065_c720;
const TRI_MORPH_COUNT: u32 = 0x0065_cc30;
const TRI_MORPH: u32 = 0x0066_2890;
/// The `std::vector` pieces the tri file data's members use (`thiscall`): the other
/// members' constructors (`0065c630`, `0065c750`) and destructors (`0065c6a0`,
/// `0065c7c0`); the allocator constructor (`006815c0`, answers `this`), the vector base's
/// (`0065d3a0`, one word: the allocator by value), the members' `_Buy`s (`0065d1c0`,
/// `0065d2b0`, `0065d3e0`, `0065d4e0`, `0065d5e0`, one word) and `_Tidy`s (`0065d240`,
/// `0065d330`, `0065d460`, `0065d560`, `0065d660`) and the base's destructor (`0065cfb0`).
const VECTOR_CONSTRUCT_0065C630: u32 = 0x0065_c630;
const VECTOR_CONSTRUCT_0065C750: u32 = 0x0065_c750;
const VECTOR_DESTRUCT_0065C6A0: u32 = 0x0065_c6a0;
const VECTOR_DESTRUCT_0065C7C0: u32 = 0x0065_c7c0;
const ALLOCATOR_CONSTRUCT: u32 = 0x0068_15c0;
const VECTOR_BASE_CONSTRUCT: u32 = 0x0065_d3a0;
const VECTOR_BASE_DESTRUCT: u32 = 0x0065_cfb0;
const BUY_0065D1C0: u32 = 0x0065_d1c0;
const BUY_0065D2B0: u32 = 0x0065_d2b0;
const BUY_0065D3E0: u32 = 0x0065_d3e0;
const BUY_0065D4E0: u32 = 0x0065_d4e0;
const BUY_0065D5E0: u32 = 0x0065_d5e0;
const TIDY_0065D240: u32 = 0x0065_d240;
const TIDY_0065D330: u32 = 0x0065_d330;
const TIDY_0065D460: u32 = 0x0065_d460;
const TIDY_0065D560: u32 = 0x0065_d560;
const TIDY_0065D660: u32 = 0x0065_d660;
/// The iterator steps (`0065dae0`, `0065db40`; `thiscall`).
const ITERATOR_STEP_0065DAE0: u32 = 0x0065_dae0;
const ITERATOR_STEP_0065DB40: u32 = 0x0065_db40;
/// The element range destructor (`0065f2b0`, two words) and the allocator's deallocate
/// (`0064df10`, two words) (`thiscall`).
const DESTROY_RANGE: u32 = 0x0065_f2b0;
const DEALLOCATE: u32 = 0x0064_df10;
/// The strided array's bounds check (`004534f0`, `thiscall`, one word).
const STRIDED_CHECK: u32 = 0x0045_34f0;
/// `NiExtraData`'s constructor (`00a7b440`, `thiscall`, one word: the name), the name
/// `00607340` answers (`cdecl`, no words) and `BSFaceGenModelExtraData`'s vtable
/// (`01065bf4`).
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b440;
const EXTRA_DATA_NAME: u32 = 0x0060_7340;
const MODEL_EXTRA_DATA_VTABLE: u32 = 0x0106_5bf4;

layout! {
    /// `BSFaceGenModel` (Xbox PDB), 0x10 bytes (the scalar deleting destructor frees 0x10).
    pub struct BSFaceGenModel: 0x10 {
        /// `pMeshData` (Xbox PDB): `BSFaceGenModelMeshData*`.
        0x08 pMeshData: u32,
        /// `pTextureData` (Xbox PDB): `BSFaceGenModelTextureData*`.
        0x0C pTextureData: u32,
    }

    /// `BSFaceGenModelMeshData` (Xbox PDB), 0x24 bytes (`LoadModelMesh` allocates 0x24).
    pub struct BSFaceGenModelMeshData: 0x24 {
        /// `EGMFilename` (Xbox PDB): a `BSStringT<char>` at +0.
        0x00 EGMFilename: u32,
        /// `pEGMData` (Xbox PDB).
        0x08 pEGMData: u32,
        /// `spModelRoot` (Xbox PDB): `NiPointer<NiNode>`.
        0x0C spModelRoot: u32,
        /// `spModel` (Xbox PDB): `NiPointer<NiTriBasedGeom>`.
        0x10 spModel: u32,
        /// `pAdditionalVertices` (Xbox PDB): `NiPoint3*`.
        0x14 pAdditionalVertices: u32,
        /// `iAdditionalVerticesCount` (Xbox PDB).
        0x18 iAdditionalVerticesCount: u32,
        /// `spMorph` (Xbox PDB): `NiPointer<BSFaceGenMorphData>`.
        0x1C spMorph: u32,
        /// `iRefCount` (Xbox PDB).
        0x20 iRefCount: u32,
    }

    /// `BSFaceGenModelTextureData` (Xbox PDB), 0xc bytes.
    pub struct BSFaceGenModelTextureData: 0x0C {
        /// `EGTFilename` (Xbox PDB): a `BSStringT<char>` at +0.
        0x00 EGTFilename: u32,
        /// `pEGTData` (Xbox PDB).
        0x08 pEGTData: u32,
    }
}

/// `NiPointer<T>::get` on the pointer at `slot`.
fn pointer_get(e: &mut Engine, slot: u32) -> u32 {
    e.call(NI_POINTER_GET, &args![slot]).u32()
}

/// `NiPointer<T>::operator=` on the pointer at `slot`.
fn pointer_assign(e: &mut Engine, slot: u32, object: u32) {
    e.call(NI_POINTER_ASSIGN, &args![slot, object]);
}

/// A member `std::vector`'s constructor as compiled here: an allocator built in a pushed
/// stack word (which holds `this` from the `PUSH ECX`) is handed to the vector base by
/// value, then `_Buy(0)`.
fn vector_construct(e: &mut Engine, this: u32, buy: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        e.mem.set_u32(slot, this);
        e.call(ALLOCATOR_CONSTRUCT, &args![slot]);
        let allocator = e.mem.u32(slot);
        e.call(VECTOR_BASE_CONSTRUCT, &args![this, allocator]);
    });
    e.call(buy, &args![this, 0u32]);
    this
}

/// A member `std::vector`'s destructor: `_Tidy`, then the base's destructor.
fn vector_destruct(e: &mut Engine, this: u32, tidy: u32) {
    e.call(tidy, &args![this]);
    e.call(VECTOR_BASE_DESTRUCT, &args![this]);
}

// Translated from 00659800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenModelMeshData`'s constructor (unnamed in the engine map; `LoadModelMesh`
/// constructs its 0x24-byte allocation with it; `thiscall`, no stack words): the EGM file
/// name constructed, the three `NiPointer`s (`spModelRoot`, `spModel`, `spMorph`)
/// constructed from 0, no EGM data, the pointers assigned 0 (model, root, morph), no
/// additional vertices, reference count 0. Answers `this`.
// Translation incomplete: 00659800 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the members) is not translated.
pub fn fn_00659800(
    e: &mut Engine,
    this: Ptr<BSFaceGenModelMeshData>,
) -> Ptr<BSFaceGenModelMeshData> {
    let base = this.addr();
    e.call(STRING_CONSTRUCT, &args![base]);
    for offset in [0xcu32, 0x10, 0x1c] {
        e.call(NI_POINTER_FROM_RAW, &args![base + offset, 0u32]);
    }
    e.set(this, BSFaceGenModelMeshData::pEGMData, 0);
    for offset in [0x10u32, 0xc, 0x1c] {
        pointer_assign(e, base + offset, 0);
    }
    e.set(this, BSFaceGenModelMeshData::pAdditionalVertices, 0);
    e.set(this, BSFaceGenModelMeshData::iAdditionalVerticesCount, 0);
    e.set(this, BSFaceGenModelMeshData::iRefCount, 0);
    this
}

// Translated from 006598d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenModelMeshData`'s destructor (unnamed in the engine map; `thiscall`, no stack
/// words): the model, root and morph pointers assigned 0, the additional vertices freed
/// (`operator delete`) when set, the EGM data deleted (`006599d0` with 1) when set, the three
/// `NiPointer` destructors (morph, model, root), the file name's destructor.
// Translation incomplete: 006598d0 - the C++ exception frame (FS:[0] chain, unwind states 3 to 0 for the members) is not translated.
pub fn fn_006598d0(e: &mut Engine, this: Ptr<BSFaceGenModelMeshData>) {
    let base = this.addr();
    for offset in [0x10u32, 0xc, 0x1c] {
        pointer_assign(e, base + offset, 0);
    }
    let vertices = e.get(this, BSFaceGenModelMeshData::pAdditionalVertices);
    if vertices != 0 {
        e.call(OPERATOR_DELETE, &args![vertices]);
    }
    let egm = e.get(this, BSFaceGenModelMeshData::pEGMData);
    if egm != 0 {
        e.call(EGM_DATA_DELETE, &args![egm, 1u32]);
    }
    for offset in [0x1cu32, 0x10, 0xc] {
        e.call(NI_POINTER_DESTRUCT, &args![base + offset]);
    }
    e.call(STRING_DESTRUCT, &args![base]);
}

// Translated from 00659a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenModelTextureData`'s constructor (unnamed in the engine map; `thiscall`, no
/// stack words): the EGT file name constructed, no EGT data. Answers `this`.
pub fn fn_00659a00(
    e: &mut Engine,
    this: Ptr<BSFaceGenModelTextureData>,
) -> Ptr<BSFaceGenModelTextureData> {
    e.call(STRING_CONSTRUCT, &args![this]);
    e.set(this, BSFaceGenModelTextureData::pEGTData, 0);
    this
}

// Translated from 00659a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenModelTextureData`'s destructor (unnamed in the engine map; `thiscall`, no stack
/// words): the EGT data deleted (`00659ab0` with 1) when set, then the file name's
/// destructor.
// Translation incomplete: 00659a20 - the C++ exception frame (FS:[0] chain, unwind state 0 for the file name) is not translated.
pub fn fn_00659a20(e: &mut Engine, this: Ptr<BSFaceGenModelTextureData>) {
    let egt = e.get(this, BSFaceGenModelTextureData::pEGTData);
    if egt != 0 {
        e.call(EGT_DATA_DELETE, &args![egt, 1u32]);
    }
    e.call(STRING_DESTRUCT, &args![this]);
}

// Translated from 00659b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenModel`'s destructor body (unnamed in the engine map; the scalar deleting
/// destructor `00659b20` runs it; `thiscall`, no stack words): the vtable, the mesh data
/// deleted (`00659c10` with 1) and the texture data deleted (`00659c40` with 1) when set,
/// then the `NiRefObject` destructor body (`00496910`).
// Translation incomplete: 00659b50 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
pub fn fn_00659b50(e: &mut Engine, this: Ptr<BSFaceGenModel>) {
    e.mem.set_u32(this.addr(), FACE_GEN_MODEL_VTABLE);
    let mesh = e.get(this, BSFaceGenModel::pMeshData);
    if mesh != 0 {
        fn_00659c10(e, Ptr::new(mesh), 1);
    }
    let texture = e.get(this, BSFaceGenModel::pTextureData);
    if texture != 0 {
        fn_00659c40(e, Ptr::new(texture), 1);
    }
    e.call(REF_OBJECT_DESTRUCT, &args![this]);
}

// Translated from 00659c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenModelMeshData`'s scalar deleting destructor (unnamed in the engine map): the
/// destructor (`006598d0`), then `operator delete` when `flags & 1`. Answers `this`.
pub fn fn_00659c10(
    e: &mut Engine,
    this: Ptr<BSFaceGenModelMeshData>,
    flags: u32,
) -> Ptr<BSFaceGenModelMeshData> {
    fn_006598d0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00659c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenModelTextureData`'s scalar deleting destructor (unnamed in the engine map): the
/// destructor (`00659a20`), then `operator delete` when `flags & 1`. Answers `this`.
pub fn fn_00659c40(
    e: &mut Engine,
    this: Ptr<BSFaceGenModelTextureData>,
    flags: u32,
) -> Ptr<BSFaceGenModelTextureData> {
    fn_00659a20(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00659c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, six words: the EGM name, the model path, the tri
/// path, the EGT name, the mesh kind, the keep-properties byte): 0 while face generation is
/// off (`0043faf0`); else `LoadModelMesh` (EGM name, model, tri, kind, byte) or-ed with the
/// texture data load (`0065a760`, EGT name).
pub fn fn_00659c70(
    e: &mut Engine,
    this: Ptr<BSFaceGenModel>,
    egm: u32,
    model: u32,
    tri: u32,
    egt: u32,
    kind: i32,
    keep_properties: u8,
) -> bool {
    if e.call(FACE_GEN_ENABLED, &args![]).u8() == 0 {
        return false;
    }
    let mut loaded =
        bs_face_gen_model_load_model_mesh(e, this, egm, model, tri, kind, keep_properties);
    loaded |= fn_0065a760(e, this, egt);
    loaded
}

/// The model loader global (`011c3b3c`) and `ModelLoader::LoadFile` (`00447080`, six
/// words); `TES::CreateDeepCopySameTextures` (`00457ba0`, two words) on the global at
/// `011dea10`; the cloning process's constructor (`004ad050`, one `float`) and destructor
/// (`004ad270`); "Could not load %s." style message `01065b50` through `005b5e40`.
const MODEL_LOADER: u32 = 0x011c_3b3c;
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080;
const TES_011DEA10: u32 = 0x011d_ea10;
const CREATE_DEEP_COPY_SAME_TEXTURES: u32 = 0x0045_7ba0;
const CLONING_PROCESS_CONSTRUCT: u32 = 0x004a_d050;
const CLONING_PROCESS_DESTRUCT: u32 = 0x004a_d270;
const LOAD_FAILED_MESSAGE: u32 = 0x0106_5b50;
const PRINT_MESSAGE: u32 = 0x005b_5e40;
/// A node's child count (`0043b480`) and child (`0043b4a0`, one word); an object's name
/// (`00413f40`, `+8`) and its text (`0043b1b0`); the text comparison `00404dc0` (`cdecl`:
/// the two texts; 0 when equal) and the names by mesh kind (`0119b734`, one word per kind).
const NODE_CHILD_COUNT: u32 = 0x0043_b480;
const NODE_CHILD: u32 = 0x0043_b4a0;
const OBJECT_NAME: u32 = 0x0041_3f40;
const FIXED_STRING_TEXT: u32 = 0x0043_b1b0;
const COMPARE_TEXT: u32 = 0x0040_4dc0;
const MESH_NAMES: u32 = 0x0119_b734;
/// `NiAVObject::RemoveProperty_ov2` (`00a5b230`, one word) with the property type
/// `004af340` answers (5), `NiObjectNET::RemoveAllControllers` (`00a5c050`), `00a5bfa0`
/// (unit `niobjectnet.cpp`), the skin instance (`0043fad0`, the `NiPointer` at +0xbc) and its
/// root setter (`00437730`, `+0x10`), `NiAVObject::UpdateProperties` (`00a5a040`), the update
/// data constructor (`0043d410`: a `float`, two words) and the update `00a59c60` (one word).
const REMOVE_PROPERTY: u32 = 0x00a5_b230;
const PROPERTY_TYPE_004AF340: u32 = 0x004a_f340;
const REMOVE_ALL_CONTROLLERS: u32 = 0x00a5_c050;
const OBJECT_NET_00A5BFA0: u32 = 0x00a5_bfa0;
const SKIN_INSTANCE: u32 = 0x0043_fad0;
const SKIN_SET_ROOT: u32 = 0x0043_7730;
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
const UPDATE_00A59C60: u32 = 0x00a5_9c60;
/// The node flag setter (`004902f0`, one word).
const NODE_SET_FLAG_004902F0: u32 = 0x0049_02f0;
/// The tri file's stream: constructor from a path (`0044a4c0`, one word) and destructor
/// (`0044a540`); `fffReadTriFile` (`00ab5ec0`, `cdecl`: the stream, the tri data; a byte).
const TRI_STREAM_CONSTRUCT: u32 = 0x0044_a4c0;
const TRI_STREAM_DESTRUCT: u32 = 0x0044_a540;
const READ_TRI_FILE: u32 = 0x00ab_5ec0;
/// A geometry's vertex count (`00456650`, a `u16`) and its data (`005495f0`, the
/// `NiPointer` at +0xb8).
const GEOMETRY_VERTEX_COUNT: u32 = 0x0045_6650;
const GEOMETRY_DATA: u32 = 0x0054_95f0;
/// `BSFaceGenMorphDataHair::BSFaceGenMorphDataHair` (`006621d0`; 0xc bytes) and
/// `BSFaceGenMorphDataHead::BSFaceGenMorphDataHead` (`00660ba0`; 0x18 bytes) (one word: the
/// tri data).
const MORPH_DATA_HAIR_CONSTRUCT: u32 = 0x0066_21d0;
const MORPH_DATA_HEAD_CONSTRUCT: u32 = 0x0066_0ba0;
/// `vector constructor iterator` (`00401050`, `stdcall`) with the trivial element
/// constructor `006815c0`; `NiPoint3::NiPoint3(x, y, z)` (`00416870`).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
const POINT3_CONSTRUCT: u32 = 0x0041_6870;
/// `NiObject::Clone_ov2` (`00a5d680`), `NiObject::CreateDeepCopy` (`00a5d510`, one word:
/// the `NiPointer` to fill), the skin data setter (`004addc0`, one word), the morph extra
/// data constructor (`006624c0`, three words; 0x18 bytes) and
/// `NiObjectNET::AddExtraData_ov2` (`00a5bca0`, one word).
const CLONE: u32 = 0x00a5_d680;
const CREATE_DEEP_COPY: u32 = 0x00a5_d510;
const SKIN_SET_DATA: u32 = 0x004a_ddc0;
const MORPH_EXTRA_DATA_CONSTRUCT: u32 = 0x0066_24c0;
const ADD_EXTRA_DATA: u32 = 0x00a5_bca0;
/// The `NiPointer` at +0xc (`0043b230`; on a skin instance, its skin data).
const SKIN_DATA_0043B230: u32 = 0x0043_b230;

// Translated from 00659ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenModel::LoadModelMesh` (engine map; `thiscall`, five words: the EGM name, the
/// model path, the tri path, the mesh kind, the keep-properties byte). 1 at once when the
/// mesh data exists or there is no EGM name. Otherwise, inside a scope timer (kind 0x38,
/// line 0x11e): new mesh data (0x24 bytes) with the EGM name. With a model path: the model
/// (`ModelLoader::LoadFile(path + 7, 0, 1, 0, 1, 1)`; without it a message and 0) deep
/// copied as `spModelRoot`; `spModel` becomes the first child's geometry (vtable slot
/// +0x1c) that fits the kind (kind 0 with a single child, kind 1 and kind 0 by name from
/// `0119b734`, any other kind the first child); the model loses its controllers and, unless
/// the byte is set, its property of type 5; a skin instance takes the model as its root;
/// flag 1; then the root updates its properties and itself. With a tri path: the tri data
/// (0xf0 bytes) read from the file; when the model's vertex count equals the tri data's
/// (`0065a5a0`), the morph data is a hair one (kind not -1) or a head one, and for a head
/// the tri vertices beyond the model's are kept as `pAdditionalVertices`; the tri data is
/// deleted. Finally the model's data consistency is set to 0 and the answer is 1.
// Translation incomplete: 00659ce0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 7 for the timer, allocations, cloning process and stream) is not translated.
#[allow(clippy::too_many_arguments)]
pub fn bs_face_gen_model_load_model_mesh(
    e: &mut Engine,
    this: Ptr<BSFaceGenModel>,
    egm: u32,
    model: u32,
    tri: u32,
    kind: i32,
    keep_properties: u8,
) -> bool {
    if e.get(this, BSFaceGenModel::pMeshData) != 0 || egm == 0 {
        return true;
    }
    let timer = e.mem.alloc(8);
    e.call(
        TIMER_SCOPE_CONSTRUCT,
        &args![timer, 0x38u32, 1u32, FACE_GEN_MODEL_SOURCE_FILE, 0x11eu32],
    );
    let loaded = load_model_mesh_body(e, this, egm, model, tri, kind, keep_properties);
    e.call(TIMER_SCOPE_DESTROY, &args![timer]);
    e.mem.free(timer);
    loaded
}

fn load_model_mesh_body(
    e: &mut Engine,
    this: Ptr<BSFaceGenModel>,
    egm: u32,
    model: u32,
    tri: u32,
    kind: i32,
    keep_properties: u8,
) -> bool {
    let block = e.call(OPERATOR_NEW, &args![0x24u32]).u32();
    let mesh = if block != 0 {
        fn_00659800(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.set(this, BSFaceGenModel::pMeshData, mesh);
    let mesh = e.get(this, BSFaceGenModel::pMeshData);
    e.call(STRING_SET, &args![mesh, egm, 0u32]);
    if model != 0 {
        let loader = e.global::<u32>(MODEL_LOADER);
        let root = e
            .call(
                MODEL_LOADER_LOAD_FILE,
                &args![loader, model.wrapping_add(7), 0u32, 1u32, 0u32, 1u32, 1u32],
            )
            .u32();
        if root == 0 {
            e.call(PRINT_MESSAGE, &args![LOAD_FAILED_MESSAGE, model]);
            return false;
        }
        load_model(e, this, root, kind, keep_properties);
    }
    if tri != 0 {
        load_tri(e, this, tri, kind);
    }
    let mesh = e.get(this, BSFaceGenModel::pMeshData);
    if mesh != 0 && pointer_get(e, mesh + 0x10) != 0 {
        let geometry = pointer_get(e, mesh + 0x10);
        fn_0065a620(e, geometry, 0);
    }
    true
}

/// `LoadModelMesh`'s model part (`root` is the loaded model).
fn load_model(
    e: &mut Engine,
    this: Ptr<BSFaceGenModel>,
    root: u32,
    kind: i32,
    keep_properties: u8,
) {
    let process = e.mem.alloc(0x20);
    e.call(CLONING_PROCESS_CONSTRUCT, &args![process, 1.0f32.to_bits()]);
    let holder = e.global::<u32>(TES_011DEA10);
    let copy = e
        .call(
            CREATE_DEEP_COPY_SAME_TEXTURES,
            &args![holder, root, process],
        )
        .u32();
    let mesh = |e: &Engine| e.get(this, BSFaceGenModel::pMeshData);
    let data = mesh(e);
    pointer_assign(e, data + 0xc, copy);
    let data = mesh(e);
    if pointer_get(e, data + 0xc) != 0 {
        let data = mesh(e);
        let model_root = pointer_get(e, data + 0xc);
        let count = e.call(NODE_CHILD_COUNT, &args![model_root]).u32();
        let mut index = 0u32;
        while index < count {
            let data = mesh(e);
            let model_root = pointer_get(e, data + 0xc);
            let child = e.call(NODE_CHILD, &args![model_root, index]).u32();
            if child != 0 {
                let take = match kind {
                    0 if count == 1 => true,
                    0 | 1 => {
                        let name = e.call(OBJECT_NAME, &args![child]).u32();
                        let text = e.call(FIXED_STRING_TEXT, &args![name]).u32();
                        let wanted = e
                            .mem
                            .u32(MESH_NAMES.wrapping_add((kind as u32).wrapping_mul(4)));
                        e.call(COMPARE_TEXT, &args![text, wanted]).u32() == 0
                    }
                    _ => true,
                };
                if take {
                    let geometry = e.vcall(child, 0x1c, &args![]).u32();
                    let data = mesh(e);
                    pointer_assign(e, data + 0x10, geometry);
                }
            }
            let data = mesh(e);
            if pointer_get(e, data + 0x10) != 0 {
                break;
            }
            index += 1;
        }
        let data = mesh(e);
        if pointer_get(e, data + 0x10) != 0 {
            if keep_properties == 0 {
                let property = e.call(PROPERTY_TYPE_004AF340, &args![]).u32();
                let data = mesh(e);
                let geometry = pointer_get(e, data + 0x10);
                e.call(REMOVE_PROPERTY, &args![geometry, property]);
            }
            let data = mesh(e);
            let geometry = pointer_get(e, data + 0x10);
            e.call(REMOVE_ALL_CONTROLLERS, &args![geometry]);
            let data = mesh(e);
            let geometry = pointer_get(e, data + 0x10);
            e.call(OBJECT_NET_00A5BFA0, &args![geometry]);
            let data = mesh(e);
            let geometry = pointer_get(e, data + 0x10);
            if e.call(SKIN_INSTANCE, &args![geometry]).u32() != 0 {
                let data = mesh(e);
                let geometry = pointer_get(e, data + 0x10);
                let data = mesh(e);
                let skinned = pointer_get(e, data + 0x10);
                let skin = e.call(SKIN_INSTANCE, &args![skinned]).u32();
                e.call(SKIN_SET_ROOT, &args![skin, geometry]);
            }
            let data = mesh(e);
            let geometry = pointer_get(e, data + 0x10);
            e.call(NODE_SET_FLAG_004902F0, &args![geometry, 1u32]);
        }
        e.call(UPDATE_PROPERTIES, &args![root]);
        e.with_stack(0xc, |e, update| {
            e.call(
                UPDATE_DATA_CONSTRUCT,
                &args![update.addr(), 0.0f32.to_bits(), 0u32, 0u32],
            );
            e.call(UPDATE_00A59C60, &args![root, update.addr()]);
        });
    }
    e.call(CLONING_PROCESS_DESTRUCT, &args![process]);
    e.mem.free(process);
}

/// `LoadModelMesh`'s tri part.
fn load_tri(e: &mut Engine, this: Ptr<BSFaceGenModel>, path: u32, kind: i32) {
    let block = e.call(OPERATOR_NEW, &args![0xf0u32]).u32();
    let tri = if block != 0 {
        fn_0065a4c0(e, Ptr::new(block)).addr()
    } else {
        0
    };
    let stream = e.mem.alloc(0x40);
    e.call(TRI_STREAM_CONSTRUCT, &args![stream, path]);
    let read = e.call(READ_TRI_FILE, &args![stream, tri]).u8() != 0;
    let mesh = |e: &Engine| e.get(this, BSFaceGenModel::pMeshData);
    let matches = read && {
        let data = mesh(e);
        let geometry = pointer_get(e, data + 0x10);
        let vertices = e.call(GEOMETRY_VERTEX_COUNT, &args![geometry]).u16() as u32;
        vertices == fff_tri_file_data_s_num_model_vtx(e, Ptr::new(tri))
    };
    e.call(TRI_STREAM_DESTRUCT, &args![stream]);
    e.mem.free(stream);
    if matches {
        if kind != -1 {
            let block = e.call(NI_ALLOCATE, &args![0xcu32]).u32();
            let morph = if block != 0 {
                e.call(MORPH_DATA_HAIR_CONSTRUCT, &args![block, tri]).u32()
            } else {
                0
            };
            let data = mesh(e);
            pointer_assign(e, data + 0x1c, morph);
        } else {
            let block = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
            let morph = if block != 0 {
                e.call(MORPH_DATA_HEAD_CONSTRUCT, &args![block, tri]).u32()
            } else {
                0
            };
            let data = mesh(e);
            pointer_assign(e, data + 0x1c, morph);
            let total = e.call(TRI_VERTEX_COUNT, &args![tri]).u32();
            let data = mesh(e);
            let geometry = pointer_get(e, data + 0x10);
            let vertices = e.call(GEOMETRY_VERTEX_COUNT, &args![geometry]).u16() as u32;
            if total > vertices {
                keep_additional_vertices(e, this, tri);
            }
        }
    }
    if tri != 0 {
        fn_0065a650(e, Ptr::new(tri), 1);
    }
}

/// The head's additional vertices: the tri vertices past the model's, copied (each through
/// `NiPoint3`) into a new array (`operator new` of 12 bytes each, the size saturating like
/// the compiler's overflow check).
fn keep_additional_vertices(e: &mut Engine, this: Ptr<BSFaceGenModel>, tri: u32) {
    let mesh = |e: &Engine| e.get(this, BSFaceGenModel::pMeshData);
    let total = e.call(TRI_VERTEX_COUNT, &args![tri]).u32();
    let data = mesh(e);
    let geometry = pointer_get(e, data + 0x10);
    let vertices = e.call(GEOMETRY_VERTEX_COUNT, &args![geometry]).u16() as u32;
    let data = mesh(e);
    e.mem.set_u32(data + 0x18, total.wrapping_sub(vertices));
    let data = mesh(e);
    let count = e.mem.u32(data + 0x18);
    let size = count.checked_mul(0xc).unwrap_or(u32::MAX);
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    let array = if block != 0 {
        e.call(
            VECTOR_CONSTRUCT,
            &args![block, 0xcu32, count, ALLOCATOR_CONSTRUCT],
        );
        block
    } else {
        0
    };
    let data = mesh(e);
    e.mem.set_u32(data + 0x14, array);
    let mut index = 0u32;
    loop {
        let data = mesh(e);
        if index >= e.mem.u32(data + 0x18) {
            break;
        }
        let mut words = [0u32; 3];
        for word in [2u32, 1, 0] {
            let first = fff_tri_file_data_s_num_model_vtx(e, Ptr::new(tri));
            let vertex = e
                .call(TRI_VERTEX, &args![tri, first.wrapping_add(index)])
                .u32();
            words[word as usize] = x87_quiet_bits(e.mem.u32(vertex + 4 * word));
        }
        e.with_stack(12, |e, point| {
            let made = e
                .call(
                    POINT3_CONSTRUCT,
                    &args![point.addr(), words[0], words[1], words[2]],
                )
                .u32();
            let data = mesh(e);
            let target = e.mem.u32(data + 0x14).wrapping_add(index.wrapping_mul(0xc));
            for word in 0..3 {
                let value = e.mem.u32(made + 4 * word);
                e.mem.set_u32(target + 4 * word, value);
            }
        });
        index += 1;
    }
}

// Translated from 0065a4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tri file data's constructor (`FffTriFileDataS` by the engine map's name for
/// `0065a5a0`; unnamed here; `thiscall`, no stack words): its ten member vectors constructed
/// (at 0, +0x18, +0x30, +0x48, +0x60, +0x78, +0x90, +0xa8, +0xc0, +0xd8). Answers `this`.
// Translation incomplete: 0065a4c0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 8 for the members) is not translated.
pub fn fn_0065a4c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(VECTOR_CONSTRUCT_0065C630, &args![base]);
    e.call(VECTOR_CONSTRUCT_0065C630, &args![base + 0x18]);
    e.call(VECTOR_CONSTRUCT_0065C750, &args![base + 0x30]);
    fn_0065c820(e, base + 0x48);
    e.call(VECTOR_CONSTRUCT_0065C630, &args![base + 0x60]);
    e.call(VECTOR_CONSTRUCT_0065C750, &args![base + 0x78]);
    fn_0065c8f0(e, base + 0x90);
    fn_0065c9c0(e, base + 0xa8);
    fn_0065ca90(e, base + 0xc0);
    fn_0065cb60(e, base + 0xd8);
    this
}

// Translated from 0065a5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FffTriFileDataS::numModelVtx` (engine map; `thiscall`, no stack words): the vertex
/// vector's size, lowered to the smallest morph's vertex count (`+0x1c` of each 0x38-byte
/// morph in the vector at `+0xd8`).
pub fn fff_tri_file_data_s_num_model_vtx(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let mut count = e.call(TRI_VERTEX_COUNT, &args![base]).u32();
    let mut index = 0u32;
    while index < e.call(TRI_MORPH_COUNT, &args![base + 0xd8]).u32() {
        let morph = e.call(TRI_MORPH, &args![base + 0xd8, index]).u32();
        if e.mem.u32(morph + 0x1c) < count {
            let morph = e.call(TRI_MORPH, &args![base + 0xd8, index]).u32();
            count = e.mem.u32(morph + 0x1c);
        }
        index += 1;
    }
    count
}

// Translated from 0065a620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a geometry, one word): the geometry data's
/// (`+0xb8`) `NiGeometryData::SetConsistency` with the word.
pub fn fn_0065a620(e: &mut Engine, geometry: u32, consistency: u32) {
    let data = pointer_get(e, geometry + 0xb8);
    e.call(SET_CONSISTENCY, &args![data, consistency]);
}

// Translated from 0065a650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tri file data's scalar deleting destructor (unnamed in the engine map): the
/// destructor (`0065a680`), then `operator delete` when `flags & 1`. Answers `this`.
pub fn fn_0065a650(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0065a680(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0065a680 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tri file data's destructor (unnamed in the engine map; `thiscall`, no stack words):
/// the ten member vectors destroyed in reverse order.
// Translation incomplete: 0065a680 - the C++ exception frame (FS:[0] chain, unwind states 8 to 0 for the members) is not translated.
pub fn fn_0065a680(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    fn_0065cbd0(e, base + 0xd8);
    fn_0065cb00(e, base + 0xc0);
    fn_0065ca30(e, base + 0xa8);
    fn_0065c960(e, base + 0x90);
    e.call(VECTOR_DESTRUCT_0065C7C0, &args![base + 0x78]);
    e.call(VECTOR_DESTRUCT_0065C6A0, &args![base + 0x60]);
    fn_0065c890(e, base + 0x48);
    e.call(VECTOR_DESTRUCT_0065C7C0, &args![base + 0x30]);
    e.call(VECTOR_DESTRUCT_0065C6A0, &args![base + 0x18]);
    e.call(VECTOR_DESTRUCT_0065C6A0, &args![base]);
}

// Translated from 0065a760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: the EGT name; the texture half of
/// `00659c70`): 1 at once when the texture data exists or there is no name; otherwise,
/// inside a scope timer (kind 0x39, line 0x1c0), new texture data (0xc bytes) whose file
/// name takes the name; answers 1.
// Translation incomplete: 0065a760 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the timer and the allocation) is not translated.
pub fn fn_0065a760(e: &mut Engine, this: Ptr<BSFaceGenModel>, egt: u32) -> bool {
    if e.get(this, BSFaceGenModel::pTextureData) != 0 || egt == 0 {
        return true;
    }
    let timer = e.mem.alloc(8);
    e.call(
        TIMER_SCOPE_CONSTRUCT,
        &args![timer, 0x39u32, 1u32, FACE_GEN_MODEL_SOURCE_FILE, 0x1c0u32],
    );
    let block = e.call(OPERATOR_NEW, &args![0xcu32]).u32();
    let texture = if block != 0 {
        fn_00659a00(e, Ptr::new(block)).addr()
    } else {
        0
    };
    e.set(this, BSFaceGenModel::pTextureData, texture);
    let texture = e.get(this, BSFaceGenModel::pTextureData);
    e.call(STRING_SET, &args![texture, egt, 0u32]);
    e.call(TIMER_SCOPE_DESTROY, &args![timer]);
    e.mem.free(timer);
    true
}

// Translated from 0065a910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the face generation coordinates, the
/// `NiPointer` that receives the new mesh): 0 without coordinates; else the new mesh
/// (`CreateNewMesh`), then the morphs applied to it (`0065a970` with the coordinates, the
/// mesh and 1); when that fails the pointer is cleared and the answer is 0.
pub fn fn_0065a910(e: &mut Engine, this: Ptr<BSFaceGenModel>, coordinates: u32, mesh: u32) -> bool {
    if coordinates == 0 {
        return false;
    }
    if !bs_face_gen_model_create_new_mesh(e, this, mesh) {
        return false;
    }
    let geometry = pointer_get(e, mesh);
    if !fn_0065a970(e, this, coordinates, geometry, 1) {
        pointer_assign(e, mesh, 0);
        return false;
    }
    true
}

// Translated from 0065b390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a strided array, two words: the index, a
/// point): the bounds check `004534f0`, then the three words of the point stored at
/// `base + index * stride` (base at +0, stride at +4).
pub fn fn_0065b390(e: &mut Engine, this: u32, index: u32, point: u32) {
    e.call(STRIDED_CHECK, &args![this, index]);
    let stride = e.mem.u32(this + 4);
    let target = index.wrapping_mul(stride).wrapping_add(e.mem.u32(this));
    for word in 0..3 {
        let value = e.mem.u32(point + 4 * word);
        e.mem.set_u32(target + 4 * word, value);
    }
}

// Translated from 0065b3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one byte): the word at `+0x14` when the byte is
/// set, else the word at `+0x10`.
pub fn fn_0065b3e0(e: &mut Engine, this: u32, second: bool) -> u32 {
    if second {
        e.mem.u32(this + 0x14)
    } else {
        e.mem.u32(this + 0x10)
    }
}

// Translated from 0065bbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenModel::CreateNewMesh` (engine map; `thiscall`, one word: the `NiPointer` that
/// receives the mesh). The pointer is cleared; 0 without a model or without its geometry
/// data. Otherwise the model is cloned (`NiObject::Clone_ov2`) into the pointer, its data is
/// deep copied and set as the clone's data (vtable slot +0xe4); a skin instance with skin
/// data gets a deep copy of it (`004addc0`); the clone gets the morph extra data (0x18
/// bytes, `006624c0` with the additional vertices and their count; when allocated) and a
/// `BSFaceGenModelExtraData` (0x10 bytes) naming this model. Answers 1.
// Translation incomplete: 0065bbc0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 5 for the local pointers and the allocations) is not translated.
pub fn bs_face_gen_model_create_new_mesh(
    e: &mut Engine,
    this: Ptr<BSFaceGenModel>,
    mesh: u32,
) -> bool {
    let locals = e.mem.alloc(0x10);
    let (data_copy, skin_copy, deep_data, deep_skin) =
        (locals, locals + 4, locals + 8, locals + 0xc);
    e.call(NI_POINTER_FROM_RAW, &args![data_copy, 0u32]);
    e.call(NI_POINTER_FROM_RAW, &args![skin_copy, 0u32]);
    pointer_assign(e, mesh, 0);
    let model_data = e.get(this, BSFaceGenModel::pMeshData);
    let usable = model_data != 0 && pointer_get(e, model_data + 0x10) != 0 && {
        let model_data = e.get(this, BSFaceGenModel::pMeshData);
        let model = pointer_get(e, model_data + 0x10);
        e.call(GEOMETRY_DATA, &args![model]).u32() != 0
    };
    if !usable {
        e.call(NI_POINTER_DESTRUCT, &args![skin_copy]);
        e.call(NI_POINTER_DESTRUCT, &args![data_copy]);
        e.mem.free(locals);
        return false;
    }
    let model_data = e.get(this, BSFaceGenModel::pMeshData);
    let model = pointer_get(e, model_data + 0x10);
    let clone = e.call(CLONE, &args![model]).u32();
    pointer_assign(e, mesh, clone);
    e.call(NI_POINTER_FROM_RAW, &args![deep_data, 0u32]);
    let model_data = e.get(this, BSFaceGenModel::pMeshData);
    let model = pointer_get(e, model_data + 0x10);
    let data = e.call(GEOMETRY_DATA, &args![model]).u32();
    e.call(CREATE_DEEP_COPY, &args![data, deep_data]);
    let copied = pointer_get(e, deep_data);
    pointer_assign(e, data_copy, copied);
    let target = pointer_get(e, mesh);
    let copied = pointer_get(e, data_copy);
    e.vcall(target, 0xe4, &args![copied]);
    let target = pointer_get(e, mesh);
    let skin = e.call(SKIN_INSTANCE, &args![target]).u32();
    if skin != 0 && e.call(SKIN_DATA_0043B230, &args![skin]).u32() != 0 {
        e.call(NI_POINTER_FROM_RAW, &args![deep_skin, 0u32]);
        let skin_data = e.call(SKIN_DATA_0043B230, &args![skin]).u32();
        e.call(CREATE_DEEP_COPY, &args![skin_data, deep_skin]);
        let copied = pointer_get(e, deep_skin);
        pointer_assign(e, skin_copy, copied);
        let copied = pointer_get(e, skin_copy);
        e.call(SKIN_SET_DATA, &args![skin, copied]);
        e.call(NI_POINTER_DESTRUCT, &args![deep_skin]);
    }
    let block = e.call(NI_ALLOCATE, &args![0x18u32]).u32();
    let morph = if block != 0 {
        let model_data = e.get(this, BSFaceGenModel::pMeshData);
        let count = e.mem.u32(model_data + 0x18);
        let model_data = e.get(this, BSFaceGenModel::pMeshData);
        let vertices = e.mem.u32(model_data + 0x14);
        let target = pointer_get(e, mesh);
        e.call(
            MORPH_EXTRA_DATA_CONSTRUCT,
            &args![block, target, vertices, count],
        )
        .u32()
    } else {
        0
    };
    if morph != 0 {
        let target = pointer_get(e, mesh);
        e.call(ADD_EXTRA_DATA, &args![target, morph]);
    }
    let block = e.call(NI_ALLOCATE, &args![0x10u32]).u32();
    let extra = if block != 0 {
        fn_0065be60(e, Ptr::new(block), this.addr()).addr()
    } else {
        0
    };
    let target = pointer_get(e, mesh);
    e.call(ADD_EXTRA_DATA, &args![target, extra]);
    e.call(NI_POINTER_DESTRUCT, &args![deep_data]);
    e.call(NI_POINTER_DESTRUCT, &args![skin_copy]);
    e.call(NI_POINTER_DESTRUCT, &args![data_copy]);
    e.mem.free(locals);
    true
}

// Translated from 0065be60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenModelExtraData`'s constructor (by the vtable it stores; unnamed in the engine
/// map; `thiscall`, one word: the model): `NiExtraData` named by `00607340`, the vtable,
/// `spModel` constructed from 0 and assigned the model. Answers `this`.
// Translation incomplete: 0065be60 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and spModel) is not translated.
pub fn fn_0065be60(e: &mut Engine, this: Ptr, model: u32) -> Ptr {
    let base = this.addr();
    let name = e.call(EXTRA_DATA_NAME, &args![]).u32();
    e.call(EXTRA_DATA_CONSTRUCT, &args![base, name]);
    e.mem.set_u32(base, MODEL_EXTRA_DATA_VTABLE);
    e.call(NI_POINTER_FROM_RAW, &args![base + 0xc, 0u32]);
    pointer_assign(e, base + 0xc, model);
    this
}

// Translated from 0065c820 (decompiled, FalloutNV.exe 1.4.0.525)
/// A tri file data member vector's constructor (unnamed in the engine map; the one at
/// +0x48; `thiscall`, no stack words): the vector base with a stack-built allocator, then
/// `_Buy(0)` (`0065d1c0`). Answers `this`.
// Translation incomplete: 0065c820 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_0065c820(e: &mut Engine, this: u32) -> u32 {
    vector_construct(e, this, BUY_0065D1C0)
}

// Translated from 0065c890 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the vector `0065c820` makes (unnamed in the engine map): `_Tidy`
/// (`0065d240`), then the vector base's destructor (`0065cfb0`).
// Translation incomplete: 0065c890 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_0065c890(e: &mut Engine, this: u32) {
    vector_destruct(e, this, TIDY_0065D240);
}

// Translated from 0065c8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the member vector at +0x90 (unnamed in the engine map): the vector
/// base, then `_Buy(0)` (`0065d2b0`). Answers `this`.
// Translation incomplete: 0065c8f0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_0065c8f0(e: &mut Engine, this: u32) -> u32 {
    vector_construct(e, this, BUY_0065D2B0)
}

// Translated from 0065c960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the vector `0065c8f0` makes (unnamed in the engine map): `_Tidy`
/// (`0065d330`), then the base's destructor.
// Translation incomplete: 0065c960 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_0065c960(e: &mut Engine, this: u32) {
    vector_destruct(e, this, TIDY_0065D330);
}

// Translated from 0065c9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the member vector at +0xa8 (unnamed in the engine map): the vector
/// base, then `_Buy(0)` (`0065d3e0`). Answers `this`.
// Translation incomplete: 0065c9c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_0065c9c0(e: &mut Engine, this: u32) -> u32 {
    vector_construct(e, this, BUY_0065D3E0)
}

// Translated from 0065ca30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the vector `0065c9c0` makes (unnamed in the engine map): `_Tidy`
/// (`0065d460`), then the base's destructor.
// Translation incomplete: 0065ca30 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_0065ca30(e: &mut Engine, this: u32) {
    vector_destruct(e, this, TIDY_0065D460);
}

// Translated from 0065ca90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the member vector at +0xc0 (unnamed in the engine map): the vector
/// base, then `_Buy(0)` (`0065d4e0`). Answers `this`.
// Translation incomplete: 0065ca90 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_0065ca90(e: &mut Engine, this: u32) -> u32 {
    vector_construct(e, this, BUY_0065D4E0)
}

// Translated from 0065cb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the vector `0065ca90` makes (unnamed in the engine map): `_Tidy`
/// (`0065d560`), then the base's destructor.
// Translation incomplete: 0065cb00 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_0065cb00(e: &mut Engine, this: u32) {
    vector_destruct(e, this, TIDY_0065D560);
}

// Translated from 0065cb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the member vector at +0xd8 (the morphs; unnamed in the engine map):
/// the vector base, then `_Buy(0)` (`0065d5e0`). Answers `this`.
// Translation incomplete: 0065cb60 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_0065cb60(e: &mut Engine, this: u32) -> u32 {
    vector_construct(e, this, BUY_0065D5E0)
}

// Translated from 0065cbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the vector `0065cb60` makes (unnamed in the engine map): `_Tidy`
/// (`0065d660`), then the base's destructor.
// Translation incomplete: 0065cbd0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_0065cbd0(e: &mut Engine, this: u32) {
    vector_destruct(e, this, TIDY_0065D660);
}

// Translated from 0065cf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// An iterator's post-increment (unnamed in the engine map; `thiscall`, two words: the
/// result, an unread word): the iterator's two words are kept, the iterator stepped
/// (`0065dae0`), and the kept words written to the result, which is answered.
pub fn fn_0065cf50(e: &mut Engine, this: u32, result: u32, _unused_2: u32) -> u32 {
    let first = e.mem.u32(this);
    let second = e.mem.u32(this + 4);
    e.call(ITERATOR_STEP_0065DAE0, &args![this]);
    e.mem.set_u32(result, first);
    e.mem.set_u32(result + 4, second);
    result
}

// Translated from 0065cf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// An iterator's pre-increment (unnamed in the engine map; `thiscall`): `0065db40`, then
/// `this`.
pub fn fn_0065cf90(e: &mut Engine, this: u32) -> u32 {
    e.call(ITERATOR_STEP_0065DB40, &args![this]);
    this
}

// Translated from 0065f240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<unsigned long>::_Tidy` (engine map; `thiscall`, no stack words): with a
/// buffer (`+0xc`), its elements destroyed (`0065f2b0` from `+0xc` to `+0x10`) and the
/// buffer handed back to the allocator (`+8`, `0064df10` with the capacity `(+0x14 - +0xc)
/// / 4`); then the three pointers cleared.
pub fn std_vector_unsigned_long_tidy(e: &mut Engine, this: u32) {
    if e.mem.u32(this + 0xc) != 0 {
        let first = e.mem.u32(this + 0xc);
        let last = e.mem.u32(this + 0x10);
        e.call(DESTROY_RANGE, &args![this, first, last]);
        let first = e.mem.u32(this + 0xc);
        let capacity = (e.mem.u32(this + 0x14).wrapping_sub(first) as i32) >> 2;
        e.call(DEALLOCATE, &args![this + 8, first, capacity as u32]);
    }
    e.mem.set_u32(this + 0xc, 0);
    e.mem.set_u32(this + 0x10, 0);
    e.mem.set_u32(this + 0x14, 0);
}

/// `BSFaceGenManager::GetFaceGenCoord` (`006521e0`, no words), the face generation lock and
/// unlock (`00652140`, `00652190`, no words) and the two "already loaded" tests
/// (`00651f70` for the mesh, `00652040` for the texture; `cdecl`, the model; a byte).
const GET_FACE_GEN_COORD: u32 = 0x0065_21e0;
const FACE_GEN_LOCK: u32 = 0x0065_2140;
const FACE_GEN_UNLOCK: u32 = 0x0065_2190;
const MESH_LOADED_TEST: u32 = 0x0065_1f70;
const TEXTURE_LOADED_TEST: u32 = 0x0065_2040;
/// A `BSStringT`'s text test (`004048e0`, `thiscall`), the EGM data's constructor
/// (`00659560`, `thiscall`: the file name; 0x34 bytes) and the EGT data's loader
/// (`00659780`, `cdecl`: the file name).
const STRING_HAS_TEXT: u32 = 0x0040_48e0;
const EGM_DATA_CONSTRUCT: u32 = 0x0065_9560;
const EGT_DATA_LOAD: u32 = 0x0065_9780;
/// The morph extra data of a geometry (`00653430`, `cdecl`) and its additional vertices
/// (`0084e3a0`, `+0xc`) and count (`0065b3e0` with 0, `+0x10`).
const MORPH_EXTRA_DATA: u32 = 0x0065_3430;
const EXTRA_VERTICES: u32 = 0x0084_e3a0;
/// The strided vertex iterator: constructor (`00534550`), element read (`00534570`: index,
/// the point to fill); the geometry data's lock (`00a67360`, one word), its vertex iterator
/// (`00a67410`: the iterator to fill), unlock (`00a673d0`), `NiGeometryData::MarkAsChanged`
/// (`00a67090`, one word) and its keep flags (`00457b80`); the geometry's `0049ec60`; the
/// iterator's stride word (`00717e50`).
const VERTEX_ITERATOR_CONSTRUCT: u32 = 0x0053_4550;
const VERTEX_ITERATOR_GET: u32 = 0x0053_4570;
const GEOMETRY_DATA_LOCK: u32 = 0x00a6_7360;
const GEOMETRY_DATA_VERTICES: u32 = 0x00a6_7410;
const GEOMETRY_DATA_UNLOCK: u32 = 0x00a6_73d0;
const GEOMETRY_DATA_MARK_AS_CHANGED: u32 = 0x00a6_7090;
const GEOMETRY_DATA_KEEP_FLAGS: u32 = 0x0045_7b80;
const GEOMETRY_0049EC60: u32 = 0x0049_ec60;
const ITERATOR_STRIDE_00717E50: u32 = 0x0071_7e50;
/// The EGM data's morph sets (two, `+4 + 0x18 * set`): their size (`00657c00`) and morph
/// (`00657c20`, one word); a morph's vertex count (`00657bc0` on `morph + 4`) and the
/// iterator over its vertex offsets (`0044a170` on `morph + 4`: the iterator to fill;
/// `0064dda0` reads the three `i16`s at the iterator).
const MORPH_SET_SIZE: u32 = 0x0065_7c00;
const MORPH_SET_MORPH: u32 = 0x0065_7c20;
const MORPH_VERTEX_COUNT: u32 = 0x0065_7bc0;
const VECTOR_BEGIN: u32 = 0x0044_a170;
const ITERATOR_DEREF: u32 = 0x0064_dda0;
/// The face generation coordinates (`0060b370` on `coords + 32 * set`, two words: a result
/// local, the morph index; `00629ab0` with 0 answers the coefficient's address).
const COORD_VALUE: u32 = 0x0060_b370;
const COORD_ADDRESS: u32 = 0x0062_9ab0;
/// `0.0` (`double`, `01012060`).
const ZERO: u32 = 0x0101_2060;
/// `BSFaceGenManager::GetDefaultBaseModulationTexture` (`006045f0`).
const DEFAULT_MODULATION_TEXTURE: u32 = 0x0060_45f0;
/// The EGT data's morph set (`+4 + 0x18 * set`): size (`0065cea0`), morph (`0065cec0`, one
/// word); a morph's channel data (`0065cc70` on `morph + 0x10 + 0x18 * channel`, one word).
const EGT_SET_SIZE: u32 = 0x0065_cea0;
const EGT_SET_MORPH: u32 = 0x0065_cec0;
const EGT_CHANNEL_DATA: u32 = 0x0065_cc70;
/// The coordinates' texture morph count (`009611e0` on `coords + 0x40 + 32 * set`).
const COORD_TEXTURE_COUNT: u32 = 0x0096_11e0;
/// "texture coordinates" message (`01065ba0`) and `fr2Assert` (`00aafb90`, `cdecl`: file,
/// line).
const TEXTURE_COUNT_MESSAGE: u32 = 0x0106_5ba0;
const FR2_ASSERT: u32 = 0x00aa_fb90;
/// The image (`0064ccf0`, one word; 0x2c bytes), its width and height setters (`007037c0`,
/// `0061cc40`), its colour vector (`0064d880`) and resize (`0064db50`), the vector's first
/// element (`0064dbd0` on an iterator) and `BSFaceGenImage::GetCompressedImage` (`0064ceb0`,
/// four words).
const IMAGE_CONSTRUCT: u32 = 0x0064_ccf0;
const IMAGE_SET_WIDTH: u32 = 0x0070_37c0;
const IMAGE_SET_HEIGHT: u32 = 0x0061_cc40;
const IMAGE_COLORS: u32 = 0x0064_d880;
const IMAGE_RESIZE: u32 = 0x0064_db50;
const ITERATOR_ELEMENT: u32 = 0x0064_dbd0;
const GET_COMPRESSED_IMAGE: u32 = 0x0064_ceb0;
/// The `float`s and the format record `GetCompressedImage` is given (`01065b98`,
/// `01023cd8`, `01016248`, `011aa2a0`), the colour scale (`01065b9c`) and the coefficient
/// scale (`010231d8`, `double`).
const IMAGE_SCALE_01065B98: u32 = 0x0106_5b98;
const IMAGE_SCALE_01023CD8: u32 = 0x0102_3cd8;
const IMAGE_SCALE_01016248: u32 = 0x0101_6248;
const IMAGE_FORMAT_011AA2A0: u32 = 0x011a_a2a0;
const COLOR_SCALE_01065B9C: u32 = 0x0106_5b9c;
const COEFFICIENT_SCALE: u32 = 0x0102_31d8;
/// `memset` (`00403d30`, `cdecl`) and `_ftol2` (`00ec62c0`; the value in `ST0` passed as an
/// `f64`).
const MEMSET: u32 = 0x0040_3d30;
const FTOL: u32 = 0x00ec_62c0;

/// The weight a coordinate gives morph `index` of set `set` (`coefficient * morph scale`,
/// rounded to a `float`).
fn morph_weight(e: &mut Engine, coords: u32, set: u32, set_address: u32, index: u32) -> u32 {
    let value = e.with_stack(0x10, |e, local| {
        let holder = e
            .call(
                COORD_VALUE,
                &args![coords.wrapping_add(set << 5), local.addr(), index],
            )
            .u32();
        e.call(COORD_ADDRESS, &args![holder, 0u32]).u32()
    });
    let morph = e.call(MORPH_SET_MORPH, &args![set_address, index]).u32();
    x87_store32(x87_mul(
        x87_m32(e.mem.u32(value)),
        x87_m32(e.mem.u32(morph)),
    ))
}

/// One offset component times the weight added to a coordinate: `FILD` of the `i16`,
/// `FMUL` of the weight, `FADD` of the coordinate, `FSTP`.
fn morphed(offset: i16, weight: u32, coordinate: u32) -> u32 {
    x87_store32(x87_add(
        x87_mul(f64::from(offset as i32), x87_m32(weight)),
        x87_m32(coordinate),
    ))
}

/// The three `i16`s at the morph iterator.
fn offset_at(e: &mut Engine, iterator: u32) -> [i16; 3] {
    let offset = e.call(ITERATOR_DEREF, &args![iterator]).u32();
    [
        e.mem.i16(offset),
        e.mem.i16(offset + 2),
        e.mem.i16(offset + 4),
    ]
}

/// The morph iterator's post-increment (`0065cf50` into a dropped local).
fn advance(e: &mut Engine, iterator: u32) {
    e.with_stack(8, |e, old| {
        fn_0065cf50(e, iterator, old.addr(), 0);
    });
}

// Translated from 0065a970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words: the face generation coordinates (the
/// default ones from `GetFaceGenCoord` when 0), the geometry, the apply-all byte): applies
/// the EGM morphs to the geometry's vertices. 0 without mesh data or coordinates. Under the
/// face generation lock: the EGM data is loaded from its file when not yet (0 without a file
/// name or data); the geometry's vertices are locked and iterated (0 without them); the
/// vertex count is lowered to the smallest morph's. For both morph sets and each morph whose
/// weight (coefficient times the morph's scale) is not 0, or for every morph with the byte
/// set: each vertex (four at a time, then the rest) moves by the morph's `i16` offsets times
/// the weight, read from and written back to the additional vertices (when the geometry has
/// morph extra data) or the geometry; past the vertex count the additional vertices up to
/// their count move too. Then the data is marked as changed unless its keep flags are
/// 0x4000 or the geometry's `0049ec60` is 0, the vertices are unlocked, and the answer is
/// 1.
// Translation incomplete: 0065a970 - the C++ exception frame (FS:[0] chain, unwind state 0 for the EGM data allocation) is not translated.
pub fn fn_0065a970(
    e: &mut Engine,
    this: Ptr<BSFaceGenModel>,
    coordinates: u32,
    geometry: u32,
    apply_all: u8,
) -> bool {
    if e.get(this, BSFaceGenModel::pMeshData) == 0 {
        return false;
    }
    let coords = if coordinates == 0 {
        e.call(GET_FACE_GEN_COORD, &args![]).u32()
    } else {
        coordinates
    };
    if coords == 0 {
        return false;
    }
    e.call(FACE_GEN_LOCK, &args![]);
    let mesh = |e: &Engine| e.get(this, BSFaceGenModel::pMeshData);
    if e.call(MESH_LOADED_TEST, &args![this]).u8() == 0 {
        let data = mesh(e);
        if e.mem.u32(data + 8) == 0 {
            let data = mesh(e);
            if e.call(STRING_HAS_TEXT, &args![data]).u32() == 0 {
                e.call(FACE_GEN_UNLOCK, &args![]);
                return false;
            }
            let block = e.call(OPERATOR_NEW, &args![0x34u32]).u32();
            let egm = if block != 0 {
                let data = mesh(e);
                let name = pointer_get(e, data);
                e.call(EGM_DATA_CONSTRUCT, &args![block, name]).u32()
            } else {
                0
            };
            let data = mesh(e);
            e.mem.set_u32(data + 8, egm);
        }
        let data = mesh(e);
        if e.mem.u32(data + 8) == 0 {
            e.call(FACE_GEN_UNLOCK, &args![]);
            return false;
        }
    }
    let extra_data = e.call(MORPH_EXTRA_DATA, &args![geometry]).u32();
    let extra = if extra_data != 0 {
        e.call(EXTRA_VERTICES, &args![extra_data]).u32()
    } else {
        0
    };
    let locals = e.mem.alloc(0x10);
    let iterator = locals;
    e.call(VERTEX_ITERATOR_CONSTRUCT, &args![iterator]);
    let mut vertices_base = 0u32;
    if geometry != 0 {
        let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
        if data != 0 && e.call(GEOMETRY_DATA_LOCK, &args![data, 1u32]).u8() != 0 {
            e.call(GEOMETRY_DATA_VERTICES, &args![data, iterator]);
            // The word the iterator answers is kept in a local nothing reads.
            let word = e.call(ITERATOR_STRIDE_00717E50, &args![iterator]).u32();
            let _stride = e.mem.u32(word);
            vertices_base = pointer_get(e, iterator);
        }
    }
    if vertices_base == 0 {
        e.call(FACE_GEN_UNLOCK, &args![]);
        e.mem.free(locals);
        return false;
    }
    let egm_set = |e: &Engine, set: u32| {
        let data = e.get(this, BSFaceGenModel::pMeshData);
        e.mem.u32(data + 8).wrapping_add(set * 0x18).wrapping_add(4)
    };
    let mut count = e.call(GEOMETRY_VERTEX_COUNT, &args![geometry]).u16() as u32;
    for set in 0..2u32 {
        let address = egm_set(e, set);
        let morphs = e.call(MORPH_SET_SIZE, &args![address]).u32();
        for index in 0..morphs {
            let address = egm_set(e, set);
            let morph = e.call(MORPH_SET_MORPH, &args![address, index]).u32();
            if e.call(MORPH_VERTEX_COUNT, &args![morph + 4]).u32() < count {
                let address = egm_set(e, set);
                let morph = e.call(MORPH_SET_MORPH, &args![address, index]).u32();
                count = e.call(MORPH_VERTEX_COUNT, &args![morph + 4]).u32();
            }
        }
    }
    let checking = face_check::enabled();
    if checking {
        // Diagnostic only (`NV_FACE_CHECK`, not exe behaviour).
        let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
        face_check::morph_start(
            e,
            this.addr(),
            coords,
            geometry,
            data,
            apply_all,
            count,
            iterator,
            extra,
        );
    }
    let zero = e.global::<f64>(ZERO);
    for set in 0..2u32 {
        let address = egm_set(e, set);
        let morphs = e.call(MORPH_SET_SIZE, &args![address]).u32();
        for index in 0..morphs {
            let address = egm_set(e, set);
            let weight = morph_weight(e, coords, set, address, index);
            // FCOMP with 0.0, TEST AH,0x44, JP: applied unless equal (NaN applies).
            if x87_m32(weight) == zero && apply_all == 0 {
                continue;
            }
            apply_morph(
                e, this, set, index, weight, iterator, extra, extra_data, count,
            );
        }
    }
    if checking {
        face_check::morph_end(e, geometry, iterator);
    }
    if e.call(GEOMETRY_0049EC60, &args![geometry]).u32() != 0 {
        let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
        if e.call(GEOMETRY_DATA_KEEP_FLAGS, &args![data]).u32() != 0x4000 {
            let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
            e.call(GEOMETRY_DATA_MARK_AS_CHANGED, &args![data, 1u32]);
        }
    }
    let data = e.call(GEOMETRY_DATA, &args![geometry]).u32();
    e.call(GEOMETRY_DATA_UNLOCK, &args![data]);
    e.call(FACE_GEN_UNLOCK, &args![]);
    e.mem.free(locals);
    true
}

/// One morph of `0065a970` applied (see there).
#[allow(clippy::too_many_arguments)]
fn apply_morph(
    e: &mut Engine,
    this: Ptr<BSFaceGenModel>,
    set: u32,
    index: u32,
    weight: u32,
    iterator: u32,
    first_extra: u32,
    extra_data: u32,
    count: u32,
) {
    let mut extra = first_extra;
    let data = e.get(this, BSFaceGenModel::pMeshData);
    let address = e.mem.u32(data + 8).wrapping_add(set * 0x18).wrapping_add(4);
    let morph = e.call(MORPH_SET_MORPH, &args![address, index]).u32();
    let offsets = e.mem.alloc(8);
    e.with_stack(8, |e, begin| {
        let first = e.call(VECTOR_BEGIN, &args![morph + 4, begin.addr()]).u32();
        let words = [e.mem.u32(first), e.mem.u32(first + 4)];
        e.mem.set_u32(offsets, words[0]);
        e.mem.set_u32(offsets + 4, words[1]);
    });
    let points = e.mem.alloc(0x40);
    let mut vertex = 0u32;
    while vertex.wrapping_add(3) < count {
        for point in 0..4 {
            e.call(ALLOCATOR_CONSTRUCT, &args![points + 0xc * point]);
        }
        if extra != 0 {
            for word in 0..12 {
                let value = e.mem.u32(extra + 4 * word);
                e.mem.set_u32(points + 4 * word, value);
            }
        } else {
            for point in 0..4 {
                e.call(
                    VERTEX_ITERATOR_GET,
                    &args![iterator, vertex + point, points + 0xc * point],
                );
            }
        }
        let mut deltas = [[0i16; 3]; 4];
        for delta in deltas.iter_mut() {
            *delta = offset_at(e, offsets);
            advance(e, offsets);
        }
        for (point, delta) in deltas.iter().enumerate() {
            for axis in 0..3 {
                let address = points + 0xc * point as u32 + 4 * axis as u32;
                let value = morphed(delta[axis], weight, e.mem.u32(address));
                e.mem.set_u32(address, value);
            }
        }
        for point in 0..4 {
            fn_0065b390(e, iterator, vertex + point, points + 0xc * point);
        }
        if extra != 0 {
            for word in 0..12 {
                let value = e.mem.u32(points + 4 * word);
                e.mem.set_u32(extra + 4 * word, value);
            }
            extra = extra.wrapping_add(0x30);
        }
        vertex = vertex.wrapping_add(4);
    }
    while vertex < count {
        e.call(ALLOCATOR_CONSTRUCT, &args![points]);
        if extra != 0 {
            for word in 0..3 {
                let value = e.mem.u32(extra + 4 * word);
                e.mem.set_u32(points + 4 * word, value);
            }
        } else {
            e.call(VERTEX_ITERATOR_GET, &args![iterator, vertex, points]);
        }
        for axis in 0..3u32 {
            let offset = e.call(ITERATOR_DEREF, &args![offsets]).u32();
            let delta = e.mem.i16(offset + 2 * axis);
            let value = morphed(delta, weight, e.mem.u32(points + 4 * axis));
            e.mem.set_u32(points + 4 * axis, value);
        }
        fn_0065b390(e, iterator, vertex, points);
        if extra != 0 {
            for word in 0..3 {
                let value = e.mem.u32(points + 4 * word);
                e.mem.set_u32(extra + 4 * word, value);
            }
            extra = extra.wrapping_add(0xc);
        }
        vertex = vertex.wrapping_add(1);
        advance(e, offsets);
    }
    if extra != 0 {
        let total = fn_0065b3e0(e, extra_data, false);
        let mut vertex = count;
        while vertex < total {
            for axis in 0..3u32 {
                let offset = e.call(ITERATOR_DEREF, &args![offsets]).u32();
                let delta = e.mem.i16(offset + 2 * axis);
                let address = extra + 4 * axis;
                let value = morphed(delta, weight, e.mem.u32(address));
                e.mem.set_u32(address, value);
            }
            extra = extra.wrapping_add(0xc);
            vertex = vertex.wrapping_add(1);
            advance(e, offsets);
        }
    }
    e.mem.free(points);
    e.mem.free(offsets);
}

// Translated from 0065b410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSFaceGenModel::ApplyCoordinateTexturingToMesh` (engine map; `thiscall`, three words:
/// the face generation coordinates (the default ones when 0), the `NiPointer` that receives
/// the texture, the morph limit (0 for all)). The pointer first takes the default base
/// modulation texture; 0 without texture data or coordinates. Under the face generation
/// lock the EGT data is loaded from its file when not yet (0 without a name or data); 0
/// when its first morph has no width or height, or when the coordinates have fewer texture
/// morphs than either EGT set (with a message). An image (0x2c bytes) of the first morph's
/// width and height gets its colours (zero) resized; an `i32` accumulator per texel channel
/// sums, for the first set's morphs up to the limit, each morph's signed byte deltas times
/// `ftol(scale * 256) * ftol(coefficient * 256)` (`fr2Assert` when a morph's size differs);
/// each colour channel becomes the sum times `01065b9c`; the accumulator is freed and the
/// pointer takes `GetCompressedImage` of the image. Answers 1.
// Translation incomplete: 0065b410 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the local NiPointer and the image allocation) is not translated.
pub fn bs_face_gen_model_apply_coordinate_texturing_to_mesh(
    e: &mut Engine,
    this: Ptr<BSFaceGenModel>,
    coordinates: u32,
    texture: u32,
    limit: u32,
) -> bool {
    let locals = e.mem.alloc(0x20);
    let image_pointer = locals;
    let iterator = locals + 4;
    e.call(NI_POINTER_FROM_RAW, &args![image_pointer, 0u32]);
    fn_0065cf90(e, iterator);
    let default = e.call(DEFAULT_MODULATION_TEXTURE, &args![]).u32();
    pointer_assign(e, texture, default);
    let done = |e: &mut Engine, answer: bool| {
        e.call(NI_POINTER_DESTRUCT, &args![image_pointer]);
        e.mem.free(locals);
        answer
    };
    let done_unlocked = |e: &mut Engine| {
        e.call(FACE_GEN_UNLOCK, &args![]);
        e.call(NI_POINTER_DESTRUCT, &args![image_pointer]);
        e.mem.free(locals);
        false
    };
    if e.get(this, BSFaceGenModel::pTextureData) == 0 {
        return done(e, false);
    }
    let coords = if coordinates == 0 {
        e.call(GET_FACE_GEN_COORD, &args![]).u32()
    } else {
        coordinates
    };
    if coords == 0 {
        return done(e, false);
    }
    e.call(FACE_GEN_LOCK, &args![]);
    let data = |e: &Engine| e.get(this, BSFaceGenModel::pTextureData);
    if e.call(TEXTURE_LOADED_TEST, &args![this]).u8() == 0 {
        let texture_data = data(e);
        if e.mem.u32(texture_data + 8) == 0 {
            let texture_data = data(e);
            if e.call(STRING_HAS_TEXT, &args![texture_data]).u32() == 0 {
                return done_unlocked(e);
            }
            let texture_data = data(e);
            let name = pointer_get(e, texture_data);
            let egt = e.call(EGT_DATA_LOAD, &args![name]).u32();
            let texture_data = data(e);
            e.mem.set_u32(texture_data + 8, egt);
        }
        let texture_data = data(e);
        if e.mem.u32(texture_data + 8) == 0 {
            return done_unlocked(e);
        }
    }
    let texture_data = data(e);
    if e.mem.u32(texture_data + 8) == 0 {
        return done_unlocked(e);
    }
    let egt_set = |e: &Engine, set: u32| {
        let texture_data = e.get(this, BSFaceGenModel::pTextureData);
        e.mem
            .u32(texture_data + 8)
            .wrapping_add(set * 0x18)
            .wrapping_add(4)
    };
    let (mut width, mut height) = (0u32, 0u32);
    let set0 = egt_set(e, 0);
    if e.call(EGT_SET_SIZE, &args![set0]).u32() != 0 {
        let set0 = egt_set(e, 0);
        let morph = e.call(EGT_SET_MORPH, &args![set0, 0u32]).u32();
        width = e.mem.u32(morph + 4);
        let set0 = egt_set(e, 0);
        let morph = e.call(EGT_SET_MORPH, &args![set0, 0u32]).u32();
        height = e.mem.u32(morph + 8);
    }
    if width == 0 || height == 0 {
        return done_unlocked(e);
    }
    for set in 0..2u32 {
        let available = e
            .call(
                COORD_TEXTURE_COUNT,
                &args![coords.wrapping_add(0x40 + (set << 5))],
            )
            .u32();
        let address = egt_set(e, set);
        if available < e.call(EGT_SET_SIZE, &args![address]).u32() {
            e.call(PRINT_MESSAGE, &args![TEXTURE_COUNT_MESSAGE]);
            return done_unlocked(e);
        }
    }
    let zero = locals + 0xc;
    for word in 0..4 {
        e.mem.set_f32(zero + 4 * word, 0.0);
    }
    let block = e.call(NI_ALLOCATE, &args![0x2cu32]).u32();
    let image = if block != 0 {
        e.call(IMAGE_CONSTRUCT, &args![block, 0u32]).u32()
    } else {
        0
    };
    pointer_assign(e, image_pointer, image);
    let image = pointer_get(e, image_pointer);
    e.call(IMAGE_SET_WIDTH, &args![image, width]);
    let image = pointer_get(e, image_pointer);
    e.call(IMAGE_SET_HEIGHT, &args![image, height]);
    let image = pointer_get(e, image_pointer);
    let colors = e.call(IMAGE_COLORS, &args![image]).u32();
    e.call(
        IMAGE_RESIZE,
        &args![colors, width.wrapping_mul(height), zero],
    );
    let texels = width.wrapping_mul(height);
    let size = texels.wrapping_mul(3).checked_mul(4).unwrap_or(u32::MAX);
    let sums = e.call(OPERATOR_NEW, &args![size]).u32();
    let cleared = (width << 2).wrapping_mul(height).wrapping_mul(3);
    e.call(MEMSET, &args![sums, 0u32, cleared]);

    // Only the first EGT set is summed.
    let set = 0u32;
    let mut morphs = {
        let address = egt_set(e, set);
        e.call(EGT_SET_SIZE, &args![address]).u32()
    };
    if limit != 0 {
        morphs = limit;
        let address = egt_set(e, set);
        if morphs > e.call(EGT_SET_SIZE, &args![address]).u32() {
            let address = egt_set(e, set);
            morphs = e.call(EGT_SET_SIZE, &args![address]).u32();
        }
    }
    let scale = e.global::<f64>(COEFFICIENT_SCALE);
    let checking = face_check::enabled();
    if checking {
        // Diagnostic only (`NV_FACE_CHECK`, not exe behaviour).
        face_check::texturing(e, this.addr(), coords, width, height, morphs, limit);
    }
    let mut weighted = 0u32;
    for index in 0..morphs {
        let address = egt_set(e, set);
        let morph = e.call(EGT_SET_MORPH, &args![address, index]).u32();
        if e.mem.u32(morph + 4) != width {
            e.call(FR2_ASSERT, &args![FACE_GEN_MODEL_SOURCE_FILE, 0x48au32]);
        }
        let address = egt_set(e, set);
        let morph = e.call(EGT_SET_MORPH, &args![address, index]).u32();
        if e.mem.u32(morph + 8) != height {
            e.call(FR2_ASSERT, &args![FACE_GEN_MODEL_SOURCE_FILE, 0x48bu32]);
        }
        let coefficient = e.with_stack(0x10, |e, local| {
            let holder = e
                .call(
                    COORD_VALUE,
                    &args![coords.wrapping_add(0x40 + (set << 5)), local.addr(), index],
                )
                .u32();
            e.call(COORD_ADDRESS, &args![holder, 0u32]).u32()
        });
        let product = x87_mul(x87_m32(e.mem.u32(coefficient)), scale);
        let weight = e.call(FTOL, &args![product]).i32();
        if weight == 0 {
            continue;
        }
        weighted += 1;
        let address = egt_set(e, set);
        let first = e.call(EGT_SET_MORPH, &args![address, index]).u32();
        let address = egt_set(e, set);
        let second = e.call(EGT_SET_MORPH, &args![address, index]).u32();
        let row_step = e.mem.u32(first + 4).wrapping_add(e.mem.u32(second + 0xc));
        for channel in 0..3u32 {
            let address = egt_set(e, set);
            let morph = e.call(EGT_SET_MORPH, &args![address, index]).u32();
            let product = x87_mul(x87_m32(e.mem.u32(morph)), scale);
            let factor = e.call(FTOL, &args![product]).i32().wrapping_mul(weight);
            let address = egt_set(e, set);
            let morph = e.call(EGT_SET_MORPH, &args![address, index]).u32();
            let mut row = e
                .call(
                    EGT_CHANNEL_DATA,
                    &args![morph + 0x10 + 0x18 * channel, 0u32],
                )
                .u32();
            for y in 0..height {
                for x in 0..width {
                    let slot = y
                        .wrapping_mul(width)
                        .wrapping_add(x)
                        .wrapping_mul(3)
                        .wrapping_add(channel);
                    let delta = e.mem.i8(row.wrapping_add(x)) as i32;
                    let address = sums.wrapping_add(slot.wrapping_mul(4));
                    let sum = delta.wrapping_mul(factor).wrapping_add(e.mem.i32(address));
                    e.mem.set_i32(address, sum);
                }
                row = row.wrapping_add(row_step);
            }
        }
    }
    if checking {
        let factor = f32::from_bits(x87_quiet_bits(e.global::<u32>(COLOR_SCALE_01065B9C)));
        face_check::texturing_sums(e, sums, texels, weighted, factor);
    }
    for channel in 0..3u32 {
        let factor = x87_quiet_bits(e.global::<u32>(COLOR_SCALE_01065B9C));
        let element = e.with_stack(8, |e, begin| {
            let first = e.call(VECTOR_BEGIN, &args![colors, begin.addr()]).u32();
            let words = [e.mem.u32(first), e.mem.u32(first + 4)];
            e.mem.set_u32(iterator, words[0]);
            e.mem.set_u32(iterator + 4, words[1]);
            e.call(ITERATOR_ELEMENT, &args![iterator]).u32()
        });
        for y in 0..height {
            for x in 0..width {
                let texel = y.wrapping_mul(width).wrapping_add(x);
                let sum = e.mem.i32(
                    sums.wrapping_add(texel.wrapping_mul(3).wrapping_add(channel).wrapping_mul(4)),
                );
                let value = x87_store32(x87_mul(f64::from(sum), x87_m32(factor)));
                let target = element
                    .wrapping_add(texel.wrapping_mul(4).wrapping_add(channel).wrapping_mul(4));
                e.mem.set_u32(target, value);
            }
        }
    }
    e.call(OPERATOR_DELETE, &args![sums]);
    let image = pointer_get(e, image_pointer);
    let x = x87_quiet_bits(e.global::<u32>(IMAGE_SCALE_01065B98));
    let y = x87_quiet_bits(e.global::<u32>(IMAGE_SCALE_01023CD8));
    let z = x87_quiet_bits(e.global::<u32>(IMAGE_SCALE_01016248));
    let compressed = e
        .call(
            GET_COMPRESSED_IMAGE,
            &args![image, x, y, z, IMAGE_FORMAT_011AA2A0],
        )
        .u32();
    pointer_assign(e, texture, compressed);
    e.call(FACE_GEN_UNLOCK, &args![]);
    done(e, true)
}

/// `00406d30` (cdecl: buffer, size, source): `strcpy_s` by its body.
const STRCPY_S_00406D30: u32 = 0x0040_6d30;
/// `00406d50` (cdecl: buffer, size, source): `strcat_s` by its body.
const STRCAT_S_00406D50: u32 = 0x0040_6d50;
/// `strrchr` (`0040ab30`, cdecl: string, character).
const STRRCHR: u32 = 0x0040_ab30;
/// `006505d0` (thiscall on the model): 0 without mesh data (+8), else a string by its
/// use here (it calls `00559450`).
const MESH_NAME_006505D0: u32 = 0x0065_05d0;
/// `0065c2c0` (thiscall on the model; unnamed, `bsfacegenmodel.cpp` by its timer's file).
const TEXTURE_LOAD_0065C2C0: u32 = 0x0065_c2c0;
/// `".egt"`.
const EGT_EXTENSION: u32 = 0x0106_5b90;

// Translated from 0065a840 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065a840 - the /GS stack cookie check (00ec408c) is not translated.
/// Unnamed in the engine map (`thiscall`, no arguments): without texture data (+0xc),
/// takes the name `006505d0` gives (when there is one) into a 0x104-byte stack buffer
/// (`strcpy_s`), and when it has a `.`, replaces the last one and what follows with
/// `".egt"` (`strcat_s`), sets that as the EGT name (`fn_0065a760`) and calls `0065c2c0`.
pub fn fn_0065a840(e: &mut Engine, this: Ptr<BSFaceGenModel>) {
    if e.get(this, BSFaceGenModel::pTextureData) != 0 {
        return;
    }
    let name = e.call(MESH_NAME_006505D0, &args![this]).u32();
    if name == 0 {
        return;
    }
    e.with_stack(0x104, |e, buffer| {
        let buffer = buffer.addr();
        e.call(STRCPY_S_00406D30, &args![buffer, 0x104u32, name]);
        let dot = e.call(STRRCHR, &args![buffer, 0x2eu32]).u32();
        if dot != 0 {
            e.mem.set_u8(dot, 0);
            e.call(STRCAT_S_00406D50, &args![buffer, 0x104u32, EGT_EXTENSION]);
            fn_0065a760(e, this, buffer);
            e.call(TEXTURE_LOAD_0065C2C0, &args![this]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_9b20, bs_face_gen_model_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x0065_a840, fn_0065a840(Ptr<BSFaceGenModel>)),
        entry!(
            0x0065_9800,
            fn_00659800(Ptr<BSFaceGenModelMeshData>) -> Ptr<BSFaceGenModelMeshData>
        ),
        entry!(0x0065_98d0, fn_006598d0(Ptr<BSFaceGenModelMeshData>)),
        entry!(
            0x0065_9a00,
            fn_00659a00(Ptr<BSFaceGenModelTextureData>) -> Ptr<BSFaceGenModelTextureData>
        ),
        entry!(0x0065_9a20, fn_00659a20(Ptr<BSFaceGenModelTextureData>)),
        entry!(0x0065_9b50, fn_00659b50(Ptr<BSFaceGenModel>)),
        entry!(
            0x0065_9c10,
            fn_00659c10(Ptr<BSFaceGenModelMeshData>, u32) -> Ptr<BSFaceGenModelMeshData>
        ),
        entry!(
            0x0065_9c40,
            fn_00659c40(Ptr<BSFaceGenModelTextureData>, u32) -> Ptr<BSFaceGenModelTextureData>
        ),
        entry!(
            0x0065_9c70,
            fn_00659c70(Ptr<BSFaceGenModel>, u32, u32, u32, u32, i32, u8) -> bool
        ),
        entry!(
            0x0065_9ce0,
            bs_face_gen_model_load_model_mesh(Ptr<BSFaceGenModel>, u32, u32, u32, i32, u8) -> bool
        ),
        entry!(0x0065_a4c0, fn_0065a4c0(Ptr) -> Ptr),
        entry!(0x0065_a5a0, fff_tri_file_data_s_num_model_vtx(Ptr) -> u32),
        entry!(0x0065_a620, fn_0065a620(u32, u32)),
        entry!(0x0065_a650, fn_0065a650(Ptr, u32) -> Ptr),
        entry!(0x0065_a680, fn_0065a680(Ptr)),
        entry!(0x0065_a760, fn_0065a760(Ptr<BSFaceGenModel>, u32) -> bool),
        entry!(
            0x0065_a910,
            fn_0065a910(Ptr<BSFaceGenModel>, u32, u32) -> bool
        ),
        entry!(
            0x0065_a970,
            fn_0065a970(Ptr<BSFaceGenModel>, u32, u32, u8) -> bool
        ),
        entry!(0x0065_b390, fn_0065b390(u32, u32, u32)),
        entry!(0x0065_b3e0, fn_0065b3e0(u32, bool) -> u32),
        entry!(
            0x0065_b410,
            bs_face_gen_model_apply_coordinate_texturing_to_mesh(
                Ptr<BSFaceGenModel>,
                u32,
                u32,
                u32,
            ) -> bool
        ),
        entry!(
            0x0065_bbc0,
            bs_face_gen_model_create_new_mesh(Ptr<BSFaceGenModel>, u32) -> bool
        ),
        entry!(0x0065_be60, fn_0065be60(Ptr, u32) -> Ptr),
        entry!(0x0065_c820, fn_0065c820(u32) -> u32),
        entry!(0x0065_c890, fn_0065c890(u32)),
        entry!(0x0065_c8f0, fn_0065c8f0(u32) -> u32),
        entry!(0x0065_c960, fn_0065c960(u32)),
        entry!(0x0065_c9c0, fn_0065c9c0(u32) -> u32),
        entry!(0x0065_ca30, fn_0065ca30(u32)),
        entry!(0x0065_ca90, fn_0065ca90(u32) -> u32),
        entry!(0x0065_cb00, fn_0065cb00(u32)),
        entry!(0x0065_cb60, fn_0065cb60(u32) -> u32),
        entry!(0x0065_cbd0, fn_0065cbd0(u32)),
        entry!(0x0065_cf50, fn_0065cf50(u32, u32, u32) -> u32),
        entry!(0x0065_cf90, fn_0065cf90(u32) -> u32),
        entry!(0x0065_f240, std_vector_unsigned_long_tidy(u32)),
    ]
}

// ---------------------------------------------------------------------------------------------
// `NV_FACE_CHECK`: diagnostic only, not exe behaviour. Reads, never writes, game memory (the
// optional image files go to `NV_FACE_CHECK_DIR`, outside the game).
//
// Splits the player's broken race-menu face between geometry and texture in one run:
// - `morph` lines (0065a970, every FaceGen morph of a mesh): the geometry's name, its data and
//   additional-vertex array, how often that data / array has been morphed so far, the largest
//   coefficient of the symmetric and asymmetric geometry sets, and how far the vertices really
//   moved in this call (largest and mean distance). The same data or array morphed again with
//   whole coordinates (not a race-menu difference) means the morph accumulates; a player
//   coefficient far above an NPC's means the coordinates are wrong.
// - `texturing` lines (0065b410, the face texture built from the EGT for the player only): the
//   EGT size, its morph count, the texture coefficients, the weights used and the summed range.
// - `image` lines (0064ceb0): the compressed image's size, channels, byte ranges per channel
//   and the texture made; with `NV_FACE_CHECK_DIR` the image is also written there as a TGA.
// ---------------------------------------------------------------------------------------------
pub(crate) mod face_check {
    use super::*;
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::{Mutex, OnceLock};

    /// Whether `NV_FACE_CHECK` is set (read once).
    pub(crate) fn enabled() -> bool {
        static ON: OnceLock<bool> = OnceLock::new();
        *ON.get_or_init(|| std::env::var_os("NV_FACE_CHECK").is_some())
    }

    static LINES: AtomicU32 = AtomicU32::new(0);
    const LIMIT: u32 = 2000;

    pub(crate) fn line(text: std::fmt::Arguments) {
        if LINES.fetch_add(1, Ordering::Relaxed) < LIMIT {
            println!("face check: {text}");
        }
    }

    #[derive(Default)]
    struct State {
        /// Morph calls seen per geometry data and per additional-vertex array.
        morphed: HashMap<u32, u32>,
        /// The vertex positions of the morph in progress (before it).
        before: Vec<[f32; 3]>,
        images: u32,
    }

    fn state() -> std::sync::MutexGuard<'static, State> {
        static STATE: OnceLock<Mutex<State>> = OnceLock::new();
        STATE
            .get_or_init(Default::default)
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    /// The name of an `NiObjectNET` (its `NiFixedString` at +8), at most 48 characters.
    pub(crate) fn name(e: &Engine, object: u32) -> String {
        if object == 0 {
            return "-".into();
        }
        let text = e.mem.u32(object.wrapping_add(8));
        if text == 0 {
            return "-".into();
        }
        let mut bytes = e.mem.cstr(text);
        bytes.truncate(48);
        String::from_utf8_lossy(&bytes).into_owned()
    }

    /// An `FR2MatrixVTC<float>`'s values (rows at +0x18, columns at +0x1c, the floats at +0xc).
    pub(crate) fn matrix_values(e: &Engine, matrix: u32) -> Vec<f32> {
        let rows = e.mem.u32(matrix.wrapping_add(0x18));
        let columns = e.mem.u32(matrix.wrapping_add(0x1c));
        let first = e.mem.u32(matrix.wrapping_add(0xc));
        let count = rows.saturating_mul(columns).min(256);
        if first == 0 {
            return Vec::new();
        }
        (0..count)
            .map(|i| e.mem.f32(first.wrapping_add(4 * i)))
            .collect()
    }

    fn largest(values: &[f32]) -> f32 {
        values.iter().fold(0.0f32, |m, v| m.max(v.abs()))
    }

    fn short(values: &[f32], n: usize) -> String {
        let parts: Vec<String> = values.iter().take(n).map(|v| format!("{v:.3}")).collect();
        parts.join(",")
    }

    fn vertex(e: &Engine, base: u32, stride: u32, index: u32) -> [f32; 3] {
        let at = base.wrapping_add(index.wrapping_mul(stride));
        [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
    }

    /// The start of a morph (0065a970): reports what is morphed and keeps the positions.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn morph_start(
        e: &Engine,
        model: u32,
        coords: u32,
        geometry: u32,
        data: u32,
        apply_all: u8,
        count: u32,
        iterator: u32,
        extra: u32,
    ) {
        let base = e.mem.u32(iterator);
        let stride = e.mem.u32(iterator.wrapping_add(4));
        let mut s = state();
        let data_seen = {
            let n = s.morphed.entry(data).or_insert(0);
            *n += 1;
            *n
        };
        let extra_seen = if extra != 0 {
            let n = s.morphed.entry(extra).or_insert(0);
            *n += 1;
            *n
        } else {
            0
        };
        s.before = (0..count.min(20000))
            .map(|i| vertex(e, base, stride, i))
            .collect();
        drop(s);
        let symmetric = matrix_values(e, coords);
        let asymmetric = matrix_values(e, coords.wrapping_add(0x20));
        let extra_first = if extra != 0 {
            format!("{:?}", vertex(e, extra, 0xc, 0))
        } else {
            "-".into()
        };
        line(format_args!(
            "morph model {model:#x} geometry {geometry:#x} '{}' data {data:#x} (morph #{data_seen}) extra {extra:#x} (#{extra_seen}, first {extra_first}) all {apply_all} vertices {count} coords {coords:#x}: sym {} max {:.3} [{}] asym {} max {:.3} [{}]",
            name(e, geometry),
            symmetric.len(),
            largest(&symmetric),
            short(&symmetric, 8),
            asymmetric.len(),
            largest(&asymmetric),
            short(&asymmetric, 6),
        ));
    }

    /// The end of a morph: how far the vertices moved.
    pub(crate) fn morph_end(e: &Engine, geometry: u32, iterator: u32) {
        let base = e.mem.u32(iterator);
        let stride = e.mem.u32(iterator.wrapping_add(4));
        let before = std::mem::take(&mut state().before);
        let (mut most, mut total, mut at) = (0.0f32, 0.0f64, 0usize);
        for (i, old) in before.iter().enumerate() {
            let new = vertex(e, base, stride, i as u32);
            let d =
                ((new[0] - old[0]).powi(2) + (new[1] - old[1]).powi(2) + (new[2] - old[2]).powi(2))
                    .sqrt();
            total += f64::from(d);
            if d > most {
                most = d;
                at = i;
            }
        }
        let mean = if before.is_empty() {
            0.0
        } else {
            total / before.len() as f64
        };
        line(format_args!(
            "morph done '{}': moved max {most:.3} (vertex {at}) mean {mean:.3}",
            name(e, geometry)
        ));
    }

    /// The texture coordinates and EGT of a face texture build (0065b410).
    pub(crate) fn texturing(
        e: &Engine,
        model: u32,
        coords: u32,
        width: u32,
        height: u32,
        morphs: u32,
        limit: u32,
    ) {
        let symmetric = matrix_values(e, coords.wrapping_add(0x40));
        let asymmetric = matrix_values(e, coords.wrapping_add(0x60));
        line(format_args!(
            "texturing model {model:#x} coords {coords:#x} egt {width}x{height} morphs {morphs} limit {limit}: texture sym {} max {:.3} [{}] asym {} max {:.3}",
            symmetric.len(),
            largest(&symmetric),
            short(&symmetric, 10),
            asymmetric.len(),
            largest(&asymmetric),
        ));
    }

    /// The summed texel deltas of a face texture build.
    pub(crate) fn texturing_sums(e: &Engine, sums: u32, texels: u32, used: u32, scale: f32) {
        let mut low = [i32::MAX; 3];
        let mut high = [i32::MIN; 3];
        for texel in 0..texels {
            for channel in 0..3u32 {
                let v = e.mem.i32(sums.wrapping_add((texel * 3 + channel) * 4));
                low[channel as usize] = low[channel as usize].min(v);
                high[channel as usize] = high[channel as usize].max(v);
            }
        }
        line(format_args!(
            "texturing sums: {used} weighted morphs; per channel {:?}..{:?}, as colours {:.1}..{:.1}",
            low,
            high,
            low.iter().copied().min().unwrap_or(0) as f32 * scale,
            high.iter().copied().max().unwrap_or(0) as f32 * scale,
        ));
    }

    /// A shader property's texture arrays (`BSShaderPPLightingProperty`: the count at +0xA8, the
    /// six arrays at +0xAC, each of the count): every entry with its `NiSourceTexture` file name
    /// (+0x34), for a head part as 00655fa0 left it.
    pub(crate) fn shader_textures(e: &Engine, part: u32, geometry: u32, shader: u32) {
        let count = u32::from(e.mem.u16(shader.wrapping_add(0xa8))).min(8);
        let mut kinds = Vec::new();
        for kind in 0..6u32 {
            let array = e.mem.u32(shader.wrapping_add(0xac + 4 * kind));
            if array == 0 {
                kinds.push(format!("k{kind}: none"));
                continue;
            }
            let entries: Vec<String> = (0..count)
                .map(|i| {
                    let texture = e.mem.u32(array.wrapping_add(4 * i));
                    if texture == 0 {
                        return "0".to_string();
                    }
                    let file = e.mem.u32(texture.wrapping_add(0x34));
                    let name = if file != 0 {
                        String::from_utf8_lossy(&e.mem.cstr(file)).into_owned()
                    } else {
                        "(no file)".into()
                    };
                    format!("{texture:#x} {name}")
                })
                .collect();
            kinds.push(format!("k{kind}: [{}]", entries.join(", ")));
        }
        line(format_args!(
            "part {part} '{}' shader {shader:#x} type {} textures: {}",
            name(e, geometry),
            e.mem.u32(shader.wrapping_add(0x1c)),
            kinds.join("; ")
        ));
    }

    /// The compressed image (0064ceb0): byte ranges and an optional TGA copy.
    pub(crate) fn image(
        e: &Engine,
        pixels: u32,
        width: u32,
        height: u32,
        channels: u32,
        made: u32,
    ) {
        let count = width.saturating_mul(height);
        let mut low = [255u8; 4];
        let mut high = [0u8; 4];
        let mut sum = [0u64; 4];
        let mut bytes = Vec::with_capacity((count * channels) as usize);
        for i in 0..count.saturating_mul(channels) {
            let b = e.mem.u8(pixels.wrapping_add(i));
            let c = (i % channels) as usize;
            low[c] = low[c].min(b);
            high[c] = high[c].max(b);
            sum[c] += u64::from(b);
            bytes.push(b);
        }
        let mean: Vec<u64> = sum.iter().map(|s| s / u64::from(count.max(1))).collect();
        let n = {
            let mut s = state();
            s.images += 1;
            s.images
        };
        // The real-device side follows the next uncompressed texture of this size.
        engine_core::units::platform::FACE_CHECK_IMAGES
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .push((u64::from(width) << 32) | u64::from(height));
        line(format_args!(
            "image #{n} {width}x{height} {channels} channels, pixel data {pixels:#x}, texture {made:#x}: min {:?} max {:?} mean {:?}",
            &low[..channels as usize],
            &high[..channels as usize],
            &mean[..channels as usize],
        ));
        if let Some(dir) = std::env::var_os("NV_FACE_CHECK_DIR") {
            let path = std::path::Path::new(&dir).join(format!("facegen-{n}-{width}x{height}.tga"));
            let mut file = vec![0u8; 18];
            file[2] = 2;
            file[12..14].copy_from_slice(&(width as u16).to_le_bytes());
            file[14..16].copy_from_slice(&(height as u16).to_le_bytes());
            file[16] = (channels * 8) as u8;
            // Rows top first (descriptor bit 5); alpha bits for four channels.
            file[17] = 0x20 | if channels == 4 { 8 } else { 0 };
            for pixel in bytes.chunks(channels as usize) {
                // The pixel data is R, G, B(, A); TGA stores B, G, R(, A).
                file.extend([pixel[2], pixel[1], pixel[0]]);
                if channels == 4 {
                    file.push(pixel[3]);
                }
            }
            match std::fs::write(&path, &file) {
                Ok(()) => line(format_args!("image #{n} written to {}", path.display())),
                Err(error) => line(format_args!("image #{n} not written: {error}")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_deleting_destructor(address: u32, body: u32, size: u32) {
        let mut e = Engine::new();
        let this = Ptr::<()>::new(e.mem.alloc(size));
        e.register(body, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        for flags in [0u32, 1, 2, 3] {
            assert_eq!(e.call(address, &args![this, flags]).u32(), this.addr());
        }
        let log = e.call_log.take().unwrap();
        let calls_to = |target: u32| {
            log.iter()
                .filter(|(x, _)| *x == target)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_eq!(calls_to(body).len(), 4);
        assert_eq!(
            calls_to(0x00aa_1460),
            vec![(0x00aa_1460, vec![this.addr(), size]); 2]
        );
    }

    #[test]
    fn bs_face_gen_model_scalar_deleting_destructor_frees_only_with_flag_bit_zero() {
        check_deleting_destructor(0x0065_9b20, 0x0065_9b50, 0x10);
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

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    fn answer(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// `NiPointer` get, assignment and construction as plain memory operations.
    fn plain_pointers(e: &mut Engine) {
        e.register(NI_POINTER_GET, |e, a| answer(e.mem.u32(a[0])));
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(NI_POINTER_FROM_RAW, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            answer(a[0])
        });
        quiet(e, &[NI_POINTER_DESTRUCT]);
    }

    /// An object whose vtable has `slots` (byte offset, target) set.
    fn object_with(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for &(slot, target) in slots {
            e.mem.set_u32(vtable + slot, target);
        }
        let object = e.mem.alloc(size.max(4));
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn mesh_and_texture_data_construct_and_destruct() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(
            &mut e,
            &[
                STRING_CONSTRUCT,
                STRING_DESTRUCT,
                OPERATOR_DELETE,
                EGM_DATA_DELETE,
                EGT_DATA_DELETE,
            ],
        );
        let mesh = e.mem.alloc(0x24);
        for offset in (0..0x24).step_by(4) {
            e.mem.set_u32(mesh + offset, 0xdead_beef);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_9800, &args![mesh]).u32(), mesh);
        assert_eq!(calls(&e, STRING_CONSTRUCT), vec![vec![mesh]]);
        for offset in [8u32, 0xc, 0x10, 0x14, 0x18, 0x1c, 0x20] {
            assert_eq!(e.mem.u32(mesh + offset), 0);
        }
        e.mem.set_u32(mesh + 0x14, 0x1400);
        e.mem.set_u32(mesh + 8, 0x0800);
        e.mem.set_u32(mesh + 0x10, 0x1000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_9c10, &args![mesh, 1u32]).u32(), mesh);
        assert_eq!(e.mem.u32(mesh + 0x10), 0);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x1400], vec![mesh]]);
        assert_eq!(calls(&e, EGM_DATA_DELETE), vec![vec![0x0800, 1]]);
        assert_eq!(
            calls(&e, NI_POINTER_DESTRUCT),
            vec![vec![mesh + 0x1c], vec![mesh + 0x10], vec![mesh + 0xc]]
        );
        assert_eq!(calls(&e, STRING_DESTRUCT), vec![vec![mesh]]);
        // Nothing held and no delete bit.
        e.mem.set_u32(mesh + 0x14, 0);
        e.mem.set_u32(mesh + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x0065_9c10, &args![mesh, 0u32]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
        assert!(calls(&e, EGM_DATA_DELETE).is_empty());

        let texture = e.mem.alloc(0xc);
        e.mem.set_u32(texture + 8, 0x77);
        assert_eq!(e.call(0x0065_9a00, &args![texture]).u32(), texture);
        assert_eq!(e.mem.u32(texture + 8), 0);
        e.mem.set_u32(texture + 8, 0x0900);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_9c40, &args![texture, 1u32]).u32(), texture);
        assert_eq!(calls(&e, EGT_DATA_DELETE), vec![vec![0x0900, 1]]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![texture]]);
        e.mem.set_u32(texture + 8, 0);
        e.call_log = Some(vec![]);
        e.call(0x0065_9a20, &args![texture]);
        assert!(calls(&e, EGT_DATA_DELETE).is_empty());
        assert_eq!(calls(&e, STRING_DESTRUCT), vec![vec![texture]]);
    }

    #[test]
    fn model_destructor_deletes_both_data() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(
            &mut e,
            &[STRING_DESTRUCT, OPERATOR_DELETE, REF_OBJECT_DESTRUCT],
        );
        let model = e.mem.alloc(0x10);
        let mesh = e.mem.alloc(0x24);
        let texture = e.mem.alloc(0xc);
        e.mem.set_u32(model + 8, mesh);
        e.mem.set_u32(model + 0xc, texture);
        e.call_log = Some(vec![]);
        e.call(0x0065_9b50, &args![model]);
        assert_eq!(e.mem.u32(model), FACE_GEN_MODEL_VTABLE);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![mesh], vec![texture]]);
        assert_eq!(calls(&e, REF_OBJECT_DESTRUCT), vec![vec![model]]);
        e.mem.set_u32(model + 8, 0);
        e.mem.set_u32(model + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0065_9b50, &args![model]);
        assert!(calls(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn load_gates_on_the_setting_and_ors_both_halves() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(
            &mut e,
            &[TIMER_SCOPE_CONSTRUCT, TIMER_SCOPE_DESTROY, STRING_SET],
        );
        e.register(FACE_GEN_ENABLED, |_, _| answer(0));
        let model = e.mem.alloc(0x10);
        assert!(!e
            .call(
                0x0065_9c70,
                &args![model, 1u32, 0u32, 0u32, 2u32, 0i32, 0u8]
            )
            .bool());
        e.register(FACE_GEN_ENABLED, |_, _| answer(1));
        // Both halves already loaded: 1, nothing made.
        e.mem.set_u32(model + 8, 0x10);
        e.mem.set_u32(model + 0xc, 0x20);
        e.call_log = Some(vec![]);
        assert!(e
            .call(
                0x0065_9c70,
                &args![model, 1u32, 0u32, 0u32, 2u32, 0i32, 0u8]
            )
            .bool());
        assert!(calls(&e, TIMER_SCOPE_CONSTRUCT).is_empty());
        // The texture half alone.
        e.mem.set_u32(model + 0xc, 0);
        quiet(&mut e, &[STRING_CONSTRUCT]);
        e.register(OPERATOR_NEW, |e, a| answer(e.mem.alloc(a[0])));
        e.call_log = Some(vec![]);
        assert!(e
            .call(
                0x0065_9c70,
                &args![model, 1u32, 0u32, 0u32, 0x2222u32, 0i32, 0u8]
            )
            .bool());
        let texture = e.mem.u32(model + 0xc);
        assert_ne!(texture, 0);
        assert_eq!(calls(&e, STRING_SET), vec![vec![texture, 0x2222, 0]]);
        let timer = calls(&e, TIMER_SCOPE_CONSTRUCT);
        assert_eq!(timer[0][1..], [0x39, 1, FACE_GEN_MODEL_SOURCE_FILE, 0x1c0]);
    }

    struct Load {
        e: Engine,
        model: u32,
        children: Vec<u32>,
    }

    /// A model loader answering a root with `names` children (each geometry answering
    /// itself from slot +0x1c).
    fn load_rig(names: &[u32]) -> Load {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(
            &mut e,
            &[
                TIMER_SCOPE_CONSTRUCT,
                TIMER_SCOPE_DESTROY,
                STRING_CONSTRUCT,
                STRING_SET,
                CLONING_PROCESS_CONSTRUCT,
                CLONING_PROCESS_DESTRUCT,
                REMOVE_PROPERTY,
                REMOVE_ALL_CONTROLLERS,
                OBJECT_NET_00A5BFA0,
                SKIN_SET_ROOT,
                NODE_SET_FLAG_004902F0,
                UPDATE_PROPERTIES,
                UPDATE_DATA_CONSTRUCT,
                UPDATE_00A59C60,
                SET_CONSISTENCY,
                PRINT_MESSAGE,
            ],
        );
        e.register(OPERATOR_NEW, |e, a| answer(e.mem.alloc(a[0])));
        e.map(MODEL_LOADER, 4);
        e.map(TES_011DEA10, 4);
        e.register(MODEL_LOADER_LOAD_FILE, |_, _| answer(0x0000_0a00));
        e.register(PROPERTY_TYPE_004AF340, |_, _| answer(5));
        let mut children = vec![];
        for &name in names {
            let child = object_with(&mut e, 0xc0, &[(0x1c, 0x0000_7c1c)]);
            e.mem.set_u32(child + 8, name);
            children.push(child);
        }
        e.register(0x0000_7c1c, |_, a| answer(a[0]));
        let root = e.mem.alloc(4);
        let list = children.clone();
        e.register_double(CREATE_DEEP_COPY_SAME_TEXTURES, move |_, _| answer(root));
        let count = list.len() as u32;
        e.register_double(NODE_CHILD_COUNT, move |_, _| answer(count));
        e.register_double(NODE_CHILD, move |_, a| answer(list[a[1] as usize]));
        e.register(OBJECT_NAME, |_, a| answer(a[0] + 8));
        e.register(FIXED_STRING_TEXT, |e, a| answer(e.mem.u32(a[0])));
        e.register(COMPARE_TEXT, |_, a| answer((a[0] != a[1]) as u32));
        e.map(MESH_NAMES, 8);
        e.set_global(MESH_NAMES, 0x0000_0111u32);
        e.set_global(MESH_NAMES + 4, 0x0000_0222u32);
        e.register(SKIN_INSTANCE, |_, _| answer(0));
        let model = e.mem.alloc(0x10);
        Load { e, model, children }
    }

    fn load(l: &mut Load, model: u32, tri: u32, kind: i32, keep: u8) -> bool {
        l.e.call_log = Some(vec![]);
        l.e.call(
            0x0065_9ce0,
            &args![l.model, 0x3333u32, model, tri, kind, keep],
        )
        .bool()
    }

    fn picked(l: &Load) -> u32 {
        let mesh = l.e.mem.u32(l.model + 8);
        l.e.mem.u32(mesh + 0x10)
    }

    #[test]
    fn load_model_mesh_picks_the_geometry_by_kind() {
        // Kind 0 with one child takes it whatever its name.
        let mut l = load_rig(&[0x999]);
        assert!(load(&mut l, 0x4440, 0, 0, 0));
        assert_eq!(picked(&l), l.children[0]);
        assert_eq!(
            calls(&l.e, MODEL_LOADER_LOAD_FILE)[0][1..],
            [0x4447, 0, 1, 0, 1, 1]
        );
        assert_eq!(calls(&l.e, REMOVE_PROPERTY), vec![vec![l.children[0], 5]]);
        assert_eq!(
            calls(&l.e, NODE_SET_FLAG_004902F0),
            vec![vec![l.children[0], 1]]
        );
        assert_eq!(calls(&l.e, SET_CONSISTENCY).len(), 1);
        // Kind 0 with several children and kind 1 match by name.
        let mut l = load_rig(&[0x999, 0x111, 0x222]);
        assert!(load(&mut l, 0x4440, 0, 0, 1));
        assert_eq!(picked(&l), l.children[1]);
        assert!(calls(&l.e, REMOVE_PROPERTY).is_empty());
        let mut l = load_rig(&[0x999, 0x111, 0x222]);
        assert!(load(&mut l, 0x4440, 0, 1, 0));
        assert_eq!(picked(&l), l.children[2]);
        // Any other kind takes the first child.
        let mut l = load_rig(&[0x999, 0x111]);
        assert!(load(&mut l, 0x4440, 0, 2, 0));
        assert_eq!(picked(&l), l.children[0]);
    }

    #[test]
    fn load_model_mesh_early_answers_and_a_missing_model() {
        let mut l = load_rig(&[0x999]);
        l.e.mem.set_u32(l.model + 8, 0x10);
        assert!(load(&mut l, 0x4440, 0, 0, 0));
        assert!(calls(&l.e, TIMER_SCOPE_CONSTRUCT).is_empty());
        let mut l = load_rig(&[0x999]);
        l.e.call_log = Some(vec![]);
        assert!(l
            .e
            .call(0x0065_9ce0, &args![l.model, 0u32, 1u32, 0u32, 0i32, 0u8])
            .bool());
        assert_eq!(l.e.mem.u32(l.model + 8), 0);
        // The loader fails: a message and 0, the mesh data kept.
        let mut l = load_rig(&[0x999]);
        l.e.register(MODEL_LOADER_LOAD_FILE, |_, _| answer(0));
        assert!(!load(&mut l, 0x4440, 0, 0, 0));
        assert_eq!(
            calls(&l.e, PRINT_MESSAGE),
            vec![vec![LOAD_FAILED_MESSAGE, 0x4440]]
        );
        assert_ne!(l.e.mem.u32(l.model + 8), 0);
        assert_eq!(calls(&l.e, TIMER_SCOPE_DESTROY).len(), 1);
    }

    /// The tri reader and the tri data's members for `LoadModelMesh`'s tri part.
    fn tri_rig(l: &mut Load, tri_vertices: u32, model_vertices: u16, read: u32) {
        quiet(
            &mut l.e,
            &[
                TRI_STREAM_CONSTRUCT,
                TRI_STREAM_DESTRUCT,
                VECTOR_CONSTRUCT_0065C630,
                VECTOR_CONSTRUCT_0065C750,
                VECTOR_DESTRUCT_0065C6A0,
                VECTOR_DESTRUCT_0065C7C0,
                ALLOCATOR_CONSTRUCT,
                VECTOR_BASE_CONSTRUCT,
                VECTOR_BASE_DESTRUCT,
                BUY_0065D1C0,
                BUY_0065D2B0,
                BUY_0065D3E0,
                BUY_0065D4E0,
                BUY_0065D5E0,
                TIDY_0065D240,
                TIDY_0065D330,
                TIDY_0065D460,
                TIDY_0065D560,
                TIDY_0065D660,
                OPERATOR_DELETE,
                VECTOR_CONSTRUCT,
            ],
        );
        l.e.register_double(READ_TRI_FILE, move |_, _| answer(read));
        l.e.register_double(GEOMETRY_VERTEX_COUNT, move |_, _| {
            answer(model_vertices as u32)
        });
        l.e.register_double(TRI_VERTEX_COUNT, move |_, _| answer(tri_vertices));
        let morphs = l.e.mem.alloc(0x38);
        l.e.mem.set_u32(morphs + 0x1c, model_vertices as u32);
        l.e.register(TRI_MORPH_COUNT, |_, _| answer(1));
        l.e.register_double(TRI_MORPH, move |_, _| answer(morphs));
        let points = l.e.mem.alloc(12 * tri_vertices.max(1));
        for i in 0..3 * tri_vertices {
            l.e.mem.set_f32(points + 4 * i, i as f32);
        }
        l.e.register_double(TRI_VERTEX, move |_, a| answer(points + 12 * a[1]));
        l.e.register(NI_ALLOCATE, |e, a| answer(e.mem.alloc(a[0])));
        l.e.register(MORPH_DATA_HAIR_CONSTRUCT, |_, a| answer(a[0] + 1));
        l.e.register(MORPH_DATA_HEAD_CONSTRUCT, |_, a| answer(a[0] + 2));
        l.e.register(POINT3_CONSTRUCT, |e, a| {
            for word in 0..3u32 {
                e.mem.set_u32(a[0] + 4 * word, a[1 + word as usize]);
            }
            answer(a[0])
        });
    }

    #[test]
    fn load_model_mesh_reads_the_tri_file_and_keeps_extra_head_vertices() {
        let mut l = load_rig(&[0x999]);
        tri_rig(&mut l, 5, 3, 1);
        assert!(load(&mut l, 0x4440, 0x5550, -1, 0));
        let mesh = l.e.mem.u32(l.model + 8);
        let morph = l.e.mem.u32(mesh + 0x1c);
        assert_eq!(morph & 3, 2);
        assert_eq!(l.e.mem.u32(mesh + 0x18), 2);
        let extra = l.e.mem.u32(mesh + 0x14);
        // Vertices 3 and 4 of the tri data.
        assert_eq!(
            (0..6)
                .map(|i| l.e.mem.f32(extra + 4 * i))
                .collect::<Vec<_>>(),
            vec![9.0, 10.0, 11.0, 12.0, 13.0, 14.0]
        );
        assert_eq!(
            calls(&l.e, VECTOR_CONSTRUCT),
            vec![vec![extra, 0xc, 2, ALLOCATOR_CONSTRUCT]]
        );
        assert_eq!(calls(&l.e, READ_TRI_FILE).len(), 1);
        assert_eq!(calls(&l.e, TRI_STREAM_DESTRUCT).len(), 1);
        // The tri data is deleted.
        assert_eq!(calls(&l.e, OPERATOR_DELETE).len(), 1);
    }

    #[test]
    fn load_model_mesh_hair_and_mismatched_tri_files() {
        let mut l = load_rig(&[0x999]);
        tri_rig(&mut l, 3, 3, 1);
        assert!(load(&mut l, 0x4440, 0x5550, 0, 0));
        let mesh = l.e.mem.u32(l.model + 8);
        assert_eq!(l.e.mem.u32(mesh + 0x1c) & 3, 1);
        assert_eq!(l.e.mem.u32(mesh + 0x14), 0);
        // A tri file that does not read, or does not match, makes no morph data.
        for (read, vertices) in [(0u32, 3u16), (1, 4)] {
            let mut l = load_rig(&[0x999]);
            tri_rig(&mut l, 3, vertices, read);
            assert!(load(&mut l, 0x4440, 0x5550, 0, 0));
            let mesh = l.e.mem.u32(l.model + 8);
            assert_eq!(l.e.mem.u32(mesh + 0x1c), 0);
        }
    }

    #[test]
    fn tri_data_members_and_vectors() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                VECTOR_CONSTRUCT_0065C630,
                VECTOR_CONSTRUCT_0065C750,
                VECTOR_DESTRUCT_0065C6A0,
                VECTOR_DESTRUCT_0065C7C0,
                VECTOR_BASE_CONSTRUCT,
                VECTOR_BASE_DESTRUCT,
                BUY_0065D1C0,
                BUY_0065D2B0,
                BUY_0065D3E0,
                BUY_0065D4E0,
                BUY_0065D5E0,
                TIDY_0065D240,
                TIDY_0065D330,
                TIDY_0065D460,
                TIDY_0065D560,
                TIDY_0065D660,
                OPERATOR_DELETE,
            ],
        );
        e.register(ALLOCATOR_CONSTRUCT, |_, a| answer(a[0]));
        let tri = e.mem.alloc(0xf0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_a4c0, &args![tri]).u32(), tri);
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT_0065C630),
            vec![vec![tri], vec![tri + 0x18], vec![tri + 0x60]]
        );
        let bases = calls(&e, VECTOR_BASE_CONSTRUCT);
        assert_eq!(
            bases,
            vec![
                vec![tri + 0x48, tri + 0x48],
                vec![tri + 0x90, tri + 0x90],
                vec![tri + 0xa8, tri + 0xa8],
                vec![tri + 0xc0, tri + 0xc0],
                vec![tri + 0xd8, tri + 0xd8]
            ]
        );
        assert_eq!(calls(&e, BUY_0065D5E0), vec![vec![tri + 0xd8, 0]]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_a650, &args![tri, 1u32]).u32(), tri);
        assert_eq!(calls(&e, TIDY_0065D660), vec![vec![tri + 0xd8]]);
        assert_eq!(calls(&e, TIDY_0065D240), vec![vec![tri + 0x48]]);
        assert_eq!(calls(&e, VECTOR_BASE_DESTRUCT).len(), 5);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT_0065C6A0),
            vec![vec![tri + 0x60], vec![tri + 0x18], vec![tri]]
        );
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![tri]]);
        // Each vector constructor and destructor on its own.
        for (construct, buy, destruct, tidy) in [
            (0x0065_c820u32, BUY_0065D1C0, 0x0065_c890u32, TIDY_0065D240),
            (0x0065_c8f0, BUY_0065D2B0, 0x0065_c960, TIDY_0065D330),
            (0x0065_c9c0, BUY_0065D3E0, 0x0065_ca30, TIDY_0065D460),
            (0x0065_ca90, BUY_0065D4E0, 0x0065_cb00, TIDY_0065D560),
            (0x0065_cb60, BUY_0065D5E0, 0x0065_cbd0, TIDY_0065D660),
        ] {
            let vector = e.mem.alloc(0x18);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(construct, &args![vector]).u32(), vector);
            assert_eq!(calls(&e, buy), vec![vec![vector, 0]]);
            e.call(destruct, &args![vector]);
            assert_eq!(calls(&e, tidy), vec![vec![vector]]);
            assert_eq!(calls(&e, VECTOR_BASE_DESTRUCT), vec![vec![vector]]);
        }
    }

    #[test]
    fn model_vertex_count_is_the_smallest_morph() {
        let mut e = Engine::new();
        let morphs = e.mem.alloc(0x38 * 3);
        for (i, count) in [7u32, 4, 9].into_iter().enumerate() {
            e.mem.set_u32(morphs + 0x38 * i as u32 + 0x1c, count);
        }
        e.register(TRI_VERTEX_COUNT, |_, _| answer(6));
        e.register(TRI_MORPH_COUNT, |_, _| answer(3));
        e.register_double(TRI_MORPH, move |_, a| answer(morphs + 0x38 * a[1]));
        let tri = e.mem.alloc(0xf0);
        assert_eq!(e.call(0x0065_a5a0, &args![tri]).u32(), 4);
        e.register(TRI_MORPH_COUNT, |_, _| answer(0));
        assert_eq!(e.call(0x0065_a5a0, &args![tri]).u32(), 6);
    }

    #[test]
    fn small_helpers() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(
            &mut e,
            &[
                SET_CONSISTENCY,
                STRIDED_CHECK,
                ITERATOR_STEP_0065DAE0,
                ITERATOR_STEP_0065DB40,
                DESTROY_RANGE,
                DEALLOCATE,
            ],
        );
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(geometry + 0xb8, 0x0b80);
        e.call_log = Some(vec![]);
        e.call(0x0065_a620, &args![geometry, 2u32]);
        assert_eq!(calls(&e, SET_CONSISTENCY), vec![vec![0x0b80, 2]]);
        // The strided store.
        let array = e.mem.alloc(8);
        let storage = e.mem.alloc(0x40);
        e.mem.set_u32(array, storage);
        e.mem.set_u32(array + 4, 0x10);
        let point = e.mem.alloc(12);
        for word in 0..3u32 {
            e.mem.set_u32(point + 4 * word, 0x20 + word);
        }
        e.call(0x0065_b390, &args![array, 2u32, point]);
        assert_eq!(calls(&e, STRIDED_CHECK), vec![vec![array, 2]]);
        assert_eq!(e.mem.u32(storage + 0x28), 0x22);
        // The word picker.
        let words = e.mem.alloc(0x18);
        e.mem.set_u32(words + 0x10, 1);
        e.mem.set_u32(words + 0x14, 2);
        assert_eq!(e.call(0x0065_b3e0, &args![words, 0u32]).u32(), 1);
        assert_eq!(e.call(0x0065_b3e0, &args![words, 1u32]).u32(), 2);
        // The iterator steps.
        let iterator = e.mem.alloc(8);
        e.mem.set_u32(iterator, 0x11);
        e.mem.set_u32(iterator + 4, 0x12);
        let result = e.mem.alloc(8);
        assert_eq!(
            e.call(0x0065_cf50, &args![iterator, result, 0u32]).u32(),
            result
        );
        assert_eq!(e.mem.u32(result + 4), 0x12);
        assert_eq!(calls(&e, ITERATOR_STEP_0065DAE0), vec![vec![iterator]]);
        assert_eq!(e.call(0x0065_cf90, &args![iterator]).u32(), iterator);
        assert_eq!(calls(&e, ITERATOR_STEP_0065DB40), vec![vec![iterator]]);
        // The vector's tidy.
        let vector = e.mem.alloc(0x18);
        e.mem.set_u32(vector + 0xc, 0x1000);
        e.mem.set_u32(vector + 0x10, 0x1008);
        e.mem.set_u32(vector + 0x14, 0x1010);
        e.call(0x0065_f240, &args![vector]);
        assert_eq!(calls(&e, DESTROY_RANGE), vec![vec![vector, 0x1000, 0x1008]]);
        assert_eq!(calls(&e, DEALLOCATE), vec![vec![vector + 8, 0x1000, 4]]);
        assert_eq!(e.mem.u32(vector + 0x14), 0);
        e.call_log = Some(vec![]);
        e.call(0x0065_f240, &args![vector]);
        assert!(calls(&e, DESTROY_RANGE).is_empty());
    }

    #[test]
    fn new_mesh_clones_the_model_and_tags_it() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(
            &mut e,
            &[
                ADD_EXTRA_DATA,
                SKIN_SET_DATA,
                EXTRA_DATA_CONSTRUCT,
                0x0000_7de4,
            ],
        );
        e.register(GEOMETRY_DATA, |e, a| answer(e.mem.u32(a[0] + 0xb8)));
        let clone = object_with(&mut e, 0xc0, &[(0xe4, 0x0000_7de4)]);
        e.register_double(CLONE, move |_, _| answer(clone));
        e.register(CREATE_DEEP_COPY, |e, a| {
            e.mem.set_u32(a[1], a[0] + 1);
            Ret::default()
        });
        e.register(SKIN_INSTANCE, |e, a| answer(e.mem.u32(a[0] + 0xbc)));
        e.register(SKIN_DATA_0043B230, |e, a| answer(e.mem.u32(a[0] + 0xc)));
        e.register(NI_ALLOCATE, |e, a| answer(e.mem.alloc(a[0])));
        e.register(MORPH_EXTRA_DATA_CONSTRUCT, |_, a| answer(a[0]));
        e.register(EXTRA_DATA_NAME, |_, _| answer(0x0000_0aaa));
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(geometry + 0xb8, 0x0000_0d00);
        let skin = e.mem.alloc(0x14);
        e.mem.set_u32(skin + 0xc, 0x0000_0500);
        e.mem.set_u32(clone + 0xbc, skin);
        let mesh = e.mem.alloc(0x24);
        e.mem.set_u32(mesh + 0x10, geometry);
        e.mem.set_u32(mesh + 0x14, 0x0000_0e00);
        e.mem.set_u32(mesh + 0x18, 3);
        let model = e.mem.alloc(0x10);
        e.mem.set_u32(model + 8, mesh);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0065_bbc0, &args![model, out]).bool());
        assert_eq!(e.mem.u32(out), clone);
        assert_eq!(calls(&e, 0x0000_7de4), vec![vec![clone, 0x0d01]]);
        assert_eq!(calls(&e, SKIN_SET_DATA), vec![vec![skin, 0x0501]]);
        let morph = calls(&e, MORPH_EXTRA_DATA_CONSTRUCT);
        assert_eq!(morph[0][1..], [clone, 0x0e00, 3]);
        let added = calls(&e, ADD_EXTRA_DATA);
        assert_eq!(added.len(), 2);
        let extra = added[1][1];
        assert_eq!(e.mem.u32(extra), MODEL_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.u32(extra + 0xc), model);
        assert_eq!(calls(&e, EXTRA_DATA_CONSTRUCT), vec![vec![extra, 0x0aaa]]);
        // Without geometry data: 0, the pointer cleared.
        e.mem.set_u32(geometry + 0xb8, 0);
        e.mem.set_u32(out, 0x1234);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0065_bbc0, &args![model, out]).bool());
        assert_eq!(e.mem.u32(out), 0);
        assert!(calls(&e, CLONE).is_empty());
    }

    #[test]
    fn coordinates_make_and_morph_a_new_mesh() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        e.register(GEOMETRY_DATA, |_, _| answer(0));
        let model = e.mem.alloc(0x10);
        let out = e.mem.alloc(4);
        assert!(!e.call(0x0065_a910, &args![model, 0u32, out]).bool());
        // No mesh data: CreateNewMesh fails.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0065_a910, &args![model, 1u32, out]).bool());
        assert!(calls(&e, GET_FACE_GEN_COORD).is_empty());
    }

    struct Morph {
        e: Engine,
        model: u32,
        geometry: u32,
        storage: u32,
    }

    /// One morph set with one morph of `vertices` offsets (each (1, 2, 3)), scale 2, and a
    /// geometry of `vertices` points stored with stride 12.
    fn morph_rig(vertices: u32, coefficient: f32) -> Morph {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(
            &mut e,
            &[
                FACE_GEN_LOCK,
                FACE_GEN_UNLOCK,
                VERTEX_ITERATOR_CONSTRUCT,
                ALLOCATOR_CONSTRUCT,
                STRIDED_CHECK,
                GEOMETRY_DATA_UNLOCK,
                GEOMETRY_DATA_MARK_AS_CHANGED,
                ITERATOR_STEP_0065DAE0,
            ],
        );
        e.register(MESH_LOADED_TEST, |_, _| answer(1));
        e.register(MORPH_EXTRA_DATA, |_, _| answer(0));
        e.register(GEOMETRY_DATA, |_, _| answer(0x0000_0d00));
        e.register(GEOMETRY_DATA_LOCK, |_, _| answer(1));
        let storage = e.mem.alloc(12 * vertices);
        for i in 0..3 * vertices {
            e.mem.set_f32(storage + 4 * i, 10.0);
        }
        e.register_double(GEOMETRY_DATA_VERTICES, move |e, a| {
            e.mem.set_u32(a[1], storage);
            e.mem.set_u32(a[1] + 4, 12);
            Ret::default()
        });
        e.register(ITERATOR_STRIDE_00717E50, |_, a| answer(a[0] + 4));
        e.register(VERTEX_ITERATOR_GET, |e, a| {
            let base = e.mem.u32(a[0]) + 12 * a[1];
            for word in 0..3u32 {
                let value = e.mem.u32(base + 4 * word);
                e.mem.set_u32(a[2] + 4 * word, value);
            }
            Ret::default()
        });
        e.register_double(GEOMETRY_VERTEX_COUNT, move |_, _| answer(vertices));
        e.register(GEOMETRY_0049EC60, |_, _| answer(1));
        e.register(GEOMETRY_DATA_KEEP_FLAGS, |_, _| answer(0));
        // The EGM data: set 0 with one morph, set 1 empty.
        let morph = e.mem.alloc(0x10);
        e.mem.set_f32(morph, 2.0);
        e.mem.set_u32(morph + 4, vertices);
        let offsets = e.mem.alloc(6 * vertices);
        for i in 0..vertices {
            e.mem.set_i16(offsets + 6 * i, 1);
            e.mem.set_i16(offsets + 6 * i + 2, 2);
            e.mem.set_i16(offsets + 6 * i + 4, 3);
        }
        let egm = e.mem.alloc(0x40);
        e.register_double(MORPH_SET_SIZE, move |_, a| answer((a[0] == egm + 4) as u32));
        e.register_double(MORPH_SET_MORPH, move |_, _| answer(morph));
        e.register(MORPH_VERTEX_COUNT, |e, a| answer(e.mem.u32(a[0])));
        e.register_double(VECTOR_BEGIN, move |e, a| {
            e.mem.set_u32(a[1], offsets);
            answer(a[1])
        });
        e.register(ITERATOR_DEREF, |e, a| answer(e.mem.u32(a[0])));
        // The post-increment step moves the iterator word by one offset.
        e.register(ITERATOR_STEP_0065DAE0, |e, a| {
            let value = e.mem.u32(a[0]) + 6;
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        let value = e.mem.alloc(4);
        e.mem.set_f32(value, coefficient);
        e.register(COORD_VALUE, |_, _| answer(0x0000_0c00));
        e.register_double(COORD_ADDRESS, move |_, _| answer(value));
        e.map(ZERO, 8);
        let mesh = e.mem.alloc(0x24);
        e.mem.set_u32(mesh + 8, egm);
        let model = e.mem.alloc(0x10);
        e.mem.set_u32(model + 8, mesh);
        Morph {
            e,
            model,
            geometry: 0x0000_0900,
            storage,
        }
    }

    #[test]
    fn morphs_move_the_vertices_by_their_weight() {
        // Six vertices: one group of four and two more; weight 0.5 * 2 = 1.
        let mut m = morph_rig(6, 0.5);
        m.e.call_log = Some(vec![]);
        assert!(m
            .e
            .call(0x0065_a970, &args![m.model, 0x0c00u32, m.geometry, 0u8])
            .bool());
        let values: Vec<f32> = (0..18).map(|i| m.e.mem.f32(m.storage + 4 * i)).collect();
        assert_eq!(values, [11.0, 12.0, 13.0].repeat(6));
        assert_eq!(calls(&m.e, GEOMETRY_DATA_MARK_AS_CHANGED).len(), 1);
        assert_eq!(calls(&m.e, FACE_GEN_UNLOCK).len(), 1);
        // A zero weight leaves them, unless the byte asks for every morph.
        let mut m = morph_rig(4, 0.0);
        m.e.call(0x0065_a970, &args![m.model, 0x0c00u32, m.geometry, 0u8]);
        assert_eq!(m.e.mem.f32(m.storage), 10.0);
        let mut m = morph_rig(4, 0.0);
        m.e.call_log = Some(vec![]);
        m.e.call(0x0065_a970, &args![m.model, 0x0c00u32, m.geometry, 1u8]);
        assert_eq!(m.e.mem.f32(m.storage), 10.0);
        assert_eq!(calls(&m.e, VERTEX_ITERATOR_GET).len(), 4);
    }

    #[test]
    fn morphs_need_mesh_data_coordinates_and_vertices() {
        let mut m = morph_rig(4, 1.0);
        m.e.register(GET_FACE_GEN_COORD, |_, _| answer(0));
        assert!(!m
            .e
            .call(0x0065_a970, &args![m.model, 0u32, m.geometry, 0u8])
            .bool());
        let mut m = morph_rig(4, 1.0);
        m.e.register(GEOMETRY_DATA_LOCK, |_, _| answer(0));
        m.e.call_log = Some(vec![]);
        assert!(!m
            .e
            .call(0x0065_a970, &args![m.model, 0x0c00u32, m.geometry, 0u8])
            .bool());
        assert_eq!(calls(&m.e, FACE_GEN_UNLOCK).len(), 1);
        let mut m = morph_rig(4, 1.0);
        m.e.mem.set_u32(m.model + 8, 0);
        assert!(!m
            .e
            .call(0x0065_a970, &args![m.model, 0x0c00u32, m.geometry, 0u8])
            .bool());
        // EGM data still to load, but no file name.
        let mut m = morph_rig(4, 1.0);
        m.e.register(MESH_LOADED_TEST, |_, _| answer(0));
        let mesh = m.e.mem.u32(m.model + 8);
        m.e.mem.set_u32(mesh + 8, 0);
        m.e.register(STRING_HAS_TEXT, |_, _| answer(0));
        assert!(!m
            .e
            .call(0x0065_a970, &args![m.model, 0x0c00u32, m.geometry, 0u8])
            .bool());
    }

    #[test]
    fn coordinate_texturing_sums_the_texture_morphs() {
        let mut e = Engine::new();
        plain_pointers(&mut e);
        quiet(
            &mut e,
            &[
                FACE_GEN_LOCK,
                FACE_GEN_UNLOCK,
                ITERATOR_STEP_0065DB40,
                IMAGE_SET_WIDTH,
                IMAGE_SET_HEIGHT,
                OPERATOR_DELETE,
                MEMSET,
                FR2_ASSERT,
            ],
        );
        e.register(DEFAULT_MODULATION_TEXTURE, |_, _| answer(0x0000_0def));
        e.register(TEXTURE_LOADED_TEST, |_, _| answer(1));
        // One EGT morph, 2 x 1 texels, scale 1, channel bytes (1, -2) for each channel.
        let morph = e.mem.alloc(0x60);
        e.mem.set_f32(morph, 1.0);
        e.mem.set_u32(morph + 4, 2);
        e.mem.set_u32(morph + 8, 1);
        e.mem.set_u32(morph + 0xc, 0);
        let bytes = e.mem.alloc(4);
        e.mem.set_i8(bytes, 1);
        e.mem.set_i8(bytes + 1, -2);
        let egt = e.mem.alloc(0x40);
        e.register_double(EGT_SET_SIZE, move |_, a| answer((a[0] == egt + 4) as u32));
        e.register_double(EGT_SET_MORPH, move |_, _| answer(morph));
        e.register_double(EGT_CHANNEL_DATA, move |_, _| answer(bytes));
        e.register(COORD_TEXTURE_COUNT, |_, _| answer(1));
        let value = e.mem.alloc(4);
        e.mem.set_f32(value, 0.5);
        e.register(COORD_VALUE, |_, _| answer(0x0000_0c00));
        e.register_double(COORD_ADDRESS, move |_, _| answer(value));
        e.map(COEFFICIENT_SCALE, 8);
        e.set_global(COEFFICIENT_SCALE, 256.0f64);
        e.register(FTOL, |_, a| {
            answer(f64::from_bits((a[0] as u64) | ((a[1] as u64) << 32)) as i32 as u32)
        });
        e.register(OPERATOR_NEW, |e, a| answer(e.mem.alloc(a[0])));
        e.register(NI_ALLOCATE, |e, a| answer(e.mem.alloc(a[0])));
        e.register(IMAGE_CONSTRUCT, |_, a| answer(a[0]));
        let colors = e.mem.alloc(0x20);
        e.register_double(IMAGE_COLORS, move |_, _| answer(colors));
        let texels = e.mem.alloc(0x20);
        e.register(IMAGE_RESIZE, |_, _| Ret::default());
        e.register_double(VECTOR_BEGIN, move |e, a| {
            e.mem.set_u32(a[1], texels);
            answer(a[1])
        });
        e.register(ITERATOR_ELEMENT, |e, a| answer(e.mem.u32(a[0])));
        for address in [
            COLOR_SCALE_01065B9C,
            IMAGE_SCALE_01065B98,
            IMAGE_SCALE_01023CD8,
            IMAGE_SCALE_01016248,
        ] {
            e.map(address, 4);
        }
        e.set_global(COLOR_SCALE_01065B9C, 0.5f32);
        e.register(GET_COMPRESSED_IMAGE, |_, _| answer(0x0000_0ccc));
        let texture = e.mem.alloc(0xc);
        e.mem.set_u32(texture + 8, egt);
        let model = e.mem.alloc(0x10);
        e.mem.set_u32(model + 0xc, texture);
        let out = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0065_b410, &args![model, 0x0c00u32, out, 0u32])
            .bool());
        // ftol(0.5 * 256) = 128, ftol(1 * 256) = 256: texel sums 128 * 256 * (1, -2), halved.
        let sums: Vec<f32> = (0..8).map(|i| e.mem.f32(texels + 4 * i)).collect();
        let a = 128.0 * 256.0 * 0.5;
        assert_eq!(sums, vec![a, a, a, 0.0, -2.0 * a, -2.0 * a, -2.0 * a, 0.0]);
        assert_eq!(e.mem.u32(out), 0x0ccc);
        assert!(calls(&e, FR2_ASSERT).is_empty());
        assert_eq!(calls(&e, MEMSET)[0][2], 2 * 4 * 3);
        // Without texture data only the default texture is set.
        e.mem.set_u32(model + 0xc, 0);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0065_b410, &args![model, 0x0c00u32, out, 0u32])
            .bool());
        assert_eq!(e.mem.u32(out), 0x0def);
        assert!(calls(&e, FACE_GEN_LOCK).is_empty());
        // Too few coordinate texture morphs: a message and 0.
        e.mem.set_u32(model + 0xc, texture);
        e.register(COORD_TEXTURE_COUNT, |_, _| answer(0));
        quiet(&mut e, &[PRINT_MESSAGE]);
        e.call_log = Some(vec![]);
        assert!(!e
            .call(0x0065_b410, &args![model, 0x0c00u32, out, 0u32])
            .bool());
        assert_eq!(calls(&e, PRINT_MESSAGE), vec![vec![TEXTURE_COUNT_MESSAGE]]);
        assert_eq!(calls(&e, FACE_GEN_UNLOCK).len(), 1);
    }

    #[test]
    fn egt_name_from_the_mesh_name() {
        let mut e = Engine::new();
        let model = Ptr::<BSFaceGenModel>::new(e.mem.alloc(0x10));
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"head.nif");
        e.map(EGT_EXTENSION, 8);
        e.mem.set_cstr(EGT_EXTENSION, b".egt");
        e.register_double(MESH_NAME_006505D0, move |_, _| name.into_ret());
        e.register(STRCPY_S_00406D30, |e, a| {
            let s = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &s);
            Ret::default()
        });
        e.register(STRRCHR, |e, a| {
            let s = e.mem.cstr(a[0]);
            let found = s.iter().rposition(|&c| u32::from(c) == a[1]);
            found.map_or(0, |i| a[0] + i as u32).into_ret()
        });
        e.register(STRCAT_S_00406D50, |e, a| {
            let mut s = e.mem.cstr(a[0]);
            s.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &s);
            Ret::default()
        });
        for addr in [
            TIMER_SCOPE_CONSTRUCT,
            TIMER_SCOPE_DESTROY,
            TEXTURE_LOAD_0065C2C0,
            OPERATOR_NEW,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        // The name the texture data takes.
        e.register(STRING_SET, |e, a| {
            assert_eq!(e.mem.cstr(a[1]), b"head.egt");
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0065_a840, &args![model]);
        assert_eq!(calls(&e, STRING_SET).len(), 1);
        assert_eq!(calls(&e, TEXTURE_LOAD_0065C2C0), vec![vec![model.addr()]]);
        // No dot: nothing more after strrchr.
        e.mem.set_cstr(name, b"head");
        e.call_log = Some(vec![]);
        e.call(0x0065_a840, &args![model]);
        assert_eq!(calls(&e, STRRCHR).len(), 1);
        assert!(calls(&e, TEXTURE_LOAD_0065C2C0).is_empty());
        // No name, or texture data already: nothing.
        e.register(MESH_NAME_006505D0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0065_a840, &args![model]);
        assert!(calls(&e, STRCPY_S_00406D30).is_empty());
        e.set(model, BSFaceGenModel::pTextureData, 0x100);
        e.call_log = Some(vec![]);
        e.call(0x0065_a840, &args![model]);
        assert!(calls(&e, MESH_NAME_006505D0).is_empty());
    }

    #[test]
    fn face_check_reads_matrix_values_and_names_without_writing() {
        let mut e = Engine::new();
        let matrix = e.mem.alloc(0x20);
        let floats = e.mem.alloc(12);
        for (i, v) in [0.5f32, -2.0, 1.25].iter().enumerate() {
            e.mem.set_f32(floats + 4 * i as u32, *v);
        }
        e.mem.set_u32(matrix + 0xc, floats);
        e.mem.set_u32(matrix + 0x18, 3);
        e.mem.set_u32(matrix + 0x1c, 1);
        assert_eq!(face_check::matrix_values(&e, matrix), vec![0.5, -2.0, 1.25]);
        e.mem.set_u32(matrix + 0xc, 0);
        assert!(face_check::matrix_values(&e, matrix).is_empty());
        let object = e.mem.alloc(0x10);
        let text = e.mem.alloc(16);
        for (i, b) in b"FaceGenFace\0".iter().enumerate() {
            e.mem.set_u8(text + i as u32, *b);
        }
        e.mem.set_u32(object + 8, text);
        assert_eq!(face_check::name(&e, object), "FaceGenFace");
        assert_eq!(face_check::name(&e, 0), "-");
    }
}
