//! `SpeedTree/run_00b2e250` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Part of `CIndexedGeometry` and its helpers, identified from the code (the engine map gives
//! them no names): the `std::vector` instantiations of the 0x54-byte element (an erase of a
//! range, allocation, element assignment and copy construction, and the range helpers), the
//! `CIndexedGeometry` constructor and destructor, the joining of every detail level's strips
//! into one strip (`00b2ed70`), and the extents computation over the vertices (`00b2fa30`).
//!
//! The 0x54-byte element (unconfirmed role) is a byte at `+0`, three words at `+4`, a float at
//! `+0x10`, a `std::string` at `+0x14` and three groups of three words at `+0x30`, `+0x3C` and
//! `+0x48`.

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{
    copy_float, StlVector, INVALID_PARAMETER, MEMCPY, OPERATOR_DELETE, OPERATOR_NEW,
};
use super::indexedgeometry::{CIndexedGeometry, CWindEngine};
use super::run_00b0eca0::advance_checked_iterator;

/// `_CxxThrowException(object, throw_info)` (`00ec7894`).
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// `std::exception::exception(const char *const &)` (`00ec78f1`, `thiscall`), the base of
/// `std::bad_alloc`, its vtable (`010176f8`) and its throw info (`01140ef8`).
const EXCEPTION_FROM_MESSAGE: u32 = 0x00ec_78f1;
const BAD_ALLOC_VTABLE: u32 = 0x0101_76f8;
const BAD_ALLOC_THROW_INFO: u32 = 0x0114_0ef8;
/// `std::string::assign(const string &, pos, count)` (`0044a580`, `thiscall`), the string's
/// `_Tidy(built, new_size)` (`0044a730`, `thiscall`) and `std::string::npos` (`010173f0`, read
/// from memory).
const STRING_ASSIGN: u32 = 0x0044_a580;
const STRING_TIDY: u32 = 0x0044_a730;
const STRING_NPOS: u32 = 0x0101_73f0;
/// The `iterator !=` of the checked vector iterators (`00b16830`, `thiscall`: the iterator, the
/// other one; returns a byte).
const ITERATOR_NOT_EQUAL: u32 = 0x00b1_6830;
/// `_Destroy(first, last)` of the 0x54-byte elements (`00b2e140`, `thiscall`).
const DESTROY_ELEMENTS: u32 = 0x00b2_e140;
/// The empty destructor of the plain-data members (`00aa7830`, `thiscall`, `this` only).
const TRIVIAL_DESTRUCTOR: u32 = 0x00aa_7830;
/// The `CIndexedGeometry` helpers outside this file: the vertex's coordinate triple (`00b30f80`,
/// `thiscall`: the vertex index), the reset of the vertex counters (`00b30da0`), and the
/// extents growth by one point (`00b3caa0`, `thiscall` on the extents: a result slot and the
/// point).
const COORD_POINTER: u32 = 0x00b3_0f80;
const RESET_COUNTERS: u32 = 0x00b3_0da0;
const EXTENTS_INCLUDE: u32 = 0x00b3_caa0;
/// The vector constructors/destructors of the other units that the constructor and destructor
/// of `CIndexedGeometry` call on their members (`thiscall`).
const CONSTRUCT_POINTER_VECTOR: u32 = 0x00b1_7db0;
const CONSTRUCT_STRIP_VECTOR: u32 = 0x00b3_1270;
const CONSTRUCT_COLOR_VECTOR: u32 = 0x00ab_a4f0;
const CONSTRUCT_FLOAT_VECTOR: u32 = 0x0044_9fd0;
const CONSTRUCT_BYTE_VECTOR: u32 = 0x0065_7b50;
const TIDY_BYTE_VECTOR: u32 = 0x0065_7f60;
const TIDY_WORD_VECTOR: u32 = 0x0065_f240;
const TIDY_STRIPS: u32 = 0x00b3_31c0;
const TIDY_STRIP_LENGTHS: u32 = 0x00b3_2c60;
const TIDY_POINTER_VECTOR: u32 = 0x00b1_7f50;
const FREE_PROXY: u32 = 0x00b3_9460;
/// What `00b2ed70` calls on its working vectors: the three sized constructors (strip lengths
/// `00b31350`, strips `00b31ad0`, triangle counts `00b311b0`), the push of an unsigned short
/// (`00b32410`), the push of an int (`00b15390`), the clear of a vector of unsigned shorts
/// (`00b32540`) and of ints (`00b32350`), the assignment of the three vectors (`00b314f0`,
/// `00b31c70`, `00b36330`) and the checked indexing of a vector of vectors (`00b319a0`) and of
/// ints (`00b31230`).
const SIZED_STRIP_LENGTHS: u32 = 0x00b3_1350;
const SIZED_STRIPS: u32 = 0x00b3_1ad0;
const SIZED_COUNTS: u32 = 0x00b3_11b0;
const PUSH_SHORT: u32 = 0x00b3_2410;
const PUSH_INT: u32 = 0x00b1_5390;
const CLEAR_SHORTS: u32 = 0x00b3_2540;
const CLEAR_INTS: u32 = 0x00b3_2350;
const ASSIGN_STRIP_LENGTHS: u32 = 0x00b3_14f0;
const ASSIGN_STRIPS: u32 = 0x00b3_1c70;
const ASSIGN_COUNTS: u32 = 0x00b3_6330;
const VECTOR_OF_VECTORS_AT: u32 = 0x00b3_19a0;
const INT_VECTOR_AT: u32 = 0x00b3_1230;
/// The scalar-deleting-destructor flag the exe folds to 0 where an inlined destructor call
/// could free its object.
const DESTRUCTOR_FREES: bool = false;

/// The size of the `CIndexedGeometry` vector members at `+0x40` (strip lengths), `+0x58`
/// (strips), `+0x28` (triangle counts) and `+0x88` (coordinates).
const STRIP_LENGTHS_AT: u32 = 0x40;
const STRIPS_AT: u32 = 0x58;
const TRIANGLE_COUNTS_AT: u32 = 0x28;
const COORDS_AT: u32 = 0x88;

/// The number of 0x54-byte elements between two pointers (signed division).
fn elements_between(first: u32, last: u32) -> u32 {
    ((last.wrapping_sub(first)) as i32 / 0x54) as u32
}

/// The debug iterator check the vector code makes: the invalid-parameter handler runs when the
/// vector is null or `position` is outside `[first, last]`.
fn check_position(e: &mut Engine, vector: u32, position: u32) {
    if vector == 0
        || e.mem.u32(vector.wrapping_add(0x0C)) > position
        || position > e.mem.u32(vector.wrapping_add(0x10))
    {
        e.call(INVALID_PARAMETER, &args![]);
    }
}

/// Copies three words from `source` to `dest`.
fn copy_triple(e: &mut Engine, dest: u32, source: u32) {
    for word in 0..3u32 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(dest.wrapping_add(4 * word), value);
    }
}

/// The fields both element copies share, in the exe's order up to the string.
fn copy_element_head(e: &mut Engine, dest: u32, source: u32) {
    let byte = e.mem.u8(source);
    e.mem.set_u8(dest, byte);
    copy_triple(e, dest + 4, source + 4);
    copy_float(e, dest + 0x10, source + 0x10);
}

/// The string assignment `dest.string = source.string` of the element copies (the whole string:
/// position 0 and `npos`).
fn assign_element_string(e: &mut Engine, dest: u32, source: u32) {
    let npos = e.global::<u32>(STRING_NPOS);
    e.call(
        STRING_ASSIGN,
        &args![dest + 0x14, source + 0x14, 0u32, npos],
    );
}

/// The three trailing groups of three words of the element copies.
fn copy_element_tail(e: &mut Engine, dest: u32, source: u32) {
    copy_triple(e, dest + 0x30, source + 0x30);
    copy_triple(e, dest + 0x3C, source + 0x3C);
    copy_triple(e, dest + 0x48, source + 0x48);
}

/// The size of the number of strips of level `level` (as a signed 16-bit index): 0 for a
/// negative level, otherwise the element count of `strips[level]` (the pointer vector, 4-byte
/// elements) truncated to 16 bits; `00b319a0` does the checked indexing.
fn strip_count(e: &mut Engine, this: u32, level: u16) -> u16 {
    if (level as i16) > -1 {
        let row = e
            .call(
                VECTOR_OF_VECTORS_AT,
                &args![this + STRIPS_AT, (level as i16) as i32],
            )
            .u32();
        ((e.mem.u32(row + 0x10).wrapping_sub(e.mem.u32(row + 0x0C))) as i32 >> 2) as u16
    } else {
        0
    }
}

/// Element `index` of the 0x18-byte-element vector at `vector` with the exe's inline check (an
/// index not below the element count is an invalid parameter); returns its address.
fn row_of(e: &mut Engine, vector: u32, index: u32) -> u32 {
    let count = ((e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C))) as i32
        / 0x18) as u32;
    if index >= count {
        e.call(INVALID_PARAMETER, &args![]);
    }
    index
        .wrapping_mul(0x18)
        .wrapping_add(e.mem.u32(vector + 0x0C))
}

/// Element `index` of a vector of unsigned shorts at `row` with the inline check.
fn short_in_row(e: &mut Engine, row: u32, index: u32) -> u16 {
    let count = ((e.mem.u32(row + 0x10).wrapping_sub(e.mem.u32(row + 0x0C))) as i32 >> 1) as u32;
    if index >= count {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem
        .u16(e.mem.u32(row + 0x0C).wrapping_add(index.wrapping_mul(2)))
}

/// Element `index` of a vector of pointers at `row` with the inline check.
fn pointer_in_row(e: &mut Engine, row: u32, index: u32) -> u32 {
    let count = ((e.mem.u32(row + 0x10).wrapping_sub(e.mem.u32(row + 0x0C))) as i32 >> 2) as u32;
    if index >= count {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem
        .u32(e.mem.u32(row + 0x0C).wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00b2e250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The erase of a range of the 0x54-byte element vector (identified from the code; `thiscall`:
/// a result slot, the first iterator's proxy and position, and the end iterator's proxy and
/// position; the proxies are not read): both positions must be inside `[first, last]` of the
/// vector (the invalid-parameter path otherwise); when the iterators differ (`00b16830`) the
/// elements from the end position on are assigned down to the first position (`00b2e3f0`), the
/// vacated tail destroyed (`00b2e140`) and the last pointer lowered. Writes the first
/// iterator (the vector's proxy and the first position) to the result slot and returns it.
pub fn fn_00b2e250(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    _unused_1: u32,
    first_position: u32,
    _unused_3: u32,
    last_position: u32,
) -> Ptr {
    let vector = this.addr();
    check_position(e, vector, first_position);
    let first_proxy = e.mem.u32(vector);
    check_position(e, vector, last_position);
    let last_proxy = e.mem.u32(vector);
    let moved = e.with_stack(0x10, |e, iterators| {
        // The first iterator at +0 and the end iterator at +8 (a proxy and a position each).
        e.mem.set_u32(iterators.addr(), first_proxy);
        e.mem.set_u32(iterators.addr() + 4, first_position);
        e.mem.set_u32(iterators.addr() + 8, last_proxy);
        e.mem.set_u32(iterators.addr() + 12, last_position);
        e.call(ITERATOR_NOT_EQUAL, &args![iterators, iterators.byte_add(8)])
            .bool()
    });
    if moved {
        let last = e.mem.u32(vector + 0x10);
        let new_last = fn_00b2e3f0(e, last_position, last, first_position);
        let last = e.mem.u32(vector + 0x10);
        e.call(DESTROY_ELEMENTS, &args![this, new_last, last]);
        e.mem.set_u32(vector + 0x10, new_last);
    }
    e.mem.set_u32(out.addr(), first_proxy);
    e.mem.set_u32(out.addr() + 4, first_position);
    out
}

// Translated from 00b2e330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked iterator's `operator+=(n)` for 4-byte elements (identified from the code; see
/// `00b104f0`): moves the element pointer by `n * 4` bytes; an iterator without a proxy, or a
/// target outside the container's `[first, last]`, takes the invalid-parameter path. Returns
/// `this`.
pub fn fn_00b2e330(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    advance_checked_iterator(e, this.addr(), count.wrapping_mul(4));
    this
}

// Translated from 00b2e3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the 0x54-byte elements `first..last` over the ones starting at `dest` (identified
/// from the code; `cdecl`; the element assignment is `00b2e680`) and returns `dest` plus the
/// whole elements' size.
pub fn fn_00b2e3f0(e: &mut Engine, first: u32, last: u32, dest: u32) -> u32 {
    let end = elements_between(first, last)
        .wrapping_mul(0x54)
        .wrapping_add(dest);
    let mut destination = dest;
    let mut source = first;
    while source != last {
        fn_00b2e680(e, Ptr::new(destination), Ptr::new(source));
        destination = destination.wrapping_add(0x54);
        source = source.wrapping_add(0x54);
    }
    end
}

// Translated from 00b2e4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocation of the 0x54-byte element vector (identified from the code;
/// `std::_Allocate(count, hint)`, the hint unused): `operator new(count * 0x54)`. A count of 0
/// allocates 0 bytes; a count too large for the multiplication throws `std::bad_alloc`.
pub fn fn_00b2e4a0(e: &mut Engine, count: u32, _unused_1: u32) -> u32 {
    // A count of 0 skips the overflow test and allocates 0 bytes.
    if count != 0 && 0xFFFF_FFFFu32 / count < 0x54 {
        e.with_stack(4, |e, message| {
            e.mem.set_u32(message.addr(), 0);
            e.with_stack(0xC, |e, exception| {
                e.call(EXCEPTION_FROM_MESSAGE, &args![exception, message]);
                e.mem.set_u32(exception.addr(), BAD_ALLOC_VTABLE);
                e.call(CXX_THROW_EXCEPTION, &args![exception, BAD_ALLOC_THROW_INFO]);
            });
        });
        unreachable!("_CxxThrowException does not return (00ec7894)")
    }
    e.call(OPERATOR_NEW, &args![count.wrapping_mul(0x54)]).u32()
}

// Translated from 00b2e500 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same assignment of 0x54-byte elements as `00b2e3f0` (identified from the code; another
/// copy of the same template function): assigns `first..last` over the elements at `dest` and
/// returns `dest` plus the whole elements' size.
pub fn fn_00b2e500(e: &mut Engine, first: u32, last: u32, dest: u32) -> u32 {
    let end = elements_between(first, last)
        .wrapping_mul(0x54)
        .wrapping_add(dest);
    let mut destination = dest;
    let mut source = first;
    while source != last {
        fn_00b2e680(e, Ptr::new(destination), Ptr::new(source));
        destination = destination.wrapping_add(0x54);
        source = source.wrapping_add(0x54);
    }
    end
}

// Translated from 00b2e570 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2e570 - the FS:[0] exception frame is not translated.
/// `std::_Uninit_copy` for the 0x54-byte element (identified from the code; `cdecl`, `first`,
/// `last`, `dest`, the allocator and two tag bytes not read): copy-constructs each element of
/// `first..last` at `dest` (`00b2e740`), 0x54 bytes apart; a null destination is skipped.
/// Returns the end of the destination.
pub fn fn_00b2e570(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let mut source = first;
    let mut destination = dest;
    while source != last {
        if destination != 0 {
            fn_00b2e740(e, Ptr::new(destination), Ptr::new(source));
        }
        destination = destination.wrapping_add(0x54);
        source = source.wrapping_add(0x54);
    }
    destination
}

// Translated from 00b2e680 (decompiled, FalloutNV.exe 1.4.0.525)
/// The assignment of the 0x54-byte element (identified from the code; `thiscall`): copies the
/// byte, the three words at `+4`, the float at `+0x10` (through the x87 stack), assigns the
/// string at `+0x14` (`0044a580`, the whole string) and copies the three groups of three words
/// at `+0x30`, `+0x3C` and `+0x48`. Returns `this`.
pub fn fn_00b2e680(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    copy_element_head(e, this.addr(), other.addr());
    assign_element_string(e, this.addr(), other.addr());
    copy_element_tail(e, this.addr(), other.addr());
    this
}

// Translated from 00b2e740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The copy construction of the 0x54-byte element (identified from the code; `thiscall`): the
/// assignment `00b2e680` with the string first set to the empty string (`0044a730` with 0 and
/// 0) instead of being assumed constructed. Returns `this`.
pub fn fn_00b2e740(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    copy_element_head(e, this.addr(), other.addr());
    e.call(STRING_TIDY, &args![this.addr() + 0x14, 0u32, 0u32]);
    assign_element_string(e, this.addr(), other.addr());
    copy_element_tail(e, this.addr(), other.addr());
    this
}

// Translated from 00b2e810 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2e810 - the FS:[0] exception frame (destroying the constructed
// members if a later constructor throws) is not translated.
/// `CIndexedGeometry::CIndexedGeometry(wind engine, retain texture coordinates)` (identified
/// from the code and the Xbox PDB layout): stores the flag (`+0`) and the wind engine (`+4`),
/// clears the weighting and lighting flags, the wind-computed pointer, the vertex size, sets
/// `m_bValid` to 1 and the wind method to 2, clears the level and vertex counters and
/// constructs the vector members (`+0x28` the triangle counts, `+0x40` and `+0x58` the strip
/// vectors, `+0x70` the colours, `+0x88`..`+0x118` the float vectors, `+0x130` and `+0x178`
/// the byte vectors, `+0x148` and `+0x160` float vectors). Returns `this`.
pub fn fn_00b2e810(
    e: &mut Engine,
    this: Ptr<CIndexedGeometry>,
    wind_engine: Ptr<CWindEngine>,
    retain_tex_coords: u8,
) -> Ptr<CIndexedGeometry> {
    e.set(
        this,
        CIndexedGeometry::m_bRetainTexCoords,
        retain_tex_coords,
    );
    e.set(this, CIndexedGeometry::m_pWindEngine, wind_engine);
    e.set(this, CIndexedGeometry::m_bVertexWeighting, 0);
    e.set(this, CIndexedGeometry::m_bManualLighting, 0);
    e.set(this, CIndexedGeometry::m_pVertexWindComputed, Ptr::NULL);
    e.set(this, CIndexedGeometry::m_nVertexSize, 0);
    e.set(this, CIndexedGeometry::m_bValid, 1);
    e.set(this, CIndexedGeometry::m_eWindMethod, 2);
    e.set(this, CIndexedGeometry::m_nNumLodLevels, 0);
    e.set(this, CIndexedGeometry::m_nVertexCounter, 0);
    e.set(this, CIndexedGeometry::m_nVertexCounterLodLevel, 0);
    e.set(this, CIndexedGeometry::m_nStripCounter, 0);
    let base = this.addr();
    for (constructor, offset) in [
        (CONSTRUCT_POINTER_VECTOR, 0x28u32),
        (CONSTRUCT_STRIP_VECTOR, 0x40),
        (CONSTRUCT_STRIP_VECTOR, 0x58),
        (CONSTRUCT_COLOR_VECTOR, 0x70),
        (CONSTRUCT_FLOAT_VECTOR, 0x88),
        (CONSTRUCT_FLOAT_VECTOR, 0xA0),
        (CONSTRUCT_FLOAT_VECTOR, 0xB8),
        (CONSTRUCT_FLOAT_VECTOR, 0xD0),
        (CONSTRUCT_FLOAT_VECTOR, 0xE8),
        (CONSTRUCT_FLOAT_VECTOR, 0x100),
        (CONSTRUCT_FLOAT_VECTOR, 0x118),
        (CONSTRUCT_BYTE_VECTOR, 0x130),
        (CONSTRUCT_FLOAT_VECTOR, 0x148),
        (CONSTRUCT_FLOAT_VECTOR, 0x160),
        (CONSTRUCT_BYTE_VECTOR, 0x178),
    ] {
        e.call(constructor, &args![base + offset]);
    }
    this
}

// Translated from 00b2e9c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2e9c0 - the FS:[0] exception frame is not translated.
/// `CIndexedGeometry::~CIndexedGeometry` (identified from the code; `fastcall`): frees the
/// wind-computed array (`+0xC`, set to null), resets the counters (`00b30da0`), tidies each
/// vector member from the last to the first (the byte vectors with `00657f60`, the
/// word vectors with `0065f240`, the strips with `00b331c0`) and frees its container proxy, and
/// tidies and frees the proxies of the two strip-length and triangle-count vectors
/// (`00b32c60`, `00b17f50` then `00b39460`).
pub fn fn_00b2e9c0(e: &mut Engine, this: Ptr<CIndexedGeometry>) {
    let base = this.addr();
    let wind_computed = e.mem.u32(base + 0x0C);
    e.call(OPERATOR_DELETE, &args![wind_computed]);
    e.mem.set_u32(base + 0x0C, 0);
    e.call(RESET_COUNTERS, &args![this]);
    for (tidy, offset) in [
        (TIDY_BYTE_VECTOR, 0x178u32),
        (TIDY_WORD_VECTOR, 0x160),
        (TIDY_WORD_VECTOR, 0x148),
        (TIDY_BYTE_VECTOR, 0x130),
        (TIDY_WORD_VECTOR, 0x118),
        (TIDY_WORD_VECTOR, 0x100),
        (TIDY_WORD_VECTOR, 0xE8),
        (TIDY_WORD_VECTOR, 0xD0),
        (TIDY_WORD_VECTOR, 0xB8),
        (TIDY_WORD_VECTOR, 0xA0),
        (TIDY_WORD_VECTOR, 0x88),
        (TIDY_WORD_VECTOR, 0x70),
        (TIDY_STRIPS, 0x58),
    ] {
        e.call(tidy, &args![base + offset]);
        let proxy = e.mem.u32(base + offset);
        e.call(OPERATOR_DELETE, &args![proxy]);
    }
    for (tidy, offset) in [(TIDY_STRIP_LENGTHS, 0x40u32), (TIDY_POINTER_VECTOR, 0x28)] {
        e.call(tidy, &args![base + offset]);
        e.call(FREE_PROXY, &args![base + offset]);
        if DESTRUCTOR_FREES {
            // The destructor's delete flag is the constant 0, so the member is never freed
            // here.
            e.call(OPERATOR_DELETE, &args![base + offset]);
        }
    }
}

// Translated from 00b2ed70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2ed70 - the FS:[0] exception frame (destroying the working vectors
// if a step throws) is not translated.
/// Joins the strips of every detail level into one strip per level (identified from the code;
/// the engine map gives it no name; the argument is a byte flag): nothing happens without
/// vertices (`m_vCoords` floats / 3 truncated to 16 bits). Otherwise new strip-length, strip and
/// triangle-count vectors are built for the `m_nNumLodLevels` levels and, per level, the sum of
/// the strip lengths decides: a level without indices gets empty rows and a count of 0; with
/// indices a block of `2 * (sum + joins)` bytes (`operator new`) is filled with the strips'
/// indices one after the other, between two strips `joins` indices that repeat the last
/// index of the first and the first index of the next (two, or with the flag set and a first
/// strip of odd length three, repeating the last index twice), the new length is pushed to the
/// level's row of lengths, the block to its row of strips and the new length minus two is
/// stored as the level's count. At the end the vertex counters are reset (`00b30da0`) and the
/// geometry's `m_vStripLengths`, `m_vStrips` and `m_vTriangleCounts` are assigned the new
/// vectors; the working vectors are then destroyed.
pub fn fn_00b2ed70(e: &mut Engine, this: Ptr<CIndexedGeometry>, flag: u8) {
    let base = this.addr();
    let coords = base + COORDS_AT;
    let vertices = (((e
        .mem
        .u32(coords + 0x10)
        .wrapping_sub(e.mem.u32(coords + 0x0C))) as i32
        >> 2) as u32
        / 3) as u16;
    if vertices == 0 {
        return;
    }
    let levels = u32::from(e.get(this, CIndexedGeometry::m_nNumLodLevels));
    e.with_stack(0x18, |e, lengths| {
        e.call(SIZED_STRIP_LENGTHS, &args![lengths, levels]);
        e.with_stack(0x18, |e, strips| {
            e.call(SIZED_STRIPS, &args![strips, levels]);
            e.with_stack(0x18, |e, counts| {
                let levels = u32::from(e.get(this, CIndexedGeometry::m_nNumLodLevels));
                e.call(SIZED_COUNTS, &args![counts, levels]);
                join_levels(
                    e,
                    this,
                    flag,
                    (lengths.addr(), strips.addr(), counts.addr()),
                );
                e.call(RESET_COUNTERS, &args![this]);
                e.call(
                    ASSIGN_STRIP_LENGTHS,
                    &args![base + STRIP_LENGTHS_AT, lengths],
                );
                e.call(ASSIGN_STRIPS, &args![base + STRIPS_AT, strips]);
                e.call(ASSIGN_COUNTS, &args![base + TRIANGLE_COUNTS_AT, counts]);
                // The destructors of the working vectors, last constructed first.
                e.call(TIDY_POINTER_VECTOR, &args![counts]);
                e.call(FREE_PROXY, &args![counts]);
                e.call(TIDY_STRIPS, &args![strips]);
                e.call(FREE_PROXY, &args![strips]);
                e.call(TIDY_STRIP_LENGTHS, &args![lengths]);
                e.call(FREE_PROXY, &args![lengths]);
            });
        });
    });
}

/// The per-level loop of `00b2ed70`: `(lengths, strips, counts)` are the working vectors (the
/// new row of lengths and of strips per level and the count per level).
fn join_levels(e: &mut Engine, this: Ptr<CIndexedGeometry>, flag: u8, working: (u32, u32, u32)) {
    let base = this.addr();
    let (lengths, strips, counts) = working;
    let mut level: u16 = 0;
    while u32::from(level) < u32::from(e.get(this, CIndexedGeometry::m_nNumLodLevels)) {
        let mut total: u32 = 0;
        let mut strip: u16 = 0;
        while u32::from(strip) < u32::from(strip_count(e, base, level)) {
            let row = row_of(e, base + STRIP_LENGTHS_AT, u32::from(level));
            let length = short_in_row(e, row, u32::from(strip));
            total = total.wrapping_add(u32::from(length));
            strip = strip.wrapping_add(1);
        }
        if (total as i32) <= 0 {
            let row = e
                .call(VECTOR_OF_VECTORS_AT, &args![lengths, u32::from(level)])
                .u32();
            e.call(CLEAR_SHORTS, &args![row]);
            let row = e
                .call(VECTOR_OF_VECTORS_AT, &args![strips, u32::from(level)])
                .u32();
            e.call(CLEAR_INTS, &args![row]);
            let slot = e
                .call(INT_VECTOR_AT, &args![counts, u32::from(level)])
                .u32();
            e.mem.set_u32(slot, 0);
        } else {
            if flag == 0 {
                let mut joins = u32::from(strip_count(e, base, level));
                if (joins as i32) > 0 {
                    joins = joins.wrapping_sub(1);
                }
                total = total.wrapping_add(joins.wrapping_mul(2));
            } else {
                let mut strip: u16 = 0;
                while (i32::from(strip)) < i32::from(strip_count(e, base, level)) - 1 {
                    let row = row_of(e, base + STRIP_LENGTHS_AT, u32::from(level));
                    let length = short_in_row(e, row, u32::from(strip));
                    total = total.wrapping_add(if (length & 1) == 0 { 2 } else { 3 });
                    strip = strip.wrapping_add(1);
                }
            }
            // `operator new(total * 2)`, all ones when the product overflows 32 bits.
            let bytes = u64::from(total) * 2;
            let size = if bytes >> 32 != 0 {
                0xFFFF_FFFF
            } else {
                bytes as u32
            };
            let block = e.call(OPERATOR_NEW, &args![size]).u32();
            let mut cursor = block;
            let mut strip: u16 = 0;
            while u32::from(strip) < u32::from(strip_count(e, base, level)) {
                let row = row_of(e, base + STRIP_LENGTHS_AT, u32::from(level));
                let length = short_in_row(e, row, u32::from(strip));
                let row = row_of(e, base + STRIPS_AT, u32::from(level));
                let source = pointer_in_row(e, row, u32::from(strip));
                e.call(MEMCPY, &args![cursor, source, u32::from(length) << 1]);
                cursor = cursor.wrapping_add(u32::from(length).wrapping_mul(2));
                if i32::from(strip) < i32::from(strip_count(e, base, level)) - 1 {
                    // The last index of this strip, then the first of the next; a strip of odd
                    // length (with the flag set) repeats its last index once more.
                    let repeat_last = flag != 0 && (length & 1) != 0;
                    stitch(e, base, (level, strip, length), &mut cursor, repeat_last);
                }
                strip = strip.wrapping_add(1);
            }
            let new_length = total as u16;
            let row = row_of(e, lengths, u32::from(level));
            e.with_stack(4, |e, value| {
                e.mem.set_u16(value.addr(), new_length);
                e.call(PUSH_SHORT, &args![row, value]);
            });
            let row = row_of(e, strips, u32::from(level));
            e.with_stack(4, |e, value| {
                e.mem.set_u32(value.addr(), block);
                e.call(PUSH_INT, &args![row, value]);
            });
            let count = ((e
                .mem
                .u32(counts + 0x10)
                .wrapping_sub(e.mem.u32(counts + 0x0C))) as i32
                >> 2) as u32;
            if u32::from(level) >= count {
                e.call(INVALID_PARAMETER, &args![]);
            }
            let slot = e
                .mem
                .u32(counts + 0x0C)
                .wrapping_add(u32::from(level).wrapping_mul(4));
            e.mem.set_u32(slot, total.wrapping_sub(2));
        }
        level = level.wrapping_add(1);
    }
}

/// The join of two strips: writes the last index of strip `strip` of `level` (`length` is its
/// length), once more when `repeat_last`, and then the first index of the next strip, advancing
/// `cursor` by 2 bytes for each. Each index is read through the inline checks of the strips.
fn stitch(
    e: &mut Engine,
    base: u32,
    (level, strip, length): (u16, u16, u16),
    cursor: &mut u32,
    repeat_last: bool,
) {
    let writes = if repeat_last { 2 } else { 1 };
    for _ in 0..writes {
        let row = row_of(e, base + STRIPS_AT, u32::from(level));
        let source = pointer_in_row(e, row, u32::from(strip));
        let last = e.mem.u16(
            source
                .wrapping_add(u32::from(length).wrapping_mul(2))
                .wrapping_sub(2),
        );
        e.mem.set_u16(*cursor, last);
        *cursor = cursor.wrapping_add(2);
    }
    let row = row_of(e, base + STRIPS_AT, u32::from(level));
    let next = pointer_in_row(e, row, u32::from(strip) + 1);
    let first = e.mem.u16(next);
    e.mem.set_u16(*cursor, first);
    *cursor = cursor.wrapping_add(2);
}

// Translated from 00b2fa30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b2fa30 - the FS:[0] exception frame is not translated.
/// The extents of the geometry's vertices (identified from the code; the engine map gives it no
/// name): for each of the `m_vCoords` floats / 3 (truncated to 16 bits) vertices, the point
/// (its coordinates copied through the x87 stack twice) is added to the extents at `extents`
/// (`00b3caa0`, called on the extents with a 0x30-byte result slot and the point) and the
/// result slot copied over the extents; the two temporaries' empty destructors run after each
/// vertex.
pub fn fn_00b2fa30(e: &mut Engine, this: Ptr<CIndexedGeometry>, extents: Ptr) {
    let coords = this.addr() + COORDS_AT;
    let count = ((e
        .mem
        .u32(coords + 0x10)
        .wrapping_sub(e.mem.u32(coords + 0x0C))) as i32
        >> 2) as u32
        / 3;
    let count = u32::from(count as u16);
    let mut vertex: i32 = 0;
    while vertex < count as i32 {
        let point = e.call(COORD_POINTER, &args![this, vertex as u32]).u32();
        e.with_stack(0xC, |e, local| {
            for axis in [0u32, 4, 8] {
                copy_float(e, local.addr() + axis, point + axis);
            }
            e.with_stack(0xC, |e, copy| {
                for axis in [0u32, 4, 8] {
                    copy_float(e, copy.addr() + axis, local.addr() + axis);
                }
                e.with_stack(0x30, |e, result| {
                    let grown = e.call(EXTENTS_INCLUDE, &args![extents, result, copy]).u32();
                    for word in 0..12u32 {
                        let value = e.mem.u32(grown + 4 * word);
                        e.mem.set_u32(extents.addr() + 4 * word, value);
                    }
                    e.call(TRIVIAL_DESTRUCTOR, &args![result.addr() + 0x18]);
                    e.call(TRIVIAL_DESTRUCTOR, &args![result]);
                });
            });
        });
        vertex += 1;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b2e250,
            fn_00b2e250(Ptr<StlVector>, Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00b2e330, fn_00b2e330(Ptr, u32) -> Ptr),
        entry!(0x00b2e3f0, fn_00b2e3f0(u32, u32, u32) -> u32),
        entry!(0x00b2e4a0, fn_00b2e4a0(u32, u32) -> u32),
        entry!(0x00b2e500, fn_00b2e500(u32, u32, u32) -> u32),
        entry!(0x00b2e570, fn_00b2e570(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b2e680, fn_00b2e680(Ptr, Ptr) -> Ptr),
        entry!(0x00b2e740, fn_00b2e740(Ptr, Ptr) -> Ptr),
        entry!(
            0x00b2e810,
            fn_00b2e810(Ptr<CIndexedGeometry>, Ptr<CWindEngine>, u8) -> Ptr<CIndexedGeometry>
        ),
        entry!(0x00b2e9c0, fn_00b2e9c0(Ptr<CIndexedGeometry>)),
        entry!(0x00b2ed70, fn_00b2ed70(Ptr<CIndexedGeometry>, u8)),
        entry!(0x00b2fa30, fn_00b2fa30(Ptr<CIndexedGeometry>, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::rc::Rc;

    type Words = Rc<RefCell<Vec<Vec<u32>>>>;
    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// The proxy word every test vector carries at `+0`.
    const PROXY: u32 = 0x5000_0001;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// An engine with the exe's data range mapped and a panicking invalid-parameter path and
    /// throw.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        e.register(INVALID_PARAMETER, |_, _| panic!("invalid parameter"));
        e.register(CXX_THROW_EXCEPTION, |_, _| panic!("thrown"));
        e
    }

    fn panics(action: impl FnOnce()) -> bool {
        catch_unwind(AssertUnwindSafe(action)).is_err()
    }

    /// A double that records its argument words and returns `result`.
    fn spy(e: &mut Engine, addr: u32, result: u32) -> Words {
        let log: Words = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(result)
        });
        log
    }

    /// Doubles for every address in `addrs` that record (address, words) in one log.
    fn spy_all(e: &mut Engine, addrs: &[u32]) -> Calls {
        let log: Calls = Rc::new(RefCell::new(Vec::new()));
        for address in addrs {
            let address = *address;
            let sink = log.clone();
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                Ret::default()
            });
        }
        log
    }

    fn vector(e: &mut Engine, size: u32, count: u32, capacity: u32) -> (u32, u32) {
        let at = e.mem.alloc(0x18);
        let data = e.mem.alloc((size * capacity).max(1));
        e.mem.set_u32(at, PROXY);
        e.mem.set_u32(at + 0x0C, data);
        e.mem.set_u32(at + 0x10, data + size * count);
        e.mem.set_u32(at + 0x14, data + size * capacity);
        (at, data)
    }

    fn iterator(e: &mut Engine, vector: u32, position: u32) -> u32 {
        let proxy = e.mem.alloc(4);
        e.mem.set_u32(proxy, vector);
        let at = e.mem.alloc(8);
        e.mem.set_u32(at, proxy);
        e.mem.set_u32(at + 4, position);
        at
    }

    /// Fills a 0x54-byte element with a recognisable pattern: byte `tag` at `+0`, words
    /// `tag + n` elsewhere (the string area holds `tag` words too).
    fn fill_element(e: &mut Engine, at: u32, tag: u32) {
        e.mem.set_u8(at, tag as u8);
        for word in 1..0x15 {
            e.mem.set_u32(at + 4 * word, tag * 0x100 + word);
        }
    }

    fn element_tag(e: &Engine, at: u32) -> u8 {
        e.mem.u8(at)
    }

    #[test]
    fn element_assignment_and_copy_construction() {
        let mut e = engine();
        let strings = spy(&mut e, STRING_ASSIGN, 0);
        let tidies = spy(&mut e, STRING_TIDY, 0);
        e.set_global(STRING_NPOS, 0xFFFF_FFFFu32);
        let source = e.mem.alloc(0x54);
        let dest = e.mem.alloc(0x54);
        fill_element(&mut e, source, 7);
        // A signalling NaN at +0x10 is quieted by the FLD/FSTP copy.
        e.mem.set_u32(source + 0x10, 0x7F80_0001);
        for (addr, constructs) in [(0x00b2_e680u32, false), (0x00b2_e740, true)] {
            let words: Vec<u32> = (0..0x15).map(|w| 0xAAAA_0000 + w).collect();
            for (i, w) in words.iter().enumerate() {
                e.mem.set_u32(dest + 4 * i as u32, *w);
            }
            strings.borrow_mut().clear();
            tidies.borrow_mut().clear();
            let r = e.call(addr, &args![Ptr::<()>::new(dest), Ptr::<()>::new(source)]);
            assert_eq!(r.u32(), dest);
            assert_eq!(e.mem.u8(dest), 7);
            // Words +4, +8, +0xC copied; +0x10 quieted; the string area untouched (the
            // double does nothing); +0x30.. copied.
            assert_eq!(e.mem.u32(dest + 4), 7 * 0x100 + 1);
            assert_eq!(e.mem.u32(dest + 0xC), 7 * 0x100 + 3);
            assert_eq!(e.mem.u32(dest + 0x10), 0x7FC0_0001);
            assert_eq!(e.mem.u32(dest + 0x14), 0xAAAA_0005);
            for offset in [0x30u32, 0x34, 0x38, 0x3C, 0x40, 0x44, 0x48, 0x4C, 0x50] {
                assert_eq!(e.mem.u32(dest + offset), e.mem.u32(source + offset));
            }
            assert_eq!(
                *strings.borrow(),
                vec![vec![dest + 0x14, source + 0x14, 0, 0xFFFF_FFFF]]
            );
            // Only the copy construction empties the string first.
            let expected: Vec<Vec<u32>> = if constructs {
                vec![vec![dest + 0x14, 0, 0]]
            } else {
                vec![]
            };
            assert_eq!(*tidies.borrow(), expected);
        }
    }

    #[test]
    fn element_ranges_are_assigned_and_constructed() {
        let mut e = engine();
        spy(&mut e, STRING_ASSIGN, 0);
        let tidies = spy(&mut e, STRING_TIDY, 0);
        let source = e.mem.alloc(0x54 * 3);
        for i in 0..3 {
            fill_element(&mut e, source + 0x54 * i, 10 + i);
        }
        let dest = e.mem.alloc(0x54 * 4);
        for addr in [0x00b2_e3f0u32, 0x00b2_e500] {
            for i in 0..4 {
                e.mem.set_u8(dest + 0x54 * i, 0);
            }
            let end = e.call(addr, &args![source, source + 0x54 * 3, dest]).u32();
            assert_eq!(end, dest + 0x54 * 3);
            assert_eq!(
                (0..4)
                    .map(|i| element_tag(&e, dest + 0x54 * i))
                    .collect::<Vec<_>>(),
                [10, 11, 12, 0]
            );
        }
        // The copy construction skips a null destination (one element: no later store).
        let end = e.call(
            0x00b2_e570,
            &args![source, source + 0x54, dest + 0x54, 0u32, 0u32, 0u32],
        );
        assert_eq!(end.u32(), dest + 0xA8);
        assert_eq!(element_tag(&e, dest + 0x54), 10);
        tidies.borrow_mut().clear();
        let end = e.call(
            0x00b2_e570,
            &args![source, source + 0x54, 0u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(end.u32(), 0x54);
        assert!(tidies.borrow().is_empty());
    }

    #[test]
    fn element_allocation_checks_the_overflow() {
        let mut e = engine();
        let sizes = spy(&mut e, OPERATOR_NEW, 0x4000);
        let messages = spy(&mut e, EXCEPTION_FROM_MESSAGE, 0);
        assert_eq!(e.call(0x00b2_e4a0, &args![0u32, 0u32]).u32(), 0x4000);
        assert_eq!(e.call(0x00b2_e4a0, &args![3u32, 0u32]).u32(), 0x4000);
        let largest = 0xFFFF_FFFFu32 / 0x54;
        assert_eq!(e.call(0x00b2_e4a0, &args![largest, 0u32]).u32(), 0x4000);
        assert_eq!(
            *sizes.borrow(),
            vec![vec![0], vec![0xFC], vec![largest * 0x54]]
        );
        let thrown: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = thrown.clone();
        e.register_double(CXX_THROW_EXCEPTION, move |e, a| {
            sink.borrow_mut().push((e.mem.u32(a[0]), a[1]));
            panic!("thrown");
        });
        assert!(panics(|| {
            fn_00b2e4a0(&mut e, largest + 1, 0);
        }));
        assert_eq!(
            *thrown.borrow(),
            vec![(BAD_ALLOC_VTABLE, BAD_ALLOC_THROW_INFO)]
        );
        assert_eq!(messages.borrow().len(), 1);
    }

    #[test]
    fn word_iterator_advance() {
        let mut e = engine();
        let (v, data) = vector(&mut e, 4, 5, 5);
        let it = iterator(&mut e, v, data);
        let it_ptr = Ptr::<()>::new(it);
        assert_eq!(e.call(0x00b2_e330, &args![it_ptr, 5u32]).u32(), it);
        assert_eq!(e.mem.u32(it + 4), data + 20);
        assert!(panics(|| {
            fn_00b2e330(&mut e, it_ptr, 1);
        }));
    }

    #[test]
    fn erase_range_moves_the_tail_down() {
        let mut e = engine();
        spy(&mut e, STRING_ASSIGN, 0);
        let destroyed = spy(&mut e, DESTROY_ELEMENTS, 0);
        let differ = Rc::new(RefCell::new(true));
        let flag = differ.clone();
        e.register_double(ITERATOR_NOT_EQUAL, move |_, _| {
            ret(u32::from(*flag.borrow()))
        });
        let (v, data) = vector(&mut e, 0x54, 4, 4);
        for i in 0..4 {
            fill_element(&mut e, data + 0x54 * i, 20 + i);
        }
        let out = e.mem.alloc(8);
        let r = e.call(
            0x00b2_e250,
            &args![
                Ptr::<StlVector>::new(v),
                Ptr::<()>::new(out),
                0u32,
                data + 0x54,
                0u32,
                data + 0xA8
            ],
        );
        assert_eq!(r.u32(), out);
        // Elements 2 and 3 move to places 1 and 2; the last pointer drops by one element.
        assert_eq!(
            (0..3)
                .map(|i| element_tag(&e, data + 0x54 * i))
                .collect::<Vec<_>>(),
            [20, 22, 23]
        );
        assert_eq!(e.mem.u32(v + 0x10), data + 0x54 * 3);
        assert_eq!(
            *destroyed.borrow(),
            vec![vec![v, data + 0x54 * 3, data + 0x54 * 4]]
        );
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (PROXY, data + 0x54));
        // Equal iterators change nothing.
        *differ.borrow_mut() = false;
        destroyed.borrow_mut().clear();
        e.call(
            0x00b2_e250,
            &args![
                Ptr::<StlVector>::new(v),
                Ptr::<()>::new(out),
                0u32,
                data,
                0u32,
                data
            ],
        );
        assert!(destroyed.borrow().is_empty());
        assert_eq!(e.mem.u32(v + 0x10), data + 0x54 * 3);
        // Positions outside the vector are invalid.
        for (first, last) in [(data + 0x54 * 9, data), (data, data + 0x54 * 9)] {
            assert!(panics(|| {
                fn_00b2e250(&mut e, Ptr::new(v), Ptr::new(out), 0, first, 0, last);
            }));
        }
    }

    const MEMBER_CONSTRUCTORS: [(u32, u32); 15] = [
        (CONSTRUCT_POINTER_VECTOR, 0x28),
        (CONSTRUCT_STRIP_VECTOR, 0x40),
        (CONSTRUCT_STRIP_VECTOR, 0x58),
        (CONSTRUCT_COLOR_VECTOR, 0x70),
        (CONSTRUCT_FLOAT_VECTOR, 0x88),
        (CONSTRUCT_FLOAT_VECTOR, 0xA0),
        (CONSTRUCT_FLOAT_VECTOR, 0xB8),
        (CONSTRUCT_FLOAT_VECTOR, 0xD0),
        (CONSTRUCT_FLOAT_VECTOR, 0xE8),
        (CONSTRUCT_FLOAT_VECTOR, 0x100),
        (CONSTRUCT_FLOAT_VECTOR, 0x118),
        (CONSTRUCT_BYTE_VECTOR, 0x130),
        (CONSTRUCT_FLOAT_VECTOR, 0x148),
        (CONSTRUCT_FLOAT_VECTOR, 0x160),
        (CONSTRUCT_BYTE_VECTOR, 0x178),
    ];

    #[test]
    fn geometry_constructor_sets_the_defaults_and_builds_the_members() {
        let mut e = engine();
        let log = spy_all(
            &mut e,
            &[
                CONSTRUCT_POINTER_VECTOR,
                CONSTRUCT_STRIP_VECTOR,
                CONSTRUCT_COLOR_VECTOR,
                CONSTRUCT_FLOAT_VECTOR,
                CONSTRUCT_BYTE_VECTOR,
            ],
        );
        let geometry = e.new_object::<CIndexedGeometry>();
        let g = geometry.addr();
        for off in (0..0x190).step_by(4) {
            e.mem.set_u32(g + off, 0xDEAD_BEEF);
        }
        let wind = e.new_object::<CWindEngine>();
        let r = e.call(0x00b2_e810, &args![geometry, wind, 1u8]);
        assert_eq!(r.u32(), g);
        assert_eq!(e.mem.u8(g), 1);
        assert_eq!(e.mem.u32(g + 4), wind.addr());
        assert_eq!((e.mem.u8(g + 8), e.mem.u8(g + 9)), (0, 0));
        assert_eq!(e.mem.u32(g + 0x0C), 0);
        assert_eq!(e.mem.u16(g + 0x10), 0);
        assert_eq!(e.mem.u8(g + 0x12), 1);
        assert_eq!(e.mem.u32(g + 0x14), 2);
        for off in [0x20, 0x22, 0x24, 0x26] {
            assert_eq!(e.mem.u16(g + off), 0);
        }
        let expected: Vec<(u32, Vec<u32>)> = MEMBER_CONSTRUCTORS
            .iter()
            .map(|(address, offset)| (*address, vec![g + offset]))
            .collect();
        assert_eq!(*log.borrow(), expected);
    }

    #[test]
    fn geometry_destructor_tidies_the_members_in_order() {
        let mut e = engine();
        let log = spy_all(
            &mut e,
            &[
                RESET_COUNTERS,
                TIDY_BYTE_VECTOR,
                TIDY_WORD_VECTOR,
                TIDY_STRIPS,
                TIDY_STRIP_LENGTHS,
                TIDY_POINTER_VECTOR,
                FREE_PROXY,
                OPERATOR_DELETE,
            ],
        );
        let geometry = e.new_object::<CIndexedGeometry>();
        let g = geometry.addr();
        e.mem.set_u32(g + 0x0C, 0x1111);
        for off in [
            0x178u32, 0x160, 0x148, 0x130, 0x118, 0x100, 0xE8, 0xD0, 0xB8, 0xA0, 0x88, 0x70, 0x58,
        ] {
            e.mem.set_u32(g + off, 0x2000_0000 + off);
        }
        e.call(0x00b2_e9c0, &args![geometry]);
        let mut expected: Vec<(u32, Vec<u32>)> =
            vec![(OPERATOR_DELETE, vec![0x1111]), (RESET_COUNTERS, vec![g])];
        for (tidy, off) in [
            (TIDY_BYTE_VECTOR, 0x178u32),
            (TIDY_WORD_VECTOR, 0x160),
            (TIDY_WORD_VECTOR, 0x148),
            (TIDY_BYTE_VECTOR, 0x130),
            (TIDY_WORD_VECTOR, 0x118),
            (TIDY_WORD_VECTOR, 0x100),
            (TIDY_WORD_VECTOR, 0xE8),
            (TIDY_WORD_VECTOR, 0xD0),
            (TIDY_WORD_VECTOR, 0xB8),
            (TIDY_WORD_VECTOR, 0xA0),
            (TIDY_WORD_VECTOR, 0x88),
            (TIDY_WORD_VECTOR, 0x70),
            (TIDY_STRIPS, 0x58),
        ] {
            expected.push((tidy, vec![g + off]));
            expected.push((OPERATOR_DELETE, vec![0x2000_0000 + off]));
        }
        expected.push((TIDY_STRIP_LENGTHS, vec![g + 0x40]));
        expected.push((FREE_PROXY, vec![g + 0x40]));
        expected.push((TIDY_POINTER_VECTOR, vec![g + 0x28]));
        expected.push((FREE_PROXY, vec![g + 0x28]));
        assert_eq!(*log.borrow(), expected);
        assert_eq!(e.mem.u32(g + 0x0C), 0);
    }

    /// What `00b2ed70` hands to the geometry's three vector assignments, as rows of values.
    struct Captured {
        lengths: Rc<RefCell<Vec<Vec<u32>>>>,
        strips: Rc<RefCell<Vec<Vec<u32>>>>,
        counts: Rc<RefCell<Vec<u32>>>,
    }

    /// Doubles for the working vectors `00b2ed70` builds: simple sized constructors (empty rows),
    /// pushes that reallocate, clears, and shallow assignments that capture what the geometry
    /// receives. The checked indexing (`00b319a0`, `00b31230`) runs for real.
    fn strip_doubles(e: &mut Engine) -> Captured {
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0].max(1))));
        e.register(MEMCPY, |e, a| {
            let data = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &data);
            ret(a[0])
        });
        for address in [
            RESET_COUNTERS,
            TIDY_POINTER_VECTOR,
            TIDY_STRIPS,
            TIDY_STRIP_LENGTHS,
            FREE_PROXY,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        for address in [SIZED_STRIP_LENGTHS, SIZED_STRIPS] {
            e.register(address, |e, a| {
                let rows = a[1];
                let data = e.mem.alloc((0x18 * rows).max(1));
                e.mem.set_u32(a[0], PROXY);
                e.mem.set_u32(a[0] + 0x0C, data);
                e.mem.set_u32(a[0] + 0x10, data + 0x18 * rows);
                e.mem.set_u32(a[0] + 0x14, data + 0x18 * rows);
                ret(a[0])
            });
        }
        e.register(SIZED_COUNTS, |e, a| {
            let n = a[1];
            let data = e.mem.alloc((4 * n).max(1));
            e.mem.set_u32(a[0], PROXY);
            e.mem.set_u32(a[0] + 0x0C, data);
            e.mem.set_u32(a[0] + 0x10, data + 4 * n);
            e.mem.set_u32(a[0] + 0x14, data + 4 * n);
            ret(a[0])
        });
        for (address, size) in [(PUSH_SHORT, 2u32), (PUSH_INT, 4)] {
            e.register_double(address, move |e, a| {
                let (row, value) = (a[0], a[1]);
                let (first, last) = (e.mem.u32(row + 0x0C), e.mem.u32(row + 0x10));
                let used = if first == 0 { 0 } else { last - first };
                let data = e.mem.alloc(used + size);
                if used != 0 {
                    let old = e.mem.bytes(first, used);
                    e.mem.write(data, &old);
                }
                let element = e.mem.bytes(value, size);
                e.mem.write(data + used, &element);
                e.mem.set_u32(row + 0x0C, data);
                e.mem.set_u32(row + 0x10, data + used + size);
                e.mem.set_u32(row + 0x14, data + used + size);
                Ret::default()
            });
        }
        for address in [CLEAR_SHORTS, CLEAR_INTS] {
            e.register(address, |e, a| {
                let first = e.mem.u32(a[0] + 0x0C);
                e.mem.set_u32(a[0] + 0x10, first);
                Ret::default()
            });
        }
        let captured = Captured {
            lengths: Rc::new(RefCell::new(Vec::new())),
            strips: Rc::new(RefCell::new(Vec::new())),
            counts: Rc::new(RefCell::new(Vec::new())),
        };
        let sink = captured.lengths.clone();
        e.register_double(ASSIGN_STRIP_LENGTHS, move |e, a| {
            *sink.borrow_mut() = rows_of_values(e, a[1], 2);
            ret(a[0])
        });
        let sink = captured.strips.clone();
        e.register_double(ASSIGN_STRIPS, move |e, a| {
            *sink.borrow_mut() = rows_of_values(e, a[1], 4);
            ret(a[0])
        });
        let sink = captured.counts.clone();
        e.register_double(ASSIGN_COUNTS, move |e, a| {
            let (first, last) = (e.mem.u32(a[1] + 0x0C), e.mem.u32(a[1] + 0x10));
            *sink.borrow_mut() = (0..(last - first) / 4)
                .map(|i| e.mem.u32(first + 4 * i))
                .collect();
            ret(a[0])
        });
        captured
    }

    /// The rows of a vector of vectors whose elements are `size` bytes, as values.
    fn rows_of_values(e: &Engine, vector: u32, size: u32) -> Vec<Vec<u32>> {
        let (first, last) = (e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10));
        (0..(last - first) / 0x18)
            .map(|row| {
                let at = first + 0x18 * row;
                let (rf, rl) = (e.mem.u32(at + 0x0C), e.mem.u32(at + 0x10));
                if rf == 0 {
                    return Vec::new();
                }
                (0..(rl - rf) / size)
                    .map(|i| {
                        if size == 2 {
                            u32::from(e.mem.u16(rf + 2 * i))
                        } else {
                            e.mem.u32(rf + 4 * i)
                        }
                    })
                    .collect()
            })
            .collect()
    }

    /// A geometry whose levels have the given strips (each a list of indices) and `vertices`
    /// vertices.
    fn geometry_with_strips(
        e: &mut Engine,
        strips: &[Vec<Vec<u16>>],
        vertices: u32,
    ) -> Ptr<CIndexedGeometry> {
        let geometry = e.new_object::<CIndexedGeometry>();
        let g = geometry.addr();
        let levels = strips.len() as u32;
        e.mem.set_u16(g + 0x20, levels as u16);
        let coords = e.mem.alloc(12 * vertices.max(1));
        e.mem.set_u32(g + 0x88 + 0x0C, coords);
        e.mem.set_u32(g + 0x88 + 0x10, coords + 12 * vertices);
        let (lengths_vector, lengths_data) = vector(e, 0x18, levels, levels);
        let (strips_vector, strips_data) = vector(e, 0x18, levels, levels);
        for (level, level_strips) in strips.iter().enumerate() {
            let level = level as u32;
            let count = level_strips.len() as u32;
            let length_values = e.mem.alloc((2 * count).max(1));
            let pointers = e.mem.alloc((4 * count).max(1));
            for (i, strip) in level_strips.iter().enumerate() {
                e.mem
                    .set_u16(length_values + 2 * i as u32, strip.len() as u16);
                let data = e.mem.alloc((2 * strip.len() as u32).max(1));
                for (k, index) in strip.iter().enumerate() {
                    e.mem.set_u16(data + 2 * k as u32, *index);
                }
                e.mem.set_u32(pointers + 4 * i as u32, data);
            }
            let (row, row_end) = (lengths_data + 0x18 * level, strips_data + 0x18 * level);
            e.mem.set_u32(row + 0x0C, length_values);
            e.mem.set_u32(row + 0x10, length_values + 2 * count);
            e.mem.set_u32(row_end + 0x0C, pointers);
            e.mem.set_u32(row_end + 0x10, pointers + 4 * count);
        }
        // The geometry's own vectors sit inline at +0x40 and +0x58.
        for (vector_at, source) in [(0x40u32, lengths_vector), (0x58, strips_vector)] {
            for word in 0..6 {
                let value = e.mem.u32(source + 4 * word);
                e.mem.set_u32(g + vector_at + 4 * word, value);
            }
        }
        geometry
    }

    fn block_values(e: &Engine, block: u32, length: u32) -> Vec<u32> {
        (0..length)
            .map(|i| u32::from(e.mem.u16(block + 2 * i)))
            .collect()
    }

    #[test]
    fn strips_are_joined_with_two_extra_indices() {
        let mut e = engine();
        let captured = strip_doubles(&mut e);
        // Level 0 has two strips; level 1 has none.
        let geometry =
            geometry_with_strips(&mut e, &[vec![vec![1, 2, 3], vec![7, 8, 9, 10]], vec![]], 1);
        e.call(0x00b2_ed70, &args![geometry, 0u8]);
        // 7 indices plus two for the one join: the last index of the first strip and the first
        // of the next.
        assert_eq!(*captured.lengths.borrow(), vec![vec![9], vec![]]);
        assert_eq!(*captured.counts.borrow(), vec![7, 0]);
        let strips = captured.strips.borrow().clone();
        assert_eq!(strips[1], Vec::<u32>::new());
        assert_eq!(
            block_values(&e, strips[0][0], 9),
            [1, 2, 3, 3, 7, 7, 8, 9, 10]
        );
    }

    #[test]
    fn flagged_join_repeats_the_last_index_of_an_odd_strip() {
        let mut e = engine();
        let captured = strip_doubles(&mut e);
        // The first strip has odd length (3): three extra indices; the second is last, so
        // nothing follows it.
        let geometry = geometry_with_strips(&mut e, &[vec![vec![1, 2, 3], vec![7, 8, 9, 10]]], 1);
        e.call(0x00b2_ed70, &args![geometry, 1u8]);
        assert_eq!(*captured.lengths.borrow(), vec![vec![10]]);
        assert_eq!(*captured.counts.borrow(), vec![8]);
        let strips = captured.strips.borrow().clone();
        assert_eq!(
            block_values(&e, strips[0][0], 10),
            [1, 2, 3, 3, 3, 7, 7, 8, 9, 10]
        );
        // An even first strip needs only the usual two.
        let mut e = engine();
        let captured = strip_doubles(&mut e);
        let geometry = geometry_with_strips(&mut e, &[vec![vec![1, 2, 3, 4], vec![7, 8, 9]]], 1);
        e.call(0x00b2_ed70, &args![geometry, 1u8]);
        assert_eq!(*captured.lengths.borrow(), vec![vec![9]]);
        let strips = captured.strips.borrow().clone();
        assert_eq!(
            block_values(&e, strips[0][0], 9),
            [1, 2, 3, 4, 4, 7, 7, 8, 9]
        );
        assert_eq!(*captured.counts.borrow(), vec![7]);
    }

    #[test]
    fn a_single_strip_is_copied_as_it_is() {
        let mut e = engine();
        let captured = strip_doubles(&mut e);
        let geometry = geometry_with_strips(&mut e, &[vec![vec![4, 5, 6, 7, 8]]], 3);
        e.call(0x00b2_ed70, &args![geometry, 0u8]);
        assert_eq!(*captured.lengths.borrow(), vec![vec![5]]);
        assert_eq!(*captured.counts.borrow(), vec![3]);
        let strips = captured.strips.borrow().clone();
        assert_eq!(block_values(&e, strips[0][0], 5), [4, 5, 6, 7, 8]);
    }

    #[test]
    fn no_vertices_means_nothing_is_rebuilt() {
        let mut e = engine();
        let captured = strip_doubles(&mut e);
        let built = spy(&mut e, SIZED_STRIPS, 0);
        let geometry = geometry_with_strips(&mut e, &[vec![vec![1, 2, 3]]], 0);
        e.call(0x00b2_ed70, &args![geometry, 0u8]);
        assert!(built.borrow().is_empty());
        assert!(captured.counts.borrow().is_empty());
        // Fewer than three coordinates truncate to no vertex.
        let g = geometry.addr();
        let coords = e.mem.u32(g + 0x88 + 0x0C);
        e.mem.set_u32(g + 0x88 + 0x10, coords + 8);
        e.call(0x00b2_ed70, &args![geometry, 0u8]);
        assert!(built.borrow().is_empty());
    }

    #[test]
    fn extents_cover_every_vertex() {
        let mut e = engine();
        let geometry = e.new_object::<CIndexedGeometry>();
        let g = geometry.addr();
        // Three vertices (nine floats).
        let (coords_vector, coords) = vector(&mut e, 4, 9, 9);
        for (i, v) in [1.0f32, 5.0, 3.0, -2.0, 7.0, 0.5, 4.0, -1.0, 2.0]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(coords + 4 * i as u32, *v);
        }
        for word in 0..6 {
            let value = e.mem.u32(coords_vector + 4 * word);
            e.mem.set_u32(g + 0x88 + 4 * word, value);
        }
        e.register(COORD_POINTER, |e, a| {
            let first = e.mem.u32(a[0] + 0x88 + 0x0C);
            ret(first + 12 * a[1])
        });
        // The extents' growth: minimum in words 0..3, maximum in words 3..6 (the rest copied).
        let points = Rc::new(RefCell::new(Vec::new()));
        let sink = points.clone();
        e.register_double(EXTENTS_INCLUDE, move |e, a| {
            let (extents, result, point) = (a[0], a[1], a[2]);
            let p: Vec<f32> = (0..3).map(|i| e.mem.f32(point + 4 * i)).collect();
            sink.borrow_mut().push(p.clone());
            for word in 0..12 {
                let value = e.mem.u32(extents + 4 * word);
                e.mem.set_u32(result + 4 * word, value);
            }
            for axis in 0..3u32 {
                let value = p[axis as usize];
                if value < e.mem.f32(result + 4 * axis) {
                    e.mem.set_f32(result + 4 * axis, value);
                }
                if e.mem.f32(result + 12 + 4 * axis) < value {
                    e.mem.set_f32(result + 12 + 4 * axis, value);
                }
            }
            ret(result)
        });
        let destructors = spy(&mut e, TRIVIAL_DESTRUCTOR, 0);
        let extents = e.mem.alloc(0x30);
        for (i, v) in [100.0f32, 100.0, 100.0, -100.0, -100.0, -100.0]
            .iter()
            .enumerate()
        {
            e.mem.set_f32(extents + 4 * i as u32, *v);
        }
        e.call(0x00b2_fa30, &args![geometry, Ptr::<()>::new(extents)]);
        assert_eq!(
            *points.borrow(),
            vec![
                vec![1.0, 5.0, 3.0],
                vec![-2.0, 7.0, 0.5],
                vec![4.0, -1.0, 2.0]
            ]
        );
        // Each vertex grows the minimum (initially 100) and the maximum (initially -100).
        let result: Vec<f32> = (0..6).map(|i| e.mem.f32(extents + 4 * i)).collect();
        assert_eq!(result, [-2.0, -1.0, 0.5, 4.0, 7.0, 3.0]);
        // Two temporaries are destroyed per vertex.
        assert_eq!(destructors.borrow().len(), 6);
    }

    #[test]
    fn extents_quiet_a_signalling_nan_coordinate() {
        let mut e = engine();
        let geometry = e.new_object::<CIndexedGeometry>();
        let g = geometry.addr();
        let (coords_vector, coords) = vector(&mut e, 4, 3, 3);
        e.mem.set_u32(coords, 0x7F80_0001);
        for word in 0..6 {
            let value = e.mem.u32(coords_vector + 4 * word);
            e.mem.set_u32(g + 0x88 + 4 * word, value);
        }
        e.register(COORD_POINTER, |e, a| ret(e.mem.u32(a[0] + 0x88 + 0x0C)));
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(EXTENTS_INCLUDE, move |e, a| {
            sink.borrow_mut().push(e.mem.u32(a[2]));
            ret(a[0])
        });
        spy(&mut e, TRIVIAL_DESTRUCTOR, 0);
        let extents = e.mem.alloc(0x30);
        e.call(0x00b2_fa30, &args![geometry, Ptr::<()>::new(extents)]);
        assert_eq!(*seen.borrow(), vec![0x7FC0_0001]);
    }
}
