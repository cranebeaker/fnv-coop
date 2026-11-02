//! `SpeedTree/run_00b0eca0` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! MSVC `std::vector` instantiations of the tree code, identified from the code (the engine map
//! gives them no names): the checked iterator of the vectors (a proxy word and an element
//! pointer), its comparisons and `+=` for 0x1C-, 4- and 1-byte elements, the `_Insert_n` of
//! the vectors of bytes, ints and 0x1C-byte structs with their allocation, fill, forward and
//! backward copy helpers, and the "vector<T> too long" throw.
//!
//! The vector is the PC's 0x18-byte `std::vector` ([`StlVector`]): first, last and end
//! pointers at `+0x0C`, `+0x10` and `+0x14`, and the allocator tag at `+8` (the address passed to
//! the copy helpers).

#[allow(unused_imports)]
use crate::prelude::*;

use super::branch::{StlVector, INVALID_PARAMETER};

/// `MemoryManager::Allocate` (`00aa3e40`, `this` = the manager, a size) and `Deallocate`
/// (`00aa4060`, `this` = the manager, the block); the manager singleton is at `011f6238`.
const ALLOCATE: u32 = 0x00aa_3e40;
const DEALLOCATE: u32 = 0x00aa_4060;
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `memmove_s(dest, dest_size, source, count)` (`00ec7ce3`, `cdecl`).
const MEMMOVE_S: u32 = 0x00ec_7ce3;
/// `_CxxThrowException(object, throw_info)` (`00ec7894`).
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// `std::exception::exception(const char *const &)` (`00ec78f1`, `thiscall`), the base of
/// `std::bad_alloc`, its vtable (`010176f8`) and its throw info (`01140ef8`).
const EXCEPTION_FROM_MESSAGE: u32 = 0x00ec_78f1;
const BAD_ALLOC_VTABLE: u32 = 0x0101_76f8;
const BAD_ALLOC_THROW_INFO: u32 = 0x0114_0ef8;
/// `std::string::string(const char *)` (`0044a4c0`, `thiscall`) and
/// `std::runtime_error::runtime_error(const std::string &)` (`0044a310`, `thiscall`).
const STRING_FROM_C_STRING: u32 = 0x0044_a4c0;
const RUNTIME_ERROR_FROM_STRING: u32 = 0x0044_a310;
/// `"vector<T> too long"`, the vtable of the `length_error` thrown for it and its throw info.
const VECTOR_TOO_LONG_TEXT: u32 = 0x0101_73c4;
const LENGTH_ERROR_VTABLE: u32 = 0x0101_73dc;
const VECTOR_TOO_LONG_THROW_INFO: u32 = 0x0114_0cb0;
/// The "vector<char> too long" thrower of the byte vector (`0065f640`, no arguments) and the
/// byte allocator `std::_Allocate<char>` (`0065fa30`, `cdecl`, a count and a hint).
const CHAR_VECTOR_TOO_LONG: u32 = 0x0065_f640;
const CHAR_ALLOCATE: u32 = 0x0065_fa30;
/// The copy helpers of the int vector outside this unit: the forward copy `00b177e0` (first,
/// last, dest, allocator), the fill `00b36b10` (dest, count, value pointer) and the backward
/// copy `00b16a60` (first, last, dest end); the fill-from-a-local `00b167e0` (`thiscall`).
const INT_COPY: u32 = 0x00b1_77e0;
const INT_FILL: u32 = 0x00b3_6b10;
const INT_COPY_BACKWARD: u32 = 0x00b1_6a60;
const INT_FILL_TAIL: u32 = 0x00b1_67e0;
/// The largest element counts of the three vectors (`max_size`): `0xFFFFFFFF` for bytes,
/// `0x3FFFFFFF` for ints and `0x9249249` for the 0x1C-byte elements.
const BYTE_MAX_COUNT: u32 = 0xFFFF_FFFF;
const INT_MAX_COUNT: u32 = 0x3FFF_FFFF;
const STRUCT_MAX_COUNT: u32 = 0x0924_9249;

/// The container a checked iterator belongs to: the first word of its proxy (`*proxy`), or 0
/// when the iterator has no proxy.
fn iterator_owner(e: &Engine, iterator: u32) -> u32 {
    let proxy = e.mem.u32(iterator);
    if proxy == 0 {
        0
    } else {
        e.mem.u32(proxy)
    }
}

/// The comparison check of the checked iterators: the invalid-parameter path runs unless the
/// iterator has a proxy and it is the proxy of `other` (an iterator, two words).
fn check_same_container(e: &mut Engine, iterator: u32, other: u32) {
    let proxy = e.mem.u32(iterator);
    if proxy == 0 || proxy != e.mem.u32(other) {
        e.call(INVALID_PARAMETER, &args![]);
    }
}

/// The checked `iterator += n` of the vector iterators, with `bytes` = `n * element size`:
/// without a proxy, or with a target outside `[first, last]` of the container, the
/// invalid-parameter path runs; the element pointer (`+4`) then advances.
pub(crate) fn advance_checked_iterator(e: &mut Engine, iterator: u32, bytes: u32) {
    if e.mem.u32(iterator) == 0 {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let target = e.mem.u32(iterator + 4).wrapping_add(bytes);
    let mut inside = false;
    let owner = iterator_owner(e, iterator);
    if target <= e.mem.u32(owner.wrapping_add(0x10)) {
        let owner = iterator_owner(e, iterator);
        inside = e.mem.u32(owner.wrapping_add(0x0C)) <= target;
    }
    if !inside {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let target = e.mem.u32(iterator + 4).wrapping_add(bytes);
    e.mem.set_u32(iterator + 4, target);
}

/// Throws a `std::bad_alloc` (the exception built from a pointer to a null `const char *`).
fn throw_bad_alloc(e: &mut Engine) -> ! {
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

/// The allocation of `count` elements of `size` bytes: a count whose byte size overflows
/// throws `std::bad_alloc`; a count of 0 allocates 0 bytes.
fn allocate_elements(e: &mut Engine, count: u32, size: u32) -> u32 {
    // A count of 0 skips the overflow test and allocates 0 bytes.
    if count != 0 && 0xFFFF_FFFFu32 / count < size {
        throw_bad_alloc(e);
    }
    e.call(ALLOCATE, &args![MEMORY_MANAGER, count.wrapping_mul(size)])
        .u32()
}

/// `MemoryManager::Deallocate` of a vector's old block.
fn deallocate(e: &mut Engine, block: u32) {
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, block]);
}

/// `memmove_s(dest, count, source, count)`.
fn memmove(e: &mut Engine, dest: u32, source: u32, count: u32) {
    e.call(MEMMOVE_S, &args![dest, count, source, count]);
}

/// Copies one 0x1C-byte element (`REP MOVSD`, seven words).
fn copy_struct(e: &mut Engine, dest: u32, source: u32) {
    for word in 0..7u32 {
        let value = e.mem.u32(source.wrapping_add(4 * word));
        e.mem.set_u32(dest.wrapping_add(4 * word), value);
    }
}

// Translated from 00b0eca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked vector iterator's `operator==` (identified from the code; `this` and the other
/// iterator are two words, a proxy and an element pointer): both must have the same nonzero
/// proxy, else the invalid-parameter path runs; true when the element pointers are equal.
pub fn fn_00b0eca0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    check_same_container(e, this.addr(), other.addr());
    e.mem.u32(this.addr() + 4) == e.mem.u32(other.addr() + 4)
}

// Translated from 00b0ecf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0ecf0 - the FS:[0] exception frame (unwinding of the new buffer if a
// copy throws) is not translated.
/// `std::vector<char>::_Insert_n(where, count, value)` (identified from the code; the first
/// argument is the iterator's proxy and is not read). Nothing happens for a count of 0; a count
/// above the room left reports a length error (`0065f640`). Without room for the new size the
/// buffer is replaced by one of at least 1.5 times the capacity: the head is copied, `count`
/// copies of `*value` are filled in, the tail is copied and the old buffer freed. With room the
/// tail moves up by `count` bytes: a tail shorter than `count` is copied up and the gap beyond
/// the old end is filled first, then `where..` is overwritten with `*value`; otherwise the last
/// `count` bytes are copied to the end, the rest moved up backwards and `where..` overwritten.
/// The value is copied to a local first, as it may point into the vector.
pub fn fn_00b0ecf0(
    e: &mut Engine,
    this: Ptr<StlVector>,
    _unused_1: u32,
    position: u32,
    count: u32,
    value: Ptr,
) {
    let vector = this.addr();
    let first = e.mem.u32(vector + 0x0C);
    let mut capacity = if first == 0 {
        0
    } else {
        e.mem.u32(vector + 0x14).wrapping_sub(first)
    };
    if count == 0 {
        return;
    }
    let size = e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C));
    if BYTE_MAX_COUNT.wrapping_sub(size) < count {
        e.call(CHAR_VECTOR_TOO_LONG, &args![]);
        return;
    }
    let size = e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C));
    if capacity < size.wrapping_add(count) {
        capacity = if BYTE_MAX_COUNT.wrapping_sub(capacity >> 1) < capacity {
            0
        } else {
            (capacity >> 1).wrapping_add(capacity)
        };
        let needed = e
            .mem
            .u32(vector + 0x10)
            .wrapping_sub(e.mem.u32(vector + 0x0C))
            .wrapping_add(count);
        if capacity < needed {
            capacity = needed;
        }
        let buffer = e.call(CHAR_ALLOCATE, &args![capacity, 0u32]).u32();
        let first = e.mem.u32(vector + 0x0C);
        let mut end = fn_00b10d60(e, first, position, buffer, vector + 8);
        fn_00b10cf0(e, end, count, value);
        end = end.wrapping_add(count);
        let last = e.mem.u32(vector + 0x10);
        fn_00b10d60(e, position, last, end, vector + 8);
        let new_size = e
            .mem
            .u32(vector + 0x10)
            .wrapping_sub(e.mem.u32(vector + 0x0C))
            .wrapping_add(count);
        let old_first = e.mem.u32(vector + 0x0C);
        if old_first != 0 {
            deallocate(e, old_first);
        }
        e.mem.set_u32(vector + 0x14, buffer.wrapping_add(capacity));
        e.mem.set_u32(vector + 0x10, buffer.wrapping_add(new_size));
        e.mem.set_u32(vector + 0x0C, buffer);
        return;
    }
    let last = e.mem.u32(vector + 0x10);
    if last.wrapping_sub(position) < count {
        let fill = e.mem.u8(value.addr());
        fn_00b10d60(e, position, last, position.wrapping_add(count), vector + 8);
        e.with_stack(4, |e, local| {
            e.mem.set_u8(local.addr(), fill);
            let last = e.mem.u32(vector + 0x10);
            let tail = last.wrapping_sub(position);
            fn_00b0f3c0(e, this.cast(), last, count.wrapping_sub(tail), local);
        });
        let grown = e.mem.u32(vector + 0x10).wrapping_add(count);
        e.mem.set_u32(vector + 0x10, grown);
        let end = e.mem.u32(vector + 0x10).wrapping_sub(count);
        let mut at = position;
        while at != end {
            e.mem.set_u8(at, fill);
            at = at.wrapping_add(1);
        }
    } else {
        let fill = e.mem.u8(value.addr());
        let moved = fn_00b10a60(
            e,
            last.wrapping_sub(count),
            last,
            e.mem.u32(vector + 0x10),
            vector + 8,
        );
        e.mem.set_u32(vector + 0x10, moved);
        fn_00b107a0(e, position, last.wrapping_sub(count), last);
        let end = position.wrapping_add(count);
        let mut at = position;
        while at != end {
            e.mem.set_u8(at, fill);
            at = at.wrapping_add(1);
        }
    }
}

// Translated from 00b0f3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills `count` bytes at `dest` with the byte at `value` (`00b10cf0`) and returns the end,
/// `dest + count` (identified from the code: the `_Uninit_fill_n` of the byte vector; `this`
/// is not read).
pub fn fn_00b0f3c0(e: &mut Engine, _this: Ptr, dest: u32, count: u32, value: Ptr) -> u32 {
    fn_00b10cf0(e, dest, count, value);
    dest.wrapping_add(count)
}

// Translated from 00b0f410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0f410 - the FS:[0] exception frame (unwinding of the new buffer if a
// copy throws) is not translated.
/// `std::vector<int>::_Insert_n(where, count, value)` (identified from the code; the first
/// argument is the iterator's proxy and is not read): the same algorithm as the byte vector's
/// (`00b0ecf0`) over 4-byte elements: a count above the room left reports a length error
/// (`00b0fb80`), growth allocates through `00b10830` and copies with `00b177e0` and the fill
/// `00b36b10`, the shorter-tail case fills the gap with `00b167e0` and the other case copies
/// the last elements to the end, then moves the rest up with `00b16a60`.
pub fn fn_00b0f410(
    e: &mut Engine,
    this: Ptr<StlVector>,
    _unused_1: u32,
    position: u32,
    count: u32,
    value: Ptr,
) {
    let vector = this.addr();
    let first = e.mem.u32(vector + 0x0C);
    let mut capacity = if first == 0 {
        0
    } else {
        ((e.mem.u32(vector + 0x14).wrapping_sub(first)) as i32 >> 2) as u32
    };
    if count == 0 {
        return;
    }
    let size = ((e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C))) as i32
        >> 2) as u32;
    if INT_MAX_COUNT.wrapping_sub(size) < count {
        fn_00b0fb80(e);
        return;
    }
    if capacity < size.wrapping_add(count) {
        capacity = if INT_MAX_COUNT.wrapping_sub(capacity >> 1) < capacity {
            0
        } else {
            (capacity >> 1).wrapping_add(capacity)
        };
        let needed = (((e
            .mem
            .u32(vector + 0x10)
            .wrapping_sub(e.mem.u32(vector + 0x0C))) as i32
            >> 2) as u32)
            .wrapping_add(count);
        if capacity < needed {
            capacity = needed;
        }
        let buffer = fn_00b10830(e, capacity, 0);
        let first = e.mem.u32(vector + 0x0C);
        let mut end = e
            .call(INT_COPY, &args![first, position, buffer, vector + 8])
            .u32();
        e.call(INT_FILL, &args![end, count, value]);
        end = end.wrapping_add(count.wrapping_mul(4));
        let last = e.mem.u32(vector + 0x10);
        e.call(INT_COPY, &args![position, last, end, vector + 8]);
        let new_size = (((e
            .mem
            .u32(vector + 0x10)
            .wrapping_sub(e.mem.u32(vector + 0x0C))) as i32
            >> 2) as u32)
            .wrapping_add(count);
        let old_first = e.mem.u32(vector + 0x0C);
        if old_first != 0 {
            deallocate(e, old_first);
        }
        e.mem
            .set_u32(vector + 0x14, buffer.wrapping_add(capacity.wrapping_mul(4)));
        e.mem
            .set_u32(vector + 0x10, buffer.wrapping_add(new_size.wrapping_mul(4)));
        e.mem.set_u32(vector + 0x0C, buffer);
        return;
    }
    let last = e.mem.u32(vector + 0x10);
    let tail = (last.wrapping_sub(position) as i32 >> 2) as u32;
    if tail < count {
        let fill = e.mem.u32(value.addr());
        let span = count.wrapping_mul(4);
        e.call(
            INT_COPY,
            &args![position, last, position.wrapping_add(span), vector + 8],
        );
        e.with_stack(4, |e, local| {
            e.mem.set_u32(local.addr(), fill);
            let last = e.mem.u32(vector + 0x10);
            let tail = (last.wrapping_sub(position) as i32 >> 2) as u32;
            e.call(
                INT_FILL_TAIL,
                &args![this, last, count.wrapping_sub(tail), local],
            );
        });
        let grown = e.mem.u32(vector + 0x10).wrapping_add(span);
        e.mem.set_u32(vector + 0x10, grown);
        let end = e.mem.u32(vector + 0x10).wrapping_sub(span);
        let mut at = position;
        while at != end {
            e.mem.set_u32(at, fill);
            at = at.wrapping_add(4);
        }
    } else {
        let fill = e.mem.u32(value.addr());
        let span = count.wrapping_mul(4);
        let moved_from = last.wrapping_sub(span);
        let moved = e
            .call(
                INT_COPY,
                &args![moved_from, last, e.mem.u32(vector + 0x10), vector + 8],
            )
            .u32();
        e.mem.set_u32(vector + 0x10, moved);
        e.call(INT_COPY_BACKWARD, &args![position, moved_from, last]);
        let end = position.wrapping_add(span);
        let mut at = position;
        while at != end {
            e.mem.set_u32(at, fill);
            at = at.wrapping_add(4);
        }
    }
}

// Translated from 00b0fb80 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0fb80 - the FS:[0] exception frame (destroying the message string
// while the exception propagates) is not translated.
/// The "vector<T> too long" throw (identified from the code): builds the string, a
/// `std::runtime_error` (`0044a310`) from it with the `length_error` vtable stored over it, and
/// throws it with `_CxxThrowException`. Does not return.
pub fn fn_00b0fb80(e: &mut Engine) {
    e.with_stack(0x1C, |e, text| {
        e.call(STRING_FROM_C_STRING, &args![text, VECTOR_TOO_LONG_TEXT]);
        // The length_error is 0x28 bytes: the 0xc-byte std::exception (00ec78e0) and the 0x1c-byte
        // message string 0044a310 builds at +0xc.
        e.with_stack(0x28, |e, error| {
            e.call(RUNTIME_ERROR_FROM_STRING, &args![error, text]);
            e.mem.set_u32(error.addr(), LENGTH_ERROR_VTABLE);
            e.call(
                CXX_THROW_EXCEPTION,
                &args![error, VECTOR_TOO_LONG_THROW_INFO],
            );
        });
    });
    unreachable!("_CxxThrowException does not return (00ec7894)")
}

// Translated from 00b0fc00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b0fc00 - the FS:[0] exception frame (unwinding of the new buffer if a
// copy throws) is not translated.
/// `std::vector<T>::_Insert_n(where, count, value)` for the 0x1C-byte element (identified from
/// the code; the first argument is the iterator's proxy and is not read): the algorithm of the
/// byte vector's (`00b0ecf0`) over 0x1C-byte elements, with the copies `00b10940` (the forward
/// copy), `00b10990` (fill), `00b10b00`/`00b10890` (the backward moves) and the allocation
/// `00b108e0`; `value` is copied to a local first. The old buffer's elements are walked (the
/// element destructor is empty) before it is freed.
pub fn fn_00b0fc00(
    e: &mut Engine,
    this: Ptr<StlVector>,
    _unused_1: u32,
    position: u32,
    count: u32,
    value: Ptr,
) {
    let vector = this.addr();
    let first = e.mem.u32(vector + 0x0C);
    let mut capacity = if first == 0 {
        0
    } else {
        ((e.mem.u32(vector + 0x14).wrapping_sub(first)) as i32 / 0x1C) as u32
    };
    if count == 0 {
        return;
    }
    let size = ((e
        .mem
        .u32(vector + 0x10)
        .wrapping_sub(e.mem.u32(vector + 0x0C))) as i32
        / 0x1C) as u32;
    if STRUCT_MAX_COUNT.wrapping_sub(size) < count {
        fn_00b0fb80(e);
        return;
    }
    if capacity < size.wrapping_add(count) {
        capacity = if STRUCT_MAX_COUNT.wrapping_sub(capacity >> 1) < capacity {
            0
        } else {
            (capacity >> 1).wrapping_add(capacity)
        };
        let needed = (((e
            .mem
            .u32(vector + 0x10)
            .wrapping_sub(e.mem.u32(vector + 0x0C))) as i32
            / 0x1C) as u32)
            .wrapping_add(count);
        if capacity < needed {
            capacity = needed;
        }
        let buffer = fn_00b108e0(e, capacity, 0);
        let first = e.mem.u32(vector + 0x0C);
        let mut end = fn_00b10940(e, first, position, buffer, vector + 8);
        // The byte after the allocator is an uninitialised stack byte in the exe; zero here.
        fn_00b10990(e, end, count, value, vector + 8, 0, 0);
        end = count.wrapping_mul(0x1C).wrapping_add(end);
        let last = e.mem.u32(vector + 0x10);
        fn_00b10940(e, position, last, end, vector + 8);
        let new_size = (((e
            .mem
            .u32(vector + 0x10)
            .wrapping_sub(e.mem.u32(vector + 0x0C))) as i32
            / 0x1C) as u32)
            .wrapping_add(count);
        let old_first = e.mem.u32(vector + 0x0C);
        if old_first != 0 {
            let mut at = e.mem.u32(vector + 0x0C);
            while at != e.mem.u32(vector + 0x10) {
                at = at.wrapping_add(0x1C);
            }
            deallocate(e, old_first);
        }
        e.mem.set_u32(
            vector + 0x14,
            capacity.wrapping_mul(0x1C).wrapping_add(buffer),
        );
        e.mem.set_u32(
            vector + 0x10,
            new_size.wrapping_mul(0x1C).wrapping_add(buffer),
        );
        e.mem.set_u32(vector + 0x0C, buffer);
        return;
    }
    let last = e.mem.u32(vector + 0x10);
    let tail = ((last.wrapping_sub(position)) as i32 / 0x1C) as u32;
    let span = count.wrapping_mul(0x1C);
    e.with_stack(0x1C, |e, local| {
        copy_struct(e, local.addr(), value.addr());
        if tail < count {
            fn_00b10940(e, position, last, position.wrapping_add(span), vector + 8);
            let last = e.mem.u32(vector + 0x10);
            let tail = ((last.wrapping_sub(position)) as i32 / 0x1C) as u32;
            fn_00b10990(e, last, count.wrapping_sub(tail), local, vector + 8, 0, 0);
            let grown = e.mem.u32(vector + 0x10).wrapping_add(span);
            e.mem.set_u32(vector + 0x10, grown);
            let end = e.mem.u32(vector + 0x10).wrapping_sub(span);
            let mut at = position;
            while at != end {
                copy_struct(e, at, local.addr());
                at = at.wrapping_add(0x1C);
            }
        } else {
            let moved_from = last.wrapping_sub(span);
            // The two tag bytes after the allocator: an uninitialised stack byte and a zero.
            let moved = fn_00b10b00(e, moved_from, last, e.mem.u32(vector + 0x10), vector + 8);
            e.mem.set_u32(vector + 0x10, moved);
            fn_00b10890(e, position, moved_from, last, 0, 0, 0);
            let end = position.wrapping_add(span);
            let mut at = position;
            while at != end {
                copy_struct(e, at, local.addr());
                at = at.wrapping_add(0x1C);
            }
        }
    });
}

// Translated from 00b10430 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked vector iterator's constructor (identified from the code; the arguments are the
/// element pointer and the vector): the vector must exist and `first <= position <= last`
/// (else the invalid-parameter path runs); stores the vector's proxy (`+0`) and `position`.
/// Returns `this`.
pub fn fn_00b10430(e: &mut Engine, this: Ptr, position: u32, vector: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    let vector = vector.addr();
    if vector == 0 || e.mem.u32(vector + 0x0C) > position || position > e.mem.u32(vector + 0x10) {
        e.call(INVALID_PARAMETER, &args![]);
    }
    let proxy = e.mem.u32(vector);
    e.mem.set_u32(this.addr(), proxy);
    e.mem.set_u32(this.addr() + 4, position);
    this
}

// Translated from 00b10490 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked vector iterator's `operator!=` (identified from the code): both must have the
/// same nonzero proxy, else the invalid-parameter path runs; true when the element pointers
/// differ.
pub fn fn_00b10490(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    check_same_container(e, this.addr(), other.addr());
    e.mem.u32(this.addr() + 4) != e.mem.u32(other.addr() + 4)
}

// Translated from 00b104f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked iterator's `operator+=(n)` for 0x1C-byte elements (identified from the code):
/// moves the element pointer by `n * 0x1C` bytes; an iterator without a proxy, or a target
/// outside the container's `[first, last]`, takes the invalid-parameter path. Returns `this`.
pub fn fn_00b104f0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    advance_checked_iterator(e, this.addr(), count.wrapping_mul(0x1C));
    this
}

// Translated from 00b105b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked iterator's `operator+=(n)` for 4-byte elements (identified from the code; see
/// `00b104f0`). Returns `this`.
pub fn fn_00b105b0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    advance_checked_iterator(e, this.addr(), count.wrapping_mul(4));
    this
}

// Translated from 00b10670 (decompiled, FalloutNV.exe 1.4.0.525)
/// The checked iterator's `operator+=(n)` for 1-byte elements (identified from the code; see
/// `00b104f0`). Returns `this`.
pub fn fn_00b10670(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    advance_checked_iterator(e, this.addr(), count);
    this
}

// Translated from 00b10730 (decompiled, FalloutNV.exe 1.4.0.525)
/// A byte range copy (identified from the code; `cdecl`, `first`, `last`, `dest`): when
/// `last - first` is positive (signed) the bytes are moved with `memmove_s`; returns `dest`
/// plus that length.
pub fn fn_00b10730(e: &mut Engine, first: u32, last: u32, dest: u32) -> u32 {
    let length = last.wrapping_sub(first);
    if (length as i32) > 0 {
        memmove(e, dest, first, length);
    }
    dest.wrapping_add(length)
}

// Translated from 00b107a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A backward byte range copy (identified from the code; `cdecl`, `first`, `last`, `dest_end`):
/// the range `first..last` is moved (`memmove_s`, when its length is positive as a signed
/// number) so that it ends at `dest_end`; returns the start of the copy, `dest_end` minus the
/// length.
pub fn fn_00b107a0(e: &mut Engine, first: u32, last: u32, dest_end: u32) -> u32 {
    let length = last.wrapping_sub(first);
    let dest = dest_end.wrapping_sub(length);
    if (length as i32) > 0 {
        memmove(e, dest, first, length);
    }
    dest
}

// Translated from 00b10830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The int vector's allocation (identified from the code; `std::_Allocate<int>(count, hint)`,
/// the hint unused): `MemoryManager::Allocate(count * 4)`. A count of 0 allocates 0 bytes; a
/// count too large for the multiplication throws `std::bad_alloc`.
pub fn fn_00b10830(e: &mut Engine, count: u32, _unused_1: u32) -> u32 {
    allocate_elements(e, count, 4)
}

// Translated from 00b10890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards `first`, `last`, `dest_end` to the backward struct copy `00b10b80` (identified from
/// the code; `cdecl`) with three tag bytes, uninitialised stack bytes in the exe and zero here.
/// Returns what it returns.
pub fn fn_00b10890(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest_end: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    fn_00b10b80(e, first, last, dest_end, 0, 0, 0)
}

// Translated from 00b108e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocation of the 0x1C-byte element vector (identified from the code;
/// `std::_Allocate(count, hint)`, the hint unused): `MemoryManager::Allocate(count * 0x1C)`. A
/// count of 0 allocates 0 bytes; a count too large for the multiplication throws
/// `std::bad_alloc`.
pub fn fn_00b108e0(e: &mut Engine, count: u32, _unused_1: u32) -> u32 {
    allocate_elements(e, count, 0x1C)
}

// Translated from 00b10940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards a range copy-construct of 0x1C-byte elements to `00b10c20` (identified from the
/// code; `cdecl`, `first`, `last`, `dest`, the allocator) with two tag bytes, uninitialised
/// stack bytes in the exe and zero here. Returns its result, the end of the copy.
pub fn fn_00b10940(e: &mut Engine, first: u32, last: u32, dest: u32, allocator: u32) -> u32 {
    fn_00b10c20(e, first, last, dest, allocator, 0, 0)
}

// Translated from 00b10990 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b10990 - the FS:[0] exception frame is not translated.
/// `std::_Uninit_fill_n` for the 0x1C-byte element (identified from the code; `cdecl`):
/// copy-constructs `count` copies of `*value` at `dest`, 0x1C bytes apart; a null destination
/// is skipped. The allocator and the two tag bytes are pushed by the callers and not read.
pub fn fn_00b10990(
    e: &mut Engine,
    dest: u32,
    count: u32,
    value: Ptr,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
    let mut dest = dest;
    let mut count = count;
    while count != 0 {
        if dest != 0 {
            copy_struct(e, dest, value.addr());
        }
        count = count.wrapping_sub(1);
        dest = dest.wrapping_add(0x1C);
    }
}

// Translated from 00b10a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A byte range copy (identified from the code; `cdecl`, `first`, `last`, `dest`, the allocator
/// word is not read): when the length is not 0 (unsigned) the bytes are moved with
/// `memmove_s`; returns `dest` plus the length.
pub fn fn_00b10a60(e: &mut Engine, first: u32, last: u32, dest: u32, _unused_3: u32) -> u32 {
    let length = last.wrapping_sub(first);
    if length != 0 {
        memmove(e, dest, first, length);
    }
    dest.wrapping_add(length)
}

// Translated from 00b10b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards a range copy-construct of 0x1C-byte elements to `00b10c20` (identified from the
/// code; the same forwarder as `00b10940`, built through one more layer of inlined helpers;
/// `cdecl`, `first`, `last`, `dest`, the allocator). Returns its result.
pub fn fn_00b10b00(e: &mut Engine, first: u32, last: u32, dest: u32, allocator: u32) -> u32 {
    fn_00b10c20(e, first, last, dest, allocator, 0, 0)
}

// Translated from 00b10b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The backward copy of 0x1C-byte elements (identified from the code; `cdecl`, `first`, `last`,
/// `dest_end`, three tag bytes not read): copies the elements from the last one down to the
/// first so that the range ends at `dest_end`. Returns the start of the copy, `dest_end` minus
/// the whole elements' size.
pub fn fn_00b10b80(
    e: &mut Engine,
    first: u32,
    last: u32,
    dest_end: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let start = dest_end
        .wrapping_sub((((last.wrapping_sub(first)) as i32 / 0x1C) as u32).wrapping_mul(0x1C));
    let mut source = last;
    let mut dest = dest_end;
    while first != source {
        source = source.wrapping_sub(0x1C);
        dest = dest.wrapping_sub(0x1C);
        copy_struct(e, dest, source);
    }
    start
}

// Translated from 00b10c20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b10c20 - the FS:[0] exception frame is not translated.
/// `std::_Uninit_copy` for the 0x1C-byte element (identified from the code; `cdecl`, `first`,
/// `last`, `dest`, the allocator and two tag bytes not read): copy-constructs each element of
/// `first..last` at `dest`, 0x1C bytes apart; a null destination is skipped. Returns the end of
/// the destination.
pub fn fn_00b10c20(
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
            copy_struct(e, dest, source);
        }
        dest = dest.wrapping_add(0x1C);
        source = source.wrapping_add(0x1C);
    }
    dest
}

// Translated from 00b10cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte fill (identified from the code; `cdecl`, `dest`, `count`, `value`): stores the byte
/// at `value` (read again for each store) into `count` bytes from `dest`.
pub fn fn_00b10cf0(e: &mut Engine, dest: u32, count: u32, value: Ptr) {
    let mut count = count;
    let mut at = dest;
    while count != 0 {
        let byte = e.mem.u8(value.addr());
        e.mem.set_u8(at, byte);
        count = count.wrapping_sub(1);
        at = at.wrapping_add(1);
    }
}

// Translated from 00b10d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A byte range copy (identified from the code; `cdecl`, `first`, `last`, `dest`, the allocator
/// word is not read): when the length is not 0 (unsigned) the bytes are moved with
/// `memmove_s`; returns `dest` plus the length.
pub fn fn_00b10d60(e: &mut Engine, first: u32, last: u32, dest: u32, _unused_3: u32) -> u32 {
    let length = last.wrapping_sub(first);
    if length != 0 {
        memmove(e, dest, first, length);
    }
    dest.wrapping_add(length)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b0eca0, fn_00b0eca0(Ptr, Ptr) -> bool),
        entry!(0x00b0ecf0, fn_00b0ecf0(Ptr<StlVector>, u32, u32, u32, Ptr)),
        entry!(0x00b0f3c0, fn_00b0f3c0(Ptr, u32, u32, Ptr) -> u32),
        entry!(0x00b0f410, fn_00b0f410(Ptr<StlVector>, u32, u32, u32, Ptr)),
        entry!(0x00b0fb80, fn_00b0fb80()),
        entry!(0x00b0fc00, fn_00b0fc00(Ptr<StlVector>, u32, u32, u32, Ptr)),
        entry!(0x00b10430, fn_00b10430(Ptr, u32, Ptr) -> Ptr),
        entry!(0x00b10490, fn_00b10490(Ptr, Ptr) -> bool),
        entry!(0x00b104f0, fn_00b104f0(Ptr, u32) -> Ptr),
        entry!(0x00b105b0, fn_00b105b0(Ptr, u32) -> Ptr),
        entry!(0x00b10670, fn_00b10670(Ptr, u32) -> Ptr),
        entry!(0x00b10730, fn_00b10730(u32, u32, u32) -> u32),
        entry!(0x00b107a0, fn_00b107a0(u32, u32, u32) -> u32),
        entry!(0x00b10830, fn_00b10830(u32, u32) -> u32),
        entry!(0x00b10890, fn_00b10890(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b108e0, fn_00b108e0(u32, u32) -> u32),
        entry!(0x00b10940, fn_00b10940(u32, u32, u32, u32) -> u32),
        entry!(0x00b10990, fn_00b10990(u32, u32, Ptr, u32, u32, u32)),
        entry!(0x00b10a60, fn_00b10a60(u32, u32, u32, u32) -> u32),
        entry!(0x00b10b00, fn_00b10b00(u32, u32, u32, u32) -> u32),
        entry!(0x00b10b80, fn_00b10b80(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b10c20, fn_00b10c20(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00b10cf0, fn_00b10cf0(u32, u32, Ptr)),
        entry!(0x00b10d60, fn_00b10d60(u32, u32, u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::panic::{catch_unwind, AssertUnwindSafe};
    use std::rc::Rc;

    /// The proxy word every test vector carries at `+0`.
    const PROXY: u32 = 0x5000_0001;

    type Words = Rc<RefCell<Vec<Vec<u32>>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    /// An engine with the exe's data range mapped, a panicking invalid-parameter path and
    /// throw, and `memmove_s` that really moves bytes.
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

    /// Allocation doubles: the manager's `Allocate` and the byte allocator hand out heap blocks
    /// (their arguments are logged), `Deallocate` logs its arguments.
    fn allocators(e: &mut Engine) -> (Words, Words) {
        let sizes: Words = Rc::new(RefCell::new(Vec::new()));
        let sink = sizes.clone();
        e.register_double(ALLOCATE, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(e.mem.alloc(a[1].clamp(1, 0x1000)))
        });
        let sink = sizes.clone();
        e.register_double(CHAR_ALLOCATE, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(e.mem.alloc(a[0].max(1)))
        });
        let freed = spy(e, DEALLOCATE, 0);
        (sizes, freed)
    }

    /// A vector of `size`-byte elements with `count` used and room for `capacity`; returns the
    /// vector and its data.
    fn vector(e: &mut Engine, size: u32, count: u32, capacity: u32) -> (u32, u32) {
        let at = e.mem.alloc(0x18);
        let data = e.mem.alloc((size * capacity).max(1));
        e.mem.set_u32(at, PROXY);
        e.mem.set_u32(at + 0x0C, data);
        e.mem.set_u32(at + 0x10, data + size * count);
        e.mem.set_u32(at + 0x14, data + size * capacity);
        (at, data)
    }

    /// A checked iterator at `position` of the container `vector`.
    fn iterator(e: &mut Engine, vector: u32, position: u32) -> u32 {
        let proxy = e.mem.alloc(4);
        e.mem.set_u32(proxy, vector);
        let at = e.mem.alloc(8);
        e.mem.set_u32(at, proxy);
        e.mem.set_u32(at + 4, position);
        at
    }

    fn bytes_of(e: &Engine, vector: u32) -> Vec<u8> {
        let (first, last) = (e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10));
        e.mem.bytes(first, last - first)
    }

    fn ints_of(e: &Engine, vector: u32) -> Vec<u32> {
        let (first, last) = (e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10));
        (0..(last - first) / 4)
            .map(|i| e.mem.u32(first + 4 * i))
            .collect()
    }

    /// The first word of each 0x1C-byte element (the test elements repeat one tag).
    fn tags_of(e: &Engine, vector: u32) -> Vec<u32> {
        let (first, last) = (e.mem.u32(vector + 0x0C), e.mem.u32(vector + 0x10));
        (0..(last - first) / 0x1C)
            .map(|i| e.mem.u32(first + 0x1C * i))
            .collect()
    }

    fn put_tags(e: &mut Engine, at: u32, tags: &[u32]) {
        for (i, tag) in tags.iter().enumerate() {
            for word in 0..7 {
                e.mem.set_u32(at + 0x1C * i as u32 + 4 * word, *tag);
            }
        }
    }

    fn put_ints(e: &mut Engine, at: u32, values: &[u32]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_u32(at + 4 * i as u32, *v);
        }
    }

    #[test]
    fn iterator_comparisons_need_the_same_proxy() {
        let mut e = engine();
        let (v, data) = vector(&mut e, 4, 4, 4);
        let a = iterator(&mut e, v, data);
        let b = e.mem.alloc(8);
        e.mem.set_u32(b, e.mem.u32(a));
        e.mem.set_u32(b + 4, data);
        let both = args![Ptr::<()>::new(a), Ptr::<()>::new(b)];
        assert!(e.call(0x00b0_eca0, &both).bool());
        assert!(!e.call(0x00b1_0490, &both).bool());
        e.mem.set_u32(b + 4, data + 4);
        assert!(!e.call(0x00b0_eca0, &both).bool());
        assert!(e.call(0x00b1_0490, &both).bool());
        // A different proxy, or none, is an invalid parameter.
        let other = iterator(&mut e, v, data);
        assert!(panics(|| {
            fn_00b0eca0(&mut e, Ptr::new(a), Ptr::new(other));
        }));
        e.mem.set_u32(a, 0);
        e.mem.set_u32(other, 0);
        assert!(panics(|| {
            fn_00b10490(&mut e, Ptr::new(a), Ptr::new(other));
        }));
    }

    #[test]
    fn iterator_constructor_checks_the_range() {
        let mut e = engine();
        let (v, data) = vector(&mut e, 4, 3, 4);
        let it = e.mem.alloc(8);
        e.mem.set_u32(it, 0xDEAD);
        let r = e.call(
            0x00b1_0430,
            &args![Ptr::<()>::new(it), data + 8, Ptr::<()>::new(v)],
        );
        assert_eq!(r.u32(), it);
        assert_eq!((e.mem.u32(it), e.mem.u32(it + 4)), (PROXY, data + 8));
        // The end is allowed; one element beyond, one before and no vector are not.
        e.call(
            0x00b1_0430,
            &args![Ptr::<()>::new(it), data + 12, Ptr::<()>::new(v)],
        );
        assert_eq!(e.mem.u32(it + 4), data + 12);
        for position in [data + 16, data - 4] {
            assert!(panics(|| {
                fn_00b10430(&mut e, Ptr::new(it), position, Ptr::new(v));
            }));
        }
        assert!(panics(|| {
            fn_00b10430(&mut e, Ptr::new(it), data, Ptr::NULL);
        }));
    }

    #[test]
    fn iterator_advance_by_element_size() {
        for (addr, size) in [
            (0x00b1_04f0u32, 0x1Cu32),
            (0x00b1_05b0, 4),
            (0x00b1_0670, 1),
        ] {
            let mut e = engine();
            let (v, data) = vector(&mut e, size, 5, 5);
            let it = iterator(&mut e, v, data + size);
            let r = e.call(addr, &args![Ptr::<()>::new(it), 3u32]);
            assert_eq!(r.u32(), it);
            assert_eq!(e.mem.u32(it + 4), data + 4 * size);
            // Back to the start (a negative count), then to the end exactly.
            e.call(addr, &args![Ptr::<()>::new(it), (-4i32) as u32]);
            assert_eq!(e.mem.u32(it + 4), data);
            e.call(addr, &args![Ptr::<()>::new(it), 5u32]);
            assert_eq!(e.mem.u32(it + 4), data + 5 * size);
            // Past the end, before the start, and without a proxy are invalid.
            assert!(panics(|| {
                e.call(addr, &args![Ptr::<()>::new(it), 1u32]);
            }));
            e.mem.set_u32(it + 4, data);
            assert!(panics(|| {
                e.call(addr, &args![Ptr::<()>::new(it), (-1i32) as u32]);
            }));
            e.mem.set_u32(it, 0);
            assert!(panics(|| {
                e.call(addr, &args![Ptr::<()>::new(it), 1u32]);
            }));
        }
    }

    #[test]
    fn byte_ranges_are_moved_with_memmove() {
        let mut e = engine();
        let src = e.mem.alloc(16);
        let dst = e.mem.alloc(16);
        e.mem.write(src, b"abcdefgh");
        e.call_log = Some(Vec::new());
        // 00b10730: positive length (signed) moves and returns the end of the destination.
        assert_eq!(
            e.call(0x00b1_0730, &args![src, src + 4, dst]).u32(),
            dst + 4
        );
        assert_eq!(e.mem.bytes(dst, 4), b"abcd");
        // A negative length neither moves nor stops: the result is dest plus the length.
        assert_eq!(
            e.call(0x00b1_0730, &args![src + 4, src, dst]).u32(),
            dst.wrapping_sub(4)
        );
        // 00b10a60 and 00b10d60 move any nonzero length (unsigned) and skip zero.
        for addr in [0x00b1_0a60u32, 0x00b1_0d60] {
            assert_eq!(
                e.call(addr, &args![src + 2, src + 6, dst, 0x77u32]).u32(),
                dst + 4
            );
            assert_eq!(e.mem.bytes(dst, 4), b"cdef");
            assert_eq!(e.call(addr, &args![src, src, dst + 8, 0u32]).u32(), dst + 8);
        }
        let log = e.call_log.take().unwrap();
        let moves = log.iter().filter(|(a, _)| *a == MEMMOVE_S).count();
        assert_eq!(moves, 3);
        // 00b107a0: the range ends at `dest_end`; the result is its start.
        assert_eq!(
            e.call(0x00b1_07a0, &args![src, src + 3, dst + 10]).u32(),
            dst + 7
        );
        assert_eq!(e.mem.bytes(dst + 7, 3), b"abc");
        assert_eq!(
            e.call(0x00b1_07a0, &args![src + 3, src, dst + 10]).u32(),
            dst + 13
        );
    }

    #[test]
    fn byte_fill_stores_the_value_count_times() {
        let mut e = engine();
        let dst = e.mem.alloc(8);
        let value = e.mem.alloc(1);
        e.mem.set_u8(value, 0x5A);
        e.call(0x00b1_0cf0, &args![dst, 3u32, Ptr::<()>::new(value)]);
        assert_eq!(e.mem.bytes(dst, 4), [0x5A, 0x5A, 0x5A, 0]);
        e.call(0x00b1_0cf0, &args![dst + 4, 0u32, Ptr::<()>::new(value)]);
        assert_eq!(e.mem.u8(dst + 4), 0);
        // 00b0f3c0 fills and returns the end.
        let r = e.call(
            0x00b0_f3c0,
            &args![Ptr::<()>::NULL, dst + 4, 2u32, Ptr::<()>::new(value)],
        );
        assert_eq!(r.u32(), dst + 6);
        assert_eq!(e.mem.bytes(dst + 4, 3), [0x5A, 0x5A, 0]);
    }

    #[test]
    fn allocations_check_the_overflow() {
        let mut e = engine();
        let (sizes, _) = allocators(&mut e);
        e.register(EXCEPTION_FROM_MESSAGE, |_, a| ret(a[0]));
        for (addr, size, largest) in [
            (0x00b1_0830u32, 4u32, 0x3FFF_FFFFu32),
            (0x00b1_08e0, 0x1C, 0x0924_9249),
        ] {
            sizes.borrow_mut().clear();
            assert_ne!(e.call(addr, &args![0u32, 0u32]).u32(), 0);
            assert_ne!(e.call(addr, &args![3u32, 0u32]).u32(), 0);
            assert_ne!(e.call(addr, &args![largest, 0u32]).u32(), 0);
            let words = sizes.borrow().clone();
            assert_eq!(
                words,
                vec![
                    vec![MEMORY_MANAGER, 0],
                    vec![MEMORY_MANAGER, 3 * size],
                    vec![MEMORY_MANAGER, largest * size],
                ]
            );
            assert!(panics(|| {
                e.call(addr, &args![largest + 1, 0u32]);
            }));
        }
    }

    #[test]
    fn bad_alloc_is_built_from_a_null_message() {
        let mut e = engine();
        allocators(&mut e);
        let seen: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let messages = spy(&mut e, EXCEPTION_FROM_MESSAGE, 0);
        let sink = seen.clone();
        e.register_double(CXX_THROW_EXCEPTION, move |e, a| {
            sink.borrow_mut().push((e.mem.u32(a[0]), a[1]));
            panic!("thrown");
        });
        assert!(panics(|| {
            fn_00b10830(&mut e, 0x4000_0000, 0);
        }));
        assert_eq!(
            *seen.borrow(),
            vec![(BAD_ALLOC_VTABLE, BAD_ALLOC_THROW_INFO)]
        );
        assert_eq!(messages.borrow().len(), 1);
    }

    #[test]
    fn too_long_throws_a_length_error() {
        let mut e = engine();
        let strings = spy(&mut e, STRING_FROM_C_STRING, 0);
        let errors = spy(&mut e, RUNTIME_ERROR_FROM_STRING, 0);
        let seen: Rc<RefCell<Vec<(u32, u32)>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = seen.clone();
        e.register_double(CXX_THROW_EXCEPTION, move |e, a| {
            sink.borrow_mut().push((e.mem.u32(a[0]), a[1]));
            panic!("thrown");
        });
        assert!(panics(|| {
            e.call(0x00b0_fb80, &args![]);
        }));
        assert_eq!(strings.borrow().len(), 1);
        assert_eq!(strings.borrow()[0][1], VECTOR_TOO_LONG_TEXT);
        // The error is built from the string the first call filled.
        assert_eq!(errors.borrow().len(), 1);
        assert_eq!(errors.borrow()[0][1], strings.borrow()[0][0]);
        assert_eq!(
            *seen.borrow(),
            vec![(LENGTH_ERROR_VTABLE, VECTOR_TOO_LONG_THROW_INFO)]
        );
    }

    #[test]
    fn struct_elements_are_copied_forward_and_backward() {
        let mut e = engine();
        let src = e.mem.alloc(0x1C * 4);
        put_tags(&mut e, src, &[1, 2, 3]);
        let dst = e.mem.alloc(0x1C * 4);
        // 00b10c20 copies each element and returns the end; the forwarders return the same.
        for addr in [0x00b1_0940u32, 0x00b1_0b00] {
            let r = e.call(addr, &args![src, src + 0x54, dst, 0u32]);
            assert_eq!(r.u32(), dst + 0x54);
            assert_eq!(
                (0..3)
                    .map(|i| e.mem.u32(dst + 0x1C * i))
                    .collect::<Vec<_>>(),
                [1, 2, 3]
            );
            assert_eq!(e.mem.u32(dst + 0x54 - 4), 3);
            put_tags(&mut e, dst, &[0, 0, 0]);
        }
        let r = e.call(
            0x00b1_0c20,
            &args![src, src + 0x1C, dst + 0x1C, 0u32, 0u32, 0u32],
        );
        assert_eq!(r.u32(), dst + 0x38);
        assert_eq!(e.mem.u32(dst + 0x1C), 1);
        // A null destination is skipped (one element, so the next store is not reached).
        let r = e.call(0x00b1_0c20, &args![src, src + 0x1C, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(r.u32(), 0x1C);
        // Backward: the range moves up by one element, overlapping itself.
        let r = e.call(
            0x00b1_0b80,
            &args![src, src + 0x54, src + 0x70, 0u32, 0u32, 0u32],
        );
        assert_eq!(r.u32(), src + 0x1C);
        assert_eq!(
            (0..4)
                .map(|i| e.mem.u32(src + 0x1C * i))
                .collect::<Vec<_>>(),
            [1, 1, 2, 3]
        );
        // The forwarder passes zero tag bytes and returns the same start.
        put_tags(&mut e, src, &[1, 2, 3, 0]);
        let r = e.call(
            0x00b1_0890,
            &args![src, src + 0x54, src + 0x70, 9u32, 9u32, 9u32],
        );
        assert_eq!(r.u32(), src + 0x1C);
        assert_eq!(
            (0..4)
                .map(|i| e.mem.u32(src + 0x1C * i))
                .collect::<Vec<_>>(),
            [1, 1, 2, 3]
        );
    }

    #[test]
    fn struct_fill_copies_the_value() {
        let mut e = engine();
        let value = e.mem.alloc(0x1C);
        put_tags(&mut e, value, &[7]);
        let dst = e.mem.alloc(0x1C * 3);
        e.call(
            0x00b1_0990,
            &args![dst, 3u32, Ptr::<()>::new(value), 0u32, 0u32, 0u32],
        );
        assert_eq!(
            (0..3)
                .map(|i| e.mem.u32(dst + 0x1C * i))
                .collect::<Vec<_>>(),
            [7, 7, 7]
        );
        assert_eq!(e.mem.u32(dst + 0x54 - 4), 7);
        // A null destination skips the store.
        e.call(
            0x00b1_0990,
            &args![0u32, 1u32, Ptr::<()>::new(value), 0u32, 0u32, 0u32],
        );
    }

    fn insert_bytes(e: &mut Engine, v: u32, position: u32, count: u32, byte: u8) {
        let value = e.mem.alloc(1);
        e.mem.set_u8(value, byte);
        e.call(
            0x00b0_ecf0,
            &args![
                Ptr::<StlVector>::new(v),
                PROXY,
                position,
                count,
                Ptr::<()>::new(value)
            ],
        );
    }

    #[test]
    fn byte_insert_with_room_and_a_long_tail() {
        let mut e = engine();
        allocators(&mut e);
        let (v, data) = vector(&mut e, 1, 5, 10);
        e.mem.write(data, b"ABCDE");
        insert_bytes(&mut e, v, data + 1, 2, b'z');
        assert_eq!(bytes_of(&e, v), b"AzzBCDE");
        assert_eq!(e.mem.u32(v + 0x0C), data);
    }

    #[test]
    fn byte_insert_with_a_short_tail() {
        let mut e = engine();
        allocators(&mut e);
        let (v, data) = vector(&mut e, 1, 3, 10);
        e.mem.write(data, b"ABC");
        insert_bytes(&mut e, v, data + 2, 3, b'z');
        assert_eq!(bytes_of(&e, v), b"ABzzzC");
        // At the end: nothing to move, the gap is filled.
        let end = e.mem.u32(v + 0x10);
        insert_bytes(&mut e, v, end, 2, b'q');
        assert_eq!(bytes_of(&e, v), b"ABzzzCqq");
    }

    #[test]
    fn byte_insert_grows_the_buffer() {
        let mut e = engine();
        let (sizes, freed) = allocators(&mut e);
        let (v, data) = vector(&mut e, 1, 3, 3);
        e.mem.write(data, b"ABC");
        insert_bytes(&mut e, v, data + 1, 2, b'q');
        assert_eq!(bytes_of(&e, v), b"AqqBC");
        // 1.5 times the capacity (4) is below the new size (5): the new size.
        assert_eq!(*sizes.borrow(), vec![vec![5, 0]]);
        assert_eq!(*freed.borrow(), vec![vec![MEMORY_MANAGER, data]]);
        let first = e.mem.u32(v + 0x0C);
        assert_ne!(first, data);
        assert_eq!(e.mem.u32(v + 0x14), first + 5);
        // An empty vector has no block to free.
        let (v2, _) = vector(&mut e, 1, 0, 0);
        for off in [0x0C, 0x10, 0x14] {
            e.mem.set_u32(v2 + off, 0);
        }
        freed.borrow_mut().clear();
        insert_bytes(&mut e, v2, 0, 3, b'q');
        assert_eq!(bytes_of(&e, v2), b"qqq");
        assert!(freed.borrow().is_empty());
    }

    #[test]
    fn byte_insert_of_nothing_and_too_long() {
        let mut e = engine();
        allocators(&mut e);
        let long = spy(&mut e, CHAR_VECTOR_TOO_LONG, 0);
        let (v, data) = vector(&mut e, 1, 3, 10);
        e.mem.write(data, b"ABC");
        insert_bytes(&mut e, v, data, 0, b'x');
        assert!(long.borrow().is_empty());
        insert_bytes(&mut e, v, data, 0xFFFF_FFFF, b'x');
        assert_eq!(long.borrow().len(), 1);
        assert_eq!(bytes_of(&e, v), b"ABC");
    }

    /// The doubles of the int vector's helpers outside this unit.
    fn int_doubles(e: &mut Engine) {
        e.register(INT_COPY, |e, a| {
            let n = (a[1].wrapping_sub(a[0])) / 4;
            let data = e.mem.bytes(a[0], n * 4);
            e.mem.write(a[2], &data);
            ret(a[2] + n * 4)
        });
        e.register(INT_FILL, |e, a| {
            let value = e.mem.u32(a[2]);
            for i in 0..a[1] {
                e.mem.set_u32(a[0] + 4 * i, value);
            }
            Ret::default()
        });
        e.register(INT_COPY_BACKWARD, |e, a| {
            let n = (a[1].wrapping_sub(a[0])) / 4;
            let data = e.mem.bytes(a[0], n * 4);
            e.mem.write(a[2] - n * 4, &data);
            ret(a[2] - n * 4)
        });
        e.register(INT_FILL_TAIL, |e, a| {
            let value = e.mem.u32(a[3]);
            for i in 0..a[2] {
                e.mem.set_u32(a[1] + 4 * i, value);
            }
            Ret::default()
        });
    }

    fn int_vector(e: &mut Engine, values: &[u32], capacity: u32) -> (u32, u32) {
        let (v, data) = vector(e, 4, values.len() as u32, capacity);
        put_ints(e, data, values);
        (v, data)
    }

    fn insert_ints(e: &mut Engine, v: u32, position: u32, count: u32, value: u32) {
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, value);
        e.call(
            0x00b0_f410,
            &args![
                Ptr::<StlVector>::new(v),
                PROXY,
                position,
                count,
                Ptr::<()>::new(slot)
            ],
        );
    }

    #[test]
    fn int_insert_in_every_case() {
        let mut e = engine();
        allocators(&mut e);
        int_doubles(&mut e);
        // Room and a long tail.
        let (v, data) = int_vector(&mut e, &[1, 2, 3, 4, 5], 10);
        insert_ints(&mut e, v, data + 4, 2, 9);
        assert_eq!(ints_of(&e, v), [1, 9, 9, 2, 3, 4, 5]);
        // Room and a short tail.
        let (v, data) = int_vector(&mut e, &[1, 2, 3], 10);
        insert_ints(&mut e, v, data + 8, 3, 9);
        assert_eq!(ints_of(&e, v), [1, 2, 9, 9, 9, 3]);
        // Growth: the capacity 3 becomes the new size 5; the old block is freed.
        let (sizes, freed) = allocators(&mut e);
        let (v, data) = int_vector(&mut e, &[1, 2, 3], 3);
        insert_ints(&mut e, v, data + 4, 2, 9);
        assert_eq!(ints_of(&e, v), [1, 9, 9, 2, 3]);
        assert_eq!(*sizes.borrow(), vec![vec![MEMORY_MANAGER, 20]]);
        assert_eq!(*freed.borrow(), vec![vec![MEMORY_MANAGER, data]]);
        let first = e.mem.u32(v + 0x0C);
        assert_eq!(e.mem.u32(v + 0x14), first + 20);
        // Growth of a larger vector takes 1.5 times the capacity.
        let (v, data) = int_vector(&mut e, &[1, 2, 3, 4], 4);
        sizes.borrow_mut().clear();
        insert_ints(&mut e, v, data + 16, 1, 7);
        assert_eq!(ints_of(&e, v), [1, 2, 3, 4, 7]);
        assert_eq!(*sizes.borrow(), vec![vec![MEMORY_MANAGER, 24]]);
        // Nothing to insert.
        let first = e.mem.u32(v + 0x0C);
        insert_ints(&mut e, v, first, 0, 7);
        assert_eq!(ints_of(&e, v), [1, 2, 3, 4, 7]);
    }

    #[test]
    fn int_insert_too_long_throws() {
        let mut e = engine();
        allocators(&mut e);
        int_doubles(&mut e);
        spy(&mut e, STRING_FROM_C_STRING, 0);
        spy(&mut e, RUNTIME_ERROR_FROM_STRING, 0);
        let (v, data) = int_vector(&mut e, &[1, 2, 3], 10);
        assert!(panics(|| insert_ints(&mut e, v, data, 0x3FFF_FFFE, 7)));
        assert_eq!(ints_of(&e, v), [1, 2, 3]);
    }

    fn insert_structs(e: &mut Engine, v: u32, position: u32, count: u32, tag: u32) {
        let slot = e.mem.alloc(0x1C);
        put_tags(e, slot, &[tag]);
        e.call(
            0x00b0_fc00,
            &args![
                Ptr::<StlVector>::new(v),
                PROXY,
                position,
                count,
                Ptr::<()>::new(slot)
            ],
        );
    }

    fn struct_vector(e: &mut Engine, tags: &[u32], capacity: u32) -> (u32, u32) {
        let (v, data) = vector(e, 0x1C, tags.len() as u32, capacity);
        put_tags(e, data, tags);
        (v, data)
    }

    #[test]
    fn struct_insert_in_every_case() {
        let mut e = engine();
        allocators(&mut e);
        // Room and a long tail.
        let (v, data) = struct_vector(&mut e, &[1, 2, 3, 4, 5], 10);
        insert_structs(&mut e, v, data + 0x1C, 2, 9);
        assert_eq!(tags_of(&e, v), [1, 9, 9, 2, 3, 4, 5]);
        // Room and a short tail.
        let (v, data) = struct_vector(&mut e, &[1, 2, 3], 10);
        insert_structs(&mut e, v, data + 0x38, 3, 9);
        assert_eq!(tags_of(&e, v), [1, 2, 9, 9, 9, 3]);
        // Growth.
        let (sizes, freed) = allocators(&mut e);
        let (v, data) = struct_vector(&mut e, &[1, 2, 3], 3);
        insert_structs(&mut e, v, data + 0x1C, 2, 9);
        assert_eq!(tags_of(&e, v), [1, 9, 9, 2, 3]);
        assert_eq!(*sizes.borrow(), vec![vec![MEMORY_MANAGER, 5 * 0x1C]]);
        assert_eq!(*freed.borrow(), vec![vec![MEMORY_MANAGER, data]]);
        let first = e.mem.u32(v + 0x0C);
        assert_eq!(e.mem.u32(v + 0x14), first + 5 * 0x1C);
        // An empty vector without a block allocates and frees nothing old.
        let (v2, _) = vector(&mut e, 0x1C, 0, 0);
        for off in [0x0C, 0x10, 0x14] {
            e.mem.set_u32(v2 + off, 0);
        }
        freed.borrow_mut().clear();
        insert_structs(&mut e, v2, 0, 2, 4);
        assert_eq!(tags_of(&e, v2), [4, 4]);
        assert!(freed.borrow().is_empty());
        // Nothing to insert.
        insert_structs(&mut e, v, first, 0, 4);
        assert_eq!(tags_of(&e, v), [1, 9, 9, 2, 3]);
    }

    #[test]
    fn struct_insert_too_long_throws() {
        let mut e = engine();
        allocators(&mut e);
        spy(&mut e, STRING_FROM_C_STRING, 0);
        spy(&mut e, RUNTIME_ERROR_FROM_STRING, 0);
        let (v, data) = struct_vector(&mut e, &[1, 2], 10);
        assert!(panics(|| insert_structs(&mut e, v, data, 0x0924_9249, 7)));
        assert_eq!(tags_of(&e, v), [1, 2]);
    }
}
