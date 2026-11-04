//! `fallout/magic/commandeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `00401290` (named `DetailedActorPathHandler::IsDetailedPathHandler` in the engine map):
/// the base body the wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const ALWAYS_TRUE_TARGET_00401290: u32 = 0x0040_1290;

// Translated from 0080b640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable word 01079ae8, just before `CommandEffect::Start`):
/// forwards `this` and one word to `00401290`, a body shared by many slots (named
/// `DetailedActorPathHandler::IsDetailedPathHandler`) that ignores both and returns `MOV
/// AL,1`. The wrapper returns that byte.
pub fn fn_0080b640(e: &mut Engine, this: Ptr, argument: u32) -> bool {
    e.call(ALWAYS_TRUE_TARGET_00401290, &args![this, argument])
        .bool()
}

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `0080b560` (thiscall on a new 0x48-byte block: caster, spell, effect item; unnamed):
/// the `CommandEffect` constructor `Copy` uses.
const CONSTRUCTOR_0080B560: u32 = 0x0080_b560;
/// `MagicTarget::GetTargetAsActor` (`00822b40`).
const MAGIC_TARGET_GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `0059bb30` (thiscall): the effect's target (+0x24).
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `0045cd60` (thiscall): the effect's caster (+0x28).
const EFFECT_CASTER_0045CD60: u32 = 0x0045_cd60;
/// `MagicCaster::GetCasterAsActor` (`00815410`).
const MAGIC_CASTER_GET_CASTER_AS_ACTOR: u32 = 0x0081_5410;
/// `ActiveEffect::Dispel` (`00804210`, thiscall: one byte).
const ACTIVE_EFFECT_DISPEL: u32 = 0x0080_4210;
/// `008d8520` (thiscall on an actor; the map's name `MiddleHighProcess::GetSavedAcquireObject`
/// may come from folded code): the object whose slots +0x27c, +0x528 and +0x52c run here.
const ACTOR_OBJECT_008D8520: u32 = 0x008d_8520;
/// The player pointer (`011dea3c`).
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// `0041ca90` (thiscall on the slot +0x27c object, returns a word; unnamed).
const KIND_0041CA90: u32 = 0x0041_ca90;
/// `00493bb0` (thiscall on the actor, `AL`; unnamed, `animation.cpp`).
const ACTOR_TEST_00493BB0: u32 = 0x0049_3bb0;
/// `Actor::IsRunningRunOnce` (`00881570`, `AL`).
const ACTOR_IS_RUNNING_RUN_ONCE: u32 = 0x0088_1570;
/// `004534f0` (thiscall on the actor: the commander; unnamed, `tes.cpp`).
const ACTOR_FOLLOW_004534F0: u32 = 0x0045_34f0;
/// `100.0` (`float`, `01016410`).
const PLUS_HUNDRED: u32 = 0x0101_6410;
/// `-100.0` (`float`, `01024100`).
const MINUS_HUNDRED: u32 = 0x0102_4100;

/// The effect's target as an actor (`00822b40` of `0059bb30`) and its caster as an actor
/// (`00815410` of `0045cd60`), null when missing.
fn actor_and_commander(e: &mut Engine, this: u32) -> (u32, u32) {
    let actor = if e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() == 0 {
        0
    } else {
        let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
        e.call(MAGIC_TARGET_GET_TARGET_AS_ACTOR, &args![target])
            .u32()
    };
    let commander = if e.call(EFFECT_CASTER_0045CD60, &args![this]).u32() == 0 {
        0
    } else {
        let caster = e.call(EFFECT_CASTER_0045CD60, &args![this]).u32();
        e.call(MAGIC_CASTER_GET_CASTER_AS_ACTOR, &args![caster])
            .u32()
    };
    (actor, commander)
}

// Translated from 0080b590 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080b590 - the exception-unwinding frame (FS:[0]) is not translated.
/// `CommandEffect::Copy` (Xbox PDB): a new 0x48-byte effect (`operator new`, then
/// `0080b560` with the caster +0x28, spell +8 and effect item +0xc; null when the
/// allocation fails), given this effect's data through `CopyData` (slot +0x44); returns it.
pub fn command_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let base = this.addr();
        let item = e.mem.u32(base + 0xc);
        let spell = e.mem.u32(base + 8);
        let caster = e.mem.u32(base + 0x28);
        e.call(CONSTRUCTOR_0080B560, &args![block, caster, spell, item])
            .u32()
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

// Translated from 0080b660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CommandEffect::Start` (Xbox PDB): with a target actor and a caster actor, the actor
/// is put under the caster's command (`InitCommandedActor`).
pub fn command_effect_start(e: &mut Engine, this: Ptr) {
    let (actor, commander) = actor_and_commander(e, this.addr());
    if actor != 0 && commander != 0 {
        command_effect_init_commanded_actor(e, actor, commander);
    }
}

// Translated from 0080b6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CommandEffect::Update` (Xbox PDB; the `float` argument is not read): with a target
/// actor and a caster actor, the effect is dispelled when the actor's slot +0x22c (with 0)
/// or +0x2e8 is true, otherwise the command is kept up (`fn_0080b8f0`).
pub fn command_effect_update(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let (actor, commander) = actor_and_commander(e, this.addr());
    if actor == 0 || commander == 0 {
        return;
    }
    if e.vcall(actor, 0x22c, &args![0u32]).bool() || e.vcall(actor, 0x2e8, &[]).bool() {
        e.call(ACTIVE_EFFECT_DISPEL, &args![this, 0u32]);
    } else {
        fn_0080b8f0(e, actor, commander);
    }
}

// Translated from 0080b7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CommandEffect::Finish` (Xbox PDB): with a target actor and a caster actor, the command
/// ends (`CleanupCommandedActor` with 1).
pub fn command_effect_finish(e: &mut Engine, this: Ptr) {
    let (actor, commander) = actor_and_commander(e, this.addr());
    if actor != 0 && commander != 0 {
        command_effect_cleanup_commanded_actor(e, actor, commander, 1);
    }
}

// Translated from 0080b850 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CommandEffect::InitCommandedActor` (Xbox PDB, cdecl: actor, commander): the actor's
/// slot +0x460 with the commander and 100.0 (`01016410`), its `008d8520` object's slot
/// +0x528 with the commander, its slot +0x3a0 with 1, 100, 0; then, unless the commander
/// is the player, `0041ca90` on the commander's `008d8520` object's slot +0x27c result when
/// it is set.
pub fn command_effect_init_commanded_actor(e: &mut Engine, actor: u32, commander: u32) {
    let disposition = x87_quiet_bits(e.global::<u32>(PLUS_HUNDRED));
    e.vcall(actor, 0x460, &args![commander, disposition]);
    let object = e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32();
    e.vcall(object, 0x528, &args![commander]);
    e.vcall(actor, 0x3a0, &args![1u32, 0x64u32, 0u32]);
    let mut package = 0;
    if commander != e.global::<u32>(PLAYER_POINTER) {
        let object = e.call(ACTOR_OBJECT_008D8520, &args![commander]).u32();
        package = e.vcall(object, 0x27c, &[]).u32();
    }
    if package != 0 {
        e.call(KIND_0041CA90, &args![package]);
    }
}

// Translated from 0080b8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (`commandeffect.cpp`; cdecl: actor, commander): keeps the command up. Without
/// the actor's `008d8520` object nothing happens; its slot +0x528 gets the commander when
/// slot +0x52c is another. Unless `00493bb0` is true for the actor: when the commander
/// (not the player) has no slot +0x27c object or one whose `0041ca90` kind is 0x12, and
/// the actor has none or is not running a run-once package (`Actor::IsRunningRunOnce`),
/// `004534f0` runs on the actor with the commander.
pub fn fn_0080b8f0(e: &mut Engine, actor: u32, commander: u32) {
    if e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32() == 0 {
        return;
    }
    let object = e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32();
    if e.vcall(object, 0x52c, &[]).u32() != commander {
        let object = e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32();
        e.vcall(object, 0x528, &args![commander]);
    }
    if e.call(ACTOR_TEST_00493BB0, &args![actor]).bool() {
        return;
    }
    let mut commander_package = 0;
    if commander != e.global::<u32>(PLAYER_POINTER) {
        let object = e.call(ACTOR_OBJECT_008D8520, &args![commander]).u32();
        commander_package = e.vcall(object, 0x27c, &[]).u32();
    }
    let object = e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32();
    let actor_package = e.vcall(object, 0x27c, &[]).u32();
    if commander_package != 0 && e.call(KIND_0041CA90, &args![commander_package]).u32() != 0x12 {
        return;
    }
    if actor_package != 0 && e.call(ACTOR_IS_RUNNING_RUN_ONCE, &args![actor]).bool() {
        return;
    }
    e.call(ACTOR_FOLLOW_004534F0, &args![actor, commander]);
}

// Translated from 0080b9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CommandEffect::CleanupCommandedActor` (Xbox PDB, cdecl: actor, commander, a byte):
/// with the byte set and the actor's `008d8520` object, that object's slot +0x528 gets
/// null; then the actor's slot +0x460 with the commander and -100.0 (`01024100`) and its
/// slot +0x3a0 with 1, -100, 0.
pub fn command_effect_cleanup_commanded_actor(
    e: &mut Engine,
    actor: u32,
    commander: u32,
    release: u8,
) {
    if e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32() != 0 && release != 0 {
        let object = e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32();
        e.vcall(object, 0x528, &args![0u32]);
    }
    let disposition = x87_quiet_bits(e.global::<u32>(MINUS_HUNDRED));
    e.vcall(actor, 0x460, &args![commander, disposition]);
    e.vcall(actor, 0x3a0, &args![1u32, 0xffff_ff9cu32, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_b640, fn_0080b640(Ptr, u32) -> bool),
        entry!(0x0080_b590, command_effect_copy(Ptr) -> u32),
        entry!(0x0080_b660, command_effect_start(Ptr)),
        entry!(0x0080_b6f0, command_effect_update(Ptr, u32)),
        entry!(0x0080_b7c0, command_effect_finish(Ptr)),
        entry!(0x0080_b850, command_effect_init_commanded_actor(u32, u32)),
        entry!(0x0080_b8f0, fn_0080b8f0(u32, u32)),
        entry!(
            0x0080_b9e0,
            command_effect_cleanup_commanded_actor(u32, u32, u8)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_fn_0080b640_forwards_this_and_the_argument_and_returns_the_byte() {
        let mut e = Engine::new();
        // The double sets bit 8 too: only the low byte (AL) is the result.
        e.register(ALWAYS_TRUE_TARGET_00401290, |_, a| {
            (0x100 | (a[1] & 1)).into_ret()
        });
        for (argument, expected) in [(0x2001u32, true), (0x2000u32, false)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0080_b640, &args![0x1000u32, argument]);
            let log = e.call_log.take().unwrap();
            let hits: Vec<_> = log
                .iter()
                .filter(|call| call.0 == ALWAYS_TRUE_TARGET_00401290)
                .collect();
            assert_eq!(hits.len(), 1);
            assert_eq!(hits[0].1, vec![0x1000u32, argument]);
            assert_eq!(result.bool(), expected);
        }
    }
}

#[cfg(test)]
mod command_tests {
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

    struct Rig {
        e: Engine,
        actor: u32,
        commander: u32,
        actor_object: u32,
        commander_object: u32,
    }

    /// An actor and a commander with their `008d8520` objects (slots +0x27c answering the
    /// packages given; the actor object's +0x52c answering `following`).
    fn rig(actor_package: u32, commander_package: u32, following: u32) -> Rig {
        let mut e = Engine::new();
        e.map(0x0101_6000, 0x1000);
        e.map(0x0102_4000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global(PLUS_HUNDRED, 100.0f32.to_bits());
        e.set_global(MINUS_HUNDRED, (-100.0f32).to_bits());
        let actor = object(&mut e, &[(0x460, 0), (0x3a0, 0), (0x22c, 0), (0x2e8, 0)]);
        let commander = object(&mut e, &[]);
        let actor_object = object(
            &mut e,
            &[(0x528, 0), (0x52c, following), (0x27c, actor_package)],
        );
        let commander_object = object(&mut e, &[(0x27c, commander_package)]);
        e.register_double(ACTOR_OBJECT_008D8520, move |_, a| {
            (if a[0] == actor {
                actor_object
            } else {
                commander_object
            })
            .into_ret()
        });
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        answer(&mut e, EFFECT_CASTER_0045CD60, 0x7100);
        answer(&mut e, MAGIC_CASTER_GET_CASTER_AS_ACTOR, commander);
        for addr in [
            ACTIVE_EFFECT_DISPEL,
            ACTOR_FOLLOW_004534F0,
            ACTOR_TEST_00493BB0,
            KIND_0041CA90,
            ACTOR_IS_RUNNING_RUN_ONCE,
        ] {
            answer(&mut e, addr, 0);
        }
        e.call_log = Some(vec![]);
        Rig {
            e,
            actor,
            commander,
            actor_object,
            commander_object,
        }
    }

    #[test]
    fn copy_allocates_constructs_and_copies_data() {
        let mut e = Engine::new();
        let this = object(&mut e, &[(0x44, 0)]);
        e.mem.set_u32(this + 8, 0x08);
        e.mem.set_u32(this + 0xc, 0x0c);
        e.mem.set_u32(this + 0x28, 0x28);
        answer(&mut e, OPERATOR_NEW, 0x9000);
        answer(&mut e, CONSTRUCTOR_0080B560, 0x9004);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_b590, &args![this]).u32(), 0x9004);
        assert_eq!(
            calls(&e, CONSTRUCTOR_0080B560),
            vec![vec![0x9000, 0x28, 0x08, 0x0c]]
        );
        assert_eq!(calls(&e, slot(&e, this, 0x44)), vec![vec![this, 0x9004]]);
        answer(&mut e, OPERATOR_NEW, 0);
        assert_eq!(e.call(0x0080_b590, &args![this]).u32(), 0);
    }

    #[test]
    fn start_puts_the_actor_under_command() {
        let Rig {
            mut e,
            actor,
            commander,
            actor_object,
            commander_object,
        } = rig(0, 0x2700, 0);
        e.call(0x0080_b660, &args![0x1000u32]);
        assert_eq!(
            calls(&e, slot(&e, actor, 0x460)),
            vec![vec![actor, commander, 100.0f32.to_bits()]]
        );
        assert_eq!(
            calls(&e, slot(&e, actor_object, 0x528)),
            vec![vec![actor_object, commander]]
        );
        assert_eq!(
            calls(&e, slot(&e, actor, 0x3a0)),
            vec![vec![actor, 1, 0x64, 0]]
        );
        assert_eq!(calls(&e, slot(&e, commander_object, 0x27c)).len(), 1);
        assert_eq!(calls(&e, KIND_0041CA90), vec![vec![0x2700]]);
        // The player's own package is not asked for.
        let mut rig = rig(0, 0x2700, 0);
        rig.e.set_global(PLAYER_POINTER, rig.commander);
        rig.e.call(0x0080_b660, &args![0x1000u32]);
        assert!(calls(&rig.e, slot(&rig.e, rig.commander_object, 0x27c)).is_empty());
    }

    #[test]
    fn update_dispels_a_busy_actor_or_keeps_the_command() {
        let Rig {
            mut e,
            actor,
            commander,
            actor_object,
            ..
        } = rig(0, 0, 0x5555);
        e.call(0x0080_b6f0, &args![0x1000u32, 0u32]);
        assert!(calls(&e, ACTIVE_EFFECT_DISPEL).is_empty());
        // Following someone else: the commander again, then the follow.
        assert_eq!(
            calls(&e, slot(&e, actor_object, 0x528)),
            vec![vec![actor_object, commander]]
        );
        assert_eq!(
            calls(&e, ACTOR_FOLLOW_004534F0),
            vec![vec![actor, commander]]
        );
        let mut rig = rig(0, 0, 0);
        let busy = slot(&rig.e, rig.actor, 0x2e8);
        answer(&mut rig.e, busy, 1);
        rig.e.call(0x0080_b6f0, &args![0x1000u32, 0u32]);
        assert_eq!(calls(&rig.e, ACTIVE_EFFECT_DISPEL), vec![vec![0x1000, 0]]);
    }

    #[test]
    fn fn_0080b8f0_follows_only_when_the_packages_allow() {
        // The commander's package of kind 0x12, the actor not running a run-once package.
        let mut rig1 = rig(0x2600, 0x2700, 0);
        answer(&mut rig1.e, KIND_0041CA90, 0x12);
        rig1.e.call(0x0080_b8f0, &args![rig1.actor, rig1.commander]);
        assert_eq!(
            calls(&rig1.e, ACTOR_FOLLOW_004534F0),
            vec![vec![rig1.actor, rig1.commander]]
        );
        // Another kind: no follow.
        let mut rig2 = rig(0x2600, 0x2700, 0);
        answer(&mut rig2.e, KIND_0041CA90, 0x11);
        rig2.e.call(0x0080_b8f0, &args![rig2.actor, rig2.commander]);
        assert!(calls(&rig2.e, ACTOR_FOLLOW_004534F0).is_empty());
        // A run-once package: no follow.
        let mut rig3 = rig(0x2600, 0, 0);
        answer(&mut rig3.e, ACTOR_IS_RUNNING_RUN_ONCE, 1);
        rig3.e.call(0x0080_b8f0, &args![rig3.actor, rig3.commander]);
        assert!(calls(&rig3.e, ACTOR_FOLLOW_004534F0).is_empty());
        // 00493bb0 true: nothing after the commander check.
        let mut rig4 = rig(0, 0, 0);
        answer(&mut rig4.e, ACTOR_TEST_00493BB0, 1);
        rig4.e.call(0x0080_b8f0, &args![rig4.actor, rig4.commander]);
        assert!(calls(&rig4.e, ACTOR_FOLLOW_004534F0).is_empty());
        // Without the actor's object nothing runs.
        let mut rig5 = rig(0, 0, 0);
        answer(&mut rig5.e, ACTOR_OBJECT_008D8520, 0);
        rig5.e.call(0x0080_b8f0, &args![rig5.actor, rig5.commander]);
        assert!(calls(&rig5.e, ACTOR_TEST_00493BB0).is_empty());
    }

    #[test]
    fn finish_and_cleanup_release_the_actor() {
        let Rig {
            mut e,
            actor,
            commander,
            actor_object,
            ..
        } = rig(0, 0, 0);
        e.call(0x0080_b7c0, &args![0x1000u32]);
        assert_eq!(
            calls(&e, slot(&e, actor_object, 0x528)),
            vec![vec![actor_object, 0]]
        );
        assert_eq!(
            calls(&e, slot(&e, actor, 0x460)),
            vec![vec![actor, commander, (-100.0f32).to_bits()]]
        );
        assert_eq!(
            calls(&e, slot(&e, actor, 0x3a0)),
            vec![vec![actor, 1, 0xffff_ff9c, 0]]
        );
        // Only the low byte counts: 0x100 keeps the object's commander.
        e.call_log = Some(vec![]);
        e.call(0x0080_b9e0, &args![actor, commander, 0x100u32]);
        assert!(calls(&e, slot(&e, actor_object, 0x528)).is_empty());
        assert_eq!(calls(&e, slot(&e, actor, 0x3a0)).len(), 1);
    }
}
