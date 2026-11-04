//! `fallout shared/magic/effectsetting.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `EffectSetting` (a base effect form): its plugin record load and save,
//! the form-id fix-up after loading, the comparison of two effects, the
//! loaded/queued counters, and the template helpers the compiler placed in
//! this unit (`BSStringT<char>::StrCmp`, an endian swap of a 16-bit value,
//! the absolute value of a `float`, and the CRT `fabs`).
//!
//! x87 note: `FLD`/`FSTP` of a signalling NaN delivers it quiet (all
//! exceptions are masked); the helpers below do that explicitly.

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `EffectSetting` (Xbox PDB), 0xC0 bytes on the Xbox. `TESForm` is 0x10
    /// smaller on PC, so every field sits 0x10 lower (checked against the
    /// offsets this unit's code uses).
    pub struct EffectSetting: 0xB0 {
        /// `data.iFlags` (Xbox PDB): `EffectSettingData` starts here.
        0x58 iFlags: u32,
        /// `data.pAssociatedItem` (Xbox PDB): a `TESForm*`, or its form id
        /// between loading and `InitItem`.
        0x60 pAssociatedItem: u32,
        /// `data.iNumCounterEffects` (Xbox PDB).
        0x6C iNumCounterEffects: i16,
        /// `data.pLight` (Xbox PDB): `TESObjectLIGH*`.
        0x70 pLight: u32,
        /// `data.fSpeed` (Xbox PDB).
        0x74 fSpeed: f32,
        /// `data.pEffectShader` (Xbox PDB): `TESEffectShader*`.
        0x78 pEffectShader: u32,
        /// `data.pEnchantEffect` (Xbox PDB): `TESEffectShader*`.
        0x7C pEnchantEffect: u32,
        /// `data.pCastSound` (Xbox PDB): `TESSound*`.
        0x80 pCastSound: u32,
        /// `data.pBoltSound` (Xbox PDB).
        0x84 pBoltSound: u32,
        /// `data.pHitSound` (Xbox PDB).
        0x88 pHitSound: u32,
        /// `data.pAreaSound` (Xbox PDB).
        0x8C pAreaSound: u32,
        /// `data.eArchetype` (Xbox PDB).
        0x98 eArchetype: u32,
        /// `iEffectLoadedCount` (Xbox PDB).
        0xA8 iEffectLoadedCount: i32,
        /// `iAssociatedItemLoadedCount` (Xbox PDB).
        0xAC iAssociatedItemLoadedCount: i32,
    }

    /// `EffectSetting::EffectSettingData` (Xbox PDB), the `DATA` record,
    /// 0x48 bytes at `EffectSetting` +0x58.
    pub struct EffectSettingData: 0x48 {}
}

/// `EffectSetting::data` (+0x58) and `counterEffects` (+0xA0, a
/// `BSSimpleList<EffectSetting*>`, Xbox PDB).
const DATA: u32 = 0x58;
const COUNTER_EFFECTS: u32 = 0xa0;
/// Base components (Xbox PDB order, PC offsets): `TESModel` +0x18,
/// `TESDescription` +0x30, `TESFullName` +0x38, `TESIcon`/`TESTexture` +0x44.
const MODEL: u32 = 0x18;
const DESCRIPTION: u32 = 0x30;
const FULL_NAME: u32 = 0x38;
const ICON: u32 = 0x44;

/// Record and chunk signatures (little-endian four-character codes).
const SIG_DATA: u32 = 0x4154_4144; // "DATA"
const SIG_ESCE: u32 = 0x4543_5345; // "ESCE"
const SIG_EDID: u32 = 0x4449_4445; // "EDID"
const SIG_DESC: u32 = 0x4353_4544; // "DESC"
const SIG_OBND: u32 = 0x444e_424f; // "OBND"
const SIG_ICON: u32 = 0x4e4f_4349; // "ICON"
const SIG_MODL: u32 = 0x4c44_4f4d; // "MODL"
const SIG_FULL: u32 = 0x4c4c_5546; // "FULL"
const SIG_MODT: u32 = 0x5444_4f4d; // "MODT"

/// `TESForm` callees (Xbox PDB names where the map has them).
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `TESForm` chunk writer with a 4-byte value `(signature, value)` (cdecl).
const ADD_CHUNK_U32: u32 = 0x0048_5910;
const LOAD_FORM: u32 = 0x0048_5110;
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
const GET_FILE: u32 = 0x0048_4e60;
/// Form lookup by id (`004839c0`, cdecl, one word).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// Form id getter of a form (`0084e3a0`, thiscall).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// `TESForm` flag test `0058cba0(form, mask)` (bool).
const TEST_FORM_FLAGS: u32 = 0x0058_cba0;
/// `TESForm` comparison of all base components `00485270(form, other)`.
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// Called at the end of `InitItem` as `00484ab0(form, 1)`.
const INIT_ITEM_DONE: u32 = 0x0048_4ab0;
/// Component savers and loaders.
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const DESCRIPTION_SAVE: u32 = 0x0048_2cb0;
const TEXTURE_SAVE: u32 = 0x0048_e370;
const MODEL_SAVE: u32 = 0x0048_91d0;
const FULL_NAME_LOAD: u32 = 0x0048_7050;
const DESCRIPTION_LOAD: u32 = 0x0048_2d50;
const TEXTURE_LOAD: u32 = 0x0048_e3d0;
const MODEL_LOAD: u32 = 0x0048_92d0;

/// `TESFile` callees (Xbox PDB names): `GetTESForm`, `GetTESChunk`,
/// `GetChunkData` (4 bytes into a pointer), `GetChunkData_ov3(buffer, size)`;
/// `004726f0` advances to the next chunk (bool), `00401660` returns the
/// current chunk's size, `00401680` says whether the file needs an endian
/// swap.
const GET_TES_FORM: u32 = 0x0047_2660;
const GET_TES_CHUNK: u32 = 0x0047_26b0;
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
const NEXT_CHUNK: u32 = 0x0047_26f0;
const CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_NEEDS_SWAP: u32 = 0x0040_1680;
/// No argument: whether saving needs an endian swap.
const SAVE_NEEDS_SWAP: u32 = 0x0040_1500;
/// Endian swap of a 32-bit value in place, `(pointer, bool)` (cdecl).
const SWAP_U32: u32 = 0x0040_1080;
/// 16-bit byte swap of a value (cdecl, one word, result in `AX`).
const SWAP_U16_VALUE: u32 = 0x00ec_3bda;
/// `memset` (`00403d30`, cdecl).
const MEMSET: u32 = 0x0040_3d30;
/// `__RTDynamicCast(object, vfdelta, source, target, isReference)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptors: `.?AVTESForm@@`, `.?AVEffectSetting@@`,
/// `.?AVTESObjectLIGH@@`, `.?AVTESSound@@`, `.?AVTESEffectShader@@` (names
/// read at descriptor +8).
const RTTI_TES_FORM: u32 = 0x0118_3028;
const RTTI_EFFECT_SETTING: u32 = 0x0118_37c4;
const RTTI_LIGHT: u32 = 0x0118_3938;
const RTTI_SOUND: u32 = 0x0118_32fc;
const RTTI_EFFECT_SHADER: u32 = 0x0118_3918;
/// Formatted warning (`005b5e40`, cdecl, variadic).
const WARNING: u32 = 0x005b_5e40;
/// `BSSimpleList` helpers: `006815c0(node)` returns the address of the
/// node's item, `00726070(node)` the next node, `00470470(list)` empties
/// the list, `005ae3d0(list, &item)` adds an item, `005ae380(list)` counts.
const NODE_ITEM: u32 = 0x0068_15c0;
const NODE_NEXT: u32 = 0x0072_6070;
const LIST_CLEAR: u32 = 0x0047_0470;
const LIST_ADD: u32 = 0x005a_e3d0;
const LIST_COUNT: u32 = 0x005a_e380;
/// `_ftol2_sse`: truncates the value in ST0 (passed as an `f64`).
const FTOL: u32 = 0x00ec_62c0;
/// `BSStringT<char>`: c-string getter (`00559450`), copy of the full name
/// into a string (`004046b0(form, out)`), destructor (`004037d0`), the
/// two comparisons `StrCmp` picks from (cdecl `(text, other)`), and the
/// string setter `00489100(component, text)` (the engine map names it
/// `TESSoundFile::SetSoundFile`, a folded body).
const STRING_C_STR: u32 = 0x0055_9450;
const GET_FULL_NAME_COPY: u32 = 0x0040_46b0;
const STRING_DESTROY: u32 = 0x0040_37d0;
const COMPARE_CASE: u32 = 0x0040_4dc0;
const COMPARE_NO_CASE: u32 = 0x0040_8b20;
const SET_STRING: u32 = 0x0048_9100;
/// `EffectSetting` flag test `00403e60(effect, mask)` (bool).
const TEST_EFFECT_FLAGS: u32 = 0x0040_3e60;
/// `00810fd0(effect)`: `data.eArchetype`.
const GET_ARCHETYPE: u32 = 0x0081_0fd0;
/// Table of 16-byte archetype entries whose first word is the name
/// (entry 0: "Value Modifier").
const ARCHETYPE_NAMES: u32 = 0x0118_3320;
/// CRT floating-point helpers used by `fabs`.
const CTRLFP: u32 = 0x00ed_5abb;
const SPTYPE: u32 = 0x00ed_5981;
const HANDLE_QNAN1: u32 = 0x00ed_5837;
const EXCEPT1: u32 = 0x00ed_588c;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;

/// Warning formats.
const MSG_COUNTER_EFFECT: u32 = 0x0101_2970;
const MSG_QUEUED_LOADED: u32 = 0x0101_2c18;
const MSG_ASSOCIATED_ITEM: u32 = 0x0101_2bc8;
const MSG_LIGHT: u32 = 0x0101_2b88;
const MSG_CAST_SOUND: u32 = 0x0101_2b40;
const MSG_BOLT_SOUND: u32 = 0x0101_2af8;
const MSG_HIT_SOUND: u32 = 0x0101_2ab0;
const MSG_AREA_SOUND: u32 = 0x0101_2a68;
const MSG_EFFECT_SHADER: u32 = 0x0101_2a20;
const MSG_ENCHANT_SHADER: u32 = 0x0101_29d0;

/// The getters and setters of the form references `InitItem` resolves and
/// `004082f0` compares (thiscall on the effect).
const GET_LIGHT: u32 = 0x004f_d380;
const SET_LIGHT: u32 = 0x0057_bd60;
const GET_CAST_SOUND: u32 = 0x004f_b070;
const SET_CAST_SOUND: u32 = 0x008a_c890;
const GET_BOLT_SOUND: u32 = 0x004f_d3c0;
const SET_BOLT_SOUND: u32 = 0x004f_b090;
const GET_HIT_SOUND: u32 = 0x004f_d3e0;
const SET_HIT_SOUND: u32 = 0x0085_7230;
const GET_AREA_SOUND: u32 = 0x008c_e390;
const SET_AREA_SOUND: u32 = 0x008c_c6d0;
const GET_EFFECT_SHADER: u32 = 0x0040_7820;
const SET_EFFECT_SHADER: u32 = 0x0044_2a80;
const GET_ENCHANT_SHADER: u32 = 0x0040_7840;
const SET_ENCHANT_SHADER: u32 = 0x004f_ebd0;

/// A signalling `float` NaN made quiet, as `FLD` delivers it.
fn quiet_f32(v: f32) -> f32 {
    let b = v.to_bits();
    if b & 0x7f80_0000 == 0x7f80_0000 && b & 0x007f_ffff != 0 {
        f32::from_bits(b | 0x0040_0000)
    } else {
        v
    }
}

/// A signalling `double` NaN made quiet.
fn quiet_f64(v: f64) -> f64 {
    let b = v.to_bits();
    if b & 0x7ff0_0000_0000_0000 == 0x7ff0_0000_0000_0000 && b & 0x000f_ffff_ffff_ffff != 0 {
        f64::from_bits(b | 0x0008_0000_0000_0000)
    } else {
        v
    }
}

/// `FLD float` then `FSTP double`: exact, NaN payload kept and quieted.
fn widen(v: f32) -> f64 {
    if v.is_nan() {
        let b = quiet_f32(v).to_bits() as u64;
        f64::from_bits(
            ((b & 0x8000_0000) << 32) | 0x7ff0_0000_0000_0000 | ((b & 0x007f_ffff) << 29),
        )
    } else {
        v as f64
    }
}

/// `FSTP float` of a `double`: rounded to nearest; a NaN keeps the top of
/// its payload and is quiet.
fn narrow(v: f64) -> f32 {
    if v.is_nan() {
        let b = quiet_f64(v).to_bits();
        f32::from_bits(
            ((b >> 32) as u32 & 0x8000_0000) | 0x7f80_0000 | ((b >> 29) as u32 & 0x007f_ffff),
        )
    } else {
        v as f32
    }
}

/// `this + offset`, or null when `this` is null (the code's base-class
/// conversion).
fn component(this: Ptr<EffectSetting>, offset: u32) -> Ptr {
    if this.is_null() {
        Ptr::NULL
    } else {
        this.byte_add(offset)
    }
}

/// The form id of `form`, or 0 for a null pointer (`Save`'s conversion).
fn form_id_or_zero(e: &mut Engine, form: u32) -> u32 {
    if form == 0 {
        0
    } else {
        e.call(GET_FORM_ID, &args![form]).u32()
    }
}

// Translated from 00407300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectSetting::Save` (Xbox PDB): writes the base components, then the
/// `DATA` chunk with the form pointers replaced by their form ids (and
/// endian-swapped around the write when needed), restores the pointers,
/// and writes one `ESCE` chunk per counter effect.
pub fn effect_setting_save(e: &mut Engine, this: Ptr<EffectSetting>) {
    e.call(START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![this.byte_add(FULL_NAME)]);
    e.call(DESCRIPTION_SAVE, &args![this.byte_add(DESCRIPTION)]);
    e.call(TEXTURE_SAVE, &args![this.byte_add(ICON), SIG_ICON]);
    e.call(MODEL_SAVE, &args![this.byte_add(MODEL), SIG_MODL, SIG_MODT]);

    let fields = [
        EffectSetting::pCastSound,
        EffectSetting::pBoltSound,
        EffectSetting::pHitSound,
        EffectSetting::pAreaSound,
        EffectSetting::pEffectShader,
        EffectSetting::pEnchantEffect,
        EffectSetting::pLight,
        EffectSetting::pAssociatedItem,
    ];
    let mut saved = [0u32; 8];
    for (slot, field) in saved.iter_mut().zip(fields) {
        *slot = e.get(this, field);
        let id = form_id_or_zero(e, *slot);
        e.set(this, field, id);
    }

    let data = this.byte_add(DATA).cast::<EffectSettingData>();
    if e.call(SAVE_NEEDS_SWAP, &[]).bool() {
        effect_setting_effect_setting_data_endian(e, data);
    }
    e.call(ADD_CHUNK_DATA, &args![SIG_DATA, data, 0x48u32]);
    if e.call(SAVE_NEEDS_SWAP, &[]).bool() {
        effect_setting_effect_setting_data_endian(e, data);
    }
    for (value, field) in saved.into_iter().zip(fields) {
        e.set(this, field, value);
    }

    let mut node = this.byte_add(COUNTER_EFFECTS).addr();
    while node != 0 {
        let item_at = e.call(NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(item_at);
        if item == 0 {
            break;
        }
        let id = e.call(GET_FORM_ID, &args![item]).u32();
        e.call(ADD_CHUNK_U32, &args![SIG_ESCE, id]);
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
    e.call(CLOSE_FORM, &args![this]);
}

// Translated from 004078c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectSetting::EffectSettingData::Endian` (Xbox PDB): swaps every
/// field's bytes in place, in the code's order (+0x00, +0x04, +0x44, then
/// +0x08 to +0x40; +0x14 is the 16-bit counter count).
pub fn effect_setting_effect_setting_data_endian(e: &mut Engine, this: Ptr<EffectSettingData>) {
    for offset in [0x00u32, 0x04, 0x44, 0x08, 0x0c, 0x10] {
        e.call(SWAP_U32, &args![this.byte_add(offset), false]);
    }
    fn_00407a90(e, this.byte_add(0x14), 0);
    for offset in (0x18u32..=0x40).step_by(4) {
        e.call(SWAP_U32, &args![this.byte_add(offset), false]);
    }
}

// Translated from 00407a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Endian swap of a 16-bit value in place (the 16-bit instance of the swap
/// template; its callers pass a second word, a `bool`, that it never reads).
pub fn fn_00407a90(e: &mut Engine, value: Ptr, _unused_1: u32) {
    let old = e.mem.u16(value.addr());
    let swapped = e.call(SWAP_U16_VALUE, &args![old as u32]).u16();
    e.mem.set_u16(value.addr(), swapped);
}

// Translated from 00407ac0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00407ac0 - the /GS cookie check (00ec408c) is not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding is not modelled)
/// `EffectSetting::Load` (Xbox PDB): false unless the file's current form
/// is type 0x10; otherwise loads the form header and reads every chunk:
/// `ESCE` (a counter effect's form id, resolved and cast to an
/// `EffectSetting`; the count is stored in `iNumCounterEffects`, a
/// warning if it is not one), `EDID` (through vtable slot 0x134), `DATA`
/// (cleared, speed 1.0, read, swapped if needed, flag 0x200000 cleared),
/// `DESC`, `OBND` (vtable slot 0xE0), `ICON`, `MODL`/`MODT` and `FULL` (the
/// components' loaders). Returns true.
pub fn effect_setting_load(e: &mut Engine, this: Ptr<EffectSetting>, file: Ptr) -> bool {
    if e.call(GET_TES_FORM, &args![file]).u32() != 0x10 {
        return false;
    }
    e.call(LOAD_FORM, &args![this, file]);
    loop {
        let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            SIG_ESCE => load_counter_effect(e, this, file),
            SIG_EDID => {
                let size = e.call(CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size, |e, buffer| {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, 0x200u32]);
                    e.vcall(this.addr(), 0x134, &args![buffer]);
                });
            }
            SIG_DATA => {
                let data = this.byte_add(DATA);
                e.call(MEMSET, &args![data, 0u32, 0x48u32]);
                e.set(this, EffectSetting::fSpeed, 1.0);
                e.call(GET_CHUNK_DATA_SIZED, &args![file, data, 0x48u32]);
                if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
                    effect_setting_effect_setting_data_endian(e, data.cast());
                }
                fn_00407e00(e, this, 0x0020_0000, false);
            }
            SIG_DESC => {
                e.call(DESCRIPTION_LOAD, &args![component(this, DESCRIPTION), file]);
            }
            SIG_OBND => {
                e.vcall(this.addr(), 0xe0, &args![file]);
            }
            SIG_ICON => {
                e.call(TEXTURE_LOAD, &args![component(this, ICON), file]);
            }
            SIG_MODL | SIG_MODT => {
                e.call(MODEL_LOAD, &args![component(this, MODEL), file]);
            }
            SIG_FULL => {
                e.call(FULL_NAME_LOAD, &args![component(this, FULL_NAME), file]);
            }
            _ => {}
        }
        if !e.call(NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    true
}

/// `Load`'s `ESCE` case: the form id local and the cast result local are
/// on the game's stack (both passed by address).
fn load_counter_effect(e: &mut Engine, this: Ptr<EffectSetting>, file: Ptr) {
    e.with_stack(8, |e, locals| {
        let id_at = locals;
        let effect_at = locals.byte_add(4);
        e.call(GET_CHUNK_DATA, &args![file, id_at]);
        if e.call(FILE_NEEDS_SWAP, &args![file]).bool() {
            e.call(SWAP_U32, &args![id_at, false]);
        }
        e.call(ADD_COMPILE_INDEX, &args![id_at, file]);
        let id = e.mem.u32(id_at.addr());
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let effect = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, RTTI_TES_FORM, RTTI_EFFECT_SETTING, 0u32],
            )
            .u32();
        e.mem.set_u32(effect_at.addr(), effect);
        if effect != 0 {
            let list = this.byte_add(COUNTER_EFFECTS);
            e.call(LIST_ADD, &args![list, effect_at]);
            let count = e.call(LIST_COUNT, &args![list]).u16();
            e.set(this, EffectSetting::iNumCounterEffects, count as i16);
        } else {
            let name = e.vcall(this.addr(), 0x130, &[]).u32();
            let id = e.mem.u32(id_at.addr());
            e.call(WARNING, &args![MSG_COUNTER_EFFECT, id, name]);
        }
    });
}

// Translated from 00407e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`set`) or clears the bits `mask` of `data.iFlags`.
pub fn fn_00407e00(e: &mut Engine, this: Ptr<EffectSetting>, mask: u32, set: bool) {
    let flags = e.get(this, EffectSetting::iFlags);
    let flags = if set { flags | mask } else { !mask & flags };
    e.set(this, EffectSetting::iFlags, flags);
}

/// One reference `InitItem` resolves: the getter's form id is fixed up for
/// the effect's file, looked up, cast to `target` and given to the setter;
/// a warning if the getter then returns null.
fn resolve_reference(
    e: &mut Engine,
    this: Ptr<EffectSetting>,
    getter: u32,
    setter: u32,
    target: u32,
    message: u32,
) {
    let id = e.call(getter, &args![this]).u32();
    if id == 0 {
        return;
    }
    e.with_stack(4, |e, id_at| {
        e.mem.set_u32(id_at.addr(), id);
        let file = e.call(GET_FILE, &args![this, -1i32]).u32();
        e.call(ADD_COMPILE_INDEX, &args![id_at, file]);
        let id = e.mem.u32(id_at.addr());
        let form = e.call(LOOKUP_FORM, &args![id]).u32();
        let cast = e
            .call(
                RT_DYNAMIC_CAST,
                &args![form, 0u32, RTTI_TES_FORM, target, 0u32],
            )
            .u32();
        e.call(setter, &args![this, cast]);
        if e.call(getter, &args![this]).u32() == 0 {
            let form_id = e.call(GET_FORM_ID, &args![this]).u32();
            let name = e.vcall(this.addr(), 0x130, &[]).u32();
            let id = e.mem.u32(id_at.addr());
            e.call(WARNING, &args![message, id, name, form_id]);
        }
    });
}

// Translated from 00407e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectSetting::InitItem` (Xbox PDB): unless `004013e0` says so, turns
/// the loaded form ids into pointers: the associated item (cleared unless
/// `004064b0` holds and the form flags 0x180000 are clear; looked up
/// without a cast), the light, the cast, bolt, hit and area sounds and the
/// two effect shaders, each with a warning when it is not found; then
/// `00484ab0(this, 1)`.
pub fn effect_setting_init_item(e: &mut Engine, this: Ptr<EffectSetting>) {
    if e.call(0x0040_13e0, &args![this]).bool() {
        return;
    }
    if !e.call(0x0040_64b0, &args![this]).bool()
        || e.call(TEST_FORM_FLAGS, &args![this, 0x0018_0000u32]).bool()
    {
        e.set(this, EffectSetting::pAssociatedItem, 0);
    } else {
        let id = e.get(this, EffectSetting::pAssociatedItem);
        if id != 0 {
            e.with_stack(4, |e, id_at| {
                e.mem.set_u32(id_at.addr(), id);
                let file = e.call(GET_FILE, &args![this, -1i32]).u32();
                e.call(ADD_COMPILE_INDEX, &args![id_at, file]);
                let id = e.mem.u32(id_at.addr());
                let form = e.call(LOOKUP_FORM, &args![id]).u32();
                e.set(this, EffectSetting::pAssociatedItem, form);
                if e.get(this, EffectSetting::pAssociatedItem) == 0 {
                    let form_id = e.call(GET_FORM_ID, &args![this]).u32();
                    let name = e.vcall(this.addr(), 0x130, &[]).u32();
                    let id = e.mem.u32(id_at.addr());
                    e.call(WARNING, &args![MSG_ASSOCIATED_ITEM, id, name, form_id]);
                }
            });
        }
    }
    let references = [
        (GET_LIGHT, SET_LIGHT, RTTI_LIGHT, MSG_LIGHT),
        (GET_CAST_SOUND, SET_CAST_SOUND, RTTI_SOUND, MSG_CAST_SOUND),
        (GET_BOLT_SOUND, SET_BOLT_SOUND, RTTI_SOUND, MSG_BOLT_SOUND),
        (GET_HIT_SOUND, SET_HIT_SOUND, RTTI_SOUND, MSG_HIT_SOUND),
        (GET_AREA_SOUND, SET_AREA_SOUND, RTTI_SOUND, MSG_AREA_SOUND),
        (
            GET_EFFECT_SHADER,
            SET_EFFECT_SHADER,
            RTTI_EFFECT_SHADER,
            MSG_EFFECT_SHADER,
        ),
        (
            GET_ENCHANT_SHADER,
            SET_ENCHANT_SHADER,
            RTTI_EFFECT_SHADER,
            MSG_ENCHANT_SHADER,
        ),
    ];
    for (getter, setter, target, message) in references {
        resolve_reference(e, this, getter, setter, target, message);
    }
    e.call(INIT_ITEM_DONE, &args![this, 1u32]);
}

// Translated from 004082f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `other` differs from this effect: true unless it is an
/// `EffectSetting` whose components compare equal (`00485270` false) and
/// whose 16 getters (`006286d0`, the float `00639b20`, `00586150`,
/// `005f5f80`, `0070ec90`, `008d8520`, the light, both shaders, the float
/// `009b0bd0`, the four sounds, the counter count and the archetype) return
/// the same values, with the same counter effects in the same order (the
/// shorter list ends the walk). Floats compare equal only when ordered and
/// equal (NaN differs).
pub fn fn_004082f0(e: &mut Engine, this: Ptr<EffectSetting>, other: Ptr) -> bool {
    let effect = e
        .call(
            RT_DYNAMIC_CAST,
            &args![other, 0u32, RTTI_TES_FORM, RTTI_EFFECT_SETTING, 0u32],
        )
        .ptr::<EffectSetting>();
    if effect.is_null() {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, other]).bool() {
        return true;
    }
    enum Getter {
        Word(u32),
        Float(u32),
        Counters,
    }
    let getters = [
        Getter::Word(0x0062_86d0),
        Getter::Float(0x0063_9b20),
        Getter::Word(0x0058_6150),
        Getter::Word(0x005f_5f80),
        Getter::Word(0x0070_ec90),
        Getter::Word(0x008d_8520),
        Getter::Word(GET_LIGHT),
        Getter::Word(GET_EFFECT_SHADER),
        Getter::Word(GET_ENCHANT_SHADER),
        Getter::Float(0x009b_0bd0),
        Getter::Word(GET_CAST_SOUND),
        Getter::Word(GET_BOLT_SOUND),
        Getter::Word(GET_HIT_SOUND),
        Getter::Word(GET_AREA_SOUND),
        Getter::Counters,
        Getter::Word(GET_ARCHETYPE),
    ];
    for getter in getters {
        let same = match getter {
            Getter::Word(addr) => {
                let mine = e.call(addr, &args![this]).u32();
                mine == e.call(addr, &args![effect]).u32()
            }
            Getter::Float(addr) => {
                let mine = e.call(addr, &args![this]).f64();
                let theirs = e.call(addr, &args![effect]).f64();
                mine == theirs
            }
            Getter::Counters => fn_00408550(e, this) == fn_00408550(e, effect),
        };
        if !same {
            return true;
        }
    }
    let mut mine = this.byte_add(COUNTER_EFFECTS).addr();
    let mut theirs = effect.byte_add(COUNTER_EFFECTS).addr();
    while mine != 0 && theirs != 0 {
        let at = e.call(NODE_ITEM, &args![mine]).u32();
        let my_item = e.mem.u32(at);
        let at = e.call(NODE_ITEM, &args![theirs]).u32();
        let their_item = e.mem.u32(at);
        if my_item != their_item {
            return true;
        }
        mine = e.call(NODE_NEXT, &args![mine]).u32();
        theirs = e.call(NODE_NEXT, &args![theirs]).u32();
    }
    false
}

// Translated from 00408550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `data.iNumCounterEffects`, sign-extended.
pub fn fn_00408550(e: &mut Engine, this: Ptr<EffectSetting>) -> i32 {
    e.get(this, EffectSetting::iNumCounterEffects) as i32
}

// Translated from 00408570 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00408570 - C++ exception frame (FS:[0], state for the temporary string) not translated
/// Sets the full name (the `TESFullName` string at +0x38, through
/// `00489100`) to `name` when it differs from a copy of the current one
/// (`StrCmp` with flag 1); the copy is a stack `BSStringT` destroyed by
/// `004037d0`.
pub fn fn_00408570(e: &mut Engine, this: Ptr<EffectSetting>, name: Ptr) {
    let differs = e.with_stack(8, |e, copy| {
        let returned = e.call(GET_FULL_NAME_COPY, &args![this, copy]).ptr::<()>();
        let result = bs_string_t_char_str_cmp(e, returned, name, true);
        e.call(STRING_DESTROY, &args![copy]);
        result
    });
    if differs != 0 {
        e.call(SET_STRING, &args![this.byte_add(FULL_NAME), name]);
    }
}

// Translated from 00408600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the counter effects with the items of the list starting at
/// `node` (up to the first null item) and counts them in
/// `iNumCounterEffects`.
pub fn fn_00408600(e: &mut Engine, this: Ptr<EffectSetting>, node: Ptr) {
    let list = this.byte_add(COUNTER_EFFECTS);
    e.call(LIST_CLEAR, &args![list]);
    e.set(this, EffectSetting::iNumCounterEffects, 0);
    let mut node = node.addr();
    while node != 0 {
        let at = e.call(NODE_ITEM, &args![node]).u32();
        let item = e.mem.u32(at);
        if item == 0 {
            break;
        }
        e.with_stack(4, |e, item_at| {
            e.mem.set_u32(item_at.addr(), item);
            e.call(LIST_ADD, &args![list, item_at]);
        });
        let count = e.get(this, EffectSetting::iNumCounterEffects);
        e.set(
            this,
            EffectSetting::iNumCounterEffects,
            count.wrapping_add(1),
        );
        node = e.call(NODE_NEXT, &args![node]).u32();
    }
}

// Translated from 00408680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether every bit of `mask` is set in `data.iFlags`.
pub fn fn_00408680(e: &mut Engine, this: Ptr<EffectSetting>, mask: u32) -> bool {
    e.get(this, EffectSetting::iFlags) & mask == mask
}

// Translated from 004086a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// 2 when flag 0x2000000 is set, plus 1 when flag 0x4000000 is set
/// (through `00403e60`).
pub fn fn_004086a0(e: &mut Engine, this: Ptr<EffectSetting>) -> u32 {
    let high = if e
        .call(TEST_EFFECT_FLAGS, &args![this, 0x0200_0000u32])
        .bool()
    {
        2
    } else {
        0
    };
    let low = e
        .call(TEST_EFFECT_FLAGS, &args![this, 0x0400_0000u32])
        .bool() as u32;
    low.wrapping_add(high)
}

/// The three counter updates shared by both counters.
fn count_away_from_zero(
    e: &mut Engine,
    this: Ptr<EffectSetting>,
    field: Field<EffectSetting, i32>,
) -> i32 {
    let count = e.get(this, field);
    let count = if count < 0 {
        count.wrapping_sub(1)
    } else {
        count.wrapping_add(1)
    };
    e.set(this, field, count);
    e.get(this, field)
}

fn count_toward_zero(
    e: &mut Engine,
    this: Ptr<EffectSetting>,
    field: Field<EffectSetting, i32>,
) -> i32 {
    let count = e.get(this, field);
    if count < 0 {
        e.set(this, field, count.wrapping_add(1));
    } else if count > 0 {
        e.set(this, field, count.wrapping_sub(1));
    }
    e.get(this, field)
}

fn count_queue(e: &mut Engine, this: Ptr<EffectSetting>, field: Field<EffectSetting, i32>) -> i32 {
    let count = e.get(this, field);
    if count > 0 {
        e.call(WARNING, &args![MSG_QUEUED_LOADED]);
    } else {
        e.set(this, field, count.wrapping_sub(1));
    }
    e.get(this, field)
}

fn count_abs(e: &mut Engine, this: Ptr<EffectSetting>, field: Field<EffectSetting, i32>) -> i32 {
    let count = e.get(this, field);
    let value = fn_00408820(e, count as f32);
    let count = e.call(FTOL, &args![widen(value)]).i32();
    e.set(this, field, count);
    e.get(this, field)
}

// Translated from 004086e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `iAssociatedItemLoadedCount` one away from zero (a queued, negative
/// count down, otherwise up) and returns it.
pub fn fn_004086e0(e: &mut Engine, this: Ptr<EffectSetting>) -> i32 {
    count_away_from_zero(e, this, EffectSetting::iAssociatedItemLoadedCount)
}

// Translated from 00408730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `iAssociatedItemLoadedCount` one toward zero and returns it.
pub fn fn_00408730(e: &mut Engine, this: Ptr<EffectSetting>) -> i32 {
    count_toward_zero(e, this, EffectSetting::iAssociatedItemLoadedCount)
}

// Translated from 00408790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues the associated item (count down) unless it is loaded (count
/// above zero: the warning "Trying to Queue up an Effect Associated Item
/// which is already loaded"); returns the count.
pub fn fn_00408790(e: &mut Engine, this: Ptr<EffectSetting>) -> i32 {
    count_queue(e, this, EffectSetting::iAssociatedItemLoadedCount)
}

// Translated from 004087e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes `iAssociatedItemLoadedCount` its absolute value (through the
/// `float` absolute value and `_ftol2`) and returns it.
pub fn fn_004087e0(e: &mut Engine, this: Ptr<EffectSetting>) -> i32 {
    count_abs(e, this, EffectSetting::iAssociatedItemLoadedCount)
}

// Translated from 00408820 (decompiled, FalloutNV.exe 1.4.0.525)
/// The absolute value of a `float` (forwards to `00408840`).
pub fn fn_00408820(e: &mut Engine, value: f32) -> f32 {
    fn_00408840(e, quiet_f32(value))
}

// Translated from 00408840 (decompiled, FalloutNV.exe 1.4.0.525)
/// The absolute value of a `float` (forwards to `00408860`).
pub fn fn_00408840(e: &mut Engine, value: f32) -> f32 {
    fn_00408860(e, quiet_f32(value))
}

// Translated from 00408860 (decompiled, FalloutNV.exe 1.4.0.525)
/// The absolute value of a `float`: `fabs` of it as a `double`, stored as
/// a `float`.
pub fn fn_00408860(e: &mut Engine, value: f32) -> f32 {
    narrow(fabs(e, widen(value)))
}

// Translated from 00408880 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectSetting::IsAssociatedItemLoaded` (Xbox PDB): the count is
/// positive.
pub fn effect_setting_is_associated_item_loaded(e: &mut Engine, this: Ptr<EffectSetting>) -> bool {
    e.get(this, EffectSetting::iAssociatedItemLoadedCount) > 0
}

// Translated from 004088a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectSetting::IsAssociatedItemQueued` (Xbox PDB): the count is
/// negative.
pub fn effect_setting_is_associated_item_queued(e: &mut Engine, this: Ptr<EffectSetting>) -> bool {
    e.get(this, EffectSetting::iAssociatedItemLoadedCount) < 0
}

// Translated from 004088c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `iEffectLoadedCount` one away from zero and returns it.
pub fn fn_004088c0(e: &mut Engine, this: Ptr<EffectSetting>) -> i32 {
    count_away_from_zero(e, this, EffectSetting::iEffectLoadedCount)
}

// Translated from 00408910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Moves `iEffectLoadedCount` one toward zero and returns it.
pub fn fn_00408910(e: &mut Engine, this: Ptr<EffectSetting>) -> i32 {
    count_toward_zero(e, this, EffectSetting::iEffectLoadedCount)
}

// Translated from 00408970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues the effect (count down) unless it is loaded (the same warning as
/// `00408790`); returns the count.
pub fn fn_00408970(e: &mut Engine, this: Ptr<EffectSetting>) -> i32 {
    count_queue(e, this, EffectSetting::iEffectLoadedCount)
}

// Translated from 004089c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes `iEffectLoadedCount` its absolute value and returns it.
pub fn fn_004089c0(e: &mut Engine, this: Ptr<EffectSetting>) -> i32 {
    count_abs(e, this, EffectSetting::iEffectLoadedCount)
}

// Translated from 00408a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectSetting::IsEffectLoaded` (Xbox PDB): the count is positive.
pub fn effect_setting_is_effect_loaded(e: &mut Engine, this: Ptr<EffectSetting>) -> bool {
    e.get(this, EffectSetting::iEffectLoadedCount) > 0
}

// Translated from 00408a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `EffectSetting::IsEffectQueued` (Xbox PDB): the count is negative.
pub fn effect_setting_is_effect_queued(e: &mut Engine, this: Ptr<EffectSetting>) -> bool {
    e.get(this, EffectSetting::iEffectLoadedCount) < 0
}

// Translated from 00408a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The name of the effect's archetype (`00810fd0`, then `00408a60`).
pub fn fn_00408a40(e: &mut Engine, this: Ptr<EffectSetting>) -> Ptr {
    let archetype = e.call(GET_ARCHETYPE, &args![this]).u32();
    fn_00408a60(e, archetype)
}

// Translated from 00408a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The name of archetype `index`: the first word of its 16-byte entry in
/// the table at `01183320`.
pub fn fn_00408a60(e: &mut Engine, index: u32) -> Ptr {
    Ptr::new(e.mem.u32(ARCHETYPE_NAMES.wrapping_add(index << 4)))
}

// Translated from 00408a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSStringT<char>::StrCmp` (Xbox PDB): compares the string with `other`
/// (`00404dc0` when `flag` is set, `00408b20` otherwise); with a null side
/// it is 0 when both are null, 1 when only `other` is, -1 otherwise.
pub fn bs_string_t_char_str_cmp(e: &mut Engine, this: Ptr, other: Ptr, flag: bool) -> i32 {
    if !other.is_null() && e.call(STRING_C_STR, &args![this]).u32() != 0 {
        let text = e.call(STRING_C_STR, &args![this]).u32();
        let compare = if flag { COMPARE_CASE } else { COMPARE_NO_CASE };
        return e.call(compare, &args![text, other]).i32();
    }
    if other.is_null() && e.call(STRING_C_STR, &args![this]).u32() == 0 {
        return 0;
    }
    if other.is_null() {
        1
    } else {
        -1
    }
}

// Translated from 00ec6cde (decompiled, FalloutNV.exe 1.4.0.525)
/// `fabs` (CRT): masks all x87 exceptions (`__ctrlfp(0x133f, 0xffff)`); a
/// finite value loses its sign bit and the control word is restored; for an
/// infinity or NaN `__sptype` decides: +inf as is, -inf negated (both after
/// restoring), a quiet NaN through `__handle_qnan1(0x15, x, cw)`, anything
/// else through `__except1(8, 0x15, x, x + 1, cw)`.
pub fn fabs(e: &mut Engine, x: f64) -> f64 {
    let saved = e.call(CTRLFP, &args![0x133fu32, 0xffffu32]).u32();
    let high = (x.to_bits() >> 48) as u32;
    if high & 0x7ff0 != 0x7ff0 {
        let result = f64::from_bits(x.to_bits() & !(1u64 << 63));
        e.call(CTRLFP, &args![saved, 0xffffu32]);
        return result;
    }
    let loaded = quiet_f64(x);
    match e.call(SPTYPE, &args![loaded]).i32() {
        1 => {
            e.call(CTRLFP, &args![saved, 0xffffu32]);
            loaded
        }
        2 => {
            e.call(CTRLFP, &args![saved, 0xffffu32]);
            -loaded
        }
        3 => e.call(HANDLE_QNAN1, &args![0x15u32, loaded, saved]).f64(),
        _ => {
            let one: f64 = e.global(ONE);
            let plus_one = loaded + one;
            e.call(EXCEPT1, &args![8u32, 0x15u32, loaded, plus_one, saved])
                .f64()
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00407300, effect_setting_save(Ptr<EffectSetting>)),
        entry!(
            0x004078c0,
            effect_setting_effect_setting_data_endian(Ptr<EffectSettingData>)
        ),
        entry!(0x00407a90, fn_00407a90(Ptr, u32)),
        entry!(
            0x00407ac0,
            effect_setting_load(Ptr<EffectSetting>, Ptr) -> bool
        ),
        entry!(0x00407e00, fn_00407e00(Ptr<EffectSetting>, u32, bool)),
        entry!(0x00407e50, effect_setting_init_item(Ptr<EffectSetting>)),
        entry!(0x004082f0, fn_004082f0(Ptr<EffectSetting>, Ptr) -> bool),
        entry!(0x00408550, fn_00408550(Ptr<EffectSetting>) -> i32),
        entry!(0x00408570, fn_00408570(Ptr<EffectSetting>, Ptr)),
        entry!(0x00408600, fn_00408600(Ptr<EffectSetting>, Ptr)),
        entry!(0x00408680, fn_00408680(Ptr<EffectSetting>, u32) -> bool),
        entry!(0x004086a0, fn_004086a0(Ptr<EffectSetting>) -> u32),
        entry!(0x004086e0, fn_004086e0(Ptr<EffectSetting>) -> i32),
        entry!(0x00408730, fn_00408730(Ptr<EffectSetting>) -> i32),
        entry!(0x00408790, fn_00408790(Ptr<EffectSetting>) -> i32),
        entry!(0x004087e0, fn_004087e0(Ptr<EffectSetting>) -> i32),
        entry!(0x00408820, fn_00408820(f32) -> f32),
        entry!(0x00408840, fn_00408840(f32) -> f32),
        entry!(0x00408860, fn_00408860(f32) -> f32),
        entry!(
            0x00408880,
            effect_setting_is_associated_item_loaded(Ptr<EffectSetting>) -> bool
        ),
        entry!(
            0x004088a0,
            effect_setting_is_associated_item_queued(Ptr<EffectSetting>) -> bool
        ),
        entry!(0x004088c0, fn_004088c0(Ptr<EffectSetting>) -> i32),
        entry!(0x00408910, fn_00408910(Ptr<EffectSetting>) -> i32),
        entry!(0x00408970, fn_00408970(Ptr<EffectSetting>) -> i32),
        entry!(0x004089c0, fn_004089c0(Ptr<EffectSetting>) -> i32),
        entry!(
            0x00408a00,
            effect_setting_is_effect_loaded(Ptr<EffectSetting>) -> bool
        ),
        entry!(
            0x00408a20,
            effect_setting_is_effect_queued(Ptr<EffectSetting>) -> bool
        ),
        entry!(0x00408a40, fn_00408a40(Ptr<EffectSetting>) -> Ptr),
        entry!(0x00408a60, fn_00408a60(u32) -> Ptr),
        entry!(0x00408a80, bs_string_t_char_str_cmp(Ptr, Ptr, bool) -> i32),
        entry!(0x00ec6cde, fabs(f64) -> f64),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Records = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Fake addresses of the virtual functions the tests install.
    const V_NAME: u32 = 0x0900_0130;
    const V_EDITOR_ID: u32 = 0x0900_0134;
    const V_OBJECT_BOUND: u32 = 0x0900_00e0;
    const NAME: u32 = 0x0abc_0000;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn none(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An effect with a vtable holding the three virtual functions used.
    fn effect(e: &mut Engine) -> Ptr<EffectSetting> {
        let this: Ptr<EffectSetting> = e.new_object();
        let vtable = e.mem.alloc(0x140);
        e.mem.set_u32(vtable + 0x130, V_NAME);
        e.mem.set_u32(vtable + 0x134, V_EDITOR_ID);
        e.mem.set_u32(vtable + 0xe0, V_OBJECT_BOUND);
        e.mem.set_u32(this.addr(), vtable);
        e.register(V_NAME, |_, _| ret(NAME));
        e.register(V_EDITOR_ID, none);
        e.register(V_OBJECT_BOUND, none);
        this
    }

    /// A two-word list node `[item, next]`; the doubles below read it so.
    fn node(e: &mut Engine, at: u32, item: u32, next: u32) {
        e.mem.set_u32(at, item);
        e.mem.set_u32(at + 4, next);
    }

    fn list_doubles(e: &mut Engine) {
        e.register(NODE_ITEM, |_, a| ret(a[0]));
        e.register(NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
    }

    fn recorder(e: &mut Engine, addr: u32, result: u32) -> Records {
        let records: Records = Rc::default();
        let r = records.clone();
        e.register_double(addr, move |_, a| {
            r.borrow_mut().push(a.to_vec());
            ret(result)
        });
        records
    }

    #[test]
    fn save_writes_form_ids_and_restores_the_pointers() {
        let mut e = Engine::new();
        let this = effect(&mut e);
        for addr in [
            START_FORM,
            FULL_NAME_SAVE,
            DESCRIPTION_SAVE,
            TEXTURE_SAVE,
            MODEL_SAVE,
            CLOSE_FORM,
            SWAP_U32,
        ] {
            e.register(addr, none);
        }
        e.register(SAVE_NEEDS_SWAP, |_, _| ret(1));
        e.register(SWAP_U16_VALUE, |_, a| ret(a[0]));
        e.register(GET_FORM_ID, |_, a| ret(a[0] + 1));
        list_doubles(&mut e);
        let written: Records = Rc::default();
        let w = written.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            // The cast sound and light as written (data +0x28, +0x18).
            w.borrow_mut().push(vec![
                a[0],
                a[2],
                e.mem.u32(a[1] + 0x28),
                e.mem.u32(a[1] + 0x18),
            ]);
            Ret::default()
        });
        let ids = recorder(&mut e, ADD_CHUNK_U32, 0);
        e.set(this, EffectSetting::pCastSound, 0x7000);
        e.set(this, EffectSetting::pLight, 0);
        e.set(this, EffectSetting::pAssociatedItem, 0x7100);
        let second = e.mem.alloc(8);
        node(&mut e, this.addr() + 0xa0, 0x5000, second);
        node(&mut e, second, 0x6000, 0);
        e.call_log = Some(vec![]);
        e.call(0x0040_7300, &args![this]);

        assert_eq!(written.borrow()[0], vec![SIG_DATA, 0x48, 0x7001, 0]);
        assert_eq!(e.get(this, EffectSetting::pCastSound), 0x7000);
        assert_eq!(e.get(this, EffectSetting::pAssociatedItem), 0x7100);
        assert_eq!(
            *ids.borrow(),
            vec![vec![SIG_ESCE, 0x5001], vec![SIG_ESCE, 0x6001]]
        );
        // Two fields plus two counter effects; the endian swap ran twice.
        assert_eq!(calls(&e, GET_FORM_ID).len(), 4);
        assert_eq!(calls(&e, SWAP_U32).len(), 2 * 17);
        assert_eq!(
            calls(&e, TEXTURE_SAVE),
            vec![vec![this.addr() + 0x44, SIG_ICON]]
        );
        assert_eq!(
            calls(&e, MODEL_SAVE),
            vec![vec![this.addr() + 0x18, SIG_MODL, SIG_MODT]]
        );
        assert_eq!(calls(&e, CLOSE_FORM), vec![vec![this.addr()]]);
    }

    #[test]
    fn endian_swaps_every_field_in_order() {
        let mut e = Engine::new();
        let data = e.mem.alloc(0x48);
        e.mem.set_u16(data + 0x14, 0x1234);
        e.register(SWAP_U32, none);
        e.register(
            SWAP_U16_VALUE,
            |_, a| ret((a[0] as u16).swap_bytes() as u32),
        );
        e.call_log = Some(vec![]);
        e.call(0x0040_78c0, &args![data]);
        let offsets: Vec<u32> = calls(&e, SWAP_U32).iter().map(|w| w[0] - data).collect();
        let mut expected = vec![0x00, 0x04, 0x44, 0x08, 0x0c, 0x10];
        expected.extend((0x18..=0x40).step_by(4));
        assert_eq!(offsets, expected);
        assert!(calls(&e, SWAP_U32).iter().all(|w| w[1] == 0));
        assert_eq!(e.mem.u16(data + 0x14), 0x3412);
    }

    #[test]
    fn swap_u16_stores_the_swapped_value() {
        let mut e = Engine::new();
        let value = e.mem.alloc(4);
        e.mem.set_u32(value, 0xffff_abcd);
        e.register(SWAP_U16_VALUE, |_, a| {
            ret(0x5555_0000 | (a[0] as u16).swap_bytes() as u32)
        });
        e.call_log = Some(vec![]);
        e.call(0x0040_7a90, &args![value, 1u32]);
        assert_eq!(calls(&e, SWAP_U16_VALUE), vec![vec![0xabcd]]);
        assert_eq!(e.mem.u32(value), 0xffff_cdab);
    }

    #[test]
    fn load_reads_every_chunk() {
        let mut e = Engine::new();
        let this = effect(&mut e);
        let file = Ptr::<()>::new(0x0f00_0000);
        e.register(GET_TES_FORM, |_, _| ret(0x10));
        let mut chunks = vec![
            SIG_ESCE, SIG_ESCE, SIG_EDID, SIG_DATA, SIG_DESC, SIG_OBND, SIG_ICON, SIG_MODL,
            SIG_MODT, SIG_FULL, 0x1234, 0,
        ]
        .into_iter();
        e.register_double(GET_TES_CHUNK, move |_, _| ret(chunks.next().unwrap()));
        e.register(NEXT_CHUNK, |_, _| ret(1));
        e.register(GET_CHUNK_DATA, |e, a| {
            e.mem.set_u32(a[1], 0x77);
            Ret::default()
        });
        e.register(FILE_NEEDS_SWAP, |_, _| ret(1));
        e.register(LOOKUP_FORM, |_, a| ret(a[0] + 0x1000));
        let mut casts = vec![0x9000u32, 0].into_iter();
        e.register_double(RT_DYNAMIC_CAST, move |_, _| ret(casts.next().unwrap()));
        let added = Rc::new(RefCell::new(vec![]));
        let a2 = added.clone();
        e.register_double(LIST_ADD, move |e, a| {
            a2.borrow_mut().push(e.mem.u32(a[1]));
            Ret::default()
        });
        e.register(LIST_COUNT, |_, _| ret(3));
        e.register(CHUNK_SIZE, |_, _| ret(0x10));
        for addr in [
            LOAD_FORM,
            SWAP_U32,
            ADD_COMPILE_INDEX,
            WARNING,
            GET_CHUNK_DATA_SIZED,
            MEMSET,
            DESCRIPTION_LOAD,
            TEXTURE_LOAD,
            MODEL_LOAD,
            FULL_NAME_LOAD,
        ] {
            e.register(addr, none);
        }
        // The 16-bit swap keeps the value, so the counter count survives.
        e.register(SWAP_U16_VALUE, |_, a| ret(a[0]));
        e.set(this, EffectSetting::iFlags, 0x0020_0001);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0040_7ac0, &args![this, file]).bool());

        assert_eq!(*added.borrow(), vec![0x9000]);
        assert_eq!(e.get(this, EffectSetting::iNumCounterEffects), 3);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST)[0],
            vec![0x1077, 0, RTTI_TES_FORM, RTTI_EFFECT_SETTING, 0]
        );
        assert_eq!(
            calls(&e, WARNING),
            vec![vec![MSG_COUNTER_EFFECT, 0x77, NAME]]
        );
        let sized = calls(&e, GET_CHUNK_DATA_SIZED);
        assert_eq!(sized[0][2], 0x200);
        assert_eq!(calls(&e, V_EDITOR_ID), vec![vec![this.addr(), sized[0][1]]]);
        assert_eq!(sized[1], vec![file.addr(), this.addr() + 0x58, 0x48]);
        assert_eq!(calls(&e, MEMSET), vec![vec![this.addr() + 0x58, 0, 0x48]]);
        assert_eq!(e.get(this, EffectSetting::fSpeed), 1.0);
        assert_eq!(e.get(this, EffectSetting::iFlags), 1);
        // DATA swapped: 17 words plus the two ESCE ids.
        assert_eq!(calls(&e, SWAP_U32).len(), 17 + 2);
        assert_eq!(
            calls(&e, V_OBJECT_BOUND),
            vec![vec![this.addr(), file.addr()]]
        );
        assert_eq!(
            calls(&e, DESCRIPTION_LOAD),
            vec![vec![this.addr() + 0x30, file.addr()]]
        );
        assert_eq!(
            calls(&e, TEXTURE_LOAD),
            vec![vec![this.addr() + 0x44, file.addr()]]
        );
        assert_eq!(calls(&e, MODEL_LOAD).len(), 2);
        assert_eq!(
            calls(&e, FULL_NAME_LOAD),
            vec![vec![this.addr() + 0x38, file.addr()]]
        );

        // Not an effect record: false, nothing loaded.
        e.register(GET_TES_FORM, |_, _| ret(0x11));
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0040_7ac0, &args![this, file]).bool());
        assert!(calls(&e, LOAD_FORM).is_empty());

        // No next chunk (AL is 0, only the upper bytes set): one chunk read.
        e.register(GET_TES_FORM, |_, _| ret(0x10));
        e.register(GET_TES_CHUNK, |_, _| ret(0x1234));
        e.register(NEXT_CHUNK, |_, _| ret(0x100));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0040_7ac0, &args![this, file]).bool());
        assert_eq!(calls(&e, GET_TES_CHUNK).len(), 1);
    }

    #[test]
    fn set_flags_sets_or_clears() {
        let mut e = Engine::new();
        let this = effect(&mut e);
        e.set(this, EffectSetting::iFlags, 0x11);
        e.call(0x0040_7e00, &args![this, 0x0300u32, true]);
        assert_eq!(e.get(this, EffectSetting::iFlags), 0x311);
        e.call(0x0040_7e00, &args![this, 0x0101u32, false]);
        assert_eq!(e.get(this, EffectSetting::iFlags), 0x210);
    }

    #[test]
    fn init_item_resolves_the_references() {
        let mut e = Engine::new();
        let this = effect(&mut e);
        // Each getter/setter pair keeps its value in a word of `store`.
        let store = e.mem.alloc(32);
        let pairs = [
            (GET_LIGHT, SET_LIGHT),
            (GET_CAST_SOUND, SET_CAST_SOUND),
            (GET_BOLT_SOUND, SET_BOLT_SOUND),
            (GET_HIT_SOUND, SET_HIT_SOUND),
            (GET_AREA_SOUND, SET_AREA_SOUND),
            (GET_EFFECT_SHADER, SET_EFFECT_SHADER),
            (GET_ENCHANT_SHADER, SET_ENCHANT_SHADER),
        ];
        for (i, (getter, setter)) in pairs.into_iter().enumerate() {
            let slot = store + 4 * i as u32;
            e.register_double(getter, move |e, _| ret(e.mem.u32(slot)));
            e.register_double(setter, move |e, a| {
                e.mem.set_u32(slot, a[1]);
                Ret::default()
            });
        }
        e.mem.set_u32(store, 0x11); // light
        e.mem.set_u32(store + 4, 0x22); // cast sound
        e.register(0x0040_13e0, |_, _| ret(0));
        e.register(0x0040_64b0, |_, _| ret(1));
        e.register(TEST_FORM_FLAGS, |_, _| ret(0));
        e.register(GET_FILE, |_, _| ret(0x900));
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id | 0x0100_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| {
            ret(if a[0] == 0x0100_0042 {
                0
            } else {
                a[0] + 0x1000
            })
        });
        e.register(RT_DYNAMIC_CAST, |_, a| {
            ret(if a[3] == RTTI_SOUND { 0 } else { a[0] })
        });
        e.register(GET_FORM_ID, |_, _| ret(0xf0f0));
        e.register(WARNING, none);
        e.register(INIT_ITEM_DONE, none);
        e.set(this, EffectSetting::pAssociatedItem, 0x42);
        e.call_log = Some(vec![]);
        e.call(0x0040_7e50, &args![this]);

        assert_eq!(e.mem.u32(store), 0x0100_1011);
        assert_eq!(e.mem.u32(store + 4), 0);
        assert_eq!(e.get(this, EffectSetting::pAssociatedItem), 0);
        assert_eq!(
            calls(&e, WARNING),
            vec![
                vec![MSG_ASSOCIATED_ITEM, 0x0100_0042, NAME, 0xf0f0],
                vec![MSG_CAST_SOUND, 0x0100_0022, NAME, 0xf0f0],
            ]
        );
        assert_eq!(calls(&e, GET_FILE)[0], vec![this.addr(), u32::MAX]);
        assert_eq!(
            calls(&e, RT_DYNAMIC_CAST)[0],
            vec![0x0100_1011, 0, RTTI_TES_FORM, RTTI_LIGHT, 0]
        );
        assert_eq!(calls(&e, RT_DYNAMIC_CAST).len(), 2);
        assert_eq!(calls(&e, INIT_ITEM_DONE), vec![vec![this.addr(), 1]]);

        // Flag 0x180000 set: the associated item is cleared, not looked up.
        e.register(TEST_FORM_FLAGS, |_, _| ret(1));
        e.set(this, EffectSetting::pAssociatedItem, 0x42);
        e.mem.set_u32(store, 0);
        e.call_log = Some(vec![]);
        e.call(0x0040_7e50, &args![this]);
        assert_eq!(e.get(this, EffectSetting::pAssociatedItem), 0);
        assert!(calls(&e, LOOKUP_FORM).is_empty());

        // Already initialized: nothing happens.
        e.register(0x0040_13e0, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0040_7e50, &args![this]);
        assert!(calls(&e, INIT_ITEM_DONE).is_empty());
    }

    fn compare_engine() -> (Engine, Ptr<EffectSetting>, Ptr<EffectSetting>) {
        let mut e = Engine::new();
        let this = effect(&mut e);
        let other = effect(&mut e);
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(0));
        for addr in [
            0x0062_86d0,
            0x0058_6150,
            0x005f_5f80,
            0x0070_ec90,
            0x008d_8520,
            GET_LIGHT,
            GET_EFFECT_SHADER,
            GET_ENCHANT_SHADER,
            GET_CAST_SOUND,
            GET_BOLT_SOUND,
            GET_HIT_SOUND,
            GET_AREA_SOUND,
            GET_ARCHETYPE,
        ] {
            e.register(addr, |_, _| ret(5));
        }
        for addr in [0x0063_9b20, 0x009b_0bd0] {
            e.register(addr, |_, _| 1.5f32.into_ret());
        }
        list_doubles(&mut e);
        for effect in [this, other] {
            let second = e.mem.alloc(8);
            node(&mut e, effect.addr() + 0xa0, 0x5000, second);
            node(&mut e, second, 0x6000, 0);
            e.set(effect, EffectSetting::iNumCounterEffects, 2);
        }
        (e, this, other)
    }

    #[test]
    fn compare_finds_differences() {
        let (mut e, this, other) = compare_engine();
        assert!(!e.call(0x0040_82f0, &args![this, other]).bool());

        // The other list is shorter: still equal.
        let (mut e2, this2, other2) = compare_engine();
        e2.mem.set_u32(other2.addr() + 0xa4, 0);
        assert!(!e2.call(0x0040_82f0, &args![this2, other2]).bool());

        // A different counter effect.
        let second = e.mem.u32(other.addr() + 0xa4);
        e.mem.set_u32(second, 0x6001);
        assert!(e.call(0x0040_82f0, &args![this, other]).bool());

        // A different getter value, a NaN float, a different count.
        let (mut e, this, other) = compare_engine();
        e.register(0x0058_6150, |_, a| ret(a[0]));
        assert!(e.call(0x0040_82f0, &args![this, other]).bool());
        let (mut e, this, other) = compare_engine();
        e.register(0x009b_0bd0, |_, _| f64::NAN.into_ret());
        assert!(e.call(0x0040_82f0, &args![this, other]).bool());
        let (mut e, this, other) = compare_engine();
        e.set(other, EffectSetting::iNumCounterEffects, 3);
        assert!(e.call(0x0040_82f0, &args![this, other]).bool());

        // Components differ, or not an effect.
        let (mut e, this, other) = compare_engine();
        e.register(COMPARE_ALL_COMPONENTS, |_, _| ret(1));
        assert!(e.call(0x0040_82f0, &args![this, other]).bool());
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0040_82f0, &args![this, other]).bool());
        assert!(calls(&e, COMPARE_ALL_COMPONENTS).is_empty());
    }

    #[test]
    fn counter_count_is_sign_extended() {
        let mut e = Engine::new();
        let this = effect(&mut e);
        e.set(this, EffectSetting::iNumCounterEffects, -2);
        assert_eq!(e.call(0x0040_8550, &args![this]).i32(), -2);
    }

    #[test]
    fn set_full_name_only_when_different() {
        let mut e = Engine::new();
        let this = effect(&mut e);
        let name = Ptr::<()>::new(0x0123_4560);
        e.register(GET_FULL_NAME_COPY, |_, a| ret(a[1]));
        e.register(STRING_C_STR, |_, _| ret(0x500));
        e.register(STRING_DESTROY, none);
        e.register(SET_STRING, none);
        e.register(COMPARE_CASE, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0040_8570, &args![this, name]);
        let copy = calls(&e, GET_FULL_NAME_COPY)[0][1];
        assert_eq!(calls(&e, COMPARE_CASE), vec![vec![0x500, name.addr()]]);
        assert_eq!(calls(&e, STRING_DESTROY), vec![vec![copy]]);
        assert_eq!(
            calls(&e, SET_STRING),
            vec![vec![this.addr() + 0x38, name.addr()]]
        );

        e.register(COMPARE_CASE, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0040_8570, &args![this, name]);
        assert!(calls(&e, SET_STRING).is_empty());
    }

    #[test]
    fn copy_counter_effects_stops_at_a_null_item() {
        let mut e = Engine::new();
        let this = effect(&mut e);
        list_doubles(&mut e);
        e.register(LIST_CLEAR, none);
        let added = Rc::new(RefCell::new(vec![]));
        let a2 = added.clone();
        e.register_double(LIST_ADD, move |e, a| {
            a2.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let (first, second, third) = (e.mem.alloc(8), e.mem.alloc(8), e.mem.alloc(8));
        node(&mut e, first, 0x5000, second);
        node(&mut e, second, 0x6000, third);
        node(&mut e, third, 0, 0);
        e.set(this, EffectSetting::iNumCounterEffects, 9);
        e.call_log = Some(vec![]);
        e.call(0x0040_8600, &args![this, first]);
        let list = this.addr() + 0xa0;
        assert_eq!(*added.borrow(), vec![(list, 0x5000), (list, 0x6000)]);
        assert_eq!(e.get(this, EffectSetting::iNumCounterEffects), 2);
        assert_eq!(calls(&e, LIST_CLEAR), vec![vec![list]]);
    }

    #[test]
    fn has_all_flags() {
        let mut e = Engine::new();
        let this = effect(&mut e);
        e.set(this, EffectSetting::iFlags, 0x0006);
        assert!(e.call(0x0040_8680, &args![this, 0x0004u32]).bool());
        assert!(!e.call(0x0040_8680, &args![this, 0x0005u32]).bool());
    }

    #[test]
    fn flag_pair_value() {
        let mut e = Engine::new();
        let this = effect(&mut e);
        e.register(TEST_EFFECT_FLAGS, |e, a| {
            ret((e.mem.u32(a[0] + 0x58) & a[1] != 0) as u32)
        });
        for (flags, expected) in [(0, 0), (0x0200_0000, 2), (0x0400_0000, 1), (0x0600_0000, 3)] {
            e.set(this, EffectSetting::iFlags, flags);
            assert_eq!(e.call(0x0040_86a0, &args![this]).u32(), expected);
        }
    }

    fn abs_doubles(e: &mut Engine) {
        e.register(CTRLFP, |_, _| ret(0x027f));
        e.register(FTOL, |_, a| ret(f64::take(a, &mut 0) as i32 as u32));
    }

    /// The four functions of one counter: away from zero, toward zero,
    /// queue, absolute value.
    fn check_counter(addrs: [u32; 4], field: Field<EffectSetting, i32>) {
        let mut e = Engine::new();
        let this = effect(&mut e);
        abs_doubles(&mut e);
        let warnings = recorder(&mut e, WARNING, 0);
        let [away, toward, queue, abs] = addrs;
        for (start, expected) in [(-2, -3), (0, 1), (4, 5)] {
            e.set(this, field, start);
            assert_eq!(e.call(away, &args![this]).i32(), expected);
        }
        for (start, expected) in [(-2, -1), (0, 0), (4, 3)] {
            e.set(this, field, start);
            assert_eq!(e.call(toward, &args![this]).i32(), expected);
        }
        for (start, expected) in [(-2, -3), (0, -1)] {
            e.set(this, field, start);
            assert_eq!(e.call(queue, &args![this]).i32(), expected);
        }
        assert!(warnings.borrow().is_empty());
        e.set(this, field, 4);
        assert_eq!(e.call(queue, &args![this]).i32(), 4);
        assert_eq!(*warnings.borrow(), vec![vec![MSG_QUEUED_LOADED]]);
        for (start, expected) in [(-7, 7), (7, 7), (0, 0)] {
            e.set(this, field, start);
            assert_eq!(e.call(abs, &args![this]).i32(), expected);
        }
    }

    #[test]
    fn associated_item_counter() {
        check_counter(
            [0x0040_86e0, 0x0040_8730, 0x0040_8790, 0x0040_87e0],
            EffectSetting::iAssociatedItemLoadedCount,
        );
    }

    #[test]
    fn effect_counter() {
        check_counter(
            [0x0040_88c0, 0x0040_8910, 0x0040_8970, 0x0040_89c0],
            EffectSetting::iEffectLoadedCount,
        );
    }

    #[test]
    fn float_absolute_value_chain() {
        let mut e = Engine::new();
        abs_doubles(&mut e);
        for addr in [0x0040_8820, 0x0040_8840, 0x0040_8860] {
            assert_eq!(e.call(addr, &args![-2.5f32]).f32(), 2.5);
            assert_eq!(e.call(addr, &args![3.0f32]).f32(), 3.0);
        }
        // A signalling NaN comes back quiet with its payload (through
        // __sptype, here saying "quiet NaN", and __handle_qnan1).
        e.register(SPTYPE, |_, _| ret(3));
        e.register(HANDLE_QNAN1, |_, a| f64::take(&a[1..], &mut 0).into_ret());
        let result = e.call(0x0040_8820, &args![f32::from_bits(0x7f80_0001)]);
        assert_eq!(narrow(result.st0).to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn loaded_and_queued_tests() {
        let mut e = Engine::new();
        let this = effect(&mut e);
        let cases = [
            (
                0x0040_8880,
                EffectSetting::iAssociatedItemLoadedCount,
                [false, false, true],
            ),
            (
                0x0040_88a0,
                EffectSetting::iAssociatedItemLoadedCount,
                [true, false, false],
            ),
            (
                0x0040_8a00,
                EffectSetting::iEffectLoadedCount,
                [false, false, true],
            ),
            (
                0x0040_8a20,
                EffectSetting::iEffectLoadedCount,
                [true, false, false],
            ),
        ];
        for (addr, field, expected) in cases {
            for (value, want) in [-1, 0, 1].into_iter().zip(expected) {
                e.set(this, field, value);
                assert_eq!(e.call(addr, &args![this]).bool(), want);
            }
        }
    }

    #[test]
    fn archetype_name_from_the_table() {
        let mut e = Engine::new();
        let this = effect(&mut e);
        e.map(ARCHETYPE_NAMES, 0x40);
        e.mem.set_u32(ARCHETYPE_NAMES, 0x0101_1ff0);
        e.mem.set_u32(ARCHETYPE_NAMES + 0x20, 0x0101_2000);
        assert_eq!(e.call(0x0040_8a60, &args![0u32]).u32(), 0x0101_1ff0);
        assert_eq!(e.call(0x0040_8a60, &args![2u32]).u32(), 0x0101_2000);
        e.register(GET_ARCHETYPE, |_, _| ret(2));
        assert_eq!(e.call(0x0040_8a40, &args![this]).u32(), 0x0101_2000);
    }

    #[test]
    fn str_cmp_handles_null_sides() {
        let mut e = Engine::new();
        let string = Ptr::<()>::new(0x0200_0000);
        let other = Ptr::<()>::new(0x0300_0000);
        e.register(COMPARE_CASE, |_, _| ret(7));
        e.register(COMPARE_NO_CASE, |_, _| ret(-7i32 as u32));
        e.register(STRING_C_STR, |_, _| ret(0x400));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0040_8a80, &args![string, other, true]).i32(), 7);
        assert_eq!(calls(&e, COMPARE_CASE), vec![vec![0x400, other.addr()]]);
        assert_eq!(e.call(0x0040_8a80, &args![string, other, false]).i32(), -7);
        assert_eq!(
            e.call(0x0040_8a80, &args![string, Ptr::<()>::NULL, true])
                .i32(),
            1
        );
        e.register(STRING_C_STR, |_, _| ret(0));
        assert_eq!(
            e.call(0x0040_8a80, &args![string, Ptr::<()>::NULL, true])
                .i32(),
            0
        );
        assert_eq!(e.call(0x0040_8a80, &args![string, other, true]).i32(), -1);
    }

    #[test]
    fn fabs_cases() {
        let mut e = Engine::new();
        e.register(CTRLFP, |_, _| ret(0x027f));
        e.map(ONE & !0xfff, 0x1000);
        e.set_global(ONE, 1.0f64);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_6cde, &args![-0.75f64]).f64(), 0.75);
        assert_eq!(
            calls(&e, CTRLFP),
            vec![vec![0x133f, 0xffff], vec![0x027f, 0xffff]]
        );

        e.register(SPTYPE, |_, a| {
            let x = f64::take(a, &mut 0);
            ret(if x == f64::INFINITY {
                1
            } else if x == f64::NEG_INFINITY {
                2
            } else {
                4
            })
        });
        assert_eq!(
            e.call(0x00ec_6cde, &args![f64::INFINITY]).f64(),
            f64::INFINITY
        );
        assert_eq!(
            e.call(0x00ec_6cde, &args![f64::NEG_INFINITY]).f64(),
            f64::INFINITY
        );

        // Neither infinity nor quiet NaN (as __sptype says): __except1.
        let nan = f64::from_bits(0xfff8_0000_0000_0002);
        e.register(EXCEPT1, |_, _| 9.0f64.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_6cde, &args![nan]).f64(), 9.0);
        let call = &calls(&e, EXCEPT1)[0];
        assert_eq!(&call[..2], &[8, 0x15]);
        assert_eq!(f64::take(&call[2..], &mut 0).to_bits(), nan.to_bits());
        assert!(f64::take(&call[4..], &mut 0).is_nan());
        assert_eq!(call[6], 0x027f);

        // Quiet NaN: __handle_qnan1(0x15, x, saved control word).
        e.register(SPTYPE, |_, _| ret(3));
        e.register(HANDLE_QNAN1, |_, _| 4.0f64.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ec_6cde, &args![nan]).f64(), 4.0);
        let call = &calls(&e, HANDLE_QNAN1)[0];
        assert_eq!((call[0], call[3]), (0x15, 0x027f));
        assert_eq!(calls(&e, CTRLFP).len(), 1);
    }

    // Moved from the CRT's tests (`units/crt.rs`) with the engine split: it runs this
    // unit's `fabs`, which the core's own tests no longer link.
    #[test]
    fn fabs_restores_the_control_word_through_ctrlfp() {
        use crate::units::crt::{ctrlfp, x87_control_word};
        let mut e = Engine::new();
        ctrlfp(&mut e, 0x037f, 0xffff);
        assert_eq!(e.call(0x00ec6cde, &args![-0.75f64]).f64(), 0.75);
        assert_eq!(x87_control_word(&mut e), 0x037f);
    }
}
