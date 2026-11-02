//! `(unplaced)/run_0097ce90` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `CombatActionActivateCombatItem`'s vtable (`0108cd74`, named by its RTTI
/// locator).
const ACTIVATE_COMBAT_ITEM_VTABLE: u32 = 0x0108_cd74;
/// The `CombatAction` base constructor (`00979bd0`, `thiscall(kind)`).
const ACTION_CONSTRUCT: u32 = 0x0097_9bd0;
/// Add a pair to the list at +0x0C (`00984390`) and at +0x14 (`0097adc0`),
/// both `thiscall(a, b)`.
const ADD_TO_LIST_0C: u32 = 0x0098_4390;
const ADD_TO_LIST_14: u32 = 0x0097_adc0;

// Translated from 0097ce90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097ce90 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionActivateCombatItem` constructor (by the vtable it sets,
/// `0108cd74`): the base constructor `00979bd0` with `kind`, the vtable,
/// then the list entries (1, -1) at +0x0C and (0xD, -1) at +0x14. Returns
/// `this`.
pub fn fn_0097ce90(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(ACTION_CONSTRUCT, &args![this, kind]);
    e.mem.set_u32(this.addr(), ACTIVATE_COMBAT_ITEM_VTABLE);
    e.call(ADD_TO_LIST_0C, &args![this, 1u32, u32::MAX]);
    e.call(ADD_TO_LIST_14, &args![this, 0xdu32, u32::MAX]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0097_ce90, fn_0097ce90(Ptr, u32) -> Ptr)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_builds_the_base_and_fills_the_lists() {
        let mut e = Engine::new();
        for addr in [ACTION_CONSTRUCT, ADD_TO_LIST_0C, ADD_TO_LIST_14] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0097_ce90, &args![this, 0x1du32]).u32(), this);
        assert_eq!(e.mem.u32(this), ACTIVATE_COMBAT_ITEM_VTABLE);
        let log: Vec<_> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x0097_ce90)
            .collect();
        assert_eq!(
            log,
            vec![
                (ACTION_CONSTRUCT, vec![this, 0x1d]),
                (ADD_TO_LIST_0C, vec![this, 1, u32::MAX]),
                (ADD_TO_LIST_14, vec![this, 0xd, u32::MAX]),
            ]
        );
    }
}
