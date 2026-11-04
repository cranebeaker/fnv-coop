//! `fallout shared/region system/tesregionpoint.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// Callees of the constructor: `006815c0` (`thiscall`, no arguments) and the two float
/// setters `0052ce20` and `004f5d90` (`thiscall`, `(value)`).
const BASE_006815C0: u32 = 0x0068_15c0;
const SET_VALUE_0052CE20: u32 = 0x0052_ce20;
const SET_VALUE_004F5D90: u32 = 0x004f_5d90;

// Translated from 004f6fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionPoint::TESRegionPoint` (Xbox PDB): `006815c0` on `this`, then `0052ce20(0.0)`
/// and `004f5d90(0.0)` on it. Answers `this`.
pub fn tes_region_point_constructor(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_006815C0, &args![this]);
    let zero = x87_store32(0.0);
    e.call(SET_VALUE_0052CE20, &args![this, zero]);
    e.call(SET_VALUE_004F5D90, &args![this, zero]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f6fc0, tes_region_point_constructor(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_point_constructor_zeroes_both_floats() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        for addr in [BASE_006815C0, SET_VALUE_0052CE20, SET_VALUE_004F5D90] {
            e.register(addr, |_, _| Ret::default());
        }
        assert_eq!(e.call(0x004f6fc0, &args![0x5000u32]).u32(), 0x5000);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (BASE_006815C0, vec![0x5000]),
                (SET_VALUE_0052CE20, vec![0x5000, 0]),
                (SET_VALUE_004F5D90, vec![0x5000, 0])
            ]
        );
    }
}
