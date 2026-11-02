//! `fallout/magic/calmeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `0080a900` (thiscall on a new 0x4c-byte block: caster, spell, effect item; unnamed):
/// the `CalmEffect` constructor `Copy` uses.
const CONSTRUCTOR_0080A900: u32 = 0x0080_a900;
/// `MagicTarget::GetTargetAsActor` (`00822b40`).
const MAGIC_TARGET_GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `0059bb30` (thiscall): the effect's target (+0x24).
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `00493bb0` (thiscall on the actor, `AL`; unnamed, `animation.cpp`).
const ACTOR_TEST_00493BB0: u32 = 0x0049_3bb0;
/// `00495460` (thiscall on the effect, returns `ST0`; unnamed): its magnitude.
const EFFECT_MAGNITUDE_00495460: u32 = 0x0049_5460;
/// `0087f9f0` (thiscall on the actor, returns `AX`; unnamed, `actor.cpp`): its level.
const ACTOR_LEVEL_0087F9F0: u32 = 0x0087_f9f0;
/// `MagicFormulas::CheckTargetLevelMagnitude` (`006494d0`, cdecl: level, magnitude).
const CHECK_TARGET_LEVEL_MAGNITUDE: u32 = 0x0064_94d0;
/// `ValueModifierEffect::Start` (`0082c000`).
const VALUE_MODIFIER_EFFECT_START: u32 = 0x0082_c000;
/// `ValueModifierEffect::Update` (`0082c660`, thiscall: one `float`).
const VALUE_MODIFIER_EFFECT_UPDATE: u32 = 0x0082_c660;
/// `ValueModifierEffect::Finish` (`0082c810`).
const VALUE_MODIFIER_EFFECT_FINISH: u32 = 0x0082_c810;
/// `CombatController::StartCalmed` (`009807d0`).
const COMBAT_CONTROLLER_START_CALMED: u32 = 0x0098_07d0;
/// `0047c850` (thiscall on the combat controller, `AL`; unnamed).
const COMBAT_CONTROLLER_TEST_0047C850: u32 = 0x0047_c850;
/// `00483710` (thiscall on the combat controller; unnamed).
const COMBAT_CONTROLLER_END_00483710: u32 = 0x0048_3710;
/// `ActiveEffect::Dispel` (`00804210`, thiscall: one byte).
const ACTIVE_EFFECT_DISPEL: u32 = 0x0080_4210;

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

// Translated from 0080a930 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080a930 - the exception-unwinding frame (FS:[0]) is not translated.
/// `CalmEffect::Copy` (Xbox PDB): a new 0x4c-byte effect (`operator new`, then `0080a900`
/// with the caster +0x28, spell +8 and effect item +0xc; null when the allocation fails),
/// given this effect's data through `CopyData` (slot +0x44); returns it.
pub fn calm_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x4cu32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let base = this.addr();
        let item = e.mem.u32(base + 0xc);
        let spell = e.mem.u32(base + 8);
        let caster = e.mem.u32(base + 0x28);
        e.call(CONSTRUCTOR_0080A900, &args![block, caster, spell, item])
            .u32()
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

// Translated from 0080a9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CalmEffect::CheckTarget` (Xbox PDB): true for a target actor whose slots +0x22c (with
/// 0) and +0x2e8 are false, `00493bb0` true, and `CheckTargetLevelMagnitude` accepts its
/// level (`0087f9f0`) for the effect's magnitude (`00495460`, stored as a `float`).
pub fn calm_effect_check_target(e: &mut Engine, this: Ptr, target: u32) -> bool {
    let actor = if target == 0 {
        0
    } else {
        e.call(MAGIC_TARGET_GET_TARGET_AS_ACTOR, &args![target])
            .u32()
    };
    if actor == 0
        || e.vcall(actor, 0x22c, &args![0u32]).bool()
        || e.vcall(actor, 0x2e8, &[]).bool()
        || !e.call(ACTOR_TEST_00493BB0, &args![actor]).bool()
    {
        return false;
    }
    let magnitude = e.call(EFFECT_MAGNITUDE_00495460, &args![this]).f64();
    let magnitude = x87_store32(magnitude);
    let level = u32::from(e.call(ACTOR_LEVEL_0087F9F0, &args![actor]).u16());
    e.call(CHECK_TARGET_LEVEL_MAGNITUDE, &args![level, magnitude])
        .bool()
}

// Translated from 0080aa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CalmEffect::Start` (Xbox PDB): `ValueModifierEffect::Start`, then the target actor's
/// combat controller (slot +0x428), when it has one, starts being calmed.
pub fn calm_effect_start(e: &mut Engine, this: Ptr) {
    e.call(VALUE_MODIFIER_EFFECT_START, &args![this]);
    let actor = target_actor(e, this.addr());
    if actor != 0 && e.vcall(actor, 0x428, &[]).u32() != 0 {
        let controller = e.vcall(actor, 0x428, &[]).u32();
        e.call(COMBAT_CONTROLLER_START_CALMED, &args![controller]);
    }
}

// Translated from 0080ab10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CalmEffect::Update` (Xbox PDB): `ValueModifierEffect::Update` with `seconds` (`FLD`/
/// `FSTP`: a signalling NaN arrives quiet); then, for a target actor, the effect is
/// dispelled when its slot +0x22c (with 0) or +0x2e8 is true, or when its combat
/// controller (slot +0x428) says no through `0047c850` (true without a controller).
pub fn calm_effect_update(e: &mut Engine, this: Ptr, seconds: f32) {
    e.call(
        VALUE_MODIFIER_EFFECT_UPDATE,
        &args![this, x87_quiet_bits(seconds.to_bits())],
    );
    let actor = target_actor(e, this.addr());
    if actor == 0 {
        return;
    }
    let calm = if actor != 0 && e.vcall(actor, 0x428, &[]).u32() != 0 {
        let controller = e.vcall(actor, 0x428, &[]).u32();
        e.call(COMBAT_CONTROLLER_TEST_0047C850, &args![controller])
            .u8()
    } else {
        1
    };
    if e.vcall(actor, 0x22c, &args![0u32]).bool() || e.vcall(actor, 0x2e8, &[]).bool() || calm == 0
    {
        e.call(ACTIVE_EFFECT_DISPEL, &args![this, 0u32]);
    }
}

// Translated from 0080abf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `CalmEffect::Finish` (Xbox PDB): `ValueModifierEffect::Finish`, then `00483710` on the
/// target actor's combat controller (slot +0x428) when it has one.
pub fn calm_effect_finish(e: &mut Engine, this: Ptr) {
    e.call(VALUE_MODIFIER_EFFECT_FINISH, &args![this]);
    let actor = target_actor(e, this.addr());
    if actor != 0 && e.vcall(actor, 0x428, &[]).u32() != 0 {
        let controller = e.vcall(actor, 0x428, &[]).u32();
        e.call(COMBAT_CONTROLLER_END_00483710, &args![controller]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_a930, calm_effect_copy(Ptr) -> u32),
        entry!(0x0080_a9e0, calm_effect_check_target(Ptr, u32) -> bool),
        entry!(0x0080_aa90, calm_effect_start(Ptr)),
        entry!(0x0080_ab10, calm_effect_update(Ptr, f32)),
        entry!(0x0080_abf0, calm_effect_finish(Ptr)),
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

    fn rig() -> Engine {
        let mut e = Engine::new();
        for addr in [
            VALUE_MODIFIER_EFFECT_START,
            VALUE_MODIFIER_EFFECT_UPDATE,
            VALUE_MODIFIER_EFFECT_FINISH,
            ACTIVE_EFFECT_DISPEL,
            COMBAT_CONTROLLER_START_CALMED,
            COMBAT_CONTROLLER_END_00483710,
        ] {
            answer(&mut e, addr, 0);
        }
        answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn copy_allocates_constructs_and_copies_data() {
        let mut e = rig();
        let this = object(&mut e, &[(0x44, 0)]);
        e.mem.set_u32(this + 8, 0x08);
        e.mem.set_u32(this + 0xc, 0x0c);
        e.mem.set_u32(this + 0x28, 0x28);
        answer(&mut e, OPERATOR_NEW, 0x9000);
        answer(&mut e, CONSTRUCTOR_0080A900, 0x9004);
        assert_eq!(e.call(0x0080_a930, &args![this]).u32(), 0x9004);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x4c]]);
        assert_eq!(
            calls(&e, CONSTRUCTOR_0080A900),
            vec![vec![0x9000, 0x28, 0x08, 0x0c]]
        );
        assert_eq!(calls(&e, slot(&e, this, 0x44)), vec![vec![this, 0x9004]]);
        answer(&mut e, OPERATOR_NEW, 0);
        assert_eq!(e.call(0x0080_a930, &args![this]).u32(), 0);
        assert_eq!(calls(&e, slot(&e, this, 0x44))[1], vec![this, 0]);
    }

    #[test]
    fn check_target_asks_the_level_formula() {
        for (busy, test, accepted, expected) in [
            (0u32, 1u32, 1u32, true),
            (0, 1, 0, false),
            (1, 1, 1, false),
            (0, 0, 1, false),
        ] {
            let mut e = rig();
            let actor = object(&mut e, &[(0x22c, busy), (0x2e8, 0)]);
            answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
            answer(&mut e, ACTOR_TEST_00493BB0, test);
            e.register(EFFECT_MAGNITUDE_00495460, |_, _| Ret {
                st0: 25.0,
                ..Ret::default()
            });
            answer(&mut e, ACTOR_LEVEL_0087F9F0, 0x0003_0007);
            answer(&mut e, CHECK_TARGET_LEVEL_MAGNITUDE, accepted);
            let result = e.call(0x0080_a9e0, &args![0x1000u32, 0x2000u32]).bool();
            assert_eq!(result, expected);
            if busy == 0 && test != 0 {
                assert_eq!(
                    calls(&e, CHECK_TARGET_LEVEL_MAGNITUDE),
                    vec![vec![7, 25.0f32.to_bits()]]
                );
            }
        }
        // No target: false without asking.
        let mut e = rig();
        assert!(!e.call(0x0080_a9e0, &args![0x1000u32, 0u32]).bool());
        assert!(calls(&e, MAGIC_TARGET_GET_TARGET_AS_ACTOR).is_empty());
    }

    #[test]
    fn start_calms_the_combat_controller() {
        let mut e = rig();
        let actor = object(&mut e, &[(0x428, 0x4280)]);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        e.call(0x0080_aa90, &args![0x1000u32]);
        assert_eq!(calls(&e, VALUE_MODIFIER_EFFECT_START), vec![vec![0x1000]]);
        assert_eq!(
            calls(&e, COMBAT_CONTROLLER_START_CALMED),
            vec![vec![0x4280]]
        );
    }

    #[test]
    fn update_dispels_when_the_target_stops_being_calm() {
        for (busy, calm, dispels) in [(0u32, 1u32, false), (1, 1, true), (0, 0, true)] {
            let mut e = rig();
            let actor = object(&mut e, &[(0x22c, busy), (0x2e8, 0), (0x428, 0x4280)]);
            answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
            answer(&mut e, COMBAT_CONTROLLER_TEST_0047C850, calm);
            e.call(0x0080_ab10, &args![0x1000u32, 0x7f80_0001u32]);
            assert_eq!(
                calls(&e, VALUE_MODIFIER_EFFECT_UPDATE),
                vec![vec![0x1000, 0x7fc0_0001]]
            );
            assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL).len(), usize::from(dispels));
        }
    }

    #[test]
    fn finish_ends_the_combat_controller() {
        let mut e = rig();
        let actor = object(&mut e, &[(0x428, 0x4280)]);
        answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
        e.call(0x0080_abf0, &args![0x1000u32]);
        assert_eq!(calls(&e, VALUE_MODIFIER_EFFECT_FINISH), vec![vec![0x1000]]);
        assert_eq!(
            calls(&e, COMBAT_CONTROLLER_END_00483710),
            vec![vec![0x4280]]
        );
    }
}
