//! `NiAnimation/nitcbrotkey.cpp` (Xbox PDB source unit), subsystem `NiAnimation`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `NiQuaternion` constructor from four floats (`W, X, Y, Z`),
/// `00a6db50`.
const QUATERNION_SET: u32 = 0x00a6_db50;

// Translated from 00a2bcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiQuaternion::operator*` (Xbox PDB; the engine map writes `operatorP`),
/// the copy of the Hamilton product emitted in this unit, returned by value:
/// `result` (hidden, first after `this`) is built by `00a6db50` from the
/// four components of `this * b`, each summed in the exe's x87 order and
/// rounded to a float. Returns `result`.
pub fn niquaternion_mul(e: &mut Engine, this: Ptr, result: Ptr, b: Ptr) -> Ptr {
    let (a, b) = (this.addr(), b.addr());
    // Each term: FLD dword [x]; FMUL dword [y].
    let term =
        |e: &Engine, x: u32, y: u32| x87_mul(x87_load32(e.mem.u32(x)), x87_m32(e.mem.u32(y)));
    // FSTP dword; FLD dword; FSTP dword (the argument slot).
    let spill = |v: f64| x87_store32(x87_load32(x87_store32(v)));
    let z = x87_add(term(e, b, a + 0xc), term(e, a, b + 0xc));
    let z = x87_add(z, term(e, b + 8, a + 4));
    let z = spill(x87_sub(z, term(e, b + 4, a + 8)));
    let y = x87_add(term(e, b + 8, a), term(e, a + 8, b));
    let y = x87_add(y, term(e, b + 4, a + 0xc));
    let y = spill(x87_sub(y, term(e, a + 4, b + 0xc)));
    let x = x87_add(term(e, a + 4, b), term(e, b + 4, a));
    let x = x87_add(x, term(e, a + 8, b + 0xc));
    let x = spill(x87_sub(x, term(e, b + 8, a + 0xc)));
    let w = x87_sub(term(e, b, a), term(e, b + 4, a + 4));
    let w = x87_sub(w, term(e, a + 8, b + 8));
    let w = spill(x87_sub(w, term(e, b + 0xc, a + 0xc)));
    e.call(QUATERNION_SET, &args![result, w, x, y, z]);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00a2bcf0, niquaternion_mul(Ptr, Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quat(e: &mut Engine, v: [f32; 4]) -> Ptr {
        let q = Ptr::new(e.mem.alloc(0x10));
        for (i, x) in v.iter().enumerate() {
            e.mem.set_f32(q.addr() + 4 * i as u32, *x);
        }
        q
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(QUATERNION_SET, |e, a| {
            for i in 0..4 {
                e.mem.set_u32(a[0] + 4 * i as u32, a[1 + i]);
            }
            Ret::default()
        });
        e
    }

    #[test]
    fn product_passes_the_hamilton_components_to_the_constructor() {
        let mut e = engine();
        let a = quat(&mut e, [1.0, 2.0, 3.0, 4.0]);
        let b = quat(&mut e, [5.0, 6.0, 7.0, 8.0]);
        let out: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.call_log = Some(vec![]);
        let r = e.call(0x00a2_bcf0, &args![a, out, b]).u32();
        assert_eq!(r, out.addr());
        let log = e.call_log.take().unwrap();
        let calls = log.iter().filter(|(f, _)| *f == QUATERNION_SET).count();
        assert_eq!(calls, 1);
        let got: Vec<f32> = (0..4).map(|i| e.mem.f32(out.addr() + 4 * i)).collect();
        assert_eq!(got, vec![-60.0, 12.0, 30.0, 24.0]);
    }

    #[test]
    fn product_quiets_a_signalling_nan_component() {
        let mut e = engine();
        let a = quat(&mut e, [1.0, 0.0, 0.0, 0.0]);
        let b = quat(&mut e, [1.0, 0.0, 0.0, 0.0]);
        e.mem.set_u32(b.addr(), 0x7f80_0001);
        let out: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.call(0x00a2_bcf0, &args![a, out, b]);
        assert_eq!(e.mem.u32(out.addr()), 0x7fc0_0001);
    }
}
