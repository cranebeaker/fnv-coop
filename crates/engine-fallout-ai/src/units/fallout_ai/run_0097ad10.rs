//! `fallout/ai/run_0097ad10` (Xbox PDB source unit), subsystem `fallout/ai`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `CombatActionAttackRanged`'s vtable (`0108cb50`, named by its RTTI
/// locator).
const ATTACK_RANGED_VTABLE: u32 = 0x0108_cb50;
/// The `CombatAction` base constructor (`00979bd0`, `thiscall(kind)`).
const ACTION_CONSTRUCT: u32 = 0x0097_9bd0;
/// Adds a pair to the list at +0x0C (`00984390`, `thiscall(a, b)`).
const ADD_TO_LIST_0C: u32 = 0x0098_4390;
/// Adds a pair to a list (`009aa2b0`, `thiscall(a, b)`).
const LIST_ADD: u32 = 0x009a_a2b0;

// Translated from 0097ad10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0097ad10 - C++ exception unwinding (FS:[0] frame state) is not translated.
/// `CombatActionAttackRanged` constructor (by the vtable it sets,
/// `0108cb50`): the base constructor `00979bd0` with `kind`, the vtable,
/// then the list entries (3, 0) at +0x04, (5, 1), (6, 0), (0xB, 1), (0xA, 1)
/// at +0x0C and (0, 1) at +0x14. Returns `this`.
pub fn fn_0097ad10(e: &mut Engine, this: Ptr, kind: u32) -> Ptr {
    e.call(ACTION_CONSTRUCT, &args![this, kind]);
    e.mem.set_u32(this.addr(), ATTACK_RANGED_VTABLE);
    fn_0097ade0(e, this, 3, 0);
    e.call(ADD_TO_LIST_0C, &args![this, 5u32, 1u32]);
    e.call(ADD_TO_LIST_0C, &args![this, 6u32, 0u32]);
    e.call(ADD_TO_LIST_0C, &args![this, 0xbu32, 1u32]);
    e.call(ADD_TO_LIST_0C, &args![this, 0xau32, 1u32]);
    fn_0097adc0(e, this, 0, 1);
    this
}

// Translated from 0097adc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `(a, b)` to the list at +0x14 (`009aa2b0`; `thiscall`, `RET 8`).
pub fn fn_0097adc0(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    e.call(LIST_ADD, &args![this.addr() + 0x14, a, b]);
}

// Translated from 0097ade0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `(a, b)` to the list at +0x04 (`009aa2b0`; `thiscall`, `RET 8`).
pub fn fn_0097ade0(e: &mut Engine, this: Ptr, a: u32, b: u32) {
    e.call(LIST_ADD, &args![this.addr() + 0x4, a, b]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0097_ad10, fn_0097ad10(Ptr, u32) -> Ptr),
        entry!(0x0097_adc0, fn_0097adc0(Ptr, u32, u32)),
        entry!(0x0097_ade0, fn_0097ade0(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_builds_the_base_and_fills_the_lists() {
        let mut e = Engine::new();
        for addr in [ACTION_CONSTRUCT, ADD_TO_LIST_0C, LIST_ADD] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x30);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0097_ad10, &args![this, 9u32]).u32(), this);
        assert_eq!(e.mem.u32(this), ATTACK_RANGED_VTABLE);
        let log: Vec<_> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x0097_ad10)
            .collect();
        assert_eq!(
            log,
            vec![
                (ACTION_CONSTRUCT, vec![this, 9]),
                (LIST_ADD, vec![this + 4, 3, 0]),
                (ADD_TO_LIST_0C, vec![this, 5, 1]),
                (ADD_TO_LIST_0C, vec![this, 6, 0]),
                (ADD_TO_LIST_0C, vec![this, 0xb, 1]),
                (ADD_TO_LIST_0C, vec![this, 0xa, 1]),
                (LIST_ADD, vec![this + 0x14, 0, 1]),
            ]
        );
    }

    #[test]
    fn list_adders_use_their_offsets() {
        let mut e = Engine::new();
        e.register(LIST_ADD, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0097_adc0, &args![0x100u32, 1u32, 2u32]);
        e.call(0x0097_ade0, &args![0x100u32, 3u32, 4u32]);
        let log: Vec<_> = e
            .call_log
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == LIST_ADD)
            .collect();
        assert_eq!(
            log,
            vec![(LIST_ADD, vec![0x114, 1, 2]), (LIST_ADD, vec![0x104, 3, 4])]
        );
    }
}
