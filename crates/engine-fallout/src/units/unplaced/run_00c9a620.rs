//! `(unplaced)/run_00c9a620` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Whether the three floats at `a` and `b` compare equal one by one, as the exe's
/// `FUCOMPP`/`TEST AH,0x44`/`JP` chain does (unordered, so a NaN is never equal), stopping
/// at the first difference.
fn three_floats_equal(e: &Engine, a: u32, b: u32) -> bool {
    (0..3).all(|i| e.mem.f32(b + 4 * i) == e.mem.f32(a + 4 * i))
}

// Translated from 00c9a620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: compares the three floats at `this` with those at `other`
/// (x, y, z in turn); answers 1 in `EAX` when all three are equal, else 0. A NaN on either
/// side is unequal.
pub fn fn_00c9a620(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    three_floats_equal(e, this.addr(), other.addr())
}

// Translated from 00c9a660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the same comparison as `00c9a620`, inlined, with the answer
/// inverted (`SETZ`): true when any of the three floats differs (or is NaN).
pub fn fn_00c9a660(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let equal = three_floats_equal(e, this.addr(), other.addr());
    !equal
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c9_a620, fn_00c9a620(Ptr, Ptr) -> bool),
        entry!(0x00c9_a660, fn_00c9a660(Ptr, Ptr) -> bool),
    ]
}

#[cfg(test)]
mod n081_tests {
    use super::*;

    fn point(e: &mut Engine, v: [f32; 3]) -> u32 {
        let p = e.mem.alloc(12);
        for (i, x) in v.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *x);
        }
        p
    }

    #[test]
    fn equal_and_not_equal_follow_each_component() {
        let mut e = Engine::new();
        let a = point(&mut e, [1.0, 2.0, 3.0]);
        for (other, equal) in [
            ([1.0, 2.0, 3.0], true),
            ([1.5, 2.0, 3.0], false),
            ([1.0, 2.5, 3.0], false),
            ([1.0, 2.0, 3.5], false),
            ([1.0, f32::NAN, 3.0], false),
        ] {
            let b = point(&mut e, other);
            assert_eq!(e.call(0x00c9_a620, &args![a, b]).u32(), u32::from(equal));
            assert_eq!(e.call(0x00c9_a660, &args![a, b]).bool(), !equal);
        }
        let zero = point(&mut e, [0.0, 0.0, 0.0]);
        let negative_zero = point(&mut e, [-0.0, 0.0, 0.0]);
        assert!(e.call(0x00c9_a620, &args![zero, negative_zero]).bool());
        let nan = point(&mut e, [f32::NAN, 0.0, 0.0]);
        assert!(!e.call(0x00c9_a620, &args![nan, nan]).bool());
    }
}
