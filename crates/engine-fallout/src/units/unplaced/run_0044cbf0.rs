//! `(unplaced)/run_0044cbf0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0044dd60` (`ECX` = the object held).
const RELEASE_HELD: u32 = 0x0044_dd60;

// Translated from 0044cbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// When the word at `this` holds an object, runs `0044dd60` on it. The
/// engine map gives it no name.
pub fn fn_0044cbf0(e: &mut Engine, this: Ptr) {
    let held = e.mem.u32(this.addr());
    if held != 0 {
        e.call(RELEASE_HELD, &args![held]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0044_cbf0, fn_0044cbf0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_0044cbf0_runs_only_on_a_held_object() {
        let mut e = Engine::new();
        e.register(RELEASE_HELD, |_, _| Ret::default());
        let slot = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        e.call(0x0044_cbf0, &args![slot]);
        e.mem.set_u32(slot, 0x5000);
        e.call(0x0044_cbf0, &args![slot]);
        let hits: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == RELEASE_HELD)
            .collect();
        assert_eq!(hits, vec![(RELEASE_HELD, vec![0x5000])]);
    }
}
