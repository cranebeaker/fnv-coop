//! `fallout shared/bgsidlemarker.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005e7ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSIdleMarker::Save` (engine map): starts the form (`004855a0`), saves
/// the idle collection at +0x30 (`004799f0`), closes the form
/// (`00485680`).
pub fn bgs_idle_marker_save(e: &mut Engine, this: Ptr) {
    e.call(0x0048_55a0, &args![this]);
    e.call(0x0047_99f0, &args![this.addr() + 0x30]);
    e.call(0x0048_5680, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x005e_7ed0, bgs_idle_marker_save(Ptr))]
}

#[cfg(test)]
mod n036_tests {
    use super::*;

    #[test]
    fn save_wraps_the_idle_collection() {
        let mut e = Engine::new();
        for callee in [0x0048_55a0, 0x0047_99f0, 0x0048_5680] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x005e_7ed0, &args![0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (0x0048_55a0, vec![0x5000]),
                (0x0047_99f0, vec![0x5030]),
                (0x0048_5680, vec![0x5000]),
            ]
        );
    }
}
