//! `(unplaced)/run_006586c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00658930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the 16-bit field at `+0xa` of `this` (zero-extended into `EAX`).
pub fn fn_00658930(e: &mut Engine, this: Ptr) -> u16 {
    e.mem.u16(this.addr() + 0xa)
}

/// The matrix's row count (+0x18) and column count (+0x1C).
const ROWS: u32 = 0x18;
const COLUMNS: u32 = 0x1c;
/// The row access `0060b370` (`thiscall (matrix, &result, row)`, `RET 8`:
/// the row's first element and the column count into the 8-byte result,
/// answered) and the address of a column in it (`00629ab0`, `thiscall (row,
/// column)`, `RET 4`).
const ELEMENT_AT: u32 = 0x0060_b370;
const ELEMENT_ADDRESS: u32 = 0x0062_9ab0;
/// The per-element value `006591f0` (`cdecl`, one float, result in ST0).
const ELEMENT_VALUE: u32 = 0x0065_91f0;
/// The matrix constructor (`00659000`, `thiscall (matrix, rows, columns)`),
/// copy into the result (`00658f30`, `thiscall (result, &matrix)`) and
/// destructor (`00449680`, `thiscall`).
const MATRIX_CONSTRUCT: u32 = 0x0065_9000;
const MATRIX_COPY: u32 = 0x0065_8f30;
const MATRIX_DESTRUCT: u32 = 0x0044_9680;

/// The address of element (`row`, `column`) of the matrix at `matrix`.
fn element(e: &mut Engine, matrix: u32, row: u32, column: u32) -> u32 {
    e.with_stack(8, |e, result| {
        let at = e.call(ELEMENT_AT, &args![matrix, result, row]).u32();
        e.call(ELEMENT_ADDRESS, &args![at, column]).u32()
    })
}

// Translated from 006588a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the sum over every element (rows, then
/// columns) of `006591f0` of its value, kept as a float (each addition
/// `value + sum`).
pub fn fn_006588a0(e: &mut Engine, this: Ptr) -> f64 {
    let mut sum = x87_store32(0.0);
    let mut row = 0u32;
    while row < e.mem.u32(this.addr().wrapping_add(ROWS)) {
        let mut column = 0u32;
        while column < e.mem.u32(this.addr().wrapping_add(COLUMNS)) {
            let at = element(e, this.addr(), row, column);
            let value = x87_quiet_bits(e.mem.u32(at));
            let mapped = e.call(ELEMENT_VALUE, &args![value]).f64();
            sum = x87_store32(x87_add(mapped, x87_m32(sum)));
            column += 1;
        }
        row += 1;
    }
    x87_load32(sum)
}

// Translated from 006587a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 006587a0 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed in the engine map: the transpose: a local matrix of `columns x
/// rows` (`00659000`) gets element (`column`, `row`) from each (`row`,
/// `column`), is copied into `result` (`00658f30`) and destroyed
/// (`00449680`). Answers `result`. The exception-unwinding frame is not
/// translated.
pub fn fn_006587a0(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    e.with_stack(0x20, |e, local| {
        let rows = e.mem.u32(this.addr().wrapping_add(ROWS));
        let columns = e.mem.u32(this.addr().wrapping_add(COLUMNS));
        e.call(MATRIX_CONSTRUCT, &args![local, columns, rows]);
        let mut row = 0u32;
        while row < e.mem.u32(this.addr().wrapping_add(ROWS)) {
            let mut column = 0u32;
            while column < e.mem.u32(this.addr().wrapping_add(COLUMNS)) {
                let from = element(e, this.addr(), row, column);
                let to = element(e, local.addr(), column, row);
                let value = x87_quiet_bits(e.mem.u32(from));
                e.mem.set_u32(to, value);
                column += 1;
            }
            row += 1;
        }
        e.call(MATRIX_COPY, &args![result, local]);
        e.call(MATRIX_DESTRUCT, &args![local]);
    });
    result
}

/// `fr2Assert` (`00aafb90`, engine map; cdecl file, line), the assertion's file name
/// (`01065830`, `matrixVT.hpp`) and its line.
const FR2_ASSERT: u32 = 0x00aa_fb90;
const ASSERT_FILE: u32 = 0x0106_5830;
const ASSERT_LINE: u32 = 0x24f;
/// The element iterators (8 bytes each): this matrix's begin and end (`0044a170` /
/// `0065f550`; `thiscall`: out iterator, answered), the other's (`0065fd10` /
/// `0065fd40`), the inequality (`0065dfd0`; `thiscall`: other iterator), the element
/// addresses (`0064dbd0` on this one's, `0064dda0` on the other's) and the post-increments
/// (`006590c0` / `00659100`; `thiscall`: out iterator, 0).
const BEGIN: u32 = 0x0044_a170;
const END: u32 = 0x0065_f550;
const OTHER_BEGIN: u32 = 0x0065_fd10;
const OTHER_END: u32 = 0x0065_fd40;
const ITERATOR_NOT_EQUAL: u32 = 0x0065_dfd0;
const ITERATOR_ELEMENT: u32 = 0x0064_dbd0;
const OTHER_ITERATOR_ELEMENT: u32 = 0x0064_dda0;
const ITERATOR_POST_INCREMENT: u32 = 0x0065_90c0;
const OTHER_ITERATOR_POST_INCREMENT: u32 = 0x0065_9100;

// Translated from 006586c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the element-wise `+=` of `other`.
/// When the row or column counts differ, `fr2Assert(matrixVT.hpp, 0x24F)` runs first (and
/// the addition goes on). Then both element iterators walk together until either reaches
/// its end, each element of `this` becoming `other + this` (x87, stored as a `float`).
/// Answers `this`.
pub fn fn_006586c0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let mine = this.addr();
    let theirs = other.addr();
    if e.mem.u32(mine.wrapping_add(ROWS)) != e.mem.u32(theirs.wrapping_add(ROWS))
        || e.mem.u32(mine.wrapping_add(COLUMNS)) != e.mem.u32(theirs.wrapping_add(COLUMNS))
    {
        e.call(FR2_ASSERT, &args![ASSERT_FILE, ASSERT_LINE]);
    }
    // The frame from EBP - 0x38; `at(k)` is EBP - k.
    e.with_stack(0x38, |e, frame| {
        let at = |k: u32| frame.addr() + 0x38 - k;
        let (walk, other_walk) = (at(0x10), at(0x8));
        e.call(BEGIN, &args![mine, walk]);
        e.call(OTHER_BEGIN, &args![theirs, other_walk]);
        loop {
            let end = e.call(END, &args![mine, at(0x28)]).u32();
            if e.call(ITERATOR_NOT_EQUAL, &args![walk, end]).u8() == 0 {
                break;
            }
            let other_end = e.call(OTHER_END, &args![theirs, at(0x30)]).u32();
            if e.call(ITERATOR_NOT_EQUAL, &args![other_walk, other_end])
                .u8()
                == 0
            {
                break;
            }
            let target = e.call(ITERATOR_ELEMENT, &args![walk]).u32();
            let source = e.call(OTHER_ITERATOR_ELEMENT, &args![other_walk]).u32();
            let sum = x87_add(x87_load32(e.mem.u32(source)), x87_m32(e.mem.u32(target)));
            e.mem.set_u32(target, x87_store32(sum));
            e.call(ITERATOR_POST_INCREMENT, &args![walk, at(0x18), 0u32]);
            e.call(
                OTHER_ITERATOR_POST_INCREMENT,
                &args![other_walk, at(0x20), 0u32],
            );
        }
    });
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00658930, fn_00658930(Ptr) -> u16),
        entry!(0x006588a0, fn_006588a0(Ptr) -> f64),
        entry!(0x006587a0, fn_006587a0(Ptr, Ptr) -> Ptr),
        entry!(0x0065_86c0, fn_006586c0(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_field_at_offset_0xa() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x50));
        e.mem.set_u16(object.addr() + 0xa, 0x1234);
        assert_eq!(e.call(0x0065_8930, &args![object]).u16(), 0x1234);
    }
}

#[cfg(test)]
mod b014_tests {
    use super::*;

    /// A 2 x 3 matrix whose element (r, c) is 10 r + c; the row access
    /// answers (row start, columns).
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(ELEMENT_AT, |e, a| {
            let data = e.mem.u32(a[0] + 0x10);
            let columns = e.mem.u32(a[0] + COLUMNS);
            e.mem.set_u32(a[1], data + 4 * a[2] * columns);
            e.mem.set_u32(a[1] + 4, columns);
            a[1].into_ret()
        });
        e.register(ELEMENT_ADDRESS, |e, a| {
            (e.mem.u32(a[0]) + 4 * a[1]).into_ret()
        });
        let this = e.mem.alloc(0x24);
        let data = e.mem.alloc(4 * 6);
        e.mem.set_u32(this + ROWS, 2);
        e.mem.set_u32(this + COLUMNS, 3);
        e.mem.set_u32(this + 0x10, data);
        for r in 0..2u32 {
            for c in 0..3u32 {
                e.mem.set_f32(data + 4 * (3 * r + c), (10 * r + c) as f32);
            }
        }
        (e, this)
    }

    #[test]
    fn sums_the_mapped_elements() {
        let (mut e, this) = engine();
        e.register(ELEMENT_VALUE, |_, a| Ret {
            st0: f64::from(f32::from_bits(a[0])) * 2.0,
            ..Ret::default()
        });
        // 2 * (0 + 1 + 2 + 10 + 11 + 12) = 72.
        assert_eq!(e.call(0x006588a0, &args![this]).f64(), 72.0);
    }

    #[test]
    fn transposes_into_the_result() {
        let (mut e, this) = engine();
        e.register(MATRIX_CONSTRUCT, |e, a| {
            let data = e.mem.alloc(4 * a[1] * a[2]);
            e.mem.set_u32(a[0] + ROWS, a[1]);
            e.mem.set_u32(a[0] + COLUMNS, a[2]);
            e.mem.set_u32(a[0] + 0x10, data);
            a[0].into_ret()
        });
        e.map(0x7300_0000, 0x40);
        e.register(MATRIX_COPY, |e, a| {
            let data = e.mem.u32(a[1] + 0x10);
            for i in 0..6 {
                let v = e.mem.u32(data + 4 * i);
                e.mem.set_u32(0x7300_0000 + 4 * i, v);
            }
            Ret::default()
        });
        e.register(MATRIX_DESTRUCT, |_, _| Ret::default());
        let result = e.mem.alloc(0x24);
        assert_eq!(e.call(0x006587a0, &args![this, result]).u32(), result);
        let got: Vec<f32> = (0..6).map(|i| e.mem.f32(0x7300_0000 + 4 * i)).collect();
        assert_eq!(got, [0.0, 10.0, 1.0, 11.0, 2.0, 12.0]);
    }
}

#[cfg(test)]
mod n076_tests {
    use super::*;

    /// A matrix of `rows x columns` floats at +0x10; iterators are a pointer word.
    fn matrix(e: &mut Engine, rows: u32, columns: u32, values: &[f32]) -> u32 {
        let this = e.mem.alloc(0x24);
        let data = e.mem.alloc(4 * values.len() as u32 + 4);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(data + 4 * i as u32, *v);
        }
        e.mem.set_u32(this + 0x10, data);
        e.mem.set_u32(this + ROWS, rows);
        e.mem.set_u32(this + COLUMNS, columns);
        this
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for address in [BEGIN, OTHER_BEGIN] {
            e.register(address, |e, a| {
                let data = e.mem.u32(a[0] + 0x10);
                e.mem.set_u32(a[1], data);
                a[1].into_ret()
            });
        }
        for address in [END, OTHER_END] {
            e.register(address, |e, a| {
                let count = e.mem.u32(a[0] + ROWS) * e.mem.u32(a[0] + COLUMNS);
                let data = e.mem.u32(a[0] + 0x10);
                e.mem.set_u32(a[1], data + 4 * count);
                a[1].into_ret()
            });
        }
        e.register(ITERATOR_NOT_EQUAL, |e, a| {
            (e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        for address in [ITERATOR_ELEMENT, OTHER_ITERATOR_ELEMENT] {
            e.register(address, |e, a| e.mem.u32(a[0]).into_ret());
        }
        for address in [ITERATOR_POST_INCREMENT, OTHER_ITERATOR_POST_INCREMENT] {
            e.register(address, |e, a| {
                let at = e.mem.u32(a[0]);
                e.mem.set_u32(a[0], at + 4);
                a[1].into_ret()
            });
        }
        e.register(FR2_ASSERT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn asserts(e: &Engine) -> usize {
        let log = e.call_log.as_ref().unwrap();
        log.iter().filter(|c| c.0 == FR2_ASSERT).count()
    }

    #[test]
    fn adds_the_other_element_by_element() {
        let mut e = engine();
        let this = matrix(&mut e, 1, 2, &[1.0, 2.0]);
        let other = matrix(&mut e, 1, 2, &[0.5, 4.0]);
        assert_eq!(e.call(0x0065_86c0, &args![this, other]).u32(), this);
        let data = e.mem.u32(this + 0x10);
        assert_eq!(e.mem.f32(data), 1.5);
        assert_eq!(e.mem.f32(data + 4), 6.0);
        assert_eq!(asserts(&e), 0);
    }

    #[test]
    fn mismatched_sizes_assert_and_stop_at_the_shorter() {
        let mut e = engine();
        let this = matrix(&mut e, 1, 3, &[1.0, 2.0, 3.0]);
        let other = matrix(&mut e, 1, 1, &[1.0]);
        e.call(0x0065_86c0, &args![this, other]);
        let data = e.mem.u32(this + 0x10);
        assert_eq!(e.mem.f32(data), 2.0);
        assert_eq!(e.mem.f32(data + 4), 2.0);
        assert_eq!(asserts(&e), 1);
        let log = e.call_log.as_ref().unwrap();
        let assert_call = log.iter().find(|c| c.0 == FR2_ASSERT).unwrap();
        assert_eq!(assert_call.1, vec![ASSERT_FILE, ASSERT_LINE]);
    }
}
