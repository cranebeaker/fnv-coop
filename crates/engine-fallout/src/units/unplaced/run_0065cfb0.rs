//! `(unplaced)/run_0065cfb0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0065cfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0065d6e0`, a destructor with an exception frame
/// that calls `0064df10` on the word at `+0` with 1 and then `00483710`, and returns
/// nothing. Parameters: one word, `this`; the disassembly of the function ends in a plain
/// `RET` and passes only `ECX` on. `0065d6e0`'s disassembly ends with its exception-frame
/// unlink after the empty `00483710`; no result is computed (Ghidra types it void).
pub fn fn_0065cfb0(e: &mut Engine, this: Ptr) {
    e.call(0x0065_d6e0, &args![this]);
}

// The rest of this unit is template code of `std::vector` and its allocator, iterators and
// algorithms for element sizes 8, 0xc, 0x10, 0x1c, 0x20, 0x2c, 0x34, 0x38 and 0x58 (unnamed in
// the map; the element types are not identified). The vectors keep their allocator at `+8`
// and `_Myfirst`, `_Mylast` and `_Myend` at `+0xc`, `+0x10` and `+0x14`; the checked
// iterators are two words (`+0`: the iterator base word `0064de50` initialises and
// `0044ac20` sets from the container, `+4`: the element pointer).

/// The vector's allocator, `_Myfirst`, `_Mylast` and `_Myend`.
const VECTOR_ALLOCATOR: u32 = 0x08;
const VECTOR_FIRST: u32 = 0x0c;
const VECTOR_LAST: u32 = 0x10;
const VECTOR_END: u32 = 0x14;
/// `0065f640` (no arguments; unnamed): what `_Buy` calls when the count is above `max_size()`
/// (the length error; it does not return in the game).
const LENGTH_ERROR_0065F640: u32 = 0x0065_f640;
/// `0064df10` (thiscall on the allocator, two words: the block and the element count): the
/// allocator's `deallocate`.
const DEALLOCATE_0064DF10: u32 = 0x0064_df10;
/// `_invalid_parameter_noinfo` shape (`00ec7c56`, no arguments): the checked iterators' failure.
const INVALID_PARAMETER_NOINFO: u32 = 0x00ec_7c56;
/// `operator new` (`00401000`) and `operator delete` (`00401030`), cdecl, one word.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00483710` (thiscall): the empty base destructor body the map puts in `TESForm`'s unit.
const EMPTY_BASE_00483710: u32 = 0x0048_3710;

/// How `_Tidy` turns the byte size of the buffer into an element count: `IDIV` by the size
/// (truncating, signed) or `SAR` by its power of two (flooring, signed).
#[derive(Clone, Copy)]
enum ElementCount {
    Divide(i32),
    Shift(u32),
}

impl ElementCount {
    fn of(self, bytes: u32) -> u32 {
        match self {
            ElementCount::Divide(size) => (bytes as i32).wrapping_div(size) as u32,
            ElementCount::Shift(shift) => ((bytes as i32) >> shift) as u32,
        }
    }
}

/// `std::vector<T>::_Buy(count)` for one element size (thiscall, one word; the result in
/// `AL`): clears `_Myfirst`, `_Mylast` and `_Myend`; 0 for a count of 0. When `max_size()`
/// (`max_size`, thiscall on the vector) is below the count it calls the length error (which
/// does not return in the game; the code after it returns 1); otherwise it allocates `count`
/// elements (`allocate`, thiscall on the allocator at `+8`) into `_Myfirst` and `_Mylast` and
/// sets `_Myend` to `_Myfirst + count * element`. Returns 1.
fn vector_buy(
    e: &mut Engine,
    this: Ptr,
    count: u32,
    max_size: u32,
    allocate: u32,
    element: u32,
) -> bool {
    let base = this.addr();
    e.mem.set_u32(base + VECTOR_FIRST, 0);
    e.mem.set_u32(base + VECTOR_LAST, 0);
    e.mem.set_u32(base + VECTOR_END, 0);
    if count == 0 {
        return false;
    }
    let max = e.call(max_size, &args![this]).u32();
    if max < count {
        e.call(LENGTH_ERROR_0065F640, &args![]);
    } else {
        let block = e
            .call(allocate, &args![base + VECTOR_ALLOCATOR, count])
            .u32();
        e.mem.set_u32(base + VECTOR_FIRST, block);
        let first = e.mem.u32(base + VECTOR_FIRST);
        e.mem.set_u32(base + VECTOR_LAST, first);
        let first = e.mem.u32(base + VECTOR_FIRST);
        e.mem.set_u32(
            base + VECTOR_END,
            count.wrapping_mul(element).wrapping_add(first),
        );
    }
    true
}

/// `std::vector<T>::_Tidy()` for one element size (thiscall): when `_Myfirst` is not null,
/// destroys `[_Myfirst, _Mylast)` (`destroy`) and deallocates the buffer
/// (`0064df10(allocator, _Myfirst, (_Myend - _Myfirst) / element)`); then clears the three
/// pointers.
fn vector_tidy(
    e: &mut Engine,
    this: Ptr,
    destroy: &dyn Fn(&mut Engine, u32, u32),
    count: ElementCount,
) {
    let base = this.addr();
    if e.mem.u32(base + VECTOR_FIRST) != 0 {
        let last = e.mem.u32(base + VECTOR_LAST);
        let first = e.mem.u32(base + VECTOR_FIRST);
        destroy(e, first, last);
        let bytes = e
            .mem
            .u32(base + VECTOR_END)
            .wrapping_sub(e.mem.u32(base + VECTOR_FIRST));
        let elements = count.of(bytes);
        let first = e.mem.u32(base + VECTOR_FIRST);
        e.call(
            DEALLOCATE_0064DF10,
            &args![base + VECTOR_ALLOCATOR, first, elements],
        );
    }
    e.mem.set_u32(base + VECTOR_FIRST, 0);
    e.mem.set_u32(base + VECTOR_LAST, 0);
    e.mem.set_u32(base + VECTOR_END, 0);
}

// Translated from 0065cfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Buy` for 0xc-byte elements (`max_size` `0065db90`, `allocate`
/// `0065dbb0`); see [`vector_buy`].
pub fn fn_0065cfd0(e: &mut Engine, this: Ptr, count: u32) -> bool {
    vector_buy(e, this, count, 0x0065_db90, 0x0065_dbb0, 0xc)
}

// Translated from 0065d050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Tidy` for 0xc-byte elements (destroy `0065db60`, count by `IDIV 0xc`).
pub fn fn_0065d050(e: &mut Engine, this: Ptr) {
    vector_tidy(
        e,
        this,
        &|e, first, last| fn_0065db60(e, this, first, last),
        ElementCount::Divide(0xc),
    );
}

// Translated from 0065d0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Buy` for 0x10-byte elements (`max_size` `0064de90`, `allocate`
/// `0064df30`).
pub fn fn_0065d0d0(e: &mut Engine, this: Ptr, count: u32) -> bool {
    vector_buy(e, this, count, 0x0064_de90, 0x0064_df30, 0x10)
}

// Translated from 0065d150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Tidy` for 0x10-byte elements (destroy `0064dee0`, thiscall on the vector
/// with first and last; count by `SAR 4`).
pub fn fn_0065d150(e: &mut Engine, this: Ptr) {
    vector_tidy(
        e,
        this,
        &|e, first, last| {
            e.call(0x0064_dee0, &args![this, first, last]);
        },
        ElementCount::Shift(4),
    );
}

// Translated from 0065d1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Buy` for 8-byte elements (`max_size` `0065dbd0`, `allocate` `0065dc20`).
pub fn fn_0065d1c0(e: &mut Engine, this: Ptr, count: u32) -> bool {
    vector_buy(e, this, count, 0x0065_dbd0, 0x0065_dc20, 8)
}

// Translated from 0065d240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Tidy` for 8-byte elements (destroy `0065dbf0`, count by `SAR 3`).
pub fn fn_0065d240(e: &mut Engine, this: Ptr) {
    vector_tidy(
        e,
        this,
        &|e, first, last| fn_0065dbf0(e, this, first, last),
        ElementCount::Shift(3),
    );
}

// Translated from 0065d2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Buy` for 0x20-byte elements (`max_size` `0065dc40`, `allocate`
/// `0065dd50`).
pub fn fn_0065d2b0(e: &mut Engine, this: Ptr, count: u32) -> bool {
    vector_buy(e, this, count, 0x0065_dc40, 0x0065_dd50, 0x20)
}

// Translated from 0065d330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Tidy` for 0x20-byte elements (destroy `0065dc60`, thiscall on the vector;
/// count by `SAR 5`).
pub fn fn_0065d330(e: &mut Engine, this: Ptr) {
    vector_tidy(
        e,
        this,
        &|e, first, last| {
            e.call(0x0065_dc60, &args![this, first, last]);
        },
        ElementCount::Shift(5),
    );
}

// Translated from 0065d3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a constructor taking an allocator by value (thiscall, one word, `RET 4`). It
/// pushes `ECX` (`this`) as the by-value allocator word for `0065dc90`, copy-constructs that
/// slot from its own argument (`0065de30`, which writes nothing), calls `0065dc90(this, slot)`
/// and copy-constructs the allocator at `+8` from its argument. Returns `this`.
pub fn fn_0065d3a0(e: &mut Engine, this: Ptr, allocator: u32) -> Ptr {
    e.with_stack(8, |e, frame| {
        // frame + 0: this function's argument; frame + 4: the word pushed for 0065dc90.
        let argument = frame.addr();
        let pushed = frame.addr() + 4;
        e.mem.set_u32(argument, allocator);
        e.mem.set_u32(pushed, this.addr());
        fn_0065de30(e, Ptr::new(pushed), argument);
        let word = e.mem.u32(pushed);
        fn_0065dc90(e, this, word);
        fn_0065de30(e, this.byte_add(VECTOR_ALLOCATOR), argument);
    });
    this
}

// Translated from 0065d3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Buy` for 0x2c-byte elements (`max_size` `0065dd70`, `allocate`
/// `0065ddc0`).
pub fn fn_0065d3e0(e: &mut Engine, this: Ptr, count: u32) -> bool {
    vector_buy(e, this, count, 0x0065_dd70, 0x0065_ddc0, 0x2c)
}

// Translated from 0065d460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Tidy` for 0x2c-byte elements (destroy `0065dd90`, thiscall on the vector;
/// count by `IDIV 0x2c`).
pub fn fn_0065d460(e: &mut Engine, this: Ptr) {
    vector_tidy(
        e,
        this,
        &|e, first, last| {
            e.call(0x0065_dd90, &args![this, first, last]);
        },
        ElementCount::Divide(0x2c),
    );
}

// Translated from 0065d4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Buy` for 0x34-byte elements (`max_size` `0065dde0`, `allocate`
/// `0065de40`).
pub fn fn_0065d4e0(e: &mut Engine, this: Ptr, count: u32) -> bool {
    vector_buy(e, this, count, 0x0065_dde0, 0x0065_de40, 0x34)
}

// Translated from 0065d560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Tidy` for 0x34-byte elements (destroy `0065de00`, count by `IDIV 0x34`).
pub fn fn_0065d560(e: &mut Engine, this: Ptr) {
    vector_tidy(
        e,
        this,
        &|e, first, last| fn_0065de00(e, this, first, last),
        ElementCount::Divide(0x34),
    );
}

// Translated from 0065d5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Buy` for 0x38-byte elements (`max_size` `0065de60`, `allocate`
/// `0065deb0`).
pub fn fn_0065d5e0(e: &mut Engine, this: Ptr, count: u32) -> bool {
    vector_buy(e, this, count, 0x0065_de60, 0x0065_deb0, 0x38)
}

// Translated from 0065d660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Tidy` for 0x38-byte elements (destroy `0065de80`, count by `IDIV 0x38`).
pub fn fn_0065d660(e: &mut Engine, this: Ptr) {
    vector_tidy(
        e,
        this,
        &|e, first, last| fn_0065de80(e, this, first, last),
        ElementCount::Divide(0x38),
    );
}

// Translated from 0065d6e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065d6e0 - the compiler's exception frame (FS:[0] chain, state at EBP-4, handler 00f005f8) is not translated
/// Unnamed: a destructor (thiscall): deallocates the one element at `+0` through the allocator
/// at `+4` (`0064df10(this + 4, [this], 1)`), then `00483710(this)`.
pub fn fn_0065d6e0(e: &mut Engine, this: Ptr) {
    let block = e.mem.u32(this.addr());
    e.call(DEALLOCATE_0064DF10, &args![this.addr() + 4, block, 1u32]);
    e.call(EMPTY_BASE_00483710, &args![this]);
}

/// The vector's `erase(first, last)` for one element size (thiscall, five words: the result
/// iterator and the two iterators by value). Builds checked iterators for both element
/// pointers (`0065ded0`) in two 8-byte locals; when they differ (`0065dfd0`), copies `[last,
/// _Mylast)` down to `first` (`copy`), destroys the leftover tail (`destroy`) and makes the
/// copy's end the new `_Mylast`. The result is the first iterator.
fn vector_erase(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    first_base: u32,
    first: u32,
    last_base: u32,
    last: u32,
    copy: fn(&mut Engine, u32, u32, u32) -> u32,
    destroy: fn(&mut Engine, Ptr, u32, u32),
) -> Ptr {
    e.with_stack(0x10, |e, frame| {
        // frame + 8: the first iterator (EBP-8); frame + 0: the last iterator (EBP-0x10).
        let first_iterator = frame.byte_add(8);
        let last_iterator = frame;
        fn_0065ded0(e, this, first_iterator, first_base, first);
        fn_0065ded0(e, this, last_iterator, last_base, last);
        if fn_0065dfd0(e, first_iterator, last_iterator) as u8 != 0 {
            let source = e.mem.u32(last_iterator.addr() + 4);
            let end = e.mem.u32(this.addr() + VECTOR_LAST);
            let destination = e.mem.u32(first_iterator.addr() + 4);
            let copied = copy(e, source, end, destination);
            let end = e.mem.u32(this.addr() + VECTOR_LAST);
            destroy(e, this, copied, end);
            e.mem.set_u32(this.addr() + VECTOR_LAST, copied);
        }
        let word = e.mem.u32(first_iterator.addr());
        e.mem.set_u32(result.addr(), word);
        let word = e.mem.u32(first_iterator.addr() + 4);
        e.mem.set_u32(result.addr() + 4, word);
    });
    result
}

// Translated from 0065d740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::erase(first, last)` for 0x1c-byte elements (copy `0065e1f0`, destroy
/// `0065df10`); see [`vector_erase`]. Returns `result`.
pub fn fn_0065d740(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    first_base: u32,
    first: u32,
    last_base: u32,
    last: u32,
) -> Ptr {
    vector_erase(
        e,
        this,
        result,
        first_base,
        first,
        last_base,
        last,
        fn_0065e1f0,
        fn_0065df10,
    )
}

// Translated from 0065d7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Buy` for 0x1c-byte elements (`max_size` `0065def0`, `allocate`
/// `0065df40`).
pub fn fn_0065d7e0(e: &mut Engine, this: Ptr, count: u32) -> bool {
    vector_buy(e, this, count, 0x0065_def0, 0x0065_df40, 0x1c)
}

// Translated from 0065d860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Tidy` for 0x1c-byte elements (destroy `0065df10`, count by `IDIV 0x1c`).
pub fn fn_0065d860(e: &mut Engine, this: Ptr) {
    vector_tidy(
        e,
        this,
        &|e, first, last| fn_0065df10(e, this, first, last),
        ElementCount::Divide(0x1c),
    );
}

// Translated from 0065d8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::erase(first, last)` for 0x58-byte elements (copy `0065e260`, destroy
/// `0065df80`); see [`vector_erase`]. Returns `result`.
pub fn fn_0065d8e0(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    first_base: u32,
    first: u32,
    last_base: u32,
    last: u32,
) -> Ptr {
    vector_erase(
        e,
        this,
        result,
        first_base,
        first,
        last_base,
        last,
        fn_0065e260,
        fn_0065df80,
    )
}

// Translated from 0065d980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Buy` for 0x58-byte elements (`max_size` `0065df60`, `allocate`
/// `0065dfb0`).
pub fn fn_0065d980(e: &mut Engine, this: Ptr, count: u32) -> bool {
    vector_buy(e, this, count, 0x0065_df60, 0x0065_dfb0, 0x58)
}

// Translated from 0065da00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Tidy` for 0x58-byte elements (destroy `0065df80`, count by `IDIV 0x58`).
pub fn fn_0065da00(e: &mut Engine, this: Ptr) {
    vector_tidy(
        e,
        this,
        &|e, first, last| fn_0065df80(e, this, first, last),
        ElementCount::Divide(0x58),
    );
}

/// `0064de50` (thiscall): initialises a checked iterator's base word. `0044ac20` (thiscall,
/// one word: the container): adopts the container.
const ITERATOR_BASE_INIT_0064DE50: u32 = 0x0064_de50;
const ITERATOR_ADOPT_0044AC20: u32 = 0x0044_ac20;

// Translated from 0065da80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the checked iterator's constructor from an element pointer and its vector
/// (thiscall, two words, `RET 8`). Initialises the base (`0064de50`); a null vector or a
/// pointer outside `[_Myfirst, _Mylast]` calls `_invalid_parameter_noinfo` (`00ec7c56`). Then
/// it adopts the vector (`0044ac20`) and keeps the pointer at `+4`. Returns `this`.
pub fn fn_0065da80(e: &mut Engine, this: Ptr, pointer: u32, vector: u32) -> Ptr {
    e.call(ITERATOR_BASE_INIT_0064DE50, &args![this]);
    if vector == 0
        || e.mem.u32(vector + VECTOR_FIRST) > pointer
        || pointer > e.mem.u32(vector + VECTOR_LAST)
    {
        e.call(INVALID_PARAMETER_NOINFO, &args![]);
    }
    e.call(ITERATOR_ADOPT_0044AC20, &args![this, vector]);
    e.mem.set_u32(this.addr() + 4, pointer);
    this
}

/// `005ca4f0` (thiscall; the result in `AL`: whether the iterator has a container) and
/// `0060b980` (thiscall: the iterator's container), unnamed in the map.
const ITERATOR_HAS_CONTAINER_005CA4F0: u32 = 0x005c_a4f0;
const ITERATOR_CONTAINER_0060B980: u32 = 0x0060_b980;
/// `004b0460` (thiscall, one word: the other iterator; the result in `AL`: whether both have
/// the same container), unnamed in the map.
const ITERATOR_COMPATIBLE_004B0460: u32 = 0x004b_0460;

// Translated from 0065dae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: a checked iterator's `operator++` with a 6-byte step (thiscall). Calls
/// `_invalid_parameter_noinfo` when the iterator has no container (`005ca4f0`) and when its
/// pointer (`+4`) is not below the container's `_Mylast` (`0060b980`); then adds 6 to the
/// pointer. Returns `this`.
pub fn fn_0065dae0(e: &mut Engine, this: Ptr) -> Ptr {
    if e.call(ITERATOR_HAS_CONTAINER_005CA4F0, &args![this]).u32() as u8 == 0 {
        e.call(INVALID_PARAMETER_NOINFO, &args![]);
    }
    let container = e.call(ITERATOR_CONTAINER_0060B980, &args![this]).u32();
    if e.mem.u32(this.addr() + 4) >= e.mem.u32(container + VECTOR_LAST) {
        e.call(INVALID_PARAMETER_NOINFO, &args![]);
    }
    let pointer = e.mem.u32(this.addr() + 4);
    e.mem.set_u32(this.addr() + 4, pointer.wrapping_add(6));
    this
}

// Translated from 0065db40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the checked iterator's default constructor (thiscall): initialises the base
/// (`0064de50`) and clears the pointer at `+4`. Returns `this`.
pub fn fn_0065db40(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(ITERATOR_BASE_INIT_0064DE50, &args![this]);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

/// The vector's `_Destroy(first, last)` for one element size (thiscall, two words, `RET 8`):
/// the cdecl range destructor with `first`, `last` and the allocator at `+8`.
fn vector_destroy(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    last: u32,
    destroy_range: fn(&mut Engine, u32, u32, u32),
) {
    destroy_range(e, first, last, this.addr() + VECTOR_ALLOCATOR);
}

// Translated from 0065db60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Destroy` for 0xc-byte elements (`0065e2d0`); see [`vector_destroy`].
pub fn fn_0065db60(e: &mut Engine, this: Ptr, first: u32, last: u32) {
    vector_destroy(e, this, first, last, fn_0065e2d0);
}

// Translated from 0065dbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Destroy` for 8-byte elements (`0065e360`).
pub fn fn_0065dbf0(e: &mut Engine, this: Ptr, first: u32, last: u32) {
    vector_destroy(e, this, first, last, fn_0065e360);
}

/// The callees of `0065dc90`: `006815c0` (thiscall: the base constructor),
/// `0044abd0` (thiscall on the allocator, one word), `006e6da0` (cdecl, two words) and
/// `008c71b0` (thiscall, one word; the map names the folded body
/// `BGSChangeFlags::IsForcedChange`, which does not fit this use).
const BASE_CTOR_006815C0: u32 = 0x0068_15c0;
const ALLOCATOR_0044ABD0: u32 = 0x0044_abd0;
const CONSTRUCT_006E6DA0: u32 = 0x006e_6da0;
const CONSTRUCT_008C71B0: u32 = 0x008c_71b0;

// Translated from 0065dc90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065dc90 - the compiler's exception frame (FS:[0] chain, state at EBP-4, handler 00eff859) is not translated
/// Unnamed: a constructor taking an allocator by value (thiscall, one word, `RET 4`). Calls
/// `006815c0(this)`, copy-constructs the allocator at `+4` from its argument (`0065de30`),
/// gets `n = 0044abd0(this + 4, 1)` and `p = 006e6da0(4, n)`; when `p` is not null,
/// `008c71b0(p, this)` is stored at `+0`, else 0. Returns `this`.
pub fn fn_0065dc90(e: &mut Engine, this: Ptr, allocator: u32) -> Ptr {
    e.call(BASE_CTOR_006815C0, &args![this]);
    e.with_stack(4, |e, argument| {
        e.mem.set_u32(argument.addr(), allocator);
        fn_0065de30(e, this.byte_add(4), argument.addr());
    });
    let count = e
        .call(ALLOCATOR_0044ABD0, &args![this.addr() + 4, 1u32])
        .u32();
    let block = e.call(CONSTRUCT_006E6DA0, &args![4u32, count]).u32();
    let value = if block != 0 {
        e.call(CONSTRUCT_008C71B0, &args![block, this]).u32()
    } else {
        0
    };
    e.mem.set_u32(this.addr(), value);
    this
}

// Translated from 0065de00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Destroy` for 0x34-byte elements (`0065e510`).
pub fn fn_0065de00(e: &mut Engine, this: Ptr, first: u32, last: u32) {
    vector_destroy(e, this, first, last, fn_0065e510);
}

// Translated from 0065de30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: an empty allocator's copy constructor (thiscall, one word that it does not read,
/// `RET 4`). Returns `this`.
pub fn fn_0065de30(_e: &mut Engine, this: Ptr, _unused_1: u32) -> Ptr {
    this
}

// Translated from 0065de80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Destroy` for 0x38-byte elements (`0065e5a0`).
pub fn fn_0065de80(e: &mut Engine, this: Ptr, first: u32, last: u32) {
    vector_destroy(e, this, first, last, fn_0065e5a0);
}

// Translated from 0065ded0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: makes the checked iterator for an element pointer of this vector (thiscall, three
/// words: the result, the by-value iterator's base word, which it does not read, and its
/// pointer; `RET 0xc`): `0065e000(result, pointer, this)`. Returns `result`.
pub fn fn_0065ded0(e: &mut Engine, this: Ptr, result: Ptr, _unused_2: u32, pointer: u32) -> Ptr {
    fn_0065e000(e, result, pointer, this.addr());
    result
}

// Translated from 0065df10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Destroy` for 0x1c-byte elements (`0065e630`).
pub fn fn_0065df10(e: &mut Engine, this: Ptr, first: u32, last: u32) {
    vector_destroy(e, this, first, last, fn_0065e630);
}

// Translated from 0065df80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Destroy` for 0x58-byte elements (`0065e6c0`).
pub fn fn_0065df80(e: &mut Engine, this: Ptr, first: u32, last: u32) {
    vector_destroy(e, this, first, last, fn_0065e6c0);
}

// Translated from 0065dfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the checked iterators' `operator!=` (thiscall, one word): 0 when `0065e1a0`
/// (`operator==`) answers true, else 1 (`NEG; SBB; ADD 1` on its `AL`).
pub fn fn_0065dfd0(e: &mut Engine, this: Ptr, other: Ptr) -> u32 {
    if fn_0065e1a0(e, this, other) {
        0
    } else {
        1
    }
}

// Translated from 0065e000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the checked iterator's constructor (thiscall, two words: the pointer and the
/// vector, `RET 8`), forwarding to `0065da80`. Returns `this`.
pub fn fn_0065e000(e: &mut Engine, this: Ptr, pointer: u32, vector: u32) -> Ptr {
    fn_0065da80(e, this, pointer, vector);
    this
}

// Translated from 0065e1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: the checked iterators' `operator==` (thiscall, one word; the result in `AL`).
/// Calls `_invalid_parameter_noinfo` when this iterator has no container (`005ca4f0`) or the
/// two are not of the same container (`004b0460`, only asked when there is one); then
/// compares the pointers at `+4`.
pub fn fn_0065e1a0(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let has = e.call(ITERATOR_HAS_CONTAINER_005CA4F0, &args![this]).u32() as u8 != 0;
    if !has
        || e.call(ITERATOR_COMPATIBLE_004B0460, &args![this, other])
            .u32() as u8
            == 0
    {
        e.call(INVALID_PARAMETER_NOINFO, &args![]);
    }
    e.mem.u32(this.addr() + 4) == e.mem.u32(other.addr() + 4)
}

/// `0065fd70` (cdecl, three words; returns its first argument) and `0065fe40` (cdecl, one word:
/// the address of an iterator; returns the plain pointer), unnamed in the map.
const FIRST_ARGUMENT_0065FD70: u32 = 0x0065_fd70;
const UNCHECKED_0065FE40: u32 = 0x0065_fe40;

/// `std::copy` through checked iterators for one element size (cdecl, three words: first,
/// last, destination; the frame: three local bytes below `EBP` and the three argument
/// words). `0065e750` gives the iterator tag (with `ECX` the address of the destination
/// word), `0065fd70` the address of the uninitialised byte at `EBP-3`, whose value is passed
/// on; `0065fe40` unwraps last and first; then `copy` (`0065e760`/`0065e7c0`) runs with the
/// unwrapped first, last, the destination, the byte, the tag and 0. Its result is the result.
fn checked_copy(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    copy: fn(&mut Engine, u32, u32, u32, u32, u32, u32) -> u32,
) -> u32 {
    e.with_stack(0x18, |e, frame| {
        // EBP is frame + 4: locals at frame + 0..4, arguments at frame + 0xc, + 0x10, + 0x14.
        let ebp = frame.addr() + 4;
        let (first_at, last_at, destination_at) = (ebp + 8, ebp + 0xc, ebp + 0x10);
        e.mem.set_u32(first_at, first);
        e.mem.set_u32(last_at, last);
        e.mem.set_u32(destination_at, destination);
        e.mem.set_u8(ebp - 1, 0);
        let tag = fn_0065e750(e, destination_at, first_at, destination_at);
        e.mem.set_u8(ebp - 2, tag);
        let zero = e.mem.u8(ebp - 1) as u32;
        let tag = e.mem.u8(ebp - 2) as u32;
        let byte_at = e
            .call(
                FIRST_ARGUMENT_0065FD70,
                &args![ebp - 3, first_at, destination_at],
            )
            .u32();
        let byte = e.mem.u8(byte_at) as u32;
        let destination = e.mem.u32(destination_at);
        let last = e.call(UNCHECKED_0065FE40, &args![last_at]).u32();
        let first = e.call(UNCHECKED_0065FE40, &args![first_at]).u32();
        copy(e, first, last, destination, byte, tag, zero)
    })
}

// Translated from 0065e1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::copy` for 0x1c-byte elements (`0065e760`); see [`checked_copy`].
pub fn fn_0065e1f0(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    checked_copy(e, first, last, destination, fn_0065e760)
}

// Translated from 0065e260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::copy` for 0x58-byte elements (`0065e7c0`); see [`checked_copy`].
pub fn fn_0065e260(e: &mut Engine, first: u32, last: u32, destination: u32) -> u32 {
    checked_copy(e, first, last, destination, fn_0065e7c0)
}

/// `std::_Destroy_range(first, last, allocator)` for one element size (cdecl, three words):
/// with the iterator tag from `0065e750` (`ECX` the address of the `first` word), the range
/// destructor `destroy` (cdecl, four words).
fn destroy_range(
    e: &mut Engine,
    first: u32,
    last: u32,
    allocator: u32,
    destroy: fn(&mut Engine, u32, u32, u32, u32),
) {
    e.with_stack(8, |e, arguments| {
        let (first_at, last_at) = (arguments.addr(), arguments.addr() + 4);
        e.mem.set_u32(first_at, first);
        e.mem.set_u32(last_at, last);
        let tag = fn_0065e750(e, first_at, first_at, last_at) as u32;
        let last = e.mem.u32(last_at);
        let first = e.mem.u32(first_at);
        destroy(e, first, last, allocator, tag);
    });
}

// Translated from 0065e2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Destroy_range` for 0xc-byte elements (`0065e820`); see [`destroy_range`].
pub fn fn_0065e2d0(e: &mut Engine, first: u32, last: u32, allocator: u32) {
    destroy_range(e, first, last, allocator, fn_0065e820);
}

/// `_CxxThrowException` (`00ec7894`, two words: the object and the throw info),
/// `std::bad_alloc`'s constructor (`0044d6c0`, thiscall, one word: the message) and the
/// throw info of `std::bad_alloc` (`01140ef8`).
const CXX_THROW_EXCEPTION: u32 = 0x00ec_7894;
const BAD_ALLOC_CTOR_0044D6C0: u32 = 0x0044_d6c0;
const BAD_ALLOC_THROW_INFO_01140EF8: u32 = 0x0114_0ef8;

/// `std::_Allocate(count, T*)` for one element size (cdecl, two words; the second is not
/// read). A count of 0 stays 0; when `0xffffffff / count` is below the element size it
/// constructs a `std::bad_alloc` (12 bytes on the stack, message 0) and throws it (which does
/// not return in the game; the code after it is the allocation). Returns `operator new(count
/// * element)`.
fn allocate_elements(e: &mut Engine, count: u32, element: u32) -> u32 {
    if count != 0 && 0xffff_ffff / count < element {
        e.with_stack(0xc, |e, exception| {
            e.call(BAD_ALLOC_CTOR_0044D6C0, &args![exception, 0u32]);
            e.call(
                CXX_THROW_EXCEPTION,
                &args![exception, BAD_ALLOC_THROW_INFO_01140EF8],
            );
        });
    }
    e.call(OPERATOR_NEW, &args![count.wrapping_mul(element)])
        .u32()
}

// Translated from 0065e310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Allocate` for 0xc-byte elements; see [`allocate_elements`].
pub fn fn_0065e310(e: &mut Engine, count: u32, _unused_2: u32) -> u32 {
    allocate_elements(e, count, 0xc)
}

// Translated from 0065e360 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Destroy_range` for 8-byte elements (`0065e850`).
pub fn fn_0065e360(e: &mut Engine, first: u32, last: u32, allocator: u32) {
    destroy_range(e, first, last, allocator, fn_0065e850);
}

// Translated from 0065e3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Allocate` for 8-byte elements.
pub fn fn_0065e3a0(e: &mut Engine, count: u32, _unused_2: u32) -> u32 {
    allocate_elements(e, count, 8)
}

// Translated from 0065e510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Destroy_range` for 0x34-byte elements (`0065e8e0`).
pub fn fn_0065e510(e: &mut Engine, first: u32, last: u32, allocator: u32) {
    destroy_range(e, first, last, allocator, fn_0065e8e0);
}

// Translated from 0065e550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Allocate` for 0x34-byte elements.
pub fn fn_0065e550(e: &mut Engine, count: u32, _unused_2: u32) -> u32 {
    allocate_elements(e, count, 0x34)
}

// Translated from 0065e5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Destroy_range` for 0x38-byte elements (`0065e910`).
pub fn fn_0065e5a0(e: &mut Engine, first: u32, last: u32, allocator: u32) {
    destroy_range(e, first, last, allocator, fn_0065e910);
}

// Translated from 0065e5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Allocate` for 0x38-byte elements.
pub fn fn_0065e5e0(e: &mut Engine, count: u32, _unused_2: u32) -> u32 {
    allocate_elements(e, count, 0x38)
}

// Translated from 0065e630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Destroy_range` for 0x1c-byte elements (`0065e940`).
pub fn fn_0065e630(e: &mut Engine, first: u32, last: u32, allocator: u32) {
    destroy_range(e, first, last, allocator, fn_0065e940);
}

// Translated from 0065e670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Allocate` for 0x1c-byte elements.
pub fn fn_0065e670(e: &mut Engine, count: u32, _unused_2: u32) -> u32 {
    allocate_elements(e, count, 0x1c)
}

// Translated from 0065e6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Destroy_range` for 0x58-byte elements (`0065e970`).
pub fn fn_0065e6c0(e: &mut Engine, first: u32, last: u32, allocator: u32) {
    destroy_range(e, first, last, allocator, fn_0065e970);
}

// Translated from 0065e700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Allocate` for 0x58-byte elements.
pub fn fn_0065e700(e: &mut Engine, count: u32, _unused_2: u32) -> u32 {
    allocate_elements(e, count, 0x58)
}

// Translated from 0065e750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed: returns an empty iterator-tag struct by value (cdecl, two words that it does not
/// read). The result in `AL` is the byte at `EBP-1` of its frame, the top byte of the `ECX`
/// it pushed: the caller's `ECX`, so it is the Rust function's first parameter and the first
/// word of the registered form, before the two stack words.
pub fn fn_0065e750(_e: &mut Engine, ecx: u32, _unused_1: u32, _unused_2: u32) -> u8 {
    (ecx >> 24) as u8
}

/// The registered form of [`fn_0065e750`]: the exe reads none of the words its callers push
/// (none, one or two of them, by caller) and only the `ECX` they leave, which the engine does
/// not model as a register. A caller that knows its `ECX` passes it as the first word; the
/// answer is that word's top byte, or 0 when no word comes (every caller hands the byte to a tag
/// parameter nothing reads).
fn registered_0065e750(e: &mut Engine, words: &[u32]) -> Ret {
    let ecx = words.first().copied().unwrap_or(0);
    u32::from(fn_0065e750(e, ecx, 0, 0)).into_ret()
}

/// `std::_Copy_opt` through unwrapped pointers for one element size (cdecl, six words: first,
/// last, destination and three tag bytes, of which it reads none). The frame's tag bytes are 0
/// at `EBP-5` and `EBP-6` and the uninitialised byte at `EBP-7` (whatever the frame holds; 0
/// here). Computes the end `destination + (last - first) / element * element` (`IDIV`, signed)
/// first, then runs `copy` with first, last, the destination unwrapped by `0065fe40`, and the
/// three bytes, and returns that end.
fn copy_elements(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    element: u32,
    copy: fn(&mut Engine, u32, u32, u32, u32, u32, u32) -> u32,
) -> u32 {
    let count = (last.wrapping_sub(first) as i32).wrapping_div(element as i32);
    let end = (count.wrapping_mul(element as i32) as u32).wrapping_add(destination);
    e.with_stack(0x1c, |e, frame| {
        // EBP is frame + 8: locals at frame + 0..8, the destination word at EBP + 0x10.
        let ebp = frame.addr() + 8;
        e.mem.set_u32(ebp + 0x10, destination);
        e.mem.set_u8(ebp - 5, 0);
        e.mem.set_u8(ebp - 6, 0);
        let byte_5 = e.mem.u8(ebp - 5) as u32;
        let byte_6 = e.mem.u8(ebp - 6) as u32;
        let byte_7 = e.mem.u8(ebp - 7) as u32;
        let unwrapped = e.call(UNCHECKED_0065FE40, &args![ebp + 0x10]).u32();
        copy(e, first, last, unwrapped, byte_7, byte_6, byte_5);
    });
    end
}

// Translated from 0065e760 (decompiled, FalloutNV.exe 1.4.0.525)
/// The copy for 0x1c-byte elements (`0065ea80`); see [`copy_elements`].
pub fn fn_0065e760(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    copy_elements(e, first, last, destination, 0x1c, fn_0065ea80)
}

// Translated from 0065e7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The copy for 0x58-byte elements (`0065eac0`); see [`copy_elements`].
pub fn fn_0065e7c0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    copy_elements(e, first, last, destination, 0x58, fn_0065eac0)
}

/// The range destructor's loop for one element size (cdecl, four words: first, last, the
/// allocator and the tag, which it does not read): `destroy(allocator, p)` for each element.
fn destroy_each(
    e: &mut Engine,
    first: u32,
    last: u32,
    allocator: u32,
    element: u32,
    destroy: fn(&mut Engine, Ptr, u32),
) {
    let mut p = first;
    while p != last {
        destroy(e, Ptr::new(allocator), p);
        p = p.wrapping_add(element);
    }
}

// Translated from 0065e820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The range destructor for 0xc-byte elements (`0065e9a0`); see [`destroy_each`].
pub fn fn_0065e820(e: &mut Engine, first: u32, last: u32, allocator: u32, _unused_4: u32) {
    destroy_each(e, first, last, allocator, 0xc, fn_0065e9a0);
}

// Translated from 0065e850 (decompiled, FalloutNV.exe 1.4.0.525)
/// The range destructor for 8-byte elements (`0065e9a0`).
pub fn fn_0065e850(e: &mut Engine, first: u32, last: u32, allocator: u32, _unused_4: u32) {
    destroy_each(e, first, last, allocator, 8, fn_0065e9a0);
}

// Translated from 0065e8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The range destructor for 0x34-byte elements (`0065ea00`).
pub fn fn_0065e8e0(e: &mut Engine, first: u32, last: u32, allocator: u32, _unused_4: u32) {
    destroy_each(e, first, last, allocator, 0x34, fn_0065ea00);
}

// Translated from 0065e910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The range destructor for 0x38-byte elements (`0065ea20`).
pub fn fn_0065e910(e: &mut Engine, first: u32, last: u32, allocator: u32, _unused_4: u32) {
    destroy_each(e, first, last, allocator, 0x38, fn_0065ea20);
}

// Translated from 0065e940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The range destructor for 0x1c-byte elements (`0065ea40`).
pub fn fn_0065e940(e: &mut Engine, first: u32, last: u32, allocator: u32, _unused_4: u32) {
    destroy_each(e, first, last, allocator, 0x1c, fn_0065ea40);
}

// Translated from 0065e970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The range destructor for 0x58-byte elements (`0065ea60`).
pub fn fn_0065e970(e: &mut Engine, first: u32, last: u32, allocator: u32, _unused_4: u32) {
    destroy_each(e, first, last, allocator, 0x58, fn_0065ea60);
}

/// `0040fbe0` (cdecl, one word; the map's name `Error` is a folded body): the element
/// destructor of the 8- and 0xc-byte allocators.
const DESTROY_0040FBE0: u32 = 0x0040_fbe0;

// Translated from 0065e9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `destroy(p)` for the 8- and 0xc-byte elements (thiscall, one word):
/// `0040fbe0(p)`.
pub fn fn_0065e9a0(e: &mut Engine, _this: Ptr, p: u32) {
    e.call(DESTROY_0040FBE0, &args![p]);
}

// Translated from 0065ea00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `destroy(p)` for 0x34-byte elements (thiscall, one word): `0065eb20(p)`.
pub fn fn_0065ea00(e: &mut Engine, _this: Ptr, p: u32) {
    fn_0065eb20(e, p);
}

// Translated from 0065ea20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `destroy(p)` for 0x38-byte elements: `0065eb30(p)`.
pub fn fn_0065ea20(e: &mut Engine, _this: Ptr, p: u32) {
    fn_0065eb30(e, p);
}

// Translated from 0065ea40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `destroy(p)` for 0x1c-byte elements: `0065eb40(p)`.
pub fn fn_0065ea40(e: &mut Engine, _this: Ptr, p: u32) {
    fn_0065eb40(e, p);
}

// Translated from 0065ea60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `destroy(p)` for 0x58-byte elements: `0065eb50(p)`.
pub fn fn_0065ea60(e: &mut Engine, _this: Ptr, p: u32) {
    fn_0065eb50(e, p);
}

/// The element-wise copy for one element size (cdecl, six words: first, last, destination and
/// three bytes it does not read): `construct(destination, first)` (thiscall on the
/// destination) for each element. Returns the destination's end.
fn construct_each(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    element: u32,
    construct: u32,
) -> u32 {
    let (mut source, mut target) = (first, destination);
    while source != last {
        e.call(construct, &args![target, source]);
        target = target.wrapping_add(element);
        source = source.wrapping_add(element);
    }
    target
}

// Translated from 0065ea80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The element-wise copy for 0x1c-byte elements (construct `0065eb60`); see
/// [`construct_each`].
pub fn fn_0065ea80(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    construct_each(e, first, last, destination, 0x1c, 0x0065_eb60)
}

// Translated from 0065eac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The element-wise copy for 0x58-byte elements (construct `0065eb90`).
pub fn fn_0065eac0(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    construct_each(e, first, last, destination, 0x58, 0x0065_eb90)
}

// Translated from 0065eb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Destroy(p)` for 0x34-byte elements (cdecl, one word): the element's deleting
/// destructor `0065ec70(p, 0)` (no delete).
pub fn fn_0065eb20(e: &mut Engine, p: u32) {
    fn_0065ec70(e, Ptr::new(p), 0);
}

// Translated from 0065eb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Destroy(p)` for 0x38-byte elements: `0065eca0(p, 0)`.
pub fn fn_0065eb30(e: &mut Engine, p: u32) {
    fn_0065eca0(e, Ptr::new(p), 0);
}

// Translated from 0065eb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Destroy(p)` for 0x1c-byte elements: `0065ecd0(p, 0)`.
pub fn fn_0065eb40(e: &mut Engine, p: u32) {
    fn_0065ecd0(e, Ptr::new(p), 0);
}

// Translated from 0065eb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Destroy(p)` for 0x58-byte elements: `0065ed00(p, 0)`.
pub fn fn_0065eb50(e: &mut Engine, p: u32) {
    fn_0065ed00(e, Ptr::new(p), 0);
}

/// A scalar deleting destructor (thiscall, one word: the flags, `RET 4`): `destroy(this)`,
/// then `operator delete(this)` when bit 0 is set. Returns `this`.
fn scalar_deleting(e: &mut Engine, this: Ptr, flags: u32, destroy: fn(&mut Engine, Ptr)) -> Ptr {
    destroy(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0065ec70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x34-byte element's scalar deleting destructor (`0065ed70`); see [`scalar_deleting`].
pub fn fn_0065ec70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, this, flags, fn_0065ed70)
}

// Translated from 0065eca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x38-byte element's scalar deleting destructor (`0065edd0`).
pub fn fn_0065eca0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, this, flags, fn_0065edd0)
}

// Translated from 0065ecd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x1c-byte element's scalar deleting destructor (`0065ee30`).
pub fn fn_0065ecd0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, this, flags, fn_0065ee30)
}

// Translated from 0065ed00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x58-byte element's scalar deleting destructor (`0065ee50`).
pub fn fn_0065ed00(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, this, flags, fn_0065ee50)
}

/// `0065c6a0`, `0044a040` (thiscall: member destructors) and `0044a540` (thiscall: the base
/// destructor), unnamed in the map.
const MEMBER_DTOR_0065C6A0: u32 = 0x0065_c6a0;
const MEMBER_DTOR_0044A040: u32 = 0x0044_a040;
const BASE_DTOR_0044A540: u32 = 0x0044_a540;

// Translated from 0065ed70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065ed70 - the compiler's exception frame (FS:[0] chain, state at EBP-4, handler 00eff888) is not translated
/// The 0x34-byte element's destructor (thiscall): `0065c6a0(this + 0x1c)`, then
/// `0044a540(this)`.
pub fn fn_0065ed70(e: &mut Engine, this: Ptr) {
    e.call(MEMBER_DTOR_0065C6A0, &args![this.addr() + 0x1c]);
    e.call(BASE_DTOR_0044A540, &args![this]);
}

// Translated from 0065edd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065edd0 - the compiler's exception frame (FS:[0] chain, state at EBP-4, handler 00eff888) is not translated
/// The 0x38-byte element's destructor (thiscall): `0044a040(this + 0x20)`, then
/// `0044a540(this)`.
pub fn fn_0065edd0(e: &mut Engine, this: Ptr) {
    e.call(MEMBER_DTOR_0044A040, &args![this.addr() + 0x20]);
    e.call(BASE_DTOR_0044A540, &args![this]);
}

// Translated from 0065ee30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x1c-byte element's destructor (thiscall): its member at `+4` (`0065ee70`).
pub fn fn_0065ee30(e: &mut Engine, this: Ptr) {
    fn_0065ee70(e, this.byte_add(4));
}

/// `` `eh vector destructor iterator' `` (`00ec5fce`, stdcall, four words: the array, the
/// element size, the count and the destructor).
const EH_VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;

// Translated from 0065ee50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x58-byte element's destructor (thiscall): destroys its array of three 0x18-byte
/// members at `+0x10` with `0065f060` through the `eh vector destructor iterator`.
pub fn fn_0065ee50(e: &mut Engine, this: Ptr) {
    e.call(
        EH_VECTOR_DESTRUCTOR_ITERATOR,
        &args![this.addr() + 0x10, 0x18u32, 3u32, 0x0065_f060u32],
    );
}

/// `0065f430` and `00657f60` (thiscall: member destructors), unnamed in the map.
const MEMBER_DTOR_0065F430: u32 = 0x0065_f430;
const MEMBER_DTOR_00657F60: u32 = 0x0065_7f60;

// Translated from 0065ee70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065ee70 - the compiler's exception frame (FS:[0] chain, state at EBP-4, handler 00eff1c8) is not translated
/// A destructor (thiscall): `0065f430(this)`, then the base's `0065cfb0(this)`.
pub fn fn_0065ee70(e: &mut Engine, this: Ptr) {
    e.call(MEMBER_DTOR_0065F430, &args![this]);
    fn_0065cfb0(e, this);
}

// Translated from 0065f060 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0065f060 - the compiler's exception frame (FS:[0] chain, state at EBP-4, handler 00eff1c8) is not translated
/// A destructor (thiscall): `00657f60(this)`, then the base's `0065cfb0(this)`.
pub fn fn_0065f060(e: &mut Engine, this: Ptr) {
    e.call(MEMBER_DTOR_00657F60, &args![this]);
    fn_0065cfb0(e, this);
}

/// `std::vector<T>::max_size()` for one element size (thiscall on the vector): the
/// allocator's `max_size` (`allocator_max_size`, thiscall on the allocator at `+8`).
fn vector_max_size(e: &mut Engine, this: Ptr, allocator_max_size: u32) -> u32 {
    e.call(allocator_max_size, &args![this.addr() + VECTOR_ALLOCATOR])
        .u32()
}

/// The allocator's `allocate(count)` for one element size (thiscall, one word, `RET 4`):
/// `std::_Allocate(count, 0)` (`allocate`, cdecl).
fn allocator_allocate(
    e: &mut Engine,
    count: u32,
    allocate: fn(&mut Engine, u32, u32) -> u32,
) -> u32 {
    allocate(e, count, 0)
}

/// The allocator's `max_size()` for one element size (thiscall, `this` not read): the
/// constant `0xFFFFFFFF / element` the compiler folded, or 1 when that is 0.
fn allocator_max_size(limit: u32) -> u32 {
    if limit > 0 {
        limit
    } else {
        1
    }
}

// Translated from 0065db90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::max_size` for 0xc-byte elements (`0065e020`); see [`vector_max_size`].
pub fn fn_0065db90(e: &mut Engine, this: Ptr) -> u32 {
    vector_max_size(e, this, 0x0065_e020)
}

// Translated from 0065dbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `allocate` for 0xc-byte elements (`0065e310`); see
/// [`allocator_allocate`].
pub fn fn_0065dbb0(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    allocator_allocate(e, count, fn_0065e310)
}

// Translated from 0065dbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::max_size` for 8-byte elements (`0065e050`).
pub fn fn_0065dbd0(e: &mut Engine, this: Ptr) -> u32 {
    vector_max_size(e, this, 0x0065_e050)
}

// Translated from 0065dc20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `allocate` for 8-byte elements (`0065e3a0`).
pub fn fn_0065dc20(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    allocator_allocate(e, count, fn_0065e3a0)
}

// Translated from 0065dc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::max_size` for 0x20-byte elements (`0065e080`).
pub fn fn_0065dc40(e: &mut Engine, this: Ptr) -> u32 {
    vector_max_size(e, this, 0x0065_e080)
}

// Translated from 0065dc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Destroy` for 0x20-byte elements (`0065e3f0`); see [`vector_destroy`].
pub fn fn_0065dc60(e: &mut Engine, this: Ptr, first: u32, last: u32) {
    vector_destroy(e, this, first, last, fn_0065e3f0);
}

// Translated from 0065dd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `allocate` for 0x20-byte elements (`0065e430`).
pub fn fn_0065dd50(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    allocator_allocate(e, count, fn_0065e430)
}

// Translated from 0065dd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::max_size` for 0x2c-byte elements (`0065e0b0`).
pub fn fn_0065dd70(e: &mut Engine, this: Ptr) -> u32 {
    vector_max_size(e, this, 0x0065_e0b0)
}

// Translated from 0065dd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::_Destroy` for 0x2c-byte elements (`0065e480`).
pub fn fn_0065dd90(e: &mut Engine, this: Ptr, first: u32, last: u32) {
    vector_destroy(e, this, first, last, fn_0065e480);
}

// Translated from 0065ddc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `allocate` for 0x2c-byte elements (`0065e4c0`).
pub fn fn_0065ddc0(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    allocator_allocate(e, count, fn_0065e4c0)
}

// Translated from 0065dde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::max_size` for 0x34-byte elements (`0065e0e0`).
pub fn fn_0065dde0(e: &mut Engine, this: Ptr) -> u32 {
    vector_max_size(e, this, 0x0065_e0e0)
}

// Translated from 0065de40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `allocate` for 0x34-byte elements (`0065e550`).
pub fn fn_0065de40(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    allocator_allocate(e, count, fn_0065e550)
}

// Translated from 0065de60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::max_size` for 0x38-byte elements (`0065e110`).
pub fn fn_0065de60(e: &mut Engine, this: Ptr) -> u32 {
    vector_max_size(e, this, 0x0065_e110)
}

// Translated from 0065deb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `allocate` for 0x38-byte elements (`0065e5e0`).
pub fn fn_0065deb0(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    allocator_allocate(e, count, fn_0065e5e0)
}

// Translated from 0065def0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::max_size` for 0x1c-byte elements (`0065e140`).
pub fn fn_0065def0(e: &mut Engine, this: Ptr) -> u32 {
    vector_max_size(e, this, 0x0065_e140)
}

// Translated from 0065df40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `allocate` for 0x1c-byte elements (`0065e670`).
pub fn fn_0065df40(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    allocator_allocate(e, count, fn_0065e670)
}

// Translated from 0065df60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::max_size` for 0x58-byte elements (`0065e170`).
pub fn fn_0065df60(e: &mut Engine, this: Ptr) -> u32 {
    vector_max_size(e, this, 0x0065_e170)
}

// Translated from 0065dfb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `allocate` for 0x58-byte elements (`0065e700`).
pub fn fn_0065dfb0(e: &mut Engine, _this: Ptr, count: u32) -> u32 {
    allocator_allocate(e, count, fn_0065e700)
}

// Translated from 0065e020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `max_size` for 0xc-byte elements: `0x15555555`; see
/// [`allocator_max_size`].
pub fn fn_0065e020(_e: &mut Engine, _this: Ptr) -> u32 {
    allocator_max_size(0x1555_5555)
}

// Translated from 0065e050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `max_size` for 8-byte elements: `0x1FFFFFFF`.
pub fn fn_0065e050(_e: &mut Engine, _this: Ptr) -> u32 {
    allocator_max_size(0x1fff_ffff)
}

// Translated from 0065e080 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `max_size` for 0x20-byte elements: `0x07FFFFFF`.
pub fn fn_0065e080(_e: &mut Engine, _this: Ptr) -> u32 {
    allocator_max_size(0x07ff_ffff)
}

// Translated from 0065e0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `max_size` for 0x2c-byte elements: `0x05D1745D`.
pub fn fn_0065e0b0(_e: &mut Engine, _this: Ptr) -> u32 {
    allocator_max_size(0x05d1_745d)
}

// Translated from 0065e0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `max_size` for 0x34-byte elements: `0x04EC4EC4`.
pub fn fn_0065e0e0(_e: &mut Engine, _this: Ptr) -> u32 {
    allocator_max_size(0x04ec_4ec4)
}

// Translated from 0065e110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `max_size` for 0x38-byte elements: `0x04924924`.
pub fn fn_0065e110(_e: &mut Engine, _this: Ptr) -> u32 {
    allocator_max_size(0x0492_4924)
}

// Translated from 0065e140 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `max_size` for 0x1c-byte elements: `0x09249249`.
pub fn fn_0065e140(_e: &mut Engine, _this: Ptr) -> u32 {
    allocator_max_size(0x0924_9249)
}

// Translated from 0065e170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `max_size` for 0x58-byte elements: `0x02E8BA2E`.
pub fn fn_0065e170(_e: &mut Engine, _this: Ptr) -> u32 {
    allocator_max_size(0x02e8_ba2e)
}

// Translated from 0065e3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Destroy_range` for 0x20-byte elements (`0065e880`); see [`destroy_range`].
pub fn fn_0065e3f0(e: &mut Engine, first: u32, last: u32, allocator: u32) {
    destroy_range(e, first, last, allocator, fn_0065e880);
}

// Translated from 0065e430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Allocate` for 0x20-byte elements; see [`allocate_elements`].
pub fn fn_0065e430(e: &mut Engine, count: u32, _unused_2: u32) -> u32 {
    allocate_elements(e, count, 0x20)
}

// Translated from 0065e480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Destroy_range` for 0x2c-byte elements (`0065e8b0`).
pub fn fn_0065e480(e: &mut Engine, first: u32, last: u32, allocator: u32) {
    destroy_range(e, first, last, allocator, fn_0065e8b0);
}

// Translated from 0065e4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Allocate` for 0x2c-byte elements.
pub fn fn_0065e4c0(e: &mut Engine, count: u32, _unused_2: u32) -> u32 {
    allocate_elements(e, count, 0x2c)
}

// Translated from 0065e880 (decompiled, FalloutNV.exe 1.4.0.525)
/// The range destructor for 0x20-byte elements (`0065e9c0`); see [`destroy_each`].
pub fn fn_0065e880(e: &mut Engine, first: u32, last: u32, allocator: u32, _unused_4: u32) {
    destroy_each(e, first, last, allocator, 0x20, fn_0065e9c0);
}

// Translated from 0065e8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The range destructor for 0x2c-byte elements (`0065e9e0`).
pub fn fn_0065e8b0(e: &mut Engine, first: u32, last: u32, allocator: u32, _unused_4: u32) {
    destroy_each(e, first, last, allocator, 0x2c, fn_0065e9e0);
}

// Translated from 0065e9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `destroy(p)` for 0x20-byte elements (thiscall, one word): `0065eb00(p)`.
pub fn fn_0065e9c0(e: &mut Engine, _this: Ptr, p: u32) {
    fn_0065eb00(e, p);
}

// Translated from 0065e9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The allocator's `destroy(p)` for 0x2c-byte elements: `0065eb10(p)`.
pub fn fn_0065e9e0(e: &mut Engine, _this: Ptr, p: u32) {
    fn_0065eb10(e, p);
}

// Translated from 0065eb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Destroy(p)` for 0x20-byte elements (cdecl, one word): `0065ec10(p, 0)`.
pub fn fn_0065eb00(e: &mut Engine, p: u32) {
    fn_0065ec10(e, Ptr::new(p), 0);
}

// Translated from 0065eb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::_Destroy(p)` for 0x2c-byte elements: `0065ec40(p, 0)`.
pub fn fn_0065eb10(e: &mut Engine, p: u32) {
    fn_0065ec40(e, Ptr::new(p), 0);
}

// Translated from 0065ec10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x20-byte element's scalar deleting destructor (`0065ed30`); see
/// [`scalar_deleting`].
pub fn fn_0065ec10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, this, flags, fn_0065ed30)
}

// Translated from 0065ec40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x2c-byte element's scalar deleting destructor (`0065ed50`).
pub fn fn_0065ec40(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, this, flags, fn_0065ed50)
}

// Translated from 0065ed30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x20-byte element's destructor (thiscall): `0044a540` on its member at `+4`.
pub fn fn_0065ed30(e: &mut Engine, this: Ptr) {
    e.call(BASE_DTOR_0044A540, &args![this.addr() + 4]);
}

// Translated from 0065ed50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x2c-byte element's destructor (thiscall): `0044a540` on its member at `+0x10`.
pub fn fn_0065ed50(e: &mut Engine, this: Ptr) {
    e.call(BASE_DTOR_0044A540, &args![this.addr() + 0x10]);
}

// Translated from 0065eb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x1c-byte element's copy constructor (thiscall, one word, `RET 4`): the float at
/// `+0` (`FLD`/`FSTP`), then the vector member at `+4` assigned from the other's
/// ([`fn_0065eed0`]). Returns `this`.
pub fn fn_0065eb60(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let value = x87_quiet_bits(e.mem.u32(other.addr()));
    e.mem.set_u32(this.addr(), value);
    fn_0065eed0(e, this.byte_add(4), other.byte_add(4));
    this
}

// Translated from 0065eb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The 0x58-byte element's copy constructor (thiscall, one word, `RET 4`): the float at
/// `+0` (`FLD`/`FSTP`) and the words at `+4`, `+8` and `+0xC`, then each of the three
/// 0x18-byte vector members at `+0x10` assigned from the other's ([`fn_0065f0c0`]).
/// Returns `this`.
pub fn fn_0065eb90(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let (to, from) = (this.addr(), other.addr());
    let value = x87_quiet_bits(e.mem.u32(from));
    e.mem.set_u32(to, value);
    for word in [4u32, 8, 0xc] {
        let value = e.mem.u32(from.wrapping_add(word));
        e.mem.set_u32(to.wrapping_add(word), value);
    }
    for i in 0..3u32 {
        let offset = i * 0x18 + 0x10;
        fn_0065f0c0(
            e,
            Ptr::new(to.wrapping_add(offset)),
            Ptr::new(from.wrapping_add(offset)),
        );
    }
    this
}

/// The callees of one `std::vector<T>::operator=` instance: `size()` and `capacity()`
/// (thiscall), `clear()` (thiscall), `_Buy(count)` (thiscall, `AL`), `_Destroy(first,
/// last)` (thiscall), `std::copy(first, last, destination)` (cdecl, answers the end) and
/// `_Ucopy(first, last, destination)` (thiscall, answers the end); the element size and
/// whether `_Tidy`'s element count divides (by `IDIV`) the byte size.
struct VectorAssign {
    size: u32,
    capacity: u32,
    clear: u32,
    buy: u32,
    destroy: u32,
    copy: u32,
    uninitialized_copy: u32,
    element: u32,
}

/// `std::vector<T>::operator=(other)` (thiscall, one word, `RET 4`), in the exe's order:
/// nothing for itself; an empty `other` clears `this`; a smaller or equal `other` is copied
/// over the start and the rest destroyed (`_Mylast = _Myfirst + other.size() * element`);
/// one that fits the capacity is copied over the existing elements and the rest
/// copy-constructed after them; otherwise the buffer is destroyed and freed (`0064df10`
/// with the element count) and, when `_Buy(other.size())` succeeds, `other` is
/// copy-constructed into the new one. Returns `this`.
fn vector_assign(e: &mut Engine, this: Ptr, other: Ptr, f: &VectorAssign) -> Ptr {
    let (mine, theirs) = (this.addr(), other.addr());
    if mine == theirs {
        return this;
    }
    let first = |e: &Engine, v: u32| e.mem.u32(v + VECTOR_FIRST);
    let last = |e: &Engine, v: u32| e.mem.u32(v + VECTOR_LAST);
    if e.call(f.size, &args![other]).u32() == 0 {
        e.call(f.clear, &args![this]);
        return this;
    }
    let other_size = e.call(f.size, &args![other]).u32();
    if other_size <= e.call(f.size, &args![this]).u32() {
        let (from, to, at) = (first(e, theirs), last(e, theirs), first(e, mine));
        let end = e.call(f.copy, &args![from, to, at]).u32();
        let old_last = last(e, mine);
        e.call(f.destroy, &args![this, end, old_last]);
        let count = e.call(f.size, &args![other]).u32();
        let new_last = count.wrapping_mul(f.element).wrapping_add(first(e, mine));
        e.mem.set_u32(mine + VECTOR_LAST, new_last);
        return this;
    }
    let other_size = e.call(f.size, &args![other]).u32();
    if other_size <= e.call(f.capacity, &args![this]).u32() {
        let count = e.call(f.size, &args![this]).u32();
        let middle = count.wrapping_mul(f.element).wrapping_add(first(e, theirs));
        let (from, at) = (first(e, theirs), first(e, mine));
        e.call(f.copy, &args![from, middle, at]);
        let (to, at) = (last(e, theirs), last(e, mine));
        let end = e
            .call(f.uninitialized_copy, &args![this, middle, to, at])
            .u32();
        e.mem.set_u32(mine + VECTOR_LAST, end);
        return this;
    }
    if first(e, mine) != 0 {
        let (old_first, old_last) = (first(e, mine), last(e, mine));
        e.call(f.destroy, &args![this, old_first, old_last]);
        let bytes = e.mem.u32(mine + VECTOR_END).wrapping_sub(first(e, mine));
        let elements = if f.element == 1 {
            bytes
        } else {
            (bytes as i32).wrapping_div(f.element as i32) as u32
        };
        let old_first = first(e, mine);
        e.call(
            DEALLOCATE_0064DF10,
            &args![mine + VECTOR_ALLOCATOR, old_first, elements],
        );
    }
    let count = e.call(f.size, &args![other]).u32();
    if e.call(f.buy, &args![this, count]).u8() != 0 {
        let (from, to, at) = (first(e, theirs), last(e, theirs), first(e, mine));
        let end = e
            .call(f.uninitialized_copy, &args![this, from, to, at])
            .u32();
        e.mem.set_u32(mine + VECTOR_LAST, end);
    }
    this
}

/// The callees of [`fn_0065eed0`] (6-byte elements).
const ASSIGN_6: VectorAssign = VectorAssign {
    size: 0x0065_7bc0,
    capacity: 0x0065_f2e0,
    clear: 0x0065_f320,
    buy: 0x0065_f380,
    destroy: 0x0065_f400,
    copy: 0x0065_f820,
    uninitialized_copy: 0x0065_f890,
    element: 6,
};

/// The callees of [`fn_0065f0c0`] (1-byte elements).
const ASSIGN_1: VectorAssign = VectorAssign {
    size: 0x0065_cc50,
    capacity: 0x0065_f4b0,
    clear: 0x0065_f4f0,
    buy: 0x0065_7ee0,
    destroy: 0x0065_f2b0,
    copy: 0x0065_f8c0,
    uninitialized_copy: 0x0065_f930,
    element: 1,
};

// Translated from 0065eed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::operator=` for 6-byte elements (callees in [`ASSIGN_6`]; the freed
/// buffer's element count by `IDIV 6`); see [`vector_assign`].
pub fn fn_0065eed0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    vector_assign(e, this, other, &ASSIGN_6)
}

// Translated from 0065f0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::vector<T>::operator=` for 1-byte elements (callees in [`ASSIGN_1`]; the freed
/// buffer's element count is its byte size); see [`vector_assign`].
pub fn fn_0065f0c0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    vector_assign(e, this, other, &ASSIGN_1)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_cfb0, fn_0065cfb0(Ptr)),
        entry!(0x0065_cfd0, fn_0065cfd0(Ptr, u32) -> bool),
        entry!(0x0065_d050, fn_0065d050(Ptr)),
        entry!(0x0065_d0d0, fn_0065d0d0(Ptr, u32) -> bool),
        entry!(0x0065_d150, fn_0065d150(Ptr)),
        entry!(0x0065_d1c0, fn_0065d1c0(Ptr, u32) -> bool),
        entry!(0x0065_d240, fn_0065d240(Ptr)),
        entry!(0x0065_d2b0, fn_0065d2b0(Ptr, u32) -> bool),
        entry!(0x0065_d330, fn_0065d330(Ptr)),
        entry!(0x0065_d3a0, fn_0065d3a0(Ptr, u32) -> Ptr),
        entry!(0x0065_d3e0, fn_0065d3e0(Ptr, u32) -> bool),
        entry!(0x0065_d460, fn_0065d460(Ptr)),
        entry!(0x0065_d4e0, fn_0065d4e0(Ptr, u32) -> bool),
        entry!(0x0065_d560, fn_0065d560(Ptr)),
        entry!(0x0065_d5e0, fn_0065d5e0(Ptr, u32) -> bool),
        entry!(0x0065_d660, fn_0065d660(Ptr)),
        entry!(0x0065_d6e0, fn_0065d6e0(Ptr)),
        entry!(0x0065_d740, fn_0065d740(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x0065_d7e0, fn_0065d7e0(Ptr, u32) -> bool),
        entry!(0x0065_d860, fn_0065d860(Ptr)),
        entry!(0x0065_d8e0, fn_0065d8e0(Ptr, Ptr, u32, u32, u32, u32) -> Ptr),
        entry!(0x0065_d980, fn_0065d980(Ptr, u32) -> bool),
        entry!(0x0065_da00, fn_0065da00(Ptr)),
        entry!(0x0065_da80, fn_0065da80(Ptr, u32, u32) -> Ptr),
        entry!(0x0065_dae0, fn_0065dae0(Ptr) -> Ptr),
        entry!(0x0065_db40, fn_0065db40(Ptr) -> Ptr),
        entry!(0x0065_db60, fn_0065db60(Ptr, u32, u32)),
        entry!(0x0065_dbf0, fn_0065dbf0(Ptr, u32, u32)),
        entry!(0x0065_dc90, fn_0065dc90(Ptr, u32) -> Ptr),
        entry!(0x0065_de00, fn_0065de00(Ptr, u32, u32)),
        entry!(0x0065_de30, fn_0065de30(Ptr, u32) -> Ptr),
        entry!(0x0065_de80, fn_0065de80(Ptr, u32, u32)),
        entry!(0x0065_ded0, fn_0065ded0(Ptr, Ptr, u32, u32) -> Ptr),
        entry!(0x0065_df10, fn_0065df10(Ptr, u32, u32)),
        entry!(0x0065_df80, fn_0065df80(Ptr, u32, u32)),
        entry!(0x0065_dfd0, fn_0065dfd0(Ptr, Ptr) -> u32),
        entry!(0x0065_e000, fn_0065e000(Ptr, u32, u32) -> Ptr),
        entry!(0x0065_e1a0, fn_0065e1a0(Ptr, Ptr) -> bool),
        entry!(0x0065_e1f0, fn_0065e1f0(u32, u32, u32) -> u32),
        entry!(0x0065_e260, fn_0065e260(u32, u32, u32) -> u32),
        entry!(0x0065_e2d0, fn_0065e2d0(u32, u32, u32)),
        entry!(0x0065_e310, fn_0065e310(u32, u32) -> u32),
        entry!(0x0065_e360, fn_0065e360(u32, u32, u32)),
        entry!(0x0065_e3a0, fn_0065e3a0(u32, u32) -> u32),
        entry!(0x0065_e510, fn_0065e510(u32, u32, u32)),
        entry!(0x0065_e550, fn_0065e550(u32, u32) -> u32),
        entry!(0x0065_e5a0, fn_0065e5a0(u32, u32, u32)),
        entry!(0x0065_e5e0, fn_0065e5e0(u32, u32) -> u32),
        entry!(0x0065_e630, fn_0065e630(u32, u32, u32)),
        entry!(0x0065_e670, fn_0065e670(u32, u32) -> u32),
        entry!(0x0065_e6c0, fn_0065e6c0(u32, u32, u32)),
        entry!(0x0065_e700, fn_0065e700(u32, u32) -> u32),
        (0x0065_e750, registered_0065e750 as AbiFn),
        entry!(0x0065_e760, fn_0065e760(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0065_e7c0, fn_0065e7c0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0065_e820, fn_0065e820(u32, u32, u32, u32)),
        entry!(0x0065_e850, fn_0065e850(u32, u32, u32, u32)),
        entry!(0x0065_e8e0, fn_0065e8e0(u32, u32, u32, u32)),
        entry!(0x0065_e910, fn_0065e910(u32, u32, u32, u32)),
        entry!(0x0065_e940, fn_0065e940(u32, u32, u32, u32)),
        entry!(0x0065_e970, fn_0065e970(u32, u32, u32, u32)),
        entry!(0x0065_e9a0, fn_0065e9a0(Ptr, u32)),
        entry!(0x0065_ea00, fn_0065ea00(Ptr, u32)),
        entry!(0x0065_ea20, fn_0065ea20(Ptr, u32)),
        entry!(0x0065_ea40, fn_0065ea40(Ptr, u32)),
        entry!(0x0065_ea60, fn_0065ea60(Ptr, u32)),
        entry!(0x0065_ea80, fn_0065ea80(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0065_eac0, fn_0065eac0(u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x0065_eb20, fn_0065eb20(u32)),
        entry!(0x0065_eb30, fn_0065eb30(u32)),
        entry!(0x0065_eb40, fn_0065eb40(u32)),
        entry!(0x0065_eb50, fn_0065eb50(u32)),
        entry!(0x0065_ec70, fn_0065ec70(Ptr, u32) -> Ptr),
        entry!(0x0065_eca0, fn_0065eca0(Ptr, u32) -> Ptr),
        entry!(0x0065_ecd0, fn_0065ecd0(Ptr, u32) -> Ptr),
        entry!(0x0065_ed00, fn_0065ed00(Ptr, u32) -> Ptr),
        entry!(0x0065_ed70, fn_0065ed70(Ptr)),
        entry!(0x0065_edd0, fn_0065edd0(Ptr)),
        entry!(0x0065_ee30, fn_0065ee30(Ptr)),
        entry!(0x0065_ee50, fn_0065ee50(Ptr)),
        entry!(0x0065_ee70, fn_0065ee70(Ptr)),
        entry!(0x0065_f060, fn_0065f060(Ptr)),
        entry!(0x0065_db90, fn_0065db90(Ptr) -> u32),
        entry!(0x0065_dbb0, fn_0065dbb0(Ptr, u32) -> u32),
        entry!(0x0065_dbd0, fn_0065dbd0(Ptr) -> u32),
        entry!(0x0065_dc20, fn_0065dc20(Ptr, u32) -> u32),
        entry!(0x0065_dc40, fn_0065dc40(Ptr) -> u32),
        entry!(0x0065_dc60, fn_0065dc60(Ptr, u32, u32)),
        entry!(0x0065_dd50, fn_0065dd50(Ptr, u32) -> u32),
        entry!(0x0065_dd70, fn_0065dd70(Ptr) -> u32),
        entry!(0x0065_dd90, fn_0065dd90(Ptr, u32, u32)),
        entry!(0x0065_ddc0, fn_0065ddc0(Ptr, u32) -> u32),
        entry!(0x0065_dde0, fn_0065dde0(Ptr) -> u32),
        entry!(0x0065_de40, fn_0065de40(Ptr, u32) -> u32),
        entry!(0x0065_de60, fn_0065de60(Ptr) -> u32),
        entry!(0x0065_deb0, fn_0065deb0(Ptr, u32) -> u32),
        entry!(0x0065_def0, fn_0065def0(Ptr) -> u32),
        entry!(0x0065_df40, fn_0065df40(Ptr, u32) -> u32),
        entry!(0x0065_df60, fn_0065df60(Ptr) -> u32),
        entry!(0x0065_dfb0, fn_0065dfb0(Ptr, u32) -> u32),
        entry!(0x0065_e020, fn_0065e020(Ptr) -> u32),
        entry!(0x0065_e050, fn_0065e050(Ptr) -> u32),
        entry!(0x0065_e080, fn_0065e080(Ptr) -> u32),
        entry!(0x0065_e0b0, fn_0065e0b0(Ptr) -> u32),
        entry!(0x0065_e0e0, fn_0065e0e0(Ptr) -> u32),
        entry!(0x0065_e110, fn_0065e110(Ptr) -> u32),
        entry!(0x0065_e140, fn_0065e140(Ptr) -> u32),
        entry!(0x0065_e170, fn_0065e170(Ptr) -> u32),
        entry!(0x0065_e3f0, fn_0065e3f0(u32, u32, u32)),
        entry!(0x0065_e430, fn_0065e430(u32, u32) -> u32),
        entry!(0x0065_e480, fn_0065e480(u32, u32, u32)),
        entry!(0x0065_e4c0, fn_0065e4c0(u32, u32) -> u32),
        entry!(0x0065_e880, fn_0065e880(u32, u32, u32, u32)),
        entry!(0x0065_e8b0, fn_0065e8b0(u32, u32, u32, u32)),
        entry!(0x0065_e9c0, fn_0065e9c0(Ptr, u32)),
        entry!(0x0065_e9e0, fn_0065e9e0(Ptr, u32)),
        entry!(0x0065_eb00, fn_0065eb00(u32)),
        entry!(0x0065_eb10, fn_0065eb10(u32)),
        entry!(0x0065_ec10, fn_0065ec10(Ptr, u32) -> Ptr),
        entry!(0x0065_ec40, fn_0065ec40(Ptr, u32) -> Ptr),
        entry!(0x0065_ed30, fn_0065ed30(Ptr)),
        entry!(0x0065_ed50, fn_0065ed50(Ptr)),
        entry!(0x0065_eb60, fn_0065eb60(Ptr, Ptr) -> Ptr),
        entry!(0x0065_eb90, fn_0065eb90(Ptr, Ptr) -> Ptr),
        entry!(0x0065_eed0, fn_0065eed0(Ptr, Ptr) -> Ptr),
        entry!(0x0065_f0c0, fn_0065f0c0(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod thunk_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn fwd_a03_0065cfb0_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0065_d6e0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0065_cfb0, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0065_d6e0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod b001_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// The vector's own deallocations (on its allocator at `+8`), leaving out those of the
    /// elements' destructors.
    fn own_deallocations(e: &Engine, base: u32) -> Vec<Vec<u32>> {
        calls_to(e, DEALLOCATE_0064DF10)
            .into_iter()
            .filter(|w| w[0] == base + 8)
            .collect()
    }

    /// The external leaves the element destructors and iterator checks end in, all doubled.
    const LEAVES: [u32; 17] = [
        DESTROY_0040FBE0,
        MEMBER_DTOR_0065C6A0,
        MEMBER_DTOR_0044A040,
        BASE_DTOR_0044A540,
        MEMBER_DTOR_0065F430,
        MEMBER_DTOR_00657F60,
        DEALLOCATE_0064DF10,
        EMPTY_BASE_00483710,
        EH_VECTOR_DESTRUCTOR_ITERATOR,
        OPERATOR_DELETE,
        0x0064_dee0,
        0x0065_dc60,
        0x0065_dd90,
        INVALID_PARAMETER_NOINFO,
        ITERATOR_BASE_INIT_0064DE50,
        ITERATOR_ADOPT_0044AC20,
        LENGTH_ERROR_0065F640,
    ];

    fn engine() -> Engine {
        let mut e = Engine::new();
        for &leaf in LEAVES.iter() {
            e.register(leaf, |_, a| a.first().copied().unwrap_or(0).into_ret());
        }
        e
    }

    fn vector(e: &mut Engine) -> Ptr {
        let this = e.mem.alloc(0x18);
        e.mem.write(this, &[0x55; 0x18]);
        Ptr::new(this)
    }

    fn check_buy(address: u32, max_size: u32, allocate: u32, element: u32) {
        let mut e = engine();
        e.register(max_size, |_, _| 0x100u32.into_ret());
        e.register(allocate, |_, a| (a[1] * 0x1000).into_ret());
        let this = vector(&mut e);
        let base = this.addr();
        e.call_log = Some(vec![]);
        assert!(!e.call(address, &args![this, 0u32]).bool());
        assert_eq!(e.mem.bytes(base + 0xc, 12), vec![0; 12]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        e.call_log = Some(vec![]);
        assert!(e.call(address, &args![this, 0x101u32]).bool());
        assert_eq!(calls_to(&e, LENGTH_ERROR_0065F640).len(), 1);
        assert!(calls_to(&e, allocate).is_empty());
        assert_eq!(e.mem.bytes(base + 0xc, 12), vec![0; 12]);
        e.call_log = Some(vec![]);
        assert!(e.call(address, &args![this, 3u32]).bool());
        assert_eq!(calls_to(&e, max_size), vec![vec![base]]);
        assert_eq!(calls_to(&e, allocate), vec![vec![base + 8, 3]]);
        assert_eq!(e.mem.u32(base + 0xc), 0x3000);
        assert_eq!(e.mem.u32(base + 0x10), 0x3000);
        assert_eq!(e.mem.u32(base + 0x14), 0x3000 + 3 * element);
    }

    #[test]
    fn buy_0065cfd0() {
        check_buy(0x0065_cfd0, 0x0065_db90, 0x0065_dbb0, 0xc);
    }
    #[test]
    fn buy_0065d0d0() {
        check_buy(0x0065_d0d0, 0x0064_de90, 0x0064_df30, 0x10);
    }
    #[test]
    fn buy_0065d1c0() {
        check_buy(0x0065_d1c0, 0x0065_dbd0, 0x0065_dc20, 8);
    }
    #[test]
    fn buy_0065d2b0() {
        check_buy(0x0065_d2b0, 0x0065_dc40, 0x0065_dd50, 0x20);
    }
    #[test]
    fn buy_0065d3e0() {
        check_buy(0x0065_d3e0, 0x0065_dd70, 0x0065_ddc0, 0x2c);
    }
    #[test]
    fn buy_0065d4e0() {
        check_buy(0x0065_d4e0, 0x0065_dde0, 0x0065_de40, 0x34);
    }
    #[test]
    fn buy_0065d5e0() {
        check_buy(0x0065_d5e0, 0x0065_de60, 0x0065_deb0, 0x38);
    }
    #[test]
    fn buy_0065d7e0() {
        check_buy(0x0065_d7e0, 0x0065_def0, 0x0065_df40, 0x1c);
    }
    #[test]
    fn buy_0065d980() {
        check_buy(0x0065_d980, 0x0065_df60, 0x0065_dfb0, 0x58);
    }

    /// `_Tidy`: nothing but the clearing for a null buffer; otherwise the elements destroyed
    /// (seen through `seen`, the leaf call each element makes, by its first word) and the
    /// buffer deallocated with the element count. `negative` is the count the code computes for
    /// a buffer whose end is 0x14 bytes below its start (`IDIV` truncates, `SAR` floors).
    fn check_tidy(address: u32, element: u32, seen: u32, offset: u32, negative: i32) {
        let mut e = engine();
        let this = vector(&mut e);
        let base = this.addr();
        e.mem.write(base + 0xc, &[0; 12]);
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        assert!(own_deallocations(&e, base).is_empty());
        let first = e.mem.alloc(element * 3);
        e.mem.set_u32(base + 0xc, first);
        e.mem.set_u32(base + 0x10, first + 2 * element);
        e.mem.set_u32(base + 0x14, first + 3 * element);
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        let firsts: Vec<u32> = calls_to(&e, seen).iter().map(|w| w[0]).collect();
        assert_eq!(firsts, vec![first + offset, first + element + offset]);
        assert_eq!(own_deallocations(&e, base), vec![vec![base + 8, first, 3]]);
        assert_eq!(e.mem.bytes(base + 0xc, 12), vec![0; 12]);
        e.mem.set_u32(base + 0xc, first);
        e.mem.set_u32(base + 0x10, first);
        e.mem.set_u32(base + 0x14, first - 0x14);
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        assert_eq!(
            own_deallocations(&e, base),
            vec![vec![base + 8, first, negative as u32]]
        );
    }

    /// The external destroy `0064dee0` and the like are called once with the vector and the
    /// range; count them by the first word.
    fn check_tidy_external(address: u32, destroy: u32, element: u32, negative: i32) {
        let mut e = engine();
        let this = vector(&mut e);
        let base = this.addr();
        let first = e.mem.alloc(element * 3);
        e.mem.set_u32(base + 0xc, first);
        e.mem.set_u32(base + 0x10, first + 2 * element);
        e.mem.set_u32(base + 0x14, first + 3 * element);
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        assert_eq!(
            calls_to(&e, destroy),
            vec![vec![base, first, first + 2 * element]]
        );
        assert_eq!(own_deallocations(&e, base), vec![vec![base + 8, first, 3]]);
        e.mem.set_u32(base + 0xc, first);
        e.mem.set_u32(base + 0x10, first);
        e.mem.set_u32(base + 0x14, first - 0x14);
        e.call_log = Some(vec![]);
        e.call(address, &args![this]);
        assert_eq!(
            own_deallocations(&e, base),
            vec![vec![base + 8, first, negative as u32]]
        );
        assert_eq!(e.mem.bytes(base + 0xc, 12), vec![0; 12]);
    }

    #[test]
    fn tidy_0065d050() {
        check_tidy(0x0065_d050, 0xc, DESTROY_0040FBE0, 0, -1);
    }
    #[test]
    fn tidy_0065d150() {
        check_tidy_external(0x0065_d150, 0x0064_dee0, 0x10, -2);
    }
    #[test]
    fn tidy_0065d240() {
        check_tidy(0x0065_d240, 8, DESTROY_0040FBE0, 0, -3);
    }
    #[test]
    fn tidy_0065d330() {
        check_tidy_external(0x0065_d330, 0x0065_dc60, 0x20, -1);
    }
    #[test]
    fn tidy_0065d460() {
        check_tidy_external(0x0065_d460, 0x0065_dd90, 0x2c, 0);
    }
    #[test]
    fn tidy_0065d560() {
        check_tidy(0x0065_d560, 0x34, MEMBER_DTOR_0065C6A0, 0x1c, 0);
    }
    #[test]
    fn tidy_0065d660() {
        check_tidy(0x0065_d660, 0x38, MEMBER_DTOR_0044A040, 0x20, 0);
    }
    #[test]
    fn tidy_0065d860() {
        check_tidy(0x0065_d860, 0x1c, MEMBER_DTOR_0065F430, 4, 0);
    }
    #[test]
    fn tidy_0065da00() {
        check_tidy(0x0065_da00, 0x58, EH_VECTOR_DESTRUCTOR_ITERATOR, 0x10, 0);
    }

    /// Doubles for `0065dc90`'s callees: `006e6da0` answers `block`, `008c71b0` its object.
    fn constructor_doubles(e: &mut Engine, block: u32) {
        e.register(BASE_CTOR_006815C0, |_, a| a[0].into_ret());
        e.register(ALLOCATOR_0044ABD0, |_, _| 0x21u32.into_ret());
        e.register_double(CONSTRUCT_006E6DA0, move |_, _| block.into_ret());
        e.register(CONSTRUCT_008C71B0, |_, a| (a[0] + 1).into_ret());
    }

    #[test]
    fn constructor_0065d3a0_builds_through_0065dc90() {
        let mut e = engine();
        constructor_doubles(&mut e, 0x7000);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_d3a0, &args![this, 0x99u32]).u32(), this);
        assert_eq!(calls_to(&e, BASE_CTOR_006815C0), vec![vec![this]]);
        assert_eq!(calls_to(&e, ALLOCATOR_0044ABD0), vec![vec![this + 4, 1]]);
        assert_eq!(calls_to(&e, CONSTRUCT_006E6DA0), vec![vec![4, 0x21]]);
        assert_eq!(calls_to(&e, CONSTRUCT_008C71B0), vec![vec![0x7000, this]]);
        assert_eq!(e.mem.u32(this), 0x7001);
    }

    #[test]
    fn constructor_0065dc90_stores_zero_when_the_allocation_fails() {
        let mut e = engine();
        constructor_doubles(&mut e, 0);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, 0x55);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_dc90, &args![this, 0u32]).u32(), this);
        assert!(calls_to(&e, CONSTRUCT_008C71B0).is_empty());
        assert_eq!(e.mem.u32(this), 0);
        constructor_doubles(&mut e, 0x7100);
        e.call(0x0065_dc90, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this), 0x7101);
    }

    #[test]
    fn destructor_0065d6e0_deallocates_one_element() {
        let mut e = engine();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0x6100);
        e.call_log = Some(vec![]);
        e.call(0x0065_d6e0, &args![this]);
        assert_eq!(
            calls_to(&e, DEALLOCATE_0064DF10),
            vec![vec![this + 4, 0x6100, 1]]
        );
        assert_eq!(calls_to(&e, EMPTY_BASE_00483710), vec![vec![this]]);
    }

    /// A vector of four elements of `element` bytes, plus iterator doubles that always pass and
    /// an unwrap (`0065fe40`) that reads the iterator's word and a `0065fd70` that returns its
    /// first argument.
    fn erase_setup(e: &mut Engine, element: u32) -> (Ptr, u32) {
        e.register(ITERATOR_HAS_CONTAINER_005CA4F0, |_, _| 1u32.into_ret());
        e.register(ITERATOR_COMPATIBLE_004B0460, |_, _| 1u32.into_ret());
        e.register(FIRST_ARGUMENT_0065FD70, |_, a| a[0].into_ret());
        e.register(UNCHECKED_0065FE40, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(0x0065_eb60, |_, a| a[0].into_ret());
        e.register(0x0065_eb90, |_, a| a[0].into_ret());
        let this = vector(e);
        let first = e.mem.alloc(element * 4);
        e.mem.set_u32(this.addr() + 0xc, first);
        e.mem.set_u32(this.addr() + 0x10, first + 4 * element);
        e.mem.set_u32(this.addr() + 0x14, first + 4 * element);
        (this, first)
    }

    fn check_erase(address: u32, element: u32, construct: u32, seen: u32, offset: u32) {
        let mut e = engine();
        let (this, first) = erase_setup(&mut e, element);
        let result = e.mem.alloc(8);
        let (from, to) = (first + element, first + 2 * element);
        e.call_log = Some(vec![]);
        let answer = e.call(address, &args![this, result, 0x11u32, from, 0x22u32, to]);
        assert_eq!(answer.u32(), result);
        assert_eq!(
            calls_to(&e, construct),
            vec![vec![from, to], vec![to, to + element]]
        );
        let destroyed: Vec<u32> = calls_to(&e, seen).iter().map(|w| w[0]).collect();
        assert_eq!(destroyed, vec![first + 3 * element + offset]);
        assert_eq!(e.mem.u32(this.addr() + 0x10), first + 3 * element);
        assert_eq!(e.mem.u32(result + 4), from);
        let adopted = calls_to(&e, ITERATOR_ADOPT_0044AC20);
        assert_eq!(adopted.len(), 2);
        assert!(adopted.iter().all(|w| w[1] == this.addr()));
        // An empty range copies and destroys nothing.
        e.call_log = Some(vec![]);
        e.call(address, &args![this, result, 0u32, from, 0u32, from]);
        assert!(calls_to(&e, construct).is_empty());
        assert_eq!(e.mem.u32(this.addr() + 0x10), first + 3 * element);
    }

    #[test]
    fn erase_0065d740() {
        check_erase(0x0065_d740, 0x1c, 0x0065_eb60, MEMBER_DTOR_0065F430, 4);
    }
    #[test]
    fn erase_0065d8e0() {
        check_erase(
            0x0065_d8e0,
            0x58,
            0x0065_eb90,
            EH_VECTOR_DESTRUCTOR_ITERATOR,
            0x10,
        );
    }

    #[test]
    fn iterator_0065da80_checks_the_range() {
        let mut e = engine();
        let this = vector(&mut e);
        e.mem.set_u32(this.addr() + 0xc, 0x1000);
        e.mem.set_u32(this.addr() + 0x10, 0x1040);
        let iterator = e.mem.alloc(8);
        for (pointer, container, invalid) in [
            (0x1000u32, this.addr(), false),
            (0x1040, this.addr(), false),
            (0x0fff, this.addr(), true),
            (0x1041, this.addr(), true),
        ] {
            e.call_log = Some(vec![]);
            let answer = e.call(0x0065_da80, &args![iterator, pointer, container]);
            assert_eq!(answer.u32(), iterator);
            assert_eq!(
                calls_to(&e, INVALID_PARAMETER_NOINFO).len(),
                invalid as usize
            );
            assert_eq!(
                calls_to(&e, ITERATOR_BASE_INIT_0064DE50),
                vec![vec![iterator]]
            );
            assert_eq!(
                calls_to(&e, ITERATOR_ADOPT_0044AC20),
                vec![vec![iterator, container]]
            );
            assert_eq!(e.mem.u32(iterator + 4), pointer);
        }
        // A null vector: the check fails before reading it.
        e.call_log = Some(vec![]);
        e.call(0x0065_da80, &args![iterator, 0x1000u32, 0u32]);
        assert_eq!(calls_to(&e, INVALID_PARAMETER_NOINFO).len(), 1);
    }

    #[test]
    fn iterator_0065dae0_steps_six_bytes_with_its_checks() {
        let mut e = engine();
        let container = vector(&mut e);
        e.mem.set_u32(container.addr() + 0x10, 0x2000);
        e.register_double(ITERATOR_CONTAINER_0060B980, move |_, _| {
            container.into_ret()
        });
        let iterator = e.mem.alloc(8);
        for (has, pointer, invalid) in [(1u32, 0x1ffau32, 0usize), (0, 0x1ffa, 1), (1, 0x2000, 1)] {
            e.register_double(ITERATOR_HAS_CONTAINER_005CA4F0, move |_, _| has.into_ret());
            e.mem.set_u32(iterator + 4, pointer);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0065_dae0, &args![iterator]).u32(), iterator);
            assert_eq!(calls_to(&e, INVALID_PARAMETER_NOINFO).len(), invalid);
            assert_eq!(e.mem.u32(iterator + 4), pointer + 6);
        }
    }

    #[test]
    fn iterator_0065db40_starts_null() {
        let mut e = engine();
        let iterator = e.mem.alloc(8);
        e.mem.set_u32(iterator + 4, 0x55);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_db40, &args![iterator]).u32(), iterator);
        assert_eq!(
            calls_to(&e, ITERATOR_BASE_INIT_0064DE50),
            vec![vec![iterator]]
        );
        assert_eq!(e.mem.u32(iterator + 4), 0);
    }

    /// `_Destroy` and `_Destroy_range`: two elements from `first`, each seen once through its
    /// leaf (`seen`, by its first word `first + offset`).
    fn check_destroy(address: u32, through_vector: bool, element: u32, seen: u32, offset: u32) {
        let mut e = engine();
        let this = vector(&mut e);
        let first = e.mem.alloc(element * 2);
        e.call_log = Some(vec![]);
        if through_vector {
            e.call(address, &args![this, first, first + 2 * element]);
        } else {
            e.call(address, &args![first, first + 2 * element, this.addr() + 8]);
        }
        let firsts: Vec<u32> = calls_to(&e, seen).iter().map(|w| w[0]).collect();
        assert_eq!(firsts, vec![first + offset, first + element + offset]);
        assert!(
            calls_to(&e, OPERATOR_DELETE).is_empty(),
            "flag 0: no delete"
        );
    }

    #[test]
    fn destroy_0065db60() {
        check_destroy(0x0065_db60, true, 0xc, DESTROY_0040FBE0, 0);
    }
    #[test]
    fn destroy_0065dbf0() {
        check_destroy(0x0065_dbf0, true, 8, DESTROY_0040FBE0, 0);
    }
    #[test]
    fn destroy_0065de00() {
        check_destroy(0x0065_de00, true, 0x34, BASE_DTOR_0044A540, 0);
    }
    #[test]
    fn destroy_0065de80() {
        check_destroy(0x0065_de80, true, 0x38, BASE_DTOR_0044A540, 0);
    }
    #[test]
    fn destroy_0065df10() {
        check_destroy(0x0065_df10, true, 0x1c, MEMBER_DTOR_0065F430, 4);
    }
    #[test]
    fn destroy_0065df80() {
        check_destroy(0x0065_df80, true, 0x58, EH_VECTOR_DESTRUCTOR_ITERATOR, 0x10);
    }
    #[test]
    fn destroy_range_0065e2d0() {
        check_destroy(0x0065_e2d0, false, 0xc, DESTROY_0040FBE0, 0);
    }
    #[test]
    fn destroy_range_0065e360() {
        check_destroy(0x0065_e360, false, 8, DESTROY_0040FBE0, 0);
    }
    #[test]
    fn destroy_range_0065e510() {
        check_destroy(0x0065_e510, false, 0x34, MEMBER_DTOR_0065C6A0, 0x1c);
    }
    #[test]
    fn destroy_range_0065e5a0() {
        check_destroy(0x0065_e5a0, false, 0x38, MEMBER_DTOR_0044A040, 0x20);
    }
    #[test]
    fn destroy_range_0065e630() {
        check_destroy(0x0065_e630, false, 0x1c, MEMBER_DTOR_0065F430, 4);
    }
    #[test]
    fn destroy_range_0065e6c0() {
        check_destroy(
            0x0065_e6c0,
            false,
            0x58,
            EH_VECTOR_DESTRUCTOR_ITERATOR,
            0x10,
        );
    }

    #[test]
    fn allocator_copy_0065de30_returns_this() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0065_de30, &args![0x1234u32, 0x5678u32]).u32(),
            0x1234
        );
        assert_eq!(e.call_log.unwrap().len(), 1);
    }

    #[test]
    fn iterator_0065ded0_and_0065e000_build_from_the_pointer() {
        for address in [0x0065_ded0u32, 0x0065_e000] {
            let mut e = engine();
            let this = vector(&mut e);
            e.mem.set_u32(this.addr() + 0xc, 0x1000);
            e.mem.set_u32(this.addr() + 0x10, 0x1040);
            let result = e.mem.alloc(8);
            e.call_log = Some(vec![]);
            let answer = if address == 0x0065_ded0 {
                e.call(address, &args![this, result, 0x77u32, 0x1010u32])
            } else {
                e.call(address, &args![result, 0x1010u32, this])
            };
            assert_eq!(answer.u32(), result);
            assert_eq!(e.mem.u32(result + 4), 0x1010);
            assert_eq!(
                calls_to(&e, ITERATOR_ADOPT_0044AC20),
                vec![vec![result, this.addr()]]
            );
            assert!(calls_to(&e, INVALID_PARAMETER_NOINFO).is_empty());
        }
    }

    #[test]
    fn iterators_compare_by_pointer_with_their_checks() {
        let mut e = engine();
        let (a, b) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(a + 4, 0x10);
        e.mem.set_u32(b + 4, 0x10);
        for (has, compatible, invalid) in [(1u32, 1u32, 0usize), (0, 1, 1), (1, 0, 1)] {
            e.register_double(ITERATOR_HAS_CONTAINER_005CA4F0, move |_, _| has.into_ret());
            e.register_double(ITERATOR_COMPATIBLE_004B0460, move |_, _| {
                compatible.into_ret()
            });
            e.call_log = Some(vec![]);
            assert!(e.call(0x0065_e1a0, &args![a, b]).bool());
            assert_eq!(calls_to(&e, INVALID_PARAMETER_NOINFO).len(), invalid);
            assert_eq!(
                calls_to(&e, ITERATOR_COMPATIBLE_004B0460).len(),
                has as usize
            );
        }
        e.mem.set_u32(b + 4, 0x20);
        assert!(!e.call(0x0065_e1a0, &args![a, b]).bool());
    }

    #[test]
    fn iterator_inequality_0065dfd0_negates_the_comparison() {
        let mut e = engine();
        e.register(ITERATOR_HAS_CONTAINER_005CA4F0, |_, _| 1u32.into_ret());
        e.register(ITERATOR_COMPATIBLE_004B0460, |_, _| 1u32.into_ret());
        let (a, b) = (e.mem.alloc(8), e.mem.alloc(8));
        e.mem.set_u32(a + 4, 0x10);
        e.mem.set_u32(b + 4, 0x10);
        assert_eq!(e.call(0x0065_dfd0, &args![a, b]).u32(), 0);
        e.mem.set_u32(b + 4, 0x11);
        assert_eq!(e.call(0x0065_dfd0, &args![a, b]).u32(), 1);
    }

    fn check_copy(address: u32, element: u32, construct: u32) {
        let mut e = engine();
        let (_, first) = erase_setup(&mut e, element);
        let destination = e.mem.alloc(element * 4);
        e.call_log = Some(vec![]);
        let end = e
            .call(address, &args![first, first + 2 * element, destination])
            .u32();
        assert_eq!(end, destination + 2 * element);
        assert_eq!(
            calls_to(&e, construct),
            vec![
                vec![destination, first],
                vec![destination + element, first + element]
            ]
        );
        let unwrap = calls_to(&e, UNCHECKED_0065FE40);
        assert_eq!(unwrap.len(), 3, "last, first, then the destination");
    }

    #[test]
    fn copy_0065e1f0() {
        check_copy(0x0065_e1f0, 0x1c, 0x0065_eb60);
    }
    #[test]
    fn copy_0065e260() {
        check_copy(0x0065_e260, 0x58, 0x0065_eb90);
    }

    fn check_inner_copy(address: u32, element: u32, construct: u32) {
        let mut e = engine();
        let (_, first) = erase_setup(&mut e, element);
        let destination = e.mem.alloc(element * 4);
        e.call_log = Some(vec![]);
        let end = e
            .call(
                address,
                &args![first, first + 3 * element, destination, 1u32, 2u32, 3u32],
            )
            .u32();
        assert_eq!(end, destination + 3 * element);
        assert_eq!(calls_to(&e, construct).len(), 3);
        assert_eq!(
            calls_to(&e, construct)[2],
            vec![destination + 2 * element, first + 2 * element]
        );
    }

    #[test]
    fn copy_0065e760() {
        check_inner_copy(0x0065_e760, 0x1c, 0x0065_eb60);
    }
    #[test]
    fn copy_0065e7c0() {
        check_inner_copy(0x0065_e7c0, 0x58, 0x0065_eb90);
    }

    fn no_copy(_: &mut Engine, _: u32, _: u32, _: u32, _: u32, _: u32, _: u32) -> u32 {
        0
    }

    #[test]
    fn copy_end_uses_the_truncating_division() {
        let mut e = engine();
        e.register(UNCHECKED_0065FE40, |e, a| e.mem.u32(a[0]).into_ret());
        // last below first by 0x1d: IDIV gives -1 element (a floor would give -2).
        let end = copy_elements(&mut e, 0x2000, 0x1fe3, 0x5000, 0x1c, no_copy);
        assert_eq!(end, 0x5000 - 0x1c);
    }

    #[test]
    fn element_copy_0065ea80_and_0065eac0_return_the_end() {
        for (address, element, construct) in [
            (0x0065_ea80u32, 0x1cu32, 0x0065_eb60u32),
            (0x0065_eac0, 0x58, 0x0065_eb90),
        ] {
            let mut e = engine();
            e.register(construct, |_, a| a[0].into_ret());
            e.call_log = Some(vec![]);
            let end = e
                .call(
                    address,
                    &args![0x1000u32, 0x1000 + 2 * element, 0x8000u32, 0u32, 0u32, 0u32],
                )
                .u32();
            assert_eq!(end, 0x8000 + 2 * element);
            assert_eq!(
                calls_to(&e, construct),
                vec![
                    vec![0x8000, 0x1000],
                    vec![0x8000 + element, 0x1000 + element]
                ]
            );
            assert_eq!(
                e.call(
                    address,
                    &args![0x1000u32, 0x1000u32, 0x8000u32, 0u32, 0u32, 0u32]
                )
                .u32(),
                0x8000
            );
        }
    }

    fn check_allocate(address: u32, element: u32) {
        let mut e = engine();
        e.register(OPERATOR_NEW, |_, a| (a[0] | 0x1_0000).into_ret());
        e.register(BAD_ALLOC_CTOR_0044D6C0, |_, a| a[0].into_ret());
        e.register(CXX_THROW_EXCEPTION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(address, &args![0u32, 0u32]).u32(), 0x1_0000);
        assert_eq!(
            e.call(address, &args![5u32, 0u32]).u32(),
            (5 * element) | 0x1_0000
        );
        assert!(calls_to(&e, CXX_THROW_EXCEPTION).is_empty());
        let limit = 0xffff_ffff / element;
        e.call(address, &args![limit, 0u32]);
        assert!(calls_to(&e, CXX_THROW_EXCEPTION).is_empty());
        e.call(address, &args![limit + 1, 0u32]);
        let thrown = calls_to(&e, CXX_THROW_EXCEPTION);
        assert_eq!(thrown.len(), 1);
        assert_eq!(thrown[0][1], BAD_ALLOC_THROW_INFO_01140EF8);
        assert_eq!(
            calls_to(&e, BAD_ALLOC_CTOR_0044D6C0),
            vec![vec![thrown[0][0], 0]]
        );
        assert_eq!(
            calls_to(&e, OPERATOR_NEW).last().unwrap(),
            &vec![(limit + 1).wrapping_mul(element)]
        );
    }

    #[test]
    fn allocate_0065e310() {
        check_allocate(0x0065_e310, 0xc);
    }
    #[test]
    fn allocate_0065e3a0() {
        check_allocate(0x0065_e3a0, 8);
    }
    #[test]
    fn allocate_0065e550() {
        check_allocate(0x0065_e550, 0x34);
    }
    #[test]
    fn allocate_0065e5e0() {
        check_allocate(0x0065_e5e0, 0x38);
    }
    #[test]
    fn allocate_0065e670() {
        check_allocate(0x0065_e670, 0x1c);
    }
    #[test]
    fn allocate_0065e700() {
        check_allocate(0x0065_e700, 0x58);
    }

    #[test]
    fn tag_0065e750_is_the_top_byte_of_ecx() {
        let mut e = engine();
        assert_eq!(
            e.call(0x0065_e750, &args![0xab12_3456u32, 1u32, 2u32])
                .u32() as u8,
            0xab
        );
        assert_eq!(fn_0065e750(&mut e, 0x0012_3456, 0, 0), 0);
    }

    fn check_destroy_each(address: u32, element: u32, seen: u32, offset: u32) {
        let mut e = engine();
        let first = e.mem.alloc(3 * element);
        e.call_log = Some(vec![]);
        e.call(address, &args![first, first + 3 * element, 0x99u32, 0u32]);
        let firsts: Vec<u32> = calls_to(&e, seen).iter().map(|w| w[0]).collect();
        assert_eq!(
            firsts,
            vec![
                first + offset,
                first + element + offset,
                first + 2 * element + offset
            ]
        );
        e.call_log = Some(vec![]);
        e.call(address, &args![first, first, 0x99u32, 0u32]);
        assert!(calls_to(&e, seen).is_empty());
    }

    #[test]
    fn destroy_each_0065e820() {
        check_destroy_each(0x0065_e820, 0xc, DESTROY_0040FBE0, 0);
    }
    #[test]
    fn destroy_each_0065e850() {
        check_destroy_each(0x0065_e850, 8, DESTROY_0040FBE0, 0);
    }
    #[test]
    fn destroy_each_0065e8e0() {
        check_destroy_each(0x0065_e8e0, 0x34, MEMBER_DTOR_0065C6A0, 0x1c);
    }
    #[test]
    fn destroy_each_0065e910() {
        check_destroy_each(0x0065_e910, 0x38, MEMBER_DTOR_0044A040, 0x20);
    }
    #[test]
    fn destroy_each_0065e940() {
        check_destroy_each(0x0065_e940, 0x1c, MEMBER_DTOR_0065F430, 4);
    }
    #[test]
    fn destroy_each_0065e970() {
        check_destroy_each(0x0065_e970, 0x58, EH_VECTOR_DESTRUCTOR_ITERATOR, 0x10);
    }

    /// One element destroyed through `address` with `words` (allocator and element, or the
    /// element alone), seen through `seen` at `element + offset`.
    fn check_destroy_one(address: u32, with_allocator: bool, seen: u32, offset: u32) {
        let mut e = engine();
        let element = e.mem.alloc(0x60);
        e.call_log = Some(vec![]);
        if with_allocator {
            e.call(address, &args![0x99u32, element]);
        } else {
            e.call(address, &args![element]);
        }
        assert_eq!(calls_to(&e, seen)[0][0], element + offset);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn allocator_destroy_0065e9a0() {
        check_destroy_one(0x0065_e9a0, true, DESTROY_0040FBE0, 0);
    }
    #[test]
    fn allocator_destroy_0065ea00() {
        check_destroy_one(0x0065_ea00, true, MEMBER_DTOR_0065C6A0, 0x1c);
    }
    #[test]
    fn allocator_destroy_0065ea20() {
        check_destroy_one(0x0065_ea20, true, MEMBER_DTOR_0044A040, 0x20);
    }
    #[test]
    fn allocator_destroy_0065ea40() {
        check_destroy_one(0x0065_ea40, true, MEMBER_DTOR_0065F430, 4);
    }
    #[test]
    fn allocator_destroy_0065ea60() {
        check_destroy_one(0x0065_ea60, true, EH_VECTOR_DESTRUCTOR_ITERATOR, 0x10);
    }
    #[test]
    fn element_destroy_0065eb20() {
        check_destroy_one(0x0065_eb20, false, MEMBER_DTOR_0065C6A0, 0x1c);
    }
    #[test]
    fn element_destroy_0065eb30() {
        check_destroy_one(0x0065_eb30, false, MEMBER_DTOR_0044A040, 0x20);
    }
    #[test]
    fn element_destroy_0065eb40() {
        check_destroy_one(0x0065_eb40, false, MEMBER_DTOR_0065F430, 4);
    }
    #[test]
    fn element_destroy_0065eb50() {
        check_destroy_one(0x0065_eb50, false, EH_VECTOR_DESTRUCTOR_ITERATOR, 0x10);
    }

    fn check_deleting(address: u32, seen: u32, offset: u32) {
        for (flags, deleted) in [(0u32, false), (1, true), (2, false), (3, true)] {
            let mut e = engine();
            let this = e.mem.alloc(0x60);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![this, flags]).u32(), this);
            assert_eq!(calls_to(&e, seen)[0][0], this + offset);
            let expected = if deleted { vec![vec![this]] } else { vec![] };
            assert_eq!(calls_to(&e, OPERATOR_DELETE), expected);
        }
    }

    #[test]
    fn deleting_0065ec70() {
        check_deleting(0x0065_ec70, MEMBER_DTOR_0065C6A0, 0x1c);
    }
    #[test]
    fn deleting_0065eca0() {
        check_deleting(0x0065_eca0, MEMBER_DTOR_0044A040, 0x20);
    }
    #[test]
    fn deleting_0065ecd0() {
        check_deleting(0x0065_ecd0, MEMBER_DTOR_0065F430, 4);
    }
    #[test]
    fn deleting_0065ed00() {
        check_deleting(0x0065_ed00, EH_VECTOR_DESTRUCTOR_ITERATOR, 0x10);
    }

    #[test]
    fn destructors_0065ed70_and_0065edd0_run_member_then_base() {
        for (address, member, offset) in [
            (0x0065_ed70u32, MEMBER_DTOR_0065C6A0, 0x1cu32),
            (0x0065_edd0, MEMBER_DTOR_0044A040, 0x20),
        ] {
            let mut e = engine();
            e.call_log = Some(vec![]);
            e.call(address, &args![0x3000u32]);
            let order: Vec<(u32, u32)> = e
                .call_log
                .take()
                .unwrap()
                .into_iter()
                .filter(|(a, _)| *a == member || *a == BASE_DTOR_0044A540)
                .map(|(a, w)| (a, w[0]))
                .collect();
            assert_eq!(
                order,
                vec![(member, 0x3000 + offset), (BASE_DTOR_0044A540, 0x3000)]
            );
        }
    }

    #[test]
    fn destructor_0065ee30_destroys_the_member_at_four() {
        let mut e = engine();
        let this = e.mem.alloc(0x1c);
        e.call_log = Some(vec![]);
        e.call(0x0065_ee30, &args![this]);
        assert_eq!(calls_to(&e, MEMBER_DTOR_0065F430), vec![vec![this + 4]]);
    }

    #[test]
    fn destructor_0065ee50_destroys_its_three_members() {
        let mut e = engine();
        e.call_log = Some(vec![]);
        e.call(0x0065_ee50, &args![0x3000u32]);
        assert_eq!(
            calls_to(&e, EH_VECTOR_DESTRUCTOR_ITERATOR),
            vec![vec![0x3010, 0x18, 3, 0x0065_f060]]
        );
    }

    #[test]
    fn destructors_0065ee70_and_0065f060_end_in_the_base() {
        for (address, member) in [
            (0x0065_ee70u32, MEMBER_DTOR_0065F430),
            (0x0065_f060, MEMBER_DTOR_00657F60),
        ] {
            let mut e = engine();
            let this = e.mem.alloc(8);
            e.mem.set_u32(this, 0x6200);
            e.call_log = Some(vec![]);
            e.call(address, &args![this]);
            assert_eq!(calls_to(&e, member), vec![vec![this]]);
            // 0065cfb0 -> 0065d6e0: the one element at +0 deallocated, then 00483710.
            assert_eq!(
                calls_to(&e, DEALLOCATE_0064DF10),
                vec![vec![this + 4, 0x6200, 1]]
            );
            assert_eq!(calls_to(&e, EMPTY_BASE_00483710), vec![vec![this]]);
        }
    }
}

#[cfg(test)]
mod n076_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    #[test]
    fn max_sizes_come_from_the_allocator_constants() {
        let mut e = Engine::new();
        let vector = e.mem.alloc(0x18);
        for (address, limit) in [
            (0x0065_db90, 0x1555_5555u32),
            (0x0065_dbd0, 0x1fff_ffff),
            (0x0065_dc40, 0x07ff_ffff),
            (0x0065_dd70, 0x05d1_745d),
            (0x0065_dde0, 0x04ec_4ec4),
            (0x0065_de60, 0x0492_4924),
            (0x0065_def0, 0x0924_9249),
            (0x0065_df60, 0x02e8_ba2e),
        ] {
            assert_eq!(e.call(address, &args![vector]).u32(), limit);
        }
        assert_eq!(allocator_max_size(0), 1);
        e.call_log = Some(vec![]);
        e.call(0x0065_e170, &args![vector + 8]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn allocators_allocate_count_times_the_element() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, a| (a[0] | 0x1000_0000).into_ret());
        for (address, element) in [
            (0x0065_dbb0, 0xcu32),
            (0x0065_dc20, 8),
            (0x0065_dd50, 0x20),
            (0x0065_ddc0, 0x2c),
            (0x0065_de40, 0x34),
            (0x0065_deb0, 0x38),
            (0x0065_df40, 0x1c),
            (0x0065_dfb0, 0x58),
        ] {
            assert_eq!(
                e.call(address, &args![0x5000u32, 3u32]).u32(),
                0x1000_0000 | (3 * element)
            );
        }
        assert_eq!(e.call(0x0065_e430, &args![0u32, 0u32]).u32(), 0x1000_0000);
        assert_eq!(e.call(0x0065_e4c0, &args![2u32, 0u32]).u32(), 0x1000_0058);
    }

    #[test]
    fn oversized_allocation_throws_bad_alloc_first() {
        let mut e = Engine::new();
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.register(BAD_ALLOC_CTOR_0044D6C0, |_, _| Ret::default());
        e.register(CXX_THROW_EXCEPTION, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0065_e430, &args![0x0800_0000u32, 0u32]);
        assert_eq!(calls_to(&e, CXX_THROW_EXCEPTION).len(), 1);
        assert_eq!(calls_to(&e, OPERATOR_NEW), vec![vec![0]]);
    }

    #[test]
    fn destroying_ranges_runs_each_element_destructor() {
        let mut e = Engine::new();
        e.register(BASE_DTOR_0044A540, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let vector = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        e.call(0x0065_dc60, &args![vector, 0x4000u32, 0x4040u32]);
        assert_eq!(
            calls_to(&e, BASE_DTOR_0044A540),
            vec![vec![0x4004], vec![0x4024]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0065_dd90, &args![vector, 0x4000u32, 0x4058u32]);
        assert_eq!(
            calls_to(&e, BASE_DTOR_0044A540),
            vec![vec![0x4010], vec![0x403c]]
        );
        // No element is deleted by the range destructors.
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
        e.call(0x0065_ec10, &args![0x4000u32, 1u32]);
        e.call(0x0065_ec40, &args![0x4000u32, 0u32]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE), vec![vec![0x4000]]);
    }

    /// Doubles for one `operator=` instance's callees on vectors in memory (element 1).
    fn assign_engine() -> Engine {
        let mut e = Engine::new();
        let f = &ASSIGN_1;
        e.register(f.size, |e, a| {
            (e.mem.u32(a[0] + VECTOR_LAST) - e.mem.u32(a[0] + VECTOR_FIRST)).into_ret()
        });
        e.register(f.capacity, |e, a| {
            (e.mem.u32(a[0] + VECTOR_END) - e.mem.u32(a[0] + VECTOR_FIRST)).into_ret()
        });
        e.register(f.clear, |_, _| Ret::default());
        e.register(f.destroy, |_, _| Ret::default());
        e.register(f.copy, |_, a| (a[2] + a[1] - a[0]).into_ret());
        e.register(f.uninitialized_copy, |_, a| (a[3] + a[2] - a[1]).into_ret());
        e.register(f.buy, |e, a| {
            e.mem.set_u32(a[0] + VECTOR_FIRST, 0x9000);
            e.mem.set_u32(a[0] + VECTOR_LAST, 0x9000);
            e.mem.set_u32(a[0] + VECTOR_END, 0x9000 + a[1]);
            1u32.into_ret()
        });
        e.register(DEALLOCATE_0064DF10, |_, _| Ret::default());
        e
    }

    fn vector(e: &mut Engine, first: u32, size: u32, capacity: u32) -> u32 {
        let v = e.mem.alloc(0x18);
        e.mem.set_u32(v + VECTOR_FIRST, first);
        e.mem.set_u32(v + VECTOR_LAST, first + size);
        e.mem.set_u32(v + VECTOR_END, first + capacity);
        v
    }

    #[test]
    fn assignment_takes_the_branch_of_the_sizes() {
        let f = &ASSIGN_1;
        let mut e = assign_engine();
        // Self-assignment does nothing; an empty source clears.
        let this = vector(&mut e, 0x1000, 4, 8);
        let empty = vector(&mut e, 0x2000, 0, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_f0c0, &args![this, this]).u32(), this);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        e.call(0x0065_f0c0, &args![this, empty]);
        assert_eq!(calls_to(&e, f.clear), vec![vec![this]]);
        // Smaller: copied over the start, the rest destroyed.
        let small = vector(&mut e, 0x2000, 2, 2);
        e.call_log = Some(vec![]);
        e.call(0x0065_f0c0, &args![this, small]);
        assert_eq!(calls_to(&e, f.copy), vec![vec![0x2000, 0x2002, 0x1000]]);
        assert_eq!(calls_to(&e, f.destroy), vec![vec![this, 0x1002, 0x1004]]);
        assert_eq!(e.mem.u32(this + VECTOR_LAST), 0x1002);
        // Within the capacity: the overlap copied, the rest copy-constructed.
        let more = vector(&mut e, 0x3000, 6, 6);
        e.call_log = Some(vec![]);
        e.call(0x0065_f0c0, &args![this, more]);
        assert_eq!(calls_to(&e, f.copy), vec![vec![0x3000, 0x3002, 0x1000]]);
        assert_eq!(
            calls_to(&e, f.uninitialized_copy),
            vec![vec![this, 0x3002, 0x3006, 0x1002]]
        );
        assert_eq!(e.mem.u32(this + VECTOR_LAST), 0x1006);
        // Beyond it: freed (the byte count) and bought anew.
        let large = vector(&mut e, 0x4000, 12, 12);
        e.call_log = Some(vec![]);
        e.call(0x0065_f0c0, &args![this, large]);
        assert_eq!(calls_to(&e, f.destroy), vec![vec![this, 0x1000, 0x1006]]);
        assert_eq!(
            calls_to(&e, DEALLOCATE_0064DF10),
            vec![vec![this + 8, 0x1000, 8]]
        );
        assert_eq!(calls_to(&e, f.buy), vec![vec![this, 12]]);
        assert_eq!(e.mem.u32(this + VECTOR_LAST), 0x900c);
    }

    #[test]
    fn six_byte_assignment_counts_elements() {
        let f = &ASSIGN_6;
        let mut e = Engine::new();
        e.register(f.size, |e, a| {
            ((e.mem.u32(a[0] + VECTOR_LAST) - e.mem.u32(a[0] + VECTOR_FIRST)) / 6).into_ret()
        });
        e.register(f.capacity, |_, _| 1u32.into_ret());
        e.register(f.destroy, |_, _| Ret::default());
        e.register(f.buy, |_, _| Ret::default());
        e.register(DEALLOCATE_0064DF10, |_, _| Ret::default());
        let this = vector(&mut e, 0x1000, 6, 12);
        let other = vector(&mut e, 0x2000, 18, 18);
        e.call_log = Some(vec![]);
        e.call(0x0065_eed0, &args![this, other]);
        assert_eq!(
            calls_to(&e, DEALLOCATE_0064DF10),
            vec![vec![this + 8, 0x1000, 2]]
        );
        // `_Buy` failed: nothing is copied.
        assert_eq!(calls_to(&e, f.buy), vec![vec![this, 3]]);
        assert!(calls_to(&e, f.uninitialized_copy).is_empty());
        // Smaller: `_Mylast` is `_Myfirst + size * 6`.
        e.register(f.copy, |_, a| a[2].into_ret());
        let this = vector(&mut e, 0x1000, 12, 12);
        let small = vector(&mut e, 0x2000, 6, 6);
        e.call(0x0065_eed0, &args![this, small]);
        assert_eq!(e.mem.u32(this + VECTOR_LAST), 0x1006);
    }

    #[test]
    fn element_copies_copy_the_float_words_and_members() {
        let mut e = Engine::new();
        // Empty member vectors: each assignment clears.
        e.register(ASSIGN_6.size, |_, _| Ret::default());
        e.register(ASSIGN_6.clear, |_, _| Ret::default());
        e.register(ASSIGN_1.size, |_, _| Ret::default());
        e.register(ASSIGN_1.clear, |_, _| Ret::default());
        let this = e.mem.alloc(0x58);
        let other = e.mem.alloc(0x58);
        e.mem.set_f32(other, 2.5);
        e.mem.set_u32(other + 0xc, 0x77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0065_eb60, &args![this, other]).u32(), this);
        assert_eq!(e.mem.f32(this), 2.5);
        assert_eq!(calls_to(&e, ASSIGN_6.clear), vec![vec![this + 4]]);
        e.call(0x0065_eb90, &args![this, other]);
        assert_eq!(e.mem.u32(this + 0xc), 0x77);
        assert_eq!(
            calls_to(&e, ASSIGN_1.clear),
            vec![vec![this + 0x10], vec![this + 0x28], vec![this + 0x40]]
        );
    }
}
