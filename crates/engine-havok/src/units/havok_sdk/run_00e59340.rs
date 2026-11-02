//! `Havok SDK/run_00e59340` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00e59340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl, `(ok, matrix, inverse, epsilon)`: inverts the 2x2 `float` matrix `m` at `matrix`
/// (words `m0 m1 m2 m3`). The determinant `m0 * m3 - m2 * m1` is computed on the x87 stack and
/// stored as a float; when its square is below `epsilon` squared (the comparison is the exe's
/// `FCOMPP` with `TEST AH, 0x41`: an equal or unordered pair counts as invertible) the byte at
/// `ok` is 0 and nothing else is written. Otherwise, with `inv = 1 / det` (stored as a float),
/// the inverse is `inverse[0] = inv * m3`, `inverse[1] = -m2 * inv`, `inverse[2] = -m1 * inv`,
/// `inverse[3] = m0 * inv` (each rounded to a float) and the byte at `ok` is 1.
pub fn fn_00e59340(e: &mut Engine, ok: u32, matrix: u32, inverse: u32, epsilon: f32) {
    let word = |e: &Engine, index: u32| e.mem.u32(matrix.wrapping_add(4 * index));
    let first = x87_mul(x87_load32(word(e, 0)), x87_m32(word(e, 3)));
    let second = x87_mul(x87_load32(word(e, 2)), x87_m32(word(e, 1)));
    let determinant_bits = x87_store32(x87_sub(first, second));
    let determinant = x87_load32(determinant_bits);
    let determinant_squared = x87_mul(determinant, determinant);
    let epsilon_value = x87_load32(epsilon.to_bits());
    let epsilon_squared = x87_mul(epsilon_value, epsilon_value);
    // FCOMPP of the two squares, `TEST AH, 0x41`: invertible unless `epsilon^2 > det^2`.
    let status =
        crate::units::havok_sdk::run_00d6bf60::fcom_status(epsilon_squared, determinant_squared);
    if status & 0x41 == 0 {
        e.mem.set_u8(ok, 0);
        return;
    }
    let reciprocal_bits = x87_store32(x87_div(1.0, determinant));
    let reciprocal = x87_load32(reciprocal_bits);
    let negated_second = -x87_load32(word(e, 1));
    let third = x87_store32(x87_mul(negated_second, reciprocal));
    let negated_third = -x87_load32(word(e, 2));
    let second_word = x87_store32(x87_mul(negated_third, reciprocal));
    let fourth = x87_store32(x87_mul(x87_load32(word(e, 0)), reciprocal));
    let first_word = x87_store32(x87_mul(reciprocal, x87_m32(word(e, 3))));
    e.mem.set_u32(inverse, first_word);
    e.mem
        .set_u32(inverse.wrapping_add(8), x87_quiet_bits(third));
    e.mem
        .set_u32(inverse.wrapping_add(4), x87_quiet_bits(second_word));
    e.mem
        .set_u32(inverse.wrapping_add(0xc), x87_quiet_bits(fourth));
    e.mem.set_u8(ok, 1);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00e5_9340, fn_00e59340(u32, u32, u32, f32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn matrix(e: &mut Engine, m: [f32; 4]) -> u32 {
        let at = e.mem.alloc(16);
        for (index, value) in m.iter().enumerate() {
            e.mem.set_f32(at + 4 * index as u32, *value);
        }
        at
    }

    fn invert(e: &mut Engine, m: [f32; 4], epsilon: f32) -> (u8, [f32; 4]) {
        let at = matrix(e, m);
        let inverse = matrix(e, [9.0; 4]);
        let ok = e.mem.alloc(8);
        e.mem.set_u8(ok, 0xee);
        e.call(0x00e5_9340, &args![ok, at, inverse, epsilon]);
        assert_eq!(e.mem.u8(ok + 1), 0);
        let words = [0, 1, 2, 3].map(|index| e.mem.f32(inverse + 4 * index));
        (e.mem.u8(ok), words)
    }

    #[test]
    fn invertible_matrix_gets_its_inverse() {
        let mut e = Engine::new();
        // det = 2 * 1 - 1 * 1 = 1.
        assert_eq!(
            invert(&mut e, [2.0, 1.0, 1.0, 1.0], 0.001),
            (1, [1.0, -1.0, -1.0, 2.0])
        );
        // det = 4 * 2 - 0 = 8, the reciprocal 0.125 is exact.
        let (ok, words) = invert(&mut e, [4.0, 0.0, 0.0, 2.0], 0.5);
        assert_eq!(ok, 1);
        assert_eq!(words[0], 0.25);
        assert_eq!(words[3], 0.5);
        assert_eq!(words[1], 0.0);
        assert!(words[1].is_sign_negative());
    }

    #[test]
    fn singular_matrix_only_clears_the_flag() {
        let mut e = Engine::new();
        assert_eq!(invert(&mut e, [1.0, 1.0, 1.0, 1.0], 0.1), (0, [9.0; 4]));
        // det^2 below epsilon^2.
        assert_eq!(invert(&mut e, [1.0, 0.5, 0.5, 1.0], 1.0), (0, [9.0; 4]));
    }

    #[test]
    fn equal_squares_count_as_invertible() {
        let mut e = Engine::new();
        let (ok, words) = invert(&mut e, [2.0, 0.0, 0.0, 1.0], 2.0);
        assert_eq!(ok, 1);
        assert_eq!(words, [0.5, 0.0, 0.0, 1.0]);
    }
}
