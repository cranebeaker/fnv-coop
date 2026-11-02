//! `SpeedTree/branch.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! SpeedTreeRT's branch builder: the `CVec3` and `CRotTransform` math it uses, the
//! `CIdvBranch` class (children, vertices, flares), the leaf placement and the MSVC
//! `std::vector` instantiations that live in this unit.
//!
//! The PC build's `std::vector` is 0x18 bytes (the checked-iterator proxy at `+0`, the
//! first, last and end pointers at `+0x0C`, `+0x10`, `+0x14`; [`StlVector`]), so every
//! class that holds one is 8 bytes per vector larger than in the Xbox PDB.
//!
//! x87 note: the exe computes in extended precision and stores `float` results. The
//! translations compute in `f64`, in the exe's order of operations, and round to `f32`
//! at each `float` store (`FSTP dword`). A float copied through the x87 stack
//! (`FLD dword`, `FSTP dword`) goes through [`x87_quiet_bits`].
//!
//! Functions that carry an `FS:[0]` exception frame are marked `Translation incomplete`
//! (the unwinding handler is not translated).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`, one byte count) and `operator delete` (`00401030`).
pub(crate) const OPERATOR_NEW: u32 = 0x0040_1000;
pub(crate) const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The runtime's invalid-parameter path the checked `std::vector` code calls when an
/// index or iterator is out of range (`00ec7c56`).
pub(crate) const INVALID_PARAMETER: u32 = 0x00ec_7c56;
/// C runtime helpers: `sqrt` (`00ec6054`), `cos` (`00ec9f30`) and `sin` (`00eca060`), each
/// `cdecl` with one `double`, and `acos` (`00eca190`, `cdecl`, one `double`; the engine map
/// calls it `cos` because the linker folded the two entry stubs), `pow` (`00ec9ac0`,
/// `cdecl`, base then exponent), the `fabs`-style helper `00ec6cde` (`cdecl`, one
/// `double`; Ghidra names it `_fabs`), `_ftol2` (`00ec62c0`, the value in `ST0`), `rand`
/// (`00ecadb8`) and `memcpy` (`00ec44d0`).
pub(crate) const SQRT: u32 = 0x00ec_6054;
pub(crate) const COS: u32 = 0x00ec_9f30;
pub(crate) const SIN: u32 = 0x00ec_a060;
const ACOS: u32 = 0x00ec_a190;
const POW: u32 = 0x00ec_9ac0;
const ABSOLUTE_VALUE: u32 = 0x00ec_6cde;
pub(crate) const FTOL: u32 = 0x00ec_62c0;
const RAND: u32 = 0x00ec_adb8;
pub(crate) const MEMCPY: u32 = 0x00ec_44d0;

/// `CIdvRandom::GetUniform` (`00b3f130`, `this` = the static generator, a low and a high
/// `float`) and the static generator object itself (`CIdvBranch::m_cRandom`, Xbox PDB; the
/// address is matched by its uses).
pub(crate) const GET_UNIFORM: u32 = 0x00b3_f130;
pub(crate) const RANDOM_GENERATOR: u32 = 0x011f_8c05;
/// `CIdvBezierSpline::Evaluate` (`00b37ad0`, `this` = the spline, one `float`, result in `ST0`).
const SPLINE_EVALUATE: u32 = 0x00b3_7ad0;
/// `CIdvBezierSpline::CIdvBezierSpline` taking the parsed string (`00b37660`).
const SPLINE_CONSTRUCT: u32 = 0x00b3_7660;
/// `CTreeFileAccess::ParseString` (`00b3ee40`, `this` = the file access, the string to fill).
const PARSE_STRING: u32 = 0x00b3_ee40;
/// The `std::string` tidy (`0044a730`, `this` = the string, a flag and a size).
const STRING_TIDY: u32 = 0x0044_a730;
/// `CVec3` helpers outside this unit: `Distance` (`00b04d40`), the sum `other + this`
/// (`00b0cf90`), the product `this * scalar` (`00b0cff0`) and the dot product (`00b2a180`).
const VEC3_DISTANCE: u32 = 0x00b0_4d40;
const VEC3_ADD: u32 = 0x00b0_cf90;
const VEC3_SCALE: u32 = 0x00b0_cff0;
const VEC3_DOT: u32 = 0x00b2_a180;

/// `CBillboardLeaf::CBillboardLeaf` (`00b40070`), `CBillboardLeaf::SetColor` (`00b404c0`).
const BILLBOARD_LEAF_CONSTRUCT: u32 = 0x00b4_0070;
const BILLBOARD_LEAF_SET_COLOR: u32 = 0x00b4_04c0;

/// The statics of `CIdvBranch` (Xbox PDB names, the addresses matched by their uses):
/// `m_pLeafInfo` (a pointer to the leaf info).
pub(crate) const LEAF_INFO: u32 = 0x011f_8bf4;
/// The index vector `BuildBlossomVectors` fills for leaf textures whose first byte is set.
const BLOSSOM_INDICES: u32 = 0x011f_8c68;
/// The index vector it fills for the other leaf textures.
const NON_BLOSSOM_INDICES: u32 = 0x011f_8c18;
/// The vector of `SIdvBranchInfo*` (`m_vBranchInfo`).
const BRANCH_INFO_LIST: u32 = 0x011f_8c50;
/// The vector of leaf pointers collected for collision (`m_vLocalLeaves`).
const LOCAL_LEAVES: u32 = 0x011f_8c80;
/// A global `CVec3` the bud computation multiplies by the branch transform (`011f8c40`).
const BUD_AXIS: u32 = 0x011f_8c40;
/// A global `float` the code wraps angles by and scales a turn by (`011f8c14`; set by a
/// static initializer, so read from memory) and a global `float` offset added to the
/// ring angle (`011f8c30`).
pub(crate) const ANGLE_PERIOD: u32 = 0x011f_8c14;
pub(crate) const RING_ANGLE_OFFSET: u32 = 0x011f_8c30;
/// A global word `00b14ae0` compares its first argument with (`011ad14c`).
const LEVEL_MARKER: u32 = 0x011a_d14c;
/// The `double` the rotation helpers divide their angle by (`010a4d90`).
pub(crate) const ANGLE_DIVISOR: u32 = 0x010a_4d90;

layout! {
    /// The PC `std::vector` (VC9, checked iterators), 0x18 bytes: the proxy at `+0`, the first,
    /// last and end pointers at `+0x0C`, `+0x10` and `+0x14`.
    pub struct StlVector: 0x18 {
        /// The checked-iterator proxy (`_Myproxy`).
        0x00 m_proxy: u32,
        /// `_Myfirst`.
        0x0C m_first: u32,
        /// `_Mylast`.
        0x10 m_last: u32,
        /// `_Myend`.
        0x14 m_end: u32,
    }

    /// `CVec3` (Xbox PDB `m_afData[3]`): three floats.
    pub struct CVec3: 0x0C {
        0x00 x: f32,
        0x04 y: f32,
        0x08 z: f32,
    }

    /// `CRotTransform` (Xbox PDB `m_afData[9]`): a 3x3 matrix of nine floats, row by row.
    pub struct CRotTransform: 0x24 {
        0x00 m_afData: f32,
    }

    /// `SIdvBranchVertex` (Xbox PDB), 0x48 bytes.
    pub struct SIdvBranchVertex: 0x48 {
        /// `m_cDirection` (Xbox PDB).
        0x00 m_cDirection: Inline<CVec3>,
        /// `m_cPos` (Xbox PDB).
        0x0C m_cPos: Inline<CVec3>,
        /// `m_fRadius` (Xbox PDB).
        0x18 m_fRadius: f32,
        /// `m_cTrans` (Xbox PDB).
        0x1C m_cTrans: Inline<CRotTransform>,
        /// `m_fRunningLength` (Xbox PDB).
        0x40 m_fRunningLength: f32,
        /// `m_fWindWeight` (Xbox PDB).
        0x44 m_fWindWeight: f32,
    }

    /// `SIdvBranch` (Xbox PDB), 0x0C bytes: an entry of a branch's child vector.
    pub struct SIdvBranch: 0x0C {
        /// `m_nPreVertexIndex` (Xbox PDB).
        0x00 m_nPreVertexIndex: i32,
        /// `m_fPercent` (Xbox PDB).
        0x04 m_fPercent: f32,
        /// `m_pBranch` (Xbox PDB).
        0x08 m_pBranch: Ptr<CIdvBranch>,
    }

    /// `SIdvBranchFlare` (Xbox PDB), 0x18 bytes.
    pub struct SIdvBranchFlare: 0x18 {
        /// `m_fAngle` (Xbox PDB).
        0x00 m_fAngle: f32,
        /// `m_fRadialInfluence` (Xbox PDB).
        0x04 m_fRadialInfluence: f32,
        /// `m_fRadialExponent` (Xbox PDB).
        0x08 m_fRadialExponent: f32,
        /// `m_fLengthInfluence` (Xbox PDB).
        0x0C m_fLengthInfluence: f32,
        /// `m_fLengthExponent` (Xbox PDB).
        0x10 m_fLengthExponent: f32,
        /// `m_fDistance` (Xbox PDB).
        0x14 m_fDistance: f32,
    }

    /// `CIdvBranch` (Xbox PDB): 0x40 bytes on the Xbox, 0x50 on the PC (two `std::vector`s
    /// of 0x18 bytes; the Xbox PDB offsets after `m_vChildren` are 8 higher here).
    pub struct CIdvBranch: 0x50 {
        /// `m_pParent` (Xbox PDB).
        0x00 m_pParent: Ptr<CIdvBranch>,
        /// `m_fPercentOfParent` (Xbox PDB).
        0x04 m_fPercentOfParent: f32,
        /// `m_vChildren` (Xbox PDB): a `std::vector<SIdvBranch>`.
        0x08 m_vChildren: Inline<StlVector>,
        /// `m_pVertices` (Xbox PDB `+0x18`).
        0x20 m_pVertices: Ptr<SIdvBranchVertex>,
        /// `m_nVertexCount` (Xbox PDB `+0x1C`).
        0x24 m_nVertexCount: i32,
        /// `m_usCrossSectionSegments` (Xbox PDB `+0x20`).
        0x28 m_usCrossSectionSegments: u16,
        /// `m_nStartingOffset` (Xbox PDB `+0x24`).
        0x2C m_nStartingOffset: i32,
        /// `m_fVolume` (Xbox PDB `+0x28`).
        0x30 m_fVolume: f32,
        /// `m_fFuzzyVolume` (Xbox PDB `+0x2C`).
        0x34 m_fFuzzyVolume: f32,
        /// `m_vFlares` (Xbox PDB `+0x30`): a `std::vector<SIdvBranchFlare>`.
        0x38 m_vFlares: Inline<StlVector>,
    }
}

/// `FLD dword`: a float field as the x87 value.
pub(crate) fn ld(e: &Engine, addr: u32) -> f64 {
    f64::from(e.mem.f32(addr))
}

/// `FSTP dword`: the x87 value rounded to a float field.
pub(crate) fn st(e: &mut Engine, addr: u32, value: f64) {
    e.mem.set_f32(addr, value as f32);
}

/// `FLD dword` then `FSTP dword` to another place: a copy that quiets a signalling NaN.
pub(crate) fn copy_float(e: &mut Engine, dst: u32, src: u32) {
    let bits = x87_quiet_bits(e.mem.u32(src));
    e.mem.set_u32(dst, bits);
}

/// `MOV` / `MOVSD` word copies.
pub(crate) fn copy_words(e: &mut Engine, dst: u32, src: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(src + 4 * i);
        e.mem.set_u32(dst + 4 * i, word);
    }
}

/// Element count of a `std::vector` whose elements are `size` bytes: `(last - first) / size`,
/// as the compiler's signed `IDIV` (an arithmetic shift for a power of two).
pub(crate) fn vector_len(e: &Engine, vector: u32, size: i32) -> u32 {
    let bytes = e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C)) as i32;
    if size.count_ones() == 1 {
        (bytes >> size.trailing_zeros()) as u32
    } else {
        (bytes / size) as u32
    }
}

/// The checked `operator[]`: the address of element `index` (`first + index * size`); an index
/// not below the element count takes the invalid-parameter path.
pub(crate) fn vector_at(e: &mut Engine, vector: u32, index: u32, size: u32) -> u32 {
    if index >= vector_len(e, vector, size as i32) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    index
        .wrapping_mul(size)
        .wrapping_add(e.mem.u32(vector + 0x0C))
}

/// `trig(angle / divisor)` as the rotation helpers compute it: the quotient is rounded to a
/// float, passed as a `double` to `function` (`cos` or `sin`) and the result rounded to a float.
pub(crate) fn scaled_trig(e: &mut Engine, function: u32, angle: f32) -> f32 {
    let divisor: f64 = e.global(ANGLE_DIVISOR);
    let argument = (f64::from(angle) / divisor) as f32;
    e.call(function, &args![f64::from(argument)]).f32()
}

// Translated from 00b10dc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b10dc0 - the FS:[0] exception frame (unwinding of the children
// vector if a later step throws) is not translated.
/// `CIdvBranch::CIdvBranch` (Xbox PDB): stores the parent and a zero percent, constructs the
/// children vector (`00b1e3e0`), clears the vertex pointer, sets the vertex count and the
/// starting offset to -1, the cross-section segment count and the volumes to zero and
/// constructs the flare vector (`00b31270`).
pub fn c_idv_branch_c_idv_branch(
    e: &mut Engine,
    this: Ptr<CIdvBranch>,
    parent: Ptr<CIdvBranch>,
) -> Ptr<CIdvBranch> {
    e.set(this, CIdvBranch::m_pParent, parent);
    e.set(this, CIdvBranch::m_fPercentOfParent, 0.0);
    e.call(0x00b1_e3e0, &args![this.addr() + 8]);
    e.set(this, CIdvBranch::m_pVertices, Ptr::NULL);
    e.set(this, CIdvBranch::m_nVertexCount, -1);
    e.set(this, CIdvBranch::m_usCrossSectionSegments, 0);
    e.set(this, CIdvBranch::m_nStartingOffset, -1);
    e.set(this, CIdvBranch::m_fVolume, 0.0);
    e.set(this, CIdvBranch::m_fFuzzyVolume, 0.0);
    e.call(0x00b3_1270, &args![this.addr() + 0x38]);
    this
}

// Translated from 00b10e90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b10e90 - the FS:[0] exception frame is not translated.
/// `CIdvBranch::~CIdvBranch` (Xbox PDB): frees the vertex array, destroys and frees every
/// child branch (clearing the child's pointer), then tears down the flare vector
/// (`00b15600`, and frees its proxy) and the children vector (`00b1fca0`, and frees its proxy).
pub fn c_idv_branch_destructor(e: &mut Engine, this: Ptr<CIdvBranch>) {
    let vertices = e.get(this, CIdvBranch::m_pVertices);
    e.call(OPERATOR_DELETE, &args![vertices]);
    e.set(this, CIdvBranch::m_pVertices, Ptr::NULL);
    let children = this.addr() + 8;
    let mut index = 0u32;
    while index < vector_len(e, children, 0x0C) {
        let entry = vector_at(e, children, index, 0x0C);
        let child: Ptr<CIdvBranch> = e.get(Ptr::<SIdvBranch>::new(entry), SIdvBranch::m_pBranch);
        if !child.is_null() {
            c_idv_branch_destructor(e, child);
            e.call(OPERATOR_DELETE, &args![child]);
        }
        let entry = vector_at(e, children, index, 0x0C);
        e.set(
            Ptr::<SIdvBranch>::new(entry),
            SIdvBranch::m_pBranch,
            Ptr::NULL,
        );
        index += 1;
    }
    let flares = this.addr() + 0x38;
    fn_00b15600(e, Ptr::new(flares));
    let flare_proxy = e.mem.u32(flares);
    e.call(OPERATOR_DELETE, &args![flare_proxy]);
    e.call(0x00b1_fca0, &args![children]);
    let children_proxy = e.mem.u32(children);
    e.call(OPERATOR_DELETE, &args![children_proxy]);
}

// Translated from 00b12720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CVec3::AngleBetween` (Xbox PDB): the arc cosine of the dot product (`00b2a180`) of `this`
/// and `other`, the dot clamped to [-1, 1] (a NaN is not clamped).
pub fn c_vec3_angle_between(e: &mut Engine, this: Ptr<CVec3>, other: Ptr<CVec3>) -> f32 {
    let mut cosine = e.call(VEC3_DOT, &args![this, other]).f32();
    let minus_one: f64 = e.global(0x0101_a6b0);
    if f64::from(cosine) < minus_one {
        cosine = x87_quiet(e.global::<f32>(0x0101_2054));
    }
    let one: f64 = e.global(0x0101_2070);
    if f64::from(cosine) > one {
        cosine = 1.0;
    }
    e.call(ACOS, &args![f64::from(cosine)]).f32()
}

// Translated from 00b12790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CVec3::Normalize` (Xbox PDB): scales `this` by `1 / sqrt(x*x + y*y + z*z)` (the sum and the
/// root are rounded to floats; a zero length gives infinities and NaNs as the exe does).
pub fn c_vec3_normalize(e: &mut Engine, this: Ptr<CVec3>) {
    let base = this.addr();
    let (x, y, z) = (ld(e, base), ld(e, base + 4), ld(e, base + 8));
    let sum = ((x * x + y * y) + z * z) as f32;
    let root = e.call(SQRT, &args![f64::from(sum)]).f64() as f32;
    let inverse = (1.0 / f64::from(root)) as f32;
    let scale = f64::from(inverse);
    st(e, base, ld(e, base) * scale);
    st(e, base + 4, ld(e, base + 4) * scale);
    st(e, base + 8, ld(e, base + 8) * scale);
}

// Translated from 00b12810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CVec3::operator%` (Xbox PDB `operatorP`): the cross product of `this` and `other`, written to
/// `out`; returns `out`.
pub fn c_vec3_operator_p(
    e: &mut Engine,
    this: Ptr<CVec3>,
    out: Ptr<CVec3>,
    other: Ptr<CVec3>,
) -> Ptr<CVec3> {
    // `FLD other[i]`, `FMUL this[j]` (a memory operand), `FSUBRP`: the x87 choice of NaN.
    let (t, o) = (this.addr(), other.addr());
    let product = |e: &Engine, i: u32, j: u32| {
        x87_mul(x87_load32(e.mem.u32(o + i)), x87_m32(e.mem.u32(t + j)))
    };
    let x = x87_sub(product(e, 8, 4), product(e, 4, 8));
    let y = x87_sub(product(e, 0, 8), product(e, 8, 0));
    let z = x87_sub(product(e, 4, 0), product(e, 0, 4));
    st(e, out.addr(), x);
    st(e, out.addr() + 4, y);
    st(e, out.addr() + 8, z);
    out
}

// Translated from 00b12890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CVec3::operator*` by a matrix (Xbox PDB `operatorP_ov2`): the vector `this` times the nine
/// floats at `matrix` (component `j` is the sum of `this[i] * matrix[3 * i + j]`), written to
/// `out`; returns `out`.
pub fn c_vec3_operator_p_ov2(
    e: &mut Engine,
    this: Ptr<CVec3>,
    out: Ptr<CVec3>,
    matrix: Ptr<CRotTransform>,
) -> Ptr<CVec3> {
    let (t, m) = (this.addr(), matrix.addr());
    let mut result = [0.0f64; 3];
    for (j, slot) in result.iter_mut().enumerate() {
        let j = j as u32;
        // `FLD matrix[..]`, `FMUL this[..]` (a memory operand), two `FADDP`: the x87 choice
        // of NaN.
        let product = |row: u32, lane: u32| {
            x87_mul(
                x87_load32(e.mem.u32(m + row + 4 * j)),
                x87_m32(e.mem.u32(t + lane)),
            )
        };
        *slot = x87_add(product(0x18, 8), x87_add(product(0x0c, 4), product(0, 0)));
    }
    for (j, value) in result.iter().enumerate() {
        st(e, out.addr() + 4 * j as u32, *value);
    }
    out
}

/// The nine floats of a matrix as x87 values.
fn matrix(e: &Engine, base: u32) -> [f64; 9] {
    let mut m = [0.0; 9];
    for (i, slot) in m.iter_mut().enumerate() {
        *slot = ld(e, base + 4 * i as u32);
    }
    m
}

// Translated from 00b12940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CRotTransform::operator*` (Xbox PDB `operatorP`): the matrix product `this * other`, built in
/// an identity-initialised temporary (`00b12af0`) and copied to `out`; returns `out`.
pub fn c_rot_transform_operator_p(
    e: &mut Engine,
    this: Ptr<CRotTransform>,
    out: Ptr<CRotTransform>,
    other: Ptr<CRotTransform>,
) -> Ptr<CRotTransform> {
    e.with_stack(0x24, |e, temp| {
        c_rot_transform_c_rot_transform(e, temp.cast());
        let a = matrix(e, this.addr());
        let b = matrix(e, other.addr());
        for row in 0..3 {
            for col in 0..3 {
                let value = (a[3 * row] * b[col] + a[3 * row + 1] * b[3 + col])
                    + a[3 * row + 2] * b[6 + col];
                st(e, temp.addr() + 4 * (3 * row + col) as u32, value);
            }
        }
        copy_words(e, out.addr(), temp.addr(), 9);
    });
    out
}

// Translated from 00b12af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CRotTransform::CRotTransform` (Xbox PDB): the identity matrix; returns `this`.
pub fn c_rot_transform_c_rot_transform(
    e: &mut Engine,
    this: Ptr<CRotTransform>,
) -> Ptr<CRotTransform> {
    let base = this.addr();
    for i in 0..9u32 {
        let value = if i % 4 == 0 { 1.0 } else { 0.0 };
        e.mem.set_f32(base + 4 * i, value);
    }
    this
}

// Translated from 00b12b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CRotTransform::operator*` by a vector (Xbox PDB `operatorP_ov2`): component `j` of `out` is
/// the sum of `other[i] * this[3 * j + i]`; returns `out`.
pub fn c_rot_transform_operator_p_ov2(
    e: &mut Engine,
    this: Ptr<CRotTransform>,
    out: Ptr<CVec3>,
    other: Ptr<CVec3>,
) -> Ptr<CVec3> {
    let (t, o) = (this.addr(), other.addr());
    let mut result = [0.0f64; 3];
    for (j, slot) in result.iter_mut().enumerate() {
        let j = j as u32;
        // `FLD other[..]`, `FMUL this[..]` (a memory operand), two `FADDP`: the x87 choice
        // of NaN.
        let product = |lane: u32| {
            x87_mul(
                x87_load32(e.mem.u32(o + lane)),
                x87_m32(e.mem.u32(t + 12 * j + lane)),
            )
        };
        *slot = x87_add(product(8), x87_add(product(4), product(0)));
    }
    for (j, value) in result.iter().enumerate() {
        st(e, out.addr() + 4 * j as u32, *value);
    }
    out
}

// Translated from 00b12c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CRotTransform::RotateYZ` (Xbox PDB): multiplies `this` on the left by the rotation built from
/// two angles (each divided by the angle divisor `010a4d90` first): `this = R * this` with
/// `R = [cosA*cosB, cosA*sinB, -sinA; -sinB, cosB, 0; sinA*cosB, sinA*sinB, cosA]`.
/// The exe returns the address of its dead stack temporary; nothing is returned here.
pub fn c_rot_transform_rotate_yz(
    e: &mut Engine,
    this: Ptr<CRotTransform>,
    angle_a: f32,
    angle_b: f32,
) {
    e.with_stack(0x24, |e, rotation| {
        c_rot_transform_c_rot_transform(e, rotation.cast());
        let cos_a = scaled_trig(e, COS, angle_a);
        let sin_a = scaled_trig(e, SIN, angle_a);
        let cos_b = scaled_trig(e, COS, angle_b);
        let sin_b = scaled_trig(e, SIN, angle_b);
        let r = rotation.addr();
        let (ca, sa, cb, sb) = (
            f64::from(cos_a),
            f64::from(sin_a),
            f64::from(cos_b),
            f64::from(sin_b),
        );
        st(e, r, ca * cb);
        st(e, r + 4, ca * sb);
        e.mem.set_f32(r + 8, -sin_a);
        e.mem.set_f32(r + 12, -sin_b);
        e.mem.set_f32(r + 16, cos_b);
        e.mem.set_f32(r + 20, 0.0);
        st(e, r + 24, sa * cb);
        st(e, r + 28, sa * sb);
        e.mem.set_f32(r + 32, cos_a);
        e.with_stack(0x24, |e, product| {
            c_rot_transform_operator_p(e, rotation.cast(), product.cast(), this);
            copy_words(e, this.addr(), product.addr(), 9);
        });
    });
}

/// The rotation about the axis `axis` by `angle` (divided by the angle divisor first), as
/// `RotateAxis` and `RotateAxisFromIdentity` compute it: `sin` is called before `cos`;
/// `c` is `cos`, `s` is `sin` and `m` is `1 - c`, and the nine entries are returned in
/// matrix order.
fn axis_rotation(e: &mut Engine, angle: f32, axis: [f32; 3]) -> [f32; 9] {
    let s = scaled_trig(e, SIN, angle);
    let c = scaled_trig(e, COS, angle);
    let m = (1.0 - f64::from(c)) as f32;
    let (ax, ay, az) = (
        f64::from(x87_quiet(axis[0])),
        f64::from(x87_quiet(axis[1])),
        f64::from(x87_quiet(axis[2])),
    );
    let (c, s, m) = (f64::from(c), f64::from(s), f64::from(m));
    [
        (m * ax * ax + c) as f32,
        (m * ax * ay + s * az) as f32,
        (m * ax * az - s * ay) as f32,
        (m * ax * ay - s * az) as f32,
        (m * ay * ay + c) as f32,
        (m * ay * az + s * ax) as f32,
        (m * ax * az + s * ay) as f32,
        (m * ay * az - s * ax) as f32,
        (m * az * az + c) as f32,
    ]
}

// Translated from 00b12d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CRotTransform::RotateAxis` (Xbox PDB): multiplies `this` on the left by the rotation of
/// `angle` about the axis `(x, y, z)` (the axis-angle matrix, `this = R * this`).
/// The exe returns the address of its dead stack temporary; nothing is returned here.
pub fn c_rot_transform_rotate_axis(
    e: &mut Engine,
    this: Ptr<CRotTransform>,
    angle: f32,
    x: f32,
    y: f32,
    z: f32,
) {
    e.with_stack(0x24, |e, rotation| {
        c_rot_transform_c_rot_transform(e, rotation.cast());
        let values = axis_rotation(e, angle, [x, y, z]);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(rotation.addr() + 4 * i as u32, *value);
        }
        e.with_stack(0x24, |e, product| {
            c_rot_transform_operator_p(e, rotation.cast(), product.cast(), this);
            copy_words(e, this.addr(), product.addr(), 9);
        });
    });
}

// Translated from 00b12ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CRotTransform::RotateAxisFromIdentity` (Xbox PDB): sets `this` to the rotation of `angle`
/// about the axis `(x, y, z)`.
pub fn c_rot_transform_rotate_axis_from_identity(
    e: &mut Engine,
    this: Ptr<CRotTransform>,
    angle: f32,
    x: f32,
    y: f32,
    z: f32,
) {
    let values = axis_rotation(e, angle, [x, y, z]);
    for (i, value) in values.iter().enumerate() {
        e.mem.set_f32(this.addr() + 4 * i as u32, *value);
    }
}

// Translated from 00b13000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `VecInterpolate` (Xbox PDB), `cdecl`: `out = a + (b - a) * t`, through the `b - a` helper
/// (`00b13040`), the scale (`00b0cff0`) and the sum (`00b0cf90`); returns `out`.
pub fn vec_interpolate(
    e: &mut Engine,
    out: Ptr<CVec3>,
    a: Ptr<CVec3>,
    b: Ptr<CVec3>,
    t: f32,
) -> Ptr<CVec3> {
    e.with_stack(0x18, |e, frame| {
        let difference = frame;
        let scaled = frame.byte_add(0x0C);
        fn_00b13040(e, b, difference.cast(), a);
        e.call(VEC3_SCALE, &args![difference, scaled, x87_quiet(t)]);
        e.call(VEC3_ADD, &args![a, out, scaled]);
    });
    out
}

// Translated from 00b13040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CVec3` difference (no map name): `out = this - other`, component by component;
/// returns `out`.
pub fn fn_00b13040(
    e: &mut Engine,
    this: Ptr<CVec3>,
    out: Ptr<CVec3>,
    other: Ptr<CVec3>,
) -> Ptr<CVec3> {
    let (t, o) = (this.addr(), other.addr());
    let x = ld(e, t) - ld(e, o);
    let y = ld(e, t + 4) - ld(e, o + 4);
    let z = ld(e, t + 8) - ld(e, o + 8);
    st(e, out.addr(), x);
    st(e, out.addr() + 4, y);
    st(e, out.addr() + 8, z);
    out
}

// Translated from 00b130a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SIdvBranchVertex::SIdvBranchVertex` (Xbox PDB): zeroes the direction and position and sets
/// the transform to the identity (`00b12af0`); returns `this`.
pub fn s_idv_branch_vertex_s_idv_branch_vertex(
    e: &mut Engine,
    this: Ptr<SIdvBranchVertex>,
) -> Ptr<SIdvBranchVertex> {
    let base = this.addr();
    for offset in [8, 4, 0, 0x14, 0x10, 0x0C] {
        e.mem.set_f32(base + offset, 0.0);
    }
    c_rot_transform_c_rot_transform(e, Ptr::new(base + 0x1C));
    this
}

// Translated from 00b134d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CRotTransform::RotateY` (Xbox PDB): multiplies `this` on the left by the rotation about the
/// Y axis, `[c, 0, -s; 0, 1, 0; s, 0, c]` with `c` and `s` the cosine and sine of the angle
/// divided by the angle divisor; the exe returns the address of its dead stack temporary,
/// nothing is returned here.
pub fn c_rot_transform_rotate_y(e: &mut Engine, this: Ptr<CRotTransform>, angle: f32) {
    e.with_stack(0x24, |e, rotation| {
        c_rot_transform_c_rot_transform(e, rotation.cast());
        let cosine = scaled_trig(e, COS, angle);
        let sine = scaled_trig(e, SIN, angle);
        let r = rotation.addr();
        e.mem.set_f32(r, cosine);
        e.mem.set_f32(r + 4, 0.0);
        e.mem.set_f32(r + 8, -sine);
        e.mem.set_f32(r + 12, 0.0);
        e.mem.set_f32(r + 16, 1.0);
        e.mem.set_f32(r + 20, 0.0);
        e.mem.set_f32(r + 24, sine);
        e.mem.set_f32(r + 28, 0.0);
        e.mem.set_f32(r + 32, cosine);
        e.with_stack(0x24, |e, product| {
            c_rot_transform_operator_p(e, rotation.cast(), product.cast(), this);
            copy_words(e, this.addr(), product.addr(), 9);
        });
    });
}

/// `[LEAF_INFO]`: the static leaf-info pointer, read afresh each time as the exe does.
fn leaf_info(e: &Engine) -> u32 {
    e.global(LEAF_INFO)
}

/// `FISTP qword` with the truncating rounding control, low dword: what `(unsigned)value` reads
/// back (the "integer indefinite" `0x8000000000000000` has a zero low word).
fn truncate_low(value: f64) -> u32 {
    if value.is_nan()
        || value >= 9_223_372_036_854_775_808.0
        || value < -9_223_372_036_854_775_808.0
    {
        0
    } else {
        value.trunc() as i64 as u32
    }
}

/// `_ftol2` of a float held on the x87 stack (the value is passed as the `double` it is).
pub(crate) fn ftol(e: &mut Engine, value: f64) -> i32 {
    e.call(FTOL, &args![value]).i32()
}

// Translated from 00b13100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CIdvBranch::ComputeBud` (Xbox PDB): builds the two vertices of a bud. The first one (at
/// `position`, `matrix` rotated about the axis `matrix * axis` by a random angle, then about
/// Y by a fixed angle) points along the bud direction `BUD_AXIS * matrix`; the second lies a
/// length (the length spline evaluated at `lerp`, scaled by `size` and divided by the segment
/// count of the branch info `info_index`, at least a global minimum) along it. Then the leaf at
/// the second vertex is made (`MakeLeaf`, `00b13c10`) with the direction from the first vertex
/// and the direction from the parent's (or its ancestor's) first vertex.
pub fn c_idv_branch_compute_bud(
    e: &mut Engine,
    this: Ptr<CIdvBranch>,
    parent: Ptr<CIdvBranch>,
    size: f32,
    info_index: u32,
    position: Ptr<CVec3>,
    lerp: f32,
    matrix: Ptr<CRotTransform>,
    axis: Ptr<CVec3>,
    leaves: Ptr<StlVector>,
    wind_weight: f32,
    wind_group: i32,
) {
    e.with_stack(0x210, |e, frame| {
        let ebp = frame.addr() + 0x200;
        let at = |offset: i32| ebp.wrapping_add(offset as u32);
        let count = vector_len(e, BRANCH_INFO_LIST, 4);
        if info_index >= count {
            e.call(INVALID_PARAMETER, &args![]);
        }
        let info = e.mem.u32(
            info_index
                .wrapping_mul(4)
                .wrapping_add(e.mem.u32(BRANCH_INFO_LIST + 0x0C)),
        );
        let y_angle = x87_quiet(e.global::<f32>(0x0103_4294));
        // Two vertices, constructed in order.
        let first = at(-0xC0);
        let second = at(-0x78);
        s_idv_branch_vertex_s_idv_branch_vertex(e, Ptr::new(first));
        s_idv_branch_vertex_s_idv_branch_vertex(e, Ptr::new(second));
        let length_spline = e.mem.u32(info + 0x60);
        let spline_value = e
            .call(SPLINE_EVALUATE, &args![length_spline, x87_quiet(lerp)])
            .f32();
        let scaled_length = (f64::from(spline_value) * f64::from(size)) as f32;
        let minimum = x87_quiet(e.global::<f32>(0x0101_3ea4));
        let per_segment = (f64::from(scaled_length) / f64::from(e.mem.i32(info + 4))) as f32;
        let bud_length = if f64::from(minimum) > f64::from(per_segment) {
            minimum
        } else {
            per_segment
        };
        let bud_length = x87_quiet(bud_length);
        let low = x87_quiet(e.global::<f32>(0x010a_54a0));
        let high = x87_quiet(e.global::<f32>(0x0106_b158));
        let angle = e
            .call(GET_UNIFORM, &args![RANDOM_GENERATOR, low, high])
            .f32();
        c_rot_transform_operator_p_ov2(e, matrix, Ptr::new(at(-0x10)), axis);
        // The first vertex: position, then the transform.
        copy_words(e, first + 0x0C, position.addr(), 3);
        copy_words(e, first + 0x1C, matrix.addr(), 9);
        let (ax, ay, az) = (
            e.mem.f32(at(-0x10)),
            e.mem.f32(at(-0x0C)),
            e.mem.f32(at(-0x08)),
        );
        c_rot_transform_rotate_axis(e, Ptr::new(first + 0x1C), angle, ax, ay, az);
        c_rot_transform_rotate_y(e, Ptr::new(first + 0x1C), y_angle);
        c_vec3_operator_p_ov2(
            e,
            Ptr::new(BUD_AXIS),
            Ptr::new(at(-0x100)),
            Ptr::new(first + 0x1C),
        );
        copy_words(e, first, at(-0x100), 3);
        // The second vertex lies `bud_length` along the direction.
        let scaled = e
            .call(VEC3_SCALE, &args![first, at(-0x10C), bud_length])
            .u32();
        let sum = e
            .call(VEC3_ADD, &args![first + 0x0C, at(-0x118), scaled])
            .u32();
        copy_words(e, second + 0x0C, sum, 3);
        fn_00b13040(
            e,
            Ptr::new(second + 0x0C),
            Ptr::new(at(-0x20)),
            Ptr::new(first + 0x0C),
        );
        c_vec3_normalize(e, Ptr::new(at(-0x20)));
        let wide = at(-0xD0);
        for offset in [8, 4, 0] {
            e.mem.set_f32(wide + offset, 0.0);
        }
        if parent.is_null() || e.get(parent, CIdvBranch::m_nVertexCount) == 0 {
            copy_words(e, wide, at(-0x20), 3);
        } else {
            let vertices = e.get(parent, CIdvBranch::m_pVertices).addr();
            copy_words(e, at(-0xE4), vertices + 0x0C, 3);
            if e.mem.u32(leaf_info(e) + 0x34) != 0 {
                let mut depth = 0i32;
                let mut ancestor = parent.addr();
                while ancestor != 0 && depth < e.mem.i32(leaf_info(e) + 0x34) {
                    ancestor = e.mem.u32(ancestor);
                    depth += 1;
                }
                if ancestor != 0 {
                    let vertices = e.mem.u32(ancestor + 0x20);
                    copy_words(e, at(-0xE4), vertices + 0x0C, 3);
                }
            }
            let difference = fn_00b13040(
                e,
                Ptr::new(second + 0x0C),
                Ptr::new(at(-0x124)),
                Ptr::new(at(-0xE4)),
            );
            copy_words(e, wide, difference.addr(), 3);
        }
        c_vec3_normalize(e, Ptr::new(wide));
        c_idv_branch_make_leaf(
            e,
            this,
            Ptr::new(second + 0x0C),
            lerp,
            this,
            Ptr::new(at(-0x20)),
            Ptr::new(wide),
            wind_weight,
            wind_group,
            leaves,
        );
    });
}

// Translated from 00b135a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the triangle-strip indices of a branch's cross-section rings (the engine map gives
/// it no name). With fewer than two cross-section segments nothing happens. Otherwise a
/// buffer of `(vertexCount - 1) * (2 * segments + 4)` 16-bit indices is allocated and
/// handed to the geometry's strip registration (`00b2fdc0`, with `ring_length`), filled
/// ring by ring from the running position (a float stepped by `segments / segments`, which
/// is 1) and the strip counter (`+0x26`) of the geometry is incremented.
pub fn fn_00b135a0(e: &mut Engine, this: Ptr<CIdvBranch>, ring_length: u16, geometry: Ptr) {
    let segments = u32::from(e.get(this, CIdvBranch::m_usCrossSectionSegments));
    if (segments as i32) < 2 {
        return;
    }
    let n = segments as i32;
    let ratio = if n == 0 {
        1.0
    } else {
        f64::from(segments as i32) / f64::from(n)
    };
    let step = ratio as f32;
    let rows = e.get(this, CIdvBranch::m_nVertexCount);
    let count = rows
        .wrapping_sub(1)
        .wrapping_mul(n.wrapping_mul(2).wrapping_add(4));
    let bytes = (count as u32).checked_mul(2).unwrap_or(u32::MAX);
    let buffer = e.call(OPERATOR_NEW, &args![bytes]).u32();
    e.call(
        0x00b2_fdc0,
        &args![
            geometry,
            u32::from(ring_length),
            buffer,
            count as u32 & 0xFFFF
        ],
    );
    let offset = e.get(this, CIdvBranch::m_nStartingOffset);
    let base = offset.wrapping_add(segments as i32).wrapping_add(1);
    let mut written = 0u32;
    let mut put = |e: &mut Engine, value: i32| {
        e.mem
            .set_u16(buffer.wrapping_add(written * 2), value as u16);
        written += 1;
    };
    let mut position = 0.0f32;
    let mut ring = 0i32;
    while ring < rows.wrapping_sub(1) {
        let t = ftol(e, f64::from(position));
        let value = base.wrapping_add(ftol(e, f64::from(position)));
        put(e, value);
        let value = ftol(e, f64::from(position)).wrapping_add(offset);
        put(e, value);
        position = (f64::from(position) + f64::from(step)) as f32;
        let mut segment = 1i32;
        while segment < n {
            let value = base.wrapping_add(ftol(e, f64::from(position)));
            put(e, value);
            let value = ftol(e, f64::from(position)).wrapping_add(offset);
            put(e, value);
            position = (f64::from(position) + f64::from(step)) as f32;
            segment += 1;
        }
        put(e, base.wrapping_add(t).wrapping_add(segments as i32));
        put(e, offset.wrapping_add(t).wrapping_add(segments as i32));
        position = t.wrapping_add(segments as i32).wrapping_add(1) as f32;
        put(e, base.wrapping_add(t));
        put(e, base.wrapping_add(t));
        ring += 1;
    }
    let strips = e.mem.u16(geometry.addr() + 0x26);
    e.mem
        .set_u16(geometry.addr() + 0x26, strips.wrapping_add(1));
}

// Translated from 00b13820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Appends `this` and then, recursively, every descendant branch (in child order) to the
/// vector of ints at `list` (the engine map gives it no name).
pub fn fn_00b13820(e: &mut Engine, this: Ptr<CIdvBranch>, list: Ptr<StlVector>) {
    push_value(e, list, this.addr());
    let children = this.addr() + 8;
    let mut index = 0u32;
    while index < vector_len(e, children, 0x0C) {
        let entry = vector_at(e, children, index, 0x0C);
        let child = e.mem.u32(entry + 8);
        fn_00b13820(e, Ptr::new(child), list);
        index += 1;
    }
}

// Translated from 00b138c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A checked-iterator call on the whole range of the `std::vector` of ints at `vector`
/// (`cdecl`, the engine map gives it no name): builds the (proxy, first) and (proxy, last)
/// iterators, taking the invalid-parameter path if the vector's pointers are not ordered, and
/// passes them with a zero flag to `00b16950`. Returns what that leaves in `EAX`.
pub fn fn_00b138c0(e: &mut Engine, vector: Ptr) -> u32 {
    let base = vector.addr();
    let last = e.mem.u32(base + 0x10);
    let last_proxy = checked_iterator(e, base, last);
    let first = e.mem.u32(base + 0x0C);
    let first_proxy = checked_iterator(e, base, first);
    fn_00b16950(e, first_proxy, first, last_proxy, last, 0)
}

// Translated from 00b13970 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b13970 - the FS:[0] exception frame (freeing the new spline's
// memory if its construction throws) is not translated.
/// `CTreeFileAccess::ParseBranchParameter` (Xbox PDB): allocates a 0x84-byte spline, parses the
/// next string of the file (`00b3ee40`) and constructs the spline from it (`00b37660`); the
/// temporary string is then tidied (`0044a730`). Returns the spline, null when the
/// allocation failed.
pub fn c_tree_file_access_parse_branch_parameter(e: &mut Engine, this: Ptr) -> Ptr {
    let memory = e.call(OPERATOR_NEW, &args![0x84u32]).u32();
    if memory == 0 {
        return Ptr::NULL;
    }
    e.with_stack(0x1C, |e, text| {
        let parsed = e.call(PARSE_STRING, &args![this, text]).u32();
        let spline = e.call(SPLINE_CONSTRUCT, &args![memory, parsed]).u32();
        e.call(STRING_TIDY, &args![text, 1u32, 0u32]);
        Ptr::new(spline)
    })
}

// Translated from 00b13a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CIdvBranch::BuildBlossomVectors` (Xbox PDB): empties the two static index vectors and, for
/// every leaf texture (0x54-byte entries of the leaf info), appends the two indices
/// `2 * i` and `2 * i + 1` to the blossom vector when the texture's first byte is set and to
/// the other vector otherwise.
pub fn c_idv_branch_build_blossom_vectors(e: &mut Engine, _this: Ptr<CIdvBranch>) {
    std_vector_int_clear(e, Ptr::new(BLOSSOM_INDICES));
    std_vector_int_clear(e, Ptr::new(NON_BLOSSOM_INDICES));
    let mut index = 0u32;
    while index < vector_len(e, leaf_info(e) + 0x10, 0x54) {
        let texture = vector_at(e, leaf_info(e) + 0x10, index, 0x54);
        let target = if e.mem.u8(texture) != 0 {
            BLOSSOM_INDICES
        } else {
            NON_BLOSSOM_INDICES
        };
        push_value(e, Ptr::new(target), index << 1);
        push_value(e, Ptr::new(target), (index << 1).wrapping_add(1));
        index += 1;
    }
}

// Translated from 00b13b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CIdvBranch::IsBlossom` (Xbox PDB): whether a leaf at `node` (`percent` along it) is a
/// blossom. When the leaf info's blossom level (`+0x34`) is zero, `percent` is replaced by the
/// node's own percent of parent (the walk up the parents before that never runs, since it
/// needs a level above 1). If the blossom distance (`+0x2C`) is below that percent, a uniform
/// random number in [0, 1] is drawn and the leaf is a blossom when the blossom weighting
/// (`+0x30`) is not below it. `this` is not read.
pub fn c_idv_branch_is_blossom(
    e: &mut Engine,
    _this: Ptr<CIdvBranch>,
    node: Ptr<CIdvBranch>,
    percent: f32,
) -> bool {
    let mut factor = x87_quiet(percent);
    if e.mem.u32(leaf_info(e) + 0x34) == 0 {
        let mut level = 1i32;
        let mut cursor = node.addr();
        // Unreachable beyond its first test: `level` starts at 1 and the loop needs it below
        // the blossom level, which is zero here.
        while cursor != 0 && e.mem.u32(cursor) != 0 && level < e.mem.i32(leaf_info(e) + 0x34) {
            cursor = e.mem.u32(cursor);
            level += 1;
        }
        if cursor != 0 {
            factor = x87_quiet(e.mem.f32(cursor + 4));
        }
    }
    let info = leaf_info(e);
    if ld(e, info + 0x2C) < f64::from(factor) {
        let random = e
            .call(GET_UNIFORM, &args![RANDOM_GENERATOR, 0.0f32, 1.0f32])
            .f64();
        if ld(e, leaf_info(e) + 0x30) >= random {
            return true;
        }
    }
    false
}

// Translated from 00b14230 (decompiled, FalloutNV.exe 1.4.0.525)
/// A fast approximate length of the `CVec3` `this` (the engine map gives it no name): the sum
/// of squares `z*z + (y*y + x*x)` is stored as a float, whose bit pattern is shifted right
/// once (arithmetic) and offset by `0x1FC00000`, and read back as the float result.
pub fn fn_00b14230(e: &mut Engine, this: Ptr<CVec3>) -> f32 {
    let base = this.addr();
    let (x, y, z) = (e.mem.u32(base), e.mem.u32(base + 4), e.mem.u32(base + 8));
    // FLD z; FMUL z; FLD y; FMUL y; FLD x; FMUL x; FADDP; FADDP: each step with the
    // x87 choice of NaN (two NaNs with equal significands give the positive one).
    let zz = x87_mul(x87_load32(z), x87_m32(z));
    let yy = x87_mul(x87_load32(y), x87_m32(y));
    let xx = x87_mul(x87_load32(x), x87_m32(x));
    let sum = x87_add(zz, x87_add(yy, xx));
    // FSTP dword, then SAR 1 and ADD 0x1FC00000 on the stored bits.
    let bits = ((x87_store32(sum) as i32) >> 1).wrapping_add(0x1FC0_0000) as u32;
    x87_quiet(f32::from_bits(bits))
}

// Translated from 00b14280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a leaf at `point` keeps clear of the leaves in `list` (a vector of leaf pointers):
/// false when the leaf-info has no textures; otherwise the larger of the texture's two floats
/// (`+0x48`, `+0x4C`) times the spacing tolerance (`+0x28`) is the half-width of a box, and
/// the result is false as soon as the point lies strictly inside the box around some leaf's
/// position (`leaf + 4`).
pub fn fn_00b14280(
    e: &mut Engine,
    _this: Ptr<CIdvBranch>,
    point: Ptr<CVec3>,
    texture_index: u32,
    list: Ptr<StlVector>,
) -> bool {
    let mut clear = true;
    if vector_len(e, leaf_info(e) + 0x10, 0x54) == 0 {
        return false;
    }
    let texture = vector_at(e, leaf_info(e) + 0x10, texture_index, 0x54);
    let first = ld(e, texture + 0x48);
    let second = ld(e, texture + 0x4C);
    let larger = if second < first { first } else { second };
    let larger = f64::from(larger as f32);
    let half_width = (larger * ld(e, leaf_info(e) + 0x28)) as f32;
    let half_width = f64::from(half_width);
    let mut index = 0u32;
    while index < vector_len(e, list.addr(), 4) && clear {
        let entry = vector_at(e, list.addr(), index, 4);
        let other = e.mem.u32(entry) + 4;
        let p = point.addr();
        let inside = |e: &Engine, axis: u32| {
            ld(e, p + axis) < ld(e, other + axis) + half_width
                && ld(e, p + axis) > ld(e, other + axis) - half_width
        };
        if inside(e, 0) && inside(e, 4) && inside(e, 8) {
            clear = false;
        }
        index += 1;
    }
    clear
}

layout! {
    /// `SIdvBranchInfo` (Xbox PDB), 0x74 bytes; the PDB offsets hold on the PC.
    pub struct SIdvBranchInfo: 0x74 {
        0x00 m_nCrossSectionSegments: i32,
        0x04 m_nSegments: i32,
        0x08 m_fFirstBranch: f32,
        0x0C m_fLastBranch: f32,
        0x10 m_fFrequency: f32,
        0x14 m_fSTile: f32,
        0x18 m_fTTile: f32,
        0x1C m_bSTileAbsolute: u8,
        0x1D m_bTTileAbsolute: u8,
        0x1E m_bRandomTCoordOffset: u8,
        0x20 m_fTwist: f32,
        0x24 m_fSegmentPackingExponent: f32,
        0x28 m_nNumFlares: i32,
        0x2C m_fFlareBalance: f32,
        0x30 m_fRadialInfluence: f32,
        0x34 m_fRadialInfluenceVariance: f32,
        0x38 m_fRadialExponent: f32,
        0x3C m_fRadialDistance: f32,
        0x40 m_fRadialVariance: f32,
        0x44 m_fLengthDistance: f32,
        0x48 m_fLengthVariance: f32,
        0x4C m_fLengthExponent: f32,
        0x50 m_pDisturbance: Ptr,
        0x54 m_pGravity: Ptr,
        0x58 m_pFlexibility: Ptr,
        0x5C m_pFlexibilityScale: Ptr,
        0x60 m_pLength: Ptr,
        0x64 m_pRadius: Ptr,
        0x68 m_pRadiusScale: Ptr,
        0x6C m_pStartAngle: Ptr,
        0x70 m_pAngleProfile: Ptr,
    }
}

/// The checked-iterator construction of the `std::vector` code: an iterator at `position` of the
/// vector at `vector` is valid when the vector exists and `first <= position <= last`
/// (otherwise the invalid-parameter path is taken, `00ec7c56`); returns the vector's proxy
/// (`+0`), which the iterator stores.
pub(crate) fn checked_iterator(e: &mut Engine, vector: u32, position: u32) -> u32 {
    if vector == 0 || e.mem.u32(vector + 0x0C) > position || position > e.mem.u32(vector + 0x10) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem.u32(vector)
}

/// Appends a word to the `std::vector` of ints at `vector` through a stack slot (the exe passes
/// `push_back` the address of a local holding the value).
fn push_value(e: &mut Engine, vector: Ptr<StlVector>, value: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        std_vector_int_push_back(e, vector, slot);
    });
}

// Translated from 00b13c10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b13c10 - the FS:[0] exception frame (freeing the leaf memory if
// its construction throws) is not translated.
/// `CIdvBranch::MakeLeaf` (Xbox PDB): picks a leaf texture index (a random entry of the
/// blossom index vector when `IsBlossom` says so, else of the other one), checks the collision
/// rule of the leaf info (`+0xC`: 1 = against the static local leaves, 2 = against `leaves`)
/// with `00b14280`, and builds a billboard leaf (`00b40070`, 0x4C bytes) at `position` whose
/// colour scale comes from the (optionally dimmed) percent, appends it to `leaves` (and to the
/// local leaves for rule 1), derives its normal, tangent and binormal from `start` and
/// `direction`, and gives it a random colour offset (`SetColor`, `00b404c0`).
pub fn c_idv_branch_make_leaf(
    e: &mut Engine,
    this: Ptr<CIdvBranch>,
    position: Ptr<CVec3>,
    percent: f32,
    node: Ptr<CIdvBranch>,
    start: Ptr<CVec3>,
    direction: Ptr<CVec3>,
    wind_weight: f32,
    wind_group: i32,
    leaves: Ptr<StlVector>,
) {
    if vector_len(e, leaf_info(e) + 0x10, 0x54) == 0 {
        return;
    }
    let is_blossom =
        vector_len(e, BLOSSOM_INDICES, 4) != 0 && c_idv_branch_is_blossom(e, this, node, percent);
    let chosen = if is_blossom {
        let mut index = 0u32;
        let count = vector_len(e, BLOSSOM_INDICES, 4);
        if count > 1 {
            let upper = x87_quiet(e.global::<f32>(0x0101_7130));
            let random = e
                .call(GET_UNIFORM, &args![RANDOM_GENERATOR, 0.0f32, upper])
                .f64();
            index = truncate_low(random) % count;
        }
        let entry = vector_at(e, BLOSSOM_INDICES, index, 4);
        e.mem.i32(entry)
    } else {
        let count = vector_len(e, NON_BLOSSOM_INDICES, 4);
        if count > 1 {
            let upper = x87_quiet(e.global::<f32>(0x0108_cb00));
            let random = e
                .call(GET_UNIFORM, &args![RANDOM_GENERATOR, 0.0f32, upper])
                .f32();
            let index = truncate_low(f64::from(random)) % vector_len(e, NON_BLOSSOM_INDICES, 4);
            let entry = vector_at(e, NON_BLOSSOM_INDICES, index, 4);
            e.mem.i32(entry)
        } else {
            if count == 0 {
                return;
            }
            // Exactly one entry here, so this emptiness check never fires.
            if vector_len(e, NON_BLOSSOM_INDICES, 4) == 0 {
                e.call(INVALID_PARAMETER, &args![]);
            }
            let first = e.mem.u32(NON_BLOSSOM_INDICES + 0x0C);
            e.mem.i32(first)
        }
    };
    let texture_index = (chosen / 2) as u32;
    let kept = match e.mem.u32(leaf_info(e) + 0x0C) {
        1 => fn_00b14280(e, this, position, texture_index, Ptr::new(LOCAL_LEAVES)),
        2 => fn_00b14280(e, this, position, texture_index, leaves),
        _ => true,
    };
    if !kept {
        return;
    }
    let scale = if e.mem.u8(leaf_info(e)) != 0 {
        let parent_percent = fn_00b15110(e, node);
        let blended = ((1.0 - f64::from(parent_percent)) * f64::from(percent)
            + f64::from(parent_percent)) as f32;
        let blended = f64::from(blended);
        let cubed = f64::from((blended * blended * blended) as f32);
        let remaining = f64::from((1.0 - ld(e, leaf_info(e) + 4)) as f32);
        (remaining * (1.0 - cubed) + cubed) as f32
    } else {
        1.0
    };
    let memory = e.call(OPERATOR_NEW, &args![0x4Cu32]).u32();
    let leaf = if memory != 0 {
        let upper = x87_quiet(e.global::<f32>(0x0102_2958));
        let random = e
            .call(GET_UNIFORM, &args![RANDOM_GENERATOR, 0.0f32, upper])
            .f64();
        let random_index = ftol(e, random);
        let textures = e.mem.i32(leaf_info(e) + 0x40);
        let remainder = random_index % textures;
        let scaled = f64::from(scale) * e.global::<f64>(0x0101_e568);
        let colour_scale = ftol(e, scaled) as u16;
        e.call(
            BILLBOARD_LEAF_CONSTRUCT,
            &args![
                memory,
                position,
                u32::from(colour_scale),
                remainder,
                x87_quiet(wind_weight),
                wind_group
            ],
        )
        .u32()
    } else {
        0
    };
    push_value(e, leaves, leaf);
    if e.mem.u32(leaf_info(e) + 0x0C) == 1 {
        push_value(e, Ptr::new(LOCAL_LEAVES), leaf);
    }
    let texture = vector_at(e, leaf_info(e) + 0x10, texture_index, 0x54);
    e.mem.set_u8(leaf + 0x40, chosen as u8);
    e.with_stack(0x60, |e, frame| {
        let normal = frame.addr();
        let tangent = frame.addr() + 0x0C;
        let binormal = frame.addr() + 0x18;
        let difference = frame.addr() + 0x24;
        let scaled = frame.addr() + 0x30;
        let sum = frame.addr() + 0x3C;
        let colour = frame.addr() + 0x48;
        for offset in [8, 4, 0] {
            e.mem.set_f32(normal + offset, 0.0);
        }
        let length = fn_00b14230(e, start);
        if f64::from(length) < e.global::<f64>(0x0103_0f28) {
            copy_words(e, normal, direction.addr(), 3);
        } else {
            let radius = x87_quiet(e.mem.f32(texture + 0x10));
            let offset = fn_00b13040(e, start, Ptr::new(difference), direction);
            let scaled_offset = e.call(VEC3_SCALE, &args![offset, scaled, radius]).u32();
            e.call(VEC3_ADD, &args![direction, sum, scaled_offset]);
            copy_words(e, normal, sum, 3);
        }
        c_vec3_normalize(e, Ptr::new(normal));
        copy_words(e, leaf + 0x1C, normal, 3);
        copy_words(e, tangent, normal, 3);
        copy_words(e, leaf + 0x28, tangent, 3);
        c_vec3_operator_p(e, Ptr::new(tangent), Ptr::new(binormal), Ptr::new(normal));
        c_vec3_normalize(e, Ptr::new(binormal));
        copy_words(e, leaf + 0x34, binormal, 3);
        let high = x87_quiet(e.mem.f32(texture + 0x10));
        let low = -x87_quiet(e.mem.f32(texture + 0x10));
        let jitter = e
            .call(GET_UNIFORM, &args![RANDOM_GENERATOR, low, high])
            .f32();
        for k in 0..3 {
            let channel = f64::from(jitter) + ld(e, texture + 4 + 4 * k);
            st(e, colour + 4 * k, channel);
        }
        e.call(BILLBOARD_LEAF_SET_COLOR, &args![leaf, colour, 1u32]);
    });
}

// Translated from 00b14450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Emits the ring of vertices of one branch segment into `geometry` (the engine map gives it no
/// name): for each of `segments + 1` steps around the ring (an angle of the global period
/// times a running fraction) it adds the texture coordinate from `profile`
/// (`00b301f0`), the ring direction built from the vertex's transform rows
/// (`00b305f0`, the cross with the direction rotated by the ring offset `011f8c30` through
/// `00b30660`), the position (the direction times the vertex radius, scaled by the sum of the
/// flares' `00b14980` values) with `00b300c0`, the colour triple (the leaf info's shade, from
/// `percent`/`parent_percent`, or one) with `00b30020`, and, when the geometry's weighting flag
/// (`+8`) is set, the wind weight `texture_coord`/`wind_flag` with `00b30560`, then counts the
/// vertex (`+0x22`).
pub fn fn_00b14450(
    e: &mut Engine,
    this: Ptr<CIdvBranch>,
    vertex: Ptr<SIdvBranchVertex>,
    percent: f32,
    parent_percent: f32,
    segments: i32,
    geometry: Ptr,
    profile: Ptr,
    texture_coord: f32,
    wind_flag: u8,
    offset: f32,
    squared: i32,
) {
    let vertex = vertex.addr();
    let profile = profile.addr();
    let mut position = 0.0f32;
    let increment = (1.0 / f64::from(segments)) as f32;
    let shade = if e.global::<u32>(LEAF_INFO) == 0 {
        1.0f32
    } else {
        let blended = ((1.0 - f64::from(parent_percent)) * f64::from(percent)
            + f64::from(parent_percent)) as f32;
        let level = if squared == 0 {
            (f64::from(blended) * f64::from(blended)) as f32
        } else {
            blended
        };
        let strength = ld(e, leaf_info(e) + 8);
        (f64::from(level) * strength + (1.0 - strength)) as f32
    };
    e.with_stack(0x60, |e, frame| {
        let texture = frame.addr();
        let ring = frame.addr() + 0x08;
        let rotated = frame.addr() + 0x14;
        let normal = frame.addr() + 0x20;
        let point = frame.addr() + 0x2C;
        let moved = frame.addr() + 0x38;
        let colour = frame.addr() + 0x44;
        for k in 0..3 {
            e.mem.set_f32(colour + 4 * k, x87_quiet(shade));
        }
        let mut step = 0i32;
        while step <= segments {
            let angle = (f64::from(e.global::<f32>(ANGLE_PERIOD)) * f64::from(position)) as f32;
            let first = (f64::from(position) * ld(e, profile)) as f32;
            let first = (ld(e, profile + 8) * f64::from(percent) + f64::from(first)) as f32;
            e.mem.set_f32(texture, first);
            let second = ((f64::from(percent) + f64::from(offset)) * ld(e, profile + 4)) as f32;
            e.mem.set_f32(texture + 4, second);
            e.call(0x00b3_01f0, &args![geometry, texture, 0xFFFF_FFFFu32]);
            let cosine = ring_trig(e, COS, angle);
            let sine = ring_trig(e, SIN, angle);
            let axis = ring_basis(e, vertex, cosine, sine);
            for (k, value) in axis.iter().enumerate() {
                e.mem.set_f32(ring + 4 * k as u32, *value);
            }
            let shifted = (ld(e, RING_ANGLE_OFFSET) + f64::from(angle)) as f32;
            let cosine = ring_trig(e, COS, shifted);
            let sine = ring_trig(e, SIN, shifted);
            let axis = ring_basis(e, vertex, cosine, sine);
            for (k, value) in axis.iter().enumerate() {
                e.mem.set_f32(rotated + 4 * k as u32, *value);
            }
            e.call(0x00b3_05f0, &args![geometry, rotated]);
            let crossed = c_vec3_operator_p(e, Ptr::new(rotated), Ptr::new(normal), Ptr::new(ring));
            e.call(0x00b3_0660, &args![geometry, crossed]);
            let mut sum = 0.0f32;
            let mut index = 0u32;
            let flares = this.addr() + 0x38;
            while index < vector_len(e, flares, 0x18) {
                let flare = vector_at(e, flares, index, 0x18);
                let value = fn_00b14980(e, Ptr::new(flare), angle, percent);
                sum = (f64::from(value) + f64::from(sum)) as f32;
                index += 1;
            }
            let factor = (1.0 + f64::from(sum)) as f32;
            let radius = ld(e, vertex + 0x18);
            for k in 0..3 {
                let value = ld(e, ring + 4 * k) * radius + ld(e, vertex + 0x0C + 4 * k);
                st(e, point + 4 * k, value);
            }
            if f64::from(factor) != e.global::<f64>(0x0101_2070) {
                for k in 0..3 {
                    let value = ld(e, ring + 4 * k) * radius * f64::from(factor)
                        + ld(e, vertex + 0x0C + 4 * k);
                    st(e, moved + 4 * k, value);
                }
                for k in 0..3 {
                    let delta = ld(e, moved + 4 * k) - ld(e, point + 4 * k);
                    st(e, normal + 4 * k, delta);
                }
                c_vec3_normalize(e, Ptr::new(normal));
                copy_words(e, ring, normal, 3);
                e.call(MEMCPY, &args![point, moved, 0x0Cu32]);
            }
            e.call(0x00b3_00c0, &args![geometry, point]);
            e.call(0x00b3_0020, &args![geometry, colour]);
            if e.mem.u8(geometry.addr() + 8) != 0 {
                e.call(
                    0x00b3_0560,
                    &args![geometry, x87_quiet(texture_coord), u32::from(wind_flag)],
                );
            }
            let counter = e.mem.u16(geometry.addr() + 0x22);
            e.mem
                .set_u16(geometry.addr() + 0x22, counter.wrapping_add(1));
            position = (f64::from(position) + f64::from(increment)) as f32;
            step += 1;
        }
    });
}

/// `cos` or `sin` of a float angle, as the ring code calls them: the angle passed as a
/// `double`, the result stored as a float.
fn ring_trig(e: &mut Engine, function: u32, angle: f32) -> f32 {
    x87_quiet(e.call(function, &args![f64::from(angle)]).f32())
}

/// `cos * row2 + sin * row3` of the vertex transform (`vertex + 0x28` and `vertex + 0x34`), per
/// component, each rounded to a float.
fn ring_basis(e: &Engine, vertex: u32, cosine: f32, sine: f32) -> [f32; 3] {
    let mut out = [0.0f32; 3];
    for (k, slot) in out.iter_mut().enumerate() {
        let k = k as u32;
        *slot = (f64::from(cosine) * ld(e, vertex + 0x28 + 4 * k)
            + f64::from(sine) * ld(e, vertex + 0x34 + 4 * k)) as f32;
    }
    out
}

// Translated from 00b14980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The influence of a flare (an `SIdvBranchFlare`, `this`) at a ring `angle` and a fraction
/// `param` along the segment (the engine map gives it no name): zero unless the angular
/// distance to the flare's angle (taking the shorter way round the period `011f8c14`, the
/// distance first compared with 3.1415927 at `010a5494`) is below the radial influence (`+4`) and
/// `distance - param` (`+0xC` minus `param`) is above the constant at `01012060`;
/// then `influence * pow(1 - angular / radial, radialExponent) *
/// pow((distance - param) / distance, lengthExponent)`.
pub fn fn_00b14980(e: &mut Engine, this: Ptr<SIdvBranchFlare>, angle: f32, param: f32) -> f32 {
    let base = this.addr();
    let mut result = 0.0f32;
    let mut flare_angle = x87_quiet(e.mem.f32(base));
    let mut angle = angle;
    let difference = (f64::from(angle) - f64::from(flare_angle)) as f32;
    let distance = x87_quiet(e.call(ABSOLUTE_VALUE, &args![f64::from(difference)]).f32());
    let limit = ld(e, 0x010a_5494);
    if limit < f64::from(distance) {
        let period = ld(e, ANGLE_PERIOD);
        if f64::from(flare_angle) > f64::from(angle) {
            angle = (f64::from(angle) + period) as f32;
        } else {
            flare_angle = (f64::from(flare_angle) + period) as f32;
        }
    }
    let difference = (f64::from(angle) - f64::from(flare_angle)) as f32;
    let distance = x87_quiet(e.call(ABSOLUTE_VALUE, &args![f64::from(difference)]).f32());
    let radial = ld(e, base + 4);
    if f64::from(distance) < radial {
        let remaining = (ld(e, base + 0x0C) - f64::from(param)) as f32;
        if f64::from(remaining) > e.global::<f64>(0x0101_2060) {
            let ratio = (1.0 - f64::from(distance) / radial) as f32;
            let exponent = x87_quiet(e.mem.f32(base + 8));
            let radial_power = e
                .call(POW, &args![f64::from(ratio), f64::from(exponent)])
                .f32();
            let along = (f64::from(remaining) / ld(e, base + 0x0C)) as f32;
            let exponent = x87_quiet(e.mem.f32(base + 0x10));
            let length_power = e
                .call(POW, &args![f64::from(along), f64::from(exponent)])
                .f32();
            result =
                (ld(e, base + 0x14) * f64::from(radial_power) * f64::from(length_power)) as f32;
        }
    }
    x87_quiet(result)
}

// Translated from 00b14ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// One, or `1 - a * b` when `marker` equals the global word at `011ad14c` (the engine map gives
/// it no name; `this` is not read).
pub fn fn_00b14ae0(e: &mut Engine, _this: Ptr, marker: i32, a: f32, b: f32) -> f32 {
    let mut result = 1.0f32;
    if marker == e.global::<i32>(LEVEL_MARKER) {
        result = (1.0 - f64::from(a) * f64::from(b)) as f32;
    }
    x87_quiet(result)
}

// Translated from 00b14b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `(0 - a) * b + a` (the engine map gives it no name; `this` is not read).
pub fn fn_00b14b10(_e: &mut Engine, _this: Ptr, a: f32, b: f32) -> f32 {
    x87_quiet(((0.0 - f64::from(a)) * f64::from(b) + f64::from(a)) as f32)
}

// Translated from 00b14b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CIdvBranch::ComputeVolume` (Xbox PDB): with at least two vertices, the volume (`+0x30`) is
/// the sum over consecutive vertex pairs of `(radius0 + radius1) * distance` (`CVec3::Distance`,
/// `00b04d40`, between the positions).
pub fn c_idv_branch_compute_volume(e: &mut Engine, this: Ptr<CIdvBranch>) {
    let base = this.addr();
    if e.mem.u32(base + 0x20) == 0 || e.mem.i32(base + 0x24) <= 1 {
        return;
    }
    e.mem.set_f32(base + 0x30, 0.0);
    let mut index = 0i32;
    while index < e.mem.i32(base + 0x24) - 1 {
        let first = (index as u32)
            .wrapping_mul(0x48)
            .wrapping_add(e.mem.u32(base + 0x20));
        let second = (index as u32)
            .wrapping_mul(0x48)
            .wrapping_add(e.mem.u32(base + 0x20))
            .wrapping_add(0x48);
        let distance = e
            .call(VEC3_DISTANCE, &args![first + 0x0C, second + 0x0C])
            .f32();
        let total =
            (ld(e, first + 0x18) + ld(e, second + 0x18)) * f64::from(distance) + ld(e, base + 0x30);
        st(e, base + 0x30, total);
        index += 1;
    }
}

// Translated from 00b14be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CIdvBranch::FillBranch` (Xbox PDB): with at least two vertices, finds the segment holding
/// the running length `length` (the first vertex whose running length, `+0x40`, is above it
/// ends the search at the vertex before it; the index stays 0 when none is) and writes the
/// vertex index to `out` and the fraction of the way through that segment to `out + 4`.
pub fn c_idv_branch_fill_branch(e: &mut Engine, this: Ptr<CIdvBranch>, out: Ptr, length: f32) {
    let base = this.addr();
    if e.mem.u32(base + 0x20) == 0 || e.mem.i32(base + 0x24) < 2 {
        return;
    }
    e.mem.set_i32(out.addr(), 0);
    let mut index = 1i32;
    while index < e.mem.i32(base + 0x24) {
        let vertex = (index as u32)
            .wrapping_mul(0x48)
            .wrapping_add(e.mem.u32(base + 0x20));
        if f64::from(length) < ld(e, vertex + 0x40) {
            e.mem.set_i32(out.addr(), index - 1);
            break;
        }
        index += 1;
    }
    let found = e.mem.i32(out.addr());
    let lower = (found as u32)
        .wrapping_mul(0x48)
        .wrapping_add(e.mem.u32(base + 0x20));
    let upper = (found.wrapping_add(1) as u32)
        .wrapping_mul(0x48)
        .wrapping_add(e.mem.u32(base + 0x20));
    let along = f64::from(length) - ld(e, lower + 0x40);
    let span = ld(e, upper + 0x40) - ld(e, lower + 0x40);
    st(e, out.addr() + 4, along / span);
}

// Translated from 00b14ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the flares of a branch from its branch info `info` (the engine map gives it no
/// name). With a nonzero flare count (`+0x28`), starting from a random angle in [0, period)
/// it pushes one `SIdvBranchFlare` per flare onto the flare vector (`this + 0x38`): the angle
/// spaced by `period / count` with a random factor (`00b14e90`) and wrapped back below the
/// period, the radial influence a random value around `+0x30` (spread `+0x34`) divided by the
/// degrees constant at `010a4d90`, the exponents copied from `+0x38` and `+0x4C`, the length
/// influence random around `+0x44` (spread `+0x48`) and the distance around `+0x3C` (spread
/// `+0x40`).
pub fn fn_00b14ca0(e: &mut Engine, this: Ptr<CIdvBranch>, info: Ptr<SIdvBranchInfo>) {
    let info = info.addr();
    if e.mem.i32(info + 0x28) == 0 {
        return;
    }
    let period = x87_quiet(e.global::<f32>(ANGLE_PERIOD));
    let start = fn_00b14e90(e, 0.0, period);
    let count = e.mem.i32(info + 0x28);
    let spacing = (f64::from(e.global::<f32>(ANGLE_PERIOD)) / f64::from(count)) as f32;
    let flares = Ptr::new(this.addr() + 0x38);
    e.with_stack(0x18, |e, flare| {
        let base = flare.addr();
        let mut index = 0i32;
        while index < e.mem.i32(info + 0x28) {
            for (offset, value) in [
                (0u32, 0.0f32),
                (4, 0.0),
                (8, 1.0),
                (0x0C, 0.0),
                (0x10, 1.0),
                (0x14, 0.0),
            ] {
                e.mem.set_f32(base + offset, value);
            }
            let low = (ld(e, info + 0x2C) * f64::from(spacing)) as f32;
            let factor = fn_00b14e90(e, x87_quiet(low), x87_quiet(spacing));
            let mut angle = (f64::from(factor) * f64::from(index) + f64::from(start)) as f32;
            if ld(e, ANGLE_PERIOD) < f64::from(angle) {
                angle = (f64::from(angle) - ld(e, ANGLE_PERIOD)) as f32;
            }
            e.mem.set_f32(base, angle);
            copy_float(e, base + 8, info + 0x38);
            copy_float(e, base + 0x10, info + 0x4C);
            let high = (ld(e, info + 0x34) + ld(e, info + 0x30)) as f32;
            let low = (ld(e, info + 0x30) - ld(e, info + 0x34)) as f32;
            let random = fn_00b14e90(e, x87_quiet(low), x87_quiet(high));
            let degrees = e.global::<f64>(ANGLE_DIVISOR);
            e.mem
                .set_f32(base + 4, (f64::from(random) / degrees) as f32);
            let high = (ld(e, info + 0x44) + ld(e, info + 0x48)) as f32;
            let low = (ld(e, info + 0x44) - ld(e, info + 0x48)) as f32;
            let random = fn_00b14e90(e, x87_quiet(low), x87_quiet(high));
            e.mem.set_f32(base + 0x0C, random);
            let high = (ld(e, info + 0x3C) + ld(e, info + 0x40)) as f32;
            let low = (ld(e, info + 0x3C) - ld(e, info + 0x40)) as f32;
            let random = fn_00b14e90(e, x87_quiet(low), x87_quiet(high));
            e.mem.set_f32(base + 0x14, random);
            fn_00b15280(e, flares, flare);
            index += 1;
        }
    });
}

// Translated from 00b14e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// A random float between `low` and `high` (`cdecl`, the engine map gives it no name):
/// `rand()` divided by the double at `01029790`, times `high - low`, plus `low`, each step
/// rounded to a float.
pub fn fn_00b14e90(e: &mut Engine, low: f32, high: f32) -> f32 {
    let random = e.call(RAND, &args![]).i32();
    let divisor = e.global::<f64>(0x0102_9790);
    let unit = (f64::from(random) / divisor) as f32;
    let span = (f64::from(high) - f64::from(low)) as f32;
    x87_quiet((f64::from(unit) * f64::from(span) + f64::from(low)) as f32)
}

// Translated from 00b14ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Emits the normals of a branch's vertex rings into `geometry` (the engine map gives it no
/// name). For each of the branch's vertices (`+0x24`) and each of the `ring_length + 1` points
/// of its ring it takes the positions of the neighbouring points around the ring (wrapping)
/// and of the same point on the previous and next ring (clamped), fetched with `00b30f80`
/// at the vertex index plus the starting offset (`+0x2C`); the normalized difference around
/// the ring crossed with the normalized difference along the branch is added with
/// `00b30190`, and the vertex counter (`+0x22`) of the geometry is incremented.
pub fn fn_00b14ed0(e: &mut Engine, this: Ptr<CIdvBranch>, geometry: Ptr, ring_length: u16) {
    let base = this.addr();
    let width = i32::from(ring_length);
    e.with_stack(0x30, |e, frame| {
        let around = frame.addr();
        let along = frame.addr() + 0x0C;
        let normal = frame.addr() + 0x18;
        let mut ring = 0i32;
        while ring < e.mem.i32(base + 0x24) {
            let row = (width + 1).wrapping_mul(ring);
            let mut point = 0i32;
            while point <= width {
                let previous = if point == 0 {
                    row.wrapping_add(width).wrapping_sub(1)
                } else {
                    row.wrapping_add(point).wrapping_sub(1)
                };
                let next = if point == width {
                    row.wrapping_add(1)
                } else {
                    row.wrapping_add(point).wrapping_add(1)
                };
                let previous = previous.wrapping_add(e.mem.i32(base + 0x2C));
                let next = next.wrapping_add(e.mem.i32(base + 0x2C));
                let from = e.call(0x00b3_0f80, &args![geometry, previous]).u32();
                let to = e.call(0x00b3_0f80, &args![geometry, next]).u32();
                for k in 0..3 {
                    let delta = ld(e, to + 4 * k) - ld(e, from + 4 * k);
                    st(e, around + 4 * k, delta);
                }
                c_vec3_normalize(e, Ptr::new(around));
                let before = if ring == 0 {
                    row.wrapping_add(point)
                } else {
                    (ring - 1).wrapping_mul(width + 1).wrapping_add(point)
                };
                let after = if ring == e.mem.i32(base + 0x24) - 1 {
                    row.wrapping_add(point)
                } else {
                    (ring + 1).wrapping_mul(width + 1).wrapping_add(point)
                };
                let before = before.wrapping_add(e.mem.i32(base + 0x2C));
                let after = after.wrapping_add(e.mem.i32(base + 0x2C));
                let from = e.call(0x00b3_0f80, &args![geometry, before]).u32();
                let to = e.call(0x00b3_0f80, &args![geometry, after]).u32();
                for k in 0..3 {
                    let delta = ld(e, to + 4 * k) - ld(e, from + 4 * k);
                    st(e, along + 4 * k, delta);
                }
                c_vec3_normalize(e, Ptr::new(along));
                c_vec3_operator_p(e, Ptr::new(around), Ptr::new(normal), Ptr::new(along));
                e.call(0x00b3_0190, &args![geometry, normal]);
                let counter = e.mem.u16(geometry.addr() + 0x22);
                e.mem
                    .set_u16(geometry.addr() + 0x22, counter.wrapping_add(1));
                point += 1;
            }
            ring += 1;
        }
    });
}

// Translated from 00b15110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The blended percent of a branch within its ancestors (`cdecl`, the engine map gives it no
/// name): zero for no branch or a branch without a parent, otherwise with `p` the parent's
/// value, `(1 - p) * percentOfParent + p`.
pub fn fn_00b15110(e: &mut Engine, branch: Ptr<CIdvBranch>) -> f32 {
    if branch.is_null() {
        return 0.0;
    }
    let parent = e.get(branch, CIdvBranch::m_pParent);
    if parent.is_null() {
        return 0.0;
    }
    let parent_value = fn_00b15110(e, parent);
    let percent = x87_quiet(e.get(branch, CIdvBranch::m_fPercentOfParent));
    x87_quiet(
        ((1.0 - f64::from(parent_value)) * f64::from(percent) + f64::from(parent_value)) as f32,
    )
}

// Translated from 00b15170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked `operator[]` of a `std::vector` of bytes (the engine map gives it no name):
/// the address `first + index`, the invalid-parameter path when `index` is not below
/// `last - first`.
pub fn fn_00b15170(e: &mut Engine, this: Ptr<StlVector>, index: u32) -> u32 {
    let base = this.addr();
    let bytes = e.mem.u32(base + 0x10).wrapping_sub(e.mem.u32(base + 0x0C));
    if index >= bytes {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem.u32(base + 0x0C).wrapping_add(index)
}

// Translated from 00b151a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b151a0 - the FS:[0] exception frame is not translated.
/// A `std::vector` destructor (the engine map gives it no name): the elements are destroyed by
/// `00b1fca0`, then the proxy block (`+0`) is freed.
pub fn fn_00b151a0(e: &mut Engine, this: Ptr<StlVector>) {
    e.call(0x00b1_fca0, &args![this]);
    let proxy = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00b15210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b15210 - the FS:[0] exception frame is not translated.
/// The destructor of the flare vector (the engine map gives it no name): the elements are
/// destroyed by `00b15600`, then the proxy block (`+0`) is freed.
pub fn fn_00b15210(e: &mut Engine, this: Ptr<StlVector>) {
    fn_00b15600(e, this);
    let proxy = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00b15280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b15280 - the last two words passed to 00b16cd0 are uninitialised
// stack bytes in the exe; zero stands in for them until 00b16cd0 is shown not to read them.
/// `push_back` of the flare vector (a `std::vector` of 0x18-byte elements; the engine map gives
/// it no name): with spare capacity the element at `value` is constructed at the end
/// (`00b16cd0`; two of the words it takes are bytes left uninitialised on the exe's stack
/// and are passed as zero) and the end advanced by 0x18; otherwise the checked insert
/// `00b154d0` at the end is used.
pub fn fn_00b15280(e: &mut Engine, this: Ptr<StlVector>, value: Ptr) {
    let base = this.addr();
    let size = vector_len(e, base, 0x18);
    let capacity = if e.mem.u32(base + 0x0C) == 0 {
        0
    } else {
        let bytes = e.mem.u32(base + 0x14).wrapping_sub(e.mem.u32(base + 0x0C)) as i32;
        (bytes / 0x18) as u32
    };
    if size < capacity {
        let last = e.mem.u32(base + 0x10);
        e.call(
            0x00b1_6cd0,
            &args![last, 1u32, value, base.wrapping_add(8), 0u32, 0u32],
        );
        e.mem.set_u32(base + 0x10, last.wrapping_add(0x18));
    } else {
        let last = e.mem.u32(base + 0x10);
        let proxy = checked_iterator(e, base, last);
        e.with_stack(8, |e, out| {
            fn_00b154d0(e, this, out, proxy, last, value);
        });
    }
}

// Translated from 00b15390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<int>::push_back`: with spare capacity the int at `value` is constructed at the
/// end (`00b36b10`) and the end advanced by 4; otherwise the checked insert `00b156b0` at the
/// end is used.
pub fn std_vector_int_push_back(e: &mut Engine, this: Ptr<StlVector>, value: Ptr) {
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
        e.call(0x00b3_6b10, &args![last, 1u32, value]);
        e.mem.set_u32(base + 0x10, last.wrapping_add(4));
    } else {
        let last = e.mem.u32(base + 0x10);
        let proxy = checked_iterator(e, base, last);
        e.with_stack(8, |e, out| {
            e.call(0x00b1_56b0, &args![this, out, proxy, last, value]);
        });
    }
}

// Translated from 00b154d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked insert of the flare vector (the engine map gives it no name): inserts the
/// element at `value` before the iterator (`proxy`, `position`) with `00b157e0`, and writes
/// to `out` the iterator (`proxy`, address) of the element at the same index
/// (`00b16890` advances an iterator from the start by that index). Returns `out`.
pub fn fn_00b154d0(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: Ptr,
) -> Ptr {
    let base = this.addr();
    let offset = if vector_len(e, base, 0x18) == 0 {
        0
    } else {
        let first = e.mem.u32(base + 0x0C);
        let first_proxy = checked_iterator(e, base, first);
        if proxy == 0 || proxy != first_proxy {
            e.call(INVALID_PARAMETER, &args![]);
        }
        (position.wrapping_sub(first) as i32) / 0x18
    };
    e.call(0x00b1_57e0, &args![this, proxy, position, 1u32, value]);
    let first = e.mem.u32(base + 0x0C);
    let first_proxy = checked_iterator(e, base, first);
    e.with_stack(8, |e, iterator| {
        e.mem.set_u32(iterator.addr(), first_proxy);
        e.mem.set_u32(iterator.addr() + 4, first);
        e.call(0x00b1_6890, &args![iterator, offset]);
        let proxy_out = e.mem.u32(iterator.addr());
        let address_out = e.mem.u32(iterator.addr() + 4);
        e.mem.set_u32(out.addr(), proxy_out);
        e.mem.set_u32(out.addr() + 4, address_out);
    });
    out
}

// Translated from 00b15600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the flare vector (the engine map gives it no name): when it has storage, walks the
/// elements (nothing to destroy) and frees the block, then zeroes the first, last and end
/// pointers. (The element count the exe also computes is not used.)
pub fn fn_00b15600(e: &mut Engine, this: Ptr<StlVector>) {
    let base = this.addr();
    if e.mem.u32(base + 0x0C) != 0 {
        let last = e.mem.u32(base + 0x10);
        let mut cursor = e.mem.u32(base + 0x0C);
        while cursor != last {
            cursor = cursor.wrapping_add(0x18);
        }
        let block = e.mem.u32(base + 0x0C);
        e.call(OPERATOR_DELETE, &args![block]);
    }
    e.mem.set_u32(base + 0x0C, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x14, 0);
}

// Translated from 00b16950 (decompiled, FalloutNV.exe 1.4.0.525)
/// A checked range operation on a `std::vector` of ints (`cdecl`, the engine map gives it no
/// name): the two iterators (`first_proxy`, `first`) and (`last_proxy`, `last`) must have the
/// same nonzero proxy (else the invalid-parameter path), then `00b16b60` is called with the
/// range and its element count (`(last - first) / 4`) and `flag`. Returns what it returns.
pub fn fn_00b16950(
    e: &mut Engine,
    first_proxy: u32,
    first: u32,
    last_proxy: u32,
    last: u32,
    flag: u8,
) -> u32 {
    if last_proxy == 0 || last_proxy != first_proxy {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let count = (last.wrapping_sub(first) as i32) >> 2;
    e.call(0x00b1_6b60, &args![first, last, count, u32::from(flag)])
        .u32()
}

// Translated from 00b32350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<int>::clear`: erases the whole range with `00b335c0`, from the checked
/// iterator at the first element to the one at the last.
pub fn std_vector_int_clear(e: &mut Engine, this: Ptr<StlVector>) {
    let base = this.addr();
    let last = e.mem.u32(base + 0x10);
    let last_proxy = checked_iterator(e, base, last);
    let first = e.mem.u32(base + 0x0C);
    let first_proxy = checked_iterator(e, base, first);
    e.with_stack(8, |e, out| {
        e.call(
            0x00b3_35c0,
            &args![this, out, first_proxy, first, last_proxy, last],
        );
    });
}

/// `CIdvBranch` statics the computation reads: the pointer to the frond engine used while a
/// frond guide is built (`011f8c00`), the running counter of generated branches (`011f8bfc`),
/// the global up vector the ring orientation is measured against (`011f8c08`), a flag that
/// selects the wind-weight blending below the marker level (`011f8c04`, a byte) and a flag
/// that lets the last level be expanded (`011ad148`, a byte).
const FROND_ENGINE: u32 = 0x011f_8c00;
const BRANCH_COUNTER: u32 = 0x011f_8bfc;
const UP_VECTOR: u32 = 0x011f_8c08;
const WIND_BLEND_FLAG: u32 = 0x011f_8c04;
const EXPAND_LAST_FLAG: u32 = 0x011a_d148;
/// `CIdvRandom::Reseed` (`00b3f160`, `this` = the generator, a seed).
const RESEED: u32 = 0x00b3_f160;
/// `CIdvBezierSpline::ScaledVariance` (`00b37fa0`, `this` = the spline, one `float`, result in
/// `ST0`).
const SCALED_VARIANCE: u32 = 0x00b3_7fa0;
/// The frond engine's getters used to decide whether the branch belongs to a frond guide: a
/// flag byte (`00b18320`, `+0x4C`) and a level (`00b18300`, `+0x48`).
const FROND_FLAG: u32 = 0x00b1_8320;
const FROND_LEVEL: u32 = 0x00b1_8300;
/// The frond engine's hooks the computation calls instead of emitting geometry.
const FROND_BEGIN_GUIDE: u32 = 0x00b1_88d0;
const FROND_ADD_POINT: u32 = 0x00b1_8a60;
const FROND_END_GUIDE: u32 = 0x00b1_8b70;
/// `std::vector<SIdvBranch>::push_back` (`00b1e4c0`).
const CHILDREN_PUSH_BACK: u32 = 0x00b1_e4c0;

/// `CIdvBezierSpline::Evaluate` at `t`, the value left on the x87 stack.
fn spline_value(e: &mut Engine, spline: u32, t: f32) -> f64 {
    e.call(SPLINE_EVALUATE, &args![spline, x87_quiet(t)]).f64()
}

/// `CIdvBezierSpline::ScaledVariance` at `t`, stored as a float.
fn scaled_variance(e: &mut Engine, spline: u32, t: f32) -> f32 {
    e.call(SCALED_VARIANCE, &args![spline, x87_quiet(t)]).f32()
}

/// The angle in degrees between a vertex direction and the up vector, and the shade derived
/// from it, `1 - |90 - angle| * 0.0111...` (the constants at `010a4d90`, `01066808` and
/// `010a5498`).
fn vertex_shade(e: &mut Engine, vertex: u32) -> (f32, f32) {
    let angle = c_vec3_angle_between(e, Ptr::new(vertex), Ptr::new(UP_VECTOR));
    let degrees = (f64::from(angle) * e.global::<f64>(ANGLE_DIVISOR)) as f32;
    let off = (e.global::<f64>(0x0106_6808) - f64::from(degrees)) as f32;
    let distance = x87_quiet(e.call(ABSOLUTE_VALUE, &args![f64::from(off)]).f32());
    let shade = (1.0 - f64::from(distance) * ld(e, 0x010a_5498)) as f32;
    (degrees, shade)
}

/// Bends a vertex's transform (`vertex + 0x1C`): the rotation about the axis perpendicular to
/// the vertex direction and the up vector (`scratch` holds the axis, an identity-built matrix and
/// the product, 0x54 bytes), by `rotation * gravity * degrees * shade` where `rotation` is the
/// angle-profile spline (`info + 0x70`) at `t` mapped from [0, 1] to [1, -1]. The transform
/// becomes `transform * rotation`.
fn bend_vertex(e: &mut Engine, scratch: u32, vertex: u32, info: u32, t: f32, gravity: f32) {
    let axis = scratch;
    let rotation = scratch + 0x0C;
    let product = scratch + 0x30;
    let (degrees, shade) = vertex_shade(e, vertex);
    c_vec3_operator_p(e, Ptr::new(vertex), Ptr::new(axis), Ptr::new(UP_VECTOR));
    c_vec3_normalize(e, Ptr::new(axis));
    let profile = e.mem.u32(info + 0x70);
    let sample = spline_value(e, profile, t);
    let half = sample - e.global::<f64>(0x0101_1588);
    let turn = ((half + half) * e.global::<f64>(0x0101_a6b0)) as f32;
    for k in 0..9 {
        e.mem
            .set_f32(rotation + 4 * k, if k % 4 == 0 { 1.0 } else { 0.0 });
    }
    let angle =
        (f64::from(turn) * f64::from(gravity) * f64::from(degrees) * f64::from(shade)) as f32;
    let (x, y, z) = (e.mem.f32(axis), e.mem.f32(axis + 4), e.mem.f32(axis + 8));
    c_rot_transform_rotate_axis_from_identity(e, Ptr::new(rotation), angle, x, y, z);
    let result = c_rot_transform_operator_p(
        e,
        Ptr::new(vertex + 0x1C),
        Ptr::new(product),
        Ptr::new(rotation),
    );
    copy_words(e, vertex + 0x1C, result.addr(), 9);
}

// Translated from 00b11050 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b11050 - the FS:[0] exception frame (freeing the vertex array and the
// new child branch if an allocation or a constructor throws) is not translated.
/// `CIdvBranch::Compute` (Xbox PDB): builds the branch of level `level` (an index into the
/// branch-info list) from its branch info: the vertices (`this + 0x20`, `info->segments + 1`
/// of them) along the length spline with the packing exponent, each rotated by the start
/// angle, the random variance, the gravity bend and the cross-section twist, with radius,
/// wind weight and running length; the ring geometry for each vertex (`00b14450`, or the
/// frond engine's hooks for a frond guide) and the normals (`00b14ed0`); then for each child
/// (the info's frequency scaled by length) the position along the branch (`FillBranch`), and
/// either a bud (`ComputeBud`, at the last level) or a new branch computed recursively and
/// appended to the children vector (or discarded when the frond engine says so). Finally the
/// local leaves are cleared (last level) and the volume is computed.
///
/// The arguments are the random `seed`, the tree `size`, the `level`, the start `position`, the
/// `percent` along the parent, `parent_percent`, the parent's `matrix` and `direction`, the
/// `geometry` the branch writes into, the `leaves` vector, the `wind_weight`, a `wind_group`
/// (replaced by the running branch counter at the marker level) and a `max_radius` cap.
pub fn c_idv_branch_compute(
    e: &mut Engine,
    this: Ptr<CIdvBranch>,
    seed: i32,
    size: f32,
    level: i32,
    position: Ptr<CVec3>,
    percent: f32,
    parent_percent: f32,
    matrix: Ptr<CRotTransform>,
    direction: Ptr<CVec3>,
    geometry: Ptr,
    leaves: Ptr<StlVector>,
    wind_weight: f32,
    wind_group: i32,
    max_radius: f32,
) {
    let base = this.addr();
    let geometry_base = geometry.addr();
    let mut seed = seed;
    let mut level = level;
    let mut wind_group = wind_group;
    let frond = e.global::<u32>(FROND_ENGINE);
    let is_frond = if e.call(FROND_FLAG, &args![frond]).bool() {
        let frond = e.global::<u32>(FROND_ENGINE);
        level >= e.call(FROND_LEVEL, &args![frond]).i32()
    } else {
        false
    };
    if level == e.global::<i32>(LEVEL_MARKER) {
        wind_group = e.global::<i32>(BRANCH_COUNTER);
        let counter = e.global::<i32>(BRANCH_COUNTER);
        e.set_global(BRANCH_COUNTER, counter.wrapping_add(1));
    }
    if level == 0 {
        c_idv_branch_build_blossom_vectors(e, this);
    }
    let info = {
        let entry = vector_at(e, BRANCH_INFO_LIST, level as u32, 4);
        e.mem.u32(entry)
    };
    if !is_frond {
        let offset = u32::from(e.mem.u16(geometry_base + 0x22));
        e.mem.set_u32(base + 0x2C, offset);
    }
    e.mem.set_f32(base + 4, x87_quiet(percent));
    if !is_frond {
        fn_00b14ca0(e, this, Ptr::new(info));
    }
    let length_spline = e.mem.u32(info + 0x60);
    let length = (spline_value(e, length_spline, percent) * f64::from(size)) as f32;
    // The exe divides the length by the segment count here and never uses the quotient.
    let _segment_length = (f64::from(length) / f64::from(e.mem.i32(info + 4))) as f32;
    let low = x87_quiet(e.global::<f32>(0x010a_54a0));
    let high = x87_quiet(e.global::<f32>(0x0106_b158));
    let start_variance = e
        .call(GET_UNIFORM, &args![RANDOM_GENERATOR, low, high])
        .f32();
    let spline = e.mem.u32(info + 0x6C);
    let start_angle = spline_value(e, spline, percent) as f32;
    let spline = e.mem.u32(info + 0x54);
    let gravity = spline_value(e, spline, percent) as f32;
    let spline = e.mem.u32(info + 0x64);
    let mut radius = (spline_value(e, spline, percent) * f64::from(size)) as f32;
    let spline = e.mem.u32(info + 0x58);
    let flexibility = spline_value(e, spline, percent) as f32;
    let segments = e.mem.u16(info);
    e.mem.set_u16(base + 0x28, segments);
    let texture_offset = if e.mem.u8(info + 0x1E) != 0 {
        let sum = f64::from(start_variance) + e.global::<f64>(0x0101_1588);
        (sum + f64::from(percent)) as f32
    } else {
        0.0
    };
    let child_count = vector_len(e, base + 8, 0x0C);
    let twist = if child_count.wrapping_add(seed as u32) % 2 == 0 {
        x87_quiet(e.mem.f32(info + 0x20))
    } else {
        -x87_quiet(e.mem.f32(info + 0x20))
    };
    let t_tile = if e.mem.u8(info + 0x1D) != 0 {
        ld(e, info + 0x18)
    } else {
        f64::from(length) / f64::from(size) * ld(e, info + 0x18)
    };
    let s_tile = if e.mem.u8(info + 0x1C) != 0 {
        ld(e, info + 0x14)
    } else {
        ld(e, info + 0x14) * f64::from(radius) * ld(e, ANGLE_PERIOD)
    };
    e.with_stack(0x200, |e, frame| {
        let profile = frame.addr();
        let scratch = frame.addr() + 0x10;
        let direction_out = frame.addr() + 0x70;
        let child_entry = frame.addr() + 0xC0;
        let interpolated = frame.addr() + 0xD0;
        let word_buffer = frame.addr() + 0x100;
        e.mem.set_f32(profile, s_tile as f32);
        e.mem.set_f32(profile + 4, t_tile as f32);
        e.mem.set_f32(profile + 8, twist);
        let spline = e.mem.u32(info + 0x68);
        let mut first_radius = (spline_value(e, spline, 0.0) * f64::from(radius)) as f32;
        if f64::from(max_radius) > e.global::<f64>(0x0101_2060) {
            let capped = f64::from(max_radius) * ld(e, 0x010a_549c);
            if f64::from(first_radius) > capped {
                first_radius = capped as f32;
                radius = first_radius;
            }
        }
        let vertex_count = e.mem.i32(info + 4).wrapping_add(1);
        e.mem.set_i32(base + 0x24, vertex_count);
        if vertex_count < 2 {
            return;
        }
        let bytes = (vertex_count as u32).checked_mul(0x48).unwrap_or(u32::MAX);
        let memory = e.call(OPERATOR_NEW, &args![bytes]).u32();
        let vertices = if memory != 0 {
            let mut cursor = memory;
            let mut remaining = vertex_count;
            loop {
                remaining = remaining.wrapping_sub(1);
                if remaining < 0 {
                    break;
                }
                s_idv_branch_vertex_s_idv_branch_vertex(e, Ptr::new(cursor));
                cursor = cursor.wrapping_add(0x48);
            }
            memory
        } else {
            0
        };
        e.mem.set_u32(base + 0x20, vertices);
        if !is_frond {
            let ring_segments = e.mem.i32(info);
            let rows = e.mem.i32(base + 0x24);
            let count16 = rows
                .wrapping_sub(1)
                .wrapping_mul(ring_segments.wrapping_mul(2).wrapping_add(4))
                as u16;
            let index_bytes = u32::from(count16).checked_mul(2).unwrap_or(u32::MAX);
            let buffer = e.call(OPERATOR_NEW, &args![index_bytes]).u32();
            let start = i32::from(e.mem.u16(geometry_base + 0x22));
            let mut written = 0u32;
            let mut vertex_offset = 0i32;
            let mut row = 0i32;
            while row < e.mem.i32(base + 0x24).wrapping_sub(1) {
                let row_start = vertex_offset;
                let mut column = 0i32;
                while column < e.mem.i32(info).wrapping_add(1) {
                    let word = e
                        .mem
                        .i32(info)
                        .wrapping_add(start)
                        .wrapping_add(vertex_offset)
                        .wrapping_add(1) as u16;
                    e.mem.set_u16(buffer.wrapping_add(written * 2), word);
                    written += 1;
                    let word = start.wrapping_add(vertex_offset) as u16;
                    e.mem.set_u16(buffer.wrapping_add(written * 2), word);
                    written += 1;
                    vertex_offset = vertex_offset.wrapping_add(1);
                    column += 1;
                }
                let closing = e
                    .mem
                    .i32(info)
                    .wrapping_add(start)
                    .wrapping_add(row_start)
                    .wrapping_add(1) as u16;
                e.mem.set_u16(buffer.wrapping_add(written * 2), closing);
                written += 1;
                e.mem.set_u16(buffer.wrapping_add(written * 2), closing);
                written += 1;
                row += 1;
            }
            e.call(
                0x00b2_fdc0,
                &args![geometry, 0u32, buffer, u32::from(count16)],
            );
            let strips = e.mem.u16(geometry_base + 0x26);
            e.mem.set_u16(geometry_base + 0x26, strips.wrapping_add(1));
        }
        let first = e.mem.u32(base + 0x20);
        copy_words(e, first + 0x0C, position.addr(), 3);
        let disturbance = e.mem.u32(info + 0x50);
        let variance_a = scaled_variance(e, disturbance, 0.0);
        let disturbance = e.mem.u32(info + 0x50);
        let variance_b = scaled_variance(e, disturbance, 0.0);
        copy_words(e, first + 0x1C, matrix.addr(), 9);
        c_rot_transform_operator_p_ov2(e, matrix, Ptr::new(word_buffer), direction);
        let (x, y, z) = (
            e.mem.f32(word_buffer),
            e.mem.f32(word_buffer + 4),
            e.mem.f32(word_buffer + 8),
        );
        c_rot_transform_rotate_axis(e, Ptr::new(first + 0x1C), start_variance, x, y, z);
        let yaw = (f64::from(start_angle) + f64::from(variance_b)) as f32;
        c_rot_transform_rotate_yz(e, Ptr::new(first + 0x1C), yaw, variance_a);
        let aimed = c_vec3_operator_p_ov2(
            e,
            Ptr::new(BUD_AXIS),
            Ptr::new(direction_out),
            Ptr::new(first + 0x1C),
        );
        copy_words(e, first, aimed.addr(), 3);
        let spline = e.mem.u32(info + 0x68);
        let first_scale = (spline_value(e, spline, 0.0) * f64::from(radius)) as f32;
        e.mem.set_f32(first + 0x18, first_scale);
        let spline = e.mem.u32(info + 0x5C);
        let flex_scale = (spline_value(e, spline, 0.0) * f64::from(flexibility)) as f32;
        bend_vertex(e, scratch, first, info, 0.0, gravity);
        let aimed = c_vec3_operator_p_ov2(
            e,
            Ptr::new(BUD_AXIS),
            Ptr::new(direction_out),
            Ptr::new(first + 0x1C),
        );
        copy_words(e, first, aimed.addr(), 3);
        if !geometry.is_null() && level == 0 {
            let diameter = (ld(e, first + 0x18) + ld(e, first + 0x18)) as f32;
            e.mem.set_f32(geometry_base + 0x18, diameter);
            let reach = (f64::from(length) * ld(e, info + 8)) as f32;
            e.mem.set_f32(geometry_base + 0x1C, reach);
        }
        let mut weight = x87_quiet(wind_weight);
        if level <= e.global::<i32>(LEVEL_MARKER) {
            let value = fn_00b14ae0(e, Ptr::new(base), level, 0.0, flex_scale);
            e.mem.set_f32(first + 0x44, value);
            weight = x87_quiet(e.mem.f32(first + 0x44));
        } else if e.global::<u8>(WIND_BLEND_FLAG) != 0 {
            let value = fn_00b14b10(e, Ptr::new(base), wind_weight, flex_scale);
            e.mem.set_f32(first + 0x44, value);
            weight = x87_quiet(e.mem.f32(first + 0x44));
        }
        let ring_segments = i32::from(e.mem.u16(base + 0x28));
        if !is_frond {
            let ring_parent = if level != 0 { parent_percent } else { 0.0 };
            fn_00b14450(
                e,
                this,
                Ptr::new(first),
                0.0,
                ring_parent,
                ring_segments,
                geometry,
                Ptr::new(profile),
                weight,
                wind_group as u8,
                texture_offset,
                level,
            );
        } else {
            let frond = e.global::<u32>(FROND_ENGINE);
            e.call(FROND_BEGIN_GUIDE, &args![frond]);
            frond_add_point(e, first, weight, wind_group);
        }
        let mut running = 0.0f32;
        let vertices = e.mem.u32(base + 0x20);
        e.mem.set_f32(vertices + 0x40, 0.0);
        let mut previous = first;
        let mut index = 1i32;
        while index < e.mem.i32(base + 0x24) {
            let vertex = (index as u32)
                .wrapping_mul(0x48)
                .wrapping_add(e.mem.u32(base + 0x20));
            let along =
                (f64::from(index) / f64::from(e.mem.i32(base + 0x24).wrapping_sub(1))) as f32;
            let exponent = x87_quiet(e.mem.f32(info + 0x24));
            let packed = e
                .call(POW, &args![f64::from(along), f64::from(exponent)])
                .f32();
            let step = (f64::from(packed) * f64::from(length) - f64::from(running)) as f32;
            let spline = e.mem.u32(info + 0x68);
            let vertex_radius = (spline_value(e, spline, packed) * f64::from(radius)) as f32;
            e.mem.set_f32(vertex + 0x18, vertex_radius);
            let spline = e.mem.u32(info + 0x5C);
            let vertex_flex = (spline_value(e, spline, packed) * f64::from(flexibility)) as f32;
            copy_words(e, vertex + 0x1C, previous + 0x1C, 9);
            let aimed = c_vec3_operator_p_ov2(
                e,
                Ptr::new(BUD_AXIS),
                Ptr::new(direction_out),
                Ptr::new(vertex + 0x1C),
            );
            copy_words(e, vertex, aimed.addr(), 3);
            bend_vertex(e, scratch, vertex, info, packed, gravity);
            let disturbance = e.mem.u32(info + 0x50);
            let variance_a = scaled_variance(e, disturbance, packed);
            let disturbance = e.mem.u32(info + 0x50);
            let variance_b = scaled_variance(e, disturbance, packed);
            c_rot_transform_rotate_yz(e, Ptr::new(vertex + 0x1C), variance_b, variance_a);
            let aimed = c_vec3_operator_p_ov2(
                e,
                Ptr::new(BUD_AXIS),
                Ptr::new(direction_out),
                Ptr::new(vertex + 0x1C),
            );
            copy_words(e, vertex, aimed.addr(), 3);
            // `previous` is never null here (it starts as the first vertex); the exe still tests it.
            let source = if previous == 0 {
                position.addr()
            } else {
                for k in 0..3 {
                    let moved = f64::from(step) * ld(e, previous + 4 * k);
                    st(e, interpolated + 4 * k, moved);
                }
                for k in 0..3 {
                    let moved = ld(e, interpolated + 4 * k) + ld(e, previous + 0x0C + 4 * k);
                    st(e, interpolated + 0x10 + 4 * k, moved);
                }
                interpolated + 0x10
            };
            copy_words(e, vertex + 0x0C, source, 3);
            let mut vertex_weight = x87_quiet(wind_weight);
            if level <= e.global::<i32>(LEVEL_MARKER) {
                let value = fn_00b14ae0(e, Ptr::new(base), level, packed, vertex_flex);
                e.mem.set_f32(vertex + 0x44, value);
                vertex_weight = x87_quiet(e.mem.f32(vertex + 0x44));
            } else if e.global::<u8>(WIND_BLEND_FLAG) != 0 {
                let value = fn_00b14b10(e, Ptr::new(base), wind_weight, vertex_flex);
                e.mem.set_f32(vertex + 0x44, value);
                vertex_weight = x87_quiet(e.mem.f32(vertex + 0x44));
            }
            if !is_frond {
                let ring_parent = if level == 0 {
                    let knee = x87_quiet(e.mem.f32(info + 8));
                    if f64::from(packed) < f64::from(knee) {
                        1.0
                    } else {
                        (ld(e, info + 8) * e.global::<f64>(0x0101_a6b0) / (1.0 - ld(e, info + 8)))
                            as f32
                    }
                } else {
                    parent_percent
                };
                fn_00b14450(
                    e,
                    this,
                    Ptr::new(vertex),
                    packed,
                    ring_parent,
                    ring_segments,
                    geometry,
                    Ptr::new(profile),
                    vertex_weight,
                    wind_group as u8,
                    texture_offset,
                    level,
                );
            } else {
                frond_add_point(e, vertex, vertex_weight, wind_group);
            }
            previous = vertex;
            running = (f64::from(running) + f64::from(step)) as f32;
            let vertices = e.mem.u32(base + 0x20);
            e.mem.set_f32(
                (index as u32).wrapping_mul(0x48).wrapping_add(vertices) + 0x40,
                running,
            );
            index += 1;
        }
        if is_frond {
            let frond = e.global::<u32>(FROND_ENGINE);
            e.call(FROND_END_GUIDE, &args![frond, x87_quiet(first_radius)]);
        } else {
            let offset = e.mem.u16(base + 0x2C);
            e.mem.set_u16(geometry_base + 0x22, offset);
            let ring_length = e.mem.u16(base + 0x28);
            fn_00b14ed0(e, this, geometry, ring_length);
        }
        let children = ftol(e, ld(e, info + 0x10) / f64::from(size) * f64::from(length));
        level = level.wrapping_add(1);
        let is_last = level >= (vector_len(e, BRANCH_INFO_LIST, 4) as i32).wrapping_sub(1);
        if !is_last || e.global::<u8>(EXPAND_LAST_FLAG) != 0 {
            let mut child = 0i32;
            while child < children {
                e.mem.set_i32(child_entry, 0);
                e.mem.set_f32(child_entry + 4, 0.0);
                e.mem.set_u32(child_entry + 8, 0);
                if !is_last {
                    seed = seed.wrapping_add(3);
                    e.call(RESEED, &args![RANDOM_GENERATOR, seed]);
                }
                let along_branch = if !is_last && child == 0 {
                    let lower = x87_quiet(e.mem.f32(info + 8));
                    let upper_a = ((ld(e, info + 0x0C) - f64::from(lower)) * ld(e, 0x0101_ffc0)
                        + f64::from(lower)) as f32;
                    let lower_b = x87_quiet(e.mem.f32(info + 8));
                    let upper_b = ((ld(e, info + 0x0C) - f64::from(lower_b)) * ld(e, 0x0101_9de0)
                        + f64::from(lower_b)) as f32;
                    e.call(
                        GET_UNIFORM,
                        &args![RANDOM_GENERATOR, x87_quiet(upper_b), x87_quiet(upper_a)],
                    )
                    .f32()
                } else {
                    let upper = x87_quiet(e.mem.f32(info + 0x0C));
                    let lower = x87_quiet(e.mem.f32(info + 8));
                    e.call(GET_UNIFORM, &args![RANDOM_GENERATOR, lower, upper])
                        .f32()
                };
                let distance = (f64::from(along_branch) * f64::from(length)) as f32;
                c_idv_branch_fill_branch(e, this, Ptr::new(child_entry), distance);
                let mut child_weight = x87_quiet(wind_weight);
                if level.wrapping_sub(1) == e.global::<i32>(LEVEL_MARKER)
                    || e.global::<u8>(WIND_BLEND_FLAG) != 0
                {
                    let found = e.mem.i32(child_entry);
                    let fraction = e.mem.f32(child_entry + 4);
                    let vertices = e.mem.u32(base + 0x20);
                    let lower = x87_quiet(
                        e.mem
                            .f32((found as u32).wrapping_mul(0x48).wrapping_add(vertices) + 0x44),
                    );
                    let upper = ld(
                        e,
                        ((found as u32).wrapping_add(1))
                            .wrapping_mul(0x48)
                            .wrapping_add(vertices)
                            + 0x44,
                    );
                    child_weight = (f64::from(fraction) * (upper - f64::from(lower))
                        + f64::from(lower)) as f32;
                }
                if !is_last {
                    e.call(RESEED, &args![RANDOM_GENERATOR, seed]);
                    let high = x87_quiet(e.global::<f32>(0x0101_6410));
                    e.call(GET_UNIFORM, &args![RANDOM_GENERATOR, 0.0f32, high]);
                }
                let found = e.mem.i32(child_entry);
                let vertices = e.mem.u32(base + 0x20);
                let near = (found as u32).wrapping_mul(0x48).wrapping_add(vertices);
                let far = (found as u32)
                    .wrapping_add(1)
                    .wrapping_mul(0x48)
                    .wrapping_add(vertices);
                let fraction = e.mem.f32(child_entry + 4);
                vec_interpolate(
                    e,
                    Ptr::new(interpolated),
                    Ptr::new(near + 0x0C),
                    Ptr::new(far + 0x0C),
                    fraction,
                );
                let mut lerp = 1.0f32;
                if f64::from(e.mem.f32(info + 0x0C)) != f64::from(e.mem.f32(info + 8)) {
                    lerp = ((f64::from(along_branch) - ld(e, info + 8))
                        / (ld(e, info + 0x0C) - ld(e, info + 8))) as f32;
                }
                if is_last {
                    c_idv_branch_compute_bud(
                        e,
                        this,
                        this,
                        size,
                        level as u32,
                        Ptr::new(interpolated),
                        lerp,
                        Ptr::new(near + 0x1C),
                        Ptr::new(near),
                        leaves,
                        child_weight,
                        wind_group,
                    );
                } else {
                    let found = e.mem.i32(child_entry);
                    let vertices = e.mem.u32(base + 0x20);
                    let near = (found as u32).wrapping_mul(0x48).wrapping_add(vertices);
                    let near_radius = x87_quiet(e.mem.f32(near + 0x18));
                    let far = (found as u32)
                        .wrapping_add(1)
                        .wrapping_mul(0x48)
                        .wrapping_add(vertices);
                    let fraction = e.mem.f32(child_entry + 4);
                    let child_radius = (f64::from(fraction)
                        * (ld(e, far + 0x18) - f64::from(near_radius))
                        + f64::from(near_radius)) as f32;
                    let memory = e.call(OPERATOR_NEW, &args![0x50u32]).u32();
                    let child_branch = if memory != 0 {
                        c_idv_branch_c_idv_branch(e, Ptr::new(memory), this).addr()
                    } else {
                        0
                    };
                    e.mem.set_u32(child_entry + 8, child_branch);
                    let mut child_percent = ((1.0 - f64::from(parent_percent)) * f64::from(lerp)
                        + f64::from(parent_percent))
                        as f32;
                    if level <= 1 {
                        child_percent =
                            (f64::from(child_percent) * f64::from(child_percent)) as f32;
                    }
                    let found = e.mem.i32(child_entry);
                    let vertices = e.mem.u32(base + 0x20);
                    let near = (found as u32).wrapping_mul(0x48).wrapping_add(vertices);
                    c_idv_branch_compute(
                        e,
                        Ptr::new(child_branch),
                        seed,
                        size,
                        level,
                        Ptr::new(interpolated),
                        lerp,
                        child_percent,
                        Ptr::new(near + 0x1C),
                        Ptr::new(near),
                        geometry,
                        leaves,
                        child_weight,
                        wind_group,
                        child_radius,
                    );
                    let frond = e.global::<u32>(FROND_ENGINE);
                    let keep = e.call(FROND_FLAG, &args![frond]).bool() && {
                        let frond = e.global::<u32>(FROND_ENGINE);
                        level >= e.call(FROND_LEVEL, &args![frond]).i32()
                    };
                    if !keep {
                        e.call(CHILDREN_PUSH_BACK, &args![base + 8, child_entry]);
                    } else {
                        let doomed = e.mem.u32(child_entry + 8);
                        if doomed != 0 {
                            c_idv_branch_destructor(e, Ptr::new(doomed));
                            e.call(OPERATOR_DELETE, &args![doomed]);
                        }
                        e.mem.set_u32(child_entry + 8, 0);
                    }
                }
                child += 1;
            }
        }
        if is_last {
            std_vector_int_clear(e, Ptr::new(LOCAL_LEAVES));
        }
        c_idv_branch_compute_volume(e, this);
    });
}

/// Hands one vertex to the frond engine (`00b18a60`): the position (three words), the
/// transform (nine words), the wind weight and the wind group, all by value.
fn frond_add_point(e: &mut Engine, vertex: u32, weight: f32, wind_group: i32) {
    let frond = e.global::<u32>(FROND_ENGINE);
    let mut words = vec![frond];
    for k in 0..3 {
        words.push(e.mem.u32(vertex + 0x0C + 4 * k));
    }
    for k in 0..9 {
        words.push(e.mem.u32(vertex + 0x1C + 4 * k));
    }
    words.push(x87_quiet(weight).to_bits());
    words.push(wind_group as u32);
    e.call(FROND_ADD_POINT, &words);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b10dc0,
            c_idv_branch_c_idv_branch(Ptr<CIdvBranch>, Ptr<CIdvBranch>) -> Ptr<CIdvBranch>
        ),
        entry!(0x00b10e90, c_idv_branch_destructor(Ptr<CIdvBranch>)),
        entry!(
            0x00b11050,
            c_idv_branch_compute(
                Ptr<CIdvBranch>,
                i32,
                f32,
                i32,
                Ptr<CVec3>,
                f32,
                f32,
                Ptr<CRotTransform>,
                Ptr<CVec3>,
                Ptr,
                Ptr<StlVector>,
                f32,
                i32,
                f32,
            )
        ),
        entry!(
            0x00b12720,
            c_vec3_angle_between(Ptr<CVec3>, Ptr<CVec3>) -> f32
        ),
        entry!(0x00b12790, c_vec3_normalize(Ptr<CVec3>)),
        entry!(
            0x00b12810,
            c_vec3_operator_p(Ptr<CVec3>, Ptr<CVec3>, Ptr<CVec3>) -> Ptr<CVec3>
        ),
        entry!(
            0x00b12890,
            c_vec3_operator_p_ov2(Ptr<CVec3>, Ptr<CVec3>, Ptr<CRotTransform>) -> Ptr<CVec3>
        ),
        entry!(
            0x00b12940,
            c_rot_transform_operator_p(
                Ptr<CRotTransform>,
                Ptr<CRotTransform>,
                Ptr<CRotTransform>,
            ) -> Ptr<CRotTransform>
        ),
        entry!(
            0x00b12af0,
            c_rot_transform_c_rot_transform(Ptr<CRotTransform>) -> Ptr<CRotTransform>
        ),
        entry!(
            0x00b12b50,
            c_rot_transform_operator_p_ov2(
                Ptr<CRotTransform>,
                Ptr<CVec3>,
                Ptr<CVec3>,
            ) -> Ptr<CVec3>
        ),
        entry!(
            0x00b12c00,
            c_rot_transform_rotate_yz(Ptr<CRotTransform>, f32, f32)
        ),
        entry!(
            0x00b12d50,
            c_rot_transform_rotate_axis(Ptr<CRotTransform>, f32, f32, f32, f32)
        ),
        entry!(
            0x00b12ec0,
            c_rot_transform_rotate_axis_from_identity(Ptr<CRotTransform>, f32, f32, f32, f32)
        ),
        entry!(
            0x00b13000,
            vec_interpolate(Ptr<CVec3>, Ptr<CVec3>, Ptr<CVec3>, f32) -> Ptr<CVec3>
        ),
        entry!(
            0x00b13040,
            fn_00b13040(Ptr<CVec3>, Ptr<CVec3>, Ptr<CVec3>) -> Ptr<CVec3>
        ),
        entry!(
            0x00b130a0,
            s_idv_branch_vertex_s_idv_branch_vertex(Ptr<SIdvBranchVertex>) -> Ptr<SIdvBranchVertex>
        ),
        entry!(
            0x00b13100,
            c_idv_branch_compute_bud(
                Ptr<CIdvBranch>,
                Ptr<CIdvBranch>,
                f32,
                u32,
                Ptr<CVec3>,
                f32,
                Ptr<CRotTransform>,
                Ptr<CVec3>,
                Ptr<StlVector>,
                f32,
                i32,
            )
        ),
        entry!(
            0x00b134d0,
            c_rot_transform_rotate_y(Ptr<CRotTransform>, f32)
        ),
        entry!(0x00b135a0, fn_00b135a0(Ptr<CIdvBranch>, u16, Ptr)),
        entry!(0x00b13820, fn_00b13820(Ptr<CIdvBranch>, Ptr<StlVector>)),
        entry!(0x00b138c0, fn_00b138c0(Ptr) -> u32),
        entry!(0x00b13970, c_tree_file_access_parse_branch_parameter(Ptr) -> Ptr),
        entry!(
            0x00b13a40,
            c_idv_branch_build_blossom_vectors(Ptr<CIdvBranch>)
        ),
        entry!(
            0x00b13b50,
            c_idv_branch_is_blossom(Ptr<CIdvBranch>, Ptr<CIdvBranch>, f32) -> bool
        ),
        entry!(
            0x00b13c10,
            c_idv_branch_make_leaf(
                Ptr<CIdvBranch>,
                Ptr<CVec3>,
                f32,
                Ptr<CIdvBranch>,
                Ptr<CVec3>,
                Ptr<CVec3>,
                f32,
                i32,
                Ptr<StlVector>,
            )
        ),
        entry!(0x00b14230, fn_00b14230(Ptr<CVec3>) -> f32),
        entry!(
            0x00b14280,
            fn_00b14280(Ptr<CIdvBranch>, Ptr<CVec3>, u32, Ptr<StlVector>) -> bool
        ),
        entry!(
            0x00b14450,
            fn_00b14450(
                Ptr<CIdvBranch>,
                Ptr<SIdvBranchVertex>,
                f32,
                f32,
                i32,
                Ptr,
                Ptr,
                f32,
                u8,
                f32,
                i32,
            )
        ),
        entry!(
            0x00b14980,
            fn_00b14980(Ptr<SIdvBranchFlare>, f32, f32) -> f32
        ),
        entry!(0x00b14ae0, fn_00b14ae0(Ptr, i32, f32, f32) -> f32),
        entry!(0x00b14b10, fn_00b14b10(Ptr, f32, f32) -> f32),
        entry!(0x00b14b40, c_idv_branch_compute_volume(Ptr<CIdvBranch>)),
        entry!(
            0x00b14be0,
            c_idv_branch_fill_branch(Ptr<CIdvBranch>, Ptr, f32)
        ),
        entry!(
            0x00b14ca0,
            fn_00b14ca0(Ptr<CIdvBranch>, Ptr<SIdvBranchInfo>)
        ),
        entry!(0x00b14e90, fn_00b14e90(f32, f32) -> f32),
        entry!(0x00b14ed0, fn_00b14ed0(Ptr<CIdvBranch>, Ptr, u16)),
        entry!(0x00b15110, fn_00b15110(Ptr<CIdvBranch>) -> f32),
        entry!(0x00b15170, fn_00b15170(Ptr<StlVector>, u32) -> u32),
        entry!(0x00b151a0, fn_00b151a0(Ptr<StlVector>)),
        entry!(0x00b15210, fn_00b15210(Ptr<StlVector>)),
        entry!(0x00b15280, fn_00b15280(Ptr<StlVector>, Ptr)),
        entry!(0x00b15390, std_vector_int_push_back(Ptr<StlVector>, Ptr)),
        entry!(
            0x00b154d0,
            fn_00b154d0(Ptr<StlVector>, Ptr, u32, u32, Ptr) -> Ptr
        ),
        entry!(0x00b15600, fn_00b15600(Ptr<StlVector>)),
        entry!(0x00b16950, fn_00b16950(u32, u32, u32, u32, u8) -> u32),
        entry!(0x00b32350, std_vector_int_clear(Ptr<StlVector>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    /// A proxy value every test vector carries at `+0`.
    const PROXY: u32 = 0x5000_0001;

    type Log = Rc<RefCell<Vec<Vec<u32>>>>;
    type Samples = Rc<RefCell<Vec<Vec<f32>>>>;

    /// An engine with the exe's data range (statics and constants) mapped as zeroes.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        e
    }

    /// The `double` argument starting at word `at`.
    fn dbl(a: &[u32], at: usize) -> f64 {
        f64::from_bits(u64::from(a[at]) | (u64::from(a[at + 1]) << 32))
    }

    fn put(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// A fresh block holding `values`.
    fn block(e: &mut Engine, values: &[f32]) -> u32 {
        let at = e.mem.alloc(4 * values.len().max(1) as u32);
        put(e, at, values);
        at
    }

    fn close(actual: &[f32], expected: &[f32]) {
        assert_eq!(actual.len(), expected.len(), "{actual:?} != {expected:?}");
        for (a, b) in actual.iter().zip(expected) {
            assert!((a - b).abs() < 1e-5, "{actual:?} != {expected:?}");
        }
    }

    /// Makes the `std::vector` at `at` hold `count` zeroed elements of `size` bytes with room for
    /// `capacity`, and returns the address of the elements.
    fn vector(e: &mut Engine, at: u32, size: u32, count: u32, capacity: u32) -> u32 {
        let data = e.mem.alloc((size * capacity).max(1));
        e.mem.set_u32(at, PROXY);
        e.mem.set_u32(at + 0x0C, data);
        e.mem.set_u32(at + 0x10, data + size * count);
        e.mem.set_u32(at + 0x14, data + size * capacity);
        data
    }

    /// Doubles that do nothing.
    fn silence(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register_double(*addr, |_, _| Ret::default());
        }
    }

    /// A double that records its argument words.
    fn spy(e: &mut Engine, addr: u32) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        log
    }

    /// A double that records `count` floats read through the pointer argument `arg`.
    fn sample(e: &mut Engine, addr: u32, arg: usize, count: u32) -> Samples {
        let log: Samples = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |e, a| {
            let at = a[arg];
            sink.borrow_mut()
                .push((0..count).map(|i| e.mem.f32(at + 4 * i)).collect());
            Ret::default()
        });
        log
    }

    /// The `CVec3` helpers outside this file: the product `this * scalar` (`out` and its address
    /// returned) and the sum.
    fn vec3_doubles(e: &mut Engine) {
        e.register_double(VEC3_SCALE, |e, a| {
            let s = f32::from_bits(a[2]);
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) * s;
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            a[1].into_ret()
        });
        e.register_double(VEC3_ADD, |e, a| {
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) + e.mem.f32(a[2] + 4 * k);
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            a[1].into_ret()
        });
    }

    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A branch whose children vector holds the given child pointers.
    fn branch_with(e: &mut Engine, children: &[u32]) -> u32 {
        let branch = e.new_object::<CIdvBranch>().addr();
        let data = vector(
            e,
            branch + 8,
            0x0C,
            children.len() as u32,
            children.len() as u32,
        );
        for (i, child) in children.iter().enumerate() {
            e.mem.set_u32(data + 0x0C * i as u32 + 8, *child);
        }
        branch
    }

    #[test]
    fn constructor_resets_the_branch() {
        let mut e = engine();
        let branch = e.new_object::<CIdvBranch>();
        let parent = e.new_object::<CIdvBranch>();
        let b = branch.addr();
        for off in (0..0x50).step_by(4) {
            e.mem.set_u32(b + off, 0xDEAD_BEEF);
        }
        let children = spy(&mut e, 0x00b1_e3e0);
        let flares = spy(&mut e, 0x00b3_1270);
        let r = e.call(0x00b1_0dc0, &args![branch, parent]).u32();
        assert_eq!(r, b);
        assert_eq!(e.mem.u32(b), parent.addr());
        assert_eq!(e.mem.f32(b + 4), 0.0);
        assert_eq!(e.mem.u32(b + 0x20), 0);
        assert_eq!(e.mem.i32(b + 0x24), -1);
        assert_eq!(e.mem.u16(b + 0x28), 0);
        assert_eq!(e.mem.i32(b + 0x2C), -1);
        assert_eq!(e.mem.f32(b + 0x30), 0.0);
        assert_eq!(e.mem.f32(b + 0x34), 0.0);
        assert_eq!(*children.borrow(), vec![vec![b + 8]]);
        assert_eq!(*flares.borrow(), vec![vec![b + 0x38]]);
    }

    #[test]
    fn destructor_frees_the_children_and_the_vectors() {
        let mut e = engine();
        let root = e.new_object::<CIdvBranch>().addr();
        let child = e.new_object::<CIdvBranch>().addr();
        let vertices = e.mem.alloc(0x48);
        e.mem.set_u32(root + 0x20, vertices);
        let entries = vector(&mut e, root + 8, 0x0C, 2, 2);
        e.mem.set_u32(entries + 8, child);
        e.mem.set_u32(root + 0x38, 0x77);
        silence(&mut e, &[OPERATOR_DELETE, 0x00b1_fca0]);
        e.call_log = Some(Vec::new());
        e.call(0x00b1_0e90, &args![Ptr::<CIdvBranch>::new(root)]);
        let log = e.call_log.take().unwrap();
        let deleted: Vec<u32> = calls_to(&log, OPERATOR_DELETE)
            .iter()
            .map(|a| a[0])
            .collect();
        assert_eq!(deleted, vec![vertices, 0, 0, 0, child, 0x77, PROXY]);
        assert_eq!(
            calls_to(&log, 0x00b1_fca0),
            vec![vec![child + 8], vec![root + 8]]
        );
        assert_eq!(e.mem.u32(root + 0x20), 0);
        assert_eq!(e.mem.u32(entries + 8), 0);
    }

    #[test]
    fn angle_between_clamps_the_dot_product() {
        let cases = [
            (2.0f32, Some(1.0f64)),
            (-3.0, Some(-1.0)),
            (0.5, Some(0.5)),
            (f32::NAN, None),
        ];
        for (dot, expected) in cases {
            let mut e = engine();
            e.set_global(0x0101_a6b0, -1.0f64);
            e.set_global(0x0101_2054, -1.0f32);
            e.set_global(0x0101_2070, 1.0f64);
            e.register_double(VEC3_DOT, move |_, _| dot.into_ret());
            let seen = Rc::new(Cell::new(f64::NAN));
            let sink = seen.clone();
            e.register_double(ACOS, move |_, a| {
                sink.set(dbl(a, 0));
                dbl(a, 0).acos().into_ret()
            });
            let (this, other) = (block(&mut e, &[0.0; 3]), block(&mut e, &[0.0; 3]));
            let result = e.call(0x00b1_2720, &args![this, other]).f32();
            match expected {
                Some(argument) => {
                    assert_eq!(seen.get(), argument);
                    assert_eq!(result, (argument.acos()) as f32);
                }
                None => {
                    assert!(seen.get().is_nan());
                    assert!(result.is_nan());
                }
            }
        }
    }

    #[test]
    fn normalize_scales_by_the_rounded_reciprocal_length() {
        let mut e = engine();
        let v = block(&mut e, &[3.0, 4.0, 0.0]);
        e.call(0x00b1_2790, &args![v]);
        let inverse = f64::from(0.2f32);
        let expected = [
            (3.0 * inverse) as f32,
            (4.0 * inverse) as f32,
            (0.0 * inverse) as f32,
        ];
        assert_eq!(floats(&e, v, 3), expected);
        let zero = block(&mut e, &[0.0; 3]);
        e.call(0x00b1_2790, &args![zero]);
        assert!(floats(&e, zero, 3).iter().all(|x| x.is_nan()));
    }

    #[test]
    fn vec3_cross_product() {
        let mut e = engine();
        let a = block(&mut e, &[1.0, 2.0, 3.0]);
        let b = block(&mut e, &[4.0, 5.0, 6.0]);
        let out = block(&mut e, &[9.0; 3]);
        let r = e.call(0x00b1_2810, &args![a, out, b]).u32();
        assert_eq!(r, out);
        assert_eq!(floats(&e, out, 3), vec![-3.0, 6.0, -3.0]);
    }

    #[test]
    fn vec3_times_matrix() {
        let mut e = engine();
        let v = block(&mut e, &[1.0, 2.0, 3.0]);
        let m = block(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let out = block(&mut e, &[0.0; 3]);
        let r = e.call(0x00b1_2890, &args![v, out, m]).u32();
        assert_eq!(r, out);
        assert_eq!(floats(&e, out, 3), vec![30.0, 36.0, 42.0]);
    }

    #[test]
    fn matrix_product() {
        let mut e = engine();
        let a = block(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let b = block(&mut e, &[9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
        let out = block(&mut e, &[0.0; 9]);
        let r = e.call(0x00b1_2940, &args![a, out, b]).u32();
        assert_eq!(r, out);
        assert_eq!(
            floats(&e, out, 9),
            vec![30.0, 24.0, 18.0, 84.0, 69.0, 54.0, 138.0, 114.0, 90.0]
        );
    }

    #[test]
    fn matrix_constructor_makes_the_identity() {
        let mut e = engine();
        let m = block(&mut e, &[7.0; 9]);
        let r = e.call(0x00b1_2af0, &args![m]).u32();
        assert_eq!(r, m);
        assert_eq!(
            floats(&e, m, 9),
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
    }

    #[test]
    fn matrix_times_vector() {
        let mut e = engine();
        let m = block(&mut e, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
        let v = block(&mut e, &[1.0, 2.0, 3.0]);
        let out = block(&mut e, &[0.0; 3]);
        let r = e.call(0x00b1_2b50, &args![m, out, v]).u32();
        assert_eq!(r, out);
        assert_eq!(floats(&e, out, 3), vec![14.0, 32.0, 50.0]);
    }

    fn identity(e: &mut Engine) -> u32 {
        block(e, &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
    }

    #[test]
    fn rotate_yz_builds_the_two_angle_rotation() {
        let mut e = engine();
        e.set_global(ANGLE_DIVISOR, 1.0f64);
        let m = identity(&mut e);
        e.call(0x00b1_2c00, &args![m, 0.5f32, 0.25f32]);
        let (ca, sa) = (0.5f32.cos(), 0.5f32.sin());
        let (cb, sb) = (0.25f32.cos(), 0.25f32.sin());
        close(
            &floats(&e, m, 9),
            &[ca * cb, ca * sb, -sa, -sb, cb, 0.0, sa * cb, sa * sb, ca],
        );
    }

    #[test]
    fn rotate_axis_multiplies_by_the_axis_rotation() {
        let mut e = engine();
        e.set_global(ANGLE_DIVISOR, 1.0f64);
        let m = identity(&mut e);
        e.call(0x00b1_2d50, &args![m, 0.5f32, 0.0f32, 0.0f32, 1.0f32]);
        let (c, s) = (0.5f32.cos(), 0.5f32.sin());
        close(&floats(&e, m, 9), &[c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn rotate_axis_from_identity_replaces_the_matrix() {
        let mut e = engine();
        e.set_global(ANGLE_DIVISOR, 1.0f64);
        let m = block(&mut e, &[5.0; 9]);
        e.call(0x00b1_2ec0, &args![m, 0.5f32, 0.0f32, 0.0f32, 1.0f32]);
        let (c, s) = (0.5f32.cos(), 0.5f32.sin());
        close(&floats(&e, m, 9), &[c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0]);
    }

    #[test]
    fn interpolation_adds_the_scaled_difference() {
        let mut e = engine();
        vec3_doubles(&mut e);
        let a = block(&mut e, &[1.0, 2.0, 3.0]);
        let b = block(&mut e, &[3.0, 6.0, 9.0]);
        let out = block(&mut e, &[0.0; 3]);
        let r = e.call(0x00b1_3000, &args![out, a, b, 0.5f32]).u32();
        assert_eq!(r, out);
        assert_eq!(floats(&e, out, 3), vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn vec3_difference() {
        let mut e = engine();
        let a = block(&mut e, &[5.0, 7.0, 9.0]);
        let b = block(&mut e, &[1.0, 2.0, 3.0]);
        let out = block(&mut e, &[0.0; 3]);
        let r = e.call(0x00b1_3040, &args![a, out, b]).u32();
        assert_eq!(r, out);
        assert_eq!(floats(&e, out, 3), vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn vertex_constructor_zeroes_the_vectors_and_sets_the_identity() {
        let mut e = engine();
        let vertex = e.mem.alloc(0x48);
        for off in (0..0x48).step_by(4) {
            e.mem.set_u32(vertex + off, 0xDEAD_BEEF);
        }
        let r = e.call(0x00b1_30a0, &args![vertex]).u32();
        assert_eq!(r, vertex);
        assert_eq!(floats(&e, vertex, 6), vec![0.0; 6]);
        assert_eq!(e.mem.u32(vertex + 0x18), 0xDEAD_BEEF);
        assert_eq!(
            floats(&e, vertex + 0x1C, 9),
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
        );
    }

    #[test]
    fn rotate_y_multiplies_by_the_y_rotation() {
        let mut e = engine();
        e.set_global(ANGLE_DIVISOR, 1.0f64);
        let m = identity(&mut e);
        e.call(0x00b1_34d0, &args![m, 0.5f32]);
        let (c, s) = (0.5f32.cos(), 0.5f32.sin());
        close(&floats(&e, m, 9), &[c, 0.0, -s, 0.0, 1.0, 0.0, s, 0.0, c]);
    }

    #[test]
    fn bud_vertices_and_leaf() {
        let mut e = engine();
        e.set_global(ANGLE_DIVISOR, 1.0f64);
        e.set_global(0x0101_3ea4, 0.5f32);
        put(&mut e, BUD_AXIS, &[0.0, 1.0, 0.0]);
        let info = e.mem.alloc(0x74);
        e.mem.set_i32(info + 4, 3);
        e.mem.set_u32(info + 0x60, 0x1234);
        let list = vector(&mut e, BRANCH_INFO_LIST, 4, 1, 1);
        e.mem.set_u32(list, info);
        vec3_doubles(&mut e);
        e.register_double(SPLINE_EVALUATE, |_, _| 2.0f32.into_ret());
        e.register_double(GET_UNIFORM, |_, _| 0.0f32.into_ret());
        let (info_block, textures) = leaf_info_block(&mut e, 1);
        e.mem.set_u32(info_block + 0x40, 1);
        put(&mut e, textures + 4, &[0.1, 0.2, 0.3, 0.5]);
        e.set_global(0x0101_e568, 255.0f64);
        e.set_global(0x0103_0f28, 0.001f64);
        let others = vector(&mut e, NON_BLOSSOM_INDICES, 4, 1, 1);
        e.mem.set_u32(others, 0);
        let leaves_at = 0x011f_8d00;
        vector(&mut e, leaves_at, 4, 0, 4);
        int_vector_doubles(&mut e);
        let leaf = e.mem.alloc(0x4C);
        e.register_double(OPERATOR_NEW, move |_, _| leaf.into_ret());
        silence(&mut e, &[BILLBOARD_LEAF_SET_COLOR]);
        let leaf_args: Log = Rc::new(RefCell::new(Vec::new()));
        let leaf_points: Samples = Rc::new(RefCell::new(Vec::new()));
        let (sink, points) = (leaf_args.clone(), leaf_points.clone());
        e.register_double(BILLBOARD_LEAF_CONSTRUCT, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            points
                .borrow_mut()
                .push((0..3).map(|k| e.mem.f32(a[1] + 4 * k)).collect());
            a[0].into_ret()
        });
        let branch = e.new_object::<CIdvBranch>();
        let matrix = identity(&mut e);
        let axis = block(&mut e, &[0.0, 0.0, 1.0]);
        let position = block(&mut e, &[1.0, 2.0, 3.0]);
        e.call(
            0x00b1_3100,
            &args![
                branch, 0u32, 3.0f32, 0u32, position, 0.25f32, matrix, axis, leaves_at, 0.75f32,
                4i32
            ],
        );
        let made = leaf_args.borrow().clone();
        assert_eq!(made.len(), 1);
        let a = &made[0];
        assert_eq!(
            vec![a[0], a[2], a[3], a[4], a[5]],
            vec![leaf, 255, 0, 0.75f32.to_bits(), 4]
        );
        close(&leaf_points.borrow()[0], &[1.0, 4.0, 3.0]);
    }

    #[test]
    fn strip_indices_of_a_branch() {
        let mut e = engine();
        let branch = e.new_object::<CIdvBranch>().addr();
        e.mem.set_u16(branch + 0x28, 2);
        e.mem.set_i32(branch + 0x24, 2);
        e.mem.set_i32(branch + 0x2C, 10);
        let geometry = e.mem.alloc(0x40);
        e.mem.set_u16(geometry + 0x26, 4);
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let sink = sizes.clone();
        e.register_double(OPERATOR_NEW, move |e, a| {
            sink.borrow_mut().push(a[0]);
            e.mem.alloc(a[0]).into_ret()
        });
        let registered = spy(&mut e, 0x00b2_fdc0);
        e.call(
            0x00b1_35a0,
            &args![Ptr::<CIdvBranch>::new(branch), 7u16, geometry],
        );
        assert_eq!(*sizes.borrow(), vec![16]);
        let call = registered.borrow()[0].clone();
        assert_eq!(call[0], geometry);
        assert_eq!(call[1], 7);
        assert_eq!(call[3], 8);
        let indices: Vec<u16> = (0..8).map(|i| e.mem.u16(call[2] + 2 * i)).collect();
        assert_eq!(indices, vec![13, 10, 14, 11, 15, 12, 13, 13]);
        assert_eq!(e.mem.u16(geometry + 0x26), 5);
        // Fewer than two segments: nothing happens.
        e.mem.set_u16(branch + 0x28, 1);
        e.call(
            0x00b1_35a0,
            &args![Ptr::<CIdvBranch>::new(branch), 7u16, geometry],
        );
        assert_eq!(registered.borrow().len(), 1);
        assert_eq!(e.mem.u16(geometry + 0x26), 5);
    }

    #[test]
    fn descendants_are_listed_in_order() {
        let mut e = engine();
        let grandchild = branch_with(&mut e, &[]);
        let first = branch_with(&mut e, &[grandchild]);
        let second = branch_with(&mut e, &[]);
        let root = branch_with(&mut e, &[first, second]);
        let list = 0x011f_8d00;
        let data = vector(&mut e, list, 4, 0, 8);
        e.register_double(0x00b3_6b10, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.call(
            0x00b1_3820,
            &args![Ptr::<CIdvBranch>::new(root), Ptr::<StlVector>::new(list)],
        );
        let listed: Vec<u32> = (0..4).map(|i| e.mem.u32(data + 4 * i)).collect();
        assert_eq!(listed, vec![root, first, grandchild, second]);
        assert_eq!(e.mem.u32(list + 0x10), data + 16);
    }

    #[test]
    fn whole_range_call_on_a_vector_of_ints() {
        let mut e = engine();
        let list = 0x011f_8d00;
        let data = vector(&mut e, list, 4, 3, 3);
        let seen: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(0x00b1_6b60, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            0x77u32.into_ret()
        });
        let r = e.call(0x00b1_38c0, &args![Ptr::<()>::new(list)]).u32();
        assert_eq!(r, 0x77);
        assert_eq!(*seen.borrow(), vec![vec![data, data + 12, 3, 0]]);
    }

    #[test]
    fn spline_is_allocated_parsed_and_the_string_tidied() {
        let mut e = engine();
        e.register_double(OPERATOR_NEW, |_, _| 0u32.into_ret());
        let none = e.call(0x00b1_3970, &args![0x1234u32]).u32();
        assert_eq!(none, 0);
        let memory = e.mem.alloc(0x84);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a[0], 0x84);
            memory.into_ret()
        });
        let parse: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = parse.clone();
        e.register_double(PARSE_STRING, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            0x4321u32.into_ret()
        });
        let construct: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = construct.clone();
        e.register_double(SPLINE_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            a[0].into_ret()
        });
        let tidy = spy(&mut e, STRING_TIDY);
        let spline = e.call(0x00b1_3970, &args![0x1234u32]).u32();
        assert_eq!(spline, memory);
        let text = parse.borrow()[0][1];
        assert_eq!(parse.borrow()[0][0], 0x1234);
        assert_eq!(*construct.borrow(), vec![vec![memory, 0x4321]]);
        assert_eq!(*tidy.borrow(), vec![vec![text, 1, 0]]);
    }

    /// The leaf-info block with `textures` 0x54-byte entries.
    fn leaf_info_block(e: &mut Engine, textures: u32) -> (u32, u32) {
        let info = e.mem.alloc(0x60);
        e.set_global(LEAF_INFO, info);
        let data = vector(e, info + 0x10, 0x54, textures, textures);
        (info, data)
    }

    /// Doubles for the integer-vector clear and push_back the blossom code uses.
    fn int_vector_doubles(e: &mut Engine) {
        e.register_double(0x00b3_35c0, |e, a| {
            let first = e.mem.u32(a[0] + 0x0C);
            e.mem.set_u32(a[0] + 0x10, first);
            Ret::default()
        });
        e.register_double(0x00b3_6b10, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
    }

    #[test]
    fn blossom_vectors_split_by_the_first_byte() {
        let mut e = engine();
        let (_, textures) = leaf_info_block(&mut e, 3);
        e.mem.set_u8(textures, 1);
        e.mem.set_u8(textures + 0x54, 0);
        e.mem.set_u8(textures + 0xA8, 1);
        let blossom = vector(&mut e, BLOSSOM_INDICES, 4, 2, 8);
        let others = vector(&mut e, NON_BLOSSOM_INDICES, 4, 2, 8);
        int_vector_doubles(&mut e);
        let this = e.new_object::<CIdvBranch>();
        e.call(0x00b1_3a40, &args![this]);
        let read = |e: &Engine, vector: u32, data: u32| -> Vec<u32> {
            let count = (e.mem.u32(vector + 0x10) - data) / 4;
            (0..count).map(|i| e.mem.u32(data + 4 * i)).collect()
        };
        assert_eq!(read(&e, BLOSSOM_INDICES, blossom), vec![0, 1, 4, 5]);
        assert_eq!(read(&e, NON_BLOSSOM_INDICES, others), vec![2, 3]);
    }

    #[test]
    fn blossom_decision_uses_the_random_draw() {
        // (level, percent of the node, percent argument, random draw, expected, drawn)
        let cases = [
            (0u32, Some(0.5f32), 0.9f32, 0.7f32, true, true),
            (0, Some(0.5), 0.9, 0.9, false, true),
            (0, Some(0.1), 0.9, 0.7, false, false),
            (1, None, 0.5, 0.7, true, true),
            (1, None, 0.1, 0.7, false, false),
        ];
        for (level, node_percent, percent, draw, expected, drawn) in cases {
            let mut e = engine();
            let (info, _) = leaf_info_block(&mut e, 0);
            e.mem.set_u32(info + 0x34, level);
            e.mem.set_f32(info + 0x2C, 0.2);
            e.mem.set_f32(info + 0x30, 0.8);
            let node = match node_percent {
                Some(p) => {
                    let node = e.new_object::<CIdvBranch>();
                    e.mem.set_f32(node.addr() + 4, p);
                    node.addr()
                }
                None => 0,
            };
            let calls = Rc::new(RefCell::new(Vec::new()));
            let sink = calls.clone();
            e.register_double(GET_UNIFORM, move |_, a| {
                sink.borrow_mut().push(a.to_vec());
                draw.into_ret()
            });
            let result = e.call(0x00b1_3b50, &args![0u32, node, percent]).bool();
            assert_eq!(result, expected, "{level} {percent}");
            let expected_calls: Vec<Vec<u32>> = if drawn {
                vec![vec![RANDOM_GENERATOR, 0.0f32.to_bits(), 1.0f32.to_bits()]]
            } else {
                vec![]
            };
            assert_eq!(*calls.borrow(), expected_calls, "{level} {percent}");
        }
    }

    #[test]
    fn approximate_length_from_the_float_bits() {
        let mut e = engine();
        let v = block(&mut e, &[3.0, 4.0, 0.0]);
        assert_eq!(e.call(0x00b1_4230, &args![v]).f32(), 5.125);
    }

    #[test]
    fn approximate_length_of_two_nans_keeps_the_positive_one() {
        // Oracle vector 00b14230_8: x = -qNaN 0xffffffff, y = -0, z = +qNaN 0x7fffffff.
        // The two NaN sums tie on the significand, so the x87 keeps the positive one:
        // the exe returns 0x5fbfffff.
        let mut e = engine();
        let v = block(&mut e, &[0.0, 0.0, 0.0]);
        e.mem.set_u32(v, 0xffff_ffff);
        e.mem.set_u32(v + 4, 0x8000_0000);
        e.mem.set_u32(v + 8, 0x7fff_ffff);
        assert_eq!(e.call(0x00b1_4230, &args![v]).f32().to_bits(), 0x5fbf_ffff);
    }

    #[test]
    fn leaf_spacing_check() {
        let mut e = engine();
        let (info, textures) = leaf_info_block(&mut e, 1);
        e.mem.set_f32(info + 0x28, 2.0);
        put(&mut e, textures + 0x48, &[0.5, 1.0]);
        let first = block(&mut e, &[0.0, 50.0, 50.0, 50.0]);
        let second = block(&mut e, &[0.0, 10.0, 10.0, 10.0]);
        let list = 0x011f_8d00;
        let data = vector(&mut e, list, 4, 2, 2);
        e.mem.set_u32(data, first);
        e.mem.set_u32(data + 4, second);
        let inside = block(&mut e, &[11.0, 9.0, 10.0]);
        let outside = block(&mut e, &[13.0, 10.0, 10.0]);
        let call = |e: &mut Engine, point: u32| {
            e.call(0x00b1_4280, &args![0u32, point, 0u32, list]).bool()
        };
        assert!(!call(&mut e, inside));
        assert!(call(&mut e, outside));
        e.mem.set_u32(list + 0x10, data);
        assert!(call(&mut e, inside));
        // No textures at all.
        let first_texture = e.mem.u32(info + 0x1C);
        e.mem.set_u32(info + 0x20, first_texture);
        assert!(!call(&mut e, outside));
    }

    #[test]
    fn leaf_is_built_and_colored() {
        let mut e = engine();
        e.set_global(0x0103_0f28, 0.001f64);
        e.set_global(0x0101_e568, 255.0f64);
        let (info, textures) = leaf_info_block(&mut e, 1);
        e.mem.set_u32(info + 0x40, 1);
        put(&mut e, textures + 4, &[0.1, 0.2, 0.3, 0.5]);
        let others = vector(&mut e, NON_BLOSSOM_INDICES, 4, 1, 1);
        e.mem.set_u32(others, 1);
        let leaves_at = 0x011f_8d00;
        let data = vector(&mut e, leaves_at, 4, 0, 4);
        int_vector_doubles(&mut e);
        let leaf = e.mem.alloc(0x4C);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a[0], 0x4C);
            leaf.into_ret()
        });
        e.register_double(GET_UNIFORM, |_, _| 0.25f32.into_ret());
        let built: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = built.clone();
        e.register_double(BILLBOARD_LEAF_CONSTRUCT, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            a[0].into_ret()
        });
        let colours = sample(&mut e, BILLBOARD_LEAF_SET_COLOR, 1, 3);
        let branch = e.new_object::<CIdvBranch>();
        let node = e.new_object::<CIdvBranch>();
        let position = block(&mut e, &[1.0, 2.0, 3.0]);
        let start = block(&mut e, &[0.0; 3]);
        let direction = block(&mut e, &[0.0, 0.0, 2.0]);
        e.call(
            0x00b1_3c10,
            &args![
                branch,
                position,
                0.5f32,
                node,
                start,
                direction,
                0.75f32,
                3i32,
                Ptr::<StlVector>::new(leaves_at)
            ],
        );
        assert_eq!(
            *built.borrow(),
            vec![vec![leaf, position, 255, 0, 0.75f32.to_bits(), 3]]
        );
        assert_eq!(e.mem.u32(leaves_at + 0x10), data + 4);
        assert_eq!(e.mem.u32(data), leaf);
        assert_eq!(e.mem.u8(leaf + 0x40), 1);
        close(&floats(&e, leaf + 0x1C, 3), &[0.0, 0.0, 1.0]);
        close(&floats(&e, leaf + 0x28, 3), &[0.0, 0.0, 1.0]);
        close(&colours.borrow()[0], &[0.35, 0.45, 0.55]);
    }

    #[test]
    fn ring_vertices_are_emitted() {
        let mut e = engine();
        e.set_global(ANGLE_PERIOD, 4.0f32);
        e.set_global(RING_ANGLE_OFFSET, 0.0f32);
        e.set_global(0x0101_2070, 1.0f64);
        let this = e.new_object::<CIdvBranch>();
        let vertex = e.mem.alloc(0x48);
        put(&mut e, vertex + 0x0C, &[1.0, 2.0, 3.0]);
        e.mem.set_f32(vertex + 0x18, 2.0);
        put(&mut e, vertex + 0x28, &[1.0, 0.0, 0.0]);
        put(&mut e, vertex + 0x34, &[0.0, 1.0, 0.0]);
        let geometry = e.mem.alloc(0x40);
        e.mem.set_u8(geometry + 8, 1);
        e.mem.set_u16(geometry + 0x22, 5);
        let profile = block(&mut e, &[1.0, 2.0, 3.0]);
        let textures = sample(&mut e, 0x00b3_01f0, 1, 2);
        let points = sample(&mut e, 0x00b3_00c0, 1, 3);
        let colours = sample(&mut e, 0x00b3_0020, 1, 3);
        let weights = spy(&mut e, 0x00b3_0560);
        silence(&mut e, &[0x00b3_05f0, 0x00b3_0660]);
        e.call(
            0x00b1_4450,
            &args![
                this, vertex, 0.5f32, 0.0f32, 2i32, geometry, profile, 0.25f32, 7u8, 0.0f32, 1i32
            ],
        );
        close(&textures.borrow().concat(), &[1.5, 1.0, 2.0, 1.0, 2.5, 1.0]);
        let expected: Vec<f32> = [0.0f32, 2.0, 4.0]
            .iter()
            .flat_map(|a| [1.0 + 2.0 * a.cos(), 2.0 + 2.0 * a.sin(), 3.0])
            .collect();
        close(&points.borrow().concat(), &expected);
        assert_eq!(colours.borrow().concat(), vec![1.0; 9]);
        assert_eq!(
            *weights.borrow(),
            vec![vec![geometry, 0.25f32.to_bits(), 7]; 3]
        );
        assert_eq!(e.mem.u16(geometry + 0x22), 8);
    }

    #[test]
    fn flare_influence() {
        let mut e = engine();
        e.set_global(ANGLE_PERIOD, 6.0f32);
        e.set_global(0x010a_5494, 3.0f32);
        e.set_global(0x0101_2060, 0.0f64);
        e.register_double(POW, |_, a| dbl(a, 0).powf(dbl(a, 2)).into_ret());
        e.register_double(ABSOLUTE_VALUE, |_, a| dbl(a, 0).abs().into_ret());
        let flare = block(&mut e, &[1.0, 1.0, 2.0, 4.0, 1.0, 3.0]);
        let influence = |e: &mut Engine, flare_angle: f32, angle: f32, param: f32| {
            e.mem.set_f32(flare, flare_angle);
            e.call(0x00b1_4980, &args![flare, angle, param]).f32()
        };
        close(&[influence(&mut e, 1.0, 1.5, 1.0)], &[0.5625]);
        assert_eq!(influence(&mut e, 1.0, 3.0, 1.0), 0.0);
        close(&[influence(&mut e, 0.5, 5.8, 1.0)], &[0.2025]);
        close(&[influence(&mut e, 5.8, 0.5, 1.0)], &[0.2025]);
        assert_eq!(influence(&mut e, 1.0, 1.5, 4.0), 0.0);
    }

    #[test]
    fn marker_level_weight() {
        let mut e = engine();
        e.set_global(LEVEL_MARKER, 3i32);
        assert_eq!(
            e.call(0x00b1_4ae0, &args![0u32, 3i32, 0.5f32, 0.5f32])
                .f32(),
            0.75
        );
        assert_eq!(
            e.call(0x00b1_4ae0, &args![0u32, 2i32, 0.5f32, 0.5f32])
                .f32(),
            1.0
        );
    }

    #[test]
    fn blended_weight() {
        let mut e = engine();
        assert_eq!(
            e.call(0x00b1_4b10, &args![0u32, 2.0f32, 0.25f32]).f32(),
            1.5
        );
    }

    #[test]
    fn volume_sums_the_segments() {
        let mut e = engine();
        e.register_double(VEC3_DISTANCE, |e, a| {
            let d: f32 = (0..3)
                .map(|k| (e.mem.f32(a[0] + 4 * k) - e.mem.f32(a[1] + 4 * k)).powi(2))
                .sum();
            d.sqrt().into_ret()
        });
        let branch = e.new_object::<CIdvBranch>().addr();
        let vertices = e.mem.alloc(3 * 0x48);
        for (i, (position, radius)) in [
            ([0.0, 0.0, 0.0], 1.0),
            ([3.0, 4.0, 0.0], 2.0),
            ([3.0, 4.0, 12.0], 3.0),
        ]
        .iter()
        .enumerate()
        {
            put(&mut e, vertices + 0x48 * i as u32 + 0x0C, position);
            e.mem.set_f32(vertices + 0x48 * i as u32 + 0x18, *radius);
        }
        e.mem.set_u32(branch + 0x20, vertices);
        e.mem.set_i32(branch + 0x24, 3);
        e.mem.set_f32(branch + 0x30, 99.0);
        e.call(0x00b1_4b40, &args![Ptr::<CIdvBranch>::new(branch)]);
        assert_eq!(e.mem.f32(branch + 0x30), 75.0);
        // One vertex: unchanged.
        e.mem.set_i32(branch + 0x24, 1);
        e.mem.set_f32(branch + 0x30, 99.0);
        e.call(0x00b1_4b40, &args![Ptr::<CIdvBranch>::new(branch)]);
        assert_eq!(e.mem.f32(branch + 0x30), 99.0);
    }

    #[test]
    fn segment_search_by_running_length() {
        let mut e = engine();
        let branch = e.new_object::<CIdvBranch>().addr();
        let vertices = e.mem.alloc(4 * 0x48);
        for (i, length) in [0.0f32, 1.0, 3.0, 6.0].iter().enumerate() {
            e.mem.set_f32(vertices + 0x48 * i as u32 + 0x40, *length);
        }
        e.mem.set_u32(branch + 0x20, vertices);
        e.mem.set_i32(branch + 0x24, 4);
        let out = e.mem.alloc(8);
        let this = Ptr::<CIdvBranch>::new(branch);
        e.call(0x00b1_4be0, &args![this, Ptr::<()>::new(out), 2.0f32]);
        assert_eq!((e.mem.i32(out), e.mem.f32(out + 4)), (1, 0.5));
        // Past every vertex: the index stays 0.
        e.call(0x00b1_4be0, &args![this, Ptr::<()>::new(out), 7.0f32]);
        assert_eq!((e.mem.i32(out), e.mem.f32(out + 4)), (0, 7.0));
        // A single vertex: nothing is written.
        e.mem.set_i32(branch + 0x24, 1);
        e.mem.set_i32(out, 9);
        e.call(0x00b1_4be0, &args![this, Ptr::<()>::new(out), 2.0f32]);
        assert_eq!(e.mem.i32(out), 9);
    }

    #[test]
    fn flares_are_spread_around_the_branch() {
        let mut e = engine();
        e.set_global(ANGLE_PERIOD, 6.0f32);
        e.set_global(ANGLE_DIVISOR, 2.0f64);
        e.set_global(0x0102_9790, 32768.0f64);
        e.register_double(RAND, |_, _| 0i32.into_ret());
        e.register_double(0x00b1_6cd0, |e, a| {
            for i in 0..6 {
                let word = e.mem.u32(a[2] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, word);
            }
            Ret::default()
        });
        let this = e.new_object::<CIdvBranch>().addr();
        let data = vector(&mut e, this + 0x38, 0x18, 0, 4);
        let info = e.mem.alloc(0x74);
        e.mem.set_i32(info + 0x28, 2);
        for (offset, value) in [
            (0x2C, 0.5f32),
            (0x30, 0.4),
            (0x34, 0.1),
            (0x38, 3.0),
            (0x3C, 5.0),
            (0x40, 1.0),
            (0x44, 7.0),
            (0x48, 2.0),
            (0x4C, 9.0),
        ] {
            e.mem.set_f32(info + offset, value);
        }
        e.call(
            0x00b1_4ca0,
            &args![
                Ptr::<CIdvBranch>::new(this),
                Ptr::<SIdvBranchInfo>::new(info)
            ],
        );
        assert_eq!(e.mem.u32(this + 0x38 + 0x10), data + 0x30);
        close(&floats(&e, data, 6), &[0.0, 0.15, 3.0, 5.0, 9.0, 4.0]);
        close(
            &floats(&e, data + 0x18, 6),
            &[1.5, 0.15, 3.0, 5.0, 9.0, 4.0],
        );
        // No flares: nothing is added.
        e.mem.set_i32(info + 0x28, 0);
        e.call(
            0x00b1_4ca0,
            &args![
                Ptr::<CIdvBranch>::new(this),
                Ptr::<SIdvBranchInfo>::new(info)
            ],
        );
        assert_eq!(e.mem.u32(this + 0x38 + 0x10), data + 0x30);
    }

    #[test]
    fn random_float_between_the_limits() {
        let mut e = engine();
        e.set_global(0x0102_9790, 32768.0f64);
        e.register_double(RAND, |_, _| 16384i32.into_ret());
        assert_eq!(e.call(0x00b1_4e90, &args![1.0f32, 3.0f32]).f32(), 2.0);
    }

    #[test]
    fn ring_normals_cross_the_two_directions() {
        let mut e = engine();
        let branch = e.new_object::<CIdvBranch>().addr();
        e.mem.set_i32(branch + 0x24, 2);
        let geometry = e.mem.alloc(0x40);
        e.mem.set_u16(geometry + 0x22, 3);
        let table = e.mem.alloc(12 * 10);
        let ring = [
            (1.0f32, 0.0f32),
            (0.0, 1.0),
            (-1.0, 0.0),
            (0.0, -1.0),
            (1.0, 0.0),
        ];
        for r in 0..2 {
            for (p, (x, y)) in ring.iter().enumerate() {
                put(&mut e, table + 12 * (5 * r + p as u32), &[*x, *y, r as f32]);
            }
        }
        e.register_double(0x00b3_0f80, move |_, a| (table + 12 * a[1]).into_ret());
        let normals = sample(&mut e, 0x00b3_0190, 1, 3);
        e.call(
            0x00b1_4ed0,
            &args![Ptr::<CIdvBranch>::new(branch), geometry, 4u16],
        );
        let seen = normals.borrow();
        assert_eq!(seen.len(), 10);
        close(&seen[0], &[1.0, 0.0, 0.0]);
        close(&seen[1], &[0.0, 1.0, 0.0]);
        assert_eq!(e.mem.u16(geometry + 0x22), 13);
    }

    #[test]
    fn blended_percent_of_the_ancestors() {
        let mut e = engine();
        let root = e.new_object::<CIdvBranch>().addr();
        let child = e.new_object::<CIdvBranch>().addr();
        let grandchild = e.new_object::<CIdvBranch>().addr();
        e.mem.set_u32(child, root);
        e.mem.set_f32(child + 4, 0.5);
        e.mem.set_u32(grandchild, child);
        e.mem.set_f32(grandchild + 4, 0.5);
        let percent = |e: &mut Engine, branch: u32| e.call(0x00b1_5110, &args![branch]).f32();
        assert_eq!(percent(&mut e, 0), 0.0);
        assert_eq!(percent(&mut e, root), 0.0);
        assert_eq!(percent(&mut e, child), 0.5);
        assert_eq!(percent(&mut e, grandchild), 0.75);
    }

    #[test]
    fn byte_vector_index_is_checked() {
        let mut e = engine();
        let invalid = spy(&mut e, INVALID_PARAMETER);
        let at = 0x011f_8d00;
        let data = vector(&mut e, at, 1, 4, 4);
        let r = e
            .call(0x00b1_5170, &args![Ptr::<StlVector>::new(at), 2u32])
            .u32();
        assert_eq!((r, invalid.borrow().len()), (data + 2, 0));
        let r = e
            .call(0x00b1_5170, &args![Ptr::<StlVector>::new(at), 4u32])
            .u32();
        assert_eq!((r, invalid.borrow().len()), (data + 4, 1));
    }

    #[test]
    fn vector_destructor_destroys_then_frees_the_proxy() {
        let mut e = engine();
        silence(&mut e, &[0x00b1_fca0, OPERATOR_DELETE]);
        let at = 0x011f_8d00;
        e.mem.set_u32(at, 0x55);
        e.call_log = Some(Vec::new());
        e.call(0x00b1_51a0, &args![Ptr::<StlVector>::new(at)]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(0x00b1_fca0, vec![at]), (OPERATOR_DELETE, vec![0x55])]
        );
    }

    #[test]
    fn flare_vector_destructor_empties_then_frees_the_proxy() {
        let mut e = engine();
        let deleted = spy(&mut e, OPERATOR_DELETE);
        let at = 0x011f_8d00;
        e.mem.set_u32(at, 0x55);
        e.mem.set_u32(at + 0x0C, 0);
        e.mem.set_u32(at + 0x10, 0x99);
        e.mem.set_u32(at + 0x14, 0x99);
        e.call(0x00b1_5210, &args![Ptr::<StlVector>::new(at)]);
        assert_eq!(*deleted.borrow(), vec![vec![0x55]]);
        assert_eq!((e.mem.u32(at + 0x10), e.mem.u32(at + 0x14)), (0, 0));
    }

    #[test]
    fn flare_push_back() {
        let mut e = engine();
        let construct = spy(&mut e, 0x00b1_6cd0);
        let at = 0x011f_8d00;
        let data = vector(&mut e, at, 0x18, 1, 2);
        let value = e.mem.alloc(0x18);
        e.call(
            0x00b1_5280,
            &args![Ptr::<StlVector>::new(at), Ptr::<()>::new(value)],
        );
        assert_eq!(
            *construct.borrow(),
            vec![vec![data + 0x18, 1, value, at + 8, 0, 0]]
        );
        assert_eq!(e.mem.u32(at + 0x10), data + 0x30);
        // Full: the checked insert at the end.
        let insert = spy(&mut e, 0x00b1_57e0);
        silence(&mut e, &[0x00b1_6890]);
        e.call(
            0x00b1_5280,
            &args![Ptr::<StlVector>::new(at), Ptr::<()>::new(value)],
        );
        assert_eq!(
            *insert.borrow(),
            vec![vec![at, PROXY, data + 0x30, 1, value]]
        );
        assert_eq!(construct.borrow().len(), 1);
    }

    #[test]
    fn int_vector_push_back() {
        let mut e = engine();
        let construct = spy(&mut e, 0x00b3_6b10);
        let at = 0x011f_8d00;
        let data = vector(&mut e, at, 4, 1, 2);
        let value = e.mem.alloc(4);
        e.call(
            0x00b1_5390,
            &args![Ptr::<StlVector>::new(at), Ptr::<()>::new(value)],
        );
        assert_eq!(*construct.borrow(), vec![vec![data + 4, 1, value]]);
        assert_eq!(e.mem.u32(at + 0x10), data + 8);
        // Full: the checked insert at the end.
        let insert = spy(&mut e, 0x00b1_56b0);
        e.call(
            0x00b1_5390,
            &args![Ptr::<StlVector>::new(at), Ptr::<()>::new(value)],
        );
        let call = insert.borrow()[0].clone();
        assert_eq!(
            (call[0], call[2], call[3], call[4]),
            (at, PROXY, data + 8, value)
        );
        assert_eq!(construct.borrow().len(), 1);
    }

    #[test]
    fn flare_insert_returns_the_iterator_at_the_same_index() {
        let mut e = engine();
        let insert = spy(&mut e, 0x00b1_57e0);
        let invalid = spy(&mut e, INVALID_PARAMETER);
        e.register_double(0x00b1_6890, |e, a| {
            let position = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, position + a[1] * 0x18);
            Ret::default()
        });
        let at = 0x011f_8d00;
        let data = vector(&mut e, at, 0x18, 2, 2);
        let value = e.mem.alloc(0x18);
        let out = e.mem.alloc(8);
        let r = e
            .call(
                0x00b1_54d0,
                &args![
                    Ptr::<StlVector>::new(at),
                    Ptr::<()>::new(out),
                    PROXY,
                    data + 0x18,
                    Ptr::<()>::new(value)
                ],
            )
            .u32();
        assert_eq!(r, out);
        assert_eq!(
            *insert.borrow(),
            vec![vec![at, PROXY, data + 0x18, 1, value]]
        );
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (PROXY, data + 0x18));
        assert_eq!(invalid.borrow().len(), 0);
        // An iterator of another vector is invalid.
        e.call(
            0x00b1_54d0,
            &args![
                Ptr::<StlVector>::new(at),
                Ptr::<()>::new(out),
                0x1234u32,
                data,
                Ptr::<()>::new(value)
            ],
        );
        assert_eq!(invalid.borrow().len(), 1);
        // An empty vector inserts at index 0.
        let empty = 0x011f_8d40;
        let first = vector(&mut e, empty, 0x18, 0, 1);
        e.call(
            0x00b1_54d0,
            &args![
                Ptr::<StlVector>::new(empty),
                Ptr::<()>::new(out),
                PROXY,
                first,
                Ptr::<()>::new(value)
            ],
        );
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (PROXY, first));
    }

    #[test]
    fn flare_vector_release() {
        let mut e = engine();
        let deleted = spy(&mut e, OPERATOR_DELETE);
        let at = 0x011f_8d00;
        let data = vector(&mut e, at, 0x18, 2, 4);
        e.call(0x00b1_5600, &args![Ptr::<StlVector>::new(at)]);
        assert_eq!(*deleted.borrow(), vec![vec![data]]);
        assert_eq!(
            (
                e.mem.u32(at + 0x0C),
                e.mem.u32(at + 0x10),
                e.mem.u32(at + 0x14)
            ),
            (0, 0, 0)
        );
        // Without storage nothing is freed.
        e.call(0x00b1_5600, &args![Ptr::<StlVector>::new(at)]);
        assert_eq!(deleted.borrow().len(), 1);
    }

    #[test]
    fn checked_range_call() {
        let mut e = engine();
        let invalid = spy(&mut e, INVALID_PARAMETER);
        let range: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = range.clone();
        e.register_double(0x00b1_6b60, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            5u32.into_ret()
        });
        let r = e
            .call(0x00b1_6950, &args![PROXY, 0x100u32, PROXY, 0x110u32, 1u8])
            .u32();
        assert_eq!(r, 5);
        assert_eq!(*range.borrow(), vec![vec![0x100, 0x110, 4, 1]]);
        assert_eq!(invalid.borrow().len(), 0);
        e.call(
            0x00b1_6950,
            &args![PROXY, 0x100u32, 0x1234u32, 0x110u32, 0u8],
        );
        e.call(0x00b1_6950, &args![0u32, 0x100u32, 0u32, 0x110u32, 0u8]);
        assert_eq!(invalid.borrow().len(), 2);
    }

    #[test]
    fn int_vector_clear_erases_the_whole_range() {
        let mut e = engine();
        let erase = spy(&mut e, 0x00b3_35c0);
        let at = 0x011f_8d00;
        let data = vector(&mut e, at, 4, 3, 3);
        e.call(0x00b3_2350, &args![Ptr::<StlVector>::new(at)]);
        let call = erase.borrow()[0].clone();
        assert_eq!(
            (call[0], call[2], call[3], call[4], call[5]),
            (at, PROXY, data, PROXY, data + 12)
        );
    }

    #[test]
    fn compute_builds_a_single_segment_branch() {
        let mut e = engine();
        e.set_global(ANGLE_DIVISOR, 1.0f64);
        e.set_global(ANGLE_PERIOD, 6.0f32);
        e.set_global(LEVEL_MARKER, 5i32);
        e.set_global(0x0101_2070, 1.0f64);
        put(&mut e, BUD_AXIS, &[0.0, 1.0, 0.0]);
        put(&mut e, UP_VECTOR, &[0.0, 0.0, 1.0]);
        leaf_info_block(&mut e, 0);
        let info = e.mem.alloc(0x74);
        e.mem.set_u16(info, 2);
        e.mem.set_i32(info + 4, 1);
        e.mem.set_f32(info + 8, 2.0);
        e.mem.set_f32(info + 0x24, 1.0);
        for offset in [0x50, 0x54, 0x58, 0x5C, 0x60, 0x64, 0x68, 0x6C, 0x70] {
            e.mem.set_u32(info + offset, 0x1000 + offset);
        }
        let list = vector(&mut e, BRANCH_INFO_LIST, 4, 1, 1);
        e.mem.set_u32(list, info);
        e.register_double(FROND_FLAG, |_, _| false.into_ret());
        e.register_double(0x00b3_35c0, |_, _| Ret::default());
        e.register_double(SPLINE_EVALUATE, |_, _| 1.0f32.into_ret());
        e.register_double(GET_UNIFORM, |_, _| 0.0f32.into_ret());
        e.register_double(SCALED_VARIANCE, |_, _| 0.0f32.into_ret());
        e.register_double(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let strips = spy(&mut e, 0x00b2_fdc0);
        e.register_double(VEC3_DOT, |_, _| 0.0f32.into_ret());
        e.register_double(ACOS, |_, a| dbl(a, 0).acos().into_ret());
        e.register_double(POW, |_, a| dbl(a, 0).powf(dbl(a, 2)).into_ret());
        e.register_double(ABSOLUTE_VALUE, |_, a| dbl(a, 0).abs().into_ret());
        e.register_double(VEC3_DISTANCE, |_, _| 4.0f32.into_ret());
        silence(
            &mut e,
            &[
                0x00b3_01f0,
                0x00b3_05f0,
                0x00b3_0660,
                0x00b3_00c0,
                0x00b3_0020,
                0x00b3_0560,
            ],
        );
        let table = e.mem.alloc(12 * 6);
        for i in 0..6u32 {
            put(
                &mut e,
                table + 12 * i,
                &[i as f32, (i * i) as f32, 1.0 + i as f32],
            );
        }
        e.register_double(0x00b3_0f80, move |_, a| (table + 12 * a[1]).into_ret());
        let normals = sample(&mut e, 0x00b3_0190, 1, 3);
        let branch = e.new_object::<CIdvBranch>();
        let b = branch.addr();
        let position = block(&mut e, &[5.0, 6.0, 7.0]);
        let matrix = identity(&mut e);
        let direction = block(&mut e, &[0.0, 1.0, 0.0]);
        let geometry = e.mem.alloc(0x40);
        let leaves = e.mem.alloc(0x18);
        e.call(
            0x00b1_1050,
            &args![
                branch,
                7i32,
                10.0f32,
                0i32,
                position,
                0.5f32,
                0.0f32,
                matrix,
                direction,
                geometry,
                Ptr::<StlVector>::new(leaves),
                0.25f32,
                2i32,
                0.0f32
            ],
        );
        assert_eq!(e.mem.f32(b + 4), 0.5);
        assert_eq!((e.mem.i32(b + 0x24), e.mem.u16(b + 0x28)), (2, 2));
        let vertices = e.mem.u32(b + 0x20);
        assert_ne!(vertices, 0);
        assert_eq!(e.mem.f32(vertices + 0x40), 0.0);
        assert_eq!(e.mem.f32(vertices + 0x48 + 0x40), 10.0);
        close(&floats(&e, vertices + 0x0C, 3), &[5.0, 6.0, 7.0]);
        assert_eq!(e.mem.f32(b + 0x30), 80.0);
        assert_eq!(e.mem.f32(geometry + 0x18), 20.0);
        let call = strips.borrow()[0].clone();
        assert_eq!((call[0], call[1], call[3]), (geometry, 0, 8));
        let indices: Vec<u16> = (0..8).map(|i| e.mem.u16(call[2] + 2 * i)).collect();
        assert_eq!(indices, vec![3, 0, 4, 1, 5, 2, 3, 3]);
        assert_eq!(e.mem.u16(geometry + 0x26), 1);
        assert_eq!(normals.borrow().len(), 6);
        assert_eq!(e.mem.u16(geometry + 0x22), 6);
    }
}
