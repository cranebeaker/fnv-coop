//! `SpeedTree/run_00b31070` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vector instantiations `CIndexedGeometry` uses for its strip vectors, identified from the
//! code (the engine map gives them no names): the vectors of 0x18-byte elements (each element a
//! `std::vector` itself) with their sized constructors, copy assignment, clear, checked index
//! and destructors, a vector constructor of 4-byte elements, and one `CIndexedGeometry` method
//! (`00b31070`) that pushes the texture coordinates of the current vertex.

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{StlVector, INVALID_PARAMETER, OPERATOR_DELETE};
use super::indexedgeometry::CIndexedGeometry;

/// `std::allocator` proxy allocation (`0044d670`, `cdecl`, a count and a hint): the container
/// proxy block of the checked vectors.
const PROXY_ALLOCATE: u32 = 0x0044_d670;
/// The vector of 0x18-byte elements' storage set-up (`00b1f590`, `thiscall`, a count; returns a
/// byte flag).
const BUY_ELEMENTS: u32 = 0x00b1_f590;
/// The checked vector's proxy constructor of the 4-byte element vector (`00b326e0`, `fastcall`)
/// and its fill construction (`00b32600`, `thiscall`: a count and the address of the value).
const PROXY_CONSTRUCT: u32 = 0x00b3_26e0;
const CONSTRUCT_FILLED: u32 = 0x00b3_2600;
/// The element-vector destructor `00b39460`: frees the proxy at the start of the object.
const FREE_PROXY: u32 = 0x00b3_9460;
/// `CIndexedGeometry::` push of one texture coordinate pair (`00b301f0`, `thiscall`: the pair's
/// address and a level argument).
const PUSH_TEX_COORD: u32 = 0x00b3_01f0;
/// `00b0d7a0`: the byte flag that decides whether the second texture coordinate is negated.
const FLIP_TEXTURE_V: u32 = 0x00b0_d7a0;
/// The scalar-deleting-destructor flag the exe folds to 0 where an inlined destructor call
/// could free its object.
const DESTRUCTOR_FREES: bool = false;

/// The callees of one vector-of-vectors instantiation, which differ only in the element type:
/// the clear, the copy-assign and its erase of a tail, the copy to a range, the uninitialised
/// copy, and the element destructor.
struct VectorOfVectors {
    clear: u32,
    assign_range: u32,
    erase_tail: u32,
    assign_into: u32,
    copy_construct: u32,
    element_destructor: u32,
}

/// The strip-lengths vector (`std::vector` of `std::vector<unsigned short>`, unconfirmed).
const STRIP_LENGTHS: VectorOfVectors = VectorOfVectors {
    clear: 0x00b3_19e0,
    assign_range: 0x00b3_5f60,
    erase_tail: 0x00b3_2bb0,
    assign_into: 0x00b3_5a90,
    copy_construct: 0x00b3_68d0,
    element_destructor: 0x00b3_4d90,
};

/// The strips vector (`std::vector` of `std::vector<unsigned short *>`, unconfirmed).
const STRIPS: VectorOfVectors = VectorOfVectors {
    clear: 0x00b3_2120,
    assign_range: 0x00b3_6000,
    erase_tail: 0x00b3_3110,
    assign_into: 0x00b3_5b60,
    copy_construct: 0x00b3_69f0,
    element_destructor: 0x00b1_7f50,
};

/// The proxy construction every vector constructor of these units starts with: a one-element
/// proxy block (`0044d670`) whose first word is the vector, stored at the vector's start (a null
/// block stays null).
fn construct_proxy(e: &mut Engine, vector: u32) {
    let proxy = e.call(PROXY_ALLOCATE, &args![1u32, 0u32]).u32();
    let stored = if proxy == 0 {
        0
    } else {
        e.mem.set_u32(proxy, vector);
        proxy
    };
    e.mem.set_u32(vector, stored);
}

/// `n` default elements: builds the vector's proxy, a default element at a local (`build`, a
/// `fastcall` constructor taking the local), fills the vector with `count` copies of it
/// (`fill`), destroys the local (`destroy`) and frees its proxy.
fn construct_sized(e: &mut Engine, this: Ptr, count: u32, (build, fill, destroy): (u32, u32, u32)) {
    construct_proxy(e, this.addr());
    e.with_stack(0x18, |e, local| {
        let element = e.call(build, &args![local]).u32();
        e.call(fill, &args![this, count, element]);
        e.call(destroy, &args![local]);
        let proxy = e.mem.u32(local.addr());
        e.call(OPERATOR_DELETE, &args![proxy]);
    });
}

/// The number of 0x18-byte elements between two pointers (signed division).
fn elements_between(first: u32, last: u32) -> u32 {
    ((last.wrapping_sub(first)) as i32 / 0x18) as u32
}

/// `vector::clear()` as the exe inlines it: the iterators for the first and the end element are
/// checked (the vector exists and `first <= position <= last`) and `erase_range` is called with
/// them, the vector and an iterator result slot.
pub(crate) fn clear_with_erase(e: &mut Engine, this: Ptr<StlVector>, erase_range: u32) {
    let vector = this.addr();
    let last = e.mem.u32(vector + 0x10);
    if vector == 0 || e.mem.u32(vector + 0x0C) > last || last > e.mem.u32(vector + 0x10) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let last_proxy = e.mem.u32(vector);
    let first = e.mem.u32(vector + 0x0C);
    if vector == 0 || e.mem.u32(vector + 0x0C) > first || first > e.mem.u32(vector + 0x10) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let first_proxy = e.mem.u32(vector);
    e.with_stack(8, |e, out| {
        e.call(
            erase_range,
            &args![this, out, first_proxy, first, last_proxy, last],
        );
    });
}

/// `vector::operator=(const vector &)` of a vector of 0x18-byte elements, with the element
/// type's helpers `callees`: nothing for a self-assignment; an empty source clears; a source no
/// larger than the target is assigned over the first elements and the rest destroyed; a source
/// larger than the target but within its capacity is assigned over the existing elements and the
/// remainder copy-constructed after them; a source beyond the capacity destroys the elements,
/// frees the storage, buys new storage and copy-constructs the whole source. Returns `this`.
fn assign_vector(
    e: &mut Engine,
    this: Ptr<StlVector>,
    other: Ptr<StlVector>,
    callees: &VectorOfVectors,
) -> Ptr<StlVector> {
    let (vector, source) = (this.addr(), other.addr());
    if vector == source {
        return this;
    }
    let source_count = elements_between(e.mem.u32(source + 0x0C), e.mem.u32(source + 0x10));
    if source_count == 0 {
        e.call(callees.clear, &args![this]);
        return this;
    }
    let source_count = elements_between(e.mem.u32(source + 0x0C), e.mem.u32(source + 0x10));
    let count = elements_between(e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10));
    if source_count > count {
        let capacity = if e.mem.u32(vector + 0x0C) == 0 {
            0
        } else {
            elements_between(e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x14))
        };
        if source_count > capacity {
            if e.mem.u32(vector + 0x0C) != 0 {
                let last = e.mem.u32(vector + 0x10);
                let mut element = e.mem.u32(vector + 0x0C);
                while element != last {
                    e.call(callees.element_destructor, &args![element]);
                    e.call(FREE_PROXY, &args![element]);
                    if DESTRUCTOR_FREES {
                        // The destructor's delete flag is the constant 0 (`XOR EDX, EDX`,
                        // `AND EDX, 1`), so the element is never freed here.
                        e.call(OPERATOR_DELETE, &args![element]);
                    }
                    element = element.wrapping_add(0x18);
                }
                let storage = e.mem.u32(vector + 0x0C);
                e.call(OPERATOR_DELETE, &args![storage]);
            }
            let wanted = elements_between(e.mem.u32(source + 0x0C), e.mem.u32(source + 0x10));
            if e.call(BUY_ELEMENTS, &args![this, wanted]).bool() {
                let end = e
                    .call(
                        callees.copy_construct,
                        &args![
                            e.mem.u32(source + 0x0C),
                            e.mem.u32(source + 0x10),
                            e.mem.u32(vector + 0x0C),
                            vector + 8,
                            0u32,
                            0u32
                        ],
                    )
                    .u32();
                e.mem.set_u32(vector + 0x10, end);
            }
        } else {
            let middle = count
                .wrapping_mul(0x18)
                .wrapping_add(e.mem.u32(source + 0x0C));
            e.call(
                callees.assign_into,
                &args![e.mem.u32(source + 0x0C), middle, e.mem.u32(vector + 0x0C)],
            );
            let end = e
                .call(
                    callees.copy_construct,
                    &args![
                        middle,
                        e.mem.u32(source + 0x10),
                        e.mem.u32(vector + 0x10),
                        vector + 8,
                        0u32,
                        0u32
                    ],
                )
                .u32();
            e.mem.set_u32(vector + 0x10, end);
        }
    } else {
        let end = e
            .call(
                callees.assign_range,
                &args![
                    e.mem.u32(source + 0x0C),
                    e.mem.u32(source + 0x10),
                    e.mem.u32(vector + 0x0C),
                    0u32,
                    0u32,
                    0u32
                ],
            )
            .u32();
        e.call(
            callees.erase_tail,
            &args![this, end, e.mem.u32(vector + 0x10)],
        );
        let new_last = elements_between(e.mem.u32(source + 0x0C), e.mem.u32(source + 0x10))
            .wrapping_mul(0x18)
            .wrapping_add(e.mem.u32(vector + 0x0C));
        e.mem.set_u32(vector + 0x10, new_last);
    }
    this
}

/// `lerp`-style blend of the x87 sequence `FLD [source]; FLD [low]; FSUBR [high]; FMULP;
/// FADD [low]`: `source * (high - low) + low`, with `low` first copied through the x87 stack
/// (`FLD`, `FSTP`, which quiets a signalling NaN); returns the float's bits.
fn blend(e: &Engine, source: u32, high: u32, low: u32) -> u32 {
    let low_bits = x87_quiet_bits(e.mem.u32(low));
    let scale = x87_load32(e.mem.u32(source));
    let span = x87_sub(x87_m32(e.mem.u32(high)), x87_load32(low_bits));
    x87_store32(x87_add(x87_mul(scale, span), x87_m32(low_bits)))
}

// Translated from 00b31070 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `CIndexedGeometry` method (identified from the code; the engine map gives it no name): when
/// the texture index stored for the current vertex (`m_vTexIndices0[m_nVertexCounter]`) equals
/// `texture`, builds the vertex's texture coordinate pair from the original coordinates
/// `m_vOrigTexCoords0[2 * counter]` and `[2 * counter + 1]` and the rectangle at `rect`
/// (`u = orig_u * (rect[0] - rect[2]) + rect[2]`, `v = orig_v * (rect[1] - rect[5]) +
/// rect[5]`), negates `v` when `00b0d7a0` says so and pushes the pair with level -1
/// (`00b301f0`). An index outside either vector takes the invalid-parameter path.
pub fn fn_00b31070(e: &mut Engine, this: Ptr<CIndexedGeometry>, texture: u8, rect: Ptr) {
    let indices = this.at(CIndexedGeometry::m_vTexIndices0).addr();
    let counter = u32::from(e.get(this, CIndexedGeometry::m_nVertexCounter));
    if counter
        >= e.mem
            .u32(indices + 0x10)
            .wrapping_sub(e.mem.u32(indices + 0x0C))
    {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let stored = e.mem.u8(e.mem.u32(indices + 0x0C).wrapping_add(counter));
    if texture != stored {
        return;
    }
    let coords = this.at(CIndexedGeometry::m_vOrigTexCoords0).addr();
    let first_index = u32::from(e.get(this, CIndexedGeometry::m_nVertexCounter)) << 1;
    let count = ((e
        .mem
        .u32(coords + 0x10)
        .wrapping_sub(e.mem.u32(coords + 0x0C))) as i32
        >> 2) as u32;
    if first_index >= count {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let first_source = e
        .mem
        .u32(coords + 0x0C)
        .wrapping_add(first_index.wrapping_mul(4));
    let u_bits = blend(e, first_source, rect.addr(), rect.addr() + 8);
    let second_index = (u32::from(e.get(this, CIndexedGeometry::m_nVertexCounter)) << 1) + 1;
    let count = ((e
        .mem
        .u32(coords + 0x10)
        .wrapping_sub(e.mem.u32(coords + 0x0C))) as i32
        >> 2) as u32;
    if second_index >= count {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let second_source = e
        .mem
        .u32(coords + 0x0C)
        .wrapping_add(second_index.wrapping_mul(4));
    let mut v_bits = blend(e, second_source, rect.addr() + 4, rect.addr() + 0x14);
    if e.call(FLIP_TEXTURE_V, &args![]).bool() {
        // FLD, FCHS, FSTP: the load quiets a signalling NaN, the negation flips the sign.
        v_bits = x87_quiet_bits(v_bits) ^ 0x8000_0000;
    }
    e.with_stack(8, |e, pair| {
        e.mem.set_u32(pair.addr(), u_bits);
        e.mem.set_u32(pair.addr() + 4, v_bits);
        e.call(PUSH_TEX_COORD, &args![this, pair, 0xFFFF_FFFFu32]);
    });
}

// Translated from 00b311b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b311b0 - the FS:[0] exception frame (releasing the vector if the
// construction throws) is not translated.
/// The sized constructor of a vector of 4-byte elements (identified from the code; `this` and the
/// element count): constructs the proxy (`00b326e0`) and fills the vector with `count` copies
/// of a zero word (`00b32600`). Returns `this`.
pub fn fn_00b311b0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    // `PUSH ECX` reserves the proxy constructor's by-value allocator word (`00b326e0` ends
    // with `RET 4`); the word the push leaves there is `ECX`, `this`.
    e.call(PROXY_CONSTRUCT, &args![this, this]);
    e.with_stack(4, |e, value| {
        e.mem.set_u32(value.addr(), 0);
        e.call(CONSTRUCT_FILLED, &args![this, count, value]);
    });
    this
}

// Translated from 00b31230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked `operator[]` of a vector of 4-byte elements (identified from the code): the
/// address `first + index * 4`; an index not below `(last - first) >> 2` takes the
/// invalid-parameter path.
pub fn fn_00b31230(e: &mut Engine, this: Ptr<StlVector>, index: u32) -> u32 {
    let vector = this.addr();
    let count = (e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C)) as i32
        >> 2) as u32;
    if index >= count {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem.u32(vector + 0x0C).wrapping_add(index.wrapping_mul(4))
}

// Translated from 00b31270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b31270 - the FS:[0] exception frame is not translated.
/// The default constructor of a vector of 0x18-byte elements (identified from the code;
/// `fastcall`): makes the container proxy (`0044d670`; its first word is the vector) and stores
/// it at the vector's start, then sets up empty storage (`00b1f590` with a count of 0).
/// Returns `this`.
pub fn fn_00b31270(e: &mut Engine, this: Ptr<StlVector>) -> Ptr<StlVector> {
    construct_proxy(e, this.addr());
    e.call(BUY_ELEMENTS, &args![this, 0u32]);
    this
}

// Translated from 00b31350 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b31350 - the FS:[0] exception frame is not translated.
/// The sized constructor of the strip-lengths vector (identified from the code; `this` and the
/// element count): builds the proxy, a default element at a local (`00b336a0`), fills the
/// vector with `count` copies of it (`00b32780`), destroys the local (`00b34d90`) and frees its
/// proxy. Returns `this`.
pub fn fn_00b31350(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    construct_sized(e, this, count, (0x00b3_36a0, 0x00b3_2780, 0x00b3_4d90));
    this
}

// Translated from 00b31480 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b31480 - the FS:[0] exception frame is not translated.
/// The destructor of the strip-lengths vector (identified from the code; `fastcall`): tidies it
/// (`00b32c60`) and frees its container proxy.
pub fn fn_00b31480(e: &mut Engine, this: Ptr<StlVector>) {
    e.call(0x00b3_2c60, &args![this]);
    let proxy = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00b314f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b314f0 - the FS:[0] exception frame is not translated.
/// `operator=` of the strip-lengths vector (identified from the code): see [`assign_vector`]
/// with the callees `00b319e0` (clear), `00b35f60`, `00b32bb0`, `00b35a90`, `00b368d0` and the
/// element destructor `00b34d90`. Returns `this`.
pub fn fn_00b314f0(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) -> Ptr<StlVector> {
    assign_vector(e, this, other, &STRIP_LENGTHS)
}

// Translated from 00b319a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked `operator[]` of a vector of 0x18-byte elements (identified from the code): the
/// address `first + index * 0x18`; an index not below the element count takes the
/// invalid-parameter path.
pub fn fn_00b319a0(e: &mut Engine, this: Ptr<StlVector>, index: u32) -> u32 {
    let vector = this.addr();
    let count = elements_between(e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10));
    if index >= count {
        e.call(INVALID_PARAMETER, &args![]);
    }
    index
        .wrapping_mul(0x18)
        .wrapping_add(e.mem.u32(vector + 0x0C))
}

// Translated from 00b319e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `clear()` of the strip-lengths vector (identified from the code; `fastcall`): erases the
/// whole range `begin()..end()` through `00b32ad0` after the iterator checks.
pub fn fn_00b319e0(e: &mut Engine, this: Ptr<StlVector>) {
    clear_with_erase(e, this, 0x00b3_2ad0);
}

// Translated from 00b31ad0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b31ad0 - the FS:[0] exception frame is not translated.
/// The sized constructor of the strips vector (identified from the code; `this` and the element
/// count): the construction of `00b31350` with the element helpers `00b17db0` (default element),
/// `00b32ce0` (fill) and `00b17f50` (destroy). Returns `this`.
pub fn fn_00b31ad0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    construct_sized(e, this, count, (0x00b1_7db0, 0x00b3_2ce0, 0x00b1_7f50));
    this
}

// Translated from 00b31c00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b31c00 - the FS:[0] exception frame is not translated.
/// The destructor of the strips vector (identified from the code; `fastcall`): tidies it
/// (`00b331c0`) and frees its container proxy.
pub fn fn_00b31c00(e: &mut Engine, this: Ptr<StlVector>) {
    e.call(0x00b3_31c0, &args![this]);
    let proxy = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00b31c70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b31c70 - the FS:[0] exception frame is not translated.
/// `operator=` of the strips vector (identified from the code): see [`assign_vector`] with the
/// callees `00b32120` (clear), `00b36000`, `00b33110`, `00b35b60`, `00b369f0` and the element
/// destructor `00b17f50`. Returns `this`.
pub fn fn_00b31c70(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) -> Ptr<StlVector> {
    assign_vector(e, this, other, &STRIPS)
}

// Translated from 00b32120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `clear()` of the strips vector (identified from the code; `fastcall`): the same as
/// `00b319e0` with the erase `00b33030`.
pub fn fn_00b32120(e: &mut Engine, this: Ptr<StlVector>) {
    clear_with_erase(e, this, 0x00b3_3030);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b31070, fn_00b31070(Ptr<CIndexedGeometry>, u8, Ptr)),
        entry!(0x00b311b0, fn_00b311b0(Ptr, u32) -> Ptr),
        entry!(0x00b31230, fn_00b31230(Ptr<StlVector>, u32) -> u32),
        entry!(0x00b31270, fn_00b31270(Ptr<StlVector>) -> Ptr<StlVector>),
        entry!(0x00b31350, fn_00b31350(Ptr, u32) -> Ptr),
        entry!(0x00b31480, fn_00b31480(Ptr<StlVector>)),
        entry!(
            0x00b314f0,
            fn_00b314f0(Ptr<StlVector>, Ptr<StlVector>) -> Ptr<StlVector>
        ),
        entry!(0x00b319a0, fn_00b319a0(Ptr<StlVector>, u32) -> u32),
        entry!(0x00b319e0, fn_00b319e0(Ptr<StlVector>)),
        entry!(0x00b31ad0, fn_00b31ad0(Ptr, u32) -> Ptr),
        entry!(0x00b31c00, fn_00b31c00(Ptr<StlVector>)),
        entry!(
            0x00b31c70,
            fn_00b31c70(Ptr<StlVector>, Ptr<StlVector>) -> Ptr<StlVector>
        ),
        entry!(0x00b32120, fn_00b32120(Ptr<StlVector>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::rc::Rc;

    type Words = Rc<RefCell<Vec<Vec<u32>>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// An engine with a panicking invalid-parameter path.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        e.register(INVALID_PARAMETER, |_, _| panic!("invalid parameter"));
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

    /// A vector of 4-byte or 0x18-byte elements with `count` used and room for `capacity`.
    fn vector(e: &mut Engine, size: u32, count: u32, capacity: u32) -> (u32, u32) {
        let at = e.mem.alloc(0x18);
        let data = e.mem.alloc((size * capacity).max(1));
        e.mem.set_u32(at, 0x5000_0001);
        e.mem.set_u32(at + 0x0C, data);
        e.mem.set_u32(at + 0x10, data + size * count);
        e.mem.set_u32(at + 0x14, data + size * capacity);
        (at, data)
    }

    #[test]
    fn texture_coordinates_are_blended_into_the_rectangle() {
        let mut e = engine();
        let geometry = e.new_object::<CIndexedGeometry>();
        let g = geometry.addr();
        // Vertex 1 of 3: its stored texture index is 5.
        e.mem.set_u16(g + 0x22, 1);
        let (_, indices) = vector(&mut e, 1, 3, 3);
        let indices_vector = e.mem.alloc(0x18);
        let _ = indices_vector;
        e.mem.set_u32(g + 0x130 + 0x0C, indices);
        e.mem.set_u32(g + 0x130 + 0x10, indices + 3);
        e.mem.write(indices, &[1, 5, 2]);
        // The original texture coordinates: (0.1, 0.2), (0.5, 0.25), (0.9, 0.8).
        let (_, coords) = vector(&mut e, 4, 6, 6);
        e.mem.set_u32(g + 0x118 + 0x0C, coords);
        e.mem.set_u32(g + 0x118 + 0x10, coords + 24);
        for (i, v) in [0.1f32, 0.2, 0.5, 0.25, 0.9, 0.8].iter().enumerate() {
            e.mem.set_f32(coords + 4 * i as u32, *v);
        }
        // The rectangle: u from 3.0 (low at +8) to 7.0 (+0), v from 1.0 (+0x14) to 2.0 (+4).
        let rect = e.mem.alloc(0x18);
        for (i, v) in [7.0f32, 2.0, 3.0, 0.0, 0.0, 1.0].iter().enumerate() {
            e.mem.set_f32(rect + 4 * i as u32, *v);
        }
        let flip = Rc::new(RefCell::new(false));
        let flag = flip.clone();
        e.register_double(FLIP_TEXTURE_V, move |_, _| ret(u32::from(*flag.borrow())));
        let pairs: Rc<RefCell<Vec<(u32, f32, f32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = pairs.clone();
        e.register_double(PUSH_TEX_COORD, move |e, a| {
            sink.borrow_mut()
                .push((a[0], e.mem.f32(a[1]), e.mem.f32(a[1] + 4), a[2]));
            Ret::default()
        });
        e.call(0x00b3_1070, &args![geometry, 5u8, Ptr::<()>::new(rect)]);
        let expected_u = ((0.5f32 as f64) * (7.0 - 3.0) + 3.0) as f32;
        let expected_v = ((0.25f32 as f64) * (2.0 - 1.0) + 1.0) as f32;
        assert_eq!(
            *pairs.borrow(),
            vec![(g, expected_u, expected_v, 0xFFFF_FFFF)]
        );
        // The flag negates v.
        *flip.borrow_mut() = true;
        pairs.borrow_mut().clear();
        e.call(0x00b3_1070, &args![geometry, 5u8, Ptr::<()>::new(rect)]);
        assert_eq!(
            *pairs.borrow(),
            vec![(g, expected_u, -expected_v, 0xFFFF_FFFF)]
        );
        // Another texture index: nothing is pushed.
        pairs.borrow_mut().clear();
        e.call(0x00b3_1070, &args![geometry, 4u8, Ptr::<()>::new(rect)]);
        assert!(pairs.borrow().is_empty());
        // A vertex past the index vector, and one past the coordinates, are invalid.
        e.mem.set_u16(g + 0x22, 3);
        assert!(panics(|| {
            fn_00b31070(&mut e, geometry, 5, Ptr::new(rect));
        }));
        e.mem.set_u16(g + 0x22, 2);
        e.mem.set_u32(g + 0x118 + 0x10, coords + 20);
        assert!(panics(|| {
            fn_00b31070(&mut e, geometry, 2, Ptr::new(rect));
        }));
        // The second coordinate alone out of range.
        e.mem.set_u16(g + 0x22, 1);
        e.mem.set_u32(g + 0x118 + 0x10, coords + 12);
        assert!(panics(|| {
            fn_00b31070(&mut e, geometry, 5, Ptr::new(rect));
        }));
    }

    #[test]
    fn signalling_nan_in_the_rectangle_is_quieted_like_the_x87() {
        let mut e = engine();
        let geometry = e.new_object::<CIndexedGeometry>();
        let g = geometry.addr();
        let (_, indices) = vector(&mut e, 1, 1, 1);
        e.mem.set_u32(g + 0x130 + 0x0C, indices);
        e.mem.set_u32(g + 0x130 + 0x10, indices + 1);
        let (_, coords) = vector(&mut e, 4, 2, 2);
        e.mem.set_u32(g + 0x118 + 0x0C, coords);
        e.mem.set_u32(g + 0x118 + 0x10, coords + 8);
        e.mem.set_f32(coords, 1.0);
        e.mem.set_f32(coords + 4, 1.0);
        let rect = e.mem.alloc(0x18);
        e.mem.set_u32(rect, 0x7F80_0001);
        e.mem.set_f32(rect + 4, 2.0);
        e.mem.set_f32(rect + 8, 0.5);
        e.mem.set_f32(rect + 0x14, 0.5);
        e.register(FLIP_TEXTURE_V, |_, _| Ret::default());
        let bits = Rc::new(RefCell::new(Vec::new()));
        let sink = bits.clone();
        e.register_double(PUSH_TEX_COORD, move |e, a| {
            sink.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        e.call(0x00b3_1070, &args![geometry, 0u8, Ptr::<()>::new(rect)]);
        // The signalling NaN operand comes out quiet, payload kept.
        assert_eq!(bits.borrow()[0], 0x7FC0_0001);
    }

    #[test]
    fn sized_constructors_fill_with_default_elements() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        let proxy_block = e.mem.alloc(4);
        let proxies = spy(&mut e, PROXY_ALLOCATE, proxy_block);
        // 00b311b0: the proxy construct, then the fill with a zero word.
        let built = spy(&mut e, PROXY_CONSTRUCT, 0);
        let filled: Rc<RefCell<Vec<(u32, u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = filled.clone();
        e.register_double(CONSTRUCT_FILLED, move |e, a| {
            sink.borrow_mut().push((a[0], a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        e.mem.set_u32(this, 0x1234);
        assert_eq!(
            e.call(0x00b3_11b0, &args![Ptr::<()>::new(this), 7u32])
                .u32(),
            this
        );
        assert_eq!(*built.borrow(), vec![vec![this, this]]);
        assert_eq!(*filled.borrow(), vec![(this, 7, 0)]);
        assert!(proxies.borrow().is_empty());
        // 00b31350 and 00b31ad0: a proxy, a default element at a local, the fill, the destroy
        // and the delete of the local's proxy.
        for (addr, build, fill, destroy) in [
            (
                0x00b3_1350u32,
                0x00b3_36a0u32,
                0x00b3_2780u32,
                0x00b3_4d90u32,
            ),
            (0x00b3_1ad0, 0x00b1_7db0, 0x00b3_2ce0, 0x00b1_7f50),
        ] {
            let mut e = engine();
            let this = e.mem.alloc(0x18);
            let proxy_block = e.mem.alloc(4);
            spy(&mut e, PROXY_ALLOCATE, proxy_block);
            let order: Rc<RefCell<Vec<(u32, Vec<u32>)>>> = Rc::new(RefCell::new(Vec::new()));
            for address in [build, fill, destroy, OPERATOR_DELETE] {
                let sink = order.clone();
                e.register_double(address, move |_, a| {
                    sink.borrow_mut().push((address, a.to_vec()));
                    ret(a[0])
                });
            }
            let r = e.call(addr, &args![Ptr::<()>::new(this), 9u32]);
            assert_eq!(r.u32(), this);
            // The vector's proxy block points back at the vector and is stored at its start.
            assert_eq!(e.mem.u32(proxy_block), this);
            assert_eq!(e.mem.u32(this), proxy_block);
            let calls = order.borrow().clone();
            let addresses: Vec<u32> = calls.iter().map(|(a, _)| *a).collect();
            assert_eq!(addresses, vec![build, fill, destroy, OPERATOR_DELETE]);
            let local = calls[0].1[0];
            // The fill gets the vector, the count and what the build returned (the local).
            assert_eq!(calls[1].1[..3], [this, 9, local]);
            assert_eq!(calls[2].1[0], local);
        }
    }

    #[test]
    fn default_constructor_makes_the_proxy_and_empty_storage() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        let proxy_block = e.mem.alloc(4);
        let proxies = spy(&mut e, PROXY_ALLOCATE, proxy_block);
        let buys = spy(&mut e, BUY_ELEMENTS, 0);
        let r = e.call(0x00b3_1270, &args![Ptr::<()>::new(this)]);
        assert_eq!(r.u32(), this);
        assert_eq!(*proxies.borrow(), vec![vec![1, 0]]);
        assert_eq!(e.mem.u32(proxy_block), this);
        assert_eq!(e.mem.u32(this), proxy_block);
        assert_eq!(*buys.borrow(), vec![vec![this, 0]]);
        // A failed proxy allocation leaves a null proxy.
        spy(&mut e, PROXY_ALLOCATE, 0);
        e.call(0x00b3_1270, &args![Ptr::<()>::new(this)]);
        assert_eq!(e.mem.u32(this), 0);
    }

    #[test]
    fn checked_indexing() {
        let mut e = engine();
        let (words, wdata) = vector(&mut e, 4, 3, 4);
        assert_eq!(
            e.call(0x00b3_1230, &args![Ptr::<()>::new(words), 2u32])
                .u32(),
            wdata + 8
        );
        assert!(panics(|| {
            fn_00b31230(&mut e, Ptr::new(words), 3);
        }));
        let (structs, sdata) = vector(&mut e, 0x18, 3, 4);
        assert_eq!(
            e.call(0x00b3_19a0, &args![Ptr::<()>::new(structs), 2u32])
                .u32(),
            sdata + 0x30
        );
        assert!(panics(|| {
            fn_00b319a0(&mut e, Ptr::new(structs), 3);
        }));
    }

    #[test]
    fn destructors_tidy_and_free_the_proxy() {
        for (addr, tidy) in [(0x00b3_1480u32, 0x00b3_2c60u32), (0x00b3_1c00, 0x00b3_31c0)] {
            let mut e = engine();
            let this = e.mem.alloc(0x18);
            e.mem.set_u32(this, 0x7777);
            let tidied = spy(&mut e, tidy, 0);
            let freed = spy(&mut e, OPERATOR_DELETE, 0);
            e.call(addr, &args![Ptr::<()>::new(this)]);
            assert_eq!(*tidied.borrow(), vec![vec![this]]);
            assert_eq!(*freed.borrow(), vec![vec![0x7777]]);
        }
    }

    #[test]
    fn clear_erases_the_whole_range() {
        for (addr, erase) in [(0x00b3_19e0u32, 0x00b3_2ad0u32), (0x00b3_2120, 0x00b3_3030)] {
            let mut e = engine();
            let (v, data) = vector(&mut e, 0x18, 3, 4);
            let erased = spy(&mut e, erase, 0);
            e.call(addr, &args![Ptr::<()>::new(v)]);
            let log = erased.borrow().clone();
            assert_eq!(log.len(), 1);
            // (this, result slot, first proxy, first, last proxy, last)
            let proxy = 0x5000_0001;
            assert_eq!(log[0][0], v);
            assert_eq!(log[0][2..], [proxy, data, proxy, data + 0x48]);
            // A first pointer past the last is invalid.
            e.mem.set_u32(v + 0x10, data - 0x18);
            assert!(panics(|| {
                e.call(addr, &args![Ptr::<()>::new(v)]);
            }));
        }
    }

    /// The doubles of one vector-of-vectors assignment; returns the log of (address, words).
    fn assign_doubles(
        e: &mut Engine,
        callees: &VectorOfVectors,
        buy_result: u32,
    ) -> Rc<RefCell<Vec<(u32, Vec<u32>)>>> {
        let log: Rc<RefCell<Vec<(u32, Vec<u32>)>>> = Rc::new(RefCell::new(Vec::new()));
        for (address, result) in [
            (callees.clear, 0),
            (callees.assign_range, 0x4444_0000),
            (callees.erase_tail, 0),
            (callees.assign_into, 0),
            (callees.copy_construct, 0x5555_0000),
            (callees.element_destructor, 0),
            (FREE_PROXY, 0),
            (OPERATOR_DELETE, 0),
            (BUY_ELEMENTS, buy_result),
        ] {
            let sink = log.clone();
            e.register_double(address, move |_, a| {
                sink.borrow_mut().push((address, a.to_vec()));
                ret(result)
            });
        }
        log
    }

    #[test]
    fn assignment_in_every_case() {
        for (addr, callees) in [(0x00b3_14f0u32, &STRIP_LENGTHS), (0x00b3_1c70, &STRIPS)] {
            // Self-assignment does nothing.
            let mut e = engine();
            let log = assign_doubles(&mut e, callees, 1);
            let (a, _) = vector(&mut e, 0x18, 2, 4);
            let this = Ptr::<()>::new(a);
            assert_eq!(e.call(addr, &args![this, this]).u32(), a);
            assert!(log.borrow().is_empty());
            // An empty source clears.
            let (empty, _) = vector(&mut e, 0x18, 0, 0);
            e.call(addr, &args![this, Ptr::<()>::new(empty)]);
            assert_eq!(*log.borrow(), vec![(callees.clear, vec![a])]);
            // A source no larger than the target: assign over, erase the tail, set the end.
            log.borrow_mut().clear();
            let (small, sdata) = vector(&mut e, 0x18, 1, 1);
            let (target, tdata) = vector(&mut e, 0x18, 3, 4);
            e.call(addr, &args![Ptr::<()>::new(target), Ptr::<()>::new(small)]);
            assert_eq!(
                *log.borrow(),
                vec![
                    (
                        callees.assign_range,
                        vec![sdata, sdata + 0x18, tdata, 0, 0, 0]
                    ),
                    (callees.erase_tail, vec![target, 0x4444_0000, tdata + 0x48]),
                ]
            );
            assert_eq!(e.mem.u32(target + 0x10), tdata + 0x18);
            // A larger source within the capacity: assign the head, construct the rest.
            log.borrow_mut().clear();
            let (big, bdata) = vector(&mut e, 0x18, 3, 3);
            let (target, tdata) = vector(&mut e, 0x18, 1, 4);
            e.call(addr, &args![Ptr::<()>::new(target), Ptr::<()>::new(big)]);
            assert_eq!(
                *log.borrow(),
                vec![
                    (callees.assign_into, vec![bdata, bdata + 0x18, tdata]),
                    (
                        callees.copy_construct,
                        vec![bdata + 0x18, bdata + 0x48, tdata + 0x18, target + 8, 0, 0]
                    ),
                ]
            );
            assert_eq!(e.mem.u32(target + 0x10), 0x5555_0000);
            // Beyond the capacity: destroy the elements (and free the old storage), buy, copy.
            log.borrow_mut().clear();
            let (target, tdata) = vector(&mut e, 0x18, 2, 2);
            e.call(addr, &args![Ptr::<()>::new(target), Ptr::<()>::new(big)]);
            assert_eq!(
                *log.borrow(),
                vec![
                    (callees.element_destructor, vec![tdata]),
                    (FREE_PROXY, vec![tdata]),
                    (callees.element_destructor, vec![tdata + 0x18]),
                    (FREE_PROXY, vec![tdata + 0x18]),
                    (OPERATOR_DELETE, vec![tdata]),
                    (BUY_ELEMENTS, vec![target, 3]),
                    (
                        callees.copy_construct,
                        vec![bdata, bdata + 0x48, tdata, target + 8, 0, 0]
                    ),
                ]
            );
            assert_eq!(e.mem.u32(target + 0x10), 0x5555_0000);
            // A target without storage destroys nothing; a failed buy copies nothing.
            let mut e = engine();
            let log = assign_doubles(&mut e, callees, 0);
            let (big, _) = vector(&mut e, 0x18, 3, 3);
            let (target, _) = vector(&mut e, 0x18, 0, 0);
            for off in [0x0C, 0x10, 0x14] {
                e.mem.set_u32(target + off, 0);
            }
            e.call(addr, &args![Ptr::<()>::new(target), Ptr::<()>::new(big)]);
            assert_eq!(*log.borrow(), vec![(BUY_ELEMENTS, vec![target, 3])]);
            assert_eq!(e.mem.u32(target + 0x10), 0);
        }
    }
}
