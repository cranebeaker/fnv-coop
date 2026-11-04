//! `fallout/magic/run_008255a0` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The magic target's vtable slot +8 (no stack words; `GetActiveEffectList` in the Xbox
/// PDB's `MagicTarget` slot list): the first node of its effect list.
const SLOT_ACTIVE_EFFECT_LIST: u32 = 0x8;
/// List node helpers (`thiscall`, no stack words): the node's item slot (`006815c0`: the
/// node itself) and the next node (`00726070`: the word at +4).
const NODE_ITEM_SLOT: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// `007df210` (`thiscall`): 1 when bit 0x80 of the effect's word at +0x18 (`iFlags`) is
/// set.
const EFFECT_FLAG_80: u32 = 0x007d_f210;
/// `008041a0` (`thiscall`): the effect's word at +0x44 (`pDisplacementSpell`, Xbox PDB).
const EFFECT_WORD_44: u32 = 0x0080_41a0;
/// `008043f0` (`activeeffect.cpp`; `thiscall`, two words: seconds, a byte): re-evaluates
/// the effect's conditions.
const EFFECT_EVALUATE_CONDITIONS: u32 = 0x0080_43f0;

// Translated from 008255a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on a magic target, plain `RET`): walks the effect
/// list (vtable +8) and calls `008043f0` with 0.0 and 1 on every effect whose flag 0x80
/// (`007df210`) is set or whose word at +0x44 (`008041a0`) is not null; null items are
/// skipped. The next node is taken before the call.
pub fn fn_008255a0(e: &mut Engine, this: Ptr) {
    let mut node = e
        .vcall(this.addr(), SLOT_ACTIVE_EFFECT_LIST, &args![])
        .u32();
    while node != 0 {
        let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
        let effect = e.mem.u32(slot);
        node = e.call(NODE_NEXT, &args![node]).u32();
        if effect != 0
            && (e.call(EFFECT_FLAG_80, &args![effect]).bool()
                || e.call(EFFECT_WORD_44, &args![effect]).u32() != 0)
        {
            e.call(EFFECT_EVALUATE_CONDITIONS, &args![effect, 0.0f32, 1u32]);
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0082_55a0, fn_008255a0(Ptr))]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLOT_DOUBLE: u32 = 0x0e60_0000;

    #[test]
    fn re_evaluates_flagged_or_displaced_effects() {
        let mut e = Engine::new();
        // Four nodes: a flagged effect, a null item, a displaced one, a plain one.
        let effects = [0x100u32, 0, 0x300, 0x400];
        let mut next = 0;
        let mut first = 0;
        for &effect in effects.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, effect);
            e.mem.set_u32(node + 4, next);
            next = node;
            first = node;
        }
        let vtable = e.mem.alloc(0x10);
        e.mem.set_u32(vtable + 8, SLOT_DOUBLE);
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, vtable);
        e.register_double(SLOT_DOUBLE, move |_, _| first.into_ret());
        e.register(NODE_ITEM_SLOT, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(EFFECT_FLAG_80, |_, a| u32::from(a[0] == 0x100).into_ret());
        e.register(EFFECT_WORD_44, |_, a| u32::from(a[0] == 0x300).into_ret());
        e.register(EFFECT_EVALUATE_CONDITIONS, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_55a0, &[this]);
        let log = e.call_log.take().unwrap();
        let evaluated: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == EFFECT_EVALUATE_CONDITIONS)
            .map(|(_, w)| w.clone())
            .collect();
        assert_eq!(evaluated, vec![vec![0x100, 0, 1], vec![0x300, 0, 1]]);
        // The flagged effect does not need the second test.
        let second_tests: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == EFFECT_WORD_44)
            .map(|(_, w)| w[0])
            .collect();
        assert_eq!(second_tests, vec![0x300, 0x400]);
    }
}
