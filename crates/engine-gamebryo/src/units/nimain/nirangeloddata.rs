//! `NiMain/nirangeloddata.cpp` (Xbox PDB source unit), subsystem `NiMain`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `NiRangeLODData::ms_RTTI` (the class's `NiRTTI` record).
const NI_RANGE_LOD_DATA_RTTI: u32 = 0x011f_5ef8;

// Translated from 00a976f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRangeLODData::GetRTTI` (Xbox PDB): the address of the class's `NiRTTI` record.
pub fn ni_range_lod_data_get_rtti(_e: &mut Engine) -> Ptr {
    Ptr::new(NI_RANGE_LOD_DATA_RTTI)
}

layout! {
    /// `NiRangeLODData` (Xbox PDB), 0x28 bytes; the PC offsets match.
    pub struct NiRangeLODData: 0x28 {
        /// `m_kCenter` (Xbox PDB, an `NiPoint3`).
        0x08 m_kCenter_x: f32,
        0x0C m_kCenter_y: f32,
        0x10 m_kCenter_z: f32,
        /// `m_kWorldCenter` (Xbox PDB).
        0x14 m_kWorldCenter_x: f32,
        0x18 m_kWorldCenter_y: f32,
        0x1C m_kWorldCenter_z: f32,
        /// `m_uiNumRanges` (Xbox PDB).
        0x20 m_uiNumRanges: u32,
        /// `m_pkRanges` (Xbox PDB): `Range` records of 0x10 bytes (`m_fNear`, `m_fFar`,
        /// `m_fWorldNear`, `m_fWorldFar`).
        0x24 m_pkRanges: u32,
    }
}

use crate::units::nimain::nicamera::NiCamera;

/// The size of one `NiRangeLODData::Range` (Xbox PDB).
const RANGE_SIZE: u32 = 0x10;
/// `NiRangeLODData`'s vtable (its RTTI names `NiRangeLODData`).
const NI_RANGE_LOD_DATA_VTABLE: u32 = 0x010a_073c;
/// `00aa09a0` (`thiscall`, no stack word; unnamed in the engine map): the `NiLODData` base
/// constructor; `00aa09c0`: its destructor body.
const LOD_DATA_CONSTRUCTOR: u32 = 0x00aa_09a0;
const LOD_DATA_DESTRUCTOR: u32 = 0x00aa_09c0;
/// `011f426c`: the three words a new centre is copied from.
const CENTER_DEFAULT: u32 = 0x011f_426c;
/// `00aa13e0` (`cdecl`, one word): allocates a block of that size; `00aa1070` and
/// `00aa10f0` (`cdecl`, one word): the Ni allocation and free; `00aa1460` (`cdecl`, (block,
/// size)): the sized delete; CRT `memcpy` (`00ec44d0`).
const ALLOCATE: u32 = 0x00aa_13e0;
const NEW_BLOCK: u32 = 0x00aa_1070;
const FREE_BLOCK: u32 = 0x00aa_10f0;
const SIZED_DELETE: u32 = 0x00aa_1460;
const MEMCPY: u32 = 0x00ec_44d0;
/// `float` -FLT_MAX (`0xff7fffff`) and FLT_MAX (`0x7f7fffff`): `GetLODIndex`'s bounds.
const LOWEST_FLOAT: u32 = 0x0101_5f5c;
const HIGHEST_FLOAT: u32 = 0x010a_0730;
/// `00a724d0` (5 bytes, `NiPSysEmitterCtlrData::LinkObject` in the engine map) and
/// `00a88a40` (5 bytes, `NiInterpolator::LoadBinary`): the base's save and load steps
/// (`thiscall`, the stream).
const BASE_SAVE_STEP: u32 = 0x00a7_24d0;
const BASE_LOAD_STEP: u32 = 0x00a8_8a40;
/// `NiColor::SaveBinary` and `NiColor::LoadBinary` (Xbox PDB; identical code for any three
/// floats, `thiscall` on them, the stream).
const THREE_FLOATS_SAVE: u32 = 0x00a6_9780;
const THREE_FLOATS_LOAD: u32 = 0x00a5_9be0;
/// `NiStream`'s input (+0x24c, read function at +8) and output (+0x250, write function at
/// +0xc); a file's position is at +4.
const STREAM_INPUT: u32 = 0x24c;
const STREAM_OUTPUT: u32 = 0x250;
/// `NiGetViewerString` (Xbox PDB, `cdecl`, one word), `NiPoint3::GetViewerString` (Xbox
/// PDB, `thiscall` on the point, the name) and `NiSprintf` (`cdecl`: buffer, size, format,
/// then the format's arguments).
const GET_VIEWER_STRING: u32 = 0x00a7_d330;
const POINT_GET_VIEWER_STRING: u32 = 0x00a5_9af0;
const NI_SPRINTF: u32 = 0x00a3_05b0;
/// `0096ad30` (`thiscall`, new capacity) and `0096ae90` (`thiscall`, index, pointer to the
/// element): the viewer-string array's set-size and set-at.
const ARRAY_SET_SIZE: u32 = 0x0096_ad30;
const ARRAY_SET_AT: u32 = 0x0096_ae90;
/// `"m_kCenter"`, `"m_kWorldCenter"`, `"range[%d] = %g   %g"`.
const NAME_CENTER: u32 = 0x010a_07fc;
const NAME_WORLD_CENTER: u32 = 0x010a_07ec;
const FORMAT_RANGE: u32 = 0x010a_07d8;

/// `FLD dword [a]`.
fn load(e: &Engine, a: u32) -> f64 {
    x87_load32(e.mem.u32(a))
}

/// A `float` memory operand at `a`.
fn operand(e: &Engine, a: u32) -> f64 {
    x87_m32(e.mem.u32(a))
}

// Translated from 00a97690 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRangeLODData::IsEqual` (Xbox PDB; the base is not asked): the same range count, and
/// every range's near and far equal (`FUCOMPP`, a NaN never equal).
pub fn ni_range_lod_data_is_equal(e: &mut Engine, this: Ptr<NiRangeLODData>, other: Ptr) -> bool {
    let count = e.get(this, NiRangeLODData::m_uiNumRanges);
    if count != e.mem.u32(other.addr() + 0x20) {
        return false;
    }
    let theirs = e.mem.u32(other.addr() + 0x24);
    let mut mine = e.get(this, NiRangeLODData::m_pkRanges);
    let gap = theirs.wrapping_sub(mine);
    let mut their_far = theirs.wrapping_add(4);
    for _ in 0..count {
        if load(e, mine) != load(e, gap.wrapping_add(mine)) {
            return false;
        }
        if load(e, mine.wrapping_add(4)) != load(e, their_far) {
            return false;
        }
        their_far = their_far.wrapping_add(RANGE_SIZE);
        mine = mine.wrapping_add(RANGE_SIZE);
    }
    true
}

/// The constructor's body, also inlined in `Duplicate`, `CreateClone` and the creation
/// function: the `NiLODData` base constructor, the vtable, the centre copied from the three
/// words at `011f426c`, no ranges (the world centre is left as it is).
fn construct(e: &mut Engine, block: u32) {
    e.call(LOD_DATA_CONSTRUCTOR, &args![block]);
    e.mem.set_u32(block, NI_RANGE_LOD_DATA_VTABLE);
    for i in 0..3 {
        let word = e.mem.u32(CENTER_DEFAULT + 4 * i);
        e.mem.set_u32(block + 8 + 4 * i, word);
    }
    e.mem.set_u32(block + 0x20, 0);
    e.mem.set_u32(block + 0x24, 0);
}

// Translated from 00a97700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack word): `NiRangeLODData`'s constructor
/// ([`construct`]). Answers `this`.
pub fn fn_00a97700(e: &mut Engine, this: Ptr<NiRangeLODData>) -> Ptr<NiRangeLODData> {
    construct(e, this.addr());
    this
}

// Translated from 00a97740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRangeLODData::_scalar_deleting_destructor_` (Xbox PDB): the vtable, the ranges freed
/// (`00aa10f0`), the `NiLODData` destructor body (`00aa09c0`), then the sized delete of
/// 0x28 bytes when bit 0 of `flags` is set. Answers `this`.
pub fn ni_range_lod_data_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<NiRangeLODData>,
    flags: u32,
) -> Ptr<NiRangeLODData> {
    let ranges = e.get(this, NiRangeLODData::m_pkRanges);
    e.mem.set_u32(this.addr(), NI_RANGE_LOD_DATA_VTABLE);
    e.call(FREE_BLOCK, &args![ranges]);
    e.call(LOD_DATA_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0x28u32]);
    }
    this
}

// Translated from 00a97780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRangeLODData::UpdateWorldData` (Xbox PDB): the world centre is the node's world
/// transform (`NiAVObject` rotation +0x68 .. +0x88, translation +0x8c, scale +0x98) applied
/// to the centre (scaled first, then each row's three products summed in the exe's operand
/// order, then the translation added), and every range's world near and far are its near
/// and far times the world scale.
pub fn ni_range_lod_data_update_world_data(e: &mut Engine, this: Ptr<NiRangeLODData>, node: Ptr) {
    let me = this.addr();
    let n = node.addr();
    let scale = load(e, n + 0x98);
    let a = x87_store32(x87_mul(scale, operand(e, me + 8)));
    let b = x87_store32(x87_mul(load(e, me + 0xc), scale));
    let c = x87_store32(x87_mul(scale, operand(e, me + 0x10)));
    let (a, b, c) = (x87_load32(a), x87_load32(b), x87_load32(c));
    let r = |e: &Engine, offset: u32| load(e, n + offset);
    let x = x87_add(
        x87_mul(r(e, 0x70), c),
        x87_add(x87_mul(r(e, 0x68), a), x87_mul(r(e, 0x6c), b)),
    );
    let x = x87_store32(x);
    let y = x87_add(
        x87_add(x87_mul(r(e, 0x74), a), x87_mul(r(e, 0x78), b)),
        x87_mul(r(e, 0x7c), c),
    );
    let y = x87_store32(y);
    let z = x87_add(
        x87_mul(c, r(e, 0x88)),
        x87_add(x87_mul(b, r(e, 0x84)), x87_mul(a, r(e, 0x80))),
    );
    let z = x87_store32(z);
    let wx = x87_store32(x87_add(x87_load32(x), operand(e, n + 0x8c)));
    let wy = x87_store32(x87_add(load(e, n + 0x90), x87_m32(y)));
    let wz = x87_store32(x87_add(load(e, n + 0x94), x87_m32(z)));
    e.mem.set_u32(me + 0x14, wx);
    e.mem.set_u32(me + 0x18, wy);
    e.mem.set_u32(me + 0x1c, wz);
    let mut i = 0u32;
    let mut offset = 0u32;
    while i < e.get(this, NiRangeLODData::m_uiNumRanges) {
        let range = e.get(this, NiRangeLODData::m_pkRanges).wrapping_add(offset);
        let v = x87_store32(x87_mul(load(e, range), operand(e, n + 0x98)));
        e.mem.set_u32(range.wrapping_add(8), v);
        let range = e.get(this, NiRangeLODData::m_pkRanges).wrapping_add(offset);
        let v = x87_store32(x87_mul(
            load(e, range.wrapping_add(4)),
            operand(e, n + 0x98),
        ));
        e.mem.set_u32(range.wrapping_add(0xc), v);
        i += 1;
        offset += RANGE_SIZE;
    }
}

// Translated from 00a978a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRangeLODData::SetNumRanges` (Xbox PDB): nothing when `count` is the current count;
/// for 0 the ranges are freed and the count and array cleared; otherwise a new array of
/// `count` ranges (`00aa1070`) gets the first `min(old, count)` (signed) of the old ones
/// (`_memcpy`, when there was an array), the old one is freed and the new one and `count`
/// stored.
pub fn ni_range_lod_data_set_num_ranges(e: &mut Engine, this: Ptr<NiRangeLODData>, count: i32) {
    type D = NiRangeLODData;
    if count as u32 == e.get(this, D::m_uiNumRanges) {
        return;
    }
    if count == 0 {
        let old = e.get(this, D::m_pkRanges);
        e.call(FREE_BLOCK, &args![old]);
        e.set(this, D::m_pkRanges, 0);
        e.set(this, D::m_uiNumRanges, 0);
        return;
    }
    let array = e.call(NEW_BLOCK, &args![(count as u32) << 4]).u32();
    let old = e.get(this, D::m_pkRanges);
    if old != 0 {
        let current = e.get(this, D::m_uiNumRanges) as i32;
        let kept = if current < count { current } else { count };
        e.call(MEMCPY, &args![array, old, (kept as u32) << 4]);
    }
    let old = e.get(this, D::m_pkRanges);
    e.call(FREE_BLOCK, &args![old]);
    e.set(this, D::m_pkRanges, array);
    e.set(this, D::m_uiNumRanges, count as u32);
}

// Translated from 00a97910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRangeLODData::GetLODLevel` (Xbox PDB; `thiscall`, `RET 8`: the camera and a word it
/// does not read): the depth of the world centre along the camera's world direction (the
/// first column of its `NiAVObject` world rotation, +0x68, +0x74, +0x80, each copied through
/// `FLD`/`FSTP`) from its world position (+0x8c), times its LOD adjust, made absolute; the
/// answer is the first range whose world near is at most it and whose world far is above
/// it (ordered), or -1.
pub fn ni_range_lod_data_get_lod_level(
    e: &mut Engine,
    this: Ptr<NiRangeLODData>,
    camera: Ptr<NiCamera>,
    _unused_1: u32,
) -> i32 {
    let me = this.addr();
    let cam = camera.addr();
    let d0 = x87_store32(x87_sub(load(e, me + 0x14), operand(e, cam + 0x8c)));
    let d1 = x87_store32(x87_sub(load(e, me + 0x18), operand(e, cam + 0x90)));
    let d2 = x87_store32(x87_sub(load(e, me + 0x1c), operand(e, cam + 0x94)));
    let r0 = x87_quiet_bits(e.mem.u32(cam + 0x68));
    let r1 = x87_quiet_bits(e.mem.u32(cam + 0x74));
    let r2 = x87_quiet_bits(e.mem.u32(cam + 0x80));
    let sum = x87_add(
        x87_mul(x87_load32(r1), x87_m32(d1)),
        x87_mul(x87_load32(r0), x87_m32(d0)),
    );
    let depth = x87_store32(x87_add(sum, x87_mul(x87_load32(r2), x87_m32(d2))));
    let lod_adjust = e.get(camera, NiCamera::m_fLODAdjust).to_bits();
    let depth = x87_store32(x87_mul(x87_load32(depth), x87_m32(lod_adjust)));
    let depth = x87_store32(x87_load32(depth).abs());
    let depth = x87_load32(x87_store32(x87_load32(depth)));
    let count = e.get(this, NiRangeLODData::m_uiNumRanges);
    if count != 0 {
        let mut range = e.get(this, NiRangeLODData::m_pkRanges).wrapping_add(0xc);
        for index in 0..count {
            // FLD world near; FCOMP depth; TEST AH,0x41; JP: only below or equal goes on.
            // Then FLD world far; FCOMP; TEST AH,0x41; JZ: above (ordered) is the level.
            if load(e, range.wrapping_sub(4)) <= depth && load(e, range) > depth {
                return index as i32;
            }
            range = range.wrapping_add(RANGE_SIZE);
        }
    }
    -1
}

// Translated from 00a979e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRangeLODData::GetLODIndex` (Xbox PDB): `lod` clamped to `0..=count - 1` (signed; -1
/// without ranges answers -1). Then `lod + 1` passes over the ranges: each pass starts
/// with the upper bound FLT_MAX and takes every range whose near lies in `(lower, upper]`
/// (ordered), making both bounds that near and the answer its index; the lower bound
/// starts at -FLT_MAX and carries over between passes. The exe unrolls the scan four
/// ranges at a time while four or more remain; every range is tested in index order
/// either way, with no calls in between.
pub fn ni_range_lod_data_get_lod_index(e: &mut Engine, this: Ptr<NiRangeLODData>, lod: i32) -> i32 {
    let lod = if lod > 0 { lod } else { 0 };
    let count = e.get(this, NiRangeLODData::m_uiNumRanges) as i32;
    let last = count.wrapping_sub(1);
    let lod = if lod < last { lod } else { last };
    let mut lower = x87_quiet_bits(e.mem.u32(LOWEST_FLOAT));
    let mut found = -1i32;
    if lod < 0 {
        return found;
    }
    let highest = load(e, HIGHEST_FLOAT);
    let mut passes = (lod as u32).wrapping_add(1);
    loop {
        let mut upper = x87_store32(highest);
        let mut index = 0i32;
        while index < e.get(this, NiRangeLODData::m_uiNumRanges) as i32 {
            let ranges = e.get(this, NiRangeLODData::m_pkRanges);
            let near_at = ranges.wrapping_add((index as u32).wrapping_mul(RANGE_SIZE));
            let near = load(e, near_at);
            // FLD near; FLD lower; FCOMPP; TEST AH,5; JP: lower below near (ordered) goes on.
            // FLD near; FLD upper; FCOMPP; TEST AH,1; JNZ: upper below near (or unordered)
            // passes it over.
            let upper_value = x87_load32(upper);
            if x87_load32(lower) < near && !(upper_value < near) && !upper_value.is_nan() {
                let ranges = e.get(this, NiRangeLODData::m_pkRanges);
                let at = ranges.wrapping_add((index as u32).wrapping_mul(RANGE_SIZE));
                lower = x87_quiet_bits(e.mem.u32(at));
                found = index;
                upper = x87_quiet_bits(e.mem.u32(at));
            }
            index += 1;
        }
        passes = passes.wrapping_sub(1);
        if passes == 0 {
            break;
        }
    }
    found
}

// Translated from 00a97ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRangeLODData::CopyMembers` (Xbox PDB; `thiscall`, (clone, cloning process), the
/// process not read; no base copy): the centre and world centre copied (six words), the
/// range count set ([`ni_range_lod_data_set_num_ranges`]) and the ranges copied
/// (`_memcpy`).
pub fn ni_range_lod_data_copy_members(
    e: &mut Engine,
    this: Ptr<NiRangeLODData>,
    clone: Ptr<NiRangeLODData>,
    _unused_1: u32,
) {
    for offset in (8u32..0x20).step_by(4) {
        let word = e.mem.u32(this.addr() + offset);
        e.mem.set_u32(clone.addr().wrapping_add(offset), word);
    }
    let count = e.get(this, NiRangeLODData::m_uiNumRanges);
    ni_range_lod_data_set_num_ranges(e, clone, count as i32);
    let count = e.get(this, NiRangeLODData::m_uiNumRanges);
    let from = e.get(this, NiRangeLODData::m_pkRanges);
    let into = e.get(clone, NiRangeLODData::m_pkRanges);
    e.call(MEMCPY, &args![into, from, count << 4]);
}

// Translated from 00a97c00 (PC disassembly, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (a function Ghidra did not create; the creation function the
/// stream registers for `NiRangeLODData`, `0109f090`): a new range LOD data (0x28 bytes
/// from `00aa13e0`, [`construct`]), or null when the allocation fails.
pub fn fn_00a97c00(e: &mut Engine) -> Ptr<NiRangeLODData> {
    let block = e.call(ALLOCATE, &args![0x28u32]).u32();
    if block == 0 {
        return Ptr::new(0);
    }
    construct(e, block);
    Ptr::new(block)
}

// Translated from 00a97c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRangeLODData::Duplicate` (Xbox PDB; `thiscall`, no stack word): a new range LOD data
/// ([`construct`]; null when the allocation fails, written through all the same) with this
/// one's centre (the world centre is not copied), range count and ranges.
pub fn ni_range_lod_data_duplicate(
    e: &mut Engine,
    this: Ptr<NiRangeLODData>,
) -> Ptr<NiRangeLODData> {
    let block = e.call(ALLOCATE, &args![0x28u32]).u32();
    if block != 0 {
        construct(e, block);
    }
    let copy: Ptr<NiRangeLODData> = Ptr::new(block);
    for offset in [8u32, 0xc, 0x10] {
        let word = e.mem.u32(this.addr() + offset);
        e.mem.set_u32(block.wrapping_add(offset), word);
    }
    let count = e.get(this, NiRangeLODData::m_uiNumRanges);
    ni_range_lod_data_set_num_ranges(e, copy, count as i32);
    let count = e.get(this, NiRangeLODData::m_uiNumRanges);
    let from = e.get(this, NiRangeLODData::m_pkRanges);
    let into = e.get(copy, NiRangeLODData::m_pkRanges);
    e.call(MEMCPY, &args![into, from, count << 4]);
    copy
}

// Translated from 00a97ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRangeLODData::CreateClone` (Xbox PDB): a new range LOD data ([`construct`]) given
/// this one's members ([`ni_range_lod_data_copy_members`]); when the allocation fails the
/// member copy runs on null and the answer is null.
pub fn ni_range_lod_data_create_clone(
    e: &mut Engine,
    this: Ptr<NiRangeLODData>,
    cloning_process: u32,
) -> Ptr<NiRangeLODData> {
    let block = e.call(ALLOCATE, &args![0x28u32]).u32();
    if block == 0 {
        ni_range_lod_data_copy_members(e, this, Ptr::new(0), cloning_process);
        return Ptr::new(0);
    }
    construct(e, block);
    ni_range_lod_data_copy_members(e, this, Ptr::new(block), cloning_process);
    Ptr::new(block)
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

// Translated from 00a97d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRangeLODData::SaveBinary` (Xbox PDB): the base's step (`00a724d0`), the centre
/// (`00a69780`), then to the stream's output the range count (the byte-count word is the
/// stream argument's stack word) and each range's near and far (4 bytes each; the
/// function's own local as the byte-count word, the index living in the argument's word).
pub fn ni_range_lod_data_save_binary(e: &mut Engine, this: Ptr<NiRangeLODData>, stream: Ptr) {
    let me = this.addr();
    e.call(BASE_SAVE_STEP, &args![this, stream]);
    e.call(THREE_FLOATS_SAVE, &args![me + 8, stream]);
    e.with_stack(8, |e, slots| {
        let argument = slots.addr();
        let local = argument + 4;
        stream_transfer(e, stream, true, me + 0x20, 4, argument, 4);
        let mut i = 0u32;
        let mut offset = 0u32;
        while i < e.get(this, NiRangeLODData::m_uiNumRanges) {
            let range = e.get(this, NiRangeLODData::m_pkRanges).wrapping_add(offset);
            stream_transfer(e, stream, true, range, 4, local, 4);
            let range = e
                .get(this, NiRangeLODData::m_pkRanges)
                .wrapping_add(offset)
                .wrapping_add(4);
            stream_transfer(e, stream, true, range, 4, local, 4);
            i += 1;
            offset += RANGE_SIZE;
        }
    });
}

// Translated from 00a97e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRangeLODData::LoadBinary` (Xbox PDB): the base's step (`00a88a40`), the centre
/// (`00a59be0`), the range count read into the stream argument's stack word (with a local
/// as the byte-count word), the count set ([`ni_range_lod_data_set_num_ranges`]), then
/// each range's near and far read (4 bytes each, a second local as the byte-count word)
/// while below that count.
pub fn ni_range_lod_data_load_binary(e: &mut Engine, this: Ptr<NiRangeLODData>, stream: Ptr) {
    let me = this.addr();
    e.call(BASE_LOAD_STEP, &args![this, stream]);
    e.call(THREE_FLOATS_LOAD, &args![me + 8, stream]);
    e.with_stack(0xc, |e, slots| {
        let argument = slots.addr();
        let first = argument + 4;
        let second = argument + 8;
        stream_transfer(e, stream, false, argument, 4, first, 4);
        let count = e.mem.u32(argument);
        ni_range_lod_data_set_num_ranges(e, this, count as i32);
        let mut i = 0u32;
        let mut offset = 0u32;
        while i < e.mem.u32(argument) {
            let range = e.get(this, NiRangeLODData::m_pkRanges).wrapping_add(offset);
            stream_transfer(e, stream, false, range, 4, second, 4);
            let range = e
                .get(this, NiRangeLODData::m_pkRanges)
                .wrapping_add(offset)
                .wrapping_add(4);
            stream_transfer(e, stream, false, range, 4, second, 4);
            i += 1;
            offset += RANGE_SIZE;
        }
    });
}

// Translated from 00a97ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiRangeLODData::GetViewerStrings` (Xbox PDB; no base class strings): the class name
/// (the first word of `NiRangeLODData::ms_RTTI`, `011f5ef8`), `m_kCenter` and
/// `m_kWorldCenter` (`NiPoint3::GetViewerString`) appended through the array's set-at with
/// the string kept in a stack local; then for each range a text of 0x80 bytes (`00aa1070`)
/// with `"range[%d] = %g   %g"` of its near and far (`NiSprintf`), stored by the inlined
/// set-at-grow: the array grows when full (`0096ad30`); an index past the count (+0xa)
/// makes it the new count and counts a non-null text in `m_usESize` (+0xc); an index
/// inside it adjusts `m_usESize` when a null slot gets a text or a text slot gets null.
pub fn ni_range_lod_data_get_viewer_strings(
    e: &mut Engine,
    this: Ptr<NiRangeLODData>,
    strings: Ptr,
) {
    let array = strings.addr();
    let grow = |e: &mut Engine, count: u32| {
        if count >= u32::from(e.mem.u16(array + 8)) {
            let grow_by = u32::from(e.mem.u16(array + 0xe));
            e.call(ARRAY_SET_SIZE, &args![strings, grow_by + count]);
        }
    };
    e.with_stack(4, |e, slot| {
        let slot = slot.addr();
        let class_name = e.global::<u32>(NI_RANGE_LOD_DATA_RTTI);
        let text = e.call(GET_VIEWER_STRING, &args![class_name]).u32();
        for step in 0..3 {
            let text = match step {
                0 => text,
                1 => e
                    .call(
                        POINT_GET_VIEWER_STRING,
                        &args![this.addr() + 8, NAME_CENTER],
                    )
                    .u32(),
                _ => e
                    .call(
                        POINT_GET_VIEWER_STRING,
                        &args![this.addr() + 0x14, NAME_WORLD_CENTER],
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
    let mut offset = 0u32;
    while i < e.get(this, NiRangeLODData::m_uiNumRanges) {
        let text = e.call(NEW_BLOCK, &args![0x80u32]).u32();
        let range = e.get(this, NiRangeLODData::m_pkRanges).wrapping_add(offset);
        let far = load(e, range.wrapping_add(4));
        let near = load(e, range);
        e.call(
            NI_SPRINTF,
            &args![text, 0x80u32, FORMAT_RANGE, i, near, far],
        );
        let index = u32::from(e.mem.u16(array + 0xa));
        grow(e, index);
        if index < u32::from(e.mem.u16(array + 0xa)) {
            let slots = e.mem.u32(array + 4);
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
        offset += RANGE_SIZE;
        i += 1;
        let slots = e.mem.u32(array + 4);
        e.mem.set_u32(slots.wrapping_add(4 * index), text);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    type D = NiRangeLODData;
    vec![
        entry!(0x00a976f0, ni_range_lod_data_get_rtti() -> Ptr),
        entry!(0x00a9_7690, ni_range_lod_data_is_equal(Ptr<D>, Ptr) -> bool),
        entry!(0x00a9_7700, fn_00a97700(Ptr<D>) -> Ptr<D>),
        entry!(
            0x00a9_7740,
            ni_range_lod_data_scalar_deleting_destructor(Ptr<D>, u32) -> Ptr<D>
        ),
        entry!(
            0x00a9_7780,
            ni_range_lod_data_update_world_data(Ptr<D>, Ptr)
        ),
        entry!(0x00a9_78a0, ni_range_lod_data_set_num_ranges(Ptr<D>, i32)),
        entry!(
            0x00a9_7910,
            ni_range_lod_data_get_lod_level(Ptr<D>, Ptr<NiCamera>, u32) -> i32
        ),
        entry!(
            0x00a9_79e0,
            ni_range_lod_data_get_lod_index(Ptr<D>, i32) -> i32
        ),
        entry!(
            0x00a9_7ba0,
            ni_range_lod_data_copy_members(Ptr<D>, Ptr<D>, u32)
        ),
        entry!(0x00a9_7c00, fn_00a97c00() -> Ptr<D>),
        entry!(0x00a9_7c50, ni_range_lod_data_duplicate(Ptr<D>) -> Ptr<D>),
        entry!(
            0x00a9_7ce0,
            ni_range_lod_data_create_clone(Ptr<D>, u32) -> Ptr<D>
        ),
        entry!(0x00a9_7d60, ni_range_lod_data_save_binary(Ptr<D>, Ptr)),
        entry!(0x00a9_7e20, ni_range_lod_data_load_binary(Ptr<D>, Ptr)),
        entry!(
            0x00a9_7ef0,
            ni_range_lod_data_get_viewer_strings(Ptr<D>, Ptr)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ni_range_lod_data_get_rtti_returns_the_class_rtti() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00a976f0, &args![]).u32(), NI_RANGE_LOD_DATA_RTTI);
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
        for page in [0x0101_5000, 0x010a_0000, 0x011f_4000, 0x011f_5000] {
            e.map(page, 0x1000);
        }
        e.mem.set_u32(LOWEST_FLOAT, 0xff7f_ffff);
        e.mem.set_u32(HIGHEST_FLOAT, 0x7f7f_ffff);
        for i in 0..3 {
            e.mem.set_f32(CENTER_DEFAULT + 4 * i, 0.5 * (i + 1) as f32);
        }
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
        e.call_log = Some(vec![]);
        e
    }

    /// A range LOD data whose ranges are `(near, far, world near, world far)`.
    fn data(e: &mut Engine, ranges: &[[f32; 4]]) -> u32 {
        let this = e.mem.alloc(0x28);
        let array = e.mem.alloc(RANGE_SIZE * ranges.len().max(1) as u32);
        for (i, r) in ranges.iter().enumerate() {
            for (j, v) in r.iter().enumerate() {
                e.mem
                    .set_f32(array + RANGE_SIZE * i as u32 + 4 * j as u32, *v);
            }
        }
        e.mem.set_u32(this + 0x20, ranges.len() as u32);
        e.mem.set_u32(this + 0x24, array);
        this
    }

    fn ranges(e: &Engine, this: u32) -> Vec<[f32; 4]> {
        let array = e.mem.u32(this + 0x24);
        (0..e.mem.u32(this + 0x20))
            .map(|i| [0, 4, 8, 12].map(|o| e.mem.f32(array + RANGE_SIZE * i + o)))
            .collect()
    }

    #[test]
    fn is_equal_compares_near_and_far() {
        let mut e = engine();
        let a = data(&mut e, &[[1.0, 2.0, 9.0, 9.0], [3.0, 4.0, 0.0, 0.0]]);
        let b = data(&mut e, &[[1.0, 2.0, 0.0, 0.0], [3.0, 4.0, 1.0, 1.0]]);
        assert!(e.call(0x00a9_7690, &args![a, b]).bool());
        let c = data(&mut e, &[[1.0, 2.0, 0.0, 0.0], [3.0, 4.5, 0.0, 0.0]]);
        assert!(!e.call(0x00a9_7690, &args![a, c]).bool());
        let d = data(&mut e, &[[1.5, 2.0, 0.0, 0.0], [3.0, 4.0, 0.0, 0.0]]);
        assert!(!e.call(0x00a9_7690, &args![a, d]).bool());
        let f = data(&mut e, &[[1.0, 2.0, 0.0, 0.0]]);
        assert!(!e.call(0x00a9_7690, &args![a, f]).bool());
        let nan = data(&mut e, &[[f32::NAN, 2.0, 0.0, 0.0]]);
        assert!(!e.call(0x00a9_7690, &args![nan, nan]).bool());
    }

    #[test]
    fn constructor_creation_and_destructor() {
        let mut e = engine();
        e.register(LOD_DATA_DESTRUCTOR, |_, _| Ret::default());
        e.register(SIZED_DELETE, |_, _| Ret::default());
        let this = e.mem.alloc(0x28);
        e.mem.set_u32(this + 0x14, 0x1234);
        assert_eq!(e.call(0x00a9_7700, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), NI_RANGE_LOD_DATA_VTABLE);
        assert_eq!(e.mem.f32(this + 0x10), 1.5);
        assert_eq!(e.mem.u32(this + 0x14), 0x1234);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        let made = e.call(0x00a9_7c00, &args![]).u32();
        assert_eq!(e.mem.f32(made + 8), 0.5);
        assert_eq!(
            calls_to(&e, LOD_DATA_CONSTRUCTOR),
            vec![vec![this], vec![made]]
        );
        e.mem.set_u32(this + 0x24, 0x5555);
        e.call(0x00a9_7740, &args![this, 0u32]);
        assert_eq!(calls_to(&e, FREE_BLOCK), vec![vec![0x5555]]);
        assert_eq!(calls_to(&e, LOD_DATA_DESTRUCTOR), vec![vec![this]]);
        assert!(calls_to(&e, SIZED_DELETE).is_empty());
        e.call(0x00a9_7740, &args![this, 1u32]);
        assert_eq!(calls_to(&e, SIZED_DELETE), vec![vec![this, 0x28]]);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00a9_7c00, &args![]).u32(), 0);
    }

    #[test]
    fn update_world_data_transforms_the_center_and_scales_the_ranges() {
        let mut e = engine();
        let this = data(&mut e, &[[1.0, 2.0, 0.0, 0.0], [3.0, 4.0, 0.0, 0.0]]);
        e.mem.set_f32(this + 8, 1.0);
        e.mem.set_f32(this + 0xc, 2.0);
        e.mem.set_f32(this + 0x10, 3.0);
        let node = e.mem.alloc(0xa0);
        // A rotation swapping x and y, translation (10, 20, 30), scale 2.
        let rotation = [0.0f32, 1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0];
        for (i, v) in rotation.iter().enumerate() {
            e.mem.set_f32(node + 0x68 + 4 * i as u32, *v);
        }
        for (i, v) in [10.0f32, 20.0, 30.0].iter().enumerate() {
            e.mem.set_f32(node + 0x8c + 4 * i as u32, *v);
        }
        e.mem.set_f32(node + 0x98, 2.0);
        e.call(0x00a9_7780, &args![this, node]);
        let world: Vec<f32> = (0..3).map(|i| e.mem.f32(this + 0x14 + 4 * i)).collect();
        assert_eq!(world, vec![14.0, 22.0, 36.0]);
        assert_eq!(
            ranges(&e, this),
            vec![[1.0, 2.0, 2.0, 4.0], [3.0, 4.0, 6.0, 8.0]]
        );
    }

    #[test]
    fn set_num_ranges_keeps_the_common_part() {
        let mut e = engine();
        let this = data(&mut e, &[[1.0, 2.0, 3.0, 4.0], [5.0, 6.0, 7.0, 8.0]]);
        e.call(0x00a9_78a0, &args![this, 1i32]);
        assert_eq!(ranges(&e, this), vec![[1.0, 2.0, 3.0, 4.0]]);
        assert_eq!(calls_to(&e, NEW_BLOCK), vec![vec![0x10]]);
        assert_eq!(calls_to(&e, MEMCPY)[0][2], 0x10);
        e.call(0x00a9_78a0, &args![this, 3i32]);
        assert_eq!(calls_to(&e, MEMCPY)[1][2], 0x10);
        e.call(0x00a9_78a0, &args![this, 3i32]);
        assert_eq!(calls_to(&e, NEW_BLOCK).len(), 2);
        e.call(0x00a9_78a0, &args![this, 0i32]);
        assert_eq!(e.mem.u32(this + 0x20), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0);
    }

    #[test]
    fn lod_level_finds_the_range_holding_the_depth() {
        let mut e = engine();
        let this = data(
            &mut e,
            &[
                [0.0, 0.0, 0.0, 10.0],
                [0.0, 0.0, 10.0, 20.0],
                [0.0, 0.0, 20.0, 40.0],
            ],
        );
        // The world centre 15 in front of a camera looking down x.
        e.mem.set_f32(this + 0x14, 15.0);
        let camera = e.mem.alloc(0x114);
        e.mem.set_f32(camera + 0x68, 1.0);
        e.mem.set_f32(camera + 0x110, 1.0);
        assert_eq!(e.call(0x00a9_7910, &args![this, camera, 0u32]).i32(), 1);
        // Behind the camera: the depth is made absolute.
        e.mem.set_f32(this + 0x14, -25.0);
        assert_eq!(e.call(0x00a9_7910, &args![this, camera, 0u32]).i32(), 2);
        // On a boundary: the near is inclusive, the far exclusive.
        e.mem.set_f32(this + 0x14, 10.0);
        assert_eq!(e.call(0x00a9_7910, &args![this, camera, 0u32]).i32(), 1);
        e.mem.set_f32(this + 0x14, 50.0);
        assert_eq!(e.call(0x00a9_7910, &args![this, camera, 0u32]).i32(), -1);
        // The LOD adjust scales the depth.
        e.mem.set_f32(this + 0x14, 15.0);
        e.mem.set_f32(camera + 0x110, 0.5);
        assert_eq!(e.call(0x00a9_7910, &args![this, camera, 0u32]).i32(), 0);
    }

    #[test]
    fn lod_index_takes_the_first_greater_near_each_pass() {
        let mut e = engine();
        // Five ranges: the unrolled scan covers the first four, the tail the fifth.
        let this = data(
            &mut e,
            &[
                [30.0, 0.0, 0.0, 0.0],
                [10.0, 0.0, 0.0, 0.0],
                [20.0, 0.0, 0.0, 0.0],
                [5.0, 0.0, 0.0, 0.0],
                [40.0, 0.0, 0.0, 0.0],
            ],
        );
        // One pass: the first near above -FLT_MAX is range 0.
        assert_eq!(e.call(0x00a9_79e0, &args![this, 0i32]).i32(), 0);
        // Two passes: then the first near above 30, range 4.
        assert_eq!(e.call(0x00a9_79e0, &args![this, 1i32]).i32(), 4);
        // Three passes: nothing is above 40, the answer stays 4.
        assert_eq!(e.call(0x00a9_79e0, &args![this, 2i32]).i32(), 4);
        // A negative lod is one pass; a large one is clamped to four.
        assert_eq!(e.call(0x00a9_79e0, &args![this, -3i32]).i32(), 0);
        assert_eq!(e.call(0x00a9_79e0, &args![this, 100i32]).i32(), 4);
        // No ranges: -1.
        let empty = data(&mut e, &[]);
        assert_eq!(e.call(0x00a9_79e0, &args![empty, 0i32]).i32(), -1);
    }

    #[test]
    fn copy_members_duplicate_and_clone() {
        let mut e = engine();
        let this = data(&mut e, &[[1.0, 2.0, 3.0, 4.0]]);
        for i in 0..6 {
            e.mem.set_f32(this + 8 + 4 * i, 10.0 + i as f32);
        }
        let clone = e.call(0x00a9_7ce0, &args![this, 0x4444u32]).u32();
        assert_eq!(e.mem.u32(clone), NI_RANGE_LOD_DATA_VTABLE);
        assert_eq!(e.mem.f32(clone + 0x1c), 15.0);
        assert_eq!(ranges(&e, clone), vec![[1.0, 2.0, 3.0, 4.0]]);
        let copy = e.call(0x00a9_7c50, &args![this]).u32();
        assert_eq!(e.mem.f32(copy + 0x10), 12.0);
        // The world centre is not copied.
        assert_eq!(e.mem.f32(copy + 0x14), 0.0);
        assert_eq!(ranges(&e, copy), vec![[1.0, 2.0, 3.0, 4.0]]);
        let target = data(&mut e, &[]);
        e.call(0x00a9_7ba0, &args![this, target, 0u32]);
        assert_eq!(e.mem.f32(target + 0x18), 14.0);
        assert_eq!(ranges(&e, target), vec![[1.0, 2.0, 3.0, 4.0]]);
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
    fn save_and_load_binary_move_the_ranges() {
        let mut e = engine();
        for double in [
            BASE_SAVE_STEP,
            BASE_LOAD_STEP,
            THREE_FLOATS_SAVE,
            THREE_FLOATS_LOAD,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        let words = vec![
            2,
            1.0f32.to_bits(),
            2.0f32.to_bits(),
            3.0f32.to_bits(),
            4.0f32.to_bits(),
        ];
        let (stream, log) = stream(&mut e, words);
        let this = data(&mut e, &[[5.0, 6.0, 0.0, 0.0], [7.0, 8.0, 0.0, 0.0]]);
        e.call(0x00a9_7d60, &args![this, stream]);
        assert_eq!(
            calls_to(&e, THREE_FLOATS_SAVE),
            vec![vec![this + 8, stream]]
        );
        let array = e.mem.u32(this + 0x24);
        let written: Vec<u32> = log.borrow().iter().map(|w| w.0).collect();
        assert_eq!(
            written,
            vec![this + 0x20, array, array + 4, array + 0x10, array + 0x14]
        );
        assert!(log.borrow()[1..].iter().all(|w| w.2 == log.borrow()[1].2));
        assert_ne!(log.borrow()[0].2, log.borrow()[1].2);
        log.borrow_mut().clear();
        let loaded = data(&mut e, &[]);
        e.call(0x00a9_7e20, &args![loaded, stream]);
        assert_eq!(
            calls_to(&e, THREE_FLOATS_LOAD),
            vec![vec![loaded + 8, stream]]
        );
        let got = ranges(&e, loaded);
        assert_eq!(
            (got[0][0], got[0][1], got[1][0], got[1][1]),
            (1.0, 2.0, 3.0, 4.0)
        );
        let log = log.borrow();
        assert_eq!(log.len(), 5);
        assert!(log.iter().all(|r| r.1 == 4 && r.3 == 4));
        assert_ne!(log[0].2, log[1].2);
    }

    #[test]
    fn viewer_strings_list_the_centers_and_ranges() {
        let mut e = engine();
        e.mem.set_u32(NI_RANGE_LOD_DATA_RTTI, 0x010a_0100);
        e.register(GET_VIEWER_STRING, |_, _| 0x5000u32.into_ret());
        e.register(POINT_GET_VIEWER_STRING, |_, _| 0x6000u32.into_ret());
        e.register(NI_SPRINTF, |_, _| Ret::default());
        e.register(ARRAY_SET_SIZE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT, |e, a| {
            let base = e.mem.u32(a[0] + 4);
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(base + 4 * a[1], value);
            e.mem.set_u16(a[0] + 0xa, a[1] as u16 + 1);
            Ret::default()
        });
        let strings = e.mem.alloc(0x10);
        let slots = e.mem.alloc(32);
        e.mem.set_u32(strings + 4, slots);
        e.mem.set_u16(strings + 8, 8);
        let this = data(&mut e, &[[1.0, 2.0, 0.0, 0.0], [3.0, 4.0, 0.0, 0.0]]);
        e.call(0x00a9_7ef0, &args![this, strings]);
        assert_eq!(calls_to(&e, GET_VIEWER_STRING), vec![vec![0x010a_0100]]);
        assert_eq!(
            calls_to(&e, POINT_GET_VIEWER_STRING),
            vec![
                vec![this + 8, NAME_CENTER],
                vec![this + 0x14, NAME_WORLD_CENTER]
            ]
        );
        let texts = calls_to(&e, NI_SPRINTF);
        assert_eq!(
            texts.iter().map(|t| t[1..].to_vec()).collect::<Vec<_>>(),
            vec![
                args![0x80u32, FORMAT_RANGE, 0u32, 1.0f64, 2.0f64],
                args![0x80u32, FORMAT_RANGE, 1u32, 3.0f64, 4.0f64]
            ]
        );
        assert!(calls_to(&e, ARRAY_SET_SIZE).is_empty());
        assert_eq!(e.mem.u16(strings + 0xa), 5);
        assert_eq!(e.mem.u16(strings + 0xc), 2);
        assert_eq!(e.mem.u32(slots + 16), texts[1][0]);
    }
}
