//! `fallout/magic/valueandconditionseffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_magic::activeeffect::ActiveEffect;

layout! {
    /// `ValueAndConditionsEffect` (Xbox PDB), 0x50 bytes: a
    /// `ValueModifierEffect` and the body part it also modifies.
    pub struct ValueAndConditionsEffect: 0x50 {
        /// `eTargetBodyPart` (Xbox PDB): an actor value, -1 for none.
        0x4C eTargetBodyPart: u32,
    }
}

/// `ValueAndConditionsEffect`'s vtable (RTTI
/// `.?AVValueAndConditionsEffect@@`).
const VALUE_AND_CONDITIONS_EFFECT_VTABLE: u32 = 0x0107_c5dc;
/// The `ValueModifierEffect` constructor (`0082ba60`), thiscall (caster,
/// spell, effect item), `RET 0xc`.
const VALUE_MODIFIER_EFFECT_CONSTRUCT: u32 = 0x0082_ba60;
/// `ValueModifierEffect::CopyData` (engine map), thiscall (other), `RET 4`.
const VALUE_MODIFIER_EFFECT_COPY_DATA: u32 = 0x0082_bbe0;
/// `ValueModifierEffect::ModifyActorValue` (engine map), thiscall (actor,
/// value, actor value), `RET 0xc`.
const VALUE_MODIFIER_EFFECT_MODIFY_ACTOR_VALUE: u32 = 0x0082_ca50;
/// `ActiveEffect::GetSaveSize` (engine map), thiscall (word), `RET 4`: a
/// 16-bit size.
const ACTIVE_EFFECT_GET_SAVE_SIZE: u32 = 0x0080_4f80;
/// `ActiveEffect::SaveGame_ov2` (`00806230`) and `ActiveEffect::LoadGame_ov2`
/// (`00806420`) (engine map), thiscall (buffer), `RET 4`.
const ACTIVE_EFFECT_SAVE_GAME_BUFFER: u32 = 0x0080_6230;
const ACTIVE_EFFECT_LOAD_GAME_BUFFER: u32 = 0x0080_6420;
/// `ValueModifierEffect::SaveGame` (`00802db0`) and
/// `ValueModifierEffect::LoadGame` (`00802dd0`) (engine map), thiscall
/// (word), `RET 4`.
const VALUE_MODIFIER_EFFECT_SAVE_GAME: u32 = 0x0080_2db0;
const VALUE_MODIFIER_EFFECT_LOAD_GAME: u32 = 0x0080_2dd0;
/// The save/load singleton (pointer global) and its writes (`008579b0`) and
/// reads (`008579e0`) of a block (address, size), thiscall, `RET 8`
/// (`tessaveloadgame.cpp`).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
const SAVE_LOAD_WRITE_008579B0: u32 = 0x0085_79b0;
const SAVE_LOAD_READ_008579E0: u32 = 0x0085_79e0;
/// `BGSSaveGameBuffer` (`00865e50`, address, size, word, `RET 0xc`) and
/// `BGSLoadGameBuffer` (`00864980`, address, size, `RET 8`) block
/// transfers, thiscall on the buffer.
const SAVE_BUFFER_WRITE_00865E50: u32 = 0x0086_5e50;
const LOAD_BUFFER_READ_00864980: u32 = 0x0086_4980;
/// The load buffer's virtual slot +0: a byte, compared with 8.
const SLOT_BUFFER_BYTE: u32 = 0x0;
/// `operator new` (`00401000`, cdecl: size).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `ActiveEffect`'s virtual slot +0x44 (`CopyData`, Xbox PDB order).
const SLOT_COPY_DATA: u32 = 0x44;
/// `__RTDynamicCast` (CRT), cdecl, and the type descriptors of
/// `ActiveEffect` and `ValueAndConditionsEffect`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_ACTIVE_EFFECT: u32 = 0x011a_0d1c;
const TYPE_VALUE_AND_CONDITIONS_EFFECT: u32 = 0x011a_13d0;
/// Setting addresses (`00403e20` gives the address of the value): the
/// multipliers of the body part (`011d1860`) and of the seven limb values
/// (`011d17ac`).
const SETTING_VALUE: u32 = 0x0040_3e20;
const BODY_PART_MULT_011D1860: u32 = 0x011d_1860;
const LIMB_MULT_011D17AC: u32 = 0x011d_17ac;

// Translated from 00806da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueAndConditionsEffect::GetSaveSize` (engine map; `thiscall`, `RET
/// 4`): `ActiveEffect::GetSaveSize` plus 4, as 16 bits.
pub fn value_and_conditions_effect_get_save_size(
    e: &mut Engine,
    this: Ptr<ValueAndConditionsEffect>,
    word: u32,
) -> u16 {
    let size = e
        .call(ACTIVE_EFFECT_GET_SAVE_SIZE, &args![this, word])
        .u16();
    size.wrapping_add(4)
}

// Translated from 0080f2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueAndConditionsEffect::LoadGame` (engine map; `thiscall`, `RET 4`):
/// `ValueModifierEffect::LoadGame`, then the body part's 4 bytes read
/// through the save/load singleton (`008579e0`).
pub fn value_and_conditions_effect_load_game(
    e: &mut Engine,
    this: Ptr<ValueAndConditionsEffect>,
    word: u32,
) {
    e.call(VALUE_MODIFIER_EFFECT_LOAD_GAME, &args![this, word]);
    let game = e.mem.u32(SAVE_LOAD_GAME);
    e.call(
        SAVE_LOAD_READ_008579E0,
        &args![game, this.addr() + 0x4c, 4u32],
    );
}

// Translated from 0080f320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueAndConditionsEffect::SaveGame` (engine map; `thiscall`, `RET 4`):
/// `ActiveEffect::SaveGame_ov2(buffer)`, then the body part's 4 bytes
/// written to the buffer (`00865e50` with 0).
pub fn value_and_conditions_effect_save_game(
    e: &mut Engine,
    this: Ptr<ValueAndConditionsEffect>,
    buffer: u32,
) {
    e.call(ACTIVE_EFFECT_SAVE_GAME_BUFFER, &args![this, buffer]);
    e.call(
        SAVE_BUFFER_WRITE_00865E50,
        &args![buffer, this.addr() + 0x4c, 4u32, 0u32],
    );
}

// Translated from 0080f350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueAndConditionsEffect::LoadGame_ov2` (engine map; `thiscall`, `RET
/// 4`): `ActiveEffect::LoadGame_ov2(buffer)`, then, when the buffer's
/// virtual slot +0 answers at least 8, the body part's 4 bytes
/// read from it (`00864980`).
pub fn value_and_conditions_effect_load_game_ov2(
    e: &mut Engine,
    this: Ptr<ValueAndConditionsEffect>,
    buffer: u32,
) {
    e.call(ACTIVE_EFFECT_LOAD_GAME_BUFFER, &args![this, buffer]);
    let byte = e.vcall(buffer, SLOT_BUFFER_BYTE, &args![]).u8();
    if byte >= 8 {
        e.call(
            LOAD_BUFFER_READ_00864980,
            &args![buffer, this.addr() + 0x4c, 4u32],
        );
    }
}

// Translated from 0082b7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `ValueAndConditionsEffect` constructor (unnamed in the engine map;
/// `thiscall`, `RET 0xc`): the `ValueModifierEffect` constructor with the
/// caster, the spell and the effect item, the class's vtable (`0107c5dc`),
/// no body part (-1). Returns `this`.
pub fn fn_0082b7b0(
    e: &mut Engine,
    this: Ptr<ValueAndConditionsEffect>,
    caster: u32,
    spell: u32,
    effect: u32,
) -> Ptr<ValueAndConditionsEffect> {
    e.call(
        VALUE_MODIFIER_EFFECT_CONSTRUCT,
        &args![this, caster, spell, effect],
    );
    e.mem
        .set_u32(this.addr(), VALUE_AND_CONDITIONS_EFFECT_VTABLE);
    e.set(this, ValueAndConditionsEffect::eTargetBodyPart, 0xffff_ffff);
    this
}

// Translated from 0082b7f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082b7f0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the new effect) is not translated.
/// `ValueAndConditionsEffect::Copy` (engine map; `thiscall`, plain `RET`):
/// a new effect (0x50 bytes, `0082b7b0` with this effect's caster, spell
/// and effect item; 0 when the allocation failed) that this effect's
/// `CopyData` (virtual slot +0x44) fills. Returns it.
pub fn value_and_conditions_effect_copy(e: &mut Engine, this: Ptr<ActiveEffect>) -> u32 {
    let memory = e.call(OPERATOR_NEW, &args![0x50u32]).u32();
    let copy = if memory == 0 {
        0
    } else {
        let caster = e.get(this, ActiveEffect::pCaster);
        let spell = e.get(this, ActiveEffect::pSpell);
        let effect = e.get(this, ActiveEffect::pEffect);
        fn_0082b7b0(e, Ptr::new(memory), caster, spell, effect).addr()
    };
    e.vcall(this.addr(), SLOT_COPY_DATA, &args![copy]);
    copy
}

// Translated from 0082b8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueAndConditionsEffect::CopyData` (engine map; `thiscall`, `RET 4`):
/// `ValueModifierEffect::CopyData(other)`, then, when `other` is a
/// `ValueAndConditionsEffect` (`__RTDynamicCast` from `ActiveEffect`), this
/// effect's body part copied into it.
pub fn value_and_conditions_effect_copy_data(
    e: &mut Engine,
    this: Ptr<ValueAndConditionsEffect>,
    other: u32,
) {
    e.call(VALUE_MODIFIER_EFFECT_COPY_DATA, &args![this, other]);
    let target = e
        .call(
            RT_DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_ACTIVE_EFFECT,
                TYPE_VALUE_AND_CONDITIONS_EFFECT,
                0u32
            ],
        )
        .u32();
    if target != 0 {
        let body_part = e.get(this, ValueAndConditionsEffect::eTargetBodyPart);
        e.set(
            Ptr::<ValueAndConditionsEffect>::new(target),
            ValueAndConditionsEffect::eTargetBodyPart,
            body_part,
        );
    }
}

// Translated from 0082b8f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082b8f0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the new effect) is not translated.
/// Unnamed in the engine map (cdecl, three words): a new
/// `ValueAndConditionsEffect` (0x50 bytes, `0082b7b0` with the caster, the
/// spell and the effect item), or 0 when the allocation failed.
pub fn fn_0082b8f0(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let memory = e.call(OPERATOR_NEW, &args![0x50u32]).u32();
    if memory == 0 {
        return 0;
    }
    fn_0082b7b0(e, Ptr::new(memory), caster, spell, effect).addr()
}

/// `value * <setting>` stored as a `float` (the setting's address from
/// `00403e20`).
fn scaled_by_setting(e: &mut Engine, value: f32, setting: u32) -> u32 {
    let mult = e.call(SETTING_VALUE, &args![setting]).u32();
    x87_store32(x87_mul(
        x87_load32(value.to_bits()),
        x87_m32(e.mem.u32(mult)),
    ))
}

// Translated from 0082b970 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueAndConditionsEffect::ModifyActorValue` (engine map; `thiscall`,
/// `RET 0xc`; the third word is not read): with a body part, the body part
/// gets `value` and the effect's own actor value (-1) gets `value * setting
/// 011d1860` (`ValueModifierEffect::ModifyActorValue` each); without one,
/// the effect's actor value gets `value` and the actor values 0x19 to 0x1f
/// each get `value * setting 011d17ac`.
pub fn value_and_conditions_effect_modify_actor_value(
    e: &mut Engine,
    this: Ptr<ValueAndConditionsEffect>,
    actor: u32,
    value: f32,
    _unused_2: u32,
) {
    let value_bits = x87_quiet_bits(value.to_bits());
    let body_part = e.get(this, ValueAndConditionsEffect::eTargetBodyPart);
    if body_part != 0xffff_ffff {
        let scaled = scaled_by_setting(e, value, BODY_PART_MULT_011D1860);
        let body_part = e.get(this, ValueAndConditionsEffect::eTargetBodyPart);
        e.call(
            VALUE_MODIFIER_EFFECT_MODIFY_ACTOR_VALUE,
            &args![this, actor, value_bits, body_part],
        );
        e.call(
            VALUE_MODIFIER_EFFECT_MODIFY_ACTOR_VALUE,
            &args![this, actor, scaled, 0xffff_ffffu32],
        );
    } else {
        e.call(
            VALUE_MODIFIER_EFFECT_MODIFY_ACTOR_VALUE,
            &args![this, actor, value_bits, 0xffff_ffffu32],
        );
        let scaled = scaled_by_setting(e, value, LIMB_MULT_011D17AC);
        for actor_value in 0x19u32..0x20 {
            e.call(
                VALUE_MODIFIER_EFFECT_MODIFY_ACTOR_VALUE,
                &args![this, actor, scaled, actor_value],
            );
        }
    }
}

// Translated from 0082ba30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ValueAndConditionsEffect::SaveGame_ov2` (engine map; `thiscall`, `RET
/// 4`): `ValueModifierEffect::SaveGame`, then the body part's 4 bytes
/// written through the save/load singleton (`008579b0`).
pub fn value_and_conditions_effect_save_game_ov2(
    e: &mut Engine,
    this: Ptr<ValueAndConditionsEffect>,
    word: u32,
) {
    e.call(VALUE_MODIFIER_EFFECT_SAVE_GAME, &args![this, word]);
    let game = e.mem.u32(SAVE_LOAD_GAME);
    e.call(
        SAVE_LOAD_WRITE_008579B0,
        &args![game, this.addr() + 0x4c, 4u32],
    );
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0080_6da0,
            value_and_conditions_effect_get_save_size(Ptr<ValueAndConditionsEffect>, u32) -> u16
        ),
        entry!(
            0x0080_f2f0,
            value_and_conditions_effect_load_game(Ptr<ValueAndConditionsEffect>, u32)
        ),
        entry!(
            0x0080_f320,
            value_and_conditions_effect_save_game(Ptr<ValueAndConditionsEffect>, u32)
        ),
        entry!(
            0x0080_f350,
            value_and_conditions_effect_load_game_ov2(Ptr<ValueAndConditionsEffect>, u32)
        ),
        entry!(
            0x0082_b7b0,
            fn_0082b7b0(
                Ptr<ValueAndConditionsEffect>,
                u32,
                u32,
                u32,
            ) -> Ptr<ValueAndConditionsEffect>
        ),
        entry!(
            0x0082_b7f0,
            value_and_conditions_effect_copy(Ptr<ActiveEffect>) -> u32
        ),
        entry!(
            0x0082_b8a0,
            value_and_conditions_effect_copy_data(Ptr<ValueAndConditionsEffect>, u32)
        ),
        entry!(0x0082_b8f0, fn_0082b8f0(u32, u32, u32) -> u32),
        entry!(
            0x0082_b970,
            value_and_conditions_effect_modify_actor_value(
                Ptr<ValueAndConditionsEffect>,
                u32,
                f32,
                u32,
            )
        ),
        entry!(
            0x0082_ba30,
            value_and_conditions_effect_save_game_ov2(Ptr<ValueAndConditionsEffect>, u32)
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

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn save_size_adds_four_in_sixteen_bits() {
        let mut e = Engine::new();
        e.register(ACTIVE_EFFECT_GET_SAVE_SIZE, |_, a| {
            (0x1_fffe + a[1]).into_ret()
        });
        assert_eq!(e.call(0x0080_6da0, &args![0x100u32, 0u32]).u32(), 2);
        let effect: Ptr<ValueAndConditionsEffect> = Ptr::new(0x100);
        assert_eq!(
            value_and_conditions_effect_get_save_size(&mut e, effect, 0x10),
            0x12
        );
    }

    #[test]
    fn load_game_reads_the_body_part_through_the_singleton() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.set_global(SAVE_LOAD_GAME, 0x4444u32);
        quiet(
            &mut e,
            &[VALUE_MODIFIER_EFFECT_LOAD_GAME, SAVE_LOAD_READ_008579E0],
        );
        e.call_log = Some(vec![]);
        e.call(0x0080_f2f0, &args![0x100u32, 7u32]);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_LOAD_GAME),
            vec![vec![0x100, 7]]
        );
        assert_eq!(
            calls(&e, SAVE_LOAD_READ_008579E0),
            vec![vec![0x4444, 0x14c, 4]]
        );
    }

    #[test]
    fn save_game_writes_the_body_part_to_the_buffer() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[ACTIVE_EFFECT_SAVE_GAME_BUFFER, SAVE_BUFFER_WRITE_00865E50],
        );
        e.call_log = Some(vec![]);
        e.call(0x0080_f320, &args![0x100u32, 0x900u32]);
        assert_eq!(
            calls(&e, ACTIVE_EFFECT_SAVE_GAME_BUFFER),
            vec![vec![0x100, 0x900]]
        );
        assert_eq!(
            calls(&e, SAVE_BUFFER_WRITE_00865E50),
            vec![vec![0x900, 0x14c, 4, 0]]
        );
    }

    #[test]
    fn buffer_load_reads_the_body_part_from_eight_on() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[ACTIVE_EFFECT_LOAD_GAME_BUFFER, LOAD_BUFFER_READ_00864980],
        );
        let buffer = e.mem.alloc(8);
        let vtable = e.mem.alloc(4);
        e.mem.set_u32(vtable, 0x00f3_0000);
        e.mem.set_u32(buffer, vtable);
        e.mem.set_u32(buffer + 4, 7);
        e.register(0x00f3_0000, |e, a| {
            (0xff00 | e.mem.u32(a[0] + 4)).into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0080_f350, &args![0x100u32, buffer]);
        assert!(calls(&e, LOAD_BUFFER_READ_00864980).is_empty());
        e.mem.set_u32(buffer + 4, 8);
        e.call(0x0080_f350, &args![0x100u32, buffer]);
        assert_eq!(
            calls(&e, LOAD_BUFFER_READ_00864980),
            vec![vec![buffer, 0x14c, 4]]
        );
        assert_eq!(calls(&e, ACTIVE_EFFECT_LOAD_GAME_BUFFER).len(), 2);
    }

    #[test]
    fn constructor_sets_vtable_and_no_body_part() {
        let mut e = Engine::new();
        quiet(&mut e, &[VALUE_MODIFIER_EFFECT_CONSTRUCT]);
        let this = e.mem.alloc(0x50);
        e.call_log = Some(vec![]);
        let result = e.call(0x0082_b7b0, &args![this, 1u32, 2u32, 3u32]).u32();
        assert_eq!(result, this);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_CONSTRUCT),
            vec![vec![this, 1, 2, 3]]
        );
        assert_eq!(e.mem.u32(this), VALUE_AND_CONDITIONS_EFFECT_VTABLE);
        assert_eq!(e.mem.u32(this + 0x4c), 0xffff_ffff);
    }

    #[test]
    fn copy_makes_a_new_effect_and_fills_it() {
        let mut e = Engine::new();
        quiet(&mut e, &[VALUE_MODIFIER_EFFECT_CONSTRUCT]);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = e.mem.alloc(0x50);
        let vtable = e.mem.alloc(0x48);
        e.mem.set_u32(vtable + 0x44, 0x00f4_0000);
        e.register(0x00f4_0000, |_, _| Ret::default());
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x28, 0xa);
        e.mem.set_u32(this + 8, 0xb);
        e.mem.set_u32(this + 0xc, 0xc);
        e.call_log = Some(vec![]);
        let copy = e.call(0x0082_b7f0, &args![this]).u32();
        assert_ne!(copy, 0);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_CONSTRUCT),
            vec![vec![copy, 0xa, 0xb, 0xc]]
        );
        assert_eq!(calls(&e, 0x00f4_0000), vec![vec![this, copy]]);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_b7f0, &args![this]).u32(), 0);
        assert_eq!(calls(&e, 0x00f4_0000), vec![vec![this, 0]]);
    }

    #[test]
    fn copy_data_passes_the_body_part_on() {
        let mut e = Engine::new();
        quiet(&mut e, &[VALUE_MODIFIER_EFFECT_COPY_DATA]);
        e.register(RT_DYNAMIC_CAST, |_, a| {
            if a[0] == 0x9000 { 0 } else { a[0] }.into_ret()
        });
        let this = e.mem.alloc(0x50);
        let other = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x4c, 0x1b);
        e.call_log = Some(vec![]);
        e.call(0x0082_b8a0, &args![this, other]);
        assert_eq!(e.mem.u32(other + 0x4c), 0x1b);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_COPY_DATA),
            vec![vec![this, other]]
        );
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST)[0],
            vec![
                other,
                0,
                TYPE_ACTIVE_EFFECT,
                TYPE_VALUE_AND_CONDITIONS_EFFECT,
                0
            ]
        );
        e.call(0x0082_b8a0, &args![this, 0x9000u32]);
    }

    #[test]
    fn create_or_zero() {
        let mut e = Engine::new();
        quiet(&mut e, &[VALUE_MODIFIER_EFFECT_CONSTRUCT]);
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let made = e.call(0x0082_b8f0, &args![1u32, 2u32, 3u32]).u32();
        assert_eq!(e.mem.u32(made), VALUE_AND_CONDITIONS_EFFECT_VTABLE);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(fn_0082b8f0(&mut e, 1, 2, 3), 0);
    }

    #[test]
    fn modify_spreads_over_body_part_or_limbs() {
        let mut e = Engine::new();
        e.map(0x011d_1000, 0x1000);
        e.set_global(BODY_PART_MULT_011D1860 + 4, 0.5f32);
        e.set_global(LIMB_MULT_011D17AC + 4, 0.25f32);
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        quiet(&mut e, &[VALUE_MODIFIER_EFFECT_MODIFY_ACTOR_VALUE]);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this + 0x4c, 0x1a);
        e.call_log = Some(vec![]);
        e.call(0x0082_b970, &args![this, 0x700u32, 8.0f32, 99u32]);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_MODIFY_ACTOR_VALUE),
            vec![
                vec![this, 0x700, 8.0f32.to_bits(), 0x1a],
                vec![this, 0x700, 4.0f32.to_bits(), 0xffff_ffff],
            ]
        );
        e.mem.set_u32(this + 0x4c, 0xffff_ffff);
        e.call_log = Some(vec![]);
        let effect: Ptr<ValueAndConditionsEffect> = Ptr::new(this);
        value_and_conditions_effect_modify_actor_value(&mut e, effect, 0x700, 8.0, 0);
        let mut expected = vec![vec![this, 0x700, 8.0f32.to_bits(), 0xffff_ffff]];
        for actor_value in 0x19..0x20 {
            expected.push(vec![this, 0x700, 2.0f32.to_bits(), actor_value]);
        }
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_MODIFY_ACTOR_VALUE),
            expected
        );
    }

    #[test]
    fn save_game_ov2_writes_the_body_part_through_the_singleton() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.set_global(SAVE_LOAD_GAME, 0x4444u32);
        quiet(
            &mut e,
            &[VALUE_MODIFIER_EFFECT_SAVE_GAME, SAVE_LOAD_WRITE_008579B0],
        );
        e.call_log = Some(vec![]);
        e.call(0x0082_ba30, &args![0x100u32, 7u32]);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_SAVE_GAME),
            vec![vec![0x100, 7]]
        );
        assert_eq!(
            calls(&e, SAVE_LOAD_WRITE_008579B0),
            vec![vec![0x4444, 0x14c, 4]]
        );
    }
}
