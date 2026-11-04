//! `fallout shared/region system/run_004f5040` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESRegionData::TESRegionData` (`004f1400`).
const TES_REGION_DATA_CTOR: u32 = 0x004f_1400;
/// The vtable of `TESRegionDataWeather` (RTTI `.?AVTESRegionDataWeather@@`, read at the word before it).
const TES_REGION_DATA_WEATHER_VTABLE: u32 = 0x0102_3e18;
/// `005822c0` (unnamed in the engine map): the constructor run on the
/// weather list at +8 (`weatherList`, Xbox PDB).
const WEATHER_LIST_CTOR_005822C0: u32 = 0x0058_22c0;

// Translated from 004f5040 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f5040 - its FS:[0] exception frame (the unwind state that destroys the base when the list constructor throws) is not translated
/// `TESRegionDataWeather` constructor (unnamed in the engine map; the
/// vtable's RTTI names the class): runs the `TESRegionData` constructor,
/// stores the `TESRegionDataWeather` vtable and constructs the weather list
/// at +8 (`005822c0`). Returns `this`.
pub fn fn_004f5040(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(TES_REGION_DATA_CTOR, &args![this]);
    e.mem.set_u32(this.addr(), TES_REGION_DATA_WEATHER_VTABLE);
    e.call(WEATHER_LIST_CTOR_005822C0, &args![this.addr() + 8]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f_5040, fn_004f5040(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004f5040_runs_the_base_then_the_list_constructor() {
        let mut e = Engine::new();
        e.register(TES_REGION_DATA_CTOR, |_, _| Ret::default());
        e.register(WEATHER_LIST_CTOR_005822C0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_5040, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_WEATHER_VTABLE);
        let calls: Vec<_> = log.into_iter().filter(|c| c.0 != 0x004f_5040).collect();
        assert_eq!(
            calls,
            vec![
                (TES_REGION_DATA_CTOR, vec![this]),
                (WEATHER_LIST_CTOR_005822C0, vec![this + 8]),
            ]
        );
    }
}
