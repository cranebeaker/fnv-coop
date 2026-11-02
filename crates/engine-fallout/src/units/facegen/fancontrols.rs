//! `FaceGen/fancontrols.obj` (Xbox PDB source unit), subsystem `FaceGen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! FaceGen's control groups (`FanGenCtlsGrpC`) and the pieces of the C++ library and of
//! `FR2MatrixVTC<float>` (a `std::vector<float>` of 0x18 bytes with its row and column counts
//! at +0x18 and +0x1C) this unit instantiates. A linear control is 0x3C bytes: a matrix and
//! its label string at +0x20.

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::facegen::binaryfile::{
    string_chars, STRING_ASSIGN, STRING_LENGTH, STRING_NPOS, STRING_TIDY, VECTOR_FIRST, VECTOR_LAST,
};
use crate::units::facegen::binaryfile::{
    INVALID_PARAMETER, MEMORY_MANAGER_FREE, MEMORY_MANAGER_OBJECT,
};
use crate::units::facegen::run_00aba3c0::checked_iterator;
use crate::units::facegen::run_00aba3c0::{
    advance_by_address, erase_by_address, grown_capacity, max_or_one, resize_with_value,
    vector_capacity, vector_size, with_value_slot, ElementSize, ResizeShape, WordsArg,
    THROW_LENGTH_0065F640, VECTOR_ALLOCATOR, VECTOR_END,
};

/// The matrix's row and column counts.
const MATRIX_ROWS: u32 = 0x18;
const MATRIX_COLUMNS: u32 = 0x1c;
/// The linear control's size and its label.
const CONTROL_SIZE: u32 = 0x3c;
const CONTROL_LABEL: u32 = 0x20;

/// `fr2Assert` (`00aafb90`, `cdecl`: source file name, line) and the source file names
/// (`"...\FaceGen\FanControls.cpp"` at `010a2c5c`, `"...\facegen\matrixVT.hpp"` at
/// `01065830`).
const FR2_ASSERT: u32 = 0x00aa_fb90;
const FAN_CONTROLS_SOURCE: u32 = 0x010a_2c5c;
const MATRIX_SOURCE: u32 = 0x0106_5830;
/// The matrix iterators: begin (`0044a170`, `thiscall`: out iterator), end (`0065f550`,
/// `thiscall`: out iterator; answers its address), made from a pointer and a container
/// (`0065da80`, `thiscall` on the iterator: pointer, container), compared (`0065dfd0`,
/// `thiscall`: the other; answers a byte, not 0 when they differ), dereferenced (`0064dbd0` and
/// `0064dda0`, `thiscall`; answer the element's address), post-incremented (`006590c0`,
/// `thiscall`: out copy and 0) and pre-incremented (`00659190`, `thiscall`).
const MATRIX_BEGIN: u32 = 0x0044_a170;
const MATRIX_END: u32 = 0x0065_f550;
const MAKE_ITERATOR: u32 = 0x0065_da80;
const ITERATORS_DIFFER: u32 = 0x0065_dfd0;
const DEREFERENCE: u32 = 0x0064_dbd0;
const DEREFERENCE_CHECKED: u32 = 0x0064_dda0;
const POST_INCREMENT: u32 = 0x0065_90c0;
const PRE_INCREMENT: u32 = 0x0065_9190;
/// The matrix's vector constructor (`00449fd0`), its resize with a value (`0060b410`:
/// count, float), the matrix copy constructor (`00658f30`: source), the matrix destructor
/// (`00449680`) and the matrix times a float (`00658260`, `FR2MatrixVTC<float>::operator*`:
/// result, float), all `thiscall`.
const VECTOR_CONSTRUCT: u32 = 0x0044_9fd0;
const VECTOR_RESIZE_WITH: u32 = 0x0060_b410;
const MATRIX_COPY: u32 = 0x0065_8f30;
const MATRIX_DESTRUCT: u32 = 0x0044_9680;
const MATRIX_TIMES_FLOAT: u32 = 0x0065_8260;
/// `0044a0a0` (`thiscall`, no stack words): what the control constructor runs on its matrix
/// after the counts.
const CONTROL_MATRIX_INIT: u32 = 0x0044_a0a0;
/// The product of a group's matrix with a vector (`00658360`, `thiscall`: result, the other),
/// the element pointer of a float vector (`0060b370`, `thiscall`: out word, index) and the
/// float vector's tidy (`0065f240`, `thiscall`).
const MATRIX_PRODUCT: u32 = 0x0065_8360;
const FLOAT_VECTOR_POINTER: u32 = 0x0060_b370;
const FLOAT_VECTOR_TIDY: u32 = 0x0065_f240;
/// The string's grow (`0044a8d0`, `thiscall`: new size, 0; answers a byte), the length throw
/// (`00ec30b0`, no arguments; does not return in the game), `memset` (`00ec61c0`, `cdecl`),
/// the string copy constructor (`0044a500`, `thiscall`: source).
const STRING_GROW: u32 = 0x0044_a8d0;
const STRING_THROW_LENGTH: u32 = 0x00ec_30b0;
const MEMSET: u32 = 0x00ec_61c0;
const STRING_COPY: u32 = 0x0044_a500;
/// The linear control's destructor (`006593e0`) and scalar deleting destructor (`00659380`:
/// flags), both `thiscall` (unit `fallout shared/facegen/run_006592c0`).
const CONTROL_DESTRUCT: u32 = 0x0065_93e0;
const CONTROL_DELETING_DESTRUCT: u32 = 0x0065_9380;
/// The linear control vector's helpers outside this unit: the allocation (`00658c10`, `cdecl`:
/// count, hint), the iterator advance (`00aaecb0`), the erase (`00aae200`), the range
/// assignment (`00aaf6f0`, `cdecl`: first, last, value) and the backward move (`00aaf860`,
/// `cdecl`, six words: first, last, end and three tags).
const CONTROL_ALLOCATE: u32 = 0x0065_8c10;
const CONTROL_ADVANCE: u32 = 0x00aa_ecb0;
const CONTROL_ERASE: u32 = 0x00aa_e200;
const CONTROL_ASSIGN: u32 = 0x00aa_f6f0;
const CONTROL_MOVE_BACKWARD: u32 = 0x00aa_f860;
/// The uninitialized copy of controls (`00aafa80`, `cdecl`, six words), unit `run_00aafa80`.
const CONTROL_UNINITIALIZED_COPY: u32 = 0x00aa_fa80;

// Translated from 00658650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FR2MatrixVTC<float>::operator*=(float)` (the map's `operator__ov3`; `thiscall`, the
/// float): walks the matrix from its begin iterator (`0044a170`) to its end (`0065f550`,
/// `0065dfd0`) and multiplies each element (`0064dbd0`) by `scale` (`FLD`, `FMUL`, `FSTP`),
/// advancing with the post-increment (`006590c0`). Returns `this`.
pub fn fn_00658650(e: &mut Engine, this: Ptr, scale: f32) -> Ptr {
    e.with_stack(0x18, |e, locals| {
        let iterator = locals.addr();
        let end = locals.addr() + 8;
        let copy = locals.addr() + 0x10;
        e.call(MATRIX_BEGIN, &args![this, iterator]);
        loop {
            let end = e.call(MATRIX_END, &args![this, end]).u32();
            if e.call(ITERATORS_DIFFER, &args![iterator, end]).u8() == 0 {
                break;
            }
            let element = e.call(DEREFERENCE, &args![iterator]).u32();
            let product = x87_mul(x87_load32(e.mem.u32(element)), x87_m32(scale.to_bits()));
            e.mem.set_u32(element, x87_store32(product));
            e.call(POST_INCREMENT, &args![iterator, copy, 0u32]);
        }
    });
    this
}

// Translated from 00aabdb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aabdb0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: one control's value (`thiscall`, four words: the group (below 5),
/// two indices (below 2) and a table of 0x40-byte vectors; result in `ST0`). Each bound that
/// is not met asserts (`fr2Assert(FanControls.cpp, 0xc6 / 0xc7 / 0xc8)`). An unloaded group
/// set (byte +0 is 0) gives 0.0. Otherwise the control at `+0x324 + group * 0xA0 + i * 0x50 +
/// j * 0x28` is multiplied with `table + j * 0x40` (`00658360`), the product's first float is
/// taken and the product freed (`0065f240` and the memory manager's free of its proxy), and the
/// control's offset (+0x24) is added.
pub fn fn_00aabdb0(
    e: &mut Engine,
    this: Ptr,
    group: i32,
    first: i32,
    second: i32,
    table: u32,
) -> f32 {
    if group >= 5 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0xc6u32]);
    }
    if first >= 2 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0xc7u32]);
    }
    if second >= 2 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0xc8u32]);
    }
    if e.mem.u8(this.addr()) == 0 {
        return 0.0;
    }
    let control = this
        .addr()
        .wrapping_add((group as u32).wrapping_mul(0xa0))
        .wrapping_add(0x324)
        .wrapping_add((first as u32).wrapping_mul(0x50))
        .wrapping_add((second as u32).wrapping_mul(0x28));
    let value = e.with_stack(0x28, |e, locals| {
        let product = locals.addr();
        let out = locals.addr() + 0x20;
        let other = table.wrapping_add((second as u32).wrapping_shl(6));
        let result = e
            .call(MATRIX_PRODUCT, &args![control, product, other])
            .u32();
        let pointer = e
            .call(FLOAT_VECTOR_POINTER, &args![result, out, 0u32])
            .u32();
        let floats = e.mem.u32(pointer);
        let value = x87_quiet_bits(e.mem.u32(floats));
        e.call(FLOAT_VECTOR_TIDY, &args![product]);
        let proxy = e.mem.u32(product);
        e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, proxy]);
        value
    });
    let offset = e.mem.u32(control.wrapping_add(0x24));
    let sum = x87_add(x87_load32(value), x87_m32(offset));
    f32::from_bits(x87_store32(sum))
}

// Translated from 00aadd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the result and a 2x2 float matrix): the inverse. The
/// determinant `m0 * m3 - m1 * m2` and its reciprocal are stored as floats; the result is
/// `(m3, -m1, -m2, m0)` times the reciprocal (`FCHS` before the multiply). Returns `result`.
pub fn fn_00aadd20(e: &mut Engine, result: u32, matrix: u32) -> u32 {
    let m: Vec<u32> = (0..4).map(|i| e.mem.u32(matrix + 4 * i)).collect();
    let a = x87_mul(x87_load32(m[0]), x87_m32(m[3]));
    let b = x87_mul(x87_load32(m[1]), x87_m32(m[2]));
    let determinant = x87_store32(x87_sub(a, b));
    let reciprocal = x87_store32(x87_div(1.0, x87_load32(determinant)));
    let scale = |value: f64| x87_store32(x87_mul(value, x87_m32(reciprocal)));
    let r0 = scale(x87_load32(m[3]));
    let r1 = scale(-x87_load32(m[1]));
    let r2 = scale(-x87_load32(m[2]));
    let r3 = scale(x87_load32(m[0]));
    for (i, word) in [r0, r1, r2, r3].iter().enumerate() {
        e.mem.set_u32(result + 4 * i as u32, *word);
    }
    result
}

// Translated from 00aadda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FR2MatrixVTC<float>::operator/(float)` (the map's `operator__ov4`; `thiscall`: the result
/// and the divisor): the matrix times `1 / divisor` (stored as a float) through `00658260`.
/// Returns `result`.
pub fn fn_00aadda0(e: &mut Engine, this: Ptr, result: u32, divisor: f32) -> u32 {
    let reciprocal = x87_store32(x87_div(1.0, x87_load32(divisor.to_bits())));
    let reciprocal = x87_store32(x87_load32(reciprocal));
    e.call(MATRIX_TIMES_FLOAT, &args![this, result, reciprocal]);
    result
}

// Translated from 00aadde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the linear control vector's `resize(count)` (`thiscall`): builds a
/// default control (`00aade20`) in the argument area and passes it by value to the resize with
/// a value (`00aadf50`), which works on it and destroys it there: the matrix's proxy names
/// that area.
pub fn fn_00aadde0(e: &mut Engine, this: Ptr, count: u32) {
    // 00aaddef: `SUB ESP,0x3C; MOV ECX,ESP`: the control is built in the outgoing argument
    // area, which stays in place as `00aadf50`'s own `[EBP+0xC]` (its `RET 0x40` frees it).
    e.with_stack(CONTROL_SIZE, |e, outgoing| {
        fn_00aade20(e, outgoing); // 00aaddf7
        resize_from_area(e, this, count, outgoing.addr()); // 00aade0c: the area itself
    });
}

// Translated from 00aade20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aade20 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: the linear control's default constructor: the matrix's vector
/// (`00449fd0`), zero rows and columns, `0044a0a0` on the matrix, and an empty label. Returns
/// `this`.
pub fn fn_00aade20(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(VECTOR_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr() + MATRIX_ROWS, 0);
    e.mem.set_u32(this.addr() + MATRIX_COLUMNS, 0);
    e.call(CONTROL_MATRIX_INIT, &args![this]);
    e.call(STRING_TIDY, &args![this.addr() + CONTROL_LABEL, 0u32, 0u32]);
    this
}

// Translated from 00aadeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::append(count, character)` (unnamed in the engine map; `thiscall`, two words):
/// a count that reaches `npos - length` calls the length throw (`00ec30b0`, which does not
/// return in the game). For a count above 0 the string grows (`0044a8d0(length + count, 0)`);
/// when it can, the characters are set (`00aae180`), the length updated and a terminating 0
/// written. Returns `this`.
pub fn fn_00aadeb0(e: &mut Engine, this: Ptr, count: u32, character: u8) -> Ptr {
    let string = this.addr();
    let npos: u32 = e.global(STRING_NPOS);
    if npos.wrapping_sub(e.mem.u32(string + STRING_LENGTH)) <= count {
        e.call(STRING_THROW_LENGTH, &[]);
    }
    if count == 0 {
        return this;
    }
    let new_length = e.mem.u32(string + STRING_LENGTH).wrapping_add(count);
    if e.call(STRING_GROW, &args![this, new_length, 0u32]).u8() == 0 {
        return this;
    }
    let length = e.mem.u32(string + STRING_LENGTH);
    fn_00aae180(e, this, length, count, character);
    e.mem.set_u32(string + STRING_LENGTH, new_length);
    let chars = string_chars(e, string);
    e.mem.set_u8(chars.wrapping_add(new_length), 0);
    this
}

// Translated from 00aadf50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aadf50 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: the linear control vector's `resize(count, value)` (`thiscall`,
/// sixteen words: the count and the control by value): grows with `00aae3a0`, or shrinks with
/// the advance `00aaecb0` and the erase `00aae200`; then the value is destroyed (`006593e0`).
pub fn fn_00aadf50(e: &mut Engine, this: Ptr, count: u32, value: [u32; 15]) {
    with_value_slot(e, &value, |e, slot| resize_from_area(e, this, count, slot));
}

/// `00aadf50`'s body over the control in its argument area at `value` (the exe's `[EBP+0xC]`,
/// where `00aadde0` builds it): the resize copies from the control there and destroys it in
/// place.
fn resize_from_area(e: &mut Engine, this: Ptr, count: u32, value: u32) {
    let shape = ResizeShape {
        size: ElementSize::Divide(CONTROL_SIZE),
        insert: |e, this, proxy, at, count, value| fn_00aae3a0(e, this, proxy, at, count, value),
        advance: |e, iterator, count| advance_by_address(CONTROL_ADVANCE, e, iterator, count),
        erase: |e, this, result, a, b, c, d| {
            erase_by_address(CONTROL_ERASE, e, this, result, a, b, c, d)
        },
    };
    resize_with_value(e, this, count, value, &shape); // 00aae002: `LEA ECX,[EBP+0xC]`
    e.call(CONTROL_DESTRUCT, &args![value]); // 00aae163: the control destroyed where it lies
}

fn fn_00aadf50_entry(e: &mut Engine, this: Ptr, count: u32, value: WordsArg<15>) {
    fn_00aadf50(e, this, count, value.0)
}

// Translated from 00aae180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `std::string::_Chassign(offset, count, character)` (unnamed in the engine map; `thiscall`,
/// three words): one character is stored directly, more through `memset` (the character
/// sign-extended).
pub fn fn_00aae180(e: &mut Engine, this: Ptr, offset: u32, count: u32, character: u8) {
    let chars = string_chars(e, this.addr());
    if count == 1 {
        e.mem.set_u8(chars.wrapping_add(offset), character);
        return;
    }
    let fill = character as i8 as i32 as u32;
    e.call(MEMSET, &args![chars.wrapping_add(offset), fill, count]);
}

// Translated from 00aae3a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aae3a0 - the compiler's exception frame (FS:[0] chain) and the /GS cookie check (00ec408c) are not translated
/// Unnamed in the engine map: the linear control vector's `_Insert_n(where, count, value)`
/// (`thiscall`, four words). Nothing for 0; past the maximum (0x4444444) `0065f640` throws.
/// Past the capacity, a block of half as much again (at least enough; `00658c10`) gets the
/// controls before `where` (`00aafa30`), `count` copies (`00aaf8a0`) and the rest (`00aaf7e0`);
/// the old controls are destroyed (`00659380` with flag 0) and their block freed. Otherwise
/// the value is copied first: when fewer than `count` controls follow `where`, they move up
/// (`00aafa30`), the gap past the old end gets copies (`00aaf8a0`) and the rest is assigned
/// (`00aaf6f0`); else the last `count` controls are copied past the end (`00aaf7e0`), the others
/// move up (`00aaf860`) and `count` controls from `where` are assigned. The copy is destroyed
/// after.
pub fn fn_00aae3a0(e: &mut Engine, this: Ptr, _unused_1: u32, at: u32, count: u32, value: u32) {
    let vector = this.addr();
    let size = ElementSize::Divide(CONTROL_SIZE);
    let allocator = vector + VECTOR_ALLOCATOR;
    let capacity = vector_capacity(e, vector, size);
    if count == 0 {
        return;
    }
    let maximum = max_or_one(0x0444_4444);
    if maximum.wrapping_sub(vector_size(e, vector, size)) < count {
        e.call(THROW_LENGTH_0065F640, &[]);
        return;
    }
    if capacity < vector_size(e, vector, size).wrapping_add(count) {
        let needed = vector_size(e, vector, size).wrapping_add(count);
        let new_capacity = grown_capacity(capacity, 0x0444_4444, needed);
        let block = e.call(CONTROL_ALLOCATE, &args![new_capacity, 0u32]).u32();
        let first = e.mem.u32(vector + VECTOR_FIRST);
        let mut cursor = fn_00aafa30(e, first, at, block, allocator);
        fn_00aaf8a0(e, cursor, count, value, allocator, 0, 0);
        cursor = count.wrapping_mul(CONTROL_SIZE).wrapping_add(cursor);
        let last = e.mem.u32(vector + VECTOR_LAST);
        fn_00aaf7e0(e, at, last, cursor, allocator);
        let new_size = vector_size(e, vector, size).wrapping_add(count);
        let first = e.mem.u32(vector + VECTOR_FIRST);
        if first != 0 {
            let last = e.mem.u32(vector + VECTOR_LAST);
            let mut element = first;
            while element != last {
                e.call(CONTROL_DELETING_DESTRUCT, &args![element, 0u32]);
                element = element.wrapping_add(CONTROL_SIZE);
            }
            let first = e.mem.u32(vector + VECTOR_FIRST);
            e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, first]);
        }
        e.mem.set_u32(
            vector + VECTOR_END,
            new_capacity.wrapping_mul(CONTROL_SIZE).wrapping_add(block),
        );
        e.mem.set_u32(
            vector + VECTOR_LAST,
            new_size.wrapping_mul(CONTROL_SIZE).wrapping_add(block),
        );
        e.mem.set_u32(vector + VECTOR_FIRST, block);
        return;
    }
    let after = size.count(e.mem.u32(vector + VECTOR_LAST).wrapping_sub(at));
    e.with_stack(CONTROL_SIZE, |e, temporary| {
        let copy = temporary.addr();
        if after < count {
            fn_00aaec10(e, temporary, Ptr::new(value));
            let last = e.mem.u32(vector + VECTOR_LAST);
            fn_00aafa30(
                e,
                at,
                last,
                count.wrapping_mul(CONTROL_SIZE).wrapping_add(at),
                allocator,
            );
            let last = e.mem.u32(vector + VECTOR_LAST);
            let missing = count.wrapping_sub(size.count(last.wrapping_sub(at)));
            fn_00aaf8a0(e, last, missing, copy, allocator, 0, 0);
            let last = count
                .wrapping_mul(CONTROL_SIZE)
                .wrapping_add(e.mem.u32(vector + VECTOR_LAST));
            e.mem.set_u32(vector + VECTOR_LAST, last);
            let end = last.wrapping_sub(count.wrapping_mul(CONTROL_SIZE));
            e.call(CONTROL_ASSIGN, &args![at, end, copy]);
            e.call(CONTROL_DESTRUCT, &args![copy]);
        } else {
            e.call(MATRIX_COPY, &args![copy, value]);
            e.call(
                STRING_COPY,
                &args![copy + CONTROL_LABEL, value + CONTROL_LABEL],
            );
            let old_last = e.mem.u32(vector + VECTOR_LAST);
            let last = e.mem.u32(vector + VECTOR_LAST);
            let tail = old_last.wrapping_sub(count.wrapping_mul(CONTROL_SIZE));
            let new_last = fn_00aaf7e0(e, tail, old_last, last, allocator);
            e.mem.set_u32(vector + VECTOR_LAST, new_last);
            let tail = old_last.wrapping_sub(count.wrapping_mul(CONTROL_SIZE));
            e.call(
                CONTROL_MOVE_BACKWARD,
                &args![at, tail, old_last, 0u32, 0u32, 0u32],
            );
            let end = count.wrapping_mul(CONTROL_SIZE).wrapping_add(at);
            e.call(CONTROL_ASSIGN, &args![at, end, copy]);
            e.call(STRING_TIDY, &args![copy + CONTROL_LABEL, 1u32, 0u32]);
            e.call(MATRIX_DESTRUCT, &args![copy]);
        }
    });
}

// Translated from 00aaec10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aaec10 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: the linear control's copy constructor (`thiscall`, the source):
/// the matrix's copy constructor (`00658f30`), then the label made empty and assigned the
/// source's. Returns `this`.
pub fn fn_00aaec10(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    e.call(MATRIX_COPY, &args![this, source]);
    let label = this.addr() + CONTROL_LABEL;
    e.call(STRING_TIDY, &args![label, 0u32, 0u32]);
    let npos: u32 = e.global(STRING_NPOS);
    e.call(
        STRING_ASSIGN,
        &args![label, source.addr() + CONTROL_LABEL, 0u32, npos],
    );
    this
}

// Translated from 00aaed70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aaed70 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `FR2MatrixVTC<float>::operator-` (the map's `operator__ov5`; `thiscall`: the result and the
/// other matrix). Matrices of different sizes assert (`fr2Assert(matrixVT.hpp, 0x1ca)`). A
/// temporary matrix of this one's size is filled with zeros (`00449fd0`, `0060b410`); iterators
/// over it, this and the other (`0065da80`) walk to this one's end (`0065dfd0`), storing
/// `this - other` per element (`0064dda0`; `FLD`, `FSUB`, `FSTP`) and advancing this one's, the
/// other's and the temporary's (`00659190`). The result is copy-constructed from the temporary
/// (`00658f30`), which is then destroyed (`00449680`). Returns `result`.
pub fn fn_00aaed70(e: &mut Engine, this: Ptr, result: u32, other: Ptr) -> u32 {
    let a = this.addr();
    let b = other.addr();
    if e.mem.u32(a + MATRIX_ROWS) != e.mem.u32(b + MATRIX_ROWS)
        || e.mem.u32(a + MATRIX_COLUMNS) != e.mem.u32(b + MATRIX_COLUMNS)
    {
        e.call(FR2_ASSERT, &args![MATRIX_SOURCE, 0x1cau32]);
    }
    let columns = e.mem.u32(a + MATRIX_COLUMNS);
    let rows = e.mem.u32(a + MATRIX_ROWS);
    e.with_stack(0x40, |e, locals| {
        let temporary = locals.addr();
        let out = locals.addr() + 0x20;
        let at_out = out;
        let at_a = out + 8;
        let at_b = out + 0x10;
        let at_end = out + 0x18;
        e.call(VECTOR_CONSTRUCT, &args![temporary]);
        e.mem.set_u32(temporary + MATRIX_ROWS, rows);
        e.mem.set_u32(temporary + MATRIX_COLUMNS, columns);
        let count = e
            .mem
            .u32(temporary + MATRIX_COLUMNS)
            .wrapping_mul(e.mem.u32(temporary + MATRIX_ROWS));
        e.call(
            VECTOR_RESIZE_WITH,
            &args![temporary, count, x87_store32(0.0)],
        );
        let first = e.mem.u32(temporary + VECTOR_FIRST);
        e.call(MAKE_ITERATOR, &args![at_out, first, temporary]);
        let first = e.mem.u32(a + VECTOR_FIRST);
        e.call(MAKE_ITERATOR, &args![at_a, first, a]);
        let first = e.mem.u32(b + VECTOR_FIRST);
        e.call(MAKE_ITERATOR, &args![at_b, first, b]);
        loop {
            let last = e.mem.u32(a + VECTOR_LAST);
            e.call(MAKE_ITERATOR, &args![at_end, last, a]);
            if e.call(ITERATORS_DIFFER, &args![at_a, at_end]).u8() == 0 {
                break;
            }
            let destination = e.call(DEREFERENCE_CHECKED, &args![at_out]).u32();
            let left = e.call(DEREFERENCE_CHECKED, &args![at_a]).u32();
            let right = e.call(DEREFERENCE_CHECKED, &args![at_b]).u32();
            let difference = x87_sub(x87_load32(e.mem.u32(left)), x87_m32(e.mem.u32(right)));
            e.mem.set_u32(destination, x87_store32(difference));
            e.call(PRE_INCREMENT, &args![at_a]);
            e.call(PRE_INCREMENT, &args![at_b]);
            e.call(PRE_INCREMENT, &args![at_out]);
        }
        e.call(MATRIX_COPY, &args![result, temporary]);
        e.call(MATRIX_DESTRUCT, &args![temporary]);
    });
    result
}

// Translated from 00aaf7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words: first, last, destination, allocator): the
/// controls' uninitialized copy through `00aafa80` (with two tag words). Returns the
/// destination's end.
pub fn fn_00aaf7e0(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    e.call(
        CONTROL_UNINITIALIZED_COPY,
        &args![first, last, destination, allocator, 0u32, 0u32],
    )
    .u32()
}

// Translated from 00aaf8a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aaf8a0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`, six words: destination, count, the value's address, the
/// allocator and two tags): copy-constructs (`00aaec10`) `count` controls from the value,
/// skipping null slots.
#[allow(clippy::too_many_arguments)]
pub fn fn_00aaf8a0(
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
            fn_00aaec10(e, Ptr::new(destination), Ptr::new(value));
        }
        count -= 1;
        destination = destination.wrapping_add(CONTROL_SIZE);
    }
}

// Translated from 00aafa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, four words): the same uninitialized copy of controls as
/// `00aaf7e0` (`00aafa80`). Returns the destination's end.
pub fn fn_00aafa30(e: &mut Engine, first: u32, last: u32, destination: u32, allocator: u32) -> u32 {
    e.call(
        CONTROL_UNINITIALIZED_COPY,
        &args![first, last, destination, allocator, 0u32, 0u32],
    )
    .u32()
}

/// The matrix helpers `readFromFile` calls: a matrix's norm (`006588a0`, `thiscall`; result in
/// `ST0`), its transpose (`006587a0`, `thiscall`: the result; answers it), the assignment
/// (`006581e0`, `FR2MatrixVTC<float>::operator=`, `thiscall`: the source), the CRT's
/// `sqrt(double)` (`00ec6054`, `cdecl`, result in `ST0`), and the free of a temporary's proxy
/// (`0065d6e0`, `thiscall`).
const MATRIX_NORM: u32 = 0x0065_88a0;
const MATRIX_TRANSPOSE: u32 = 0x0065_87a0;
const MATRIX_ASSIGN: u32 = 0x0065_81e0;
const SQRT: u32 = 0x00ec_6054;
const VECTOR_FREE_PROXY: u32 = 0x0065_d6e0;
/// The smallest norm `readFromFile` accepts without asserting (`double` 0.001 at `0101e788`).
const NORM_THRESHOLD: u32 = 0x0101_e788;
/// The binary file's `read` (vtable slot +0x10: buffer, element size, count).
const FILE_READ: u32 = 0x10;

/// `FanGenCtlsGrpC` (Xbox PDB) on PC, 0xFEC bytes: `initialized` (+0), `distrib` (+0x04, five
/// race distributions of 0xA0 bytes), `attr` (+0x324, five groups of 2x2 linear controls of
/// 0x28 bytes: a matrix, its inverse norm +0x20 and its offset +0x24), `race` (+0x644, 5x5 race
/// offset controls of 0x48 bytes: two matrices, the inverse norm sum +0x40 and an offset +0x44),
/// `basis` (+0xD4C, five groups of two pairs of matrices) and `invAttCov` (+0xFCC, five groups
/// of two 2x2 float matrices).
const ATTRIBUTES: u32 = 0x324;
const RACE_OFFSETS: u32 = 0x644;
const BASIS: u32 = 0xd4c;
const INVERSE_COVARIANCE: u32 = 0xfcc;

/// `file.read(buffer, size, count)`.
fn file_read(e: &mut Engine, file: Ptr, buffer: u32, size: u32, count: u32) -> bool {
    e.vcall(file.addr(), FILE_READ, &args![buffer, size, count])
        .u8()
        != 0
}

/// Sets a matrix's counts and resizes its floats to `count` zeros (`0060b410`).
fn shape_matrix(e: &mut Engine, matrix: u32, rows: u32, columns: u32, count: u32) {
    e.mem.set_u32(matrix + MATRIX_ROWS, rows);
    e.mem.set_u32(matrix + MATRIX_COLUMNS, columns);
    e.call(VECTOR_RESIZE_WITH, &args![matrix, count, x87_store32(0.0)]);
}

/// The address of a matrix's floats (`0060b370` with index 0, its out word at `out`).
fn matrix_floats(e: &mut Engine, matrix: u32, out: u32) -> u32 {
    let pointer = e
        .call(FLOAT_VECTOR_POINTER, &args![matrix, out, 0u32])
        .u32();
    e.mem.u32(pointer)
}

/// Frees a temporary matrix: its floats (`0065f240`) and its proxy (the memory manager).
fn free_temporary(e: &mut Engine, matrix: u32) {
    e.call(FLOAT_VECTOR_TIDY, &args![matrix]);
    let proxy = e.mem.u32(matrix);
    e.call(MEMORY_MANAGER_FREE, &args![MEMORY_MANAGER_OBJECT, proxy]);
}

/// A matrix's norm through `sqrt`: the norm (`006588a0`) stored as a float, passed as a
/// `double` to `sqrt`, and the result stored as a float; it asserts (`fr2Assert(FanControls.cpp,
/// line)`) unless the result is above 0.001.
fn checked_norm(e: &mut Engine, matrix: u32, line: u32) -> u32 {
    let squared = x87_store32(e.call(MATRIX_NORM, &args![matrix]).f64());
    let root = e.call(SQRT, &args![x87_load32(squared)]).f64();
    let norm = x87_quiet_bits(x87_store32(root));
    let threshold: f64 = e.global(NORM_THRESHOLD);
    if !(x87_load32(norm) > threshold) {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, line]);
    }
    norm
}

// Translated from 00aaaf10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aaaf10 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `FanGenCtlsGrpC::readFromFile` (Xbox PDB; `thiscall`: the binary file and the 2x2 count
/// table `n`, whose first column `n[k][0]` gives the sizes). Any failed read answers false.
/// 1. For each group and each 2x2 linear control: a 1 x `n[j][0]` matrix of zeros, its floats
///    and its offset (+0x24) read, and the inverse of its norm stored (+0x20).
/// 2. For each pair of different groups: two 1 x `n[k][0]` matrices read, their norms summed,
///    the offset (+0x44) read and the inverse of the sum stored (+0x40).
/// 3. For each group's distribution: two `n[k][0]` x 1 matrices, a square matrix of the total
///    size (`n[0][0] + n[1][0]`) and two `n[k][0]` square matrices, all read.
/// 4. For each group and control `i`: the basis' first matrix becomes control `[0][i]` over its
///    norm (`sqrt`, asserting above 0.001; `00aadda0`, `006581e0`); the second becomes control
///    `[1][i]` minus the first times the first float of `[1][i] * transpose(first)`
///    (`006587a0`, `00658360`, `00658260`, `00aaed70`), then is divided by its own norm (line
///    0x98; `00658650` with the inverse). Every temporary is freed.
/// 5. A constant-true check (`fr2Assert` at line 0xa1 never runs). For each group and control
///    `i`, the 2x2 matrix of first floats of `[a][i] * transpose([b][i])` is summed into
///    `invAttCov` and replaced by its inverse (`00aadd20`).
///
/// Then `initialized` is set and the answer is true.
pub fn fan_gen_ctls_grp_c_read_from_file(e: &mut Engine, this: Ptr, file: Ptr, table: u32) -> bool {
    let base = this.addr();
    let size_at = |e: &Engine, k: u32| e.mem.u32(table + k * 8);
    e.with_stack(0x100, |e, locals| {
        let out = locals.addr();
        let first_temporary = locals.addr() + 0x20;
        let second_temporary = locals.addr() + 0x40;
        let third_temporary = locals.addr() + 0x60;
        let fourth_temporary = locals.addr() + 0x80;
        let fifth_temporary = locals.addr() + 0xa0;
        let inverse = locals.addr() + 0xc0;
        let attribute = |group: u32, row: u32, column: u32| {
            base + ATTRIBUTES + group * 0xa0 + row * 0x50 + column * 0x28
        };
        for group in 0..5u32 {
            for row in 0..2u32 {
                for column in 0..2u32 {
                    let size = size_at(e, column);
                    let control = attribute(group, row, column);
                    shape_matrix(e, control, 1, size, size);
                    let floats = matrix_floats(e, control, out);
                    let count = size_at(e, column);
                    if !file_read(e, file, floats, 4, count) {
                        return false;
                    }
                    if !file_read(e, file, control + 0x24, 4, 1) {
                        return false;
                    }
                    let norm = e.call(MATRIX_NORM, &args![control]).f64();
                    e.mem
                        .set_u32(control + 0x20, x87_store32(x87_div(1.0, norm)));
                }
            }
        }
        for group in 0..5u32 {
            for other in 0..5u32 {
                if other == group {
                    continue;
                }
                let offsets = base + RACE_OFFSETS + group * 0x168 + other * 0x48;
                let mut sum = x87_store32(0.0);
                for k in 0..2u32 {
                    let size = size_at(e, k);
                    let control = offsets + k * 0x20;
                    shape_matrix(e, control, 1, size, size);
                    let floats = matrix_floats(e, control, out);
                    let count = size_at(e, k);
                    if !file_read(e, file, floats, 4, count) {
                        return false;
                    }
                    let norm = e.call(MATRIX_NORM, &args![control]).f64();
                    sum = x87_store32(x87_add(norm, x87_m32(sum)));
                }
                if !file_read(e, file, offsets + 0x44, 4, 1) {
                    return false;
                }
                let reciprocal = x87_div(1.0, x87_load32(sum));
                e.mem.set_u32(offsets + 0x40, x87_store32(reciprocal));
            }
        }
        let total = size_at(e, 0).wrapping_add(size_at(e, 1));
        for group in 0..5u32 {
            let distribution = base + 4 + group * 0xa0;
            for k in 0..2u32 {
                let size = size_at(e, k);
                let mean = distribution + k * 0x20;
                shape_matrix(e, mean, size, 1, size);
                let floats = matrix_floats(e, mean, out);
                if !file_read(e, file, floats, 4, size) {
                    return false;
                }
            }
            let covariance = base + 0x44 + group * 0xa0;
            shape_matrix(e, covariance, total, total, total.wrapping_mul(total));
            let floats = matrix_floats(e, covariance, out);
            if !file_read(e, file, floats, 4, total.wrapping_mul(total)) {
                return false;
            }
            for k in 0..2u32 {
                let size = size_at(e, k);
                let block = distribution + 0x60 + k * 0x20;
                shape_matrix(e, block, size, size, size.wrapping_mul(size));
                let floats = matrix_floats(e, block, out);
                if !file_read(e, file, floats, 4, size.wrapping_mul(size)) {
                    return false;
                }
            }
        }
        for group in 0..5u32 {
            for control_index in 0..2u32 {
                let control = base + ATTRIBUTES + group * 0xa0 + control_index * 0x28;
                let basis = base + BASIS + group * 0x80 + control_index * 0x40;
                let norm = checked_norm(e, control, 0x90);
                let divided =
                    fn_00aadda0(e, Ptr::new(control), first_temporary, f32::from_bits(norm));
                e.call(MATRIX_ASSIGN, &args![basis, divided]);
                e.call(FLOAT_VECTOR_TIDY, &args![first_temporary]);
                e.call(VECTOR_FREE_PROXY, &args![first_temporary]);
                let transposed = e
                    .call(MATRIX_TRANSPOSE, &args![basis, second_temporary])
                    .u32();
                let product = e
                    .call(
                        MATRIX_PRODUCT,
                        &args![control + 0x50, third_temporary, transposed],
                    )
                    .u32();
                let floats = matrix_floats(e, product, out);
                let projection = x87_quiet_bits(e.mem.u32(floats));
                free_temporary(e, third_temporary);
                free_temporary(e, second_temporary);
                let scaled = e
                    .call(
                        MATRIX_TIMES_FLOAT,
                        &args![basis, fourth_temporary, x87_quiet_bits(projection)],
                    )
                    .u32();
                let difference = fn_00aaed70(
                    e,
                    Ptr::new(control + 0x50),
                    fifth_temporary,
                    Ptr::new(scaled),
                );
                e.call(MATRIX_ASSIGN, &args![basis + 0x20, difference]);
                free_temporary(e, fifth_temporary);
                free_temporary(e, fourth_temporary);
                let norm = checked_norm(e, basis + 0x20, 0x98);
                let reciprocal = x87_store32(x87_div(1.0, x87_load32(norm)));
                fn_00658650(
                    e,
                    Ptr::new(basis + 0x20),
                    f32::from_bits(x87_quiet_bits(reciprocal)),
                );
            }
        }
        // The code tests a constant 1 here; its assertion (line 0xa1) never runs.
        if std::hint::black_box(1u32) == 0 {
            e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0xa1u32]);
        }
        for group in 0..5u32 {
            for control_index in 0..2u32 {
                let covariance = base + INVERSE_COVARIANCE + group * 0x20 + control_index * 0x10;
                e.call(MEMSET, &args![covariance, 0u32, 0x10u32]);
                for a in 0..2u32 {
                    for b in 0..2u32 {
                        let cell = covariance + a * 8 + b * 4;
                        let transposed = e
                            .call(
                                MATRIX_TRANSPOSE,
                                &args![attribute(group, b, control_index), first_temporary],
                            )
                            .u32();
                        let product = e
                            .call(
                                MATRIX_PRODUCT,
                                &args![
                                    attribute(group, a, control_index),
                                    second_temporary,
                                    transposed
                                ],
                            )
                            .u32();
                        let floats = matrix_floats(e, product, out);
                        let sum = x87_add(x87_load32(e.mem.u32(floats)), x87_m32(e.mem.u32(cell)));
                        e.mem.set_u32(cell, x87_store32(sum));
                        free_temporary(e, second_temporary);
                        free_temporary(e, first_temporary);
                    }
                }
                let result = fn_00aadd20(e, inverse, covariance);
                for i in 0..4u32 {
                    let word = e.mem.u32(result + 4 * i);
                    e.mem.set_u32(covariance + 4 * i, word);
                }
            }
        }
        e.mem.set_u8(base, 1);
        true
    })
}

/// More matrix helpers of the control functions below: the element-wise `+=` (`006586c0`,
/// `thiscall`: the other matrix; asserts `matrixVT.hpp` line 0x24f on a size mismatch), the
/// element-wise sum (`00658500`, the map's `FR2MatrixVTC<float>::operator__ov2`; `thiscall`:
/// result, other), the zeroing of a matrix's floats (`0060b3b0`, `thiscall`; a `memset` of
/// rows * columns floats), the address of a float by index (`006578b0`, `thiscall`: the index,
/// bounds-checked), and the other matrix's begin iterator (`0065fd10`, `thiscall`: out iterator).
const MATRIX_ADD_ASSIGN: u32 = 0x0065_86c0;
const MATRIX_SUM: u32 = 0x0065_8500;
const MATRIX_CLEAR: u32 = 0x0060_b3b0;
const MATRIX_FLOAT_AT: u32 = 0x0065_78b0;
const MATRIX_CONST_BEGIN: u32 = 0x0065_fd10;
/// A normally distributed random number (`00ac6360`, no arguments, result in `ST0`: the polar
/// method over the uniform generator `00ac6220`).
const RANDOM_NORMAL: u32 = 0x00ac_6360;
/// `memcpy` (`00ec44d0`, `cdecl`).
const MEMCPY: u32 = 0x00ec_44d0;
/// The control iterators' compare (`00ac0460`, `thiscall` on an iterator: the other; a failed
/// check unless both have the same non-null proxy; answers a byte, not 0 when the pointers
/// differ).
const CONTROL_ITERATORS_DIFFER: u32 = 0x00ac_0460;
/// The controls' backward move (`00aaf9a0`, `cdecl`, six words: first, last, end and three tags),
/// unit `run_00aaf9a0`.
const CONTROL_COPY_BACKWARD: u32 = 0x00aa_f9a0;
/// The clamp bounds of `00aabf30` and `00aac170`: lower `15.0` and `-4.0` (`0101e580`,
/// `01077848`), upper `65.0` and `4.0` (`0102243c`, `0101e570`), floats.
const LOWER_BOUND_0: u32 = 0x0101_e580;
const LOWER_BOUND_1: u32 = 0x0107_7848;
const UPPER_BOUND_0: u32 = 0x0102_243c;
const UPPER_BOUND_1: u32 = 0x0101_e570;
/// `double` constants: 0.01 (`01016408`), 0.0 (`01012060`), 1.0 (`01012070`), 2.0 (`01011590`)
/// and 1.5 (`01016ff0`); `float` constants: 0.01 (`01013ea4`), -1.0 (`01012054`) and 2.0
/// (`010162c0`).
const SMALLEST_VALUE: u32 = 0x0101_6408;
const SMALLEST_VALUE_FLOAT: u32 = 0x0101_3ea4;
const DOUBLE_ZERO: u32 = 0x0101_2060;
const DOUBLE_ONE: u32 = 0x0101_2070;
const DOUBLE_TWO: u32 = 0x0101_1590;
const DOUBLE_ONE_AND_A_HALF: u32 = 0x0101_6ff0;
const FLOAT_MINUS_ONE: u32 = 0x0101_2054;
const FLOAT_TWO: u32 = 0x0101_62c0;

/// The linear control `[first][second]` of `group` (`attr`, +0x324).
fn attribute_control(this: u32, group: i32, first: i32, second: i32) -> u32 {
    this.wrapping_add(ATTRIBUTES)
        .wrapping_add((group as u32).wrapping_mul(0xa0))
        .wrapping_add((first as u32).wrapping_mul(0x50))
        .wrapping_add((second as u32).wrapping_mul(0x28))
}

/// The 2x2 float matrix `[second]` of `group`'s `invAttCov` (+0xFCC).
fn inverse_covariance(this: u32, group: i32, second: i32) -> u32 {
    this.wrapping_add(INVERSE_COVARIANCE)
        .wrapping_add((group as u32).wrapping_shl(5))
        .wrapping_add((second as u32).wrapping_shl(4))
}

/// The race offset control `[group][other]` (+0x644).
fn race_offset(this: u32, group: i32, other: i32) -> u32 {
    this.wrapping_add(RACE_OFFSETS)
        .wrapping_add((group as u32).wrapping_mul(0x168))
        .wrapping_add((other as u32).wrapping_mul(0x48))
}

/// The distribution matrix `[index]` (`0x20` bytes each) of `group` (+0x04).
fn distribution(this: u32, group: i32, index: u32) -> u32 {
    this.wrapping_add(4)
        .wrapping_add((group as u32).wrapping_mul(0xa0))
        .wrapping_add(index.wrapping_shl(5))
}

/// The basis matrix `[first][index]` of `group` (+0xD4C).
fn basis(this: u32, group: i32, first: i32, index: u32) -> u32 {
    this.wrapping_add(BASIS)
        .wrapping_add((group as u32).wrapping_shl(7))
        .wrapping_add((first as u32).wrapping_shl(6))
        .wrapping_add(index.wrapping_shl(5))
}

/// The matrix `[first][second]` of a 2x2 table of matrices (`0x40` bytes a row).
fn table_matrix(table: u32, first: u32, second: u32) -> u32 {
    table
        .wrapping_add(first.wrapping_shl(6))
        .wrapping_add(second.wrapping_shl(5))
}

/// Frees a temporary matrix through its floats (`0065f240`) and its proxy (`0065d6e0`).
fn release_temporary(e: &mut Engine, matrix: u32) {
    e.call(FLOAT_VECTOR_TIDY, &args![matrix]);
    e.call(VECTOR_FREE_PROXY, &args![matrix]);
}

/// `target += scale * transpose(source)`: the transpose (`006587a0`) into `transposed`, times
/// the float (`00658260`) into `scaled`, `006586c0` on the target, then the scaled and the
/// transposed temporary freed (`0065f240` and the memory manager).
fn add_scaled_transpose(
    e: &mut Engine,
    source: u32,
    scale: u32,
    target: u32,
    transposed: u32,
    scaled: u32,
) {
    let transpose = e.call(MATRIX_TRANSPOSE, &args![source, transposed]).u32();
    let product = e
        .call(MATRIX_TIMES_FLOAT, &args![transpose, scaled, scale])
        .u32();
    e.call(MATRIX_ADD_ASSIGN, &args![target, product]);
    free_temporary(e, scaled);
    free_temporary(e, transposed);
}

/// `sqrt(norm(matrix))` as the code computes it: the norm (`006588a0`) stored as a float,
/// passed as a `double` to `sqrt`, the root stored as a float.
fn root_of_norm(e: &mut Engine, matrix: u32) -> u32 {
    let squared = x87_store32(e.call(MATRIX_NORM, &args![matrix]).f64());
    x87_store32(e.call(SQRT, &args![x87_load32(squared)]).f64())
}

/// `numerator / sqrt(count)`: `sqrt` of the `double`, stored as a float, divides the numerator
/// (kept as a `double`: `FDIVR`), and the quotient is stored as a float.
fn divided_by_root(e: &mut Engine, numerator: u32, count: f64) -> u32 {
    let root = x87_store32(e.call(SQRT, &args![count]).f64());
    x87_store32(x87_div(x87_load32(numerator), x87_load32(root)))
}

// Translated from 00aabf30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aabf30 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: sets one control (`thiscall`, five words: the group, two indices,
/// the wanted value and a table of 0x40-byte matrices). A group of 5 or more asserts
/// (`fr2Assert(FanControls.cpp, 0xde)`); nothing more happens while the set is not loaded (byte
/// +0 is 0). The value is clamped to `[lower[first], upper[first]]` (lower 15.0 and -4.0, upper
/// 65.0 and 4.0; a lower bound above it, then an upper bound below it, replaces it) and the
/// control's current value (`00aabdb0`) is subtracted. A 2-vector holding that difference at
/// `first` (zero elsewhere) is multiplied with the group's `invAttCov[second]` (`00aaf2b0`), and
/// `table[second]` gets the transpose of the control `[first][second]` times the product's
/// element `first` added (`006587a0`, `00658260`, `006586c0`). The locals live in the code's
/// frame: `first` indexes the bounds, the vector and the product unchecked, so an index past 1
/// reads and writes the frame's other slots, as here (the frame is laid out as the exe's).
#[allow(clippy::too_many_arguments)]
pub fn fn_00aabf30(
    e: &mut Engine,
    this: Ptr,
    group: i32,
    first: i32,
    second: i32,
    value: f32,
    table: u32,
) {
    if group >= 5 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0xdeu32]);
    }
    if e.mem.u8(this.addr()) == 0 {
        return;
    }
    e.with_stack(0x78, |e, frame| {
        // `at(-0x18)` is `[EBP - 0x18]` of the exe's frame.
        let ebp = frame.addr() + 0x78;
        let at = |offset: u32| ebp.wrapping_sub(offset);
        let slot = |offset: u32| at(offset).wrapping_add((first as u32).wrapping_mul(4));
        for (offset, constant) in [
            (0x18, LOWER_BOUND_0),
            (0x14, LOWER_BOUND_1),
            (0x34, UPPER_BOUND_0),
            (0x30, UPPER_BOUND_1),
        ] {
            let bits: u32 = e.global(constant);
            e.mem.set_u32(at(offset), x87_quiet_bits(bits));
        }
        e.mem.set_u32(at(0x1c), x87_quiet_bits(value.to_bits()));
        let clamped = x87_load32(e.mem.u32(at(0x1c)));
        let lower = e.mem.u32(slot(0x18));
        if x87_load32(lower) > clamped {
            e.mem.set_u32(at(0x1c), x87_quiet_bits(lower));
        }
        let clamped = x87_load32(e.mem.u32(at(0x1c)));
        let upper = e.mem.u32(slot(0x34));
        if x87_load32(upper) < clamped {
            e.mem.set_u32(at(0x1c), x87_quiet_bits(upper));
        }
        let current = fn_00aabdb0(e, this, group, first, second, table);
        let difference = x87_sub(x87_m32(e.mem.u32(at(0x1c))), f64::from(current));
        e.mem.set_u32(at(0x10), x87_store32(difference));
        e.mem.set_u32(at(0x24), x87_store32(0.0));
        e.mem.set_u32(at(0x20), x87_store32(0.0));
        let difference = e.mem.u32(at(0x10));
        e.mem.set_u32(slot(0x24), x87_quiet_bits(difference));
        let covariance = inverse_covariance(this.addr(), group, second);
        fn_00aaf2b0(e, at(0x2c), covariance, at(0x24));
        let control = attribute_control(this.addr(), group, first, second);
        let scale = x87_quiet_bits(e.mem.u32(slot(0x2c)));
        let target = table.wrapping_add((second as u32).wrapping_shl(6));
        add_scaled_transpose(e, control, scale, target, at(0x54), at(0x74));
    });
}

// Translated from 00aac170 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aac170 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: sets both controls of a column (`thiscall`, four words: the
/// group, the column `second`, two wanted values and a table of 0x40-byte matrices). A group of
/// 5 or more asserts (`fr2Assert(FanControls.cpp, 0x10a)`); nothing more happens while the set
/// is not loaded. Each value `k` is clamped to `[lower[k], upper[k]]` (as in `00aabf30`) and
/// the control `[k][second]`'s current value (`00aabdb0`) subtracted; the 2-vector of
/// differences is multiplied with `invAttCov[second]` (`00aaf2b0`), and for each `k`
/// `table[second]` gets the transpose of control `[k][second]` times the product's element `k`
/// added.
pub fn fn_00aac170(e: &mut Engine, this: Ptr, group: i32, second: i32, values: u32, table: u32) {
    if group >= 5 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x10au32]);
    }
    if e.mem.u8(this.addr()) == 0 {
        return;
    }
    let lower: [u32; 2] = [
        x87_quiet_bits(e.global(LOWER_BOUND_0)),
        x87_quiet_bits(e.global(LOWER_BOUND_1)),
    ];
    let upper: [u32; 2] = [
        x87_quiet_bits(e.global(UPPER_BOUND_0)),
        x87_quiet_bits(e.global(UPPER_BOUND_1)),
    ];
    e.with_stack(0x50, |e, locals| {
        let differences = locals.addr();
        let product = locals.addr() + 8;
        let transposed = locals.addr() + 0x10;
        let scaled = locals.addr() + 0x30;
        e.mem.set_u32(differences, x87_store32(0.0));
        e.mem.set_u32(differences + 4, x87_store32(0.0));
        for k in 0..2u32 {
            let mut clamped = x87_quiet_bits(e.mem.u32(values.wrapping_add(4 * k)));
            if x87_load32(lower[k as usize]) > x87_load32(clamped) {
                clamped = lower[k as usize];
            }
            if x87_load32(upper[k as usize]) < x87_load32(clamped) {
                clamped = upper[k as usize];
            }
            let current = fn_00aabdb0(e, this, group, k as i32, second, table);
            let difference = x87_sub(x87_m32(clamped), f64::from(current));
            e.mem.set_u32(differences + 4 * k, x87_store32(difference));
        }
        let covariance = inverse_covariance(this.addr(), group, second);
        fn_00aaf2b0(e, product, covariance, differences);
        let target = table.wrapping_add((second as u32).wrapping_shl(6));
        for k in 0..2u32 {
            let control = attribute_control(this.addr(), group, k as i32, second);
            let scale = x87_quiet_bits(e.mem.u32(product + 4 * k));
            add_scaled_transpose(e, control, scale, target, transposed, scaled);
        }
    });
}

// Translated from 00aac3f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aac3f0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a control's value read back from a table of matrices (`thiscall`,
/// four words: the group, `first`, `second` and the table; result in `ST0`). Bounds that are
/// not met assert (`fr2Assert(FanControls.cpp, 0x134 / 0x135 / 0x136)`); an unloaded set gives
/// 0.0.
/// - `second` 1: `sqrt(norm(table[first][1]))` (`006588a0`, `sqrt`) over the square root of
///   that matrix's row count.
/// - `second` 0: `table[first][0]` minus the distribution `[first]` (`00aaed70`), then for both
///   basis matrices `b` of `[first]`: minus `b`'s transpose times the first float of `b * it`
///   (`00658360`, `006587a0`, `00658260`, `00aaef90`). The answer is `sqrt(norm(d))` of
///   `d = covariance block [first] * it` (`00658360` with the matrix at +0x60 of the
///   distribution) over `sqrt(rows(d) - 2.0)`.
/// - Any other `second` asserts (line 0x158) and gives 0.0.
pub fn fn_00aac3f0(
    e: &mut Engine,
    this: Ptr,
    group: i32,
    first: i32,
    second: i32,
    table: u32,
) -> f32 {
    if group >= 5 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x134u32]);
    }
    if first >= 2 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x135u32]);
    }
    if second >= 2 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x136u32]);
    }
    if e.mem.u8(this.addr()) == 0 {
        return 0.0;
    }
    let matrix = table_matrix(table, first as u32, second as u32);
    if second == 1 {
        let root = root_of_norm(e, matrix);
        let rows = e.mem.u32(matrix.wrapping_add(MATRIX_ROWS));
        return f32::from_bits(divided_by_root(e, root, f64::from(rows)));
    }
    if second != 0 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x158u32]);
        return 0.0;
    }
    e.with_stack(0xa8, |e, locals| {
        let difference = locals.addr();
        let deviation = locals.addr() + 0x20;
        let product = locals.addr() + 0x40;
        let out = locals.addr() + 0x60;
        let transposed = locals.addr() + 0x68;
        let scaled = locals.addr() + 0x88;
        let mean = distribution(this.addr(), group, first as u32);
        fn_00aaed70(e, Ptr::new(matrix), difference, Ptr::new(mean));
        for index in 0..2u32 {
            let direction = basis(this.addr(), group, first, index);
            let result = e
                .call(MATRIX_PRODUCT, &args![direction, product, difference])
                .u32();
            let floats = matrix_floats(e, result, out);
            let projection = x87_quiet_bits(e.mem.u32(floats));
            release_temporary(e, product);
            let transpose = e
                .call(MATRIX_TRANSPOSE, &args![direction, transposed])
                .u32();
            let times = e
                .call(MATRIX_TIMES_FLOAT, &args![transpose, scaled, projection])
                .u32();
            fn_00aaef90(e, Ptr::new(difference), Ptr::new(times));
            release_temporary(e, scaled);
            release_temporary(e, transposed);
        }
        let covariance = distribution(this.addr(), group, first as u32).wrapping_add(0x60);
        e.call(MATRIX_PRODUCT, &args![covariance, deviation, difference]);
        let root = root_of_norm(e, deviation);
        let rows = e.mem.u32(deviation + MATRIX_ROWS);
        let two: f64 = e.global(DOUBLE_TWO);
        let count = x87_store32(x87_sub(f64::from(rows), two));
        let value = divided_by_root(e, root, x87_load32(count));
        e.call(MATRIX_DESTRUCT, &args![deviation]);
        e.call(MATRIX_DESTRUCT, &args![difference]);
        f32::from_bits(value)
    })
}

// Translated from 00aac820 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aac820 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: the inverse of `00aac3f0`, setting the value read back from a
/// table of matrices (`thiscall`, five words: the group, `first`, `second`, the wanted value
/// and the table). Bounds that are not met assert (`fr2Assert(FanControls.cpp, 0x172 / 0x173 /
/// 0x174)`); nothing more happens while the set is not loaded. A value below 0.01 (`double`)
/// becomes 0.01 (`float`).
/// - `second` 1: with `s` the current value as `00aac3f0` computes it, a positive `s` scales
///   `table[first][1]` by `value / s` (`00658650`).
/// - `second` 0: `d = table[first][0] - distribution[first]` (`00aaed70`); its projection
///   `p` on the two basis matrices (`p += transpose(b) * first float of (b * d)`, `0060b3b0`,
///   `00658360`, `006587a0`, `00658260`, `006586c0`) is subtracted from `d` (`00aaef90`). With
///   `s = 00aac3f0(group, first, 0, table)`, a positive `s` scales `d` by `value / s`
///   (`00658650`) and `table[first][0]` becomes `d + p + distribution[first]` (`00658500`,
///   `006581e0`). Every temporary is freed.
/// - Any other `second`: nothing.
pub fn fn_00aac820(
    e: &mut Engine,
    this: Ptr,
    group: i32,
    first: i32,
    second: i32,
    value: f32,
    table: u32,
) {
    if group >= 5 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x172u32]);
    }
    if first >= 2 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x173u32]);
    }
    if second >= 2 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x174u32]);
    }
    if e.mem.u8(this.addr()) == 0 {
        return;
    }
    let mut value = value.to_bits();
    let smallest: f64 = e.global(SMALLEST_VALUE);
    if x87_load32(value) < smallest {
        value = x87_quiet_bits(e.global(SMALLEST_VALUE_FLOAT));
    }
    let zero: f64 = e.global(DOUBLE_ZERO);
    let matrix = table_matrix(table, first as u32, second as u32);
    if second == 1 {
        let root = root_of_norm(e, matrix);
        let rows = e.mem.u32(matrix.wrapping_add(MATRIX_ROWS));
        let current = divided_by_root(e, root, f64::from(rows));
        if x87_load32(current) > zero {
            let scale = x87_store32(x87_div(x87_load32(value), x87_m32(current)));
            fn_00658650(e, Ptr::new(matrix), f32::from_bits(x87_quiet_bits(scale)));
        }
        return;
    }
    if second != 0 {
        return;
    }
    e.with_stack(0x100, |e, locals| {
        let difference = locals.addr();
        let projection = locals.addr() + 0x20;
        let product = locals.addr() + 0x40;
        let out = locals.addr() + 0x60;
        let transposed = locals.addr() + 0x68;
        let scaled = locals.addr() + 0x88;
        let sum = locals.addr() + 0xa8;
        let total = locals.addr() + 0xc8;
        let mean = distribution(this.addr(), group, first as u32);
        fn_00aaed70(e, Ptr::new(matrix), difference, Ptr::new(mean));
        let columns = e.mem.u32(matrix.wrapping_add(MATRIX_COLUMNS));
        let rows = e.mem.u32(matrix.wrapping_add(MATRIX_ROWS));
        e.call(VECTOR_CONSTRUCT, &args![projection]);
        e.mem.set_u32(projection + MATRIX_ROWS, rows);
        e.mem.set_u32(projection + MATRIX_COLUMNS, columns);
        let count = e
            .mem
            .u32(projection + MATRIX_COLUMNS)
            .wrapping_mul(e.mem.u32(projection + MATRIX_ROWS));
        e.call(
            VECTOR_RESIZE_WITH,
            &args![projection, count, x87_store32(0.0)],
        );
        e.call(MATRIX_CLEAR, &args![projection]);
        for index in 0..2u32 {
            let direction = basis(this.addr(), group, first, index);
            let result = e
                .call(MATRIX_PRODUCT, &args![direction, product, difference])
                .u32();
            let floats = matrix_floats(e, result, out);
            let along = x87_quiet_bits(e.mem.u32(floats));
            release_temporary(e, product);
            let transpose = e
                .call(MATRIX_TRANSPOSE, &args![direction, transposed])
                .u32();
            let times = e
                .call(MATRIX_TIMES_FLOAT, &args![transpose, scaled, along])
                .u32();
            e.call(MATRIX_ADD_ASSIGN, &args![projection, times]);
            release_temporary(e, scaled);
            release_temporary(e, transposed);
        }
        fn_00aaef90(e, Ptr::new(difference), Ptr::new(projection));
        let current = fn_00aac3f0(e, this, group, first, second, table).to_bits();
        if !(x87_load32(current) > zero) {
            release_temporary(e, projection);
            release_temporary(e, difference);
            return;
        }
        let scale = x87_store32(x87_div(x87_load32(value), x87_m32(current)));
        fn_00658650(
            e,
            Ptr::new(difference),
            f32::from_bits(x87_quiet_bits(scale)),
        );
        let partial = e
            .call(MATRIX_SUM, &args![difference, sum, projection])
            .u32();
        let whole = e.call(MATRIX_SUM, &args![partial, total, mean]).u32();
        e.call(MATRIX_ASSIGN, &args![matrix, whole]);
        release_temporary(e, total);
        release_temporary(e, sum);
        release_temporary(e, projection);
        e.call(MATRIX_DESTRUCT, &args![difference]);
    });
}

// Translated from 00aacdb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aacdb0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a race offset control's value (`thiscall`, three words: the
/// group, the other group and a table of 0x40-byte matrices; result in `ST0`). Groups of 5 or
/// more and equal groups assert (`fr2Assert(FanControls.cpp, 0x1b4 / 0x1b5 / 0x1b6)`); an
/// unloaded set gives 0.0. Otherwise the first floats of `race[group][other].m[k] *
/// table[k]` (`00658360`) for `k` 0 and 1 are summed (as floats) and the control's offset
/// (+0x44) added.
pub fn fn_00aacdb0(e: &mut Engine, this: Ptr, group: i32, other: i32, table: u32) -> f32 {
    if group >= 5 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x1b4u32]);
    }
    if other >= 5 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x1b5u32]);
    }
    if group == other {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x1b6u32]);
    }
    if e.mem.u8(this.addr()) == 0 {
        return 0.0;
    }
    let control = race_offset(this.addr(), group, other);
    let mut sum = x87_store32(0.0);
    e.with_stack(0x28, |e, locals| {
        let product = locals.addr();
        let out = locals.addr() + 0x20;
        for k in 0..2u32 {
            let matrix = control.wrapping_add(k << 5);
            let other_matrix = table.wrapping_add(k << 6);
            let result = e
                .call(MATRIX_PRODUCT, &args![matrix, product, other_matrix])
                .u32();
            let floats = matrix_floats(e, result, out);
            sum = x87_store32(x87_add(x87_load32(e.mem.u32(floats)), x87_m32(sum)));
            free_temporary(e, product);
        }
    });
    let offset = e.mem.u32(control.wrapping_add(0x44));
    f32::from_bits(x87_store32(x87_add(x87_load32(sum), x87_m32(offset))))
}

// Translated from 00aacf40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aacf40 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: sets a race offset control (`thiscall`, four words: the group, the
/// other group, the wanted value and a table of 0x40-byte matrices). The bounds assert as in
/// `00aacdb0` (lines 0x1d4, 0x1d5, 0x1d6); nothing more happens while the set is not loaded.
/// The step `(value - current) * inverse norm sum` (`00aacdb0`, the control's +0x40) times the
/// transpose of `race[group][other].m[k]` is added to `table[k]` for `k` 0 and 1.
pub fn fn_00aacf40(e: &mut Engine, this: Ptr, group: i32, other: i32, value: f32, table: u32) {
    if group >= 5 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x1d4u32]);
    }
    if other >= 5 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x1d5u32]);
    }
    if group == other {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x1d6u32]);
    }
    if e.mem.u8(this.addr()) == 0 {
        return;
    }
    let current = fn_00aacdb0(e, this, group, other, table);
    let control = race_offset(this.addr(), group, other);
    let difference = x87_sub(x87_m32(value.to_bits()), f64::from(current));
    let weight = e.mem.u32(control.wrapping_add(0x40));
    let step = x87_quiet_bits(x87_store32(x87_mul(difference, x87_m32(weight))));
    e.with_stack(0x40, |e, locals| {
        let transposed = locals.addr();
        let scaled = locals.addr() + 0x20;
        for k in 0..2u32 {
            let matrix = control.wrapping_add(k << 5);
            let target = table.wrapping_add(k << 6);
            add_scaled_transpose(e, matrix, step, target, transposed, scaled);
        }
    });
}

// Translated from 00aad140 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aad140 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: random values for a group (`thiscall`, two words: the group and a
/// table of 2x2 matrices). A group of 5 or more asserts (`fr2Assert(FanControls.cpp, 0x1f2)`);
/// nothing more happens while the set is not loaded. Each matrix's floats (as many as its row
/// count) get normally distributed values (`00ac6360`). The two column matrices `[0][0]` and
/// `[1][0]` are stacked (`00aaf330`) and multiplied by the group's covariance block (+0x44 of
/// the distribution, `00658360`, `006581e0`); then `table[0][0]` becomes its first `n0` rows
/// plus the distribution's first mean and `table[1][0]` its next `n1` rows plus the second
/// mean (`00aaf110`, `00658500`, `006581e0`; `n0`, `n1` the row counts of `table[0][0]` and
/// `table[1][0]`).
pub fn fn_00aad140(e: &mut Engine, this: Ptr, group: i32, table: u32) {
    if group >= 5 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x1f2u32]);
    }
    if e.mem.u8(this.addr()) == 0 {
        return;
    }
    e.with_stack(0xe0, |e, locals| {
        let out = locals.addr();
        let stacked = locals.addr() + 8;
        let product = locals.addr() + 0x28;
        let upper = locals.addr() + 0x48;
        let upper_sum = locals.addr() + 0x68;
        let lower = locals.addr() + 0x88;
        let lower_sum = locals.addr() + 0xa8;
        for first in 0..2u32 {
            for second in 0..2u32 {
                let matrix = table_matrix(table, first, second);
                let mut index = 0u32;
                while index < e.mem.u32(matrix.wrapping_add(MATRIX_ROWS)) {
                    let pointer = e
                        .call(FLOAT_VECTOR_POINTER, &args![matrix, out, index])
                        .u32();
                    let element = e.mem.u32(pointer);
                    let random = e.call(RANDOM_NORMAL, &[]).f64();
                    e.mem.set_u32(element, x87_store32(random));
                    index = index.wrapping_add(1);
                }
            }
        }
        fn_00aaf330(e, stacked, table, table.wrapping_add(0x40));
        let covariance = this
            .addr()
            .wrapping_add((group as u32).wrapping_mul(0xa0))
            .wrapping_add(0x44);
        let result = e
            .call(MATRIX_PRODUCT, &args![covariance, product, stacked])
            .u32();
        e.call(MATRIX_ASSIGN, &args![stacked, result]);
        release_temporary(e, product);
        let first_rows = e.mem.u32(table.wrapping_add(MATRIX_ROWS));
        let second_rows = e.mem.u32(table.wrapping_add(0x40 + MATRIX_ROWS));
        let part = fn_00aaf110(e, Ptr::new(stacked), upper, 0, 0, first_rows, 1);
        let mean = distribution(this.addr(), group, 0);
        let sum = e.call(MATRIX_SUM, &args![part, upper_sum, mean]).u32();
        e.call(MATRIX_ASSIGN, &args![table, sum]);
        free_temporary(e, upper_sum);
        free_temporary(e, upper);
        let part = fn_00aaf110(e, Ptr::new(stacked), lower, first_rows, 0, second_rows, 1);
        let mean = distribution(this.addr(), group, 1);
        let sum = e.call(MATRIX_SUM, &args![part, lower_sum, mean]).u32();
        e.call(MATRIX_ASSIGN, &args![table.wrapping_add(0x40), sum]);
        free_temporary(e, lower_sum);
        free_temporary(e, lower);
        e.call(MATRIX_DESTRUCT, &args![stacked]);
    });
}

/// The random fill of `00aad500` and `00aad6b0` for one element: `source[row][column]` plus
/// `random * scale * 1.5` (kept as a `double`) stored as a float in `destination[row][column]`
/// (`0060b370` for both rows).
fn add_random(
    e: &mut Engine,
    scale: f32,
    source: u32,
    destination: u32,
    row: u32,
    column: u32,
    outs: u32,
) {
    let random = e.call(RANDOM_NORMAL, &[]).f64();
    let factor: f64 = e.global(DOUBLE_ONE_AND_A_HALF);
    let step = x87_mul(x87_mul(random, x87_m32(scale.to_bits())), factor);
    let pointer = e
        .call(FLOAT_VECTOR_POINTER, &args![source, outs, row])
        .u32();
    let element = e.mem.u32(pointer).wrapping_add(column.wrapping_mul(4));
    let pointer = e
        .call(FLOAT_VECTOR_POINTER, &args![destination, outs + 8, row])
        .u32();
    let sum = x87_store32(x87_add(x87_load32(e.mem.u32(element)), step));
    let target = e.mem.u32(pointer).wrapping_add(column.wrapping_mul(4));
    e.mem.set_u32(target, x87_quiet_bits(sum));
}

/// Shapes `destination[first][second]` like `source[first][second]` (rows and columns, the
/// floats resized to zeros) and calls `element(second, from, to, row, column)` for each
/// element, the counts read again at each step.
fn for_each_shaped(
    e: &mut Engine,
    source: u32,
    destination: u32,
    mut element: impl FnMut(&mut Engine, u32, u32, u32, u32, u32),
) {
    for first in 0..2u32 {
        for second in 0..2u32 {
            let from = table_matrix(source, first, second);
            let to = table_matrix(destination, first, second);
            let columns = e.mem.u32(from.wrapping_add(MATRIX_COLUMNS));
            let rows = e.mem.u32(from.wrapping_add(MATRIX_ROWS));
            e.mem.set_u32(to.wrapping_add(MATRIX_ROWS), rows);
            e.mem.set_u32(to.wrapping_add(MATRIX_COLUMNS), columns);
            e.call(
                VECTOR_RESIZE_WITH,
                &args![to, rows.wrapping_mul(columns), x87_store32(0.0)],
            );
            let mut row = 0u32;
            while row < e.mem.u32(from.wrapping_add(MATRIX_ROWS)) {
                let mut column = 0u32;
                while column < e.mem.u32(from.wrapping_add(MATRIX_COLUMNS)) {
                    element(e, second, from, to, row, column);
                    column = column.wrapping_add(1);
                }
                row = row.wrapping_add(1);
            }
        }
    }
}

// Translated from 00aad500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words: a scale and two tables of 2x2 matrices;
/// `this` is not used): every matrix of `destination` takes the shape of its `source` matrix
/// (zeros, `0060b410`), and each element becomes the source's plus a normally distributed
/// random number (`00ac6360`) times `scale * 1.5`.
pub fn fn_00aad500(e: &mut Engine, _this: Ptr, scale: f32, source: u32, destination: u32) {
    e.with_stack(0x10, |e, outs| {
        let outs = outs.addr();
        for_each_shaped(e, source, destination, |e, _, from, to, row, column| {
            add_random(e, scale, from, to, row, column, outs);
        });
    });
}

// Translated from 00aad6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words, `this` not used): as `00aad500` for the
/// first column of matrices (`[k][0]`); the elements of the second column (`[k][1]`) are set to
/// 0.0.
pub fn fn_00aad6b0(e: &mut Engine, _this: Ptr, scale: f32, source: u32, destination: u32) {
    e.with_stack(0x18, |e, outs| {
        let outs = outs.addr();
        for_each_shaped(
            e,
            source,
            destination,
            |e, second, from, to, row, column| {
                if second == 0 {
                    add_random(e, scale, from, to, row, column, outs);
                } else {
                    let pointer = e
                        .call(FLOAT_VECTOR_POINTER, &args![to, outs + 0x10, row])
                        .u32();
                    let target = e.mem.u32(pointer).wrapping_add(column.wrapping_mul(4));
                    e.mem.set_u32(target, x87_store32(0.0));
                }
            },
        );
    });
}

// Translated from 00aad890 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aad890 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `FanGenCtlsGrpC::reflect` (engine map; `thiscall`, two words: the group and a table of 2x2
/// matrices). A group of 5 or more asserts (`fr2Assert(FanControls.cpp, 0x275)`); nothing more
/// happens while the set is not loaded. For `k` 0 and 1: `table[k][1]` is negated (times the
/// float -1.0, `00658260`, `006581e0`) and `table[k][0]` becomes `2.0 * distribution[k] -
/// table[k][0]` (`00658260`, `00aaed70`, `006581e0`).
pub fn fan_gen_ctls_grp_c_reflect(e: &mut Engine, this: Ptr, group: i32, table: u32) {
    if group >= 5 {
        e.call(FR2_ASSERT, &args![FAN_CONTROLS_SOURCE, 0x275u32]);
    }
    if e.mem.u8(this.addr()) == 0 {
        return;
    }
    e.with_stack(0x60, |e, locals| {
        let negated = locals.addr();
        let doubled = locals.addr() + 0x20;
        let reflected = locals.addr() + 0x40;
        for k in 0..2u32 {
            let second = table_matrix(table, k, 1);
            let minus_one = x87_quiet_bits(e.global(FLOAT_MINUS_ONE));
            let result = e
                .call(MATRIX_TIMES_FLOAT, &args![second, negated, minus_one])
                .u32();
            e.call(MATRIX_ASSIGN, &args![second, result]);
            free_temporary(e, negated);
            let mean = distribution(this.addr(), group, k);
            let two = x87_quiet_bits(e.global(FLOAT_TWO));
            let twice = e.call(MATRIX_TIMES_FLOAT, &args![mean, doubled, two]).u32();
            let first = table_matrix(table, k, 0);
            let result = fn_00aaed70(e, Ptr::new(twice), reflected, Ptr::new(first));
            e.call(MATRIX_ASSIGN, &args![first, result]);
            free_temporary(e, reflected);
            free_temporary(e, doubled);
        }
    });
}

// Translated from 00aadac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aadac0 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// `FanGenCtlsGrpC::tween` (engine map; `thiscall`, six words: `first`, `second`, the weight
/// `t` and three tables of 2x2 matrices `a`, `b` and `out`). With `va` and `vb` the values of
/// `a` and `b` read back by `00aac3f0` (group 0), the wanted value is `vb` for `t < 0`, `va` for
/// `t > 1` and `va * t + (1 - t) * vb` otherwise (and for NaN). `out[first][second]` becomes
/// `a[first][second] * t + b[first][second] * (1 - t)` (`00658260`, `00658500`, `006581e0`),
/// then the wanted value is set on `out` (`00aac820`, group 0).
#[allow(clippy::too_many_arguments)]
pub fn fan_gen_ctls_grp_c_tween(
    e: &mut Engine,
    this: Ptr,
    first: i32,
    second: i32,
    weight: f32,
    a: u32,
    b: u32,
    out: u32,
) {
    let value_a = x87_store32(f64::from(fn_00aac3f0(e, this, 0, first, second, a)));
    let value_b = x87_store32(f64::from(fn_00aac3f0(e, this, 0, first, second, b)));
    let t = weight.to_bits();
    let zero: f64 = e.global(DOUBLE_ZERO);
    let one: f64 = e.global(DOUBLE_ONE);
    let wanted = if x87_load32(t) < zero {
        x87_quiet_bits(value_b)
    } else if x87_load32(t) > one {
        x87_quiet_bits(value_a)
    } else {
        let left = x87_mul(x87_load32(value_a), x87_m32(t));
        let right = x87_mul(x87_sub(1.0, x87_load32(t)), x87_m32(value_b));
        x87_store32(x87_add(left, right))
    };
    let complement = x87_quiet_bits(x87_store32(x87_sub(1.0, x87_load32(t))));
    e.with_stack(0x60, |e, locals| {
        let scaled_a = locals.addr();
        let scaled_b = locals.addr() + 0x20;
        let sum = locals.addr() + 0x40;
        let from_b = table_matrix(b, first as u32, second as u32);
        let part_b = e
            .call(MATRIX_TIMES_FLOAT, &args![from_b, scaled_b, complement])
            .u32();
        let from_a = table_matrix(a, first as u32, second as u32);
        let part_a = e
            .call(
                MATRIX_TIMES_FLOAT,
                &args![from_a, scaled_a, x87_quiet_bits(t)],
            )
            .u32();
        let total = e.call(MATRIX_SUM, &args![part_a, sum, part_b]).u32();
        let target = table_matrix(out, first as u32, second as u32);
        e.call(MATRIX_ASSIGN, &args![target, total]);
        free_temporary(e, sum);
        free_temporary(e, scaled_a);
        free_temporary(e, scaled_b);
    });
    fn_00aac820(
        e,
        this,
        0,
        first,
        second,
        f32::from_bits(x87_quiet_bits(wanted)),
        out,
    );
}

// Translated from 00aae200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the linear control vector's `erase(first, last)` (`thiscall`, five
/// words: the result iterator and the two iterators; their proxy words are not read). Both
/// pointers are checked to lie in the vector. When they differ (`00ac0460`), the controls from
/// `last` on are assigned down to `first` (`00aaf750`), the controls past the new end are
/// destroyed (`006593e0`; the inlined deleting destructor's flag is the constant 0, so its free
/// never runs) and the end moves. The result is the iterator at `first`. Returns `result`.
pub fn fn_00aae200(
    e: &mut Engine,
    this: Ptr,
    result: Ptr,
    _unused_2: u32,
    first: u32,
    _unused_4: u32,
    last: u32,
) -> Ptr {
    let vector = this.addr();
    e.with_stack(0x10, |e, iterators| {
        let first_iterator = iterators.addr();
        let last_iterator = iterators.addr() + 8;
        let first_proxy = checked_iterator(e, vector, first);
        e.mem.set_u32(first_iterator, first_proxy);
        e.mem.set_u32(first_iterator + 4, first);
        let last_proxy = checked_iterator(e, vector, last);
        e.mem.set_u32(last_iterator, last_proxy);
        e.mem.set_u32(last_iterator + 4, last);
        let differ = e
            .call(
                CONTROL_ITERATORS_DIFFER,
                &args![first_iterator, last_iterator],
            )
            .u8();
        if differ != 0 {
            let end = e.mem.u32(vector + VECTOR_LAST);
            // The three tag words are bytes the callee never reads.
            let new_last = fn_00aaf750(e, last, end, first, 0, 0, 0);
            let end = e.mem.u32(vector + VECTOR_LAST);
            let mut element = new_last;
            while element != end {
                e.call(CONTROL_DESTRUCT, &args![element]);
                element = element.wrapping_add(CONTROL_SIZE);
            }
            e.mem.set_u32(vector + VECTOR_LAST, new_last);
        }
        e.mem.set_u32(result.addr(), first_proxy);
        e.mem.set_u32(result.addr() + 4, first);
    });
    result
}

// Translated from 00aaecb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the linear control iterator's `+=` (`thiscall`, the count). A
/// null proxy fails a check; the moved pointer must lie within the container
/// (`first <= pointer + count * 0x3C <= last`, unsigned; the container is the proxy's first
/// word, read at 0 when the proxy is null) or the check fails. The pointer (+4) moves. Returns
/// `this`.
pub fn fn_00aaecb0(e: &mut Engine, this: Ptr, count: u32) -> Ptr {
    let iterator = this.addr();
    let container = |e: &mut Engine| {
        let proxy = e.mem.u32(iterator);
        if proxy == 0 {
            0
        } else {
            e.mem.u32(proxy)
        }
    };
    if e.mem.u32(iterator) == 0 {
        e.call(INVALID_PARAMETER, &[]);
    }
    let moved = |e: &Engine| {
        count
            .wrapping_mul(CONTROL_SIZE)
            .wrapping_add(e.mem.u32(iterator + 4))
    };
    let owner = container(e);
    let mut ok = moved(e) <= e.mem.u32(owner.wrapping_add(VECTOR_LAST));
    if ok {
        let owner = container(e);
        ok = e.mem.u32(owner.wrapping_add(VECTOR_FIRST)) <= moved(e);
    }
    if !ok {
        e.call(INVALID_PARAMETER, &[]);
    }
    let pointer = moved(e);
    e.mem.set_u32(iterator + 4, pointer);
    this
}

// Translated from 00aaef90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FR2MatrixVTC<float>::operator-=` (unnamed in the engine map; `thiscall`, the other matrix).
/// Matrices of different sizes assert (`fr2Assert(matrixVT.hpp, 0x29e)`). Iterators over this
/// (`0065da80`) and the other (`0065fd10`) walk until either reaches its end (each end iterator
/// checked against its vector, `0065dfd0`); each element of this loses the other's (`0064dda0`;
/// `FLD`, `FSUBR`, `FSTP`) and both advance (`00659190`). Returns `this`.
pub fn fn_00aaef90(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let a = this.addr();
    let b = other.addr();
    if e.mem.u32(a + MATRIX_ROWS) != e.mem.u32(b + MATRIX_ROWS)
        || e.mem.u32(a + MATRIX_COLUMNS) != e.mem.u32(b + MATRIX_COLUMNS)
    {
        e.call(FR2_ASSERT, &args![MATRIX_SOURCE, 0x29eu32]);
    }
    e.with_stack(0x20, |e, locals| {
        let at_a = locals.addr();
        let at_b = locals.addr() + 8;
        let end_a = locals.addr() + 0x10;
        let end_b = locals.addr() + 0x18;
        let first = e.mem.u32(a + VECTOR_FIRST);
        e.call(MAKE_ITERATOR, &args![at_a, first, a]);
        e.call(MATRIX_CONST_BEGIN, &args![b, at_b]);
        loop {
            let last = e.mem.u32(a + VECTOR_LAST);
            let proxy = checked_iterator(e, a, last);
            e.mem.set_u32(end_a, proxy);
            e.mem.set_u32(end_a + 4, last);
            if e.call(ITERATORS_DIFFER, &args![at_a, end_a]).u8() == 0 {
                break;
            }
            let last = e.mem.u32(b + VECTOR_LAST);
            let proxy = checked_iterator(e, b, last);
            e.mem.set_u32(end_b, proxy);
            e.mem.set_u32(end_b + 4, last);
            if e.call(ITERATORS_DIFFER, &args![at_b, end_b]).u8() == 0 {
                break;
            }
            let target = e.call(DEREFERENCE_CHECKED, &args![at_a]).u32();
            let source = e.call(DEREFERENCE_CHECKED, &args![at_b]).u32();
            let difference = x87_sub(x87_m32(e.mem.u32(target)), x87_load32(e.mem.u32(source)));
            e.mem.set_u32(target, x87_store32(difference));
            e.call(PRE_INCREMENT, &args![at_a]);
            e.call(PRE_INCREMENT, &args![at_b]);
        }
    });
    this
}

// Translated from 00aaf110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aaf110 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map: a block of this matrix (`thiscall`, five words: the result, the
/// first row and column, the row and column counts). The code's "below 0" checks of the first
/// row and column are unsigned (they never assert, line 0x425); a block past the matrix asserts
/// (`fr2Assert(matrixVT.hpp, 0x427)`). A temporary of `rows x columns` zeros (`00449fd0`,
/// `0060b410`) gets each row's `columns` floats copied (`0060b370`, `memcpy`) when both counts
/// are above 0; the result is copy-constructed from it (`00658f30`) and it is destroyed
/// (`00449680`). Returns `result`.
// The unsigned "below 0" checks never fail: the assertion is kept as the code has it.
#[allow(unused_comparisons, clippy::absurd_extreme_comparisons)]
pub fn fn_00aaf110(
    e: &mut Engine,
    this: Ptr,
    result: u32,
    row: u32,
    column: u32,
    rows: u32,
    columns: u32,
) -> u32 {
    let matrix = this.addr();
    if column < 0 || row < 0 {
        e.call(FR2_ASSERT, &args![MATRIX_SOURCE, 0x425u32]);
    }
    if column.wrapping_add(columns) > e.mem.u32(matrix + MATRIX_COLUMNS)
        || row.wrapping_add(rows) > e.mem.u32(matrix + MATRIX_ROWS)
    {
        e.call(FR2_ASSERT, &args![MATRIX_SOURCE, 0x427u32]);
    }
    e.with_stack(0x30, |e, locals| {
        let block = locals.addr();
        let out_source = locals.addr() + 0x20;
        let out_block = locals.addr() + 0x28;
        e.call(VECTOR_CONSTRUCT, &args![block]);
        e.mem.set_u32(block + MATRIX_ROWS, rows);
        e.mem.set_u32(block + MATRIX_COLUMNS, columns);
        e.call(
            VECTOR_RESIZE_WITH,
            &args![block, columns.wrapping_mul(rows), x87_store32(0.0)],
        );
        if rows > 0 && columns > 0 {
            let mut index = 0u32;
            while index < e.mem.u32(block + MATRIX_ROWS) {
                let width = e.mem.u32(block + MATRIX_COLUMNS);
                let pointer = e
                    .call(
                        FLOAT_VECTOR_POINTER,
                        &args![matrix, out_source, index.wrapping_add(row)],
                    )
                    .u32();
                let source = e.mem.u32(pointer).wrapping_add(column.wrapping_mul(4));
                let pointer = e
                    .call(FLOAT_VECTOR_POINTER, &args![block, out_block, index])
                    .u32();
                let destination = e.mem.u32(pointer);
                e.call(MEMCPY, &args![destination, source, width.wrapping_shl(2)]);
                index = index.wrapping_add(1);
            }
        }
        e.call(MATRIX_COPY, &args![result, block]);
        e.call(MATRIX_DESTRUCT, &args![block]);
    });
    result
}

// Translated from 00aaf2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: the result, a 2x2 float matrix and a 2-vector): the
/// product `(m0 * v0 + m1 * v1, m2 * v0 + m3 * v1)`, each sum stored as a float. Returns
/// `result`.
pub fn fn_00aaf2b0(e: &mut Engine, result: u32, matrix: u32, vector: u32) -> u32 {
    let m = |e: &Engine, i: u32| e.mem.u32(matrix.wrapping_add(4 * i));
    let v = |e: &Engine, i: u32| e.mem.u32(vector.wrapping_add(4 * i));
    let first = x87_add(
        x87_mul(x87_load32(m(e, 0)), x87_m32(v(e, 0))),
        x87_mul(x87_load32(m(e, 1)), x87_m32(v(e, 1))),
    );
    let first = x87_store32(first);
    let second = x87_add(
        x87_mul(x87_load32(m(e, 2)), x87_m32(v(e, 0))),
        x87_mul(x87_load32(m(e, 3)), x87_m32(v(e, 1))),
    );
    let second = x87_store32(second);
    e.mem.set_u32(result, x87_quiet_bits(first));
    e.mem
        .set_u32(result.wrapping_add(4), x87_quiet_bits(second));
    result
}

// Translated from 00aaf330 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aaf330 - the compiler's exception frame (FS:[0] chain, cookie 011c16bc) is not translated
/// Unnamed in the engine map (`cdecl`: the result and two matrices): `top` stacked on `bottom`.
/// When both are empty (no rows or no columns) the result is an empty 0 x 0 matrix (`00449fd0`,
/// `0060b410`); when one is empty the result is a copy of the other (`00658f30`). Otherwise
/// different column counts assert (`fr2Assert(matrixVT.hpp, 0x4fe)`), and a temporary of
/// `rows(top) + rows(bottom)` rows gets `top`'s floats at its start (`006578b0`, `memcpy`) and
/// `bottom`'s from row `rows(top)` (`0060b370`, `memcpy`); the result is copy-constructed from
/// it and it is destroyed (`00449680`). Returns `result`.
pub fn fn_00aaf330(e: &mut Engine, result: u32, top: u32, bottom: u32) -> u32 {
    let empty = |e: &Engine, m: u32| {
        e.mem.u32(m.wrapping_add(MATRIX_ROWS)) == 0
            || e.mem.u32(m.wrapping_add(MATRIX_COLUMNS)) == 0
    };
    let top_empty = empty(e, top);
    let bottom_empty = empty(e, bottom);
    if top_empty && bottom_empty {
        e.call(VECTOR_CONSTRUCT, &args![result]);
        e.mem.set_u32(result.wrapping_add(MATRIX_ROWS), 0);
        e.mem.set_u32(result.wrapping_add(MATRIX_COLUMNS), 0);
        let count = e
            .mem
            .u32(result.wrapping_add(MATRIX_COLUMNS))
            .wrapping_mul(e.mem.u32(result.wrapping_add(MATRIX_ROWS)));
        e.call(VECTOR_RESIZE_WITH, &args![result, count, x87_store32(0.0)]);
        return result;
    }
    if top_empty {
        e.call(MATRIX_COPY, &args![result, bottom]);
        return result;
    }
    if bottom_empty {
        e.call(MATRIX_COPY, &args![result, top]);
        return result;
    }
    if e.mem.u32(top.wrapping_add(MATRIX_COLUMNS)) != e.mem.u32(bottom.wrapping_add(MATRIX_COLUMNS))
    {
        e.call(FR2_ASSERT, &args![MATRIX_SOURCE, 0x4feu32]);
    }
    let columns = e.mem.u32(top.wrapping_add(MATRIX_COLUMNS));
    let top_rows = e.mem.u32(top.wrapping_add(MATRIX_ROWS));
    let bottom_rows = e.mem.u32(bottom.wrapping_add(MATRIX_ROWS));
    e.with_stack(0x30, |e, locals| {
        let stacked = locals.addr();
        let out_bottom = locals.addr() + 0x20;
        let out_stacked = locals.addr() + 0x28;
        e.call(VECTOR_CONSTRUCT, &args![stacked]);
        e.mem
            .set_u32(stacked + MATRIX_ROWS, top_rows.wrapping_add(bottom_rows));
        e.mem.set_u32(stacked + MATRIX_COLUMNS, columns);
        let count = e
            .mem
            .u32(stacked + MATRIX_COLUMNS)
            .wrapping_mul(e.mem.u32(stacked + MATRIX_ROWS));
        e.call(VECTOR_RESIZE_WITH, &args![stacked, count, x87_store32(0.0)]);
        let rows = e.mem.u32(top.wrapping_add(MATRIX_ROWS));
        let width = e.mem.u32(top.wrapping_add(MATRIX_COLUMNS));
        let source = e.call(MATRIX_FLOAT_AT, &args![top, 0u32]).u32();
        let destination = e.call(MATRIX_FLOAT_AT, &args![stacked, 0u32]).u32();
        e.call(
            MEMCPY,
            &args![
                destination,
                source,
                rows.wrapping_mul(width).wrapping_shl(2)
            ],
        );
        let rows = e.mem.u32(bottom.wrapping_add(MATRIX_ROWS));
        let width = e.mem.u32(bottom.wrapping_add(MATRIX_COLUMNS));
        let pointer = e
            .call(FLOAT_VECTOR_POINTER, &args![bottom, out_bottom, 0u32])
            .u32();
        let source = e.mem.u32(pointer);
        let offset = e.mem.u32(top.wrapping_add(MATRIX_ROWS));
        let pointer = e
            .call(FLOAT_VECTOR_POINTER, &args![stacked, out_stacked, offset])
            .u32();
        let destination = e.mem.u32(pointer);
        e.call(
            MEMCPY,
            &args![
                destination,
                source,
                rows.wrapping_mul(width).wrapping_shl(2)
            ],
        );
        e.call(MATRIX_COPY, &args![result, stacked]);
        e.call(MATRIX_DESTRUCT, &args![stacked]);
    });
    result
}

/// One control assigned from another: the matrix (`006581e0`) and the label (`0044a580` with
/// offset 0 and `npos`).
fn assign_control(e: &mut Engine, destination: u32, source: u32) {
    e.call(MATRIX_ASSIGN, &args![destination, source]);
    let npos: u32 = e.global(STRING_NPOS);
    e.call(
        STRING_ASSIGN,
        &args![
            destination.wrapping_add(CONTROL_LABEL),
            source.wrapping_add(CONTROL_LABEL),
            0u32,
            npos
        ],
    );
}

// Translated from 00aaf6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: first, last and a control): assigns the control to each
/// control from `first` to `last` (matrix `006581e0`, label `0044a580`).
pub fn fn_00aaf6f0(e: &mut Engine, first: u32, last: u32, value: u32) {
    let mut element = first;
    while element != last {
        assign_control(e, element, value);
        element = element.wrapping_add(CONTROL_SIZE);
    }
}

// Translated from 00aaf750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words: first, last, destination and three tag
/// words it does not read): assigns the controls from `first` to `last` to those from
/// `destination` on, in order. Returns the destination's end, `destination + (last - first) /
/// 0x3C * 0x3C` (a signed division).
pub fn fn_00aaf750(
    e: &mut Engine,
    first: u32,
    last: u32,
    destination: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    let end = ((last.wrapping_sub(first) as i32 / CONTROL_SIZE as i32) as u32)
        .wrapping_mul(CONTROL_SIZE)
        .wrapping_add(destination);
    let mut target = destination;
    let mut element = first;
    while element != last {
        assign_control(e, target, element);
        target = target.wrapping_add(CONTROL_SIZE);
        element = element.wrapping_add(CONTROL_SIZE);
    }
    end
}

// Translated from 00aaf860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, six words: first, last, end and three tag bytes, of
/// which only the first is read): the controls' backward copy through `00aaf9a0` with the
/// first tag byte, a byte of its own and 0. Answers what `00aaf9a0` answers (left in `EAX`).
pub fn fn_00aaf860(
    e: &mut Engine,
    first: u32,
    last: u32,
    end: u32,
    tag: u32,
    _unused_4: u32,
    _unused_5: u32,
) -> u32 {
    // The second tag pushed is an uninitialized stack byte; `00aaf9a0` reads none of the three.
    e.call(
        CONTROL_COPY_BACKWARD,
        &args![first, last, end, tag & 0xff, 0u32, 0u32],
    )
    .u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0065_8650, fn_00658650(Ptr, f32) -> Ptr),
        entry!(
            0x00aa_af10,
            fan_gen_ctls_grp_c_read_from_file(Ptr, Ptr, u32) -> bool
        ),
        entry!(0x00aa_bdb0, fn_00aabdb0(Ptr, i32, i32, i32, u32) -> f32),
        entry!(0x00aa_dd20, fn_00aadd20(u32, u32) -> u32),
        entry!(0x00aa_dda0, fn_00aadda0(Ptr, u32, f32) -> u32),
        entry!(0x00aa_dde0, fn_00aadde0(Ptr, u32)),
        entry!(0x00aa_de20, fn_00aade20(Ptr) -> Ptr),
        entry!(0x00aa_deb0, fn_00aadeb0(Ptr, u32, u8) -> Ptr),
        entry!(0x00aa_df50, fn_00aadf50_entry(Ptr, u32, WordsArg<15>)),
        entry!(0x00aa_e180, fn_00aae180(Ptr, u32, u32, u8)),
        entry!(0x00aa_e3a0, fn_00aae3a0(Ptr, u32, u32, u32, u32)),
        entry!(0x00aa_ec10, fn_00aaec10(Ptr, Ptr) -> Ptr),
        entry!(0x00aa_ed70, fn_00aaed70(Ptr, u32, Ptr) -> u32),
        entry!(0x00aa_f7e0, fn_00aaf7e0(u32, u32, u32, u32) -> u32),
        entry!(0x00aa_f8a0, fn_00aaf8a0(u32, u32, u32, u32, u32, u32)),
        entry!(0x00aa_fa30, fn_00aafa30(u32, u32, u32, u32) -> u32),
        entry!(0x00aa_bf30, fn_00aabf30(Ptr, i32, i32, i32, f32, u32)),
        entry!(0x00aa_c170, fn_00aac170(Ptr, i32, i32, u32, u32)),
        entry!(0x00aa_c3f0, fn_00aac3f0(Ptr, i32, i32, i32, u32) -> f32),
        entry!(0x00aa_c820, fn_00aac820(Ptr, i32, i32, i32, f32, u32)),
        entry!(0x00aa_cdb0, fn_00aacdb0(Ptr, i32, i32, u32) -> f32),
        entry!(0x00aa_cf40, fn_00aacf40(Ptr, i32, i32, f32, u32)),
        entry!(0x00aa_d140, fn_00aad140(Ptr, i32, u32)),
        entry!(0x00aa_d500, fn_00aad500(Ptr, f32, u32, u32)),
        entry!(0x00aa_d6b0, fn_00aad6b0(Ptr, f32, u32, u32)),
        entry!(0x00aa_d890, fan_gen_ctls_grp_c_reflect(Ptr, i32, u32)),
        entry!(
            0x00aa_dac0,
            fan_gen_ctls_grp_c_tween(Ptr, i32, i32, f32, u32, u32, u32)
        ),
        entry!(
            0x00aa_e200,
            fn_00aae200(Ptr, Ptr, u32, u32, u32, u32) -> Ptr
        ),
        entry!(0x00aa_ecb0, fn_00aaecb0(Ptr, u32) -> Ptr),
        entry!(0x00aa_ef90, fn_00aaef90(Ptr, Ptr) -> Ptr),
        entry!(
            0x00aa_f110,
            fn_00aaf110(Ptr, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00aa_f2b0, fn_00aaf2b0(u32, u32, u32) -> u32),
        entry!(0x00aa_f330, fn_00aaf330(u32, u32, u32) -> u32),
        entry!(0x00aa_f6f0, fn_00aaf6f0(u32, u32, u32)),
        entry!(
            0x00aa_f750,
            fn_00aaf750(u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(
            0x00aa_f860,
            fn_00aaf860(u32, u32, u32, u32, u32, u32) -> u32
        ),
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

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// Doubles for the float vectors: a resize allocates `count` floats (the block at +0x0C),
    /// the element pointer gives that block, the strings and the allocator are quiet.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x10000);
        e.set_global(STRING_NPOS, u32::MAX);
        e.set_global(NORM_THRESHOLD, 0.001f64);
        e.register(VECTOR_RESIZE_WITH, |e, a| {
            let block = e.mem.alloc((a[1] * 4).max(4));
            for i in 0..a[1] {
                e.mem.set_u32(block + 4 * i, a[2]);
            }
            e.mem.set_u32(a[0] + 0xc, block);
            e.mem.set_u32(a[0] + 0x10, block + 4 * a[1]);
            Ret::default()
        });
        e.register(FLOAT_VECTOR_POINTER, |e, a| {
            let block = e.mem.u32(a[0] + 0xc);
            e.mem.set_u32(a[1], block + 4 * a[2]);
            a[1].into_ret()
        });
        e.register(MEMSET, |e, a| {
            let fill = vec![a[1] as u8; a[2] as usize];
            e.mem.write(a[0], &fill);
            Ret::default()
        });
        quiet(
            &mut e,
            &[
                FR2_ASSERT,
                STRING_TIDY,
                STRING_ASSIGN,
                STRING_COPY,
                MEMORY_MANAGER_FREE,
                FLOAT_VECTOR_TIDY,
                VECTOR_FREE_PROXY,
                VECTOR_CONSTRUCT,
                MATRIX_COPY,
                MATRIX_DESTRUCT,
                MATRIX_ASSIGN,
                MATRIX_TIMES_FLOAT,
                CONTROL_MATRIX_INIT,
                CONTROL_DESTRUCT,
                CONTROL_DELETING_DESTRUCT,
                CONTROL_ASSIGN,
                CONTROL_MOVE_BACKWARD,
                MAKE_ITERATOR,
                PRE_INCREMENT,
                POST_INCREMENT,
            ],
        );
        e
    }

    #[test]
    fn scale_multiplies_each_element() {
        let mut e = engine();
        let floats = e.mem.alloc(8);
        e.mem.set_f32(floats, 1.5);
        e.mem.set_f32(floats + 4, -2.0);
        // The iterator walks the two floats: begin stores the pointer, the end compare stops
        // after two, the dereference reads it and the post-increment advances it.
        e.register_double(MATRIX_BEGIN, move |e, a| {
            e.mem.set_u32(a[1] + 4, floats);
            Ret::default()
        });
        e.register(MATRIX_END, |_, a| a[1].into_ret());
        e.register_double(ITERATORS_DIFFER, move |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != floats + 8).into_ret()
        });
        e.register(DEREFERENCE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(POST_INCREMENT, |e, a| {
            let pointer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, pointer + 4);
            Ret::default()
        });
        let matrix = e.mem.alloc(0x20);
        assert_eq!(e.call(0x0065_8650, &args![matrix, 3.0f32]).u32(), matrix);
        assert_eq!(e.mem.f32(floats), 4.5);
        assert_eq!(e.mem.f32(floats + 4), -6.0);
    }

    #[test]
    fn control_value_adds_the_offset_and_asserts_bounds() {
        let mut e = engine();
        let this = e.mem.alloc(0x1000);
        e.register(MATRIX_PRODUCT, |e, a| {
            let block = e.mem.alloc(4);
            e.mem.set_f32(block, 2.5);
            e.mem.set_u32(a[1] + 0xc, block);
            a[1].into_ret()
        });
        // Not loaded: 0.0, but the bounds are still checked.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_bdb0, &args![this, 5i32, 2i32, 1i32, 0u32])
                .f32(),
            0.0
        );
        let log = e.call_log.take().unwrap();
        let lines: Vec<u32> = calls_to(&log, FR2_ASSERT).iter().map(|w| w[1]).collect();
        assert_eq!(lines, vec![0xc6, 0xc7]);
        e.mem.set_u8(this, 1);
        let control = this + 0x324 + 2 * 0xa0 + 0x50 + 0x28;
        e.mem.set_f32(control + 0x24, 0.25);
        let table = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_bdb0, &args![this, 2i32, 1i32, 1i32, table])
                .f32(),
            2.75
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MATRIX_PRODUCT)[0][0], control);
        assert_eq!(calls_to(&log, MATRIX_PRODUCT)[0][2], table + 0x40);
    }

    #[test]
    fn inverse_of_a_2x2_matrix() {
        let mut e = engine();
        let matrix = e.mem.alloc(16);
        for (i, v) in [4.0f32, 7.0, 2.0, 6.0].iter().enumerate() {
            e.mem.set_f32(matrix + 4 * i as u32, *v);
        }
        let result = e.mem.alloc(16);
        assert_eq!(e.call(0x00aa_dd20, &args![result, matrix]).u32(), result);
        let r: Vec<f32> = (0..4).map(|i| e.mem.f32(result + 4 * i)).collect();
        assert_eq!(r, vec![0.6, -0.7, -0.2, 0.4]);
    }

    #[test]
    fn divide_multiplies_by_the_reciprocal() {
        let mut e = engine();
        let matrix = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00aa_dda0, &args![matrix, 0x500u32, 4.0f32]).u32(),
            0x500
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MATRIX_TIMES_FLOAT),
            vec![vec![matrix, 0x500, 0.25f32.to_bits()]]
        );
    }

    #[test]
    fn default_control_and_its_copy() {
        let mut e = engine();
        let control = e.mem.alloc(CONTROL_SIZE);
        e.mem.set_u32(control + MATRIX_ROWS, 5);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_de20, &args![control]).u32(), control);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(control + MATRIX_ROWS), 0);
        assert_eq!(calls_to(&log, CONTROL_MATRIX_INIT), vec![vec![control]]);
        assert_eq!(
            calls_to(&log, STRING_TIDY),
            vec![vec![control + 0x20, 0, 0]]
        );
        let copy = e.mem.alloc(CONTROL_SIZE);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_ec10, &args![copy, control]).u32(), copy);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MATRIX_COPY), vec![vec![copy, control]]);
        assert_eq!(
            calls_to(&log, STRING_ASSIGN),
            vec![vec![copy + 0x20, control + 0x20, 0, u32::MAX]]
        );
    }

    #[test]
    fn string_append_grows_and_fills() {
        let mut e = engine();
        let string = e.mem.alloc(0x1c);
        e.mem.write(string + 4, b"ab\0");
        e.mem.set_u32(string + STRING_LENGTH, 2);
        e.mem.set_u32(string + 0x18, 15);
        e.register(STRING_GROW, |_, _| 1u32.into_ret());
        quiet(&mut e, &[STRING_THROW_LENGTH]);
        assert_eq!(
            e.call(0x00aa_deb0, &args![string, 3u32, b'x']).u32(),
            string
        );
        // Three characters through memset (the double above zero-fills): length 5, then 0.
        assert_eq!(e.mem.u32(string + STRING_LENGTH), 5);
        assert_eq!(e.mem.bytes(string + 4, 6), b"abxxx\0".to_vec());
        // One character is stored directly.
        e.call_log = Some(vec![]);
        e.call(0x00aa_deb0, &args![string, 1u32, b'y']);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MEMSET).is_empty());
        assert_eq!(e.mem.bytes(string + 4, 7), b"abxxxy\0".to_vec());
        // Nothing for 0; a failed grow leaves the string.
        e.call(0x00aa_deb0, &args![string, 0u32, b'z']);
        e.register(STRING_GROW, |_, _| Ret::default());
        e.call(0x00aa_deb0, &args![string, 2u32, b'z']);
        assert_eq!(e.mem.u32(string + STRING_LENGTH), 6);
        // A count reaching npos - length throws.
        e.call_log = Some(vec![]);
        e.call(0x00aa_deb0, &args![string, u32::MAX - 6, b'z']);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, STRING_THROW_LENGTH).len(), 1);
        // The character assignment on its own: a sign-extended memset fill.
        e.call_log = Some(vec![]);
        e.call(0x00aa_e180, &args![string, 0u32, 2u32, 0x80u8]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MEMSET),
            vec![vec![string + 4, 0xffff_ff80, 2]]
        );
    }

    /// Doubles for the control vector's helpers outside the unit: the allocation and the
    /// uninitialized copy (`00aafa80`, copying each control's words).
    fn control_engine() -> Engine {
        let mut e = engine();
        e.register(CONTROL_ALLOCATE, |e, a| {
            e.mem.alloc((a[0] * CONTROL_SIZE).max(4)).into_ret()
        });
        e.register(CONTROL_UNINITIALIZED_COPY, |e, a| {
            let bytes = e.mem.bytes(a[0], a[1] - a[0]);
            if a[2] != 0 && !bytes.is_empty() {
                e.mem.write(a[2], &bytes);
            }
            (a[2] + (a[1] - a[0])).into_ret()
        });
        e.register(MATRIX_COPY, |e, a| {
            let word = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], word);
            Ret::default()
        });
        e
    }

    fn control_vector(e: &mut Engine, tags: &[u32], capacity: u32) -> Ptr {
        let vector = e.mem.alloc(0x18);
        let block = e.mem.alloc((capacity * CONTROL_SIZE).max(4));
        for (i, tag) in tags.iter().enumerate() {
            e.mem.set_u32(block + CONTROL_SIZE * i as u32, *tag);
        }
        e.mem.set_u32(vector + VECTOR_FIRST, block);
        e.mem.set_u32(
            vector + VECTOR_LAST,
            block + CONTROL_SIZE * tags.len() as u32,
        );
        e.mem
            .set_u32(vector + VECTOR_END, block + CONTROL_SIZE * capacity);
        Ptr::new(vector)
    }

    fn tags_of(e: &Engine, vector: Ptr) -> Vec<u32> {
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        let last = e.mem.u32(vector.addr() + VECTOR_LAST);
        (first..last)
            .step_by(CONTROL_SIZE as usize)
            .map(|a| e.mem.u32(a))
            .collect()
    }

    #[test]
    fn control_resize_works_on_the_control_where_it_was_built() {
        // 00aaddef..00aade0c: 00aadde0 builds the default control (00aade20) in 00aadf50's
        // argument area, so the matrix's proxy (00449fd0: it points back at the matrix) names
        // the control the resize copies from (00aae3a0) and destroys there (00aae163).
        let mut e = control_engine();
        e.register_double(VECTOR_CONSTRUCT, |e, a| {
            let proxy = e.mem.alloc(4);
            e.mem.set_u32(proxy, a[0]);
            e.mem.set_u32(a[0], proxy);
            Ret::default()
        });
        let destroyed = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let sink = destroyed.clone();
        e.register_double(CONTROL_DESTRUCT, move |e, a| {
            let proxy = e.mem.u32(a[0]);
            sink.borrow_mut().push((a[0], e.mem.u32(proxy)));
            Ret::default()
        });
        let vector = control_vector(&mut e, &[], 0);
        e.call_log = Some(vec![]);
        e.call(0x00aa_dde0, &args![vector, 1u32]);
        let log = e.call_log.take().unwrap();
        let built = calls_to(&log, VECTOR_CONSTRUCT)[0][0];
        let copies = calls_to(&log, MATRIX_COPY);
        assert!(!copies.is_empty());
        assert!(copies.iter().all(|copy| copy[1] == built));
        assert_eq!(*destroyed.borrow(), vec![(built, built)]);
        assert_eq!(tags_of(&e, vector).len(), 1);
    }

    #[test]
    fn control_resize_and_insert() {
        let mut e = control_engine();
        // Resize an empty vector to two default controls (00aadde0 -> 00aadf50 -> 00aae3a0).
        let vector = control_vector(&mut e, &[], 0);
        e.call_log = Some(vec![]);
        e.call(0x00aa_dde0, &args![vector, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CONTROL_ALLOCATE), vec![vec![2, 0]]);
        assert_eq!(tags_of(&e, vector).len(), 2);
        // The value is destroyed after the resize.
        assert_eq!(calls_to(&log, CONTROL_DESTRUCT).len(), 1);

        // Reallocating insert: the old controls are destroyed with flag 0.
        let vector = control_vector(&mut e, &[1, 2], 2);
        let value = e.mem.alloc(CONTROL_SIZE);
        e.mem.set_u32(value, 9);
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        e.call_log = Some(vec![]);
        e.call(
            0x00aa_e3a0,
            &args![vector, 0u32, first + CONTROL_SIZE, 1u32, value],
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(tags_of(&e, vector), vec![1, 9, 2]);
        assert_eq!(
            calls_to(&log, CONTROL_DELETING_DESTRUCT),
            vec![vec![first, 0], vec![first + CONTROL_SIZE, 0]]
        );

        // With room: the copy (by its two constructors) and the backward move.
        let vector = control_vector(&mut e, &[1, 2, 3], 8);
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        e.call_log = Some(vec![]);
        e.call(0x00aa_e3a0, &args![vector, 0u32, first, 1u32, value]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, STRING_COPY).len(), 1);
        assert_eq!(calls_to(&log, CONTROL_MOVE_BACKWARD).len(), 1);
        assert_eq!(calls_to(&log, MATRIX_DESTRUCT).len(), 1);
        assert_eq!(tags_of(&e, vector).len(), 4);
        // With room at the end: the tail is filled with copies.
        let last = e.mem.u32(vector.addr() + VECTOR_LAST);
        e.call_log = Some(vec![]);
        e.call(0x00aa_e3a0, &args![vector, 0u32, last, 2u32, value]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CONTROL_DESTRUCT).len(), 1);
        assert_eq!(tags_of(&e, vector)[4..], [9, 9]);
        // Shrinking resizes erase through 00aae200 after advancing.
        quiet(&mut e, &[CONTROL_ADVANCE, CONTROL_ERASE]);
        e.call_log = Some(vec![]);
        e.call(0x00aa_dde0, &args![vector, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, CONTROL_ERASE).len(), 1);
        assert_eq!(calls_to(&log, CONTROL_ADVANCE)[0][1], 1);
    }

    #[test]
    fn control_copies_and_fills() {
        let mut e = control_engine();
        let block = e.mem.alloc(CONTROL_SIZE * 2);
        e.call_log = Some(vec![]);
        e.call(0x00aa_f8a0, &args![block, 2u32, block, 0u32, 0u32, 0u32]);
        e.call(0x00aa_f8a0, &args![0u32, 1u32, block, 0u32, 0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MATRIX_COPY),
            vec![vec![block, block], vec![block + CONTROL_SIZE, block]]
        );
        let target = e.mem.alloc(CONTROL_SIZE * 2);
        for addr in [0x00aa_f7e0u32, 0x00aa_fa30] {
            e.call_log = Some(vec![]);
            let end = e
                .call(addr, &args![block, block + CONTROL_SIZE, target, 0u32])
                .u32();
            let log = e.call_log.take().unwrap();
            assert_eq!(end, target + CONTROL_SIZE);
            assert_eq!(
                calls_to(&log, CONTROL_UNINITIALIZED_COPY),
                vec![vec![block, block + CONTROL_SIZE, target, 0, 0, 0]]
            );
        }
    }

    #[test]
    fn matrix_difference_walks_both_matrices() {
        let mut e = engine();
        let a = e.mem.alloc(0x20);
        let b = e.mem.alloc(0x20);
        let left = e.mem.alloc(8);
        let right = e.mem.alloc(8);
        e.mem.set_f32(left, 5.0);
        e.mem.set_f32(left + 4, 1.0);
        e.mem.set_f32(right, 2.0);
        e.mem.set_f32(right + 4, 4.0);
        for (m, block) in [(a, left), (b, right)] {
            e.mem.set_u32(m + MATRIX_ROWS, 1);
            e.mem.set_u32(m + MATRIX_COLUMNS, 2);
            e.mem.set_u32(m + VECTOR_FIRST, block);
            e.mem.set_u32(m + VECTOR_LAST, block + 8);
        }
        // Iterators hold the pointer at +4; the end is this matrix's last pointer.
        e.register(MAKE_ITERATOR, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(ITERATORS_DIFFER, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        e.register(DEREFERENCE_CHECKED, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(PRE_INCREMENT, |e, a| {
            let pointer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, pointer + 4);
            Ret::default()
        });
        let result = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_ed70, &args![a, result, b]).u32(), result);
        let log = e.call_log.take().unwrap();
        let copy = calls_to(&log, MATRIX_COPY);
        assert_eq!(copy.len(), 1);
        let temporary = copy[0][1];
        let floats = e.mem.u32(temporary + 0xc);
        assert_eq!(e.mem.f32(floats), 3.0);
        assert_eq!(e.mem.f32(floats + 4), -3.0);
        assert!(calls_to(&log, FR2_ASSERT).is_empty());
        // Different sizes assert.
        e.mem.set_u32(b + MATRIX_ROWS, 2);
        e.call_log = Some(vec![]);
        e.call(0x00aa_ed70, &args![a, result, b]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FR2_ASSERT), vec![vec![MATRIX_SOURCE, 0x1ca]]);
    }

    /// An engine for `readFromFile`: every read succeeds (zero bytes) until `fail_at` reads,
    /// norms are 4.0, the transpose records its source and the product gives 2.0 for a
    /// control times its own transpose, else 0.0.
    fn reader(fail_at: usize) -> (Engine, Ptr, Ptr, u32) {
        let mut e = engine();
        let mut reads = 0usize;
        e.register_double(0x0040_8000, move |e, a| {
            reads += 1;
            if reads == fail_at {
                return Ret::default();
            }
            let zeros = vec![0u8; (a[2] * a[3]) as usize];
            e.mem.write(a[1], &zeros);
            1u32.into_ret()
        });
        e.put_vtable(0x0090_b000, &[0, 0, 0, 0, 0x0040_8000]);
        let file = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(file.addr(), 0x0090_b000);
        e.register(MATRIX_NORM, |_, _| st0(4.0));
        e.register(SQRT, |_, a| st0(f64::take(a, &mut 0).sqrt()));
        e.register(MATRIX_TRANSPOSE, |e, a| {
            e.mem.set_u32(a[1] + 0x18, a[0]);
            a[1].into_ret()
        });
        e.register(MATRIX_PRODUCT, |e, a| {
            let source = e.mem.u32(a[2] + 0x18);
            let block = e.mem.alloc(4);
            e.mem.set_f32(block, if source == a[0] { 2.0 } else { 0.0 });
            e.mem.set_u32(a[1] + 0xc, block);
            a[1].into_ret()
        });
        e.register(ITERATORS_DIFFER, |_, _| Ret::default());
        e.register(MATRIX_BEGIN, |_, _| Ret::default());
        e.register(MATRIX_END, |_, a| a[1].into_ret());
        e.register(MATRIX_TIMES_FLOAT, |e, a| {
            let rows = e.mem.u32(a[0] + MATRIX_ROWS);
            let columns = e.mem.u32(a[0] + MATRIX_COLUMNS);
            e.mem.set_u32(a[1] + MATRIX_ROWS, rows);
            e.mem.set_u32(a[1] + MATRIX_COLUMNS, columns);
            a[1].into_ret()
        });
        e.register(MATRIX_ASSIGN, |e, a| {
            let rows = e.mem.u32(a[1] + MATRIX_ROWS);
            let columns = e.mem.u32(a[1] + MATRIX_COLUMNS);
            e.mem.set_u32(a[0] + MATRIX_ROWS, rows);
            e.mem.set_u32(a[0] + MATRIX_COLUMNS, columns);
            a[0].into_ret()
        });
        let this = Ptr::new(e.mem.alloc(0x1000));
        let table = e.mem.alloc(0x10);
        e.mem.set_u32(table, 1);
        e.mem.set_u32(table + 8, 2);
        (e, this, file, table)
    }

    #[test]
    fn read_from_file_reads_every_part() {
        let (mut e, this, file, table) = reader(0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00aa_af10, &args![this, file, table]).bool());
        let log = e.call_log.take().unwrap();
        let base = this.addr();
        assert_eq!(e.mem.u8(base), 1);
        // Linear controls: 1 x n[j][0] matrices, inverse norm 1/4.
        assert_eq!(e.mem.u32(base + ATTRIBUTES + 0x28 + MATRIX_COLUMNS), 2);
        assert_eq!(e.mem.f32(base + ATTRIBUTES + 0x20), 0.25);
        // Race offsets: the inverse of the two norms' sum, 1/8.
        assert_eq!(e.mem.f32(base + RACE_OFFSETS + 0x48 + 0x40), 0.125);
        // Distributions: a 3 x 3 covariance (1 + 2).
        assert_eq!(e.mem.u32(base + 0x44 + MATRIX_ROWS), 3);
        // The inverse covariance of diag(2, 2).
        let inverse: Vec<f32> = (0..4)
            .map(|i| e.mem.f32(base + INVERSE_COVARIANCE + 4 * i))
            .collect();
        assert_eq!(inverse, vec![0.5, 0.0, 0.0, 0.5]);
        // No norm below 0.001, so no assertion.
        assert!(calls_to(&log, FR2_ASSERT).is_empty());
        // Each basis' second matrix is scaled by its inverse norm (sqrt(4) = 2).
        assert_eq!(calls_to(&log, SQRT).len(), 20);
    }

    #[test]
    fn read_from_file_stops_at_a_failed_read_and_asserts_small_norms() {
        for fail_at in [1usize, 2, 41, 43, 101, 103] {
            let (mut e, this, file, table) = reader(fail_at);
            assert!(
                !e.call(0x00aa_af10, &args![this, file, table]).bool(),
                "{fail_at}"
            );
            assert_eq!(e.mem.u8(this.addr()), 0);
        }
        let (mut e, this, file, table) = reader(0);
        e.register(MATRIX_NORM, |_, _| st0(0.0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x00aa_af10, &args![this, file, table]).bool());
        let log = e.call_log.take().unwrap();
        let lines: Vec<u32> = calls_to(&log, FR2_ASSERT).iter().map(|w| w[1]).collect();
        assert_eq!(lines.len(), 20);
        assert_eq!(&lines[..2], &[0x90, 0x98]);
    }

    #[test]
    fn control_resize_with_value_takes_fifteen_words() {
        let mut e = control_engine();
        let vector = control_vector(&mut e, &[], 0);
        let mut call = vec![vector.addr(), 1];
        call.extend(7..22u32);
        e.call_log = Some(vec![]);
        e.call(0x00aa_df50, &call);
        let log = e.call_log.take().unwrap();
        assert_eq!(tags_of(&e, vector), vec![7]);
        let destroyed = calls_to(&log, CONTROL_DESTRUCT);
        assert_eq!(destroyed.len(), 1);
        assert_eq!(e.mem.u32(destroyed[0][0]), 7);
    }

    /// The `double` argument of a `cdecl` call (two words).
    fn double_arg(a: &[u32]) -> f64 {
        f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32)
    }

    /// An engine for the control functions: the constants mapped, matrix products answering
    /// `product` as their first float (and 6 rows), transposes and scalings answering their
    /// result slot, `sqrt` computed, the iterator compare quiet.
    fn controls_engine(product: f32) -> Engine {
        let mut e = engine();
        e.map(0x0102_0000, 0x10000);
        e.map(0x0107_0000, 0x10000);
        e.set_global(LOWER_BOUND_0, 15.0f32);
        e.set_global(LOWER_BOUND_1, -4.0f32);
        e.set_global(UPPER_BOUND_0, 65.0f32);
        e.set_global(UPPER_BOUND_1, 4.0f32);
        e.set_global(SMALLEST_VALUE, 0.01f64);
        e.set_global(SMALLEST_VALUE_FLOAT, 0.01f32);
        e.set_global(DOUBLE_ZERO, 0.0f64);
        e.set_global(DOUBLE_ONE, 1.0f64);
        e.set_global(DOUBLE_TWO, 2.0f64);
        e.set_global(DOUBLE_ONE_AND_A_HALF, 1.5f64);
        e.set_global(FLOAT_MINUS_ONE, -1.0f32);
        e.set_global(FLOAT_TWO, 2.0f32);
        e.register_double(MATRIX_PRODUCT, move |e, a| {
            let block = e.mem.alloc(4);
            e.mem.set_f32(block, product);
            e.mem.set_u32(a[1] + 0xc, block);
            e.mem.set_u32(a[1] + MATRIX_ROWS, 6);
            a[1].into_ret()
        });
        e.register(MATRIX_TRANSPOSE, |_, a| a[1].into_ret());
        e.register(MATRIX_TIMES_FLOAT, |_, a| a[1].into_ret());
        e.register(MATRIX_SUM, |_, a| a[1].into_ret());
        e.register(SQRT, |_, a| st0(double_arg(a).sqrt()));
        quiet(
            &mut e,
            &[
                MATRIX_ADD_ASSIGN,
                MATRIX_CLEAR,
                ITERATORS_DIFFER,
                MATRIX_CONST_BEGIN,
                INVALID_PARAMETER,
            ],
        );
        e
    }

    /// A loaded control set (byte +0 set).
    fn control_set(e: &mut Engine) -> Ptr {
        let this = e.mem.alloc(0x1000);
        e.mem.set_u8(this, 1);
        Ptr::new(this)
    }

    /// Makes `00658650`'s walk visit one float (returned, 1.0 at first) and stop: its begin
    /// iterator is marked, and only a marked iterator differs from its end (once). Other
    /// iterator walks stop at once.
    fn scale_probe(e: &mut Engine) -> u32 {
        const MARK: u32 = 0x5ca1_e000;
        let element = e.mem.alloc(4);
        e.mem.set_f32(element, 1.0);
        e.register(ITERATORS_DIFFER, |e, a| {
            let marked = e.mem.u32(a[0]) == MARK;
            e.mem.set_u32(a[0], 0);
            u32::from(marked).into_ret()
        });
        e.register(MATRIX_BEGIN, |e, a| {
            e.mem.set_u32(a[1], MARK);
            Ret::default()
        });
        e.register(MATRIX_END, |_, a| a[1].into_ret());
        e.register_double(DEREFERENCE, move |_, _| element.into_ret());
        element
    }

    fn floats_of(log: &Log, addr: u32, word: usize) -> Vec<f32> {
        calls_to(log, addr)
            .iter()
            .map(|w| f32::from_bits(w[word]))
            .collect()
    }

    #[test]
    fn set_control_clamps_and_adds_the_scaled_transpose() {
        let mut e = controls_engine(1.0);
        let this = control_set(&mut e);
        let base = this.addr();
        // Control [0][1] of group 0 has the offset 0.5: its current value is 1.5.
        e.mem.set_f32(base + 0x324 + 0x28 + 0x24, 0.5);
        e.mem.set_f32(base + 0x324 + 0x50 + 0x28 + 0x24, 0.5);
        let covariance = base + 0xfcc + 0x10;
        for (i, v) in [2.0f32, 0.0, 0.0, 3.0].iter().enumerate() {
            e.mem.set_f32(covariance + 4 * i as u32, *v);
        }
        let table = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        e.call(0x00aa_bf30, &args![this, 0i32, 0i32, 1i32, 100.0f32, table]);
        let log = e.call_log.take().unwrap();
        // 100 is clamped to 65: (65 - 1.5) * 2 = 127.
        assert_eq!(floats_of(&log, MATRIX_TIMES_FLOAT, 2), vec![127.0]);
        assert_eq!(calls_to(&log, MATRIX_TRANSPOSE)[0][0], base + 0x324 + 0x28);
        assert_eq!(calls_to(&log, MATRIX_ADD_ASSIGN)[0][0], table + 0x40);
        assert_eq!(calls_to(&log, MEMORY_MANAGER_FREE).len(), 3);
        // -10 at index 1 is clamped to -4: (-4 - 1.5) * 3 = -16.5.
        e.call_log = Some(vec![]);
        e.call(0x00aa_bf30, &args![this, 0i32, 1i32, 1i32, -10.0f32, table]);
        let log = e.call_log.take().unwrap();
        assert_eq!(floats_of(&log, MATRIX_TIMES_FLOAT, 2), vec![-16.5]);
        assert!(calls_to(&log, FR2_ASSERT).is_empty());
        // Group 5 asserts; an unloaded set does nothing more.
        e.mem.set_u8(base, 0);
        e.call_log = Some(vec![]);
        e.call(0x00aa_bf30, &args![this, 5i32, 0i32, 1i32, 1.0f32, table]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FR2_ASSERT),
            vec![vec![FAN_CONTROLS_SOURCE, 0xde]]
        );
        assert!(calls_to(&log, MATRIX_TIMES_FLOAT).is_empty());
    }

    #[test]
    fn set_column_clamps_both_values() {
        let mut e = controls_engine(1.0);
        let this = control_set(&mut e);
        let base = this.addr();
        for first in 0..2u32 {
            e.mem.set_f32(base + 0x324 + first * 0x50 + 0x24, 0.5);
        }
        let covariance = base + 0xfcc;
        for (i, v) in [1.0f32, 0.0, 0.0, 1.0].iter().enumerate() {
            e.mem.set_f32(covariance + 4 * i as u32, *v);
        }
        let values = e.mem.alloc(8);
        e.mem.set_f32(values, 100.0);
        e.mem.set_f32(values + 4, -10.0);
        let table = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        e.call(0x00aa_c170, &args![this, 0i32, 0i32, values, table]);
        let log = e.call_log.take().unwrap();
        assert_eq!(floats_of(&log, MATRIX_TIMES_FLOAT, 2), vec![63.5, -5.5]);
        let sources: Vec<u32> = calls_to(&log, MATRIX_TRANSPOSE)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(sources, vec![base + 0x324, base + 0x324 + 0x50]);
        assert_eq!(calls_to(&log, MATRIX_ADD_ASSIGN).len(), 2);
        e.call_log = Some(vec![]);
        e.call(0x00aa_c170, &args![this, 7i32, 0i32, values, table]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FR2_ASSERT)[0],
            vec![FAN_CONTROLS_SOURCE, 0x10a]
        );
    }

    #[test]
    fn control_value_from_a_table() {
        let mut e = controls_engine(0.5);
        let this = control_set(&mut e);
        let table = e.mem.alloc(0x80);
        // [1][1]: sqrt(16) / sqrt(4 rows) = 2.
        e.mem.set_u32(table + 0x40 + 0x20 + MATRIX_ROWS, 4);
        e.register(MATRIX_NORM, |_, _| st0(16.0));
        assert_eq!(
            e.call(0x00aa_c3f0, &args![this, 0i32, 1i32, 1i32, table])
                .f32(),
            2.0
        );
        // [0][0]: the products report 6 rows, so sqrt(16) / sqrt(6 - 2) = 2.
        e.call_log = Some(vec![]);
        let value = e
            .call(0x00aa_c3f0, &args![this, 1i32, 0i32, 0i32, table])
            .f32();
        let log = e.call_log.take().unwrap();
        assert_eq!(value, 2.0);
        assert_eq!(calls_to(&log, MATRIX_PRODUCT).len(), 3);
        assert_eq!(floats_of(&log, MATRIX_TIMES_FLOAT, 2), vec![0.5, 0.5]);
        let bases: Vec<u32> = calls_to(&log, MATRIX_TRANSPOSE)
            .iter()
            .map(|w| w[0])
            .collect();
        let base = this.addr() + 0xd4c + 0x80;
        assert_eq!(bases, vec![base, base + 0x20]);
        assert_eq!(calls_to(&log, MATRIX_DESTRUCT).len(), 3);
        // Any other column asserts twice and gives 0.
        e.call_log = Some(vec![]);
        let value = e
            .call(0x00aa_c3f0, &args![this, 0i32, 0i32, 2i32, table])
            .f32();
        let log = e.call_log.take().unwrap();
        assert_eq!(value, 0.0);
        let lines: Vec<u32> = calls_to(&log, FR2_ASSERT).iter().map(|w| w[1]).collect();
        assert_eq!(lines, vec![0x136, 0x158]);
        e.mem.set_u8(this.addr(), 0);
        assert_eq!(
            e.call(0x00aa_c3f0, &args![this, 0i32, 1i32, 1i32, table])
                .f32(),
            0.0
        );
    }

    #[test]
    fn set_value_scales_the_table() {
        let mut e = controls_engine(0.5);
        let this = control_set(&mut e);
        let table = e.mem.alloc(0x80);
        e.mem.set_u32(table + 0x20 + MATRIX_ROWS, 4);
        e.register(MATRIX_NORM, |_, _| st0(16.0));
        // Column 1: the current value is 2, so the matrix is scaled by 6 / 2.
        let element = scale_probe(&mut e);
        e.call(0x00aa_c820, &args![this, 0i32, 0i32, 1i32, 6.0f32, table]);
        assert_eq!(e.mem.f32(element), 3.0);
        // A value below 0.01 becomes 0.01.
        let element = scale_probe(&mut e);
        e.call(0x00aa_c820, &args![this, 0i32, 0i32, 1i32, 0.0f32, table]);
        let expected = x87_store32(x87_div(f64::from(0.01f32), 2.0));
        assert_eq!(e.mem.u32(element), expected);
        // Column 0: the projection is built and the table assigned d + p + mean.
        let element = scale_probe(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x00aa_c820, &args![this, 0i32, 1i32, 0i32, 6.0f32, table]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.f32(element), 3.0);
        assert_eq!(calls_to(&log, MATRIX_CLEAR).len(), 1);
        assert_eq!(calls_to(&log, MATRIX_ADD_ASSIGN).len(), 2);
        let sums = calls_to(&log, MATRIX_SUM);
        assert_eq!(sums.len(), 2);
        assert_eq!(sums[1][2], this.addr() + 4 + 0x20);
        assert_eq!(
            calls_to(&log, MATRIX_ASSIGN),
            vec![vec![table + 0x40, sums[1][1]]]
        );
        // A current value of 0 leaves the table.
        e.register(MATRIX_NORM, |_, _| st0(0.0));
        e.call_log = Some(vec![]);
        e.call(0x00aa_c820, &args![this, 0i32, 1i32, 0i32, 6.0f32, table]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MATRIX_SUM).is_empty());
        assert!(calls_to(&log, MATRIX_ASSIGN).is_empty());
        let lines: Vec<u32> = calls_to(&log, FR2_ASSERT).iter().map(|w| w[1]).collect();
        assert!(lines.is_empty());
    }

    #[test]
    fn race_offset_value_and_step() {
        let mut e = controls_engine(1.5);
        let this = control_set(&mut e);
        let control = this.addr() + 0x644 + 0x168 + 2 * 0x48;
        e.mem.set_f32(control + 0x44, 0.25);
        e.mem.set_f32(control + 0x40, 0.5);
        let table = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        let value = e.call(0x00aa_cdb0, &args![this, 1i32, 2i32, table]).f32();
        let log = e.call_log.take().unwrap();
        assert_eq!(value, 3.25);
        let products = calls_to(&log, MATRIX_PRODUCT);
        assert_eq!(products[1][0], control + 0x20);
        assert_eq!(products[1][2], table + 0x40);
        // The step (5 - 3.25) * 0.5 times each transpose is added to table[k].
        e.call_log = Some(vec![]);
        e.call(0x00aa_cf40, &args![this, 1i32, 2i32, 5.0f32, table]);
        let log = e.call_log.take().unwrap();
        assert_eq!(floats_of(&log, MATRIX_TIMES_FLOAT, 2), vec![0.875, 0.875]);
        let targets: Vec<u32> = calls_to(&log, MATRIX_ADD_ASSIGN)
            .iter()
            .map(|w| w[0])
            .collect();
        assert_eq!(targets, vec![table, table + 0x40]);
        // Equal groups assert.
        e.call_log = Some(vec![]);
        e.call(0x00aa_cf40, &args![this, 2i32, 2i32, 5.0f32, table]);
        let log = e.call_log.take().unwrap();
        let lines: Vec<u32> = calls_to(&log, FR2_ASSERT).iter().map(|w| w[1]).collect();
        assert_eq!(lines, vec![0x1d6, 0x1b6]);
        e.mem.set_u8(this.addr(), 0);
        assert_eq!(
            e.call(0x00aa_cdb0, &args![this, 1i32, 2i32, table]).f32(),
            0.0
        );
    }

    /// Row pointers for the float vectors (`0060b370`): row `index` of a matrix starts
    /// `index * columns` floats in.
    fn row_pointers(e: &mut Engine) {
        e.register(FLOAT_VECTOR_POINTER, |e, a| {
            let block = e.mem.u32(a[0] + 0xc);
            let columns = e.mem.u32(a[0] + MATRIX_COLUMNS);
            e.mem.set_u32(a[1], block + 4 * a[2] * columns);
            a[1].into_ret()
        });
        e.register(MATRIX_FLOAT_AT, |e, a| {
            (e.mem.u32(a[0] + 0xc) + 4 * a[1]).into_ret()
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        // The copy constructor keeps the source's counts and floats (block pointer).
        e.register(MATRIX_COPY, |e, a| {
            for offset in [0xc, MATRIX_ROWS, MATRIX_COLUMNS] {
                let word = e.mem.u32(a[1] + offset);
                e.mem.set_u32(a[0] + offset, word);
            }
            Ret::default()
        });
    }

    /// A matrix of `rows x columns` with the given floats.
    fn matrix(e: &mut Engine, at: u32, rows: u32, columns: u32, floats: &[f32]) {
        let block = e.mem.alloc((4 * floats.len() as u32).max(4));
        for (i, v) in floats.iter().enumerate() {
            e.mem.set_f32(block + 4 * i as u32, *v);
        }
        e.mem.set_u32(at + 0xc, block);
        e.mem.set_u32(at + MATRIX_ROWS, rows);
        e.mem.set_u32(at + MATRIX_COLUMNS, columns);
    }

    fn floats(e: &Engine, matrix: u32, count: u32) -> Vec<f32> {
        let block = e.mem.u32(matrix + 0xc);
        (0..count).map(|i| e.mem.f32(block + 4 * i)).collect()
    }

    #[test]
    fn block_and_stack_copy_floats() {
        let mut e = controls_engine(0.0);
        row_pointers(&mut e);
        let source = e.mem.alloc(0x20);
        let values: Vec<f32> = (0..9).map(|i| i as f32).collect();
        matrix(&mut e, source, 3, 3, &values);
        let result = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x00aa_f110, &args![source, result, 1u32, 1u32, 2u32, 2u32])
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, result);
        assert_eq!(floats(&e, result, 4), vec![4.0, 5.0, 7.0, 8.0]);
        assert!(calls_to(&log, FR2_ASSERT).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x00aa_f110, &args![source, result, 0u32, 2u32, 1u32, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FR2_ASSERT), vec![vec![MATRIX_SOURCE, 0x427]]);
        // Stacking: a 1x2 on a 2x2.
        let top = e.mem.alloc(0x20);
        let bottom = e.mem.alloc(0x20);
        matrix(&mut e, top, 1, 2, &[1.0, 2.0]);
        matrix(&mut e, bottom, 2, 2, &[3.0, 4.0, 5.0, 6.0]);
        assert_eq!(
            e.call(0x00aa_f330, &args![result, top, bottom]).u32(),
            result
        );
        assert_eq!(e.mem.u32(result + MATRIX_ROWS), 3);
        assert_eq!(floats(&e, result, 6), vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        // An empty one gives a copy of the other; two give an empty matrix.
        let empty = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x00aa_f330, &args![result, empty, bottom]);
        e.call(0x00aa_f330, &args![result, top, empty]);
        e.call(0x00aa_f330, &args![result, empty, empty]);
        let log = e.call_log.take().unwrap();
        let copies: Vec<u32> = calls_to(&log, MATRIX_COPY).iter().map(|w| w[1]).collect();
        assert_eq!(copies, vec![bottom, top]);
        assert_eq!(calls_to(&log, VECTOR_CONSTRUCT), vec![vec![result]]);
        assert_eq!(calls_to(&log, VECTOR_RESIZE_WITH), vec![vec![result, 0, 0]]);
        // Different column counts assert.
        matrix(&mut e, bottom, 2, 1, &[3.0, 4.0]);
        e.call_log = Some(vec![]);
        e.call(0x00aa_f330, &args![result, top, bottom]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FR2_ASSERT), vec![vec![MATRIX_SOURCE, 0x4fe]]);
    }

    #[test]
    fn two_by_two_times_vector() {
        let mut e = engine();
        let m = e.mem.alloc(0x10);
        let v = e.mem.alloc(8);
        let out = e.mem.alloc(8);
        for (i, x) in [1.0f32, 2.0, 3.0, 4.0].iter().enumerate() {
            e.mem.set_f32(m + 4 * i as u32, *x);
        }
        e.mem.set_f32(v, 5.0);
        e.mem.set_f32(v + 4, 6.0);
        assert_eq!(e.call(0x00aa_f2b0, &args![out, m, v]).u32(), out);
        assert_eq!(e.mem.f32(out), 17.0);
        assert_eq!(e.mem.f32(out + 4), 39.0);
    }

    #[test]
    fn random_group_values_fill_and_split() {
        let mut e = controls_engine(0.0);
        row_pointers(&mut e);
        e.register(RANDOM_NORMAL, |_, _| st0(0.5));
        let this = control_set(&mut e);
        let table = e.mem.alloc(0x80);
        matrix(&mut e, table, 2, 1, &[0.0, 0.0]);
        matrix(&mut e, table + 0x20, 1, 1, &[0.0]);
        matrix(&mut e, table + 0x40, 3, 1, &[0.0, 0.0, 0.0]);
        matrix(&mut e, table + 0x60, 0, 1, &[]);
        e.call_log = Some(vec![]);
        e.call(0x00aa_d140, &args![this, 1i32, table]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, RANDOM_NORMAL).len(), 6);
        assert_eq!(floats(&e, table + 0x20, 1), vec![0.5]);
        let group = this.addr() + 0xa0;
        assert_eq!(calls_to(&log, MATRIX_PRODUCT)[0][0], group + 0x44);
        let means: Vec<u32> = calls_to(&log, MATRIX_SUM).iter().map(|w| w[2]).collect();
        assert_eq!(means, vec![group + 4, group + 0x24]);
        let targets: Vec<u32> = calls_to(&log, MATRIX_ASSIGN).iter().map(|w| w[0]).collect();
        assert_eq!(targets[1..], [table, table + 0x40]);
        e.call_log = Some(vec![]);
        e.call(0x00aa_d140, &args![this, 5i32, table]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FR2_ASSERT)[0],
            vec![FAN_CONTROLS_SOURCE, 0x1f2]
        );
    }

    #[test]
    fn random_offsets_from_a_source_table() {
        let mut e = controls_engine(0.0);
        row_pointers(&mut e);
        e.register(RANDOM_NORMAL, |_, _| st0(0.5));
        let source = e.mem.alloc(0x80);
        for k in 0..4u32 {
            matrix(&mut e, source + 0x20 * k, 1, 2, &[1.0, 2.0]);
        }
        let destination = e.mem.alloc(0x80);
        let this = e.mem.alloc(4);
        // 0.5 * 2 * 1.5 = 1.5 is added to each element.
        e.call(0x00aa_d500, &args![this, 2.0f32, source, destination]);
        for k in 0..4u32 {
            assert_eq!(floats(&e, destination + 0x20 * k, 2), vec![2.5, 3.5]);
            assert_eq!(e.mem.u32(destination + 0x20 * k + MATRIX_COLUMNS), 2);
        }
        e.call(0x00aa_d6b0, &args![this, 2.0f32, source, destination]);
        for k in 0..2u32 {
            assert_eq!(floats(&e, destination + 0x40 * k, 2), vec![2.5, 3.5]);
            assert_eq!(floats(&e, destination + 0x40 * k + 0x20, 2), vec![0.0, 0.0]);
        }
    }

    #[test]
    fn reflect_negates_and_mirrors() {
        let mut e = controls_engine(0.0);
        let this = control_set(&mut e);
        let table = e.mem.alloc(0x80);
        e.call_log = Some(vec![]);
        e.call(0x00aa_d890, &args![this, 1i32, table]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            floats_of(&log, MATRIX_TIMES_FLOAT, 2),
            vec![-1.0, 2.0, -1.0, 2.0]
        );
        let sources: Vec<u32> = calls_to(&log, MATRIX_TIMES_FLOAT)
            .iter()
            .map(|w| w[0])
            .collect();
        let group = this.addr() + 0xa0;
        assert_eq!(
            sources,
            vec![table + 0x20, group + 4, table + 0x60, group + 0x24]
        );
        let targets: Vec<u32> = calls_to(&log, MATRIX_ASSIGN).iter().map(|w| w[0]).collect();
        assert_eq!(
            targets,
            vec![table + 0x20, table, table + 0x60, table + 0x40]
        );
        e.call_log = Some(vec![]);
        e.call(0x00aa_d890, &args![this, 6i32, table]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FR2_ASSERT),
            vec![vec![FAN_CONTROLS_SOURCE, 0x275]]
        );
    }

    #[test]
    fn tween_blends_values_and_matrices() {
        let mut e = controls_engine(0.0);
        let this = control_set(&mut e);
        let a = e.mem.alloc(0x80);
        let b = e.mem.alloc(0x80);
        let out = e.mem.alloc(0x80);
        for table in [a, b, out] {
            e.mem.set_u32(table + 0x20 + MATRIX_ROWS, 4);
        }
        // The values: a 2, b 4, out 2 (sqrt of the norm over sqrt(4)).
        e.register_double(MATRIX_NORM, move |_, w| {
            st0(if w[0] == b + 0x20 { 64.0 } else { 16.0 })
        });
        for (t, scale) in [(0.25f32, 1.75f32), (-1.0, 2.0), (2.0, 1.0)] {
            let element = scale_probe(&mut e);
            e.call_log = Some(vec![]);
            e.call(0x00aa_dac0, &args![this, 0i32, 1i32, t, a, b, out]);
            let log = e.call_log.take().unwrap();
            assert_eq!(e.mem.f32(element), scale, "{t}");
            let scales = floats_of(&log, MATRIX_TIMES_FLOAT, 2);
            assert_eq!(scales, vec![1.0 - t, t]);
            assert_eq!(calls_to(&log, MATRIX_ASSIGN)[0][0], out + 0x20);
        }
    }

    #[test]
    fn erase_moves_down_and_destroys_the_tail() {
        let mut e = control_engine();
        e.register(CONTROL_ITERATORS_DIFFER, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        quiet(&mut e, &[INVALID_PARAMETER]);
        let vector = control_vector(&mut e, &[1, 2, 3], 3);
        e.mem.set_u32(vector.addr(), 0x77);
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        let result = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        let back = e
            .call(
                0x00aa_e200,
                &args![
                    vector,
                    result,
                    0u32,
                    first + CONTROL_SIZE,
                    0u32,
                    first + 2 * CONTROL_SIZE
                ],
            )
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, result);
        assert_eq!(e.mem.u32(result), 0x77);
        assert_eq!(e.mem.u32(result + 4), first + CONTROL_SIZE);
        assert_eq!(
            calls_to(&log, MATRIX_ASSIGN),
            vec![vec![first + CONTROL_SIZE, first + 2 * CONTROL_SIZE]]
        );
        assert_eq!(
            calls_to(&log, CONTROL_DESTRUCT),
            vec![vec![first + 2 * CONTROL_SIZE]]
        );
        assert_eq!(
            e.mem.u32(vector.addr() + VECTOR_LAST),
            first + 2 * CONTROL_SIZE
        );
        assert!(calls_to(&log, INVALID_PARAMETER).is_empty());
        // An empty range changes nothing; a pointer outside the vector fails a check.
        e.call_log = Some(vec![]);
        e.call(
            0x00aa_e200,
            &args![vector, result, 0u32, first, 0u32, first],
        );
        e.call(
            0x00aa_e200,
            &args![vector, result, 0u32, first + 0x1000, 0u32, first + 0x1000],
        );
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, CONTROL_DESTRUCT).is_empty());
        assert_eq!(calls_to(&log, INVALID_PARAMETER).len(), 2);
    }

    #[test]
    fn iterator_advance_checks_the_range() {
        let mut e = control_engine();
        quiet(&mut e, &[INVALID_PARAMETER]);
        let vector = control_vector(&mut e, &[1, 2], 2);
        let proxy = e.mem.alloc(4);
        e.mem.set_u32(proxy, vector.addr());
        let first = e.mem.u32(vector.addr() + VECTOR_FIRST);
        let iterator = e.mem.alloc(8);
        e.mem.set_u32(iterator, proxy);
        e.mem.set_u32(iterator + 4, first);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_ecb0, &args![iterator, 2u32]).u32(), iterator);
        assert_eq!(e.mem.u32(iterator + 4), first + 2 * CONTROL_SIZE);
        e.call(0x00aa_ecb0, &args![iterator, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(iterator + 4), first + 3 * CONTROL_SIZE);
        assert_eq!(calls_to(&log, INVALID_PARAMETER).len(), 1);
    }

    #[test]
    fn subtract_in_place_walks_both_matrices() {
        let mut e = controls_engine(0.0);
        e.register(MAKE_ITERATOR, |e, a| {
            e.mem.set_u32(a[0], a[2]);
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(MATRIX_CONST_BEGIN, |e, a| {
            let first = e.mem.u32(a[0] + VECTOR_FIRST);
            e.mem.set_u32(a[1], a[0]);
            e.mem.set_u32(a[1] + 4, first);
            a[1].into_ret()
        });
        e.register(ITERATORS_DIFFER, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != e.mem.u32(a[1] + 4)).into_ret()
        });
        e.register(DEREFERENCE_CHECKED, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(PRE_INCREMENT, |e, a| {
            let pointer = e.mem.u32(a[0] + 4);
            e.mem.set_u32(a[0] + 4, pointer + 4);
            Ret::default()
        });
        let a = e.mem.alloc(0x20);
        let b = e.mem.alloc(0x20);
        matrix(&mut e, a, 1, 2, &[5.0, 7.0]);
        matrix(&mut e, b, 1, 2, &[1.5, 2.0]);
        for m in [a, b] {
            let block = e.mem.u32(m + 0xc);
            e.mem.set_u32(m + VECTOR_FIRST, block);
            e.mem.set_u32(m + VECTOR_LAST, block + 8);
        }
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00aa_ef90, &args![a, b]).u32(), a);
        let log = e.call_log.take().unwrap();
        assert_eq!(floats(&e, a, 2), vec![3.5, 5.0]);
        assert!(calls_to(&log, FR2_ASSERT).is_empty());
        e.mem.set_u32(b + MATRIX_ROWS, 2);
        e.call_log = Some(vec![]);
        e.call(0x00aa_ef90, &args![a, b]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FR2_ASSERT), vec![vec![MATRIX_SOURCE, 0x29e]]);
    }

    #[test]
    fn control_range_assignments() {
        let mut e = control_engine();
        let block = e.mem.alloc(4 * CONTROL_SIZE);
        let value = e.mem.alloc(CONTROL_SIZE);
        e.call_log = Some(vec![]);
        // `engine` doubles 00aaf6f0 and 00aaf860 for the older tests: these call the functions.
        fn_00aaf6f0(&mut e, block, block + 2 * CONTROL_SIZE, value);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MATRIX_ASSIGN),
            vec![vec![block, value], vec![block + CONTROL_SIZE, value]]
        );
        assert_eq!(
            calls_to(&log, STRING_ASSIGN)[1],
            vec![block + CONTROL_SIZE + 0x20, value + 0x20, 0, u32::MAX]
        );
        e.call_log = Some(vec![]);
        let end = e
            .call(
                0x00aa_f750,
                &args![block, block + 2 * CONTROL_SIZE, value, 0u32, 0u32, 0u32],
            )
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(end, value + 2 * CONTROL_SIZE);
        assert_eq!(
            calls_to(&log, MATRIX_ASSIGN),
            vec![
                vec![value, block],
                vec![value + CONTROL_SIZE, block + CONTROL_SIZE]
            ]
        );
        e.register(CONTROL_COPY_BACKWARD, |_, _| 0x1234u32.into_ret());
        e.call_log = Some(vec![]);
        let back = fn_00aaf860(&mut e, 1, 2, 3, 0x1ff, 9, 9);
        let log = e.call_log.take().unwrap();
        assert_eq!(back, 0x1234);
        assert_eq!(
            calls_to(&log, CONTROL_COPY_BACKWARD),
            vec![vec![1, 2, 3, 0xff, 0, 0]]
        );
    }
}
