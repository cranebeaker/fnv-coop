//! `fallout shared/tesaiform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The function `fn_0047ede0` calls on the member (`0067e400`).
const CALLEE_0067E400: u32 = 0x0067_e400;

// Translated from 0047ede0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Named `TESAIForm::ClearDataComponent` (Xbox PDB name in the engine map). Thin wrapper: calls `0067e400` with `ECX` = `this + 0x18` and passes on nothing (the callee returns no value).
pub fn fn_0047ede0(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_0067E400, &args![this.byte_add(0x18)]);
}

/// The four byte setters of the AI data (`0047ee00`, `0047eed0`, `0047ef00`, `0047ef50`):
/// store `value` at `offset` and, when `notify` is not zero, run `0047f790` with 8 on the same
/// object.
fn set_ai_byte(e: &mut Engine, this: Ptr, offset: u32, value: u8, notify: u8) {
    e.mem.set_u8(this.addr() + offset, value);
    if notify != 0 {
        e.call(0x0047_f790, &args![this, 8u32]);
    }
}

// Translated from 0047ee00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0x4` of the AI data (the part of `TESAIForm` at `+0x90` of an actor base)
/// and, when `notify` is not zero, runs `0047f790` with 8. The engine map gives it no name.
pub fn fn_0047ee00(e: &mut Engine, this: Ptr, value: u8, notify: u8) {
    set_ai_byte(e, this, 4, value, notify);
}

// Translated from 0047eed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0x5` of the AI data and, when `notify` is not zero, runs `0047f790`
/// with 8. The engine map gives it no name.
pub fn fn_0047eed0(e: &mut Engine, this: Ptr, value: u8, notify: u8) {
    set_ai_byte(e, this, 5, value, notify);
}

// Translated from 0047ef00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0x6` of the AI data and, when `notify` is not zero, runs `0047f790`
/// with 8. The engine map gives it no name.
pub fn fn_0047ef00(e: &mut Engine, this: Ptr, value: u8, notify: u8) {
    set_ai_byte(e, this, 6, value, notify);
}

// Translated from 0047ef50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the byte at `+0x7` of the AI data and, when `notify` is not zero, runs `0047f790`
/// with 8. The engine map gives it no name.
pub fn fn_0047ef50(e: &mut Engine, this: Ptr, value: u8, notify: u8) {
    set_ai_byte(e, this, 7, value, notify);
}

// Translated from 0047f060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `value` is in `0x20..=0x2d` (signed compare). The engine map gives it no name.
pub fn fn_0047f060(_e: &mut Engine, value: i32) -> bool {
    (0x20..0x2e).contains(&value)
}

/// `TESAIForm`'s vtable (RTTI `TESAIForm`).
const TES_AI_FORM_VTABLE: u32 = 0x0101_be98;
/// `00500940` (`ECX` = form): the address of the package list (`this +
/// 0x18`, `AIPackList`, Xbox PDB).
const PACKAGE_LIST: u32 = 0x0050_0940;
/// `004181e0` (`ECX` = reference): the base form whose `TESAIForm` (at
/// +0x90) the two static functions use.
const REFERENCE_BASE: u32 = 0x0041_81e0;
/// `00403e20` (`ECX` = setting): the address of the setting's `float`.
const SETTING_FLOAT: u32 = 0x0040_3e20;
/// The guard byte of the five copied settings (set once they are copied).
const SETTINGS_COPIED: u32 = 0x011c_52c0;
/// The five (setting, copy) pairs `InitializeDataComponent` fills once.
const COPIED_SETTINGS: [(u32, u32); 5] = [
    (0x011c_533c, 0x011c_52a8),
    (0x011c_5320, 0x011c_52ac),
    (0x011c_52fc, 0x011c_52b0),
    (0x011c_530c, 0x011c_52b4),
    (0x011c_52e4, 0x011c_52b8),
];

// Translated from 0047ee30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cAssistance` (Xbox PDB `AIDATA`; +0x12).
pub fn fn_0047ee30(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0x12)
}

// Translated from 0047ef30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cResponsibility` (Xbox PDB `AIDATA`; +0x7).
pub fn fn_0047ef30(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 7)
}

// Translated from 0047f790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks the owning form changed: the form is `this - 0x90` (the
/// `TESAIForm` at +0x90 of an actor base; none for a null `this`), and its
/// vtable slot 0x48 (`AddChange` in the Xbox PDB's `TESForm` vtable) gets
/// `flags`.
pub fn fn_0047f790(e: &mut Engine, this: Ptr, flags: u32) {
    let owner = if this.is_null() {
        0
    } else {
        this.addr().wrapping_sub(0x90)
    };
    if owner != 0 {
        e.vcall(owner, 0x48, &args![flags]);
    }
}

// Translated from 0047ee50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `cAssistance` (+0x12) and marks the form changed (`0047f790`, 8).
pub fn fn_0047ee50(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x12, value);
    fn_0047f790(e, this, 8);
}

// Translated from 0047ee70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `bAggroRadius` (+0x13) and marks the form changed (`0047f790`, 8).
pub fn fn_0047ee70(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x13, value);
    fn_0047f790(e, this, 8);
}

// Translated from 0047ee90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `uiAggroRadius` (+0x14) and marks the form changed (`0047f790`,
/// 8).
pub fn fn_0047ee90(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x14, value);
    fn_0047f790(e, this, 8);
}

// Translated from 0047ef80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `cMood` (+0x8) when `value` is below 8, and then marks the form
/// changed (`0047f790`, 8); a larger value changes nothing.
pub fn fn_0047ef80(e: &mut Engine, this: Ptr, value: u8) {
    if i32::from(value) < 8 {
        e.mem.set_u8(this.addr() + 8, value);
        fn_0047f790(e, this, 8);
    }
}

// Translated from 0047efb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `iServiceFlags` (+0xC) and, when `notify` is not zero, marks the
/// form changed (`0047f790`, 8).
pub fn fn_0047efb0(e: &mut Engine, this: Ptr, flags: u32, notify: u8) {
    e.mem.set_u32(this.addr() + 0xc, flags);
    if notify != 0 {
        fn_0047f790(e, this, 8);
    }
}

// Translated from 0047f000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `cTrainingSkill` (+0x10) plus 0x20 (the actor-value number).
pub fn fn_0047f000(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.mem.u8(this.addr() + 0x10)) + 0x20
}

// Translated from 0047f020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `cTrainingSkill` (+0x10) to `skill - 0x20` when `skill` is in
/// `0x20..=0x2D` (`0047f060`), then marks the form changed (`0047f790`, 8)
/// either way.
pub fn fn_0047f020(e: &mut Engine, this: Ptr, skill: u32) {
    if fn_0047f060(e, skill as i32) {
        e.mem
            .set_u8(this.addr() + 0x10, skill.wrapping_sub(0x20) as u8);
    }
    fn_0047f790(e, this, 8);
}

// Translated from 0047f090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `cTrainingLevel` (+0x11) and marks the form changed (`0047f790`,
/// 8).
pub fn fn_0047f090(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x11, value);
    fn_0047f790(e, this, 8);
}

// Translated from 0047ed10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAIForm::InitializeDataComponent` (Xbox PDB): the AI data's start
/// values (all 0 but `cConfidence` 4), and, the first time (guard byte
/// `011c52c0`), copies five settings' floats (`00403e20` on each) into
/// the globals `011c52a8`..`011c52b8`.
pub fn tes_ai_form_initialize_data_component(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u8(base + 4, 0);
    e.mem.set_u8(base + 5, 4);
    e.mem.set_u8(base + 6, 0);
    e.mem.set_u8(base + 7, 0);
    e.mem.set_u8(base + 8, 0);
    e.mem.set_u32(base + 0xc, 0);
    e.mem.set_u8(base + 0x10, 0);
    e.mem.set_u8(base + 0x11, 0);
    e.mem.set_u8(base + 0x12, 0);
    e.mem.set_u8(base + 0x13, 0);
    e.mem.set_u32(base + 0x14, 0);
    if e.global::<u8>(SETTINGS_COPIED) != 0 {
        return;
    }
    for (setting, copy) in COPIED_SETTINGS {
        let value_at = e.call(SETTING_FLOAT, &args![setting]).u32();
        let bits = e.mem.u32(value_at);
        e.set_global::<u32>(copy, x87_quiet_bits(bits));
    }
    e.set_global::<u8>(SETTINGS_COPIED, 1);
}

// Translated from 0047ec40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0047ec40 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `TESAIForm::TESAIForm` (Xbox PDB): the `BaseFormComponent` base
/// (`00402d30`), the vtable, the package list (`00680890` on +0x18) and
/// `InitializeDataComponent`. Returns `this`.
pub fn tes_ai_form_tes_ai_form(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(this.addr(), TES_AI_FORM_VTABLE);
    e.call(0x0068_0890, &args![this.addr() + 0x18]);
    tes_ai_form_initialize_data_component(e, this);
    this
}

// Translated from 0047ecb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0047ecb0 - the exception-unwinding frame (FS:[0]) and its state numbers are not translated
/// `TESAIForm`'s destructor: the vtable, `ClearDataComponent`
/// (`0047ede0`), then the package list (`0067e3a0` on +0x18).
pub fn fn_0047ecb0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), TES_AI_FORM_VTABLE);
    fn_0047ede0(e, this);
    e.call(0x0067_e3a0, &args![this.addr() + 0x18]);
}

// Translated from 0047f0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies an `AIDATA` record (`data`, the Xbox PDB layout: aggression,
/// confidence, energy, responsibility, mood, service flags at +8, training
/// skill and level at +0xC/+0xD, assistance +0xE, aggro-radius flag +0xF,
/// aggro radius +0x10) into the form through the setters (those with a
/// notify flag get 1), the training bytes directly, and clamps
/// `cConfidence` to 0 when it is 5 or more. Nothing for a null `data`.
pub fn fn_0047f0b0(e: &mut Engine, this: Ptr, data: Ptr) {
    if data.is_null() {
        return;
    }
    let d = data.addr();
    let base = this.addr();
    let byte = |e: &Engine, offset: u32| e.mem.u8(d + offset);
    let v = byte(e, 0);
    fn_0047ee00(e, this, v, 1);
    let v = byte(e, 0xe);
    fn_0047ee50(e, this, v);
    let v = byte(e, 1);
    fn_0047eed0(e, this, v, 1);
    let v = byte(e, 2);
    fn_0047ef00(e, this, v, 1);
    let v = byte(e, 3);
    fn_0047ef50(e, this, v, 1);
    let flags = e.mem.u32(d + 8);
    fn_0047efb0(e, this, flags, 1);
    let v = byte(e, 4);
    fn_0047ef80(e, this, v);
    let v = byte(e, 0xf);
    fn_0047ee70(e, this, v);
    let radius = e.mem.u32(d + 0x10);
    fn_0047ee90(e, this, radius);
    let v = byte(e, 0xc);
    e.mem.set_u8(base + 0x10, v);
    let v = byte(e, 0xd);
    e.mem.set_u8(base + 0x11, v);
    if i32::from(e.mem.u8(base + 5)) >= 5 {
        e.mem.set_u8(base + 5, 0);
    }
}

// Translated from 0047f1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the AI data of `other` (a `TESAIForm`) through its getters
/// (`004f1540`, `0047ee30`, `004f6de0`, `005bb4d0`, `0047ef30`,
/// `0062f2f0`, `00705cf0`, `00825c00`, `0084e3a0`, `0047f000`,
/// `00824060`) into this form's setters, in the exe's order; the setters
/// with a notify flag get 1.
pub fn fn_0047f1e0(e: &mut Engine, this: Ptr, other: Ptr) {
    let get = |e: &mut Engine, getter: u32| e.call(getter, &args![other]).u32();
    let v = get(e, 0x004f_1540) as u8;
    fn_0047ee00(e, this, v, 1);
    let v = fn_0047ee30(e, other);
    fn_0047ee50(e, this, v);
    let v = get(e, 0x004f_6de0) as u8;
    fn_0047eed0(e, this, v, 1);
    let v = get(e, 0x005b_b4d0) as u8;
    fn_0047ef00(e, this, v, 1);
    let v = fn_0047ef30(e, other);
    fn_0047ef50(e, this, v, 1);
    let v = get(e, 0x0062_f2f0) as u8;
    fn_0047ef80(e, this, v);
    let v = get(e, 0x0070_5cf0) as u8;
    fn_0047ee70(e, this, v);
    let v = get(e, 0x0082_5c00);
    fn_0047ee90(e, this, v);
    let v = get(e, 0x0084_e3a0);
    fn_0047efb0(e, this, v, 1);
    let v = fn_0047f000(e, other);
    fn_0047f020(e, this, v);
    let v = get(e, 0x0082_4060) as u8;
    fn_0047f090(e, this, v);
}

// Translated from 0047f2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies `other`'s package list into this form's (`0067e440` on this
/// form's list with `other`'s, both through `00500940`).
pub fn fn_0047f2d0(e: &mut Engine, this: Ptr, other: Ptr) {
    let source = e.call(PACKAGE_LIST, &args![other]).u32();
    let list = e.call(PACKAGE_LIST, &args![this]).u32();
    e.call(0x0067_e440, &args![list, source]);
}

// Translated from 0047f190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAIForm::CopyComponent` (Xbox PDB): casts `other` (a
/// `BaseFormComponent`) to a `TESAIForm` (`__RTDynamicCast`); with one,
/// copies its AI data (`0047f1e0`) and packages (`0047f2d0`).
pub fn tes_ai_form_copy_component(e: &mut Engine, this: Ptr, other: Ptr) {
    let cast = e
        .call(
            0x00ec_43fb,
            &args![other, 0u32, 0x0118_3040u32, 0x0118_6cd4u32, 0u32],
        )
        .u32();
    if cast == 0 {
        return;
    }
    fn_0047f1e0(e, this, Ptr::new(cast));
    fn_0047f2d0(e, this, Ptr::new(cast));
}

// Translated from 0047f500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds `package` to the form's package list (`0067e420` on the list from
/// `00500940`; `BSSimpleList<..>::AddTail` in the engine map).
pub fn fn_0047f500(e: &mut Engine, this: Ptr, package: u32) {
    let list = e.call(PACKAGE_LIST, &args![this]).u32();
    e.call(0x0067_e420, &args![list, package]);
}

// Translated from 0047f4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAIForm::InitItem` (Xbox PDB): unless the byte `00408d60` returns
/// for `011c5348` is set, resolves the package list
/// (`PackageList::InitItem`, `0067e560`, with `form`).
pub fn tes_ai_form_init_item(e: &mut Engine, this: Ptr, form: Ptr) {
    let flag_at = e.call(0x0040_8d60, &args![0x011c_5348u32]).u32();
    if e.mem.u8(flag_at) != 0 {
        return;
    }
    let list = e.call(PACKAGE_LIST, &args![this]).u32();
    e.call(0x0067_e560, &args![list, form]);
}

/// The `TESAIForm` of `reference`'s base form (`004181e0`, +0x90), or 0.
fn reference_ai_form(e: &mut Engine, reference: Ptr) -> u32 {
    let base = e.call(REFERENCE_BASE, &args![reference]).u32();
    if base != 0 {
        base + 0x90
    } else {
        0
    }
}

// Translated from 0047f520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAIForm::GetCurrentPackage` (Xbox PDB, `cdecl`): for a reference,
/// `PackageList::FindCurrentPackage` (`0067e780`) of its base form's
/// `TESAIForm` with the reference; 0 for a null reference or base.
pub fn tes_ai_form_get_current_package(e: &mut Engine, reference: Ptr) -> u32 {
    if reference.is_null() {
        return 0;
    }
    let ai = reference_ai_form(e, reference);
    if ai == 0 {
        return 0;
    }
    let list = e.call(PACKAGE_LIST, &args![ai]).u32();
    e.call(0x0067_e780, &args![list, reference]).u32()
}

// Translated from 0047f590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAIForm::GetMissedPackages` (Xbox PDB, `cdecl`: reference, a word,
/// two floats): `0067ec40` on the package list of the reference's base
/// form's `TESAIForm` with the reference, the word and the two floats (each
/// copied through `FLD`/`FSTP`). Nothing for a null reference or base.
pub fn tes_ai_form_get_missed_packages(
    e: &mut Engine,
    reference: Ptr,
    value: u32,
    first: f32,
    second: f32,
) {
    if reference.is_null() {
        return;
    }
    let ai = reference_ai_form(e, reference);
    if ai == 0 {
        return;
    }
    let list = e.call(PACKAGE_LIST, &args![ai]).u32();
    e.call(
        0x0067_ec40,
        &args![
            list,
            reference,
            value,
            x87_quiet_bits(first.to_bits()),
            x87_quiet_bits(second.to_bits())
        ],
    );
}

/// The five copied setting floats `InitializeDataComponent` fills
/// (`011c52a8`..`011c52b8`), read by index by `0047eeb0`.
const COPIED_SETTING_FLOATS: u32 = 0x011c_52a8;
/// `__RTDynamicCast` (`00ec43fb`).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `BaseFormComponent` type descriptor (`01183040`).
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
/// The `TESAIForm` type descriptor (`01186cd4`).
const TYPE_TES_AI_FORM: u32 = 0x0118_6cd4;
/// `00401170` (`ECX` = form): its form type.
const FORM_TYPE: u32 = 0x0040_1170;
/// `BGSEquipType::GetEquipType` (engine map name of `00479430`; cdecl: a
/// form).
const GET_EQUIP_TYPE: u32 = 0x0047_9430;
/// `0067e4a0` (`ECX` = package list, the other list; `RET 4`): nonzero
/// byte when the two package lists differ.
const PACKAGE_LISTS_DIFFER: u32 = 0x0067_e4a0;
/// The save/load game object (`011de45c`, a pointer).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `008579b0` (`ECX` = save/load game; buffer, size; `RET 8`): writes bytes.
const SAVE_WRITE: u32 = 0x0085_79b0;
/// `008579e0` (`ECX` = save/load game; buffer, size; `RET 8`): reads bytes.
const LOAD_READ: u32 = 0x0085_79e0;

// Translated from 0047eeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The copied setting float number `index` (`011c52a8` + 4 * index, loaded
/// with `FLD`), 0.0 (`FLDZ`) for an index of 5 or more (`cdecl`, one byte).
pub fn fn_0047eeb0(e: &mut Engine, index: u8) -> f32 {
    if i32::from(index) >= 5 {
        return 0.0;
    }
    let bits = e.mem.u32(COPIED_SETTING_FLOATS + 4 * u32::from(index));
    f32::from_bits(x87_quiet_bits(bits))
}

// Translated from 0047efe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `iServiceFlags` (+0xC) has any of `flags` (`RET 4`).
pub fn fn_0047efe0(e: &mut Engine, this: Ptr, flags: u32) -> bool {
    e.mem.u32(this.addr() + 0xc) & flags != 0
}

// Translated from 0047f300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAIForm::CompareComponent` (Xbox PDB; `RET 4`): casts `other` (a
/// `BaseFormComponent`) to a `TESAIForm` (`__RTDynamicCast`); true when the
/// cast fails or when any getter (`004f1540`, `0047ee30`, `004f6de0`,
/// `005bb4d0`, `0047ef30`, `0062f2f0` as bytes, `00825c00` as a word,
/// `00705cf0` as a byte, `0084e3a0` and `0047f000` as words, `00824060` as
/// a byte, in that order) gives `this` and the cast form different values,
/// or when the package lists differ (`0067e4a0` on this form's list with
/// the other's, both through `00500940`).
pub fn tes_ai_form_compare_component(e: &mut Engine, this: Ptr, other: Ptr) -> bool {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_AI_FORM,
                0u32
            ],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    let cast = Ptr::new(cast);
    let byte = |e: &mut Engine, getter: u32, form: Ptr| e.call(getter, &args![form]).u8();
    let word = |e: &mut Engine, getter: u32, form: Ptr| e.call(getter, &args![form]).u32();
    if byte(e, 0x004f_1540, this) != byte(e, 0x004f_1540, cast) {
        return true;
    }
    if fn_0047ee30(e, this) != fn_0047ee30(e, cast) {
        return true;
    }
    if byte(e, 0x004f_6de0, this) != byte(e, 0x004f_6de0, cast) {
        return true;
    }
    if byte(e, 0x005b_b4d0, this) != byte(e, 0x005b_b4d0, cast) {
        return true;
    }
    if fn_0047ef30(e, this) != fn_0047ef30(e, cast) {
        return true;
    }
    if byte(e, 0x0062_f2f0, this) != byte(e, 0x0062_f2f0, cast) {
        return true;
    }
    if word(e, 0x0082_5c00, this) != word(e, 0x0082_5c00, cast) {
        return true;
    }
    if byte(e, 0x0070_5cf0, this) != byte(e, 0x0070_5cf0, cast) {
        return true;
    }
    if word(e, 0x0084_e3a0, this) != word(e, 0x0084_e3a0, cast) {
        return true;
    }
    if fn_0047f000(e, this) != fn_0047f000(e, cast) {
        return true;
    }
    if byte(e, 0x0082_4060, this) != byte(e, 0x0082_4060, cast) {
        return true;
    }
    let theirs = e.call(PACKAGE_LIST, &args![cast]).u32();
    let mine = e.call(PACKAGE_LIST, &args![this]).u32();
    e.call(PACKAGE_LISTS_DIFFER, &args![mine, theirs]).u8() != 0
}

// Translated from 0047f600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAIForm::SellBuysItem` (Xbox PDB; the item form; `RET 4`): whether
/// the service flags (`0047efe0`) cover the item's form type (`00401170`):
/// type 0x18 needs flag 2, 0x19 flag 8, 0x1F, 0x32 and 0x67 flag 0x400,
/// 0x28 and 0x29 flag 1; for type 0x2F the flag follows the item's equip
/// type (`00479430`): 0xA flag 0x20, 0xB flag 0x40, 0xC flag 0x10, 0xD
/// flag 4. Any other type or equip type: false.
pub fn tes_ai_form_sell_buys_item(e: &mut Engine, this: Ptr, item: Ptr) -> bool {
    let kind = e.call(FORM_TYPE, &args![item]).u32();
    // The compiler's switch: a byte table (`0047f728`, indexed by type -
    // 0x18, compared as unsigned up to 0x4F) picks one of seven cases.
    match kind.wrapping_sub(0x18) {
        0 => fn_0047efe0(e, this, 2),
        1 => fn_0047efe0(e, this, 8),
        7 | 26 | 79 => fn_0047efe0(e, this, 0x400),
        16 | 17 => fn_0047efe0(e, this, 1),
        23 => {
            let equip = e.call(GET_EQUIP_TYPE, &args![item]).u32();
            match equip.wrapping_sub(0xa) {
                0 => fn_0047efe0(e, this, 0x20),
                1 => fn_0047efe0(e, this, 0x40),
                2 => fn_0047efe0(e, this, 0x10),
                3 => fn_0047efe0(e, this, 4),
                _ => false,
            }
        }
        _ => false,
    }
}

// Translated from 0047f7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAIForm::GetSaveSize` (Xbox PDB; the change flags; `RET 4`): 5 (one
/// per saved byte, summed as a `u16`) when change flag 8 is set, else 0.
pub fn tes_ai_form_get_save_size(_e: &mut Engine, _this: Ptr, flags: u32) -> u16 {
    let mut size: u16 = 0;
    if flags & 8 != 0 {
        for _ in 0..5 {
            size += 1;
        }
    }
    size
}

// Translated from 0047f840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAIForm::SaveGame` (Xbox PDB; the change flags; `RET 4`): with change
/// flag 8, writes the five bytes at +4..+8 one at a time (`008579b0` on the
/// save/load game at `011de45c`).
pub fn tes_ai_form_save_game(e: &mut Engine, this: Ptr, flags: u32) {
    if flags & 8 == 0 {
        return;
    }
    for offset in 4..9 {
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(SAVE_WRITE, &args![game, this.addr() + offset, 1u32]);
    }
}

// Translated from 0047f8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESAIForm::LoadGame` (Xbox PDB; the change flags and a word it does not
/// read; `RET 8`): with change flag 8, reads the five bytes at +4..+8 one
/// at a time (`008579e0`).
pub fn tes_ai_form_load_game(e: &mut Engine, this: Ptr, flags: u32, _unused_2: u32) {
    if flags & 8 == 0 {
        return;
    }
    for offset in 4..9 {
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(LOAD_READ, &args![game, this.addr() + offset, 1u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0047_ede0, fn_0047ede0(Ptr)),
        entry!(0x0047_ee00, fn_0047ee00(Ptr, u8, u8)),
        entry!(0x0047_eed0, fn_0047eed0(Ptr, u8, u8)),
        entry!(0x0047_ef00, fn_0047ef00(Ptr, u8, u8)),
        entry!(0x0047_ef50, fn_0047ef50(Ptr, u8, u8)),
        entry!(0x0047_f060, fn_0047f060(i32) -> bool),
        entry!(0x0047_ee30, fn_0047ee30(Ptr) -> u8),
        entry!(0x0047_ef30, fn_0047ef30(Ptr) -> u8),
        entry!(0x0047_f790, fn_0047f790(Ptr, u32)),
        entry!(0x0047_ee50, fn_0047ee50(Ptr, u8)),
        entry!(0x0047_ee70, fn_0047ee70(Ptr, u8)),
        entry!(0x0047_ee90, fn_0047ee90(Ptr, u32)),
        entry!(0x0047_ef80, fn_0047ef80(Ptr, u8)),
        entry!(0x0047_efb0, fn_0047efb0(Ptr, u32, u8)),
        entry!(0x0047_f000, fn_0047f000(Ptr) -> u32),
        entry!(0x0047_f020, fn_0047f020(Ptr, u32)),
        entry!(0x0047_f090, fn_0047f090(Ptr, u8)),
        entry!(0x0047_ed10, tes_ai_form_initialize_data_component(Ptr)),
        entry!(0x0047_ec40, tes_ai_form_tes_ai_form(Ptr) -> Ptr),
        entry!(0x0047_ecb0, fn_0047ecb0(Ptr)),
        entry!(0x0047_f0b0, fn_0047f0b0(Ptr, Ptr)),
        entry!(0x0047_f1e0, fn_0047f1e0(Ptr, Ptr)),
        entry!(0x0047_f2d0, fn_0047f2d0(Ptr, Ptr)),
        entry!(0x0047_f190, tes_ai_form_copy_component(Ptr, Ptr)),
        entry!(0x0047_f500, fn_0047f500(Ptr, u32)),
        entry!(0x0047_f4c0, tes_ai_form_init_item(Ptr, Ptr)),
        entry!(0x0047_f520, tes_ai_form_get_current_package(Ptr) -> u32),
        entry!(
            0x0047_f590,
            tes_ai_form_get_missed_packages(Ptr, u32, f32, f32)
        ),
        entry!(0x0047_eeb0, fn_0047eeb0(u8) -> f32),
        entry!(0x0047_efe0, fn_0047efe0(Ptr, u32) -> bool),
        entry!(0x0047_f300, tes_ai_form_compare_component(Ptr, Ptr) -> bool),
        entry!(0x0047_f600, tes_ai_form_sell_buys_item(Ptr, Ptr) -> bool),
        entry!(0x0047_f7e0, tes_ai_form_get_save_size(Ptr, u32) -> u16),
        entry!(0x0047_f840, tes_ai_form_save_game(Ptr, u32)),
        entry!(0x0047_f8c0, tes_ai_form_load_game(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn l68d_0047ede0_passes_this_plus_18_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_0067E400, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_ede0, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_0067E400).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x18]);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;

    /// Calls the setter at `addr` and returns the object's bytes and the logged calls.
    #[allow(clippy::type_complexity)]
    fn run(addr: u32, value: u32, notify: u32) -> (Vec<u8>, Vec<(u32, Vec<u32>)>, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xcd; 0x10]);
        e.register(0x0047_f790, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(addr, &[this, value, notify]);
        let log = e.call_log.take().unwrap();
        (e.mem.bytes(this, 0x10), log[1..].to_vec(), this)
    }

    #[test]
    fn the_four_setters_store_their_byte_and_notify_only_when_asked() {
        for (addr, offset) in [
            (0x0047_ee00, 4),
            (0x0047_eed0, 5),
            (0x0047_ef00, 6),
            (0x0047_ef50, 7),
        ] {
            let (bytes, calls, _) = run(addr, 0x1_0032, 0);
            let mut want = vec![0xcd_u8; 0x10];
            want[offset] = 0x32;
            assert_eq!(bytes, want, "{addr:08x}");
            assert!(calls.is_empty());

            let (bytes, calls, this) = run(addr, 7, 0x100_0001);
            want[offset] = 7;
            assert_eq!(bytes, want, "{addr:08x}");
            assert_eq!(calls, [(0x0047_f790, vec![this, 8])]);

            // Only the low byte of the flag word counts.
            let (_, calls, _) = run(addr, 7, 0x100);
            assert!(calls.is_empty());
        }
    }

    #[test]
    fn fn_0047f060_accepts_0x20_to_0x2d() {
        let mut e = Engine::new();
        for (value, want) in [
            (0x1f, false),
            (0x20, true),
            (0x2d, true),
            (0x2e, false),
            (-1, false),
            (i32::MIN, false),
        ] {
            assert_eq!(e.call(0x0047_f060, &[value as u32]).bool(), want, "{value}");
        }
    }
}

#[cfg(test)]
mod b020_tests {
    use super::*;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls(e: &mut Engine, skip: u32) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != skip)
            .collect()
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An actor base with its vtable's slot 0x48 at `00001048` and its
    /// `TESAIForm` at +0x90 (returned).
    fn ai_form(e: &mut Engine) -> u32 {
        e.put_vtable(0x0150_0000, &[0; 0x20]);
        e.mem.set_u32(0x0150_0048, 0x0000_1048);
        e.register(0x0000_1048, |_, _| Ret::default());
        let owner = e.mem.alloc(0xc0);
        e.mem.set_u32(owner, 0x0150_0000);
        owner + 0x90
    }

    #[test]
    fn b020_getters() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x20);
        e.mem.set_u8(p + 0x12, 0x33);
        e.mem.set_u8(p + 7, 0x44);
        e.mem.set_u8(p + 0x10, 3);
        assert_eq!(e.call(0x0047_ee30, &args![p]).u32(), 0x33);
        assert_eq!(e.call(0x0047_ef30, &args![p]).u32(), 0x44);
        assert_eq!(e.call(0x0047_f000, &args![p]).u32(), 0x23);
    }

    #[test]
    fn b020_0047f790_notifies_the_owner() {
        let mut e = Engine::new();
        let ai = ai_form(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0047_f790, &args![ai, 8u32]);
        assert_eq!(calls_to(&e, 0x0000_1048), vec![vec![ai - 0x90, 8]]);
        e.call_log = Some(vec![]);
        e.call(0x0047_f790, &args![0u32, 8u32]);
        assert!(calls_to(&e, 0x0000_1048).is_empty());
    }

    #[test]
    fn b020_setters_store_and_notify() {
        let mut e = Engine::new();
        let ai = ai_form(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x0047_ee50, &args![ai, 0x101u32]);
        e.call(0x0047_ee70, &args![ai, 1u32]);
        e.call(0x0047_ee90, &args![ai, 0x1234u32]);
        e.call(0x0047_f090, &args![ai, 7u32]);
        e.call(0x0047_efb0, &args![ai, 0x55u32, 0u32]);
        assert_eq!(e.mem.u8(ai + 0x12), 1);
        assert_eq!(e.mem.u8(ai + 0x13), 1);
        assert_eq!(e.mem.u32(ai + 0x14), 0x1234);
        assert_eq!(e.mem.u8(ai + 0x11), 7);
        assert_eq!(e.mem.u32(ai + 0xc), 0x55);
        assert_eq!(calls_to(&e, 0x0000_1048).len(), 4);
        e.call(0x0047_efb0, &args![ai, 0x66u32, 0x100u32]);
        assert_eq!(calls_to(&e, 0x0000_1048).len(), 4);
        e.call(0x0047_efb0, &args![ai, 0x66u32, 1u32]);
        assert_eq!(calls_to(&e, 0x0000_1048).len(), 5);
        // Mood: only below 8.
        e.call(0x0047_ef80, &args![ai, 7u32]);
        assert_eq!(e.mem.u8(ai + 8), 7);
        e.call(0x0047_ef80, &args![ai, 8u32]);
        assert_eq!(e.mem.u8(ai + 8), 7);
        assert_eq!(calls_to(&e, 0x0000_1048).len(), 6);
        // Training skill: only 0x20..=0x2d, notified either way.
        e.call(0x0047_f020, &args![ai, 0x2du32]);
        assert_eq!(e.mem.u8(ai + 0x10), 0xd);
        e.call(0x0047_f020, &args![ai, 0x2eu32]);
        assert_eq!(e.mem.u8(ai + 0x10), 0xd);
        assert_eq!(calls_to(&e, 0x0000_1048).len(), 8);
    }

    #[test]
    fn b020_0047ed10_sets_defaults_and_copies_settings_once() {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        for (i, (setting, _)) in COPIED_SETTINGS.iter().enumerate() {
            e.set_global::<u32>(setting + 0x10, 0x4000_0000 + i as u32);
        }
        e.set_global::<u32>(0x011c_533c + 0x10, 0x7f80_0001);
        e.register(SETTING_FLOAT, |_, a| ret(a[0] + 0x10));
        let p = e.mem.alloc(0x20);
        for o in 4..0x18 {
            e.mem.set_u8(p + o, 0xee);
        }
        e.call_log = Some(vec![]);
        e.call(0x0047_ed10, &args![p]);
        assert_eq!(e.mem.u8(p + 5), 4);
        for o in [4u32, 6, 7, 8, 0x10, 0x11, 0x12, 0x13] {
            assert_eq!(e.mem.u8(p + o), 0, "+{o:#x}");
        }
        assert_eq!(e.mem.u32(p + 0xc), 0);
        assert_eq!(e.mem.u32(p + 0x14), 0);
        assert_eq!(e.global::<u32>(0x011c_52a8), 0x7fc0_0001);
        assert_eq!(e.global::<u32>(0x011c_52b8), 0x4000_0004);
        assert_eq!(e.global::<u8>(SETTINGS_COPIED), 1);
        assert_eq!(calls_to(&e, SETTING_FLOAT).len(), 5);
        e.call_log = Some(vec![]);
        e.call(0x0047_ed10, &args![p]);
        assert!(calls_to(&e, SETTING_FLOAT).is_empty());
    }

    #[test]
    fn b020_0047ec40_constructs_and_0047ecb0_destroys() {
        let mut e = Engine::new();
        e.map(0x011c_5000, 0x1000);
        e.set_global::<u8>(SETTINGS_COPIED, 1);
        for a in [0x0040_2d30, 0x0068_0890, 0x0067_e3a0, 0x0067_e400] {
            e.register(a, |_, _| Ret::default());
        }
        let p = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_ec40, &args![p]).u32(), p);
        assert_eq!(e.mem.u32(p), TES_AI_FORM_VTABLE);
        assert_eq!(e.mem.u8(p + 5), 4);
        assert_eq!(
            calls(&mut e, 0x0047_ec40),
            vec![(0x0040_2d30, vec![p]), (0x0068_0890, vec![p + 0x18])]
        );
        e.mem.set_u32(p, 0);
        e.call_log = Some(vec![]);
        e.call(0x0047_ecb0, &args![p]);
        assert_eq!(e.mem.u32(p), TES_AI_FORM_VTABLE);
        assert_eq!(
            calls(&mut e, 0x0047_ecb0),
            vec![(0x0067_e400, vec![p + 0x18]), (0x0067_e3a0, vec![p + 0x18])]
        );
    }

    #[test]
    fn b020_0047f0b0_copies_a_record() {
        let mut e = Engine::new();
        let ai = ai_form(&mut e);
        let data = e.mem.alloc(0x14);
        let bytes = [1u8, 6, 3, 4, 9, 0, 0, 0, 0x78, 0x56, 0x34, 0x12, 5, 6, 7, 1];
        e.mem.write(data, &bytes);
        e.mem.set_u32(data + 0x10, 0x200);
        e.call_log = Some(vec![]);
        e.call(0x0047_f0b0, &args![ai, data]);
        assert_eq!(e.mem.u8(ai + 4), 1);
        assert_eq!(e.mem.u8(ai + 5), 0, "confidence 6 is clamped to 0");
        assert_eq!(e.mem.u8(ai + 6), 3);
        assert_eq!(e.mem.u8(ai + 7), 4);
        assert_eq!(e.mem.u8(ai + 8), 0, "mood 9 is refused");
        assert_eq!(e.mem.u32(ai + 0xc), 0x1234_5678);
        assert_eq!(e.mem.u8(ai + 0x10), 5);
        assert_eq!(e.mem.u8(ai + 0x11), 6);
        assert_eq!(e.mem.u8(ai + 0x12), 7);
        assert_eq!(e.mem.u8(ai + 0x13), 1);
        assert_eq!(e.mem.u32(ai + 0x14), 0x200);
        e.call_log = Some(vec![]);
        e.call(0x0047_f0b0, &args![ai, 0u32]);
        assert!(calls(&mut e, 0x0047_f0b0).is_empty());
    }

    #[test]
    fn b020_0047f190_copies_another_ai_form() {
        let mut e = Engine::new();
        let ai = ai_form(&mut e);
        let other = e.mem.alloc(0x20);
        e.mem.set_u8(other + 0x12, 9);
        e.mem.set_u8(other + 7, 2);
        e.mem.set_u8(other + 0x10, 4);
        e.register(0x00ec_43fb, |_, a| ret(a[0]));
        for (getter, value) in [
            (0x004f_1540u32, 0x101u32),
            (0x004f_6de0, 2),
            (0x005b_b4d0, 3),
            (0x0062_f2f0, 5),
            (0x0070_5cf0, 1),
            (0x0082_5c00, 0x400),
            (0x0084_e3a0, 0x77),
            (0x0082_4060, 8),
        ] {
            e.register_double(getter, move |_, _| ret(value));
        }
        e.register(PACKAGE_LIST, |_, a| ret(a[0] + 0x18));
        e.register(0x0067_e440, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0047_f190, &args![ai, other]);
        assert_eq!(
            calls_to(&e, 0x00ec_43fb),
            vec![vec![other, 0, 0x0118_3040, 0x0118_6cd4, 0]]
        );
        assert_eq!(e.mem.u8(ai + 4), 1);
        assert_eq!(e.mem.u8(ai + 0x12), 9);
        assert_eq!(e.mem.u8(ai + 5), 2);
        assert_eq!(e.mem.u8(ai + 6), 3);
        assert_eq!(e.mem.u8(ai + 7), 2);
        assert_eq!(e.mem.u8(ai + 8), 5);
        assert_eq!(e.mem.u8(ai + 0x13), 1);
        assert_eq!(e.mem.u32(ai + 0x14), 0x400);
        assert_eq!(e.mem.u32(ai + 0xc), 0x77);
        assert_eq!(e.mem.u8(ai + 0x10), 4);
        assert_eq!(e.mem.u8(ai + 0x11), 8);
        assert_eq!(
            calls_to(&e, 0x0067_e440),
            vec![vec![ai + 0x18, other + 0x18]]
        );
        e.register(0x00ec_43fb, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0047_f190, &args![ai, other]);
        assert!(calls_to(&e, 0x0067_e440).is_empty());
    }

    #[test]
    fn b020_package_list_calls() {
        let mut e = Engine::new();
        e.register(PACKAGE_LIST, |_, a| ret(a[0] + 0x18));
        e.register(0x0067_e420, |_, _| Ret::default());
        e.register(0x0067_e560, |_, _| Ret::default());
        e.map(0x011c_5000, 0x1000);
        e.register(0x0040_8d60, |_, a| ret(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x0047_f500, &args![0x3000u32, 0x55u32]);
        e.call(0x0047_f4c0, &args![0x3000u32, 0x66u32]);
        assert_eq!(calls_to(&e, 0x0067_e420), vec![vec![0x3018, 0x55]]);
        assert_eq!(calls_to(&e, 0x0067_e560), vec![vec![0x3018, 0x66]]);
        e.set_global::<u8>(0x011c_5348, 1);
        e.call(0x0047_f4c0, &args![0x3000u32, 0x66u32]);
        assert_eq!(calls_to(&e, 0x0067_e560).len(), 1);
    }

    #[test]
    fn b020_static_package_queries() {
        let mut e = Engine::new();
        e.register(REFERENCE_BASE, |_, a| {
            ret(if a[0] == 0x4444 { 0 } else { 0x5000 })
        });
        e.register(PACKAGE_LIST, |_, a| ret(a[0] + 0x18));
        e.register(0x0067_e780, |_, a| ret(a[1] + 1));
        e.register(0x0067_ec40, |_, _| Ret::default());
        assert_eq!(e.call(0x0047_f520, &args![0u32]).u32(), 0);
        assert_eq!(e.call(0x0047_f520, &args![0x4444u32]).u32(), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_f520, &args![0x6000u32]).u32(), 0x6001);
        assert_eq!(calls_to(&e, 0x0067_e780), vec![vec![0x50a8, 0x6000]]);
        e.call_log = Some(vec![]);
        e.call(0x0047_f590, &args![0x6000u32, 3u32, 1.5f32, 0x7f80_0001u32]);
        assert_eq!(
            calls_to(&e, 0x0067_ec40),
            vec![vec![0x50a8, 0x6000, 3, 1.5f32.to_bits(), 0x7fc0_0001]]
        );
        e.call(0x0047_f590, &args![0u32, 3u32, 1.5f32, 1.5f32]);
        e.call(0x0047_f590, &args![0x4444u32, 3u32, 1.5f32, 1.5f32]);
        assert_eq!(calls_to(&e, 0x0067_ec40).len(), 1);
    }
}

#[cfg(test)]
mod tests_n042 {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn run(e: &mut Engine, addr: u32, words: &[u32]) -> (Ret, Log) {
        e.call_log = Some(vec![]);
        let result = e.call(addr, words);
        let log = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != addr)
            .collect();
        (result, log)
    }

    fn calls(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn copied_setting_float_by_index() {
        let mut e = Engine::new();
        e.map(COPIED_SETTING_FLOATS, 0x14);
        for i in 0..5 {
            e.mem
                .set_u32(COPIED_SETTING_FLOATS + 4 * i, (i as f32 + 0.5).to_bits());
        }
        e.mem.set_u32(COPIED_SETTING_FLOATS + 8, 0x7f80_0001);
        assert_eq!(e.call(0x0047_eeb0, &args![1u32]).f32(), 1.5);
        assert_eq!(e.call(0x0047_eeb0, &args![4u32]).f32(), 4.5);
        assert_eq!(
            e.call(0x0047_eeb0, &args![0x102u32]).f32().to_bits(),
            0x7fc0_0001
        );
        assert_eq!(e.call(0x0047_eeb0, &args![5u32]).f32(), 0.0);
    }

    #[test]
    fn service_flag_test() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.set_u32(this + 0xc, 0x0408);
        assert!(e.call(0x0047_efe0, &args![this, 0x400u32]).bool());
        assert!(!e.call(0x0047_efe0, &args![this, 0x3u32]).bool());
    }

    const GETTERS: [u32; 8] = [
        0x004f_1540,
        0x004f_6de0,
        0x005b_b4d0,
        0x0062_f2f0,
        0x0082_5c00,
        0x0070_5cf0,
        0x0084_e3a0,
        0x0082_4060,
    ];

    fn compare_engine() -> Engine {
        let mut e = Engine::new();
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        // Each getter returns the word at +0x40 of the form (plus
        // `0x100` for the byte getters, which the compare must cut off).
        for getter in GETTERS {
            e.register(getter, |e, a| e.mem.u32(a[0] + 0x40).into_ret());
        }
        e.register(PACKAGE_LIST, |_, a| (a[0] + 0x18).into_ret());
        e.register(PACKAGE_LISTS_DIFFER, |e, a| {
            u32::from(e.mem.u32(a[0]) != e.mem.u32(a[1])).into_ret()
        });
        e
    }

    #[test]
    fn compare_component_checks_every_getter_and_the_packages() {
        let mut e = compare_engine();
        let this = e.mem.alloc(0x60);
        let other = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x40, 0x105);
        e.mem.set_u32(other + 0x40, 0x105);
        let (ret, log) = run(&mut e, 0x0047_f300, &[this, other]);
        assert!(!ret.bool());
        assert_eq!(
            calls(&log, DYNAMIC_CAST),
            vec![vec![
                other,
                0,
                TYPE_BASE_FORM_COMPONENT,
                TYPE_TES_AI_FORM,
                0
            ]]
        );
        assert_eq!(
            calls(&log, PACKAGE_LISTS_DIFFER),
            vec![vec![this + 0x18, other + 0x18]]
        );
        // The byte getters see equal bytes, the word getters differ.
        e.mem.set_u32(other + 0x40, 0x205);
        let (ret, log) = run(&mut e, 0x0047_f300, &[this, other]);
        assert!(ret.bool());
        assert_eq!(calls(&log, 0x0082_5c00).len(), 2);
        assert!(calls(&log, 0x0070_5cf0).is_empty());
        // The packages differ.
        e.mem.set_u32(other + 0x40, 0x105);
        e.mem.set_u32(other + 0x18, 1);
        assert!(e.call(0x0047_f300, &args![this, other]).bool());
        // A direct field getter differs.
        e.mem.set_u32(other + 0x18, 0);
        e.mem.set_u8(other + 0x12, 3);
        let (ret, log) = run(&mut e, 0x0047_f300, &[this, other]);
        assert!(ret.bool());
        assert!(calls(&log, 0x004f_6de0).is_empty());
        e.register(DYNAMIC_CAST, |_, _| Ret::default());
        let (ret, log) = run(&mut e, 0x0047_f300, &[this, other]);
        assert!(ret.bool());
        assert_eq!(log.len(), 1);
    }

    #[test]
    fn sell_buys_item_by_form_type_and_equip_type() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        let item = e.mem.alloc(0x40);
        e.register(FORM_TYPE, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(GET_EQUIP_TYPE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        let check = |e: &mut Engine, kind: u32, equip: u32, flags: u32| {
            e.mem.set_u32(item, kind);
            e.mem.set_u32(item + 4, equip);
            e.mem.set_u32(this + 0xc, flags);
            e.call(0x0047_f600, &args![this, item]).bool()
        };
        for (kind, flag) in [
            (0x18, 2),
            (0x19, 8),
            (0x1f, 0x400),
            (0x32, 0x400),
            (0x67, 0x400),
            (0x28, 1),
            (0x29, 1),
        ] {
            assert!(check(&mut e, kind, 0, flag), "type {kind:#x}");
            assert!(!check(&mut e, kind, 0, !flag), "type {kind:#x}");
        }
        for (equip, flag) in [(0xa, 0x20), (0xb, 0x40), (0xc, 0x10), (0xd, 4)] {
            assert!(check(&mut e, 0x2f, equip, flag));
            assert!(!check(&mut e, 0x2f, equip, !flag));
        }
        assert!(!check(&mut e, 0x2f, 0xe, !0));
        assert!(!check(&mut e, 0x2f, 9, !0));
        assert!(!check(&mut e, 0x1a, 0, !0));
        assert!(!check(&mut e, 0x68, 0, !0));
        assert!(!check(&mut e, 0x17, 0, !0));
    }

    #[test]
    fn save_size_save_and_load_follow_change_flag_8() {
        let mut e = Engine::new();
        e.register(SAVE_WRITE, |_, _| Ret::default());
        e.register(LOAD_READ, |_, _| Ret::default());
        e.map(SAVE_LOAD_GAME, 4);
        e.mem.set_u32(SAVE_LOAD_GAME, 0x7700);
        assert_eq!(e.call(0x0047_f7e0, &args![0u32, 8u32]).u32() & 0xffff, 5);
        assert_eq!(e.call(0x0047_f7e0, &args![0u32, 7u32]).u32() & 0xffff, 0);
        let this = e.mem.alloc(0x20);
        let (_, log) = run(&mut e, 0x0047_f840, &[this, 8]);
        let expected: Vec<Vec<u32>> = (4..9).map(|o| vec![0x7700, this + o, 1]).collect();
        assert_eq!(calls(&log, SAVE_WRITE), expected);
        let (_, log) = run(&mut e, 0x0047_f8c0, &[this, 0xff, 0]);
        assert_eq!(calls(&log, LOAD_READ), expected);
        let (_, log) = run(&mut e, 0x0047_f840, &[this, 7]);
        assert!(log.is_empty());
        let (_, log) = run(&mut e, 0x0047_f8c0, &[this, 7, 0]);
        assert!(log.is_empty());
    }
}
