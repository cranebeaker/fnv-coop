//! `fallout shared/bgsdestructibleobjectform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A form that can be destroyed carries a `BGSDestructibleObjectForm`
//! component: a vtable and a pointer to its [`DestructibleObjectData`]
//! (total health, stage count, flags, an array of
//! [`DestructibleObjectStage`] pointers, and the preloaded replacement
//! models). Each stage names the health percentage at which it applies and
//! what happens there (a model-damage stage number, an explosion, debris, a
//! replacement model, flags). The component sits at a different offset in
//! each form type: `GetDestructionForm` (`00475400`) finds it.
//!
//! This is a debug-style build: every function keeps its locals in memory,
//! so the translations follow the stack-slot logic closely. x87 note: the
//! game computes in extended precision and stores `float` results; the
//! translations compute in `f64` and round to `f32` where the code stores a
//! `float`. The `FISTP` conversions (after the control word is switched to
//! round toward zero) give the "integer indefinite" value for an
//! out-of-range input: [`x87_truncate_i32`].
//!
//! The compiler's exception-unwinding frames (`FS:[0]` chains) are not
//! translated.
//!
//! This unit is translated over several sessions: the first 40 functions
//! (`004751d0` to `00477640`) were done first; the second session did the
//! remaining 29 (`00477780` to `004792f0`): the unit is complete.

#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------
// Layouts
// ---------------------------------------------------------------------

layout! {
    /// `BGSDestructibleObjectForm` (Xbox PDB), 8 bytes: the component's
    /// vtable and its data.
    pub struct BGSDestructibleObjectForm: 0x08 {
        /// `pData` (Xbox PDB).
        0x04 pData: Ptr<DestructibleObjectData>,
    }

    /// `DestructibleObjectData` (Xbox PDB), 0x14 bytes on the PC as well.
    pub struct DestructibleObjectData: 0x14 {
        /// `iHealth` (Xbox PDB): the object's total health.
        0x00 iHealth: u32,
        /// `cNumStages` (Xbox PDB).
        0x04 cNumStages: u8,
        /// `cFlags` (Xbox PDB).
        0x05 cFlags: u8,
        /// `pStagesArray` (Xbox PDB): `DestructibleObjectStage**`.
        0x08 pStagesArray: Ptr,
        /// `iReplacementModelRefCount` (Xbox PDB).
        0x0C iReplacementModelRefCount: i32,
        /// `spPreloadedReplacementModels` (Xbox PDB): `NiPointer<QueuedFile>`.
        0x10 spPreloadedReplacementModels: Ptr,
    }

    /// `DestructibleObjectStage` (Xbox PDB), 0x18 bytes on the PC as well.
    pub struct DestructibleObjectStage: 0x18 {
        /// `cModelDamageStage` (Xbox PDB).
        0x00 cModelDamageStage: u8,
        /// `cHealthPercentage` (Xbox PDB): the stage applies once the
        /// object's health has fallen to this percentage.
        0x01 cHealthPercentage: u8,
        /// `cFlags` (Xbox PDB): see [`STAGE_FLAG_CAP_DAMAGE`] and the two
        /// after it.
        0x02 cFlags: u8,
        /// `iSelfDamagePerSecond` (Xbox PDB).
        0x04 iSelfDamagePerSecond: u32,
        /// `pExplosion` (Xbox PDB): `BGSExplosion*`.
        0x08 pExplosion: Ptr,
        /// `pDebris` (Xbox PDB): `BGSDebris*`.
        0x0C pDebris: Ptr,
        /// `iDebrisCount` (Xbox PDB).
        0x10 iDebrisCount: u32,
        /// `pReplacementModel` (Xbox PDB): `TESModelTextureSwap*`.
        0x14 pReplacementModel: Ptr,
    }
}

/// `DestructibleObjectStage::FLAG_CAP_DAMAGE` (Xbox PDB).
pub(crate) const STAGE_FLAG_CAP_DAMAGE: u8 = 1;
/// `DestructibleObjectStage::FLAG_DISABLE_OBJECT` (Xbox PDB).
pub(crate) const STAGE_FLAG_DISABLE_OBJECT: u8 = 2;
/// `DestructibleObjectStage::FLAG_DESTROY_OBJECT` (Xbox PDB).
pub(crate) const STAGE_FLAG_DESTROY_OBJECT: u8 = 4;

// ---------------------------------------------------------------------
// The exe's data
// ---------------------------------------------------------------------

/// `double 0.0`.
pub(crate) const ZERO: u32 = 0x0101_2060;
/// `double 100.0`.
pub(crate) const HUNDRED: u32 = 0x0101_7a40;
/// `double -1.0`: the health `ExtraDataList::GetObjectHealth` reports for
/// an object that was never damaged.
pub(crate) const NO_HEALTH: u32 = 0x0101_a6b0;
/// `double 4294967296.0` (2^32), the divisor that turns a random 32-bit
/// word into a fraction.
pub(crate) const TWO_POW_32: u32 = 0x0101_a6b8;
/// `float -1.0`.
pub(crate) const MINUS_ONE: u32 = 0x0101_2054;
/// `float 1.0` is loaded with `FLD1`; `double 1.0` here.
pub(crate) const ONE: u32 = 0x0101_2070;

/// `BGSDestructibleObjectForm::DestructibleObjects` (Xbox PDB), the static
/// `NiTMap<TESObjectREFR *, unsigned int>` of the references that take
/// self damage; the value is the damage per second.
pub(crate) const DESTRUCTIBLE_OBJECTS: u32 = 0x011c_4104;
/// The `BSRandom` object `RandomFloat` draws from, constructed on first use
/// (bit 0 of the word at [`RANDOM_GENERATOR_INITIALIZED`] says it is).
pub(crate) const RANDOM_GENERATOR: u32 = 0x011c_4180;
pub(crate) const RANDOM_GENERATOR_INITIALIZED: u32 = 0x011c_4b4c;

// ---------------------------------------------------------------------
// Callees outside this file
// ---------------------------------------------------------------------

/// Returns the form type byte (+4) of its `ECX`.
const GET_FORM_TYPE: u32 = 0x0040_1170;
/// Returns the word at +4 of its `ECX`: the component's data pointer.
const GET_COMPONENT_DATA: u32 = 0x0072_6070;
/// `ECX` = reference: its `ExtraDataList`.
const GET_EXTRA_LIST: u32 = 0x005d_43c0;
/// `ExtraDataList::GetObjectHealth` (Xbox PDB), `ECX` = list: the health in
/// ST0, -1.0 for an object that was never damaged.
const GET_OBJECT_HEALTH: u32 = 0x0041_b6b0;
/// `ECX` = list, `float` argument: sets the object's health.
const SET_OBJECT_HEALTH: u32 = 0x0041_b6e0;
/// Truncates its `float` argument toward zero and returns it in ST0.
const TRUNCATE_FLOAT: u32 = 0x0040_4090;
/// Two tests of a reference (`ECX`) after which the damage routine does
/// nothing.
const REFERENCE_TEST_A: u32 = 0x0044_0da0;
const REFERENCE_TEST_B: u32 = 0x0044_0d80;
/// Returns the word its `ECX` points at (`NiPointer` get).
const POINTER_GET: u32 = 0x0055_9450;
/// `NiPointer<NiNode>` constructor (`ECX` = holder, the node as argument)
/// and destructor (`ECX` = holder).
const NODE_HOLDER_CONSTRUCT: u32 = 0x0063_3c90;
const NODE_HOLDER_DESTROY: u32 = 0x0045_cec0;
/// The transform (13 floats: rotation, translation, scale) of an
/// `NiAVObject` (`ECX`).
const GET_TRANSFORM: u32 = 0x0046_1130;
/// The translation (3 floats) of an `NiAVObject` (`ECX`).
const GET_TRANSLATION: u32 = 0x0045_bb80;
/// `NiAVObject::GetWorldBound` (Xbox PDB), `ECX` = object: 4 floats
/// (centre, radius).
const GET_WORLD_BOUND: u32 = 0x0043_d450;
/// Returns the word at +0x40 of its `ECX` (a reference).
const GET_REFERENCE_WORD: u32 = 0x008d_6f30;
/// `NiNode` child count (`ECX` = node) and child at an index.
const GET_CHILD_COUNT: u32 = 0x0043_b480;
const GET_CHILD_AT: u32 = 0x0043_b4a0;
/// Checked cast of a node against a type descriptor (cdecl: descriptor,
/// node): the result is the byte in `AL`.
const CHECKED_CAST: u32 = 0x0043_b300;
/// `BSRangeNode::SetCurrent` (Xbox PDB), `ECX` = node, byte argument: the
/// byte in `AL` is whether the node is now shown.
const RANGE_NODE_SET_CURRENT: u32 = 0x00c4_55a0;
/// `TESObjectREFR::GetOrientation` (Xbox PDB), `ECX` = reference: writes an
/// `NiMatrix3` (9 floats) to the argument.
const GET_ORIENTATION: u32 = 0x0056_fa00;
/// `TESObjectREFR::Update3DPosition` (Xbox PDB), `ECX` = reference.
const UPDATE_3D_POSITION: u32 = 0x0056_2020;
/// `TESObjectREFR::GetScale` (Xbox PDB), `ECX` = reference: ST0.
const GET_SCALE: u32 = 0x0056_7400;
/// `ECX` = debris form: spawns the debris (value of `008d6f30`, position
/// pointer, count, scale).
const SPAWN_DEBRIS: u32 = 0x004f_a6f0;
/// Creates an explosion (cdecl, 16 argument words).
const CREATE_EXPLOSION: u32 = 0x009a_c9c0;
/// `NiTMapBase::RemoveAt` and `SetAt` (Xbox PDB), `ECX` = map.
const MAP_REMOVE_AT: u32 = 0x0040_5430;
const MAP_SET_AT: u32 = 0x0084_4700;
/// `BSSimpleArray` of 16 bytes: constructor and destructor (`ECX`).
const ARRAY_CONSTRUCT: u32 = 0x0047_9110;
const ARRAY_DESTROY: u32 = 0x0047_9140;
/// `ECX` = array: the element count (`iSize`).
const ARRAY_SIZE: u32 = 0x0044_ddc0;
/// `ECX` = array, index argument: pointer to the element.
const ARRAY_ELEMENT: u32 = 0x006a_7ad0;
/// Reads the `float` a setting pointer points at (`ECX` = setting).
const SETTING_POINTER: u32 = 0x0040_3e20;

/// Maps a `float` to the integer `FISTP` stores after the control word is
/// switched to truncation: out of range and NaN give `0x80000000`.
pub(crate) fn x87_truncate_i32(value: f64) -> i32 {
    if value.is_nan() || value >= 2147483648.0 || value <= -2147483649.0 {
        i32::MIN
    } else {
        value as i32
    }
}

/// The same for a 64-bit store (`FISTP qword`): the indefinite value is
/// `0x8000000000000000`.
pub(crate) fn x87_truncate_i64(value: f64) -> i64 {
    if value.is_nan() || value >= 9.223_372_036_854_776e18 || value < -9.223_372_036_854_776e18 {
        i64::MIN
    } else {
        value as i64
    }
}

/// The percentage of `max` that `health` is, rounded up and truncated to a
/// byte: `health * 100 / max` stored as a `float`, then `00476b20`, then
/// `FISTP`, low byte. Every stage lookup does exactly this.
pub(crate) fn health_percentage(e: &mut Engine, health: f32, max: u32) -> u8 {
    let hundred: f64 = e.global(HUNDRED);
    let ratio = (health as f64 * hundred / max as f64) as f32;
    let rounded = fn_00476b20(e, ratio);
    x87_truncate_i32(rounded as f64) as u8
}

/// Reads `count` bytes at `from` and writes them at `to` (the game's
/// `REP MOVSD` of a structure).
fn copy_bytes(e: &mut Engine, from: u32, to: u32, count: u32) {
    let bytes = e.mem.bytes(from, count);
    e.mem.write(to, &bytes);
}

/// The stage pointer at `index` of the data's stage array (no range check).
fn stage_at(
    e: &Engine,
    data: Ptr<DestructibleObjectData>,
    index: u32,
) -> Ptr<DestructibleObjectStage> {
    let stages = e.get(data, DestructibleObjectData::pStagesArray);
    Ptr::new(e.mem.u32(stages.addr().wrapping_add(index.wrapping_mul(4))))
}

// ---------------------------------------------------------------------
// Translations
// ---------------------------------------------------------------------

// Translated from 004751d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DestructibleObjectData` constructor: everything zero, and the
/// `NiPointer<QueuedFile>` at +0x10 constructed from null (`00528cb0`).
pub fn fn_004751d0(
    e: &mut Engine,
    this: Ptr<DestructibleObjectData>,
) -> Ptr<DestructibleObjectData> {
    e.set(this, DestructibleObjectData::iHealth, 0);
    e.set(this, DestructibleObjectData::cNumStages, 0);
    e.set(this, DestructibleObjectData::cFlags, 0);
    e.set(this, DestructibleObjectData::pStagesArray, Ptr::NULL);
    e.set(this, DestructibleObjectData::iReplacementModelRefCount, 0);
    let preloaded = this.addr() + 0x10;
    e.call(0x0052_8cb0, &args![preloaded, 0u32]);
    this
}

// Translated from 004753a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DestructibleObjectData` scalar deleting destructor: runs the
/// destructor body (`00475220`) and frees the block when bit 0 of `flags`
/// is set. Returns `this`.
pub fn fn_004753a0(
    e: &mut Engine,
    this: Ptr<DestructibleObjectData>,
    flags: u32,
) -> Ptr<DestructibleObjectData> {
    e.call(0x0047_5220, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 004753d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDestructibleObjectForm::IsDestructible` (Xbox PDB), cdecl: whether
/// the form has a destruction component with at least one valid stage.
pub fn bgs_destructible_object_form_is_destructible(e: &mut Engine, form: Ptr) -> bool {
    let component = bgs_destructible_object_form_get_destruction_form(e, form);
    if component.is_null() {
        return false;
    }
    bgs_destructible_object_form_has_valid_stages(e, component.cast())
}

// Translated from 00475400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDestructibleObjectForm::GetDestructionForm` (Xbox PDB), cdecl: the
/// address of the form's `BGSDestructibleObjectForm` component, or 0.
///
/// The component sits at a different offset in each form type; the
/// compiler's switch maps the form type byte (`0x15` to `0x67`, other
/// values give 0) to the offsets below through a byte table (`004758c8`)
/// and a jump table (`00475870`). Two types reach it through a second base
/// class (the intermediate pointer is itself checked for null). The names
/// of the form types are not confirmed from the exe, so the cases are
/// numbered.
pub fn bgs_destructible_object_form_get_destruction_form(e: &mut Engine, form: Ptr) -> Ptr {
    if form.is_null() {
        return Ptr::NULL;
    }
    let form_type = e.call(GET_FORM_TYPE, &args![form]).u32();
    let base = form.addr();
    // Adds `offset` to a non-null pointer.
    let at = |base: u32, offset: i32| -> u32 {
        if base == 0 {
            0
        } else {
            base.wrapping_add(offset as u32)
        }
    };
    let component = match form_type {
        0x15 | 0x16 | 0x17 | 0x1c | 0x27 => at(base, 0x68),
        0x18 => at(base, 0x14c),
        0x19 => at(base, 0x9c),
        0x1a => at(base, 0x144),
        0x1b => at(base, 0x7c),
        0x1d | 0x1e => at(base, 0x94),
        0x1f | 0x2e | 0x32 | 0x67 => at(base, 0x84),
        0x22 => at(at(base, -0x14), 0xc),
        0x25 => at(base, 0x54),
        0x26 => at(at(base, -0xc), 0x74),
        0x28 => at(base, 0xb4),
        0x29 => at(base, 0x88),
        0x2a | 0x2b => at(base, 0x104),
        0x2f => at(base, 0xa4),
        0x33 => at(base, 0x58),
        _ => 0,
    };
    Ptr::new(component)
}

// Translated from 00475920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDestructibleObjectForm::GetModelSwapIndex` (Xbox PDB), cdecl: the
/// index of the stage whose replacement model is `model`, or 0xffffffff.
pub fn bgs_destructible_object_form_get_model_swap_index(
    e: &mut Engine,
    form: Ptr,
    model: u32,
) -> u32 {
    let component = bgs_destructible_object_form_get_destruction_form(e, form);
    if component.is_null() {
        return u32::MAX;
    }
    let data: Ptr<DestructibleObjectData> = e.call(GET_COMPONENT_DATA, &args![component]).ptr();
    if data.is_null() {
        return u32::MAX;
    }
    let count = e.get(data, DestructibleObjectData::cNumStages) as u32;
    for index in 0..count {
        let stage = stage_at(e, data, index);
        if e.get(stage, DestructibleObjectStage::pReplacementModel)
            .addr()
            == model
        {
            return index;
        }
    }
    u32::MAX
}

// Translated from 004759a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl: the replacement model of stage `index` of the form's destruction
/// component, 0 when there is none or the index is -1 or past the last
/// stage. (The index is compared as a signed number: another negative
/// index reads outside the array, as in the game.)
pub fn fn_004759a0(e: &mut Engine, form: Ptr, index: i32) -> u32 {
    let component = bgs_destructible_object_form_get_destruction_form(e, form);
    if component.is_null() || index == -1 {
        return 0;
    }
    let data: Ptr<DestructibleObjectData> = e.call(GET_COMPONENT_DATA, &args![component]).ptr();
    if data.is_null() {
        return 0;
    }
    if index < e.get(data, DestructibleObjectData::cNumStages) as i32 {
        let stage = stage_at(e, data, index as u32);
        e.get(stage, DestructibleObjectStage::pReplacementModel)
            .addr()
    } else {
        0
    }
}

// Translated from 00475a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDestructibleObjectForm::HasValidStages` (Xbox PDB): whether some
/// stage is non-null and has a replacement model, an explosion, a debris
/// form or the disable flag.
pub fn bgs_destructible_object_form_has_valid_stages(
    e: &mut Engine,
    this: Ptr<BGSDestructibleObjectForm>,
) -> bool {
    let data = e.get(this, BGSDestructibleObjectForm::pData);
    if data.is_null() {
        return false;
    }
    let mut index = 0u32;
    while index < e.get(data, DestructibleObjectData::cNumStages) as u32 {
        let stage = stage_at(e, data, index);
        if !stage.is_null()
            && (!e
                .get(stage, DestructibleObjectStage::pReplacementModel)
                .is_null()
                || !e.get(stage, DestructibleObjectStage::pExplosion).is_null()
                || !e.get(stage, DestructibleObjectStage::pDebris).is_null()
                || fn_00475a90(e, stage))
        {
            return true;
        }
        index += 1;
    }
    false
}

// Translated from 00475a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the stage has [`STAGE_FLAG_DISABLE_OBJECT`].
pub fn fn_00475a90(e: &mut Engine, stage: Ptr<DestructibleObjectStage>) -> bool {
    e.get(stage, DestructibleObjectStage::cFlags) & STAGE_FLAG_DISABLE_OBJECT != 0
}

// Translated from 00475ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The reference's total health for the percentage arithmetic. When
/// virtual slot 0x100 of the reference answers true, the value of the
/// interface embedded at +0xa4 (its slot 4, argument 0x10, a `float`
/// result) truncated to an integer; otherwise the component's `iHealth`.
pub fn fn_00475ab0(e: &mut Engine, this: Ptr<BGSDestructibleObjectForm>, reference: Ptr) -> u32 {
    if e.vcall(reference.addr(), 0x100, &args![]).bool() {
        let interface = reference.addr() + 0xa4;
        let value = e.vcall(interface, 4, &args![0x10u32]).f64();
        x87_truncate_i64(value) as u32
    } else {
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        e.get(data, DestructibleObjectData::iHealth)
    }
}

// Translated from 004768c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiMatrix3::Transpose` (Xbox PDB): writes the transpose of the matrix
/// `this` to `result` and returns `result`. The three rows are copied to a
/// stack array of `NiPoint3` (the array constructor `00401050` runs the
/// no-op `NiPoint3` constructor `006815c0` on each), then stored as the
/// columns of the result.
pub fn ni_matrix3_transpose(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    e.with_stack(0x24, |e, rows| {
        let row = |i: u32| rows.addr() + 12 * i;
        e.call(0x0040_1050, &args![rows, 12u32, 3i32, 0x0068_15c0u32]);
        fn_00476930(e, this, 0, Ptr::new(row(0)));
        fn_00476930(e, this, 1, Ptr::new(row(1)));
        fn_00476930(e, this, 2, Ptr::new(row(2)));
        fn_00476980(
            e,
            result,
            Ptr::new(row(0)),
            Ptr::new(row(1)),
            Ptr::new(row(2)),
        );
    });
    result
}

// Translated from 00476930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies row `row` (3 floats) of the matrix `this` to `out`.
pub fn fn_00476930(e: &mut Engine, this: Ptr, row: u32, out: Ptr) {
    let from = this.addr().wrapping_add(row.wrapping_mul(12));
    for i in 0..3 {
        let value = e.mem.f32(from + 4 * i);
        e.mem.set_f32(out.addr() + 4 * i, value);
    }
}

// Translated from 00476980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores three `NiPoint3` as the columns 0, 1, 2 of the matrix `this`.
/// Returns `this`.
pub fn fn_00476980(e: &mut Engine, this: Ptr, column0: Ptr, column1: Ptr, column2: Ptr) -> Ptr {
    fn_004769c0(e, this, 0, column0);
    fn_004769c0(e, this, 1, column1);
    fn_004769c0(e, this, 2, column2);
    this
}

// Translated from 004769c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the three floats at `source` in column `column` of the matrix
/// `this` (elements `column`, `column + 3`, `column + 6`).
pub fn fn_004769c0(e: &mut Engine, this: Ptr, column: u32, source: Ptr) {
    for i in 0..3 {
        let value = e.mem.f32(source.addr() + 4 * i);
        let to = this
            .addr()
            .wrapping_add(column.wrapping_mul(4))
            .wrapping_add(12 * i);
        e.mem.set_f32(to, value);
    }
}

// Translated from 00476a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the stage has [`STAGE_FLAG_CAP_DAMAGE`].
pub fn fn_00476a00(e: &mut Engine, stage: Ptr<DestructibleObjectStage>) -> bool {
    e.get(stage, DestructibleObjectStage::cFlags) & STAGE_FLAG_CAP_DAMAGE != 0
}

// Translated from 00476a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the stage has [`STAGE_FLAG_DESTROY_OBJECT`].
pub fn fn_00476a20(e: &mut Engine, stage: Ptr<DestructibleObjectStage>) -> bool {
    e.get(stage, DestructibleObjectStage::cFlags) & STAGE_FLAG_DESTROY_OBJECT != 0
}

// Translated from 00476a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stage `index` of the component, null when there is no data or the index
/// is not below the stage count.
pub fn fn_00476a40(
    e: &mut Engine,
    this: Ptr<BGSDestructibleObjectForm>,
    index: u32,
) -> Ptr<DestructibleObjectStage> {
    let data = e.get(this, BGSDestructibleObjectForm::pData);
    if data.is_null() || index >= e.get(data, DestructibleObjectData::cNumStages) as u32 {
        return Ptr::NULL;
    }
    stage_at(e, data, index)
}

// Translated from 00476a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the (empty) constructors `006815c0` of an `NiMatrix3` at `this` and
/// an `NiPoint3` at `this + 0x24`: the first two members of an
/// `NiTransform`. Returns `this`.
pub fn fn_00476a80(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0068_15c0, &args![this]);
    let translation = this.addr() + 0x24;
    e.call(0x0068_15c0, &args![translation]);
    this
}

// Translated from 00476ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the two `float`s at +0xb4 and +0xb8 of `this` to 1.0 and then runs
/// `00476ae0` with 1 (which sets a flag through `0043b370`). `this` is the
/// object the damage routine finds at slot 0x10 of the node's slot-0xc
/// result (a Havok collision object).
pub fn fn_00476ab0(e: &mut Engine, this: Ptr) {
    e.mem.set_f32(this.addr() + 0xb8, 1.0);
    e.mem.set_f32(this.addr() + 0xb4, 1.0);
    fn_00476ae0(e, this, 1);
}

// Translated from 00476ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its byte argument to `00476b00`.
pub fn fn_00476ae0(e: &mut Engine, this: Ptr, flag: u8) {
    fn_00476b00(e, this, flag);
}

// Translated from 00476b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0043b370` on `this` with the byte and the flag word 0x4000.
pub fn fn_00476b00(e: &mut Engine, this: Ptr, flag: u8) {
    e.call(0x0043_b370, &args![this, flag, 0x4000u32]);
}

// Translated from 00476b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Rounds a `float` up: the truncation `00404090` of `value`, plus 1.0 when
/// that is below `value`.
pub fn fn_00476b20(e: &mut Engine, value: f32) -> f32 {
    let truncated = e.call(TRUNCATE_FLOAT, &args![value]).f32();
    let zero: f64 = e.global(ZERO);
    // `FCOMP` then `TEST AH,1` (C0): taken for below and for unordered, so a
    // NaN difference also adds 1.0 (found by the oracle check, #189).
    #[allow(clippy::neg_cmp_op_on_partial_ord)]
    let below_or_unordered = !(truncated as f64 - value as f64 >= zero);
    let result = if below_or_unordered {
        let one: f64 = e.global(ONE);
        truncated as f64 + one
    } else {
        truncated as f64
    };
    result as f32
}

// Translated from 00476b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `RandomFloat` (Xbox PDB), cdecl: a random `float` in `[min, max]` drawn
/// from the static generator (`00476c00`).
pub fn random_float(e: &mut Engine, min: f32, max: f32) -> f32 {
    let generator = fn_00476c00(e);
    fn_00476b90(e, generator, min, max)
}

// Translated from 00476b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `min + (max - min) * r / 2^32` for a random 32-bit word `r` of the
/// generator `this`, computed in extended precision and stored as a
/// `float`.
pub fn fn_00476b90(e: &mut Engine, this: Ptr, min: f32, max: f32) -> f32 {
    let word = fn_00476be0(e, this);
    let divisor: f64 = e.global(TWO_POW_32);
    let span = max as f64 - min as f64;
    ((span * word as f64) / divisor + min as f64) as f32
}

// Translated from 00476be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSRandom::UnsignedInt` (`00aa5230`) with the largest bound,
/// 0xffffffff.
pub fn fn_00476be0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(0x00aa_5230, &args![this, u32::MAX]).u32()
}

// Translated from 00476c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The static `BSRandom` generator, constructed (`BSRandom::BSRandom`,
/// `00aa5140`) on the first call.
pub fn fn_00476c00(e: &mut Engine) -> Ptr {
    let guard: u32 = e.global(RANDOM_GENERATOR_INITIALIZED);
    if guard & 1 == 0 {
        e.set_global(RANDOM_GENERATOR_INITIALIZED, guard | 1);
        e.call(0x00aa_5140, &args![RANDOM_GENERATOR]);
    }
    Ptr::new(RANDOM_GENERATOR)
}

// Translated from 00476c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `value` in the word at +0xe0 of `this` (`pWeaponSource` of an
/// `Explosion` in the Xbox PDB, 0xf0 there).
pub fn fn_00476c70(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0xe0, value);
}

// Translated from 00476c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0xf8 of `this`.
pub fn fn_00476c90(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0xf8)
}

// Translated from 00476cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the node tree below `node` and creates the explosion form
/// `explosion` at every node that is a checked cast to the type at
/// `01202e7c` and that `BSRangeNode::SetCurrent` (byte `selector`) accepts.
/// The explosion is created with the node's transform and translation, the
/// word `008d6f30` reads from `reference`, and `reference` twice. Returns
/// the number of explosions created. `this` (the `ECX` the game passes) is
/// only handed on to the recursion.
#[allow(clippy::only_used_in_recursion)]
pub fn fn_00476cb0(
    e: &mut Engine,
    this: Ptr<BGSDestructibleObjectForm>,
    node: Ptr,
    explosion: Ptr,
    selector: u8,
    reference: Ptr,
) -> i32 {
    if node.is_null() {
        return 0;
    }
    let mut created = 0i32;
    if e.call(CHECKED_CAST, &args![0x0120_2e7cu32, node]).bool()
        && e.call(RANGE_NODE_SET_CURRENT, &args![node, selector])
            .bool()
    {
        let transform = e.call(GET_TRANSFORM, &args![node]).u32();
        let translation = e.call(GET_TRANSLATION, &args![node]).u32();
        let word = e.call(GET_REFERENCE_WORD, &args![reference]).u32();
        let mut words = vec![explosion.addr(), reference.addr(), reference.addr(), word];
        for i in 0..3 {
            words.push(e.mem.u32(translation + 4 * i));
        }
        for i in 0..9 {
            words.push(e.mem.u32(transform + 4 * i));
        }
        e.call(CREATE_EXPLOSION, &words);
        created += 1;
    }
    let children = e.vcall(node.addr(), 0xc, &args![]).u32();
    if children != 0 {
        let mut index = 0u32;
        while index < e.call(GET_CHILD_COUNT, &args![children]).u32() {
            let child = e.call(GET_CHILD_AT, &args![children, index]).ptr::<()>();
            created =
                created.wrapping_add(fn_00476cb0(e, this, child, explosion, selector, reference));
            index += 1;
        }
    }
    created
}

// Translated from 00476dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDestructibleObjectForm::SpawnDebrisAtDebrisNodes` (Xbox PDB): walks
/// the node tree below `node` and, at every node that is a checked cast to
/// the type at `01202e84` and that `BSRangeNode::SetCurrent` (byte
/// `selector`) accepts, spawns `count` of the debris form `debris` (with
/// the node's translation, the word `008d6f30` reads from `reference`, and
/// the reference's scale). Returns the number of nodes that got debris.
#[allow(clippy::only_used_in_recursion)]
pub fn bgs_destructible_object_form_spawn_debris_at_debris_nodes(
    e: &mut Engine,
    this: Ptr<BGSDestructibleObjectForm>,
    node: Ptr,
    debris: Ptr,
    selector: u8,
    reference: Ptr,
    count: u32,
) -> i32 {
    if node.is_null() {
        return 0;
    }
    let mut spawned = 0i32;
    if e.call(CHECKED_CAST, &args![0x0120_2e84u32, node]).bool()
        && e.call(RANGE_NODE_SET_CURRENT, &args![node, selector])
            .bool()
    {
        let scale = e.call(GET_SCALE, &args![reference]).f32();
        let translation = e.call(GET_TRANSLATION, &args![node]).u32();
        let word = e.call(GET_REFERENCE_WORD, &args![reference]).u32();
        e.call(
            SPAWN_DEBRIS,
            &args![debris, word, translation, count, scale],
        );
        spawned += 1;
    }
    let children = e.vcall(node.addr(), 0xc, &args![]).u32();
    if children != 0 {
        let mut index = 0u32;
        while index < e.call(GET_CHILD_COUNT, &args![children]).u32() {
            let child = e.call(GET_CHILD_AT, &args![children, index]).ptr::<()>();
            spawned =
                spawned.wrapping_add(bgs_destructible_object_form_spawn_debris_at_debris_nodes(
                    e, this, child, debris, selector, reference, count,
                ));
            index += 1;
        }
    }
    spawned
}

// Translated from 00476ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDestructibleObjectForm::UpdateDamageStageNodes` (Xbox PDB), cdecl:
/// walks the node tree below `node` and calls `BSRangeNode::SetCurrent`
/// with the damage stage `stage` on every node that is a checked cast to
/// the type at `01202e8c`.
pub fn bgs_destructible_object_form_update_damage_stage_nodes(
    e: &mut Engine,
    node: Ptr,
    stage: u8,
) {
    if node.is_null() {
        return;
    }
    if e.call(CHECKED_CAST, &args![0x0120_2e8cu32, node]).bool() {
        e.call(RANGE_NODE_SET_CURRENT, &args![node, stage]);
    }
    let children = e.vcall(node.addr(), 0xc, &args![]).u32();
    if children != 0 {
        let mut index = 0u32;
        while index < e.call(GET_CHILD_COUNT, &args![children]).u32() {
            let child = e.call(GET_CHILD_AT, &args![children, index]).ptr::<()>();
            bgs_destructible_object_form_update_damage_stage_nodes(e, child, stage);
            index += 1;
        }
    }
}

// Translated from 00476f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDestructibleObjectForm::UpdateCurrentDamageStage` (Xbox PDB): sets
/// the damage-stage nodes of the reference's 3D from its current health.
/// Returns false when the reference has no 3D (virtual slot 0x1d0 answers
/// 0), or when no damage stage applies and `force` is false.
///
/// The damage stage is that of the last stage (in order) whose percentage is
/// at least the health percentage and whose `cModelDamageStage` is not 0;
/// a health of -1.0 (never damaged) looks nothing up.
pub fn bgs_destructible_object_form_update_current_damage_stage(
    e: &mut Engine,
    this: Ptr<BGSDestructibleObjectForm>,
    reference: Ptr,
    force: bool,
) -> bool {
    if e.vcall(reference.addr(), 0x1d0, &args![]).u32() == 0 {
        return false;
    }
    let list = e.call(GET_EXTRA_LIST, &args![reference]).u32();
    let health = e.call(GET_OBJECT_HEALTH, &args![list]).f32();
    let mut damage_stage = 0u32;
    let no_health: f64 = e.global(NO_HEALTH);
    if health as f64 == no_health {
        if !force {
            return false;
        }
    } else {
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        let max = e.get(data, DestructibleObjectData::iHealth);
        let percentage = health_percentage(e, health, max);
        let mut index = 0u32;
        while index < e.get(data, DestructibleObjectData::cNumStages) as u32 {
            let stage = fn_00476a40(e, this, index);
            if e.get(stage, DestructibleObjectStage::cHealthPercentage) < percentage {
                break;
            }
            let model_stage = e.get(stage, DestructibleObjectStage::cModelDamageStage);
            if model_stage != 0 {
                damage_stage = model_stage as u32;
            }
            index += 1;
        }
    }
    if damage_stage == 0 && !force {
        return false;
    }
    let node = e.vcall(reference.addr(), 0x1d0, &args![]).ptr::<()>();
    bgs_destructible_object_form_update_damage_stage_nodes(e, node, damage_stage as u8);
    true
}

// Translated from 00477090 (decompiled, FalloutNV.exe 1.4.0.525)
/// From the reference's health percentage, walks the stages: a stage whose
/// percentage is below it ends the walk if the previous stage had no
/// self-damage value (`iSelfDamagePerSecond` 0; the first stage counts as
/// having none), otherwise it records the stage's replacement model; every
/// stage visited becomes the "previous" one. When the last stage visited
/// has a percentage below the reference's, the health is set to that
/// percentage of the total (`0041b6e0`), the replacement model (if any) is
/// applied (virtual slot 0x1cc with 0 and 1, then `0042e150` with the model
/// and the word `007af430` gives), and the damage-stage nodes are refreshed
/// (`UpdateCurrentDamageStage` with `force` false). Does nothing for a
/// health of -1.0.
pub fn fn_00477090(e: &mut Engine, this: Ptr<BGSDestructibleObjectForm>, reference: Ptr) {
    let list = e.call(GET_EXTRA_LIST, &args![reference]).u32();
    let health = e.call(GET_OBJECT_HEALTH, &args![list]).f32();
    let no_health: f64 = e.global(NO_HEALTH);
    if health as f64 == no_health {
        return;
    }
    let data = e.get(this, BGSDestructibleObjectForm::pData);
    let max = e.get(data, DestructibleObjectData::iHealth);
    let percentage = health_percentage(e, health, max);
    let mut previous_rate = 0u32;
    let mut previous_percentage = 100u8;
    let mut model = 0u32;
    let mut index = 0u32;
    while index < e.get(data, DestructibleObjectData::cNumStages) as u32 {
        let stage = stage_at(e, data, index);
        if e.get(stage, DestructibleObjectStage::cHealthPercentage) < percentage {
            if previous_rate == 0 {
                break;
            }
            let replacement = e.get(stage, DestructibleObjectStage::pReplacementModel);
            if !replacement.is_null() {
                model = replacement.addr();
            }
        }
        previous_rate = e.get(stage, DestructibleObjectStage::iSelfDamagePerSecond);
        previous_percentage = e.get(stage, DestructibleObjectStage::cHealthPercentage);
        index += 1;
    }
    if previous_percentage < percentage {
        let hundred: f64 = e.global(HUNDRED);
        let new_health = (previous_percentage as f64 / hundred * max as f64) as f32;
        let list = e.call(GET_EXTRA_LIST, &args![reference]).u32();
        e.call(SET_OBJECT_HEALTH, &args![list, new_health]);
        if model != 0 {
            e.vcall(reference.addr(), 0x1cc, &args![0u32, 1u32]);
            let form = e.call(0x007a_f430, &args![reference]).u32();
            let list = e.call(GET_EXTRA_LIST, &args![reference]).u32();
            e.call(0x0042_e150, &args![list, model, form]);
        }
        bgs_destructible_object_form_update_current_damage_stage(e, this, reference, false);
    }
}

// Translated from 00477230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the node tree below `node` and appends to the `BSSimpleArray` at
/// `array` the pointer of every node-as-container (`node`'s virtual slot
/// 0xc result) that passes `009a3830` against the word `004772e0` returns
/// (and whose `00413f40`/`0043b1b0` lookup is non-zero). The pointer is
/// passed to the array's add routine (`007cb2e0`) by address of a local.
#[allow(clippy::only_used_in_recursion)]
pub fn fn_00477230(e: &mut Engine, this: Ptr<BGSDestructibleObjectForm>, node: Ptr, array: Ptr) {
    if node.is_null() {
        return;
    }
    let children = e.vcall(node.addr(), 0xc, &args![]).u32();
    if children == 0 {
        return;
    }
    let lookup = e.call(0x0041_3f40, &args![children]).u32();
    if e.call(0x0043_b1b0, &args![lookup]).u32() != 0 {
        let filter = fn_004772e0(e);
        let lookup = e.call(0x0041_3f40, &args![children]).u32();
        if e.call(0x009a_3830, &args![lookup, filter]).bool() {
            let slot = e.mem.alloc(4);
            e.mem.set_u32(slot, children);
            e.call(0x007c_b2e0, &args![array, slot]);
            e.mem.free(slot);
        }
    }
    let mut index = 0u32;
    while index < e.call(GET_CHILD_COUNT, &args![children]).u32() {
        let child = e.call(GET_CHILD_AT, &args![children, index]).ptr::<()>();
        fn_00477230(e, this, child, array);
        index += 1;
    }
}

// Translated from 004772e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at `011c61d8`.
pub fn fn_004772e0(e: &mut Engine) -> u32 {
    e.global(0x011c_61d8)
}

// Translated from 004772f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies self damage: for every `(reference, damage per second)` entry of
/// the destructible-objects map (`DESTRUCTIBLE_OBJECTS`) whose reference
/// passes both tests `00440da0` and `00440d80` (they must be false),
/// computes `rate * scale` (the scale is the `float` `0084d030` reads from
/// the object at `011f6394`) and hands it to the reference: when the word
/// `0043d4d0` returns for the object at `011c3ea4` is 1, to virtual slot
/// 0x338 (rate, 0.0, 0) for a reference whose slot 0x100 answers true and to
/// slot 0x144 (rate, 1) for the others; otherwise to `0087af50`, the
/// object `004537b0` returns taking (reference, rate, 1).
pub fn fn_004772f0(e: &mut Engine) {
    let scale = e.call(0x0084_d030, &args![0x011f_6394u32]).f32();
    let mut position = e.call(0x004b_9ba0, &args![DESTRUCTIBLE_OBJECTS]).u32();
    // The iterator position, the reference and the damage rate: locals
    // whose addresses `006b7f20` receives.
    let locals = e.mem.alloc(12);
    while position != 0 {
        e.mem.set_u32(locals, position);
        e.mem.set_u32(locals + 4, 0);
        e.mem.set_u32(locals + 8, 0);
        e.call(
            0x006b_7f20,
            &args![DESTRUCTIBLE_OBJECTS, locals, locals + 4, locals + 8],
        );
        position = e.mem.u32(locals);
        let reference = e.mem.u32(locals + 4);
        let rate = e.mem.u32(locals + 8);
        if reference != 0
            && !e.call(REFERENCE_TEST_A, &args![reference]).bool()
            && !e.call(REFERENCE_TEST_B, &args![reference]).bool()
        {
            let damage = (rate as f64 * scale as f64) as f32;
            let mode_pointer = e.call(0x0043_d4d0, &args![0x011c_3ea4u32]).u32();
            if e.mem.u32(mode_pointer) == 1 {
                if e.vcall(reference, 0x100, &args![]).bool() {
                    e.vcall(reference, 0x338, &args![damage, 0.0f32, 0u32]);
                } else {
                    e.vcall(reference, 0x144, &args![damage, 1u32]);
                }
            } else {
                let queue = e.call(0x0045_37b0, &args![]).u32();
                e.call(0x0087_af50, &args![queue, reference, damage, 1u32]);
            }
        }
    }
    e.mem.free(locals);
}

// Translated from 00477410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `reference` from the destructible-objects map
/// (`NiTMapBase::RemoveAt`, `00405430`).
pub fn fn_00477410(e: &mut Engine, reference: Ptr) {
    e.call(MAP_REMOVE_AT, &args![DESTRUCTIBLE_OBJECTS, reference]);
}

// Translated from 00477430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl: the index of the first stage of `reference`'s destruction
/// component whose percentage is below the reference's health percentage,
/// or the stage count when there is none; 0xffffffff when `reference` is
/// null, has no destruction data or has never been damaged (-1.0).
pub fn fn_00477430(e: &mut Engine, reference: Ptr) -> u32 {
    if reference.is_null() {
        return u32::MAX;
    }
    let form = e.call(0x007a_f430, &args![reference]).u32();
    let component = bgs_destructible_object_form_get_destruction_form(e, Ptr::new(form));
    let data: Ptr<DestructibleObjectData> = if component.is_null() {
        Ptr::NULL
    } else {
        e.call(GET_COMPONENT_DATA, &args![component]).ptr()
    };
    if data.is_null() {
        return u32::MAX;
    }
    let list = e.call(GET_EXTRA_LIST, &args![reference]).u32();
    let health = e.call(GET_OBJECT_HEALTH, &args![list]).f32();
    let no_health: f64 = e.global(NO_HEALTH);
    if health as f64 == no_health {
        return u32::MAX;
    }
    let max = fn_00475ab0(e, component.cast(), reference);
    let percentage = health_percentage(e, health, max);
    let count = e.get(data, DestructibleObjectData::cNumStages) as u32;
    for index in 0..count {
        let stage = stage_at(e, data, index);
        if e.get(stage, DestructibleObjectStage::cHealthPercentage) < percentage {
            return index;
        }
    }
    count
}

// Translated from 00477560 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDestructibleObjectForm::HideSmallDebris` (Xbox PDB), cdecl: when the
/// node has a collision object (`0043b610`) whose pointer at +0x10 (via
/// `006fa820`) leads to a value with type 0x13 (`0043b4f0`/`0043b4d0`),
/// the node is hidden (`00450f90` with 1) and the result is true;
/// otherwise the same is tried on every child, and the result is true if
/// any child was hidden.
pub fn bgs_destructible_object_form_hide_small_debris(e: &mut Engine, node: Ptr) -> bool {
    let mut result = false;
    if node.is_null() {
        return result;
    }
    let mut hide = false;
    let collision = e.call(0x0043_b610, &args![node]).u32();
    if collision != 0 {
        let shape = e.call(0x006f_a820, &args![collision]).u32();
        if shape != 0 {
            let out = e.mem.alloc(8);
            let value = e.call(0x0043_b4f0, &args![shape, out]).u32();
            if e.call(0x0043_b4d0, &args![value]).u32() == 0x13 {
                hide = true;
            }
            e.mem.free(out);
        }
    }
    if hide {
        result = true;
        e.call(0x0045_0f90, &args![node, 1u32]);
    } else {
        let children = e.vcall(node.addr(), 0xc, &args![]).u32();
        if children != 0 {
            let count = e.call(GET_CHILD_COUNT, &args![children]).u32();
            for index in 0..count {
                let child = e.call(GET_CHILD_AT, &args![children, index]).ptr::<()>();
                if bgs_destructible_object_form_hide_small_debris(e, child) {
                    result = true;
                }
            }
        }
    }
    result
}

// Translated from 00477640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDestructibleObjectForm::QueueFiles` (Xbox PDB): for every stage
/// queues the replacement model (`00443d30` on the object at `011c3b3c`
/// with `arg2`, `arg3`, the LOD multiplier `TES::GetLODMult` (`0045c6b0`)
/// of `arg1`, 1, 0, 0), and calls virtual slot 0x10 (`arg2`, `arg3`) of
/// the `TESModel` embedded at +0x64 of the stage's explosion and at +0x18
/// of its debris form.
pub fn bgs_destructible_object_form_queue_files(
    e: &mut Engine,
    this: Ptr<BGSDestructibleObjectForm>,
    arg1: u32,
    arg2: u32,
    arg3: u32,
) {
    let data = e.get(this, BGSDestructibleObjectForm::pData);
    if data.is_null() {
        return;
    }
    let mut index = 0u32;
    while index < e.get(data, DestructibleObjectData::cNumStages) as u32 {
        let stage = stage_at(e, data, index);
        let model = e.get(stage, DestructibleObjectStage::pReplacementModel);
        if !model.is_null() {
            let lod = e.call(0x0045_c6b0, &args![arg1]).u32();
            let queue: u32 = e.global(0x011c_3b3c);
            e.call(
                0x0044_3d30,
                &args![queue, model, arg2, arg3, lod, 1u32, 0u32, 0u32],
            );
        }
        let stage = stage_at(e, data, index);
        let explosion = e.get(stage, DestructibleObjectStage::pExplosion);
        if !explosion.is_null() {
            e.vcall(explosion.addr() + 0x64, 0x10, &args![arg2, arg3]);
        }
        let stage = stage_at(e, data, index);
        let debris = e.get(stage, DestructibleObjectStage::pDebris);
        if !debris.is_null() {
            e.vcall(debris.addr() + 0x18, 0x10, &args![arg2, arg3]);
        }
        index += 1;
    }
}

// Translated from 00475b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies `damage` to a destructible `reference` (the third stack word,
/// `_unused_3`, is never read).
///
/// Does nothing without data, without a reference, for a damage that is not
/// above 0, when either reference test (`00440da0`, `00440d80`) is true, or
/// when the reference has no 3D (virtual slot 0x1d0). The total health is
/// `00475ab0`; the current health is the extra data's (`GetObjectHealth`),
/// or the total when it reports -1.0, and nothing happens at 0. The new
/// health is the current minus the damage, replaced by the actor value
/// (slot 0x100 true: slot 8 of the interface at +0xa4 with 0x10) and
/// clamped to 0. The old and new health percentages (rounded up, bytes)
/// select the stages crossed:
///
/// - a stage with a percentage below the old and at least the new one is
///   crossed: its damage stage, replacement model and self-damage value
///   are remembered (`crossed_*`), and its flags act at once (disable:
///   virtual slot 0x224 true runs `009bc8f0`, otherwise
///   `Script::AddPendingDisabledReference` and `TESForm::SetDelete`;
///   destroy: `00484650`; cap damage: the new health becomes that stage's
///   percentage of the total, and the walk stops);
/// - a stage at or above the old percentage and above the new one is
///   remembered as passed earlier (`earlier_*`).
///
/// Then the new health is stored (`00484650` when it is 0 or less first),
/// virtual slot 0x48 is called with 0x80000000, and when some stage was
/// crossed (`crossed_end > earlier_end`): the replacement model is swapped
/// in (keeping the reference's orientation and position relative to the
/// new 3D, and activating the new 3D in the Havok world) or, without one,
/// the damage stage nodes are updated; the self-damage map is updated
/// (set when the crossed stage has a value and health remains, removed
/// otherwise). Then for each crossed stage the action flag `0x200000` is
/// set (`Script::SetActionFlag`), and debris and explosions are spawned at
/// the 3D's nodes, falling back to the reference's own position. At
/// exactly 0 health `004778a0` runs. Finally the combat threat map is told
/// of the object (`009a54a0`, or `009a51e0` when no value results).
///
/// The local `BSSimpleArray`s and `NiTransform`s the game keeps on its
/// stack are engine blocks here; the unwinding frame is not translated.
pub fn fn_00475b20(
    e: &mut Engine,
    this: Ptr<BGSDestructibleObjectForm>,
    reference: Ptr,
    damage: f32,
    _unused_3: u32,
) {
    let data = e.get(this, BGSDestructibleObjectForm::pData);
    let zero: f64 = e.global(ZERO);
    if data.is_null() || reference.is_null() || damage as f64 <= zero {
        return;
    }
    if e.call(REFERENCE_TEST_A, &args![reference]).bool()
        || e.call(REFERENCE_TEST_B, &args![reference]).bool()
    {
        return;
    }
    if e.vcall(reference.addr(), 0x1d0, &args![]).u32() == 0 {
        return;
    }
    let max_health = fn_00475ab0(e, this, reference);
    let list = e.call(GET_EXTRA_LIST, &args![reference]).u32();
    let mut health = e.call(GET_OBJECT_HEALTH, &args![list]).f32();
    let no_health: f64 = e.global(NO_HEALTH);
    if health as f64 == no_health {
        health = max_health as f32;
    }
    if health as f64 == zero {
        return;
    }
    let old_percentage = health_percentage(e, health, max_health);
    let mut new_health = (health as f64 - damage as f64) as f32;
    if e.vcall(reference.addr(), 0x100, &args![]).bool() {
        let value = e.vcall(reference.addr() + 0xa4, 8, &args![0x10u32]).i32();
        new_health = value as f32;
    }
    if new_health as f64 <= zero {
        new_health = 0.0;
    }
    let mut new_percentage = health_percentage(e, new_health, max_health);
    if new_percentage == 0 && new_health as f64 > zero {
        new_percentage = 1;
    }

    // `NiPointer<NiNode>` holding the reference's 3D for the debris and
    // explosion walks.
    let holder = e.mem.alloc(4);
    let root = e.vcall(reference.addr(), 0x1d0, &args![]).u32();
    e.call(NODE_HOLDER_CONSTRUCT, &args![holder, root]);

    let mut crossed_end = 0u32;
    let mut earlier_end = 0u32;
    let mut crossed_model = 0u32;
    let mut crossed_damage_stage = 0u32;
    let mut earlier_damage_stage = 0u32;
    let mut crossed_rate = 0u32;
    let mut remaining_rate = 0u32;
    let hundred: f64 = e.global(HUNDRED);

    let mut index = 0u32;
    while index < e.get(data, DestructibleObjectData::cNumStages) as u32 {
        let stage = fn_00476a40(e, this, index);
        let stage_percentage = e.get(stage, DestructibleObjectStage::cHealthPercentage);
        if stage_percentage < old_percentage && stage_percentage >= new_percentage {
            let model_stage = e.get(stage, DestructibleObjectStage::cModelDamageStage);
            if model_stage != 0 {
                crossed_damage_stage = model_stage as u32;
            }
            let replacement = e.get(stage, DestructibleObjectStage::pReplacementModel);
            if !replacement.is_null() {
                crossed_model = replacement.addr();
            }
            crossed_rate = e.get(stage, DestructibleObjectStage::iSelfDamagePerSecond);
            crossed_end = index + 1;
            if fn_00475a90(e, stage) {
                if e.vcall(reference.addr(), 0x224, &args![]).bool() {
                    e.call(0x009b_c8f0, &args![reference]);
                } else {
                    e.call(0x005a_a500, &args![reference, 0u32]);
                    e.call(0x0048_4530, &args![reference, 1u32]);
                }
            }
            if fn_00476a20(e, stage) {
                e.call(0x0048_4650, &args![reference, 1u32]);
            }
            if fn_00476a00(e, stage) {
                let product = stage_percentage as u32 * max_health;
                new_health = (product as f64 / hundred) as f32;
                new_percentage = stage_percentage;
                break;
            }
        } else if old_percentage <= stage_percentage && new_percentage < stage_percentage {
            earlier_end = index + 1;
            let model_stage = e.get(stage, DestructibleObjectStage::cModelDamageStage);
            if model_stage != 0 {
                earlier_damage_stage = model_stage as u32;
            }
            remaining_rate = e.get(stage, DestructibleObjectStage::iSelfDamagePerSecond);
        }
        index += 1;
    }

    if new_health as f64 <= zero {
        e.call(0x0048_4650, &args![reference, 1u32]);
    }
    let list = e.call(GET_EXTRA_LIST, &args![reference]).u32();
    e.call(SET_OBJECT_HEALTH, &args![list, new_health]);
    e.vcall(reference.addr(), 0x48, &args![0x8000_0000u32]);

    if crossed_end > earlier_end {
        if !e.call(REFERENCE_TEST_A, &args![reference]).bool() {
            if crossed_model != 0 {
                swap_in_replacement_model(e, this, reference, crossed_model);
            } else if crossed_damage_stage > earlier_damage_stage {
                let node = e.call(0x0043_fcd0, &args![reference]).ptr::<()>();
                bgs_destructible_object_form_update_damage_stage_nodes(
                    e,
                    node,
                    crossed_damage_stage as u8,
                );
                e.call(0x0057_a3c0, &args![reference, 0u32]);
            }
        }
        if crossed_rate != 0 && new_health as f64 > zero {
            e.call(
                MAP_SET_AT,
                &args![DESTRUCTIBLE_OBJECTS, reference, crossed_rate],
            );
            remaining_rate = crossed_rate;
        } else {
            e.call(MAP_REMOVE_AT, &args![DESTRUCTIBLE_OBJECTS, reference]);
        }
    }

    // Effects of the crossed stages: debris and explosions.
    let mut crossed = earlier_end;
    while crossed < crossed_end {
        let stage = fn_00476a40(e, this, crossed);
        let list = e.call(GET_EXTRA_LIST, &args![reference]).u32();
        e.call(0x005a_c750, &args![reference, list, 0x0020_0000u32]);
        let selector_word = if crossed_damage_stage != 0 {
            crossed_damage_stage
        } else if earlier_damage_stage != 0 {
            earlier_damage_stage
        } else {
            0
        };
        let selector = selector_word as u8;
        let debris = e.get(stage, DestructibleObjectStage::pDebris);
        let count = e.get(stage, DestructibleObjectStage::iDebrisCount);
        if !debris.is_null() && count != 0 {
            let node = e.call(POINTER_GET, &args![holder]).ptr::<()>();
            let spawned = bgs_destructible_object_form_spawn_debris_at_debris_nodes(
                e, this, node, debris, selector, reference, count,
            );
            if spawned == 0 {
                // No debris node: spawn at the reference's own position.
                let position = e.vcall(reference.addr(), 0x1f4, &args![]).u32();
                let copy = e.mem.alloc(12);
                copy_bytes(e, position, copy, 12);
                let scale = e.call(GET_SCALE, &args![reference]).f32();
                let word = e.call(GET_REFERENCE_WORD, &args![reference]).u32();
                e.call(SPAWN_DEBRIS, &args![debris, word, copy, count, scale]);
                e.mem.free(copy);
            }
        }
        let explosion = e.get(stage, DestructibleObjectStage::pExplosion);
        if !explosion.is_null() {
            let node = e.call(POINTER_GET, &args![holder]).ptr::<()>();
            let created = fn_00476cb0(e, this, node, explosion, selector, reference);
            if created == 0 {
                create_explosion_near_bound(e, reference, holder, explosion);
            }
        }
        crossed += 1;
    }

    if new_health as f64 == zero {
        e.call(0x0047_78a0, &args![this]);
    }

    // The combat threat map is told about the object (its `float` result,
    // -1.0 for "none").
    let threat_key = e.mem.alloc(4);
    let minus_one: f32 = e.global(MINUS_ONE);
    let mut result = minus_one;
    if remaining_rate != 0 {
        let setting_a = setting_value(e, 0x011c_e720);
        let setting_b = setting_value(e, 0x011c_f1b0);
        result = e
            .call(
                0x0047_7a50,
                &args![
                    this,
                    new_percentage as u32,
                    setting_b,
                    setting_a,
                    threat_key
                ],
            )
            .f32();
        let limit = setting_value(e, 0x011c_eabc);
        if (limit as f64) < result as f64 {
            result = minus_one;
        }
    }
    if result as f64 == no_health {
        let setting_a = setting_value(e, 0x011c_e720);
        let setting_b = setting_value(e, 0x011c_f1b0);
        let candidate = e
            .call(
                0x0047_7970,
                &args![
                    this,
                    new_percentage as u32,
                    setting_b,
                    setting_a,
                    threat_key
                ],
            )
            .f32();
        if candidate as f64 != no_health {
            let difference = new_health as f64 - candidate as f64;
            let margin = setting_value(e, 0x011c_e1b8);
            if margin as f64 > difference {
                result = setting_value(e, 0x011c_ef20);
            }
        }
    }
    let threat_source: u32 = e.global(0x011f_1958);
    let threat_map = e.call(0x0082_5c00, &args![threat_source]).u32();
    if result as f64 != no_health {
        let key = e.mem.u32(threat_key);
        e.call(0x009a_54a0, &args![threat_map, reference, key, result]);
    } else {
        e.call(0x009a_51e0, &args![threat_map, reference]);
    }
    e.mem.free(threat_key);
    e.call(NODE_HOLDER_DESTROY, &args![holder]);
    e.mem.free(holder);
}

/// The `float` a setting pointer (`00403e20`, `ECX` = setting) points at.
fn setting_value(e: &mut Engine, setting: u32) -> f32 {
    let pointer = e.call(SETTING_POINTER, &args![setting]).u32();
    e.mem.f32(pointer)
}

/// The part of the damage routine (`00475b20`) that replaces the
/// reference's model: gathers the 3D's `BSSimpleArray` of nodes
/// (`00477230`), swaps the model in through the extra data and the routines
/// at `011ddf38`/`011dea10`, re-gathers the nodes, and carries the old
/// transform over (rotation `old * transpose(new) * orientation`, position
/// moved by the difference of the translations), then activates the new 3D
/// in its Havok world (`bhkWorld::Activate`, `00c6a270`).
fn swap_in_replacement_model(
    e: &mut Engine,
    this: Ptr<BGSDestructibleObjectForm>,
    reference: Ptr,
    model: u32,
) {
    let data = e.get(this, BGSDestructibleObjectForm::pData);
    let nodes = e.mem.alloc(0x10);
    e.call(ARRAY_CONSTRUCT, &args![nodes]);
    // `NiTransform` of the first gathered node before the swap.
    let old_transform = e.mem.alloc(0x34);
    fn_00476a80(e, Ptr::new(old_transform));
    let root = e.vcall(reference.addr(), 0x1d0, &args![]).ptr::<()>();
    fn_00477230(e, this, root, Ptr::new(nodes));
    if e.call(ARRAY_SIZE, &args![nodes]).u32() != 0 {
        let element = e.call(ARRAY_ELEMENT, &args![nodes, 0u32]).u32();
        let first = e.mem.u32(element);
        let transform = e.call(GET_TRANSFORM, &args![first]).u32();
        copy_bytes(e, transform, old_transform, 0x34);
    }
    let mut had_preloaded_model = false;
    let preloaded = e.call(POINTER_GET, &args![data.addr() + 0x10]).u32();
    if preloaded != 0 {
        e.call(0x0040_b460, &args![data.addr() + 0xc]);
        had_preloaded_model = true;
    }
    let flag_b = e.call(0x0057_b200, &args![reference]).u8();
    let swap_flags: u32 = e.global(0x011d_df38);
    let flag_c = e.call(0x0046_23f0, &args![swap_flags, 0u32]).u8();
    e.vcall(reference.addr(), 0x1cc, &args![0u32, 1u32]);
    if flag_b != 0 {
        e.call(0x0057_b240, &args![reference, 1u32]);
    }
    e.call(0x0046_23f0, &args![swap_flags, flag_c as u32]);
    let form = e.call(0x007a_f430, &args![reference]).u32();
    let list = e.call(GET_EXTRA_LIST, &args![reference]).u32();
    e.call(0x0042_e150, &args![list, model, form]);
    e.vcall(reference.addr(), 0x48, &args![0x8000_0000u32]);
    let swap_object: u32 = e.global(0x011d_ea10);
    let word = e.call(GET_REFERENCE_WORD, &args![reference]).u32();
    e.call(
        0x0045_1ef0,
        &args![swap_object, reference, word, 0u32, 0u32],
    );

    if e.vcall(reference.addr(), 0x1d0, &args![]).u32() != 0 {
        let new_root = e.vcall(reference.addr(), 0x1d0, &args![]).u32();
        let container = if new_root != 0 {
            e.vcall(new_root, 0xc, &args![]).u32()
        } else {
            0
        };
        let collision = if container != 0 {
            e.vcall(container, 0x10, &args![]).u32()
        } else {
            0
        };
        if collision != 0 {
            fn_00476ab0(e, Ptr::new(collision));
        }
        if e.call(ARRAY_SIZE, &args![nodes]).u32() != 0 {
            let new_nodes = e.mem.alloc(0x10);
            e.call(ARRAY_CONSTRUCT, &args![new_nodes]);
            fn_00477230(e, this, Ptr::new(new_root), Ptr::new(new_nodes));
            if e.call(ARRAY_SIZE, &args![new_nodes]).u32() != 0 {
                carry_over_transform(e, reference, old_transform, new_nodes);
            }
            e.call(ARRAY_DESTROY, &args![new_nodes]);
            e.mem.free(new_nodes);
        }
        e.call(0x00c6_a270, &args![new_root, 1u32, 1u32, 0u32]);
    }
    if had_preloaded_model {
        e.call(0x0047_78a0, &args![this]);
    }
    e.call(ARRAY_DESTROY, &args![nodes]);
    e.mem.free(nodes);
    e.mem.free(old_transform);
}

/// Puts the reference's orientation and position back relative to the new
/// 3D: the rotation of the old transform times the transpose of the new
/// node's, times the reference's orientation, is set as the orientation;
/// the position is the old translation minus the new one, plus the
/// reference's position.
fn carry_over_transform(e: &mut Engine, reference: Ptr, old_transform: u32, new_nodes: u32) {
    let new_transform = e.mem.alloc(0x34);
    let element = e.call(ARRAY_ELEMENT, &args![new_nodes, 0u32]).u32();
    let first = e.mem.u32(element);
    let source = e.call(GET_TRANSFORM, &args![first]).u32();
    copy_bytes(e, source, new_transform, 0x34);
    let transposed = e.mem.alloc(0x24);
    ni_matrix3_transpose(e, Ptr::new(new_transform), Ptr::new(transposed));
    let old_rotation = e.mem.alloc(0x24);
    copy_bytes(e, old_transform, old_rotation, 0x24);
    let orientation = e.mem.alloc(0x24);
    e.call(GET_ORIENTATION, &args![reference, orientation]);
    let product = e.mem.alloc(0x24);
    let combined = e.mem.alloc(0x24);
    let first_product = e
        .call(0x0043_f8d0, &args![old_rotation, product, transposed])
        .u32();
    e.call(0x0043_f8d0, &args![first_product, combined, orientation]);
    let mut words = vec![reference.addr()];
    for i in 0..9 {
        words.push(e.mem.u32(combined + 4 * i));
    }
    e.call(0x0056_fa90, &words);
    e.call(UPDATE_3D_POSITION, &args![reference]);

    // The second look at the new node's transform, as the game does.
    let element = e.call(ARRAY_ELEMENT, &args![new_nodes, 0u32]).u32();
    let first = e.mem.u32(element);
    let source = e.call(GET_TRANSFORM, &args![first]).u32();
    copy_bytes(e, source, new_transform, 0x34);
    let difference = e.mem.alloc(12);
    e.call(
        0x0043_9ef0,
        &args![old_transform + 0x24, difference, new_transform + 0x24],
    );
    let position = e.vcall(reference.addr(), 0x1f4, &args![]).u32();
    e.call(0x0063_c8a0, &args![difference, position]);
    e.call(0x0057_5830, &args![reference, difference]);
    e.call(UPDATE_3D_POSITION, &args![reference]);
    for block in [
        new_transform,
        transposed,
        old_rotation,
        orientation,
        product,
        combined,
        difference,
    ] {
        e.mem.free(block);
    }
}

/// Creates the explosion `explosion` near the 3D's bound when no node took
/// it: the reference's orientation, and a position that is the centre of
/// the 3D's world bound plus a random vector (x and y in [-1, 1], z in
/// [-1, 0], scaled by the radius).
fn create_explosion_near_bound(e: &mut Engine, reference: Ptr, holder: u32, explosion: Ptr) {
    let orientation = e.mem.alloc(0x24);
    e.call(GET_ORIENTATION, &args![reference, orientation]);
    let node = e.call(POINTER_GET, &args![holder]).u32();
    let bound_source = e.call(GET_WORLD_BOUND, &args![node]).u32();
    let bound = e.mem.alloc(16);
    copy_bytes(e, bound_source, bound, 16);
    let vector = e.mem.alloc(12);
    e.call(0x0068_15c0, &args![vector]);
    let minus_one: f32 = e.global(MINUS_ONE);
    let x = random_float(e, minus_one, 1.0);
    e.mem.set_f32(vector, x);
    let y = random_float(e, minus_one, 1.0);
    e.mem.set_f32(vector + 4, y);
    let z = random_float(e, minus_one, 0.0);
    e.mem.set_f32(vector + 8, z);
    let radius = e.call(0x0084_d030, &args![bound]).f32();
    e.call(0x0043_9180, &args![vector, radius]);
    let position = e.mem.alloc(12);
    let bound_centre = e.call(0x0068_15c0, &args![bound]).u32();
    e.call(0x0043_9e90, &args![bound_centre, position, vector]);

    let mut words = vec![explosion.addr()];
    let word = e.call(GET_REFERENCE_WORD, &args![reference]).u32();
    let is_projectile_like = e.vcall(reference.addr(), 0x224, &args![]).bool();
    if is_projectile_like {
        let extra = e.call(0x0087_4480, &args![reference]).u32();
        words.extend_from_slice(&[extra, 0, word]);
    } else {
        words.extend_from_slice(&[reference.addr(), reference.addr(), word]);
    }
    for i in 0..3 {
        words.push(e.mem.u32(position + 4 * i));
    }
    for i in 0..9 {
        words.push(e.mem.u32(orientation + 4 * i));
    }
    let created = e.call(CREATE_EXPLOSION, &words).u32();
    if is_projectile_like && created != 0 && fn_00476c90(e, reference) != 0 {
        let value = fn_00476c90(e, reference);
        fn_00476c70(e, Ptr::new(created), value);
    }
    for block in [orientation, bound, vector, position] {
        e.mem.free(block);
    }
}

// ---------------------------------------------------------------------
// Second session: preloading, self damage, saving, loading, copying
// ---------------------------------------------------------------------

/// `operator new` (cdecl: size) and `operator delete` (cdecl: block).
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `memset` (cdecl: block, value, size).
const MEMSET: u32 = 0x0040_3d30;
/// `__RTDynamicCast` (cdecl: object, vfptr delta, source type descriptor,
/// target type descriptor, reference flag).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Writes a line to the log (cdecl: format, arguments...).
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// `ECX` = form: the form id (used in the log messages and in `Save`).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// `NiPointer<QueuedFile>::operator=` (`ECX` = the holder, the file as
/// argument).
const QUEUED_FILE_ASSIGN: u32 = 0x006f_74f0;
/// Whether the plugin data is stored the other way round (`Save` and
/// `LoadChunk` swap the bytes of their structures when it answers yes).
const IS_SWAPPED_BYTE_ORDER: u32 = 0x0040_1500;
const IS_SWAPPED_BYTE_ORDER_FILE: u32 = 0x0040_1680;

/// The chunk tags of the form's plugin records (four ASCII letters).
pub(crate) const CHUNK_DEST: u32 = 0x5453_4544;
pub(crate) const CHUNK_DSTD: u32 = 0x4454_5344;
pub(crate) const CHUNK_DSTF: u32 = 0x4654_5344;
pub(crate) const CHUNK_DMDL: u32 = 0x4c44_4d44;
pub(crate) const CHUNK_DMDT: u32 = 0x5444_4d44;
pub(crate) const CHUNK_DMDS: u32 = 0x5344_4d44;

// Translated from 00477780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDestructibleObjectForm::PreloadReplacementModels` (Xbox PDB): with
/// data present and nothing preloaded yet, creates the `QueuedFile`
/// (0x28 bytes, constructed with 5 by `00c3c590`), stores it in the
/// data's preloaded-models holder (+0x10), queues the files
/// (`QueueFiles`, `queue_arg`, 5, the file) and, when the holder's answer
/// to `00446990` is 0, clears the holder again. When a file is held at the
/// end, `0040b460` is called on the reference count at data +0x0C.
/// (The compiler's exception frame is not translated.)
pub fn bgs_destructible_object_form_preload_replacement_models(
    e: &mut Engine,
    this: Ptr<BGSDestructibleObjectForm>,
    queue_arg: u32,
) {
    if e.get(this, BGSDestructibleObjectForm::pData).is_null() {
        return;
    }
    let holder = |e: &Engine| e.get(this, BGSDestructibleObjectForm::pData).addr() + 0x10;
    let current = holder(e);
    if e.call(POINTER_GET, &args![current]).u32() == 0 {
        let block = e.call(OPERATOR_NEW, &args![0x28u32]).u32();
        let file = if block != 0 {
            e.call(0x00c3_c590, &args![block, 5u32]).u32()
        } else {
            0
        };
        let current = holder(e);
        e.call(QUEUED_FILE_ASSIGN, &args![current, file]);
        let current = holder(e);
        let held = e.call(POINTER_GET, &args![current]).u32();
        bgs_destructible_object_form_queue_files(e, this, queue_arg, 5, held);
        let current = holder(e);
        let held = e.call(POINTER_GET, &args![current]).u32();
        if e.call(0x0044_6990, &args![held]).u32() == 0 {
            let current = holder(e);
            e.call(QUEUED_FILE_ASSIGN, &args![current, 0u32]);
        }
    }
    let current = holder(e);
    if e.call(POINTER_GET, &args![current]).u32() != 0 {
        let count = e.get(this, BGSDestructibleObjectForm::pData).addr() + 0xc;
        e.call(0x0040_b460, &args![count]);
    }
}

// Translated from 004778a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Counterpart of the preloading (fastcall `this`): with data present and
/// a preloaded file held, calls `004019a0` on the reference count at data
/// +0x0C and, once the count is 0 or less, clears the holder.
pub fn fn_004778a0(e: &mut Engine, this: Ptr<BGSDestructibleObjectForm>) {
    let data = e.get(this, BGSDestructibleObjectForm::pData);
    if data.is_null() {
        return;
    }
    if e.call(POINTER_GET, &args![data.addr() + 0x10]).u32() == 0 {
        return;
    }
    let data = e.get(this, BGSDestructibleObjectForm::pData);
    e.call(0x0040_19a0, &args![data.addr() + 0xc]);
    let data = e.get(this, BGSDestructibleObjectForm::pData);
    if e.get(data, DestructibleObjectData::iReplacementModelRefCount) <= 0 {
        e.call(QUEUED_FILE_ASSIGN, &args![data.addr() + 0x10, 0u32]);
    }
}

// Translated from 00477900 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSExplosion::GetRadiusBSUnits` (Xbox PDB): the explosion's radius
/// (`float` at +0x7C), multiplied by the setting at `011d1218` unless
/// `00477950(1)` answers yes.
pub fn bgs_explosion_get_radius_bs_units(e: &mut Engine, this: Ptr) -> f32 {
    let radius = e.mem.f32(this.addr() + 0x7c);
    if e.call(0x0047_7950, &args![this, 1u32]).u8() != 0 {
        radius
    } else {
        let setting = e.call(SETTING_POINTER, &args![0x011d_1218u32]).u32();
        let factor = e.mem.f32(setting);
        (radius as f64 * factor as f64) as f32
    }
}

// Translated from 00477970 (decompiled, FalloutNV.exe 1.4.0.525)
/// First stage (in order) whose health percentage is below `percentage`
/// and whose explosion has a radius (`GetRadiusBSUnits`) of at least
/// `radius_limit` and a value from `006a78f0` of at least `other_limit`:
/// returns the health at which the stage begins (`percentage / 100` of the
/// total health, as `float`) and stores the explosion through `out` when
/// it is not null. Without such a stage the result is -1.0.
pub fn fn_00477970(
    e: &mut Engine,
    this: Ptr<BGSDestructibleObjectForm>,
    percentage: u8,
    radius_limit: f32,
    other_limit: f32,
    out: Ptr,
) -> f32 {
    let mut index = 0u32;
    loop {
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        if index >= e.get(data, DestructibleObjectData::cNumStages) as u32 {
            return e.global(MINUS_ONE);
        }
        let stage = stage_at(e, data, index);
        let stage_percentage = e.get(stage, DestructibleObjectStage::cHealthPercentage);
        let explosion = e.get(stage, DestructibleObjectStage::pExplosion);
        if stage_percentage < percentage && !explosion.is_null() {
            let radius = bgs_explosion_get_radius_bs_units(e, explosion) as f64;
            if radius_limit as f64 <= radius {
                let other = e.call(0x006a_78f0, &args![explosion]).f32() as f64;
                if other_limit as f64 <= other {
                    let hundred: f64 = e.global(HUNDRED);
                    let data = e.get(this, BGSDestructibleObjectForm::pData);
                    let health = e.get(data, DestructibleObjectData::iHealth);
                    let value = ((stage_percentage as f64 / hundred) * health as f64) as f32;
                    if !out.is_null() {
                        e.mem.set_u32(out.addr(), explosion.addr());
                    }
                    return value;
                }
            }
        }
        index += 1;
    }
}

// Translated from 00477a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the stages in order; for each stage below `percentage` adds the
/// health between the previous stage's threshold and this one's, divided
/// by the previous stage's self damage per second, to a running total
/// (`float`), and returns that total at the first such stage that has an
/// explosion passing the same two limits as `00477970`, storing the
/// explosion through `out` when it is not null. Stops (result -1.0) at the
/// first stage below `percentage` that follows a stage with no self damage.
pub fn fn_00477a50(
    e: &mut Engine,
    this: Ptr<BGSDestructibleObjectForm>,
    percentage: u8,
    radius_limit: f32,
    other_limit: f32,
    out: Ptr,
) -> f32 {
    let hundred: f64 = e.global(HUNDRED);
    let data = e.get(this, BGSDestructibleObjectForm::pData);
    let total_health = e.get(data, DestructibleObjectData::iHealth);
    let mut total = 0.0f32;
    let mut threshold = ((total_health as f64 * percentage as f64) / hundred) as f32;
    let mut previous_rate = 0u32;
    let mut index = 0u32;
    loop {
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        if index >= e.get(data, DestructibleObjectData::cNumStages) as u32 {
            break;
        }
        let stage = stage_at(e, data, index);
        let stage_percentage = e.get(stage, DestructibleObjectStage::cHealthPercentage);
        if stage_percentage < percentage {
            if previous_rate == 0 {
                break;
            }
            let health = e.get(data, DestructibleObjectData::iHealth);
            let here = ((health as f64 * stage_percentage as f64) / hundred) as f32;
            total =
                (((threshold as f64 - here as f64) / previous_rate as f64) + total as f64) as f32;
            threshold = here;
            let explosion = e.get(stage, DestructibleObjectStage::pExplosion);
            if !explosion.is_null() {
                let radius = bgs_explosion_get_radius_bs_units(e, explosion) as f64;
                if radius_limit as f64 <= radius {
                    let other = e.call(0x006a_78f0, &args![explosion]).f32() as f64;
                    if other_limit as f64 <= other {
                        if !out.is_null() {
                            e.mem.set_u32(out.addr(), explosion.addr());
                        }
                        return total;
                    }
                }
            }
        }
        previous_rate = e.get(stage, DestructibleObjectStage::iSelfDamagePerSecond);
        index += 1;
    }
    e.global(MINUS_ONE)
}

// Translated from 00477ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDestructibleObjectForm::SetSelfDamage` (Xbox PDB), cdecl: with a
/// damage of 0 removes the reference from the self-damage map
/// ([`DESTRUCTIBLE_OBJECTS`], `NiTMapBase::RemoveAt`), otherwise sets its
/// entry to `damage_per_second` (`SetAt`).
pub fn bgs_destructible_object_form_set_self_damage(
    e: &mut Engine,
    reference: Ptr,
    damage_per_second: u32,
) {
    if damage_per_second == 0 {
        e.call(MAP_REMOVE_AT, &args![DESTRUCTIBLE_OBJECTS, reference]);
    } else {
        e.call(
            MAP_SET_AT,
            &args![DESTRUCTIBLE_OBJECTS, reference, damage_per_second],
        );
    }
}

// Translated from 00477d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Cdecl: brings a damaged reference's 3D to its current state and takes
/// it out of the self-damage and threat bookkeeping. Does nothing unless
/// `00452370` accepts the reference and its object health is not -1.0
/// (never damaged). Then: `0041b7c0` on its extra data list; when
/// `00477ba0` answers yes, `00484650(0)`; if the list holds a model swap
/// (`0042e250`) the 3D is rebuilt as in the damage routine (`0042e280`,
/// the flag word at `011ddf38` cleared around virtual slot 0x1CC, the swap
/// object at `011dea10`, the collision object's `fn_00476ab0`, and
/// `bhkWorld::Activate`), otherwise the damage stage nodes are updated
/// with stage 0. Finally `00573f40` for a reference that `00440da0`
/// answers yes for, the reference leaves the self-damage map, and the
/// threat map forgets it (`009a51e0`).
pub fn fn_00477d10(e: &mut Engine, reference: Ptr) {
    if e.call(0x0045_2370, &args![reference]).u8() == 0 {
        return;
    }
    let form = e.call(0x007a_f430, &args![reference]).u32();
    // The component is looked up and never used (debug build).
    let _component = bgs_destructible_object_form_get_destruction_form(e, Ptr::new(form));
    let list = e.call(GET_EXTRA_LIST, &args![reference]).u32();
    let health = e.call(GET_OBJECT_HEALTH, &args![list]).f32();
    let no_health: f64 = e.global(NO_HEALTH);
    if health as f64 == no_health {
        return;
    }
    let list = e.call(GET_EXTRA_LIST, &args![reference]).u32();
    e.call(0x0041_b7c0, &args![list]);
    if e.call(0x0047_7ba0, &args![reference]).u8() != 0 {
        e.call(0x0048_4650, &args![reference, 0u32]);
    }
    let list = e.call(GET_EXTRA_LIST, &args![reference]).u32();
    let model_swap = e.call(0x0042_e250, &args![list]).u32();
    if model_swap != 0 {
        let list = e.call(GET_EXTRA_LIST, &args![reference]).u32();
        e.call(0x0042_e280, &args![list]);
        if e.vcall(reference.addr(), 0x1d0, &args![]).u32() != 0 {
            let swap_flags: u32 = e.global(0x011d_df38);
            let saved = e.call(0x0046_23f0, &args![swap_flags, 0u32]).u8();
            e.vcall(reference.addr(), 0x1cc, &args![0u32, 1u32]);
            e.call(0x0046_23f0, &args![swap_flags, saved as u32]);
            let word = e.call(GET_REFERENCE_WORD, &args![reference]).u32();
            let swap_object: u32 = e.global(0x011d_ea10);
            e.call(
                0x0045_1ef0,
                &args![swap_object, reference, word, 0u32, 0u32],
            );
            if e.vcall(reference.addr(), 0x1d0, &args![]).u32() != 0 {
                let root = e.vcall(reference.addr(), 0x1d0, &args![]).u32();
                let container = if root != 0 {
                    e.vcall(root, 0xc, &args![]).u32()
                } else {
                    0
                };
                let collision = if container != 0 {
                    e.vcall(container, 0x10, &args![]).u32()
                } else {
                    0
                };
                if collision != 0 {
                    fn_00476ab0(e, Ptr::new(collision));
                }
                e.call(0x00c6_a270, &args![root, 1u32, 1u32, 0u32]);
            }
        }
    } else if e.vcall(reference.addr(), 0x1d0, &args![]).u32() != 0 {
        // 00477ec2-00477ecd: the 0 pushed first is `00476ec0`'s stage; the
        // getter `0043fcd0` is thiscall with no arguments (plain RET).
        let node = e.call(0x0043_fcd0, &args![reference]).u32();
        bgs_destructible_object_form_update_damage_stage_nodes(e, Ptr::new(node), 0);
    }
    if e.call(REFERENCE_TEST_A, &args![reference]).u8() != 0 {
        e.call(0x0057_3f40, &args![reference]);
    }
    e.call(MAP_REMOVE_AT, &args![DESTRUCTIBLE_OBJECTS, reference]);
    let threat_source: u32 = e.global(0x011f_1958);
    let threat_map = e.call(0x0082_5c00, &args![threat_source]).u32();
    e.call(0x009a_51e0, &args![threat_map, reference]);
}

// Translated from 00477f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDestructibleObjectForm::Save` (Xbox PDB), fastcall `this`: with
/// data present writes the chunk `DEST` (8 bytes: health, stage count,
/// flags; two bytes of the buffer are never set), then for every stage a
/// `DSTD` chunk (0x14 bytes: percentage, index, model damage stage, flags,
/// self damage per second, the explosion's and the debris form's ids, the
/// debris count), the stage's replacement model through
/// `TESModelTextureSwap::Save` (`0048a520` with `DMDL`, `DMDT`, `DMDS`)
/// when it has one, and the end chunk `DSTF`. When `00401500` says the
/// byte order is swapped, the structures are swapped before and back after
/// each write (`00503210` for the header, [`fn_00478130`] for a stage).
pub fn bgs_destructible_object_form_save(e: &mut Engine, this: Ptr<BGSDestructibleObjectForm>) {
    if e.get(this, BGSDestructibleObjectForm::pData).is_null() {
        return;
    }
    let header = e.mem.alloc(8);
    fn_004781b0(e, Ptr::new(header));
    let data = e.get(this, BGSDestructibleObjectForm::pData);
    let health = e.get(data, DestructibleObjectData::iHealth);
    e.mem.set_u32(header, health);
    let data = e.get(this, BGSDestructibleObjectForm::pData);
    let count = e.get(data, DestructibleObjectData::cNumStages);
    e.mem.set_u8(header + 4, count);
    let data = e.get(this, BGSDestructibleObjectForm::pData);
    let flags = e.get(data, DestructibleObjectData::cFlags);
    e.mem.set_u8(header + 5, flags);
    if e.call(IS_SWAPPED_BYTE_ORDER, &args![]).u8() != 0 {
        e.call(0x0050_3210, &args![header]);
    }
    e.call(0x0048_5990, &args![CHUNK_DEST, header, 8u32]);
    if e.call(IS_SWAPPED_BYTE_ORDER, &args![]).u8() != 0 {
        e.call(0x0050_3210, &args![header]);
    }
    e.mem.free(header);

    let record = e.mem.alloc(0x14);
    let mut index = 0u32;
    loop {
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        if index >= e.get(data, DestructibleObjectData::cNumStages) as u32 {
            break;
        }
        e.call(MEMSET, &args![record, 0u32, 0x14u32]);
        e.mem.set_u8(record + 1, index as u8);
        let current = |e: &Engine| {
            let data = e.get(this, BGSDestructibleObjectForm::pData);
            stage_at(e, data, index)
        };
        let stage = current(e);
        let percentage = e.get(stage, DestructibleObjectStage::cHealthPercentage);
        e.mem.set_u8(record, percentage);
        let stage = current(e);
        let rate = e.get(stage, DestructibleObjectStage::iSelfDamagePerSecond);
        e.mem.set_u32(record + 4, rate);
        let stage = current(e);
        let damage_stage = e.get(stage, DestructibleObjectStage::cModelDamageStage);
        e.mem.set_u8(record + 2, damage_stage);
        let stage = current(e);
        let stage_flags = e.get(stage, DestructibleObjectStage::cFlags);
        e.mem.set_u8(record + 3, stage_flags);
        let stage = current(e);
        let debris_count = e.get(stage, DestructibleObjectStage::iDebrisCount);
        e.mem.set_u32(record + 0x10, debris_count);
        let stage = current(e);
        let explosion = e.get(stage, DestructibleObjectStage::pExplosion);
        if !explosion.is_null() {
            let stage = current(e);
            let explosion = e.get(stage, DestructibleObjectStage::pExplosion);
            let id = e.call(GET_FORM_ID, &args![explosion]).u32();
            e.mem.set_u32(record + 8, id);
        }
        let stage = current(e);
        let debris = e.get(stage, DestructibleObjectStage::pDebris);
        if !debris.is_null() {
            let id = e.call(GET_FORM_ID, &args![debris]).u32();
            e.mem.set_u32(record + 0xc, id);
        }
        if e.call(IS_SWAPPED_BYTE_ORDER, &args![]).u8() != 0 {
            fn_00478130(e, Ptr::new(record));
        }
        e.call(0x0048_5990, &args![CHUNK_DSTD, record, 0x14u32]);
        if e.call(IS_SWAPPED_BYTE_ORDER, &args![]).u8() != 0 {
            fn_00478130(e, Ptr::new(record));
        }
        let stage = current(e);
        let model = e.get(stage, DestructibleObjectStage::pReplacementModel);
        if !model.is_null() {
            e.call(
                0x0048_a520,
                &args![model, CHUNK_DMDL, CHUNK_DMDT, CHUNK_DMDS],
            );
        }
        e.call(0x0048_56d0, &args![CHUNK_DSTF]);
        index += 1;
    }
    e.mem.free(record);
}

// Translated from 00478130 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall `this` = a `DSTD` record: calls `00401080(address, 0)` on each
/// of the four words at +4, +8, +0x0C and +0x10 (the byte-order swap).
pub fn fn_00478130(e: &mut Engine, this: Ptr) {
    for offset in [4u32, 8, 0xc, 0x10] {
        e.call(0x0040_1080, &args![this.addr() + offset, 0u32]);
    }
}

// Translated from 004781b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall `this` = a `DEST` record (8 bytes): zeroes the health word and
/// the two bytes at +4 and +5 (bytes 6 and 7 are left alone). Returns
/// `this`.
pub fn fn_004781b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u8(this.addr() + 4, 0);
    e.mem.set_u8(this.addr() + 5, 0);
    this
}

// Translated from 004781e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDestructibleObjectForm::LoadChunk` (Xbox PDB), cdecl: reads the
/// form's chunks from the plugin `file`. Does nothing without a form or a
/// file. The current chunk type (`TESFile::GetTESChunk`, `004726b0`) is
/// read first, then the component's data is created if missing
/// ([`fn_00478600`]).
///
/// * `DEST`: an 8-byte record (health, stage count, flags) fills the data
///   and the stage array is allocated ([`fn_00478e90`]); a record of
///   another size is the old format: it is logged ("Old Destruction data
///   found on form ... It needs to be resaved.") and the data deleted
///   ([`fn_00478690`]).
/// * `DSTD`: a 0x14-byte record fills the stage it numbers (when that
///   number is below the stage count). Then chunks are read until `DSTF`,
///   the end of the record or a `file` that has no more: `DMDL` (the model
///   path, through slot 0x18 of the stage's model), `DMDT` (the model's
///   texture chunk, `004893e0`) and `DMDS` (the texture swaps,
///   `0048a7b0` with the chunk's bytes, size and version) after creating
///   the stage's model ([`fn_00478570`]).
///
/// The stack buffers the game takes with `__alloca_probe_16` are heap
/// blocks here, freed at the end. The stack cookie is not translated.
pub fn bgs_destructible_object_form_load_chunk(
    e: &mut Engine,
    this: Ptr<BGSDestructibleObjectForm>,
    file: Ptr,
) {
    if file.is_null() || this.is_null() {
        return;
    }
    let chunk = e.call(0x0047_26b0, &args![file]).u32();
    fn_00478600(e, this);
    let data: Ptr<DestructibleObjectData> =
        Ptr::new(e.call(GET_COMPONENT_DATA, &args![this]).u32());
    if chunk == CHUNK_DSTD {
        load_stage_chunk(e, file, data);
    } else if chunk == CHUNK_DEST {
        let header = e.mem.alloc(8);
        fn_004781b0(e, Ptr::new(header));
        let size = e.call(0x0040_1660, &args![file]).u32();
        if size == 8 {
            e.call(0x0047_2890, &args![file, header, 0u32]);
            if e.call(IS_SWAPPED_BYTE_ORDER_FILE, &args![file]).u8() != 0 {
                e.call(0x0050_3210, &args![header]);
            }
            let health = e.mem.u32(header);
            e.set(data, DestructibleObjectData::iHealth, health);
            let count = e.mem.u8(header + 4);
            e.set(data, DestructibleObjectData::cNumStages, count);
            let flags = e.mem.u8(header + 5);
            e.set(data, DestructibleObjectData::cFlags, flags);
            if count != 0 {
                let stages = fn_00478e90(e, this, count);
                e.set(data, DestructibleObjectData::pStagesArray, stages);
            }
        } else {
            let form = e
                .call(
                    DYNAMIC_CAST,
                    &args![this, 0u32, 0x0118_32acu32, 0x0118_3028u32, 0u32],
                )
                .u32();
            let name = e.vcall(form, 0x130, &args![]).u32();
            let id = e.call(GET_FORM_ID, &args![form]).u32();
            e.call(LOG_MESSAGE, &args![0x0101_a6c8u32, id, name]);
            fn_00478690(e, this);
        }
        e.mem.free(header);
    }
}

/// The `DSTD` part of [`bgs_destructible_object_form_load_chunk`].
fn load_stage_chunk(e: &mut Engine, file: Ptr, data: Ptr<DestructibleObjectData>) {
    let record = e.mem.alloc(0x14);
    e.call(MEMSET, &args![record, 0u32, 0x14u32]);
    e.call(0x0047_2890, &args![file, record, 0x14u32]);
    if e.call(IS_SWAPPED_BYTE_ORDER_FILE, &args![file]).u8() != 0 {
        fn_00478130(e, Ptr::new(record));
    }
    let index = e.mem.u8(record + 1);
    if (index as u32) < e.get(data, DestructibleObjectData::cNumStages) as u32 {
        let stage = stage_at(e, data, index as u32);
        let damage_stage = e.mem.u8(record + 2);
        e.set(
            stage,
            DestructibleObjectStage::cModelDamageStage,
            damage_stage,
        );
        let percentage = e.mem.u8(record);
        e.set(
            stage,
            DestructibleObjectStage::cHealthPercentage,
            percentage,
        );
        let stage_flags = e.mem.u8(record + 3);
        e.set(stage, DestructibleObjectStage::cFlags, stage_flags);
        let rate = e.mem.u32(record + 4);
        e.set(stage, DestructibleObjectStage::iSelfDamagePerSecond, rate);
        let explosion = e.mem.u32(record + 8);
        e.set(
            stage,
            DestructibleObjectStage::pExplosion,
            Ptr::new(explosion),
        );
        let debris = e.mem.u32(record + 0xc);
        e.set(stage, DestructibleObjectStage::pDebris, Ptr::new(debris));
        let count = e.mem.u32(record + 0x10);
        e.set(stage, DestructibleObjectStage::iDebrisCount, count);

        let mut buffers = vec![];
        loop {
            let chunk = e.call(0x0047_26b0, &args![file]).u32();
            if chunk == 0 {
                break;
            }
            let stage = stage_at(e, data, index as u32);
            match chunk {
                CHUNK_DSTF => break,
                CHUNK_DMDL => {
                    fn_00478570(e, stage);
                    let size = e.call(0x0040_1660, &args![file]).u32();
                    let buffer = e.mem.alloc(size);
                    buffers.push(buffer);
                    e.call(0x0047_2890, &args![file, buffer, 0u32]);
                    let stage = stage_at(e, data, index as u32);
                    let model = e.get(stage, DestructibleObjectStage::pReplacementModel);
                    e.vcall(model.addr(), 0x18, &args![buffer]);
                }
                CHUNK_DMDT => {
                    fn_00478570(e, stage);
                    let stage = stage_at(e, data, index as u32);
                    let model = e.get(stage, DestructibleObjectStage::pReplacementModel);
                    e.call(0x0048_93e0, &args![model, file]);
                }
                CHUNK_DMDS => {
                    fn_00478570(e, stage);
                    let size = e.call(0x0040_1660, &args![file]).u32();
                    let buffer = e.mem.alloc(size);
                    buffers.push(buffer);
                    e.call(0x0047_2890, &args![file, buffer, size]);
                    let version = e.call(0x0040_3570, &args![file]).u32() & 0xffff;
                    let stage = stage_at(e, data, index as u32);
                    let model = e.get(stage, DestructibleObjectStage::pReplacementModel);
                    e.call(0x0048_a7b0, &args![model, buffer, size, version]);
                }
                _ => {}
            }
            if e.call(0x0047_26f0, &args![file]).u8() == 0 {
                break;
            }
        }
        for buffer in buffers {
            e.mem.free(buffer);
        }
    }
    e.mem.free(record);
}

// Translated from 00478570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall `this` = a stage: creates its replacement model (0x20 bytes,
/// constructed by `0048a3d0`) when it has none.
pub fn fn_00478570(e: &mut Engine, this: Ptr<DestructibleObjectStage>) {
    if !e
        .get(this, DestructibleObjectStage::pReplacementModel)
        .is_null()
    {
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
    let model = if block != 0 {
        e.call(0x0048_a3d0, &args![block]).u32()
    } else {
        0
    };
    e.set(
        this,
        DestructibleObjectStage::pReplacementModel,
        Ptr::new(model),
    );
}

// Translated from 00478600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall `this` = the component: creates its data (0x14 bytes,
/// constructed by `004751d0`) when it has none.
pub fn fn_00478600(e: &mut Engine, this: Ptr<BGSDestructibleObjectForm>) {
    if !e.get(this, BGSDestructibleObjectForm::pData).is_null() {
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    let data = if block != 0 {
        fn_004751d0(e, Ptr::new(block))
    } else {
        Ptr::NULL
    };
    e.set(this, BGSDestructibleObjectForm::pData, data);
}

// Translated from 00478690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fastcall `this` = the component: deletes its data (the scalar deleting
/// destructor `004753a0` with the delete flag) and clears the pointer.
pub fn fn_00478690(e: &mut Engine, this: Ptr<BGSDestructibleObjectForm>) {
    let data = e.get(this, BGSDestructibleObjectForm::pData);
    if !data.is_null() {
        fn_004753a0(e, data, 1);
    }
    e.set(this, BGSDestructibleObjectForm::pData, Ptr::NULL);
}

// Translated from 004786e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the destruction data of another form into this component
/// (`this` = the component, `other` the other form): the other form is cast
/// (`__RTDynamicCast`, types `01183040` to `011832ac`); nothing happens
/// when that fails. Its component data is read with `00726070`: without
/// any, this component's data is deleted; otherwise this component's data
/// is created, or (when it exists) its stages are deleted (`004752e0`
/// with the delete flag) and its stage array freed; then health, stage
/// count and flags are copied, the stage array allocated
/// ([`fn_00478e90`]) and every stage copied ([`fn_00478fd0`]).
pub fn fn_004786e0(e: &mut Engine, this: Ptr<BGSDestructibleObjectForm>, other: Ptr) {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, 0x0118_3040u32, 0x0118_32acu32, 0u32],
        )
        .u32();
    if cast == 0 {
        return;
    }
    let source: Ptr<DestructibleObjectData> =
        Ptr::new(e.call(GET_COMPONENT_DATA, &args![cast]).u32());
    if source.is_null() {
        fn_00478690(e, this);
        return;
    }
    if e.get(this, BGSDestructibleObjectForm::pData).is_null() {
        let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
        let data = if block != 0 {
            fn_004751d0(e, Ptr::new(block))
        } else {
            Ptr::NULL
        };
        e.set(this, BGSDestructibleObjectForm::pData, data);
    } else {
        let mut index = 0u32;
        loop {
            let data = e.get(this, BGSDestructibleObjectForm::pData);
            if index >= e.get(data, DestructibleObjectData::cNumStages) as u32 {
                break;
            }
            let stage = stage_at(e, data, index);
            if !stage.is_null() {
                e.call(0x0047_52e0, &args![stage, 1u32]);
            }
            index += 1;
        }
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        let stages = e.get(data, DestructibleObjectData::pStagesArray);
        e.call(OPERATOR_DELETE, &args![stages]);
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        e.set(data, DestructibleObjectData::pStagesArray, Ptr::NULL);
    }
    let data = e.get(this, BGSDestructibleObjectForm::pData);
    let flags = e.get(source, DestructibleObjectData::cFlags);
    e.set(data, DestructibleObjectData::cFlags, flags);
    let health = e.get(source, DestructibleObjectData::iHealth);
    e.set(data, DestructibleObjectData::iHealth, health);
    let count = e.get(source, DestructibleObjectData::cNumStages);
    e.set(data, DestructibleObjectData::cNumStages, count);
    if count != 0 {
        let stages = fn_00478e90(e, this, count);
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        e.set(data, DestructibleObjectData::pStagesArray, stages);
        let mut index = 0u32;
        loop {
            let data = e.get(this, BGSDestructibleObjectForm::pData);
            if index >= e.get(data, DestructibleObjectData::cNumStages) as u32 {
                break;
            }
            let from = stage_at(e, source, index);
            let to = stage_at(e, data, index);
            fn_00478fd0(e, to, from);
            index += 1;
        }
    }
}

// Translated from 00478900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether another form's destruction data differs from this component's
/// (`other` is cast as in [`fn_004786e0`]): true when the cast fails, when
/// only one of the two has data, or when health, flags, stage count or any
/// stage differs. Per stage the compared fields are: percentage,
/// self damage per second, model damage stage, flags, explosion, debris,
/// the replacement model's presence, and the models themselves (virtual
/// slot 0x0C of this stage's model with the other's as argument, true when
/// it answers yes). The debris count is compared with itself (the code
/// reads the other form's stage on both sides), so it never differs.
pub fn fn_00478900(e: &mut Engine, this: Ptr<BGSDestructibleObjectForm>, other: Ptr) -> bool {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![other, 0u32, 0x0118_3040u32, 0x0118_32acu32, 0u32],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    let theirs: Ptr<DestructibleObjectData> =
        Ptr::new(e.call(GET_COMPONENT_DATA, &args![cast]).u32());
    let ours = e.get(this, BGSDestructibleObjectForm::pData);
    if theirs.is_null() || ours.is_null() {
        return !(theirs.is_null() && ours.is_null());
    }
    if e.get(ours, DestructibleObjectData::iHealth)
        != e.get(theirs, DestructibleObjectData::iHealth)
        || e.get(ours, DestructibleObjectData::cFlags)
            != e.get(theirs, DestructibleObjectData::cFlags)
        || e.get(ours, DestructibleObjectData::cNumStages)
            != e.get(theirs, DestructibleObjectData::cNumStages)
    {
        return true;
    }
    let mut index = 0u32;
    loop {
        let ours = e.get(this, BGSDestructibleObjectForm::pData);
        if index >= e.get(ours, DestructibleObjectData::cNumStages) as u32 {
            return false;
        }
        let a = stage_at(e, ours, index);
        let b = stage_at(e, theirs, index);
        if e.get(a, DestructibleObjectStage::cHealthPercentage)
            != e.get(b, DestructibleObjectStage::cHealthPercentage)
            || e.get(a, DestructibleObjectStage::iSelfDamagePerSecond)
                != e.get(b, DestructibleObjectStage::iSelfDamagePerSecond)
            || e.get(a, DestructibleObjectStage::cModelDamageStage)
                != e.get(b, DestructibleObjectStage::cModelDamageStage)
            || e.get(a, DestructibleObjectStage::cFlags)
                != e.get(b, DestructibleObjectStage::cFlags)
            || e.get(a, DestructibleObjectStage::pExplosion)
                != e.get(b, DestructibleObjectStage::pExplosion)
            || e.get(a, DestructibleObjectStage::pDebris)
                != e.get(b, DestructibleObjectStage::pDebris)
        {
            return true;
        }
        // The debris count of `b` against itself: never different.
        let model_a = e.get(a, DestructibleObjectStage::pReplacementModel);
        let model_b = e.get(b, DestructibleObjectStage::pReplacementModel);
        if model_a.is_null() != model_b.is_null() {
            return true;
        }
        if !model_a.is_null() && e.vcall(model_a.addr(), 0xc, &args![model_b]).u8() != 0 {
            return true;
        }
        index += 1;
    }
}

// Translated from 00478be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSDestructibleObjectForm::InitItem` (Xbox PDB), `this` = the
/// component, `owner` the form that holds it: for every stage turns the
/// explosion and debris form ids into forms (the id is made absolute with
/// `TESForm::AddCompileIndex` against `owner`'s file, looked up with
/// `004839c0` and cast; the explosion is cast to `0118620c`, the debris to
/// `011861f4`) and logs "MASTERFILE: Unable to find stage %i ..." when
/// nothing is found, naming the owner by its name (virtual slot 0x130,
/// when `00474cb0` says it has one), its form id, or as unknown when
/// `owner` is null. Calls `0048aa80(owner)` on every replacement model.
/// For an owner of form type 0x2A or 0x2B the data's health is then
/// `005f0b00(owner)` (`TESActorBase::GetHealth`).
pub fn bgs_destructible_object_form_init_item(
    e: &mut Engine,
    this: Ptr<BGSDestructibleObjectForm>,
    owner: Ptr,
) {
    if e.get(this, BGSDestructibleObjectForm::pData).is_null() {
        return;
    }
    let mut index = 0u32;
    loop {
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        if index >= e.get(data, DestructibleObjectData::cNumStages) as u32 {
            break;
        }
        let stage = stage_at(e, data, index);
        if !e.get(stage, DestructibleObjectStage::pExplosion).is_null() {
            resolve_stage_form(
                e,
                this,
                owner,
                index,
                8,
                0x0118_620c,
                [0x0101_a800, 0x0101_a898, 0x0101_a848],
            );
        }
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        let stage = stage_at(e, data, index);
        if !e.get(stage, DestructibleObjectStage::pDebris).is_null() {
            resolve_stage_form(
                e,
                this,
                owner,
                index,
                0xc,
                0x0118_61f4,
                [0x0101_a720, 0x0101_a7b8, 0x0101_a768],
            );
        }
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        let stage = stage_at(e, data, index);
        let model = e.get(stage, DestructibleObjectStage::pReplacementModel);
        if !model.is_null() {
            e.call(0x0048_aa80, &args![model, owner]);
        }
        index += 1;
    }
    let form_type = e.call(GET_FORM_TYPE, &args![owner]).u32() as i32;
    if (0x2a..=0x2b).contains(&form_type) {
        let health = e.call(0x005f_0b00, &args![owner]).u32();
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        e.set(data, DestructibleObjectData::iHealth, health);
    }
}

/// One form id of a stage of [`bgs_destructible_object_form_init_item`]
/// (the word at `offset` of the stage): made absolute, looked up, cast to
/// `target_type` and stored back; when the result is null the matching
/// message of `messages` (unknown owner, owner with a name, owner by form
/// id) is logged with the stage index and the id.
fn resolve_stage_form(
    e: &mut Engine,
    this: Ptr<BGSDestructibleObjectForm>,
    owner: Ptr,
    index: u32,
    offset: u32,
    target_type: u32,
    messages: [u32; 3],
) {
    let stage_word = |e: &Engine| {
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        stage_at(e, data, index).addr() + offset
    };
    let id = e.mem.alloc(4);
    let word = stage_word(e);
    let stored = e.mem.u32(word);
    e.mem.set_u32(id, stored);
    let file = e.call(0x0048_4e60, &args![owner, 0xffff_ffffu32]).u32();
    e.call(0x0048_5d50, &args![id, file]);
    let absolute = e.mem.u32(id);
    let form = e.call(0x0048_39c0, &args![absolute]).u32();
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, 0x0118_3028u32, target_type, 0u32],
        )
        .u32();
    let word = stage_word(e);
    e.mem.set_u32(word, cast);
    let word = stage_word(e);
    if e.mem.u32(word) == 0 {
        let absolute = e.mem.u32(id);
        if owner.is_null() {
            e.call(LOG_MESSAGE, &args![messages[0], index, absolute]);
        } else if e.call(0x0047_4cb0, &args![owner]).u32() != 0 {
            let name = e.vcall(owner.addr(), 0x130, &args![]).u32();
            e.call(LOG_MESSAGE, &args![messages[1], index, absolute, name]);
        } else {
            let form_id = e.call(GET_FORM_ID, &args![owner]).u32();
            e.call(LOG_MESSAGE, &args![messages[2], index, absolute, form_id]);
        }
    }
    e.mem.free(id);
}

// Translated from 00478e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thiscall (`this`, the component, is not read), one byte parameter:
/// allocates an array of `count` stage pointers and, for each, a stage
/// (0x18 bytes, constructed by [`fn_00478f70`]). Returns 0 for a count of
/// 0.
pub fn fn_00478e90(e: &mut Engine, _this: Ptr<BGSDestructibleObjectForm>, count: u8) -> Ptr {
    if count == 0 {
        return Ptr::NULL;
    }
    let array = e.call(OPERATOR_NEW, &args![count as u32 * 4]).u32();
    for index in 0..count as u32 {
        let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
        let stage = if block != 0 {
            fn_00478f70(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.mem.set_u32(array + index * 4, stage);
    }
    Ptr::new(array)
}

// Translated from 00478f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `DestructibleObjectStage` constructor (fastcall `this`): everything
/// zero (bytes 0 to 2, the words at +4 to +0x14; byte 3 is left alone).
/// Returns `this`.
pub fn fn_00478f70(
    e: &mut Engine,
    this: Ptr<DestructibleObjectStage>,
) -> Ptr<DestructibleObjectStage> {
    e.set(this, DestructibleObjectStage::cModelDamageStage, 0);
    e.set(this, DestructibleObjectStage::cHealthPercentage, 0);
    e.set(this, DestructibleObjectStage::cFlags, 0);
    e.set(this, DestructibleObjectStage::iSelfDamagePerSecond, 0);
    e.set(this, DestructibleObjectStage::pExplosion, Ptr::NULL);
    e.set(this, DestructibleObjectStage::pDebris, Ptr::NULL);
    e.set(this, DestructibleObjectStage::iDebrisCount, 0);
    e.set(this, DestructibleObjectStage::pReplacementModel, Ptr::NULL);
    this
}

// Translated from 00478fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies stage `from` into `this` (nothing when `from` is null): the
/// fields, then the replacement model: this stage's model is deleted
/// (virtual slot 0x10 with 1) and cleared; when `from` has one, a new model
/// is created (0x20 bytes, `0048a3d0`) and made a copy of it (virtual slot
/// 0x08 with the other model).
pub fn fn_00478fd0(
    e: &mut Engine,
    this: Ptr<DestructibleObjectStage>,
    from: Ptr<DestructibleObjectStage>,
) {
    if from.is_null() {
        return;
    }
    let percentage = e.get(from, DestructibleObjectStage::cHealthPercentage);
    e.set(this, DestructibleObjectStage::cHealthPercentage, percentage);
    let rate = e.get(from, DestructibleObjectStage::iSelfDamagePerSecond);
    e.set(this, DestructibleObjectStage::iSelfDamagePerSecond, rate);
    let damage_stage = e.get(from, DestructibleObjectStage::cModelDamageStage);
    e.set(
        this,
        DestructibleObjectStage::cModelDamageStage,
        damage_stage,
    );
    let flags = e.get(from, DestructibleObjectStage::cFlags);
    e.set(this, DestructibleObjectStage::cFlags, flags);
    let explosion = e.get(from, DestructibleObjectStage::pExplosion);
    e.set(this, DestructibleObjectStage::pExplosion, explosion);
    let debris = e.get(from, DestructibleObjectStage::pDebris);
    e.set(this, DestructibleObjectStage::pDebris, debris);
    let count = e.get(from, DestructibleObjectStage::iDebrisCount);
    e.set(this, DestructibleObjectStage::iDebrisCount, count);
    let old_model = e.get(this, DestructibleObjectStage::pReplacementModel);
    if !old_model.is_null() {
        e.vcall(old_model.addr(), 0x10, &args![1u32]);
        e.set(this, DestructibleObjectStage::pReplacementModel, Ptr::NULL);
    }
    let source_model = e.get(from, DestructibleObjectStage::pReplacementModel);
    if !source_model.is_null() {
        let block = e.call(OPERATOR_NEW, &args![0x20u32]).u32();
        let model = if block != 0 {
            e.call(0x0048_a3d0, &args![block]).u32()
        } else {
            0
        };
        e.set(
            this,
            DestructibleObjectStage::pReplacementModel,
            Ptr::new(model),
        );
        let model = e.get(this, DestructibleObjectStage::pReplacementModel);
        e.vcall(model.addr(), 0x8, &args![source_model]);
    }
}

// Translated from 00479110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the 16-byte `BSSimpleArray` of node pointers (fastcall
/// `this`): sets the vtable (`0101a8e8`) and calls `006b3eb0(0, 0)`.
/// Returns `this`.
pub fn fn_00479110(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0101_a8e8);
    e.call(0x006b_3eb0, &args![this, 0u32, 0u32]);
    this
}

// Translated from 00479140 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of that array (fastcall `this`): sets the vtable
/// (`0101a8e8`) and calls `008454f0(1)`.
pub fn fn_00479140(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_a8e8);
    e.call(0x0084_54f0, &args![this, 1u32]);
}

// Translated from 00479160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSimpleArray<NiNode *, 1024>::scalar deleting destructor` (Xbox PDB):
/// runs [`fn_00479140`] and frees the block when bit 0 of `flags` is set.
/// Returns `this`.
pub fn bs_simple_array_ni_node_p_1024_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00479140(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00479190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the self-damage map (`NiTMap` of references to
/// `unsigned int`): runs the base constructor [`fn_004791f0`] with `size`
/// (the number of buckets) and sets the vtable to `0101a8fc`. Returns
/// `this`.
pub fn fn_00479190(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    fn_004791f0(e, this, size);
    e.mem.set_u32(this.addr(), 0x0101_a8fc);
    this
}

// Translated from 004791c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<TESObjectREFR *, unsigned int>::scalar deleting destructor`
/// (Xbox PDB): runs the destructor body [`fn_00479260`] and frees the block
/// when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_tes_object_refr_p_unsigned_int_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00479260(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004791f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase` constructor (thiscall, `size` = bucket count): vtable
/// `0101a91c`, the count at +4, zero at +0x0C, the bucket array at +8
/// (`size * 4` bytes from `00aa1070`) cleared with `memset`. Returns
/// `this`.
pub fn fn_004791f0(e: &mut Engine, this: Ptr, size: u32) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0101_a91c);
    e.mem.set_u32(this.addr() + 4, size);
    e.mem.set_u32(this.addr() + 0xc, 0);
    let buckets = e.call(0x00aa_1070, &args![size << 2]).u32();
    e.mem.set_u32(this.addr() + 8, buckets);
    let buckets = e.mem.u32(this.addr() + 8);
    let size = e.mem.u32(this.addr() + 4) << 2;
    e.call(MEMSET, &args![buckets, 0u32, size]);
    this
}

// Translated from 00479260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of the self-damage map (fastcall `this`): vtable
/// `0101a8fc`, `00438af0` (empties the map), then the base destructor
/// [`fn_004792c0`]. (The compiler's exception frame is not translated.)
pub fn fn_00479260(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_a8fc);
    e.call(0x0043_8af0, &args![this]);
    fn_004792c0(e, this);
}

// Translated from 004792c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase` destructor body (fastcall `this`): vtable `0101a91c`,
/// `00438af0` (empties the map), then frees the bucket array at +8
/// (`00aa10f0`).
pub fn fn_004792c0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_a91c);
    e.call(0x0043_8af0, &args![this]);
    let buckets = e.mem.u32(this.addr() + 8);
    e.call(0x00aa_10f0, &args![buckets]);
}

// Translated from 004792f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<..., TESObjectREFR *, unsigned int>::scalar deleting
/// destructor` (Xbox PDB): runs the base destructor body [`fn_004792c0`]
/// and frees the block when bit 0 of `flags` is set. Returns `this`.
pub fn ni_t_map_base_tes_object_refr_p_unsigned_int_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_004792c0(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004751d0,
            fn_004751d0(Ptr<DestructibleObjectData>) -> Ptr<DestructibleObjectData>
        ),
        entry!(
            0x004753a0,
            fn_004753a0(Ptr<DestructibleObjectData>, u32) -> Ptr<DestructibleObjectData>
        ),
        entry!(
            0x004753d0,
            bgs_destructible_object_form_is_destructible(Ptr) -> bool
        ),
        entry!(
            0x00475400,
            bgs_destructible_object_form_get_destruction_form(Ptr) -> Ptr
        ),
        entry!(
            0x00475920,
            bgs_destructible_object_form_get_model_swap_index(Ptr, u32) -> u32
        ),
        entry!(0x004759a0, fn_004759a0(Ptr, i32) -> u32),
        entry!(
            0x00475a00,
            bgs_destructible_object_form_has_valid_stages(Ptr<BGSDestructibleObjectForm>) -> bool
        ),
        entry!(
            0x00475a90,
            fn_00475a90(Ptr<DestructibleObjectStage>) -> bool
        ),
        entry!(
            0x00475ab0,
            fn_00475ab0(Ptr<BGSDestructibleObjectForm>, Ptr) -> u32
        ),
        entry!(
            0x00475b20,
            fn_00475b20(Ptr<BGSDestructibleObjectForm>, Ptr, f32, u32)
        ),
        entry!(0x004768c0, ni_matrix3_transpose(Ptr, Ptr) -> Ptr),
        entry!(0x00476930, fn_00476930(Ptr, u32, Ptr)),
        entry!(0x00476980, fn_00476980(Ptr, Ptr, Ptr, Ptr) -> Ptr),
        entry!(0x004769c0, fn_004769c0(Ptr, u32, Ptr)),
        entry!(
            0x00476a00,
            fn_00476a00(Ptr<DestructibleObjectStage>) -> bool
        ),
        entry!(
            0x00476a20,
            fn_00476a20(Ptr<DestructibleObjectStage>) -> bool
        ),
        entry!(
            0x00476a40,
            fn_00476a40(Ptr<BGSDestructibleObjectForm>, u32) -> Ptr<DestructibleObjectStage>
        ),
        entry!(0x00476a80, fn_00476a80(Ptr) -> Ptr),
        entry!(0x00476ab0, fn_00476ab0(Ptr)),
        entry!(0x00476ae0, fn_00476ae0(Ptr, u8)),
        entry!(0x00476b00, fn_00476b00(Ptr, u8)),
        entry!(0x00476b20, fn_00476b20(f32) -> f32),
        entry!(0x00476b70, random_float(f32, f32) -> f32),
        entry!(0x00476b90, fn_00476b90(Ptr, f32, f32) -> f32),
        entry!(0x00476be0, fn_00476be0(Ptr) -> u32),
        entry!(0x00476c00, fn_00476c00() -> Ptr),
        entry!(0x00476c70, fn_00476c70(Ptr, u32)),
        entry!(0x00476c90, fn_00476c90(Ptr) -> u32),
        entry!(
            0x00476cb0,
            fn_00476cb0(Ptr<BGSDestructibleObjectForm>, Ptr, Ptr, u8, Ptr) -> i32
        ),
        entry!(
            0x00476dd0,
            bgs_destructible_object_form_spawn_debris_at_debris_nodes(
                Ptr<BGSDestructibleObjectForm>,
                Ptr,
                Ptr,
                u8,
                Ptr,
                u32,
            ) -> i32
        ),
        entry!(
            0x00476ec0,
            bgs_destructible_object_form_update_damage_stage_nodes(Ptr, u8)
        ),
        entry!(
            0x00476f50,
            bgs_destructible_object_form_update_current_damage_stage(
                Ptr<BGSDestructibleObjectForm>,
                Ptr,
                bool,
            ) -> bool
        ),
        entry!(0x00477090, fn_00477090(Ptr<BGSDestructibleObjectForm>, Ptr)),
        entry!(
            0x00477230,
            fn_00477230(Ptr<BGSDestructibleObjectForm>, Ptr, Ptr)
        ),
        entry!(0x004772e0, fn_004772e0() -> u32),
        entry!(0x004772f0, fn_004772f0()),
        entry!(0x00477410, fn_00477410(Ptr)),
        entry!(0x00477430, fn_00477430(Ptr) -> u32),
        entry!(
            0x00477560,
            bgs_destructible_object_form_hide_small_debris(Ptr) -> bool
        ),
        entry!(
            0x00477640,
            bgs_destructible_object_form_queue_files(Ptr<BGSDestructibleObjectForm>, u32, u32, u32)
        ),
        entry!(
            0x00477780,
            bgs_destructible_object_form_preload_replacement_models(
                Ptr<BGSDestructibleObjectForm>,
                u32,
            )
        ),
        entry!(0x004778a0, fn_004778a0(Ptr<BGSDestructibleObjectForm>)),
        entry!(0x00477900, bgs_explosion_get_radius_bs_units(Ptr) -> f32),
        entry!(
            0x00477970,
            fn_00477970(Ptr<BGSDestructibleObjectForm>, u8, f32, f32, Ptr) -> f32
        ),
        entry!(
            0x00477a50,
            fn_00477a50(Ptr<BGSDestructibleObjectForm>, u8, f32, f32, Ptr) -> f32
        ),
        entry!(
            0x00477ce0,
            bgs_destructible_object_form_set_self_damage(Ptr, u32)
        ),
        entry!(0x00477d10, fn_00477d10(Ptr)),
        entry!(
            0x00477f20,
            bgs_destructible_object_form_save(Ptr<BGSDestructibleObjectForm>)
        ),
        entry!(0x00478130, fn_00478130(Ptr)),
        entry!(0x004781b0, fn_004781b0(Ptr) -> Ptr),
        entry!(
            0x004781e0,
            bgs_destructible_object_form_load_chunk(Ptr<BGSDestructibleObjectForm>, Ptr)
        ),
        entry!(0x00478570, fn_00478570(Ptr<DestructibleObjectStage>)),
        entry!(0x00478600, fn_00478600(Ptr<BGSDestructibleObjectForm>)),
        entry!(0x00478690, fn_00478690(Ptr<BGSDestructibleObjectForm>)),
        entry!(0x004786e0, fn_004786e0(Ptr<BGSDestructibleObjectForm>, Ptr)),
        entry!(
            0x00478900,
            fn_00478900(Ptr<BGSDestructibleObjectForm>, Ptr) -> bool
        ),
        entry!(
            0x00478be0,
            bgs_destructible_object_form_init_item(Ptr<BGSDestructibleObjectForm>, Ptr)
        ),
        entry!(
            0x00478e90,
            fn_00478e90(Ptr<BGSDestructibleObjectForm>, u8) -> Ptr
        ),
        entry!(
            0x00478f70,
            fn_00478f70(Ptr<DestructibleObjectStage>) -> Ptr<DestructibleObjectStage>
        ),
        entry!(
            0x00478fd0,
            fn_00478fd0(Ptr<DestructibleObjectStage>, Ptr<DestructibleObjectStage>)
        ),
        entry!(0x00479110, fn_00479110(Ptr) -> Ptr),
        entry!(0x00479140, fn_00479140(Ptr)),
        entry!(
            0x00479160,
            bs_simple_array_ni_node_p_1024_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00479190, fn_00479190(Ptr, u32) -> Ptr),
        entry!(
            0x004791c0,
            ni_t_map_tes_object_refr_p_unsigned_int_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x004791f0, fn_004791f0(Ptr, u32) -> Ptr),
        entry!(0x00479260, fn_00479260(Ptr)),
        entry!(0x004792c0, fn_004792c0(Ptr)),
        entry!(
            0x004792f0,
            ni_t_map_base_tes_object_refr_p_unsigned_int_scalar_deleting_destructor(Ptr, u32)
                -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn noop(e: &mut Engine, addresses: &[u32]) {
        for &address in addresses {
            e.register(address, |_, _| Ret::default());
        }
    }

    /// An engine with the exe's constants the code reads mapped and set.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_2000u32,
            0x0101_7000,
            0x0101_a000,
            0x011c_4000,
            0x011c_6000,
            0x011c_3000,
            0x011c_e000,
            0x011c_f000,
            0x011f_6000,
            0x011f_1000,
            0x011d_d000,
            0x011d_e000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO, 0.0f64);
        e.set_global(ONE, 1.0f64);
        e.set_global(HUNDRED, 100.0f64);
        e.set_global(NO_HEALTH, -1.0f64);
        e.set_global(TWO_POW_32, 4294967296.0f64);
        e.set_global(MINUS_ONE, -1.0f32);
        // `00404090`: truncate toward zero.
        e.register(TRUNCATE_FLOAT, |_, a| {
            f32::from_bits(a[0]).trunc().into_ret()
        });
        e
    }

    /// An object of `size` bytes whose vtable has the given slots (byte
    /// offset, function address).
    fn object_with_vtable(e: &mut Engine, size: u32, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(size);
        let table = e.mem.alloc(0x400);
        e.mem.set_u32(object, table);
        for &(offset, function) in slots {
            e.mem.set_u32(table + offset, function);
        }
        object
    }

    #[derive(Clone, Copy, Default)]
    struct StageSpec {
        damage_stage: u8,
        percentage: u8,
        flags: u8,
        rate: u32,
        explosion: u32,
        debris: u32,
        count: u32,
        model: u32,
    }

    /// A component with data of total health `health` and the stages.
    fn make_component(
        e: &mut Engine,
        health: u32,
        stages: &[StageSpec],
    ) -> (Ptr<BGSDestructibleObjectForm>, Ptr<DestructibleObjectData>) {
        let component: Ptr<BGSDestructibleObjectForm> = e.new_object();
        let data: Ptr<DestructibleObjectData> = e.new_object();
        e.set(component, BGSDestructibleObjectForm::pData, data);
        e.set(data, DestructibleObjectData::iHealth, health);
        e.set(data, DestructibleObjectData::cNumStages, stages.len() as u8);
        let array = e.mem.alloc(4 * stages.len() as u32 + 4);
        e.set(data, DestructibleObjectData::pStagesArray, Ptr::new(array));
        for (i, spec) in stages.iter().enumerate() {
            let stage: Ptr<DestructibleObjectStage> = e.new_object();
            e.set(
                stage,
                DestructibleObjectStage::cModelDamageStage,
                spec.damage_stage,
            );
            e.set(
                stage,
                DestructibleObjectStage::cHealthPercentage,
                spec.percentage,
            );
            e.set(stage, DestructibleObjectStage::cFlags, spec.flags);
            e.set(
                stage,
                DestructibleObjectStage::iSelfDamagePerSecond,
                spec.rate,
            );
            e.set(
                stage,
                DestructibleObjectStage::pExplosion,
                Ptr::new(spec.explosion),
            );
            e.set(
                stage,
                DestructibleObjectStage::pDebris,
                Ptr::new(spec.debris),
            );
            e.set(stage, DestructibleObjectStage::iDebrisCount, spec.count);
            e.set(
                stage,
                DestructibleObjectStage::pReplacementModel,
                Ptr::new(spec.model),
            );
            e.mem.set_u32(array + 4 * i as u32, stage.addr());
        }
        (component, data)
    }

    fn stage_pointer(
        e: &Engine,
        data: Ptr<DestructibleObjectData>,
        i: u32,
    ) -> Ptr<DestructibleObjectStage> {
        stage_at(e, data, i)
    }

    #[test]
    fn data_constructor_zeroes_the_fields_and_constructs_the_preloaded_pointer() {
        let mut e = engine();
        e.register(0x0052_8cb0, |_, _| Ret::default());
        let data: Ptr<DestructibleObjectData> = e.new_object();
        e.mem.write(data.addr(), &[0xff; 0x14]);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x0047_51d0, &args![data])
            .ptr::<DestructibleObjectData>();
        assert_eq!(back, data);
        assert_eq!(e.get(data, DestructibleObjectData::iHealth), 0);
        assert_eq!(e.get(data, DestructibleObjectData::cNumStages), 0);
        assert_eq!(e.get(data, DestructibleObjectData::cFlags), 0);
        assert!(e.get(data, DestructibleObjectData::pStagesArray).is_null());
        assert_eq!(
            e.get(data, DestructibleObjectData::iReplacementModelRefCount),
            0
        );
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0052_8cb0, vec![data.addr() + 0x10, 0])));
    }

    #[test]
    fn data_destructor_frees_only_when_asked() {
        let mut e = engine();
        noop(&mut e, &[0x0047_5220, 0x0040_1030]);
        let data: Ptr<DestructibleObjectData> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_53a0, &args![data, 0u32]).u32(), data.addr());
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0047_5220, vec![data.addr()])));
        assert!(!log.iter().any(|(address, _)| *address == 0x0040_1030));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_53a0, &args![data, 1u32]).u32(), data.addr());
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0040_1030, vec![data.addr()])));
    }

    /// `00401170` reads the form type byte at +4.
    fn form_type_double(e: &mut Engine) {
        e.register(GET_FORM_TYPE, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register(GET_COMPONENT_DATA, |e, a| ret(e.mem.u32(a[0] + 4)));
    }

    fn form_of_type(e: &mut Engine, form_type: u8) -> u32 {
        let form = e.mem.alloc(0x200);
        e.mem.set_u8(form + 4, form_type);
        form
    }

    #[test]
    fn destruction_form_offset_depends_on_the_form_type() {
        let mut e = engine();
        form_type_double(&mut e);
        // (form type, offset of the component from the form)
        let table: [(u8, i32); 22] = [
            (0x15, 0x68),
            (0x16, 0x68),
            (0x17, 0x68),
            (0x18, 0x14c),
            (0x19, 0x9c),
            (0x1a, 0x144),
            (0x1b, 0x7c),
            (0x1c, 0x68),
            (0x1d, 0x94),
            (0x1e, 0x94),
            (0x1f, 0x84),
            (0x22, -0x14 + 0xc),
            (0x25, 0x54),
            (0x26, -0xc + 0x74),
            (0x27, 0x68),
            (0x28, 0xb4),
            (0x29, 0x88),
            (0x2a, 0x104),
            (0x2b, 0x104),
            (0x2e, 0x84),
            (0x2f, 0xa4),
            (0x33, 0x58),
        ];
        for (form_type, offset) in table {
            let form = form_of_type(&mut e, form_type);
            let got = e.call(0x0047_5400, &args![form]).u32();
            assert_eq!(got, form.wrapping_add(offset as u32), "type {form_type:#x}");
        }
        for form_type in [0x32u8, 0x67] {
            let form = form_of_type(&mut e, form_type);
            assert_eq!(e.call(0x0047_5400, &args![form]).u32(), form + 0x84);
        }
        // Types outside the table and a null form have no component.
        for form_type in [0x14u8, 0x20, 0x30, 0x68, 0xff] {
            let form = form_of_type(&mut e, form_type);
            assert_eq!(e.call(0x0047_5400, &args![form]).u32(), 0);
        }
        assert_eq!(e.call(0x0047_5400, &args![0u32]).u32(), 0);
    }

    #[test]
    fn is_destructible_needs_a_component_with_a_valid_stage() {
        let mut e = engine();
        form_type_double(&mut e);
        let form = form_of_type(&mut e, 0x15);
        // No data at all.
        assert!(!e.call(0x0047_53d0, &args![form]).bool());
        let (component, _) = make_component(
            &mut e,
            100,
            &[StageSpec {
                model: 0x1000,
                ..StageSpec::default()
            }],
        );
        let data = e.get(component, BGSDestructibleObjectForm::pData);
        e.mem.set_u32(form + 0x68 + 4, data.addr());
        assert!(e.call(0x0047_53d0, &args![form]).bool());
        // A type without a component, and a null form.
        let other = form_of_type(&mut e, 0x14);
        assert!(!e.call(0x0047_53d0, &args![other]).bool());
        assert!(!e.call(0x0047_53d0, &args![0u32]).bool());
    }

    #[test]
    fn model_swap_index_finds_the_stage_with_the_model() {
        let mut e = engine();
        form_type_double(&mut e);
        let form = form_of_type(&mut e, 0x15);
        let (component, data) = make_component(
            &mut e,
            100,
            &[
                StageSpec {
                    model: 0x1000,
                    ..StageSpec::default()
                },
                StageSpec {
                    model: 0x2000,
                    ..StageSpec::default()
                },
            ],
        );
        let _ = component;
        e.mem.set_u32(form + 0x68 + 4, data.addr());
        assert_eq!(e.call(0x0047_5920, &args![form, 0x2000u32]).u32(), 1);
        assert_eq!(e.call(0x0047_5920, &args![form, 0x1000u32]).u32(), 0);
        assert_eq!(e.call(0x0047_5920, &args![form, 0x3000u32]).u32(), u32::MAX);
        // No component for the type, no data for the component.
        let other = form_of_type(&mut e, 0x14);
        assert_eq!(
            e.call(0x0047_5920, &args![other, 0x1000u32]).u32(),
            u32::MAX
        );
        let empty = form_of_type(&mut e, 0x16);
        assert_eq!(
            e.call(0x0047_5920, &args![empty, 0x1000u32]).u32(),
            u32::MAX
        );
    }

    #[test]
    fn model_at_stage_checks_the_index() {
        let mut e = engine();
        form_type_double(&mut e);
        let form = form_of_type(&mut e, 0x15);
        let (_, data) = make_component(
            &mut e,
            100,
            &[
                StageSpec {
                    model: 0x1000,
                    ..StageSpec::default()
                },
                StageSpec {
                    model: 0x2000,
                    ..StageSpec::default()
                },
            ],
        );
        e.mem.set_u32(form + 0x68 + 4, data.addr());
        assert_eq!(e.call(0x0047_59a0, &args![form, 1i32]).u32(), 0x2000);
        assert_eq!(e.call(0x0047_59a0, &args![form, 0i32]).u32(), 0x1000);
        // Past the last stage, and -1.
        assert_eq!(e.call(0x0047_59a0, &args![form, 2i32]).u32(), 0);
        assert_eq!(e.call(0x0047_59a0, &args![form, -1i32]).u32(), 0);
        // No component, no data.
        let other = form_of_type(&mut e, 0x14);
        assert_eq!(e.call(0x0047_59a0, &args![other, 0i32]).u32(), 0);
        let empty = form_of_type(&mut e, 0x16);
        assert_eq!(e.call(0x0047_59a0, &args![empty, 0i32]).u32(), 0);
    }

    #[test]
    fn has_valid_stages_looks_for_something_to_do() {
        let mut e = engine();
        let empty: Ptr<BGSDestructibleObjectForm> = e.new_object();
        assert!(!e.call(0x0047_5a00, &args![empty]).bool());
        let nothing = StageSpec::default();
        let (component, _) = make_component(&mut e, 100, &[nothing, nothing]);
        assert!(!e.call(0x0047_5a00, &args![component]).bool());
        for spec in [
            StageSpec {
                model: 1,
                ..nothing
            },
            StageSpec {
                explosion: 1,
                ..nothing
            },
            StageSpec {
                debris: 1,
                ..nothing
            },
            StageSpec {
                flags: STAGE_FLAG_DISABLE_OBJECT,
                ..nothing
            },
        ] {
            let (component, _) = make_component(&mut e, 100, &[nothing, spec]);
            assert!(e.call(0x0047_5a00, &args![component]).bool());
        }
        // The other two flags do not make a stage valid, and a null entry
        // is skipped.
        let (component, data) = make_component(
            &mut e,
            100,
            &[
                StageSpec {
                    flags: STAGE_FLAG_CAP_DAMAGE | STAGE_FLAG_DESTROY_OBJECT,
                    ..nothing
                },
                nothing,
            ],
        );
        let first = stage_pointer(&e, data, 0);
        let array = e.get(data, DestructibleObjectData::pStagesArray).addr();
        e.mem.set_u32(array + 4, 0);
        assert!(!e.call(0x0047_5a00, &args![component]).bool());
        let _ = first;
    }

    #[test]
    fn stage_flag_tests() {
        let mut e = engine();
        for (flag, address) in [
            (STAGE_FLAG_DISABLE_OBJECT, 0x0047_5a90u32),
            (STAGE_FLAG_CAP_DAMAGE, 0x0047_6a00),
            (STAGE_FLAG_DESTROY_OBJECT, 0x0047_6a20),
        ] {
            let with: Ptr<DestructibleObjectStage> = e.new_object();
            e.set(with, DestructibleObjectStage::cFlags, flag | 0x80);
            assert!(e.call(address, &args![with]).bool());
            let without: Ptr<DestructibleObjectStage> = e.new_object();
            e.set(without, DestructibleObjectStage::cFlags, 7 & !flag);
            assert!(!e.call(address, &args![without]).bool());
        }
    }

    #[test]
    fn total_health_comes_from_the_actor_value_or_the_data() {
        let mut e = engine();
        let (component, _) = make_component(&mut e, 123, &[]);
        let actor_slot = 0x0f00_0001u32;
        let value_slot = 0x0f00_0002u32;
        e.register(actor_slot, |e, a| ret(e.mem.u32(a[0] + 0x150)));
        e.register(value_slot, |_, a| {
            assert_eq!(a[1], 0x10);
            250.7f32.into_ret()
        });
        let reference = object_with_vtable(&mut e, 0x200, &[(0x100, actor_slot)]);
        let interface_table = e.mem.alloc(0x40);
        e.mem.set_u32(interface_table + 4, value_slot);
        e.mem.set_u32(reference + 0xa4, interface_table);
        // Not an actor: the data's health.
        assert_eq!(e.call(0x0047_5ab0, &args![component, reference]).u32(), 123);
        // An actor: the interface's value, truncated.
        e.mem.set_u32(reference + 0x150, 1);
        assert_eq!(e.call(0x0047_5ab0, &args![component, reference]).u32(), 250);
    }

    #[test]
    fn transpose_swaps_rows_and_columns() {
        let mut e = engine();
        e.register(0x0040_1050, |_, _| Ret::default());
        let matrix = e.mem.alloc(0x24);
        for i in 0..9u32 {
            e.mem.set_f32(matrix + 4 * i, (i + 1) as f32);
        }
        let result = e.mem.alloc(0x24);
        e.call_log = Some(vec![]);
        let back = e.call(0x0047_68c0, &args![matrix, result]).u32();
        assert_eq!(back, result);
        let log = e.call_log.take().unwrap();
        let constructor = log
            .iter()
            .find(|(address, _)| *address == 0x0040_1050)
            .unwrap();
        assert_eq!(&constructor.1[1..], &[12, 3, 0x0068_15c0]);
        let got: Vec<f32> = (0..9).map(|i| e.mem.f32(result + 4 * i)).collect();
        assert_eq!(got, [1.0, 4.0, 7.0, 2.0, 5.0, 8.0, 3.0, 6.0, 9.0]);
    }

    #[test]
    fn row_copy_and_column_store() {
        let mut e = engine();
        let matrix = e.mem.alloc(0x24);
        for i in 0..9u32 {
            e.mem.set_f32(matrix + 4 * i, (i + 1) as f32);
        }
        let out = e.mem.alloc(12);
        e.call(0x0047_6930, &args![matrix, 2u32, out]);
        assert_eq!(
            [e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(out + 8)],
            [7.0, 8.0, 9.0]
        );
        // Column 1 of a fresh matrix.
        let target = e.mem.alloc(0x24);
        e.call(0x0047_69c0, &args![target, 1u32, out]);
        assert_eq!(
            [
                e.mem.f32(target + 4),
                e.mem.f32(target + 16),
                e.mem.f32(target + 28)
            ],
            [7.0, 8.0, 9.0]
        );
        // Three columns.
        let a = e.mem.alloc(12);
        let b = e.mem.alloc(12);
        let c = e.mem.alloc(12);
        for i in 0..3u32 {
            e.mem.set_f32(a + 4 * i, 1.0 + i as f32);
            e.mem.set_f32(b + 4 * i, 4.0 + i as f32);
            e.mem.set_f32(c + 4 * i, 7.0 + i as f32);
        }
        let all = e.mem.alloc(0x24);
        assert_eq!(e.call(0x0047_6980, &args![all, a, b, c]).u32(), all);
        let got: Vec<f32> = (0..9).map(|i| e.mem.f32(all + 4 * i)).collect();
        assert_eq!(got, [1.0, 4.0, 7.0, 2.0, 5.0, 8.0, 3.0, 6.0, 9.0]);
    }

    #[test]
    fn stage_lookup_is_bounded_by_the_count() {
        let mut e = engine();
        let (component, data) = make_component(&mut e, 100, &[StageSpec::default(); 2]);
        let expected = stage_pointer(&e, data, 1);
        assert_eq!(
            e.call(0x0047_6a40, &args![component, 1u32]).u32(),
            expected.addr()
        );
        assert_eq!(e.call(0x0047_6a40, &args![component, 2u32]).u32(), 0);
        let empty: Ptr<BGSDestructibleObjectForm> = e.new_object();
        assert_eq!(e.call(0x0047_6a40, &args![empty, 0u32]).u32(), 0);
    }

    #[test]
    fn transform_constructors_run_on_both_members() {
        let mut e = engine();
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        let transform = e.mem.alloc(0x34);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_6a80, &args![transform]).u32(), transform);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0068_15c0, vec![transform])));
        assert!(log.contains(&(0x0068_15c0, vec![transform + 0x24])));
    }

    #[test]
    fn flag_setters_forward_to_the_model_routine() {
        let mut e = engine();
        e.register(0x0043_b370, |_, _| Ret::default());
        let object = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        e.call(0x0047_6ab0, &args![object]);
        assert_eq!(e.mem.f32(object + 0xb4), 1.0);
        assert_eq!(e.mem.f32(object + 0xb8), 1.0);
        e.call(0x0047_6ae0, &args![object, 0u8]);
        e.call(0x0047_6b00, &args![object, 1u8]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(address, _)| *address == 0x0043_b370)
            .map(|(_, a)| a.clone())
            .collect();
        assert_eq!(
            calls,
            vec![
                vec![object, 1, 0x4000],
                vec![object, 0, 0x4000],
                vec![object, 1, 0x4000]
            ]
        );
    }

    #[test]
    fn round_up_adds_one_below_the_value() {
        let mut e = engine();
        for (value, expected) in [
            (2.5f32, 3.0f32),
            (3.0, 3.0),
            (-2.5, -2.0),
            (0.0001, 1.0),
            (-0.5, 0.0),
            (0.0, 0.0),
        ] {
            let got = e.call(0x0047_6b20, &args![value]).f32();
            assert_eq!(got, expected, "ceil of {value}");
        }
    }

    #[test]
    fn round_up_of_nan_adds_one_like_the_exe() {
        // The exe's `00404090` gives 0.0 for NaN; the compare is then
        // unordered, which C0 counts as "below" (oracle check, #189).
        let mut e = engine();
        e.register(TRUNCATE_FLOAT, |_, a| {
            let v = f32::from_bits(a[0]);
            (if v.is_nan() { 0.0f32 } else { v.trunc() }).into_ret()
        });
        for bits in [0x7fc0_0000u32, 0xffc0_0000, 0x7fc1_2345, 0x7f80_0001] {
            let got = e.call(0x0047_6b20, &args![f32::from_bits(bits)]).f32();
            assert_eq!(got, 1.0, "round up of NaN {bits:#010x}");
        }
    }

    #[test]
    fn random_generator_is_constructed_once() {
        let mut e = engine();
        e.register_double(0x00aa_5140, {
            let mut count = 0u32;
            move |_, a| {
                assert_eq!(a[0], RANDOM_GENERATOR);
                count += 1;
                ret(count)
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_6c00, &args![]).u32(), RANDOM_GENERATOR);
        assert_eq!(e.call(0x0047_6c00, &args![]).u32(), RANDOM_GENERATOR);
        let log = e.call_log.take().unwrap();
        let constructions = log
            .iter()
            .filter(|(address, _)| *address == 0x00aa_5140)
            .count();
        assert_eq!(constructions, 1);
        assert_eq!(e.global::<u32>(RANDOM_GENERATOR_INITIALIZED) & 1, 1);
    }

    #[test]
    fn random_word_uses_the_full_range() {
        let mut e = engine();
        e.register(0x00aa_5230, |_, a| {
            assert_eq!(a, &[0x1234, u32::MAX]);
            ret(0xc000_0000)
        });
        assert_eq!(e.call(0x0047_6be0, &args![0x1234u32]).u32(), 0xc000_0000);
    }

    #[test]
    fn random_float_scales_the_word_into_the_range() {
        let mut e = engine();
        e.register(0x00aa_5140, |_, _| Ret::default());
        e.register(0x00aa_5230, |_, _| ret(0xc000_0000));
        // The method form on an explicit generator.
        let value = e
            .call(0x0047_6b90, &args![RANDOM_GENERATOR, 10.0f32, 20.0f32])
            .f32();
        assert_eq!(value, 17.5);
        // The cdecl form draws from the static generator.
        let value = e.call(0x0047_6b70, &args![-1.0f32, 1.0f32]).f32();
        assert_eq!(value, 0.5);
        assert_eq!(e.global::<u32>(RANDOM_GENERATOR_INITIALIZED) & 1, 1);
    }

    #[test]
    fn explosion_word_accessors() {
        let mut e = engine();
        let object = e.mem.alloc(0x100);
        e.mem.set_u32(object + 0xf8, 0xabcd);
        assert_eq!(e.call(0x0047_6c90, &args![object]).u32(), 0xabcd);
        e.call(0x0047_6c70, &args![object, 0x77u32]);
        assert_eq!(e.mem.u32(object + 0xe0), 0x77);
    }

    // ---- node trees --------------------------------------------------

    /// Virtual slot 0xc of a node: its container (+8).
    const NODE_CONTAINER: u32 = 0x0f00_0010;

    /// Doubles for the node-walking helpers. A node keeps a bit mask at
    /// +0x20 of the types it is a checked cast to (1: `01202e7c`, 2:
    /// `01202e84`, 4: `01202e8c`), a flag at +0x24 for `SetCurrent`, its
    /// transform at +0x30 and its translation at +0x80. A container keeps
    /// the child count at +0 and the children from +4.
    fn node_doubles(e: &mut Engine) {
        e.register(NODE_CONTAINER, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(GET_CHILD_COUNT, |e, a| ret(e.mem.u32(a[0])));
        e.register(GET_CHILD_AT, |e, a| ret(e.mem.u32(a[0] + 4 + 4 * a[1])));
        e.register(CHECKED_CAST, |e, a| {
            let bit = match a[0] {
                0x0120_2e7c => 1,
                0x0120_2e84 => 2,
                _ => 4,
            };
            ret((e.mem.u32(a[1] + 0x20) & bit != 0) as u32)
        });
        e.register(RANGE_NODE_SET_CURRENT, |e, a| ret(e.mem.u32(a[0] + 0x24)));
        e.register(GET_TRANSFORM, |_, a| ret(a[0] + 0x30));
        e.register(GET_TRANSLATION, |_, a| ret(a[0] + 0x80));
        e.register(GET_REFERENCE_WORD, |_, _| ret(0x1234));
        e.register(GET_SCALE, |_, _| 1.5f32.into_ret());
    }

    fn make_node(e: &mut Engine, cast_mask: u32, accepts: bool, children: &[u32]) -> u32 {
        let node = object_with_vtable(e, 0x100, &[(0xc, NODE_CONTAINER)]);
        e.mem.set_u32(node + 0x20, cast_mask);
        e.mem.set_u32(node + 0x24, accepts as u32);
        for i in 0..3u32 {
            e.mem.set_f32(node + 0x80 + 4 * i, 1.0 + i as f32);
        }
        for i in 0..13u32 {
            e.mem.set_u32(node + 0x30 + 4 * i, 100 + i);
        }
        if !children.is_empty() {
            let container = e.mem.alloc(0x80);
            e.mem.set_u32(container, children.len() as u32);
            for (i, child) in children.iter().enumerate() {
                e.mem.set_u32(container + 4 + 4 * i as u32, *child);
            }
            e.mem.set_u32(node + 8, container);
        }
        node
    }

    #[test]
    fn explosions_are_created_at_the_accepting_nodes() {
        let mut e = engine();
        node_doubles(&mut e);
        noop(&mut e, &[CREATE_EXPLOSION]);
        let hit = make_node(&mut e, 1, true, &[]);
        let rejected = make_node(&mut e, 1, false, &[]);
        let other_type = make_node(&mut e, 2, true, &[]);
        let root = make_node(&mut e, 1, true, &[rejected, hit, other_type]);
        let this: Ptr<BGSDestructibleObjectForm> = e.new_object();
        let reference = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        let created = e
            .call(0x0047_6cb0, &args![this, root, 0x7000u32, 3u32, reference])
            .i32();
        assert_eq!(created, 2);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == CREATE_EXPLOSION).collect();
        assert_eq!(calls.len(), 2);
        // The explosion, the reference twice, the reference's word, the
        // node's translation (3 words) and transform (9 words).
        let args_of_root = &calls[0].1;
        assert_eq!(args_of_root.len(), 16);
        assert_eq!(&args_of_root[..4], &[0x7000, reference, reference, 0x1234]);
        assert_eq!(
            &args_of_root[4..7],
            &[1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
        );
        assert_eq!(
            &args_of_root[7..],
            &[100, 101, 102, 103, 104, 105, 106, 107, 108]
        );
        // The selector byte reached `SetCurrent` for every cast node.
        let selectors: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == RANGE_NODE_SET_CURRENT)
            .map(|(_, a)| (a[0], a[1]))
            .collect();
        assert_eq!(selectors, vec![(root, 3), (rejected, 3), (hit, 3)]);
        // No node, no explosions.
        assert_eq!(
            e.call(0x0047_6cb0, &args![this, 0u32, 0x7000u32, 3u32, reference])
                .i32(),
            0
        );
    }

    #[test]
    fn debris_is_spawned_at_the_debris_nodes() {
        let mut e = engine();
        node_doubles(&mut e);
        noop(&mut e, &[SPAWN_DEBRIS]);
        let hit = make_node(&mut e, 2, true, &[]);
        let rejected = make_node(&mut e, 2, false, &[]);
        let root = make_node(&mut e, 0, true, &[rejected, hit]);
        let this: Ptr<BGSDestructibleObjectForm> = e.new_object();
        let reference = e.mem.alloc(0x100);
        e.call_log = Some(vec![]);
        let spawned = e
            .call(
                0x0047_6dd0,
                &args![this, root, 0x7100u32, 5u32, reference, 4u32],
            )
            .i32();
        assert_eq!(spawned, 1);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == SPAWN_DEBRIS).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(
            calls[0].1,
            vec![0x7100, 0x1234, hit + 0x80, 4, 1.5f32.to_bits()]
        );
        assert_eq!(
            e.call(
                0x0047_6dd0,
                &args![this, 0u32, 0x7100u32, 5u32, reference, 4u32]
            )
            .i32(),
            0
        );
    }

    #[test]
    fn damage_stage_nodes_get_the_stage() {
        let mut e = engine();
        node_doubles(&mut e);
        let leaf = make_node(&mut e, 4, true, &[]);
        let skipped = make_node(&mut e, 1, true, &[]);
        let root = make_node(&mut e, 4, true, &[skipped, leaf]);
        e.call_log = Some(vec![]);
        e.call(0x0047_6ec0, &args![root, 3u8]);
        e.call(0x0047_6ec0, &args![0u32, 3u8]);
        let log = e.call_log.take().unwrap();
        let set: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == RANGE_NODE_SET_CURRENT)
            .map(|(_, a)| a.clone())
            .collect();
        assert_eq!(set, vec![vec![root, 3], vec![leaf, 3]]);
    }

    // ---- the reference the damage routines work on ------------------

    const REFERENCE_GET_3D: u32 = 0x0f00_0001;
    const REFERENCE_IS_ACTOR: u32 = 0x0f00_0002;
    const REFERENCE_SLOT_224: u32 = 0x0f00_0003;
    const REFERENCE_POSITION: u32 = 0x0f00_0004;
    const REFERENCE_NOOP: u32 = 0x0f00_0005;
    const REFERENCE_GET_VALUE: u32 = 0x0f00_0006;
    const REFERENCE_SLOT_338: u32 = 0x0f00_0007;
    const REFERENCE_SLOT_144: u32 = 0x0f00_0008;

    /// The doubles of the reference's virtual slots and of the extra data:
    /// +0x150 the 3D, +0x154 the actor flag, +0x158 the answer of slot
    /// 0x224, +0x160 the position, +0x180 the extra data list holding the
    /// health as a `float`.
    fn reference_doubles(e: &mut Engine) {
        e.register(REFERENCE_GET_3D, |e, a| ret(e.mem.u32(a[0] + 0x150)));
        e.register(REFERENCE_IS_ACTOR, |e, a| ret(e.mem.u32(a[0] + 0x154)));
        e.register(REFERENCE_SLOT_224, |e, a| ret(e.mem.u32(a[0] + 0x158)));
        e.register(REFERENCE_POSITION, |_, a| ret(a[0] + 0x160));
        e.register(REFERENCE_NOOP, |_, _| Ret::default());
        e.register(REFERENCE_GET_VALUE, |e, a| ret(e.mem.u32(a[0] + 0x15c)));
        e.register(GET_EXTRA_LIST, |_, a| ret(a[0] + 0x180));
        e.register(GET_OBJECT_HEALTH, |e, a| e.mem.f32(a[0]).into_ret());
        e.register(SET_OBJECT_HEALTH, |e, a| {
            e.mem.set_f32(a[0], f32::from_bits(a[1]));
            Ret::default()
        });
    }

    fn make_reference(e: &mut Engine, three_d: u32, health: f32) -> u32 {
        let reference = object_with_vtable(
            e,
            0x200,
            &[
                (0x1d0, REFERENCE_GET_3D),
                (0x100, REFERENCE_IS_ACTOR),
                (0x224, REFERENCE_SLOT_224),
                (0x1f4, REFERENCE_POSITION),
                (0x48, REFERENCE_NOOP),
                (0x1cc, REFERENCE_NOOP),
                (0x338, REFERENCE_SLOT_338),
                (0x144, REFERENCE_SLOT_144),
            ],
        );
        e.mem.set_u32(reference + 0x150, three_d);
        for i in 0..3u32 {
            e.mem
                .set_f32(reference + 0x160 + 4 * i, 10.0 * (i + 1) as f32);
        }
        e.mem.set_f32(reference + 0x180, health);
        reference
    }

    #[test]
    fn current_damage_stage_follows_the_health() {
        let mut e = engine();
        node_doubles(&mut e);
        reference_doubles(&mut e);
        let stages = [
            StageSpec {
                damage_stage: 1,
                percentage: 75,
                ..StageSpec::default()
            },
            StageSpec {
                damage_stage: 0,
                percentage: 60,
                ..StageSpec::default()
            },
            StageSpec {
                damage_stage: 2,
                percentage: 50,
                ..StageSpec::default()
            },
            StageSpec {
                damage_stage: 3,
                percentage: 25,
                ..StageSpec::default()
            },
        ];
        let (this, _) = make_component(&mut e, 100, &stages);
        let node = make_node(&mut e, 4, true, &[]);
        let reference = make_reference(&mut e, node, 45.0);
        e.call_log = Some(vec![]);
        // Health 45 %: the stages at 75, 60 and 50 count (the one at 25 ends
        // the walk); the last damage stage among them is 2.
        assert!(e.call(0x0047_6f50, &args![this, reference, false]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(RANGE_NODE_SET_CURRENT, vec![node, 2])));
        // Health 20 %: all four count.
        e.mem.set_f32(reference + 0x180, 20.0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_6f50, &args![this, reference, false]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(RANGE_NODE_SET_CURRENT, vec![node, 3])));
        // Health above every stage: no damage stage, so nothing unless forced.
        // A stage below the health ends the walk before any damage stage.
        let first_only = [StageSpec {
            damage_stage: 1,
            percentage: 50,
            ..StageSpec::default()
        }];
        let (low, _) = make_component(&mut e, 100, &first_only);
        e.mem.set_f32(reference + 0x180, 60.0);
        assert!(!e.call(0x0047_6f50, &args![low, reference, false]).bool());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_6f50, &args![low, reference, true]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(RANGE_NODE_SET_CURRENT, vec![node, 0])));
    }

    #[test]
    fn current_damage_stage_handles_undamaged_and_unseen_references() {
        let mut e = engine();
        node_doubles(&mut e);
        reference_doubles(&mut e);
        let (this, _) = make_component(
            &mut e,
            100,
            &[StageSpec {
                damage_stage: 1,
                percentage: 50,
                ..StageSpec::default()
            }],
        );
        let node = make_node(&mut e, 4, true, &[]);
        // Never damaged (-1.0): false unless forced.
        let reference = make_reference(&mut e, node, -1.0);
        assert!(!e.call(0x0047_6f50, &args![this, reference, false]).bool());
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_6f50, &args![this, reference, true]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(RANGE_NODE_SET_CURRENT, vec![node, 0])));
        // No 3D at all.
        let hidden = make_reference(&mut e, 0, 50.0);
        assert!(!e.call(0x0047_6f50, &args![this, hidden, true]).bool());
    }

    #[test]
    fn repair_lowers_the_health_to_the_stage_reached() {
        let mut e = engine();
        node_doubles(&mut e);
        reference_doubles(&mut e);
        noop(&mut e, &[0x0042_e150]);
        e.register(0x007a_f430, |_, _| ret(0x4242));
        let stages = [
            StageSpec {
                percentage: 80,
                rate: 5,
                model: 0x100,
                ..StageSpec::default()
            },
            StageSpec {
                percentage: 50,
                rate: 7,
                model: 0x200,
                ..StageSpec::default()
            },
            StageSpec {
                percentage: 20,
                rate: 9,
                ..StageSpec::default()
            },
        ];
        let (this, _) = make_component(&mut e, 100, &stages);
        // No 3D, so the damage-stage refresh at the end stops at once.
        let reference = make_reference(&mut e, 0, 60.0);
        e.call_log = Some(vec![]);
        e.call(0x0047_7090, &args![this, reference]);
        let log = e.call_log.take().unwrap();
        // The walk ends on the last stage (20 %): health 20, model 0x200.
        assert_eq!(e.mem.f32(reference + 0x180), 20.0);
        assert!(log.contains(&(0x0042_e150, vec![reference + 0x180, 0x200, 0x4242])));
        assert!(log.contains(&(REFERENCE_NOOP, vec![reference, 0, 1])));
        // The last stage reached is above the health: nothing changes.
        let (high, _) = make_component(
            &mut e,
            100,
            &[StageSpec {
                percentage: 90,
                rate: 3,
                ..StageSpec::default()
            }],
        );
        e.mem.set_f32(reference + 0x180, 60.0);
        e.call(0x0047_7090, &args![high, reference]);
        assert_eq!(e.mem.f32(reference + 0x180), 60.0);
        // A first stage below the health without a previous rate stops the walk.
        let (stop, _) = make_component(
            &mut e,
            100,
            &[StageSpec {
                percentage: 40,
                rate: 0,
                ..StageSpec::default()
            }],
        );
        e.call(0x0047_7090, &args![stop, reference]);
        assert_eq!(e.mem.f32(reference + 0x180), 60.0);
        // Undamaged: nothing.
        e.mem.set_f32(reference + 0x180, -1.0);
        e.call(0x0047_7090, &args![this, reference]);
        assert_eq!(e.mem.f32(reference + 0x180), -1.0);
    }

    #[test]
    fn nodes_are_collected_into_the_array() {
        use std::cell::RefCell;
        use std::rc::Rc;
        let mut e = engine();
        node_doubles(&mut e);
        e.register(0x0041_3f40, |_, a| ret(a[0]));
        // `0043b1b0` and `009a3830` read flags at +0x40 and +0x44 of the container.
        e.register(0x0043_b1b0, |e, a| ret(e.mem.u32(a[0] + 0x40)));
        e.register(0x009a_3830, |e, a| {
            assert_eq!(a[1], 0xf11);
            ret(e.mem.u32(a[0] + 0x44))
        });
        e.set_global(0x011c_61d8, 0xf11u32);
        let added = Rc::new(RefCell::new(Vec::new()));
        let sink = added.clone();
        e.register_double(0x007c_b2e0, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let leaf = make_node(&mut e, 0, true, &[]);
        let inner = make_node(&mut e, 0, true, &[leaf]);
        let root = make_node(&mut e, 0, true, &[inner]);
        let root_container = e.mem.u32(root + 8);
        let inner_container = e.mem.u32(inner + 8);
        // The root's container passes both checks; the inner one only the first.
        e.mem.set_u32(root_container + 0x40, 1);
        e.mem.set_u32(root_container + 0x44, 1);
        e.mem.set_u32(inner_container + 0x40, 1);
        let this: Ptr<BGSDestructibleObjectForm> = e.new_object();
        let array = e.mem.alloc(0x10);
        e.call(0x0047_7230, &args![this, root, array]);
        assert_eq!(*added.borrow(), vec![(array, root_container)]);
        // A null node is ignored.
        e.call(0x0047_7230, &args![this, 0u32, array]);
        assert_eq!(added.borrow().len(), 1);
        assert_eq!(e.call(0x0047_72e0, &args![]).u32(), 0xf11);
    }

    #[test]
    fn self_damage_goes_to_every_reference_in_the_map() {
        use std::cell::Cell;
        use std::rc::Rc;
        let mut e = engine();
        reference_doubles(&mut e);
        e.register(0x0084_d030, |_, a| {
            assert_eq!(a[0], 0x011f_6394);
            2.0f32.into_ret()
        });
        e.register(0x004b_9ba0, |_, a| {
            assert_eq!(a[0], DESTRUCTIBLE_OBJECTS);
            ret(1)
        });
        let actor = make_reference(&mut e, 0, 50.0);
        e.mem.set_u32(actor + 0x154, 1);
        let plain = make_reference(&mut e, 0, 50.0);
        let skipped = make_reference(&mut e, 0, 50.0);
        e.mem.set_u32(skipped + 0x15c, 1);
        let entries = [
            (actor, 10u32, 2u32),
            (plain, 4, 3),
            (skipped, 6, 0),
            (0, 0, 0),
        ];
        let step = Rc::new(Cell::new(0usize));
        let counter = step.clone();
        e.register_double(0x006b_7f20, move |e, a| {
            assert_eq!(a[0], DESTRUCTIBLE_OBJECTS);
            let (key, value, next) = entries[counter.get()];
            counter.set(counter.get() + 1);
            e.mem.set_u32(a[1], next);
            e.mem.set_u32(a[2], key);
            e.mem.set_u32(a[3], value);
            Ret::default()
        });
        // The skipped reference fails the first test.
        e.register(REFERENCE_TEST_A, |e, a| ret(e.mem.u32(a[0] + 0x15c)));
        e.register(REFERENCE_TEST_B, |_, _| Ret::default());
        e.register(0x0043_d4d0, |_, a| {
            assert_eq!(a[0], 0x011c_3ea4);
            ret(0x011c_3ea8)
        });
        e.set_global(0x011c_3ea8, 1u32);
        noop(&mut e, &[REFERENCE_SLOT_338, REFERENCE_SLOT_144]);
        e.call_log = Some(vec![]);
        e.call(0x0047_72f0, &args![]);
        let log = e.call_log.take().unwrap();
        let slots: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == REFERENCE_SLOT_338 || *a == REFERENCE_SLOT_144)
            .cloned()
            .collect();
        assert_eq!(
            slots,
            vec![
                (REFERENCE_SLOT_338, vec![actor, 20.0f32.to_bits(), 0, 0]),
                (REFERENCE_SLOT_144, vec![plain, 8.0f32.to_bits(), 1]),
            ]
        );
        // Any other mode hands the damage to the queue object.
        e.set_global(0x011c_3ea8, 0u32);
        step.set(0);
        e.register(0x0045_37b0, |_, _| ret(0x5000));
        noop(&mut e, &[0x0087_af50]);
        e.call_log = Some(vec![]);
        e.call(0x0047_72f0, &args![]);
        let log = e.call_log.take().unwrap();
        let queued: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == 0x0087_af50)
            .cloned()
            .collect();
        assert_eq!(
            queued,
            vec![
                (0x0087_af50, vec![0x5000, actor, 20.0f32.to_bits(), 1]),
                (0x0087_af50, vec![0x5000, plain, 8.0f32.to_bits(), 1]),
            ]
        );
    }

    #[test]
    fn removing_from_the_map_forwards_the_reference() {
        let mut e = engine();
        noop(&mut e, &[MAP_REMOVE_AT]);
        e.call_log = Some(vec![]);
        e.call(0x0047_7410, &args![0x1234u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(MAP_REMOVE_AT, vec![DESTRUCTIBLE_OBJECTS, 0x1234])));
    }

    #[test]
    fn stage_index_reached_by_a_reference() {
        let mut e = engine();
        form_type_double(&mut e);
        reference_doubles(&mut e);
        let form = form_of_type(&mut e, 0x15);
        let stages = [
            StageSpec {
                percentage: 75,
                ..StageSpec::default()
            },
            StageSpec {
                percentage: 50,
                ..StageSpec::default()
            },
            StageSpec {
                percentage: 25,
                ..StageSpec::default()
            },
        ];
        let (_, data) = make_component(&mut e, 100, &stages);
        e.mem.set_u32(form + 0x68 + 4, data.addr());
        e.register(0x007a_f430, |e, a| ret(e.mem.u32(a[0] + 0x1a0)));
        let reference = make_reference(&mut e, 0, 60.0);
        e.mem.set_u32(reference + 0x1a0, form);
        // 60 %: the first stage below it is stage 1 (50).
        assert_eq!(e.call(0x0047_7430, &args![reference]).u32(), 1);
        e.mem.set_f32(reference + 0x180, 10.0);
        assert_eq!(e.call(0x0047_7430, &args![reference]).u32(), 3);
        e.mem.set_f32(reference + 0x180, 100.0);
        assert_eq!(e.call(0x0047_7430, &args![reference]).u32(), 0);
        // Undamaged, no component, null reference.
        e.mem.set_f32(reference + 0x180, -1.0);
        assert_eq!(e.call(0x0047_7430, &args![reference]).u32(), u32::MAX);
        let other = form_of_type(&mut e, 0x14);
        let unseen = make_reference(&mut e, 0, 60.0);
        e.mem.set_u32(unseen + 0x1a0, other);
        assert_eq!(e.call(0x0047_7430, &args![unseen]).u32(), u32::MAX);
        assert_eq!(e.call(0x0047_7430, &args![0u32]).u32(), u32::MAX);
    }

    #[test]
    fn small_debris_is_hidden() {
        let mut e = engine();
        node_doubles(&mut e);
        noop(&mut e, &[0x0045_0f90]);
        // A node keeps its collision object at +0x28; the collision object
        // its shape at +0x2c; the shape its type word at +0x30.
        e.register(0x0043_b610, |e, a| ret(e.mem.u32(a[0] + 0x28)));
        e.register(0x006f_a820, |e, a| ret(e.mem.u32(a[0] + 0x2c)));
        e.register(0x0043_b4f0, |e, a| {
            let kind = e.mem.u32(a[0] + 0x30);
            e.mem.set_u32(a[1], kind);
            ret(a[1])
        });
        e.register(0x0043_b4d0, |e, a| ret(e.mem.u32(a[0]) & 0x7f));
        let with_shape = |e: &mut Engine, node: u32, kind: u32| {
            let shape = e.mem.alloc(0x40);
            e.mem.set_u32(shape + 0x30, kind);
            let collision = e.mem.alloc(0x40);
            e.mem.set_u32(collision + 0x2c, shape);
            e.mem.set_u32(node + 0x28, collision);
        };
        let small = make_node(&mut e, 0, true, &[]);
        with_shape(&mut e, small, 0x13);
        let big = make_node(&mut e, 0, true, &[]);
        with_shape(&mut e, big, 0x07);
        let plain = make_node(&mut e, 0, true, &[]);
        let parent = make_node(&mut e, 0, true, &[big, plain, small]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0047_7560, &args![parent]).bool());
        let log = e.call_log.take().unwrap();
        let hidden: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == 0x0045_0f90)
            .cloned()
            .collect();
        assert_eq!(hidden, vec![(0x0045_0f90, vec![small, 1])]);
        // Nothing small below a node: false and nothing hidden; a null node
        // is false too.
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0047_7560, &args![big]).bool());
        assert!(!e.call(0x0047_7560, &args![0u32]).bool());
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == 0x0045_0f90));
    }

    #[test]
    fn files_of_every_stage_are_queued() {
        let mut e = engine();
        e.register(0x0045_c6b0, |_, a| ret(a[0] + 100));
        noop(&mut e, &[0x0044_3d30, REFERENCE_NOOP]);
        e.set_global(0x011c_3b3c, 0x9000u32);
        // Explosion and debris forms with an embedded model whose vtable
        // has slot 0x10.
        let explosion = e.mem.alloc(0x100);
        let explosion_table = e.mem.alloc(0x40);
        e.mem.set_u32(explosion_table + 0x10, REFERENCE_NOOP);
        e.mem.set_u32(explosion + 0x64, explosion_table);
        let debris = e.mem.alloc(0x100);
        let debris_table = e.mem.alloc(0x40);
        e.mem.set_u32(debris_table + 0x10, REFERENCE_NOOP);
        e.mem.set_u32(debris + 0x18, debris_table);
        let stages = [
            StageSpec {
                model: 0x3000,
                ..StageSpec::default()
            },
            StageSpec {
                explosion,
                debris,
                ..StageSpec::default()
            },
            StageSpec::default(),
        ];
        let (this, _) = make_component(&mut e, 100, &stages);
        e.call_log = Some(vec![]);
        e.call(0x0047_7640, &args![this, 7u32, 8u32, 9u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0044_3d30, vec![0x9000, 0x3000, 8, 9, 107, 1, 0, 0])));
        assert!(log.contains(&(REFERENCE_NOOP, vec![explosion + 0x64, 8, 9])));
        assert!(log.contains(&(REFERENCE_NOOP, vec![debris + 0x18, 8, 9])));
        // A component without data does nothing.
        let empty: Ptr<BGSDestructibleObjectForm> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0047_7640, &args![empty, 7u32, 8u32, 9u32]);
        assert!(e.call_log.take().unwrap().len() == 1);
    }

    // ---- the damage routine -------------------------------------------

    /// Doubles for everything the damage routine calls. Settings (`float`s
    /// read through `00403e20`): A 1.0, B 2.0, limit 100.0, margin 5.0,
    /// fallback 9.0.
    fn damage_engine() -> Engine {
        let mut e = engine();
        node_doubles(&mut e);
        reference_doubles(&mut e);
        noop(
            &mut e,
            &[
                REFERENCE_TEST_A,
                REFERENCE_TEST_B,
                NODE_HOLDER_DESTROY,
                0x005a_a500,
                0x0048_4530,
                0x0048_4650,
                0x009b_c8f0,
                0x005a_c750,
                MAP_SET_AT,
                MAP_REMOVE_AT,
                SPAWN_DEBRIS,
                CREATE_EXPLOSION,
                0x0057_a3c0,
                0x0047_78a0,
                0x009a_54a0,
                0x009a_51e0,
            ],
        );
        e.register(NODE_HOLDER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(0x0043_fcd0, |e, a| ret(e.mem.u32(a[0] + 0x150)));
        e.register(0x0047_7a50, |_, _| (-1.0f32).into_ret());
        e.register(0x0047_7970, |_, _| (-1.0f32).into_ret());
        e.register(SETTING_POINTER, |_, a| ret(a[0]));
        e.register(0x0082_5c00, |_, _| ret(0x7777));
        e.set_global(0x011c_e720, 1.0f32);
        e.set_global(0x011c_f1b0, 2.0f32);
        e.set_global(0x011c_eabc, 100.0f32);
        e.set_global(0x011c_e1b8, 5.0f32);
        e.set_global(0x011c_ef20, 9.0f32);
        e
    }

    fn damage(e: &mut Engine, this: Ptr<BGSDestructibleObjectForm>, reference: u32, amount: f32) {
        e.call(0x0047_5b20, &args![this, reference, amount, 0u32]);
    }

    fn health_of(e: &Engine, reference: u32) -> f32 {
        e.mem.f32(reference + 0x180)
    }

    fn calls_to(log: &[(u32, Vec<u32>)], address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    #[test]
    fn damage_is_ignored_when_it_does_not_apply() {
        let mut e = damage_engine();
        let stage = StageSpec {
            percentage: 80,
            damage_stage: 1,
            ..StageSpec::default()
        };
        let (this, _) = make_component(&mut e, 100, &[stage]);
        let root = make_node(&mut e, 0, true, &[]);
        let reference = make_reference(&mut e, root, 100.0);
        // Not above 0.
        damage(&mut e, this, reference, 0.0);
        damage(&mut e, this, reference, -5.0);
        // No reference, no data.
        e.call(0x0047_5b20, &args![this, 0u32, 10.0f32, 0u32]);
        let (no_data, _) = make_component(&mut e, 100, &[]);
        let data = e.get(no_data, BGSDestructibleObjectForm::pData);
        e.set(no_data, BGSDestructibleObjectForm::pData, Ptr::NULL);
        let _ = data;
        damage(&mut e, no_data, reference, 10.0);
        // No 3D.
        let hidden = make_reference(&mut e, 0, 100.0);
        damage(&mut e, this, hidden, 10.0);
        assert_eq!(health_of(&e, reference), 100.0);
        assert_eq!(health_of(&e, hidden), 100.0);
        // Either reference test.
        e.register(REFERENCE_TEST_A, |_, _| ret(1));
        damage(&mut e, this, reference, 10.0);
        assert_eq!(health_of(&e, reference), 100.0);
        e.register(REFERENCE_TEST_A, |_, _| Ret::default());
        e.register(REFERENCE_TEST_B, |_, _| ret(1));
        damage(&mut e, this, reference, 10.0);
        assert_eq!(health_of(&e, reference), 100.0);
        // An exhausted object (0 health) is left alone as well.
        e.register(REFERENCE_TEST_B, |_, _| Ret::default());
        e.mem.set_f32(reference + 0x180, 0.0);
        damage(&mut e, this, reference, 10.0);
        assert_eq!(health_of(&e, reference), 0.0);
    }

    #[test]
    fn damage_crossing_a_stage_updates_nodes_map_debris_and_threats() {
        let mut e = damage_engine();
        let stage = StageSpec {
            percentage: 80,
            damage_stage: 1,
            rate: 5,
            debris: 0x2000,
            count: 2,
            ..StageSpec::default()
        };
        let (this, data) = make_component(&mut e, 100, &[stage]);
        let root = make_node(&mut e, 0, true, &[]);
        let reference = make_reference(&mut e, root, 100.0);
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 30.0);
        let log = e.call_log.take().unwrap();
        let _ = data;
        assert_eq!(health_of(&e, reference), 70.0);
        // Virtual slot 0x48 with 0x80000000.
        assert!(log.contains(&(REFERENCE_NOOP, vec![reference, 0x8000_0000])));
        // The damage-stage nodes and sounds.
        assert!(log.contains(&(0x0057_a3c0, vec![reference, 0])));
        // The self-damage map.
        assert!(log.contains(&(MAP_SET_AT, vec![DESTRUCTIBLE_OBJECTS, reference, 5])));
        assert!(calls_to(&log, MAP_REMOVE_AT).is_empty());
        // The crossed stage's action flag, and debris at the reference's own
        // position (no debris node).
        assert!(log.contains(&(0x005a_c750, vec![reference, reference + 0x180, 0x0020_0000])));
        let spawned = calls_to(&log, SPAWN_DEBRIS);
        assert_eq!(spawned.len(), 1);
        assert_eq!(spawned[0][0], 0x2000);
        assert_eq!(spawned[0][1], 0x1234);
        assert_eq!(spawned[0][3..], [2, 1.5f32.to_bits()]);
        // The threat routines get the new percentage and the settings.
        let threat = calls_to(&log, 0x0047_7a50);
        assert_eq!(threat.len(), 1);
        assert_eq!(
            threat[0][..4],
            [this.addr(), 70, 2.0f32.to_bits(), 1.0f32.to_bits()]
        );
        // Nothing came back: the object is removed from the threat map.
        assert!(log.contains(&(0x009a_51e0, vec![0x7777, reference])));
        assert!(calls_to(&log, 0x009a_54a0).is_empty());
        // Alive: no death routine.
        assert!(calls_to(&log, 0x0047_78a0).is_empty());
    }

    #[test]
    fn damage_without_a_self_damage_rate_leaves_the_map() {
        let mut e = damage_engine();
        let stage = StageSpec {
            percentage: 80,
            damage_stage: 1,
            ..StageSpec::default()
        };
        let (this, _) = make_component(&mut e, 100, &[stage]);
        let root = make_node(&mut e, 0, true, &[]);
        let reference = make_reference(&mut e, root, 100.0);
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 30.0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(MAP_REMOVE_AT, vec![DESTRUCTIBLE_OBJECTS, reference])));
        assert!(calls_to(&log, MAP_SET_AT).is_empty());
        // No rate: the first threat routine is not asked.
        assert!(calls_to(&log, 0x0047_7a50).is_empty());
        assert_eq!(calls_to(&log, 0x0047_7970).len(), 1);
    }

    #[test]
    fn damage_below_the_old_stage_does_nothing_more() {
        let mut e = damage_engine();
        let stage = StageSpec {
            percentage: 80,
            damage_stage: 1,
            rate: 5,
            ..StageSpec::default()
        };
        let (this, _) = make_component(&mut e, 100, &[stage]);
        let root = make_node(&mut e, 0, true, &[]);
        // 70 % already; 70 -> 60 crosses nothing, and the stage at 80 is
        // an earlier one ("passed" before).
        let reference = make_reference(&mut e, root, 70.0);
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 10.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(health_of(&e, reference), 60.0);
        assert!(calls_to(&log, 0x0057_a3c0).is_empty());
        assert!(calls_to(&log, MAP_SET_AT).is_empty());
        assert!(calls_to(&log, MAP_REMOVE_AT).is_empty());
        assert!(calls_to(&log, 0x005a_c750).is_empty());
        // The earlier stage's rate (5) is still handed to the threat routine.
        assert_eq!(calls_to(&log, 0x0047_7a50).len(), 1);
    }

    #[test]
    fn a_stage_can_cap_the_damage() {
        let mut e = damage_engine();
        let stages = [
            StageSpec {
                percentage: 80,
                flags: STAGE_FLAG_CAP_DAMAGE,
                rate: 3,
                ..StageSpec::default()
            },
            StageSpec {
                percentage: 60,
                rate: 4,
                ..StageSpec::default()
            },
        ];
        let (this, _) = make_component(&mut e, 100, &stages);
        let root = make_node(&mut e, 0, true, &[]);
        let reference = make_reference(&mut e, root, 100.0);
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 50.0);
        let log = e.call_log.take().unwrap();
        // The health stops at the stage's 80 % and the walk ends there.
        assert_eq!(health_of(&e, reference), 80.0);
        assert!(log.contains(&(MAP_SET_AT, vec![DESTRUCTIBLE_OBJECTS, reference, 3])));
        // New percentage 80 for the threat routine.
        assert_eq!(calls_to(&log, 0x0047_7a50)[0][1], 80);
    }

    #[test]
    fn stage_flags_disable_and_destroy() {
        // Disable: the plain way (slot 0x224 false) adds the pending
        // disabled reference and deletes the form.
        let mut e = damage_engine();
        let stage = StageSpec {
            percentage: 80,
            flags: STAGE_FLAG_DISABLE_OBJECT,
            ..StageSpec::default()
        };
        let (this, _) = make_component(&mut e, 100, &[stage]);
        let root = make_node(&mut e, 0, true, &[]);
        let reference = make_reference(&mut e, root, 100.0);
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 30.0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x005a_a500, vec![reference, 0])));
        assert!(log.contains(&(0x0048_4530, vec![reference, 1])));
        assert!(calls_to(&log, 0x009b_c8f0).is_empty());
        // Slot 0x224 true: the other routine.
        e.mem.set_f32(reference + 0x180, 100.0);
        e.mem.set_u32(reference + 0x158, 1);
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 30.0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x009b_c8f0, vec![reference])));
        assert!(calls_to(&log, 0x005a_a500).is_empty());
        // Destroy.
        let stage = StageSpec {
            percentage: 80,
            flags: STAGE_FLAG_DESTROY_OBJECT,
            ..StageSpec::default()
        };
        let (this, _) = make_component(&mut e, 100, &[stage]);
        e.mem.set_f32(reference + 0x180, 100.0);
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 30.0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0048_4650, vec![reference, 1])));
    }

    #[test]
    fn lethal_damage_runs_the_death_routine() {
        let mut e = damage_engine();
        let stage = StageSpec {
            percentage: 25,
            rate: 5,
            explosion: 0,
            ..StageSpec::default()
        };
        let (this, _) = make_component(&mut e, 100, &[stage]);
        let root = make_node(&mut e, 0, true, &[]);
        let reference = make_reference(&mut e, root, 100.0);
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 150.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(health_of(&e, reference), 0.0);
        // At 0 health the form is flagged (`00484650`) before the health is
        // stored, the object leaves the self-damage map, and `004778a0` runs.
        assert!(log.contains(&(0x0048_4650, vec![reference, 1])));
        assert!(log.contains(&(MAP_REMOVE_AT, vec![DESTRUCTIBLE_OBJECTS, reference])));
        assert!(log.contains(&(0x0047_78a0, vec![this.addr()])));
        assert!(calls_to(&log, MAP_SET_AT).is_empty());
    }

    #[test]
    fn an_actors_health_is_its_actor_value() {
        let mut e = damage_engine();
        let stage = StageSpec {
            percentage: 50,
            ..StageSpec::default()
        };
        let (this, _) = make_component(&mut e, 100, &[stage]);
        let root = make_node(&mut e, 0, true, &[]);
        let reference = make_reference(&mut e, root, 100.0);
        e.mem.set_u32(reference + 0x154, 1);
        // Interface at +0xa4: slot 4 gives the total (200.0), slot 8 the
        // current value after the damage (80).
        e.register(0x0f00_0020, |_, _| 200.0f32.into_ret());
        e.register(0x0f00_0021, |_, a| {
            assert_eq!(a[1], 0x10);
            ret(80)
        });
        let table = e.mem.alloc(0x40);
        e.mem.set_u32(table + 4, 0x0f00_0020);
        e.mem.set_u32(table + 8, 0x0f00_0021);
        e.mem.set_u32(reference + 0xa4, table);
        damage(&mut e, this, reference, 30.0);
        assert_eq!(health_of(&e, reference), 80.0);
    }

    /// Doubles for the model swap.
    fn swap_doubles(e: &mut Engine, node: u32) {
        e.set_global(0x011c_4800, node);
        e.set_global(0x011d_df38, 0x1111u32);
        e.set_global(0x011d_ea10, 0x2222u32);
        e.register(ARRAY_CONSTRUCT, |e, a| {
            let buffer = e.mem.alloc(4);
            let node: u32 = e.global(0x011c_4800);
            e.mem.set_u32(buffer, node);
            e.mem.set_u32(a[0] + 4, buffer);
            e.mem.set_u32(a[0] + 8, 1);
            ret(a[0])
        });
        e.register(ARRAY_SIZE, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(ARRAY_ELEMENT, |e, a| ret(e.mem.u32(a[0] + 4) + 4 * a[1]));
        noop(
            e,
            &[
                ARRAY_DESTROY,
                0x0040_b460,
                0x0057_b240,
                0x0042_e150,
                0x0045_1ef0,
                0x00c6_a270,
                GET_ORIENTATION,
                0x0056_fa90,
                UPDATE_3D_POSITION,
                0x0043_9ef0,
                0x0063_c8a0,
                0x0057_5830,
                0x0040_1050,
            ],
        );
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x0043_f8d0, |_, a| ret(a[1]));
        e.register(0x0057_b200, |_, _| ret(1));
        e.register(0x0046_23f0, |_, _| ret(0x33));
        e.register(0x007a_f430, |_, _| ret(0x4242));
    }

    #[test]
    fn a_replacement_model_is_swapped_in_with_the_old_transform() {
        let mut e = damage_engine();
        let stage = StageSpec {
            percentage: 80,
            model: 0x6000,
            ..StageSpec::default()
        };
        let (this, data) = make_component(&mut e, 100, &[stage]);
        e.mem.set_u32(data.addr() + 0x10, 0x5000);
        let root = make_node(&mut e, 0, true, &[]);
        let gathered = make_node(&mut e, 0, true, &[]);
        swap_doubles(&mut e, gathered);
        let reference = make_reference(&mut e, root, 100.0);
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 30.0);
        let log = e.call_log.take().unwrap();
        // No damage-stage update: the model path was taken.
        assert!(calls_to(&log, 0x0057_a3c0).is_empty());
        // The preloaded model is counted and released at the end.
        assert!(log.contains(&(0x0040_b460, vec![data.addr() + 0xc])));
        assert!(log.contains(&(0x0047_78a0, vec![this.addr()])));
        // The model goes into the extra data with the form `007af430` gave.
        assert!(log.contains(&(0x0042_e150, vec![reference + 0x180, 0x6000, 0x4242])));
        assert!(log.contains(&(0x0045_1ef0, vec![0x2222, reference, 0x1234, 0, 0])));
        // The flags around the swap.
        let flags = calls_to(&log, 0x0046_23f0);
        assert_eq!(flags, vec![vec![0x1111, 0], vec![0x1111, 0x33]]);
        assert!(log.contains(&(0x0057_b240, vec![reference, 1])));
        // Both node arrays were gathered, the new 3D activated.
        assert_eq!(calls_to(&log, ARRAY_CONSTRUCT).len(), 2);
        assert_eq!(calls_to(&log, ARRAY_DESTROY).len(), 2);
        assert!(log.contains(&(0x00c6_a270, vec![root, 1, 1, 0])));
        // The orientation and the position are carried over.
        let orientation = calls_to(&log, 0x0056_fa90);
        assert_eq!(orientation.len(), 1);
        assert_eq!(orientation[0].len(), 10);
        assert_eq!(orientation[0][0], reference);
        assert_eq!(calls_to(&log, 0x0057_5830).len(), 1);
        assert_eq!(calls_to(&log, UPDATE_3D_POSITION).len(), 2);
        // The translation difference is `old translation - new translation`
        // with the nodes' translations (+0x24 of the copied transforms).
        let difference = calls_to(&log, 0x0043_9ef0);
        assert_eq!(difference.len(), 1);
        assert_eq!(difference[0].len(), 3);
        // The matrix product runs twice (old * transpose(new) * orientation).
        assert_eq!(calls_to(&log, 0x0043_f8d0).len(), 2);
    }

    #[test]
    fn a_replacement_model_without_nodes_skips_the_transform() {
        let mut e = damage_engine();
        let stage = StageSpec {
            percentage: 80,
            model: 0x6000,
            ..StageSpec::default()
        };
        let (this, _) = make_component(&mut e, 100, &[stage]);
        let root = make_node(&mut e, 0, true, &[]);
        swap_doubles(&mut e, root);
        // Arrays that stay empty.
        e.register(ARRAY_CONSTRUCT, |_, a| ret(a[0]));
        let reference = make_reference(&mut e, root, 100.0);
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 30.0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x0042_e150, vec![reference + 0x180, 0x6000, 0x4242])));
        assert!(calls_to(&log, 0x0056_fa90).is_empty());
        assert!(calls_to(&log, 0x0057_5830).is_empty());
        assert_eq!(calls_to(&log, ARRAY_CONSTRUCT).len(), 1);
        assert!(log.contains(&(0x00c6_a270, vec![root, 1, 1, 0])));
        // No preloaded model: no release.
        assert!(calls_to(&log, 0x0047_78a0).is_empty());
    }

    #[test]
    fn an_explosion_is_created_at_a_random_point_of_the_bound() {
        use std::cell::Cell;
        use std::rc::Rc;
        let mut e = damage_engine();
        let stage = StageSpec {
            percentage: 80,
            explosion: 0x7000,
            ..StageSpec::default()
        };
        let (this, _) = make_component(&mut e, 100, &[stage]);
        let root = make_node(&mut e, 0, true, &[]);
        for (i, v) in [5.0f32, 6.0, 7.0, 2.0].iter().enumerate() {
            e.mem.set_f32(root + 0x90 + 4 * i as u32, *v);
        }
        e.register(GET_WORLD_BOUND, |_, a| ret(a[0] + 0x90));
        e.register(0x0068_15c0, |_, a| ret(a[0]));
        e.register(0x0084_d030, |e, a| e.mem.f32(a[0] + 0xc).into_ret());
        e.register(0x0043_9180, |e, a| {
            // Keep the vector and the radius for the test.
            for i in 0..3 {
                let v = e.mem.u32(a[0] + 4 * i);
                e.mem.set_u32(0x011c_4900 + 4 * i, v);
            }
            e.mem.set_u32(0x011c_4910, a[1]);
            Ret::default()
        });
        noop(&mut e, &[0x0043_9e90, 0x00aa_5140, GET_ORIENTATION]);
        e.register(0x00aa_5230, |_, _| ret(0x8000_0000));
        let reference = make_reference(&mut e, root, 100.0);
        e.mem.set_u32(reference + 0xf8, 0xcafe);
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 30.0);
        let log = e.call_log.take().unwrap();
        // x and y are in [-1, 1], z in [-1, 0]; the word is exactly half.
        assert_eq!(e.mem.f32(0x011c_4900), 0.0);
        assert_eq!(e.mem.f32(0x011c_4904), 0.0);
        assert_eq!(e.mem.f32(0x011c_4908), -0.5);
        assert_eq!(e.mem.f32(0x011c_4910), 2.0);
        let created = calls_to(&log, CREATE_EXPLOSION);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0].len(), 16);
        assert_eq!(created[0][..4], [0x7000, reference, reference, 0x1234]);

        // Slot 0x224 true: the other arguments, and the new explosion
        // copies the reference's word at +0xf8 into its +0xe0.
        e.mem.set_f32(reference + 0x180, 100.0);
        e.mem.set_u32(reference + 0x158, 1);
        e.register(0x0087_4480, |_, _| ret(0x9999));
        let spawned = Rc::new(Cell::new(0u32));
        let slot = spawned.clone();
        e.register_double(CREATE_EXPLOSION, move |e, _| {
            let block = e.mem.alloc(0x100);
            slot.set(block);
            ret(block)
        });
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 30.0);
        let log = e.call_log.take().unwrap();
        let created = calls_to(&log, CREATE_EXPLOSION);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][..4], [0x7000, 0x9999, 0, 0x1234]);
        assert_eq!(e.mem.u32(spawned.get() + 0xe0), 0xcafe);
    }

    #[test]
    fn explosions_prefer_the_nodes_of_the_3d() {
        let mut e = damage_engine();
        let stage = StageSpec {
            percentage: 80,
            explosion: 0x7000,
            debris: 0x2000,
            count: 3,
            damage_stage: 4,
            ..StageSpec::default()
        };
        let (this, _) = make_component(&mut e, 100, &[stage]);
        // A node that is both an explosion and a debris node.
        let hit = make_node(&mut e, 1 | 2, true, &[]);
        let root = make_node(&mut e, 0, true, &[hit]);
        let reference = make_reference(&mut e, root, 100.0);
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 30.0);
        let log = e.call_log.take().unwrap();
        let created = calls_to(&log, CREATE_EXPLOSION);
        assert_eq!(created.len(), 1);
        assert_eq!(created[0][..4], [0x7000, reference, reference, 0x1234]);
        // The debris came from the node's translation, not the reference's.
        let spawned = calls_to(&log, SPAWN_DEBRIS);
        assert_eq!(spawned.len(), 1);
        assert_eq!(
            spawned[0],
            vec![0x2000, 0x1234, hit + 0x80, 3, 1.5f32.to_bits()]
        );
        // The damage stage reached `SetCurrent` as the selector.
        assert!(log.contains(&(RANGE_NODE_SET_CURRENT, vec![hit, 4])));
    }

    #[test]
    fn the_threat_map_gets_the_result_of_the_routines() {
        let stage = StageSpec {
            percentage: 80,
            rate: 5,
            ..StageSpec::default()
        };
        // The first routine answers 3.0 (and a key): the object is added.
        let mut e = damage_engine();
        let (this, _) = make_component(&mut e, 100, &[stage]);
        let root = make_node(&mut e, 0, true, &[]);
        let reference = make_reference(&mut e, root, 100.0);
        e.register(0x0047_7a50, |e, a| {
            e.mem.set_u32(a[4], 0x55);
            3.0f32.into_ret()
        });
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 30.0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x009a_54a0, vec![0x7777, reference, 0x55, 3.0f32.to_bits()])));
        assert!(calls_to(&log, 0x0047_7970).is_empty());
        assert!(calls_to(&log, 0x009a_51e0).is_empty());

        // The answer is above the limit setting (100.0 here set to 1.0):
        // discarded, the second routine is asked, and a margin above the
        // health difference selects the fallback value.
        let mut e = damage_engine();
        let (this, _) = make_component(&mut e, 100, &[stage]);
        let root = make_node(&mut e, 0, true, &[]);
        let reference = make_reference(&mut e, root, 100.0);
        e.register(0x0047_7a50, |_, _| 3.0f32.into_ret());
        e.register(0x0047_7970, |_, _| 4.0f32.into_ret());
        e.set_global(0x011c_eabc, 1.0f32);
        e.set_global(0x011c_e1b8, 100.0f32);
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 30.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, 0x0047_7970).len(), 1);
        assert!(log.contains(&(0x009a_54a0, vec![0x7777, reference, 0, 9.0f32.to_bits()])));

        // A margin below the difference keeps "no result".
        let mut e = damage_engine();
        let (this, _) = make_component(&mut e, 100, &[stage]);
        let root = make_node(&mut e, 0, true, &[]);
        let reference = make_reference(&mut e, root, 100.0);
        e.register(0x0047_7970, |_, _| 4.0f32.into_ret());
        e.call_log = Some(vec![]);
        damage(&mut e, this, reference, 30.0);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(0x009a_51e0, vec![0x7777, reference])));
    }

    // -----------------------------------------------------------------
    // Second session
    // -----------------------------------------------------------------

    use std::cell::RefCell;
    use std::rc::Rc;

    fn log_of(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap()
    }

    /// `operator new`, `operator delete` and `memset` as plain doubles.
    fn memory_doubles(e: &mut Engine) {
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            ret(a[0])
        });
    }

    fn preload_engine(answer: u32) -> Engine {
        let mut e = engine();
        memory_doubles(&mut e);
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(QUEUED_FILE_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(0x00c3_c590, |_, a| ret(a[0]));
        e.register_double(0x0044_6990, move |_, _| ret(answer));
        noop(&mut e, &[0x0040_b460, 0x0045_c6b0, 0x0044_3d30]);
        e
    }

    #[test]
    fn preloading_creates_the_file_queues_it_and_counts_it() {
        let mut e = preload_engine(1);
        let (this, data) = make_component(&mut e, 100, &[]);
        e.call_log = Some(vec![]);
        e.call(0x0047_7780, &args![this, 0x55u32]);
        let log = log_of(&mut e);
        let holder = data.addr() + 0x10;
        let file = e.mem.u32(holder);
        assert_ne!(file, 0);
        assert!(log.contains(&(OPERATOR_NEW, vec![0x28])));
        assert!(log.contains(&(0x00c3_c590, vec![file, 5])));
        assert!(log.contains(&(QUEUED_FILE_ASSIGN, vec![holder, file])));
        assert!(log.contains(&(0x0044_6990, vec![file])));
        assert!(log.contains(&(0x0040_b460, vec![data.addr() + 0xc])));
        assert!(!log.contains(&(QUEUED_FILE_ASSIGN, vec![holder, 0])));
    }

    #[test]
    fn preloading_clears_the_holder_when_the_file_is_refused() {
        let mut e = preload_engine(0);
        let (this, data) = make_component(&mut e, 100, &[]);
        e.call_log = Some(vec![]);
        e.call(0x0047_7780, &args![this, 0x55u32]);
        let log = log_of(&mut e);
        let holder = data.addr() + 0x10;
        assert_eq!(e.mem.u32(holder), 0);
        assert!(log.contains(&(QUEUED_FILE_ASSIGN, vec![holder, 0])));
        assert!(calls_to(&log, 0x0040_b460).is_empty());
    }

    #[test]
    fn preloading_does_nothing_new_when_a_file_is_held_or_without_data() {
        let mut e = preload_engine(1);
        let (this, data) = make_component(&mut e, 100, &[]);
        e.mem.set_u32(data.addr() + 0x10, 0x1234);
        e.call_log = Some(vec![]);
        e.call(0x0047_7780, &args![this, 0x55u32]);
        let log = log_of(&mut e);
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(calls_to(&log, 0x0040_b460).len(), 1);

        let empty: Ptr<BGSDestructibleObjectForm> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0047_7780, &args![empty, 0x55u32]);
        assert_eq!(log_of(&mut e).len(), 1);
    }

    #[test]
    fn the_preload_counter_clears_the_holder_at_zero() {
        let mut e = engine();
        e.register(POINTER_GET, |e, a| ret(e.mem.u32(a[0])));
        e.register(QUEUED_FILE_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        // `004019a0` takes one off the count at its argument.
        e.register(0x0040_19a0, |e, a| {
            let count = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], count.wrapping_sub(1));
            Ret::default()
        });
        let (this, data) = make_component(&mut e, 100, &[]);
        let holder = data.addr() + 0x10;
        e.mem.set_u32(holder, 0x1234);
        e.set(data, DestructibleObjectData::iReplacementModelRefCount, 2);
        e.call(0x0047_78a0, &args![this]);
        assert_eq!(e.mem.u32(holder), 0x1234);
        e.call(0x0047_78a0, &args![this]);
        assert_eq!(e.mem.u32(holder), 0);
        // Nothing held: the counter is left alone.
        e.call(0x0047_78a0, &args![this]);
        assert_eq!(
            e.get(data, DestructibleObjectData::iReplacementModelRefCount),
            0
        );
        // No data at all.
        let empty: Ptr<BGSDestructibleObjectForm> = e.new_object();
        e.call(0x0047_78a0, &args![empty]);
    }

    fn limit_engine() -> Engine {
        let mut e = engine();
        // `00477950` answers the byte at +0x100, `006a78f0` the float at +0x110.
        e.register(0x0047_7950, |e, a| ret(e.mem.u8(a[0] + 0x100) as u32));
        e.register(0x006a_78f0, |e, a| e.mem.f32(a[0] + 0x110).into_ret());
        e
    }

    fn limit_explosion(e: &mut Engine, radius: f32, other: f32) -> u32 {
        let explosion = e.mem.alloc(0x120);
        e.mem.set_f32(explosion + 0x7c, radius);
        e.mem.set_u8(explosion + 0x100, 1);
        e.mem.set_f32(explosion + 0x110, other);
        explosion
    }

    #[test]
    fn explosion_radius_is_scaled_by_the_setting_unless_the_flag_is_set() {
        let mut e = limit_engine();
        let setting = e.mem.alloc(8);
        e.mem.set_f32(setting, 2.0);
        e.register_double(SETTING_POINTER, move |_, _| ret(setting));
        let explosion = limit_explosion(&mut e, 5.0, 0.0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_7900, &args![explosion]).f32(), 5.0);
        let log = log_of(&mut e);
        assert!(calls_to(&log, SETTING_POINTER).is_empty());
        assert!(log.contains(&(0x0047_7950, vec![explosion, 1])));

        e.mem.set_u8(explosion + 0x100, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_7900, &args![explosion]).f32(), 10.0);
        let log = log_of(&mut e);
        assert!(log.contains(&(SETTING_POINTER, vec![0x011d_1218])));
    }

    #[test]
    fn the_first_stage_that_passes_both_limits_gives_its_health() {
        let mut e = limit_engine();
        let near = limit_explosion(&mut e, 5.0, 3.0);
        let far = limit_explosion(&mut e, 1.0, 3.0);
        let stages = [
            StageSpec {
                percentage: 80,
                explosion: far,
                ..StageSpec::default()
            },
            StageSpec {
                percentage: 40,
                explosion: near,
                ..StageSpec::default()
            },
            StageSpec {
                percentage: 10,
                ..StageSpec::default()
            },
        ];
        let (this, _) = make_component(&mut e, 200, &stages);
        let out = e.mem.alloc(4);
        let run = |e: &mut Engine, percentage: u32, radius: f32, other: f32, out: u32| {
            e.call(0x0047_7970, &args![this, percentage, radius, other, out])
                .f32()
        };
        // Stage 0 (80) is not below 60; stage 1 passes: 40% of 200.
        assert_eq!(run(&mut e, 60, 2.0, 2.0, out), 80.0);
        assert_eq!(e.mem.u32(out), near);
        // The radius limit is above every radius: nothing, out untouched.
        e.mem.set_u32(out, 0);
        assert_eq!(run(&mut e, 60, 6.0, 2.0, out), -1.0);
        assert_eq!(e.mem.u32(out), 0);
        // The second value is below the limit.
        assert_eq!(run(&mut e, 60, 2.0, 3.5, out), -1.0);
        // The nearer stage: 80% of 200, and no out pointer is fine.
        assert_eq!(run(&mut e, 90, 0.5, 2.0, 0), 160.0);
        assert_eq!(run(&mut e, 90, 0.5, 2.0, out), 160.0);
        assert_eq!(e.mem.u32(out), far);
    }

    #[test]
    fn the_time_to_the_first_matching_stage_adds_up_the_stages() {
        let mut e = limit_engine();
        let explosion = limit_explosion(&mut e, 5.0, 3.0);
        let stages = [
            StageSpec {
                percentage: 100,
                rate: 2,
                ..StageSpec::default()
            },
            StageSpec {
                percentage: 60,
                rate: 4,
                explosion,
                ..StageSpec::default()
            },
            StageSpec {
                percentage: 20,
                rate: 0,
                ..StageSpec::default()
            },
        ];
        let (this, _) = make_component(&mut e, 100, &stages);
        let out = e.mem.alloc(4);
        // From 80: (80 - 60) / 2 at stage 1, which has the explosion.
        let result = e
            .call(0x0047_7a50, &args![this, 80u32, 2.0f32, 2.0f32, out])
            .f32();
        assert_eq!(result, 10.0);
        assert_eq!(e.mem.u32(out), explosion);
        // The explosion fails the limit: the walk goes on, adding
        // (60 - 20) / 4 at stage 2, and ends without a match.
        e.mem.set_u32(out, 0);
        let result = e
            .call(0x0047_7a50, &args![this, 80u32, 9.0f32, 2.0f32, out])
            .f32();
        assert_eq!(result, -1.0);
        assert_eq!(e.mem.u32(out), 0);
        // A stage below the percentage right after a stage without self
        // damage stops the walk.
        let stages = [StageSpec {
            percentage: 70,
            rate: 0,
            explosion,
            ..StageSpec::default()
        }];
        let (this, _) = make_component(&mut e, 100, &stages);
        let result = e
            .call(0x0047_7a50, &args![this, 80u32, 2.0f32, 2.0f32, 0u32])
            .f32();
        assert_eq!(result, -1.0);
    }

    #[test]
    fn the_total_of_the_walk_includes_every_stage_passed() {
        let mut e = limit_engine();
        let explosion = limit_explosion(&mut e, 5.0, 3.0);
        let stages = [
            StageSpec {
                percentage: 100,
                rate: 2,
                ..StageSpec::default()
            },
            StageSpec {
                percentage: 60,
                rate: 4,
                ..StageSpec::default()
            },
            StageSpec {
                percentage: 20,
                explosion,
                ..StageSpec::default()
            },
        ];
        let (this, _) = make_component(&mut e, 100, &stages);
        // (80 - 60) / 2 + (60 - 20) / 4 = 20.
        let result = e
            .call(0x0047_7a50, &args![this, 80u32, 2.0f32, 2.0f32, 0u32])
            .f32();
        assert_eq!(result, 20.0);
    }

    #[test]
    fn self_damage_sets_or_removes_the_map_entry() {
        let mut e = engine();
        noop(&mut e, &[MAP_SET_AT, MAP_REMOVE_AT]);
        e.call_log = Some(vec![]);
        e.call(0x0047_7ce0, &args![0x1000u32, 0u32]);
        e.call(0x0047_7ce0, &args![0x1000u32, 7u32]);
        let log = log_of(&mut e);
        assert!(log.contains(&(MAP_REMOVE_AT, vec![DESTRUCTIBLE_OBJECTS, 0x1000])));
        assert!(log.contains(&(MAP_SET_AT, vec![DESTRUCTIBLE_OBJECTS, 0x1000, 7])));
        assert_eq!(calls_to(&log, MAP_SET_AT).len(), 1);
    }

    /// The doubles of the self-damage finish routine; the reference's
    /// +0x1f0 is `00452370`'s answer, +0x1f4 `00440da0`'s, +0x1f8
    /// `00477ba0`'s, +0x1e8 the node `0043fcd0` gives, and the extra list
    /// (at +0x180) answers the model swap at its +0x7c.
    fn finish_engine() -> Engine {
        let mut e = engine();
        node_doubles(&mut e);
        reference_doubles(&mut e);
        form_type_double(&mut e);
        e.register(0x0045_2370, |e, a| ret(e.mem.u32(a[0] + 0x1f0)));
        e.register(REFERENCE_TEST_A, |e, a| ret(e.mem.u32(a[0] + 0x1f4)));
        e.register(0x0047_7ba0, |e, a| ret(e.mem.u32(a[0] + 0x1f8)));
        e.register(0x0043_fcd0, |e, a| ret(e.mem.u32(a[0] + 0x1e8)));
        e.register(0x0042_e250, |e, a| ret(e.mem.u32(a[0] + 0x7c)));
        e.register(0x007a_f430, |_, a| ret(a[0]));
        e.register(0x0046_23f0, |_, _| ret(7));
        e.register(0x0082_5c00, |_, _| ret(0x7777));
        noop(
            &mut e,
            &[
                0x0041_b7c0,
                0x0048_4650,
                0x0042_e280,
                0x0045_1ef0,
                0x00c6_a270,
                0x0057_3f40,
                MAP_REMOVE_AT,
                0x009a_51e0,
                0x0043_b370,
            ],
        );
        e.set_global(0x011d_df38, 0x1111u32);
        e.set_global(0x011d_ea10, 0x2222u32);
        e.set_global(0x011f_1958, 0x3333u32);
        e
    }

    #[test]
    fn finishing_ignores_refused_and_undamaged_references() {
        let mut e = finish_engine();
        let root = make_node(&mut e, 0, true, &[]);
        let reference = make_reference(&mut e, root, 100.0);
        e.call_log = Some(vec![]);
        e.call(0x0047_7d10, &args![reference]);
        let log = log_of(&mut e);
        assert_eq!(log.len(), 2);

        // Accepted, but never damaged (-1.0).
        let reference = make_reference(&mut e, root, -1.0);
        e.mem.set_u32(reference + 0x1f0, 1);
        e.call_log = Some(vec![]);
        e.call(0x0047_7d10, &args![reference]);
        let log = log_of(&mut e);
        assert!(calls_to(&log, 0x0041_b7c0).is_empty());
        assert!(calls_to(&log, MAP_REMOVE_AT).is_empty());
    }

    #[test]
    fn finishing_without_a_model_swap_updates_the_damage_stage_nodes() {
        let mut e = finish_engine();
        let root = make_node(&mut e, 0, true, &[]);
        let node = make_node(&mut e, 0, true, &[]);
        let reference = make_reference(&mut e, root, 100.0);
        e.mem.set_u32(reference + 0x1f0, 1);
        e.mem.set_u32(reference + 0x1f4, 1);
        e.mem.set_u32(reference + 0x1f8, 1);
        e.mem.set_u32(reference + 0x1e8, node);
        e.call_log = Some(vec![]);
        e.call(0x0047_7d10, &args![reference]);
        let log = log_of(&mut e);
        assert!(log.contains(&(0x0041_b7c0, vec![reference + 0x180])));
        assert!(log.contains(&(0x0048_4650, vec![reference, 0])));
        assert!(log.contains(&(0x0043_fcd0, vec![reference])));
        assert!(log.contains(&(CHECKED_CAST, vec![0x0120_2e8c, node])));
        assert!(log.contains(&(0x0057_3f40, vec![reference])));
        assert!(log.contains(&(MAP_REMOVE_AT, vec![DESTRUCTIBLE_OBJECTS, reference])));
        assert!(log.contains(&(0x009a_51e0, vec![0x7777, reference])));
        assert!(calls_to(&log, 0x0042_e280).is_empty());
        assert!(calls_to(&log, 0x0045_1ef0).is_empty());
    }

    #[test]
    fn finishing_with_a_model_swap_rebuilds_and_activates_the_3d() {
        let mut e = finish_engine();
        // The root's slot 0xc gives a container whose slot 0x10 gives the
        // collision object.
        e.register(0x0f00_0011, |e, a| ret(e.mem.u32(a[0] + 8)));
        let collision = e.mem.alloc(0x100);
        let container = object_with_vtable(&mut e, 0x20, &[(0x10, 0x0f00_0011)]);
        e.mem.set_u32(container + 8, collision);
        let root = make_node(&mut e, 0, true, &[]);
        e.mem.set_u32(root + 8, container);
        let reference = make_reference(&mut e, root, 100.0);
        e.mem.set_u32(reference + 0x1f0, 1);
        e.mem.set_u32(reference + 0x1fc, 1);
        e.call_log = Some(vec![]);
        e.call(0x0047_7d10, &args![reference]);
        let log = log_of(&mut e);
        assert!(log.contains(&(0x0042_e280, vec![reference + 0x180])));
        // The flag word is cleared around slot 0x1cc and restored with the
        // byte `004623f0` answered.
        assert!(log.contains(&(0x0046_23f0, vec![0x1111, 0])));
        assert!(log.contains(&(0x0046_23f0, vec![0x1111, 7])));
        assert!(log.contains(&(0x0045_1ef0, vec![0x2222, reference, 0x1234, 0, 0])));
        assert!(log.contains(&(0x0043_b370, vec![collision, 1, 0x4000])));
        assert_eq!(e.mem.f32(collision + 0xb4), 1.0);
        assert!(log.contains(&(0x00c6_a270, vec![root, 1, 1, 0])));
        assert!(calls_to(&log, 0x0043_fcd0).is_empty());
        // The reference test said no: `00573f40` is not called.
        assert!(calls_to(&log, 0x0057_3f40).is_empty());
    }

    type WrittenChunk = (u32, Vec<u8>);

    struct SavedChunks {
        chunks: Rc<RefCell<Vec<WrittenChunk>>>,
    }

    fn save_engine(swapped: u32) -> (Engine, SavedChunks) {
        let mut e = engine();
        memory_doubles(&mut e);
        let chunks: Rc<RefCell<Vec<WrittenChunk>>> = Rc::default();
        let sink = chunks.clone();
        e.register_double(0x0048_5990, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.bytes(a[1], a[2])));
            Ret::default()
        });
        let sink = chunks.clone();
        e.register_double(0x0048_56d0, move |_, a| {
            sink.borrow_mut().push((a[0], vec![]));
            Ret::default()
        });
        e.register_double(IS_SWAPPED_BYTE_ORDER, move |_, _| ret(swapped));
        e.register(GET_FORM_ID, |_, a| ret(a[0] | 0x0100_0000));
        noop(&mut e, &[0x0050_3210, 0x0040_1080, 0x0048_a520]);
        (e, SavedChunks { chunks })
    }

    fn save_stages() -> [StageSpec; 2] {
        [
            StageSpec {
                damage_stage: 2,
                percentage: 80,
                flags: 1,
                rate: 5,
                explosion: 0x5000,
                debris: 0x6000,
                count: 3,
                model: 0,
            },
            StageSpec {
                damage_stage: 4,
                percentage: 40,
                flags: 2,
                rate: 0,
                explosion: 0,
                debris: 0,
                count: 0,
                model: 0x7000,
            },
        ]
    }

    #[test]
    fn saving_writes_the_header_the_stages_and_their_models() {
        let (mut e, saved) = save_engine(0);
        let (this, data) = make_component(&mut e, 100, &save_stages());
        e.set(data, DestructibleObjectData::cFlags, 1);
        e.call_log = Some(vec![]);
        e.call(0x0047_7f20, &args![this]);
        let log = log_of(&mut e);
        let chunks = saved.chunks.borrow();
        assert_eq!(chunks[0], (CHUNK_DEST, vec![100, 0, 0, 0, 2, 1, 0, 0]));
        assert_eq!(
            chunks[1],
            (
                CHUNK_DSTD,
                vec![
                    80, 0, 2, 1, 5, 0, 0, 0, 0x00, 0x50, 0x00, 0x01, 0x00, 0x60, 0x00, 0x01, 3, 0,
                    0, 0
                ]
            )
        );
        assert_eq!(chunks[2], (CHUNK_DSTF, vec![]));
        assert_eq!(
            chunks[3],
            (
                CHUNK_DSTD,
                vec![40, 1, 4, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
            )
        );
        assert_eq!(chunks[4], (CHUNK_DSTF, vec![]));
        assert_eq!(chunks.len(), 5);
        // The model of stage 1 is saved between its record and its end.
        let order: Vec<u32> = log
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| [0x0048_5990, 0x0048_a520, 0x0048_56d0].contains(a))
            .collect();
        assert_eq!(
            order,
            [
                0x0048_5990,
                0x0048_5990,
                0x0048_56d0,
                0x0048_5990,
                0x0048_a520,
                0x0048_56d0
            ]
        );
        assert!(log.contains(&(
            0x0048_a520,
            vec![0x7000, CHUNK_DMDL, CHUNK_DMDT, CHUNK_DMDS]
        )));
        // Only the explosion and debris forms that exist are asked.
        assert_eq!(calls_to(&log, GET_FORM_ID).len(), 2);
        assert!(calls_to(&log, 0x0050_3210).is_empty());
        assert!(calls_to(&log, 0x0040_1080).is_empty());
    }

    #[test]
    fn saving_swaps_the_records_around_each_write_when_asked() {
        let (mut e, _saved) = save_engine(1);
        let (this, _) = make_component(&mut e, 100, &save_stages());
        e.call_log = Some(vec![]);
        e.call(0x0047_7f20, &args![this]);
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, 0x0050_3210).len(), 2);
        // Four words, before and after, for each of the two stages.
        assert_eq!(calls_to(&log, 0x0040_1080).len(), 16);
    }

    #[test]
    fn saving_nothing_without_data() {
        let (mut e, saved) = save_engine(0);
        let this: Ptr<BGSDestructibleObjectForm> = e.new_object();
        e.call(0x0047_7f20, &args![this]);
        assert!(saved.chunks.borrow().is_empty());
    }

    #[test]
    fn the_stage_record_swap_touches_the_four_words() {
        let mut e = engine();
        noop(&mut e, &[0x0040_1080]);
        e.call_log = Some(vec![]);
        e.call(0x0047_8130, &args![0x4000u32]);
        let log = log_of(&mut e);
        assert_eq!(
            log[1..],
            [
                (0x0040_1080, vec![0x4004, 0]),
                (0x0040_1080, vec![0x4008, 0]),
                (0x0040_1080, vec![0x400c, 0]),
                (0x0040_1080, vec![0x4010, 0]),
            ]
        );
    }

    #[test]
    fn the_header_constructor_leaves_bytes_six_and_seven() {
        let mut e = engine();
        let header = e.mem.alloc(8);
        e.mem.write(header, &[0xff; 8]);
        assert_eq!(e.call(0x0047_81b0, &args![header]).u32(), header);
        assert_eq!(e.mem.bytes(header, 8), [0, 0, 0, 0, 0, 0, 0xff, 0xff]);
    }

    type FileChunks = Rc<RefCell<(Vec<(u32, Vec<u8>)>, usize)>>;

    /// A plugin file that hands out its chunks in order: `GetTESChunk`
    /// (`004726b0`) takes the next one, the reads (`00472890`) and the
    /// size (`00401660`) are about the one taken last, and `004726f0`
    /// says whether any are left.
    fn file_doubles(e: &mut Engine, chunks: Vec<(u32, Vec<u8>)>, swapped: u32) -> FileChunks {
        let state: FileChunks = Rc::new(RefCell::new((chunks, 0)));
        let s = state.clone();
        e.register_double(0x0047_26b0, move |_, _| {
            let mut s = s.borrow_mut();
            s.1 += 1;
            let tag = s.0.get(s.1 - 1).map_or(0, |chunk| chunk.0);
            ret(tag)
        });
        let s = state.clone();
        e.register_double(0x0047_26f0, move |_, _| {
            let s = s.borrow();
            ret((s.1 < s.0.len()) as u32)
        });
        let s = state.clone();
        e.register_double(0x0040_1660, move |_, _| {
            let s = s.borrow();
            ret(s.0[s.1 - 1].1.len() as u32)
        });
        let s = state.clone();
        e.register_double(0x0047_2890, move |e, a| {
            let s = s.borrow();
            e.mem.write(a[1], &s.0[s.1 - 1].1);
            Ret::default()
        });
        e.register_double(IS_SWAPPED_BYTE_ORDER_FILE, move |_, _| ret(swapped));
        e.register(0x0040_3570, |_, _| ret(0x1_0004));
        state
    }

    const MODEL_SLOT_LOAD: u32 = 0x0f00_0201;
    const MODEL_SLOT_DELETE: u32 = 0x0f00_0202;
    const MODEL_SLOT_COPY: u32 = 0x0f00_0203;
    const MODEL_SLOT_COMPARE: u32 = 0x0f00_0204;
    const FORM_SLOT_NAME: u32 = 0x0f00_0205;

    /// A model object whose vtable has the four slots the unit uses.
    fn make_model(e: &mut Engine) -> u32 {
        object_with_vtable(
            e,
            0x40,
            &[
                (0x08, MODEL_SLOT_COPY),
                (0x0c, MODEL_SLOT_COMPARE),
                (0x10, MODEL_SLOT_DELETE),
                (0x18, MODEL_SLOT_LOAD),
            ],
        )
    }

    fn model_doubles(e: &mut Engine) {
        noop(e, &[MODEL_SLOT_LOAD, MODEL_SLOT_DELETE, MODEL_SLOT_COPY]);
        // Two models "differ" when the word at +0x10 of the first is set.
        e.register(MODEL_SLOT_COMPARE, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(0x0048_a3d0, |e, _| ret(make_model(e)));
    }

    fn load_engine(chunks: Vec<(u32, Vec<u8>)>, swapped: u32) -> (Engine, FileChunks) {
        let mut e = engine();
        memory_doubles(&mut e);
        model_doubles(&mut e);
        e.register(GET_COMPONENT_DATA, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0052_8cb0, |_, _| Ret::default());
        e.register(0x0047_5220, |_, _| Ret::default());
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(GET_FORM_ID, |_, _| ret(0xabc));
        e.register(FORM_SLOT_NAME, |_, _| ret(0x4444));
        noop(
            &mut e,
            &[
                LOG_MESSAGE,
                0x0048_93e0,
                0x0048_a7b0,
                0x0050_3210,
                0x0040_1080,
            ],
        );
        let state = file_doubles(&mut e, chunks, swapped);
        (e, state)
    }

    fn stage_record() -> Vec<u8> {
        let mut record = vec![70, 1, 3, 5];
        record.extend_from_slice(&9u32.to_le_bytes());
        record.extend_from_slice(&0x5000u32.to_le_bytes());
        record.extend_from_slice(&0x6000u32.to_le_bytes());
        record.extend_from_slice(&4u32.to_le_bytes());
        record
    }

    #[test]
    fn a_stage_record_fills_its_stage_and_the_model_chunks_follow() {
        let (mut e, state) = load_engine(
            vec![
                (CHUNK_DSTD, stage_record()),
                (CHUNK_DMDL, b"abc\0".to_vec()),
                (CHUNK_DMDT, vec![]),
                (CHUNK_DMDS, vec![1, 2, 3]),
                (CHUNK_DSTF, vec![]),
                (CHUNK_DMDL, b"zzz\0".to_vec()),
            ],
            0,
        );
        let path: Rc<RefCell<Vec<u8>>> = Rc::default();
        let sink = path.clone();
        e.register_double(MODEL_SLOT_LOAD, move |e, a| {
            *sink.borrow_mut() = e.mem.bytes(a[1], 4);
            Ret::default()
        });
        let spec = StageSpec {
            percentage: 10,
            ..StageSpec::default()
        };
        let (this, data) = make_component(&mut e, 100, &[spec, spec]);
        let file = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0047_81e0, &args![this, file]);
        let log = log_of(&mut e);
        let stage = stage_at(&e, data, 1);
        assert_eq!(e.get(stage, DestructibleObjectStage::cHealthPercentage), 70);
        assert_eq!(e.get(stage, DestructibleObjectStage::cModelDamageStage), 3);
        assert_eq!(e.get(stage, DestructibleObjectStage::cFlags), 5);
        assert_eq!(
            e.get(stage, DestructibleObjectStage::iSelfDamagePerSecond),
            9
        );
        assert_eq!(
            e.get(stage, DestructibleObjectStage::pExplosion).addr(),
            0x5000
        );
        assert_eq!(
            e.get(stage, DestructibleObjectStage::pDebris).addr(),
            0x6000
        );
        assert_eq!(e.get(stage, DestructibleObjectStage::iDebrisCount), 4);
        // Stage 0 is untouched.
        let other = stage_at(&e, data, 0);
        assert_eq!(e.get(other, DestructibleObjectStage::cHealthPercentage), 10);
        // The model was created once and got the three chunks.
        let model = e
            .get(stage, DestructibleObjectStage::pReplacementModel)
            .addr();
        assert_ne!(model, 0);
        assert_eq!(calls_to(&log, 0x0048_a3d0).len(), 1);
        assert_eq!(*path.borrow(), b"abc\0");
        assert!(log.contains(&(0x0048_93e0, vec![model, file])));
        assert!(log.contains(&(0x0048_a7b0, vec![model, log_buffer(&log), 3, 4])));
        // `DSTF` ended it: three "any left" questions, four chunk reads.
        assert_eq!(calls_to(&log, 0x0047_26f0).len(), 3);
        assert_eq!(calls_to(&log, 0x0047_26b0).len(), 5);
        assert_eq!(state.borrow().1, 5);
        assert!(calls_to(&log, 0x0040_1080).is_empty());
    }

    /// The buffer address `0048a7b0` was given (its second word).
    fn log_buffer(log: &[(u32, Vec<u32>)]) -> u32 {
        calls_to(log, 0x0048_a7b0)[0][1]
    }

    #[test]
    fn a_stage_record_beyond_the_stage_count_is_ignored() {
        let mut record = stage_record();
        record[1] = 5;
        let (mut e, _state) = load_engine(
            vec![(CHUNK_DSTD, record), (CHUNK_DMDL, b"abc\0".to_vec())],
            1,
        );
        let (this, _) = make_component(&mut e, 100, &[StageSpec::default()]);
        let file = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0047_81e0, &args![this, file]);
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, 0x0047_26b0).len(), 1);
        assert_eq!(calls_to(&log, 0x0040_1080).len(), 4);
        assert!(calls_to(&log, 0x0048_a3d0).is_empty());
    }

    #[test]
    fn the_header_chunk_fills_the_data_and_allocates_the_stages() {
        let mut header = 500u32.to_le_bytes().to_vec();
        header.extend_from_slice(&[3, 2, 0, 0]);
        let (mut e, _state) = load_engine(vec![(CHUNK_DEST, header)], 1);
        // No data yet: `LoadChunk` creates it first.
        let this: Ptr<BGSDestructibleObjectForm> = e.new_object();
        let file = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0047_81e0, &args![this, file]);
        let log = log_of(&mut e);
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        assert!(!data.is_null());
        assert_eq!(e.get(data, DestructibleObjectData::iHealth), 500);
        assert_eq!(e.get(data, DestructibleObjectData::cNumStages), 3);
        assert_eq!(e.get(data, DestructibleObjectData::cFlags), 2);
        for index in 0..3 {
            assert!(!stage_at(&e, data, index).is_null());
        }
        let sizes: Vec<u32> = calls_to(&log, OPERATOR_NEW).iter().map(|a| a[0]).collect();
        assert_eq!(sizes, [0x14, 12, 0x18, 0x18, 0x18]);
        assert_eq!(calls_to(&log, 0x0050_3210).len(), 1);
    }

    #[test]
    fn an_old_format_header_is_logged_and_the_data_dropped() {
        let (mut e, _state) = load_engine(vec![(CHUNK_DEST, vec![0; 4])], 0);
        let (this, data) = make_component(&mut e, 100, &[]);
        let table = e.mem.alloc(0x400);
        e.mem.set_u32(table + 0x130, FORM_SLOT_NAME);
        e.mem.set_u32(this.addr(), table);
        let file = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0047_81e0, &args![this, file]);
        let log = log_of(&mut e);
        assert!(log.contains(&(LOG_MESSAGE, vec![0x0101_a6c8, 0xabc, 0x4444])));
        assert!(log.contains(&(
            DYNAMIC_CAST,
            vec![this.addr(), 0, 0x0118_32ac, 0x0118_3028, 0]
        )));
        assert!(log.contains(&(0x0047_5220, vec![data.addr()])));
        assert!(log.contains(&(OPERATOR_DELETE, vec![data.addr()])));
        assert!(e.get(this, BGSDestructibleObjectForm::pData).is_null());
    }

    #[test]
    fn loading_needs_a_form_and_a_file() {
        let (mut e, _state) = load_engine(vec![], 0);
        let this: Ptr<BGSDestructibleObjectForm> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0047_81e0, &args![this, 0u32]);
        e.call(0x0047_81e0, &args![0u32, 0x1000u32]);
        assert_eq!(log_of(&mut e).len(), 2);
    }

    #[test]
    fn a_stage_gets_its_model_only_once() {
        let mut e = engine();
        memory_doubles(&mut e);
        model_doubles(&mut e);
        let stage: Ptr<DestructibleObjectStage> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0047_8570, &args![stage]);
        let model = e.get(stage, DestructibleObjectStage::pReplacementModel);
        assert!(!model.is_null());
        let log = log_of(&mut e);
        assert!(log.contains(&(OPERATOR_NEW, vec![0x20])));
        e.call_log = Some(vec![]);
        e.call(0x0047_8570, &args![stage]);
        assert_eq!(log_of(&mut e).len(), 1);
        assert_eq!(
            e.get(stage, DestructibleObjectStage::pReplacementModel),
            model
        );
    }

    #[test]
    fn a_component_gets_its_data_only_once() {
        let mut e = engine();
        memory_doubles(&mut e);
        e.register(0x0052_8cb0, |_, _| Ret::default());
        let this: Ptr<BGSDestructibleObjectForm> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0047_8600, &args![this]);
        let log = log_of(&mut e);
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        assert!(!data.is_null());
        assert!(log.contains(&(OPERATOR_NEW, vec![0x14])));
        assert!(log.contains(&(0x0052_8cb0, vec![data.addr() + 0x10, 0])));
        e.call_log = Some(vec![]);
        e.call(0x0047_8600, &args![this]);
        assert_eq!(log_of(&mut e).len(), 1);
    }

    #[test]
    fn deleting_the_data_clears_the_pointer() {
        let mut e = engine();
        noop(&mut e, &[0x0047_5220, OPERATOR_DELETE]);
        let (this, data) = make_component(&mut e, 100, &[]);
        e.call_log = Some(vec![]);
        e.call(0x0047_8690, &args![this]);
        let log = log_of(&mut e);
        assert!(log.contains(&(0x0047_5220, vec![data.addr()])));
        assert!(log.contains(&(OPERATOR_DELETE, vec![data.addr()])));
        assert!(e.get(this, BGSDestructibleObjectForm::pData).is_null());
        e.call_log = Some(vec![]);
        e.call(0x0047_8690, &args![this]);
        assert_eq!(log_of(&mut e).len(), 1);
    }

    fn copy_engine() -> Engine {
        let mut e = engine();
        memory_doubles(&mut e);
        model_doubles(&mut e);
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(GET_COMPONENT_DATA, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(0x0052_8cb0, |_, _| Ret::default());
        e.register(0x0047_5220, |_, _| Ret::default());
        e.register(0x0047_52e0, |_, _| Ret::default());
        e
    }

    #[test]
    fn copying_creates_the_data_and_copies_the_stages() {
        let mut e = copy_engine();
        let source_stages = [
            StageSpec {
                damage_stage: 1,
                percentage: 90,
                flags: 2,
                rate: 3,
                explosion: 0x5000,
                debris: 0x6000,
                count: 7,
                model: 0,
            },
            StageSpec {
                percentage: 30,
                ..StageSpec::default()
            },
        ];
        let (other, source) = make_component(&mut e, 77, &source_stages);
        e.set(source, DestructibleObjectData::cFlags, 3);
        let this: Ptr<BGSDestructibleObjectForm> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0047_86e0, &args![this, other]);
        let log = log_of(&mut e);
        assert!(log.contains(&(
            DYNAMIC_CAST,
            vec![other.addr(), 0, 0x0118_3040, 0x0118_32ac, 0]
        )));
        let data = e.get(this, BGSDestructibleObjectForm::pData);
        assert_ne!(data, source);
        assert_eq!(e.get(data, DestructibleObjectData::iHealth), 77);
        assert_eq!(e.get(data, DestructibleObjectData::cFlags), 3);
        assert_eq!(e.get(data, DestructibleObjectData::cNumStages), 2);
        let stage = stage_at(&e, data, 0);
        assert_ne!(stage, stage_at(&e, source, 0));
        assert_eq!(e.get(stage, DestructibleObjectStage::cHealthPercentage), 90);
        assert_eq!(e.get(stage, DestructibleObjectStage::cModelDamageStage), 1);
        assert_eq!(e.get(stage, DestructibleObjectStage::cFlags), 2);
        assert_eq!(
            e.get(stage, DestructibleObjectStage::iSelfDamagePerSecond),
            3
        );
        assert_eq!(
            e.get(stage, DestructibleObjectStage::pExplosion).addr(),
            0x5000
        );
        assert_eq!(
            e.get(stage, DestructibleObjectStage::pDebris).addr(),
            0x6000
        );
        assert_eq!(e.get(stage, DestructibleObjectStage::iDebrisCount), 7);
        let stage = stage_at(&e, data, 1);
        assert_eq!(e.get(stage, DestructibleObjectStage::cHealthPercentage), 30);
        assert!(calls_to(&log, 0x0047_52e0).is_empty());
    }

    #[test]
    fn copying_over_existing_data_deletes_its_stages_first() {
        let mut e = copy_engine();
        let (other, _) = make_component(
            &mut e,
            50,
            &[StageSpec {
                percentage: 12,
                ..StageSpec::default()
            }],
        );
        let (this, old) = make_component(&mut e, 10, &[StageSpec::default(); 3]);
        let old_array = e.get(old, DestructibleObjectData::pStagesArray);
        let old_stage = stage_at(&e, old, 1);
        e.call_log = Some(vec![]);
        e.call(0x0047_86e0, &args![this, other]);
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, 0x0047_52e0).len(), 3);
        assert!(log.contains(&(0x0047_52e0, vec![old_stage.addr(), 1])));
        assert!(log.contains(&(OPERATOR_DELETE, vec![old_array.addr()])));
        // The same data object is reused.
        assert_eq!(e.get(this, BGSDestructibleObjectForm::pData), old);
        assert_eq!(e.get(old, DestructibleObjectData::iHealth), 50);
        assert_eq!(e.get(old, DestructibleObjectData::cNumStages), 1);
        assert_ne!(e.get(old, DestructibleObjectData::pStagesArray), old_array);
        let stage = stage_at(&e, old, 0);
        assert_eq!(e.get(stage, DestructibleObjectStage::cHealthPercentage), 12);
    }

    #[test]
    fn copying_from_nothing_deletes_the_data() {
        let mut e = copy_engine();
        let (this, data) = make_component(&mut e, 10, &[]);
        let empty: Ptr<BGSDestructibleObjectForm> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0047_86e0, &args![this, empty]);
        let log = log_of(&mut e);
        assert!(log.contains(&(0x0047_5220, vec![data.addr()])));
        assert!(e.get(this, BGSDestructibleObjectForm::pData).is_null());

        // The cast to the component type fails: nothing happens.
        let (this, data) = make_component(&mut e, 10, &[]);
        e.call_log = Some(vec![]);
        e.call(0x0047_86e0, &args![this, 0u32]);
        assert_eq!(log_of(&mut e).len(), 2);
        assert_eq!(e.get(this, BGSDestructibleObjectForm::pData), data);
    }

    fn base_spec() -> StageSpec {
        StageSpec {
            damage_stage: 1,
            percentage: 50,
            flags: 1,
            rate: 3,
            explosion: 0x5000,
            debris: 0x6000,
            count: 2,
            model: 0,
        }
    }

    /// Whether `00478900` finds the other form different after `change`
    /// was applied to its data and its second stage.
    fn differs_after(
        change: impl FnOnce(&mut Engine, Ptr<DestructibleObjectData>, Ptr<DestructibleObjectStage>),
    ) -> bool {
        let mut e = copy_engine();
        let spec = base_spec();
        let (ours, _) = make_component(&mut e, 100, &[spec, spec]);
        let (theirs, theirs_data) = make_component(&mut e, 100, &[spec, spec]);
        let stage = stage_at(&e, theirs_data, 1);
        change(&mut e, theirs_data, stage);
        e.call(0x0047_8900, &args![ours, theirs]).bool()
    }

    #[test]
    fn identical_data_does_not_differ() {
        assert!(!differs_after(|_, _, _| {}));
    }

    #[test]
    fn every_compared_field_makes_the_data_differ() {
        assert!(differs_after(|e, d, _| e.set(
            d,
            DestructibleObjectData::iHealth,
            101
        )));
        assert!(differs_after(|e, d, _| e.set(
            d,
            DestructibleObjectData::cFlags,
            1
        )));
        assert!(differs_after(|e, d, _| e.set(
            d,
            DestructibleObjectData::cNumStages,
            3
        )));
        assert!(differs_after(|e, _, s| e.set(
            s,
            DestructibleObjectStage::cHealthPercentage,
            51
        )));
        assert!(differs_after(|e, _, s| e.set(
            s,
            DestructibleObjectStage::iSelfDamagePerSecond,
            4
        )));
        assert!(differs_after(|e, _, s| e.set(
            s,
            DestructibleObjectStage::cModelDamageStage,
            2
        )));
        assert!(differs_after(|e, _, s| e.set(
            s,
            DestructibleObjectStage::cFlags,
            0
        )));
        assert!(differs_after(|e, _, s| e.set(
            s,
            DestructibleObjectStage::pExplosion,
            Ptr::new(0x5001)
        )));
        assert!(differs_after(|e, _, s| e.set(
            s,
            DestructibleObjectStage::pDebris,
            Ptr::new(0)
        )));
        // The debris count is compared with itself: no difference.
        assert!(!differs_after(|e, _, s| e.set(
            s,
            DestructibleObjectStage::iDebrisCount,
            99
        )));
    }

    #[test]
    fn the_replacement_models_are_compared_by_presence_and_by_the_model() {
        // Only the other has one.
        assert!(differs_after(|e, _, s| {
            let model = make_model(e);
            e.set(
                s,
                DestructibleObjectStage::pReplacementModel,
                Ptr::new(model),
            );
        }));
        // Both have one: the answer of slot 0x0c (about our model) counts.
        for (answer, expected) in [(0u32, false), (1, true)] {
            let mut e = copy_engine();
            let ours_model = make_model(&mut e);
            let theirs_model = make_model(&mut e);
            e.mem.set_u32(ours_model + 0x10, answer);
            let spec = StageSpec {
                model: ours_model,
                ..base_spec()
            };
            let (ours, _) = make_component(&mut e, 100, &[spec]);
            let spec = StageSpec {
                model: theirs_model,
                ..base_spec()
            };
            let (theirs, _) = make_component(&mut e, 100, &[spec]);
            e.call_log = Some(vec![]);
            let result = e.call(0x0047_8900, &args![ours, theirs]).bool();
            assert_eq!(result, expected);
            let log = log_of(&mut e);
            assert!(log.contains(&(MODEL_SLOT_COMPARE, vec![ours_model, theirs_model])));
        }
    }

    #[test]
    fn missing_data_and_failed_casts_differ_unless_both_are_empty() {
        let mut e = copy_engine();
        let (with_data, _) = make_component(&mut e, 100, &[]);
        let without: Ptr<BGSDestructibleObjectForm> = e.new_object();
        let other_without: Ptr<BGSDestructibleObjectForm> = e.new_object();
        assert!(e.call(0x0047_8900, &args![with_data, without]).bool());
        assert!(e.call(0x0047_8900, &args![without, with_data]).bool());
        assert!(!e.call(0x0047_8900, &args![without, other_without]).bool());
        assert!(e.call(0x0047_8900, &args![with_data, 0u32]).bool());
    }

    const OWNER_NAME_FLAG: u32 = 0x100;

    fn init_engine() -> Engine {
        let mut e = engine();
        model_doubles(&mut e);
        e.register(0x0048_4e60, |_, _| ret(0x9000));
        // `AddCompileIndex` makes the id absolute: adds 0x1000.
        e.register(0x0048_5d50, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 0x1000);
            Ret::default()
        });
        // The lookup finds every form except those with bit 5 set.
        e.register(0x0048_39c0, |_, a| {
            ret(if a[0] & 0x20 != 0 { 0 } else { a[0] })
        });
        e.register(DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(0x0047_4cb0, |e, a| ret(e.mem.u32(a[0] + OWNER_NAME_FLAG)));
        e.register(GET_FORM_ID, |_, _| ret(0xabcdef));
        e.register(FORM_SLOT_NAME, |_, _| ret(0x4444));
        e.register(GET_FORM_TYPE, |e, a| {
            ret(if a[0] == 0 {
                0
            } else {
                e.mem.u8(a[0] + 4) as u32
            })
        });
        e.register(0x005f_0b00, |_, _| ret(777));
        noop(&mut e, &[LOG_MESSAGE, 0x0048_aa80]);
        e
    }

    fn make_owner(e: &mut Engine, form_type: u8, named: bool) -> u32 {
        let owner = object_with_vtable(e, 0x200, &[(0x130, FORM_SLOT_NAME)]);
        e.mem.set_u8(owner + 4, form_type);
        e.mem.set_u32(owner + OWNER_NAME_FLAG, named as u32);
        owner
    }

    #[test]
    fn init_item_resolves_the_ids_and_logs_what_it_cannot_find() {
        let mut e = init_engine();
        let owner = make_owner(&mut e, 0x20, true);
        // The explosion id resolves (0x10 + 0x1000); the debris (0x20) does not.
        let spec = StageSpec {
            explosion: 0x10,
            debris: 0x20,
            model: 0x7000,
            ..StageSpec::default()
        };
        let (this, data) = make_component(&mut e, 100, &[spec]);
        e.call_log = Some(vec![]);
        e.call(0x0047_8be0, &args![this, owner]);
        let log = log_of(&mut e);
        let stage = stage_at(&e, data, 0);
        assert_eq!(
            e.get(stage, DestructibleObjectStage::pExplosion).addr(),
            0x1010
        );
        assert!(e.get(stage, DestructibleObjectStage::pDebris).is_null());
        assert!(log.contains(&(0x0048_4e60, vec![owner, 0xffff_ffff])));
        assert!(log.contains(&(DYNAMIC_CAST, vec![0x1010, 0, 0x0118_3028, 0x0118_620c, 0])));
        assert!(log.contains(&(DYNAMIC_CAST, vec![0, 0, 0x0118_3028, 0x0118_61f4, 0])));
        assert_eq!(calls_to(&log, LOG_MESSAGE).len(), 1);
        assert!(log.contains(&(LOG_MESSAGE, vec![0x0101_a7b8, 0, 0x1020, 0x4444])));
        assert!(log.contains(&(0x0048_aa80, vec![0x7000, owner])));
        // Form type 0x20: the health stays.
        assert_eq!(e.get(data, DestructibleObjectData::iHealth), 100);
    }

    #[test]
    fn init_item_names_the_owner_by_id_without_a_name_and_as_unknown_without_one() {
        // Explosion that does not resolve, owner without a name.
        let mut e = init_engine();
        let owner = make_owner(&mut e, 0x20, false);
        let spec = StageSpec {
            explosion: 0x20,
            ..StageSpec::default()
        };
        let (this, _) = make_component(&mut e, 100, &[spec]);
        e.call_log = Some(vec![]);
        e.call(0x0047_8be0, &args![this, owner]);
        let log = log_of(&mut e);
        assert!(log.contains(&(LOG_MESSAGE, vec![0x0101_a848, 0, 0x1020, 0xabcdef])));

        // Unknown owner; both fail; the second stage has index 1.
        let mut e = init_engine();
        let failing = StageSpec {
            explosion: 0x20,
            debris: 0x20,
            ..StageSpec::default()
        };
        let (this, _) = make_component(&mut e, 100, &[StageSpec::default(), failing]);
        e.call_log = Some(vec![]);
        e.call(0x0047_8be0, &args![this, 0u32]);
        let log = log_of(&mut e);
        assert!(log.contains(&(LOG_MESSAGE, vec![0x0101_a800, 1, 0x1020])));
        assert!(log.contains(&(LOG_MESSAGE, vec![0x0101_a720, 1, 0x1020])));
        assert_eq!(calls_to(&log, LOG_MESSAGE).len(), 2);
    }

    #[test]
    fn init_item_takes_the_health_of_actor_bases() {
        for (form_type, health) in [(0x29u8, 100u32), (0x2a, 777), (0x2b, 777), (0x2c, 100)] {
            let mut e = init_engine();
            let owner = make_owner(&mut e, form_type, false);
            let (this, data) = make_component(&mut e, 100, &[]);
            e.call(0x0047_8be0, &args![this, owner]);
            assert_eq!(e.get(data, DestructibleObjectData::iHealth), health);
        }
        // No data: nothing is asked.
        let mut e = init_engine();
        let owner = make_owner(&mut e, 0x2a, false);
        let this: Ptr<BGSDestructibleObjectForm> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0047_8be0, &args![this, owner]);
        assert_eq!(log_of(&mut e).len(), 1);
    }

    #[test]
    fn the_stage_array_is_allocated_with_a_stage_for_each_pointer() {
        let mut e = engine();
        memory_doubles(&mut e);
        let this: Ptr<BGSDestructibleObjectForm> = e.new_object();
        assert!(e
            .call(0x0047_8e90, &args![this, 0u32])
            .ptr::<()>()
            .is_null());
        e.call_log = Some(vec![]);
        let array = e.call(0x0047_8e90, &args![this, 3u32]).u32();
        let log = log_of(&mut e);
        let sizes: Vec<u32> = calls_to(&log, OPERATOR_NEW).iter().map(|a| a[0]).collect();
        assert_eq!(sizes, [12, 0x18, 0x18, 0x18]);
        let stages: Vec<u32> = (0..3).map(|i| e.mem.u32(array + 4 * i)).collect();
        assert!(stages.iter().all(|s| *s != 0));
        assert_ne!(stages[0], stages[1]);
        assert_ne!(stages[1], stages[2]);
    }

    #[test]
    fn a_failed_stage_allocation_leaves_a_null_pointer() {
        let mut e = engine();
        let mut calls = 0;
        e.register_double(OPERATOR_NEW, move |e, a| {
            calls += 1;
            ret(if calls == 2 { 0 } else { e.mem.alloc(a[0]) })
        });
        let this: Ptr<BGSDestructibleObjectForm> = e.new_object();
        let array = e.call(0x0047_8e90, &args![this, 2u32]).u32();
        assert_eq!(e.mem.u32(array), 0);
        assert_ne!(e.mem.u32(array + 4), 0);
    }

    #[test]
    fn the_stage_constructor_zeroes_all_but_byte_three() {
        let mut e = engine();
        let stage: Ptr<DestructibleObjectStage> = e.new_object();
        e.mem.write(stage.addr(), &[0xff; 0x18]);
        assert_eq!(
            e.call(0x0047_8f70, &args![stage]).ptr::<()>().addr(),
            stage.addr()
        );
        let bytes = e.mem.bytes(stage.addr(), 0x18);
        assert_eq!(bytes[..3], [0, 0, 0]);
        assert_eq!(bytes[3], 0xff);
        assert!(bytes[4..].iter().all(|b| *b == 0));
    }

    #[test]
    fn copying_a_stage_replaces_its_model_with_a_copy() {
        let mut e = engine();
        memory_doubles(&mut e);
        model_doubles(&mut e);
        let from_model = make_model(&mut e);
        let old_model = make_model(&mut e);
        let from = StageSpec {
            percentage: 33,
            model: from_model,
            ..base_spec()
        };
        let to = StageSpec {
            model: old_model,
            ..StageSpec::default()
        };
        let (_, from_data) = make_component(&mut e, 10, &[from]);
        let (_, to_data) = make_component(&mut e, 10, &[to]);
        let from_stage = stage_at(&e, from_data, 0);
        let to_stage = stage_at(&e, to_data, 0);
        e.call_log = Some(vec![]);
        e.call(0x0047_8fd0, &args![to_stage, from_stage]);
        let log = log_of(&mut e);
        assert_eq!(
            e.get(to_stage, DestructibleObjectStage::cHealthPercentage),
            33
        );
        assert_eq!(e.get(to_stage, DestructibleObjectStage::iDebrisCount), 2);
        assert_eq!(
            e.get(to_stage, DestructibleObjectStage::pExplosion).addr(),
            0x5000
        );
        assert!(log.contains(&(MODEL_SLOT_DELETE, vec![old_model, 1])));
        let new_model = e
            .get(to_stage, DestructibleObjectStage::pReplacementModel)
            .addr();
        assert_ne!(new_model, old_model);
        assert_ne!(new_model, from_model);
        assert!(log.contains(&(OPERATOR_NEW, vec![0x20])));
        assert!(log.contains(&(MODEL_SLOT_COPY, vec![new_model, from_model])));
    }

    #[test]
    fn copying_a_stage_without_a_model_drops_the_model_and_a_null_source_does_nothing() {
        let mut e = engine();
        memory_doubles(&mut e);
        model_doubles(&mut e);
        let old_model = make_model(&mut e);
        let to = StageSpec {
            model: old_model,
            ..StageSpec::default()
        };
        let (_, to_data) = make_component(&mut e, 10, &[to]);
        let (_, from_data) = make_component(&mut e, 10, &[base_spec()]);
        let to_stage = stage_at(&e, to_data, 0);
        let from_stage = stage_at(&e, from_data, 0);
        e.call(0x0047_8fd0, &args![to_stage, 0u32]);
        assert_eq!(
            e.get(to_stage, DestructibleObjectStage::pReplacementModel)
                .addr(),
            old_model
        );
        e.call_log = Some(vec![]);
        e.call(0x0047_8fd0, &args![to_stage, from_stage]);
        let log = log_of(&mut e);
        assert!(log.contains(&(MODEL_SLOT_DELETE, vec![old_model, 1])));
        assert!(e
            .get(to_stage, DestructibleObjectStage::pReplacementModel)
            .is_null());
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
    }

    #[test]
    fn the_node_array_constructor_and_destructors_set_the_vtable() {
        let mut e = engine();
        noop(&mut e, &[0x006b_3eb0, 0x0084_54f0, OPERATOR_DELETE]);
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_9110, &args![array]).u32(), array);
        let log = log_of(&mut e);
        assert_eq!(e.mem.u32(array), 0x0101_a8e8);
        assert!(log.contains(&(0x006b_3eb0, vec![array, 0, 0])));

        e.mem.set_u32(array, 0);
        e.call_log = Some(vec![]);
        e.call(0x0047_9140, &args![array]);
        let log = log_of(&mut e);
        assert_eq!(e.mem.u32(array), 0x0101_a8e8);
        assert!(log.contains(&(0x0084_54f0, vec![array, 1])));
    }

    #[test]
    fn the_node_array_deleting_destructor_frees_when_asked() {
        let mut e = engine();
        noop(&mut e, &[0x0084_54f0, OPERATOR_DELETE]);
        let array = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_9160, &args![array, 0u32]).u32(), array);
        let log = log_of(&mut e);
        assert!(log.contains(&(0x0084_54f0, vec![array, 1])));
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x0047_9160, &args![array, 1u32]);
        let log = log_of(&mut e);
        assert!(log.contains(&(OPERATOR_DELETE, vec![array])));
    }

    fn map_engine() -> Engine {
        let mut e = engine();
        memory_doubles(&mut e);
        e.register(0x00aa_1070, |e, a| ret(e.mem.alloc(a[0])));
        noop(&mut e, &[0x00aa_10f0, 0x0043_8af0]);
        e
    }

    #[test]
    fn the_map_base_constructor_allocates_cleared_buckets() {
        let mut e = map_engine();
        let map = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0047_91f0, &args![map, 8u32]).u32(), map);
        let log = log_of(&mut e);
        assert_eq!(e.mem.u32(map), 0x0101_a91c);
        assert_eq!(e.mem.u32(map + 4), 8);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        let buckets = e.mem.u32(map + 8);
        assert!(log.contains(&(0x00aa_1070, vec![32])));
        assert!(log.contains(&(MEMSET, vec![buckets, 0, 32])));
    }

    #[test]
    fn the_map_constructor_sets_the_derived_vtable() {
        let mut e = map_engine();
        let map = e.mem.alloc(0x10);
        assert_eq!(e.call(0x0047_9190, &args![map, 4u32]).u32(), map);
        assert_eq!(e.mem.u32(map), 0x0101_a8fc);
        assert_eq!(e.mem.u32(map + 4), 4);
        assert_ne!(e.mem.u32(map + 8), 0);
    }

    #[test]
    fn the_map_destructors_empty_the_map_and_free_the_buckets() {
        let mut e = map_engine();
        let map = e.mem.alloc(0x10);
        e.mem.set_u32(map + 8, 0x4444);
        e.call_log = Some(vec![]);
        e.call(0x0047_92c0, &args![map]);
        let log = log_of(&mut e);
        assert_eq!(e.mem.u32(map), 0x0101_a91c);
        assert_eq!(
            log[1..],
            [(0x0043_8af0, vec![map]), (0x00aa_10f0, vec![0x4444])]
        );

        // The derived body empties the map once itself and once in the base.
        e.call_log = Some(vec![]);
        e.call(0x0047_9260, &args![map]);
        let log = log_of(&mut e);
        assert_eq!(calls_to(&log, 0x0043_8af0).len(), 2);
        assert_eq!(calls_to(&log, 0x00aa_10f0).len(), 1);
        assert_eq!(e.mem.u32(map), 0x0101_a91c);
    }

    #[test]
    fn the_map_deleting_destructors_free_only_when_asked() {
        for address in [0x0047_91c0u32, 0x0047_92f0] {
            let mut e = map_engine();
            let map = e.mem.alloc(0x10);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![map, 0u32]).u32(), map);
            assert!(calls_to(&log_of(&mut e), OPERATOR_DELETE).is_empty());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(address, &args![map, 1u32]).u32(), map);
            assert!(log_of(&mut e).contains(&(OPERATOR_DELETE, vec![map])));
        }
    }
}
