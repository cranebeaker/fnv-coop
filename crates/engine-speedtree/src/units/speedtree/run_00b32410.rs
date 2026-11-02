//! `SpeedTree/run_00b32410` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The checked-iterator (VC9 debug) `std::vector` instantiations of the SpeedTree runtime for
//! 16-bit elements (the strips of `CIndexedGeometry` are `std::vector<unsigned short>`), for
//! 32-bit elements (`int`, `unsigned long` and pointers share the shape) and for vectors of those
//! (0x18 bytes each: the PC `std::vector` is the proxy at `+0`, the first, last and end
//! pointers at `+0x0C`, `+0x10` and `+0x14`; [`StlVector`]), and the wind engine's small
//! parameter object. The engine map gives these bodies no names, so they are `fn_<address>`.
//!
//! Every element copy loop and the constructors of the compiler's debug code carry bytes of the
//! empty allocator object that the exe leaves uninitialised; they are passed as zeros.
//! Functions that carry an `FS:[0]` exception frame are marked `Translation incomplete` (the
//! unwinding handler is not translated; the throws are).

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{
    checked_iterator, vector_len, StlVector, INVALID_PARAMETER, OPERATOR_DELETE, OPERATOR_NEW,
};

/// `00b0fb80`: the `std::vector` length error (throws; does not return).
pub(crate) const LENGTH_ERROR: u32 = 0x00b0_fb80;
/// `00ec7ce3`: `memmove_s(destination, capacity, source, count)` (`cdecl`).
pub(crate) const MEMMOVE_S: u32 = 0x00ec_7ce3;
/// `0044d670`: the proxy allocator `allocate(1, 0)` (`cdecl`).
pub(crate) const ALLOCATE_PROXY: u32 = 0x0044_d670;
/// `00b23aa0`: `allocate(count, 0)` of the vector-of-vectors storage (`cdecl`, 0x18 bytes each).
const ALLOCATE_VECTORS: u32 = 0x00b2_3aa0;
/// `00b16830`: whether two checked iterators differ (`thiscall`, `this` = the first iterator
/// pair, the second pair's address).
const ITERATORS_DIFFER: u32 = 0x00b1_6830;
/// `00b16890`: advances an iterator of the vector-of-vectors by a count (`thiscall`).
const ADVANCE_VECTOR_ITERATOR: u32 = 0x00b1_6890;
/// `00b2e330`: advances an iterator of a 32-bit element vector by a count (`thiscall`).
const ADVANCE_WORD_ITERATOR: u32 = 0x00b2_e330;
/// `00ac1000`: the same advance, as the one-element insert calls it (`thiscall`).
const ADVANCE_WORD_ITERATOR_INSERT: u32 = 0x00ac_1000;
/// `00b16070`: the insertion of `count` copies into a 32-bit element vector (`thiscall`: the
/// vector, the iterator's proxy and position, the count, the address of the value).
const WORD_INSERT_N: u32 = 0x00b1_6070;
/// `00abc320`: the same insertion as the one-element insert calls it.
const WORD_INSERT_N_ONE: u32 = 0x00ab_c320;
/// `00b42e50`: the copy of `[first, last)` to `dest`, returning the end (`cdecl`), of a 32-bit
/// element vector.
const WORD_COPY: u32 = 0x00b4_2e50;
/// `00b24960`: the checked uninitialised copy of a 32-bit element vector (`cdecl`, six words).
const WORD_UNINIT_COPY_CHECKED: u32 = 0x00b2_4960;
/// `00b17e90`, `00b17f50`, `00b17db0`, `00b177e0`: the 32-bit element vector's buy, tidy,
/// default constructor and uninitialised copy.
const WORD_BUY: u32 = 0x00b1_7e90;
const WORD_TIDY: u32 = 0x00b1_7f50;
const WORD_DEFAULT_CONSTRUCT: u32 = 0x00b1_7db0;
const WORD_UNINIT_COPY: u32 = 0x00b1_77e0;
/// `00b32350`: `clear` of a 32-bit element vector.
const WORD_CLEAR: u32 = 0x00b3_2350;
/// `00b1f590`: the buy of a vector of vectors (`thiscall`, a count, answers a `bool`).
const VECTORS_BUY: u32 = 0x00b1_f590;
/// `00b39460`: frees the block the object's first word points at (`thiscall`).
const PROXY_RELEASE: u32 = 0x00b3_9460;
/// `00b20ba0`: the container proxy constructor (`thiscall`, `this` = the vector, one word, the
/// empty allocator).
const PROXY_CONSTRUCT: u32 = 0x00b2_0ba0;
/// `00ec78f1` (the exception base constructor), the `std::bad_alloc` vtable and its throw
/// information, and `_CxxThrowException` (`00ec7894`).
const EXCEPTION_FROM_MESSAGE: u32 = 0x00ec_78f1;
const BAD_ALLOC_VTABLE: u32 = 0x0101_76f8;
const BAD_ALLOC_THROW_INFO: u32 = 0x0114_0ef8;
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;

/// The most 16-bit elements a vector can hold (`0x7fffffff`, `max_size`).
const SHORT_LIMIT: u32 = 0x7fff_ffff;
/// The most vectors a vector of vectors can hold (`0xaaaaaaa`: `0xffffffff / 0x18` rounded as
/// the allocator's `max_size` does).
const VECTORS_LIMIT: u32 = 0x0aaa_aaaa;

/// The helpers one vector-of-vectors instantiation calls for its element vector.
struct VectorElement {
    /// The element's tidy (`this`).
    tidy: u32,
    /// The element's copy constructor (`this`, the source).
    copy_construct: u32,
    /// The element's assignment (`this`, the source).
    assign: u32,
    /// The element's swap (`this`, the other).
    swap: u32,
    /// The element's default constructor (`this`).
    default_construct: u32,
    /// The destruction of a range of elements (`this`, first, last).
    destroy_range: u32,
    /// The fill construction (`dst`, count, value, three words).
    uninit_fill: u32,
    /// The construction by moving a range (`first`, `last`, `dest`, three words).
    uninit_move: u32,
    /// The backward move of a range (`first`, `last`, `dest_end`).
    move_backward: u32,
    /// The erase of a range (`this`, `out`, four iterator words).
    erase: u32,
    /// The insertion of `count` copies (`this`, proxy, position, count, value).
    insert_n: u32,
}

/// The vector of 16-bit element vectors.
const SHORT_VECTORS: VectorElement = VectorElement {
    tidy: 0x00b3_4d90,
    copy_construct: 0x00b3_57d0,
    assign: 0x00b3_6600,
    swap: 0x00b3_7190,
    default_construct: 0x00b3_36a0,
    destroy_range: 0x00b3_2bb0,
    uninit_fill: 0x00b3_60a0,
    uninit_move: 0x00b3_6bf0,
    move_backward: 0x00b3_5ce0,
    erase: 0x00b3_2ad0,
    insert_n: 0x00b3_3bd0,
};

/// The vector of 32-bit element vectors.
const WORD_VECTORS: VectorElement = VectorElement {
    tidy: WORD_TIDY,
    copy_construct: 0x00b3_55d0,
    assign: 0x00b3_6330,
    swap: 0x00b3_6f60,
    default_construct: WORD_DEFAULT_CONSTRUCT,
    destroy_range: 0x00b3_3110,
    uninit_fill: 0x00b3_61b0,
    uninit_move: 0x00b3_6d70,
    move_backward: 0x00b3_5da0,
    erase: 0x00b3_3030,
    insert_n: 0x00b3_4450,
};

/// The first element pointer (`+0x0C`).
pub(crate) fn first_of(e: &Engine, vector: u32) -> u32 {
    e.mem.u32(vector + 0x0C)
}

/// The last element pointer (`+0x10`).
pub(crate) fn last_of(e: &Engine, vector: u32) -> u32 {
    e.mem.u32(vector + 0x10)
}

/// `(*(vector + field) - first) / size` as the compiler's signed shift or `IDIV`.
pub(crate) fn span(e: &Engine, vector: u32, field: u32, size: i32) -> u32 {
    let bytes = e.mem.u32(vector + field).wrapping_sub(first_of(e, vector)) as i32;
    if size.count_ones() == 1 {
        (bytes >> size.trailing_zeros()) as u32
    } else {
        (bytes / size) as u32
    }
}

/// The capacity of a vector: zero without storage, else `(end - first) / size`.
fn capacity_of(e: &Engine, vector: u32, size: i32) -> u32 {
    if first_of(e, vector) == 0 {
        0
    } else {
        span(e, vector, 0x14, size)
    }
}

/// Zeroes the first, last and end pointers.
pub(crate) fn clear_pointers(e: &mut Engine, vector: u32) {
    e.mem.set_u32(vector + 0x0C, 0);
    e.mem.set_u32(vector + 0x10, 0);
    e.mem.set_u32(vector + 0x14, 0);
}

/// A flag the compiler folded to zero (`XOR ECX,ECX; AND ECX,1`): the code behind it never
/// runs, but it is translated; `black_box` keeps the optimiser from erasing the branch.
fn folded_zero() -> u32 {
    std::hint::black_box(0)
}

/// The `std::bad_alloc` throw of the allocators: the exception is built from a pointer to a
/// null message, gets the `bad_alloc` vtable and is thrown (`_CxxThrowException`). Does not
/// return.
fn throw_bad_alloc(e: &mut Engine) -> ! {
    e.with_stack(4, |e, message| {
        e.mem.set_u32(message.addr(), 0);
        e.with_stack(0xc, |e, exception| {
            e.call(EXCEPTION_FROM_MESSAGE, &args![exception, message]);
            e.mem.set_u32(exception.addr(), BAD_ALLOC_VTABLE);
            e.call(CXX_THROW_EXCEPTION, &args![exception, BAD_ALLOC_THROW_INFO]);
        });
    });
    unreachable!("__CxxThrowException does not return (00ec7894)")
}

/// The allocator of `count` elements of `size` bytes: zero takes no allocation, a count whose
/// size does not fit in 32 bits throws `std::bad_alloc`; otherwise `operator new(count * size)`.
pub(crate) fn allocate_elements(e: &mut Engine, count: u32, size: u32) -> u32 {
    if count != 0 && 0xffff_ffffu32 / count < size {
        throw_bad_alloc(e);
    }
    e.call(OPERATOR_NEW, &args![count.wrapping_mul(size)]).u32()
}

/// The copy of a range with `memmove_s` (`destination`, its capacity, the source, the size).
pub(crate) fn move_bytes(e: &mut Engine, destination: u32, source: u32, bytes: u32) {
    e.call(MEMMOVE_S, &args![destination, bytes, source, bytes]);
}

/// Allocates the container proxy and points its first word back at the vector (the pointer
/// stays null when the allocation fails) and stores it in the vector's `+0`.
pub(crate) fn construct_proxy(e: &mut Engine, this: Ptr<StlVector>) {
    let block = e.call(ALLOCATE_PROXY, &args![1u32, 0u32]).u32();
    let proxy = if block != 0 {
        e.mem.set_u32(block, this.addr());
        block
    } else {
        0
    };
    e.mem.set_u32(this.addr(), proxy);
}

/// The checked erase of `[first, last)` in a vector whose elements are `size` bytes: the
/// iterators at both positions are checked; when they differ, the elements after the range are
/// moved down with `shift` (`last`, the end, `first`; answers the new end), `release` is told
/// about the elements past the new end (`this`, the new end, the old end; `None` when the
/// elements need no destruction) and the end is set to the new end. The iterator at `first` is
/// written to `out`, which is returned.
fn erase_range(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    first: u32,
    last: u32,
    shift: u32,
    release: Option<u32>,
) -> Ptr {
    let v = this.addr();
    let first_iterator = [checked_iterator(e, v, first), first];
    let last_iterator = [checked_iterator(e, v, last), last];
    let differ = e.with_stack(0x10, |e, pairs| {
        let first_pair = pairs.addr();
        let last_pair = pairs.addr() + 8;
        e.mem.set_u32(first_pair, first_iterator[0]);
        e.mem.set_u32(first_pair + 4, first_iterator[1]);
        e.mem.set_u32(last_pair, last_iterator[0]);
        e.mem.set_u32(last_pair + 4, last_iterator[1]);
        e.call(ITERATORS_DIFFER, &args![first_pair, last_pair])
            .bool()
    });
    if differ {
        let end = last_of(e, v);
        let new_end = e.call(shift, &args![last, end, first]).u32();
        if let Some(release) = release {
            let end = last_of(e, v);
            e.call(release, &args![this, new_end, end]);
        }
        e.mem.set_u32(v + 0x10, new_end);
    }
    e.mem.set_u32(out.addr(), first_iterator[0]);
    e.mem.set_u32(out.addr() + 4, first_iterator[1]);
    out
}

// Translated from 00b32410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<unsigned short>::push_back` (the engine map gives the body no name): with
/// spare capacity (fewer elements than the capacity) the element at `value` is stored at the end
/// (`00b36b80`, a count of one) and the end advanced by one element; otherwise the checked
/// insert at the end iterator (`00b339d0`) grows the vector.
pub fn fn_00b32410(e: &mut Engine, this: Ptr<StlVector>, value: u32) {
    let v = this.addr();
    let size = span(e, v, 0x10, 2);
    let capacity = capacity_of(e, v, 2);
    if size < capacity {
        let end = last_of(e, v);
        fn_00b36b80(e, end, 1, value);
        // `MOV EDX,1; SHL EDX,1; ADD EDX,end`: one element further.
        e.mem.set_u32(v + 0x10, end.wrapping_add(2));
    } else {
        let end = last_of(e, v);
        let proxy = checked_iterator(e, v, end);
        e.with_stack(8, |e, out| {
            fn_00b339d0(e, this, out, proxy, end, value);
        });
    }
}

// Translated from 00b32540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<unsigned short>::clear`: erases the whole range with `00b33af0`, from the
/// checked iterator at the first element to the one at the last (the last is checked first).
pub fn fn_00b32540(e: &mut Engine, this: Ptr<StlVector>) {
    let v = this.addr();
    let last = last_of(e, v);
    let last_proxy = checked_iterator(e, v, last);
    let first = first_of(e, v);
    let first_proxy = checked_iterator(e, v, first);
    e.with_stack(8, |e, out| {
        fn_00b33af0(e, this, out, first_proxy, first, last_proxy, last);
    });
}

// Translated from 00b32600 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b32600 - the FS:[0] exception frame (the handler that tidies the
// vector if a copy throws) is not translated.
/// The constructor of a 32-bit element vector of `count` copies of the element at `value`
/// (the engine map gives it no name): buys the storage (`00b17e90`) and, when it answers true,
/// fills it (`00b35ca0`) and sets the end after the filled elements.
pub fn fn_00b32600(e: &mut Engine, this: Ptr<StlVector>, count: u32, value: u32) {
    let v = this.addr();
    if e.call(WORD_BUY, &args![this, count]).bool() {
        let first = first_of(e, v);
        fn_00b35ca0(e, first, count, value, v + 8);
        e.mem
            .set_u32(v + 0x10, first.wrapping_add(count.wrapping_mul(4)));
    }
}

// Translated from 00b326e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b326e0 - the FS:[0] exception frame is not translated.
/// The checked-iterator container proxy constructor of a `std::vector` (the engine map gives it
/// no name; the one stack word is the empty allocator, not read): allocates a proxy
/// (`0044d670`), points its first word back at the vector (no proxy when the allocation fails)
/// and stores it at `+0`. Returns `this`.
pub fn fn_00b326e0(e: &mut Engine, this: Ptr<StlVector>, _unused_1: u32) -> Ptr<StlVector> {
    construct_proxy(e, this);
    this
}

/// The constructor of a vector of vectors of `count` copies of the vector at `value`: buys the
/// storage (`00b1f590`) and, when it answers true, constructs the copies with `fill`
/// (`dst`, count, value and three words: the allocator at `this + 8`, a byte the exe leaves
/// uninitialised and a zero) and sets the end after them.
fn construct_vectors(
    e: &mut Engine,
    this: Ptr<StlVector>,
    count: u32,
    value: u32,
    fill: fn(&mut Engine, u32, u32, u32, u32, u32, u32),
) {
    let v = this.addr();
    if e.call(VECTORS_BUY, &args![this, count]).bool() {
        let first = first_of(e, v);
        fill(e, first, count, value, v + 8, 0, 0);
        e.mem
            .set_u32(v + 0x10, count.wrapping_mul(0x18).wrapping_add(first));
    }
}

// Translated from 00b32780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b32780 - the FS:[0] exception frame is not translated.
/// The constructor of a vector of 16-bit element vectors of `count` copies of the vector at
/// `value` (the engine map gives it no name); see [`construct_vectors`].
pub fn fn_00b32780(e: &mut Engine, this: Ptr<StlVector>, count: u32, value: u32) {
    construct_vectors(e, this, count, value, fn_00b360a0);
}

/// [`resize_vectors_at`] with the value as the uniform ABI hands it, six words: they are laid
/// out in an area that stands for the callee's argument area (`[EBP+0xC]`..`[EBP+0x20]`).
fn resize_vectors(
    e: &mut Engine,
    element: &VectorElement,
    this: Ptr<StlVector>,
    new_size: u32,
    value: [u32; 6],
) {
    e.with_stack(0x18, |e, slot| {
        for (i, word) in value.iter().enumerate() {
            e.mem.set_u32(slot.addr() + 4 * i as u32, *word);
        }
        resize_vectors_at(e, element, this, new_size, slot.cast());
    });
}

/// The `resize(new_size, value)` of a vector of vectors, the value passed by value: a vector
/// in the callee's argument area (`[EBP+0xC]`), which `value` names. The code works on it in
/// place (`LEA ECX,[EBP+0xC]`), so its proxy must name that area, and tears it down at its
/// end: its storage tidied with the element's tidy and its proxy freed. Growing inserts the
/// missing copies with the element's insert (`this`, the end iterator, the count, the address
/// of the value); shrinking advances the iterator at the first element by the new size
/// (`00b16890`) and erases from there to the end.
fn resize_vectors_at(
    e: &mut Engine,
    element: &VectorElement,
    this: Ptr<StlVector>,
    new_size: u32,
    value: Ptr<StlVector>,
) {
    let v = this.addr();
    if vector_len(e, v, 0x18) < new_size {
        let last = last_of(e, v);
        let proxy = checked_iterator(e, v, last);
        let count = new_size.wrapping_sub(vector_len(e, v, 0x18));
        // 00b32e92 (00b32932 in 00b32880): `LEA ECX,[EBP+0xC]`, the argument area itself.
        e.call(element.insert_n, &args![this, proxy, last, count, value]);
    } else if new_size < vector_len(e, v, 0x18) {
        let last = last_of(e, v);
        let last_proxy = checked_iterator(e, v, last);
        let first = first_of(e, v);
        let first_proxy = checked_iterator(e, v, first);
        let advanced = e.with_stack(8, |e, pair| {
            e.mem.set_u32(pair.addr(), first_proxy);
            e.mem.set_u32(pair.addr() + 4, first);
            e.call(ADVANCE_VECTOR_ITERATOR, &args![pair, new_size]);
            (e.mem.u32(pair.addr()), e.mem.u32(pair.addr() + 4))
        });
        e.with_stack(8, |e, out| {
            e.call(
                element.erase,
                &args![this, out, advanced.0, advanced.1, last_proxy, last],
            );
        });
    }
    e.call(element.tidy, &args![value]); // 00b32ff3 (00b32a93): the area's vector, in place
    let proxy = e.mem.u32(value.addr()); // 00b33002 (00b32aa2)
    e.call(OPERATOR_DELETE, &args![proxy]); // 00b33012 (00b32ab2)
}

// Translated from 00b32880 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b32880 - the FS:[0] exception frame is not translated.
/// `resize(new_size, value)` of a vector of 16-bit element vectors (the engine map gives it no
/// name); the value (a 0x18-byte vector) is passed by value in the six words after `new_size`
/// (`RET 0x1C`). `value` is the address of the argument area that holds it (the exe's
/// `[EBP+0xC]`): the one caller, `00b2fb30`, builds the vector there (`00b2fb77`..`00b2fb82`),
/// so its proxy names that area, which this code copies from and tears down in place. See
/// [`resize_vectors_at`].
pub fn fn_00b32880(e: &mut Engine, this: Ptr<StlVector>, new_size: u32, value: Ptr<StlVector>) {
    resize_vectors_at(e, &SHORT_VECTORS, this, new_size, value); // 00b32932, 00b32a93: in place
}

/// Through the uniform ABI the six words of `00b32880`'s by-value vector arrive as values: they
/// are laid out in an area that stands for its argument area ([`resize_vectors`]). The exe's
/// one caller, `00b2fb30` (`00b2fb9b`), calls [`fn_00b32880`] with the area it built the
/// vector in.
fn entry_00b32880(e: &mut Engine, a: &[u32]) -> Ret {
    let mut value = [0u32; 6];
    value.copy_from_slice(&a[2..8]); // 00b32880's [EBP+0xC]..[EBP+0x20]
    resize_vectors(e, &SHORT_VECTORS, Ptr::new(a[0]), a[1], value);
    Ret::default()
}

// Translated from 00b32ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked erase of `[first, last)` from a vector of 16-bit element vectors (the engine
/// map gives it no name; the iterators' proxy words, at words 2 and 4, are not read): when the
/// iterators differ, the elements after the range are assigned down (`00b35a90`: last, the end,
/// first; answers the new end), the vectors past the new end are destroyed (`00b32bb0`) and the
/// end set. The iterator at `first` is written to `out`, which is returned.
pub fn fn_00b32ad0(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    _unused_2: u32,
    first: u32,
    _unused_4: u32,
    last: u32,
) -> Ptr {
    erase_range(e, this, out, first, last, 0x00b3_5a90, Some(0x00b3_2bb0))
}

/// The destruction of the vectors in `[first, last)`: each one tidied with `tidy` and its proxy
/// block freed. (The code after the free, that would free the element itself, sits behind a
/// flag the compiler folded to zero.)
fn destroy_vectors(e: &mut Engine, tidy: u32, first: u32, last: u32) {
    let mut element = first;
    while element != last {
        e.call(tidy, &args![element]);
        let proxy = e.mem.u32(element);
        e.call(OPERATOR_DELETE, &args![proxy]);
        if folded_zero() & 1 != 0 {
            e.call(OPERATOR_DELETE, &args![element]);
        }
        element = element.wrapping_add(0x18);
    }
}

// Translated from 00b32bb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b32bb0 - the FS:[0] exception frame is not translated.
/// Destroys the 16-bit element vectors in `[first, last)` of a vector of vectors (the engine
/// map gives it no name; `this` is not read): each one tidied (`00b34d90`) and its proxy freed.
pub fn fn_00b32bb0(e: &mut Engine, _this: Ptr<StlVector>, first: u32, last: u32) {
    destroy_vectors(e, 0x00b3_4d90, first, last);
}

/// The tidy of a vector of vectors: with storage, destroys the elements (`destroy`: this, the
/// first and last pointers) and frees the block; then zeroes the three pointers. (The element
/// count the exe also computes is not used.)
fn tidy_vectors(e: &mut Engine, this: Ptr<StlVector>, destroy: u32) {
    let v = this.addr();
    if first_of(e, v) != 0 {
        let first = first_of(e, v);
        let last = last_of(e, v);
        e.call(destroy, &args![this, first, last]);
        e.call(OPERATOR_DELETE, &args![first]);
    }
    clear_pointers(e, v);
}

// Translated from 00b32c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tidies a vector of 16-bit element vectors (the engine map gives it no name): destroys the
/// elements (`00b32bb0`), frees the block and zeroes the pointers.
pub fn fn_00b32c60(e: &mut Engine, this: Ptr<StlVector>) {
    tidy_vectors(e, this, 0x00b3_2bb0);
}

// Translated from 00b32ce0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b32ce0 - the FS:[0] exception frame is not translated.
/// The constructor of a vector of 32-bit element vectors of `count` copies of the vector at
/// `value` (the engine map gives it no name); see [`construct_vectors`].
pub fn fn_00b32ce0(e: &mut Engine, this: Ptr<StlVector>, count: u32, value: u32) {
    construct_vectors(e, this, count, value, fn_00b361b0);
}

// Translated from 00b32de0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b32de0 - the FS:[0] exception frame is not translated.
/// `resize(new_size, value)` of a vector of 32-bit element vectors (the engine map gives it no
/// name); the value (a 0x18-byte vector) is passed by value in the six words after `new_size`
/// (`RET 0x1C`). `value` is the address of the argument area that holds it (the exe's
/// `[EBP+0xC]`): the one caller, `00b2fb30`, builds the vector there (`00b2fba0`..`00b2fbab`),
/// so its proxy names that area, which this code copies from and tears down in place. See
/// [`resize_vectors_at`].
pub fn fn_00b32de0(e: &mut Engine, this: Ptr<StlVector>, new_size: u32, value: Ptr<StlVector>) {
    resize_vectors_at(e, &WORD_VECTORS, this, new_size, value); // 00b32e92, 00b32ff3: in place
}

/// Through the uniform ABI the six words of `00b32de0`'s by-value vector arrive as values: they
/// are laid out in an area that stands for its argument area ([`resize_vectors`]). The exe's
/// one caller, `00b2fb30` (`00b2fbc4`), calls [`fn_00b32de0`] with the area it built the
/// vector in.
fn entry_00b32de0(e: &mut Engine, a: &[u32]) -> Ret {
    let mut value = [0u32; 6];
    value.copy_from_slice(&a[2..8]); // 00b32de0's [EBP+0xC]..[EBP+0x20]
    resize_vectors(e, &WORD_VECTORS, Ptr::new(a[0]), a[1], value);
    Ret::default()
}

// Translated from 00b33030 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked erase of `[first, last)` from a vector of 32-bit element vectors (the engine map
/// gives it no name): as `00b32ad0`, with the assigning copy `00b35b60` and the destruction
/// `00b33110`.
pub fn fn_00b33030(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    _unused_2: u32,
    first: u32,
    _unused_4: u32,
    last: u32,
) -> Ptr {
    erase_range(e, this, out, first, last, 0x00b3_5b60, Some(0x00b3_3110))
}

// Translated from 00b33110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b33110 - the FS:[0] exception frame is not translated.
/// Destroys the 32-bit element vectors in `[first, last)` of a vector of vectors (the engine
/// map gives it no name; `this` is not read): each one tidied (`00b17f50`) and its proxy freed.
pub fn fn_00b33110(e: &mut Engine, _this: Ptr<StlVector>, first: u32, last: u32) {
    destroy_vectors(e, WORD_TIDY, first, last);
}

// Translated from 00b331c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tidies a vector of 32-bit element vectors (the engine map gives it no name): destroys the
/// elements (`00b33110`), frees the block and zeroes the pointers.
pub fn fn_00b331c0(e: &mut Engine, this: Ptr<StlVector>) {
    tidy_vectors(e, this, 0x00b3_3110);
}

// Translated from 00b33240 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked insert of one 32-bit element before the iterator (`proxy`, `position`) (the
/// engine map gives it no name; `PUSH_COLOR_SLOW` of `CIndexedGeometry`): remembers the index of
/// the position (0 for an empty vector, otherwise after checking that the iterator belongs to
/// the vector), inserts with `00abc320` (`this`, proxy, position, 1, value), then writes to
/// `out` the iterator (proxy, first element) advanced by that index (`00ac1000`). Returns `out`.
pub fn fn_00b33240(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: u32,
) -> Ptr {
    let v = this.addr();
    let index = if span(e, v, 0x10, 4) == 0 {
        0
    } else {
        let first = first_of(e, v);
        let iterator_proxy = checked_iterator(e, v, first);
        if proxy == 0 || proxy != iterator_proxy {
            e.call(INVALID_PARAMETER, &args![]);
        }
        (position.wrapping_sub(first) as i32 >> 2) as u32
    };
    e.call(
        WORD_INSERT_N_ONE,
        &args![this, proxy, position, 1u32, value],
    );
    let first = first_of(e, v);
    let first_proxy = checked_iterator(e, v, first);
    e.with_stack(8, |e, iterator| {
        e.mem.set_u32(iterator.addr(), first_proxy);
        e.mem.set_u32(iterator.addr() + 4, first);
        e.call(ADVANCE_WORD_ITERATOR_INSERT, &args![iterator, index]);
        let proxy_out = e.mem.u32(iterator.addr());
        let position_out = e.mem.u32(iterator.addr() + 4);
        e.mem.set_u32(out.addr(), proxy_out);
        e.mem.set_u32(out.addr() + 4, position_out);
    });
    out
}

// Translated from 00b33370 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b33370 - the FS:[0] exception frame is not translated.
/// The destructor of a 32-bit element vector (the engine map gives it no name): tidies it
/// (`00b17f50`) and frees its proxy block.
pub fn fn_00b33370(e: &mut Engine, this: Ptr<StlVector>) {
    e.call(WORD_TIDY, &args![this]);
    let proxy = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

/// How an element vector is resized: the element size and the insert (`this`, the end
/// iterator's proxy and position, the count, the address of the value), the iterator advance
/// (the iterator, the count) and the erase (`this`, `out`, four iterator words) it calls.
struct ElementOps {
    size: i32,
    insert_n: u32,
    advance: u32,
    erase: u32,
}

/// The `resize(new_size, value)` of an element vector, the value one word passed by value.
/// Growing inserts the missing copies; shrinking advances the iterator at the first element by
/// the new size and erases from there to the end.
fn resize_elements(
    e: &mut Engine,
    ops: &ElementOps,
    this: Ptr<StlVector>,
    new_size: u32,
    value: u32,
) {
    let v = this.addr();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        if span(e, v, 0x10, ops.size) < new_size {
            let last = last_of(e, v);
            let proxy = checked_iterator(e, v, last);
            let count = new_size.wrapping_sub(span(e, v, 0x10, ops.size));
            e.call(ops.insert_n, &args![this, proxy, last, count, slot]);
        } else if new_size < span(e, v, 0x10, ops.size) {
            let last = last_of(e, v);
            let last_proxy = checked_iterator(e, v, last);
            let first = first_of(e, v);
            let first_proxy = checked_iterator(e, v, first);
            let advanced = e.with_stack(8, |e, pair| {
                e.mem.set_u32(pair.addr(), first_proxy);
                e.mem.set_u32(pair.addr() + 4, first);
                e.call(ops.advance, &args![pair, new_size]);
                (e.mem.u32(pair.addr()), e.mem.u32(pair.addr() + 4))
            });
            e.with_stack(8, |e, out| {
                e.call(
                    ops.erase,
                    &args![this, out, advanced.0, advanced.1, last_proxy, last],
                );
            });
        }
    });
}

// Translated from 00b333e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `resize(new_size, value)` of a 32-bit element vector (`GROW_INT_VECTOR` of
/// `CIndexedGeometry`; the engine map gives it no name): see [`resize_elements`] (insert
/// `00b16070`, advance `00b2e330`, erase `00b335c0`).
pub fn fn_00b333e0(e: &mut Engine, this: Ptr<StlVector>, new_size: u32, value: u32) {
    let ops = ElementOps {
        size: 4,
        insert_n: WORD_INSERT_N,
        advance: ADVANCE_WORD_ITERATOR,
        erase: 0x00b3_35c0,
    };
    resize_elements(e, &ops, this, new_size, value);
}

// Translated from 00b335c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked erase of `[first, last)` from a 32-bit element vector (the engine map gives it
/// no name; the iterators' proxy words, at words 2 and 4, are not read): when the iterators
/// differ, the elements after the range are copied down (`00b42e50`: last, the end, first;
/// answers the new end) and the end set there. The iterator at `first` is written to `out`,
/// which is returned.
pub fn fn_00b335c0(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    _unused_2: u32,
    first: u32,
    _unused_4: u32,
    last: u32,
) -> Ptr {
    erase_range(e, this, out, first, last, WORD_COPY, None)
}

// Translated from 00b336a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b336a0 - the FS:[0] exception frame is not translated.
/// The default constructor of a 16-bit element vector (`CONSTRUCT_SHORT_VECTOR` of
/// `CIndexedGeometry`; the engine map gives it no name): the container proxy, then the buy of
/// zero elements (`00b34cd0`). Returns `this`.
pub fn fn_00b336a0(e: &mut Engine, this: Ptr<StlVector>) -> Ptr<StlVector> {
    construct_proxy(e, this);
    fn_00b34cd0(e, this, 0);
    this
}

// Translated from 00b33780 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b33780 - the FS:[0] exception frame is not translated.
/// The destructor of a 16-bit element vector (the engine map gives it no name): tidies it
/// (`00b34d90`) and frees its proxy block.
pub fn fn_00b33780(e: &mut Engine, this: Ptr<StlVector>) {
    fn_00b34d90(e, this);
    let proxy = e.mem.u32(this.addr());
    e.call(OPERATOR_DELETE, &args![proxy]);
}

// Translated from 00b337f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `resize(new_size, value)` of a 16-bit element vector (`GROW_SHORT_VECTOR` of
/// `CIndexedGeometry`; the engine map gives it no name): see [`resize_elements`] (insert
/// `00b34e10`, advance `00b359d0`, erase `00b33af0`).
pub fn fn_00b337f0(e: &mut Engine, this: Ptr<StlVector>, new_size: u32, value: u32) {
    let ops = ElementOps {
        size: 2,
        insert_n: 0x00b3_4e10,
        advance: 0x00b3_59d0,
        erase: 0x00b3_3af0,
    };
    resize_elements(e, &ops, this, new_size, value);
}

// Translated from 00b339d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked insert of one 16-bit element before the iterator (`proxy`, `position`) (the
/// engine map gives it no name): remembers the index of the position (0 for an empty vector,
/// otherwise after checking that the iterator belongs to the vector), inserts with `00b34e10`
/// (`this`, proxy, position, 1, value), then writes to `out` the iterator (proxy, first
/// element) advanced by that index (`00b359d0`). Returns `out`.
pub fn fn_00b339d0(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    proxy: u32,
    position: u32,
    value: u32,
) -> Ptr {
    let v = this.addr();
    let index = if span(e, v, 0x10, 2) == 0 {
        0
    } else {
        let first = first_of(e, v);
        let iterator_proxy = checked_iterator(e, v, first);
        if proxy == 0 || proxy != iterator_proxy {
            e.call(INVALID_PARAMETER, &args![]);
        }
        (position.wrapping_sub(first) as i32 >> 1) as u32
    };
    fn_00b34e10(e, this, proxy, position, 1, value);
    let first = first_of(e, v);
    let first_proxy = checked_iterator(e, v, first);
    e.with_stack(8, |e, iterator| {
        e.mem.set_u32(iterator.addr(), first_proxy);
        e.mem.set_u32(iterator.addr() + 4, first);
        fn_00b359d0(e, iterator, index);
        let proxy_out = e.mem.u32(iterator.addr());
        let position_out = e.mem.u32(iterator.addr() + 4);
        e.mem.set_u32(out.addr(), proxy_out);
        e.mem.set_u32(out.addr() + 4, position_out);
    });
    out
}

// Translated from 00b33af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked erase of `[first, last)` from a 16-bit element vector (the engine map gives it
/// no name; the iterators' proxy words, at words 2 and 4, are not read): when the iterators
/// differ, the elements after the range are copied down (`00b35c30`: last, the end, first;
/// answers the new end) and the end set there. The iterator at `first` is written to `out`,
/// which is returned.
pub fn fn_00b33af0(
    e: &mut Engine,
    this: Ptr<StlVector>,
    out: Ptr,
    _unused_2: u32,
    first: u32,
    _unused_4: u32,
    last: u32,
) -> Ptr {
    erase_range(e, this, out, first, last, 0x00b3_5c30, None)
}

/// The insertion of `count` copies of the vector at `value` before `position` in a vector of
/// vectors (`_Insert_n`; the first word, the iterator's proxy, is not read). A count of zero
/// does nothing; a result above the maximum size (`0xaaaaaaa` vectors) raises the length error
/// (`00b0fb80`). Without room the vector grows to one and a half times its capacity (at least
/// the new size, `00b23aa0`): the vectors before the position are moved to the new storage, the
/// copies constructed and the later vectors moved after them, the old ones destroyed and the old
/// storage freed. With room, a temporary copy of the value is made: when fewer vectors follow
/// the position than are inserted the tail moves up, copies are constructed past the old end
/// and the rest of the range assigned; otherwise the last `count` vectors are moved past the
/// end, the others moved up and the range assigned.
fn insert_vectors(
    e: &mut Engine,
    element: &VectorElement,
    this: Ptr<StlVector>,
    position: u32,
    count: u32,
    value: u32,
) {
    let v = this.addr();
    let mut capacity = capacity_of(e, v, 0x18);
    if count == 0 {
        return;
    }
    // The limit is `max_size`, at least one.
    let limit = VECTORS_LIMIT.max(1);
    if limit.wrapping_sub(vector_len(e, v, 0x18)) < count {
        e.call(LENGTH_ERROR, &args![]);
        return;
    }
    if capacity < vector_len(e, v, 0x18).wrapping_add(count) {
        capacity = if limit.wrapping_sub(capacity >> 1) < capacity {
            0
        } else {
            (capacity >> 1).wrapping_add(capacity)
        };
        if capacity < vector_len(e, v, 0x18).wrapping_add(count) {
            capacity = vector_len(e, v, 0x18).wrapping_add(count);
        }
        let storage = e.call(ALLOCATE_VECTORS, &args![capacity, 0u32]).u32();
        let first = first_of(e, v);
        let moved = e
            .call(
                element.uninit_move,
                &args![first, position, storage, v + 8, 0u32, 0u32],
            )
            .u32();
        e.call(
            element.uninit_fill,
            &args![moved, count, value, v + 8, 0u32, 0u32],
        );
        let filled = count.wrapping_mul(0x18).wrapping_add(moved);
        let last = last_of(e, v);
        e.call(
            element.uninit_move,
            &args![position, last, filled, v + 8, 0u32, 0u32],
        );
        let new_size = vector_len(e, v, 0x18).wrapping_add(count);
        if first_of(e, v) != 0 {
            let first = first_of(e, v);
            let last = last_of(e, v);
            e.call(element.destroy_range, &args![this, first, last]);
            let block = first_of(e, v);
            e.call(OPERATOR_DELETE, &args![block]);
        }
        e.mem
            .set_u32(v + 0x14, capacity.wrapping_mul(0x18).wrapping_add(storage));
        e.mem
            .set_u32(v + 0x10, new_size.wrapping_mul(0x18).wrapping_add(storage));
        e.mem.set_u32(v + 0x0C, storage);
    } else if ((last_of(e, v).wrapping_sub(position) as i32 / 0x18) as u32) < count {
        // Fewer vectors follow the position than are inserted.
        e.with_stack(0x18, |e, temp| {
            e.call(element.copy_construct, &args![temp, value]);
            let last = last_of(e, v);
            let target = count.wrapping_mul(0x18).wrapping_add(position);
            e.call(
                element.uninit_move,
                &args![position, last, target, v + 8, 0u32, 0u32],
            );
            let following = (last_of(e, v).wrapping_sub(position) as i32 / 0x18) as u32;
            let surplus = count.wrapping_sub(following);
            let end = last_of(e, v);
            e.call(
                element.uninit_fill,
                &args![end, surplus, temp, v + 8, 0u32, 0u32],
            );
            let new_end = count.wrapping_mul(0x18).wrapping_add(last_of(e, v));
            e.mem.set_u32(v + 0x10, new_end);
            let stop = last_of(e, v).wrapping_sub(count.wrapping_mul(0x18));
            let mut cursor = position;
            while cursor != stop {
                e.call(element.assign, &args![cursor, temp]);
                cursor = cursor.wrapping_add(0x18);
            }
            e.call(element.tidy, &args![temp]);
            e.call(PROXY_RELEASE, &args![temp]);
        });
    } else {
        e.with_stack(0x18, |e, temp| {
            e.call(element.copy_construct, &args![temp, value]);
            let last = last_of(e, v);
            let tail = last.wrapping_sub(count.wrapping_mul(0x18));
            let end = e
                .call(
                    element.uninit_move,
                    &args![tail, last, last_of(e, v), v + 8, 0u32, 0u32],
                )
                .u32();
            e.mem.set_u32(v + 0x10, end);
            e.call(element.move_backward, &args![position, tail, last]);
            let stop = count.wrapping_mul(0x18).wrapping_add(position);
            let mut cursor = position;
            while cursor != stop {
                e.call(element.assign, &args![cursor, temp]);
                cursor = cursor.wrapping_add(0x18);
            }
            e.call(element.tidy, &args![temp]);
            let proxy = e.mem.u32(temp.addr());
            e.call(OPERATOR_DELETE, &args![proxy]);
        });
    }
}

// Translated from 00b33bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b33bd0 - the FS:[0] exception frame (the handlers that free the new
// storage and destroy the copied vectors if a copy throws) is not translated.
/// `_Insert_n` of a vector of 16-bit element vectors (the engine map gives it no name; the
/// first word, the iterator's proxy, is not read): see [`insert_vectors`].
pub fn fn_00b33bd0(
    e: &mut Engine,
    this: Ptr<StlVector>,
    _unused_1: u32,
    position: u32,
    count: u32,
    value: u32,
) {
    insert_vectors(e, &SHORT_VECTORS, this, position, count, value);
}

// Translated from 00b34450 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b34450 - the FS:[0] exception frame (the handlers that free the new
// storage and destroy the copied vectors if a copy throws) is not translated.
/// `_Insert_n` of a vector of 32-bit element vectors (the engine map gives it no name; the
/// first word, the iterator's proxy, is not read): see [`insert_vectors`].
pub fn fn_00b34450(
    e: &mut Engine,
    this: Ptr<StlVector>,
    _unused_1: u32,
    position: u32,
    count: u32,
    value: u32,
) {
    insert_vectors(e, &WORD_VECTORS, this, position, count, value);
}

// Translated from 00b34cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The buy of a 16-bit element vector (the engine map gives it no name): clears the three
/// pointers; with a count of zero answers false; a count above `0x7fffffff` raises the length
/// error (`00b0fb80`); otherwise allocates the storage (`00b35f00`), points the first, last and
/// end at it and answers true.
pub fn fn_00b34cd0(e: &mut Engine, this: Ptr<StlVector>, count: u32) -> bool {
    let v = this.addr();
    clear_pointers(e, v);
    if count == 0 {
        return false;
    }
    let limit = SHORT_LIMIT.max(1);
    if limit < count {
        e.call(LENGTH_ERROR, &args![]);
    } else {
        let storage = fn_00b35f00(e, count, 0);
        e.mem.set_u32(v + 0x0C, storage);
        e.mem.set_u32(v + 0x10, storage);
        e.mem
            .set_u32(v + 0x14, count.wrapping_mul(2).wrapping_add(storage));
    }
    true
}

// Translated from 00b34d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The tidy of a 16-bit element vector (the engine map gives it no name): frees the storage
/// (when there is any) and zeroes the first, last and end pointers.
pub fn fn_00b34d90(e: &mut Engine, this: Ptr<StlVector>) {
    let v = this.addr();
    if first_of(e, v) != 0 {
        let block = first_of(e, v);
        e.call(OPERATOR_DELETE, &args![block]);
    }
    clear_pointers(e, v);
}

// Translated from 00b34e10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b34e10 - the FS:[0] exception frame (the handler that frees the new
// storage if a copy throws) is not translated.
/// `_Insert_n` of a 16-bit element vector (the engine map gives it no name; the first word, the
/// iterator's proxy, is not read): inserts `count` copies of the element at `value` before
/// `position`. A count of zero does nothing; a result above `0x7fffffff` elements raises the
/// length error (`00b0fb80`). Without room the vector grows to one and a half times its
/// capacity (at least the new size, `00b35f00`): the elements before the position are copied to
/// the new storage (`00b36ef0`), the copies stored (`00b36b80`) and the later elements copied
/// after them, the old storage freed. With room, when fewer elements follow the position than
/// are inserted the tail moves up (`00b36ef0`), copies are stored past the old end (`00b35580`)
/// and the rest of the range filled; otherwise the last `count` elements move past the end
/// (`00b36ef0`), the others up (`00b35e60`) and the range is filled.
pub fn fn_00b34e10(
    e: &mut Engine,
    this: Ptr<StlVector>,
    _unused_1: u32,
    position: u32,
    count: u32,
    value: u32,
) {
    let v = this.addr();
    let mut capacity = capacity_of(e, v, 2);
    if count == 0 {
        return;
    }
    // The limit is `max_size`, at least one.
    let limit = SHORT_LIMIT.max(1);
    if limit.wrapping_sub(span(e, v, 0x10, 2)) < count {
        e.call(LENGTH_ERROR, &args![]);
        return;
    }
    if capacity < span(e, v, 0x10, 2).wrapping_add(count) {
        capacity = if limit.wrapping_sub(capacity >> 1) < capacity {
            0
        } else {
            (capacity >> 1).wrapping_add(capacity)
        };
        if capacity < span(e, v, 0x10, 2).wrapping_add(count) {
            capacity = span(e, v, 0x10, 2).wrapping_add(count);
        }
        let storage = fn_00b35f00(e, capacity, 0);
        let first = first_of(e, v);
        let moved = fn_00b36ef0(e, first, position, storage, v + 8);
        fn_00b36b80(e, moved, count, value);
        let filled = moved.wrapping_add(count.wrapping_mul(2));
        let last = last_of(e, v);
        fn_00b36ef0(e, position, last, filled, v + 8);
        let new_size = span(e, v, 0x10, 2).wrapping_add(count);
        if first_of(e, v) != 0 {
            let block = first_of(e, v);
            e.call(OPERATOR_DELETE, &args![block]);
        }
        e.mem
            .set_u32(v + 0x14, storage.wrapping_add(capacity.wrapping_mul(2)));
        e.mem
            .set_u32(v + 0x10, storage.wrapping_add(new_size.wrapping_mul(2)));
        e.mem.set_u32(v + 0x0C, storage);
    } else if ((last_of(e, v).wrapping_sub(position) as i32 >> 1) as u32) < count {
        // Fewer elements follow the position than are inserted.
        let element = e.mem.u16(value);
        let last = last_of(e, v);
        fn_00b36ef0(
            e,
            position,
            last,
            position.wrapping_add(count.wrapping_mul(2)),
            v + 8,
        );
        e.with_stack(4, |e, slot| {
            e.mem.set_u16(slot.addr(), element);
            let end = last_of(e, v);
            let following = (last_of(e, v).wrapping_sub(position) as i32 >> 1) as u32;
            fn_00b35580(e, this, end, count.wrapping_sub(following), slot.addr());
        });
        let new_end = last_of(e, v).wrapping_add(count.wrapping_mul(2));
        e.mem.set_u32(v + 0x10, new_end);
        let stop = last_of(e, v).wrapping_sub(count.wrapping_mul(2));
        let mut cursor = position;
        while cursor != stop {
            e.mem.set_u16(cursor, element);
            cursor = cursor.wrapping_add(2);
        }
    } else {
        let element = e.mem.u16(value);
        let last = last_of(e, v);
        let tail = last.wrapping_sub(count.wrapping_mul(2));
        let end = fn_00b36ef0(e, tail, last, last_of(e, v), v + 8);
        e.mem.set_u32(v + 0x10, end);
        fn_00b35e60(e, position, tail, last);
        let stop = position.wrapping_add(count.wrapping_mul(2));
        let mut cursor = position;
        while cursor != stop {
            e.mem.set_u16(cursor, element);
            cursor = cursor.wrapping_add(2);
        }
    }
}

// Translated from 00b35580 (decompiled, FalloutNV.exe 1.4.0.525)
/// The fill construction of 16-bit elements (`std::_Uninit_fill_n`, as the exe instantiates
/// it; `this`, the empty allocator, is not read): stores `count` copies of the element at
/// `value` at `destination` (`00b36b80`) and answers the address after them.
pub fn fn_00b35580(
    e: &mut Engine,
    _this: Ptr<StlVector>,
    destination: u32,
    count: u32,
    value: u32,
) -> u32 {
    fn_00b36b80(e, destination, count, value);
    destination.wrapping_add(count.wrapping_mul(2))
}

/// The copy constructor of an element vector: constructs the proxy (`00b20ba0`; the word the exe
/// pushes is `ECX`, which still holds `this`: the empty allocator argument, not read), buys the
/// storage for the other vector's element count (`buy`: this, the count; answers a `bool`) and,
/// when it answers true, copies the other vector's elements to it (`copy`: the iterators at the
/// first and last element as proxy and position words, the destination and the allocator at
/// `this + 8`; answers the end) and sets the end there. Returns `this`.
fn copy_construct_elements(
    e: &mut Engine,
    this: Ptr<StlVector>,
    other: Ptr<StlVector>,
    shift: u32,
    buy: u32,
    copy: u32,
) -> Ptr<StlVector> {
    let v = this.addr();
    let o = other.addr();
    e.call(PROXY_CONSTRUCT, &args![this, this]);
    let count = (last_of(e, o).wrapping_sub(first_of(e, o)) as i32 >> shift) as u32;
    if e.call(buy, &args![this, count]).bool() {
        let last = last_of(e, o);
        let last_proxy = checked_iterator(e, o, last);
        let first = first_of(e, o);
        let first_proxy = checked_iterator(e, o, first);
        let destination = first_of(e, v);
        let end = e
            .call(
                copy,
                &args![first_proxy, first, last_proxy, last, destination, v + 8],
            )
            .u32();
        e.mem.set_u32(v + 0x10, end);
    }
    this
}

// Translated from 00b355d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b355d0 - the FS:[0] exception frame is not translated.
/// The copy constructor of a 32-bit element vector (the engine map gives it no name): see
/// [`copy_construct_elements`] (buy `00b17e90`, copy `00b24960`).
pub fn fn_00b355d0(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) -> Ptr<StlVector> {
    copy_construct_elements(e, this, other, 2, WORD_BUY, WORD_UNINIT_COPY_CHECKED)
}

// Translated from 00b357d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b357d0 - the FS:[0] exception frame is not translated.
/// The copy constructor of a 16-bit element vector (the engine map gives it no name): see
/// [`copy_construct_elements`] (buy `00b34cd0`, copy `00b362c0`).
pub fn fn_00b357d0(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) -> Ptr<StlVector> {
    copy_construct_elements(e, this, other, 1, 0x00b3_4cd0, 0x00b3_62c0)
}

// Translated from 00b359d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Advances a checked iterator of a 16-bit element vector (`this` = the iterator: proxy at
/// `+0`, position at `+4`) by `count` elements (the engine map gives it no name): a null proxy
/// takes the invalid-parameter path; so does a new position above the vector's last pointer or
/// (checked separately) below its first. The position is advanced and `this` returned.
pub fn fn_00b359d0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    let it = this.addr();
    if e.mem.u32(it) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    // The container behind the proxy: its first word.
    let container = if e.mem.u32(it) != 0 {
        e.mem.u32(e.mem.u32(it))
    } else {
        0
    };
    let target = e.mem.u32(it + 4).wrapping_add(count.wrapping_mul(2));
    if target > e.mem.u32(container.wrapping_add(0x10)) {
        e.call(INVALID_PARAMETER, &args![]);
    } else {
        let container = if e.mem.u32(it) != 0 {
            e.mem.u32(e.mem.u32(it))
        } else {
            0
        };
        if target < e.mem.u32(container.wrapping_add(0x0C)) {
            e.call(INVALID_PARAMETER, &args![]);
        }
    }
    let position = e.mem.u32(it + 4).wrapping_add(count.wrapping_mul(2));
    e.mem.set_u32(it + 4, position);
    this
}

/// The assigning copy of `[first, last)` of vectors to `dest` (the vectors' assignment `assign`:
/// `this` = the destination, the source): answers the end of the destination, computed from the
/// range's element count (a signed `IDIV`) before the copying.
fn assign_vectors(e: &mut Engine, assign: u32, first: u32, last: u32, dest: u32) -> u32 {
    let end = ((last.wrapping_sub(first) as i32 / 0x18) as u32)
        .wrapping_mul(0x18)
        .wrapping_add(dest);
    let mut source = first;
    let mut target = dest;
    while source != last {
        e.call(assign, &args![target, source]);
        source = source.wrapping_add(0x18);
        target = target.wrapping_add(0x18);
    }
    end
}

// Translated from 00b35a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The assigning copy of the 16-bit element vectors in `[first, last)` to `dest` (the engine
/// map gives it no name): see [`assign_vectors`] (assignment `00b36600`). Answers the end.
pub fn fn_00b35a90(e: &mut Engine, first: u32, last: u32, dest: u32) -> u32 {
    assign_vectors(e, 0x00b3_6600, first, last, dest)
}

// Translated from 00b35b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The assigning copy of the 32-bit element vectors in `[first, last)` to `dest` (the engine
/// map gives it no name): see [`assign_vectors`] (assignment `00b36330`). Answers the end.
pub fn fn_00b35b60(e: &mut Engine, first: u32, last: u32, dest: u32) -> u32 {
    assign_vectors(e, 0x00b3_6330, first, last, dest)
}

// Translated from 00b35c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The copy of the 16-bit elements in `[first, last)` to `dest` (`std::_Copy_opt`, as the exe
/// instantiates it; the engine map gives it no name): with a positive element count (signed),
/// `memmove_s` (`00ec7ce3`); answers `dest` plus the count.
pub fn fn_00b35c30(e: &mut Engine, first: u32, last: u32, dest: u32) -> u32 {
    let count = last.wrapping_sub(first) as i32 >> 1;
    let end = dest.wrapping_add((count as u32).wrapping_mul(2));
    if count > 0 {
        move_bytes(e, dest, first, (count as u32).wrapping_shl(1));
    }
    end
}

// Translated from 00b35ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The fill of 32-bit elements (the engine map gives it no name; the fourth word, the
/// allocator, is not read): `00b36b10` with `first`, `count` and the address of the value.
pub fn fn_00b35ca0(e: &mut Engine, first: u32, count: u32, value: u32, _unused_3: u32) {
    fn_00b36b10(e, first, count, value);
}

/// The backward move of vectors by swapping: walking `last` down to `first` and
/// `destination_end` down with it, the destination vector is swapped with the source one
/// (`swap`: `this` = the destination, the other). Answers the start of the destination range.
fn swap_vectors_backward(
    e: &mut Engine,
    swap: u32,
    first: u32,
    last: u32,
    destination_end: u32,
) -> u32 {
    let start = destination_end
        .wrapping_sub(((last.wrapping_sub(first) as i32 / 0x18) as u32).wrapping_mul(0x18));
    let mut source = last;
    let mut target = destination_end;
    while first != source {
        source = source.wrapping_sub(0x18);
        target = target.wrapping_sub(0x18);
        e.call(swap, &args![target, source]);
    }
    start
}

// Translated from 00b35ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The backward move of the 16-bit element vectors in `[first, last)` to end at
/// `destination_end` (the engine map gives it no name): see [`swap_vectors_backward`] (swap
/// `00b37190`). Answers the start of the destination.
pub fn fn_00b35ce0(e: &mut Engine, first: u32, last: u32, destination_end: u32) -> u32 {
    swap_vectors_backward(e, 0x00b3_7190, first, last, destination_end)
}

// Translated from 00b35da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The backward move of the 32-bit element vectors in `[first, last)` to end at
/// `destination_end` (the engine map gives it no name): see [`swap_vectors_backward`] (swap
/// `00b36f60`). Answers the start of the destination.
pub fn fn_00b35da0(e: &mut Engine, first: u32, last: u32, destination_end: u32) -> u32 {
    swap_vectors_backward(e, 0x00b3_6f60, first, last, destination_end)
}

// Translated from 00b35e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The backward copy of the 16-bit elements in `[first, last)` to end at `destination_end`
/// (`std::_Copy_backward_opt`, as the exe instantiates it; the engine map gives it no name):
/// with a positive element count (signed), `memmove_s` (`00ec7ce3`); answers the start of the
/// destination.
pub fn fn_00b35e60(e: &mut Engine, first: u32, last: u32, destination_end: u32) -> u32 {
    let count = last.wrapping_sub(first) as i32 >> 1;
    let start = destination_end.wrapping_sub((count as u32).wrapping_shl(1));
    if count > 0 {
        move_bytes(e, start, first, (count as u32).wrapping_shl(1));
    }
    start
}

// Translated from 00b35f00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator of 16-bit elements (`std::allocator<unsigned short>::allocate`; the second
/// word is not read): a count of zero allocates zero bytes; a count above `0x7fffffff` throws
/// `std::bad_alloc`; otherwise `operator new(count * 2)`.
pub fn fn_00b35f00(e: &mut Engine, count: u32, _unused_1: u32) -> u32 {
    allocate_elements(e, count, 2)
}

// Translated from 00b35f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The assigning copy of the 16-bit element vectors in `[first, last)` to `dest` (a second
/// instantiation of `00b35a90`; the three words after `dest` are not read): see
/// [`assign_vectors`]. Answers the end.
pub fn fn_00b35f60(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    assign_vectors(e, 0x00b3_6600, first, last, dest)
}

// Translated from 00b36000 (decompiled, FalloutNV.exe 1.4.0.525)
/// The assigning copy of the 32-bit element vectors in `[first, last)` to `dest` (a second
/// instantiation of `00b35b60`; the three words after `dest` are not read): see
/// [`assign_vectors`]. Answers the end.
pub fn fn_00b36000(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    assign_vectors(e, 0x00b3_6330, first, last, dest)
}

/// The fill construction of `count` vectors at `dest` from the vector at `value`: each
/// destination that is not null is copy-constructed (`copy_construct`: `this`, the source).
fn construct_copies(e: &mut Engine, copy_construct: u32, dest: u32, count: u32, value: u32) {
    let mut remaining = count;
    let mut target = dest;
    while remaining != 0 {
        if target != 0 {
            e.call(copy_construct, &args![target, value]);
        }
        target = target.wrapping_add(0x18);
        remaining -= 1;
    }
}

// Translated from 00b360a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b360a0 - the FS:[0] exception frame is not translated.
/// The fill construction of `count` 16-bit element vectors at `dest` from the vector at `value`
/// (`std::_Uninit_fill_n`, as the exe instantiates it; the three words after `value`, the
/// allocator and two bytes, are not read): each non-null destination is copy-constructed
/// (`00b357d0`).
pub fn fn_00b360a0(
    e: &mut Engine,
    dest: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    construct_copies(e, 0x00b3_57d0, dest, count, value);
}

// Translated from 00b361b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b361b0 - the FS:[0] exception frame is not translated.
/// The fill construction of `count` 32-bit element vectors at `dest` from the vector at `value`
/// (a second instantiation of `00b360a0`; the three words after `value` are not read): each
/// non-null destination is copy-constructed (`00b355d0`).
pub fn fn_00b361b0(
    e: &mut Engine,
    dest: u32,
    count: u32,
    value: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    construct_copies(e, 0x00b3_55d0, dest, count, value);
}

// Translated from 00b362c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked uninitialised copy of 16-bit elements (`std::_Uninit_copy`, as the exe
/// instantiates it; the words at 0, 2 and 5, the iterator proxies and the allocator, are not
/// read): with an element count (arithmetic shift) that is not zero, `memmove_s`
/// (`00ec7ce3`) from `first` to `dest`; answers `dest` plus the count.
pub fn fn_00b362c0(
    e: &mut Engine,
    _unused_0: u32,
    first: u32,
    _unused_2: u32,
    last: u32,
    dest: u32,
    _unused_5: u32,
) -> u32 {
    let count = last.wrapping_sub(first) as i32 >> 1;
    let end = dest.wrapping_add((count as u32).wrapping_mul(2));
    if count != 0 {
        move_bytes(e, dest, first, (count as u32).wrapping_shl(1));
    }
    end
}

/// The assignment of an element vector (`operator=`): nothing for itself; an empty source
/// clears the destination (`clear`); a source longer than the destination either needs more
/// room than the capacity (the old storage is freed, `buy` makes the new and `uninit_copy`
/// copies the source into it) or copies the first part over the existing elements (`copy`) and
/// constructs the rest after them (`uninit_copy`); a source no longer than the destination
/// copies over the start and sets the end. `uninit_copy` takes the first and last pointers,
/// the destination and the allocator at `this + 8`. Returns `this`.
#[allow(clippy::too_many_arguments)]
fn assign_elements(
    e: &mut Engine,
    this: Ptr<StlVector>,
    other: Ptr<StlVector>,
    shift: u32,
    clear: u32,
    buy: u32,
    copy: u32,
    uninit_copy: u32,
) -> Ptr<StlVector> {
    let v = this.addr();
    let o = other.addr();
    if v != o {
        let other_size = (last_of(e, o).wrapping_sub(first_of(e, o)) as i32 >> shift) as u32;
        let size = (last_of(e, v).wrapping_sub(first_of(e, v)) as i32 >> shift) as u32;
        if other_size == 0 {
            e.call(clear, &args![this]);
        } else if size < other_size {
            let capacity = if first_of(e, v) == 0 {
                0
            } else {
                (e.mem.u32(v + 0x14).wrapping_sub(first_of(e, v)) as i32 >> shift) as u32
            };
            if capacity < other_size {
                if first_of(e, v) != 0 {
                    let block = first_of(e, v);
                    e.call(OPERATOR_DELETE, &args![block]);
                }
                if e.call(buy, &args![this, other_size]).bool() {
                    let end = e
                        .call(
                            uninit_copy,
                            &args![first_of(e, o), last_of(e, o), first_of(e, v), v + 8],
                        )
                        .u32();
                    e.mem.set_u32(v + 0x10, end);
                }
            } else {
                let middle = first_of(e, o).wrapping_add(
                    (((last_of(e, v).wrapping_sub(first_of(e, v))) as i32 >> shift) as u32)
                        << shift,
                );
                e.call(copy, &args![first_of(e, o), middle, first_of(e, v)]);
                let end = e
                    .call(
                        uninit_copy,
                        &args![middle, last_of(e, o), last_of(e, v), v + 8],
                    )
                    .u32();
                e.mem.set_u32(v + 0x10, end);
            }
        } else {
            e.call(copy, &args![first_of(e, o), last_of(e, o), first_of(e, v)]);
            let end = first_of(e, v).wrapping_add(other_size << shift);
            e.mem.set_u32(v + 0x10, end);
        }
    }
    this
}

// Translated from 00b36330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The assignment of a 32-bit element vector (the engine map gives it no name): see
/// [`assign_elements`] (clear `00b32350`, buy `00b17e90`, copy `00b42e50`, uninitialised copy
/// `00b177e0`). Returns `this`.
pub fn fn_00b36330(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) -> Ptr<StlVector> {
    assign_elements(
        e,
        this,
        other,
        2,
        WORD_CLEAR,
        WORD_BUY,
        WORD_COPY,
        WORD_UNINIT_COPY,
    )
}

// Translated from 00b36600 (decompiled, FalloutNV.exe 1.4.0.525)
/// The assignment of a 16-bit element vector (the engine map gives it no name): see
/// [`assign_elements`] (clear `00b32540`, buy `00b34cd0`, copy `00b35c30`, uninitialised copy
/// `00b36ef0`). Returns `this`.
pub fn fn_00b36600(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) -> Ptr<StlVector> {
    assign_elements(
        e,
        this,
        other,
        1,
        0x00b3_2540,
        0x00b3_4cd0,
        0x00b3_5c30,
        0x00b3_6ef0,
    )
}

/// The copy construction of the vectors in `[first, last)` at `dest` (`copy_construct`: `this`,
/// the source): each non-null destination is constructed, the destination advanced with the
/// source. Answers the final destination.
fn construct_vectors_from(
    e: &mut Engine,
    copy_construct: u32,
    first: u32,
    last: u32,
    dest: u32,
) -> u32 {
    let mut source = first;
    let mut target = dest;
    while source != last {
        if target != 0 {
            e.call(copy_construct, &args![target, source]);
        }
        target = target.wrapping_add(0x18);
        source = source.wrapping_add(0x18);
    }
    target
}

// Translated from 00b368d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b368d0 - the FS:[0] exception frame is not translated.
/// The uninitialised copy of the 16-bit element vectors in `[first, last)` to `dest`
/// (`std::_Uninit_copy`, as the exe instantiates it; the three words after `dest`, the
/// allocator and two bytes, are not read): each non-null destination is copy-constructed
/// (`00b357d0`). Answers the end of the destination.
pub fn fn_00b368d0(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    construct_vectors_from(e, 0x00b3_57d0, first, last, dest)
}

// Translated from 00b369f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b369f0 - the FS:[0] exception frame is not translated.
/// The uninitialised copy of the 32-bit element vectors in `[first, last)` to `dest` (a second
/// instantiation of `00b368d0`; the three words after `dest` are not read): each non-null
/// destination is copy-constructed (`00b355d0`). Answers the end of the destination.
pub fn fn_00b369f0(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    construct_vectors_from(e, 0x00b3_55d0, first, last, dest)
}

// Translated from 00b36b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The fill of 32-bit elements (`std::_Fill_n`, as the exe instantiates it; the engine map
/// gives it no name): stores the element at `value` into `count` consecutive elements from
/// `dest` (the value is read again for every element).
pub fn fn_00b36b10(e: &mut Engine, dest: u32, count: u32, value: u32) {
    let mut remaining = count;
    let mut target = dest;
    while remaining != 0 {
        let element = e.mem.u32(value);
        e.mem.set_u32(target, element);
        remaining -= 1;
        target = target.wrapping_add(4);
    }
}

// Translated from 00b36b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The fill of 16-bit elements (`std::_Fill_n`, as the exe instantiates it; the engine map
/// gives it no name): stores the element at `value` into `count` consecutive elements from
/// `dest` (the value is read again for every element).
pub fn fn_00b36b80(e: &mut Engine, dest: u32, count: u32, value: u32) {
    let mut remaining = count;
    let mut target = dest;
    while remaining != 0 {
        let element = e.mem.u16(value);
        e.mem.set_u16(target, element);
        remaining -= 1;
        target = target.wrapping_add(2);
    }
}

/// The construction of the vectors of `[first, last)` at `dest` by moving: a temporary empty
/// vector is made (`default_construct`); for each element a non-null destination is
/// copy-constructed from the temporary (`copy_construct`) and then swapped with the source
/// vector (`swap`); the temporary is tidied (`tidy`) and its proxy freed. Answers the final
/// destination.
fn move_construct_vectors(
    e: &mut Engine,
    element: &VectorElement,
    first: u32,
    last: u32,
    dest: u32,
) -> u32 {
    e.with_stack(0x18, |e, temp| {
        e.call(element.default_construct, &args![temp]);
        let mut source = first;
        let mut target = dest;
        while source != last {
            if target != 0 {
                e.call(element.copy_construct, &args![target, temp]);
            }
            e.call(element.swap, &args![target, source]);
            target = target.wrapping_add(0x18);
            source = source.wrapping_add(0x18);
        }
        e.call(element.tidy, &args![temp]);
        let proxy = e.mem.u32(temp.addr());
        e.call(OPERATOR_DELETE, &args![proxy]);
        target
    })
}

// Translated from 00b36bf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b36bf0 - the FS:[0] exception frame is not translated.
/// The construction of the 16-bit element vectors of `[first, last)` at `dest` by moving them
/// (`std::_Uninit_move`, as the exe instantiates it; the three words after `dest` are not
/// read): see [`move_construct_vectors`]. Answers the end of the destination.
pub fn fn_00b36bf0(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    move_construct_vectors(e, &SHORT_VECTORS, first, last, dest)
}

// Translated from 00b36d70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b36d70 - the FS:[0] exception frame is not translated.
/// The construction of the 32-bit element vectors of `[first, last)` at `dest` by moving them
/// (a second instantiation of `00b36bf0`; the three words after `dest` are not read): see
/// [`move_construct_vectors`]. Answers the end of the destination.
pub fn fn_00b36d70(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    move_construct_vectors(e, &WORD_VECTORS, first, last, dest)
}

// Translated from 00b36ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The uninitialised copy of 16-bit elements (`std::_Uninit_copy`, as the exe instantiates it;
/// the allocator word is not read): with an element count (arithmetic shift) that is not zero,
/// `memmove_s` (`00ec7ce3`) from `first` to `dest`; answers `dest` plus the count.
pub fn fn_00b36ef0(e: &mut Engine, first: u32, last: u32, dest: u32, _unused_3: u32) -> u32 {
    let count = last.wrapping_sub(first) as i32 >> 1;
    let end = dest.wrapping_add((count as u32).wrapping_mul(2));
    if count != 0 {
        move_bytes(e, dest, first, (count as u32).wrapping_shl(1));
    }
    end
}

/// Whether the vectors' allocators compare equal, which the compiler folded to a constant
/// (`MOV ECX,1; TEST ECX,ECX`): the elements' storage is then exchanged. The other branch (the
/// elements copied through a temporary) never runs, but it is translated.
const ALLOCATORS_EQUAL: bool = true;

/// Swaps the words at `first` and `second` when they are different addresses.
fn swap_words(e: &mut Engine, first: u32, second: u32) {
    if first != second {
        let a = e.mem.u32(first);
        let b = e.mem.u32(second);
        e.mem.set_u32(first, b);
        e.mem.set_u32(second, a);
    }
}

/// The swap of two element vectors: nothing for the same vector. With equal allocators the
/// proxies are swapped (and then the first words of the two proxy blocks, the owners they
/// record), then the first, last and end pointers. Otherwise the elements are exchanged through
/// a temporary: the same two proxy swaps, a copy of `this` is made (`copy_construct`), `this` is
/// assigned `other` and `other` the copy (`assign`), and the temporary is tidied (`tidy`) and
/// its proxy freed.
fn swap_elements(
    e: &mut Engine,
    this: Ptr<StlVector>,
    other: Ptr<StlVector>,
    copy_construct: u32,
    assign: u32,
    tidy: u32,
) {
    let v = this.addr();
    let o = other.addr();
    if v == o {
        return;
    }
    swap_words(e, v, o);
    let theirs = e.mem.u32(o);
    let ours = e.mem.u32(v);
    swap_words(e, ours, theirs);
    if ALLOCATORS_EQUAL {
        for offset in [0x0C, 0x10, 0x14] {
            swap_words(e, v + offset, o + offset);
        }
    } else {
        e.with_stack(0x18, |e, temp| {
            e.call(copy_construct, &args![temp, this]);
            e.call(assign, &args![this, other]);
            e.call(assign, &args![other, temp]);
            e.call(tidy, &args![temp]);
            let proxy = e.mem.u32(temp.addr());
            e.call(OPERATOR_DELETE, &args![proxy]);
        });
    }
}

// Translated from 00b36f60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b36f60 - the FS:[0] exception frame is not translated.
/// `swap` of two 32-bit element vectors (`std::vector::swap`, as the exe instantiates it; the
/// engine map gives it no name): see [`swap_elements`] (copy constructor `00b355d0`, assignment
/// `00b36330`, tidy `00b17f50`).
pub fn fn_00b36f60(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) {
    swap_elements(e, this, other, 0x00b3_55d0, 0x00b3_6330, WORD_TIDY);
}

// Translated from 00b37190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b37190 - the FS:[0] exception frame is not translated.
/// `swap` of two 16-bit element vectors (`std::vector::swap`, as the exe instantiates it; the
/// engine map gives it no name): see [`swap_elements`] (copy constructor `00b357d0`, assignment
/// `00b36600`, tidy `00b34d90`).
pub fn fn_00b37190(e: &mut Engine, this: Ptr<StlVector>, other: Ptr<StlVector>) {
    swap_elements(e, this, other, 0x00b3_57d0, 0x00b3_6600, 0x00b3_4d90);
}

/// The statics and constants of `CWindEngine` (Xbox PDB names, the addresses matched by their
/// uses): `m_fTime` (`011f8d80`, a `float`) and the two `float` constants the leaf angle
/// computations scale by (`010a5784` and `010a5788`).
const WIND_TIME: u32 = 0x011f_8d80;
const LEAF_FREQUENCY_SCALE: u32 = 0x010a_5784;
const LEAF_THROW_SCALE: u32 = 0x010a_5788;
/// The `float` the wind strength starts at (`0101622c`).
const DEFAULT_WIND_STRENGTH: u32 = 0x0101_622c;

// Translated from 00b373c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CWindEngine::CWindEngine` (Xbox PDB name of the class, 0x44 bytes; confirmed by the field
/// offsets of the PDB): `m_fTimeFrequencyShift` 0, `m_fWindStrength` from the constant at
/// `0101622c`, the three wind methods (`m_eBranchMethod`, `m_eFrondMethod`, `m_eLeafMethod`) 2,
/// `m_bRockingLeaves` 1, `m_fLeafFactors` 1.0 and 1.0, `m_fLeafFrequency` and `m_fLeafThrow`
/// 0, `m_nStartingMatrix` 0, `m_nMatrixSpan` 4, `m_uiNumLeafAngles`, `m_pRockingAngles` and
/// `m_pRustleAngles` 0 and the two speed-wind scalars 1.0. Returns `this`.
pub fn fn_00b373c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, 0);
    let strength = x87_quiet_bits(e.mem.u32(DEFAULT_WIND_STRENGTH));
    e.mem.set_u32(base + 0x04, strength);
    e.mem.set_u32(base + 0x08, 2);
    e.mem.set_u32(base + 0x0C, 2);
    e.mem.set_u32(base + 0x10, 2);
    e.mem.set_u8(base + 0x14, 1);
    e.mem.set_u32(base + 0x20, 0);
    e.mem.set_u32(base + 0x24, 0);
    e.mem.set_u32(base + 0x28, 0);
    e.mem.set_u32(base + 0x2C, 4);
    e.mem.set_u32(base + 0x30, 0);
    e.mem.set_u32(base + 0x34, 0);
    e.mem.set_u32(base + 0x38, 0);
    e.mem.set_f32(base + 0x3C, 1.0);
    e.mem.set_f32(base + 0x40, 1.0);
    e.mem.set_f32(base + 0x1C, 1.0);
    e.mem.set_f32(base + 0x18, 1.0);
    this
}

// Translated from 00b37470 (decompiled, FalloutNV.exe 1.4.0.525)
/// A setter of `CWindEngine` (the engine map gives it no name; its first argument, the wind
/// strength, is stored in `m_fWindStrength`; the other two are not named in the PDB): the leaf
/// frequency (`+0x20`) becomes the constant `010a5784` times the strength times the second leaf
/// factor (`+0x1C`), the leaf throw (`+0x24`) the constant `010a5788` times the strength times
/// the first leaf factor (`+0x18`), and the time frequency shift (`+0`) becomes `m_fTime`
/// times (the constant times `second` times the second leaf factor) plus `third`, minus
/// `m_fTime` times the new leaf frequency; the leaf angle count and the rocking angles
/// (`+0x30`, `+0x34`) are cleared. Returns the new shift in `ST0`. The x87 computes in
/// extended precision; each `float` store rounds.
pub fn fn_00b37470(e: &mut Engine, this: Ptr, strength: f32, second: f32, third: f32) -> f32 {
    let base = this.addr();
    let scale = x87_load32(e.mem.u32(LEAF_FREQUENCY_SCALE));
    let leaf_second = x87_m32(e.mem.u32(base + 0x1C));
    let product = x87_mul(x87_mul(scale, x87_m32(second.to_bits())), leaf_second);
    let temporary = x87_store32(product);
    let copied = x87_quiet_bits(strength.to_bits());
    e.mem.set_u32(base + 0x04, copied);
    let scale = x87_load32(e.mem.u32(LEAF_FREQUENCY_SCALE));
    let frequency = x87_mul(
        x87_mul(scale, x87_m32(e.mem.u32(base + 0x04))),
        x87_m32(e.mem.u32(base + 0x1C)),
    );
    e.mem.set_u32(base + 0x20, x87_store32(frequency));
    let scale = x87_load32(e.mem.u32(LEAF_THROW_SCALE));
    let throw = x87_mul(
        x87_mul(scale, x87_m32(e.mem.u32(base + 0x04))),
        x87_m32(e.mem.u32(base + 0x18)),
    );
    e.mem.set_u32(base + 0x24, x87_store32(throw));
    let time = x87_load32(e.mem.u32(WIND_TIME));
    let lead = x87_add(x87_mul(time, x87_m32(temporary)), x87_m32(third.to_bits()));
    let time = x87_load32(e.mem.u32(WIND_TIME));
    let trail = x87_mul(time, x87_m32(e.mem.u32(base + 0x20)));
    e.mem.set_u32(base, x87_store32(x87_sub(lead, trail)));
    e.mem.set_u32(base + 0x34, 0);
    e.mem.set_u32(base + 0x30, 0);
    f32::from_bits(x87_quiet_bits(e.mem.u32(base)))
}

// Translated from 00b37510 (decompiled, FalloutNV.exe 1.4.0.525)
/// A setter of `CWindEngine` taking a wind info (`SIdvWindInfo` of the Xbox PDB: `m_cLeafFactors`
/// at 0, `m_cLeafOscillation` at 0xC, `m_fStrength` at 0x18; the engine map gives the function
/// no name): copies the first two leaf factors into `m_fLeafFactors` (`+0x18`, `+0x1C`), sets
/// the leaf frequency (`+0x20`) to the constant `010a5784` times the info's strength times the
/// new second factor, the leaf throw (`+0x24`) to the constant `010a5788` times the strength
/// times the new first factor, and the wind strength (`+4`) to the info's strength. Floats are
/// copied through the x87 stack and the products rounded at each store.
pub fn fn_00b37510(e: &mut Engine, this: Ptr, info: Ptr) {
    let base = this.addr();
    let source = info.addr();
    let first_factor = x87_quiet_bits(e.mem.u32(source));
    e.mem.set_u32(base + 0x18, first_factor);
    let second_factor = x87_quiet_bits(e.mem.u32(source + 4));
    e.mem.set_u32(base + 0x1C, second_factor);
    let scale = x87_load32(e.mem.u32(LEAF_FREQUENCY_SCALE));
    let frequency = x87_mul(
        x87_mul(scale, x87_m32(e.mem.u32(source + 0x18))),
        x87_m32(e.mem.u32(base + 0x1C)),
    );
    e.mem.set_u32(base + 0x20, x87_store32(frequency));
    let scale = x87_load32(e.mem.u32(LEAF_THROW_SCALE));
    let throw = x87_mul(
        x87_mul(scale, x87_m32(e.mem.u32(source + 0x18))),
        x87_m32(e.mem.u32(base + 0x18)),
    );
    e.mem.set_u32(base + 0x24, x87_store32(throw));
    let strength = x87_quiet_bits(e.mem.u32(source + 0x18));
    e.mem.set_u32(base + 0x04, strength);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b32410, fn_00b32410(Ptr<StlVector>, u32)),
        entry!(0x00b32540, fn_00b32540(Ptr<StlVector>)),
        entry!(0x00b32600, fn_00b32600(Ptr<StlVector>, u32, u32)),
        entry!(
            0x00b326e0,
            fn_00b326e0(Ptr<StlVector>, u32) -> Ptr<StlVector>
        ),
        entry!(0x00b32780, fn_00b32780(Ptr<StlVector>, u32, u32)),
        (0x00b32880, entry_00b32880 as AbiFn), // the by-value vector laid out in an area
        entry!(
            0x00b32ad0,
            fn_00b32ad0(Ptr<StlVector>, Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00b32bb0, fn_00b32bb0(Ptr<StlVector>, u32, u32)),
        entry!(0x00b32c60, fn_00b32c60(Ptr<StlVector>)),
        entry!(0x00b32ce0, fn_00b32ce0(Ptr<StlVector>, u32, u32)),
        (0x00b32de0, entry_00b32de0 as AbiFn), // the by-value vector laid out in an area
        entry!(
            0x00b33030,
            fn_00b33030(Ptr<StlVector>, Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00b33110, fn_00b33110(Ptr<StlVector>, u32, u32)),
        entry!(0x00b331c0, fn_00b331c0(Ptr<StlVector>)),
        entry!(
            0x00b33240,
            fn_00b33240(Ptr<StlVector>, Ptr, u32, u32, u32) -> Ptr
        ),
        entry!(0x00b33370, fn_00b33370(Ptr<StlVector>)),
        entry!(0x00b333e0, fn_00b333e0(Ptr<StlVector>, u32, u32)),
        entry!(
            0x00b335c0,
            fn_00b335c0(Ptr<StlVector>, Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00b336a0, fn_00b336a0(Ptr<StlVector>) -> Ptr<StlVector>),
        entry!(0x00b33780, fn_00b33780(Ptr<StlVector>)),
        entry!(0x00b337f0, fn_00b337f0(Ptr<StlVector>, u32, u32)),
        entry!(
            0x00b339d0,
            fn_00b339d0(Ptr<StlVector>, Ptr, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x00b33af0,
            fn_00b33af0(Ptr<StlVector>, Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00b33bd0, fn_00b33bd0(Ptr<StlVector>, u32, u32, u32, u32)),
        entry!(0x00b34450, fn_00b34450(Ptr<StlVector>, u32, u32, u32, u32)),
        entry!(0x00b34cd0, fn_00b34cd0(Ptr<StlVector>, u32) -> bool),
        entry!(0x00b34d90, fn_00b34d90(Ptr<StlVector>)),
        entry!(0x00b34e10, fn_00b34e10(Ptr<StlVector>, u32, u32, u32, u32)),
        entry!(
            0x00b35580,
            fn_00b35580(Ptr<StlVector>, u32, u32, u32) -> u32
        ),
        entry!(
            0x00b355d0,
            fn_00b355d0(Ptr<StlVector>, Ptr<StlVector>) -> Ptr<StlVector>
        ),
        entry!(
            0x00b357d0,
            fn_00b357d0(Ptr<StlVector>, Ptr<StlVector>) -> Ptr<StlVector>
        ),
        entry!(0x00b359d0, fn_00b359d0(Ptr, u32) -> Ptr),
        entry!(0x00b35a90, fn_00b35a90(u32, u32, u32) -> u32),
        entry!(0x00b35b60, fn_00b35b60(u32, u32, u32) -> u32),
        entry!(0x00b35c30, fn_00b35c30(u32, u32, u32) -> u32),
        entry!(0x00b35ca0, fn_00b35ca0(u32, u32, u32, u32)),
        entry!(0x00b35ce0, fn_00b35ce0(u32, u32, u32) -> u32),
        entry!(0x00b35da0, fn_00b35da0(u32, u32, u32) -> u32),
        entry!(0x00b35e60, fn_00b35e60(u32, u32, u32) -> u32),
        entry!(0x00b35f00, fn_00b35f00(u32, u32) -> u32),
        entry!(
            0x00b35f60,
            fn_00b35f60(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00b36000,
            fn_00b36000(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00b360a0, fn_00b360a0(u32, u32, u32, u32, u32, u32)),
        entry!(0x00b361b0, fn_00b361b0(u32, u32, u32, u32, u32, u32)),
        entry!(
            0x00b362c0,
            fn_00b362c0(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00b36330,
            fn_00b36330(Ptr<StlVector>, Ptr<StlVector>) -> Ptr<StlVector>
        ),
        entry!(
            0x00b36600,
            fn_00b36600(Ptr<StlVector>, Ptr<StlVector>) -> Ptr<StlVector>
        ),
        entry!(
            0x00b368d0,
            fn_00b368d0(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00b369f0,
            fn_00b369f0(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00b36b10, fn_00b36b10(u32, u32, u32)),
        entry!(0x00b36b80, fn_00b36b80(u32, u32, u32)),
        entry!(
            0x00b36bf0,
            fn_00b36bf0(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00b36d70,
            fn_00b36d70(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00b36ef0, fn_00b36ef0(u32, u32, u32, u32) -> u32),
        entry!(0x00b36f60, fn_00b36f60(Ptr<StlVector>, Ptr<StlVector>)),
        entry!(0x00b37190, fn_00b37190(Ptr<StlVector>, Ptr<StlVector>)),
        entry!(0x00b373c0, fn_00b373c0(Ptr) -> Ptr),
        entry!(0x00b37470, fn_00b37470(Ptr, f32, f32, f32) -> f32),
        entry!(0x00b37510, fn_00b37510(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Rc<RefCell<Vec<Vec<u32>>>>;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// The doubled iterator advance: the position moves by `count` elements of `size` bytes.
    fn advance(e: &mut Engine, a: &[u32], size: u32) -> Ret {
        let position = e.mem.u32(a[0] + 4).wrapping_add(a[1].wrapping_mul(size));
        e.mem.set_u32(a[0] + 4, position);
        ret(a[0])
    }

    /// An engine with the exe's data range mapped and every callee outside this file doubled:
    /// the allocator, `memmove_s`, the iterator helpers, the proxy helpers and the throws.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(MEMMOVE_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(ALLOCATE_PROXY, |e, a| ret(e.mem.alloc(4 * a[0])));
        e.register(ALLOCATE_VECTORS, |e, a| ret(e.mem.alloc(a[0] * 0x18)));
        e.register(ITERATORS_DIFFER, |e, a| {
            let differ =
                e.mem.u32(a[0]) != e.mem.u32(a[1]) || e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4);
            ret(u32::from(differ))
        });
        e.register(ADVANCE_VECTOR_ITERATOR, |e, a| advance(e, a, 0x18));
        e.register(ADVANCE_WORD_ITERATOR, |e, a| advance(e, a, 4));
        e.register(ADVANCE_WORD_ITERATOR_INSERT, |e, a| advance(e, a, 4));
        e.register(PROXY_RELEASE, |e, a| {
            let block = e.mem.u32(a[0]);
            e.mem.free(block);
            Ret::default()
        });
        e.register(PROXY_CONSTRUCT, |e, a| {
            let proxy = e.mem.alloc(4);
            e.mem.set_u32(proxy, a[0]);
            e.mem.set_u32(a[0], proxy);
            ret(a[0])
        });
        e.register(VECTORS_BUY, |e, a| {
            clear_pointers(e, a[0]);
            if a[1] == 0 {
                return ret(0);
            }
            let storage = e.mem.alloc(a[1] * 0x18);
            e.mem.set_u32(a[0] + 0x0C, storage);
            e.mem.set_u32(a[0] + 0x10, storage);
            e.mem.set_u32(a[0] + 0x14, storage + a[1] * 0x18);
            ret(1)
        });
        e.register(WORD_COPY, |e, a| {
            let bytes = e.mem.bytes(a[0], a[1].wrapping_sub(a[0]));
            e.mem.write(a[2], &bytes);
            ret(a[2].wrapping_add(bytes.len() as u32))
        });
        e.register(WORD_UNINIT_COPY_CHECKED, |e, a| {
            let bytes = e.mem.bytes(a[1], a[3].wrapping_sub(a[1]));
            e.mem.write(a[4], &bytes);
            ret(a[4].wrapping_add(bytes.len() as u32))
        });
        e.register(LENGTH_ERROR, |_, _| -> Ret { panic!("vector too long") });
        e.register(EXCEPTION_FROM_MESSAGE, |_, a| ret(a[0]));
        e.register(CXX_THROW_EXCEPTION, |_, a| -> Ret {
            panic!("thrown {:#x}", a[1])
        });
        e
    }

    /// A double that records the argument words of every call and answers 0.
    fn recording(e: &mut Engine, address: u32) -> Calls {
        let log: Calls = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(address, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        log
    }

    /// Gives the vector object at `v` a proxy and storage holding `bytes` (elements of `size`
    /// bytes) with room for `spare` more elements; no storage at all for an empty, spare-less one.
    fn fill(e: &mut Engine, v: u32, bytes: &[u8], size: u32, spare: u32) {
        let proxy = e.mem.alloc(4);
        e.mem.set_u32(proxy, v);
        e.mem.set_u32(v, proxy);
        let count = bytes.len() as u32 / size;
        if count + spare > 0 {
            let storage = e.mem.alloc((count + spare) * size);
            e.mem.write(storage, bytes);
            e.mem.set_u32(v + 0x0C, storage);
            e.mem.set_u32(v + 0x10, storage + count * size);
            e.mem.set_u32(v + 0x14, storage + (count + spare) * size);
        }
    }

    fn element_bytes(values: &[u32], size: u32) -> Vec<u8> {
        values
            .iter()
            .flat_map(|value| value.to_le_bytes()[..size as usize].to_vec())
            .collect()
    }

    /// A vector object (proxy, no storage).
    fn bare(e: &mut Engine) -> u32 {
        let v = e.mem.alloc(0x18);
        fill(e, v, &[], 4, 0);
        v
    }

    fn vector(e: &mut Engine, values: &[u32], size: u32, spare: u32) -> u32 {
        let v = e.mem.alloc(0x18);
        fill(e, v, &element_bytes(values, size), size, spare);
        v
    }

    /// A vector of vectors (each row a vector of `size`-byte elements), with room for `spare`
    /// more rows; the inner vectors sit in the outer storage.
    fn nested(e: &mut Engine, rows: &[&[u32]], size: u32, spare: u32) -> u32 {
        let v = bare(e);
        let count = rows.len() as u32;
        let storage = e.mem.alloc((count + spare).max(1) * 0x18);
        for (i, row) in rows.iter().enumerate() {
            fill(
                e,
                storage + 0x18 * i as u32,
                &element_bytes(row, size),
                size,
                0,
            );
        }
        e.mem.set_u32(v + 0x0C, storage);
        e.mem.set_u32(v + 0x10, storage + 0x18 * count);
        e.mem.set_u32(v + 0x14, storage + 0x18 * (count + spare));
        v
    }

    /// The elements of the vector at `v` (`size` bytes each).
    fn elements(e: &Engine, v: u32, size: u32) -> Vec<u32> {
        let (first, last) = (first_of(e, v), last_of(e, v));
        (0..last.wrapping_sub(first) / size)
            .map(|i| {
                if size == 2 {
                    u32::from(e.mem.u16(first + 2 * i))
                } else {
                    e.mem.u32(first + 4 * i)
                }
            })
            .collect()
    }

    /// The rows of a vector of vectors.
    fn rows_of(e: &Engine, v: u32, size: u32) -> Vec<Vec<u32>> {
        let (first, last) = (first_of(e, v), last_of(e, v));
        (0..last.wrapping_sub(first) / 0x18)
            .map(|i| elements(e, first + 0x18 * i, size))
            .collect()
    }

    /// The six words of a vector object, as a by-value argument.
    fn words6(e: &Engine, v: u32) -> [u32; 6] {
        let mut out = [0u32; 6];
        for (i, word) in out.iter_mut().enumerate() {
            *word = e.mem.u32(v + 4 * i as u32);
        }
        out
    }

    /// A block holding one 16-bit value (the argument of the insertions is its address).
    fn value16(e: &mut Engine, value: u16) -> u32 {
        let at = e.mem.alloc(4);
        e.mem.set_u16(at, value);
        at
    }

    fn value32(e: &mut Engine, value: u32) -> u32 {
        let at = e.mem.alloc(4);
        e.mem.set_u32(at, value);
        at
    }

    // ---- vectors of 16-bit elements -------------------------------------------------------

    #[test]
    fn push_back_stores_with_room_and_grows_without() {
        let mut e = engine();
        let v = vector(&mut e, &[1, 2], 2, 1);
        let value = value16(&mut e, 7);
        e.call(0x00b3_2410, &args![v, value]);
        assert_eq!(elements(&e, v, 2), vec![1, 2, 7]);
        let value = value16(&mut e, 9);
        e.call(0x00b3_2410, &args![v, value]);
        assert_eq!(elements(&e, v, 2), vec![1, 2, 7, 9]);
    }

    #[test]
    fn clear_erases_the_whole_range() {
        let mut e = engine();
        let v = vector(&mut e, &[1, 2, 3], 2, 0);
        e.call(0x00b3_2540, &args![v]);
        assert_eq!(elements(&e, v, 2), Vec::<u32>::new());
    }

    #[test]
    fn the_short_vector_default_constructor_makes_a_proxy_and_an_empty_vector() {
        let mut e = engine();
        let v = e.mem.alloc(0x18);
        assert_eq!(e.call(0x00b3_36a0, &args![v]).u32(), v);
        let proxy = e.mem.u32(v);
        assert_ne!(proxy, 0);
        assert_eq!(e.mem.u32(proxy), v);
        assert_eq!(
            (first_of(&e, v), last_of(&e, v), e.mem.u32(v + 0x14)),
            (0, 0, 0)
        );
    }

    #[test]
    fn the_short_vector_destructor_frees_the_storage_and_the_proxy() {
        let mut e = engine();
        let v = vector(&mut e, &[1, 2], 2, 0);
        let (proxy, storage) = (e.mem.u32(v), first_of(&e, v));
        e.call(0x00b3_3780, &args![v]);
        assert_eq!(first_of(&e, v), 0);
        assert!(e.mem.block_size(storage).is_none());
        assert!(e.mem.block_size(proxy).is_none());
    }

    #[test]
    fn short_resize_inserts_the_missing_elements_or_erases_the_surplus() {
        let mut e = engine();
        let v = vector(&mut e, &[1, 2], 2, 0);
        e.call(0x00b3_37f0, &args![v, 4u32, 7u32]);
        assert_eq!(elements(&e, v, 2), vec![1, 2, 7, 7]);
        let w = vector(&mut e, &[1, 2, 3, 4], 2, 0);
        e.call(0x00b3_37f0, &args![w, 2u32, 7u32]);
        assert_eq!(elements(&e, w, 2), vec![1, 2]);
    }

    #[test]
    fn short_insert_one_returns_the_iterator_at_the_index() {
        let mut e = engine();
        let v = vector(&mut e, &[1, 3], 2, 1);
        let (proxy, first) = (e.mem.u32(v), first_of(&e, v));
        let value = value16(&mut e, 2);
        let out = e.mem.alloc(8);
        let result = e
            .call(0x00b3_39d0, &args![v, out, proxy, first + 2, value])
            .u32();
        assert_eq!(result, out);
        assert_eq!(elements(&e, v, 2), vec![1, 2, 3]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (proxy, first + 2));
    }

    #[test]
    fn short_erase_copies_the_tail_down() {
        let mut e = engine();
        let v = vector(&mut e, &[1, 2, 3, 4], 2, 0);
        let (proxy, first) = (e.mem.u32(v), first_of(&e, v));
        let out = e.mem.alloc(8);
        let result = e
            .call(
                0x00b3_3af0,
                &args![v, out, proxy, first + 2, proxy, first + 6],
            )
            .u32();
        assert_eq!(result, out);
        assert_eq!(elements(&e, v, 2), vec![1, 4]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (proxy, first + 2));
    }

    #[test]
    fn the_short_buy_answers_false_for_zero_and_allocates_otherwise() {
        let mut e = engine();
        let v = e.mem.alloc(0x18);
        assert!(!e.call(0x00b3_4cd0, &args![v, 0u32]).bool());
        assert!(e.call(0x00b3_4cd0, &args![v, 3u32]).bool());
        let first = first_of(&e, v);
        assert_ne!(first, 0);
        assert_eq!(last_of(&e, v), first);
        assert_eq!(e.mem.u32(v + 0x14), first + 6);
        let log = recording(&mut e, LENGTH_ERROR);
        assert!(e.call(0x00b3_4cd0, &args![v, 0x8000_0000u32]).bool());
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(first_of(&e, v), 0);
    }

    #[test]
    fn the_short_tidy_frees_the_storage_and_clears_the_pointers() {
        let mut e = engine();
        let v = vector(&mut e, &[1, 2], 2, 1);
        let storage = first_of(&e, v);
        e.call(0x00b3_4d90, &args![v]);
        assert_eq!(
            (first_of(&e, v), last_of(&e, v), e.mem.u32(v + 0x14)),
            (0, 0, 0)
        );
        assert!(e.mem.block_size(storage).is_none());
        e.call(0x00b3_4d90, &args![v]);
    }

    #[test]
    fn short_insert_n_grows_or_moves_the_tail() {
        // Without room: the vector grows and the copies go between the halves.
        let mut e = engine();
        let v = vector(&mut e, &[1, 2], 2, 0);
        let (proxy, first) = (e.mem.u32(v), first_of(&e, v));
        let value = value16(&mut e, 5);
        e.call(0x00b3_4e10, &args![v, proxy, first + 2, 2u32, value]);
        assert_eq!(elements(&e, v, 2), vec![1, 5, 5, 2]);
        // Fewer elements follow the position than are inserted.
        let v = vector(&mut e, &[1, 2], 2, 3);
        let (proxy, first) = (e.mem.u32(v), first_of(&e, v));
        let value = value16(&mut e, 6);
        e.call(0x00b3_4e10, &args![v, proxy, first + 2, 3u32, value]);
        assert_eq!(elements(&e, v, 2), vec![1, 6, 6, 6, 2]);
        // At least as many follow as are inserted.
        let v = vector(&mut e, &[1, 2, 3], 2, 2);
        let (proxy, first) = (e.mem.u32(v), first_of(&e, v));
        let value = value16(&mut e, 8);
        e.call(0x00b3_4e10, &args![v, proxy, first + 2, 1u32, value]);
        assert_eq!(elements(&e, v, 2), vec![1, 8, 2, 3]);
        // A count of zero does nothing.
        e.call(0x00b3_4e10, &args![v, proxy, first + 2, 0u32, value]);
        assert_eq!(elements(&e, v, 2), vec![1, 8, 2, 3]);
    }

    #[test]
    fn short_insert_n_raises_the_length_error_for_too_many() {
        let mut e = engine();
        let log = recording(&mut e, LENGTH_ERROR);
        let v = vector(&mut e, &[1, 2], 2, 0);
        let (proxy, first) = (e.mem.u32(v), first_of(&e, v));
        let value = value16(&mut e, 5);
        e.call(0x00b3_4e10, &args![v, proxy, first, 0x7fff_ffffu32, value]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(elements(&e, v, 2), vec![1, 2]);
    }

    #[test]
    fn the_short_fill_construction_answers_the_end() {
        let mut e = engine();
        let v = bare(&mut e);
        let destination = e.mem.alloc(8);
        let value = value16(&mut e, 5);
        let end = e
            .call(0x00b3_5580, &args![v, destination, 3u32, value])
            .u32();
        assert_eq!(end, destination + 6);
        assert_eq!(
            (
                e.mem.u16(destination),
                e.mem.u16(destination + 2),
                e.mem.u16(destination + 4)
            ),
            (5, 5, 5)
        );
    }

    #[test]
    fn the_short_copy_constructor_copies_the_elements() {
        let mut e = engine();
        let source = vector(&mut e, &[4, 5, 6], 2, 0);
        let this = e.mem.alloc(0x18);
        assert_eq!(e.call(0x00b3_57d0, &args![this, source]).u32(), this);
        assert_eq!(elements(&e, this, 2), vec![4, 5, 6]);
        let proxy = e.mem.u32(this);
        assert_eq!(e.mem.u32(proxy), this);
    }

    #[test]
    fn the_short_iterator_advance_moves_the_position() {
        let mut e = engine();
        let v = vector(&mut e, &[1, 2, 3, 4], 2, 0);
        let (proxy, first) = (e.mem.u32(v), first_of(&e, v));
        let iterator = e.mem.alloc(8);
        e.mem.set_u32(iterator, proxy);
        e.mem.set_u32(iterator + 4, first);
        assert_eq!(e.call(0x00b3_59d0, &args![iterator, 2u32]).u32(), iterator);
        assert_eq!(e.mem.u32(iterator + 4), first + 4);
    }

    #[test]
    #[should_panic(expected = "CRT invalid parameter")]
    fn the_short_iterator_advance_checks_the_end() {
        let mut e = engine();
        // `_invalid_parameter_noinfo` reports through 00ec7c30, which ends the process in
        // the exe (Watson); the double stops the test there.
        e.register(0x00ec_7c30, |_, _| panic!("CRT invalid parameter"));
        let v = vector(&mut e, &[1, 2, 3, 4], 2, 0);
        let (proxy, first) = (e.mem.u32(v), first_of(&e, v));
        let iterator = e.mem.alloc(8);
        e.mem.set_u32(iterator, proxy);
        e.mem.set_u32(iterator + 4, first);
        e.call(0x00b3_59d0, &args![iterator, 5u32]);
    }

    #[test]
    fn the_short_copy_answers_the_end_and_skips_empty_and_negative_ranges() {
        let mut e = engine();
        let source = e.mem.alloc(8);
        for i in 0..3 {
            e.mem.set_u16(source + 2 * i, i as u16 + 1);
        }
        let destination = e.mem.alloc(8);
        let end = e
            .call(0x00b3_5c30, &args![source, source + 6, destination])
            .u32();
        assert_eq!(end, destination + 6);
        assert_eq!(e.mem.u16(destination + 4), 3);
        let log = recording(&mut e, MEMMOVE_S);
        assert_eq!(
            e.call(0x00b3_5c30, &args![source, source, destination])
                .u32(),
            destination
        );
        e.call(0x00b3_5c30, &args![source + 6, source, destination]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn the_short_backward_copy_answers_the_start() {
        let mut e = engine();
        let block = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_u16(block + 2 * i, i as u16 + 1);
        }
        let start = e
            .call(0x00b3_5e60, &args![block, block + 6, block + 8])
            .u32();
        assert_eq!(start, block + 2);
        let moved: Vec<u16> = (0..4).map(|i| e.mem.u16(block + 2 * i)).collect();
        assert_eq!(moved, vec![1, 1, 2, 3]);
    }

    #[test]
    fn the_short_allocator_allocates_and_throws_for_a_huge_count() {
        let mut e = engine();
        let block = e.call(0x00b3_5f00, &args![4u32, 0u32]).u32();
        assert_eq!(e.mem.block_size(block), Some(8));
        assert_ne!(e.call(0x00b3_5f00, &args![0u32, 0u32]).u32(), 0);
    }

    #[test]
    #[should_panic(expected = "thrown 0x1140ef8")]
    fn the_short_allocator_throws_bad_alloc_when_the_size_overflows() {
        let mut e = engine();
        e.call(0x00b3_5f00, &args![0x8000_0001u32, 0u32]);
    }

    #[test]
    fn the_checked_short_copy_skips_an_empty_range() {
        let mut e = engine();
        let source = e.mem.alloc(8);
        e.mem.set_u16(source, 9);
        e.mem.set_u16(source + 2, 8);
        let destination = e.mem.alloc(8);
        let end = e
            .call(
                0x00b3_62c0,
                &args![0u32, source, 0u32, source + 4, destination, 0u32],
            )
            .u32();
        assert_eq!(end, destination + 4);
        assert_eq!(e.mem.u16(destination + 2), 8);
        let log = recording(&mut e, MEMMOVE_S);
        e.call(
            0x00b3_62c0,
            &args![0u32, source, 0u32, source, destination, 0u32],
        );
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn the_short_unchecked_copy_skips_an_empty_range() {
        let mut e = engine();
        let source = e.mem.alloc(8);
        e.mem.set_u16(source, 3);
        let destination = e.mem.alloc(8);
        assert_eq!(
            e.call(0x00b3_6ef0, &args![source, source + 2, destination, 0u32])
                .u32(),
            destination + 2
        );
        assert_eq!(e.mem.u16(destination), 3);
        let log = recording(&mut e, MEMMOVE_S);
        e.call(0x00b3_6ef0, &args![source, source, destination, 0u32]);
        assert!(log.borrow().is_empty());
    }

    #[test]
    fn the_short_fill_stores_the_value_count_times() {
        let mut e = engine();
        let destination = e.mem.alloc(8);
        let value = value16(&mut e, 0x1234);
        e.call(0x00b3_6b80, &args![destination, 3u32, value]);
        assert_eq!(
            (
                e.mem.u16(destination),
                e.mem.u16(destination + 2),
                e.mem.u16(destination + 4),
                e.mem.u16(destination + 6)
            ),
            (0x1234, 0x1234, 0x1234, 0)
        );
        e.call(0x00b3_6b80, &args![destination, 0u32, value]);
    }

    /// The assignment of an element vector (`address`, elements of `size` bytes).
    fn check_assignment(address: u32, size: u32) {
        let mut e = engine();
        // Itself: nothing changes.
        let a = vector(&mut e, &[1, 2], size, 0);
        assert_eq!(e.call(address, &args![a, a]).u32(), a);
        assert_eq!(elements(&e, a, size), vec![1, 2]);
        // An empty source clears the destination.
        let empty = bare(&mut e);
        e.call(address, &args![a, empty]);
        assert_eq!(elements(&e, a, size), Vec::<u32>::new());
        // A longer source than the capacity: new storage.
        let b = vector(&mut e, &[1], size, 0);
        let long = vector(&mut e, &[7, 8, 9], size, 0);
        e.call(address, &args![b, long]);
        assert_eq!(elements(&e, b, size), vec![7, 8, 9]);
        // A longer source that fits the capacity: the first part is assigned over, the rest
        // constructed after it.
        let c = vector(&mut e, &[1], size, 3);
        e.call(address, &args![c, long]);
        assert_eq!(elements(&e, c, size), vec![7, 8, 9]);
        // A shorter source: copied over the start.
        let d = vector(&mut e, &[1, 2, 3], size, 0);
        let short = vector(&mut e, &[5], size, 0);
        e.call(address, &args![d, short]);
        assert_eq!(elements(&e, d, size), vec![5]);
    }

    #[test]
    fn the_short_vector_assignment_covers_every_case() {
        check_assignment(0x00b3_6600, 2);
    }

    #[test]
    fn the_word_vector_assignment_covers_every_case() {
        check_assignment(0x00b3_6330, 4);
    }

    /// The swap of two element vectors (`address`, `size`-byte elements).
    fn check_swap(address: u32, size: u32) {
        let mut e = engine();
        let a = vector(&mut e, &[1, 2], size, 0);
        let b = vector(&mut e, &[3, 4, 5], size, 1);
        e.call(address, &args![a, b]);
        assert_eq!(elements(&e, a, size), vec![3, 4, 5]);
        assert_eq!(elements(&e, b, size), vec![1, 2]);
        for v in [a, b] {
            let proxy = e.mem.u32(v);
            assert_eq!(e.mem.u32(proxy), v);
        }
        e.call(address, &args![a, a]);
        assert_eq!(elements(&e, a, size), vec![3, 4, 5]);
    }

    #[test]
    fn the_short_vector_swap_exchanges_storage_and_proxies() {
        check_swap(0x00b3_7190, 2);
    }

    #[test]
    fn the_word_vector_swap_exchanges_storage_and_proxies() {
        check_swap(0x00b3_6f60, 4);
    }

    // ---- vectors of 32-bit elements -------------------------------------------------------

    #[test]
    fn the_word_constructor_fills_the_new_storage() {
        let mut e = engine();
        let v = bare(&mut e);
        let value = value32(&mut e, 0x55);
        e.call(0x00b3_2600, &args![v, 3u32, value]);
        assert_eq!(elements(&e, v, 4), vec![0x55; 3]);
        let w = bare(&mut e);
        e.call(0x00b3_2600, &args![w, 0u32, value]);
        assert_eq!(last_of(&e, w), 0);
    }

    #[test]
    fn the_proxy_constructor_allocates_a_proxy_pointing_back() {
        let mut e = engine();
        let v = e.mem.alloc(0x18);
        assert_eq!(e.call(0x00b3_26e0, &args![v, 0u32]).u32(), v);
        let proxy = e.mem.u32(v);
        assert_ne!(proxy, 0);
        assert_eq!(e.mem.u32(proxy), v);
        e.register(ALLOCATE_PROXY, |_, _| Ret::default());
        let w = e.mem.alloc(0x18);
        e.call(0x00b3_26e0, &args![w, 0u32]);
        assert_eq!(e.mem.u32(w), 0);
    }

    #[test]
    fn the_word_insert_one_passes_the_value_and_advances_the_iterator() {
        let mut e = engine();
        let log = recording(&mut e, WORD_INSERT_N_ONE);
        let v = vector(&mut e, &[1, 2, 3], 4, 1);
        let (proxy, first) = (e.mem.u32(v), first_of(&e, v));
        let out = e.mem.alloc(8);
        let result = e
            .call(0x00b3_3240, &args![v, out, proxy, first + 4, 0xabcdu32])
            .u32();
        assert_eq!(result, out);
        assert_eq!(*log.borrow(), vec![vec![v, proxy, first + 4, 1, 0xabcd]]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (proxy, first + 4));
        // An empty vector: the index is zero.
        let empty = bare(&mut e);
        let empty_proxy = e.mem.u32(empty);
        e.call(0x00b3_3240, &args![empty, out, empty_proxy, 0u32, 0x1u32]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (empty_proxy, 0));
    }

    #[test]
    fn the_word_destructor_frees_the_storage_and_the_proxy() {
        let mut e = engine();
        let v = vector(&mut e, &[1, 2], 4, 0);
        let (proxy, storage) = (e.mem.u32(v), first_of(&e, v));
        e.call(0x00b3_3370, &args![v]);
        assert_eq!(first_of(&e, v), 0);
        assert!(e.mem.block_size(storage).is_none());
        assert!(e.mem.block_size(proxy).is_none());
    }

    #[test]
    fn word_resize_inserts_through_the_word_insert_or_erases() {
        let mut e = engine();
        let seen = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(WORD_INSERT_N, move |e, a| {
            sink.borrow_mut().push((a.to_vec(), e.mem.u32(a[4])));
            Ret::default()
        });
        let v = vector(&mut e, &[1, 2], 4, 0);
        let (proxy, last) = (e.mem.u32(v), last_of(&e, v));
        e.call(0x00b3_33e0, &args![v, 5u32, 0x77u32]);
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        assert_eq!(seen[0].0[..4], [v, proxy, last, 3]);
        assert_eq!(seen[0].1, 0x77);
        let w = vector(&mut e, &[1, 2, 3, 4], 4, 0);
        e.call(0x00b3_33e0, &args![w, 2u32, 0x77u32]);
        assert_eq!(elements(&e, w, 4), vec![1, 2]);
    }

    #[test]
    fn the_word_erase_copies_the_tail_down() {
        let mut e = engine();
        let v = vector(&mut e, &[1, 2, 3, 4], 4, 0);
        let (proxy, first) = (e.mem.u32(v), first_of(&e, v));
        let out = e.mem.alloc(8);
        let result = e
            .call(
                0x00b3_35c0,
                &args![v, out, proxy, first + 4, proxy, first + 12],
            )
            .u32();
        assert_eq!(result, out);
        assert_eq!(elements(&e, v, 4), vec![1, 4]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (proxy, first + 4));
    }

    #[test]
    fn the_word_copy_constructor_copies_the_elements() {
        let mut e = engine();
        let source = vector(&mut e, &[4, 5, 6], 4, 0);
        let this = e.mem.alloc(0x18);
        assert_eq!(e.call(0x00b3_55d0, &args![this, source]).u32(), this);
        assert_eq!(elements(&e, this, 4), vec![4, 5, 6]);
    }

    #[test]
    fn the_word_fill_wrapper_fills_the_elements() {
        let mut e = engine();
        let destination = e.mem.alloc(16);
        let value = value32(&mut e, 0xfeed);
        e.call(0x00b3_5ca0, &args![destination, 3u32, value, 0u32]);
        assert_eq!(
            (
                e.mem.u32(destination),
                e.mem.u32(destination + 8),
                e.mem.u32(destination + 12)
            ),
            (0xfeed, 0xfeed, 0)
        );
    }

    #[test]
    fn the_word_fill_stores_the_value_count_times() {
        let mut e = engine();
        let destination = e.mem.alloc(16);
        let value = value32(&mut e, 0x1234_5678);
        e.call(0x00b3_6b10, &args![destination, 3u32, value]);
        assert_eq!(e.mem.u32(destination + 8), 0x1234_5678);
        assert_eq!(e.mem.u32(destination + 12), 0);
        e.call(0x00b3_6b10, &args![destination, 0u32, value]);
    }

    // ---- vectors of vectors ---------------------------------------------------------------

    /// Checks the constructor of a vector of vectors (`address`, `size`-byte elements).
    fn check_vectors_constructor(address: u32, size: u32) {
        let mut e = engine();
        let source = vector(&mut e, &[4, 5, 6], size, 0);
        let v = bare(&mut e);
        e.call(address, &args![v, 2u32, source]);
        assert_eq!(rows_of(&e, v, size), vec![vec![4, 5, 6], vec![4, 5, 6]]);
        let empty = bare(&mut e);
        e.call(address, &args![empty, 0u32, source]);
        assert_eq!(rows_of(&e, empty, size), Vec::<Vec<u32>>::new());
    }

    #[test]
    fn the_short_vectors_constructor_copies_the_value() {
        check_vectors_constructor(0x00b3_2780, 2);
    }

    #[test]
    fn the_word_vectors_constructor_copies_the_value() {
        check_vectors_constructor(0x00b3_2ce0, 4);
    }

    /// Checks `resize(new_size, value)` of a vector of vectors (`address`).
    fn check_vectors_resize(address: u32, size: u32) {
        let mut e = engine();
        let outer = nested(&mut e, &[&[1]], size, 0);
        let source = vector(&mut e, &[9, 9], size, 0);
        let w = words6(&e, source);
        e.call(
            address,
            &args![outer, 3u32, w[0], w[1], w[2], w[3], w[4], w[5]],
        );
        assert_eq!(
            rows_of(&e, outer, size),
            vec![vec![1], vec![9, 9], vec![9, 9]]
        );
        let source = vector(&mut e, &[5], size, 0);
        let w = words6(&e, source);
        e.call(
            address,
            &args![outer, 1u32, w[0], w[1], w[2], w[3], w[4], w[5]],
        );
        assert_eq!(rows_of(&e, outer, size), vec![vec![1]]);
    }

    #[test]
    fn the_short_vectors_resize_grows_and_shrinks() {
        check_vectors_resize(0x00b3_2880, 2);
    }

    #[test]
    fn the_word_vectors_resize_grows_and_shrinks() {
        check_vectors_resize(0x00b3_2de0, 4);
    }

    /// Checks the erase of the middle row of a vector of vectors (`address`).
    fn check_vectors_erase(address: u32, size: u32) {
        let mut e = engine();
        let v = nested(&mut e, &[&[1], &[2], &[3]], size, 0);
        let first = first_of(&e, v);
        let proxy = e.mem.u32(v);
        let out = e.mem.alloc(8);
        let result = e
            .call(
                address,
                &args![v, out, proxy, first + 0x18, proxy, first + 0x30],
            )
            .u32();
        assert_eq!(result, out);
        assert_eq!(rows_of(&e, v, size), vec![vec![1], vec![3]]);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (proxy, first + 0x18));
    }

    #[test]
    fn the_short_vectors_erase_assigns_down_and_destroys_the_tail() {
        check_vectors_erase(0x00b3_2ad0, 2);
    }

    #[test]
    fn the_word_vectors_erase_assigns_down_and_destroys_the_tail() {
        check_vectors_erase(0x00b3_3030, 4);
    }

    /// Checks the destruction of a range of vectors (`address`).
    fn check_vectors_destroy(address: u32, size: u32) {
        let mut e = engine();
        let v = nested(&mut e, &[&[1], &[2]], size, 0);
        let (first, last) = (first_of(&e, v), last_of(&e, v));
        let proxy = e.mem.u32(first);
        let storage = first_of(&e, first);
        e.call(address, &args![v, first, last]);
        assert_eq!((first_of(&e, first), last_of(&e, first)), (0, 0));
        assert!(e.mem.block_size(storage).is_none());
        assert!(e.mem.block_size(proxy).is_none());
    }

    #[test]
    fn the_short_vectors_destroy_range_tidies_and_frees_each_vector() {
        check_vectors_destroy(0x00b3_2bb0, 2);
    }

    #[test]
    fn the_word_vectors_destroy_range_tidies_and_frees_each_vector() {
        check_vectors_destroy(0x00b3_3110, 4);
    }

    /// Checks the tidy of a vector of vectors (`address`).
    fn check_vectors_tidy(address: u32, size: u32) {
        let mut e = engine();
        let v = nested(&mut e, &[&[1], &[2]], size, 0);
        let storage = first_of(&e, v);
        e.call(address, &args![v]);
        assert_eq!(
            (first_of(&e, v), last_of(&e, v), e.mem.u32(v + 0x14)),
            (0, 0, 0)
        );
        assert!(e.mem.block_size(storage).is_none());
        e.call(address, &args![v]);
    }

    #[test]
    fn the_short_vectors_tidy_destroys_the_rows_and_frees_the_block() {
        check_vectors_tidy(0x00b3_2c60, 2);
    }

    #[test]
    fn the_word_vectors_tidy_destroys_the_rows_and_frees_the_block() {
        check_vectors_tidy(0x00b3_31c0, 4);
    }

    /// Checks `_Insert_n` of a vector of vectors (`address`) in its three cases and the length
    /// error.
    fn check_vectors_insert_n(address: u32, size: u32) {
        let mut e = engine();
        // Without room.
        let v = nested(&mut e, &[&[1]], size, 0);
        let source = vector(&mut e, &[9, 9], size, 0);
        let (proxy, last) = (e.mem.u32(v), last_of(&e, v));
        e.call(address, &args![v, proxy, last, 2u32, source]);
        assert_eq!(rows_of(&e, v, size), vec![vec![1], vec![9, 9], vec![9, 9]]);
        // Fewer rows follow the position than are inserted.
        let v = nested(&mut e, &[&[1], &[2]], size, 3);
        let source = vector(&mut e, &[7], size, 0);
        let (proxy, first) = (e.mem.u32(v), first_of(&e, v));
        e.call(address, &args![v, proxy, first + 0x18, 3u32, source]);
        assert_eq!(
            rows_of(&e, v, size),
            vec![vec![1], vec![7], vec![7], vec![7], vec![2]]
        );
        // At least as many follow as are inserted.
        let v = nested(&mut e, &[&[1], &[2], &[3]], size, 2);
        let source = vector(&mut e, &[9], size, 0);
        let (proxy, first) = (e.mem.u32(v), first_of(&e, v));
        e.call(address, &args![v, proxy, first + 0x18, 1u32, source]);
        assert_eq!(
            rows_of(&e, v, size),
            vec![vec![1], vec![9], vec![2], vec![3]]
        );
        // A count of zero does nothing.
        e.call(address, &args![v, proxy, first, 0u32, source]);
        assert_eq!(rows_of(&e, v, size).len(), 4);
        // Too many rows.
        let log = recording(&mut e, LENGTH_ERROR);
        e.call(address, &args![v, proxy, first, 0x0aaa_aaaau32, source]);
        assert_eq!(log.borrow().len(), 1);
        assert_eq!(rows_of(&e, v, size).len(), 4);
    }

    #[test]
    fn the_short_vectors_insert_n_covers_its_three_cases() {
        check_vectors_insert_n(0x00b3_3bd0, 2);
    }

    #[test]
    fn the_word_vectors_insert_n_covers_its_three_cases() {
        check_vectors_insert_n(0x00b3_4450, 4);
    }

    /// Checks an assigning copy of vectors (`address`).
    fn check_vectors_assign_copy(address: u32, size: u32, extra_words: usize) {
        let mut e = engine();
        let source = nested(&mut e, &[&[1], &[2]], size, 0);
        let target = nested(&mut e, &[&[0], &[0]], size, 0);
        let (first, last) = (first_of(&e, source), last_of(&e, source));
        let dest = first_of(&e, target);
        let mut words = vec![first, last, dest];
        words.resize(3 + extra_words, 0);
        let end = e.call(address, &words).u32();
        assert_eq!(end, dest + 0x30);
        assert_eq!(rows_of(&e, target, size), vec![vec![1], vec![2]]);
    }

    #[test]
    fn the_short_vectors_assign_copy_assigns_each_vector() {
        check_vectors_assign_copy(0x00b3_5a90, 2, 0);
    }

    #[test]
    fn the_word_vectors_assign_copy_assigns_each_vector() {
        check_vectors_assign_copy(0x00b3_5b60, 4, 0);
    }

    #[test]
    fn the_second_short_vectors_assign_copy_assigns_each_vector() {
        check_vectors_assign_copy(0x00b3_5f60, 2, 3);
    }

    #[test]
    fn the_second_word_vectors_assign_copy_assigns_each_vector() {
        check_vectors_assign_copy(0x00b3_6000, 4, 3);
    }

    /// Checks the backward move of vectors (`address`): the rows shift up by one slot, the
    /// slot before them ending up empty.
    fn check_vectors_move_backward(address: u32, size: u32) {
        let mut e = engine();
        let v = nested(&mut e, &[&[1], &[2], &[3], &[]], size, 0);
        let first = first_of(&e, v);
        let start = e
            .call(address, &args![first, first + 0x48, first + 0x60])
            .u32();
        assert_eq!(start, first + 0x18);
        let rows: Vec<Vec<u32>> = (0..4)
            .map(|i| elements(&e, first + 0x18 * i, size))
            .collect();
        assert_eq!(rows, vec![vec![], vec![1], vec![2], vec![3]]);
    }

    #[test]
    fn the_short_vectors_move_backward_swaps_each_vector() {
        check_vectors_move_backward(0x00b3_5ce0, 2);
    }

    #[test]
    fn the_word_vectors_move_backward_swaps_each_vector() {
        check_vectors_move_backward(0x00b3_5da0, 4);
    }

    /// Checks the fill construction of vectors (`address`).
    fn check_vectors_fill(address: u32, size: u32) {
        let mut e = engine();
        let source = vector(&mut e, &[4], size, 0);
        let dest = e.mem.alloc(0x30);
        e.call(address, &args![dest, 2u32, source, 0u32, 0u32, 0u32]);
        assert_eq!(elements(&e, dest, size), vec![4]);
        assert_eq!(elements(&e, dest + 0x18, size), vec![4]);
        // A null destination is skipped.
        e.call(address, &args![0u32, 1u32, source, 0u32, 0u32, 0u32]);
    }

    #[test]
    fn the_short_vectors_fill_copy_constructs_each_destination() {
        check_vectors_fill(0x00b3_60a0, 2);
    }

    #[test]
    fn the_word_vectors_fill_copy_constructs_each_destination() {
        check_vectors_fill(0x00b3_61b0, 4);
    }

    /// Checks the construction of a range of vectors from another (`address`).
    fn check_vectors_uninit_copy(address: u32, size: u32) {
        let mut e = engine();
        let source = nested(&mut e, &[&[1], &[2]], size, 0);
        let (first, last) = (first_of(&e, source), last_of(&e, source));
        let dest = e.mem.alloc(0x30);
        let end = e
            .call(address, &args![first, last, dest, 0u32, 0u32, 0u32])
            .u32();
        assert_eq!(end, dest + 0x30);
        assert_eq!(elements(&e, dest, size), vec![1]);
        assert_eq!(elements(&e, dest + 0x18, size), vec![2]);
        // A null destination is skipped but still advanced (only the first one is null).
        assert_eq!(
            e.call(address, &args![first, first + 0x18, 0u32, 0u32, 0u32, 0u32])
                .u32(),
            0x18
        );
    }

    #[test]
    fn the_short_vectors_uninitialised_copy_constructs_each_vector() {
        check_vectors_uninit_copy(0x00b3_68d0, 2);
    }

    #[test]
    fn the_word_vectors_uninitialised_copy_constructs_each_vector() {
        check_vectors_uninit_copy(0x00b3_69f0, 4);
    }

    /// Checks the construction of a range of vectors by moving them (`address`).
    fn check_vectors_uninit_move(address: u32, size: u32) {
        let mut e = engine();
        let source = nested(&mut e, &[&[1], &[2]], size, 0);
        let (first, last) = (first_of(&e, source), last_of(&e, source));
        let dest = e.mem.alloc(0x30);
        let end = e
            .call(address, &args![first, last, dest, 0u32, 0u32, 0u32])
            .u32();
        assert_eq!(end, dest + 0x30);
        assert_eq!(elements(&e, dest, size), vec![1]);
        assert_eq!(elements(&e, dest + 0x18, size), vec![2]);
        assert_eq!(rows_of(&e, source, size), vec![vec![], vec![]]);
    }

    #[test]
    fn the_short_vectors_uninitialised_move_swaps_each_vector_into_place() {
        check_vectors_uninit_move(0x00b3_6bf0, 2);
    }

    #[test]
    fn the_word_vectors_uninitialised_move_swaps_each_vector_into_place() {
        check_vectors_uninit_move(0x00b3_6d70, 4);
    }

    // ---- the wind engine ------------------------------------------------------------------

    fn set_floats(e: &mut Engine, at: u32, values: &[f32]) {
        for (i, value) in values.iter().enumerate() {
            e.mem.set_f32(at + 4 * i as u32, *value);
        }
    }

    #[test]
    fn the_wind_engine_constructor_sets_its_defaults() {
        let mut e = engine();
        e.mem.set_u32(0x0101_622c, 0x3e80_0000);
        let this = e.mem.alloc(0x44);
        assert_eq!(e.call(0x00b3_73c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.f32(this + 0x04), 0.25);
        assert_eq!(
            (
                e.mem.u32(this + 8),
                e.mem.u32(this + 0xC),
                e.mem.u32(this + 0x10)
            ),
            (2, 2, 2)
        );
        assert_eq!(e.mem.u8(this + 0x14), 1);
        assert_eq!((e.mem.f32(this + 0x18), e.mem.f32(this + 0x1C)), (1.0, 1.0));
        assert_eq!((e.mem.u32(this + 0x20), e.mem.u32(this + 0x24)), (0, 0));
        assert_eq!((e.mem.u32(this + 0x28), e.mem.u32(this + 0x2C)), (0, 4));
        assert_eq!(
            (
                e.mem.u32(this + 0x30),
                e.mem.u32(this + 0x34),
                e.mem.u32(this + 0x38)
            ),
            (0, 0, 0)
        );
        assert_eq!((e.mem.f32(this + 0x3C), e.mem.f32(this + 0x40)), (1.0, 1.0));
    }

    #[test]
    fn the_wind_setter_computes_the_leaf_frequency_throw_and_shift() {
        let mut e = engine();
        e.mem.set_f32(0x011f_8d80, 2.0);
        e.mem.set_f32(0x010a_5784, 3.0);
        e.mem.set_f32(0x010a_5788, 5.0);
        let this = e.mem.alloc(0x44);
        set_floats(&mut e, this + 0x18, &[0.5, 0.25]);
        e.mem.set_u32(this + 0x30, 9);
        e.mem.set_u32(this + 0x34, 9);
        let shift = e
            .call(0x00b3_7470, &args![this, 0.5f32, 2.0f32, 1.0f32])
            .f32();
        assert_eq!(shift, 3.25);
        assert_eq!(e.mem.f32(this), 3.25);
        assert_eq!(e.mem.f32(this + 4), 0.5);
        assert_eq!(e.mem.f32(this + 0x20), 0.375);
        assert_eq!(e.mem.f32(this + 0x24), 1.25);
        assert_eq!((e.mem.u32(this + 0x30), e.mem.u32(this + 0x34)), (0, 0));
    }

    #[test]
    fn the_wind_info_setter_copies_the_leaf_factors_and_strength() {
        let mut e = engine();
        e.mem.set_f32(0x010a_5784, 3.0);
        e.mem.set_f32(0x010a_5788, 5.0);
        let this = e.mem.alloc(0x44);
        let info = e.mem.alloc(0x1c);
        set_floats(&mut e, info, &[0.5, 0.25]);
        e.mem.set_f32(info + 0x18, 2.0);
        e.call(0x00b3_7510, &args![this, info]);
        assert_eq!(e.mem.f32(this + 0x18), 0.5);
        assert_eq!(e.mem.f32(this + 0x1C), 0.25);
        assert_eq!(e.mem.f32(this + 0x20), 1.5);
        assert_eq!(e.mem.f32(this + 0x24), 5.0);
        assert_eq!(e.mem.f32(this + 4), 2.0);
    }
}
