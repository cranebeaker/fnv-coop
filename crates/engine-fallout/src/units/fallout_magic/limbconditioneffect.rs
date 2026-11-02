//! `fallout/magic/limbconditioneffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `LimbConditionEffect` (Xbox PDB, 0x50 bytes) is a `ValueModifierEffect` with an
//! `ActorValue::Index eTargetBodyPart` at +0x4C (-1: every limb).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_magic::activeeffect::ActiveEffect;

/// `LimbConditionEffect`'s vtable (`0107a3fc`).
const LIMB_CONDITION_EFFECT_VTABLE: u32 = 0x0107_a3fc;
/// The `ValueModifierEffect` constructor body `0082ba60` (`thiscall`, `RET 0xC`).
const VALUE_MODIFIER_EFFECT_CONSTRUCTOR: u32 = 0x0082_ba60;
/// `ValueModifierEffect::CopyData` (engine map, `0082bbe0`, `thiscall`, `RET 4`).
const VALUE_MODIFIER_EFFECT_COPY_DATA: u32 = 0x0082_bbe0;
/// `ValueModifierEffect::ModifyActorValue` (engine map, `0082ca50`, `thiscall`, `RET 0xC`:
/// actor, `float` value, actor value index).
const VALUE_MODIFIER_EFFECT_MODIFY_ACTOR_VALUE: u32 = 0x0082_ca50;
/// `00401000`: the game's `operator new` (`cdecl`, one word).
const ALLOCATE: u32 = 0x0040_1000;
/// `__RTDynamicCast` and the RTTI type descriptors `.?AVActiveEffect@@` and
/// `.?AVLimbConditionEffect@@`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_ACTIVE_EFFECT: u32 = 0x011a_0d1c;
const TYPE_LIMB_CONDITION_EFFECT: u32 = 0x011a_118c;
/// `00403e20` (`thiscall`): the address of a `float` setting's value.
const SETTING_VALUE: u32 = 0x0040_3e20;
/// The `float` setting read (and its product left unused) for a single limb (`011d1860`).
const SINGLE_LIMB_SETTING: u32 = 0x011d_1860;
/// The `float` setting that scales the value for every limb (`011d17ac`).
const ALL_LIMBS_SETTING: u32 = 0x011d_17ac;
/// The limb condition actor values run for "every limb": indices 0x19 to 0x1F.
const FIRST_LIMB_VALUE: u32 = 0x19;
const END_LIMB_VALUE: u32 = 0x20;

// Translated from 0080f060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `LimbConditionEffect` constructor (`thiscall`, `RET 0xC`): the `ValueModifierEffect`
/// body (`0082ba60`) with the caster, spell and effect item, its vtable (`0107a3fc`) and
/// the body part +0x4C set to -1. Returns `this`.
pub fn fn_0080f060(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(
        VALUE_MODIFIER_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, effect],
    );
    e.mem.set_u32(this.addr(), LIMB_CONDITION_EFFECT_VTABLE);
    e.mem.set_u32(this.addr().wrapping_add(0x4c), 0xffff_ffff);
    this
}

// Translated from 0080f0a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080f0a0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `LimbConditionEffect::Copy` (Xbox PDB): a new `LimbConditionEffect` (0x50 bytes,
/// `00401000`) with this effect's caster (+0x28), spell (+8) and effect item (+0xC), or
/// null when the allocation failed; this effect's slot +0x44 (`CopyData`) copies itself into
/// it. Returns it.
pub fn limb_condition_effect_copy(e: &mut Engine, this: Ptr<ActiveEffect>) -> u32 {
    let block = e.call(ALLOCATE, &args![0x50u32]).u32();
    let copy = if block != 0 {
        let caster = e.get(this, ActiveEffect::pCaster);
        let spell = e.get(this, ActiveEffect::pSpell);
        let effect = e.get(this, ActiveEffect::pEffect);
        fn_0080f060(e, Ptr::new(block), caster, spell, effect).addr()
    } else {
        0
    };
    e.vcall(this.addr(), 0x44, &args![copy]);
    copy
}

// Translated from 0080f150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LimbConditionEffect::CopyData` (Xbox PDB; `thiscall`, `RET 4`):
/// `ValueModifierEffect::CopyData` into the target, then, when the target is a
/// `LimbConditionEffect`, copies the body part (+0x4C) into it.
pub fn limb_condition_effect_copy_data(e: &mut Engine, this: Ptr, target: u32) {
    e.call(VALUE_MODIFIER_EFFECT_COPY_DATA, &args![this, target]);
    let limb_effect = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                target,
                0u32,
                TYPE_ACTIVE_EFFECT,
                TYPE_LIMB_CONDITION_EFFECT,
                0u32
            ],
        )
        .u32();
    if limb_effect != 0 {
        let part = e.mem.u32(this.addr().wrapping_add(0x4c));
        e.mem.set_u32(limb_effect.wrapping_add(0x4c), part);
    }
}

// Translated from 0080f1a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080f1a0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map (`cdecl`, three words): a new `LimbConditionEffect` (0x50
/// bytes, `00401000`) with the caster, spell and effect item, or null when the allocation
/// failed.
pub fn fn_0080f1a0(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x50u32]).u32();
    if block != 0 {
        fn_0080f060(e, Ptr::new(block), caster, spell, effect).addr()
    } else {
        0
    }
}

// Translated from 0080f220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `LimbConditionEffect::ModifyActorValue` (Xbox PDB; `thiscall`, `RET 0xC`: the actor, the
/// `float` value, and a third word it never reads). With a body part (+0x4C not -1):
/// `ValueModifierEffect::ModifyActorValue(actor, value, part)` (the setting at `011d1860` is
/// read and multiplied with the value, and the product is left unused). Without one: the
/// value times the setting at `011d17ac`, for each limb condition value 0x19 to 0x1F.
pub fn limb_condition_effect_modify_actor_value(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    value: f32,
    _unused_3: u32,
) {
    let part = e.mem.u32(this.addr().wrapping_add(0x4c));
    if part != 0xffff_ffff {
        let setting = e.call(SETTING_VALUE, &args![SINGLE_LIMB_SETTING]).u32();
        let _unused_product = x87_store32(x87_mul(
            x87_load32(value.to_bits()),
            x87_m32(e.mem.u32(setting)),
        ));
        let value = x87_quiet_bits(value.to_bits());
        e.call(
            VALUE_MODIFIER_EFFECT_MODIFY_ACTOR_VALUE,
            &args![this, actor, value, part],
        );
    } else {
        let setting = e.call(SETTING_VALUE, &args![ALL_LIMBS_SETTING]).u32();
        let scaled = x87_store32(x87_mul(
            x87_load32(value.to_bits()),
            x87_m32(e.mem.u32(setting)),
        ));
        for index in FIRST_LIMB_VALUE..END_LIMB_VALUE {
            e.call(
                VALUE_MODIFIER_EFFECT_MODIFY_ACTOR_VALUE,
                &args![this, actor, x87_quiet_bits(scaled), index],
            );
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_f060, fn_0080f060(Ptr, u32, u32, u32) -> Ptr),
        entry!(
            0x0080_f0a0,
            limb_condition_effect_copy(Ptr<ActiveEffect>) -> u32
        ),
        entry!(0x0080_f150, limb_condition_effect_copy_data(Ptr, u32)),
        entry!(0x0080_f1a0, fn_0080f1a0(u32, u32, u32) -> u32),
        entry!(
            0x0080_f220,
            limb_condition_effect_modify_actor_value(Ptr, u32, f32, u32)
        ),
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
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn constructor_copy_and_create() {
        let mut e = Engine::new();
        e.register(VALUE_MODIFIER_EFFECT_CONSTRUCTOR, |_, _| Ret::default());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0310_0000, |_, _| Ret::default());
        let mut slots = vec![0u32; 0x48 / 4];
        slots[0x44 / 4] = 0x0310_0000;
        e.put_vtable(0x0300_0000, &slots);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x50);
        assert_eq!(
            e.call(0x0080_f060, &args![this, 1u32, 2u32, 3u32]).u32(),
            this
        );
        assert_eq!(e.mem.u32(this), LIMB_CONDITION_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x4c), 0xffff_ffff);
        e.mem.set_u32(this, 0x0300_0000);
        e.mem.set_u32(this + 8, 0x22);
        e.mem.set_u32(this + 0xc, 0x33);
        e.mem.set_u32(this + 0x28, 0x11);
        let copy = e.call(0x0080_f0a0, &args![this]).u32();
        assert_eq!(e.mem.u32(copy), LIMB_CONDITION_EFFECT_VTABLE);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_CONSTRUCTOR).last(),
            Some(&vec![copy, 0x11, 0x22, 0x33])
        );
        assert_eq!(calls(&e, 0x0310_0000), vec![vec![this, copy]]);
        let made = e.call(0x0080_f1a0, &args![4u32, 5u32, 6u32]).u32();
        assert_eq!(e.mem.u32(made + 0x4c), 0xffff_ffff);
        assert_eq!(calls(&e, ALLOCATE), vec![vec![0x50], vec![0x50]]);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x0080_f0a0, &args![this]).u32(), 0);
        assert_eq!(e.call(0x0080_f1a0, &args![4u32, 5u32, 6u32]).u32(), 0);
    }

    #[test]
    fn copy_data_copies_the_body_part_into_limb_effects_only() {
        let mut e = Engine::new();
        e.register(VALUE_MODIFIER_EFFECT_COPY_DATA, |_, _| Ret::default());
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x50);
        let target = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x4c, 0x1b);
        e.call(0x0080_f150, &args![this, target]);
        assert_eq!(e.mem.u32(target + 0x4c), 0x1b);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![
                target,
                0,
                TYPE_ACTIVE_EFFECT,
                TYPE_LIMB_CONDITION_EFFECT,
                0
            ]]
        );
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_COPY_DATA),
            vec![vec![this, target]]
        );
        e.register(RT_DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.mem.set_u32(this + 0x4c, 0x1c);
        e.call(0x0080_f150, &args![this, target]);
        assert_eq!(e.mem.u32(target + 0x4c), 0x1b);
    }

    #[test]
    fn modify_actor_value_one_limb_or_all_seven() {
        let mut e = Engine::new();
        e.register(VALUE_MODIFIER_EFFECT_MODIFY_ACTOR_VALUE, |_, _| {
            Ret::default()
        });
        e.register(SETTING_VALUE, |e, a| {
            let p = e.mem.alloc(4);
            e.mem
                .set_f32(p, if a[0] == ALL_LIMBS_SETTING { 2.0 } else { 3.0 });
            p.into_ret()
        });
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x4c, 0x1a);
        e.call(0x0080_f220, &args![this, 0x7000u32, 1.5f32, 9u32]);
        assert_eq!(calls(&e, SETTING_VALUE), vec![vec![SINGLE_LIMB_SETTING]]);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_MODIFY_ACTOR_VALUE),
            vec![vec![this, 0x7000, 1.5f32.to_bits(), 0x1a]]
        );
        e.mem.set_u32(this + 0x4c, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(0x0080_f220, &args![this, 0x7000u32, 1.5f32, 9u32]);
        assert_eq!(calls(&e, SETTING_VALUE), vec![vec![ALL_LIMBS_SETTING]]);
        let all = calls(&e, VALUE_MODIFIER_EFFECT_MODIFY_ACTOR_VALUE);
        let indices: Vec<u32> = all.iter().map(|c| c[3]).collect();
        assert_eq!(indices, (0x19..0x20).collect::<Vec<u32>>());
        assert!(all
            .iter()
            .all(|c| c[2] == 3.0f32.to_bits() && c[1] == 0x7000));
    }
}
