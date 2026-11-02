//! `fallout/magic/turnundeadeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `ActiveEffect::CheckDeletedCaster` (engine map), thiscall (effect, reference).
const ACTIVE_EFFECT_CHECK_DELETED_CASTER: u32 = 0x0080_6200;
/// `ActiveEffect::Dispel` (engine map), thiscall (effect, flag).
const ACTIVE_EFFECT_DISPEL: u32 = 0x0080_4210;

// Translated from 0080ba50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TurnUndeadEffect::CheckDeletedCaster` (Xbox PDB), thiscall
/// `(reference)`: runs `ActiveEffect::CheckDeletedCaster` and, when it
/// answers true, dispels the effect (`ActiveEffect::Dispel(1)`); answers
/// the base's answer.
pub fn turn_undead_effect_check_deleted_caster(e: &mut Engine, this: Ptr, reference: u32) -> bool {
    let deleted = e
        .call(ACTIVE_EFFECT_CHECK_DELETED_CASTER, &args![this, reference])
        .u8();
    if deleted != 0 {
        e.call(ACTIVE_EFFECT_DISPEL, &args![this, 1u32]);
    }
    deleted != 0
}

/// `bTurned` (Xbox PDB): the byte at +0x48 of a `TurnUndeadEffect`.
const TURNED: u32 = 0x48;
/// The `ActiveEffect` constructor (`00803d30`), thiscall (caster, spell,
/// effect item), `RET 0xc`.
const ACTIVE_EFFECT_CONSTRUCT: u32 = 0x0080_3d30;
/// `TurnUndeadEffect`'s vtable (`0107c53c`).
const TURN_UNDEAD_EFFECT_VTABLE: u32 = 0x0107_c53c;
/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `ActiveEffect`'s virtual slot +0x44 (`CopyData`, Xbox PDB order).
const SLOT_COPY_DATA: u32 = 0x44;
/// `MagicTarget::GetTargetAsActor` (`00822b40`) and
/// `MagicCaster::GetCasterAsActor` (`00815410`) (engine map), thiscall.
const TARGET_AS_ACTOR: u32 = 0x0082_2b40;
const CASTER_AS_ACTOR: u32 = 0x0081_5410;
/// The effect's target (`0059bb30`) and caster (`0045cd60`), thiscall on
/// the effect (folded code: the words at +0x24 and +0x28).
const EFFECT_TARGET: u32 = 0x0059_bb30;
const EFFECT_CASTER: u32 = 0x0045_cd60;
/// The effect's magnitude (`00495460`, a float in `ST0`; folded code).
const EFFECT_MAGNITUDE: u32 = 0x0049_5460;
/// The actor's level (`0087f9f0`, `AX`).
const ACTOR_LEVEL: u32 = 0x0087_f9f0;
/// `MagicFormulas::CheckTargetLevelMagnitude` (engine map), cdecl (level,
/// magnitude).
const CHECK_TARGET_LEVEL_MAGNITUDE: u32 = 0x0064_94d0;
/// `__RTDynamicCast` (CRT), cdecl, and the type descriptors of
/// `MagicTarget` and `Creature`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_MAGIC_TARGET: u32 = 0x011a_0ef8;
const TYPE_CREATURE: u32 = 0x011a_106c;
/// Thiscall on the actor: `AL` (`00608d80`, `tesnpc.cpp`).
const ACTOR_TEST_00608D80: u32 = 0x0060_8d80;
/// The target actor's virtual slots `CheckTarget` asks (+0x22c with 0,
/// +0x2e8), `Start` and `Finish` use (+0x428, a word; +0x434 with the
/// caster).
const SLOT_ACTOR_22C: u32 = 0x22c;
const SLOT_ACTOR_2E8: u32 = 0x2e8;
const SLOT_ACTOR_428: u32 = 0x428;
const SLOT_ACTOR_434: u32 = 0x434;
/// Thiscall on what slot +0x428 returns (caster, word), `RET 8`
/// (`008d0600`; folded code named `ProcessLists::PrintLists`).
const WORD_CALL_008D0600: u32 = 0x008d_0600;
/// Thiscall on the actor: `AL` (`00493bb0`, `animation.cpp`).
const ACTOR_TEST_00493BB0: u32 = 0x0049_3bb0;
/// Thiscall on the actor: a word (`008d8520`, folded code), whose virtual
/// slot +0x33c `Finish` calls.
const ACTOR_MEMBER_008D8520: u32 = 0x008d_8520;
const SLOT_33C: u32 = 0x33c;
/// `ActiveEffect::SaveGame` (`008050d0`) and `ActiveEffect::LoadGame`
/// (`00805330`) (engine map), thiscall (word), `RET 4`.
const ACTIVE_EFFECT_SAVE_GAME: u32 = 0x0080_50d0;
const ACTIVE_EFFECT_LOAD_GAME: u32 = 0x0080_5330;
/// The save/load singleton (pointer global) and its block write
/// (`008579b0`) and read (`008579e0`), thiscall (address, size), `RET 8`.
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const SAVE_LOAD_WRITE_008579B0: u32 = 0x0085_79b0;
const SAVE_LOAD_READ_008579E0: u32 = 0x0085_79e0;

// Translated from 0082b370 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TurnUndeadEffect` constructor (unnamed in the engine map;
/// `thiscall`, `RET 0xc`): the `ActiveEffect` constructor with the caster,
/// the spell and the effect item, the class's vtable (`0107c53c`),
/// `bTurned` cleared. Returns `this`.
pub fn fn_0082b370(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(ACTIVE_EFFECT_CONSTRUCT, &args![this, caster, spell, effect]);
    e.mem.set_u32(this.addr(), TURN_UNDEAD_EFFECT_VTABLE);
    e.mem.set_u8(this.addr() + TURNED, 0);
    this
}

// Translated from 0082b3b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082b3b0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the new effect) is not translated.
/// `TurnUndeadEffect::Copy` (engine map; `thiscall`, plain `RET`): a new
/// effect (0x4c bytes, `0082b370` with this effect's caster (+0x28), spell
/// (+8) and effect item (+0xc); 0 when the allocation failed) that gets
/// this effect's `bTurned` and that this effect's `CopyData` (virtual slot
/// +0x44) fills. Returns it. (A failed allocation writes `bTurned` at
/// address 0x48, as the exe does.)
pub fn turn_undead_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let memory = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    let copy = if memory == 0 {
        0
    } else {
        let caster = e.mem.u32(this.addr() + 0x28);
        let spell = e.mem.u32(this.addr() + 8);
        let effect = e.mem.u32(this.addr() + 0xc);
        fn_0082b370(e, Ptr::new(memory), caster, spell, effect).addr()
    };
    let turned = e.mem.u8(this.addr() + TURNED);
    e.mem.set_u8(copy.wrapping_add(TURNED), turned);
    e.vcall(this.addr(), SLOT_COPY_DATA, &args![copy]);
    copy
}

// Translated from 0082b460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TurnUndeadEffect::CheckTarget` (engine map; `thiscall`, `RET 4`):
/// whether `target` can be turned: it must be an actor
/// (`MagicTarget::GetTargetAsActor`) for which the virtual slots +0x22c
/// (with 0) and +0x2e8 answer false, whose level (`0087f9f0`) passes
/// `MagicFormulas::CheckTargetLevelMagnitude` with the effect's magnitude,
/// that is not a `Creature` (`__RTDynamicCast` of `target`), and for which
/// `00608d80` answers true.
pub fn turn_undead_effect_check_target(e: &mut Engine, this: Ptr, target: u32) -> bool {
    let actor = if target != 0 {
        e.call(TARGET_AS_ACTOR, &args![target]).u32()
    } else {
        0
    };
    if actor == 0 {
        return false;
    }
    if e.vcall(actor, SLOT_ACTOR_22C, &args![0u32]).bool() {
        return false;
    }
    if e.vcall(actor, SLOT_ACTOR_2E8, &args![]).bool() {
        return false;
    }
    let magnitude = x87_store32(e.call(EFFECT_MAGNITUDE, &args![this]).f64());
    let level = u32::from(e.call(ACTOR_LEVEL, &args![actor]).u16());
    if !e
        .call(CHECK_TARGET_LEVEL_MAGNITUDE, &args![level, magnitude])
        .bool()
    {
        return false;
    }
    let creature = e
        .call(
            RT_DYNAMIC_CAST,
            &args![target, 0u32, TYPE_MAGIC_TARGET, TYPE_CREATURE, 0u32],
        )
        .u32();
    if creature != 0 {
        return false;
    }
    e.call(ACTOR_TEST_00608D80, &args![actor]).bool()
}

/// The target and caster actors of an effect, each 0 when the effect has
/// none (`0059bb30`/`00822b40` and `0045cd60`/`00815410`, the getters
/// called twice as the exe does).
fn target_and_caster(e: &mut Engine, this: Ptr) -> (u32, u32) {
    let target = if e.call(EFFECT_TARGET, &args![this]).u32() != 0 {
        let target = e.call(EFFECT_TARGET, &args![this]).u32();
        e.call(TARGET_AS_ACTOR, &args![target]).u32()
    } else {
        0
    };
    let caster = if e.call(EFFECT_CASTER, &args![this]).u32() != 0 {
        let caster = e.call(EFFECT_CASTER, &args![this]).u32();
        e.call(CASTER_AS_ACTOR, &args![caster]).u32()
    } else {
        0
    };
    (target, caster)
}

// Translated from 0082b530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TurnUndeadEffect::Start` (engine map; `thiscall`, plain `RET`): with a
/// target and a caster actor and a non-null word from the target's virtual
/// slot +0x428, `008d0600` runs on that word with the caster and 0, and
/// `bTurned` is set.
pub fn turn_undead_effect_start(e: &mut Engine, this: Ptr) {
    let (target, caster) = target_and_caster(e, this);
    if target == 0 || caster == 0 {
        return;
    }
    if e.vcall(target, SLOT_ACTOR_428, &args![]).u32() == 0 {
        return;
    }
    let word = e.vcall(target, SLOT_ACTOR_428, &args![]).u32();
    e.call(WORD_CALL_008D0600, &args![word, caster, 0u32]);
    e.mem.set_u8(this.addr() + TURNED, 1);
}

// Translated from 0082b5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TurnUndeadEffect::Finish` (engine map; `thiscall`, plain `RET`): only
/// for a turned effect with a target actor for which `00493bb0` answers
/// true: the target's virtual slot +0x434 runs with the caster actor (or
/// 0); then, with a caster and a non-null `008d8520` word of the target,
/// that word's virtual slot +0x33c runs with the target, the caster, 1,
/// eight zeros, 1 and 0.
pub fn turn_undead_effect_finish(e: &mut Engine, this: Ptr) {
    if e.mem.u8(this.addr() + TURNED) == 0 {
        return;
    }
    let (target, caster) = target_and_caster(e, this);
    if target == 0 || !e.call(ACTOR_TEST_00493BB0, &args![target]).bool() {
        return;
    }
    e.vcall(target, SLOT_ACTOR_434, &args![caster]);
    if caster == 0 || e.call(ACTOR_MEMBER_008D8520, &args![target]).u32() == 0 {
        return;
    }
    let word = e.call(ACTOR_MEMBER_008D8520, &args![target]).u32();
    e.vcall(
        word,
        SLOT_33C,
        &args![target, caster, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32],
    );
}

// Translated from 0082b6d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082b6d0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the new effect) is not translated.
/// Unnamed in the engine map (cdecl, three words): a new
/// `TurnUndeadEffect` (0x4c bytes, `0082b370` with the caster, the spell
/// and the effect item), or 0 when the allocation failed.
pub fn fn_0082b6d0(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let memory = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    if memory == 0 {
        return 0;
    }
    fn_0082b370(e, Ptr::new(memory), caster, spell, effect).addr()
}

// Translated from 0082b750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TurnUndeadEffect::SaveGame` (engine map; `thiscall`, `RET 4`):
/// `ActiveEffect::SaveGame`, then `bTurned` (1 byte) written through the
/// save/load singleton (`008579b0`).
pub fn turn_undead_effect_save_game(e: &mut Engine, this: Ptr, word: u32) {
    e.call(ACTIVE_EFFECT_SAVE_GAME, &args![this, word]);
    let game = e.mem.u32(SAVE_LOAD_GAME);
    e.call(
        SAVE_LOAD_WRITE_008579B0,
        &args![game, this.addr() + TURNED, 1u32],
    );
}

// Translated from 0082b780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TurnUndeadEffect::LoadGame` (engine map; `thiscall`, `RET 4`):
/// `ActiveEffect::LoadGame`, then `bTurned` (1 byte) read through the
/// save/load singleton (`008579e0`).
pub fn turn_undead_effect_load_game(e: &mut Engine, this: Ptr, word: u32) {
    e.call(ACTIVE_EFFECT_LOAD_GAME, &args![this, word]);
    let game = e.mem.u32(SAVE_LOAD_GAME);
    e.call(
        SAVE_LOAD_READ_008579E0,
        &args![game, this.addr() + TURNED, 1u32],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0080ba50,
            turn_undead_effect_check_deleted_caster(Ptr, u32) -> bool
        ),
        entry!(0x0082_b370, fn_0082b370(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0082_b3b0, turn_undead_effect_copy(Ptr) -> u32),
        entry!(0x0082_b460, turn_undead_effect_check_target(Ptr, u32) -> bool),
        entry!(0x0082_b530, turn_undead_effect_start(Ptr)),
        entry!(0x0082_b5e0, turn_undead_effect_finish(Ptr)),
        entry!(0x0082_b6d0, fn_0082b6d0(u32, u32, u32) -> u32),
        entry!(0x0082_b750, turn_undead_effect_save_game(Ptr, u32)),
        entry!(0x0082_b780, turn_undead_effect_load_game(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_deleted_caster_dispels_the_effect() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_CHECK_DELETED_CASTER, |_, a| Ret {
            eax: u32::from(a[1] == 7),
            ..Ret::default()
        });
        e.register(ACTIVE_EFFECT_DISPEL, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0080_ba50, &args![0x100u32, 7u32]).bool());
        assert!(!e.call(0x0080_ba50, &args![0x100u32, 8u32]).bool());
        let log = e.call_log.take().unwrap();
        let dispels: Vec<_> = log.iter().filter(|c| c.0 == ACTIVE_EFFECT_DISPEL).collect();
        assert_eq!(dispels.len(), 1);
        assert_eq!(dispels[0].1, vec![0x100, 1]);
    }
}

#[cfg(test)]
mod turn_undead_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const ACTOR_SLOT_BASE: u32 = 0x00f6_0000;

    /// An actor whose vtable slots point at doubles `ACTOR_SLOT_BASE + slot`:
    /// +0x22c answers the byte at +0x10, +0x2e8 the byte at +0x11, +0x428
    /// the word at +0x14; +0x434 does nothing.
    fn actor(e: &mut Engine) -> u32 {
        let actor = e.mem.alloc(0x20);
        let vtable = e.mem.alloc(0x440);
        for slot in [
            SLOT_ACTOR_22C,
            SLOT_ACTOR_2E8,
            SLOT_ACTOR_428,
            SLOT_ACTOR_434,
        ] {
            e.mem.set_u32(vtable + slot, ACTOR_SLOT_BASE + slot);
        }
        e.mem.set_u32(actor, vtable);
        actor
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ACTOR_SLOT_BASE + SLOT_ACTOR_22C, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x10)).into_ret()
        });
        e.register(ACTOR_SLOT_BASE + SLOT_ACTOR_2E8, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x11)).into_ret()
        });
        e.register(ACTOR_SLOT_BASE + SLOT_ACTOR_428, |e, a| {
            e.mem.u32(a[0] + 0x14).into_ret()
        });
        e.register(ACTOR_SLOT_BASE + SLOT_ACTOR_434, |_, _| Ret::default());
        // Target/caster: the effect's words +0x24/+0x28; as actors: the
        // word at +0x18 of those.
        e.register(EFFECT_TARGET, |e, a| e.mem.u32(a[0] + 0x24).into_ret());
        e.register(EFFECT_CASTER, |e, a| e.mem.u32(a[0] + 0x28).into_ret());
        e.register(TARGET_AS_ACTOR, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(CASTER_AS_ACTOR, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e
    }

    /// A magic target or caster object whose actor is `actor`.
    fn holder(e: &mut Engine, actor: u32) -> u32 {
        let holder = e.mem.alloc(0x20);
        e.mem.set_u32(holder + 0x18, actor);
        holder
    }

    #[test]
    fn constructor_sets_vtable_and_clears_turned() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0x4c);
        e.mem.set_u8(this + TURNED, 1);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0082_b370, &args![this, 1u32, 2u32, 3u32]).u32(),
            this
        );
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_CONSTRUCT),
            vec![vec![this, 1, 2, 3]]
        );
        assert_eq!(e.mem.u32(this), TURN_UNDEAD_EFFECT_VTABLE);
        assert_eq!(e.mem.u8(this + TURNED), 0);
    }

    #[test]
    fn copy_carries_turned_and_fills_the_copy() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_CONSTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.mem.alloc(0x4c);
        let vtable = e.mem.alloc(0x48);
        e.mem.set_u32(vtable + 0x44, 0x00f7_0000);
        e.register(0x00f7_0000, |_, _| Ret::default());
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x28, 0xa);
        e.mem.set_u32(this + 8, 0xb);
        e.mem.set_u32(this + 0xc, 0xc);
        e.mem.set_u8(this + TURNED, 1);
        e.call_log = Some(vec![]);
        let copy = e.call(0x0082_b3b0, &args![this]).u32();
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x4c]]);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_CONSTRUCT),
            vec![vec![copy, 0xa, 0xb, 0xc]]
        );
        assert_eq!(e.mem.u8(copy + TURNED), 1);
        assert_eq!(calls(&e, 0x00f7_0000), vec![vec![this, copy]]);
    }

    #[test]
    fn check_target_needs_a_turnable_actor() {
        let mut e = engine();
        e.register(EFFECT_MAGNITUDE, |_, _| Ret {
            st0: 25.0,
            ..Ret::default()
        });
        e.register(ACTOR_LEVEL, |_, _| 0xffff_0007u32.into_ret());
        e.register(CHECK_TARGET_LEVEL_MAGNITUDE, |_, _| 1u32.into_ret());
        e.register(RT_DYNAMIC_CAST, |e, a| e.mem.u32(a[0] + 0x1c).into_ret());
        e.register(ACTOR_TEST_00608D80, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x12)).into_ret()
        });
        let this = Ptr::new(e.mem.alloc(0x4c));
        let a = actor(&mut e);
        let target = holder(&mut e, a);
        e.mem.set_u8(a + 0x12, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0082_b460, &args![this, target]).bool());
        assert_eq!(
            calls(&e, CHECK_TARGET_LEVEL_MAGNITUDE),
            vec![vec![7, 25.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, ACTOR_SLOT_BASE + SLOT_ACTOR_22C),
            vec![vec![a, 0]]
        );
        assert!(!turn_undead_effect_check_target(&mut e, this, 0));
        // A creature is not turned.
        e.mem.set_u32(target + 0x1c, 0x1234);
        assert!(!turn_undead_effect_check_target(&mut e, this, target));
        e.mem.set_u32(target + 0x1c, 0);
        e.mem.set_u8(a + 0x12, 0);
        assert!(!turn_undead_effect_check_target(&mut e, this, target));
        e.mem.set_u8(a + 0x12, 1);
        e.mem.set_u8(a + 0x11, 1);
        assert!(!turn_undead_effect_check_target(&mut e, this, target));
        e.mem.set_u8(a + 0x11, 0);
        e.mem.set_u8(a + 0x10, 1);
        assert!(!turn_undead_effect_check_target(&mut e, this, target));
        e.mem.set_u8(a + 0x10, 0);
        e.register(CHECK_TARGET_LEVEL_MAGNITUDE, |_, _| Ret::default());
        assert!(!turn_undead_effect_check_target(&mut e, this, target));
    }

    #[test]
    fn start_turns_with_target_and_caster() {
        let mut e = engine();
        e.register(WORD_CALL_008D0600, |_, _| Ret::default());
        let this = e.mem.alloc(0x4c);
        let t = actor(&mut e);
        let c = actor(&mut e);
        let target = holder(&mut e, t);
        let caster = holder(&mut e, c);
        e.mem.set_u32(this + 0x24, target);
        e.mem.set_u32(this + 0x28, caster);
        e.mem.set_u32(t + 0x14, 0x5555);
        e.call_log = Some(vec![]);
        e.call(0x0082_b530, &args![this]);
        assert_eq!(calls(&e, WORD_CALL_008D0600), vec![vec![0x5555, c, 0]]);
        assert_eq!(e.mem.u8(this + TURNED), 1);
        // No word from slot +0x428: nothing.
        e.mem.set_u8(this + TURNED, 0);
        e.mem.set_u32(t + 0x14, 0);
        turn_undead_effect_start(&mut e, Ptr::new(this));
        assert_eq!(e.mem.u8(this + TURNED), 0);
        // No caster: nothing.
        e.mem.set_u32(t + 0x14, 0x5555);
        e.mem.set_u32(this + 0x28, 0);
        turn_undead_effect_start(&mut e, Ptr::new(this));
        assert_eq!(e.mem.u8(this + TURNED), 0);
    }

    #[test]
    fn finish_releases_a_turned_target() {
        let mut e = engine();
        e.register(ACTOR_TEST_00493BB0, |_, _| 1u32.into_ret());
        let process = e.mem.alloc(8);
        let process_vtable = e.mem.alloc(0x340);
        e.mem.set_u32(process_vtable + SLOT_33C, 0x00f8_0000);
        e.mem.set_u32(process, process_vtable);
        e.register(0x00f8_0000, |_, _| Ret::default());
        e.register_double(ACTOR_MEMBER_008D8520, move |_, _| process.into_ret());
        let this = e.mem.alloc(0x4c);
        let t = actor(&mut e);
        let c = actor(&mut e);
        let target = holder(&mut e, t);
        let caster = holder(&mut e, c);
        e.mem.set_u32(this + 0x24, target);
        e.mem.set_u32(this + 0x28, caster);
        e.call_log = Some(vec![]);
        e.call(0x0082_b5e0, &args![this]);
        assert!(calls(&e, ACTOR_SLOT_BASE + SLOT_ACTOR_434).is_empty());
        e.mem.set_u8(this + TURNED, 1);
        turn_undead_effect_finish(&mut e, Ptr::new(this));
        assert_eq!(
            calls(&e, ACTOR_SLOT_BASE + SLOT_ACTOR_434),
            vec![vec![t, c]]
        );
        assert_eq!(
            calls(&e, 0x00f8_0000),
            vec![vec![process, t, c, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]]
        );
        // Without a caster: slot +0x434 with 0, nothing else.
        e.mem.set_u32(this + 0x28, 0);
        e.call_log = Some(vec![]);
        turn_undead_effect_finish(&mut e, Ptr::new(this));
        assert_eq!(
            calls(&e, ACTOR_SLOT_BASE + SLOT_ACTOR_434),
            vec![vec![t, 0]]
        );
        assert!(calls(&e, 0x00f8_0000).is_empty());
    }

    #[test]
    fn create_or_zero() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_CONSTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let made = e.call(0x0082_b6d0, &args![1u32, 2u32, 3u32]).u32();
        assert_eq!(e.mem.u32(made), TURN_UNDEAD_EFFECT_VTABLE);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(fn_0082b6d0(&mut e, 1, 2, 3), 0);
    }

    fn save_load_engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.set_global(SAVE_LOAD_GAME, 0x4444u32);
        for addr in [
            ACTIVE_EFFECT_SAVE_GAME,
            ACTIVE_EFFECT_LOAD_GAME,
            SAVE_LOAD_WRITE_008579B0,
            SAVE_LOAD_READ_008579E0,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn save_writes_the_turned_byte() {
        let mut e = save_load_engine();
        e.call(0x0082_b750, &args![0x100u32, 7u32]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_SAVE_GAME), vec![vec![0x100, 7]]);
        assert_eq!(
            calls(&e, SAVE_LOAD_WRITE_008579B0),
            vec![vec![0x4444, 0x148, 1]]
        );
    }

    #[test]
    fn load_reads_the_turned_byte() {
        let mut e = save_load_engine();
        e.call(0x0082_b780, &args![0x100u32, 8u32]);
        assert_eq!(calls(&e, ACTIVE_EFFECT_LOAD_GAME), vec![vec![0x100, 8]]);
        assert_eq!(
            calls(&e, SAVE_LOAD_READ_008579E0),
            vec![vec![0x4444, 0x148, 1]]
        );
    }
}
