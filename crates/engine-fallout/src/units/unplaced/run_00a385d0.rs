//! `(unplaced)/run_00a385d0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The invalid marker as a `double` (`010241b0`, -FLT_MAX) the parts are
/// compared with, and as the `float` (`01096b8c`, -FLT_MAX) written for an
/// invalid part.
const INVALID_DOUBLE: u32 = 0x0102_41b0;
const INVALID_FLOAT: u32 = 0x0109_6b8c;
/// `NiQuaternion::UnitInverse` (`00a6dc00`, engine map; cdecl: the result's
/// address, the quaternion; returns the result's address).
const QUATERNION_UNIT_INVERSE: u32 = 0x00a6_dc00;
/// The transform's parts (as the code reads them): the translation (three
/// `float`s at +0), the rotation (a quaternion at +0xc, its second word at
/// +0x10 the one tested) and the scale (+0x1c).
const TRANSLATION: u32 = 0x00;
const ROTATION: u32 = 0x0c;
const ROTATION_TESTED: u32 = 0x10;
const SCALE: u32 = 0x1c;

// Translated from 00a385d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the inverse of a translation/rotation/scale transform into `out`
/// (thiscall, `RET 4`; unnamed in the engine map). Each part whose tested
/// `float` equals the invalid marker (-FLT_MAX, `FUCOM` with the `double`
/// at `010241b0`; a NaN is not equal) is marked invalid in `out` (the
/// `float` at `01096b8c` written to that tested word only); otherwise the
/// scale becomes `1 / scale`, the rotation its unit inverse
/// (`NiQuaternion::UnitInverse`) and the translation its negation.
pub fn fn_00a385d0(e: &mut Engine, this: Ptr, out: Ptr) {
    let source = this.addr();
    let target = out.addr();
    let invalid = e.global::<f64>(INVALID_DOUBLE);
    let invalid_float = x87_quiet_bits(e.global::<u32>(INVALID_FLOAT));
    let is_invalid = |e: &Engine, address: u32| x87_load32(e.mem.u32(address)) == invalid;

    if is_invalid(e, source + SCALE) {
        e.mem.set_u32(target + SCALE, invalid_float);
    } else {
        let scale = x87_load32(e.mem.u32(source + SCALE));
        e.mem
            .set_u32(target + SCALE, x87_store32(x87_div(1.0, scale)));
    }

    if is_invalid(e, source + ROTATION_TESTED) {
        e.mem.set_u32(target + ROTATION_TESTED, invalid_float);
    } else {
        e.with_stack(16, |e, local| {
            let inverse = e
                .call(QUATERNION_UNIT_INVERSE, &args![local, source + ROTATION])
                .u32();
            for i in 0..4 {
                let word = e.mem.u32(inverse + i * 4);
                e.mem.set_u32(target + ROTATION + i * 4, word);
            }
        });
    }

    if is_invalid(e, source + TRANSLATION) {
        e.mem.set_u32(target + TRANSLATION, invalid_float);
        return;
    }
    // FLD/FCHS/FSTP: a signalling NaN is quieted, the sign flipped.
    for i in 0..3 {
        let bits = x87_quiet_bits(e.mem.u32(source + TRANSLATION + i * 4));
        e.mem
            .set_u32(target + TRANSLATION + i * 4, bits ^ 0x8000_0000);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a3_85d0, fn_00a385d0(Ptr, Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const MINUS_FLT_MAX: u32 = 0xff7f_ffff;

    fn setup(e: &mut Engine) {
        e.map(INVALID_DOUBLE, 8);
        e.mem.set_f64(INVALID_DOUBLE, -f64::from(f32::MAX));
        e.map(INVALID_FLOAT, 4);
        e.mem.set_u32(INVALID_FLOAT, MINUS_FLT_MAX);
        // The unit inverse: the conjugate (w, -x, -y, -z) written to the
        // result.
        e.register(QUATERNION_UNIT_INVERSE, |e, a| {
            for i in 0..4 {
                let bits = e.mem.u32(a[1] + i * 4);
                e.mem
                    .set_u32(a[0] + i * 4, if i == 0 { bits } else { bits ^ 0x8000_0000 });
            }
            a[0].into_ret()
        });
    }

    fn transform(e: &mut Engine, words: [u32; 8]) -> u32 {
        let this = e.mem.alloc(0x20);
        for (i, word) in words.iter().enumerate() {
            e.mem.set_u32(this + i as u32 * 4, *word);
        }
        this
    }

    #[test]
    fn inverts_each_valid_part() {
        let mut e = Engine::new();
        setup(&mut e);
        let f = |v: f32| v.to_bits();
        let this = transform(
            &mut e,
            [
                f(1.0),
                f(-2.0),
                f(0.5),
                f(1.0),
                f(0.0),
                f(0.0),
                f(0.0),
                f(4.0),
            ],
        );
        let out = e.mem.alloc(0x20);
        e.call(0x00a3_85d0, &args![this, out]);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(out + i * 4)).collect();
        assert_eq!(
            words,
            vec![
                f(-1.0),
                f(2.0),
                f(-0.5),
                f(1.0),
                f(-0.0),
                f(-0.0),
                f(-0.0),
                f(0.25)
            ]
        );
    }

    #[test]
    fn invalid_parts_mark_only_their_tested_word() {
        let mut e = Engine::new();
        setup(&mut e);
        let this = transform(
            &mut e,
            [MINUS_FLT_MAX, 1, 2, 3, MINUS_FLT_MAX, 5, 6, MINUS_FLT_MAX],
        );
        let out = e.mem.alloc(0x20);
        for i in 0..8 {
            e.mem.set_u32(out + i * 4, 0x1111_0000 + i);
        }
        e.call_log = Some(vec![]);
        e.call(0x00a3_85d0, &args![this, out]);
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(out + i * 4)).collect();
        assert_eq!(
            words,
            vec![
                MINUS_FLT_MAX,
                0x1111_0001,
                0x1111_0002,
                0x1111_0003,
                MINUS_FLT_MAX,
                0x1111_0005,
                0x1111_0006,
                MINUS_FLT_MAX
            ]
        );
        assert!(!e
            .call_log
            .iter()
            .flatten()
            .any(|(a, _)| *a == QUATERNION_UNIT_INVERSE));
    }

    #[test]
    fn nans_are_not_invalid_and_are_quieted() {
        let mut e = Engine::new();
        setup(&mut e);
        let this = transform(&mut e, [0x7f80_0001, 0, 0, 0, 0, 0, 0, 0x7f80_0001]);
        let out = e.mem.alloc(0x20);
        e.call(0x00a3_85d0, &args![this, out]);
        assert_eq!(e.mem.u32(out), 0xffc0_0001);
        assert_eq!(e.mem.u32(out + SCALE), 0x7fc0_0001);
        // The other translation words are negated zeros.
        assert_eq!(e.mem.u32(out + 4), 0x8000_0000);
    }
}
