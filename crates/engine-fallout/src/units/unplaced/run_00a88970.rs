//! `(unplaced)/run_00a88970` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// CRT `sqrt` helper taking and returning ST0 (`00ec6040`).
const CRT_SQRT: u32 = 0x00ec_6040;
/// The `NiQuaternion` constructor from four floats (`W, X, Y, Z`),
/// `00a6db50`.
const QUATERNION_SET: u32 = 0x00a6_db50;
/// `float` 0.001: below it the sine counts as zero.
const SINE_EPSILON: u32 = 0x0109_c9c0;

/// `FSTP dword tmp; FLD dword tmp`.
fn spill(v: f64) -> f64 {
    x87_load32(x87_store32(v))
}

// Translated from 00a88970 (decompiled, FalloutNV.exe 1.4.0.525)
/// The quaternion exponential, returned by value (`cdecl`, hidden `result`
/// first; no Xbox PDB name): the angle is the length of the vector part
/// `(X, Y, Z)` of `q` (CRT `sqrt`, rounded to a float); `result` is built
/// by `00a6db50` from `cos(angle)` and the vector part scaled by
/// `sin(angle) / angle` (1 when `0.001` is above `|sin|`). Returns
/// `result`.
///
/// The exe takes sine and cosine with one `FSINCOS` of the float angle;
/// they are computed here in `f64`, which can differ from the
/// extended-precision results in the last bit of a float.
pub fn fn_00a88970(e: &mut Engine, result: Ptr, q: Ptr) -> Ptr {
    let q = q.addr();
    let ld = |e: &Engine, a: u32| x87_load32(e.mem.u32(a));
    let y = ld(e, q + 8);
    let x = ld(e, q + 4);
    let z = ld(e, q + 0xc);
    let xx = x87_mul(x, x);
    let yy = x87_mul(y, y);
    let sum = x87_add(yy, xx);
    let zz = x87_mul(z, z);
    let length_squared = spill(x87_add(sum, zz));
    let root = e.call(CRT_SQRT, &args![length_squared]).f64();
    let angle_bits = x87_store32(spill(root));
    let angle = x87_load32(angle_bits);
    let cos_bits = x87_store32(angle.cos());
    let sine = spill(angle.sin());
    let magnitude = spill(sine.abs());
    let epsilon = ld(e, SINE_EPSILON);
    // FLD 0.001; FCOMPP; TEST AH,0x41; JNZ: below, equal or unordered
    // divides.
    let scale = if epsilon > magnitude {
        1.0
    } else {
        x87_div(sine, x87_m32(angle_bits))
    };
    let k = spill(scale);
    let vz = x87_mul(ld(e, q + 0xc), k);
    let vy = x87_mul(k, x87_m32(e.mem.u32(q + 8)));
    let vx = x87_mul(k, x87_m32(e.mem.u32(q + 4)));
    let words = [
        x87_quiet_bits(cos_bits),
        x87_store32(spill(vx)),
        x87_store32(spill(vy)),
        x87_store32(spill(vz)),
    ];
    e.call(
        QUATERNION_SET,
        &args![result, words[0], words[1], words[2], words[3]],
    );
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a88970, fn_00a88970(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0109_c000, 0x1000);
        e.set_global(SINE_EPSILON, 0x3a83_126fu32);
        e.register(CRT_SQRT, |_, a| f64::take(a, &mut 0).sqrt().into_ret());
        e.register(QUATERNION_SET, |_, _| Ret::default());
        e
    }

    fn set_words(e: &Engine) -> Vec<u32> {
        let log = e.call_log.as_ref().unwrap();
        let call = log.iter().find(|(a, _)| *a == QUATERNION_SET).unwrap();
        call.1[1..].to_vec()
    }

    #[test]
    fn exponential_scales_the_vector_by_sine_over_angle() {
        let mut e = engine();
        let q: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_f32(q.addr() + 4, 0.3);
        e.mem.set_f32(q.addr() + 8, 0.4);
        let out: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.call_log = Some(vec![]);
        let r = e.call(0x00a8_8970, &args![out, q]).u32();
        assert_eq!(r, out.addr());
        let x = 0.3f32 as f64;
        let y = 0.4f32 as f64;
        let angle = ((y * y + x * x) as f32 as f64).sqrt() as f32 as f64;
        let sine = angle.sin() as f32 as f64;
        let k = (sine / angle) as f32 as f64;
        assert_eq!(
            set_words(&e),
            vec![
                (angle.cos() as f32).to_bits(),
                ((k * x) as f32).to_bits(),
                ((k * y) as f32).to_bits(),
                0,
            ]
        );
    }

    #[test]
    fn a_zero_vector_gives_the_identity() {
        let mut e = engine();
        let q: Ptr = Ptr::new(e.mem.alloc(0x10));
        let out: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.call_log = Some(vec![]);
        e.call(0x00a8_8970, &args![out, q]);
        assert_eq!(set_words(&e), vec![0x3f80_0000, 0, 0, 0]);
    }
}
