//! `Havok SDK/run_00d4e480` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The CRT helper `00eca0a0` (value in and out in `ST0`; the other translations use it as sine) and
/// the CRT square root `00ec6040` (argument in `ST0`, result in `ST0`).
const CRT_SECOND_TRIG: u32 = 0x00ec_a0a0;
const SQUARE_ROOT: u32 = 0x00ec_6040;

type Lanes = [f32; 4];

fn lanes_00d4e480(e: &Engine, at: u32) -> Lanes {
    [0u32, 1, 2, 3].map(|lane| e.mem.f32(at + lane * 4))
}

fn store_00d4e480(e: &mut Engine, at: u32, value: Lanes) {
    for (lane, word) in value.into_iter().enumerate() {
        e.mem.set_u32(at + lane as u32 * 4, word.to_bits());
    }
}

fn yzx_00d4e480(v: Lanes) -> Lanes {
    [v[1], v[2], v[0], v[3]]
}

fn zxy_00d4e480(v: Lanes) -> Lanes {
    [v[2], v[0], v[1], v[3]]
}

// Translated from 00d4e480 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name (thiscall, `RET 4`: one stack float `angle`; descriptions, not confirmed): with
/// `s = float(G(angle))` (`G` = `00eca0a0`) and `r = float(sqrt(float(1 - s * s)))` (x87 double
/// arithmetic, rounded to a float at each store; `sqrt` = `00ec6040`), `v1` the vector at
/// `this + 0x140` and `v2` the one at `this + 0x150`: stores at `this + 0x100`
/// `q = s * (v2 x v1) + r * v1` and at `this + 0x120` `q x v2` (cross products as `a.yzx * b.zxy -
/// a.zxy * b.yzx`, four lanes).
pub fn fn_00d4e480(e: &mut Engine, this: Ptr, angle: f32) {
    let me = this.addr();
    let wide = e
        .call(CRT_SECOND_TRIG, &args![x87_load32(angle.to_bits())])
        .f64();
    // FSTP / FLD / FSTP / FLD of the result: a float round trip, twice.
    let sine_bits = x87_store32(x87_load32(x87_store32(wide)));
    let sine = x87_load32(sine_bits);
    let remainder = x87_sub(1.0, x87_mul(sine, sine));
    let remainder_bits = x87_store32(remainder);
    let root = e
        .call(SQUARE_ROOT, &args![x87_load32(remainder_bits)])
        .f64();
    let cosine_bits = x87_store32(x87_load32(x87_store32(root)));
    let (s, r) = (f32::from_bits(sine_bits), f32::from_bits(cosine_bits));
    let v2 = lanes_00d4e480(e, me + 0x150);
    let v1 = lanes_00d4e480(e, me + 0x140);
    let positive = [0usize, 1, 2, 3].map(|lane| yzx_00d4e480(v2)[lane] * zxy_00d4e480(v1)[lane]);
    let negative = [0usize, 1, 2, 3].map(|lane| zxy_00d4e480(v2)[lane] * yzx_00d4e480(v1)[lane]);
    let cross = [0usize, 1, 2, 3].map(|lane| (positive[lane] - negative[lane]) * s);
    let v1_again = lanes_00d4e480(e, me + 0x140);
    let q = [0usize, 1, 2, 3].map(|lane| cross[lane] + r * v1_again[lane]);
    store_00d4e480(e, me + 0x100, q);
    let v2 = lanes_00d4e480(e, me + 0x150);
    let positive = [0usize, 1, 2, 3].map(|lane| yzx_00d4e480(v2)[lane] * zxy_00d4e480(q)[lane]);
    let negative = [0usize, 1, 2, 3].map(|lane| zxy_00d4e480(v2)[lane] * yzx_00d4e480(q)[lane]);
    let result = [0usize, 1, 2, 3].map(|lane| negative[lane] - positive[lane]);
    store_00d4e480(e, me + 0x120, result);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00d4_e480, fn_00d4e480(Ptr, f32))]
}

#[cfg(test)]
mod havok_a04_tests {
    use super::*;

    #[test]
    fn fn_00d4e480_rotates_the_first_vector_about_the_second() {
        let mut e = Engine::new();
        e.register(CRT_SECOND_TRIG, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sin(),
            ..Ret::default()
        });
        e.register(SQUARE_ROOT, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        let this = e.mem.alloc(0x180);
        store_00d4e480(&mut e, this + 0x140, [1.0, 0.0, 0.0, 0.0]);
        store_00d4e480(&mut e, this + 0x150, [0.0, 1.0, 0.0, 0.0]);
        e.call(0x00d4_e480, &args![this, 0.5f32]);
        let sine = 0.5f64.sin() as f32;
        let sine_wide = f64::from(sine);
        let remainder = (1.0 - sine_wide * sine_wide) as f32;
        let cosine = f64::from(remainder).sqrt() as f32;
        // q = s * (v2 x v1) + r * v1 = (r, 0, -s); q x v2 = (s, 0, r).
        let q = lanes_00d4e480(&e, this + 0x100);
        assert_eq!(q, [cosine, 0.0, -sine, 0.0]);
        let rotated = lanes_00d4e480(&e, this + 0x120);
        assert_eq!(rotated, [sine, 0.0, cosine, 0.0]);
    }
}
