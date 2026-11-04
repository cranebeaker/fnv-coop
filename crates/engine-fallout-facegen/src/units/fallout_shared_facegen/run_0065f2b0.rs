//! `fallout shared/facegen/run_0065f2b0` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_0065f580` calls on the member (`0065f7c0`).
const CALLEE_0065F7C0: u32 = 0x0065_f7c0;

/// The function `fn_0065f6e0` calls on the member (`0065f7f0`).
const CALLEE_0065F7F0: u32 = 0x0065_f7f0;

// Translated from 0065f580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0065f7c0` with `ECX` = `this + 0x8` and passes on its result.
pub fn fn_0065f580(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_0065F7C0, &args![this.byte_add(0x8)]).u32()
}

// Translated from 0065f6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `0065f7f0` with `ECX` = `this + 0x8` and passes on its result.
pub fn fn_0065f6e0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(CALLEE_0065F7F0, &args![this.byte_add(0x8)]).u32()
}

// The functions below are `std::vector` template code for 6-byte elements and the copy
// algorithms it uses (unnamed in the map; the element type is not identified), plus the
// copy constructors of two vectors and of the 0x28-byte record holding three of them. The
// vectors keep their allocator at `+8` and `_Myfirst`, `_Mylast` and `_Myend` at `+0xc`,
// `+0x10` and `+0x14`.

/// The vector's allocator, `_Myfirst`, `_Mylast` and `_Myend`.
const VECTOR_ALLOCATOR: u32 = 0x08;
const VECTOR_FIRST: u32 = 0x0c;
const VECTOR_LAST: u32 = 0x10;
const VECTOR_END: u32 = 0x14;
/// `0065e750` (registered with the caller's `ECX` as its first word, then two stack words it
/// does not read): gives the iterator tag byte.
const ITERATOR_TAG_0065E750: u32 = 0x0065_e750;
/// `0043b1d0` (cdecl, two words; reads only the first): the word an iterator points at.
const UNWRAP_0043B1D0: u32 = 0x0043_b1d0;
/// `operator new` (`00401000`, cdecl, one word), `std::bad_alloc`'s constructor (`0044d6c0`,
/// thiscall, one word), `_CxxThrowException` (`00ec7894`, two words; it does not return in the
/// game) and `std::bad_alloc`'s throw information (`01140ef8`).
const OPERATOR_NEW: u32 = 0x0040_1000;
const BAD_ALLOC_CTOR_0044D6C0: u32 = 0x0044_d6c0;
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
const BAD_ALLOC_THROW_INFO_01140EF8: u32 = 0x0114_0ef8;

// Translated from 0065f2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::_Destroy(first, last)` for the 6-byte vector (thiscall, two words, `RET
/// 8`): [`fn_0065f960`] with the range and the allocator at `+8`.
pub fn fn_0065f2b0(e: &mut Engine, this: Ptr, first: u32, last: u32) {
    fn_0065f960(e, first, last, this.addr() + VECTOR_ALLOCATOR);
}

// Translated from 0065f380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::_Buy(count)` for 6-byte elements (thiscall, one word, `RET 4`; the result
/// in `AL`): clears `_Myfirst`, `_Mylast` and `_Myend`; 0 for a count of 0. When `max_size()`
/// ([`fn_0065f580`]) is below the count it calls the length error `0065f640` (no arguments; it
/// does not return in the game, the code after it returns 1); otherwise it allocates `count`
/// elements (`0065f6c0`, thiscall on the allocator) into `_Myfirst` and `_Mylast` and sets
/// `_Myend` to `_Myfirst + count * 6`. Returns 1.
pub fn fn_0065f380(e: &mut Engine, this: Ptr, count: u32) -> bool {
    let base = this.addr();
    e.mem.set_u32(base + VECTOR_FIRST, 0);
    e.mem.set_u32(base + VECTOR_LAST, 0);
    e.mem.set_u32(base + VECTOR_END, 0);
    if count == 0 {
        return false;
    }
    if fn_0065f580(e, this) < count {
        e.call(0x0065_f640, &args![]);
    } else {
        let block = e
            .call(0x0065_f6c0, &args![base + VECTOR_ALLOCATOR, count])
            .u32();
        e.mem.set_u32(base + VECTOR_FIRST, block);
        let first = e.mem.u32(base + VECTOR_FIRST);
        e.mem.set_u32(base + VECTOR_LAST, first);
        let first = e.mem.u32(base + VECTOR_FIRST);
        e.mem
            .set_u32(base + VECTOR_END, count.wrapping_mul(6).wrapping_add(first));
    }
    true
}

// Translated from 0065f400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the second 6-byte vector's `_Destroy(first, last)` (thiscall, two words, `RET 8`):
/// [`fn_0065f9a0`] with the range and the allocator at `+8`.
pub fn fn_0065f400(e: &mut Engine, this: Ptr, first: u32, last: u32) {
    fn_0065f9a0(e, first, last, this.addr() + VECTOR_ALLOCATOR);
}

// Translated from 0065f430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::_Tidy()` for 6-byte elements (thiscall): when `_Myfirst` is set,
/// destroys `[_Myfirst, _Mylast)` ([`fn_0065f400`]) and deallocates the buffer
/// (`0064df10(allocator, _Myfirst, (_Myend - _Myfirst) / 6)`, signed division); then clears the
/// three pointers.
pub fn fn_0065f430(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + VECTOR_FIRST) != 0 {
        let first = e.mem.u32(base + VECTOR_FIRST);
        let last = e.mem.u32(base + VECTOR_LAST);
        fn_0065f400(e, this, first, last);
        let bytes = e
            .mem
            .u32(base + VECTOR_END)
            .wrapping_sub(e.mem.u32(base + VECTOR_FIRST));
        let count = (bytes as i32).wrapping_div(6) as u32;
        let first = e.mem.u32(base + VECTOR_FIRST);
        e.call(0x0064_df10, &args![base + VECTOR_ALLOCATOR, first, count]);
    }
    e.mem.set_u32(base + VECTOR_FIRST, 0);
    e.mem.set_u32(base + VECTOR_LAST, 0);
    e.mem.set_u32(base + VECTOR_END, 0);
}

// Translated from 0065f550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::end()` (thiscall, one word: the iterator it fills; `RET 4`): the checked
/// iterator constructor `0065e000` (thiscall on the result) with `_Mylast` and the vector.
/// Returns the iterator.
pub fn fn_0065f550(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let last = e.mem.u32(this.addr() + VECTOR_LAST);
    e.call(0x0065_e000, &args![out, last, this]);
    out
}

// Translated from 0065f7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the byte allocator's `allocate(count)` (thiscall, one word, `RET 4`; `this` is not
/// read): [`fn_0065fa30`] with the count and 0, its result returned.
pub fn fn_0065f7a0(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    fn_0065fa30(e, count, 0)
}

// Translated from 0065f7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the byte allocator's `max_size()` (thiscall): the local `0xffffffff`, or 1 when
/// that is 0 (the compiler kept the test; it always returns `0xffffffff`).
pub fn fn_0065f7f0(_e: &mut Engine, _this: Ptr) -> u32 {
    let count = std::hint::black_box(0xffff_ffffu32);
    if count > 0 {
        count
    } else {
        1
    }
}

/// `_Destroy_range(first, last, allocator)` for the 6-byte vectors (cdecl, three words): the
/// iterator tag from `0065e750` (`ECX` the address of the `first` argument word), then
/// `destroy` (cdecl) with the range, the allocator and the tag byte.
fn destroy_range(
    e: &mut Engine,
    first: u32,
    last: u32,
    allocator: u32,
    destroy: &dyn Fn(&mut Engine, u32, u32, u32, u32),
) {
    e.with_stack(8, |e, arguments| {
        let (first_at, last_at) = (arguments.addr(), arguments.addr() + 4);
        e.mem.set_u32(first_at, first);
        e.mem.set_u32(last_at, last);
        let tag = e
            .call(ITERATOR_TAG_0065E750, &args![first_at, first_at, last_at])
            .u32() as u8;
        let last = e.mem.u32(last_at);
        let first = e.mem.u32(first_at);
        destroy(e, first, last, allocator, tag as u32);
    });
}

// Translated from 0065f960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Destroy_range` for the first 6-byte vector (cdecl, three words; see
/// [`destroy_range`]): its range destructor is the empty stub `0040fbe0` (four words).
pub fn fn_0065f960(e: &mut Engine, first: u32, last: u32, allocator: u32) {
    destroy_range(
        e,
        first,
        last,
        allocator,
        &|e, first, last, allocator, tag| {
            e.call(0x0040_fbe0, &args![first, last, allocator, tag]);
        },
    );
}

// Translated from 0065f9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Destroy_range` for the second 6-byte vector (cdecl, three words; see
/// [`destroy_range`]): its range destructor is [`fn_0065ff00`].
pub fn fn_0065f9a0(e: &mut Engine, first: u32, last: u32, allocator: u32) {
    destroy_range(
        e,
        first,
        last,
        allocator,
        &|e, first, last, allocator, tag| {
            fn_0065ff00(e, first, last, allocator, tag);
        },
    );
}

/// `std::_Allocate(count, T*)` for one element size: a count of 0 stays 0; when `0xffffffff /
/// count` is below the element size it constructs a `std::bad_alloc` (12 bytes on the stack,
/// message 0) and throws it (which does not return in the game; the code after it is the
/// allocation). Returns `operator new(bytes)`, `bytes` being what `size` makes of the count.
fn allocate_elements(e: &mut Engine, count: u32, element: u32, size: fn(u32) -> u32) -> u32 {
    if count != 0 && 0xffff_ffff / count < element {
        e.with_stack(0xc, |e, exception| {
            e.call(BAD_ALLOC_CTOR_0044D6C0, &args![exception, 0u32]);
            e.call(
                CXX_THROW_EXCEPTION,
                &args![exception, BAD_ALLOC_THROW_INFO_01140EF8],
            );
        });
    }
    e.call(OPERATOR_NEW, &args![size(count)]).u32()
}

// Translated from 0065f9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `std::_Allocate` for 6-byte elements (cdecl, two words; the second is not read):
/// see [`allocate_elements`]; the size is `count * 6` (`IMUL`).
pub fn fn_0065f9e0(e: &mut Engine, count: u32, _unused_2: u32) -> u32 {
    allocate_elements(e, count, 6, |count| count.wrapping_mul(6))
}

// Translated from 0065fa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `std::_Allocate` for bytes (cdecl, two words; the second is not read): see
/// [`allocate_elements`] with element size 1 (the throw is reached by no count); the size is
/// the count.
pub fn fn_0065fa30(e: &mut Engine, count: u32, _unused_2: u32) -> u32 {
    allocate_elements(e, count, 1, |count| count)
}

// Translated from 0065fab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the copy constructor of a 0x58-byte record (thiscall, one word: the source; `RET
/// 4`): copies the float at `+0` (`FLD`/`FSTP`) and the words at `+4`, `+8` and `+0xc`, then
/// copy-constructs the three 0x18-byte vectors at `+0x10` with the `eh vector copy constructor
/// iterator` (`00ecaebb`, stdcall, six words: destination, source, 0x18, 3, the copy
/// constructor [`fn_0065fc10`] and the destructor `0065f060`). Returns `this`.
pub fn fn_0065fab0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let base = this.addr();
    let from = source.addr();
    let value = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(base, value);
    for offset in [4u32, 8, 0xc] {
        let word = e.mem.u32(from + offset);
        e.mem.set_u32(base + offset, word);
    }
    e.call(
        0x00ec_aebb,
        &args![
            base + 0x10,
            from + 0x10,
            0x18u32,
            3u32,
            0x0065_fc10u32,
            0x0065_f060u32
        ],
    );
    this
}

/// The vector copy constructor shape the two copy constructors below share: the allocator is copied into a
/// stack word (`0065de30` on the word, which `PUSH ECX` set to `this`) and passed by value to
/// the base constructor `0065d3a0`; the size of `other` (`size`, thiscall) is bought (`buy`,
/// thiscall, result in `AL`), and when that succeeds `copy` (thiscall, five words: `begin()`
/// and `end()` of `other` as two-word iterators from [`fn_0065fd10`] and [`fn_0065fd40`], then
/// `_Myfirst`) fills the buffer and its result becomes `_Mylast`. Returns `this`.
fn vector_copy(
    e: &mut Engine,
    this: Ptr,
    other: Ptr,
    size: u32,
    buy: &dyn Fn(&mut Engine, Ptr, u32) -> bool,
    copy: u32,
) -> Ptr {
    let base = this.addr();
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), base);
        e.call(0x0065_de30, &args![slot, other.addr() + VECTOR_ALLOCATOR]);
        let word = e.mem.u32(slot.addr());
        e.call(0x0065_d3a0, &args![base, word]);
    });
    let count = e.call(size, &args![other]).u32();
    if buy(e, this, count) {
        e.with_stack(0x10, |e, iterators| {
            // `begin()` at +0 (EBP-0x24), `end()` at +8 (EBP-0x1c).
            let end = fn_0065fd40(e, other, Ptr::new(iterators.addr() + 8)).addr();
            let begin = fn_0065fd10(e, other, iterators).addr();
            let first = e.mem.u32(base + VECTOR_FIRST);
            let (begin_base, begin_pointer) = (e.mem.u32(begin), e.mem.u32(begin + 4));
            let (end_base, end_pointer) = (e.mem.u32(end), e.mem.u32(end + 4));
            let result = e
                .call(
                    copy,
                    &args![
                        base,
                        begin_base,
                        begin_pointer,
                        end_base,
                        end_pointer,
                        first
                    ],
                )
                .u32();
            e.mem.set_u32(base + VECTOR_LAST, result);
        });
    }
    this
}

// Translated from 0065fb10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065fb10 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and its catch block (`_Tidy` and rethrow) are not translated
/// Unnamed: the first 6-byte vector's copy constructor (thiscall, one word, `RET 4`): see
/// [`vector_copy`] with `size` `00657bc0`, `buy` [`fn_0065f380`] and `copy` `0065ff30`.
pub fn fn_0065fb10(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    vector_copy(e, this, other, 0x0065_7bc0, &fn_0065f380, 0x0065_ff30)
}

// Translated from 0065fc10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065fc10 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and its catch block (`_Tidy` and rethrow) are not translated
/// Unnamed: the copy constructor of the vector in the 0x18-byte slots of [`fn_0065fab0`]
/// (thiscall, one word, `RET 4`): see [`vector_copy`] with `size` `0065cc50`, `buy` `00657ee0`
/// (called by address) and `copy` `0065ff60`.
pub fn fn_0065fc10(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    vector_copy(
        e,
        this,
        other,
        0x0065_cc50,
        &|e, this, count| e.call(0x0065_7ee0, &args![this, count]).bool(),
        0x0065_ff60,
    )
}

// Translated from 0065fd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::begin()` (thiscall, one word: the iterator it fills; `RET 4`): the checked
/// iterator constructor `0065da80` (thiscall on the result) with `_Myfirst` and the vector.
pub fn fn_0065fd10(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let first = e.mem.u32(this.addr() + VECTOR_FIRST);
    e.call(0x0065_da80, &args![out, first, this]);
    out
}

// Translated from 0065fd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::end()` (thiscall, one word: the iterator it fills; `RET 4`): the checked
/// iterator constructor `0065da80` (thiscall on the result) with `_Mylast` and the vector.
pub fn fn_0065fd40(e: &mut Engine, this: Ptr, out: Ptr) -> Ptr {
    let last = e.mem.u32(this.addr() + VECTOR_LAST);
    e.call(0x0065_da80, &args![out, last, this]);
    out
}

// Translated from 0065fd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl; its callers push three words): returns its first argument.
pub fn fn_0065fd70(_e: &mut Engine, first: u32, _unused_2: u32, _unused_3: u32) -> u32 {
    first
}

// Translated from 0065fe40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: unwraps a checked iterator (cdecl, one word: the iterator's address): `0043b1d0`
/// with the address and the byte at `EBP-1` (the top byte of the caller's `ECX`, which the
/// `PUSH ECX` of the frame left there); `0043b1d0` reads only its first word, so the byte is
/// passed as 0 here. Returns `0043b1d0`'s result.
pub fn fn_0065fe40(e: &mut Engine, iterator: u32) -> u32 {
    e.call(UNWRAP_0043B1D0, &args![iterator, 0u32]).u32()
}

/// The shape of the checked uninitialized copy and copy wrappers (cdecl, four words: first,
/// last, destination, allocator; a frame of four local bytes below `EBP`). The byte at `EBP-1`
/// is set to 0; the iterator tag comes from `0065e750` with `ECX` the address of the
/// destination word (pushed: the addresses of the first and destination words, or with
/// `one_push` only the destination's, the next stack word being the frame's local) and is
/// kept at `EBP-2`; last and first are unwrapped ([`fn_0065fe40`]); then `next` runs with the
/// unwrapped first, last, the destination, the allocator, the tag and the zero byte.
fn checked_forward(
    e: &mut Engine,
    words: [u32; 4],
    one_push: bool,
    next: fn(&mut Engine, u32, u32, u32, u32, u32, u32) -> u32,
) -> u32 {
    e.with_stack(0x1c, |e, frame| {
        // EBP is frame + 4: the local at frame + 0..4, the arguments from EBP + 8.
        let ebp = frame.addr() + 4;
        let (first_at, last_at, destination_at, allocator_at) =
            (ebp + 8, ebp + 0xc, ebp + 0x10, ebp + 0x14);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(first_at + 4 * i as u32, *word);
        }
        e.mem.set_u8(ebp - 1, 0);
        let tag = if one_push {
            let local = e.mem.u32(ebp - 4);
            e.call(
                ITERATOR_TAG_0065E750,
                &args![destination_at, destination_at, local],
            )
        } else {
            e.call(
                ITERATOR_TAG_0065E750,
                &args![destination_at, first_at, destination_at],
            )
        }
        .u32() as u8;
        e.mem.set_u8(ebp - 2, tag);
        let zero = e.mem.u8(ebp - 1) as u32;
        let tag = e.mem.u8(ebp - 2) as u32;
        let allocator = e.mem.u32(allocator_at);
        let destination = e.mem.u32(destination_at);
        let last = fn_0065fe40(e, last_at);
        let first = fn_0065fe40(e, first_at);
        next(e, first, last, destination, allocator, tag, zero)
    })
}

// Translated from 0065fde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Uninitialized_copy` for the first 6-byte vector (cdecl, four words): see
/// [`checked_forward`] (two pushes), then [`fn_00660070`].
pub fn fn_0065fde0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    checked_forward(e, [first, last, destination, allocator], false, fn_00660070)
}

// Translated from 0065fea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the plain-data copy wrapper for the second 6-byte vector (cdecl, four words): see
/// [`checked_forward`] (two pushes), then [`fn_00660270`].
pub fn fn_0065fea0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    checked_forward(e, [first, last, destination, allocator], false, fn_00660270)
}

// Translated from 0065ff00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the range destructor of the second 6-byte vector (cdecl; its caller pushes four
/// words, it reads three): `0065e9a0` (thiscall on the allocator, one word) on each element
/// of `[first, last)`.
pub fn fn_0065ff00(e: &mut Engine, first: u32, last: u32, allocator: u32, _unused_4: u32) {
    let mut element = first;
    while element != last {
        e.call(0x0065_e9a0, &args![allocator, element]);
        element = element.wrapping_add(6);
    }
}

// Translated from 0065ff90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a checked uninitialized copy wrapper (cdecl, four words): see [`checked_forward`]
/// (one push), then [`fn_00660210`].
pub fn fn_0065ff90(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    checked_forward(e, [first, last, destination, allocator], true, fn_00660210)
}

// Translated from 0065ffe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a checked uninitialized copy wrapper (cdecl, four words): see [`checked_forward`]
/// (one push), then [`fn_00660230`].
pub fn fn_0065ffe0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    checked_forward(e, [first, last, destination, allocator], true, fn_00660230)
}

/// The element loop of `_Uninitialized_copy` for one element size (cdecl, six words; the tag
/// words are not read): `construct` (thiscall on the allocator, two words: the destination and
/// the source element) for each element, both pointers advancing by the element size. Returns
/// the destination's end.
fn construct_each(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    allocator: u32,
    element: u32,
    construct: u32,
) -> u32 {
    let (mut source, mut target) = (first, destination);
    while source != last {
        e.call(construct, &args![allocator, target, source]);
        target = target.wrapping_add(element);
        source = source.wrapping_add(element);
    }
    target
}

// Translated from 00660070 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00660070 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and its catch block (destroy the built part and rethrow) are not translated
/// Unnamed: `_Uninitialized_copy` for 6-byte elements (cdecl, six words): see
/// [`construct_each`] with `006601f0`.
pub fn fn_00660070(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    allocator: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    construct_each(e, first, last, destination, allocator, 6, 0x0066_01f0)
}

// Translated from 00660210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, six words; the last two are not read): [`fn_00660340`] with the first four,
/// its result returned.
pub fn fn_00660210(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    allocator: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    fn_00660340(e, first, last, destination, allocator)
}

// Translated from 00660230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (cdecl, six words; the last two are not read): [`fn_006603a0`] with the first four,
/// its result returned.
pub fn fn_00660230(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    allocator: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    fn_006603a0(e, first, last, destination, allocator)
}

// Translated from 00660270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the plain-data copy (cdecl, six words; it reads the first three): `memmove_s`
/// (`00ec7ce3`, cdecl: destination, size, source, count) of the `last - first` bytes when that
/// is not 0. Returns `destination + (last - first)`.
pub fn fn_00660270(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    let bytes = last.wrapping_sub(first);
    let end = destination.wrapping_add(bytes);
    if bytes > 0 {
        e.call(0x00ec_7ce3, &args![destination, bytes, first, bytes]);
    }
    end
}

// Translated from 00660340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Uninitialized_copy` for 0x1c-byte elements (cdecl, four words): see
/// [`checked_forward`] (two pushes), then [`fn_00660510`].
pub fn fn_00660340(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    checked_forward(e, [first, last, destination, allocator], false, fn_00660510)
}

// Translated from 006603a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Uninitialized_copy` for 0x58-byte elements (cdecl, four words): see
/// [`checked_forward`] (two pushes), then [`fn_006605d0`].
pub fn fn_006603a0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    checked_forward(e, [first, last, destination, allocator], false, fn_006605d0)
}

// Translated from 00660510 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00660510 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and its catch block (destroy the built part and rethrow) are not translated
/// Unnamed: `_Uninitialized_copy` for 0x1c-byte elements (cdecl, six words): see
/// [`construct_each`] with `00660300`.
pub fn fn_00660510(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    allocator: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    construct_each(e, first, last, destination, allocator, 0x1c, 0x0066_0300)
}

// Translated from 006605d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006605d0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) and its catch block (destroy the built part and rethrow) are not translated
/// Unnamed: `_Uninitialized_copy` for 0x58-byte elements (cdecl, six words): see
/// [`construct_each`] with `00660320`.
pub fn fn_006605d0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    allocator: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    construct_each(e, first, last, destination, allocator, 0x58, 0x0066_0320)
}

/// The checked iterator constructor (`0065ded0`, thiscall on the vector: the out iterator,
/// the owner and the pointer) and comparison (`0065dfd0`, thiscall on an iterator, one word:
/// the other; result in `AL`).
const MAKE_ITERATOR_0065DED0: u32 = 0x0065_ded0;
const ITERATORS_DIFFER_0065DFD0: u32 = 0x0065_dfd0;
/// `vector::begin()` of the 6-byte vectors (`0044a170`, thiscall, one word: the out
/// iterator; answers its address).
const BEGIN_0044A170: u32 = 0x0044_a170;
/// The string from a C string (`0044a4c0`, thiscall, one word), `std::length_error`'s
/// constructor (`0044a2e0`, thiscall, one word: the message string), the message
/// (`vector<T> too long`) and `std::length_error`'s throw information.
const STRING_FROM_C_0044A4C0: u32 = 0x0044_a4c0;
const LENGTH_ERROR_CTOR_0044A2E0: u32 = 0x0044_a2e0;
const VECTOR_TOO_LONG: u32 = 0x0101_73c4;
const LENGTH_ERROR_THROW_INFO_01140CB0: u32 = 0x0114_0cb0;
/// `memmove_s` (`00ec7ce3`, cdecl: destination, size, source, count).
const MEMMOVE_S: u32 = 0x00ec_7ce3;
/// Placement `operator new` (`006e6da0`, cdecl: size, place).
const PLACEMENT_NEW_006E6DA0: u32 = 0x006e_6da0;
/// The word at +4 of an object (`00726070`, thiscall).
const WORD_AT_FOUR_00726070: u32 = 0x0072_6070;

/// A copy's next step (cdecl, six words: first, last, destination and three tags).
type CopyStep = fn(&mut Engine, u32, u32, u32, u32, u32, u32) -> u32;

// Translated from 0065f2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::capacity()` for the 6-byte vector (thiscall): 0 without `_Myfirst`,
/// else `(_Myend - _Myfirst) / 6` (signed division).
pub fn fn_0065f2e0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    if e.mem.u32(base + VECTOR_FIRST) == 0 {
        return 0;
    }
    let bytes = e
        .mem
        .u32(base + VECTOR_END)
        .wrapping_sub(e.mem.u32(base + VECTOR_FIRST));
    (bytes as i32).wrapping_div(6) as u32
}

// Translated from 0065f320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::clear()` for the second 6-byte vector (thiscall): [`fn_0065f5a0`] with
/// `begin()` (`0044a170`) and `end()` ([`fn_0065f550`]), the result iterator a stack
/// local.
pub fn fn_0065f320(e: &mut Engine, this: Ptr) {
    e.with_stack(0x2c, |e, frame| {
        let ebp = frame.addr() + 0x2c;
        let at = fn_0065f550(e, this, Ptr::new(ebp - 0x10)).addr();
        let (end_owner, end) = (e.mem.u32(at), e.mem.u32(at + 4));
        let at = e.call(BEGIN_0044A170, &args![this, ebp - 0x20]).u32();
        let (begin_owner, begin) = (e.mem.u32(at), e.mem.u32(at.wrapping_add(4)));
        fn_0065f5a0(
            e,
            this,
            Ptr::new(ebp - 0x28),
            begin_owner,
            begin,
            end_owner,
            end,
        );
    });
}

// Translated from 0065f4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::capacity()` for a byte vector (thiscall): 0 without `_Myfirst`, else
/// `_Myend - _Myfirst`.
pub fn fn_0065f4b0(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    if e.mem.u32(base + VECTOR_FIRST) == 0 {
        return 0;
    }
    e.mem
        .u32(base + VECTOR_END)
        .wrapping_sub(e.mem.u32(base + VECTOR_FIRST))
}

// Translated from 0065f4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::clear()` for the first 6-byte vector (thiscall): [`fn_0065f700`] with
/// `begin()` (`0044a170`) and `end()` ([`fn_0065f550`]), the result iterator a stack
/// local.
pub fn fn_0065f4f0(e: &mut Engine, this: Ptr) {
    e.with_stack(0x2c, |e, frame| {
        let ebp = frame.addr() + 0x2c;
        let at = fn_0065f550(e, this, Ptr::new(ebp - 0x10)).addr();
        let (end_owner, end) = (e.mem.u32(at), e.mem.u32(at + 4));
        let at = e.call(BEGIN_0044A170, &args![this, ebp - 0x20]).u32();
        let (begin_owner, begin) = (e.mem.u32(at), e.mem.u32(at.wrapping_add(4)));
        fn_0065f700(
            e,
            this,
            Ptr::new(ebp - 0x28),
            begin_owner,
            begin,
            end_owner,
            end,
        );
    });
}

/// `vector::erase(first, last)` for one 6-byte vector (thiscall; `out` the result iterator,
/// the two iterators as two words each): both iterators are rebuilt (`0065ded0`); when they
/// differ (`0065dfd0`), the tail from `last` is copied down to `first` (`copy`), the elements
/// after the new end destroyed (`destroy`) and `_Mylast` moved. The result is the first
/// iterator.
fn erase_range(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    first: [u32; 2],
    last: [u32; 2],
    copy: fn(&mut Engine, u32, u32, u32) -> u32,
    destroy: fn(&mut Engine, Ptr, u32, u32),
) -> Ptr {
    let base = this.addr();
    e.with_stack(0x10, |e, frame| {
        let (from, to) = (frame.addr() + 8, frame.addr());
        e.call(
            MAKE_ITERATOR_0065DED0,
            &args![this, from, first[0], first[1]],
        );
        e.call(MAKE_ITERATOR_0065DED0, &args![this, to, last[0], last[1]]);
        if e.call(ITERATORS_DIFFER_0065DFD0, &args![from, to]).bool() {
            let old_last = e.mem.u32(base + VECTOR_LAST);
            let (source, target) = (e.mem.u32(to + 4), e.mem.u32(from + 4));
            let new_last = copy(e, source, old_last, target);
            let old_last = e.mem.u32(base + VECTOR_LAST);
            destroy(e, this, new_last, old_last);
            e.mem.set_u32(base + VECTOR_LAST, new_last);
        }
        let (owner, pointer) = (e.mem.u32(from), e.mem.u32(from + 4));
        e.mem.set_u32(out.addr(), owner);
        e.mem.set_u32(out.addr() + 4, pointer);
    });
    out
}

// Translated from 0065f5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::erase(first, last)` for the second 6-byte vector (thiscall, five words:
/// the result iterator's address and the two iterators, `RET 0x14`): see [`erase_range`]
/// with the copy [`fn_0065f820`] and [`fn_0065f400`]. Answers the result's address.
pub fn fn_0065f5a0(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    first_owner: u32,
    first: u32,
    last_owner: u32,
    last: u32,
) -> Ptr {
    erase_range(
        e,
        this,
        out,
        [first_owner, first],
        [last_owner, last],
        fn_0065f820,
        fn_0065f400,
    )
}

// Translated from 0065f640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::_Xlen()` (no arguments): makes the string `vector<T> too long`
/// (`0044a4c0`), a `std::length_error` from it (`0044a2e0`) and throws it
/// (`_CxxThrowException`, which does not return in the game).
// Translation incomplete: 0065f640 - the C++ exception-unwinding frame (FS:[0]) that destroys the message string is not translated
pub fn fn_0065f640(e: &mut Engine) {
    e.with_stack(0x50, |e, frame| {
        let ebp = frame.addr() + 0x50;
        let (message, error) = (ebp - 0x50, ebp - 0x34);
        e.call(STRING_FROM_C_0044A4C0, &args![message, VECTOR_TOO_LONG]);
        e.call(LENGTH_ERROR_CTOR_0044A2E0, &args![error, message]);
        e.call(
            CXX_THROW_EXCEPTION,
            &args![error, LENGTH_ERROR_THROW_INFO_01140CB0],
        );
    });
}

// Translated from 0065f6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the 6-byte allocator's `allocate(count)` (thiscall, one word, `RET 4`; `this` is
/// not read): [`fn_0065f9e0`] with the count and 0.
pub fn fn_0065f6c0(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    fn_0065f9e0(e, count, 0)
}

// Translated from 0065f700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::erase(first, last)` for the first 6-byte vector (thiscall, five words,
/// `RET 0x14`): see [`erase_range`] with the copy [`fn_0065f8c0`] and [`fn_0065f2b0`].
/// Answers the result's address.
pub fn fn_0065f700(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    first_owner: u32,
    first: u32,
    last_owner: u32,
    last: u32,
) -> Ptr {
    erase_range(
        e,
        this,
        out,
        [first_owner, first],
        [last_owner, last],
        fn_0065f8c0,
        fn_0065f2b0,
    )
}

// Translated from 0065f7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the 6-byte allocator's `max_size()` (thiscall): the local `0x2aaaaaaa`, or 1 when
/// that is 0 (the compiler kept the test; it always returns `0x2aaaaaaa`).
pub fn fn_0065f7c0(_e: &mut Engine, _this: Ptr) -> u32 {
    let count = std::hint::black_box(0x2aaa_aaaau32);
    if count > 0 {
        count
    } else {
        1
    }
}

/// `std::copy(first, last, destination)` of the checked library for one element type
/// (cdecl, three words): the pointer category tag (`0065e750` with `ECX` the address of the
/// destination word, the addresses of the first and destination words pushed), the iterator
/// category ([`fn_0065fd70`] with the category byte's address), the unchecked pointers of
/// `last` and `first` ([`fn_0065fe40`]), then `next` with them, the destination, the
/// category, the tag and the range-checked tag 0.
fn checked_copy(e: &mut Engine, first: u32, last: u32, destination: u32, next: CopyStep) -> u32 {
    e.with_stack(0x10, |e, frame| {
        // The local word at EBP-4 holds the bytes at EBP-3 (category), EBP-2 (tag) and EBP-1
        // (the range-checked tag); the argument words follow it.
        let local = frame.addr();
        let (category, tag, checked) = (local + 1, local + 2, local + 3);
        let (first_at, last_at, destination_at) = (local + 4, local + 8, local + 0xc);
        e.mem.set_u32(first_at, first);
        e.mem.set_u32(last_at, last);
        e.mem.set_u32(destination_at, destination);
        e.mem.set_u8(checked, 0);
        let pointer_tag = e
            .call(
                ITERATOR_TAG_0065E750,
                &args![destination_at, first_at, destination_at],
            )
            .u32() as u8;
        e.mem.set_u8(tag, pointer_tag);
        let checked = u32::from(e.mem.u8(checked));
        let pointer_tag = u32::from(e.mem.u8(tag));
        let at = fn_0065fd70(e, category, first_at, destination_at);
        let iterator_tag = u32::from(e.mem.u8(at));
        let destination = e.mem.u32(destination_at);
        let last = fn_0065fe40(e, last_at);
        let first = fn_0065fe40(e, first_at);
        next(
            e,
            first,
            last,
            destination,
            iterator_tag,
            pointer_tag,
            checked,
        )
    })
}

// Translated from 0065f820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `std::copy` for the second 6-byte vector (cdecl, three words): see
/// [`checked_copy`], then [`fn_0065fd80`].
pub fn fn_0065f820(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    checked_copy(e, first, last, destination, fn_0065fd80)
}

// Translated from 0065f890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::_Ucopy(first, last, destination)` for the first 6-byte vector
/// (thiscall, three words, `RET 0xc`): [`fn_0065fde0`] with them and the allocator at `+8`.
pub fn fn_0065f890(e: &mut Engine, this: Ptr, first: u32, last: u32, destination: u32) -> u32 {
    fn_0065fde0(e, first, last, destination, this.addr() + VECTOR_ALLOCATOR)
}

// Translated from 0065f8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `std::copy` for the first 6-byte vector (cdecl, three words): see
/// [`checked_copy`], then [`fn_0065fe60`].
pub fn fn_0065f8c0(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    checked_copy(e, first, last, destination, fn_0065fe60)
}

// Translated from 0065f930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::_Ucopy(first, last, destination)` for the second 6-byte vector
/// (thiscall, three words, `RET 0xc`): [`fn_0065fea0`] with them and the allocator at `+8`.
pub fn fn_0065f930(e: &mut Engine, this: Ptr, first: u32, last: u32, destination: u32) -> u32 {
    fn_0065fea0(e, first, last, destination, this.addr() + VECTOR_ALLOCATOR)
}

// Translated from 0065fa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the copy constructor of a 0x1c-byte record (thiscall, one word: the source,
/// `RET 4`): copies the float at `+0` (`FLD`/`FSTP`), then copy-constructs the first 6-byte
/// vector at `+4` ([`fn_0065fb10`]). Answers `this`.
pub fn fn_0065fa80(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let value = x87_quiet_bits(e.mem.u32(source.addr()));
    e.mem.set_u32(this.addr(), value);
    fn_0065fb10(e, Ptr::new(this.addr() + 4), Ptr::new(source.addr() + 4));
    this
}

// Translated from 0065fd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the 6-byte copy's random-access, non-scalar form (`cdecl`, six words: first,
/// last, destination and three tag words that are not read): copies the elements
/// ([`fn_00660030`] with the destination's unchecked pointer, [`fn_0065fe40`]) and answers
/// `destination + (last - first) / 6 * 6` (signed division). The exe passes on a tag byte of
/// its own frame that it never sets (not read by the copy); here the frame starts zeroed.
pub fn fn_0065fd80(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let count = (last.wrapping_sub(first) as i32).wrapping_div(6);
    let end = (count.wrapping_mul(6) as u32).wrapping_add(destination);
    e.with_stack(8, |e, frame| {
        let (destination_at, unset_tag) = (frame.addr(), frame.addr() + 4);
        e.mem.set_u32(destination_at, destination);
        let unset = u32::from(e.mem.u8(unset_tag));
        let target = fn_0065fe40(e, destination_at);
        fn_00660030(e, first, last, target, unset, 0, 0);
    });
    end
}

// Translated from 0065fe60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the first 6-byte vector's plain-data copy (`cdecl`, six words; it reads the first
/// three): `memmove_s` (`00ec7ce3`: destination, size, source, count) of the `last - first`
/// bytes when that is above 0 (signed). Answers `destination + (last - first)`.
pub fn fn_0065fe60(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let bytes = last.wrapping_sub(first);
    let end = destination.wrapping_add(bytes);
    if bytes as i32 > 0 {
        e.call(MEMMOVE_S, &args![destination, bytes, first, bytes]);
    }
    end
}

// Translated from 0065ff30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `vector::_Ucopy(first, last, destination)` with checked iterators for the first
/// 6-byte vector (the copy of [`fn_0065fb10`]; thiscall, five words: the two iterators as two
/// words each and the destination, `RET 0x14`): [`fn_00660130`] with them and the allocator
/// at `+8`.
pub fn fn_0065ff30(
    e: &mut Engine,
    this: Ptr,
    first_owner: u32,
    first: u32,
    last_owner: u32,
    last: u32,
    destination: u32,
) -> u32 {
    let allocator = this.addr() + VECTOR_ALLOCATOR;
    fn_00660130(
        e,
        first_owner,
        first,
        last_owner,
        last,
        destination,
        allocator,
    )
}

// Translated from 0065ff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the same for the vector [`fn_0065fc10`] copies (thiscall, five words, `RET
/// 0x14`): [`fn_00660190`] with them and the allocator at `+8`.
pub fn fn_0065ff60(
    e: &mut Engine,
    this: Ptr,
    first_owner: u32,
    first: u32,
    last_owner: u32,
    last: u32,
    destination: u32,
) -> u32 {
    let allocator = this.addr() + VECTOR_ALLOCATOR;
    fn_00660190(
        e,
        first_owner,
        first,
        last_owner,
        last,
        destination,
        allocator,
    )
}

// Translated from 00660030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the 6-byte element copy loop (`cdecl`, six words; the three tags are not read):
/// copies each element (a word and a 16-bit word) from `first` up to `last` to `destination`
/// on; answers the end of the destination.
pub fn fn_00660030(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let (mut source, mut target) = (first, destination);
    while source != last {
        let word = e.mem.u32(source);
        e.mem.set_u32(target, word);
        let half = e.mem.u16(source.wrapping_add(4));
        e.mem.set_u16(target.wrapping_add(4), half);
        target = target.wrapping_add(6);
        source = source.wrapping_add(6);
    }
    target
}

/// The checked uninitialized copy with checked iterators (cdecl, six words: the first and
/// last iterators as two words each, the destination and the allocator): the pointer
/// category tag (`0065e750` with `ECX` the address of the destination word, the addresses of
/// the first iterator and the destination pushed), the unwrapped last and first iterators
/// ([`fn_00660250`]), then `next` with them, the destination, the allocator, the tag and the
/// range-checked tag 0.
fn checked_record_copy(e: &mut Engine, words: [u32; 6], next: CopyStep) -> u32 {
    e.with_stack(0x1c, |e, frame| {
        // The local word at EBP-4 holds the tag (EBP-2) and the range-checked tag (EBP-1);
        // the argument words follow it.
        let local = frame.addr();
        let (tag, checked) = (local + 2, local + 3);
        let (first_at, last_at) = (local + 4, local + 0xc);
        let (destination_at, allocator_at) = (local + 0x14, local + 0x18);
        for (index, word) in words.iter().enumerate() {
            e.mem.set_u32(first_at + 4 * index as u32, *word);
        }
        e.mem.set_u8(checked, 0);
        let pointer_tag = e
            .call(
                ITERATOR_TAG_0065E750,
                &args![destination_at, first_at, destination_at],
            )
            .u32() as u8;
        e.mem.set_u8(tag, pointer_tag);
        let checked = u32::from(e.mem.u8(checked));
        let pointer_tag = u32::from(e.mem.u8(tag));
        let allocator = e.mem.u32(allocator_at);
        let destination = e.mem.u32(destination_at);
        let last = fn_00660250(e, last_at);
        let first = fn_00660250(e, first_at);
        next(e, first, last, destination, allocator, pointer_tag, checked)
    })
}

// Translated from 00660130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the checked uninitialized copy of the first 6-byte vector (cdecl, six words): see
/// [`checked_record_copy`], then [`fn_00660070`] (each element constructed).
pub fn fn_00660130(
    e: &mut Engine,
    first_owner: u32,
    first: u32,
    last_owner: u32,
    last: u32,
    destination: u32,
    allocator: u32,
) -> u32 {
    let words = [first_owner, first, last_owner, last, destination, allocator];
    checked_record_copy(e, words, fn_00660070)
}

// Translated from 00660190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the checked uninitialized copy of the vector [`fn_0065fc10`] copies (cdecl, six
/// words): see [`checked_record_copy`], then [`fn_00660270`] (the bytes moved).
pub fn fn_00660190(
    e: &mut Engine,
    first_owner: u32,
    first: u32,
    last_owner: u32,
    last: u32,
    destination: u32,
    allocator: u32,
) -> u32 {
    let words = [first_owner, first, last_owner, last, destination, allocator];
    checked_record_copy(e, words, fn_00660270)
}

// Translated from 006601f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the 6-byte allocator's `construct(place, value)` (thiscall, two words, `RET 8`;
/// `this` is not read): [`fn_006602b0`] with them.
pub fn fn_006601f0(e: &mut Engine, _this: Ptr, place: u32, value: u32) {
    fn_006602b0(e, place, value);
}

// Translated from 00660250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: unwraps a checked iterator (`cdecl`, one word: the iterator's address):
/// [`fn_00660400`] with the address and the byte at `EBP-1` (the top byte of the caller's
/// `ECX`, which the frame's `PUSH ECX` left there); [`fn_00660400`] reads only its first word,
/// so the byte is passed as 0 here.
pub fn fn_00660250(e: &mut Engine, iterator: u32) -> u32 {
    fn_00660400(e, iterator, 0)
}

// Translated from 006602b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Construct(place, value)` for 6-byte elements (`cdecl`, two words): placement
/// `operator new` (`006e6da0` with 6 and the place); when that answers a place, the word and
/// 16-bit word of the value are copied there. Answers the place.
pub fn fn_006602b0(e: &mut Engine, place: u32, value: u32) -> u32 {
    let at = e.call(PLACEMENT_NEW_006E6DA0, &args![6u32, place]).u32();
    if at != 0 {
        let word = e.mem.u32(value);
        e.mem.set_u32(at, word);
        let half = e.mem.u16(value.wrapping_add(4));
        e.mem.set_u16(at.wrapping_add(4), half);
    }
    at
}

// Translated from 00660300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the 0x1c-byte record allocator's `construct(place, value)` (thiscall, two words,
/// `RET 8`; `this` is not read): [`fn_00660410`] with them.
pub fn fn_00660300(e: &mut Engine, _this: Ptr, place: u32, value: u32) {
    fn_00660410(e, place, value);
}

// Translated from 00660320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the 0x58-byte record allocator's `construct(place, value)` (thiscall, two
/// words, `RET 8`; `this` is not read): [`fn_00660490`] with them.
pub fn fn_00660320(e: &mut Engine, _this: Ptr, place: u32, value: u32) {
    fn_00660490(e, place, value);
}

// Translated from 00660400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the word an iterator points at (`cdecl`, two words; it reads only the first):
/// `00726070` (the word at +4) on the iterator's address.
pub fn fn_00660400(e: &mut Engine, iterator: u32, _unused_1: u32) -> u32 {
    e.call(WORD_AT_FOUR_00726070, &args![iterator]).u32()
}

// Translated from 00660410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Construct(place, value)` for the 0x1c-byte records (`cdecl`, two words):
/// placement `operator new` (`006e6da0` with 0x1c and the place), then the copy constructor
/// [`fn_0065fa80`] there from `value`. Answers the record (0 without a place).
// Translation incomplete: 00660410 - the C++ exception-unwinding frame (FS:[0]) that frees the place when the copy throws is not translated
pub fn fn_00660410(e: &mut Engine, place: u32, value: u32) -> u32 {
    let at = e.call(PLACEMENT_NEW_006E6DA0, &args![0x1cu32, place]).u32();
    if at == 0 {
        return 0;
    }
    fn_0065fa80(e, Ptr::new(at), Ptr::new(value)).addr()
}

// Translated from 00660490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: `_Construct(place, value)` for the 0x58-byte records (`cdecl`, two words):
/// placement `operator new` (`006e6da0` with 0x58 and the place), then the copy constructor
/// [`fn_0065fab0`] there from `value`. Answers the record (0 without a place).
// Translation incomplete: 00660490 - the C++ exception-unwinding frame (FS:[0]) that frees the place when the copy throws is not translated
pub fn fn_00660490(e: &mut Engine, place: u32, value: u32) -> u32 {
    let at = e.call(PLACEMENT_NEW_006E6DA0, &args![0x58u32, place]).u32();
    if at == 0 {
        return 0;
    }
    fn_0065fab0(e, Ptr::new(at), Ptr::new(value)).addr()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_f580, fn_0065f580(Ptr) -> u32),
        entry!(0x0065_f6e0, fn_0065f6e0(Ptr) -> u32),
        entry!(0x0065_f2b0, fn_0065f2b0(Ptr, u32, u32)),
        entry!(0x0065_f380, fn_0065f380(Ptr, u32) -> bool),
        entry!(0x0065_f400, fn_0065f400(Ptr, u32, u32)),
        entry!(0x0065_f430, fn_0065f430(Ptr)),
        entry!(0x0065_f550, fn_0065f550(Ptr, Ptr) -> Ptr),
        entry!(0x0065_f7a0, fn_0065f7a0(Ptr, u32) -> u32),
        entry!(0x0065_f7f0, fn_0065f7f0(Ptr) -> u32),
        entry!(0x0065_f960, fn_0065f960(u32, u32, u32)),
        entry!(0x0065_f9a0, fn_0065f9a0(u32, u32, u32)),
        entry!(0x0065_f9e0, fn_0065f9e0(u32, u32) -> u32),
        entry!(0x0065_fa30, fn_0065fa30(u32, u32) -> u32),
        entry!(0x0065_fab0, fn_0065fab0(Ptr, Ptr) -> Ptr),
        entry!(0x0065_fb10, fn_0065fb10(Ptr, Ptr) -> Ptr),
        entry!(0x0065_fc10, fn_0065fc10(Ptr, Ptr) -> Ptr),
        entry!(0x0065_fd10, fn_0065fd10(Ptr, Ptr) -> Ptr),
        entry!(0x0065_fd40, fn_0065fd40(Ptr, Ptr) -> Ptr),
        entry!(0x0065_fd70, fn_0065fd70(u32, u32, u32) -> u32),
        entry!(0x0065_fde0, fn_0065fde0(u32, u32, u32, u32) -> u32),
        entry!(0x0065_fe40, fn_0065fe40(u32) -> u32),
        entry!(0x0065_fea0, fn_0065fea0(u32, u32, u32, u32) -> u32),
        entry!(0x0065_ff00, fn_0065ff00(u32, u32, u32, u32)),
        entry!(0x0065_ff90, fn_0065ff90(u32, u32, u32, u32) -> u32),
        entry!(0x0065_ffe0, fn_0065ffe0(u32, u32, u32, u32) -> u32),
        entry!(0x0066_0070, fn_00660070(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0066_0210, fn_00660210(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0066_0230, fn_00660230(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0066_0270, fn_00660270(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0066_0340, fn_00660340(u32, u32, u32, u32) -> u32),
        entry!(0x0066_03a0, fn_006603a0(u32, u32, u32, u32) -> u32),
        entry!(0x0066_0510, fn_00660510(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0066_05d0, fn_006605d0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0065_f2e0, fn_0065f2e0(Ptr) -> u32),
        entry!(0x0065_f320, fn_0065f320(Ptr)),
        entry!(0x0065_f4b0, fn_0065f4b0(Ptr) -> u32),
        entry!(0x0065_f4f0, fn_0065f4f0(Ptr)),
        entry!(0x0065_f5a0, fn_0065f5a0(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x0065_f640, fn_0065f640()),
        entry!(0x0065_f6c0, fn_0065f6c0(Ptr, u32) -> u32),
        entry!(0x0065_f700, fn_0065f700(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x0065_f7c0, fn_0065f7c0(Ptr) -> u32),
        entry!(0x0065_f820, fn_0065f820(u32, u32, u32) -> u32),
        entry!(0x0065_f890, fn_0065f890(Ptr, u32, u32, u32) -> u32),
        entry!(0x0065_f8c0, fn_0065f8c0(u32, u32, u32) -> u32),
        entry!(0x0065_f930, fn_0065f930(Ptr, u32, u32, u32) -> u32),
        entry!(0x0065_fa80, fn_0065fa80(Ptr, Ptr) -> Ptr),
        entry!(0x0065_fd80, fn_0065fd80(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0065_fe60, fn_0065fe60(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0065_ff30, fn_0065ff30(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0065_ff60, fn_0065ff60(Ptr, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0066_0030, fn_00660030(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0066_0130, fn_00660130(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0066_0190, fn_00660190(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0066_01f0, fn_006601f0(Ptr, u32, u32)),
        entry!(0x0066_0250, fn_00660250(u32) -> u32),
        entry!(0x0066_02b0, fn_006602b0(u32, u32) -> u32),
        entry!(0x0066_0300, fn_00660300(Ptr, u32, u32)),
        entry!(0x0066_0320, fn_00660320(Ptr, u32, u32)),
        entry!(0x0066_0400, fn_00660400(u32, u32) -> u32),
        entry!(0x0066_0410, fn_00660410(u32, u32) -> u32),
        entry!(0x0066_0490, fn_00660490(u32, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_0065f580_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0065F7C0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_f580, &args![0x4100u32]).u32(), 0x145d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0065F7C0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x8]);
    }

    #[test]
    fn l68d_0065f6e0_passes_this_plus_8_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0065F7F0, |_, a| Ret {
            eax: a[0] ^ 0x5555,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_f6e0, &args![0x4200u32]).u32(), 0x175d);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0065F7F0).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4200 + 0x8]);
    }
}

#[cfg(test)]
mod b046_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// Doubles for the iterator helpers: the tag is 0x5a, unwrapping reads the word.
    fn iterator_doubles(e: &mut Engine) {
        e.register(ITERATOR_TAG_0065E750, |_, _| 0x1_5au32.into_ret());
        e.register(UNWRAP_0043B1D0, |e, a| e.mem.u32(a[0]).into_ret());
    }

    #[test]
    fn b046_0065f2b0_and_0065f960_destroy_through_the_empty_stub() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        e.register(0x0040_fbe0, quiet);
        e.call_log = Some(vec![]);
        e.call(0x0065_f2b0, &args![0x4000u32, 0x100u32, 0x112u32]);
        let tag = calls_to(&e, ITERATOR_TAG_0065E750);
        assert_eq!(tag.len(), 1);
        assert_eq!(tag[0][0], tag[0][1]);
        assert_eq!(tag[0][2], tag[0][1] + 4);
        assert_eq!(
            calls_to(&e, 0x0040_fbe0),
            vec![vec![0x100, 0x112, 0x4008, 0x5a]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0065_f960, &args![1u32, 2u32, 3u32]);
        assert_eq!(calls_to(&e, 0x0040_fbe0), vec![vec![1, 2, 3, 0x5a]]);
    }

    #[test]
    fn b046_0065f380_buys_six_byte_elements() {
        let mut e = Engine::new();
        e.register(0x0065_f7c0, |_, _| 10u32.into_ret());
        e.register(0x0065_f640, quiet);
        e.register(0x0065_f6c0, |_, _| 0x9000u32.into_ret());
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0xcc; 0x18]);
        assert!(!e.call(0x0065_f380, &args![this, 0u32]).bool());
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0065_f380, &args![this, 11u32]).bool());
        assert_eq!(calls_to(&e, 0x0065_f640).len(), 1);
        assert!(calls_to(&e, 0x0065_f6c0).is_empty());
        assert_eq!(e.mem.u32(this + 0xc), 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0065_f380, &args![this, 10u32]).bool());
        assert_eq!(calls_to(&e, 0x0065_f6c0), vec![vec![this + 8, 10]]);
        assert_eq!(e.mem.u32(this + 0xc), 0x9000);
        assert_eq!(e.mem.u32(this + 0x10), 0x9000);
        assert_eq!(e.mem.u32(this + 0x14), 0x9000 + 60);
    }

    #[test]
    fn b046_0065f400_0065f9a0_and_0065ff00_destroy_each_element() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        e.register(0x0065_e9a0, quiet);
        e.call_log = Some(vec![]);
        e.call(0x0065_f400, &args![0x4000u32, 0x100u32, 0x10cu32]);
        assert_eq!(
            calls_to(&e, 0x0065_e9a0),
            vec![vec![0x4008, 0x100], vec![0x4008, 0x106]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0065_f9a0, &args![0x100u32, 0x100u32, 0x50u32]);
        assert!(calls_to(&e, 0x0065_e9a0).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0065_ff00, &args![0x200u32, 0x206u32, 0x50u32, 0u32]);
        assert_eq!(calls_to(&e, 0x0065_e9a0), vec![vec![0x50, 0x200]]);
    }

    #[test]
    fn b046_0065f430_tidies_the_buffer() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        e.register(0x0065_e9a0, quiet);
        e.register(0x0064_df10, quiet);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x0065_f430, &args![this]);
        assert!(calls_to(&e, 0x0064_df10).is_empty());
        e.mem.set_u32(this + 0xc, 0x1000);
        e.mem.set_u32(this + 0x10, 0x1006);
        e.mem.set_u32(this + 0x14, 0x1012);
        e.call_log = Some(vec![]);
        e.call(0x0065_f430, &args![this]);
        assert_eq!(calls_to(&e, 0x0065_e9a0), vec![vec![this + 8, 0x1000]]);
        assert_eq!(calls_to(&e, 0x0064_df10), vec![vec![this + 8, 0x1000, 3]]);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(e.mem.u32(this + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x14), 0);
    }

    #[test]
    fn b046_0065f550_0065fd10_and_0065fd40_make_iterators() {
        let mut e = Engine::new();
        e.register(0x0065_e000, quiet);
        e.register(0x0065_da80, quiet);
        let this = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0xc, 0x100);
        e.mem.set_u32(this + 0x10, 0x200);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_f550, &args![this, 0x50u32]).u32(), 0x50);
        assert_eq!(e.call(0x0065_fd10, &args![this, 0x60u32]).u32(), 0x60);
        assert_eq!(e.call(0x0065_fd40, &args![this, 0x70u32]).u32(), 0x70);
        assert_eq!(calls_to(&e, 0x0065_e000), vec![vec![0x50, 0x200, this]]);
        assert_eq!(
            calls_to(&e, 0x0065_da80),
            vec![vec![0x60, 0x100, this], vec![0x70, 0x200, this]]
        );
    }

    #[test]
    fn b046_0065f7a0_0065fa30_and_0065f9e0_allocate() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, a| (a[0] + 1).into_ret());
        e.register(BAD_ALLOC_CTOR_0044D6C0, quiet);
        e.register(CXX_THROW_EXCEPTION, quiet);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_f7a0, &args![0u32, 7u32]).u32(), 8);
        assert_eq!(e.call(0x0065_fa30, &args![0u32, 0u32]).u32(), 1);
        assert_eq!(e.call(0x0065_f9e0, &args![3u32, 0u32]).u32(), 19);
        assert_eq!(e.call(0x0065_f9e0, &args![0u32, 0u32]).u32(), 1);
        assert!(calls_to(&e, CXX_THROW_EXCEPTION).is_empty());
        e.call_log = Some(vec![]);
        // 0xffffffff / 0x30000000 is 5, below 6: the throw, then the wrapped size.
        assert_eq!(
            e.call(0x0065_f9e0, &args![0x3000_0000u32, 0u32]).u32(),
            0x2000_0001
        );
        let thrown = calls_to(&e, CXX_THROW_EXCEPTION);
        assert_eq!(thrown.len(), 1);
        assert_eq!(thrown[0][1], BAD_ALLOC_THROW_INFO_01140EF8);
        assert_eq!(
            calls_to(&e, BAD_ALLOC_CTOR_0044D6C0),
            vec![vec![thrown[0][0], 0]]
        );
    }

    #[test]
    fn b046_0065f7f0_answers_all_ones() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x0065_f7f0, &args![0u32]).u32(), 0xffff_ffff);
    }

    #[test]
    fn b046_0065fab0_copies_the_head_and_the_three_vectors() {
        let mut e = Engine::new();
        e.register(0x00ec_aebb, quiet);
        let source = e.mem.alloc(0x58);
        e.mem.set_u32(source, 0x7f80_0001);
        e.mem.set_u32(source + 4, 4);
        e.mem.set_u32(source + 8, 8);
        e.mem.set_u32(source + 0xc, 0xc);
        let this = e.mem.alloc(0x58);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_fab0, &args![this, source]).u32(), this);
        assert_eq!(e.mem.u32(this), 0x7fc0_0001);
        assert_eq!(e.mem.u32(this + 8), 8);
        assert_eq!(e.mem.u32(this + 0xc), 0xc);
        assert_eq!(
            calls_to(&e, 0x00ec_aebb),
            vec![vec![
                this + 0x10,
                source + 0x10,
                0x18,
                3,
                0x0065_fc10,
                0x0065_f060
            ]]
        );
    }

    fn copy_world(e: &mut Engine) {
        e.register(0x0065_de30, quiet);
        e.register(0x0065_d3a0, quiet);
        e.register(0x0065_7bc0, |_, _| 2u32.into_ret());
        e.register(0x0065_cc50, |_, _| 2u32.into_ret());
        e.register(0x0065_f7c0, |_, _| 10u32.into_ret());
        e.register(0x0065_f6c0, |_, _| 0x9000u32.into_ret());
        e.register(0x0065_da80, |e, a| {
            e.mem.set_u32(a[0], 0xb0b0);
            e.mem.set_u32(a[0] + 4, a[1]);
            a[0].into_ret()
        });
        e.register(0x0065_ff30, |_, _| 0x900cu32.into_ret());
        e.register(0x0065_ff60, |_, _| 0x900cu32.into_ret());
    }

    #[test]
    fn b046_0065fb10_copies_a_six_byte_vector() {
        let mut e = Engine::new();
        copy_world(&mut e);
        let other = e.mem.alloc(0x18);
        e.mem.set_u32(other + 0xc, 0x100);
        e.mem.set_u32(other + 0x10, 0x10c);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_fb10, &args![this, other]).u32(), this);
        let slot = calls_to(&e, 0x0065_de30);
        assert_eq!(slot.len(), 1);
        assert_eq!(slot[0][1], other + 8);
        assert_eq!(calls_to(&e, 0x0065_d3a0), vec![vec![this, this]]);
        assert_eq!(calls_to(&e, 0x0065_7bc0), vec![vec![other]]);
        assert_eq!(
            calls_to(&e, 0x0065_ff30),
            vec![vec![this, 0xb0b0, 0x100, 0xb0b0, 0x10c, 0x9000]]
        );
        assert_eq!(e.mem.u32(this + 0x10), 0x900c);
        assert_eq!(e.mem.u32(this + 0x14), 0x900c);
    }

    #[test]
    fn b046_0065fc10_copies_only_when_bought() {
        let mut e = Engine::new();
        copy_world(&mut e);
        e.register(0x0065_7ee0, |_, _| 0x100u32.into_ret());
        let other = e.mem.alloc(0x18);
        let this = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_fc10, &args![this, other]).u32(), this);
        assert_eq!(calls_to(&e, 0x0065_7ee0), vec![vec![this, 2]]);
        assert!(calls_to(&e, 0x0065_ff60).is_empty());
        e.register(0x0065_7ee0, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0065_fc10, &args![this, other]);
        assert_eq!(calls_to(&e, 0x0065_ff60).len(), 1);
        assert_eq!(e.mem.u32(this + 0x10), 0x900c);
    }

    #[test]
    fn b046_0065fd70_and_0065fe40_pass_through() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        assert_eq!(e.call(0x0065_fd70, &args![5u32, 6u32, 7u32]).u32(), 5);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x4444);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_fe40, &args![slot]).u32(), 0x4444);
        assert_eq!(calls_to(&e, UNWRAP_0043B1D0), vec![vec![slot, 0]]);
    }

    #[test]
    fn b046_0065fde0_and_00660070_construct_each_six_byte_element() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        e.register(0x0066_01f0, quiet);
        e.call_log = Some(vec![]);
        let end = e
            .call(0x0065_fde0, &args![0x100u32, 0x10cu32, 0x800u32, 0x50u32])
            .u32();
        assert_eq!(end, 0x80c);
        let tag = calls_to(&e, ITERATOR_TAG_0065E750);
        assert_eq!(tag.len(), 1);
        assert_eq!(tag[0][0], tag[0][2]);
        assert_eq!(tag[0][1] + 8, tag[0][0]);
        assert_eq!(
            calls_to(&e, 0x0066_01f0),
            vec![vec![0x50, 0x800, 0x100], vec![0x50, 0x806, 0x106]]
        );
        e.call_log = Some(vec![]);
        let words = args![0x200u32, 0x206u32, 0x900u32, 0x60u32, 0x5au32, 0u32];
        assert_eq!(e.call(0x0066_0070, &words).u32(), 0x906);
        assert_eq!(calls_to(&e, 0x0066_01f0), vec![vec![0x60, 0x900, 0x200]]);
    }

    #[test]
    fn b046_0065fea0_and_00660270_move_the_bytes() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        e.register(0x00ec_7ce3, quiet);
        e.call_log = Some(vec![]);
        let end = e
            .call(0x0065_fea0, &args![0x100u32, 0x10cu32, 0x800u32, 0x50u32])
            .u32();
        assert_eq!(end, 0x80c);
        assert_eq!(
            calls_to(&e, 0x00ec_7ce3),
            vec![vec![0x800, 0xc, 0x100, 0xc]]
        );
        e.call_log = Some(vec![]);
        let words = args![0x100u32, 0x100u32, 0x800u32, 0u32, 0u32, 0u32];
        assert_eq!(e.call(0x0066_0270, &words).u32(), 0x800);
        assert!(calls_to(&e, 0x00ec_7ce3).is_empty());
    }

    #[test]
    fn b046_0065ff90_00660210_00660340_and_00660510_reach_the_0x1c_loop() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        e.register(0x0066_0300, quiet);
        e.call_log = Some(vec![]);
        let end = e
            .call(0x0065_ff90, &args![0x100u32, 0x138u32, 0x800u32, 0x50u32])
            .u32();
        assert_eq!(end, 0x838);
        let tags = calls_to(&e, ITERATOR_TAG_0065E750);
        assert_eq!(tags.len(), 2);
        // The one-push form: the destination's address twice, then the frame's local (0).
        assert_eq!(tags[0][0], tags[0][1]);
        assert_eq!(tags[0][2], 0);
        assert_eq!(
            calls_to(&e, 0x0066_0300),
            vec![vec![0x50, 0x800, 0x100], vec![0x50, 0x81c, 0x11c]]
        );
        e.call_log = Some(vec![]);
        let words = args![0x100u32, 0x11cu32, 0x800u32, 0x50u32, 1u32, 2u32];
        assert_eq!(e.call(0x0066_0210, &words).u32(), 0x81c);
        assert_eq!(
            e.call(0x0066_0340, &args![0x100u32, 0x100u32, 0x800u32, 0x50u32])
                .u32(),
            0x800
        );
        assert_eq!(e.call(0x0066_0510, &words).u32(), 0x81c);
    }

    #[test]
    fn b046_0065ffe0_00660230_006603a0_and_006605d0_reach_the_0x58_loop() {
        let mut e = Engine::new();
        iterator_doubles(&mut e);
        e.register(0x0066_0320, quiet);
        e.call_log = Some(vec![]);
        let end = e
            .call(0x0065_ffe0, &args![0x100u32, 0x158u32, 0x800u32, 0x50u32])
            .u32();
        assert_eq!(end, 0x858);
        assert_eq!(calls_to(&e, 0x0066_0320), vec![vec![0x50, 0x800, 0x100]]);
        let words = args![0x100u32, 0x1b0u32, 0x800u32, 0x50u32, 1u32, 2u32];
        assert_eq!(e.call(0x0066_0230, &words).u32(), 0x8b0);
        assert_eq!(
            e.call(0x0066_03a0, &args![0x100u32, 0x158u32, 0x800u32, 0x50u32])
                .u32(),
            0x858
        );
        assert_eq!(e.call(0x0066_05d0, &words).u32(), 0x8b0);
    }
}

#[cfg(test)]
mod n063_tests {
    use super::*;

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, words)| words.clone())
            .collect()
    }

    fn quiet(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// The iterator helpers as plain operations: the tag is 0x5a, unwrapping reads the
    /// word, a checked iterator is (owner, pointer), iterators differ by their pointers,
    /// `begin()` and `end()` give `_Myfirst` and `_Mylast`, placement new answers the place.
    fn iterator_world(e: &mut Engine) {
        e.register(ITERATOR_TAG_0065E750, |_, _| 0x1_5au32.into_ret());
        e.register(UNWRAP_0043B1D0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(MAKE_ITERATOR_0065DED0, |e, a| {
            e.mem.set_u32(a[1], a[2]);
            e.mem.set_u32(a[1] + 4, a[3]);
            Ret::default()
        });
        e.register(ITERATORS_DIFFER_0065DFD0, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        e.register(BEGIN_0044A170, |e, a| {
            e.mem.set_u32(a[1], a[0]);
            let first = e.mem.u32(a[0] + VECTOR_FIRST);
            e.mem.set_u32(a[1] + 4, first);
            a[1].into_ret()
        });
        e.register(0x0065_e000, |e, a| {
            e.mem.set_u32(a[0], a[2]);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(PLACEMENT_NEW_006E6DA0, |_, a| a[1].into_ret());
        e.register(WORD_AT_FOUR_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
    }

    /// A vector of `count` 6-byte elements (element `i` holds `0x100 * i + 1` and `i`).
    fn vector(e: &mut Engine, count: u32) -> u32 {
        let this = e.mem.alloc(0x18);
        let buffer = e.mem.alloc(6 * count.max(1) + 6);
        for index in 0..count {
            e.mem.set_u32(buffer + 6 * index, 0x100 * index + 1);
            e.mem.set_u16(buffer + 6 * index + 4, index as u16);
        }
        e.mem.set_u32(this + VECTOR_FIRST, buffer);
        e.mem.set_u32(this + VECTOR_LAST, buffer + 6 * count);
        e.mem.set_u32(this + VECTOR_END, buffer + 6 * count + 6);
        this
    }

    #[test]
    fn capacities_count_elements_and_bytes() {
        let mut e = Engine::new();
        let this = vector(&mut e, 3);
        assert_eq!(e.call(0x0065_f2e0, &args![this]).u32(), 4);
        assert_eq!(e.call(0x0065_f4b0, &args![this]).u32(), 24);
        e.mem.set_u32(this + VECTOR_FIRST, 0);
        assert_eq!(e.call(0x0065_f2e0, &args![this]).u32(), 0);
        assert_eq!(e.call(0x0065_f4b0, &args![this]).u32(), 0);
    }

    #[test]
    fn erase_0065f5a0_moves_the_tail_down() {
        let mut e = Engine::new();
        iterator_world(&mut e);
        e.register(0x0065_e9a0, quiet);
        let this = vector(&mut e, 4);
        let first = e.mem.u32(this + VECTOR_FIRST);
        let out = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let result = e
            .call(
                0x0065_f5a0,
                &args![this, out, 0x77u32, first + 6, 0x77u32, first + 12],
            )
            .u32();
        assert_eq!(result, out);
        assert_eq!((e.mem.u32(out), e.mem.u32(out + 4)), (0x77, first + 6));
        // Elements 2 and 3 now at 1 and 2; the old last element destroyed.
        assert_eq!(e.mem.u32(first + 6), 0x201);
        assert_eq!(e.mem.u16(first + 10), 2);
        assert_eq!(e.mem.u32(first + 12), 0x301);
        assert_eq!(e.mem.u32(this + VECTOR_LAST), first + 18);
        assert_eq!(calls_to(&e, 0x0065_e9a0), vec![vec![this + 8, first + 18]]);
        // Equal iterators change nothing.
        e.call_log = Some(vec![]);
        e.call(0x0065_f5a0, &args![this, out, 0u32, first, 0u32, first]);
        assert!(calls_to(&e, ITERATOR_TAG_0065E750).is_empty());
        assert_eq!(e.mem.u32(this + VECTOR_LAST), first + 18);
    }

    #[test]
    fn clear_0065f320_destroys_every_element() {
        let mut e = Engine::new();
        iterator_world(&mut e);
        e.register(0x0065_e9a0, quiet);
        let this = vector(&mut e, 2);
        let first = e.mem.u32(this + VECTOR_FIRST);
        e.call_log = Some(vec![]);
        e.call(0x0065_f320, &args![this]);
        assert_eq!(e.mem.u32(this + VECTOR_LAST), first);
        assert_eq!(
            calls_to(&e, 0x0065_e9a0),
            vec![vec![this + 8, first], vec![this + 8, first + 6]]
        );
    }

    #[test]
    fn erase_0065f700_and_clear_0065f4f0_copy_bytes_and_destroy_through_the_stub() {
        let mut e = Engine::new();
        iterator_world(&mut e);
        e.register(0x0040_fbe0, quiet);
        e.register(MEMMOVE_S, quiet);
        let this = vector(&mut e, 3);
        let first = e.mem.u32(this + VECTOR_FIRST);
        let out = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0065_f700, &args![this, out, 1u32, first, 1u32, first + 6]);
        assert_eq!(
            calls_to(&e, MEMMOVE_S),
            vec![vec![first, 12, first + 6, 12]]
        );
        assert_eq!(e.mem.u32(this + VECTOR_LAST), first + 12);
        assert_eq!(
            calls_to(&e, 0x0040_fbe0),
            vec![vec![first + 12, first + 18, this + 8, 0x5a]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0065_f4f0, &args![this]);
        assert_eq!(e.mem.u32(this + VECTOR_LAST), first);
        assert!(calls_to(&e, MEMMOVE_S).is_empty());
    }

    #[test]
    fn length_error_0065f640_throws_vector_too_long() {
        let mut e = Engine::new();
        for address in [
            STRING_FROM_C_0044A4C0,
            LENGTH_ERROR_CTOR_0044A2E0,
            CXX_THROW_EXCEPTION,
        ] {
            e.register(address, quiet);
        }
        e.call_log = Some(vec![]);
        e.call(0x0065_f640, &args![]);
        let message = calls_to(&e, STRING_FROM_C_0044A4C0);
        assert_eq!(message[0][1], VECTOR_TOO_LONG);
        let error = calls_to(&e, LENGTH_ERROR_CTOR_0044A2E0);
        assert_eq!(error[0][1], message[0][0]);
        assert_eq!(error[0][0], message[0][0] + 0x1c);
        assert_eq!(
            calls_to(&e, CXX_THROW_EXCEPTION),
            vec![vec![error[0][0], LENGTH_ERROR_THROW_INFO_01140CB0]]
        );
    }

    #[test]
    fn allocator_0065f6c0_and_0065f7c0() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, a| (0x8000 + a[0]).into_ret());
        assert_eq!(
            e.call(0x0065_f6c0, &args![0x1234u32, 5u32]).u32(),
            0x8000 + 30
        );
        assert_eq!(e.call(0x0065_f7c0, &args![0u32]).u32(), 0x2aaa_aaaa);
    }

    #[test]
    fn copies_0065f820_0065fd80_and_00660030_copy_six_byte_elements() {
        let mut e = Engine::new();
        iterator_world(&mut e);
        let source = vector(&mut e, 3);
        let first = e.mem.u32(source + VECTOR_FIRST);
        let target = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        let end = e.call(0x0065_f820, &args![first, first + 18, target]).u32();
        assert_eq!(end, target + 18);
        assert_eq!(e.mem.u32(target + 12), 0x201);
        assert_eq!(e.mem.u16(target + 16), 2);
        let tag = calls_to(&e, ITERATOR_TAG_0065E750);
        assert_eq!(tag[0][0], tag[0][2]);
        assert_eq!(e.mem.u32(tag[0][1]), first);
        assert_eq!(e.mem.u32(tag[0][2]), target);
        let end = e
            .call(
                0x0065_fd80,
                &args![first, first + 6, target, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(end, target + 6);
        let end = e
            .call(0x0066_0030, &args![first, first, target, 0u32, 0u32, 0u32])
            .u32();
        assert_eq!(end, target);
    }

    #[test]
    fn copy_0065f8c0_and_0065fe60_move_bytes_only_when_some() {
        let mut e = Engine::new();
        iterator_world(&mut e);
        e.register(MEMMOVE_S, quiet);
        e.call_log = Some(vec![]);
        let end = e
            .call(0x0065_f8c0, &args![0x100u32, 0x10cu32, 0x400u32])
            .u32();
        assert_eq!(end, 0x40c);
        assert_eq!(calls_to(&e, MEMMOVE_S), vec![vec![0x400, 12, 0x100, 12]]);
        e.call_log = Some(vec![]);
        let end = e
            .call(
                0x0065_fe60,
                &args![0x10cu32, 0x100u32, 0x400u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(end, 0x3f4);
        assert!(calls_to(&e, MEMMOVE_S).is_empty());
    }

    #[test]
    fn uninitialized_copies_construct_through_the_allocators() {
        let mut e = Engine::new();
        iterator_world(&mut e);
        let source = vector(&mut e, 2);
        let first = e.mem.u32(source + VECTOR_FIRST);
        let this = e.mem.alloc(0x18);
        let target = e.mem.alloc(0x10);
        // `_Ucopy` of the first vector: each element through `006601f0` and `006602b0`.
        let end = e
            .call(0x0065_f890, &args![this, first, first + 12, target])
            .u32();
        assert_eq!(end, target + 12);
        assert_eq!(e.mem.u32(target + 6), 0x101);
        assert_eq!(e.mem.u16(target + 10), 1);
        // A null place constructs nothing.
        e.register(PLACEMENT_NEW_006E6DA0, |_, _| Ret::default());
        assert_eq!(e.call(0x0066_02b0, &args![0u32, first]).u32(), 0);
        // The second vector's `_Ucopy` moves the bytes.
        e.register(MEMMOVE_S, quiet);
        e.call_log = Some(vec![]);
        let end = e
            .call(0x0065_f930, &args![this, first, first + 12, target])
            .u32();
        assert_eq!(end, target + 12);
        assert_eq!(calls_to(&e, MEMMOVE_S), vec![vec![target, 12, first, 12]]);
    }

    #[test]
    fn checked_copies_unwrap_the_iterators_and_construct_each_element() {
        let mut e = Engine::new();
        iterator_world(&mut e);
        e.register(0x0065_de30, quiet);
        e.register(0x0065_d3a0, quiet);
        e.register(0x0065_7bc0, |_, _| 0u32.into_ret());
        e.register(0x0065_cc50, |_, _| 0u32.into_ret());
        e.register(0x0065_7ee0, |_, _| 0u32.into_ret());
        e.register(0x00ec_aebb, quiet);
        e.register(MEMMOVE_S, quiet);
        // The source words (6-byte elements for the checked copies).
        let records = e.mem.alloc(0x38);
        e.mem.set_u32(records, 0x3f80_0000);
        e.mem.set_u32(records + 0x1c, 0x4000_0000);
        let target = e.mem.alloc(0x38);
        let this = e.mem.alloc(0x18);
        // An empty range: both iterators unwrapped through `00660400` (their word +4),
        // nothing constructed.
        e.call_log = Some(vec![]);
        let end = e
            .call(
                0x0065_ff30,
                &args![this, 0u32, records, 0u32, records, target],
            )
            .u32();
        assert_eq!(end, target);
        let unwrapped = calls_to(&e, WORD_AT_FOUR_00726070);
        assert_eq!(unwrapped.len(), 2);
        assert_eq!(e.mem.u32(unwrapped[0][0] + 4), records);
        assert!(calls_to(&e, PLACEMENT_NEW_006E6DA0).is_empty());
        let tag = calls_to(&e, ITERATOR_TAG_0065E750);
        assert_eq!(e.mem.u32(tag[0][0]), target);
        assert_eq!(e.mem.u32(tag[0][1] + 4), records);
        // `00660130` constructs each 6-byte element of `[first, last)` (`006601f0`).
        e.call_log = Some(vec![]);
        let end = e
            .call(
                0x0066_0130,
                &args![0u32, records, 0u32, records + 12, target, this + 8],
            )
            .u32();
        assert_eq!(end, target + 12);
        assert_eq!(e.mem.u32(target), 0x3f80_0000);
        assert_eq!(
            calls_to(&e, 0x0066_01f0),
            vec![
                vec![this + 8, target, records],
                vec![this + 8, target + 6, records + 6]
            ]
        );
        // The 0x58-byte records go through `0065fab0` (one record).
        e.call_log = Some(vec![]);
        let big = e.mem.alloc(0x58);
        let place = e.mem.alloc(0x58);
        e.call(0x0066_0490, &args![place, big]);
        assert_eq!(calls_to(&e, 0x00ec_aebb).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x0065_ff60, &args![this, 0u32, 0u32, 0u32, 0u32, place]);
        e.call(0x0066_0190, &args![0u32, 0u32, 0u32, 0u32, place, this + 8]);
        assert_eq!(calls_to(&e, MEMMOVE_S).len(), 0);
        e.call(
            0x0066_0190,
            &args![0u32, records, 0u32, records + 12, place, this + 8],
        );
        assert_eq!(calls_to(&e, MEMMOVE_S), vec![vec![place, 12, records, 12]]);
    }

    #[test]
    fn constructs_and_record_copy_0065fa80() {
        let mut e = Engine::new();
        iterator_world(&mut e);
        e.register(0x0065_de30, quiet);
        e.register(0x0065_d3a0, quiet);
        e.register(0x0065_7bc0, |_, _| 0u32.into_ret());
        let source = e.mem.alloc(0x1c);
        e.mem.set_u32(source, 0x7f80_0001);
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_fa80, &args![this, source]).u32(), this);
        // A signalling NaN is quieted by the FLD/FSTP copy.
        assert_eq!(e.mem.u32(this), 0x7fc0_0001);
        assert_eq!(calls_to(&e, 0x0065_7bc0), vec![vec![source + 4]]);
        // The construct wrappers forward place and value.
        let place = e.mem.alloc(0x1c);
        e.call(0x0066_0300, &args![0u32, place, source]);
        assert_eq!(e.mem.u32(place), 0x7fc0_0001);
        let small = e.mem.alloc(8);
        e.mem.set_u32(small, 0x1234_5678);
        e.mem.set_u16(small + 4, 0x9abc);
        let place = e.mem.alloc(8);
        e.call(0x0066_01f0, &args![0u32, place, small]);
        assert_eq!(e.mem.u32(place), 0x1234_5678);
        assert_eq!(e.mem.u16(place + 4), 0x9abc);
        e.register(PLACEMENT_NEW_006E6DA0, |_, _| Ret::default());
        assert_eq!(e.call(0x0066_0410, &args![0u32, source]).u32(), 0);
        assert_eq!(e.call(0x0066_0490, &args![0u32, source]).u32(), 0);
        e.call_log = Some(vec![]);
        e.call(0x0066_0320, &args![0u32, 0u32, source]);
        assert_eq!(calls_to(&e, PLACEMENT_NEW_006E6DA0), vec![vec![0x58, 0]]);
        // The unwrapping helpers read the word +4 of the iterator.
        let iterator = e.mem.alloc(8);
        e.mem.set_u32(iterator + 4, 0x4242);
        assert_eq!(e.call(0x0066_0250, &args![iterator]).u32(), 0x4242);
        assert_eq!(e.call(0x0066_0400, &args![iterator, 9u32]).u32(), 0x4242);
    }
}
