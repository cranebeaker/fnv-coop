//! `(unplaced)/run_00987820` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00990890` (`thiscall`; `combatgroupstrategy.cpp`): the group's member count, and
/// `00903600` (`thiscall`, one word): member `i`.
const MEMBER_COUNT_00990890: u32 = 0x0099_0890;
const MEMBER_AT_00903600: u32 = 0x0090_3600;
/// `00980a30` (`thiscall`, one word; `combatcontroller.cpp`): run on each member's combat
/// controller with the actor.
const CONTROLLER_CALL_00980A30: u32 = 0x0098_0a30;
/// `CombatGroup::RemoveTarget` (`00986500`, engine map; `thiscall`, one word).
const COMBAT_GROUP_REMOVE_TARGET: u32 = 0x0098_6500;

// Translated from 00987820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a combat group, one word): for each member
/// (`00990890`, `00903600`) with a combat controller (its vtable slot +0x428), runs
/// `00980a30(actor)` on the controller; then, when the actor is 3D-loaded (vtable +0x100),
/// `CombatGroup::RemoveTarget(actor)`.
pub fn fn_00987820(e: &mut Engine, this: Ptr, actor: u32) {
    let count = e.call(MEMBER_COUNT_00990890, &args![this]).u32();
    let mut i = 0u32;
    while i < count {
        let member = e.call(MEMBER_AT_00903600, &args![this, i]).u32();
        let controller = e.vcall(member, 0x428, &[]).u32();
        if controller != 0 {
            e.call(CONTROLLER_CALL_00980A30, &args![controller, actor]);
        }
        i += 1;
    }
    if e.vcall(actor, 0x100, &[]).bool() {
        e.call(COMBAT_GROUP_REMOVE_TARGET, &args![this, actor]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0098_7820, fn_00987820(Ptr, u32))]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = [(u32, Vec<u32>)];

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object whose vtable slot `slot` answers `value`.
    fn object(e: &mut Engine, slot: u32, value: u32) -> u32 {
        let vtable = e.mem.alloc(0x430);
        e.mem.set_u32(vtable + slot, vtable + slot);
        e.register_double(vtable + slot, move |_, _| value.into_ret());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, vtable);
        object
    }

    #[test]
    fn tells_each_controller_then_removes_a_loaded_target() {
        let mut e = Engine::new();
        let with = object(&mut e, 0x428, 0xc0c0);
        let without = object(&mut e, 0x428, 0);
        e.register(MEMBER_COUNT_00990890, |_, _| 2u32.into_ret());
        e.register_double(MEMBER_AT_00903600, move |_, a| {
            if a[1] == 0 {
                without.into_ret()
            } else {
                with.into_ret()
            }
        });
        e.register(CONTROLLER_CALL_00980A30, |_, _| Ret::default());
        e.register(COMBAT_GROUP_REMOVE_TARGET, |_, _| Ret::default());
        let loaded = object(&mut e, 0x100, 1);
        let unloaded = object(&mut e, 0x100, 0);
        e.call_log = Some(vec![]);
        e.call(0x0098_7820, &args![0x6000u32, loaded]);
        e.call(0x0098_7820, &args![0x6000u32, unloaded]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, CONTROLLER_CALL_00980A30),
            vec![vec![0xc0c0, loaded], vec![0xc0c0, unloaded]]
        );
        assert_eq!(
            calls_to(&log, COMBAT_GROUP_REMOVE_TARGET),
            vec![vec![0x6000, loaded]]
        );
    }
}
