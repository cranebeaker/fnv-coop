//! `(unplaced)/run_00824110` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `MiddleHighProcess::GetSavedAcquireObject` (`008d8520`, engine map; `thiscall`): asked
/// of the reference this magic target belongs to.
const PROCESS_OBJECT_008D8520: u32 = 0x008d_8520;
/// `008d6f30` (`thiscall`) and `TESObjectCELL::Get3D` (`00545cb0`, engine map;
/// `thiscall`): the reference's cell and its 3D.
const CELL_008D6F30: u32 = 0x008d_6f30;
const TES_OBJECT_CELL_GET_3D: u32 = 0x0054_5cb0;
/// The effect list's node calls (`thiscall`): `008256d0` (`bool`), `00726070` (the next
/// node) and `006815c0` (the address of the node's item).
const NODE_CHECK_008256D0: u32 = 0x0082_56d0;
const NODE_NEXT_00726070: u32 = 0x0072_6070;
const NODE_ITEM_006815C0: u32 = 0x0068_15c0;
/// The effect calls (`thiscall`): `0044ddc0` (a word compared with `kind`), `00824060`
/// (`bool`), `00804560` (one float), `00705cf0` (`bool`).
const EFFECT_KIND_0044DDC0: u32 = 0x0044_ddc0;
const EFFECT_CHECK_00824060: u32 = 0x0082_4060;
const EFFECT_SET_00804560: u32 = 0x0080_4560;
const EFFECT_CHECK_00705CF0: u32 = 0x0070_5cf0;
/// `MagicTarget::RemoveEffect` (`008251c0`, engine map; `thiscall`, two words).
const MAGIC_TARGET_REMOVE_EFFECT: u32 = 0x0082_51c0;
/// The float `00804560` gets (`0101e2bc`).
const EFFECT_VALUE_0101E2BC: u32 = 0x0101_e2bc;

// Translated from 00824110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a magic target, one word): walks the effect list
/// (vtable +8 gives the first node) and, for each effect whose `0044ddc0` word is `kind`,
/// either calls its slot +0x54 (when `00824060` answers true) or `00804560(0101e2bc)` (and
/// the walk continues from the node after the current one again), then removes it
/// (`MagicTarget::RemoveEffect(effect, 0)`) when `00705cf0` answers true. The walk stops at
/// the first node `008256d0` answers true for. When the target answers true from vtable
/// +0xC, it runs only for an owner (`this - 0x94`, a reference) whose process object
/// (`008d8520`) is set, with a 3D-loaded base (vtable +4, its slot +0x1D0) whose cell
/// (`008d6f30`) has 3D.
pub fn fn_00824110(e: &mut Engine, this: Ptr, kind: u32) {
    let at = this.addr();
    let mut node = e.vcall(at, 0x8, &[]).u32();
    if e.vcall(at, 0xc, &[]).bool() {
        let owner = if at != 0 { at.wrapping_sub(0x94) } else { 0 };
        if owner != 0 && e.call(PROCESS_OBJECT_008D8520, &args![owner]).u32() == 0 {
            return;
        }
        let reference = e.vcall(at, 0x4, &[]).u32();
        if e.vcall(reference, 0x1d0, &[]).u32() == 0 {
            return;
        }
        let reference = e.vcall(at, 0x4, &[]).u32();
        if e.call(CELL_008D6F30, &args![reference]).u32() == 0 {
            return;
        }
        let reference = e.vcall(at, 0x4, &[]).u32();
        let cell = e.call(CELL_008D6F30, &args![reference]).u32();
        if e.call(TES_OBJECT_CELL_GET_3D, &args![cell]).u32() == 0 {
            return;
        }
    }
    while node != 0 {
        if e.call(NODE_CHECK_008256D0, &args![node]).bool() {
            return;
        }
        let mut next = e.call(NODE_NEXT_00726070, &args![node]).u32();
        let item = e.call(NODE_ITEM_006815C0, &args![node]).u32();
        let effect = e.mem.u32(item);
        if effect != 0 && e.call(EFFECT_KIND_0044DDC0, &args![effect]).u32() == kind {
            if !e.call(EFFECT_CHECK_00824060, &args![effect]).bool() {
                let value = x87_quiet_bits(e.mem.u32(EFFECT_VALUE_0101E2BC));
                e.call(EFFECT_SET_00804560, &args![effect, value]);
                next = e.call(NODE_NEXT_00726070, &args![node]).u32();
            } else {
                e.vcall(effect, 0x54, &[]);
            }
            if e.call(EFFECT_CHECK_00705CF0, &args![effect]).bool() {
                e.call(MAGIC_TARGET_REMOVE_EFFECT, &args![at, effect, 0u32]);
            }
        }
        node = next;
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0082_4110, fn_00824110(Ptr, u32))]
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

    /// An object whose vtable slots answer the given words.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x400);
        for &(slot, value) in slots {
            let target = vtable + slot;
            e.mem.set_u32(vtable + slot, target);
            e.register_double(target, move |_, _| value.into_ret());
        }
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object, vtable);
        object
    }

    /// A list of three nodes (a node is two words: the effect, the next node).
    fn list(e: &mut Engine, effects: [u32; 3]) -> u32 {
        let mut next = 0;
        for effect in effects.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *effect);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NODE_CHECK_008256D0, |_, _| Ret::default());
        e.register(NODE_NEXT_00726070, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(NODE_ITEM_006815C0, |_, a| a[0].into_ret());
        e.register(EFFECT_KIND_0044DDC0, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(EFFECT_CHECK_00824060, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(EFFECT_CHECK_00705CF0, |e, a| {
            e.mem.u32(a[0] + 12).into_ret()
        });
        for address in [EFFECT_SET_00804560, MAGIC_TARGET_REMOVE_EFFECT] {
            e.register(address, |_, _| Ret::default());
        }
        e.map(EFFECT_VALUE_0101E2BC & !0xfff, 0x1000);
        e.mem.set_f32(EFFECT_VALUE_0101E2BC, 0.5);
        e
    }

    /// An effect of `kind` (+4) answering `check` (+8) and `remove` (+12), with a slot
    /// +0x54.
    fn effect(e: &mut Engine, kind: u32, check: u32, remove: u32) -> u32 {
        let effect = object(e, &[(0x54, 0)]);
        e.mem.set_u32(effect + 4, kind);
        e.mem.set_u32(effect + 8, check);
        e.mem.set_u32(effect + 12, remove);
        effect
    }

    #[test]
    fn handles_the_effects_of_the_kind() {
        let mut e = engine();
        let first = effect(&mut e, 5, 0, 1);
        let second = effect(&mut e, 6, 0, 1);
        let third = effect(&mut e, 5, 1, 0);
        let head = list(&mut e, [first, second, third]);
        let this = object(&mut e, &[(0x8, head), (0xc, 0)]);
        e.call_log = Some(vec![]);
        e.call(0x0082_4110, &args![this, 5u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, EFFECT_SET_00804560),
            vec![vec![first, 0.5f32.to_bits()]]
        );
        assert_eq!(
            calls_to(&log, MAGIC_TARGET_REMOVE_EFFECT),
            vec![vec![this, first, 0]]
        );
        let slot = e.mem.u32(e.mem.u32(third) + 0x54);
        assert_eq!(calls_to(&log, slot), vec![vec![third]]);
    }

    #[test]
    fn stops_at_a_node_that_answers_true() {
        let mut e = engine();
        let first = effect(&mut e, 5, 0, 1);
        let head = list(&mut e, [first, first, first]);
        let second_node = e.mem.u32(head + 4);
        e.register_double(NODE_CHECK_008256D0, move |_, a| {
            u32::from(a[0] == second_node).into_ret()
        });
        let this = object(&mut e, &[(0x8, head), (0xc, 0)]);
        e.call_log = Some(vec![]);
        e.call(0x0082_4110, &args![this, 5u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, EFFECT_SET_00804560).len(), 1);
    }

    #[test]
    fn needs_a_loaded_owner_when_asked() {
        let mut e = engine();
        let first = effect(&mut e, 5, 0, 0);
        let head = list(&mut e, [first, 0, 0]);
        let reference = object(&mut e, &[(0x1d0, 0x1d00)]);
        let block = e.mem.alloc(0x200);
        let this = block + 0x94;
        let vtable = object(&mut e, &[(0x4, reference), (0x8, head), (0xc, 1)]);
        let table = e.mem.u32(vtable);
        e.mem.set_u32(this, table);
        e.register(PROCESS_OBJECT_008D8520, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_4110, &args![this, 5u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, PROCESS_OBJECT_008D8520), vec![vec![block]]);
        assert!(calls_to(&log, NODE_CHECK_008256D0).is_empty());
        e.register(PROCESS_OBJECT_008D8520, |_, _| 1u32.into_ret());
        e.register(CELL_008D6F30, |_, _| 0xce11u32.into_ret());
        e.register(TES_OBJECT_CELL_GET_3D, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_4110, &args![this, 5u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, TES_OBJECT_CELL_GET_3D), vec![vec![0xce11]]);
        assert!(calls_to(&log, NODE_CHECK_008256D0).is_empty());
        e.register(TES_OBJECT_CELL_GET_3D, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0082_4110, &args![this, 5u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, EFFECT_SET_00804560).len(), 1);
    }
}
