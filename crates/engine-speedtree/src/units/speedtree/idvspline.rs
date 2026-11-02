//! `SpeedTree/idvspline.cpp` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `CIdvBezierSpline` is a tree-editor curve: control points with tangents
//! (parsed from a text description), the Bezier segments between them and a
//! table of 500 evenly spaced samples that `Evaluate` interpolates in. The
//! splines are cached by name in a global `std::map`. The unit also holds
//! `CVec`, a small float vector of up to five components, and the
//! `std::vector<CVec>`, `std::vector<float>` and `std::map<string,
//! CIdvBezierSpline*>` template code the spline uses.
//!
//! The PC `std::vector` carries an iterator-debugging proxy word in front of
//! its pointers: the proxy at +0, the first, last and end pointers at +0xc,
//! +0x10 and +0x14. A vector is 0x18 bytes, so the PC spline is 0x84 bytes
//! (the Xbox PDB has 0x5c). The checked vector accesses call the CRT's
//! invalid-parameter handler (`00ec7c56`) when an index is out of range; that
//! handler does not return in the exe, and the code after it is the normal
//! path.

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new(size)` (`cdecl`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(pointer)` (`cdecl`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The CRT's invalid-parameter handler (`cdecl`, no arguments).
const INVALID_PARAMETER: u32 = 0x00ec_7c56;
/// `_ftol2_sse` (`00ec62c0`): truncates the float in ST0 to an integer in
/// EAX. The uniform form takes it as an `f64` argument.
const FTOL: u32 = 0x00ec_62c0;
/// `_atexit(function)` (`cdecl`).
const ATEXIT: u32 = 0x00ec_658f;
/// `isspace(int)` (`cdecl`).
const ISSPACE: u32 = 0x00ec_8965;
/// `sscanf(buffer, format, ...)` (`cdecl`).
const SSCANF: u32 = 0x00ec_a4a6;
/// `strlen(string)` (`cdecl`).
const STRLEN: u32 = 0x00ec_6130;
/// `strcmp(a, b)` (`cdecl`).
const STRCMP: u32 = 0x00ec_6da0;
/// `atof(string)` (`cdecl`; `double` in ST0).
const ATOF: u32 = 0x00ec_a573;
/// `atol(string)` (`cdecl`).
const ATOL: u32 = 0x00ec_a6d3;
/// `std::basic_string::assign(const string&, pos, count)` (`thiscall`).
const STRING_ASSIGN: u32 = 0x0044_a580;
/// `std::basic_string::_Tidy(built, new_size)` (`thiscall`).
const STRING_TIDY: u32 = 0x0044_a730;
/// `std::basic_string<char>::basic_string(const char*)` (`thiscall`).
const STRING_FROM_C_STRING: u32 = 0x0044_a4c0;
/// `std::basic_string::compare(const string&)` (`thiscall`, `int` result).
const STRING_COMPARE: u32 = 0x00ac_9da0;
/// `std::runtime_error::runtime_error(const string&)` (`thiscall`).
const RUNTIME_ERROR_FROM_STRING: u32 = 0x0044_a310;
/// `_CxxThrowException(object, throw_info)`.
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// `std::exception::exception(const char* const&)` (`thiscall`), the base
/// of `std::bad_alloc`.
const EXCEPTION_FROM_MESSAGE: u32 = 0x00ec_78f1;
/// `std::string::npos`.
const STRING_NPOS: u32 = 0x0101_73f0;
/// `"%s"`.
const FORMAT_STRING: u32 = 0x0101_9f08;
/// `"BezierSpline"`.
const BEZIER_SPLINE_TAG: u32 = 0x010a_57e8;
/// `499.0` (`double`): the index scale of the 500-sample table.
const SAMPLE_SCALE: u32 = 0x010a_57e0;
/// `1/499 = 0.002004008f`: the spacing of the samples.
const SAMPLE_SPACING: u32 = 0x010a_57dc;
/// `0.5` (`double`).
const HALF: u32 = 0x0101_1588;

/// `CIdvBezierSpline::CIdvBezierSpline(CIdvBezierSpline const&)` (`00b377c0`).
const SPLINE_COPY_CONSTRUCTOR: u32 = 0x00b3_77c0;
/// `CIdvBezierSpline::~CIdvBezierSpline` (`00b37880`).
const SPLINE_DESTRUCTOR: u32 = 0x00b3_7880;
/// The spline's field-wise assignment (`thiscall`: `this`, the source;
/// unnamed in the engine map).
const SPLINE_ASSIGN: u32 = 0x00b1_c7a0;
/// `CIdvBezierSpline::Parse(const string&)` (`00b37cc0`).
const SPLINE_PARSE: u32 = 0x00b3_7cc0;
/// The sampler that builds the 500-entry table (`00b38430`).
const SPLINE_BUILD_SAMPLES: u32 = 0x00b3_8430;
/// The point at `t` on the Bezier chain (`00b38b50`).
const SPLINE_POINT_AT: u32 = 0x00b3_8b50;
/// `CIdvBezierSpline::AddPoint` (`00b380e0`).
const SPLINE_ADD_POINT: u32 = 0x00b3_80e0;
/// The cache of splines by name (`std::map<std::string, CIdvBezierSpline*>`,
/// a global; unnamed in the engine map).
const SPLINE_CACHE: u32 = 0x011f_8e2c;
/// The map's `_Myhead` pointer slot (the cache object plus 0x18).
const SPLINE_CACHE_HEAD: u32 = 0x011f_8e44;
/// `std::map` `operator[]` of the cache (`00b39210`).
const MAP_INDEX: u32 = 0x00b3_9210;
/// The map's `lower_bound` (`00b3afe0`).
const MAP_LOWER_BOUND: u32 = 0x00b3_afe0;
/// The map's insert (`00b398c0`).
const MAP_INSERT: u32 = 0x00b3_98c0;
/// The tree's subtree erase (`00b3a050`).
const MAP_ERASE_SUBTREE: u32 = 0x00b3_a050;
/// The map iterator's dereference (`00b3b090`): the address of the node's
/// pair.
const MAP_ITERATOR_VALUE: u32 = 0x00b3_b090;
/// The map iterator's increment (`00b3b100`).
const MAP_ITERATOR_INCREMENT: u32 = 0x00b3_b100;
/// The map's `begin()` (`00b393b0`).
const MAP_BEGIN: u32 = 0x00b3_93b0;
/// The map's `clear()` (`00b39400`).
const MAP_CLEAR: u32 = 0x00b3_9400;
/// The iterator `operator!=` (`thiscall`, `AL` result; `00b16830`).
const ITERATOR_NOT_EQUAL: u32 = 0x00b1_6830;
/// The vector iterator's `operator+=` (`thiscall`: the iterator, the count).
const ITERATOR_ADVANCE: u32 = 0x00b1_6890;
/// `std::vector<CVec>::vector()` (`thiscall`, `00b31270`).
const VECTOR_CVEC_CONSTRUCT: u32 = 0x00b3_1270;
/// `std::vector<float>::vector()` (`thiscall`, `00449fd0`).
const VECTOR_FLOAT_CONSTRUCT: u32 = 0x0044_9fd0;
/// `std::vector<float>::_Tidy()` (`thiscall`, `0065f240`).
const VECTOR_FLOAT_TIDY: u32 = 0x0065_f240;
/// `std::vector<float>::push_back(const float&)` (`thiscall`, `006578e0`).
const VECTOR_FLOAT_PUSH_BACK: u32 = 0x0065_78e0;
/// `std::vector<CVec>::clear()` (`thiscall`, `00b1e750`).
const VECTOR_CVEC_CLEAR: u32 = 0x00b1_e750;
/// `std::vector<CVec>::_Tidy()` (`thiscall`, `00b397f0`).
const VECTOR_CVEC_TIDY: u32 = 0x00b3_97f0;
/// `std::vector<CVec>::resize(n, value)` (`thiscall`, `00b39490`).
const VECTOR_CVEC_RESIZE: u32 = 0x00b3_9490;
/// `std::vector<CVec>::insert(position, count, value)` (`00b3a0d0`).
const VECTOR_CVEC_INSERT_N: u32 = 0x00b3_a0d0;
/// The vector's insert of one element (`thiscall`, `00b396c0`).
const VECTOR_CVEC_INSERT_ONE: u32 = 0x00b3_96c0;
/// The vector's fill-construct of `count` copies (`cdecl`, `00b3b5f0`).
const VECTOR_CVEC_FILL: u32 = 0x00b3_b5f0;
/// The vector's `erase(first, last)` of `CVec`s (`thiscall`, `00b1ffd0`).
const VECTOR_CVEC_ERASE: u32 = 0x00b1_ffd0;
/// The vector's capacity growth helper (`thiscall`, `00b0fb80`): the
/// `length_error` throw for a too long vector.
const VECTOR_LENGTH_ERROR: u32 = 0x00b0_fb80;
/// `CVec::CVec()` (`thiscall`, `00b3c470`): zeros, size 3.
const CVEC_DEFAULT: u32 = 0x00b3_c470;
/// `CVec::CVec(int size)` (`thiscall`, `00b3c4b0`): zeros, the size clamped
/// to 5.
const CVEC_SIZED: u32 = 0x00b3_c4b0;
/// `CVec::CVec(float, float)` (`thiscall`, `00b3c500`).
const CVEC_FROM_TWO: u32 = 0x00b3_c500;
/// `CVec::CVec(float x5)` (`thiscall`, `00b3c550`).
const CVEC_FROM_FIVE: u32 = 0x00b3_c550;
/// `CVec::Length` (`thiscall`, `00b3c5a0`; `float` in ST0).
const CVEC_LENGTH: u32 = 0x00b3_c5a0;
/// `CVec::Normalize` (`thiscall`, `00b3c600`).
const CVEC_NORMALIZE: u32 = 0x00b3_c600;
/// `CVec::operator+` (`thiscall`, `00b3c660`).
const CVEC_ADD: u32 = 0x00b3_c660;
/// `CVec::operator-` (`thiscall`, `00b3c750`).
const CVEC_SUBTRACT: u32 = 0x00b3_c750;
/// `CVec::operator*(float)` (`thiscall`, `00b3c840`).
const CVEC_SCALE: u32 = 0x00b3_c840;
/// `CVec`'s destructor (`thiscall`, `00aa7830`): does nothing; the linker
/// shares it with other empty destructors.
const TRIVIAL_DESTRUCTOR: u32 = 0x00aa_7830;
/// `CIdvRandom::CIdvRandom` (`thiscall`, `00b3f100`).
const RANDOM_CONSTRUCTOR: u32 = 0x00b3_f100;
/// `CIdvRandom::GetUniform` (`thiscall`: `this`, from, to; `float` in ST0).
const RANDOM_GET_UNIFORM: u32 = 0x00b3_f130;

/// The static `CIdvRandom` of `Evaluate`, its init flag and its atexit
/// destructor.
const EVALUATE_RANDOM: (u32, u32, u32) = (0x011f_8e10, 0x011f_8e14, 0x00fd_c700);
/// The static `CIdvRandom` of `ScaledVariance`, its init flag and its atexit
/// destructor.
const VARIANCE_RANDOM: (u32, u32, u32) = (0x011f_8e18, 0x011f_8e1c, 0x00fd_c710);

layout! {
    /// A PC `std::vector` as the code reads it: the iterator-debugging proxy
    /// word, then the first, last and end pointers.
    pub struct StdVector: 0x18 {
        /// The container proxy (iterator checking).
        0x00 proxy: u32,
        /// `_Myfirst`.
        0x0C first: u32,
        /// `_Mylast`.
        0x10 last: u32,
        /// `_Myend`.
        0x14 end: u32,
    }

    /// `CVec` (Xbox PDB), 0x18 bytes: up to five floats and their count.
    pub struct CVec: 0x18 {
        /// `m_afData[0]` (Xbox PDB).
        0x00 m_afData0: f32,
        /// `m_afData[1]` (Xbox PDB).
        0x04 m_afData1: f32,
        /// `m_afData[2]` (Xbox PDB).
        0x08 m_afData2: f32,
        /// `m_afData[3]` (Xbox PDB).
        0x0C m_afData3: f32,
        /// `m_afData[4]` (Xbox PDB).
        0x10 m_afData4: f32,
        /// `m_nSize` (Xbox PDB).
        0x14 m_nSize: i32,
    }

    /// `CIdvBezierSpline` (Xbox PDB) on PC, 0x84 bytes; each of the five
    /// vectors is 0x18 bytes (0x10 on Xbox).
    pub struct CIdvBezierSpline: 0x84 {
        /// `m_fMin` (Xbox PDB).
        0x00 m_fMin: f32,
        /// `m_fMax` (Xbox PDB).
        0x04 m_fMax: f32,
        /// `m_fVariance` (Xbox PDB).
        0x08 m_fVariance: f32,
        /// `m_vControlPoints` (Xbox PDB): `vector<CVec>`.
        0x0C m_vControlPoints: Inline<StdVector>,
        /// `m_vControlPointTangents` (Xbox PDB): `vector<CVec>`.
        0x24 m_vControlPointTangents: Inline<StdVector>,
        /// `m_vControlPointTangentLengths` (Xbox PDB): `vector<float>`.
        0x3C m_vControlPointTangentLengths: Inline<StdVector>,
        /// `m_vEvenlySpacedPoints` (Xbox PDB): `vector<CVec>`.
        0x54 m_vEvenlySpacedPoints: Inline<StdVector>,
        /// `m_vSplinePoints` (Xbox PDB): `vector<CVec>`.
        0x6C m_vSplinePoints: Inline<StdVector>,
    }
}

/// The number of elements of the vector at `vector`: the byte difference of
/// the last and first pointers divided (signed) by `stride`.
fn element_count(e: &Engine, vector: u32, stride: i32) -> i32 {
    let first = e.mem.u32(vector.wrapping_add(0x0c));
    let last = e.mem.u32(vector.wrapping_add(0x10));
    (last.wrapping_sub(first) as i32) / stride
}

/// The checked vector access: the invalid-parameter handler runs unless
/// `index` is below `count` (an unsigned compare).
fn check_index(e: &mut Engine, index: u32, count: i32) {
    if index >= count as u32 {
        e.call(INVALID_PARAMETER, &args![]);
    }
}

/// The address of element `index` of the `CVec` vector at `vector`.
fn cvec_element(e: &Engine, vector: u32, index: u32) -> u32 {
    index
        .wrapping_mul(0x18)
        .wrapping_add(e.mem.u32(vector.wrapping_add(0x0c)))
}

/// The debug iterator check the vector code makes: the invalid-parameter
/// handler runs when the vector is null or `position` is outside
/// `[first, last]`.
fn check_position(e: &mut Engine, vector: u32, position: u32) {
    let first = e.mem.u32(vector.wrapping_add(0x0c));
    let last = e.mem.u32(vector.wrapping_add(0x10));
    if vector == 0 || first > position || position > last {
        e.call(INVALID_PARAMETER, &args![]);
    }
}

/// Copies the six words of a `CVec` from `from` to `to`.
fn copy_cvec(e: &mut Engine, from: u32, to: u32) {
    for i in 0..6 {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

/// The data of a `std::string` (`c_str()`): the inline buffer, or the heap
/// pointer once the reserve reaches 0x10.
fn string_data(e: &Engine, string: u32) -> u32 {
    if e.mem.u32(string.wrapping_add(0x18)) < 0x10 {
        string.wrapping_add(4)
    } else {
        e.mem.u32(string.wrapping_add(4))
    }
}

/// `x87 FLD f32 / FSUB / FMUL / FADD / FSTP f32`: `(b - a) * t + a` computed
/// in double precision from the quieted `a` and rounded to `f32`.
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    let a = x87_quiet(a);
    ((b as f64 - a as f64) * t as f64 + a as f64) as f32
}

/// The first use of a function-local static `CIdvRandom`: sets the init flag,
/// constructs the object and registers its destructor with `atexit`.
fn init_static_random(e: &mut Engine, (object, flag, destructor): (u32, u32, u32)) {
    if e.global::<u32>(flag) & 1 == 0 {
        let flags = e.global::<u32>(flag);
        e.set_global(flag, flags | 1);
        e.call(RANDOM_CONSTRUCTOR, &args![object]);
        e.call(ATEXIT, &args![destructor]);
    }
}

/// `operator new`-style allocation of the container proxy
/// (`thiscall`-free helper at `0044d670`: count, hint).
const PROXY_ALLOCATE: u32 = 0x0044_d670;
/// The vector allocator's `allocate(count, hint)` (`cdecl`, `00b23aa0`).
const VECTOR_ALLOCATE: u32 = 0x00b2_3aa0;
/// `std::_Uninit_copy(first, last, dest, allocator, tag, tag)` (`cdecl`,
/// `00b25a90`; unnamed in the engine map).
const UNINIT_COPY: u32 = 0x00b2_5a90;
/// The `CVec` range copy-construct, `first, last, dest, allocator` (`cdecl`,
/// `00b3b5a0`).
const CVEC_RANGE_COPY: u32 = 0x00b3_b5a0;
/// The second instance of the `CVec` range copy-construct (`cdecl`,
/// `00b3b6f0`).
const CVEC_RANGE_COPY_ALIAS: u32 = 0x00b3_b6f0;
/// `std::copy_backward(first, last, dest_last)` (`cdecl`, `00b16a10`).
const COPY_BACKWARD: u32 = 0x00b1_6a10;
/// The map iterator constructor from a node and a container (`thiscall`,
/// `00b3b050`).
const MAP_ITERATOR_NEW: u32 = 0x00b3_b050;
/// The map iterator's decrement (`thiscall`, `00b3b440`).
const MAP_ITERATOR_DECREMENT: u32 = 0x00b3_b440;
/// The iterator `operator==` (`thiscall`, `AL` result; `00b2ac30`).
const ITERATOR_EQUAL: u32 = 0x00b2_ac30;
/// The tree's left rotation (`thiscall`, `00b3b1d0`).
const TREE_ROTATE_LEFT: u32 = 0x00b3_b1d0;
/// The tree's right rotation (`thiscall`, `00b3b280`).
const TREE_ROTATE_RIGHT: u32 = 0x00b3_b280;
/// The tree's `_Buynode` (`thiscall`, `00b3b330`).
const TREE_BUY_NODE: u32 = 0x00b3_b330;
/// The tree's node allocator (`cdecl`: count, hint; `00b3b540`).
const TREE_ALLOCATE_NODE: u32 = 0x00b3_b540;
/// The tree's `_Insert` of a new node (`thiscall`, `00b3ac00`).
const TREE_INSERT_NODE: u32 = 0x00b3_ac00;
/// The tree's unique insert (`thiscall`, `00b3a9b0`).
const TREE_INSERT_UNIQUE: u32 = 0x00b3_a9b0;
/// The tree's erase of a range of iterators (`thiscall`, `00b3bb90`).
const TREE_ERASE_RANGE: u32 = 0x00b3_bb90;
/// The tree's erase of one iterator (`thiscall`, `00b3bd80`).
const TREE_ERASE_ONE: u32 = 0x00b3_bd80;
/// The tree's `_Init` (`thiscall`, `00b3b810`).
const TREE_INIT: u32 = 0x00b3_b810;
/// The tree's head node builder (`thiscall`, `00b3b880`).
const TREE_BUY_HEAD: u32 = 0x00b3_b880;
/// The head node's allocator (`thiscall`, `00b3b9d0`).
const TREE_ALLOCATE_HEAD: u32 = 0x00b3_b9d0;
/// The tree's container proxy builder (`thiscall`, `00b3b9f0`).
const TREE_PROXY_NEW: u32 = 0x00b3_b9f0;
/// The `std::exception` base constructor (`thiscall`, `00ec78f1`) for the
/// `bad_alloc` throw.
const BAD_ALLOC_CONSTRUCTOR: u32 = EXCEPTION_FROM_MESSAGE;
/// The `std::bad_alloc` vtable.
const BAD_ALLOC_VTABLE: u32 = 0x0101_76f8;
/// The `std::bad_alloc` throw info.
const BAD_ALLOC_THROW_INFO: u32 = 0x0114_0ef8;
/// `"invalid map/set<T> iterator"`, the text of the erase-at-end throw.
const MAP_ITERATOR_TEXT: u32 = 0x010a_5804;
/// The vtable of the exception thrown by that erase.
const MAP_ITERATOR_VTABLE: u32 = 0x010a_57fc;
/// The throw info of that exception.
const MAP_ITERATOR_THROW_INFO: u32 = 0x0117_327c;
/// `"map/set<T> too long"`.
const MAP_TOO_LONG_TEXT: u32 = 0x010a_31a0;
/// The vtable of the `length_error` thrown for it.
const MAP_TOO_LONG_VTABLE: u32 = 0x0101_73dc;
/// The throw info of that exception.
const MAP_TOO_LONG_THROW_INFO: u32 = 0x0114_0cb0;

/// The largest element count of a `vector<CVec>` (`0xaaaaaaa`, the maximum
/// the allocator reports for 0x18 byte elements).
const CVEC_MAX_COUNT: u32 = 0x0aaa_aaaa;
/// The largest node count of the map (`0x7ffffff`).
const MAP_MAX_COUNT: u32 = 0x07ff_ffff;
/// The scalar-deleting-destructor flag the exe folds to 0 where an inlined
/// destructor call could free its object.
const DESTRUCTOR_FREES: bool = false;
/// The red colour of a tree node.
const NODE_RED: u8 = 0;
/// The black colour of a tree node.
const NODE_BLACK: u8 = 1;
/// `std::vector<CVec>::push_back(const CVec&)` (`thiscall`, `00b39100`).
const VECTOR_CVEC_PUSH_BACK: u32 = 0x00b3_9100;

// Translated from 00b37660 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b37660 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `CIdvBezierSpline::CIdvBezierSpline(const std::string& name)` (Xbox PDB):
/// builds the five empty vectors, then looks `name` up in the cache of
/// splines. A miss parses the text description, builds the 500 samples,
/// stores a copy of the spline in the cache (a null pointer when the
/// allocation fails); a hit assigns the cached spline. Returns `this`.
pub fn cidv_bezier_spline_constructor(
    e: &mut Engine,
    this: Ptr<CIdvBezierSpline>,
    name: Ptr,
) -> Ptr<CIdvBezierSpline> {
    e.call(
        VECTOR_CVEC_CONSTRUCT,
        &args![this.at(CIdvBezierSpline::m_vControlPoints)],
    );
    e.call(
        VECTOR_CVEC_CONSTRUCT,
        &args![this.at(CIdvBezierSpline::m_vControlPointTangents)],
    );
    e.call(
        VECTOR_FLOAT_CONSTRUCT,
        &args![this.at(CIdvBezierSpline::m_vControlPointTangentLengths)],
    );
    e.call(
        VECTOR_CVEC_CONSTRUCT,
        &args![this.at(CIdvBezierSpline::m_vEvenlySpacedPoints)],
    );
    e.call(
        VECTOR_CVEC_CONSTRUCT,
        &args![this.at(CIdvBezierSpline::m_vSplinePoints)],
    );
    let slot = e.call(MAP_INDEX, &args![SPLINE_CACHE, name]).u32();
    let cached = e.mem.u32(slot);
    if cached == 0 {
        e.call(SPLINE_PARSE, &args![this, name]);
        e.call(SPLINE_BUILD_SAMPLES, &args![this, 500u32]);
        let object = e.call(OPERATOR_NEW, &args![0x84u32]).u32();
        let copy = if object == 0 {
            0
        } else {
            e.call(SPLINE_COPY_CONSTRUCTOR, &args![object, this]).u32()
        };
        let slot = e.call(MAP_INDEX, &args![SPLINE_CACHE, name]).u32();
        e.mem.set_u32(slot, copy);
    } else {
        e.call(SPLINE_ASSIGN, &args![this, cached]);
    }
    this
}

// Translated from 00b377c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b377c0 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `CIdvBezierSpline::CIdvBezierSpline(const CIdvBezierSpline&)` (identified
/// from its use as the copy for the cache): the five empty vectors, then the
/// field-wise assignment from `other`. Returns `this`.
pub fn fn_00b377c0(
    e: &mut Engine,
    this: Ptr<CIdvBezierSpline>,
    other: Ptr<CIdvBezierSpline>,
) -> Ptr<CIdvBezierSpline> {
    e.call(
        VECTOR_CVEC_CONSTRUCT,
        &args![this.at(CIdvBezierSpline::m_vControlPoints)],
    );
    e.call(
        VECTOR_CVEC_CONSTRUCT,
        &args![this.at(CIdvBezierSpline::m_vControlPointTangents)],
    );
    e.call(
        VECTOR_FLOAT_CONSTRUCT,
        &args![this.at(CIdvBezierSpline::m_vControlPointTangentLengths)],
    );
    e.call(
        VECTOR_CVEC_CONSTRUCT,
        &args![this.at(CIdvBezierSpline::m_vEvenlySpacedPoints)],
    );
    e.call(
        VECTOR_CVEC_CONSTRUCT,
        &args![this.at(CIdvBezierSpline::m_vSplinePoints)],
    );
    e.call(SPLINE_ASSIGN, &args![this, other]);
    this
}

// Translated from 00b37880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b37880 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `CIdvBezierSpline::~CIdvBezierSpline` (Xbox PDB): destroys the five
/// vectors in reverse order of construction (spline points, evenly spaced
/// points, tangent lengths, tangents, control points); each is tidied and
/// its container proxy freed.
pub fn cidv_bezier_spline_destructor(e: &mut Engine, this: Ptr<CIdvBezierSpline>) {
    for field in [
        CIdvBezierSpline::m_vSplinePoints,
        CIdvBezierSpline::m_vEvenlySpacedPoints,
    ] {
        let vector = this.at(field);
        e.call(VECTOR_CVEC_TIDY, &args![vector]);
        let proxy = e.mem.u32(vector.addr());
        e.call(OPERATOR_DELETE, &args![proxy]);
    }
    let lengths = this.at(CIdvBezierSpline::m_vControlPointTangentLengths);
    e.call(VECTOR_FLOAT_TIDY, &args![lengths]);
    let proxy = e.mem.u32(lengths.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
    for field in [
        CIdvBezierSpline::m_vControlPointTangents,
        CIdvBezierSpline::m_vControlPoints,
    ] {
        let vector = this.at(field);
        e.call(VECTOR_CVEC_TIDY, &args![vector]);
        let proxy = e.mem.u32(vector.addr());
        e.call(OPERATOR_DELETE, &args![proxy]);
    }
}

// Translated from 00b37a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the spline cache (identified from the code; the engine map has
/// no name): every cached spline is destroyed and freed, its slot cleared,
/// then the map is cleared.
pub fn fn_00b37a00(e: &mut Engine) {
    // The exe's frame (`SUB ESP,0x3c` under `EBP`): 0x3c bytes, the iterator at +0x34
    // (`EBP-8`) and the end iterator at +0x2c (`EBP-0x10`).
    e.with_stack(0x3c, |e, frame| {
        let frame = frame.addr();
        let iterator = frame + 0x34;
        e.call(MAP_BEGIN, &args![SPLINE_CACHE, iterator]);
        let end = frame + 0x2c;
        loop {
            // The end iterator: the cache's proxy and its head node. (The null
            // check of the cache's address is always satisfied.)
            let head = e.mem.u32(SPLINE_CACHE_HEAD);
            let proxy = e.mem.u32(SPLINE_CACHE);
            e.mem.set_u32(end, proxy);
            e.mem.set_u32(end + 4, head);
            if !e.call(ITERATOR_NOT_EQUAL, &args![iterator, end]).bool() {
                break;
            }
            let pair = e.call(MAP_ITERATOR_VALUE, &args![iterator]).u32();
            let spline = e.mem.u32(pair.wrapping_add(0x1c));
            if spline != 0 {
                e.call(SPLINE_DESTRUCTOR, &args![spline]);
                // The scalar-deleting-destructor flag is the constant 1 here, so
                // the delete always runs.
                e.call(OPERATOR_DELETE, &args![spline]);
            }
            let pair = e.call(MAP_ITERATOR_VALUE, &args![iterator]).u32();
            e.mem.set_u32(pair.wrapping_add(0x1c), 0);
            e.call(MAP_ITERATOR_INCREMENT, &args![iterator]);
        }
        e.call(MAP_CLEAR, &args![SPLINE_CACHE]);
    });
}

// Translated from 00b37ad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b37ad0 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `CIdvBezierSpline::Evaluate(float t)` (Xbox PDB): the curve's value at
/// `t` in `0..=1`, scaled into `[min, max]`, plus a random amount within
/// the spline's variance. With a table of exactly 500 samples the value is
/// the sample at `trunc(t * 499)` (the last sample for 499), interpolated
/// linearly with the next one by the rest of `t * 499` expressed in sample
/// spacings. With any other table size the result is 0. The random draw uses
/// a function-local `CIdvRandom`.
pub fn cidv_bezier_spline_evaluate(e: &mut Engine, this: Ptr<CIdvBezierSpline>, t: f32) -> f32 {
    let mut value = 0.0f32;
    let samples = this.addr() + 0x54;
    if element_count(e, samples, 0x18) == 500 {
        let scale = e.global::<f64>(SAMPLE_SCALE);
        let index = e.call(FTOL, &args![t as f64 * scale]).u32();
        if index == 499 {
            check_index(e, index, element_count(e, samples, 0x18));
            let point = cvec_element(e, samples, index);
            value = x87_quiet(f32::from_bits(e.mem.u32(point + 4)));
        } else {
            let spacing = e.global::<f32>(SAMPLE_SPACING) as f64;
            let fraction = ((t as f64 - (index as i32 as f64) * spacing) / spacing) as f32;
            let next = index.wrapping_add(1);
            check_index(e, next, element_count(e, samples, 0x18));
            let upper = cvec_element(e, samples, next);
            check_index(e, index, element_count(e, samples, 0x18));
            let lower = cvec_element(e, samples, index);
            let low = x87_quiet(f32::from_bits(e.mem.u32(lower + 4)));
            let high = f32::from_bits(e.mem.u32(upper + 4));
            value = ((high as f64 - low as f64) * fraction as f64 + low as f64) as f32;
        }
        let min = e.get(this, CIdvBezierSpline::m_fMin) as f64;
        let max = e.get(this, CIdvBezierSpline::m_fMax) as f64;
        value = ((max - min) * value as f64 + min) as f32;
        init_static_random(e, EVALUATE_RANDOM);
        let variance = x87_quiet(e.get(this, CIdvBezierSpline::m_fVariance));
        let drawn = e
            .call(
                RANDOM_GET_UNIFORM,
                &args![EVALUATE_RANDOM.0, -variance, variance],
            )
            .f64();
        value = (drawn + value as f64) as f32;
    }
    x87_quiet(value)
}

// Translated from 00b37cc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b37cc0 - /GS cookie check (00ec408c) is not translated.
/// `CIdvBezierSpline::Parse(const std::string& text)` (Xbox PDB): reads the
/// text description. The first token must be `BezierSpline`, followed by the
/// minimum, maximum and variance; a `{` then starts a point count and, for
/// each point, five numbers: the position (two floats), the tangent (two
/// floats) and the tangent length, which go to `AddPoint`. Anything else
/// leaves the spline as it was.
pub fn cidv_bezier_spline_parse(e: &mut Engine, this: Ptr<CIdvBezierSpline>, text: Ptr) {
    // The exe's frame (`SUB ESP,0x12c` under `EBP`): 0x12c bytes, the token buffer at +0x24
    // (`EBP-0x108`) and the numbers at +8 (point `EBP-0x124`, tangent `EBP-0x118`).
    e.with_stack(0x12c, |e, frame| {
        let frame = frame.addr();
        let token = frame + 0x24;
        let numbers = frame + 8;
        let mut position = string_data(e, text.addr());
        position = next_token(e, position, token);
        if e.call(STRCMP, &args![token, BEZIER_SPLINE_TAG]).i32() == 0 {
            for field in [
                CIdvBezierSpline::m_fMin,
                CIdvBezierSpline::m_fMax,
                CIdvBezierSpline::m_fVariance,
            ] {
                position = next_token(e, position, token);
                let value = e.call(ATOF, &args![token]).f64() as f32;
                e.set(this, field, value);
            }
            position = next_token(e, position, token);
            if e.mem.u8(token) as i8 as i32 == 0x7b {
                position = next_token(e, position, token);
                let count = e.call(ATOL, &args![token]).i32();
                let mut i = 0i32;
                while i < count {
                    // Five numbers: the point (two floats), the tangent (two
                    // floats) and the tangent length. The buffer holds them as
                    // the exe does: point at +0 and +4, length at +8, tangent
                    // at +0xc and +0x10.
                    for offset in [0u32, 4, 0xc, 0x10, 8] {
                        position = next_token(e, position, token);
                        let value = e.call(ATOF, &args![token]).f64() as f32;
                        e.mem.set_f32(numbers + offset, value);
                    }
                    let length = x87_quiet_bits(e.mem.u32(numbers + 8));
                    e.call(
                        SPLINE_ADD_POINT,
                        &args![this, numbers, numbers + 0xc, length],
                    );
                    i = i.wrapping_add(1);
                }
            }
        }
    });
}

// Translated from 00b37f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NextToken(text, token)` (identified from the code; the engine map has
/// no name; `cdecl`): skips white space at `text`, reads the next
/// whitespace-delimited word into `token` with `sscanf("%s")` and returns the
/// position just after it.
pub fn next_token(e: &mut Engine, text: u32, token: u32) -> u32 {
    let mut text = text;
    loop {
        let character = e.mem.u8(text) as i8 as i32;
        if e.call(ISSPACE, &args![character]).i32() == 0 {
            break;
        }
        text = text.wrapping_add(1);
    }
    e.call(SSCANF, &args![text, FORMAT_STRING, token]);
    let length = e.call(STRLEN, &args![token]).u32();
    length.wrapping_add(text)
}

// Translated from 00b37fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b37fa0 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `CIdvBezierSpline::ScaledVariance(float t)` (Xbox PDB): with exactly 500
/// samples, a random amount within `+-variance * sample(round(t * 499))`
/// (the sample's second component); otherwise 0. The draw uses a
/// function-local `CIdvRandom`.
pub fn cidv_bezier_spline_scaled_variance(
    e: &mut Engine,
    this: Ptr<CIdvBezierSpline>,
    t: f32,
) -> f32 {
    let mut value = 0.0f32;
    let samples = this.addr() + 0x54;
    if element_count(e, samples, 0x18) == 500 {
        let scale = e.global::<f64>(SAMPLE_SCALE);
        let half = e.global::<f64>(HALF);
        let index = e.call(FTOL, &args![t as f64 * scale + half]).u32();
        check_index(e, index, element_count(e, samples, 0x18));
        let point = cvec_element(e, samples, index);
        value = x87_quiet(f32::from_bits(e.mem.u32(point + 4)));
        init_static_random(e, VARIANCE_RANDOM);
        let variance = e.get(this, CIdvBezierSpline::m_fVariance) as f64;
        let high = (variance * value as f64) as f32;
        let low =
            ((-x87_quiet(e.get(this, CIdvBezierSpline::m_fVariance))) as f64 * value as f64) as f32;
        value = e
            .call(RANDOM_GET_UNIFORM, &args![VARIANCE_RANDOM.0, low, high])
            .f32();
    }
    x87_quiet(value)
}

/// A `CVec` passed by value: its six words, as the code pushes them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CVecArg(pub [u32; 6]);

impl Arg for CVecArg {
    const WORDS: usize = 6;
    fn take(words: &[u32], i: &mut usize) -> Self {
        let mut value = [0u32; 6];
        for word in value.iter_mut() {
            *word = u32::take(words, i);
        }
        CVecArg(value)
    }
    fn put(self, out: &mut Vec<u32>) {
        out.extend_from_slice(&self.0);
    }
}

/// The six words of the `CVec` at `address`.
fn cvec_words(e: &Engine, address: u32) -> CVecArg {
    let mut words = [0u32; 6];
    for (i, word) in words.iter_mut().enumerate() {
        *word = e.mem.u32(address.wrapping_add(4 * i as u32));
    }
    CVecArg(words)
}

// Translated from 00b380e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b380e0 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `CIdvBezierSpline::AddPoint` (identified from the code; the engine map
/// has no name): adds a control point (`point`, two floats) with a tangent
/// (`tangent`, two floats, normalised) and a tangent `length`. When there
/// already is a point, the Bezier handles are appended to the spline points:
/// the previous point plus its tangent times its length, and the new point
/// minus the new tangent times `length`. Then the point is appended to the
/// control points and the spline points, the tangent to the tangents and the
/// length to the lengths.
pub fn fn_00b380e0(
    e: &mut Engine,
    this: Ptr<CIdvBezierSpline>,
    point: Ptr,
    tangent: Ptr,
    length: f32,
) {
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x274`, the cookie): 0x284 bytes
    // below `EBP`, then the saved `EBP`, the return address and the three argument slots; the
    // float pushed for the lengths is `length`'s own slot (`EBP+0x10`, +0x294). The new point
    // at +0x260 (`EBP-0x24`), the new tangent at +0x248 (`EBP-0x3c`), the first scaled tangent
    // and handle at +0x22c and +0x214 (`EBP-0x58`, `EBP-0x70`), the second at +0x1fc and +0x1e4
    // (`EBP-0x88`, `EBP-0xa0`).
    e.with_stack(0x298, |e, frame| {
        let frame = frame.addr();
        e.mem.set_u32(frame + 0x28c, point.addr());
        e.mem.set_u32(frame + 0x290, tangent.addr());
        e.mem.set_u32(frame + 0x294, length.to_bits());
        add_point_in_frame(e, this, point, tangent, length, frame);
    });
}

/// [`fn_00b380e0`] in the exe's frame (`frame` is its lowest address).
fn add_point_in_frame(
    e: &mut Engine,
    this: Ptr<CIdvBezierSpline>,
    point: Ptr,
    tangent: Ptr,
    length: f32,
    frame: u32,
) {
    let new_point = frame + 0x260;
    let x = x87_quiet(e.mem.f32(point.addr()));
    let y = x87_quiet(e.mem.f32(point.addr() + 4));
    e.call(CVEC_FROM_TWO, &args![new_point, x, y]);
    let new_tangent = frame + 0x248;
    let x = x87_quiet(e.mem.f32(tangent.addr()));
    let y = x87_quiet(e.mem.f32(tangent.addr() + 4));
    e.call(CVEC_FROM_TWO, &args![new_tangent, x, y]);
    e.call(CVEC_NORMALIZE, &args![new_tangent]);

    let controls = this.addr() + 0x0c;
    let tangents = this.addr() + 0x24;
    let lengths = this.addr() + 0x3c;
    let spline_points = this.addr() + 0x6c;
    if element_count(e, controls, 0x18) != 0 {
        let last = (element_count(e, controls, 0x18) - 1) as u32;
        let length_count = (e
            .mem
            .u32(lengths + 0x10)
            .wrapping_sub(e.mem.u32(lengths + 0x0c)) as i32)
            >> 2;
        check_index(e, last, length_count);
        let previous_length = e.mem.u32(lengths + 0x0c).wrapping_add(last.wrapping_mul(4));
        check_index(e, last, element_count(e, tangents, 0x18));
        let previous_tangent = cvec_element(e, tangents, last);
        let scaled = frame + 0x22c;
        let factor = x87_quiet(e.mem.f32(previous_length));
        let scaled = e
            .call(CVEC_SCALE, &args![previous_tangent, scaled, factor])
            .u32();
        check_index(e, last, element_count(e, controls, 0x18));
        let previous_point = cvec_element(e, controls, last);
        let handle = frame + 0x214;
        let handle = e
            .call(CVEC_ADD, &args![previous_point, handle, scaled])
            .u32();
        e.call(VECTOR_CVEC_PUSH_BACK, &args![spline_points, handle]);
        e.call(TRIVIAL_DESTRUCTOR, &args![handle]);
        e.call(TRIVIAL_DESTRUCTOR, &args![scaled]);

        let scaled = frame + 0x1fc;
        let scaled = e
            .call(CVEC_SCALE, &args![new_tangent, scaled, x87_quiet(length)])
            .u32();
        let handle = frame + 0x1e4;
        let handle = e
            .call(CVEC_SUBTRACT, &args![new_point, handle, scaled])
            .u32();
        e.call(VECTOR_CVEC_PUSH_BACK, &args![spline_points, handle]);
        e.call(TRIVIAL_DESTRUCTOR, &args![handle]);
        e.call(TRIVIAL_DESTRUCTOR, &args![scaled]);
    }
    e.call(VECTOR_CVEC_PUSH_BACK, &args![controls, new_point]);
    e.call(VECTOR_CVEC_PUSH_BACK, &args![spline_points, new_point]);
    e.call(VECTOR_CVEC_PUSH_BACK, &args![tangents, new_tangent]);
    let length_slot = frame + 0x294;
    e.call(VECTOR_FLOAT_PUSH_BACK, &args![lengths, length_slot]);
    e.call(TRIVIAL_DESTRUCTOR, &args![new_tangent]);
    e.call(TRIVIAL_DESTRUCTOR, &args![new_point]);
}

// Translated from 00b38430 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b38430 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// Builds the table of `count` evenly spaced samples (identified from the
/// code; the engine map has no name): evaluates the Bezier chain at
/// `i / count` into a temporary vector, then for each `k / count` finds the
/// segment of that vector whose first components bracket it and stores the
/// sample whose first component is `k / count` and second the linear
/// interpolation of the segment's second components. The first sample is the
/// first control point, the last one the last control point.
pub fn fn_00b38430(e: &mut Engine, this: Ptr<CIdvBezierSpline>, count: u32) {
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x184`, the cookie): 0x194
    // bytes. The local vector at +0x170 (`EBP-0x24`), the first blank vector at +0x10c
    // (`EBP-0x88`) and the second at +0xc4 (`EBP-0xd0`), the point result at +0x12c
    // (`EBP-0x68`) and the sample at +0x14c (`EBP-0x48`).
    e.with_stack(0x194, |e, frame| {
        resample_in_frame(e, this, count, frame.addr())
    });
}

/// [`fn_00b38430`] in the exe's frame (`frame` is its lowest address).
fn resample_in_frame(e: &mut Engine, this: Ptr<CIdvBezierSpline>, count: u32, frame: u32) {
    let local = frame + 0x170;
    e.call(VECTOR_CVEC_CONSTRUCT, &args![local]);
    let blank = frame + 0x10c;
    e.call(CVEC_DEFAULT, &args![blank]);
    let blank_value = cvec_words(e, blank);
    e.call(VECTOR_CVEC_RESIZE, &args![local, count, blank_value]);
    e.call(TRIVIAL_DESTRUCTOR, &args![blank]);

    let mut i = 0u32;
    while i < count {
        let u = ((i as u64 as i64 as f64) / (count as u64 as i64 as f64)) as f32;
        let result = frame + 0x12c;
        let point = e.call(SPLINE_POINT_AT, &args![this, result, u]).u32();
        check_index(e, i, element_count(e, local, 0x18));
        let destination = cvec_element(e, local, i);
        copy_cvec(e, point, destination);
        e.call(TRIVIAL_DESTRUCTOR, &args![result]);
        i = i.wrapping_add(1);
    }

    let samples = this.addr() + 0x54;
    e.call(VECTOR_CVEC_CLEAR, &args![samples]);
    let second_blank = frame + 0xc4;
    e.call(CVEC_DEFAULT, &args![second_blank]);
    let blank_value = cvec_words(e, second_blank);
    e.call(VECTOR_CVEC_RESIZE, &args![samples, count, blank_value]);
    e.call(TRIVIAL_DESTRUCTOR, &args![second_blank]);

    let mut start = 0u32;
    let controls = this.addr() + 0x0c;
    // The unsigned "above zero" test after `TEST` is a non-zero test.
    if element_count(e, controls, 0x18) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let first_control = cvec_element(e, controls, 0);
    if element_count(e, samples, 0x18) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let first_sample = cvec_element(e, samples, 0);
    copy_cvec(e, first_control, first_sample);

    let mut k = 1u32;
    while k < count.wrapping_sub(1) {
        let mut t = ((k as u64 as i64 as f64) / (count as u64 as i64 as f64)) as f32;
        let mut j = start;
        while j < count.wrapping_sub(1) {
            check_index(e, j, element_count(e, local, 0x18));
            let lower = cvec_element(e, local, j);
            let lower_x = e.mem.f32(lower);
            // `t >= lower.x`, false when either is a NaN.
            if lower_x <= t {
                let next = j.wrapping_add(1);
                check_index(e, next, element_count(e, local, 0x18));
                let upper = cvec_element(e, local, next);
                let upper_x = e.mem.f32(upper);
                // `upper.x > t`, false when either is a NaN.
                if upper_x > t {
                    start = j;
                    break;
                }
            }
            j = j.wrapping_add(1);
        }
        let sample = frame + 0x14c;
        e.call(
            CVEC_FROM_FIVE,
            &args![sample, x87_quiet(t), 0.0f32, 0.0f32, 0.0f32, 0.0f32],
        );
        check_index(e, start, element_count(e, local, 0x18));
        let a = cvec_element(e, local, start);
        let next = start.wrapping_add(1);
        check_index(e, next, element_count(e, local, 0x18));
        let b = cvec_element(e, local, next);
        check_index(e, start, element_count(e, local, 0x18));
        let c = cvec_element(e, local, start);
        t = ((t as f64 - e.mem.f32(a) as f64) / (e.mem.f32(b) as f64 - e.mem.f32(c) as f64)) as f32;
        check_index(e, next, element_count(e, local, 0x18));
        let d = cvec_element(e, local, next);
        check_index(e, start, element_count(e, local, 0x18));
        let f = cvec_element(e, local, start);
        let low = x87_quiet(e.mem.f32(f + 4));
        let y = ((e.mem.f32(d + 4) as f64 - low as f64) * t as f64 + low as f64) as f32;
        e.mem.set_f32(sample + 4, x87_quiet(y));
        check_index(e, k, element_count(e, samples, 0x18));
        let destination = cvec_element(e, samples, k);
        copy_cvec(e, sample, destination);
        e.call(TRIVIAL_DESTRUCTOR, &args![sample]);
        k = k.wrapping_add(1);
    }

    let control_count = element_count(e, controls, 0x18);
    let last = (control_count - 1) as u32;
    check_index(e, last, element_count(e, controls, 0x18));
    let last_control = cvec_element(e, controls, last);
    let end = count.wrapping_sub(1);
    check_index(e, end, element_count(e, samples, 0x18));
    let last_sample = cvec_element(e, samples, end);
    copy_cvec(e, last_control, last_sample);

    e.call(VECTOR_CVEC_TIDY, &args![local]);
    let proxy = e.mem.u32(local);
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00b38b50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b38b50 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// The point of the Bezier chain at `t` (identified from the code; the engine
/// map has no name): `t` is clamped to `0..=1` (a NaN becomes 0), scaled by
/// the number of segments (control points minus one); the integer part picks
/// the segment (the last control point at the end) and the rest is the
/// parameter of the cubic Bezier through the segment's four spline points.
/// With fewer than two control points the result is the zero point of size 2.
/// Returns `out`.
pub fn fn_00b38b50(e: &mut Engine, this: Ptr<CIdvBezierSpline>, out: Ptr, t: f32) -> Ptr {
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0xa8`, the cookie): 0xb8 bytes,
    // the result at +0x90 (`EBP-0x28`) and the segment point at +0x64 (`EBP-0x54`).
    e.with_stack(0xb8, |e, frame| {
        point_at_in_frame(e, this, out, t, frame.addr())
    })
}

/// [`fn_00b38b50`] in the exe's frame (`frame` is its lowest address).
fn point_at_in_frame(
    e: &mut Engine,
    this: Ptr<CIdvBezierSpline>,
    out: Ptr,
    t: f32,
    frame: u32,
) -> Ptr {
    let result = frame + 0x90;
    e.call(CVEC_SIZED, &args![result, 2u32]);
    let controls = this.addr() + 0x0c;
    let count = element_count(e, controls, 0x18);
    if count > 1 {
        // `t > 0 ? t : 0`, then `1 > t ? t : 1` (a NaN takes the bound).
        let mut t = t;
        t = if t > 0.0 { t } else { 0.0 };
        t = x87_quiet(t);
        t = if 1.0 > t { t } else { 1.0 };
        t = x87_quiet(t);
        t = ((count - 1) as f64 * t as f64) as f32;
        let index = e.call(FTOL, &args![t as f64]).i32();
        let fraction = (t as f64 - index as f64) as f32;
        let mut found = false;
        let mut i = 0i32;
        let spline_points = this.addr() + 0x6c;
        while i < count && !found {
            if i == count - 1 {
                check_index(e, i as u32, element_count(e, controls, 0x18));
                let control = cvec_element(e, controls, i as u32);
                copy_cvec(e, control, result);
                found = true;
            } else if (t as f64) >= (i as f64) && (t as f64) < ((i + 1) as f64) {
                let mut addresses = [0u32; 4];
                // The four spline points of the segment: 3i + 3, 3i + 2,
                // 3i + 1 and 3i, each checked in that order.
                for (slot, step) in [3i32, 2, 1, 0].into_iter().enumerate() {
                    let index = i.wrapping_mul(3).wrapping_add(step) as u32;
                    check_index(e, index, element_count(e, spline_points, 0x18));
                    addresses[slot] = cvec_element(e, spline_points, index);
                }
                let temporary = frame + 0x64;
                let point = e
                    .call(
                        0x00b3_8f40,
                        &args![
                            this,
                            temporary,
                            addresses[3],
                            addresses[2],
                            addresses[1],
                            addresses[0],
                            fraction
                        ],
                    )
                    .u32();
                copy_cvec(e, point, result);
                e.call(TRIVIAL_DESTRUCTOR, &args![temporary]);
                found = true;
            }
            i = i.wrapping_add(1);
        }
    }
    copy_cvec(e, result, out.addr());
    e.call(TRIVIAL_DESTRUCTOR, &args![result]);
    out
}

// Translated from 00b38f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The cubic Bezier point (identified from the code; the engine map has no
/// name; `thiscall` with `this` unused): de Casteljau on the first two
/// components of the four control points `p0..p3` at `t`, each `lerp` being
/// `(b - a) * t + a` rounded to `f32`. Builds the two-float `CVec` at `out`
/// and returns it.
#[allow(clippy::too_many_arguments)] // The exe passes the output and four points.
pub fn fn_00b38f40(
    e: &mut Engine,
    _this: Ptr,
    out: Ptr,
    p0: Ptr,
    p1: Ptr,
    p2: Ptr,
    p3: Ptr,
    t: f32,
) -> Ptr {
    let at = |e: &Engine, p: Ptr, offset: u32| e.mem.f32(p.addr() + offset);
    let q0 = [
        lerp(at(e, p0, 0), at(e, p1, 0), t),
        lerp(at(e, p0, 4), at(e, p1, 4), t),
    ];
    let q1 = [
        lerp(at(e, p1, 0), at(e, p2, 0), t),
        lerp(at(e, p1, 4), at(e, p2, 4), t),
    ];
    let q2 = [
        lerp(at(e, p2, 0), at(e, p3, 0), t),
        lerp(at(e, p2, 4), at(e, p3, 4), t),
    ];
    let r0 = [lerp(q0[0], q1[0], t), lerp(q0[1], q1[1], t)];
    let r1 = [lerp(q1[0], q2[0], t), lerp(q1[1], q2[1], t)];
    let y = lerp(r0[1], r1[1], t);
    let x = lerp(r0[0], r1[0], t);
    e.call(CVEC_FROM_TWO, &args![out, x87_quiet(x), x87_quiet(y)]);
    out
}

// Translated from 00b3c500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CVec::CVec(float x, float y)` (identified from the Xbox PDB layout it
/// fills; the engine map has no name): the first two components set, the
/// rest cleared, the size 2. Returns `this`.
pub fn cvec_from_two(e: &mut Engine, this: Ptr<CVec>, x: f32, y: f32) -> Ptr<CVec> {
    e.set(this, CVec::m_afData0, x87_quiet(x));
    e.set(this, CVec::m_afData1, x87_quiet(y));
    e.set(this, CVec::m_afData4, 0.0);
    e.set(this, CVec::m_afData3, 0.0);
    e.set(this, CVec::m_afData2, 0.0);
    e.set(this, CVec::m_nSize, 2);
    this
}

// Translated from 00b3c550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CVec::CVec(float, float, float, float, float)` (identified from the Xbox
/// PDB layout it fills; the engine map has no name): all five components
/// set, the size 5. Returns `this`.
pub fn cvec_from_five(
    e: &mut Engine,
    this: Ptr<CVec>,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    f: f32,
) -> Ptr<CVec> {
    e.set(this, CVec::m_afData0, x87_quiet(a));
    e.set(this, CVec::m_afData1, x87_quiet(b));
    e.set(this, CVec::m_afData2, x87_quiet(c));
    e.set(this, CVec::m_afData3, x87_quiet(d));
    e.set(this, CVec::m_afData4, x87_quiet(f));
    e.set(this, CVec::m_nSize, 5);
    this
}

// Translated from 00b3c600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CVec::Normalize` (Xbox PDB): divides every component by the length
/// (`CVec::Length`, stored as a `float`), unless the length compares equal to
/// 0 (a NaN length divides).
pub fn cvec_normalize(e: &mut Engine, this: Ptr<CVec>) {
    let length = e.call(CVEC_LENGTH, &args![this]).f32();
    // FCOMP with 0.0, then `TEST AH, 0x44` / `JNP`: only an equal result
    // skips the loop.
    if length != 0.0 {
        let mut i = 0i32;
        while i < e.get(this, CVec::m_nSize) {
            let address = this.addr().wrapping_add((i as u32).wrapping_mul(4));
            let divided = e.mem.f32(address) as f64 / length as f64;
            e.mem.set_f32(address, divided as f32);
            i = i.wrapping_add(1);
        }
    }
}

/// The component-wise `CVec` operators: a local `CVec` of the smaller size
/// (the smaller of the two sizes for `+` and `-`), `op` applied to the
/// components `0..this.size` and the six words copied to `out`. A size above
/// 6 would write past the local into the caller's frame; every `CVec`
/// constructor limits the size to 5, so that case is unreachable.
/// The local sits at `local_offset` of the operator's `frame_size`-byte frame (the exception
/// record, `SUB ESP` and the cookie under `EBP`).
fn cvec_combine(
    e: &mut Engine,
    this: Ptr<CVec>,
    out: Ptr<CVec>,
    local_size: i32,
    (frame_size, local_offset): (u32, u32),
    component: impl FnMut(&Engine, i32) -> f32,
) -> Ptr<CVec> {
    e.with_stack(frame_size, |e, frame| {
        combine_into(
            e,
            this,
            out,
            local_size,
            frame.addr() + local_offset,
            component,
        )
    })
}

/// [`cvec_combine`] with its local `CVec` at `local`.
fn combine_into(
    e: &mut Engine,
    this: Ptr<CVec>,
    out: Ptr<CVec>,
    local_size: i32,
    local: u32,
    mut component: impl FnMut(&Engine, i32) -> f32,
) -> Ptr<CVec> {
    e.call(CVEC_SIZED, &args![local, local_size]);
    let mut i = 0i32;
    while i < e.get(this, CVec::m_nSize) {
        let value = component(e, i);
        if (0..6).contains(&i) {
            e.mem.set_f32(local + 4 * i as u32, value);
        }
        i = i.wrapping_add(1);
    }
    copy_cvec(e, local, out.addr());
    e.call(TRIVIAL_DESTRUCTOR, &args![local]);
    out
}

// Translated from 00b3c660 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3c660 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `CVec::operator+` (Xbox PDB): the result `out` has the smaller of the two
/// sizes; its components `0..this.size` are the sums, each rounded to `f32`.
/// Returns `out`.
pub fn cvec_add(e: &mut Engine, this: Ptr<CVec>, out: Ptr<CVec>, other: Ptr<CVec>) -> Ptr<CVec> {
    let size = e.get(this, CVec::m_nSize);
    let other_size = e.get(other, CVec::m_nSize);
    let local_size = if size < other_size { size } else { other_size };
    // The frame: 0x38 bytes, the local at +0x14 (`EBP-0x24`).
    cvec_combine(e, this, out, local_size, (0x38, 0x14), |e, i| {
        let offset = (i as u32).wrapping_mul(4);
        let a = e.mem.f32(this.addr().wrapping_add(offset));
        let b = e.mem.f32(other.addr().wrapping_add(offset));
        (a as f64 + b as f64) as f32
    })
}

// Translated from 00b3c750 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3c750 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `CVec::operator-` (Xbox PDB): the result `out` has the smaller of the two
/// sizes; its components `0..this.size` are the differences, each rounded to
/// `f32`. Returns `out`.
pub fn cvec_subtract(
    e: &mut Engine,
    this: Ptr<CVec>,
    out: Ptr<CVec>,
    other: Ptr<CVec>,
) -> Ptr<CVec> {
    let size = e.get(this, CVec::m_nSize);
    let other_size = e.get(other, CVec::m_nSize);
    let local_size = if size < other_size { size } else { other_size };
    // The frame: 0x38 bytes, the local at +0x14 (`EBP-0x24`).
    cvec_combine(e, this, out, local_size, (0x38, 0x14), |e, i| {
        let offset = (i as u32).wrapping_mul(4);
        let a = e.mem.f32(this.addr().wrapping_add(offset));
        let b = e.mem.f32(other.addr().wrapping_add(offset));
        (a as f64 - b as f64) as f32
    })
}

// Translated from 00b3c840 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3c840 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `CVec::operator*(float)` (Xbox PDB): the result `out` has the size of
/// `this`; its components are the products with `factor`, each rounded to
/// `f32`. Returns `out`.
pub fn cvec_scale(e: &mut Engine, this: Ptr<CVec>, out: Ptr<CVec>, factor: f32) -> Ptr<CVec> {
    let size = e.get(this, CVec::m_nSize);
    // The frame: 0x34 bytes, the local at +0x10 (`EBP-0x24`).
    cvec_combine(e, this, out, size, (0x34, 0x10), |e, i| {
        let offset = (i as u32).wrapping_mul(4);
        let a = e.mem.f32(this.addr().wrapping_add(offset));
        (a as f64 * factor as f64) as f32
    })
}

// Translated from 00b39090 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b39090 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `std::vector<CVec>::~vector` (`fastcall`; identified from the code): tidies
/// the vector, then frees its container proxy.
pub fn cvec_vector_destructor(e: &mut Engine, this: Ptr<StdVector>) {
    e.call(VECTOR_CVEC_TIDY, &args![this]);
    let proxy = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00b39100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<CVec>::push_back(const CVec&)` (identified from the code):
/// with room left (size below the capacity; a vector without a buffer has
/// capacity 0) the value is copied to the end and the end advanced; otherwise
/// the insert at the end reallocates.
pub fn cvec_vector_push_back(e: &mut Engine, this: Ptr<StdVector>, value: Ptr) {
    let vector = this.addr();
    let size = element_count(e, vector, 0x18) as u32;
    let capacity = if e.mem.u32(vector + 0x0c) == 0 {
        0
    } else {
        let first = e.mem.u32(vector + 0x0c);
        let end = e.mem.u32(vector + 0x14);
        ((end.wrapping_sub(first) as i32) / 0x18) as u32
    };
    if size < capacity {
        let end = e.mem.u32(vector + 0x10);
        // The two byte arguments after the allocator are the allocator tags
        // of the call; the callee does not read them (the second is an
        // uninitialised stack byte in the exe).
        e.call(
            VECTOR_CVEC_FILL,
            &args![end, 1u32, value, vector + 8, 0u32, 0u32],
        );
        e.mem.set_u32(vector + 0x10, end.wrapping_add(0x18));
    } else {
        let end = e.mem.u32(vector + 0x10);
        check_position(e, vector, end);
        let proxy = e.mem.u32(vector);
        // The exe's frame (`SUB ESP,0x60` under `EBP`): the returned iterator at +0x48
        // (`EBP-0x18`).
        e.with_stack(0x60, |e, frame| {
            let out = frame.addr() + 0x48;
            e.call(VECTOR_CVEC_INSERT_ONE, &args![this, out, proxy, end, value]);
        });
    }
}

// Translated from 00b393b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map's `begin()` (identified from the code; the engine map has no
/// name): the iterator for the leftmost node (the head's left link): the
/// proxy word of the map, then the node. Returns `out`.
pub fn fn_00b393b0(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let head = e.mem.u32(this.addr() + 0x18);
    let leftmost = e.mem.u32(head);
    e.mem.set_u32(out.addr(), 0);
    e.mem.set_u32(out.addr() + 4, leftmost);
    if this.addr() == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let proxy = e.mem.u32(this.addr());
    e.mem.set_u32(out.addr(), proxy);
    out
}

// Translated from 00b39400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map's `clear()` (identified from the code; the engine map has no
/// name): erases the tree below the root, then makes the head's parent, left
/// and right links point at the head and the size 0.
pub fn fn_00b39400(e: &mut Engine, this: Ptr) {
    let map = this.addr();
    let head = e.mem.u32(map + 0x18);
    let root = e.mem.u32(head + 4);
    e.call(MAP_ERASE_SUBTREE, &args![this, root]);
    let head = e.mem.u32(map + 0x18);
    e.mem.set_u32(head + 4, head);
    e.mem.set_u32(map + 0x1c, 0);
    let head = e.mem.u32(map + 0x18);
    e.mem.set_u32(head, head);
    let head = e.mem.u32(map + 0x18);
    e.mem.set_u32(head + 8, head);
}

// Translated from 00b39460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the container proxy at the start of the object (identified from the
/// code; the engine map has no name).
pub fn fn_00b39460(e: &mut Engine, this: Ptr) {
    let proxy = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00b396c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<CVec>::insert(position, value)` (identified from the code):
/// remembers the offset of `position` from the first element (0 for an empty
/// vector; the iterators are checked), inserts one copy of `value` there
/// (`00b3a0d0`, which may reallocate) and writes the iterator for the same
/// offset in the new buffer to `out`. Returns `out`.
pub fn cvec_vector_insert_one(
    e: &mut Engine,
    this: Ptr<StdVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: Ptr,
) -> Ptr {
    let vector = this.addr();
    let offset = if element_count(e, vector, 0x18) == 0 {
        0
    } else {
        let first = e.mem.u32(vector + 0x0c);
        check_position(e, vector, first);
        let own_proxy = e.mem.u32(vector);
        if proxy == 0 || proxy != own_proxy {
            e.call(INVALID_PARAMETER, &args![]);
        }
        (position.wrapping_sub(first) as i32) / 0x18
    };
    e.call(
        VECTOR_CVEC_INSERT_N,
        &args![this, proxy, position, 1u32, value],
    );
    let first = e.mem.u32(vector + 0x0c);
    check_position(e, vector, first);
    // The exe's frame (`SUB ESP,0x34` under `EBP`): the iterator at +0x10 (`EBP-0x24`).
    e.with_stack(0x34, |e, frame| {
        let iterator = frame.addr() + 0x10;
        e.mem.set_u32(iterator, e.mem.u32(vector));
        e.mem.set_u32(iterator + 4, first);
        e.call(ITERATOR_ADVANCE, &args![iterator, offset]);
        let proxy = e.mem.u32(iterator);
        let position = e.mem.u32(iterator + 4);
        e.mem.set_u32(out.addr(), proxy);
        e.mem.set_u32(out.addr() + 4, position);
    });
    out
}

// Translated from 00b397f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<CVec>::_Tidy` (identified from the code): when there is a
/// buffer, runs the (empty) element destructor over `first..last` and frees
/// the buffer; then clears the three pointers.
pub fn cvec_vector_tidy(e: &mut Engine, this: Ptr<StdVector>) {
    let vector = this.addr();
    if e.mem.u32(vector + 0x0c) != 0 {
        let last = e.mem.u32(vector + 0x10);
        let mut element = e.mem.u32(vector + 0x0c);
        while element != last {
            e.call(TRIVIAL_DESTRUCTOR, &args![element]);
            if DESTRUCTOR_FREES {
                // The destructor's delete flag is the constant 0 (`XOR EAX, EAX`,
                // `AND EAX, 1`), so the element is never freed here.
                e.call(OPERATOR_DELETE, &args![element]);
            }
            element = element.wrapping_add(0x18);
        }
        let buffer = e.mem.u32(vector + 0x0c);
        e.call(OPERATOR_DELETE, &args![buffer]);
    }
    e.mem.set_u32(vector + 0x0c, 0);
    e.mem.set_u32(vector + 0x10, 0);
    e.mem.set_u32(vector + 0x14, 0);
}

// Translated from 00b39490 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b39490 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `std::vector<CVec>::resize(new_size, value)` (identified from the code;
/// `value` is passed by value and destroyed at the end): grows with copies of
/// `value` (insert at the end) or erases the elements from `new_size` on. The
/// sizes are compared as unsigned numbers.
pub fn cvec_vector_resize(e: &mut Engine, this: Ptr<StdVector>, new_size: u32, value: CVecArg) {
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0xb8`, the cookie): 0xc8 bytes
    // below `EBP`, then the saved `EBP`, the return address, `new_size` (+0xd0) and the `CVec`
    // passed by value (+0xd4, `EBP+0xc`), which is the copy inserted. The advanced iterator at
    // +0x68 (`EBP-0x60`) and the erase result at +0x7c (`EBP-0x4c`).
    e.with_stack(0xec, |e, frame| {
        resize_in_frame(e, this, new_size, value, frame.addr())
    });
}

/// [`cvec_vector_resize`] in the exe's frame (`frame` is its lowest address).
fn resize_in_frame(
    e: &mut Engine,
    this: Ptr<StdVector>,
    new_size: u32,
    value: CVecArg,
    frame: u32,
) {
    let vector = this.addr();
    e.mem.set_u32(frame + 0xd0, new_size);
    let copy = frame + 0xd4;
    for (i, word) in value.0.iter().enumerate() {
        e.mem.set_u32(copy + 4 * i as u32, *word);
    }
    if (element_count(e, vector, 0x18) as u32) < new_size {
        let last = e.mem.u32(vector + 0x10);
        check_position(e, vector, last);
        let proxy = e.mem.u32(vector);
        let missing = new_size.wrapping_sub(element_count(e, vector, 0x18) as u32);
        e.call(
            VECTOR_CVEC_INSERT_N,
            &args![this, proxy, last, missing, copy],
        );
    } else if new_size < element_count(e, vector, 0x18) as u32 {
        let last = e.mem.u32(vector + 0x10);
        check_position(e, vector, last);
        let last_proxy = e.mem.u32(vector);
        let first = e.mem.u32(vector + 0x0c);
        check_position(e, vector, first);
        let first_proxy = e.mem.u32(vector);
        let iterator = frame + 0x68;
        e.mem.set_u32(iterator, first_proxy);
        e.mem.set_u32(iterator + 4, first);
        e.call(ITERATOR_ADVANCE, &args![iterator, new_size]);
        let erase_proxy = e.mem.u32(iterator);
        let erase_position = e.mem.u32(iterator + 4);
        let result = frame + 0x7c;
        e.call(
            VECTOR_CVEC_ERASE,
            &args![this, result, erase_proxy, erase_position, last_proxy, last],
        );
    }
    e.call(TRIVIAL_DESTRUCTOR, &args![copy]);
}

// Translated from 00b3a0d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3a0d0 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `std::vector<CVec>::_Insert_n(where, count, value)` (identified from the
/// code; the first argument is the iterator's proxy and is not read).
/// Nothing happens for a count of 0; a count above the room left reports a
/// length error. Without room for the new size the buffer is replaced by one
/// of at least 1.5 times the capacity: the head is copied, `count` copies of
/// `value` are filled in, the tail is copied and the old buffer freed. With
/// room, the tail moves up by `count` elements: when the tail is shorter than
/// `count` the gap beyond the old end is filled with copies of `value` first,
/// and the elements from `where` to the old end are then overwritten with
/// `value`. `value` is copied to a local first, as it may point into the
/// vector.
pub fn cvec_vector_insert_n(
    e: &mut Engine,
    this: Ptr<StdVector>,
    _proxy: u32,
    position: u32,
    count: u32,
    value: Ptr,
) {
    let vector = this.addr();
    let first = e.mem.u32(vector + 0x0c);
    let capacity = if first == 0 {
        0
    } else {
        ((e.mem.u32(vector + 0x14).wrapping_sub(first)) as i32 / 0x18) as u32
    };
    if count == 0 {
        return;
    }
    // `max > 0 ? max : 1` takes the maximum: the constant is never 0.
    let size = element_count(e, vector, 0x18) as u32;
    if CVEC_MAX_COUNT.wrapping_sub(size) < count {
        e.call(VECTOR_LENGTH_ERROR, &args![]);
        return;
    }
    let needed = size.wrapping_add(count);
    if capacity < needed {
        let half = capacity >> 1;
        let mut new_capacity = if CVEC_MAX_COUNT.wrapping_sub(half) < capacity {
            0
        } else {
            capacity.wrapping_add(half)
        };
        if new_capacity < needed {
            new_capacity = needed;
        }
        let buffer = e.call(VECTOR_ALLOCATE, &args![new_capacity, 0u32]).u32();
        // The two byte arguments after the allocator of the helpers are
        // allocator tags that the callees do not read (the first is an
        // uninitialised stack byte in the exe).
        let first = e.mem.u32(vector + 0x0c);
        let head_end = e
            .call(CVEC_RANGE_COPY, &args![first, position, buffer, vector + 8])
            .u32();
        e.call(
            VECTOR_CVEC_FILL,
            &args![head_end, count, value, vector + 8, 0u32, 0u32],
        );
        let tail_start = head_end.wrapping_add(count.wrapping_mul(0x18));
        let last = e.mem.u32(vector + 0x10);
        e.call(
            CVEC_RANGE_COPY,
            &args![position, last, tail_start, vector + 8],
        );
        let new_size = (element_count(e, vector, 0x18) as u32).wrapping_add(count);
        if e.mem.u32(vector + 0x0c) != 0 {
            let last = e.mem.u32(vector + 0x10);
            let mut element = e.mem.u32(vector + 0x0c);
            while element != last {
                e.call(TRIVIAL_DESTRUCTOR, &args![element]);
                if DESTRUCTOR_FREES {
                    // The inlined destructor call's delete flag is the
                    // constant 0, so the element is never freed here.
                    e.call(OPERATOR_DELETE, &args![element]);
                }
                element = element.wrapping_add(0x18);
            }
            let old_buffer = e.mem.u32(vector + 0x0c);
            e.call(OPERATOR_DELETE, &args![old_buffer]);
        }
        e.mem.set_u32(
            vector + 0x14,
            new_capacity.wrapping_mul(0x18).wrapping_add(buffer),
        );
        e.mem.set_u32(
            vector + 0x10,
            new_size.wrapping_mul(0x18).wrapping_add(buffer),
        );
        e.mem.set_u32(vector + 0x0c, buffer);
        return;
    }
    let last = e.mem.u32(vector + 0x10);
    let tail = (last.wrapping_sub(position) as i32 / 0x18) as u32;
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x248`, the cookie): 0x258
    // bytes. The copy of `value` is at +0x224 (`EBP-0x34`) when the tail is shorter than
    // `count`, else at +0x208 (`EBP-0x50`).
    e.with_stack(0x258, |e, frame| {
        let local = frame.addr() + if tail < count { 0x224 } else { 0x208 };
        copy_cvec(e, value.addr(), local);
        let span = count.wrapping_mul(0x18);
        if tail < count {
            e.call(
                CVEC_RANGE_COPY_ALIAS,
                &args![position, last, position.wrapping_add(span), vector + 8],
            );
            let tail = (e.mem.u32(vector + 0x10).wrapping_sub(position) as i32 / 0x18) as u32;
            let missing = count.wrapping_sub(tail);
            let old_last = e.mem.u32(vector + 0x10);
            e.call(
                VECTOR_CVEC_FILL,
                &args![old_last, missing, local, vector + 8, 0u32, 0u32],
            );
            let grown = span.wrapping_add(e.mem.u32(vector + 0x10));
            e.mem.set_u32(vector + 0x10, grown);
            let end = e.mem.u32(vector + 0x10).wrapping_sub(span);
            let mut element = position;
            while element != end {
                copy_cvec(e, local, element);
                element = element.wrapping_add(0x18);
            }
        } else {
            let moved_from = last.wrapping_sub(span);
            let current_last = e.mem.u32(vector + 0x10);
            let new_last = e
                .call(
                    CVEC_RANGE_COPY_ALIAS,
                    &args![moved_from, last, current_last, vector + 8],
                )
                .u32();
            e.mem.set_u32(vector + 0x10, new_last);
            e.call(COPY_BACKWARD, &args![position, moved_from, last]);
            let end = span.wrapping_add(position);
            let mut element = position;
            while element != end {
                copy_cvec(e, local, element);
                element = element.wrapping_add(0x18);
            }
        }
        e.call(TRIVIAL_DESTRUCTOR, &args![local]);
    });
}

// Translated from 00b3b5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `CVec` range copy-construct (identified from the code): forwards
/// `first`, `last`, `dest` and the allocator to `_Uninit_copy` with the two
/// allocator tag bytes (the first an uninitialised stack byte in the exe, 0
/// here; the second 0). Returns what it returns, the end of the copy.
pub fn fn_00b3b5a0(e: &mut Engine, first: u32, last: u32, dest: u32, allocator: u32) -> u32 {
    e.call(
        UNINIT_COPY,
        &args![first, last, dest, allocator, 0u32, 0u32],
    )
    .u32()
}

// Translated from 00b3b6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same range copy-construct as `00b3b5a0`, built from a deeper chain of
/// forwarding helpers (identified from the code): forwards `first`, `last`,
/// `dest` and the allocator to `_Uninit_copy` and returns its result.
pub fn fn_00b3b6f0(e: &mut Engine, first: u32, last: u32, dest: u32, allocator: u32) -> u32 {
    e.call(
        UNINIT_COPY,
        &args![first, last, dest, allocator, 0u32, 0u32],
    )
    .u32()
}

// Translated from 00b3b5f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3b5f0 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// `std::_Uninit_fill_n` for `CVec` (identified from the code): copy-constructs
/// `count` copies of `*value` at `dest`, 0x18 bytes apart; a null destination
/// is skipped. The allocator and the two tag bytes are pushed by the callers
/// and not read.
pub fn cvec_uninit_fill_n(
    e: &mut Engine,
    dest: u32,
    count: u32,
    value: Ptr,
    _allocator: u32,
    _first_tag: u32,
    _second_tag: u32,
) {
    let mut dest = dest;
    let mut count = count;
    while count != 0 {
        if dest != 0 {
            copy_cvec(e, value.addr(), dest);
        }
        count = count.wrapping_sub(1);
        dest = dest.wrapping_add(0x18);
    }
}

/// The left link of tree node `node`.
fn node_left(e: &Engine, node: u32) -> u32 {
    e.mem.u32(node)
}

/// The parent link of tree node `node`.
fn node_parent(e: &Engine, node: u32) -> u32 {
    e.mem.u32(node.wrapping_add(4))
}

/// The right link of tree node `node`.
fn node_right(e: &Engine, node: u32) -> u32 {
    e.mem.u32(node.wrapping_add(8))
}

/// The colour byte of tree node `node`, sign-extended as the code reads it.
fn node_color(e: &Engine, node: u32) -> i32 {
    e.mem.u8(node.wrapping_add(0x2c)) as i8 as i32
}

/// Whether tree node `node` is the nil (head) node: the sign-extended
/// `_Isnil` byte is non-zero.
fn node_is_nil(e: &Engine, node: u32) -> bool {
    e.mem.u8(node.wrapping_add(0x2d)) as i8 != 0
}

/// The head node of the map at `map`.
fn map_head(e: &Engine, map: u32) -> u32 {
    e.mem.u32(map.wrapping_add(0x18))
}

/// The debug iterator check against a container proxy: the proxy must be
/// non-null and equal to `other`, or the invalid-parameter handler runs.
fn check_proxy(e: &mut Engine, proxy: u32, other: u32) {
    if proxy == 0 || proxy != other {
        e.call(INVALID_PARAMETER, &args![]);
    }
}

/// `string::compare(a, b) < 0`: the `SETL` of the result.
fn key_less(e: &mut Engine, a: u32, b: u32) -> bool {
    e.call(STRING_COMPARE, &args![a, b]).i32() < 0
}

/// Throws an exception built from a message: the text string, the
/// `runtime_error`-style constructor, the vtable stored over the object and
/// `_CxxThrowException`.
fn throw_with_text(e: &mut Engine, text: u32, vtable: u32, throw_info: u32) -> ! {
    e.with_stack(0x1c, |e, string| {
        e.call(STRING_FROM_C_STRING, &args![string, text]);
        // A std::runtime_error is 0x28 bytes: the 0xc-byte std::exception (00ec78e0) and the
        // 0x1c-byte message string 0044a310 builds at +0xc.
        e.with_stack(0x28, |e, exception| {
            e.call(RUNTIME_ERROR_FROM_STRING, &args![exception, string]);
            e.mem.set_u32(exception.addr(), vtable);
            e.call(CXX_THROW_EXCEPTION, &args![exception, throw_info]);
        });
    });
    unreachable!("__CxxThrowException does not return (00ec7894)")
}

// Translated from 00b3b050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map iterator constructor (identified from the code; `iterator(node,
/// container)`): the node, and the container's proxy word (the container
/// must not be null). Returns `this`.
pub fn fn_00b3b050(e: &mut Engine, this: Ptr, node: u32, container: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, node);
    if container == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let proxy = e.mem.u32(container);
    e.mem.set_u32(this.addr(), proxy);
    this
}

// Translated from 00b3b090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map iterator's dereference (identified from the code): checks that
/// the iterator has a container and does not point at the head node, and
/// returns the address of the node's pair (node + 0xc).
pub fn fn_00b3b090(e: &mut Engine, this: Ptr) -> u32 {
    let iterator = this.addr();
    let proxy = e.mem.u32(iterator);
    if proxy == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let container = if e.mem.u32(iterator) != 0 {
        e.mem.u32(proxy)
    } else {
        0
    };
    let node = e.mem.u32(iterator + 4);
    if node == e.mem.u32(container.wrapping_add(0x18)) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    node.wrapping_add(0xc)
}

// Translated from 00b3b100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map iterator's `operator++` (identified from the code): the in-order
/// successor of the node. Incrementing the head node reports an invalid
/// parameter and leaves the iterator unchanged.
pub fn fn_00b3b100(e: &mut Engine, this: Ptr) {
    let iterator = this.addr();
    if e.mem.u32(iterator) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let node = e.mem.u32(iterator + 4);
    if node_is_nil(e, node) {
        e.call(INVALID_PARAMETER, &args![]);
        return;
    }
    let right = node_right(e, node);
    if !node_is_nil(e, right) {
        let mut leftmost = right;
        while !node_is_nil(e, node_left(e, leftmost)) {
            leftmost = node_left(e, leftmost);
        }
        e.mem.set_u32(iterator + 4, leftmost);
    } else {
        loop {
            let current = e.mem.u32(iterator + 4);
            let parent = node_parent(e, current);
            if !node_is_nil(e, parent) && current == node_right(e, parent) {
                e.mem.set_u32(iterator + 4, parent);
            } else {
                e.mem.set_u32(iterator + 4, parent);
                break;
            }
        }
    }
}

// Translated from 00b3b440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map iterator's `operator--` (identified from the code): the in-order
/// predecessor. From the head node it moves to the rightmost node; moving
/// before the first node reports an invalid parameter.
pub fn fn_00b3b440(e: &mut Engine, this: Ptr) {
    let iterator = this.addr();
    if e.mem.u32(iterator) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let node = e.mem.u32(iterator + 4);
    if node_is_nil(e, node) {
        let rightmost = node_right(e, node);
        e.mem.set_u32(iterator + 4, rightmost);
        if node_is_nil(e, rightmost) {
            e.call(INVALID_PARAMETER, &args![]);
        }
        return;
    }
    let left = node_left(e, node);
    if !node_is_nil(e, left) {
        let mut rightmost = left;
        while !node_is_nil(e, node_right(e, rightmost)) {
            rightmost = node_right(e, rightmost);
        }
        e.mem.set_u32(iterator + 4, rightmost);
        return;
    }
    let parent = loop {
        let current = e.mem.u32(iterator + 4);
        let parent = node_parent(e, current);
        if !node_is_nil(e, parent) && current == node_left(e, parent) {
            e.mem.set_u32(iterator + 4, parent);
        } else {
            break parent;
        }
    };
    let current = e.mem.u32(iterator + 4);
    if node_is_nil(e, current) {
        e.call(INVALID_PARAMETER, &args![]);
    } else {
        e.mem.set_u32(iterator + 4, parent);
    }
}

// Translated from 00b3afe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map's `lower_bound(key)` (identified from the code): walks down from
/// the root, going right while the node's key compares below `key` and
/// remembering the node (starting from the head node) where it goes left.
/// Returns the node, not an iterator.
pub fn fn_00b3afe0(e: &mut Engine, this: Ptr, key: Ptr) -> u32 {
    let head = map_head(e, this.addr());
    let mut node = node_parent(e, head);
    let mut result = head;
    while !node_is_nil(e, node) {
        if key_less(e, node.wrapping_add(0xc), key.addr()) {
            node = node_right(e, node);
        } else {
            result = node;
            node = node_left(e, node);
        }
    }
    result
}

// Translated from 00b3b1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tree's left rotation about `node` (identified from the code): the
/// right child takes its place.
pub fn fn_00b3b1d0(e: &mut Engine, this: Ptr, node: u32) {
    let child = node_right(e, node);
    let inner = node_left(e, child);
    e.mem.set_u32(node.wrapping_add(8), inner);
    if !node_is_nil(e, node_left(e, child)) {
        let inner = node_left(e, child);
        e.mem.set_u32(inner.wrapping_add(4), node);
    }
    let parent = node_parent(e, node);
    e.mem.set_u32(child.wrapping_add(4), parent);
    let head = map_head(e, this.addr());
    if node == node_parent(e, head) {
        e.mem.set_u32(head.wrapping_add(4), child);
    } else {
        let parent = node_parent(e, node);
        if node == node_left(e, parent) {
            e.mem.set_u32(parent, child);
        } else {
            e.mem.set_u32(parent.wrapping_add(8), child);
        }
    }
    e.mem.set_u32(child, node);
    e.mem.set_u32(node.wrapping_add(4), child);
}

// Translated from 00b3b280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tree's right rotation about `node` (identified from the code): the
/// left child takes its place.
pub fn fn_00b3b280(e: &mut Engine, this: Ptr, node: u32) {
    let child = node_left(e, node);
    let inner = node_right(e, child);
    e.mem.set_u32(node, inner);
    if !node_is_nil(e, node_right(e, child)) {
        let inner = node_right(e, child);
        e.mem.set_u32(inner.wrapping_add(4), node);
    }
    let parent = node_parent(e, node);
    e.mem.set_u32(child.wrapping_add(4), parent);
    let head = map_head(e, this.addr());
    if node == node_parent(e, head) {
        e.mem.set_u32(head.wrapping_add(4), child);
    } else {
        let parent = node_parent(e, node);
        if node == node_right(e, parent) {
            e.mem.set_u32(parent.wrapping_add(8), child);
        } else {
            e.mem.set_u32(parent, child);
        }
    }
    e.mem.set_u32(child.wrapping_add(8), node);
    e.mem.set_u32(node.wrapping_add(4), child);
}

// Translated from 00b3b540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map's node allocator (identified from the code; `allocate(count,
/// hint)`, the hint unused): `operator new(count * 0x30)`. A count of 0
/// allocates 0 bytes; a count too large for the multiplication throws
/// `std::bad_alloc`.
pub fn fn_00b3b540(e: &mut Engine, count: u32, _hint: u32) -> u32 {
    if count > 0 && 0xffff_ffffu32 / count < 0x30 {
        // The bad_alloc is built from a pointer to a null `const char*`.
        e.with_stack(4, |e, message| {
            e.mem.set_u32(message.addr(), 0);
            e.with_stack(0xc, |e, exception| {
                e.call(BAD_ALLOC_CONSTRUCTOR, &args![exception, message]);
                e.mem.set_u32(exception.addr(), BAD_ALLOC_VTABLE);
                e.call(CXX_THROW_EXCEPTION, &args![exception, BAD_ALLOC_THROW_INFO]);
            });
        });
        unreachable!("__CxxThrowException does not return (00ec7894)")
    }
    // For a count of 0 the code sets the count to 0 and allocates 0 bytes.
    e.call(OPERATOR_NEW, &args![count.wrapping_mul(0x30)]).u32()
}

// Translated from 00b3b330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3b330 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// The tree's `_Buynode(left, parent, right, value, color)` (identified from
/// the code): allocates one node; unless the allocation returned null, sets
/// its three links, copies the key string (an empty string assigned from the
/// pair's) and the mapped value, the colour byte and a clear nil flag.
/// Returns the allocation.
pub fn fn_00b3b330(
    e: &mut Engine,
    _this: Ptr,
    left: u32,
    parent: u32,
    right: u32,
    pair: Ptr,
    color: u32,
) -> u32 {
    let node = e.call(TREE_ALLOCATE_NODE, &args![1u32, 0u32]).u32();
    if node != 0 {
        e.mem.set_u32(node, left);
        e.mem.set_u32(node.wrapping_add(4), parent);
        e.mem.set_u32(node.wrapping_add(8), right);
        let key = node.wrapping_add(0xc);
        e.call(STRING_TIDY, &args![key, 0u32, 0u32]);
        let npos = e.global::<u32>(STRING_NPOS);
        e.call(STRING_ASSIGN, &args![key, pair, 0u32, npos]);
        let mapped = e.mem.u32(pair.addr().wrapping_add(0x1c));
        e.mem.set_u32(key.wrapping_add(0x1c), mapped);
        e.mem.set_u8(node.wrapping_add(0x2c), color as u8);
        e.mem.set_u8(node.wrapping_add(0x2d), 0);
    }
    node
}

// Translated from 00b3a050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tree's `_Erase(node)` (identified from the code): erases the subtree
/// of `node` (recursing on the right links, looping on the left): every
/// node's key string is tidied and the node freed.
pub fn fn_00b3a050(e: &mut Engine, this: Ptr, node: u32) {
    let mut next = node;
    let mut current = node;
    while !node_is_nil(e, next) {
        let right = node_right(e, next);
        e.call(MAP_ERASE_SUBTREE, &args![this, right]);
        next = node_left(e, next);
        e.call(STRING_TIDY, &args![current.wrapping_add(0xc), 1u32, 0u32]);
        if DESTRUCTOR_FREES {
            // The inlined string destructor's delete flag is the constant 0.
            e.call(OPERATOR_DELETE, &args![current]);
        }
        e.call(OPERATOR_DELETE, &args![current]);
        current = next;
    }
}

// Translated from 00b3b9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tree's node allocator for the head node (identified from the code;
/// `this` is the allocator inside the map): `allocate(count, 0)`.
pub fn fn_00b3b9d0(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    e.call(TREE_ALLOCATE_NODE, &args![count, 0u32]).u32()
}

// Translated from 00b3b880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3b880 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// The tree's `_Buyheadnode` (identified from the code): allocates the head
/// node through the allocator at map + 0xc, clears its three links (each only
/// when the address is non-null), and sets the colour byte 1 and the nil flag
/// 0. Returns the node.
pub fn fn_00b3b880(e: &mut Engine, this: Ptr) -> u32 {
    let node = e
        .call(TREE_ALLOCATE_HEAD, &args![this.addr() + 0xc, 1u32])
        .u32();
    if node != 0 {
        e.mem.set_u32(node, 0);
    }
    if node.wrapping_add(4) != 0 {
        e.mem.set_u32(node.wrapping_add(4), 0);
    }
    if node.wrapping_add(8) != 0 {
        e.mem.set_u32(node.wrapping_add(8), 0);
    }
    e.mem.set_u8(node.wrapping_add(0x2c), NODE_BLACK);
    e.mem.set_u8(node.wrapping_add(0x2d), 0);
    node
}

// Translated from 00b3b810 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tree's `_Init` (identified from the code): builds the head node, marks
/// it nil, points its parent, left and right links at itself and sets the
/// size to 0.
pub fn fn_00b3b810(e: &mut Engine, this: Ptr) {
    let map = this.addr();
    let head = e.call(TREE_BUY_HEAD, &args![this]).u32();
    e.mem.set_u32(map + 0x18, head);
    let head = e.mem.u32(map + 0x18);
    e.mem.set_u8(head.wrapping_add(0x2d), 1);
    let head = e.mem.u32(map + 0x18);
    let own = e.mem.u32(map + 0x18);
    e.mem.set_u32(head.wrapping_add(4), own);
    let head = e.mem.u32(map + 0x18);
    let own = e.mem.u32(map + 0x18);
    e.mem.set_u32(head, own);
    let head = e.mem.u32(map + 0x18);
    let own = e.mem.u32(map + 0x18);
    e.mem.set_u32(head.wrapping_add(8), own);
    e.mem.set_u32(map + 0x1c, 0);
}

// Translated from 00b3b9f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3b9f0 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// Builds the map's container proxy (identified from the code): a heap block
/// whose first word points back at the map (a null allocation stays null),
/// stored in the map's first word. The comparator byte and the allocator
/// pointer are not read. Returns `this`.
pub fn fn_00b3b9f0(e: &mut Engine, this: Ptr, _comparator: u32, _allocator: u32) -> Ptr {
    let block = e.call(PROXY_ALLOCATE, &args![1u32, 0u32]).u32();
    let proxy = if block != 0 {
        e.mem.set_u32(block, this.addr());
        block
    } else {
        0
    };
    e.mem.set_u32(this.addr(), proxy);
    this
}

// Translated from 00b3b770 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3b770 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// The map constructor taking a comparator (identified from the code): reads
/// the comparator's byte, builds the container proxy and initialises the
/// tree. The second argument is not read. Returns `this`.
pub fn fn_00b3b770(e: &mut Engine, this: Ptr, comparator: Ptr, _allocator: u32) -> Ptr {
    let byte = e.mem.u8(comparator.addr()) as u32;
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x3c`, the cookie): 0x4c bytes.
    // The allocator goes by value in a slot made by `PUSH ECX` while `ECX` holds the address
    // of the byte at +0x1e (`EBP-0x2e`, 00b3b79e), so that address is the word pushed.
    e.with_stack(0x4c, |e, frame| {
        let allocator = frame.addr() + 0x1e;
        e.call(TREE_PROXY_NEW, &args![this, byte, allocator]);
        e.call(TREE_INIT, &args![this]);
    });
    this
}

// Translated from 00b3ba90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map's `_Tidy` (identified from the code): erases every node (the range
/// from the first node to the head), frees the head node and clears the head
/// pointer and the size.
pub fn fn_00b3ba90(e: &mut Engine, this: Ptr) {
    let map = this.addr();
    let head = map_head(e, map);
    if map == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let end_proxy = e.mem.u32(map);
    let begin = node_left(e, map_head(e, map));
    if map == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let begin_proxy = e.mem.u32(map);
    // The exe's frame (`SUB ESP,0x8c` under `EBP`): the erase result at +0x64 (`EBP-0x28`).
    e.with_stack(0x8c, |e, frame| {
        let out = frame.addr() + 0x64;
        e.call(
            TREE_ERASE_RANGE,
            &args![this, out, begin_proxy, begin, end_proxy, head],
        );
    });
    let head = map_head(e, map);
    e.call(OPERATOR_DELETE, &args![head]);
    e.mem.set_u32(map + 0x18, 0);
    e.mem.set_u32(map + 0x1c, 0);
}

/// Sets the colour byte of tree node `node`.
fn set_node_color(e: &mut Engine, node: u32, color: u8) {
    e.mem.set_u8(node.wrapping_add(0x2c), color);
}

/// The tree's `_Insert` call: writes the iterator of the new node to `out`
/// and returns the address the callee returns.
fn insert_node(e: &mut Engine, map: u32, out: u32, left: u32, parent: u32, pair: u32) -> u32 {
    e.call(TREE_INSERT_NODE, &args![map, out, left, parent, pair])
        .u32()
}

// Translated from 00b3ac00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3ac00 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// The tree's `_Insert(left_flag, where, value)` (identified from the code):
/// adds one node as the left (`left_flag` byte set) or right child of
/// `where` and rebalances the red-black tree. A map already at its maximum
/// size throws a `length_error`. The first node becomes the root and the
/// leftmost and rightmost links; a new leftmost or rightmost child updates the
/// head's links. A red parent is repaired with the usual recolouring or one
/// or two rotations, and the root is made black. Writes the iterator of the
/// new node (the map's proxy, the node) to `out`, and returns `out`.
pub fn fn_00b3ac00(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    left_flag: u32,
    place: u32,
    pair: Ptr,
) -> Ptr {
    let map = this.addr();
    // `max > 0 ? max : 1`: the constant is never 0.
    let limit = MAP_MAX_COUNT.wrapping_sub(1);
    if limit <= e.mem.u32(map + 0x1c) {
        throw_with_text(
            e,
            MAP_TOO_LONG_TEXT,
            MAP_TOO_LONG_VTABLE,
            MAP_TOO_LONG_THROW_INFO,
        );
    }
    let head = map_head(e, map);
    let node = e
        .call(TREE_BUY_NODE, &args![this, head, place, head, pair, 0u32])
        .u32();
    let size = e.mem.u32(map + 0x1c).wrapping_add(1);
    e.mem.set_u32(map + 0x1c, size);
    let head = map_head(e, map);
    if place == head {
        e.mem.set_u32(head.wrapping_add(4), node);
        let head = map_head(e, map);
        e.mem.set_u32(head, node);
        let head = map_head(e, map);
        e.mem.set_u32(head.wrapping_add(8), node);
    } else if left_flag & 0xff != 0 {
        e.mem.set_u32(place, node);
        let head = map_head(e, map);
        if place == node_left(e, head) {
            e.mem.set_u32(head, node);
        }
    } else {
        e.mem.set_u32(place.wrapping_add(8), node);
        let head = map_head(e, map);
        if place == node_right(e, head) {
            e.mem.set_u32(head.wrapping_add(8), node);
        }
    }
    let mut x = node;
    while node_color(e, node_parent(e, x)) == 0 {
        let parent = node_parent(e, x);
        let grand = node_parent(e, parent);
        if parent == node_left(e, grand) {
            let uncle = node_right(e, grand);
            if node_color(e, uncle) == 0 {
                set_node_color(e, parent, NODE_BLACK);
                set_node_color(e, uncle, NODE_BLACK);
                set_node_color(e, grand, NODE_RED);
                x = grand;
            } else {
                if x == node_right(e, parent) {
                    x = parent;
                    e.call(TREE_ROTATE_LEFT, &args![this, x]);
                }
                let parent = node_parent(e, x);
                set_node_color(e, parent, NODE_BLACK);
                let grand = node_parent(e, parent);
                set_node_color(e, grand, NODE_RED);
                e.call(TREE_ROTATE_RIGHT, &args![this, grand]);
            }
        } else {
            let uncle = node_left(e, grand);
            if node_color(e, uncle) == 0 {
                set_node_color(e, parent, NODE_BLACK);
                set_node_color(e, uncle, NODE_BLACK);
                set_node_color(e, grand, NODE_RED);
                x = grand;
            } else {
                if x == node_left(e, parent) {
                    x = parent;
                    e.call(TREE_ROTATE_RIGHT, &args![this, x]);
                }
                let parent = node_parent(e, x);
                set_node_color(e, parent, NODE_BLACK);
                let grand = node_parent(e, parent);
                set_node_color(e, grand, NODE_RED);
                e.call(TREE_ROTATE_LEFT, &args![this, grand]);
            }
        }
    }
    let head = map_head(e, map);
    let root = node_parent(e, head);
    set_node_color(e, root, NODE_BLACK);
    e.mem.set_u32(out.addr(), 0);
    e.mem.set_u32(out.addr() + 4, node);
    if map == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let proxy = e.mem.u32(map);
    e.mem.set_u32(out.addr(), proxy);
    out
}

/// The unique insert (`00b3a9b0`) with its flag for equal keys. The exe's
/// code tests the constant 0 (`XOR EDX, EDX` / `JZ`) before the
/// equal-keys block, so `equal_keys` is always false there: a `map` never
/// holds equal keys, and the block is the `multimap` form of the template,
/// kept by the compiler's branch folding as unreachable code.
///
/// The exe's frame (`SUB ESP,0xb8` under `EBP`): 0xb8 bytes, the iterator `j` at +0xa4
/// (`EBP-0x14`), the begin iterator at +0x90 (`EBP-0x28`), and the `_Insert` results of the
/// three calls at +0x98, +0x84 and +0x78 (`EBP-0x20`, `EBP-0x34`, `EBP-0x40`).
fn insert_unique(e: &mut Engine, this: Ptr, out: Ptr, pair: Ptr, equal_keys: bool) -> Ptr {
    e.with_stack(0xb8, |e, frame| {
        insert_unique_in_frame(e, this, out, pair, equal_keys, frame.addr())
    })
}

/// [`insert_unique`] in the exe's frame (`frame` is its lowest address).
fn insert_unique_in_frame(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    pair: Ptr,
    equal_keys: bool,
    frame: u32,
) -> Ptr {
    let map = this.addr();
    let head = map_head(e, map);
    let mut x = node_parent(e, head);
    let mut y = head;
    let mut left = true;
    while !node_is_nil(e, x) {
        y = x;
        left = key_less(e, pair.addr(), x.wrapping_add(0xc));
        x = if left {
            node_left(e, x)
        } else {
            node_right(e, x)
        };
    }
    if equal_keys {
        // Unreachable (see above): always insert, with `true` as the flag.
        let inserted = insert_node(e, map, frame + 0x98, left as u32, y, pair.addr());
        let proxy = e.mem.u32(inserted);
        let node = e.mem.u32(inserted + 4);
        e.mem.set_u32(out.addr(), proxy);
        e.mem.set_u32(out.addr() + 4, node);
        e.mem.set_u8(out.addr() + 8, 1);
        return out;
    }
    let j = frame + 0xa4;
    e.call(MAP_ITERATOR_NEW, &args![j, y, this]);
    if left {
        let begin = frame + 0x90;
        let first = node_left(e, map_head(e, map));
        e.call(MAP_ITERATOR_NEW, &args![begin, first, this]);
        let j_proxy = e.mem.u32(j);
        let begin_proxy = e.mem.u32(begin);
        check_proxy(e, j_proxy, begin_proxy);
        if e.mem.u32(j + 4) == e.mem.u32(begin + 4) {
            let inserted = insert_node(e, map, frame + 0x84, 1, y, pair.addr());
            let proxy = e.mem.u32(inserted);
            let node = e.mem.u32(inserted + 4);
            e.mem.set_u32(out.addr(), proxy);
            e.mem.set_u32(out.addr() + 4, node);
            e.mem.set_u8(out.addr() + 8, 1);
            return out;
        }
        e.call(MAP_ITERATOR_DECREMENT, &args![j]);
    }
    let previous = e.mem.u32(j + 4);
    if key_less(e, previous.wrapping_add(0xc), pair.addr()) {
        let inserted = insert_node(e, map, frame + 0x78, left as u32, y, pair.addr());
        let proxy = e.mem.u32(inserted);
        let node = e.mem.u32(inserted + 4);
        e.mem.set_u32(out.addr(), proxy);
        e.mem.set_u32(out.addr() + 4, node);
        e.mem.set_u8(out.addr() + 8, 1);
        return out;
    }
    let proxy = e.mem.u32(j);
    let node = e.mem.u32(j + 4);
    e.mem.set_u32(out.addr(), proxy);
    e.mem.set_u32(out.addr() + 4, node);
    e.mem.set_u8(out.addr() + 8, 0);
    out
}

// Translated from 00b3a9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tree's unique `insert(value)` (identified from the code): finds the
/// leaf where the key belongs (left while the key compares below the node's
/// key) and writes a `pair<iterator, bool>` to `out` (the iterator's two
/// words, then the byte). When the leaf is on the left side and is the first
/// node, or when the node before the insertion point has a key below the new
/// key, the new node is inserted (`_Insert`) and the flag is 1; otherwise the
/// existing node with an equal key is returned with the flag 0. Returns `out`.
pub fn fn_00b3a9b0(e: &mut Engine, this: Ptr, out: Ptr, pair: Ptr) -> Ptr {
    insert_unique(e, this, out, pair, false)
}

/// The iterator locals of the hint insert in `00b398c0`'s frame: the hint (the argument
/// itself), the begin and end iterators, the iterator stepped back or forward, and the end
/// iterator compared with the step forward.
#[derive(Clone, Copy)]
struct HintSlots {
    hint: u32,
    begin: u32,
    end: u32,
    step: u32,
    last: u32,
}

/// The hint insert's body for equal keys (`multimap`): the block that the
/// exe's code skips with a constant-false test, translated as written. Returns
/// whether it inserted (the result is in `out`); false falls to the plain
/// insert. `slots` are its iterator locals.
fn hint_insert_equal_keys(
    e: &mut Engine,
    this: u32,
    out: u32,
    hint_proxy: u32,
    hint_node: u32,
    pair: u32,
    slots: HintSlots,
) -> bool {
    let head = map_head(e, this);
    let first = node_left(e, head);
    let begin = slots.begin;
    e.call(MAP_ITERATOR_NEW, &args![begin, first, this]);
    let begin_proxy = e.mem.u32(begin);
    check_proxy(e, hint_proxy, begin_proxy);
    if hint_node == e.mem.u32(begin + 4) {
        if key_less(e, hint_node.wrapping_add(0xc), pair) {
            return false;
        }
        insert_node(e, this, out, 1, hint_node, pair);
        return true;
    }
    let end = slots.end;
    let head = map_head(e, this);
    e.call(MAP_ITERATOR_NEW, &args![end, head, this]);
    let end_proxy = e.mem.u32(end);
    check_proxy(e, hint_proxy, end_proxy);
    if hint_node == e.mem.u32(end + 4) {
        let rightmost = node_right(e, map_head(e, this));
        if key_less(e, pair, rightmost.wrapping_add(0xc)) {
            return false;
        }
        let rightmost = node_right(e, map_head(e, this));
        insert_node(e, this, out, 0, rightmost, pair);
        return true;
    }
    if !key_less(e, hint_node.wrapping_add(0xc), pair) {
        let previous = slots.step;
        e.mem.set_u32(previous, hint_proxy);
        e.mem.set_u32(previous + 4, hint_node);
        e.call(MAP_ITERATOR_DECREMENT, &args![previous]);
        let previous_node = e.mem.u32(previous + 4);
        if !key_less(e, pair, previous_node.wrapping_add(0xc)) {
            if node_is_nil(e, node_right(e, previous_node)) {
                insert_node(e, this, out, 0, previous_node, pair);
            } else {
                insert_node(e, this, out, 1, hint_node, pair);
            }
            return true;
        }
    }
    if key_less(e, pair, hint_node.wrapping_add(0xc)) {
        return false;
    }
    let next = slots.step;
    e.mem.set_u32(next, hint_proxy);
    e.mem.set_u32(next + 4, hint_node);
    let end = slots.last;
    let head = map_head(e, this);
    e.call(MAP_ITERATOR_NEW, &args![end, head, this]);
    e.call(MAP_ITERATOR_INCREMENT, &args![next]);
    let next_proxy = e.mem.u32(next);
    let end_proxy = e.mem.u32(end);
    check_proxy(e, next_proxy, end_proxy);
    let next_node = e.mem.u32(next + 4);
    if next_node != e.mem.u32(end + 4) && key_less(e, next_node.wrapping_add(0xc), pair) {
        return false;
    }
    if node_is_nil(e, node_right(e, hint_node)) {
        insert_node(e, this, out, 0, hint_node, pair);
    } else {
        insert_node(e, this, out, 1, next_node, pair);
    }
    true
}

/// The hint insert's body for unique keys, the block the exe runs: tries the
/// cheap cases around the hint (before the first node, after the last node,
/// between the hint and its neighbour) and returns whether one inserted the
/// node (the result is in `out`); false falls to the plain insert.
fn hint_insert_unique(
    e: &mut Engine,
    this: u32,
    out: u32,
    hint_proxy: u32,
    hint_node: u32,
    pair: u32,
    slots: HintSlots,
) -> bool {
    let hint = slots.hint;
    e.mem.set_u32(hint, hint_proxy);
    e.mem.set_u32(hint + 4, hint_node);
    let begin = slots.begin;
    let first = node_left(e, map_head(e, this));
    e.call(MAP_ITERATOR_NEW, &args![begin, first, this]);
    if e.call(ITERATOR_EQUAL, &args![hint, begin]).bool() {
        if key_less(e, pair, hint_node.wrapping_add(0xc)) {
            insert_node(e, this, out, 1, hint_node, pair);
            return true;
        }
        return false;
    }
    let end = slots.end;
    let head = map_head(e, this);
    e.call(MAP_ITERATOR_NEW, &args![end, head, this]);
    if e.call(ITERATOR_EQUAL, &args![hint, end]).bool() {
        let rightmost = node_right(e, map_head(e, this));
        if key_less(e, rightmost.wrapping_add(0xc), pair) {
            let rightmost = node_right(e, map_head(e, this));
            insert_node(e, this, out, 0, rightmost, pair);
            return true;
        }
        return false;
    }
    if key_less(e, pair, hint_node.wrapping_add(0xc)) {
        let previous = slots.step;
        e.mem.set_u32(previous, hint_proxy);
        e.mem.set_u32(previous + 4, hint_node);
        e.call(MAP_ITERATOR_DECREMENT, &args![previous]);
        let previous_node = e.mem.u32(previous + 4);
        if key_less(e, previous_node.wrapping_add(0xc), pair) {
            if node_is_nil(e, node_right(e, previous_node)) {
                insert_node(e, this, out, 0, previous_node, pair);
            } else {
                insert_node(e, this, out, 1, hint_node, pair);
            }
            return true;
        }
    }
    if key_less(e, hint_node.wrapping_add(0xc), pair) {
        let next = slots.step;
        e.mem.set_u32(next, hint_proxy);
        e.mem.set_u32(next + 4, hint_node);
        let end = slots.last;
        let head = map_head(e, this);
        e.call(MAP_ITERATOR_NEW, &args![end, head, this]);
        e.call(MAP_ITERATOR_INCREMENT, &args![next]);
        let next_node = e.mem.u32(next + 4);
        if e.call(ITERATOR_EQUAL, &args![next, end]).bool()
            || key_less(e, pair, next_node.wrapping_add(0xc))
        {
            if node_is_nil(e, node_right(e, hint_node)) {
                insert_node(e, this, out, 0, hint_node, pair);
            } else {
                insert_node(e, this, out, 1, next_node, pair);
            }
            return true;
        }
    }
    false
}

/// The hint insert with its equal-keys flag. The exe's code tests the
/// constant 0 (`XOR EAX, EAX` / `JZ`) before the equal-keys block, so
/// `equal_keys` is always false there (see `fn_00b398c0`).
fn insert_with_hint(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    hint_proxy: u32,
    hint_node: u32,
    pair: Ptr,
    equal_keys: bool,
) -> Ptr {
    let map = this.addr();
    if e.mem.u32(map + 0x1c) == 0 {
        let head = map_head(e, map);
        insert_node(e, map, out.addr(), 1, head, pair.addr());
        return out;
    }
    // The exe's frame (`SUB ESP,0x1c0` under `EBP`): 0x1c0 bytes, then the saved `EBP`, the
    // return address and the arguments (`out` +0x1c8, the hint +0x1cc, `pair` +0x1d4). The
    // unique path's iterators: begin +0x198 (`EBP-0x28`), end +0x190 (`EBP-0x30`), the step
    // +0x1b8 (`EBP-8`), the end compared with it +0x188 (`EBP-0x38`); the equal-keys block's:
    // +0x1b0, +0x1a8, +0x1b8, +0x1a0 (`EBP-0x10`, `-0x18`, `-8`, `-0x20`). The plain insert's
    // result at +0x17c (`EBP-0x44`).
    e.with_stack(0x1d8, |e, frame| {
        let frame = frame.addr();
        let inserted = if equal_keys {
            let slots = HintSlots {
                hint: frame + 0x1cc,
                begin: frame + 0x1b0,
                end: frame + 0x1a8,
                step: frame + 0x1b8,
                last: frame + 0x1a0,
            };
            hint_insert_equal_keys(
                e,
                map,
                out.addr(),
                hint_proxy,
                hint_node,
                pair.addr(),
                slots,
            )
        } else {
            let slots = HintSlots {
                hint: frame + 0x1cc,
                begin: frame + 0x198,
                end: frame + 0x190,
                step: frame + 0x1b8,
                last: frame + 0x188,
            };
            hint_insert_unique(
                e,
                map,
                out.addr(),
                hint_proxy,
                hint_node,
                pair.addr(),
                slots,
            )
        };
        if !inserted {
            let plain = frame + 0x17c;
            let result = e.call(TREE_INSERT_UNIQUE, &args![this, plain, pair]).u32();
            let proxy = e.mem.u32(result);
            let node = e.mem.u32(result + 4);
            e.mem.set_u32(out.addr(), proxy);
            e.mem.set_u32(out.addr() + 4, node);
        }
    });
    out
}

// Translated from 00b398c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tree's `insert(hint, value)` (identified from the code): into an empty
/// tree the node becomes the root (`_Insert` left of the head). Otherwise the
/// hint is tried: the key before the first node goes left of it; the key
/// after the last node goes right of it; a key between the hint and the node
/// before or after it goes beside the hint (left or right child, depending on
/// whether the neighbour has a free right child); anything else falls to the
/// plain insert. Writes the new iterator to `out` and returns `out`. The
/// exe's block for equal keys sits behind a constant-false test and is never
/// run (`insert_with_hint` has it, flagged by its last argument).
pub fn fn_00b398c0(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    hint_proxy: u32,
    hint_node: u32,
    pair: Ptr,
) -> Ptr {
    // `XOR EAX, EAX` / `JZ 00b39cdf`: the equal-keys block is skipped.
    insert_with_hint(e, this, out, hint_proxy, hint_node, pair, false)
}

// Translated from 00b3bb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tree's `erase(first, last)` (identified from the code; iterators are
/// passed as their two words): erasing the whole range, from the first node to
/// the head, clears the map and returns the first node. Otherwise each node in
/// the range is erased in turn, advancing the iterator first. Writes the
/// iterator of the node after the range (the map's proxy, the node) to `out`
/// and returns `out`.
pub fn fn_00b3bb90(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    first_proxy: u32,
    first_node: u32,
    last_proxy: u32,
    last_node: u32,
) -> Ptr {
    let map = this.addr();
    let head = map_head(e, map);
    let begin = node_left(e, head);
    if map == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let own_proxy = e.mem.u32(map);
    if first_proxy == 0 || first_proxy != own_proxy {
        e.call(INVALID_PARAMETER, &args![]);
    }
    if first_node == begin {
        let head = map_head(e, map);
        if map == 0 {
            e.call(INVALID_PARAMETER, &args![]);
        }
        let own_proxy = e.mem.u32(map);
        if last_proxy == 0 || last_proxy != own_proxy {
            e.call(INVALID_PARAMETER, &args![]);
        }
        if last_node == head {
            e.call(MAP_CLEAR, &args![this]);
            let head = map_head(e, map);
            let leftmost = node_left(e, head);
            e.mem.set_u32(out.addr(), 0);
            e.mem.set_u32(out.addr() + 4, leftmost);
            if map == 0 {
                e.call(INVALID_PARAMETER, &args![]);
            }
            let proxy = e.mem.u32(map);
            e.mem.set_u32(out.addr(), proxy);
            return out;
        }
    }
    // The exe's frame (`SUB ESP,0x5c` under `EBP`): 0x5c bytes, then the saved `EBP`, the
    // return address and the arguments (`out` +0x64, `first` +0x68, `last` +0x70). The cursor
    // is the `first` argument itself (`EBP+0xc`, 00b3bd05); the erase result is at +0x3c
    // (`EBP-0x20`).
    let node = e.with_stack(0x78, |e, frame| {
        let frame = frame.addr();
        let cursor = frame + 0x68;
        e.mem.set_u32(cursor, first_proxy);
        e.mem.set_u32(cursor + 4, first_node);
        if first_proxy == 0 || first_proxy != last_proxy {
            e.call(INVALID_PARAMETER, &args![]);
        }
        while e.mem.u32(cursor + 4) != last_node {
            let saved_proxy = e.mem.u32(cursor);
            let saved_node = e.mem.u32(cursor + 4);
            e.call(MAP_ITERATOR_INCREMENT, &args![cursor]);
            let erased = frame + 0x3c;
            e.call(
                TREE_ERASE_ONE,
                &args![this, erased, saved_proxy, saved_node],
            );
            if e.mem.u32(cursor) == 0 || e.mem.u32(cursor) != last_proxy {
                e.call(INVALID_PARAMETER, &args![]);
            }
        }
        e.mem.u32(cursor + 4)
    });
    e.mem.set_u32(out.addr(), 0);
    e.mem.set_u32(out.addr() + 4, node);
    if map == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let proxy = e.mem.u32(map);
    e.mem.set_u32(out.addr(), proxy);
    out
}

// Translated from 00b3bd80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b3bd80 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// The tree's `erase(iterator)` (identified from the code; the iterator is
/// passed as its two words): erasing the head node throws; otherwise the node
/// is unlinked as in the standard red-black deletion (a node with two
/// children is replaced by its in-order successor), the leftmost and
/// rightmost links of the head are updated, a black node's removal is
/// repaired by recolouring and rotations, the key string is tidied, the node
/// freed and the size reduced. Writes the iterator of the next node (the
/// map's proxy, the successor) to `out` and returns `out`.
pub fn fn_00b3bd80(e: &mut Engine, this: Ptr, out: Ptr, iterator_proxy: u32, node: u32) -> Ptr {
    let map = this.addr();
    if node_is_nil(e, node) {
        throw_with_text(
            e,
            MAP_ITERATOR_TEXT,
            MAP_ITERATOR_VTABLE,
            MAP_ITERATOR_THROW_INFO,
        );
    }
    let erased = node;
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0xfc`, the cookie): 0x10c bytes,
    // then the saved `EBP`, the return address and the arguments; the iterator stepped is the
    // argument itself (`EBP+0xc`, +0x118, 00b3be0d).
    let successor = e.with_stack(0x120, |e, frame| {
        let cursor = frame.addr() + 0x118;
        e.mem.set_u32(cursor, iterator_proxy);
        e.mem.set_u32(cursor + 4, node);
        e.call(MAP_ITERATOR_INCREMENT, &args![cursor]);
        e.mem.u32(cursor + 4)
    });
    let (y, x) = if node_is_nil(e, node_left(e, erased)) {
        (erased, node_right(e, erased))
    } else if node_is_nil(e, node_right(e, erased)) {
        (erased, node_left(e, erased))
    } else {
        (successor, node_right(e, successor))
    };
    let mut parent = node_parent(e, erased);
    if y == erased {
        if !node_is_nil(e, x) {
            e.mem.set_u32(x.wrapping_add(4), parent);
        }
        let head = map_head(e, map);
        if node_parent(e, head) == erased {
            e.mem.set_u32(head.wrapping_add(4), x);
        } else if node_left(e, parent) == erased {
            e.mem.set_u32(parent, x);
        } else {
            e.mem.set_u32(parent.wrapping_add(8), x);
        }
        let head = map_head(e, map);
        if node_left(e, head) == erased {
            let leftmost = if node_is_nil(e, x) {
                parent
            } else {
                let mut walk = x;
                while !node_is_nil(e, node_left(e, walk)) {
                    walk = node_left(e, walk);
                }
                walk
            };
            let head = map_head(e, map);
            e.mem.set_u32(head, leftmost);
        }
        let head = map_head(e, map);
        if node_right(e, head) == erased {
            let rightmost = if node_is_nil(e, x) {
                parent
            } else {
                let mut walk = x;
                while !node_is_nil(e, node_right(e, walk)) {
                    walk = node_right(e, walk);
                }
                walk
            };
            let head = map_head(e, map);
            e.mem.set_u32(head.wrapping_add(8), rightmost);
        }
    } else {
        let left = node_left(e, erased);
        e.mem.set_u32(left.wrapping_add(4), y);
        e.mem.set_u32(y, left);
        if y == node_right(e, erased) {
            parent = y;
        } else {
            parent = node_parent(e, y);
            if !node_is_nil(e, x) {
                e.mem.set_u32(x.wrapping_add(4), parent);
            }
            e.mem.set_u32(parent, x);
            let right = node_right(e, erased);
            e.mem.set_u32(y.wrapping_add(8), right);
            e.mem.set_u32(right.wrapping_add(4), y);
        }
        let head = map_head(e, map);
        if node_parent(e, head) == erased {
            e.mem.set_u32(head.wrapping_add(4), y);
        } else {
            let above = node_parent(e, erased);
            if node_left(e, above) == erased {
                e.mem.set_u32(above, y);
            } else {
                e.mem.set_u32(above.wrapping_add(8), y);
            }
        }
        let above = node_parent(e, erased);
        e.mem.set_u32(y.wrapping_add(4), above);
        // std::swap of the two colour bytes (the addresses always differ).
        let y_color = e.mem.u8(y.wrapping_add(0x2c));
        let erased_color = e.mem.u8(erased.wrapping_add(0x2c));
        e.mem.set_u8(y.wrapping_add(0x2c), erased_color);
        e.mem.set_u8(erased.wrapping_add(0x2c), y_color);
    }
    let mut x = x;
    if node_color(e, erased) == i32::from(NODE_BLACK) {
        let mut first_pass = true;
        loop {
            if !first_pass {
                parent = node_parent(e, x);
            }
            first_pass = false;
            let head = map_head(e, map);
            if x == node_parent(e, head) || node_color(e, x) != i32::from(NODE_BLACK) {
                break;
            }
            if x == node_left(e, parent) {
                let mut sibling = node_right(e, parent);
                if node_color(e, sibling) == i32::from(NODE_RED) {
                    set_node_color(e, sibling, NODE_BLACK);
                    set_node_color(e, parent, NODE_RED);
                    e.call(TREE_ROTATE_LEFT, &args![this, parent]);
                    sibling = node_right(e, parent);
                }
                if node_is_nil(e, sibling) {
                    x = parent;
                } else if node_color(e, node_left(e, sibling)) == i32::from(NODE_BLACK)
                    && node_color(e, node_right(e, sibling)) == i32::from(NODE_BLACK)
                {
                    set_node_color(e, sibling, NODE_RED);
                    x = parent;
                } else {
                    if node_color(e, node_right(e, sibling)) == i32::from(NODE_BLACK) {
                        let inner = node_left(e, sibling);
                        set_node_color(e, inner, NODE_BLACK);
                        set_node_color(e, sibling, NODE_RED);
                        e.call(TREE_ROTATE_RIGHT, &args![this, sibling]);
                        sibling = node_right(e, parent);
                    }
                    let parent_color = e.mem.u8(parent.wrapping_add(0x2c));
                    set_node_color(e, sibling, parent_color);
                    set_node_color(e, parent, NODE_BLACK);
                    let outer = node_right(e, sibling);
                    set_node_color(e, outer, NODE_BLACK);
                    e.call(TREE_ROTATE_LEFT, &args![this, parent]);
                    break;
                }
            } else {
                let mut sibling = node_left(e, parent);
                if node_color(e, sibling) == i32::from(NODE_RED) {
                    set_node_color(e, sibling, NODE_BLACK);
                    set_node_color(e, parent, NODE_RED);
                    e.call(TREE_ROTATE_RIGHT, &args![this, parent]);
                    sibling = node_left(e, parent);
                }
                if node_is_nil(e, sibling) {
                    x = parent;
                } else if node_color(e, node_right(e, sibling)) == i32::from(NODE_BLACK)
                    && node_color(e, node_left(e, sibling)) == i32::from(NODE_BLACK)
                {
                    set_node_color(e, sibling, NODE_RED);
                    x = parent;
                } else {
                    if node_color(e, node_left(e, sibling)) == i32::from(NODE_BLACK) {
                        let inner = node_right(e, sibling);
                        set_node_color(e, inner, NODE_BLACK);
                        set_node_color(e, sibling, NODE_RED);
                        e.call(TREE_ROTATE_LEFT, &args![this, sibling]);
                        sibling = node_left(e, parent);
                    }
                    let parent_color = e.mem.u8(parent.wrapping_add(0x2c));
                    set_node_color(e, sibling, parent_color);
                    set_node_color(e, parent, NODE_BLACK);
                    let outer = node_left(e, sibling);
                    set_node_color(e, outer, NODE_BLACK);
                    e.call(TREE_ROTATE_RIGHT, &args![this, parent]);
                    break;
                }
            }
        }
        set_node_color(e, x, NODE_BLACK);
    }
    e.call(STRING_TIDY, &args![erased.wrapping_add(0xc), 1u32, 0u32]);
    if DESTRUCTOR_FREES {
        // The inlined string destructor's delete flag is the constant 0.
        e.call(OPERATOR_DELETE, &args![erased]);
    }
    e.call(OPERATOR_DELETE, &args![erased]);
    if e.mem.u32(map + 0x1c) > 0 {
        let size = e.mem.u32(map + 0x1c).wrapping_sub(1);
        e.mem.set_u32(map + 0x1c, size);
    }
    e.mem.set_u32(out.addr(), 0);
    e.mem.set_u32(out.addr() + 4, successor);
    if map == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let proxy = e.mem.u32(map);
    e.mem.set_u32(out.addr(), proxy);
    out
}

// Translated from 00b39210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b39210 - exception unwinding frame (FS:[0], /GS cookie) is not translated.
/// The map's `operator[](key)` (identified from the code): `lower_bound`,
/// with the iterator checked against the map's own proxy. When the node is the
/// head or the key compares below its key, a pair (the key, a null mapped
/// pointer) is inserted with the node as the hint and the new iterator
/// replaces it. Returns the address of the mapped pointer (node pair + 0x1c).
pub fn fn_00b39210(e: &mut Engine, this: Ptr, key: Ptr) -> u32 {
    let map = this.addr();
    let node = e.call(MAP_LOWER_BOUND, &args![this, key]).u32();
    // The exe's frame (`EBP` frame, exception record, `SUB ESP,0x7c`, the cookie): 0x8c bytes,
    // the iterator at +0x78 (`EBP-0x14`), the key pair at +0x50 (`EBP-0x3c`) and the insert
    // result at +0x3c (`EBP-0x50`).
    e.with_stack(0x8c, |e, frame| {
        let frame = frame.addr();
        let iterator = frame + 0x78;
        e.mem.set_u32(iterator, 0);
        e.mem.set_u32(iterator + 4, node);
        let head = map_head(e, map);
        if map == 0 {
            e.call(INVALID_PARAMETER, &args![]);
        }
        let proxy = e.mem.u32(map);
        e.mem.set_u32(iterator, proxy);
        if map == 0 {
            e.call(INVALID_PARAMETER, &args![]);
        }
        let end_proxy = e.mem.u32(map);
        check_proxy(e, proxy, end_proxy);
        if node == head || key_less(e, key.addr(), node.wrapping_add(0xc)) {
            let pair = frame + 0x50;
            e.mem.set_u32(pair + 0x1c, 0);
            e.call(STRING_TIDY, &args![pair, 0u32, 0u32]);
            let npos = e.global::<u32>(STRING_NPOS);
            e.call(STRING_ASSIGN, &args![pair, key, 0u32, npos]);
            e.mem.set_u32(pair + 0x1c, 0);
            let hint_proxy = e.mem.u32(iterator);
            let hint_node = e.mem.u32(iterator + 4);
            let out = frame + 0x3c;
            let result = e
                .call(MAP_INSERT, &args![this, out, hint_proxy, hint_node, pair])
                .u32();
            let new_proxy = e.mem.u32(result);
            let new_node = e.mem.u32(result + 4);
            e.mem.set_u32(iterator, new_proxy);
            e.mem.set_u32(iterator + 4, new_node);
            e.call(STRING_TIDY, &args![pair, 1u32, 0u32]);
        }
        let value = e.call(MAP_ITERATOR_VALUE, &args![iterator]).u32();
        value.wrapping_add(0x1c)
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b37660,
            cidv_bezier_spline_constructor(Ptr<CIdvBezierSpline>, Ptr) -> Ptr<CIdvBezierSpline>
        ),
        entry!(
            0x00b377c0,
            fn_00b377c0(Ptr<CIdvBezierSpline>, Ptr<CIdvBezierSpline>) -> Ptr<CIdvBezierSpline>
        ),
        entry!(
            0x00b37880,
            cidv_bezier_spline_destructor(Ptr<CIdvBezierSpline>)
        ),
        entry!(0x00b37a00, fn_00b37a00()),
        entry!(
            0x00b37ad0,
            cidv_bezier_spline_evaluate(Ptr<CIdvBezierSpline>, f32) -> f32
        ),
        entry!(
            0x00b37cc0,
            cidv_bezier_spline_parse(Ptr<CIdvBezierSpline>, Ptr)
        ),
        entry!(0x00b37f50, next_token(u32, u32) -> u32),
        entry!(
            0x00b37fa0,
            cidv_bezier_spline_scaled_variance(Ptr<CIdvBezierSpline>, f32) -> f32
        ),
        entry!(
            0x00b380e0,
            fn_00b380e0(Ptr<CIdvBezierSpline>, Ptr, Ptr, f32)
        ),
        entry!(0x00b38430, fn_00b38430(Ptr<CIdvBezierSpline>, u32)),
        entry!(
            0x00b38b50,
            fn_00b38b50(Ptr<CIdvBezierSpline>, Ptr, f32) -> Ptr
        ),
        entry!(0x00b38f40, fn_00b38f40(Ptr, Ptr, Ptr, Ptr, Ptr, Ptr, f32) -> Ptr),
        entry!(0x00b39090, cvec_vector_destructor(Ptr<StdVector>)),
        entry!(0x00b39100, cvec_vector_push_back(Ptr<StdVector>, Ptr)),
        entry!(0x00b39210, fn_00b39210(Ptr, Ptr) -> u32),
        entry!(0x00b393b0, fn_00b393b0(Ptr, Ptr) -> Ptr),
        entry!(0x00b39400, fn_00b39400(Ptr)),
        entry!(0x00b39460, fn_00b39460(Ptr)),
        entry!(0x00b39490, cvec_vector_resize(Ptr<StdVector>, u32, CVecArg)),
        entry!(
            0x00b396c0,
            cvec_vector_insert_one(Ptr<StdVector>, Ptr, u32, u32, Ptr) -> Ptr
        ),
        entry!(0x00b397f0, cvec_vector_tidy(Ptr<StdVector>)),
        entry!(0x00b398c0, fn_00b398c0(Ptr, Ptr, u32, u32, Ptr) -> Ptr),
        entry!(0x00b3a050, fn_00b3a050(Ptr, u32)),
        entry!(
            0x00b3a0d0,
            cvec_vector_insert_n(Ptr<StdVector>, u32, u32, u32, Ptr)
        ),
        entry!(0x00b3a9b0, fn_00b3a9b0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00b3ac00, fn_00b3ac00(Ptr, Ptr, u32, u32, Ptr) -> Ptr),
        entry!(0x00b3afe0, fn_00b3afe0(Ptr, Ptr) -> u32),
        entry!(0x00b3b050, fn_00b3b050(Ptr, u32, u32) -> Ptr),
        entry!(0x00b3b090, fn_00b3b090(Ptr) -> u32),
        entry!(0x00b3b100, fn_00b3b100(Ptr)),
        entry!(0x00b3b1d0, fn_00b3b1d0(Ptr, u32)),
        entry!(0x00b3b280, fn_00b3b280(Ptr, u32)),
        entry!(0x00b3b330, fn_00b3b330(Ptr, u32, u32, u32, Ptr, u32) -> u32),
        entry!(0x00b3b440, fn_00b3b440(Ptr)),
        entry!(0x00b3b540, fn_00b3b540(u32, u32) -> u32),
        entry!(0x00b3b5a0, fn_00b3b5a0(u32, u32, u32, u32) -> u32),
        entry!(0x00b3b5f0, cvec_uninit_fill_n(u32, u32, Ptr, u32, u32, u32)),
        entry!(0x00b3b6f0, fn_00b3b6f0(u32, u32, u32, u32) -> u32),
        entry!(0x00b3b770, fn_00b3b770(Ptr, Ptr, u32) -> Ptr),
        entry!(0x00b3b810, fn_00b3b810(Ptr)),
        entry!(0x00b3b880, fn_00b3b880(Ptr) -> u32),
        entry!(0x00b3b9d0, fn_00b3b9d0(Ptr, u32) -> u32),
        entry!(0x00b3b9f0, fn_00b3b9f0(Ptr, u32, u32) -> Ptr),
        entry!(0x00b3ba90, fn_00b3ba90(Ptr)),
        entry!(0x00b3bb90, fn_00b3bb90(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00b3bd80, fn_00b3bd80(Ptr, Ptr, u32, u32) -> Ptr),
        entry!(0x00b3c500, cvec_from_two(Ptr<CVec>, f32, f32) -> Ptr<CVec>),
        entry!(
            0x00b3c550,
            cvec_from_five(Ptr<CVec>, f32, f32, f32, f32, f32) -> Ptr<CVec>
        ),
        entry!(0x00b3c600, cvec_normalize(Ptr<CVec>)),
        entry!(
            0x00b3c660,
            cvec_add(Ptr<CVec>, Ptr<CVec>, Ptr<CVec>) -> Ptr<CVec>
        ),
        entry!(
            0x00b3c750,
            cvec_subtract(Ptr<CVec>, Ptr<CVec>, Ptr<CVec>) -> Ptr<CVec>
        ),
        entry!(
            0x00b3c840,
            cvec_scale(Ptr<CVec>, Ptr<CVec>, f32) -> Ptr<CVec>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// The invalid-parameter handler's call count.
    type Invalid = Rc<Cell<u32>>;

    /// A `std::vector` constructor: zeroed pointers and a container proxy
    /// pointing back at the vector.
    fn construct_vector(e: &mut Engine, a: &[u32]) -> Ret {
        let this = a[0];
        for i in 0..6 {
            e.mem.set_u32(this + 4 * i, 0);
        }
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(proxy, this);
        e.mem.set_u32(this, proxy);
        ret(this)
    }

    /// `vector<float>::_Tidy`.
    fn tidy_float_vector(e: &mut Engine, a: &[u32]) -> Ret {
        let first = e.mem.u32(a[0] + 0x0c);
        e.mem.free(first);
        for i in 3..6 {
            e.mem.set_u32(a[0] + 4 * i, 0);
        }
        Ret::default()
    }

    /// `vector<float>::push_back(const float&)`: reallocates every time.
    fn push_back_float(e: &mut Engine, a: &[u32]) -> Ret {
        let this = a[0];
        let value = e.mem.u32(a[1]);
        let first = e.mem.u32(this + 0x0c);
        let count = (e.mem.u32(this + 0x10) - first) / 4;
        let buffer = e.mem.alloc((count + 1) * 4);
        for i in 0..count {
            let word = e.mem.u32(first + 4 * i);
            e.mem.set_u32(buffer + 4 * i, word);
        }
        e.mem.set_u32(buffer + 4 * count, value);
        e.mem.free(first);
        e.mem.set_u32(this + 0x0c, buffer);
        e.mem.set_u32(this + 0x10, buffer + 4 * (count + 1));
        e.mem.set_u32(this + 0x14, buffer + 4 * (count + 1));
        Ret::default()
    }

    /// `CVec::CVec(int)`: five zeroed floats, the size clamped to 5.
    fn construct_sized_cvec(e: &mut Engine, a: &[u32]) -> Ret {
        for i in 0..5 {
            e.mem.set_u32(a[0] + 4 * i, 0);
        }
        let size = (a[1] as i32).min(5);
        e.mem.set_i32(a[0] + 0x14, size);
        ret(a[0])
    }

    /// `CVec::Length`: the square root of the sum of squares, as an `f32`.
    fn cvec_length(e: &mut Engine, a: &[u32]) -> Ret {
        let size = e.mem.i32(a[0] + 0x14);
        let mut sum = 0.0f64;
        for i in 0..size.max(0) as u32 {
            let x = e.mem.f32(a[0] + 4 * i) as f64;
            sum += x * x;
        }
        Ret {
            st0: (sum.sqrt() as f32) as f64,
            ..Ret::default()
        }
    }

    /// `vector<CVec>::erase(first, last)`: moves the tail down.
    fn erase_cvecs(e: &mut Engine, a: &[u32]) -> Ret {
        let (this, out, position, last) = (a[0], a[1], a[3], a[5]);
        let end = e.mem.u32(this + 0x10);
        let removed = last - position;
        let mut from = last;
        let mut to = position;
        while from != end {
            for i in 0..6 {
                let word = e.mem.u32(from + 4 * i);
                e.mem.set_u32(to + 4 * i, word);
            }
            from += 0x18;
            to += 0x18;
        }
        e.mem.set_u32(this + 0x10, end - removed);
        e.mem.set_u32(out, a[2]);
        e.mem.set_u32(out + 4, position);
        ret(out)
    }

    /// `_Uninit_copy(first, last, dest, ...)`: copies the bytes.
    fn uninit_copy(e: &mut Engine, a: &[u32]) -> Ret {
        let bytes = e.mem.bytes(a[0], a[1] - a[0]);
        e.mem.write(a[2], &bytes);
        ret(a[2] + (a[1] - a[0]))
    }

    /// `copy_backward(first, last, dest_last)`.
    fn copy_backward(e: &mut Engine, a: &[u32]) -> Ret {
        let bytes = e.mem.bytes(a[0], a[1] - a[0]);
        e.mem.write(a[2] - (a[1] - a[0]), &bytes);
        ret(a[2] - (a[1] - a[0]))
    }

    /// A `std::string` holding `text`: inline up to 15 characters, else on
    /// the heap (reserve and size as the library keeps them).
    fn put_string(e: &mut Engine, text: &str) -> u32 {
        let string = e.mem.alloc(0x1c);
        write_string(e, string, text);
        string
    }

    fn write_string(e: &mut Engine, string: u32, text: &str) {
        e.mem.set_u32(string + 0x14, text.len() as u32);
        if text.len() < 0x10 {
            e.mem.set_cstr(string + 4, text.as_bytes());
            e.mem.set_u32(string + 0x18, 0x0f);
        } else {
            let data = e.mem.alloc(text.len() as u32 + 1);
            e.mem.set_cstr(data, text.as_bytes());
            e.mem.set_u32(string + 4, data);
            e.mem.set_u32(string + 0x18, text.len() as u32);
        }
    }

    fn string_text(e: &Engine, string: u32) -> String {
        String::from_utf8(e.mem.cstr(string_data(e, string))).unwrap()
    }

    /// An engine with the library and CRT callees the unit uses, as plain
    /// doubles: allocation, `std::string`, the `CVec` and vector helpers,
    /// the iterator comparisons, `atof`, `sscanf`, the random generator.
    fn world() -> (Engine, Invalid) {
        let mut e = Engine::new();
        for page in [
            0x0101_1000,
            0x0101_7000,
            0x0101_9000,
            0x010a_3000,
            0x010a_5000,
            0x011f_8000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(STRING_NPOS, u32::MAX);
        e.set_global(SAMPLE_SCALE, 499.0f64);
        e.set_global(HALF, 0.5f64);
        e.set_global(SAMPLE_SPACING, 1.0f32 / 499.0);
        e.mem.set_cstr(BEZIER_SPLINE_TAG, b"BezierSpline");
        e.mem.set_cstr(FORMAT_STRING, b"%s");
        let invalid: Invalid = Rc::new(Cell::new(0));
        let count = invalid.clone();
        e.register_double(INVALID_PARAMETER, move |_, _| {
            count.set(count.get() + 1);
            Ret::default()
        });
        e.register_double(OPERATOR_NEW, |e, a| {
            let block = e.mem.alloc(a[0].max(4));
            ret(block)
        });
        e.register_double(OPERATOR_DELETE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register_double(PROXY_ALLOCATE, |e, _| {
            let block = e.mem.alloc(8);
            ret(block)
        });
        e.register_double(VECTOR_CVEC_CONSTRUCT, construct_vector);
        e.register_double(VECTOR_FLOAT_CONSTRUCT, construct_vector);
        e.register_double(VECTOR_FLOAT_TIDY, tidy_float_vector);
        e.register_double(VECTOR_FLOAT_PUSH_BACK, push_back_float);
        e.register_double(VECTOR_CVEC_CLEAR, |e, a| {
            let first = e.mem.u32(a[0] + 0x0c);
            e.mem.set_u32(a[0] + 0x10, first);
            Ret::default()
        });
        e.register_double(VECTOR_CVEC_ERASE, erase_cvecs);
        e.register_double(ITERATOR_ADVANCE, |e, a| {
            let node = e.mem.u32(a[0] + 4);
            e.mem
                .set_u32(a[0] + 4, node.wrapping_add(a[1].wrapping_mul(0x18)));
            Ret::default()
        });
        e.register_double(CVEC_SIZED, construct_sized_cvec);
        e.register_double(CVEC_DEFAULT, |e, a| {
            for i in 0..5 {
                e.mem.set_u32(a[0] + 4 * i, 0);
            }
            e.mem.set_i32(a[0] + 0x14, 3);
            ret(a[0])
        });
        e.register_double(CVEC_LENGTH, cvec_length);
        e.register_double(TRIVIAL_DESTRUCTOR, |_, _| Ret::default());
        e.register_double(VECTOR_ALLOCATE, |e, a| {
            let block = e.mem.alloc(a[0] * 0x18);
            ret(block)
        });
        e.register_double(UNINIT_COPY, uninit_copy);
        e.register_double(COPY_BACKWARD, copy_backward);
        e.register_double(FTOL, |_, a| {
            let value = f64::from_bits(a[0] as u64 | (a[1] as u64) << 32);
            ret(value as i32 as u32)
        });
        e.register_double(ATEXIT, |_, _| Ret::default());
        e.register_double(RANDOM_CONSTRUCTOR, |_, a| ret(a[0]));
        e.register_double(RANDOM_GET_UNIFORM, |_, _| Ret::default());
        e.register_double(ISSPACE, |_, a| ret(matches!(a[0], 9..=13 | 32) as u32));
        e.register_double(SSCANF, |e, a| {
            let text = e.mem.cstr(a[0]);
            let word: Vec<u8> = text
                .iter()
                .copied()
                .take_while(|c| !c.is_ascii_whitespace())
                .collect();
            e.mem.set_cstr(a[2], &word);
            ret(1)
        });
        e.register_double(STRLEN, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        e.register_double(STRCMP, |e, a| {
            ret(e.mem.cstr(a[0]).cmp(&e.mem.cstr(a[1])) as i32 as u32)
        });
        e.register_double(ATOF, |e, a| {
            let text = String::from_utf8(e.mem.cstr(a[0])).unwrap();
            Ret {
                st0: text.parse::<f64>().unwrap_or(0.0),
                ..Ret::default()
            }
        });
        e.register_double(ATOL, |e, a| {
            let text = String::from_utf8(e.mem.cstr(a[0])).unwrap();
            ret(text.parse::<i32>().unwrap_or(0) as u32)
        });
        e.register_double(STRING_FROM_C_STRING, |e, a| {
            let text = String::from_utf8(e.mem.cstr(a[1])).unwrap();
            write_string(e, a[0], &text);
            ret(a[0])
        });
        e.register_double(STRING_TIDY, |e, a| {
            for i in 0..7 {
                e.mem.set_u32(a[0] + 4 * i, 0);
            }
            Ret::default()
        });
        e.register_double(STRING_ASSIGN, |e, a| {
            for i in 0..7 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, word);
            }
            ret(a[0])
        });
        e.register_double(STRING_COMPARE, |e, a| {
            let left = string_text(e, a[0]);
            let right = string_text(e, a[1]);
            ret(left.cmp(&right) as i32 as u32)
        });
        e.register_double(ITERATOR_EQUAL, |e, a| {
            ret((e.mem.u32(a[0] + 4) == e.mem.u32(a[1] + 4)) as u32)
        });
        e.register_double(ITERATOR_NOT_EQUAL, |e, a| {
            ret((e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)) as u32)
        });
        e.register_double(RUNTIME_ERROR_FROM_STRING, |_, a| ret(a[0]));
        e.register_double(CXX_THROW_EXCEPTION, |_, a| {
            panic!("thrown {:#x}", a[1]);
        });
        (e, invalid)
    }

    /// A `CVec` with the given components (size = their number).
    fn cvec(e: &mut Engine, values: &[f32]) -> u32 {
        let block = e.mem.alloc(0x18);
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(block + 4 * i as u32, *value);
        }
        e.mem.set_i32(block + 0x14, values.len() as i32);
        block
    }

    fn cvec_components(e: &Engine, block: u32) -> (Vec<u32>, i32) {
        (
            (0..5).map(|i| e.mem.u32(block + 4 * i)).collect(),
            e.mem.i32(block + 0x14),
        )
    }

    fn bits(values: &[f32]) -> Vec<u32> {
        values.iter().map(|v| v.to_bits()).collect()
    }

    /// A `vector<CVec>` whose elements have the given first components, in a
    /// buffer with room for `capacity` elements.
    fn cvec_vector(e: &mut Engine, markers: &[f32], capacity: u32) -> u32 {
        let vector = e.mem.alloc(0x18);
        e.call(VECTOR_CVEC_CONSTRUCT, &args![vector]);
        if capacity > 0 {
            let buffer = e.mem.alloc(capacity * 0x18);
            for (i, marker) in markers.iter().enumerate() {
                let element = buffer + 0x18 * i as u32;
                e.mem.set_f32(element, *marker);
                e.mem.set_i32(element + 0x14, 2);
            }
            e.mem.set_u32(vector + 0x0c, buffer);
            e.mem
                .set_u32(vector + 0x10, buffer + 0x18 * markers.len() as u32);
            e.mem.set_u32(vector + 0x14, buffer + 0x18 * capacity);
        }
        vector
    }

    fn markers(e: &Engine, vector: u32) -> Vec<f32> {
        let first = e.mem.u32(vector + 0x0c);
        let count = (e.mem.u32(vector + 0x10) - first) / 0x18;
        (0..count).map(|i| e.mem.f32(first + 0x18 * i)).collect()
    }

    fn capacity(e: &Engine, vector: u32) -> u32 {
        let first = e.mem.u32(vector + 0x0c);
        (e.mem.u32(vector + 0x14) - first) / 0x18
    }

    #[test]
    fn the_layouts_match_the_pc_offsets() {
        assert_eq!(StdVector::SIZE, 0x18);
        assert_eq!(CVec::SIZE, 0x18);
        assert_eq!(CVec::m_nSize.off, 0x14);
        assert_eq!(CIdvBezierSpline::SIZE, 0x84);
        assert_eq!(CIdvBezierSpline::m_vControlPoints.off, 0x0c);
        assert_eq!(CIdvBezierSpline::m_vControlPointTangentLengths.off, 0x3c);
        assert_eq!(CIdvBezierSpline::m_vSplinePoints.off, 0x6c);
    }

    #[test]
    fn cvec_constructors_set_the_components_and_the_size() {
        let (mut e, _) = world();
        let this = e.mem.alloc(0x18);
        for i in 0..6 {
            e.mem.set_u32(this + 4 * i, 0xdead_beef);
        }
        // A signalling NaN is quieted by the load and store.
        let signalling = f32::from_bits(0x7f80_0001);
        let returned = e.call(0x00b3_c500, &args![this, signalling, 2.5f32]).u32();
        assert_eq!(returned, this);
        assert_eq!(
            cvec_components(&e, this),
            (vec![0x7fc0_0001, 2.5f32.to_bits(), 0, 0, 0], 2)
        );
        e.call(
            0x00b3_c550,
            &args![this, 1.0f32, 2.0f32, 3.0f32, 4.0f32, signalling],
        );
        assert_eq!(
            cvec_components(&e, this),
            (
                vec![
                    1.0f32.to_bits(),
                    2.0f32.to_bits(),
                    3.0f32.to_bits(),
                    4.0f32.to_bits(),
                    0x7fc0_0001
                ],
                5
            )
        );
    }

    #[test]
    fn normalize_divides_unless_the_length_is_zero() {
        let (mut e, _) = world();
        let vector = cvec(&mut e, &[3.0, 4.0]);
        e.call(0x00b3_c600, &args![vector]);
        assert_eq!(
            cvec_components(&e, vector).0[..2],
            bits(&[(3.0f64 / 5.0) as f32, (4.0f64 / 5.0) as f32])
        );
        let zero = cvec(&mut e, &[0.0, -0.0, 2.0]);
        // The length of the first three components is 2, but a zero vector
        // of size 2 has length 0 and stays as it is.
        let flat = cvec(&mut e, &[0.0, -0.0]);
        e.call(0x00b3_c600, &args![flat]);
        assert_eq!(cvec_components(&e, flat).0[..2], bits(&[0.0, -0.0]));
        e.call(0x00b3_c600, &args![zero]);
        assert_eq!(cvec_components(&e, zero).0[2], 1.0f32.to_bits());
        // A NaN length is not equal to 0: the components are divided.
        e.register_double(CVEC_LENGTH, |_, _| Ret {
            st0: f64::NAN,
            ..Ret::default()
        });
        let divided = cvec(&mut e, &[1.0]);
        e.call(0x00b3_c600, &args![divided]);
        assert!(e.mem.f32(divided).is_nan());
    }

    #[test]
    fn the_cvec_operators_use_the_smaller_size_and_this_size_components() {
        let (mut e, _) = world();
        let a = cvec(&mut e, &[1.0, 2.0, 3.0]);
        let b = cvec(&mut e, &[10.0, 20.0]);
        e.mem.set_f32(b + 8, 100.0);
        let out = e.mem.alloc(0x18);
        let returned = e.call(0x00b3_c660, &args![a, out, b]).u32();
        assert_eq!(returned, out);
        // The result has the smaller size, 2, but three components are
        // computed from the two operands (the third of `b` is stale memory).
        assert_eq!(
            cvec_components(&e, out),
            (bits(&[11.0, 22.0, 103.0, 0.0, 0.0]), 2)
        );
        e.call(0x00b3_c750, &args![a, out, b]);
        assert_eq!(
            cvec_components(&e, out),
            (bits(&[-9.0, -18.0, -97.0, 0.0, 0.0]), 2)
        );
        e.call(0x00b3_c840, &args![a, out, 0.1f32]);
        let scaled: Vec<f32> = [1.0f64, 2.0, 3.0]
            .iter()
            .map(|v| (v * 0.1f32 as f64) as f32)
            .collect();
        assert_eq!(
            cvec_components(&e, out),
            (bits(&[scaled[0], scaled[1], scaled[2], 0.0, 0.0]), 3)
        );
    }

    #[test]
    fn push_back_copies_into_spare_room_or_inserts() {
        let (mut e, invalid) = world();
        let vector = cvec_vector(&mut e, &[1.0], 2);
        let value = cvec(&mut e, &[2.0, 0.5]);
        let fills = Rc::new(RefCell::new(Vec::new()));
        let log = fills.clone();
        e.register_double(VECTOR_CVEC_FILL, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00b3_9100, &args![vector, value]);
        let end = e.mem.u32(vector + 0x0c) + 0x18;
        assert_eq!(*fills.borrow(), vec![vec![end, 1, value, vector + 8, 0, 0]]);
        assert_eq!(e.mem.u32(vector + 0x10), end + 0x18);
        assert_eq!(invalid.get(), 0);
        // Without spare room it is an insert at the end: the buffer grows
        // (a fresh engine, with the unit's own fill).
        let (mut e, invalid) = world();
        let full = cvec_vector(&mut e, &[], 0);
        for marker in [5.0f32, 6.0, 7.0] {
            let value = cvec(&mut e, &[marker]);
            e.call(0x00b3_9100, &args![full, value]);
        }
        assert_eq!(markers(&e, full), [5.0, 6.0, 7.0]);
        // 0 -> 1 -> 2 -> 3 elements of capacity.
        assert_eq!(capacity(&e, full), 3);
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn insert_n_shifts_the_tail_when_there_is_room() {
        let (mut e, invalid) = world();
        // The tail is at least as long as the count.
        let vector = cvec_vector(&mut e, &[1.0, 2.0, 3.0], 6);
        let value = cvec(&mut e, &[9.0]);
        let first = e.mem.u32(vector + 0x0c);
        e.call(0x00b3_a0d0, &args![vector, 0u32, first + 0x18, 2u32, value]);
        assert_eq!(markers(&e, vector), [1.0, 9.0, 9.0, 2.0, 3.0]);
        assert_eq!(e.mem.u32(vector + 0x0c), first);
        assert_eq!(capacity(&e, vector), 6);
        // The tail is shorter than the count.
        let vector = cvec_vector(&mut e, &[1.0, 2.0], 8);
        let first = e.mem.u32(vector + 0x0c);
        e.call(0x00b3_a0d0, &args![vector, 0u32, first + 0x18, 3u32, value]);
        assert_eq!(markers(&e, vector), [1.0, 9.0, 9.0, 9.0, 2.0]);
        // Inserting at the end.
        let vector = cvec_vector(&mut e, &[1.0], 4);
        let last = e.mem.u32(vector + 0x10);
        e.call(0x00b3_a0d0, &args![vector, 0u32, last, 2u32, value]);
        assert_eq!(markers(&e, vector), [1.0, 9.0, 9.0]);
        // A count of 0 does nothing.
        e.call(0x00b3_a0d0, &args![vector, 0u32, last, 0u32, value]);
        assert_eq!(markers(&e, vector), [1.0, 9.0, 9.0]);
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn insert_n_copies_the_value_before_moving_the_tail() {
        let (mut e, _) = world();
        // The value is the last element, which the shift overwrites.
        let vector = cvec_vector(&mut e, &[1.0, 2.0, 3.0], 5);
        let first = e.mem.u32(vector + 0x0c);
        e.call(0x00b3_a0d0, &args![vector, 0u32, first, 1u32, first + 0x30]);
        assert_eq!(markers(&e, vector), [3.0, 1.0, 2.0, 3.0]);
        // And the second branch: the value is an element of the gap.
        let vector = cvec_vector(&mut e, &[1.0, 2.0], 6);
        let first = e.mem.u32(vector + 0x0c);
        e.call(0x00b3_a0d0, &args![vector, 0u32, first, 3u32, first + 0x18]);
        assert_eq!(markers(&e, vector), [2.0, 2.0, 2.0, 1.0, 2.0]);
    }

    #[test]
    fn insert_n_reallocates_with_the_growth_rule() {
        let (mut e, _) = world();
        let value = cvec(&mut e, &[9.0]);
        // 2 elements of capacity 2: needs 4; 1.5 * 2 = 3 is not enough.
        let vector = cvec_vector(&mut e, &[1.0, 2.0], 2);
        let old = e.mem.u32(vector + 0x0c);
        let freed = Rc::new(RefCell::new(Vec::new()));
        let log = freed.clone();
        e.register_double(OPERATOR_DELETE, move |e, a| {
            log.borrow_mut().push(a[0]);
            e.mem.free(a[0]);
            Ret::default()
        });
        e.call(0x00b3_a0d0, &args![vector, 0u32, old + 0x18, 2u32, value]);
        assert_eq!(markers(&e, vector), [1.0, 9.0, 9.0, 2.0]);
        assert_eq!(capacity(&e, vector), 4);
        assert_eq!(*freed.borrow(), vec![old]);
        // Capacity 4 with 4 elements, one more: 1.5 * 4 = 6.
        let one = [9.0f32];
        let at = e.mem.u32(vector + 0x10);
        let value = cvec(&mut e, &one);
        e.call(0x00b3_a0d0, &args![vector, 0u32, at, 1u32, value]);
        assert_eq!(markers(&e, vector), [1.0, 9.0, 9.0, 2.0, 9.0]);
        assert_eq!(capacity(&e, vector), 6);
        // An empty vector: capacity 0 grows to exactly the count.
        let empty = cvec_vector(&mut e, &[], 0);
        e.call(0x00b3_a0d0, &args![empty, 0u32, 0u32, 3u32, value]);
        assert_eq!(markers(&e, empty), [9.0, 9.0, 9.0]);
        assert_eq!(capacity(&e, empty), 3);
    }

    #[test]
    fn insert_n_reports_a_count_beyond_the_maximum() {
        let (mut e, _) = world();
        let vector = cvec_vector(&mut e, &[1.0], 1);
        let value = cvec(&mut e, &[9.0]);
        let errors = Rc::new(Cell::new(0));
        let count = errors.clone();
        e.register_double(VECTOR_LENGTH_ERROR, move |_, _| {
            count.set(count.get() + 1);
            Ret::default()
        });
        let first = e.mem.u32(vector + 0x0c);
        e.call(
            0x00b3_a0d0,
            &args![vector, 0u32, first, 0x0aaa_aaaau32, value],
        );
        assert_eq!(errors.get(), 1);
        assert_eq!(markers(&e, vector), [1.0]);
    }

    #[test]
    fn the_range_copies_and_the_fill_forward_their_arguments() {
        let (mut e, _) = world();
        let calls = Rc::new(RefCell::new(Vec::new()));
        let log = calls.clone();
        e.register_double(UNINIT_COPY, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            ret(0x1234)
        });
        assert_eq!(
            e.call(0x00b3_b5a0, &args![1u32, 2u32, 3u32, 4u32]).u32(),
            0x1234
        );
        assert_eq!(
            e.call(0x00b3_b6f0, &args![5u32, 6u32, 7u32, 8u32]).u32(),
            0x1234
        );
        assert_eq!(
            *calls.borrow(),
            vec![vec![1, 2, 3, 4, 0, 0], vec![5, 6, 7, 8, 0, 0]]
        );
        // The fill copies the six words to each slot, skipping a null one.
        let value = cvec(&mut e, &[4.0, 5.0]);
        let block = e.mem.alloc(0x48);
        e.call(0x00b3_b5f0, &args![block, 3u32, value, 0u32, 0u32, 0u32]);
        for i in 0..3 {
            assert_eq!(
                cvec_components(&e, block + 0x18 * i),
                (bits(&[4.0, 5.0, 0.0, 0.0, 0.0]), 2)
            );
        }
        e.call(0x00b3_b5f0, &args![0u32, 1u32, value, 0u32, 0u32, 0u32]);
    }

    #[test]
    fn tidy_and_destructor_free_the_buffer_and_the_proxy() {
        let (mut e, _) = world();
        let freed = Rc::new(RefCell::new(Vec::new()));
        let log = freed.clone();
        e.register_double(OPERATOR_DELETE, move |e, a| {
            log.borrow_mut().push(a[0]);
            e.mem.free(a[0]);
            Ret::default()
        });
        let destroyed = Rc::new(RefCell::new(Vec::new()));
        let log = destroyed.clone();
        e.register_double(TRIVIAL_DESTRUCTOR, move |_, a| {
            log.borrow_mut().push(a[0]);
            Ret::default()
        });
        let vector = cvec_vector(&mut e, &[1.0, 2.0], 3);
        let buffer = e.mem.u32(vector + 0x0c);
        let proxy = e.mem.u32(vector);
        e.call(0x00b3_97f0, &args![vector]);
        assert_eq!(*destroyed.borrow(), vec![buffer, buffer + 0x18]);
        assert_eq!(*freed.borrow(), vec![buffer]);
        assert_eq!(e.mem.u32(vector + 0x0c), 0);
        assert_eq!(e.mem.u32(vector + 0x10), 0);
        assert_eq!(e.mem.u32(vector + 0x14), 0);
        // An empty vector has nothing to free; the destructor frees the proxy.
        e.call(0x00b3_9090, &args![vector]);
        assert_eq!(*freed.borrow(), vec![buffer, proxy]);
    }

    #[test]
    fn resize_grows_with_copies_and_shrinks_by_erasing() {
        let (mut e, invalid) = world();
        let vector = cvec_vector(&mut e, &[1.0, 2.0], 8);
        let fill = CVecArg([7.0f32.to_bits(), 0, 0, 0, 0, 2]);
        e.call(0x00b3_9490, &args![vector, 5u32, fill]);
        assert_eq!(markers(&e, vector), [1.0, 2.0, 7.0, 7.0, 7.0]);
        e.call(0x00b3_9490, &args![vector, 3u32, fill]);
        assert_eq!(markers(&e, vector), [1.0, 2.0, 7.0]);
        e.call(0x00b3_9490, &args![vector, 3u32, fill]);
        assert_eq!(markers(&e, vector), [1.0, 2.0, 7.0]);
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn insert_one_returns_an_iterator_at_the_same_offset() {
        let (mut e, invalid) = world();
        let vector = cvec_vector(&mut e, &[1.0, 2.0, 3.0], 3);
        let value = cvec(&mut e, &[9.0]);
        let proxy = e.mem.u32(vector);
        let first = e.mem.u32(vector + 0x0c);
        let out = e.mem.alloc(8);
        let returned = e
            .call(0x00b3_96c0, &args![vector, out, proxy, first + 0x18, value])
            .u32();
        assert_eq!(returned, out);
        assert_eq!(markers(&e, vector), [1.0, 9.0, 2.0, 3.0]);
        // The vector was reallocated; the iterator is at index 1 of the new
        // buffer and carries the vector's proxy.
        let new_first = e.mem.u32(vector + 0x0c);
        assert_ne!(new_first, first);
        assert_eq!(e.mem.u32(out), proxy);
        assert_eq!(e.mem.u32(out + 4), new_first + 0x18);
        // Into an empty vector: offset 0.
        let empty = cvec_vector(&mut e, &[], 0);
        let proxy = e.mem.u32(empty);
        e.call(0x00b3_96c0, &args![empty, out, proxy, 0u32, value]);
        assert_eq!(markers(&e, empty), [9.0]);
        assert_eq!(e.mem.u32(out + 4), e.mem.u32(empty + 0x0c));
        assert_eq!(invalid.get(), 0);
    }

    // ---- the map ----

    /// A map object built by its constructor.
    fn new_map(e: &mut Engine) -> u32 {
        let map = e.mem.alloc(0x20);
        let comparator = e.mem.alloc(4);
        let returned = e.call(0x00b3_b770, &args![map, comparator, 0u32]).u32();
        assert_eq!(returned, map);
        map
    }

    fn map_head_of(e: &Engine, map: u32) -> u32 {
        e.mem.u32(map + 0x18)
    }

    /// The in-order keys of the tree, checking the red-black rules: a black
    /// root, black head, no red node with a red child, the same number of
    /// black nodes on every path, consistent parent links, and the head's
    /// leftmost and rightmost links.
    fn check_tree(e: &Engine, map: u32) -> Vec<String> {
        let head = map_head_of(e, map);
        assert_eq!(e.mem.u8(head + 0x2d), 1, "the head is nil");
        assert_eq!(e.mem.u8(head + 0x2c), 1, "the head is black");
        let root = e.mem.u32(head + 4);
        let mut keys = Vec::new();
        if root == head {
            assert_eq!(e.mem.u32(head), head);
            assert_eq!(e.mem.u32(head + 8), head);
        } else {
            assert_eq!(e.mem.u8(root + 0x2c), 1, "the root is black");
            assert_eq!(e.mem.u32(root + 4), head);
            black_height(e, root, head, &mut keys);
            let first = string_text(e, e.mem.u32(head) + 0xc);
            let last = string_text(e, e.mem.u32(head + 8) + 0xc);
            assert_eq!(first, keys[0]);
            assert_eq!(last, *keys.last().unwrap());
        }
        assert_eq!(e.mem.u32(map + 0x1c) as usize, keys.len());
        let mut sorted = keys.clone();
        sorted.sort();
        assert_eq!(keys, sorted);
        keys
    }

    fn black_height(e: &Engine, node: u32, head: u32, keys: &mut Vec<String>) -> u32 {
        if node == head {
            return 1;
        }
        let color = e.mem.u8(node + 0x2c);
        assert!(color <= 1);
        assert_eq!(e.mem.u8(node + 0x2d), 0);
        let (left, right) = (e.mem.u32(node), e.mem.u32(node + 8));
        for child in [left, right] {
            if child != head {
                assert_eq!(e.mem.u32(child + 4), node, "parent link");
                if color == 0 {
                    assert_eq!(e.mem.u8(child + 0x2c), 1, "red node with a red child");
                }
            }
        }
        let low = black_height(e, left, head, keys);
        keys.push(string_text(e, node + 0xc));
        let high = black_height(e, right, head, keys);
        assert_eq!(low, high, "black height");
        low + color as u32
    }

    fn key(i: u32) -> String {
        format!("k{i:03}")
    }

    /// `map[key]`: the address of the mapped pointer.
    fn index(e: &mut Engine, map: u32, name: &str) -> u32 {
        let text = put_string(e, name);
        let slot = e.call(0x00b3_9210, &args![map, text]).u32();
        e.mem.free(text);
        slot
    }

    fn find(e: &mut Engine, map: u32, name: &str) -> u32 {
        let text = put_string(e, name);
        let node = e.call(0x00b3_afe0, &args![map, text]).u32();
        e.mem.free(text);
        node
    }

    /// A key and a mapped value as the insert functions take them.
    fn put_pair(e: &mut Engine, name: &str, value: u32) -> u32 {
        let pair = e.mem.alloc(0x20);
        write_string(e, pair, name);
        e.mem.set_u32(pair + 0x1c, value);
        pair
    }

    /// A permutation of `0..41`.
    fn scrambled() -> Vec<u32> {
        (0..41).map(|i| (i * 17 + 5) % 41).collect()
    }

    #[test]
    fn the_constructor_builds_an_empty_tree_with_a_head_node() {
        let (mut e, invalid) = world();
        let map = new_map(&mut e);
        let proxy = e.mem.u32(map);
        assert_eq!(e.mem.u32(proxy), map);
        let head = map_head_of(&e, map);
        for offset in [0, 4, 8] {
            assert_eq!(e.mem.u32(head + offset), head);
        }
        assert_eq!(check_tree(&e, map), Vec::<String>::new());
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn the_proxy_stays_null_when_its_allocation_fails() {
        let (mut e, _) = world();
        e.register_double(PROXY_ALLOCATE, |_, _| Ret::default());
        let map = e.mem.alloc(0x20);
        e.mem.set_u32(map, 0x1234);
        assert_eq!(e.call(0x00b3_b9f0, &args![map, 0u32, 0u32]).u32(), map);
        assert_eq!(e.mem.u32(map), 0);
    }

    #[test]
    fn operator_index_inserts_and_finds_in_sorted_order() {
        let (mut e, invalid) = world();
        let map = new_map(&mut e);
        let mut slots = Vec::new();
        for (n, i) in scrambled().into_iter().enumerate() {
            let slot = index(&mut e, map, &key(i));
            assert_eq!(e.mem.u32(slot), 0, "a new mapped pointer is null");
            e.mem.set_u32(slot, 1000 + i);
            slots.push((i, slot));
            let keys = check_tree(&e, map);
            assert_eq!(keys.len(), n + 1);
        }
        let keys = check_tree(&e, map);
        assert_eq!(keys, (0..41).map(key).collect::<Vec<_>>());
        for (i, slot) in slots {
            assert_eq!(index(&mut e, map, &key(i)), slot);
            assert_eq!(e.mem.u32(slot), 1000 + i);
        }
        assert_eq!(e.mem.u32(map + 0x1c), 41);
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn ascending_and_descending_inserts_stay_balanced() {
        for descending in [false, true] {
            let (mut e, invalid) = world();
            let map = new_map(&mut e);
            for n in 0..30 {
                let i = if descending { 29 - n } else { n };
                index(&mut e, map, &key(i));
                check_tree(&e, map);
            }
            assert_eq!(check_tree(&e, map).len(), 30);
            assert_eq!(invalid.get(), 0);
        }
    }

    /// Erases `name` through the tree's `erase(iterator)`; returns the
    /// iterator it answers.
    fn erase_key(e: &mut Engine, map: u32, name: &str) -> (u32, u32) {
        let node = find(e, map, name);
        assert_ne!(node, map_head_of(e, map), "{name} is in the tree");
        let proxy = e.mem.u32(map);
        let out = e.mem.alloc(8);
        let returned = e.call(0x00b3_bd80, &args![map, out, proxy, node]).u32();
        assert_eq!(returned, out);
        let result = (e.mem.u32(out), e.mem.u32(out + 4));
        e.mem.free(out);
        result
    }

    #[test]
    fn erasing_one_node_at_a_time_keeps_the_tree_valid() {
        let (mut e, invalid) = world();
        let map = new_map(&mut e);
        for i in scrambled() {
            index(&mut e, map, &key(i));
        }
        let freed = Rc::new(RefCell::new(Vec::new()));
        let log = freed.clone();
        e.register_double(OPERATOR_DELETE, move |e, a| {
            log.borrow_mut().push(a[0]);
            e.mem.free(a[0]);
            Ret::default()
        });
        let proxy = e.mem.u32(map);
        let mut remaining: Vec<u32> = (0..41).collect();
        for (n, i) in scrambled().into_iter().rev().enumerate() {
            let victim = find(&mut e, map, &key(i));
            let (iterator_proxy, successor) = erase_key(&mut e, map, &key(i));
            remaining.retain(|&r| r != i);
            let keys = check_tree(&e, map);
            assert_eq!(keys, remaining.iter().map(|&r| key(r)).collect::<Vec<_>>());
            assert_eq!(iterator_proxy, proxy);
            // The answer is the next key, or the head after the last one.
            let next = remaining.iter().find(|&&r| r > i);
            match next {
                Some(&r) => assert_eq!(string_text(&e, successor + 0xc), key(r)),
                None => assert_eq!(successor, map_head_of(&e, map)),
            }
            assert_eq!(freed.borrow().last(), Some(&victim));
            assert_eq!(freed.borrow().len(), n + 1);
        }
        assert_eq!(e.mem.u32(map + 0x1c), 0);
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn erase_removes_the_two_child_and_one_child_cases() {
        let (mut e, invalid) = world();
        let map = new_map(&mut e);
        for i in 0..15 {
            index(&mut e, map, &key(i));
        }
        // Whatever the shape, erase the middle keys first, then the ends.
        for i in [7, 3, 11, 0, 14, 5, 9, 1, 13, 2, 4, 6, 8, 10, 12] {
            erase_key(&mut e, map, &key(i));
            check_tree(&e, map);
        }
        assert_eq!(check_tree(&e, map), Vec::<String>::new());
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn iterators_walk_forward_and_backward() {
        let (mut e, invalid) = world();
        let map = new_map(&mut e);
        for i in scrambled().into_iter().take(12) {
            index(&mut e, map, &key(i));
        }
        let expected = check_tree(&e, map);
        let head = map_head_of(&e, map);
        let proxy = e.mem.u32(map);
        let iterator = e.mem.alloc(8);
        assert_eq!(e.call(0x00b3_93b0, &args![map, iterator]).u32(), iterator);
        assert_eq!(e.mem.u32(iterator), proxy);
        let mut seen = Vec::new();
        while e.mem.u32(iterator + 4) != head {
            let pair = e.call(0x00b3_b090, &args![iterator]).u32();
            seen.push(string_text(&e, pair));
            e.call(0x00b3_b100, &args![iterator]);
        }
        assert_eq!(seen, expected);
        assert_eq!(invalid.get(), 0);
        // Backward from the head (the rightmost node first).
        let mut backward = Vec::new();
        e.call(0x00b3_b440, &args![iterator]);
        loop {
            let pair = e.call(0x00b3_b090, &args![iterator]).u32();
            backward.push(string_text(&e, pair));
            if e.mem.u32(iterator + 4) == e.mem.u32(head) {
                break;
            }
            e.call(0x00b3_b440, &args![iterator]);
        }
        backward.reverse();
        assert_eq!(backward, expected);
        assert_eq!(invalid.get(), 0);
        // Incrementing the head node, or dereferencing it, is reported and
        // leaves the iterator where it was.
        e.mem.set_u32(iterator + 4, head);
        e.call(0x00b3_b100, &args![iterator]);
        assert_eq!(invalid.get(), 1);
        assert_eq!(e.mem.u32(iterator + 4), head);
        e.call(0x00b3_b090, &args![iterator]);
        assert_eq!(invalid.get(), 2);
        // A null proxy is reported by every iterator operation.
        e.mem.set_u32(iterator, 0);
        let first = e.mem.u32(head);
        e.mem.set_u32(iterator + 4, first);
        e.call(0x00b3_b100, &args![iterator]);
        e.call(0x00b3_b440, &args![iterator]);
        assert!(invalid.get() >= 4);
    }

    #[test]
    fn the_iterator_constructor_takes_the_containers_proxy() {
        let (mut e, invalid) = world();
        let map = new_map(&mut e);
        let iterator = e.mem.alloc(8);
        e.call(0x00b3_b050, &args![iterator, 0x1234u32, map]);
        assert_eq!(e.mem.u32(iterator), e.mem.u32(map));
        assert_eq!(e.mem.u32(iterator + 4), 0x1234);
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn lower_bound_finds_the_first_node_not_below_the_key() {
        let (mut e, _) = world();
        let map = new_map(&mut e);
        for i in (0..20).step_by(2) {
            index(&mut e, map, &key(i));
        }
        let head = map_head_of(&e, map);
        for i in 0..19 {
            let node = find(&mut e, map, &key(i));
            assert_ne!(node, head);
            assert_eq!(string_text(&e, node + 0xc), key(i + (i & 1)));
        }
        assert_eq!(find(&mut e, map, &key(19)), head);
        assert_eq!(find(&mut e, map, "zzz"), head);
        let empty = new_map(&mut e);
        assert_eq!(find(&mut e, empty, "a"), map_head_of(&e, empty));
    }

    #[test]
    fn erase_range_removes_the_nodes_between_two_iterators() {
        let (mut e, invalid) = world();
        let map = new_map(&mut e);
        for i in scrambled() {
            index(&mut e, map, &key(i));
        }
        let proxy = e.mem.u32(map);
        let (first, last) = (find(&mut e, map, &key(10)), find(&mut e, map, &key(25)));
        let out = e.mem.alloc(8);
        let returned = e
            .call(0x00b3_bb90, &args![map, out, proxy, first, proxy, last])
            .u32();
        assert_eq!(returned, out);
        assert_eq!(e.mem.u32(out), proxy);
        assert_eq!(e.mem.u32(out + 4), last);
        let expected: Vec<String> = (0..41).filter(|i| !(10..25).contains(i)).map(key).collect();
        assert_eq!(check_tree(&e, map), expected);
        // An empty range erases nothing.
        e.call(0x00b3_bb90, &args![map, out, proxy, last, proxy, last]);
        assert_eq!(check_tree(&e, map), expected);
        assert_eq!(e.mem.u32(out + 4), last);
        // The whole tree: the map is cleared and the answer is the head.
        let head = map_head_of(&e, map);
        let begin = e.mem.u32(head);
        e.call(0x00b3_bb90, &args![map, out, proxy, begin, proxy, head]);
        assert_eq!(check_tree(&e, map), Vec::<String>::new());
        assert_eq!(e.mem.u32(out + 4), head);
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn a_mismatched_iterator_proxy_is_reported() {
        let (mut e, invalid) = world();
        let map = new_map(&mut e);
        index(&mut e, map, "a");
        let head = map_head_of(&e, map);
        let out = e.mem.alloc(8);
        let begin = e.mem.u32(head);
        e.call(0x00b3_bb90, &args![map, out, 0u32, begin, 0u32, head]);
        assert_eq!(invalid.get(), 2);
    }

    #[test]
    fn clear_and_tidy_free_every_node() {
        let (mut e, invalid) = world();
        let map = new_map(&mut e);
        for i in 0..9 {
            index(&mut e, map, &key(i));
        }
        let freed = Rc::new(RefCell::new(Vec::new()));
        let log = freed.clone();
        e.register_double(OPERATOR_DELETE, move |e, a| {
            log.borrow_mut().push(a[0]);
            e.mem.free(a[0]);
            Ret::default()
        });
        let head = map_head_of(&e, map);
        e.call(0x00b3_9400, &args![map]);
        assert_eq!(freed.borrow().len(), 9);
        assert_eq!(check_tree(&e, map), Vec::<String>::new());
        assert_eq!(map_head_of(&e, map), head);
        // It still works afterwards; tidy frees the nodes and the head.
        for i in 0..5 {
            index(&mut e, map, &key(i));
        }
        freed.borrow_mut().clear();
        e.call(0x00b3_ba90, &args![map]);
        assert_eq!(freed.borrow().len(), 6);
        assert_eq!(freed.borrow().last(), Some(&head));
        assert_eq!(e.mem.u32(map + 0x18), 0);
        assert_eq!(e.mem.u32(map + 0x1c), 0);
        // The proxy is freed on its own.
        let proxy = e.mem.u32(map);
        freed.borrow_mut().clear();
        e.call(0x00b3_9460, &args![map]);
        assert_eq!(*freed.borrow(), vec![proxy]);
        assert_eq!(invalid.get(), 0);
    }

    fn subtree_size(e: &Engine, node: u32, head: u32) -> usize {
        if node == head {
            0
        } else {
            1 + subtree_size(e, e.mem.u32(node), head) + subtree_size(e, e.mem.u32(node + 8), head)
        }
    }

    #[test]
    fn erase_subtree_frees_a_subtree_and_tidies_each_key() {
        let (mut e, _) = world();
        let map = new_map(&mut e);
        for i in 0..7 {
            index(&mut e, map, &key(i));
        }
        let tidied = Rc::new(RefCell::new(Vec::new()));
        let log = tidied.clone();
        e.register_double(STRING_TIDY, move |_, a| {
            log.borrow_mut().push((a[0], a[1], a[2]));
            Ret::default()
        });
        let freed = Rc::new(RefCell::new(Vec::new()));
        let log = freed.clone();
        e.register_double(OPERATOR_DELETE, move |e, a| {
            log.borrow_mut().push(a[0]);
            e.mem.free(a[0]);
            Ret::default()
        });
        let head = map_head_of(&e, map);
        let root = e.mem.u32(head + 4);
        let left = e.mem.u32(root);
        e.call(0x00b3_a050, &args![map, left]);
        // Each node of the left subtree: its key tidied with (1, 0).
        let count = subtree_size(&e, left, head);
        assert!(count >= 1);
        assert_eq!(freed.borrow().len(), count);
        assert_eq!(tidied.borrow().len(), count);
        assert!(tidied.borrow().iter().all(|t| t.1 == 1 && t.2 == 0));
        let freed_nodes: Vec<u32> = tidied.borrow().iter().map(|t| t.0 - 0xc).collect();
        let mut sorted = freed_nodes.clone();
        let mut deleted = freed.borrow().clone();
        sorted.sort();
        deleted.sort();
        assert_eq!(sorted, deleted);
        // The nil node is a no-op.
        e.call(0x00b3_a050, &args![map, head]);
        assert_eq!(freed.borrow().len(), count);
    }

    #[test]
    fn buynode_builds_a_node_from_the_pair() {
        let (mut e, invalid) = world();
        let map = new_map(&mut e);
        let pair = put_pair(&mut e, "hello", 0xcafe);
        let node = e
            .call(
                0x00b3_b330,
                &args![map, 1u32, 2u32, 3u32, pair, 0x1_0001u32],
            )
            .u32();
        assert_eq!(e.mem.u32(node), 1);
        assert_eq!(e.mem.u32(node + 4), 2);
        assert_eq!(e.mem.u32(node + 8), 3);
        assert_eq!(string_text(&e, node + 0xc), "hello");
        assert_eq!(e.mem.u32(node + 0x28), 0xcafe);
        // The colour is the low byte of the argument.
        assert_eq!(e.mem.u8(node + 0x2c), 1);
        assert_eq!(e.mem.u8(node + 0x2d), 0);
        assert_eq!(invalid.get(), 0);
        // A failed allocation leaves the node unbuilt.
        e.register_double(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(
            e.call(0x00b3_b330, &args![map, 1u32, 2u32, 3u32, pair, 0u32])
                .u32(),
            0
        );
    }

    #[test]
    fn node_allocation_asks_for_the_count_times_the_node_size() {
        let (mut e, _) = world();
        let sizes = Rc::new(RefCell::new(Vec::new()));
        let log = sizes.clone();
        e.register_double(OPERATOR_NEW, move |e, a| {
            log.borrow_mut().push(a[0]);
            ret(e.mem.alloc(a[0].clamp(4, 0x1000)))
        });
        e.call(0x00b3_b540, &args![1u32, 0u32]);
        e.call(0x00b3_b540, &args![3u32, 0u32]);
        e.call(0x00b3_b540, &args![0u32, 0u32]);
        e.call(0x00b3_b540, &args![0x0555_5555u32, 0u32]);
        let map = e.mem.alloc(0x20);
        e.call(0x00b3_b9d0, &args![map, 5u32]);
        assert_eq!(*sizes.borrow(), vec![0x30, 0x90, 0, 0xffff_fff0, 0xf0]);
    }

    #[test]
    #[should_panic(expected = "thrown 0x1140ef8")]
    fn a_node_count_that_overflows_throws_bad_alloc() {
        let (mut e, _) = world();
        e.register_double(BAD_ALLOC_CONSTRUCTOR, |_, a| ret(a[0]));
        e.call(0x00b3_b540, &args![0x0555_5556u32, 0u32]);
    }

    #[test]
    #[should_panic(expected = "thrown 0x117327c")]
    fn erasing_the_head_node_throws() {
        let (mut e, _) = world();
        let map = new_map(&mut e);
        let head = map_head_of(&e, map);
        let proxy = e.mem.u32(map);
        let out = e.mem.alloc(8);
        e.call(0x00b3_bd80, &args![map, out, proxy, head]);
    }

    #[test]
    fn the_erase_throw_builds_its_exception_from_the_text() {
        let (mut e, _) = world();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let log = seen.clone();
        e.register_double(STRING_FROM_C_STRING, move |_, a| {
            log.borrow_mut().push(("text", a[1]));
            ret(a[0])
        });
        let log = seen.clone();
        e.register_double(RUNTIME_ERROR_FROM_STRING, move |_, a| {
            log.borrow_mut().push(("exception", a[1]));
            ret(a[0])
        });
        let log = seen.clone();
        e.register_double(CXX_THROW_EXCEPTION, move |e, a| {
            log.borrow_mut().push(("vtable", e.mem.u32(a[0])));
            log.borrow_mut().push(("info", a[1]));
            panic!("thrown");
        });
        let map = new_map(&mut e);
        let head = map_head_of(&e, map);
        let proxy = e.mem.u32(map);
        let out = e.mem.alloc(8);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.call(0x00b3_bd80, &args![map, out, proxy, head]);
        }));
        assert!(result.is_err());
        assert_eq!(seen.borrow()[0], ("text", MAP_ITERATOR_TEXT));
        assert_eq!(seen.borrow()[1].0, "exception");
        assert_eq!(seen.borrow()[2], ("vtable", MAP_ITERATOR_VTABLE));
        assert_eq!(seen.borrow()[3], ("info", MAP_ITERATOR_THROW_INFO));
    }

    #[test]
    #[should_panic(expected = "thrown 0x1140cb0")]
    fn inserting_into_a_full_map_throws_length_error() {
        let (mut e, _) = world();
        let map = new_map(&mut e);
        e.mem.set_u32(map + 0x1c, 0x07ff_fffe);
        let pair = put_pair(&mut e, "a", 0);
        let out = e.mem.alloc(8);
        let head = map_head_of(&e, map);
        e.call(0x00b3_ac00, &args![map, out, 1u32, head, pair]);
    }

    #[test]
    fn insert_node_links_the_first_node_as_root_and_both_ends() {
        let (mut e, invalid) = world();
        let map = new_map(&mut e);
        let head = map_head_of(&e, map);
        let pair = put_pair(&mut e, "a", 5);
        let out = e.mem.alloc(8);
        let returned = e
            .call(0x00b3_ac00, &args![map, out, 1u32, head, pair])
            .u32();
        assert_eq!(returned, out);
        let node = e.mem.u32(head + 4);
        assert_ne!(node, head);
        assert_eq!(e.mem.u32(head), node);
        assert_eq!(e.mem.u32(head + 8), node);
        assert_eq!(e.mem.u32(out), e.mem.u32(map));
        assert_eq!(e.mem.u32(out + 4), node);
        assert_eq!(e.mem.u32(map + 0x1c), 1);
        assert_eq!(e.mem.u32(node + 0x28), 5);
        assert_eq!(check_tree(&e, map), vec!["a".to_string()]);
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn unique_insert_reports_whether_the_key_was_new() {
        let (mut e, invalid) = world();
        let map = new_map(&mut e);
        let out = e.mem.alloc(12);
        for (name, new) in [
            ("m", 1u8),
            ("c", 1),
            ("t", 1),
            ("m", 0),
            ("a", 1),
            ("c", 0),
            ("z", 1),
        ] {
            let pair = put_pair(&mut e, name, 3);
            let returned = e.call(0x00b3_a9b0, &args![map, out, pair]).u32();
            assert_eq!(returned, out);
            assert_eq!(e.mem.u8(out + 8), new, "{name}");
            assert_eq!(string_text(&e, e.mem.u32(out + 4) + 0xc), name);
            assert_eq!(e.mem.u32(out), e.mem.u32(map));
            check_tree(&e, map);
        }
        assert_eq!(check_tree(&e, map), ["a", "c", "m", "t", "z"]);
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn unique_insert_for_equal_keys_always_inserts() {
        // The branch the exe skips with a constant test: translated as
        // written, it inserts at the leaf the search found.
        let (mut e, _) = world();
        let map = new_map(&mut e);
        let out = e.mem.alloc(12);
        for name in ["m", "c", "t", "a", "z", "e"] {
            let pair = put_pair(&mut e, name, 3);
            let returned =
                insert_unique(&mut e, Ptr::new(map), Ptr::new(out), Ptr::new(pair), true);
            assert_eq!(returned.addr(), out);
            assert_eq!(e.mem.u8(out + 8), 1);
            assert_eq!(string_text(&e, e.mem.u32(out + 4) + 0xc), name);
            check_tree(&e, map);
        }
        assert_eq!(check_tree(&e, map), ["a", "c", "e", "m", "t", "z"]);
    }

    /// The nodes of the tree in order, then the head.
    fn nodes_in_order(e: &Engine, map: u32) -> Vec<u32> {
        fn walk(e: &Engine, node: u32, head: u32, out: &mut Vec<u32>) {
            if node != head {
                walk(e, e.mem.u32(node), head, out);
                out.push(node);
                walk(e, e.mem.u32(node + 8), head, out);
            }
        }
        let head = map_head_of(e, map);
        let mut out = Vec::new();
        walk(e, e.mem.u32(head + 4), head, &mut out);
        out.push(head);
        out
    }

    #[test]
    fn insert_with_a_hint_works_wherever_the_hint_points() {
        let existing = [20, 40, 60, 80, 100, 120];
        // Before everything, in each gap, after everything.
        let new_keys = [5, 30, 50, 70, 90, 110, 130];
        for equal_keys in [false, true] {
            for &new_key in &new_keys {
                for hint_index in 0..=existing.len() {
                    let (mut e, invalid) = world();
                    let map = new_map(&mut e);
                    for &k in &existing {
                        index(&mut e, map, &key(k));
                    }
                    let hint = nodes_in_order(&e, map)[hint_index];
                    let pair = put_pair(&mut e, &key(new_key), 77);
                    let out = e.mem.alloc(8);
                    let proxy = e.mem.u32(map);
                    let returned = insert_with_hint(
                        &mut e,
                        Ptr::new(map),
                        Ptr::new(out),
                        proxy,
                        hint,
                        Ptr::new(pair),
                        equal_keys,
                    );
                    assert_eq!(returned.addr(), out);
                    let keys = check_tree(&e, map);
                    assert_eq!(keys.len(), existing.len() + 1);
                    assert!(keys.contains(&key(new_key)));
                    let node = e.mem.u32(out + 4);
                    assert_eq!(string_text(&e, node + 0xc), key(new_key));
                    assert_eq!(e.mem.u32(node + 0x28), 77);
                    assert_eq!(e.mem.u32(out), proxy);
                    assert_eq!(invalid.get(), 0, "hint {hint_index}, key {new_key}");
                }
            }
        }
    }

    #[test]
    fn insert_with_a_hint_into_an_empty_map_makes_the_root() {
        let (mut e, invalid) = world();
        let map = new_map(&mut e);
        let head = map_head_of(&e, map);
        let proxy = e.mem.u32(map);
        let pair = put_pair(&mut e, "only", 1);
        let out = e.mem.alloc(8);
        let returned = e
            .call(0x00b3_98c0, &args![map, out, proxy, head, pair])
            .u32();
        assert_eq!(returned, out);
        assert_eq!(check_tree(&e, map), ["only"]);
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn a_wrong_hint_proxy_is_reported_by_the_hint_insert() {
        for equal_keys in [false, true] {
            let (mut e, invalid) = world();
            let map = new_map(&mut e);
            index(&mut e, map, "b");
            let begin = nodes_in_order(&e, map)[0];
            let pair = put_pair(&mut e, "a", 1);
            let out = e.mem.alloc(8);
            insert_with_hint(
                &mut e,
                Ptr::new(map),
                Ptr::new(out),
                0,
                begin,
                Ptr::new(pair),
                equal_keys,
            );
            // The equal-keys form checks the proxy itself; the form the exe
            // runs compares iterators in `00b2ac30`, which is a double here.
            assert_eq!(invalid.get(), u32::from(equal_keys));
            check_tree(&e, map);
        }
    }

    // ---- the spline ----

    fn new_spline(e: &mut Engine) -> u32 {
        let spline = e.mem.alloc(0x84);
        for offset in [0x0c, 0x24, 0x54, 0x6c] {
            e.call(VECTOR_CVEC_CONSTRUCT, &args![spline + offset]);
        }
        e.call(VECTOR_FLOAT_CONSTRUCT, &args![spline + 0x3c]);
        spline
    }

    /// The first two components of each element of a `vector<CVec>`.
    fn points(e: &Engine, vector: u32) -> Vec<(f32, f32)> {
        let first = e.mem.u32(vector + 0x0c);
        let count = (e.mem.u32(vector + 0x10) - first) / 0x18;
        (0..count)
            .map(|i| (e.mem.f32(first + 0x18 * i), e.mem.f32(first + 0x18 * i + 4)))
            .collect()
    }

    fn floats(e: &Engine, vector: u32) -> Vec<f32> {
        let first = e.mem.u32(vector + 0x0c);
        let count = (e.mem.u32(vector + 0x10) - first) / 4;
        (0..count).map(|i| e.mem.f32(first + 4 * i)).collect()
    }

    /// Replaces the sample table with `count` samples whose second
    /// component is `f(k)`.
    fn set_samples(e: &mut Engine, spline: u32, count: u32, f: impl Fn(u32) -> f32) {
        let table = spline + 0x54;
        let buffer = e.mem.alloc(count * 0x18);
        for k in 0..count {
            e.mem.set_f32(buffer + 0x18 * k, k as f32 / count as f32);
            e.mem.set_f32(buffer + 0x18 * k + 4, f(k));
            e.mem.set_i32(buffer + 0x18 * k + 0x14, 2);
        }
        e.mem.set_u32(table + 0x0c, buffer);
        e.mem.set_u32(table + 0x10, buffer + 0x18 * count);
        e.mem.set_u32(table + 0x14, buffer + 0x18 * count);
    }

    /// The spline assignment (`00b1c7a0`) as a deep copy of the fields.
    fn assign_spline(e: &mut Engine, a: &[u32]) -> Ret {
        let (to, from) = (a[0], a[1]);
        for offset in [0, 4, 8] {
            let word = e.mem.u32(from + offset);
            e.mem.set_u32(to + offset, word);
        }
        for (offset, stride) in [
            (0x0c, 0x18),
            (0x24, 0x18),
            (0x3c, 4),
            (0x54, 0x18),
            (0x6c, 0x18),
        ] {
            let first = e.mem.u32(from + offset + 0x0c);
            let bytes = e.mem.u32(from + offset + 0x10) - first;
            let old = e.mem.u32(to + offset + 0x0c);
            e.mem.free(old);
            let (copy, end) = if first == 0 {
                (0, 0)
            } else {
                let copy = e.mem.alloc(bytes.max(stride));
                let data = e.mem.bytes(first, bytes);
                e.mem.write(copy, &data);
                (copy, copy + bytes)
            };
            e.mem.set_u32(to + offset + 0x0c, copy);
            e.mem.set_u32(to + offset + 0x10, end);
            e.mem.set_u32(to + offset + 0x14, end);
        }
        Ret::default()
    }

    fn spline_world() -> (Engine, Invalid) {
        let (mut e, invalid) = world();
        e.register_double(SPLINE_ASSIGN, assign_spline);
        (e, invalid)
    }

    fn assert_close(actual: f32, expected: f32, tolerance: f32) {
        assert!(
            (actual - expected).abs() <= tolerance,
            "{actual} is not within {tolerance} of {expected}"
        );
    }

    #[test]
    fn next_token_skips_white_space_and_reads_one_word() {
        let (mut e, _) = world();
        let text = e.mem.alloc(32);
        e.mem.set_cstr(text, b"  \t first  second");
        let token = e.mem.alloc(32);
        let after = e.call(0x00b3_7f50, &args![text, token]).u32();
        assert_eq!(e.mem.cstr(token), b"first");
        assert_eq!(after, text + 9);
        let after = e.call(0x00b3_7f50, &args![after, token]).u32();
        assert_eq!(e.mem.cstr(token), b"second");
        assert_eq!(after, text + 17);
    }

    #[test]
    fn the_cubic_matches_the_bernstein_form() {
        let (mut e, _) = world();
        let p0 = cvec(&mut e, &[0.0, 0.0]);
        let p1 = cvec(&mut e, &[1.0, 2.0]);
        let p2 = cvec(&mut e, &[3.0, 4.0]);
        let p3 = cvec(&mut e, &[4.0, 8.0]);
        let out = e.mem.alloc(0x18);
        for t in [0.0f32, 0.25, 0.5, 0.8, 1.0] {
            let returned = e
                .call(0x00b3_8f40, &args![0u32, out, p0, p1, p2, p3, t])
                .u32();
            assert_eq!(returned, out);
            let u = 1.0 - t;
            let bernstein = |a: f32, b: f32, c: f32, d: f32| {
                u * u * u * a + 3.0 * u * u * t * b + 3.0 * u * t * t * c + t * t * t * d
            };
            assert_close(e.mem.f32(out), bernstein(0.0, 1.0, 3.0, 4.0), 1e-5);
            assert_close(e.mem.f32(out + 4), bernstein(0.0, 2.0, 4.0, 8.0), 1e-5);
            assert_eq!(e.mem.i32(out + 0x14), 2);
        }
        // The ends are exact.
        e.call(0x00b3_8f40, &args![0u32, out, p0, p1, p2, p3, 0.0f32]);
        assert_eq!((e.mem.f32(out), e.mem.f32(out + 4)), (0.0, 0.0));
        e.call(0x00b3_8f40, &args![0u32, out, p0, p1, p2, p3, 1.0f32]);
        assert_eq!((e.mem.f32(out), e.mem.f32(out + 4)), (4.0, 8.0));
    }

    /// A spline with two points: (0, 0) with tangent (1, 0) and length 0.25,
    /// and (1, 1) with tangent (0, 1) and length 0.5.
    fn two_point_spline(e: &mut Engine) -> u32 {
        let spline = new_spline(e);
        let point = cvec(e, &[0.0, 0.0]);
        let tangent = cvec(e, &[1.0, 0.0]);
        e.call(0x00b3_80e0, &args![spline, point, tangent, 0.25f32]);
        let point = cvec(e, &[1.0, 1.0]);
        let tangent = cvec(e, &[0.0, 1.0]);
        e.call(0x00b3_80e0, &args![spline, point, tangent, 0.5f32]);
        spline
    }

    #[test]
    fn add_point_appends_the_handles_between_points() {
        let (mut e, invalid) = world();
        let spline = new_spline(&mut e);
        let point = cvec(&mut e, &[2.0, 3.0]);
        let tangent = cvec(&mut e, &[3.0, 4.0]);
        e.call(0x00b3_80e0, &args![spline, point, tangent, 0.5f32]);
        assert_eq!(points(&e, spline + 0x0c), [(2.0, 3.0)]);
        assert_eq!(points(&e, spline + 0x6c), [(2.0, 3.0)]);
        let normalised = ((3.0f64 / 5.0) as f32, (4.0f64 / 5.0) as f32);
        assert_eq!(points(&e, spline + 0x24), [normalised]);
        assert_eq!(floats(&e, spline + 0x3c), [0.5]);
        // The second point adds the previous point plus its tangent times
        // its length, and the new point minus its tangent times the length.
        let spline = two_point_spline(&mut e);
        assert_eq!(points(&e, spline + 0x0c), [(0.0, 0.0), (1.0, 1.0)]);
        assert_eq!(
            points(&e, spline + 0x6c),
            [(0.0, 0.0), (0.25, 0.0), (1.0, 0.5), (1.0, 1.0)]
        );
        assert_eq!(points(&e, spline + 0x24), [(1.0, 0.0), (0.0, 1.0)]);
        assert_eq!(floats(&e, spline + 0x3c), [0.25, 0.5]);
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn the_point_at_t_follows_the_bezier_segments() {
        let (mut e, invalid) = world();
        let spline = two_point_spline(&mut e);
        let out = e.mem.alloc(0x18);
        let at = |e: &mut Engine, t: f32| {
            let returned = e.call(0x00b3_8b50, &args![spline, out, t]).u32();
            assert_eq!(returned, out);
            assert_eq!(e.mem.i32(out + 0x14), 2);
            (e.mem.f32(out), e.mem.f32(out + 4))
        };
        assert_eq!(at(&mut e, 0.0), (0.0, 0.0));
        assert_eq!(at(&mut e, 1.0), (1.0, 1.0));
        let (x, y) = at(&mut e, 0.5);
        assert_close(x, 0.59375, 1e-6);
        assert_close(y, 0.3125, 1e-6);
        // Outside 0..=1 the parameter is clamped; a NaN becomes 0.
        assert_eq!(at(&mut e, -3.0), (0.0, 0.0));
        assert_eq!(at(&mut e, 7.0), (1.0, 1.0));
        assert_eq!(at(&mut e, f32::NAN), (0.0, 0.0));
        assert_eq!(invalid.get(), 0);
        // With fewer than two control points the answer is the zero point.
        let one = new_spline(&mut e);
        let point = cvec(&mut e, &[5.0, 6.0]);
        let tangent = cvec(&mut e, &[1.0, 0.0]);
        e.call(0x00b3_80e0, &args![one, point, tangent, 0.5f32]);
        e.call(0x00b3_8b50, &args![one, out, 0.5f32]);
        assert_eq!(cvec_components(&e, out), (bits(&[0.0; 5]), 2));
    }

    #[test]
    fn the_sample_table_runs_from_the_first_to_the_last_control_point() {
        let (mut e, invalid) = world();
        let spline = two_point_spline(&mut e);
        e.call(0x00b3_8430, &args![spline, 500u32]);
        let samples = points(&e, spline + 0x54);
        assert_eq!(samples.len(), 500);
        assert_eq!(samples[0], (0.0, 0.0));
        assert_eq!(samples[499], (1.0, 1.0));
        // Sample k lies at x = k / 500 on the curve: solve x(u) = k / 500.
        let curve = |u: f64| {
            let v = 1.0 - u;
            (
                3.0 * v * v * u * 0.25 + 3.0 * v * u * u + u * u * u,
                3.0 * v * u * u * 0.5 + u * u * u,
            )
        };
        for k in [1u32, 100, 250, 400, 498] {
            let x = k as f64 / 500.0;
            let (mut low, mut high) = (0.0f64, 1.0f64);
            for _ in 0..60 {
                let middle = (low + high) / 2.0;
                if curve(middle).0 < x {
                    low = middle;
                } else {
                    high = middle;
                }
            }
            let (sample_x, sample_y) = samples[k as usize];
            assert_close(sample_x, x as f32, 1e-6);
            assert_close(sample_y, curve(low).1 as f32, 2e-4);
        }
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn evaluate_interpolates_the_table_and_adds_the_random_amount() {
        let (mut e, invalid) = world();
        let spline = e.mem.alloc(0x84);
        for offset in [0x0c, 0x24, 0x54, 0x6c] {
            e.call(VECTOR_CVEC_CONSTRUCT, &args![spline + offset]);
        }
        set_samples(&mut e, spline, 500, |k| k as f32 / 499.0);
        e.mem.set_f32(spline, 2.0);
        e.mem.set_f32(spline + 4, 4.0);
        e.mem.set_f32(spline + 8, 0.5);
        let draws = Rc::new(RefCell::new(Vec::new()));
        let log = draws.clone();
        e.register_double(RANDOM_GET_UNIFORM, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                st0: 0.25,
                ..Ret::default()
            }
        });
        let constructed = Rc::new(RefCell::new(Vec::new()));
        let log = constructed.clone();
        e.register_double(RANDOM_CONSTRUCTOR, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(a[0])
        });
        let registered = Rc::new(RefCell::new(Vec::new()));
        let log = registered.clone();
        e.register_double(ATEXIT, move |_, a| {
            log.borrow_mut().push(a[0]);
            ret(0)
        });
        let evaluate = |e: &mut Engine, t: f32| e.call(0x00b3_7ad0, &args![spline, t]).f32();
        assert_close(evaluate(&mut e, 0.5), 3.25, 1e-5);
        assert_close(evaluate(&mut e, 0.0), 2.25, 1e-6);
        // The last sample is used alone.
        assert_close(evaluate(&mut e, 1.0), 4.25, 1e-6);
        assert_close(evaluate(&mut e, 0.999), 4.25 - 0.001 * 2.0, 1e-4);
        // The random range is the variance either way round.
        assert_eq!(
            draws.borrow()[0][1..],
            [(-0.5f32).to_bits(), 0.5f32.to_bits()]
        );
        assert_eq!(draws.borrow()[0][0], EVALUATE_RANDOM.0);
        // The generator is built and registered once.
        assert_eq!(*constructed.borrow(), vec![EVALUATE_RANDOM.0]);
        assert_eq!(*registered.borrow(), vec![EVALUATE_RANDOM.2]);
        assert_eq!(e.global::<u32>(EVALUATE_RANDOM.1) & 1, 1);
        // A table of any other size gives 0 and draws nothing.
        let count = draws.borrow().len();
        set_samples(&mut e, spline, 10, |k| k as f32);
        assert_eq!(evaluate(&mut e, 0.5), 0.0);
        assert_eq!(draws.borrow().len(), count);
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn evaluate_quiets_a_signalling_nan_variance() {
        let (mut e, _) = world();
        let spline = e.mem.alloc(0x84);
        for offset in [0x0c, 0x24, 0x54, 0x6c] {
            e.call(VECTOR_CVEC_CONSTRUCT, &args![spline + offset]);
        }
        set_samples(&mut e, spline, 500, |_| 0.5);
        e.mem.set_u32(spline + 8, 0x7f80_0001);
        let draws = Rc::new(RefCell::new(Vec::new()));
        let log = draws.clone();
        e.register_double(RANDOM_GET_UNIFORM, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.call(0x00b3_7ad0, &args![spline, 0.5f32]);
        // `-variance` and `variance`, both quiet NaNs (the sign flipped).
        assert_eq!(draws.borrow()[0][1], 0xffc0_0001);
        assert_eq!(draws.borrow()[0][2], 0x7fc0_0001);
    }

    #[test]
    fn scaled_variance_draws_within_the_samples_second_component() {
        let (mut e, invalid) = world();
        let spline = e.mem.alloc(0x84);
        for offset in [0x0c, 0x24, 0x54, 0x6c] {
            e.call(VECTOR_CVEC_CONSTRUCT, &args![spline + offset]);
        }
        set_samples(&mut e, spline, 500, |k| k as f32 / 100.0);
        e.mem.set_f32(spline + 8, 2.0);
        let draws = Rc::new(RefCell::new(Vec::new()));
        let log = draws.clone();
        e.register_double(RANDOM_GET_UNIFORM, move |_, a| {
            log.borrow_mut().push(a.to_vec());
            Ret {
                st0: 1.5,
                ..Ret::default()
            }
        });
        // t * 499 + 0.5 = 249.5 + 0.5 truncates to 250: the value is 2.5.
        let result = e.call(0x00b3_7fa0, &args![spline, 0.5f32]).f32();
        assert_eq!(result, 1.5);
        assert_eq!(
            draws.borrow()[0][1..],
            [(-5.0f32).to_bits(), 5.0f32.to_bits()]
        );
        assert_eq!(draws.borrow()[0][0], VARIANCE_RANDOM.0);
        // Another table size gives 0 without a draw.
        set_samples(&mut e, spline, 3, |_| 1.0);
        assert_eq!(e.call(0x00b3_7fa0, &args![spline, 0.5f32]).f32(), 0.0);
        assert_eq!(draws.borrow().len(), 1);
        assert_eq!(invalid.get(), 0);
    }

    const SPLINE_TEXT: &str = "BezierSpline 2 4 0.5 { 2 0 0 1 0 0.25 1 1 0 1 0.5 }";

    #[test]
    fn parse_reads_the_header_and_the_points() {
        let (mut e, invalid) = world();
        let spline = new_spline(&mut e);
        let text = put_string(&mut e, SPLINE_TEXT);
        e.call(0x00b3_7cc0, &args![spline, text]);
        assert_eq!(
            (
                e.mem.f32(spline),
                e.mem.f32(spline + 4),
                e.mem.f32(spline + 8)
            ),
            (2.0, 4.0, 0.5)
        );
        assert_eq!(points(&e, spline + 0x0c), [(0.0, 0.0), (1.0, 1.0)]);
        assert_eq!(points(&e, spline + 0x24), [(1.0, 0.0), (0.0, 1.0)]);
        assert_eq!(floats(&e, spline + 0x3c), [0.25, 0.5]);
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn parse_ignores_text_that_does_not_start_with_the_tag() {
        let (mut e, _) = world();
        let spline = new_spline(&mut e);
        e.mem.set_f32(spline, 9.0);
        let text = put_string(&mut e, "Other 2 4 0.5 { 1 0 0 1 0 0.25 }");
        e.call(0x00b3_7cc0, &args![spline, text]);
        assert_eq!(e.mem.f32(spline), 9.0);
        assert_eq!(points(&e, spline + 0x0c), []);
        // Without the opening brace only the header is read.
        let text = put_string(&mut e, "BezierSpline 2 4 0.5 [ 1 0 0 1 0 0.25 ]");
        e.call(0x00b3_7cc0, &args![spline, text]);
        assert_eq!(e.mem.f32(spline), 2.0);
        assert_eq!(points(&e, spline + 0x0c), []);
        // A count of 0, or a negative one, adds no point.
        for count in ["0", "-3"] {
            let text = put_string(
                &mut e,
                &format!("BezierSpline 1 3 0 {{ {count} 5 5 1 0 1 }}"),
            );
            e.call(0x00b3_7cc0, &args![spline, text]);
            assert_eq!(e.mem.f32(spline), 1.0);
            assert_eq!(points(&e, spline + 0x0c), []);
        }
    }

    /// The cache of splines, constructed.
    fn cache_world() -> (Engine, Invalid) {
        let (mut e, invalid) = spline_world();
        let comparator = e.mem.alloc(4);
        e.call(0x00b3_b770, &args![SPLINE_CACHE, comparator, 0u32]);
        (e, invalid)
    }

    fn cached(e: &mut Engine, name: &str) -> u32 {
        let text = put_string(e, name);
        let slot = e.call(0x00b3_9210, &args![SPLINE_CACHE, text]).u32();
        e.mem.u32(slot)
    }

    #[test]
    fn the_constructor_parses_once_and_then_copies_from_the_cache() {
        let (mut e, invalid) = cache_world();
        let parses = Rc::new(Cell::new(0));
        let count = parses.clone();
        // `Parse` compares its first token with the tag once per call.
        e.register_double(STRCMP, move |e, a| {
            count.set(count.get() + 1);
            ret(e.mem.cstr(a[0]).cmp(&e.mem.cstr(a[1])) as i32 as u32)
        });
        let name = put_string(&mut e, SPLINE_TEXT);
        let first = e.mem.alloc(0x84);
        let returned = e.call(0x00b3_7660, &args![first, name]).u32();
        assert_eq!(returned, first);
        assert_eq!(parses.get(), 1);
        assert_eq!(e.mem.f32(first + 4), 4.0);
        assert_eq!(points(&e, first + 0x54).len(), 500);
        // The cache holds a copy under the name.
        let copy = cached(&mut e, SPLINE_TEXT);
        assert_ne!(copy, 0);
        assert_ne!(copy, first);
        assert_eq!(points(&e, copy + 0x54), points(&e, first + 0x54));
        assert_eq!(e.mem.u32(SPLINE_CACHE + 0x1c), 1);
        // The second construction copies from the cache.
        let second = e.mem.alloc(0x84);
        e.call(0x00b3_7660, &args![second, name]);
        assert_eq!(parses.get(), 1);
        assert_eq!(points(&e, second + 0x54), points(&e, first + 0x54));
        assert_eq!(points(&e, second + 0x0c), [(0.0, 0.0), (1.0, 1.0)]);
        assert_eq!(e.mem.f32(second), 2.0);
        assert_eq!(invalid.get(), 0);
        // The cache is emptied: the cached spline is destroyed and freed.
        e.call(0x00b3_7a00, &args![]);
        assert_eq!(e.mem.u32(SPLINE_CACHE + 0x1c), 0);
        check_tree(&e, SPLINE_CACHE);
        assert_eq!(invalid.get(), 0);
    }

    #[test]
    fn a_failed_allocation_caches_a_null_pointer() {
        let (mut e, _) = cache_world();
        e.register_double(OPERATOR_NEW, |e, a| {
            if a[0] == 0x84 {
                Ret::default()
            } else {
                ret(e.mem.alloc(a[0].max(4)))
            }
        });
        let parses = Rc::new(Cell::new(0));
        let count = parses.clone();
        // `Parse` compares its first token with the tag once per call.
        e.register_double(STRCMP, move |e, a| {
            count.set(count.get() + 1);
            ret(e.mem.cstr(a[0]).cmp(&e.mem.cstr(a[1])) as i32 as u32)
        });
        let name = put_string(&mut e, SPLINE_TEXT);
        let spline = e.mem.alloc(0x84);
        e.call(0x00b3_7660, &args![spline, name]);
        assert_eq!(cached(&mut e, SPLINE_TEXT), 0);
        // A null entry is a miss: the text is parsed again.
        let other = e.mem.alloc(0x84);
        e.call(0x00b3_7660, &args![other, name]);
        assert_eq!(parses.get(), 2);
        // Clearing the cache skips the null entry.
        e.call(0x00b3_7a00, &args![]);
        assert_eq!(e.mem.u32(SPLINE_CACHE + 0x1c), 0);
    }

    #[test]
    fn the_copy_constructor_builds_the_vectors_then_assigns() {
        let (mut e, _) = world();
        let order = Rc::new(RefCell::new(Vec::new()));
        for address in [VECTOR_CVEC_CONSTRUCT, VECTOR_FLOAT_CONSTRUCT, SPLINE_ASSIGN] {
            let log = order.clone();
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a.to_vec()));
                ret(a[0])
            });
        }
        let (this, other) = (0x1000u32, 0x2000u32);
        assert_eq!(e.call(0x00b3_77c0, &args![this, other]).u32(), this);
        assert_eq!(
            *order.borrow(),
            vec![
                (VECTOR_CVEC_CONSTRUCT, vec![this + 0x0c]),
                (VECTOR_CVEC_CONSTRUCT, vec![this + 0x24]),
                (VECTOR_FLOAT_CONSTRUCT, vec![this + 0x3c]),
                (VECTOR_CVEC_CONSTRUCT, vec![this + 0x54]),
                (VECTOR_CVEC_CONSTRUCT, vec![this + 0x6c]),
                (SPLINE_ASSIGN, vec![this, other]),
            ]
        );
    }

    #[test]
    fn the_destructor_tidies_the_vectors_in_reverse_order() {
        let (mut e, _) = world();
        let order = Rc::new(RefCell::new(Vec::new()));
        for address in [VECTOR_CVEC_TIDY, VECTOR_FLOAT_TIDY, OPERATOR_DELETE] {
            let log = order.clone();
            e.register_double(address, move |_, a| {
                log.borrow_mut().push((address, a[0]));
                Ret::default()
            });
        }
        let spline = e.mem.alloc(0x84);
        for (i, offset) in [0x0c, 0x24, 0x3c, 0x54, 0x6c].into_iter().enumerate() {
            e.mem.set_u32(spline + offset, 0x100 + i as u32);
        }
        e.call(0x00b3_7880, &args![spline]);
        assert_eq!(
            *order.borrow(),
            vec![
                (VECTOR_CVEC_TIDY, spline + 0x6c),
                (OPERATOR_DELETE, 0x104),
                (VECTOR_CVEC_TIDY, spline + 0x54),
                (OPERATOR_DELETE, 0x103),
                (VECTOR_FLOAT_TIDY, spline + 0x3c),
                (OPERATOR_DELETE, 0x102),
                (VECTOR_CVEC_TIDY, spline + 0x24),
                (OPERATOR_DELETE, 0x101),
                (VECTOR_CVEC_TIDY, spline + 0x0c),
                (OPERATOR_DELETE, 0x100),
            ]
        );
    }

    // ---- the exe's stack frames ----

    /// The calls made to `address` while the call log was on.
    fn logged(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == address)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The local the next `with_stack(size)` on the running thread takes (the outermost frame
    /// of the call that follows), with the call log switched on.
    fn next_frame(e: &mut Engine, size: u32) -> u32 {
        let frame = e.next_stack_local(size);
        e.call_log = Some(vec![]);
        frame
    }

    /// The local a `with_stack(size)` takes directly inside the next `with_stack(outer)` on the
    /// running thread (a callee's frame nested in the frame [`next_frame`] answers for `outer`).
    fn nested_frame(e: &mut Engine, outer: u32, size: u32) -> u32 {
        e.with_stack(outer, |e, _| e.next_stack_local(size))
    }

    #[test]
    fn the_cvec_operators_keep_their_local_at_the_exe_frame_offset() {
        let (mut e, _) = world();
        let a = cvec(&mut e, &[1.0, 2.0]);
        let b = cvec(&mut e, &[3.0, 4.0]);
        let out = e.mem.alloc(0x18);
        for address in [0x00b3_c660u32, 0x00b3_c750] {
            let frame = next_frame(&mut e, 0x38);
            e.call(address, &args![a, out, b]);
            assert_eq!(logged(&e, CVEC_SIZED)[0][0], frame + 0x14);
        }
        let frame = next_frame(&mut e, 0x34);
        e.call(0x00b3_c840, &args![a, out, 2.0f32]);
        assert_eq!(logged(&e, CVEC_SIZED)[0][0], frame + 0x10);
    }

    #[test]
    fn add_point_keeps_its_locals_and_the_length_argument_slot() {
        let (mut e, _) = world();
        let spline = new_spline(&mut e);
        let point = cvec(&mut e, &[0.0, 0.0]);
        let tangent = cvec(&mut e, &[1.0, 0.0]);
        e.call(0x00b3_80e0, &args![spline, point, tangent, 0.25f32]);
        let point = cvec(&mut e, &[1.0, 1.0]);
        let tangent = cvec(&mut e, &[0.0, 1.0]);
        let frame = next_frame(&mut e, 0x298);
        e.call(0x00b3_80e0, &args![spline, point, tangent, 0.5f32]);
        let made: Vec<u32> = logged(&e, CVEC_FROM_TWO).iter().map(|c| c[0]).collect();
        assert_eq!(made, vec![frame + 0x260, frame + 0x248]);
        let scaled: Vec<u32> = logged(&e, CVEC_SCALE).iter().map(|c| c[1]).collect();
        assert_eq!(scaled, vec![frame + 0x22c, frame + 0x1fc]);
        assert_eq!(logged(&e, CVEC_ADD)[0][1], frame + 0x214);
        assert_eq!(logged(&e, CVEC_SUBTRACT)[0][1], frame + 0x1e4);
        assert_eq!(
            logged(&e, VECTOR_FLOAT_PUSH_BACK),
            vec![vec![spline + 0x3c, frame + 0x294]]
        );
        assert_eq!(e.mem.f32(frame + 0x294), 0.5);
    }

    #[test]
    fn parse_keeps_the_token_and_the_numbers_at_the_exe_frame_offsets() {
        let (mut e, _) = world();
        let spline = new_spline(&mut e);
        let text = put_string(&mut e, SPLINE_TEXT);
        let frame = next_frame(&mut e, 0x12c);
        e.call(0x00b3_7cc0, &args![spline, text]);
        assert_eq!(logged(&e, STRCMP)[0][0], frame + 0x24);
        let added = logged(&e, SPLINE_ADD_POINT);
        assert_eq!(added.len(), 2);
        assert_eq!(added[0][1..3], [frame + 8, frame + 0x14]);
    }

    #[test]
    fn the_point_at_t_and_the_sample_table_use_the_exe_frame_offsets() {
        let (mut e, _) = world();
        let spline = two_point_spline(&mut e);
        let out = e.mem.alloc(0x18);
        let frame = next_frame(&mut e, 0xb8);
        e.call(0x00b3_8b50, &args![spline, out, 0.5f32]);
        assert_eq!(logged(&e, CVEC_SIZED)[0][0], frame + 0x90);
        assert_eq!(logged(&e, 0x00b3_8f40)[0][1], frame + 0x64);
        let frame = next_frame(&mut e, 0x194);
        e.call(0x00b3_8430, &args![spline, 4u32]);
        assert_eq!(logged(&e, VECTOR_CVEC_CONSTRUCT)[0][0], frame + 0x170);
        let blanks: Vec<u32> = logged(&e, CVEC_DEFAULT).iter().map(|c| c[0]).collect();
        assert_eq!(blanks, vec![frame + 0x10c, frame + 0xc4]);
        let points_at = logged(&e, SPLINE_POINT_AT);
        assert_eq!(points_at.len(), 4);
        assert!(points_at.iter().all(|c| c[1] == frame + 0x12c));
        let samples = logged(&e, CVEC_FROM_FIVE);
        assert_eq!(samples.len(), 2);
        assert!(samples.iter().all(|c| c[0] == frame + 0x14c));
    }

    #[test]
    fn the_vector_helpers_use_the_exe_frame_offsets() {
        let (mut e, _) = world();
        // push_back without spare room: the insert's result at +0x48 of 0x60 bytes, the
        // insert's advanced iterator at +0x10 of its 0x34 bytes.
        let full = cvec_vector(&mut e, &[1.0], 1);
        let value = cvec(&mut e, &[2.0]);
        // The insert (`00b396c0`) runs inside push_back's frame, so its 0x34 bytes sit below it.
        let insert_frame = nested_frame(&mut e, 0x60, 0x34);
        let frame = next_frame(&mut e, 0x60);
        e.call(0x00b3_9100, &args![full, value]);
        assert_eq!(logged(&e, VECTOR_CVEC_INSERT_ONE)[0][1], frame + 0x48);
        assert_eq!(logged(&e, ITERATOR_ADVANCE)[0][0], insert_frame + 0x10);
        // resize: the value in its argument slot (+0xd4), the iterator at +0x68 and the
        // erase result at +0x7c.
        let vector = cvec_vector(&mut e, &[1.0, 2.0], 8);
        let fill = CVecArg([7.0f32.to_bits(), 0, 0, 0, 0, 2]);
        let frame = next_frame(&mut e, 0xec);
        e.call(0x00b3_9490, &args![vector, 3u32, fill]);
        assert_eq!(logged(&e, VECTOR_CVEC_INSERT_N)[0][4], frame + 0xd4);
        let frame = next_frame(&mut e, 0xec);
        e.call(0x00b3_9490, &args![vector, 1u32, fill]);
        assert_eq!(logged(&e, ITERATOR_ADVANCE)[0][0], frame + 0x68);
        assert_eq!(logged(&e, VECTOR_CVEC_ERASE)[0][1], frame + 0x7c);
        // insert_n with a tail shorter than the count: the copy at +0x224.
        let vector = cvec_vector(&mut e, &[1.0, 2.0], 8);
        let proxy = e.mem.u32(vector);
        let last = e.mem.u32(vector + 0x10);
        let frame = next_frame(&mut e, 0x258);
        e.call(0x00b3_a0d0, &args![vector, proxy, last, 2u32, value]);
        assert_eq!(logged(&e, VECTOR_CVEC_FILL)[0][2], frame + 0x224);
    }

    #[test]
    fn the_map_helpers_use_the_exe_frame_offsets() {
        let (mut e, _) = world();
        // The constructor pushes the address of its byte at +0x1e as the allocator word.
        let map = e.mem.alloc(0x20);
        let comparator = e.mem.alloc(4);
        let frame = next_frame(&mut e, 0x4c);
        e.call(0x00b3_b770, &args![map, comparator, 0u32]);
        assert_eq!(logged(&e, TREE_PROXY_NEW)[0][2], frame + 0x1e);
        // operator[] of a new key: the iterator at +0x78, the pair at +0x50, the insert
        // result at +0x3c.
        let text = put_string(&mut e, "a");
        let frame = next_frame(&mut e, 0x8c);
        e.call(0x00b3_9210, &args![map, text]);
        let inserted = logged(&e, MAP_INSERT);
        assert_eq!(
            (inserted[0][1], inserted[0][4]),
            (frame + 0x3c, frame + 0x50)
        );
        assert_eq!(logged(&e, MAP_ITERATOR_VALUE)[0][0], frame + 0x78);
        // The unique insert: `j` at +0xa4 of its 0xb8 bytes.
        let pair = put_pair(&mut e, "b", 1);
        let out = e.mem.alloc(0xc);
        let frame = next_frame(&mut e, 0xb8);
        e.call(0x00b3_a9b0, &args![map, out, pair]);
        assert_eq!(logged(&e, MAP_ITERATOR_NEW)[0][0], frame + 0xa4);
        // Erasing one node steps the iterator argument in place (+0x118).
        let head = map_head_of(&e, map);
        let first = e.mem.u32(head);
        let proxy = e.mem.u32(map);
        let frame = next_frame(&mut e, 0x120);
        e.call(0x00b3_bd80, &args![map, out, proxy, first]);
        assert_eq!(logged(&e, MAP_ITERATOR_INCREMENT)[0][0], frame + 0x118);
        // Erasing a range that is not the whole tree: the cursor is the `first` argument
        // (+0x68), each erase result at +0x3c.
        for name in ["c", "d"] {
            let pair = put_pair(&mut e, name, 1);
            e.call(0x00b3_a9b0, &args![map, out, pair]);
        }
        let head = map_head_of(&e, map);
        let begin = e.mem.u32(head);
        let frame = next_frame(&mut e, 0x78);
        e.call(
            0x00b3_bb90,
            &args![map, out, proxy, begin, proxy, e.mem.u32(head + 8)],
        );
        let erased = logged(&e, TREE_ERASE_ONE);
        assert!(!erased.is_empty());
        assert!(erased.iter().all(|c| c[1] == frame + 0x3c));
        assert_eq!(logged(&e, MAP_ITERATOR_INCREMENT)[0][0], frame + 0x68);
        // Clearing everything: the erase result at +0x64 of 0x8c bytes.
        let frame = next_frame(&mut e, 0x8c);
        e.call(0x00b3_ba90, &args![map]);
        assert_eq!(logged(&e, TREE_ERASE_RANGE)[0][1], frame + 0x64);
    }

    #[test]
    fn clearing_the_cache_keeps_both_iterators_at_the_exe_frame_offsets() {
        let (mut e, _) = cache_world();
        cached(&mut e, SPLINE_TEXT);
        let frame = next_frame(&mut e, 0x3c);
        e.call(0x00b3_7a00, &args![]);
        assert_eq!(logged(&e, MAP_BEGIN)[0][1], frame + 0x34);
        let compares = logged(&e, ITERATOR_NOT_EQUAL);
        assert!(!compares.is_empty());
        assert!(compares
            .iter()
            .all(|c| c[..2] == [frame + 0x34, frame + 0x2c]));
    }

    #[test]
    fn the_hint_insert_keeps_its_iterators_at_the_exe_frame_offsets() {
        let (mut e, _) = world();
        let map = new_map(&mut e);
        for k in [20, 40] {
            index(&mut e, map, &key(k));
        }
        let head = map_head_of(&e, map);
        let proxy = e.mem.u32(map);
        // A key below the last one with the end as the hint: the cheap cases fail and the
        // plain insert runs.
        let pair = put_pair(&mut e, &key(30), 1);
        let out = e.mem.alloc(8);
        let frame = next_frame(&mut e, 0x1d8);
        e.call(0x00b3_98c0, &args![map, out, proxy, head, pair]);
        let made: Vec<u32> = logged(&e, MAP_ITERATOR_NEW)
            .iter()
            .take(2)
            .map(|c| c[0])
            .collect();
        assert_eq!(made, vec![frame + 0x198, frame + 0x190]);
        let compared: Vec<Vec<u32>> = logged(&e, ITERATOR_EQUAL)
            .iter()
            .take(2)
            .map(|c| c[..2].to_vec())
            .collect();
        assert_eq!(
            compared,
            vec![
                vec![frame + 0x1cc, frame + 0x198],
                vec![frame + 0x1cc, frame + 0x190]
            ]
        );
        assert_eq!(
            logged(&e, TREE_INSERT_UNIQUE)[0][..3],
            [map, frame + 0x17c, pair]
        );
        assert_eq!(e.mem.u32(frame + 0x1d0), head);
    }
}
