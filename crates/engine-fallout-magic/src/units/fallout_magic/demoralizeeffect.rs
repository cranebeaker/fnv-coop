//! `fallout/magic/demoralizeeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `0080c890` (thiscall on a new 0x4c-byte block: caster, spell, effect item; unnamed):
/// the `DemoralizeEffect` constructor `Copy` uses.
const CONSTRUCTOR_0080C890: u32 = 0x0080_c890;
/// `MagicTarget::GetTargetAsActor` (`00822b40`).
const MAGIC_TARGET_GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `0059bb30` (thiscall): the effect's target (+0x24).
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `0045cd60` (thiscall): the effect's caster (+0x28).
const EFFECT_CASTER_0045CD60: u32 = 0x0045_cd60;
/// `MagicCaster::GetCasterAsActor` (`00815410`).
const MAGIC_CASTER_GET_CASTER_AS_ACTOR: u32 = 0x0081_5410;
/// `00495460` (thiscall on the effect, returns `ST0`; unnamed): its magnitude.
const EFFECT_MAGNITUDE_00495460: u32 = 0x0049_5460;
/// `0087f9f0` (thiscall on the actor, returns `AX`; unnamed, `actor.cpp`): its level.
const ACTOR_LEVEL_0087F9F0: u32 = 0x0087_f9f0;
/// `MagicFormulas::CheckTargetLevelMagnitude` (`006494d0`, cdecl: level, magnitude).
const CHECK_TARGET_LEVEL_MAGNITUDE: u32 = 0x0064_94d0;
/// `__RTDynamicCast` (`00ec43fb`, cdecl: object, vfDelta, source type, target type, isReference).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `MagicTarget`'s RTTI type descriptor (`.?AVMagicTarget@@`).
const MAGIC_TARGET_TYPE: u32 = 0x011a_0ef8;
/// `Creature`'s RTTI type descriptor (`.?AVCreature@@`).
const CREATURE_TYPE: u32 = 0x011a_106c;
/// `00608d80` (thiscall on the actor, `AL`; unnamed, `tesnpc.cpp`).
const ACTOR_TEST_00608D80: u32 = 0x0060_8d80;
/// `ProcessLists::PrintLists` (map name, `008d0600`, thiscall on the combat controller:
/// two words; the body here takes the caster and 0).
const COMBAT_CONTROLLER_008D0600: u32 = 0x008d_0600;
/// `Actor::IsFleeing` (`008a6650`, thiscall: one word).
const ACTOR_IS_FLEEING: u32 = 0x008a_6650;
/// `Actor::EndInterruptPackage` (`00881680`, thiscall: one word).
const ACTOR_END_INTERRUPT_PACKAGE: u32 = 0x0088_1680;
/// `Actor::RemoveFleeTarget` (`008a6730`, thiscall: the caster).
const ACTOR_REMOVE_FLEE_TARGET: u32 = 0x008a_6730;
/// `00493bb0` (thiscall on the actor, `AL`; unnamed, `animation.cpp`).
const ACTOR_TEST_00493BB0: u32 = 0x0049_3bb0;
/// `008d8520` (thiscall on an actor; the map's name `MiddleHighProcess::GetSavedAcquireObject`
/// may come from folded code): the object whose slot +0x33c runs here.
const ACTOR_OBJECT_008D8520: u32 = 0x008d_8520;
/// `-1.0` (`float`, `01012054`).
const MINUS_ONE: u32 = 0x0101_2054;

/// The effect's target as an actor and its caster as an actor (null when missing).
fn actor_and_caster(e: &mut Engine, this: u32) -> (u32, u32) {
    let actor = if e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() == 0 {
        0
    } else {
        let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
        e.call(MAGIC_TARGET_GET_TARGET_AS_ACTOR, &args![target])
            .u32()
    };
    let caster = if e.call(EFFECT_CASTER_0045CD60, &args![this]).u32() == 0 {
        0
    } else {
        let caster = e.call(EFFECT_CASTER_0045CD60, &args![this]).u32();
        e.call(MAGIC_CASTER_GET_CASTER_AS_ACTOR, &args![caster])
            .u32()
    };
    (actor, caster)
}

// Translated from 0080c8d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080c8d0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `DemoralizeEffect::Copy` (Xbox PDB): a new 0x4c-byte effect (`operator new`, then
/// `0080c890` with the caster +0x28, spell +8 and effect item +0xc; null when the
/// allocation fails) with this effect's byte at +0x48, given this effect's data through
/// `CopyData` (slot +0x44); returns it.
pub fn demoralize_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    let base = this.addr();
    let copy = if block == 0 {
        0
    } else {
        let item = e.mem.u32(base + 0xc);
        let spell = e.mem.u32(base + 8);
        let caster = e.mem.u32(base + 0x28);
        e.call(CONSTRUCTOR_0080C890, &args![block, caster, spell, item])
            .u32()
    };
    let started = e.mem.u8(base + 0x48);
    e.mem.set_u8(copy.wrapping_add(0x48), started);
    e.vcall(base, 0x44, &args![copy]);
    copy
}

// Translated from 0080c980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DemoralizeEffect::CheckTarget` (Xbox PDB): false without a target actor, when its slot
/// +0x22c (with 0) or +0x2e8 is true, or when `CheckTargetLevelMagnitude` refuses its level
/// for the magnitude; otherwise true for a `Creature` (`__RTDynamicCast` from
/// `MagicTarget`) and, for others, unless `00608d80` is true.
pub fn demoralize_effect_check_target(e: &mut Engine, this: Ptr, target: u32) -> bool {
    let actor = if target == 0 {
        0
    } else {
        e.call(MAGIC_TARGET_GET_TARGET_AS_ACTOR, &args![target])
            .u32()
    };
    if actor == 0 || e.vcall(actor, 0x22c, &args![0u32]).bool() || e.vcall(actor, 0x2e8, &[]).bool()
    {
        return false;
    }
    let magnitude = x87_store32(e.call(EFFECT_MAGNITUDE_00495460, &args![this]).f64());
    let level = u32::from(e.call(ACTOR_LEVEL_0087F9F0, &args![actor]).u16());
    if !e
        .call(CHECK_TARGET_LEVEL_MAGNITUDE, &args![level, magnitude])
        .bool()
    {
        return false;
    }
    let creature = e
        .call(
            RT_DYNAMIC_CAST,
            &args![target, 0u32, MAGIC_TARGET_TYPE, CREATURE_TYPE, 0u32],
        )
        .u32();
    creature != 0 || !e.call(ACTOR_TEST_00608D80, &args![actor]).bool()
}

// Translated from 0080ca50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DemoralizeEffect::Start` (Xbox PDB): with a target actor and a caster actor: an actor
/// with a combat controller (slot +0x428) passes the caster to `008d0600` (with 0); one
/// without that is not fleeing (`Actor::IsFleeing` with 0) ends its interrupt package
/// (with 0) and flees from the caster (slot +0x410 with the caster, 1, 1, 1, 0, 0 and -1.0
/// twice). Either way the byte at +0x48 is set.
pub fn demoralize_effect_start(e: &mut Engine, this: Ptr) {
    let (actor, caster) = actor_and_caster(e, this.addr());
    if actor == 0 || caster == 0 {
        return;
    }
    if e.vcall(actor, 0x428, &[]).u32() != 0 {
        let controller = e.vcall(actor, 0x428, &[]).u32();
        e.call(COMBAT_CONTROLLER_008D0600, &args![controller, caster, 0u32]);
        e.mem.set_u8(this.addr() + 0x48, 1);
        return;
    }
    if e.call(ACTOR_IS_FLEEING, &args![actor, 0u32]).bool() {
        return;
    }
    e.call(ACTOR_END_INTERRUPT_PACKAGE, &args![actor, 0u32]);
    let minus_one = x87_quiet_bits(e.global::<u32>(MINUS_ONE));
    e.vcall(
        actor,
        0x410,
        &args![caster, 1u32, 1u32, 1u32, 0u32, 0u32, minus_one, minus_one],
    );
    e.mem.set_u8(this.addr() + 0x48, 1);
}

// Translated from 0080cb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DemoralizeEffect::Finish` (Xbox PDB): only once started (+0x48) and for a target
/// actor: when `00493bb0` is true, its slot +0x434 with the caster and, with a caster and
/// the actor's `008d8520` object, that object's slot +0x33c (actor, caster, 1, eight 0s, 1,
/// 0); otherwise a fleeing actor (`Actor::IsFleeing` with 0) stops fleeing the caster
/// (`Actor::RemoveFleeTarget`).
pub fn demoralize_effect_finish(e: &mut Engine, this: Ptr) {
    if e.mem.u8(this.addr() + 0x48) == 0 {
        return;
    }
    let (actor, caster) = actor_and_caster(e, this.addr());
    if actor == 0 {
        return;
    }
    if e.call(ACTOR_TEST_00493BB0, &args![actor]).bool() {
        e.vcall(actor, 0x434, &args![caster]);
        if caster != 0 && e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32() != 0 {
            let object = e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32();
            e.vcall(
                object,
                0x33c,
                &args![
                    actor, caster, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 1u32, 0u32
                ],
            );
        }
    } else if e.call(ACTOR_IS_FLEEING, &args![actor, 0u32]).bool() {
        e.call(ACTOR_REMOVE_FLEE_TARGET, &args![actor, caster]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_c8d0, demoralize_effect_copy(Ptr) -> u32),
        entry!(0x0080_c980, demoralize_effect_check_target(Ptr, u32) -> bool),
        entry!(0x0080_ca50, demoralize_effect_start(Ptr)),
        entry!(0x0080_cb60, demoralize_effect_finish(Ptr)),
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

    /// A target actor (the given slots) and the caster actor 0x5100.
    fn rig(slots: &[(u32, u32)]) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.set_global(MINUS_ONE, (-1.0f32).to_bits());
        let actor = object(&mut e, slots);
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        answer(&mut e, EFFECT_CASTER_0045CD60, 0x7100);
        answer(&mut e, MAGIC_CASTER_GET_CASTER_AS_ACTOR, 0x5100);
        for addr in [
            COMBAT_CONTROLLER_008D0600,
            ACTOR_END_INTERRUPT_PACKAGE,
            ACTOR_REMOVE_FLEE_TARGET,
        ] {
            answer(&mut e, addr, 0);
        }
        let this = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        (e, actor, this)
    }

    #[test]
    fn copy_carries_the_started_byte() {
        let mut e = Engine::new();
        let this = object(&mut e, &[(0x44, 0)]);
        e.mem.set_u32(this + 8, 0x08);
        e.mem.set_u32(this + 0xc, 0x0c);
        e.mem.set_u32(this + 0x28, 0x28);
        e.mem.set_u8(this + 0x48, 1);
        let block = e.mem.alloc(0x4c);
        answer(&mut e, OPERATOR_NEW, block);
        answer(&mut e, CONSTRUCTOR_0080C890, block);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_c8d0, &args![this]).u32(), block);
        assert_eq!(
            calls(&e, CONSTRUCTOR_0080C890),
            vec![vec![block, 0x28, 0x08, 0x0c]]
        );
        assert_eq!(e.mem.u8(block + 0x48), 1);
        assert_eq!(calls(&e, slot(&e, this, 0x44)), vec![vec![this, block]]);
    }

    #[test]
    fn check_target_accepts_creatures_and_others_unless_00608d80() {
        for (creature, test, expected) in [(true, 1u32, true), (false, 0, true), (false, 1, false)]
        {
            let (mut e, actor, this) = rig(&[(0x22c, 0), (0x2e8, 0)]);
            e.register(EFFECT_MAGNITUDE_00495460, |_, _| Ret {
                st0: 30.0,
                ..Ret::default()
            });
            answer(&mut e, ACTOR_LEVEL_0087F9F0, 5);
            answer(&mut e, CHECK_TARGET_LEVEL_MAGNITUDE, 1);
            answer(&mut e, RT_DYNAMIC_CAST, if creature { actor } else { 0 });
            answer(&mut e, ACTOR_TEST_00608D80, test);
            let result = e.call(0x0080_c980, &args![this, 0x2000u32]).bool();
            assert_eq!(result, expected);
            assert_eq!(
                calls(&e, CHECK_TARGET_LEVEL_MAGNITUDE),
                vec![vec![5, 30.0f32.to_bits()]]
            );
            assert_eq!(
                calls(&e, RT_DYNAMIC_CAST),
                vec![vec![0x2000, 0, MAGIC_TARGET_TYPE, CREATURE_TYPE, 0]]
            );
        }
        // A refused level, or a busy actor: false.
        let (mut e, _, this) = rig(&[(0x22c, 0), (0x2e8, 0)]);
        e.register(EFFECT_MAGNITUDE_00495460, |_, _| Ret::default());
        answer(&mut e, ACTOR_LEVEL_0087F9F0, 5);
        answer(&mut e, CHECK_TARGET_LEVEL_MAGNITUDE, 0);
        assert!(!e.call(0x0080_c980, &args![this, 0x2000u32]).bool());
        let (mut e, _, this) = rig(&[(0x22c, 1)]);
        assert!(!e.call(0x0080_c980, &args![this, 0x2000u32]).bool());
    }

    #[test]
    fn start_sets_a_controller_or_makes_the_actor_flee() {
        let (mut e, _, this) = rig(&[(0x428, 0x4280)]);
        e.call(0x0080_ca50, &args![this]);
        assert_eq!(
            calls(&e, COMBAT_CONTROLLER_008D0600),
            vec![vec![0x4280, 0x5100, 0]]
        );
        assert_eq!(e.mem.u8(this + 0x48), 1);
        let (mut e, actor, this) = rig(&[(0x428, 0), (0x410, 0)]);
        answer(&mut e, ACTOR_IS_FLEEING, 0);
        e.call(0x0080_ca50, &args![this]);
        assert_eq!(calls(&e, ACTOR_END_INTERRUPT_PACKAGE), vec![vec![actor, 0]]);
        let minus_one = (-1.0f32).to_bits();
        assert_eq!(
            calls(&e, slot(&e, actor, 0x410)),
            vec![vec![actor, 0x5100, 1, 1, 1, 0, 0, minus_one, minus_one]]
        );
        assert_eq!(e.mem.u8(this + 0x48), 1);
        // Already fleeing: nothing, and not started.
        let (mut e, _, this) = rig(&[(0x428, 0), (0x410, 0)]);
        answer(&mut e, ACTOR_IS_FLEEING, 1);
        e.call(0x0080_ca50, &args![this]);
        assert!(calls(&e, ACTOR_END_INTERRUPT_PACKAGE).is_empty());
        assert_eq!(e.mem.u8(this + 0x48), 0);
    }

    #[test]
    fn finish_restores_the_actor() {
        let (mut e, actor, this) = rig(&[(0x434, 0)]);
        e.mem.set_u8(this + 0x48, 1);
        answer(&mut e, ACTOR_TEST_00493BB0, 1);
        let object = object(&mut e, &[(0x33c, 0)]);
        answer(&mut e, ACTOR_OBJECT_008D8520, object);
        e.call(0x0080_cb60, &args![this]);
        assert_eq!(calls(&e, slot(&e, actor, 0x434)), vec![vec![actor, 0x5100]]);
        assert_eq!(
            calls(&e, slot(&e, object, 0x33c)),
            vec![vec![object, actor, 0x5100, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0]]
        );
        let (mut e, actor, this) = rig(&[]);
        e.mem.set_u8(this + 0x48, 1);
        answer(&mut e, ACTOR_TEST_00493BB0, 0);
        answer(&mut e, ACTOR_IS_FLEEING, 1);
        e.call(0x0080_cb60, &args![this]);
        assert_eq!(
            calls(&e, ACTOR_REMOVE_FLEE_TARGET),
            vec![vec![actor, 0x5100]]
        );
        // Not started: nothing.
        let (mut e, _, this) = rig(&[]);
        e.call(0x0080_cb60, &args![this]);
        assert!(calls(&e, EFFECT_TARGET_0059BB30).is_empty());
    }
}
