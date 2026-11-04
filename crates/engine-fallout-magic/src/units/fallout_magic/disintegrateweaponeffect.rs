//! `fallout/magic/disintegrateweaponeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `0080d6b0` (thiscall on a new 0x48-byte block: caster, spell, effect item; unnamed):
/// the `DisintegrateWeaponEffect` constructor `Copy` uses.
const CONSTRUCTOR_0080D6B0: u32 = 0x0080_d6b0;
/// `ActiveEffect::CopyData` (`00804090`, thiscall: the target effect).
const ACTIVE_EFFECT_COPY_DATA: u32 = 0x0080_4090;
/// `MagicTarget::GetTargetAsActor` (`00822b40`).
const MAGIC_TARGET_GET_TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `0059bb30` (thiscall): the effect's target (+0x24).
const EFFECT_TARGET_0059BB30: u32 = 0x0059_bb30;
/// `008d8520` (thiscall on an actor; the map's name `MiddleHighProcess::GetSavedAcquireObject`
/// may come from folded code): the object whose slot +0x148 gives the weapon here.
const ACTOR_OBJECT_008D8520: u32 = 0x008d_8520;
/// `00495460` (thiscall on the effect, returns `ST0`; unnamed): its magnitude.
const EFFECT_MAGNITUDE_00495460: u32 = 0x0049_5460;
/// `005b6f70` (no arguments, `AL`; unnamed, `tesscriptfunctions.cpp`).
const TEST_005B6F70: u32 = 0x005b_6f70;
/// `ItemChange::GetFullName` (`004be2d0`).
const ITEM_CHANGE_GET_FULL_NAME: u32 = 0x004b_e2d0;
/// `0055d520` (thiscall on the actor, no stack words; unnamed, `tesobjectrefr.cpp`): its
/// name.
const ACTOR_NAME_0055D520: u32 = 0x0055_d520;
/// `00703c00` (cdecl, variable arguments, a format first; unnamed, `interface.cpp`).
const MESSAGE_00703C00: u32 = 0x0070_3c00;
/// `"%s's %s takes %0.2f disintegrate weapon damage!"` (`0107a058`).
const WEAPON_DAMAGE_FORMAT: u32 = 0x0107_a058;

// Translated from 0080d710 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0080d710 - the exception-unwinding frame (FS:[0]) is not translated.
/// `DisintegrateWeaponEffect::Copy` (Xbox PDB): a new 0x48-byte effect (`operator new`,
/// then `0080d6b0` with the caster +0x28, spell +8 and effect item +0xc; null when the
/// allocation fails), given this effect's data through `ActiveEffect::CopyData` (called
/// directly); returns it.
pub fn disintegrate_weapon_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(OPERATOR_NEW, &args![0x48u32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let base = this.addr();
        let item = e.mem.u32(base + 0xc);
        let spell = e.mem.u32(base + 8);
        let caster = e.mem.u32(base + 0x28);
        e.call(CONSTRUCTOR_0080D6B0, &args![block, caster, spell, item])
            .u32()
    };
    e.call(ACTIVE_EFFECT_COPY_DATA, &args![this, copy]);
    copy
}

// Translated from 0080d7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DisintegrateWeaponEffect::Start` (Xbox PDB): for a target actor, the weapon its
/// `008d8520` object holds (slot +0x148) is damaged by the magnitude (actor slot +0x3c8
/// with the weapon, the magnitude as a `float` and 0) when there is one; when `005b6f70`
/// is true the damage is reported (`00703c00` with the format, the actor's name, the
/// weapon's name and the magnitude as a `double`), with or without a weapon.
pub fn disintegrate_weapon_effect_start(e: &mut Engine, this: Ptr) {
    let actor = if e.call(EFFECT_TARGET_0059BB30, &args![this]).u32() == 0 {
        0
    } else {
        let target = e.call(EFFECT_TARGET_0059BB30, &args![this]).u32();
        e.call(MAGIC_TARGET_GET_TARGET_AS_ACTOR, &args![target])
            .u32()
    };
    if actor == 0 {
        return;
    }
    let object = e.call(ACTOR_OBJECT_008D8520, &args![actor]).u32();
    let weapon = e.vcall(object, 0x148, &[]).u32();
    if weapon != 0 {
        let magnitude = x87_store32(e.call(EFFECT_MAGNITUDE_00495460, &args![this]).f64());
        e.vcall(actor, 0x3c8, &args![weapon, magnitude, 0u32]);
    }
    if e.call(TEST_005B6F70, &[]).bool() {
        let magnitude = e.call(EFFECT_MAGNITUDE_00495460, &args![this]).f64();
        let item_name = e.call(ITEM_CHANGE_GET_FULL_NAME, &args![weapon]).u32();
        let actor_name = e.call(ACTOR_NAME_0055D520, &args![actor]).u32();
        e.call(
            MESSAGE_00703C00,
            &args![WEAPON_DAMAGE_FORMAT, actor_name, item_name, magnitude],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0080_d710, disintegrate_weapon_effect_copy(Ptr) -> u32),
        entry!(0x0080_d7b0, disintegrate_weapon_effect_start(Ptr)),
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

    /// An object whose vtable slot `offset` answers `value` (its double sits at the slot).
    fn object(e: &mut Engine, offset: u32, value: u32) -> u32 {
        let vtable = e.mem.alloc(0x400);
        e.mem.set_u32(vtable + offset, vtable + offset);
        answer(e, vtable + offset, value);
        let this = e.mem.alloc(0x10);
        e.mem.set_u32(this, vtable);
        this
    }

    #[test]
    fn copy_constructs_and_copies_through_active_effect() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x48);
        e.mem.set_u32(this + 8, 0x08);
        e.mem.set_u32(this + 0xc, 0x0c);
        e.mem.set_u32(this + 0x28, 0x28);
        answer(&mut e, OPERATOR_NEW, 0x9000);
        answer(&mut e, CONSTRUCTOR_0080D6B0, 0x9004);
        answer(&mut e, ACTIVE_EFFECT_COPY_DATA, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0080_d710, &args![this]).u32(), 0x9004);
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x48]]);
        assert_eq!(
            calls(&e, CONSTRUCTOR_0080D6B0),
            vec![vec![0x9000, 0x28, 0x08, 0x0c]]
        );
        assert_eq!(calls(&e, ACTIVE_EFFECT_COPY_DATA), vec![vec![this, 0x9004]]);
    }

    #[test]
    fn start_damages_the_held_weapon_and_reports_it() {
        for (weapon, damages) in [(0x4800u32, 1usize), (0, 0)] {
            let mut e = Engine::new();
            let actor = object(&mut e, 0x3c8, 0);
            let holder = object(&mut e, 0x148, weapon);
            answer(&mut e, EFFECT_TARGET_0059BB30, 0x7000);
            answer(&mut e, MAGIC_TARGET_GET_TARGET_AS_ACTOR, actor);
            answer(&mut e, ACTOR_OBJECT_008D8520, holder);
            e.register(EFFECT_MAGNITUDE_00495460, |_, _| Ret {
                st0: 2.5,
                ..Ret::default()
            });
            answer(&mut e, TEST_005B6F70, 1);
            answer(&mut e, ITEM_CHANGE_GET_FULL_NAME, 0x6100);
            answer(&mut e, ACTOR_NAME_0055D520, 0x6200);
            answer(&mut e, MESSAGE_00703C00, 0);
            e.call_log = Some(vec![]);
            e.call(0x0080_d7b0, &args![0x1000u32]);
            let damage = calls(&e, e.mem.u32(actor) + 0x3c8);
            assert_eq!(damage.len(), damages);
            if damages == 1 {
                assert_eq!(damage[0], vec![actor, weapon, 2.5f32.to_bits(), 0]);
            }
            assert_eq!(calls(&e, ITEM_CHANGE_GET_FULL_NAME), vec![vec![weapon]]);
            let bits = 2.5f64.to_bits();
            assert_eq!(
                calls(&e, MESSAGE_00703C00),
                vec![vec![
                    WEAPON_DAMAGE_FORMAT,
                    0x6200,
                    0x6100,
                    bits as u32,
                    (bits >> 32) as u32
                ]]
            );
        }
    }
}
