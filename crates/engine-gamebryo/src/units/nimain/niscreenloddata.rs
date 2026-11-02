//! `NiMain/niscreenloddata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiScreenLODData::ms_RTTI` (the class's `NiRTTI` record).
const NI_SCREEN_LOD_DATA_RTTI: u32 = 0x011f_5ee8;

// Translated from 00a95cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenLODData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_screen_lod_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_SCREEN_LOD_DATA_RTTI)
}

layout! {
    /// `NiScreenLODData` (Xbox PDB), 0x30 bytes; the PC offsets match.
    pub struct NiScreenLODData: 0x30 {
        /// `m_kBound` (Xbox PDB, an `NiBound`): its centre and radius.
        0x08 m_kBound_center_x: f32,
        0x0C m_kBound_center_y: f32,
        0x10 m_kBound_center_z: f32,
        0x14 m_kBound_radius: f32,
        /// `m_kWorldBound` (Xbox PDB): its centre and radius.
        0x18 m_kWorldBound_center_x: f32,
        0x1C m_kWorldBound_center_y: f32,
        0x20 m_kWorldBound_center_z: f32,
        0x24 m_kWorldBound_radius: f32,
        /// `m_uiNumProportions` (Xbox PDB).
        0x28 m_uiNumProportions: u32,
        /// `m_pfProportions` (Xbox PDB): the `float` array.
        0x2C m_pfProportions: u32,
    }
}

use crate::units::nimain::nicamera::NiCamera;

/// `NiScreenLODData`'s vtable (its RTTI names `NiScreenLODData`).
const NI_SCREEN_LOD_DATA_VTABLE: u32 = 0x010a_051c;
/// `00aa09a0` (`thiscall`, no stack word; unnamed in the engine map): the `NiLODData` base
/// constructor; `00aa09c0`: its destructor body.
const LOD_DATA_CONSTRUCTOR: u32 = 0x00aa_09a0;
const LOD_DATA_DESTRUCTOR: u32 = 0x00aa_09c0;
/// `011f426c`: the three words a new bound's centre is copied from.
const BOUND_CENTER_DEFAULT: u32 = 0x011f_426c;
/// `00aa13e0` (`cdecl`, one word): allocates a block of that size; `00aa1070` and
/// `00aa10f0` (`cdecl`, one word): the Ni allocation and free; `00aa1460` (`cdecl`, (block,
/// size)): the sized delete; CRT `memcpy` (`00ec44d0`).
const ALLOCATE: u32 = 0x00aa_13e0;
const NEW_BLOCK: u32 = 0x00aa_1070;
const FREE_BLOCK: u32 = 0x00aa_10f0;
const SIZED_DELETE: u32 = 0x00aa_1460;
const MEMCPY: u32 = 0x00ec_44d0;
/// CRT `sqrt` taking and returning ST0 (`00ec6040`).
const CRT_SQRT: u32 = 0x00ec_6040;
/// `float` 1e-6 (`0x358637bd`).
const DISTANCE_EPSILON: u32 = 0x0109_59f4;
/// `double` 2.0.
const TWO_DOUBLE: u32 = 0x0101_1590;
/// `NiLODNode::UpdateAllChildren` (Xbox PDB, `thiscall` on the node, no stack word).
const LOD_NODE_UPDATE_ALL_CHILDREN: u32 = 0x00a9_91d0;
/// `NiBound::Merge` (Xbox PDB, `thiscall` on the bound, the other bound).
const BOUND_MERGE: u32 = 0x00a7_f3f0;
/// `011f4288`: the bound merged for a child without one.
const EMPTY_BOUND: u32 = 0x011f_4288;
/// `NiTransform::InvertNonUniform` (engine map, `004b4880`; `thiscall` on the transform,
/// the result).
const TRANSFORM_INVERT_NON_UNIFORM: u32 = 0x004b_4880;
/// `NiBound::Update` (engine map, `00a7fdd0`; `thiscall` on the bound written, (bound,
/// transform)).
const BOUND_UPDATE: u32 = 0x00a7_fdd0;
/// `00a724d0` (`thiscall`, the stream; a 5-byte body the engine map names
/// `NiPSysEmitterCtlrData::LinkObject`): the base's save step; `00a88a40` (5 bytes,
/// `NiInterpolator::LoadBinary` in the engine map): the base's load step.
const BASE_SAVE_STEP: u32 = 0x00a7_24d0;
const BASE_LOAD_STEP: u32 = 0x00a8_8a40;
/// `NiSphere::SaveBinary` and `NiSphere::LoadBinary` (engine map; `thiscall` on the bound,
/// the stream).
const BOUND_SAVE_BINARY: u32 = 0x00eb_5410;
const BOUND_LOAD_BINARY: u32 = 0x00a6_9ac0;
/// `00a926b0` (`thiscall` on a bound, the other bound; unnamed in the engine map): the exe
/// treats a nonzero answer as the bounds differing.
const BOUND_COMPARE: u32 = 0x00a9_26b0;
/// `NiStream`'s input (+0x24c, read function at +8) and output (+0x250, write function at
/// +0xc); a file's position is at +4.
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
/// `NiGetViewerString` (Xbox PDB, `cdecl`, one word) and `NiBound::GetViewerString` (Xbox
/// PDB, `thiscall` on the bound, the name).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const BOUND_GET_VIEWER_STRING: u32 = 0x00a7_fb20;
/// `NiSprintf` (`00a305b0`, `cdecl`: buffer, size, format, then the format's arguments).
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `0096ad30` (`thiscall`, new capacity) and `0096ae90` (`thiscall`, index, pointer to the
/// element): the viewer-string array's set-size and set-at.
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `"Bound"`, `"World Bound"`, `"Proportion[%d] = %g"`.
const NAME_BOUND: u32 = 0x010a_05d8;
const NAME_WORLD_BOUND: u32 = 0x010a_05cc;
const FORMAT_PROPORTION: u32 = 0x010a_05b8;

// Translated from 00a95c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenLODData::GetLODIndex` (Xbox PDB): 0 for a negative `lod`, else `lod` clamped
/// to the proportion count (signed compares).
pub fn ni_screen_lod_data_get_lod_index(
    e: &mut Engine,
    this: Ptr<NiScreenLODData>,
    lod: i32,
) -> i32 {
    if lod < 0 {
        return 0;
    }
    let count = e.get(this, NiScreenLODData::m_uiNumProportions) as i32;
    if lod < count {
        lod
    } else {
        count
    }
}

// Translated from 00a95cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenLODData::NiScreenLODData` (Xbox PDB): the `NiLODData` base constructor
/// (`00aa09a0`), the vtable, both bounds' centres copied from the three words at `011f426c`
/// and their radii 0.0, no proportions. Answers `this`.
pub fn ni_screen_lod_data_ni_screen_lod_data(
    e: &mut Engine,
    this: Ptr<NiScreenLODData>,
) -> Ptr<NiScreenLODData> {
    let base = this.addr();
    e.call(LOD_DATA_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(base, NI_SCREEN_LOD_DATA_VTABLE);
    for bound in [8u32, 0x18] {
        for i in 0..3 {
            let word = e.mem.u32(BOUND_CENTER_DEFAULT + 4 * i);
            e.mem.set_u32(base + bound + 4 * i, word);
        }
        e.mem.set_u32(base + bound + 0xc, x87_store32(0.0));
    }
    e.set(this, NiScreenLODData::m_uiNumProportions, 0);
    e.set(this, NiScreenLODData::m_pfProportions, 0);
    this
}

// Translated from 00a95d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenLODData::_scalar_deleting_destructor_` (Xbox PDB): the vtable, the proportions
/// freed (`00aa10f0`), the `NiLODData` destructor body (`00aa09c0`), then the sized delete
/// of 0x30 bytes when bit 0 of `flags` is set. Answers `this`.
pub fn ni_screen_lod_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiScreenLODData>,
    flags: u32,
) -> Ptr<NiScreenLODData> {
    let proportions = e.get(this, NiScreenLODData::m_pfProportions);
    e.mem.set_u32(this.addr(), NI_SCREEN_LOD_DATA_VTABLE);
    e.call(FREE_BLOCK, &args![proportions]);
    e.call(LOD_DATA_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x30u32]);
    }
    this
}

// Translated from 00a95d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word): by its body
/// `NiScreenLODData::SetNumProportions`. Nothing when `count` is the current count; for 0
/// the array is freed and the count and array cleared; otherwise a new array of `count`
/// floats (`00aa1070`) gets the first `min(old, count)` (signed) of the old ones
/// (`_memcpy`, when there was an array), the old one is freed and the new one and `count`
/// stored.
pub fn fn_00a95d60(e: &mut Engine, this: Ptr<NiScreenLODData>, count: i32) {
    type D = NiScreenLODData;
    if count as u32 == e.get(this, D::m_uiNumProportions) {
        return;
    }
    if count == 0 {
        let old = e.get(this, D::m_pfProportions);
        e.call(FREE_BLOCK, &args![old]);
        e.set(this, D::m_pfProportions, 0);
        e.set(this, D::m_uiNumProportions, 0);
        return;
    }
    let array = e
        .call(NEW_BLOCK, &args![(count as u32).wrapping_mul(4)])
        .u32();
    let old = e.get(this, D::m_pfProportions);
    if old != 0 {
        let current = e.get(this, D::m_uiNumProportions) as i32;
        let kept = if current < count { current } else { count };
        e.call(MEMCPY, &args![array, old, (kept as u32).wrapping_mul(4)]);
    }
    let old = e.get(this, D::m_pfProportions);
    e.call(FREE_BLOCK, &args![old]);
    e.set(this, D::m_pfProportions, array);
    e.set(this, D::m_uiNumProportions, count as u32);
}

// Translated from 00a95de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenLODData::GetLODLevel` (Xbox PDB; `thiscall`, `RET 8`: the camera and a word it
/// does not read). The distance from the camera's world position (`NiAVObject` world
/// translation, +0x8c) to the world bound's centre (CRT `sqrt`, stored as a float): 0 when
/// the world radius is above it, or when it is not above 1e-6 (ordered). Otherwise the
/// screen size `max(2 / (right - left), 2 / (top - bottom))` of the camera's frustum
/// (the second when the first is not below it or either is NaN) times the radius (divided
/// by the distance unless the frustum is orthographic) times the camera's LOD adjust; the
/// answer is the index of the first proportion at most that size (ordered), or the
/// proportion count.
pub fn ni_screen_lod_data_get_lod_level(
    e: &mut Engine,
    this: Ptr<NiScreenLODData>,
    camera: Ptr<NiCamera>,
    _unused_1: u32,
) -> u32 {
    type D = NiScreenLODData;
    let ld = |e: &Engine, a: u32| x87_load32(e.mem.u32(a));
    let operand = |e: &Engine, a: u32| x87_m32(e.mem.u32(a));
    let me = this.addr();
    let cam = camera.addr();
    let d0 = x87_store32(x87_sub(ld(e, me + 0x18), operand(e, cam + 0x8c)));
    let d1 = x87_store32(x87_sub(ld(e, me + 0x1c), operand(e, cam + 0x90)));
    let d2 = x87_store32(x87_sub(ld(e, me + 0x20), operand(e, cam + 0x94)));
    let (d0, d1, d2) = (x87_load32(d0), x87_load32(d1), x87_load32(d2));
    let sum = x87_add(x87_mul(d1, d1), x87_mul(d0, d0));
    let length_squared = x87_load32(x87_store32(x87_add(sum, x87_mul(d2, d2))));
    let root = e.call(CRT_SQRT, &args![length_squared]).f64();
    let distance = x87_load32(x87_store32(x87_load32(x87_store32(root))));
    let radius = ld(e, me + 0x24);
    // FLD radius; FCOMP distance; TEST AH,0x41; JZ: a radius above the distance answers 0.
    if radius > distance {
        return 0;
    }
    // FLD 1e-6; FCOMP distance; TEST AH,1; JZ: 1e-6 not below the distance answers 0.
    if !(ld(e, DISTANCE_EPSILON) < distance) && !distance.is_nan() {
        return 0;
    }
    let scale = if e.get(camera, NiCamera::m_kViewFrustum_m_bOrtho) {
        ld(e, me + 0x24)
    } else {
        x87_div(operand(e, me + 0x24), distance)
    };
    let scale = x87_store32(scale);
    let two = e.mem.f64(TWO_DOUBLE);
    let width = x87_sub(
        ld(e, cam + 0xe0),
        x87_m32(e.get(camera, NiCamera::m_kViewFrustum_m_fLeft).to_bits()),
    );
    let across = x87_store32(x87_div(two, width));
    let height = x87_sub(
        ld(e, cam + 0xe4),
        x87_m32(e.get(camera, NiCamera::m_kViewFrustum_m_fBottom).to_bits()),
    );
    let up = x87_store32(x87_div(two, height));
    let (across, up) = (x87_load32(across), x87_load32(up));
    // FCOM up against across; TEST AH,5; JP: `up` below `across` (ordered) keeps `across`.
    let size = if up < across { across } else { up };
    let size = x87_load32(x87_store32(size));
    let size = x87_mul(size, x87_m32(scale));
    let lod_adjust = e.get(camera, NiCamera::m_fLODAdjust).to_bits();
    let size = x87_load32(x87_store32(x87_mul(size, x87_m32(lod_adjust))));
    let count = e.get(this, D::m_uiNumProportions);
    let mut index = 0u32;
    if count != 0 {
        let mut proportion = e.get(this, D::m_pfProportions);
        loop {
            // FLD proportion; FCOMP size; TEST AH,0x41; JNP: below or equal ends the search.
            let value = ld(e, proportion);
            if value <= size {
                break;
            }
            index += 1;
            proportion = proportion.wrapping_add(4);
            if index >= count {
                break;
            }
        }
    }
    index
}

// Translated from 00a95f20 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; the creation function the
/// stream registers for `NiScreenLODData`, `0109f054`): a new screen LOD data (0x30 bytes
/// from `00aa13e0`, the constructor reached with a `JMP`), or null when the allocation
/// fails.
pub fn fn_00a95f20(e: &mut Engine) -> Ptr<NiScreenLODData> {
    let block = e.call(ALLOCATE, &args![0x30u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    ni_screen_lod_data_ni_screen_lod_data(e, Ptr::new(block))
}

/// The body `Duplicate` and `CreateClone` share: a new screen LOD data (null when the
/// allocation fails, written through all the same), the model bound copied (the radius
/// through `FLD`/`FSTP`), the proportion count set ([`fn_00a95d60`]) and the proportions
/// copied (`_memcpy`).
fn duplicate(e: &mut Engine, this: Ptr<NiScreenLODData>) -> Ptr<NiScreenLODData> {
    type D = NiScreenLODData;
    let block = e.call(ALLOCATE, &args![0x30u32]).u32();
    let copy: Ptr<D> = if block == 0 {
        Ptr::new(0)
    } else {
        ni_screen_lod_data_ni_screen_lod_data(e, Ptr::new(block))
    };
    let me = this.addr();
    let to = copy.addr();
    let x = e.mem.u32(me + 8);
    let radius = x87_quiet_bits(e.mem.u32(me + 0x14));
    e.mem.set_u32(to.wrapping_add(8), x);
    let y = e.mem.u32(me + 0xc);
    e.mem.set_u32(to.wrapping_add(0xc), y);
    let z = e.mem.u32(me + 0x10);
    e.mem.set_u32(to.wrapping_add(0x14), radius);
    e.mem.set_u32(to.wrapping_add(0x10), z);
    let count = e.get(this, D::m_uiNumProportions);
    fn_00a95d60(e, copy, count as i32);
    let count = e.get(this, D::m_uiNumProportions);
    let from = e.get(this, D::m_pfProportions);
    let into = e.get(copy, D::m_pfProportions);
    e.call(MEMCPY, &args![into, from, count.wrapping_mul(4)]);
    copy
}

// Translated from 00a95f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenLODData::Duplicate` (Xbox PDB; `thiscall`, no stack word): a copy with the
/// model bound and the proportions ([`duplicate`]).
pub fn ni_screen_lod_data_duplicate(
    e: &mut Engine,
    this: Ptr<NiScreenLODData>,
) -> Ptr<NiScreenLODData> {
    duplicate(e, this)
}

// Translated from 00a95fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenLODData::AutoSetBound` (Xbox PDB): the node's children updated
/// (`NiLODNode::UpdateAllChildren`), the world radius cleared, every child's world bound
/// (the `NiAVObject` bound pointer at +0x20, or `011f4288` when null) merged into the
/// world bound (`NiBound::Merge`) for the node's children (array +0xa0, 16-bit count
/// +0xa6), then the model bound recomputed from the world bound and the inverse of the
/// node's world transform (+0x68, `InvertNonUniform` into a 0x34-byte stack local,
/// `NiBound::Update`).
pub fn ni_screen_lod_data_auto_set_bound(e: &mut Engine, this: Ptr<NiScreenLODData>, node: Ptr) {
    let me = this.addr();
    let node = node.addr();
    e.call(LOD_NODE_UPDATE_ALL_CHILDREN, &args![node]);
    e.mem.set_u32(me + 0x24, x87_store32(0.0));
    let mut i = 0u32;
    while i < u32::from(e.mem.u16(node + 0xa6)) {
        let children = e.mem.u32(node + 0xa0);
        let child = e.mem.u32(children.wrapping_add(4 * i));
        if child != 0 {
            let bound = e.mem.u32(child.wrapping_add(0x20));
            let bound = if bound == 0 { EMPTY_BOUND } else { bound };
            e.call(BOUND_MERGE, &args![me + 0x18, bound]);
        }
        i += 1;
    }
    e.with_stack(0x34, |e, inverse| {
        e.call(TRANSFORM_INVERT_NON_UNIFORM, &args![node + 0x68, inverse]);
        e.call(BOUND_UPDATE, &args![me + 8, me + 0x18, inverse]);
    });
}

// Translated from 00a96030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenLODData::CreateClone` (Xbox PDB; `thiscall`, `RET 4`: the cloning process,
/// not read): the same copy as `Duplicate` ([`duplicate`]).
pub fn ni_screen_lod_data_create_clone(
    e: &mut Engine,
    this: Ptr<NiScreenLODData>,
    _unused_0: u32,
) -> Ptr<NiScreenLODData> {
    duplicate(e, this)
}

/// One read or write through the stream's input (read function at +8) or output (write
/// function at +0xc): `function(file, at, size, count, 1)` with the word at `count` set to
/// `count_value` first, the answer added to the file's position.
fn stream_transfer(
    e: &mut Engine,
    stream: Ptr,
    write: bool,
    at: u32,
    size: u32,
    count: u32,
    count_value: u32,
) {
    let (file_slot, function_slot) = if write {
        (STREAM_OUTPUT, 0xc)
    } else {
        (STREAM_INPUT, 8)
    };
    let file = e.mem.u32(stream.addr().wrapping_add(file_slot));
    let function = e.mem.u32(file.wrapping_add(function_slot));
    e.mem.set_u32(count, count_value);
    let done = e.call(function, &args![file, at, size, count, 1u32]).u32();
    let position = e.mem.u32(file.wrapping_add(4)).wrapping_add(done);
    e.mem.set_u32(file.wrapping_add(4), position);
}

// Translated from 00a960a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenLODData::SaveBinary` (Xbox PDB): the base's step (`00a724d0`), both bounds
/// (`00eb5410`), then to the stream's output the proportion count and each proportion (4
/// bytes each; the byte-count word is the stream argument's stack word).
pub fn ni_screen_lod_data_save_binary(e: &mut Engine, this: Ptr<NiScreenLODData>, stream: Ptr) {
    type D = NiScreenLODData;
    let me = this.addr();
    e.call(BASE_SAVE_STEP, &args![this, stream]);
    e.call(BOUND_SAVE_BINARY, &args![me + 8, stream]);
    e.call(BOUND_SAVE_BINARY, &args![me + 0x18, stream]);
    e.with_stack(4, |e, count| {
        let count = count.addr();
        stream_transfer(e, stream, true, me + 0x28, 4, count, 4);
        let mut i = 0u32;
        while i < e.get(this, D::m_uiNumProportions) {
            let at = e.get(this, D::m_pfProportions).wrapping_add(4 * i);
            stream_transfer(e, stream, true, at, 4, count, 4);
            i += 1;
        }
    });
}

// Translated from 00a96130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenLODData::IsEqual` (Xbox PDB): both bounds compared (`00a926b0` answering zero;
/// the `NiObject` base is not asked), the same proportion count, then every proportion
/// equal (`FUCOMPP`, a NaN never equal).
pub fn ni_screen_lod_data_is_equal(e: &mut Engine, this: Ptr<NiScreenLODData>, other: Ptr) -> bool {
    type D = NiScreenLODData;
    let me = this.addr();
    let them = other.addr();
    if e.call(BOUND_COMPARE, &args![me + 8, them + 8]).bool() {
        return false;
    }
    if e.call(BOUND_COMPARE, &args![me + 0x18, them + 0x18]).bool() {
        return false;
    }
    let count = e.get(this, D::m_uiNumProportions);
    if count != e.mem.u32(them + 0x28) {
        return false;
    }
    let mut mine = e.get(this, D::m_pfProportions);
    let gap = e.mem.u32(them + 0x2c).wrapping_sub(mine);
    for _ in 0..count {
        let theirs = x87_load32(e.mem.u32(gap.wrapping_add(mine)));
        let value = x87_load32(e.mem.u32(mine));
        if value != theirs {
            return false;
        }
        mine = mine.wrapping_add(4);
    }
    true
}

// Translated from 00a961a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenLODData::UpdateWorldData` (Xbox PDB): with a model radius of 0 (`FCOMP`, equal
/// and ordered) the bound is set from the node ([`ni_screen_lod_data_auto_set_bound`], a
/// `JMP`); otherwise the world bound is the model bound under the node's world transform
/// (+0x68, `NiBound::Update`).
pub fn ni_screen_lod_data_update_world_data(e: &mut Engine, this: Ptr<NiScreenLODData>, node: Ptr) {
    let radius = x87_m32(e.mem.u32(this.addr() + 0x14));
    if 0.0 == radius {
        ni_screen_lod_data_auto_set_bound(e, this, node);
        return;
    }
    let me = this.addr();
    e.call(BOUND_UPDATE, &args![me + 0x18, me + 8, node.addr() + 0x68]);
}

// Translated from 00a961d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenLODData::LoadBinary` (Xbox PDB): the base's step (`00a88a40`), both bounds
/// (`00a69ac0`), the proportion count read into the stream argument's stack word (the
/// byte-count word being the function's own local), the count set ([`fn_00a95d60`]), then
/// each proportion read (4 bytes) while below that count.
pub fn ni_screen_lod_data_load_binary(e: &mut Engine, this: Ptr<NiScreenLODData>, stream: Ptr) {
    let me = this.addr();
    e.call(BASE_LOAD_STEP, &args![this, stream]);
    e.call(BOUND_LOAD_BINARY, &args![me + 8, stream]);
    e.call(BOUND_LOAD_BINARY, &args![me + 0x18, stream]);
    e.with_stack(8, |e, slots| {
        let argument = slots.addr();
        let count = argument + 4;
        stream_transfer(e, stream, false, argument, 4, count, 4);
        let proportions = e.mem.u32(argument);
        fn_00a95d60(e, this, proportions as i32);
        let mut i = 0u32;
        while i < e.mem.u32(argument) {
            let at = e
                .get(this, NiScreenLODData::m_pfProportions)
                .wrapping_add(4 * i);
            stream_transfer(e, stream, false, at, 4, count, 4);
            i += 1;
        }
    });
}

// Translated from 00a96270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiScreenLODData::GetViewerStrings` (Xbox PDB; no base class strings): the class name
/// (the first word of `NiScreenLODData::ms_RTTI`, `011f5ee8`, through `NiGetViewerString`),
/// `"Bound"` and `"World Bound"` (`NiBound::GetViewerString` of each bound) appended
/// through the array's set-at with the string kept in a stack local; then for each
/// proportion a text of 0x80 bytes (`00aa1070`) with `"Proportion[%d] = %g"` (`NiSprintf`)
/// stored by the inlined set-at-grow: the array grows when full (`0096ad30`); an index past
/// the count (+0xa) makes it the new count and counts a non-null text in `m_usESize` (+0xc);
/// an index inside it adjusts `m_usESize` when a null slot gets a text or a text slot gets
/// null.
pub fn ni_screen_lod_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiScreenLODData>,
    strings: Ptr,
) {
    type D = NiScreenLODData;
    let array = strings.addr();
    let grow = |e: &mut Engine, count: u32| {
        if count >= u32::from(e.mem.u16(array + 8)) {
            let grow_by = u32::from(e.mem.u16(array + 0xe));
            e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
        }
    };
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.global::<u32>(NI_SCREEN_LOD_DATA_RTTI);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        for step in 0..3 {
            let text = match step {
                0 => text,
                1 => e
                    .call(BOUND_GET_VIEWER_STRING, &args![this.addr() + 8, NAME_BOUND])
                    .u32(),
                _ => e
                    .call(
                        BOUND_GET_VIEWER_STRING,
                        &args![this.addr() + 0x18, NAME_WORLD_BOUND],
                    )
                    .u32(),
            };
            let count = u32::from(e.mem.u16(array + 0xa));
            e.mem.set_u32(slot, text);
            grow(e, count);
            e.call(ARRAY_SET_AT, &args![strings, count, slot]);
        }
    });
    let mut i = 0u32;
    while i < e.get(this, D::m_uiNumProportions) {
        let text = e.call(NEW_BLOCK, &args![0x80u32]).u32();
        let proportions = e.get(this, D::m_pfProportions);
        let value = x87_load32(e.mem.u32(proportions.wrapping_add(4 * i)));
        e.call(
            NI_SPRINTF,
            &args![text, 0x80u32, FORMAT_PROPORTION, i, value],
        );
        let index = u32::from(e.mem.u16(array + 0xa));
        grow(e, index);
        let slots = e.mem.u32(array + 4);
        if index < u32::from(e.mem.u16(array + 0xa)) {
            let held = e.mem.u32(slots.wrapping_add(4 * index));
            if text != 0 {
                if held == 0 {
                    let used = e.mem.u16(array + 0xc).wrapping_add(1);
                    e.mem.set_u16(array + 0xc, used);
                }
            } else if held != 0 {
                let used = e.mem.u16(array + 0xc).wrapping_add(0xffff);
                e.mem.set_u16(array + 0xc, used);
            }
        } else {
            e.mem.set_u16(array + 0xa, (index + 1) as u16);
            if text != 0 {
                let used = e.mem.u16(array + 0xc).wrapping_add(1);
                e.mem.set_u16(array + 0xc, used);
            }
        }
        let slots = e.mem.u32(array + 4);
        e.mem.set_u32(slots.wrapping_add(4 * index), text);
        i += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00a95cb0, ni_screen_lod_data_get_rtti() -> Ptr),
        entry!(
            0x00a9_5c80,
            ni_screen_lod_data_get_lod_index(Ptr<NiScreenLODData>, i32) -> i32
        ),
        entry!(
            0x00a9_5cc0,
            ni_screen_lod_data_ni_screen_lod_data(Ptr<NiScreenLODData>) -> Ptr<NiScreenLODData>
        ),
        entry!(
            0x00a9_5d20,
            ni_screen_lod_data_scalar_deleting_destructor(
                Ptr<NiScreenLODData>,
                u32,
            ) -> Ptr<NiScreenLODData>
        ),
        entry!(0x00a9_5d60, fn_00a95d60(Ptr<NiScreenLODData>, i32)),
        entry!(
            0x00a9_5de0,
            ni_screen_lod_data_get_lod_level(Ptr<NiScreenLODData>, Ptr<NiCamera>, u32) -> u32
        ),
        entry!(0x00a9_5f20, fn_00a95f20() -> Ptr<NiScreenLODData>),
        entry!(
            0x00a9_5f40,
            ni_screen_lod_data_duplicate(Ptr<NiScreenLODData>) -> Ptr<NiScreenLODData>
        ),
        entry!(
            0x00a9_5fb0,
            ni_screen_lod_data_auto_set_bound(Ptr<NiScreenLODData>, Ptr)
        ),
        entry!(
            0x00a9_6030,
            ni_screen_lod_data_create_clone(Ptr<NiScreenLODData>, u32) -> Ptr<NiScreenLODData>
        ),
        entry!(
            0x00a9_60a0,
            ni_screen_lod_data_save_binary(Ptr<NiScreenLODData>, Ptr)
        ),
        entry!(
            0x00a9_6130,
            ni_screen_lod_data_is_equal(Ptr<NiScreenLODData>, Ptr) -> bool
        ),
        entry!(
            0x00a9_61a0,
            ni_screen_lod_data_update_world_data(Ptr<NiScreenLODData>, Ptr)
        ),
        entry!(
            0x00a9_61d0,
            ni_screen_lod_data_load_binary(Ptr<NiScreenLODData>, Ptr)
        ),
        entry!(
            0x00a9_6270,
            ni_screen_lod_data_get_viewer_strings(Ptr<NiScreenLODData>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_screen_lod_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a95cb0, &args![]).u32(), NI_SCREEN_LOD_DATA_RTTI);
    }
}

#[cfg(test)]
mod n018_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .clone()
            .unwrap_or_default()
            .into_iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, words)| words)
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_1000, 0x0109_5000, 0x011f_4000, 0x011f_5000] {
            e.map(page, 0x1000);
        }
        e.mem.set_f64(TWO_DOUBLE, 2.0);
        e.mem.set_u32(DISTANCE_EPSILON, 0x3586_37bd);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NEW_BLOCK, |e, a| e.mem.alloc(a[0].max(1)).into_ret());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.register(LOD_DATA_CONSTRUCTOR, |_, _| Ret::default());
        e.register(MEMCPY, |e, a| {
            for i in 0..a[2] {
                let byte = e.mem.u8(a[1] + i);
                e.mem.set_u8(a[0] + i, byte);
            }
            Ret::default()
        });
        e.register(CRT_SQRT, |_, a| f64::take(a, &mut 0).sqrt().into_ret());
        e.call_log = Some(vec![]);
        e
    }

    /// A screen LOD data with the given proportions.
    fn data(e: &mut Engine, proportions: &[f32]) -> u32 {
        let this = e.mem.alloc(0x30);
        let array = e.mem.alloc(4 * proportions.len().max(1) as u32);
        for (i, p) in proportions.iter().enumerate() {
            e.mem.set_f32(array + 4 * i as u32, *p);
        }
        e.mem.set_u32(this + 0x28, proportions.len() as u32);
        e.mem.set_u32(this + 0x2c, array);
        this
    }

    fn proportions(e: &Engine, this: u32) -> Vec<f32> {
        let array = e.mem.u32(this + 0x2c);
        (0..e.mem.u32(this + 0x28))
            .map(|i| e.mem.f32(array + 4 * i))
            .collect()
    }

    #[test]
    fn lod_index_is_clamped() {
        let mut e = engine();
        let this = data(&mut e, &[1.0, 2.0, 3.0]);
        assert_eq!(e.call(0x00a9_5c80, &args![this, -5i32]).i32(), 0);
        assert_eq!(e.call(0x00a9_5c80, &args![this, 2i32]).i32(), 2);
        assert_eq!(e.call(0x00a9_5c80, &args![this, 7i32]).i32(), 3);
    }

    #[test]
    fn constructor_and_creation_function() {
        let mut e = engine();
        for i in 0..3 {
            e.mem.set_f32(BOUND_CENTER_DEFAULT + 4 * i, 1.0 + i as f32);
        }
        let this = e.mem.alloc(0x30);
        e.mem.set_u32(this + 0x14, 0xdead);
        assert_eq!(e.call(0x00a9_5cc0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_SCREEN_LOD_DATA_VTABLE);
        let floats: Vec<f32> = (0..8).map(|i| e.mem.f32(this + 8 + 4 * i)).collect();
        assert_eq!(floats, vec![1.0, 2.0, 3.0, 0.0, 1.0, 2.0, 3.0, 0.0]);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        let made = e.call(0x00a9_5f20, &args![]).u32();
        assert_eq!(e.mem.u32(made), NI_SCREEN_LOD_DATA_VTABLE);
        assert_eq!(
            calls_to(&e, LOD_DATA_CONSTRUCTOR),
            vec![vec![this], vec![made]]
        );
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_5f20, &args![]).u32(), 0);
    }

    #[test]
    fn destructor_frees_and_deletes_on_bit_0() {
        let mut e = engine();
        e.register(LOD_DATA_DESTRUCTOR, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = data(&mut e, &[1.0]);
        let array = e.mem.u32(this + 0x2c);
        e.call(0x00a9_5d20, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this), NI_SCREEN_LOD_DATA_VTABLE);
        assert_eq!(calls_to(&e, FREE_BLOCK), vec![vec![array]]);
        assert_eq!(calls_to(&e, LOD_DATA_DESTRUCTOR), vec![vec![this]]);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        e.call(0x00a9_5d20, &args![this, 1u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x30]]);
    }

    #[test]
    fn set_num_proportions_keeps_the_common_part() {
        let mut e = engine();
        let this = data(&mut e, &[1.0, 2.0, 3.0]);
        let old = e.mem.u32(this + 0x2c);
        e.call(0x00a9_5d60, &args![this, 2i32]);
        assert_eq!(proportions(&e, this), vec![1.0, 2.0]);
        assert_eq!(calls_to(&e, MEMCPY)[0][2], 8);
        assert_eq!(calls_to(&e, FREE_BLOCK), vec![vec![old]]);
        e.call(0x00a9_5d60, &args![this, 4i32]);
        assert_eq!(calls_to(&e, MEMCPY)[1][2], 8);
        assert_eq!(e.mem.u32(this + 0x28), 4);
        // The same count does nothing; 0 frees and clears.
        e.call(0x00a9_5d60, &args![this, 4i32]);
        assert_eq!(calls_to(&e, NEW_BLOCK).len(), 2);
        e.call(0x00a9_5d60, &args![this, 0i32]);
        assert_eq!(e.mem.u32(this + 0x28), 0);
        assert_eq!(e.mem.u32(this + 0x2c), 0);
        // Without an array nothing is copied.
        e.call(0x00a9_5d60, &args![this, 3i32]);
        assert_eq!(calls_to(&e, MEMCPY).len(), 2);
    }

    /// A camera at the origin with frustum (-1, 1, 0.5, -0.5), LOD adjust 1.
    fn camera(e: &mut Engine, ortho: bool) -> u32 {
        let camera = e.mem.alloc(0x114);
        e.mem.set_f32(camera + 0xdc, -1.0);
        e.mem.set_f32(camera + 0xe0, 1.0);
        e.mem.set_f32(camera + 0xe4, 0.5);
        e.mem.set_f32(camera + 0xe8, -0.5);
        e.mem.set_u8(camera + 0xf4, ortho as u8);
        e.mem.set_f32(camera + 0x110, 1.0);
        camera
    }

    #[test]
    fn lod_level_picks_the_first_proportion_at_most_the_screen_size() {
        let mut e = engine();
        // World bound at distance 10, radius 5: size max(1, 2) * 5 / 10 = 1.
        let this = data(&mut e, &[4.0, 2.0, 1.0, 0.5]);
        e.mem.set_f32(this + 0x18, 6.0);
        e.mem.set_f32(this + 0x1c, 8.0);
        e.mem.set_f32(this + 0x24, 5.0);
        let cam = camera(&mut e, false);
        assert_eq!(e.call(0x00a9_5de0, &args![this, cam, 0u32]).u32(), 2);
        // Orthographic: the radius itself, size 10: the first proportion.
        let ortho = camera(&mut e, true);
        assert_eq!(e.call(0x00a9_5de0, &args![this, ortho, 0u32]).u32(), 0);
        // Every proportion above the size: the count.
        e.mem.set_f32(cam + 0x110, 0.1);
        assert_eq!(e.call(0x00a9_5de0, &args![this, cam, 0u32]).u32(), 4);
        // Inside the bound (radius above the distance): 0.
        e.mem.set_f32(this + 0x24, 50.0);
        assert_eq!(e.call(0x00a9_5de0, &args![this, cam, 0u32]).u32(), 0);
    }

    #[test]
    fn lod_level_is_0_at_the_camera_position() {
        let mut e = engine();
        let this = data(&mut e, &[4.0]);
        let cam = camera(&mut e, false);
        e.mem.set_f32(this + 0x24, -1.0);
        // Distance 0: not above 1e-6.
        assert_eq!(e.call(0x00a9_5de0, &args![this, cam, 0u32]).u32(), 0);
        // An empty proportion list answers 0 too.
        let empty = data(&mut e, &[]);
        e.mem.set_f32(empty + 0x18, 3.0);
        assert_eq!(e.call(0x00a9_5de0, &args![empty, cam, 0u32]).u32(), 0);
    }

    #[test]
    fn duplicate_and_clone_copy_the_bound_and_proportions() {
        let mut e = engine();
        let this = data(&mut e, &[3.0, 1.5]);
        for i in 0..4 {
            e.mem.set_f32(this + 8 + 4 * i, 10.0 + i as f32);
        }
        e.mem.set_u32(this + 0x14, 0x7f80_0001);
        for address in [0x00a9_5f40u32, 0x00a9_6030] {
            let copy = if address == 0x00a9_5f40 {
                e.call(address, &args![this]).u32()
            } else {
                e.call(address, &args![this, 0x4444u32]).u32()
            };
            assert_eq!(e.mem.u32(copy), NI_SCREEN_LOD_DATA_VTABLE);
            assert_eq!(e.mem.f32(copy + 8), 10.0);
            assert_eq!(e.mem.f32(copy + 0x10), 12.0);
            assert_eq!(e.mem.u32(copy + 0x14), 0x7fc0_0001);
            assert_eq!(proportions(&e, copy), vec![3.0, 1.5]);
        }
    }

    #[test]
    fn auto_set_bound_merges_the_children_and_inverts() {
        let mut e = engine();
        for double in [
            LOD_NODE_UPDATE_ALL_CHILDREN,
            BOUND_MERGE,
            TRANSFORM_INVERT_NON_UNIFORM,
            BOUND_UPDATE,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        let this = data(&mut e, &[]);
        e.mem.set_f32(this + 0x24, 9.0);
        let node = e.mem.alloc(0xb0);
        let children = e.mem.alloc(12);
        let with_bound = e.mem.alloc(0x24);
        e.mem.set_u32(with_bound + 0x20, 0x5000);
        let without = e.mem.alloc(0x24);
        e.mem.set_u32(children, with_bound);
        e.mem.set_u32(children + 8, without);
        e.mem.set_u32(node + 0xa0, children);
        e.mem.set_u16(node + 0xa6, 3);
        e.call(0x00a9_5fb0, &args![this, node]);
        assert_eq!(e.mem.f32(this + 0x24), 0.0);
        assert_eq!(calls_to(&e, LOD_NODE_UPDATE_ALL_CHILDREN), vec![vec![node]]);
        assert_eq!(
            calls_to(&e, BOUND_MERGE),
            vec![vec![this + 0x18, 0x5000], vec![this + 0x18, EMPTY_BOUND]]
        );
        let invert = calls_to(&e, TRANSFORM_INVERT_NON_UNIFORM);
        assert_eq!(invert[0][0], node + 0x68);
        assert_eq!(
            calls_to(&e, BOUND_UPDATE),
            vec![vec![this + 8, this + 0x18, invert[0][1]]]
        );
    }

    #[test]
    fn update_world_data_transforms_or_sets_the_bound() {
        let mut e = engine();
        for double in [
            LOD_NODE_UPDATE_ALL_CHILDREN,
            TRANSFORM_INVERT_NON_UNIFORM,
            BOUND_UPDATE,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        let this = data(&mut e, &[]);
        let node = e.mem.alloc(0xb0);
        e.mem.set_f32(this + 0x14, 2.0);
        e.call(0x00a9_61a0, &args![this, node]);
        assert_eq!(
            calls_to(&e, BOUND_UPDATE),
            vec![vec![this + 0x18, this + 8, node + 0x68]]
        );
        assert!(calls_to(&e, LOD_NODE_UPDATE_ALL_CHILDREN).is_empty());
        e.mem.set_f32(this + 0x14, 0.0);
        e.call(0x00a9_61a0, &args![this, node]);
        assert_eq!(calls_to(&e, LOD_NODE_UPDATE_ALL_CHILDREN), vec![vec![node]]);
    }

    #[test]
    fn is_equal_compares_bounds_and_proportions() {
        let mut e = engine();
        e.register(BOUND_COMPARE, |_, _| 0u32.into_ret());
        let a = data(&mut e, &[1.0, 2.0]);
        let b = data(&mut e, &[1.0, 2.0]);
        assert!(e.call(0x00a9_6130, &args![a, b]).bool());
        assert_eq!(
            calls_to(&e, BOUND_COMPARE),
            vec![vec![a + 8, b + 8], vec![a + 0x18, b + 0x18]]
        );
        let c = data(&mut e, &[1.0, 2.5]);
        assert!(!e.call(0x00a9_6130, &args![a, c]).bool());
        let d = data(&mut e, &[1.0]);
        assert!(!e.call(0x00a9_6130, &args![a, d]).bool());
        let nan = data(&mut e, &[f32::NAN, 2.0]);
        assert!(!e.call(0x00a9_6130, &args![nan, nan]).bool());
        e.register(BOUND_COMPARE, |_, _| 1u32.into_ret());
        assert!(!e.call(0x00a9_6130, &args![a, b]).bool());
    }

    type Transfers = Rc<RefCell<Vec<(u32, u32, u32, u32)>>>;

    fn stream(e: &mut Engine, words: Vec<u32>) -> (u32, Transfers) {
        let log: Transfers = Rc::default();
        let reads = log.clone();
        let mut queue = words.into_iter();
        e.register_double(0x0700_0008, move |e, a| {
            reads.borrow_mut().push((a[1], a[2], a[3], e.mem.u32(a[3])));
            let word = queue.next().unwrap();
            e.mem.set_u32(a[1], word);
            a[2].into_ret()
        });
        let writes = log.clone();
        e.register_double(0x0700_000c, move |e, a| {
            writes
                .borrow_mut()
                .push((a[1], a[2], a[3], e.mem.u32(a[3])));
            a[2].into_ret()
        });
        let file = e.mem.alloc(0x10);
        e.mem.set_u32(file + 8, 0x0700_0008);
        e.mem.set_u32(file + 0xc, 0x0700_000c);
        let stream = e.mem.alloc(0x254);
        e.mem.set_u32(stream + STREAM_INPUT, file);
        e.mem.set_u32(stream + STREAM_OUTPUT, file);
        (stream, log)
    }

    #[test]
    fn save_and_load_binary_move_the_proportions() {
        let mut e = engine();
        for double in [
            BASE_SAVE_STEP,
            BASE_LOAD_STEP,
            BOUND_SAVE_BINARY,
            BOUND_LOAD_BINARY,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        let (stream, log) = stream(
            &mut e,
            vec![3, 1.0f32.to_bits(), 2.0f32.to_bits(), 4.0f32.to_bits()],
        );
        let this = data(&mut e, &[5.0, 6.0]);
        e.call(0x00a9_60a0, &args![this, stream]);
        assert_eq!(
            calls_to(&e, BOUND_SAVE_BINARY),
            vec![vec![this + 8, stream], vec![this + 0x18, stream]]
        );
        let array = e.mem.u32(this + 0x2c);
        let written: Vec<(u32, u32)> = log.borrow().iter().map(|w| (w.0, w.3)).collect();
        assert_eq!(written, vec![(this + 0x28, 4), (array, 4), (array + 4, 4)]);
        log.borrow_mut().clear();
        let loaded = data(&mut e, &[]);
        e.call(0x00a9_61d0, &args![loaded, stream]);
        assert_eq!(
            calls_to(&e, BOUND_LOAD_BINARY),
            vec![vec![loaded + 8, stream], vec![loaded + 0x18, stream]]
        );
        assert_eq!(proportions(&e, loaded), vec![1.0, 2.0, 4.0]);
        let log = log.borrow();
        assert_eq!(log.len(), 4);
        assert!(log.iter().all(|r| r.1 == 4 && r.3 == 4 && r.2 == log[0].2));
        assert_ne!(log[0].0, log[0].2);
    }

    #[test]
    fn viewer_strings_list_the_bounds_and_proportions() {
        let mut e = engine();
        e.mem.set_u32(NI_SCREEN_LOD_DATA_RTTI, 0x0109_5000);
        e.register(GET_VIEWER_STRING, |_, _| 0x5000u32.into_ret());
        e.register(BOUND_GET_VIEWER_STRING, |_, a| {
            (0x6000 + a[1] % 0x100).into_ret()
        });
        e.register(NI_SPRINTF, |_, _| Ret::default());
        e.register(ARRAY_SET_SIZE, |e, a| {
            // Grows the slots to the new capacity.
            let slots = e.mem.alloc(4 * a[1]);
            let old = e.mem.u32(a[0] + 4);
            let count = u32::from(e.mem.u16(a[0] + 0xa));
            for i in 0..count {
                let v = e.mem.u32(old + 4 * i);
                e.mem.set_u32(slots + 4 * i, v);
            }
            e.mem.set_u32(a[0] + 4, slots);
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
        let strings = e.mem.alloc(0x10);
        let slots = e.mem.alloc(16);
        e.mem.set_u32(strings + 4, slots);
        e.mem.set_u16(strings + 8, 4);
        e.mem.set_u16(strings + 0xe, 2);
        let this = data(&mut e, &[0.5, 0.25]);
        e.call(0x00a9_6270, &args![this, strings]);
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x0109_5000]]);
        assert_eq!(
            calls_to(&e, BOUND_GET_VIEWER_STRING),
            vec![
                vec![this + 8, NAME_BOUND],
                vec![this + 0x18, NAME_WORLD_BOUND]
            ]
        );
        let texts = calls_to(&e, NI_SPRINTF);
        assert_eq!(
            texts.iter().map(|t| t[1..].to_vec()).collect::<Vec<_>>(),
            vec![
                args![0x80u32, FORMAT_PROPORTION, 0u32, 0.5f64],
                args![0x80u32, FORMAT_PROPORTION, 1u32, 0.25f64]
            ]
        );
        // The fifth string grew the array; both texts counted as set.
        assert_eq!(calls_to(&e, ARRAY_SET_SIZE), vec![vec![strings, 6]]);
        assert_eq!(e.mem.u16(strings + 0xa), 5);
        assert_eq!(e.mem.u16(strings + 0xc), 2);
        let slots = e.mem.u32(strings + 4);
        assert_eq!(e.mem.u32(slots + 12), texts[0][0]);
        assert_eq!(e.mem.u32(slots + 16), texts[1][0]);
    }
}
