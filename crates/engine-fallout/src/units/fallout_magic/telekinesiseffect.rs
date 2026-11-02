//! `fallout/magic/telekinesiseffect.cpp` (Xbox PDB source unit), subsystem `fallout/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `0082bbe0` (named `ValueModifierEffect::CopyData` in the engine map): the base body the
/// wrapper forwards to; `thiscall`, `this` plus one stack word (`RET 4`).
const COPY_DATA_TARGET_0082BBE0: u32 = 0x0082_bbe0;

// Translated from 0082a650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TelekinesisEffect::CopyData` (Xbox PDB
/// `?CopyData@TelekinesisEffect@@MAAXPAVActiveEffect@@@Z`, void, vtable word 010792f0 and
/// others): forwards `this` and the `ActiveEffect` pointer to the base body `0082bbe0`
/// (`ValueModifierEffect::CopyData`). Returns nothing.
pub fn telekinesis_effect_copy_data(e: &mut Engine, this: Ptr, target: Ptr) {
    e.call(COPY_DATA_TARGET_0082BBE0, &args![this, target]);
}

/// `004ae750` (`thiscall`; engine map `bhkCharacterProxy::operatorP`, a folded body):
/// an object of the effect, or null.
const OBJECT_004AE750: u32 = 0x004a_e750;
/// `009611e0` (`thiscall`): the word at +0x18.
const WORD_18_009611E0: u32 = 0x0096_11e0;
/// `004b5a80` (`globalfunc.cpp`, `cdecl`, one word).
const GLOBAL_004B5A80: u32 = 0x004b_5a80;

// Translated from 0082aeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): passes [`fn_0082aee0`]'s result
/// to `004b5a80` and returns what that returns.
pub fn fn_0082aeb0(e: &mut Engine, this: Ptr) -> u32 {
    let object = fn_0082aee0(e, this);
    e.call(GLOBAL_004B5A80, &args![object]).u32()
}

// Translated from 0082aee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, plain `RET`): the word at +0x18 (`009611e0`) of
/// the object `004ae750` returns for `this`, or null when there is none.
pub fn fn_0082aee0(e: &mut Engine, this: Ptr) -> u32 {
    let object = e.call(OBJECT_004AE750, &args![this]).u32();
    if object == 0 {
        return 0;
    }
    e.call(WORD_18_009611E0, &args![object]).u32()
}

layout! {
    /// `TelekinesisEffect` (Xbox PDB), 0x60 bytes: a `ValueModifierEffect`
    /// and the spring that holds the grabbed reference.
    pub struct TelekinesisEffect: 0x60 {
        /// `spSpring` (`NiPointer<bhkMouseSpringAction>`).
        0x4C spSpring: u32,
        /// `fHoldDistance`.
        0x50 fHoldDistance: u32,
        /// `fMoveVelocity`.
        0x54 fMoveVelocity: u32,
        /// `pRef` (`TESObjectREFR *`).
        0x58 pRef: u32,
        /// `bThrow`.
        0x5C bThrow: u8,
        /// `bFirstUpdate`.
        0x5D bFirstUpdate: u8,
    }
}

/// The player (`PlayerCharacter *`).
const PLAYER: u32 = 0x011d_ea3c;
/// The save/load singleton (pointer global).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// A global object whose controls `Update` reads (pointer global,
/// `011dea0c`), its `00877720` (thiscall, no stack word) and the test
/// `00a24660(control, word)` on what that returns.
const CONTROLS_011DEA0C: u32 = 0x011d_ea0c;
const CONTROL_00877720: u32 = 0x0087_7720;
const CONTROL_DOWN_00A24660: u32 = 0x00a2_4660;
/// `ValueModifierEffect`'s constructor (`0082ba60`), `Start` (`0082c000`),
/// `Update` (`0082c660`, seconds), `Finish` (`0082c810`), `GetSaveSize`
/// (`00802d90`), `SaveGame` (`00802db0`), `LoadGame` (`00802dd0`),
/// `FinishInitLoadGame` (`0082cd10`) and its destructor body (`0080d930`).
const VALUE_MODIFIER_EFFECT_CONSTRUCT: u32 = 0x0082_ba60;
const VALUE_MODIFIER_EFFECT_START: u32 = 0x0082_c000;
const VALUE_MODIFIER_EFFECT_UPDATE: u32 = 0x0082_c660;
const VALUE_MODIFIER_EFFECT_FINISH: u32 = 0x0082_c810;
const VALUE_MODIFIER_EFFECT_GET_SAVE_SIZE: u32 = 0x0080_2d90;
const VALUE_MODIFIER_EFFECT_SAVE_GAME: u32 = 0x0080_2db0;
const VALUE_MODIFIER_EFFECT_LOAD_GAME: u32 = 0x0080_2dd0;
const VALUE_MODIFIER_EFFECT_FINISH_INIT_LOAD_GAME: u32 = 0x0082_cd10;
const VALUE_MODIFIER_EFFECT_DESTRUCT: u32 = 0x0080_d930;
/// `TelekinesisEffect`'s vtable (`0107c48c`).
const TELEKINESIS_EFFECT_VTABLE: u32 = 0x0107_c48c;
/// The spring pointer's constructor (`00633c90`, the pointer, `RET 4`),
/// assignment (`0066b0d0`, `RET 4`) and destructor (`0045cec0`).
const POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const POINTER_ASSIGN: u32 = 0x0066_b0d0;
const POINTER_DESTRUCT: u32 = 0x0045_cec0;
/// `operator new` (`00401000`) and free (`00401030`), cdecl.
const OPERATOR_NEW: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// `ActiveEffect`'s virtual slot +0x44 (`CopyData`, Xbox PDB order).
const SLOT_COPY_DATA: u32 = 0x44;
/// `ActiveEffect::Dispel` (engine map), thiscall (word), `RET 4`.
const ACTIVE_EFFECT_DISPEL: u32 = 0x0080_4210;
/// The effect's caster (`0045cd60`, folded code: the word at +0x28).
const EFFECT_CASTER: u32 = 0x0045_cd60;
/// The effect's effect setting (`007df1d0`) and the setting's word
/// `00407820` (a hit shader, as `Finish` hands it to `ProcessLists`).
const EFFECT_SETTING: u32 = 0x007d_f1d0;
const SETTING_MEMBER_00407820: u32 = 0x0040_7820;
/// `Interface::GetTelekinesisREFR` (engine map), no arguments.
const GET_TELEKINESIS_REFR: u32 = 0x0070_3390;
/// The process lists (`011e0e80`), `ProcessLists::AddTempEffect`
/// (`00973fd0`, effect, `RET 4`) and `ProcessLists::FinishMagicShaderHitEffect`
/// (`00974a50`, reference, shader, `RET 8`) (engine map).
const PROCESS_LISTS: u32 = 0x011e_0e80;
const PROCESS_LISTS_ADD_TEMP_EFFECT: u32 = 0x0097_3fd0;
const PROCESS_LISTS_FINISH_SHADER_HIT_EFFECT: u32 = 0x0097_4a50;
/// The save/load singleton's byte at +0x80 (`008df040`), its numeric-id
/// save (`00857a10`) and load (`00857aa0`) (address, size; engine map
/// `TESSaveLoadGame::SaveNumericID` / `LoadNumericID`), and its block
/// write (`008579b0`) and read (`008579e0`), thiscall, `RET 8`.
const SAVE_LOAD_BYTE_AT_80: u32 = 0x008d_f040;
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
const SAVE_LOAD_WRITE: u32 = 0x0085_79b0;
const SAVE_LOAD_READ: u32 = 0x0085_79e0;
/// A form's id (`0084e3a0`, the word at +0xc) and the form with an id
/// (`004839c0`, cdecl).
const FORM_ID: u32 = 0x0084_e3a0;
const LOOK_UP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (CRT), cdecl, and the type descriptors of `TESForm`
/// and `TESObjectREFR`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// Setting addresses (`00403e20` gives the address of the value).
const SETTING_VALUE: u32 = 0x0040_3e20;
const SETTING_011D17E8: u32 = 0x011d_17e8;
const SETTING_011D1CF4: u32 = 0x011d_1cf4;
const SETTING_011D1A3C: u32 = 0x011d_1a3c;
const SETTING_011D1CAC: u32 = 0x011d_1cac;
const SETTING_011D1D24: u32 = 0x011d_1d24;
const SETTING_011D15E0: u32 = 0x011d_15e0;
const SETTING_011D18FC: u32 = 0x011d_18fc;
/// Constants: a `double` (`0101ffa0`), the `float`s at `01022958` (the hold
/// distance when `0047c850` is false) and `01012054`.
const CONSTANT_0101FFA0: u32 = 0x0101_ffa0;
const CONSTANT_01022958: u32 = 0x0102_2958;
const CONSTANT_01012054: u32 = 0x0101_2054;

/// The address of a setting's value (`00403e20`).
fn setting_address(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_VALUE, &args![setting]).u32()
}

/// The `float` value of a setting, as `FLD float` loads it.
fn setting_value(e: &mut Engine, setting: u32) -> f64 {
    let value = setting_address(e, setting);
    x87_load32(e.mem.u32(value))
}

/// `ActiveEffect::Dispel(0)`.
fn dispel(e: &mut Engine, this: Ptr<TelekinesisEffect>) {
    e.call(ACTIVE_EFFECT_DISPEL, &args![this, 0u32]);
}

/// A `TelekinesisEffect` member read as a `float` by `FLD float`.
fn load_field(
    e: &Engine,
    this: Ptr<TelekinesisEffect>,
    field: Field<TelekinesisEffect, u32>,
) -> f64 {
    x87_load32(e.get(this, field))
}

// Translated from 0082a460 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082a460 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the spring pointer) is not translated.
/// The `TelekinesisEffect` constructor (unnamed in the engine map;
/// `thiscall`, `RET 0xc`): the `ValueModifierEffect` constructor with the
/// caster, the spell and the effect item, the class's vtable (`0107c48c`),
/// the spring pointer (+0x4c) constructed and set to null, hold distance
/// and move velocity 0.0, no reference, `bThrow` and `bFirstUpdate` false.
/// Returns `this`.
pub fn fn_0082a460(
    e: &mut Engine,
    this: Ptr<TelekinesisEffect>,
    caster: u32,
    spell: u32,
    effect: u32,
) -> Ptr<TelekinesisEffect> {
    e.call(
        VALUE_MODIFIER_EFFECT_CONSTRUCT,
        &args![this, caster, spell, effect],
    );
    e.mem.set_u32(this.addr(), TELEKINESIS_EFFECT_VTABLE);
    e.call(POINTER_CONSTRUCT, &args![this.addr() + 0x4c, 0u32]);
    e.call(POINTER_ASSIGN, &args![this.addr() + 0x4c, 0u32]);
    e.set(this, TelekinesisEffect::fHoldDistance, 0.0f32.to_bits());
    e.set(this, TelekinesisEffect::fMoveVelocity, 0.0f32.to_bits());
    e.set(this, TelekinesisEffect::pRef, 0);
    e.set(this, TelekinesisEffect::bThrow, 0);
    e.set(this, TelekinesisEffect::bFirstUpdate, 0);
    this
}

// Translated from 0082a510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TelekinesisEffect`'s scalar deleting destructor (engine map;
/// `thiscall`, `RET 4`): the destructor body (`0082a540`), then frees
/// `this` (`00401030`) when bit 0 of `flags` is set. Returns `this`.
pub fn telekinesis_effect_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<TelekinesisEffect>,
    flags: u32,
) -> Ptr<TelekinesisEffect> {
    fn_0082a540(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0082a540 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082a540 - the C++ exception frame (FS:[0] chain, unwind state 0 around the spring pointer) is not translated.
/// The `TelekinesisEffect` destructor body (unnamed in the engine map): the
/// spring pointer (+0x4c, `0045cec0`), then the `ValueModifierEffect`
/// destructor body (`0080d930`).
pub fn fn_0082a540(e: &mut Engine, this: Ptr<TelekinesisEffect>) {
    e.call(POINTER_DESTRUCT, &args![this.addr() + 0x4c]);
    e.call(VALUE_MODIFIER_EFFECT_DESTRUCT, &args![this]);
}

// Translated from 0082a5a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082a5a0 - the C++ exception frame (FS:[0] chain, unwind state 0 around the new effect) is not translated.
/// `TelekinesisEffect::Copy` (engine map; `thiscall`, plain `RET`): a new
/// effect (0x60 bytes, `0082a460` with this effect's caster (+0x28), spell
/// (+8) and effect item (+0xc); 0 when the allocation failed) that this
/// effect's `CopyData` (virtual slot +0x44) fills. Returns it.
pub fn telekinesis_effect_copy(e: &mut Engine, this: Ptr<TelekinesisEffect>) -> u32 {
    let memory = e.call(OPERATOR_NEW, &args![0x60u32]).u32();
    let copy = if memory == 0 {
        0
    } else {
        let caster = e.mem.u32(this.addr() + 0x28);
        let spell = e.mem.u32(this.addr() + 8);
        let effect = e.mem.u32(this.addr() + 0xc);
        fn_0082a460(e, Ptr::new(memory), caster, spell, effect).addr()
    };
    e.vcall(this.addr(), SLOT_COPY_DATA, &args![copy]);
    copy
}

// Translated from 0082a670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TelekinesisEffect::Start` (engine map; `thiscall`, plain `RET`):
/// `ValueModifierEffect::Start`, the reference from
/// `Interface::GetTelekinesisREFR`, the spring (`CreateSpring`), then
/// `bFirstUpdate` cleared.
pub fn telekinesis_effect_start(e: &mut Engine, this: Ptr<TelekinesisEffect>) {
    e.call(VALUE_MODIFIER_EFFECT_START, &args![this]);
    let reference = e.call(GET_TELEKINESIS_REFR, &args![]).u32();
    e.set(this, TelekinesisEffect::pRef, reference);
    telekinesis_effect_create_spring(e, this);
    e.set(this, TelekinesisEffect::bFirstUpdate, 0);
}

/// `CreateSpring`'s check of the reference's object: its model's
/// `0043b4d0` (through `0043b4f0` with a stack word) is 4 or else 5.
fn spring_object_kind_ok(e: &mut Engine, object: u32, frame: u32) -> bool {
    let first = e
        .call(0x0043_b4f0, &args![object, frame + 0x80 - 0x58])
        .u32();
    if e.call(0x0043_b4d0, &args![first]).u32() == 4 {
        return true;
    }
    let second = e
        .call(0x0043_b4f0, &args![object, frame + 0x80 - 0x5c])
        .u32();
    e.call(0x0043_b4d0, &args![second]).u32() == 5
}

// Translated from 0082a6a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082a6a0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 around the two allocations) is not translated.
/// `TelekinesisEffect::CreateSpring` (engine map; `thiscall`, plain `RET`):
/// dispels the effect (`Dispel(0)`) without a reference or a caster, when
/// the reference's form (`007af430`) is of type 0x1b, or when the
/// reference's node (virtual slot +0x1d0) has no collision object
/// (`FindFirstCollisionObject`, then `006fa820`) whose kind
/// (`0043b4f0`/`0043b4d0`) is 4 or 5. Otherwise: the telekinesis listeners
/// are removed from the object (`00807ac0`), a 0x1c-byte spring action is
/// made (`00807790` with the reference, the object and the word
/// `0043b4f0` gives; the result is not kept here), the node's fade-node
/// LOD type is set to 6 (`00653270`, `00b4dec0`), the player's grab (when
/// `0082aa90` gives one) is ended (`00961280`), the hold distance becomes
/// the float at `01022958` unless `0047c850` is true for the save/load
/// singleton, and the player makes the mouse spring
/// (`PlayerCharacter::CreateMouseSpring` with the reference, 3 and the
/// hold distance). The eye vector (`Actor::GetEyeVector`) is taken; unless
/// `0047c850` says so, the hold distance becomes the dot product
/// (`NiPoint3::Dot`) of the eye direction and the offset from the eye to
/// the grabbed object's position (`0082aa50`, `00439ef0`), stored into the
/// player (`0082aab0`) and the direction (`00439180`). Last, when the
/// effect setting has a hit shader (`00407820`), a 0x6c-byte hit effect is
/// made (`0081f580` with the reference, the shader and the float at
/// `01012054`): added to the process lists when its virtual slot +0xc4
/// answers true, deleted (virtual slot +0 with 1) otherwise.
pub fn telekinesis_effect_create_spring(e: &mut Engine, this: Ptr<TelekinesisEffect>) {
    if e.get(this, TelekinesisEffect::pRef) == 0 || e.call(EFFECT_CASTER, &args![this]).u32() == 0 {
        dispel(e, this);
        return;
    }
    let reference = e.get(this, TelekinesisEffect::pRef);
    let form = e.call(0x007a_f430, &args![reference]).u32();
    if e.call(0x0040_1170, &args![form]).u32() == 0x1b {
        dispel(e, this);
        return;
    }
    let reference = e.get(this, TelekinesisEffect::pRef);
    let node = e.vcall(reference, 0x1d0, &args![]).u32();
    let collision = if node != 0 {
        e.call(0x004b_5260, &args![node]).u32()
    } else {
        0
    };
    let object = if collision != 0 {
        e.call(0x006f_a820, &args![collision]).u32()
    } else {
        0
    };
    if object != 0 {
        e.call(0x0080_7ac0, &args![object]);
    }
    e.with_stack(0x80, |e, frame| {
        let frame = frame.addr();
        let local = |offset: u32| frame + 0x80 - offset;
        if object == 0 || !spring_object_kind_ok(e, object, frame) {
            dispel(e, this);
            return;
        }
        e.call(OBJECT_004AE750, &args![object]);
        let memory = e.call(OPERATOR_NEW, &args![0x1cu32]).u32();
        if memory != 0 {
            let kind = e.call(0x0043_b4f0, &args![object, local(0x68)]).u32();
            let kind = e.mem.u32(kind);
            let reference = e.get(this, TelekinesisEffect::pRef);
            e.call(0x0080_7790, &args![memory, reference, object, kind]);
        }
        let fade = e.call(0x0065_3270, &args![0x011f_9140u32, node]).u32();
        if fade != 0 {
            e.call(0x00b4_dec0, &args![fade, 6u32]);
        }
        let player = e.global::<u32>(PLAYER);
        if fn_0082aa90(e, Ptr::new(player)) != 0 {
            let player = e.global::<u32>(PLAYER);
            e.call(0x0096_1280, &args![player]);
        }
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        if !e.call(0x0047_c850, &args![game]).bool() {
            let distance = x87_quiet_bits(e.mem.u32(CONSTANT_01022958));
            e.set(this, TelekinesisEffect::fHoldDistance, distance);
        }
        let distance = x87_quiet_bits(e.get(this, TelekinesisEffect::fHoldDistance));
        let reference = e.get(this, TelekinesisEffect::pRef);
        let player = e.global::<u32>(PLAYER);
        e.call(0x0095_f930, &args![player, reference, 3u32, distance]);
        e.call(0x0068_15c0, &args![local(0x28)]);
        e.call(0x0068_15c0, &args![local(0x34)]);
        let player = e.global::<u32>(PLAYER);
        e.call(0x008a_ff80, &args![player, local(0x28), local(0x34), 1u32]);
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        if !e.call(0x0047_c850, &args![game]).bool() {
            e.call(0x0068_15c0, &args![local(0x50)]);
            let player = e.global::<u32>(PLAYER);
            let grab = fn_0082aa90(e, Ptr::new(player));
            fn_0082aa50(e, Ptr::new(grab), local(0x50));
            e.call(0x0043_9ef0, &args![local(0x50), local(0x44), local(0x28)]);
            let dot = e.call(0x004b_6190, &args![local(0x34), local(0x44)]).f64();
            e.set(this, TelekinesisEffect::fHoldDistance, x87_store32(dot));
            let distance = x87_quiet_bits(e.get(this, TelekinesisEffect::fHoldDistance));
            let player = e.global::<u32>(PLAYER);
            fn_0082aab0(e, Ptr::new(player), f32::from_bits(distance));
            let distance = x87_quiet_bits(e.get(this, TelekinesisEffect::fHoldDistance));
            e.call(0x0043_9180, &args![local(0x34), distance]);
        }
        let setting = e.call(EFFECT_SETTING, &args![this]).u32();
        if e.call(SETTING_MEMBER_00407820, &args![setting]).u32() == 0 {
            return;
        }
        let memory = e.call(0x00aa_13e0, &args![0x6cu32]).u32();
        let hit_effect = if memory != 0 {
            let scale = x87_quiet_bits(e.mem.u32(CONSTANT_01012054));
            let setting = e.call(EFFECT_SETTING, &args![this]).u32();
            let shader = e.call(SETTING_MEMBER_00407820, &args![setting]).u32();
            let reference = e.get(this, TelekinesisEffect::pRef);
            e.call(0x0081_f580, &args![memory, reference, shader, scale])
                .u32()
        } else {
            0
        };
        if e.vcall(hit_effect, 0xc4, &args![]).bool() {
            e.call(
                PROCESS_LISTS_ADD_TEMP_EFFECT,
                &args![PROCESS_LISTS, hit_effect],
            );
        } else if hit_effect != 0 {
            e.vcall(hit_effect, 0x0, &args![1u32]);
        }
    });
}

// Translated from 0082aa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): when `004ae750` gives
/// an object for `this`, `00458620(position, object + 0x30)` (cdecl).
pub fn fn_0082aa50(e: &mut Engine, this: Ptr, position: u32) {
    let object = e.call(OBJECT_004AE750, &args![this]).u32();
    if object != 0 {
        e.call(0x0045_8620, &args![position, object + 0x30]);
    }
}

// Translated from 0082aa90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, folded code): the word at +0x634
/// (`00559450` on `this + 0x634`; on the player, what `Update` grabs).
pub fn fn_0082aa90(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x0055_9450, &args![this.addr() + 0x634]).u32()
}

// Translated from 0082aab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): stores the float at
/// +0x644 (`FLD`/`FSTP`).
pub fn fn_0082aab0(e: &mut Engine, this: Ptr, value: f32) {
    e.mem
        .set_u32(this.addr() + 0x644, x87_quiet_bits(value.to_bits()));
}

/// `0089f4e0` on the player.
fn player_word_0089f4e0(e: &mut Engine) -> u32 {
    let player = e.global::<u32>(PLAYER);
    e.call(0x0089_f4e0, &args![player]).u32()
}

/// `008d6f30` on the player's `0089f4e0` word.
fn player_word_008d6f30(e: &mut Engine) -> u32 {
    let word = player_word_0089f4e0(e);
    e.call(0x008d_6f30, &args![word]).u32()
}

/// Whether control `control` is down (`00877720` on the object at `011dea0c`,
/// then `00a24660(control, 0)` on what it returns).
fn control_down(e: &mut Engine, control: u32) -> bool {
    let controls = e.global::<u32>(CONTROLS_011DEA0C);
    // 0082ac06 and the other sites: `00877720` takes no stack word (plain RET); the
    // pushed control and 0 are the two words of `00a24660` (RET 8).
    let state = e.call(CONTROL_00877720, &args![controls]).u32();
    e.call(CONTROL_DOWN_00A24660, &args![state, control, 0u32])
        .u32()
        != 0
}

// Translated from 0082aad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TelekinesisEffect::Update` (engine map; `thiscall`, `RET 4`):
/// `ValueModifierEffect::Update(seconds)`; dispels the effect when the
/// player's grab (`0082aa90`) has nothing to hold (`0082aeb0`), or unless
/// the player's `0089f4e0` word is the effect's reference, the player's
/// `0082af20` is 3 and the caster's and the player's `004543c0` words agree
/// (each 0 without the words leading to it). Then the move velocity
/// follows the controls of the object at `011dea0c`: with controls 6 and 4
/// both down the effect throws when the velocity is above `setting 011d1a3c
/// * 0.1 + setting 011d1cf4` and is dispelled; with 4 it grows by `setting
/// 011d1a3c * seconds` from at least `setting 011d1cf4` up to at most
/// `setting 011d1cac`; with 6 it shrinks likewise between the negated
/// settings; with neither it is 0. The hold distance becomes the player's
/// (`0082af40`) plus velocity times seconds, at least `setting 011d1d24`
/// and at most `setting 011d18fc * (magnitude * setting 011d15e0)`; it is
/// stored into the player (`0082aab0`), and unless this is the first
/// update `00960520(seconds)` runs on the player. `bFirstUpdate` is then
/// cleared.
pub fn telekinesis_effect_update(e: &mut Engine, this: Ptr<TelekinesisEffect>, seconds: f32) {
    let seconds_bits = x87_quiet_bits(seconds.to_bits());
    e.call(VALUE_MODIFIER_EFFECT_UPDATE, &args![this, seconds_bits]);
    let player = e.global::<u32>(PLAYER);
    let grab = fn_0082aa90(e, Ptr::new(player));
    if fn_0082aeb0(e, Ptr::new(grab)) == 0 {
        dispel(e, this);
        return;
    }
    let caster_word =
        if e.call(EFFECT_CASTER, &args![this]).u32() != 0 && player_word_008d6f30(e) != 0 {
            let caster = e.call(EFFECT_CASTER, &args![this]).u32();
            let actor = e.vcall(caster, 0x2c, &args![]).u32();
            let process = e.call(0x008d_6f30, &args![actor]).u32();
            e.call(0x0045_43c0, &args![process]).u32()
        } else {
            0
        };
    let player_word = if player_word_0089f4e0(e) != 0 && player_word_008d6f30(e) != 0 {
        let process = player_word_008d6f30(e);
        e.call(0x0045_43c0, &args![process]).u32()
    } else {
        0
    };
    let player = e.global::<u32>(PLAYER);
    if grab == 0
        || e.get(this, TelekinesisEffect::pRef) != player_word_0089f4e0(e)
        || fn_0082af20(e, Ptr::new(player)) != 3
        || caster_word != player_word
    {
        dispel(e, this);
        return;
    }
    let seconds_m = x87_m32(seconds.to_bits());
    if control_down(e, 6) && control_down(e, 4) {
        let base = setting_address(e, SETTING_011D1CF4);
        let rate = setting_value(e, SETTING_011D1A3C);
        let tenth: f64 = e.global(CONSTANT_0101FFA0);
        let limit = x87_add(x87_mul(rate, tenth), x87_m32(e.mem.u32(base)));
        // `FCOMPP; TEST AH,0x41; JNZ`: throw when the velocity is above.
        if load_field(e, this, TelekinesisEffect::fMoveVelocity) > limit {
            e.set(this, TelekinesisEffect::bThrow, 1);
        }
        dispel(e, this);
    } else if control_down(e, 4) {
        if setting_value(e, SETTING_011D1CF4)
            > load_field(e, this, TelekinesisEffect::fMoveVelocity)
        {
            let minimum = setting_address(e, SETTING_011D1CF4);
            let minimum = x87_quiet_bits(e.mem.u32(minimum));
            e.set(this, TelekinesisEffect::fMoveVelocity, minimum);
        }
        let rate = setting_value(e, SETTING_011D1A3C);
        let velocity = x87_add(
            x87_mul(rate, seconds_m),
            x87_m32(e.get(this, TelekinesisEffect::fMoveVelocity)),
        );
        e.set(
            this,
            TelekinesisEffect::fMoveVelocity,
            x87_store32(velocity),
        );
        // `TEST AH,5; JP`: clamped when the maximum is below.
        if setting_value(e, SETTING_011D1CAC)
            < load_field(e, this, TelekinesisEffect::fMoveVelocity)
        {
            let maximum = setting_address(e, SETTING_011D1CAC);
            let maximum = x87_quiet_bits(e.mem.u32(maximum));
            e.set(this, TelekinesisEffect::fMoveVelocity, maximum);
        }
    } else if control_down(e, 6) {
        let minimum = -setting_value(e, SETTING_011D1CF4);
        if load_field(e, this, TelekinesisEffect::fMoveVelocity) > minimum {
            let value = -setting_value(e, SETTING_011D1CF4);
            e.set(this, TelekinesisEffect::fMoveVelocity, x87_store32(value));
        }
        let rate = setting_value(e, SETTING_011D1A3C);
        let step = x87_mul(rate, seconds_m);
        let velocity = x87_sub(x87_m32(e.get(this, TelekinesisEffect::fMoveVelocity)), step);
        e.set(
            this,
            TelekinesisEffect::fMoveVelocity,
            x87_store32(velocity),
        );
        let maximum = -setting_value(e, SETTING_011D1CAC);
        if load_field(e, this, TelekinesisEffect::fMoveVelocity) < maximum {
            let value = -setting_value(e, SETTING_011D1CAC);
            e.set(this, TelekinesisEffect::fMoveVelocity, x87_store32(value));
        }
    } else {
        e.set(this, TelekinesisEffect::fMoveVelocity, 0.0f32.to_bits());
    }
    let player = e.global::<u32>(PLAYER);
    let held = fn_0082af40(e, Ptr::new(player));
    e.set(
        this,
        TelekinesisEffect::fHoldDistance,
        x87_store32(f64::from(held)),
    );
    let distance = x87_add(
        x87_mul(
            load_field(e, this, TelekinesisEffect::fMoveVelocity),
            seconds_m,
        ),
        x87_m32(e.get(this, TelekinesisEffect::fHoldDistance)),
    );
    e.set(
        this,
        TelekinesisEffect::fHoldDistance,
        x87_store32(distance),
    );
    if setting_value(e, SETTING_011D1D24) > load_field(e, this, TelekinesisEffect::fHoldDistance) {
        let minimum = setting_address(e, SETTING_011D1D24);
        let minimum = x87_quiet_bits(e.mem.u32(minimum));
        e.set(this, TelekinesisEffect::fHoldDistance, minimum);
    }
    let reach = |e: &mut Engine| {
        let mult = setting_address(e, SETTING_011D15E0);
        let magnitude = e.call(0x0049_5460, &args![this]).f64();
        let scaled = x87_mul(magnitude, x87_m32(e.mem.u32(mult)));
        let range = setting_value(e, SETTING_011D18FC);
        x87_mul(range, scaled)
    };
    let maximum = reach(e);
    if load_field(e, this, TelekinesisEffect::fHoldDistance) > maximum {
        let maximum = reach(e);
        e.set(this, TelekinesisEffect::fHoldDistance, x87_store32(maximum));
    }
    let distance = x87_quiet_bits(e.get(this, TelekinesisEffect::fHoldDistance));
    let player = e.global::<u32>(PLAYER);
    fn_0082aab0(e, Ptr::new(player), f32::from_bits(distance));
    if e.get(this, TelekinesisEffect::bFirstUpdate) == 0 {
        let player = e.global::<u32>(PLAYER);
        e.call(0x0096_0520, &args![player, seconds_bits]);
    }
    e.set(this, TelekinesisEffect::bFirstUpdate, 0);
}

// Translated from 0082af20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, folded code): the word at
/// +0x63c.
pub fn fn_0082af20(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x63c)
}

// Translated from 0082af40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, a float in `ST0`): the float at
/// +0x644 (`FLD`).
pub fn fn_0082af40(e: &mut Engine, this: Ptr) -> f32 {
    f32::from_bits(x87_quiet_bits(e.mem.u32(this.addr() + 0x644)))
}

// Translated from 0082af60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TelekinesisEffect::Finish` (engine map; `thiscall`, plain `RET`): lets
/// go of the reference (`0082afa0`), ends the hit shader on it
/// (`ProcessLists::FinishMagicShaderHitEffect` with the reference and the
/// effect setting's shader, `00407820`), then `ValueModifierEffect::Finish`.
pub fn telekinesis_effect_finish(e: &mut Engine, this: Ptr<TelekinesisEffect>) {
    fn_0082afa0(e, this);
    let setting = e.call(EFFECT_SETTING, &args![this]).u32();
    let shader = e.call(SETTING_MEMBER_00407820, &args![setting]).u32();
    let reference = e.get(this, TelekinesisEffect::pRef);
    e.call(
        PROCESS_LISTS_FINISH_SHADER_HIT_EFFECT,
        &args![PROCESS_LISTS, reference, shader],
    );
    e.call(VALUE_MODIFIER_EFFECT_FINISH, &args![this]);
}

// Translated from 0082afa0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082afa0 - the /GS security cookie check (00ec408c) is not translated.
/// Unnamed in the engine map (`thiscall`, plain `RET`): when the player
/// holds something (`0082aa90`), the held object (`0082aee0`) is let go:
/// the player's `00961280` runs, then, with an object and its reference
/// (`004b5a80`): when throwing (`bThrow`), the impulse is the player's
/// rotation (`00430830`, a matrix from its three angles through
/// `00a59660`, column 1 taken by `00439f50`) scaled (`0045bb20`) by
/// `setting 011d17e8 + fMoveVelocity`, converted (`004a3e00`), multiplied
/// (`00627920`) by a vector made (`005dbf20`) from the reference's
/// `004b5400`, and applied to the reference's body
/// (`hkpRigidBody::applyLinearImpulse`); otherwise `005615d0(011f426c)` runs
/// on the reference. The vectors and the matrix are on the game's stack.
pub fn fn_0082afa0(e: &mut Engine, this: Ptr<TelekinesisEffect>) {
    let player = e.global::<u32>(PLAYER);
    if fn_0082aa90(e, Ptr::new(player)) == 0 {
        return;
    }
    let player = e.global::<u32>(PLAYER);
    let grab = fn_0082aa90(e, Ptr::new(player));
    let object = fn_0082aee0(e, Ptr::new(grab));
    let player = e.global::<u32>(PLAYER);
    e.call(0x0096_1280, &args![player]);
    if object == 0 {
        return;
    }
    let reference = e.call(GLOBAL_004B5A80, &args![object]).u32();
    if e.get(this, TelekinesisEffect::bThrow) == 0 {
        e.call(0x0056_15d0, &args![reference, 0x011f_426cu32]);
        return;
    }
    e.with_stack(0x80, |e, frame| {
        let frame = frame.addr();
        let local = |offset: u32| frame + 0x80 - offset;
        e.call(0x0068_15c0, &args![local(0x1c)]);
        e.call(0x0068_15c0, &args![local(0x40)]);
        let player = e.global::<u32>(PLAYER);
        let angles = e.call(0x0043_0830, &args![player]).u32();
        for i in 0..3 {
            let word = e.mem.u32(angles + 4 * i);
            e.mem.set_u32(local(0x4c) + 4 * i, word);
        }
        let x = x87_quiet_bits(e.mem.u32(local(0x4c)));
        let y = x87_quiet_bits(e.mem.u32(local(0x48)));
        let z = x87_quiet_bits(e.mem.u32(local(0x44)));
        e.call(0x00a5_9660, &args![local(0x40), z, x, y]);
        e.call(0x0043_9f50, &args![local(0x40), 1u32, local(0x1c)]);
        let base = setting_value(e, SETTING_011D17E8);
        let speed = x87_store32(x87_add(
            base,
            x87_m32(e.get(this, TelekinesisEffect::fMoveVelocity)),
        ));
        e.mem.set_u32(local(0x50), speed);
        let speed = x87_quiet_bits(speed);
        e.call(0x0045_bb20, &args![local(0x1c), local(0x5c), speed]);
        e.call(0x0068_15c0, &args![local(0x70)]);
        e.call(0x004a_3e00, &args![local(0x70), local(0x5c)]);
        let mass = x87_store32(e.call(0x004b_5400, &args![reference]).f64());
        e.call(0x005d_bf20, &args![local(0x80), mass]);
        e.call(0x0062_7920, &args![local(0x70), local(0x80)]);
        let body = e.call(OBJECT_004AE750, &args![reference]).u32();
        e.call(0x0081_82d0, &args![body, local(0x70)]);
    });
}

// Translated from 0082b110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082b110 - the C++ exception frame (FS:[0] chain, unwind state 0 around the new effect) is not translated.
/// Unnamed in the engine map (cdecl, three words): a new
/// `TelekinesisEffect` (0x60 bytes, `0082a460` with the caster, the spell
/// and the effect item), or 0 when the allocation failed.
pub fn fn_0082b110(e: &mut Engine, caster: u32, spell: u32, effect: u32) -> u32 {
    let memory = e.call(OPERATOR_NEW, &args![0x60u32]).u32();
    if memory == 0 {
        return 0;
    }
    fn_0082a460(e, Ptr::new(memory), caster, spell, effect).addr()
}

// Translated from 0082b190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TelekinesisEffect::GetSaveSize` (engine map; `thiscall`, `RET 4`):
/// `ValueModifierEffect::GetSaveSize` plus 4 (the reference's id), plus 8
/// more (hold distance and velocity) when the save/load singleton's byte
/// at +0x80 (`008df040`) is at least 0x40; 16 bits.
pub fn telekinesis_effect_get_save_size(
    e: &mut Engine,
    this: Ptr<TelekinesisEffect>,
    word: u32,
) -> u16 {
    let base = e
        .call(VALUE_MODIFIER_EFFECT_GET_SAVE_SIZE, &args![this, word])
        .u16();
    let mut size = base.wrapping_add(4);
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    if e.call(SAVE_LOAD_BYTE_AT_80, &args![game]).u8() >= 0x40 {
        size = size.wrapping_add(4).wrapping_add(4);
    }
    size
}

// Translated from 0082b200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TelekinesisEffect::SaveGame` (engine map; `thiscall`, `RET 4`):
/// `ValueModifierEffect::SaveGame`, the reference's form id (0 without one)
/// through `TESSaveLoadGame::SaveNumericID` (from a stack word), then, for
/// its byte at +0x80 (`008df040`) from 0x40 on, the hold distance and the velocity (4
/// bytes each, `008579b0`).
pub fn telekinesis_effect_save_game(e: &mut Engine, this: Ptr<TelekinesisEffect>, word: u32) {
    e.call(VALUE_MODIFIER_EFFECT_SAVE_GAME, &args![this, word]);
    e.with_stack(4, |e, id| {
        e.mem.set_u32(id.addr(), 0);
        let reference = e.get(this, TelekinesisEffect::pRef);
        if reference != 0 {
            let form_id = e.call(FORM_ID, &args![reference]).u32();
            e.mem.set_u32(id.addr(), form_id);
        }
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_NUMERIC_ID, &args![game, id, 4u32]);
    });
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    if e.call(SAVE_LOAD_BYTE_AT_80, &args![game]).u8() >= 0x40 {
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_LOAD_WRITE, &args![game, this.addr() + 0x50, 4u32]);
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_LOAD_WRITE, &args![game, this.addr() + 0x54, 4u32]);
    }
}

// Translated from 0082b290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TelekinesisEffect::LoadGame` (engine map; `thiscall`, `RET 4`):
/// `ValueModifierEffect::LoadGame`, the reference's id into `pRef`
/// (`TESSaveLoadGame::LoadNumericID` into a stack word, resolved later by
/// `FinishInitLoadGame`), then, for its byte at +0x80 from 0x40 on (`008df040`), the hold
/// distance and the velocity (`008579e0`).
pub fn telekinesis_effect_load_game(e: &mut Engine, this: Ptr<TelekinesisEffect>, word: u32) {
    e.call(VALUE_MODIFIER_EFFECT_LOAD_GAME, &args![this, word]);
    e.with_stack(4, |e, id| {
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(LOAD_NUMERIC_ID, &args![game, id, 4u32]);
        let value = e.mem.u32(id.addr());
        e.set(this, TelekinesisEffect::pRef, value);
    });
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    if e.call(SAVE_LOAD_BYTE_AT_80, &args![game]).u8() >= 0x40 {
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_LOAD_READ, &args![game, this.addr() + 0x50, 4u32]);
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_LOAD_READ, &args![game, this.addr() + 0x54, 4u32]);
    }
}

// Translated from 0082b300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TelekinesisEffect::FinishInitLoadGame` (engine map; `thiscall`, `RET
/// 4`): `ValueModifierEffect::FinishInitLoadGame`; the loaded id in `pRef`
/// becomes its form (`004839c0`) as a `TESObjectREFR` (`__RTDynamicCast`);
/// with a reference the spring is made again (`CreateSpring`); then
/// `bFirstUpdate` is set.
pub fn telekinesis_effect_finish_init_load_game(
    e: &mut Engine,
    this: Ptr<TelekinesisEffect>,
    word: u32,
) {
    e.call(
        VALUE_MODIFIER_EFFECT_FINISH_INIT_LOAD_GAME,
        &args![this, word],
    );
    let id = e.get(this, TelekinesisEffect::pRef);
    if id != 0 {
        let form = e.call(LOOK_UP_FORM, &args![id]).u32();
        let reference = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
            )
            .u32();
        e.set(this, TelekinesisEffect::pRef, reference);
    }
    if e.get(this, TelekinesisEffect::pRef) != 0 {
        telekinesis_effect_create_spring(e, this);
    }
    e.set(this, TelekinesisEffect::bFirstUpdate, 1);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082_a650, telekinesis_effect_copy_data(Ptr, Ptr)),
        entry!(0x0082_aeb0, fn_0082aeb0(Ptr) -> u32),
        entry!(0x0082_aee0, fn_0082aee0(Ptr) -> u32),
        entry!(
            0x0082_a460,
            fn_0082a460(Ptr<TelekinesisEffect>, u32, u32, u32) -> Ptr<TelekinesisEffect>
        ),
        entry!(
            0x0082_a510,
            telekinesis_effect_scalar_deleting_destructor(
                Ptr<TelekinesisEffect>,
                u32,
            ) -> Ptr<TelekinesisEffect>
        ),
        entry!(0x0082_a540, fn_0082a540(Ptr<TelekinesisEffect>)),
        entry!(
            0x0082_a5a0,
            telekinesis_effect_copy(Ptr<TelekinesisEffect>) -> u32
        ),
        entry!(
            0x0082_a670,
            telekinesis_effect_start(Ptr<TelekinesisEffect>)
        ),
        entry!(
            0x0082_a6a0,
            telekinesis_effect_create_spring(Ptr<TelekinesisEffect>)
        ),
        entry!(0x0082_aa50, fn_0082aa50(Ptr, u32)),
        entry!(0x0082_aa90, fn_0082aa90(Ptr) -> u32),
        entry!(0x0082_aab0, fn_0082aab0(Ptr, f32)),
        entry!(
            0x0082_aad0,
            telekinesis_effect_update(Ptr<TelekinesisEffect>, f32)
        ),
        entry!(0x0082_af20, fn_0082af20(Ptr) -> u32),
        entry!(0x0082_af40, fn_0082af40(Ptr) -> f32),
        entry!(
            0x0082_af60,
            telekinesis_effect_finish(Ptr<TelekinesisEffect>)
        ),
        entry!(0x0082_afa0, fn_0082afa0(Ptr<TelekinesisEffect>)),
        entry!(0x0082_b110, fn_0082b110(u32, u32, u32) -> u32),
        entry!(
            0x0082_b190,
            telekinesis_effect_get_save_size(Ptr<TelekinesisEffect>, u32) -> u16
        ),
        entry!(
            0x0082_b200,
            telekinesis_effect_save_game(Ptr<TelekinesisEffect>, u32)
        ),
        entry!(
            0x0082_b290,
            telekinesis_effect_load_game(Ptr<TelekinesisEffect>, u32)
        ),
        entry!(
            0x0082_b300,
            telekinesis_effect_finish_init_load_game(Ptr<TelekinesisEffect>, u32)
        ),
    ]
}

#[cfg(test)]
mod object_tests {
    use super::*;

    #[test]
    fn fn_0082aee0_follows_the_object_to_its_word_18() {
        let mut e = Engine::new();
        e.register(OBJECT_004AE750, |_, a| (a[0] + 0x100).into_ret());
        e.register(WORD_18_009611E0, |_, a| (a[0] + 0x18).into_ret());
        assert_eq!(e.call(0x0082_aee0, &[0x1000]).u32(), 0x1118);
        e.register(OBJECT_004AE750, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_aee0, &[0x1000]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == WORD_18_009611E0));
    }

    #[test]
    fn fn_0082aeb0_passes_it_on() {
        let mut e = Engine::new();
        e.register(OBJECT_004AE750, |_, a| (a[0] + 0x100).into_ret());
        e.register(WORD_18_009611E0, |_, a| (a[0] + 0x18).into_ret());
        e.register(GLOBAL_004B5A80, |_, a| (a[0] ^ 0xffff).into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0082_aeb0, &[0x1000]).u32(), 0x1118 ^ 0xffff);
        let log = e.call_log.take().unwrap();
        let passed: Vec<_> = log.iter().filter(|(a, _)| *a == GLOBAL_004B5A80).collect();
        assert_eq!(passed[0].1, vec![0x1118]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn forwarder_telekinesis_effect_copy_data_forwards_this_and_the_argument() {
        let mut e = Engine::new();
        e.register(COPY_DATA_TARGET_0082BBE0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_a650, &args![0x1000u32, 0x2000u32]);
        let log = e.call_log.take().unwrap();
        let hits: Vec<_> = log
            .iter()
            .filter(|call| call.0 == COPY_DATA_TARGET_0082BBE0)
            .collect();
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].1, vec![0x1000u32, 0x2000]);
    }
}

#[cfg(test)]
mod telekinesis_tests {
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

    const SLOTS: u32 = 0x00fa_0000;

    /// An object whose vtable slots `slots` point at `SLOTS + slot`.
    fn with_vtable(e: &mut Engine, size: u32, slots: &[u32]) -> u32 {
        let object = e.mem.alloc(size);
        let vtable = e.mem.alloc(0x200);
        for slot in slots {
            e.mem.set_u32(vtable + slot, SLOTS + slot);
        }
        e.mem.set_u32(object, vtable);
        object
    }

    /// The globals mapped, the player (0x700 bytes) in place, settings with
    /// their values at +4.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        e.map(0x011d_1000, 0x1000);
        e.map(0x0101_f000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0102_2000, 0x1000);
        let player = e.mem.alloc(0x700);
        e.set_global(PLAYER, player);
        e.set_global(SAVE_LOAD_GAME, 0x4444u32);
        e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
        e.register(0x0055_9450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(ACTIVE_EFFECT_DISPEL, |_, _| Ret::default());
        (e, player)
    }

    fn effect(e: &mut Engine) -> Ptr<TelekinesisEffect> {
        Ptr::new(e.mem.alloc(0x60))
    }

    #[test]
    fn constructor_starts_empty() {
        let (mut e, _) = engine();
        quiet(
            &mut e,
            &[
                VALUE_MODIFIER_EFFECT_CONSTRUCT,
                POINTER_CONSTRUCT,
                POINTER_ASSIGN,
            ],
        );
        let this = e.mem.alloc(0x60);
        e.mem.write(this, &[0xcc; 0x60]);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x0082_a460, &args![this, 1u32, 2u32, 3u32]).u32(),
            this
        );
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_CONSTRUCT),
            vec![vec![this, 1, 2, 3]]
        );
        assert_eq!(calls(&e, POINTER_CONSTRUCT), vec![vec![this + 0x4c, 0]]);
        assert_eq!(calls(&e, POINTER_ASSIGN), vec![vec![this + 0x4c, 0]]);
        assert_eq!(e.mem.u32(this), TELEKINESIS_EFFECT_VTABLE);
        assert_eq!(e.mem.bytes(this + 0x50, 0xe), vec![0; 0xe]);
    }

    #[test]
    fn destructor_body_releases_the_spring_then_the_base() {
        let (mut e, _) = engine();
        quiet(&mut e, &[POINTER_DESTRUCT, VALUE_MODIFIER_EFFECT_DESTRUCT]);
        e.call_log = Some(vec![]);
        e.call(0x0082_a540, &args![0x100u32]);
        let order: Vec<(u32, Vec<u32>)> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != 0x0082_a540)
            .collect();
        assert_eq!(
            order,
            vec![
                (POINTER_DESTRUCT, vec![0x14c]),
                (VALUE_MODIFIER_EFFECT_DESTRUCT, vec![0x100]),
            ]
        );
    }

    #[test]
    fn deleting_destructor_frees_on_flag_one() {
        let (mut e, _) = engine();
        quiet(
            &mut e,
            &[POINTER_DESTRUCT, VALUE_MODIFIER_EFFECT_DESTRUCT, FREE],
        );
        for (flags, frees) in [(0u32, 0usize), (2, 0), (1, 1), (3, 1)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0082_a510, &args![0x100u32, flags]).u32(), 0x100);
            assert_eq!(calls(&e, FREE).len(), frees);
        }
    }

    #[test]
    fn copy_makes_a_new_effect_and_fills_it() {
        let (mut e, _) = engine();
        quiet(
            &mut e,
            &[
                VALUE_MODIFIER_EFFECT_CONSTRUCT,
                POINTER_CONSTRUCT,
                POINTER_ASSIGN,
            ],
        );
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let this = with_vtable(&mut e, 0x60, &[0x44]);
        e.register(SLOTS + 0x44, |_, _| Ret::default());
        e.mem.set_u32(this + 0x28, 0xa);
        e.mem.set_u32(this + 8, 0xb);
        e.mem.set_u32(this + 0xc, 0xc);
        e.call_log = Some(vec![]);
        let copy = e.call(0x0082_a5a0, &args![this]).u32();
        assert_eq!(calls(&e, OPERATOR_NEW), vec![vec![0x60]]);
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_CONSTRUCT),
            vec![vec![copy, 0xa, 0xb, 0xc]]
        );
        assert_eq!(calls(&e, SLOTS + 0x44), vec![vec![this, copy]]);
    }

    #[test]
    fn start_takes_the_interface_reference() {
        let (mut e, _) = engine();
        quiet(&mut e, &[VALUE_MODIFIER_EFFECT_START]);
        e.register(GET_TELEKINESIS_REFR, |_, _| Ret::default());
        let this = effect(&mut e);
        e.set(this, TelekinesisEffect::bFirstUpdate, 1);
        e.set(this, TelekinesisEffect::pRef, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x0082_a670, &args![this]);
        // No reference: the spring is not made and the effect is dispelled.
        assert_eq!(e.get(this, TelekinesisEffect::pRef), 0);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL), vec![vec![this.addr(), 0]]);
        assert_eq!(e.get(this, TelekinesisEffect::bFirstUpdate), 0);
    }

    /// Doubles for the whole `CreateSpring` path. Returns the reference.
    fn spring_world(e: &mut Engine, player: u32, kind: u32) -> u32 {
        let reference = with_vtable(e, 0x20, &[0x1d0]);
        e.register(SLOTS + 0x1d0, |_, _| 0x5100u32.into_ret());
        e.register(EFFECT_CASTER, |e, a| e.mem.u32(a[0] + 0x28).into_ret());
        e.register(0x007a_f430, |_, _| 0x5200u32.into_ret());
        e.register(0x0040_1170, |_, _| 0x10u32.into_ret());
        e.register(0x004b_5260, |_, _| 0x5300u32.into_ret());
        e.register(0x006f_a820, |_, _| 0x5400u32.into_ret());
        let kind_word = e.mem.alloc(4);
        e.mem.set_u32(kind_word, kind);
        e.register_double(0x0043_b4f0, move |_, _| kind_word.into_ret());
        e.register(0x0043_b4d0, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(OBJECT_004AE750, |_, a| (a[0] + 0x10).into_ret());
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x00aa_13e0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x0065_3270, |_, _| 0x5500u32.into_ret());
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        e.register(0x004b_6190, |_, _| Ret {
            st0: 42.0,
            ..Ret::default()
        });
        quiet(
            e,
            &[
                0x0080_7ac0,
                0x0080_7790,
                0x00b4_dec0,
                0x0096_1280,
                0x0095_f930,
                0x008a_ff80,
                0x0045_8620,
                0x0043_9ef0,
                0x0043_9180,
                PROCESS_LISTS_ADD_TEMP_EFFECT,
            ],
        );
        e.register(0x0047_c850, |_, _| Ret::default());
        e.register(EFFECT_SETTING, |_, _| 0x5600u32.into_ret());
        e.register(SETTING_MEMBER_00407820, |_, _| Ret::default());
        // The player holds something.
        e.mem.set_u32(player + 0x634, 0x5700);
        e.set_global(CONSTANT_01022958, 10000.0f32);
        e.set_global(CONSTANT_01012054, -1.0f32);
        reference
    }

    #[test]
    fn create_spring_dispels_without_a_usable_object() {
        let (mut e, player) = engine();
        let reference = spring_world(&mut e, player, 4);
        let this = effect(&mut e);
        e.set(this, TelekinesisEffect::pRef, reference);
        // No caster.
        e.call_log = Some(vec![]);
        telekinesis_effect_create_spring(&mut e, this);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL).len(), 1);
        e.mem.set_u32(this.addr() + 0x28, 0x77);
        // A form of type 0x1b.
        e.register(0x0040_1170, |_, _| 0x1bu32.into_ret());
        e.call_log = Some(vec![]);
        telekinesis_effect_create_spring(&mut e, this);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL).len(), 1);
        assert!(calls(&e, 0x004b_5260).is_empty());
        e.register(0x0040_1170, |_, _| 0x10u32.into_ret());
        // An object of another kind.
        let other = e.mem.alloc(4);
        e.mem.set_u32(other, 3);
        e.register_double(0x0043_b4f0, move |_, _| other.into_ret());
        e.call_log = Some(vec![]);
        telekinesis_effect_create_spring(&mut e, this);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL).len(), 1);
        assert_eq!(calls(&e, 0x0043_b4d0).len(), 2);
        assert!(calls(&e, 0x0080_7790).is_empty());
        // No collision object.
        e.register(0x004b_5260, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        telekinesis_effect_create_spring(&mut e, this);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL).len(), 1);
        assert!(calls(&e, 0x0080_7ac0).is_empty());
    }

    #[test]
    fn create_spring_makes_the_spring_and_the_hit_effect() {
        let (mut e, player) = engine();
        let reference = spring_world(&mut e, player, 5);
        let this = effect(&mut e);
        e.set(this, TelekinesisEffect::pRef, reference);
        e.mem.set_u32(this.addr() + 0x28, 0x77);
        e.register(SETTING_MEMBER_00407820, |_, _| 0x5800u32.into_ret());
        let hit = with_vtable(&mut e, 0x10, &[0xc4, 0x0]);
        e.register(SLOTS + 0xc4, |_, _| 1u32.into_ret());
        e.register(SLOTS, |_, _| Ret::default());
        e.register_double(0x0081_f580, move |_, _| hit.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0082_a6a0, &args![this]);
        assert!(calls(&e, ACTIVE_EFFECT_DISPEL).is_empty());
        assert_eq!(calls(&e, 0x0080_7ac0), vec![vec![0x5400]]);
        let spring = calls(&e, 0x0080_7790);
        assert_eq!(spring[0][1..], [reference, 0x5400, 5]);
        assert_eq!(calls(&e, 0x0065_3270), vec![vec![0x011f_9140, 0x5100]]);
        assert_eq!(calls(&e, 0x00b4_dec0), vec![vec![0x5500, 6]]);
        assert_eq!(calls(&e, 0x0096_1280), vec![vec![player]]);
        assert_eq!(
            calls(&e, 0x0095_f930),
            vec![vec![player, reference, 3, 10000.0f32.to_bits()]]
        );
        // The dot product became the hold distance and went to the player.
        assert_eq!(
            e.get(this, TelekinesisEffect::fHoldDistance),
            42.0f32.to_bits()
        );
        assert_eq!(e.mem.u32(player + 0x644), 42.0f32.to_bits());
        assert_eq!(calls(&e, 0x0045_8620)[0][1], 0x5700 + 0x10 + 0x30);
        let hits = calls(&e, 0x0081_f580);
        assert_eq!(hits[0][1..], [reference, 0x5800, (-1.0f32).to_bits()]);
        assert_eq!(
            calls(&e, PROCESS_LISTS_ADD_TEMP_EFFECT),
            vec![vec![PROCESS_LISTS, hit]]
        );
        // Not added: deleted instead; and `0047c850` true keeps the distance.
        e.register(SLOTS + 0xc4, |_, _| Ret::default());
        e.register(0x0047_c850, |_, _| 1u32.into_ret());
        e.set(this, TelekinesisEffect::fHoldDistance, 7.0f32.to_bits());
        e.call_log = Some(vec![]);
        telekinesis_effect_create_spring(&mut e, this);
        assert_eq!(calls(&e, SLOTS), vec![vec![hit, 1]]);
        assert!(calls(&e, 0x004b_6190).is_empty());
        assert_eq!(
            e.get(this, TelekinesisEffect::fHoldDistance),
            7.0f32.to_bits()
        );
    }

    #[test]
    fn grabbed_object_position() {
        let (mut e, _) = engine();
        e.register(OBJECT_004AE750, |_, a| {
            if a[0] == 1 { 0 } else { 0x9000 }.into_ret()
        });
        e.register(0x0045_8620, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0082_aa50, &args![2u32, 0x300u32]);
        fn_0082aa50(&mut e, Ptr::new(1), 0x300);
        assert_eq!(calls(&e, 0x0045_8620), vec![vec![0x300, 0x9030]]);
    }

    #[test]
    fn player_grab_word() {
        let (mut e, player) = engine();
        e.mem.set_u32(player + 0x634, 0x1234);
        assert_eq!(e.call(0x0082_aa90, &args![player]).u32(), 0x1234);
    }

    #[test]
    fn hold_distance_store_and_read() {
        let (mut e, player) = engine();
        e.call(0x0082_aab0, &args![player, 0x7f80_0001u32]);
        assert_eq!(e.mem.u32(player + 0x644), 0x7fc0_0001);
        e.mem.set_u32(player + 0x644, 0x7f80_0002);
        assert_eq!(fn_0082af40(&mut e, Ptr::new(player)).to_bits(), 0x7fc0_0002);
        e.mem.set_u32(player + 0x644, 3.5f32.to_bits());
        assert_eq!(e.call(0x0082_af40, &args![player]).f32(), 3.5);
    }

    #[test]
    fn grab_state_word() {
        let (mut e, player) = engine();
        e.mem.set_u32(player + 0x63c, 3);
        assert_eq!(e.call(0x0082_af20, &args![player]).u32(), 3);
    }

    /// Doubles for `Update`: a grab holding something, the player's word
    /// and the caster's agreeing, state 3, the controls from a byte mask at
    /// `0x011d_e900`.
    fn update_world(e: &mut Engine, player: u32) -> Ptr<TelekinesisEffect> {
        quiet(e, &[VALUE_MODIFIER_EFFECT_UPDATE, 0x0096_0520]);
        e.mem.set_u32(player + 0x634, 0x5700);
        e.mem.set_u32(player + 0x63c, 3);
        e.register(OBJECT_004AE750, |_, _| 0x5800u32.into_ret());
        e.register(WORD_18_009611E0, |_, _| 0x5900u32.into_ret());
        e.register(GLOBAL_004B5A80, |_, _| 0x5a00u32.into_ret());
        let caster = with_vtable(e, 0x10, &[0x2c]);
        e.register(SLOTS + 0x2c, |_, _| 0x6100u32.into_ret());
        e.register(0x0089_f4e0, |_, _| 0x6000u32.into_ret());
        e.register(0x008d_6f30, |_, a| (a[0] + 1).into_ret());
        e.register(0x0045_43c0, |_, a| (a[0] & 0xf).into_ret());
        e.register(CONTROL_00877720, |_, a| {
            assert_eq!(a.len(), 1);
            0x6300u32.into_ret()
        });
        e.register(CONTROL_DOWN_00A24660, |e, a| {
            assert_eq!((a[0], a[2]), (0x6300, 0));
            u32::from(e.mem.u32(0x011d_e900) & (1 << a[1]) != 0).into_ret()
        });
        e.set_global(0x011d_e900u32, 0u32);
        e.set_global(CONTROLS_011DEA0C, 0x6200u32);
        e.set_global(CONSTANT_0101FFA0, 0.5f64);
        for (setting, value) in [
            (SETTING_011D1CF4, 2.0f32),
            (SETTING_011D1A3C, 4.0),
            (SETTING_011D1CAC, 10.0),
            (SETTING_011D1D24, 50.0),
            (SETTING_011D15E0, 100.0),
            (SETTING_011D18FC, 2.0),
        ] {
            e.set_global(setting + 4, value);
        }
        e.register(0x0049_5460, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        let this = effect(e);
        e.mem.set_u32(this.addr() + 0x28, caster);
        e.register(EFFECT_CASTER, |e, a| e.mem.u32(a[0] + 0x28).into_ret());
        e.set(this, TelekinesisEffect::pRef, 0x6000);
        e.mem.set_u32(player + 0x644, 100.0f32.to_bits());
        this
    }

    fn velocity(e: &Engine, this: Ptr<TelekinesisEffect>) -> f32 {
        f32::from_bits(e.get(this, TelekinesisEffect::fMoveVelocity))
    }

    #[test]
    fn update_moves_the_held_reference_with_the_controls() {
        let (mut e, player) = engine();
        let this = update_world(&mut e, player);
        // Control 4: from at least 2.0, plus 4 * 0.5, at most 10.
        e.set_global(0x011d_e900u32, 1u32 << 4);
        e.call_log = Some(vec![]);
        e.call(0x0082_aad0, &args![this, 0.5f32]);
        assert!(calls(&e, ACTIVE_EFFECT_DISPEL).is_empty());
        assert_eq!(velocity(&e, this), 4.0);
        // Hold distance: 100 + 4 * 0.5 = 102, at most 2 * (1 * 100) = 200.
        assert_eq!(
            e.get(this, TelekinesisEffect::fHoldDistance),
            102.0f32.to_bits()
        );
        assert_eq!(e.mem.u32(player + 0x644), 102.0f32.to_bits());
        assert_eq!(calls(&e, 0x0096_0520), vec![vec![player, 0.5f32.to_bits()]]);
        e.set(this, TelekinesisEffect::fMoveVelocity, 9.0f32.to_bits());
        telekinesis_effect_update(&mut e, this, 0.5);
        assert_eq!(velocity(&e, this), 10.0);
        // Control 6: down to at most -2, minus 2, at least -10.
        e.set_global(0x011d_e900u32, 1u32 << 6);
        e.set(this, TelekinesisEffect::fMoveVelocity, 1.0f32.to_bits());
        telekinesis_effect_update(&mut e, this, 0.5);
        assert_eq!(velocity(&e, this), -4.0);
        e.set(this, TelekinesisEffect::fMoveVelocity, (-9.0f32).to_bits());
        telekinesis_effect_update(&mut e, this, 0.5);
        assert_eq!(velocity(&e, this), -10.0);
        // Neither: 0; the distance is clamped between 50 and 200.
        e.set_global(0x011d_e900u32, 0u32);
        e.mem.set_u32(player + 0x644, 10.0f32.to_bits());
        telekinesis_effect_update(&mut e, this, 0.5);
        assert_eq!(velocity(&e, this), 0.0);
        assert_eq!(
            e.get(this, TelekinesisEffect::fHoldDistance),
            50.0f32.to_bits()
        );
        e.mem.set_u32(player + 0x644, 500.0f32.to_bits());
        e.set(this, TelekinesisEffect::bFirstUpdate, 1);
        e.call_log = Some(vec![]);
        telekinesis_effect_update(&mut e, this, 0.5);
        assert_eq!(
            e.get(this, TelekinesisEffect::fHoldDistance),
            200.0f32.to_bits()
        );
        assert!(calls(&e, 0x0096_0520).is_empty());
        assert_eq!(e.get(this, TelekinesisEffect::bFirstUpdate), 0);
    }

    #[test]
    fn update_throws_or_dispels() {
        let (mut e, player) = engine();
        let this = update_world(&mut e, player);
        // Both controls: throw when the velocity is above 4 * 0.5 + 2 = 4.
        e.set_global(0x011d_e900u32, (1u32 << 4) | (1 << 6));
        e.set(this, TelekinesisEffect::fMoveVelocity, 4.0f32.to_bits());
        e.call_log = Some(vec![]);
        telekinesis_effect_update(&mut e, this, 0.5);
        assert_eq!(e.get(this, TelekinesisEffect::bThrow), 0);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL).len(), 1);
        e.set(this, TelekinesisEffect::fMoveVelocity, 4.5f32.to_bits());
        telekinesis_effect_update(&mut e, this, 0.5);
        assert_eq!(e.get(this, TelekinesisEffect::bThrow), 1);
        // Another reference, state or a disagreeing word: dispelled at once.
        e.set_global(0x011d_e900u32, 0u32);
        for change in 0..3 {
            match change {
                0 => e.set(this, TelekinesisEffect::pRef, 0x6001),
                1 => {
                    e.set(this, TelekinesisEffect::pRef, 0x6000);
                    e.mem.set_u32(player + 0x63c, 2);
                }
                _ => {
                    e.mem.set_u32(player + 0x63c, 3);
                    e.register(SLOTS + 0x2c, |_, _| 0x6102u32.into_ret());
                }
            }
            e.call_log = Some(vec![]);
            telekinesis_effect_update(&mut e, this, 0.5);
            assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL).len(), 1);
            assert!(calls(&e, CONTROL_00877720).is_empty());
        }
        // Nothing held: dispelled.
        e.register(GLOBAL_004B5A80, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        telekinesis_effect_update(&mut e, this, 0.5);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL).len(), 1);
    }

    #[test]
    fn finish_lets_go_and_ends_the_shader() {
        let (mut e, player) = engine();
        e.mem.set_u32(player + 0x634, 0);
        e.register(EFFECT_SETTING, |_, _| 0x5600u32.into_ret());
        e.register(SETTING_MEMBER_00407820, |_, _| 0x5800u32.into_ret());
        quiet(
            &mut e,
            &[
                PROCESS_LISTS_FINISH_SHADER_HIT_EFFECT,
                VALUE_MODIFIER_EFFECT_FINISH,
            ],
        );
        let this = effect(&mut e);
        e.set(this, TelekinesisEffect::pRef, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x0082_af60, &args![this]);
        assert_eq!(
            calls(&e, PROCESS_LISTS_FINISH_SHADER_HIT_EFFECT),
            vec![vec![PROCESS_LISTS, 0x6000, 0x5800]]
        );
        assert_eq!(
            calls(&e, VALUE_MODIFIER_EFFECT_FINISH),
            vec![vec![this.addr()]]
        );
    }

    #[test]
    fn letting_go_drops_or_throws() {
        let (mut e, player) = engine();
        e.mem.set_u32(player + 0x634, 0x5700);
        e.register(OBJECT_004AE750, |_, a| (a[0] + 0x100).into_ret());
        e.register(WORD_18_009611E0, |_, _| 0x5900u32.into_ret());
        e.register(GLOBAL_004B5A80, |_, _| 0x5a00u32.into_ret());
        quiet(
            &mut e,
            &[
                0x0096_1280,
                0x0056_15d0,
                0x00a5_9660,
                0x0043_9f50,
                0x0045_bb20,
                0x004a_3e00,
                0x005d_bf20,
                0x0062_7920,
                0x0081_82d0,
            ],
        );
        e.register(0x0068_15c0, |_, a| a[0].into_ret());
        let angles = e.mem.alloc(12);
        for (i, v) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_u32(angles + 4 * i as u32, v.to_bits());
        }
        e.register_double(0x0043_0830, move |_, _| angles.into_ret());
        e.register(0x004b_5400, |_, _| Ret {
            st0: 8.0,
            ..Ret::default()
        });
        e.set_global(SETTING_011D17E8 + 4, 5.0f32);
        let this = effect(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0082_afa0, &args![this]);
        assert_eq!(calls(&e, 0x0096_1280), vec![vec![player]]);
        assert_eq!(calls(&e, 0x0056_15d0), vec![vec![0x5a00, 0x011f_426c]]);
        // Throwing.
        e.set(this, TelekinesisEffect::bThrow, 1);
        e.set(this, TelekinesisEffect::fMoveVelocity, 1.5f32.to_bits());
        e.call_log = Some(vec![]);
        fn_0082afa0(&mut e, this);
        let matrix = calls(&e, 0x00a5_9660);
        assert_eq!(
            matrix[0][1..],
            [3.0f32.to_bits(), 1.0f32.to_bits(), 2.0f32.to_bits()]
        );
        let scaled = calls(&e, 0x0045_bb20);
        assert_eq!(scaled[0][2], 6.5f32.to_bits());
        assert_eq!(calls(&e, 0x005d_bf20)[0][1], 8.0f32.to_bits());
        assert_eq!(calls(&e, 0x0081_82d0)[0][0], 0x5a00 + 0x100);
        assert!(calls(&e, 0x0056_15d0).is_empty());
        // Nothing held: nothing at all.
        e.mem.set_u32(player + 0x634, 0);
        e.call_log = Some(vec![]);
        fn_0082afa0(&mut e, this);
        assert!(calls(&e, 0x0096_1280).is_empty());
    }

    #[test]
    fn create_or_zero() {
        let (mut e, _) = engine();
        quiet(
            &mut e,
            &[
                VALUE_MODIFIER_EFFECT_CONSTRUCT,
                POINTER_CONSTRUCT,
                POINTER_ASSIGN,
            ],
        );
        e.register(OPERATOR_NEW, |e, a| e.mem.alloc(a[0]).into_ret());
        let made = e.call(0x0082_b110, &args![1u32, 2u32, 3u32]).u32();
        assert_eq!(e.mem.u32(made), TELEKINESIS_EFFECT_VTABLE);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        assert_eq!(fn_0082b110(&mut e, 1, 2, 3), 0);
    }

    #[test]
    fn save_size_by_the_singleton_byte() {
        let (mut e, _) = engine();
        e.register(VALUE_MODIFIER_EFFECT_GET_SAVE_SIZE, |_, _| {
            0x20u32.into_ret()
        });
        e.register(SAVE_LOAD_BYTE_AT_80, |e, _| {
            e.mem.u32(0x011d_e900).into_ret()
        });
        e.set_global(0x011d_e900u32, 0x3fu32);
        assert_eq!(e.call(0x0082_b190, &args![0x100u32, 0u32]).u32(), 0x24);
        e.set_global(0x011d_e900u32, 0x40u32);
        let this: Ptr<TelekinesisEffect> = Ptr::new(0x100);
        assert_eq!(telekinesis_effect_get_save_size(&mut e, this, 0), 0x2c);
    }

    #[test]
    fn save_writes_the_reference_id_and_the_floats() {
        let (mut e, _) = engine();
        quiet(&mut e, &[VALUE_MODIFIER_EFFECT_SAVE_GAME, SAVE_LOAD_WRITE]);
        e.register(FORM_ID, |_, a| (a[0] + 1).into_ret());
        e.register(SAVE_LOAD_BYTE_AT_80, |_, _| 0x40u32.into_ret());
        let ids = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = ids.clone();
        e.register_double(SAVE_NUMERIC_ID, move |e, a| {
            seen.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        let this = effect(&mut e);
        e.set(this, TelekinesisEffect::pRef, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x0082_b200, &args![this, 7u32]);
        e.set(this, TelekinesisEffect::pRef, 0);
        telekinesis_effect_save_game(&mut e, this, 7);
        assert_eq!(*ids.borrow(), vec![0x6001, 0]);
        let base = this.addr();
        assert_eq!(
            calls(&e, SAVE_LOAD_WRITE),
            vec![
                vec![0x4444, base + 0x50, 4],
                vec![0x4444, base + 0x54, 4],
                vec![0x4444, base + 0x50, 4],
                vec![0x4444, base + 0x54, 4],
            ]
        );
        e.register(SAVE_LOAD_BYTE_AT_80, |_, _| 0x3fu32.into_ret());
        e.call_log = Some(vec![]);
        telekinesis_effect_save_game(&mut e, this, 7);
        assert!(calls(&e, SAVE_LOAD_WRITE).is_empty());
    }

    #[test]
    fn load_reads_the_reference_id_and_the_floats() {
        let (mut e, _) = engine();
        quiet(&mut e, &[VALUE_MODIFIER_EFFECT_LOAD_GAME, SAVE_LOAD_READ]);
        e.register(LOAD_NUMERIC_ID, |e, a| {
            e.mem.set_u32(a[1], 0x0001_2345);
            Ret::default()
        });
        e.register(SAVE_LOAD_BYTE_AT_80, |_, _| 0x3fu32.into_ret());
        let this = effect(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0082_b290, &args![this, 7u32]);
        assert_eq!(e.get(this, TelekinesisEffect::pRef), 0x0001_2345);
        assert!(calls(&e, SAVE_LOAD_READ).is_empty());
        e.register(SAVE_LOAD_BYTE_AT_80, |_, _| 0x41u32.into_ret());
        telekinesis_effect_load_game(&mut e, this, 7);
        assert_eq!(calls(&e, SAVE_LOAD_READ).len(), 2);
    }

    #[test]
    fn finish_init_load_game_resolves_the_reference() {
        let (mut e, _) = engine();
        quiet(&mut e, &[VALUE_MODIFIER_EFFECT_FINISH_INIT_LOAD_GAME]);
        e.register(LOOK_UP_FORM, |_, a| (a[0] + 0x10).into_ret());
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        let this = effect(&mut e);
        e.set(this, TelekinesisEffect::pRef, 0x0001_2345);
        e.call_log = Some(vec![]);
        e.call(0x0082_b300, &args![this, 7u32]);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST),
            vec![vec![0x0001_2355, 0, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0]]
        );
        // Not a reference: no spring (no caster lookup), first update set.
        assert_eq!(e.get(this, TelekinesisEffect::pRef), 0);
        assert!(calls(&e, ACTIVE_EFFECT_DISPEL).is_empty());
        assert_eq!(e.get(this, TelekinesisEffect::bFirstUpdate), 1);
        // A reference without a caster: the spring attempt dispels.
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(EFFECT_CASTER, |_, _| Ret::default());
        e.set(this, TelekinesisEffect::pRef, 0x0001_2345);
        e.call_log = Some(vec![]);
        telekinesis_effect_finish_init_load_game(&mut e, this, 7);
        assert_eq!(e.get(this, TelekinesisEffect::pRef), 0x0001_2355);
        assert_eq!(calls(&e, ACTIVE_EFFECT_DISPEL).len(), 1);
    }
}
