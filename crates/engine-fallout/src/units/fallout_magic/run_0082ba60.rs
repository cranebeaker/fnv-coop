//! `fallout/magic/run_0082ba60` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_magic::activeeffect::ActiveEffect;

/// `ActiveEffect`'s constructor body (`00803d30`: `this`; caster, magic
/// item, effect item).
const ACTIVE_EFFECT_CONSTRUCT: u32 = 0x0080_3d30;
/// `ValueModifierEffect`'s vtable (RTTI `.?AVValueModifierEffect@@`).
const VALUE_MODIFIER_EFFECT_VTABLE: u32 = 0x0107_c68c;
/// `EffectItem`'s effect setting (`00825c00`, `ECX` = effect item).
const EFFECT_ITEM_SETTING: u32 = 0x0082_5c00;
/// The setting's test for a second actor value (`00403e80`) and that
/// value (`00586150`), `ECX` = setting.
const SETTING_HAS_SECOND_VALUE: u32 = 0x0040_3e80;
const SETTING_SECOND_VALUE: u32 = 0x0058_6150;
/// The effect item's actor value (`00403ea0`, `ECX` = effect item).
const EFFECT_ITEM_ACTOR_VALUE: u32 = 0x0040_3ea0;
/// The setting's flag test (`00403e60`, `ECX` = setting; flag).
const SETTING_HAS_FLAG: u32 = 0x0040_3e60;
/// `ValueModifierEffect::eWhichActorValue` (Xbox PDB), +0x48.
const WHICH_ACTOR_VALUE: u32 = 0x48;

// Translated from 0082ba60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueModifierEffect`'s constructor (`this`; caster, magic item, effect
/// item): the `ActiveEffect` part, the vtable, the actor value
/// (`eWhichActorValue`, +0x48) from the setting's second value when it has
/// one (`00403e80`, `00586150`) or else the effect item's, and magnitude
/// 1.0 when the setting has flag 0x100. Returns `this`.
// Translation incomplete: 0082ba60 - SEH exception frame (FS:[0]) that destroys the ActiveEffect part on unwinding is not modelled.
pub fn fn_0082ba60(
    e: &mut Engine,
    this: Ptr<ActiveEffect>,
    caster: u32,
    spell: u32,
    effect: u32,
) -> Ptr<ActiveEffect> {
    e.call(ACTIVE_EFFECT_CONSTRUCT, &args![this, caster, spell, effect]);
    e.mem.set_u32(this.addr(), VALUE_MODIFIER_EFFECT_VTABLE);
    let setting = e.call(EFFECT_ITEM_SETTING, &args![effect]).u32();
    let value = if e.call(SETTING_HAS_SECOND_VALUE, &args![setting]).bool() {
        e.call(SETTING_SECOND_VALUE, &args![setting]).u32()
    } else {
        e.call(EFFECT_ITEM_ACTOR_VALUE, &args![effect]).u32()
    };
    e.mem
        .set_u32(this.addr().wrapping_add(WHICH_ACTOR_VALUE), value);
    let setting = e.call(EFFECT_ITEM_SETTING, &args![effect]).u32();
    if e.call(SETTING_HAS_FLAG, &args![setting, 0x100u32]).bool() {
        e.set(this, ActiveEffect::fMagnitude, 1.0);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(
        0x0082_ba60,
        fn_0082ba60(Ptr<ActiveEffect>, u32, u32, u32) -> Ptr<ActiveEffect>
    )]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    #[test]
    fn picks_the_actor_value_and_magnitude() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_CONSTRUCT, |e, a| {
            e.mem.set_f32(a[0] + 0x1c, 7.0);
            ret(a[0])
        });
        e.register(EFFECT_ITEM_SETTING, |_, _| ret(0x500));
        e.register(SETTING_SECOND_VALUE, |_, _| ret(0x2a));
        e.register(EFFECT_ITEM_ACTOR_VALUE, |_, _| ret(0x10));
        e.register(SETTING_HAS_SECOND_VALUE, |_, _| ret(1));
        e.register(SETTING_HAS_FLAG, |_, a| ret(u32::from(a[1] == 0x100)));
        let effect = e.mem.alloc(0x4c);
        e.call_log = Some(vec![]);
        let words = args![effect, 1u32, 2u32, 3u32];
        assert_eq!(e.call(0x0082_ba60, &words).u32(), effect);
        assert_eq!(e.mem.u32(effect), VALUE_MODIFIER_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(effect + 0x48), 0x2a);
        assert_eq!(e.mem.f32(effect + 0x1c), 1.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (ACTIVE_EFFECT_CONSTRUCT, vec![effect, 1, 2, 3]));
        // No second value and no flag: the effect item's value, magnitude kept.
        e.register(SETTING_HAS_SECOND_VALUE, |_, _| ret(0));
        e.register(SETTING_HAS_FLAG, |_, _| ret(0));
        fn_0082ba60(&mut e, Ptr::new(effect), 1, 2, 3);
        assert_eq!(e.mem.u32(effect + 0x48), 0x10);
        assert_eq!(e.mem.f32(effect + 0x1c), 7.0);
    }
}
