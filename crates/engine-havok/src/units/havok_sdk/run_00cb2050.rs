//! `Havok SDK/run_00cb2050` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The four-word mask `ANDPS` applies to each of the first three rows (`010c4c10`: all ones in
/// the first three lanes, zero in the fourth, as the decompilation shows).
const ROW_MASK: u32 = 0x010c_4c10;
/// The constant vector whose fourth lane ends up in the fourth lane of the last row
/// (`010c7170`).
const LAST_ROW_CONSTANT: u32 = 0x010c_7170;

/// Copies a four-row, 16-byte-per-row matrix from `source` to `destination`: each of the first
/// three rows is ANDed lane by lane with the mask at `010c4c10`; the fourth row keeps its first
/// three lanes and takes its fourth lane from the vector at `010c7170` (`UNPCKHPS` then
/// `SHUFPS 0xc4`: `(row[0], row[1], row[2], constant[3])`).
fn copy_masked_matrix(e: &mut Engine, source: u32, destination: u32) {
    for row in 0..3 {
        for lane in 0..4 {
            let mask = e.mem.u32(ROW_MASK + 4 * lane);
            let word = e.mem.u32(source + 0x10 * row + 4 * lane);
            e.mem
                .set_u32(destination + 0x10 * row + 4 * lane, word & mask);
        }
    }
    for lane in 0..3 {
        let word = e.mem.u32(source + 0x30 + 4 * lane);
        e.mem.set_u32(destination + 0x30 + 4 * lane, word);
    }
    let last = e.mem.u32(LAST_ROW_CONSTANT + 12);
    e.mem.set_u32(destination + 0x3c, last);
}

// Translated from 00cb2050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the 4x4 float matrix at `this` to `out` with [`copy_masked_matrix`] (the fourth
/// column of the first three rows cleared, the last lane of the last row set from the
/// constant). The class is unconfirmed.
pub fn fn_00cb2050(e: &mut Engine, this: Ptr, out: Ptr) {
    copy_masked_matrix(e, this.addr(), out.addr());
}

// Translated from 00cb20b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same copy in the other direction: the matrix at `source` into `this`, with
/// [`copy_masked_matrix`]. The class is unconfirmed.
pub fn fn_00cb20b0(e: &mut Engine, this: Ptr, source: Ptr) {
    copy_masked_matrix(e, source.addr(), this.addr());
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00cb_2050, fn_00cb2050(Ptr, Ptr)),
        entry!(0x00cb_20b0, fn_00cb20b0(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The mask and constant of the exe, and a matrix of distinct words.
    fn rig() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0030_0000);
        for (lane, word) in [0xffff_ffffu32, 0xffff_ffff, 0xffff_ffff, 0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(ROW_MASK + 4 * lane as u32, word);
        }
        for (lane, word) in [0x1u32, 0x2, 0x3, 0x3f80_0000].into_iter().enumerate() {
            e.mem.set_u32(LAST_ROW_CONSTANT + 4 * lane as u32, word);
        }
        let matrix = e.mem.alloc(0x40);
        for i in 0..16 {
            e.mem.set_u32(matrix + 4 * i, 0x1111_0000 + i);
        }
        (e, matrix)
    }

    fn words(e: &Engine, at: u32) -> Vec<u32> {
        (0..16).map(|i| e.mem.u32(at + 4 * i)).collect()
    }

    /// The expected copy: the fourth lane of rows 0 to 2 cleared by the mask and the last
    /// word taken from the constant.
    fn expected(source: &[u32]) -> Vec<u32> {
        let mut want = source.to_vec();
        want[3] = 0;
        want[7] = 0;
        want[11] = 0;
        want[15] = 0x3f80_0000;
        want
    }

    #[test]
    fn matrix_copy_masks_the_fourth_lane_of_three_rows_and_sets_the_last_word() {
        let (mut e, matrix) = rig();
        let out = e.mem.alloc(0x40);
        e.call(0x00cb_2050, &args![matrix, out]);
        assert_eq!(words(&e, out), expected(&words(&e, matrix)));
        // The mask is read from memory: a different mask is honoured.
        e.mem.set_u32(ROW_MASK, 0x0000_ffff);
        e.call(0x00cb_2050, &args![matrix, out]);
        assert_eq!(e.mem.u32(out), 0x1111_0000 & 0xffff);
    }

    #[test]
    fn matrix_copy_in_the_other_direction_is_the_same_copy() {
        let (mut e, matrix) = rig();
        let this = e.mem.alloc(0x40);
        e.call(0x00cb_20b0, &args![this, matrix]);
        assert_eq!(words(&e, this), expected(&words(&e, matrix)));
    }
}
