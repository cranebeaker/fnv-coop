//! `BSHavok/run_00cafef0` (Xbox PDB source unit), subsystem `BSHavok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00cafef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_00cafef0` (unnamed in the Xbox PDB): no argument besides `this`. Does nothing when the
/// pointer at `+0x30` is 0. Otherwise walks the 12-byte elements that pointer leads to (`+0x10`
/// rows of `+0x14` elements, both signed counts; the walk reads nothing when a count is not
/// positive), reading the float at `+8` of each. A minimum and a maximum both start as the
/// float of the first element. A float replaces the minimum unless it is greater than it, and
/// replaces the maximum unless the maximum is greater than it, so a NaN replaces both. Then
/// the float at `+0x34` plus the minimum is stored at `+0x18` and plus the maximum at `+0x1c`.
/// The loads and stores go through the x87 stack, and the sums are `f64` rounded to `f32`.
pub fn fn_00cafef0(e: &mut Engine, this: u32) {
    let mut cursor = e.mem.u32(this + 0x30);
    if cursor == 0 {
        return;
    }
    let first = x87_quiet(e.mem.f32(cursor + 8));
    let mut minimum = first;
    let mut maximum = first;
    let rows = e.mem.i32(this + 0x10);
    if rows > 0 {
        let columns = e.mem.i32(this + 0x14);
        for _ in 0..rows {
            for _ in 0..columns.max(0) {
                let value = x87_quiet(e.mem.f32(cursor + 8));
                if !(value > minimum) {
                    minimum = value;
                }
                if !(maximum > value) {
                    maximum = value;
                }
                cursor = cursor.wrapping_add(12);
            }
        }
    }
    let base = f64::from(e.mem.f32(this + 0x34));
    e.mem
        .set_f32(this + 0x1c, (base + f64::from(maximum)) as f32);
    e.mem
        .set_f32(this + 0x18, (base + f64::from(minimum)) as f32);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00cafef0, fn_00cafef0(u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An object with `rows` x `columns` of 12-byte elements whose float at `+8` is taken
    /// from `values`, and the base float 100.
    fn setup(rows: i32, columns: i32, values: &[f32]) -> (Engine, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x40);
        let data = e.mem.alloc(12 * values.len() as u32 + 12);
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(data + 12 * i as u32 + 8, *v);
        }
        e.mem.set_u32(this + 0x30, data);
        e.mem.set_i32(this + 0x10, rows);
        e.mem.set_i32(this + 0x14, columns);
        e.mem.set_f32(this + 0x34, 100.0);
        (e, this)
    }

    #[test]
    fn stores_the_base_plus_the_extremes() {
        let (mut e, this) = setup(2, 2, &[3.0, -1.0, 7.5, 2.0]);
        e.call(0x00ca_fef0, &args![this]);
        assert_eq!(e.mem.f32(this + 0x18), 99.0);
        assert_eq!(e.mem.f32(this + 0x1c), 107.5);
    }

    #[test]
    fn a_missing_array_changes_nothing() {
        let (mut e, this) = setup(2, 2, &[1.0; 4]);
        e.mem.set_u32(this + 0x30, 0);
        e.call(0x00ca_fef0, &args![this]);
        assert_eq!(e.mem.f32(this + 0x18), 0.0);
        assert_eq!(e.mem.f32(this + 0x1c), 0.0);
    }

    #[test]
    fn without_rows_both_extremes_are_the_first_float() {
        let (mut e, this) = setup(0, 2, &[4.0, 9.0]);
        e.call(0x00ca_fef0, &args![this]);
        assert_eq!(e.mem.f32(this + 0x18), 104.0);
        assert_eq!(e.mem.f32(this + 0x1c), 104.0);
        e.mem.set_i32(this + 0x10, -3);
        e.call(0x00ca_fef0, &args![this]);
        assert_eq!(e.mem.f32(this + 0x18), 104.0);
    }

    #[test]
    fn without_columns_the_walk_reads_nothing() {
        let (mut e, this) = setup(3, 0, &[4.0, 9.0]);
        e.call(0x00ca_fef0, &args![this]);
        assert_eq!(e.mem.f32(this + 0x18), 104.0);
        assert_eq!(e.mem.f32(this + 0x1c), 104.0);
    }

    #[test]
    fn a_nan_float_replaces_both_extremes() {
        let (mut e, this) = setup(1, 2, &[1.0, f32::NAN]);
        e.call(0x00ca_fef0, &args![this]);
        assert!(e.mem.f32(this + 0x18).is_nan());
        assert!(e.mem.f32(this + 0x1c).is_nan());
    }
}
