//! `fallout shared/region system/run_004f1ed0` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESRegionData::TESRegionData` (`004f1400`).
const TES_REGION_DATA_CTOR: u32 = 0x004f_1400;
/// The vtable of `TESRegionDataImposter` (RTTI `.?AVTESRegionDataImposter@@`, read at the word before it).
const TES_REGION_DATA_IMPOSTER_VTABLE: u32 = 0x0102_3bc8;
/// `0096a2d0` (unnamed in the engine map): the constructor run on the
/// `BSSimpleList` at +8 (`m_Imposters`, Xbox PDB).
const LIST_CTOR_0096A2D0: u32 = 0x0096_a2d0;

// Translated from 004f1ed0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004f1ed0 - its FS:[0] exception frame (the unwind state that destroys the base when the list constructor throws) is not translated
/// `TESRegionDataImposter` constructor (unnamed in the engine map; the
/// vtable's RTTI names the class): runs the `TESRegionData` constructor,
/// stores the `TESRegionDataImposter` vtable and constructs the list at +8
/// (`0096a2d0`). Returns `this`.
pub fn fn_004f1ed0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(TES_REGION_DATA_CTOR, &args![this]);
    e.mem.set_u32(this.addr(), TES_REGION_DATA_IMPOSTER_VTABLE);
    e.call(LIST_CTOR_0096A2D0, &args![this.addr() + 8]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x004f_1ed0, fn_004f1ed0(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_004f1ed0_runs_the_base_then_the_list_constructor() {
        let mut e = Engine::new();
        e.register(TES_REGION_DATA_CTOR, |_, _| Ret::default());
        e.register(LIST_CTOR_0096A2D0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_1ed0, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this), TES_REGION_DATA_IMPOSTER_VTABLE);
        let calls: Vec<_> = log.into_iter().filter(|c| c.0 != 0x004f_1ed0).collect();
        assert_eq!(
            calls,
            vec![
                (TES_REGION_DATA_CTOR, vec![this]),
                (LIST_CTOR_0096A2D0, vec![this + 8]),
            ]
        );
    }
}
