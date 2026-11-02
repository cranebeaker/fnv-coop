//! `SpeedTree/run_00b23f10` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The first part is the MSVC `std::vector` and sort code of the frond engine's instantiations
//! (the engine map names none of it): the range copies and fills for the element types
//! `SFrondVertex` (0x38 bytes), `SFrondGuide` (0x38), the guide-vector and float-vector elements
//! (0x18), `SFrondTexture` (0x2C) and `CVec` (0xC), the vertex and guide vector assignments and
//! erases, the guide sort's partition, median, heap and insertion routines, and the vector
//! swaps. The rest are the `CLeafGeometry` member functions and the lighting engine's
//! constructor that the linker placed after them (the layouts follow the Xbox PDB; the PC
//! offsets are the same for those classes).
//!
//! The PC build's `std::vector` is 0x18 bytes ([`StlVector`]: the checked-iterator proxy at
//! `+0`, the first, last and end pointers at `+0x0C`, `+0x10`, `+0x14`). The allocator and the
//! two flag bytes of the exe's `std::vector` code are passed on the stack by the compiler; the
//! flag bytes are left uninitialised in the exe and are passed here as zeros. Functions that
//! carry an `FS:[0]` exception frame (the handlers that unwind a half-built vector are not
//! translated) are marked `Translation incomplete`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{
    checked_iterator, copy_float, copy_words, scaled_trig, vector_at, CRotTransform, CVec3,
    StlVector, ANGLE_DIVISOR, COS, MEMCPY, OPERATOR_DELETE, OPERATOR_NEW, SIN,
};
use super::frondengine::{SFrondGuide, SFrondTexture};

/// The constructors, destructors and helpers of other units this file calls (all by address):
/// the guide copy constructor (`00b1ff50`, `thiscall`, one argument), the guide-vector copy
/// constructor (`00b231f0`), the texture copy constructor (`00b22f80`), the float-vector copy
/// constructor (`00b22ff0`) and the guide-vector range assign (`00b23760`), all in
/// `run_00b1ff50`.
const GUIDE_COPY_CONSTRUCT: u32 = 0x00b1_ff50;
const GUIDE_VECTOR_COPY_CONSTRUCT: u32 = 0x00b2_31f0;
const TEXTURE_COPY_CONSTRUCT: u32 = 0x00b2_2f80;
const FLOAT_VECTOR_COPY_CONSTRUCT: u32 = 0x00b2_2ff0;
const GUIDE_ASSIGN_RANGE: u32 = 0x00b2_3760;
/// `memmove_s` (`00ec7ce3`, `cdecl`: destination, size, source, count).
const MEMMOVE_S: u32 = 0x00ec_7ce3;
/// `[010173f0]`: the `npos` the string assignment takes as its count, and the string
/// `assign(source, offset, count)` (`0044a580`, `thiscall`).
const STRING_NPOS: u32 = 0x0101_73f0;
const STRING_ASSIGN: u32 = 0x0044_a580;
/// `_Buy(this, count)` of the vertex vector (`00b1eb00`, `RET 4`, a flag in `AL`); the
/// vertex vector's element tidy (`00b1e970`, the destructor of the guide's vertex vector,
/// `thiscall`); `operator delete` of the word at `this` (`00b39460`).
const VERTEX_VECTOR_BUY: u32 = 0x00b1_eb00;
const VERTEX_VECTOR_TIDY: u32 = 0x00b1_e970;
const PROXY_RELEASE: u32 = 0x00b3_9460;
/// Whether two checked iterators differ (`00b16830`, `thiscall`: this and the other iterator,
/// both addresses of a proxy and a position).
const ITERATORS_DIFFER: u32 = 0x00b1_6830;
/// The guide vector's clear (`00b1de20`, `fastcall`), its range destroy (`00b1ebc0`, `thiscall`:
/// this, first, last), its tidy (`00b1ec70`) and its default constructor (`00b1d8a0`).
const GUIDE_VECTOR_ERASE_ALL: u32 = 0x00b1_de20;
const GUIDE_DESTROY_RANGE: u32 = 0x00b1_ebc0;
const GUIDE_VECTOR_TIDY: u32 = 0x00b1_ec70;
const GUIDE_VECTOR_DEFAULT_CONSTRUCT: u32 = 0x00b1_d8a0;
/// The guide vector's `operator=` (`00b24f40`, this file) and the float vector's helpers: its
/// default constructor (`00449fd0`), tidy (`0065f240`), `operator=` (`00b1d5d0`, `RET 4`) and
/// `push_back` (`006578e0`: this and the address of the value).
const GUIDE_VECTOR_ASSIGN: u32 = 0x00b2_4f40;
const FLOAT_VECTOR_CONSTRUCT: u32 = 0x0044_9fd0;
const FLOAT_VECTOR_TIDY: u32 = 0x0065_f240;
const FLOAT_VECTOR_ASSIGN: u32 = 0x00b1_d5d0;
const FLOAT_VECTOR_PUSH: u32 = 0x0065_78e0;
/// The CRT `atan2` (`00ec9760`; `cdecl`, two doubles, the result in `ST0`); `[010a4d90]` is
/// the double the angles are scaled by (shared with `branch.rs`).
const ATAN2: u32 = 0x00ec_9760;
/// The destructor the linker shares between the empty classes: it does nothing (`00aa7830`).
const TRIVIAL_DESTRUCTOR: u32 = 0x00aa_7830;
/// `CRotTransform::CRotTransform` (the identity, `00b12af0`), the matrix product
/// `this * other` (`00b12940`: this, a result slot, the other matrix) and the product of a
/// vector with a rotation (`00b12890`: this, a result slot, the matrix; the slot comes back).
const CROT_TRANSFORM_CONSTRUCT: u32 = 0x00b1_2af0;
const CROT_TRANSFORM_MULTIPLY: u32 = 0x00b1_2940;
const VEC3_TIMES_ROTATION: u32 = 0x00b1_2890;
/// The product of a vector with a wind matrix (`00b05000`: this, a result slot, the matrix; the
/// slot comes back), the distance of two vectors (`00b04d40`, in `ST0`), and a byte flag that
/// decides whether the second texture coordinate is negated (`00b0d7a0`).
const VEC3_TIMES_MATRIX: u32 = 0x00b0_5000;
const VEC3_DISTANCE: u32 = 0x00b0_4d40;
const FLIP_TEXTURE_V: u32 = 0x00b0_d7a0;
/// `[01012054]`: the float the texture coordinate is multiplied by when flipped; `[01016248]`
/// (0.5) and `[0102295c]` (a float read like it) the exe loads when it builds the billboard
/// vectors; the base pointer of the wind matrices (`[011f8d88]`); the two floats of the lighting
/// engine's first material block (`01030ff0`, `0101df44`).
const FLIP_SIGN: u32 = 0x0101_2054;
const HALF_FLOAT: u32 = 0x0101_6248;
const NEGATIVE_HALF_FLOAT: u32 = 0x0102_295c;
const WIND_MATRICES: u32 = 0x011f_8d88;
const MATERIAL_FIRST: u32 = 0x0103_0ff0;
const MATERIAL_SECOND: u32 = 0x0101_df44;
/// The statics `00b27080` fills: the guard word (`011f8d04`; bits 0 to 4 say which block has
/// run), the four direction vectors (`011f8cf8`, `011f8cec`, `011f8ce0`, `011f8cd4`), the table
/// of four vectors (`011f8ca4`) and the results (`011f8d08`, four vectors, 12 bytes apart).
const BILLBOARD_GUARD: u32 = 0x011f_8d04;
const BILLBOARD_VECTOR_A: u32 = 0x011f_8cf8;
const BILLBOARD_VECTOR_B: u32 = 0x011f_8cec;
const BILLBOARD_VECTOR_C: u32 = 0x011f_8ce0;
const BILLBOARD_VECTOR_D: u32 = 0x011f_8cd4;
const BILLBOARD_TABLE: u32 = 0x011f_8ca4;
const BILLBOARD_RESULTS: u32 = 0x011f_8d08;
/// The `SLeaf` base of `SLodGeometry`: its constructor (`00b02c90`) and destructor
/// (`00b02d40`), both `thiscall` with `this` only.
const LEAF_BASE_CONSTRUCT: u32 = 0x00b0_2c90;
const LEAF_BASE_DESTRUCT: u32 = 0x00b0_2d40;
/// The `SLodGeometry` constructor (`00b274a0`), destructor (`00b274d0`) and vector deleting
/// destructor (`00b277b0`), this file's, and the CRT's `eh vector constructor iterator`
/// (`00ec782f`: array, size, count, constructor, destructor) and `eh vector destructor
/// iterator` (`00ec5fce`: array, size, count, destructor).
const LOD_CONSTRUCTOR: u32 = 0x00b2_74a0;
const LOD_DESTRUCTOR: u32 = 0x00b2_74d0;
const LOD_VECTOR_DELETING_DESTRUCTOR: u32 = 0x00b2_77b0;
const EH_VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
const VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// The extents object helpers of `run_00b3c910`: the constructor (`00b3c930`), the expansion by a
/// point (`00b3caa0`: this, a result slot, the point; the slot comes back) and the merge
/// (`00b3c9a0`: this, a result slot, the other extents; the slot comes back).
const EXTENTS_CONSTRUCT: u32 = 0x00b3_c930;
const EXTENTS_EXPAND: u32 = 0x00b3_caa0;
const EXTENTS_MERGE: u32 = 0x00b3_c9a0;

layout! {
    /// `CLeafGeometry::SLodGeometry` (Xbox PDB), 0x44 bytes (the PC offsets are the same): the
    /// `CSpeedTreeRT::SGeometry::SLeaf` base up to `+0x3C`, then `m_bValid` and the original
    /// centre coordinates.
    pub struct SLodGeometry: 0x44 {
        /// `m_bIsActive` (Xbox PDB, `SLeaf`).
        0x00 m_bIsActive: u8,
        /// `m_fAlphaTestValue` (Xbox PDB, `SLeaf`).
        0x04 m_fAlphaTestValue: f32,
        /// `m_nDiscreteLodLevel` (Xbox PDB, `SLeaf`).
        0x08 m_nDiscreteLodLevel: i32,
        /// `m_usLeafCount` (Xbox PDB, `SLeaf`).
        0x0C m_usLeafCount: u16,
        /// `m_pLeafMapIndices` (Xbox PDB, `SLeaf`): one byte per leaf.
        0x10 m_pLeafMapIndices: Ptr,
        /// `m_pLeafClusterIndices` (Xbox PDB, `SLeaf`): one byte per leaf.
        0x14 m_pLeafClusterIndices: Ptr,
        /// `m_pCenterCoords` (Xbox PDB, `SLeaf`): three floats per leaf.
        0x18 m_pCenterCoords: Ptr,
        /// `m_pLeafMapTexCoords` (Xbox PDB, `SLeaf`).
        0x1C m_pLeafMapTexCoords: Ptr,
        /// `m_pLeafMapCoords` (Xbox PDB, `SLeaf`).
        0x20 m_pLeafMapCoords: Ptr,
        /// `m_pColors` (Xbox PDB, `SLeaf`).
        0x24 m_pColors: Ptr,
        /// `m_pNormals` (Xbox PDB, `SLeaf`).
        0x28 m_pNormals: Ptr,
        /// `m_pBinormals` (Xbox PDB, `SLeaf`).
        0x2C m_pBinormals: Ptr,
        /// `m_pTangents` (Xbox PDB, `SLeaf`).
        0x30 m_pTangents: Ptr,
        /// `m_pWindWeights` (Xbox PDB, `SLeaf`).
        0x34 m_pWindWeights: Ptr,
        /// `m_pWindMatrixIndices` (Xbox PDB, `SLeaf`).
        0x38 m_pWindMatrixIndices: Ptr,
        /// `m_bValid` (Xbox PDB).
        0x3C m_bValid: u8,
        /// `m_pOrigCenterCoords` (Xbox PDB).
        0x40 m_pOrigCenterCoords: Ptr,
    }

    /// `CLeafGeometry` (Xbox PDB), 0x30 bytes (the PC offsets are the same).
    pub struct CLeafGeometry: 0x30 {
        /// `m_bManualLighting` (Xbox PDB).
        0x00 m_bManualLighting: u8,
        /// `m_bVertexWeighting` (Xbox PDB).
        0x01 m_bVertexWeighting: u8,
        /// `m_pWindEngine` (Xbox PDB).
        0x04 m_pWindEngine: Ptr,
        /// `m_usNumRockingGroups` (Xbox PDB).
        0x08 m_usNumRockingGroups: u16,
        /// `m_pTimeOffsets` (Xbox PDB).
        0x0C m_pTimeOffsets: Ptr,
        /// `m_pLeafVertexes` (Xbox PDB): one block pointer per LOD.
        0x10 m_pLeafVertexes: Ptr,
        /// `m_pLeafTexCoords` (Xbox PDB).
        0x14 m_pLeafTexCoords: Ptr,
        /// `m_pVertexProgramTable` (Xbox PDB).
        0x18 m_pVertexProgramTable: Ptr,
        /// `m_usNumTextures` (Xbox PDB).
        0x1C m_usNumTextures: u16,
        /// `m_pLeafDimensions` (Xbox PDB): a `CVec3` per texture.
        0x20 m_pLeafDimensions: Ptr,
        /// `m_pLeafOrigins` (Xbox PDB): a `CVec3` per texture.
        0x24 m_pLeafOrigins: Ptr,
        /// `m_usNumLods` (Xbox PDB).
        0x28 m_usNumLods: u16,
        /// `m_pLods` (Xbox PDB): the `SLodGeometry` array.
        0x2C m_pLods: Ptr,
    }
}

/// The capacity, in elements, of the `std::vector` at `vector` whose elements are `size` bytes:
/// zero without storage, else `(end - first) / size` as the compiler's signed `IDIV`.
fn vector_capacity(e: &Engine, vector: u32, size: i32) -> u32 {
    if e.mem.u32(vector + 0x0C) == 0 {
        0
    } else {
        (e.mem
            .u32(vector + 0x14)
            .wrapping_sub(e.mem.u32(vector + 0x0C)) as i32
            / size) as u32
    }
}

/// The flags the compiler-generated deleting destructors test after destroying an element: the
/// exe computes `0 & 1`, so the element's own `operator delete` that follows never runs.
const NO_DELETE: u32 = 0;

/// Whether the "delete the element too" bit of `flags` is set.
fn delete_flag(flags: u32) -> bool {
    flags & 1 != 0
}

/// Copies `words` consecutive words from `source` to `target` (`MOVSD`).
fn copy_range_words(e: &mut Engine, target: u32, source: u32, words: u32) {
    for word in 0..words {
        let value = e.mem.u32(source + 4 * word);
        e.mem.set_u32(target + 4 * word, value);
    }
}

/// The element count `(last - first) / size` as the compiler's signed `IDIV`.
fn element_count(first: u32, last: u32, size: i32) -> u32 {
    (last.wrapping_sub(first) as i32 / size) as u32
}

// Translated from 00b23f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy-constructs the textures `first..last` at `destination` (`cdecl`, four arguments, the
/// last the allocator, which is not read): `00b25ba0` with two flag bytes the exe leaves
/// uninitialised (zeros here). Returns what it returns, the end of the destination.
pub fn fn_00b23f10(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00b25ba0(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00b23f60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b23f60 - the FS:[0] exception frame is not translated.
/// Fills `count` vertices (0x38 bytes, 14 words) from `destination` on with copies of the one at
/// `value` (`cdecl`, six arguments: the allocator and two flag bytes are not read); a null
/// destination slot is skipped as in the placement `new`.
pub fn fn_00b23f60(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let mut target = destination;
    let mut left = count;
    while left != 0 {
        if target != 0 {
            copy_range_words(e, target, value, 14);
        }
        target = target.wrapping_add(0x38);
        left = left.wrapping_sub(1);
    }
}

// Translated from 00b24030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy-constructs the guides `first..last` at `destination` (`cdecl`, four arguments, the last
/// the allocator, which is not read): `00b25fa0` with two flag bytes the exe leaves
/// uninitialised (zeros here). Returns what it returns, the end of the destination.
pub fn fn_00b24030(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00b25fa0(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00b240b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the guides `first..last` backwards over the guides that end at `destination_end`
/// (`cdecl`, six arguments, the last three flag bytes not read): from the last element down,
/// each destination guide is assigned the source guide (`00b249d0`). Returns the start of the
/// destination range (`destination_end` minus the element count times 0x38).
pub fn fn_00b240b0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination_end: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let start = destination_end.wrapping_sub(element_count(first, last, 0x38).wrapping_mul(0x38));
    let mut source = last;
    let mut target = destination_end;
    while first != source {
        source = source.wrapping_sub(0x38);
        target = target.wrapping_sub(0x38);
        fn_00b249d0(e, Ptr::new(target), Ptr::new(source));
    }
    start
}

// Translated from 00b24180 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b24180 - the FS:[0] exception frame (the handler that destroys the
// copies already made) is not translated.
/// Fills `count` guides (0x38 bytes) from `destination` on with copy constructions of the one at
/// `value` (`00b1ff50`; `cdecl`, six arguments: the allocator and two flag bytes are not read);
/// a null destination slot is skipped as in the placement `new`.
pub fn fn_00b24180(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let mut target = destination;
    let mut left = count;
    while left != 0 {
        if target != 0 {
            e.call(GUIDE_COPY_CONSTRUCT, &args![target, value]);
        }
        target = target.wrapping_add(0x38);
        left = left.wrapping_sub(1);
    }
}

// Translated from 00b24290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b24290 - the FS:[0] exception frame (the handler that destroys the
// copies already made) is not translated.
/// Fills `count` guide vectors (0x18 bytes) from `destination` on with copy constructions of the
/// one at `value` (`00b231f0`; `cdecl`, six arguments: the allocator and two flag bytes are not
/// read); a null destination slot is skipped as in the placement `new`.
pub fn fn_00b24290(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let mut target = destination;
    let mut left = count;
    while left != 0 {
        if target != 0 {
            e.call(GUIDE_VECTOR_COPY_CONSTRUCT, &args![target, value]);
        }
        target = target.wrapping_add(0x18);
        left = left.wrapping_sub(1);
    }
}

// Translated from 00b243a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b243a0 - the FS:[0] exception frame (the handler that destroys the
// copies already made) is not translated.
/// Fills `count` textures (0x2C bytes) from `destination` on with copy constructions of the one
/// at `value` (`00b22f80`; `cdecl`, six arguments: the allocator and two flag bytes are not
/// read); a null destination slot is skipped as in the placement `new`.
pub fn fn_00b243a0(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let mut target = destination;
    let mut left = count;
    while left != 0 {
        if target != 0 {
            e.call(TEXTURE_COPY_CONSTRUCT, &args![target, value]);
        }
        target = target.wrapping_add(0x2C);
        left = left.wrapping_sub(1);
    }
}

// Translated from 00b244a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b244a0 - the FS:[0] exception frame is not translated.
/// Fills `count` `CVec` elements (0xC bytes, three words) from `destination` on with copies of
/// the one at `value` (`cdecl`, six arguments: the allocator and two flag bytes are not read);
/// a null destination slot is skipped as in the placement `new`.
pub fn fn_00b244a0(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let mut target = destination;
    let mut left = count;
    while left != 0 {
        if target != 0 {
            copy_range_words(e, target, value, 3);
        }
        target = target.wrapping_add(0xC);
        left = left.wrapping_sub(1);
    }
}

// Translated from 00b24580 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b24580 - the FS:[0] exception frame (the handler that destroys the
// copies already made) is not translated.
/// Fills `count` float vectors (0x18 bytes) from `destination` on with copy constructions of the
/// one at `value` (`00b22ff0`; `cdecl`, six arguments: the allocator and two flag bytes are not
/// read); a null destination slot is skipped as in the placement `new`.
pub fn fn_00b24580(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let mut target = destination;
    let mut left = count;
    while left != 0 {
        if target != 0 {
            e.call(FLOAT_VECTOR_COPY_CONSTRUCT, &args![target, value]);
        }
        target = target.wrapping_add(0x18);
        left = left.wrapping_sub(1);
    }
}

// Translated from 00b24690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy-constructs the vertices `first..last` at `destination` (`cdecl`, four arguments, the
/// last the allocator, which is not read): `00b26db0` with two flag bytes the exe leaves
/// uninitialised (zeros here). Returns what it returns, the end of the destination.
pub fn fn_00b24690(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00b26db0(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00b24710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the vertices `first..last` backwards so that they end at `destination_end` (`cdecl`,
/// six arguments, the last three flag bytes not read): from the last element down, 14 words
/// each. Returns the start of the destination range (`destination_end` minus the element
/// count times 0x38).
pub fn fn_00b24710(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination_end: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let mut source = last;
    let mut target = destination_end;
    while first != source {
        source = source.wrapping_sub(0x38);
        target = target.wrapping_sub(0x38);
        copy_range_words(e, target, source, 14);
    }
    destination_end.wrapping_sub(element_count(first, last, 0x38).wrapping_mul(0x38))
}

// Translated from 00b247b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy-constructs the textures `first..last` at `destination` (`cdecl`, four arguments, the
/// last the allocator, which is not read): `00b25ba0` with two flag bytes the exe leaves
/// uninitialised (zeros here). Returns what it returns, the end of the destination.
pub fn fn_00b247b0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00b25ba0(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00b24830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the textures `first..last` backwards over the textures that end at
/// `destination_end` (`cdecl`, six arguments, the last three flag bytes not read): from the last
/// element down, each destination texture is assigned the source texture (`00b24a80`). Returns
/// the start of the destination range (`destination_end` minus the element count times 0x2C).
pub fn fn_00b24830(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination_end: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let start = destination_end.wrapping_sub(element_count(first, last, 0x2C).wrapping_mul(0x2C));
    let mut source = last;
    let mut target = destination_end;
    while first != source {
        source = source.wrapping_sub(0x2C);
        target = target.wrapping_sub(0x2C);
        fn_00b24a80(e, Ptr::new(target), Ptr::new(source));
    }
    start
}

// Translated from 00b248c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the `CVec` elements `first..last` (0xC bytes) backwards so that they end at
/// `destination_end` (`cdecl`, six arguments, the last three flag bytes not read): from the last
/// element down, three words each. Returns the start of the destination range
/// (`destination_end` minus the element count times 0xC).
pub fn fn_00b248c0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination_end: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let start = destination_end.wrapping_sub(element_count(first, last, 0xC).wrapping_mul(0xC));
    let mut source = last;
    let mut target = destination_end;
    while first != source {
        source = source.wrapping_sub(0xC);
        target = target.wrapping_sub(0xC);
        copy_range_words(e, target, source, 3);
    }
    start
}

// Translated from 00b24960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the words of a checked range to `destination` (`cdecl`, six arguments: the proxies of
/// the two iterators, the first and last position, the destination and the allocator, of which
/// the proxies and the allocator are not read): the byte count `(last - first) >> 2` words is
/// moved with `memmove_s` (`00ec7ce3`) when it is not zero. Returns the destination plus four
/// bytes per word.
pub fn fn_00b24960(
    e: &mut Engine,
    _unused_0: u32,
    first: u32,
    _unused_2: u32,
    last: u32,
    destination: u32,
    _unused_5: u32,
) -> u32 {
    let words = (last.wrapping_sub(first) as i32 >> 2) as u32;
    if words != 0 {
        let bytes = words << 2;
        e.call(MEMMOVE_S, &args![destination, bytes, first, bytes]);
    }
    destination.wrapping_add(words.wrapping_mul(4))
}

// Translated from 00b249d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SFrondGuide::operator=` (the engine map gives it no name; `thiscall`, one argument): assigns
/// the vertex vector (`00b24ae0`) and copies the length, radius (floats through the x87 stack),
/// the frond map index byte, the offset angle, the surface area, the fuzzy surface area (floats)
/// and the vertex start index and vertices-per-vertex count (words). Returns `this`.
pub fn fn_00b249d0(
    e: &mut Engine,
    this: Ptr<SFrondGuide>,
    other: Ptr<SFrondGuide>,
) -> Ptr<SFrondGuide> {
    let (t, o) = (this.addr(), other.addr());
    fn_00b24ae0(e, this.cast(), other.cast());
    copy_float(e, t + 0x18, o + 0x18);
    copy_float(e, t + 0x1C, o + 0x1C);
    let index = e.mem.u8(o + 0x20);
    e.mem.set_u8(t + 0x20, index);
    copy_float(e, t + 0x24, o + 0x24);
    copy_float(e, t + 0x28, o + 0x28);
    copy_float(e, t + 0x2C, o + 0x2C);
    let word = e.mem.u32(o + 0x30);
    e.mem.set_u32(t + 0x30, word);
    let word = e.mem.u32(o + 0x34);
    e.mem.set_u32(t + 0x34, word);
    this
}

// Translated from 00b24a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SFrondTexture::operator=` (the engine map gives it no name; `thiscall`, one argument):
/// assigns the other's file name string (`0044a580`, offset 0, the `npos` count at `[010173f0]`)
/// and copies the four floats at `+0x1C` to `+0x28` through the x87 stack. Returns `this`.
pub fn fn_00b24a80(
    e: &mut Engine,
    this: Ptr<SFrondTexture>,
    other: Ptr<SFrondTexture>,
) -> Ptr<SFrondTexture> {
    let (t, o) = (this.addr(), other.addr());
    let npos = e.mem.u32(STRING_NPOS);
    e.call(STRING_ASSIGN, &args![t, o, 0u32, npos]);
    for offset in [0x1Cu32, 0x20, 0x24, 0x28] {
        copy_float(e, t + offset, o + offset);
    }
    this
}

// Translated from 00b24ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `operator=` of the vertex vector (a `std::vector<SFrondVertex>`; the engine map gives it no
/// name; `thiscall`, one argument): nothing for a self-assignment; an empty source clears
/// the vector (`00b25490`). When the source has no more elements than this vector, they are
/// copied over the first ones (`00b264f0`) and the end moves to match. When it has more but they
/// fit in the capacity, the first `size` of them are copied over the existing elements
/// (`00b260c0`) and the rest constructed at the end (`00b26db0`). Otherwise the old block is
/// freed and, when `_Buy` (`00b1eb00`) gives room, all the source's elements are constructed in
/// the new block. Returns `this`.
pub fn fn_00b24ae0(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) -> Ptr<StlVector> {
    let (t, o) = (this.addr(), other.addr());
    if t == o {
        return this;
    }
    let source_count = element_count(e.mem.u32(o + 0x0C), e.mem.u32(o + 0x10), 0x38);
    if source_count == 0 {
        fn_00b25490(e, this);
        return this;
    }
    let own_count = element_count(e.mem.u32(t + 0x0C), e.mem.u32(t + 0x10), 0x38);
    if source_count <= own_count {
        let (first, last) = (e.mem.u32(o + 0x0C), e.mem.u32(o + 0x10));
        let storage = e.mem.u32(t + 0x0C);
        let mut element = fn_00b264f0(e, first, last, storage, 0, 0, 0);
        // The loop that destroys the elements past the copy has nothing to run.
        let end = e.mem.u32(t + 0x10);
        while element != end {
            element = element.wrapping_add(0x38);
        }
        let new_end = element_count(e.mem.u32(o + 0x0C), e.mem.u32(o + 0x10), 0x38)
            .wrapping_mul(0x38)
            .wrapping_add(e.mem.u32(t + 0x0C));
        e.mem.set_u32(t + 0x10, new_end);
        return this;
    }
    let capacity = vector_capacity(e, t, 0x38);
    if capacity >= source_count {
        let middle = own_count
            .wrapping_mul(0x38)
            .wrapping_add(e.mem.u32(o + 0x0C));
        let (first, storage) = (e.mem.u32(o + 0x0C), e.mem.u32(t + 0x0C));
        fn_00b260c0(e, first, middle, storage);
        let (last, own_last) = (e.mem.u32(o + 0x10), e.mem.u32(t + 0x10));
        let end = fn_00b26db0(e, middle, last, own_last, t + 8, 0, 0);
        e.mem.set_u32(t + 0x10, end);
        return this;
    }
    if e.mem.u32(t + 0x0C) != 0 {
        // The loop that destroys the old vertices has nothing to run.
        let mut element = e.mem.u32(t + 0x0C);
        let last = e.mem.u32(t + 0x10);
        while element != last {
            element = element.wrapping_add(0x38);
        }
        let block = e.mem.u32(t + 0x0C);
        e.call(OPERATOR_DELETE, &args![block]);
    }
    let wanted = element_count(e.mem.u32(o + 0x0C), e.mem.u32(o + 0x10), 0x38);
    if e.call(VERTEX_VECTOR_BUY, &args![this, wanted]).bool() {
        let (first, last, storage) = (
            e.mem.u32(o + 0x0C),
            e.mem.u32(o + 0x10),
            e.mem.u32(t + 0x0C),
        );
        let end = fn_00b26db0(e, first, last, storage, t + 8, 0, 0);
        e.mem.set_u32(t + 0x10, end);
    }
    this
}

// Translated from 00b24f40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b24f40 - the FS:[0] exception frame (the handlers that unwind the
// guides being constructed) is not translated.
/// `operator=` of the guide vector (a `std::vector<SFrondGuide>`; the engine map gives it no
/// name; `thiscall`, one argument): nothing for a self-assignment; an empty source erases the
/// vector (`00b1de20`). When this vector is shorter than the source: if the capacity is too
/// small the old guides are destroyed (`00b1e970` on each, then the proxy released by
/// `00b39460`) and the block freed and, when `_Buy` (`00b1eb00`) gives room, all the source's
/// guides are copy constructed (`00b25fa0`); otherwise the first `size` guides are assigned
/// (`00b23760`) and the rest constructed at the end (`00b25fa0`). Else the source's guides are
/// assigned over the first ones (`00b253f0`), the guides past them are destroyed (`00b1ebc0`)
/// and the end moves to match. Returns `this`.
pub fn fn_00b24f40(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) -> Ptr<StlVector> {
    let (t, o) = (this.addr(), other.addr());
    if t == o {
        return this;
    }
    let source_count = element_count(e.mem.u32(o + 0x0C), e.mem.u32(o + 0x10), 0x38);
    if source_count == 0 {
        e.call(GUIDE_VECTOR_ERASE_ALL, &args![this]);
        return this;
    }
    let own_count = element_count(e.mem.u32(t + 0x0C), e.mem.u32(t + 0x10), 0x38);
    if own_count < source_count {
        let capacity = vector_capacity(e, t, 0x38);
        if capacity < source_count {
            if e.mem.u32(t + 0x0C) != 0 {
                let last = e.mem.u32(t + 0x10);
                let mut element = e.mem.u32(t + 0x0C);
                while element != last {
                    e.call(VERTEX_VECTOR_TIDY, &args![element]);
                    e.call(PROXY_RELEASE, &args![element]);
                    if delete_flag(NO_DELETE) {
                        e.call(OPERATOR_DELETE, &args![element]);
                    }
                    element = element.wrapping_add(0x38);
                }
                let block = e.mem.u32(t + 0x0C);
                e.call(OPERATOR_DELETE, &args![block]);
            }
            let wanted = element_count(e.mem.u32(o + 0x0C), e.mem.u32(o + 0x10), 0x38);
            if e.call(VERTEX_VECTOR_BUY, &args![this, wanted]).bool() {
                let (first, last, storage) = (
                    e.mem.u32(o + 0x0C),
                    e.mem.u32(o + 0x10),
                    e.mem.u32(t + 0x0C),
                );
                let end = fn_00b25fa0(e, first, last, storage, t + 8, 0, 0);
                e.mem.set_u32(t + 0x10, end);
            }
        } else {
            let middle = own_count
                .wrapping_mul(0x38)
                .wrapping_add(e.mem.u32(o + 0x0C));
            let (first, storage) = (e.mem.u32(o + 0x0C), e.mem.u32(t + 0x0C));
            e.call(GUIDE_ASSIGN_RANGE, &args![first, middle, storage]);
            let (last, own_last) = (e.mem.u32(o + 0x10), e.mem.u32(t + 0x10));
            let end = fn_00b25fa0(e, middle, last, own_last, t + 8, 0, 0);
            e.mem.set_u32(t + 0x10, end);
        }
    } else {
        let (first, last, storage) = (
            e.mem.u32(o + 0x0C),
            e.mem.u32(o + 0x10),
            e.mem.u32(t + 0x0C),
        );
        let assigned = fn_00b253f0(e, first, last, storage, 0, 0, 0);
        let own_last = e.mem.u32(t + 0x10);
        e.call(GUIDE_DESTROY_RANGE, &args![this, assigned, own_last]);
        let new_end = element_count(e.mem.u32(o + 0x0C), e.mem.u32(o + 0x10), 0x38)
            .wrapping_mul(0x38)
            .wrapping_add(e.mem.u32(t + 0x0C));
        e.mem.set_u32(t + 0x10, new_end);
    }
    this
}

// Translated from 00b253f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the guides `first..last` over the guides from `destination` on (`cdecl`, six
/// arguments, the last three flag bytes not read): `00b249d0` on each pair in order. Returns
/// the destination plus the element count times 0x38.
pub fn fn_00b253f0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let end = element_count(first, last, 0x38)
        .wrapping_mul(0x38)
        .wrapping_add(destination);
    let mut target = destination;
    let mut source = first;
    while source != last {
        fn_00b249d0(e, Ptr::new(target), Ptr::new(source));
        target = target.wrapping_add(0x38);
        source = source.wrapping_add(0x38);
    }
    end
}

// Translated from 00b25490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the vertex vector (the engine map gives it no name; `fastcall`, `this` only): erases
/// the whole range with `00b25580`, from the checked iterator at the first element to the one
/// at the last.
pub fn fn_00b25490(e: &mut Engine, this: Ptr<StlVector>) {
    let t = this.addr();
    let last = e.mem.u32(t + 0x10);
    let last_proxy = checked_iterator(e, t, last);
    let first = e.mem.u32(t + 0x0C);
    let first_proxy = checked_iterator(e, t, first);
    e.with_stack(8, |e, out| {
        fn_00b25580(e, this, out, first_proxy, first, last_proxy, last);
    });
}

// Translated from 00b25580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked range erase of the vertex vector (the engine map gives it no name; `thiscall`,
/// five arguments): the iterators at `first` and `last` (their proxy words are not read) are
/// made with the range checks; when they differ the vertices from `last` to the end are copied
/// down over the erased ones (`00b260c0`), the elements left past the new end are destroyed
/// (nothing to do) and the end pointer is set to the new end. Writes the iterator at `first`
/// (proxy, position) to `out` and returns it.
pub fn fn_00b25580(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    _unused_2: u32,
    first: u32,
    _unused_4: u32,
    last: u32,
) -> Ptr {
    let t = this.addr();
    let first_proxy = checked_iterator(e, t, first);
    let last_proxy = checked_iterator(e, t, last);
    let differ = e.with_stack(0x10, |e, frame| {
        let a = frame.addr();
        let b = a + 8;
        e.mem.set_u32(a, first_proxy);
        e.mem.set_u32(a + 4, first);
        e.mem.set_u32(b, last_proxy);
        e.mem.set_u32(b + 4, last);
        e.call(ITERATORS_DIFFER, &args![a, b]).bool()
    });
    if differ {
        let end = e.mem.u32(t + 0x10);
        let new_end = fn_00b260c0(e, last, end, first);
        let end = e.mem.u32(t + 0x10);
        let mut element = new_end;
        while element != end {
            element = element.wrapping_add(0x38);
        }
        e.mem.set_u32(t + 0x10, new_end);
    }
    e.mem.set_u32(out.addr(), first_proxy);
    e.mem.set_u32(out.addr() + 4, first);
    out
}

// Translated from 00b25680 (decompiled, FalloutNV.exe 1.4.0.525)
/// The three-way partition of the guide sort (`std::_Unguarded_partition`'s relative; the engine
/// map gives it no name; `cdecl`, four arguments, the last a byte-sized empty comparison
/// object): the pivot is the middle guide, after `00b26160` has put a median of the range there.
/// Equal keys (by `<` in both directions, so also incomparable ones) around the pivot are
/// gathered next to it by swapping guides (`00b26ab0`): the equal ones below the pivot to its
/// end, the equal ones above it past the pivot's run. Writes the start and the end of the equal
/// run to `out` and returns `out`. The keys are the fuzzy surface areas at `+0x2C`.
pub fn fn_00b25680(e: &mut Engine, out: Ptr, first: u32, last: u32, comparison: u8) -> Ptr {
    let key = |e: &Engine, guide: u32| e.mem.f32(guide.wrapping_add(0x2C));
    let middle = (element_count(first, last, 0x38) as i32 / 2).wrapping_mul(0x38) as u32;
    let middle = middle.wrapping_add(first);
    fn_00b26160(e, first, middle, last.wrapping_sub(0x38), comparison);
    // `pivot` runs down over the keys equal to the pivot's, `run_end` up past them.
    let mut pivot = middle;
    let mut run_end = pivot.wrapping_add(0x38);
    while first < pivot {
        // The exe compares the previous guide's key (`pivot - 0xC` is `pivot - 0x38 + 0x2C`).
        if key(e, pivot) < e.mem.f32(pivot.wrapping_sub(0xC)) {
            break;
        }
        if e.mem.f32(pivot.wrapping_sub(0xC)) < key(e, pivot) {
            break;
        }
        pivot = pivot.wrapping_sub(0x38);
    }
    while run_end < last {
        if key(e, pivot) < key(e, run_end) {
            break;
        }
        if key(e, run_end) < key(e, pivot) {
            break;
        }
        run_end = run_end.wrapping_add(0x38);
    }
    let mut up = run_end;
    let mut down = pivot;
    let mut low_run = pivot;
    let mut high_run = run_end;
    // `up` scans above the run, `down` below it; `low_run` and `high_run` are the run's ends.
    loop {
        while up < last {
            if key(e, up) < key(e, low_run) {
                up = up.wrapping_add(0x38);
                continue;
            }
            if key(e, low_run) < key(e, up) {
                break;
            }
            let moved = high_run;
            high_run = high_run.wrapping_add(0x38);
            fn_00b26ab0(e, Ptr::new(moved), Ptr::new(up));
            up = up.wrapping_add(0x38);
        }
        while first < down {
            if key(e, low_run) < e.mem.f32(down.wrapping_sub(0xC)) {
                down = down.wrapping_sub(0x38);
                continue;
            }
            if e.mem.f32(down.wrapping_sub(0xC)) < key(e, low_run) {
                break;
            }
            low_run = low_run.wrapping_sub(0x38);
            fn_00b26ab0(e, Ptr::new(low_run), Ptr::new(down.wrapping_sub(0x38)));
            down = down.wrapping_sub(0x38);
        }
        if down == first && up == last {
            e.mem.set_u32(out.addr(), low_run);
            e.mem.set_u32(out.addr() + 4, high_run);
            return out;
        }
        if down == first {
            if high_run != up {
                fn_00b26ab0(e, Ptr::new(low_run), Ptr::new(high_run));
            }
            high_run = high_run.wrapping_add(0x38);
            let (moved, partner) = (up, low_run);
            low_run = low_run.wrapping_add(0x38);
            up = up.wrapping_add(0x38);
            fn_00b26ab0(e, Ptr::new(partner), Ptr::new(moved));
        } else if up == last {
            down = down.wrapping_sub(0x38);
            low_run = low_run.wrapping_sub(0x38);
            if down != low_run {
                fn_00b26ab0(e, Ptr::new(down), Ptr::new(low_run));
            }
            high_run = high_run.wrapping_sub(0x38);
            fn_00b26ab0(e, Ptr::new(low_run), Ptr::new(high_run));
        } else {
            down = down.wrapping_sub(0x38);
            fn_00b26ab0(e, Ptr::new(up), Ptr::new(down));
            up = up.wrapping_add(0x38);
        }
    }
}

// Translated from 00b25a90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b25a90 - the FS:[0] exception frame is not translated.
/// Copies the 0x18-byte elements `first..last` (six words each) to uninitialised storage from
/// `destination` on (`cdecl`, six arguments: the allocator and two flag bytes are not read); a
/// null destination slot is skipped as in the placement `new`. Returns the end of the
/// destination.
pub fn fn_00b25a90(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let mut target = destination;
    let mut source = first;
    while source != last {
        if target != 0 {
            copy_range_words(e, target, source, 6);
        }
        target = target.wrapping_add(0x18);
        source = source.wrapping_add(0x18);
    }
    target
}

// Translated from 00b25ba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b25ba0 - the FS:[0] exception frame (the handler that destroys the
// copies already made) is not translated.
/// Copy-constructs the textures `first..last` at uninitialised storage from `destination` on
/// (`00b22f80`; `cdecl`, six arguments: the allocator and two flag bytes are not read); a null
/// destination slot is skipped as in the placement `new`. Returns the end of the destination.
pub fn fn_00b25ba0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let mut target = destination;
    let mut source = first;
    while source != last {
        if target != 0 {
            e.call(TEXTURE_COPY_CONSTRUCT, &args![target, source]);
        }
        target = target.wrapping_add(0x2C);
        source = source.wrapping_add(0x2C);
    }
    target
}

// Translated from 00b25ca0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b25ca0 - the FS:[0] exception frame (the handlers that destroy the
// temporary and the elements already made) is not translated.
/// Moves the guide vectors `first..last` (0x18-byte elements) into uninitialised storage from
/// `destination` on (`cdecl`, six arguments: the allocator and two flag bytes are not read): an
/// empty guide vector is made on the stack (`00b1d8a0`); for each element a copy of it is
/// constructed at the destination slot (`00b231f0`, skipped for a null slot) and the destination
/// then exchanges its contents with the source element (`00b267a0`). The temporary is tidied
/// (`00b1ec70`) and its proxy deleted. Returns the end of the destination.
pub fn fn_00b25ca0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    e.with_stack(0x18, |e, temporary| {
        let temp = temporary.addr();
        e.call(GUIDE_VECTOR_DEFAULT_CONSTRUCT, &args![temp]);
        let mut target = destination;
        let mut source = first;
        while source != last {
            if target != 0 {
                e.call(GUIDE_VECTOR_COPY_CONSTRUCT, &args![target, temp]);
            }
            fn_00b267a0(e, Ptr::new(target), Ptr::new(source));
            target = target.wrapping_add(0x18);
            source = source.wrapping_add(0x18);
        }
        e.call(GUIDE_VECTOR_TIDY, &args![temp]);
        let proxy = e.mem.u32(temp);
        e.call(OPERATOR_DELETE, &args![proxy]);
        target
    })
}

// Translated from 00b25e20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b25e20 - the FS:[0] exception frame (the handlers that destroy the
// temporary and the elements already made) is not translated.
/// Moves the float vectors `first..last` (0x18-byte elements) into uninitialised storage from
/// `destination` on (`cdecl`, six arguments: the allocator and two flag bytes are not read): an
/// empty float vector is made on the stack (`00449fd0`); for each element a copy of it is
/// constructed at the destination slot (`00b22ff0`, skipped for a null slot) and the destination
/// then exchanges its contents with the source element (`00b26570`). The temporary is tidied
/// (`0065f240`) and its proxy deleted. Returns the end of the destination.
pub fn fn_00b25e20(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    e.with_stack(0x18, |e, temporary| {
        let temp = temporary.addr();
        e.call(FLOAT_VECTOR_CONSTRUCT, &args![temp]);
        let mut target = destination;
        let mut source = first;
        while source != last {
            if target != 0 {
                e.call(FLOAT_VECTOR_COPY_CONSTRUCT, &args![target, temp]);
            }
            fn_00b26570(e, Ptr::new(target), Ptr::new(source));
            target = target.wrapping_add(0x18);
            source = source.wrapping_add(0x18);
        }
        e.call(FLOAT_VECTOR_TIDY, &args![temp]);
        let proxy = e.mem.u32(temp);
        e.call(OPERATOR_DELETE, &args![proxy]);
        target
    })
}

// Translated from 00b25fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b25fa0 - the FS:[0] exception frame (the handler that destroys the
// copies already made) is not translated.
/// Copy-constructs the guides `first..last` at uninitialised storage from `destination` on
/// (`00b1ff50`; `cdecl`, six arguments: the allocator and two flag bytes are not read); a null
/// destination slot is skipped as in the placement `new`. Returns the end of the destination.
pub fn fn_00b25fa0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let mut target = destination;
    let mut source = first;
    while source != last {
        if target != 0 {
            e.call(GUIDE_COPY_CONSTRUCT, &args![target, source]);
        }
        target = target.wrapping_add(0x38);
        source = source.wrapping_add(0x38);
    }
    target
}

// Translated from 00b260c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the vertices `first..last` (0x38 bytes, 14 words) over the vertices from
/// `destination` on (`cdecl`, three arguments). Returns the destination plus the element count
/// times 0x38.
pub fn fn_00b260c0(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    let mut target = destination;
    let mut source = first;
    while source != last {
        copy_range_words(e, target, source, 14);
        target = target.wrapping_add(0x38);
        source = source.wrapping_add(0x38);
    }
    element_count(first, last, 0x38)
        .wrapping_mul(0x38)
        .wrapping_add(destination)
}

// Translated from 00b26160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The pivot choice of the guide sort (the engine map gives it no name; `cdecl`, four arguments,
/// the last a byte-sized empty comparison object that is not read): puts the median of three
/// guides of `first`, `middle` and `last` at `middle` (`00b269d0`). Over 40 elements between
/// `first` and `last` (`last` is the last guide, not the end) it takes the median of three
/// medians of three, spaced an eighth of the range apart (`(count + 1) / 8` guides, rounded
/// toward zero).
pub fn fn_00b26160(e: &mut Engine, first: u32, middle: u32, last: u32, comparison: u8) {
    let comparison = u32::from(comparison);
    if element_count(first, last, 0x38) as i32 > 0x28 {
        let count = element_count(first, last, 0x38) as i32 + 1;
        // `ADD 7 if negative; SAR 3`: a division by 8 toward zero.
        let step = ((count + ((count >> 31) & 7)) >> 3).wrapping_mul(0x38) as u32;
        fn_00b269d0(
            e,
            first,
            first.wrapping_add(step),
            first.wrapping_add(step.wrapping_mul(2)),
            comparison,
        );
        fn_00b269d0(
            e,
            middle.wrapping_sub(step),
            middle,
            middle.wrapping_add(step),
            comparison,
        );
        fn_00b269d0(
            e,
            last.wrapping_sub(step.wrapping_mul(2)),
            last.wrapping_sub(step),
            last,
            comparison,
        );
        fn_00b269d0(
            e,
            first.wrapping_add(step),
            middle,
            last.wrapping_sub(step),
            comparison,
        );
    } else {
        fn_00b269d0(e, first, middle, last, comparison);
    }
}

// Translated from 00b26260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::make_heap` of the guide sort (the engine map gives it no name; `cdecl`, five arguments,
/// the third a byte-sized empty comparison object, the last two zeros): for each of the first
/// half of the guides, from the middle down to the first, a copy of the guide is made
/// (`00b1ff50`) in the outgoing argument area of `00b26b50`, which sifts it down and destroys it
/// there (the copy's proxy names that area).
pub fn fn_00b26260(
    e: &mut Engine,
    first: u32,
    last: u32,
    comparison: u8,
    _unused_3: u32,
    _unused_4: u32,
) {
    let count = element_count(first, last, 0x38) as i32;
    let mut half = count / 2;
    while half > 0 {
        half -= 1;
        let element = (half.wrapping_mul(0x38) as u32).wrapping_add(first);
        // 00b2629f: `SUB ESP,0x38; MOV EDX,ESP`: the copy is built in the outgoing argument
        // area, which stays in place as `00b26b50`'s own `[EBP+0x14]` (its caller's
        // `ADD ESP,0x48` frees it).
        e.with_stack(0x38, |e, outgoing| {
            e.call(GUIDE_COPY_CONSTRUCT, &args![outgoing, element]); // 00b262aa
            fn_00b26b50(e, first, half, count, outgoing.cast(), comparison); // 00b262be: the area itself
        });
    }
}

// Translated from 00b262d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b262d0 - the FS:[0] exception frame (the handler that destroys the
// temporary guide) is not translated.
/// The insertion sort of the guide sort (`std::_Insertion_sort`; the engine map gives it no
/// name; `cdecl`, four arguments, the last two not read): for each guide after the first, a copy
/// is made on the stack (`00b1ff50`); when its key (the fuzzy surface area) is greater than the
/// first guide's, the range before it is moved up by one (`00b26ce0`) and the copy assigned to
/// the first slot; otherwise the guides before it with a smaller key are moved up one by one
/// (`00b249d0`) and the copy assigned to the hole. The copy is tidied (`00b1e970`) and its proxy
/// deleted.
pub fn fn_00b262d0(e: &mut Engine, first: u32, last: u32, _unused_2: u32, _unused_3: u32) {
    if first == last {
        return;
    }
    let mut current = first;
    loop {
        current = current.wrapping_add(0x38);
        if current == last {
            break;
        }
        let mut hole = current;
        e.with_stack(0x38, |e, copy| {
            let temp = copy.addr();
            e.call(GUIDE_COPY_CONSTRUCT, &args![temp, current]);
            let temp_key = e.mem.f32(temp + 0x2C);
            if e.mem.f32(first.wrapping_add(0x2C)) < temp_key {
                hole = hole.wrapping_add(0x38);
                fn_00b26ce0(e, first, current, hole);
                fn_00b249d0(e, Ptr::new(first), Ptr::new(temp));
            } else {
                let mut previous = hole;
                loop {
                    hole = previous;
                    previous = previous.wrapping_sub(0x38);
                    if e.mem.f32(previous.wrapping_add(0x2C)) < temp_key {
                        fn_00b249d0(e, Ptr::new(hole), Ptr::new(previous));
                    } else {
                        break;
                    }
                }
                fn_00b249d0(e, Ptr::new(hole), Ptr::new(temp));
            }
            e.call(VERTEX_VECTOR_TIDY, &args![temp]);
            let proxy = e.mem.u32(temp);
            e.call(OPERATOR_DELETE, &args![proxy]);
        });
    }
}

// Translated from 00b26450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy-constructs the guides `first..last` at `destination` (`cdecl`, four arguments, the last
/// the allocator, which is not read): `00b25fa0` with two flag bytes the exe leaves
/// uninitialised (zeros here). Returns what it returns, the end of the destination.
pub fn fn_00b26450(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00b25fa0(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00b264a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy-constructs the vertices `first..last` at `destination` (`cdecl`, four arguments, the
/// last the allocator, which is not read): `00b26db0` with two flag bytes the exe leaves
/// uninitialised (zeros here). Returns what it returns, the end of the destination.
pub fn fn_00b264a0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00b26db0(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00b264f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the vertices `first..last` (0x38 bytes, 14 words) over the vertices from
/// `destination` on (`cdecl`, six arguments: the last three are not read). Returns the
/// destination plus the element count times 0x38.
pub fn fn_00b264f0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let mut target = destination;
    let mut source = first;
    while source != last {
        copy_range_words(e, target, source, 14);
        target = target.wrapping_add(0x38);
        source = source.wrapping_add(0x38);
    }
    element_count(first, last, 0x38)
        .wrapping_mul(0x38)
        .wrapping_add(destination)
}

/// Whether the exe's vector swap takes its first branch: it tests the constant 1 (the
/// allocators of the two vectors compare equal), so the copying branch after it never runs.
const SAME_ALLOCATOR: bool = true;

// Translated from 00b26570 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b26570 - the FS:[0] exception frame (the unwinding of the
// temporary in the copying branch) is not translated.
/// `std::vector<float>::swap` (the engine map gives it no name; `thiscall`, one argument): nothing
/// for a swap with itself. The exe then tests the constant 1 (equal allocators): the proxy words
/// of the two vectors are exchanged, the first words of the two proxy blocks (their owner
/// pointers) are exchanged, and the first, last and end pointers at `+0xC`, `+0x10` and `+0x14`
/// are exchanged. The branch after that, which never runs, swaps through a copy: the proxy
/// words and the proxy blocks' first words are exchanged, a copy of `this` is built on the stack
/// (`00b22ff0`), `this` is assigned the other (`00b1d5d0`), the other the copy, and the copy is
/// tidied (`0065f240`) and its proxy deleted.
pub fn fn_00b26570(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) {
    vector_swap(
        e,
        this,
        other,
        SAME_ALLOCATOR,
        FLOAT_VECTOR_COPY_CONSTRUCT,
        FLOAT_VECTOR_ASSIGN,
        FLOAT_VECTOR_TIDY,
    );
}

// Translated from 00b267a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b267a0 - the FS:[0] exception frame (the unwinding of the
// temporary in the copying branch) is not translated.
/// `std::vector<SFrondGuide>::swap` (the engine map gives it no name; `thiscall`, one
/// argument): like [`fn_00b26570`] for a guide vector; its copying branch, which never runs,
/// copy constructs the temporary with `00b231f0`, assigns with `00b24f40` and tidies with
/// `00b1ec70`.
pub fn fn_00b267a0(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) {
    vector_swap(
        e,
        this,
        other,
        SAME_ALLOCATOR,
        GUIDE_VECTOR_COPY_CONSTRUCT,
        GUIDE_VECTOR_ASSIGN,
        GUIDE_VECTOR_TIDY,
    );
}

/// The body of the two vector swaps: exchanges the words at `a` and `b` when they differ.
fn swap_words(e: &mut Engine, a: u32, b: u32) {
    if a != b {
        let first = e.mem.u32(a);
        let second = e.mem.u32(b);
        e.mem.set_u32(a, second);
        e.mem.set_u32(b, first);
    }
}

/// `vector::swap(this, other)` of the exe's two instances: `same_allocator` is the exe's constant
/// condition (always true); `copy_construct`, `assign` and `tidy` are the element type's vector
/// helpers used by the branch that makes a copy.
fn vector_swap(
    e: &mut Engine,
    this: Ptr<StlVector>,
    other: Ptr<StlVector>,
    same_allocator: bool,
    copy_construct: u32,
    assign: u32,
    tidy: u32,
) {
    let (t, o) = (this.addr(), other.addr());
    if t == o {
        return;
    }
    if same_allocator {
        swap_words(e, t, o);
        let (this_proxy, other_proxy) = (e.mem.u32(t), e.mem.u32(o));
        swap_words(e, other_proxy, this_proxy);
        swap_words(e, o + 0x0C, t + 0x0C);
        swap_words(e, o + 0x10, t + 0x10);
        swap_words(e, o + 0x14, t + 0x14);
        return;
    }
    // Not reached: the exe's constant condition above is always true.
    swap_words(e, t, o);
    let (this_proxy, other_proxy) = (e.mem.u32(t), e.mem.u32(o));
    swap_words(e, other_proxy, this_proxy);
    e.with_stack(0x18, |e, copy| {
        let temp = copy.addr();
        e.call(copy_construct, &args![temp, t]);
        e.call(assign, &args![t, o]);
        e.call(assign, &args![o, temp]);
        e.call(tidy, &args![temp]);
        let proxy = e.mem.u32(temp);
        e.call(OPERATOR_DELETE, &args![proxy]);
    });
}

// Translated from 00b269d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The median of three of the guide sort (the engine map gives it no name; `cdecl`, four
/// arguments, the last a byte-sized empty comparison object that is not read): orders the three
/// guides `a`, `b`, `c` by their keys (the fuzzy surface areas at `+0x2C`), swapping them
/// (`00b26ab0`) so that the key of `b` is not above `a`'s and `c`'s is not above `b`'s, in the
/// exe's three comparisons: `a < b`, then `b < c`, then `a < b` again.
pub fn fn_00b269d0(e: &mut Engine, a: u32, b: u32, c: u32, _unused_3: u32) {
    if e.mem.f32(a.wrapping_add(0x2C)) < e.mem.f32(b.wrapping_add(0x2C)) {
        fn_00b26ab0(e, Ptr::new(b), Ptr::new(a));
    }
    if e.mem.f32(b.wrapping_add(0x2C)) < e.mem.f32(c.wrapping_add(0x2C)) {
        fn_00b26ab0(e, Ptr::new(c), Ptr::new(b));
    }
    if e.mem.f32(a.wrapping_add(0x2C)) < e.mem.f32(b.wrapping_add(0x2C)) {
        fn_00b26ab0(e, Ptr::new(b), Ptr::new(a));
    }
}

// Translated from 00b26ab0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b26ab0 - the FS:[0] exception frame (the handler that destroys the
// temporary guide) is not translated.
/// Swaps two guides (`std::iter_swap`; the engine map gives it no name; `cdecl`, two
/// arguments): nothing when they are the same; otherwise a copy of the first is made on the
/// stack (`00b1ff50`), the first is assigned the second (`00b249d0`), the second the copy, and
/// the copy is tidied (`00b1e970`) and its proxy deleted.
pub fn fn_00b26ab0(e: &mut Engine, first: Ptr<SFrondGuide>, second: Ptr<SFrondGuide>) {
    if first == second {
        return;
    }
    e.with_stack(0x38, |e, copy| {
        e.call(GUIDE_COPY_CONSTRUCT, &args![copy, first]);
        fn_00b249d0(e, first, second);
        fn_00b249d0(e, second, copy.cast());
        e.call(VERTEX_VECTOR_TIDY, &args![copy]);
        let proxy = e.mem.u32(copy.addr());
        e.call(OPERATOR_DELETE, &args![proxy]);
    });
}

/// Destroys the guide passed by value to `00b26b50` and `00b26e80` where it lies, in the
/// callee's own argument area (`[EBP+0x14]`), as both do at their end: its vertex vector tidied
/// (`00b1e970`, `LEA ECX,[EBP+0x14]`) and its proxy (the word at `+0`) deleted.
fn destroy_guide_argument(e: &mut Engine, value: Ptr<SFrondGuide>) {
    e.call(VERTEX_VECTOR_TIDY, &args![value]);
    let proxy = e.mem.u32(value.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

/// Through the uniform ABI the 14 words of a by-value guide arrive as values: they are laid out
/// in an area that stands for the callee's argument area (`[EBP+0x14]`), and `body` runs on it.
/// The exe's callers build the copy in that area themselves (`00b262aa`, `00b26c44`,
/// `00b26ff1`), as `fn_00b26260`, `fn_00b26b50` and `fn_00b26f90` do.
fn with_guide_words(
    e: &mut Engine,
    words: &[u32],
    body: impl FnOnce(&mut Engine, Ptr<SFrondGuide>),
) {
    e.with_stack(0x38, |e, area| {
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(area.addr() + 4 * i as u32, *word);
        }
        body(e, area.cast());
    });
}

// Translated from 00b26b50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b26b50 - the FS:[0] exception frame (the handler that destroys the
// by-value guide argument) is not translated.
/// `std::_Adjust_heap` of the guide sort (the engine map gives it no name; `cdecl`: the first
/// guide, the hole and bottom indices, the guide `value` passed by value (14 words, copy
/// constructed by the caller) and the byte-sized empty comparison object): moves the hole down
/// by assigning to it the child with the larger key (the second child unless the first has a
/// key below it; `00b249d0`), until the hole has no children left (a lone last child is
/// assigned too); then pushes `value` up from the hole (`00b26e80`, with a fresh copy of
/// `value` built in its argument area). `value` is destroyed at the end (`00b1e970` and its
/// proxy deleted).
///
/// `value` is the address of the argument area that holds the by-value guide (the exe's
/// `[EBP+0x14]`): the caller built the copy there, so the proxy of its vertex vector names that
/// area, and it is worked on and destroyed in place.
pub fn fn_00b26b50(
    e: &mut Engine,
    first: u32,
    hole: i32,
    bottom: i32,
    value: Ptr<SFrondGuide>,
    comparison: u8,
) {
    let guide = |index: i32| (index.wrapping_mul(0x38) as u32).wrapping_add(first);
    let top = hole;
    let mut hole = hole;
    let mut child = hole.wrapping_mul(2).wrapping_add(2);
    while child < bottom {
        if e.mem.f32(guide(child.wrapping_sub(1)).wrapping_add(0x2C))
            < e.mem.f32(guide(child).wrapping_add(0x2C))
        {
            child = child.wrapping_sub(1);
        }
        fn_00b249d0(e, Ptr::new(guide(hole)), Ptr::new(guide(child)));
        hole = child;
        child = child.wrapping_mul(2).wrapping_add(2);
    }
    if child == bottom {
        fn_00b249d0(
            e,
            Ptr::new(guide(hole)),
            Ptr::new(guide(bottom.wrapping_sub(1))),
        );
        hole = bottom.wrapping_sub(1);
    }
    // 00b26c38: `SUB ESP,0x38; MOV ECX,ESP`: the copy of `value` is built in the outgoing
    // argument area, which stays in place as `00b26e80`'s own `[EBP+0x14]` (`ADD ESP,0x48`
    // at 00b26c5d frees it).
    e.with_stack(0x38, |e, outgoing| {
        e.call(GUIDE_COPY_CONSTRUCT, &args![outgoing, value]); // 00b26c44: from [EBP+0x14]
        fn_00b26e80(e, first, hole, top, outgoing.cast(), comparison); // 00b26c58: the area itself
    });
    destroy_guide_argument(e, value); // 00b26c6e..00b26c87: `value` at [EBP+0x14] itself
}

// Translated from 00b26ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::pop_heap` of the guide sort (`cdecl`, three arguments, the last a byte-sized empty
/// comparison object): with more than one guide, calls `00b26f90` on the range.
pub fn fn_00b26ca0(e: &mut Engine, first: u32, last: u32, comparison: u8) {
    if element_count(first, last, 0x38) as i32 > 1 {
        fn_00b26f90(e, first, last, comparison, 0);
    }
}

// Translated from 00b26ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the guides `first..last` backwards over the guides that end at `destination_end`
/// (`cdecl`, three arguments): from the last element down, each destination guide is assigned
/// the source guide (`00b249d0`). Returns the start of the destination range (`destination_end`
/// minus the element count times 0x38).
pub fn fn_00b26ce0(e: &mut Engine, first: u32, last: u32, destination_end: u32) -> u32 {
    let start = destination_end.wrapping_sub(element_count(first, last, 0x38).wrapping_mul(0x38));
    let mut source = last;
    let mut target = destination_end;
    while first != source {
        source = source.wrapping_sub(0x38);
        target = target.wrapping_sub(0x38);
        fn_00b249d0(e, Ptr::new(target), Ptr::new(source));
    }
    start
}

// Translated from 00b26db0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b26db0 - the FS:[0] exception frame is not translated.
/// Copies the vertices `first..last` (0x38 bytes, 14 words) to uninitialised storage from
/// `destination` on (`cdecl`, six arguments: the allocator and two flag bytes are not read); a
/// null destination slot is skipped as in the placement `new`. Returns the end of the
/// destination.
pub fn fn_00b26db0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let mut target = destination;
    let mut source = first;
    while source != last {
        if target != 0 {
            copy_range_words(e, target, source, 14);
        }
        target = target.wrapping_add(0x38);
        source = source.wrapping_add(0x38);
    }
    target
}

// Translated from 00b26e80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b26e80 - the FS:[0] exception frame (the handler that destroys the
// by-value guide argument) is not translated.
/// `std::_Push_heap_0` of the guide sort (the engine map gives it no name; `cdecl`: the first
/// guide, the hole and top indices, the guide `value` passed by value (14 words) and the
/// byte-sized empty comparison object): while the hole is above `top` and the parent's key
/// (`(hole - 1) / 2`, the fuzzy surface area at `+0x2C`) is greater than `value`'s, the parent
/// is assigned to the hole (`00b249d0`) and the hole moves to it; then `value` is assigned to
/// the hole and destroyed (`00b1e970`, its proxy deleted).
///
/// `value` is the address of the argument area that holds the by-value guide (the exe's
/// `[EBP+0x14]`): the caller built the copy there (`00b26c38`..`00b26c44`), so the proxy of its
/// vertex vector names that area, and it is read and destroyed in place.
pub fn fn_00b26e80(
    e: &mut Engine,
    first: u32,
    hole: i32,
    top: i32,
    value: Ptr<SFrondGuide>,
    _comparison: u8,
) {
    let guide = |index: i32| (index.wrapping_mul(0x38) as u32).wrapping_add(first);
    let argument = value.addr(); // [EBP+0x14]: the guide is worked on where the caller built it
    let mut hole = hole;
    let mut parent = hole.wrapping_sub(1) / 2;
    while top < hole {
        if e.mem.f32(argument + 0x2C) < e.mem.f32(guide(parent).wrapping_add(0x2C)) {
            fn_00b249d0(e, Ptr::new(guide(hole)), Ptr::new(guide(parent)));
            hole = parent;
            parent = hole.wrapping_sub(1) / 2;
        } else {
            break;
        }
    }
    fn_00b249d0(e, Ptr::new(guide(hole)), value); // 00b26f2f: `LEA EAX,[EBP+0x14]`
    destroy_guide_argument(e, value); // 00b26f4f..00b26f6e: `value` at [EBP+0x14] itself
}

// Translated from 00b26f90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b26f90 - the FS:[0] exception frame (the handler that destroys the
// temporary guide) is not translated.
/// `std::_Pop_heap` of the guide sort (the engine map gives it no name; `cdecl`, four arguments,
/// the third a byte-sized empty comparison object, the last not read): takes a copy of the last
/// guide (`00b1ff50`), assigns the first guide to the last slot (`00b249d0`) and sifts the copy
/// down from the root over the shortened range (`00b26b50`, with a fresh copy of it passed by
/// value, built in `00b26b50`'s argument area). The temporary is tidied (`00b1e970`) and its
/// proxy deleted.
pub fn fn_00b26f90(e: &mut Engine, first: u32, last: u32, comparison: u8, _unused_3: u32) {
    let last_guide = last.wrapping_sub(0x38);
    e.with_stack(0x38, |e, temporary| {
        let temp = temporary.addr();
        e.call(GUIDE_COPY_CONSTRUCT, &args![temp, last_guide]);
        fn_00b249d0(e, Ptr::new(last_guide), Ptr::new(first));
        // 00b26fe5: `SUB ESP,0x38; MOV ECX,ESP`: the copy is built in the outgoing argument
        // area, which stays in place as `00b26b50`'s own `[EBP+0x14]` (`ADD ESP,0x48` at
        // 00b27019 frees it).
        e.with_stack(0x38, |e, outgoing| {
            e.call(GUIDE_COPY_CONSTRUCT, &args![outgoing, temp]); // 00b26ff1
            let count = element_count(first, last_guide, 0x38) as i32;
            fn_00b26b50(e, first, 0, count, outgoing.cast(), comparison); // 00b27014: the area itself
        });
        e.call(VERTEX_VECTOR_TIDY, &args![temp]);
        let proxy = e.mem.u32(temp);
        e.call(OPERATOR_DELETE, &args![proxy]);
    });
}

// Translated from 00b27060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the byte at `+0x30` of its `this` (`fastcall`; the Xbox PDB's `CSimpleBillboard`
/// has `m_bValid` there, but the engine map gives the function no name, so only the body is
/// asserted). Returns `this`.
pub fn fn_00b27060(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr() + 0x30, 0);
    this
}

// Translated from 00b27080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the four static billboard direction vectors from a direction (`cdecl`, one argument,
/// a pointer to three floats; the engine map gives it no name): the angle is
/// `atan2(direction.y, direction.x)` (`00ec9760`) rounded to a float and multiplied by the
/// double at `[010a4d90]` (57.29578; the exe stores and reloads the float, which changes
/// nothing for a float result). The static vector table the function fills on its first run
/// (guard bits 0 to 4 at `[011f8d04]`, each block run once: four unit-ish vectors at
/// `011f8cf8`, `011f8cec`, `011f8ce0` and `011f8cd4`, the table of four zeroed `CVec3`s at
/// `011f8ca4`) is copied into that table (`011f8ca4` to `011f8cd0`), an identity matrix is
/// rotated by the angle (`00b273d0`) and each of the four vectors is multiplied by it
/// (`00b12890`), the results stored to the four vectors at `011f8d08`, `011f8d14`, `011f8d20`
/// and `011f8d2c`.
pub fn fn_00b27080(e: &mut Engine, direction: Ptr<CVec3>) {
    let d = direction.addr();
    // The direction's floats are copied through the x87 stack into locals (the third is not used).
    let x = x87_quiet(e.mem.f32(d));
    let y = x87_quiet(e.mem.f32(d + 4));
    let angle = e.call(ATAN2, &args![f64::from(y), f64::from(x)]).f64();
    let angle_float = x87_load32(x87_store32(angle));
    let angle_float = x87_load32(x87_store32(angle_float));
    let scale: f64 = e.global(ANGLE_DIVISOR);
    let angle = f32::from_bits(x87_store32(x87_mul(angle_float, scale)));
    let flags = e.mem.u32(BILLBOARD_GUARD);
    if flags & 1 == 0 {
        e.mem
            .set_u32(BILLBOARD_GUARD, e.mem.u32(BILLBOARD_GUARD) | 1);
        e.mem.set_f32(BILLBOARD_VECTOR_A, 0.0);
        copy_float(e, BILLBOARD_VECTOR_A + 4, HALF_FLOAT);
        e.mem.set_f32(BILLBOARD_VECTOR_A + 8, 1.0);
    }
    if e.mem.u32(BILLBOARD_GUARD) & 2 == 0 {
        e.mem
            .set_u32(BILLBOARD_GUARD, e.mem.u32(BILLBOARD_GUARD) | 2);
        e.mem.set_f32(BILLBOARD_VECTOR_B, 0.0);
        copy_float(e, BILLBOARD_VECTOR_B + 4, NEGATIVE_HALF_FLOAT);
        e.mem.set_f32(BILLBOARD_VECTOR_B + 8, 1.0);
    }
    if e.mem.u32(BILLBOARD_GUARD) & 4 == 0 {
        e.mem
            .set_u32(BILLBOARD_GUARD, e.mem.u32(BILLBOARD_GUARD) | 4);
        e.mem.set_f32(BILLBOARD_VECTOR_C, 0.0);
        copy_float(e, BILLBOARD_VECTOR_C + 4, NEGATIVE_HALF_FLOAT);
        e.mem.set_f32(BILLBOARD_VECTOR_C + 8, 0.0);
    }
    if e.mem.u32(BILLBOARD_GUARD) & 8 == 0 {
        e.mem
            .set_u32(BILLBOARD_GUARD, e.mem.u32(BILLBOARD_GUARD) | 8);
        e.mem.set_f32(BILLBOARD_VECTOR_D, 0.0);
        copy_float(e, BILLBOARD_VECTOR_D + 4, HALF_FLOAT);
        e.mem.set_f32(BILLBOARD_VECTOR_D + 8, 0.0);
    }
    if e.mem.u32(BILLBOARD_GUARD) & 0x10 == 0 {
        e.mem
            .set_u32(BILLBOARD_GUARD, e.mem.u32(BILLBOARD_GUARD) | 0x10);
        // Four vectors of the table, three floats each, zeroed one at a time.
        let mut count = 4i32;
        let mut vector = BILLBOARD_TABLE;
        loop {
            count -= 1;
            if count < 0 {
                break;
            }
            e.mem.set_f32(vector + 8, 0.0);
            e.mem.set_f32(vector + 4, 0.0);
            e.mem.set_f32(vector, 0.0);
            vector = vector.wrapping_add(0xC);
        }
    }
    let sources = [
        BILLBOARD_VECTOR_A,
        BILLBOARD_VECTOR_B,
        BILLBOARD_VECTOR_C,
        BILLBOARD_VECTOR_D,
    ];
    for (i, source) in sources.iter().enumerate() {
        for axis in [0u32, 4, 8] {
            copy_float(e, BILLBOARD_TABLE + 0xC * i as u32 + axis, source + axis);
        }
    }
    e.with_stack(0x24, |e, matrix| {
        let m = matrix.addr();
        for (i, value) in [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(m + 4 * i as u32, *value);
        }
        fn_00b273d0(e, matrix.cast(), x87_quiet(angle));
        for i in 0..4u32 {
            let source = BILLBOARD_TABLE + 0xC * i;
            let product = e.with_stack(0xC, |e, slot| {
                let result = e.call(VEC3_TIMES_ROTATION, &args![source, slot, m]).u32();
                [
                    e.mem.u32(result),
                    e.mem.u32(result + 4),
                    e.mem.u32(result + 8),
                ]
            });
            for (axis, word) in product.iter().enumerate() {
                e.mem
                    .set_u32(BILLBOARD_RESULTS + 0xC * i + 4 * axis as u32, *word);
            }
        }
    });
}

// Translated from 00b273d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rotates a matrix about the z axis (the engine map gives it no name; `thiscall`, one float
/// argument, `RET 4`): builds a matrix with `cos`, `sin` of `angle / [010a4d90]` (each rounded to
/// a float, through the x87 stack) as the rows `(cos, sin, 0)`, `(-sin, cos, 0)` and
/// `(0, 0, 1)` over a freshly constructed identity (`00b12af0`), multiplies it with the matrix
/// at `this` (`00b12940`, result in a local) and copies the nine words of the product to `this`.
pub fn fn_00b273d0(e: &mut Engine, this: Ptr<CRotTransform>, angle: f32) {
    e.with_stack(0x24, |e, rotation| {
        let m = rotation.addr();
        e.call(CROT_TRANSFORM_CONSTRUCT, &args![m]);
        let cosine = x87_quiet(scaled_trig(e, COS, angle));
        let sine = x87_quiet(scaled_trig(e, SIN, angle));
        e.mem.set_f32(m, cosine);
        e.mem.set_f32(m + 4, sine);
        e.mem.set_f32(m + 8, 0.0);
        e.mem
            .set_u32(m + 12, x87_store32(-x87_load32(sine.to_bits())));
        e.mem.set_f32(m + 16, cosine);
        e.mem.set_f32(m + 20, 0.0);
        e.mem.set_f32(m + 24, 0.0);
        e.mem.set_f32(m + 28, 0.0);
        e.mem.set_f32(m + 32, 1.0);
        e.with_stack(0x24, |e, product| {
            let result = e
                .call(CROT_TRANSFORM_MULTIPLY, &args![m, product, this])
                .u32();
            copy_words(e, this.addr(), result, 9);
        });
    });
}

// Translated from 00b274a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CLeafGeometry::SLodGeometry::SLodGeometry` (Xbox PDB layout; `fastcall`, `this` only):
/// constructs the `SLeaf` base (`00b02c90`), then clears `m_bValid` (`+0x3C`) and
/// `m_pOrigCenterCoords` (`+0x40`). Returns `this`.
pub fn fn_00b274a0(e: &mut Engine, this: Ptr<SLodGeometry>) -> Ptr<SLodGeometry> {
    e.call(LEAF_BASE_CONSTRUCT, &args![this]);
    e.set(this, SLodGeometry::m_bValid, 0);
    e.set(this, SLodGeometry::m_pOrigCenterCoords, Ptr::NULL);
    this
}

// Translated from 00b274d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the arrays of one `CLeafGeometry::SLodGeometry` (Xbox PDB layout; `fastcall`, `this`
/// only): the pointers at `+0x10`, `+0x14`, `+0x18`, `+0x24`, `+0x28`, `+0x2C`, `+0x30`,
/// `+0x1C`, `+0x20`, `+0x34` and `+0x38` are released with `operator delete` and cleared, in
/// that order (all released first, then all cleared), then the original centre coordinates
/// at `+0x40` are released and cleared and the `SLeaf` base destructor (`00b02d40`) runs.
pub fn fn_00b274d0(e: &mut Engine, this: Ptr<SLodGeometry>) {
    let t = this.addr();
    for offset in LOD_ARRAYS {
        let array = e.mem.u32(t + offset);
        e.call(OPERATOR_DELETE, &args![array]);
    }
    for offset in LOD_ARRAYS {
        e.mem.set_u32(t + offset, 0);
    }
    let original = e.mem.u32(t + 0x40);
    e.call(OPERATOR_DELETE, &args![original]);
    e.mem.set_u32(t + 0x40, 0);
    e.call(LEAF_BASE_DESTRUCT, &args![this]);
}

/// The array pointers of an `SLodGeometry` that `00b274d0` and `00b28ed0` release, in the
/// order the exe releases them (the original centre coordinates at `+0x40` come last).
const LOD_ARRAYS: [u32; 11] = [
    0x10, 0x14, 0x18, 0x24, 0x28, 0x2C, 0x30, 0x1C, 0x20, 0x34, 0x38,
];

// Translated from 00b27660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CLeafGeometry::CLeafGeometry(CWindEngine *)` (Xbox PDB layout; `thiscall`, one argument,
/// `RET 4`): clears the manual-lighting and vertex-weighting bytes, stores the wind engine, sets
/// the rocking group count to 3 and clears the time offsets, leaf vertex, texture coordinate and
/// vertex program table pointers, the texture count, the leaf dimension and origin pointers,
/// the LOD count and the LOD pointer. Returns `this`.
pub fn fn_00b27660(
    e: &mut Engine,
    this: Ptr<CLeafGeometry>,
    wind_engine: Ptr,
) -> Ptr<CLeafGeometry> {
    e.set(this, CLeafGeometry::m_bManualLighting, 0);
    e.set(this, CLeafGeometry::m_bVertexWeighting, 0);
    e.set(this, CLeafGeometry::m_pWindEngine, wind_engine);
    e.set(this, CLeafGeometry::m_usNumRockingGroups, 3);
    e.set(this, CLeafGeometry::m_pTimeOffsets, Ptr::NULL);
    e.set(this, CLeafGeometry::m_pLeafVertexes, Ptr::NULL);
    e.set(this, CLeafGeometry::m_pLeafTexCoords, Ptr::NULL);
    e.set(this, CLeafGeometry::m_pVertexProgramTable, Ptr::NULL);
    e.set(this, CLeafGeometry::m_usNumTextures, 0);
    e.set(this, CLeafGeometry::m_pLeafDimensions, Ptr::NULL);
    e.set(this, CLeafGeometry::m_pLeafOrigins, Ptr::NULL);
    e.set(this, CLeafGeometry::m_usNumLods, 0);
    e.set(this, CLeafGeometry::m_pLods, Ptr::NULL);
    this
}

// Translated from 00b276f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CLeafGeometry` clean-up (the engine map gives it no name; `fastcall`, `this` only): releases
/// the leaf dimensions (`+0x20`) and origins (`+0x24`) with `operator delete`; when the LOD array
/// (`+0x2C`) exists, runs its vector deleting destructor (`00b277b0`, flag 3: destroy the
/// 0x44-byte elements and free the block, so the array's own storage is gone); releases the
/// vertex program table (`+0x18`); then clears the time offsets, texture coordinate, dimension,
/// origin, LOD and vertex program table pointers (`+0xC`, `+0x14`, `+0x20`, `+0x24`, `+0x2C`,
/// `+0x18`; `+0x10`, the leaf vertex table, is not touched).
pub fn fn_00b276f0(e: &mut Engine, this: Ptr<CLeafGeometry>) {
    let t = this.addr();
    let dimensions = e.mem.u32(t + 0x20);
    e.call(OPERATOR_DELETE, &args![dimensions]);
    let origins = e.mem.u32(t + 0x24);
    e.call(OPERATOR_DELETE, &args![origins]);
    let lods = e.mem.u32(t + 0x2C);
    if lods != 0 {
        e.call(LOD_VECTOR_DELETING_DESTRUCTOR, &args![lods, 3u32]);
    }
    let table = e.mem.u32(t + 0x18);
    e.call(OPERATOR_DELETE, &args![table]);
    for offset in [0x0Cu32, 0x14, 0x20, 0x24, 0x2C, 0x18] {
        e.mem.set_u32(t + offset, 0);
    }
}

// Translated from 00b277b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The vector deleting destructor of the 0x44-byte `SLodGeometry` (`thiscall`, one argument,
/// `RET 4`): with bit 1 of `flags` set the array's element count (the word before the block) is
/// handed to the CRT's vector destructor iterator (`00ec5fce`: block, 0x44, count,
/// `00b274d0`) and, with bit 0 also set, the block `this - 4` is freed; the result is
/// `this - 4`. Without bit 1 the destructor `00b274d0` runs on the one object and, with bit 0
/// set, the object is freed; the result is `this`.
pub fn fn_00b277b0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let t = this.addr();
    if flags & 2 != 0 {
        let count = e.mem.u32(t.wrapping_sub(4));
        e.call(
            VECTOR_DESTRUCTOR_ITERATOR,
            &args![t, 0x44u32, count, LOD_DESTRUCTOR],
        );
        if flags & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![t.wrapping_sub(4)]);
        }
        Ptr::new(t.wrapping_sub(4))
    } else {
        fn_00b274d0(e, this.cast());
        if flags & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![t]);
        }
        this
    }
}

// Translated from 00b27820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks every LOD of a `CLeafGeometry` as not valid (the engine map gives it no name;
/// `fastcall`, `this` only): with a LOD array (`+0x2C`), clears `m_bValid` (`+0x3C`) of each of
/// the `m_usNumLods` (`+0x28`) elements.
pub fn fn_00b27820(e: &mut Engine, this: Ptr<CLeafGeometry>) {
    if e.get(this, CLeafGeometry::m_pLods).is_null() {
        return;
    }
    let mut index = 0i32;
    while index < i32::from(e.get(this, CLeafGeometry::m_usNumLods)) {
        let lod = (index.wrapping_mul(0x44) as u32)
            .wrapping_add(e.get(this, CLeafGeometry::m_pLods).addr());
        e.mem.set_u8(lod + 0x3C, 0);
        index = index.wrapping_add(1);
    }
}

// Translated from 00b27870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the leaf vertex table into the vertex program table (the engine map gives it no name;
/// `thiscall`, one argument, `RET 4`): with a vertex program table (`+0x18`), a leaf vertex
/// pointer table (`+0x10`) whose first entry is not null, the table is filled with
/// `m_usNumTextures * m_usNumRockingGroups` blocks of 128 bytes copied (`00ec44d0`) from that
/// first entry, and the word count to the end of the copy (`(end - table) >> 2`) is stored
/// to `*out`. Returns the table (`+0x18`).
pub fn fn_00b27870(e: &mut Engine, this: Ptr<CLeafGeometry>, out: Ptr) -> u32 {
    let table = e.get(this, CLeafGeometry::m_pVertexProgramTable).addr();
    let blocks = u32::from(e.get(this, CLeafGeometry::m_usNumTextures))
        .wrapping_mul(u32::from(e.get(this, CLeafGeometry::m_usNumRockingGroups)));
    let words = blocks << 1 << 2 << 2;
    let vertexes = e.get(this, CLeafGeometry::m_pLeafVertexes).addr();
    if table != 0
        && vertexes != 0
        && e.mem.u32(vertexes) != 0
        && e.get(this, CLeafGeometry::m_pVertexProgramTable).addr() != 0
    {
        let source = e.mem.u32(vertexes);
        e.call(MEMCPY, &args![table, source, words << 2]);
        let end = table.wrapping_add(words.wrapping_mul(4));
        let table_now = e.get(this, CLeafGeometry::m_pVertexProgramTable).addr();
        e.mem
            .set_u32(out.addr(), (end.wrapping_sub(table_now) as i32 >> 2) as u32);
    }
    e.get(this, CLeafGeometry::m_pVertexProgramTable).addr()
}

// Translated from 00b28280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Transforms the leaf geometry by a matrix (the engine map gives it no name; `thiscall`, one
/// argument, `RET 4`: a pointer to a matrix whose first float is also the uniform scale): with
/// LODs (`+0x2C`) and leaf dimensions (`+0x20`), every leaf centre of every LOD (`+0x18` of the
/// LOD, three floats each, copied through the x87 stack) is multiplied by the matrix
/// (`00b05000`) and copied back (`memcpy`, 12 bytes); with vertex weighting (byte `+1`) the
/// original centres (`+0x40` of the LOD) likewise. Then each leaf dimension vector (`+0x20`,
/// one per texture) is scaled by the matrix's first float (`00b28440`).
pub fn fn_00b28280(e: &mut Engine, this: Ptr<CLeafGeometry>, matrix: Ptr) {
    if e.get(this, CLeafGeometry::m_pLods).is_null()
        || e.get(this, CLeafGeometry::m_pLeafDimensions).is_null()
    {
        return;
    }
    let mut lod_index = 0i32;
    while lod_index < i32::from(e.get(this, CLeafGeometry::m_usNumLods)) {
        let lod = (lod_index.wrapping_mul(0x44) as u32)
            .wrapping_add(e.get(this, CLeafGeometry::m_pLods).addr());
        let mut leaf = 0i32;
        while leaf < i32::from(e.mem.u16(lod + 0x0C)) {
            let center =
                (leaf.wrapping_mul(3).wrapping_mul(4) as u32).wrapping_add(e.mem.u32(lod + 0x18));
            transform_leaf_vector(e, center, matrix);
            if e.get(this, CLeafGeometry::m_bVertexWeighting) != 0 {
                let original = (leaf.wrapping_mul(3).wrapping_mul(4) as u32)
                    .wrapping_add(e.mem.u32(lod + 0x40));
                transform_leaf_vector(e, original, matrix);
            }
            leaf = leaf.wrapping_add(1);
        }
        lod_index = lod_index.wrapping_add(1);
    }
    let scale = x87_quiet(e.mem.f32(matrix.addr()));
    let mut texture = 0i32;
    while texture < i32::from(e.get(this, CLeafGeometry::m_usNumTextures)) {
        let dimension = (texture.wrapping_mul(0xC) as u32)
            .wrapping_add(e.get(this, CLeafGeometry::m_pLeafDimensions).addr());
        fn_00b28440(e, Ptr::new(dimension), x87_quiet(scale));
        texture = texture.wrapping_add(1);
    }
}

/// Multiplies the three floats at `vector` by the matrix at `matrix` the way `00b28280` does
/// (twice through the x87 stack into a local, `00b05000`, the product copied to the local
/// word by word) and copies the local back with `memcpy`.
fn transform_leaf_vector(e: &mut Engine, vector: u32, matrix: Ptr) {
    e.with_stack(0x18, |e, frame| {
        let local = frame.addr();
        let slot = local + 0x0C;
        for axis in [0u32, 4, 8] {
            let value = x87_quiet_bits(x87_quiet_bits(e.mem.u32(vector + axis)));
            e.mem.set_u32(local + axis, value);
        }
        let result = e.call(VEC3_TIMES_MATRIX, &args![local, slot, matrix]).u32();
        for axis in [0u32, 4, 8] {
            let word = e.mem.u32(result + axis);
            e.mem.set_u32(local + axis, word);
        }
        e.call(MEMCPY, &args![vector, local, 0xCu32]);
    });
}

// Translated from 00b28440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scales the three floats at `this` by `scalar` (`CVec3::operator*=`-like; the engine map gives
/// it no name; `thiscall`, one float argument, `RET 4`): each component is loaded
/// (`FLD`), multiplied by the float operand and stored. Returns `this`.
pub fn fn_00b28440(e: &mut Engine, this: Ptr<CVec3>, scalar: f32) -> Ptr<CVec3> {
    let t = this.addr();
    for axis in [0u32, 4, 8] {
        let product = x87_mul(x87_load32(e.mem.u32(t + axis)), x87_m32(scalar.to_bits()));
        e.mem.set_u32(t + axis, x87_store32(product));
    }
    this
}

// Translated from 00b28480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Takes the leaf data of a tree's leaf engine (the engine map gives it no name; `thiscall`,
/// three arguments, `RET 0xC`: the LOD count, a second value passed on to `00b29380`, and the
/// source object): copies the rocking group count (word at `source + 0x40` to `+8`) and the
/// words at `source + 0x54`, `+0x4C`, `+0x50` to `+0xC`, `+0x10`, `+0x14`; the texture count
/// (`+0x1C`) is the element count of the source's 0x54-byte element vector at `source + 0x10`;
/// it allocates (`operator new`, an overflowing array size becomes `0xFFFFFFFF`) and zeroes two
/// arrays of that many `CVec3` (`+0x20`, `+0x24`), fills them from `source`'s elements (the
/// three words at `element + 0x48` and `element + 0x30`, with the vector's range check), and
/// allocates the vertex program table of `textures * rocking groups * 32` words (`+0x18`); then
/// calls `00b29380` with the LOD count and the second value.
pub fn fn_00b28480(
    e: &mut Engine,
    this: Ptr<CLeafGeometry>,
    lod_count: u16,
    second: u32,
    source: Ptr,
) {
    let t = this.addr();
    let s = source.addr();
    let groups = e.mem.u16(s + 0x40);
    e.set(this, CLeafGeometry::m_usNumRockingGroups, groups);
    let word = e.mem.u32(s + 0x54);
    e.mem.set_u32(t + 0x0C, word);
    let word = e.mem.u32(s + 0x4C);
    e.mem.set_u32(t + 0x10, word);
    let word = e.mem.u32(s + 0x50);
    e.mem.set_u32(t + 0x14, word);
    let elements = s + 0x10;
    let textures = (e
        .mem
        .u32(elements + 0x10)
        .wrapping_sub(e.mem.u32(elements + 0x0C)) as i32
        / 0x54) as u16;
    e.set(this, CLeafGeometry::m_usNumTextures, textures);
    let count = u32::from(e.get(this, CLeafGeometry::m_usNumTextures));
    let dimensions = allocate_zeroed_vectors(e, count);
    e.set(this, CLeafGeometry::m_pLeafDimensions, Ptr::new(dimensions));
    let count = u32::from(e.get(this, CLeafGeometry::m_usNumTextures));
    let origins = allocate_zeroed_vectors(e, count);
    e.set(this, CLeafGeometry::m_pLeafOrigins, Ptr::new(origins));
    let mut index = 0i32;
    while index < i32::from(e.get(this, CLeafGeometry::m_usNumTextures)) {
        let element = vector_at(e, elements, index as u32, 0x54);
        let target = (index.wrapping_mul(0xC) as u32)
            .wrapping_add(e.get(this, CLeafGeometry::m_pLeafDimensions).addr());
        copy_range_words(e, target, element + 0x48, 3);
        let element = vector_at(e, elements, index as u32, 0x54);
        let target = (index.wrapping_mul(0xC) as u32)
            .wrapping_add(e.get(this, CLeafGeometry::m_pLeafOrigins).addr());
        copy_range_words(e, target, element + 0x30, 3);
        index = index.wrapping_add(1);
    }
    let words = u32::from(e.get(this, CLeafGeometry::m_usNumTextures))
        .wrapping_mul(u32::from(e.get(this, CLeafGeometry::m_usNumRockingGroups)))
        << 1
        << 2
        << 2;
    let (bytes, overflow) = words.overflowing_mul(4);
    let size = if overflow { u32::MAX } else { bytes };
    let table = e.call(OPERATOR_NEW, &args![size]).u32();
    e.set(this, CLeafGeometry::m_pVertexProgramTable, Ptr::new(table));
    fn_00b29380(e, this, lod_count, second);
}

/// The `new CVec3[count]` of `00b28480`: `operator new` of `count * 12` bytes (`0xFFFFFFFF` when
/// the multiplication overflows) and, when it gave memory, three float zeros per element
/// (written highest first as the exe does); returns the block, or zero.
fn allocate_zeroed_vectors(e: &mut Engine, count: u32) -> u32 {
    let (bytes, overflow) = count.overflowing_mul(0xC);
    let size = if overflow { u32::MAX } else { bytes };
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    let mut left = count as i32;
    let mut vector = block;
    loop {
        left = left.wrapping_sub(1);
        if left < 0 {
            break;
        }
        e.mem.set_f32(vector + 8, 0.0);
        e.mem.set_f32(vector + 4, 0.0);
        e.mem.set_f32(vector, 0.0);
        vector = vector.wrapping_add(0xC);
    }
    block
}

// Translated from 00b28720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the texture coordinates of one leaf into the two texture coordinate blocks the vertex
/// program table keeps (the engine map gives it no name; `thiscall`, two arguments, `RET 8`: the
/// index and a pointer to eight floats): with a table (`+0x14`) and a source, the sign is `1.0`,
/// or the float at `[01012054]` when `00b0d7a0` says to flip. The block at `index * 0x40`
/// bytes gets `s0, sign*t0, s1, sign*t1, s2, sign*t2, s3, sign*t3` (the floats at
/// `source + 0, 4, ..., 0x1C`: even ones copied, odd ones multiplied by the sign), the block 32
/// bytes after it the same with the pairs swapped as `s1, sign*t0, s0, sign*t1, s3, sign*t2,
/// s2, sign*t3` (source words 2, 1, 0, 3, 6, 5, 4, 7).
pub fn fn_00b28720(e: &mut Engine, this: Ptr<CLeafGeometry>, index: u32, source: Ptr) {
    if e.mem.u32(this.addr() + 0x14) == 0 || source.is_null() {
        return;
    }
    let flipped = e.call(FLIP_TEXTURE_V, &args![]).bool();
    let sign = if flipped {
        x87_quiet(e.mem.f32(FLIP_SIGN))
    } else {
        1.0f32
    };
    let sign = x87_quiet(sign);
    let s = source.addr();
    let table = e.mem.u32(this.addr() + 0x14);
    let first = (index << 1 << 3).wrapping_mul(4).wrapping_add(table);
    let layout = [
        (0u32, false),
        (1, true),
        (2, false),
        (3, true),
        (4, false),
        (5, true),
        (6, false),
        (7, true),
    ];
    for (position, (word, multiplied)) in layout.iter().enumerate() {
        let target = first + 4 * position as u32;
        if *multiplied {
            let product = x87_mul(x87_load32(sign.to_bits()), x87_m32(e.mem.u32(s + 4 * word)));
            e.mem.set_u32(target, x87_store32(product));
        } else {
            copy_float(e, target, s + 4 * word);
        }
    }
    let second = ((index << 1).wrapping_mul(8).wrapping_add(8))
        .wrapping_mul(4)
        .wrapping_add(table);
    let swapped = [
        (2u32, false),
        (1, true),
        (0, false),
        (3, true),
        (6, false),
        (5, true),
        (4, false),
        (7, true),
    ];
    for (position, (word, multiplied)) in swapped.iter().enumerate() {
        let target = second + 4 * position as u32;
        if *multiplied {
            let product = x87_mul(x87_load32(sign.to_bits()), x87_m32(e.mem.u32(s + 4 * word)));
            e.mem.set_u32(target, x87_store32(product));
        } else {
            copy_float(e, target, s + 4 * word);
        }
    }
}

/// The result of one `FMUL float ptr [..]` or `FADD float ptr [..]` style step, `FSTP dword`:
/// the value `a op b` rounded to a float, as its bits.
fn store(value: f64) -> u32 {
    x87_store32(value)
}

// Translated from 00b288f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the leaf corner offsets of one LOD (the engine map gives it no name; `thiscall`, five
/// arguments, `RET 0x14`: the output structure, the LOD index, two words that are not read and a
/// float step): needs the LOD array (`+0x2C`), the index below the LOD count, the leaf vertex
/// table (`+0x10`), the origins (`+0x24`) and dimensions (`+0x20`), and the wind engine (`+4`)
/// and its value at `+0xC`; a LOD whose `m_bValid` (`+0x3C`) is clear is built: when the wind
/// engine's method (`[+4] + 0x10`) is 1 the LOD's centres are first wind-blended (`00b29190`);
/// the scale is 1, or `lod * step + 1` for a LOD above zero; for each of the `2 * textures`
/// corner sets, the origin of the texture `idx / 2` (the first float is `1 - it` for even
/// `idx`) and its dimensions give four corner vectors in the exe's order of float operations
/// (`(1 - o) * scale * d`, `-(o * d) * scale`, as `(0, a, b)`, `(0, -(o0 d0) s, b)`,
/// `(0, -(o0 d0) s, -(o1 d1) s)` and `(0, a, -(o1 d1) s)`); the four are written (`memcpy`, 12
/// bytes each, 16 bytes apart) for each rocking group `0..[+8]` into the LOD's block of the leaf
/// vertex table (`[+0x10] + lod * 4`, block `idx * groups * 64` bytes). The LOD is marked valid and
/// its 15 words copied to the output structure, whose first byte is set to 1 and its third word to
/// the LOD index.
pub fn fn_00b288f0(
    e: &mut Engine,
    this: Ptr<CLeafGeometry>,
    out: Ptr,
    lod: u16,
    _unused_3: u32,
    _unused_4: u32,
    step: f32,
) {
    let t = this.addr();
    let lod_index = u32::from(lod);
    if e.get(this, CLeafGeometry::m_pLods).is_null()
        || lod_index >= u32::from(e.get(this, CLeafGeometry::m_usNumLods))
        || e.get(this, CLeafGeometry::m_pLeafVertexes).is_null()
        || e.get(this, CLeafGeometry::m_pLeafOrigins).is_null()
        || e.get(this, CLeafGeometry::m_pLeafDimensions).is_null()
        || e.get(this, CLeafGeometry::m_pWindEngine).is_null()
        || e.mem.u32(t + 0x0C) == 0
    {
        return;
    }
    let lod_struct = lod_index
        .wrapping_mul(0x44)
        .wrapping_add(e.get(this, CLeafGeometry::m_pLods).addr());
    if e.mem.u8(lod_struct + 0x3C) == 0 {
        let wind_engine = e.get(this, CLeafGeometry::m_pWindEngine).addr();
        if e.mem.u32(wind_engine + 0x10) == 1 {
            fn_00b29190(e, this, lod);
        }
        let mut scale = 1.0f32.to_bits();
        if lod > 0 {
            // `FILD` of the LOD index, `FMUL float [step]`, `FADD 1.0`.
            let scaled = x87_mul(f64::from(lod), x87_m32(step.to_bits()));
            scale = store(x87_add(scaled, 1.0));
        }
        let groups = u32::from(e.get(this, CLeafGeometry::m_usNumRockingGroups));
        let mut corner_set = 0i32;
        while corner_set < i32::from(e.get(this, CLeafGeometry::m_usNumTextures)) << 1 {
            let texture = (corner_set / 2).wrapping_mul(0xC) as u32;
            let origin_at = e
                .get(this, CLeafGeometry::m_pLeafOrigins)
                .addr()
                .wrapping_add(texture);
            let mut origin = [
                e.mem.u32(origin_at),
                e.mem.u32(origin_at + 4),
                e.mem.u32(origin_at + 8),
            ];
            if corner_set & 1 == 0 {
                // `FLD o0; FLD1; FSUBRP; FSTP; FLD; FSTP`: `1 - o0` through a float local.
                origin[0] = x87_quiet_bits(store(x87_sub(1.0, x87_load32(origin[0]))));
            }
            let dimension_at = e
                .get(this, CLeafGeometry::m_pLeafDimensions)
                .addr()
                .wrapping_add(texture);
            let (d0, d1) = (e.mem.u32(dimension_at), e.mem.u32(dimension_at + 4));
            let one_minus = |bits: u32| x87_sub(1.0, x87_load32(bits));
            // `(1 - o) * scale * d` and `-(o * d) * scale`, one float store each.
            let a = store(x87_mul(
                x87_mul(one_minus(origin[0]), x87_m32(scale)),
                x87_m32(d0),
            ));
            let b = store(x87_mul(
                x87_mul(one_minus(origin[1]), x87_m32(scale)),
                x87_m32(d1),
            ));
            let zero = 0.0f32.to_bits();
            let f = store(x87_mul(
                -x87_mul(x87_load32(origin[0]), x87_m32(d0)),
                x87_m32(scale),
            ));
            let h = store(x87_mul(
                -x87_mul(x87_load32(origin[1]), x87_m32(d1)),
                x87_m32(scale),
            ));
            let copy = |bits: u32| x87_quiet_bits(bits);
            let vectors = [
                [zero, copy(a), copy(b)],
                [zero, copy(f), copy(b)],
                [zero, copy(f), copy(h)],
                [zero, copy(a), copy(h)],
            ];
            let leaf_vertexes = e.get(this, CLeafGeometry::m_pLeafVertexes).addr();
            let mut group = 0i32;
            while group < i32::from(e.get(this, CLeafGeometry::m_usNumRockingGroups)) {
                let block = e
                    .mem
                    .u32(leaf_vertexes.wrapping_add(lod_index.wrapping_mul(4)));
                let base = block
                    .wrapping_add(
                        (groups << 4)
                            .wrapping_mul(corner_set as u32)
                            .wrapping_mul(4),
                    )
                    .wrapping_add(((group as u32) << 4).wrapping_mul(4));
                let mut target = base;
                for vector in &vectors {
                    e.with_stack(0xC, |e, local| {
                        for (axis, word) in vector.iter().enumerate() {
                            e.mem.set_u32(local.addr() + 4 * axis as u32, *word);
                        }
                        e.call(MEMCPY, &args![target, local, 0xCu32]);
                    });
                    target = target.wrapping_add(0x10);
                }
                group = group.wrapping_add(1);
            }
            corner_set = corner_set.wrapping_add(1);
        }
        e.mem.set_u8(lod_struct + 0x3C, 1);
    }
    copy_range_words(e, out.addr(), lod_struct, 15);
    e.mem.set_u8(out.addr(), 1);
    e.mem.set_u32(out.addr() + 8, lod_index);
}

// Translated from 00b28e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Hands out the data of one LOD (the engine map gives it no name; `thiscall`, five arguments,
/// `RET 0x14`: the output structure, the LOD index and three words that are not read): with the
/// same preconditions as `00b288f0` (the LOD array, the index below the LOD count, the leaf
/// vertex table, the origins and dimensions, the wind engine and the word at `+0xC`), the LOD's
/// 15 words are copied to the output structure, whose first byte is set to 1 and its third word
/// to the LOD index.
pub fn fn_00b28e40(
    e: &mut Engine,
    this: Ptr<CLeafGeometry>,
    out: Ptr,
    lod: u16,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let t = this.addr();
    let lod_index = u32::from(lod);
    if e.get(this, CLeafGeometry::m_pLods).is_null()
        || lod_index as i32 >= i32::from(e.get(this, CLeafGeometry::m_usNumLods))
        || e.get(this, CLeafGeometry::m_pLeafVertexes).is_null()
        || e.get(this, CLeafGeometry::m_pLeafOrigins).is_null()
        || e.get(this, CLeafGeometry::m_pLeafDimensions).is_null()
        || e.get(this, CLeafGeometry::m_pWindEngine).is_null()
        || e.mem.u32(t + 0x0C) == 0
    {
        return;
    }
    let lod_struct = lod_index
        .wrapping_mul(0x44)
        .wrapping_add(e.get(this, CLeafGeometry::m_pLods).addr());
    copy_range_words(e, out.addr(), lod_struct, 15);
    e.mem.set_u8(out.addr(), 1);
    e.mem.set_u32(out.addr() + 8, lod_index);
}

// Translated from 00b28ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the arrays of every LOD of a `CLeafGeometry` (the engine map gives it no name;
/// `fastcall`, `this` only): with a LOD array (`+0x2C`) and a nonzero LOD count (`+0x28`), for
/// each LOD the pointers at `+0x10`, `+0x14`, `+0x18`, `+0x24`, `+0x28`, `+0x2C`, `+0x30`,
/// `+0x1C`, `+0x20`, `+0x34` and `+0x38` are released with `operator delete` and cleared, in
/// that order (all released first), then the original centre coordinates at `+0x40` are
/// released and cleared.
pub fn fn_00b28ed0(e: &mut Engine, this: Ptr<CLeafGeometry>) {
    if e.get(this, CLeafGeometry::m_pLods).is_null() || e.get(this, CLeafGeometry::m_usNumLods) == 0
    {
        return;
    }
    let mut index = 0i32;
    while index < i32::from(e.get(this, CLeafGeometry::m_usNumLods)) {
        let lod = |e: &Engine| {
            (index.wrapping_mul(0x44) as u32)
                .wrapping_add(e.get(this, CLeafGeometry::m_pLods).addr())
        };
        for offset in LOD_ARRAYS {
            let array = e.mem.u32(lod(e) + offset);
            e.call(OPERATOR_DELETE, &args![array]);
        }
        for offset in LOD_ARRAYS {
            let at = lod(e) + offset;
            e.mem.set_u32(at, 0);
        }
        let original = e.mem.u32(lod(e) + 0x40);
        e.call(OPERATOR_DELETE, &args![original]);
        let at = lod(e) + 0x40;
        e.mem.set_u32(at, 0);
        index = index.wrapping_add(1);
    }
}

// Translated from 00b29190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Blends the centres of one LOD with the wind matrices (the engine map gives it no name;
/// `thiscall`, one argument, `RET 4`: the LOD index): needs the wind engine (`+4`), the index
/// below the LOD count, vertex weighting (byte `+1`), the LOD array and, in the LOD, the
/// original centres (`+0x40`) and the centres (`+0x18`). For each leaf, with the wind weight `w`
/// (`+0x34`), the original centre `c` (`+0x40`) multiplied by the wind matrix
/// `[011f8d88] + matrix_index * 0x40` (`+0x38` byte; `00b05000`) gives `v`; the centre becomes
/// `c + (v - c) * w` per axis in the exe's float order (`(v - c) * w + c`), written with
/// `memcpy`.
pub fn fn_00b29190(e: &mut Engine, this: Ptr<CLeafGeometry>, lod: u16) {
    let lod_index = u32::from(lod);
    if e.get(this, CLeafGeometry::m_pWindEngine).is_null()
        || lod_index as i32 >= i32::from(e.get(this, CLeafGeometry::m_usNumLods))
        || e.get(this, CLeafGeometry::m_bVertexWeighting) == 0
        || e.get(this, CLeafGeometry::m_pLods).is_null()
    {
        return;
    }
    let lod_struct = lod_index
        .wrapping_mul(0x44)
        .wrapping_add(e.get(this, CLeafGeometry::m_pLods).addr());
    if lod_struct == 0 || e.mem.u32(lod_struct + 0x40) == 0 || e.mem.u32(lod_struct + 0x18) == 0 {
        return;
    }
    let mut leaf = 0i32;
    while leaf < i32::from(e.mem.u16(lod_struct + 0x0C)) {
        let index = leaf as u32;
        let weight = x87_quiet_bits(
            e.mem
                .u32(e.mem.u32(lod_struct + 0x34).wrapping_add(index * 4)),
        );
        let center = e
            .mem
            .u32(lod_struct + 0x40)
            .wrapping_add(index.wrapping_mul(12));
        let c = [
            x87_quiet_bits(e.mem.u32(center)),
            x87_quiet_bits(e.mem.u32(center + 4)),
            x87_quiet_bits(e.mem.u32(center + 8)),
        ];
        let matrix_index = u32::from(e.mem.u8(e.mem.u32(lod_struct + 0x38).wrapping_add(index)));
        let matrix = (matrix_index << 6).wrapping_add(e.mem.u32(WIND_MATRICES));
        let v = e.with_stack(0x18, |e, frame| {
            let local = frame.addr();
            for (axis, bits) in c.iter().enumerate() {
                e.mem.set_u32(local + 4 * axis as u32, *bits);
            }
            let result = e
                .call(VEC3_TIMES_MATRIX, &args![local, local + 0xC, matrix])
                .u32();
            [
                e.mem.u32(result),
                e.mem.u32(result + 4),
                e.mem.u32(result + 8),
            ]
        });
        let blend = |v: u32, c: u32| {
            // `(v - c) * w + c` with the float operands taken from memory.
            store(x87_add(
                x87_mul(x87_sub(x87_load32(v), x87_m32(c)), x87_m32(weight)),
                x87_m32(c),
            ))
        };
        let z = blend(v[2], c[2]);
        let y = blend(v[1], c[1]);
        // `FLD w; FLD c0; FSUBR [v0]; FMULP; FADD c0`.
        let x = store(x87_add(
            x87_mul(x87_load32(weight), x87_sub(x87_m32(v[0]), x87_load32(c[0]))),
            x87_m32(c[0]),
        ));
        let result = [x87_quiet_bits(x), x87_quiet_bits(y), x87_quiet_bits(z)];
        e.with_stack(0xC, |e, local| {
            for (axis, bits) in result.iter().enumerate() {
                e.mem.set_u32(local.addr() + 4 * axis as u32, *bits);
            }
            let target = e
                .mem
                .u32(lod_struct + 0x18)
                .wrapping_add(index.wrapping_mul(12));
            e.call(MEMCPY, &args![target, local, 0xCu32]);
        });
        leaf = leaf.wrapping_add(1);
    }
}

// Translated from 00b29fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the lighting engine (the engine map gives it no name; the field offsets
/// match the Xbox PDB's `CLightingEngine`; `fastcall`, `this` only): the branch and leaf
/// lighting methods (`+0`, `+0x38`) are 0, the leaf lighting adjustment scalar (`+0x70`) is the
/// float at `[01016248]`, the static lighting style and frond method (`+0x74`, `+0x78`) are 0,
/// and the three 13-float material blocks (`+4`, `+0x3C`, `+0x7C`, `0x34` bytes each) are
/// copied (`memcpy`) from stack arrays: `[01030ff0]` three times, `[0101df44]` three times and
/// seven zeros; then `1.0` three times, `[01016248]` three times and seven zeros twice.
/// Returns `this`.
pub fn fn_00b29fd0(e: &mut Engine, this: Ptr) -> Ptr {
    let t = this.addr();
    e.mem.set_u32(t, 0);
    e.mem.set_u32(t + 0x38, 0);
    copy_float(e, t + 0x70, HALF_FLOAT);
    e.mem.set_u32(t + 0x74, 0);
    e.mem.set_u32(t + 0x78, 0);
    let first = x87_quiet_bits(e.mem.u32(MATERIAL_FIRST));
    let second = x87_quiet_bits(e.mem.u32(MATERIAL_SECOND));
    let half = x87_quiet_bits(e.mem.u32(HALF_FLOAT));
    let one = 1.0f32.to_bits();
    let zero = 0.0f32.to_bits();
    let branch = [
        first, first, first, second, second, second, zero, zero, zero, zero, zero, zero, zero,
    ];
    let other = [
        one, one, one, half, half, half, zero, zero, zero, zero, zero, zero, zero,
    ];
    e.with_stack(0x9C, |e, frame| {
        let f = frame.addr();
        for (i, word) in branch.iter().enumerate() {
            e.mem.set_u32(f + 4 * i as u32, *word);
        }
        for (i, word) in other.iter().enumerate() {
            e.mem.set_u32(f + 0x34 + 4 * i as u32, *word);
            e.mem.set_u32(f + 0x68 + 4 * i as u32, *word);
        }
        e.call(MEMCPY, &args![t + 4, f, 0x34u32]);
        e.call(MEMCPY, &args![t + 0x3C, f + 0x34, 0x34u32]);
        e.call(MEMCPY, &args![t + 0x7C, f + 0x68, 0x34u32]);
    });
    this
}

// Translated from 00b2a180 (decompiled, FalloutNV.exe 1.4.0.525)
/// The dot product of two three-float vectors (`CVec3`; the engine map gives it no name;
/// `thiscall`, one argument, `RET 4`; the result in `ST0`): `a.x * b.x + a.y * b.y + a.z * b.z`
/// in the exe's order (each product `FLD` times a float memory operand, the sum left to
/// right), stored to a float local and loaded again.
pub fn fn_00b2a180(e: &mut Engine, this: Ptr<CVec3>, other: Ptr<CVec3>) -> f32 {
    let (a, b) = (this.addr(), other.addr());
    let product = |e: &Engine, axis: u32| {
        x87_mul(
            x87_load32(e.mem.u32(a + axis)),
            x87_m32(e.mem.u32(b + axis)),
        )
    };
    let sum = x87_add(product(e, 0), product(e, 4));
    let sum = x87_add(sum, product(e, 8));
    f32::from_bits(x87_quiet_bits(store(sum)))
}

/// `CVec3::Distance(this, other)` (`00b04d40`, `thiscall`, `RET 4`; the distance in `ST0`) of the
/// vector at `vector` to the vector built from `point`, which is written to `scratch` first (the
/// exe builds the point on its stack just before the call).
fn distance_to(e: &mut Engine, vector: u32, scratch: u32, point: [u32; 3]) -> f64 {
    for (axis, word) in point.iter().enumerate() {
        e.mem.set_u32(scratch + 4 * axis as u32, *word);
    }
    e.call(VEC3_DISTANCE, &args![vector, scratch]).f64()
}

// Translated from 00b27910 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b27910 - the FS:[0] exception frame (the handler that tidies the
// radius vector and the temporary extents if a call throws) is not translated.
/// Accumulates the extents of all leaves (the engine map gives it no name; `thiscall`, one
/// argument, `RET 4`: a pointer to the 12-word extents object that receives the result; the
/// calls `00b3c930`, `00b3caa0` and `00b3c9a0` are the extents' constructor, the expansion by a
/// point and the merge; `00b3caa0` and `00b3c9a0` return the 12-word product in a slot that
/// is copied back). With the leaf dimensions (`+0x20`), origins (`+0x24`) and LODs (`+0x2C`)
/// present: first, a radius per texture (`m_usNumTextures`, `+0x1C`) is pushed to a local
/// `std::vector<float>` (`00449fd0`, `006578e0`): the largest distance (`00b04d40`; each time
/// the float `max` is below it the distance is taken again and stored) from
/// `(dimension.x * origin.x, dimension.y * origin.y, 0)` to `(0, 0, 0)`, `(dimension.x, 0, 0)`,
/// `(dimension.x, dimension.y, 0)` and `(0, dimension.y, 0)`. Then for every leaf of every LOD
/// (the LOD's `+0x0C` leaves, centres at `+0x18`, texture bytes at `+0x10`): an extents is made
/// (`00b3c930`), the radius `r` of texture `byte / 2` is read (with the vector's range check),
/// the six points `centre + (+-r, 0, 0)`, `(0, +-r, 0)`, `(0, 0, +-r)` expand the extents
/// (`00b3caa0`) and the leaf's extents are merged into the output (`00b3c9a0`); the temporaries'
/// trivial destructors (`00aa7830`) run in the exe's order. The radius vector is tidied and its
/// proxy deleted at the end.
pub fn fn_00b27910(e: &mut Engine, this: Ptr<CLeafGeometry>, extents: Ptr) {
    if e.get(this, CLeafGeometry::m_pLeafOrigins).is_null()
        || e.get(this, CLeafGeometry::m_pLeafDimensions).is_null()
        || e.get(this, CLeafGeometry::m_pLods).is_null()
    {
        return;
    }
    e.with_stack(0x18, |e, radii| {
        let radii = radii.addr();
        e.call(FLOAT_VECTOR_CONSTRUCT, &args![radii]);
        let mut texture = 0i32;
        while texture < i32::from(e.get(this, CLeafGeometry::m_usNumTextures)) {
            let dimension = (texture.wrapping_mul(0xC) as u32)
                .wrapping_add(e.get(this, CLeafGeometry::m_pLeafDimensions).addr());
            let origin = (texture.wrapping_mul(0xC) as u32)
                .wrapping_add(e.get(this, CLeafGeometry::m_pLeafOrigins).addr());
            let a = x87_store32(x87_mul(
                x87_load32(e.mem.u32(dimension)),
                x87_m32(e.mem.u32(origin)),
            ));
            let b = x87_store32(x87_mul(
                x87_load32(e.mem.u32(dimension + 4)),
                x87_m32(e.mem.u32(origin + 4)),
            ));
            let (dx, dy) = (e.mem.u32(dimension), e.mem.u32(dimension + 4));
            let mut largest = 0.0f32.to_bits();
            e.with_stack(0x18, |e, frame| {
                let vector = frame.addr();
                let scratch = vector + 0xC;
                e.mem.set_u32(vector, x87_quiet_bits(a));
                e.mem.set_u32(vector + 4, x87_quiet_bits(b));
                e.mem.set_u32(vector + 8, 0.0f32.to_bits());
                let zero = 0.0f32.to_bits();
                let candidates = [
                    [zero, zero, zero],
                    [x87_quiet_bits(x87_quiet_bits(dx)), zero, zero],
                    [x87_quiet_bits(dx), x87_quiet_bits(dy), zero],
                    [zero, x87_quiet_bits(dy), zero],
                ];
                for point in candidates {
                    let first = distance_to(e, vector, scratch, point);
                    if x87_load32(largest) < first {
                        // The exe measures the same point again and keeps that result.
                        let again = distance_to(e, vector, scratch, point);
                        largest = x87_store32(again);
                    }
                }
            });
            e.with_stack(4, |e, value| {
                e.mem.set_u32(value.addr(), largest);
                e.call(FLOAT_VECTOR_PUSH, &args![radii, value]);
            });
            texture = texture.wrapping_add(1);
        }
        let mut lod_index = 0i32;
        while lod_index < i32::from(e.get(this, CLeafGeometry::m_usNumLods)) {
            let lod = (lod_index.wrapping_mul(0x44) as u32)
                .wrapping_add(e.get(this, CLeafGeometry::m_pLods).addr());
            let mut leaf = 0u32;
            while leaf < u32::from(e.mem.u16(lod + 0x0C)) {
                e.with_stack(0x30 * 2 + 0xC, |e, frame| {
                    let working = frame.addr();
                    let slot = working + 0x30;
                    let point = slot + 0x30;
                    e.call(EXTENTS_CONSTRUCT, &args![working]);
                    let center = e
                        .mem
                        .u32(lod + 0x18)
                        .wrapping_add(leaf.wrapping_mul(3).wrapping_mul(4));
                    let c = [
                        x87_quiet_bits(e.mem.u32(center)),
                        x87_quiet_bits(e.mem.u32(center + 4)),
                        x87_quiet_bits(e.mem.u32(center + 8)),
                    ];
                    let texture_byte =
                        u32::from(e.mem.u8(e.mem.u32(lod + 0x10).wrapping_add(leaf)));
                    let index = (texture_byte as i32 / 2) as u32;
                    let element = vector_at(e, radii, index, 4);
                    let r = x87_quiet_bits(e.mem.u32(element));
                    let plus = |bits: u32| x87_store32(x87_add(x87_load32(r), x87_m32(bits)));
                    let minus = |bits: u32| x87_store32(x87_sub(x87_m32(bits), x87_load32(r)));
                    let points = [
                        [plus(c[0]), c[1], c[2]],
                        [minus(c[0]), c[1], c[2]],
                        [c[0], plus(c[1]), c[2]],
                        [c[0], minus(c[1]), c[2]],
                        [c[0], c[1], plus(c[2])],
                        [c[0], c[1], minus(c[2])],
                    ];
                    for coordinates in points {
                        for (axis, word) in coordinates.iter().enumerate() {
                            e.mem.set_u32(point + 4 * axis as u32, *word);
                        }
                        let result = e.call(EXTENTS_EXPAND, &args![working, slot, point]).u32();
                        copy_words(e, working, result, 12);
                        e.call(TRIVIAL_DESTRUCTOR, &args![slot + 0x18]);
                        e.call(TRIVIAL_DESTRUCTOR, &args![slot]);
                    }
                    let merged = e.call(EXTENTS_MERGE, &args![extents, slot, working]).u32();
                    copy_words(e, extents.addr(), merged, 12);
                    e.call(TRIVIAL_DESTRUCTOR, &args![slot + 0x18]);
                    e.call(TRIVIAL_DESTRUCTOR, &args![slot]);
                    e.call(TRIVIAL_DESTRUCTOR, &args![working + 0x18]);
                    e.call(TRIVIAL_DESTRUCTOR, &args![working]);
                });
                leaf = leaf.wrapping_add(1);
            }
            lod_index = lod_index.wrapping_add(1);
        }
        e.call(FLOAT_VECTOR_TIDY, &args![radii]);
        let proxy = e.mem.u32(radii);
        e.call(OPERATOR_DELETE, &args![proxy]);
    });
}

// Translated from 00b29380 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b29380 - the FS:[0] exception frame (the unwinding of the LOD array
// if its construction throws) is not translated.
/// Builds the LOD arrays from the leaf lists (the engine map gives it no name; `thiscall`, two
/// arguments, `RET 8`: the LOD count and a pointer to that many `std::vector<CBillboardLeaf *>`,
/// 0x18 bytes each): needs the leaves, the leaf vertex table (`+0x10`), the wind engine (`+4`)
/// and the vertex program table (`+0x14`). The LOD count is stored (`+0x28`); a block of
/// `4 + count * 0x44` bytes is allocated (`0xFFFFFFFF` on overflow) holding the count and the
/// `SLodGeometry` elements, constructed with `00b274a0` (destructor `00b274d0`) by the CRT's
/// `eh vector constructor iterator` (`00ec782f`), and its elements' address stored at `+0x2C`.
/// For each LOD: the leaf count (`+0x0C`) is the vector's element count; the arrays are
/// allocated (`operator new`, 4 bytes per leaf, 12 per vector, an overflowing size `0xFFFFFFFF`)
/// and filled leaf by leaf, each with the vector's range check:
/// `+0x20` the address in the leaf vertex table (`table[lod] + (rocking groups * 16 * texture
/// index + angle index * 16) * 4`); with vertex weighting (byte `+1`) `+0x40` the leaf
/// positions (`leaf + 4`, three floats), `+0x34` `1 - wind weight` and `+0x38` the wind matrix
/// bytes `(wind group mod span) + starting matrix` (the wind engine's `+0x28` and `+0x2C`, as
/// bytes; a zero span faults); then `+0x18` the positions, `+0x10` the texture bytes, `+0x14`
/// `groups * texture + angle` bytes, `+0x1C` `[+0x14] + texture * 32`, `+0x24` the colour words
/// and, without manual lighting (byte `+0`), `+0x28` the normals (`leaf + 0x1C`) and `+0x30`,
/// `+0x2C` the tangents (`leaf + 0x28`) and binormals (`leaf + 0x34`).
pub fn fn_00b29380(e: &mut Engine, this: Ptr<CLeafGeometry>, lod_count: u16, leaves: u32) {
    let t = this.addr();
    if leaves == 0 || e.mem.u32(t + 0x10) == 0 || e.mem.u32(t + 4) == 0 || e.mem.u32(t + 0x14) == 0
    {
        return;
    }
    e.set(this, CLeafGeometry::m_usNumLods, lod_count);
    let count = u32::from(lod_count);
    // `new SLodGeometry[count]`: the count word and the elements, the size checked for overflow.
    let (bytes, overflow) = count.overflowing_mul(0x44);
    let size = if overflow { u32::MAX } else { bytes };
    let (total, carry) = size.overflowing_add(4);
    let total = if carry { u32::MAX } else { total };
    let block = e.call(OPERATOR_NEW, &args![total]).u32();
    let lods = if block != 0 {
        e.mem.set_u32(block, count);
        e.call(
            EH_VECTOR_CONSTRUCTOR_ITERATOR,
            &args![block + 4, 0x44u32, count, LOD_CONSTRUCTOR, LOD_DESTRUCTOR],
        );
        block + 4
    } else {
        0
    };
    e.set(this, CLeafGeometry::m_pLods, Ptr::new(lods));
    let mut lod_index = 0u16;
    while i32::from(lod_index) < i32::from(lod_count) {
        let lod = u32::from(lod_index)
            .wrapping_mul(0x44)
            .wrapping_add(e.get(this, CLeafGeometry::m_pLods).addr());
        let list = u32::from(lod_index).wrapping_mul(0x18).wrapping_add(leaves);
        let leaf_count =
            (e.mem.u32(list + 0x10).wrapping_sub(e.mem.u32(list + 0x0C)) as i32 >> 2) as u16;
        e.mem.set_u16(lod + 0x0C, leaf_count);
        let n = u32::from(e.mem.u16(lod + 0x0C));
        let array = new_array(e, n, 4);
        e.mem.set_u32(lod + 0x20, array);
        // `+0x20`: where each leaf's vertex block lies in the leaf vertex table.
        let mut i = 0i32;
        while i < i32::from(e.mem.u16(lod + 0x0C)) {
            let slot = vector_at(e, list, i as u32, 4);
            let texture = u32::from(e.mem.u8(e.mem.u32(slot) + 0x40));
            let slot = vector_at(e, list, i as u32, 4);
            let groups = u32::from(e.get(this, CLeafGeometry::m_usNumRockingGroups));
            let table = e.mem.u32(
                e.get(this, CLeafGeometry::m_pLeafVertexes)
                    .addr()
                    .wrapping_add(u32::from(lod_index) * 4),
            );
            let block = table.wrapping_add((groups << 4).wrapping_mul(texture).wrapping_mul(4));
            let angle = u32::from(e.mem.u8(e.mem.u32(slot) + 0x10));
            let value = block.wrapping_add((angle << 4).wrapping_mul(4));
            let at = e.mem.u32(lod + 0x20).wrapping_add((i as u32) * 4);
            e.mem.set_u32(at, value);
            i = i.wrapping_add(1);
        }
        if e.get(this, CLeafGeometry::m_bVertexWeighting) != 0 {
            let n = u32::from(e.mem.u16(lod + 0x0C));
            let array = new_array(e, n.wrapping_mul(3), 4);
            e.mem.set_u32(lod + 0x40, array);
            copy_leaf_floats(e, lod, list, 0x40, 0x04);
            let n = u32::from(e.mem.u16(lod + 0x0C));
            let array = new_array(e, n, 4);
            e.mem.set_u32(lod + 0x34, array);
            let mut i = 0i32;
            while i < i32::from(e.mem.u16(lod + 0x0C)) {
                let slot = vector_at(e, list, i as u32, 4);
                let weight = x87_quiet_bits(e.mem.u32(e.mem.u32(slot) + 0x44));
                let value = x87_store32(x87_sub(1.0, x87_load32(weight)));
                let at = e.mem.u32(lod + 0x34).wrapping_add((i as u32) * 4);
                e.mem.set_u32(at, value);
                i = i.wrapping_add(1);
            }
            let n = u32::from(e.mem.u16(lod + 0x0C));
            let array = e.call(OPERATOR_NEW, &args![n]).u32();
            e.mem.set_u32(lod + 0x38, array);
            let mut i = 0i32;
            while i < i32::from(e.mem.u16(lod + 0x0C)) {
                let slot = vector_at(e, list, i as u32, 4);
                let group = u32::from(e.mem.u8(e.mem.u32(slot) + 0x48));
                let wind_engine = e.mem.u32(t + 4);
                let start = e.mem.u32(wind_engine + 0x28);
                let span = e.mem.u32(wind_engine + 0x2C);
                let divisor = (span & 0xFF) as i32;
                let remainder = (group as i32) % divisor;
                let matrix = ((start & 0xFF) as u8).wrapping_add(remainder as u8);
                let at = e.mem.u32(lod + 0x38).wrapping_add(i as u32);
                e.mem.set_u8(at, matrix);
                i = i.wrapping_add(1);
            }
        }
        let n = u32::from(e.mem.u16(lod + 0x0C));
        let array = new_array(e, n.wrapping_mul(3), 4);
        e.mem.set_u32(lod + 0x18, array);
        copy_leaf_floats(e, lod, list, 0x18, 0x04);
        let n = u32::from(e.mem.u16(lod + 0x0C));
        let array = e.call(OPERATOR_NEW, &args![n]).u32();
        e.mem.set_u32(lod + 0x10, array);
        let mut i = 0u32;
        while i < u32::from(e.mem.u16(lod + 0x0C)) {
            let slot = vector_at(e, list, i, 4);
            let texture = e.mem.u8(e.mem.u32(slot) + 0x40);
            let at = e.mem.u32(lod + 0x10).wrapping_add(i);
            e.mem.set_u8(at, texture);
            i = i.wrapping_add(1);
        }
        let n = u32::from(e.mem.u16(lod + 0x0C));
        let array = e.call(OPERATOR_NEW, &args![n]).u32();
        e.mem.set_u32(lod + 0x14, array);
        let mut i = 0u32;
        while i < u32::from(e.mem.u16(lod + 0x0C)) {
            let slot = vector_at(e, list, i, 4);
            let texture = u32::from(e.mem.u8(e.mem.u32(slot) + 0x40));
            let slot = vector_at(e, list, i, 4);
            let groups = u32::from(e.get(this, CLeafGeometry::m_usNumRockingGroups));
            let angle = u32::from(e.mem.u8(e.mem.u32(slot) + 0x10));
            let value = groups.wrapping_mul(texture).wrapping_add(angle) as u8;
            let at = e.mem.u32(lod + 0x14).wrapping_add(i);
            e.mem.set_u8(at, value);
            i = i.wrapping_add(1);
        }
        let n = u32::from(e.mem.u16(lod + 0x0C));
        let array = new_array(e, n, 4);
        e.mem.set_u32(lod + 0x1C, array);
        let mut i = 0u32;
        while i < u32::from(e.mem.u16(lod + 0x0C)) {
            let slot = vector_at(e, list, i, 4);
            let texture = u32::from(e.mem.u8(e.mem.u32(slot) + 0x40));
            let value = e
                .mem
                .u32(t + 0x14)
                .wrapping_add((texture << 3).wrapping_mul(4));
            let at = e.mem.u32(lod + 0x1C).wrapping_add(i * 4);
            e.mem.set_u32(at, value);
            i = i.wrapping_add(1);
        }
        let n = u32::from(e.mem.u16(lod + 0x0C));
        let array = new_array(e, n, 4);
        e.mem.set_u32(lod + 0x24, array);
        let mut i = 0u32;
        while i < u32::from(e.mem.u16(lod + 0x0C)) {
            let slot = vector_at(e, list, i, 4);
            let color = e.mem.u32(e.mem.u32(slot) + 0x14);
            let at = e.mem.u32(lod + 0x24).wrapping_add(i * 4);
            e.mem.set_u32(at, color);
            i = i.wrapping_add(1);
        }
        if e.get(this, CLeafGeometry::m_bManualLighting) == 0 {
            let n = u32::from(e.mem.u16(lod + 0x0C));
            let array = new_array(e, n.wrapping_mul(3), 4);
            e.mem.set_u32(lod + 0x28, array);
            copy_leaf_floats(e, lod, list, 0x28, 0x1C);
        }
        if e.get(this, CLeafGeometry::m_bManualLighting) == 0 {
            let n = u32::from(e.mem.u16(lod + 0x0C));
            let array = new_array(e, n.wrapping_mul(3), 4);
            e.mem.set_u32(lod + 0x30, array);
            let n = u32::from(e.mem.u16(lod + 0x0C));
            let array = new_array(e, n.wrapping_mul(3), 4);
            e.mem.set_u32(lod + 0x2C, array);
            let mut i = 0u32;
            while i < u32::from(e.mem.u16(lod + 0x0C)) {
                let slot = vector_at(e, list, i, 4);
                let leaf = e.mem.u32(slot);
                let target = e
                    .mem
                    .u32(lod + 0x30)
                    .wrapping_add(i.wrapping_mul(3).wrapping_mul(4));
                copy_three_floats(e, target, leaf + 0x28);
                let slot = vector_at(e, list, i, 4);
                let leaf = e.mem.u32(slot);
                let target = e
                    .mem
                    .u32(lod + 0x2C)
                    .wrapping_add(i.wrapping_mul(3).wrapping_mul(4));
                copy_three_floats(e, target, leaf + 0x34);
                i = i.wrapping_add(1);
            }
        }
        lod_index = lod_index.wrapping_add(1);
    }
}

/// `new float[count]`-style allocation: `operator new(count * element)`, `0xFFFFFFFF` when the
/// multiplication overflows (the exe tests the carry of `MUL`).
fn new_array(e: &mut Engine, count: u32, element: u32) -> u32 {
    let (bytes, overflow) = count.overflowing_mul(element);
    let size = if overflow { u32::MAX } else { bytes };
    e.call(OPERATOR_NEW, &args![size]).u32()
}

/// Copies three floats from `from` to `to` through the x87 stack (`FLD`/`FSTP`, so a signalling
/// NaN is quieted).
fn copy_three_floats(e: &mut Engine, to: u32, from: u32) {
    for axis in [0u32, 4, 8] {
        copy_float(e, to + axis, from + axis);
    }
}

/// For each leaf `i` of the leaf list at `list` (the exe's range check on the list, then three
/// floats copied from `leaf + offset` to the LOD's array at `lod + array` plus 12 bytes per
/// leaf).
fn copy_leaf_floats(e: &mut Engine, lod: u32, list: u32, array: u32, offset: u32) {
    let mut i = 0i32;
    while i < i32::from(e.mem.u16(lod + 0x0C)) {
        let slot = vector_at(e, list, i as u32, 4);
        let leaf = e.mem.u32(slot);
        let target = e
            .mem
            .u32(lod + array)
            .wrapping_add(i.wrapping_mul(3).wrapping_mul(4) as u32);
        copy_three_floats(e, target, leaf + offset);
        i = i.wrapping_add(1);
    }
}

/// The entry of `00b26b50` (`cdecl`: the first guide, the hole and bottom indices, the guide
/// passed by value as 14 words and the comparison byte, 18 words in all): the guide's words are
/// laid out in an area standing for its `[EBP+0x14]` ([`with_guide_words`]).
fn entry_00b26b50(e: &mut Engine, a: &[u32]) -> Ret {
    with_guide_words(e, &a[3..17], |e, value| {
        fn_00b26b50(e, a[0], a[1] as i32, a[2] as i32, value, a[17] as u8);
    });
    Ret::default()
}

/// The entry of `00b26e80` (`cdecl`: the first guide, the hole and top indices, the guide
/// passed by value as 14 words and the comparison byte, 18 words in all): the guide's words are
/// laid out in an area standing for its `[EBP+0x14]` ([`with_guide_words`]).
fn entry_00b26e80(e: &mut Engine, a: &[u32]) -> Ret {
    with_guide_words(e, &a[3..17], |e, value| {
        fn_00b26e80(e, a[0], a[1] as i32, a[2] as i32, value, a[17] as u8);
    });
    Ret::default()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b23f10, fn_00b23f10(u32, u32, u32, u32) -> u32),
        entry!(0x00b23f60, fn_00b23f60(u32, u32, u32, u32, u32, u32)),
        entry!(0x00b24030, fn_00b24030(u32, u32, u32, u32) -> u32),
        entry!(0x00b240b0, fn_00b240b0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b24180, fn_00b24180(u32, u32, u32, u32, u32, u32)),
        entry!(0x00b24290, fn_00b24290(u32, u32, u32, u32, u32, u32)),
        entry!(0x00b243a0, fn_00b243a0(u32, u32, u32, u32, u32, u32)),
        entry!(0x00b244a0, fn_00b244a0(u32, u32, u32, u32, u32, u32)),
        entry!(0x00b24580, fn_00b24580(u32, u32, u32, u32, u32, u32)),
        entry!(0x00b24690, fn_00b24690(u32, u32, u32, u32) -> u32),
        entry!(0x00b24710, fn_00b24710(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b247b0, fn_00b247b0(u32, u32, u32, u32) -> u32),
        entry!(0x00b24830, fn_00b24830(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b248c0, fn_00b248c0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b24960, fn_00b24960(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(
            0x00b249d0,
            fn_00b249d0(Ptr<SFrondGuide>, Ptr<SFrondGuide>) -> Ptr<SFrondGuide>
        ),
        entry!(
            0x00b24a80,
            fn_00b24a80(Ptr<SFrondTexture>, Ptr<SFrondTexture>) -> Ptr<SFrondTexture>
        ),
        entry!(
            0x00b24ae0,
            fn_00b24ae0(Ptr<StlVector>, Ptr<StlVector>) -> Ptr<StlVector>
        ),
        entry!(
            0x00b24f40,
            fn_00b24f40(Ptr<StlVector>, Ptr<StlVector>) -> Ptr<StlVector>
        ),
        entry!(0x00b253f0, fn_00b253f0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b25490, fn_00b25490(Ptr<StlVector>)),
        entry!(
            0x00b25580,
            fn_00b25580(Ptr<StlVector>, Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00b25680, fn_00b25680(Ptr, u32, u32, u8) -> Ptr),
        entry!(0x00b25a90, fn_00b25a90(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b25ba0, fn_00b25ba0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b25ca0, fn_00b25ca0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b25e20, fn_00b25e20(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b25fa0, fn_00b25fa0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b260c0, fn_00b260c0(u32, u32, u32) -> u32),
        entry!(0x00b26160, fn_00b26160(u32, u32, u32, u8)),
        entry!(0x00b26260, fn_00b26260(u32, u32, u8, u32, u32)),
        entry!(0x00b262d0, fn_00b262d0(u32, u32, u32, u32)),
        entry!(0x00b26450, fn_00b26450(u32, u32, u32, u32) -> u32),
        entry!(0x00b264a0, fn_00b264a0(u32, u32, u32, u32) -> u32),
        entry!(0x00b264f0, fn_00b264f0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b26570, fn_00b26570(Ptr<StlVector>, Ptr<StlVector>)),
        entry!(0x00b267a0, fn_00b267a0(Ptr<StlVector>, Ptr<StlVector>)),
        entry!(0x00b269d0, fn_00b269d0(u32, u32, u32, u32)),
        entry!(0x00b26ab0, fn_00b26ab0(Ptr<SFrondGuide>, Ptr<SFrondGuide>)),
        (0x00b26b50, entry_00b26b50 as AbiFn),
        entry!(0x00b26ca0, fn_00b26ca0(u32, u32, u8)),
        entry!(0x00b26ce0, fn_00b26ce0(u32, u32, u32) -> u32),
        entry!(0x00b26db0, fn_00b26db0(u32, u32, u32, u32, u32, u32) -> u32),
        (0x00b26e80, entry_00b26e80 as AbiFn),
        entry!(0x00b26f90, fn_00b26f90(u32, u32, u8, u32)),
        entry!(0x00b27060, fn_00b27060(Ptr) -> Ptr),
        entry!(0x00b27080, fn_00b27080(Ptr<CVec3>)),
        entry!(0x00b273d0, fn_00b273d0(Ptr<CRotTransform>, f32)),
        entry!(
            0x00b274a0,
            fn_00b274a0(Ptr<SLodGeometry>) -> Ptr<SLodGeometry>
        ),
        entry!(0x00b274d0, fn_00b274d0(Ptr<SLodGeometry>)),
        entry!(
            0x00b27660,
            fn_00b27660(Ptr<CLeafGeometry>, Ptr) -> Ptr<CLeafGeometry>
        ),
        entry!(0x00b276f0, fn_00b276f0(Ptr<CLeafGeometry>)),
        entry!(0x00b277b0, fn_00b277b0(Ptr, u32) -> Ptr),
        entry!(0x00b27820, fn_00b27820(Ptr<CLeafGeometry>)),
        entry!(0x00b27870, fn_00b27870(Ptr<CLeafGeometry>, Ptr) -> u32),
        entry!(0x00b27910, fn_00b27910(Ptr<CLeafGeometry>, Ptr)),
        entry!(0x00b28280, fn_00b28280(Ptr<CLeafGeometry>, Ptr)),
        entry!(0x00b28440, fn_00b28440(Ptr<CVec3>, f32) -> Ptr<CVec3>),
        entry!(0x00b28480, fn_00b28480(Ptr<CLeafGeometry>, u16, u32, Ptr)),
        entry!(0x00b28720, fn_00b28720(Ptr<CLeafGeometry>, u32, Ptr)),
        entry!(
            0x00b288f0,
            fn_00b288f0(Ptr<CLeafGeometry>, Ptr, u16, u32, u32, f32)
        ),
        entry!(
            0x00b28e40,
            fn_00b28e40(Ptr<CLeafGeometry>, Ptr, u16, u32, u32, u32)
        ),
        entry!(0x00b28ed0, fn_00b28ed0(Ptr<CLeafGeometry>)),
        entry!(0x00b29190, fn_00b29190(Ptr<CLeafGeometry>, u16)),
        entry!(0x00b29380, fn_00b29380(Ptr<CLeafGeometry>, u16, u32)),
        entry!(0x00b29fd0, fn_00b29fd0(Ptr) -> Ptr),
        entry!(0x00b2a180, fn_00b2a180(Ptr<CVec3>, Ptr<CVec3>) -> f32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::speedtree::branch::INVALID_PARAMETER;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    /// The proxy word the test vectors carry.
    const PROXY: u32 = 0x0090_0000;

    /// Callees outside this file; every test runs them as do-nothing doubles unless it
    /// supplies its own. The guide copy constructor (`00b1ff50`) is the real one, which needs
    /// the proxy allocation and `_Buy` doubles below.
    const EXTERNAL: &[u32] = &[
        GUIDE_VECTOR_COPY_CONSTRUCT,
        TEXTURE_COPY_CONSTRUCT,
        FLOAT_VECTOR_COPY_CONSTRUCT,
        GUIDE_ASSIGN_RANGE,
        STRING_ASSIGN,
        VERTEX_VECTOR_BUY,
        VERTEX_VECTOR_TIDY,
        PROXY_RELEASE,
        ITERATORS_DIFFER,
        GUIDE_VECTOR_ERASE_ALL,
        GUIDE_DESTROY_RANGE,
        GUIDE_VECTOR_TIDY,
        GUIDE_VECTOR_DEFAULT_CONSTRUCT,
        FLOAT_VECTOR_CONSTRUCT,
        FLOAT_VECTOR_TIDY,
        FLOAT_VECTOR_ASSIGN,
        FLOAT_VECTOR_PUSH,
        TRIVIAL_DESTRUCTOR,
        CROT_TRANSFORM_CONSTRUCT,
        CROT_TRANSFORM_MULTIPLY,
        VEC3_TIMES_ROTATION,
        VEC3_TIMES_MATRIX,
        VEC3_DISTANCE,
        FLIP_TEXTURE_V,
        LEAF_BASE_CONSTRUCT,
        LEAF_BASE_DESTRUCT,
        VECTOR_DESTRUCTOR_ITERATOR,
        EXTENTS_CONSTRUCT,
        EXTENTS_EXPAND,
        EXTENTS_MERGE,
        INVALID_PARAMETER,
        0x0044_d670,
    ];

    /// An engine with the exe's data range mapped as zeroes and every outside callee of the file
    /// silenced; the proxy allocation hands out blocks.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        for addr in EXTERNAL {
            e.register(*addr, |_, _| Ret::default());
        }
        e.register(0x0044_d670, |e, _| Ret {
            eax: e.mem.alloc(8),
            ..Ret::default()
        });
        e
    }

    /// The calls of `addr` in the log, argument words by call.
    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// Makes the `std::vector` header at `at` hold `count` zeroed elements of `size` bytes with
    /// room for `capacity`, and returns the address of the elements.
    fn vector(e: &mut Engine, at: u32, size: u32, count: u32, capacity: u32) -> u32 {
        let data = e.mem.alloc((size * capacity).max(1));
        e.mem.set_u32(at, PROXY);
        e.mem.set_u32(at + 0x0C, data);
        e.mem.set_u32(at + 0x10, data + size * count);
        e.mem.set_u32(at + 0x14, data + size * capacity);
        data
    }

    /// A double that answers `result` in `EAX`.
    fn reply(e: &mut Engine, addr: u32, result: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax: result,
            ..Ret::default()
        });
    }

    /// A double that records its argument words (the log is returned) and answers zero.
    fn spy(e: &mut Engine, addr: u32) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        log
    }

    /// Writes `words` from `at`.
    fn put_words(e: &mut Engine, at: u32, words: &[u32]) {
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, *word);
        }
    }

    /// `count` words from `at`.
    fn words(e: &Engine, at: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    /// `count` guides in one block; guide `i` has the key `keys[i]` (the fuzzy surface area at
    /// `+0x2C`) and the tag `i + 1` as its length (`+0x18`). Returns the first guide.
    fn guides(e: &mut Engine, keys: &[f32]) -> u32 {
        let base = e.mem.alloc(0x38 * keys.len() as u32);
        for (i, key) in keys.iter().enumerate() {
            let guide = base + 0x38 * i as u32;
            e.mem.set_f32(guide + 0x2C, *key);
            e.mem.set_f32(guide + 0x18, (i + 1) as f32);
        }
        base
    }

    /// The keys of `count` guides.
    fn keys(e: &Engine, base: u32, count: u32) -> Vec<f32> {
        (0..count)
            .map(|i| e.mem.f32(base + 0x38 * i + 0x2C))
            .collect()
    }

    /// The tags of `count` guides.
    fn tags(e: &Engine, base: u32, count: u32) -> Vec<f32> {
        (0..count)
            .map(|i| e.mem.f32(base + 0x38 * i + 0x18))
            .collect()
    }

    /// A fill with copies of one element given by words: the destination is filled in order and
    /// a null destination does nothing for its own slot.
    fn check_word_fill(address: u32, size: u32) {
        let mut e = engine();
        let value = e.mem.alloc(size);
        for w in 0..size / 4 {
            e.mem.set_u32(value + 4 * w, 0x300 + w);
        }
        let target = e.mem.alloc(3 * size);
        e.call(address, &args![target, 3u32, value, 0u32, 0u32, 0u32]);
        for i in 0..3 {
            for w in 0..size / 4 {
                assert_eq!(e.mem.u32(target + i * size + 4 * w), 0x300 + w);
            }
        }
        // A null destination skips its slot (and a count of zero does nothing).
        e.call(address, &args![0u32, 1u32, value, 0u32, 0u32, 0u32]);
        e.call(address, &args![target, 0u32, value, 0u32, 0u32, 0u32]);
    }

    /// A fill with copy constructions: `constructor` is called with each slot and the value; a
    /// null slot is skipped.
    fn check_constructing_fill(address: u32, size: u32, constructor: u32) {
        let mut e = engine();
        let log = spy(&mut e, constructor);
        e.call(
            address,
            &args![0x1000u32, 3u32, 0x2000u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(
            *log.borrow(),
            vec![
                vec![0x1000, 0x2000],
                vec![0x1000 + size, 0x2000],
                vec![0x1000 + 2 * size, 0x2000]
            ]
        );
        log.borrow_mut().clear();
        e.call(address, &args![0u32, 1u32, 0x2000u32, 0u32, 0u32, 0u32]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn vertices_are_filled_with_copies() {
        check_word_fill(0x00b2_3f60, 0x38);
    }

    #[test]
    fn cvecs_are_filled_with_copies() {
        check_word_fill(0x00b2_44a0, 0xC);
    }

    #[test]
    fn guides_are_filled_with_copy_constructions() {
        check_constructing_fill(0x00b2_4180, 0x38, GUIDE_COPY_CONSTRUCT);
    }

    #[test]
    fn guide_vectors_are_filled_with_copy_constructions() {
        check_constructing_fill(0x00b2_4290, 0x18, GUIDE_VECTOR_COPY_CONSTRUCT);
    }

    #[test]
    fn textures_are_filled_with_copy_constructions() {
        check_constructing_fill(0x00b2_43a0, 0x2C, TEXTURE_COPY_CONSTRUCT);
    }

    #[test]
    fn float_vectors_are_filled_with_copy_constructions() {
        check_constructing_fill(0x00b2_4580, 0x18, FLOAT_VECTOR_COPY_CONSTRUCT);
    }

    /// A range copy that constructs: the copy of each source element goes to the destination
    /// slot (words or `constructor` calls), a null slot is skipped and the end is returned.
    fn check_constructing_copy(address: u32, size: u32, constructor: u32) {
        let mut e = engine();
        let log = spy(&mut e, constructor);
        let end = e
            .call(
                address,
                &args![0x1000u32, 0x1000 + 2 * size, 0x2000u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(end, 0x2000 + 2 * size);
        assert_eq!(
            *log.borrow(),
            vec![vec![0x2000, 0x1000], vec![0x2000 + size, 0x1000 + size]]
        );
        log.borrow_mut().clear();
        e.call(
            address,
            &args![0x1000u32, 0x1000 + size, 0u32, 0u32, 0u32, 0u32],
        );
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn textures_are_copy_constructed_into_storage() {
        check_constructing_copy(0x00b2_5ba0, 0x2C, TEXTURE_COPY_CONSTRUCT);
    }

    #[test]
    fn guides_are_copy_constructed_into_storage() {
        check_constructing_copy(0x00b2_5fa0, 0x38, GUIDE_COPY_CONSTRUCT);
    }

    /// A range copy of words into storage: the words are copied, the end returned and a null
    /// destination slot skipped.
    fn check_word_copy(address: u32, size: u32) {
        let mut e = engine();
        let source = e.mem.alloc(2 * size);
        for w in 0..2 * size / 4 {
            e.mem.set_u32(source + 4 * w, 0x700 + w);
        }
        let target = e.mem.alloc(2 * size);
        let end = e
            .call(
                address,
                &args![source, source + 2 * size, target, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(end, target + 2 * size);
        assert_eq!(
            words(&e, target, 2 * size / 4),
            words(&e, source, 2 * size / 4)
        );
        // A null destination skips the slot but still advances.
        assert_eq!(
            e.call(
                address,
                &args![source, source + size, 0u32, 0u32, 0u32, 0u32]
            )
            .u32(),
            size
        );
    }

    #[test]
    fn six_word_elements_are_copied_into_storage() {
        check_word_copy(0x00b2_5a90, 0x18);
    }

    #[test]
    fn vertices_are_copied_into_storage() {
        check_word_copy(0x00b2_6db0, 0x38);
    }

    /// A wrapper of a range copy: forwards its four words and the zero flag words and returns
    /// the end of the destination.
    fn check_range_wrapper(address: u32, size: u32, constructor: Option<u32>) {
        let mut e = engine();
        let source = e.mem.alloc(2 * size);
        let target = e.mem.alloc(2 * size);
        for w in 0..2 * size / 4 {
            e.mem.set_u32(source + 4 * w, 0x900 + w);
        }
        let log = constructor.map(|c| spy(&mut e, c));
        let end = e
            .call(address, &args![source, source + 2 * size, target, 0x55u32])
            .u32();
        assert_eq!(end, target + 2 * size);
        match log {
            Some(log) => assert_eq!(log.borrow().len(), 2),
            None => assert_eq!(
                words(&e, target, 2 * size / 4),
                words(&e, source, 2 * size / 4)
            ),
        }
    }

    #[test]
    fn the_texture_copy_wrappers_forward_to_the_constructing_copy() {
        check_range_wrapper(0x00b2_3f10, 0x2C, Some(TEXTURE_COPY_CONSTRUCT));
        check_range_wrapper(0x00b2_47b0, 0x2C, Some(TEXTURE_COPY_CONSTRUCT));
    }

    #[test]
    fn the_guide_copy_wrappers_forward_to_the_constructing_copy() {
        check_range_wrapper(0x00b2_4030, 0x38, Some(GUIDE_COPY_CONSTRUCT));
        check_range_wrapper(0x00b2_6450, 0x38, Some(GUIDE_COPY_CONSTRUCT));
    }

    #[test]
    fn the_vertex_copy_wrappers_forward_to_the_word_copy() {
        check_range_wrapper(0x00b2_4690, 0x38, None);
        check_range_wrapper(0x00b2_64a0, 0x38, None);
    }

    /// An overwriting copy of 14-word vertices in order.
    fn check_overwrite(address: u32, extra: usize) {
        let mut e = engine();
        let source = e.mem.alloc(0x70);
        let target = e.mem.alloc(0x70);
        for w in 0..28 {
            e.mem.set_u32(source + 4 * w, 0x400 + w);
        }
        let mut a = vec![source, source + 0x70, target];
        a.resize(3 + extra, 0);
        let end = e.call(address, &a).u32();
        assert_eq!(end, target + 0x70);
        assert_eq!(words(&e, target, 28), words(&e, source, 28));
    }

    #[test]
    fn vertices_are_copied_over_vertices() {
        check_overwrite(0x00b2_60c0, 0);
        check_overwrite(0x00b2_64f0, 3);
    }

    #[test]
    fn vertices_are_copied_backwards_to_end_at_the_destination_end() {
        let mut e = engine();
        let source = e.mem.alloc(0x70);
        let target = e.mem.alloc(0x70);
        for w in 0..28 {
            e.mem.set_u32(source + 4 * w, 0x400 + w);
        }
        let start = e
            .call(
                0x00b2_4710,
                &args![source, source + 0x70, target + 0x70, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(start, target);
        assert_eq!(words(&e, target, 28), words(&e, source, 28));
    }

    #[test]
    fn cvecs_are_copied_backwards_to_end_at_the_destination_end() {
        let mut e = engine();
        let source = e.mem.alloc(0x18);
        let target = e.mem.alloc(0x18);
        put_words(&mut e, source, &[1, 2, 3, 4, 5, 6]);
        let start = e
            .call(
                0x00b2_48c0,
                &args![source, source + 0x18, target + 0x18, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(start, target);
        assert_eq!(words(&e, target, 6), vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn words_are_moved_with_memmove_s_and_the_end_returned() {
        let mut e = engine();
        // `memmove_s` (`00ec7ce3`) is an unplaced unit of `engine-fallout`, which this
        // crate's tests do not link since the engine split: a double with its body for
        // the moves made here (a size of 0 succeeds; valid pointers with room move
        // through `memmove`, `00ec7230`).
        e.register(0x00ec_7ce3, |e, a| {
            assert!(a[3] == 0 || (a[0] != 0 && a[2] != 0 && a[1] >= a[3]));
            if a[3] != 0 {
                e.call(0x00ec_7230, &[a[0], a[2], a[3]]);
            }
            0u32.into_ret()
        });
        let source = e.mem.alloc(16);
        let target = e.mem.alloc(16);
        put_words(&mut e, source, &[10, 20, 30, 40]);
        let end = e
            .call(
                0x00b2_4960,
                &args![PROXY, source, PROXY, source + 16, target, 0u32],
            )
            .u32();
        assert_eq!(end, target + 16);
        assert_eq!(words(&e, target, 4), vec![10, 20, 30, 40]);
        // An empty range moves nothing.
        assert_eq!(
            e.call(
                0x00b2_4960,
                &args![PROXY, source, PROXY, source, target, 0u32]
            )
            .u32(),
            target
        );
    }

    #[test]
    fn a_guide_assignment_copies_the_fields_and_the_vertex_vector() {
        let mut e = engine();
        let source = e.new_object::<SFrondGuide>();
        let target = e.new_object::<SFrondGuide>();
        let s = source.addr();
        e.mem.set_u32(s + 0x18, 0x7f80_0001);
        e.mem.set_f32(s + 0x1C, 2.5);
        e.mem.set_u8(s + 0x20, 7);
        e.mem.set_f32(s + 0x24, 0.25);
        e.mem.set_f32(s + 0x28, 0.5);
        e.mem.set_u32(s + 0x2C, 0x7f80_0002);
        e.mem.set_u32(s + 0x30, 0x1234);
        e.mem.set_u32(s + 0x34, 0x5678);
        let result = e
            .call(0x00b2_49d0, &args![target, source])
            .ptr::<SFrondGuide>();
        assert_eq!(result, target);
        let t = target.addr();
        assert_eq!(e.mem.u32(t + 0x18), 0x7fc0_0001);
        assert_eq!(e.mem.f32(t + 0x1C), 2.5);
        assert_eq!(e.mem.u8(t + 0x20), 7);
        assert_eq!(e.mem.f32(t + 0x24), 0.25);
        assert_eq!(e.mem.f32(t + 0x28), 0.5);
        assert_eq!(e.mem.u32(t + 0x2C), 0x7fc0_0002);
        assert_eq!(e.mem.u32(t + 0x30), 0x1234);
        assert_eq!(e.mem.u32(t + 0x34), 0x5678);
    }

    #[test]
    fn a_texture_assignment_assigns_the_string_and_copies_the_floats() {
        let mut e = engine();
        e.mem.set_u32(STRING_NPOS, 0xffff_ffff);
        let log = spy(&mut e, STRING_ASSIGN);
        let source = e.new_object::<SFrondTexture>();
        let target = e.new_object::<SFrondTexture>();
        let s = source.addr();
        e.mem.set_f32(s + 0x1C, 1.5);
        e.mem.set_f32(s + 0x20, 2.5);
        e.mem.set_u32(s + 0x24, 0x7f80_0003);
        e.mem.set_f32(s + 0x28, 4.5);
        let result = e
            .call(0x00b2_4a80, &args![target, source])
            .ptr::<SFrondTexture>();
        assert_eq!(result, target);
        assert_eq!(*log.borrow(), vec![vec![target.addr(), s, 0, 0xffff_ffff]]);
        let t = target.addr();
        assert_eq!(e.mem.f32(t + 0x1C), 1.5);
        assert_eq!(e.mem.f32(t + 0x20), 2.5);
        assert_eq!(e.mem.u32(t + 0x24), 0x7fc0_0003);
        assert_eq!(e.mem.f32(t + 0x28), 4.5);
    }

    /// Guides assigned over one another by a range function: `address` takes `(first, last,
    /// destination)` (plus `extra` flag words) and returns the destination's end or start.
    #[test]
    fn guides_are_assigned_forwards_and_backwards() {
        let mut e = engine();
        let source = guides(&mut e, &[1.0, 2.0]);
        let target = guides(&mut e, &[0.0, 0.0]);
        let end = e
            .call(
                0x00b2_53f0,
                &args![source, source + 0x70, target, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(end, target + 0x70);
        assert_eq!(keys(&e, target, 2), vec![1.0, 2.0]);
        assert_eq!(tags(&e, target, 2), vec![1.0, 2.0]);
        // Backwards: the guides end at the destination's end.
        let back = guides(&mut e, &[0.0, 0.0]);
        let start = e
            .call(
                0x00b2_40b0,
                &args![source, source + 0x70, back + 0x70, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(start, back);
        assert_eq!(keys(&e, back, 2), vec![1.0, 2.0]);
        let again = guides(&mut e, &[0.0, 0.0]);
        let start = e
            .call(0x00b2_6ce0, &args![source, source + 0x70, again + 0x70])
            .u32();
        assert_eq!(start, again);
        assert_eq!(tags(&e, again, 2), vec![1.0, 2.0]);
    }

    #[test]
    fn textures_are_assigned_backwards() {
        let mut e = engine();
        let log = spy(&mut e, STRING_ASSIGN);
        let source = e.mem.alloc(0x58);
        let target = e.mem.alloc(0x58);
        let start = e
            .call(
                0x00b2_4830,
                &args![source, source + 0x58, target + 0x58, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(start, target);
        let assigned = log.borrow();
        assert_eq!(assigned.len(), 2);
        assert_eq!(assigned[0][..2], [target + 0x2C, source + 0x2C]);
        assert_eq!(assigned[1][..2], [target, source]);
    }

    /// A vertex vector (0x38-byte elements) of `count` elements with room for `capacity`, each
    /// element's first word `1000 * (i + 1) + base`; returns the vector and its elements.
    fn vertex_vector(e: &mut Engine, count: u32, capacity: u32, base: u32) -> (u32, u32) {
        let this = e.mem.alloc(0x18);
        let data = vector(e, this, 0x38, count, capacity);
        for i in 0..count {
            e.mem.set_u32(data + 0x38 * i, 1000 * (i + 1) + base);
        }
        (this, data)
    }

    #[test]
    fn clearing_the_vertex_vector_erases_everything() {
        let mut e = engine();
        let (this, data) = vertex_vector(&mut e, 2, 4, 0);
        reply(&mut e, ITERATORS_DIFFER, 1);
        e.call(0x00b2_5490, &args![this]);
        assert_eq!(e.mem.u32(this + 0x10), data);
    }

    #[test]
    fn erasing_a_vertex_range_copies_the_rest_down() {
        let mut e = engine();
        let (this, data) = vertex_vector(&mut e, 3, 4, 0);
        let out = e.mem.alloc(8);
        reply(&mut e, ITERATORS_DIFFER, 1);
        let result = e
            .call(
                0x00b2_5580,
                &args![this, out, 0u32, data + 0x38, 0u32, data + 0x70],
            )
            .u32();
        assert_eq!(result, out);
        assert_eq!(e.mem.u32(data + 0x38), 3000);
        assert_eq!(e.mem.u32(this + 0x10), data + 0x70);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (PROXY, data + 0x38));
        // Equal iterators leave the vector alone.
        reply(&mut e, ITERATORS_DIFFER, 0);
        e.call(0x00b2_5580, &args![this, out, 0u32, data, 0u32, data]);
        assert_eq!(e.mem.u32(this + 0x10), data + 0x70);
    }

    #[test]
    fn assigning_a_vertex_vector_covers_the_four_cases() {
        let mut e = engine();
        // A self-assignment does nothing.
        let (this, data) = vertex_vector(&mut e, 2, 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b2_4ae0, &args![this, this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // An empty source clears.
        let (empty, _) = vertex_vector(&mut e, 0, 0, 0);
        reply(&mut e, ITERATORS_DIFFER, 1);
        let result = e.call(0x00b2_4ae0, &args![this, empty]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this + 0x10), data);
        // A source no longer than the vector overwrites the first elements.
        let (this, data) = vertex_vector(&mut e, 3, 4, 0);
        let (source, _) = vertex_vector(&mut e, 2, 2, 50);
        e.call(0x00b2_4ae0, &args![this, source]);
        assert_eq!(e.mem.u32(data), 1050);
        assert_eq!(e.mem.u32(data + 0x38), 2050);
        assert_eq!(e.mem.u32(this + 0x10), data + 0x70);
        // A longer source that fits the capacity: the first part overwrites, the rest is built.
        let (this, data) = vertex_vector(&mut e, 1, 4, 0);
        let (source, _) = vertex_vector(&mut e, 3, 3, 60);
        e.call(0x00b2_4ae0, &args![this, source]);
        assert_eq!(
            [
                e.mem.u32(data),
                e.mem.u32(data + 0x38),
                e.mem.u32(data + 0x70)
            ],
            [1060, 2060, 3060]
        );
        assert_eq!(e.mem.u32(this + 0x10), data + 0xA8);
        // A longer source than the capacity: the block is freed and `_Buy` provides a new one.
        let (this, data) = vertex_vector(&mut e, 1, 1, 0);
        let (source, _) = vertex_vector(&mut e, 2, 2, 70);
        let storage = e.mem.alloc(0x70);
        e.register_double(VERTEX_VECTOR_BUY, move |e, a| {
            e.mem.set_u32(a[0] + 0x0C, storage);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.call(0x00b2_4ae0, &args![this, source]);
        assert_eq!(e.mem.block_size(data), None);
        assert_eq!(e.mem.u32(storage), 1070);
        assert_eq!(e.mem.u32(storage + 0x38), 2070);
        assert_eq!(e.mem.u32(this + 0x10), storage + 0x70);
    }

    #[test]
    fn assigning_a_guide_vector_covers_the_cases() {
        let mut e = engine();
        // A self-assignment does nothing.
        let (this, _) = vertex_vector(&mut e, 2, 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b2_4f40, &args![this, this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // An empty source erases through `00b1de20`.
        let (empty, _) = vertex_vector(&mut e, 0, 0, 0);
        e.call_log = Some(vec![]);
        e.call(0x00b2_4f40, &args![this, empty]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, GUIDE_VECTOR_ERASE_ALL), vec![vec![this]]);
        // A longer source than the capacity: the old guides are tidied and freed, then the
        // copies are made into the new block.
        let (this, data) = vertex_vector(&mut e, 1, 1, 0);
        let (source, first) = vertex_vector(&mut e, 2, 2, 5);
        let storage = e.mem.alloc(0x70);
        e.register_double(VERTEX_VECTOR_BUY, move |e, a| {
            e.mem.set_u32(a[0] + 0x0C, storage);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let copy = spy(&mut e, GUIDE_COPY_CONSTRUCT);
        e.call_log = Some(vec![]);
        e.call(0x00b2_4f40, &args![this, source]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, VERTEX_VECTOR_TIDY), vec![vec![data]]);
        assert_eq!(calls(&log, PROXY_RELEASE), vec![vec![data]]);
        assert_eq!(e.mem.block_size(data), None);
        assert_eq!(
            *copy.borrow(),
            vec![vec![storage, first], vec![storage + 0x38, first + 0x38]]
        );
        assert_eq!(e.mem.u32(this + 0x10), storage + 0x70);
        // A longer source within the capacity: the first guides are assigned (`00b23760`), the
        // rest constructed.
        let (this, data) = vertex_vector(&mut e, 1, 4, 0);
        let (source, first) = vertex_vector(&mut e, 3, 3, 5);
        let assign = spy(&mut e, GUIDE_ASSIGN_RANGE);
        copy.borrow_mut().clear();
        e.call(0x00b2_4f40, &args![this, source]);
        assert_eq!(*assign.borrow(), vec![vec![first, first + 0x38, data]]);
        assert_eq!(
            *copy.borrow(),
            vec![
                vec![data + 0x38, first + 0x38],
                vec![data + 0x70, first + 0x70]
            ]
        );
        assert_eq!(e.mem.u32(this + 0x10), data + 0xA8);
        // A source no longer than the vector: assigned over, the surplus destroyed.
        let (this, data) = vertex_vector(&mut e, 3, 4, 0);
        let (source, _) = guide_vector_of(&mut e, &[1.0, 2.0]);
        e.call_log = Some(vec![]);
        e.call(0x00b2_4f40, &args![this, source]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, GUIDE_DESTROY_RANGE),
            vec![vec![this, data + 0x70, data + 0xA8]]
        );
        assert_eq!(e.mem.u32(this + 0x10), data + 0x70);
    }

    /// A guide vector holding guides with the given keys: returns the vector and its elements.
    fn guide_vector_of(e: &mut Engine, keys: &[f32]) -> (u32, u32) {
        let this = e.mem.alloc(0x18);
        let data = vector(e, this, 0x38, keys.len() as u32, keys.len() as u32);
        for (i, key) in keys.iter().enumerate() {
            e.mem.set_f32(data + 0x38 * i as u32 + 0x2C, *key);
            e.mem.set_f32(data + 0x38 * i as u32 + 0x18, (i + 1) as f32);
        }
        (this, data)
    }

    /// A vector header at `at` with its own proxy block (whose first word points back to it) and
    /// the given first, last and end pointers.
    fn owned_vector(e: &mut Engine, at: u32, first: u32, last: u32, end: u32) -> u32 {
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(proxy, at);
        e.mem.set_u32(at, proxy);
        e.mem.set_u32(at + 0x0C, first);
        e.mem.set_u32(at + 0x10, last);
        e.mem.set_u32(at + 0x14, end);
        proxy
    }

    /// A double for a vector constructor of the exe: gives the vector at `a[0]` a proxy.
    fn constructor_double(e: &mut Engine, addr: u32) -> Rc<RefCell<Vec<Vec<u32>>>> {
        let log: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            let proxy = e.mem.alloc(8);
            e.mem.set_u32(proxy, a[0]);
            e.mem.set_u32(a[0], proxy);
            Ret::default()
        });
        log
    }

    #[test]
    fn vector_ranges_move_into_storage_by_swapping_with_a_fresh_vector() {
        // The guide-vector and float-vector range moves: a copy of an empty vector is made
        // for each destination slot, then the slot swaps with the source.
        for (address, empty, copy) in [
            (
                0x00b2_5ca0u32,
                GUIDE_VECTOR_DEFAULT_CONSTRUCT,
                GUIDE_VECTOR_COPY_CONSTRUCT,
            ),
            (
                0x00b2_5e20,
                FLOAT_VECTOR_CONSTRUCT,
                FLOAT_VECTOR_COPY_CONSTRUCT,
            ),
        ] {
            let mut e = engine();
            let make = constructor_double(&mut e, empty);
            let construct = constructor_double(&mut e, copy);
            let source = e.mem.alloc(0x30);
            let target = e.mem.alloc(0x30);
            owned_vector(&mut e, source, 0x21, 0x22, 0x23);
            owned_vector(&mut e, source + 0x18, 0x31, 0x32, 0x33);
            let end = e
                .call(
                    address,
                    &args![source, source + 0x30, target, 0u32, 0u32, 0u32],
                )
                .u32();
            assert_eq!(end, target + 0x30);
            // The temporary was made once and each slot copy constructed from it.
            assert_eq!(make.borrow().len(), 1);
            let temp = make.borrow()[0][0];
            assert_eq!(
                *construct.borrow(),
                vec![vec![target, temp], vec![target + 0x18, temp]]
            );
            // The slots took the sources' pointers; their proxies point back to them.
            assert_eq!(e.mem.u32(target + 0x0C), 0x21);
            assert_eq!(e.mem.u32(target + 0x18 + 0x0C), 0x31);
            assert_eq!(e.mem.u32(e.mem.u32(target)), target);
            assert_eq!(e.mem.u32(e.mem.u32(target + 0x18)), target + 0x18);
            assert_eq!(e.mem.u32(source + 0x0C), 0);
        }
    }

    #[test]
    fn swapping_vectors_exchanges_pointers_and_proxy_owners() {
        for address in [0x00b2_6570u32, 0x00b2_67a0] {
            let mut e = engine();
            let a = e.mem.alloc(0x18);
            let b = e.mem.alloc(0x18);
            let proxy_a = owned_vector(&mut e, a, 0x11, 0x12, 0x13);
            let proxy_b = owned_vector(&mut e, b, 0x21, 0x22, 0x23);
            e.call(address, &args![a, b]);
            assert_eq!(words(&e, a + 0x0C, 3), vec![0x21, 0x22, 0x23]);
            assert_eq!(words(&e, b + 0x0C, 3), vec![0x11, 0x12, 0x13]);
            assert_eq!((e.mem.u32(a), e.mem.u32(b)), (proxy_b, proxy_a));
            // Each proxy points to its new owner.
            assert_eq!((e.mem.u32(proxy_b), e.mem.u32(proxy_a)), (a, b));
            // A swap with itself does nothing.
            e.call(address, &args![a, a]);
            assert_eq!(words(&e, a + 0x0C, 3), vec![0x21, 0x22, 0x23]);
        }
    }

    #[test]
    fn the_unreachable_copying_swap_exchanges_through_a_copy() {
        let mut e = engine();
        let copy = constructor_double(&mut e, GUIDE_VECTOR_COPY_CONSTRUCT);
        let assign = spy(&mut e, GUIDE_VECTOR_ASSIGN);
        let tidy = spy(&mut e, GUIDE_VECTOR_TIDY);
        let a = e.mem.alloc(0x18);
        let b = e.mem.alloc(0x18);
        let proxy_a = owned_vector(&mut e, a, 0x11, 0x12, 0x13);
        let proxy_b = owned_vector(&mut e, b, 0x21, 0x22, 0x23);
        vector_swap(
            &mut e,
            Ptr::new(a),
            Ptr::new(b),
            false,
            GUIDE_VECTOR_COPY_CONSTRUCT,
            GUIDE_VECTOR_ASSIGN,
            GUIDE_VECTOR_TIDY,
        );
        // The proxies were exchanged first, then a copy of `this` made, both vectors assigned.
        assert_eq!((e.mem.u32(a), e.mem.u32(b)), (proxy_b, proxy_a));
        let temp = copy.borrow()[0][0];
        assert_eq!(copy.borrow()[0], vec![temp, a]);
        assert_eq!(*assign.borrow(), vec![vec![a, b], vec![b, temp]]);
        assert_eq!(*tidy.borrow(), vec![vec![temp]]);
    }

    /// The median of three guides: the keys end up in descending order.
    #[test]
    fn the_median_of_three_orders_the_guides_descending() {
        let permutations = [
            [1.0f32, 2.0, 3.0],
            [1.0, 3.0, 2.0],
            [2.0, 1.0, 3.0],
            [2.0, 3.0, 1.0],
            [3.0, 1.0, 2.0],
            [3.0, 2.0, 1.0],
        ];
        for keys_in in permutations {
            let mut e = engine();
            let base = guides(&mut e, &keys_in);
            e.call(0x00b2_69d0, &args![base, base + 0x38, base + 0x70, 0u32]);
            assert_eq!(keys(&e, base, 3), vec![3.0, 2.0, 1.0], "{keys_in:?}");
            // The tags travel with the keys.
            let t = tags(&e, base, 3);
            for (i, key) in keys(&e, base, 3).iter().enumerate() {
                let origin = keys_in.iter().position(|k| k == key).unwrap();
                assert_eq!(t[i], (origin + 1) as f32);
            }
        }
    }

    #[test]
    fn the_pivot_choice_uses_nine_samples_for_long_ranges() {
        // Up to 40 elements: one median of three at the first, middle and last.
        let mut e = engine();
        let base = guides(&mut e, &[1.0, 5.0, 3.0]);
        e.call(0x00b2_6160, &args![base, base + 0x38, base + 0x70, 0u8]);
        assert_eq!(keys(&e, base, 3), vec![5.0, 3.0, 1.0]);
        // Over 40 elements: the median of the medians of three spaced an eighth apart. The
        // test repeats the procedure on a plain vector of keys.
        let mut e = engine();
        let count = 49usize;
        let keys_in: Vec<f32> = (0..count).map(|i| ((i * 37) % 49) as f32).collect();
        let base = guides(&mut e, &keys_in);
        let middle = base + 0x38 * 24;
        let last = base + 0x38 * 48;
        e.call(0x00b2_6160, &args![base, middle, last, 0u8]);
        let mut expect = keys_in.clone();
        let step = (48 + 1) / 8;
        let median = |v: &mut Vec<f32>, a: usize, b: usize, c: usize| {
            if v[a] < v[b] {
                v.swap(a, b);
            }
            if v[b] < v[c] {
                v.swap(b, c);
            }
            if v[a] < v[b] {
                v.swap(a, b);
            }
        };
        median(&mut expect, 0, step, 2 * step);
        median(&mut expect, 24 - step, 24, 24 + step);
        median(&mut expect, 48 - 2 * step, 48 - step, 48);
        median(&mut expect, step, 24, 48 - step);
        assert_eq!(keys(&e, base, count as u32), expect);
    }

    #[test]
    fn swapping_two_guides_exchanges_all_their_fields() {
        let mut e = engine();
        let base = guides(&mut e, &[1.0, 2.0]);
        e.call(0x00b2_6ab0, &args![base, base + 0x38]);
        assert_eq!(keys(&e, base, 2), vec![2.0, 1.0]);
        assert_eq!(tags(&e, base, 2), vec![2.0, 1.0]);
        // The same guide twice does nothing.
        e.call_log = Some(vec![]);
        e.call(0x00b2_6ab0, &args![base, base]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn the_insertion_sort_orders_the_guides_descending() {
        let mut e = engine();
        let input = [3.0f32, 1.0, 4.0, 1.5, 5.0, 9.0, 2.0, 6.0];
        let base = guides(&mut e, &input);
        e.call(
            0x00b2_62d0,
            &args![base, base + 0x38 * input.len() as u32, 0u32, 0u32],
        );
        let mut expect = input.to_vec();
        expect.sort_by(|a, b| b.partial_cmp(a).unwrap());
        assert_eq!(keys(&e, base, input.len() as u32), expect);
        // Each tag stayed with its key.
        let t = tags(&e, base, input.len() as u32);
        for (i, key) in expect.iter().enumerate() {
            assert_eq!(
                t[i],
                (input.iter().position(|k| k == key).unwrap() + 1) as f32
            );
        }
        // A range of one guide is left alone.
        let single = guides(&mut e, &[7.0]);
        e.call(0x00b2_62d0, &args![single, single + 0x38, 0u32, 0u32]);
        assert_eq!(keys(&e, single, 1), vec![7.0]);
    }

    /// The words of the guide at `guide`, as the exe passes it by value.
    fn guide_words_of(e: &mut Engine, guide: u32) -> [u32; 14] {
        let mut w = [0u32; 14];
        for (i, word) in w.iter_mut().enumerate() {
            *word = e.mem.u32(guide + 4 * i as u32);
        }
        w
    }

    /// A guide with the key `key` that stands for a by-value argument: its proxy word is a live
    /// block the callee frees.
    fn argument_guide(e: &mut Engine, key: f32) -> u32 {
        let guide = guides(e, &[key]);
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(guide, proxy);
        guide
    }

    #[test]
    fn pushing_onto_the_heap_moves_parents_down() {
        let mut e = engine();
        // A min-heap of keys [1, 3, 2, 5, 4] with room for one more.
        let base = guides(&mut e, &[1.0, 3.0, 2.0, 5.0, 4.0, 0.0]);
        let value = argument_guide(&mut e, 0.0);
        let proxy = e.mem.u32(value);
        fn_00b26e80(&mut e, base, 5, 0, Ptr::new(value), 0);
        assert_eq!(keys(&e, base, 6), vec![0.0, 3.0, 1.0, 5.0, 4.0, 2.0]);
        // The by-value argument was destroyed: its proxy block is gone.
        assert_eq!(e.mem.block_size(proxy), None);
        // The entry takes the words and the comparison byte (18 in all).
        let base = guides(&mut e, &[1.0, 3.0, 2.0, 5.0, 4.0, 0.0]);
        let value = argument_guide(&mut e, 0.0);
        let mut a = vec![base, 5, 0];
        a.extend(guide_words_of(&mut e, value));
        a.push(0);
        e.call(0x00b2_6e80, &a);
        assert_eq!(keys(&e, base, 6), vec![0.0, 3.0, 1.0, 5.0, 4.0, 2.0]);
    }

    #[test]
    fn adjusting_the_heap_sifts_a_new_root_down() {
        let mut e = engine();
        let base = guides(&mut e, &[1.0, 3.0, 2.0, 5.0, 4.0, 6.0]);
        let value = argument_guide(&mut e, 10.0);
        fn_00b26b50(&mut e, base, 0, 6, Ptr::new(value), 0);
        assert_eq!(keys(&e, base, 6), vec![2.0, 3.0, 6.0, 5.0, 4.0, 10.0]);
        // Through the entry, with a heap that has a smaller second child somewhere.
        let base = guides(&mut e, &[9.0, 3.0, 4.0, 8.0, 7.0, 6.0, 5.0]);
        let value = argument_guide(&mut e, 10.0);
        let mut a = vec![base, 0, 7];
        a.extend(guide_words_of(&mut e, value));
        a.push(0);
        e.call(0x00b2_6b50, &a);
        // Index 0: the children 1 (key 3) and 2 (key 4): the first has the smaller key, so
        // child 2 is taken only when child 1's key is below it: the exe takes index 1 (3 < 4
        // is the test on the first against the second); then 3 (8) and 4 (7): 4; then 9 is out
        // of range, the hole is 4 and the value 10 is pushed up from it.
        let k = keys(&e, base, 7);
        let mut sorted = k.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(sorted, vec![3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 10.0]);
        for child in 1..7 {
            assert!(k[(child - 1) / 2] <= k[child], "{k:?}");
        }
    }

    #[test]
    fn popping_the_heap_puts_the_smallest_last() {
        let mut e = engine();
        let input = [1.0f32, 3.0, 2.0, 5.0, 4.0, 6.0];
        let base = guides(&mut e, &input);
        e.call(0x00b2_6ca0, &args![base, base + 0x38 * 6, 0u32]);
        let k = keys(&e, base, 6);
        assert_eq!(k[5], 1.0);
        let mut rest = k[..5].to_vec();
        rest.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(rest, vec![2.0, 3.0, 4.0, 5.0, 6.0]);
        // The remaining five still form a min-heap.
        for child in 1..5 {
            assert!(k[(child - 1) / 2] <= k[child], "{k:?}");
        }
        // A single guide is not popped.
        let single = guides(&mut e, &[2.0]);
        e.call_log = Some(vec![]);
        e.call(0x00b2_6ca0, &args![single, single + 0x38, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    /// The guides the heap routines pass by value are copy constructed in the callee's argument
    /// area and worked on and destroyed there (00b26fe5, 00b26c38, 00b2629f): every guide the
    /// vertex tidy (00b1e970) runs on carries a proxy that names that guide. Stack locals are
    /// separate blocks that are never handed out again here (the freed-block check, reporting
    /// only), so a guide built in one slot and moved into another would name its old slot.
    #[test]
    fn heap_guides_passed_by_value_stay_where_they_were_built() {
        let mut e = engine();
        e.mem.enable_freed_check(false);
        let destroyed: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = destroyed.clone();
        e.register_double(VERTEX_VECTOR_TIDY, move |e, a| {
            let proxy = e.mem.u32(a[0]);
            sink.borrow_mut().push((a[0], e.mem.u32(proxy)));
            Ret::default()
        });
        // Popping (00b26f90): the copy for 00b26b50, the one 00b26b50 makes for 00b26e80, and
        // the temporary.
        let base = guides(&mut e, &[1.0, 3.0, 2.0]);
        e.call(0x00b2_6ca0, &args![base, base + 0x38 * 3, 0u32]);
        // Making a heap (00b26260): the copy for 00b26b50 and the one it makes for 00b26e80.
        let base = guides(&mut e, &[3.0, 1.0, 2.0]);
        e.call(0x00b2_6260, &args![base, base + 0x38 * 3, 0u32, 0u32, 0u32]);
        let destroyed = destroyed.borrow();
        assert_eq!(destroyed.len(), 5, "{destroyed:x?}");
        for (guide, owner) in destroyed.iter() {
            assert_eq!(owner, guide, "{destroyed:x?}");
        }
    }

    #[test]
    fn making_a_heap_and_popping_it_sorts_descending() {
        let mut e = engine();
        let input = [4.0f32, 8.0, 1.0, 9.0, 2.0, 7.0, 3.0, 6.0, 5.0, 0.5];
        let n = input.len() as u32;
        let base = guides(&mut e, &input);
        e.call(0x00b2_6260, &args![base, base + 0x38 * n, 0u32, 0u32, 0u32]);
        let k = keys(&e, base, n);
        for child in 1..k.len() {
            assert!(k[(child - 1) / 2] <= k[child], "{k:?}");
        }
        let mut end = base + 0x38 * n;
        while (end - base) / 0x38 > 1 {
            e.call(0x00b2_6ca0, &args![base, end, 0u32]);
            end -= 0x38;
        }
        let mut expect = input.to_vec();
        expect.sort_by(|a, b| b.partial_cmp(a).unwrap());
        assert_eq!(keys(&e, base, n), expect);
    }

    #[test]
    fn the_partition_gathers_the_keys_equal_to_the_pivot() {
        let mut e = engine();
        let input = [2.0f32, 5.0, 1.0, 5.0, 3.0, 5.0, 4.0, 5.0, 0.0];
        let n = input.len() as u32;
        let base = guides(&mut e, &input);
        let out = e.mem.alloc(8);
        let result = e
            .call(0x00b2_5680, &args![out, base, base + 0x38 * n, 0u8])
            .u32();
        assert_eq!(result, out);
        let (low, high) = (e.mem.u32(out), e.mem.u32(out + 4));
        let k = keys(&e, base, n);
        let (low, high) = ((low - base) / 0x38, (high - base) / 0x38);
        assert!(low < high);
        let pivot = k[low as usize];
        // Larger keys first, then the run of the pivot's key, then the smaller ones.
        for (i, key) in k.iter().enumerate() {
            let i = i as u32;
            if i < low {
                assert!(*key > pivot, "{k:?}");
            } else if i < high {
                assert_eq!(*key, pivot, "{k:?}");
            } else {
                assert!(*key < pivot, "{k:?}");
            }
        }
        let mut sorted = k.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut expect = input.to_vec();
        expect.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(sorted, expect);
    }

    /// Sets the float at `at`.
    fn set_f32(e: &mut Engine, at: u32, value: f32) {
        e.mem.set_f32(at, value);
    }

    /// `count` floats from `at`.
    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// Writes `values` as floats from `at`.
    fn put_floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *value);
        }
    }

    #[test]
    fn the_flag_byte_is_cleared() {
        let mut e = engine();
        let object = e.mem.alloc(0x40);
        e.mem.set_u8(object + 0x30, 5);
        let result = e.call(0x00b2_7060, &args![object]).u32();
        assert_eq!(result, object);
        assert_eq!(e.mem.u8(object + 0x30), 0);
    }

    #[test]
    fn the_billboard_vectors_are_built_once_and_rotated_by_the_angle() {
        let mut e = engine();
        set_f32(&mut e, HALF_FLOAT, 0.5);
        set_f32(&mut e, NEGATIVE_HALF_FLOAT, -0.5);
        e.mem.set_f64(ANGLE_DIVISOR, 57.295_779_513_082_32);
        let atan = Rc::new(RefCell::new(Vec::new()));
        let sink = atan.clone();
        e.register_double(ATAN2, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                st0: 0.5,
                ..Ret::default()
            }
        });
        // The product of a matrix with the matrix at `this` is the rotation itself here.
        e.register(CROT_TRANSFORM_MULTIPLY, |e, a| {
            for i in 0..9 {
                let word = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let matrices = Rc::new(RefCell::new(Vec::new()));
        let seen = matrices.clone();
        e.register_double(VEC3_TIMES_ROTATION, move |e, a| {
            seen.borrow_mut().push(words(e, a[2], 9));
            for i in 0..3 {
                let word = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(a[1] + 4 * i, word);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let direction = e.mem.alloc(12);
        put_floats(&mut e, direction, &[3.0, 4.0, 5.0]);
        e.call(0x00b2_7080, &args![direction]);
        // `atan2(y, x)` with doubles.
        assert_eq!(
            *atan.borrow(),
            vec![vec![
                4.0f64.to_bits() as u32,
                (4.0f64.to_bits() >> 32) as u32,
                3.0f64.to_bits() as u32,
                (3.0f64.to_bits() >> 32) as u32
            ]]
        );
        assert_eq!(e.mem.u32(BILLBOARD_GUARD), 0x1F);
        let table = |e: &Engine| floats(e, BILLBOARD_TABLE, 12);
        assert_eq!(
            table(&e),
            vec![0.0, 0.5, 1.0, 0.0, -0.5, 1.0, 0.0, -0.5, 0.0, 0.0, 0.5, 0.0]
        );
        // The four products are the vectors (the double copies them).
        assert_eq!(floats(&e, BILLBOARD_RESULTS, 12), table(&e));
        // The rotation about z: `(cos, sin, 0)`, `(-sin, cos, 0)`, `(0, 0, 1)` for the angle
        // `0.5 * 57.29578` rounded to a float and divided by the same constant.
        let angle = (0.5f32 as f64 * 57.295_779_513_082_32) as f32;
        let argument = (f64::from(angle) / 57.295_779_513_082_32) as f32;
        let cos = f64::from(argument).cos() as f32;
        let sin = f64::from(argument).sin() as f32;
        let expect: Vec<u32> = [cos, sin, 0.0, -sin, cos, 0.0, 0.0, 0.0, 1.0]
            .iter()
            .map(|f| f.to_bits())
            .collect();
        assert!(matrices.borrow().iter().all(|m| *m == expect));
        assert_eq!(matrices.borrow().len(), 4);
        // A second run keeps the static vectors it already built.
        put_floats(&mut e, BILLBOARD_VECTOR_A, &[0.0, 9.0, 1.0]);
        e.call(0x00b2_7080, &args![direction]);
        assert_eq!(e.mem.f32(BILLBOARD_VECTOR_A + 4), 9.0);
        assert_eq!(e.mem.f32(BILLBOARD_TABLE + 4), 9.0);
    }

    #[test]
    fn rotating_a_matrix_about_z_multiplies_it_with_the_sine_and_cosine() {
        let mut e = engine();
        e.mem.set_f64(ANGLE_DIVISOR, 57.295_779_513_082_32);
        let built = spy(&mut e, CROT_TRANSFORM_CONSTRUCT);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        let product = e.mem.alloc(0x24);
        put_words(&mut e, product, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
        e.register_double(CROT_TRANSFORM_MULTIPLY, move |e, a| {
            sink.borrow_mut().push((words(e, a[0], 9), a[1], a[2]));
            Ret {
                eax: product,
                ..Ret::default()
            }
        });
        let this = e.mem.alloc(0x24);
        fn_00b273d0(&mut e, Ptr::new(this), 90.0);
        assert_eq!(built.borrow().len(), 1);
        let argument = (90.0f64 / 57.295_779_513_082_32) as f32;
        let cos = f64::from(argument).cos() as f32;
        let sin = f64::from(argument).sin() as f32;
        let expect: Vec<u32> = [cos, sin, 0.0, -sin, cos, 0.0, 0.0, 0.0, 1.0]
            .iter()
            .map(|f| f.to_bits())
            .collect();
        let calls = seen.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, expect);
        assert_eq!(calls[0].2, this);
        // The product replaced the matrix at `this`.
        assert_eq!(words(&e, this, 9), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn a_lod_geometry_is_constructed_over_its_leaf_base() {
        let mut e = engine();
        let log = spy(&mut e, LEAF_BASE_CONSTRUCT);
        let lod = e.new_object::<SLodGeometry>();
        e.mem.set_u8(lod.addr() + 0x3C, 1);
        e.mem.set_u32(lod.addr() + 0x40, 0x1234);
        let result = e.call(0x00b2_74a0, &args![lod]).ptr::<SLodGeometry>();
        assert_eq!(result, lod);
        assert_eq!(*log.borrow(), vec![vec![lod.addr()]]);
        assert_eq!(e.mem.u8(lod.addr() + 0x3C), 0);
        assert_eq!(e.mem.u32(lod.addr() + 0x40), 0);
    }

    #[test]
    fn a_lod_geometry_frees_its_arrays_in_order_then_its_base() {
        let mut e = engine();
        let base = spy(&mut e, LEAF_BASE_DESTRUCT);
        let lod = e.new_object::<SLodGeometry>();
        let mut blocks = Vec::new();
        for offset in LOD_ARRAYS.iter().chain(&[0x40u32]) {
            let block = e.mem.alloc(8);
            e.mem.set_u32(lod.addr() + offset, block);
            blocks.push(block);
        }
        e.call(0x00b2_74d0, &args![lod]);
        for block in &blocks {
            assert_eq!(e.mem.block_size(*block), None);
        }
        for offset in LOD_ARRAYS.iter().chain(&[0x40u32]) {
            assert_eq!(e.mem.u32(lod.addr() + offset), 0);
        }
        assert_eq!(*base.borrow(), vec![vec![lod.addr()]]);
    }

    #[test]
    fn a_leaf_geometry_starts_with_three_rocking_groups_and_nothing_else() {
        let mut e = engine();
        let this = e.new_object::<CLeafGeometry>();
        for offset in (0..0x30).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xdead_beef);
        }
        let result = e
            .call(0x00b2_7660, &args![this, 0x4444u32])
            .ptr::<CLeafGeometry>();
        assert_eq!(result, this);
        let t = this.addr();
        assert_eq!((e.mem.u8(t), e.mem.u8(t + 1)), (0, 0));
        assert_eq!(e.mem.u32(t + 4), 0x4444);
        assert_eq!(e.mem.u16(t + 8), 3);
        assert_eq!(e.mem.u16(t + 0x1C), 0);
        assert_eq!(e.mem.u16(t + 0x28), 0);
        for offset in [0x0Cu32, 0x10, 0x14, 0x18, 0x20, 0x24, 0x2C] {
            assert_eq!(e.mem.u32(t + offset), 0);
        }
    }

    #[test]
    fn cleaning_up_a_leaf_geometry_frees_its_tables() {
        let mut e = engine();
        let destroyed = spy(&mut e, LOD_VECTOR_DELETING_DESTRUCTOR);
        let this = e.new_object::<CLeafGeometry>();
        let t = this.addr();
        let (dims, origins, table) = (e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(t + 0x20, dims);
        e.mem.set_u32(t + 0x24, origins);
        e.mem.set_u32(t + 0x18, table);
        e.mem.set_u32(t + 0x2C, 0x5000);
        e.mem.set_u32(t + 0x10, 0x7777);
        e.mem.set_u32(t + 0x0C, 1);
        e.mem.set_u32(t + 0x14, 2);
        e.call(0x00b2_76f0, &args![this]);
        assert_eq!(*destroyed.borrow(), vec![vec![0x5000, 3]]);
        for block in [dims, origins, table] {
            assert_eq!(e.mem.block_size(block), None);
        }
        for offset in [0x0Cu32, 0x14, 0x18, 0x20, 0x24, 0x2C] {
            assert_eq!(e.mem.u32(t + offset), 0);
        }
        // The leaf vertex table is not touched.
        assert_eq!(e.mem.u32(t + 0x10), 0x7777);
        // Without LODs the vector destructor is not called.
        destroyed.borrow_mut().clear();
        let other = e.new_object::<CLeafGeometry>();
        e.call(0x00b2_76f0, &args![other]);
        assert!(destroyed.borrow().is_empty());
    }

    #[test]
    fn the_lod_vector_deleting_destructor_covers_both_forms() {
        let mut e = engine();
        let iterator = spy(&mut e, VECTOR_DESTRUCTOR_ITERATOR);
        // The array form: the count sits before the block.
        let block = e.mem.alloc(4 + 0x88);
        e.mem.set_u32(block, 2);
        let result = e.call(0x00b2_77b0, &args![block + 4, 3u32]).u32();
        assert_eq!(result, block);
        assert_eq!(
            *iterator.borrow(),
            vec![vec![block + 4, 0x44, 2, LOD_DESTRUCTOR]]
        );
        assert_eq!(e.mem.block_size(block), None);
        // The array form without the delete bit leaves the block.
        let block = e.mem.alloc(4 + 0x88);
        e.mem.set_u32(block, 2);
        let result = e.call(0x00b2_77b0, &args![block + 4, 2u32]).u32();
        assert_eq!(result, block);
        assert_eq!(e.mem.block_size(block), Some(4 + 0x88 + 4));
        // The single form destroys one object and deletes it with bit 0.
        let base = spy(&mut e, LEAF_BASE_DESTRUCT);
        let object = e.mem.alloc(0x44);
        let result = e.call(0x00b2_77b0, &args![object, 1u32]).u32();
        assert_eq!(result, object);
        assert_eq!(*base.borrow(), vec![vec![object]]);
        assert_eq!(e.mem.block_size(object), None);
        let object = e.mem.alloc(0x44);
        e.call(0x00b2_77b0, &args![object, 0u32]);
        assert_eq!(e.mem.block_size(object), Some(0x48));
    }

    #[test]
    fn invalidating_the_lods_clears_their_valid_bytes() {
        let mut e = engine();
        let this = e.new_object::<CLeafGeometry>();
        let lods = e.mem.alloc(0x88);
        e.mem.set_u8(lods + 0x3C, 1);
        e.mem.set_u8(lods + 0x44 + 0x3C, 1);
        e.mem.set_u32(this.addr() + 0x2C, lods);
        e.mem.set_u16(this.addr() + 0x28, 2);
        e.call(0x00b2_7820, &args![this]);
        assert_eq!(
            (e.mem.u8(lods + 0x3C), e.mem.u8(lods + 0x44 + 0x3C)),
            (0, 0)
        );
        // Without a LOD array nothing is read.
        let other = e.new_object::<CLeafGeometry>();
        e.mem.set_u16(other.addr() + 0x28, 2);
        e.call(0x00b2_7820, &args![other]);
    }

    #[test]
    fn the_leaf_vertex_table_is_copied_into_the_vertex_program_table() {
        let mut e = engine();
        let this = e.new_object::<CLeafGeometry>();
        let t = this.addr();
        // Two textures and three rocking groups: six 128-byte blocks.
        e.mem.set_u16(t + 0x1C, 2);
        e.mem.set_u16(t + 8, 3);
        let source = e.mem.alloc(768);
        for i in 0..192 {
            e.mem.set_u32(source + 4 * i, 0x6000 + i);
        }
        let table = e.mem.alloc(768);
        let pointers = e.mem.alloc(4);
        e.mem.set_u32(pointers, source);
        e.mem.set_u32(t + 0x10, pointers);
        e.mem.set_u32(t + 0x18, table);
        let out = e.mem.alloc(4);
        let result = e.call(0x00b2_7870, &args![this, out]).u32();
        assert_eq!(result, table);
        assert_eq!(words(&e, table, 192), words(&e, source, 192));
        assert_eq!(e.mem.u32(out), 192);
        // Without a first leaf vertex table nothing is copied.
        e.mem.set_u32(pointers, 0);
        e.mem.set_u32(out, 5);
        e.call(0x00b2_7870, &args![this, out]);
        assert_eq!(e.mem.u32(out), 5);
    }

    #[test]
    fn the_extents_of_all_leaves_grow_by_the_radius_of_each_texture() {
        let mut e = engine();
        // The distance of two vectors, as the real function computes it.
        e.register(VEC3_DISTANCE, |e, a| {
            let (p, q) = (a[0], a[1]);
            let sum: f64 = (0..3)
                .map(|i| {
                    let d = f64::from(e.mem.f32(p + 4 * i)) - f64::from(e.mem.f32(q + 4 * i));
                    d * d
                })
                .sum();
            Ret {
                st0: sum.sqrt(),
                ..Ret::default()
            }
        });
        // The radius vector: `push_back` appends the float at the address.
        e.register(FLOAT_VECTOR_PUSH, |e, a| {
            let (vector, value) = (a[0], e.mem.u32(a[1]));
            if e.mem.u32(vector + 0x0C) == 0 {
                let block = e.mem.alloc(64);
                e.mem.set_u32(vector + 0x0C, block);
                e.mem.set_u32(vector + 0x10, block);
                e.mem.set_u32(vector + 0x14, block + 64);
            }
            let last = e.mem.u32(vector + 0x10);
            e.mem.set_u32(last, value);
            e.mem.set_u32(vector + 0x10, last + 4);
            Ret::default()
        });
        let expand = Rc::new(RefCell::new(Vec::new()));
        let sink = expand.clone();
        e.register_double(EXTENTS_EXPAND, move |e, a| {
            sink.borrow_mut().push(floats(e, a[2], 3));
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register(EXTENTS_MERGE, |_, a| Ret {
            eax: a[1],
            ..Ret::default()
        });
        let merged = spy(&mut e, TRIVIAL_DESTRUCTOR);
        let constructed = spy(&mut e, EXTENTS_CONSTRUCT);
        let this = e.new_object::<CLeafGeometry>();
        let t = this.addr();
        e.mem.set_u16(t + 0x1C, 1);
        let dims = e.mem.alloc(12);
        let origins = e.mem.alloc(12);
        put_floats(&mut e, dims, &[2.0, 4.0, 0.0]);
        put_floats(&mut e, origins, &[0.5, 0.25, 0.0]);
        e.mem.set_u32(t + 0x20, dims);
        e.mem.set_u32(t + 0x24, origins);
        // One LOD with one leaf at (1, 2, 3) of texture byte 1 (texture 0).
        let lods = e.mem.alloc(0x44);
        e.mem.set_u32(t + 0x2C, lods);
        e.mem.set_u16(t + 0x28, 1);
        e.mem.set_u16(lods + 0x0C, 1);
        let centers = e.mem.alloc(12);
        put_floats(&mut e, centers, &[1.0, 2.0, 3.0]);
        e.mem.set_u32(lods + 0x18, centers);
        let texture_bytes = e.mem.alloc(8);
        e.mem.set_u8(texture_bytes, 1);
        e.mem.set_u32(lods + 0x10, texture_bytes);
        let extents = e.mem.alloc(0x30);
        e.call(0x00b2_7910, &args![this, extents]);
        // The radius is the largest corner distance, from (1, 1, 0) to (2, 4, 0) or (0, 4, 0).
        let radius = (10.0f64).sqrt() as f32;
        let points = expand.borrow();
        assert_eq!(points.len(), 6);
        assert_eq!(points[0], vec![1.0 + radius, 2.0, 3.0]);
        assert_eq!(points[1], vec![1.0 - radius, 2.0, 3.0]);
        assert_eq!(points[2], vec![1.0, 2.0 + radius, 3.0]);
        assert_eq!(points[3], vec![1.0, 2.0 - radius, 3.0]);
        assert_eq!(points[4], vec![1.0, 2.0, 3.0 + radius]);
        assert_eq!(points[5], vec![1.0, 2.0, 3.0 - radius]);
        // The leaf's extents were made once; every temporary was destroyed (two parts each).
        assert_eq!(constructed.borrow().len(), 1);
        assert_eq!(merged.borrow().len(), 16);
    }

    #[test]
    fn leaf_geometry_is_transformed_by_a_matrix_and_the_dimensions_are_scaled() {
        let mut e = engine();
        // The product doubles the vector (`this`) into the slot.
        e.register(VEC3_TIMES_MATRIX, |e, a| {
            for i in 0..3 {
                let v = e.mem.f32(a[0] + 4 * i);
                e.mem.set_f32(a[1] + 4 * i, v * 2.0);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let this = e.new_object::<CLeafGeometry>();
        let t = this.addr();
        let lods = e.mem.alloc(0x44);
        e.mem.set_u32(t + 0x2C, lods);
        e.mem.set_u16(t + 0x28, 1);
        e.mem.set_u16(lods + 0x0C, 2);
        let centers = e.mem.alloc(24);
        put_floats(&mut e, centers, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        e.mem.set_u32(lods + 0x18, centers);
        let originals = e.mem.alloc(24);
        put_floats(&mut e, originals, &[7.0, 8.0, 9.0, 1.5, 2.5, 3.5]);
        e.mem.set_u32(lods + 0x40, originals);
        let dims = e.mem.alloc(24);
        put_floats(&mut e, dims, &[1.0, 1.0, 1.0, 2.0, 2.0, 2.0]);
        e.mem.set_u32(t + 0x20, dims);
        e.mem.set_u16(t + 0x1C, 2);
        let matrix = e.mem.alloc(0x24);
        put_floats(
            &mut e,
            matrix,
            &[3.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 3.0],
        );
        e.call(0x00b2_8280, &args![this, matrix]);
        assert_eq!(floats(&e, centers, 6), vec![2.0, 4.0, 6.0, 8.0, 10.0, 12.0]);
        // Without vertex weighting the original centres are left alone.
        assert_eq!(floats(&e, originals, 6), vec![7.0, 8.0, 9.0, 1.5, 2.5, 3.5]);
        assert_eq!(floats(&e, dims, 6), vec![3.0, 3.0, 3.0, 6.0, 6.0, 6.0]);
        // With vertex weighting they are transformed too.
        e.mem.set_u8(t + 1, 1);
        put_floats(&mut e, centers, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        put_floats(&mut e, dims, &[1.0, 1.0, 1.0, 2.0, 2.0, 2.0]);
        e.call(0x00b2_8280, &args![this, matrix]);
        assert_eq!(
            floats(&e, originals, 6),
            vec![14.0, 16.0, 18.0, 3.0, 5.0, 7.0]
        );
    }

    #[test]
    fn a_vector_is_scaled_component_wise() {
        let mut e = engine();
        let v = e.mem.alloc(12);
        put_floats(&mut e, v, &[1.0, -2.0, 0.5]);
        let result = e.call(0x00b2_8440, &args![v, 4.0f32]).u32();
        assert_eq!(result, v);
        assert_eq!(floats(&e, v, 3), vec![4.0, -8.0, 2.0]);
        // A signalling NaN is quieted by the load.
        e.mem.set_u32(v, 0x7f80_0001);
        e.call(0x00b2_8440, &args![v, 1.0f32]);
        let bits = e.mem.u32(v);
        assert!(f32::from_bits(bits).is_nan() && bits & 0x0040_0000 != 0);
    }

    #[test]
    fn the_leaf_data_is_taken_from_the_source_and_the_arrays_made() {
        let mut e = engine();
        let this = e.new_object::<CLeafGeometry>();
        let t = this.addr();
        let source = e.mem.alloc(0x60);
        let s = source;
        e.mem.set_u16(s + 0x40, 3);
        e.mem.set_u32(s + 0x54, 0x111);
        e.mem.set_u32(s + 0x4C, 0x222);
        e.mem.set_u32(s + 0x50, 0x333);
        // Two textures of 0x54 bytes: sizes at `+0x48` and origins at `+0x30`.
        let textures = vector_block(&mut e, s + 0x10, 0x54, 2);
        put_floats(&mut e, textures + 0x48, &[1.0, 2.0, 3.0]);
        put_floats(&mut e, textures + 0x30, &[4.0, 5.0, 6.0]);
        put_floats(&mut e, textures + 0x54 + 0x48, &[7.0, 8.0, 9.0]);
        put_floats(&mut e, textures + 0x54 + 0x30, &[10.0, 11.0, 12.0]);
        e.call(0x00b2_8480, &args![this, 2u32, 0u32, source]);
        assert_eq!(e.mem.u16(t + 8), 3);
        assert_eq!(e.mem.u32(t + 0x0C), 0x111);
        assert_eq!(e.mem.u32(t + 0x10), 0x222);
        assert_eq!(e.mem.u32(t + 0x14), 0x333);
        assert_eq!(e.mem.u16(t + 0x1C), 2);
        assert_eq!(
            floats(&e, e.mem.u32(t + 0x20), 6),
            vec![1.0, 2.0, 3.0, 7.0, 8.0, 9.0]
        );
        assert_eq!(
            floats(&e, e.mem.u32(t + 0x24), 6),
            vec![4.0, 5.0, 6.0, 10.0, 11.0, 12.0]
        );
        // The vertex program table holds textures * groups * 32 words.
        assert_eq!(e.mem.block_size(e.mem.u32(t + 0x18)), Some(2 * 3 * 128));
    }

    /// A vector header at `at` over `count` zeroed elements of `size` bytes; returns the elements.
    fn vector_block(e: &mut Engine, at: u32, size: u32, count: u32) -> u32 {
        vector(e, at, size, count, count)
    }

    #[test]
    fn texture_coordinates_are_stored_twice_with_the_flip_sign() {
        let mut e = engine();
        let this = e.new_object::<CLeafGeometry>();
        let t = this.addr();
        let table = e.mem.alloc(0x80);
        e.mem.set_u32(t + 0x14, table);
        let source = e.mem.alloc(32);
        put_floats(&mut e, source, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        // Without the flip the sign is 1.
        e.call(0x00b2_8720, &args![this, 1u32, source]);
        assert_eq!(
            floats(&e, table + 0x40, 8),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]
        );
        assert_eq!(
            floats(&e, table + 0x60, 8),
            vec![3.0, 2.0, 1.0, 4.0, 7.0, 6.0, 5.0, 8.0]
        );
        // With the flip the odd coordinates are multiplied by the float at `[01012054]`.
        reply(&mut e, FLIP_TEXTURE_V, 1);
        set_f32(&mut e, FLIP_SIGN, -1.0);
        e.call(0x00b2_8720, &args![this, 0u32, source]);
        assert_eq!(
            floats(&e, table, 8),
            vec![1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0]
        );
        assert_eq!(
            floats(&e, table + 0x20, 8),
            vec![3.0, -2.0, 1.0, -4.0, 7.0, -6.0, 5.0, -8.0]
        );
        // Without a table nothing happens.
        let bare = e.new_object::<CLeafGeometry>();
        e.call(0x00b2_8720, &args![bare, 0u32, source]);
    }

    /// A leaf geometry prepared for the LOD builders: one texture, two rocking groups, two
    /// LODs, the leaf vertex table with a block per LOD and a wind engine whose method is
    /// `method`. Returns the geometry, the LOD array and the leaf vertex table.
    fn lod_geometry(e: &mut Engine, method: u32) -> (Ptr<CLeafGeometry>, u32, u32) {
        let this = e.new_object::<CLeafGeometry>();
        let t = this.addr();
        let wind = e.mem.alloc(0x44);
        e.mem.set_u32(wind + 0x10, method);
        e.mem.set_u32(t + 4, wind);
        e.mem.set_u16(t + 8, 2);
        e.mem.set_u32(t + 0x0C, 1);
        let table = e.mem.alloc(8);
        let blocks = [e.mem.alloc(0x400), e.mem.alloc(0x400)];
        e.mem.set_u32(table, blocks[0]);
        e.mem.set_u32(table + 4, blocks[1]);
        e.mem.set_u32(t + 0x10, table);
        e.mem.set_u16(t + 0x1C, 1);
        let dims = e.mem.alloc(12);
        let origins = e.mem.alloc(12);
        put_floats(e, dims, &[2.0, 4.0, 0.0]);
        put_floats(e, origins, &[0.25, 0.75, 0.0]);
        e.mem.set_u32(t + 0x20, dims);
        e.mem.set_u32(t + 0x24, origins);
        let lods = e.mem.alloc(0x88);
        e.mem.set_u32(t + 0x2C, lods);
        e.mem.set_u16(t + 0x28, 2);
        (this, lods, table)
    }

    #[test]
    fn a_lods_corner_vectors_are_built_from_the_origins_and_dimensions() {
        let mut e = engine();
        let (this, lods, table) = lod_geometry(&mut e, 0);
        let blocks = [e.mem.u32(table), e.mem.u32(table + 4)];
        let out = e.mem.alloc(0x40);
        // LOD 1 with the step 0.5 has the scale 1.5.
        e.call(0x00b2_88f0, &args![this, out, 1u32, 0u32, 0u32, 0.5f32]);
        let lod = lods + 0x44;
        assert_eq!(e.mem.u8(lod + 0x3C), 1);
        // Corner set 0 (`1 - origin.x` for the even one): a, b, f, h as in the exe.
        let a = (1.0 - 0.75f32) * 1.5 * 2.0;
        let b = (1.0 - 0.75f32) * 1.5 * 4.0;
        let f = -(0.75f32 * 2.0) * 1.5;
        let h = -(0.75f32 * 4.0) * 1.5;
        let vectors = [[0.0, a, b], [0.0, f, b], [0.0, f, h], [0.0, a, h]];
        for group in 0..2u32 {
            for (i, vector) in vectors.iter().enumerate() {
                let at = blocks[1] + 0x40 * group + 0x10 * i as u32;
                assert_eq!(floats(&e, at, 3), vector.to_vec());
            }
        }
        // Corner set 1 uses the origin as it is.
        let a = (1.0 - 0.25f32) * 1.5 * 2.0;
        let f = -(0.25f32 * 2.0) * 1.5;
        let odd = [[0.0, a, b], [0.0, f, b], [0.0, f, h], [0.0, a, h]];
        for (i, vector) in odd.iter().enumerate() {
            // Block `idx * groups * 64` bytes in, first group.
            let at = blocks[1] + 2 * 0x40 + 0x10 * i as u32;
            assert_eq!(floats(&e, at, 3), vector.to_vec());
        }
        // The output holds the LOD's 15 words with the active flag and the index.
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(e.mem.u32(out + 8), 1);
        // A LOD that is already valid is not built again.
        e.mem.set_f32(blocks[1] + 4, 99.0);
        e.call(0x00b2_88f0, &args![this, out, 1u32, 0u32, 0u32, 0.5f32]);
        assert_eq!(e.mem.f32(blocks[1] + 4), 99.0);
        // An index past the LOD count is refused without touching the output.
        let refused = e.mem.alloc(0x40);
        e.call(0x00b2_88f0, &args![this, refused, 2u32, 0u32, 0u32, 0.5f32]);
        assert_eq!(e.mem.u8(refused), 0);
    }

    #[test]
    fn building_a_lod_first_blends_the_centres_when_the_wind_method_is_one() {
        let mut e = engine();
        let (this, _, _) = lod_geometry(&mut e, 1);
        // The blend is `00b29190`: it needs vertex weighting and the LOD's centres.
        let out = e.mem.alloc(0x40);
        e.call_log = Some(vec![]);
        e.call(0x00b2_88f0, &args![this, out, 0u32, 0u32, 0u32, 0.0f32]);
        let log = e.call_log.take().unwrap();
        // The blend refused (no vertex weighting) and the build went on to mark the LOD.
        assert!(calls(&log, VEC3_TIMES_MATRIX).is_empty());
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn handing_out_a_lod_copies_its_words() {
        let mut e = engine();
        let (this, lods, _) = lod_geometry(&mut e, 0);
        for i in 0..15 {
            e.mem.set_u32(lods + 0x44 + 4 * i, 0x70 + i);
        }
        let out = e.mem.alloc(0x40);
        e.call(0x00b2_8e40, &args![this, out, 1u32, 0u32, 0u32, 0u32]);
        assert_eq!(e.mem.u8(out), 1);
        assert_eq!(e.mem.u32(out + 4), 0x71);
        assert_eq!(e.mem.u32(out + 8), 1);
        assert_eq!(e.mem.u32(out + 12), 0x73);
        // An index past the LOD count leaves the output alone.
        let refused = e.mem.alloc(0x40);
        e.call(0x00b2_8e40, &args![this, refused, 5u32, 0u32, 0u32, 0u32]);
        assert_eq!(e.mem.u8(refused), 0);
    }

    #[test]
    fn freeing_all_lods_frees_every_array() {
        let mut e = engine();
        let this = e.new_object::<CLeafGeometry>();
        let t = this.addr();
        let lods = e.mem.alloc(0x88);
        e.mem.set_u32(t + 0x2C, lods);
        e.mem.set_u16(t + 0x28, 2);
        let mut blocks = Vec::new();
        for lod in 0..2u32 {
            for offset in LOD_ARRAYS.iter().chain(&[0x40u32]) {
                let block = e.mem.alloc(8);
                e.mem.set_u32(lods + 0x44 * lod + offset, block);
                blocks.push(block);
            }
        }
        e.call(0x00b2_8ed0, &args![this]);
        for block in &blocks {
            assert_eq!(e.mem.block_size(*block), None);
        }
        for lod in 0..2u32 {
            for offset in LOD_ARRAYS.iter().chain(&[0x40u32]) {
                assert_eq!(e.mem.u32(lods + 0x44 * lod + offset), 0);
            }
        }
        // Without a LOD count nothing is read.
        let other = e.new_object::<CLeafGeometry>();
        e.mem.set_u32(other.addr() + 0x2C, 0x1234);
        e.call(0x00b2_8ed0, &args![other]);
    }

    #[test]
    fn the_wind_blend_moves_each_centre_toward_the_matrix_product() {
        let mut e = engine();
        // The wind matrices: the base pointer lives at `[011f8d88]`.
        e.mem.set_u32(WIND_MATRICES, 0x0150_0000);
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(VEC3_TIMES_MATRIX, move |e, a| {
            sink.borrow_mut().push(a[2]);
            put_floats(e, a[1], &[5.0, 6.0, 7.0]);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        let (this, lods, _) = lod_geometry(&mut e, 0);
        e.mem.set_u8(this.addr() + 1, 1);
        let lod = lods + 0x44;
        e.mem.set_u16(lod + 0x0C, 1);
        let originals = e.mem.alloc(12);
        put_floats(&mut e, originals, &[1.0, 2.0, 3.0]);
        let centers = e.mem.alloc(12);
        let weights = e.mem.alloc(4);
        put_floats(&mut e, weights, &[0.25]);
        let matrix_bytes = e.mem.alloc(8);
        e.mem.set_u8(matrix_bytes, 3);
        e.mem.set_u32(lod + 0x40, originals);
        e.mem.set_u32(lod + 0x18, centers);
        e.mem.set_u32(lod + 0x34, weights);
        e.mem.set_u32(lod + 0x38, matrix_bytes);
        e.call(0x00b2_9190, &args![this, 1u32]);
        // `c + (v - c) * w` per axis.
        assert_eq!(floats(&e, centers, 3), vec![2.0, 3.0, 4.0]);
        assert_eq!(*seen.borrow(), vec![0x0150_0000 + (3 << 6)]);
        // Without vertex weighting nothing is done.
        e.mem.set_u8(this.addr() + 1, 0);
        put_floats(&mut e, centers, &[0.0, 0.0, 0.0]);
        e.call(0x00b2_9190, &args![this, 1u32]);
        assert_eq!(floats(&e, centers, 3), vec![0.0, 0.0, 0.0]);
    }

    /// A leaf object (`CBillboardLeaf`, 0x4C bytes) with the fields the LOD arrays read.
    fn leaf(e: &mut Engine, index: u32, texture: u8, angle: u8, group: u8) -> u32 {
        let leaf = e.mem.alloc(0x4C);
        let f = index as f32;
        put_floats(e, leaf + 4, &[f + 1.0, f + 2.0, f + 3.0]);
        e.mem.set_u8(leaf + 0x10, angle);
        e.mem.set_u32(leaf + 0x14, 0xAA00_0000 + index);
        put_floats(e, leaf + 0x1C, &[f + 0.1, f + 0.2, f + 0.3]);
        put_floats(e, leaf + 0x28, &[f + 0.4, f + 0.5, f + 0.6]);
        put_floats(e, leaf + 0x34, &[f + 0.7, f + 0.8, f + 0.9]);
        e.mem.set_u8(leaf + 0x40, texture);
        put_floats(e, leaf + 0x44, &[0.25 * (index + 1) as f32]);
        e.mem.set_u32(leaf + 0x48, u32::from(group));
        leaf
    }

    #[test]
    fn the_lod_arrays_are_filled_from_the_leaf_lists() {
        let mut e = engine();
        let this = e.new_object::<CLeafGeometry>();
        let t = this.addr();
        // Two rocking groups; the leaf vertex table has one block per LOD.
        e.mem.set_u16(t + 8, 2);
        let table = e.mem.alloc(8);
        let block = e.mem.alloc(0x400);
        e.mem.set_u32(table, block);
        e.mem.set_u32(t + 0x10, table);
        let texcoords = e.mem.alloc(0x100);
        e.mem.set_u32(t + 0x14, texcoords);
        let wind = e.mem.alloc(0x44);
        e.mem.set_u32(wind + 0x28, 10);
        e.mem.set_u32(wind + 0x2C, 4);
        e.mem.set_u32(t + 4, wind);
        e.mem.set_u8(t + 1, 1);
        // One LOD of two leaves: texture 1 angle 2 wind group 6, texture 0 angle 1 group 3.
        let first = leaf(&mut e, 0, 1, 2, 6);
        let second = leaf(&mut e, 1, 0, 1, 3);
        let list = e.mem.alloc(0x18);
        let items = vector_block(&mut e, list, 4, 2);
        e.mem.set_u32(items, first);
        e.mem.set_u32(items + 4, second);
        e.call(0x00b2_9380, &args![this, 1u32, list]);
        assert_eq!(e.mem.u16(t + 0x28), 1);
        let lods = e.mem.u32(t + 0x2C);
        assert_eq!(e.mem.u32(lods - 4), 1);
        assert_eq!(e.mem.u16(lods + 0x0C), 2);
        // `+0x20`: block + (groups * 16 * texture + angle * 16) * 4.
        let slots = e.mem.u32(lods + 0x20);
        assert_eq!(e.mem.u32(slots), block + (2 * 16 + 2 * 16) * 4);
        assert_eq!(e.mem.u32(slots + 4), block + 16 * 4);
        // Vertex weighting: positions, `1 - wind weight`, wind matrix bytes.
        assert_eq!(
            floats(&e, e.mem.u32(lods + 0x40), 6),
            vec![1.0, 2.0, 3.0, 2.0, 3.0, 4.0]
        );
        assert_eq!(floats(&e, e.mem.u32(lods + 0x34), 2), vec![0.75, 0.5]);
        // Group 6 mod span 4 = 2 plus the start 10; group 3 mod 4 = 3 plus 10.
        let matrices = e.mem.u32(lods + 0x38);
        assert_eq!((e.mem.u8(matrices), e.mem.u8(matrices + 1)), (12, 13));
        assert_eq!(
            floats(&e, e.mem.u32(lods + 0x18), 6),
            vec![1.0, 2.0, 3.0, 2.0, 3.0, 4.0]
        );
        // Texture bytes, `groups * texture + angle`, the texture coordinate slots and colours.
        let bytes = e.mem.u32(lods + 0x10);
        assert_eq!((e.mem.u8(bytes), e.mem.u8(bytes + 1)), (1, 0));
        let angles = e.mem.u32(lods + 0x14);
        assert_eq!((e.mem.u8(angles), e.mem.u8(angles + 1)), (4, 1));
        let coords = e.mem.u32(lods + 0x1C);
        assert_eq!(
            (e.mem.u32(coords), e.mem.u32(coords + 4)),
            (texcoords + 32, texcoords)
        );
        let colors = e.mem.u32(lods + 0x24);
        assert_eq!(
            (e.mem.u32(colors), e.mem.u32(colors + 4)),
            (0xAA00_0000, 0xAA00_0001)
        );
        // Without manual lighting: normals, tangents and binormals.
        assert_eq!(floats(&e, e.mem.u32(lods + 0x28), 3), vec![0.1, 0.2, 0.3]);
        assert_eq!(floats(&e, e.mem.u32(lods + 0x30), 3), vec![0.4, 0.5, 0.6]);
        assert_eq!(floats(&e, e.mem.u32(lods + 0x2C), 3), vec![0.7, 0.8, 0.9]);
    }

    #[test]
    fn the_lod_arrays_need_the_tables_and_leaves() {
        let mut e = engine();
        let this = e.new_object::<CLeafGeometry>();
        e.call_log = Some(vec![]);
        e.call(0x00b2_9380, &args![this, 1u32, 0x1234u32]);
        // The first test fails: no vertex table, so nothing is allocated.
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u16(this.addr() + 0x28), 0);
    }

    #[test]
    fn the_lighting_engine_starts_with_its_material_blocks() {
        let mut e = engine();
        set_f32(&mut e, MATERIAL_FIRST, 0.8);
        set_f32(&mut e, MATERIAL_SECOND, 0.2);
        set_f32(&mut e, HALF_FLOAT, 0.5);
        let this = e.mem.alloc(0xB0);
        for i in 0..0x2C {
            e.mem.set_u32(this + 4 * i, 0xdead_beef);
        }
        let result = e.call(0x00b2_9fd0, &args![this]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.f32(this + 0x70), 0.5);
        assert_eq!(e.mem.u32(this + 0x74), 0);
        assert_eq!(e.mem.u32(this + 0x78), 0);
        let first = [
            0.8f32, 0.8, 0.8, 0.2, 0.2, 0.2, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let other = [
            1.0f32, 1.0, 1.0, 0.5, 0.5, 0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
        ];
        assert_eq!(floats(&e, this + 4, 13), first.to_vec());
        assert_eq!(floats(&e, this + 0x3C, 13), other.to_vec());
        assert_eq!(floats(&e, this + 0x7C, 13), other.to_vec());
    }

    #[test]
    fn the_dot_product_adds_the_three_products_left_to_right() {
        let mut e = engine();
        let a = e.mem.alloc(12);
        let b = e.mem.alloc(12);
        put_floats(&mut e, a, &[1.0, 2.0, 3.0]);
        put_floats(&mut e, b, &[4.0, 5.0, 6.0]);
        let result = e.call(0x00b2_a180, &args![a, b]).f32();
        assert_eq!(result, 32.0);
        // The sum is formed in the exe's order: `(x + y) + z`, each product exact.
        put_floats(&mut e, a, &[1.0e8, 1.0, -1.0e8]);
        put_floats(&mut e, b, &[1.0, 1.0, 1.0]);
        assert_eq!(e.call(0x00b2_a180, &args![a, b]).f32(), 1.0);
        // A NaN operand comes back quiet.
        e.mem.set_u32(a, 0x7f80_0001);
        let nan = e.call(0x00b2_a180, &args![a, b]).f32();
        assert!(nan.is_nan());
    }
}
