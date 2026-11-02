//! `fallout shared/havok/run_00621b60` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The array's own destructor body (`00621c60`; it calls `00621c90` with a
/// zero flag byte). `cameracaster.rs` calls the function below as the
/// destructor of the collector's hit array.
const ARRAY_DESTRUCTOR: u32 = 0x0062_1c60;
/// The empty base destructor (`00483710`: it only keeps `this` in a local).
const BASE_DESTRUCTOR: u32 = 0x0048_3710;

// Translated from 00621b60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00621b60 - the FS:[0] exception frame (state 0 while the array body runs) is not translated
/// The destructor of the hit array that `hkpAllCdPointCollector` holds at
/// `+0x10`: the array's body (`00621c60`), then the base (`00483710`). The
/// engine map names this address `CAtlWinModule::~CAtlWinModule` because the
/// linker folded identical code; the body is the one described here.
pub fn fn_00621b60(e: &mut Engine, this: Ptr) {
    e.call(ARRAY_DESTRUCTOR, &args![this]);
    e.call(BASE_DESTRUCTOR, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0062_1b60, fn_00621b60(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn body_runs_before_the_base() {
        let mut e = Engine::new();
        e.register(ARRAY_DESTRUCTOR, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_1b60, &args![0x4400u32]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .into_iter()
            .filter(|(a, _)| *a == ARRAY_DESTRUCTOR || *a == BASE_DESTRUCTOR)
            .collect();
        assert_eq!(
            calls,
            vec![
                (ARRAY_DESTRUCTOR, vec![0x4400]),
                (BASE_DESTRUCTOR, vec![0x4400])
            ]
        );
    }
}
