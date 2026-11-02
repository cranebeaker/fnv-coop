//! `fallout shared/havok/teswindlistener.cpp` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! x87 note: the parameter is loaded and stored again with `FLD`/`FSTP` (a
//! signalling NaN comes out quiet, [`x87_quiet_bits`]) and negated with
//! `FCHS` (which flips the sign bit of any value, a NaN included).

#[allow(unused_imports)]
use crate::prelude::*;

/// `RandomFloat(min, max)` (engine map name of `00476b70`; cdecl, two
/// floats pushed as words, result in `ST0`).
const RANDOM_FLOAT: u32 = 0x0047_6b70;

// Translated from 0062e910 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWindListener::GetRandom` (Xbox PDB name, from the engine map; the
/// address is in the vtables at `0102f134` and `0104e478`): a random float
/// between `-limit` and `limit`, `RandomFloat(-limit, limit)`. `this` is
/// not read. The function ends with the result of `RandomFloat` still in
/// `ST0`, so the return type is `f32`.
pub fn tes_wind_listener_get_random(e: &mut Engine, _this: Ptr, limit: f32) -> f32 {
    let positive = x87_quiet_bits(limit.to_bits());
    let negative = positive ^ 0x8000_0000;
    e.call(RANDOM_FLOAT, &args![negative, positive]).f32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0062_e910, tes_wind_listener_get_random(Ptr, f32) -> f32)]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(limit: f32) -> (Vec<Vec<u32>>, f32) {
        let mut e = Engine::new();
        e.register(RANDOM_FLOAT, |_, _| Ret {
            st0: 0.25,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0062_e910, &args![0x1234u32, limit]).f32();
        let calls = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == RANDOM_FLOAT)
            .map(|(_, w)| w)
            .collect();
        (calls, result)
    }

    #[test]
    fn range_is_minus_limit_to_limit_and_the_result_is_passed_on() {
        let (calls, result) = run(2.5);
        assert_eq!(calls, vec![vec![(-2.5f32).to_bits(), 2.5f32.to_bits()]]);
        assert_eq!(result, 0.25);
    }

    #[test]
    fn zero_limit_gives_minus_zero_as_the_minimum() {
        let (calls, _) = run(0.0);
        assert_eq!(calls, vec![vec![0x8000_0000, 0]]);
    }

    #[test]
    fn signalling_nan_is_quieted_before_it_is_negated() {
        let (calls, _) = run(f32::from_bits(0x7fa0_0000));
        assert_eq!(calls, vec![vec![0xffe0_0000, 0x7fe0_0000]]);
    }
}
