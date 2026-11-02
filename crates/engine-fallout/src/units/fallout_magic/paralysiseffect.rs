//! `fallout/magic/paralysiseffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0082cd10` (named `ValueModifierEffect::FinishInitLoadGame` in the engine map): the
/// base body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const FINISH_INIT_LOAD_GAME_TARGET_0082CD10: u32 = 0x0082_cd10;

// Translated from 00826550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParalysisEffect::FinishInitLoadGame` (Xbox PDB
/// `?FinishInitLoadGame@ParalysisEffect@@MAAXPAVActor@@@Z`, void, vtable word 0107bfe8):
/// forwards `this` and the `Actor` pointer to the base body `0082cd10`
/// (`ValueModifierEffect::FinishInitLoadGame`). Returns nothing.
pub fn paralysis_effect_finish_init_load_game(e: &mut Engine, this: Ptr, actor: Ptr) {
    e.call(FINISH_INIT_LOAD_GAME_TARGET_0082CD10, &args![this, actor]);
}

/// The `ValueModifierEffect` constructor body `0082ba60` (`thiscall`, three
/// words), `ValueModifierEffect::Start` (`0082c000`), `Update` (`0082c660`,
/// one `float`) and `Finish` (`0082c810`).
const VALUE_MODIFIER_CONSTRUCT: u32 = 0x0082_ba60;
const VALUE_MODIFIER_START: u32 = 0x0082_c000;
const VALUE_MODIFIER_UPDATE: u32 = 0x0082_c660;
const VALUE_MODIFIER_FINISH: u32 = 0x0082_c810;
/// `ParalysisEffect`'s vtable (`0107bfbc`, RTTI `.?AVParalysisEffect@@`).
const PARALYSIS_EFFECT_VTABLE: u32 = 0x0107_bfbc;
/// `operator new` (`00401000`, `cdecl`: the size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `0059bb30` (`thiscall`): the effect's magic target;
/// `MagicTarget::GetTargetAsActor` (engine map, `00822b40`).
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
const GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `MobileObject::StopCurrentDialogue` (engine map, `00934250`, `thiscall`).
const STOP_CURRENT_DIALOGUE: u32 = 0x0093_4250;
/// `ActiveEffect::Dispel` (engine map, `00804210`, `thiscall`, one word).
const DISPEL: u32 = 0x0080_4210;
/// `004534f0` (`thiscall`, one word; does nothing), `0043fcd0`
/// (`thiscall` on the reference: its rigid body) and
/// `bhkRigidBody::SetFixed` (engine map, `00c8f210`, `cdecl`, three words).
const REVERT_BASE_004534F0: u32 = 0x0045_34f0;
const RIGID_BODY_0043FCD0: u32 = 0x0043_fcd0;
const SET_FIXED: u32 = 0x00c8_f210;

/// The effect's target as an actor (`0059bb30`, then
/// `MagicTarget::GetTargetAsActor`), or 0.
fn target_actor(e: &mut Engine, this: Ptr) -> u32 {
    if e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() == 0 {
        return 0;
    }
    let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
    e.call(GET_TARGET_AS_ACTOR, &args![target]).u32()
}

// Translated from 008262e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParalysisEffect`'s constructor (`thiscall`, `RET 0xC`: caster, magic
/// item, effect item): the `ValueModifierEffect` constructor (`0082ba60`),
/// then `ParalysisEffect`'s vtable. Returns `this`.
pub fn fn_008262e0(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(
        VALUE_MODIFIER_CONSTRUCT,
        &args![this, caster, spell, effect],
    );
    e.mem.set_u32(this.addr(), PARALYSIS_EFFECT_VTABLE);
    this
}

// Translated from 00826310 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParalysisEffect::Copy` (engine map; `thiscall`): a new 0x4C-byte
/// `ParalysisEffect` (`00401000`, then `008262e0` with the caster +0x28,
/// magic item +8 and effect item +0xC, or 0 when the allocation fails),
/// handed to `this`'s slot +0x44 (`CopyData`). Returns the copy.
// Translation incomplete: 00826310 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
pub fn paralysis_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    let copy = if block != 0 {
        let base = this.addr();
        let caster = e.mem.u32(base.wrapping_add(0x28));
        let spell = e.mem.u32(base.wrapping_add(8));
        let effect = e.mem.u32(base.wrapping_add(0xc));
        fn_008262e0(e, Ptr::new(block), caster, spell, effect).addr()
    } else {
        0
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

// Translated from 008263c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParalysisEffect::Start` (engine map; `thiscall`): with a target actor,
/// stops its dialogue (`MobileObject::StopCurrentDialogue`), calls its
/// slot +0x234 (the answer is not used) and `ValueModifierEffect::Start`.
pub fn paralysis_effect_start(e: &mut Engine, this: Ptr) {
    let actor = target_actor(e, this);
    if actor == 0 {
        return;
    }
    e.call(STOP_CURRENT_DIALOGUE, &args![actor]);
    e.vcall(actor, 0x234, &[]);
    e.call(VALUE_MODIFIER_START, &args![this]);
}

// Translated from 00826430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParalysisEffect::Update` (engine map; `thiscall`, `RET 4`: the elapsed
/// seconds): `ValueModifierEffect::Update` with the seconds, then the
/// effect is dispelled (with 1) when the target actor's slot +0x22C (with
/// 0) answers true.
pub fn paralysis_effect_update(e: &mut Engine, this: Ptr, seconds: f32) {
    let seconds = x87_quiet_bits(seconds.to_bits());
    e.call(VALUE_MODIFIER_UPDATE, &args![this, seconds]);
    let actor = target_actor(e, this);
    if actor != 0 && e.vcall(actor, 0x22c, &args![0u32]).bool() {
        e.call(DISPEL, &args![this, 1u32]);
    }
}

// Translated from 008264b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParalysisEffect::Finish` (engine map; `thiscall`):
/// `ValueModifierEffect::Finish`.
pub fn paralysis_effect_finish(e: &mut Engine, this: Ptr) {
    e.call(VALUE_MODIFIER_FINISH, &args![this]);
}

// Translated from 008264d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`cdecl`, three words: caster, magic item, effect item): a new
/// 0x4C-byte `ParalysisEffect` (`00401000`, then `008262e0`), or 0 when the
/// allocation fails.
// Translation incomplete: 008264d0 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
pub fn fn_008264d0(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    if block != 0 {
        return fn_008262e0(e, Ptr::new(block), caster, spell, effect).addr();
    }
    0
}

// Translated from 00826570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ParalysisEffect::Revert` (engine map; `thiscall`, `RET 4`: the
/// reference): `004534f0`, then, when the reference has a rigid body
/// (`0043fcd0`), `bhkRigidBody::SetFixed(body, 0, 1)`.
pub fn paralysis_effect_revert(e: &mut Engine, this: Ptr, reference: u32) {
    e.call(REVERT_BASE_004534F0, &args![this, reference]);
    let body = e.call(RIGID_BODY_0043FCD0, &args![reference]).u32();
    if body != 0 {
        e.call(SET_FIXED, &args![body, 0u32, 1u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0082_6550,
            paralysis_effect_finish_init_load_game(Ptr, Ptr)
        ),
        entry!(0x0082_62e0, fn_008262e0(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0082_6310, paralysis_effect_copy(Ptr) -> u32),
        entry!(0x0082_63c0, paralysis_effect_start(Ptr)),
        entry!(0x0082_6430, paralysis_effect_update(Ptr, f32)),
        entry!(0x0082_64b0, paralysis_effect_finish(Ptr)),
        entry!(0x0082_64d0, fn_008264d0(u32, u32, u32) -> u32),
        entry!(0x0082_6570, paralysis_effect_revert(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_paralysis_effect_finish_init_load_game_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(FINISH_INIT_LOAD_GAME_TARGET_0082CD10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_6550, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == FINISH_INIT_LOAD_GAME_TARGET_0082CD10)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod tests_n048 {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn zero(e: &mut Engine, addrs: &[u32]) {
        for addr in addrs {
            e.register(*addr, |_, _| Ret::default());
        }
    }

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let r = e.call(addr, words);
        (r, e.call_log.take().unwrap())
    }

    /// A target actor whose slots +0x234 and +0x22C go to `0x0300_0234` and
    /// `0x0300_022c` (answering `knocked`).
    fn actor(e: &mut Engine, knocked: u32) -> u32 {
        let actor = e.mem.alloc(4);
        let vtable = e.mem.alloc(0x238);
        e.mem.set_u32(vtable + 0x234, 0x0300_0234);
        e.mem.set_u32(vtable + 0x22c, 0x0300_022c);
        e.mem.set_u32(actor, vtable);
        e.register(0x0300_0234, |_, _| Ret::default());
        e.register_double(0x0300_022c, move |_, _| ret(knocked));
        e.register(EFFECT_TARGET_0059BB30, |_, _| ret(0x7a7a));
        e.register_double(GET_TARGET_AS_ACTOR, move |_, _| ret(actor));
        actor
    }

    #[test]
    fn constructor_copy_and_maker() {
        let mut e = Engine::new();
        zero(&mut e, &[VALUE_MODIFIER_CONSTRUCT, 0x0300_0044]);
        let block = e.mem.alloc(0x4c);
        e.register_double(OPERATOR_NEW, move |_, _| ret(block));
        let this = e.mem.alloc(0x4c);
        let vtable = e.mem.alloc(0x48);
        e.mem.set_u32(vtable + 0x44, 0x0300_0044);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x28, 1);
        e.mem.set_u32(this + 8, 2);
        e.mem.set_u32(this + 0xc, 3);
        let (r, log) = run(&mut e, 0x0082_6310, &args![this]);
        assert_eq!(r.u32(), block);
        assert_eq!(e.mem.u32(block), PARALYSIS_EFFECT_VTABLE);
        assert_eq!(calls(&log, OPERATOR_NEW), vec![vec![0x4c]]);
        assert_eq!(
            calls(&log, VALUE_MODIFIER_CONSTRUCT),
            vec![vec![block, 1, 2, 3]]
        );
        assert_eq!(calls(&log, 0x0300_0044), vec![vec![this, block]]);
        let (r, log) = run(&mut e, 0x0082_64d0, &args![4u32, 5u32, 6u32]);
        assert_eq!(r.u32(), block);
        assert_eq!(
            calls(&log, VALUE_MODIFIER_CONSTRUCT),
            vec![vec![block, 4, 5, 6]]
        );
        e.register(OPERATOR_NEW, |_, _| ret(0));
        assert_eq!(e.call(0x0082_64d0, &args![4u32, 5u32, 6u32]).u32(), 0);
        let (r, log) = run(&mut e, 0x0082_6310, &args![this]);
        assert_eq!(r.u32(), 0);
        assert_eq!(calls(&log, 0x0300_0044), vec![vec![this, 0]]);
    }

    #[test]
    fn start_needs_a_target_actor() {
        let mut e = Engine::new();
        zero(&mut e, &[STOP_CURRENT_DIALOGUE, VALUE_MODIFIER_START]);
        let target = actor(&mut e, 0);
        let (_, log) = run(&mut e, 0x0082_63c0, &args![0x40u32]);
        assert_eq!(calls(&log, GET_TARGET_AS_ACTOR), vec![vec![0x7a7a]]);
        assert_eq!(calls(&log, STOP_CURRENT_DIALOGUE), vec![vec![target]]);
        assert_eq!(calls(&log, 0x0300_0234), vec![vec![target]]);
        assert_eq!(calls(&log, VALUE_MODIFIER_START), vec![vec![0x40]]);
        e.register(EFFECT_TARGET_0059BB30, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0082_63c0, &args![0x40u32]);
        assert!(calls(&log, VALUE_MODIFIER_START).is_empty());
        assert!(calls(&log, GET_TARGET_AS_ACTOR).is_empty());
    }

    #[test]
    fn update_dispels_a_knocked_out_target() {
        let mut e = Engine::new();
        zero(&mut e, &[VALUE_MODIFIER_UPDATE, DISPEL]);
        actor(&mut e, 0);
        let (_, log) = run(&mut e, 0x0082_6430, &args![0x40u32, 0.25f32.to_bits()]);
        assert_eq!(
            calls(&log, VALUE_MODIFIER_UPDATE),
            vec![vec![0x40, 0.25f32.to_bits()]]
        );
        assert!(calls(&log, DISPEL).is_empty());
        let target = actor(&mut e, 0x101);
        let (_, log) = run(&mut e, 0x0082_6430, &args![0x40u32, 0.25f32.to_bits()]);
        assert_eq!(calls(&log, 0x0300_022c), vec![vec![target, 0]]);
        assert_eq!(calls(&log, DISPEL), vec![vec![0x40, 1]]);
    }

    #[test]
    fn finish_and_revert() {
        let mut e = Engine::new();
        zero(
            &mut e,
            &[VALUE_MODIFIER_FINISH, REVERT_BASE_004534F0, SET_FIXED],
        );
        let (_, log) = run(&mut e, 0x0082_64b0, &args![0x40u32]);
        assert_eq!(calls(&log, VALUE_MODIFIER_FINISH), vec![vec![0x40]]);
        e.register(RIGID_BODY_0043FCD0, |_, _| ret(0));
        let (_, log) = run(&mut e, 0x0082_6570, &args![0x40u32, 0x50u32]);
        assert_eq!(calls(&log, REVERT_BASE_004534F0), vec![vec![0x40, 0x50]]);
        assert_eq!(calls(&log, RIGID_BODY_0043FCD0), vec![vec![0x50]]);
        assert!(calls(&log, SET_FIXED).is_empty());
        e.register(RIGID_BODY_0043FCD0, |_, _| ret(0xb0d));
        let (_, log) = run(&mut e, 0x0082_6570, &args![0x40u32, 0x50u32]);
        assert_eq!(calls(&log, SET_FIXED), vec![vec![0xb0d, 0, 1]]);
    }
}
