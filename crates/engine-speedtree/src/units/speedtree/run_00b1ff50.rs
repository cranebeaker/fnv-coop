//! `SpeedTree/run_00b1ff50` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The MSVC `std::vector` instantiations of the frond code (the engine map names none of
//! them): the copy constructors of `SFrondGuide` (0x38 bytes), of the vertex vector (elements
//! 0x38 bytes), of the guide vector, of the texture element `SFrondTexture` (0x2C bytes) and of
//! a `std::vector<float>`, the `_Insert_n` of the vectors of vertices, guide vectors (0x18-byte
//! elements), textures, `CVec` (0xC bytes) and float vectors, the checked iterator advances, the
//! element allocators, the element copies and the introspective sort of the guide vector.
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
    checked_iterator, copy_float, vector_len, StlVector, INVALID_PARAMETER, OPERATOR_DELETE,
    OPERATOR_NEW,
};
use super::frondengine::{SFrondGuide, SFrondTexture};

/// The proxy allocation of the checked-iterator `std::vector` constructors (`0044d670`,
/// `cdecl`, `(1, 0)`).
const ALLOCATE_PROXY: u32 = 0x0044_d670;
/// `std::vector<SFrondVertex>::_Buy(this, count)` (`00b1eb00`, `RET 4`): allocates room for
/// `count` elements, answering whether any was made.
const VERTEX_VECTOR_BUY: u32 = 0x00b1_eb00;
/// The length error the `std::vector` code raises for a request above the maximum size (it does
/// not return).
const LENGTH_ERROR: u32 = 0x00b0_fb80;
/// Whether two checked iterators differ (`00b16830`, `thiscall`: this and the other iterator,
/// both addresses of a proxy and a position).
const ITERATORS_DIFFER: u32 = 0x00b1_6830;
/// The destructor the linker shares between the empty classes (`CVec`): it does nothing
/// (`00aa7830`).
const TRIVIAL_DESTRUCTOR: u32 = 0x00aa_7830;
/// The `std::string` helpers: `_Tidy(built, new size)` (`0044a730`; `(0, 0)` makes the string
/// empty, `(1, 0)` destroys it) and `assign(source, offset, count)` (`0044a580`); `[010173f0]` is
/// the `npos` the string assignments pass as their count.
const STRING_TIDY: u32 = 0x0044_a730;
const STRING_ASSIGN: u32 = 0x0044_a580;
const STRING_NPOS: u32 = 0x0101_73f0;
/// The `std::exception` base constructor taking a message pointer (`00ec78f1`), `bad_alloc`'s
/// vtable and throw information, and `_CxxThrowException` (`00ec7894`; it does not return).
const EXCEPTION_FROM_MESSAGE: u32 = 0x00ec_78f1;
const BAD_ALLOC_VTABLE: u32 = 0x0101_76f8;
const BAD_ALLOC_THROW_INFO: u32 = 0x0114_0ef8;
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// The vertex vector's range helpers in `run_00b23f10`: the copy of a range to uninitialised
/// storage (`00b26db0`: first, last, destination, the allocator and two flag bytes), the same
/// with four words (`00b264a0`: first, last, destination, the allocator), the fill with copies
/// of one vertex (`00b23f60`: destination, count, value, the allocator, two flags) and the copy
/// of a range up to its end (`00b24690`: first, last, destination end, the allocator).
const VERTEX_COPY_RANGE: u32 = 0x00b2_6db0;
const VERTEX_RELOCATE: u32 = 0x00b2_64a0;
const VERTEX_FILL: u32 = 0x00b2_3f60;
const VERTEX_TAIL_COPY: u32 = 0x00b2_4690;
/// The guide vector's copy of a range to uninitialised storage (`00b25fa0`: first, last,
/// destination, the allocator, two flags), in `run_00b23f10`.
const GUIDE_COPY_RANGE: u32 = 0x00b2_5fa0;
/// The guide-vector vector's helpers in `run_00b23f10`: the move of a range into uninitialised
/// storage (`00b25ca0`: first, last, destination, the allocator, two flags), the fill with copies of
/// one guide vector (`00b24290`: destination, count, value, the allocator, two flags), the
/// guide vector's `operator=` (`00b24f40`: this, the other), and, from other units, its tidy
/// (`00b1ec70`: this) and the release of a proxy block (`00b39460`: `operator delete` of the
/// word at `this`).
const GUIDE_VECTOR_COPY_RANGE: u32 = 0x00b2_5ca0;
const GUIDE_VECTOR_FILL: u32 = 0x00b2_4290;
const GUIDE_VECTOR_ASSIGN: u32 = 0x00b2_4f40;
const GUIDE_VECTOR_TIDY: u32 = 0x00b1_ec70;
const PROXY_RELEASE: u32 = 0x00b3_9460;
/// The texture vector's helpers in `run_00b23f10`: the head copy of a range (`00b23f10`: first,
/// last, destination, the allocator), its tail copy (`00b247b0`: the same), the fill (`00b243a0`:
/// destination, count, value, the allocator, two flags) and the assignment of one texture
/// (`00b24a80`: this, the other).
const TEXTURE_RELOCATE: u32 = 0x00b2_3f10;
const TEXTURE_TAIL_COPY: u32 = 0x00b2_47b0;
const TEXTURE_FILL: u32 = 0x00b2_43a0;
const TEXTURE_ASSIGN: u32 = 0x00b2_4a80;
/// The `CVec` vector's helpers: the range copy (`00b17740`: first, last, destination, the
/// allocator), the tail copy (`00b16dc0`: first, last, destination end, the allocator), the
/// move up (`00b169c0`: first, last, end of the destination), from other units, and the fill
/// (`00b244a0`: destination, count, value, the allocator, two flags) in `run_00b23f10`.
const CVEC_COPY_RANGE: u32 = 0x00b1_7740;
const CVEC_TAIL_COPY: u32 = 0x00b1_6dc0;
const CVEC_MOVE_UP: u32 = 0x00b1_69c0;
const CVEC_FILL: u32 = 0x00b2_44a0;
/// The float-vector vector's helpers: the move of a range into uninitialised storage
/// (`00b25e20`: first, last, destination, the allocator, two flags) and the fill (`00b24580`)
/// in `run_00b23f10`; the element tidy (`0065f240`), the release of a temporary (`0065d6e0`)
/// and the float vector's `operator=` (`00b1d5d0`, `RET 4`) from other units.
const FLOAT_VECTOR_COPY_RANGE: u32 = 0x00b2_5e20;
const FLOAT_VECTOR_FILL: u32 = 0x00b2_4580;
const FLOAT_VECTOR_TIDY: u32 = 0x0065_f240;
const FLOAT_VECTOR_RELEASE: u32 = 0x0065_d6e0;
const FLOAT_VECTOR_ASSIGN: u32 = 0x00b1_d5d0;
/// The float vector's own helpers used by its copy constructor: the proxy construction
/// (`0065dc90`, `thiscall`, `RET 4`), the reserve (`0044a240`, `RET 4`) and the copy of words
/// (`00b24960`: two iterators as proxy and address, the destination, the allocator).
const FLOAT_VECTOR_CONSTRUCT_PROXY: u32 = 0x0065_dc90;
const FLOAT_VECTOR_RESERVE: u32 = 0x0044_a240;
const FLOAT_COPY_WORDS: u32 = 0x00b2_4960;
/// The guide assignment (`00b249d0`, `thiscall`, one argument) and the backward assignment loops
/// of `run_00b23f10`: guides (`00b240b0`), vertices as words (`00b24710`) and textures
/// (`00b24830`), each taking `(first, last, destination end)` and three flag words.
const GUIDE_ASSIGN: u32 = 0x00b2_49d0;
const GUIDE_RANGE_BACKWARD: u32 = 0x00b2_40b0;
const VERTEX_RANGE_BACKWARD: u32 = 0x00b2_4710;
const TEXTURE_RANGE_BACKWARD: u32 = 0x00b2_4830;
/// The vector swaps of the guide-vector (`00b267a0`) and float-vector (`00b26570`) elements.
const GUIDE_VECTOR_MOVE_ELEMENT: u32 = 0x00b2_67a0;
const FLOAT_VECTOR_MOVE_ELEMENT: u32 = 0x00b2_6570;
/// The sort helpers of `run_00b23f10`: the partition (`00b25680`: out pair, first, last, the
/// comparison byte), the heap maker (`00b26260`: first, last, comparison, two zeros), the pop
/// (`00b26ca0`: first, last, comparison) and the insertion sort (`00b262d0`: first, last,
/// comparison, zero).
const GUIDE_PARTITION: u32 = 0x00b2_5680;
const GUIDE_MAKE_HEAP: u32 = 0x00b2_6260;
const GUIDE_POP_HEAP: u32 = 0x00b2_6ca0;
const GUIDE_INSERTION_SORT: u32 = 0x00b2_62d0;

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

/// What `_Insert_n` decides before it touches an element (every instance has the same
/// prologue): nothing to insert, a request above the maximum size, storage to grow to
/// (`Grow` holds the new capacity), or room for the elements.
enum Room {
    Nothing,
    TooLong,
    Grow(u32),
    Fits,
}

/// The `_Insert_n` prologue for elements of `size` bytes and a vector that can hold at most
/// `limit` of them: with `count` elements to insert, the capacity grows by half (at least to the
/// new size) when the new size does not fit.
fn insert_room(e: &Engine, vector: u32, size: i32, limit: u32, count: u32) -> Room {
    // The exe tests `limit > 0` before using the limit (a constant, so the 1 is never used).
    let limit = limit.max(1);
    let capacity = vector_capacity(e, vector, size);
    if count == 0 {
        return Room::Nothing;
    }
    let len = vector_len(e, vector, size);
    if limit.wrapping_sub(len) < count {
        return Room::TooLong;
    }
    let required = len.wrapping_add(count);
    if capacity >= required {
        return Room::Fits;
    }
    let mut grown = if limit.wrapping_sub(capacity >> 1) < capacity {
        0
    } else {
        (capacity >> 1).wrapping_add(capacity)
    };
    if grown < required {
        grown = required;
    }
    Room::Grow(grown)
}

/// The tail every growing `_Insert_n` ends with: the new end of the storage and of the elements
/// and the new first pointer (the old storage was released by the caller).
fn finish_growth(
    e: &mut Engine,
    vector: u32,
    size: u32,
    storage: u32,
    capacity: u32,
    new_len: u32,
) {
    e.mem.set_u32(
        vector + 0x14,
        capacity.wrapping_mul(size).wrapping_add(storage),
    );
    e.mem.set_u32(
        vector + 0x10,
        new_len.wrapping_mul(size).wrapping_add(storage),
    );
    e.mem.set_u32(vector + 0x0C, storage);
}

/// The `bad_alloc` throw of the element allocators (`_Allocate`): a count whose byte size
/// overflows. `__CxxThrowException` does not return.
fn throw_bad_alloc(e: &mut Engine) -> ! {
    // The exception is built from a pointer to a null `const char *`.
    e.with_stack(4, |e, message| {
        e.mem.set_u32(message.addr(), 0);
        e.with_stack(0xC, |e, exception| {
            e.call(EXCEPTION_FROM_MESSAGE, &args![exception, message]);
            e.mem.set_u32(exception.addr(), BAD_ALLOC_VTABLE);
            e.call(CXX_THROW_EXCEPTION, &args![exception, BAD_ALLOC_THROW_INFO]);
        });
    });
    unreachable!("__CxxThrowException does not return (00ec7894)")
}

/// The element allocators: a count of zero allocates nothing (the byte count is zero), a count
/// whose byte size does not fit throws `std::bad_alloc`, else `operator new(count * size)`.
fn allocate_elements(e: &mut Engine, count: u32, size: u32) -> u32 {
    if count > 0 && 0xffff_ffffu32 / count < size {
        throw_bad_alloc(e);
    }
    e.call(OPERATOR_NEW, &args![count.wrapping_mul(size)]).u32()
}

/// The checked iterator advance of the `std::vector` code: the iterator at `this` (a proxy and
/// a position) must have a proxy, and the new position `position + count * size` must lie in
/// the owner's `first..=last` (else the invalid-parameter path); then it moves.
fn advance_iterator(e: &mut Engine, this: u32, count: i32, size: u32) -> u32 {
    if e.mem.u32(this) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let moved = (count as u32)
        .wrapping_mul(size)
        .wrapping_add(e.mem.u32(this + 4));
    let last = e.mem.u32(iterator_owner(e, this) + 0x10);
    if moved <= last {
        let first = e.mem.u32(iterator_owner(e, this) + 0x0C);
        if first > moved {
            e.call(INVALID_PARAMETER, &args![]);
        }
    } else {
        e.call(INVALID_PARAMETER, &args![]);
    }
    e.mem.set_u32(this + 4, moved);
    this
}

/// The vector a checked iterator belongs to: the word 0 of its proxy, zero without a proxy.
fn iterator_owner(e: &Engine, iterator: u32) -> u32 {
    let proxy = e.mem.u32(iterator);
    if proxy == 0 {
        0
    } else {
        e.mem.u32(proxy)
    }
}

// Translated from 00b1ff50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SFrondGuide::SFrondGuide(const SFrondGuide &)` (the engine map gives it no name; `thiscall`,
/// one argument): copy-constructs the vertex vector (`00b200f0`) and copies the length, radius
/// (floats, through the x87 stack), the frond map index byte, the offset angle, the surface
/// area and fuzzy surface area (floats) and the vertex start index and vertices-per-vertex
/// count (words). Returns `this`.
pub fn fn_00b1ff50(
    e: &mut Engine,
    this: Ptr<SFrondGuide>,
    other: Ptr<SFrondGuide>,
) -> Ptr<SFrondGuide> {
    let (t, o) = (this.addr(), other.addr());
    fn_00b200f0(e, this.cast(), other.cast());
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

// Translated from 00b1ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked range erase of the guide-vector vector (0x18-byte elements; the engine map gives
/// it no name; `thiscall`, five arguments): the iterators at `first` and `last` (their proxy
/// words, the second and fourth arguments, are not read) are made with the range checks; when
/// they differ the elements from `last` to the end are copied down over the erased ones
/// (`00b236a0`), the elements that are left past the new end are destroyed (`00aa7830`, which
/// does nothing) and the end pointer is set to the new end. Writes the iterator at `first`
/// (proxy, position) to `out` and returns it.
pub fn fn_00b1ffd0(
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
    e.with_stack(0x10, |e, frame| {
        let a = frame.addr();
        let b = a + 8;
        e.mem.set_u32(a, first_proxy);
        e.mem.set_u32(a + 4, first);
        e.mem.set_u32(b, last_proxy);
        e.mem.set_u32(b + 4, last);
        if e.call(ITERATORS_DIFFER, &args![a, b]).bool() {
            let end = e.mem.u32(t + 0x10);
            let new_end = fn_00b236a0(e, last, end, first);
            let end = e.mem.u32(t + 0x10);
            let mut element = new_end;
            while element != end {
                e.call(TRIVIAL_DESTRUCTOR, &args![element]);
                if delete_flag(NO_DELETE) {
                    e.call(OPERATOR_DELETE, &args![element]);
                }
                element = element.wrapping_add(0x18);
            }
            e.mem.set_u32(t + 0x10, new_end);
        }
    });
    e.mem.set_u32(out.addr(), first_proxy);
    e.mem.set_u32(out.addr() + 4, first);
    out
}

// Translated from 00b200f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b200f0 - the FS:[0] exception frame (the handler that destroys the
// half-built vector) is not translated.
/// The copy constructor of the vertex vector (a `std::vector<SFrondVertex>`; the engine map
/// gives it no name; `thiscall`, one argument): builds the proxy (`00b20ba0`), reserves room for
/// the other vector's element count (`00b1eb00`) and, when that made room, copies the other's
/// elements to the new storage (`00b26db0`) and sets the end pointer after them. Returns `this`.
pub fn fn_00b200f0(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) -> Ptr<StlVector> {
    let (t, o) = (this.addr(), other.addr());
    fn_00b20ba0(e, this, 0);
    let count = (e.mem.u32(o + 0x10).wrapping_sub(e.mem.u32(o + 0x0C)) as i32 / 0x38) as u32;
    if e.call(VERTEX_VECTOR_BUY, &args![this, count]).bool() {
        let last = e.mem.u32(o + 0x10);
        checked_iterator(e, o, last);
        let first = e.mem.u32(o + 0x0C);
        checked_iterator(e, o, first);
        let storage = e.mem.u32(t + 0x0C);
        let end = e
            .call(
                VERTEX_COPY_RANGE,
                &args![first, last, storage, t + 8, 0u32, 0u32],
            )
            .u32();
        e.mem.set_u32(t + 0x10, end);
    }
    this
}

// Translated from 00b20360 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b20360 - the FS:[0] exception frame (the handler that frees the new
// storage and destroys the copied elements if a copy throws) is not translated.
/// `_Insert_n` of the vertex vector (0x38-byte elements; the engine map gives it no name;
/// `thiscall`, four arguments, the first, a proxy, is not read): inserts `count` copies of the
/// vertex at `value` before `position`. A count of zero does nothing; a result above the maximum
/// size (`0x4924924` elements) raises the length error (`00b0fb80`). Without room the vector
/// grows (`00b23980`): the elements before the position are copied to the new storage
/// (`00b264a0`), the copies of the value made (`00b23f60`) and the later elements copied after
/// them, the old storage freed. With room, the value is first copied to a local (14 words);
/// when fewer elements follow the position than are inserted the tail moves up (`00b264a0`),
/// copies of the value are made past the old end (`00b23f60`) and the range is overwritten with
/// the value; otherwise the last `count` elements are copied past the end (`00b24690`), the
/// others moved up (`00b23930`) and the range overwritten.
pub fn fn_00b20360(
    e: &mut Engine,
    this: Ptr<StlVector>,
    _unused_1: u32,
    position: u32,
    count: u32,
    value: Ptr,
) {
    let t = this.addr();
    match insert_room(e, t, 0x38, 0x0492_4924, count) {
        Room::Nothing => {}
        Room::TooLong => {
            e.call(LENGTH_ERROR, &args![]);
        }
        Room::Grow(capacity) => {
            let storage = fn_00b23980(e, capacity, 0);
            let first = e.mem.u32(t + 0x0C);
            let copied = e
                .call(VERTEX_RELOCATE, &args![first, position, storage, t + 8])
                .u32();
            e.call(VERTEX_FILL, &args![copied, count, value, t + 8, 0u32, 0u32]);
            let filled = count.wrapping_mul(0x38).wrapping_add(copied);
            let last = e.mem.u32(t + 0x10);
            e.call(VERTEX_RELOCATE, &args![position, last, filled, t + 8]);
            let new_len = vector_len(e, t, 0x38).wrapping_add(count);
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
            finish_growth(e, t, 0x38, storage, capacity, new_len);
        }
        Room::Fits => {
            let after = (e.mem.u32(t + 0x10).wrapping_sub(position) as i32 / 0x38) as u32;
            let mut temporary = [0u32; 14];
            for (i, word) in temporary.iter_mut().enumerate() {
                *word = e.mem.u32(value.addr() + 4 * i as u32);
            }
            if after < count {
                let last = e.mem.u32(t + 0x10);
                e.call(
                    VERTEX_RELOCATE,
                    &args![
                        position,
                        last,
                        count.wrapping_mul(0x38).wrapping_add(position),
                        t + 8
                    ],
                );
                e.with_stack(0x38, |e, copy| {
                    for (i, word) in temporary.iter().enumerate() {
                        e.mem.set_u32(copy.addr() + 4 * i as u32, *word);
                    }
                    let last = e.mem.u32(t + 0x10);
                    let surplus =
                        count.wrapping_sub((last.wrapping_sub(position) as i32 / 0x38) as u32);
                    e.call(VERTEX_FILL, &args![last, surplus, copy, t + 8, 0u32, 0u32]);
                });
                let end = count.wrapping_mul(0x38).wrapping_add(e.mem.u32(t + 0x10));
                e.mem.set_u32(t + 0x10, end);
                let stop = e.mem.u32(t + 0x10).wrapping_sub(count.wrapping_mul(0x38));
                let mut element = position;
                while element != stop {
                    for (i, word) in temporary.iter().enumerate() {
                        e.mem.set_u32(element + 4 * i as u32, *word);
                    }
                    element = element.wrapping_add(0x38);
                }
            } else {
                let old_last = e.mem.u32(t + 0x10);
                let tail = old_last.wrapping_sub(count.wrapping_mul(0x38));
                let this_last = e.mem.u32(t + 0x10);
                let end = e
                    .call(VERTEX_TAIL_COPY, &args![tail, old_last, this_last, t + 8])
                    .u32();
                e.mem.set_u32(t + 0x10, end);
                fn_00b23930(e, position, tail, old_last);
                let stop = count.wrapping_mul(0x38).wrapping_add(position);
                let mut element = position;
                while element != stop {
                    for (i, word) in temporary.iter().enumerate() {
                        e.mem.set_u32(element + 4 * i as u32, *word);
                    }
                    element = element.wrapping_add(0x38);
                }
            }
        }
    }
}

// Translated from 00b20ba0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b20ba0 - the FS:[0] exception frame is not translated.
/// The proxy construction of the vertex vector, the guide vector and their kin (the engine map
/// gives it no name; `thiscall`, `RET 4`: the one stack word is not read; the callers push a
/// dummy): allocates the 8-byte proxy (`0044d670`, `(1, 0)`), stores `this` in its first word
/// when it was allocated, and stores the proxy (or zero) in the vector's first word. Returns
/// `this`.
pub fn fn_00b20ba0(e: &mut Engine, this: Ptr<StlVector>, _unused_1: u32) -> Ptr<StlVector> {
    let proxy = e.call(ALLOCATE_PROXY, &args![1u32, 0u32]).u32();
    let stored = if proxy == 0 {
        0
    } else {
        e.mem.set_u32(proxy, this.addr());
        proxy
    };
    e.mem.set_u32(this.addr(), stored);
    this
}

// Translated from 00b20c30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b20c30 - the FS:[0] exception frame is not translated.
/// Destroys the guide vectors in `first..last` (0x18-byte elements; the engine map gives it no
/// name; `thiscall`, two arguments): for each, the guide vector's tidy (`00b1ec70`) and the
/// release of its proxy block (`operator delete` of the element's first word). The second
/// delete of the exe's loop (guarded by a flag the compiler folded to zero) never runs.
pub fn fn_00b20c30(e: &mut Engine, _this: u32, first: u32, last: u32) {
    let mut element = first;
    while element != last {
        e.call(GUIDE_VECTOR_TIDY, &args![element]);
        let proxy = e.mem.u32(element);
        e.call(OPERATOR_DELETE, &args![proxy]);
        if delete_flag(NO_DELETE) {
            e.call(OPERATOR_DELETE, &args![element]);
        }
        element = element.wrapping_add(0x18);
    }
}

// Translated from 00b20ce0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b20ce0 - the FS:[0] exception frame (the handlers that unwind the
// new storage and the temporary guide vector if a copy throws) is not translated.
/// `_Insert_n` of the guide-vector vector (0x18-byte elements, each a guide vector; the engine
/// map gives it no name; `thiscall`, four arguments, the first, a proxy, is not read): inserts
/// `count` copies of the guide vector at `value` before `position`. A count of zero does
/// nothing; a result above the maximum size (`0xAAAAAAA` elements) raises the length error
/// (`00b0fb80`). Without room the vector grows (`00b23aa0`): the elements before the position
/// are copied to the new storage (`00b25ca0`), the copies of the value made (`00b24290`) and the
/// later elements copied after them, the old elements destroyed (`00b20c30`) and the old block
/// freed. With room, a temporary copy of the value is made (`00b231f0`); when fewer elements
/// follow the position than are inserted the tail moves up (`00b25ca0`), copies of the value
/// are made past the old end and the range is assigned the temporary (`00b24f40`); otherwise the
/// last `count` elements are copied past the end (`00b25ca0`), the others moved up (`00b239e0`)
/// and the range assigned. The temporary is destroyed with `00b1ec70` and its proxy released.
pub fn fn_00b20ce0(
    e: &mut Engine,
    this: Ptr<StlVector>,
    _unused_1: u32,
    position: u32,
    count: u32,
    value: Ptr,
) {
    let t = this.addr();
    match insert_room(e, t, 0x18, 0x0aaa_aaaa, count) {
        Room::Nothing => {}
        Room::TooLong => {
            e.call(LENGTH_ERROR, &args![]);
        }
        Room::Grow(capacity) => {
            let storage = fn_00b23aa0(e, capacity, 0);
            let first = e.mem.u32(t + 0x0C);
            let copied = e
                .call(
                    GUIDE_VECTOR_COPY_RANGE,
                    &args![first, position, storage, t + 8, 0u32, 0u32],
                )
                .u32();
            e.call(
                GUIDE_VECTOR_FILL,
                &args![copied, count, value, t + 8, 0u32, 0u32],
            );
            let filled = count.wrapping_mul(0x18).wrapping_add(copied);
            let last = e.mem.u32(t + 0x10);
            e.call(
                GUIDE_VECTOR_COPY_RANGE,
                &args![position, last, filled, t + 8, 0u32, 0u32],
            );
            let new_len = vector_len(e, t, 0x18).wrapping_add(count);
            if e.mem.u32(t + 0x0C) != 0 {
                let first = e.mem.u32(t + 0x0C);
                let last = e.mem.u32(t + 0x10);
                fn_00b20c30(e, 0, first, last);
                let block = e.mem.u32(t + 0x0C);
                e.call(OPERATOR_DELETE, &args![block]);
            }
            finish_growth(e, t, 0x18, storage, capacity, new_len);
        }
        Room::Fits => {
            let after = (e.mem.u32(t + 0x10).wrapping_sub(position) as i32 / 0x18) as u32;
            e.with_stack(0x18, |e, temporary| {
                let temp = temporary.addr();
                fn_00b231f0(e, temporary.cast(), value.cast());
                if after < count {
                    let last = e.mem.u32(t + 0x10);
                    e.call(
                        GUIDE_VECTOR_COPY_RANGE,
                        &args![
                            position,
                            last,
                            count.wrapping_mul(0x18).wrapping_add(position),
                            t + 8,
                            0u32,
                            0u32
                        ],
                    );
                    let last = e.mem.u32(t + 0x10);
                    let surplus =
                        count.wrapping_sub((last.wrapping_sub(position) as i32 / 0x18) as u32);
                    e.call(
                        GUIDE_VECTOR_FILL,
                        &args![last, surplus, temp, t + 8, 0u32, 0u32],
                    );
                    let end = count.wrapping_mul(0x18).wrapping_add(e.mem.u32(t + 0x10));
                    e.mem.set_u32(t + 0x10, end);
                    let stop = e.mem.u32(t + 0x10).wrapping_sub(count.wrapping_mul(0x18));
                    let mut element = position;
                    while element != stop {
                        e.call(GUIDE_VECTOR_ASSIGN, &args![element, temp]);
                        element = element.wrapping_add(0x18);
                    }
                    e.call(GUIDE_VECTOR_TIDY, &args![temp]);
                    e.call(PROXY_RELEASE, &args![temp]);
                } else {
                    let old_last = e.mem.u32(t + 0x10);
                    let tail = old_last.wrapping_sub(count.wrapping_mul(0x18));
                    let this_last = e.mem.u32(t + 0x10);
                    let end = e
                        .call(
                            GUIDE_VECTOR_COPY_RANGE,
                            &args![tail, old_last, this_last, t + 8, 0u32, 0u32],
                        )
                        .u32();
                    e.mem.set_u32(t + 0x10, end);
                    fn_00b239e0(e, position, tail, old_last);
                    let stop = count.wrapping_mul(0x18).wrapping_add(position);
                    let mut element = position;
                    while element != stop {
                        e.call(GUIDE_VECTOR_ASSIGN, &args![element, temp]);
                        element = element.wrapping_add(0x18);
                    }
                    e.call(GUIDE_VECTOR_TIDY, &args![temp]);
                    let proxy = e.mem.u32(temp);
                    e.call(OPERATOR_DELETE, &args![proxy]);
                }
            });
        }
    }
}

// Translated from 00b21560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b21560 - the FS:[0] exception frame (the handlers that unwind the
// new storage and the temporary texture if a copy throws) is not translated, and the `/GS`
// cookie check (`00ec408c`) at its end is left out.
/// `_Insert_n` of the texture vector (0x2C-byte elements, each an `SFrondTexture`; the engine map
/// gives it no name; `thiscall`, four arguments, the first, a proxy, is not read): inserts
/// `count` copies of the texture at `value` before `position`. A count of zero does nothing; a
/// result above the maximum size (`0x5D1745D` elements) raises the length error (`00b0fb80`).
/// Without room the vector grows (`00b23b50`): the elements before the position are copied to
/// the new storage (`00b23f10`), the copies of the value made (`00b243a0`) and the later
/// elements copied after them (`00b247b0`), the old elements' strings destroyed (`0044a730`)
/// and the old block freed. With room, a temporary copy of the value is made (`00b22f80`); when
/// fewer elements follow the position than are inserted the tail moves up (`00b23f10`), copies
/// of the value are made past the old end and the range is assigned the temporary (`00b24a80`);
/// otherwise the last `count` elements are copied past the end (`00b247b0`), the others moved up
/// (`00b23b00`) and the range assigned. The temporary's string is destroyed with `0044a730`.
pub fn fn_00b21560(
    e: &mut Engine,
    this: Ptr<StlVector>,
    _unused_1: u32,
    position: u32,
    count: u32,
    value: Ptr,
) {
    let t = this.addr();
    match insert_room(e, t, 0x2C, 0x05d1_745d, count) {
        Room::Nothing => {}
        Room::TooLong => {
            e.call(LENGTH_ERROR, &args![]);
        }
        Room::Grow(capacity) => {
            let storage = fn_00b23b50(e, capacity, 0);
            let first = e.mem.u32(t + 0x0C);
            let copied = e
                .call(TEXTURE_RELOCATE, &args![first, position, storage, t + 8])
                .u32();
            e.call(
                TEXTURE_FILL,
                &args![copied, count, value, t + 8, 0u32, 0u32],
            );
            let filled = count.wrapping_mul(0x2C).wrapping_add(copied);
            let last = e.mem.u32(t + 0x10);
            e.call(TEXTURE_TAIL_COPY, &args![position, last, filled, t + 8]);
            let new_len = vector_len(e, t, 0x2C).wrapping_add(count);
            if e.mem.u32(t + 0x0C) != 0 {
                let last = e.mem.u32(t + 0x10);
                let mut element = e.mem.u32(t + 0x0C);
                while element != last {
                    e.call(STRING_TIDY, &args![element, 1u32, 0u32]);
                    if delete_flag(NO_DELETE) {
                        e.call(OPERATOR_DELETE, &args![element]);
                    }
                    element = element.wrapping_add(0x2C);
                }
                let block = e.mem.u32(t + 0x0C);
                e.call(OPERATOR_DELETE, &args![block]);
            }
            finish_growth(e, t, 0x2C, storage, capacity, new_len);
        }
        Room::Fits => {
            let after = (e.mem.u32(t + 0x10).wrapping_sub(position) as i32 / 0x2C) as u32;
            e.with_stack(0x2C, |e, temporary| {
                let temp = temporary.addr();
                fn_00b22f80(e, temporary.cast(), value.cast());
                if after < count {
                    let last = e.mem.u32(t + 0x10);
                    e.call(
                        TEXTURE_RELOCATE,
                        &args![
                            position,
                            last,
                            count.wrapping_mul(0x2C).wrapping_add(position),
                            t + 8
                        ],
                    );
                    let last = e.mem.u32(t + 0x10);
                    let surplus =
                        count.wrapping_sub((last.wrapping_sub(position) as i32 / 0x2C) as u32);
                    e.call(TEXTURE_FILL, &args![last, surplus, temp, t + 8, 0u32, 0u32]);
                    let end = count.wrapping_mul(0x2C).wrapping_add(e.mem.u32(t + 0x10));
                    e.mem.set_u32(t + 0x10, end);
                    let stop = e.mem.u32(t + 0x10).wrapping_sub(count.wrapping_mul(0x2C));
                    let mut element = position;
                    while element != stop {
                        e.call(TEXTURE_ASSIGN, &args![element, temp]);
                        element = element.wrapping_add(0x2C);
                    }
                } else {
                    let old_last = e.mem.u32(t + 0x10);
                    let tail = old_last.wrapping_sub(count.wrapping_mul(0x2C));
                    let this_last = e.mem.u32(t + 0x10);
                    let end = e
                        .call(TEXTURE_TAIL_COPY, &args![tail, old_last, this_last, t + 8])
                        .u32();
                    e.mem.set_u32(t + 0x10, end);
                    fn_00b23b00(e, position, tail, old_last);
                    let stop = count.wrapping_mul(0x2C).wrapping_add(position);
                    let mut element = position;
                    while element != stop {
                        e.call(TEXTURE_ASSIGN, &args![element, temp]);
                        element = element.wrapping_add(0x2C);
                    }
                }
                e.call(STRING_TIDY, &args![temp, 1u32, 0u32]);
            });
        }
    }
}

// Translated from 00b21e00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b21e00 - the FS:[0] exception frame (the handler that frees the new
// storage if a copy throws) is not translated.
/// `_Insert_n` of the `CVec` vector (0xC-byte elements, three floats; the engine map gives it no
/// name; `thiscall`, four arguments, the first, a proxy, is not read): inserts `count` copies of
/// the element at `value` before `position`. A count of zero does nothing; a result above the
/// maximum size (`0x15555555` elements) raises the length error (`00b0fb80`). Without room the
/// vector grows (`00b23bb0`): the elements before the position are copied to the new storage
/// (`00b17740`), the copies of the value made (`00b244a0`) and the later elements copied after
/// them, the old block freed. With room, the value is first copied to a local (three words);
/// when fewer elements follow the position than are inserted the tail moves up (`00b17740`),
/// copies of the value are made past the old end and the range is overwritten with the value;
/// otherwise the last `count` elements are copied past the end (`00b16dc0`), the others moved
/// up (`00b169c0`) and the range overwritten.
pub fn fn_00b21e00(
    e: &mut Engine,
    this: Ptr<StlVector>,
    _unused_1: u32,
    position: u32,
    count: u32,
    value: Ptr,
) {
    let t = this.addr();
    match insert_room(e, t, 0xC, 0x1555_5555, count) {
        Room::Nothing => {}
        Room::TooLong => {
            e.call(LENGTH_ERROR, &args![]);
        }
        Room::Grow(capacity) => {
            let storage = fn_00b23bb0(e, capacity, 0);
            let first = e.mem.u32(t + 0x0C);
            let copied = e
                .call(CVEC_COPY_RANGE, &args![first, position, storage, t + 8])
                .u32();
            e.call(CVEC_FILL, &args![copied, count, value, t + 8, 0u32, 0u32]);
            let filled = count.wrapping_mul(0xC).wrapping_add(copied);
            let last = e.mem.u32(t + 0x10);
            e.call(CVEC_COPY_RANGE, &args![position, last, filled, t + 8]);
            let new_len = vector_len(e, t, 0xC).wrapping_add(count);
            if e.mem.u32(t + 0x0C) != 0 {
                // The loop that destroys the old elements has nothing to run.
                let mut element = e.mem.u32(t + 0x0C);
                let last = e.mem.u32(t + 0x10);
                while element != last {
                    element = element.wrapping_add(0xC);
                }
                let block = e.mem.u32(t + 0x0C);
                e.call(OPERATOR_DELETE, &args![block]);
            }
            finish_growth(e, t, 0xC, storage, capacity, new_len);
        }
        Room::Fits => {
            let after = (e.mem.u32(t + 0x10).wrapping_sub(position) as i32 / 0xC) as u32;
            let temporary = [
                e.mem.u32(value.addr()),
                e.mem.u32(value.addr() + 4),
                e.mem.u32(value.addr() + 8),
            ];
            if after < count {
                let last = e.mem.u32(t + 0x10);
                e.call(
                    CVEC_COPY_RANGE,
                    &args![
                        position,
                        last,
                        count.wrapping_mul(0xC).wrapping_add(position),
                        t + 8
                    ],
                );
                e.with_stack(0xC, |e, copy| {
                    for (i, word) in temporary.iter().enumerate() {
                        e.mem.set_u32(copy.addr() + 4 * i as u32, *word);
                    }
                    let last = e.mem.u32(t + 0x10);
                    let surplus =
                        count.wrapping_sub((last.wrapping_sub(position) as i32 / 0xC) as u32);
                    e.call(CVEC_FILL, &args![last, surplus, copy, t + 8, 0u32, 0u32]);
                });
                let end = count.wrapping_mul(0xC).wrapping_add(e.mem.u32(t + 0x10));
                e.mem.set_u32(t + 0x10, end);
                let stop = e.mem.u32(t + 0x10).wrapping_sub(count.wrapping_mul(0xC));
                let mut element = position;
                while element != stop {
                    for (i, word) in temporary.iter().enumerate() {
                        e.mem.set_u32(element + 4 * i as u32, *word);
                    }
                    element = element.wrapping_add(0xC);
                }
            } else {
                let old_last = e.mem.u32(t + 0x10);
                let tail = old_last.wrapping_sub(count.wrapping_mul(0xC));
                let this_last = e.mem.u32(t + 0x10);
                let end = e
                    .call(CVEC_TAIL_COPY, &args![tail, old_last, this_last, t + 8])
                    .u32();
                e.mem.set_u32(t + 0x10, end);
                e.call(CVEC_MOVE_UP, &args![position, tail, old_last]);
                let stop = count.wrapping_mul(0xC).wrapping_add(position);
                let mut element = position;
                while element != stop {
                    for (i, word) in temporary.iter().enumerate() {
                        e.mem.set_u32(element + 4 * i as u32, *word);
                    }
                    element = element.wrapping_add(0xC);
                }
            }
        }
    }
}

// Translated from 00b22650 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b22650 - the FS:[0] exception frame is not translated.
/// Destroys the float vectors in `first..last` (0x18-byte elements; the engine map gives it no
/// name; `thiscall`, two arguments): for each, the float vector's tidy (`0065f240`) and the
/// release of its proxy block (`operator delete` of the element's first word).
pub fn fn_00b22650(e: &mut Engine, _this: u32, first: u32, last: u32) {
    let mut element = first;
    while element != last {
        e.call(FLOAT_VECTOR_TIDY, &args![element]);
        let proxy = e.mem.u32(element);
        e.call(OPERATOR_DELETE, &args![proxy]);
        if delete_flag(NO_DELETE) {
            e.call(OPERATOR_DELETE, &args![element]);
        }
        element = element.wrapping_add(0x18);
    }
}

// Translated from 00b22700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b22700 - the FS:[0] exception frame (the handlers that unwind the
// new storage and the temporary float vector if a copy throws) is not translated.
/// `_Insert_n` of the vector of float vectors (0x18-byte elements; the engine map gives it no
/// name; `thiscall`, four arguments, the first, a proxy, is not read): inserts `count` copies of
/// the float vector at `value` before `position`. A count of zero does nothing; a result above
/// the maximum size (`0xAAAAAAA` elements) raises the length error (`00b0fb80`). Without room
/// the vector grows (`00b23aa0`): the elements before the position are copied to the new storage
/// (`00b25e20`), the copies of the value made (`00b24580`) and the later elements copied after
/// them, the old elements destroyed (`00b22650`) and the old block freed. With room, a
/// temporary copy of the value is made (`00b22ff0`); when fewer elements follow the position
/// than are inserted the tail moves up (`00b25e20`), copies of the value are made past the old
/// end and the range is assigned the temporary (`00b1d5d0`); otherwise the last `count` elements
/// are copied past the end (`00b25e20`), the others moved up (`00b23c10`) and the range
/// assigned. The temporary is tidied (`0065f240`) and released (`0065d6e0`, or `operator
/// delete` of its proxy in the second case).
pub fn fn_00b22700(
    e: &mut Engine,
    this: Ptr<StlVector>,
    _unused_1: u32,
    position: u32,
    count: u32,
    value: Ptr,
) {
    let t = this.addr();
    match insert_room(e, t, 0x18, 0x0aaa_aaaa, count) {
        Room::Nothing => {}
        Room::TooLong => {
            e.call(LENGTH_ERROR, &args![]);
        }
        Room::Grow(capacity) => {
            let storage = fn_00b23aa0(e, capacity, 0);
            let first = e.mem.u32(t + 0x0C);
            let copied = e
                .call(
                    FLOAT_VECTOR_COPY_RANGE,
                    &args![first, position, storage, t + 8, 0u32, 0u32],
                )
                .u32();
            e.call(
                FLOAT_VECTOR_FILL,
                &args![copied, count, value, t + 8, 0u32, 0u32],
            );
            let filled = count.wrapping_mul(0x18).wrapping_add(copied);
            let last = e.mem.u32(t + 0x10);
            e.call(
                FLOAT_VECTOR_COPY_RANGE,
                &args![position, last, filled, t + 8, 0u32, 0u32],
            );
            let new_len = vector_len(e, t, 0x18).wrapping_add(count);
            if e.mem.u32(t + 0x0C) != 0 {
                let first = e.mem.u32(t + 0x0C);
                let last = e.mem.u32(t + 0x10);
                fn_00b22650(e, 0, first, last);
                let block = e.mem.u32(t + 0x0C);
                e.call(OPERATOR_DELETE, &args![block]);
            }
            finish_growth(e, t, 0x18, storage, capacity, new_len);
        }
        Room::Fits => {
            let after = (e.mem.u32(t + 0x10).wrapping_sub(position) as i32 / 0x18) as u32;
            e.with_stack(0x18, |e, temporary| {
                let temp = temporary.addr();
                fn_00b22ff0(e, temporary.cast(), value.cast());
                if after < count {
                    let last = e.mem.u32(t + 0x10);
                    e.call(
                        FLOAT_VECTOR_COPY_RANGE,
                        &args![
                            position,
                            last,
                            count.wrapping_mul(0x18).wrapping_add(position),
                            t + 8,
                            0u32,
                            0u32
                        ],
                    );
                    let last = e.mem.u32(t + 0x10);
                    let surplus =
                        count.wrapping_sub((last.wrapping_sub(position) as i32 / 0x18) as u32);
                    e.call(
                        FLOAT_VECTOR_FILL,
                        &args![last, surplus, temp, t + 8, 0u32, 0u32],
                    );
                    let end = count.wrapping_mul(0x18).wrapping_add(e.mem.u32(t + 0x10));
                    e.mem.set_u32(t + 0x10, end);
                    let stop = e.mem.u32(t + 0x10).wrapping_sub(count.wrapping_mul(0x18));
                    let mut element = position;
                    while element != stop {
                        e.call(FLOAT_VECTOR_ASSIGN, &args![element, temp]);
                        element = element.wrapping_add(0x18);
                    }
                    e.call(FLOAT_VECTOR_TIDY, &args![temp]);
                    e.call(FLOAT_VECTOR_RELEASE, &args![temp]);
                } else {
                    let old_last = e.mem.u32(t + 0x10);
                    let tail = old_last.wrapping_sub(count.wrapping_mul(0x18));
                    let this_last = e.mem.u32(t + 0x10);
                    let end = e
                        .call(
                            FLOAT_VECTOR_COPY_RANGE,
                            &args![tail, old_last, this_last, t + 8, 0u32, 0u32],
                        )
                        .u32();
                    e.mem.set_u32(t + 0x10, end);
                    fn_00b23c10(e, position, tail, old_last);
                    let stop = count.wrapping_mul(0x18).wrapping_add(position);
                    let mut element = position;
                    while element != stop {
                        e.call(FLOAT_VECTOR_ASSIGN, &args![element, temp]);
                        element = element.wrapping_add(0x18);
                    }
                    e.call(FLOAT_VECTOR_TIDY, &args![temp]);
                    let proxy = e.mem.u32(temp);
                    e.call(OPERATOR_DELETE, &args![proxy]);
                }
            });
        }
    }
}

// Translated from 00b22f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SFrondTexture::SFrondTexture(const SFrondTexture &)` (the engine map gives it no name;
/// `thiscall`, one argument): makes the file name string empty (`0044a730`, `(0, 0)`), assigns
/// the other's string over it (`0044a580`, offset 0, the `npos` count at `[010173f0]`) and
/// copies the four floats at `+0x1C` to `+0x28` through the x87 stack. Returns `this`.
pub fn fn_00b22f80(
    e: &mut Engine,
    this: Ptr<SFrondTexture>,
    other: Ptr<SFrondTexture>,
) -> Ptr<SFrondTexture> {
    let (t, o) = (this.addr(), other.addr());
    e.call(STRING_TIDY, &args![t, 0u32, 0u32]);
    let npos = e.mem.u32(STRING_NPOS);
    e.call(STRING_ASSIGN, &args![t, o, 0u32, npos]);
    for offset in [0x1Cu32, 0x20, 0x24, 0x28] {
        copy_float(e, t + offset, o + offset);
    }
    this
}

// Translated from 00b22ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b22ff0 - the FS:[0] exception frame (the handler that destroys the
// half-built vector) is not translated.
/// The copy constructor of a `std::vector<float>` (the engine map gives it no name; `thiscall`,
/// one argument): builds the proxy (`0065dc90`, with a dummy stack word), reserves room for the
/// other vector's element count (`0044a240`, the byte difference shifted right by 2) and, when
/// that made room, copies the other's elements to the new storage (`00b24960`) and sets the end
/// pointer after them. Returns `this`.
pub fn fn_00b22ff0(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) -> Ptr<StlVector> {
    let (t, o) = (this.addr(), other.addr());
    e.call(FLOAT_VECTOR_CONSTRUCT_PROXY, &args![this, 0u32]);
    let count = (e.mem.u32(o + 0x10).wrapping_sub(e.mem.u32(o + 0x0C)) as i32 >> 2) as u32;
    if e.call(FLOAT_VECTOR_RESERVE, &args![this, count]).bool() {
        let last = e.mem.u32(o + 0x10);
        let last_proxy = checked_iterator(e, o, last);
        let first = e.mem.u32(o + 0x0C);
        let first_proxy = checked_iterator(e, o, first);
        let storage = e.mem.u32(t + 0x0C);
        let end = e
            .call(
                FLOAT_COPY_WORDS,
                &args![first_proxy, first, last_proxy, last, storage, t + 8],
            )
            .u32();
        e.mem.set_u32(t + 0x10, end);
    }
    this
}

// Translated from 00b231f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b231f0 - the FS:[0] exception frame (the handler that destroys the
// half-built vector) is not translated.
/// The copy constructor of the guide vector (a `std::vector<SFrondGuide>`; the engine map gives
/// it no name; `thiscall`, one argument): builds the proxy (`00b20ba0`), reserves room for the
/// other vector's element count (`00b1eb00`) and, when that made room, copies the other's
/// guides to the new storage (`00b25fa0`) and sets the end pointer after them. Returns `this`.
pub fn fn_00b231f0(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) -> Ptr<StlVector> {
    let (t, o) = (this.addr(), other.addr());
    fn_00b20ba0(e, this, 0);
    let count = (e.mem.u32(o + 0x10).wrapping_sub(e.mem.u32(o + 0x0C)) as i32 / 0x38) as u32;
    if e.call(VERTEX_VECTOR_BUY, &args![this, count]).bool() {
        let last = e.mem.u32(o + 0x10);
        checked_iterator(e, o, last);
        let first = e.mem.u32(o + 0x0C);
        checked_iterator(e, o, first);
        let storage = e.mem.u32(t + 0x0C);
        let end = e
            .call(
                GUIDE_COPY_RANGE,
                &args![first, last, storage, t + 8, 0u32, 0u32],
            )
            .u32();
        e.mem.set_u32(t + 0x10, end);
    }
    this
}

// Translated from 00b23460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked iterator advance of the guide and vertex vectors (the engine map gives it no
/// name; `thiscall`, one argument; the iterator at `this` is a proxy and a position, 0x38-byte
/// elements): the iterator needs a proxy, and `position + count * 0x38` has to lie between the
/// owner vector's first and last pointers (else the invalid-parameter path, `00ec7c56`); then
/// the position moves. Returns `this`.
pub fn fn_00b23460(e: &mut Engine, this: Ptr, count: i32) -> Ptr {
    advance_iterator(e, this.addr(), count, 0x38);
    this
}

// Translated from 00b23520 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_00b23460`] for the texture vector (0x2C-byte elements).
pub fn fn_00b23520(e: &mut Engine, this: Ptr, count: i32) -> Ptr {
    advance_iterator(e, this.addr(), count, 0x2C);
    this
}

// Translated from 00b235e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_00b23460`] for the `CVec` vector (0xC-byte elements).
pub fn fn_00b235e0(e: &mut Engine, this: Ptr, count: i32) -> Ptr {
    advance_iterator(e, this.addr(), count, 0xC);
    this
}

// Translated from 00b236a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 0x18-byte elements `first..last` (six words each) to `destination`, element by
/// element in order (`cdecl`, three arguments). Returns the destination plus the element count
/// times 0x18.
pub fn fn_00b236a0(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    let mut target = destination;
    let mut source = first;
    while source != last {
        for word in 0..6 {
            let value = e.mem.u32(source + 4 * word);
            e.mem.set_u32(target + 4 * word, value);
        }
        target = target.wrapping_add(0x18);
        source = source.wrapping_add(0x18);
    }
    ((last.wrapping_sub(first) as i32 / 0x18) as u32)
        .wrapping_mul(0x18)
        .wrapping_add(destination)
}

// Translated from 00b23760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Assigns the guides `first..last` to the guides from `destination` on (`00b249d0` on each pair,
/// in order; `cdecl`, three arguments). Returns the destination plus the element count times
/// 0x38.
pub fn fn_00b23760(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    let mut target = destination;
    let mut source = first;
    while source != last {
        e.call(GUIDE_ASSIGN, &args![target, source]);
        target = target.wrapping_add(0x38);
        source = source.wrapping_add(0x38);
    }
    ((last.wrapping_sub(first) as i32 / 0x38) as u32)
        .wrapping_mul(0x38)
        .wrapping_add(destination)
}

// Translated from 00b23830 (decompiled, FalloutNV.exe 1.4.0.525)
/// [`fn_00b23760`] for the textures (0x2C-byte elements, `00b24a80` on each pair).
pub fn fn_00b23830(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    let mut target = destination;
    let mut source = first;
    while source != last {
        e.call(TEXTURE_ASSIGN, &args![target, source]);
        target = target.wrapping_add(0x2C);
        source = source.wrapping_add(0x2C);
    }
    ((last.wrapping_sub(first) as i32 / 0x2C) as u32)
        .wrapping_mul(0x2C)
        .wrapping_add(destination)
}

// Translated from 00b238e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the guides `first..last` backwards so that they end at `destination_end` (`cdecl`, three
/// arguments): calls the backward assignment loop `00b240b0` with three flag bytes the exe
/// leaves uninitialised (zeros here) and returns what it returns.
pub fn fn_00b238e0(e: &mut Engine, first: u32, last: u32, destination_end: u32) -> u32 {
    e.call(
        GUIDE_RANGE_BACKWARD,
        &args![first, last, destination_end, 0u32, 0u32, 0u32],
    )
    .u32()
}

// Translated from 00b23930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the vertices `first..last` backwards so that they end at `destination_end` (`cdecl`,
/// three arguments): calls the backward word-copy loop `00b24710` with three flag bytes the exe
/// leaves uninitialised (zeros here) and returns what it returns.
pub fn fn_00b23930(e: &mut Engine, first: u32, last: u32, destination_end: u32) -> u32 {
    e.call(
        VERTEX_RANGE_BACKWARD,
        &args![first, last, destination_end, 0u32, 0u32, 0u32],
    )
    .u32()
}

// Translated from 00b23980 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator of the guide and vertex vectors (`_Allocate(count, hint)`, the hint is not
/// read; `cdecl`, two arguments): `operator new(count * 0x38)`; a count of zero allocates zero
/// bytes and a count whose byte size overflows throws `std::bad_alloc`.
pub fn fn_00b23980(e: &mut Engine, count: u32, _unused_1: u32) -> u32 {
    allocate_elements(e, count, 0x38)
}

// Translated from 00b239e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the guide vectors `first..last` (0x18-byte elements) backwards so that they end at
/// `destination_end`: from the last element down, each destination element exchanges its
/// contents with the source element (`00b267a0`, the vector swap). `cdecl`, three arguments;
/// two flag bytes the exe leaves uninitialised are not used. Returns the destination start
/// (`destination_end` minus the element count times 0x18).
pub fn fn_00b239e0(e: &mut Engine, first: u32, last: u32, destination_end: u32) -> u32 {
    let start = destination_end
        .wrapping_sub(((last.wrapping_sub(first) as i32 / 0x18) as u32).wrapping_mul(0x18));
    let mut target = destination_end;
    let mut source = last;
    while first != source {
        source = source.wrapping_sub(0x18);
        target = target.wrapping_sub(0x18);
        e.call(GUIDE_VECTOR_MOVE_ELEMENT, &args![target, source]);
    }
    start
}

// Translated from 00b23aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator of the vectors of 0x18-byte elements (guide vectors, float vectors; `cdecl`,
/// two arguments, the hint is not read): `operator new(count * 0x18)`; a count of zero allocates
/// zero bytes and a count whose byte size overflows throws `std::bad_alloc`.
pub fn fn_00b23aa0(e: &mut Engine, count: u32, _unused_1: u32) -> u32 {
    allocate_elements(e, count, 0x18)
}

// Translated from 00b23b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the textures `first..last` backwards so that they end at `destination_end` (`cdecl`,
/// three arguments): calls the backward assignment loop `00b24830` with three flag bytes the exe
/// leaves uninitialised (zeros here) and returns what it returns.
pub fn fn_00b23b00(e: &mut Engine, first: u32, last: u32, destination_end: u32) -> u32 {
    e.call(
        TEXTURE_RANGE_BACKWARD,
        &args![first, last, destination_end, 0u32, 0u32, 0u32],
    )
    .u32()
}

// Translated from 00b23b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator of the texture vector (`cdecl`, two arguments, the hint is not read):
/// `operator new(count * 0x2C)`; a count of zero allocates zero bytes and a count whose byte
/// size overflows throws `std::bad_alloc`.
pub fn fn_00b23b50(e: &mut Engine, count: u32, _unused_1: u32) -> u32 {
    allocate_elements(e, count, 0x2C)
}

// Translated from 00b23bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator of the `CVec` vector (`cdecl`, two arguments, the hint is not read):
/// `operator new(count * 0xC)`; a count of zero allocates zero bytes and a count whose byte size
/// overflows throws `std::bad_alloc`.
pub fn fn_00b23bb0(e: &mut Engine, count: u32, _unused_1: u32) -> u32 {
    allocate_elements(e, count, 0xC)
}

// Translated from 00b23c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves the float vectors `first..last` (0x18-byte elements) backwards so that they end at
/// `destination_end`: from the last element down, each destination element exchanges its
/// contents with the source element (`00b26570`, the vector swap). `cdecl`, three arguments.
/// Returns the destination start (`destination_end` minus the element count times 0x18).
pub fn fn_00b23c10(e: &mut Engine, first: u32, last: u32, destination_end: u32) -> u32 {
    let mut target = destination_end;
    let mut source = last;
    while first != source {
        source = source.wrapping_sub(0x18);
        target = target.wrapping_sub(0x18);
        e.call(FLOAT_VECTOR_MOVE_ELEMENT, &args![target, source]);
    }
    destination_end
        .wrapping_sub(((last.wrapping_sub(first) as i32 / 0x18) as u32).wrapping_mul(0x18))
}

// Translated from 00b23cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The introspective sort of the guide vector (`std::_Sort(first, last, ideal, comparison)`
/// for 0x38-byte guides; the engine map gives it no name; `cdecl`, four arguments, the last a
/// byte-sized empty comparison object). While more than 32 elements remain and the allowed
/// depth `ideal` is positive: partitions the range (`00b25680`), shrinks the depth to
/// `ideal / 2 + ideal / 4` and sorts the smaller part by recursion, going on with the other
/// part. Then, if more than 32 elements are left (the depth ran out), makes a heap of the range
/// (`00b26260`) and pops it element by element (`00b26ca0`); otherwise, with more than one
/// element, runs the insertion sort (`00b262d0`).
pub fn fn_00b23cd0(e: &mut Engine, first: u32, last: u32, ideal: i32, comparison: u8) {
    let mut first = first;
    let mut last = last;
    let mut ideal = ideal;
    let mut count = (last.wrapping_sub(first) as i32) / 0x38;
    while count > 0x20 && ideal > 0 {
        let (middle_first, middle_last) = e.with_stack(8, |e, pair| {
            e.call(
                GUIDE_PARTITION,
                &args![pair, first, last, u32::from(comparison)],
            );
            (e.mem.u32(pair.addr()), e.mem.u32(pair.addr() + 4))
        });
        ideal /= 2;
        ideal = ideal / 2 + ideal;
        let below = (middle_first.wrapping_sub(first) as i32) / 0x38;
        let above = (last.wrapping_sub(middle_last) as i32) / 0x38;
        if below < above {
            fn_00b23cd0(e, first, middle_first, ideal, comparison);
            first = middle_last;
        } else {
            fn_00b23cd0(e, middle_last, last, ideal, comparison);
            last = middle_first;
        }
        count = (last.wrapping_sub(first) as i32) / 0x38;
    }
    if count > 0x20 {
        if (last.wrapping_sub(first) as i32) / 0x38 > 1 {
            e.call(
                GUIDE_MAKE_HEAP,
                &args![first, last, u32::from(comparison), 0u32, 0u32],
            );
        }
        let mut end = last;
        while (end.wrapping_sub(first) as i32) / 0x38 > 1 {
            e.call(GUIDE_POP_HEAP, &args![first, end, u32::from(comparison)]);
            end = end.wrapping_sub(0x38);
        }
    } else if count > 1 {
        e.call(
            GUIDE_INSERTION_SORT,
            &args![first, last, u32::from(comparison), 0u32],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00b1ff50,
            fn_00b1ff50(Ptr<SFrondGuide>, Ptr<SFrondGuide>) -> Ptr<SFrondGuide>
        ),
        entry!(
            0x00b1ffd0,
            fn_00b1ffd0(Ptr<StlVector>, Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x00b200f0,
            fn_00b200f0(Ptr<StlVector>, Ptr<StlVector>) -> Ptr<StlVector>
        ),
        entry!(0x00b20360, fn_00b20360(Ptr<StlVector>, u32, u32, u32, Ptr)),
        entry!(
            0x00b20ba0,
            fn_00b20ba0(Ptr<StlVector>, u32) -> Ptr<StlVector>
        ),
        entry!(0x00b20c30, fn_00b20c30(u32, u32, u32)),
        entry!(0x00b20ce0, fn_00b20ce0(Ptr<StlVector>, u32, u32, u32, Ptr)),
        entry!(0x00b21560, fn_00b21560(Ptr<StlVector>, u32, u32, u32, Ptr)),
        entry!(0x00b21e00, fn_00b21e00(Ptr<StlVector>, u32, u32, u32, Ptr)),
        entry!(0x00b22650, fn_00b22650(u32, u32, u32)),
        entry!(0x00b22700, fn_00b22700(Ptr<StlVector>, u32, u32, u32, Ptr)),
        entry!(
            0x00b22f80,
            fn_00b22f80(Ptr<SFrondTexture>, Ptr<SFrondTexture>) -> Ptr<SFrondTexture>
        ),
        entry!(
            0x00b22ff0,
            fn_00b22ff0(Ptr<StlVector>, Ptr<StlVector>) -> Ptr<StlVector>
        ),
        entry!(
            0x00b231f0,
            fn_00b231f0(Ptr<StlVector>, Ptr<StlVector>) -> Ptr<StlVector>
        ),
        entry!(0x00b23460, fn_00b23460(Ptr, i32) -> Ptr),
        entry!(0x00b23520, fn_00b23520(Ptr, i32) -> Ptr),
        entry!(0x00b235e0, fn_00b235e0(Ptr, i32) -> Ptr),
        entry!(0x00b236a0, fn_00b236a0(u32, u32, u32) -> u32),
        entry!(0x00b23760, fn_00b23760(u32, u32, u32) -> u32),
        entry!(0x00b23830, fn_00b23830(u32, u32, u32) -> u32),
        entry!(0x00b238e0, fn_00b238e0(u32, u32, u32) -> u32),
        entry!(0x00b23930, fn_00b23930(u32, u32, u32) -> u32),
        entry!(0x00b23980, fn_00b23980(u32, u32) -> u32),
        entry!(0x00b239e0, fn_00b239e0(u32, u32, u32) -> u32),
        entry!(0x00b23aa0, fn_00b23aa0(u32, u32) -> u32),
        entry!(0x00b23b00, fn_00b23b00(u32, u32, u32) -> u32),
        entry!(0x00b23b50, fn_00b23b50(u32, u32) -> u32),
        entry!(0x00b23bb0, fn_00b23bb0(u32, u32) -> u32),
        entry!(0x00b23c10, fn_00b23c10(u32, u32, u32) -> u32),
        entry!(0x00b23cd0, fn_00b23cd0(u32, u32, i32, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    /// The proxy word the test vectors carry.
    const PROXY: u32 = 0x0090_0000;

    /// Callees outside this file; every test runs them as do-nothing doubles unless it
    /// supplies its own.
    const EXTERNAL: &[u32] = &[
        ALLOCATE_PROXY,
        VERTEX_VECTOR_BUY,
        LENGTH_ERROR,
        ITERATORS_DIFFER,
        TRIVIAL_DESTRUCTOR,
        STRING_TIDY,
        STRING_ASSIGN,
        EXCEPTION_FROM_MESSAGE,
        CXX_THROW_EXCEPTION,
        VERTEX_COPY_RANGE,
        VERTEX_RELOCATE,
        VERTEX_FILL,
        VERTEX_TAIL_COPY,
        GUIDE_COPY_RANGE,
        GUIDE_VECTOR_COPY_RANGE,
        GUIDE_VECTOR_FILL,
        GUIDE_VECTOR_ASSIGN,
        GUIDE_VECTOR_TIDY,
        PROXY_RELEASE,
        TEXTURE_RELOCATE,
        TEXTURE_TAIL_COPY,
        TEXTURE_FILL,
        TEXTURE_ASSIGN,
        CVEC_COPY_RANGE,
        CVEC_TAIL_COPY,
        CVEC_MOVE_UP,
        CVEC_FILL,
        FLOAT_VECTOR_COPY_RANGE,
        FLOAT_VECTOR_FILL,
        FLOAT_VECTOR_TIDY,
        FLOAT_VECTOR_RELEASE,
        FLOAT_VECTOR_ASSIGN,
        FLOAT_VECTOR_CONSTRUCT_PROXY,
        FLOAT_VECTOR_RESERVE,
        FLOAT_COPY_WORDS,
        GUIDE_ASSIGN,
        GUIDE_RANGE_BACKWARD,
        VERTEX_RANGE_BACKWARD,
        TEXTURE_RANGE_BACKWARD,
        GUIDE_VECTOR_MOVE_ELEMENT,
        FLOAT_VECTOR_MOVE_ELEMENT,
        GUIDE_PARTITION,
        GUIDE_MAKE_HEAP,
        GUIDE_POP_HEAP,
        GUIDE_INSERTION_SORT,
        INVALID_PARAMETER,
    ];

    /// An engine with the exe's data range mapped as zeroes and every outside callee of the file
    /// silenced.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        for addr in EXTERNAL {
            e.register(*addr, |_, _| Ret::default());
        }
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

    /// A double for a range copy: answers the destination plus the range's byte length.
    fn range_copy(e: &mut Engine, addr: u32) {
        e.register(addr, |_, a| Ret {
            eax: a[2].wrapping_add(a[1].wrapping_sub(a[0])),
            ..Ret::default()
        });
    }

    #[test]
    fn the_guide_copy_constructor_copies_the_vector_and_the_fields() {
        let mut e = engine();
        let proxies = Rc::new(RefCell::new(0u32));
        let count = proxies.clone();
        e.register_double(ALLOCATE_PROXY, move |e, _| {
            *count.borrow_mut() += 1;
            Ret {
                eax: e.mem.alloc(8),
                ..Ret::default()
            }
        });
        let source = e.new_object::<SFrondGuide>();
        let target = e.new_object::<SFrondGuide>();
        let s = source.addr();
        e.mem.set_u32(s + 0x18, 0x7f80_0001);
        e.mem.set_f32(s + 0x1C, 2.5);
        e.mem.set_u8(s + 0x20, 7);
        e.mem.set_f32(s + 0x24, 0.25);
        e.mem.set_u32(s + 0x28, 0x7f80_0002);
        e.mem.set_f32(s + 0x2C, -1.0);
        e.mem.set_u32(s + 0x30, 0x1234);
        e.mem.set_u32(s + 0x34, 0x5678);
        let result = e
            .call(0x00b1_ff50, &args![target, source])
            .ptr::<SFrondGuide>();
        assert_eq!(result, target);
        let t = target.addr();
        // The vertex vector got its proxy; a float copy through the x87 stack quiets a NaN.
        assert_eq!(*proxies.borrow(), 1);
        assert_eq!(e.mem.u32(t + 0x18), 0x7fc0_0001);
        assert_eq!(e.mem.f32(t + 0x1C), 2.5);
        assert_eq!(e.mem.u8(t + 0x20), 7);
        assert_eq!(e.mem.f32(t + 0x24), 0.25);
        assert_eq!(e.mem.u32(t + 0x28), 0x7fc0_0002);
        assert_eq!(e.mem.f32(t + 0x2C), -1.0);
        assert_eq!(e.mem.u32(t + 0x30), 0x1234);
        assert_eq!(e.mem.u32(t + 0x34), 0x5678);
    }

    #[test]
    fn erasing_a_range_copies_the_rest_down_and_destroys_the_surplus() {
        let mut e = engine();
        let this = e.mem.alloc(0x18);
        let data = vector(&mut e, this, 0x18, 3, 4);
        for i in 0..18 {
            e.mem.set_u32(data + 4 * i, 0x100 + i);
        }
        let out = e.mem.alloc(8);
        reply(&mut e, ITERATORS_DIFFER, 1);
        e.call_log = Some(vec![]);
        let first = data + 0x18;
        let last = data + 0x30;
        let result = e
            .call(0x00b1_ffd0, &args![this, out, 0u32, first, 0u32, last])
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, out);
        // The third element moved over the second; the end moved up by one element.
        assert_eq!(e.mem.u32(first), 0x100 + 12);
        assert_eq!(e.mem.u32(this + 0x10), data + 0x30);
        assert_eq!(calls(&log, TRIVIAL_DESTRUCTOR), vec![vec![data + 0x30]]);
        // The iterator written back is the first one.
        assert_eq!(e.mem.u32(out), PROXY);
        assert_eq!(e.mem.u32(out + 4), first);
        // Equal iterators erase nothing.
        reply(&mut e, ITERATORS_DIFFER, 0);
        e.mem.set_u32(this + 0x10, data + 0x30);
        e.call(0x00b1_ffd0, &args![this, out, 0u32, first, 0u32, first]);
        assert_eq!(e.mem.u32(this + 0x10), data + 0x30);
    }

    /// A copy constructor over a `size`-byte element vector: `buy` and `copy` are the callees
    /// `_Buy` and the range copy.
    fn check_copy_constructor(address: u32, size: u32, copy: u32) {
        let mut e = engine();
        let proxy = e.mem.alloc(8);
        reply(&mut e, ALLOCATE_PROXY, proxy);
        let other = e.mem.alloc(0x18);
        let source = vector(&mut e, other, size, 2, 2);
        let this = e.mem.alloc(0x18);
        // `_Buy` made room: the new block is at `this + 0x0C`.
        let storage = e.mem.alloc(2 * size);
        e.register_double(VERTEX_VECTOR_BUY, move |e, a| {
            e.mem.set_u32(a[0] + 0x0C, storage);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        range_copy(&mut e, copy);
        e.call_log = Some(vec![]);
        let result = e.call(address, &args![this, other]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), proxy);
        assert_eq!(e.mem.u32(proxy), this);
        assert_eq!(calls(&log, VERTEX_VECTOR_BUY), vec![vec![this, 2]]);
        assert_eq!(
            calls(&log, copy),
            vec![vec![source, source + 2 * size, storage, this + 8, 0, 0]]
        );
        assert_eq!(e.mem.u32(this + 0x10), storage + 2 * size);
        // Without room nothing is copied.
        let empty = e.mem.alloc(0x18);
        reply(&mut e, VERTEX_VECTOR_BUY, 0);
        e.call_log = Some(vec![]);
        e.call(address, &args![empty, other]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, copy).is_empty());
        assert_eq!(e.mem.u32(empty + 0x10), 0);
    }

    #[test]
    fn the_vertex_vector_copy_constructor_copies_the_elements() {
        check_copy_constructor(0x00b2_00f0, 0x38, VERTEX_COPY_RANGE);
    }

    #[test]
    fn the_guide_vector_copy_constructor_copies_the_guides() {
        check_copy_constructor(0x00b2_31f0, 0x38, GUIDE_COPY_RANGE);
    }

    /// The description of an `_Insert_n` instance for `check_insert`.
    struct Insert {
        run: fn(&mut Engine, Ptr<StlVector>, u32, u32, u32, Ptr),
        size: u32,
        limit: u32,
        head: u32,
        fill: u32,
        tail: u32,
        in_place_tail: u32,
        move_up: u32,
        assign: Option<u32>,
    }

    /// The vector `len` elements long with room for `capacity`, and a value block `size` bytes
    /// long holding a recognisable pattern.
    fn insert_setup(e: &mut Engine, size: u32, len: u32, capacity: u32) -> (u32, u32, u32) {
        let this = e.mem.alloc(0x18);
        let data = vector(e, this, size, len, capacity);
        let value = e.mem.alloc(size);
        for word in 0..size / 4 {
            e.mem.set_u32(value + 4 * word, 0xA0 + word);
        }
        (this, data, value)
    }

    /// The four decisions of an `_Insert_n` instance, checked on their effects: nothing for a
    /// zero count, the length error above the maximum size, growth (head copy, fill, tail copy;
    /// the vector's pointers move to the new block and the old block is freed), and the two
    /// in-place cases (the value is assigned over the range).
    fn check_insert(spec: &Insert) {
        let size = spec.size;
        let mut e = engine();
        // A count of zero does nothing.
        let (this, data, value) = insert_setup(&mut e, size, 2, 2);
        e.call_log = Some(vec![]);
        (spec.run)(&mut e, Ptr::new(this), 0, data + size, 0, Ptr::new(value));
        assert!(e.call_log.take().unwrap().is_empty());
        // Too many elements raise the length error and change nothing.
        e.call_log = Some(vec![]);
        (spec.run)(
            &mut e,
            Ptr::new(this),
            0,
            data + size,
            spec.limit,
            Ptr::new(value),
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, LENGTH_ERROR).len(), 1);
        assert_eq!(e.mem.u32(this + 0x0C), data);
        // Growth: the head copy, the fill and the tail copy, then the new block.
        for addr in [spec.head, spec.tail, spec.in_place_tail] {
            range_copy(&mut e, addr);
        }
        e.call_log = Some(vec![]);
        (spec.run)(&mut e, Ptr::new(this), 0, data + size, 3, Ptr::new(value));
        let log = e.call_log.take().unwrap();
        let storage = e.mem.u32(this + 0x0C);
        assert_ne!(storage, data);
        // Capacity 2 grows by half to 3, below the 5 needed, so it becomes 5.
        assert_eq!(e.mem.u32(this + 0x10), storage + 5 * size);
        assert_eq!(e.mem.u32(this + 0x14), storage + 5 * size);
        let head_calls = calls(&log, spec.head);
        assert_eq!(head_calls[0][..3], [data, data + size, storage]);
        let copied = storage + size;
        assert_eq!(calls(&log, spec.fill)[0][..3], [copied, 3, value]);
        let tail_calls = calls(&log, spec.tail);
        let tail_call = if spec.tail == spec.head {
            &tail_calls[1]
        } else {
            &tail_calls[0]
        };
        assert_eq!(
            tail_call[..3],
            [data + size, data + 2 * size, copied + 3 * size]
        );
        assert_eq!(e.mem.block_size(data), None);
        // With room, fewer elements after the position than inserted: the tail moves up, copies
        // fill past the old end and the range is overwritten with the value.
        let (this, data, value) = insert_setup(&mut e, size, 2, 6);
        e.call_log = Some(vec![]);
        (spec.run)(&mut e, Ptr::new(this), 0, data + size, 3, Ptr::new(value));
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x10), data + 5 * size);
        assert_eq!(calls(&log, spec.fill).len(), 1);
        match spec.assign {
            Some(assign) => assert_eq!(calls(&log, assign).len(), 1),
            None => assert_eq!(e.mem.u32(data + size), e.mem.u32(value)),
        }
        // With room and enough elements after the position: the last ones are copied past the
        // end, the others moved up and the range overwritten.
        let (this, data, value) = insert_setup(&mut e, size, 4, 6);
        e.call_log = Some(vec![]);
        (spec.run)(&mut e, Ptr::new(this), 0, data, 2, Ptr::new(value));
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, spec.fill).is_empty());
        assert_eq!(calls(&log, spec.in_place_tail).len(), 1);
        assert!(!calls(&log, spec.move_up).is_empty());
        match spec.assign {
            Some(assign) => assert_eq!(calls(&log, assign).len(), 2),
            None => {
                assert_eq!(e.mem.u32(data), e.mem.u32(value));
                assert_eq!(e.mem.u32(data + size), e.mem.u32(value));
            }
        }
    }

    #[test]
    fn inserting_vertices_grows_or_overwrites_in_place() {
        check_insert(&Insert {
            run: fn_00b20360,
            size: 0x38,
            limit: 0x0492_4924,
            head: VERTEX_RELOCATE,
            fill: VERTEX_FILL,
            tail: VERTEX_RELOCATE,
            in_place_tail: VERTEX_TAIL_COPY,
            move_up: VERTEX_RANGE_BACKWARD,
            assign: None,
        });
    }

    #[test]
    fn inserting_guide_vectors_grows_or_assigns_in_place() {
        check_insert(&Insert {
            run: fn_00b20ce0,
            size: 0x18,
            limit: 0x0aaa_aaaa,
            head: GUIDE_VECTOR_COPY_RANGE,
            fill: GUIDE_VECTOR_FILL,
            tail: GUIDE_VECTOR_COPY_RANGE,
            in_place_tail: GUIDE_VECTOR_COPY_RANGE,
            move_up: GUIDE_VECTOR_MOVE_ELEMENT,
            assign: Some(GUIDE_VECTOR_ASSIGN),
        });
    }

    #[test]
    fn inserting_textures_grows_or_assigns_in_place() {
        check_insert(&Insert {
            run: fn_00b21560,
            size: 0x2C,
            limit: 0x05d1_745d,
            head: TEXTURE_RELOCATE,
            fill: TEXTURE_FILL,
            tail: TEXTURE_TAIL_COPY,
            in_place_tail: TEXTURE_TAIL_COPY,
            move_up: TEXTURE_RANGE_BACKWARD,
            assign: Some(TEXTURE_ASSIGN),
        });
    }

    #[test]
    fn inserting_cvecs_grows_or_overwrites_in_place() {
        check_insert(&Insert {
            run: fn_00b21e00,
            size: 0xC,
            limit: 0x1555_5555,
            head: CVEC_COPY_RANGE,
            fill: CVEC_FILL,
            tail: CVEC_COPY_RANGE,
            in_place_tail: CVEC_TAIL_COPY,
            move_up: CVEC_MOVE_UP,
            assign: None,
        });
    }

    #[test]
    fn inserting_float_vectors_grows_or_assigns_in_place() {
        check_insert(&Insert {
            run: fn_00b22700,
            size: 0x18,
            limit: 0x0aaa_aaaa,
            head: FLOAT_VECTOR_COPY_RANGE,
            fill: FLOAT_VECTOR_FILL,
            tail: FLOAT_VECTOR_COPY_RANGE,
            in_place_tail: FLOAT_VECTOR_COPY_RANGE,
            move_up: FLOAT_VECTOR_MOVE_ELEMENT,
            assign: Some(FLOAT_VECTOR_ASSIGN),
        });
    }

    #[test]
    fn the_proxy_is_allocated_and_points_back_to_the_vector() {
        let mut e = engine();
        let proxy = e.mem.alloc(8);
        reply(&mut e, ALLOCATE_PROXY, proxy);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        let result = e.call(0x00b2_0ba0, &args![this, 0u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, this);
        assert_eq!(calls(&log, ALLOCATE_PROXY), vec![vec![1, 0]]);
        assert_eq!(e.mem.u32(this), proxy);
        assert_eq!(e.mem.u32(proxy), this);
        // A failed allocation leaves a null proxy.
        reply(&mut e, ALLOCATE_PROXY, 0);
        let other = e.mem.alloc(0x18);
        e.call(0x00b2_0ba0, &args![other, 0u32]);
        assert_eq!(e.mem.u32(other), 0);
    }

    /// Two 0x18-byte elements whose first words are live proxy blocks.
    fn two_elements(e: &mut Engine) -> (u32, [u32; 2]) {
        let data = e.mem.alloc(0x30);
        let proxies = [e.mem.alloc(8), e.mem.alloc(8)];
        e.mem.set_u32(data, proxies[0]);
        e.mem.set_u32(data + 0x18, proxies[1]);
        (data, proxies)
    }

    #[test]
    fn destroying_guide_vectors_tidies_each_and_frees_its_proxy() {
        let mut e = engine();
        let (data, proxies) = two_elements(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00b2_0c30, &args![0u32, data, data + 0x30]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, GUIDE_VECTOR_TIDY),
            vec![vec![data], vec![data + 0x18]]
        );
        assert_eq!(e.mem.block_size(proxies[0]), None);
        assert_eq!(e.mem.block_size(proxies[1]), None);
    }

    #[test]
    fn destroying_float_vectors_tidies_each_and_frees_its_proxy() {
        let mut e = engine();
        let (data, proxies) = two_elements(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00b2_2650, &args![0u32, data, data + 0x30]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, FLOAT_VECTOR_TIDY),
            vec![vec![data], vec![data + 0x18]]
        );
        assert_eq!(e.mem.block_size(proxies[0]), None);
        assert_eq!(e.mem.block_size(proxies[1]), None);
    }

    #[test]
    fn the_texture_copy_constructor_empties_assigns_and_copies_the_floats() {
        let mut e = engine();
        e.mem.set_u32(STRING_NPOS, 0xffff_ffff);
        let source = e.new_object::<SFrondTexture>();
        let target = e.new_object::<SFrondTexture>();
        let s = source.addr();
        e.mem.set_f32(s + 0x1C, 1.5);
        e.mem.set_f32(s + 0x20, 2.5);
        e.mem.set_u32(s + 0x24, 0x7f80_0003);
        e.mem.set_f32(s + 0x28, 4.5);
        e.call_log = Some(vec![]);
        let result = e.call(0x00b2_2f80, &args![target, source]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, target.addr());
        assert_eq!(calls(&log, STRING_TIDY), vec![vec![target.addr(), 0, 0]]);
        assert_eq!(
            calls(&log, STRING_ASSIGN),
            vec![vec![target.addr(), s, 0, 0xffff_ffff]]
        );
        let t = target.addr();
        assert_eq!(e.mem.f32(t + 0x1C), 1.5);
        assert_eq!(e.mem.f32(t + 0x20), 2.5);
        assert_eq!(e.mem.u32(t + 0x24), 0x7fc0_0003);
        assert_eq!(e.mem.f32(t + 0x28), 4.5);
    }

    #[test]
    fn the_float_vector_copy_constructor_copies_the_words() {
        let mut e = engine();
        let other = e.mem.alloc(0x18);
        let source = vector(&mut e, other, 4, 3, 3);
        let this = e.mem.alloc(0x18);
        let storage = e.mem.alloc(12);
        e.register_double(FLOAT_VECTOR_RESERVE, move |e, a| {
            e.mem.set_u32(a[0] + 0x0C, storage);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        reply(&mut e, FLOAT_COPY_WORDS, storage + 12);
        e.call_log = Some(vec![]);
        let result = e.call(0x00b2_2ff0, &args![this, other]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, this);
        assert_eq!(
            calls(&log, FLOAT_VECTOR_CONSTRUCT_PROXY),
            vec![vec![this, 0]]
        );
        assert_eq!(calls(&log, FLOAT_VECTOR_RESERVE), vec![vec![this, 3]]);
        assert_eq!(
            calls(&log, FLOAT_COPY_WORDS),
            vec![vec![PROXY, source, PROXY, source + 12, storage, this + 8]]
        );
        assert_eq!(e.mem.u32(this + 0x10), storage + 12);
        // Without room nothing is copied.
        reply(&mut e, FLOAT_VECTOR_RESERVE, 0);
        let empty = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x00b2_2ff0, &args![empty, other]);
        assert!(calls(&e.call_log.take().unwrap(), FLOAT_COPY_WORDS).is_empty());
    }

    /// An iterator over a vector of `size`-byte elements: the iterator at `position`, and its
    /// owner's `first` and `last`; returns the iterator's address.
    fn iterator(e: &mut Engine, size: u32, position_index: u32) -> (u32, u32) {
        let owner = e.mem.alloc(0x18);
        let data = vector(e, owner, size, 4, 4);
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(proxy, owner);
        let at = e.mem.alloc(8);
        e.mem.set_u32(at, proxy);
        e.mem.set_u32(at + 4, data + position_index * size);
        (at, data)
    }

    /// The checked iterator advance for `size`-byte elements: in range moves silently, past the
    /// end or before the start takes the invalid-parameter path.
    fn check_advance(address: u32, size: u32) {
        let mut e = engine();
        let (at, data) = iterator(&mut e, size, 1);
        e.call_log = Some(vec![]);
        let result = e.call(address, &args![at, 2i32]).u32();
        assert_eq!(result, at);
        assert_eq!(e.mem.u32(at + 4), data + 3 * size);
        assert!(calls(&e.call_log.take().unwrap(), INVALID_PARAMETER).is_empty());
        // One past the last element is allowed; two are not.
        e.call_log = Some(vec![]);
        e.call(address, &args![at, 1i32]);
        assert_eq!(e.mem.u32(at + 4), data + 4 * size);
        assert!(calls(&e.call_log.take().unwrap(), INVALID_PARAMETER).is_empty());
        e.call_log = Some(vec![]);
        e.call(address, &args![at, 1i32]);
        assert_eq!(
            calls(&e.call_log.take().unwrap(), INVALID_PARAMETER).len(),
            1
        );
        // Moving before the first element takes the path as well.
        let (back, _) = iterator(&mut e, size, 0);
        e.call_log = Some(vec![]);
        e.call(address, &args![back, -1i32]);
        assert_eq!(
            calls(&e.call_log.take().unwrap(), INVALID_PARAMETER).len(),
            1
        );
    }

    #[test]
    fn the_guide_iterator_advances_in_guide_elements() {
        check_advance(0x00b2_3460, 0x38);
    }

    #[test]
    fn the_texture_iterator_advances_in_texture_elements() {
        check_advance(0x00b2_3520, 0x2C);
    }

    #[test]
    fn the_cvec_iterator_advances_in_cvec_elements() {
        check_advance(0x00b2_35e0, 0xC);
    }

    #[test]
    fn six_word_elements_are_copied_in_order() {
        let mut e = engine();
        let source = e.mem.alloc(0x30);
        let target = e.mem.alloc(0x30);
        for i in 0..12 {
            e.mem.set_u32(source + 4 * i, 0x500 + i);
        }
        let end = e
            .call(0x00b2_36a0, &args![source, source + 0x30, target])
            .u32();
        assert_eq!(end, target + 0x30);
        for i in 0..12 {
            assert_eq!(e.mem.u32(target + 4 * i), 0x500 + i);
        }
    }

    #[test]
    fn assigning_guides_pairs_them_in_order() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let end = e
            .call(0x00b2_3760, &args![0x1000u32, 0x1070u32, 0x2000u32])
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(end, 0x2070);
        assert_eq!(
            calls(&log, GUIDE_ASSIGN),
            vec![vec![0x2000, 0x1000], vec![0x2038, 0x1038]]
        );
    }

    #[test]
    fn assigning_textures_pairs_them_in_order() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let end = e
            .call(0x00b2_3830, &args![0x1000u32, 0x1058u32, 0x2000u32])
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(end, 0x2058);
        assert_eq!(
            calls(&log, TEXTURE_ASSIGN),
            vec![vec![0x2000, 0x1000], vec![0x202C, 0x102C]]
        );
    }

    /// A wrapper that hands its three words and three zero flag words to `callee` and returns
    /// its result.
    fn check_wrapper(address: u32, callee: u32) {
        let mut e = engine();
        reply(&mut e, callee, 0x1234);
        e.call_log = Some(vec![]);
        let result = e.call(address, &args![0x10u32, 0x20u32, 0x30u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, 0x1234);
        assert_eq!(calls(&log, callee), vec![vec![0x10, 0x20, 0x30, 0, 0, 0]]);
    }

    #[test]
    fn the_guide_backward_wrapper_passes_its_words_on() {
        check_wrapper(0x00b2_38e0, GUIDE_RANGE_BACKWARD);
    }

    #[test]
    fn the_vertex_backward_wrapper_passes_its_words_on() {
        check_wrapper(0x00b2_3930, VERTEX_RANGE_BACKWARD);
    }

    #[test]
    fn the_texture_backward_wrapper_passes_its_words_on() {
        check_wrapper(0x00b2_3b00, TEXTURE_RANGE_BACKWARD);
    }

    /// An element allocator: `operator new(count * size)`, zero for no elements, and the
    /// `std::bad_alloc` throw when the byte size does not fit.
    fn check_allocator(address: u32, size: u32) {
        let mut e = engine();
        let block = e.call(address, &args![3u32, 0u32]).u32();
        assert_eq!(e.mem.block_size(block), Some((3 * size).div_ceil(8) * 8));
        // A count of zero allocates the minimum block (a zero-byte request).
        let empty = e.call(address, &args![0u32, 0u32]).u32();
        assert_ne!(empty, 0);
        // The largest count whose byte size fits is allowed; one more throws.
        let too_many = 0xffff_ffffu32 / size + 1;
        e.register_double(CXX_THROW_EXCEPTION, |e, a| {
            assert_eq!(e.mem.u32(a[0]), BAD_ALLOC_VTABLE);
            assert_eq!(a[1], BAD_ALLOC_THROW_INFO);
            panic!("thrown")
        });
        e.register_double(EXCEPTION_FROM_MESSAGE, |e, a| {
            // The message pointer points at a null `const char *`.
            assert_eq!(e.mem.u32(a[1]), 0);
            Ret::default()
        });
        let thrown = catch_unwind(AssertUnwindSafe(|| {
            e.call(address, &args![too_many, 0u32]);
        }));
        assert!(thrown.is_err());
    }

    #[test]
    fn the_guide_allocator_checks_the_byte_size() {
        check_allocator(0x00b2_3980, 0x38);
    }

    #[test]
    fn the_guide_vector_allocator_checks_the_byte_size() {
        check_allocator(0x00b2_3aa0, 0x18);
    }

    #[test]
    fn the_texture_allocator_checks_the_byte_size() {
        check_allocator(0x00b2_3b50, 0x2C);
    }

    #[test]
    fn the_cvec_allocator_checks_the_byte_size() {
        check_allocator(0x00b2_3bb0, 0xC);
    }

    /// The backward move of vectors by swapping: from the last element down, each destination
    /// element is swapped with the source element.
    fn check_backward(address: u32, swap: u32) {
        let mut e = engine();
        e.call_log = Some(vec![]);
        let start = e
            .call(address, &args![0x1000u32, 0x1030u32, 0x2030u32])
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(start, 0x2000);
        assert_eq!(
            calls(&log, swap),
            vec![vec![0x2018, 0x1018], vec![0x2000, 0x1000]]
        );
    }

    #[test]
    fn moving_guide_vectors_back_swaps_each() {
        check_backward(0x00b2_39e0, GUIDE_VECTOR_MOVE_ELEMENT);
    }

    #[test]
    fn moving_float_vectors_back_swaps_each() {
        check_backward(0x00b2_3c10, FLOAT_VECTOR_MOVE_ELEMENT);
    }

    #[test]
    fn the_sort_inserts_small_ranges_and_heap_sorts_deep_ones() {
        let mut e = engine();
        let base = 0x0100_0000u32;
        // One element or none: nothing to do.
        e.call_log = Some(vec![]);
        e.call(0x00b2_3cd0, &args![base, base + 0x38, 5i32, 9u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // Up to 32 elements: the insertion sort.
        e.call_log = Some(vec![]);
        e.call(0x00b2_3cd0, &args![base, base + 0x38 * 32, 5i32, 9u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, GUIDE_INSERTION_SORT),
            vec![vec![base, base + 0x38 * 32, 9, 0]]
        );
        // More than 32 elements with no depth left: the heap is made and popped one by one.
        e.call_log = Some(vec![]);
        e.call(0x00b2_3cd0, &args![base, base + 0x38 * 40, 0i32, 9u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, GUIDE_MAKE_HEAP),
            vec![vec![base, base + 0x38 * 40, 9, 0, 0]]
        );
        let pops = calls(&log, GUIDE_POP_HEAP);
        assert_eq!(pops.len(), 39);
        assert_eq!(pops[0], vec![base, base + 0x38 * 40, 9]);
        assert_eq!(pops[38], vec![base, base + 0x38 * 2, 9]);
        // With depth: the partition splits the range and the parts are sorted in turn.
        e.register(GUIDE_PARTITION, |e, a| {
            let (first, last) = (a[1], a[2]);
            let middle = first + ((last - first) / 0x38 / 2) * 0x38;
            e.mem.set_u32(a[0], middle);
            e.mem.set_u32(a[0] + 4, middle + 0x38);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00b2_3cd0, &args![base, base + 0x38 * 100, 1i32, 9u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, GUIDE_PARTITION).len(), 1);
        // The upper 49 elements, then the lower 50, both without depth left: two heaps.
        assert_eq!(calls(&log, GUIDE_MAKE_HEAP).len(), 2);
        assert_eq!(calls(&log, GUIDE_POP_HEAP).len(), 48 + 49);
    }
}
