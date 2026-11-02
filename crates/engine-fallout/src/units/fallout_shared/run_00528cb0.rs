//! `fallout shared/run_00528cb0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00528cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `target` in the pointer slot `this` and, when it is not null, runs `0092c870` on it
/// (`ECX` = the target). Returns `this`. The engine map gives it no name, so the class is
/// unconfirmed.
pub fn fn_00528cb0(e: &mut Engine, this: Ptr, target: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), target.addr());
    if e.mem.u32(this.addr()) != 0 {
        let stored = e.mem.u32(this.addr());
        e.call(0x0092_c870, &args![stored]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0052_8cb0, fn_00528cb0(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;

    #[test]
    fn fn_00528cb0_stores_the_target_and_counts_it() {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        e.register(0x0092_c870, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let result = e.call(
            0x0052_8cb0,
            &args![Ptr::<()>::new(slot), Ptr::<()>::new(0x4321)],
        );
        assert_eq!(result.u32(), slot);
        assert_eq!(e.mem.u32(slot), 0x4321);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1..], [(0x0092_c870, vec![0x4321])]);
    }

    #[test]
    fn fn_00528cb0_leaves_a_null_target_uncounted() {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0xdead);
        e.call_log = Some(vec![]);
        let result = e.call(0x0052_8cb0, &args![Ptr::<()>::new(slot), Ptr::<()>::new(0)]);
        assert_eq!(result.u32(), slot);
        assert_eq!(e.mem.u32(slot), 0);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
