//! `fallout/magic/sundamageeffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `SunDamageEffect` (Xbox PDB, 0x50 bytes): the `ActiveEffect` base (0x48 bytes; `pSpell`
//! +8, `pEffect` +0xC, `fMagnitude` +0x1C, `pTarget` +0x24, `pCaster` +0x28, `pHitEffects`
//! +0x40) plus `fGlowEffect` +0x48, `bHitEffectApplied` +0x4C and
//! `bDoNotTakeDamageNextUpdate` +0x4D (the PC code uses the same offsets). The compiler's
//! exception frames (`FS:[0]`) are not translated; the functions that have them carry a
//! `Translation incomplete` marker.

#[allow(unused_imports)]
use crate::prelude::*;

/// `SunDamageEffect`'s vtable (`0107c3ec`).
const SUN_DAMAGE_EFFECT_VTABLE: u32 = 0x0107_c3ec;
/// `ActiveEffect` fields (Xbox PDB).
const SPELL: u32 = 0x8;
const EFFECT: u32 = 0xc;
const CASTER: u32 = 0x28;
const HIT_EFFECTS: u32 = 0x40;
/// `fGlowEffect`, `bHitEffectApplied`, `bDoNotTakeDamageNextUpdate` (Xbox PDB).
const GLOW_EFFECT: u32 = 0x48;
const HIT_EFFECT_APPLIED: u32 = 0x4c;
const SKIP_NEXT_DAMAGE: u32 = 0x4d;

/// The allocators `00401000` and `00aa13e0` (`cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
const ALLOCATE_AA13E0: u32 = 0x00aa_13e0;
/// The `ActiveEffect` constructor `00803d30` (`thiscall`, three words).
const ACTIVE_EFFECT_CONSTRUCTOR: u32 = 0x0080_3d30;
/// The `ActiveEffect` vtable slot `Copy` hands the copy to (`CopyData`, +0x44).
const SLOT_COPY_DATA: u32 = 0x44;
/// `0059bb30` (`thiscall`): the effect's `pTarget` (+0x24).
const ACTIVE_EFFECT_TARGET: u32 = 0x0059_bb30;
/// The magic target's vtable slot +4 (no stack words; `GetTargetStatsObject` in the Xbox
/// PDB's `MagicTarget` slot list): its reference.
const TARGET_SLOT_REFERENCE: u32 = 0x4;
/// `008d6f30` (`thiscall`): the reference's word at +0x40 (its cell).
const REFERENCE_CELL: u32 = 0x008d_6f30;
/// `TESObjectREFR::GetInterior` (engine map, `00575d10`; `thiscall`): a byte.
const REFERENCE_IS_INTERIOR: u32 = 0x0057_5d10;
/// `00454b10` and `004518e0` (`thiscall` on a cell): bits 0x80 and 0x2 of its byte at
/// +0x24.
const CELL_FLAG_80: u32 = 0x0045_4b10;
const CELL_FLAG_02: u32 = 0x0045_18e0;
/// The player (`PlayerCharacter*` global at `011dea3c`); its `MagicTarget` base is at
/// +0x94.
const PLAYER: u32 = 0x011d_ea3c;
const PLAYER_MAGIC_TARGET: u32 = 0x94;
/// A `float` the effect keeps across calls (`011a14f8`): the countdown of the player's
/// pain reaction.
const PAIN_TIMER: u32 = 0x011a_14f8;
/// `00403e20` (`thiscall`): the address of a setting's value (+4); the settings at
/// `011d1944` and `011d18d8` restart the countdown.
const SETTING_VALUE: u32 = 0x0040_3e20;
const SETTING_011D1944: u32 = 0x011d_1944;
const SETTING_011D18D8: u32 = 0x011d_18d8;
/// The `double`s the damage and the countdown are compared with (`01016408`, `01012060`,
/// `010231b0`).
const DAMAGE_THRESHOLD: u32 = 0x0101_6408;
const COMPARED_DOUBLE: u32 = 0x0101_2060;
const RESTART_010231B0: u32 = 0x0102_31b0;
/// The player's vtable slot +0x338 (three words: amount, 0.0, 0) the damage goes to.
const PLAYER_SLOT_338: u32 = 0x338;
/// `Actor::TriggerPain` (engine map, `008a7d50`; `thiscall`, two words).
const ACTOR_TRIGGER_PAIN: u32 = 0x008a_7d50;
/// The object at `011dea10` and `008d8520` (`thiscall`): the word at its +0x68 (the
/// engine map's name, `MiddleHighProcess::GetSavedAcquireObject`, is folded code).
const OBJECT_011DEA10: u32 = 0x011d_ea10;
const WORD_68: u32 = 0x008d_8520;
/// `0044edb0` and `00825c00` (`thiscall`): the words at +0x10 and +0x14.
const WORD_10: u32 = 0x0044_edb0;
const WORD_14: u32 = 0x0082_5c00;
/// `004ed230` (`teswater.cpp`; `thiscall`, three words): a float in `ST0`.
const WATER_VALUE_004ED230: u32 = 0x004e_d230;
/// `006447d0` (`thiscall`): the float at +0xF4 in `ST0`.
const FLOAT_F4: u32 = 0x0064_47d0;
/// `00966a20`, `00595ea0`, `00596030` (`thiscall`): floats in `ST0`.
const INPUT_00966A20: u32 = 0x0096_6a20;
const INPUT_00595EA0: u32 = 0x0059_5ea0;
const INPUT_00596030: u32 = 0x0059_6030;
/// `00495460` (`thiscall`): the effect's `fMagnitude` (+0x1C) in `ST0`.
const EFFECT_MAGNITUDE: u32 = 0x0049_5460;
/// `TESObjectREFR::GetRelevantWaterHeight` (engine map, `0057b0a0`; `thiscall`): a float
/// in `ST0`.
const REFERENCE_WATER_HEIGHT: u32 = 0x0057_b0a0;
/// The reference's vtable slot +0x1F4 (no stack words): the address of its position.
const SLOT_POSITION: u32 = 0x1f4;
/// `00649390` (`magicformulas.cpp`; `cdecl`, seven words): the damage, in `ST0`.
const SUN_DAMAGE_FORMULA: u32 = 0x0064_9390;
/// `007df1d0` (`thiscall` on an effect): its `EffectSetting` (`00825c00` of `pEffect`);
/// `00407820` (`thiscall` on an `EffectSetting`): its word at +0x78 (the shader the hit
/// effect uses).
const EFFECT_SETTING: u32 = 0x007d_f1d0;
const SETTING_SHADER: u32 = 0x0040_7820;
/// `0055b980` (`thiscall`): the effect's word at +0x2C (`eSpellType`).
const SPELL_TYPE: u32 = 0x0055_b980;
/// List helpers (`thiscall`): empty test (`008256d0`), clear (`00470470`), deleting
/// destructor (`004702f0`, one word), node construction (`00470440`, one word: the
/// address of the item), item slot (`006815c0`) and next node (`00726070`).
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_CLEAR: u32 = 0x0047_0470;
const LIST_DELETE: u32 = 0x0047_02f0;
const LIST_NODE_CONSTRUCT: u32 = 0x0047_0440;
const NODE_ITEM_SLOT: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
/// The `MagicShaderHitEffect` constructor (`0081f580`; `thiscall`, three words:
/// reference, shader, float) and the float it gets (`01012054`).
const SHADER_HIT_EFFECT_CONSTRUCTOR: u32 = 0x0081_f580;
const HIT_EFFECT_FLOAT: u32 = 0x0101_2054;
/// The hit effect's vtable slots +0xC4 (no stack words: whether it can be added) and +0
/// (one word: the deleting destructor).
const SLOT_C4: u32 = 0xc4;
/// `00984f60` (`thiscall`, one word): stores the word at +0x18 of the hit effect.
const HIT_EFFECT_SET_18: u32 = 0x0098_4f60;
/// `00461310` (`thiscall`, one word): stores the byte at +0x24 of the hit effect.
const HIT_EFFECT_SET_24: u32 = 0x0046_1310;
/// The process lists (`011e0e80`), `ProcessLists::AddTempEffect` (`00973fd0`, one word)
/// and `ProcessLists::FinishMagicShaderHitEffect` (`00974a50`, two words) (engine map).
const PROCESS_LISTS: u32 = 0x011e_0e80;
const ADD_TEMP_EFFECT: u32 = 0x0097_3fd0;
const FINISH_MAGIC_SHADER_HIT_EFFECT: u32 = 0x0097_4a50;
/// `ActiveEffect::GetSaveSize`, `::SaveGame`, `::LoadGame` (engine map, `00804f80`,
/// `008050d0`, `00805330`; `thiscall`, one word each).
const ACTIVE_EFFECT_GET_SAVE_SIZE: u32 = 0x0080_4f80;
const ACTIVE_EFFECT_SAVE_GAME: u32 = 0x0080_50d0;
const ACTIVE_EFFECT_LOAD_GAME: u32 = 0x0080_5330;
/// The save game object (`011de45c`), its version byte (`008df040`, +0x80) and its raw
/// write (`008579b0`) and read (`008579e0`) (`thiscall`, two words: address, size).
const SAVE_GAME: u32 = 0x011d_e45c;
const SAVE_VERSION: u32 = 0x008d_f040;
const SAVE_WRITE: u32 = 0x0085_79b0;
const SAVE_READ: u32 = 0x0085_79e0;
/// The save version from which `fGlowEffect` is saved.
const VERSION_GLOW: i32 = 0x71;

/// The save game object's version byte, as the exe compares it (`MOVZX`, signed `CMP`).
fn save_version(e: &mut Engine) -> i32 {
    let save_game: u32 = e.global(SAVE_GAME);
    i32::from(e.call(SAVE_VERSION, &args![save_game]).u8())
}

/// The target's reference: `pTarget`'s slot +4 (the caller has checked the target).
fn target_reference(e: &mut Engine, this: Ptr) -> u32 {
    let target = e.call(ACTIVE_EFFECT_TARGET, &args![this]).u32();
    e.vcall(target, TARGET_SLOT_REFERENCE, &args![]).u32()
}

/// The float both `00829bb0` and `0082a1b0` compute from the object `008d8520` answers
/// (the same instructions in each; x87, rounded to `float` at each `float` store): the
/// `004ed230` value (with 5, 1.0, 0.0) of its word at +0x10 (0.0 when null), and, when its
/// word at +0x14 is not null, that value times its float at +0xF4 plus the +0x14 word's
/// `004ed230` value times `1 - float at +0xF4` (the latter kept in a `double`).
fn blended_water_value(e: &mut Engine, object: u32) -> u32 {
    let mut value = if e.call(WORD_10, &args![object]).u32() != 0 {
        let word = e.call(WORD_10, &args![object]).u32();
        let result = e
            .call(WATER_VALUE_004ED230, &args![word, 5u32, 1.0f32, 0.0f32])
            .f64();
        x87_store32(result)
    } else {
        x87_store32(0.0)
    };
    if e.call(WORD_14, &args![object]).u32() != 0 {
        let weight = x87_store32(e.call(FLOAT_F4, &args![object]).f64());
        value = x87_store32(x87_mul(x87_load32(value), x87_m32(weight)));
        // FLD weight; FLD1; FSUBRP: 1.0 - weight, stored as a `double`.
        let rest = x87_sub(1.0, x87_load32(weight));
        let word = e.call(WORD_14, &args![object]).u32();
        let other = e
            .call(WATER_VALUE_004ED230, &args![word, 5u32, 1.0f32, 0.0f32])
            .f64();
        value = x87_store32(x87_add(x87_mul(other, rest), x87_m32(value)));
    }
    value
}

// Translated from 00829a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `SunDamageEffect` constructor (by the vtable `SunDamageEffect::Copy` gives its
/// copies; unnamed in the engine map; `thiscall`, `RET 0xc`): the `ActiveEffect`
/// constructor `00803d30` with the three words, the vtable, `fGlowEffect` 0.0,
/// `bDoNotTakeDamageNextUpdate` and `bHitEffectApplied` cleared. Returns `this`.
pub fn fn_00829a70(e: &mut Engine, this: Ptr, caster: u32, spell: u32, effect: u32) -> Ptr {
    let base = this.addr();
    e.call(
        ACTIVE_EFFECT_CONSTRUCTOR,
        &args![this, caster, spell, effect],
    );
    e.mem.set_u32(base, SUN_DAMAGE_EFFECT_VTABLE);
    e.mem.set_f32(base + GLOW_EFFECT, 0.0);
    e.mem.set_u8(base + SKIP_NEXT_DAMAGE, 0);
    e.mem.set_u8(base + HIT_EFFECT_APPLIED, 0);
    this
}

// Translated from 00829ac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00829ac0 - the C++ exception-unwinding frame (FS:[0]) that frees the allocation when the constructor throws is not translated
/// `SunDamageEffect::Copy` (Xbox PDB): allocates a `SunDamageEffect` (0x50 bytes),
/// constructs it with this effect's `pCaster`, `pSpell` and `pEffect` (0 when the
/// allocation failed), lets this effect copy its data into it (vtable +0x44), then copies
/// `fGlowEffect` (`FLD`/`FSTP`), `bHitEffectApplied` and `bDoNotTakeDamageNextUpdate`
/// into it (into address 0 too, as the exe does, when the allocation failed). Returns the
/// copy.
pub fn sun_damage_effect_copy(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    let block = e.call(ALLOCATE, &args![0x50u32]).u32();
    let copy = if block == 0 {
        0
    } else {
        let caster = e.mem.u32(base + CASTER);
        let spell = e.mem.u32(base + SPELL);
        let effect = e.mem.u32(base + EFFECT);
        fn_00829a70(e, Ptr::new(block), caster, spell, effect).addr()
    };
    e.vcall(base, SLOT_COPY_DATA, &args![copy]);
    let glow = x87_quiet_bits(e.mem.u32(base + GLOW_EFFECT));
    e.mem.set_u32(copy + GLOW_EFFECT, glow);
    let applied = e.mem.u8(base + HIT_EFFECT_APPLIED);
    e.mem.set_u8(copy + HIT_EFFECT_APPLIED, applied);
    let skip = e.mem.u8(base + SKIP_NEXT_DAMAGE);
    e.mem.set_u8(copy + SKIP_NEXT_DAMAGE, skip);
    copy
}

// Translated from 00829bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SunDamageEffect::Update` (Xbox PDB; `thiscall`, `RET 4`: the seconds). A set
/// `bDoNotTakeDamageNextUpdate` is cleared and nothing else happens. Without a target or
/// its reference nothing happens. A reference inside (`00575d10`) a cell without flag 0x80
/// (`00454b10`), or without a cell, turns the hit effect off (`00829e00` with 0) and, for
/// the player, restarts the pain countdown (`011a14f8`) from the setting `011d1944`.
/// Otherwise the damage (`0082a1b0`, rounded to `float`) turns the hit effect on when it is
/// above the `double` at `01016408`; for the player such damage times the seconds goes to
/// its slot +0x338 (with 0.0 and 0), the countdown triggers the pain reaction
/// (`008a7d50` with 1, 1) and restarts from `011d18d8` when below the `double` at
/// `01012060`, restarts from `011d1944` when equal to the `double` at `010231b0`, and loses
/// the seconds. For the player the function then computes the blended value of the object
/// `008d8520` answers for `011dea10` and keeps it nowhere (the calls are made).
pub fn sun_damage_effect_update(e: &mut Engine, this: Ptr, seconds: f32) {
    let base = this.addr();
    if e.mem.u8(base + SKIP_NEXT_DAMAGE) != 0 {
        e.mem.set_u8(base + SKIP_NEXT_DAMAGE, 0);
        return;
    }
    if e.call(ACTIVE_EFFECT_TARGET, &args![this]).u32() == 0 {
        return;
    }
    if target_reference(e, this) == 0 {
        return;
    }
    let reference = target_reference(e, this);
    let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
    let player: u32 = e.global(PLAYER);
    if e.call(REFERENCE_IS_INTERIOR, &args![reference]).bool()
        && (cell == 0 || !e.call(CELL_FLAG_80, &args![cell]).bool())
    {
        fn_00829e00(e, this, 0);
        if reference == player {
            let value = e.call(SETTING_VALUE, &args![SETTING_011D1944]).u32();
            let restart = x87_quiet_bits(e.mem.u32(value));
            e.set_global::<u32>(PAIN_TIMER, restart);
        }
        return;
    }
    let damage = x87_store32(fn_0082a1b0(e, this));
    let threshold: f64 = e.global(DAMAGE_THRESHOLD);
    let above = x87_load32(damage) > threshold;
    fn_00829e00(e, this, u8::from(above));
    if reference != player {
        return;
    }
    let threshold: f64 = e.global(DAMAGE_THRESHOLD);
    if x87_load32(damage) > threshold {
        let amount = x87_store32(x87_mul(x87_load32(damage), x87_m32(seconds.to_bits())));
        let player: u32 = e.global(PLAYER);
        e.vcall(
            player,
            PLAYER_SLOT_338,
            &args![x87_quiet_bits(amount), 0.0f32, 0u32],
        );
        let timer = x87_load32(e.global::<u32>(PAIN_TIMER));
        let compared: f64 = e.global(COMPARED_DOUBLE);
        // TEST AH,0x05; JP: on to the pain reaction only when below (ordered).
        if timer < compared {
            let player: u32 = e.global(PLAYER);
            e.call(ACTOR_TRIGGER_PAIN, &args![player, 1u32, 1u32]);
            let value = e.call(SETTING_VALUE, &args![SETTING_011D18D8]).u32();
            let restart = x87_quiet_bits(e.mem.u32(value));
            e.set_global::<u32>(PAIN_TIMER, restart);
        } else {
            let timer = x87_load32(e.global::<u32>(PAIN_TIMER));
            let compared: f64 = e.global(RESTART_010231B0);
            // TEST AH,0x44; JP: on only when equal (ordered).
            if timer == compared {
                let value = e.call(SETTING_VALUE, &args![SETTING_011D1944]).u32();
                let restart = x87_quiet_bits(e.mem.u32(value));
                e.set_global::<u32>(PAIN_TIMER, restart);
            }
        }
        let timer = x87_load32(e.global::<u32>(PAIN_TIMER));
        let left = x87_store32(x87_sub(timer, x87_m32(seconds.to_bits())));
        e.set_global::<u32>(PAIN_TIMER, left);
    }
    let holder: u32 = e.global(OBJECT_011DEA10);
    let object = e.call(WORD_68, &args![holder]).u32();
    let _unused_value = blended_water_value(e, object);
}

// Translated from 00829e00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00829e00 - the C++ exception-unwinding frame (FS:[0]) that frees the hit effect or its list node when a constructor throws is not translated
/// Unnamed in the engine map (`thiscall`, `RET 4`: a byte): turns the effect's shader hit
/// effect on or off. Nothing happens without the effect setting's shader (`007df1d0`,
/// `00407820`), without a target or when the spell type (`0055b980`) is not 4. On: when
/// `pHitEffects` is null or empty, any old list is cleared and deleted, then a 0x6c-byte
/// hit effect is built (`00aa13e0`, `0081f580` with the target's reference, the shader and
/// the float at `01012054`); when its slot +0xC4 answers true it is added to the process
/// lists (`00973fd0`), put in a new one-node list (8 bytes, `00401000`, `00470440`) that
/// becomes `pHitEffects`, and given this effect (`00984f60`); otherwise it is deleted
/// (slot 0 with 1). Off: with a list, each hit effect until an empty node gets its byte
/// +0x24 set (`00461310` with 1) and its +0x18 word cleared (`00984f60` with 0), the list
/// is cleared, deleted and dropped, and the process lists finish the reference's shader
/// hit effect (`00974a50`).
pub fn fn_00829e00(e: &mut Engine, this: Ptr, on: u8) {
    let base = this.addr();
    let setting = e.call(EFFECT_SETTING, &args![this]).u32();
    let shader = e.call(SETTING_SHADER, &args![setting]).u32();
    if shader == 0
        || e.call(ACTIVE_EFFECT_TARGET, &args![this]).u32() == 0
        || e.call(SPELL_TYPE, &args![this]).u32() != 4
    {
        return;
    }
    if on != 0 {
        let list = e.mem.u32(base + HIT_EFFECTS);
        if list != 0 && !e.call(LIST_IS_EMPTY, &args![list]).bool() {
            return;
        }
        if e.mem.u32(base + HIT_EFFECTS) != 0 {
            let list = e.mem.u32(base + HIT_EFFECTS);
            e.call(LIST_CLEAR, &args![list]);
            let list = e.mem.u32(base + HIT_EFFECTS);
            if list != 0 {
                e.call(LIST_DELETE, &args![list, 1u32]);
            }
            e.mem.set_u32(base + HIT_EFFECTS, 0);
        }
        let block = e.call(ALLOCATE_AA13E0, &args![0x6cu32]).u32();
        let effect = if block == 0 {
            0
        } else {
            let reference = target_reference(e, this);
            let float = x87_quiet_bits(e.global::<u32>(HIT_EFFECT_FLOAT));
            e.call(
                SHADER_HIT_EFFECT_CONSTRUCTOR,
                &args![block, reference, shader, float],
            )
            .u32()
        };
        if e.vcall(effect, SLOT_C4, &args![]).bool() {
            e.call(ADD_TEMP_EFFECT, &args![PROCESS_LISTS, effect]);
            let node_block = e.call(ALLOCATE, &args![8u32]).u32();
            let node = if node_block == 0 {
                0
            } else {
                e.with_stack(4, |e, item| {
                    e.mem.set_u32(item.addr(), effect);
                    e.call(LIST_NODE_CONSTRUCT, &args![node_block, item]).u32()
                })
            };
            e.mem.set_u32(base + HIT_EFFECTS, node);
            e.call(HIT_EFFECT_SET_18, &args![effect, base]);
        } else if effect != 0 {
            e.vcall(effect, 0, &args![1u32]);
        }
        return;
    }
    if e.mem.u32(base + HIT_EFFECTS) == 0 {
        return;
    }
    let mut node = e.mem.u32(base + HIT_EFFECTS);
    while node != 0 && !e.call(LIST_IS_EMPTY, &args![node]).bool() {
        let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
        let hit_effect = e.mem.u32(slot);
        e.call(HIT_EFFECT_SET_24, &args![hit_effect, 1u32]);
        e.call(HIT_EFFECT_SET_18, &args![hit_effect, 0u32]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    let list = e.mem.u32(base + HIT_EFFECTS);
    e.call(LIST_CLEAR, &args![list]);
    let list = e.mem.u32(base + HIT_EFFECTS);
    if list != 0 {
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
    e.mem.set_u32(base + HIT_EFFECTS, 0);
    let reference = target_reference(e, this);
    e.call(
        FINISH_MAGIC_SHADER_HIT_EFFECT,
        &args![PROCESS_LISTS, reference, shader],
    );
}

// Translated from 0082a0d0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082a0d0 - the FS:[0] exception frame (freeing the block when the constructor throws) is not translated
/// Unnamed in the engine map (`cdecl`, three words): allocates a 0x50-byte
/// `SunDamageEffect` (`00401000`) and constructs it with the three words (`00829a70`);
/// returns it, or null when the allocation fails.
pub fn fn_0082a0d0(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x50u32]).u32();
    if block == 0 {
        return 0;
    }
    fn_00829a70(e, Ptr::new(block), caster, spell, effect).addr()
}

// Translated from 0082a150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SunDamageEffect::IsCausingDamage` (Xbox PDB): true when the target is the player's
/// magic target (the player + 0x94; null without a player) and the damage (`0082a1b0`, the
/// full `ST0`) is above the `double` at `01016408` (ordered).
pub fn sun_damage_effect_is_causing_damage(e: &mut Engine, this: Ptr) -> bool {
    let player: u32 = e.global(PLAYER);
    let player_target = if player == 0 {
        0
    } else {
        player + PLAYER_MAGIC_TARGET
    };
    if e.call(ACTIVE_EFFECT_TARGET, &args![this]).u32() != player_target {
        return false;
    }
    let damage = fn_0082a1b0(e, this);
    let threshold: f64 = e.global(DAMAGE_THRESHOLD);
    damage > threshold
}

// Translated from 0082a1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): the sun damage in `ST0`. 0.0 without a target
/// reference, or for a reference inside (`00575d10`) a cell without flag 0x80 or without a
/// cell. Otherwise `00649390` of the effect's magnitude (`00495460`), three floats of the
/// object `008d8520` answers for `011dea10` (`00966a20`, `00595ea0`, `00596030`), 1, its
/// blended value (see `blended_water_value`) and whether the reference stands at or below
/// its water height (when its cell has flag 2: `0057b0a0` against the position's z,
/// vtable +0x1F4).
pub fn fn_0082a1b0(e: &mut Engine, this: Ptr) -> f64 {
    let reference = if e.call(ACTIVE_EFFECT_TARGET, &args![this]).u32() != 0 {
        target_reference(e, this)
    } else {
        0
    };
    let cell = if reference != 0 {
        e.call(REFERENCE_CELL, &args![reference]).u32()
    } else {
        0
    };
    if reference == 0 {
        return 0.0;
    }
    if e.call(REFERENCE_IS_INTERIOR, &args![reference]).bool()
        && (cell == 0 || !e.call(CELL_FLAG_80, &args![cell]).bool())
    {
        return 0.0;
    }
    let holder: u32 = e.global(OBJECT_011DEA10);
    let object = e.call(WORD_68, &args![holder]).u32();
    let first = x87_store32(e.call(INPUT_00966A20, &args![object]).f64());
    let second = x87_store32(e.call(INPUT_00595EA0, &args![object]).f64());
    let third = x87_store32(e.call(INPUT_00596030, &args![object]).f64());
    let always: u8 = 1;
    let blended = blended_water_value(e, object);
    let mut under_water: u8 = 0;
    let cell = e.call(REFERENCE_CELL, &args![reference]).u32();
    if e.call(CELL_FLAG_02, &args![cell]).bool() {
        let height = e.call(REFERENCE_WATER_HEIGHT, &args![reference]).f64();
        let position = e.vcall(reference, SLOT_POSITION, &args![]).u32();
        let z = x87_load32(e.mem.u32(position + 8));
        // FCOMP; TEST AH,0x41; JP: set when below or equal (ordered).
        if z <= height {
            under_water = 1;
        }
    }
    let magnitude = x87_store32(e.call(EFFECT_MAGNITUDE, &args![this]).f64());
    e.call(
        SUN_DAMAGE_FORMULA,
        &args![
            magnitude,
            x87_quiet_bits(first),
            x87_quiet_bits(second),
            x87_quiet_bits(third),
            u32::from(always),
            x87_quiet_bits(blended),
            u32::from(under_water)
        ],
    )
    .f64()
}

// Translated from 0082a390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SunDamageEffect::GetSaveSize` (Xbox PDB; `thiscall`, `RET 4`; a 16-bit sum): the
/// `ActiveEffect` size (`00804f80` with the same word), plus 4 from save version 0x71 on.
pub fn sun_damage_effect_get_save_size(e: &mut Engine, this: Ptr, argument: u32) -> u16 {
    let mut size = e
        .call(ACTIVE_EFFECT_GET_SAVE_SIZE, &args![this, argument])
        .u16();
    if save_version(e) >= VERSION_GLOW {
        size = size.wrapping_add(4);
    }
    size
}

// Translated from 0082a3e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SunDamageEffect::SaveGame` (Xbox PDB; `thiscall`, `RET 4`): the `ActiveEffect` data
/// (`008050d0`), then from save version 0x71 on the four bytes of `fGlowEffect`
/// (`008579b0`).
pub fn sun_damage_effect_save_game(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(ACTIVE_EFFECT_SAVE_GAME, &args![this, argument]);
    if save_version(e) >= VERSION_GLOW {
        let save_game: u32 = e.global(SAVE_GAME);
        e.call(
            SAVE_WRITE,
            &args![save_game, this.addr() + GLOW_EFFECT, 4u32],
        );
    }
}

// Translated from 0082a420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SunDamageEffect::LoadGame` (Xbox PDB; `thiscall`, `RET 4`): the `ActiveEffect` data
/// (`00805330`), then from save version 0x71 on the four bytes of `fGlowEffect`
/// (`008579e0`).
pub fn sun_damage_effect_load_game(e: &mut Engine, this: Ptr, argument: u32) {
    e.call(ACTIVE_EFFECT_LOAD_GAME, &args![this, argument]);
    if save_version(e) >= VERSION_GLOW {
        let save_game: u32 = e.global(SAVE_GAME);
        e.call(
            SAVE_READ,
            &args![save_game, this.addr() + GLOW_EFFECT, 4u32],
        );
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082_9a70, fn_00829a70(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x0082_9ac0, sun_damage_effect_copy(Ptr) -> u32),
        entry!(0x0082_9bb0, sun_damage_effect_update(Ptr, f32)),
        entry!(0x0082_9e00, fn_00829e00(Ptr, u8)),
        entry!(0x0082_a0d0, fn_0082a0d0(u32, u32, u32) -> u32),
        entry!(0x0082_a150, sun_damage_effect_is_causing_damage(Ptr) -> bool),
        entry!(0x0082_a1b0, fn_0082a1b0(Ptr) -> f64),
        entry!(0x0082_a390, sun_damage_effect_get_save_size(Ptr, u32) -> u16),
        entry!(0x0082_a3e0, sun_damage_effect_save_game(Ptr, u32)),
        entry!(0x0082_a420, sun_damage_effect_load_game(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const REFERENCE_SLOT_DOUBLE: u32 = 0x0e90_0000;
    const POSITION_DOUBLE: u32 = 0x0e90_0100;
    const PLAYER_338_DOUBLE: u32 = 0x0e90_0200;
    const COPY_DATA_DOUBLE: u32 = 0x0e90_0300;
    const HIT_C4_DOUBLE: u32 = 0x0e90_0400;
    const HIT_DELETE_DOUBLE: u32 = 0x0e90_0500;

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
        reference: u32,
        hit: u32,
    }

    /// A sun damage effect (spell type 4) whose target (+0x24) answers `reference` through
    /// its slot +4; the reference is the player when `is_player`; the reference's cell at
    /// 0x7700; settings, doubles and callees doubled. `0082a1b0`'s inputs are doubled so
    /// that the formula answers `damage`.
    fn world(is_player: bool, damage: f64) -> World {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x100);
        e.map(0x0101_6000, 0x1000);
        e.map(0x0102_3000, 0x1000);
        e.map(0x011a_1000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.set_global::<f64>(DAMAGE_THRESHOLD, 0.0);
        e.set_global::<f64>(COMPARED_DOUBLE, 0.0);
        e.set_global::<f64>(RESTART_010231B0, 1.0);
        e.set_global::<f32>(HIT_EFFECT_FLOAT, -1.0);
        e.set_global::<f32>(PAIN_TIMER, 5.0);
        e.set_global::<u32>(SAVE_GAME, 0x3000);
        e.set_global::<u32>(OBJECT_011DEA10, 0x3100);
        let target_vtable = e.mem.alloc(0x10);
        e.mem.set_u32(target_vtable + 4, REFERENCE_SLOT_DOUBLE);
        let target = e.mem.alloc(0x10);
        e.mem.set_u32(target, target_vtable);
        let own_vtable = e.mem.alloc(0x50);
        e.mem.set_u32(own_vtable + SLOT_COPY_DATA, COPY_DATA_DOUBLE);
        let this = e.mem.alloc(0x50);
        e.mem.set_u32(this, own_vtable);
        e.mem.set_u32(this + 0x24, target);
        e.mem.set_u32(this + 0x2c, 4);
        let reference_vtable = e.mem.alloc(0x400);
        e.mem
            .set_u32(reference_vtable + SLOT_POSITION, POSITION_DOUBLE);
        e.mem
            .set_u32(reference_vtable + PLAYER_SLOT_338, PLAYER_338_DOUBLE);
        let reference = e.mem.alloc(0x100);
        e.mem.set_u32(reference, reference_vtable);
        let player = if is_player {
            reference
        } else {
            let other = e.mem.alloc(0x100);
            e.mem.set_u32(other, reference_vtable);
            other
        };
        e.set_global::<u32>(PLAYER, player);
        let position = e.mem.alloc(0xc);
        e.mem.set_f32(position + 8, 10.0);
        let hit_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(hit_vtable, HIT_DELETE_DOUBLE);
        e.mem.set_u32(hit_vtable + SLOT_C4, HIT_C4_DOUBLE);
        let hit = e.mem.alloc(0x6c);
        e.mem.set_u32(hit, hit_vtable);
        let settings = e.mem.alloc(8);
        e.mem.set_f32(settings, 30.0);
        e.mem.set_f32(settings + 4, 40.0);
        e.register(ACTIVE_EFFECT_TARGET, |e, a| {
            e.mem.u32(a[0] + 0x24).into_ret()
        });
        e.register_double(REFERENCE_SLOT_DOUBLE, move |_, _| reference.into_ret());
        e.register(REFERENCE_CELL, |_, _| 0x7700u32.into_ret());
        e.register(REFERENCE_IS_INTERIOR, |_, _| 0u32.into_ret());
        e.register(CELL_FLAG_80, |_, _| 0u32.into_ret());
        e.register(CELL_FLAG_02, |_, _| 0u32.into_ret());
        e.register_double(SETTING_VALUE, move |_, a| {
            if a[0] == SETTING_011D1944 {
                settings.into_ret()
            } else {
                (settings + 4).into_ret()
            }
        });
        e.register(WORD_68, |_, _| 0x3200u32.into_ret());
        e.register(WORD_10, |_, _| 0u32.into_ret());
        e.register(WORD_14, |_, _| 0u32.into_ret());
        e.register(INPUT_00966A20, |_, _| 1.0f64.into_ret());
        e.register(INPUT_00595EA0, |_, _| 2.0f64.into_ret());
        e.register(INPUT_00596030, |_, _| 3.0f64.into_ret());
        e.register(EFFECT_MAGNITUDE, |_, _| 4.0f64.into_ret());
        e.register(REFERENCE_WATER_HEIGHT, |_, _| 10.0f64.into_ret());
        e.register_double(POSITION_DOUBLE, move |_, _| position.into_ret());
        e.register_double(SUN_DAMAGE_FORMULA, move |_, _| damage.into_ret());
        e.register(EFFECT_SETTING, |_, _| 0x8800u32.into_ret());
        e.register(SETTING_SHADER, |_, _| 0x8900u32.into_ret());
        e.register(SPELL_TYPE, |e, a| e.mem.u32(a[0] + 0x2c).into_ret());
        e.register(LIST_IS_EMPTY, |_, _| 0u32.into_ret());
        e.register_double(ALLOCATE_AA13E0, move |_, _| hit.into_ret());
        e.register(SHADER_HIT_EFFECT_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(HIT_C4_DOUBLE, |_, _| 1u32.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(LIST_NODE_CONSTRUCT, |e, a| {
            let item = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, 0);
            a[0].into_ret()
        });
        e.register(NODE_ITEM_SLOT, |_, a| a[0].into_ret());
        e.register(NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        for callee in [
            PLAYER_338_DOUBLE,
            COPY_DATA_DOUBLE,
            HIT_DELETE_DOUBLE,
            ACTOR_TRIGGER_PAIN,
            ACTIVE_EFFECT_CONSTRUCTOR,
            ADD_TEMP_EFFECT,
            FINISH_MAGIC_SHADER_HIT_EFFECT,
            HIT_EFFECT_SET_18,
            HIT_EFFECT_SET_24,
            LIST_CLEAR,
            LIST_DELETE,
            ACTIVE_EFFECT_SAVE_GAME,
            ACTIVE_EFFECT_LOAD_GAME,
            SAVE_WRITE,
            SAVE_READ,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        World {
            e,
            this,
            reference,
            hit,
        }
    }

    #[test]
    fn constructor_copy_and_factory() {
        let mut w = world(false, 0.0);
        let block = w.e.mem.alloc(0x50);
        for i in 0..20 {
            w.e.mem.set_u32(block + 4 * i, 0xffff_ffff);
        }
        assert_eq!(w.e.call(0x0082_9a70, &[block, 1, 2, 3]).u32(), block);
        assert_eq!(
            calls(&w.e, ACTIVE_EFFECT_CONSTRUCTOR),
            vec![vec![block, 1, 2, 3]]
        );
        assert_eq!(w.e.mem.u32(block), SUN_DAMAGE_EFFECT_VTABLE);
        assert_eq!(w.e.mem.u32(block + GLOW_EFFECT), 0);
        assert_eq!(w.e.mem.u32(block + HIT_EFFECT_APPLIED), 0xffff_0000);

        w.e.mem.set_u32(w.this + SPELL, 0x81);
        w.e.mem.set_u32(w.this + EFFECT, 0x82);
        w.e.mem.set_u32(w.this + CASTER, 0x83);
        w.e.mem.set_u32(w.this + GLOW_EFFECT, 0x7f80_0001);
        w.e.mem.set_u8(w.this + HIT_EFFECT_APPLIED, 1);
        w.e.mem.set_u8(w.this + SKIP_NEXT_DAMAGE, 1);
        let copy = w.e.call(0x0082_9ac0, &[w.this]).u32();
        assert_eq!(
            calls(&w.e, ACTIVE_EFFECT_CONSTRUCTOR)[1],
            vec![copy, 0x83, 0x81, 0x82]
        );
        assert_eq!(calls(&w.e, COPY_DATA_DOUBLE), vec![vec![w.this, copy]]);
        assert_eq!(w.e.mem.u32(copy + GLOW_EFFECT), 0x7fc0_0001);
        assert_eq!(w.e.mem.u8(copy + HIT_EFFECT_APPLIED), 1);
        assert_eq!(w.e.mem.u8(copy + SKIP_NEXT_DAMAGE), 1);

        let made = w.e.call(0x0082_a0d0, &[4, 5, 6]).u32();
        assert_eq!(
            calls(&w.e, ACTIVE_EFFECT_CONSTRUCTOR)[2],
            vec![made, 4, 5, 6]
        );
        w.e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert_eq!(w.e.call(0x0082_a0d0, &[4, 5, 6]).u32(), 0);
        assert_eq!(calls(&w.e, ACTIVE_EFFECT_CONSTRUCTOR).len(), 3);
    }

    #[test]
    fn damage_formula_gets_its_inputs() {
        let mut w = world(false, 2.5);
        assert_eq!(w.e.call(0x0082_a1b0, &[w.this]).f64(), 2.5);
        assert_eq!(
            calls(&w.e, SUN_DAMAGE_FORMULA),
            vec![vec![
                4.0f32.to_bits(),
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits(),
                1,
                0,
                0
            ]]
        );
        // A cell with flag 2 and the reference at its water height: under water.
        w.e.register(CELL_FLAG_02, |_, _| 1u32.into_ret());
        w.e.call(0x0082_a1b0, &[w.this]);
        assert_eq!(calls(&w.e, SUN_DAMAGE_FORMULA)[1][6], 1);
        w.e.register(REFERENCE_WATER_HEIGHT, |_, _| 9.0f64.into_ret());
        w.e.call(0x0082_a1b0, &[w.this]);
        assert_eq!(calls(&w.e, SUN_DAMAGE_FORMULA)[2][6], 0);
    }

    #[test]
    fn damage_blends_the_two_values() {
        let mut w = world(false, 2.5);
        w.e.register(WORD_10, |_, _| 0x10u32.into_ret());
        w.e.register(WORD_14, |_, _| 0x14u32.into_ret());
        w.e.register(FLOAT_F4, |_, _| 0.25f64.into_ret());
        w.e.register(WATER_VALUE_004ED230, |_, a| {
            if a[0] == 0x10 {
                8.0f64.into_ret()
            } else {
                4.0f64.into_ret()
            }
        });
        w.e.call(0x0082_a1b0, &[w.this]);
        // 8 * 0.25 + 4 * (1 - 0.25) = 5
        assert_eq!(calls(&w.e, SUN_DAMAGE_FORMULA)[0][5], 5.0f32.to_bits());
        assert_eq!(
            calls(&w.e, WATER_VALUE_004ED230),
            vec![
                vec![0x10, 5, 1.0f32.to_bits(), 0],
                vec![0x14, 5, 1.0f32.to_bits(), 0]
            ]
        );
    }

    #[test]
    fn damage_is_zero_without_reference_or_in_a_closed_interior() {
        let mut w = world(false, 2.5);
        w.e.register(REFERENCE_SLOT_DOUBLE, |_, _| 0u32.into_ret());
        assert_eq!(w.e.call(0x0082_a1b0, &[w.this]).f64(), 0.0);
        let mut w = world(false, 2.5);
        w.e.register(REFERENCE_IS_INTERIOR, |_, _| 1u32.into_ret());
        assert_eq!(w.e.call(0x0082_a1b0, &[w.this]).f64(), 0.0);
        w.e.register(CELL_FLAG_80, |_, _| 1u32.into_ret());
        assert_eq!(w.e.call(0x0082_a1b0, &[w.this]).f64(), 2.5);
        assert!(calls(&w.e, SUN_DAMAGE_FORMULA).len() == 1);
    }

    #[test]
    fn is_causing_damage_only_for_the_player_target() {
        let mut w = world(true, 2.5);
        let player: u32 = w.e.global(PLAYER);
        assert!(!w.e.call(0x0082_a150, &[w.this]).bool());
        w.e.mem.set_u32(w.this + 0x24, player + PLAYER_MAGIC_TARGET);
        // The player's magic target answers the reference through slot +4 too.
        let target_vtable = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(target_vtable + 4, REFERENCE_SLOT_DOUBLE);
        w.e.mem.set_u32(player + PLAYER_MAGIC_TARGET, target_vtable);
        assert!(w.e.call(0x0082_a150, &[w.this]).bool());
        w.e.register(SUN_DAMAGE_FORMULA, |_, _| 0.0f64.into_ret());
        assert!(!w.e.call(0x0082_a150, &[w.this]).bool());
    }

    #[test]
    fn update_skips_once_when_asked() {
        let mut w = world(true, 2.5);
        w.e.mem.set_u8(w.this + SKIP_NEXT_DAMAGE, 1);
        w.e.call(0x0082_9bb0, &args![w.this, 0.5f32]);
        assert_eq!(w.e.mem.u8(w.this + SKIP_NEXT_DAMAGE), 0);
        assert!(calls(&w.e, ACTIVE_EFFECT_TARGET).is_empty());
    }

    #[test]
    fn update_damages_the_player_and_counts_down() {
        let mut w = world(true, 2.5);
        w.e.call(0x0082_9bb0, &args![w.this, 0.5f32]);
        // The hit effect is turned on: built, added and listed.
        assert_eq!(
            calls(&w.e, SHADER_HIT_EFFECT_CONSTRUCTOR),
            vec![vec![w.hit, w.reference, 0x8900, (-1.0f32).to_bits()]]
        );
        assert_eq!(
            calls(&w.e, ADD_TEMP_EFFECT),
            vec![vec![PROCESS_LISTS, w.hit]]
        );
        let list = w.e.mem.u32(w.this + HIT_EFFECTS);
        assert_eq!(w.e.mem.u32(list), w.hit);
        assert_eq!(calls(&w.e, HIT_EFFECT_SET_18), vec![vec![w.hit, w.this]]);
        // 2.5 * 0.5 to the player.
        assert_eq!(
            calls(&w.e, PLAYER_338_DOUBLE),
            vec![vec![w.reference, 1.25f32.to_bits(), 0, 0]]
        );
        // The countdown 5.0 is not below 0 nor equal to 1.0: it only loses the seconds.
        assert!(calls(&w.e, ACTOR_TRIGGER_PAIN).is_empty());
        assert_eq!(w.e.global::<f32>(PAIN_TIMER), 4.5);
        // The blended value's object is asked.
        assert_eq!(calls(&w.e, WORD_68).len(), 2);
    }

    #[test]
    fn update_countdown_restarts() {
        let mut w = world(true, 2.5);
        w.e.set_global::<f32>(PAIN_TIMER, -1.0);
        w.e.call(0x0082_9bb0, &args![w.this, 0.5f32]);
        assert_eq!(
            calls(&w.e, ACTOR_TRIGGER_PAIN),
            vec![vec![w.reference, 1, 1]]
        );
        // Restarted from the setting at 011d18d8 (40.0), then minus the seconds.
        assert_eq!(w.e.global::<f32>(PAIN_TIMER), 39.5);

        let mut w = world(true, 2.5);
        w.e.set_global::<f32>(PAIN_TIMER, 1.0);
        w.e.call(0x0082_9bb0, &args![w.this, 0.5f32]);
        assert!(calls(&w.e, ACTOR_TRIGGER_PAIN).is_empty());
        // Restarted from the setting at 011d1944 (30.0).
        assert_eq!(w.e.global::<f32>(PAIN_TIMER), 29.5);
    }

    #[test]
    fn update_without_damage_turns_the_hit_effect_off() {
        let mut w = world(true, 0.0);
        let node = w.e.mem.alloc(8);
        w.e.mem.set_u32(node, 0x9900);
        w.e.mem.set_u32(w.this + HIT_EFFECTS, node);
        w.e.call(0x0082_9bb0, &args![w.this, 0.5f32]);
        assert_eq!(calls(&w.e, HIT_EFFECT_SET_24), vec![vec![0x9900, 1]]);
        assert_eq!(calls(&w.e, HIT_EFFECT_SET_18), vec![vec![0x9900, 0]]);
        assert_eq!(calls(&w.e, LIST_CLEAR), vec![vec![node]]);
        assert_eq!(calls(&w.e, LIST_DELETE), vec![vec![node, 1]]);
        assert_eq!(w.e.mem.u32(w.this + HIT_EFFECTS), 0);
        assert_eq!(
            calls(&w.e, FINISH_MAGIC_SHADER_HIT_EFFECT),
            vec![vec![PROCESS_LISTS, w.reference, 0x8900]]
        );
        assert!(calls(&w.e, PLAYER_338_DOUBLE).is_empty());
        assert_eq!(w.e.global::<f32>(PAIN_TIMER), 5.0);
    }

    #[test]
    fn update_in_a_closed_interior_restarts_the_countdown() {
        let mut w = world(true, 2.5);
        w.e.register(REFERENCE_IS_INTERIOR, |_, _| 1u32.into_ret());
        w.e.call(0x0082_9bb0, &args![w.this, 0.5f32]);
        assert!(calls(&w.e, SUN_DAMAGE_FORMULA).is_empty());
        assert_eq!(w.e.global::<f32>(PAIN_TIMER), 30.0);
        // Not the player: nothing restarts.
        let mut w = world(false, 2.5);
        w.e.register(REFERENCE_IS_INTERIOR, |_, _| 1u32.into_ret());
        w.e.call(0x0082_9bb0, &args![w.this, 0.5f32]);
        assert_eq!(w.e.global::<f32>(PAIN_TIMER), 5.0);
    }

    #[test]
    fn update_for_another_reference_stops_after_the_hit_effect() {
        let mut w = world(false, 2.5);
        w.e.call(0x0082_9bb0, &args![w.this, 0.5f32]);
        assert_eq!(calls(&w.e, ADD_TEMP_EFFECT).len(), 1);
        assert!(calls(&w.e, PLAYER_338_DOUBLE).is_empty());
        assert_eq!(calls(&w.e, WORD_68).len(), 1);
    }

    #[test]
    fn hit_effect_on_is_skipped_or_deleted() {
        // A list that is not empty: nothing is built.
        let mut w = world(false, 2.5);
        let node = w.e.mem.alloc(8);
        w.e.mem.set_u32(w.this + HIT_EFFECTS, node);
        w.e.call(0x0082_9e00, &[w.this, 1]);
        assert!(calls(&w.e, ALLOCATE_AA13E0).is_empty());
        // An empty list is dropped and a new effect that cannot be added is deleted.
        w.e.register(LIST_IS_EMPTY, |_, _| 1u32.into_ret());
        w.e.register(HIT_C4_DOUBLE, |_, _| 0u32.into_ret());
        w.e.call(0x0082_9e00, &[w.this, 1]);
        assert_eq!(calls(&w.e, LIST_DELETE), vec![vec![node, 1]]);
        assert_eq!(calls(&w.e, HIT_DELETE_DOUBLE), vec![vec![w.hit, 1]]);
        assert_eq!(w.e.mem.u32(w.this + HIT_EFFECTS), 0);
        // Another spell type does nothing.
        w.e.mem.set_u32(w.this + 0x2c, 3);
        w.e.call(0x0082_9e00, &[w.this, 1]);
        assert_eq!(calls(&w.e, ALLOCATE_AA13E0).len(), 1);
    }

    #[test]
    fn save_size_save_and_load_by_version() {
        let mut w = world(false, 0.0);
        w.e.register(SAVE_VERSION, |_, _| 0x71u32.into_ret());
        w.e.register(ACTIVE_EFFECT_GET_SAVE_SIZE, |_, _| 0x30u32.into_ret());
        assert_eq!(w.e.call(0x0082_a390, &[w.this, 9]).u32(), 0x34);
        w.e.call(0x0082_a3e0, &[w.this, 9]);
        w.e.call(0x0082_a420, &[w.this, 9]);
        assert_eq!(
            calls(&w.e, SAVE_WRITE),
            vec![vec![0x3000, w.this + GLOW_EFFECT, 4]]
        );
        assert_eq!(
            calls(&w.e, SAVE_READ),
            vec![vec![0x3000, w.this + GLOW_EFFECT, 4]]
        );
        assert_eq!(calls(&w.e, ACTIVE_EFFECT_SAVE_GAME), vec![vec![w.this, 9]]);
        assert_eq!(calls(&w.e, ACTIVE_EFFECT_LOAD_GAME), vec![vec![w.this, 9]]);
        w.e.register(SAVE_VERSION, |_, _| 0x70u32.into_ret());
        assert_eq!(w.e.call(0x0082_a390, &[w.this, 9]).u32(), 0x30);
        w.e.call(0x0082_a3e0, &[w.this, 9]);
        w.e.call(0x0082_a420, &[w.this, 9]);
        assert_eq!(calls(&w.e, SAVE_WRITE).len(), 1);
        assert_eq!(calls(&w.e, SAVE_READ).len(), 1);
    }
}
