//! `FaceGen/run_00aba3c0` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The C++ library's `std::vector` code that FaceGen's containers instantiate (debug
//! iterators: a failed check calls `_invalid_parameter_noinfo` and the code goes on), and
//! `FutBinaryFileC`'s constructor.
//!
//! The vector (0x18 bytes on PC) holds its container proxy at +0x00, its allocator at +0x08
//! and its first, last and end pointers at +0x0C, +0x10 and +0x14. A checked iterator is two
//! words: the container proxy and the element pointer. Element counts divide the byte span by
//! the element size as the code does: a signed shift for sizes that are powers of two, a
//! signed division otherwise.
//!
//! The library's tag objects (empty iterator-category and allocator-traits structs passed by
//! value) are passed as two byte-sized words: the code copies the second from an uninitialized
//! stack byte, so it has no defined value; the translations pass 0 for both (no callee reads
//! them).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::facegen::binaryfile::{
    string_chars, with_string_arg, StringArg, INVALID_PARAMETER, MEMORY_MANAGER_FREE,
    MEMORY_MANAGER_OBJECT, STRING_ASSIGN, STRING_LENGTH, STRING_NPOS, STRING_TIDY, VECTOR_FIRST,
    VECTOR_LAST, VECTOR_PROXY,
};

/// The vector's end pointer (+0x14) and allocator (+0x08).
pub(crate) const VECTOR_END: u32 = 0x14;
pub(crate) const VECTOR_ALLOCATOR: u32 = 0x08;

/// The container proxy's allocation (`0044d670`, `cdecl`: count and hint).
const ALLOCATE_PROXY: u32 = 0x0044_d670;
/// The memory manager's allocation (`00aa3e40`, `thiscall` on `011f6238`, the size).
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// The byte allocation (`0065fa30`, `cdecl`: count and hint).
const ALLOCATE_BYTES: u32 = 0x0065_fa30;
/// The vectors' "too long" throws: `00ac0b00` and `0065f640` (no arguments; they do not
/// return in the game).
const THROW_LENGTH_00AC0B00: u32 = 0x00ac_0b00;
pub(crate) const THROW_LENGTH_0065F640: u32 = 0x0065_f640;
/// `memmove_s` (`00ec7ce3`, `cdecl`: destination, size, source, count).
const MEMMOVE_S: u32 = 0x00ec_7ce3;
/// `std::bad_alloc`'s construction for the size overflow of `00ac1720`: the exception base
/// constructor (`00ec78f1`, `thiscall`: the address of the message pointer), the vtable it then
/// gets (`010176f8`), the throw information (`01140ef8`) and `_CxxThrowException`
/// (`00ec7894`).
const EXCEPTION_CONSTRUCT: u32 = 0x00ec_78f1;
const BAD_ALLOC_VTABLE: u32 = 0x0101_76f8;
const BAD_ALLOC_THROW_INFO: u32 = 0x0114_0ef8;
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
/// `fr2Assert` (`00aafb90`, `cdecl`: source file name, line) and the source file name it gets
/// from `FutBinaryFileC`'s constructor.
const FR2_ASSERT: u32 = 0x00aa_fb90;
const BINARY_FILE_SOURCE: u32 = 0x010a_2d98;
/// `FutBinaryFileC`'s vtable (RTTI).
const FUT_BINARY_FILE_VTABLE: u32 = 0x010a_2c20;

/// A failed debug check when `ok` is false.
pub(crate) fn check(e: &mut Engine, ok: bool) {
    if !ok {
        e.call(INVALID_PARAMETER, &[]);
    }
}

/// The checked iterator the code makes for `pointer` in the vector at `vector`: it must lie
/// within the vector (`first <= pointer <= last`, the vector not null). Returns the proxy word.
pub(crate) fn checked_iterator(e: &mut Engine, vector: u32, pointer: u32) -> u32 {
    let ok = vector != 0
        && e.mem.u32(vector + VECTOR_FIRST) <= pointer
        && pointer <= e.mem.u32(vector + VECTOR_LAST);
    check(e, ok);
    e.mem.u32(vector + VECTOR_PROXY)
}

/// How a vector's element counts come from byte spans: a signed shift or a signed division.
#[derive(Clone, Copy)]
pub(crate) enum ElementSize {
    Shift(u32),
    Divide(u32),
}

impl ElementSize {
    pub(crate) fn bytes(self) -> u32 {
        match self {
            ElementSize::Shift(shift) => 1 << shift,
            ElementSize::Divide(size) => size,
        }
    }

    pub(crate) fn count(self, span: u32) -> u32 {
        match self {
            ElementSize::Shift(shift) => ((span as i32) >> shift) as u32,
            ElementSize::Divide(size) => ((span as i32) / size as i32) as u32,
        }
    }
}

/// The vector's size (last - first, in elements).
pub(crate) fn vector_size(e: &Engine, vector: u32, size: ElementSize) -> u32 {
    let span = e
        .mem
        .u32(vector + VECTOR_LAST)
        .wrapping_sub(e.mem.u32(vector + VECTOR_FIRST));
    size.count(span)
}

/// The callee shapes the vector templates take: insert-n (vector, where proxy, where pointer,
/// count, value address), iterator advance (iterator, count), erase (vector, result, first
/// proxy, first pointer, last proxy, last pointer), allocation (count), 4-word copy (first,
/// last, destination, allocator), fill (vector or destination, destination or count, count or
/// value, value or allocator) and copy construction (destination, source).
pub(crate) type InsertFn = fn(&mut Engine, Ptr, u32, u32, u32, u32);
pub(crate) type AdvanceFn = fn(&mut Engine, Ptr, u32);
pub(crate) type EraseFn = fn(&mut Engine, Ptr, Ptr, u32, u32, u32, u32);
type AllocateFn = fn(&mut Engine, u32) -> u32;
type CopyFn = fn(&mut Engine, u32, u32, u32, u32) -> u32;
type FillFn = fn(&mut Engine, u32, u32, u32, u32);
type ConstructFn = fn(&mut Engine, Ptr, Ptr) -> Ptr;

/// The parts of a `resize(count, value)` instance that differ: the element size, the
/// insert-n (`thiscall`: where proxy, where pointer, count, address of the value), the
/// iterator advance (`thiscall` on the iterator: the count) and the erase (`thiscall`: the
/// result, first proxy, first pointer, last proxy, last pointer).
pub(crate) struct ResizeShape {
    pub(crate) size: ElementSize,
    pub(crate) insert: InsertFn,
    pub(crate) advance: AdvanceFn,
    pub(crate) erase: EraseFn,
}

/// The `resize(count, value)` template: grows with insert-n at the end (the value given by
/// the address of its argument slot), or shrinks by erasing from `begin + count` to the end.
pub(crate) fn resize_with_value(
    e: &mut Engine,
    this: Ptr,
    count: u32,
    value: u32,
    shape: &ResizeShape,
) {
    let vector = this.addr();
    let size = vector_size(e, vector, shape.size);
    if size < count {
        let last = e.mem.u32(vector + VECTOR_LAST);
        let proxy = checked_iterator(e, vector, last);
        let missing = count.wrapping_sub(vector_size(e, vector, shape.size));
        (shape.insert)(e, this, proxy, last, missing, value);
        return;
    }
    if count >= vector_size(e, vector, shape.size) {
        return;
    }
    let last = e.mem.u32(vector + VECTOR_LAST);
    let end_proxy = checked_iterator(e, vector, last);
    let first = e.mem.u32(vector + VECTOR_FIRST);
    let begin_proxy = checked_iterator(e, vector, first);
    e.with_stack(0x10, |e, locals| {
        let iterator = locals.addr();
        let result = locals.addr() + 8;
        e.mem.set_u32(iterator, begin_proxy);
        e.mem.set_u32(iterator + 4, first);
        (shape.advance)(e, Ptr::new(iterator), count);
        let new_proxy = e.mem.u32(iterator);
        let new_pointer = e.mem.u32(iterator + 4);
        (shape.erase)(
            e,
            this,
            Ptr::new(result),
            new_proxy,
            new_pointer,
            end_proxy,
            last,
        );
    });
}

/// A by-value argument of `words` words placed in memory (its argument slot) for `f`.
pub(crate) fn with_value_slot<R>(
    e: &mut Engine,
    words: &[u32],
    f: impl FnOnce(&mut Engine, u32) -> R,
) -> R {
    e.with_stack(4 * words.len() as u32, |e, slot| {
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(slot.addr() + 4 * i as u32, *word);
        }
        f(e, slot.addr())
    })
}

fn insert_by_address(
    addr: u32,
    e: &mut Engine,
    this: Ptr,
    proxy: u32,
    at: u32,
    count: u32,
    value: u32,
) {
    e.call(addr, &args![this, proxy, at, count, value]);
}

pub(crate) fn advance_by_address(addr: u32, e: &mut Engine, iterator: Ptr, count: u32) {
    e.call(addr, &args![iterator, count]);
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn erase_by_address(
    addr: u32,
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    first_proxy: u32,
    first: u32,
    last_proxy: u32,
    last: u32,
) {
    e.call(
        addr,
        &args![this, result, first_proxy, first, last_proxy, last],
    );
}

// Translated from 00aba4f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aba4f0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a 4-byte-element vector's default constructor: its container
/// proxy (one, `0044d670(1, 0)`, pointing back at the vector; null when the allocation fails)
/// at +0, then the buy of 0 elements (`00abc260`), which only clears the pointers. Returns
/// `this`.
pub fn fn_00aba4f0(e: &mut Engine, this: Ptr) -> Ptr {
    let proxy = e.call(ALLOCATE_PROXY, &args![1u32, 0u32]).u32();
    let proxy = if proxy != 0 {
        e.mem.set_u32(proxy, this.addr());
        proxy
    } else {
        0
    };
    e.mem.set_u32(this.addr(), proxy);
    fn_00abc260(e, this, 0);
    this
}

// Translated from 00aba5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 4-byte-element vector's `resize(count, value)` (`thiscall`,
/// two words): grows by inserting copies of `value` at the end (`00abc320`), or shrinks by
/// erasing from `begin + count` (`00ac1000`) to the end (`00abc180`).
pub fn fn_00aba5d0(e: &mut Engine, this: Ptr, count: u32, value: u32) {
    let shape = ResizeShape {
        size: ElementSize::Shift(2),
        insert: |e, this, proxy, at, count, value| fn_00abc320(e, this, proxy, at, count, value),
        advance: |e, iterator, count| {
            fn_00ac1000(e, iterator, count);
        },
        erase: |e, this, result, a, b, c, d| {
            fn_00abc180(e, this, result, a, b, c, d);
        },
    };
    with_value_slot(e, &[value], |e, slot| {
        resize_with_value(e, this, count, slot, &shape)
    });
}

// Translated from 00aba7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 12-byte-element vector's `resize(count, value)` (`thiscall`,
/// four words: the count and the value's three): grows with `00abca90`, or shrinks with the
/// advance `00ac0dc0` and the erase `00aba9a0`.
pub fn fn_00aba7b0(
    e: &mut Engine,
    this: Ptr,
    count: u32,
    value_0: u32,
    value_1: u32,
    value_2: u32,
) {
    let shape = ResizeShape {
        size: ElementSize::Divide(0xc),
        insert: |e, this, proxy, at, count, value| fn_00abca90(e, this, proxy, at, count, value),
        advance: |e, iterator, count| advance_by_address(0x00ac_0dc0, e, iterator, count),
        erase: |e, this, result, a, b, c, d| {
            erase_by_address(0x00ab_a9a0, e, this, result, a, b, c, d)
        },
    };
    with_value_slot(e, &[value_0, value_1, value_2], |e, slot| {
        resize_with_value(e, this, count, slot, &shape)
    });
}

// Translated from 00abaaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 16-byte-element vector's `resize(count, value)` (`thiscall`,
/// five words): grows with `00abd2e0`, or shrinks with the advance `00ac0e80` and the erase
/// `00abac80`.
pub fn fn_00abaaa0(e: &mut Engine, this: Ptr, count: u32, value: [u32; 4]) {
    let shape = ResizeShape {
        size: ElementSize::Shift(4),
        insert: |e, this, proxy, at, count, value| {
            insert_by_address(0x00ab_d2e0, e, this, proxy, at, count, value)
        },
        advance: |e, iterator, count| advance_by_address(0x00ac_0e80, e, iterator, count),
        erase: |e, this, result, a, b, c, d| {
            erase_by_address(0x00ab_ac80, e, this, result, a, b, c, d)
        },
    };
    with_value_slot(e, &value, |e, slot| {
        resize_with_value(e, this, count, slot, &shape)
    });
}

// Translated from 00abad80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an 8-byte-element vector's `resize(count, value)` (`thiscall`,
/// three words): grows with `00abdb10`, or shrinks with the advance `00ac0f40` and the erase
/// `00abaf60`.
pub fn fn_00abad80(e: &mut Engine, this: Ptr, count: u32, value_0: u32, value_1: u32) {
    let shape = ResizeShape {
        size: ElementSize::Shift(3),
        insert: |e, this, proxy, at, count, value| fn_00abdb10(e, this, proxy, at, count, value),
        advance: |e, iterator, count| advance_by_address(0x00ac_0f40, e, iterator, count),
        erase: |e, this, result, a, b, c, d| {
            erase_by_address(0x00ab_af60, e, this, result, a, b, c, d)
        },
    };
    with_value_slot(e, &[value_0, value_1], |e, slot| {
        resize_with_value(e, this, count, slot, &shape)
    });
}

// Translated from 00abb060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00abb060 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a 0x20-byte-element vector's `resize(count, value)` (`thiscall`,
/// nine words: the count and the value, a word and a string): grows with `00abe310`, or
/// shrinks with the advance `00ac0b80` and the erase `00abb280`; then the value's string (+4)
/// is destroyed (`0044a730(1, 0)`).
pub fn fn_00abb060(e: &mut Engine, this: Ptr, count: u32, value: [u32; 8]) {
    let shape = ResizeShape {
        size: ElementSize::Shift(5),
        insert: |e, this, proxy, at, count, value| {
            insert_by_address(0x00ab_e310, e, this, proxy, at, count, value)
        },
        advance: |e, iterator, count| advance_by_address(0x00ac_0b80, e, iterator, count),
        erase: |e, this, result, a, b, c, d| {
            erase_by_address(0x00ab_b280, e, this, result, a, b, c, d)
        },
    };
    with_value_slot(e, &value, |e, slot| {
        resize_with_value(e, this, count, slot, &shape);
        e.call(STRING_TIDY, &args![slot + 4, 1u32, 0u32]);
    });
}

// Translated from 00abb3c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00abb3c0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a 0x2C-byte-element vector's `resize(count, value)` (`thiscall`,
/// twelve words): grows with `00abeb00`, or shrinks with the advance `00ac0c40` and the erase
/// `00abb5f0`; then the value's string at +0x10 is destroyed (`0044a730(1, 0)`).
pub fn fn_00abb3c0(e: &mut Engine, this: Ptr, count: u32, value: [u32; 11]) {
    let shape = ResizeShape {
        size: ElementSize::Divide(0x2c),
        insert: |e, this, proxy, at, count, value| {
            insert_by_address(0x00ab_eb00, e, this, proxy, at, count, value)
        },
        advance: |e, iterator, count| advance_by_address(0x00ac_0c40, e, iterator, count),
        erase: |e, this, result, a, b, c, d| {
            erase_by_address(0x00ab_b5f0, e, this, result, a, b, c, d)
        },
    };
    with_value_slot(e, &value, |e, slot| {
        resize_with_value(e, this, count, slot, &shape);
        e.call(STRING_TIDY, &args![slot + 0x10, 1u32, 0u32]);
    });
}

// Translated from 00abb700 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00abb700 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a 0x34-byte-element vector's `resize(count, value)` (`thiscall`,
/// fourteen words): grows with `00abf210`, or shrinks with the advance `00ac0d00` and the
/// erase `00abb930`; then the value is destroyed (`0065ed70`, `thiscall`).
///
/// `value` is the address of the argument area that holds the by-value element (the exe's
/// `[EBP+0xC]`): both callers build the element there (`00ab5165`..`00ab5170`,
/// `00ab61a4`..`00ab61af`), so the proxy of its 12-byte vector (`0065dc90`) names that area;
/// it is read and destroyed in place.
pub fn fn_00abb700(e: &mut Engine, this: Ptr, count: u32, value: Ptr) {
    let shape = ResizeShape {
        size: ElementSize::Divide(0x34),
        insert: |e, this, proxy, at, count, value| fn_00abf210(e, this, proxy, at, count, value),
        advance: |e, iterator, count| advance_by_address(0x00ac_0d00, e, iterator, count),
        erase: |e, this, result, a, b, c, d| {
            erase_by_address(0x00ab_b930, e, this, result, a, b, c, d)
        },
    };
    resize_with_value(e, this, count, value.addr(), &shape); // 00abb7b2: `LEA ECX,[EBP+0xC]`
    e.call(VALUE_34_DESTRUCT, &args![value]); // 00abb913: `LEA ECX,[EBP+0xC]`, in place
}

// Translated from 00abbae0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00abbae0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a 0x38-byte-element vector's `resize(count, value)` (`thiscall`,
/// fifteen words): grows with `00abfa90`, or shrinks with the advance `00ad3750` and the
/// erase `00abbd10`; then the value is destroyed (`0065edd0`, `thiscall`).
pub fn fn_00abbae0(e: &mut Engine, this: Ptr, count: u32, value: [u32; 14]) {
    with_value_slot(e, &value, |e, slot| resize_38_in_area(e, this, count, slot));
}

/// `00abbae0`'s body over the value in its argument area at `value` (the exe's `[EBP+0xC]`):
/// the callers that build the element in their outgoing argument area (`00ab46f0` at
/// `00ab5467`, `00ab5ec0` at `00ab61d8`) pass that area itself, so the proxy of the element's
/// 4-byte vector (+0x20, `00aba4f0`) names the vector the resize copies from; the value is
/// destroyed there.
pub(crate) fn resize_38_in_area(e: &mut Engine, this: Ptr, count: u32, value: u32) {
    let shape = ResizeShape {
        size: ElementSize::Divide(0x38),
        insert: |e, this, proxy, at, count, value| {
            insert_by_address(0x00ab_fa90, e, this, proxy, at, count, value) // 00abbbc6
        },
        advance: |e, iterator, count| advance_by_address(0x00ad_3750, e, iterator, count),
        erase: |e, this, result, a, b, c, d| {
            erase_by_address(0x00ab_bd10, e, this, result, a, b, c, d)
        },
    };
    resize_with_value(e, this, count, value, &shape); // 00abbb92: `LEA ECX,[EBP+0xC]`
    e.call(VALUE_38_DESTRUCT, &args![value]); // 00abbcf3: the value destroyed where it lies
}

/// The destructors of the 0x34- and 0x38-byte elements (`0065ed70`, `0065edd0`; `thiscall`).
const VALUE_34_DESTRUCT: u32 = 0x0065_ed70;
const VALUE_38_DESTRUCT: u32 = 0x0065_edd0;

// Translated from 00abbfc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00abbfc0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: the byte vector's construct-n (`thiscall`: count, address of the
/// byte): buys `count` bytes (`00ac03a0`) and, when that succeeds, fills them with the byte
/// (`00ac16e0`) and sets the last pointer to `first + count`.
pub fn fn_00abbfc0(e: &mut Engine, this: Ptr, count: u32, value: u32) {
    if !fn_00ac03a0(e, this, count) {
        return;
    }
    let first = e.mem.u32(this.addr() + VECTOR_FIRST);
    fn_00ac16e0(e, first, count, value, this.addr() + VECTOR_ALLOCATOR);
    e.mem
        .set_u32(this.addr() + VECTOR_LAST, first.wrapping_add(count));
}

// Translated from 00abc0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a vector's tidy: frees the elements (the memory manager's free)
/// when there are any, and clears the first, last and end pointers.
pub fn fn_00abc0a0(e: &mut Engine, this: Ptr) {
    let first = e.mem.u32(this.addr() + VECTOR_FIRST);
    if first != 0 {
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, first]);
    }
    e.mem.set_u32(this.addr() + VECTOR_FIRST, 0);
    e.mem.set_u32(this.addr() + VECTOR_LAST, 0);
    e.mem.set_u32(this.addr() + VECTOR_END, 0);
}

// Translated from 00abc120 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00abc120 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: frees a vector's container proxy (the word at +0) through the
/// memory manager.
pub fn fn_00abc120(e: &mut Engine, this: Ptr) {
    let proxy = e.mem.u32(this.addr() + VECTOR_PROXY);
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, proxy]);
}

// Translated from 00abc180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 4-byte-element vector's `erase(first, last)` (`thiscall`, five
/// words: the result and the two iterators). Both iterators are checked and remade on this
/// vector's proxy; when they differ (`00ac0460`) the elements from `last` to the end move down
/// to `first` (`00ac13c0`) and the last pointer becomes the end of the moved block. The result
/// is the first iterator; returns `result`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00abc180(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    _first_proxy: u32,
    first: u32,
    _last_proxy: u32,
    last: u32,
) -> Ptr {
    let vector = this.addr();
    let first_proxy = checked_iterator(e, vector, first);
    let last_proxy = checked_iterator(e, vector, last);
    e.with_stack(0x10, |e, iterators| {
        let begin = iterators.addr();
        let end = iterators.addr() + 8;
        e.mem.set_u32(begin, first_proxy);
        e.mem.set_u32(begin + 4, first);
        e.mem.set_u32(end, last_proxy);
        e.mem.set_u32(end + 4, last);
        if fn_00ac0460(e, Ptr::new(begin), Ptr::new(end)) != 0 {
            let old_last = e.mem.u32(vector + VECTOR_LAST);
            let moved = fn_00ac13c0(e, last, old_last, first);
            e.mem.set_u32(vector + VECTOR_LAST, moved);
        }
    });
    e.mem.set_u32(result.addr(), first_proxy);
    e.mem.set_u32(result.addr() + 4, first);
    result
}

// Translated from 00abc260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 4-byte-element vector's buy (`thiscall`, the count): clears the
/// pointers; 0 answers false; a count above the maximum (0x3FFFFFFF) calls the length throw
/// (`00ac0b00`) and answers true; otherwise `count` elements are allocated (`00ac1720`), first
/// and last point at them and end after them. Answers true.
pub fn fn_00abc260(e: &mut Engine, this: Ptr, count: u32) -> bool {
    let vector = this.addr();
    e.mem.set_u32(vector + VECTOR_FIRST, 0);
    e.mem.set_u32(vector + VECTOR_LAST, 0);
    e.mem.set_u32(vector + VECTOR_END, 0);
    if count == 0 {
        return false;
    }
    let maximum = max_or_one(0x3fff_ffff);
    if maximum < count {
        e.call(THROW_LENGTH_00AC0B00, &[]);
        return true;
    }
    let block = fn_00ac1720(e, count, 0);
    e.mem.set_u32(vector + VECTOR_FIRST, block);
    let first = e.mem.u32(vector + VECTOR_FIRST);
    e.mem.set_u32(vector + VECTOR_LAST, first);
    let first = e.mem.u32(vector + VECTOR_FIRST);
    e.mem.set_u32(
        vector + VECTOR_END,
        first.wrapping_add(count.wrapping_mul(4)),
    );
    true
}

/// The library's `max_size()` as the code tests it: the maximum, or 1 when it is 0 (never for
/// these constants; the code keeps the test).
pub(crate) fn max_or_one(maximum: u32) -> u32 {
    if maximum > 0 {
        maximum
    } else {
        1
    }
}

/// The capacity the insert-n grows to: half as much again, 0 when that passes the maximum,
/// and at least `needed`.
pub(crate) fn grown_capacity(capacity: u32, maximum: u32, needed: u32) -> u32 {
    let maximum = max_or_one(maximum);
    let grown = if maximum.wrapping_sub(capacity >> 1) < capacity {
        0
    } else {
        (capacity >> 1).wrapping_add(capacity)
    };
    if grown < needed {
        needed
    } else {
        grown
    }
}

/// The capacity of the vector (end - first, in elements), 0 without elements.
pub(crate) fn vector_capacity(e: &Engine, vector: u32, size: ElementSize) -> u32 {
    let first = e.mem.u32(vector + VECTOR_FIRST);
    if first == 0 {
        return 0;
    }
    size.count(e.mem.u32(vector + VECTOR_END).wrapping_sub(first))
}

/// The parts of the insert-n instances over plain elements (4, 8 and 12 bytes) that differ.
struct PlainInsertShape {
    pub(crate) size: ElementSize,
    maximum: u32,
    throw_length: u32,
    /// The allocation (count) and the 4-word copy (first, last, destination, allocator).
    allocate: AllocateFn,
    copy: CopyFn,
    /// The fill of new elements (vector, destination, count, value address) after a
    /// reallocation and after the moved tail.
    fill: FillFn,
    fill_tail: FillFn,
    /// The copy of the last `count` elements past the end (first, last, destination,
    /// allocator) and the backward move of the rest (`cdecl`: first, last, end).
    copy_tail: CopyFn,
    move_backward: u32,
    /// The value's words as the code copies them (`MOV`, or `FLD`/`FSTP` for floats).
    load: fn(&Engine, u32) -> Vec<u32>,
    /// Whether the reallocation walks the old elements (an empty destructor loop).
    walks_old: bool,
}

/// The insert-n template (`_Insert_n(where, count, value)`) over plain elements.
fn insert_plain(
    e: &mut Engine,
    this: Ptr,
    at: u32,
    count: u32,
    value: u32,
    shape: &PlainInsertShape,
) {
    let vector = this.addr();
    let bytes = shape.size.bytes();
    let allocator = vector + VECTOR_ALLOCATOR;
    let capacity = vector_capacity(e, vector, shape.size);
    if count == 0 {
        return;
    }
    let maximum = max_or_one(shape.maximum);
    if maximum.wrapping_sub(vector_size(e, vector, shape.size)) < count {
        e.call(shape.throw_length, &[]);
        return;
    }
    if capacity < vector_size(e, vector, shape.size).wrapping_add(count) {
        let needed = vector_size(e, vector, shape.size).wrapping_add(count);
        let new_capacity = grown_capacity(capacity, shape.maximum, needed);
        let block = (shape.allocate)(e, new_capacity);
        let first = e.mem.u32(vector + VECTOR_FIRST);
        let mut cursor = (shape.copy)(e, first, at, block, allocator);
        (shape.fill)(e, vector, cursor, count, value);
        cursor = cursor.wrapping_add(count.wrapping_mul(bytes));
        let last = e.mem.u32(vector + VECTOR_LAST);
        (shape.copy)(e, at, last, cursor, allocator);
        let new_size = vector_size(e, vector, shape.size).wrapping_add(count);
        let first = e.mem.u32(vector + VECTOR_FIRST);
        if first != 0 {
            if shape.walks_old {
                let last = e.mem.u32(vector + VECTOR_LAST);
                let mut element = first;
                while element != last {
                    element = element.wrapping_add(bytes);
                }
            }
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, first]);
        }
        e.mem.set_u32(
            vector + VECTOR_END,
            block.wrapping_add(new_capacity.wrapping_mul(bytes)),
        );
        e.mem.set_u32(
            vector + VECTOR_LAST,
            block.wrapping_add(new_size.wrapping_mul(bytes)),
        );
        e.mem.set_u32(vector + VECTOR_FIRST, block);
        return;
    }
    let after = shape
        .size
        .count(e.mem.u32(vector + VECTOR_LAST).wrapping_sub(at));
    let words = (shape.load)(e, value);
    let store = |e: &mut Engine, from: u32, to: u32| {
        let mut element = from;
        while element != to {
            for (i, word) in words.iter().enumerate() {
                e.mem.set_u32(element + 4 * i as u32, *word);
            }
            element = element.wrapping_add(bytes);
        }
    };
    if after < count {
        e.with_stack(bytes, |e, temporary| {
            for (i, word) in words.iter().enumerate() {
                e.mem.set_u32(temporary.addr() + 4 * i as u32, *word);
            }
            let last = e.mem.u32(vector + VECTOR_LAST);
            (shape.copy)(
                e,
                at,
                last,
                at.wrapping_add(count.wrapping_mul(bytes)),
                allocator,
            );
            let last = e.mem.u32(vector + VECTOR_LAST);
            let missing = count.wrapping_sub(shape.size.count(last.wrapping_sub(at)));
            (shape.fill_tail)(e, vector, last, missing, temporary.addr());
        });
        let last = e
            .mem
            .u32(vector + VECTOR_LAST)
            .wrapping_add(count.wrapping_mul(bytes));
        e.mem.set_u32(vector + VECTOR_LAST, last);
        store(e, at, last.wrapping_sub(count.wrapping_mul(bytes)));
    } else {
        let old_last = e.mem.u32(vector + VECTOR_LAST);
        let tail = old_last.wrapping_sub(count.wrapping_mul(bytes));
        let last = e.mem.u32(vector + VECTOR_LAST);
        let new_last = (shape.copy_tail)(e, tail, old_last, last, allocator);
        e.mem.set_u32(vector + VECTOR_LAST, new_last);
        e.call(shape.move_backward, &args![at, tail, old_last]);
        store(e, at, at.wrapping_add(count.wrapping_mul(bytes)));
    }
}

/// A value's words read with `MOV`.
fn load_words<const N: usize>(e: &Engine, value: u32) -> Vec<u32> {
    (0..N as u32).map(|i| e.mem.u32(value + 4 * i)).collect()
}

/// A value of two floats read with `FLD`/`FSTP` (a signalling NaN comes out quiet).
fn load_floats(e: &Engine, value: u32) -> Vec<u32> {
    (0..2u32)
        .map(|i| x87_quiet_bits(e.mem.u32(value + 4 * i)))
        .collect()
}

// Translated from 00abc320 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00abc320 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a 4-byte-element vector's `_Insert_n(where, count, value)`
/// (`thiscall`, four words: the iterator's proxy and pointer, the count, the value's address).
/// Nothing for 0; more than the maximum (0x3FFFFFFF) calls the length throw (`00ac0b00`). Past
/// the capacity, a block of half as much again (at least enough; `00ac1720`) gets the elements
/// before `where` (`00ac35d0`), `count` copies (`00ac3450`) and the rest, and the old block is
/// freed. Otherwise, when fewer than `count` elements follow `where`, they move up
/// (`00ac35d0`), the gap past the old end is filled (`00ac0ab0`) and the rest set; else the last
/// `count` elements are copied past the end (`00ac35d0`), the others move up (`00ac1440`) and
/// `count` elements from `where` are set. The value is read before anything moves.
pub fn fn_00abc320(e: &mut Engine, this: Ptr, _unused_1: u32, at: u32, count: u32, value: u32) {
    let shape = PlainInsertShape {
        size: ElementSize::Shift(2),
        maximum: 0x3fff_ffff,
        throw_length: THROW_LENGTH_00AC0B00,
        allocate: |e, count| fn_00ac1720(e, count, 0),
        copy: |e, first, last, destination, allocator| {
            fn_00ac35d0(e, first, last, destination, allocator)
        },
        fill: |e, _, destination, count, value| fn_00ac3450(e, destination, count, value),
        fill_tail: |e, vector, destination, count, value| {
            e.call(0x00ac_0ab0, &args![vector, destination, count, value]);
        },
        copy_tail: |e, first, last, destination, allocator| {
            fn_00ac35d0(e, first, last, destination, allocator)
        },
        move_backward: 0x00ac_1440,
        load: load_words::<1>,
        walks_old: false,
    };
    insert_plain(e, this, at, count, value, &shape);
}

// Translated from 00abca90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00abca90 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a 12-byte-element vector's `_Insert_n(where, count, value)`, as
/// `00abc320` with the maximum 0x15555555 (`0065f640` throws), the allocation `0065e310`, the
/// copies `00ac3580`, the fills `00ac20d0`, the tail copy `00ac1af0` and the move `00ac1370`;
/// the reallocation walks the old elements first.
pub fn fn_00abca90(e: &mut Engine, this: Ptr, _unused_1: u32, at: u32, count: u32, value: u32) {
    let shape = PlainInsertShape {
        size: ElementSize::Divide(0xc),
        maximum: 0x1555_5555,
        throw_length: THROW_LENGTH_0065F640,
        allocate: |e, count| e.call(0x0065_e310, &args![count, 0u32]).u32(),
        copy: |e, first, last, destination, allocator| {
            fn_00ac3580(e, first, last, destination, allocator)
        },
        fill: |e, vector, destination, count, value| {
            fn_00ac20d0(
                e,
                destination,
                count,
                value,
                vector + VECTOR_ALLOCATOR,
                0,
                0,
            )
        },
        fill_tail: |e, vector, destination, count, value| {
            fn_00ac20d0(
                e,
                destination,
                count,
                value,
                vector + VECTOR_ALLOCATOR,
                0,
                0,
            )
        },
        copy_tail: |e, first, last, destination, allocator| {
            e.call(0x00ac_1af0, &args![first, last, destination, allocator])
                .u32()
        },
        move_backward: 0x00ac_1370,
        load: load_words::<3>,
        walks_old: true,
    };
    insert_plain(e, this, at, count, value, &shape);
}

// Translated from 00abdb10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00abdb10 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: an 8-byte-element (two floats) vector's `_Insert_n(where, count,
/// value)`, as `00abc320` with the maximum 0x1FFFFFFF (`0065f640` throws), the allocation
/// `0065e3a0`, the copies `00ac3690`, the fills `00ac2290`, the tail copy `00ac1c90` and the move
/// `00ac1530`; the value is read as floats (`FLD`/`FSTP`), and the reallocation walks the old
/// elements first.
pub fn fn_00abdb10(e: &mut Engine, this: Ptr, _unused_1: u32, at: u32, count: u32, value: u32) {
    let shape = PlainInsertShape {
        size: ElementSize::Shift(3),
        maximum: 0x1fff_ffff,
        throw_length: THROW_LENGTH_0065F640,
        allocate: |e, count| e.call(0x0065_e3a0, &args![count, 0u32]).u32(),
        copy: |e, first, last, destination, allocator| {
            fn_00ac3690(e, first, last, destination, allocator)
        },
        fill: |e, vector, destination, count, value| {
            fn_00ac2290(
                e,
                destination,
                count,
                value,
                vector + VECTOR_ALLOCATOR,
                0,
                0,
            )
        },
        fill_tail: |e, vector, destination, count, value| {
            fn_00ac2290(
                e,
                destination,
                count,
                value,
                vector + VECTOR_ALLOCATOR,
                0,
                0,
            )
        },
        copy_tail: |e, first, last, destination, allocator| {
            e.call(0x00ac_1c90, &args![first, last, destination, allocator])
                .u32()
        },
        move_backward: 0x00ac_1530,
        load: load_floats,
        walks_old: true,
    };
    insert_plain(e, this, at, count, value, &shape);
}

/// The parts of the insert-n instances over the 0x34- and 0x38-byte elements (a string and a
/// vector or two) that differ.
struct ObjectInsertShape {
    size: u32,
    maximum: u32,
    allocate: u32,
    /// The 4-word copies (first, last, destination, allocator) into new and after the end.
    copy: CopyFn,
    copy_tail: CopyFn,
    /// The fill of new elements (destination, count, value, allocator).
    fill: FillFn,
    /// The element's copy constructor (`thiscall`: the source) and destructor.
    construct: ConstructFn,
    destruct: u32,
    /// The assignment over a range (`cdecl`: first, last, value) and the backward move
    /// (`cdecl`, six words: first, last, end and three tags).
    assign: u32,
    move_backward: u32,
}

/// The insert-n template over elements with constructors: as [`insert_plain`], but the value
/// is copied into a temporary element (and destroyed after), the reallocation destroys every
/// old element (its free of each element is guarded by a mask the compiler folded to 0, so it
/// never runs and is left out), and the elements are set with the range assignment.
fn insert_objects(
    e: &mut Engine,
    this: Ptr,
    at: u32,
    count: u32,
    value: u32,
    shape: &ObjectInsertShape,
) {
    let vector = this.addr();
    let size = ElementSize::Divide(shape.size);
    let bytes = shape.size;
    let allocator = vector + VECTOR_ALLOCATOR;
    let capacity = vector_capacity(e, vector, size);
    if count == 0 {
        return;
    }
    let maximum = max_or_one(shape.maximum);
    if maximum.wrapping_sub(vector_size(e, vector, size)) < count {
        e.call(THROW_LENGTH_0065F640, &[]);
        return;
    }
    if capacity < vector_size(e, vector, size).wrapping_add(count) {
        let needed = vector_size(e, vector, size).wrapping_add(count);
        let new_capacity = grown_capacity(capacity, shape.maximum, needed);
        let block = e.call(shape.allocate, &args![new_capacity, 0u32]).u32();
        let first = e.mem.u32(vector + VECTOR_FIRST);
        let mut cursor = (shape.copy)(e, first, at, block, allocator);
        (shape.fill)(e, cursor, count, value, allocator);
        cursor = count.wrapping_mul(bytes).wrapping_add(cursor);
        let last = e.mem.u32(vector + VECTOR_LAST);
        (shape.copy_tail)(e, at, last, cursor, allocator);
        let new_size = vector_size(e, vector, size).wrapping_add(count);
        let first = e.mem.u32(vector + VECTOR_FIRST);
        if first != 0 {
            let last = e.mem.u32(vector + VECTOR_LAST);
            let mut element = first;
            while element != last {
                e.call(shape.destruct, &args![element]);
                element = element.wrapping_add(bytes);
            }
            let first = e.mem.u32(vector + VECTOR_FIRST);
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, first]);
        }
        e.mem.set_u32(
            vector + VECTOR_END,
            new_capacity.wrapping_mul(bytes).wrapping_add(block),
        );
        e.mem.set_u32(
            vector + VECTOR_LAST,
            new_size.wrapping_mul(bytes).wrapping_add(block),
        );
        e.mem.set_u32(vector + VECTOR_FIRST, block);
        return;
    }
    let after = size.count(e.mem.u32(vector + VECTOR_LAST).wrapping_sub(at));
    e.with_stack(bytes, |e, temporary| {
        (shape.construct)(e, temporary, Ptr::new(value));
        let temporary = temporary.addr();
        if after < count {
            let last = e.mem.u32(vector + VECTOR_LAST);
            (shape.copy)(
                e,
                at,
                last,
                count.wrapping_mul(bytes).wrapping_add(at),
                allocator,
            );
            let last = e.mem.u32(vector + VECTOR_LAST);
            let missing = count.wrapping_sub(size.count(last.wrapping_sub(at)));
            (shape.fill)(e, last, missing, temporary, allocator);
            let last = count
                .wrapping_mul(bytes)
                .wrapping_add(e.mem.u32(vector + VECTOR_LAST));
            e.mem.set_u32(vector + VECTOR_LAST, last);
            let end = last.wrapping_sub(count.wrapping_mul(bytes));
            e.call(shape.assign, &args![at, end, temporary]);
        } else {
            let old_last = e.mem.u32(vector + VECTOR_LAST);
            let last = e.mem.u32(vector + VECTOR_LAST);
            let tail = old_last.wrapping_sub(count.wrapping_mul(bytes));
            let new_last = (shape.copy_tail)(e, tail, old_last, last, allocator);
            e.mem.set_u32(vector + VECTOR_LAST, new_last);
            let tail = old_last.wrapping_sub(count.wrapping_mul(bytes));
            e.call(
                shape.move_backward,
                &args![at, tail, old_last, 0u32, 0u32, 0u32],
            );
            let end = count.wrapping_mul(bytes).wrapping_add(at);
            e.call(shape.assign, &args![at, end, temporary]);
        }
        e.call(shape.destruct, &args![temporary]);
    });
}

// Translated from 00abf210 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00abf210 - the compiler's exception frame (FS:[0] chain) and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map: a 0x34-byte-element vector's `_Insert_n(where, count, value)`
/// (`thiscall`, four words), the object template with the maximum 0x4EC4EC4 (`0065f640`
/// throws), the allocation `0065e550`, the copies `00ac3780` (into new) and `00ac1ee0` (the
/// rest, and past the end), the fill `00ac2540`, the element copy `00ac0530` and destructor
/// `0065ed70`, the range assignment `00ac1670` and the backward move `00ac1f60`.
pub fn fn_00abf210(e: &mut Engine, this: Ptr, _unused_1: u32, at: u32, count: u32, value: u32) {
    let shape = ObjectInsertShape {
        size: 0x34,
        maximum: 0x04ec_4ec4,
        allocate: 0x0065_e550,
        copy: |e, first, last, destination, allocator| {
            fn_00ac3780(e, first, last, destination, allocator)
        },
        copy_tail: |e, first, last, destination, allocator| {
            fn_00ac1ee0(e, first, last, destination, allocator)
        },
        fill: |e, destination, count, value, allocator| {
            fn_00ac2540(e, destination, count, value, allocator, 0, 0)
        },
        construct: fn_00ac0530,
        destruct: VALUE_34_DESTRUCT,
        assign: 0x00ac_1670,
        move_backward: 0x00ac_1f60,
    };
    insert_objects(e, this, at, count, value, &shape);
}

// Translated from 00abfa90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00abfa90 - the compiler's exception frame (FS:[0] chain) and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map: a 0x38-byte-element vector's `_Insert_n(where, count, value)`,
/// as `00abf210` with the maximum 0x4924924, the allocation `0065e5e0`, the copies `00ac37d0`
/// and `00ac1fa0`, the fill `00ac2650`, the element copy `00ac05b0` and destructor `0065edd0`,
/// the range assignment `00ac2760` and the backward move `00ac2020`.
pub fn fn_00abfa90(e: &mut Engine, this: Ptr, _unused_1: u32, at: u32, count: u32, value: u32) {
    let shape = ObjectInsertShape {
        size: 0x38,
        maximum: 0x0492_4924,
        allocate: 0x0065_e5e0,
        copy: |e, first, last, destination, allocator| {
            fn_00ac37d0(e, first, last, destination, allocator)
        },
        copy_tail: |e, first, last, destination, allocator| {
            fn_00ac1fa0(e, first, last, destination, allocator)
        },
        fill: |e, destination, count, value, allocator| {
            fn_00ac2650(e, destination, count, value, allocator, 0, 0)
        },
        construct: fn_00ac05b0,
        destruct: VALUE_38_DESTRUCT,
        assign: 0x00ac_2760,
        move_backward: 0x00ac_2020,
    };
    insert_objects(e, this, at, count, value, &shape);
}

// Translated from 00ac03a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the byte vector's buy (`thiscall`, the count): clears the
/// pointers; 0 answers false; a count above the maximum (`0xFFFFFFFF`) calls the length throw
/// (`00ac0b00`) and answers true; otherwise `count` bytes are allocated (`0065fa30(count, 0)`),
/// first and last point at them and end after them. Answers true.
pub fn fn_00ac03a0(e: &mut Engine, this: Ptr, count: u32) -> bool {
    let vector = this.addr();
    e.mem.set_u32(vector + VECTOR_FIRST, 0);
    e.mem.set_u32(vector + VECTOR_LAST, 0);
    e.mem.set_u32(vector + VECTOR_END, 0);
    if count == 0 {
        return false;
    }
    let maximum = max_or_one(u32::MAX);
    if maximum < count {
        e.call(THROW_LENGTH_00AC0B00, &[]);
        return true;
    }
    let block = e.call(ALLOCATE_BYTES, &args![count, 0u32]).u32();
    e.mem.set_u32(vector + VECTOR_FIRST, block);
    let first = e.mem.u32(vector + VECTOR_FIRST);
    e.mem.set_u32(vector + VECTOR_LAST, first);
    let first = e.mem.u32(vector + VECTOR_FIRST);
    e.mem
        .set_u32(vector + VECTOR_END, first.wrapping_add(count));
    true
}

// Translated from 00ac0460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a checked iterator's `operator!=` (`thiscall`, the other
/// iterator): both must belong to the same container (a proxy that is set and equal, else
/// `_invalid_parameter_noinfo`); answers 1 when the pointers differ, else 0.
pub fn fn_00ac0460(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    let proxy = e.mem.u32(this.addr());
    if proxy == 0 || proxy != e.mem.u32(other.addr()) {
        e.call(INVALID_PARAMETER, &[]);
    }
    u32::from(e.mem.u32(this.addr() + 4) != e.mem.u32(other.addr() + 4))
}

// Translated from 00ac0530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac0530 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: the 0x34-byte element's copy constructor (`thiscall`, the
/// source): the string at +0 is made empty and assigned the source's, then the 12-byte-element
/// vector at +0x1C is copy-constructed (`00ac0640`). Returns `this`.
pub fn fn_00ac0530(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(STRING_TIDY, &args![this, 0u32, 0u32]);
    let npos: u32 = e.global(STRING_NPOS);
    e.call(STRING_ASSIGN, &args![this, source, 0u32, npos]);
    fn_00ac0640(
        e,
        Ptr::new(this.addr() + 0x1c),
        Ptr::new(source.addr() + 0x1c),
    );
    this
}

// Translated from 00ac05b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac05b0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: the 0x38-byte element's copy constructor (`thiscall`, the
/// source): the string at +0 is made empty and assigned the source's, the word at +0x1C is
/// copied and the 4-byte-element vector at +0x20 is copy-constructed (`00ac08b0`). Returns
/// `this`.
pub fn fn_00ac05b0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(STRING_TIDY, &args![this, 0u32, 0u32]);
    let npos: u32 = e.global(STRING_NPOS);
    e.call(STRING_ASSIGN, &args![this, source, 0u32, npos]);
    let word = e.mem.u32(source.addr() + 0x1c);
    e.mem.set_u32(this.addr() + 0x1c, word);
    fn_00ac08b0(
        e,
        Ptr::new(this.addr() + 0x20),
        Ptr::new(source.addr() + 0x20),
    );
    this
}

/// The checked end and begin iterators of `source` the copy constructors make: (end proxy,
/// end pointer, begin proxy, begin pointer).
fn source_range(e: &mut Engine, source: u32) -> (u32, u32, u32, u32) {
    let last = e.mem.u32(source + VECTOR_LAST);
    let end_proxy = checked_iterator(e, source, last);
    let first = e.mem.u32(source + VECTOR_FIRST);
    let begin_proxy = checked_iterator(e, source, first);
    (end_proxy, last, begin_proxy, first)
}

// Translated from 00ac0640 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac0640 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a 12-byte-element vector's copy constructor (`thiscall`, the
/// source). Builds the vector base (`0065dc90`, given the word the code pushes as the
/// allocator: `this`) and buys room for the source's count (`0065cfd0`); when that succeeds, the
/// source's iterators are checked and its elements copied to the new first pointer
/// (`00ac3370`), which sets the last pointer. Returns `this`.
pub fn fn_00ac0640(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let vector = this.addr();
    e.call(0x0065_dc90, &args![vector, vector]);
    let count = vector_size(e, source.addr(), ElementSize::Divide(0xc));
    if e.call(0x0065_cfd0, &args![vector, count]).u8() == 0 {
        return this;
    }
    let (_, end, _, begin) = source_range(e, source.addr());
    let destination = e.mem.u32(vector + VECTOR_FIRST);
    let last = fn_00ac3370(e, begin, end, destination, vector + VECTOR_ALLOCATOR, 0, 0);
    e.mem.set_u32(vector + VECTOR_LAST, last);
    this
}

// Translated from 00ac08b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac08b0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a 4-byte-element vector's copy constructor (`thiscall`, the
/// source). Builds the vector base (`00ad33c0`, given `this` as the allocator word) and buys
/// room for the source's count (`00abc260`); when that succeeds, the source's iterators are
/// checked and its elements copied to the new first pointer (`00ac2060`, given the iterators'
/// words, the destination and the allocator's address), which sets the last pointer. Returns
/// `this`.
pub fn fn_00ac08b0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let vector = this.addr();
    e.call(0x00ad_33c0, &args![vector, vector]);
    let count = vector_size(e, source.addr(), ElementSize::Shift(2));
    if !fn_00abc260(e, this, count) {
        return this;
    }
    let (end_proxy, end, begin_proxy, begin) = source_range(e, source.addr());
    let destination = e.mem.u32(vector + VECTOR_FIRST);
    let last = e
        .call(
            0x00ac_2060,
            &args![
                begin_proxy,
                begin,
                end_proxy,
                end,
                destination,
                vector + VECTOR_ALLOCATOR
            ],
        )
        .u32();
    e.mem.set_u32(vector + VECTOR_LAST, last);
    this
}

// Translated from 00ac1000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a checked 4-byte-element iterator's `operator+=` (`thiscall`, the
/// count). It must have a container (proxy set); the new pointer must not pass the container's
/// last pointer (+0x10) nor fall below its first (+0x0C), else `_invalid_parameter_noinfo`. The
/// container is the word the proxy points at; without a proxy the code reads the container
/// fields at address 0. The pointer moves by `count` elements. Returns `this`.
pub fn fn_00ac1000(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    let iterator = this.addr();
    check(e, e.mem.u32(iterator) != 0);
    let container = |e: &Engine| {
        let proxy = e.mem.u32(iterator);
        if proxy != 0 {
            e.mem.u32(proxy)
        } else {
            0
        }
    };
    let moved = |e: &Engine| e.mem.u32(iterator + 4).wrapping_add(count.wrapping_mul(4));
    let owner = container(e);
    if moved(e) > e.mem.u32(owner.wrapping_add(VECTOR_LAST)) {
        e.call(INVALID_PARAMETER, &[]);
    } else {
        let owner = container(e);
        if moved(e) < e.mem.u32(owner.wrapping_add(VECTOR_FIRST)) {
            e.call(INVALID_PARAMETER, &[]);
        }
    }
    let pointer = moved(e);
    e.mem.set_u32(iterator + 4, pointer);
    this
}

// Translated from 00ac13c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: first, last, destination): moves the 4-byte elements
/// from first to last to the destination (`memmove_s`, when the signed count is above 0) and
/// returns the destination's end.
pub fn fn_00ac13c0(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    let count = (last.wrapping_sub(first) as i32) >> 2;
    let end = destination.wrapping_add((count as u32).wrapping_mul(4));
    if count > 0 {
        let bytes = (count as u32).wrapping_shl(2);
        e.call(MEMMOVE_S, &args![destination, bytes, first, bytes]);
    }
    end
}

// Translated from 00ac16e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words: destination, count, the byte's address and
/// the allocator's): fills `count` bytes with the byte (`00ac3300`).
pub fn fn_00ac16e0(e: &mut Engine, destination: u32, count: u32, value: u32, _unused_4: u32) {
    fn_00ac3300(e, destination, count, value);
}

// Translated from 00ac1720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: count and an unused hint): allocates `count` 4-byte
/// elements through the memory manager (`00aa3e40`). A count of 0 allocates 0 bytes; a count
/// whose bytes overflow (`0xFFFFFFFF / count < 4`) throws `std::bad_alloc` (the exception's
/// base constructor `00ec78f1` with the address of a null message, its vtable `010176f8`,
/// `_CxxThrowException` with `01140ef8`; it does not return in the game, the code after it goes
/// on).
// The exe tests count == 0 before dividing; keep its order rather than checked_div.
#[allow(clippy::manual_checked_ops)]
pub fn fn_00ac1720(e: &mut Engine, count: u32, _unused_2: u32) -> u32 {
    let count = if count == 0 {
        0
    } else {
        if u32::MAX / count < 4 {
            e.with_stack(0x10, |e, locals| {
                let message = locals.addr();
                let exception = locals.addr() + 4;
                e.mem.set_u32(message, 0);
                e.call(EXCEPTION_CONSTRUCT, &args![exception, message]);
                e.mem.set_u32(exception, BAD_ALLOC_VTABLE);
                e.call(CXX_THROW_EXCEPTION, &args![exception, BAD_ALLOC_THROW_INFO]);
            });
        }
        count
    };
    e.call(
        MEMORY_MANAGER_ALLOCATE,
        &args![MEMORY_MANAGER_OBJECT, count.wrapping_shl(2)],
    )
    .u32()
}

// Translated from 00ac1ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words: first, last, destination, allocator): the
/// 0x34-byte elements' uninitialized copy through `00ac3bd0` (with two tag words). Returns
/// the destination's end.
pub fn fn_00ac1ee0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00ac3bd0(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00ac1fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words): the 0x38-byte elements' uninitialized copy
/// through `00ac3ce0`. Returns the destination's end.
pub fn fn_00ac1fa0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00ac3ce0(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00ac20d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac20d0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words: destination, count, the value's address, the
/// allocator and two tags): constructs `count` 12-byte copies of the value (each slot that is
/// not null gets the three words).
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac20d0(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) {
    let mut destination = destination;
    let mut count = count;
    while count > 0 {
        if destination != 0 {
            for i in 0..3 {
                let word = e.mem.u32(value + 4 * i);
                e.mem.set_u32(destination + 4 * i, word);
            }
        }
        count -= 1;
        destination = destination.wrapping_add(0xc);
    }
}

/// A float pair copied with `FLD`/`FSTP` (signalling NaNs come out quiet).
fn copy_float_pair(e: &mut Engine, destination: u32, source: u32) {
    let x = x87_quiet_bits(e.mem.u32(source));
    e.mem.set_u32(destination, x);
    let y = x87_quiet_bits(e.mem.u32(source + 4));
    e.mem.set_u32(destination + 4, y);
}

// Translated from 00ac2290 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac2290 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words): constructs `count` copies of a float pair
/// (`FLD`/`FSTP`) at the destination, skipping null slots.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac2290(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) {
    let mut destination = destination;
    let mut count = count;
    while count > 0 {
        if destination != 0 {
            copy_float_pair(e, destination, value);
        }
        count -= 1;
        destination = destination.wrapping_add(8);
    }
}

// Translated from 00ac2540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac2540 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words): copy-constructs (`00ac0530`) `count`
/// 0x34-byte elements from the value at the destination, skipping null slots.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac2540(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) {
    let mut destination = destination;
    let mut count = count;
    while count > 0 {
        if destination != 0 {
            fn_00ac0530(e, Ptr::new(destination), Ptr::new(value));
        }
        count -= 1;
        destination = destination.wrapping_add(0x34);
    }
}

// Translated from 00ac2650 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac2650 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words): copy-constructs (`00ac05b0`) `count`
/// 0x38-byte elements from the value at the destination, skipping null slots.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac2650(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) {
    let mut destination = destination;
    let mut count = count;
    while count > 0 {
        if destination != 0 {
            fn_00ac05b0(e, Ptr::new(destination), Ptr::new(value));
        }
        count -= 1;
        destination = destination.wrapping_add(0x38);
    }
}

// Translated from 00ac2f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 4-byte-element vector's `clear()`: erases (`00abc180`) from the
/// checked begin iterator to the checked end iterator.
pub fn fn_00ac2f70(e: &mut Engine, this: Ptr) {
    let vector = this.addr();
    let last = e.mem.u32(vector + VECTOR_LAST);
    let end_proxy = checked_iterator(e, vector, last);
    let first = e.mem.u32(vector + VECTOR_FIRST);
    let begin_proxy = checked_iterator(e, vector, first);
    e.with_stack(8, |e, result| {
        fn_00abc180(e, this, result, begin_proxy, first, end_proxy, last);
    });
}

// Translated from 00ac3300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: destination, count, the byte's address): sets `count`
/// bytes to the byte.
pub fn fn_00ac3300(e: &mut Engine, destination: u32, count: u32, value: u32) {
    let mut destination = destination;
    let mut count = count;
    while count > 0 {
        let byte = e.mem.u8(value);
        e.mem.set_u8(destination, byte);
        count -= 1;
        destination = destination.wrapping_add(1);
    }
}

// Translated from 00ac3370 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac3370 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words: first, last, destination, allocator, two
/// tags): copies the 12-byte elements from first to last to the destination, skipping null
/// slots. Returns the destination's end.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac3370(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let mut first = first;
    let mut destination = destination;
    while first != last {
        if destination != 0 {
            for i in 0..3 {
                let word = e.mem.u32(first + 4 * i);
                e.mem.set_u32(destination + 4 * i, word);
            }
        }
        destination = destination.wrapping_add(0xc);
        first = first.wrapping_add(0xc);
    }
    destination
}

// Translated from 00ac3450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: destination, count, the value's address): sets `count`
/// words to the value.
pub fn fn_00ac3450(e: &mut Engine, destination: u32, count: u32, value: u32) {
    let mut destination = destination;
    let mut count = count;
    while count > 0 {
        let word = e.mem.u32(value);
        e.mem.set_u32(destination, word);
        count -= 1;
        destination = destination.wrapping_add(4);
    }
}

// Translated from 00ac3580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words): the 12-byte elements' copy through
/// `00ac3370` (with two tag words). Returns the destination's end.
pub fn fn_00ac3580(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00ac3370(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00ac35d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words: first, last, destination, allocator): moves
/// the 4-byte elements from first to last to the destination (`memmove_s`, when the count is
/// not 0) and returns the destination's end.
pub fn fn_00ac35d0(e: &mut Engine, first: u32, last: u32, destination: u32, _unused_4: u32) -> u32 {
    let count = ((last.wrapping_sub(first) as i32) >> 2) as u32;
    let end = destination.wrapping_add(count.wrapping_mul(4));
    if count != 0 {
        let bytes = count.wrapping_shl(2);
        e.call(MEMMOVE_S, &args![destination, bytes, first, bytes]);
    }
    end
}

// Translated from 00ac3690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words): the float pairs' copy through `00ac3900`
/// (with two tag words). Returns the destination's end.
pub fn fn_00ac3690(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00ac3900(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00ac3780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words): the 0x34-byte elements' copy through
/// `00ac3bd0`. Returns the destination's end.
pub fn fn_00ac3780(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00ac3bd0(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00ac37d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words): the 0x38-byte elements' copy through
/// `00ac3ce0`. Returns the destination's end.
pub fn fn_00ac37d0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00ac3ce0(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00ac3900 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac3900 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words): copies the float pairs from first to last to
/// the destination (`FLD`/`FSTP`), skipping null slots. Returns the destination's end.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac3900(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let mut first = first;
    let mut destination = destination;
    while first != last {
        if destination != 0 {
            copy_float_pair(e, destination, first);
        }
        destination = destination.wrapping_add(8);
        first = first.wrapping_add(8);
    }
    destination
}

// Translated from 00ac3bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac3bd0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words): copy-constructs (`00ac0530`) the 0x34-byte
/// elements from first to last at the destination, skipping null slots. Returns the
/// destination's end.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac3bd0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let mut first = first;
    let mut destination = destination;
    while first != last {
        if destination != 0 {
            fn_00ac0530(e, Ptr::new(destination), Ptr::new(first));
        }
        destination = destination.wrapping_add(0x34);
        first = first.wrapping_add(0x34);
    }
    destination
}

// Translated from 00ac3ce0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac3ce0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words): copy-constructs (`00ac05b0`) the 0x38-byte
/// elements from first to last at the destination, skipping null slots. Returns the
/// destination's end.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac3ce0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let mut first = first;
    let mut destination = destination;
    while first != last {
        if destination != 0 {
            fn_00ac05b0(e, Ptr::new(destination), Ptr::new(first));
        }
        destination = destination.wrapping_add(0x38);
        first = first.wrapping_add(0x38);
    }
    destination
}

// Translated from 00ac3df0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac3df0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: `FutBinaryFileC`'s constructor (`thiscall`, the file id string by
/// value): sets the vtable, makes the name (+0x04) and the id (+0x20) empty strings, assigns the
/// id, asserts its length is 8 (`fr2Assert(binaryFile.cpp, 0x22)` otherwise), clears the file
/// pointer (+0x3C), sets the id's character 5 (the version) to `'0'` (after the library's
/// length check) and destroys the argument. Returns `this`.
pub fn fn_00ac3df0(e: &mut Engine, this: Ptr, file_id: StringArg) -> Ptr {
    let base = this.addr();
    with_string_arg(e, file_id, |e, id| {
        e.mem.set_u32(base, FUT_BINARY_FILE_VTABLE);
        e.call(STRING_TIDY, &args![base + 4, 0u32, 0u32]);
        e.call(STRING_TIDY, &args![base + 0x20, 0u32, 0u32]);
        let npos: u32 = e.global(STRING_NPOS);
        e.call(STRING_ASSIGN, &args![base + 0x20, id, 0u32, npos]);
        if e.mem.u32(base + 0x20 + STRING_LENGTH) != 8 {
            e.call(FR2_ASSERT, &args![BINARY_FILE_SOURCE, 0x22u32]);
        }
        e.mem.set_u32(base + 0x3c, 0);
        check(e, e.mem.u32(base + 0x20 + STRING_LENGTH) >= 5);
        let chars = string_chars(e, base + 0x20);
        e.mem.set_u8(chars.wrapping_add(5), b'0');
        e.call(STRING_TIDY, &args![id, 1u32, 0u32]);
    });
    this
}

/// `std::string::assign(other, 0, npos)` (`0044a580`) of the string at `to` from `from`.
fn assign_string(e: &mut Engine, to: u32, from: u32) {
    let npos: u32 = e.global(STRING_NPOS);
    e.call(STRING_ASSIGN, &args![to, from, 0u32, npos]);
}

/// Copies `count` words from `from` to `to` (`MOV`s).
fn copy_words(e: &mut Engine, to: u32, from: u32, count: u32) {
    for i in 0..count {
        let word = e.mem.u32(from.wrapping_add(4 * i));
        e.mem.set_u32(to.wrapping_add(4 * i), word);
    }
}

// Translated from 00aba3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 12-byte-element vector's `insert(where, value)` (`thiscall`,
/// four words: the result, the iterator's proxy and pointer, the value's address). The
/// iterator's offset from the begin iterator (0 for an empty vector; otherwise its proxy must be
/// set and equal to the begin iterator's, and the offset is the signed element count), then one
/// copy inserted (`00abca90`), and the result is the new begin iterator advanced by the offset
/// (`00ac0dc0`). Returns `result`.
pub fn fn_00aba3c0(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    where_proxy: u32,
    at: u32,
    value: u32,
) -> Ptr {
    let vector = this.addr();
    let offset = if vector_size(e, vector, ElementSize::Divide(0xc)) == 0 {
        0
    } else {
        let first = e.mem.u32(vector + VECTOR_FIRST);
        let begin_proxy = checked_iterator(e, vector, first);
        check(e, where_proxy != 0 && where_proxy == begin_proxy);
        ElementSize::Divide(0xc).count(at.wrapping_sub(first))
    };
    fn_00abca90(e, this, where_proxy, at, 1, value);
    let first = e.mem.u32(vector + VECTOR_FIRST);
    let proxy = checked_iterator(e, vector, first);
    e.with_stack(8, |e, iterator| {
        e.mem.set_u32(iterator.addr(), proxy);
        e.mem.set_u32(iterator.addr() + 4, first);
        fn_00ac0dc0(e, iterator, offset);
        let proxy = e.mem.u32(iterator.addr());
        let pointer = e.mem.u32(iterator.addr() + 4);
        e.mem.set_u32(result.addr(), proxy);
        e.mem.set_u32(result.addr() + 4, pointer);
    });
    result
}

/// The `erase(first, last)` template: both iterators are checked and remade on this vector's
/// proxy; when they differ (`00ac0460`), `finish` moves the tail down and ends the vector. The
/// result is the first iterator; returns `result`.
fn erase_range(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    first: u32,
    last: u32,
    finish: impl FnOnce(&mut Engine, u32),
) -> Ptr {
    let vector = this.addr();
    let first_proxy = checked_iterator(e, vector, first);
    let last_proxy = checked_iterator(e, vector, last);
    e.with_stack(0x10, |e, iterators| {
        let begin = iterators.addr();
        let end = iterators.addr() + 8;
        e.mem.set_u32(begin, first_proxy);
        e.mem.set_u32(begin + 4, first);
        e.mem.set_u32(end, last_proxy);
        e.mem.set_u32(end + 4, last);
        if fn_00ac0460(e, Ptr::new(begin), Ptr::new(end)) != 0 {
            finish(e, vector);
        }
    });
    e.mem.set_u32(result.addr(), first_proxy);
    e.mem.set_u32(result.addr() + 4, first);
    result
}

/// The plain elements' erase tail: the elements from `last` to the end move down to `first`
/// (`mover`: first, last, destination), an empty walk over the freed elements, and the last
/// pointer becomes the end of the moved block.
fn erase_plain_tail(
    e: &mut Engine,
    vector: u32,
    first: u32,
    last: u32,
    bytes: u32,
    mover: fn(&mut Engine, u32, u32, u32) -> u32,
) {
    let old_last = e.mem.u32(vector + VECTOR_LAST);
    let moved = mover(e, last, old_last, first);
    let end = e.mem.u32(vector + VECTOR_LAST);
    let mut element = moved;
    while element != end {
        element = element.wrapping_add(bytes);
    }
    e.mem.set_u32(vector + VECTOR_LAST, moved);
}

/// The 6-word movers of the string elements (first, last, destination and three tags).
type MoverFn = fn(&mut Engine, u32, u32, u32, u32, u32, u32) -> u32;

/// The string elements' erase tail: the elements from `last` to the end are assigned down to
/// `first` (`mover`), the leftover ones destroyed (`destroy_range`, `thiscall`: from, to) and the
/// last pointer becomes the end of the moved block.
fn erase_strings_tail(
    e: &mut Engine,
    vector: u32,
    first: u32,
    last: u32,
    mover: MoverFn,
    destroy_range: u32,
) {
    let old_last = e.mem.u32(vector + VECTOR_LAST);
    let moved = mover(e, last, old_last, first, 0, 0, 0);
    let end = e.mem.u32(vector + VECTOR_LAST);
    e.call(destroy_range, &args![vector, moved, end]);
    e.mem.set_u32(vector + VECTOR_LAST, moved);
}

/// The 0x34- and 0x38-byte elements' erase tail: as [`erase_strings_tail`], but each leftover
/// element is destroyed (`destruct`, `thiscall`). The free of each element that follows is
/// guarded by a mask the compiler folded to 0 (`XOR EAX,EAX; AND EAX,1`), so it never runs and
/// is left out.
fn erase_objects_tail(
    e: &mut Engine,
    vector: u32,
    first: u32,
    last: u32,
    bytes: u32,
    mover: MoverFn,
    destruct: u32,
) {
    let old_last = e.mem.u32(vector + VECTOR_LAST);
    let moved = mover(e, last, old_last, first, 0, 0, 0);
    let end = e.mem.u32(vector + VECTOR_LAST);
    let mut element = moved;
    while element != end {
        e.call(destruct, &args![element]);
        element = element.wrapping_add(bytes);
    }
    e.mem.set_u32(vector + VECTOR_LAST, moved);
}

// Translated from 00aba9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 12-byte-element vector's `erase(first, last)` (`thiscall`, five
/// words: the result and the two iterators), the tail moved by `00ac1170`. Returns `result`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00aba9a0(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    _first_proxy: u32,
    first: u32,
    _last_proxy: u32,
    last: u32,
) -> Ptr {
    erase_range(e, this, result, first, last, |e, vector| {
        erase_plain_tail(e, vector, first, last, 0xc, fn_00ac1170)
    })
}

// Translated from 00abac80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 16-byte-element vector's `erase(first, last)`, as `00aba9a0`
/// with the mover `00ac1220`. Returns `result`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00abac80(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    _first_proxy: u32,
    first: u32,
    _last_proxy: u32,
    last: u32,
) -> Ptr {
    erase_range(e, this, result, first, last, |e, vector| {
        erase_plain_tail(e, vector, first, last, 0x10, fn_00ac1220)
    })
}

// Translated from 00abaf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an 8-byte-element vector's `erase(first, last)`, as `00aba9a0`
/// with the mover `00ac12d0`. Returns `result`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00abaf60(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    _first_proxy: u32,
    first: u32,
    _last_proxy: u32,
    last: u32,
) -> Ptr {
    erase_range(e, this, result, first, last, |e, vector| {
        erase_plain_tail(e, vector, first, last, 8, fn_00ac12d0)
    })
}

/// The range destructions of the 0x20- and 0x2C-byte elements (`0065dc60`, `0065dd90`;
/// `thiscall` on the vector: from, to).
const DESTROY_RANGE_20: u32 = 0x0065_dc60;
const DESTROY_RANGE_2C: u32 = 0x0065_dd90;

// Translated from 00abb280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 0x20-byte-element (a word and a string) vector's
/// `erase(first, last)`: the tail assigned down by `00ac1780` (given two tag words the code
/// copies from uninitialized stack bytes and a 0; the mover does not read them), the leftover
/// elements destroyed by `0065dc60`. Returns `result`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00abb280(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    _first_proxy: u32,
    first: u32,
    _last_proxy: u32,
    last: u32,
) -> Ptr {
    erase_range(e, this, result, first, last, |e, vector| {
        erase_strings_tail(e, vector, first, last, fn_00ac1780, DESTROY_RANGE_20)
    })
}

// Translated from 00abb5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 0x2C-byte-element (four words and a string) vector's
/// `erase(first, last)`, as `00abb280` with the mover `00ac1810` and the range destruction
/// `0065dd90`. Returns `result`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00abb5f0(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    _first_proxy: u32,
    first: u32,
    _last_proxy: u32,
    last: u32,
) -> Ptr {
    erase_range(e, this, result, first, last, |e, vector| {
        erase_strings_tail(e, vector, first, last, fn_00ac1810, DESTROY_RANGE_2C)
    })
}

// Translated from 00abb930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 0x34-byte-element vector's `erase(first, last)`: the tail
/// assigned down by `00ac18c0`, each leftover element destroyed (`0065ed70`). Returns
/// `result`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00abb930(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    _first_proxy: u32,
    first: u32,
    _last_proxy: u32,
    last: u32,
) -> Ptr {
    erase_range(e, this, result, first, last, |e, vector| {
        erase_objects_tail(e, vector, first, last, 0x34, fn_00ac18c0, VALUE_34_DESTRUCT)
    })
}

// Translated from 00abbd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 0x38-byte-element vector's `erase(first, last)`, as `00abb930`
/// with the mover `00ac1980` and the destructor `0065edd0`. Returns `result`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00abbd10(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    _first_proxy: u32,
    first: u32,
    _last_proxy: u32,
    last: u32,
) -> Ptr {
    erase_range(e, this, result, first, last, |e, vector| {
        erase_objects_tail(e, vector, first, last, 0x38, fn_00ac1980, VALUE_38_DESTRUCT)
    })
}

// Translated from 00abbeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a checked iterator's constructor (`thiscall`: the pointer and the
/// vector): the proxy cleared, then (the pointer must lie within the vector, else
/// `_invalid_parameter_noinfo`) the vector's proxy and the pointer. Returns `this`.
pub fn fn_00abbeb0(e: &mut Engine, this: Ptr, pointer: u32, vector: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    let ok = vector != 0
        && e.mem.u32(vector + VECTOR_FIRST) <= pointer
        && pointer <= e.mem.u32(vector + VECTOR_LAST);
    check(e, ok);
    let proxy = e.mem.u32(vector + VECTOR_PROXY);
    e.mem.set_u32(this.addr(), proxy);
    e.mem.set_u32(this.addr() + 4, pointer);
    this
}

// Translated from 00abbf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a checked iterator's `operator==` (`thiscall`, the other): both
/// must belong to the same container (a proxy that is set and equal, else
/// `_invalid_parameter_noinfo`); whether the pointers are equal.
pub fn fn_00abbf00(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let proxy = e.mem.u32(this.addr());
    if proxy == 0 || proxy != e.mem.u32(other.addr()) {
        e.call(INVALID_PARAMETER, &[]);
    }
    e.mem.u32(this.addr() + 4) == e.mem.u32(other.addr() + 4)
}

// Translated from 00abbf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a checked iterator's `operator*` (`thiscall`): the proxy must be
/// set and the pointer below the container's last pointer (else `_invalid_parameter_noinfo`;
/// without a proxy the code reads the container's fields at address 0). Returns the pointer.
pub fn fn_00abbf50(e: &mut Engine, this: Ptr) -> u32 {
    let iterator = this.addr();
    check(e, e.mem.u32(iterator) != 0);
    let proxy = e.mem.u32(iterator);
    let container = if proxy != 0 { e.mem.u32(proxy) } else { 0 };
    if e.mem.u32(iterator + 4) >= e.mem.u32(container.wrapping_add(VECTOR_LAST)) {
        e.call(INVALID_PARAMETER, &[]);
    }
    e.mem.u32(iterator + 4)
}

// Translated from 00ac04c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 0x2C-byte element's copy constructor (`thiscall`, the
/// source): its first four words, then the string at +0x10 made empty and assigned the
/// source's. Returns `this`.
pub fn fn_00ac04c0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    copy_words(e, this.addr(), source.addr(), 4);
    let string = this.addr() + 0x10;
    e.call(STRING_TIDY, &args![string, 0u32, 0u32]);
    assign_string(e, string, source.addr() + 0x10);
    this
}

// Translated from 00ac0ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `this` not read; three words: destination, count, the
/// value's address): sets `count` words to the value (`00ac3450`); returns the destination's
/// end.
pub fn fn_00ac0ab0(e: &mut Engine, _this: u32, destination: u32, count: u32, value: u32) -> u32 {
    fn_00ac3450(e, destination, count, value);
    destination.wrapping_add(count.wrapping_mul(4))
}

/// `"vector<T> too long"`, the `std::string` constructor from text (`0044a4c0`), the
/// `std::runtime_error`-style constructor of the exception (`0044a310`, `thiscall`: the
/// string's address), the vtable it then gets (`010173dc`) and the throw information
/// (`01140cb0`).
const TOO_LONG_TEXT: u32 = 0x0101_73c4;
const STRING_FROM_TEXT: u32 = 0x0044_a4c0;
const LENGTH_ERROR_CONSTRUCT: u32 = 0x0044_a310;
const LENGTH_ERROR_VTABLE: u32 = 0x0101_73dc;
const LENGTH_ERROR_THROW_INFO: u32 = 0x0114_0cb0;

// Translated from 00ac0b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac0b00 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (no arguments): throws the vectors' length error: a string of
/// `"vector<T> too long"` (`0044a4c0`), the exception made from it (`0044a310`) with the vtable
/// `010173dc`, and `_CxxThrowException` with `01140cb0` (it does not return in the game).
pub fn fn_00ac0b00(e: &mut Engine) {
    e.with_stack(0x50, |e, locals| {
        let text = locals.addr();
        let exception = locals.addr() + 0x1c;
        e.call(STRING_FROM_TEXT, &args![text, TOO_LONG_TEXT]);
        e.call(LENGTH_ERROR_CONSTRUCT, &args![exception, text]);
        e.mem.set_u32(exception, LENGTH_ERROR_VTABLE);
        e.call(
            CXX_THROW_EXCEPTION,
            &args![exception, LENGTH_ERROR_THROW_INFO],
        );
    });
}

/// A checked iterator's `operator+=` over `bytes`-sized elements, as `00ac1000` does it for 4:
/// the proxy must be set; the moved pointer must neither pass the container's last pointer nor
/// fall below its first (without a proxy the code reads them at address 0). Returns `this`.
fn advance_iterator(e: &mut Engine, this: Ptr, count: u32, bytes: u32) -> Ptr {
    let iterator = this.addr();
    check(e, e.mem.u32(iterator) != 0);
    let container = |e: &Engine| {
        let proxy = e.mem.u32(iterator);
        if proxy != 0 {
            e.mem.u32(proxy)
        } else {
            0
        }
    };
    let moved = |e: &Engine| {
        count
            .wrapping_mul(bytes)
            .wrapping_add(e.mem.u32(iterator + 4))
    };
    let owner = container(e);
    if moved(e) > e.mem.u32(owner.wrapping_add(VECTOR_LAST)) {
        e.call(INVALID_PARAMETER, &[]);
    } else {
        let owner = container(e);
        if moved(e) < e.mem.u32(owner.wrapping_add(VECTOR_FIRST)) {
            e.call(INVALID_PARAMETER, &[]);
        }
    }
    let pointer = moved(e);
    e.mem.set_u32(iterator + 4, pointer);
    this
}

// Translated from 00ac0b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a checked 0x20-byte-element iterator's `operator+=` (`thiscall`,
/// the count). Returns `this`.
pub fn fn_00ac0b80(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    advance_iterator(e, this, count, 0x20)
}

// Translated from 00ac0c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a checked 0x2C-byte-element iterator's `operator+=`. Returns
/// `this`.
pub fn fn_00ac0c40(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    advance_iterator(e, this, count, 0x2c)
}

// Translated from 00ac0d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a checked 0x34-byte-element iterator's `operator+=`. Returns
/// `this`.
pub fn fn_00ac0d00(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    advance_iterator(e, this, count, 0x34)
}

// Translated from 00ac0dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a checked 12-byte-element iterator's `operator+=`. Returns
/// `this`.
pub fn fn_00ac0dc0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    advance_iterator(e, this, count, 0xc)
}

// Translated from 00ac0e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a checked 16-byte-element iterator's `operator+=`. Returns
/// `this`.
pub fn fn_00ac0e80(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    advance_iterator(e, this, count, 0x10)
}

// Translated from 00ac0f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a checked 8-byte-element iterator's `operator+=`. Returns
/// `this`.
pub fn fn_00ac0f40(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    advance_iterator(e, this, count, 8)
}

// Translated from 00ac10c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an `NiPoint3` sum (`thiscall`: the result, the other point):
/// each coordinate `other + this` (`FLD`, `FADD`), stored as `float`s and copied to the
/// result. Returns `result`.
pub fn fn_00ac10c0(e: &mut Engine, this: Ptr, result: Ptr, other: Ptr) -> Ptr {
    let mut sum = [0u32; 3];
    for (i, word) in sum.iter_mut().enumerate() {
        let at = 4 * i as u32;
        let a = x87_load32(e.mem.u32(other.addr() + at));
        let b = x87_m32(e.mem.u32(this.addr() + at));
        *word = x87_store32(x87_add(a, b));
    }
    for (i, word) in sum.iter().enumerate() {
        e.mem
            .set_u32(result.addr() + 4 * i as u32, x87_quiet_bits(*word));
    }
    result
}

// Translated from 00ac1120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an `NiPoint3` scaled by a `float` (`thiscall`: the result, the
/// scale): each coordinate `scale * this` (`FLD`, `FMUL`), stored as `float`s and copied to the
/// result. Returns `result`.
pub fn fn_00ac1120(e: &mut Engine, this: Ptr, result: Ptr, scale: f32) -> Ptr {
    let mut product = [0u32; 3];
    for (i, word) in product.iter_mut().enumerate() {
        let a = x87_load32(scale.to_bits());
        let b = x87_m32(e.mem.u32(this.addr() + 4 * i as u32));
        *word = x87_store32(x87_mul(a, b));
    }
    for (i, word) in product.iter().enumerate() {
        e.mem
            .set_u32(result.addr() + 4 * i as u32, x87_quiet_bits(*word));
    }
    result
}

/// The forward copy of elements by `MOV`s from first to last to the destination;
/// returns `destination + count * size` (the count a signed division by the size).
fn copy_forward(e: &mut Engine, first: u32, last: u32, destination: u32, size: ElementSize) -> u32 {
    let bytes = size.bytes();
    let end = size
        .count(last.wrapping_sub(first))
        .wrapping_mul(bytes)
        .wrapping_add(destination);
    let mut to = destination;
    let mut from = first;
    while from != last {
        copy_words(e, to, from, bytes / 4);
        to = to.wrapping_add(bytes);
        from = from.wrapping_add(bytes);
    }
    end
}

/// The backward copy of elements by `MOV`s from last down to first, ending at the
/// destination; returns `destination - count * size`.
fn copy_backward(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    size: ElementSize,
) -> u32 {
    let bytes = size.bytes();
    let start = destination.wrapping_sub(size.count(last.wrapping_sub(first)).wrapping_mul(bytes));
    let mut to = destination;
    let mut from = last;
    while first != from {
        from = from.wrapping_sub(bytes);
        to = to.wrapping_sub(bytes);
        copy_words(e, to, from, bytes / 4);
    }
    start
}

// Translated from 00ac1170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: first, last, destination): copies the 12-byte elements
/// forward; returns the destination's end.
pub fn fn_00ac1170(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    copy_forward(e, first, last, destination, ElementSize::Divide(0xc))
}

// Translated from 00ac1220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: first, last, destination): copies the 16-byte elements
/// forward; returns the destination's end.
pub fn fn_00ac1220(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    copy_forward(e, first, last, destination, ElementSize::Shift(4))
}

// Translated from 00ac12d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: first, last, destination): copies the 8-byte elements
/// forward (as words); returns the destination's end.
pub fn fn_00ac12d0(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    copy_forward(e, first, last, destination, ElementSize::Shift(3))
}

// Translated from 00ac1370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: first, last, destination end): the 12-byte elements'
/// backward copy through `00ac1a50` (with two tag words the code copies from uninitialized stack
/// bytes and a 0; the callee does not read them). Returns its result.
pub fn fn_00ac1370(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    fn_00ac1a50(e, first, last, destination, 0, 0, 0)
}

// Translated from 00ac1440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: first, last, destination end): moves the 4-byte elements
/// from first to last so they end at the destination (`memmove_s`, when the signed count is
/// above 0); returns the moved block's start.
pub fn fn_00ac1440(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    let count = (last.wrapping_sub(first) as i32) >> 2;
    let start = destination.wrapping_sub((count as u32).wrapping_mul(4));
    if count > 0 {
        let bytes = (count as u32).wrapping_shl(2);
        e.call(MEMMOVE_S, &args![start, bytes, first, bytes]);
    }
    start
}

// Translated from 00ac14e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words): the 16-byte elements' backward copy through
/// `00ac1bf0`. Returns its result.
pub fn fn_00ac14e0(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    fn_00ac1bf0(e, first, last, destination, 0, 0, 0)
}

// Translated from 00ac1530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words): the 8-byte elements' backward copy through
/// `00ac1d10`. Returns its result.
pub fn fn_00ac1530(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    fn_00ac1d10(e, first, last, destination, 0, 0, 0)
}

// Translated from 00ac1580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, three words): the 0x20-byte elements' backward
/// assignment through `00ac3030`. Returns its result.
pub fn fn_00ac1580(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    fn_00ac3030(e, first, last, destination, 0, 0, 0)
}

// Translated from 00ac1600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: first, last, the value's address): assigns the
/// 0x2C-byte value to each element from first to last (four words and the string at +0x10).
pub fn fn_00ac1600(e: &mut Engine, first: u32, last: u32, value: u32) {
    let mut element = first;
    while element != last {
        copy_words(e, element, value, 4);
        assign_string(e, element + 0x10, value + 0x10);
        element = element.wrapping_add(0x2c);
    }
}

// Translated from 00ac1670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: first, last, the value's address): assigns the
/// 0x34-byte value to each element from first to last (the string, then the vector at +0x1C,
/// `00ac27c0`).
pub fn fn_00ac1670(e: &mut Engine, first: u32, last: u32, value: u32) {
    let mut element = first;
    while element != last {
        assign_string(e, element, value);
        fn_00ac27c0(e, Ptr::new(element + 0x1c), Ptr::new(value + 0x1c));
        element = element.wrapping_add(0x34);
    }
}

/// A forward assignment of string elements from first to last to the destination: `assign`
/// for each pair (destination, source); returns the destination's end (`destination + count
/// * bytes`, the count a signed division).
fn assign_forward(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    size: ElementSize,
    assign: impl Fn(&mut Engine, u32, u32),
) -> u32 {
    let bytes = size.bytes();
    let end = size
        .count(last.wrapping_sub(first))
        .wrapping_mul(bytes)
        .wrapping_add(destination);
    let mut to = destination;
    let mut from = first;
    while from != last {
        assign(e, to, from);
        to = to.wrapping_add(bytes);
        from = from.wrapping_add(bytes);
    }
    end
}

/// A backward assignment of string elements from last down to first, ending at the
/// destination: `assign` for each pair; returns `destination - count * bytes`.
fn assign_backward(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    size: ElementSize,
    assign: impl Fn(&mut Engine, u32, u32),
) -> u32 {
    let bytes = size.bytes();
    let start = destination.wrapping_sub(size.count(last.wrapping_sub(first)).wrapping_mul(bytes));
    let mut to = destination;
    let mut from = last;
    while first != from {
        to = to.wrapping_sub(bytes);
        from = from.wrapping_sub(bytes);
        assign(e, to, from);
    }
    start
}

/// The 0x20-byte element's assignment: the word, then the string at +4.
fn assign_20(e: &mut Engine, to: u32, from: u32) {
    let word = e.mem.u32(from);
    e.mem.set_u32(to, word);
    assign_string(e, to + 4, from + 4);
}

/// The 0x2C-byte element's assignment: four words, then the string at +0x10.
fn assign_2c(e: &mut Engine, to: u32, from: u32) {
    copy_words(e, to, from, 4);
    assign_string(e, to + 0x10, from + 0x10);
}

/// The 0x34-byte element's assignment: the string, then the vector at +0x1C (`00ac27c0`).
fn assign_34(e: &mut Engine, to: u32, from: u32) {
    assign_string(e, to, from);
    fn_00ac27c0(e, Ptr::new(to + 0x1c), Ptr::new(from + 0x1c));
}

/// The 0x38-byte element's assignment: the string, the word at +0x1C, then the vector at +0x20
/// (`00ac2ca0`).
fn assign_38(e: &mut Engine, to: u32, from: u32) {
    assign_string(e, to, from);
    let word = e.mem.u32(from + 0x1c);
    e.mem.set_u32(to + 0x1c, word);
    fn_00ac2ca0(e, Ptr::new(to + 0x20), Ptr::new(from + 0x20));
}

// Translated from 00ac1780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words: first, last, destination and three tags it
/// does not read): assigns the 0x20-byte elements forward; returns the destination's end.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac1780(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    assign_forward(
        e,
        first,
        last,
        destination,
        ElementSize::Shift(5),
        assign_20,
    )
}

// Translated from 00ac1810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words): assigns the 0x2C-byte elements forward;
/// returns the destination's end.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac1810(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    assign_forward(
        e,
        first,
        last,
        destination,
        ElementSize::Divide(0x2c),
        assign_2c,
    )
}

// Translated from 00ac18c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words): assigns the 0x34-byte elements forward;
/// returns the destination's end.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac18c0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    assign_forward(
        e,
        first,
        last,
        destination,
        ElementSize::Divide(0x34),
        assign_34,
    )
}

// Translated from 00ac1980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words): assigns the 0x38-byte elements forward;
/// returns the destination's end.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac1980(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    assign_forward(
        e,
        first,
        last,
        destination,
        ElementSize::Divide(0x38),
        assign_38,
    )
}

// Translated from 00ac1a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words: first, last, destination end and three tags it
/// does not read): copies the 12-byte elements backward; returns the block's new start.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac1a50(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    copy_backward(e, first, last, destination, ElementSize::Divide(0xc))
}

// Translated from 00ac1af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words: first, last, destination, allocator): the
/// 12-byte elements' copy through `00ac3370`. Returns the destination's end.
pub fn fn_00ac1af0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00ac3370(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00ac1b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words): the 16-byte elements' copy through
/// `00ac3820`. Returns the destination's end.
pub fn fn_00ac1b70(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00ac3820(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00ac1bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words): copies the 16-byte elements backward;
/// returns the block's new start.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac1bf0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    copy_backward(e, first, last, destination, ElementSize::Shift(4))
}

// Translated from 00ac1c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words): the float pairs' copy through `00ac3900`.
/// Returns the destination's end.
pub fn fn_00ac1c90(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00ac3900(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00ac1d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words): copies the 8-byte elements backward (as
/// words); returns the block's new start.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac1d10(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    copy_backward(e, first, last, destination, ElementSize::Shift(3))
}

// Translated from 00ac1da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words): the 0x20-byte elements' copy construction
/// through `00ac39e0`. Returns the destination's end.
pub fn fn_00ac1da0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00ac39e0(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00ac1e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words): the 0x2C-byte elements' copy construction
/// through `00ac3ac0`. Returns the destination's end.
pub fn fn_00ac1e20(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00ac3ac0(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00ac1ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words: first, last, destination end, a byte tag
/// passed on zero-extended and two words it does not read): the 0x2C-byte elements' backward assignment through `00ac30c0`.
/// Returns its result.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac1ea0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    tag: u8,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    fn_00ac30c0(e, first, last, destination, u32::from(tag), 0, 0)
}

// Translated from 00ac1f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words): the 0x34-byte elements' backward
/// assignment through `00ac3170`. Returns its result.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac1f60(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    tag: u8,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    fn_00ac3170(e, first, last, destination, u32::from(tag), 0, 0)
}

// Translated from 00ac2020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words): the 0x38-byte elements' backward
/// assignment through `00ac3230`. Returns its result.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac2020(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    tag: u8,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    fn_00ac3230(e, first, last, destination, u32::from(tag), 0, 0)
}

// Translated from 00ac2060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words: the first iterator's proxy and pointer, the
/// last iterator's, the destination and the allocator): moves the 4-byte elements between the
/// two pointers to the destination (`memmove_s`, when the count is not 0) and returns the
/// destination's end.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac2060(
    e: &mut Engine,
    _first_proxy: u32,
    first: u32,
    _last_proxy: u32,
    last: u32,
    destination: u32,
    _unused_6: u32,
) -> u32 {
    let count = ((last.wrapping_sub(first) as i32) >> 2) as u32;
    let end = destination.wrapping_add(count.wrapping_mul(4));
    if count != 0 {
        let bytes = count.wrapping_shl(2);
        e.call(MEMMOVE_S, &args![destination, bytes, first, bytes]);
    }
    end
}

// Translated from 00ac21b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac21b0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words: destination, count, the value's address, the
/// allocator and two tags): constructs `count` 16-byte copies of the value (each slot that is
/// not null gets the four words).
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac21b0(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) {
    let mut destination = destination;
    let mut count = count;
    while count > 0 {
        if destination != 0 {
            copy_words(e, destination, value, 4);
        }
        count -= 1;
        destination = destination.wrapping_add(0x10);
    }
}

// Translated from 00ac2370 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac2370 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words): constructs `count` 0x20-byte copies of the
/// value (`00ac34c0`, which skips a null slot).
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac2370(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) {
    let mut destination = destination;
    let mut count = count;
    while count > 0 {
        fn_00ac34c0(e, destination, value);
        count -= 1;
        destination = destination.wrapping_add(0x20);
    }
}

// Translated from 00ac2440 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac2440 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words): copy-constructs (`00ac04c0`) `count`
/// 0x2C-byte elements from the value at the destination, skipping null slots.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac2440(
    e: &mut Engine,
    destination: u32,
    count: u32,
    value: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) {
    let mut destination = destination;
    let mut count = count;
    while count > 0 {
        if destination != 0 {
            fn_00ac04c0(e, Ptr::new(destination), Ptr::new(value));
        }
        count -= 1;
        destination = destination.wrapping_add(0x2c);
    }
}

// Translated from 00ac2760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the 0x38-byte element's `operator=` (`thiscall`, the other): the
/// string, the word at +0x1C, the vector at +0x20 (`00ac2ca0`). Returns `this`.
pub fn fn_00ac2760(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    assign_38(e, this.addr(), other.addr());
    this
}

// Translated from 00ac2c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words: first, last, destination and three tags it
/// does not read): copies the 12-byte elements forward; returns the destination's end.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac2c20(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    copy_forward(e, first, last, destination, ElementSize::Divide(0xc))
}

// Translated from 00ac3030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words: first, last, destination end and three tags it
/// does not read): assigns the 0x20-byte elements backward; returns the block's new start.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac3030(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    assign_backward(
        e,
        first,
        last,
        destination,
        ElementSize::Shift(5),
        assign_20,
    )
}

// Translated from 00ac30c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words): assigns the 0x2C-byte elements backward;
/// returns the block's new start.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac30c0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    assign_backward(
        e,
        first,
        last,
        destination,
        ElementSize::Divide(0x2c),
        assign_2c,
    )
}

// Translated from 00ac3170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words): assigns the 0x34-byte elements backward;
/// returns the block's new start.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac3170(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    assign_backward(
        e,
        first,
        last,
        destination,
        ElementSize::Divide(0x34),
        assign_34,
    )
}

// Translated from 00ac3230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words): assigns the 0x38-byte elements backward;
/// returns the block's new start.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac3230(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    assign_backward(
        e,
        first,
        last,
        destination,
        ElementSize::Divide(0x38),
        assign_38,
    )
}

// Translated from 00ac34c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac34c0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`: destination, source): constructs a 0x20-byte element
/// at a destination that is not null: the word, then the string at +4 made empty and assigned
/// the source's.
pub fn fn_00ac34c0(e: &mut Engine, destination: u32, source: u32) {
    if destination == 0 {
        return;
    }
    let word = e.mem.u32(source);
    e.mem.set_u32(destination, word);
    let string = destination + 4;
    e.call(STRING_TIDY, &args![string, 0u32, 0u32]);
    assign_string(e, string, source + 4);
}

// Translated from 00ac3640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words): the 16-byte elements' copy through
/// `00ac3820`. Returns the destination's end.
pub fn fn_00ac3640(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00ac3820(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00ac36e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words): the 0x20-byte elements' copy construction
/// through `00ac39e0`. Returns the destination's end.
pub fn fn_00ac36e0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00ac39e0(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00ac3730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words): the 0x2C-byte elements' copy construction
/// through `00ac3ac0`. Returns the destination's end.
pub fn fn_00ac3730(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    fn_00ac3ac0(e, first, last, destination, allocator, 0, 0)
}

// Translated from 00ac3820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac3820 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words: first, last, destination, allocator, two
/// tags): copies the 16-byte elements from first to last to the destination, skipping null
/// slots. Returns the destination's end.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac3820(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let mut first = first;
    let mut destination = destination;
    while first != last {
        if destination != 0 {
            copy_words(e, destination, first, 4);
        }
        destination = destination.wrapping_add(0x10);
        first = first.wrapping_add(0x10);
    }
    destination
}

// Translated from 00ac39e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac39e0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words): constructs (`00ac34c0`) the 0x20-byte
/// elements from first to last at the destination. Returns the destination's end.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac39e0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let mut first = first;
    let mut destination = destination;
    while first != last {
        fn_00ac34c0(e, destination, first);
        destination = destination.wrapping_add(0x20);
        first = first.wrapping_add(0x20);
    }
    destination
}

// Translated from 00ac3ac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac3ac0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words): copy-constructs (`00ac04c0`) the 0x2C-byte
/// elements from first to last at the destination, skipping null slots. Returns the
/// destination's end.
#[allow(clippy::too_many_arguments)]
pub fn fn_00ac3ac0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let mut first = first;
    let mut destination = destination;
    while first != last {
        if destination != 0 {
            fn_00ac04c0(e, Ptr::new(destination), Ptr::new(first));
        }
        destination = destination.wrapping_add(0x2c);
        first = first.wrapping_add(0x2c);
    }
    destination
}

// Translated from 00abd2e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00abd2e0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a 16-byte-element vector's `_Insert_n(where, count, value)`, as
/// `00abc320` with the maximum 0xFFFFFFF (`0065f640` throws), the allocation `0064e430`, the
/// copies `00ac3640`, the fills `00ac21b0`, the tail copy `00ac1b70` and the move `00ac14e0`;
/// the value is read as four words, and the reallocation walks the old elements first.
pub fn fn_00abd2e0(e: &mut Engine, this: Ptr, _unused_1: u32, at: u32, count: u32, value: u32) {
    let shape = PlainInsertShape {
        size: ElementSize::Shift(4),
        maximum: 0x0fff_ffff,
        throw_length: THROW_LENGTH_0065F640,
        allocate: |e, count| e.call(0x0064_e430, &args![count, 0u32]).u32(),
        copy: fn_00ac3640,
        fill: |e, vector, destination, count, value| {
            fn_00ac21b0(
                e,
                destination,
                count,
                value,
                vector + VECTOR_ALLOCATOR,
                0,
                0,
            )
        },
        fill_tail: |e, vector, destination, count, value| {
            fn_00ac21b0(
                e,
                destination,
                count,
                value,
                vector + VECTOR_ALLOCATOR,
                0,
                0,
            )
        },
        copy_tail: fn_00ac1b70,
        move_backward: 0x00ac_14e0,
        load: load_words::<4>,
        walks_old: true,
    };
    insert_plain(e, this, at, count, value, &shape);
}

/// The 6-word fills (destination, count, value address, allocator and two tags).
type Fill6Fn = fn(&mut Engine, u32, u32, u32, u32, u32, u32);

/// The parts of the insert-n instances over the 0x20- and 0x2C-byte elements (words and a
/// string) that differ.
struct StringInsertShape {
    size: ElementSize,
    maximum: u32,
    allocate: u32,
    /// The copy construction (first, last, destination, allocator) of the elements before
    /// `where` into a new block and of the moved-up elements; the one of the rest into a new
    /// block; the one past the old end.
    copy: CopyFn,
    copy_rest: CopyFn,
    copy_tail: CopyFn,
    fill: Fill6Fn,
    /// The range destruction of the old elements (`thiscall` on the vector: from, to).
    destroy_range: u32,
    /// The backward assignment (first, last, end).
    move_backward: fn(&mut Engine, u32, u32, u32),
    /// The temporary copy of the value: its construction (temporary, value), the assignment
    /// of a range from it (first, last, temporary) and its destruction.
    make_temp: fn(&mut Engine, u32, u32),
    assign_range: fn(&mut Engine, u32, u32, u32),
    destroy_temp: fn(&mut Engine, u32),
}

/// The insert-n template over the string elements: as [`insert_objects`], but the old
/// elements of a reallocation are destroyed as a range, and the value's temporary is made,
/// assigned from and destroyed by the shape's own steps.
fn insert_strings(
    e: &mut Engine,
    this: Ptr,
    at: u32,
    count: u32,
    value: u32,
    shape: &StringInsertShape,
) {
    let vector = this.addr();
    let size = shape.size;
    let bytes = size.bytes();
    let allocator = vector + VECTOR_ALLOCATOR;
    let capacity = vector_capacity(e, vector, size);
    if count == 0 {
        return;
    }
    let maximum = max_or_one(shape.maximum);
    if maximum.wrapping_sub(vector_size(e, vector, size)) < count {
        e.call(THROW_LENGTH_0065F640, &[]);
        return;
    }
    if capacity < vector_size(e, vector, size).wrapping_add(count) {
        let needed = vector_size(e, vector, size).wrapping_add(count);
        let new_capacity = grown_capacity(capacity, shape.maximum, needed);
        let block = e.call(shape.allocate, &args![new_capacity, 0u32]).u32();
        let first = e.mem.u32(vector + VECTOR_FIRST);
        let mut cursor = (shape.copy)(e, first, at, block, allocator);
        (shape.fill)(e, cursor, count, value, allocator, 0, 0);
        cursor = count.wrapping_mul(bytes).wrapping_add(cursor);
        let last = e.mem.u32(vector + VECTOR_LAST);
        (shape.copy_rest)(e, at, last, cursor, allocator);
        let new_size = vector_size(e, vector, size).wrapping_add(count);
        if e.mem.u32(vector + VECTOR_FIRST) != 0 {
            let first = e.mem.u32(vector + VECTOR_FIRST);
            let last = e.mem.u32(vector + VECTOR_LAST);
            e.call(shape.destroy_range, &args![vector, first, last]);
            let first = e.mem.u32(vector + VECTOR_FIRST);
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, first]);
        }
        e.mem.set_u32(
            vector + VECTOR_END,
            new_capacity.wrapping_mul(bytes).wrapping_add(block),
        );
        e.mem.set_u32(
            vector + VECTOR_LAST,
            new_size.wrapping_mul(bytes).wrapping_add(block),
        );
        e.mem.set_u32(vector + VECTOR_FIRST, block);
        return;
    }
    let after = size.count(e.mem.u32(vector + VECTOR_LAST).wrapping_sub(at));
    e.with_stack(bytes, |e, temporary| {
        let temporary = temporary.addr();
        (shape.make_temp)(e, temporary, value);
        if after < count {
            let last = e.mem.u32(vector + VECTOR_LAST);
            (shape.copy)(
                e,
                at,
                last,
                count.wrapping_mul(bytes).wrapping_add(at),
                allocator,
            );
            let last = e.mem.u32(vector + VECTOR_LAST);
            let missing = count.wrapping_sub(size.count(last.wrapping_sub(at)));
            (shape.fill)(e, last, missing, temporary, allocator, 0, 0);
            let last = count
                .wrapping_mul(bytes)
                .wrapping_add(e.mem.u32(vector + VECTOR_LAST));
            e.mem.set_u32(vector + VECTOR_LAST, last);
            let end = last.wrapping_sub(count.wrapping_mul(bytes));
            (shape.assign_range)(e, at, end, temporary);
        } else {
            let old_last = e.mem.u32(vector + VECTOR_LAST);
            let last = e.mem.u32(vector + VECTOR_LAST);
            let tail = old_last.wrapping_sub(count.wrapping_mul(bytes));
            let new_last = (shape.copy_tail)(e, tail, old_last, last, allocator);
            e.mem.set_u32(vector + VECTOR_LAST, new_last);
            (shape.move_backward)(e, at, tail, old_last);
            let end = count.wrapping_mul(bytes).wrapping_add(at);
            (shape.assign_range)(e, at, end, temporary);
        }
        (shape.destroy_temp)(e, temporary);
    });
}

// Translated from 00abe310 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00abe310 - the compiler's exception frame (FS:[0] chain) and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map: a 0x20-byte-element (a word and a string) vector's
/// `_Insert_n(where, count, value)` (`thiscall`, four words), the string template with the
/// maximum 0x7FFFFFF (`0065f640` throws), the allocation `0065e430`, the copies `00ac36e0`
/// (into new, up and past the end) and `00ac1da0` (the rest), the fill `00ac2370`, the range
/// destruction `0065dc60` and the backward assignment `00ac1580`; the temporary is the value's
/// word and a copy of its string (made empty, assigned), the range assignment an inline loop
/// (the word, then the string's `assign`), and the temporary's string is destroyed
/// (`0044a730(1, 0)`).
pub fn fn_00abe310(e: &mut Engine, this: Ptr, _unused_1: u32, at: u32, count: u32, value: u32) {
    let shape = StringInsertShape {
        size: ElementSize::Shift(5),
        maximum: 0x07ff_ffff,
        allocate: 0x0065_e430,
        copy: fn_00ac36e0,
        copy_rest: fn_00ac1da0,
        copy_tail: fn_00ac36e0,
        fill: fn_00ac2370,
        destroy_range: DESTROY_RANGE_20,
        move_backward: |e, first, last, end| {
            fn_00ac1580(e, first, last, end);
        },
        make_temp: |e, temporary, value| {
            let word = e.mem.u32(value);
            e.mem.set_u32(temporary, word);
            e.call(STRING_TIDY, &args![temporary + 4, 0u32, 0u32]);
            assign_string(e, temporary + 4, value + 4);
        },
        assign_range: |e, first, last, temporary| {
            let mut element = first;
            while element != last {
                let word = e.mem.u32(temporary);
                e.mem.set_u32(element, word);
                assign_string(e, element + 4, temporary + 4);
                element = element.wrapping_add(0x20);
            }
        },
        destroy_temp: |e, temporary| {
            e.call(STRING_TIDY, &args![temporary + 4, 1u32, 0u32]);
        },
    };
    insert_strings(e, this, at, count, value, &shape);
}

// Translated from 00abeb00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00abeb00 - the compiler's exception frame (FS:[0] chain) and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map: a 0x2C-byte-element (four words and a string) vector's
/// `_Insert_n(where, count, value)`, the string template with the maximum 0x5D1745D, the
/// allocation `0065e4c0`, the copies `00ac3730` (into new and up) and `00ac1e20` (the rest and
/// past the end), the fill `00ac2440`, the range destruction `0065dd90` and the backward
/// assignment `00ac1ea0` (given tag words the code copies from uninitialized stack bytes; the
/// callees do not read them); the temporary is copy-constructed (`00ac04c0`), the range
/// assigned from it with `00ac1600`, and its string (+0x10) destroyed (`0044a730(1, 0)`).
pub fn fn_00abeb00(e: &mut Engine, this: Ptr, _unused_1: u32, at: u32, count: u32, value: u32) {
    let shape = StringInsertShape {
        size: ElementSize::Divide(0x2c),
        maximum: 0x05d1_745d,
        allocate: 0x0065_e4c0,
        copy: fn_00ac3730,
        copy_rest: fn_00ac1e20,
        copy_tail: fn_00ac1e20,
        fill: fn_00ac2440,
        destroy_range: DESTROY_RANGE_2C,
        move_backward: |e, first, last, end| {
            fn_00ac1ea0(e, first, last, end, 0, 0, 0);
        },
        make_temp: |e, temporary, value| {
            fn_00ac04c0(e, Ptr::new(temporary), Ptr::new(value));
        },
        assign_range: fn_00ac1600,
        destroy_temp: |e, temporary| {
            e.call(STRING_TIDY, &args![temporary + 0x10, 1u32, 0u32]);
        },
    };
    insert_strings(e, this, at, count, value, &shape);
}

/// A 12-byte-element vector's clear (`00ab9cd0`, `thiscall`) and buy (`0065cfd0`, `thiscall`:
/// the count; answers `AL`).
const CLEAR_12: u32 = 0x00ab_9cd0;
const BUY_12: u32 = 0x0065_cfd0;

// Translated from 00ac27c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 12-byte-element vector's `operator=` (`thiscall`, the other).
/// Nothing for itself; an empty other clears it (`00ab9cd0`). When the other is not larger,
/// its elements are copied over the first ones (`00ac2c20`, an empty walk over the rest) and
/// the last pointer set; when it fits the capacity, the first ones are copied (`00ac1170`) and
/// the rest constructed after them (`00ac3370`); otherwise the old block is freed (after an
/// empty walk), room bought (`0065cfd0`) and, when that succeeds, the elements constructed
/// (`00ac3370`). Returns `this`.
pub fn fn_00ac27c0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let vector = this.addr();
    let source = other.addr();
    let size = ElementSize::Divide(0xc);
    if vector == source {
        return this;
    }
    if vector_size(e, source, size) == 0 {
        e.call(CLEAR_12, &args![vector]);
        return this;
    }
    let wanted = vector_size(e, source, size);
    if wanted <= vector_size(e, vector, size) {
        let first = e.mem.u32(vector + VECTOR_FIRST);
        let source_last = e.mem.u32(source + VECTOR_LAST);
        let source_first = e.mem.u32(source + VECTOR_FIRST);
        let moved = fn_00ac2c20(e, source_first, source_last, first, 0, 0, 0);
        let end = e.mem.u32(vector + VECTOR_LAST);
        let mut element = moved;
        while element != end {
            element = element.wrapping_add(0xc);
        }
        let last = vector_size(e, source, size)
            .wrapping_mul(0xc)
            .wrapping_add(e.mem.u32(vector + VECTOR_FIRST));
        e.mem.set_u32(vector + VECTOR_LAST, last);
        return this;
    }
    let wanted = vector_size(e, source, size);
    if wanted <= vector_capacity(e, vector, size) {
        let middle = vector_size(e, vector, size)
            .wrapping_mul(0xc)
            .wrapping_add(e.mem.u32(source + VECTOR_FIRST));
        let first = e.mem.u32(vector + VECTOR_FIRST);
        let source_first = e.mem.u32(source + VECTOR_FIRST);
        fn_00ac1170(e, source_first, middle, first);
        let last = e.mem.u32(vector + VECTOR_LAST);
        let source_last = e.mem.u32(source + VECTOR_LAST);
        let end = fn_00ac3370(
            e,
            middle,
            source_last,
            last,
            vector + VECTOR_ALLOCATOR,
            0,
            0,
        );
        e.mem.set_u32(vector + VECTOR_LAST, end);
        return this;
    }
    if e.mem.u32(vector + VECTOR_FIRST) != 0 {
        let last = e.mem.u32(vector + VECTOR_LAST);
        let mut element = e.mem.u32(vector + VECTOR_FIRST);
        while element != last {
            element = element.wrapping_add(0xc);
        }
        let first = e.mem.u32(vector + VECTOR_FIRST);
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, first]);
    }
    let wanted = vector_size(e, source, size);
    if e.call(BUY_12, &args![vector, wanted]).u8() == 0 {
        return this;
    }
    let first = e.mem.u32(vector + VECTOR_FIRST);
    let source_last = e.mem.u32(source + VECTOR_LAST);
    let source_first = e.mem.u32(source + VECTOR_FIRST);
    let end = fn_00ac3370(
        e,
        source_first,
        source_last,
        first,
        vector + VECTOR_ALLOCATOR,
        0,
        0,
    );
    e.mem.set_u32(vector + VECTOR_LAST, end);
    this
}

// Translated from 00ac2ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: a 4-byte-element vector's `operator=` (`thiscall`, the other), as
/// `00ac27c0` with the clear `00ac2f70`, the moves `00ac13c0` (over the first elements) and
/// `00ac35d0` (after them), the buy `00abc260`, and no walks. Returns `this`.
pub fn fn_00ac2ca0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let vector = this.addr();
    let source = other.addr();
    let size = ElementSize::Shift(2);
    if vector == source {
        return this;
    }
    if vector_size(e, source, size) == 0 {
        fn_00ac2f70(e, this);
        return this;
    }
    let wanted = vector_size(e, source, size);
    if wanted <= vector_size(e, vector, size) {
        let first = e.mem.u32(vector + VECTOR_FIRST);
        let source_last = e.mem.u32(source + VECTOR_LAST);
        let source_first = e.mem.u32(source + VECTOR_FIRST);
        fn_00ac13c0(e, source_first, source_last, first);
        let last = vector_size(e, source, size)
            .wrapping_mul(4)
            .wrapping_add(e.mem.u32(vector + VECTOR_FIRST));
        e.mem.set_u32(vector + VECTOR_LAST, last);
        return this;
    }
    let wanted = vector_size(e, source, size);
    if wanted <= vector_capacity(e, vector, size) {
        let middle = vector_size(e, vector, size)
            .wrapping_mul(4)
            .wrapping_add(e.mem.u32(source + VECTOR_FIRST));
        let first = e.mem.u32(vector + VECTOR_FIRST);
        let source_first = e.mem.u32(source + VECTOR_FIRST);
        fn_00ac13c0(e, source_first, middle, first);
        let last = e.mem.u32(vector + VECTOR_LAST);
        let source_last = e.mem.u32(source + VECTOR_LAST);
        let end = fn_00ac35d0(e, middle, source_last, last, vector + VECTOR_ALLOCATOR);
        e.mem.set_u32(vector + VECTOR_LAST, end);
        return this;
    }
    if e.mem.u32(vector + VECTOR_FIRST) != 0 {
        let first = e.mem.u32(vector + VECTOR_FIRST);
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, first]);
    }
    let wanted = vector_size(e, source, size);
    if !fn_00abc260(e, this, wanted) {
        return this;
    }
    let first = e.mem.u32(vector + VECTOR_FIRST);
    let source_last = e.mem.u32(source + VECTOR_LAST);
    let source_first = e.mem.u32(source + VECTOR_FIRST);
    let end = fn_00ac35d0(
        e,
        source_first,
        source_last,
        first,
        vector + VECTOR_ALLOCATOR,
    );
    e.mem.set_u32(vector + VECTOR_LAST, end);
    this
}

// Translated from 00ac3f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets a `FutBinaryFileC`'s file id (`thiscall`, the address of a
/// string): asserts the id's length is 8 (`fr2Assert(binaryFile.cpp, 0x30)`) and that no file
/// is open (+0x3C, line 0x31), assigns the id (+0x20), and sets its character 5 (the version)
/// to `'0'` after the library's length check.
pub fn fn_00ac3f40(e: &mut Engine, this: Ptr, file_id: u32) {
    let base = this.addr();
    if e.mem.u32(file_id + STRING_LENGTH) != 8 {
        e.call(FR2_ASSERT, &args![BINARY_FILE_SOURCE, 0x30u32]);
    }
    if e.mem.u32(base + 0x3c) != 0 {
        e.call(FR2_ASSERT, &args![BINARY_FILE_SOURCE, 0x31u32]);
    }
    assign_string(e, base + 0x20, file_id);
    check(e, e.mem.u32(base + 0x20 + STRING_LENGTH) >= 5);
    let chars = string_chars(e, base + 0x20);
    e.mem.set_u8(chars.wrapping_add(5), b'0');
}

/// `fopen` (`00ec9a47`, cdecl: name, mode), `"rb"`, `fread` (`00ecb3a8`, cdecl: buffer, size,
/// count, file), `fclose` (`00ec9907`, cdecl), `strlen` (`00ec6130`, cdecl).
const FOPEN: u32 = 0x00ec_9a47;
const READ_BINARY_MODE: u32 = 0x0106_e79c;
const FREAD: u32 = 0x00ec_b3a8;
const FCLOSE: u32 = 0x00ec_9907;
const STRLEN: u32 = 0x00ec_6130;
/// The byte vector's construct-with-size (`00aba0d0`, `thiscall`: the count), `futSetLastError`
/// (`00ac6430`, cdecl: the code and a string by value) and the string compare (`00ac6100`,
/// `thiscall` on a string: position, count, text, text count; 0 when equal).
const BYTE_VECTOR_CONSTRUCT: u32 = 0x00ab_a0d0;
const SET_LAST_ERROR: u32 = 0x00ac_6430;
const STRING_COMPARE: u32 = 0x00ac_6100;

// Translated from 00ac3fe0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ac3fe0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: whether the file named by a string (`thiscall`, its address)
/// starts with this `FutBinaryFileC`'s id. Opens it (`fopen` `"rb"`; failing: `futSetLastError`
/// 4 with a copy of the name, false), reads the id's length of bytes (+0x34) into a byte vector
/// one longer (`00aba0d0`; a short read: closes, `futSetLastError` 1, frees the vector, false),
/// ends them with 0, closes the file, copies the id's version character (5) over the read one
/// and compares the id with them (`00ac6100`, the read text's `strlen`): true when equal. The
/// vector is freed on every path (its tidy `00abc0a0`, then its proxy through the memory
/// manager), and the file closed again on each path when still open (never, as the code clears
/// the pointer after closing). The library's checks call `_invalid_parameter_noinfo`.
pub fn fn_00ac3fe0(e: &mut Engine, this: Ptr, name: u32) -> bool {
    let base = this.addr();
    let path = string_chars(e, name);
    let mut file = e.call(FOPEN, &args![path, READ_BINARY_MODE]).u32();
    let close_if_open = |e: &mut Engine, file: u32| {
        if file != 0 {
            e.call(FCLOSE, &args![file]);
        }
    };
    if file == 0 {
        let copy = crate::units::facegen::binaryfile::string_copy_arg(e, name);
        e.call(SET_LAST_ERROR, &args![4u32, copy]);
        close_if_open(e, file);
        return false;
    }
    e.with_stack(0x18, |e, buffer| {
        let buffer = buffer.addr();
        let length = e.mem.u32(base + 0x34);
        e.call(
            BYTE_VECTOR_CONSTRUCT,
            &args![buffer, length.wrapping_add(1)],
        );
        let data_size = |e: &Engine| {
            e.mem
                .u32(buffer + VECTOR_LAST)
                .wrapping_sub(e.mem.u32(buffer + VECTOR_FIRST))
        };
        let length = e.mem.u32(base + 0x34);
        check(e, data_size(e) != 0);
        let data = e.mem.u32(buffer + VECTOR_FIRST);
        let read = e.call(FREAD, &args![data, length, 1u32, file]).u32();
        if read != 1 {
            close_if_open(e, file);
            file = 0;
            let copy = crate::units::facegen::binaryfile::string_copy_arg(e, name);
            e.call(SET_LAST_ERROR, &args![1u32, copy]);
            fn_00abc0a0(e, Ptr::new(buffer));
            fn_00abc120(e, Ptr::new(buffer));
            close_if_open(e, file);
            return false;
        }
        let length = e.mem.u32(base + 0x34);
        check(e, length < data_size(e));
        let data = e.mem.u32(buffer + VECTOR_FIRST);
        e.mem.set_u8(data.wrapping_add(length), 0);
        close_if_open(e, file);
        file = 0;
        let id = base + 0x20;
        check(e, e.mem.u32(id + STRING_LENGTH) >= 5);
        let chars = string_chars(e, id);
        check(e, data_size(e) > 5);
        let data = e.mem.u32(buffer + VECTOR_FIRST);
        let version = e.mem.u8(chars.wrapping_add(5));
        e.mem.set_u8(data.wrapping_add(5), version);
        check(e, data_size(e) != 0);
        let text = e.mem.u32(buffer + VECTOR_FIRST);
        let text_length = e.call(STRLEN, &args![text]).u32();
        let id_length = e.mem.u32(id + STRING_LENGTH);
        let compared = e
            .call(
                STRING_COMPARE,
                &args![id, 0u32, id_length, text, text_length],
            )
            .u32();
        let same = compared == 0;
        fn_00abc0a0(e, Ptr::new(buffer));
        if same {
            let proxy = e.mem.u32(buffer + VECTOR_PROXY);
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, proxy]);
        } else {
            fn_00abc120(e, Ptr::new(buffer));
        }
        close_if_open(e, file);
        same
    })
}

/// Several words passed by value (a vector element), in memory order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WordsArg<const N: usize>(pub [u32; N]);

impl<const N: usize> Arg for WordsArg<N> {
    const WORDS: usize = N;
    fn take(words: &[u32], i: &mut usize) -> Self {
        let mut out = [0u32; N];
        for word in out.iter_mut() {
            *word = u32::take(words, i);
        }
        WordsArg(out)
    }
    fn put(self, out: &mut Vec<u32>) {
        out.extend_from_slice(&self.0);
    }
}

/// The entries of the by-value resizes: their value words as a [`WordsArg`].
fn resize_16(e: &mut Engine, this: Ptr, count: u32, value: WordsArg<4>) {
    fn_00abaaa0(e, this, count, value.0)
}
fn resize_32(e: &mut Engine, this: Ptr, count: u32, value: WordsArg<8>) {
    fn_00abb060(e, this, count, value.0)
}
fn resize_44(e: &mut Engine, this: Ptr, count: u32, value: WordsArg<11>) {
    fn_00abb3c0(e, this, count, value.0)
}
/// `00abb700` takes the address of its value: the words are laid out in an area that stands for
/// its argument area (`[EBP+0xC]`). Its two callers (`00ab46f0`, `00ab5ec0`) build the element
/// in that area themselves and call [`fn_00abb700`] with it.
fn resize_52(e: &mut Engine, this: Ptr, count: u32, value: WordsArg<13>) {
    with_value_slot(e, &value.0, |e, slot| {
        fn_00abb700(e, this, count, Ptr::new(slot))
    })
}
fn resize_56(e: &mut Engine, this: Ptr, count: u32, value: WordsArg<14>) {
    fn_00abbae0(e, this, count, value.0)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ab_a3c0, fn_00aba3c0(Ptr, Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00ab_a9a0, fn_00aba9a0(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ab_ac80, fn_00abac80(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ab_af60, fn_00abaf60(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ab_b280, fn_00abb280(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ab_b5f0, fn_00abb5f0(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ab_b930, fn_00abb930(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ab_bd10, fn_00abbd10(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ab_beb0, fn_00abbeb0(Ptr, u32, u32) -> Ptr),
        entry!(0x00ab_bf00, fn_00abbf00(Ptr, Ptr) -> bool),
        entry!(0x00ab_bf50, fn_00abbf50(Ptr) -> u32),
        entry!(0x00ab_d2e0, fn_00abd2e0(Ptr, u32, u32, u32, u32)),
        entry!(0x00ab_e310, fn_00abe310(Ptr, u32, u32, u32, u32)),
        entry!(0x00ab_eb00, fn_00abeb00(Ptr, u32, u32, u32, u32)),
        entry!(0x00ac_04c0, fn_00ac04c0(Ptr, Ptr) -> Ptr),
        entry!(0x00ac_0ab0, fn_00ac0ab0(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_0b00, fn_00ac0b00()),
        entry!(0x00ac_0b80, fn_00ac0b80(Ptr, u32) -> Ptr),
        entry!(0x00ac_0c40, fn_00ac0c40(Ptr, u32) -> Ptr),
        entry!(0x00ac_0d00, fn_00ac0d00(Ptr, u32) -> Ptr),
        entry!(0x00ac_0dc0, fn_00ac0dc0(Ptr, u32) -> Ptr),
        entry!(0x00ac_0e80, fn_00ac0e80(Ptr, u32) -> Ptr),
        entry!(0x00ac_0f40, fn_00ac0f40(Ptr, u32) -> Ptr),
        entry!(0x00ac_10c0, fn_00ac10c0(Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x00ac_1120, fn_00ac1120(Ptr, Ptr, f32) -> Ptr),
        entry!(0x00ac_1170, fn_00ac1170(u32, u32, u32) -> u32),
        entry!(0x00ac_1220, fn_00ac1220(u32, u32, u32) -> u32),
        entry!(0x00ac_12d0, fn_00ac12d0(u32, u32, u32) -> u32),
        entry!(0x00ac_1370, fn_00ac1370(u32, u32, u32) -> u32),
        entry!(0x00ac_1440, fn_00ac1440(u32, u32, u32) -> u32),
        entry!(0x00ac_14e0, fn_00ac14e0(u32, u32, u32) -> u32),
        entry!(0x00ac_1530, fn_00ac1530(u32, u32, u32) -> u32),
        entry!(0x00ac_1580, fn_00ac1580(u32, u32, u32) -> u32),
        entry!(0x00ac_1600, fn_00ac1600(u32, u32, u32)),
        entry!(0x00ac_1670, fn_00ac1670(u32, u32, u32)),
        entry!(0x00ac_1780, fn_00ac1780(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_1810, fn_00ac1810(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_18c0, fn_00ac18c0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_1980, fn_00ac1980(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_1a50, fn_00ac1a50(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_1af0, fn_00ac1af0(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_1b70, fn_00ac1b70(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_1bf0, fn_00ac1bf0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_1c90, fn_00ac1c90(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_1d10, fn_00ac1d10(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_1da0, fn_00ac1da0(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_1e20, fn_00ac1e20(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_1ea0, fn_00ac1ea0(u32, u32, u32, u8, u32, u32) -> u32),
        entry!(0x00ac_1f60, fn_00ac1f60(u32, u32, u32, u8, u32, u32) -> u32),
        entry!(0x00ac_2020, fn_00ac2020(u32, u32, u32, u8, u32, u32) -> u32),
        entry!(0x00ac_2060, fn_00ac2060(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_21b0, fn_00ac21b0(u32, u32, u32, u32, u32, u32)),
        entry!(0x00ac_2370, fn_00ac2370(u32, u32, u32, u32, u32, u32)),
        entry!(0x00ac_2440, fn_00ac2440(u32, u32, u32, u32, u32, u32)),
        entry!(0x00ac_2760, fn_00ac2760(Ptr, Ptr) -> Ptr),
        entry!(0x00ac_27c0, fn_00ac27c0(Ptr, Ptr) -> Ptr),
        entry!(0x00ac_2c20, fn_00ac2c20(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_2ca0, fn_00ac2ca0(Ptr, Ptr) -> Ptr),
        entry!(0x00ac_3030, fn_00ac3030(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_30c0, fn_00ac30c0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_3170, fn_00ac3170(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_3230, fn_00ac3230(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_34c0, fn_00ac34c0(u32, u32)),
        entry!(0x00ac_3640, fn_00ac3640(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_36e0, fn_00ac36e0(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_3730, fn_00ac3730(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_3820, fn_00ac3820(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_39e0, fn_00ac39e0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_3ac0, fn_00ac3ac0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_3f40, fn_00ac3f40(Ptr, u32)),
        entry!(0x00ac_3fe0, fn_00ac3fe0(Ptr, u32) -> bool),
        entry!(0x00ab_a4f0, fn_00aba4f0(Ptr) -> Ptr),
        entry!(0x00ab_a5d0, fn_00aba5d0(Ptr, u32, u32)),
        entry!(0x00ab_a7b0, fn_00aba7b0(Ptr, u32, u32, u32, u32)),
        entry!(0x00ab_aaa0, resize_16(Ptr, u32, WordsArg<4>)),
        entry!(0x00ab_ad80, fn_00abad80(Ptr, u32, u32, u32)),
        entry!(0x00ab_b060, resize_32(Ptr, u32, WordsArg<8>)),
        entry!(0x00ab_b3c0, resize_44(Ptr, u32, WordsArg<11>)),
        entry!(0x00ab_b700, resize_52(Ptr, u32, WordsArg<13>)),
        entry!(0x00ab_bae0, resize_56(Ptr, u32, WordsArg<14>)),
        entry!(0x00ab_bfc0, fn_00abbfc0(Ptr, u32, u32)),
        entry!(0x00ab_c0a0, fn_00abc0a0(Ptr)),
        entry!(0x00ab_c120, fn_00abc120(Ptr)),
        entry!(0x00ab_c180, fn_00abc180(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x00ab_c260, fn_00abc260(Ptr, u32) -> bool),
        entry!(0x00ab_c320, fn_00abc320(Ptr, u32, u32, u32, u32)),
        entry!(0x00ab_ca90, fn_00abca90(Ptr, u32, u32, u32, u32)),
        entry!(0x00ab_db10, fn_00abdb10(Ptr, u32, u32, u32, u32)),
        entry!(0x00ab_f210, fn_00abf210(Ptr, u32, u32, u32, u32)),
        entry!(0x00ab_fa90, fn_00abfa90(Ptr, u32, u32, u32, u32)),
        entry!(0x00ac_03a0, fn_00ac03a0(Ptr, u32) -> bool),
        entry!(0x00ac_0460, fn_00ac0460(Ptr, Ptr) -> u32),
        entry!(0x00ac_0530, fn_00ac0530(Ptr, Ptr) -> Ptr),
        entry!(0x00ac_05b0, fn_00ac05b0(Ptr, Ptr) -> Ptr),
        entry!(0x00ac_0640, fn_00ac0640(Ptr, Ptr) -> Ptr),
        entry!(0x00ac_08b0, fn_00ac08b0(Ptr, Ptr) -> Ptr),
        entry!(0x00ac_1000, fn_00ac1000(Ptr, u32) -> Ptr),
        entry!(0x00ac_13c0, fn_00ac13c0(u32, u32, u32) -> u32),
        entry!(0x00ac_16e0, fn_00ac16e0(u32, u32, u32, u32)),
        entry!(0x00ac_1720, fn_00ac1720(u32, u32) -> u32),
        entry!(0x00ac_1ee0, fn_00ac1ee0(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_1fa0, fn_00ac1fa0(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_20d0, fn_00ac20d0(u32, u32, u32, u32, u32, u32)),
        entry!(0x00ac_2290, fn_00ac2290(u32, u32, u32, u32, u32, u32)),
        entry!(0x00ac_2540, fn_00ac2540(u32, u32, u32, u32, u32, u32)),
        entry!(0x00ac_2650, fn_00ac2650(u32, u32, u32, u32, u32, u32)),
        entry!(0x00ac_2f70, fn_00ac2f70(Ptr)),
        entry!(0x00ac_3300, fn_00ac3300(u32, u32, u32)),
        entry!(0x00ac_3370, fn_00ac3370(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_3450, fn_00ac3450(u32, u32, u32)),
        entry!(0x00ac_3580, fn_00ac3580(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_35d0, fn_00ac35d0(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_3690, fn_00ac3690(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_3780, fn_00ac3780(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_37d0, fn_00ac37d0(u32, u32, u32, u32) -> u32),
        entry!(0x00ac_3900, fn_00ac3900(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_3bd0, fn_00ac3bd0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_3ce0, fn_00ac3ce0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00ac_3df0, fn_00ac3df0(Ptr, StringArg) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls_to(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// The allocators, `memmove_s` (a real move) and quiet checks and throws.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global(STRING_NPOS, u32::MAX);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            e.mem.alloc(a[1].max(4)).into_ret()
        });
        e.register(ALLOCATE_BYTES, |e, a| e.mem.alloc(a[0].max(4)).into_ret());
        e.register(ALLOCATE_PROXY, |e, _| e.mem.alloc(8).into_ret());
        e.register(MEMMOVE_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                MEMORY_MANAGER_FREE,
                INVALID_PARAMETER,
                THROW_LENGTH_00AC0B00,
                THROW_LENGTH_0065F640,
                EXCEPTION_CONSTRUCT,
                CXX_THROW_EXCEPTION,
            ],
        );
        e
    }

    /// A vector of words at a fresh 0x18-byte object, with room for `capacity`.
    fn word_vector(e: &mut Engine, words: &[u32], capacity: u32) -> Ptr {
        let vector = e.mem.alloc(0x18);
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(proxy, vector);
        e.mem.set_u32(vector + VECTOR_PROXY, proxy);
        let block = e.mem.alloc(4 * capacity.max(1));
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *w);
        }
        e.mem.set_u32(vector + VECTOR_FIRST, block);
        e.mem
            .set_u32(vector + VECTOR_LAST, block + 4 * words.len() as u32);
        e.mem.set_u32(vector + VECTOR_END, block + 4 * capacity);
        Ptr::new(vector)
    }

    fn words_of(e: &Engine, vector: Ptr) -> Vec<u32> {
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        let last = e.mem.u32(vector.addr() + VECTOR_LAST);
        (first..last).step_by(4).map(|a| e.mem.u32(a)).collect()
    }

    #[test]
    fn default_vector_links_its_proxy_and_buys_nothing() {
        let mut e = engine();
        let vector = e.mem.alloc(0x18);
        e.mem.set_u32(vector + VECTOR_FIRST, 7);
        assert_eq!(e.call(0x00ab_a4f0, &args![vector]).u32(), vector);
        let proxy = e.mem.u32(vector);
        assert_eq!(e.mem.u32(proxy), vector);
        assert_eq!(e.mem.u32(vector + VECTOR_FIRST), 0);
    }

    #[test]
    fn word_buy_allocates_or_throws() {
        let mut e = engine();
        let vector = e.mem.alloc(0x18);
        assert!(!e.call(0x00ab_c260, &args![vector, 0u32]).bool());
        assert!(e.call(0x00ab_c260, &args![vector, 3u32]).bool());
        let first = e.mem.u32(vector + VECTOR_FIRST);
        assert_ne!(first, 0);
        assert_eq!(e.mem.u32(vector + VECTOR_LAST), first);
        assert_eq!(e.mem.u32(vector + VECTOR_END), first + 12);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ab_c260, &args![vector, 0x4000_0000u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, THROW_LENGTH_00AC0B00).len(), 1);
        assert_eq!(e.mem.u32(vector + VECTOR_FIRST), 0);
    }

    #[test]
    fn byte_buy_allocates() {
        let mut e = engine();
        let vector = e.mem.alloc(0x18);
        assert!(!e.call(0x00ac_03a0, &args![vector, 0u32]).bool());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ac_03a0, &args![vector, 5u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, ALLOCATE_BYTES), vec![vec![5, 0]]);
        let first = e.mem.u32(vector + VECTOR_FIRST);
        assert_eq!(e.mem.u32(vector + VECTOR_END), first + 5);
    }

    #[test]
    fn construct_n_fills_the_bytes() {
        let mut e = engine();
        let vector = e.mem.alloc(0x18);
        let value = e.mem.alloc(4);
        e.mem.set_u8(value, 0x5a);
        e.call(0x00ab_bfc0, &args![vector, 3u32, value]);
        let first = e.mem.u32(vector + VECTOR_FIRST);
        assert_eq!(e.mem.u32(vector + VECTOR_LAST), first + 3);
        assert_eq!(e.mem.bytes(first, 3), vec![0x5a; 3]);
        // The fill helpers on their own.
        let block = e.mem.alloc(8);
        e.call(0x00ac_16e0, &args![block, 2u32, value, 0u32]);
        e.call(0x00ac_3300, &args![block + 2, 1u32, value]);
        assert_eq!(e.mem.bytes(block, 4), vec![0x5a, 0x5a, 0x5a, 0]);
    }

    #[test]
    fn tidy_frees_elements_and_proxy() {
        let mut e = engine();
        let vector = word_vector(&mut e, &[1, 2], 4);
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        let proxy = e.mem.u32(vector.addr());
        e.call_log = Some(vec![]);
        e.call(0x00ab_c0a0, &args![vector]);
        e.call(0x00ab_c120, &args![vector]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MEMORY_MANAGER_FREE),
            vec![
                vec![MEMORY_MANAGER_OBJECT, first],
                vec![MEMORY_MANAGER_OBJECT, proxy]
            ]
        );
        assert_eq!(e.mem.u32(vector.addr() + VECTOR_FIRST), 0);
        assert_eq!(e.mem.u32(vector.addr() + VECTOR_END), 0);
        // An empty vector frees no elements.
        e.call_log = Some(vec![]);
        e.call(0x00ab_c0a0, &args![vector]);
        assert!(calls_to(&e.call_log.take().unwrap(), MEMORY_MANAGER_FREE).is_empty());
    }

    #[test]
    fn iterator_compare_and_advance_check_their_container() {
        let mut e = engine();
        let vector = word_vector(&mut e, &[1, 2, 3], 3);
        let proxy = e.mem.u32(vector.addr());
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        let a = e.mem.alloc(8);
        let b = e.mem.alloc(8);
        e.mem.set_u32(a, proxy);
        e.mem.set_u32(a + 4, first);
        e.mem.set_u32(b, proxy);
        e.mem.set_u32(b + 4, first + 4);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ac_0460, &args![a, b]).u32(), 1);
        e.mem.set_u32(b + 4, first);
        assert_eq!(e.call(0x00ac_0460, &args![a, b]).u32(), 0);
        assert!(calls_to(&e.call_log.take().unwrap(), INVALID_PARAMETER).is_empty());
        e.mem.set_u32(b, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x00ac_0460, &args![a, b]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), INVALID_PARAMETER).len(),
            1
        );

        // Advance: within the vector, then past its end.
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ac_1000, &args![a, 3u32]).u32(), a);
        assert_eq!(e.mem.u32(a + 4), first + 12);
        assert!(calls_to(&e.call_log.take().unwrap(), INVALID_PARAMETER).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x00ac_1000, &args![a, 1u32]);
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), INVALID_PARAMETER).len(),
            1
        );
        assert_eq!(e.mem.u32(a + 4), first + 16);
    }

    #[test]
    fn word_moves_return_the_end() {
        let mut e = engine();
        let source = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_u32(source + 4 * i, 10 + i);
        }
        let destination = e.mem.alloc(16);
        assert_eq!(
            e.call(0x00ac_13c0, &args![source, source + 8, destination])
                .u32(),
            destination + 8
        );
        assert_eq!(e.mem.u32(destination + 4), 11);
        assert_eq!(
            e.call(
                0x00ac_35d0,
                &args![source + 8, source + 16, destination, 0u32]
            )
            .u32(),
            destination + 8
        );
        assert_eq!(e.mem.u32(destination), 12);
        // Empty ranges move nothing.
        e.call_log = Some(vec![]);
        e.call(0x00ac_13c0, &args![source, source, destination]);
        e.call(0x00ac_35d0, &args![source, source, destination, 0u32]);
        assert!(calls_to(&e.call_log.take().unwrap(), MEMMOVE_S).is_empty());
    }

    #[test]
    fn word_allocation_sizes_and_overflow() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x00ac_1720, &args![0u32, 0u32]);
        e.call(0x00ac_1720, &args![3u32, 0u32]);
        e.call(0x00ac_1720, &args![0x4000_0000u32, 0u32]);
        let log = e.call_log.take().unwrap();
        let sizes: Vec<u32> = calls_to(&log, MEMORY_MANAGER_ALLOCATE)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(sizes, vec![0, 12, 0]);
        let throw = calls_to(&log, CXX_THROW_EXCEPTION);
        assert_eq!(throw.len(), 1);
        assert_eq!(throw[0][1], BAD_ALLOC_THROW_INFO);
        assert_eq!(calls_to(&log, EXCEPTION_CONSTRUCT)[0][0], throw[0][0]);
    }

    #[test]
    fn erase_and_clear_move_the_tail() {
        let mut e = engine();
        let vector = word_vector(&mut e, &[1, 2, 3, 4], 4);
        let proxy = e.mem.u32(vector.addr());
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        let result = e.mem.alloc(8);
        let back = e
            .call(
                0x00ab_c180,
                &args![vector, result, 0u32, first + 4, 0u32, first + 12],
            )
            .u32();
        assert_eq!(back, result);
        assert_eq!(words_of(&e, vector), vec![1, 4]);
        assert_eq!(e.mem.u32(result), proxy);
        assert_eq!(e.mem.u32(result + 4), first + 4);
        // An empty range erases nothing.
        e.call(
            0x00ab_c180,
            &args![vector, result, 0u32, first, 0u32, first],
        );
        assert_eq!(words_of(&e, vector), vec![1, 4]);
        e.call(0x00ac_2f70, &args![vector]);
        assert!(words_of(&e, vector).is_empty());
    }

    /// Doubles for the 4-byte insert's helpers outside the unit: the tail fill (`00ac0ab0`)
    /// and the backward move (`00ac1440`).
    fn insert_engine() -> Engine {
        let mut e = engine();
        e.register(0x00ac_0ab0, |e, a| {
            let value = e.mem.u32(a[3]);
            for i in 0..a[2] {
                e.mem.set_u32(a[1] + 4 * i, value);
            }
            Ret::default()
        });
        e.register(0x00ac_1440, |e, a| {
            let bytes = e.mem.bytes(a[0], a[1] - a[0]);
            e.mem.write(a[2] - (a[1] - a[0]), &bytes);
            Ret::default()
        });
        e
    }

    #[test]
    fn word_insert_reallocates_or_shifts() {
        let mut e = insert_engine();
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 9);
        // No room: reallocation.
        let vector = word_vector(&mut e, &[1, 2, 3], 3);
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        e.call_log = Some(vec![]);
        e.call(0x00ab_c320, &args![vector, 0u32, first + 4, 2u32, value]);
        let log = e.call_log.take().unwrap();
        assert_eq!(words_of(&e, vector), vec![1, 9, 9, 2, 3]);
        assert_eq!(
            calls_to(&log, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, first]]
        );
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        // Capacity 3 grows by half to 4, below the 5 needed: 5.
        assert_eq!(e.mem.u32(vector.addr() + VECTOR_END), first + 20);

        // Room, fewer elements after `where` than inserted.
        let vector = word_vector(&mut e, &[1, 2, 3], 8);
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        e.call(0x00ab_c320, &args![vector, 0u32, first + 8, 2u32, value]);
        assert_eq!(words_of(&e, vector), vec![1, 2, 9, 9, 3]);

        // Room, more elements after `where`.
        let vector = word_vector(&mut e, &[1, 2, 3, 4], 8);
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        e.call(0x00ab_c320, &args![vector, 0u32, first + 4, 1u32, value]);
        assert_eq!(words_of(&e, vector), vec![1, 9, 2, 3, 4]);

        // Nothing for 0, the length throw past the maximum.
        e.call_log = Some(vec![]);
        e.call(0x00ab_c320, &args![vector, 0u32, first, 0u32, value]);
        e.call(
            0x00ab_c320,
            &args![vector, 0u32, first, 0x4000_0000u32, value],
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, THROW_LENGTH_00AC0B00).len(), 1);
        assert_eq!(words_of(&e, vector), vec![1, 9, 2, 3, 4]);
        // The word fill on its own.
        let block = e.mem.alloc(8);
        e.call(0x00ac_3450, &args![block, 2u32, value]);
        assert_eq!(e.mem.u32(block + 4), 9);
    }

    #[test]
    fn word_resize_grows_and_shrinks() {
        let mut e = insert_engine();
        let vector = word_vector(&mut e, &[1, 2], 2);
        e.call(0x00ab_a5d0, &args![vector, 4u32, 7u32]);
        assert_eq!(words_of(&e, vector), vec![1, 2, 7, 7]);
        e.call(0x00ab_a5d0, &args![vector, 1u32, 7u32]);
        assert_eq!(words_of(&e, vector), vec![1]);
        e.call(0x00ab_a5d0, &args![vector, 1u32, 7u32]);
        assert_eq!(words_of(&e, vector), vec![1]);
    }

    /// A vector of `size`-byte elements holding the words.
    fn element_vector(e: &mut Engine, words: &[u32], size: u32, capacity: u32) -> Ptr {
        let vector = word_vector(e, words, capacity * size / 4);
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        e.mem
            .set_u32(vector.addr() + VECTOR_END, first + capacity * size);
        vector
    }

    #[test]
    fn triple_insert_resize_and_copies() {
        let mut e = engine();
        e.register(0x0065_e310, |e, a| {
            e.mem.alloc((a[0] * 12).max(4)).into_ret()
        });
        let vector = element_vector(&mut e, &[1, 2, 3], 12, 1);
        // Resize to 3 (reallocates through 00abca90, 00ac3580 / 00ac3370, 00ac20d0).
        e.call(0x00ab_a7b0, &args![vector, 3u32, 7u32, 8u32, 9u32]);
        assert_eq!(words_of(&e, vector), vec![1, 2, 3, 7, 8, 9, 7, 8, 9]);
        // Room for more: insert at the end (no element follows).
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        e.mem.set_u32(vector.addr() + VECTOR_END, first + 0x100);
        let value = e.mem.alloc(12);
        e.mem.write(value, &[4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0]);
        let last = e.mem.u32(vector.addr() + VECTOR_LAST);
        e.call(0x00ab_ca90, &args![vector, 0u32, last, 1u32, value]);
        assert_eq!(words_of(&e, vector)[9..], [4, 5, 6]);
        // The copies on their own; a null destination is skipped.
        let destination = e.mem.alloc(24);
        assert_eq!(
            e.call(0x00ac_3580, &args![first, first + 24, destination, 0u32])
                .u32(),
            destination + 24
        );
        assert_eq!(e.mem.u32(destination + 12), 7);
        assert_eq!(
            e.call(
                0x00ac_3370,
                &args![first, first + 12, 0u32, 0u32, 0u32, 0u32]
            )
            .u32(),
            12
        );
        e.call(0x00ac_20d0, &args![0u32, 1u32, value, 0u32, 0u32, 0u32]);
        e.call(
            0x00ac_20d0,
            &args![destination, 1u32, value, 0u32, 0u32, 0u32],
        );
        assert_eq!(e.mem.u32(destination + 8), 6);
    }

    #[test]
    fn float_pair_insert_resize_and_copies() {
        let mut e = engine();
        e.register(0x0065_e3a0, |e, a| {
            e.mem.alloc((a[0] * 8).max(4)).into_ret()
        });
        let vector = element_vector(&mut e, &[1, 2], 8, 1);
        let signalling = 0x7f80_0001u32;
        e.register(0x00ab_af60, |e, a| {
            e.mem.set_u32(a[0] + VECTOR_LAST, a[3]);
            Ret::default()
        });
        e.register(0x00ac_0f40, |e, a| {
            let pointer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, pointer + 8 * a[1]);
            Ret::default()
        });
        e.call(
            0x00ab_ad80,
            &args![vector, 2u32, signalling, 0x3f80_0000u32],
        );
        // The new element's floats went through FLD/FSTP: the signalling NaN is quiet.
        assert_eq!(words_of(&e, vector), vec![1, 2, 0x7fc0_0001, 0x3f80_0000]);
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        e.call(0x00ab_ad80, &args![vector, 1u32, 0u32, 0u32]);
        assert_eq!(words_of(&e, vector), vec![1, 2]);
        let destination = e.mem.alloc(16);
        e.mem.set_u32(first, signalling);
        assert_eq!(
            e.call(0x00ac_3690, &args![first, first + 8, destination, 0u32])
                .u32(),
            destination + 8
        );
        assert_eq!(e.mem.u32(destination), 0x7fc0_0001);
        assert_eq!(
            e.call(
                0x00ac_3900,
                &args![first, first + 8, 0u32, 0u32, 0u32, 0u32]
            )
            .u32(),
            8
        );
        e.call(
            0x00ac_2290,
            &args![destination + 8, 1u32, first, 0u32, 0u32, 0u32],
        );
        assert_eq!(e.mem.u32(destination + 12), 2);
        // Inserting directly: room after the end.
        e.mem.set_u32(vector.addr() + VECTOR_END, first + 0x40);
        let value = e.mem.alloc(8);
        e.call(0x00ab_db10, &args![vector, 0u32, first + 8, 1u32, value]);
        assert_eq!(words_of(&e, vector).len(), 4);
    }

    #[test]
    fn by_address_resizes_call_their_helpers_and_destroy_the_value() {
        let mut e = engine();
        quiet(
            &mut e,
            &[
                0x00ab_d2e0,
                0x00ab_e310,
                0x00ab_eb00,
                STRING_TIDY,
                STRING_ASSIGN,
                VALUE_34_DESTRUCT,
                VALUE_38_DESTRUCT,
                0x0065_dc90,
                0x0065_cfd0,
                0x00ad_33c0,
            ],
        );
        e.register(0x0065_e550, |e, a| {
            e.mem.alloc((a[0] * 0x34).max(4)).into_ret()
        });
        e.register(0x0065_e5e0, |e, a| {
            e.mem.alloc((a[0] * 0x38).max(4)).into_ret()
        });
        // The first three insert through a callee outside the unit; each grows an empty
        // vector at its end by 3, given the address of the value, then destroys the value's
        // string (none for the first).
        let cases: [(u32, u32, Option<u32>, usize); 3] = [
            (0x00ab_aaa0, 0x00ab_d2e0, None, 4),
            (0x00ab_b060, 0x00ab_e310, Some(4), 8),
            (0x00ab_b3c0, 0x00ab_eb00, Some(0x10), 11),
        ];
        for (resize, insert, string, words) in cases {
            let vector = element_vector(&mut e, &[], 4, 0);
            let mut call = vec![vector.addr(), 3];
            call.extend((0..words as u32).map(|i| 0x100 + i));
            e.call_log = Some(vec![]);
            e.call(resize, &call);
            let log = e.call_log.take().unwrap();
            let inserted = calls_to(&log, insert);
            assert_eq!(inserted.len(), 1, "{resize:08x}");
            assert_eq!(inserted[0][3], 3);
            let slot = inserted[0][4];
            assert_eq!(e.mem.u32(slot), 0x100);
            let tidies = calls_to(&log, STRING_TIDY);
            match string {
                Some(offset) => assert_eq!(tidies, vec![vec![slot + offset, 1, 0]]),
                None => assert!(tidies.is_empty()),
            }
        }
        // The other two insert in the unit: three new elements, then the value destroyed.
        for (resize, allocate, destroy, words) in [
            (0x00ab_b700u32, 0x0065_e550u32, VALUE_34_DESTRUCT, 13usize),
            (0x00ab_bae0, 0x0065_e5e0, VALUE_38_DESTRUCT, 14),
        ] {
            let vector = element_vector(&mut e, &[], 4, 0);
            let mut call = vec![vector.addr(), 3];
            call.extend((0..words as u32).map(|i| 0x100 + i));
            e.call_log = Some(vec![]);
            e.call(resize, &call);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, allocate), vec![vec![3, 0]]);
            assert_eq!(calls_to(&log, destroy).len(), 1, "{resize:08x}");
        }
    }

    #[test]
    fn element_copies_build_strings_and_vectors() {
        let mut e = engine();
        e.register(STRING_TIDY, |_, _| Ret::default());
        e.register(STRING_ASSIGN, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x1c);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        quiet(&mut e, &[0x0065_dc90, 0x00ad_33c0]);
        e.register(0x0065_cfd0, |e, a| {
            let block = e.mem.alloc((a[1] * 12).max(4));
            e.mem.set_u32(a[0] + VECTOR_FIRST, block);
            e.mem.set_u32(a[0] + VECTOR_LAST, block);
            1u32.into_ret()
        });
        e.register(0x00ac_2060, |e, a| {
            let bytes = e.mem.bytes(a[1], a[3] - a[1]);
            e.mem.write(a[4], &bytes);
            (a[4] + a[3] - a[1]).into_ret()
        });
        // A 0x34-byte element: a string and a 12-byte-element vector of one element.
        let source = e.mem.alloc(0x34);
        e.mem.set_u32(source + 4, 0x6f6c_6568);
        let triples = e.mem.alloc(12);
        e.mem.write(triples, &[1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]);
        e.mem.set_u32(source + 0x1c + VECTOR_FIRST, triples);
        e.mem.set_u32(source + 0x1c + VECTOR_LAST, triples + 12);
        let copy = e.mem.alloc(0x34);
        assert_eq!(e.call(0x00ac_0530, &args![copy, source]).u32(), copy);
        assert_eq!(e.mem.u32(copy + 4), 0x6f6c_6568);
        let copied = e.mem.u32(copy + 0x1c + VECTOR_FIRST);
        assert_eq!(e.mem.u32(copy + 0x1c + VECTOR_LAST), copied + 12);
        assert_eq!(e.mem.u32(copied + 8), 3);
        // A 0x38-byte element: a string, a word and a word vector.
        let source = e.mem.alloc(0x38);
        e.mem.set_u32(source + 0x1c, 0x55);
        let words = e.mem.alloc(8);
        e.mem.set_u32(words, 0x11);
        e.mem.set_u32(words + 4, 0x22);
        e.mem.set_u32(source + 0x20 + VECTOR_FIRST, words);
        e.mem.set_u32(source + 0x20 + VECTOR_LAST, words + 8);
        let copy = e.mem.alloc(0x38);
        assert_eq!(e.call(0x00ac_05b0, &args![copy, source]).u32(), copy);
        assert_eq!(e.mem.u32(copy + 0x1c), 0x55);
        let copied = e.mem.u32(copy + 0x20 + VECTOR_FIRST);
        assert_eq!(e.mem.u32(copied + 4), 0x22);
        // A failed buy copies nothing.
        e.register(0x0065_cfd0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00ac_0640, &args![copy + 0x1c, source + 0x1c]);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x0065_dc90).len() == 1);
    }

    #[test]
    fn object_fills_and_copies_skip_null_slots() {
        let mut e = engine();
        e.register(0x00ac_0530, |_, a| a[0].into_ret());
        quiet(
            &mut e,
            &[
                STRING_TIDY,
                STRING_ASSIGN,
                0x0065_dc90,
                0x0065_cfd0,
                0x00ad_33c0,
            ],
        );
        let block = e.mem.alloc(0x100);
        // 0x34-byte elements: two copies, then a null destination.
        e.call_log = Some(vec![]);
        e.call(0x00ac_2540, &args![block, 2u32, block, 0u32, 0u32, 0u32]);
        e.call(0x00ac_2540, &args![0u32, 1u32, block, 0u32, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        let tidies = calls_to(&log, STRING_TIDY);
        assert_eq!(tidies.len(), 2);
        assert_eq!(tidies[1][0], block + 0x34);
        // 0x38-byte elements.
        e.call_log = Some(vec![]);
        e.call(0x00ac_2650, &args![block, 2u32, block, 0u32, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, STRING_TIDY)[1][0], block + 0x38);
        // The range copies return the destination's end (null destinations are skipped).
        let target = e.mem.alloc(0x100);
        let ends = [
            e.call(
                0x00ac_3bd0,
                &args![block, block + 0x68, target, 0u32, 0u32, 0u32],
            ),
            e.call(
                0x00ac_3ce0,
                &args![block, block + 0x70, target, 0u32, 0u32, 0u32],
            ),
            e.call(0x00ac_1ee0, &args![block, block + 0x34, 0u32, 0u32]),
            e.call(0x00ac_1fa0, &args![block, block + 0x38, 0u32, 0u32]),
            e.call(0x00ac_3780, &args![block, block, target, 0u32]),
            e.call(0x00ac_37d0, &args![block, block + 0x38, target, 0u32]),
        ];
        let ends: Vec<u32> = ends.iter().map(|r| r.u32()).collect();
        assert_eq!(
            ends,
            vec![
                target + 0x68,
                target + 0x70,
                0x34,
                0x38,
                target,
                target + 0x38
            ]
        );
    }

    #[test]
    fn object_inserts_reallocate_and_destroy_the_old_elements() {
        for (insert, size, allocate, copy, destroy) in [
            (
                0x00ab_f210u32,
                0x34u32,
                0x0065_e550u32,
                0x00ac_0530u32,
                VALUE_34_DESTRUCT,
            ),
            (
                0x00ab_fa90,
                0x38,
                0x0065_e5e0,
                0x00ac_05b0,
                VALUE_38_DESTRUCT,
            ),
        ] {
            let mut e = engine();
            e.register_double(allocate, move |e, a| {
                e.mem.alloc((a[0] * size).max(4)).into_ret()
            });
            e.register(copy, |_, a| a[0].into_ret());
            quiet(
                &mut e,
                &[
                    VALUE_34_DESTRUCT,
                    VALUE_38_DESTRUCT,
                    STRING_TIDY,
                    STRING_ASSIGN,
                    0x0065_dc90,
                    0x0065_cfd0,
                    0x00ad_33c0,
                    0x00ac_1670,
                    0x00ac_2760,
                    0x00ac_1f60,
                    0x00ac_2020,
                ],
            );
            let vector = element_vector(&mut e, &[0; 0x1c], size, 2);
            let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
            e.mem.set_u32(vector.addr() + VECTOR_LAST, first + 2 * size);
            let value = e.mem.alloc(size);
            e.call_log = Some(vec![]);
            e.call(insert, &args![vector, 0u32, first + size, 1u32, value]);
            let log = e.call_log.take().unwrap();
            // Two old elements destroyed, the old block freed, capacity 3.
            assert_eq!(calls_to(&log, destroy).len(), 2, "{insert:08x}");
            assert_eq!(
                calls_to(&log, MEMORY_MANAGER_FREE),
                vec![vec![MEMORY_MANAGER_OBJECT, first]]
            );
            let new_first = e.mem.u32(vector.addr() + VECTOR_FIRST);
            assert_eq!(e.mem.u32(vector.addr() + VECTOR_LAST), new_first + 3 * size);
            assert_eq!(e.mem.u32(vector.addr() + VECTOR_END), new_first + 3 * size);

            // With room: the value is copied into a temporary, which is destroyed after.
            e.mem
                .set_u32(vector.addr() + VECTOR_END, new_first + 8 * size);
            e.call_log = Some(vec![]);
            e.call(insert, &args![vector, 0u32, new_first, 1u32, value]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, destroy).len(), 1, "{insert:08x}");
            assert_eq!(e.mem.u32(vector.addr() + VECTOR_LAST), new_first + 4 * size);
        }
    }

    #[test]
    fn binary_file_constructor_sets_the_id_and_version() {
        let mut e = engine();
        e.register(STRING_TIDY, |_, _| Ret::default());
        e.register(STRING_ASSIGN, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x1c);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(FR2_ASSERT, |_, _| Ret::default());
        let file = e.mem.alloc(0x48);
        let id = e.mem.alloc(0x1c);
        e.mem.write(id + 4, b"FRCTL001");
        e.mem.set_u32(id + STRING_LENGTH, 8);
        e.mem.set_u32(id + 0x18, 15);
        let words = StringArg::load(&e, id);
        e.mem.set_u32(file + 0x3c, 9);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ac_3df0, &args![file, words]).u32(), file);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(file), FUT_BINARY_FILE_VTABLE);
        assert_eq!(e.mem.bytes(file + 0x24, 8), b"FRCTL001".to_vec());
        assert_eq!(e.mem.u32(file + 0x3c), 0);
        assert!(calls_to(&log, FR2_ASSERT).is_empty());
        // A short id asserts and still gets its version character.
        e.mem.set_u32(id + STRING_LENGTH, 6);
        let words = StringArg::load(&e, id);
        e.call_log = Some(vec![]);
        e.call(0x00ac_3df0, &args![file, words]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FR2_ASSERT),
            vec![vec![BINARY_FILE_SOURCE, 0x22]]
        );
        assert_eq!(e.mem.u8(file + 0x24 + 5), b'0');
    }

    #[test]
    fn word_vector_copy_constructor() {
        let mut e = engine();
        quiet(&mut e, &[0x00ad_33c0]);
        e.register(0x00ac_2060, |e, a| {
            let bytes = e.mem.bytes(a[1], a[3] - a[1]);
            e.mem.write(a[4], &bytes);
            (a[4] + a[3] - a[1]).into_ret()
        });
        let source = word_vector(&mut e, &[5, 6, 7], 3);
        let copy = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ac_08b0, &args![copy, source]).u32(), copy);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x00ad_33c0), vec![vec![copy, copy]]);
        assert_eq!(words_of(&e, Ptr::new(copy)), vec![5, 6, 7]);
        // An empty source buys nothing and copies nothing.
        let empty = word_vector(&mut e, &[], 1);
        e.call_log = Some(vec![]);
        e.call(0x00ac_08b0, &args![copy, empty]);
        assert!(calls_to(&e.call_log.take().unwrap(), 0x00ac_2060).is_empty());
    }
}

#[cfg(test)]
mod n062_tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    /// The allocators, `memmove_s` (a real move), strings as seven plain words (the assign
    /// copies them, the tidy leaves them), quiet checks and throws, and a call log.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_7000, 0x1000);
        e.set_global(STRING_NPOS, u32::MAX);
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| {
            e.mem.alloc(a[1].max(4)).into_ret()
        });
        e.register(MEMMOVE_S, |e, a| {
            let bytes = e.mem.bytes(a[2], a[3]);
            e.mem.write(a[0], &bytes);
            Ret::default()
        });
        e.register(STRING_ASSIGN, |e, a| {
            for i in 0..7 {
                let word = e.mem.u32(a[1] + 4 * i);
                e.mem.set_u32(a[0] + 4 * i, word);
            }
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                STRING_TIDY,
                MEMORY_MANAGER_FREE,
                INVALID_PARAMETER,
                THROW_LENGTH_0065F640,
                CXX_THROW_EXCEPTION,
            ],
        );
        e.call_log = Some(vec![]);
        e
    }

    /// A vector (0x18 bytes, its proxy pointing back at it) of `bytes`-sized elements holding
    /// `words`, with room for `capacity` elements.
    fn vector(e: &mut Engine, bytes: u32, words: &[u32], capacity: u32) -> u32 {
        let vector = e.mem.alloc(0x18);
        let proxy = e.mem.alloc(8);
        e.mem.set_u32(proxy, vector);
        e.mem.set_u32(vector + VECTOR_PROXY, proxy);
        let block = e.mem.alloc((bytes * capacity).max(4));
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *w);
        }
        e.mem.set_u32(vector + VECTOR_FIRST, block);
        e.mem
            .set_u32(vector + VECTOR_LAST, block + 4 * words.len() as u32);
        e.mem.set_u32(vector + VECTOR_END, block + bytes * capacity);
        vector
    }

    fn contents(e: &Engine, vector: u32) -> Vec<u32> {
        let first = e.mem.u32(vector + VECTOR_FIRST);
        let last = e.mem.u32(vector + VECTOR_LAST);
        (first..last).step_by(4).map(|a| e.mem.u32(a)).collect()
    }

    fn block(e: &mut Engine, words: &[u32]) -> u32 {
        let block = e.mem.alloc(4 * words.len().max(1) as u32);
        for (i, w) in words.iter().enumerate() {
            e.mem.set_u32(block + 4 * i as u32, *w);
        }
        block
    }

    fn read(e: &Engine, at: u32, count: u32) -> Vec<u32> {
        (0..count).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    /// `words` numbered from `start`.
    fn numbers(start: u32, count: u32) -> Vec<u32> {
        (start..start + count).collect()
    }

    fn invalid(e: &Engine) -> usize {
        calls(e, INVALID_PARAMETER).len()
    }

    #[test]
    fn insert_one_twelve_byte_element_answers_its_iterator() {
        let mut e = engine();
        let v = vector(&mut e, 0xc, &numbers(1, 6), 4);
        let proxy = e.mem.u32(v);
        let first = e.mem.u32(v + VECTOR_FIRST);
        let value = block(&mut e, &[7, 8, 9]);
        let result = e.mem.alloc(8);
        let r = e.call(0x00ab_a3c0, &args![v, result, proxy, first + 0xc, value]);
        assert_eq!(r.u32(), result);
        assert_eq!(contents(&e, v), vec![1, 2, 3, 7, 8, 9, 4, 5, 6]);
        assert_eq!(e.mem.u32(result), proxy);
        assert_eq!(e.mem.u32(result + 4), e.mem.u32(v + VECTOR_FIRST) + 0xc);
        assert_eq!(invalid(&e), 0);
        // A wrong proxy is reported.
        e.call(0x00ab_a3c0, &args![v, result, 0u32, first, value]);
        assert_eq!(invalid(&e), 1);
    }

    /// Erases elements 1..2 of a three-element vector of `bytes` and answers the result.
    fn erase_middle(e: &mut Engine, addr: u32, bytes: u32) -> (u32, u32) {
        let words = numbers(1, 3 * bytes / 4);
        let v = vector(e, bytes, &words, 3);
        let proxy = e.mem.u32(v);
        let first = e.mem.u32(v + VECTOR_FIRST);
        let result = e.mem.alloc(8);
        let r = e.call(
            addr,
            &args![v, result, proxy, first + bytes, proxy, first + 2 * bytes],
        );
        assert_eq!(r.u32(), result);
        assert_eq!(read(e, result, 2), vec![proxy, first + bytes]);
        assert_eq!(e.mem.u32(v + VECTOR_LAST), first + 2 * bytes);
        // An empty range changes nothing.
        e.call(
            addr,
            &args![v, result, proxy, first + bytes, proxy, first + bytes],
        );
        assert_eq!(e.mem.u32(v + VECTOR_LAST), first + 2 * bytes);
        (v, first)
    }

    #[test]
    fn erase_twelve_byte_elements() {
        let mut e = engine();
        let (v, _) = erase_middle(&mut e, 0x00ab_a9a0, 0xc);
        assert_eq!(contents(&e, v), vec![1, 2, 3, 7, 8, 9]);
    }

    #[test]
    fn erase_sixteen_byte_elements() {
        let mut e = engine();
        let (v, _) = erase_middle(&mut e, 0x00ab_ac80, 0x10);
        assert_eq!(contents(&e, v), vec![1, 2, 3, 4, 9, 10, 11, 12]);
    }

    #[test]
    fn erase_eight_byte_elements() {
        let mut e = engine();
        let (v, _) = erase_middle(&mut e, 0x00ab_af60, 8);
        assert_eq!(contents(&e, v), vec![1, 2, 5, 6]);
    }

    #[test]
    fn erase_word_and_string_elements_destroys_the_rest() {
        let mut e = engine();
        quiet(&mut e, &[DESTROY_RANGE_20]);
        let (v, first) = erase_middle(&mut e, 0x00ab_b280, 0x20);
        assert_eq!(contents(&e, v), [numbers(1, 8), numbers(17, 8)].concat());
        assert_eq!(
            calls(&e, DESTROY_RANGE_20),
            vec![vec![v, first + 0x40, first + 0x60]]
        );
    }

    #[test]
    fn erase_four_words_and_string_elements_destroys_the_rest() {
        let mut e = engine();
        quiet(&mut e, &[DESTROY_RANGE_2C]);
        let (v, first) = erase_middle(&mut e, 0x00ab_b5f0, 0x2c);
        assert_eq!(contents(&e, v), [numbers(1, 11), numbers(23, 11)].concat());
        assert_eq!(
            calls(&e, DESTROY_RANGE_2C),
            vec![vec![v, first + 0x58, first + 0x84]]
        );
    }

    #[test]
    fn erase_0x34_elements_destroys_each_one_left() {
        let mut e = engine();
        quiet(&mut e, &[VALUE_34_DESTRUCT, CLEAR_12]);
        let (_, first) = erase_middle(&mut e, 0x00ab_b930, 0x34);
        assert_eq!(calls(&e, VALUE_34_DESTRUCT), vec![vec![first + 0x68]]);
        // The string moved down, then the vector assigned (an empty source clears).
        assert_eq!(read(&e, first + 0x34, 7), numbers(27, 7));
        assert_eq!(calls(&e, CLEAR_12), vec![vec![first + 0x34 + 0x1c]]);
    }

    #[test]
    fn erase_0x38_elements_destroys_each_one_left() {
        let mut e = engine();
        quiet(&mut e, &[VALUE_38_DESTRUCT]);
        let (_, first) = erase_middle(&mut e, 0x00ab_bd10, 0x38);
        assert_eq!(calls(&e, VALUE_38_DESTRUCT), vec![vec![first + 0x70]]);
        // The string and the word moved down (the inner vector, its source empty, cleared).
        assert_eq!(read(&e, first + 0x38, 8), numbers(29, 8));
    }

    #[test]
    fn iterator_constructor_checks_its_vector() {
        let mut e = engine();
        let v = vector(&mut e, 4, &[1, 2], 2);
        let first = e.mem.u32(v + VECTOR_FIRST);
        let iterator = e.mem.alloc(8);
        assert_eq!(
            e.call(0x00ab_beb0, &args![iterator, first + 4, v]).u32(),
            iterator
        );
        assert_eq!(read(&e, iterator, 2), vec![e.mem.u32(v), first + 4]);
        assert_eq!(invalid(&e), 0);
        e.call(0x00ab_beb0, &args![iterator, first + 12, v]);
        assert_eq!(invalid(&e), 1);
    }

    #[test]
    fn iterator_equality_checks_its_container() {
        let mut e = engine();
        let a = block(&mut e, &[0x40, 0x100]);
        let b = block(&mut e, &[0x40, 0x100]);
        assert!(e.call(0x00ab_bf00, &args![a, b]).bool());
        e.mem.set_u32(b + 4, 0x104);
        assert!(!e.call(0x00ab_bf00, &args![a, b]).bool());
        assert_eq!(invalid(&e), 0);
        e.mem.set_u32(b, 0x44);
        e.call(0x00ab_bf00, &args![a, b]);
        assert_eq!(invalid(&e), 1);
    }

    #[test]
    fn iterator_dereference_checks_the_end() {
        let mut e = engine();
        let v = vector(&mut e, 4, &[1, 2], 2);
        let first = e.mem.u32(v + VECTOR_FIRST);
        let proxy = e.mem.u32(v);
        let iterator = block(&mut e, &[proxy, first + 4]);
        assert_eq!(e.call(0x00ab_bf50, &args![iterator]).u32(), first + 4);
        assert_eq!(invalid(&e), 0);
        e.mem.set_u32(iterator + 4, first + 8);
        assert_eq!(e.call(0x00ab_bf50, &args![iterator]).u32(), first + 8);
        assert_eq!(invalid(&e), 1);
    }

    #[test]
    fn insert_n_sixteen_byte_elements() {
        let mut e = engine();
        e.register(0x0064_e430, |e, a| e.mem.alloc(16 * a[0]).into_ret());
        let v = vector(&mut e, 0x10, &numbers(1, 8), 3);
        let first = e.mem.u32(v + VECTOR_FIRST);
        let value = block(&mut e, &[9, 9, 9, 9]);
        // In place, one following: it is copied past the end and the value stored.
        e.call(0x00ab_d2e0, &args![v, 0u32, first + 0x10, 1u32, value]);
        assert_eq!(
            contents(&e, v),
            [numbers(1, 4), vec![9; 4], numbers(5, 4)].concat()
        );
        // Past the capacity: a new block of half as much again.
        e.call(0x00ab_d2e0, &args![v, 0u32, first, 1u32, value]);
        assert_eq!(
            e.mem.u32(v + VECTOR_LAST) - e.mem.u32(v + VECTOR_FIRST),
            0x40
        );
        assert_eq!(calls(&e, 0x0064_e430), vec![vec![4, 0]]);
        assert_eq!(contents(&e, v)[..4], [9; 4]);
        // Too many: the length throw.
        e.call(0x00ab_d2e0, &args![v, 0u32, first, 0x1000_0000u32, value]);
        assert_eq!(calls(&e, THROW_LENGTH_0065F640).len(), 1);
    }

    #[test]
    fn insert_n_word_and_string_elements() {
        let mut e = engine();
        quiet(&mut e, &[DESTROY_RANGE_20]);
        e.register(0x0065_e430, |e, a| e.mem.alloc(0x20 * a[0]).into_ret());
        let v = vector(&mut e, 0x20, &numbers(1, 16), 3);
        let first = e.mem.u32(v + VECTOR_FIRST);
        let value = block(&mut e, &numbers(100, 8));
        // In place with more following: the tail copied past the end, the rest moved up.
        e.call(0x00ab_e310, &args![v, 0u32, first, 1u32, value]);
        assert_eq!(contents(&e, v), [numbers(100, 8), numbers(1, 16)].concat());
        // Growing: the old elements destroyed as a range and freed.
        e.call(0x00ab_e310, &args![v, 0u32, first + 0x60, 1u32, value]);
        assert_eq!(
            calls(&e, DESTROY_RANGE_20),
            vec![vec![v, first, first + 0x60]]
        );
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, first]]
        );
        assert_eq!(contents(&e, v).len(), 32);
        // The temporary's string destroyed with (1, 0).
        assert!(calls(&e, STRING_TIDY).iter().any(|c| c[1..] == [1, 0]));
        // Nothing for 0.
        e.call_log = Some(vec![]);
        e.call(0x00ab_e310, &args![v, 0u32, first, 0u32, value]);
        assert_eq!(calls(&e, STRING_TIDY).len(), 0);
    }

    #[test]
    fn insert_n_four_words_and_string_elements() {
        let mut e = engine();
        quiet(&mut e, &[DESTROY_RANGE_2C]);
        e.register(0x0065_e4c0, |e, a| e.mem.alloc(0x2c * a[0]).into_ret());
        let v = vector(&mut e, 0x2c, &numbers(1, 11), 3);
        let first = e.mem.u32(v + VECTOR_FIRST);
        let value = block(&mut e, &numbers(100, 11));
        // In place at the end: the gap filled from the temporary.
        e.call(0x00ab_eb00, &args![v, 0u32, first + 0x2c, 2u32, value]);
        assert_eq!(
            contents(&e, v),
            [numbers(1, 11), numbers(100, 11), numbers(100, 11)].concat()
        );
        // In place before the last one: moved up and assigned.
        e.mem.set_u32(v + VECTOR_LAST, first + 0x58);
        e.call(0x00ab_eb00, &args![v, 0u32, first, 1u32, value]);
        assert_eq!(
            contents(&e, v),
            [numbers(100, 11), numbers(1, 11), numbers(100, 11)].concat()
        );
        // Too many: the length throw.
        e.call(0x00ab_eb00, &args![v, 0u32, first, 0x0600_0000u32, value]);
        assert_eq!(calls(&e, THROW_LENGTH_0065F640).len(), 1);
    }

    #[test]
    fn four_words_and_string_copy_constructor() {
        let mut e = engine();
        let source = block(&mut e, &numbers(1, 11));
        let this = e.mem.alloc(0x2c);
        assert_eq!(e.call(0x00ac_04c0, &args![this, source]).u32(), this);
        assert_eq!(read(&e, this, 11), numbers(1, 11));
        assert_eq!(calls(&e, STRING_TIDY), vec![vec![this + 0x10, 0, 0]]);
    }

    #[test]
    fn word_fill_answers_its_end() {
        let mut e = engine();
        let value = block(&mut e, &[7]);
        let to = e.mem.alloc(12);
        assert_eq!(
            e.call(0x00ac_0ab0, &args![0u32, to, 3u32, value]).u32(),
            to + 12
        );
        assert_eq!(read(&e, to, 3), vec![7; 3]);
    }

    #[test]
    fn length_error_is_thrown() {
        let mut e = engine();
        quiet(&mut e, &[STRING_FROM_TEXT, LENGTH_ERROR_CONSTRUCT]);
        e.call(0x00ac_0b00, &[]);
        let text = calls(&e, STRING_FROM_TEXT);
        assert_eq!(text[0][1], TOO_LONG_TEXT);
        let thrown = calls(&e, CXX_THROW_EXCEPTION);
        assert_eq!(thrown[0][1], LENGTH_ERROR_THROW_INFO);
        assert_eq!(
            calls(&e, LENGTH_ERROR_CONSTRUCT),
            vec![vec![thrown[0][0], text[0][0]]]
        );
    }

    /// An iterator over a three-element vector of `bytes`, advanced by 2 (inside) then by 2
    /// more (past the end, reported).
    fn advance(addr: u32, bytes: u32) {
        let mut e = engine();
        let v = vector(&mut e, bytes, &vec![0; 3 * bytes as usize / 4], 3);
        let first = e.mem.u32(v + VECTOR_FIRST);
        let proxy = e.mem.u32(v);
        let iterator = block(&mut e, &[proxy, first]);
        assert_eq!(e.call(addr, &args![iterator, 2u32]).u32(), iterator);
        assert_eq!(e.mem.u32(iterator + 4), first + 2 * bytes);
        assert_eq!(invalid(&e), 0);
        e.call(addr, &args![iterator, 2u32]);
        assert_eq!(invalid(&e), 1);
        assert_eq!(e.mem.u32(iterator + 4), first + 4 * bytes);
        // Below the first: reported too.
        e.call(addr, &args![iterator, u32::MAX.wrapping_mul(5)]);
        assert_eq!(invalid(&e), 2);
    }

    #[test]
    fn advance_0x20() {
        advance(0x00ac_0b80, 0x20);
    }

    #[test]
    fn advance_0x2c() {
        advance(0x00ac_0c40, 0x2c);
    }

    #[test]
    fn advance_0x34() {
        advance(0x00ac_0d00, 0x34);
    }

    #[test]
    fn advance_0xc() {
        advance(0x00ac_0dc0, 0xc);
    }

    #[test]
    fn advance_0x10() {
        advance(0x00ac_0e80, 0x10);
    }

    #[test]
    fn advance_8() {
        advance(0x00ac_0f40, 8);
    }

    #[test]
    fn point_sum_and_scale() {
        let mut e = engine();
        let a = block(&mut e, &[1.5f32.to_bits(), 2.0f32.to_bits(), 0x7f80_0001]);
        let b = block(
            &mut e,
            &[0.25f32.to_bits(), (-2.0f32).to_bits(), 1.0f32.to_bits()],
        );
        let result = e.mem.alloc(12);
        assert_eq!(e.call(0x00ac_10c0, &args![a, result, b]).u32(), result);
        assert_eq!(e.mem.f32(result), 1.75);
        assert_eq!(e.mem.f32(result + 4), 0.0);
        assert_eq!(e.mem.u32(result + 8), 0x7fc0_0001);
        assert_eq!(e.call(0x00ac_1120, &args![a, result, 2.0f32]).u32(), result);
        assert_eq!(
            read(&e, result, 2),
            vec![3.0f32.to_bits(), 4.0f32.to_bits()]
        );
        assert_eq!(e.mem.u32(result + 8), 0x7fc0_0001);
    }

    /// A forward copy of three `bytes` elements answers the destination's end.
    fn forward(addr: u32, bytes: u32, extra: usize) {
        let mut e = engine();
        let words = numbers(1, 3 * bytes / 4);
        let from = block(&mut e, &words);
        let to = e.mem.alloc(3 * bytes);
        let mut a = vec![from, from + 3 * bytes, to];
        a.resize(3 + extra, 0);
        assert_eq!(e.call(addr, &a).u32(), to + 3 * bytes);
        assert_eq!(read(&e, to, 3 * bytes / 4), words);
    }

    #[test]
    fn forward_copy_12() {
        forward(0x00ac_1170, 0xc, 0);
    }

    #[test]
    fn forward_copy_16() {
        forward(0x00ac_1220, 0x10, 0);
    }

    #[test]
    fn forward_copy_8() {
        forward(0x00ac_12d0, 8, 0);
    }

    #[test]
    fn forward_copy_12_with_tags() {
        forward(0x00ac_2c20, 0xc, 3);
    }

    /// A backward copy of two `bytes` elements one element up answers the new start.
    fn backward(addr: u32, bytes: u32, extra: usize) {
        let mut e = engine();
        let words = numbers(1, 3 * bytes / 4);
        let at = block(&mut e, &words);
        let mut a = vec![at, at + 2 * bytes, at + 3 * bytes];
        a.resize(3 + extra, 0);
        assert_eq!(e.call(addr, &a).u32(), at + bytes);
        let n = bytes / 4;
        assert_eq!(
            read(&e, at, 3 * n),
            [numbers(1, n), numbers(1, 2 * n)].concat()
        );
    }

    #[test]
    fn backward_copy_12() {
        backward(0x00ac_1370, 0xc, 0);
    }

    #[test]
    fn backward_copy_12_with_tags() {
        backward(0x00ac_1a50, 0xc, 3);
    }

    #[test]
    fn backward_copy_16() {
        backward(0x00ac_14e0, 0x10, 0);
    }

    #[test]
    fn backward_copy_16_with_tags() {
        backward(0x00ac_1bf0, 0x10, 3);
    }

    #[test]
    fn backward_copy_8() {
        backward(0x00ac_1530, 8, 0);
    }

    #[test]
    fn backward_copy_8_with_tags() {
        backward(0x00ac_1d10, 8, 3);
    }

    #[test]
    fn backward_assign_0x20() {
        backward(0x00ac_1580, 0x20, 0);
    }

    #[test]
    fn backward_assign_0x20_with_tags() {
        backward(0x00ac_3030, 0x20, 3);
    }

    #[test]
    fn backward_assign_0x2c() {
        backward(0x00ac_30c0, 0x2c, 3);
    }

    #[test]
    fn backward_assign_0x2c_with_a_byte_tag() {
        backward(0x00ac_1ea0, 0x2c, 3);
    }

    #[test]
    fn word_moves_up_to_an_end() {
        let mut e = engine();
        let at = block(&mut e, &[1, 2, 3, 4]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00ac_1440, &args![at, at + 8, at + 16]).u32(),
            at + 8
        );
        assert_eq!(read(&e, at, 4), vec![1, 2, 1, 2]);
        assert_eq!(calls(&e, MEMMOVE_S), vec![vec![at + 8, 8, at, 8]]);
        // Nothing to move: no call.
        assert_eq!(e.call(0x00ac_1440, &args![at, at, at + 16]).u32(), at + 16);
        assert_eq!(calls(&e, MEMMOVE_S).len(), 1);
    }

    #[test]
    fn range_assign_0x2c() {
        let mut e = engine();
        let value = block(&mut e, &numbers(50, 11));
        let at = e.mem.alloc(0x58);
        e.call(0x00ac_1600, &args![at, at + 0x58, value]);
        assert_eq!(
            read(&e, at, 22),
            [numbers(50, 11), numbers(50, 11)].concat()
        );
        assert_eq!(calls(&e, STRING_ASSIGN).len(), 2);
    }

    #[test]
    fn range_assign_0x34() {
        let mut e = engine();
        quiet(&mut e, &[CLEAR_12]);
        let value = e.mem.alloc(0x34);
        e.mem.set_u32(value, 0x51);
        let at = e.mem.alloc(0x68);
        e.call(0x00ac_1670, &args![at, at + 0x68, value]);
        assert_eq!((e.mem.u32(at), e.mem.u32(at + 0x34)), (0x51, 0x51));
        assert_eq!(calls(&e, CLEAR_12), vec![vec![at + 0x1c], vec![at + 0x50]]);
    }

    #[test]
    fn forward_assign_0x20() {
        forward(0x00ac_1780, 0x20, 3);
    }

    #[test]
    fn forward_assign_0x2c() {
        forward(0x00ac_1810, 0x2c, 3);
    }

    #[test]
    fn forward_assign_0x34() {
        let mut e = engine();
        quiet(&mut e, &[CLEAR_12]);
        let from = e.mem.alloc(0x68);
        e.mem.set_u32(from + 0x34, 0x77);
        let to = e.mem.alloc(0x68);
        let r = e.call(0x00ac_18c0, &args![from, from + 0x68, to, 0u32, 0u32, 0u32]);
        assert_eq!(r.u32(), to + 0x68);
        assert_eq!(e.mem.u32(to + 0x34), 0x77);
        assert_eq!(calls(&e, CLEAR_12), vec![vec![to + 0x1c], vec![to + 0x50]]);
    }

    #[test]
    fn forward_assign_0x38() {
        let mut e = engine();
        let from = e.mem.alloc(0x38);
        e.mem.set_u32(from, 0x66);
        e.mem.set_u32(from + 0x1c, 0x67);
        let to = e.mem.alloc(0x38);
        let r = e.call(0x00ac_1980, &args![from, from + 0x38, to, 0u32, 0u32, 0u32]);
        assert_eq!(r.u32(), to + 0x38);
        assert_eq!((e.mem.u32(to), e.mem.u32(to + 0x1c)), (0x66, 0x67));
        // The inner vectors are empty: only the string is assigned.
        assert_eq!(calls(&e, STRING_ASSIGN).len(), 1);
    }

    /// A four-word copy through a forwarding function answers the destination's end.
    fn forwarded(addr: u32, bytes: u32) {
        let mut e = engine();
        let words = numbers(1, 2 * bytes / 4);
        let from = block(&mut e, &words);
        let to = e.mem.alloc(2 * bytes);
        let r = e.call(addr, &args![from, from + 2 * bytes, to, 0u32]);
        assert_eq!(r.u32(), to + 2 * bytes);
        assert_eq!(read(&e, to, 2 * bytes / 4), words);
    }

    #[test]
    fn forwarded_copy_12() {
        forwarded(0x00ac_1af0, 0xc);
    }

    #[test]
    fn forwarded_copy_16() {
        forwarded(0x00ac_1b70, 0x10);
    }

    #[test]
    fn forwarded_copy_float_pairs() {
        forwarded(0x00ac_1c90, 8);
    }

    #[test]
    fn forwarded_construct_0x20() {
        forwarded(0x00ac_1da0, 0x20);
    }

    #[test]
    fn forwarded_construct_0x2c() {
        forwarded(0x00ac_1e20, 0x2c);
    }

    #[test]
    fn forwarded_copy_16_again() {
        forwarded(0x00ac_3640, 0x10);
    }

    #[test]
    fn forwarded_construct_0x20_again() {
        forwarded(0x00ac_36e0, 0x20);
    }

    #[test]
    fn forwarded_construct_0x2c_again() {
        forwarded(0x00ac_3730, 0x2c);
    }

    #[test]
    fn copy_16_skips_null_slots() {
        forward(0x00ac_3820, 0x10, 3);
        let mut e = engine();
        let from = block(&mut e, &numbers(1, 4));
        assert_eq!(
            e.call(0x00ac_3820, &args![from, from + 16, 0u32, 0u32, 0u32, 0u32])
                .u32(),
            16
        );
    }

    #[test]
    fn construct_0x20_elements() {
        forward(0x00ac_39e0, 0x20, 3);
    }

    #[test]
    fn construct_0x2c_elements_skips_null_slots() {
        forward(0x00ac_3ac0, 0x2c, 3);
        let mut e = engine();
        let from = block(&mut e, &numbers(1, 11));
        e.call(
            0x00ac_3ac0,
            &args![from, from + 0x2c, 0u32, 0u32, 0u32, 0u32],
        );
        assert!(calls(&e, STRING_TIDY).is_empty());
    }

    #[test]
    fn backward_assign_0x34() {
        let mut e = engine();
        quiet(&mut e, &[CLEAR_12]);
        let at = e.mem.alloc(0x68);
        e.mem.set_u32(at, 0x31);
        let r = e.call(
            0x00ac_1f60,
            &args![at, at + 0x34, at + 0x68, 0u32, 0u32, 0u32],
        );
        assert_eq!(r.u32(), at + 0x34);
        assert_eq!(e.mem.u32(at + 0x34), 0x31);
        assert_eq!(calls(&e, CLEAR_12), vec![vec![at + 0x50]]);
    }

    #[test]
    fn backward_assign_0x34_without_forwarding() {
        let mut e = engine();
        quiet(&mut e, &[CLEAR_12]);
        let at = e.mem.alloc(0x68);
        e.mem.set_u32(at, 0x32);
        e.call(
            0x00ac_3170,
            &args![at, at + 0x34, at + 0x68, 0u32, 0u32, 0u32],
        );
        assert_eq!(e.mem.u32(at + 0x34), 0x32);
    }

    #[test]
    fn backward_assign_0x38() {
        let mut e = engine();
        let at = e.mem.alloc(0x70);
        e.mem.set_u32(at + 0x1c, 0x41);
        let r = e.call(
            0x00ac_2020,
            &args![at, at + 0x38, at + 0x70, 0u32, 0u32, 0u32],
        );
        assert_eq!(r.u32(), at + 0x38);
        assert_eq!(e.mem.u32(at + 0x38 + 0x1c), 0x41);
    }

    #[test]
    fn backward_assign_0x38_without_forwarding() {
        let mut e = engine();
        let at = e.mem.alloc(0x70);
        e.mem.set_u32(at + 0x1c, 0x42);
        e.call(
            0x00ac_3230,
            &args![at, at + 0x38, at + 0x70, 0u32, 0u32, 0u32],
        );
        assert_eq!(e.mem.u32(at + 0x38 + 0x1c), 0x42);
    }

    #[test]
    fn iterator_word_move() {
        let mut e = engine();
        let from = block(&mut e, &[1, 2, 3]);
        let to = e.mem.alloc(12);
        let r = e.call(0x00ac_2060, &args![0u32, from, 0u32, from + 12, to, 0u32]);
        assert_eq!(r.u32(), to + 12);
        assert_eq!(read(&e, to, 3), vec![1, 2, 3]);
        assert_eq!(
            e.call(0x00ac_2060, &args![0u32, from, 0u32, from, to, 0u32])
                .u32(),
            to
        );
        assert_eq!(calls(&e, MEMMOVE_S).len(), 1);
    }

    /// A fill of two elements of `bytes` at a destination (and none at a null one).
    fn fill(addr: u32, bytes: u32) {
        let mut e = engine();
        let value = block(&mut e, &numbers(1, bytes / 4));
        let to = e.mem.alloc(2 * bytes);
        e.call(addr, &args![to, 2u32, value, 0u32, 0u32, 0u32]);
        assert_eq!(
            read(&e, to, 2 * bytes / 4),
            [numbers(1, bytes / 4), numbers(1, bytes / 4)].concat()
        );
        e.call_log = Some(vec![]);
        e.call(addr, &args![0u32, 1u32, value, 0u32, 0u32, 0u32]);
        assert!(calls(&e, STRING_TIDY).is_empty());
    }

    #[test]
    fn fill_16() {
        fill(0x00ac_21b0, 0x10);
    }

    #[test]
    fn fill_0x20() {
        fill(0x00ac_2370, 0x20);
    }

    #[test]
    fn fill_0x2c() {
        fill(0x00ac_2440, 0x2c);
    }

    #[test]
    fn construct_0x20_element() {
        let mut e = engine();
        let source = block(&mut e, &numbers(1, 8));
        let to = e.mem.alloc(0x20);
        e.call(0x00ac_34c0, &args![to, source]);
        assert_eq!(read(&e, to, 8), numbers(1, 8));
        assert_eq!(calls(&e, STRING_TIDY), vec![vec![to + 4, 0, 0]]);
        e.call(0x00ac_34c0, &args![0u32, source]);
        assert_eq!(calls(&e, STRING_TIDY).len(), 1);
    }

    #[test]
    fn assign_0x38_element() {
        let mut e = engine();
        let other = e.mem.alloc(0x38);
        e.mem.set_u32(other, 0x11);
        e.mem.set_u32(other + 0x1c, 0x22);
        let this = e.mem.alloc(0x38);
        assert_eq!(e.call(0x00ac_2760, &args![this, other]).u32(), this);
        assert_eq!((e.mem.u32(this), e.mem.u32(this + 0x1c)), (0x11, 0x22));
    }

    #[test]
    fn assign_twelve_byte_vectors() {
        let mut e = engine();
        quiet(&mut e, &[CLEAR_12]);
        e.register(BUY_12, |e, a| {
            let block = e.mem.alloc(12 * a[1]);
            e.mem.set_u32(a[0] + VECTOR_FIRST, block);
            e.mem.set_u32(a[0] + VECTOR_LAST, block);
            e.mem.set_u32(a[0] + VECTOR_END, block + 12 * a[1]);
            1u32.into_ret()
        });
        let this = vector(&mut e, 0xc, &numbers(1, 6), 3);
        // Smaller: copied over the first ones.
        let other = vector(&mut e, 0xc, &numbers(10, 3), 1);
        assert_eq!(e.call(0x00ac_27c0, &args![this, other]).u32(), this);
        assert_eq!(contents(&e, this), numbers(10, 3));
        // Within the capacity: copied and constructed.
        let other = vector(&mut e, 0xc, &numbers(20, 9), 3);
        e.call(0x00ac_27c0, &args![this, other]);
        assert_eq!(contents(&e, this), numbers(20, 9));
        // Larger: freed, bought, constructed.
        let first = e.mem.u32(this + VECTOR_FIRST);
        let other = vector(&mut e, 0xc, &numbers(30, 12), 4);
        e.call(0x00ac_27c0, &args![this, other]);
        assert_eq!(contents(&e, this), numbers(30, 12));
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, first]]
        );
        // Empty: cleared; itself: nothing.
        let other = vector(&mut e, 0xc, &[], 1);
        e.call(0x00ac_27c0, &args![this, other]);
        e.call(0x00ac_27c0, &args![this, this]);
        assert_eq!(calls(&e, CLEAR_12), vec![vec![this]]);
    }

    #[test]
    fn assign_word_vectors() {
        let mut e = engine();
        let this = vector(&mut e, 4, &[1, 2, 3], 4);
        let other = vector(&mut e, 4, &[7, 8], 2);
        assert_eq!(e.call(0x00ac_2ca0, &args![this, other]).u32(), this);
        assert_eq!(contents(&e, this), vec![7, 8]);
        let other = vector(&mut e, 4, &[4, 5, 6, 7], 4);
        e.call(0x00ac_2ca0, &args![this, other]);
        assert_eq!(contents(&e, this), vec![4, 5, 6, 7]);
        let first = e.mem.u32(this + VECTOR_FIRST);
        let other = vector(&mut e, 4, &numbers(1, 6), 6);
        e.call(0x00ac_2ca0, &args![this, other]);
        assert_eq!(contents(&e, this), numbers(1, 6));
        assert_eq!(
            calls(&e, MEMORY_MANAGER_FREE),
            vec![vec![MEMORY_MANAGER_OBJECT, first]]
        );
        let other = vector(&mut e, 4, &[], 1);
        e.call(0x00ac_2ca0, &args![this, other]);
        assert!(contents(&e, this).is_empty());
    }

    #[test]
    fn set_file_id_asserts_and_sets_the_version() {
        let mut e = engine();
        quiet(&mut e, &[FR2_ASSERT]);
        let this = e.mem.alloc(0x48);
        let id = e.mem.alloc(0x1c);
        e.mem.write(id + 4, b"FRTRI103\0");
        e.mem.set_u32(id + STRING_LENGTH, 8);
        e.mem.set_u32(id + 0x18, 0xf);
        e.call(0x00ac_3f40, &args![this, id]);
        assert!(calls(&e, FR2_ASSERT).is_empty());
        assert_eq!(e.mem.bytes(this + 0x24, 8), b"FRTRI003".to_vec());
        e.mem.set_u32(id + STRING_LENGTH, 4);
        e.mem.set_u32(this + 0x3c, 1);
        e.call(0x00ac_3f40, &args![this, id]);
        assert_eq!(
            calls(&e, FR2_ASSERT),
            vec![
                vec![BINARY_FILE_SOURCE, 0x30],
                vec![BINARY_FILE_SOURCE, 0x31]
            ]
        );
        assert_eq!(invalid(&e), 1);
    }

    /// A `FutBinaryFileC` whose id is `"FRTRI003"`, and a name string.
    fn id_check_engine() -> (Engine, u32, u32) {
        let mut e = engine();
        let this = e.mem.alloc(0x48);
        e.mem.write(this + 0x24, b"FRTRI003\0");
        e.mem.set_u32(this + 0x34, 8);
        e.mem.set_u32(this + 0x38, 0xf);
        let name = e.mem.alloc(0x1c);
        e.mem.set_u32(name + 0x18, 0xf);
        e.register(BYTE_VECTOR_CONSTRUCT, |e, a| {
            let block = e.mem.alloc(a[1]);
            e.mem.set_u32(a[0] + VECTOR_FIRST, block);
            e.mem.set_u32(a[0] + VECTOR_LAST, block + a[1]);
            Ret::default()
        });
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(STRING_COMPARE, |e, a| {
            let id = e.mem.bytes(a[0] + 4, a[2]);
            let text = e.mem.bytes(a[3], a[4]);
            u32::from(id != text).into_ret()
        });
        quiet(&mut e, &[FCLOSE, SET_LAST_ERROR]);
        (e, this, name)
    }

    #[test]
    fn file_id_check_reads_and_compares() {
        let (mut e, this, name) = id_check_engine();
        e.register(FOPEN, |_, _| 0x5000u32.into_ret());
        e.register(FREAD, |e, a| {
            e.mem.write(a[0], b"FRTRI103");
            1u32.into_ret()
        });
        // The version character comes from the id: equal.
        assert!(e.call(0x00ac_3fe0, &args![this, name]).bool());
        assert_eq!(calls(&e, FOPEN), vec![vec![name + 4, READ_BINARY_MODE]]);
        assert_eq!(calls(&e, FCLOSE), vec![vec![0x5000]]);
        assert_eq!(calls(&e, MEMORY_MANAGER_FREE).len(), 2);
        // Another id: not equal.
        e.register(FREAD, |e, a| {
            e.mem.write(a[0], b"FRTRX003");
            1u32.into_ret()
        });
        assert!(!e.call(0x00ac_3fe0, &args![this, name]).bool());
        // A short read: closed, error 1.
        e.call_log = Some(vec![]);
        e.register(FREAD, |_, _| Ret::default());
        assert!(!e.call(0x00ac_3fe0, &args![this, name]).bool());
        assert_eq!(calls(&e, FCLOSE).len(), 1);
        assert_eq!(calls(&e, SET_LAST_ERROR)[0][0], 1);
        // No file: error 4.
        e.call_log = Some(vec![]);
        e.register(FOPEN, |_, _| Ret::default());
        assert!(!e.call(0x00ac_3fe0, &args![this, name]).bool());
        assert_eq!(calls(&e, SET_LAST_ERROR)[0][0], 4);
        assert!(calls(&e, FCLOSE).is_empty());
    }
}
