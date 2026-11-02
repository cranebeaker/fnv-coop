//! `SpeedTree/run_00b42a60` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! MSVC `std::vector<bool>` instantiations of the tree code, identified from the code (the
//! engine map gives them no names): the checked bit iterator (a proxy word, a pointer to the
//! 32-bit word and the bit number in it; 12 bytes) with its `+=`, `++` and conversion, the fill
//! of a range of bits, and the helpers of the vector of 8-byte elements the unit also holds
//! (iterator `+=`, allocation, fill, forward and backward copy).
//!
//! The vector's container (`*proxy` of an iterator) keeps the bit count at `+8` and the
//! `std::vector<unsigned int>` of the words at `+0x0C` (its first and last pointers at `+0x18`
//! and `+0x1C`).

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{INVALID_PARAMETER, OPERATOR_NEW};
use super::run_00b0eca0::advance_checked_iterator;

/// `memmove_s(dest, dest_size, source, count)` (`00ec7ce3`, `cdecl`).
const MEMMOVE_S: u32 = 0x00ec_7ce3;
/// `_CxxThrowException(object, throw_info)` (`00ec7894`).
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// `std::exception::exception(const char *const &)` (`00ec78f1`, `thiscall`), the base of
/// `std::bad_alloc`, its vtable (`010176f8`) and its throw info (`01140ef8`).
const EXCEPTION_FROM_MESSAGE: u32 = 0x00ec_78f1;
const BAD_ALLOC_VTABLE: u32 = 0x0101_76f8;
const BAD_ALLOC_THROW_INFO: u32 = 0x0114_0ef8;
/// The assignment of one bit through an iterator (`00b41ae0`, `thiscall`: the iterator, the
/// value byte).
const BIT_ASSIGN: u32 = 0x00b4_1ae0;

/// An iterator's fields: the proxy, the pointer to the 32-bit word and the bit in it.
const ITERATOR_POINTER: u32 = 4;
const ITERATOR_BIT: u32 = 8;

/// The container an iterator belongs to: the first word of its proxy, or 0 without a proxy.
fn iterator_owner(e: &Engine, iterator: u32) -> u32 {
    let proxy = e.mem.u32(iterator);
    if proxy == 0 {
        0
    } else {
        e.mem.u32(proxy)
    }
}

/// The index of the iterator's bit from the start of the words:
/// `(((pointer - first word) >> 2) << 5) + bit`, `bit` read before the first word is looked up
/// (`00b42c60`).
fn bit_position(e: &mut Engine, iterator: u32) -> u32 {
    let bit = e.mem.u32(iterator + ITERATOR_BIT);
    let first = fn_00b42c60(e, Ptr::new(iterator));
    let pointer = e.mem.u32(iterator + ITERATOR_POINTER);
    (((pointer.wrapping_sub(first)) as i32 >> 2) as u32)
        .wrapping_shl(5)
        .wrapping_add(bit)
}

/// Copies one 8-byte element.
fn copy_pair(e: &mut Engine, dest: u32, source: u32) {
    let high = e.mem.u32(source.wrapping_add(4));
    let low = e.mem.u32(source);
    e.mem.set_u32(dest, low);
    e.mem.set_u32(dest.wrapping_add(4), high);
}

// Translated from 00b42a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked bit iterator's `operator+=(n)` (identified from the code): nothing for 0;
/// otherwise the iterator needs a proxy and a word pointer, and the target bit index must be
/// inside `0..=bit count` (a backward move before the start is an invalid parameter, as is a
/// forward move past the end); then the bit number and the word pointer move so that the bit
/// number stays below 32. Returns `this`.
pub fn fn_00b42a60(e: &mut Engine, this: Ptr, count: i32) -> Ptr {
    let iterator = this.addr();
    if count == 0 {
        return this;
    }
    if e.mem.u32(iterator) == 0 || e.mem.u32(iterator + ITERATOR_POINTER) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let step = count as u32;
    if count < 0 {
        let position = bit_position(e, iterator);
        if position < step.wrapping_neg() {
            e.call(INVALID_PARAMETER, &args![]);
        }
    } else {
        let position = bit_position(e, iterator);
        let owner = iterator_owner(e, iterator);
        if position.wrapping_add(step) > e.mem.u32(owner.wrapping_add(8)) {
            e.call(INVALID_PARAMETER, &args![]);
        }
    }
    if count < 0 && e.mem.u32(iterator + ITERATOR_BIT) < step.wrapping_neg() {
        let bit = e.mem.u32(iterator + ITERATOR_BIT).wrapping_add(step);
        e.mem.set_u32(iterator + ITERATOR_BIT, bit);
        let back = ((0xFFFF_FFFFu32.wrapping_sub(bit)) >> 5)
            .wrapping_mul(4)
            .wrapping_add(4);
        let pointer = e.mem.u32(iterator + ITERATOR_POINTER).wrapping_sub(back);
        e.mem.set_u32(iterator + ITERATOR_POINTER, pointer);
        let bit = e.mem.u32(iterator + ITERATOR_BIT) % 0x20;
        e.mem.set_u32(iterator + ITERATOR_BIT, bit);
    } else {
        let bit = e.mem.u32(iterator + ITERATOR_BIT).wrapping_add(step);
        e.mem.set_u32(iterator + ITERATOR_BIT, bit);
        let pointer = e
            .mem
            .u32(iterator + ITERATOR_POINTER)
            .wrapping_add((e.mem.u32(iterator + ITERATOR_BIT) >> 5).wrapping_mul(4));
        e.mem.set_u32(iterator + ITERATOR_POINTER, pointer);
        let bit = e.mem.u32(iterator + ITERATOR_BIT) % 0x20;
        e.mem.set_u32(iterator + ITERATOR_BIT, bit);
    }
    this
}

// Translated from 00b42be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds the bit iterator `this` from the one at `other` (identified from the code): the word
/// pointer and bit number are copied and the proxy is taken from the first word of the
/// container of `other` (`*(*other.proxy)`); an iterator without a proxy is an invalid
/// parameter. Returns `this`.
pub fn fn_00b42be0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let owner = iterator_owner(e, other.addr());
    let bit = e.mem.u32(other.addr() + ITERATOR_BIT);
    let pointer = e.mem.u32(other.addr() + ITERATOR_POINTER);
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + ITERATOR_POINTER, pointer);
    e.mem.set_u32(this.addr() + ITERATOR_BIT, bit);
    if owner == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let proxy = e.mem.u32(owner);
    e.mem.set_u32(this.addr(), proxy);
    this
}

// Translated from 00b42c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The first word of the bit vector's storage as seen from an iterator (identified from the
/// code; `fastcall`, `this` = the iterator): the first pointer of the `std::vector<unsigned
/// int>` inside the container (`+0x18`), after the checks that the vector exists and that its
/// first pointer is not past its last. Returns that pointer.
pub fn fn_00b42c60(e: &mut Engine, this: Ptr) -> u32 {
    let owner = iterator_owner(e, this.addr());
    let words = owner.wrapping_add(0x0C);
    let first = e.mem.u32(words.wrapping_add(0x0C));
    // The compiler tests the sum `owner + 0x0C` for zero (the flags of the `ADD`).
    if words == 0
        || e.mem.u32(words.wrapping_add(0x0C)) > first
        || first > e.mem.u32(words.wrapping_add(0x10))
    {
        e.call(INVALID_PARAMETER, &args![]);
    }
    first
}

// Translated from 00b42ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked iterator's `operator+=(n)` for 8-byte elements (identified from the code; see
/// `00b104f0`): moves the element pointer by `n * 8` bytes; an iterator without a proxy, or a
/// target outside the container's `[first, last]`, takes the invalid-parameter path. Returns
/// `this`.
pub fn fn_00b42ce0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    advance_checked_iterator(e, this.addr(), count.wrapping_mul(8));
    this
}

// Translated from 00b42da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the backward copy of 8-byte elements to `00b43090` (identified from the code;
/// `cdecl`) with three tag bytes, uninitialised stack bytes in the exe and zero here. Returns
/// what it returns.
pub fn fn_00b42da0(e: &mut Engine, first: u32, last: u32, dest_end: u32) -> u32 {
    fn_00b43090(e, first, last, dest_end, 0, 0, 0)
}

// Translated from 00b42df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocation of the 8-byte element vector (identified from the code; `std::_Allocate(count,
/// hint)`, the hint unused): `operator new(count * 8)`. A count of 0 allocates 0 bytes; a count
/// too large for the multiplication throws `std::bad_alloc`.
pub fn fn_00b42df0(e: &mut Engine, count: u32, _unused_1: u32) -> u32 {
    // A count of 0 skips the overflow test and allocates 0 bytes.
    if count != 0 && 0xFFFF_FFFFu32 / count < 8 {
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
    e.call(OPERATOR_NEW, &args![count.wrapping_shl(3)]).u32()
}

// Translated from 00b42e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// A copy of 4-byte words (identified from the code; `cdecl`, `first`, `last`, `dest`): the
/// word count is `(last - first) >> 2` (signed); a positive count is moved with `memmove_s`.
/// Returns `dest` plus the count of bytes.
pub fn fn_00b42e50(e: &mut Engine, first: u32, last: u32, dest: u32) -> u32 {
    let words = (last.wrapping_sub(first)) as i32 >> 2;
    if words > 0 {
        let bytes = (words as u32).wrapping_shl(2);
        e.call(MEMMOVE_S, &args![dest, bytes, first, bytes]);
    }
    dest.wrapping_add((words as u32).wrapping_mul(4))
}

// Translated from 00b42ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills a range of a bit vector (identified from the code; `cdecl`, the first iterator and the
/// end iterator by value, three words each, and a pointer to the value byte): while the
/// iterator's word pointer and bit differ from the end's, a copy of the iterator is made
/// (`00b42be0`), the bit assigned the value (`00b41ae0`) and the iterator advanced one bit
/// (`00b43120`).
pub fn fn_00b42ed0(
    e: &mut Engine,
    first_proxy: u32,
    first_pointer: u32,
    first_bit: u32,
    _last_proxy: u32,
    last_pointer: u32,
    last_bit: u32,
    value: Ptr,
) {
    e.with_stack(0xC, |e, first| {
        e.mem.set_u32(first.addr(), first_proxy);
        e.mem
            .set_u32(first.addr() + ITERATOR_POINTER, first_pointer);
        e.mem.set_u32(first.addr() + ITERATOR_BIT, first_bit);
        while e.mem.u32(first.addr() + ITERATOR_POINTER) != last_pointer
            || e.mem.u32(first.addr() + ITERATOR_BIT) != last_bit
        {
            e.with_stack(0xC, |e, copy| {
                fn_00b42be0(e, copy, first);
                let byte = u32::from(e.mem.u8(value.addr()));
                e.call(BIT_ASSIGN, &args![copy, byte]);
            });
            fn_00b43120(e, first);
        }
    });
}

// Translated from 00b42f40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b42f40 - the FS:[0] exception frame is not translated.
/// `std::_Uninit_fill_n` for the 8-byte element (identified from the code; `cdecl`): copies
/// the element at `value` to `count` places from `dest`, 8 bytes apart; a null destination is
/// skipped.
pub fn fn_00b42f40(e: &mut Engine, dest: u32, count: u32, value: Ptr) {
    let mut dest = dest;
    let mut count = count;
    while count != 0 {
        if dest != 0 {
            copy_pair(e, dest, value.addr());
        }
        count = count.wrapping_sub(1);
        dest = dest.wrapping_add(8);
    }
}

// Translated from 00b43010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards a range copy-construct of 8-byte elements to `00b43230` (identified from the code;
/// `cdecl`, `first`, `last`, `dest`, the allocator) with two tag bytes, uninitialised stack
/// bytes in the exe and zero here. Returns its result.
pub fn fn_00b43010(e: &mut Engine, first: u32, last: u32, dest: u32, allocator: u32) -> u32 {
    fn_00b43230(e, first, last, dest, allocator, 0, 0)
}

// Translated from 00b43090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The backward copy of 8-byte elements (identified from the code; `cdecl`, `first`, `last`,
/// `dest_end`, three tag bytes not read): copies the elements from the last one down to the
/// first so that the range ends at `dest_end`. Returns the start of the copy, `dest_end` minus
/// the whole elements' size.
pub fn fn_00b43090(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest_end: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let start =
        dest_end.wrapping_sub((((last.wrapping_sub(first)) as i32 >> 3) as u32).wrapping_shl(3));
    let mut source = last;
    let mut dest = dest_end;
    while first != source {
        source = source.wrapping_sub(8);
        dest = dest.wrapping_sub(8);
        let low = e.mem.u32(source);
        let high = e.mem.u32(source.wrapping_add(4));
        e.mem.set_u32(dest, low);
        e.mem.set_u32(dest.wrapping_add(4), high);
    }
    start
}

// Translated from 00b43120 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked bit iterator's `operator++` (identified from the code; `fastcall`, `this` = the
/// iterator): needs a proxy and a word pointer, and the bit index after the step must not be
/// past the bit count (else the invalid-parameter path); the bit number advances, wrapping from
/// 31 to 0 with the word pointer moving on by 4.
pub fn fn_00b43120(e: &mut Engine, this: Ptr) {
    let iterator = this.addr();
    if e.mem.u32(iterator) == 0 || e.mem.u32(iterator + ITERATOR_POINTER) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let position = bit_position(e, iterator);
    let owner = iterator_owner(e, iterator);
    if position.wrapping_add(1) > e.mem.u32(owner.wrapping_add(8)) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    if e.mem.u32(iterator + ITERATOR_BIT) < 0x1F {
        let bit = e.mem.u32(iterator + ITERATOR_BIT).wrapping_add(1);
        e.mem.set_u32(iterator + ITERATOR_BIT, bit);
    } else {
        e.mem.set_u32(iterator + ITERATOR_BIT, 0);
        let pointer = e.mem.u32(iterator + ITERATOR_POINTER).wrapping_add(4);
        e.mem.set_u32(iterator + ITERATOR_POINTER, pointer);
    }
}

// Translated from 00b431e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards a range copy-construct of 8-byte elements to `00b43230` (identified from the code;
/// the same forwarder as `00b43010`, one layer of inlined helpers fewer; `cdecl`, `first`,
/// `last`, `dest`, the allocator). Returns its result.
pub fn fn_00b431e0(e: &mut Engine, first: u32, last: u32, dest: u32, allocator: u32) -> u32 {
    fn_00b43230(e, first, last, dest, allocator, 0, 0)
}

// Translated from 00b43230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b43230 - the FS:[0] exception frame is not translated.
/// `std::_Uninit_copy` for the 8-byte element (identified from the code; `cdecl`, `first`,
/// `last`, `dest`, the allocator and two tag bytes not read): copies each element of
/// `first..last` to `dest`, 8 bytes apart; a null destination is skipped. Returns the end of
/// the destination.
pub fn fn_00b43230(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let mut source = first;
    let mut dest = dest;
    while source != last {
        if dest != 0 {
            copy_pair(e, dest, source);
        }
        dest = dest.wrapping_add(8);
        source = source.wrapping_add(8);
    }
    dest
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b42a60, fn_00b42a60(Ptr, i32) -> Ptr),
        entry!(0x00b42be0, fn_00b42be0(Ptr, Ptr) -> Ptr),
        entry!(0x00b42c60, fn_00b42c60(Ptr) -> u32),
        entry!(0x00b42ce0, fn_00b42ce0(Ptr, u32) -> Ptr),
        entry!(0x00b42da0, fn_00b42da0(u32, u32, u32) -> u32),
        entry!(0x00b42df0, fn_00b42df0(u32, u32) -> u32),
        entry!(0x00b42e50, fn_00b42e50(u32, u32, u32) -> u32),
        entry!(0x00b42ed0, fn_00b42ed0(u32, u32, u32, u32, u32, u32, Ptr)),
        entry!(0x00b42f40, fn_00b42f40(u32, u32, Ptr)),
        entry!(0x00b43010, fn_00b43010(u32, u32, u32, u32) -> u32),
        entry!(0x00b43090, fn_00b43090(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b43120, fn_00b43120(Ptr)),
        entry!(0x00b431e0, fn_00b431e0(u32, u32, u32, u32) -> u32),
        entry!(0x00b43230, fn_00b43230(u32, u32, u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::rc::Rc;

    /// The proxy word a test container carries at `+0`.
    const PROXY: u32 = 0x7000_0001;

    type Words = Rc<RefCell<Vec<Vec<u32>>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// An engine with a panicking invalid-parameter path and throw, and `memmove_s` that really
    /// moves bytes.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0020_0000);
        e.register(INVALID_PARAMETER, |_, _| panic!("invalid parameter"));
        e.register(CXX_THROW_EXCEPTION, |_, _| panic!("thrown"));
        e.register(MEMMOVE_S, |e, a| {
            let data = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &data);
            Ret::default()
        });
        e
    }

    fn panics(action: impl FnOnce()) -> bool {
        catch_unwind(AssertUnwindSafe(action)).is_err()
    }

    fn spy(e: &mut Engine, addr: u32, result: u32) -> Words {
        let log: Words = Rc::new(RefCell::new(Vec::new()));
        let sink = log.clone();
        e.register_double(addr, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(result)
        });
        log
    }

    /// A bit vector of `bits` bits in `words` words: returns the container, its proxy block and
    /// the words.
    fn bit_vector(e: &mut Engine, bits: u32, words: u32) -> (u32, u32, u32) {
        let container = e.mem.alloc(0x24);
        let data = e.mem.alloc(4 * words);
        e.mem.set_u32(container, PROXY);
        e.mem.set_u32(container + 8, bits);
        e.mem.set_u32(container + 0x18, data);
        e.mem.set_u32(container + 0x1C, data + 4 * words);
        e.mem.set_u32(container + 0x20, data + 4 * words);
        let proxy = e.mem.alloc(4);
        e.mem.set_u32(proxy, container);
        (container, proxy, data)
    }

    fn iterator(e: &mut Engine, proxy: u32, pointer: u32, bit: u32) -> u32 {
        let at = e.mem.alloc(12);
        e.mem.set_u32(at, proxy);
        e.mem.set_u32(at + 4, pointer);
        e.mem.set_u32(at + 8, bit);
        at
    }

    fn state(e: &Engine, iterator: u32) -> (u32, u32) {
        (e.mem.u32(iterator + 4), e.mem.u32(iterator + 8))
    }

    #[test]
    fn bit_iterator_moves_within_the_bits() {
        let mut e = engine();
        let (_, proxy, data) = bit_vector(&mut e, 70, 3);
        let it = iterator(&mut e, proxy, data, 0);
        let it_ptr = Ptr::<()>::new(it);
        assert_eq!(e.call(0x00b4_2a60, &args![it_ptr, 3i32]).u32(), it);
        assert_eq!(state(&e, it), (data, 3));
        e.call(0x00b4_2a60, &args![it_ptr, 33i32]);
        assert_eq!(state(&e, it), (data + 4, 4));
        // Backward across a word start: the bit number wraps below zero.
        e.call(0x00b4_2a60, &args![it_ptr, -5i32]);
        assert_eq!(state(&e, it), (data, 31));
        // Backward inside a word.
        e.call(0x00b4_2a60, &args![it_ptr, -30i32]);
        assert_eq!(state(&e, it), (data, 1));
        // To the very end, then no further.
        e.call(0x00b4_2a60, &args![it_ptr, 69i32]);
        assert_eq!(state(&e, it), (data + 8, 6));
        assert!(panics(|| {
            fn_00b42a60(&mut e, it_ptr, 1);
        }));
        // Before the start.
        let start = iterator(&mut e, proxy, data, 2);
        assert!(panics(|| {
            fn_00b42a60(&mut e, Ptr::new(start), -3);
        }));
    }

    #[test]
    fn bit_iterator_move_of_zero_and_without_proxy() {
        let mut e = engine();
        let (_, proxy, data) = bit_vector(&mut e, 70, 3);
        let it = iterator(&mut e, 0, data, 1);
        // A move of 0 does nothing, whatever the iterator.
        assert_eq!(
            e.call(0x00b4_2a60, &args![Ptr::<()>::new(it), 0i32]).u32(),
            it
        );
        assert!(panics(|| {
            fn_00b42a60(&mut e, Ptr::new(it), 2);
        }));
        let it = iterator(&mut e, proxy, 0, 1);
        assert!(panics(|| {
            fn_00b42a60(&mut e, Ptr::new(it), 2);
        }));
    }

    #[test]
    fn bit_iterator_increment() {
        let mut e = engine();
        let (_, proxy, data) = bit_vector(&mut e, 34, 2);
        let it = iterator(&mut e, proxy, data, 5);
        e.call(0x00b4_3120, &args![Ptr::<()>::new(it)]);
        assert_eq!(state(&e, it), (data, 6));
        e.mem.set_u32(it + 8, 31);
        e.call(0x00b4_3120, &args![Ptr::<()>::new(it)]);
        assert_eq!(state(&e, it), (data + 4, 0));
        // Position 34 is the end: no step beyond.
        let end = iterator(&mut e, proxy, data + 4, 2);
        assert!(panics(|| {
            fn_00b43120(&mut e, Ptr::new(end));
        }));
        let none = iterator(&mut e, 0, data, 2);
        assert!(panics(|| {
            fn_00b43120(&mut e, Ptr::new(none));
        }));
    }

    #[test]
    fn bit_iterator_conversion_takes_the_container_proxy() {
        let mut e = engine();
        let (_, proxy, data) = bit_vector(&mut e, 70, 3);
        let other = iterator(&mut e, proxy, data + 4, 9);
        let this = iterator(&mut e, 0x1234, 0, 0);
        let r = e.call(
            0x00b4_2be0,
            &args![Ptr::<()>::new(this), Ptr::<()>::new(other)],
        );
        assert_eq!(r.u32(), this);
        assert_eq!(e.mem.u32(this), PROXY);
        assert_eq!(state(&e, this), (data + 4, 9));
        let none = iterator(&mut e, 0, data, 1);
        assert!(panics(|| {
            fn_00b42be0(&mut e, Ptr::new(this), Ptr::new(none));
        }));
    }

    #[test]
    fn bit_vector_first_word() {
        let mut e = engine();
        let (container, proxy, data) = bit_vector(&mut e, 70, 3);
        let it = iterator(&mut e, proxy, data, 0);
        assert_eq!(e.call(0x00b4_2c60, &args![Ptr::<()>::new(it)]).u32(), data);
        // A first pointer past the last is an invalid parameter.
        e.mem.set_u32(container + 0x1C, data - 4);
        assert!(panics(|| {
            fn_00b42c60(&mut e, Ptr::new(it));
        }));
    }

    #[test]
    fn bit_range_fill_assigns_every_bit() {
        let mut e = engine();
        let (_, proxy, data) = bit_vector(&mut e, 70, 3);
        let assigned: Rc<RefCell<Vec<(u32, u32, u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = assigned.clone();
        e.register_double(BIT_ASSIGN, move |e, a| {
            let copy = a[0];
            sink.borrow_mut().push((
                e.mem.u32(copy),
                e.mem.u32(copy + 4),
                e.mem.u32(copy + 8),
                a[1],
            ));
            Ret::default()
        });
        let value = e.mem.alloc(1);
        e.mem.set_u8(value, 1);
        e.call(
            0x00b4_2ed0,
            &args![
                proxy,
                data,
                30u32,
                proxy,
                data + 4,
                1u32,
                Ptr::<()>::new(value)
            ],
        );
        // Bits 30, 31 and 0 of the next word: three assignments with the converted proxy.
        assert_eq!(
            *assigned.borrow(),
            vec![
                (PROXY, data, 30, 1),
                (PROXY, data, 31, 1),
                (PROXY, data + 4, 0, 1)
            ]
        );
        // An empty range assigns nothing.
        assigned.borrow_mut().clear();
        e.call(
            0x00b4_2ed0,
            &args![proxy, data, 3u32, proxy, data, 3u32, Ptr::<()>::new(value)],
        );
        assert!(assigned.borrow().is_empty());
    }

    #[test]
    fn pair_iterator_advance() {
        let mut e = engine();
        let container = e.mem.alloc(0x18);
        let data = e.mem.alloc(8 * 4);
        e.mem.set_u32(container, PROXY);
        e.mem.set_u32(container + 0x0C, data);
        e.mem.set_u32(container + 0x10, data + 8 * 4);
        let proxy = e.mem.alloc(4);
        e.mem.set_u32(proxy, container);
        let it = iterator(&mut e, proxy, data + 8, 0);
        let it_ptr = Ptr::<()>::new(it);
        e.call(0x00b4_2ce0, &args![it_ptr, 2u32]);
        assert_eq!(e.mem.u32(it + 4), data + 24);
        e.call(0x00b4_2ce0, &args![it_ptr, (-3i32) as u32]);
        assert_eq!(e.mem.u32(it + 4), data);
        assert!(panics(|| {
            fn_00b42ce0(&mut e, it_ptr, 5);
        }));
        assert!(panics(|| {
            fn_00b42ce0(&mut e, it_ptr, (-1i32) as u32);
        }));
    }

    #[test]
    fn pair_allocation_checks_the_overflow() {
        let mut e = engine();
        let sizes = spy(&mut e, OPERATOR_NEW, 0x4000);
        let messages = spy(&mut e, EXCEPTION_FROM_MESSAGE, 0);
        assert_eq!(e.call(0x00b4_2df0, &args![0u32, 0u32]).u32(), 0x4000);
        assert_eq!(e.call(0x00b4_2df0, &args![5u32, 0u32]).u32(), 0x4000);
        assert_eq!(
            e.call(0x00b4_2df0, &args![0x1FFF_FFFFu32, 0u32]).u32(),
            0x4000
        );
        assert_eq!(*sizes.borrow(), vec![vec![0], vec![40], vec![0xFFFF_FFF8]]);
        let thrown: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = thrown.clone();
        e.register_double(CXX_THROW_EXCEPTION, move |e, a| {
            sink.borrow_mut().push((e.mem.u32(a[0]), a[1]));
            panic!("thrown");
        });
        assert!(panics(|| {
            fn_00b42df0(&mut e, 0x2000_0000, 0);
        }));
        assert_eq!(
            *thrown.borrow(),
            vec![(BAD_ALLOC_VTABLE, BAD_ALLOC_THROW_INFO)]
        );
        assert_eq!(messages.borrow().len(), 1);
        assert_eq!(sizes.borrow().len(), 3);
    }

    #[test]
    fn word_range_copy() {
        let mut e = engine();
        let src = e.mem.alloc(16);
        let dst = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_u32(src + 4 * i, 10 + i);
        }
        e.call_log = Some(Vec::new());
        assert_eq!(
            e.call(0x00b4_2e50, &args![src, src + 12, dst]).u32(),
            dst + 12
        );
        assert_eq!(e.mem.u32(dst + 8), 12);
        // An empty or negative range copies nothing; the result adds the (signed) word count.
        assert_eq!(e.call(0x00b4_2e50, &args![src, src, dst]).u32(), dst);
        assert_eq!(
            e.call(0x00b4_2e50, &args![src + 8, src, dst + 8]).u32(),
            dst
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(log.iter().filter(|(a, _)| *a == MEMMOVE_S).count(), 1);
    }

    fn put_pairs(e: &mut Engine, at: u32, values: &[u32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(at + 8 * i as u32, *v);
            e.mem.set_u32(at + 8 * i as u32 + 4, *v + 100);
        }
    }

    fn pair_values(e: &Engine, at: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(at + 8 * i)).collect()
    }

    #[test]
    fn pair_fill_and_copy() {
        let mut e = engine();
        let value = e.mem.alloc(8);
        put_pairs(&mut e, value, &[7]);
        let dst = e.mem.alloc(8 * 4);
        e.call(0x00b4_2f40, &args![dst, 3u32, Ptr::<()>::new(value)]);
        assert_eq!(pair_values(&e, dst, 3), [7, 7, 7]);
        assert_eq!(e.mem.u32(dst + 20), 107);
        // A null destination skips the store.
        e.call(0x00b4_2f40, &args![0u32, 1u32, Ptr::<()>::new(value)]);
        // 00b43230 and its forwarders copy the range and return the end.
        let src = e.mem.alloc(8 * 4);
        put_pairs(&mut e, src, &[1, 2, 3]);
        let dst = e.mem.alloc(8 * 4);
        let r = e.call(0x00b4_3230, &args![src, src + 24, dst, 0u32, 0u32, 0u32]);
        assert_eq!(r.u32(), dst + 24);
        assert_eq!(pair_values(&e, dst, 3), [1, 2, 3]);
        assert_eq!(e.mem.u32(dst + 20), 103);
        for addr in [0x00b4_3010u32, 0x00b4_31e0] {
            let dst = e.mem.alloc(8 * 4);
            let r = e.call(addr, &args![src, src + 16, dst, 0u32]);
            assert_eq!(r.u32(), dst + 16);
            assert_eq!(pair_values(&e, dst, 2), [1, 2]);
        }
        let r = e.call(0x00b4_3230, &args![src, src + 8, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(r.u32(), 8);
    }

    #[test]
    fn pair_backward_copy() {
        let mut e = engine();
        let src = e.mem.alloc(8 * 4);
        put_pairs(&mut e, src, &[1, 2, 3]);
        let r = e.call(
            0x00b4_3090,
            &args![src, src + 24, src + 32, 0u32, 0u32, 0u32],
        );
        assert_eq!(r.u32(), src + 8);
        assert_eq!(pair_values(&e, src, 4), [1, 1, 2, 3]);
        assert_eq!(e.mem.u32(src + 28), 103);
        put_pairs(&mut e, src, &[1, 2, 3]);
        let r = e.call(0x00b4_2da0, &args![src, src + 24, src + 32]);
        assert_eq!(r.u32(), src + 8);
        assert_eq!(pair_values(&e, src, 4), [1, 1, 2, 3]);
    }
}
