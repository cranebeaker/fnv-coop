//! `fallout/magic/frenzyeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use super::activeeffect::{load_bytes, save_bytes};

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `0082ba60` (thiscall: three words; unnamed): the `ValueModifierEffect` constructor.
const VALUE_MODIFIER_EFFECT_CONSTRUCTOR: u32 = 0x0082_ba60;
/// `FrenzyEffect`'s vtable (stored by its constructor).
const FRENZY_EFFECT_VTABLE: u32 = 0x0107_a16c;
/// The byte at +0x4c: set by `Start` when the target turned on the caster.
const STARTED: u32 = 0x4c;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, vfDelta, source type, target type, isReference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `MagicTarget`'s RTTI type descriptor (`.?AVMagicTarget@@`).
const MAGIC_TARGET_TYPE: u32 = 0x011a_0ef8;
/// `Character`'s RTTI type descriptor (`.?AVCharacter@@`).
const CHARACTER_TYPE: u32 = 0x011a_0f14;
/// `00495460` (thiscall on the effect, returns `ST0`; unnamed): its magnitude.
const EFFECT_MAGNITUDE_00495460: u32 = 0x0049_5460;
/// `0087f9f0` (thiscall on the actor, returns `AX`; unnamed, `actor.cpp`): its level.
const ACTOR_LEVEL_0087F9F0: u32 = 0x0087_f9f0;
/// `MagicFormulas::CheckTargetLevelMagnitude` (`006494d0`, cdecl: level, magnitude).
const CHECK_TARGET_LEVEL_MAGNITUDE: u32 = 0x0064_94d0;
/// `ValueModifierEffect::Start` (`0082c000`).
const VALUE_MODIFIER_EFFECT_START: u32 = 0x0082_c000;
/// `ValueModifierEffect::Finish` (`0082c810`).
const VALUE_MODIFIER_EFFECT_FINISH: u32 = 0x0082_c810;
/// `ValueModifierEffect::GetSaveSize` (`00802d90`, thiscall: one word; returns `AX`).
const VALUE_MODIFIER_EFFECT_GET_SAVE_SIZE: u32 = 0x0080_2d90;
/// `ValueModifierEffect::SaveGame` (`00802db0`, thiscall: one word).
const VALUE_MODIFIER_EFFECT_SAVE_GAME: u32 = 0x0080_2db0;
/// `ValueModifierEffect::LoadGame` (`00802dd0`, thiscall: one word).
const VALUE_MODIFIER_EFFECT_LOAD_GAME: u32 = 0x0080_2dd0;
/// `MagicTarget::GetTargetAsActor` (`00822b40`).
const MAGIC_TARGET_GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `0059bb30` (thiscall): the effect's target (+0x24).
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `0045cd60` (thiscall): the effect's caster (+0x28).
const EFFECT_CASTER_0045CD60: u32 = 0x0045_cd60;
/// `MagicCaster::GetCasterAsActor` (`00815410`).
const MAGIC_CASTER_GET_CASTER_AS_ACTOR: u32 = 0x0081_5410;
/// `00493bb0` (thiscall on the actor, `AL`; unnamed, `animation.cpp`).
const ACTOR_TEST_00493BB0: u32 = 0x0049_3bb0;
/// `BaseProcess::GetActorPackageThatIsRunning` (map name, `00717e50`, thiscall on the
/// process lists, no stack words; the body here gives a list of actors).
const ACTOR_LIST_00717E50: u32 = 0x0071_7e50;
/// The `ProcessLists` object (`011e0e80`).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// `005be5c0` (thiscall on that list: one word; unnamed): its count.
const LIST_COUNT_005BE5C0: u32 = 0x005b_e5c0;
/// `00968670` (thiscall on that list: an index; unnamed): its element.
const LIST_ELEMENT_00968670: u32 = 0x0096_8670;
/// `Actor::RemoveFleeTarget` (`008a6730`, thiscall: one word).
const ACTOR_REMOVE_FLEE_TARGET: u32 = 0x008a_6730;

// Translated from 0080dbb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`frenzyeffect.cpp`; thiscall, three words): the `FrenzyEffect` constructor: the
/// `ValueModifierEffect` part (`0082ba60` with the three words), its vtable and the byte at
/// +0x4c clear; returns `this`.
pub fn fn_0080dbb0(e: &mut Engine, this: Ptr, caster: u32, spell: u32, item: u32) -> u32 {
    e.call(
        VALUE_MODIFIER_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, item],
    );
    e.mem.set_u32(this.addr(), FRENZY_EFFECT_VTABLE);
    e.mem.set_u8(this.addr() + STARTED, 0);
    this.addr()
}

// Translated from 0080dbf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080dbf0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `FrenzyEffect::Copy` (Xbox PDB): a new 0x50-byte effect (`operator new`, then
/// `fn_0080dbb0` with the caster +0x28, spell +8 and effect item +0xc; null when the
/// allocation fails) with this effect's byte at +0x4c, given this effect's data through
/// `CopyData` (slot +0x44); returns it.
pub fn frenzy_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x50u32]).u32();
    let base = this.addr();
    let copy = if block == 0 {
        0
    } else {
        let item = e.mem.u32(base + 0xc);
        let spell = e.mem.u32(base + 8);
        let caster = e.mem.u32(base + 0x28);
        fn_0080dbb0(e, Ptr::new(block), caster, spell, item)
    };
    let started = e.mem.u8(base + STARTED);
    e.mem.set_u8(copy.wrapping_add(STARTED), started);
    e.vcall(base, 0x44, &args![copy]);
    copy
}

// Translated from 0080dca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FrenzyEffect::CheckTarget` (Xbox PDB): true for a target that is a `Character`
/// (`__RTDynamicCast` from `MagicTarget`) whose slots +0x22c (with 0) and +0x2e8 are false
/// and whose level `CheckTargetLevelMagnitude` accepts for the magnitude.
pub fn frenzy_effect_check_target(e: &mut Engine, this: Ptr, target: u32) -> bool {
    let character = e
        .call(
            RT_DYNAMIC_CAST,
            &args![target, 0u32, MAGIC_TARGET_TYPE, CHARACTER_TYPE, 0u32],
        )
        .u32();
    if character == 0
        || e.vcall(character, 0x22c, &args![0u32]).bool()
        || e.vcall(character, 0x2e8, &[]).bool()
    {
        return false;
    }
    let magnitude = x87_store32(e.call(EFFECT_MAGNITUDE_00495460, &args![this]).f64());
    let level = u32::from(e.call(ACTOR_LEVEL_0087F9F0, &args![character]).u16());
    e.call(CHECK_TARGET_LEVEL_MAGNITUDE, &args![level, magnitude])
        .bool()
}

/// The effect's target as an actor (`00822b40` of `0059bb30`), null without a target.
fn target_actor(e: &mut Engine, this: u32) -> u32 {
    if e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() == 0 {
        0
    } else {
        let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
        e.call(MAGIC_TARGET_GET_TARGET_AS_ACTOR, &args![target])
            .u32()
    }
}

// Translated from 0080dd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FrenzyEffect::Start` (Xbox PDB): `ValueModifierEffect::Start`, then the byte at +0x4c
/// is set for a target actor with a caster actor when `00493bb0` is true for the target.
pub fn frenzy_effect_start(e: &mut Engine, this: Ptr) {
    e.call(VALUE_MODIFIER_EFFECT_START, &args![this]);
    let actor = target_actor(e, this.addr());
    let caster = if e.call(EFFECT_CASTER_0045CD60, &args![this]).u32() == 0 {
        0
    } else {
        let caster = e.call(EFFECT_CASTER_0045CD60, &args![this]).u32();
        e.call(MAGIC_CASTER_GET_CASTER_AS_ACTOR, &args![caster])
            .u32()
    };
    if actor != 0 && caster != 0 && e.call(ACTOR_TEST_00493BB0, &args![actor]).bool() {
        e.mem.set_u8(this.addr() + STARTED, 1);
    }
}

// Translated from 0080ddd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FrenzyEffect::Finish` (Xbox PDB): `ValueModifierEffect::Finish`; then, unless the byte
/// at +0x4c is set, for a target actor with `00493bb0` true: its slot +0x434 with 0, and
/// each other actor of the process lists' actor list (`00717e50`, counted by `005be5c0`
/// with 0) whose slot +0x100 is true stops fleeing it (`RemoveFleeTarget`) and, when
/// `00493bb0` is true for it, gets slot +0x434 with the target.
pub fn frenzy_effect_finish(e: &mut Engine, this: Ptr) {
    e.call(VALUE_MODIFIER_EFFECT_FINISH, &args![this]);
    if e.mem.u8(this.addr() + STARTED) != 0 {
        return;
    }
    let actor = target_actor(e, this.addr());
    if actor == 0 || !e.call(ACTOR_TEST_00493BB0, &args![actor]).bool() {
        return;
    }
    e.vcall(actor, 0x434, &args![0u32]);
    let list = e.call(ACTOR_LIST_00717E50, &args![PROCESS_LISTS]).u32();
    let mut index = 0u32;
    while index < e.call(LIST_COUNT_005BE5C0, &args![list, 0u32]).u32() {
        let other = e.call(LIST_ELEMENT_00968670, &args![list, index]).u32();
        if other != actor {
            let other = if other != 0 && e.vcall(other, 0x100, &[]).bool() {
                other
            } else {
                0
            };
            if other != 0 {
                e.call(ACTOR_REMOVE_FLEE_TARGET, &args![other, actor]);
                if e.call(ACTOR_TEST_00493BB0, &args![other]).bool() {
                    e.vcall(other, 0x434, &args![actor]);
                }
            }
        }
        index += 1;
    }
}

// Translated from 0080df10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080df10 - the exception-unwinding frame (FS:[0]) is not translated.
/// Unnamed (`frenzyeffect.cpp`; cdecl, three words): a new 0x50-byte `FrenzyEffect`
/// (`fn_0080dbb0` with the three words; null when the allocation fails).
pub fn fn_0080df10(e: &mut Engine, caster: u32, spell: u32, item: u32) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x50u32]).u32();
    if block == 0 {
        0
    } else {
        fn_0080dbb0(e, Ptr::new(block), caster, spell, item)
    }
}

// Translated from 0080df90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`frenzyeffect.cpp`; the `FrenzyEffect` save size): `ValueModifierEffect::
/// GetSaveSize` plus the byte at +0x4c. Returns `AX`.
pub fn fn_0080df90(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    let mut size = 0u16;
    let base = e
        .call(VALUE_MODIFIER_EFFECT_GET_SAVE_SIZE, &args![this, argument])
        .u16();
    size = size.wrapping_add(base);
    size.wrapping_add(1)
}

// Translated from 0080dfd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FrenzyEffect::SaveGame` (Xbox PDB): `ValueModifierEffect::SaveGame`, then the byte at
/// +0x4c.
pub fn frenzy_effect_save_game(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(VALUE_MODIFIER_EFFECT_SAVE_GAME, &args![this, argument]);
    save_bytes(e, this.addr() + STARTED, 1);
}

// Translated from 0080e000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FrenzyEffect::LoadGame` (Xbox PDB): `ValueModifierEffect::LoadGame`, then the byte at
/// +0x4c.
pub fn frenzy_effect_load_game(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(VALUE_MODIFIER_EFFECT_LOAD_GAME, &args![this, argument]);
    load_bytes(e, this.addr() + STARTED, 1);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_dbb0, fn_0080dbb0(Ptr, u32, u32, u32) -> u32),
        entry!(0x0080_dbf0, frenzy_effect_copy(Ptr) -> u32),
        entry!(0x0080_dca0, frenzy_effect_check_target(Ptr, u32) -> bool),
        entry!(0x0080_dd30, frenzy_effect_start(Ptr)),
        entry!(0x0080_ddd0, frenzy_effect_finish(Ptr)),
        entry!(0x0080_df10, fn_0080df10(u32, u32, u32) -> u32),
        entry!(0x0080_df90, fn_0080df90(Ptr, u32) -> u16),
        entry!(0x0080_dfd0, frenzy_effect_save_game(Ptr, u32)),
        entry!(0x0080_e000, frenzy_effect_load_game(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    /// An object whose vtable slots answer the given words; each slot's double sits at the
    /// slot's own address.
    fn object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let vtable = e.mem.alloc(0x600);
        for (offset, value) in slots {
            e.mem.set_u32(vtable + offset, vtable + offset);
            answer(e, vtable + offset, *value);
        }
        let this = e.mem.alloc(0x100);
        e.mem.set_u32(this, vtable);
        this
    }

    fn slot(e: &Engine, object: u32, offset: u32) -> u32 {
        e.mem.u32(object) + offset
    }

    #[test]
    fn constructor_and_copy_carry_the_started_byte() {
        let mut e = Engine::new();
        answer(&mut e, VALUE_MODIFIER_EFFECT_CONSTRUCTOR, 0);
        let this = object(&mut e, &[(0x44, 0)]);
        e.mem.set_u32(this + 8, 0x08);
        e.mem.set_u32(this + 0xc, 0x0c);
        e.mem.set_u32(this + 0x28, 0x28);
        e.mem.set_u8(this + 0x4c, 1);
        let block = e.mem.alloc(0x50);
        e.mem.set_u8(block + 0x4c, 7);
        answer(&mut e, OPERATOR_NEW, block);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_dbf0, &args![this]).u32(), block);
        assert_eq!(e.mem.u32(block), FRENZY_EFFECT_VTABLE);
        assert_eq!(e.mem.u8(block + 0x4c), 1);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_CONSTRUCTOR),
            vec![vec![block, 0x28, 0x08, 0x0c]]
        );
        assert_eq!(calls(&e, slot(&e, this, 0x44)), vec![vec![this, block]]);
        e.mem.set_u8(block + 0x4c, 7);
        assert_eq!(e.call(0x0080_df10, &args![1u32, 2u32, 3u32]).u32(), block);
        assert_eq!(e.mem.u8(block + 0x4c), 0);
        answer(&mut e, OPERATOR_NEW, 0);
        assert_eq!(e.call(0x0080_df10, &args![1u32, 2u32, 3u32]).u32(), 0);
    }

    #[test]
    fn check_target_needs_a_free_character_and_the_level_formula() {
        for (cast, busy, accepted, expected) in [
            (true, 0u32, 1u32, true),
            (true, 0, 0, false),
            (true, 1, 1, false),
            (false, 0, 1, false),
        ] {
            let mut e = Engine::new();
            let character = object(&mut e, &[(0x22c, busy), (0x2e8, 0)]);
            answer(&mut e, RT_DYNAMIC_CAST, if cast { character } else { 0 });
            e.register(EFFECT_MAGNITUDE_00495460, |_, _| Ret {
                st0: 8.0,
                ..Ret::default()
            });
            answer(&mut e, ACTOR_LEVEL_0087F9F0, 3);
            answer(&mut e, CHECK_TARGET_LEVEL_MAGNITUDE, accepted);
            e.call_log = Some(vec![]);
            assert_eq!(
                e.call(0x0080_dca0, &args![0x1000u32, 0x2000u32]).bool(),
                expected
            );
            assert_eq!(
                calls(&e, RT_DYNAMIC_CAST),
                vec![vec![0x2000, 0, MAGIC_TARGET_TYPE, CHARACTER_TYPE, 0]]
            );
        }
    }

    #[test]
    fn start_marks_a_turned_target() {
        for (test, started) in [(1u32, 1u8), (0, 0)] {
            let mut e = Engine::new();
            answer(&mut e, VALUE_MODIFIER_EFFECT_START, 0);
            answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
            answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, 0x5000);
            answer(&mut e, EFFECT_CASTER_0045CD60, 0x7100);
            answer(&mut e, MAGIC_CASTER_GET_CASTER_AS_ACTOR, 0x5100);
            answer(&mut e, ACTOR_TEST_00493BB0, test);
            let this = e.mem.alloc(0x50);
            e.call_log = Some(vec![]);
            e.call(0x0080_dd30, &args![this]);
            assert_eq!(calls(&e, VALUE_MODIFIER_EFFECT_START), vec![vec![this]]);
            assert_eq!(calls(&e, ACTOR_TEST_00493BB0), vec![vec![0x5000]]);
            assert_eq!(e.mem.u8(this + 0x4c), started);
        }
    }

    #[test]
    fn finish_calms_the_actors_around_the_target() {
        let mut e = Engine::new();
        answer(&mut e, VALUE_MODIFIER_EFFECT_FINISH, 0);
        let actor = object(&mut e, &[(0x434, 0)]);
        let ally = object(&mut e, &[(0x100, 1), (0x434, 0)]);
        let stranger = object(&mut e, &[(0x100, 0)]);
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        answer(&mut e, ACTOR_TEST_00493BB0, 1);
        answer(&mut e, ACTOR_LIST_00717E50, 0x6000);
        answer(&mut e, LIST_COUNT_005BE5C0, 4);
        let actors = [actor, ally, stranger, 0];
        e.register_double(LIST_ELEMENT_00968670, move |_, a| {
            actors[a[1] as usize].into_ret()
        });
        answer(&mut e, ACTOR_REMOVE_FLEE_TARGET, 0);
        let this = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        e.call(0x0080_ddd0, &args![this]);
        assert_eq!(calls(&e, slot(&e, actor, 0x434)), vec![vec![actor, 0]]);
        assert_eq!(calls(&e, ACTOR_LIST_00717E50), vec![vec![PROCESS_LISTS]]);
        assert_eq!(calls(&e, LIST_COUNT_005BE5C0).len(), 5);
        assert_eq!(calls(&e, ACTOR_REMOVE_FLEE_TARGET), vec![vec![ally, actor]]);
        assert_eq!(calls(&e, slot(&e, ally, 0x434)), vec![vec![ally, actor]]);
        // Started effects skip all of it.
        e.mem.set_u8(this + 0x4c, 1);
        e.call_log = Some(vec![]);
        e.call(0x0080_ddd0, &args![this]);
        assert!(calls(&e, EFFECT_TARGET_0059BB30).is_empty());
    }

    #[test]
    fn save_size_save_and_load_add_the_byte() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let game = e.mem.alloc(0x100);
        e.set_global(0x011d_e45c, game);
        answer(&mut e, VALUE_MODIFIER_EFFECT_GET_SAVE_SIZE, 0x0003_0010);
        answer(&mut e, VALUE_MODIFIER_EFFECT_SAVE_GAME, 0);
        answer(&mut e, VALUE_MODIFIER_EFFECT_LOAD_GAME, 0);
        answer(&mut e, 0x0085_79b0, 0);
        answer(&mut e, 0x0085_79e0, 0);
        let this = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_df90, &args![this, 0x77u32]).u16(), 0x11);
        e.call(0x0080_dfd0, &args![this, 0x77u32]);
        e.call(0x0080_e000, &args![this, 0x77u32]);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_SAVE_GAME),
            vec![vec![this, 0x77]]
        );
        assert_eq!(calls(&e, 0x0085_79b0), vec![vec![game, this + 0x4c, 1]]);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_LOAD_GAME),
            vec![vec![this, 0x77]]
        );
        assert_eq!(calls(&e, 0x0085_79e0), vec![vec![game, this + 0x4c, 1]]);
    }
}
