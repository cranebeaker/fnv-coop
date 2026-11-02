//! `SpeedTree/indexedgeometry.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{
    checked_iterator, copy_float, copy_words, ftol, ld, std_vector_int_clear, vector_at,
    vector_len, CVec3, StlVector, INVALID_PARAMETER, MEMCPY, OPERATOR_NEW,
};
use super::run_00b32410::{fn_00b32880, fn_00b32de0};

/// The helpers of the `std::vector` instantiations that live in other units, as this unit calls
/// them: the clear and the (count, fill value) growth of the strip vectors, the pushes of a
/// float and of a byte, the colour push's slow path and the fill of `00ac3450`. (The growths of
/// the strip-length and strip vectors, `00b32880` and `00b32de0`, take their row vector in
/// place: [`fn_00b32880`], [`fn_00b32de0`].)
const CLEAR_STRIP_LENGTHS: u32 = 0x00b3_19e0;
const CLEAR_STRIPS: u32 = 0x00b3_2120;
const CONSTRUCT_SHORT_VECTOR: u32 = 0x00b3_36a0;
const CONSTRUCT_POINTER_VECTOR: u32 = 0x00b1_7db0;
const GROW_INT_VECTOR: u32 = 0x00b3_33e0;
const GROW_SHORT_VECTOR: u32 = 0x00b3_37f0;
const GROW_FLOAT_VECTOR: u32 = 0x0060_b410;
const GROW_BYTE_VECTOR: u32 = 0x00b0_e200;
const PUSH_FLOAT: u32 = 0x0065_78e0;
const PUSH_BYTE: u32 = 0x00b0_dae0;
const PUSH_COLOR_SLOW: u32 = 0x00b3_3240;
const FILL_WORDS: u32 = 0x00ac_3450;
/// `00b30f80`: the address of the coordinate triple of a vertex (geometry, index).
const COORD_POINTER: u32 = 0x00b3_0f80;
/// `00b05000`: the `CVec3` times a matrix (`this` = the vector, then a result slot and the
/// matrix); returns the result slot.
const VEC3_TIMES_MATRIX: u32 = 0x00b0_5000;
/// `00b0d7a0`: a byte flag that decides whether the second texture coordinate is negated.
const FLIP_TEXTURE_V: u32 = 0x00b0_d7a0;
/// `00b0a9a0`: the number of strips of a detail level (geometry, level), in `AX`.
const STRIP_COUNT: u32 = 0x00b0_a9a0;
/// `[011f8d88]`: the base of the table of 0x40-byte wind matrices.
const WIND_MATRICES: u32 = 0x011f_8d88;
/// `0101e568`: the `double` the colour channels are scaled by.
const COLOR_SCALE: u32 = 0x0101_e568;

layout! {
    /// `CIndexedGeometry` (Xbox PDB), 0x190 bytes on the PC: each of its fifteen
    /// `std::vector`s is 8 bytes larger than on the Xbox (0x18 instead of 0x10), so the offsets
    /// after `m_vTriangleCounts` are higher than in the PDB (the scalar fields below `+0x28` are
    /// unchanged).
    pub struct CIndexedGeometry: 0x190 {
        /// `m_bRetainTexCoords` (Xbox PDB).
        0x00 m_bRetainTexCoords: u8,
        /// `m_pWindEngine` (Xbox PDB).
        0x04 m_pWindEngine: Ptr<CWindEngine>,
        /// `m_bVertexWeighting` (Xbox PDB).
        0x08 m_bVertexWeighting: u8,
        /// `m_bManualLighting` (Xbox PDB).
        0x09 m_bManualLighting: u8,
        /// `m_pVertexWindComputed` (Xbox PDB): one byte per vertex.
        0x0C m_pVertexWindComputed: Ptr,
        /// `m_nVertexSize` (Xbox PDB).
        0x10 m_nVertexSize: u16,
        /// `m_bValid` (Xbox PDB).
        0x12 m_bValid: u8,
        /// `m_eWindMethod` (Xbox PDB).
        0x14 m_eWindMethod: u32,
        /// `m_fTrunkWidth` (Xbox PDB).
        0x18 m_fTrunkWidth: f32,
        /// `m_fTrunkLength` (Xbox PDB).
        0x1C m_fTrunkLength: f32,
        /// `m_nNumLodLevels` (Xbox PDB).
        0x20 m_nNumLodLevels: u16,
        /// `m_nVertexCounter` (Xbox PDB).
        0x22 m_nVertexCounter: u16,
        /// `m_nVertexCounterLodLevel` (Xbox PDB).
        0x24 m_nVertexCounterLodLevel: u16,
        /// `m_nStripCounter` (Xbox PDB).
        0x26 m_nStripCounter: u16,
        /// `m_vTriangleCounts` (Xbox PDB `+0x28`): a `std::vector<int>`.
        0x28 m_vTriangleCounts: Inline<StlVector>,
        /// `m_vStripLengths` (Xbox PDB `+0x38`): a vector of `std::vector<unsigned short>`.
        0x40 m_vStripLengths: Inline<StlVector>,
        /// `m_vStrips` (Xbox PDB `+0x48`): a vector of `std::vector<unsigned short *>`.
        0x58 m_vStrips: Inline<StlVector>,
        /// `m_vColors` (Xbox PDB `+0x58`): a `std::vector<unsigned long>`.
        0x70 m_vColors: Inline<StlVector>,
        /// `m_vCoords` (Xbox PDB `+0x68`): a `std::vector<float>`, three per vertex.
        0x88 m_vCoords: Inline<StlVector>,
        /// `m_vOrigCoords` (Xbox PDB `+0x78`).
        0xA0 m_vOrigCoords: Inline<StlVector>,
        /// `m_vNormals` (Xbox PDB `+0x88`).
        0xB8 m_vNormals: Inline<StlVector>,
        /// `m_vBinormals` (Xbox PDB `+0x98`).
        0xD0 m_vBinormals: Inline<StlVector>,
        /// `m_vTangents` (Xbox PDB `+0xA8`).
        0xE8 m_vTangents: Inline<StlVector>,
        /// `m_vTexCoords0` (Xbox PDB `+0xB8`): two floats per vertex.
        0x100 m_vTexCoords0: Inline<StlVector>,
        /// `m_vOrigTexCoords0` (Xbox PDB `+0xC8`).
        0x118 m_vOrigTexCoords0: Inline<StlVector>,
        /// `m_vTexIndices0` (Xbox PDB `+0xD8`): a `std::vector<unsigned char>`.
        0x130 m_vTexIndices0: Inline<StlVector>,
        /// `m_vTexCoords1` (Xbox PDB `+0xE8`).
        0x148 m_vTexCoords1: Inline<StlVector>,
        /// `m_vWindWeights` (Xbox PDB `+0xF8`).
        0x160 m_vWindWeights: Inline<StlVector>,
        /// `m_vWindMatrixIndices` (Xbox PDB `+0x108`): a `std::vector<unsigned char>`.
        0x178 m_vWindMatrixIndices: Inline<StlVector>,
    }

    /// `CWindEngine` (Xbox PDB), 0x44 bytes: only the two fields the wind computation reads.
    pub struct CWindEngine: 0x44 {
        /// `m_nStartingMatrix` (Xbox PDB).
        0x28 m_nStartingMatrix: u32,
        /// `m_nMatrixSpan` (Xbox PDB).
        0x2C m_nMatrixSpan: u32,
    }
}

/// Appends the float at `value` to the `std::vector<float>` at `vector` (`006578e0`, the value
/// is passed by address).
fn push_float(e: &mut Engine, vector: u32, value: u32) {
    e.call(PUSH_FLOAT, &args![vector, value]);
}

/// Copies the three floats at `point` to a local (`FLD`/`FSTP`, so a signalling NaN is quieted)
/// and pushes them one by one onto the `std::vector<float>` at `vector`.
fn push_vec3_copy(e: &mut Engine, vector: u32, point: u32) {
    e.with_stack(0x0C, |e, local| {
        for axis in [0u32, 4, 8] {
            copy_float(e, local.addr() + axis, point + axis);
        }
        for axis in [0u32, 4, 8] {
            push_float(e, vector, local.addr() + axis);
        }
    });
}

/// The vertex counter (`+0x22`), read afresh as the exe does.
fn vertex_counter(e: &Engine, this: Ptr<CIndexedGeometry>) -> u32 {
    u32::from(e.get(this, CIndexedGeometry::m_nVertexCounter))
}

// Translated from 00b2fb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CIndexedGeometry::SetNumLodLevels` (Xbox PDB): empties the strip-length, strip and
/// triangle-count vectors, stores the level count, then grows the strip-length and strip
/// vectors to `count` empty rows (each filled with a fresh, default-constructed row vector
/// passed by value; each row is built in place in the argument area of its growth, `00b32880`
/// and `00b32de0`) and the triangle counts to `count` zeros, and zeroes every triangle count.
pub fn c_indexed_geometry_set_num_lod_levels(
    e: &mut Engine,
    this: Ptr<CIndexedGeometry>,
    count: u16,
) {
    let base = this.addr();
    e.call(CLEAR_STRIP_LENGTHS, &args![base + 0x40]);
    e.call(CLEAR_STRIPS, &args![base + 0x58]);
    std_vector_int_clear(e, Ptr::new(base + 0x28));
    e.set(this, CIndexedGeometry::m_nNumLodLevels, count);
    // 00b2fb77: `SUB ESP,0x18; MOV ECX,ESP`: the row vector is built in the outgoing argument
    // area, which stays in place as `00b32880`'s own `[EBP+0xC]` (its proxy names that area;
    // `00b32880` tidies it and deletes the proxy there, and its `RET 0x1C` frees it).
    e.with_stack(0x18, |e, outgoing| {
        e.call(CONSTRUCT_SHORT_VECTOR, &args![outgoing]); // 00b2fb82
        let lengths = Ptr::new(base + 0x40);
        fn_00b32880(e, lengths, u32::from(count), outgoing.cast()); // 00b2fb9b: the area itself
    });
    // 00b2fba0: `SUB ESP,0x18; MOV ECX,ESP`: the pointer row is built in the outgoing argument
    // area, which stays in place as `00b32de0`'s own `[EBP+0xC]` (its proxy names that area;
    // `00b32de0` tears it down there and its `RET 0x1C` frees it).
    e.with_stack(0x18, |e, outgoing| {
        e.call(CONSTRUCT_POINTER_VECTOR, &args![outgoing]); // 00b2fbab
        let strips = Ptr::new(base + 0x58);
        fn_00b32de0(e, strips, u32::from(count), outgoing.cast()); // 00b2fbc4: the area itself
    });
    e.call(GROW_INT_VECTOR, &args![base + 0x28, count, 0u32]);
    let mut level = 0u32;
    while (level as i32) < i32::from(count) {
        let slot = vector_at(e, base + 0x28, level, 4);
        e.mem.set_u32(slot, 0);
        level += 1;
    }
}

// Translated from 00b2fc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies the transform (a matrix, `00b05000`) to every vertex coordinate triple, in place:
/// the triple is copied (`FLD`/`FSTP`), multiplied and written back with `memcpy`; when the
/// geometry has vertex weighting (`+8`) and a non-empty original-coordinate vector, the
/// original triple of the vertex is transformed the same way. The engine map gives it no name.
pub fn fn_00b2fc50(e: &mut Engine, this: Ptr<CIndexedGeometry>, transform: Ptr) {
    let base = this.addr();
    let vertices = (vector_len(e, base + 0x88, 4) / 3) as u16;
    let mut vertex = 0i32;
    while vertex < i32::from(vertices) {
        let point = e.call(COORD_POINTER, &args![this, vertex as u32]).u32();
        transform_triple(e, point, transform);
        if e.get(this, CIndexedGeometry::m_bVertexWeighting) != 0
            && vector_len(e, base + 0xA0, 4) != 0
        {
            let original = fn_00b31020(e, this, vertex as u32).addr();
            transform_triple(e, original, transform);
        }
        vertex += 1;
    }
}

/// One step of `00b2fc50`: copies the triple at `point`, multiplies it by `transform`
/// (`00b05000`), copies the product the callee returns and writes it back with `memcpy`.
fn transform_triple(e: &mut Engine, point: u32, transform: Ptr) {
    e.with_stack(0x18, |e, frame| {
        let local = frame.addr();
        let slot = frame.addr() + 0x0C;
        for axis in [0u32, 4, 8] {
            copy_float(e, local + axis, point + axis);
        }
        let product = e
            .call(VEC3_TIMES_MATRIX, &args![local, slot, transform])
            .u32();
        copy_words(e, local, product, 3);
        e.call(MEMCPY, &args![point, local, 12u32]);
    });
}

// Translated from 00b2fdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Registers a strip in detail level `level` at the current strip counter (`+0x26`): grows the
/// level's strip-length vector (`00b337f0`) and strip vector (`00b333e0`) to hold the counter
/// plus one entries, stores `length` as the new strip's length and `address` as its index
/// pointer, and adds `length - 2` to the level's triangle count. The engine map gives it no
/// name.
pub fn fn_00b2fdc0(
    e: &mut Engine,
    this: Ptr<CIndexedGeometry>,
    level: u16,
    address: u32,
    length: u16,
) {
    let base = this.addr();
    let level = u32::from(level);
    let row = vector_at(e, base + 0x40, level, 0x18);
    let strip = u32::from(e.get(this, CIndexedGeometry::m_nStripCounter));
    e.call(GROW_SHORT_VECTOR, &args![row, strip + 1, 0u32]);
    let row = vector_at(e, base + 0x40, level, 0x18);
    let strip = u32::from(e.get(this, CIndexedGeometry::m_nStripCounter));
    let slot = vector_at(e, row, strip, 2);
    e.mem.set_u16(slot, length);
    let row = vector_at(e, base + 0x58, level, 0x18);
    let strip = u32::from(e.get(this, CIndexedGeometry::m_nStripCounter));
    e.call(GROW_INT_VECTOR, &args![row, strip + 1, 0u32]);
    let row = vector_at(e, base + 0x58, level, 0x18);
    let strip = u32::from(e.get(this, CIndexedGeometry::m_nStripCounter));
    let slot = vector_at(e, row, strip, 4);
    e.mem.set_u32(slot, address);
    let counts = vector_at(e, base + 0x28, level, 4);
    let total = u32::from(length)
        .wrapping_add(e.mem.u32(counts))
        .wrapping_sub(2);
    e.mem.set_u32(counts, total);
}

// Translated from 00b30020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs the colour at `color` (`00b30060`) and appends the result to the colour vector
/// (`+0x70`, `00b32210`). The engine map gives it no name.
pub fn fn_00b30020(e: &mut Engine, this: Ptr<CIndexedGeometry>, color: Ptr<CVec3>) {
    let packed = fn_00b30060(e, this, color);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), packed);
        fn_00b32210(e, Ptr::new(this.addr() + 0x70), slot);
    });
}

// Translated from 00b30060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Packs a colour of three floats into `0xFFBBGGRR`: each channel is multiplied by the
/// `double` at `0101e568` and truncated (`_ftol2`); red is the low byte, green is shifted by 8
/// and blue by 16, and the alpha byte is added as `0xFF000000`. `this` is not read. The engine
/// map gives it no name.
pub fn fn_00b30060(e: &mut Engine, _this: Ptr<CIndexedGeometry>, color: Ptr<CVec3>) -> u32 {
    let scale: f64 = e.global(COLOR_SCALE);
    let red = ld(e, color.addr()) * scale;
    let red = ftol(e, red);
    let green = ld(e, color.addr() + 4) * scale;
    let green = ftol(e, green);
    let blue = ld(e, color.addr() + 8) * scale;
    let blue = ftol(e, blue);
    (red as u32)
        .wrapping_add((green as u32) << 8)
        .wrapping_add((blue as u32) << 16)
        .wrapping_add(0xff00_0000)
}

// Translated from 00b300c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a vertex position to the coordinate vector (`+0x88`) as three floats (copied
/// through locals), and, when the geometry has vertex weighting (`+8`) and wind method 1
/// (`+0x14`), to the original-coordinate vector (`+0xA0`) too. The engine map gives it no name.
pub fn fn_00b300c0(e: &mut Engine, this: Ptr<CIndexedGeometry>, point: Ptr<CVec3>) {
    let base = this.addr();
    e.with_stack(0x0C, |e, local| {
        for axis in [0u32, 4, 8] {
            copy_float(e, local.addr() + axis, point.addr() + axis);
        }
        for axis in [0u32, 4, 8] {
            push_float(e, base + 0x88, local.addr() + axis);
        }
        if e.get(this, CIndexedGeometry::m_bVertexWeighting) != 0
            && e.get(this, CIndexedGeometry::m_eWindMethod) == 1
        {
            for axis in [0u32, 4, 8] {
                push_float(e, base + 0xA0, local.addr() + axis);
            }
        }
    });
}

// Translated from 00b30190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a normal to the normal vector (`+0xB8`): the three floats are pushed by their
/// address in `normal` directly. The engine map gives it no name.
pub fn fn_00b30190(e: &mut Engine, this: Ptr<CIndexedGeometry>, normal: Ptr<CVec3>) {
    for axis in [0u32, 4, 8] {
        push_float(e, this.addr() + 0xB8, normal.addr() + axis);
    }
}

// Translated from 00b301f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the texture coordinates (two floats at `coords`) of the vertex being added (the
/// vertex counter, `+0x22`). When the counter equals half the texture-coordinate vector's
/// size, the vectors are grown first (two floats; the texture-index and original
/// coordinates too when `texture_index` is not negative and the geometry retains its texture
/// coordinates, `+0`). The texture index (low byte) and the original coordinates are stored
/// under the same conditions; the current coordinates are always stored, with the second one
/// negated when `00b0d7a0` says so. The engine map gives it no name.
pub fn fn_00b301f0(e: &mut Engine, this: Ptr<CIndexedGeometry>, coords: Ptr, texture_index: i16) {
    let base = this.addr();
    if vertex_counter(e, this) == vector_len(e, base + 0x100, 4) >> 1 {
        let size = vector_len(e, base + 0x100, 4).wrapping_add(2);
        e.call(GROW_FLOAT_VECTOR, &args![base + 0x100, size, 0.0f32]);
        if texture_index > -1 && e.get(this, CIndexedGeometry::m_bRetainTexCoords) != 0 {
            let size = vector_len(e, base + 0x130, 1).wrapping_add(1);
            e.call(GROW_BYTE_VECTOR, &args![base + 0x130, size, 0u32]);
            let size = vector_len(e, base + 0x118, 4).wrapping_add(2);
            e.call(GROW_FLOAT_VECTOR, &args![base + 0x118, size, 0.0f32]);
        }
    }
    if texture_index > -1 && e.get(this, CIndexedGeometry::m_bRetainTexCoords) != 0 {
        let vertex = vertex_counter(e, this);
        let slot = vector_at(e, base + 0x130, vertex, 1);
        e.mem.set_u8(slot, texture_index as u8);
        let vertex = vertex_counter(e, this);
        let slot = vector_at(e, base + 0x118, vertex << 1, 4);
        copy_float(e, slot, coords.addr());
        let vertex = vertex_counter(e, this);
        let slot = vector_at(e, base + 0x118, vertex.wrapping_mul(2).wrapping_add(1), 4);
        copy_float(e, slot, coords.addr() + 4);
    }
    let vertex = vertex_counter(e, this);
    let slot = vector_at(e, base + 0x100, vertex << 1, 4);
    copy_float(e, slot, coords.addr());
    let flip = e.call(FLIP_TEXTURE_V, &args![]).bool();
    let second = x87_quiet_bits(e.mem.u32(coords.addr() + 4));
    let second = if flip { second ^ 0x8000_0000 } else { second };
    let vertex = vertex_counter(e, this);
    let slot = vector_at(e, base + 0x100, vertex.wrapping_mul(2).wrapping_add(1), 4);
    e.mem.set_u32(slot, second);
}

// Translated from 00b304f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CIndexedGeometry::AddVertexTexCoord1` (Xbox PDB): appends the two floats at `coords` to
/// the second texture-coordinate vector (`+0x148`); the second one is negated when
/// `00b0d7a0` says so.
pub fn c_indexed_geometry_add_vertex_tex_coord1(
    e: &mut Engine,
    this: Ptr<CIndexedGeometry>,
    coords: Ptr,
) {
    let vector = this.addr() + 0x148;
    push_float(e, vector, coords.addr());
    if e.call(FLIP_TEXTURE_V, &args![]).bool() {
        let negated = x87_quiet_bits(e.mem.u32(coords.addr() + 4)) ^ 0x8000_0000;
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), negated);
            push_float(e, vector, slot.addr());
        });
    } else {
        push_float(e, vector, coords.addr() + 4);
    }
}

// Translated from 00b30560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends the wind data of a vertex: the wind weight vector (`+0x160`) gets `1 - weight`, the
/// wind matrix index vector (`+0x178`) gets `group % span + start` (as a byte), where `start` and
/// `span` are the wind engine's starting matrix and matrix span (`+0x28`, `+0x2C` of
/// `+4`). A zero span is a division fault in the exe (it panics here). The engine map gives it
/// no name.
pub fn fn_00b30560(e: &mut Engine, this: Ptr<CIndexedGeometry>, weight: f32, group: u8) {
    let base = this.addr();
    let weight = (1.0 - f64::from(weight)) as f32;
    let wind = e.get(this, CIndexedGeometry::m_pWindEngine);
    let start = e.get(wind, CWindEngine::m_nStartingMatrix);
    let span = e.get(wind, CWindEngine::m_nMatrixSpan);
    let group = (u32::from(group) % span).wrapping_add(start) as u8;
    e.with_stack(8, |e, slots| {
        e.mem.set_f32(slots.addr(), weight);
        push_float(e, base + 0x160, slots.addr());
        e.mem.set_u8(slots.addr() + 4, group);
        e.call(PUSH_BYTE, &args![base + 0x178, slots.addr() + 4]);
    });
}

// Translated from 00b305f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a tangent to the tangent vector (`+0xE8`): the three floats are copied through
/// locals and pushed. The engine map gives it no name.
pub fn fn_00b305f0(e: &mut Engine, this: Ptr<CIndexedGeometry>, tangent: Ptr<CVec3>) {
    push_vec3_copy(e, this.addr() + 0xE8, tangent.addr());
}

// Translated from 00b30660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends a binormal to the binormal vector (`+0xD0`): the three floats are copied through
/// locals and pushed. The engine map gives it no name.
pub fn fn_00b30660(e: &mut Engine, this: Ptr<CIndexedGeometry>, binormal: Ptr<CVec3>) {
    push_vec3_copy(e, this.addr() + 0xD0, binormal.addr());
}

// Translated from 00b306d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CIndexedGeometry::ResetStripCounter` (Xbox PDB): makes `level` the current vertex-counter
/// detail level (`+0x24`), zeroes the strip counter (`+0x26`) and the level's triangle count.
pub fn c_indexed_geometry_reset_strip_counter(
    e: &mut Engine,
    this: Ptr<CIndexedGeometry>,
    level: u16,
) {
    e.set(this, CIndexedGeometry::m_nVertexCounterLodLevel, level);
    e.set(this, CIndexedGeometry::m_nStripCounter, 0u16);
    let slot = vector_at(e, this.addr() + 0x28, u32::from(level), 4);
    e.mem.set_u32(slot, 0);
}

// Translated from 00b30730 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the first strip pointer of detail level `level` (the first element of the
/// level's row of the strip vector, `+0x58`), or null when that row is empty. The engine map
/// gives it no name.
pub fn fn_00b30730(e: &mut Engine, this: Ptr<CIndexedGeometry>, level: u16) -> Ptr {
    let base = this.addr();
    let row = vector_at(e, base + 0x58, u32::from(level), 0x18);
    if vector_len(e, row, 4) == 0 {
        return Ptr::NULL;
    }
    let row = vector_at(e, base + 0x58, u32::from(level), 0x18);
    // Never taken: the row was just found non-empty, and this is the checked `[0]` of it.
    if vector_len(e, row, 4) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    Ptr::new(e.mem.u32(row + 0x0C))
}

// Translated from 00b30800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CIndexedGeometry::ComputeWindEffect` (Xbox PDB): does nothing (false) once the geometry is
/// valid (`+0x12`). Otherwise allocates the per-vertex "wind computed" bytes if needed, clears
/// them, and for every vertex of every strip of detail level `level` that was not done yet
/// moves its coordinate triple from the original one: the original is multiplied by the wind
/// matrix of the vertex (`00b05000`, the matrix table at `[011f8d88]` indexed by the vertex's
/// wind matrix index) and blended back with the vertex's wind weight,
/// `(rotated - original) * weight + original`, per axis. Finally marks the geometry valid and
/// returns true.
pub fn c_indexed_geometry_compute_wind_effect(
    e: &mut Engine,
    this: Ptr<CIndexedGeometry>,
    level: u16,
) -> bool {
    let base = this.addr();
    if e.get(this, CIndexedGeometry::m_bValid) != 0 {
        return false;
    }
    let vertices = (vector_len(e, base + 0x88, 4) / 3) as u16;
    if e.get(this, CIndexedGeometry::m_pVertexWindComputed)
        .is_null()
    {
        let memory = e.call(OPERATOR_NEW, &args![u32::from(vertices)]).u32();
        e.set(
            this,
            CIndexedGeometry::m_pVertexWindComputed,
            Ptr::new(memory),
        );
    }
    let mut vertex = 0i32;
    while vertex < i32::from(vertices) {
        let flags = e.get(this, CIndexedGeometry::m_pVertexWindComputed).addr();
        e.mem.set_u8(flags + vertex as u32, 0);
        vertex += 1;
    }
    // Scratch for the rotated point (0x00), the callee's result slot (0x0C) and the blended
    // point (0x18).
    e.with_stack(0x24, |e, frame| {
        let point = frame.addr();
        let slot = frame.addr() + 0x0C;
        let blended = frame.addr() + 0x18;
        let mut strip = 0i32;
        while strip < i32::from(e.call(STRIP_COUNT, &args![this, u32::from(level)]).u16()) {
            let mut index = 0i32;
            loop {
                let row = vector_at(e, base + 0x40, u32::from(level), 0x18);
                let length_slot = vector_at(e, row, strip as u32, 2);
                if index >= i32::from(e.mem.u16(length_slot)) {
                    break;
                }
                let row = vector_at(e, base + 0x58, u32::from(level), 0x18);
                let strip_slot = vector_at(e, row, strip as u32, 4);
                let indices = e.mem.u32(strip_slot);
                let vertex = u32::from(e.mem.u16(indices + (index as u32) * 2));
                let flags = e.get(this, CIndexedGeometry::m_pVertexWindComputed).addr();
                if e.mem.u8(flags + vertex) == 0 {
                    let weight_slot = vector_at(e, base + 0x160, vertex, 4);
                    let weight = f64::from(f32::from_bits(x87_quiet_bits(e.mem.u32(weight_slot))));
                    if vector_len(e, base + 0xA0, 4) == 0 {
                        e.call(INVALID_PARAMETER, &args![]);
                    }
                    let original = e.mem.u32(base + 0xA0 + 0x0C).wrapping_add(vertex * 12);
                    for axis in [0u32, 4, 8] {
                        copy_float(e, point + axis, original + axis);
                    }
                    let matrix_slot = vector_at(e, base + 0x178, vertex, 1);
                    let matrix_index = u32::from(e.mem.u8(matrix_slot));
                    let matrix = (matrix_index << 6).wrapping_add(e.global::<u32>(WIND_MATRICES));
                    let rotated = e.call(VEC3_TIMES_MATRIX, &args![point, slot, matrix]).u32();
                    copy_words(e, point, rotated, 3);
                    for axis in [8u32, 4, 0] {
                        let from =
                            f64::from(f32::from_bits(x87_quiet_bits(e.mem.u32(original + axis))));
                        let moved = ld(e, point + axis);
                        e.mem
                            .set_f32(blended + axis, ((moved - from) * weight + from) as f32);
                    }
                    if vector_len(e, base + 0x88, 4) == 0 {
                        e.call(INVALID_PARAMETER, &args![]);
                    }
                    let destination = e.mem.u32(base + 0x88 + 0x0C).wrapping_add(vertex * 12);
                    e.call(MEMCPY, &args![destination, blended, 12u32]);
                    let flags = e.get(this, CIndexedGeometry::m_pVertexWindComputed).addr();
                    e.mem.set_u8(flags + vertex, 1);
                }
                index += 1;
            }
            strip += 1;
        }
    });
    e.set(this, CIndexedGeometry::m_bValid, 1u8);
    true
}

// Translated from 00b31020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the original-coordinate triple of vertex `index` (`+0xA0` first + `index *
/// 12`); an empty vector takes the invalid-parameter path. The engine map gives it no name.
pub fn fn_00b31020(e: &mut Engine, this: Ptr<CIndexedGeometry>, index: u32) -> Ptr {
    let vector = this.addr() + 0xA0;
    if vector_len(e, vector, 4) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    Ptr::new(
        e.mem
            .u32(vector + 0x0C)
            .wrapping_add(index.wrapping_mul(3).wrapping_mul(4)),
    )
}

// Translated from 00b32210 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<unsigned long>::push_back` as the colour vector uses it: with spare capacity
/// the word at `value` is stored at the end (`00ac3450`) and the end advanced by 4; otherwise the
/// checked insert `00b33240` at the end is used.
pub fn fn_00b32210(e: &mut Engine, this: Ptr<StlVector>, value: Ptr) {
    let base = this.addr();
    let size = vector_len(e, base, 4);
    let capacity = if e.mem.u32(base + 0x0C) == 0 {
        0
    } else {
        let bytes = e.mem.u32(base + 0x14).wrapping_sub(e.mem.u32(base + 0x0C)) as i32;
        (bytes >> 2) as u32
    };
    if size < capacity {
        let last = e.mem.u32(base + 0x10);
        e.call(FILL_WORDS, &args![last, 1u32, value]);
        e.mem.set_u32(base + 0x10, last.wrapping_add(4));
    } else {
        let last = e.mem.u32(base + 0x10);
        let proxy = checked_iterator(e, base, last);
        e.with_stack(8, |e, out| {
            e.call(PUSH_COLOR_SLOW, &args![this, out, proxy, last, value]);
        });
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b2_fb30,
            c_indexed_geometry_set_num_lod_levels(Ptr<CIndexedGeometry>, u16)
        ),
        entry!(0x00b2_fc50, fn_00b2fc50(Ptr<CIndexedGeometry>, Ptr)),
        entry!(
            0x00b2_fdc0,
            fn_00b2fdc0(Ptr<CIndexedGeometry>, u16, u32, u16)
        ),
        entry!(0x00b3_0020, fn_00b30020(Ptr<CIndexedGeometry>, Ptr<CVec3>)),
        entry!(
            0x00b3_0060,
            fn_00b30060(Ptr<CIndexedGeometry>, Ptr<CVec3>) -> u32
        ),
        entry!(0x00b3_00c0, fn_00b300c0(Ptr<CIndexedGeometry>, Ptr<CVec3>)),
        entry!(0x00b3_0190, fn_00b30190(Ptr<CIndexedGeometry>, Ptr<CVec3>)),
        entry!(0x00b3_01f0, fn_00b301f0(Ptr<CIndexedGeometry>, Ptr, i16)),
        entry!(
            0x00b3_04f0,
            c_indexed_geometry_add_vertex_tex_coord1(Ptr<CIndexedGeometry>, Ptr)
        ),
        entry!(0x00b3_0560, fn_00b30560(Ptr<CIndexedGeometry>, f32, u8)),
        entry!(0x00b3_05f0, fn_00b305f0(Ptr<CIndexedGeometry>, Ptr<CVec3>)),
        entry!(0x00b3_0660, fn_00b30660(Ptr<CIndexedGeometry>, Ptr<CVec3>)),
        entry!(
            0x00b3_06d0,
            c_indexed_geometry_reset_strip_counter(Ptr<CIndexedGeometry>, u16)
        ),
        entry!(0x00b3_0730, fn_00b30730(Ptr<CIndexedGeometry>, u16) -> Ptr),
        entry!(
            0x00b3_0800,
            c_indexed_geometry_compute_wind_effect(Ptr<CIndexedGeometry>, u16) -> bool
        ),
        entry!(0x00b3_1020, fn_00b31020(Ptr<CIndexedGeometry>, u32) -> Ptr),
        entry!(0x00b3_2210, fn_00b32210(Ptr<StlVector>, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::super::branch::OPERATOR_DELETE;
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A zeroed geometry object.
    fn geometry(e: &mut Engine) -> Ptr<CIndexedGeometry> {
        e.new_object()
    }

    /// Makes the `std::vector` at `vector` hold `count` zeroed elements of `size` bytes in
    /// fresh memory and returns the address of the elements.
    fn fill(e: &mut Engine, vector: u32, size: u32, count: u32) -> u32 {
        let data = e.mem.alloc((size * count).max(1));
        e.mem.set_u32(vector, 1);
        e.mem.set_u32(vector + 0x0C, data);
        e.mem.set_u32(vector + 0x10, data + size * count);
        e.mem.set_u32(vector + 0x14, data + size * count);
        data
    }

    /// A double for a `(vector, count, fill value)` growth: the vector gets `count` elements of
    /// `size` bytes.
    fn grow(e: &mut Engine, addr: u32, size: u32) {
        e.register_double(addr, move |e, a| {
            fill(e, a[0], size, a[1]);
            Ret::default()
        });
    }

    /// A double that records (vector, value pointer, the value read through it).
    fn record(e: &mut Engine, addr: u32, bytes: bool) -> Rc<RefCell<Vec<(u32, u32, u32)>>> {
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(addr, move |e, a| {
            let value = if bytes {
                u32::from(e.mem.u8(a[1]))
            } else {
                e.mem.u32(a[1])
            };
            log.borrow_mut().push((a[0], a[1], value));
            Ret::default()
        });
        seen
    }

    fn triple(e: &mut Engine, values: [f32; 3]) -> u32 {
        let at = e.mem.alloc(12);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
        at
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// What the strip rows' growth (`00b32de0`, run directly on the row built in its argument
    /// area) calls on the way: the insertion of the row's copies (`00b34450`: the vector, the
    /// end iterator's proxy and position, the count, the row's address) and the tidy of the row
    /// (`00b17f50`), whose proxy then goes to `operator delete`.
    const STRIPS_INSERT: u32 = 0x00b3_4450;
    const STRIPS_ROW_TIDY: u32 = 0x00b1_7f50;

    /// `00b32880`'s helpers for the strip-length rows (run directly on the row built in its
    /// argument area): the insert of `count` copies (`00b33bd0`) and the row's tidy
    /// (`00b34d90`), whose proxy then goes to `operator delete`.
    const STRIP_LENGTHS_INSERT: u32 = 0x00b3_3bd0;
    const STRIP_LENGTHS_ROW_TIDY: u32 = 0x00b3_4d90;

    #[test]
    fn set_num_lod_levels_resets_the_vectors() {
        let mut e = Engine::new();
        let g = geometry(&mut e);
        let base = g.addr();
        for addr in [CLEAR_STRIP_LENGTHS, CLEAR_STRIPS, 0x00b3_35c0] {
            e.register_double(addr, |_, _| Ret::default());
        }
        // Both growths (00b32880, 00b32de0) run: their inserts, the rows' tidies and the
        // deletes of the rows' proxies are doubled.
        for addr in [
            STRIP_LENGTHS_INSERT,
            STRIP_LENGTHS_ROW_TIDY,
            STRIPS_ROW_TIDY,
            OPERATOR_DELETE,
        ] {
            e.register_double(addr, |_, _| Ret::default());
        }
        e.register_double(CONSTRUCT_SHORT_VECTOR, |e, a| {
            for i in 0..6 {
                e.mem.set_u32(a[0] + 4 * i, 0x100 + i);
            }
            Ret::default()
        });
        e.register_double(CONSTRUCT_POINTER_VECTOR, |e, a| {
            for i in 0..6 {
                e.mem.set_u32(a[0] + 4 * i, 0x200 + i);
            }
            Ret::default()
        });
        // The row's six words as the insertion of its copies finds them.
        let rows = Rc::new(RefCell::new(Vec::new()));
        let sink = rows.clone();
        e.register_double(STRIPS_INSERT, move |e, a| {
            let row: Vec<u32> = (0..6).map(|i| e.mem.u32(a[4] + 4 * i)).collect();
            sink.borrow_mut().push(row);
            Ret::default()
        });
        e.register_double(GROW_INT_VECTOR, |e, a| {
            let data = fill(e, a[0], 4, a[1]);
            for i in 0..a[1] {
                e.mem.set_u32(data + 4 * i, 7);
            }
            Ret::default()
        });
        e.call_log = Some(Vec::new());
        e.call(0x00b2_fb30, &args![g, 3u16]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(g, CIndexedGeometry::m_nNumLodLevels), 3);
        // The three length rows are inserted from the row built in place (its words
        // 0x100..0x105), which is then tidied.
        let row = calls_to(&log, CONSTRUCT_SHORT_VECTOR)[0][0];
        assert_eq!(
            calls_to(&log, STRIP_LENGTHS_INSERT),
            vec![vec![base + 0x40, 0, 0, 3, row]]
        );
        assert_eq!(calls_to(&log, STRIP_LENGTHS_ROW_TIDY), vec![vec![row]]);
        // The empty strip vector gets three copies of the row at its end (proxy 0, position 0).
        let strips = calls_to(&log, STRIPS_INSERT);
        assert_eq!(strips.len(), 1);
        assert_eq!(strips[0][..4], [base + 0x58, 0, 0, 3]);
        assert_eq!(
            *rows.borrow(),
            vec![vec![0x200, 0x201, 0x202, 0x203, 0x204, 0x205]]
        );
        // Each row's proxy (its first word) is deleted: the length row's, then the strip row's.
        assert_eq!(
            calls_to(&log, OPERATOR_DELETE),
            vec![vec![0x100], vec![0x200]]
        );
        assert_eq!(calls_to(&log, CLEAR_STRIP_LENGTHS), vec![vec![base + 0x40]]);
        assert_eq!(calls_to(&log, CLEAR_STRIPS), vec![vec![base + 0x58]]);
        let counts = e.mem.u32(base + 0x28 + 0x0C);
        for i in 0..3 {
            assert_eq!(e.mem.u32(counts + 4 * i), 0);
        }
    }

    #[test]
    fn set_num_lod_levels_grows_the_strips_from_the_row_in_its_argument_area() {
        // 00b2fba0..00b2fbc4: the pointer row is built (00b17db0) in 00b32de0's argument area,
        // so its proxy names the vector 00b32de0 copies from (00b32e92) and tears down
        // (00b32ff3, 00b33012). With freed blocks held back (the freed-block check), a row
        // built in an area freed before 00b32de0 ran left its proxy naming that freed area, not
        // the vector the copies were made from.
        let mut e = Engine::new();
        e.mem.enable_freed_check(false);
        let g = geometry(&mut e);
        let base = g.addr();
        for addr in [
            CLEAR_STRIP_LENGTHS,
            CLEAR_STRIPS,
            0x00b3_35c0,
            CONSTRUCT_SHORT_VECTOR,
            STRIP_LENGTHS_INSERT,
            STRIP_LENGTHS_ROW_TIDY,
            STRIPS_ROW_TIDY,
            OPERATOR_DELETE,
        ] {
            e.register_double(addr, |_, _| Ret::default());
        }
        grow(&mut e, GROW_INT_VECTOR, 4);
        // 00b17db0: the row gets a proxy whose first word names the row.
        let built = Rc::new(RefCell::new(Vec::new()));
        let sink = built.clone();
        e.register_double(CONSTRUCT_POINTER_VECTOR, move |e, a| {
            let proxy = e.mem.alloc(4);
            e.mem.set_u32(proxy, a[0]);
            e.mem.set_u32(a[0], proxy);
            sink.borrow_mut().push((a[0], proxy));
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        // 00b34450: the copies are made from the row, whose checked iterators find their
        // container through its proxy: the row's address and the vector its proxy names.
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(STRIPS_INSERT, move |e, a| {
            let proxy = e.mem.u32(a[4]);
            sink.borrow_mut().push((a[4], e.mem.u32(proxy)));
            Ret::default()
        });
        e.call_log = Some(Vec::new());
        e.call(0x00b2_fb30, &args![g, 3u16]);
        let log = e.call_log.take().unwrap();
        let (row, proxy) = built.borrow()[0];
        assert_eq!(*seen.borrow(), vec![(row, row)]);
        assert_eq!(
            calls_to(&log, STRIPS_INSERT)[0][..4],
            [base + 0x58, 0, 0, 3]
        );
        // 00b32ff3..00b33012: the same row is tidied and its proxy deleted (after the length
        // row's proxy, which 00b32880 deletes first).
        assert_eq!(calls_to(&log, STRIPS_ROW_TIDY), vec![vec![row]]);
        let deletes = calls_to(&log, OPERATOR_DELETE);
        assert_eq!(deletes.len(), 2);
        assert_eq!(deletes[1], vec![proxy]);
    }

    /// 00b2fb77..00b2fb9b: the row vector is built (00b336a0) in 00b32880's argument area, so
    /// the proxy it allocates names the vector the resize hands its insert and then tidies,
    /// deleting that proxy. Built in a local and passed as words, the row the resize worked on
    /// was a copy elsewhere and its proxy named the caller's ended local. The freed check is on
    /// because under it stack locals are separate blocks: without it the copy's area reuses the
    /// ended local's address, so the stale proxy would name the copy by chance.
    #[test]
    fn set_num_lod_levels_grows_the_lengths_from_the_row_its_proxy_names() {
        let mut e = Engine::new();
        e.mem.enable_freed_check(false);
        let g = geometry(&mut e);
        let base = g.addr();
        for addr in [CLEAR_STRIP_LENGTHS, CLEAR_STRIPS, 0x00b3_35c0] {
            e.register_double(addr, |_, _| Ret::default());
        }
        for addr in [
            CONSTRUCT_POINTER_VECTOR,
            STRIPS_INSERT,
            STRIPS_ROW_TIDY,
            STRIP_LENGTHS_ROW_TIDY,
            OPERATOR_DELETE,
        ] {
            e.register_double(addr, |_, _| Ret::default());
        }
        e.register(0x0044_d670, |e, a| e.mem.alloc(4 * a[0]).into_ret());
        grow(&mut e, GROW_INT_VECTOR, 4);
        // The insert sees the value it is handed and the vector its proxy names.
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(STRIP_LENGTHS_INSERT, move |e, a| {
            let proxy = e.mem.u32(a[4]);
            sink.borrow_mut().push((a[4], proxy, e.mem.u32(proxy)));
            Ret::default()
        });
        e.call_log = Some(Vec::new());
        e.call(0x00b2_fb30, &args![g, 2u16]);
        let log = e.call_log.take().unwrap();
        let row = calls_to(&log, CONSTRUCT_SHORT_VECTOR)[0][0];
        let (value, proxy, named) = seen.borrow()[0];
        assert_eq!((value, named), (row, row));
        assert_eq!(
            calls_to(&log, STRIP_LENGTHS_INSERT)[0][..4],
            [base + 0x40, 0, 0, 2]
        );
        assert_eq!(calls_to(&log, STRIP_LENGTHS_ROW_TIDY), vec![vec![row]]);
        assert_eq!(calls_to(&log, OPERATOR_DELETE)[0], vec![proxy]);
    }

    #[test]
    fn transform_moves_the_vertices_and_their_originals() {
        for weighting in [1u8, 0] {
            let mut e = Engine::new();
            let g = geometry(&mut e);
            let base = g.addr();
            let coords = fill(&mut e, base + 0x88, 4, 6);
            let originals = fill(&mut e, base + 0xA0, 4, 6);
            for i in 0..6 {
                e.mem.set_f32(coords + 4 * i, i as f32);
                e.mem.set_f32(originals + 4 * i, 100.0 + i as f32);
            }
            // The first coordinate of vertex 1 is a signalling NaN.
            e.mem.set_u32(coords + 12, 0x7fa0_0000);
            e.mem.set_u8(base + 8, weighting);
            e.register_double(COORD_POINTER, |e, a| {
                let first = e.mem.u32(a[0] + 0x88 + 0x0C);
                (first + 12 * a[1]).into_ret()
            });
            e.register_double(VEC3_TIMES_MATRIX, |e, a| {
                // result = input with y raised by the low word of the matrix
                let x = e.mem.u32(a[0]);
                let y = e.mem.f32(a[0] + 4) + a[2] as f32;
                let z = e.mem.u32(a[0] + 8);
                e.mem.set_u32(a[1], x);
                e.mem.set_f32(a[1] + 4, y);
                e.mem.set_u32(a[1] + 8, z);
                a[1].into_ret()
            });
            e.call(0x00b2_fc50, &args![g, 3u32]);
            assert_eq!(e.mem.f32(coords + 4), 4.0);
            assert_eq!(e.mem.u32(coords + 12), 0x7fe0_0000);
            assert_eq!(e.mem.f32(coords + 16), 7.0);
            assert_eq!(e.mem.f32(coords + 8), 2.0);
            if weighting == 1 {
                assert_eq!(e.mem.f32(originals + 4), 104.0);
                assert_eq!(e.mem.f32(originals + 16), 107.0);
                assert_eq!(e.mem.f32(originals), 100.0);
            } else {
                assert_eq!(e.mem.f32(originals + 4), 101.0);
            }
        }
    }

    #[test]
    fn registers_a_strip_in_a_level() {
        let mut e = Engine::new();
        let g = geometry(&mut e);
        let base = g.addr();
        let lengths = fill(&mut e, base + 0x40, 0x18, 2);
        let strips = fill(&mut e, base + 0x58, 0x18, 2);
        let counts = fill(&mut e, base + 0x28, 4, 2);
        e.mem.set_u32(counts + 4, 6);
        e.set(g, CIndexedGeometry::m_nStripCounter, 1u16);
        grow(&mut e, GROW_SHORT_VECTOR, 2);
        grow(&mut e, GROW_INT_VECTOR, 4);
        e.call(0x00b2_fdc0, &args![g, 1u16, 0x1234u32, 7u16]);
        let short_row = e.mem.u32(lengths + 0x18 + 0x0C);
        assert_eq!(e.mem.u16(short_row + 2), 7);
        let int_row = e.mem.u32(strips + 0x18 + 0x0C);
        assert_eq!(e.mem.u32(int_row + 4), 0x1234);
        assert_eq!(e.mem.u32(counts + 4), 11);
        assert_eq!(e.mem.u32(counts), 0);
    }

    #[test]
    fn packs_a_colour() {
        let mut e = Engine::new();
        let g = geometry(&mut e);
        e.map(COLOR_SCALE, 8);
        e.set_global(COLOR_SCALE, 255.0f64);
        let color = triple(&mut e, [1.0, 0.5, 0.0]);
        assert_eq!(e.call(0x00b3_0060, &args![g, color]).u32(), 0xff00_7fff);
        let color = triple(&mut e, [0.0, 0.0, 1.0]);
        assert_eq!(e.call(0x00b3_0060, &args![g, color]).u32(), 0xffff_0000);
    }

    #[test]
    fn appends_a_packed_colour() {
        let mut e = Engine::new();
        let g = geometry(&mut e);
        let base = g.addr();
        e.map(COLOR_SCALE, 8);
        e.set_global(COLOR_SCALE, 255.0f64);
        let data = fill(&mut e, base + 0x70, 4, 0);
        e.mem.set_u32(base + 0x70 + 0x14, data + 16);
        e.register_double(FILL_WORDS, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        let color = triple(&mut e, [1.0, 1.0, 1.0]);
        e.call(0x00b3_0020, &args![g, color]);
        assert_eq!(e.mem.u32(data), 0xffff_ffff);
        assert_eq!(e.mem.u32(base + 0x70 + 0x10), data + 4);
    }

    #[test]
    fn colour_push_falls_back_to_the_checked_insert() {
        let mut e = Engine::new();
        let vector = e.mem.alloc(0x18);
        let data = fill(&mut e, vector, 4, 2);
        let value = e.mem.alloc(4);
        e.call_log = Some(Vec::new());
        e.register_double(PUSH_COLOR_SLOW, |_, _| Ret::default());
        e.call(0x00b3_2210, &args![vector, value]);
        let log = e.call_log.take().unwrap();
        let calls = calls_to(&log, PUSH_COLOR_SLOW);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0][0], vector);
        assert_eq!(&calls[0][2..], &[1, data + 8, value]);
        // A vector with no storage at all has capacity zero: the slow path too.
        let empty = e.mem.alloc(0x18);
        e.call(0x00b3_2210, &args![empty, value]);
    }

    #[test]
    fn appends_a_position_with_its_original() {
        for method in [1u32, 0] {
            let mut e = Engine::new();
            let g = geometry(&mut e);
            let base = g.addr();
            let seen = record(&mut e, PUSH_FLOAT, false);
            e.mem.set_u8(base + 8, 1);
            e.set(g, CIndexedGeometry::m_eWindMethod, method);
            let point = triple(&mut e, [1.0, 2.0, 3.0]);
            e.mem.set_u32(point + 4, 0x7fa0_0001);
            e.call(0x00b3_00c0, &args![g, point]);
            let seen = seen.borrow();
            let values = [0x3f80_0000, 0x7fe0_0001, 0x4040_0000];
            assert_eq!(seen.len(), if method == 1 { 6 } else { 3 });
            for (i, (vector, _, value)) in seen.iter().enumerate() {
                assert_eq!(*vector, base + if i < 3 { 0x88 } else { 0xA0 });
                assert_eq!(*value, values[i % 3]);
            }
        }
    }

    #[test]
    fn appends_a_normal_by_address() {
        let mut e = Engine::new();
        let g = geometry(&mut e);
        let seen = record(&mut e, PUSH_FLOAT, false);
        let normal = triple(&mut e, [0.0, 1.0, 0.0]);
        e.call(0x00b3_0190, &args![g, normal]);
        let seen = seen.borrow();
        let pointers: Vec<u32> = seen.iter().map(|s| s.1).collect();
        assert_eq!(pointers, vec![normal, normal + 4, normal + 8]);
        assert!(seen.iter().all(|s| s.0 == g.addr() + 0xB8));
    }

    #[test]
    fn appends_tangents_and_binormals_through_copies() {
        for (address, offset) in [(0x00b3_05f0u32, 0xE8u32), (0x00b3_0660, 0xD0)] {
            let mut e = Engine::new();
            let g = geometry(&mut e);
            let seen = record(&mut e, PUSH_FLOAT, false);
            let vector = triple(&mut e, [4.0, 5.0, 6.0]);
            e.mem.set_u32(vector + 8, 0xffa0_0000);
            e.call(address, &args![g, vector]);
            let seen = seen.borrow();
            let values: Vec<u32> = seen.iter().map(|s| s.2).collect();
            assert_eq!(values, vec![0x4080_0000, 0x40a0_0000, 0xffe0_0000]);
            assert!(seen
                .iter()
                .all(|s| s.0 == g.addr() + offset && s.1 != vector));
        }
    }

    #[test]
    fn stores_texture_coordinates_of_a_vertex() {
        let mut e = Engine::new();
        let g = geometry(&mut e);
        let base = g.addr();
        grow(&mut e, GROW_FLOAT_VECTOR, 4);
        grow(&mut e, GROW_BYTE_VECTOR, 1);
        e.register_double(FLIP_TEXTURE_V, |_, _| 1u32.into_ret());
        e.mem.set_u8(base, 1);
        let coords = triple(&mut e, [0.25, 0.75, 0.0]);
        e.call(0x00b3_01f0, &args![g, coords, 5u16]);
        let current = e.mem.u32(base + 0x100 + 0x0C);
        assert_eq!(e.mem.f32(current), 0.25);
        assert_eq!(e.mem.f32(current + 4), -0.75);
        let originals = e.mem.u32(base + 0x118 + 0x0C);
        assert_eq!(e.mem.f32(originals), 0.25);
        assert_eq!(e.mem.f32(originals + 4), 0.75);
        assert_eq!(e.mem.u8(e.mem.u32(base + 0x130 + 0x0C)), 5);
    }

    #[test]
    fn stores_texture_coordinates_without_an_index() {
        let mut e = Engine::new();
        let g = geometry(&mut e);
        let base = g.addr();
        grow(&mut e, GROW_FLOAT_VECTOR, 4);
        e.register_double(FLIP_TEXTURE_V, |_, _| 0u32.into_ret());
        e.mem.set_u8(base, 1);
        let coords = triple(&mut e, [0.25, 0.75, 0.0]);
        e.mem.set_u32(coords + 4, 0x7fa0_0000);
        e.call_log = Some(Vec::new());
        e.call(0x00b3_01f0, &args![g, coords, -1i16]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, GROW_BYTE_VECTOR).is_empty());
        let current = e.mem.u32(base + 0x100 + 0x0C);
        assert_eq!(e.mem.f32(current), 0.25);
        assert_eq!(e.mem.u32(current + 4), 0x7fe0_0000);
        assert_eq!(e.mem.u32(base + 0x118 + 0x0C), 0);
    }

    #[test]
    fn adds_the_second_texture_coordinate_set() {
        for flip in [0u32, 1] {
            let mut e = Engine::new();
            let g = geometry(&mut e);
            let seen = record(&mut e, PUSH_FLOAT, false);
            e.register_double(FLIP_TEXTURE_V, move |_, _| flip.into_ret());
            let coords = triple(&mut e, [0.5, 2.0, 0.0]);
            e.call(0x00b3_04f0, &args![g, coords]);
            let seen = seen.borrow();
            assert_eq!(seen.len(), 2);
            assert!(seen.iter().all(|s| s.0 == g.addr() + 0x148));
            assert_eq!((seen[0].1, seen[0].2), (coords, 0x3f00_0000));
            if flip == 1 {
                assert_ne!(seen[1].1, coords + 4);
                assert_eq!(seen[1].2, 0xc000_0000);
            } else {
                assert_eq!((seen[1].1, seen[1].2), (coords + 4, 0x4000_0000));
            }
        }
    }

    #[test]
    fn adds_the_wind_data_of_a_vertex() {
        let mut e = Engine::new();
        let g = geometry(&mut e);
        let wind = e.mem.alloc(0x44);
        e.mem.set_u32(wind + 0x28, 3);
        e.mem.set_u32(wind + 0x2C, 4);
        e.set(g, CIndexedGeometry::m_pWindEngine, Ptr::new(wind));
        let floats = record(&mut e, PUSH_FLOAT, false);
        let bytes = record(&mut e, PUSH_BYTE, true);
        e.call(0x00b3_0560, &args![g, 0.25f32, 6u8]);
        assert_eq!(
            floats
                .borrow()
                .iter()
                .map(|s| (s.0, s.2))
                .collect::<Vec<_>>(),
            vec![(g.addr() + 0x160, 0.75f32.to_bits())]
        );
        assert_eq!(
            bytes
                .borrow()
                .iter()
                .map(|s| (s.0, s.2))
                .collect::<Vec<_>>(),
            vec![(g.addr() + 0x178, 5)]
        );
    }

    #[test]
    fn resets_the_strip_counter() {
        let mut e = Engine::new();
        let g = geometry(&mut e);
        let base = g.addr();
        let counts = fill(&mut e, base + 0x28, 4, 3);
        e.mem.set_u32(counts + 8, 9);
        e.set(g, CIndexedGeometry::m_nStripCounter, 4u16);
        e.call(0x00b3_06d0, &args![g, 2u16]);
        assert_eq!(e.get(g, CIndexedGeometry::m_nVertexCounterLodLevel), 2);
        assert_eq!(e.get(g, CIndexedGeometry::m_nStripCounter), 0);
        assert_eq!(e.mem.u32(counts + 8), 0);
    }

    #[test]
    fn finds_the_first_strip_pointer_of_a_level() {
        let mut e = Engine::new();
        let g = geometry(&mut e);
        let base = g.addr();
        let rows = fill(&mut e, base + 0x58, 0x18, 2);
        let pointers = fill(&mut e, rows + 0x18, 4, 2);
        assert_eq!(e.call(0x00b3_0730, &args![g, 1u16]).u32(), pointers);
        assert_eq!(e.call(0x00b3_0730, &args![g, 0u16]).u32(), 0);
    }

    #[test]
    fn blends_the_wind_into_the_coordinates() {
        let mut e = Engine::new();
        let g = geometry(&mut e);
        let base = g.addr();
        e.register_double(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register_double(STRIP_COUNT, |_, _| 1u32.into_ret());
        let matrices = Rc::new(RefCell::new(Vec::new()));
        let log = matrices.clone();
        e.register_double(VEC3_TIMES_MATRIX, move |e, a| {
            log.borrow_mut().push(a[2]);
            for axis in 0..3 {
                let value = e.mem.f32(a[0] + 4 * axis) + 10.0 * (axis + 1) as f32;
                e.mem.set_f32(a[1] + 4 * axis, value);
            }
            a[1].into_ret()
        });
        e.map(WIND_MATRICES, 4);
        e.set_global(WIND_MATRICES, 0x6000_0000u32);
        let coords = fill(&mut e, base + 0x88, 4, 6);
        let originals = fill(&mut e, base + 0xA0, 4, 6);
        for i in 0..6 {
            e.mem.set_f32(coords + 4 * i, 50.0);
            e.mem.set_f32(originals + 4 * i, i as f32);
        }
        let weights = fill(&mut e, base + 0x160, 4, 2);
        e.mem.set_f32(weights, 0.5);
        e.mem.set_f32(weights + 4, 1.0);
        let indices = fill(&mut e, base + 0x178, 1, 2);
        e.mem.set_u8(indices + 1, 1);
        let lengths = fill(&mut e, base + 0x40, 0x18, 1);
        let row = fill(&mut e, lengths, 2, 1);
        e.mem.set_u16(row, 2);
        let strips = fill(&mut e, base + 0x58, 0x18, 1);
        let pointers = fill(&mut e, strips, 4, 1);
        let vertices = e.mem.alloc(4);
        e.mem.set_u16(vertices, 0);
        e.mem.set_u16(vertices + 2, 1);
        e.mem.set_u32(pointers, vertices);
        assert!(e.call(0x00b3_0800, &args![g, 0u16]).bool());
        // vertex 0: original (0, 1, 2), weight 0.5; vertex 1: original (3, 4, 5), weight 1.
        let flags = e.get(g, CIndexedGeometry::m_pVertexWindComputed).addr();
        assert_eq!((e.mem.u8(flags), e.mem.u8(flags + 1)), (1, 1));
        let moved: Vec<f32> = (0..6).map(|i| e.mem.f32(coords + 4 * i)).collect();
        assert_eq!(moved, vec![5.0, 11.0, 17.0, 13.0, 24.0, 35.0]);
        assert_eq!(*matrices.borrow(), vec![0x6000_0000, 0x6000_0040]);
        assert_eq!(e.get(g, CIndexedGeometry::m_bValid), 1);
        assert!(!e.call(0x00b3_0800, &args![g, 0u16]).bool());
    }

    #[test]
    fn finds_an_original_coordinate_triple() {
        let mut e = Engine::new();
        let g = geometry(&mut e);
        let originals = fill(&mut e, g.addr() + 0xA0, 4, 6);
        assert_eq!(e.call(0x00b3_1020, &args![g, 1u32]).u32(), originals + 12);
    }
}
