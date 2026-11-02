//! `NiMain/nitextureeffect.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiTextureEffect::ms_RTTI` (the class's `NiRTTI` record).
const NI_TEXTURE_EFFECT_RTTI: u32 = 0x011f_5e94;

// Translated from 00a927d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_texture_effect_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_TEXTURE_EFFECT_RTTI)
}

use crate::units::nimain::nimainsdm::set_literal_fixed_string;

/// The static fixed strings `00a927e0` sets: (global, address of the literal), in order.
const STATIC_STRINGS: [(u32, u32); 4] = [
    (0x011f_5e9c, 0x0109_fe10),
    (0x011f_5ea0, 0x0109_fe00),
    (0x011f_5ea4, 0x0109_fdf0),
    (0x011f_5ea8, 0x0109_fde8),
];

// Translated from 00a927e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::_SDMInit` (Xbox PDB; run by the start-up `00a89cc0`, no arguments):
/// sets four static fixed strings from literals (`STATIC_STRINGS`), each only when the
/// global does not hold the literal's address already.
pub fn ni_texture_effect_sdm_init(e: &mut Engine) {
    for (slot, literal) in STATIC_STRINGS {
        set_literal_fixed_string(e, slot, literal);
    }
}

/// `00a5b690` (`cdecl`, one word): the fixed-string handle for a C string (null gives the
/// null handle).
const FIXED_STRING_FROM_TEXT: u32 = 0x00a5_b690;
/// Import slot of `InterlockedDecrement` (`stdcall`, one word: the address of the count).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The handle of the empty `NiFixedString`, which is never counted.
const EMPTY_STRING_HANDLE: u32 = 0x0109_b220;

// Translated from 00a928b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `nitextureeffect.cpp`, no arguments; one of the `NiMain`
/// shutdown steps of `00a8a280`): for each of the four static fixed strings that
/// `NiTextureEffect::_SDMInit` sets (`011f5e9c`, `011f5ea0`, `011f5ea4`, `011f5ea8`, in that
/// order) that is not null, stores the handle `00a5b690(0)` gives and then releases the old
/// handle (`InterlockedDecrement` of its count 8 bytes before it) unless it is the empty
/// string's. The decompiler shows the empty-string comparison as a comparison with 0; the
/// disassembly compares with the global `0109b220`.
pub fn fn_00a928b0(e: &mut Engine) {
    for (slot, _) in STATIC_STRINGS {
        let old = e.mem.u32(slot);
        if old != 0 {
            let handle = e.call(FIXED_STRING_FROM_TEXT, &args![0u32]).u32();
            e.mem.set_u32(slot, handle);
            if old != e.global::<u32>(EMPTY_STRING_HANDLE) {
                e.call(INTERLOCKED_DECREMENT, &args![old.wrapping_sub(8)]);
            }
        }
    }
}

use crate::types::NiTArray;
use crate::units::nimain::nimainsdm::{
    assign_ref_counted, interlocked_decrement, interlocked_increment,
};
use crate::units::nimain::nistream::{NiBinaryStream, NiStream};

/// `NiTextureEffect` fields at their PC offsets (the Xbox PDB names them; its offsets are
/// 0x2C higher, from the 16-byte-aligned transforms of `NiAVObject`): `m_kModelProjMat`
/// +0xC4 (nine floats), `m_kModelProjTrans` +0xE8, `m_kWorldProjMat` +0xF4,
/// `m_kWorldProjTrans` +0x118, `m_spTexture` +0x124, `m_eFilter` +0x128, `m_eClamp` +0x12C,
/// `m_eTextureMode` +0x130, `m_eCoordMode` +0x134, `m_bPlaneEnable` +0x138,
/// `m_kModelPlane` +0x13C (normal, constant) and `m_kWorldPlane` +0x14C; 0x15C bytes.
const MODEL_PROJ_MAT: u32 = 0xc4;
const MODEL_PROJ_TRANS: u32 = 0xe8;
const WORLD_PROJ_MAT: u32 = 0xf4;
const WORLD_PROJ_TRANS: u32 = 0x118;
const TEXTURE: u32 = 0x124;
const FILTER: u32 = 0x128;
const CLAMP: u32 = 0x12c;
const TEXTURE_MODE: u32 = 0x130;
const COORD_MODE: u32 = 0x134;
const PLANE_ENABLE: u32 = 0x138;
const MODEL_PLANE: u32 = 0x13c;
const WORLD_PLANE: u32 = 0x14c;
const NI_TEXTURE_EFFECT_SIZE: u32 = 0x15c;
/// `NiAVObject` world transform at its PC offsets: rotation +0x68 (nine floats, row by
/// row), translation +0x8C, scale +0x98.
const WORLD_ROTATE: u32 = 0x68;
const WORLD_TRANSLATE: u32 = 0x8c;
const WORLD_SCALE: u32 = 0x98;
/// `NiDynamicEffect` bytes the constructor and the projection update touch: the effect
/// type byte +0x9D and the revision counter +0xA8 (Xbox PDB `m_ucEffectType`,
/// `m_uiRevID`, at PC offsets).
const EFFECT_TYPE: u32 = 0x9d;
const REVISION: u32 = 0xa8;
/// The class vtable (`0109fd0c`).
const NI_TEXTURE_EFFECT_VTABLE: u32 = 0x0109_fd0c;
/// The nine floats the constructor copies into both projection matrices (`011a9448`) and
/// the point it gives both planes as their normal (`011a946c`).
const CONSTRUCT_MATRIX: u32 = 0x011a_9448;
const CONSTRUCT_PLANE_NORMAL: u32 = 0x011a_946c;
/// `0.5` (`float`, `01016248`), `0.5` and `-0.5` (`double`s, `01011588`, `01011598`): the
/// sphere-map projection constants.
const HALF_FLOAT: u32 = 0x0101_6248;
const HALF_DOUBLE: u32 = 0x0101_1588;
const MINUS_HALF_DOUBLE: u32 = 0x0101_1598;
/// The global object at `011f4748`: when set, the destructor calls its slot +0xF8 with the
/// effect.
const GLOBAL_011F4748: u32 = 0x011f_4748;
const GLOBAL_EFFECT_SLOT: u32 = 0xf8;
/// `NiNew` allocator (`00aa13e0`, cdecl: size), `NiAlloc` (`00aa1070`, cdecl: size) and the
/// sized delete (`00aa1460`, cdecl `(pointer, size)`).
const NI_NEW: u32 = 0x00aa_13e0;
const NI_ALLOC: u32 = 0x00aa_1070;
const SIZED_DELETE: u32 = 0x00aa_1460;
/// `NiStricmp` (`00aa1360`, cdecl: two strings; 0 when equal) and `NiSprintf` (`00a305b0`,
/// cdecl: buffer, size, format, then the format words).
const NI_STRICMP: u32 = 0x00aa_1360;
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `NiDynamicEffect`: the constructor (`00a69320`), the destructor body `00a69030`
/// (unnamed), `LinkObject` (`00a69070`), the member copy `00a68d40` (unnamed),
/// `IsEqual` (`00a68d80`), `LoadBinary` (`00a68ea0`), `SaveBinary` (`00a68f80`) and
/// `GetViewerStrings` (`00a69140`); `NiAVObject::UpdateWorldData` (`00a68c60`); the base
/// `RegisterStreamables` `00a6ff10` (five bytes; the engine map name
/// `NiCamera::RegisterStreamables` is a folded body). All thiscall.
const DYNAMIC_EFFECT_CONSTRUCT: u32 = 0x00a6_9320;
const DYNAMIC_EFFECT_DESTRUCT_00A69030: u32 = 0x00a6_9030;
const DYNAMIC_EFFECT_LINK_OBJECT: u32 = 0x00a6_9070;
const DYNAMIC_EFFECT_COPY_MEMBERS_00A68D40: u32 = 0x00a6_8d40;
const DYNAMIC_EFFECT_IS_EQUAL: u32 = 0x00a6_8d80;
const DYNAMIC_EFFECT_LOAD_BINARY: u32 = 0x00a6_8ea0;
const DYNAMIC_EFFECT_SAVE_BINARY: u32 = 0x00a6_8f80;
const DYNAMIC_EFFECT_GET_VIEWER_STRINGS: u32 = 0x00a6_9140;
const AV_OBJECT_UPDATE_WORLD_DATA: u32 = 0x00a6_8c60;
const BASE_REGISTER_STREAMABLES_00A6FF10: u32 = 0x00a6_ff10;
/// `NiPlane::NiPlane_ov2` (`00a69970`, thiscall: the normal, the constant `float`).
const PLANE_CONSTRUCT: u32 = 0x00a6_9970;
/// `NiStream::GetObjectFromLinkID` (`00a63430`, thiscall on the stream) and the stream
/// call `00a645b0` (unnamed; thiscall on the stream, no arguments) `LoadBinary` makes
/// for the texture link; the stream slot +0x30 (`SaveLinkID` in the Xbox PDB order).
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
const STREAM_00A645B0: u32 = 0x00a6_45b0;
const STREAM_SAVE_LINK_ID_SLOT: u32 = 0x30;
/// The matrix, point and plane helpers: `NiMatrix3::LoadBinary` (`00a59a30`),
/// `SaveBinary` (`00a59a90`), `operator==` (`004d9ae0`), `Transpose` (`004768c0`: the
/// result), `operator*` (`0043f8d0`: the result, the other matrix), `operator*_ov2`
/// (`00a85520`: the result, a `float`) and `GetViewerString` (`00a598f0`: the name);
/// the matrix-point product `004b4500` (unnamed; the result, the point); the point
/// `LoadBinary`/`SaveBinary` (`00a59be0`/`00a69780`, engine map `NiColor::`, folded), its
/// difference test `00439090` and `NiPoint3::GetViewerString` (`00a59af0`); the plane
/// `LoadBinary`/`SaveBinary` (`00a69ac0`/`00eb5410`, engine map `NiSphere::`, folded), its
/// difference test `00a926b0` (unnamed: nonzero when a component differs) and
/// `NiPlane::GetViewerString` (`00a69a60`). All thiscall on the object.
const MATRIX_LOAD_BINARY: u32 = 0x00a5_9a30;
const MATRIX_SAVE_BINARY: u32 = 0x00a5_9a90;
const MATRIX_EQUAL: u32 = 0x004d_9ae0;
const MATRIX_TRANSPOSE: u32 = 0x0047_68c0;
const MATRIX_TIMES_MATRIX: u32 = 0x0043_f8d0;
const MATRIX_TIMES_FLOAT: u32 = 0x00a8_5520;
const MATRIX_GET_VIEWER_STRING: u32 = 0x00a5_98f0;
const MATRIX_TIMES_POINT_004B4500: u32 = 0x004b_4500;
const POINT_LOAD_BINARY: u32 = 0x00a5_9be0;
const POINT_SAVE_BINARY: u32 = 0x00a6_9780;
const POINT_NOT_EQUAL_00439090: u32 = 0x0043_9090;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
const PLANE_LOAD_BINARY: u32 = 0x00a6_9ac0;
const PLANE_SAVE_BINARY: u32 = 0x00eb_5410;
const PLANE_NOT_EQUAL_00A926B0: u32 = 0x00a9_26b0;
const PLANE_GET_VIEWER_STRING: u32 = 0x00a6_9a60;
/// `NiTexturingProperty`'s clamp and filter strings (`00a69b90`, `00a69c50`; cdecl: a name
/// and the mode).
const TEXTURING_CLAMP_STRING: u32 = 0x00a6_9b90;
const TEXTURING_FILTER_STRING: u32 = 0x00a6_9c50;
/// The texture slots this file calls: `RegisterStreamables` +0x54.
const SLOT_REGISTER_STREAMABLES: u32 = 0x54;
/// `NiGetViewerString` (cdecl, the class name) and `NiGetViewerString_ov2` (cdecl, a name
/// and a `bool` byte).
const VIEWER_STRING_NAME: u32 = 0x00a7_d330;
const VIEWER_STRING_BOOL: u32 = 0x00a7_d370;
/// The strings array `SetSize` (thiscall, the new size) and `SetAt` (thiscall, the index
/// and the address of the element).
const STRING_ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const STRING_ARRAY_SET_AT: u32 = 0x0096_ae90;
/// The file version below which `LoadBinary` skips two 2-byte words, and the one below
/// which it skips one more.
const VERSION_TWO_OLD_WORDS: u32 = 0x0a03_0004;
const VERSION_ONE_OLD_WORD: u32 = 0x0401_0010;
/// `NiStream::m_uiNifFileVersion` (Xbox PDB) +0xD8.
const STREAM_FILE_VERSION: u32 = 0xd8;
/// The formats of the texture-mode strings (`"%s = PROJECTED_LIGHT"`,
/// `"%s = PROJECTED_SHADOW"`, `"%s = ENVIRONMENT_MAP"`, `"%s = FOG_MAP"`) and of the
/// coordinate-mode strings (`"%s = WORLD_PARALLEL"`, `"%s = WORLD_PERSPECTIVE"`,
/// `"%s = SPHERE_MAP"`, `"%s = SPECULAR_CUBE_MAP"`, `"%s = DIFFUSE_CUBE_MAP"`,
/// `"%s = UNKNOWN!!!"`).
const FORMAT_PROJECTED_LIGHT: u32 = 0x0109_fe60;
const FORMAT_PROJECTED_SHADOW: u32 = 0x0109_fe48;
const FORMAT_ENVIRONMENT_MAP: u32 = 0x0109_fe30;
const FORMAT_FOG_MAP: u32 = 0x0109_fe20;
const FORMAT_WORLD_PARALLEL: u32 = 0x0109_fed0;
const FORMAT_WORLD_PERSPECTIVE: u32 = 0x0109_feb8;
const FORMAT_SPHERE_MAP: u32 = 0x0109_fea8;
const FORMAT_SPECULAR_CUBE_MAP: u32 = 0x0109_fe90;
const FORMAT_DIFFUSE_CUBE_MAP: u32 = 0x0109_fe78;
const FORMAT_UNKNOWN: u32 = 0x0109_b2ec;
/// The viewer-string names: `"m_kModelProjMat"`, `"m_kModelProjTrans"`,
/// `"m_eTextureMode"`, `"m_eFilter"`, `"m_eClamp"`, `"m_eCoordMode"`, `"m_bPlaneEnable"`
/// and `"m_kModelPlane"`.
const TEXT_MODEL_PROJ_MAT: u32 = 0x0109_ff50;
const TEXT_MODEL_PROJ_TRANS: u32 = 0x0109_ff3c;
const TEXT_TEXTURE_MODE: u32 = 0x0109_ff2c;
const TEXT_FILTER: u32 = 0x0109_ff20;
const TEXT_CLAMP: u32 = 0x0109_ff14;
const TEXT_COORD_MODE: u32 = 0x0109_ff04;
const TEXT_PLANE_ENABLE: u32 = 0x0109_fef4;
const TEXT_MODEL_PLANE: u32 = 0x0109_fee4;

/// Adds the string word at `slot` at the end of the strings array (`SetSize` to the size
/// plus the grow-by step when the array is full, then `SetAt`).
fn add_viewer_string(e: &mut Engine, strings: Ptr<NiTArray>, slot: u32) {
    let index = u32::from(e.get(strings, NiTArray::m_usSize));
    if index >= u32::from(e.get(strings, NiTArray::m_usMaxSize)) {
        let grow = u32::from(e.get(strings, NiTArray::m_usGrowBy));
        e.call(STRING_ARRAY_SET_SIZE, &args![strings, grow + index]);
    }
    e.call(STRING_ARRAY_SET_AT, &args![strings, index, slot]);
}

/// Copies `count` words from `from` to `to` (the code `REP MOVSD`).
fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from + i * 4);
        e.mem.set_u32(to + i * 4, word);
    }
}

/// The length of the zero-terminated string at `text` (the code inline `strlen`).
fn string_length(e: &Engine, text: u32) -> u32 {
    let mut length = 0u32;
    while e.mem.u8(text.wrapping_add(length)) != 0 {
        length = length.wrapping_add(1);
    }
    length
}

/// Releases a counted object (count at +4): when `InterlockedDecrement` reaches 0 its
/// vtable slot +4 (`DeleteThis`) runs.
fn release(e: &mut Engine, object: u32) {
    if interlocked_decrement(e, object + 4) == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// One transfer of `size` bytes at `buffer` through the stream `m_pkIstr` (`read`) or
/// `m_pkOstr`, its function `m_pfnRead`/`m_pfnWrite`, with the component size `size` written
/// to the word at `sizes` first; the stream position moves on by the result.
fn transfer(e: &mut Engine, stream: Ptr<NiStream>, read: bool, buffer: u32, size: u32, sizes: u32) {
    let binary = if read {
        Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkIstr))
    } else {
        Ptr::<NiBinaryStream>::new(e.get(stream, NiStream::m_pkOstr))
    };
    let function = if read {
        e.get(binary, NiBinaryStream::m_pfnRead)
    } else {
        e.get(binary, NiBinaryStream::m_pfnWrite)
    };
    e.mem.set_u32(sizes, size);
    let moved = e
        .call(function, &args![binary, buffer, size, sizes, 1u32])
        .u32();
    let position = e.get(binary, NiBinaryStream::m_uiAbsoluteCurrentPos);
    e.set(
        binary,
        NiBinaryStream::m_uiAbsoluteCurrentPos,
        position.wrapping_add(moved),
    );
}

// Translated from 00a92710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::NiTextureEffect` (Xbox PDB; thiscall, no arguments): the
/// `NiDynamicEffect` constructor, the class vtable, both projection matrices copied from
/// the nine floats at `011a9448` and both translations 0, no texture, both planes built
/// with the normal at `011a946c` and constant 0 (`NiPlane::NiPlane_ov2`), filter 0, clamp
/// 3, texture and coordinate modes 0, the plane disabled and the effect type byte 5.
/// Returns `this`.
pub fn ni_texture_effect_ni_texture_effect(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(DYNAMIC_EFFECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(base, NI_TEXTURE_EFFECT_VTABLE);
    copy_words(e, base + MODEL_PROJ_MAT, CONSTRUCT_MATRIX, 9);
    for i in 0..3 {
        e.mem.set_u32(base + MODEL_PROJ_TRANS + i * 4, 0);
    }
    copy_words(e, base + WORLD_PROJ_MAT, CONSTRUCT_MATRIX, 9);
    for i in 0..3 {
        e.mem.set_u32(base + WORLD_PROJ_TRANS + i * 4, 0);
    }
    e.mem.set_u32(base + TEXTURE, 0);
    e.call(
        PLANE_CONSTRUCT,
        &args![base + MODEL_PLANE, CONSTRUCT_PLANE_NORMAL, 0u32],
    );
    e.call(
        PLANE_CONSTRUCT,
        &args![base + WORLD_PLANE, CONSTRUCT_PLANE_NORMAL, 0u32],
    );
    e.mem.set_u32(base + FILTER, 0);
    e.mem.set_u32(base + TEXTURE_MODE, 0);
    e.mem.set_u32(base + COORD_MODE, 0);
    e.mem.set_u8(base + PLANE_ENABLE, 0);
    e.mem.set_u8(base + EFFECT_TYPE, 5);
    e.mem.set_u32(base + CLAMP, 3);
    this
}

// Translated from 00a92960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (cdecl: the address of a string pointer, an out index): finds
/// the string among the four static fixed strings `NiTextureEffect::_SDMInit` sets
/// (`011f5e9c` onwards): the same pointer, or both non-null and equal ignoring case
/// (`NiStricmp`). Stores the index and answers true when found; false otherwise.
pub fn fn_00a92960(e: &mut Engine, text: u32, index: u32) -> bool {
    for (i, (slot, _)) in STATIC_STRINGS.iter().enumerate() {
        let wanted = e.mem.u32(text);
        let known = e.mem.u32(*slot);
        let found = known == wanted
            || (wanted != 0 && known != 0 && e.call(NI_STRICMP, &args![known, wanted]).u32() == 0);
        if found {
            e.mem.set_u32(index, i as u32);
            return true;
        }
    }
    false
}

// Translated from 00a929b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The class stream loader (Gamebryo `CreateObject`; Ghidra made no function of it):
/// registered with `NiStream::RegisterLoader` under `"NiTextureEffect"` at `00a8a0e2`. A
/// new 0x15C-byte object (`00aa13e0`) handed to the constructor (a tail jump), whose
/// result it returns; null when the allocation fails.
pub fn ni_texture_effect_create_object(e: &mut Engine) -> Ptr {
    let memory = e.call(NI_NEW, &args![NI_TEXTURE_EFFECT_SIZE]).u32();
    if memory == 0 {
        return Ptr::NULL;
    }
    ni_texture_effect_ni_texture_effect(e, Ptr::new(memory))
}

// Translated from 00a929d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::LinkObject` (Xbox PDB; thiscall, `RET 4`): `NiDynamicEffect::LinkObject`,
/// then the texture pointer (+0x124) set to the stream next linked object
/// (`NiStream::GetObjectFromLinkID`; the old texture released, the new one counted).
pub fn ni_texture_effect_link_object(e: &mut Engine, this: Ptr, stream: Ptr) {
    e.call(DYNAMIC_EFFECT_LINK_OBJECT, &args![this, stream]);
    let texture = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    assign_ref_counted(e, this.addr() + TEXTURE, texture);
}

// Translated from 00a92a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::RegisterStreamables` (Xbox PDB; thiscall, `RET 4`): false when the base
/// `RegisterStreamables` (`00a6ff10`) is; otherwise the texture (when there is one)
/// registers too (its slot +0x54, result ignored) and the answer is true.
pub fn ni_texture_effect_register_streamables(e: &mut Engine, this: Ptr, stream: Ptr) -> bool {
    if e.call(BASE_REGISTER_STREAMABLES_00A6FF10, &args![this, stream])
        .u32() as u8
        == 0
    {
        return false;
    }
    let texture = e.mem.u32(this.addr() + TEXTURE);
    if texture != 0 {
        e.vcall(texture, SLOT_REGISTER_STREAMABLES, &args![stream]);
    }
    true
}

// Translated from 00a92a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::GetViewerString` (Xbox PDB; cdecl: a name and the texture mode): a new
/// `NiAlloc` buffer of the name length plus 0x19, filled by `NiSprintf` with the mode
/// name for modes 0 to 3 (the code jump table at `00a92af0`); any other mode leaves the
/// buffer as allocated. Returns the buffer.
pub fn ni_texture_effect_get_viewer_string(e: &mut Engine, name: u32, mode: u32) -> u32 {
    let size = string_length(e, name).wrapping_add(0x19);
    let buffer = e.call(NI_ALLOC, &args![size]).u32();
    let format = match mode {
        0 => FORMAT_PROJECTED_LIGHT,
        1 => FORMAT_PROJECTED_SHADOW,
        2 => FORMAT_ENVIRONMENT_MAP,
        3 => FORMAT_FOG_MAP,
        _ => return buffer,
    };
    e.call(NI_SPRINTF, &args![buffer, size, format, name]);
    buffer
}

// Translated from 00a92b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::GetViewerString_ov2` (Xbox PDB; cdecl: a name and the coordinate
/// mode): a new `NiAlloc` buffer of the name length plus 0x19, filled by `NiSprintf` with
/// the mode name for modes 0 to 4 (the code jump table at `00a92bbc`) or
/// `"%s = UNKNOWN!!!"`. Returns the buffer.
pub fn ni_texture_effect_get_viewer_string_ov2(e: &mut Engine, name: u32, mode: u32) -> u32 {
    let size = string_length(e, name).wrapping_add(0x19);
    let buffer = e.call(NI_ALLOC, &args![size]).u32();
    let format = match mode {
        0 => FORMAT_WORLD_PARALLEL,
        1 => FORMAT_WORLD_PERSPECTIVE,
        2 => FORMAT_SPHERE_MAP,
        3 => FORMAT_SPECULAR_CUBE_MAP,
        4 => FORMAT_DIFFUSE_CUBE_MAP,
        _ => FORMAT_UNKNOWN,
    };
    e.call(NI_SPRINTF, &args![buffer, size, format, name]);
    buffer
}

// Translated from 00a92bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::_scalar_deleting_destructor_` (Xbox PDB; thiscall, `RET 4`): the class
/// vtable; the global object at `011f4748`, when set, gets its slot +0xF8 with `this`; the
/// texture is released (the field is left as it is); `NiDynamicEffect` destructor body
/// (`00a69030`); the 0x15C-byte object freed when bit 0 of `flags` is set. Returns `this`.
pub fn ni_texture_effect_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), NI_TEXTURE_EFFECT_VTABLE);
    let global = e.global::<u32>(GLOBAL_011F4748);
    if global != 0 {
        e.vcall(global, GLOBAL_EFFECT_SLOT, &args![this]);
    }
    let texture = e.mem.u32(this.addr() + TEXTURE);
    if texture != 0 {
        release(e, texture);
    }
    e.call(DYNAMIC_EFFECT_DESTRUCT_00A69030, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, NI_TEXTURE_EFFECT_SIZE]);
    }
    this
}

// Translated from 00a92c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::UpdateProjection` (Xbox PDB; thiscall, no arguments). The world
/// projection for coordinate mode 2 (sphere map): the matrix rows `0.5 * (r02, r12, r22)`,
/// `-0.5 * (r01, r11, r21)` and zeros from the world rotation `r` (its row-major words),
/// the translation `(0.5, 0.5, 0)`. For modes 3 and 4 (cube maps): the model projection
/// times the transposed world rotation, translation 0. Otherwise: the model projection
/// times the transposed world rotation times `1 / scale`, translation the model
/// translation less that matrix times the world translation. Then the world plane: its
/// normal the world rotation times the model plane normal, its constant the world
/// translation dot that normal plus the model constant times the scale; and the revision
/// counter (+0xA8) goes up. x87 arithmetic in the code order.
pub fn ni_texture_effect_update_projection(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let field = |e: &Engine, offset: u32| e.mem.u32(base + offset);
    let coord_mode = field(e, COORD_MODE);
    if coord_mode == 2 {
        let half = x87_store32(x87_load32(e.mem.u32(HALF_FLOAT)));
        e.mem.set_u32(base + WORLD_PROJ_TRANS + 4, half);
        e.mem.set_u32(base + WORLD_PROJ_TRANS, half);
        let zero = x87_store32(0.0);
        e.mem.set_u32(base + WORLD_PROJ_TRANS + 8, zero);
        let plus = x87_load64(e.mem.u64(HALF_DOUBLE));
        let minus = x87_load64(e.mem.u64(MINUS_HALF_DOUBLE));
        let r = |e: &Engine, i: u32| x87_load32(e.mem.u32(base + WORLD_ROTATE + i * 4));
        // The stored first-column word, reloaded from a local.
        let a = x87_load32(x87_store32(r(e, 1)));
        let product = x87_mul(r(e, 2), plus);
        e.mem.set_u32(base + WORLD_PROJ_MAT, x87_store32(product));
        let product = x87_mul(a, minus);
        e.mem
            .set_u32(base + WORLD_PROJ_MAT + 0x0c, x87_store32(product));
        e.mem
            .set_u32(base + WORLD_PROJ_MAT + 0x18, x87_store32(0.0));
        let c = x87_load32(x87_store32(r(e, 4)));
        let product = x87_mul(r(e, 5), plus);
        e.mem
            .set_u32(base + WORLD_PROJ_MAT + 4, x87_store32(product));
        let product = x87_mul(c, minus);
        e.mem
            .set_u32(base + WORLD_PROJ_MAT + 0x10, x87_store32(product));
        e.mem
            .set_u32(base + WORLD_PROJ_MAT + 0x1c, x87_store32(0.0));
        let f = x87_load32(x87_store32(r(e, 7)));
        let product = x87_mul(plus, r(e, 8));
        e.mem
            .set_u32(base + WORLD_PROJ_MAT + 8, x87_store32(product));
        let product = x87_mul(minus, f);
        e.mem
            .set_u32(base + WORLD_PROJ_MAT + 0x14, x87_store32(product));
        e.mem
            .set_u32(base + WORLD_PROJ_MAT + 0x20, x87_store32(0.0));
    } else {
        // The frame: 0x88 bytes of locals (the code `SUB ESP,0x88`).
        e.with_stack(0x88, |e, frame| {
            let local = |offset: u32| frame.addr() + offset - 4;
            if coord_mode == 3 || coord_mode == 4 {
                let zero = x87_store32(0.0);
                for i in [2u32, 1, 0] {
                    e.mem.set_u32(base + WORLD_PROJ_TRANS + i * 4, zero);
                }
                let transposed = e
                    .call(MATRIX_TRANSPOSE, &args![base + WORLD_ROTATE, local(0x44)])
                    .u32();
                let product = e
                    .call(
                        MATRIX_TIMES_MATRIX,
                        &args![base + MODEL_PROJ_MAT, local(0x20), transposed],
                    )
                    .u32();
                copy_words(e, base + WORLD_PROJ_MAT, product, 9);
            } else {
                let scale = x87_load32(field(e, WORLD_SCALE));
                let inverse = x87_store32(x87_div(1.0, scale));
                let pushed = x87_store32(x87_load32(inverse));
                let transposed = e
                    .call(MATRIX_TRANSPOSE, &args![base + WORLD_ROTATE, local(0x20)])
                    .u32();
                let product = e
                    .call(
                        MATRIX_TIMES_MATRIX,
                        &args![base + MODEL_PROJ_MAT, local(0x38), transposed],
                    )
                    .u32();
                let scaled = e
                    .call(MATRIX_TIMES_FLOAT, &args![product, local(0x68), pushed])
                    .u32();
                copy_words(e, base + WORLD_PROJ_MAT, scaled, 9);
                let moved = e
                    .call(
                        MATRIX_TIMES_POINT_004B4500,
                        &args![base + WORLD_PROJ_MAT, local(0x14), base + WORLD_TRANSLATE],
                    )
                    .u32();
                for i in 0..3 {
                    let difference = x87_sub(
                        x87_load32(field(e, MODEL_PROJ_TRANS + i * 4)),
                        x87_m32(e.mem.u32(moved + i * 4)),
                    );
                    e.mem
                        .set_u32(base + WORLD_PROJ_TRANS + i * 4, x87_store32(difference));
                }
            }
        });
    }
    let r = |e: &Engine, i: u32| x87_load32(e.mem.u32(base + WORLD_ROTATE + i * 4));
    let p = |e: &Engine, i: u32| x87_m32(e.mem.u32(base + MODEL_PLANE + i * 4));
    let sum = x87_add(x87_mul(r(e, 1), p(e, 1)), x87_mul(r(e, 0), p(e, 0)));
    let nx = x87_store32(x87_add(sum, x87_mul(r(e, 2), p(e, 2))));
    let sum = x87_add(x87_mul(r(e, 3), p(e, 0)), x87_mul(r(e, 4), p(e, 1)));
    let ny = x87_store32(x87_add(sum, x87_mul(r(e, 5), p(e, 2))));
    let sum = x87_add(x87_mul(r(e, 6), p(e, 0)), x87_mul(r(e, 7), p(e, 1)));
    let nz = x87_store32(x87_add(sum, x87_mul(r(e, 8), p(e, 2))));
    let t = |e: &Engine, i: u32| x87_load32(e.mem.u32(base + WORLD_TRANSLATE + i * 4));
    let sum = x87_add(x87_mul(t(e, 1), x87_m32(ny)), x87_mul(t(e, 0), x87_m32(nx)));
    let revision = e.mem.u32(base + REVISION).wrapping_add(1);
    e.mem.set_u32(base + REVISION, revision);
    let last = x87_mul(t(e, 2), x87_m32(nz));
    e.mem.set_u32(base + WORLD_PLANE, nx);
    e.mem.set_u32(base + WORLD_PLANE + 4, ny);
    e.mem.set_u32(base + WORLD_PLANE + 8, nz);
    let dot = x87_load32(x87_store32(x87_add(sum, last)));
    let offset = x87_mul(
        x87_load32(field(e, MODEL_PLANE + 0xc)),
        x87_m32(field(e, WORLD_SCALE)),
    );
    let constant = x87_load32(x87_store32(x87_add(dot, offset)));
    e.mem
        .set_u32(base + WORLD_PLANE + 0xc, x87_store32(constant));
}

// Translated from 00a92ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::UpdateWorldData` (Xbox PDB; thiscall, `RET 4`):
/// `NiAVObject::UpdateWorldData` with its argument, then
/// [`ni_texture_effect_update_projection`].
pub fn ni_texture_effect_update_world_data(e: &mut Engine, this: Ptr, data: u32) {
    e.call(AV_OBJECT_UPDATE_WORLD_DATA, &args![this, data]);
    ni_texture_effect_update_projection(e, this);
}

// Translated from 00a92f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map; the body is `NiTextureEffect`'s member copy (thiscall,
/// `RET 8`: the destination and the cloning process): `NiDynamicEffect`'s copy
/// (`00a68d40`); the texture pointer (the destination old one released when different, the
/// new one counted); the model projection matrix and translation, filter, clamp, texture
/// and coordinate modes, plane flag, model plane and world plane (the world projection is
/// not copied).
pub fn fn_00a92f10(e: &mut Engine, this: Ptr, dest: Ptr, cloning: u32) {
    e.call(
        DYNAMIC_EFFECT_COPY_MEMBERS_00A68D40,
        &args![this, dest, cloning],
    );
    let (source, target) = (this.addr(), dest.addr());
    let old = e.mem.u32(target + TEXTURE);
    if old != e.mem.u32(source + TEXTURE) {
        if old != 0 {
            release(e, old);
        }
        let texture = e.mem.u32(source + TEXTURE);
        e.mem.set_u32(target + TEXTURE, texture);
        if texture != 0 {
            interlocked_increment(e, texture + 4);
        }
    }
    copy_words(e, target + MODEL_PROJ_MAT, source + MODEL_PROJ_MAT, 9);
    copy_words(e, target + MODEL_PROJ_TRANS, source + MODEL_PROJ_TRANS, 3);
    copy_words(e, target + FILTER, source + FILTER, 4);
    let enable = e.mem.u8(source + PLANE_ENABLE);
    e.mem.set_u8(target + PLANE_ENABLE, enable);
    copy_words(e, target + MODEL_PLANE, source + MODEL_PLANE, 4);
    copy_words(e, target + WORLD_PLANE, source + WORLD_PLANE, 4);
}

// Translated from 00a93040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::IsEqual` (Xbox PDB; thiscall, `RET 4`): `NiDynamicEffect::IsEqual`, the
/// model projection matrices equal and translations not different (each test made on the
/// other object, with this one as the argument), the same filter, clamp, texture and
/// coordinate modes and plane flag, and the model planes not different.
pub fn ni_texture_effect_is_equal(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let (mine, theirs) = (this.addr(), other.addr());
    if e.call(DYNAMIC_EFFECT_IS_EQUAL, &args![this, other]).u32() as u8 == 0 {
        return false;
    }
    if e.call(
        MATRIX_EQUAL,
        &args![theirs + MODEL_PROJ_MAT, mine + MODEL_PROJ_MAT],
    )
    .u32() as u8
        == 0
    {
        return false;
    }
    if e.call(
        POINT_NOT_EQUAL_00439090,
        &args![theirs + MODEL_PROJ_TRANS, mine + MODEL_PROJ_TRANS],
    )
    .u32() as u8
        != 0
    {
        return false;
    }
    for offset in [FILTER, CLAMP, TEXTURE_MODE, COORD_MODE] {
        if e.mem.u32(theirs + offset) != e.mem.u32(mine + offset) {
            return false;
        }
    }
    if e.mem.u8(theirs + PLANE_ENABLE) != e.mem.u8(mine + PLANE_ENABLE) {
        return false;
    }
    e.call(
        PLANE_NOT_EQUAL_00A926B0,
        &args![theirs + MODEL_PLANE, mine + MODEL_PLANE],
    )
    .u32() as u8
        == 0
}

// Translated from 00a930f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::CreateClone` (Xbox PDB; thiscall, `RET 4`: the cloning process): a new
/// 0x15C-byte object (`00aa13e0`) built by the constructor, then the member copy
/// ([`fn_00a92f10`]) into it. When the allocation fails the copy still runs, with a null
/// destination, and the result is null.
pub fn ni_texture_effect_create_clone(e: &mut Engine, this: Ptr, cloning: u32) -> Ptr {
    let memory = e.call(NI_NEW, &args![NI_TEXTURE_EFFECT_SIZE]).u32();
    if memory == 0 {
        fn_00a92f10(e, this, Ptr::NULL, cloning);
        return Ptr::NULL;
    }
    let clone = ni_texture_effect_ni_texture_effect(e, Ptr::new(memory));
    fn_00a92f10(e, this, clone, cloning);
    clone
}

// Translated from 00a93140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::LoadBinary` (Xbox PDB; thiscall, `RET 4`): `NiDynamicEffect::LoadBinary`,
/// the model projection matrix and translation, the filter, clamp, texture and coordinate
/// modes (4 bytes each, read into two locals the code alternates between), the stream
/// call `00a645b0` (its result unused), the plane flag (one byte read over the low byte of
/// the stream argument own stack word, then set to whether it is nonzero) and the model
/// plane. Files before version `0a030004` then have two 2-byte words, and files before
/// `04010010` one more, read into a local and dropped.
pub fn ni_texture_effect_load_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    let base = this.addr();
    e.call(DYNAMIC_EFFECT_LOAD_BINARY, &args![this, stream]);
    e.call(MATRIX_LOAD_BINARY, &args![base + MODEL_PROJ_MAT, stream]);
    e.call(POINT_LOAD_BINARY, &args![base + MODEL_PROJ_TRANS, stream]);
    // Locals A and B (`SUB ESP,8`), then the stream argument stack word.
    e.with_stack(12, |e, frame| {
        let a = frame.addr();
        let b = frame.addr() + 4;
        let argument = frame.addr() + 8;
        e.mem.set_u32(argument, stream.addr());
        transfer(e, stream, true, b, 4, a);
        let filter = e.mem.u32(b);
        e.mem.set_u32(base + FILTER, filter);
        for offset in [CLAMP, TEXTURE_MODE, COORD_MODE] {
            transfer(e, stream, true, a, 4, b);
            let value = e.mem.u32(a);
            e.mem.set_u32(base + offset, value);
        }
        e.call(STREAM_00A645B0, &args![stream]);
        transfer(e, stream, true, argument, 1, b);
        let enable = u8::from(e.mem.u8(argument) != 0);
        e.mem.set_u8(base + PLANE_ENABLE, enable);
        e.call(PLANE_LOAD_BINARY, &args![base + MODEL_PLANE, stream]);
        let version = e.mem.u32(stream.addr() + STREAM_FILE_VERSION);
        if version < VERSION_TWO_OLD_WORDS {
            transfer(e, stream, true, a, 2, b);
            transfer(e, stream, true, a, 2, b);
        }
        let version = e.mem.u32(stream.addr() + STREAM_FILE_VERSION);
        if version < VERSION_ONE_OLD_WORD {
            transfer(e, stream, true, a, 2, b);
        }
    });
}

// Translated from 00a932f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::SaveBinary` (Xbox PDB; thiscall, `RET 4`): `NiDynamicEffect::SaveBinary`,
/// the model projection matrix and translation, the filter, clamp, texture and coordinate
/// modes (4 bytes each), the texture link (stream slot +0x30), the plane flag (its byte
/// written over the low byte of the stream argument own stack word, then written from
/// there, one byte) and the model plane.
pub fn ni_texture_effect_save_binary(e: &mut Engine, this: Ptr, stream: Ptr<NiStream>) {
    let base = this.addr();
    e.call(DYNAMIC_EFFECT_SAVE_BINARY, &args![this, stream]);
    e.call(MATRIX_SAVE_BINARY, &args![base + MODEL_PROJ_MAT, stream]);
    e.call(POINT_SAVE_BINARY, &args![base + MODEL_PROJ_TRANS, stream]);
    // The local (`PUSH ECX`), then the stream argument stack word.
    e.with_stack(8, |e, frame| {
        let local = frame.addr();
        let argument = frame.addr() + 4;
        e.mem.set_u32(argument, stream.addr());
        for offset in [FILTER, CLAMP, TEXTURE_MODE, COORD_MODE] {
            transfer(e, stream, false, base + offset, 4, local);
        }
        let texture = e.mem.u32(base + TEXTURE);
        e.vcall(stream.addr(), STREAM_SAVE_LINK_ID_SLOT, &args![texture]);
        let enable = e.mem.u8(base + PLANE_ENABLE);
        e.mem.set_u8(argument, enable);
        transfer(e, stream, false, argument, 1, local);
    });
    e.call(PLANE_SAVE_BINARY, &args![base + MODEL_PLANE, stream]);
}

// Translated from 00a93400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTextureEffect::GetViewerStrings` (Xbox PDB; thiscall, `RET 4`):
/// `NiDynamicEffect::GetViewerStrings`, then the class name, the model projection matrix and
/// translation (their `GetViewerString`s), the texture mode, the filter and clamp (through
/// `NiTexturingProperty`'s strings), the texture mode again, the coordinate mode, the plane
/// flag and the model plane, each held in the argument stack word and added at the array
/// end.
pub fn ni_texture_effect_get_viewer_strings(e: &mut Engine, this: Ptr, strings: Ptr<NiTArray>) {
    let base = this.addr();
    e.call(DYNAMIC_EFFECT_GET_VIEWER_STRINGS, &args![this, strings]);
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let add = |e: &mut Engine, text: u32| {
            e.mem.set_u32(slot, text);
            add_viewer_string(e, strings, slot);
        };
        let class_name = e.global::<u32>(NI_TEXTURE_EFFECT_RTTI);
        let text = e.call(VIEWER_STRING_NAME, &args![class_name]).u32();
        add(e, text);
        let text = e
            .call(
                MATRIX_GET_VIEWER_STRING,
                &args![base + MODEL_PROJ_MAT, TEXT_MODEL_PROJ_MAT],
            )
            .u32();
        add(e, text);
        let text = e
            .call(
                POINT_GET_VIEWER_STRING,
                &args![base + MODEL_PROJ_TRANS, TEXT_MODEL_PROJ_TRANS],
            )
            .u32();
        add(e, text);
        let mode = e.mem.u32(base + TEXTURE_MODE);
        let text = ni_texture_effect_get_viewer_string(e, TEXT_TEXTURE_MODE, mode);
        add(e, text);
        let filter = e.mem.u32(base + FILTER);
        let text = e
            .call(TEXTURING_FILTER_STRING, &args![TEXT_FILTER, filter])
            .u32();
        add(e, text);
        let clamp = e.mem.u32(base + CLAMP);
        let text = e
            .call(TEXTURING_CLAMP_STRING, &args![TEXT_CLAMP, clamp])
            .u32();
        add(e, text);
        let mode = e.mem.u32(base + TEXTURE_MODE);
        let text = ni_texture_effect_get_viewer_string(e, TEXT_TEXTURE_MODE, mode);
        add(e, text);
        let mode = e.mem.u32(base + COORD_MODE);
        let text = ni_texture_effect_get_viewer_string_ov2(e, TEXT_COORD_MODE, mode);
        add(e, text);
        let enable = u32::from(e.mem.u8(base + PLANE_ENABLE));
        let text = e
            .call(VIEWER_STRING_BOOL, &args![TEXT_PLANE_ENABLE, enable])
            .u32();
        add(e, text);
        let text = e
            .call(
                PLANE_GET_VIEWER_STRING,
                &args![base + MODEL_PLANE, TEXT_MODEL_PLANE],
            )
            .u32();
        add(e, text);
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a927d0, ni_texture_effect_get_rtti() -> Ptr),
        entry!(0x00a9_27e0, ni_texture_effect_sdm_init()),
        entry!(0x00a9_28b0, fn_00a928b0()),
        entry!(0x00a9_2710, ni_texture_effect_ni_texture_effect(Ptr) -> Ptr),
        entry!(0x00a9_2960, fn_00a92960(u32, u32) -> bool),
        entry!(0x00a9_29b0, ni_texture_effect_create_object() -> Ptr),
        entry!(0x00a9_29d0, ni_texture_effect_link_object(Ptr, Ptr)),
        entry!(
            0x00a9_2a30,
            ni_texture_effect_register_streamables(Ptr, Ptr) -> bool
        ),
        entry!(0x00a9_2a60, ni_texture_effect_get_viewer_string(u32, u32) -> u32),
        entry!(
            0x00a9_2b00,
            ni_texture_effect_get_viewer_string_ov2(u32, u32) -> u32
        ),
        entry!(
            0x00a9_2bd0,
            ni_texture_effect_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00a9_2c40, ni_texture_effect_update_projection(Ptr)),
        entry!(0x00a9_2ef0, ni_texture_effect_update_world_data(Ptr, u32)),
        entry!(0x00a9_2f10, fn_00a92f10(Ptr, Ptr, u32)),
        entry!(0x00a9_3040, ni_texture_effect_is_equal(Ptr, Ptr) -> bool),
        entry!(0x00a9_30f0, ni_texture_effect_create_clone(Ptr, u32) -> Ptr),
        entry!(
            0x00a9_3140,
            ni_texture_effect_load_binary(Ptr, Ptr<NiStream>)
        ),
        entry!(
            0x00a9_32f0,
            ni_texture_effect_save_binary(Ptr, Ptr<NiStream>)
        ),
        entry!(
            0x00a9_3400,
            ni_texture_effect_get_viewer_strings(Ptr, Ptr<NiTArray>)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_texture_effect_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a927d0, &args![]).u32(), NI_TEXTURE_EFFECT_RTTI);
    }
}

#[cfg(test)]
mod start_up_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::*;

    #[test]
    fn sets_the_four_strings_unless_already_set() {
        let mut e = engine_with_counting();
        e.register(GET_FIXED_STRING, |_, a| (a[0] + 0x1000).into_ret());
        let empty = e.mem.u32(EMPTY_FIXED_STRING);
        for (slot, _) in STATIC_STRINGS {
            e.mem.set_u32(slot, empty);
        }
        e.mem.set_u32(0x011f_5ea0, 0x0109_fe00);
        e.call_log = Some(vec![]);
        e.call(0x00a9_27e0, &args![]);
        assert_eq!(e.mem.u32(0x011f_5e9c), 0x010a_0e10);
        assert_eq!(e.mem.u32(0x011f_5ea0), 0x0109_fe00);
        assert_eq!(e.mem.u32(0x011f_5ea4), 0x010a_0df0);
        assert_eq!(e.mem.u32(0x011f_5ea8), 0x010a_0de8);
        assert_eq!(
            calls_to(&e, GET_FIXED_STRING),
            vec![vec![0x0109_fe10], vec![0x0109_fdf0], vec![0x0109_fde8]]
        );
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00a928b0_clears_the_set_strings_and_releases_the_counted_ones() {
        let mut e = Engine::new();
        e.map(EMPTY_STRING_HANDLE & !0xfff, 0x1000);
        e.map(0x011f_5000, 0x1000);
        e.mem.set_u32(EMPTY_STRING_HANDLE, 0x4444);
        e.register(FIXED_STRING_FROM_TEXT, |_, _| 0x7770u32.into_ret());
        e.register(INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.mem.set_u32(0x011f_5e9c, 0x1008);
        e.mem.set_u32(0x011f_5ea0, 0);
        e.mem.set_u32(0x011f_5ea4, 0x4444);
        e.mem.set_u32(0x011f_5ea8, 0x2008);
        e.call_log = Some(vec![]);
        e.call(0x00a9_28b0, &args![]);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|call| call.0 != 0x00a9_28b0)
            .collect();
        assert_eq!(
            log,
            vec![
                (FIXED_STRING_FROM_TEXT, vec![0]),
                (INTERLOCKED_DECREMENT, vec![0x1000]),
                (FIXED_STRING_FROM_TEXT, vec![0]),
                (FIXED_STRING_FROM_TEXT, vec![0]),
                (INTERLOCKED_DECREMENT, vec![0x2000]),
            ]
        );
        assert_eq!(e.mem.u32(0x011f_5e9c), 0x7770);
        assert_eq!(e.mem.u32(0x011f_5ea0), 0);
        assert_eq!(e.mem.u32(0x011f_5ea4), 0x7770);
        assert_eq!(e.mem.u32(0x011f_5ea8), 0x7770);
    }
}

#[cfg(test)]
mod n020_tests {
    use super::*;
    use crate::units::nimain::nimainsdm::test_support::{calls_to, engine_with_counting};

    const VTABLE: u32 = 0x0710_0000;
    const DELETE_DOUBLE: u32 = 0x0720_0000;
    const REGISTER_DOUBLE: u32 = 0x0720_0010;
    const SAVE_LINK_DOUBLE: u32 = 0x0720_0020;
    const GLOBAL_DOUBLE: u32 = 0x0720_0030;
    const TRANSFER: u32 = 0x0720_0040;

    /// The counting engine with one vtable whose used slots are doubles, the constants the
    /// projection reads and doubles for every callee outside this file.
    fn engine() -> Engine {
        let mut e = engine_with_counting();
        let mut slots = vec![0u32; 0x100 / 4];
        slots[1] = DELETE_DOUBLE;
        slots[SLOT_REGISTER_STREAMABLES as usize / 4] = REGISTER_DOUBLE;
        slots[STREAM_SAVE_LINK_ID_SLOT as usize / 4] = SAVE_LINK_DOUBLE;
        slots[GLOBAL_EFFECT_SLOT as usize / 4] = GLOBAL_DOUBLE;
        e.put_vtable(VTABLE, &slots);
        e.map(0x0101_1000, 0x6000);
        e.mem.set_u32(HALF_FLOAT, 0.5f32.to_bits());
        e.mem.set_u64(HALF_DOUBLE, 0.5f64.to_bits());
        e.mem.set_u64(MINUS_HALF_DOUBLE, (-0.5f64).to_bits());
        e.map(0x011a_9000, 0x1000);
        for (i, word) in (1..=9u32).enumerate() {
            e.mem.set_u32(CONSTRUCT_MATRIX + 4 * i as u32, word);
        }
        for addr in [
            DELETE_DOUBLE,
            REGISTER_DOUBLE,
            SAVE_LINK_DOUBLE,
            GLOBAL_DOUBLE,
            DYNAMIC_EFFECT_CONSTRUCT,
            DYNAMIC_EFFECT_DESTRUCT_00A69030,
            DYNAMIC_EFFECT_LINK_OBJECT,
            DYNAMIC_EFFECT_COPY_MEMBERS_00A68D40,
            DYNAMIC_EFFECT_LOAD_BINARY,
            DYNAMIC_EFFECT_SAVE_BINARY,
            DYNAMIC_EFFECT_GET_VIEWER_STRINGS,
            AV_OBJECT_UPDATE_WORLD_DATA,
            PLANE_CONSTRUCT,
            MATRIX_LOAD_BINARY,
            MATRIX_SAVE_BINARY,
            POINT_LOAD_BINARY,
            POINT_SAVE_BINARY,
            PLANE_LOAD_BINARY,
            PLANE_SAVE_BINARY,
            STREAM_00A645B0,
            SIZED_DELETE,
            NI_SPRINTF,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(NI_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_ALLOC, |_, a| (0x0c00_0000 + a[0]).into_ret());
        e
    }

    fn effect(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(NI_TEXTURE_EFFECT_SIZE);
        e.mem.set_u32(this, VTABLE);
        this
    }

    fn texture(e: &mut Engine, count: u32) -> u32 {
        let texture = e.mem.alloc(0x30);
        e.mem.set_u32(texture, VTABLE);
        e.mem.set_u32(texture + 4, count);
        texture
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut e = engine();
        let this = e.mem.alloc(NI_TEXTURE_EFFECT_SIZE);
        for offset in (0..NI_TEXTURE_EFFECT_SIZE).step_by(4) {
            e.mem.set_u32(this + offset, 0xffff_ffff);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_2710, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_TEXTURE_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(this + MODEL_PROJ_MAT + 0x20), 9);
        assert_eq!(e.mem.u32(this + WORLD_PROJ_MAT), 1);
        assert_eq!(e.mem.u32(this + WORLD_PROJ_TRANS + 8), 0);
        assert_eq!(e.mem.u32(this + TEXTURE), 0);
        assert_eq!(e.mem.u32(this + CLAMP), 3);
        assert_eq!(e.mem.u32(this + COORD_MODE), 0);
        assert_eq!(e.mem.u8(this + PLANE_ENABLE), 0);
        assert_eq!(e.mem.u8(this + EFFECT_TYPE), 5);
        assert_eq!(
            calls_to(&e, PLANE_CONSTRUCT),
            vec![
                vec![this + MODEL_PLANE, CONSTRUCT_PLANE_NORMAL, 0],
                vec![this + WORLD_PLANE, CONSTRUCT_PLANE_NORMAL, 0]
            ]
        );
    }

    #[test]
    fn static_string_lookup_matches_pointers_or_names() {
        let mut e = engine();
        e.register(NI_STRICMP, |e, a| {
            u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        let names: Vec<u32> = (0..4)
            .map(|i| {
                let s = e.mem.alloc(4);
                e.mem.set_u32(s, 0x41 + i);
                s
            })
            .collect();
        for (i, (slot, _)) in STATIC_STRINGS.iter().enumerate() {
            e.mem.set_u32(*slot, names[i]);
        }
        let holder = e.mem.alloc(8);
        // The same pointer.
        e.mem.set_u32(holder, names[2]);
        assert_eq!(
            e.call(0x00a9_2960, &args![holder, holder + 4]).u32() & 0xff,
            1
        );
        assert_eq!(e.mem.u32(holder + 4), 2);
        // An equal name elsewhere.
        let copy = e.mem.alloc(4);
        e.mem.set_u32(copy, 0x44);
        e.mem.set_u32(holder, copy);
        assert_eq!(
            e.call(0x00a9_2960, &args![holder, holder + 4]).u32() & 0xff,
            1
        );
        assert_eq!(e.mem.u32(holder + 4), 3);
        // Not there.
        e.mem.set_u32(copy, 0x50);
        assert_eq!(
            e.call(0x00a9_2960, &args![holder, holder + 4]).u32() & 0xff,
            0
        );
    }

    #[test]
    fn create_object_constructs_or_gives_null() {
        let mut e = engine();
        let made = e.call(0x00a9_29b0, &args![]).u32();
        assert_eq!(e.mem.u32(made), NI_TEXTURE_EFFECT_VTABLE);
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_29b0, &args![]).u32(), 0);
    }

    #[test]
    fn link_object_swaps_in_the_linked_texture() {
        let mut e = engine();
        let old = texture(&mut e, 1);
        let new = texture(&mut e, 0);
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| new.into_ret());
        let this = effect(&mut e);
        e.mem.set_u32(this + TEXTURE, old);
        e.call_log = Some(vec![]);
        e.call(0x00a9_29d0, &args![this, 0x50u32]);
        assert_eq!(e.mem.u32(this + TEXTURE), new);
        assert_eq!(e.mem.u32(new + 4), 1);
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![old]]);
        assert_eq!(
            calls_to(&e, DYNAMIC_EFFECT_LINK_OBJECT),
            vec![vec![this, 0x50]]
        );
    }

    #[test]
    fn register_streamables_passes_on_to_the_texture() {
        let mut e = engine();
        e.register(BASE_REGISTER_STREAMABLES_00A6FF10, |_, _| 1u32.into_ret());
        let this = effect(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_2a30, &args![this, 0x50u32]).u32() & 0xff, 1);
        let t = texture(&mut e, 1);
        e.mem.set_u32(this + TEXTURE, t);
        assert_eq!(e.call(0x00a9_2a30, &args![this, 0x50u32]).u32() & 0xff, 1);
        assert_eq!(calls_to(&e, REGISTER_DOUBLE), vec![vec![t, 0x50]]);
        e.register(BASE_REGISTER_STREAMABLES_00A6FF10, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_2a30, &args![this, 0x50u32]).u32() & 0xff, 0);
        assert_eq!(calls_to(&e, REGISTER_DOUBLE).len(), 1);
    }

    #[test]
    fn mode_strings_pick_their_formats() {
        let mut e = engine();
        let name = e.mem.alloc(4);
        e.mem.write(name, b"ab");
        e.call_log = Some(vec![]);
        for mode in 0..5u32 {
            e.call(0x00a9_2a60, &args![name, mode]);
        }
        for mode in 0..6u32 {
            e.call(0x00a9_2b00, &args![name, mode]);
        }
        let formats: Vec<u32> = calls_to(&e, NI_SPRINTF).iter().map(|w| w[2]).collect();
        assert_eq!(
            formats,
            vec![
                FORMAT_PROJECTED_LIGHT,
                FORMAT_PROJECTED_SHADOW,
                FORMAT_ENVIRONMENT_MAP,
                FORMAT_FOG_MAP,
                FORMAT_WORLD_PARALLEL,
                FORMAT_WORLD_PERSPECTIVE,
                FORMAT_SPHERE_MAP,
                FORMAT_SPECULAR_CUBE_MAP,
                FORMAT_DIFFUSE_CUBE_MAP,
                FORMAT_UNKNOWN
            ]
        );
        assert!(calls_to(&e, NI_ALLOC).iter().all(|w| w[0] == 2 + 0x19));
        assert_eq!(e.call(0x00a9_2a60, &args![name, 7u32]).u32(), 0x0c00_001b);
    }

    #[test]
    fn deleting_destructor_tells_the_global_and_releases_the_texture() {
        let mut e = engine();
        let this = effect(&mut e);
        let t = texture(&mut e, 1);
        e.mem.set_u32(this + TEXTURE, t);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_2bd0, &args![this, 1u32]).u32(), this);
        assert!(calls_to(&e, GLOBAL_DOUBLE).is_empty());
        assert_eq!(calls_to(&e, DELETE_DOUBLE), vec![vec![t]]);
        assert_eq!(e.mem.u32(this + TEXTURE), t);
        assert_eq!(
            calls_to(&e, SIZED_DELETE),
            vec![vec![this, NI_TEXTURE_EFFECT_SIZE]]
        );
        let global = e.mem.alloc(4);
        e.mem.set_u32(global, VTABLE);
        e.mem.set_u32(GLOBAL_011F4748, global);
        e.mem.set_u32(this + TEXTURE, 0);
        e.call(0x00a9_2bd0, &args![this, 0u32]);
        assert_eq!(calls_to(&e, GLOBAL_DOUBLE), vec![vec![global, this]]);
        assert_eq!(calls_to(&e, SIZED_DELETE).len(), 1);
    }

    fn set_floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// An effect with a world rotation 1..9, translation (1, 2, 3), scale 2, model plane
    /// normal (1, 0, 0) and constant 3.
    fn placed_effect(e: &mut Engine, coord_mode: u32) -> u32 {
        let this = effect(e);
        let rotation: Vec<f32> = (1..=9).map(|v| v as f32).collect();
        set_floats(e, this + WORLD_ROTATE, &rotation);
        set_floats(e, this + WORLD_TRANSLATE, &[1.0, 2.0, 3.0]);
        e.mem.set_f32(this + WORLD_SCALE, 2.0);
        set_floats(e, this + MODEL_PLANE, &[1.0, 0.0, 0.0, 3.0]);
        e.mem.set_u32(this + COORD_MODE, coord_mode);
        this
    }

    #[test]
    fn sphere_map_projection_and_the_world_plane() {
        let mut e = engine();
        let this = placed_effect(&mut e, 2);
        e.call(0x00a9_2c40, &args![this]);
        assert_eq!(
            floats(&e, this + WORLD_PROJ_MAT, 9),
            vec![1.5, 3.0, 4.5, -1.0, -2.5, -4.0, 0.0, 0.0, 0.0]
        );
        assert_eq!(floats(&e, this + WORLD_PROJ_TRANS, 3), vec![0.5, 0.5, 0.0]);
        // Normal: the rotation's first column; constant: (1, 2, 3) . (1, 4, 7) + 3 * 2.
        assert_eq!(
            floats(&e, this + WORLD_PLANE, 4),
            vec![1.0, 4.0, 7.0, 30.0 + 6.0]
        );
        assert_eq!(e.mem.u32(this + REVISION), 1);
    }

    #[test]
    fn cube_map_projection_multiplies_by_the_transposed_rotation() {
        let mut e = engine();
        e.register(MATRIX_TRANSPOSE, |_, a| a[1].into_ret());
        e.register(MATRIX_TIMES_MATRIX, |e, a| {
            for i in 0..9 {
                e.mem.set_f32(a[1] + 4 * i, 10.0 + i as f32);
            }
            a[1].into_ret()
        });
        let this = placed_effect(&mut e, 4);
        e.mem.set_f32(this + WORLD_PROJ_TRANS, 7.0);
        e.call_log = Some(vec![]);
        e.call(0x00a9_2c40, &args![this]);
        let transposes = calls_to(&e, MATRIX_TRANSPOSE);
        assert_eq!(transposes[0][0], this + WORLD_ROTATE);
        let products = calls_to(&e, MATRIX_TIMES_MATRIX);
        assert_eq!(products[0][0], this + MODEL_PROJ_MAT);
        assert_eq!(products[0][2], transposes[0][1]);
        assert_eq!(e.mem.f32(this + WORLD_PROJ_MAT + 0x20), 18.0);
        assert_eq!(floats(&e, this + WORLD_PROJ_TRANS, 3), vec![0.0; 3]);
    }

    #[test]
    fn other_projections_scale_and_move_the_translation() {
        let mut e = engine();
        e.register(MATRIX_TRANSPOSE, |_, a| a[1].into_ret());
        e.register(MATRIX_TIMES_MATRIX, |_, a| a[1].into_ret());
        e.register(MATRIX_TIMES_FLOAT, |e, a| {
            assert_eq!(f32::from_bits(a[2]), 0.5);
            for i in 0..9 {
                e.mem.set_f32(a[1] + 4 * i, i as f32);
            }
            a[1].into_ret()
        });
        e.register(MATRIX_TIMES_POINT_004B4500, |e, a| {
            set_floats(e, a[1], &[1.0, 1.5, -2.0]);
            a[1].into_ret()
        });
        let this = placed_effect(&mut e, 0);
        set_floats(&mut e, this + MODEL_PROJ_TRANS, &[4.0, 4.0, 4.0]);
        e.call_log = Some(vec![]);
        e.call(0x00a9_2c40, &args![this]);
        let points = calls_to(&e, MATRIX_TIMES_POINT_004B4500);
        assert_eq!(points[0][0], this + WORLD_PROJ_MAT);
        assert_eq!(points[0][2], this + WORLD_TRANSLATE);
        assert_eq!(e.mem.f32(this + WORLD_PROJ_MAT + 0x20), 8.0);
        assert_eq!(floats(&e, this + WORLD_PROJ_TRANS, 3), vec![3.0, 2.5, 6.0]);
    }

    #[test]
    fn update_world_data_runs_the_base_then_the_projection() {
        let mut e = engine();
        let this = placed_effect(&mut e, 2);
        e.call_log = Some(vec![]);
        e.call(0x00a9_2ef0, &args![this, 0x44u32]);
        assert_eq!(
            calls_to(&e, AV_OBJECT_UPDATE_WORLD_DATA),
            vec![vec![this, 0x44]]
        );
        assert_eq!(e.mem.u32(this + REVISION), 1);
    }

    #[test]
    fn member_copy_takes_the_texture_and_the_settings() {
        let mut e = engine();
        let source = effect(&mut e);
        for offset in (MODEL_PROJ_MAT..NI_TEXTURE_EFFECT_SIZE).step_by(4) {
            e.mem.set_u32(source + offset, offset);
        }
        let t = texture(&mut e, 1);
        e.mem.set_u32(source + TEXTURE, t);
        let dest = effect(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a9_2f10, &args![source, dest, 6u32]);
        assert_eq!(
            calls_to(&e, DYNAMIC_EFFECT_COPY_MEMBERS_00A68D40),
            vec![vec![source, dest, 6]]
        );
        assert_eq!(e.mem.u32(dest + TEXTURE), t);
        assert_eq!(e.mem.u32(t + 4), 2);
        for offset in [
            MODEL_PROJ_MAT,
            MODEL_PROJ_TRANS + 8,
            FILTER,
            COORD_MODE,
            MODEL_PLANE,
        ] {
            assert_eq!(e.mem.u32(dest + offset), offset);
        }
        assert_eq!(e.mem.u32(dest + WORLD_PLANE + 0xc), WORLD_PLANE + 0xc);
        assert_eq!(e.mem.u8(dest + PLANE_ENABLE), PLANE_ENABLE as u8);
        // The world projection is not copied.
        assert_eq!(e.mem.u32(dest + WORLD_PROJ_MAT), 0);
    }

    #[test]
    fn is_equal_runs_each_test_on_the_other_object() {
        let mut e = engine();
        e.register(DYNAMIC_EFFECT_IS_EQUAL, |_, _| 1u32.into_ret());
        e.register(MATRIX_EQUAL, |_, _| 1u32.into_ret());
        e.register(POINT_NOT_EQUAL_00439090, |_, _| 0u32.into_ret());
        e.register(PLANE_NOT_EQUAL_00A926B0, |_, _| 0u32.into_ret());
        let a = effect(&mut e);
        let b = effect(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a9_3040, &args![a, b]).u32() & 0xff, 1);
        assert_eq!(
            calls_to(&e, MATRIX_EQUAL),
            vec![vec![b + MODEL_PROJ_MAT, a + MODEL_PROJ_MAT]]
        );
        assert_eq!(
            calls_to(&e, PLANE_NOT_EQUAL_00A926B0),
            vec![vec![b + MODEL_PLANE, a + MODEL_PLANE]]
        );
        e.mem.set_u8(b + PLANE_ENABLE, 1);
        assert_eq!(e.call(0x00a9_3040, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u8(b + PLANE_ENABLE, 0);
        e.mem.set_u32(b + CLAMP, 1);
        assert_eq!(e.call(0x00a9_3040, &args![a, b]).u32() & 0xff, 0);
        e.mem.set_u32(b + CLAMP, 0);
        e.register(PLANE_NOT_EQUAL_00A926B0, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x00a9_3040, &args![a, b]).u32() & 0xff, 0);
        e.register(POINT_NOT_EQUAL_00439090, |_, _| 1u32.into_ret());
        assert_eq!(e.call(0x00a9_3040, &args![a, b]).u32() & 0xff, 0);
    }

    #[test]
    fn create_clone_constructs_and_copies() {
        let mut e = engine();
        let this = effect(&mut e);
        e.mem.set_u32(this + COORD_MODE, 4);
        let clone = e.call(0x00a9_30f0, &args![this, 2u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_TEXTURE_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(clone + COORD_MODE), 4);
        e.register(NI_NEW, |_, _| 0u32.into_ret());
        // The copy into a null destination writes at address 0 + its offsets.
        e.map(0, NI_TEXTURE_EFFECT_SIZE);
        assert_eq!(e.call(0x00a9_30f0, &args![this, 2u32]).u32(), 0);
        assert_eq!(e.mem.u32(COORD_MODE), 4);
    }

    /// A stream whose transfer double logs (buffer, size) and fills reads with 0x0101_0101
    /// truncated to the size.
    fn stream(e: &mut Engine, version: u32) -> u32 {
        e.register(TRANSFER, |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2]);
            for i in 0..a[2] {
                e.mem.set_u8(a[1] + i, 1);
            }
            a[2].into_ret()
        });
        let stream = e.mem.alloc(0x5c4);
        e.mem.set_u32(stream, VTABLE);
        let binary = e.mem.alloc(0x10);
        e.mem.set_u32(binary + 8, TRANSFER);
        e.mem.set_u32(binary + 0xc, TRANSFER);
        e.mem.set_u32(stream + 0x24c, binary);
        e.mem.set_u32(stream + 0x250, binary);
        e.mem.set_u32(stream + STREAM_FILE_VERSION, version);
        stream
    }

    #[test]
    fn load_binary_reads_the_settings_and_old_words() {
        let mut e = engine();
        let s = stream(&mut e, 0x0400_0000);
        let this = effect(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a9_3140, &args![this, s]);
        for offset in [FILTER, CLAMP, TEXTURE_MODE, COORD_MODE] {
            assert_eq!(e.mem.u32(this + offset), 0x0101_0101);
        }
        assert_eq!(e.mem.u8(this + PLANE_ENABLE), 1);
        let sizes: Vec<u32> = calls_to(&e, TRANSFER).iter().map(|w| w[2]).collect();
        assert_eq!(sizes, vec![4, 4, 4, 4, 1, 2, 2, 2]);
        assert_eq!(
            calls_to(&e, PLANE_LOAD_BINARY),
            vec![vec![this + MODEL_PLANE, s]]
        );
        assert_eq!(calls_to(&e, STREAM_00A645B0), vec![vec![s]]);
        let s = stream(&mut e, 0x0a03_0004);
        e.call_log = Some(vec![]);
        e.call(0x00a9_3140, &args![this, s]);
        assert_eq!(calls_to(&e, TRANSFER).len(), 5);
        let s = stream(&mut e, 0x0a00_0000);
        e.call_log = Some(vec![]);
        e.call(0x00a9_3140, &args![this, s]);
        assert_eq!(calls_to(&e, TRANSFER).len(), 7);
    }

    #[test]
    fn save_binary_writes_the_settings_link_and_flag() {
        let mut e = engine();
        let s = stream(&mut e, 0x1400_0000);
        let this = effect(&mut e);
        e.mem.set_u32(this + TEXTURE, 0x0abc_0000);
        e.call_log = Some(vec![]);
        e.call(0x00a9_32f0, &args![this, s]);
        let writes: Vec<(u32, u32)> = calls_to(&e, TRANSFER)
            .iter()
            .map(|w| (w[1], w[2]))
            .collect();
        assert_eq!(
            writes[..4],
            [
                (this + FILTER, 4),
                (this + CLAMP, 4),
                (this + TEXTURE_MODE, 4),
                (this + COORD_MODE, 4)
            ]
        );
        assert_eq!(writes[4].1, 1);
        assert_eq!(calls_to(&e, SAVE_LINK_DOUBLE), vec![vec![s, 0x0abc_0000]]);
        assert_eq!(
            calls_to(&e, PLANE_SAVE_BINARY),
            vec![vec![this + MODEL_PLANE, s]]
        );
    }

    #[test]
    fn viewer_strings_list_the_settings_in_order() {
        let mut e = engine();
        e.mem.set_u32(NI_TEXTURE_EFFECT_RTTI, 0x0104_0000);
        e.map(0x0109_f000, 0x1000);
        e.register(VIEWER_STRING_NAME, |_, a| (a[0] + 1).into_ret());
        e.register(MATRIX_GET_VIEWER_STRING, |_, a| a[1].into_ret());
        e.register(POINT_GET_VIEWER_STRING, |_, a| a[1].into_ret());
        e.register(PLANE_GET_VIEWER_STRING, |_, a| a[1].into_ret());
        e.register(TEXTURING_FILTER_STRING, |_, a| a[0].into_ret());
        e.register(TEXTURING_CLAMP_STRING, |_, a| a[0].into_ret());
        e.register(VIEWER_STRING_BOOL, |_, a| (a[0] + a[1]).into_ret());
        e.register(STRING_ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(STRING_ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + a[1] * 4, value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(strings + 4, base);
        e.mem.set_u16(strings + 0xe, 4);
        let this = effect(&mut e);
        e.mem.set_u8(this + PLANE_ENABLE, 1);
        e.call(0x00a9_3400, &args![this, strings]);
        let words: Vec<u32> = (0..10).map(|i| e.mem.u32(base + 4 * i)).collect();
        // The mode strings are `NiAlloc` buffers of the name length (0 here) plus 0x19.
        assert_eq!(
            words,
            vec![
                0x0104_0001,
                TEXT_MODEL_PROJ_MAT,
                TEXT_MODEL_PROJ_TRANS,
                0x0c00_0019,
                TEXT_FILTER,
                TEXT_CLAMP,
                0x0c00_0019,
                0x0c00_0019,
                TEXT_PLANE_ENABLE + 1,
                TEXT_MODEL_PLANE
            ]
        );
    }
}
