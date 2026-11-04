//! `fallout shared/pathfinding/run_006c7530` (Xbox PDB source unit), subsystem `fallout shared/pathfinding`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The vtable `006c7530` writes (`0106c708`) and the `thiscall` set-up it
/// calls with two zero words (`006b3eb0`, `RET 8`; the engine map files it
/// under `pathsmootherpov.cpp`).
const VTABLE_006C7530: u32 = 0x0106_c708;
const SET_UP: u32 = 0x006b_3eb0;

// Translated from 006c7530 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor (the engine map has no name): the vtable `0106c708`, then
/// `006b3eb0` with two zero words. Answers `this`.
pub fn fn_006c7530(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_006C7530);
    e.call(SET_UP, &args![this, 0u32, 0u32]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x006c_7530, fn_006c7530(Ptr) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_sets_the_vtable_then_sets_up() {
        let mut e = Engine::new();
        e.register(SET_UP, |e, a| {
            assert_eq!(e.mem.u32(a[0]), VTABLE_006C7530);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x10);
        assert_eq!(e.call(0x006c_7530, &args![this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (SET_UP, vec![this, 0, 0]));
    }
}
