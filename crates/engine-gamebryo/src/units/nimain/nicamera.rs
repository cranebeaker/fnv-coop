//! `NiMain/nicamera.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `NiCamera` derives from `NiAVObject`. On PC its size is 0x114 (the
//! allocation in `00a713e0`); every field the Xbox PDB places after the
//! `NiAVObject` base sits 0x24 lower than the PDB says, because the PC
//! `NiAVObject` is 0x9C bytes (the Xbox one is 0xC0).
//!
//! The fields of the `NiAVObject` base that the code reads are read at their
//! PC offsets with a comment naming them: the world transform
//! (`m_kWorld`, rotation at +0x68 as nine floats row by row, translation at
//! +0x8C), the local rotation (`m_kLocal.m_Rotate`, +0x34) and the parent
//! (`m_pkParent`, +0x18, in [`NiAVObject`]).
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results. The translations compute in `f64` and round to `f32` wherever the
//! code stores a `float` (or keeps a value in a `float` local), the closest
//! match; results can differ only in the last bit of a result rounded twice.

// The comparisons are written as the x87 code tests them: a negated `<=` is false
// for a NaN, which is what the game does, and a clamp would behave differently.
#![allow(clippy::neg_cmp_op_on_partial_ord, clippy::manual_clamp)]

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{NiAVObject, NiTArray};

/// `NiCamera`'s virtual table.
const NI_CAMERA_VTABLE: u32 = 0x0109_cb9c;
/// `NiCamera::ms_RTTI` (the `NiRTTI` object, whose first word is the class
/// name string).
const NI_CAMERA_RTTI: u32 = 0x011f_4a18;
/// The size of an `NiCamera` allocation.
const NI_CAMERA_SIZE: u32 = 0x114;

/// `NiAVObject::m_kWorld.m_Rotate` (PC +0x68): a 3x3 float matrix, row by
/// row.
const WORLD_ROTATE: u32 = 0x68;
/// `NiAVObject::m_kWorld.m_Translate` (PC +0x8C).
const WORLD_TRANSLATE: u32 = 0x8c;
/// `NiAVObject::m_kLocal.m_Rotate` (PC +0x34).
const LOCAL_ROTATE: u32 = 0x34;
/// `NiAVObject::m_kWorldBound` pointer (PC +0x20, [`NiAVObject`]).
const WORLD_BOUND: u32 = 0x20;
/// `m_aafWorldToCam`: the 4x4 world-to-camera matrix, row by row.
const WORLD_TO_CAM: u32 = 0x9c;

/// `NiAVObject::LoadBinary` (`00a5a510`), `__thiscall(this, stream)`.
const AV_OBJECT_LOAD_BINARY: u32 = 0x00a5_a510;
/// `NiAVObject::SaveBinary` (`00a5a900`), `__thiscall(this, stream)`.
const AV_OBJECT_SAVE_BINARY: u32 = 0x00a5_a900;
/// `NiAVObject::LinkObject` (`00a5a7f0`), `__thiscall(this, stream)`.
const AV_OBJECT_LINK_OBJECT: u32 = 0x00a5_a7f0;
/// `NiAVObject::RegisterStreamables` (`00a59f00`), `__thiscall(this,
/// stream) -> bool`.
const AV_OBJECT_REGISTER_STREAMABLES: u32 = 0x00a5_9f00;
/// `NiAVObject::IsEqual` (`00a5a260`), `__thiscall(this, other) -> bool`.
const AV_OBJECT_IS_EQUAL: u32 = 0x00a5_a260;
/// `NiAVObject::GetViewerStrings` (`00a5ad00`), `__thiscall(this, strings)`.
const AV_OBJECT_GET_VIEWER_STRINGS: u32 = 0x00a5_ad00;
/// `NiAVObject::UpdateWorldData` (`00a68c60`), `__thiscall(this,
/// update_data)`.
const AV_OBJECT_UPDATE_WORLD_DATA: u32 = 0x00a6_8c60;
/// `NiAVObject::NiAVObject` (`00a5aa20`), `__thiscall(this)`.
const AV_OBJECT_CONSTRUCT: u32 = 0x00a5_aa20;
/// The base-class destructor body the scalar deleting destructor calls
/// (`00a5a390`, in the `NiAVObject` source unit), `__thiscall(this)`.
const AV_OBJECT_DESTRUCT: u32 = 0x00a5_a390;
/// The copy of the `NiAVObject` members (`00a5ac90`, which the engine map
/// names `NiGeometry::CopyMembers`), `__thiscall(this, target,
/// copy_process)`.
const COPY_MEMBERS: u32 = 0x00a5_ac90;

/// `operator new` of the `Ni` allocator, `__cdecl(size) -> block`
/// (`00aa13e0`).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// A `__cdecl(size) -> block` allocator (`00aa1070`) used for the viewer
/// string buffer.
const NI_ALLOCATE_BUFFER: u32 = 0x00aa_1070;
/// `operator delete` of the `Ni` allocator, `__cdecl(block, size)`
/// (`00aa1460`).
const NI_FREE: u32 = 0x00aa_1460;
/// The allocator of the world bound (`00aa3e40`), `__thiscall(pool, size) ->
/// block`; the code passes the object at `011f6238` as `this`.
const BOUND_ALLOCATE: u32 = 0x00aa_3e40;
/// The object at `011f6238` that `UpdateWorldBound` passes to
/// [`BOUND_ALLOCATE`].
const BOUND_POOL: u32 = 0x011f_6238;
/// `memcpy`, `__cdecl(destination, source, size)` (`00ec44d0`).
const MEMCPY: u32 = 0x00ec_44d0;
/// `NiSprintf` (`00a305b0`), `__cdecl(buffer, size, format, ...)`.
const NI_SPRINTF: u32 = 0x00a3_05b0;

/// `NiPoint3::Unitize` in place (`004a0c10`; the engine map has no name for
/// it), `__thiscall(vector)`.
const POINT_NORMALIZE: u32 = 0x004a_0c10;
/// `NiPoint3::UnitCross` (`0053d1a0`), `__thiscall(this, result, other) ->
/// result`.
const UNIT_CROSS: u32 = 0x0053_d1a0;
/// `NiMatrix3::TransposeTimes` (`00a588a0`), `__thiscall(this, result,
/// other) -> result`.
const MATRIX_TRANSPOSE_TIMES: u32 = 0x00a5_88a0;

/// `NiStream::GetObjectFromLinkID` (`00a63430`), `__thiscall(stream) ->
/// object`.
const STREAM_GET_OBJECT_FROM_LINK_ID: u32 = 0x00a6_3430;
/// `NiStream::GetNumberOfLinkIDs` (`00a63460`), `__thiscall(stream) ->
/// count`.
const STREAM_GET_NUMBER_OF_LINK_IDS: u32 = 0x00a6_3460;
/// Reads one 4-byte value through the stream's reader and appends it to the
/// stream's list (`00a645b0`, unnamed in the engine map), `__thiscall(stream)`.
const STREAM_READ_LINK_ID: u32 = 0x00a6_45b0;
/// `NiStream::ReadMultipleLinkIDs` (`00a64670`), `__thiscall(stream)`.
const STREAM_READ_MULTIPLE_LINK_IDS: u32 = 0x00a6_4670;
/// The `KERNEL32` import slots `InterlockedIncrement` / `InterlockedDecrement`
/// (Ghidra names them in the wrappers `0040b460` / `004019a0`).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;

/// `NiFrustum::NiFrustum` (`00a71b70`), `__thiscall(this, ortho)`.
const FRUSTUM_CONSTRUCT: u32 = 0x00a7_1b70;
/// `NiFrustum::GetViewerString` (`00a71bc0`), `__thiscall(this, name) ->
/// string`.
const FRUSTUM_GET_VIEWER_STRING: u32 = 0x00a7_1bc0;
/// `NiPoint3::GetViewerString` (`00a59af0`), `__thiscall(this, name) ->
/// string`.
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
/// `NiGetViewerString` (`00a7d330`), `__cdecl(name) -> string`.
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
/// `NiGetViewerString_ov9` (`00a7d550`), `__cdecl(name, float) -> string`.
const GET_VIEWER_STRING_FLOAT: u32 = 0x00a7_d550;
/// Grows or shrinks a pointer array to `size` elements (`0096ad30`,
/// `__thiscall(array, size)`): this is the `NiTArray` resize shape (it
/// sets `m_usMaxSize` and reallocates `m_pBase`).
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
/// Stores a pointer at an index of a pointer array and keeps `m_usSize` and
/// `m_usESize` (`0096ae90`, `__thiscall(array, index, &value)`).
const ARRAY_SET_AT: u32 = 0x0096_ae90;

/// The global object (`011f4748`) that [`fn_00a6fa30`] gets as `this` from
/// [`fn_00a71080`]; other units call it the renderer singleton.
const RENDERER_OBJECT: u32 = 0x011f_4748;
/// `4294967296.0` as a `float` (the unsigned-integer fix-up of `FILD`).
const UNSIGNED_BIAS: u32 = 0x0109_9378;
/// `0.5` (`double`).
const HALF: u32 = 0x0101_1588;
/// `-1.0` (`float`).
const MINUS_ONE: u32 = 0x0101_2054;
/// `0.5` (`float`): the smallest squared length `LookAtWorldPoint` accepts
/// for a cross product.
const MIN_AXIS_LENGTH_SQUARED: u32 = 0x0101_6248;
/// The float `1e-10` widened to a `double` (bytes `00 00 00 e0 df 7c db
/// 3d`): the smallest squared distance `LookAtWorldPoint` accepts.
const MIN_DISTANCE_SQUARED: u32 = 0x0109_cc78;
/// `2.0` (`double`).
const TWO: u32 = 0x0101_1590;
/// The constants the constructor loads: far plane (`2.0`), top and right
/// edge (`0.5`), bottom and left edge (`-0.5`), minimum near plane (`0.1`)
/// and the largest far-to-near ratio (`10000.0`), all `float`.
const DEFAULT_FAR: u32 = 0x0101_62c0;
const DEFAULT_HALF_EXTENT: u32 = 0x0101_6248;
const DEFAULT_NEGATIVE_HALF_EXTENT: u32 = 0x0102_295c;
const DEFAULT_MIN_NEAR: u32 = 0x0101_e2bc;
const DEFAULT_MAX_FAR_NEAR_RATIO: u32 = 0x0102_2958;
/// Words that `UpdateWorldBound` copies into a new bound (`011f426c` to
/// `011f4274`).
const BOUND_DEFAULT_WORDS: u32 = 0x011f_426c;

/// Vtable slots of the camera used here (Xbox PDB names, offsets from the PC
/// code): `SetBound` (+0x78), `UpdateWorldData` (+0xB8) and
/// `UpdateWorldBound` (+0xBC).
const SLOT_SET_BOUND: u32 = 0x78;
const SLOT_UPDATE_WORLD_DATA: u32 = 0xb8;
const SLOT_UPDATE_WORLD_BOUND: u32 = 0xbc;
/// The slot of the stream that `SaveBinary` calls with 0 (`0x30`).
const STREAM_SLOT_SAVE_LINK_ID: u32 = 0x30;
/// Slots of the stream's reader and writer objects: read (`+8`) and write
/// (`+0xC`), called as `(object, buffer, size, &size, 1) -> bytes`.
const IO_SLOT_READ: u32 = 0x08;
const IO_SLOT_WRITE: u32 = 0x0c;
/// Stream fields: the reader object (`+0x24C`), the writer object (`+0x250`),
/// the file version (`+0xD8`) and a 16-bit value of the stream (`+0x284`).
const STREAM_READER: u32 = 0x24c;
const STREAM_WRITER: u32 = 0x250;
const STREAM_VERSION: u32 = 0xd8;
const STREAM_UNUSED_USHORT: u32 = 0x284;
/// File versions the stream code tests (`10.0.1.2`, `10.0.1.5`, `4.1.0.4`,
/// `10.0.1.7` and `10.0.1.15` written as the game compares them).
const VERSION_A000102: u32 = 0x0a00_0102;
const VERSION_A000105: u32 = 0x0a00_0105;
const VERSION_4010004: u32 = 0x0401_0004;
const VERSION_A000107: u32 = 0x0a00_0107;
const VERSION_A00010F: u32 = 0x0a00_010f;
/// Slot 0xC8 of the object `fn_00a6fa30` is called on (returns the group to
/// measure when none is given), and the slots `0x8C` / `0x90` of that group
/// (Xbox PDB names of those offsets on the render-target group: `GetWidth`,
/// `GetHeight`; unconfirmed on PC), each called with 0.
const RENDERER_SLOT_C8: u32 = 0xc8;
const GROUP_SLOT_8C: u32 = 0x8c;
const GROUP_SLOT_90: u32 = 0x90;

/// The viewer-string names `GetViewerStrings` passes (read from the exe).
const NAME_VIEW_FRUSTUM: u32 = 0x0109_ccf4;
const NAME_PORT: u32 = 0x0109_ccec;
const NAME_WORLD_DIR: u32 = 0x0109_cce0;
const NAME_WORLD_UP: u32 = 0x0109_ccd4;
const NAME_WORLD_RIGHT: u32 = 0x0109_ccc4;
const NAME_MIN_NEAR_PLANE_DIST: u32 = 0x0109_ccb0;
const NAME_MAX_FAR_NEAR_RATIO: u32 = 0x0109_cc9c;
/// `"%s = (L=%g,R=%g,T=%g,B=%g)"`.
const RECT_FORMAT: u32 = 0x0109_cc80;

layout! {
    /// `NiCamera` (Xbox PDB), 0x114 bytes on PC. Offsets are the PC ones
    /// (Xbox PDB offset minus 0x24).
    pub struct NiCamera: 0x114 {
        /// `m_aafWorldToCam` (Xbox PDB), row 0: the camera-space x row.
        0x9C m_aafWorldToCam_0_0: f32,
        0xA0 m_aafWorldToCam_0_1: f32,
        0xA4 m_aafWorldToCam_0_2: f32,
        0xA8 m_aafWorldToCam_0_3: f32,
        /// Row 1: the camera-space y row.
        0xAC m_aafWorldToCam_1_0: f32,
        0xB0 m_aafWorldToCam_1_1: f32,
        0xB4 m_aafWorldToCam_1_2: f32,
        0xB8 m_aafWorldToCam_1_3: f32,
        /// Row 2: the camera-space depth row.
        0xBC m_aafWorldToCam_2_0: f32,
        0xC0 m_aafWorldToCam_2_1: f32,
        0xC4 m_aafWorldToCam_2_2: f32,
        0xC8 m_aafWorldToCam_2_3: f32,
        /// Row 3: the homogeneous w row.
        0xCC m_aafWorldToCam_3_0: f32,
        0xD0 m_aafWorldToCam_3_1: f32,
        0xD4 m_aafWorldToCam_3_2: f32,
        0xD8 m_aafWorldToCam_3_3: f32,
        /// `m_kViewFrustum.m_fLeft` (`NiFrustum`, Xbox PDB).
        0xDC m_kViewFrustum_m_fLeft: f32,
        /// `m_kViewFrustum.m_fRight`.
        0xE0 m_kViewFrustum_m_fRight: f32,
        /// `m_kViewFrustum.m_fTop`.
        0xE4 m_kViewFrustum_m_fTop: f32,
        /// `m_kViewFrustum.m_fBottom`.
        0xE8 m_kViewFrustum_m_fBottom: f32,
        /// `m_kViewFrustum.m_fNear`.
        0xEC m_kViewFrustum_m_fNear: f32,
        /// `m_kViewFrustum.m_fFar`.
        0xF0 m_kViewFrustum_m_fFar: f32,
        /// `m_kViewFrustum.m_bOrtho`.
        0xF4 m_kViewFrustum_m_bOrtho: bool,
        /// `m_fMinNearPlaneDist` (Xbox PDB).
        0xF8 m_fMinNearPlaneDist: f32,
        /// `m_fMaxFarNearRatio` (Xbox PDB).
        0xFC m_fMaxFarNearRatio: f32,
        /// `m_kPort.m_left` (`NiRect<float>`, Xbox PDB).
        0x100 m_kPort_m_left: f32,
        /// `m_kPort.m_right`.
        0x104 m_kPort_m_right: f32,
        /// `m_kPort.m_top`.
        0x108 m_kPort_m_top: f32,
        /// `m_kPort.m_bottom`.
        0x10C m_kPort_m_bottom: f32,
        /// `m_fLODAdjust` (Xbox PDB).
        0x110 m_fLODAdjust: f32,
    }
}

/// The nine floats of the world rotation (`NiAVObject::m_kWorld.m_Rotate`),
/// row `row`, column `column`.
fn world_rotation(e: &Engine, this: Ptr<NiCamera>, row: u32, column: u32) -> f64 {
    e.mem
        .f32(this.addr() + WORLD_ROTATE + 4 * (3 * row + column)) as f64
}

/// One component of the world translation (`NiAVObject::m_kWorld.m_Translate`).
fn world_translation(e: &Engine, this: Ptr<NiCamera>, axis: u32) -> f64 {
    e.mem.f32(this.addr() + WORLD_TRANSLATE + 4 * axis) as f64
}

/// An entry of the world-to-camera matrix.
fn world_to_cam(e: &Engine, this: Ptr<NiCamera>, row: u32, column: u32) -> f64 {
    e.mem
        .f32(this.addr() + WORLD_TO_CAM + 4 * (4 * row + column)) as f64
}

fn set_world_to_cam(e: &mut Engine, this: Ptr<NiCamera>, row: u32, column: u32, value: f64) {
    e.mem.set_f32(
        this.addr() + WORLD_TO_CAM + 4 * (4 * row + column),
        value as f32,
    );
}

/// Row `row` of the matrix applied to a point, summed in the order the code
/// adds: `((m0 * x + m1 * y) + m2 * z) + m3`.
fn transform_row(e: &Engine, this: Ptr<NiCamera>, row: u32, x: f64, y: f64, z: f64) -> f64 {
    world_to_cam(e, this, row, 0) * x
        + world_to_cam(e, this, row, 1) * y
        + world_to_cam(e, this, row, 2) * z
        + world_to_cam(e, this, row, 3)
}

/// [`transform_row`] as the x87 code runs it on the point at `point`: `FLD` the
/// matrix entry, `FMUL` the point's float in memory, `FADDP`, and `FADD` the
/// fourth entry in memory, with the x87 choice of NaN.
fn transform_row_x87(e: &Engine, this: Ptr<NiCamera>, row: u32, point: u32) -> f64 {
    let entry = |column: u32| this.addr() + WORLD_TO_CAM + 4 * (4 * row + column);
    let product = |column: u32| {
        x87_mul(
            x87_load32(e.mem.u32(entry(column))),
            x87_m32(e.mem.u32(point + 4 * column)),
        )
    };
    let sum = x87_add(x87_add(product(0), product(1)), product(2));
    x87_add(sum, x87_m32(e.mem.u32(entry(3))))
}

/// [`map_to_port`] as `NiCamera::WorldPtToScreenPt` inlines it, on the x87
/// stack: `FLD right`, `FSUB left` (and `FLD top`, `FSUB bottom`; `FLD right`,
/// `FADD left`; `FLD bottom`, `FADD top`), the double `0.5`, the coordinates
/// read back from `out_x` / `out_y`, with the x87 choice of NaN.
fn map_to_port_x87(e: &mut Engine, this: Ptr<NiCamera>, out_x: Ptr, out_y: Ptr) {
    let half = e.global::<f64>(HALF);
    let me = this.addr();
    let ld = |e: &Engine, address: u32| x87_load32(e.mem.u32(address));
    let m = |e: &Engine, address: u32| x87_m32(e.mem.u32(address));
    let (left, right, top, bottom) = (me + 0x100, me + 0x104, me + 0x108, me + 0x10c);
    let width = x87_mul(x87_sub(ld(e, right), m(e, left)), half);
    let scaled_x = x87_mul(width, ld(e, out_x.addr()));
    e.mem.set_u32(out_x.addr(), x87_store32(scaled_x));
    let height = x87_mul(x87_sub(ld(e, top), m(e, bottom)), half);
    let scaled_y = x87_mul(height, m(e, out_y.addr()));
    e.mem.set_u32(out_y.addr(), x87_store32(scaled_y));
    let centre_x = x87_mul(x87_add(ld(e, right), m(e, left)), half);
    let shifted_x = x87_add(centre_x, m(e, out_x.addr()));
    e.mem.set_u32(out_x.addr(), x87_store32(shifted_x));
    let centre_y = x87_mul(half, x87_add(ld(e, bottom), m(e, top)));
    let shifted_y = x87_add(centre_y, m(e, out_y.addr()));
    e.mem.set_u32(out_y.addr(), x87_store32(shifted_y));
}

/// The point at `p` (three floats) as `f64`s.
fn point(e: &Engine, p: Ptr) -> (f64, f64, f64) {
    (
        e.mem.f32(p.addr()) as f64,
        e.mem.f32(p.addr() + 4) as f64,
        e.mem.f32(p.addr() + 8) as f64,
    )
}

/// `(y * y + x * x) + z * z` as the code evaluates the squared length, stored
/// in a `float`.
fn squared_length(e: &Engine, vector: u32) -> f32 {
    let x = e.mem.f32(vector) as f64;
    let y = e.mem.f32(vector + 4) as f64;
    let z = e.mem.f32(vector + 8) as f64;
    ((y * y + x * x) + z * z) as f32
}

/// Copies `words` 32-bit words.
fn copy_words(e: &mut Engine, destination: u32, source: u32, words: u32) {
    for i in 0..words {
        let value = e.mem.u32(source + 4 * i);
        e.mem.set_u32(destination + 4 * i, value);
    }
}

/// A zeroed `NiUpdateData` as the camera code builds it on the stack:
/// `fTime` = 0 and the five flags clear (Xbox PDB `NiUpdateData`, 0xC bytes).
fn clear_update_data(e: &mut Engine, update_data: u32) {
    e.mem.set_f32(update_data, 0.0);
    for flag in 4..9 {
        e.mem.set_u8(update_data + flag, 0);
    }
}

// Translated from 00a6fa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Turns a pixel position into a position in `[0, 1]` of the render target
/// group: `x / width` and `1 - y / height`, where the sizes come from the
/// group's virtual methods `+0x8C` and `+0x90` (each called with 0). When
/// `target_group` is null the group is the result of `this`'s virtual method
/// `+0xC8`. Returns false, writing nothing, when either size is 0. The
/// pixel coordinates and sizes are treated as unsigned (a negative `FILD`
/// result gets 2^32 added). The engine map has no name for this function;
/// [`fn_00a71080`] calls it with the object stored at `011f4748`.
pub fn fn_00a6fa30(
    e: &mut Engine,
    this: Ptr,
    pixel_x: u32,
    pixel_y: u32,
    out_x: Ptr,
    out_y: Ptr,
    target_group: Ptr,
) -> bool {
    let group = if target_group.is_null() {
        e.vcall(this.addr(), RENDERER_SLOT_C8, &args![]).u32()
    } else {
        target_group.addr()
    };
    let width = e.vcall(group, GROUP_SLOT_8C, &args![0u32]).u32();
    let height = e.vcall(group, GROUP_SLOT_90, &args![0u32]).u32();
    if width == 0 || height == 0 {
        return false;
    }
    let bias = e.global::<f32>(UNSIGNED_BIAS) as f64;
    let unsigned = |value: u32| {
        let signed = value as i32 as f64;
        if (value as i32) < 0 {
            signed + bias
        } else {
            signed
        }
    };
    let x = unsigned(pixel_x) / unsigned(width);
    e.mem.set_f32(out_x.addr(), x as f32);
    let y = 1.0 - unsigned(pixel_y) / unsigned(height);
    e.mem.set_f32(out_y.addr(), y as f32);
    true
}

// Translated from 00a6faf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::SetViewFrustum` (Xbox PDB): copies the frustum at `frustum`
/// (left, right, top, bottom, near, far, ortho byte) into the camera, except
/// the near plane: that is the frustum's near plane raised to
/// `far / m_fMaxFarNearRatio` when that is larger, then raised to
/// `m_fMinNearPlaneDist` when that is larger.
pub fn ni_camera_set_view_frustum(e: &mut Engine, this: Ptr<NiCamera>, frustum: Ptr) {
    let near = e.mem.f32(frustum.addr() + 0x10);
    e.set(this, NiCamera::m_kViewFrustum_m_fNear, near);
    let far = e.mem.f32(frustum.addr() + 0x14);
    let ratio = e.get(this, NiCamera::m_fMaxFarNearRatio);
    let limit = (far as f64 / ratio as f64) as f32;
    if near < limit {
        e.set(this, NiCamera::m_kViewFrustum_m_fNear, limit);
    }
    let current = e.get(this, NiCamera::m_kViewFrustum_m_fNear);
    let minimum = e.get(this, NiCamera::m_fMinNearPlaneDist);
    if minimum > current {
        e.set(this, NiCamera::m_kViewFrustum_m_fNear, minimum);
    }
    let left = e.mem.f32(frustum.addr());
    e.set(this, NiCamera::m_kViewFrustum_m_fLeft, left);
    let right = e.mem.f32(frustum.addr() + 4);
    e.set(this, NiCamera::m_kViewFrustum_m_fRight, right);
    let top = e.mem.f32(frustum.addr() + 8);
    e.set(this, NiCamera::m_kViewFrustum_m_fTop, top);
    let bottom = e.mem.f32(frustum.addr() + 0xc);
    e.set(this, NiCamera::m_kViewFrustum_m_fBottom, bottom);
    e.set(this, NiCamera::m_kViewFrustum_m_fFar, far);
    let ortho = e.mem.u8(frustum.addr() + 0x18);
    e.mem.set_u8(this.addr() + 0xf4, ortho);
}

// Translated from 00a6fb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Re-fits the view frustum to an aspect `ratio` while keeping its centre
/// (the engine map has no name). With `fit_vertical` set the top and bottom
/// become `centre_y +/- (right - left) / ratio * 0.5`; otherwise the left
/// and right become `centre_x +/- (top - bottom) * ratio * 0.5`.
pub fn fn_00a6fb90(e: &mut Engine, this: Ptr<NiCamera>, ratio: f32, fit_vertical: bool) {
    // The x87 code: `FLD` a field, `FSUB`/`FADD`/`FMUL`/`FDIV` a float in memory
    // (a field, the ratio argument or a scratch float), stores rounding to a
    // float; every operation keeps the x87 choice of NaN.
    let half = e.global::<f64>(HALF);
    let me = this.addr();
    let ld = |e: &Engine, offset: u32| x87_load32(e.mem.u32(me + offset));
    let m = |e: &Engine, offset: u32| x87_m32(e.mem.u32(me + offset));
    let (left, right, top, bottom) = (0xdc, 0xe0, 0xe4, 0xe8);
    // With `fit_vertical`: `(right - left) / ratio` and the centre of bottom and
    // top, written to top and bottom; else `(top - bottom) * ratio` and the
    // centre of right and left, written to right and left.
    let (span, sum, low, high) = if fit_vertical {
        let width = x87_store32(x87_sub(ld(e, right), m(e, left)));
        let quotient = x87_div(x87_load32(width), x87_m32(ratio.to_bits()));
        (quotient, x87_add(ld(e, bottom), m(e, top)), bottom, top)
    } else {
        let height = x87_store32(x87_sub(ld(e, top), m(e, bottom)));
        let product = x87_mul(x87_load32(height), x87_m32(ratio.to_bits()));
        (product, x87_add(ld(e, right), m(e, left)), left, right)
    };
    let delta = x87_load32(x87_store32(x87_mul(span, half)));
    let centre = x87_load32(x87_store32(x87_mul(half, sum)));
    e.mem
        .set_u32(me + high, x87_store32(x87_add(centre, delta)));
    e.mem.set_u32(me + low, x87_store32(x87_sub(centre, delta)));
}

// Translated from 00a6fc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::WorldPtToScreenPt` (Xbox PDB): projects the world point
/// `world_point` with the world-to-camera matrix, writes the port position
/// to `out_x` / `out_y` and returns whether the point is in front of the
/// camera (`w > min_w`) and inside the view (both normalised coordinates in
/// `[-1, 1]`). The outputs are written as soon as `w` passes: a point
/// outside the view returns false with the normalised coordinates stored.
pub fn ni_camera_world_pt_to_screen_pt(
    e: &mut Engine,
    this: Ptr<NiCamera>,
    world_point: Ptr,
    out_x: Ptr,
    out_y: Ptr,
    min_w: f32,
) -> bool {
    let point = world_point.addr();
    let w = x87_store32(transform_row_x87(e, this, 3, point));
    if !(min_w < f32::from_bits(w)) {
        return false;
    }
    let inverse = x87_store32(x87_div(1.0, x87_load32(w)));
    let screen_x = x87_store32(transform_row_x87(e, this, 0, point));
    e.mem.set_u32(out_x.addr(), screen_x);
    let screen_y = x87_store32(transform_row_x87(e, this, 1, point));
    e.mem.set_u32(out_y.addr(), screen_y);
    // `FLD [out_x]` times the scratch `inverse`; `inverse` times `[out_y]`.
    let normalised_x = x87_mul(x87_load32(e.mem.u32(out_x.addr())), x87_load32(inverse));
    e.mem.set_u32(out_x.addr(), x87_store32(normalised_x));
    let normalised_y = x87_mul(x87_load32(inverse), x87_m32(e.mem.u32(out_y.addr())));
    e.mem.set_u32(out_y.addr(), x87_store32(normalised_y));
    let normalised_x = e.mem.f32(out_x.addr());
    let normalised_y = e.mem.f32(out_y.addr());
    let minus_one = e.global::<f32>(MINUS_ONE);
    if !(minus_one <= normalised_x) || !(normalised_x <= 1.0) {
        return false;
    }
    if !(minus_one <= normalised_y) || !(normalised_y <= 1.0) {
        return false;
    }
    map_to_port_x87(e, this, out_x, out_y);
    true
}

/// Maps normalised coordinates in `[-1, 1]` at `out_x` / `out_y` to the port:
/// `x * (right - left) * 0.5 + (right + left) * 0.5` and the same for y with
/// `(top - bottom)` and `(bottom + top)`.
fn map_to_port(e: &mut Engine, this: Ptr<NiCamera>, out_x: Ptr, out_y: Ptr) {
    let half = e.global::<f64>(HALF);
    let left = e.get(this, NiCamera::m_kPort_m_left) as f64;
    let right = e.get(this, NiCamera::m_kPort_m_right) as f64;
    let top = e.get(this, NiCamera::m_kPort_m_top) as f64;
    let bottom = e.get(this, NiCamera::m_kPort_m_bottom) as f64;
    let x = e.mem.f32(out_x.addr()) as f64;
    let scaled_x = ((right - left) * half * x) as f32;
    e.mem.set_f32(out_x.addr(), scaled_x);
    let y = e.mem.f32(out_y.addr()) as f64;
    let scaled_y = ((top - bottom) * half * y) as f32;
    e.mem.set_f32(out_y.addr(), scaled_y);
    let shifted_x = ((right + left) * half + scaled_x as f64) as f32;
    e.mem.set_f32(out_x.addr(), shifted_x);
    let shifted_y = (half * (bottom + top) + scaled_y as f64) as f32;
    e.mem.set_f32(out_y.addr(), shifted_y);
}

// Translated from 00a6fdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`ni_camera_world_pt_to_screen_pt`] but on `|w|` and without the
/// view test (the engine map has no name): when `min_w < |w|` it writes the
/// port position to `out_x` / `out_y` and the depth (`row 2` of the matrix
/// applied to the point, times `1 / |w|`) to `out_depth` and returns true;
/// otherwise it returns false and writes nothing.
pub fn fn_00a6fdb0(
    e: &mut Engine,
    this: Ptr<NiCamera>,
    world_point: Ptr,
    out_x: Ptr,
    out_y: Ptr,
    out_depth: Ptr,
    min_w: f32,
) -> bool {
    let (x, y, z) = point(e, world_point);
    let w = transform_row(e, this, 3, x, y, z) as f32;
    let magnitude = w.abs();
    if !(min_w < magnitude) {
        return false;
    }
    let inverse = (1.0 / magnitude as f64) as f32;
    let screen_x = transform_row(e, this, 0, x, y, z) as f32;
    e.mem.set_f32(out_x.addr(), screen_x);
    let screen_y = transform_row(e, this, 1, x, y, z) as f32;
    e.mem.set_f32(out_y.addr(), screen_y);
    let normalised_x = (screen_x as f64 * inverse as f64) as f32;
    e.mem.set_f32(out_x.addr(), normalised_x);
    let normalised_y = (screen_y as f64 * inverse as f64) as f32;
    e.mem.set_f32(out_y.addr(), normalised_y);
    map_to_port(e, this, out_x, out_y);
    let depth = transform_row(e, this, 2, x, y, z) as f32;
    e.mem
        .set_f32(out_depth.addr(), (inverse as f64 * depth as f64) as f32);
    true
}

// Translated from 00a6ff10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::RegisterStreamables` (Xbox PDB): a 5-byte jump to the
/// `NiAVObject` version (`00a59f00`); returns its result.
pub fn ni_camera_register_streamables(e: &mut Engine, this: Ptr<NiCamera>, stream: Ptr) -> bool {
    e.call(AV_OBJECT_REGISTER_STREAMABLES, &args![this, stream])
        .bool()
}

// Translated from 00a6ff20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::IsEqual` (Xbox PDB): the `NiAVObject` comparison (`00a5a260`),
/// then the frustum (left, right, top, bottom, near, far; not the ortho
/// byte), the port and `m_fLODAdjust`, each compared as floats (so a NaN is
/// never equal). `m_fMinNearPlaneDist` and `m_fMaxFarNearRatio` are not
/// compared.
pub fn ni_camera_is_equal(e: &mut Engine, this: Ptr<NiCamera>, other: Ptr<NiCamera>) -> bool {
    if !e.call(AV_OBJECT_IS_EQUAL, &args![this, other]).bool() {
        return false;
    }
    for offset in [
        0xdcu32, 0xe0, 0xe4, 0xe8, 0xec, 0xf0, 0x100, 0x104, 0x108, 0x10c, 0x110,
    ] {
        if e.mem.f32(this.addr() + offset) != e.mem.f32(other.addr() + offset) {
            return false;
        }
    }
    true
}

// Translated from 00a70040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI`.
pub fn ni_camera_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_CAMERA_RTTI)
}

// Translated from 00a70050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::_scalar_deleting_destructor_` (Xbox PDB): sets the camera's
/// virtual table, runs the base destructor body (`00a5a390`) and, when bit 0
/// of `flags` is set, frees the 0x114-byte block (`00aa1460`); returns
/// `this`.
pub fn ni_camera_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiCamera>,
    flags: u32,
) -> Ptr<NiCamera> {
    e.mem.set_u32(this.addr(), NI_CAMERA_VTABLE);
    e.call(AV_OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE, &args![this, NI_CAMERA_SIZE]);
    }
    this
}

// Translated from 00a70080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fits the near and far plane to a bounding sphere (the engine map has no
/// name for it). `sphere` holds the centre and the radius. The sphere's
/// distance along the camera's world x axis (`m_kWorld` rotation column 0)
/// is `d`; if `d + radius <= 0` (the sphere is behind) it returns false (a
/// NaN `d + radius` does not fail and goes on as below).
/// Otherwise the far plane becomes `d + radius`, the near plane becomes
/// `d - radius`, raised to `(d + radius) / m_fMaxFarNearRatio` and then to
/// `m_fMinNearPlaneDist` when those are larger, then the camera's virtual
/// `UpdateWorldData` (`+0xB8`, with a cleared `NiUpdateData`) and
/// `UpdateWorldBound` (`+0xBC`) run, and it returns true.
pub fn fn_00a70080(e: &mut Engine, this: Ptr<NiCamera>, sphere: Ptr) -> bool {
    let (cx, cy, cz) = point(e, sphere);
    let radius = e.mem.f32(sphere.addr() + 0xc) as f64;
    let axis = [
        world_rotation(e, this, 0, 0) as f32,
        world_rotation(e, this, 1, 0) as f32,
        world_rotation(e, this, 2, 0) as f32,
    ];
    let dx = (cx - world_translation(e, this, 0)) as f32;
    let dy = (cy - world_translation(e, this, 1)) as f32;
    let dz = (cz - world_translation(e, this, 2)) as f32;
    let distance = ((dy as f64 * axis[1] as f64 + dx as f64 * axis[0] as f64)
        + axis[2] as f64 * dz as f64) as f32;
    let mut near = (distance as f64 - radius) as f32;
    let far = (distance as f64 + radius) as f32;
    // 00a70102 FCOM far with 0, TEST AH,0x41, JP 00a70118: only an ordered
    // far <= 0 fails; a NaN far (0x41, even parity) goes on.
    if far <= 0.0 {
        return false;
    }
    let ratio = e.get(this, NiCamera::m_fMaxFarNearRatio);
    let limit = (far as f64 / ratio as f64) as f32;
    if limit > near {
        near = limit;
    }
    let minimum = e.get(this, NiCamera::m_fMinNearPlaneDist);
    if minimum > near {
        near = minimum;
    }
    e.set(this, NiCamera::m_kViewFrustum_m_fNear, near);
    e.set(this, NiCamera::m_kViewFrustum_m_fFar, far);
    e.with_stack(0xc, |e, update_data| {
        clear_update_data(e, update_data.addr());
        e.vcall(
            this.addr(),
            SLOT_UPDATE_WORLD_DATA,
            &args![update_data.addr()],
        );
    });
    e.vcall(this.addr(), SLOT_UPDATE_WORLD_BOUND, &args![]);
    true
}

// Translated from 00a701b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::LookAtWorldPoint` (Xbox PDB): turns the camera to look at
/// `target` with `up` as the up hint. With `d` = camera world translation -
/// `target`, normalised (false if its squared length is below `1e-10`),
/// `first = up x d` and `second = d x first` (each `UnitCross`; false if a
/// squared length is below 0.5; a NaN squared length passes each test),
/// the new local rotation is the matrix with
/// columns `-d`, `second` and `first`, premultiplied by the transpose of the
/// parent's world rotation when there is a parent. It then runs the
/// camera's virtual `UpdateWorldData` (`+0xB8`, cleared `NiUpdateData`) and
/// `UpdateWorldBound` (`+0xBC`) and returns true.
pub fn ni_camera_look_at_world_point(
    e: &mut Engine,
    this: Ptr<NiCamera>,
    target: Ptr,
    up: Ptr,
) -> bool {
    let minimum_distance = e.global::<f64>(MIN_DISTANCE_SQUARED);
    let minimum_axis = e.global::<f32>(MIN_AXIS_LENGTH_SQUARED);
    // Locals: d, first, second (3 floats each), the matrix and the
    // transposed copy (9 floats each), the update data.
    e.with_stack(0x78, |e, frame| {
        let direction = frame.addr();
        let first = direction + 0xc;
        let second = direction + 0x18;
        let matrix = direction + 0x24;
        let transposed = direction + 0x48;
        let update_data = direction + 0x6c;
        for axis in 0..3 {
            let value =
                world_translation(e, this, axis) - e.mem.f32(target.addr() + 4 * axis) as f64;
            e.mem.set_f32(direction + 4 * axis, value as f32);
        }
        // Each test is FCOMP, TEST AH,0x5 and a parity jump (00a7020f JNP
        // fail, 00a70263 JNP fail, 00a702b1 JP on): only an ordered
        // less-than fails; a NaN squared length (0x5, even parity) goes on.
        if (squared_length(e, direction) as f64) < minimum_distance {
            return false;
        }
        e.call(POINT_NORMALIZE, &args![direction]);
        e.call(UNIT_CROSS, &args![up, first, direction]);
        if squared_length(e, first) < minimum_axis {
            return false;
        }
        e.call(UNIT_CROSS, &args![direction, second, first]);
        if squared_length(e, second) < minimum_axis {
            return false;
        }
        for row in 0..3u32 {
            let toward = -e.mem.f32(direction + 4 * row);
            e.mem.set_f32(matrix + 12 * row, toward);
            let side = e.mem.f32(second + 4 * row);
            e.mem.set_f32(matrix + 12 * row + 4, side);
            let ahead = e.mem.f32(first + 4 * row);
            e.mem.set_f32(matrix + 12 * row + 8, ahead);
        }
        // NiAVObject::m_pkParent (+0x18)
        let parent = e.get(this.cast::<NiAVObject>(), NiAVObject::m_pkParent);
        if parent != 0 {
            let product = e
                .call(
                    MATRIX_TRANSPOSE_TIMES,
                    &args![parent + WORLD_ROTATE, transposed, matrix],
                )
                .u32();
            copy_words(e, matrix, product, 9);
        }
        copy_words(e, this.addr() + LOCAL_ROTATE, matrix, 9);
        clear_update_data(e, update_data);
        e.vcall(this.addr(), SLOT_UPDATE_WORLD_DATA, &args![update_data]);
        e.vcall(this.addr(), SLOT_UPDATE_WORLD_BOUND, &args![]);
        true
    })
}

// Translated from 00a703a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A pick ray for a point of the view plane (the engine map has no name):
/// `a` and `b` are the offsets along the world rotation's column 2 and
/// column 1. For a perspective camera the direction written to
/// `out_direction` is `column0 + a * column2 + b * column1`, normalised
/// (`004a0c10`), and the origin written to `out_origin` is the world
/// translation. For an orthographic camera (`m_bOrtho`) the direction is
/// column 0 normalised and the origin is `translation + a * column2 + b *
/// column1`.
pub fn fn_00a703a0(
    e: &mut Engine,
    this: Ptr<NiCamera>,
    a: f32,
    b: f32,
    out_origin: Ptr,
    out_direction: Ptr,
) {
    let along_column2 = |e: &Engine, row: u32| (a as f64 * world_rotation(e, this, row, 2)) as f32;
    let along_column1 = |e: &Engine, row: u32| (b as f64 * world_rotation(e, this, row, 1)) as f32;
    if e.get(this, NiCamera::m_kViewFrustum_m_bOrtho) {
        for row in 0..3 {
            let value = e.mem.f32(this.addr() + WORLD_ROTATE + 4 * (3 * row));
            e.mem.set_f32(out_direction.addr() + 4 * row, value);
        }
        e.call(POINT_NORMALIZE, &args![out_direction]);
        for axis in 0..3 {
            let moved = world_translation(e, this, axis) + along_column2(e, axis) as f64;
            let moved = moved as f32 as f64 + along_column1(e, axis) as f64;
            e.mem.set_f32(out_origin.addr() + 4 * axis, moved as f32);
        }
    } else {
        for row in 0..3 {
            let moved = world_rotation(e, this, row, 0) + along_column2(e, row) as f64;
            let moved = moved as f32 as f64 + along_column1(e, row) as f64;
            e.mem.set_f32(out_direction.addr() + 4 * row, moved as f32);
        }
        e.call(POINT_NORMALIZE, &args![out_direction]);
        copy_words(e, out_origin.addr(), this.addr() + WORLD_TRANSLATE, 3);
    }
}

/// The clamp `ScreenSpaceBoundBound` applies to the x range and to the y
/// range: `low` and `high` into `[-1, 1]`, each comparison false for a NaN.
fn clamp_range(low: &mut f32, high: &mut f32, minus_one: f32) {
    if *low < minus_one {
        *low = minus_one;
        if *high < minus_one {
            *high = minus_one;
        } else if 1.0 < *high {
            *high = 1.0;
        }
    } else if 1.0 < *high {
        *high = 1.0;
        if 1.0 < *low {
            *low = 1.0;
        }
    }
}

// Translated from 00a705f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::ScreenSpaceBoundBound` (Xbox PDB): the screen-space box of a
/// bounding sphere. `sphere` holds the centre and radius; `out_min` and
/// `out_max` receive (x, y, z) of the box corners, x and y in `[-1, 1]`
/// (the whole view, `-1` / `1`, when a corner point has `w <= min_w`) and z
/// the depth range along the camera axis in `[0, 1]` (both 0 for an
/// orthographic camera). The x range projects the two points `centre -/+
/// radius * (d x column1)` and the y range the two points `centre -/+
/// radius * (column2 x d)`, where `d` is the vector from the camera to the
/// centre and the products are `UnitCross` (`0053d1a0`).
pub fn ni_camera_screen_space_bound_bound(
    e: &mut Engine,
    this: Ptr<NiCamera>,
    sphere: Ptr,
    out_min: Ptr,
    out_max: Ptr,
    min_w: f32,
) {
    let minus_one = e.global::<f32>(MINUS_ONE);
    let (cx, cy, cz) = point(e, sphere);
    let radius = e.mem.f32(sphere.addr() + 0xc);
    let near = e.get(this, NiCamera::m_kViewFrustum_m_fNear);
    let far = e.get(this, NiCamera::m_kViewFrustum_m_fFar);
    let ortho = e.get(this, NiCamera::m_kViewFrustum_m_bOrtho);
    // d, the unit-cross results (3 floats each) and a column copy.
    e.with_stack(0x30, |e, frame| {
        let delta = frame.addr();
        let result = delta + 0xc;
        let column = delta + 0x18;
        let dx = (cx - world_translation(e, this, 0)) as f32;
        let dy = (cy - world_translation(e, this, 1)) as f32;
        let dz = (cz - world_translation(e, this, 2)) as f32;
        e.mem.set_f32(delta, dx);
        e.mem.set_f32(delta + 4, dy);
        e.mem.set_f32(delta + 8, dz);
        let inverse_depth = (1.0 / (far as f64 - near as f64)) as f32;
        let axis = [
            world_rotation(e, this, 0, 0) as f32,
            world_rotation(e, this, 1, 0) as f32,
            world_rotation(e, this, 2, 0) as f32,
        ];
        let along = ((axis[1] as f64 * dy as f64 + axis[0] as f64 * dx as f64)
            + axis[2] as f64 * dz as f64) as f32;
        let depth = (along as f64 - near as f64) as f32;
        if ortho {
            e.mem.set_f32(out_min.addr() + 8, 0.0);
            e.mem.set_f32(out_max.addr() + 8, 0.0);
        } else {
            let low = (depth as f64 - radius as f64) as f32;
            e.mem.set_f32(
                out_min.addr() + 8,
                (inverse_depth as f64 * low as f64) as f32,
            );
            let high = (depth as f64 + radius as f64) as f32;
            e.mem.set_f32(
                out_max.addr() + 8,
                (inverse_depth as f64 * high as f64) as f32,
            );
        }
        let ranges = projected_ranges(e, this, delta, result, column, (cx, cy, cz), radius, min_w);
        match ranges {
            Some((min_x, max_x, min_y, max_y)) => {
                e.mem.set_f32(out_min.addr(), min_x);
                e.mem.set_f32(out_max.addr(), max_x);
                e.mem.set_f32(out_min.addr() + 4, min_y);
                e.mem.set_f32(out_max.addr() + 4, max_y);
            }
            None => {
                e.mem.set_f32(out_min.addr() + 4, minus_one);
                e.mem.set_f32(out_min.addr(), minus_one);
                e.mem.set_f32(out_max.addr() + 4, 1.0);
                e.mem.set_f32(out_max.addr(), 1.0);
            }
        }
        let mut min_x = e.mem.f32(out_min.addr());
        let mut max_x = e.mem.f32(out_max.addr());
        clamp_range(&mut min_x, &mut max_x, minus_one);
        e.mem.set_f32(out_min.addr(), min_x);
        e.mem.set_f32(out_max.addr(), max_x);
        let mut min_y = e.mem.f32(out_min.addr() + 4);
        let mut max_y = e.mem.f32(out_max.addr() + 4);
        clamp_range(&mut min_y, &mut max_y, minus_one);
        e.mem.set_f32(out_min.addr() + 4, min_y);
        e.mem.set_f32(out_max.addr() + 4, max_y);
        let mut min_z = e.mem.f32(out_min.addr() + 8);
        let mut max_z = e.mem.f32(out_max.addr() + 8);
        if min_z < 0.0 {
            min_z = 0.0;
            if max_z < 0.0 {
                max_z = 0.0;
            } else if 1.0 < max_z {
                max_z = 1.0;
            }
        } else if 1.0 < max_z {
            max_z = 1.0;
            if 1.0 < min_z {
                min_z = 1.0;
            }
        }
        e.mem.set_f32(out_min.addr() + 8, min_z);
        e.mem.set_f32(out_max.addr() + 8, max_z);
    });
}

/// The four screen-space limits of `ScreenSpaceBoundBound`, or `None` when
/// one of the four projected points has `w <= min_w` (the code then writes
/// the whole view). The outputs stay untouched until the end: the original
/// writes the earlier ones first, but a failure overwrites all four.
#[allow(clippy::too_many_arguments)]
fn projected_ranges(
    e: &mut Engine,
    this: Ptr<NiCamera>,
    delta: u32,
    result: u32,
    column: u32,
    centre: (f64, f64, f64),
    radius: f32,
    min_w: f32,
) -> Option<(f32, f32, f32, f32)> {
    let (cx, cy, cz) = centre;
    let radius = radius as f64;
    // The offset `radius * unit` of one of the two tangent directions, as
    // floats, and the two points centre -/+ offset.
    let offsets = |e: &Engine, unit: u32| {
        [
            (e.mem.f32(unit) as f64 * radius) as f32,
            (e.mem.f32(unit + 4) as f64 * radius) as f32,
            (radius * e.mem.f32(unit + 8) as f64) as f32,
        ]
    };
    let project = |e: &Engine, p: [f32; 3], row: u32| -> Option<f32> {
        let (x, y, z) = (p[0] as f64, p[1] as f64, p[2] as f64);
        let w = transform_row(e, this, 3, x, y, z) as f32;
        if !(min_w < w) {
            return None;
        }
        let numerator = transform_row(e, this, row, x, y, z);
        Some((numerator / w as f64) as f32)
    };
    // First pair: d x column1 gives the x range.
    for row in 0..3 {
        e.mem
            .set_f32(column + 4 * row, world_rotation(e, this, row, 1) as f32);
    }
    e.call(UNIT_CROSS, &args![delta, result, column]);
    let offset = offsets(e, result);
    let low = [
        (cx - offset[0] as f64) as f32,
        (cy - offset[1] as f64) as f32,
        (cz - offset[2] as f64) as f32,
    ];
    let min_x = project(e, low, 0)?;
    let high = [
        (cx + offset[0] as f64) as f32,
        (offset[1] as f64 + cy) as f32,
        (offset[2] as f64 + cz) as f32,
    ];
    let max_x = project(e, high, 0)?;
    // Second pair: column2 x d gives the y range.
    for row in 0..3 {
        e.mem
            .set_f32(column + 4 * row, world_rotation(e, this, row, 2) as f32);
    }
    e.call(UNIT_CROSS, &args![column, result, delta]);
    let offset = offsets(e, result);
    let low = [
        (cx - offset[0] as f64) as f32,
        (cy - offset[1] as f64) as f32,
        (cz - offset[2] as f64) as f32,
    ];
    let min_y = project(e, low, 1)?;
    let high = [
        (cx + offset[0] as f64) as f32,
        (offset[1] as f64 + cy) as f32,
        (offset[2] as f64 + cz) as f32,
    ];
    let max_y = project(e, high, 1)?;
    Some((min_x, max_x, min_y, max_y))
}

// Translated from 00a70b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::UpdateWorldBound` (Xbox PDB): when the camera has no world
/// bound (`+0x20`) it allocates 0x10 bytes (`00aa3e40` on the object at
/// `011f6238`), installs them with the virtual `SetBound` (`+0x78`) and
/// fills them with the three words at `011f426c`, `011f4270`, `011f4274` and
/// radius 0; then the bound's centre becomes the world translation.
pub fn ni_camera_update_world_bound(e: &mut Engine, this: Ptr<NiCamera>) {
    let bound = e.mem.u32(this.addr() + WORLD_BOUND);
    if bound == 0 {
        let block = e.call(BOUND_ALLOCATE, &args![BOUND_POOL, 0x10u32]).u32();
        e.vcall(this.addr(), SLOT_SET_BOUND, &args![block]);
        let bound = e.mem.u32(this.addr() + WORLD_BOUND);
        copy_words(e, bound, BOUND_DEFAULT_WORDS, 3);
        e.mem.set_f32(bound + 0xc, 0.0);
    }
    let bound = e.mem.u32(this.addr() + WORLD_BOUND);
    copy_words(e, bound, this.addr() + WORLD_TRANSLATE, 3);
}

// Translated from 00a70ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::WorldToCameraMatrix` (Xbox PDB): rebuilds the 4x4
/// `m_aafWorldToCam` from the world transform and the frustum. Row 3 is the
/// homogeneous w row, row 0 and row 1 the x and y rows (scaled by the
/// frustum width and height, shifted by its centre for a perspective
/// camera), row 2 the depth row (scaled by `far / (far - near)` for a
/// perspective camera). The camera looks along the world rotation's column
/// 0 and its x / y axes are columns 2 and 1.
pub fn ni_camera_world_to_camera_matrix(e: &mut Engine, this: Ptr<NiCamera>) {
    let mut r = [[0f64; 3]; 3];
    for (row, entries) in r.iter_mut().enumerate() {
        for (column, entry) in entries.iter_mut().enumerate() {
            *entry = world_rotation(e, this, row as u32, column as u32);
        }
    }
    let t = [
        world_translation(e, this, 0),
        world_translation(e, this, 1),
        world_translation(e, this, 2),
    ];
    // The products and sums go through the x87 stack with its choice of NaN
    // (`mul`, `add`); the rotation and translation are copied into scratch
    // floats first and loaded from there (quiet). `rounded` is a store to a
    // scratch float and the load back. The frustum fields that are memory
    // operands (`FADD`/`FSUB` of left, top, bottom and near) keep a signalling
    // NaN as one (`x87_m32`).
    let (mul, add, sub, div) = (x87_mul, x87_add, x87_sub, x87_div);
    let rounded = |value: f64| x87_load32(x87_store32(value));
    // Minus the dot product of the translation with columns 2, 1 and 0.
    let dot = |column: usize| {
        let sum = add(mul(t[0], r[0][column]), mul(t[1], r[1][column]));
        rounded(add(sum, mul(t[2], r[2][column])))
    };
    let n2 = -dot(2);
    let n1 = -dot(1);
    let n0 = -dot(0);
    let me = this.addr();
    let ld = |e: &Engine, offset: u32| x87_load32(e.mem.u32(me + offset));
    let m = |e: &Engine, offset: u32| x87_m32(e.mem.u32(me + offset));
    let (left, right, top, bottom, near, far) = (0xdc, 0xe0, 0xe4, 0xe8, 0xec, 0xf0);
    let sum_horizontal = rounded(add(ld(e, right), m(e, left)));
    let sum_vertical = rounded(add(ld(e, bottom), m(e, top)));
    let inverse_width = rounded(div(1.0, sub(ld(e, right), m(e, left))));
    let inverse_height = rounded(div(1.0, sub(ld(e, top), m(e, bottom))));
    let inverse_depth = rounded(div(1.0, sub(ld(e, far), m(e, near))));
    let two = e.global::<f64>(TWO);
    let a = rounded(mul(two, inverse_width));
    if e.get(this, NiCamera::m_kViewFrustum_m_bOrtho) {
        let b = rounded(mul(two, inverse_height));
        set_world_to_cam(e, this, 0, 0, mul(a, r[0][2]));
        set_world_to_cam(e, this, 0, 1, mul(a, r[1][2]));
        set_world_to_cam(e, this, 0, 2, mul(a, r[2][2]));
        let centre_x = rounded(mul(inverse_width, sum_horizontal));
        set_world_to_cam(e, this, 0, 3, add(centre_x, mul(a, n2)));
        set_world_to_cam(e, this, 1, 0, mul(b, r[0][1]));
        set_world_to_cam(e, this, 1, 1, mul(b, r[1][1]));
        set_world_to_cam(e, this, 1, 2, mul(b, r[2][1]));
        let centre_y = rounded(mul(inverse_height, sum_vertical));
        set_world_to_cam(e, this, 1, 3, add(centre_y, mul(b, n1)));
        set_world_to_cam(e, this, 2, 0, mul(inverse_depth, r[0][0]));
        set_world_to_cam(e, this, 2, 1, mul(inverse_depth, r[1][0]));
        set_world_to_cam(e, this, 2, 2, mul(inverse_depth, r[2][0]));
        let near_shift = rounded(mul(-ld(e, near), inverse_depth));
        set_world_to_cam(e, this, 2, 3, add(mul(n0, inverse_depth), near_shift));
        set_world_to_cam(e, this, 3, 0, 0.0);
        set_world_to_cam(e, this, 3, 1, 0.0);
        set_world_to_cam(e, this, 3, 2, 0.0);
        set_world_to_cam(e, this, 3, 3, 1.0);
    } else {
        let p = rounded(mul(-sum_horizontal, inverse_width));
        let b = rounded(mul(two, inverse_height));
        let q = rounded(mul(-sum_vertical, inverse_height));
        let far_scale = rounded(mul(ld(e, far), inverse_depth));
        set_world_to_cam(e, this, 0, 0, add(mul(a, r[0][2]), mul(p, r[0][0])));
        set_world_to_cam(e, this, 0, 1, add(mul(p, r[1][0]), mul(a, r[1][2])));
        set_world_to_cam(e, this, 0, 2, add(mul(p, r[2][0]), mul(a, r[2][2])));
        set_world_to_cam(e, this, 0, 3, add(mul(p, n0), mul(a, n2)));
        set_world_to_cam(e, this, 1, 0, add(mul(q, r[0][0]), mul(b, r[0][1])));
        set_world_to_cam(e, this, 1, 1, add(mul(q, r[1][0]), mul(b, r[1][1])));
        set_world_to_cam(e, this, 1, 2, add(mul(q, r[2][0]), mul(b, r[2][1])));
        set_world_to_cam(e, this, 1, 3, add(mul(q, n0), mul(b, n1)));
        set_world_to_cam(e, this, 2, 0, mul(far_scale, r[0][0]));
        set_world_to_cam(e, this, 2, 1, mul(far_scale, r[1][0]));
        set_world_to_cam(e, this, 2, 2, mul(far_scale, r[2][0]));
        let near_shift = rounded(mul(-ld(e, near), far_scale));
        set_world_to_cam(e, this, 2, 3, add(mul(far_scale, n0), near_shift));
        set_world_to_cam(e, this, 3, 0, r[0][0]);
        set_world_to_cam(e, this, 3, 1, r[1][0]);
        set_world_to_cam(e, this, 3, 2, r[2][0]);
        set_world_to_cam(e, this, 3, 3, n0);
    }
}

// Translated from 00a70f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the camera into `target` for a clone (the engine map has no
/// name): the base members (`00a5ac90` with the copy process), the 64-byte
/// world-to-camera matrix (`memcpy`), the world translation words, the
/// frustum (7 words from `+0xDC`, including the ortho byte), the port,
/// `m_fLODAdjust`, `m_fMinNearPlaneDist` and `m_fMaxFarNearRatio`. A null
/// `target` faults, as in the game.
pub fn fn_00a70f60(e: &mut Engine, this: Ptr<NiCamera>, target: Ptr<NiCamera>, copy_process: u32) {
    e.call(COPY_MEMBERS, &args![this, target, copy_process]);
    e.call(
        MEMCPY,
        &args![
            target.addr() + WORLD_TO_CAM,
            this.addr() + WORLD_TO_CAM,
            0x40u32
        ],
    );
    copy_words(
        e,
        target.addr() + WORLD_TRANSLATE,
        this.addr() + WORLD_TRANSLATE,
        3,
    );
    copy_words(e, target.addr() + 0xdc, this.addr() + 0xdc, 7);
    copy_words(e, target.addr() + 0x100, this.addr() + 0x100, 4);
    let lod = e.get(this, NiCamera::m_fLODAdjust);
    e.set(target, NiCamera::m_fLODAdjust, lod);
    let minimum = e.get(this, NiCamera::m_fMinNearPlaneDist);
    e.set(target, NiCamera::m_fMinNearPlaneDist, minimum);
    let ratio = e.get(this, NiCamera::m_fMaxFarNearRatio);
    e.set(target, NiCamera::m_fMaxFarNearRatio, ratio);
}

// Translated from 00a71020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRect<float>::GetViewerString` (Xbox PDB): allocates a buffer of
/// `strlen(name) + 0x40` bytes (`00aa1070`), formats `"%s = (L=%g,R=%g,T=%g,B=%g)"`
/// into it with `NiSprintf` (`00a305b0`; the four floats as doubles) and
/// returns the buffer. `this` is the rectangle (left, right, top, bottom).
pub fn ni_rect_float_get_viewer_string(e: &mut Engine, this: Ptr, name: Ptr) -> Ptr {
    let size = e.mem.cstr(name.addr()).len() as u32 + 0x40;
    let buffer = e.call(NI_ALLOCATE_BUFFER, &args![size]).u32();
    let left = e.mem.f32(this.addr()) as f64;
    let right = e.mem.f32(this.addr() + 4) as f64;
    let top = e.mem.f32(this.addr() + 8) as f64;
    let bottom = e.mem.f32(this.addr() + 0xc) as f64;
    e.call(
        NI_SPRINTF,
        &args![buffer, size, RECT_FORMAT, name, left, right, top, bottom],
    );
    Ptr::new(buffer)
}

// Translated from 00a71080 (decompiled, FalloutNV.exe 1.4.0.525)
/// A pick ray for a pixel (the engine map has no name). With the object at
/// `011f4748` (returns false when it is null) it turns the pixel into
/// normalised target coordinates ([`fn_00a6fa30`] with `target_group`), and
/// returns false unless they lie inside the port (`m_kPort`; a NaN
/// coordinate or edge counts as inside). Inside, it maps
/// them linearly onto the view frustum's left/right and bottom/top
/// extents and calls [`fn_00a703a0`] with them and the two output pointers.
pub fn fn_00a71080(
    e: &mut Engine,
    this: Ptr<NiCamera>,
    pixel_x: u32,
    pixel_y: u32,
    out_origin: Ptr,
    out_direction: Ptr,
    target_group: Ptr,
) -> bool {
    let renderer = e.global::<u32>(RENDERER_OBJECT);
    if renderer == 0 {
        return false;
    }
    e.with_stack(8, |e, normalised| {
        let out_x = normalised;
        let out_y = normalised.byte_add(4);
        if !fn_00a6fa30(
            e,
            Ptr::new(renderer),
            pixel_x,
            pixel_y,
            out_x,
            out_y,
            target_group,
        ) {
            return false;
        }
        let x = e.mem.f32(out_x.addr());
        let y = e.mem.f32(out_y.addr());
        let left = e.get(this, NiCamera::m_kPort_m_left);
        let right = e.get(this, NiCamera::m_kPort_m_right);
        let top = e.get(this, NiCamera::m_kPort_m_top);
        let bottom = e.get(this, NiCamera::m_kPort_m_bottom);
        // Each test fails only on an ordered compare, so a NaN coordinate or
        // port edge passes: right and top (00a710c8 / 00a710f2, TEST AH,0x5,
        // JNP fail) fail only when the edge is smaller, left and bottom
        // (00a710db / 00a71105, TEST AH,0x41, JZ fail) only when it is larger.
        if right < x || left > x || top < y || bottom > y {
            return false;
        }
        let frustum_top = e.get(this, NiCamera::m_kViewFrustum_m_fTop) as f64;
        let frustum_bottom = e.get(this, NiCamera::m_kViewFrustum_m_fBottom) as f64;
        let fraction_y = ((y as f64 - bottom as f64) / (top as f64 - bottom as f64)) as f32;
        let view_y = (fraction_y as f64 * (frustum_top - frustum_bottom) + frustum_bottom) as f32;
        let frustum_right = e.get(this, NiCamera::m_kViewFrustum_m_fRight) as f64;
        let frustum_left = e.get(this, NiCamera::m_kViewFrustum_m_fLeft) as f64;
        let fraction_x = ((x as f64 - left as f64) / (right as f64 - left as f64)) as f32;
        let view_x = (fraction_x as f64 * (frustum_right - frustum_left) + frustum_left) as f32;
        fn_00a703a0(e, this, view_x, view_y, out_origin, out_direction);
        true
    })
}

// Translated from 00a711b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::UpdateWorldData` (Xbox PDB): the `NiAVObject` update
/// (`00a68c60`), then [`ni_camera_world_to_camera_matrix`].
pub fn ni_camera_update_world_data(e: &mut Engine, this: Ptr<NiCamera>, update_data: Ptr) {
    e.call(AV_OBJECT_UPDATE_WORLD_DATA, &args![this, update_data]);
    ni_camera_world_to_camera_matrix(e, this);
}

/// Adds a reference to the object (`InterlockedIncrement` of its count at
/// +4), as the stream code does around a link.
fn add_reference(e: &mut Engine, object: u32) {
    e.call(INTERLOCKED_INCREMENT, &args![object + 4]);
}

/// Drops the reference: `InterlockedDecrement` of the count at +4 and, when
/// it reaches 0, the object's virtual `DeleteThis` (slot +4).
fn release_reference(e: &mut Engine, object: u32) {
    let remaining = e.call(INTERLOCKED_DECREMENT, &args![object + 4]).u32();
    if remaining == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// `count = GetNumberOfLinkIDs(); repeat count times { object =
/// GetObjectFromLinkID(); if object { add a reference; release it } }`.
/// The pair keeps the object alive across the call; an object whose count
/// was 0 is deleted.
fn touch_linked_objects(e: &mut Engine, stream: Ptr) {
    let count = e.call(STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
    for _ in 0..count {
        let object = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
        if object != 0 {
            add_reference(e, object);
            release_reference(e, object);
        }
    }
}

// Translated from 00a711d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::LinkObject` (Xbox PDB): the `NiAVObject` link step
/// (`00a5a7f0`), then the link IDs the camera's `LoadBinary` read. The first
/// one (the scene root; its object is held with an extra reference until the
/// end) and, for every list the file version has, each linked object is
/// fetched (`GetObjectFromLinkID`) and given a reference that is dropped
/// again (deleting objects that were unreferenced). Files from version
/// `4.1.0.4` on have a second list, and versions `10.0.1.7` to below
/// `10.0.1.15` a third, which is only fetched.
pub fn ni_camera_link_object(e: &mut Engine, this: Ptr<NiCamera>, stream: Ptr) {
    e.call(AV_OBJECT_LINK_OBJECT, &args![this, stream]);
    let held = e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]).u32();
    if held != 0 {
        add_reference(e, held);
    }
    touch_linked_objects(e, stream);
    let version = e.mem.u32(stream.addr() + STREAM_VERSION);
    if version >= VERSION_4010004 {
        touch_linked_objects(e, stream);
        let version = e.mem.u32(stream.addr() + STREAM_VERSION);
        if (VERSION_A000107..VERSION_A00010F).contains(&version) {
            let count = e.call(STREAM_GET_NUMBER_OF_LINK_IDS, &args![stream]).u32();
            for _ in 0..count {
                e.call(STREAM_GET_OBJECT_FROM_LINK_ID, &args![stream]);
            }
        }
    }
    if held != 0 {
        release_reference(e, held);
    }
}

// Translated from 00a712f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `NiCamera` constructor (the engine map has no name): runs the
/// `NiAVObject` constructor (`00a5aa20`), sets the virtual table, builds the
/// frustum (`NiFrustum` constructor with 0; left/bottom `-0.5`, right/top
/// `0.5`, near `1.0`, far, minimum near plane `0.1`, ratio `10000`, read
/// from the exe's constants), the port (0, 1, 1, 0), `m_fLODAdjust` 1.0,
/// then updates the world data with a cleared `NiUpdateData` (`00a68c60`),
/// the world-to-camera matrix and the world bound. Returns `this`.
pub fn fn_00a712f0(e: &mut Engine, this: Ptr<NiCamera>) -> Ptr<NiCamera> {
    e.call(AV_OBJECT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), NI_CAMERA_VTABLE);
    e.call(FRUSTUM_CONSTRUCT, &args![this.addr() + 0xdc, 0u32]);
    for field in [
        NiCamera::m_kPort_m_left,
        NiCamera::m_kPort_m_right,
        NiCamera::m_kPort_m_top,
        NiCamera::m_kPort_m_bottom,
    ] {
        e.set(this, field, 0.0);
    }
    e.set(this, NiCamera::m_kViewFrustum_m_bOrtho, false);
    e.set(this, NiCamera::m_kViewFrustum_m_fNear, 1.0);
    let far = e.global::<f32>(DEFAULT_FAR);
    e.set(this, NiCamera::m_kViewFrustum_m_fFar, far);
    let top = e.global::<f32>(DEFAULT_HALF_EXTENT);
    e.set(this, NiCamera::m_kViewFrustum_m_fTop, top);
    let bottom = e.global::<f32>(DEFAULT_NEGATIVE_HALF_EXTENT);
    e.set(this, NiCamera::m_kViewFrustum_m_fBottom, bottom);
    e.set(this, NiCamera::m_kViewFrustum_m_fLeft, bottom);
    e.set(this, NiCamera::m_kViewFrustum_m_fRight, top);
    let minimum = e.global::<f32>(DEFAULT_MIN_NEAR);
    e.set(this, NiCamera::m_fMinNearPlaneDist, minimum);
    let ratio = e.global::<f32>(DEFAULT_MAX_FAR_NEAR_RATIO);
    e.set(this, NiCamera::m_fMaxFarNearRatio, ratio);
    e.set(this, NiCamera::m_kPort_m_top, 1.0);
    e.set(this, NiCamera::m_kPort_m_right, 1.0);
    e.set(this, NiCamera::m_fLODAdjust, 1.0);
    e.set(this, NiCamera::m_kPort_m_bottom, 0.0);
    e.set(this, NiCamera::m_kPort_m_left, 0.0);
    e.with_stack(0xc, |e, update_data| {
        clear_update_data(e, update_data.addr());
        e.call(
            AV_OBJECT_UPDATE_WORLD_DATA,
            &args![this, update_data.addr()],
        );
    });
    ni_camera_world_to_camera_matrix(e, this);
    ni_camera_update_world_bound(e, this);
    this
}

// Translated from 00a713e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::CreateClone` (Xbox PDB): allocates 0x114 bytes (`00aa13e0`),
/// constructs a camera in them ([`fn_00a712f0`]), copies this camera into it
/// ([`fn_00a70f60`]) and returns it. When the allocation fails the original
/// still calls the copy with a null target (which faults) and returns 0.
pub fn ni_camera_create_clone(
    e: &mut Engine,
    this: Ptr<NiCamera>,
    copy_process: u32,
) -> Ptr<NiCamera> {
    let block = e.call(NI_ALLOCATE, &args![NI_CAMERA_SIZE]).u32();
    if block != 0 {
        let clone = fn_00a712f0(e, Ptr::new(block));
        fn_00a70f60(e, this, clone, copy_process);
        clone
    } else {
        fn_00a70f60(e, this, Ptr::NULL, copy_process);
        Ptr::NULL
    }
}

/// Reads `size` bytes from the stream's reader object into `destination`:
/// the reader's virtual method at +8 gets `(reader, destination, size,
/// &size, 1)` and returns the number of bytes, which the reader adds to its
/// position (+4). The reader is fetched from the stream (+0x24C) each time.
fn stream_read(e: &mut Engine, stream: Ptr, destination: u32, size: u32) {
    let reader = e.mem.u32(stream.addr() + STREAM_READER);
    stream_transfer(e, reader, IO_SLOT_READ, destination, size);
}

/// Writes `size` bytes from `source` through the stream's writer object
/// (+0x250, virtual method +0xC), as [`stream_read`].
fn stream_write(e: &mut Engine, stream: Ptr, source: u32, size: u32) {
    let writer = e.mem.u32(stream.addr() + STREAM_WRITER);
    stream_transfer(e, writer, IO_SLOT_WRITE, source, size);
}

/// The transfer function is a word of the binary stream object itself (00a71482: `MOV EAX,
/// [EBX+0x8]`, then `CALL EAX` with the object first, cdecl: `ADD ESP,0x14`), not a virtual slot:
/// a virtual call at +8 reached the stream's Seek (00aa1a90) with five words.
fn stream_transfer(e: &mut Engine, object: u32, slot: u32, buffer: u32, size: u32) {
    let transfer = e.mem.u32(object + slot);
    let transferred = e.with_stack(4, |e, size_variable| {
        e.mem.set_u32(size_variable.addr(), size);
        e.call(
            transfer,
            &args![object, buffer, size, size_variable.addr(), 1u32],
        )
        .u32()
    });
    let position = e.mem.u32(object + 4);
    e.mem
        .set_u32(object + 4, position.wrapping_add(transferred));
}

// Translated from 00a71450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::LoadBinary` (Xbox PDB): the `NiAVObject` load (`00a5a510`),
/// then through the stream's reader: from file version `10.0.1.2` a 16-bit
/// value that nothing uses (older files take the stream's own 16-bit value
/// at +0x284, also unused); six floats into the frustum's left, right, top,
/// bottom, near and far; from `10.0.1.5` the ortho byte (older files clear
/// it); five floats into the port's left, right, top, bottom and
/// `m_fLODAdjust`. Then it reads one link ID (`00a645b0`) and the multiple
/// link IDs (`00a64670`), again from file version `4.1.0.4`, and once more
/// for versions `10.0.1.7` to below `10.0.1.15`.
pub fn ni_camera_load_binary(e: &mut Engine, this: Ptr<NiCamera>, stream: Ptr) {
    e.call(AV_OBJECT_LOAD_BINARY, &args![this, stream]);
    let version = e.mem.u32(stream.addr() + STREAM_VERSION);
    if version < VERSION_A000102 {
        // The 16-bit value at +0x284 goes to a local that is never used.
        let _ = e.mem.u16(stream.addr() + STREAM_UNUSED_USHORT);
    } else {
        e.with_stack(4, |e, unused| stream_read(e, stream, unused.addr(), 2));
    }
    for offset in [0xdcu32, 0xe0, 0xe4, 0xe8, 0xec, 0xf0] {
        stream_read(e, stream, this.addr() + offset, 4);
    }
    if e.mem.u32(stream.addr() + STREAM_VERSION) < VERSION_A000105 {
        e.set(this, NiCamera::m_kViewFrustum_m_bOrtho, false);
    } else {
        let ortho = e.with_stack(4, |e, byte| {
            stream_read(e, stream, byte.addr(), 1);
            e.mem.u8(byte.addr())
        });
        e.set(this, NiCamera::m_kViewFrustum_m_bOrtho, ortho != 0);
    }
    for offset in [0x100u32, 0x104, 0x108, 0x10c, 0x110] {
        stream_read(e, stream, this.addr() + offset, 4);
    }
    e.call(STREAM_READ_LINK_ID, &args![stream]);
    e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
    let version = e.mem.u32(stream.addr() + STREAM_VERSION);
    if version >= VERSION_4010004 {
        e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
        let version = e.mem.u32(stream.addr() + STREAM_VERSION);
        if (VERSION_A000107..VERSION_A00010F).contains(&version) {
            e.call(STREAM_READ_MULTIPLE_LINK_IDS, &args![stream]);
        }
    }
}

// Translated from 00a716c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::SaveBinary` (Xbox PDB): the `NiAVObject` save (`00a5a900`),
/// then through the stream's writer: a 16-bit 0, the frustum's left, right,
/// top, bottom, near and far, the ortho byte, the port's left, right, top,
/// bottom and `m_fLODAdjust`. Then the stream's virtual method `+0x30` is
/// called with 0 (a null link ID), and two 32-bit zeros are written (empty
/// link lists).
pub fn ni_camera_save_binary(e: &mut Engine, this: Ptr<NiCamera>, stream: Ptr) {
    e.call(AV_OBJECT_SAVE_BINARY, &args![this, stream]);
    e.with_stack(4, |e, zero| stream_write(e, stream, zero.addr(), 2));
    for offset in [0xdcu32, 0xe0, 0xe4, 0xe8, 0xec, 0xf0] {
        stream_write(e, stream, this.addr() + offset, 4);
    }
    e.with_stack(4, |e, byte| {
        let ortho = e.mem.u8(this.addr() + 0xf4);
        e.mem.set_u8(byte.addr(), ortho);
        stream_write(e, stream, byte.addr(), 1);
    });
    for offset in [0x100u32, 0x104, 0x108, 0x10c, 0x110] {
        stream_write(e, stream, this.addr() + offset, 4);
    }
    e.vcall(stream.addr(), STREAM_SLOT_SAVE_LINK_ID, &args![0u32]);
    for _ in 0..2 {
        e.with_stack(4, |e, zero| stream_write(e, stream, zero.addr(), 4));
    }
}

/// Appends the string pointer `text` to the viewer-string array: when the
/// array's count (`m_usSize`) is not below its capacity (`m_usMaxSize`) it
/// first grows by `m_usGrowBy` (`0096ad30` to count + grow-by), then stores
/// the pointer at the old count (`0096ae90`).
fn add_viewer_string(e: &mut Engine, strings: Ptr, text: u32) {
    let array = strings.cast::<NiTArray>();
    let count = e.get(array, NiTArray::m_usSize) as u32;
    let capacity = e.get(array, NiTArray::m_usMaxSize) as u32;
    if count >= capacity {
        let grow_by = e.get(array, NiTArray::m_usGrowBy) as u32;
        e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
    }
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), text);
        e.call(ARRAY_SET_AT, &args![strings, count, value.addr()]);
    });
}

// Translated from 00a71920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiCamera::GetViewerStrings` (Xbox PDB): the `NiAVObject` strings
/// (`00a5ad00`), then eight more appended to `strings`: the class name
/// (`NiGetViewerString` of the first word of the `NiRTTI` at `011f4a18`),
/// `m_kViewFrustum`, `m_kPort`, the world rotation's columns 0, 1 and 2 as
/// `m_kWorldDir`, `m_kWorldUp` and `m_kWorldRight`, and the two floats
/// `m_fMinNearPlaneDist` and `m_fMaxFarNearRatio`.
pub fn ni_camera_get_viewer_strings(e: &mut Engine, this: Ptr<NiCamera>, strings: Ptr) {
    e.call(AV_OBJECT_GET_VIEWER_STRINGS, &args![this, strings]);
    let class_name = e.global::<u32>(NI_CAMERA_RTTI);
    let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
    add_viewer_string(e, strings, text);
    let text = e
        .call(
            FRUSTUM_GET_VIEWER_STRING,
            &args![this.addr() + 0xdc, NAME_VIEW_FRUSTUM],
        )
        .u32();
    add_viewer_string(e, strings, text);
    let text =
        ni_rect_float_get_viewer_string(e, Ptr::new(this.addr() + 0x100), Ptr::new(NAME_PORT))
            .addr();
    add_viewer_string(e, strings, text);
    for (column, name) in [
        (0u32, NAME_WORLD_DIR),
        (1, NAME_WORLD_UP),
        (2, NAME_WORLD_RIGHT),
    ] {
        let text = e.with_stack(0xc, |e, vector| {
            for row in 0..3u32 {
                let value = e
                    .mem
                    .u32(this.addr() + WORLD_ROTATE + 4 * (3 * row + column));
                e.mem.set_u32(vector.addr() + 4 * row, value);
            }
            e.call(POINT_GET_VIEWER_STRING, &args![vector.addr(), name])
                .u32()
        });
        add_viewer_string(e, strings, text);
    }
    let minimum = e.get(this, NiCamera::m_fMinNearPlaneDist);
    let text = e
        .call(
            GET_VIEWER_STRING_FLOAT,
            &args![NAME_MIN_NEAR_PLANE_DIST, minimum],
        )
        .u32();
    add_viewer_string(e, strings, text);
    let ratio = e.get(this, NiCamera::m_fMaxFarNearRatio);
    let text = e
        .call(
            GET_VIEWER_STRING_FLOAT,
            &args![NAME_MAX_FAR_NEAR_RATIO, ratio],
        )
        .u32();
    add_viewer_string(e, strings, text);
}

// Translated from 00a71430 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; by its body the creation function of `NiCamera`; no
/// arguments): a 0x114-byte block from `operator new`, constructed by [`fn_00a712f0`] (a
/// tail jump), or null when the allocation fails.
pub fn fn_00a71430(e: &mut Engine) -> Ptr<NiCamera> {
    let block = e.call(NI_ALLOCATE, &args![NI_CAMERA_SIZE]).u32();
    if block == 0 {
        return Ptr::NULL;
    }
    fn_00a712f0(e, Ptr::new(block))
}

#[cfg(test)]
mod b018_tests {
    use super::*;

    #[test]
    fn fn_00a71430_allocates_and_constructs_a_camera() {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_1430, &[]).u32(), 0);
        assert_eq!(
            e.call_log.take().unwrap(),
            vec![(0x00a7_1430, vec![]), (NI_ALLOCATE, vec![NI_CAMERA_SIZE])]
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a7_1430, fn_00a71430() -> Ptr<NiCamera>),
        entry!(0x00a6fa30, fn_00a6fa30(Ptr, u32, u32, Ptr, Ptr, Ptr) -> bool),
        entry!(0x00a6faf0, ni_camera_set_view_frustum(Ptr<NiCamera>, Ptr)),
        entry!(0x00a6fb90, fn_00a6fb90(Ptr<NiCamera>, f32, bool)),
        entry!(
            0x00a6fc50,
            ni_camera_world_pt_to_screen_pt(Ptr<NiCamera>, Ptr, Ptr, Ptr, f32) -> bool
        ),
        entry!(
            0x00a6fdb0,
            fn_00a6fdb0(Ptr<NiCamera>, Ptr, Ptr, Ptr, Ptr, f32) -> bool
        ),
        entry!(
            0x00a6ff10,
            ni_camera_register_streamables(Ptr<NiCamera>, Ptr) -> bool
        ),
        entry!(
            0x00a6ff20,
            ni_camera_is_equal(Ptr<NiCamera>, Ptr<NiCamera>) -> bool
        ),
        entry!(0x00a70040, ni_camera_get_rtti() -> Ptr),
        entry!(
            0x00a70050,
            ni_camera_scalar_deleting_destructor(Ptr<NiCamera>, u32) -> Ptr<NiCamera>
        ),
        entry!(0x00a70080, fn_00a70080(Ptr<NiCamera>, Ptr) -> bool),
        entry!(
            0x00a701b0,
            ni_camera_look_at_world_point(Ptr<NiCamera>, Ptr, Ptr) -> bool
        ),
        entry!(0x00a703a0, fn_00a703a0(Ptr<NiCamera>, f32, f32, Ptr, Ptr)),
        entry!(
            0x00a705f0,
            ni_camera_screen_space_bound_bound(Ptr<NiCamera>, Ptr, Ptr, Ptr, f32)
        ),
        entry!(0x00a70b40, ni_camera_update_world_bound(Ptr<NiCamera>)),
        entry!(0x00a70ba0, ni_camera_world_to_camera_matrix(Ptr<NiCamera>)),
        entry!(0x00a70f60, fn_00a70f60(Ptr<NiCamera>, Ptr<NiCamera>, u32)),
        entry!(0x00a71020, ni_rect_float_get_viewer_string(Ptr, Ptr) -> Ptr),
        entry!(
            0x00a71080,
            fn_00a71080(Ptr<NiCamera>, u32, u32, Ptr, Ptr, Ptr) -> bool
        ),
        entry!(0x00a711b0, ni_camera_update_world_data(Ptr<NiCamera>, Ptr)),
        entry!(0x00a711d0, ni_camera_link_object(Ptr<NiCamera>, Ptr)),
        entry!(0x00a712f0, fn_00a712f0(Ptr<NiCamera>) -> Ptr<NiCamera>),
        entry!(
            0x00a713e0,
            ni_camera_create_clone(Ptr<NiCamera>, u32) -> Ptr<NiCamera>
        ),
        entry!(0x00a71450, ni_camera_load_binary(Ptr<NiCamera>, Ptr)),
        entry!(0x00a716c0, ni_camera_save_binary(Ptr<NiCamera>, Ptr)),
        entry!(0x00a71920, ni_camera_get_viewer_strings(Ptr<NiCamera>, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    fn eax(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// The double at `01017cf8` that `Unitize` (`004a0c27`) and `UnitCross`
    /// (`0053d227`) compare a length with: the float `1e-6` widened (bytes
    /// `00 00 00 a0 f7 c6 b0 3e`). The test doubles of those two read it.
    const MIN_UNIT_LENGTH: u32 = 0x0101_7cf8;

    /// An engine with the exe constants the code reads mapped and set.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
            0x0101_e000,
            0x0102_2000,
            0x0109_9000,
            0x0109_c000,
            0x011f_4000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(HALF, 0.5f64);
        e.set_global(TWO, 2.0f64);
        e.set_global(MINUS_ONE, -1.0f32);
        e.set_global(MIN_AXIS_LENGTH_SQUARED, 0.5f32);
        // The exe's bytes at 0109cc78 are 00 00 00 e0 df 7c db 3d: the float
        // 1e-10 widened, not the double 1e-10.
        e.set_global(MIN_DISTANCE_SQUARED, f64::from(1e-10f32));
        e.set_global(MIN_UNIT_LENGTH, f64::from(1e-6f32));
        e.set_global(UNSIGNED_BIAS, 4294967296.0f32);
        e.set_global(DEFAULT_FAR, 2.0f32);
        e.set_global(DEFAULT_NEGATIVE_HALF_EXTENT, -0.5f32);
        e.set_global(DEFAULT_MIN_NEAR, 0.1f32);
        e.set_global(DEFAULT_MAX_FAR_NEAR_RATIO, 10000.0f32);
        e
    }

    fn camera(e: &mut Engine) -> Ptr<NiCamera> {
        e.new_object::<NiCamera>()
    }

    fn put_f32s(e: &mut Engine, addr: u32, values: &[f32]) {
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(addr + 4 * i as u32, *value);
        }
    }

    fn get_f32s(e: &Engine, addr: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(addr + 4 * i)).collect()
    }

    fn set_rotation(e: &mut Engine, cam: Ptr<NiCamera>, rows: [[f32; 3]; 3]) {
        for (i, row) in rows.iter().enumerate() {
            put_f32s(e, cam.addr() + WORLD_ROTATE + 12 * i as u32, row);
        }
    }

    fn identity(e: &mut Engine, cam: Ptr<NiCamera>) {
        set_rotation(e, cam, [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]]);
    }

    fn set_translation(e: &mut Engine, cam: Ptr<NiCamera>, translation: [f32; 3]) {
        put_f32s(e, cam.addr() + WORLD_TRANSLATE, &translation);
    }

    fn set_matrix(e: &mut Engine, cam: Ptr<NiCamera>, rows: [[f32; 4]; 4]) {
        for (i, row) in rows.iter().enumerate() {
            put_f32s(e, cam.addr() + WORLD_TO_CAM + 16 * i as u32, row);
        }
    }

    fn matrix(e: &Engine, cam: Ptr<NiCamera>) -> [[f32; 4]; 4] {
        let mut rows = [[0.0; 4]; 4];
        for (i, row) in rows.iter_mut().enumerate() {
            row.copy_from_slice(&get_f32s(e, cam.addr() + WORLD_TO_CAM + 16 * i as u32, 4));
        }
        rows
    }

    /// Frustum (left, right, top, bottom, near, far) and port (left, right,
    /// top, bottom) of a camera.
    fn set_view(e: &mut Engine, cam: Ptr<NiCamera>, frustum: [f32; 6], port: [f32; 4]) {
        put_f32s(e, cam.addr() + 0xdc, &frustum[..4]);
        put_f32s(e, cam.addr() + 0xec, &frustum[4..]);
        put_f32s(e, cam.addr() + 0x100, &port);
    }

    /// A vtable at `addr` with `(byte offset, target)` entries.
    fn vtable(e: &mut Engine, addr: u32, entries: &[(u32, u32)]) {
        let last = entries.iter().map(|(slot, _)| *slot).max().unwrap_or(0);
        let mut words = vec![0u32; (last / 4 + 1) as usize];
        for (slot, target) in entries {
            words[(*slot / 4) as usize] = *target;
        }
        e.put_vtable(addr, &words);
    }

    /// A camera whose virtual `UpdateWorldData` / `UpdateWorldBound` slots
    /// are the two recording doubles.
    const UPDATE_DATA_DOUBLE: u32 = 0x0070_0100;
    const UPDATE_BOUND_DOUBLE: u32 = 0x0070_0110;
    const CAMERA_TABLE: u32 = 0x0300_0000;

    fn camera_with_vtable(e: &mut Engine) -> Ptr<NiCamera> {
        e.register(UPDATE_DATA_DOUBLE, |_, _| Ret::default());
        e.register(UPDATE_BOUND_DOUBLE, |_, _| Ret::default());
        vtable(
            e,
            CAMERA_TABLE,
            &[
                (SLOT_UPDATE_WORLD_DATA, UPDATE_DATA_DOUBLE),
                (SLOT_UPDATE_WORLD_BOUND, UPDATE_BOUND_DOUBLE),
            ],
        );
        let cam = camera(e);
        e.mem.set_u32(cam.addr(), CAMERA_TABLE);
        cam
    }

    fn out_pair(e: &mut Engine, value: f32) -> (Ptr, Ptr) {
        let block = e.mem.alloc(8);
        put_f32s(e, block, &[value, value]);
        (Ptr::new(block), Ptr::new(block + 4))
    }

    // ---- fn_00a6fa30

    const GROUP: u32 = 0x0300_0100;
    const GROUP_TABLE: u32 = 0x0300_0200;
    const RENDERER_TABLE: u32 = 0x0300_0300;
    const WIDTH_DOUBLE: u32 = 0x0070_0200;
    const HEIGHT_DOUBLE: u32 = 0x0070_0210;
    const ZERO_WIDTH_DOUBLE: u32 = 0x0070_0220;
    const FIND_GROUP_DOUBLE: u32 = 0x0070_0230;

    fn renderer_with_group(e: &mut Engine, width_double: u32) -> Ptr {
        e.register(WIDTH_DOUBLE, |_, _| eax(800));
        e.register(ZERO_WIDTH_DOUBLE, |_, _| eax(0));
        e.register(HEIGHT_DOUBLE, |_, _| eax(600));
        e.register(FIND_GROUP_DOUBLE, |_, _| eax(GROUP));
        e.map(GROUP, 0x1000);
        e.map(GROUP_TABLE, 0x1000);
        vtable(
            e,
            GROUP_TABLE,
            &[
                (GROUP_SLOT_8C, width_double),
                (GROUP_SLOT_90, HEIGHT_DOUBLE),
            ],
        );
        e.mem.set_u32(GROUP, GROUP_TABLE);
        vtable(e, RENDERER_TABLE, &[(RENDERER_SLOT_C8, FIND_GROUP_DOUBLE)]);
        let renderer = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(renderer.addr(), RENDERER_TABLE);
        renderer
    }

    #[test]
    fn pixel_position_is_divided_by_the_group_size() {
        let mut e = engine();
        let renderer = renderer_with_group(&mut e, WIDTH_DOUBLE);
        let (out_x, out_y) = out_pair(&mut e, 7.0);
        e.call_log = Some(vec![]);
        let result = e.call(
            0x00a6_fa30,
            &args![renderer, 400u32, 150u32, out_x, out_y, 0u32],
        );
        assert!(result.bool());
        assert_eq!(e.mem.f32(out_x.addr()), 0.5);
        assert_eq!(e.mem.f32(out_y.addr()), 0.75);
        let log = e.call_log.take().unwrap();
        // The null group asks the object (+0xC8), then width (+0x8C), height
        // (+0x90), each with 0.
        assert_eq!(log[1], (FIND_GROUP_DOUBLE, vec![renderer.addr()]));
        assert_eq!(log[2], (WIDTH_DOUBLE, vec![GROUP, 0]));
        assert_eq!(log[3], (HEIGHT_DOUBLE, vec![GROUP, 0]));
    }

    #[test]
    fn a_given_group_is_used_without_asking_the_object() {
        let mut e = engine();
        renderer_with_group(&mut e, WIDTH_DOUBLE);
        let (out_x, out_y) = out_pair(&mut e, 7.0);
        e.call_log = Some(vec![]);
        let result = e.call(
            0x00a6_fa30,
            &args![
                Ptr::<()>::NULL,
                800u32,
                0u32,
                out_x,
                out_y,
                Ptr::<()>::new(GROUP)
            ],
        );
        assert!(result.bool());
        assert_eq!(e.mem.f32(out_x.addr()), 1.0);
        assert_eq!(e.mem.f32(out_y.addr()), 1.0);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(address, _)| *address != FIND_GROUP_DOUBLE));
    }

    #[test]
    fn a_zero_size_returns_false_and_writes_nothing() {
        let mut e = engine();
        let renderer = renderer_with_group(&mut e, ZERO_WIDTH_DOUBLE);
        let (out_x, out_y) = out_pair(&mut e, 7.0);
        let result = e.call(
            0x00a6_fa30,
            &args![renderer, 400u32, 150u32, out_x, out_y, 0u32],
        );
        assert!(!result.bool());
        assert_eq!(get_f32s(&e, out_x.addr(), 2), vec![7.0, 7.0]);
    }

    #[test]
    fn pixel_coordinates_are_unsigned() {
        let mut e = engine();
        let renderer = renderer_with_group(&mut e, WIDTH_DOUBLE);
        let (out_x, out_y) = out_pair(&mut e, 0.0);
        e.call(
            0x00a6_fa30,
            &args![renderer, 0xffff_ffffu32, 0u32, out_x, out_y, 0u32],
        );
        assert_eq!(e.mem.f32(out_x.addr()), (4294967295.0f64 / 800.0) as f32);
    }

    // ---- SetViewFrustum

    fn frustum_source(e: &mut Engine, values: [f32; 6], ortho: u8) -> Ptr {
        let block = e.mem.alloc(0x1c);
        put_f32s(e, block, &values);
        e.mem.set_u8(block + 0x18, ortho);
        Ptr::new(block)
    }

    #[test]
    fn the_near_plane_is_raised_to_far_over_the_ratio() {
        let mut e = engine();
        let cam = camera(&mut e);
        e.set(cam, NiCamera::m_fMaxFarNearRatio, 4.0);
        e.set(cam, NiCamera::m_fMinNearPlaneDist, 0.1);
        let source = frustum_source(&mut e, [-1.0, 1.0, 0.75, -0.75, 0.5, 100.0], 1);
        e.call(0x00a6_faf0, &args![cam, source]);
        assert_eq!(
            get_f32s(&e, cam.addr() + 0xdc, 6),
            vec![-1.0, 1.0, 0.75, -0.75, 25.0, 100.0]
        );
        assert_eq!(e.mem.u8(cam.addr() + 0xf4), 1);
    }

    #[test]
    fn the_near_plane_is_raised_to_the_minimum() {
        let mut e = engine();
        let cam = camera(&mut e);
        e.set(cam, NiCamera::m_fMaxFarNearRatio, 4.0);
        e.set(cam, NiCamera::m_fMinNearPlaneDist, 3.0);
        // far / ratio = 0.5 is not larger than the near plane 0.5.
        let source = frustum_source(&mut e, [-1.0, 1.0, 1.0, -1.0, 0.5, 2.0], 0);
        e.call(0x00a6_faf0, &args![cam, source]);
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fNear), 3.0);
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fFar), 2.0);
        assert!(!e.get(cam, NiCamera::m_kViewFrustum_m_bOrtho));
    }

    #[test]
    fn a_larger_near_plane_is_kept() {
        let mut e = engine();
        let cam = camera(&mut e);
        e.set(cam, NiCamera::m_fMaxFarNearRatio, 4.0);
        e.set(cam, NiCamera::m_fMinNearPlaneDist, 0.1);
        let source = frustum_source(&mut e, [-1.0, 1.0, 1.0, -1.0, 30.0, 100.0], 0);
        e.call(0x00a6_faf0, &args![cam, source]);
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fNear), 30.0);
    }

    // ---- fn_00a6fb90

    #[test]
    fn the_vertical_extent_is_fitted_to_the_width_over_the_ratio() {
        let mut e = engine();
        let cam = camera(&mut e);
        set_view(&mut e, cam, [-1.0, 1.0, 1.0, 0.0, 1.0, 2.0], [0.0; 4]);
        e.call(0x00a6_fb90, &args![cam, 4.0f32, true]);
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fTop), 0.75);
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fBottom), 0.25);
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fLeft), -1.0);
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fRight), 1.0);
    }

    #[test]
    fn the_horizontal_extent_is_fitted_to_the_height_times_the_ratio() {
        let mut e = engine();
        let cam = camera(&mut e);
        set_view(&mut e, cam, [1.0, 3.0, 2.0, 0.0, 1.0, 2.0], [0.0; 4]);
        e.call(0x00a6_fb90, &args![cam, 3.0f32, false]);
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fRight), 5.0);
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fLeft), -1.0);
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fTop), 2.0);
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fBottom), 0.0);
    }

    // ---- WorldPtToScreenPt and fn_00a6fdb0

    fn projecting_camera(e: &mut Engine, w_row: [f32; 4]) -> Ptr<NiCamera> {
        let cam = camera(e);
        set_matrix(
            e,
            cam,
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                w_row,
            ],
        );
        set_view(e, cam, [-1.0; 6], [0.0, 1.0, 1.0, 0.0]);
        cam
    }

    fn world_point(e: &mut Engine, values: [f32; 3]) -> Ptr {
        let block = e.mem.alloc(0xc);
        put_f32s(e, block, &values);
        Ptr::new(block)
    }

    #[test]
    fn a_visible_point_is_mapped_into_the_port() {
        let mut e = engine();
        let cam = projecting_camera(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let world = world_point(&mut e, [0.5, 0.5, 2.0]);
        let (out_x, out_y) = out_pair(&mut e, 9.0);
        let result = e.call(0x00a6_fc50, &args![cam, world, out_x, out_y, 0.001f32]);
        assert!(result.bool());
        // w = 2, so x' = y' = 0.25; the port (0, 1, 1, 0) maps to 0.625.
        assert_eq!(get_f32s(&e, out_x.addr(), 2), vec![0.625, 0.625]);
    }

    #[test]
    fn a_point_behind_the_camera_writes_nothing() {
        let mut e = engine();
        let cam = projecting_camera(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let world = world_point(&mut e, [0.5, 0.5, 0.0]);
        let (out_x, out_y) = out_pair(&mut e, 9.0);
        let result = e.call(0x00a6_fc50, &args![cam, world, out_x, out_y, 0.001f32]);
        assert!(!result.bool());
        assert_eq!(get_f32s(&e, out_x.addr(), 2), vec![9.0, 9.0]);
    }

    #[test]
    fn a_point_outside_the_view_returns_false_with_the_normalised_values() {
        let mut e = engine();
        let cam = projecting_camera(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let world = world_point(&mut e, [4.0, 0.0, 2.0]);
        let (out_x, out_y) = out_pair(&mut e, 9.0);
        let result = e.call(0x00a6_fc50, &args![cam, world, out_x, out_y, 0.001f32]);
        assert!(!result.bool());
        assert_eq!(get_f32s(&e, out_x.addr(), 2), vec![2.0, 0.0]);
    }

    #[test]
    fn a_point_below_the_view_is_rejected_on_y() {
        let mut e = engine();
        let cam = projecting_camera(&mut e, [0.0, 0.0, 1.0, 0.0]);
        let world = world_point(&mut e, [0.0, -6.0, 2.0]);
        let (out_x, out_y) = out_pair(&mut e, 9.0);
        let result = e.call(0x00a6_fc50, &args![cam, world, out_x, out_y, 0.001f32]);
        assert!(!result.bool());
        assert_eq!(get_f32s(&e, out_x.addr(), 2), vec![0.0, -3.0]);
    }

    #[test]
    fn the_depth_variant_uses_the_magnitude_of_w() {
        let mut e = engine();
        let cam = projecting_camera(&mut e, [0.0, 0.0, -1.0, 0.0]);
        let world = world_point(&mut e, [0.5, 0.5, 2.0]);
        let (out_x, out_y) = out_pair(&mut e, 9.0);
        let depth = Ptr::<()>::new(e.mem.alloc(4));
        let result = e.call(
            0x00a6_fdb0,
            &args![cam, world, out_x, out_y, depth, 0.001f32],
        );
        assert!(result.bool());
        // w = -2, |w| = 2: x' = y' = 0.25 mapped to 0.625; the depth row
        // gives 2, times 1 / |w|.
        assert_eq!(get_f32s(&e, out_x.addr(), 2), vec![0.625, 0.625]);
        assert_eq!(e.mem.f32(depth.addr()), 1.0);
    }

    #[test]
    fn the_depth_variant_needs_the_magnitude_above_the_minimum() {
        let mut e = engine();
        let cam = projecting_camera(&mut e, [0.0, 0.0, -1.0, 0.0]);
        let world = world_point(&mut e, [0.5, 0.5, 2.0]);
        let (out_x, out_y) = out_pair(&mut e, 9.0);
        let depth = Ptr::<()>::new(e.mem.alloc(4));
        e.mem.set_f32(depth.addr(), 9.0);
        let result = e.call(0x00a6_fdb0, &args![cam, world, out_x, out_y, depth, 2.0f32]);
        assert!(!result.bool());
        assert_eq!(get_f32s(&e, out_x.addr(), 2), vec![9.0, 9.0]);
        assert_eq!(e.mem.f32(depth.addr()), 9.0);
    }

    // ---- RegisterStreamables, IsEqual, GetRTTI, destructor

    #[test]
    fn register_streamables_forwards_to_the_base() {
        let mut e = engine();
        e.register(AV_OBJECT_REGISTER_STREAMABLES, |_, _| eax(1));
        let cam = camera(&mut e);
        e.call_log = Some(vec![]);
        let result = e.call(0x00a6_ff10, &args![cam, 0x1234u32]);
        assert!(result.bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (AV_OBJECT_REGISTER_STREAMABLES, vec![cam.addr(), 0x1234])
        );
        e.register(AV_OBJECT_REGISTER_STREAMABLES, |_, _| eax(0));
        assert!(!e.call(0x00a6_ff10, &args![cam, 0x1234u32]).bool());
    }

    fn equal_cameras(e: &mut Engine) -> (Ptr<NiCamera>, Ptr<NiCamera>) {
        e.register(AV_OBJECT_IS_EQUAL, |_, _| eax(1));
        let a = camera(e);
        let b = camera(e);
        for cam in [a, b] {
            set_view(
                e,
                cam,
                [-1.0, 1.0, 0.5, -0.5, 1.0, 10.0],
                [0.0, 1.0, 1.0, 0.0],
            );
            e.set(cam, NiCamera::m_fLODAdjust, 1.0);
        }
        (a, b)
    }

    #[test]
    fn identical_cameras_are_equal() {
        let mut e = engine();
        let (a, b) = equal_cameras(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a6_ff20, &args![a, b]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (AV_OBJECT_IS_EQUAL, vec![a.addr(), b.addr()]));
    }

    #[test]
    fn a_failing_base_comparison_decides() {
        let mut e = engine();
        let (a, b) = equal_cameras(&mut e);
        e.register(AV_OBJECT_IS_EQUAL, |_, _| eax(0));
        assert!(!e.call(0x00a6_ff20, &args![a, b]).bool());
    }

    #[test]
    fn each_compared_field_decides() {
        for offset in [
            0xdcu32, 0xe0, 0xe4, 0xe8, 0xec, 0xf0, 0x100, 0x104, 0x108, 0x10c, 0x110,
        ] {
            let mut e = engine();
            let (a, b) = equal_cameras(&mut e);
            e.mem.set_f32(b.addr() + offset, 123.0);
            assert!(!e.call(0x00a6_ff20, &args![a, b]).bool(), "{offset:#x}");
        }
    }

    #[test]
    fn the_ortho_flag_and_the_near_plane_limits_are_not_compared() {
        let mut e = engine();
        let (a, b) = equal_cameras(&mut e);
        e.mem.set_u8(b.addr() + 0xf4, 1);
        e.mem.set_f32(b.addr() + 0xf8, 5.0);
        e.mem.set_f32(b.addr() + 0xfc, 5.0);
        assert!(e.call(0x00a6_ff20, &args![a, b]).bool());
    }

    #[test]
    fn a_nan_is_not_equal_to_itself() {
        let mut e = engine();
        let (a, b) = equal_cameras(&mut e);
        e.mem.set_f32(a.addr() + 0xdc, f32::NAN);
        e.mem.set_f32(b.addr() + 0xdc, f32::NAN);
        assert!(!e.call(0x00a6_ff20, &args![a, b]).bool());
    }

    #[test]
    fn get_rtti_is_the_class_rtti_address() {
        let mut e = engine();
        assert_eq!(e.call(0x00a7_0040, &args![]).u32(), 0x011f_4a18);
    }

    #[test]
    fn the_destructor_frees_only_when_bit_zero_is_set() {
        let mut e = engine();
        e.register(AV_OBJECT_DESTRUCT, |_, _| Ret::default());
        e.register(NI_FREE, |_, _| Ret::default());
        let cam = camera(&mut e);
        e.call_log = Some(vec![]);
        let result = e.call(0x00a7_0050, &args![cam, 0u32]);
        assert_eq!(result.u32(), cam.addr());
        assert_eq!(e.mem.u32(cam.addr()), 0x0109_cb9c);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 2);
        assert_eq!(log[1], (AV_OBJECT_DESTRUCT, vec![cam.addr()]));
        e.call_log = Some(vec![]);
        e.call(0x00a7_0050, &args![cam, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[2], (NI_FREE, vec![cam.addr(), 0x114]));
    }

    // ---- fn_00a70080

    fn sphere(e: &mut Engine, values: [f32; 4]) -> Ptr {
        let block = e.mem.alloc(0x10);
        put_f32s(e, block, &values);
        Ptr::new(block)
    }

    /// The camera at (1, 2, 3) looking along world x with `ratio` 10 and
    /// `minimum` 0.1, near 7 and far 9 beforehand.
    fn fitting_camera(e: &mut Engine) -> Ptr<NiCamera> {
        let cam = camera_with_vtable(e);
        set_rotation(e, cam, [[1.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]]);
        set_translation(e, cam, [1.0, 2.0, 3.0]);
        e.set(cam, NiCamera::m_fMaxFarNearRatio, 10.0);
        e.set(cam, NiCamera::m_fMinNearPlaneDist, 0.1);
        e.set(cam, NiCamera::m_kViewFrustum_m_fNear, 7.0);
        e.set(cam, NiCamera::m_kViewFrustum_m_fFar, 9.0);
        cam
    }

    #[test]
    fn the_planes_are_fitted_to_the_sphere() {
        let mut e = engine();
        let cam = fitting_camera(&mut e);
        let ball = sphere(&mut e, [11.0, 2.0, 3.0, 2.0]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a7_0080, &args![cam, ball]).bool());
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fNear), 8.0);
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fFar), 12.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 3);
        assert_eq!(log[1].0, UPDATE_DATA_DOUBLE);
        assert_eq!(log[1].1[0], cam.addr());
        assert_eq!(log[2], (UPDATE_BOUND_DOUBLE, vec![cam.addr()]));
    }

    #[test]
    fn the_update_data_is_cleared() {
        let mut e = engine();
        let cam = fitting_camera(&mut e);
        e.map(0x0050_0000, 0x1000);
        e.register(UPDATE_DATA_DOUBLE, |e, a| {
            let bytes = e.mem.bytes(a[1], 0xc);
            e.mem.write(0x0050_0000, &bytes);
            Ret::default()
        });
        let ball = sphere(&mut e, [11.0, 2.0, 3.0, 2.0]);
        e.call(0x00a7_0080, &args![cam, ball]);
        assert_eq!(e.mem.bytes(0x0050_0000, 9), vec![0; 9]);
    }

    #[test]
    fn a_sphere_behind_the_camera_changes_nothing() {
        let mut e = engine();
        let cam = fitting_camera(&mut e);
        let ball = sphere(&mut e, [-5.0, 2.0, 3.0, 2.0]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a7_0080, &args![cam, ball]).bool());
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fNear), 7.0);
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fFar), 9.0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn the_near_plane_is_at_least_far_over_the_ratio() {
        let mut e = engine();
        let cam = fitting_camera(&mut e);
        let ball = sphere(&mut e, [2.0, 2.0, 3.0, 1.0]);
        assert!(e.call(0x00a7_0080, &args![cam, ball]).bool());
        // d = 1, so near would be 0 and far 2; 2 / 10 raises the near plane.
        assert_eq!(
            e.get(cam, NiCamera::m_kViewFrustum_m_fNear),
            (2.0f64 / 10.0) as f32
        );
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fFar), 2.0);
    }

    #[test]
    fn the_near_plane_is_at_least_the_minimum() {
        let mut e = engine();
        let cam = fitting_camera(&mut e);
        e.set(cam, NiCamera::m_fMinNearPlaneDist, 0.5);
        let ball = sphere(&mut e, [2.0, 2.0, 3.0, 1.0]);
        assert!(e.call(0x00a7_0080, &args![cam, ball]).bool());
        assert_eq!(e.get(cam, NiCamera::m_kViewFrustum_m_fNear), 0.5);
    }

    #[test]
    fn a_nan_radius_still_fits_the_planes() {
        let mut e = engine();
        let cam = fitting_camera(&mut e);
        let ball = sphere(&mut e, [11.0, 2.0, 3.0, f32::NAN]);
        e.call_log = Some(vec![]);
        // 00a70109: an unordered FCOM against 0 takes the JP and goes on.
        assert!(e.call(0x00a7_0080, &args![cam, ball]).bool());
        assert!(e.get(cam, NiCamera::m_kViewFrustum_m_fNear).is_nan());
        assert!(e.get(cam, NiCamera::m_kViewFrustum_m_fFar).is_nan());
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 3);
        assert_eq!(log[1].0, UPDATE_DATA_DOUBLE);
        assert_eq!(log[2], (UPDATE_BOUND_DOUBLE, vec![cam.addr()]));
    }

    // ---- LookAtWorldPoint

    fn read3(e: &Engine, addr: u32) -> [f32; 3] {
        [e.mem.f32(addr), e.mem.f32(addr + 4), e.mem.f32(addr + 8)]
    }

    /// The length as `00457990` computes it: `FLD` each float, `FMUL` the
    /// same float in memory, `(x * x + y * y) + z * z` stored to a float, and
    /// the CRT `sqrt` (`00ec6054`, reached through `004579e0` and
    /// `004019d0`) of that, stored to a float.
    fn length3(vector: [f32; 3]) -> f32 {
        let square = |v: f32| x87_mul(x87_load32(v.to_bits()), x87_m32(v.to_bits()));
        let sum = x87_add(
            x87_add(square(vector[0]), square(vector[1])),
            square(vector[2]),
        );
        f64::from(f32::from_bits(x87_store32(sum))).sqrt() as f32
    }

    /// `Unitize` (`004a0c10`): when the length is ordered above `minimum`
    /// (`004a0c27` FCOMP, `004a0c2f` TEST AH,0x41, `004a0c32` JNZ to the
    /// zero path), each float times `1 / length` (`FLD1`, `FDIVRP`, stored to
    /// a float; then `FLD` the float, `FMUL` that); otherwise, a NaN length
    /// included, the zero vector (`004a0c6b`).
    fn unitize3(vector: [f32; 3], minimum: f64) -> [f32; 3] {
        let length = length3(vector);
        if !(length as f64 > minimum) {
            return [0.0; 3];
        }
        let inverse = x87_store32(x87_div(1.0, x87_load32(length.to_bits())));
        vector.map(|v| {
            f32::from_bits(x87_store32(x87_mul(
                x87_load32(v.to_bits()),
                x87_m32(inverse),
            )))
        })
    }

    /// `UnitCross` (`0053d1a0`): the cross product `a x b` (each component
    /// `FLD`, `FMUL`, `FLD`, `FMUL`, `FSUBP`, stored to a float); when its
    /// length is ordered above `minimum` (`0053d227` FCOMP, `0053d22f` TEST
    /// AH,0x41, `0053d232` JNZ to the zero path), `0053d280` scales it by
    /// `1 / length` (the reciprocal stored to a float, then `FLD` it, `FMUL`
    /// the component); otherwise, a NaN length included, the zero vector
    /// (`0053d24e`).
    fn unit_cross3(a: [f32; 3], b: [f32; 3], minimum: f64) -> [f32; 3] {
        let product =
            |i: usize, j: usize| x87_mul(x87_load32(a[i].to_bits()), x87_m32(b[j].to_bits()));
        let term = |i: usize, j: usize, k: usize, l: usize| {
            f32::from_bits(x87_store32(x87_sub(product(i, j), product(k, l))))
        };
        let cross = [term(1, 2, 2, 1), term(2, 0, 0, 2), term(0, 1, 1, 0)];
        let length = length3(cross);
        if !(length as f64 > minimum) {
            return [0.0; 3];
        }
        let inverse = x87_store32(x87_div(1.0, x87_load32(length.to_bits())));
        cross.map(|v| {
            f32::from_bits(x87_store32(x87_mul(
                x87_load32(inverse),
                x87_m32(v.to_bits()),
            )))
        })
    }

    /// Doubles for `Unitize` and `UnitCross` that do what the exe's do
    /// (`unitize3`, `unit_cross3`), with the minimum length read from
    /// `01017cf8`.
    fn vector_doubles(e: &mut Engine) {
        e.register(POINT_NORMALIZE, |e, a| {
            let unit = unitize3(read3(e, a[0]), e.global::<f64>(MIN_UNIT_LENGTH));
            put_f32s(e, a[0], &unit);
            Ret::default()
        });
        e.register(UNIT_CROSS, |e, a| {
            let minimum = e.global::<f64>(MIN_UNIT_LENGTH);
            let unit = unit_cross3(read3(e, a[0]), read3(e, a[2]), minimum);
            put_f32s(e, a[1], &unit);
            eax(a[1])
        });
    }

    fn rotation_of(e: &Engine, cam: Ptr<NiCamera>) -> Vec<f32> {
        get_f32s(e, cam.addr() + LOCAL_ROTATE, 9)
    }

    #[test]
    fn looking_down_the_axis_builds_the_expected_rotation() {
        let mut e = engine();
        vector_doubles(&mut e);
        let cam = camera_with_vtable(&mut e);
        set_translation(&mut e, cam, [0.0, 0.0, 10.0]);
        let target = world_point(&mut e, [0.0, 0.0, 0.0]);
        let up = world_point(&mut e, [0.0, 1.0, 0.0]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a7_01b0, &args![cam, target, up]).bool());
        // d = (0, 0, 1); up x d = (1, 0, 0); d x that = (0, 1, 0).
        assert_eq!(
            rotation_of(&e, cam),
            vec![0.0, 0.0, 1.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.0]
        );
        let log = e.call_log.take().unwrap();
        let calls: Vec<u32> = log.iter().map(|(address, _)| *address).collect();
        assert_eq!(
            calls,
            vec![
                0x00a7_01b0,
                POINT_NORMALIZE,
                UNIT_CROSS,
                UNIT_CROSS,
                UPDATE_DATA_DOUBLE,
                UPDATE_BOUND_DOUBLE
            ]
        );
        // UnitCross(up, result, d) then UnitCross(d, result, first).
        assert_eq!(log[2].1[0], up.addr());
        assert_eq!(log[3].1[0], log[1].1[0]);
        assert_eq!(log[3].1[2], log[2].1[1]);
    }

    #[test]
    fn a_parent_premultiplies_with_the_transposed_world_rotation() {
        let mut e = engine();
        vector_doubles(&mut e);
        e.register(MATRIX_TRANSPOSE_TIMES, |e, a| {
            put_f32s(e, a[1], &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
            eax(a[1])
        });
        let cam = camera_with_vtable(&mut e);
        let parent = e.mem.alloc(0xa0);
        e.mem.set_u32(cam.addr() + 0x18, parent);
        set_translation(&mut e, cam, [0.0, 0.0, 10.0]);
        let target = world_point(&mut e, [0.0, 0.0, 0.0]);
        let up = world_point(&mut e, [0.0, 1.0, 0.0]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00a7_01b0, &args![cam, target, up]).bool());
        assert_eq!(
            rotation_of(&e, cam),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
        );
        let log = e.call_log.take().unwrap();
        let (_, args) = log
            .iter()
            .find(|(address, _)| *address == MATRIX_TRANSPOSE_TIMES)
            .unwrap();
        assert_eq!(args[0], parent + 0x68);
    }

    #[test]
    fn a_target_at_the_camera_fails() {
        let mut e = engine();
        vector_doubles(&mut e);
        let cam = camera_with_vtable(&mut e);
        set_translation(&mut e, cam, [1.0, 2.0, 3.0]);
        put_f32s(&mut e, cam.addr() + LOCAL_ROTATE, &[5.0; 9]);
        let target = world_point(&mut e, [1.0, 2.0, 3.0]);
        let up = world_point(&mut e, [0.0, 1.0, 0.0]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a7_01b0, &args![cam, target, up]).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(rotation_of(&e, cam), vec![5.0; 9]);
    }

    #[test]
    fn an_up_vector_along_the_view_fails_after_the_first_cross() {
        let mut e = engine();
        vector_doubles(&mut e);
        let cam = camera_with_vtable(&mut e);
        set_translation(&mut e, cam, [0.0, 0.0, 10.0]);
        put_f32s(&mut e, cam.addr() + LOCAL_ROTATE, &[5.0; 9]);
        let target = world_point(&mut e, [0.0, 0.0, 0.0]);
        let up = world_point(&mut e, [0.0, 0.0, 1.0]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00a7_01b0, &args![cam, target, up]).bool());
        let calls: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .map(|(address, _)| *address)
            .collect();
        assert_eq!(calls, vec![0x00a7_01b0, POINT_NORMALIZE, UNIT_CROSS]);
        assert_eq!(rotation_of(&e, cam), vec![5.0; 9]);
    }

    #[test]
    fn a_nan_target_passes_the_distance_test_then_fails_the_first_cross() {
        let mut e = engine();
        vector_doubles(&mut e);
        let cam = camera_with_vtable(&mut e);
        set_translation(&mut e, cam, [0.0, 0.0, 10.0]);
        put_f32s(&mut e, cam.addr() + LOCAL_ROTATE, &[5.0; 9]);
        let target = world_point(&mut e, [f32::NAN, 0.0, 0.0]);
        let up = world_point(&mut e, [0.0, 1.0, 0.0]);
        e.call_log = Some(vec![]);
        // d = (NaN, 0, 10): its squared length is NaN, an unordered FCOMP
        // (0x5, even parity) that 00a7020f does not fail on. Unitize's NaN
        // length is not above 1e-6 (004a0c32), so d becomes zero; up x 0 is
        // zero (0053d232), and its squared length 0 is an ordered less-than
        // 0.5 (0x1, odd parity) that fails at 00a70263.
        assert!(!e.call(0x00a7_01b0, &args![cam, target, up]).bool());
        let calls: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .map(|(address, _)| *address)
            .collect();
        assert_eq!(calls, vec![0x00a7_01b0, POINT_NORMALIZE, UNIT_CROSS]);
        assert_eq!(rotation_of(&e, cam), vec![5.0; 9]);
    }

    #[test]
    fn a_nan_first_cross_passes_its_test_then_the_second_cross_fails() {
        let mut e = engine();
        vector_doubles(&mut e);
        let cam = camera_with_vtable(&mut e);
        set_translation(&mut e, cam, [1.0, 1.0, 1.0]);
        put_f32s(&mut e, cam.addr() + LOCAL_ROTATE, &[5.0; 9]);
        let target = world_point(&mut e, [0.0, 0.0, 0.0]);
        let up = world_point(&mut e, [f32::INFINITY, 0.0, 0.0]);
        e.call_log = Some(vec![]);
        // d = (1, 1, 1) unitizes to (a, a, a). up x d = (0, -inf, inf) has
        // the length inf, ordered above 1e-6, so UnitCross scales it by
        // 1 / inf = 0: (0, NaN, NaN). That NaN squared length passes
        // 00a70263; d x (0, NaN, NaN) is all NaN, which UnitCross makes zero
        // (0053d232), and its squared length 0 fails at 00a702b1.
        assert!(!e.call(0x00a7_01b0, &args![cam, target, up]).bool());
        let calls: Vec<u32> = e
            .call_log
            .take()
            .unwrap()
            .iter()
            .map(|(address, _)| *address)
            .collect();
        assert_eq!(
            calls,
            vec![0x00a7_01b0, POINT_NORMALIZE, UNIT_CROSS, UNIT_CROSS]
        );
        assert_eq!(rotation_of(&e, cam), vec![5.0; 9]);
    }

    // ---- fn_00a703a0

    fn ray_camera(e: &mut Engine, ortho: bool) -> Ptr<NiCamera> {
        e.register(POINT_NORMALIZE, |_, _| Ret::default());
        let cam = camera(e);
        identity(e, cam);
        set_translation(e, cam, [1.0, 2.0, 3.0]);
        e.set(cam, NiCamera::m_kViewFrustum_m_bOrtho, ortho);
        cam
    }

    #[test]
    fn a_perspective_ray_starts_at_the_camera() {
        let mut e = engine();
        let cam = ray_camera(&mut e, false);
        let (origin, direction) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        e.call_log = Some(vec![]);
        e.call(0x00a7_03a0, &args![cam, 2.0f32, 3.0f32, origin, direction]);
        assert_eq!(read3(&e, origin), [1.0, 2.0, 3.0]);
        // column 0 + 2 * column 2 + 3 * column 1
        assert_eq!(read3(&e, direction), [1.0, 3.0, 2.0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (POINT_NORMALIZE, vec![direction]));
    }

    #[test]
    fn an_orthographic_ray_moves_the_origin() {
        let mut e = engine();
        let cam = ray_camera(&mut e, true);
        let (origin, direction) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        e.call_log = Some(vec![]);
        e.call(0x00a7_03a0, &args![cam, 2.0f32, 3.0f32, origin, direction]);
        assert_eq!(read3(&e, direction), [1.0, 0.0, 0.0]);
        // translation + 2 * column 2 + 3 * column 1
        assert_eq!(read3(&e, origin), [1.0, 5.0, 5.0]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (POINT_NORMALIZE, vec![direction]));
    }

    // ---- ScreenSpaceBoundBound

    /// A camera at the origin whose world rotation's column 0 is (0, 0, 1),
    /// with the x and y rows picking x and y and the w row picking z.
    fn bound_camera(e: &mut Engine) -> Ptr<NiCamera> {
        let cam = camera(e);
        set_rotation(e, cam, [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
        set_matrix(
            e,
            cam,
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0; 4],
                [0.0, 0.0, 1.0, 0.0],
            ],
        );
        set_view(e, cam, [-1.0, 1.0, 1.0, -1.0, 1.0, 11.0], [0.0; 4]);
        cam
    }

    type CrossLog = Rc<RefCell<Vec<([f32; 3], [f32; 3])>>>;

    /// A `UnitCross` double that writes (1, 0, 0) the first time and
    /// (0, 1, 0) the second, logging the vectors it was given.
    fn scripted_crosses(e: &mut Engine) -> CrossLog {
        let log: CrossLog = Rc::new(RefCell::new(vec![]));
        let recorded = log.clone();
        e.register_double(UNIT_CROSS, move |e, a| {
            let call = recorded.borrow().len();
            recorded.borrow_mut().push((read3(e, a[0]), read3(e, a[2])));
            let result = if call == 0 {
                [1.0, 0.0, 0.0]
            } else {
                [0.0, 1.0, 0.0]
            };
            put_f32s(e, a[1], &result);
            eax(a[1])
        });
        log
    }

    fn run_bound(e: &mut Engine, cam: Ptr<NiCamera>, min_w: f32) -> ([f32; 3], [f32; 3]) {
        let ball = sphere(e, [0.0, 0.0, 5.0, 1.0]);
        let (min, max) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        e.call(0x00a7_05f0, &args![cam, ball, min, max, min_w]);
        (read3(e, min), read3(e, max))
    }

    #[test]
    fn the_sphere_projects_to_a_box() {
        let mut e = engine();
        let cam = bound_camera(&mut e);
        let log = scripted_crosses(&mut e);
        let (min, max) = run_bound(&mut e, cam, 0.001);
        let inverse_depth = (1.0f64 / 10.0) as f32;
        let fifth = (1.0f64 / 5.0) as f32;
        assert_eq!(min[0], -fifth);
        assert_eq!(max[0], fifth);
        assert_eq!(min[1], -fifth);
        assert_eq!(max[1], fifth);
        // depth along column 0 is 5 - near 1 = 4; 4 -/+ radius 1.
        assert_eq!(min[2], (inverse_depth as f64 * 3.0) as f32);
        assert_eq!(max[2], (inverse_depth as f64 * 5.0) as f32);
        // d x column 1, then column 2 x d, with d = (0, 0, 5).
        let log = log.borrow();
        assert_eq!(log.len(), 2);
        assert_eq!(log[0], ([0.0, 0.0, 5.0], [0.0, 0.0, 0.0]));
        assert_eq!(log[1], ([0.0; 3], [0.0, 0.0, 5.0]));
    }

    #[test]
    fn the_cross_products_use_columns_one_and_two() {
        let mut e = engine();
        let cam = bound_camera(&mut e);
        set_rotation(
            &mut e,
            cam,
            [[0.0, 0.25, 0.5], [0.0, 0.75, 1.5], [1.0, 2.25, 3.5]],
        );
        let log = scripted_crosses(&mut e);
        run_bound(&mut e, cam, 0.001);
        let log = log.borrow();
        assert_eq!(log[0].1, [0.25, 0.75, 2.25]);
        assert_eq!(log[1].0, [0.5, 1.5, 3.5]);
    }

    #[test]
    fn a_box_beyond_the_view_is_clamped() {
        let mut e = engine();
        let cam = bound_camera(&mut e);
        e.mem.set_f32(cam.addr() + WORLD_TO_CAM, 10.0);
        scripted_crosses(&mut e);
        let (min, max) = run_bound(&mut e, cam, 0.001);
        assert_eq!((min[0], max[0]), (-1.0, 1.0));
        let fifth = (1.0f64 / 5.0) as f32;
        assert_eq!((min[1], max[1]), (-fifth, fifth));
    }

    #[test]
    fn a_box_to_the_right_of_the_view_collapses_to_the_edge() {
        let mut e = engine();
        let cam = bound_camera(&mut e);
        // x rows: 20 * (-1, 1) / 5 = (-4, 4) shifted by the offset 15 / 5.
        e.mem.set_f32(cam.addr() + WORLD_TO_CAM, 20.0);
        e.mem.set_f32(cam.addr() + WORLD_TO_CAM + 12, 15.0);
        scripted_crosses(&mut e);
        let (min, max) = run_bound(&mut e, cam, 0.001);
        // min.x = (-20 + 15) / 5 = -1, max.x = (20 + 15) / 5 = 7 -> clamped.
        assert_eq!((min[0], max[0]), (-1.0, 1.0));
        e.mem.set_f32(cam.addr() + WORLD_TO_CAM + 12, 30.0);
        scripted_crosses(&mut e);
        let (min, max) = run_bound(&mut e, cam, 0.001);
        // (10, 50) / 5 = (2, 10): both above 1.
        assert_eq!((min[0], max[0]), (1.0, 1.0));
    }

    #[test]
    fn a_box_to_the_left_of_the_view_collapses_to_minus_one() {
        let mut e = engine();
        let cam = bound_camera(&mut e);
        e.mem.set_f32(cam.addr() + WORLD_TO_CAM, 20.0);
        e.mem.set_f32(cam.addr() + WORLD_TO_CAM + 12, -30.0);
        scripted_crosses(&mut e);
        let (min, max) = run_bound(&mut e, cam, 0.001);
        // (-50, -10) / 5 = (-10, -2)
        assert_eq!((min[0], max[0]), (-1.0, -1.0));
    }

    #[test]
    fn a_point_behind_the_camera_gives_the_whole_view() {
        let mut e = engine();
        let cam = bound_camera(&mut e);
        scripted_crosses(&mut e);
        // w = 5 is not above min_w = 5.
        let (min, max) = run_bound(&mut e, cam, 5.0);
        assert_eq!((min[0], min[1], max[0], max[1]), (-1.0, -1.0, 1.0, 1.0));
        let inverse_depth = (1.0f64 / 10.0) as f32;
        assert_eq!(min[2], (inverse_depth as f64 * 3.0) as f32);
    }

    #[test]
    fn a_failure_on_the_second_pair_also_gives_the_whole_view() {
        let mut e = engine();
        let cam = bound_camera(&mut e);
        // Make the w row depend on y: w = z - 5 * y.
        e.mem.set_f32(cam.addr() + WORLD_TO_CAM + 0xd0 - 0x9c, -5.0);
        scripted_crosses(&mut e);
        let (min, max) = run_bound(&mut e, cam, 0.001);
        // First pair (x -/+ 1): y = 0, so w = 5. Second pair (y -/+ 1): the
        // low point has w = 10, the high point w = 5 - 5 = 0 <= 0.001.
        assert_eq!((min[0], min[1], max[0], max[1]), (-1.0, -1.0, 1.0, 1.0));
    }

    #[test]
    fn the_depth_range_is_clamped_to_zero_and_one() {
        let mut e = engine();
        let cam = bound_camera(&mut e);
        set_view(&mut e, cam, [-1.0, 1.0, 1.0, -1.0, 4.5, 14.5], [0.0; 4]);
        scripted_crosses(&mut e);
        let (min, max) = run_bound(&mut e, cam, 0.001);
        // depth 0.5: low -0.5 -> negative -> 0; high 1.5 / 10 = 0.15.
        assert_eq!(min[2], 0.0);
        assert_eq!(max[2], (0.1f32 as f64 * 1.5) as f32);
        set_view(&mut e, cam, [-1.0, 1.0, 1.0, -1.0, 1.0, 1.2], [0.0; 4]);
        scripted_crosses(&mut e);
        let (min, max) = run_bound(&mut e, cam, 0.001);
        // 1 / 0.2 = 5: low 3 * 5 = 15 -> 1, high 5 * 5 = 25 -> 1.
        assert_eq!((min[2], max[2]), (1.0, 1.0));
        set_view(&mut e, cam, [-1.0, 1.0, 1.0, -1.0, 6.0, 16.0], [0.0; 4]);
        scripted_crosses(&mut e);
        let (min, max) = run_bound(&mut e, cam, 0.001);
        // depth -1: low -2 is negative -> 0, high 0 stays 0.
        assert_eq!((min[2], max[2]), (0.0, 0.0));
        set_view(&mut e, cam, [-1.0, 1.0, 1.0, -1.0, 8.0, 18.0], [0.0; 4]);
        scripted_crosses(&mut e);
        let (min, max) = run_bound(&mut e, cam, 0.001);
        // depth -3: low and high are both negative -> 0.
        assert_eq!((min[2], max[2]), (0.0, 0.0));
        set_view(&mut e, cam, [-1.0, 1.0, 1.0, -1.0, 4.5, 4.6], [0.0; 4]);
        scripted_crosses(&mut e);
        let (min, max) = run_bound(&mut e, cam, 0.001);
        // inverse depth about 10: low -0.5 -> negative -> 0, high 1.5 -> 15 -> 1.
        assert_eq!((min[2], max[2]), (0.0, 1.0));
    }

    #[test]
    fn an_orthographic_box_has_no_depth_range() {
        let mut e = engine();
        let cam = bound_camera(&mut e);
        e.set(cam, NiCamera::m_kViewFrustum_m_bOrtho, true);
        scripted_crosses(&mut e);
        let (min, max) = run_bound(&mut e, cam, 0.001);
        assert_eq!((min[2], max[2]), (0.0, 0.0));
        let fifth = (1.0f64 / 5.0) as f32;
        assert_eq!((min[0], max[0]), (-fifth, fifth));
    }

    // ---- UpdateWorldBound

    const BOUND_BLOCK: u32 = 0x0300_1000;

    #[test]
    fn a_missing_bound_is_allocated_and_centred() {
        let mut e = engine();
        e.map(BOUND_BLOCK, 0x1000);
        e.register(BOUND_ALLOCATE, |e, a| {
            assert_eq!(a, &[BOUND_POOL, 0x10]);
            put_f32s(e, BOUND_BLOCK, &[5.0, 5.0, 5.0, 5.0]);
            eax(BOUND_BLOCK)
        });
        e.register(0x0070_0300, |e, a| {
            e.mem.set_u32(a[0] + WORLD_BOUND, a[1]);
            Ret::default()
        });
        let cam = camera(&mut e);
        vtable(&mut e, CAMERA_TABLE, &[(SLOT_SET_BOUND, 0x0070_0300)]);
        e.mem.set_u32(cam.addr(), CAMERA_TABLE);
        set_translation(&mut e, cam, [1.0, 2.0, 3.0]);
        e.call(0x00a7_0b40, &args![cam]);
        assert_eq!(e.mem.u32(cam.addr() + WORLD_BOUND), BOUND_BLOCK);
        assert_eq!(get_f32s(&e, BOUND_BLOCK, 4), vec![1.0, 2.0, 3.0, 0.0]);
    }

    #[test]
    fn an_existing_bound_only_moves_its_centre() {
        let mut e = engine();
        let cam = camera(&mut e);
        let bound = e.mem.alloc(0x10);
        put_f32s(&mut e, bound, &[9.0, 9.0, 9.0, 4.0]);
        e.mem.set_u32(cam.addr() + WORLD_BOUND, bound);
        set_translation(&mut e, cam, [1.0, 2.0, 3.0]);
        e.call(0x00a7_0b40, &args![cam]);
        assert_eq!(get_f32s(&e, bound, 4), vec![1.0, 2.0, 3.0, 4.0]);
    }

    // ---- WorldToCameraMatrix

    fn symmetric_view(e: &mut Engine, cam: Ptr<NiCamera>) {
        set_view(e, cam, [-1.0, 1.0, 1.0, -1.0, 1.0, 3.0], [0.0; 4]);
    }

    fn off_centre_view(e: &mut Engine, cam: Ptr<NiCamera>) {
        set_view(e, cam, [-1.0, 3.0, 2.0, 0.0, 1.0, 5.0], [0.0; 4]);
    }

    #[test]
    fn a_symmetric_perspective_camera() {
        let mut e = engine();
        let cam = camera(&mut e);
        identity(&mut e, cam);
        symmetric_view(&mut e, cam);
        e.call(0x00a7_0ba0, &args![cam]);
        assert_eq!(
            matrix(&e, cam),
            [
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [1.5, 0.0, 0.0, -1.5],
                [1.0, 0.0, 0.0, 0.0],
            ]
        );
    }

    #[test]
    fn an_off_centre_perspective_camera_with_a_translation() {
        let mut e = engine();
        let cam = camera(&mut e);
        identity(&mut e, cam);
        set_translation(&mut e, cam, [1.0, 2.0, 3.0]);
        off_centre_view(&mut e, cam);
        e.call(0x00a7_0ba0, &args![cam]);
        assert_eq!(
            matrix(&e, cam),
            [
                [-0.5, 0.0, 0.5, -1.0],
                [-1.0, 1.0, 0.0, -1.0],
                [1.25, 0.0, 0.0, -2.5],
                [1.0, 0.0, 0.0, -1.0],
            ]
        );
    }

    #[test]
    fn a_rotated_perspective_camera_uses_the_columns_of_the_rotation() {
        let mut e = engine();
        let cam = camera(&mut e);
        set_rotation(
            &mut e,
            cam,
            [[0.0, 0.0, 1.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]],
        );
        symmetric_view(&mut e, cam);
        e.call(0x00a7_0ba0, &args![cam]);
        assert_eq!(
            matrix(&e, cam),
            [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 1.5, 0.0, -1.5],
                [0.0, 1.0, 0.0, 0.0],
            ]
        );
    }

    #[test]
    fn an_orthographic_camera() {
        let mut e = engine();
        let cam = camera(&mut e);
        identity(&mut e, cam);
        set_translation(&mut e, cam, [1.0, 2.0, 3.0]);
        off_centre_view(&mut e, cam);
        e.set(cam, NiCamera::m_kViewFrustum_m_bOrtho, true);
        e.call(0x00a7_0ba0, &args![cam]);
        assert_eq!(
            matrix(&e, cam),
            [
                [0.0, 0.0, 0.5, -1.0],
                [0.0, 1.0, 0.0, -1.0],
                [0.25, 0.0, 0.0, -0.5],
                [0.0, 0.0, 0.0, 1.0],
            ]
        );
    }

    #[test]
    fn update_world_data_runs_the_base_then_rebuilds_the_matrix() {
        let mut e = engine();
        e.register(AV_OBJECT_UPDATE_WORLD_DATA, |_, _| Ret::default());
        let cam = camera(&mut e);
        identity(&mut e, cam);
        symmetric_view(&mut e, cam);
        e.call_log = Some(vec![]);
        e.call(0x00a7_11b0, &args![cam, 0x77u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (AV_OBJECT_UPDATE_WORLD_DATA, vec![cam.addr(), 0x77])
        );
        assert_eq!(matrix(&e, cam)[3], [1.0, 0.0, 0.0, 0.0]);
    }

    // ---- fn_00a70f60

    #[test]
    fn the_copy_takes_the_camera_state() {
        let mut e = engine();
        e.register(COPY_MEMBERS, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let source = camera(&mut e);
        let target = camera(&mut e);
        let numbers: Vec<f32> = (1..=16).map(|i| i as f32).collect();
        put_f32s(&mut e, source.addr() + WORLD_TO_CAM, &numbers);
        set_translation(&mut e, source, [7.0, 8.0, 9.0]);
        set_view(
            &mut e,
            source,
            [-1.0, 2.0, 3.0, -4.0, 5.0, 6.0],
            [10.0, 11.0, 12.0, 13.0],
        );
        e.set(source, NiCamera::m_kViewFrustum_m_bOrtho, true);
        e.set(source, NiCamera::m_fLODAdjust, 14.0);
        e.set(source, NiCamera::m_fMinNearPlaneDist, 15.0);
        e.set(source, NiCamera::m_fMaxFarNearRatio, 16.0);
        e.call_log = Some(vec![]);
        e.call(0x00a7_0f60, &args![source, target, 0x99u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (COPY_MEMBERS, vec![source.addr(), target.addr(), 0x99])
        );
        assert_eq!(
            log[2],
            (
                MEMCPY,
                vec![
                    target.addr() + WORLD_TO_CAM,
                    source.addr() + WORLD_TO_CAM,
                    0x40
                ]
            )
        );
        assert_eq!(get_f32s(&e, target.addr() + WORLD_TO_CAM, 16), numbers);
        assert_eq!(read3(&e, target.addr() + WORLD_TRANSLATE), [7.0, 8.0, 9.0]);
        assert_eq!(
            get_f32s(&e, target.addr() + 0xdc, 6),
            vec![-1.0, 2.0, 3.0, -4.0, 5.0, 6.0]
        );
        assert_eq!(e.mem.u8(target.addr() + 0xf4), 1);
        assert_eq!(get_f32s(&e, target.addr() + 0xf8, 2), vec![15.0, 16.0]);
        assert_eq!(
            get_f32s(&e, target.addr() + 0x100, 5),
            vec![10.0, 11.0, 12.0, 13.0, 14.0]
        );
    }

    // ---- NiRect<float>::GetViewerString

    #[test]
    fn the_rectangle_string_is_formatted_into_a_buffer() {
        let mut e = engine();
        e.register(NI_ALLOCATE_BUFFER, |e, a| eax(e.mem.alloc(a[0])));
        e.register(NI_SPRINTF, |_, _| Ret::default());
        let rectangle = e.mem.alloc(0x10);
        put_f32s(&mut e, rectangle, &[1.5, 2.5, 3.5, 4.5]);
        let name = e.mem.alloc(8);
        e.mem.set_cstr(name, b"m_kPort");
        e.call_log = Some(vec![]);
        let buffer = e.call(0x00a7_1020, &args![rectangle, name]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NI_ALLOCATE_BUFFER, vec![7 + 0x40]));
        assert_eq!(
            log[2],
            (
                NI_SPRINTF,
                args![
                    buffer,
                    7u32 + 0x40,
                    RECT_FORMAT,
                    name,
                    1.5f64,
                    2.5f64,
                    3.5f64,
                    4.5f64
                ]
                .to_vec()
            )
        );
    }

    // ---- fn_00a71080

    /// The renderer object at `011f4748` with an 800 x 600 group, and a
    /// camera whose port is the unit square and whose frustum spans x -2..2
    /// and y -1..1, at (1, 2, 3) with the identity rotation.
    fn pick_setup(e: &mut Engine) -> Ptr<NiCamera> {
        let renderer = renderer_with_group(e, WIDTH_DOUBLE);
        e.set_global(RENDERER_OBJECT, renderer.addr());
        let cam = ray_camera(e, false);
        set_view(
            e,
            cam,
            [-2.0, 2.0, 1.0, -1.0, 1.0, 2.0],
            [0.0, 1.0, 1.0, 0.0],
        );
        cam
    }

    #[test]
    fn a_pixel_becomes_a_ray() {
        let mut e = engine();
        let cam = pick_setup(&mut e);
        let (origin, direction) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        let result = e.call(
            0x00a7_1080,
            &args![cam, 400u32, 150u32, origin, direction, 0u32],
        );
        assert!(result.bool());
        // (0.5, 0.75) in the port -> (0, 0.5) in the frustum; the ray is
        // column 0 + 0 * column 2 + 0.5 * column 1.
        assert_eq!(read3(&e, origin), [1.0, 2.0, 3.0]);
        assert_eq!(read3(&e, direction), [1.0, 0.5, 0.0]);
    }

    #[test]
    fn no_renderer_gives_no_ray() {
        let mut e = engine();
        let cam = pick_setup(&mut e);
        e.set_global(RENDERER_OBJECT, 0u32);
        let (origin, direction) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        let result = e.call(
            0x00a7_1080,
            &args![cam, 400u32, 150u32, origin, direction, 0u32],
        );
        assert!(!result.bool());
        assert_eq!(read3(&e, origin), [0.0; 3]);
    }

    #[test]
    fn a_pixel_outside_the_port_gives_no_ray() {
        let mut e = engine();
        let cam = pick_setup(&mut e);
        let (origin, direction) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        // The port's right edge is 0.25, the pixel maps to x = 0.5.
        e.set(cam, NiCamera::m_kPort_m_right, 0.25);
        assert!(!e
            .call(
                0x00a7_1080,
                &args![cam, 400u32, 150u32, origin, direction, 0u32]
            )
            .bool());
        // Port left edge above x.
        e.set(cam, NiCamera::m_kPort_m_right, 1.0);
        e.set(cam, NiCamera::m_kPort_m_left, 0.75);
        assert!(!e
            .call(
                0x00a7_1080,
                &args![cam, 400u32, 150u32, origin, direction, 0u32]
            )
            .bool());
        // Port top edge below y = 0.75.
        e.set(cam, NiCamera::m_kPort_m_left, 0.0);
        e.set(cam, NiCamera::m_kPort_m_top, 0.5);
        assert!(!e
            .call(
                0x00a7_1080,
                &args![cam, 400u32, 150u32, origin, direction, 0u32]
            )
            .bool());
        // Port bottom edge above y.
        e.set(cam, NiCamera::m_kPort_m_top, 1.0);
        e.set(cam, NiCamera::m_kPort_m_bottom, 0.8);
        assert!(!e
            .call(
                0x00a7_1080,
                &args![cam, 400u32, 150u32, origin, direction, 0u32]
            )
            .bool());
        assert_eq!(read3(&e, origin), [0.0; 3]);
    }

    #[test]
    fn a_nan_port_edge_still_gives_a_ray() {
        let mut e = engine();
        let cam = pick_setup(&mut e);
        // 00a710c8: right against x unordered (0x5, even parity) passes.
        e.set(cam, NiCamera::m_kPort_m_right, f32::NAN);
        let (origin, direction) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        assert!(e
            .call(
                0x00a7_1080,
                &args![cam, 400u32, 150u32, origin, direction, 0u32]
            )
            .bool());
        assert_eq!(read3(&e, origin), [1.0, 2.0, 3.0]);
        assert!(read3(&e, direction)[0].is_nan());
        // 00a710f2: top against y unordered passes the same way.
        e.set(cam, NiCamera::m_kPort_m_right, 1.0);
        e.set(cam, NiCamera::m_kPort_m_top, f32::NAN);
        let (origin, direction) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        assert!(e
            .call(
                0x00a7_1080,
                &args![cam, 400u32, 150u32, origin, direction, 0u32]
            )
            .bool());
        assert_eq!(read3(&e, origin), [1.0, 2.0, 3.0]);
        assert!(read3(&e, direction)[1].is_nan());
    }

    #[test]
    fn a_zero_sized_group_gives_no_ray() {
        let mut e = engine();
        let cam = pick_setup(&mut e);
        let renderer = renderer_with_group(&mut e, ZERO_WIDTH_DOUBLE);
        e.set_global(RENDERER_OBJECT, renderer.addr());
        let (origin, direction) = (e.mem.alloc(0xc), e.mem.alloc(0xc));
        assert!(!e
            .call(
                0x00a7_1080,
                &args![cam, 400u32, 150u32, origin, direction, 0u32]
            )
            .bool());
    }

    // ---- LinkObject

    const OBJECT_TABLE: u32 = 0x0300_2000;
    const DELETE_DOUBLE: u32 = 0x0070_0400;

    fn counted_object(e: &mut Engine, count: u32) -> u32 {
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, OBJECT_TABLE);
        e.mem.set_u32(object + 4, count);
        object
    }

    fn counter_of(e: &Engine, object: u32) -> u32 {
        e.mem.u32(object + 4)
    }

    /// The stream and the doubles `LinkObject` needs; `objects` and `counts`
    /// are what `GetObjectFromLinkID` and `GetNumberOfLinkIDs` return in
    /// turn.
    fn link_setup(e: &mut Engine, version: u32, objects: Vec<u32>, counts: Vec<u32>) -> Ptr {
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        vtable(e, OBJECT_TABLE, &[(4, DELETE_DOUBLE)]);
        e.register(AV_OBJECT_LINK_OBJECT, |_, _| Ret::default());
        e.register(INTERLOCKED_INCREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], value);
            eax(value)
        });
        e.register(INTERLOCKED_DECREMENT, |e, a| {
            let value = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], value);
            eax(value)
        });
        let objects = Rc::new(RefCell::new(std::collections::VecDeque::from(objects)));
        e.register_double(STREAM_GET_OBJECT_FROM_LINK_ID, move |_, _| {
            eax(objects.borrow_mut().pop_front().unwrap_or(0))
        });
        let counts = Rc::new(RefCell::new(std::collections::VecDeque::from(counts)));
        e.register_double(STREAM_GET_NUMBER_OF_LINK_IDS, move |_, _| {
            eax(counts.borrow_mut().pop_front().unwrap_or(0))
        });
        let stream = Ptr::new(e.mem.alloc(0x300));
        e.mem.set_u32(stream.addr() + STREAM_VERSION, version);
        stream
    }

    fn addresses(log: &[(u32, Vec<u32>)], wanted: &[u32]) -> Vec<u32> {
        log.iter()
            .map(|(address, _)| *address)
            .filter(|address| wanted.contains(address))
            .collect()
    }

    #[test]
    fn link_object_walks_all_three_lists_in_a_new_file() {
        let mut e = engine();
        let held = counted_object(&mut e, 1);
        let unreferenced = counted_object(&mut e, 0);
        let shared = counted_object(&mut e, 3);
        let second = counted_object(&mut e, 1);
        let stream = link_setup(
            &mut e,
            0x0a00_0108,
            vec![held, unreferenced, shared, second],
            vec![2, 1, 2],
        );
        let cam = camera(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a7_11d0, &args![cam, stream]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (AV_OBJECT_LINK_OBJECT, vec![cam.addr(), stream.addr()])
        );
        assert_eq!(
            addresses(
                &log,
                &[
                    STREAM_GET_OBJECT_FROM_LINK_ID,
                    STREAM_GET_NUMBER_OF_LINK_IDS
                ]
            ),
            vec![
                STREAM_GET_OBJECT_FROM_LINK_ID,
                STREAM_GET_NUMBER_OF_LINK_IDS,
                STREAM_GET_OBJECT_FROM_LINK_ID,
                STREAM_GET_OBJECT_FROM_LINK_ID,
                STREAM_GET_NUMBER_OF_LINK_IDS,
                STREAM_GET_OBJECT_FROM_LINK_ID,
                STREAM_GET_NUMBER_OF_LINK_IDS,
                STREAM_GET_OBJECT_FROM_LINK_ID,
                STREAM_GET_OBJECT_FROM_LINK_ID,
            ]
        );
        // Only the object nobody referenced is deleted.
        let deletes: Vec<_> = log
            .iter()
            .filter(|(address, _)| *address == DELETE_DOUBLE)
            .collect();
        assert_eq!(deletes.len(), 1);
        assert_eq!(deletes[0].1, vec![unreferenced]);
        assert_eq!(counter_of(&e, held), 1);
        assert_eq!(counter_of(&e, unreferenced), 0);
        assert_eq!(counter_of(&e, shared), 3);
        assert_eq!(counter_of(&e, second), 1);
        // The held object got a reference first and gave it back last.
        assert_eq!(
            log.iter()
                .filter(|(address, args)| *address == INTERLOCKED_INCREMENT && args[0] == held + 4)
                .count(),
            1
        );
    }

    #[test]
    fn an_old_file_has_one_list() {
        let mut e = engine();
        let held = counted_object(&mut e, 1);
        let other = counted_object(&mut e, 2);
        let stream = link_setup(&mut e, 0x0401_0003, vec![held, other], vec![1]);
        let cam = camera(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a7_11d0, &args![cam, stream]);
        let log = e.call_log.take().unwrap();
        assert_eq!(addresses(&log, &[STREAM_GET_NUMBER_OF_LINK_IDS]).len(), 1);
        assert_eq!(counter_of(&e, other), 2);
        assert_eq!(counter_of(&e, held), 1);
    }

    #[test]
    fn a_middle_file_has_two_lists() {
        let mut e = engine();
        let held = counted_object(&mut e, 1);
        let stream = link_setup(&mut e, 0x0a00_0105, vec![held], vec![0, 0, 5]);
        let cam = camera(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a7_11d0, &args![cam, stream]);
        let log = e.call_log.take().unwrap();
        assert_eq!(addresses(&log, &[STREAM_GET_NUMBER_OF_LINK_IDS]).len(), 2);
    }

    #[test]
    fn a_missing_first_link_takes_no_reference() {
        let mut e = engine();
        let stream = link_setup(&mut e, 0x0401_0003, vec![], vec![0]);
        let cam = camera(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a7_11d0, &args![cam, stream]);
        let log = e.call_log.take().unwrap();
        assert!(addresses(&log, &[INTERLOCKED_INCREMENT, INTERLOCKED_DECREMENT]).is_empty());
    }

    // ---- the constructor and CreateClone

    const SET_BOUND_DOUBLE: u32 = 0x0070_0300;

    fn construction_doubles(e: &mut Engine) {
        e.register(AV_OBJECT_CONSTRUCT, |e, a| {
            put_f32s(
                e,
                a[0] + WORLD_ROTATE,
                &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
            );
            Ret::default()
        });
        e.register(FRUSTUM_CONSTRUCT, |e, a| {
            e.mem.set_u8(a[0] + 0x18, a[1] as u8);
            Ret::default()
        });
        e.register(AV_OBJECT_UPDATE_WORLD_DATA, |_, _| Ret::default());
        e.register(BOUND_ALLOCATE, |e, _| eax(e.mem.alloc(0x10)));
        e.register(SET_BOUND_DOUBLE, |e, a| {
            e.mem.set_u32(a[0] + WORLD_BOUND, a[1]);
            Ret::default()
        });
        let mut slots = vec![0u32; (SLOT_SET_BOUND / 4 + 1) as usize];
        slots[(SLOT_SET_BOUND / 4) as usize] = SET_BOUND_DOUBLE;
        e.put_vtable(NI_CAMERA_VTABLE, &slots);
    }

    #[test]
    fn fn_00a71430_constructs_the_allocated_camera() {
        let mut e = engine();
        construction_doubles(&mut e);
        let cam = camera(&mut e);
        e.register_double(NI_ALLOCATE, move |_, _| eax(cam.addr()));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00a7_1430, &[]).u32(), cam.addr());
        assert_eq!(e.mem.u32(cam.addr()), NI_CAMERA_VTABLE);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (NI_ALLOCATE, vec![NI_CAMERA_SIZE]));
        assert!(log.contains(&(AV_OBJECT_CONSTRUCT, vec![cam.addr()])));
    }

    #[test]
    fn the_constructor_sets_the_defaults() {
        let mut e = engine();
        construction_doubles(&mut e);
        let cam = camera(&mut e);
        e.mem.set_u8(cam.addr() + 0xf4, 1);
        e.call_log = Some(vec![]);
        let result = e.call(0x00a7_12f0, &args![cam]);
        assert_eq!(result.u32(), cam.addr());
        assert_eq!(e.mem.u32(cam.addr()), NI_CAMERA_VTABLE);
        assert_eq!(
            get_f32s(&e, cam.addr() + 0xdc, 6),
            vec![-0.5, 0.5, 0.5, -0.5, 1.0, 2.0]
        );
        assert!(!e.get(cam, NiCamera::m_kViewFrustum_m_bOrtho));
        assert_eq!(e.get(cam, NiCamera::m_fMinNearPlaneDist), 0.1);
        assert_eq!(e.get(cam, NiCamera::m_fMaxFarNearRatio), 10000.0);
        assert_eq!(
            get_f32s(&e, cam.addr() + 0x100, 5),
            vec![0.0, 1.0, 1.0, 0.0, 1.0]
        );
        assert_eq!(
            matrix(&e, cam),
            [
                [0.0, 0.0, 2.0, 0.0],
                [0.0, 2.0, 0.0, 0.0],
                [2.0, 0.0, 0.0, -2.0],
                [1.0, 0.0, 0.0, 0.0],
            ]
        );
        let bound = e.mem.u32(cam.addr() + WORLD_BOUND);
        assert_eq!(get_f32s(&e, bound, 4), vec![0.0; 4]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter().map(|(address, _)| *address).collect::<Vec<_>>(),
            vec![
                0x00a7_12f0,
                AV_OBJECT_CONSTRUCT,
                FRUSTUM_CONSTRUCT,
                AV_OBJECT_UPDATE_WORLD_DATA,
                BOUND_ALLOCATE,
                SET_BOUND_DOUBLE
            ]
        );
        assert_eq!(log[2].1, vec![cam.addr() + 0xdc, 0]);
        assert_eq!(log[3].1[0], cam.addr());
    }

    #[test]
    fn a_clone_is_a_new_camera_with_the_copied_state() {
        let mut e = engine();
        construction_doubles(&mut e);
        e.register(NI_ALLOCATE, |e, a| {
            assert_eq!(a, &[0x114]);
            eax(e.mem.alloc(0x114))
        });
        e.register(COPY_MEMBERS, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        let source = camera(&mut e);
        let numbers: Vec<f32> = (1..=16).map(|i| i as f32).collect();
        put_f32s(&mut e, source.addr() + WORLD_TO_CAM, &numbers);
        set_view(
            &mut e,
            source,
            [-1.0, 2.0, 3.0, -4.0, 5.0, 6.0],
            [10.0, 11.0, 12.0, 13.0],
        );
        e.call_log = Some(vec![]);
        let clone = e.call(0x00a7_13e0, &args![source, 0x99u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_ne!(clone, 0);
        assert_ne!(clone, source.addr());
        assert_eq!(
            addresses(
                &log,
                &[NI_ALLOCATE, AV_OBJECT_CONSTRUCT, COPY_MEMBERS, MEMCPY]
            ),
            vec![NI_ALLOCATE, AV_OBJECT_CONSTRUCT, COPY_MEMBERS, MEMCPY]
        );
        assert!(log.contains(&(COPY_MEMBERS, vec![source.addr(), clone, 0x99])));
        assert_eq!(e.mem.u32(clone), NI_CAMERA_VTABLE);
        assert_eq!(get_f32s(&e, clone + WORLD_TO_CAM, 16), numbers);
        assert_eq!(
            get_f32s(&e, clone + 0xdc, 6),
            vec![-1.0, 2.0, 3.0, -4.0, 5.0, 6.0]
        );
    }

    type WriteLog = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;
    type VectorLog = Rc<RefCell<Vec<(u32, [f32; 3])>>>;

    // ---- LoadBinary and SaveBinary

    const READ_DOUBLE: u32 = 0x0070_0500;
    const WRITE_DOUBLE: u32 = 0x0070_0510;
    const STREAM_TABLE: u32 = 0x0300_3200;
    const SAVE_LINK_DOUBLE: u32 = 0x0070_0520;

    /// A stream with a reader whose read function writes the number of the
    /// call as a float (4 bytes), `0x1234` (2 bytes) or 1 (1 byte).
    fn load_stream(e: &mut Engine, version: u32) -> (Ptr, u32) {
        let counter = Rc::new(Cell::new(0u32));
        e.register_double(READ_DOUBLE, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2], "the size variable holds the size");
            assert_eq!(a[4], 1);
            let call = counter.get() + 1;
            counter.set(call);
            match a[2] {
                4 => e.mem.set_f32(a[1], call as f32),
                2 => e.mem.set_u16(a[1], 0x1234),
                1 => e.mem.set_u8(a[1], 1),
                other => panic!("size {other}"),
            }
            eax(a[2])
        });
        // The read function is the binary stream object's own word at +8 (00a71482).
        let reader = e.mem.alloc(0x10);
        e.mem.set_u32(reader + IO_SLOT_READ, READ_DOUBLE);
        e.register(AV_OBJECT_LOAD_BINARY, |_, _| Ret::default());
        e.register(STREAM_READ_LINK_ID, |_, _| Ret::default());
        e.register(STREAM_READ_MULTIPLE_LINK_IDS, |_, _| Ret::default());
        let stream = Ptr::new(e.mem.alloc(0x300));
        e.mem.set_u32(stream.addr() + STREAM_VERSION, version);
        e.mem.set_u32(stream.addr() + STREAM_READER, reader);
        e.mem.set_u16(stream.addr() + STREAM_UNUSED_USHORT, 0x4321);
        (stream, reader)
    }

    fn read_sizes(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter()
            .filter(|(address, _)| *address == READ_DOUBLE)
            .map(|(_, args)| args[2])
            .collect()
    }

    #[test]
    fn a_new_file_reads_every_field() {
        let mut e = engine();
        let (stream, reader) = load_stream(&mut e, 0x0a00_0108);
        let cam = camera(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00a7_1450, &args![cam, stream]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (AV_OBJECT_LOAD_BINARY, vec![cam.addr(), stream.addr()])
        );
        assert_eq!(
            read_sizes(&log),
            vec![2, 4, 4, 4, 4, 4, 4, 1, 4, 4, 4, 4, 4]
        );
        assert_eq!(
            get_f32s(&e, cam.addr() + 0xdc, 6),
            vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0]
        );
        assert!(e.get(cam, NiCamera::m_kViewFrustum_m_bOrtho));
        assert_eq!(
            get_f32s(&e, cam.addr() + 0x100, 5),
            vec![9.0, 10.0, 11.0, 12.0, 13.0]
        );
        assert_eq!(e.mem.u32(reader + 4), 2 + 24 + 1 + 20);
        assert_eq!(addresses(&log, &[STREAM_READ_LINK_ID]).len(), 1);
        assert_eq!(addresses(&log, &[STREAM_READ_MULTIPLE_LINK_IDS]).len(), 3);
    }

    #[test]
    fn an_old_file_skips_the_short_values() {
        let mut e = engine();
        let (stream, reader) = load_stream(&mut e, 0x0401_0003);
        let cam = camera(&mut e);
        e.set(cam, NiCamera::m_kViewFrustum_m_bOrtho, true);
        e.call_log = Some(vec![]);
        e.call(0x00a7_1450, &args![cam, stream]);
        let log = e.call_log.take().unwrap();
        assert_eq!(read_sizes(&log), vec![4; 11]);
        assert_eq!(
            get_f32s(&e, cam.addr() + 0xdc, 6),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]
        );
        assert!(!e.get(cam, NiCamera::m_kViewFrustum_m_bOrtho));
        assert_eq!(
            get_f32s(&e, cam.addr() + 0x100, 5),
            vec![7.0, 8.0, 9.0, 10.0, 11.0]
        );
        assert_eq!(e.mem.u32(reader + 4), 44);
        assert_eq!(addresses(&log, &[STREAM_READ_MULTIPLE_LINK_IDS]).len(), 1);
    }

    #[test]
    fn a_middle_file_reads_the_short_value_but_not_the_ortho_byte() {
        let mut e = engine();
        let (stream, _) = load_stream(&mut e, 0x0a00_0103);
        let cam = camera(&mut e);
        e.set(cam, NiCamera::m_kViewFrustum_m_bOrtho, true);
        e.call_log = Some(vec![]);
        e.call(0x00a7_1450, &args![cam, stream]);
        let log = e.call_log.take().unwrap();
        let mut sizes = vec![2];
        sizes.extend([4; 11]);
        assert_eq!(read_sizes(&log), sizes);
        assert!(!e.get(cam, NiCamera::m_kViewFrustum_m_bOrtho));
        assert_eq!(addresses(&log, &[STREAM_READ_MULTIPLE_LINK_IDS]).len(), 2);
    }

    /// A stream whose writer and own virtual method log what they get.
    fn save_stream(e: &mut Engine) -> (Ptr, u32, WriteLog) {
        let events: WriteLog = Rc::new(RefCell::new(vec![]));
        let recorded = events.clone();
        e.register_double(WRITE_DOUBLE, move |e, a| {
            assert_eq!(e.mem.u32(a[3]), a[2], "the size variable holds the size");
            assert_eq!(a[4], 1);
            recorded.borrow_mut().push((a[2], e.mem.bytes(a[1], a[2])));
            eax(a[2])
        });
        let recorded = events.clone();
        e.register_double(SAVE_LINK_DOUBLE, move |_, a| {
            assert_eq!(a[1], 0);
            recorded.borrow_mut().push((0xffff, vec![]));
            Ret::default()
        });
        // The write function is the binary stream object's own word at +0xc (00a716ef).
        vtable(
            e,
            STREAM_TABLE,
            &[(STREAM_SLOT_SAVE_LINK_ID, SAVE_LINK_DOUBLE)],
        );
        let writer = e.mem.alloc(0x10);
        e.mem.set_u32(writer + IO_SLOT_WRITE, WRITE_DOUBLE);
        e.register(AV_OBJECT_SAVE_BINARY, |_, _| Ret::default());
        let stream = Ptr::new(e.mem.alloc(0x300));
        e.mem.set_u32(stream.addr(), STREAM_TABLE);
        e.mem.set_u32(stream.addr() + STREAM_WRITER, writer);
        (stream, writer, events)
    }

    #[test]
    fn save_binary_writes_the_fields_in_order() {
        let mut e = engine();
        let (stream, writer, events) = save_stream(&mut e);
        let cam = camera(&mut e);
        set_view(
            &mut e,
            cam,
            [1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0, 10.0],
        );
        e.set(cam, NiCamera::m_kViewFrustum_m_bOrtho, true);
        e.set(cam, NiCamera::m_fLODAdjust, 11.0);
        e.call_log = Some(vec![]);
        e.call(0x00a7_16c0, &args![cam, stream]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (AV_OBJECT_SAVE_BINARY, vec![cam.addr(), stream.addr()])
        );
        let float = |value: f32| (4, value.to_le_bytes().to_vec());
        let mut expected = vec![(2, vec![0, 0])];
        expected.extend((1..=6).map(|i| float(i as f32)));
        expected.push((1, vec![1]));
        expected.extend((7..=11).map(|i| float(i as f32)));
        expected.push((0xffff, vec![]));
        expected.push((4, vec![0; 4]));
        expected.push((4, vec![0; 4]));
        assert_eq!(*events.borrow(), expected);
        assert_eq!(e.mem.u32(writer + 4), 2 + 24 + 1 + 20 + 8);
    }

    #[test]
    fn save_binary_writes_a_clear_ortho_byte() {
        let mut e = engine();
        let (stream, _, events) = save_stream(&mut e);
        let cam = camera(&mut e);
        e.call(0x00a7_16c0, &args![cam, stream]);
        assert_eq!(events.borrow()[7], (1, vec![0]));
    }

    // ---- GetViewerStrings

    #[test]
    fn the_viewer_strings_are_appended_in_order() {
        let mut e = engine();
        e.set_global(NI_CAMERA_RTTI, 0x0123_4567u32);
        e.mem.set_cstr(NAME_PORT, b"m_kPort");
        e.register(AV_OBJECT_GET_VIEWER_STRINGS, |_, _| Ret::default());
        e.register(GET_VIEWER_STRING, |_, _| eax(0xa001));
        e.register(FRUSTUM_GET_VIEWER_STRING, |_, _| eax(0xa002));
        e.register(NI_SPRINTF, |_, _| Ret::default());
        let buffer = Rc::new(Cell::new(0u32));
        let remembered = buffer.clone();
        e.register_double(NI_ALLOCATE_BUFFER, move |e, a| {
            let block = e.mem.alloc(a[0]);
            remembered.set(block);
            eax(block)
        });
        let vectors: VectorLog = Rc::new(RefCell::new(vec![]));
        let recorded = vectors.clone();
        e.register_double(POINT_GET_VIEWER_STRING, move |e, a| {
            let mut list = recorded.borrow_mut();
            list.push((a[1], read3(e, a[0])));
            eax(0xa004 + list.len() as u32 - 1)
        });
        e.register(GET_VIEWER_STRING_FLOAT, |_, a| {
            eax(if a[0] == NAME_MIN_NEAR_PLANE_DIST {
                0xa007
            } else {
                0xa008
            })
        });
        e.register(ARRAY_SET_SIZE, |e, a| {
            e.mem.set_u16(a[0] + 8, a[1] as u16);
            Ret::default()
        });
        e.register(ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = Ptr::<()>::new(e.mem.alloc(0x10));
        let base = e.mem.alloc(0x40);
        e.mem.set_u32(strings.addr() + 4, base);
        e.mem.set_u16(strings.addr() + 0xe, 3);
        let cam = camera(&mut e);
        set_rotation(
            &mut e,
            cam,
            [[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]],
        );
        e.set(cam, NiCamera::m_fMinNearPlaneDist, 0.25);
        e.set(cam, NiCamera::m_fMaxFarNearRatio, 500.0);
        e.call_log = Some(vec![]);
        e.call(0x00a7_1920, &args![cam, strings]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1],
            (
                AV_OBJECT_GET_VIEWER_STRINGS,
                vec![cam.addr(), strings.addr()]
            )
        );
        assert_eq!(log[2], (GET_VIEWER_STRING, vec![0x0123_4567]));
        assert!(log.contains(&(
            FRUSTUM_GET_VIEWER_STRING,
            vec![cam.addr() + 0xdc, NAME_VIEW_FRUSTUM]
        )));
        let rectangle = log
            .iter()
            .find(|(address, _)| *address == NI_SPRINTF)
            .unwrap();
        assert_eq!(
            rectangle.1[..4],
            [buffer.get(), 0x47, RECT_FORMAT, NAME_PORT]
        );
        assert_eq!(e.mem.u16(strings.addr() + 0xa), 8);
        let stored: Vec<u32> = (0..8).map(|i| e.mem.u32(base + 4 * i)).collect();
        assert_eq!(
            stored,
            vec![
                0xa001,
                0xa002,
                buffer.get(),
                0xa004,
                0xa005,
                0xa006,
                0xa007,
                0xa008
            ]
        );
        let grows: Vec<_> = log
            .iter()
            .filter(|(address, _)| *address == ARRAY_SET_SIZE)
            .map(|(_, args)| args.clone())
            .collect();
        let array = strings.addr();
        assert_eq!(grows, vec![vec![array, 3], vec![array, 6], vec![array, 9]]);
        assert_eq!(
            *vectors.borrow(),
            vec![
                (NAME_WORLD_DIR, [1.0, 4.0, 7.0]),
                (NAME_WORLD_UP, [2.0, 5.0, 8.0]),
                (NAME_WORLD_RIGHT, [3.0, 6.0, 9.0]),
            ]
        );
        let floats: Vec<_> = log
            .iter()
            .filter(|(address, _)| *address == GET_VIEWER_STRING_FLOAT)
            .map(|(_, args)| args.clone())
            .collect();
        assert_eq!(
            floats,
            vec![
                vec![NAME_MIN_NEAR_PLANE_DIST, 0.25f32.to_bits()],
                vec![NAME_MAX_FAR_NEAR_RATIO, 500.0f32.to_bits()],
            ]
        );
    }
}
