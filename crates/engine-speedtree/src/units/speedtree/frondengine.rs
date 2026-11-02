//! `SpeedTree/frondengine.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! SpeedTreeRT's frond engine: `CFrondEngine` reads the frond settings of a tree file
//! (`Parse`), collects the guides the branch builder hands it (`BeginGuide`, `AddPoint`,
//! `EndGuide`), reduces them to level-of-detail guide lists (`BuildGuideLods`) and writes the
//! blade or cross-section geometry of every level into the tree's `CIndexedGeometry`
//! (`Compute`). The unit also holds the MSVC `std::vector` instantiations for its own element
//! types: `SFrondGuide` (0x38 bytes), `SFrondVertex` (0x38 bytes), `SFrondTexture` (0x2C bytes),
//! the vectors of guide vectors and of float vectors (0x18 bytes) and the `CIdvBezierSpline`
//! vectors (`CVec` and `float`).
//!
//! The PC build's `std::vector` is 0x18 bytes ([`StlVector`]), so every class holding one is 8
//! bytes per vector larger than in the Xbox PDB. A checked iterator is two words, the vector's
//! proxy and the element address; the checked accesses call the invalid-parameter path
//! (`00ec7c56`) when an index or iterator is out of range.
//!
//! Functions that carry an `FS:[0]` exception frame or call the `/GS` cookie check are marked
//! `Translation incomplete` (the unwinding handlers are not translated).

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{
    checked_iterator, copy_float, copy_words, ftol, ld, scaled_trig, st, vector_at, vector_len,
    CRotTransform, CVec3, StlVector, COS, INVALID_PARAMETER, MEMCPY, OPERATOR_DELETE, OPERATOR_NEW,
    SIN,
};
use super::indexedgeometry::CIndexedGeometry;

/// The `std::string` helpers the file uses: `_Tidy(built, new size)` (`0044a730`; `(0, 0)` makes
/// the string empty, `(1, 0)` destroys it), the constructor from a `char *` (`0044a4c0`), the
/// copy constructor (`0044a500`) and `assign(source, offset, count)` (`0044a580`).
const STRING_TIDY: u32 = 0x0044_a730;
const STRING_FROM_CHARS: u32 = 0x0044_a4c0;
const STRING_COPY: u32 = 0x0044_a500;
const STRING_ASSIGN: u32 = 0x0044_a580;
/// `[010173f0]`: the `npos` the string assignments pass as their count.
const STRING_NPOS: u32 = 0x0101_73f0;
/// `CTreeFileAccess::ParseInt`, `ParseFloat` (the value in `ST0`), `ParseString` (into the
/// string at its argument), the boolean parse (`00b06180`) and `ParseBranchParameter`.
const PARSE_INT: u32 = 0x00b3_ed80;
const PARSE_FLOAT: u32 = 0x00b3_ed20;
const PARSE_STRING: u32 = 0x00b3_ee40;
const PARSE_BOOL: u32 = 0x00b0_6180;
const PARSE_BRANCH_PARAMETER: u32 = 0x00b1_3970;
/// `CIdvFilename::NoPath(this, result)` (`00b09f90`).
const FILENAME_NO_PATH: u32 = 0x00b0_9f90;
/// The exception object constructor (`00b09480`: this, message string, flag), the base
/// exception constructor (`00ec78e0`) and `_CxxThrowException` (`00ec7894`; it does not return).
const EXCEPTION_CONSTRUCT: u32 = 0x00b0_9480;
const BASE_EXCEPTION_CONSTRUCT: u32 = 0x00ec_78e0;
const THROW: u32 = 0x00ec_7894;
/// The throw information of the file's `throw` statements (`01170384`) and of the "frond
/// vertices exceed" exception (`0117105c`), and that exception's vtable (`010a307c`).
const THROW_INFO: u32 = 0x0117_0384;
const OVERFLOW_THROW_INFO: u32 = 0x0117_105c;
const OVERFLOW_EXCEPTION_VTABLE: u32 = 0x010a_307c;
/// `vsprintf` (`00ec6cc4`).
const VSPRINTF: u32 = 0x00ec_6cc4;
/// `CIdvBezierSpline`: the constructor from a string (`00b37660`), the destructor (`00b37880`)
/// and `Evaluate` (`00b37ad0`, `this` = the spline, one `float`, result in `ST0`).
const SPLINE_CONSTRUCT: u32 = 0x00b3_7660;
const SPLINE_DESTRUCT: u32 = 0x00b3_7880;
const SPLINE_EVALUATE: u32 = 0x00b3_7ad0;
/// The proxy allocation of the checked-iterator `std::vector` constructor (`0044d670`, `cdecl`,
/// `(1, 0)`).
const ALLOCATE_PROXY: u32 = 0x0044_d670;
/// The length error the `std::vector` code raises for a request above the maximum size (it does
/// not return) and the element allocator of the texture vector (`00b23b50`, `cdecl`, count then
/// 0).
const LENGTH_ERROR: u32 = 0x00b0_fb80;
const ALLOCATE_TEXTURES: u32 = 0x00b2_3b50;
/// `CIdvRandom::CIdvRandom` and `GetUniform` (`this` is not read).
const RANDOM_CONSTRUCT: u32 = 0x00b3_f100;
const GET_UNIFORM: u32 = 0x00b3_f130;
/// The `CVec3` helpers outside this file: the sum `this + other` (`00b0cf90`), the product
/// `this * scalar` (`00b0cff0`), the difference `this - other` (`00b13040`), `Normalize`
/// (`00b12790`), the cross product (`00b12810`), the product with a rotation (`00b12890`), the
/// approximate length (`00b14230`, `ST0`) and `Distance` (`00b04d40`, `ST0`).
const VEC3_ADD: u32 = 0x00b0_cf90;
const VEC3_SCALE: u32 = 0x00b0_cff0;
const VEC3_SUBTRACT: u32 = 0x00b1_3040;
const VEC3_NORMALIZE: u32 = 0x00b1_2790;
const VEC3_CROSS: u32 = 0x00b1_2810;
const VEC3_TIMES_ROTATION: u32 = 0x00b1_2890;
const VEC3_LENGTH: u32 = 0x00b1_4230;
const VEC3_DISTANCE: u32 = 0x00b0_4d40;
/// The `std::vector<CVec3>` helpers: constructor (`00b1e3e0`), `push_back` (`00b1e4c0`) and the
/// tidy (`00b1fca0`); the `std::vector<float>` constructor (`00449fd0`), `push_back`
/// (`006578e0`), `operator[]` (`006578b0`) and tidy (`0065f240`); the proxy release of those
/// vectors (`0065d6e0`, `00b39460`: `operator delete` of the word at `this`).
const CVEC3_VECTOR_CONSTRUCT: u32 = 0x00b1_e3e0;
const CVEC3_VECTOR_PUSH: u32 = 0x00b1_e4c0;
const CVEC3_VECTOR_TIDY: u32 = 0x00b1_fca0;
const FLOAT_VECTOR_CONSTRUCT: u32 = 0x0044_9fd0;
const FLOAT_VECTOR_PUSH: u32 = 0x0065_78e0;
const FLOAT_VECTOR_AT: u32 = 0x0065_78b0;
const FLOAT_VECTOR_TIDY: u32 = 0x0065_f240;
const FLOAT_VECTOR_RELEASE: u32 = 0x0065_d6e0;
const PROXY_RELEASE: u32 = 0x00b3_9460;
/// The vectors of float vectors: constructor (`00b31270`, also the guide-lod vector's) and
/// tidy (`00b1fe80`). The guide vector's tidy (`00b1ec70`), the guide's own vertex-vector tidy
/// (`00b1e970`), the `SFrondGuide` copy constructor (`00b1ff50`) and assignment (`00b249d0`).
const VECTOR_VECTOR_CONSTRUCT: u32 = 0x00b3_1270;
const FLOAT_VECTOR_VECTOR_TIDY: u32 = 0x00b1_fe80;
const GUIDE_VECTOR_TIDY: u32 = 0x00b1_ec70;
const VERTEX_VECTOR_TIDY: u32 = 0x00b1_e970;
const GUIDE_COPY_CONSTRUCT: u32 = 0x00b1_ff50;
const GUIDE_ASSIGN: u32 = 0x00b2_49d0;
/// `CIndexedGeometry` members this file calls: `SetNumLodLevels` (`00b2fb30`), the strip
/// registration (`00b2fdc0`), `ResetStripCounter` (`00b306d0`), the vertex coordinate
/// (`00b300c0`), texture coordinate (`00b301f0`), colour (`00b30020`), normal (`00b30190`),
/// tangent (`00b305f0`), binormal (`00b30660`) and wind weight (`00b30560`) adders, and the
/// address of the coordinates (`00b30f80`) and texture coordinates (`00b30fd0`) of a vertex.
const GEOMETRY_SET_NUM_LOD_LEVELS: u32 = 0x00b2_fb30;
const GEOMETRY_ADD_STRIP: u32 = 0x00b2_fdc0;
const GEOMETRY_RESET_STRIP_COUNTER: u32 = 0x00b3_06d0;
const GEOMETRY_ADD_COORDINATE: u32 = 0x00b3_00c0;
const GEOMETRY_ADD_TEX_COORD: u32 = 0x00b3_01f0;
const GEOMETRY_ADD_COLOR: u32 = 0x00b3_0020;
const GEOMETRY_ADD_NORMAL: u32 = 0x00b3_0190;
const GEOMETRY_ADD_TANGENT: u32 = 0x00b3_05f0;
const GEOMETRY_ADD_BINORMAL: u32 = 0x00b3_0660;
const GEOMETRY_ADD_WIND_WEIGHT: u32 = 0x00b3_0560;
const GEOMETRY_COORD_POINTER: u32 = 0x00b3_0f80;
const GEOMETRY_TEX_COORD_POINTER: u32 = 0x00b3_0fd0;
/// `00b31070`: sets the texture coordinates of one vertex of a geometry (this = geometry, the
/// texture index and the eight floats).
const GEOMETRY_SET_TEX_COORDS: u32 = 0x00b3_1070;
/// The constants the file reads (all by address, as the exe does): `1.0` and `0.0` (`double`s),
/// `0.5` (`float`), the `double` the length and radius are scaled by (`01011588`), `180.0`-style
/// divisor base (`double`), the mirroring factor (`double`), the minimum normal length (`float`),
/// the texture-coordinate scale (`float`), the angle offset (`float`), the upper bound of the
/// random texture angle (`float`) and the default large-retention percent (`float`).
const ONE_DOUBLE: u32 = 0x0101_2070;
const ZERO_DOUBLE: u32 = 0x0101_2060;
const HALF_FLOAT: u32 = 0x0101_6248;
const SCALE_DOUBLE: u32 = 0x0101_1588;
const BLADE_ANGLE_NUMERATOR: u32 = 0x0104_ed58;
const MIRROR_FACTOR: u32 = 0x0101_a6b0;
const MINIMUM_NORMAL_LENGTH: u32 = 0x010a_54ec;
const TEXTURE_SCALE: u32 = 0x010a_54e8;
const PROFILE_ANGLE_OFFSET: u32 = 0x011f_8c9c;
const RANDOM_UPPER_BOUND: u32 = 0x0101_7130;
const LARGE_RETENTION_DEFAULT: u32 = 0x0101_81bc;
/// The text of the default profile spline (`010a54f0`) and the messages (`010a558c`,
/// `010a55ac`, `010a55dc`, `010a55f8`).
const DEFAULT_PROFILE_TEXT: u32 = 0x010a_54f0;
const MALFORMED_FROND_MESSAGE: u32 = 0x010a_558c;
const MALFORMED_TEXTURE_MESSAGE: u32 = 0x010a_55ac;
const VERTICES_EXCEED_MESSAGE: u32 = 0x010a_55dc;
const DEFAULT_REACHED_MESSAGE: u32 = 0x010a_55f8;
/// The CRT `atan2` (`00ec9760`; `cdecl`, two `double`s, the result in `ST0`).
const ATAN2: u32 = 0x00ec_9760;
/// `CRotTransform::CRotTransform` (the identity, `00b12af0`) and the matrix product
/// `this * other` (`00b12940`: this, a result slot, the other matrix).
const CROT_TRANSFORM_CONSTRUCT: u32 = 0x00b1_2af0;
const CROT_TRANSFORM_MULTIPLY: u32 = 0x00b1_2940;
/// The destructor the linker shares between the empty classes (`CVec`, `CIdvRandom`): it does
/// nothing (`00aa7830`).
const TRIVIAL_DESTRUCTOR: u32 = 0x00aa_7830;
/// The checked-iterator helpers of the guide vectors: advance an iterator by a count
/// (`00b23460`: the iterator, the count), whether two iterators differ (`00b16830`) and
/// the sort of a range (`00b23cd0`: first, last, element count, an empty comparison object).
const ITERATOR_ADVANCE: u32 = 0x00b2_3460;
const ITERATORS_DIFFER: u32 = 0x00b1_6830;
const GUIDE_SORT: u32 = 0x00b2_3cd0;
/// The `std::vector<CVec>` helpers of `operator=` (`00b1d130`): clear (`00b1e750`), assign over
/// a range (`00b236a0`), construct a range at the end (`00b25a90`) and reserve (`00b1f590`).
const VECTOR_CLEAR: u32 = 0x00b1_e750;
const VECTOR_ASSIGN_RANGE: u32 = 0x00b2_36a0;
const VECTOR_CONSTRUCT_RANGE: u32 = 0x00b2_5a90;
const VECTOR_RESERVE: u32 = 0x00b1_f590;
/// The `std::vector<float>` helpers of `operator=` (`00b1d5d0`): clear (`0044a0a0`), copy a range
/// (`0044d600`), construct a range at the end (`0060c320`) and reserve (`0044a240`).
const FLOAT_VECTOR_CLEAR: u32 = 0x0044_a0a0;
const FLOAT_COPY_RANGE: u32 = 0x0044_d600;
const FLOAT_CONSTRUCT_RANGE: u32 = 0x0060_c320;
const FLOAT_RESERVE: u32 = 0x0044_a240;
/// The guide-vector helpers: move a range down over an erased one (`00b23760`: the first
/// element kept, the end, the destination), move one up (`00b238e0`: first, last, the end of the
/// destination), destroy a range (`00b1ebc0`: this, first, last), the same for the guide-lod
/// vector (`00b20c30`), allocate storage (`00b23980`: count, 0), copy a range to uninitialised
/// storage (`00b26450` and `00b24030`: first, last, destination, the allocator), construct
/// copies of a value (`00b24180`: destination, count, value, the allocator and two flag bytes)
/// and assign one guide to another (`00b249d0`).
const GUIDE_MOVE_DOWN: u32 = 0x00b2_3760;
const GUIDE_MOVE_UP: u32 = 0x00b2_38e0;
const GUIDE_DESTROY_RANGE: u32 = 0x00b1_ebc0;
const GUIDE_LOD_DESTROY_RANGE: u32 = 0x00b2_0c30;
const GUIDE_ALLOCATE: u32 = 0x00b2_3980;
const GUIDE_RELOCATE_COPY: u32 = 0x00b2_6450;
const GUIDE_TAIL_COPY: u32 = 0x00b2_4030;
const GUIDE_FILL: u32 = 0x00b2_4180;
/// The texture vector's move-down helper (`00b23830`, like `GUIDE_MOVE_DOWN`).
const TEXTURE_MOVE_DOWN: u32 = 0x00b2_3830;
/// The most elements the guide vector (`0xFFFFFFFF / 0x38`) and the texture vector
/// (`0xFFFFFFFF / 0x2C`) can hold.
const GUIDE_LIMIT: u32 = 0x0492_4924;
const TEXTURE_LIMIT: u32 = 0x05d1_745d;

layout! {
    /// `CFrondEngine` (Xbox PDB), 0x84 bytes on the PC: the three `std::vector`s are 8 bytes
    /// larger than on the Xbox (0x18 instead of 0x10), so the offsets after `m_vGuides` are 8
    /// higher than in the PDB, and 0x10 higher after `m_vGuideLods`.
    pub struct CFrondEngine: 0x84 {
        /// `m_pGeometry` (Xbox PDB).
        0x00 m_pGeometry: Ptr<CIndexedGeometry>,
        /// `m_pLightingEngine` (Xbox PDB).
        0x04 m_pLightingEngine: Ptr,
        /// `m_vGuides` (Xbox PDB `+0x08`): a `std::vector<SFrondGuide>`.
        0x08 m_vGuides: Inline<StlVector>,
        /// `m_vGuideLods` (Xbox PDB `+0x18`): a vector of `std::vector<SFrondGuide>`.
        0x20 m_vGuideLods: Inline<StlVector>,
        /// `m_eFrondType` (Xbox PDB `+0x28`).
        0x38 m_eFrondType: u32,
        /// `m_nNumBlades` (Xbox PDB `+0x2C`).
        0x3C m_nNumBlades: u32,
        /// `m_pProfile` (Xbox PDB `+0x30`): a `CIdvBezierSpline *`.
        0x40 m_pProfile: Ptr,
        /// `m_nProfileSegments` (Xbox PDB `+0x34`).
        0x44 m_nProfileSegments: u32,
        /// `m_nLevel` (Xbox PDB `+0x38`).
        0x48 m_nLevel: u32,
        /// `m_bEnabled` (Xbox PDB `+0x3C`).
        0x4C m_bEnabled: u8,
        /// `m_vTextures` (Xbox PDB `+0x40`): a `std::vector<SFrondTexture>`.
        0x50 m_vTextures: Inline<StlVector>,
        /// `m_nNumLodLevels` (Xbox PDB `+0x50`).
        0x68 m_nNumLodLevels: u32,
        /// `m_fMaxSurfaceAreaPercent` (Xbox PDB `+0x54`).
        0x6C m_fMaxSurfaceAreaPercent: f32,
        /// `m_fMinSurfaceAreaPercent` (Xbox PDB `+0x58`).
        0x70 m_fMinSurfaceAreaPercent: f32,
        /// `m_fReductionFuzziness` (Xbox PDB `+0x5C`).
        0x74 m_fReductionFuzziness: f32,
        /// `m_fLargeRetentionPercent` (Xbox PDB `+0x60`).
        0x78 m_fLargeRetentionPercent: f32,
        /// `m_nMinLengthSegments` (Xbox PDB `+0x64`).
        0x7C m_nMinLengthSegments: u32,
        /// `m_nMinCrossSegments` (Xbox PDB `+0x68`).
        0x80 m_nMinCrossSegments: u32,
    }

    /// `SFrondVertex` (Xbox PDB), 0x38 bytes (no `std::vector` inside, so the same on the PC).
    pub struct SFrondVertex: 0x38 {
        /// `m_cPos` (Xbox PDB).
        0x00 m_cPos: Inline<CVec3>,
        /// `m_cTrans` (Xbox PDB).
        0x0C m_cTrans: Inline<CRotTransform>,
        /// `m_fCrossSectionWeight` (Xbox PDB).
        0x30 m_fCrossSectionWeight: f32,
        /// `m_nWindGroup` (Xbox PDB).
        0x34 m_nWindGroup: i32,
    }

    /// `SFrondGuide` (Xbox PDB), 0x38 bytes on the PC (its vertex vector is 8 bytes larger).
    pub struct SFrondGuide: 0x38 {
        /// `m_vVertices` (Xbox PDB): a `std::vector<SFrondVertex>`.
        0x00 m_vVertices: Inline<StlVector>,
        /// `m_fLength` (Xbox PDB `+0x10`).
        0x18 m_fLength: f32,
        /// `m_fRadius` (Xbox PDB `+0x14`).
        0x1C m_fRadius: f32,
        /// `m_chFrondMapIndex` (Xbox PDB `+0x18`).
        0x20 m_chFrondMapIndex: u8,
        /// `m_fOffsetAngle` (Xbox PDB `+0x1C`).
        0x24 m_fOffsetAngle: f32,
        /// `m_fSurfaceArea` (Xbox PDB `+0x20`).
        0x28 m_fSurfaceArea: f32,
        /// `m_fFuzzySurfaceArea` (Xbox PDB `+0x24`).
        0x2C m_fFuzzySurfaceArea: f32,
        /// `m_nVertexStartIndex` (Xbox PDB `+0x28`).
        0x30 m_nVertexStartIndex: u32,
        /// `m_nVerticesPerGuideVertex` (Xbox PDB `+0x2C`).
        0x34 m_nVerticesPerGuideVertex: u32,
    }

    /// `SFrondTexture` (Xbox PDB), 0x2C bytes: a `CIdvFilename` (a 0x1C-byte `std::string`) at
    /// `+0`, then the floats below.
    pub struct SFrondTexture: 0x2C {
        /// `m_fAspectRatio` (Xbox PDB).
        0x1C m_fAspectRatio: f32,
        /// `m_fSizeScale` (Xbox PDB).
        0x20 m_fSizeScale: f32,
        /// `m_fMinAngleOffset` (Xbox PDB).
        0x24 m_fMinAngleOffset: f32,
        /// `m_fMaxAngleOffset` (Xbox PDB).
        0x28 m_fMaxAngleOffset: f32,
    }
}

/// `FISTP qword` with the truncating rounding control (the exe sets the control word's rounding
/// bits to 11 around it), low dword: the "integer indefinite" `0x8000000000000000` of a NaN or
/// an out-of-range value has a zero low word.
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

/// The capacity of a `std::vector` whose elements are `size` bytes: zero without storage, else
/// `(end - first) / size` as the compiler's signed `IDIV`.
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

/// The `push_back` of the exe's `std::vector` instantiations whose elements are `size` bytes:
/// with spare capacity the element at `value` is constructed at the end by `construct`
/// (`cdecl`: destination, 1, value, the allocator at `this + 8`, two flag bytes the exe leaves
/// uninitialised and zero, passed here as zeros) and the end advanced by `size`; otherwise the
/// checked insert `insert` (`this`, an out iterator, the iterator at the end, the value) is used.
fn push_back_element(
    e: &mut Engine,
    this: Ptr<StlVector>,
    value: Ptr,
    size: i32,
    construct: u32,
    insert: u32,
) {
    let base = this.addr();
    let count = vector_len(e, base, size);
    let capacity = vector_capacity(e, base, size);
    if count < capacity {
        let last = e.mem.u32(base + 0x10);
        e.call(construct, &args![last, 1u32, value, base + 8, 0u32, 0u32]);
        e.mem.set_u32(base + 0x10, last.wrapping_add(size as u32));
    } else {
        let last = e.mem.u32(base + 0x10);
        let proxy = checked_iterator(e, base, last);
        e.with_stack(8, |e, out| {
            e.call(insert, &args![this, out, proxy, last, value]);
        });
    }
}

/// The checked insert of one element before the iterator (`proxy`, `position`) in the exe's
/// `std::vector` instantiations whose elements are `size` bytes: remembers the index of the
/// position (0 for an empty vector, otherwise after checking that the iterator belongs to the
/// vector), inserts with `insert_n` (`this`, proxy, position, 1, value), then writes to `out`
/// the iterator (proxy, first element) advanced by that index (`advance`: the iterator, the
/// count). Returns `out`.
#[allow(clippy::too_many_arguments)]
fn insert_one(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: Ptr,
    size: i32,
    insert_n: u32,
    advance: u32,
) -> Ptr {
    let base = this.addr();
    let index = if vector_len(e, base, size) == 0 {
        0
    } else {
        let first = e.mem.u32(base + 0x0C);
        let iterator_proxy = checked_iterator(e, base, first);
        if proxy == 0 || proxy != iterator_proxy {
            e.call(INVALID_PARAMETER, &args![]);
        }
        (position.wrapping_sub(first) as i32) / size
    };
    e.call(insert_n, &args![this, proxy, position, 1u32, value]);
    let first = e.mem.u32(base + 0x0C);
    let first_proxy = checked_iterator(e, base, first);
    e.with_stack(8, |e, iterator| {
        e.mem.set_u32(iterator.addr(), first_proxy);
        e.mem.set_u32(iterator.addr() + 4, first);
        e.call(advance, &args![iterator, index as u32]);
        let proxy_out = e.mem.u32(iterator.addr());
        let position_out = e.mem.u32(iterator.addr() + 4);
        e.mem.set_u32(out.addr(), proxy_out);
        e.mem.set_u32(out.addr() + 4, position_out);
    });
    out
}

/// The `clear` of the exe's `std::vector` instantiations: erases the whole range with `erase`
/// (`this`, an out iterator, the iterators at the first and at the last element).
fn erase_all(e: &mut Engine, this: Ptr<StlVector>, erase: u32) {
    let base = this.addr();
    let last = e.mem.u32(base + 0x10);
    let last_proxy = checked_iterator(e, base, last);
    let first = e.mem.u32(base + 0x0C);
    let first_proxy = checked_iterator(e, base, first);
    e.with_stack(8, |e, out| {
        e.call(
            erase,
            &args![this, out, first_proxy, first, last_proxy, last],
        );
    });
}

/// The checked-iterator `std::vector` constructor: allocates the proxy (`0044d670`), points its
/// first word back at the vector (none when the allocation fails), stores it as the vector's
/// proxy and calls `reserve` (`this`, 0). Returns `this`.
fn construct_vector(e: &mut Engine, this: Ptr<StlVector>, reserve: u32) -> Ptr<StlVector> {
    let proxy = e.call(ALLOCATE_PROXY, &args![1u32, 0u32]).u32();
    if proxy != 0 {
        e.mem.set_u32(proxy, this.addr());
    }
    e.mem.set_u32(this.addr(), proxy);
    e.call(reserve, &args![this, 0u32]);
    this
}

/// Reads the iterator (proxy at `+0`, position at `+4`) at `iterator`.
fn iterator_words(e: &Engine, iterator: u32) -> (u32, u32) {
    (e.mem.u32(iterator), e.mem.u32(iterator + 4))
}

/// Formats `format` with the word `value` into a string (`IdvFormatString`), builds the
/// exception object from it (`00b09480`) and throws it (`_CxxThrowException`); the throw does
/// not return in the exe.
fn throw_formatted(e: &mut Engine, format: u32, value: u32) {
    // The message string at +0, the 0x28-byte exception 00b09480 builds (std::exception and
    // its string at +0xc) at +0x20, so the frame runs to +0x48.
    e.with_stack(0x48, |e, frame| {
        let message = frame;
        let object: Ptr = Ptr::new(frame.addr() + 0x20);
        let text = idv_format_string(e, message, Ptr::new(format), &[value]);
        e.call(EXCEPTION_CONSTRUCT, &args![object, text, 0u32]);
        e.call(THROW, &args![object, THROW_INFO]);
    });
}

/// Throws the "default reached in CFrondEngine::Compute()" exception: the message string is
/// built from its text (`0044a4c0`), the exception object from it (`00b09480`) and thrown; the
/// throw does not return in the exe.
fn throw_default_reached(e: &mut Engine) {
    // The message string at +0, the 0x28-byte exception 00b09480 builds (std::exception and
    // its string at +0xc) at +0x20, so the frame runs to +0x48.
    e.with_stack(0x48, |e, frame| {
        let message = frame;
        let object: Ptr = Ptr::new(frame.addr() + 0x20);
        e.call(STRING_FROM_CHARS, &args![message, DEFAULT_REACHED_MESSAGE]);
        e.call(EXCEPTION_CONSTRUCT, &args![object, message, 0u32]);
        e.call(THROW, &args![object, THROW_INFO]);
    });
}

// Translated from 00b17fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b17fd0 - the FS:[0] exception frame (unwinding of the members
// already constructed if a later step throws) is not translated.
/// `CFrondEngine::CFrondEngine` (Xbox PDB): clears the geometry and lighting-engine pointers,
/// constructs the guide vector (`00b1d8a0`), the guide-lod vector (`00b31270`) and the texture
/// vector (`00b1e090`), and sets the defaults: frond type 1, 2 blades, 4 profile segments,
/// level 1, disabled, 4 LOD levels, a maximum surface-area percent of 1, minimum percent and
/// fuzziness 0, a large-retention percent copied from `010181bc`, 2 minimum length segments and
/// 1 minimum cross-section segment. The profile is a new 0x84-byte spline constructed from the
/// string at `010a54f0` (none when the allocation fails). Returns `this`.
pub fn c_frond_engine_c_frond_engine(e: &mut Engine, this: Ptr<CFrondEngine>) -> Ptr<CFrondEngine> {
    let base = this.addr();
    e.set(this, CFrondEngine::m_pGeometry, Ptr::NULL);
    e.set(this, CFrondEngine::m_pLightingEngine, Ptr::NULL);
    fn_00b1d8a0(e, Ptr::new(base + 8));
    e.call(VECTOR_VECTOR_CONSTRUCT, &args![base + 0x20]);
    e.set(this, CFrondEngine::m_eFrondType, 1);
    e.set(this, CFrondEngine::m_nNumBlades, 2);
    e.set(this, CFrondEngine::m_nProfileSegments, 4);
    e.set(this, CFrondEngine::m_nLevel, 1);
    e.set(this, CFrondEngine::m_bEnabled, 0);
    fn_00b1e090(e, Ptr::new(base + 0x50));
    e.set(this, CFrondEngine::m_nNumLodLevels, 4);
    e.set(this, CFrondEngine::m_fMaxSurfaceAreaPercent, 1.0);
    e.set(this, CFrondEngine::m_fMinSurfaceAreaPercent, 0.0);
    e.set(this, CFrondEngine::m_fReductionFuzziness, 0.0);
    copy_float(e, base + 0x78, LARGE_RETENTION_DEFAULT);
    e.set(this, CFrondEngine::m_nMinLengthSegments, 2);
    e.set(this, CFrondEngine::m_nMinCrossSegments, 1);
    let memory = e.call(OPERATOR_NEW, &args![0x84u32]).u32();
    e.with_stack(0x1C, |e, text| {
        let spline = if memory != 0 {
            let text = e
                .call(STRING_FROM_CHARS, &args![text, DEFAULT_PROFILE_TEXT])
                .u32();
            e.call(SPLINE_CONSTRUCT, &args![memory, text]).u32()
        } else {
            0
        };
        e.set(this, CFrondEngine::m_pProfile, Ptr::new(spline));
        if memory != 0 {
            e.call(STRING_TIDY, &args![text, 1u32, 0u32]);
        }
    });
    this
}

// Translated from 00b181b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b181b0 - the FS:[0] exception frame (cleanup states of the member
// destructors) is not translated.
/// The destructor of the frond engine (the engine map gives it no name): destroys the profile
/// spline (`00b37880`, then `operator delete`; the delete flag is the constant 1) and clears the
/// pointer, then tidies and frees the proxies of the texture vector (`00b1f9e0`), the guide-lod
/// vector (`00b1f650`) and the guide vector (`00b1ec70`).
pub fn fn_00b181b0(e: &mut Engine, this: Ptr<CFrondEngine>) {
    let base = this.addr();
    let spline = e.get(this, CFrondEngine::m_pProfile).addr();
    if spline != 0 {
        e.call(SPLINE_DESTRUCT, &args![spline]);
        // `MOV EAX,1; AND EAX,1; JZ`: the delete flag is always set.
        e.call(OPERATOR_DELETE, &args![spline]);
    }
    e.set(this, CFrondEngine::m_pProfile, Ptr::NULL);
    fn_00b1f9e0(e, Ptr::new(base + 0x50));
    let proxy = e.mem.u32(base + 0x50);
    e.call(OPERATOR_DELETE, &args![proxy]);
    fn_00b1f650(e, Ptr::new(base + 0x20));
    let proxy = e.mem.u32(base + 0x20);
    e.call(OPERATOR_DELETE, &args![proxy]);
    e.call(GUIDE_VECTOR_TIDY, &args![base + 8]);
    let proxy = e.mem.u32(base + 8);
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00b18300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `m_nLevel` (`+0x48`) of the frond engine (the engine map gives it no name).
pub fn fn_00b18300(e: &mut Engine, this: Ptr<CFrondEngine>) -> u32 {
    e.get(this, CFrondEngine::m_nLevel)
}

// Translated from 00b18320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns `m_bEnabled` (`+0x4C`) of the frond engine (the engine map gives it no name).
pub fn fn_00b18320(e: &mut Engine, this: Ptr<CFrondEngine>) -> u8 {
    e.get(this, CFrondEngine::m_bEnabled)
}

// Translated from 00b18340 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b18340 - the FS:[0] exception frame (cleanup of the temporary
// strings if a token read throws) and the /GS cookie check (00ec408c) are not translated.
/// `CFrondEngine::Parse` (Xbox PDB): reads tokens from the tree file until `0x32C9` and stores
/// each setting: `0x32CA` level, `0x32CB` frond type, `0x32CC` number of blades, `0x32CD` the
/// profile spline (`ParseBranchParameter`, then `SetProfile`), `0x32CE` profile segments,
/// `0x32CF` the enabled flag, `0x32D0` the texture list (the list is cleared, then each texture
/// is read as a run of tokens up to `0x36B1`: `0x36B2` the file name without its path,
/// `0x36B3`..`0x36B6` the aspect ratio, size scale and the two angle offsets), `0x32D1` number of
/// LOD levels, `0x32D2`..`0x32D5` the four surface-area floats, `0x36B7` and `0x36B8` the minimum
/// length and cross-section segments. An unknown token throws an exception built from a
/// formatted message ("malformed frond info (token %d)", "malformed frond texture information
/// (token %d)"); the throw does not return in the exe.
pub fn c_frond_engine_parse(e: &mut Engine, this: Ptr<CFrondEngine>, file: Ptr) {
    let mut token = e.call(PARSE_INT, &args![file]).i32();
    loop {
        match token {
            0x32ca => {
                let value = e.call(PARSE_INT, &args![file]).u32();
                e.set(this, CFrondEngine::m_nLevel, value);
            }
            0x32cb => {
                let value = e.call(PARSE_INT, &args![file]).u32();
                e.set(this, CFrondEngine::m_eFrondType, value);
            }
            0x32cc => {
                let value = e.call(PARSE_INT, &args![file]).u32();
                e.set(this, CFrondEngine::m_nNumBlades, value);
            }
            0x32cd => {
                let spline = e.call(PARSE_BRANCH_PARAMETER, &args![file]).ptr::<()>();
                c_frond_engine_set_profile(e, this, spline);
            }
            0x32ce => {
                let value = e.call(PARSE_INT, &args![file]).u32();
                e.set(this, CFrondEngine::m_nProfileSegments, value);
            }
            0x32cf => {
                let value = e.call(PARSE_BOOL, &args![file]).u8();
                e.set(this, CFrondEngine::m_bEnabled, value);
            }
            0x32d0 => {
                if !parse_textures(e, this, file) {
                    return;
                }
            }
            0x32d1 => {
                let value = e.call(PARSE_INT, &args![file]).u32();
                e.set(this, CFrondEngine::m_nNumLodLevels, value);
            }
            0x32d2 => {
                let value = e.call(PARSE_FLOAT, &args![file]).f32();
                e.set(this, CFrondEngine::m_fMaxSurfaceAreaPercent, value);
            }
            0x32d3 => {
                let value = e.call(PARSE_FLOAT, &args![file]).f32();
                e.set(this, CFrondEngine::m_fMinSurfaceAreaPercent, value);
            }
            0x32d4 => {
                let value = e.call(PARSE_FLOAT, &args![file]).f32();
                e.set(this, CFrondEngine::m_fReductionFuzziness, value);
            }
            0x32d5 => {
                let value = e.call(PARSE_FLOAT, &args![file]).f32();
                e.set(this, CFrondEngine::m_fLargeRetentionPercent, value);
            }
            0x36b7 => {
                let value = e.call(PARSE_INT, &args![file]).u32();
                e.set(this, CFrondEngine::m_nMinLengthSegments, value);
            }
            0x36b8 => {
                let value = e.call(PARSE_INT, &args![file]).u32();
                e.set(this, CFrondEngine::m_nMinCrossSegments, value);
            }
            _ => {
                throw_formatted(e, MALFORMED_FROND_MESSAGE, token as u32);
                return;
            }
        }
        token = e.call(PARSE_INT, &args![file]).i32();
        if token == 0x32c9 {
            return;
        }
    }
}

/// The texture list of `Parse` (token `0x32D0`): clears the texture vector, reads the count and
/// for each texture a run of tokens up to `0x36B1`, then appends it. Returns false when a
/// malformed token threw (the exe does not come back from that).
fn parse_textures(e: &mut Engine, this: Ptr<CFrondEngine>, file: Ptr) -> bool {
    let textures = Ptr::new(this.addr() + 0x50);
    fn_00b1e2f0(e, textures);
    let count = e.call(PARSE_INT, &args![file]).i32();
    e.with_stack(0xA0, |e, frame| {
        let parsed = frame.addr();
        let copy = frame.addr() + 0x20;
        let no_path = frame.addr() + 0x40;
        let texture = frame.addr() + 0x60;
        let mut index = 0i32;
        while index < count {
            fn_00b1d100(e, Ptr::new(texture));
            copy_float(e, texture + 0x1C, HALF_FLOAT);
            e.mem.set_f32(texture + 0x20, 1.0);
            e.mem.set_f32(texture + 0x24, 0.0);
            e.mem.set_f32(texture + 0x28, 0.0);
            // The first token read is dropped, the second one decides.
            e.call(PARSE_INT, &args![file]);
            let mut token = e.call(PARSE_INT, &args![file]).i32();
            loop {
                match token {
                    0x36b2 => {
                        e.call(PARSE_STRING, &args![file, parsed]);
                        e.call(STRING_TIDY, &args![copy, 0u32, 0u32]);
                        let npos: u32 = e.global(STRING_NPOS);
                        e.call(STRING_ASSIGN, &args![copy, parsed, 0u32, npos]);
                        e.call(STRING_TIDY, &args![parsed, 1u32, 0u32]);
                        let npos: u32 = e.global(STRING_NPOS);
                        e.call(STRING_ASSIGN, &args![texture, copy, 0u32, npos]);
                        e.call(STRING_TIDY, &args![copy, 1u32, 0u32]);
                        let stripped = e.call(FILENAME_NO_PATH, &args![texture, no_path]).u32();
                        let npos: u32 = e.global(STRING_NPOS);
                        e.call(STRING_ASSIGN, &args![texture, stripped, 0u32, npos]);
                        e.call(STRING_TIDY, &args![no_path, 1u32, 0u32]);
                    }
                    0x36b3..=0x36b6 => {
                        let value = e.call(PARSE_FLOAT, &args![file]).f32();
                        e.mem
                            .set_f32(texture + 0x1C + 4 * (token as u32 - 0x36b3), value);
                    }
                    _ => {
                        throw_formatted(e, MALFORMED_TEXTURE_MESSAGE, token as u32);
                        return false;
                    }
                }
                token = e.call(PARSE_INT, &args![file]).i32();
                if token == 0x36b1 {
                    break;
                }
            }
            fn_00b1e1e0(e, textures, Ptr::new(texture));
            e.call(STRING_TIDY, &args![texture, 1u32, 0u32]);
            index += 1;
        }
        true
    })
}

// Translated from 00b18860 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b18860 - the /GS cookie check (00ec408c) is not translated.
/// `IdvFormatString` (engine map; `cdecl`, variable arguments): formats `format` into a
/// 0x400-byte stack buffer with `vsprintf` (the variable arguments are passed as an argument
/// list, here copied into game memory) and constructs the string `out` from the buffer
/// (`0044a4c0`). Returns `out`.
pub fn idv_format_string(e: &mut Engine, out: Ptr, format: Ptr, arguments: &[u32]) -> Ptr {
    e.with_stack(0x400 + 4 * arguments.len().max(1) as u32, |e, frame| {
        let buffer = frame.addr();
        let list = frame.addr() + 0x400;
        for (i, word) in arguments.iter().enumerate() {
            e.mem.set_u32(list + 4 * i as u32, *word);
        }
        e.call(VSPRINTF, &args![buffer, format, list]);
        e.call(STRING_FROM_CHARS, &args![out, buffer]);
    });
    out
}

/// The registered form of `IdvFormatString`: two fixed words (the string to build and the
/// format), then however many arguments the caller pushed.
fn idv_format_string_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() >= 2,
        "IdvFormatString needs 2 argument words, got {}",
        words.len()
    );
    let out = idv_format_string(e, Ptr::new(words[0]), Ptr::new(words[1]), &words[2..]);
    Ret {
        eax: out.addr(),
        ..Ret::default()
    }
}

// Translated from 00b188d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b188d0 - the FS:[0] exception frame (cleanup of the temporary
// guide if the push throws) is not translated.
/// `CFrondEngine::BeginGuide` (name unconfirmed; the engine map gives it none): appends a new,
/// default-constructed guide to the guide vector (`+8`): builds a temporary `SFrondGuide`
/// (`00b18970`), pushes it (`00b1db40`), then destroys the temporary (its vertex vector is
/// tidied with `00b1e970` and the proxy freed).
pub fn fn_00b188d0(e: &mut Engine, this: Ptr<CFrondEngine>) {
    e.with_stack(0x38, |e, guide| {
        fn_00b18970(e, guide.cast());
        fn_00b1db40(e, Ptr::new(this.addr() + 8), guide);
        e.call(VERTEX_VECTOR_TIDY, &args![guide]);
        let proxy = e.mem.u32(guide.addr());
        e.call(OPERATOR_DELETE, &args![proxy]);
    });
}

// Translated from 00b18970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SFrondGuide::SFrondGuide` (the engine map gives it no name): constructs the vertex vector
/// (`00b1d8a0`) and zeroes length, radius, map index, offset angle, the two surface areas, the
/// vertex start index and the vertices per guide vertex. Returns `this`.
pub fn fn_00b18970(e: &mut Engine, this: Ptr<SFrondGuide>) -> Ptr<SFrondGuide> {
    fn_00b1d8a0(e, this.cast());
    e.set(this, SFrondGuide::m_fLength, 0.0);
    e.set(this, SFrondGuide::m_fRadius, 0.0);
    e.set(this, SFrondGuide::m_chFrondMapIndex, 0);
    e.set(this, SFrondGuide::m_fOffsetAngle, 0.0);
    e.set(this, SFrondGuide::m_fSurfaceArea, 0.0);
    e.set(this, SFrondGuide::m_fFuzzySurfaceArea, 0.0);
    e.set(this, SFrondGuide::m_nVertexStartIndex, 0);
    e.set(this, SFrondGuide::m_nVerticesPerGuideVertex, 0);
    this
}

// Translated from 00b189f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b189f0 - the FS:[0] exception frame (cleanup state of the
// member destructor) is not translated.
/// `SFrondGuide`'s destructor (the engine map gives it no name): tidies the vertex vector
/// (`00b1e970`) and frees its proxy.
pub fn fn_00b189f0(e: &mut Engine, this: Ptr<SFrondGuide>) {
    e.call(VERTEX_VECTOR_TIDY, &args![this]);
    let proxy = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00b18a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CFrondEngine::AddPoint` (name unconfirmed; the engine map gives it none): appends a vertex
/// to the last guide of the guide vector (`+8`): builds an `SFrondVertex` on the stack from the
/// position (three words), the nine matrix words, the cross-section weight (copied through the
/// x87 stack) and the wind group, and pushes it onto the vertex vector of the last guide
/// (`00b1d980`); an empty guide vector takes the invalid-parameter path.
#[allow(clippy::too_many_arguments)]
pub fn fn_00b18a60(
    e: &mut Engine,
    this: Ptr<CFrondEngine>,
    x: u32,
    y: u32,
    z: u32,
    m0: u32,
    m1: u32,
    m2: u32,
    m3: u32,
    m4: u32,
    m5: u32,
    m6: u32,
    m7: u32,
    m8: u32,
    weight: f32,
    wind_group: u32,
) {
    e.with_stack(0x38, |e, vertex| {
        fn_00b18b20(e, vertex.cast());
        let v = vertex.addr();
        e.mem.set_u32(v, x);
        e.mem.set_u32(v + 4, y);
        e.mem.set_u32(v + 8, z);
        for (i, word) in [m0, m1, m2, m3, m4, m5, m6, m7, m8].iter().enumerate() {
            e.mem.set_u32(v + 0x0C + 4 * i as u32, *word);
        }
        e.mem.set_u32(v + 0x30, x87_quiet_bits(weight.to_bits()));
        e.mem.set_u32(v + 0x34, wind_group);
        let guides = this.addr() + 8;
        let last = vector_len(e, guides, 0x38).wrapping_sub(1);
        let guide = vector_at(e, guides, last, 0x38);
        fn_00b1d980(e, Ptr::new(guide), vertex);
    });
}

// Translated from 00b18b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SFrondVertex::SFrondVertex` (the engine map gives it no name): zeroes the position, builds
/// the identity transform (`00b12af0`), zeroes the cross-section weight and the wind group.
/// Returns `this`.
pub fn fn_00b18b20(e: &mut Engine, this: Ptr<SFrondVertex>) -> Ptr<SFrondVertex> {
    let base = this.addr();
    for offset in [8, 4, 0] {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.call(CROT_TRANSFORM_CONSTRUCT, &args![base + 0x0C]);
    e.set(this, SFrondVertex::m_fCrossSectionWeight, 0.0);
    e.set(this, SFrondVertex::m_nWindGroup, 0);
    this
}

// Translated from 00b18b70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b18b70 - the FS:[0] exception frame (the destructor of the local
// `CIdvRandom`, run on unwinding) is not translated.
/// `CFrondEngine::EndGuide` (Xbox PDB): finishes the last guide of the guide vector. Its
/// length (`+0x18`) is the sum of the distances between consecutive vertices (`Distance`,
/// `00b04d40`, each rounded to a float when added). Without frond textures the map index
/// (`+0x20`) of the element of the guide vector at the loop's final index (the exe indexes the
/// guide vector, not the guide's vertices, with it) is cleared, the radius (`+0x1C`) is half the
/// length and the offset angle (`+0x24`) is zero. With textures a random texture is chosen
/// (`GetUniform(0, 100000.0)` truncated, modulo the texture count, stored as the map index),
/// the radius is half the length times the texture's aspect ratio and size scale, and the offset
/// angle is a random value between the texture's two angle offsets, mirrored for odd guide
/// counts. The surface area (`+0x28`) is the length times `area_scale`.
pub fn c_frond_engine_end_guide(e: &mut Engine, this: Ptr<CFrondEngine>, area_scale: f32) {
    let base = this.addr();
    let guides = base + 8;
    let last_index = vector_len(e, guides, 0x38).wrapping_sub(1);
    let guide = vector_at(e, guides, last_index, 0x38);
    let mut i = 0u32;
    while i < vector_len(e, guide, 0x38).wrapping_sub(1) {
        let next = vector_at(e, guide, i + 1, 0x38);
        let current = vector_at(e, guide, i, 0x38);
        let distance = e.call(VEC3_DISTANCE, &args![current, next]).f64();
        let length = ld(e, guide + 0x18);
        st(e, guide + 0x18, distance + length);
        i += 1;
    }
    e.with_stack(4, |e, random| {
        e.call(RANDOM_CONSTRUCT, &args![random]);
        let textures = base + 0x50;
        if vector_len(e, textures, 0x2C) == 0 {
            let element = vector_at(e, guides, i, 0x38);
            e.mem.set_u8(element + 0x20, 0);
            let radius = ld(e, guide + 0x18) * e.global::<f64>(SCALE_DOUBLE);
            st(e, guide + 0x1C, radius);
            e.mem.set_f32(guide + 0x24, 0.0);
        } else {
            let texture_count = vector_len(e, textures, 0x2C);
            let high = x87_quiet_bits(e.global::<u32>(RANDOM_UPPER_BOUND));
            let draw = e.call(GET_UNIFORM, &args![random, 0u32, high]).f64();
            let choice = truncate_low(draw) % texture_count;
            e.mem.set_u8(guide + 0x20, choice as u8);
            let index = u32::from(e.mem.u8(guide + 0x20));
            let texture = vector_at(e, textures, index, 0x2C);
            let radius =
                ld(e, guide + 0x18) * ld(e, texture + 0x1C) * e.global::<f64>(SCALE_DOUBLE);
            st(e, guide + 0x1C, radius);
            let index = u32::from(e.mem.u8(guide + 0x20));
            let texture = vector_at(e, textures, index, 0x2C);
            let radius = ld(e, guide + 0x1C) * ld(e, texture + 0x20);
            st(e, guide + 0x1C, radius);
            let index = u32::from(e.mem.u8(guide + 0x20));
            let high_texture = vector_at(e, textures, index, 0x2C);
            let index = u32::from(e.mem.u8(guide + 0x20));
            let low_texture = vector_at(e, textures, index, 0x2C);
            let low = x87_quiet_bits(e.mem.u32(low_texture + 0x24));
            let high = x87_quiet_bits(e.mem.u32(high_texture + 0x28));
            let angle = e.call(GET_UNIFORM, &args![random, low, high]).f64();
            st(e, guide + 0x24, angle);
            if vector_len(e, guides, 0x38) % 2 == 1 {
                let mirrored = ld(e, guide + 0x24) * e.global::<f64>(MIRROR_FACTOR);
                st(e, guide + 0x24, mirrored);
            }
        }
        let area = ld(e, guide + 0x18) * f64::from(area_scale);
        st(e, guide + 0x28, area);
        e.call(TRIVIAL_DESTRUCTOR, &args![random]);
    });
}

// Translated from 00b18f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CFrondEngine::SetProfile` (Xbox PDB): when `spline` is not the current profile, destroys the
/// current one (`00b37880`, then `operator delete`; the delete flag is the constant 1) and
/// stores the new one.
pub fn c_frond_engine_set_profile(e: &mut Engine, this: Ptr<CFrondEngine>, spline: Ptr) {
    let current = e.get(this, CFrondEngine::m_pProfile);
    if current.addr() != spline.addr() {
        let old = e.get(this, CFrondEngine::m_pProfile);
        if !old.is_null() {
            e.call(SPLINE_DESTRUCT, &args![old]);
            // `MOV EDX,1; AND EDX,1; JZ`: the delete flag is always set.
            e.call(OPERATOR_DELETE, &args![old]);
        }
        e.set(this, CFrondEngine::m_pProfile, spline);
    }
}

// Translated from 00b18fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b18fd0 - the FS:[0] exception frame (cleanup of the temporaries of
// the throw paths) is not translated.
/// `CFrondEngine::Compute` (Xbox PDB): stores the geometry and the lighting engine, builds the
/// guide lods (`BuildGuideLods`), then counts the vertices the geometry will receive: for each
/// guide of the first lod, twice its vertex count times the blade count (frond type 0) or its
/// vertex count times `2 * profile segments - 1` (type 1); any other type throws "default
/// reached in CFrondEngine::Compute()", a count above 0xFFFF throws "frond vertices exceed %d".
/// It sets the geometry's lod level count and writes every guide of the first lod
/// (`00b19ad0` for type 0, `00b1b000` for type 1), then for each lod level resets the strip
/// counter (`00b306d0`) and registers the strips of its guides: `00b1aed0` for type 0; for type
/// 1 the first lod's vertices-per-guide-vertex are copied to the lod's guides and `00b1b960`
/// builds the strips. The throws do not return in the exe.
pub fn c_frond_engine_compute(
    e: &mut Engine,
    this: Ptr<CFrondEngine>,
    geometry: Ptr<CIndexedGeometry>,
    lighting: Ptr,
) {
    let base = this.addr();
    let lods = base + 0x20;
    e.set(this, CFrondEngine::m_pGeometry, geometry);
    e.set(this, CFrondEngine::m_pLightingEngine, lighting);
    let mut total = 0i32;
    c_frond_engine_build_guide_lods(e, this);
    let mut guide_index = 0u32;
    loop {
        let first_lod = vector_at(e, lods, 0, 0x18);
        if guide_index >= vector_len(e, first_lod, 0x38) {
            break;
        }
        match e.get(this, CFrondEngine::m_eFrondType) {
            0 => {
                let lod = vector_at(e, lods, 0, 0x18);
                let guide = vector_at(e, lod, guide_index, 0x38);
                let vertices = (vector_len(e, guide, 0x38) << 1) as i32;
                let blades = e.get(this, CFrondEngine::m_nNumBlades) as i32;
                total = vertices.wrapping_mul(blades).wrapping_add(total);
            }
            1 => {
                let lod = vector_at(e, lods, 0, 0x18);
                let guide = vector_at(e, lod, guide_index, 0x38);
                let segments = e.get(this, CFrondEngine::m_nProfileSegments) as i32;
                let per_vertex = segments.wrapping_add(segments).wrapping_sub(1);
                let vertices = vector_len(e, guide, 0x38) as i32;
                total = vertices.wrapping_mul(per_vertex).wrapping_add(total);
            }
            _ => {
                throw_default_reached(e);
                return;
            }
        }
        guide_index += 1;
    }
    if total > 0xFFFF {
        e.with_stack(0x60, |e, frame| {
            let text = idv_format_string(e, frame, Ptr::new(VERTICES_EXCEED_MESSAGE), &[0xFFFFu32]);
            let exception = frame.addr() + 0x20;
            e.call(BASE_EXCEPTION_CONSTRUCT, &args![exception]);
            e.mem.set_u32(exception, OVERFLOW_EXCEPTION_VTABLE);
            e.call(STRING_COPY, &args![exception + 0x0C, text]);
            e.call(THROW, &args![exception, OVERFLOW_THROW_INFO]);
        });
        return;
    }
    let lod_levels = e.get(this, CFrondEngine::m_nNumLodLevels) & 0xFFFF;
    e.call(GEOMETRY_SET_NUM_LOD_LEVELS, &args![geometry, lod_levels]);
    match e.get(this, CFrondEngine::m_eFrondType) {
        0 => {
            let mut guide_index = 0u32;
            loop {
                let first_lod = vector_at(e, lods, 0, 0x18);
                if guide_index >= vector_len(e, first_lod, 0x38) {
                    break;
                }
                let lod = vector_at(e, lods, 0, 0x18);
                let guide = vector_at(e, lod, guide_index, 0x38);
                fn_00b19ad0(e, this, Ptr::new(guide));
                guide_index += 1;
            }
            let mut level = 0u16;
            while u32::from(level) < vector_len(e, lods, 0x18) {
                let current = e.get(this, CFrondEngine::m_pGeometry);
                e.call(
                    GEOMETRY_RESET_STRIP_COUNTER,
                    &args![current, u32::from(level)],
                );
                let mut guide_index = 0u32;
                loop {
                    let lod = vector_at(e, lods, u32::from(level), 0x18);
                    if guide_index >= vector_len(e, lod, 0x38) {
                        break;
                    }
                    let lod = vector_at(e, lods, u32::from(level), 0x18);
                    let guide = vector_at(e, lod, guide_index, 0x38);
                    let first_lod = vector_at(e, lods, 0, 0x18);
                    let first_guide = vector_at(e, first_lod, guide_index, 0x38);
                    let start = e.mem.u32(first_guide + 0x30);
                    fn_00b1aed0(e, this, level, start, Ptr::new(guide));
                    guide_index += 1;
                }
                level = level.wrapping_add(1);
            }
        }
        1 => {
            let mut guide_index = 0u32;
            loop {
                let first_lod = vector_at(e, lods, 0, 0x18);
                if guide_index >= vector_len(e, first_lod, 0x38) {
                    break;
                }
                let lod = vector_at(e, lods, 0, 0x18);
                let guide = vector_at(e, lod, guide_index, 0x38);
                fn_00b1b000(e, this, Ptr::new(guide));
                guide_index += 1;
            }
            let mut level = 0u16;
            while u32::from(level) < vector_len(e, lods, 0x18) {
                let current = e.get(this, CFrondEngine::m_pGeometry);
                e.call(
                    GEOMETRY_RESET_STRIP_COUNTER,
                    &args![current, u32::from(level)],
                );
                let mut guide_index = 0u32;
                loop {
                    let lod = vector_at(e, lods, u32::from(level), 0x18);
                    if guide_index >= vector_len(e, lod, 0x38) {
                        break;
                    }
                    let first_lod = vector_at(e, lods, 0, 0x18);
                    let first_guide = vector_at(e, first_lod, guide_index, 0x38);
                    let lod = vector_at(e, lods, u32::from(level), 0x18);
                    let guide = vector_at(e, lod, guide_index, 0x38);
                    let per_vertex = e.mem.u32(first_guide + 0x34);
                    e.mem.set_u32(guide + 0x34, per_vertex);
                    let lod = vector_at(e, lods, u32::from(level), 0x18);
                    let guide = vector_at(e, lod, guide_index, 0x38);
                    let first_lod = vector_at(e, lods, 0, 0x18);
                    let first_guide = fn_00b1db00(e, Ptr::new(first_lod), guide_index);
                    let start = e.mem.u32(first_guide.addr() + 0x30);
                    fn_00b1b960(e, this, u32::from(level), start, Ptr::new(guide));
                    guide_index += 1;
                }
                level = level.wrapping_add(1);
            }
        }
        _ => throw_default_reached(e),
    }
}

/// The tail the two vertices of a blade share (`00b19ad0`): white colour (`00b30020`), the
/// guide direction as normal (`00b30190`), the blade direction as tangent (`00b305f0`), the
/// cross product of the two as binormal (`00b12810`) normalised unless it is shorter than the
/// minimum (then the direction is used), the binormal (`00b30660`), the vertex's wind weight
/// when the geometry has vertex weighting and the vertex counter increment. The first vertex
/// reads the vertex twice through checked accesses and takes the weight from the vector's
/// storage, the second reads it through `operator[]` (`00b1db00`).
#[allow(clippy::too_many_arguments)]
fn blade_vertex_tail(
    e: &mut Engine,
    this: Ptr<CFrondEngine>,
    guide: u32,
    index: u32,
    direction: u32,
    profile: u32,
    binormal: u32,
    color: u32,
    cross: u32,
    second: bool,
) {
    let geometry = e.get(this, CFrondEngine::m_pGeometry);
    for k in 0..4u32 {
        e.mem.set_f32(color + 4 * k, 1.0);
    }
    e.call(GEOMETRY_ADD_COLOR, &args![geometry, color]);
    e.call(GEOMETRY_ADD_NORMAL, &args![geometry, direction]);
    e.call(GEOMETRY_ADD_TANGENT, &args![geometry, profile]);
    let product = e.call(VEC3_CROSS, &args![profile, cross, direction]).u32();
    copy_words(e, binormal, product, 3);
    let length = e.call(VEC3_LENGTH, &args![binormal]).f64();
    let minimum = f64::from(e.global::<f32>(MINIMUM_NORMAL_LENGTH));
    if minimum > length {
        copy_words(e, binormal, direction, 3);
    } else {
        e.call(VEC3_NORMALIZE, &args![binormal]);
    }
    e.call(GEOMETRY_ADD_BINORMAL, &args![geometry, binormal]);
    if e.get(geometry, CIndexedGeometry::m_bVertexWeighting) != 0 {
        if second {
            let vertex = vector_at(e, guide, index, 0x38);
            let group = e.mem.u8(vertex + 0x34);
            let element = fn_00b1db00(e, Ptr::new(guide), index).addr();
            let weight = x87_quiet_bits(e.mem.u32(element + 0x30));
            e.call(
                GEOMETRY_ADD_WIND_WEIGHT,
                &args![geometry, weight, u32::from(group)],
            );
        } else {
            let vertex = vector_at(e, guide, index, 0x38);
            let _ = vector_at(e, guide, index, 0x38);
            let group = e.mem.u8(vertex + 0x34);
            let first = e.mem.u32(guide + 0x0C);
            let weight = x87_quiet_bits(e.mem.u32(first + index.wrapping_mul(0x38) + 0x30));
            e.call(
                GEOMETRY_ADD_WIND_WEIGHT,
                &args![geometry, weight, u32::from(group)],
            );
        }
    }
    let counter = e.get(geometry, CIndexedGeometry::m_nVertexCounter);
    e.set(
        geometry,
        CIndexedGeometry::m_nVertexCounter,
        counter.wrapping_add(1),
    );
}

// Translated from 00b19ad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b19ad0 - the FS:[0] exception frame (cleanup of the per-blade
// vectors if a call throws) is not translated.
/// Writes the blade geometry of one guide (frond type 0; the engine map gives it no name), for
/// an engine that has a geometry and a lighting engine. The guide's start index (`+0x30`)
/// becomes the geometry's vertex counter and its vertices per guide vertex (`+0x34`) 2. For
/// each blade (the angle step is `180 / blades` degrees; the blade angle plus the guide's offset
/// angle (`+0x24`) rotates a copy of the guide vertex's transform about X with `00b1ae00`) the
/// Y axis times that transform is the blade's direction. Each guide vertex then gives two
/// geometry vertices: its position plus and minus the radius (`+0x1C`) times the blade
/// direction (averaged with the neighbouring ring for the middle vertices; the last vertex uses
/// the direction of the previous one), with texture coordinates (1 or 0, the position along
/// the guide) and the attributes of [`blade_vertex_tail`]. Finally the texture coordinates of
/// each vertex pair are rewritten from the accumulated lengths of the two edges.
pub fn fn_00b19ad0(e: &mut Engine, this: Ptr<CFrondEngine>, guide: Ptr<SFrondGuide>) {
    let g = guide.addr();
    if e.get(this, CFrondEngine::m_pGeometry).is_null()
        || e.get(this, CFrondEngine::m_pLightingEngine).is_null()
    {
        return;
    }
    let blades = e.get(this, CFrondEngine::m_nNumBlades);
    let step = (e.global::<f64>(BLADE_ANGLE_NUMERATOR) / f64::from(blades)) as f32;
    let geometry = e.get(this, CFrondEngine::m_pGeometry);
    let counter = e.get(geometry, CIndexedGeometry::m_nVertexCounter);
    e.mem.set_u32(g + 0x30, u32::from(counter));
    e.mem.set_u32(g + 0x34, 2);
    e.with_stack(0x200, |e, frame| {
        let f = frame.addr();
        let rotation = f;
        let axis = f + 0x24;
        let direction = f + 0x30;
        let profile = f + 0x3C;
        let point_a = f + 0x48;
        let point_b = f + 0x54;
        let point_c = f + 0x60;
        let binormal = f + 0x6C;
        let texture = f + 0x78;
        let color = f + 0x80;
        let points = f + 0x90;
        let left_lengths = f + 0xA8;
        let right_lengths = f + 0xC0;
        let left_total = f + 0xD8;
        let right_total = f + 0xDC;
        let left_texture = f + 0xE0;
        let right_texture = f + 0xE8;
        let edge_a = f + 0xF0;
        let edge_b = f + 0xFC;
        let scratch = |k: u32| f + 0x108 + 0x0C * k;
        let mut blade = 0u32;
        while blade < e.get(this, CFrondEngine::m_nNumBlades) {
            let geometry = e.get(this, CFrondEngine::m_pGeometry);
            let start = u32::from(e.get(geometry, CIndexedGeometry::m_nVertexCounter));
            e.call(CVEC3_VECTOR_CONSTRUCT, &args![points]);
            for k in [8u32, 4, 0] {
                e.mem.set_f32(binormal + k, 0.0);
            }
            let mut i = 0u32;
            while i < vector_len(e, g, 0x38) {
                for (k, value) in [1.0f32, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]
                    .iter()
                    .enumerate()
                {
                    e.mem.set_f32(rotation + 4 * k as u32, *value);
                }
                if i < vector_len(e, g, 0x38).wrapping_sub(1) {
                    let vertex = vector_at(e, g, i, 0x38);
                    copy_words(e, rotation, vertex + 0x0C, 9);
                } else {
                    let vertex = vector_at(e, g, i.wrapping_sub(1), 0x38);
                    copy_words(e, rotation, vertex + 0x0C, 9);
                }
                let angle = (f64::from(blade) * f64::from(step) + ld(e, g + 0x24)) as f32;
                fn_00b1ae00(e, Ptr::new(rotation), angle);
                e.mem.set_f32(axis, 0.0);
                e.mem.set_f32(axis + 4, 1.0);
                e.mem.set_f32(axis + 8, 0.0);
                e.call(VEC3_TIMES_ROTATION, &args![axis, profile, rotation]);
                e.call(CVEC3_VECTOR_PUSH, &args![points, profile]);
                for k in [8u32, 4, 0] {
                    e.mem.set_f32(direction + k, 0.0);
                }
                // The direction along the guide: the second vertex minus the first for the
                // first vertex, otherwise this vertex minus the first.
                if i == 0 {
                    let first = vector_at(e, g, 0, 0x38);
                    let second = vector_at(e, g, i.wrapping_add(1), 0x38);
                    let difference = e
                        .call(VEC3_SUBTRACT, &args![second, scratch(0), first])
                        .u32();
                    copy_words(e, direction, difference, 3);
                } else {
                    let first = vector_at(e, g, 0, 0x38);
                    let current = vector_at(e, g, i, 0x38);
                    let difference = e
                        .call(VEC3_SUBTRACT, &args![current, scratch(0), first])
                        .u32();
                    copy_words(e, direction, difference, 3);
                }
                e.call(VEC3_NORMALIZE, &args![direction]);
                for k in [8u32, 4, 0] {
                    e.mem.set_f32(point_a + k, 0.0);
                    e.mem.set_f32(point_b + k, 0.0);
                }
                let count = vector_len(e, g, 0x38);
                let radius = x87_quiet_bits(e.mem.u32(g + 0x1C));
                let half = x87_quiet_bits(e.global::<u32>(HALF_FLOAT));
                if i == count.wrapping_sub(1) && count != 0 {
                    // The last vertex: the previous vertex's direction.
                    let toward = vector_at(e, points, i.wrapping_sub(1), 0x0C);
                    let vertex = vector_at(e, g, i, 0x38);
                    let scaled = e.call(VEC3_SCALE, &args![toward, scratch(1), radius]).u32();
                    let sum = e.call(VEC3_ADD, &args![vertex, scratch(2), scaled]).u32();
                    copy_words(e, point_a, sum, 3);
                    let toward = vector_at(e, points, i.wrapping_sub(1), 0x0C);
                    let vertex = vector_at(e, g, i, 0x38);
                    let scaled = e.call(VEC3_SCALE, &args![toward, scratch(3), radius]).u32();
                    let difference = e
                        .call(VEC3_SUBTRACT, &args![vertex, scratch(4), scaled])
                        .u32();
                    copy_words(e, point_b, difference, 3);
                } else if i > 0 && i < count.wrapping_sub(1) {
                    // A middle vertex: the average of the previous vertex's ring and its own.
                    for k in [8u32, 4, 0] {
                        e.mem.set_f32(point_c + k, 0.0);
                    }
                    let toward = vector_at(e, points, i.wrapping_sub(1), 0x0C);
                    let vertex = vector_at(e, g, i, 0x38);
                    let scaled = e.call(VEC3_SCALE, &args![toward, scratch(5), radius]).u32();
                    let sum = e.call(VEC3_ADD, &args![vertex, scratch(6), scaled]).u32();
                    copy_words(e, point_c, sum, 3);
                    let vertex = vector_at(e, g, i, 0x38);
                    let scaled = e
                        .call(VEC3_SCALE, &args![profile, scratch(7), radius])
                        .u32();
                    let sum = e.call(VEC3_ADD, &args![vertex, scratch(8), scaled]).u32();
                    copy_words(e, point_a, sum, 3);
                    let total = e.call(VEC3_ADD, &args![point_a, scratch(9), point_c]).u32();
                    let average = e.call(VEC3_SCALE, &args![total, scratch(10), half]).u32();
                    copy_words(e, point_a, average, 3);
                    let toward = vector_at(e, points, i.wrapping_sub(1), 0x0C);
                    let vertex = vector_at(e, g, i, 0x38);
                    let scaled = e
                        .call(VEC3_SCALE, &args![toward, scratch(11), radius])
                        .u32();
                    let difference = e
                        .call(VEC3_SUBTRACT, &args![vertex, scratch(12), scaled])
                        .u32();
                    copy_words(e, point_c, difference, 3);
                    let vertex = vector_at(e, g, i, 0x38);
                    let scaled = e
                        .call(VEC3_SCALE, &args![profile, scratch(13), radius])
                        .u32();
                    let difference = e
                        .call(VEC3_SUBTRACT, &args![vertex, scratch(14), scaled])
                        .u32();
                    copy_words(e, point_b, difference, 3);
                    let total = e
                        .call(VEC3_ADD, &args![point_b, scratch(15), point_c])
                        .u32();
                    let average = e.call(VEC3_SCALE, &args![total, scratch(16), half]).u32();
                    copy_words(e, point_b, average, 3);
                } else {
                    // The first vertex.
                    let vertex = vector_at(e, g, i, 0x38);
                    let scaled = e
                        .call(VEC3_SCALE, &args![profile, scratch(5), radius])
                        .u32();
                    let sum = e.call(VEC3_ADD, &args![vertex, scratch(6), scaled]).u32();
                    copy_words(e, point_a, sum, 3);
                    let vertex = vector_at(e, g, i, 0x38);
                    let scaled = e
                        .call(VEC3_SCALE, &args![profile, scratch(7), radius])
                        .u32();
                    let difference = e
                        .call(VEC3_SUBTRACT, &args![vertex, scratch(8), scaled])
                        .u32();
                    copy_words(e, point_b, difference, 3);
                }
                let along = (f64::from(i)
                    / (f64::from(vector_len(e, g, 0x38)) - e.global::<f64>(ONE_DOUBLE)))
                    as f32;
                let map_index = u32::from(e.mem.u8(g + 0x20));
                let geometry = e.get(this, CFrondEngine::m_pGeometry);
                e.call(GEOMETRY_ADD_COORDINATE, &args![geometry, point_a]);
                e.mem.set_f32(texture, 1.0);
                e.mem.set_f32(texture + 4, x87_quiet(along));
                e.call(GEOMETRY_ADD_TEX_COORD, &args![geometry, texture, map_index]);
                blade_vertex_tail(
                    e,
                    this,
                    g,
                    i,
                    direction,
                    profile,
                    binormal,
                    color,
                    scratch(0),
                    false,
                );
                let geometry = e.get(this, CFrondEngine::m_pGeometry);
                e.call(GEOMETRY_ADD_COORDINATE, &args![geometry, point_b]);
                e.mem.set_f32(texture, 0.0);
                let map_index = u32::from(e.mem.u8(g + 0x20));
                e.call(GEOMETRY_ADD_TEX_COORD, &args![geometry, texture, map_index]);
                blade_vertex_tail(
                    e,
                    this,
                    g,
                    i,
                    direction,
                    profile,
                    binormal,
                    color,
                    scratch(1),
                    true,
                );
                i += 1;
            }
            // The accumulated lengths of the two edges of the blade.
            e.mem.set_f32(left_total, 0.0);
            e.mem.set_f32(right_total, 0.0);
            e.call(FLOAT_VECTOR_CONSTRUCT, &args![left_lengths]);
            e.call(FLOAT_VECTOR_CONSTRUCT, &args![right_lengths]);
            let count = vector_len(e, g, 0x38);
            let mut k = 0u32;
            while k < count.wrapping_sub(1) {
                let geometry = e.get(this, CFrondEngine::m_pGeometry);
                let first = e.mem.u32(g + 0x30);
                let pair = first.wrapping_add(k.wrapping_mul(2));
                let near = e.call(GEOMETRY_COORD_POINTER, &args![geometry, pair]).u32();
                let far = e
                    .call(
                        GEOMETRY_COORD_POINTER,
                        &args![
                            geometry,
                            first.wrapping_add(k.wrapping_mul(2)).wrapping_add(2)
                        ],
                    )
                    .u32();
                for axis_index in 0..3u32 {
                    copy_float(e, edge_a + 4 * axis_index, near + 4 * axis_index);
                }
                fn_00b1ada0(
                    e,
                    Ptr::new(edge_b),
                    x87_quiet(e.mem.f32(far)),
                    x87_quiet(e.mem.f32(far + 4)),
                    x87_quiet(e.mem.f32(far + 8)),
                );
                let distance = e.call(VEC3_DISTANCE, &args![edge_a, edge_b]).f64();
                let total = distance + ld(e, left_total);
                st(e, left_total, total);
                e.call(FLOAT_VECTOR_PUSH, &args![left_lengths, left_total]);
                let near = e
                    .call(
                        GEOMETRY_COORD_POINTER,
                        &args![
                            geometry,
                            first.wrapping_add(k.wrapping_mul(2)).wrapping_add(1)
                        ],
                    )
                    .u32();
                let far = e
                    .call(
                        GEOMETRY_COORD_POINTER,
                        &args![
                            geometry,
                            first.wrapping_add(k.wrapping_mul(2)).wrapping_add(3)
                        ],
                    )
                    .u32();
                fn_00b1add0(
                    e,
                    Ptr::new(edge_a),
                    x87_quiet(e.mem.f32(near)),
                    x87_quiet(e.mem.f32(near + 4)),
                    x87_quiet(e.mem.f32(near + 8)),
                );
                fn_00b1add0(
                    e,
                    Ptr::new(edge_b),
                    x87_quiet(e.mem.f32(far)),
                    x87_quiet(e.mem.f32(far + 4)),
                    x87_quiet(e.mem.f32(far + 8)),
                );
                let distance = e.call(VEC3_DISTANCE, &args![edge_a, edge_b]).f64();
                let total = distance + ld(e, right_total);
                st(e, right_total, total);
                e.call(FLOAT_VECTOR_PUSH, &args![right_lengths, right_total]);
                k += 1;
            }
            let geometry = e.get(this, CFrondEngine::m_pGeometry);
            e.set(
                geometry,
                CIndexedGeometry::m_nVertexCounter,
                start.wrapping_add(2) as u16,
            );
            let mut k = 1u32;
            while k < count.wrapping_sub(1) {
                let geometry = e.get(this, CFrondEngine::m_pGeometry);
                let first = e.mem.u32(g + 0x30);
                let pointer = e
                    .call(
                        GEOMETRY_TEX_COORD_POINTER,
                        &args![geometry, first.wrapping_add(k.wrapping_mul(2))],
                    )
                    .u32();
                let u = x87_quiet_bits(e.mem.u32(pointer));
                e.mem.set_u32(left_texture, u);
                let length = e
                    .call(FLOAT_VECTOR_AT, &args![left_lengths, k.wrapping_sub(1)])
                    .u32();
                let v = (f64::from(e.mem.f32(length)) / ld(e, left_total) * ld(e, TEXTURE_SCALE))
                    as f32;
                e.mem.set_f32(left_texture + 4, v);
                let map_index = u32::from(e.mem.u8(g + 0x20));
                e.call(
                    GEOMETRY_ADD_TEX_COORD,
                    &args![geometry, left_texture, map_index],
                );
                let geometry = e.get(this, CFrondEngine::m_pGeometry);
                let counter = e.get(geometry, CIndexedGeometry::m_nVertexCounter);
                e.set(
                    geometry,
                    CIndexedGeometry::m_nVertexCounter,
                    counter.wrapping_add(1),
                );
                let first = e.mem.u32(g + 0x30);
                let pointer = e
                    .call(
                        GEOMETRY_TEX_COORD_POINTER,
                        &args![
                            geometry,
                            first.wrapping_add(k.wrapping_mul(2)).wrapping_add(1)
                        ],
                    )
                    .u32();
                let u = x87_quiet_bits(e.mem.u32(pointer));
                e.mem.set_u32(right_texture, u);
                let length = e
                    .call(FLOAT_VECTOR_AT, &args![right_lengths, k.wrapping_sub(1)])
                    .u32();
                let v = (f64::from(e.mem.f32(length)) / ld(e, right_total) * ld(e, TEXTURE_SCALE))
                    as f32;
                e.mem.set_f32(right_texture + 4, v);
                let map_index = u32::from(e.mem.u8(g + 0x20));
                e.call(
                    GEOMETRY_ADD_TEX_COORD,
                    &args![geometry, right_texture, map_index],
                );
                let geometry = e.get(this, CFrondEngine::m_pGeometry);
                let counter = e.get(geometry, CIndexedGeometry::m_nVertexCounter);
                e.set(
                    geometry,
                    CIndexedGeometry::m_nVertexCounter,
                    counter.wrapping_add(1),
                );
                k += 1;
            }
            for _ in 0..2 {
                let geometry = e.get(this, CFrondEngine::m_pGeometry);
                let counter = e.get(geometry, CIndexedGeometry::m_nVertexCounter);
                e.set(
                    geometry,
                    CIndexedGeometry::m_nVertexCounter,
                    counter.wrapping_add(1),
                );
            }
            e.call(FLOAT_VECTOR_TIDY, &args![right_lengths]);
            e.call(FLOAT_VECTOR_RELEASE, &args![right_lengths]);
            e.call(FLOAT_VECTOR_TIDY, &args![left_lengths]);
            e.call(FLOAT_VECTOR_RELEASE, &args![left_lengths]);
            e.call(CVEC3_VECTOR_TIDY, &args![points]);
            e.call(PROXY_RELEASE, &args![points]);
            blade += 1;
        }
    });
}

// Translated from 00b1ada0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CVec3::CVec3(x, y, z)` (the engine map gives it no name): stores the three floats (each
/// through the x87 stack) and returns `this`.
pub fn fn_00b1ada0(e: &mut Engine, this: Ptr<CVec3>, x: f32, y: f32, z: f32) -> Ptr<CVec3> {
    let base = this.addr();
    e.mem.set_u32(base, x87_quiet_bits(x.to_bits()));
    e.mem.set_u32(base + 4, x87_quiet_bits(y.to_bits()));
    e.mem.set_u32(base + 8, x87_quiet_bits(z.to_bits()));
    this
}

// Translated from 00b1add0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores three floats (each through the x87 stack) at the `CVec3` `this` (the engine map gives
/// it no name; the code of `00b1ada0` without the return value).
pub fn fn_00b1add0(e: &mut Engine, this: Ptr<CVec3>, x: f32, y: f32, z: f32) {
    let base = this.addr();
    e.mem.set_u32(base, x87_quiet_bits(x.to_bits()));
    e.mem.set_u32(base + 4, x87_quiet_bits(y.to_bits()));
    e.mem.set_u32(base + 8, x87_quiet_bits(z.to_bits()));
}

// Translated from 00b1ae00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Multiplies `this` on the left by the rotation about the X axis,
/// `[1, 0, 0; 0, c, s; 0, -s, c]` with `c` and `s` the cosine and sine of the angle divided by
/// the angle divisor (`010a4d90`), each rounded to a float (the engine map gives it no name; the
/// exe returns nothing): `this = rotation * this` through the matrix product (`00b12940`).
pub fn fn_00b1ae00(e: &mut Engine, this: Ptr<CRotTransform>, angle: f32) {
    e.with_stack(0x48, |e, frame| {
        let rotation = frame.addr();
        let product = frame.addr() + 0x24;
        e.call(CROT_TRANSFORM_CONSTRUCT, &args![rotation]);
        let cosine = scaled_trig(e, COS, angle);
        let sine = scaled_trig(e, SIN, angle);
        e.mem.set_f32(rotation, 1.0);
        e.mem.set_f32(rotation + 4, 0.0);
        e.mem.set_f32(rotation + 8, 0.0);
        e.mem.set_f32(rotation + 12, 0.0);
        e.mem.set_f32(rotation + 16, cosine);
        e.mem.set_f32(rotation + 20, sine);
        e.mem.set_f32(rotation + 24, 0.0);
        e.mem.set_u32(rotation + 28, sine.to_bits() ^ 0x8000_0000);
        e.mem.set_f32(rotation + 32, cosine);
        let result = e
            .call(CROT_TRANSFORM_MULTIPLY, &args![rotation, product, this])
            .u32();
        copy_words(e, this.addr(), result, 9);
    });
}

// Translated from 00b1aed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// For every blade (`+0x3C`) of an engine that has a geometry (the engine map gives it no
/// name): builds the triangle-strip index list of one guide's vertex vector, `2 * vertices`
/// indices of the form `(blade * 2) * vertices + offset + i`, in a new buffer (`operator new`
/// of twice the 16-bit index count), and registers it as a strip of detail level `level` in the
/// geometry (`00b2fdc0`), then increments the geometry's strip counter (`+0x26`).
pub fn fn_00b1aed0(
    e: &mut Engine,
    this: Ptr<CFrondEngine>,
    level: u16,
    offset: u32,
    vertices: Ptr<StlVector>,
) {
    if e.get(this, CFrondEngine::m_pGeometry).is_null() {
        return;
    }
    let mut blade = 0u32;
    while blade < e.get(this, CFrondEngine::m_nNumBlades) {
        let length = (vector_len(e, vertices.addr(), 0x38) << 1) as u16;
        // `MUL` by 2 with the overflow of the 32-bit product turned into all ones: never
        // reached for a 16-bit count.
        let bytes = u32::from(length).checked_mul(2).unwrap_or(u32::MAX);
        let buffer = e.call(OPERATOR_NEW, &args![bytes]).u32();
        let mut written = 0u32;
        let mut index = 0u32;
        while index < vector_len(e, vertices.addr(), 0x38) << 1 {
            let per_blade = vector_len(e, vertices.addr(), 0x38);
            let value = (blade << 1)
                .wrapping_mul(per_blade)
                .wrapping_add(offset)
                .wrapping_add(index);
            e.mem.set_u16(buffer + written * 2, value as u16);
            written += 1;
            index += 1;
        }
        let geometry = e.get(this, CFrondEngine::m_pGeometry);
        e.call(
            GEOMETRY_ADD_STRIP,
            &args![geometry, u32::from(level), buffer, u32::from(length)],
        );
        let geometry = e.get(this, CFrondEngine::m_pGeometry);
        let strips = e.get(geometry, CIndexedGeometry::m_nStripCounter);
        e.set(
            geometry,
            CIndexedGeometry::m_nStripCounter,
            strips.wrapping_add(1),
        );
        blade += 1;
    }
}

// Translated from 00b1b000 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b1b000 - the FS:[0] exception frame (cleanup of the local vectors
// if a call throws) is not translated.
/// Writes the cross-section geometry of one guide (frond type 1; the engine map gives it no
/// name), for an engine that has a geometry and a lighting engine. The profile points and
/// normals of the guide's radius are built (`00b1be50`); the guide's vertices per guide vertex
/// (`+0x34`) is the number of profile points and its start index (`+0x30`) the geometry's
/// vertex counter. For each guide vertex and profile point, the profile point times the vertex's
/// transform and times the previous vertex's transform (both turned about X by the guide's
/// offset angle, `00b1ae00`) are averaged and added to the vertex position; the texture
/// coordinates are the positions of the point in the profile and of the vertex along the guide,
/// the colour is white, the normal is the averaged transformed profile normal, the tangent the
/// normalised sum of the two transformed profile points, and the binormal the normalised cross
/// product of the two; the wind weight follows when the geometry has vertex weighting. The
/// second pass rewrites the second texture coordinate of every vertex after the first ring as
/// the accumulated length along its profile column over the column's total, times `010a54e8`.
pub fn fn_00b1b000(e: &mut Engine, this: Ptr<CFrondEngine>, guide: Ptr<SFrondGuide>) {
    let g = guide.addr();
    if e.get(this, CFrondEngine::m_pGeometry).is_null()
        || e.get(this, CFrondEngine::m_pLightingEngine).is_null()
    {
        return;
    }
    e.with_stack(0x280, |e, frame| {
        let f = frame.addr();
        let normals = f;
        let column_lengths = f + 0x18;
        let column_totals = f + 0x30;
        let points = f + 0x48;
        let first_matrix = f + 0x60;
        let second_matrix = f + 0x84;
        let first_point = f + 0xA8;
        let second_point = f + 0xB4;
        let position = f + 0xC0;
        let tangent = f + 0xCC;
        let normal = f + 0xD8;
        let first_normal = f + 0xE4;
        let second_normal = f + 0xF0;
        let binormal = f + 0xFC;
        let texture = f + 0x108;
        let color = f + 0x110;
        let column = f + 0x120;
        let accumulated = f + 0x138;
        let zero = f + 0x13C;
        let edge_a = f + 0x140;
        let edge_b = f + 0x14C;
        let column_texture = f + 0x158;
        let scratch = |k: u32| f + 0x160 + 0x0C * k;
        e.call(CVEC3_VECTOR_CONSTRUCT, &args![points]);
        e.call(CVEC3_VECTOR_CONSTRUCT, &args![normals]);
        fn_00b1be50(e, this, guide, Ptr::new(points), Ptr::new(normals));
        let per_vertex = vector_len(e, points, 0x0C);
        e.mem.set_u32(g + 0x34, per_vertex);
        let geometry = e.get(this, CFrondEngine::m_pGeometry);
        let start = e.get(geometry, CIndexedGeometry::m_nVertexCounter);
        e.mem.set_u32(g + 0x30, u32::from(start));
        let one = e.global::<f64>(ONE_DOUBLE);
        let mut i = 0u32;
        while i < vector_len(e, g, 0x38) {
            let along = (f64::from(i) / (f64::from(vector_len(e, g, 0x38)) - one)) as f32;
            let mut j = 0u32;
            while j < e.mem.u32(g + 0x34) {
                let across = (f64::from(j) / (f64::from(e.mem.u32(g + 0x34)) - one)) as f32;
                let vertex = vector_at(e, g, i, 0x38);
                copy_words(e, first_matrix, vertex + 0x0C, 9);
                let previous = if i == 0 {
                    first_matrix
                } else {
                    vector_at(e, g, i.wrapping_sub(1), 0x38) + 0x0C
                };
                copy_words(e, second_matrix, previous, 9);
                let offset_angle = x87_quiet(e.mem.f32(g + 0x24));
                fn_00b1ae00(e, Ptr::new(first_matrix), offset_angle);
                let offset_angle = x87_quiet(e.mem.f32(g + 0x24));
                fn_00b1ae00(e, Ptr::new(second_matrix), offset_angle);
                let point = vector_at(e, points, j, 0x0C);
                e.call(
                    VEC3_TIMES_ROTATION,
                    &args![point, first_point, first_matrix],
                );
                let point = vector_at(e, points, j, 0x0C);
                e.call(
                    VEC3_TIMES_ROTATION,
                    &args![point, second_point, second_matrix],
                );
                let vertex = vector_at(e, g, i, 0x38);
                let half = x87_quiet_bits(e.global::<u32>(HALF_FLOAT));
                let sum = e
                    .call(VEC3_ADD, &args![first_point, scratch(0), second_point])
                    .u32();
                let average = e.call(VEC3_SCALE, &args![sum, scratch(1), half]).u32();
                e.call(VEC3_ADD, &args![vertex, position, average]);
                e.call(VEC3_ADD, &args![first_point, tangent, second_point]);
                e.call(VEC3_NORMALIZE, &args![tangent]);
                let geometry = e.get(this, CFrondEngine::m_pGeometry);
                e.call(GEOMETRY_ADD_COORDINATE, &args![geometry, position]);
                e.mem.set_f32(texture, x87_quiet(across));
                e.mem.set_f32(texture + 4, x87_quiet(along));
                let map_index = u32::from(e.mem.u8(g + 0x20));
                e.call(GEOMETRY_ADD_TEX_COORD, &args![geometry, texture, map_index]);
                let element = vector_at(e, normals, j, 0x0C);
                e.call(
                    VEC3_TIMES_ROTATION,
                    &args![element, first_normal, first_matrix],
                );
                let element = vector_at(e, normals, j, 0x0C);
                e.call(
                    VEC3_TIMES_ROTATION,
                    &args![element, second_normal, second_matrix],
                );
                let half = x87_quiet_bits(e.global::<u32>(HALF_FLOAT));
                let sum = e
                    .call(VEC3_ADD, &args![first_normal, scratch(2), second_normal])
                    .u32();
                e.call(VEC3_SCALE, &args![sum, normal, half]);
                e.call(VEC3_NORMALIZE, &args![normal]);
                for k in 0..4u32 {
                    e.mem.set_f32(color + 4 * k, 1.0);
                }
                e.call(GEOMETRY_ADD_COLOR, &args![geometry, color]);
                e.call(GEOMETRY_ADD_NORMAL, &args![geometry, normal]);
                e.call(GEOMETRY_ADD_TANGENT, &args![geometry, tangent]);
                e.call(VEC3_CROSS, &args![normal, binormal, tangent]);
                e.call(VEC3_NORMALIZE, &args![binormal]);
                e.call(GEOMETRY_ADD_BINORMAL, &args![geometry, binormal]);
                if e.get(geometry, CIndexedGeometry::m_bVertexWeighting) != 0 {
                    let element = fn_00b1db00(e, guide.cast(), i).addr();
                    let group = e.mem.u8(element + 0x34);
                    let element = fn_00b1db00(e, guide.cast(), i).addr();
                    let weight = x87_quiet_bits(e.mem.u32(element + 0x30));
                    e.call(
                        GEOMETRY_ADD_WIND_WEIGHT,
                        &args![geometry, weight, u32::from(group)],
                    );
                }
                let counter = e.get(geometry, CIndexedGeometry::m_nVertexCounter);
                e.set(
                    geometry,
                    CIndexedGeometry::m_nVertexCounter,
                    counter.wrapping_add(1),
                );
                j += 1;
            }
            i += 1;
        }
        // The accumulated length of every profile column.
        e.call(FLOAT_VECTOR_CONSTRUCT, &args![column_totals]);
        e.call(VECTOR_VECTOR_CONSTRUCT, &args![column_lengths]);
        let mut c = 0u32;
        while c < e.mem.u32(g + 0x34) {
            e.mem.set_f32(accumulated, 0.0);
            e.call(FLOAT_VECTOR_CONSTRUCT, &args![column]);
            e.mem.set_f32(zero, 0.0);
            e.call(FLOAT_VECTOR_PUSH, &args![column, zero]);
            let mut r = 1u32;
            while r < vector_len(e, g, 0x38) {
                let per_vertex = e.mem.u32(g + 0x34);
                let start = e.mem.u32(g + 0x30);
                let index_a = start
                    .wrapping_add(r.wrapping_mul(per_vertex))
                    .wrapping_add(c);
                let index_b = start
                    .wrapping_add(r.wrapping_sub(1).wrapping_mul(per_vertex))
                    .wrapping_add(c);
                let geometry = e.get(this, CFrondEngine::m_pGeometry);
                let pointer_a = e
                    .call(GEOMETRY_COORD_POINTER, &args![geometry, index_a])
                    .u32();
                let pointer_b = e
                    .call(GEOMETRY_COORD_POINTER, &args![geometry, index_b])
                    .u32();
                fn_00b1ada0(
                    e,
                    Ptr::new(edge_a),
                    x87_quiet(e.mem.f32(pointer_a)),
                    x87_quiet(e.mem.f32(pointer_a + 4)),
                    x87_quiet(e.mem.f32(pointer_a + 8)),
                );
                fn_00b1ada0(
                    e,
                    Ptr::new(edge_b),
                    x87_quiet(e.mem.f32(pointer_b)),
                    x87_quiet(e.mem.f32(pointer_b + 4)),
                    x87_quiet(e.mem.f32(pointer_b + 8)),
                );
                let distance = e.call(VEC3_DISTANCE, &args![edge_a, edge_b]).f64();
                let total = distance + ld(e, accumulated);
                st(e, accumulated, total);
                e.call(FLOAT_VECTOR_PUSH, &args![column, accumulated]);
                r += 1;
            }
            fn_00b1e640(e, Ptr::new(column_lengths), Ptr::new(column));
            e.call(FLOAT_VECTOR_PUSH, &args![column_totals, accumulated]);
            e.call(FLOAT_VECTOR_TIDY, &args![column]);
            e.call(FLOAT_VECTOR_RELEASE, &args![column]);
            c += 1;
        }
        let mut c = 0u32;
        while c < e.mem.u32(g + 0x34) {
            let mut r = 1u32;
            while r < vector_len(e, g, 0x38) {
                let per_vertex = e.mem.u32(g + 0x34);
                let start = e.mem.u32(g + 0x30);
                let index = start
                    .wrapping_add(r.wrapping_mul(per_vertex))
                    .wrapping_add(c) as u16;
                let geometry = e.get(this, CFrondEngine::m_pGeometry);
                e.set(geometry, CIndexedGeometry::m_nVertexCounter, index);
                let pointer = e
                    .call(
                        GEOMETRY_TEX_COORD_POINTER,
                        &args![geometry, u32::from(index)],
                    )
                    .u32();
                let across = x87_quiet_bits(e.mem.u32(pointer));
                e.mem.set_u32(column_texture, across);
                let row = vector_at(e, column_lengths, c, 0x18);
                let length = e.call(FLOAT_VECTOR_AT, &args![row, r]).u32();
                let total = e.call(FLOAT_VECTOR_AT, &args![column_totals, c]).u32();
                let along = (f64::from(e.mem.f32(length)) / f64::from(e.mem.f32(total))
                    * f64::from(e.global::<f32>(TEXTURE_SCALE))) as f32;
                e.mem.set_f32(column_texture + 4, along);
                let map_index = u32::from(e.mem.u8(g + 0x20));
                e.call(
                    GEOMETRY_ADD_TEX_COORD,
                    &args![geometry, column_texture, map_index],
                );
                let counter = e.get(geometry, CIndexedGeometry::m_nVertexCounter);
                e.set(
                    geometry,
                    CIndexedGeometry::m_nVertexCounter,
                    counter.wrapping_add(1),
                );
                r += 1;
            }
            c += 1;
        }
        e.call(FLOAT_VECTOR_VECTOR_TIDY, &args![column_lengths]);
        e.call(PROXY_RELEASE, &args![column_lengths]);
        e.call(FLOAT_VECTOR_TIDY, &args![column_totals]);
        e.call(FLOAT_VECTOR_RELEASE, &args![column_totals]);
        e.call(CVEC3_VECTOR_TIDY, &args![normals]);
        e.call(PROXY_RELEASE, &args![normals]);
        e.call(CVEC3_VECTOR_TIDY, &args![points]);
        e.call(PROXY_RELEASE, &args![points]);
    });
}

/// The index of a strip column of `fn_00b1b960`: `column * cross_step` truncated, the last
/// column and anything above the largest index (`verticesPerGuideVertex - 1`) clamped to it.
fn strip_column_index(
    e: &Engine,
    guide: u32,
    column: i32,
    cross_count: i32,
    cross_step: f32,
) -> u32 {
    let last = e.mem.u32(guide + 0x34).wrapping_sub(1);
    let mut index = truncate_low(f64::from(column) * f64::from(cross_step));
    if column == cross_count.wrapping_sub(1) {
        index = last;
    }
    if index > last {
        index = last;
    }
    index
}

// Translated from 00b1b960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the triangle-strip indices of one guide for the cross-section frond (frond type 1;
/// the engine map gives it no name) at detail level `level`, for an engine that has a geometry.
/// The segment counts are scaled down with the level: the length segments are
/// `s * (vertices - (minLength + 1)) + (minLength + 1)` and the cross segments
/// `s * (verticesPerGuideVertex - (2 * minCross + 1)) + (2 * minCross + 1)` (truncated, by
/// `_ftol2`), with `s = 1 - level / (levels - 1)` (1 for a single level). Each of the
/// `length segments - 1` rows takes the guide vertices at `row * step` and `(row + 1) * step`
/// (the last row, or a vertex index past the end, clamps to the final vertex) and gives, left to
/// right on even rows and right to left on odd rows, the pair of indices of the two rings
/// (`offset + ring * verticesPerGuideVertex + column`, the column `j * crossStep` clamped), then
/// repeats the last index to join the rows. The list is registered as a strip of the level
/// (`00b2fdc0`) and the geometry's strip counter incremented.
pub fn fn_00b1b960(
    e: &mut Engine,
    this: Ptr<CFrondEngine>,
    level: u32,
    offset: u32,
    guide: Ptr<SFrondGuide>,
) {
    let g = guide.addr();
    if e.get(this, CFrondEngine::m_pGeometry).is_null() {
        return;
    }
    let lod_count = e.get(this, CFrondEngine::m_nNumLodLevels);
    let scale = if lod_count == 1 {
        1.0f32
    } else {
        let ratio = f64::from(level) / (f64::from(lod_count) - e.global::<f64>(ONE_DOUBLE));
        (1.0 - ratio) as f32
    };
    let vertex_count = vector_len(e, g, 0x38);
    let minimum_length = f64::from(
        e.get(this, CFrondEngine::m_nMinLengthSegments)
            .wrapping_add(1),
    ) as f32;
    let length_value = (f64::from(scale) * (f64::from(vertex_count) - f64::from(minimum_length))
        + f64::from(minimum_length)) as f32;
    let length_count = ftol(e, f64::from(length_value));
    let length_step =
        (f64::from(vector_len(e, g, 0x38)) / f64::from(length_count.wrapping_sub(1))) as f32;
    let minimum_cross = f64::from(
        e.get(this, CFrondEngine::m_nMinCrossSegments)
            .wrapping_mul(2)
            .wrapping_add(1),
    ) as f32;
    let per_vertex = e.mem.u32(g + 0x34);
    let cross_value = (f64::from(scale) * (f64::from(per_vertex) - f64::from(minimum_cross))
        + f64::from(minimum_cross)) as f32;
    let cross_count = ftol(e, f64::from(cross_value));
    let cross_step =
        (f64::from(e.mem.u32(g + 0x34)) / f64::from(cross_count.wrapping_sub(1))) as f32;
    let rows = length_count.wrapping_sub(1);
    let total = cross_count
        .wrapping_mul(2)
        .wrapping_add(1)
        .wrapping_mul(rows)
        .wrapping_sub(1);
    // `MUL` by 2: an overflow of the 32-bit product gives all ones.
    let bytes = (total as u32).checked_mul(2).unwrap_or(u32::MAX);
    let buffer = e.call(OPERATOR_NEW, &args![bytes]).u32();
    let mut written = 0u32;
    let mut row = 0i32;
    while row < length_count.wrapping_sub(1) {
        let near = ftol(e, f64::from(row) * f64::from(length_step));
        let mut far = ftol(e, f64::from(row.wrapping_add(1)) * f64::from(length_step));
        let last_vertex = (vector_len(e, g, 0x38) as i32).wrapping_sub(1);
        if row == length_count.wrapping_sub(2) || far > last_vertex {
            far = (vector_len(e, g, 0x38) as i32).wrapping_sub(1);
        }
        let gap = far.wrapping_sub(near);
        let per_vertex = e.mem.u32(g + 0x34);
        let base = (near as u32).wrapping_mul(per_vertex);
        if row % 2 == 0 {
            let mut column = 0i32;
            while column < cross_count {
                let index = strip_column_index(e, g, column, cross_count, cross_step);
                let first = offset.wrapping_add(base).wrapping_add(index);
                e.mem.set_u16(buffer + 2 * written, first as u16);
                written += 1;
                let across = (gap as u32).wrapping_mul(e.mem.u32(g + 0x34));
                let second = offset
                    .wrapping_add(base)
                    .wrapping_add(index)
                    .wrapping_add(across);
                e.mem.set_u16(buffer + 2 * written, second as u16);
                written += 1;
                column += 1;
            }
        } else {
            let mut column = cross_count.wrapping_sub(1);
            while column >= 0 {
                let index = strip_column_index(e, g, column, cross_count, cross_step);
                let first = offset.wrapping_add(base).wrapping_add(index);
                e.mem.set_u16(buffer + 2 * written, first as u16);
                written += 1;
                let across = (gap as u32).wrapping_mul(e.mem.u32(g + 0x34));
                let second = offset
                    .wrapping_add(base)
                    .wrapping_add(index)
                    .wrapping_add(across);
                e.mem.set_u16(buffer + 2 * written, second as u16);
                written += 1;
                column -= 1;
            }
        }
        if row < length_count.wrapping_sub(2) {
            let previous = e.mem.u16(buffer + 2 * written.wrapping_sub(1));
            e.mem.set_u16(buffer + 2 * written, previous);
            written += 1;
        }
        row += 1;
    }
    let geometry = e.get(this, CFrondEngine::m_pGeometry);
    e.call(
        GEOMETRY_ADD_STRIP,
        &args![geometry, level & 0xFFFF, buffer, (total as u32) & 0xFFFF],
    );
    let geometry = e.get(this, CFrondEngine::m_pGeometry);
    let strips = e.get(geometry, CIndexedGeometry::m_nStripCounter);
    e.set(
        geometry,
        CIndexedGeometry::m_nStripCounter,
        strips.wrapping_add(1),
    );
}

/// The direction (as an angle) of the profile between two of its points (`00b1be50`): the
/// points are fetched with four checked accesses (the exe's two reads of each) and `atan2(dz,
/// dy)` is taken of the float differences of their Z and Y components, to, from.
fn profile_slope(e: &mut Engine, points: u32, from: u32, to: u32) -> f32 {
    let to_y = vector_at(e, points, to, 0x0C);
    let from_y = vector_at(e, points, from, 0x0C);
    let to_z = vector_at(e, points, to, 0x0C);
    let from_z = vector_at(e, points, from, 0x0C);
    let dz = (ld(e, to_z + 8) - ld(e, from_z + 8)) as f32;
    let dy = (ld(e, to_y + 4) - ld(e, from_y + 4)) as f32;
    e.call(ATAN2, &args![f64::from(dz), f64::from(dy)]).f32()
}

// Translated from 00b1be50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the profile of a cross-section frond for the guide `guide` (the engine map gives it no
/// name): `points` receives, for each of the profile's segments (`+0x44`) with `t = 1 - i /
/// (segments - 1)`, the point `(0, -t * radius, spline(t) * radius - spline(0) * radius)`
/// (`CVec3` pushes, `00b1e4c0`), then the same points mirrored (the second component times the
/// mirror factor) from the second to last back to the first. `normals` receives one normal per
/// point: `(0, cos(a), sin(a))` normalised, where `a` is the direction of the profile at the
/// point (`atan2` of the differences of the neighbouring points, the mean of the two for a
/// middle point, a fixed angle for the point at `segments - 1`) plus the angle offset at
/// `011f8c9c`.
pub fn fn_00b1be50(
    e: &mut Engine,
    this: Ptr<CFrondEngine>,
    guide: Ptr<SFrondGuide>,
    points: Ptr<StlVector>,
    normals: Ptr<StlVector>,
) {
    let g = guide.addr();
    let p = points.addr();
    let profile = e.get(this, CFrondEngine::m_pProfile).addr();
    let radius = ld(e, g + 0x1C);
    let offset = (e.call(SPLINE_EVALUATE, &args![profile, 0u32]).f64() * radius) as f32;
    let one = e.global::<f64>(ONE_DOUBLE);
    let mut i = 0u32;
    while i < e.get(this, CFrondEngine::m_nProfileSegments) {
        let segments = e.get(this, CFrondEngine::m_nProfileSegments);
        let t = (1.0 - f64::from(i) / (f64::from(segments) - one)) as f32;
        e.with_stack(0x0C, |e, point| {
            let at = point.addr();
            for k in [8u32, 4, 0] {
                e.mem.set_f32(at + k, 0.0);
            }
            e.mem.set_f32(at, 0.0);
            let y = (-f64::from(t) * ld(e, g + 0x1C)) as f32;
            e.mem.set_f32(at + 4, y);
            let spline = e.get(this, CFrondEngine::m_pProfile);
            let value = e
                .call(SPLINE_EVALUATE, &args![spline, x87_quiet_bits(t.to_bits())])
                .f64();
            let z = (value * ld(e, g + 0x1C) - f64::from(offset)) as f32;
            e.mem.set_f32(at + 8, z);
            e.call(CVEC3_VECTOR_PUSH, &args![points, point]);
        });
        i += 1;
    }
    let mut k = e
        .get(this, CFrondEngine::m_nProfileSegments)
        .wrapping_sub(2) as i32;
    while k >= 0 {
        let element = vector_at(e, p, k as u32, 0x0C);
        e.with_stack(0x0C, |e, copy| {
            copy_words(e, copy.addr(), element, 3);
            let mirrored = ld(e, copy.addr() + 4) * e.global::<f64>(MIRROR_FACTOR);
            st(e, copy.addr() + 4, mirrored);
            e.call(CVEC3_VECTOR_PUSH, &args![points, copy]);
        });
        k -= 1;
    }
    let mut i = 0u32;
    while i < vector_len(e, p, 0x0C) {
        let segments = e.get(this, CFrondEngine::m_nProfileSegments);
        let angle = if i == segments.wrapping_sub(1) {
            x87_quiet(e.global::<f32>(PROFILE_ANGLE_OFFSET))
        } else if i == 0 {
            let slope = profile_slope(e, p, 0, i.wrapping_add(1));
            (f64::from(slope) + f64::from(e.global::<f32>(PROFILE_ANGLE_OFFSET))) as f32
        } else if i == vector_len(e, p, 0x0C).wrapping_sub(1) {
            let slope = profile_slope(e, p, i.wrapping_sub(1), i);
            (f64::from(slope) + f64::from(e.global::<f32>(PROFILE_ANGLE_OFFSET))) as f32
        } else {
            let forward = profile_slope(e, p, i, i.wrapping_add(1));
            let backward = profile_slope(e, p, i.wrapping_sub(1), i);
            ((f64::from(backward) + f64::from(forward)) * e.global::<f64>(SCALE_DOUBLE)
                + f64::from(e.global::<f32>(PROFILE_ANGLE_OFFSET))) as f32
        };
        let sine = e.call(SIN, &args![f64::from(angle)]).f32();
        let cosine = e.call(COS, &args![f64::from(angle)]).f32();
        e.with_stack(0x0C, |e, normal| {
            let at = normal.addr();
            e.mem.set_f32(at, 0.0);
            e.mem.set_f32(at + 4, cosine);
            e.mem.set_f32(at + 8, sine);
            e.call(VEC3_NORMALIZE, &args![normal]);
            e.call(CVEC3_VECTOR_PUSH, &args![normals, normal]);
        });
        i += 1;
    }
}

// Translated from 00b1c6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the texture coordinates of every vertex of `geometry` (the engine map gives it no
/// name; `this` is not read): copies the eight floats at `coords`, negates the odd ones when
/// `flip` is set (`FCHS`), resets the geometry's vertex counter (`+0x22`) and for each vertex
/// (the coordinate vector's float count over three) calls `00b31070` with the texture index and
/// the floats, counting the vertex counter up.
pub fn fn_00b1c6d0(
    e: &mut Engine,
    _this: Ptr,
    geometry: Ptr<CIndexedGeometry>,
    texture_index: u8,
    coords: Ptr,
    flip: u8,
) {
    e.with_stack(0x20, |e, local| {
        e.call(MEMCPY, &args![local, coords, 0x20u32]);
        if flip != 0 {
            let mut k = 1u32;
            while k < 8 {
                let bits = x87_quiet_bits(e.mem.u32(local.addr() + 4 * k)) ^ 0x8000_0000;
                e.mem.set_u32(local.addr() + 4 * k, bits);
                k += 2;
            }
        }
        e.set(geometry, CIndexedGeometry::m_nVertexCounter, 0);
        let mut vertex = 0i32;
        loop {
            let floats =
                (e.mem
                    .u32(geometry.addr() + 0x88 + 0x10)
                    .wrapping_sub(e.mem.u32(geometry.addr() + 0x88 + 0x0C)) as i32
                    >> 2) as u32;
            let count = i32::from((floats / 3) as u16);
            if vertex >= count {
                break;
            }
            e.call(
                GEOMETRY_SET_TEX_COORDS,
                &args![geometry, u32::from(texture_index), local],
            );
            let counter = e.get(geometry, CIndexedGeometry::m_nVertexCounter);
            e.set(
                geometry,
                CIndexedGeometry::m_nVertexCounter,
                counter.wrapping_add(1),
            );
            vertex += 1;
        }
    });
}

// Translated from 00b1c7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The assignment of a `CIdvBezierSpline`'s fields (called from its constructor code at
/// `00b37794` and from `00b377c0`; the engine map gives it no name, so the identity is
/// unconfirmed): copies the three floats at `+0` (through the x87 stack) and the five vectors:
/// the `CVec` vectors at `+0x0C`, `+0x24`, `+0x54` and `+0x6C` (`00b1d130`) and the float
/// vector at `+0x3C` (`00b1d5d0`). Returns `this`.
pub fn fn_00b1c7a0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let (t, o) = (this.addr(), other.addr());
    for offset in [0u32, 4, 8] {
        copy_float(e, t + offset, o + offset);
    }
    fn_00b1d130(e, Ptr::new(t + 0x0C), Ptr::new(o + 0x0C));
    fn_00b1d130(e, Ptr::new(t + 0x24), Ptr::new(o + 0x24));
    fn_00b1d5d0(e, Ptr::new(t + 0x3C), Ptr::new(o + 0x3C));
    fn_00b1d130(e, Ptr::new(t + 0x54), Ptr::new(o + 0x54));
    fn_00b1d130(e, Ptr::new(t + 0x6C), Ptr::new(o + 0x6C));
    this
}

// Translated from 00b1c850 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b1c850 - the FS:[0] exception frame (cleanup of the local vectors
// and of the `CIdvRandom` if a call throws) is not translated.
/// `CFrondEngine::BuildGuideLods` (Xbox PDB; `fastcall`, `this` in ECX): reduces the guide
/// vector (`+8`) to the guide lists of every level of detail (`m_vGuideLods`, `+0x20`). It sums
/// the guides' surface areas (`+0x28`) and finds the largest. A guide whose area is at most the
/// largest times `1 - largeRetentionPercent` (`+0x78`) keeps a fuzzy area (`+0x2C`):
/// `r * largest^2 + (1 - r) * area` with `r = GetUniform(0, fuzziness)` (`+0x74`); any other
/// guide is copied to a local list and erased from the guide vector. Both lists are sorted
/// (`00b23cd0`) and the removed guides inserted at the front of the guide vector one by one.
/// For each lod level `l` (`+0x68` of them) the area budget is the total times
/// `max% + (min% - max%) * l / (levels - 1)` (`max%` alone for a single level); the guides, in
/// order, are copied to a list while the sum of their areas does not exceed the budget (a
/// budget not above zero gives an empty list), and the list is appended to the guide-lod
/// vector.
pub fn c_frond_engine_build_guide_lods(e: &mut Engine, this: Ptr<CFrondEngine>) {
    let base = this.addr();
    let guides = base + 8;
    let mut total = 0.0f32;
    let mut largest = 0.0f32;
    let mut i = 0u32;
    while i < vector_len(e, guides, 0x38) {
        let guide = vector_at(e, guides, i, 0x38);
        total = (f64::from(total) + ld(e, guide + 0x28)) as f32;
        let guide = vector_at(e, guides, i, 0x38);
        let area = e.mem.f32(guide + 0x28);
        if largest < area {
            let guide = vector_at(e, guides, i, 0x38);
            largest = x87_quiet(e.mem.f32(guide + 0x28));
        }
        i += 1;
    }
    let largest_squared = (f64::from(largest) * f64::from(largest)) as f32;
    e.with_stack(0x20, |e, frame| {
        let removed = frame.addr();
        let random = frame.addr() + 0x18;
        e.call(RANDOM_CONSTRUCT, &args![random]);
        let retention_gap = (1.0 - ld(e, base + 0x78)) as f32;
        fn_00b1d8a0(e, Ptr::new(removed));
        let mut i = 0u32;
        while i < vector_len(e, guides, 0x38) {
            let guide = vector_at(e, guides, i, 0x38);
            let area = x87_quiet(e.mem.f32(guide + 0x28));
            let threshold = f64::from(largest) * f64::from(retention_gap);
            if f64::from(area) > threshold {
                // Too small to keep: copy it to the removed list and erase it.
                let guide = vector_at(e, guides, i, 0x38);
                fn_00b1db40(e, Ptr::new(removed), Ptr::new(guide));
                let first = e.mem.u32(guides + 0x0C);
                e.with_stack(0x20, |e, iterators| {
                    let at = iterators.addr();
                    fn_00b42980(e, iterators, first, Ptr::new(guides));
                    let (proxy, position) = iterator_words(e, at);
                    e.mem.set_u32(at + 8, proxy);
                    e.mem.set_u32(at + 12, position);
                    e.call(ITERATOR_ADVANCE, &args![at + 8, i]);
                    let (proxy, position) = iterator_words(e, at + 8);
                    fn_00b1dd80(e, Ptr::new(guides), Ptr::new(at + 0x10), proxy, position);
                });
                i = i.wrapping_sub(1);
            } else {
                let high = x87_quiet_bits(e.mem.u32(base + 0x74));
                let draw = e.call(GET_UNIFORM, &args![random, 0u32, high]).f32();
                let complement = (1.0 - f64::from(draw)) as f32;
                let fuzzy = (f64::from(complement) * f64::from(area)
                    + f64::from(draw) * f64::from(largest_squared))
                    as f32;
                let guide = vector_at(e, guides, i, 0x38);
                e.mem.set_f32(guide + 0x2C, x87_quiet(fuzzy));
            }
            i = i.wrapping_add(1);
        }
        sort_guides(e, guides);
        sort_guides(e, removed);
        let mut k = 0u32;
        while k < vector_len(e, removed, 0x38) {
            e.with_stack(0x20, |e, iterators| {
                let at = iterators.addr();
                let first = e.mem.u32(guides + 0x0C);
                fn_00b42980(e, iterators, first, Ptr::new(guides));
                let (proxy, position) = iterator_words(e, at);
                let element = vector_at(e, removed, k, 0x38);
                fn_00b1dc50(
                    e,
                    Ptr::new(guides),
                    Ptr::new(at + 8),
                    proxy,
                    position,
                    Ptr::new(element),
                );
            });
            k += 1;
        }
        let mut level = 0u32;
        while level < e.get(this, CFrondEngine::m_nNumLodLevels) {
            let percent = if e.get(this, CFrondEngine::m_nNumLodLevels) == 1 {
                x87_quiet(e.get(this, CFrondEngine::m_fMaxSurfaceAreaPercent))
            } else {
                let maximum = x87_quiet(e.get(this, CFrondEngine::m_fMaxSurfaceAreaPercent));
                let last_level = e.get(this, CFrondEngine::m_nNumLodLevels).wrapping_sub(1);
                let t = (f64::from(level) / f64::from(last_level)) as f32;
                let minimum = ld(e, base + 0x70);
                (f64::from(t) * (minimum - f64::from(maximum)) + f64::from(maximum)) as f32
            };
            let budget = (f64::from(percent) * f64::from(total)) as f32;
            let mut accumulated = 0.0f32;
            e.with_stack(0x20, |e, lod| {
                let list = lod.addr();
                fn_00b1d8a0(e, Ptr::new(list));
                if f64::from(budget) > e.global::<f64>(ZERO_DOUBLE) {
                    let mut j = 0u32;
                    while j < vector_len(e, guides, 0x38)
                        && f64::from(accumulated) <= f64::from(budget)
                    {
                        let guide = vector_at(e, guides, j, 0x38);
                        accumulated = (f64::from(accumulated) + ld(e, guide + 0x28)) as f32;
                        let guide = vector_at(e, guides, j, 0x38);
                        fn_00b1db40(e, Ptr::new(list), Ptr::new(guide));
                        j += 1;
                    }
                }
                fn_00b1df80(e, Ptr::new(base + 0x20), Ptr::new(list));
                e.call(GUIDE_VECTOR_TIDY, &args![list]);
                e.call(PROXY_RELEASE, &args![list]);
            });
            level += 1;
        }
        e.call(GUIDE_VECTOR_TIDY, &args![removed]);
        e.call(PROXY_RELEASE, &args![removed]);
        e.call(TRIVIAL_DESTRUCTOR, &args![random]);
    });
}

/// `std::sort` of a whole guide vector (0x38-byte elements): the iterators at the last and at
/// the first element (`00b42980`), their distance (`00b1ff00`) and the sort itself
/// (`00b23cd0`: first position, last position, the element count, an empty comparison object).
fn sort_guides(e: &mut Engine, vector: u32) {
    e.with_stack(0x10, |e, iterators| {
        let last_iterator = iterators.addr();
        let first_iterator = iterators.addr() + 8;
        let last = e.mem.u32(vector + 0x10);
        fn_00b42980(e, iterators, last, Ptr::new(vector));
        let first = e.mem.u32(vector + 0x0C);
        fn_00b42980(e, Ptr::new(first_iterator), first, Ptr::new(vector));
        let count = fn_00b1ff00(e, Ptr::new(last_iterator), Ptr::new(first_iterator));
        let (_, last_position) = iterator_words(e, last_iterator);
        let (_, first_position) = iterator_words(e, first_iterator);
        e.call(
            GUIDE_SORT,
            &args![first_position, last_position, count as u32, 0u32],
        );
    });
}

// Translated from 00b1d100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SFrondTexture::SFrondTexture` (the engine map gives it no name): makes the file-name string
/// empty (`_Tidy(false, 0)`, `0044a730`); the other fields are left as they are. Returns `this`.
pub fn fn_00b1d100(e: &mut Engine, this: Ptr<SFrondTexture>) -> Ptr<SFrondTexture> {
    e.call(STRING_TIDY, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00b1d130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<CVec>::operator=` (the engine map gives it no name; 0x18-byte elements whose
/// destructor `00aa7830` does nothing). Nothing happens for a self-assignment; an empty source
/// clears the target (`00b1e750`); a source no longer than the target is copied over it
/// (`00b23e80`), the surplus elements destroyed and the end set after the copy; a source that
/// fits in the capacity is assigned over the existing elements (`00b236a0`) and the rest
/// constructed at the end (`00b25a90`); otherwise the old storage is destroyed and freed, the
/// source's size reserved (`00b1f590`) and the whole source constructed in it. Returns `this`.
pub fn fn_00b1d130(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) -> Ptr<StlVector> {
    let (t, o) = (this.addr(), other.addr());
    if t == o {
        return this;
    }
    if vector_len(e, o, 0x18) == 0 {
        e.call(VECTOR_CLEAR, &args![this]);
        return this;
    }
    let source = vector_len(e, o, 0x18);
    let current = vector_len(e, t, 0x18);
    if source <= current {
        let first = e.mem.u32(o + 0x0C);
        let last = e.mem.u32(o + 0x10);
        let target = e.mem.u32(t + 0x0C);
        // The three bytes the exe pushes after the three words are uninitialised stack.
        let copied_end = fn_00b23e80(e, first, last, target, 0, 0, 0);
        let old_last = e.mem.u32(t + 0x10);
        let mut element = copied_end;
        while element != old_last {
            e.call(TRIVIAL_DESTRUCTOR, &args![element]);
            // `XOR EAX,EAX; AND EAX,1`: the delete flag is the constant 0.
            element = element.wrapping_add(0x18);
        }
        let end = vector_len(e, o, 0x18)
            .wrapping_mul(0x18)
            .wrapping_add(e.mem.u32(t + 0x0C));
        e.mem.set_u32(t + 0x10, end);
        return this;
    }
    let capacity = vector_capacity(e, t, 0x18);
    if source <= capacity {
        let middle = vector_len(e, t, 0x18)
            .wrapping_mul(0x18)
            .wrapping_add(e.mem.u32(o + 0x0C));
        let (first, target) = (e.mem.u32(o + 0x0C), e.mem.u32(t + 0x0C));
        e.call(VECTOR_ASSIGN_RANGE, &args![first, middle, target]);
        let (end, last) = (e.mem.u32(t + 0x10), e.mem.u32(o + 0x10));
        let result = e
            .call(
                VECTOR_CONSTRUCT_RANGE,
                &args![middle, last, end, t + 8, 0u32, 0u32],
            )
            .u32();
        e.mem.set_u32(t + 0x10, result);
        return this;
    }
    if e.mem.u32(t + 0x0C) != 0 {
        let first = e.mem.u32(t + 0x0C);
        let last = e.mem.u32(t + 0x10);
        let mut element = first;
        while element != last {
            e.call(TRIVIAL_DESTRUCTOR, &args![element]);
            element = element.wrapping_add(0x18);
        }
        e.call(OPERATOR_DELETE, &args![first]);
    }
    let size = vector_len(e, o, 0x18);
    if e.call(VECTOR_RESERVE, &args![this, size]).bool() {
        let target = e.mem.u32(t + 0x0C);
        let (first, last) = (e.mem.u32(o + 0x0C), e.mem.u32(o + 0x10));
        let result = e
            .call(
                VECTOR_CONSTRUCT_RANGE,
                &args![first, last, target, t + 8, 0u32, 0u32],
            )
            .u32();
        e.mem.set_u32(t + 0x10, result);
    }
    this
}

// Translated from 00b1d5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<float>::operator=` (the engine map gives it no name; 4-byte elements): nothing
/// happens for a self-assignment; an empty source clears the target (`0044a0a0`); a source no
/// longer than the target is copied over it (`0044d600`: first, last, destination) and the end
/// set after the copy; a source that fits in the capacity is copied over the existing elements
/// and the rest constructed at the end (`0060c320`: first, last, destination, the allocator at
/// `+8`); otherwise the old storage is freed, the new size reserved (`0044a240`) and the whole
/// source constructed in it. Returns `this`.
pub fn fn_00b1d5d0(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) -> Ptr<StlVector> {
    let (t, o) = (this.addr(), other.addr());
    if t == o {
        return this;
    }
    if vector_len(e, o, 4) == 0 {
        e.call(FLOAT_VECTOR_CLEAR, &args![this]);
        return this;
    }
    let source = vector_len(e, o, 4);
    let current = vector_len(e, t, 4);
    if source <= current {
        let (first, last, target) = (
            e.mem.u32(o + 0x0C),
            e.mem.u32(o + 0x10),
            e.mem.u32(t + 0x0C),
        );
        e.call(FLOAT_COPY_RANGE, &args![first, last, target]);
        let end = e
            .mem
            .u32(t + 0x0C)
            .wrapping_add(vector_len(e, o, 4).wrapping_mul(4));
        e.mem.set_u32(t + 0x10, end);
        return this;
    }
    let capacity = vector_capacity(e, t, 4);
    if source <= capacity {
        let middle = e
            .mem
            .u32(o + 0x0C)
            .wrapping_add(vector_len(e, t, 4).wrapping_mul(4));
        let (first, target) = (e.mem.u32(o + 0x0C), e.mem.u32(t + 0x0C));
        e.call(FLOAT_COPY_RANGE, &args![first, middle, target]);
        let (end, last) = (e.mem.u32(t + 0x10), e.mem.u32(o + 0x10));
        let result = e
            .call(FLOAT_CONSTRUCT_RANGE, &args![middle, last, end, t + 8])
            .u32();
        e.mem.set_u32(t + 0x10, result);
    } else {
        if e.mem.u32(t + 0x0C) != 0 {
            let storage = e.mem.u32(t + 0x0C);
            e.call(OPERATOR_DELETE, &args![storage]);
        }
        let size = vector_len(e, o, 4);
        if e.call(FLOAT_RESERVE, &args![this, size]).bool() {
            let (target, last, first) = (
                e.mem.u32(t + 0x0C),
                e.mem.u32(o + 0x10),
                e.mem.u32(o + 0x0C),
            );
            let result = e
                .call(FLOAT_CONSTRUCT_RANGE, &args![first, last, target, t + 8])
                .u32();
            e.mem.set_u32(t + 0x10, result);
        }
    }
    this
}

// Translated from 00b1d8a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b1d8a0 - the FS:[0] exception frame (cleanup state of the
// constructor) is not translated.
/// The constructor of the guide vector (`std::vector<SFrondGuide>`, the engine map gives it no
/// name): allocates the checked-iterator proxy pointing back at the vector and reserves
/// nothing (`00b1eb00`, 0). Returns `this`.
pub fn fn_00b1d8a0(e: &mut Engine, this: Ptr<StlVector>) -> Ptr<StlVector> {
    construct_vector(e, this, 0x00b1_eb00)
}

// Translated from 00b1d980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `push_back` of the vertex vector (`std::vector<SFrondVertex>`, 0x38-byte elements; the engine
/// map gives it no name): constructs the element at `value` in spare capacity (`00b23f60`) or
/// inserts it at the end (`00b1e840`).
pub fn fn_00b1d980(e: &mut Engine, this: Ptr<StlVector>, value: Ptr) {
    push_back_element(e, this, value, 0x38, 0x00b2_3f60, 0x00b1_e840);
}

// Translated from 00b1da90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b1da90 - the FS:[0] exception frame (cleanup state of the member
// destructor) is not translated.
/// The destructor of the guide vector (the engine map gives it no name): tidies it (`00b1ec70`)
/// and frees its proxy.
pub fn fn_00b1da90(e: &mut Engine, this: Ptr<StlVector>) {
    e.call(GUIDE_VECTOR_TIDY, &args![this]);
    let proxy = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00b1db00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked `operator[]` of the 0x38-byte-element vectors (the engine map gives it no name):
/// the address of element `index`; an index not below the element count takes the
/// invalid-parameter path.
pub fn fn_00b1db00(e: &mut Engine, this: Ptr<StlVector>, index: u32) -> Ptr {
    Ptr::new(vector_at(e, this.addr(), index, 0x38))
}

// Translated from 00b1db40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `push_back` of the guide vector (`std::vector<SFrondGuide>`, 0x38-byte elements; the engine
/// map gives it no name): constructs the element at `value` in spare capacity (`00b24180`) or
/// inserts it at the end (`00b1dc50`).
pub fn fn_00b1db40(e: &mut Engine, this: Ptr<StlVector>, value: Ptr) {
    push_back_element(e, this, value, 0x38, 0x00b2_4180, 0x00b1_dc50);
}

// Translated from 00b1dc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked insert of one guide before the iterator (`proxy`, `position`) (the engine map
/// gives it no name): inserts with `00b1ecf0` and writes to `out` the iterator at the same index
/// afterwards (advanced with `00b23460`). Returns `out`.
pub fn fn_00b1dc50(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: Ptr,
) -> Ptr {
    insert_one(
        e,
        this,
        out,
        proxy,
        position,
        value,
        0x38,
        0x00b1_ecf0,
        0x00b2_3460,
    )
}

// Translated from 00b1dd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The erase of one element of the guide vector at the iterator position `position` (the
/// engine map gives it no name): moves the later elements down (`00b23760`), destroys the last
/// element (`00b1ebc0`), shortens the vector by one element and writes the iterator at the
/// same position to `out` (an invalid position takes the invalid-parameter path). Returns `out`.
pub fn fn_00b1dd80(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    _unused_2: u32,
    position: u32,
) -> Ptr {
    let base = this.addr();
    let last = e.mem.u32(base + 0x10);
    e.call(
        GUIDE_MOVE_DOWN,
        &args![position.wrapping_add(0x38), last, position],
    );
    let last = e.mem.u32(base + 0x10);
    e.call(
        GUIDE_DESTROY_RANGE,
        &args![this, last.wrapping_sub(0x38), last],
    );
    let last = e.mem.u32(base + 0x10).wrapping_sub(0x38);
    e.mem.set_u32(base + 0x10, last);
    e.mem.set_u32(out.addr(), 0);
    let proxy = checked_iterator(e, base, position);
    e.mem.set_u32(out.addr(), proxy);
    e.mem.set_u32(out.addr() + 4, position);
    out
}

// Translated from 00b1de20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `clear` of the guide vector (the engine map gives it no name): erases the whole range with
/// `00b1ea20`.
pub fn fn_00b1de20(e: &mut Engine, this: Ptr<StlVector>) {
    erase_all(e, this, 0x00b1_ea20);
}

// Translated from 00b1df10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b1df10 - the FS:[0] exception frame (cleanup state of the member
// destructor) is not translated.
/// The destructor of the guide-lod vector (a vector of guide vectors; the engine map gives it no
/// name): tidies it (`00b1f650`) and frees its proxy.
pub fn fn_00b1df10(e: &mut Engine, this: Ptr<StlVector>) {
    fn_00b1f650(e, this);
    let proxy = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00b1df80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `push_back` of the guide-lod vector (a vector of guide vectors, 0x18-byte elements; the
/// engine map gives it no name): constructs the element at `value` in spare capacity
/// (`00b24290`) or inserts it at the end (`00b1f460`).
pub fn fn_00b1df80(e: &mut Engine, this: Ptr<StlVector>, value: Ptr) {
    push_back_element(e, this, value, 0x18, 0x00b2_4290, 0x00b1_f460);
}

// Translated from 00b1e090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b1e090 - the FS:[0] exception frame (cleanup state of the
// constructor) is not translated.
/// The constructor of the texture vector (`std::vector<SFrondTexture>`, the engine map gives it
/// no name): allocates the checked-iterator proxy pointing back at the vector and reserves
/// nothing (`00b1f920`, 0). Returns `this`.
pub fn fn_00b1e090(e: &mut Engine, this: Ptr<StlVector>) -> Ptr<StlVector> {
    construct_vector(e, this, 0x00b1_f920)
}

// Translated from 00b1e170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b1e170 - the FS:[0] exception frame (cleanup state of the member
// destructor) is not translated.
/// The destructor of the texture vector (the engine map gives it no name): tidies it
/// (`00b1f9e0`) and frees its proxy.
pub fn fn_00b1e170(e: &mut Engine, this: Ptr<StlVector>) {
    fn_00b1f9e0(e, this);
    let proxy = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00b1e1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `push_back` of the texture vector (0x2C-byte elements; the engine map gives it no name):
/// constructs the element at `value` in spare capacity (`00b243a0`) or inserts it at the end
/// (`00b1f6d0`).
pub fn fn_00b1e1e0(e: &mut Engine, this: Ptr<StlVector>, value: Ptr) {
    push_back_element(e, this, value, 0x2C, 0x00b2_43a0, 0x00b1_f6d0);
}

// Translated from 00b1e2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<SFrondTexture>::clear` (engine map): erases the whole range with `00b1f800`.
pub fn fn_00b1e2f0(e: &mut Engine, this: Ptr<StlVector>) {
    erase_all(e, this, 0x00b1_f800);
}

// Translated from 00b1e640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `push_back` of the vector of float vectors (0x18-byte elements; the engine map gives it no
/// name): constructs the element at `value` in spare capacity (`00b24580`) or inserts it at the
/// end (`00b1fd50`).
pub fn fn_00b1e640(e: &mut Engine, this: Ptr<StlVector>, value: Ptr) {
    push_back_element(e, this, value, 0x18, 0x00b2_4580, 0x00b1_fd50);
}

// Translated from 00b1e840 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked insert of one vertex before the iterator (`proxy`, `position`) of a vector with
/// 0x38-byte elements (the engine map gives it no name): inserts with `00b20360` and writes to
/// `out` the iterator at the same index afterwards (advanced with `00b23460`). Returns `out`.
pub fn fn_00b1e840(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: Ptr,
) -> Ptr {
    insert_one(
        e,
        this,
        out,
        proxy,
        position,
        value,
        0x38,
        0x00b2_0360,
        0x00b2_3460,
    )
}

// Translated from 00b1ea20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The erase of the range [`first_position`, `last_position`) of the guide vector (the engine
/// map gives it no name; the proxy arguments are not read): the iterators of both positions
/// are built with the vector's own proxy and, when they differ (`00b16830`), the elements
/// after the range are moved down (`00b23760`), the vacated tail destroyed (`00b1ebc0`) and the
/// vector shortened. `out` receives the iterator at `first_position`. Returns `out`.
pub fn fn_00b1ea20(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    _unused_2: u32,
    first_position: u32,
    _unused_4: u32,
    last_position: u32,
) -> Ptr {
    let base = this.addr();
    let first_proxy = checked_iterator(e, base, first_position);
    let last_proxy = checked_iterator(e, base, last_position);
    e.with_stack(0x10, |e, frame| {
        let first = frame.addr();
        let last = frame.addr() + 8;
        e.mem.set_u32(first, first_proxy);
        e.mem.set_u32(first + 4, first_position);
        e.mem.set_u32(last, last_proxy);
        e.mem.set_u32(last + 4, last_position);
        if e.call(ITERATORS_DIFFER, &args![first, last]).bool() {
            let end = e.mem.u32(base + 0x10);
            let new_end = e
                .call(GUIDE_MOVE_DOWN, &args![last_position, end, first_position])
                .u32();
            let end = e.mem.u32(base + 0x10);
            e.call(GUIDE_DESTROY_RANGE, &args![this, new_end, end]);
            e.mem.set_u32(base + 0x10, new_end);
        }
        let (proxy, position) = iterator_words(e, first);
        e.mem.set_u32(out.addr(), proxy);
        e.mem.set_u32(out.addr() + 4, position);
    });
    out
}

// Translated from 00b1ecf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b1ecf0 - the FS:[0] exception frame (the handler that frees the new
// storage and destroys the copied elements if a copy throws, and the cleanup of the temporary
// guide) is not translated.
/// The insertion of `count` copies of the guide at `value` before the position `position` of
/// the guide vector (`std::vector<SFrondGuide>::_Insert_n`; the engine map gives it no name;
/// the first argument, a proxy, is not read). A count of zero does nothing; a result above the
/// maximum size (`0x4924924` elements) raises the length error (`00b0fb80`). Without room the
/// vector grows to one and a half times its capacity (at least the new size, `00b23980`): the
/// elements before the position are copied to the new storage (`00b26450`), the copies of the
/// value constructed (`00b24180`) and the later elements copied after them (`00b24030`), the old
/// elements destroyed (`00b1ebc0`) and its storage freed. With room, a temporary copy of the
/// value is made (`00b1ff50`); when fewer elements follow the position than are inserted the
/// tail moves up (`00b26450`), copies of the value are constructed past the old end
/// (`00b24180`) and the rest of the range assigned (`00b249d0`); otherwise the last `count`
/// elements are constructed past the end (`00b24030`), the others moved up (`00b238e0`) and the
/// range assigned.
pub fn fn_00b1ecf0(
    e: &mut Engine,
    this: Ptr<StlVector>,
    _unused_1: u32,
    position: u32,
    count: u32,
    value: Ptr,
) {
    let t = this.addr();
    let mut capacity = vector_capacity(e, t, 0x38);
    if count == 0 {
        return;
    }
    // The maximum element count: `0xFFFFFFFF / 0x38` (at least one).
    let limit = GUIDE_LIMIT.max(1);
    let size = vector_len(e, t, 0x38);
    if limit.wrapping_sub(size) < count {
        e.call(LENGTH_ERROR, &args![]);
        return;
    }
    let required = vector_len(e, t, 0x38).wrapping_add(count);
    if capacity < required {
        let limit = GUIDE_LIMIT.max(1);
        capacity = if limit.wrapping_sub(capacity >> 1) < capacity {
            0
        } else {
            (capacity >> 1).wrapping_add(capacity)
        };
        let required = vector_len(e, t, 0x38).wrapping_add(count);
        if capacity < required {
            capacity = required;
        }
        let storage = e.call(GUIDE_ALLOCATE, &args![capacity, 0u32]).u32();
        let first = e.mem.u32(t + 0x0C);
        let copied = e
            .call(GUIDE_RELOCATE_COPY, &args![first, position, storage, t + 8])
            .u32();
        e.call(GUIDE_FILL, &args![copied, count, value, t + 8, 0u32, 0u32]);
        let filled = count.wrapping_mul(0x38).wrapping_add(copied);
        let last = e.mem.u32(t + 0x10);
        e.call(GUIDE_TAIL_COPY, &args![position, last, filled, t + 8]);
        let new_size = vector_len(e, t, 0x38).wrapping_add(count);
        if e.mem.u32(t + 0x0C) != 0 {
            let first = e.mem.u32(t + 0x0C);
            let last = e.mem.u32(t + 0x10);
            e.call(GUIDE_DESTROY_RANGE, &args![this, first, last]);
            e.call(OPERATOR_DELETE, &args![first]);
        }
        e.mem
            .set_u32(t + 0x14, capacity.wrapping_mul(0x38).wrapping_add(storage));
        e.mem
            .set_u32(t + 0x10, new_size.wrapping_mul(0x38).wrapping_add(storage));
        e.mem.set_u32(t + 0x0C, storage);
        return;
    }
    let after = (e.mem.u32(t + 0x10).wrapping_sub(position) as i32 / 0x38) as u32;
    e.with_stack(0x38, |e, temporary| {
        let temp = temporary.addr();
        e.call(GUIDE_COPY_CONSTRUCT, &args![temp, value]);
        if after < count {
            // Fewer elements follow the position than are inserted.
            let last = e.mem.u32(t + 0x10);
            e.call(
                GUIDE_RELOCATE_COPY,
                &args![
                    position,
                    last,
                    count.wrapping_mul(0x38).wrapping_add(position),
                    t + 8
                ],
            );
            let last = e.mem.u32(t + 0x10);
            let surplus = count.wrapping_sub((last.wrapping_sub(position) as i32 / 0x38) as u32);
            e.call(GUIDE_FILL, &args![last, surplus, temp, t + 8, 0u32, 0u32]);
            let end = count.wrapping_mul(0x38).wrapping_add(e.mem.u32(t + 0x10));
            e.mem.set_u32(t + 0x10, end);
            let stop = e.mem.u32(t + 0x10).wrapping_sub(count.wrapping_mul(0x38));
            let mut element = position;
            while element != stop {
                e.call(GUIDE_ASSIGN, &args![element, temp]);
                element = element.wrapping_add(0x38);
            }
            e.call(VERTEX_VECTOR_TIDY, &args![temp]);
            e.call(PROXY_RELEASE, &args![temp]);
        } else {
            let last = e.mem.u32(t + 0x10);
            let tail = last.wrapping_sub(count.wrapping_mul(0x38));
            let end = e
                .call(GUIDE_TAIL_COPY, &args![tail, last, last, t + 8])
                .u32();
            e.mem.set_u32(t + 0x10, end);
            e.call(GUIDE_MOVE_UP, &args![position, tail, last]);
            let stop = count.wrapping_mul(0x38).wrapping_add(position);
            let mut element = position;
            while element != stop {
                e.call(GUIDE_ASSIGN, &args![element, temp]);
                element = element.wrapping_add(0x38);
            }
            e.call(VERTEX_VECTOR_TIDY, &args![temp]);
            let proxy = e.mem.u32(temp);
            e.call(OPERATOR_DELETE, &args![proxy]);
        }
    });
}

// Translated from 00b1f460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked insert of one guide vector before the iterator (`proxy`, `position`) of the
/// guide-lod vector (0x18-byte elements; the engine map gives it no name): inserts with
/// `00b20ce0` and writes to `out` the iterator at the same index afterwards (advanced with
/// `00b16890`). Returns `out`.
pub fn fn_00b1f460(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: Ptr,
) -> Ptr {
    insert_one(
        e,
        this,
        out,
        proxy,
        position,
        value,
        0x18,
        0x00b2_0ce0,
        0x00b1_6890,
    )
}

// Translated from 00b1f650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tidies the guide-lod vector (0x18-byte elements; the engine map gives it no name): when it
/// has storage, destroys the elements (`00b20c30`: first, last) and frees the storage, then
/// zeroes the first, last and end pointers.
pub fn fn_00b1f650(e: &mut Engine, this: Ptr<StlVector>) {
    let base = this.addr();
    if e.mem.u32(base + 0x0C) != 0 {
        let first = e.mem.u32(base + 0x0C);
        let last = e.mem.u32(base + 0x10);
        e.call(GUIDE_LOD_DESTROY_RANGE, &args![this, first, last]);
        let storage = e.mem.u32(base + 0x0C);
        e.call(OPERATOR_DELETE, &args![storage]);
    }
    e.mem.set_u32(base + 0x0C, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x14, 0);
}

// Translated from 00b1f6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked insert of one texture before the iterator (`proxy`, `position`) of the texture
/// vector (0x2C-byte elements; the engine map gives it no name): inserts with `00b21560` and
/// writes to `out` the iterator at the same index afterwards (advanced with `00b23520`).
/// Returns `out`.
pub fn fn_00b1f6d0(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: Ptr,
) -> Ptr {
    insert_one(
        e,
        this,
        out,
        proxy,
        position,
        value,
        0x2C,
        0x00b2_1560,
        0x00b2_3520,
    )
}

// Translated from 00b1f800 (decompiled, FalloutNV.exe 1.4.0.525)
/// The erase of the range [`first_position`, `last_position`) of the texture vector (the engine
/// map gives it no name; the proxy arguments are not read): the iterators of both positions
/// are built with the vector's own proxy and, when they differ (`00b16830`), the elements after
/// the range are moved down (`00b23830`), the vacated tail's strings destroyed (`_Tidy(1, 0)`)
/// and the vector shortened. `out` receives the iterator at `first_position`. Returns `out`.
pub fn fn_00b1f800(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    _unused_2: u32,
    first_position: u32,
    _unused_4: u32,
    last_position: u32,
) -> Ptr {
    let base = this.addr();
    let first_proxy = checked_iterator(e, base, first_position);
    let last_proxy = checked_iterator(e, base, last_position);
    e.with_stack(0x10, |e, frame| {
        let first = frame.addr();
        let last = frame.addr() + 8;
        e.mem.set_u32(first, first_proxy);
        e.mem.set_u32(first + 4, first_position);
        e.mem.set_u32(last, last_proxy);
        e.mem.set_u32(last + 4, last_position);
        if e.call(ITERATORS_DIFFER, &args![first, last]).bool() {
            let end = e.mem.u32(base + 0x10);
            let new_end = e
                .call(
                    TEXTURE_MOVE_DOWN,
                    &args![last_position, end, first_position],
                )
                .u32();
            let old_end = e.mem.u32(base + 0x10);
            let mut element = new_end;
            while element != old_end {
                e.call(STRING_TIDY, &args![element, 1u32, 0u32]);
                // `XOR EDX,EDX; AND EDX,1`: the delete flag is the constant 0.
                element = element.wrapping_add(0x2C);
            }
            e.mem.set_u32(base + 0x10, new_end);
        }
        let (proxy, position) = iterator_words(e, first);
        e.mem.set_u32(out.addr(), proxy);
        e.mem.set_u32(out.addr() + 4, position);
    });
    out
}

// Translated from 00b1f920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The storage reservation of the texture vector (the engine map gives it no name; `_Buy`):
/// clears the first, last and end pointers; a count of zero returns false; a count above the
/// maximum (`0x5D1745D` elements) raises the length error (`00b0fb80`); otherwise it allocates
/// `count` elements (`00b23b50`) and sets first, last and end (`count * 0x2C` further).
/// Returns true unless the count was zero.
pub fn fn_00b1f920(e: &mut Engine, this: Ptr<StlVector>, count: u32) -> bool {
    let base = this.addr();
    e.mem.set_u32(base + 0x0C, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x14, 0);
    if count == 0 {
        return false;
    }
    // The maximum element count, `0xFFFFFFFF / 0x2C` (at least one).
    let limit = TEXTURE_LIMIT.max(1);
    if limit < count {
        e.call(LENGTH_ERROR, &args![]);
        return true;
    }
    let storage = e.call(ALLOCATE_TEXTURES, &args![count, 0u32]).u32();
    e.mem.set_u32(base + 0x0C, storage);
    e.mem.set_u32(base + 0x10, storage);
    e.mem
        .set_u32(base + 0x14, count.wrapping_mul(0x2C).wrapping_add(storage));
    true
}

// Translated from 00b1f9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tidies the texture vector (the engine map gives it no name): when it has storage, destroys
/// the file-name string of each element (`_Tidy(1, 0)`) and frees the storage, then zeroes the
/// first, last and end pointers.
pub fn fn_00b1f9e0(e: &mut Engine, this: Ptr<StlVector>) {
    let base = this.addr();
    if e.mem.u32(base + 0x0C) != 0 {
        let first = e.mem.u32(base + 0x0C);
        let last = e.mem.u32(base + 0x10);
        let mut element = first;
        while element != last {
            e.call(STRING_TIDY, &args![element, 1u32, 0u32]);
            // `XOR EAX,EAX; AND EAX,1`: the delete flag is the constant 0.
            element = element.wrapping_add(0x2C);
        }
        let storage = e.mem.u32(base + 0x0C);
        e.call(OPERATOR_DELETE, &args![storage]);
    }
    e.mem.set_u32(base + 0x0C, 0);
    e.mem.set_u32(base + 0x10, 0);
    e.mem.set_u32(base + 0x14, 0);
}

// Translated from 00b1ff00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The difference of two checked iterators of the guide vector (the engine map gives it no
/// name; `this` and `other` are the iterators, proxy at `+0`, position at `+4`): both must have
/// the same non-null proxy (otherwise the invalid-parameter path); the result is the position
/// difference over 0x38.
pub fn fn_00b1ff00(e: &mut Engine, this: Ptr, other: Ptr) -> i32 {
    let proxy = e.mem.u32(this.addr());
    if proxy == 0 || proxy != e.mem.u32(other.addr()) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let bytes = e
        .mem
        .u32(this.addr() + 4)
        .wrapping_sub(e.mem.u32(other.addr() + 4)) as i32;
    bytes / 0x38
}

// Translated from 00b23e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 0x18-byte elements [`first`, `last`) to `destination` one after the other (six
/// words each; the engine map gives it no name; `cdecl`, six words, the last three never read)
/// and returns the end of the copy, `destination + count * 0x18`.
pub fn fn_00b23e80(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let count = (last.wrapping_sub(first) as i32) / 0x18;
    let end = (count as u32).wrapping_mul(0x18).wrapping_add(destination);
    let mut source = first;
    let mut target = destination;
    while source != last {
        copy_words(e, target, source, 6);
        source = source.wrapping_add(0x18);
        target = target.wrapping_add(0x18);
    }
    end
}

// Translated from 00b42980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Vector_const_iterator<SFrondGuide>`'s constructor (Xbox PDB name): the iterator at
/// `this` (proxy at `+0`, position at `+4`) for `position` in `vector`; clears the proxy first,
/// takes the invalid-parameter path unless the vector exists and `position` lies between its
/// first and last pointers, then stores the vector's proxy and the position. Returns `this`.
pub fn fn_00b42980(e: &mut Engine, this: Ptr, position: u32, vector: Ptr<StlVector>) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    let proxy = checked_iterator(e, vector.addr(), position);
    e.mem.set_u32(this.addr(), proxy);
    e.mem.set_u32(this.addr() + 4, position);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b17fd0,
            c_frond_engine_c_frond_engine(Ptr<CFrondEngine>) -> Ptr<CFrondEngine>
        ),
        entry!(0x00b181b0, fn_00b181b0(Ptr<CFrondEngine>)),
        entry!(0x00b18300, fn_00b18300(Ptr<CFrondEngine>) -> u32),
        entry!(0x00b18320, fn_00b18320(Ptr<CFrondEngine>) -> u8),
        entry!(0x00b18340, c_frond_engine_parse(Ptr<CFrondEngine>, Ptr)),
        (0x00b18860, idv_format_string_entry as AbiFn),
        entry!(0x00b188d0, fn_00b188d0(Ptr<CFrondEngine>)),
        entry!(
            0x00b18970,
            fn_00b18970(Ptr<SFrondGuide>) -> Ptr<SFrondGuide>
        ),
        entry!(0x00b189f0, fn_00b189f0(Ptr<SFrondGuide>)),
        entry!(
            0x00b18a60,
            fn_00b18a60(
                Ptr<CFrondEngine>,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                f32,
                u32,
            )
        ),
        entry!(
            0x00b18b20,
            fn_00b18b20(Ptr<SFrondVertex>) -> Ptr<SFrondVertex>
        ),
        entry!(0x00b18b70, c_frond_engine_end_guide(Ptr<CFrondEngine>, f32)),
        entry!(
            0x00b18f60,
            c_frond_engine_set_profile(Ptr<CFrondEngine>, Ptr)
        ),
        entry!(
            0x00b18fd0,
            c_frond_engine_compute(Ptr<CFrondEngine>, Ptr<CIndexedGeometry>, Ptr)
        ),
        entry!(0x00b19ad0, fn_00b19ad0(Ptr<CFrondEngine>, Ptr<SFrondGuide>)),
        entry!(
            0x00b1ada0,
            fn_00b1ada0(Ptr<CVec3>, f32, f32, f32) -> Ptr<CVec3>
        ),
        entry!(0x00b1add0, fn_00b1add0(Ptr<CVec3>, f32, f32, f32)),
        entry!(0x00b1ae00, fn_00b1ae00(Ptr<CRotTransform>, f32)),
        entry!(
            0x00b1aed0,
            fn_00b1aed0(Ptr<CFrondEngine>, u16, u32, Ptr<StlVector>)
        ),
        entry!(0x00b1b000, fn_00b1b000(Ptr<CFrondEngine>, Ptr<SFrondGuide>)),
        entry!(
            0x00b1b960,
            fn_00b1b960(Ptr<CFrondEngine>, u32, u32, Ptr<SFrondGuide>)
        ),
        entry!(
            0x00b1be50,
            fn_00b1be50(Ptr<CFrondEngine>, Ptr<SFrondGuide>, Ptr<StlVector>, Ptr<StlVector>)
        ),
        entry!(
            0x00b1c6d0,
            fn_00b1c6d0(Ptr, Ptr<CIndexedGeometry>, u8, Ptr, u8)
        ),
        entry!(0x00b1c7a0, fn_00b1c7a0(Ptr, Ptr) -> Ptr),
        entry!(
            0x00b1c850,
            c_frond_engine_build_guide_lods(Ptr<CFrondEngine>)
        ),
        entry!(
            0x00b1d100,
            fn_00b1d100(Ptr<SFrondTexture>) -> Ptr<SFrondTexture>
        ),
        entry!(
            0x00b1d130,
            fn_00b1d130(Ptr<StlVector>, Ptr<StlVector>) -> Ptr<StlVector>
        ),
        entry!(
            0x00b1d5d0,
            fn_00b1d5d0(Ptr<StlVector>, Ptr<StlVector>) -> Ptr<StlVector>
        ),
        entry!(0x00b1d8a0, fn_00b1d8a0(Ptr<StlVector>) -> Ptr<StlVector>),
        entry!(0x00b1d980, fn_00b1d980(Ptr<StlVector>, Ptr)),
        entry!(0x00b1da90, fn_00b1da90(Ptr<StlVector>)),
        entry!(0x00b1db00, fn_00b1db00(Ptr<StlVector>, u32) -> Ptr),
        entry!(0x00b1db40, fn_00b1db40(Ptr<StlVector>, Ptr)),
        entry!(
            0x00b1dc50,
            fn_00b1dc50(Ptr<StlVector>, Ptr, u32, u32, Ptr) -> Ptr
        ),
        entry!(
            0x00b1dd80,
            fn_00b1dd80(Ptr<StlVector>, Ptr, u32, u32) -> Ptr
        ),
        entry!(0x00b1de20, fn_00b1de20(Ptr<StlVector>)),
        entry!(0x00b1df10, fn_00b1df10(Ptr<StlVector>)),
        entry!(0x00b1df80, fn_00b1df80(Ptr<StlVector>, Ptr)),
        entry!(0x00b1e090, fn_00b1e090(Ptr<StlVector>) -> Ptr<StlVector>),
        entry!(0x00b1e170, fn_00b1e170(Ptr<StlVector>)),
        entry!(0x00b1e1e0, fn_00b1e1e0(Ptr<StlVector>, Ptr)),
        entry!(0x00b1e2f0, fn_00b1e2f0(Ptr<StlVector>)),
        entry!(0x00b1e640, fn_00b1e640(Ptr<StlVector>, Ptr)),
        entry!(
            0x00b1e840,
            fn_00b1e840(Ptr<StlVector>, Ptr, u32, u32, Ptr) -> Ptr
        ),
        entry!(
            0x00b1ea20,
            fn_00b1ea20(Ptr<StlVector>, Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00b1ecf0, fn_00b1ecf0(Ptr<StlVector>, u32, u32, u32, Ptr)),
        entry!(
            0x00b1f460,
            fn_00b1f460(Ptr<StlVector>, Ptr, u32, u32, Ptr) -> Ptr
        ),
        entry!(0x00b1f650, fn_00b1f650(Ptr<StlVector>)),
        entry!(
            0x00b1f6d0,
            fn_00b1f6d0(Ptr<StlVector>, Ptr, u32, u32, Ptr) -> Ptr
        ),
        entry!(
            0x00b1f800,
            fn_00b1f800(Ptr<StlVector>, Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00b1f920, fn_00b1f920(Ptr<StlVector>, u32) -> bool),
        entry!(0x00b1f9e0, fn_00b1f9e0(Ptr<StlVector>)),
        entry!(0x00b1ff00, fn_00b1ff00(Ptr, Ptr) -> i32),
        entry!(0x00b23e80, fn_00b23e80(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b42980, fn_00b42980(Ptr, u32, Ptr<StlVector>) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    /// The proxy word every test vector carries at `+0`.
    const PROXY: u32 = 0x5000_0001;

    /// A list a double fills in while the test runs.
    type Record<T> = Rc<RefCell<Vec<T>>>;
    type Log = Record<Vec<u32>>;

    /// The callees of this file that live outside it; every test runs them as do-nothing
    /// doubles unless it supplies its own.
    const EXTERNAL: &[u32] = &[
        STRING_FROM_CHARS,
        STRING_COPY,
        STRING_TIDY,
        STRING_ASSIGN,
        PARSE_INT,
        PARSE_FLOAT,
        PARSE_STRING,
        PARSE_BOOL,
        PARSE_BRANCH_PARAMETER,
        FILENAME_NO_PATH,
        EXCEPTION_CONSTRUCT,
        BASE_EXCEPTION_CONSTRUCT,
        THROW,
        VSPRINTF,
        SPLINE_CONSTRUCT,
        SPLINE_DESTRUCT,
        SPLINE_EVALUATE,
        ALLOCATE_PROXY,
        LENGTH_ERROR,
        ALLOCATE_TEXTURES,
        RANDOM_CONSTRUCT,
        GET_UNIFORM,
        VEC3_ADD,
        VEC3_SCALE,
        VEC3_SUBTRACT,
        VEC3_NORMALIZE,
        VEC3_CROSS,
        VEC3_TIMES_ROTATION,
        VEC3_LENGTH,
        VEC3_DISTANCE,
        CVEC3_VECTOR_CONSTRUCT,
        CVEC3_VECTOR_PUSH,
        CVEC3_VECTOR_TIDY,
        FLOAT_VECTOR_CONSTRUCT,
        FLOAT_VECTOR_PUSH,
        FLOAT_VECTOR_AT,
        FLOAT_VECTOR_TIDY,
        FLOAT_VECTOR_RELEASE,
        PROXY_RELEASE,
        VECTOR_VECTOR_CONSTRUCT,
        FLOAT_VECTOR_VECTOR_TIDY,
        GUIDE_VECTOR_TIDY,
        VERTEX_VECTOR_TIDY,
        GUIDE_COPY_CONSTRUCT,
        GUIDE_ASSIGN,
        GEOMETRY_SET_NUM_LOD_LEVELS,
        GEOMETRY_ADD_STRIP,
        GEOMETRY_RESET_STRIP_COUNTER,
        GEOMETRY_ADD_COORDINATE,
        GEOMETRY_ADD_TEX_COORD,
        GEOMETRY_ADD_COLOR,
        GEOMETRY_ADD_NORMAL,
        GEOMETRY_ADD_TANGENT,
        GEOMETRY_ADD_BINORMAL,
        GEOMETRY_ADD_WIND_WEIGHT,
        GEOMETRY_COORD_POINTER,
        GEOMETRY_TEX_COORD_POINTER,
        GEOMETRY_SET_TEX_COORDS,
        ATAN2,
        CROT_TRANSFORM_CONSTRUCT,
        CROT_TRANSFORM_MULTIPLY,
        TRIVIAL_DESTRUCTOR,
        ITERATOR_ADVANCE,
        ITERATORS_DIFFER,
        GUIDE_SORT,
        VECTOR_CLEAR,
        VECTOR_ASSIGN_RANGE,
        VECTOR_CONSTRUCT_RANGE,
        VECTOR_RESERVE,
        FLOAT_VECTOR_CLEAR,
        FLOAT_COPY_RANGE,
        FLOAT_CONSTRUCT_RANGE,
        FLOAT_RESERVE,
        GUIDE_MOVE_DOWN,
        GUIDE_MOVE_UP,
        GUIDE_DESTROY_RANGE,
        GUIDE_LOD_DESTROY_RANGE,
        GUIDE_ALLOCATE,
        GUIDE_RELOCATE_COPY,
        GUIDE_TAIL_COPY,
        GUIDE_FILL,
        TEXTURE_MOVE_DOWN,
        OPERATOR_DELETE,
        0x00b1_eb00,
        0x00b2_3f60,
        0x00b2_4290,
        0x00b2_43a0,
        0x00b2_4580,
        0x00b2_0360,
        0x00b2_0ce0,
        0x00b1_fd50,
        0x00b2_1560,
        0x00b2_3520,
        0x00b1_6890,
    ];

    /// An engine with the exe's data range (statics and constants) mapped as zeroes and every
    /// outside callee of the file silenced.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        for addr in EXTERNAL {
            e.register(*addr, |_, _| Ret::default());
        }
        e
    }

    /// A double that records its argument words and answers `result` in `EAX`.
    fn spy(
        e: &mut Engine,
        addr: u32,
        result: impl Fn(&mut Engine, &[u32]) -> u32 + 'static,
    ) -> Log {
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                eax: result(e, a),
                ..Ret::default()
            }
        });
        log
    }

    /// A double that answers the values of `script` one by one in `EAX` (zero when it runs
    /// out) and records its argument words.
    fn feed(e: &mut Engine, addr: u32, script: &[u32]) -> Log {
        let queue: Rc<RefCell<VecDeque<u32>>> =
            Rc::new(RefCell::new(script.iter().copied().collect()));
        spy(e, addr, move |_, _| {
            queue.borrow_mut().pop_front().unwrap_or(0)
        })
    }

    /// A double that answers the values of `script` one by one in `ST0` and records its
    /// argument words.
    fn feed_floats(e: &mut Engine, addr: u32, script: &[f64]) -> Log {
        let queue: Rc<RefCell<VecDeque<f64>>> =
            Rc::new(RefCell::new(script.iter().copied().collect()));
        let log: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret {
                st0: queue.borrow_mut().pop_front().unwrap_or(0.0),
                ..Ret::default()
            }
        });
        log
    }

    /// A double that answers `result` in `ST0`.
    fn spy_float(e: &mut Engine, addr: u32, result: impl Fn(&mut Engine, &[u32]) -> f64 + 'static) {
        e.register_double(addr, move |e, a| Ret {
            st0: result(e, a),
            ..Ret::default()
        });
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

    /// The `count` floats at `at`.
    fn floats(e: &Engine, at: u32, count: u32) -> Vec<f32> {
        (0..count).map(|i| e.mem.f32(at + 4 * i)).collect()
    }

    /// Writes `values` as floats from `at`.
    fn put(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *v);
        }
    }

    /// A frond engine whose three vectors are empty.
    fn frond(e: &mut Engine) -> Ptr<CFrondEngine> {
        let this = e.new_object::<CFrondEngine>();
        for offset in [0x08u32, 0x20, 0x50] {
            e.mem.set_u32(this.addr() + offset, PROXY);
        }
        this
    }

    /// A guide whose vertex vector holds `count` vertices (room for `capacity`); returns the
    /// address of the vertices.
    fn guide_with(e: &mut Engine, guide: u32, count: u32, capacity: u32) -> u32 {
        vector(e, guide, 0x38, count, capacity)
    }

    /// The word at `at`.
    fn word(e: &Engine, at: u32) -> u32 {
        e.mem.u32(at)
    }

    #[test]
    fn constructor_sets_the_defaults_and_builds_the_profile() {
        let mut e = engine();
        e.set_global(LARGE_RETENTION_DEFAULT, 0.05f32);
        let proxies = spy(&mut e, ALLOCATE_PROXY, |e, _| e.mem.alloc(8));
        let lods = spy(&mut e, VECTOR_VECTOR_CONSTRUCT, |_, a| a[0]);
        let text = spy(&mut e, STRING_FROM_CHARS, |_, a| a[0]);
        let spline = spy(&mut e, SPLINE_CONSTRUCT, |_, a| a[0]);
        let tidy = spy(&mut e, STRING_TIDY, |_, _| 0);
        let this = e.new_object::<CFrondEngine>();
        let b = this.addr();
        for offset in (0..0x84).step_by(4) {
            e.mem.set_u32(b + offset, 0xCDCD_CDCD);
        }
        let result = e.call(0x00b1_7fd0, &args![this]).u32();
        assert_eq!(result, b);
        assert_eq!(word(&e, b), 0);
        assert_eq!(word(&e, b + 4), 0);
        assert_eq!(word(&e, b + 0x38), 1);
        assert_eq!(word(&e, b + 0x3C), 2);
        assert_eq!(word(&e, b + 0x44), 4);
        assert_eq!(word(&e, b + 0x48), 1);
        assert_eq!(e.mem.u8(b + 0x4C), 0);
        assert_eq!(word(&e, b + 0x68), 4);
        assert_eq!(floats(&e, b + 0x6C, 4), vec![1.0, 0.0, 0.0, 0.05]);
        assert_eq!(word(&e, b + 0x7C), 2);
        assert_eq!(word(&e, b + 0x80), 1);
        // The two std::vectors made here got proxies pointing back at them.
        assert_eq!(proxies.borrow().len(), 2);
        let guide_proxy = word(&e, b + 8);
        assert_eq!(word(&e, guide_proxy), b + 8);
        let texture_proxy = word(&e, b + 0x50);
        assert_eq!(word(&e, texture_proxy), b + 0x50);
        assert_eq!(*lods.borrow(), vec![vec![b + 0x20]]);
        // The profile: a new 0x84-byte spline built from the default text.
        let profile = word(&e, b + 0x40);
        assert_ne!(profile, 0);
        assert_eq!(spline.borrow().len(), 1);
        assert_eq!(spline.borrow()[0][0], profile);
        assert_eq!(text.borrow().len(), 1);
        assert_eq!(text.borrow()[0][1], DEFAULT_PROFILE_TEXT);
        // The temporary string is destroyed.
        let destroyed = tidy.borrow().last().cloned().unwrap();
        assert_eq!(destroyed[1..], [1, 0]);
    }

    #[test]
    fn destructor_destroys_the_profile_and_releases_the_vectors() {
        let mut e = engine();
        let this = frond(&mut e);
        let b = this.addr();
        e.mem.set_u32(b + 0x40, 0x7000);
        e.mem.set_u32(b + 0x08, 0x5101);
        e.mem.set_u32(b + 0x20, 0x5102);
        e.mem.set_u32(b + 0x50, 0x5103);
        let destroyed = spy(&mut e, SPLINE_DESTRUCT, |_, _| 0);
        let deleted = spy(&mut e, OPERATOR_DELETE, |_, _| 0);
        let tidied = spy(&mut e, GUIDE_VECTOR_TIDY, |_, _| 0);
        e.call(0x00b1_81b0, &args![this]);
        assert_eq!(*destroyed.borrow(), vec![vec![0x7000]]);
        assert_eq!(word(&e, b + 0x40), 0);
        let order: Vec<u32> = deleted.borrow().iter().map(|a| a[0]).collect();
        assert_eq!(order, vec![0x7000, 0x5103, 0x5102, 0x5101]);
        assert_eq!(*tidied.borrow(), vec![vec![b + 8]]);
    }

    #[test]
    fn destructor_without_a_profile_only_releases_the_vectors() {
        let mut e = engine();
        let this = frond(&mut e);
        let deleted = spy(&mut e, OPERATOR_DELETE, |_, _| 0);
        let destroyed = spy(&mut e, SPLINE_DESTRUCT, |_, _| 0);
        e.call(0x00b1_81b0, &args![this]);
        assert!(destroyed.borrow().is_empty());
        assert_eq!(deleted.borrow().len(), 3);
        assert_eq!(word(&e, this.addr() + 0x40), 0);
    }

    #[test]
    fn level_getter_returns_the_level() {
        let mut e = engine();
        let this = frond(&mut e);
        e.mem.set_u32(this.addr() + 0x48, 7);
        assert_eq!(e.call(0x00b1_8300, &args![this]).u32(), 7);
    }

    #[test]
    fn enabled_getter_returns_the_flag_byte() {
        let mut e = engine();
        let this = frond(&mut e);
        e.mem.set_u8(this.addr() + 0x4C, 1);
        assert_eq!(e.call(0x00b1_8320, &args![this]).u8(), 1);
        e.mem.set_u8(this.addr() + 0x4C, 0);
        assert_eq!(e.call(0x00b1_8320, &args![this]).u8(), 0);
    }

    #[test]
    fn parse_reads_every_setting() {
        let mut e = engine();
        let this = frond(&mut e);
        let b = this.addr();
        feed(
            &mut e,
            PARSE_INT,
            &[
                0x32ca, 5, 0x32cb, 1, 0x32cc, 3, 0x32cd, 0x32ce, 7, 0x32cf, 0x32d1, 6, 0x32d2,
                0x32d3, 0x32d4, 0x32d5, 0x36b7, 9, 0x36b8, 2, 0x32c9,
            ],
        );
        feed_floats(&mut e, PARSE_FLOAT, &[0.25, 0.5, 0.75, 0.125]);
        feed(&mut e, PARSE_BOOL, &[1]);
        let parameters = feed(&mut e, PARSE_BRANCH_PARAMETER, &[0x8000]);
        let destroyed = spy(&mut e, SPLINE_DESTRUCT, |_, _| 0);
        e.mem.set_u32(b + 0x40, 0x7000);
        e.call(0x00b1_8340, &args![this, 0x1234u32]);
        assert_eq!(word(&e, b + 0x48), 5);
        assert_eq!(word(&e, b + 0x38), 1);
        assert_eq!(word(&e, b + 0x3C), 3);
        assert_eq!(*parameters.borrow(), vec![vec![0x1234]]);
        // The new profile replaces (and destroys) the old one.
        assert_eq!(word(&e, b + 0x40), 0x8000);
        assert_eq!(*destroyed.borrow(), vec![vec![0x7000]]);
        assert_eq!(word(&e, b + 0x44), 7);
        assert_eq!(e.mem.u8(b + 0x4C), 1);
        assert_eq!(word(&e, b + 0x68), 6);
        assert_eq!(floats(&e, b + 0x6C, 4), vec![0.25, 0.5, 0.75, 0.125]);
        assert_eq!(word(&e, b + 0x7C), 9);
        assert_eq!(word(&e, b + 0x80), 2);
    }

    #[test]
    fn parse_reads_a_texture_list() {
        let mut e = engine();
        e.set_global(STRING_NPOS, 0xFFFF_FFFFu32);
        e.set_global(HALF_FLOAT, 0.5f32);
        let this = frond(&mut e);
        feed(
            &mut e,
            PARSE_INT,
            &[
                0x32d0, 1, 0xDEAD, 0x36b2, 0x36b3, 0x36b4, 0x36b5, 0x36b6, 0x36b1, 0x32c9,
            ],
        );
        feed_floats(&mut e, PARSE_FLOAT, &[1.5, 2.5, 10.0, 20.0]);
        let parsed = spy(&mut e, PARSE_STRING, |_, _| 0);
        let assigned = spy(&mut e, STRING_ASSIGN, |_, _| 0);
        let tidied = spy(&mut e, STRING_TIDY, |_, _| 0);
        let stripped = spy(&mut e, FILENAME_NO_PATH, |_, a| a[1]);
        let cleared = spy(&mut e, 0x00b1_f800, |_, _| 0);
        let pushed: Record<Vec<f32>> = Rc::new(RefCell::new(Vec::new()));
        let sink = pushed.clone();
        e.register_double(0x00b1_f6d0, move |e, a| {
            sink.borrow_mut().push(floats(e, a[4] + 0x1C, 4));
            Ret::default()
        });
        e.call(0x00b1_8340, &args![this, 0x1234u32]);
        // The list is cleared first, the file's string read and trimmed, and the texture pushed.
        assert_eq!(cleared.borrow().len(), 1);
        assert_eq!(parsed.borrow().len(), 1);
        assert_eq!(parsed.borrow()[0][0], 0x1234);
        let assign = assigned.borrow().clone();
        assert_eq!(assign.len(), 3);
        for call in &assign {
            assert_eq!(call[2..], [0, 0xFFFF_FFFF]);
        }
        let texture = assign[1][0];
        assert_eq!(assign[2][0], texture);
        assert_eq!(assign[2][1], stripped.borrow()[0][1]);
        assert_eq!(stripped.borrow()[0][0], texture);
        assert_eq!(*pushed.borrow(), vec![vec![1.5, 2.5, 10.0, 20.0]]);
        assert_eq!(tidied.borrow().len(), 6);
    }

    #[test]
    fn parse_keeps_the_default_texture_values_without_tokens() {
        let mut e = engine();
        e.set_global(HALF_FLOAT, 0.5f32);
        let this = frond(&mut e);
        feed(&mut e, PARSE_INT, &[0x32d0, 1, 0, 0x36b2, 0x36b1, 0x32c9]);
        let pushed: Record<Vec<f32>> = Rc::new(RefCell::new(Vec::new()));
        let sink = pushed.clone();
        e.register_double(0x00b1_f6d0, move |e, a| {
            sink.borrow_mut().push(floats(e, a[4] + 0x1C, 4));
            Ret::default()
        });
        e.call(0x00b1_8340, &args![this, 0x1234u32]);
        assert_eq!(*pushed.borrow(), vec![vec![0.5, 1.0, 0.0, 0.0]]);
    }

    #[test]
    fn parse_throws_on_an_unknown_token() {
        let mut e = engine();
        let this = frond(&mut e);
        feed(&mut e, PARSE_INT, &[0x1234]);
        let formatted: Record<(u32, u32)> = Rc::new(RefCell::new(Vec::new()));
        let sink = formatted.clone();
        e.register_double(VSPRINTF, move |e, a| {
            sink.borrow_mut().push((a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        spy(&mut e, STRING_FROM_CHARS, |_, a| a[0]);
        let constructed = spy(&mut e, EXCEPTION_CONSTRUCT, |_, _| 0);
        let thrown = spy(&mut e, THROW, |_, _| 0);
        e.call(0x00b1_8340, &args![this, 0x1234u32]);
        assert_eq!(*formatted.borrow(), vec![(MALFORMED_FROND_MESSAGE, 0x1234)]);
        assert_eq!(constructed.borrow().len(), 1);
        assert_eq!(constructed.borrow()[0][2], 0);
        let object = constructed.borrow()[0][0];
        assert_eq!(*thrown.borrow(), vec![vec![object, THROW_INFO]]);
    }

    #[test]
    fn parse_throws_on_an_unknown_texture_token() {
        let mut e = engine();
        e.set_global(HALF_FLOAT, 0.5f32);
        let this = frond(&mut e);
        feed(&mut e, PARSE_INT, &[0x32d0, 1, 0, 0x9999]);
        let formatted: Record<(u32, u32)> = Rc::new(RefCell::new(Vec::new()));
        let sink = formatted.clone();
        e.register_double(VSPRINTF, move |e, a| {
            sink.borrow_mut().push((a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        spy(&mut e, STRING_FROM_CHARS, |_, a| a[0]);
        let thrown = spy(&mut e, THROW, |_, _| 0);
        let pushed = spy(&mut e, 0x00b1_f6d0, |_, _| 0);
        e.call(0x00b1_8340, &args![this, 0x1234u32]);
        assert_eq!(
            *formatted.borrow(),
            vec![(MALFORMED_TEXTURE_MESSAGE, 0x9999)]
        );
        assert_eq!(thrown.borrow().len(), 1);
        // The parse stops there: the texture is not pushed.
        assert!(pushed.borrow().is_empty());
    }

    #[test]
    fn format_string_formats_into_the_string() {
        let mut e = engine();
        let seen: Record<(u32, u32, Vec<u32>)> = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(VSPRINTF, move |e, a| {
            let list = vec![e.mem.u32(a[2]), e.mem.u32(a[2] + 4)];
            sink.borrow_mut().push((a[0], a[1], list));
            Ret::default()
        });
        let built = spy(&mut e, STRING_FROM_CHARS, |_, a| a[0]);
        let out = e.mem.alloc(0x20);
        let result = e
            .call(0x00b1_8860, &args![out, 0x0102_0000u32, 7u32, 9u32])
            .u32();
        assert_eq!(result, out);
        let call = seen.borrow()[0].clone();
        assert_eq!(call.1, 0x0102_0000);
        assert_eq!(call.2, vec![7, 9]);
        // The string is made from the formatted buffer.
        assert_eq!(*built.borrow(), vec![vec![out, call.0]]);
    }

    #[test]
    fn begin_guide_pushes_a_fresh_guide() {
        let mut e = engine();
        let this = frond(&mut e);
        let b = this.addr();
        let proxies = spy(&mut e, ALLOCATE_PROXY, |e, _| e.mem.alloc(8));
        let captured: Record<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
        let sink = captured.clone();
        e.register_double(0x00b1_dc50, move |e, a| {
            let words = (0..14).map(|i| e.mem.u32(a[4] + 4 * i)).collect();
            sink.borrow_mut().push(words);
            Ret::default()
        });
        let tidied = spy(&mut e, VERTEX_VECTOR_TIDY, |_, _| 0);
        let deleted = spy(&mut e, OPERATOR_DELETE, |_, _| 0);
        e.call(0x00b1_88d0, &args![this]);
        // The guide is pushed at the end of the guide vector: the iterator at the end.
        let pushed = captured.borrow()[0].clone();
        let proxy = pushed[0];
        assert_ne!(proxy, 0);
        assert_eq!(pushed[1..], [0; 13]);
        assert_eq!(proxies.borrow().len(), 1);
        // The temporary guide is then destroyed.
        assert_eq!(tidied.borrow().len(), 1);
        assert_eq!(*deleted.borrow(), vec![vec![proxy]]);
        assert_eq!(word(&e, b + 0x10), 0);
    }

    #[test]
    fn guide_constructor_zeroes_its_fields() {
        let mut e = engine();
        spy(&mut e, ALLOCATE_PROXY, |e, _| e.mem.alloc(8));
        let guide = e.new_object::<SFrondGuide>();
        let g = guide.addr();
        for offset in (0..0x38).step_by(4) {
            e.mem.set_u32(g + offset, 0xCDCD_CDCD);
        }
        let result = e.call(0x00b1_8970, &args![guide]).u32();
        assert_eq!(result, g);
        assert_ne!(word(&e, g), 0);
        assert_eq!(floats(&e, g + 0x18, 2), vec![0.0, 0.0]);
        assert_eq!(e.mem.u8(g + 0x20), 0);
        assert_eq!(floats(&e, g + 0x24, 3), vec![0.0, 0.0, 0.0]);
        assert_eq!(word(&e, g + 0x30), 0);
        assert_eq!(word(&e, g + 0x34), 0);
    }

    #[test]
    fn guide_destructor_tidies_the_vertices() {
        let mut e = engine();
        let guide = e.new_object::<SFrondGuide>();
        e.mem.set_u32(guide.addr(), 0x5105);
        let tidied = spy(&mut e, VERTEX_VECTOR_TIDY, |_, _| 0);
        let deleted = spy(&mut e, OPERATOR_DELETE, |_, _| 0);
        e.call(0x00b1_89f0, &args![guide]);
        assert_eq!(*tidied.borrow(), vec![vec![guide.addr()]]);
        assert_eq!(*deleted.borrow(), vec![vec![0x5105]]);
    }

    #[test]
    fn add_point_pushes_a_vertex_onto_the_last_guide() {
        let mut e = engine();
        let this = frond(&mut e);
        let b = this.addr();
        let guides = vector(&mut e, b + 8, 0x38, 2, 2);
        let last = guides + 0x38;
        let vertices = guide_with(&mut e, last, 0, 4);
        let captured: Record<Vec<u32>> = Rc::new(RefCell::new(Vec::new()));
        let sink = captured.clone();
        e.register_double(0x00b2_3f60, move |e, a| {
            let mut words: Vec<u32> = a.to_vec();
            words.extend((0..14).map(|i| e.mem.u32(a[2] + 4 * i)));
            sink.borrow_mut().push(words);
            Ret::default()
        });
        let weight = f32::from_bits(0x7F80_0001);
        e.call(
            0x00b1_8a60,
            &args![
                this, 1u32, 2u32, 3u32, 11u32, 12u32, 13u32, 14u32, 15u32, 16u32, 17u32, 18u32,
                19u32, weight, 5u32
            ],
        );
        let call = captured.borrow()[0].clone();
        // Constructed at the end of the last guide's vertex vector, one element, the allocator
        // at `+8`.
        assert_eq!(call[..2], [vertices, 1]);
        assert_eq!(call[3], last + 8);
        let vertex = &call[6..];
        assert_eq!(vertex[..3], [1, 2, 3]);
        assert_eq!(vertex[3..12], [11, 12, 13, 14, 15, 16, 17, 18, 19]);
        // The weight is copied through the x87 stack: the signalling NaN comes back quiet.
        assert_eq!(vertex[12], 0x7FC0_0001);
        assert_eq!(vertex[13], 5);
        assert_eq!(word(&e, last + 0x10), vertices + 0x38);
    }

    #[test]
    #[should_panic(expected = "invalid parameter reached")]
    fn add_point_without_a_guide_takes_the_invalid_path() {
        let mut e = engine();
        let this = frond(&mut e);
        // The exe goes on with the element before the first one (a wild address) after the
        // invalid-parameter call; here the call stops the run, which shows it comes first.
        e.register_double(INVALID_PARAMETER, |_, _| {
            panic!("invalid parameter reached")
        });
        e.call(
            0x00b1_8a60,
            &args![
                this, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32,
                0.0f32, 0u32
            ],
        );
    }

    #[test]
    fn vertex_constructor_clears_the_vertex() {
        let mut e = engine();
        let vertex = e.new_object::<SFrondVertex>();
        let v = vertex.addr();
        for offset in (0..0x38).step_by(4) {
            e.mem.set_u32(v + offset, 0xCDCD_CDCD);
        }
        let identity = spy(&mut e, CROT_TRANSFORM_CONSTRUCT, |_, a| a[0]);
        let result = e.call(0x00b1_8b20, &args![vertex]).u32();
        assert_eq!(result, v);
        assert_eq!(floats(&e, v, 3), vec![0.0, 0.0, 0.0]);
        assert_eq!(*identity.borrow(), vec![vec![v + 0x0C]]);
        assert_eq!(word(&e, v + 0x30), 0);
        assert_eq!(word(&e, v + 0x34), 0);
        // The transform itself is the identity constructor's business.
        assert_eq!(word(&e, v + 0x0C), 0xCDCD_CDCD);
    }

    /// A guide vertex vector of the last of `guides` guides at `this + 8`, holding vertices at
    /// `positions`; returns the address of the last guide (the vertices are 0x38 bytes each,
    /// the position at `+0`).
    fn guides_ending_with(
        e: &mut Engine,
        this: Ptr<CFrondEngine>,
        guides: u32,
        positions: &[[f32; 3]],
    ) -> u32 {
        let first = vector(e, this.addr() + 8, 0x38, guides, guides);
        let last = first + 0x38 * (guides - 1);
        let vertices = guide_with(e, last, positions.len() as u32, positions.len() as u32);
        for (i, position) in positions.iter().enumerate() {
            put(e, vertices + 0x38 * i as u32, position);
        }
        last
    }

    /// `Distance` as a double: the Euclidean distance of the two `CVec3`s (`ST0`).
    fn distance_double(e: &mut Engine) {
        e.register_double(VEC3_DISTANCE, |e, a| {
            let p = floats(e, a[0], 3);
            let q = floats(e, a[1], 3);
            let sum: f64 = (0..3)
                .map(|k| (f64::from(p[k]) - f64::from(q[k])).powi(2))
                .sum();
            Ret {
                st0: f64::from(sum.sqrt() as f32),
                ..Ret::default()
            }
        });
    }

    #[test]
    fn end_guide_without_textures_clears_the_map_index_of_the_loop_vertex_slot() {
        let mut e = engine();
        e.set_global(SCALE_DOUBLE, 0.5f64);
        let this = frond(&mut e);
        let b = this.addr();
        let last = guides_ending_with(&mut e, this, 3, &[[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]]);
        let middle = last - 0x38;
        e.mem.set_u8(middle + 0x20, 0x77);
        e.mem.set_u8(last + 0x20, 0x55);
        e.mem.set_f32(last + 0x24, 99.0);
        distance_double(&mut e);
        let destroyed = spy(&mut e, TRIVIAL_DESTRUCTOR, |_, _| 0);
        let constructed = spy(&mut e, RANDOM_CONSTRUCT, |_, a| a[0]);
        e.call(0x00b1_8b70, &args![this, 2.0f32]);
        assert_eq!(e.mem.f32(last + 0x18), 5.0);
        assert_eq!(e.mem.f32(last + 0x1C), 2.5);
        assert_eq!(e.mem.f32(last + 0x24), 0.0);
        assert_eq!(e.mem.f32(last + 0x28), 10.0);
        // The exe clears the map index of the guide at the loop's final index (vertex count
        // minus one), here the middle one, not the guide being ended.
        assert_eq!(e.mem.u8(middle + 0x20), 0);
        assert_eq!(e.mem.u8(last + 0x20), 0x55);
        assert_eq!(constructed.borrow().len(), 1);
        assert_eq!(destroyed.borrow().len(), 1);
        assert_eq!(destroyed.borrow()[0][0], constructed.borrow()[0][0]);
        assert_eq!(word(&e, b + 0x50 + 0x0C), 0);
    }

    #[test]
    fn end_guide_with_textures_picks_one_at_random() {
        let mut e = engine();
        e.set_global(SCALE_DOUBLE, 0.5f64);
        e.set_global(MIRROR_FACTOR, -1.0f64);
        e.set_global(RANDOM_UPPER_BOUND, 100000.0f32);
        let this = frond(&mut e);
        let b = this.addr();
        let last = guides_ending_with(&mut e, this, 3, &[[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]]);
        let textures = vector(&mut e, b + 0x50, 0x2C, 3, 3);
        // Texture 1: aspect 2, size scale 3, angle offsets 10 and 30.
        put(&mut e, textures + 0x2C + 0x1C, &[2.0, 3.0, 10.0, 30.0]);
        distance_double(&mut e);
        let draws = feed_floats(&mut e, GET_UNIFORM, &[7.9, 20.0]);
        e.mem.set_f32(last + 0x24, 99.0);
        e.call(0x00b1_8b70, &args![this, 2.0f32]);
        // Draw 7.9 truncated, modulo three textures: index 1.
        assert_eq!(e.mem.u8(last + 0x20), 1);
        let draws = draws.borrow().clone();
        assert_eq!(draws.len(), 2);
        assert_eq!(draws[0][1..], [0, 100000.0f32.to_bits()]);
        assert_eq!(draws[1][1..], [10.0f32.to_bits(), 30.0f32.to_bits()]);
        // Length 5, half of it times the aspect ratio and the size scale.
        assert_eq!(e.mem.f32(last + 0x1C), 15.0);
        // The angle: the drawn 20, mirrored because the guide count (3) is odd.
        assert_eq!(e.mem.f32(last + 0x24), -20.0);
        assert_eq!(e.mem.f32(last + 0x28), 10.0);
    }

    #[test]
    fn end_guide_keeps_the_angle_for_an_even_guide_count() {
        let mut e = engine();
        e.set_global(SCALE_DOUBLE, 0.5f64);
        e.set_global(MIRROR_FACTOR, -1.0f64);
        let this = frond(&mut e);
        let b = this.addr();
        let last = guides_ending_with(&mut e, this, 2, &[[0.0, 0.0, 0.0], [3.0, 4.0, 0.0]]);
        let textures = vector(&mut e, b + 0x50, 0x2C, 1, 1);
        put(&mut e, textures + 0x1C, &[1.0, 1.0, 0.0, 0.0]);
        distance_double(&mut e);
        feed_floats(&mut e, GET_UNIFORM, &[0.0, 12.5]);
        e.call(0x00b1_8b70, &args![this, 1.0f32]);
        assert_eq!(e.mem.f32(last + 0x24), 12.5);
    }

    #[test]
    fn set_profile_replaces_the_profile() {
        let mut e = engine();
        let this = frond(&mut e);
        let b = this.addr();
        e.mem.set_u32(b + 0x40, 0x7000);
        let destroyed = spy(&mut e, SPLINE_DESTRUCT, |_, _| 0);
        let deleted = spy(&mut e, OPERATOR_DELETE, |_, _| 0);
        // The same spline: nothing happens.
        e.call(0x00b1_8f60, &args![this, 0x7000u32]);
        assert!(destroyed.borrow().is_empty());
        e.call(0x00b1_8f60, &args![this, 0x8000u32]);
        assert_eq!(*destroyed.borrow(), vec![vec![0x7000]]);
        assert_eq!(*deleted.borrow(), vec![vec![0x7000]]);
        assert_eq!(word(&e, b + 0x40), 0x8000);
        // No profile to destroy.
        e.mem.set_u32(b + 0x40, 0);
        e.call(0x00b1_8f60, &args![this, 0x9000u32]);
        assert_eq!(destroyed.borrow().len(), 1);
        assert_eq!(word(&e, b + 0x40), 0x9000);
    }

    /// A frond engine whose guide-lod vector holds the given lods, each a list of guides given
    /// by (vertex count, start index, vertices per guide vertex).
    fn engine_with_lods(e: &mut Engine, lods: &[Vec<(u32, u32, u32)>]) -> Ptr<CFrondEngine> {
        let this = frond(e);
        let count = lods.len() as u32;
        let rows = vector(e, this.addr() + 0x20, 0x18, count, count);
        for (l, guides) in lods.iter().enumerate() {
            let header = rows + 0x18 * l as u32;
            let n = guides.len() as u32;
            let data = vector(e, header, 0x38, n, n);
            for (m, (vertices, start, per_vertex)) in guides.iter().enumerate() {
                let guide = data + 0x38 * m as u32;
                vector(e, guide, 0x38, *vertices, *vertices);
                e.mem.set_u32(guide + 0x30, *start);
                e.mem.set_u32(guide + 0x34, *per_vertex);
            }
        }
        this
    }

    /// The doubles `Compute` needs for an empty guide list: a proxy for the vectors it builds.
    fn compute_doubles(e: &mut Engine) {
        spy(e, ALLOCATE_PROXY, |e, _| e.mem.alloc(8));
    }

    /// A strip registration recorded by the geometry double: (level, indices).
    type Strips = Record<(u32, Vec<u16>)>;

    fn strip_double(e: &mut Engine) -> Strips {
        let strips: Strips = Rc::new(RefCell::new(Vec::new()));
        let sink = strips.clone();
        e.register_double(GEOMETRY_ADD_STRIP, move |e, a| {
            let indices = (0..a[3]).map(|i| e.mem.u16(a[2] + 2 * i)).collect();
            sink.borrow_mut().push((a[1], indices));
            Ret::default()
        });
        strips
    }

    #[test]
    fn compute_writes_the_blade_strips_of_every_lod() {
        let mut e = engine();
        compute_doubles(&mut e);
        let this = engine_with_lods(
            &mut e,
            &[vec![(3, 0x10, 0), (2, 0x40, 0)], vec![(2, 0x99, 0)]],
        );
        let b = this.addr();
        e.mem.set_u32(b + 0x38, 0);
        e.mem.set_u32(b + 0x3C, 2);
        let geometry = e.new_object::<CIndexedGeometry>();
        let strips = strip_double(&mut e);
        let levels = spy(&mut e, GEOMETRY_SET_NUM_LOD_LEVELS, |_, _| 0);
        let resets = spy(&mut e, GEOMETRY_RESET_STRIP_COUNTER, |_, _| 0);
        let thrown = spy(&mut e, THROW, |_, _| 0);
        e.call(0x00b1_8fd0, &args![this, geometry, 0u32]);
        assert_eq!(word(&e, b), geometry.addr());
        assert_eq!(word(&e, b + 4), 0);
        assert_eq!(*levels.borrow(), vec![vec![geometry.addr(), 0]]);
        assert_eq!(
            *resets.borrow(),
            vec![vec![geometry.addr(), 0], vec![geometry.addr(), 1]]
        );
        assert!(thrown.borrow().is_empty());
        let range = |from: u16, count: u16| (from..from + count).collect::<Vec<u16>>();
        // Two blades per guide, `2 * vertices` indices each, `(blade * 2) * vertices + start + i`.
        let expected = vec![
            (0, range(0x10, 6)),
            (0, range(0x16, 6)),
            (0, range(0x40, 4)),
            (0, range(0x44, 4)),
            (1, range(0x10, 4)),
            (1, range(0x14, 4)),
        ];
        assert_eq!(*strips.borrow(), expected);
        assert_eq!(e.mem.u16(geometry.addr() + 0x26), 6);
    }

    #[test]
    fn compute_writes_the_cross_section_strips_of_every_lod() {
        let mut e = engine();
        compute_doubles(&mut e);
        e.set_global(ONE_DOUBLE, 1.0f64);
        let this = engine_with_lods(
            &mut e,
            &[vec![(3, 0x10, 5), (2, 0x40, 5)], vec![(2, 0x99, 0)]],
        );
        let b = this.addr();
        e.mem.set_u32(b + 0x38, 1);
        e.mem.set_u32(b + 0x44, 3);
        let geometry = e.new_object::<CIndexedGeometry>();
        let strips = strip_double(&mut e);
        spy(&mut e, GEOMETRY_RESET_STRIP_COUNTER, |_, _| 0);
        e.call(0x00b1_8fd0, &args![this, geometry, 0u32]);
        // The second lod's guide takes the first lod's vertices per guide vertex.
        let lods = word(&e, b + 0x20 + 0x0C);
        let second_lod = word(&e, lods + 0x18 + 0x0C);
        assert_eq!(word(&e, second_lod + 0x34), 5);
        let lengths: Vec<(u32, usize)> = strips
            .borrow()
            .iter()
            .map(|(level, indices)| (*level, indices.len()))
            .collect();
        assert_eq!(lengths, vec![(0, 21), (0, 10), (1, 37)]);
    }

    #[test]
    fn compute_throws_for_an_unknown_frond_type() {
        let mut e = engine();
        compute_doubles(&mut e);
        let this = engine_with_lods(&mut e, &[vec![(3, 0, 0)]]);
        e.mem.set_u32(this.addr() + 0x38, 7);
        let geometry = e.new_object::<CIndexedGeometry>();
        let text = spy(&mut e, STRING_FROM_CHARS, |_, a| a[0]);
        let constructed = spy(&mut e, EXCEPTION_CONSTRUCT, |_, _| 0);
        let thrown = spy(&mut e, THROW, |_, _| 0);
        let levels = spy(&mut e, GEOMETRY_SET_NUM_LOD_LEVELS, |_, _| 0);
        e.call(0x00b1_8fd0, &args![this, geometry, 0u32]);
        assert_eq!(text.borrow().len(), 1);
        assert_eq!(text.borrow()[0][1], DEFAULT_REACHED_MESSAGE);
        let object = constructed.borrow()[0][0];
        assert_eq!(constructed.borrow()[0][1..], [text.borrow()[0][0], 0]);
        assert_eq!(*thrown.borrow(), vec![vec![object, THROW_INFO]]);
        // The throw does not return: nothing is written to the geometry.
        assert!(levels.borrow().is_empty());
    }

    #[test]
    fn compute_throws_when_the_vertices_exceed_the_limit() {
        let mut e = engine();
        compute_doubles(&mut e);
        let this = engine_with_lods(&mut e, &[vec![(1, 0, 0)]]);
        let b = this.addr();
        e.mem.set_u32(b + 0x38, 0);
        e.mem.set_u32(b + 0x3C, 2);
        // The first guide of the first lod claims 40000 vertices (only the header is read).
        let lods = word(&e, b + 0x20 + 0x0C);
        let guides = word(&e, lods + 0x0C);
        e.mem.set_u32(guides + 0x0C, 0x1000);
        e.mem.set_u32(guides + 0x10, 0x1000 + 0x38 * 40000);
        let geometry = e.new_object::<CIndexedGeometry>();
        let formatted: Record<(u32, u32)> = Rc::new(RefCell::new(Vec::new()));
        let sink = formatted.clone();
        e.register_double(VSPRINTF, move |e, a| {
            sink.borrow_mut().push((a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        spy(&mut e, STRING_FROM_CHARS, |_, a| a[0]);
        let base = spy(&mut e, BASE_EXCEPTION_CONSTRUCT, |_, _| 0);
        let copied = spy(&mut e, STRING_COPY, |_, _| 0);
        let seen: Record<(u32, u32, u32)> = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(THROW, move |e, a| {
            sink.borrow_mut().push((a[1], a[0], e.mem.u32(a[0])));
            Ret::default()
        });
        let levels = spy(&mut e, GEOMETRY_SET_NUM_LOD_LEVELS, |_, _| 0);
        e.call(0x00b1_8fd0, &args![this, geometry, 0u32]);
        assert_eq!(*formatted.borrow(), vec![(VERTICES_EXCEED_MESSAGE, 0xFFFF)]);
        let (info, exception, vtable) = seen.borrow()[0];
        assert_eq!(info, OVERFLOW_THROW_INFO);
        assert_eq!(vtable, OVERFLOW_EXCEPTION_VTABLE);
        assert_eq!(*base.borrow(), vec![vec![exception]]);
        assert_eq!(copied.borrow()[0][0], exception + 0x0C);
        assert!(levels.borrow().is_empty());
    }

    /// Doubles that give the vectors `BuildGuideLods` fills real storage and make the guide
    /// helpers move and copy memory: the guide vector's reserve (room for eight guides), the
    /// element constructors (guide and guide list), the move-down of an erase and the iterator
    /// advance.
    fn lod_doubles(e: &mut Engine) {
        spy(e, ALLOCATE_PROXY, |e, _| e.mem.alloc(8));
        spy(e, 0x00b1_eb00, |e, a| {
            let block = e.mem.alloc(0x38 * 8);
            e.mem.set_u32(a[0] + 0x0C, block);
            e.mem.set_u32(a[0] + 0x10, block);
            e.mem.set_u32(a[0] + 0x14, block + 0x38 * 8);
            0
        });
        spy(e, GUIDE_FILL, |e, a| {
            for k in 0..a[1] {
                for w in 0..14 {
                    let v = e.mem.u32(a[2] + 4 * w);
                    e.mem.set_u32(a[0] + 0x38 * k + 4 * w, v);
                }
            }
            0
        });
        spy(e, 0x00b2_4290, |e, a| {
            for w in 0..6 {
                let v = e.mem.u32(a[2] + 4 * w);
                e.mem.set_u32(a[0] + 4 * w, v);
            }
            0
        });
        spy(e, GUIDE_MOVE_DOWN, |e, a| {
            let bytes = e.mem.bytes(a[0], a[1] - a[0]);
            e.mem.write(a[2], &bytes);
            a[2] + (a[1] - a[0])
        });
        spy(e, ITERATOR_ADVANCE, |e, a| {
            let position = e.mem.u32(a[0] + 4) + 0x38 * a[1];
            e.mem.set_u32(a[0] + 4, position);
            0
        });
    }

    #[test]
    fn build_guide_lods_keeps_the_large_guides_out_of_the_reduction() {
        let mut e = engine();
        lod_doubles(&mut e);
        let this = frond(&mut e);
        let b = this.addr();
        let guides = vector(&mut e, b + 8, 0x38, 4, 4);
        for (k, area) in [10.0f32, 6.0, 3.0, 2.0].iter().enumerate() {
            e.mem.set_f32(guides + 0x38 * k as u32 + 0x28, *area);
        }
        vector(&mut e, b + 0x20, 0x18, 0, 4);
        e.mem.set_u32(b + 0x68, 2);
        put(&mut e, b + 0x6C, &[1.0, 0.2, 0.3, 0.25]);
        let draws = feed_floats(&mut e, GET_UNIFORM, &[0.25, 0.5, 0.75]);
        let sorts = spy(&mut e, GUIDE_SORT, |_, _| 0);
        let inserts = spy(&mut e, 0x00b1_ecf0, |_, _| 0);
        e.call(0x00b1_c850, &args![this]);
        // Guide 0 (10 > 10 * (1 - 0.25)) leaves the vector; the others get a fuzzy area
        // `r * 10^2 + (1 - r) * area` with one random draw each in `[0, 0.3)`.
        let first = word(&e, b + 8 + 0x0C);
        let last = word(&e, b + 8 + 0x10);
        assert_eq!((last - first) / 0x38, 3);
        let areas: Vec<f32> = (0..3).map(|k| e.mem.f32(first + 0x38 * k + 0x28)).collect();
        assert_eq!(areas, vec![6.0, 3.0, 2.0]);
        let fuzzy: Vec<f32> = (0..3).map(|k| e.mem.f32(first + 0x38 * k + 0x2C)).collect();
        assert_eq!(fuzzy, vec![29.5, 51.5, 75.5]);
        let draws = draws.borrow().clone();
        assert_eq!(draws.len(), 3);
        assert_eq!(draws[0][1..], [0, 0.3f32.to_bits()]);
        // Both lists are sorted: the three guides left, the one removed.
        let sorted: Vec<(u32, u32)> = sorts.borrow().iter().map(|a| (a[2], a[3])).collect();
        assert_eq!(sorted, vec![(3, 0), (1, 0)]);
        // The removed guide goes back in at the front.
        let inserted = inserts.borrow().clone();
        assert_eq!(inserted.len(), 1);
        assert_eq!(inserted[0][2], first);
        assert_eq!(inserted[0][3..4], [1]);
        // The lods: the budget is the total area (21) times 1.0 for lod 0 and 0.2 for lod 1.
        let rows = word(&e, b + 0x20 + 0x0C);
        assert_eq!((word(&e, b + 0x20 + 0x10) - rows) / 0x18, 2);
        let lod_areas = |e: &Engine, row: u32| -> Vec<f32> {
            let data = word(e, row + 0x0C);
            let end = word(e, row + 0x10);
            (0..(end - data) / 0x38)
                .map(|k| e.mem.f32(data + 0x38 * k + 0x28))
                .collect()
        };
        assert_eq!(lod_areas(&e, rows), vec![6.0, 3.0, 2.0]);
        assert_eq!(lod_areas(&e, rows + 0x18), vec![6.0]);
    }

    #[test]
    fn build_guide_lods_with_one_lod_uses_the_maximum_percent() {
        let mut e = engine();
        lod_doubles(&mut e);
        let this = frond(&mut e);
        let b = this.addr();
        let guides = vector(&mut e, b + 8, 0x38, 2, 2);
        e.mem.set_f32(guides + 0x28, 4.0);
        e.mem.set_f32(guides + 0x38 + 0x28, 2.0);
        vector(&mut e, b + 0x20, 0x18, 0, 4);
        e.mem.set_u32(b + 0x68, 1);
        // Maximum percent 0.5 of a total of 6: a budget of 3; the first guide (4) already
        // passes it, so only it is taken.
        put(&mut e, b + 0x6C, &[0.5, 0.0, 0.0, 0.0]);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        spy(&mut e, GUIDE_SORT, |_, _| 0);
        feed_floats(&mut e, GET_UNIFORM, &[0.0, 0.0]);
        e.call(0x00b1_c850, &args![this]);
        let rows = word(&e, b + 0x20 + 0x0C);
        assert_eq!((word(&e, b + 0x20 + 0x10) - rows) / 0x18, 1);
        let data = word(&e, rows + 0x0C);
        let end = word(&e, rows + 0x10);
        assert_eq!((end - data) / 0x38, 1);
        assert_eq!(e.mem.f32(data + 0x28), 4.0);
    }

    #[test]
    fn build_guide_lods_with_a_zero_budget_gives_an_empty_list() {
        let mut e = engine();
        lod_doubles(&mut e);
        let this = frond(&mut e);
        let b = this.addr();
        let guides = vector(&mut e, b + 8, 0x38, 1, 1);
        e.mem.set_f32(guides + 0x28, 4.0);
        vector(&mut e, b + 0x20, 0x18, 0, 4);
        e.mem.set_u32(b + 0x68, 1);
        put(&mut e, b + 0x6C, &[0.0, 0.0, 0.0, 0.0]);
        e.set_global(ZERO_DOUBLE, 0.0f64);
        spy(&mut e, GUIDE_SORT, |_, _| 0);
        feed_floats(&mut e, GET_UNIFORM, &[0.0]);
        e.call(0x00b1_c850, &args![this]);
        let rows = word(&e, b + 0x20 + 0x0C);
        assert_eq!((word(&e, b + 0x20 + 0x10) - rows) / 0x18, 1);
        assert_eq!(word(&e, rows + 0x0C), word(&e, rows + 0x10));
    }

    /// The adders of a geometry as the exe's callees saw them: (adder address, the floats it
    /// was given; the texture coordinate adder also the texture index, the wind weight adder
    /// the group).
    type Adds = Record<(u32, Vec<f32>)>;

    /// Doubles for the geometry: positions and texture coordinates are stored per vertex at the
    /// geometry's vertex counter (`+0x22`) in two tables the pointer getters read, the other
    /// adders only record. Returns the two tables and the record.
    fn geometry_doubles(e: &mut Engine) -> (u32, u32, Adds) {
        let coords = e.mem.alloc(12 * 64);
        let tex = e.mem.alloc(8 * 64);
        let adds: Adds = Rc::new(RefCell::new(Vec::new()));
        let sink = adds.clone();
        e.register_double(GEOMETRY_ADD_COORDINATE, move |e, a| {
            let index = u32::from(e.mem.u16(a[0] + 0x22));
            for k in 0..3 {
                let v = e.mem.u32(a[1] + 4 * k);
                e.mem.set_u32(coords + 12 * index + 4 * k, v);
            }
            sink.borrow_mut()
                .push((GEOMETRY_ADD_COORDINATE, floats(e, a[1], 3)));
            Ret::default()
        });
        let sink = adds.clone();
        e.register_double(GEOMETRY_ADD_TEX_COORD, move |e, a| {
            let index = u32::from(e.mem.u16(a[0] + 0x22));
            for k in 0..2 {
                let v = e.mem.u32(a[1] + 4 * k);
                e.mem.set_u32(tex + 8 * index + 4 * k, v);
            }
            let mut seen = floats(e, a[1], 2);
            seen.push(a[2] as f32);
            sink.borrow_mut().push((GEOMETRY_ADD_TEX_COORD, seen));
            Ret::default()
        });
        for (adder, count) in [
            (GEOMETRY_ADD_COLOR, 4),
            (GEOMETRY_ADD_NORMAL, 3),
            (GEOMETRY_ADD_TANGENT, 3),
            (GEOMETRY_ADD_BINORMAL, 3),
        ] {
            let sink = adds.clone();
            e.register_double(adder, move |e, a| {
                sink.borrow_mut().push((adder, floats(e, a[1], count)));
                Ret::default()
            });
        }
        let sink = adds.clone();
        e.register_double(GEOMETRY_ADD_WIND_WEIGHT, move |_, a| {
            sink.borrow_mut().push((
                GEOMETRY_ADD_WIND_WEIGHT,
                vec![f32::from_bits(a[1]), a[2] as f32],
            ));
            Ret::default()
        });
        spy(e, GEOMETRY_COORD_POINTER, move |_, a| coords + 12 * a[1]);
        spy(e, GEOMETRY_TEX_COORD_POINTER, move |_, a| tex + 8 * a[1]);
        (coords, tex, adds)
    }

    /// The adds of one kind, in order.
    fn adds_of(adds: &Adds, adder: u32) -> Vec<Vec<f32>> {
        adds.borrow()
            .iter()
            .filter(|(a, _)| *a == adder)
            .map(|(_, v)| v.clone())
            .collect()
    }

    /// The `CVec3` helpers as plain doubles; the product with a rotation leaves the vector as
    /// it is (the identity).
    fn vec3_doubles(e: &mut Engine) {
        e.register_double(VEC3_ADD, |e, a| {
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) + e.mem.f32(a[2] + 4 * k);
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register_double(VEC3_SUBTRACT, |e, a| {
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) - e.mem.f32(a[2] + 4 * k);
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register_double(VEC3_SCALE, |e, a| {
            let scale = f32::from_bits(a[2]);
            for k in 0..3 {
                let v = e.mem.f32(a[0] + 4 * k) * scale;
                e.mem.set_f32(a[1] + 4 * k, v);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register_double(VEC3_TIMES_ROTATION, |e, a| {
            for k in 0..3 {
                let v = e.mem.u32(a[0] + 4 * k);
                e.mem.set_u32(a[1] + 4 * k, v);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register_double(VEC3_CROSS, |e, a| {
            let (p, q) = (floats(e, a[0], 3), floats(e, a[2], 3));
            let cross = [
                p[1] * q[2] - p[2] * q[1],
                p[2] * q[0] - p[0] * q[2],
                p[0] * q[1] - p[1] * q[0],
            ];
            put(e, a[1], &cross);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.register_double(VEC3_NORMALIZE, |e, a| {
            let v = floats(e, a[0], 3);
            let length = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
            if length > 0.0 {
                put(e, a[0], &[v[0] / length, v[1] / length, v[2] / length]);
            }
            Ret::default()
        });
        e.register_double(VEC3_LENGTH, |e, a| {
            let v = floats(e, a[0], 3);
            Ret {
                st0: f64::from((v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()),
                ..Ret::default()
            }
        });
        distance_double(e);
    }

    /// Appends the `size` bytes at `from` to the `std::vector` header at `vector`, allocating
    /// room for sixteen elements on the first append.
    fn append(e: &mut Engine, vector: u32, size: u32, from: u32) {
        let mut first = e.mem.u32(vector + 0x0C);
        if first == 0 {
            first = e.mem.alloc(size * 16);
            e.mem.set_u32(vector + 0x0C, first);
            e.mem.set_u32(vector + 0x10, first);
            e.mem.set_u32(vector + 0x14, first + size * 16);
        }
        let last = e.mem.u32(vector + 0x10);
        let bytes = e.mem.bytes(from, size);
        e.mem.write(last, &bytes);
        e.mem.set_u32(vector + 0x10, last + size);
    }

    /// The vector helpers of the geometry builders as doubles: the `CVec3` and float vectors
    /// really append and index, and the vector of float vectors has room for eight rows.
    fn vector_doubles(e: &mut Engine) {
        for addr in [CVEC3_VECTOR_CONSTRUCT, FLOAT_VECTOR_CONSTRUCT] {
            e.register_double(addr, |e, a| {
                for k in 0..6 {
                    e.mem.set_u32(a[0] + 4 * k, 0);
                }
                Ret::default()
            });
        }
        e.register_double(CVEC3_VECTOR_PUSH, |e, a| {
            append(e, a[0], 12, a[1]);
            Ret::default()
        });
        e.register_double(FLOAT_VECTOR_PUSH, |e, a| {
            append(e, a[0], 4, a[1]);
            Ret::default()
        });
        spy(e, FLOAT_VECTOR_AT, |e, a| e.mem.u32(a[0] + 0x0C) + 4 * a[1]);
        e.register_double(VECTOR_VECTOR_CONSTRUCT, |e, a| {
            let block = e.mem.alloc(0x18 * 8);
            e.mem.set_u32(a[0] + 0x0C, block);
            e.mem.set_u32(a[0] + 0x10, block);
            e.mem.set_u32(a[0] + 0x14, block + 0x18 * 8);
            Ret::default()
        });
        e.register_double(0x00b2_4580, |e, a| {
            for k in 0..6 {
                let v = e.mem.u32(a[2] + 4 * k);
                e.mem.set_u32(a[0] + 4 * k, v);
            }
            Ret::default()
        });
    }

    /// The rotation about X the angle helper builds, recorded; the product leaves the matrix
    /// it multiplies unchanged.
    fn rotation_double(e: &mut Engine) -> Record<Vec<f32>> {
        let seen: Record<Vec<f32>> = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(CROT_TRANSFORM_MULTIPLY, move |e, a| {
            sink.borrow_mut().push(floats(e, a[0], 9));
            for k in 0..9 {
                let v = e.mem.u32(a[2] + 4 * k);
                e.mem.set_u32(a[1] + 4 * k, v);
            }
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        seen
    }

    /// Writes the globals the geometry builders read.
    fn builder_globals(e: &mut Engine) {
        e.set_global(BLADE_ANGLE_NUMERATOR, 180.0f64);
        e.set_global(ONE_DOUBLE, 1.0f64);
        e.set_global(SCALE_DOUBLE, 0.5f64);
        e.set_global(MIRROR_FACTOR, -1.0f64);
        e.set_global(HALF_FLOAT, 0.5f32);
        e.set_global(MINIMUM_NORMAL_LENGTH, 0.1f32);
        e.set_global(TEXTURE_SCALE, 0.999f32);
        e.set_global(PROFILE_ANGLE_OFFSET, 0.0f32);
        e.set_global(0x010a_4d90u32, 1.0f64);
    }

    /// Writes a guide with the given vertices: position `(0, 0, z)`, the identity transform, the
    /// weight and the wind group; the radius 1, the map index 3 and a zero offset angle.
    fn blade_guide(e: &mut Engine, vertices: &[(f32, f32, u8)]) -> u32 {
        let guide = e.mem.alloc(0x38);
        let count = vertices.len() as u32;
        let data = guide_with(e, guide, count, count);
        for (i, (z, weight, group)) in vertices.iter().enumerate() {
            let v = data + 0x38 * i as u32;
            put(e, v, &[0.0, 0.0, *z]);
            put(e, v + 0x0C, &[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0]);
            e.mem.set_f32(v + 0x30, *weight);
            e.mem.set_u32(v + 0x34, u32::from(*group));
        }
        e.mem.set_f32(guide + 0x1C, 1.0);
        e.mem.set_u8(guide + 0x20, 3);
        e.mem.set_f32(guide + 0x24, 0.0);
        guide
    }

    /// An engine ready to build geometry: a geometry whose vertex counter is `counter` (with
    /// vertex weighting if `weighting`) and a lighting engine.
    fn builder_engine(
        e: &mut Engine,
        counter: u16,
        weighting: bool,
    ) -> (Ptr<CFrondEngine>, Ptr<CIndexedGeometry>) {
        let this = frond(e);
        let geometry = e.new_object::<CIndexedGeometry>();
        e.mem.set_u16(geometry.addr() + 0x22, counter);
        e.mem.set_u8(geometry.addr() + 8, u8::from(weighting));
        e.mem.set_u32(this.addr(), geometry.addr());
        e.mem.set_u32(this.addr() + 4, 0x1);
        (this, geometry)
    }

    #[test]
    fn blade_geometry_is_written_for_every_vertex_pair() {
        let mut e = engine();
        builder_globals(&mut e);
        vec3_doubles(&mut e);
        vector_doubles(&mut e);
        let (coords, tex, adds) = geometry_doubles(&mut e);
        let rotations = rotation_double(&mut e);
        let (this, geometry) = builder_engine(&mut e, 7, true);
        e.mem.set_u32(this.addr() + 0x3C, 2);
        let guide = blade_guide(&mut e, &[(0.0, 0.5, 1), (10.0, 0.25, 2), (30.0, 0.125, 3)]);
        e.call(0x00b1_9ad0, &args![this, guide]);
        let g = geometry.addr();
        // The guide's start index and its two vertices per guide vertex.
        assert_eq!(word(&e, guide + 0x30), 7);
        assert_eq!(word(&e, guide + 0x34), 2);
        // Two blades of three vertex pairs, every vertex with all its attributes.
        assert_eq!(adds_of(&adds, GEOMETRY_ADD_COORDINATE).len(), 12);
        assert_eq!(adds_of(&adds, GEOMETRY_ADD_TEX_COORD).len(), 16);
        assert_eq!(adds_of(&adds, GEOMETRY_ADD_COLOR).len(), 12);
        assert_eq!(adds_of(&adds, GEOMETRY_ADD_NORMAL).len(), 12);
        assert_eq!(adds_of(&adds, GEOMETRY_ADD_TANGENT).len(), 12);
        assert_eq!(adds_of(&adds, GEOMETRY_ADD_BINORMAL).len(), 12);
        assert_eq!(adds_of(&adds, GEOMETRY_ADD_WIND_WEIGHT).len(), 12);
        // Blade 0: the first vertex is the position plus and minus the radius times the blade
        // direction (0, 1, 0); the middle one averages with the previous ring; the last one
        // uses the previous direction.
        let positions = adds_of(&adds, GEOMETRY_ADD_COORDINATE);
        let expected: Vec<Vec<f32>> = vec![
            vec![0.0, 1.0, 0.0],
            vec![0.0, -1.0, 0.0],
            vec![0.0, 1.0, 10.0],
            vec![0.0, -1.0, 10.0],
            vec![0.0, 1.0, 30.0],
            vec![0.0, -1.0, 30.0],
        ];
        assert_eq!(positions[..6], expected[..]);
        assert_eq!(positions[6..], expected[..]);
        // The white colour, the guide direction as normal, the blade direction as tangent and
        // their cross product as binormal.
        let colors = adds_of(&adds, GEOMETRY_ADD_COLOR);
        assert_eq!(colors[0], vec![1.0, 1.0, 1.0, 1.0]);
        assert_eq!(adds_of(&adds, GEOMETRY_ADD_NORMAL)[0], vec![0.0, 0.0, 1.0]);
        assert_eq!(adds_of(&adds, GEOMETRY_ADD_TANGENT)[0], vec![0.0, 1.0, 0.0]);
        assert_eq!(
            adds_of(&adds, GEOMETRY_ADD_BINORMAL)[0],
            vec![1.0, 0.0, 0.0]
        );
        // The wind weight and group of each vertex, once per geometry vertex of the pair.
        let weights = adds_of(&adds, GEOMETRY_ADD_WIND_WEIGHT);
        assert_eq!(
            weights[..6],
            [
                vec![0.5, 1.0],
                vec![0.5, 1.0],
                vec![0.25, 2.0],
                vec![0.25, 2.0],
                vec![0.125, 3.0],
                vec![0.125, 3.0]
            ]
        );
        // The first texture coordinates: 1 on the near edge, 0 on the far one, the position
        // along the guide, and the map index 3.
        let textures = adds_of(&adds, GEOMETRY_ADD_TEX_COORD);
        assert_eq!(textures[0], vec![1.0, 0.0, 3.0]);
        assert_eq!(textures[1], vec![0.0, 0.0, 3.0]);
        assert_eq!(textures[2], vec![1.0, 0.5, 3.0]);
        assert_eq!(textures[4], vec![1.0, 1.0, 3.0]);
        // Then the rewritten texture coordinates of the middle vertex pair: the accumulated
        // edge length (10 of 30) times 0.999.
        let rewritten = (10.0f64 / 30.0 * f64::from(0.999f32)) as f32;
        assert_eq!(textures[6], vec![1.0, rewritten, 3.0]);
        assert_eq!(textures[7], vec![0.0, rewritten, 3.0]);
        assert_eq!(floats(&e, tex + 8 * 9, 2), vec![1.0, rewritten]);
        assert_eq!(floats(&e, tex + 8 * 10, 2), vec![0.0, rewritten]);
        // The position table holds the vertices the pointer getter was asked for.
        assert_eq!(floats(&e, coords + 12 * 7, 3), vec![0.0, 1.0, 0.0]);
        // The second blade starts after the first one's six vertices; the counter ends
        // after the last.
        assert_eq!(e.mem.u16(g + 0x22), 19);
        assert_eq!(floats(&e, coords + 12 * 13, 3), vec![0.0, 1.0, 0.0]);
        // The rotations: blade 0 by 0, blade 1 by 90 (one per vertex).
        let rotations = rotations.borrow().clone();
        assert_eq!(rotations.len(), 6);
        assert_eq!(
            rotations[0],
            vec![1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, -0.0, 1.0]
        );
        let turned = (90.0f64).cos() as f32;
        assert_eq!(rotations[3][4], turned);
        assert_eq!(rotations[3][8], turned);
    }

    #[test]
    fn blade_geometry_without_weighting_or_lighting() {
        let mut e = engine();
        builder_globals(&mut e);
        vec3_doubles(&mut e);
        vector_doubles(&mut e);
        let (_, _, adds) = geometry_doubles(&mut e);
        rotation_double(&mut e);
        let (this, geometry) = builder_engine(&mut e, 0, false);
        e.mem.set_u32(this.addr() + 0x3C, 1);
        let guide = blade_guide(&mut e, &[(0.0, 0.5, 1), (4.0, 0.25, 2)]);
        e.call(0x00b1_9ad0, &args![this, guide]);
        assert_eq!(adds_of(&adds, GEOMETRY_ADD_COORDINATE).len(), 4);
        assert!(adds_of(&adds, GEOMETRY_ADD_WIND_WEIGHT).is_empty());
        // Two vertices: the last one is a vertex pair too, and the counter ends after both.
        assert_eq!(e.mem.u16(geometry.addr() + 0x22), 4);
        assert_eq!(word(&e, guide + 0x30), 0);
        // Without a lighting engine nothing is done.
        let (this, geometry) = builder_engine(&mut e, 0, false);
        e.mem.set_u32(this.addr() + 4, 0);
        e.mem.set_u32(this.addr() + 0x3C, 1);
        e.call(0x00b1_9ad0, &args![this, guide]);
        assert_eq!(adds_of(&adds, GEOMETRY_ADD_COORDINATE).len(), 4);
        assert_eq!(e.mem.u16(geometry.addr() + 0x22), 0);
    }

    #[test]
    fn cross_section_geometry_is_written_for_every_profile_point() {
        let mut e = engine();
        builder_globals(&mut e);
        vec3_doubles(&mut e);
        vector_doubles(&mut e);
        let (coords, tex, adds) = geometry_doubles(&mut e);
        rotation_double(&mut e);
        let (this, geometry) = builder_engine(&mut e, 7, true);
        let b = this.addr();
        e.mem.set_u32(b + 0x40, 0x7000);
        e.mem.set_u32(b + 0x44, 3);
        spy_float(&mut e, SPLINE_EVALUATE, |_, a| {
            f64::from(f32::from_bits(a[1]))
        });
        spy_float(&mut e, ATAN2, |_, a| {
            f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32))
                .atan2(f64::from_bits(u64::from(a[2]) | (u64::from(a[3]) << 32)))
        });
        let guide = blade_guide(&mut e, &[(0.0, 0.5, 1), (10.0, 0.25, 2)]);
        e.call(0x00b1_b000, &args![this, guide]);
        // The profile of three segments mirrored has five points.
        assert_eq!(word(&e, guide + 0x34), 5);
        assert_eq!(word(&e, guide + 0x30), 7);
        let positions = adds_of(&adds, GEOMETRY_ADD_COORDINATE);
        assert_eq!(positions.len(), 10);
        // Vertex 0 at z = 0: the position plus the profile point `(0, -t, t)`, the mirrored half
        // coming back up.
        let profile: [[f32; 2]; 5] = [
            [-1.0, 1.0],
            [-0.5, 0.5],
            [-0.0, 0.0],
            [0.5, 0.5],
            [1.0, 1.0],
        ];
        for (j, point) in profile.iter().enumerate() {
            assert_eq!(positions[j], vec![0.0, point[0], point[1]]);
            assert_eq!(positions[5 + j], vec![0.0, point[0], point[1] + 10.0]);
        }
        // The texture coordinates: the position of the point in the profile and of the vertex
        // along the guide.
        let textures = adds_of(&adds, GEOMETRY_ADD_TEX_COORD);
        assert_eq!(textures.len(), 15);
        assert_eq!(textures[0], vec![0.0, 0.0, 3.0]);
        assert_eq!(textures[2], vec![0.5, 0.0, 3.0]);
        assert_eq!(textures[9], vec![1.0, 1.0, 3.0]);
        // Colour, normal, tangent and binormal once per geometry vertex; wind weights too.
        for adder in [
            GEOMETRY_ADD_COLOR,
            GEOMETRY_ADD_NORMAL,
            GEOMETRY_ADD_TANGENT,
            GEOMETRY_ADD_BINORMAL,
            GEOMETRY_ADD_WIND_WEIGHT,
        ] {
            assert_eq!(adds_of(&adds, adder).len(), 10);
        }
        assert_eq!(adds_of(&adds, GEOMETRY_ADD_WIND_WEIGHT)[0], vec![0.5, 1.0]);
        assert_eq!(adds_of(&adds, GEOMETRY_ADD_WIND_WEIGHT)[9], vec![0.25, 2.0]);
        // The second pass rewrites the second texture coordinate of the second ring: the length
        // along the column (10) over the column's total (10) times 0.999, the first one kept.
        let rewritten = (10.0f64 / 10.0 * f64::from(0.999f32)) as f32;
        for c in 0..5u32 {
            let row = floats(&e, tex + 8 * (7 + 5 + c), 2);
            assert_eq!(row, vec![c as f32 / 4.0, rewritten]);
        }
        assert_eq!(e.mem.u16(geometry.addr() + 0x22), 17);
        assert_eq!(floats(&e, coords + 12 * 7, 3), vec![0.0, -1.0, 1.0]);
    }

    #[test]
    fn cross_section_geometry_needs_a_lighting_engine() {
        let mut e = engine();
        builder_globals(&mut e);
        vec3_doubles(&mut e);
        vector_doubles(&mut e);
        let (_, _, adds) = geometry_doubles(&mut e);
        let (this, _) = builder_engine(&mut e, 0, false);
        e.mem.set_u32(this.addr() + 4, 0);
        let guide = blade_guide(&mut e, &[(0.0, 0.5, 1)]);
        e.call(0x00b1_b000, &args![this, guide]);
        assert!(adds.borrow().is_empty());
        let (this, _) = builder_engine(&mut e, 0, false);
        e.mem.set_u32(this.addr(), 0);
        e.call(0x00b1_b000, &args![this, guide]);
        assert!(adds.borrow().is_empty());
    }

    /// A strip registration of `fn_00b1b960`: the level, the length and the indices.
    fn strip_of(
        e: &mut Engine,
        this: Ptr<CFrondEngine>,
        level: u32,
        guide: u32,
    ) -> (u32, Vec<u16>) {
        let strips = strip_double(e);
        e.call(0x00b1_b960, &args![this, level, 100u32, guide]);
        let strips = strips.borrow();
        assert_eq!(strips.len(), 1);
        strips[0].clone()
    }

    #[test]
    fn strip_indices_zigzag_over_the_rings() {
        let mut e = engine();
        e.set_global(ONE_DOUBLE, 1.0f64);
        let this = frond(&mut e);
        let geometry = e.new_object::<CIndexedGeometry>();
        let b = this.addr();
        e.mem.set_u32(b, geometry.addr());
        e.mem.set_u32(b + 0x68, 1);
        e.mem.set_u32(b + 0x7C, 1);
        e.mem.set_u32(b + 0x80, 1);
        let guide = e.mem.alloc(0x38);
        guide_with(&mut e, guide, 3, 3);
        e.mem.set_u32(guide + 0x34, 5);
        let (level, indices) = strip_of(&mut e, this, 0, guide);
        assert_eq!(level, 0);
        // Two rows of five column pairs; the even row left to right, the odd row right to left,
        // the last index repeated to join them.
        let expected: Vec<u16> = vec![
            100, 105, 101, 106, 102, 107, 103, 108, 104, 109, 109, // row 0
            109, 114, 108, 113, 107, 112, 106, 111, 105, 110, // row 1
        ];
        assert_eq!(indices, expected);
        assert_eq!(e.mem.u16(geometry.addr() + 0x26), 1);
    }

    #[test]
    fn strip_segments_shrink_with_the_level() {
        let mut e = engine();
        e.set_global(ONE_DOUBLE, 1.0f64);
        let this = frond(&mut e);
        let geometry = e.new_object::<CIndexedGeometry>();
        let b = this.addr();
        e.mem.set_u32(b, geometry.addr());
        e.mem.set_u32(b + 0x68, 3);
        e.mem.set_u32(b + 0x7C, 1);
        e.mem.set_u32(b + 0x80, 1);
        let guide = e.mem.alloc(0x38);
        guide_with(&mut e, guide, 3, 3);
        e.mem.set_u32(guide + 0x34, 5);
        // Level 1 of 3: s = 0.5, so 2.5 length segments (2) and 4 cross segments.
        let (level, indices) = strip_of(&mut e, this, 1, guide);
        assert_eq!(level, 1);
        let expected: Vec<u16> = vec![100, 110, 101, 111, 103, 113, 104, 114];
        assert_eq!(indices, expected);
    }

    #[test]
    fn strips_need_a_geometry() {
        let mut e = engine();
        let this = frond(&mut e);
        let strips = strip_double(&mut e);
        e.call(0x00b1_b960, &args![this, 0u32, 0u32, 0u32]);
        assert!(strips.borrow().is_empty());
    }

    #[test]
    fn profile_points_and_normals_follow_the_spline() {
        let mut e = engine();
        builder_globals(&mut e);
        e.set_global(PROFILE_ANGLE_OFFSET, 0.5f32);
        vec3_doubles(&mut e);
        vector_doubles(&mut e);
        let this = frond(&mut e);
        let b = this.addr();
        e.mem.set_u32(b + 0x40, 0x7000);
        e.mem.set_u32(b + 0x44, 3);
        spy_float(&mut e, SPLINE_EVALUATE, |_, a| {
            f64::from(f32::from_bits(a[1]))
        });
        spy_float(&mut e, ATAN2, |_, a| {
            f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32))
                .atan2(f64::from_bits(u64::from(a[2]) | (u64::from(a[3]) << 32)))
        });
        let guide = e.mem.alloc(0x38);
        e.mem.set_f32(guide + 0x1C, 2.0);
        let points = e.mem.alloc(0x18);
        let normals = e.mem.alloc(0x18);
        e.call(0x00b1_be50, &args![this, guide, points, normals]);
        // Radius 2: the points (0, -2t, 2t) for t = 1, 0.5, 0, then the first two mirrored.
        let count = (word(&e, points + 0x10) - word(&e, points + 0x0C)) / 12;
        assert_eq!(count, 5);
        let first = word(&e, points + 0x0C);
        let seen: Vec<Vec<f32>> = (0..5).map(|k| floats(&e, first + 12 * k, 3)).collect();
        assert_eq!(
            seen,
            vec![
                vec![0.0, -2.0, 2.0],
                vec![0.0, -1.0, 1.0],
                vec![0.0, -0.0, 0.0],
                vec![0.0, 1.0, 1.0],
                vec![0.0, 2.0, 2.0],
            ]
        );
        // The normals: (0, cos a, sin a) with a the direction of the profile (the mean of its
        // two sides in the middle) plus the angle offset; the point at `segments - 1` uses
        // just the offset.
        let quarter = std::f64::consts::FRAC_PI_4;
        let offset = 0.5f64;
        let angles = [
            -quarter + offset,
            -quarter + offset,
            offset,
            quarter + offset,
            quarter + offset,
        ];
        let normal_start = word(&e, normals + 0x0C);
        assert_eq!((word(&e, normals + 0x10) - normal_start) / 12, 5);
        for (k, angle) in angles.iter().enumerate() {
            let n = floats(&e, normal_start + 12 * k as u32, 3);
            assert_eq!(n[0], 0.0);
            assert!((f64::from(n[1]) - angle.cos()).abs() < 1e-5, "{k}: {n:?}");
            assert!((f64::from(n[2]) - angle.sin()).abs() < 1e-5, "{k}: {n:?}");
        }
    }

    #[test]
    fn texture_coordinates_are_set_for_every_vertex() {
        let mut e = engine();
        let geometry = e.new_object::<CIndexedGeometry>();
        let g = geometry.addr();
        // Two vertices' worth of coordinates (six floats).
        vector(&mut e, g + 0x88, 4, 6, 6);
        e.mem.set_u16(g + 0x22, 9);
        let seen: Record<(Vec<u32>, Vec<f32>)> = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(GEOMETRY_SET_TEX_COORDS, move |e, a| {
            sink.borrow_mut().push((a.to_vec(), floats(e, a[2], 8)));
            Ret::default()
        });
        let coords = e.mem.alloc(0x20);
        put(&mut e, coords, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0]);
        e.call(0x00b1_c6d0, &args![0u32, geometry, 2u32, coords, 1u8]);
        assert_eq!(seen.borrow().len(), 2);
        let flipped = vec![1.0, -2.0, 3.0, -4.0, 5.0, -6.0, 7.0, -8.0];
        for (call, values) in seen.borrow().iter() {
            assert_eq!(call[..2], [g, 2]);
            assert_eq!(*values, flipped);
        }
        assert_eq!(e.mem.u16(g + 0x22), 2);
        // Without the flip flag the floats are passed as they are.
        e.call(0x00b1_c6d0, &args![0u32, geometry, 2u32, coords, 0u8]);
        assert_eq!(seen.borrow().len(), 4);
        let plain = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        assert_eq!(seen.borrow()[2].1, plain);
        assert_eq!(seen.borrow()[3].1, plain);
        assert_eq!(e.mem.u16(g + 0x22), 2);
    }

    #[test]
    fn spline_assignment_copies_the_floats_and_the_vectors() {
        let mut e = engine();
        let source = e.mem.alloc(0x84);
        let target = e.mem.alloc(0x84);
        for (k, bits) in [0x7F80_0001u32, 0x3F80_0000, 0x4000_0000]
            .iter()
            .enumerate()
        {
            e.mem.set_u32(source + 4 * k as u32, *bits);
        }
        for offset in [0x0Cu32, 0x24, 0x3C, 0x54, 0x6C] {
            e.mem.set_u32(source + offset, PROXY);
            e.mem.set_u32(target + offset, PROXY);
        }
        let cleared = spy(&mut e, VECTOR_CLEAR, |_, _| 0);
        let float_cleared = spy(&mut e, FLOAT_VECTOR_CLEAR, |_, _| 0);
        let result = e.call(0x00b1_c7a0, &args![target, source]).u32();
        assert_eq!(result, target);
        // The floats are copied through the x87 stack: the signalling NaN comes back quiet.
        assert_eq!(word(&e, target), 0x7FC0_0001);
        assert_eq!(word(&e, target + 4), 0x3F80_0000);
        assert_eq!(word(&e, target + 8), 0x4000_0000);
        // Empty sources clear the targets: the four `CVec` vectors and the float one.
        let cleared: Vec<u32> = cleared.borrow().iter().map(|a| a[0]).collect();
        assert_eq!(
            cleared,
            vec![target + 0x0C, target + 0x24, target + 0x54, target + 0x6C]
        );
        assert_eq!(*float_cleared.borrow(), vec![vec![target + 0x3C]]);
    }

    #[test]
    fn vector_constructor_from_three_floats_returns_the_vector() {
        let mut e = engine();
        let this = e.mem.alloc(0x0C);
        let nan = f32::from_bits(0x7F80_0001);
        let result = e.call(0x00b1_ada0, &args![this, nan, 2.0f32, 3.0f32]).u32();
        assert_eq!(result, this);
        assert_eq!(word(&e, this), 0x7FC0_0001);
        assert_eq!(floats(&e, this + 4, 2), vec![2.0, 3.0]);
    }

    #[test]
    fn vector_setter_stores_three_floats() {
        let mut e = engine();
        let this = e.mem.alloc(0x0C);
        let nan = f32::from_bits(0xFF80_0001);
        e.call(0x00b1_add0, &args![this, 1.0f32, nan, 3.0f32]);
        assert_eq!(e.mem.f32(this), 1.0);
        assert_eq!(word(&e, this + 4), 0xFFC0_0001);
        assert_eq!(e.mem.f32(this + 8), 3.0);
    }

    #[test]
    fn rotation_about_x_multiplies_the_matrix() {
        let mut e = engine();
        e.set_global(0x010a_4d90u32, 1.0f64);
        let this = e.mem.alloc(0x24);
        let seen: Record<(Vec<f32>, Vec<u32>)> = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        let identity = spy(&mut e, CROT_TRANSFORM_CONSTRUCT, |_, a| a[0]);
        e.register_double(CROT_TRANSFORM_MULTIPLY, move |e, a| {
            sink.borrow_mut().push((floats(e, a[0], 9), a.to_vec()));
            put(e, a[1], &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
            Ret {
                eax: a[1],
                ..Ret::default()
            }
        });
        e.call(0x00b1_ae00, &args![this, 0.5f32]);
        let (rotation, call) = seen.borrow()[0].clone();
        let (cosine, sine) = ((0.5f64).cos() as f32, (0.5f64).sin() as f32);
        assert_eq!(
            rotation,
            vec![1.0, 0.0, 0.0, 0.0, cosine, sine, 0.0, -sine, cosine]
        );
        // `rotation * this`, the product written over `this`.
        assert_eq!(call[0], identity.borrow()[0][0]);
        assert_eq!(call[2], this);
        assert_eq!(
            floats(&e, this, 9),
            vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]
        );
    }

    #[test]
    fn blade_strips_are_registered_per_blade() {
        let mut e = engine();
        let this = frond(&mut e);
        let strips = strip_double(&mut e);
        // No geometry: nothing.
        let vertices = e.mem.alloc(0x18);
        vector(&mut e, vertices, 0x38, 3, 3);
        e.call(0x00b1_aed0, &args![this, 2u16, 0x20u32, vertices]);
        assert!(strips.borrow().is_empty());
        let geometry = e.new_object::<CIndexedGeometry>();
        e.mem.set_u32(this.addr(), geometry.addr());
        e.mem.set_u32(this.addr() + 0x3C, 2);
        e.call(0x00b1_aed0, &args![this, 2u16, 0x20u32, vertices]);
        let expected: Vec<(u32, Vec<u16>)> = vec![
            (2, vec![0x20, 0x21, 0x22, 0x23, 0x24, 0x25]),
            (2, vec![0x26, 0x27, 0x28, 0x29, 0x2A, 0x2B]),
        ];
        assert_eq!(*strips.borrow(), expected);
        assert_eq!(e.mem.u16(geometry.addr() + 0x26), 2);
    }

    /// A vector header at a fresh address holding `count` elements of `size` bytes with room for
    /// `capacity`; returns (the header, the elements).
    fn new_vector(e: &mut Engine, size: u32, count: u32, capacity: u32) -> (u32, u32) {
        let at = e.mem.alloc(0x18);
        let data = vector(e, at, size, count, capacity);
        (at, data)
    }

    /// The `push_back` of a vector of `size`-byte elements: with spare room the element is
    /// constructed at the end by `construct`; without it `insert` is called with the iterator at
    /// the end.
    fn check_push_back(address: u32, size: u32, construct: u32, insert: u32) {
        let mut e = engine();
        let constructed = spy(&mut e, construct, |_, _| 0);
        let inserted = spy(&mut e, insert, |_, _| 0);
        let (roomy, data) = new_vector(&mut e, size, 1, 3);
        e.call(address, &args![roomy, 0x4242u32]);
        assert_eq!(
            *constructed.borrow(),
            vec![vec![data + size, 1, 0x4242, roomy + 8, 0, 0]]
        );
        assert!(inserted.borrow().is_empty());
        assert_eq!(word(&e, roomy + 0x10), data + 2 * size);
        let (full, full_data) = new_vector(&mut e, size, 2, 2);
        e.call(address, &args![full, 0x4343u32]);
        assert_eq!(constructed.borrow().len(), 1);
        let call = inserted.borrow()[0].clone();
        // `this`, the out iterator, the iterator at the end (proxy and position), the value.
        assert_eq!(call[0], full);
        assert_eq!(call[2..], [PROXY, full_data + 2 * size, 0x4343]);
        assert_eq!(word(&e, full + 0x10), full_data + 2 * size);
    }

    #[test]
    fn vertex_vector_push_back_constructs_or_inserts() {
        check_push_back(0x00b1_d980, 0x38, 0x00b2_3f60, 0x00b1_e840);
    }

    #[test]
    fn guide_vector_push_back_constructs_or_inserts() {
        check_push_back(0x00b1_db40, 0x38, 0x00b2_4180, 0x00b1_dc50);
    }

    #[test]
    fn guide_lod_vector_push_back_constructs_or_inserts() {
        check_push_back(0x00b1_df80, 0x18, 0x00b2_4290, 0x00b1_f460);
    }

    #[test]
    fn texture_vector_push_back_constructs_or_inserts() {
        check_push_back(0x00b1_e1e0, 0x2C, 0x00b2_43a0, 0x00b1_f6d0);
    }

    #[test]
    fn float_vector_vector_push_back_constructs_or_inserts() {
        check_push_back(0x00b1_e640, 0x18, 0x00b2_4580, 0x00b1_fd50);
    }

    /// The checked insert of one element: the vector's iterator at `position` is turned into the
    /// element's index, `insert_n` called, and the iterator at the same index made again with
    /// `advance`.
    fn check_insert(address: u32, size: u32, insert_n: u32, advance: u32) {
        let mut e = engine();
        let inserted = spy(&mut e, insert_n, |_, _| 0);
        let advanced = spy(&mut e, advance, move |e, a| {
            let position = e.mem.u32(a[0] + 4) + size * a[1];
            e.mem.set_u32(a[0] + 4, position);
            0
        });
        // An empty vector: index 0.
        let (empty, empty_data) = new_vector(&mut e, size, 0, 4);
        let out = e.mem.alloc(8);
        let result = e
            .call(address, &args![empty, out, PROXY, empty_data, 0x4242u32])
            .u32();
        assert_eq!(result, out);
        assert_eq!(
            *inserted.borrow(),
            vec![vec![empty, PROXY, empty_data, 1, 0x4242]]
        );
        assert_eq!(advanced.borrow()[0][1], 0);
        assert_eq!((word(&e, out), word(&e, out + 4)), (PROXY, empty_data));
        // Three elements, the third position: index 2, the iterator advanced to it.
        let (full, data) = new_vector(&mut e, size, 3, 4);
        e.call(
            address,
            &args![full, out, PROXY, data + 2 * size, 0x4343u32],
        );
        assert_eq!(
            inserted.borrow()[1],
            vec![full, PROXY, data + 2 * size, 1, 0x4343]
        );
        assert_eq!(advanced.borrow()[1][1], 2);
        assert_eq!((word(&e, out), word(&e, out + 4)), (PROXY, data + 2 * size));
    }

    #[test]
    fn guide_vector_insert_remembers_the_index() {
        check_insert(0x00b1_dc50, 0x38, 0x00b1_ecf0, 0x00b2_3460);
    }

    #[test]
    fn vertex_vector_insert_remembers_the_index() {
        check_insert(0x00b1_e840, 0x38, 0x00b2_0360, 0x00b2_3460);
    }

    #[test]
    fn guide_lod_vector_insert_remembers_the_index() {
        check_insert(0x00b1_f460, 0x18, 0x00b2_0ce0, 0x00b1_6890);
    }

    #[test]
    fn texture_vector_insert_remembers_the_index() {
        check_insert(0x00b1_f6d0, 0x2C, 0x00b2_1560, 0x00b2_3520);
    }

    #[test]
    fn insert_with_a_foreign_iterator_takes_the_invalid_path() {
        let mut e = engine();
        let invalid = spy(&mut e, INVALID_PARAMETER, |_, _| 0);
        spy(&mut e, 0x00b1_ecf0, |_, _| 0);
        spy(&mut e, ITERATOR_ADVANCE, |_, _| 0);
        let (full, data) = new_vector(&mut e, 0x38, 3, 4);
        let out = e.mem.alloc(8);
        e.call(0x00b1_dc50, &args![full, out, 0x1234u32, data, 0u32]);
        assert_eq!(invalid.borrow().len(), 1);
    }

    #[test]
    fn clearing_a_guide_vector_erases_the_whole_range() {
        let mut e = engine();
        let erased = spy(&mut e, 0x00b1_ea20, |_, _| 0);
        let (at, data) = new_vector(&mut e, 0x38, 2, 2);
        e.call(0x00b1_de20, &args![at]);
        let call = erased.borrow()[0].clone();
        assert_eq!(call[0], at);
        assert_eq!(call[2..], [PROXY, data, PROXY, data + 0x70]);
    }

    #[test]
    fn clearing_a_texture_vector_erases_the_whole_range() {
        let mut e = engine();
        let erased = spy(&mut e, 0x00b1_f800, |_, _| 0);
        let (at, data) = new_vector(&mut e, 0x2C, 3, 3);
        e.call(0x00b1_e2f0, &args![at]);
        let call = erased.borrow()[0].clone();
        assert_eq!(call[0], at);
        assert_eq!(call[2..], [PROXY, data, PROXY, data + 3 * 0x2C]);
    }

    #[test]
    fn vector_constructors_allocate_the_proxy() {
        let mut e = engine();
        let proxies = spy(&mut e, ALLOCATE_PROXY, |e, _| e.mem.alloc(8));
        let reserved = spy(&mut e, 0x00b1_eb00, |_, _| 0);
        let at = e.mem.alloc(0x18);
        let result = e.call(0x00b1_d8a0, &args![at]).u32();
        assert_eq!(result, at);
        assert_eq!(*proxies.borrow(), vec![vec![1, 0]]);
        let proxy = word(&e, at);
        assert_ne!(proxy, 0);
        assert_eq!(word(&e, proxy), at);
        assert_eq!(*reserved.borrow(), vec![vec![at, 0]]);
        // A failed allocation leaves no proxy.
        let mut e = engine();
        spy(&mut e, ALLOCATE_PROXY, |_, _| 0);
        let reserved = spy(&mut e, 0x00b1_eb00, |_, _| 0);
        let at = e.mem.alloc(0x18);
        e.mem.set_u32(at, 0xCDCD_CDCD);
        e.call(0x00b1_d8a0, &args![at]);
        assert_eq!(word(&e, at), 0);
        assert_eq!(reserved.borrow().len(), 1);
    }

    #[test]
    fn texture_vector_constructor_reserves_nothing() {
        let mut e = engine();
        spy(&mut e, ALLOCATE_PROXY, |e, _| e.mem.alloc(8));
        let at = e.mem.alloc(0x18);
        for offset in [0x0Cu32, 0x10, 0x14] {
            e.mem.set_u32(at + offset, 0xCDCD_CDCD);
        }
        let result = e.call(0x00b1_e090, &args![at]).u32();
        assert_eq!(result, at);
        let proxy = word(&e, at);
        assert_eq!(word(&e, proxy), at);
        // The reservation of zero elements leaves the vector empty.
        for offset in [0x0Cu32, 0x10, 0x14] {
            assert_eq!(word(&e, at + offset), 0);
        }
    }

    #[test]
    fn vector_destructors_tidy_and_free_the_proxy() {
        let mut e = engine();
        let tidied = spy(&mut e, GUIDE_VECTOR_TIDY, |_, _| 0);
        let deleted = spy(&mut e, OPERATOR_DELETE, |_, _| 0);
        let at = e.mem.alloc(0x18);
        e.mem.set_u32(at, 0x5107);
        e.call(0x00b1_da90, &args![at]);
        assert_eq!(*tidied.borrow(), vec![vec![at]]);
        assert_eq!(*deleted.borrow(), vec![vec![0x5107]]);
        // The guide-lod vector without storage: only its proxy goes.
        let lods = e.mem.alloc(0x18);
        e.mem.set_u32(lods, 0x5108);
        e.call(0x00b1_df10, &args![lods]);
        assert_eq!(deleted.borrow()[1], vec![0x5108]);
        // The texture vector too.
        let textures = e.mem.alloc(0x18);
        e.mem.set_u32(textures, 0x5109);
        e.call(0x00b1_e170, &args![textures]);
        assert_eq!(deleted.borrow()[2], vec![0x5109]);
        assert_eq!(deleted.borrow().len(), 3);
    }

    #[test]
    fn guide_lod_vector_tidy_destroys_the_elements() {
        let mut e = engine();
        let destroyed = spy(&mut e, GUIDE_LOD_DESTROY_RANGE, |_, _| 0);
        let deleted = spy(&mut e, OPERATOR_DELETE, |_, _| 0);
        let (at, data) = new_vector(&mut e, 0x18, 2, 4);
        e.call(0x00b1_f650, &args![at]);
        assert_eq!(*destroyed.borrow(), vec![vec![at, data, data + 0x30]]);
        assert_eq!(*deleted.borrow(), vec![vec![data]]);
        assert_eq!(
            [
                word(&e, at + 0x0C),
                word(&e, at + 0x10),
                word(&e, at + 0x14)
            ],
            [0, 0, 0]
        );
        // Without storage nothing is destroyed.
        e.call(0x00b1_f650, &args![at]);
        assert_eq!(destroyed.borrow().len(), 1);
    }

    #[test]
    fn texture_vector_tidy_destroys_the_names() {
        let mut e = engine();
        let tidied = spy(&mut e, STRING_TIDY, |_, _| 0);
        let deleted = spy(&mut e, OPERATOR_DELETE, |_, _| 0);
        let (at, data) = new_vector(&mut e, 0x2C, 2, 4);
        e.call(0x00b1_f9e0, &args![at]);
        assert_eq!(
            *tidied.borrow(),
            vec![vec![data, 1, 0], vec![data + 0x2C, 1, 0]]
        );
        assert_eq!(*deleted.borrow(), vec![vec![data]]);
        assert_eq!(
            [
                word(&e, at + 0x0C),
                word(&e, at + 0x10),
                word(&e, at + 0x14)
            ],
            [0, 0, 0]
        );
    }

    #[test]
    fn guide_vector_index_is_checked() {
        let mut e = engine();
        let invalid = spy(&mut e, INVALID_PARAMETER, |_, _| 0);
        let (at, data) = new_vector(&mut e, 0x38, 3, 3);
        assert_eq!(e.call(0x00b1_db00, &args![at, 2u32]).u32(), data + 0x70);
        assert!(invalid.borrow().is_empty());
        assert_eq!(e.call(0x00b1_db00, &args![at, 3u32]).u32(), data + 0xA8);
        assert_eq!(invalid.borrow().len(), 1);
    }

    #[test]
    fn erasing_one_guide_moves_the_rest_down() {
        let mut e = engine();
        let moved = spy(&mut e, GUIDE_MOVE_DOWN, |_, _| 0);
        let destroyed = spy(&mut e, GUIDE_DESTROY_RANGE, |_, _| 0);
        let (at, data) = new_vector(&mut e, 0x38, 3, 3);
        let out = e.mem.alloc(8);
        let result = e
            .call(0x00b1_dd80, &args![at, out, 0x1234u32, data + 0x38])
            .u32();
        assert_eq!(result, out);
        assert_eq!(
            *moved.borrow(),
            vec![vec![data + 0x70, data + 0xA8, data + 0x38]]
        );
        assert_eq!(
            *destroyed.borrow(),
            vec![vec![at, data + 0x70, data + 0xA8]]
        );
        assert_eq!(word(&e, at + 0x10), data + 0x70);
        assert_eq!((word(&e, out), word(&e, out + 4)), (PROXY, data + 0x38));
    }

    #[test]
    fn erasing_a_range_of_guides() {
        let mut e = engine();
        let differ = spy(&mut e, ITERATORS_DIFFER, |_, _| 1);
        // The move returns the end of what is kept: here the first position plus the tail.
        let moved = spy(&mut e, GUIDE_MOVE_DOWN, |_, a| a[2] + (a[1] - a[0]));
        let destroyed = spy(&mut e, GUIDE_DESTROY_RANGE, |_, _| 0);
        let (at, data) = new_vector(&mut e, 0x38, 4, 4);
        let out = e.mem.alloc(8);
        let result = e
            .call(
                0x00b1_ea20,
                &args![at, out, 1u32, data + 0x38, 2u32, data + 0x70],
            )
            .u32();
        assert_eq!(result, out);
        assert_eq!(differ.borrow().len(), 1);
        assert_eq!(
            *moved.borrow(),
            vec![vec![data + 0x70, data + 0xE0, data + 0x38]]
        );
        assert_eq!(
            *destroyed.borrow(),
            vec![vec![at, data + 0xA8, data + 0xE0]]
        );
        assert_eq!(word(&e, at + 0x10), data + 0xA8);
        assert_eq!((word(&e, out), word(&e, out + 4)), (PROXY, data + 0x38));
        // An empty range changes nothing.
        e.register(ITERATORS_DIFFER, |_, _| Ret::default());
        e.call(
            0x00b1_ea20,
            &args![at, out, 1u32, data + 0x38, 2u32, data + 0x38],
        );
        assert_eq!(moved.borrow().len(), 1);
        assert_eq!(word(&e, at + 0x10), data + 0xA8);
    }

    #[test]
    fn erasing_a_range_of_textures() {
        let mut e = engine();
        spy(&mut e, ITERATORS_DIFFER, |_, _| 1);
        let moved = spy(&mut e, TEXTURE_MOVE_DOWN, |_, a| a[2] + (a[1] - a[0]));
        let tidied = spy(&mut e, STRING_TIDY, |_, _| 0);
        let (at, data) = new_vector(&mut e, 0x2C, 3, 3);
        let out = e.mem.alloc(8);
        e.call(
            0x00b1_f800,
            &args![at, out, 1u32, data + 0x2C, 2u32, data + 0x58],
        );
        assert_eq!(
            *moved.borrow(),
            vec![vec![data + 0x58, data + 0x84, data + 0x2C]]
        );
        // The vacated tail's names are destroyed and the vector shortened.
        assert_eq!(*tidied.borrow(), vec![vec![data + 0x58, 1, 0]]);
        assert_eq!(word(&e, at + 0x10), data + 0x58);
        assert_eq!((word(&e, out), word(&e, out + 4)), (PROXY, data + 0x2C));
    }

    #[test]
    fn texture_vector_reservation() {
        let mut e = engine();
        let allocated = spy(&mut e, ALLOCATE_TEXTURES, |e, _| e.mem.alloc(0x2C * 3));
        let too_long = spy(&mut e, LENGTH_ERROR, |_, _| 0);
        let at = e.mem.alloc(0x18);
        for offset in [0x0Cu32, 0x10, 0x14] {
            e.mem.set_u32(at + offset, 0xCDCD_CDCD);
        }
        // Nothing to reserve: false, the pointers cleared.
        assert!(!e.call(0x00b1_f920, &args![at, 0u32]).bool());
        assert_eq!(word(&e, at + 0x0C), 0);
        assert!(allocated.borrow().is_empty());
        // Three textures.
        assert!(e.call(0x00b1_f920, &args![at, 3u32]).bool());
        assert_eq!(*allocated.borrow(), vec![vec![3, 0]]);
        let storage = word(&e, at + 0x0C);
        assert_ne!(storage, 0);
        assert_eq!(word(&e, at + 0x10), storage);
        assert_eq!(word(&e, at + 0x14), storage + 3 * 0x2C);
        // More than the maximum: the length error, no allocation, the pointers cleared.
        assert!(e.call(0x00b1_f920, &args![at, 0x05D1_745Eu32]).bool());
        assert_eq!(too_long.borrow().len(), 1);
        assert_eq!(allocated.borrow().len(), 1);
        assert_eq!(word(&e, at + 0x0C), 0);
        // The maximum itself is allowed.
        assert!(e.call(0x00b1_f920, &args![at, 0x05D1_745Du32]).bool());
        assert_eq!(too_long.borrow().len(), 1);
        assert_eq!(allocated.borrow().len(), 2);
    }

    #[test]
    fn iterator_difference_counts_guides() {
        let mut e = engine();
        let invalid = spy(&mut e, INVALID_PARAMETER, |_, _| 0);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        for (iterator, position) in [(first, 0x1000 + 0x38 * 5), (second, 0x1000)] {
            e.mem.set_u32(iterator, PROXY);
            e.mem.set_u32(iterator + 4, position);
        }
        assert_eq!(e.call(0x00b1_ff00, &args![first, second]).i32(), 5);
        assert_eq!(e.call(0x00b1_ff00, &args![second, first]).i32(), -5);
        assert!(invalid.borrow().is_empty());
        // Iterators of different vectors, and an iterator without a vector.
        e.mem.set_u32(second, 0x77);
        e.call(0x00b1_ff00, &args![first, second]);
        assert_eq!(invalid.borrow().len(), 1);
        e.mem.set_u32(first, 0);
        e.mem.set_u32(second, 0);
        e.call(0x00b1_ff00, &args![first, second]);
        assert_eq!(invalid.borrow().len(), 2);
    }

    #[test]
    fn guide_range_copy_returns_the_end() {
        let mut e = engine();
        let source = e.mem.alloc(0x18 * 2);
        let target = e.mem.alloc(0x18 * 2);
        for k in 0..12 {
            e.mem.set_u32(source + 4 * k, 0x100 + k);
        }
        let end = e
            .call(
                0x00b2_3e80,
                &args![source, source + 0x30, target, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(end, target + 0x30);
        for k in 0..12 {
            assert_eq!(word(&e, target + 4 * k), 0x100 + k);
        }
    }

    #[test]
    fn guide_iterator_constructor_checks_the_position() {
        let mut e = engine();
        let invalid = spy(&mut e, INVALID_PARAMETER, |_, _| 0);
        let (at, data) = new_vector(&mut e, 0x38, 2, 2);
        let iterator = e.mem.alloc(8);
        e.mem.set_u32(iterator, 0xCDCD_CDCD);
        let result = e.call(0x00b4_2980, &args![iterator, data + 0x38, at]).u32();
        assert_eq!(result, iterator);
        assert_eq!(
            (word(&e, iterator), word(&e, iterator + 4)),
            (PROXY, data + 0x38)
        );
        assert!(invalid.borrow().is_empty());
        // The end is a valid position; anything beyond it is not.
        e.call(0x00b4_2980, &args![iterator, data + 0x70, at]);
        assert!(invalid.borrow().is_empty());
        e.call(0x00b4_2980, &args![iterator, data + 0xA8, at]);
        assert_eq!(invalid.borrow().len(), 1);
    }

    #[test]
    fn texture_constructor_empties_the_name() {
        let mut e = engine();
        let tidied = spy(&mut e, STRING_TIDY, |_, _| 0);
        let texture = e.new_object::<SFrondTexture>();
        let result = e.call(0x00b1_d100, &args![texture]).u32();
        assert_eq!(result, texture.addr());
        assert_eq!(*tidied.borrow(), vec![vec![texture.addr(), 0, 0]]);
    }

    /// Fills the `count` 0x18-byte elements at `at` with a word pattern from `seed`.
    fn fill_elements(e: &mut Engine, at: u32, count: u32, seed: u32) {
        for k in 0..count * 6 {
            e.mem.set_u32(at + 4 * k, seed + k);
        }
    }

    #[test]
    fn vector_assignment_ignores_a_self_assignment_and_clears_for_an_empty_source() {
        let mut e = engine();
        let cleared = spy(&mut e, VECTOR_CLEAR, |_, _| 0);
        let (target, _) = new_vector(&mut e, 0x18, 2, 2);
        let (empty, _) = new_vector(&mut e, 0x18, 0, 0);
        assert_eq!(e.call(0x00b1_d130, &args![target, target]).u32(), target);
        assert!(cleared.borrow().is_empty());
        assert_eq!(e.call(0x00b1_d130, &args![target, empty]).u32(), target);
        assert_eq!(*cleared.borrow(), vec![vec![target]]);
    }

    #[test]
    fn vector_assignment_copies_a_shorter_source_over_the_target() {
        let mut e = engine();
        let destroyed = spy(&mut e, TRIVIAL_DESTRUCTOR, |_, _| 0);
        let (target, target_data) = new_vector(&mut e, 0x18, 3, 3);
        let (source, source_data) = new_vector(&mut e, 0x18, 2, 2);
        fill_elements(&mut e, source_data, 2, 0x200);
        let result = e.call(0x00b1_d130, &args![target, source]).u32();
        assert_eq!(result, target);
        for k in 0..12 {
            assert_eq!(word(&e, target_data + 4 * k), 0x200 + k);
        }
        // The surplus element is destroyed and the end moved.
        assert_eq!(*destroyed.borrow(), vec![vec![target_data + 0x30]]);
        assert_eq!(word(&e, target + 0x10), target_data + 0x30);
    }

    #[test]
    fn vector_assignment_within_the_capacity_constructs_the_rest() {
        let mut e = engine();
        let assigned = spy(&mut e, VECTOR_ASSIGN_RANGE, |_, _| 0);
        let constructed = spy(&mut e, VECTOR_CONSTRUCT_RANGE, |_, _| 0x7777);
        let (target, target_data) = new_vector(&mut e, 0x18, 1, 4);
        let (source, source_data) = new_vector(&mut e, 0x18, 3, 3);
        e.call(0x00b1_d130, &args![target, source]);
        // The first element is assigned over, the other two constructed at the target's end.
        assert_eq!(
            *assigned.borrow(),
            vec![vec![source_data, source_data + 0x18, target_data]]
        );
        assert_eq!(
            *constructed.borrow(),
            vec![vec![
                source_data + 0x18,
                source_data + 0x48,
                target_data + 0x18,
                target + 8,
                0,
                0
            ]]
        );
        assert_eq!(word(&e, target + 0x10), 0x7777);
    }

    #[test]
    fn vector_assignment_beyond_the_capacity_reallocates() {
        let mut e = engine();
        let destroyed = spy(&mut e, TRIVIAL_DESTRUCTOR, |_, _| 0);
        let deleted = spy(&mut e, OPERATOR_DELETE, |_, _| 0);
        let reserved = spy(&mut e, VECTOR_RESERVE, |e, a| {
            let block = e.mem.alloc(0x18 * 3);
            e.mem.set_u32(a[0] + 0x0C, block);
            1
        });
        let constructed = spy(&mut e, VECTOR_CONSTRUCT_RANGE, |_, _| 0x7777);
        let (target, target_data) = new_vector(&mut e, 0x18, 1, 1);
        let (source, source_data) = new_vector(&mut e, 0x18, 3, 3);
        e.call(0x00b1_d130, &args![target, source]);
        assert_eq!(*destroyed.borrow(), vec![vec![target_data]]);
        assert_eq!(*deleted.borrow(), vec![vec![target_data]]);
        assert_eq!(*reserved.borrow(), vec![vec![target, 3]]);
        let block = word(&e, target + 0x0C);
        assert_eq!(
            *constructed.borrow(),
            vec![vec![
                source_data,
                source_data + 0x48,
                block,
                target + 8,
                0,
                0
            ]]
        );
        assert_eq!(word(&e, target + 0x10), 0x7777);
        // A failed reservation leaves the vector as the reservation made it.
        let mut e = engine();
        spy(&mut e, OPERATOR_DELETE, |_, _| 0);
        spy(&mut e, VECTOR_RESERVE, |_, _| 0);
        let constructed = spy(&mut e, VECTOR_CONSTRUCT_RANGE, |_, _| 0x7777);
        let (target, _) = new_vector(&mut e, 0x18, 1, 1);
        let (source, _) = new_vector(&mut e, 0x18, 3, 3);
        e.call(0x00b1_d130, &args![target, source]);
        assert!(constructed.borrow().is_empty());
    }

    #[test]
    fn float_vector_assignment_follows_the_same_cases() {
        let mut e = engine();
        let cleared = spy(&mut e, FLOAT_VECTOR_CLEAR, |_, _| 0);
        let copied = spy(&mut e, FLOAT_COPY_RANGE, |_, _| 0);
        let constructed = spy(&mut e, FLOAT_CONSTRUCT_RANGE, |_, _| 0x7777);
        let reserved = spy(&mut e, FLOAT_RESERVE, |e, a| {
            let block = e.mem.alloc(16);
            e.mem.set_u32(a[0] + 0x0C, block);
            1
        });
        let deleted = spy(&mut e, OPERATOR_DELETE, |_, _| 0);
        let (target, target_data) = new_vector(&mut e, 4, 3, 3);
        let (source, source_data) = new_vector(&mut e, 4, 2, 2);
        let (empty, _) = new_vector(&mut e, 4, 0, 0);
        // Self-assignment and an empty source.
        e.call(0x00b1_d5d0, &args![target, target]);
        e.call(0x00b1_d5d0, &args![target, empty]);
        assert_eq!(*cleared.borrow(), vec![vec![target]]);
        // A shorter source is copied over the target and the end set after it.
        e.call(0x00b1_d5d0, &args![target, source]);
        assert_eq!(
            *copied.borrow(),
            vec![vec![source_data, source_data + 8, target_data]]
        );
        assert_eq!(word(&e, target + 0x10), target_data + 8);
        // A longer one within the capacity: copied over, the rest constructed.
        let (roomy, roomy_data) = new_vector(&mut e, 4, 1, 4);
        let (longer, longer_data) = new_vector(&mut e, 4, 3, 3);
        e.call(0x00b1_d5d0, &args![roomy, longer]);
        assert_eq!(
            copied.borrow()[1],
            vec![longer_data, longer_data + 4, roomy_data]
        );
        assert_eq!(
            *constructed.borrow(),
            vec![vec![
                longer_data + 4,
                longer_data + 12,
                roomy_data + 4,
                roomy + 8
            ]]
        );
        assert_eq!(word(&e, roomy + 0x10), 0x7777);
        // One beyond it: the storage is freed and the whole source constructed in new storage.
        let (small, small_data) = new_vector(&mut e, 4, 1, 1);
        e.call(0x00b1_d5d0, &args![small, longer]);
        assert_eq!(*deleted.borrow(), vec![vec![small_data]]);
        assert_eq!(*reserved.borrow(), vec![vec![small, 3]]);
        let block = word(&e, small + 0x0C);
        assert_eq!(
            constructed.borrow()[1],
            vec![longer_data, longer_data + 12, block, small + 8]
        );
    }

    /// The doubles of the guide vector's insertion: the allocation, the
    /// copies of ranges and the fill; each answers `result` where the exe uses its result.
    struct InsertDoubles {
        allocated: Log,
        relocated: Log,
        filled: Log,
        tail_copied: Log,
        destroyed: Log,
        deleted: Log,
        moved_up: Log,
        assigned: Log,
        copy_constructed: Log,
        vertex_tidied: Log,
        released: Log,
        length_errors: Log,
    }

    fn insert_doubles(e: &mut Engine, storage: u32) -> InsertDoubles {
        let relocated = spy(e, GUIDE_RELOCATE_COPY, |_, a| a[2] + (a[1] - a[0]));
        let tail_copied = spy(e, GUIDE_TAIL_COPY, |_, a| a[2] + (a[1] - a[0]));
        InsertDoubles {
            allocated: spy(e, GUIDE_ALLOCATE, move |_, _| storage),
            relocated,
            filled: spy(e, GUIDE_FILL, |_, _| 0),
            tail_copied,
            destroyed: spy(e, GUIDE_DESTROY_RANGE, |_, _| 0),
            deleted: spy(e, OPERATOR_DELETE, |_, _| 0),
            moved_up: spy(e, GUIDE_MOVE_UP, |_, _| 0),
            assigned: spy(e, GUIDE_ASSIGN, |_, _| 0),
            copy_constructed: spy(e, GUIDE_COPY_CONSTRUCT, |_, _| 0),
            vertex_tidied: spy(e, VERTEX_VECTOR_TIDY, |_, _| 0),
            released: spy(e, PROXY_RELEASE, |_, _| 0),
            length_errors: spy(e, LENGTH_ERROR, |_, _| 0),
        }
    }

    #[test]
    fn inserting_nothing_or_too_much_changes_nothing() {
        let mut e = engine();
        let doubles = insert_doubles(&mut e, 0);
        let (at, data) = new_vector(&mut e, 0x38, 2, 2);
        e.call(0x00b1_ecf0, &args![at, PROXY, data, 0u32, 0x4242u32]);
        assert!(doubles.length_errors.borrow().is_empty());
        e.call(
            0x00b1_ecf0,
            &args![at, PROXY, data, 0x0492_4925u32, 0x4242u32],
        );
        assert_eq!(doubles.length_errors.borrow().len(), 1);
        assert!(doubles.allocated.borrow().is_empty());
        assert_eq!(word(&e, at + 0x10), data + 0x70);
    }

    #[test]
    fn inserting_without_room_grows_the_vector() {
        let mut e = engine();
        let storage = e.mem.alloc(0x38 * 4);
        let doubles = insert_doubles(&mut e, storage);
        let (at, data) = new_vector(&mut e, 0x38, 2, 2);
        e.call(0x00b1_ecf0, &args![at, PROXY, data + 0x38, 1u32, 0x4242u32]);
        // One and a half times the capacity, at least the new size.
        assert_eq!(*doubles.allocated.borrow(), vec![vec![3, 0]]);
        // The elements before the position, the copy of the value, the elements after it.
        assert_eq!(
            *doubles.relocated.borrow(),
            vec![vec![data, data + 0x38, storage, at + 8]]
        );
        assert_eq!(
            *doubles.filled.borrow(),
            vec![vec![storage + 0x38, 1, 0x4242, at + 8, 0, 0]]
        );
        assert_eq!(
            *doubles.tail_copied.borrow(),
            vec![vec![data + 0x38, data + 0x70, storage + 0x70, at + 8]]
        );
        // The old elements are destroyed, its storage freed, the new one adopted.
        assert_eq!(
            *doubles.destroyed.borrow(),
            vec![vec![at, data, data + 0x70]]
        );
        assert_eq!(*doubles.deleted.borrow(), vec![vec![data]]);
        assert_eq!(word(&e, at + 0x0C), storage);
        assert_eq!(word(&e, at + 0x10), storage + 0xA8);
        assert_eq!(word(&e, at + 0x14), storage + 0xA8);
    }

    #[test]
    fn inserting_more_than_the_elements_after_the_position() {
        let mut e = engine();
        let doubles = insert_doubles(&mut e, 0);
        let (at, data) = new_vector(&mut e, 0x38, 3, 8);
        e.call(0x00b1_ecf0, &args![at, PROXY, data + 0x70, 3u32, 0x4242u32]);
        // A temporary copy of the value; the one element after the position moves up by three,
        // two more copies of the value are constructed past the old end, and the three slots at
        // the position are assigned.
        assert_eq!(doubles.copy_constructed.borrow().len(), 1);
        let temporary = doubles.copy_constructed.borrow()[0][0];
        assert_eq!(doubles.copy_constructed.borrow()[0][1], 0x4242);
        assert_eq!(
            *doubles.relocated.borrow(),
            vec![vec![data + 0x70, data + 0xA8, data + 0x70 + 0xA8, at + 8]]
        );
        assert_eq!(
            *doubles.filled.borrow(),
            vec![vec![data + 0xA8, 2, temporary, at + 8, 0, 0]]
        );
        assert_eq!(word(&e, at + 0x10), data + 0xA8 + 0xA8);
        assert_eq!(
            *doubles.assigned.borrow(),
            vec![vec![data + 0x70, temporary]]
        );
        assert_eq!(*doubles.vertex_tidied.borrow(), vec![vec![temporary]]);
        assert_eq!(*doubles.released.borrow(), vec![vec![temporary]]);
    }

    #[test]
    fn inserting_fewer_than_the_elements_after_the_position() {
        let mut e = engine();
        let doubles = insert_doubles(&mut e, 0);
        let (at, data) = new_vector(&mut e, 0x38, 4, 8);
        e.call(0x00b1_ecf0, &args![at, PROXY, data, 2u32, 0x4242u32]);
        let temporary = doubles.copy_constructed.borrow()[0][0];
        // The last two elements are copied past the end, the others move up, the two slots at
        // the position are assigned and the temporary is destroyed.
        assert_eq!(
            *doubles.tail_copied.borrow(),
            vec![vec![data + 0xE0 - 0x70, data + 0xE0, data + 0xE0, at + 8]]
        );
        assert_eq!(word(&e, at + 0x10), data + 0xE0 + 0x70);
        assert_eq!(
            *doubles.moved_up.borrow(),
            vec![vec![data, data + 0x70, data + 0xE0]]
        );
        assert_eq!(
            *doubles.assigned.borrow(),
            vec![vec![data, temporary], vec![data + 0x38, temporary]]
        );
        assert_eq!(*doubles.vertex_tidied.borrow(), vec![vec![temporary]]);
        assert_eq!(doubles.deleted.borrow().len(), 1);
    }
}
