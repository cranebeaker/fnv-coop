//! `fallout/magic/shieldeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `ShieldEffect` (Xbox PDB, 0x50 bytes): a `ValueModifierEffect` (the `ActiveEffect`
//! base, 0x48 bytes, and `eWhichActorValue` at +0x48) plus `eAdditionalActorValue` at
//! +0x4C. The `ActiveEffect` fields used (Xbox PDB, same PC offsets): `pSpell` +8,
//! `pEffect` +0xC, `fMagnitude` +0x1C, `fDuration` +0x20, `pCaster` +0x28, `eSpellType`
//! +0x2C. The compiler's exception frames (`FS:[0]`) are not translated; the functions that
//! have them carry a `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

/// `ShieldEffect`'s vtable (`0107c1fc`, RTTI `.?AVShieldEffect@@`).
const SHIELD_EFFECT_VTABLE: u32 = 0x0107_c1fc;
/// `eAdditionalActorValue` (Xbox PDB) +0x4C.
const ADDITIONAL_ACTOR_VALUE: u32 = 0x4c;
/// `ActiveEffect` fields (Xbox PDB).
const SPELL: u32 = 0x8;
const EFFECT: u32 = 0xc;
const MAGNITUDE: u32 = 0x1c;
const DURATION: u32 = 0x20;
const CASTER: u32 = 0x28;
const SPELL_TYPE: u32 = 0x2c;

/// The allocator `00401000` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// The `ValueModifierEffect` constructor `0082ba60` (unnamed in the engine map;
/// `thiscall`, three words).
const VALUE_MODIFIER_EFFECT_CONSTRUCTOR: u32 = 0x0082_ba60;
/// `00403ea0` (`effectitem.cpp`; `thiscall` on an `EffectItem`): an actor value index (-1
/// when `00403ee0` says it has none).
const EFFECT_ITEM_ACTOR_VALUE: u32 = 0x0040_3ea0;
/// `00673400` (`thiscall`, one word): stores the word at +0x48 (the engine map's name,
/// `LowProcess::SetSecondGenericLocation`, is folded code).
const SET_WORD_48: u32 = 0x0067_3400;
/// `ValueModifierEffect::CopyData` (engine map, `0082bbe0`; `thiscall`, one word).
const VALUE_MODIFIER_EFFECT_COPY_DATA: u32 = 0x0082_bbe0;
/// `ValueModifierEffect::Start`, `::Update` (one float), `::Finish` (engine map,
/// `0082c000`, `0082c660`, `0082c810`; `thiscall`).
const VALUE_MODIFIER_EFFECT_START: u32 = 0x0082_c000;
const VALUE_MODIFIER_EFFECT_UPDATE: u32 = 0x0082_c660;
const VALUE_MODIFIER_EFFECT_FINISH: u32 = 0x0082_c810;
/// `ValueModifierEffect::GetSaveSize`, `::SaveGame`, `::LoadGame` (engine map,
/// `00802d90`, `00802db0`, `00802dd0`; `thiscall`, one word each).
const VALUE_MODIFIER_EFFECT_GET_SAVE_SIZE: u32 = 0x0080_2d90;
const VALUE_MODIFIER_EFFECT_SAVE_GAME: u32 = 0x0080_2db0;
const VALUE_MODIFIER_EFFECT_LOAD_GAME: u32 = 0x0080_2dd0;
/// `__RTDynamicCast` (engine map, `00ec43fb`; `cdecl`, five words) and the type descriptors
/// of `ActiveEffect` and `ShieldEffect`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
const ACTIVE_EFFECT_TYPE: u32 = 0x011a_0d1c;
const SHIELD_EFFECT_TYPE: u32 = 0x011a_14b0;
/// `00825c00` (`thiscall` on an `EffectItem`): the word at +0x14 (its `EffectSetting`).
const EFFECT_ITEM_SETTING: u32 = 0x0082_5c00;
/// `00403e60` (`thiscall` on an `EffectSetting`, one word: a flag): whether the flag is
/// set (`0058cba0`).
const EFFECT_SETTING_HAS_FLAG: u32 = 0x0040_3e60;
/// `0059bb30` (`thiscall`): the effect's `pTarget` (+0x24).
const ACTIVE_EFFECT_TARGET: u32 = 0x0059_bb30;
/// `MagicTarget::GetTargetAsActor` (engine map, `00822b40`; `thiscall`).
const TARGET_AS_ACTOR: u32 = 0x0082_2b40;
/// `0068a810` (`thiscall`): the word at +0x4C (`eAdditionalActorValue`).
const ADDITIONAL_ACTOR_VALUE_GETTER: u32 = 0x0068_a810;
/// The `double` the duration is compared with (`01012060`).
const COMPARED_DOUBLE: u32 = 0x0101_2060;
/// The actor value index that `008282e0` leaves alone (0x4D; its meaning is not confirmed
/// here).
const SKIPPED_ACTOR_VALUE: u32 = 0x4d;
/// The spell type `008282e0` and `008281b0` treat apart (4; its meaning is not confirmed
/// here).
const SPELL_TYPE_4: u32 = 4;
/// The actor's vtable slots `008282e0` modifies the value through: +0x3B4 (two words:
/// index, amount) for spell type 4; else +0x39C or +0x3AC (three words: index, amount,
/// 0) by the effect setting's flag 2.
const ACTOR_SLOT_3B4: u32 = 0x3b4;
const ACTOR_SLOT_39C: u32 = 0x39c;
const ACTOR_SLOT_3AC: u32 = 0x3ac;
/// The `ActiveEffect` vtable slot `Copy` hands the copy to (`CopyData`, +0x44).
const SLOT_COPY_DATA: u32 = 0x44;
/// The save game object (`011de45c`), its version byte (`008df040`, +0x80) and its raw
/// write (`008579b0`) and read (`008579e0`) (`thiscall`, two words: address, size).
const SAVE_GAME: u32 = 0x011d_e45c;
const SAVE_VERSION: u32 = 0x008d_f040;
const SAVE_WRITE: u32 = 0x0085_79b0;
const SAVE_READ: u32 = 0x0085_79e0;
/// The save version from which `eAdditionalActorValue` is saved.
const VERSION_ADDITIONAL_VALUE: i32 = 0x6e;

/// The save game object's version byte, as the exe compares it (`MOVZX`, signed `CMP`).
fn save_version(e: &mut Engine) -> i32 {
    let save_game: u32 = e.global(SAVE_GAME);
    i32::from(e.call(SAVE_VERSION, &args![save_game]).u8())
}

/// Whether the effect's `EffectSetting` has flag 2 (`00825c00`, `00403e60`).
fn setting_flag_2(e: &mut Engine, this: Ptr) -> bool {
    let item = e.mem.u32(this.addr() + EFFECT);
    let setting = e.call(EFFECT_ITEM_SETTING, &args![item]).u32();
    e.call(EFFECT_SETTING_HAS_FLAG, &args![setting, 2u32])
        .bool()
}

/// The effect's target as an actor (`0059bb30`, then `00822b40` when there is a target),
/// or 0.
fn target_actor(e: &mut Engine, this: Ptr) -> u32 {
    if e.call(ACTIVE_EFFECT_TARGET, &args![this]).u32() == 0 {
        return 0;
    }
    let target = e.call(ACTIVE_EFFECT_TARGET, &args![this]).u32();
    e.call(TARGET_AS_ACTOR, &args![target]).u32()
}

// Translated from 00827f20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00827f20 - the C++ exception-unwinding frame (FS:[0]) that destroys the base when the actor value lookup throws is not translated
/// The `ShieldEffect` constructor (by the vtable it stores; unnamed in the engine map;
/// `thiscall`, `RET 0xc`): the `ValueModifierEffect` constructor `0082ba60` with the three
/// words, the vtable, then the word at +0x48 set (`00673400`) to the effect item's actor
/// value (`00403ea0`). Returns `this`.
pub fn fn_00827f20(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    e.call(
        VALUE_MODIFIER_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, effect],
    );
    e.mem.set_u32(this.addr(), SHIELD_EFFECT_VTABLE);
    let value = e.call(EFFECT_ITEM_ACTOR_VALUE, &args![effect]).u32();
    e.call(SET_WORD_48, &args![this, value]);
    this
}

// Translated from 00827fa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00827fa0 - the C++ exception-unwinding frame (FS:[0]) that frees the allocation when the constructor throws is not translated
/// `ShieldEffect::Copy` (Xbox PDB): allocates a `ShieldEffect` (0x50 bytes), constructs
/// it with this effect's `pCaster`, `pSpell` and `pEffect` (0 when the allocation failed),
/// lets this effect copy its data into it (vtable +0x44 with the copy) and returns the
/// copy.
pub fn shield_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let block = e.call(ALLOCATE, &args![0x50u32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let caster = e.mem.u32(base + CASTER);
        let spell = e.mem.u32(base + SPELL);
        let effect = e.mem.u32(base + EFFECT);
        fn_00827f20(e, Ptr::new(block), caster, spell, effect).addr()
    };
    e.vcall(base, SLOT_COPY_DATA, &args![copy]);
    copy
}

// Translated from 00828050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShieldEffect::CopyData` (Xbox PDB; `thiscall`, `RET 4`): the `ValueModifierEffect`
/// data (`0082bbe0`), then, when the target is a `ShieldEffect` (a dynamic cast from
/// `ActiveEffect`), this effect's `eAdditionalActorValue`.
pub fn shield_effect_copy_data(e: &mut Engine, this: Ptr, target: u32) {
    e.call(VALUE_MODIFIER_EFFECT_COPY_DATA, &args![this, target]);
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![target, 0u32, ACTIVE_EFFECT_TYPE, SHIELD_EFFECT_TYPE, 0u32],
        )
        .u32();
    if cast != 0 {
        let value = e.mem.u32(this.addr() + ADDITIONAL_ACTOR_VALUE);
        e.mem.set_u32(cast + ADDITIONAL_ACTOR_VALUE, value);
    }
}

// Translated from 008280a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008280a0 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
/// Unnamed in the engine map (`cdecl`, three words): allocates a 0x50-byte `ShieldEffect`
/// (`00401000`) and constructs it with the three words (`00827f20`); returns it, or null
/// when the allocation fails.
pub fn fn_008280a0(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x50u32]).u32();
    if block == 0 {
        return 0;
    }
    fn_00827f20(e, Ptr::new(block), caster, spell, effect).addr()
}

// Translated from 00828120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShieldEffect::Start` (Xbox PDB): the `ValueModifierEffect` start (`0082c000`); then,
/// when the effect setting has flag 2 or `fDuration` equals the `double` at `01012060`
/// (`FCOMP`, `TEST AH,0x44`, `JP`: equal and ordered), adds `fMagnitude` to the target
/// actor's additional value (`008282e0`).
pub fn shield_effect_start(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(VALUE_MODIFIER_EFFECT_START, &args![this]);
    if !setting_flag_2(e, this) {
        let duration = x87_load32(e.mem.u32(base + DURATION));
        let compared: f64 = e.global(COMPARED_DOUBLE);
        if duration != compared {
            return;
        }
    }
    let actor = target_actor(e, this);
    let magnitude = x87_quiet(e.mem.f32(base + MAGNITUDE));
    shield_effect_modify_additional_actor_value(e, this, actor, magnitude);
}

// Translated from 008281b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShieldEffect::Update` (Xbox PDB; `thiscall`, `RET 4`): the `ValueModifierEffect`
/// update with the seconds (`0082c660`); then, unless the effect setting has flag 2, when
/// `fDuration` is greater than the `double` at `01012060` (ordered) or the spell type is
/// 4, adds `fMagnitude * seconds` (x87, rounded to `float`) to the target actor's
/// additional value (`008282e0`).
pub fn shield_effect_update(e: &mut Engine, this: Ptr, seconds: f32) {
    let base = this.addr();
    e.call(
        VALUE_MODIFIER_EFFECT_UPDATE,
        &args![this, x87_quiet(seconds)],
    );
    if setting_flag_2(e, this) {
        return;
    }
    let actor = target_actor(e, this);
    let duration = x87_load32(e.mem.u32(base + DURATION));
    let compared: f64 = e.global(COMPARED_DOUBLE);
    // TEST AH,0x41; JZ: straight on when greater (ordered).
    if !(duration > compared) && e.mem.u32(base + SPELL_TYPE) != SPELL_TYPE_4 {
        return;
    }
    let amount = x87_mul(
        x87_load32(e.mem.u32(base + MAGNITUDE)),
        x87_m32(seconds.to_bits()),
    );
    let amount = f32::from_bits(x87_store32(amount));
    shield_effect_modify_additional_actor_value(e, this, actor, x87_quiet(amount));
}

// Translated from 00828260 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShieldEffect::Finish` (Xbox PDB): the `ValueModifierEffect` finish (`0082c810`); then,
/// when the effect setting has flag 2, takes `fMagnitude` back off the target actor's
/// additional value (`008282e0` with `-fMagnitude`, `FCHS`).
pub fn shield_effect_finish(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.call(VALUE_MODIFIER_EFFECT_FINISH, &args![this]);
    if !setting_flag_2(e, this) {
        return;
    }
    let actor = target_actor(e, this);
    let magnitude = x87_quiet(e.mem.f32(base + MAGNITUDE));
    shield_effect_modify_additional_actor_value(e, this, actor, -magnitude);
}

// Translated from 008282e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShieldEffect::ModifyAdditionalActorValue` (Xbox PDB; `thiscall`, `RET 8`: the actor
/// and the amount): nothing without an actor or when `eAdditionalActorValue` (`0068a810`)
/// is 0x4D. For spell type 4 the actor's slot +0x3B4 gets (index, amount); otherwise slot
/// +0x39C (when the effect setting has flag 2) or +0x3AC gets (index, amount, 0). The
/// amount is passed through `FLD`/`FSTP` (a signalling NaN goes quiet).
pub fn shield_effect_modify_additional_actor_value(
    e: &mut Engine,
    this: Ptr,
    actor: u32,
    amount: f32,
) {
    let base = this.addr();
    if actor == 0
        || e.call(ADDITIONAL_ACTOR_VALUE_GETTER, &args![this]).u32() == SKIPPED_ACTOR_VALUE
    {
        return;
    }
    let amount = x87_quiet(amount);
    if e.mem.u32(base + SPELL_TYPE) == SPELL_TYPE_4 {
        let index = e.call(ADDITIONAL_ACTOR_VALUE_GETTER, &args![this]).u32();
        e.vcall(actor, ACTOR_SLOT_3B4, &args![index, amount]);
        return;
    }
    let slot = if setting_flag_2(e, this) {
        ACTOR_SLOT_39C
    } else {
        ACTOR_SLOT_3AC
    };
    let index = e.call(ADDITIONAL_ACTOR_VALUE_GETTER, &args![this]).u32();
    e.vcall(actor, slot, &args![index, amount, 0u32]);
}

// Translated from 008283a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShieldEffect::GetSaveSize` (Xbox PDB; `thiscall`, `RET 4`; a 16-bit sum): the
/// `ValueModifierEffect` size (`00802d90` with the same word), plus 4 from save version
/// 0x6E on.
pub fn shield_effect_get_save_size(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    let mut size = e
        .call(VALUE_MODIFIER_EFFECT_GET_SAVE_SIZE, &args![this, argument])
        .u16();
    if save_version(e) >= VERSION_ADDITIONAL_VALUE {
        size = size.wrapping_add(4);
    }
    size
}

// Translated from 008283f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShieldEffect::SaveGame` (Xbox PDB; `thiscall`, `RET 4`): the `ValueModifierEffect`
/// data (`00802db0`), then from save version 0x6E on the four bytes of
/// `eAdditionalActorValue` (`008579b0`).
pub fn shield_effect_save_game(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(VALUE_MODIFIER_EFFECT_SAVE_GAME, &args![this, argument]);
    if save_version(e) >= VERSION_ADDITIONAL_VALUE {
        let save_game: u32 = e.global(SAVE_GAME);
        e.call(
            SAVE_WRITE,
            &args![save_game, this.addr() + ADDITIONAL_ACTOR_VALUE, 4u32],
        );
    }
}

// Translated from 00828430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ShieldEffect::LoadGame` (Xbox PDB; `thiscall`, `RET 4`): the `ValueModifierEffect`
/// data (`00802dd0`), then from save version 0x6E on the four bytes of
/// `eAdditionalActorValue` (`008579e0`).
pub fn shield_effect_load_game(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(VALUE_MODIFIER_EFFECT_LOAD_GAME, &args![this, argument]);
    if save_version(e) >= VERSION_ADDITIONAL_VALUE {
        let save_game: u32 = e.global(SAVE_GAME);
        e.call(
            SAVE_READ,
            &args![save_game, this.addr() + ADDITIONAL_ACTOR_VALUE, 4u32],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082_7f20, fn_00827f20(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0082_7fa0, shield_effect_copy(Ptr) -> u32),
        entry!(0x0082_8050, shield_effect_copy_data(Ptr, u32)),
        entry!(0x0082_80a0, fn_008280a0(u32, u32, u32) -> u32),
        entry!(0x0082_8120, shield_effect_start(Ptr)),
        entry!(0x0082_81b0, shield_effect_update(Ptr, f32)),
        entry!(0x0082_8260, shield_effect_finish(Ptr)),
        entry!(
            0x0082_82e0,
            shield_effect_modify_additional_actor_value(Ptr, u32, f32)
        ),
        entry!(0x0082_83a0, shield_effect_get_save_size(Ptr, u32) -> u16),
        entry!(0x0082_83f0, shield_effect_save_game(Ptr, u32)),
        entry!(0x0082_8430, shield_effect_load_game(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SLOT_3B4_DOUBLE: u32 = 0x0e80_0000;
    const SLOT_39C_DOUBLE: u32 = 0x0e80_0100;
    const SLOT_3AC_DOUBLE: u32 = 0x0e80_0200;
    const COPY_DATA_DOUBLE: u32 = 0x0e80_0300;

    fn calls(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    struct World {
        e: Engine,
        this: u32,
        actor: u32,
    }

    /// A shield effect (magnitude 3.0, duration `duration`, spell type `spell_type`,
    /// additional value 7) whose target is an actor with doubled slots, the effect
    /// setting's flag 2 answering `flag`, every callee doubled.
    fn world(flag: bool, duration: f32, spell_type: u32) -> World {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x100);
        e.map(0x011d_e000, 0x1000);
        e.set_global::<f64>(COMPARED_DOUBLE, 0.0);
        e.set_global::<u32>(SAVE_GAME, 0x3000);
        let own_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(own_vtable + SLOT_COPY_DATA, COPY_DATA_DOUBLE);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this, own_vtable);
        e.mem.set_u32(this + SPELL, 0x8000);
        e.mem.set_u32(this + EFFECT, 0x9000);
        e.mem.set_f32(this + MAGNITUDE, 3.0);
        e.mem.set_f32(this + DURATION, duration);
        e.mem.set_u32(this + 0x24, 0xa000);
        e.mem.set_u32(this + CASTER, 0xb000);
        e.mem.set_u32(this + SPELL_TYPE, spell_type);
        e.mem.set_u32(this + ADDITIONAL_ACTOR_VALUE, 7);
        let actor_vtable = e.mem.alloc(0x400);
        e.mem
            .set_u32(actor_vtable + ACTOR_SLOT_3B4, SLOT_3B4_DOUBLE);
        e.mem
            .set_u32(actor_vtable + ACTOR_SLOT_39C, SLOT_39C_DOUBLE);
        e.mem
            .set_u32(actor_vtable + ACTOR_SLOT_3AC, SLOT_3AC_DOUBLE);
        let actor = e.mem.alloc(0x10);
        e.mem.set_u32(actor, actor_vtable);
        e.register(ACTIVE_EFFECT_TARGET, |e, a| {
            e.mem.u32(a[0] + 0x24).into_ret()
        });
        e.register_double(TARGET_AS_ACTOR, move |_, _| actor.into_ret());
        e.register(EFFECT_ITEM_SETTING, |_, a| (a[0] + 1).into_ret());
        e.register_double(EFFECT_SETTING_HAS_FLAG, move |_, _| {
            u32::from(flag).into_ret()
        });
        e.register(ADDITIONAL_ACTOR_VALUE_GETTER, |e, a| {
            e.mem.u32(a[0] + ADDITIONAL_ACTOR_VALUE).into_ret()
        });
        e.register(EFFECT_ITEM_ACTOR_VALUE, |_, _| 0x21u32.into_ret());
        e.register(SET_WORD_48, |e, a| {
            e.mem.set_u32(a[0] + 0x48, a[1]);
            Ret::default()
        });
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        for callee in [
            SLOT_3B4_DOUBLE,
            SLOT_39C_DOUBLE,
            SLOT_3AC_DOUBLE,
            COPY_DATA_DOUBLE,
            VALUE_MODIFIER_EFFECT_CONSTRUCTOR,
            VALUE_MODIFIER_EFFECT_COPY_DATA,
            VALUE_MODIFIER_EFFECT_START,
            VALUE_MODIFIER_EFFECT_UPDATE,
            VALUE_MODIFIER_EFFECT_FINISH,
            VALUE_MODIFIER_EFFECT_SAVE_GAME,
            VALUE_MODIFIER_EFFECT_LOAD_GAME,
            SAVE_WRITE,
            SAVE_READ,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        World { e, this, actor }
    }

    fn modified(w: &World) -> Vec<(u32, Vec<u32>)> {
        let mut out = vec![];
        for slot in [SLOT_3B4_DOUBLE, SLOT_39C_DOUBLE, SLOT_3AC_DOUBLE] {
            for words in calls(&w.e, slot) {
                out.push((slot, words));
            }
        }
        out
    }

    #[test]
    fn constructor_copy_and_factory() {
        let mut w = world(false, 0.0, 0);
        let block = w.e.mem.alloc(0x50);
        assert_eq!(w.e.call(0x0082_7f20, &[block, 1, 2, 0x9000]).u32(), block);
        assert_eq!(
            calls(&w.e, VALUE_MODIFIER_EFFECT_CONSTRUCTOR),
            vec![vec![block, 1, 2, 0x9000]]
        );
        assert_eq!(calls(&w.e, EFFECT_ITEM_ACTOR_VALUE), vec![vec![0x9000]]);
        assert_eq!(w.e.mem.u32(block), SHIELD_EFFECT_VTABLE);
        assert_eq!(w.e.mem.u32(block + 0x48), 0x21);

        let copy = w.e.mem.alloc(0x50);
        w.e.register_double(ALLOCATE, move |_, _| copy.into_ret());
        assert_eq!(w.e.call(0x0082_7fa0, &[w.this]).u32(), copy);
        assert_eq!(
            calls(&w.e, VALUE_MODIFIER_EFFECT_CONSTRUCTOR)[1],
            vec![copy, 0xb000, 0x8000, 0x9000]
        );
        assert_eq!(calls(&w.e, COPY_DATA_DOUBLE), vec![vec![w.this, copy]]);

        assert_eq!(w.e.call(0x0082_80a0, &[4, 5, 6]).u32(), copy);
        assert_eq!(calls(&w.e, ALLOCATE), vec![vec![0x50], vec![0x50]]);

        w.e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(w.e.call(0x0082_80a0, &[4, 5, 6]).u32(), 0);
        assert_eq!(w.e.call(0x0082_7fa0, &[w.this]).u32(), 0);
        assert_eq!(calls(&w.e, COPY_DATA_DOUBLE)[1], vec![w.this, 0]);
        assert_eq!(calls(&w.e, VALUE_MODIFIER_EFFECT_CONSTRUCTOR).len(), 3);
    }

    #[test]
    fn copy_data_copies_the_additional_value_to_a_shield() {
        let mut w = world(false, 0.0, 0);
        let target = w.e.mem.alloc(0x50);
        w.e.call(0x0082_8050, &[w.this, target]);
        assert_eq!(
            calls(&w.e, VALUE_MODIFIER_EFFECT_COPY_DATA),
            vec![vec![w.this, target]]
        );
        assert_eq!(
            calls(&w.e, DYNAMIC_CAST),
            vec![vec![target, 0, ACTIVE_EFFECT_TYPE, SHIELD_EFFECT_TYPE, 0]]
        );
        assert_eq!(w.e.mem.u32(target + ADDITIONAL_ACTOR_VALUE), 7);
        w.e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        let other = w.e.mem.alloc(0x50);
        w.e.call(0x0082_8050, &[w.this, other]);
        assert_eq!(w.e.mem.u32(other + ADDITIONAL_ACTOR_VALUE), 0);
    }

    #[test]
    fn start_adds_the_magnitude_when_flagged_or_instant() {
        let mut w = world(true, 5.0, 0);
        w.e.call(0x0082_8120, &[w.this]);
        assert_eq!(calls(&w.e, VALUE_MODIFIER_EFFECT_START), vec![vec![w.this]]);
        assert_eq!(
            modified(&w),
            vec![(SLOT_39C_DOUBLE, vec![w.actor, 7, 3.0f32.to_bits(), 0])]
        );
        // Without the flag, only a duration equal to 0.0 counts.
        let mut w = world(false, 0.0, 0);
        w.e.call(0x0082_8120, &[w.this]);
        assert_eq!(
            modified(&w),
            vec![(SLOT_3AC_DOUBLE, vec![w.actor, 7, 3.0f32.to_bits(), 0])]
        );
        let mut w = world(false, 5.0, 0);
        w.e.call(0x0082_8120, &[w.this]);
        assert!(modified(&w).is_empty());
        let mut w = world(false, f32::NAN, 0);
        w.e.call(0x0082_8120, &[w.this]);
        assert!(modified(&w).is_empty());
    }

    #[test]
    fn update_adds_magnitude_times_seconds() {
        let mut w = world(false, 5.0, 0);
        w.e.call(0x0082_81b0, &args![w.this, 0.5f32]);
        assert_eq!(
            calls(&w.e, VALUE_MODIFIER_EFFECT_UPDATE),
            vec![vec![w.this, 0.5f32.to_bits()]]
        );
        assert_eq!(
            modified(&w),
            vec![(SLOT_3AC_DOUBLE, vec![w.actor, 7, 1.5f32.to_bits(), 0])]
        );
        // A zero duration counts only for spell type 4.
        let mut w = world(false, 0.0, 0);
        w.e.call(0x0082_81b0, &args![w.this, 0.5f32]);
        assert!(modified(&w).is_empty());
        let mut w = world(false, 0.0, 4);
        w.e.call(0x0082_81b0, &args![w.this, 0.5f32]);
        assert_eq!(
            modified(&w),
            vec![(SLOT_3B4_DOUBLE, vec![w.actor, 7, 1.5f32.to_bits()])]
        );
        // With flag 2 nothing is added.
        let mut w = world(true, 5.0, 0);
        w.e.call(0x0082_81b0, &args![w.this, 0.5f32]);
        assert!(modified(&w).is_empty());
        assert!(calls(&w.e, TARGET_AS_ACTOR).is_empty());
    }

    #[test]
    fn finish_takes_the_magnitude_back_when_flagged() {
        let mut w = world(true, 5.0, 0);
        w.e.call(0x0082_8260, &[w.this]);
        assert_eq!(
            calls(&w.e, VALUE_MODIFIER_EFFECT_FINISH),
            vec![vec![w.this]]
        );
        assert_eq!(
            modified(&w),
            vec![(SLOT_39C_DOUBLE, vec![w.actor, 7, (-3.0f32).to_bits(), 0])]
        );
        let mut w = world(false, 5.0, 0);
        w.e.call(0x0082_8260, &[w.this]);
        assert!(modified(&w).is_empty());
    }

    #[test]
    fn modify_skips_without_actor_or_for_value_0x4d() {
        let mut w = world(false, 5.0, 0);
        w.e.call(0x0082_82e0, &args![w.this, 0u32, 1.0f32]);
        assert!(calls(&w.e, ADDITIONAL_ACTOR_VALUE_GETTER).is_empty());
        w.e.mem.set_u32(w.this + ADDITIONAL_ACTOR_VALUE, 0x4d);
        w.e.call(0x0082_82e0, &args![w.this, w.actor, 1.0f32]);
        assert!(modified(&w).is_empty());
        // A signalling NaN amount goes quiet.
        w.e.mem.set_u32(w.this + ADDITIONAL_ACTOR_VALUE, 9);
        w.e.call(0x0082_82e0, &[w.this, w.actor, 0x7f80_0001]);
        assert_eq!(
            modified(&w),
            vec![(SLOT_3AC_DOUBLE, vec![w.actor, 9, 0x7fc0_0001, 0])]
        );
    }

    #[test]
    fn save_size_save_and_load_by_version() {
        let mut w = world(false, 5.0, 0);
        w.e.register(SAVE_VERSION, |_, _| 0x6eu32.into_ret());
        w.e.register(VALUE_MODIFIER_EFFECT_GET_SAVE_SIZE, |_, _| {
            0xfffeu32.into_ret()
        });
        assert_eq!(w.e.call(0x0082_83a0, &[w.this, 9]).u32(), 2);
        w.e.call(0x0082_83f0, &[w.this, 9]);
        w.e.call(0x0082_8430, &[w.this, 9]);
        assert_eq!(
            calls(&w.e, SAVE_WRITE),
            vec![vec![0x3000, w.this + ADDITIONAL_ACTOR_VALUE, 4]]
        );
        assert_eq!(
            calls(&w.e, SAVE_READ),
            vec![vec![0x3000, w.this + ADDITIONAL_ACTOR_VALUE, 4]]
        );
        assert_eq!(
            calls(&w.e, VALUE_MODIFIER_EFFECT_SAVE_GAME),
            vec![vec![w.this, 9]]
        );
        assert_eq!(
            calls(&w.e, VALUE_MODIFIER_EFFECT_LOAD_GAME),
            vec![vec![w.this, 9]]
        );

        w.e.register(SAVE_VERSION, |_, _| 0x6du32.into_ret());
        assert_eq!(w.e.call(0x0082_83a0, &[w.this, 9]).u32(), 0xfffe);
        w.e.call(0x0082_83f0, &[w.this, 9]);
        w.e.call(0x0082_8430, &[w.this, 9]);
        assert_eq!(calls(&w.e, SAVE_WRITE).len(), 1);
        assert_eq!(calls(&w.e, SAVE_READ).len(), 1);
    }
}
