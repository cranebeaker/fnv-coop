//! `(unplaced)/run_00a813c0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `011ab3fc`: a `float` the step angle is made from.
const ANGLE_SCALE: u32 = 0x011a_b3fc;
/// `0109e268`: a `double` the scale is multiplied by.
const ANGLE_FACTOR: u32 = 0x0109_e268;
/// `00eca0a0` and `00ec9f70`: the C library's sine and cosine (value in `ST0`, answer in
/// `ST0`, so a leading `f64` word pair).
const SINE: u32 = 0x00ec_a0a0;
const COSINE: u32 = 0x00ec_9f70;
/// The first table of 0x200 floats `00a813c0` fills (sine of the angles), `011f52e0`.
const SINE_TABLE: u32 = 0x011f_52e0;
/// The second table of 0x200 floats (cosine of the angles), `011f4ae0`.
const COSINE_TABLE: u32 = 0x011f_4ae0;

// Translated from 00a813c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no unit in the engine map; run by the start-up `00a89cc0`, no arguments):
/// fills two tables of 0x200 floats, `011f52e0` with the sine and `011f4ae0` with the
/// cosine of an angle that starts at 0.0 and grows by the step `float(011ab3fc) *
/// double(0109e268)` per entry (the C library's functions `00eca0a0` / `00ec9f70`, each
/// result stored as a `float` and copied once more as `FLD`/`FSTP` does).
pub fn fn_00a813c0(e: &mut Engine) {
    let mut angle = 0.0f32.to_bits();
    let step = x87_mul(
        x87_m32(e.mem.u32(ANGLE_SCALE)),
        e.global::<f64>(ANGLE_FACTOR),
    );
    let mut offset = 0u32;
    loop {
        let sine = e.call(SINE, &args![x87_load32(angle)]).f64();
        let sine = x87_store32(sine);
        e.mem
            .set_u32(SINE_TABLE.wrapping_add(offset), x87_quiet_bits(sine));
        let cosine = e.call(COSINE, &args![x87_load32(angle)]).f64();
        let cosine = x87_store32(cosine);
        offset = offset.wrapping_add(4);
        e.mem.set_u32(
            COSINE_TABLE.wrapping_add(offset - 4),
            x87_quiet_bits(cosine),
        );
        angle = x87_store32(x87_add(x87_load32(angle), step));
        if offset >= 0x800 {
            break;
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a8_13c0, fn_00a813c0())]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_both_tables_with_the_library_answers() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x2000);
        e.map(0x011a_b000, 0x1000);
        e.map(0x0109_e000, 0x1000);
        e.mem.set_u32(ANGLE_SCALE, 2.0f32.to_bits());
        e.set_global::<f64>(ANGLE_FACTOR, 0.25);
        e.register(SINE, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (x * 10.0).into_ret()
        });
        e.register(COSINE, |_, a| {
            let x = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (x * 100.0).into_ret()
        });
        e.call(0x00a8_13c0, &args![]);
        // The angle grows by 2.0 * 0.25 = 0.5 per entry.
        for index in [0u32, 1, 2, 0x1ff] {
            let angle = index as f32 * 0.5;
            assert_eq!(
                f32::from_bits(e.mem.u32(SINE_TABLE + 4 * index)),
                angle * 10.0
            );
            assert_eq!(
                f32::from_bits(e.mem.u32(COSINE_TABLE + 4 * index)),
                angle * 100.0
            );
        }
    }
}
