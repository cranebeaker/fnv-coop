//! `fallout shared/run_004de420` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds two groups of functions that the linker placed together
//! (the Xbox PDB names none of them):
//!
//! - the `INIPrefSettingCollection` singleton (`004de420`..`004de550`) and
//!   the destructor of `SettingT<INIPrefSettingCollection>`, whose vtable
//!   (`01022738`) and run-time type information name the classes;
//! - the static state of `ScreenCustomSplatter` (`004de580`..`004dea80`):
//!   the extra data tag `"CSD"` (`BSCustomSplatterExtraData::pTag`), the
//!   per-frame step over the splatter children of `ScreenCustomSplatter::spRoot`
//!   and the function that adds a splatter. Their statics are the ones
//!   the Xbox PDB lists for `ScreenCustomSplatter` (`bEnabled`,
//!   `bSecondsForFade`, `fFadeTime`, `uiCount`, `spRoot`); the three float
//!   statics at `011feda4`, `011ad874` and `011ad878` are read by the blood
//!   splatter shader (`BSShaderBloodSplatter::CreateConstantMaps`,
//!   `00b80c20`).
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results. The translations compute in `f64` and round to `f32` where the
//! code stores a `float`.

#[allow(unused_imports)]
use crate::prelude::*;

/// The `INIPrefSettingCollection` singleton pointer (null until first use).
const INI_PREF_SETTING_COLLECTION_INSTANCE: u32 = 0x011f_35a0;
/// Size of an `INIPrefSettingCollection` (Xbox PDB `0x114`; the PC
/// allocation is the same).
const INI_PREF_SETTING_COLLECTION_SIZE: u32 = 0x114;
/// Vtable of `INIPrefSettingCollection` (RTTI `.?AVINIPrefSettingCollection@@`).
const INI_PREF_SETTING_COLLECTION_VTABLE: u32 = 0x0102_2744;
/// Vtable of `SettingT<INIPrefSettingCollection>` (RTTI
/// `.?AV?$SettingT@VINIPrefSettingCollection@@@@`).
const SETTING_T_INI_PREF_VTABLE: u32 = 0x0102_2738;
/// Vtable byte offset of `Remove` in the setting collection (Xbox PDB
/// `SettingCollection<Setting>::Remove`, `0044f870` on the PC).
const COLLECTION_REMOVE_SLOT: u32 = 0x08;

/// Setting collection base constructor (`SettingCollectionList<Setting>`).
const SETTING_COLLECTION_LIST_CONSTRUCT: u32 = 0x0044_f600;
/// The destructor body that `INIPrefSettingCollection`'s deleting destructor
/// runs.
const SETTING_COLLECTION_DESTRUCT: u32 = 0x0045_d240;
/// `Setting` destructor body (sets the base vtable, frees a name that
/// starts with `'S'`).
const SETTING_DESTRUCT: u32 = 0x0040_4950;
/// `operator new(size)`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(pointer)`.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Allocator used for the 0x1c byte extra data (`NiAlloc`-style, size).
const ALLOCATE: u32 = 0x00aa_13e0;
/// `NiFixedString(const char*)` constructor.
const FIXED_STRING_CONSTRUCT: u32 = 0x0043_8170;
/// `NiFixedString` destructor.
const FIXED_STRING_DESTRUCT: u32 = 0x0043_81b0;
/// `NiExtraData(const NiFixedString& name)` constructor.
const EXTRA_DATA_CONSTRUCT: u32 = 0x00a7_b440;

/// `BSCustomSplatterExtraData::pTag` (Xbox PDB static `NiFixedString*`).
const SPLATTER_TAG: u32 = 0x011c_77e4;
/// `"CSD"`: the text of the tag.
const SPLATTER_TAG_TEXT: u32 = 0x0102_2898;
/// `ScreenCustomSplatter::uiCount` (Xbox PDB static).
const SPLATTER_COUNT: u32 = 0x011c_77dc;
/// `ScreenCustomSplatter::bEnabled` (Xbox PDB static, one byte).
const SPLATTER_ENABLED: u32 = 0x011c_77e8;
/// `ScreenCustomSplatter::bSecondsForFade` (Xbox PDB static, one byte).
const SPLATTER_SECONDS_FOR_FADE: u32 = 0x011c_77e9;
/// `ScreenCustomSplatter::fFadeTime` (Xbox PDB static, `float`).
const SPLATTER_FADE_TIME: u32 = 0x011c_77ec;
/// `ScreenCustomSplatter::spRoot` (Xbox PDB static `NiPointer<NiNode>`).
const SPLATTER_ROOT: u32 = 0x011c_7810;
/// `"ScreenSplatterRoot"`: the name given to every splatter node.
const SPLATTER_ROOT_NAME: u32 = 0x0102_28a8;
/// Vtable of `BSCustomSplatterExtraData` (RTTI `.?AVBSCustomSplatterExtraData@@`).
const CUSTOM_SPLATTER_EXTRA_DATA_VTABLE: u32 = 0x0102_28c4;
/// The global time object whose float at `+0xc` is the frame time
/// (`0084d030` reads it).
const FRAME_TIME_OBJECT: u32 = 0x011f_6394;
/// `+0xc` getter of the frame time object, in `ST0`.
const FRAME_TIME_GET: u32 = 0x0084_d030;
/// The game setting `fBloodSplatterDuration` (value at `+4`).
const SETTING_DURATION: u32 = 0x011c_fd4c;
/// The game setting `fBloodSplatterFadeStart`.
const SETTING_FADE_START: u32 = 0x011c_fc2c;
/// The game setting `fBloodSplatterFlareOffsetScale`.
const SETTING_FLARE_OFFSET_SCALE: u32 = 0x011c_eeb4;
/// The game setting `fBloodSplatterFlareMult`.
const SETTING_FLARE_MULT: u32 = 0x011c_f6a8;
/// Setting value pointer getter (`Setting*` to the value at `+4`; null gives
/// a scratch zero).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// Float static copied from `fBloodSplatterFlareOffsetScale` each frame.
const FLARE_OFFSET_SCALE: u32 = 0x011f_eda4;
/// Float static copied from `fBloodSplatterFlareMult` each frame.
const FLARE_MULT: u32 = 0x011a_d874;
/// Float static that rises towards 1.0 by `frame time / 0.4` each frame.
const FLARE_FADE_IN: u32 = 0x011a_d878;
/// `0.0` (`double`).
const ZERO: u32 = 0x0101_2060;
/// `1.0` (`double`).
const ONE: u32 = 0x0101_2070;
/// `0.4` (`double`): the length in seconds of the flare fade in.
const FLARE_FADE_IN_SECONDS: u32 = 0x0102_28a0;

/// `NiPointer` dereference (`*this`).
const POINTER_GET: u32 = 0x0055_9450;
/// Child count of a `NiNode` (the 16-bit count of its child array).
const NODE_CHILD_COUNT: u32 = 0x0043_b480;
/// Child `index` of a `NiNode` (`this`, index).
const NODE_CHILD_AT: u32 = 0x0043_b4a0;
/// `NiObjectNET::GetExtraData(NiFixedString*)`.
const GET_EXTRA_DATA: u32 = 0x00a5_bdd0;
/// `+0xc` of the extra data: the `BSCustomSplatterData`.
const EXTRA_DATA_SPLATTER_DATA: u32 = 0x0048_d150;
/// `lerp(a, b, t0, t1, t)` as `fn_004b3ab0`.
const LERP: u32 = 0x004b_3ab0;
/// `NiUpdateData(time, bool, bool)` constructor (`this`, float, byte, byte).
const UPDATE_DATA_CONSTRUCT: u32 = 0x0043_d410;
/// `NiAVObject::Update(NiUpdateData&)` (the virtual at `+0xa4` with
/// `(data, 0)`, then the parent's at `+0xfc`).
const UPDATE: u32 = 0x00a5_9c60;
/// `min(a, b)` of two floats, in `ST0`.
const MIN: u32 = 0x0040_ebd0;
/// Creates the splatter node (`count`, two floats), returns the node.
const CREATE_SPLATTER_NODE: u32 = 0x004d_f040;
/// `NiObjectNET::AddExtraData(NiExtraData*)`.
const ADD_EXTRA_DATA: u32 = 0x00a5_bca0;
/// `NiObjectNET::SetName(NiFixedString&)`.
const SET_NAME: u32 = 0x00a5_b950;
/// `+0x14` of the extra data: the `fAlpha` of its `BSCustomSplatterData`.
const EXTRA_DATA_ALPHA: u32 = 0x007d_6bb0;
/// Hands the alpha pointer to the blood splatter shader property of every
/// geometry below the node.
const SET_ALPHA_POINTER: u32 = 0x00b8_1420;

/// Virtual slot (byte offset) of `NiNode::AttachChild(child, firstAvailable)`.
const NODE_ATTACH_CHILD_SLOT: u32 = 0xdc;
/// Virtual slot (byte offset) of `NiNode::DetachChildAt(index)`.
const NODE_DETACH_CHILD_AT_SLOT: u32 = 0xf0;

// Translated from 004de420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SettingT<INIPrefSettingCollection>` destructor body (class from the
/// vtable `01022738`'s run-time type information; the deleting destructor
/// `004de3f0` calls it): sets that vtable, removes the setting from the
/// `INIPrefSettingCollection` singleton (virtual `Remove`, vtable `+8`),
/// then runs the `Setting` destructor.
// Translation incomplete: 004de420 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn setting_t_ini_pref_setting_collection_destructor(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), SETTING_T_INI_PREF_VTABLE);
    let collection = fn_004de490(e);
    e.vcall(collection.addr(), COLLECTION_REMOVE_SLOT, &args![this]);
    e.call(SETTING_DESTRUCT, &args![this]);
}

// Translated from 004de490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes sure the `INIPrefSettingCollection` singleton exists (`004de4a0`)
/// and returns it. The method name is not confirmed.
pub fn fn_004de490(e: &mut Engine) -> Ptr {
    fn_004de4a0(e);
    Ptr::new(e.mem.u32(INI_PREF_SETTING_COLLECTION_INSTANCE))
}

// Translated from 004de4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the `INIPrefSettingCollection` singleton on first use: allocates
/// 0x114 bytes and constructs it in place (null when the allocation fails).
// Translation incomplete: 004de4a0 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn fn_004de4a0(e: &mut Engine) {
    if e.mem.u32(INI_PREF_SETTING_COLLECTION_INSTANCE) != 0 {
        return;
    }
    let block = e
        .call(OPERATOR_NEW, &args![INI_PREF_SETTING_COLLECTION_SIZE])
        .ptr::<()>();
    let instance = if block.is_null() {
        0
    } else {
        ini_pref_setting_collection_construct(e, block).addr()
    };
    e.mem
        .set_u32(INI_PREF_SETTING_COLLECTION_INSTANCE, instance);
}

// Translated from 004de530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `INIPrefSettingCollection` constructor: the base
/// (`SettingCollectionList<Setting>`) constructor, then its own vtable.
pub fn ini_pref_setting_collection_construct(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(SETTING_COLLECTION_LIST_CONSTRUCT, &args![this]);
    e.mem
        .set_u32(this.addr(), INI_PREF_SETTING_COLLECTION_VTABLE);
    this
}

// Translated from 004de550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `INIPrefSettingCollection` deleting destructor (first slot of its
/// vtable): runs the destructor body and, with bit 0 of `flags` set, frees
/// the object. Returns `this`.
pub fn ini_pref_setting_collection_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    e.call(SETTING_COLLECTION_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 004de580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates `BSCustomSplatterExtraData::pTag` (Xbox PDB static
/// `NiFixedString*`): a 4 byte block holding the fixed string `"CSD"`
/// (null when the allocation fails).
// Translation incomplete: 004de580 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn fn_004de580(e: &mut Engine) {
    let block = e.call(OPERATOR_NEW, &args![4u32]).ptr::<()>();
    let tag = if block.is_null() {
        0
    } else {
        e.call(FIXED_STRING_CONSTRUCT, &args![block, SPLATTER_TAG_TEXT])
            .u32()
    };
    e.mem.set_u32(SPLATTER_TAG, tag);
}

/// The float a setting holds (`Setting*` to its value, as `00403e20` gives).
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.f32(value)
}

/// `NiPointer<NiNode>` dereference of `ScreenCustomSplatter::spRoot`.
fn splatter_root(e: &mut Engine) -> u32 {
    e.call(POINTER_GET, &args![SPLATTER_ROOT]).u32()
}

// Translated from 004de600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Per-frame step over the splatters of `ScreenCustomSplatter::spRoot`
/// (the statics are those of the Xbox PDB `ScreenCustomSplatter`; the
/// method name is not confirmed). Does nothing while `bEnabled` is 0.
/// Otherwise, for every child that carries the `"CSD"` extra data: adds the
/// frame time to `fAge`; an age that reached `fDuration` takes the child's
/// `uiCount` off the global count and detaches the child (virtual `+0xf0`
/// with the index, which is not adjusted for the shifted children); a
/// younger one gets its `fAlpha` (`1 - lerp(0, 1, start, fDuration, age)`
/// once the fade has begun: with `bSecondsForFade` clear when
/// `age / fDuration >= fFadeTime` and `start = fBloodSplatterFadeStart *
/// fDuration`; set when `age >= fDuration - fFadeTime` and `start =
/// (fDuration - fFadeTime) / fDuration * fDuration`) and is updated with the
/// age as time. Afterwards the flare statics are refreshed from the two
/// flare settings and the fade-in static rises by `frame time / 0.4`
/// (capped at 1.0) while below 1.0.
pub fn fn_004de600(e: &mut Engine) {
    if e.mem.u8(SPLATTER_ENABLED) == 0 {
        return;
    }
    let root = splatter_root(e);
    let count = e.call(NODE_CHILD_COUNT, &args![root]).u32();
    let frame_time = e.call(FRAME_TIME_GET, &args![FRAME_TIME_OBJECT]).f32();
    let mut index = 0u32;
    while index < count {
        let root = splatter_root(e);
        let child = e.call(NODE_CHILD_AT, &args![root, index]).u32();
        if child != 0 {
            let tag = fn_004de890(e);
            let extra = e.call(GET_EXTRA_DATA, &args![child, tag]).u32();
            let data = e.call(EXTRA_DATA_SPLATTER_DATA, &args![extra]).u32();
            // BSCustomSplatterData: fAge +0, fDuration +4, fAlpha +8, uiCount +0xc.
            let age = (e.mem.f32(data) as f64 + frame_time as f64) as f32;
            e.mem.set_f32(data, age);
            let duration = e.mem.f32(data + 4);
            if duration <= age {
                let total = e
                    .mem
                    .u32(SPLATTER_COUNT)
                    .wrapping_sub(e.mem.u32(data + 0xc));
                e.mem.set_u32(SPLATTER_COUNT, total);
                let root = splatter_root(e);
                e.vcall(root, NODE_DETACH_CHILD_AT_SLOT, &args![index]);
            } else {
                let fade_time = e.mem.f32(SPLATTER_FADE_TIME);
                if e.mem.u8(SPLATTER_SECONDS_FOR_FADE) == 0 {
                    if fade_time as f64 <= age as f64 / duration as f64 {
                        let start =
                            (setting_float(e, SETTING_FADE_START) as f64 * duration as f64) as f32;
                        let lerp = e
                            .call(LERP, &args![0.0f32, 1.0f32, start, duration, age])
                            .f64();
                        e.mem.set_f32(data + 8, (1.0 - lerp) as f32);
                    }
                } else if duration as f64 - fade_time as f64 <= age as f64 {
                    let fraction = ((duration as f64 - fade_time as f64) / duration as f64) as f32;
                    let start = (fraction as f64 * duration as f64) as f32;
                    let lerp = e
                        .call(LERP, &args![0.0f32, 1.0f32, start, duration, age])
                        .f64();
                    e.mem.set_f32(data + 8, (1.0 - lerp) as f32);
                }
                e.with_stack(0x10, |e, update_data| {
                    e.call(UPDATE_DATA_CONSTRUCT, &args![update_data, age, 1u32, 0u32]);
                    e.call(UPDATE, &args![child, update_data]);
                });
            }
        }
        index += 1;
    }
    let offset_scale = setting_float(e, SETTING_FLARE_OFFSET_SCALE);
    fn_004de8a0(e, offset_scale);
    let flare_mult = setting_float(e, SETTING_FLARE_MULT);
    fn_004de8b0(e, flare_mult);
    let fade_in = fn_004de8d0(e);
    if (fade_in as f64) < e.mem.f64(ONE) {
        let raised = (frame_time as f64 / e.mem.f64(FLARE_FADE_IN_SECONDS) + fade_in as f64) as f32;
        let capped = e.call(MIN, &args![1.0f32, raised]).f32();
        fn_004de8c0(e, capped);
    }
}

// Translated from 004de890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCustomSplatterExtraData::pTag` (Xbox PDB static `NiFixedString*`),
/// as created by `004de580`.
pub fn fn_004de890(e: &mut Engine) -> Ptr {
    Ptr::new(e.mem.u32(SPLATTER_TAG))
}

// Translated from 004de8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the float at `011feda4` (the value `fn_004de600` copies from
/// `fBloodSplatterFlareOffsetScale`).
pub fn fn_004de8a0(e: &mut Engine, value: f32) {
    e.mem.set_f32(FLARE_OFFSET_SCALE, value);
}

// Translated from 004de8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the float at `011ad874` (the value `fn_004de600` copies from
/// `fBloodSplatterFlareMult`).
pub fn fn_004de8b0(e: &mut Engine, value: f32) {
    e.mem.set_f32(FLARE_MULT, value);
}

// Translated from 004de8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the float at `011ad878` (the fade-in value `fn_004de600` raises).
pub fn fn_004de8c0(e: &mut Engine, value: f32) {
    e.mem.set_f32(FLARE_FADE_IN, value);
}

// Translated from 004de8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The float at `011ad878`, in `ST0`.
pub fn fn_004de8d0(e: &mut Engine) -> f32 {
    e.mem.f32(FLARE_FADE_IN)
}

// Translated from 004de8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a splatter (`ScreenCustomSplatter` statics, method name not
/// confirmed; cdecl, five words). Does nothing while `bEnabled` is 0 or
/// `count` is 0. Creates the splatter node (`004df040(count, argument_4,
/// argument_5)`), adds `count` to the global count, gives the node a
/// `BSCustomSplatterExtraData` (age 0, the duration, alpha 1, `count`; the
/// duration is `duration` when it is above 0, else `fBloodSplatterDuration`)
/// and the name `"ScreenSplatterRoot"`, attaches it to `spRoot`, hands the
/// alpha's address to the shader properties below it and updates `spRoot`
/// with time 0.
// Translation incomplete: 004de8e0 - C++ exception unwinding frame (FS:[0]) not translated.
pub fn fn_004de8e0(
    e: &mut Engine,
    count: u32,
    duration: f32,
    _unused_3: u32,
    argument_4: f32,
    argument_5: f32,
) {
    if e.mem.u8(SPLATTER_ENABLED) == 0 || count == 0 {
        return;
    }
    let node = e
        .call(CREATE_SPLATTER_NODE, &args![count, argument_4, argument_5])
        .u32();
    let total = e.mem.u32(SPLATTER_COUNT).wrapping_add(count);
    e.mem.set_u32(SPLATTER_COUNT, total);
    let duration = if duration as f64 > e.mem.f64(ZERO) {
        duration
    } else {
        setting_float(e, SETTING_DURATION)
    };
    let extra = e.with_stack(0x10, |e, data| {
        e.mem.set_f32(data.addr(), 0.0);
        e.mem.set_f32(data.addr() + 4, duration);
        e.mem.set_f32(data.addr() + 8, 1.0);
        e.mem.set_u32(data.addr() + 0xc, count);
        let block = e.call(ALLOCATE, &args![0x1cu32]).ptr::<()>();
        if block.is_null() {
            0
        } else {
            fn_004dea80(e, block, data).addr()
        }
    });
    e.call(ADD_EXTRA_DATA, &args![node, extra]);
    e.with_stack(4, |e, name| {
        e.call(FIXED_STRING_CONSTRUCT, &args![name, SPLATTER_ROOT_NAME]);
        e.call(SET_NAME, &args![node, name]);
        e.call(FIXED_STRING_DESTRUCT, &args![name]);
    });
    let root = splatter_root(e);
    e.vcall(root, NODE_ATTACH_CHILD_SLOT, &args![node, 1u32]);
    let alpha = e.call(EXTRA_DATA_ALPHA, &args![extra]).u32();
    e.call(SET_ALPHA_POINTER, &args![node, alpha]);
    e.with_stack(0x10, |e, update_data| {
        e.call(
            UPDATE_DATA_CONSTRUCT,
            &args![update_data, 0.0f32, 0u32, 0u32],
        );
        let root = splatter_root(e);
        e.call(UPDATE, &args![root, update_data]);
    });
}

// Translated from 004dea80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSCustomSplatterExtraData` constructor (class from the vtable
/// `010228c4`'s run-time type information): `NiExtraData` constructor with
/// the tag as name, its own vtable, and a copy of the 16 bytes of
/// `BSCustomSplatterData` (`fAge`, `fDuration`, `fAlpha`, `uiCount`) at
/// `+0xc`. Returns `this`.
pub fn fn_004dea80(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    let tag = fn_004de890(e);
    e.call(EXTRA_DATA_CONSTRUCT, &args![this, tag]);
    e.mem
        .set_u32(this.addr(), CUSTOM_SPLATTER_EXTRA_DATA_VTABLE);
    for word in 0..4 {
        let value = e.mem.u32(data.addr() + 4 * word);
        e.mem.set_u32(this.addr() + 0xc + 4 * word, value);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004de420,
            setting_t_ini_pref_setting_collection_destructor(Ptr)
        ),
        entry!(0x004de490, fn_004de490() -> Ptr),
        entry!(0x004de4a0, fn_004de4a0()),
        entry!(0x004de530, ini_pref_setting_collection_construct(Ptr) -> Ptr),
        entry!(
            0x004de550,
            ini_pref_setting_collection_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x004de580, fn_004de580()),
        entry!(0x004de600, fn_004de600()),
        entry!(0x004de890, fn_004de890() -> Ptr),
        entry!(0x004de8a0, fn_004de8a0(f32)),
        entry!(0x004de8b0, fn_004de8b0(f32)),
        entry!(0x004de8c0, fn_004de8c0(f32)),
        entry!(0x004de8d0, fn_004de8d0() -> f32),
        entry!(0x004de8e0, fn_004de8e0(u32, f32, u32, f32, f32)),
        entry!(0x004dea80, fn_004dea80(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Where the doubles keep their values: the root node, the children, ...
    const NODE_VTABLE: u32 = 0x0300_0000;
    const NODE_DOUBLE_DETACH: u32 = 0x0070_0000;
    const NODE_DOUBLE_ATTACH: u32 = 0x0070_0010;

    /// Maps the statics the functions touch and the constants they read.
    fn prepared() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_7000, 0x1000);
        e.map(0x011f_3000, 0x1000);
        e.map(0x011f_e000, 0x1000);
        e.map(0x011a_d000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0102_2000, 0x1000);
        e.mem.set_f64(ZERO, 0.0);
        e.mem.set_f64(ONE, 1.0);
        e.mem.set_f64(FLARE_FADE_IN_SECONDS, 0.4);
        e
    }

    /// A double that returns the pointer of a 4 byte block holding `value`
    /// for the setting at `setting`.
    fn setting_doubles(e: &mut Engine, values: &[(u32, f32)]) {
        let table: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
        for (setting, value) in values {
            let block = e.mem.alloc(8);
            e.mem.set_f32(block, *value);
            table.borrow_mut().push((*setting, block));
        }
        e.register_double(SETTING_VALUE, move |_, a| {
            let block = table
                .borrow()
                .iter()
                .find(|(setting, _)| *setting == a[0])
                .map(|(_, block)| *block)
                .expect("unexpected setting");
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
    }

    #[test]
    fn destructor_removes_the_setting_from_the_collection() {
        let mut e = prepared();
        let collection = e.mem.alloc(0x20);
        e.put_vtable(0x0300_1000, &[0, 0, 0x0070_0020]);
        e.mem.set_u32(collection, 0x0300_1000);
        e.mem
            .set_u32(INI_PREF_SETTING_COLLECTION_INSTANCE, collection);
        e.register(0x0070_0020, |_, _| Ret::default());
        e.register(SETTING_DESTRUCT, |_, _| Ret::default());
        let setting = Ptr::<()>::new(e.mem.alloc(0x10));
        e.call_log = Some(vec![]);
        e.call(0x004d_e420, &args![setting]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(setting.addr()), SETTING_T_INI_PREF_VTABLE);
        assert_eq!(log[1], (0x0070_0020, vec![collection, setting.addr()]));
        assert_eq!(log[2], (SETTING_DESTRUCT, vec![setting.addr()]));
    }

    #[test]
    fn instance_is_created_once() {
        let mut e = prepared();
        let block = e.mem.alloc(INI_PREF_SETTING_COLLECTION_SIZE);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a, [INI_PREF_SETTING_COLLECTION_SIZE]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(SETTING_COLLECTION_LIST_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        assert_eq!(e.call(0x004d_e490, &[]).u32(), block);
        assert_eq!(e.mem.u32(block), INI_PREF_SETTING_COLLECTION_VTABLE);
        // The second call finds the instance and allocates nothing.
        e.register(OPERATOR_NEW, |_, _| panic!("allocated twice"));
        assert_eq!(e.call(0x004d_e4a0, &[]).u32(), 0);
        assert_eq!(e.call(0x004d_e490, &[]).u32(), block);
    }

    #[test]
    fn instance_stays_null_when_the_allocation_fails() {
        let mut e = prepared();
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.register(SETTING_COLLECTION_LIST_CONSTRUCT, |_, _| {
            panic!("constructed without memory")
        });
        e.call(0x004d_e4a0, &[]);
        assert_eq!(e.mem.u32(INI_PREF_SETTING_COLLECTION_INSTANCE), 0);
    }

    #[test]
    fn constructor_runs_the_base_then_sets_the_vtable() {
        let mut e = prepared();
        e.register(SETTING_COLLECTION_LIST_CONSTRUCT, |_, _| Ret::default());
        let object = Ptr::<()>::new(e.mem.alloc(INI_PREF_SETTING_COLLECTION_SIZE));
        assert_eq!(e.call(0x004d_e530, &args![object]).u32(), object.addr());
        assert_eq!(e.mem.u32(object.addr()), INI_PREF_SETTING_COLLECTION_VTABLE);
    }

    #[test]
    fn deleting_destructor_frees_only_with_the_flag() {
        let mut e = prepared();
        e.register(SETTING_COLLECTION_DESTRUCT, |_, _| Ret::default());
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        let object = Ptr::<()>::new(e.mem.alloc(0x20));
        e.call_log = Some(vec![]);
        e.call(0x004d_e550, &args![object, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x004d_e550, &args![object, 1u32]).u32(),
            object.addr()
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(log[2], (OPERATOR_DELETE, vec![object.addr()]));
    }

    #[test]
    fn tag_holds_the_fixed_string_csd() {
        let mut e = prepared();
        let block = e.mem.alloc(4);
        e.register_double(OPERATOR_NEW, move |_, a| {
            assert_eq!(a, [4]);
            Ret {
                eax: block,
                ..Ret::default()
            }
        });
        e.register(FIXED_STRING_CONSTRUCT, |_, a| {
            assert_eq!(a[1], SPLATTER_TAG_TEXT);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.call(0x004d_e580, &[]);
        assert_eq!(e.mem.u32(SPLATTER_TAG), block);
        assert_eq!(e.call(0x004d_e890, &[]).u32(), block);
    }

    #[test]
    fn tag_is_null_when_the_allocation_fails() {
        let mut e = prepared();
        e.mem.set_u32(SPLATTER_TAG, 0x1234);
        e.register(OPERATOR_NEW, |_, _| Ret::default());
        e.call(0x004d_e580, &[]);
        assert_eq!(e.mem.u32(SPLATTER_TAG), 0);
    }

    #[test]
    fn float_statics_are_stored_and_read() {
        let mut e = prepared();
        e.call(0x004d_e8a0, &args![1.5f32]);
        e.call(0x004d_e8b0, &args![2.5f32]);
        e.call(0x004d_e8c0, &args![0.25f32]);
        assert_eq!(e.mem.f32(FLARE_OFFSET_SCALE), 1.5);
        assert_eq!(e.mem.f32(FLARE_MULT), 2.5);
        assert_eq!(e.mem.f32(FLARE_FADE_IN), 0.25);
        assert_eq!(e.call(0x004d_e8d0, &[]).f32(), 0.25);
    }

    #[test]
    fn extra_data_constructor_copies_the_data() {
        let mut e = prepared();
        e.mem.set_u32(SPLATTER_TAG, 0x0abc);
        e.register_double(EXTRA_DATA_CONSTRUCT, |_, a| {
            assert_eq!(a[1], 0x0abc);
            Ret::default()
        });
        let this = Ptr::<()>::new(e.mem.alloc(0x1c));
        let data = Ptr::<()>::new(e.mem.alloc(0x10));
        for (i, w) in [1.0f32, 2.0, 3.0].iter().enumerate() {
            e.mem.set_f32(data.addr() + 4 * i as u32, *w);
        }
        e.mem.set_u32(data.addr() + 12, 9);
        assert_eq!(e.call(0x004d_ea80, &args![this, data]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), CUSTOM_SPLATTER_EXTRA_DATA_VTABLE);
        assert_eq!(e.mem.f32(this.addr() + 0xc), 1.0);
        assert_eq!(e.mem.f32(this.addr() + 0x10), 2.0);
        assert_eq!(e.mem.f32(this.addr() + 0x14), 3.0);
        assert_eq!(e.mem.u32(this.addr() + 0x18), 9);
    }

    /// Everything the add function touches, with the calls it makes logged.
    struct AddRig {
        node: u32,
        root: u32,
        extra: u32,
    }

    fn add_rig(e: &mut Engine, node_block_fails: bool) -> AddRig {
        let node = e.mem.alloc(0x20);
        let root = e.mem.alloc(0x20);
        let extra = e.mem.alloc(0x20);
        e.put_vtable(NODE_VTABLE, &[NODE_DOUBLE_ATTACH; 0xdc / 4 + 1]);
        e.mem.set_u32(root, NODE_VTABLE);
        e.mem.set_u32(SPLATTER_ROOT, root);
        e.mem.set_u32(SPLATTER_TAG, 0x0abc);
        e.mem.set_u8(SPLATTER_ENABLED, 1);
        setting_doubles(e, &[(SETTING_DURATION, 5.0)]);
        e.register_double(CREATE_SPLATTER_NODE, move |_, _| Ret {
            eax: node,
            ..Ret::default()
        });
        e.register(POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register_double(ALLOCATE, move |_, _| Ret {
            eax: if node_block_fails { 0 } else { extra },
            ..Ret::default()
        });
        e.register(EXTRA_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(ADD_EXTRA_DATA, |_, _| Ret::default());
        e.register(FIXED_STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(SET_NAME, |_, _| Ret::default());
        e.register(FIXED_STRING_DESTRUCT, |_, _| Ret::default());
        e.register(NODE_DOUBLE_ATTACH, |_, _| Ret::default());
        e.register(EXTRA_DATA_ALPHA, |_, a| Ret {
            eax: a[0] + 0x14,
            ..Ret::default()
        });
        e.register(SET_ALPHA_POINTER, |_, _| Ret::default());
        e.register(UPDATE_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(UPDATE, |_, _| Ret::default());
        AddRig { node, root, extra }
    }

    #[test]
    fn add_builds_the_node_with_its_data() {
        let mut e = prepared();
        let rig = add_rig(&mut e, false);
        e.mem.set_u32(SPLATTER_COUNT, 10);
        e.call_log = Some(vec![]);
        e.call(0x004d_e8e0, &args![3u32, 2.5f32, 0u32, 0.5f32, 0.75f32]);
        let log = e.call_log.take().unwrap();
        let called: Vec<u32> = log.iter().map(|(addr, _)| *addr).collect();
        assert_eq!(called[1], CREATE_SPLATTER_NODE);
        assert_eq!(
            log[1].1,
            vec![3, 0.5f32.to_bits(), 0.75f32.to_bits()],
            "count and the two floats"
        );
        assert_eq!(e.mem.u32(SPLATTER_COUNT), 13);
        // The data built on the stack was copied into the extra data.
        assert_eq!(e.mem.f32(rig.extra + 0xc), 0.0);
        assert_eq!(e.mem.f32(rig.extra + 0x10), 2.5);
        assert_eq!(e.mem.f32(rig.extra + 0x14), 1.0);
        assert_eq!(e.mem.u32(rig.extra + 0x18), 3);
        assert!(log.contains(&(ADD_EXTRA_DATA, vec![rig.node, rig.extra])));
        assert!(log.contains(&(NODE_DOUBLE_ATTACH, vec![rig.root, rig.node, 1])));
        assert!(log.contains(&(SET_ALPHA_POINTER, vec![rig.node, rig.extra + 0x14])));
        let name_set = called.iter().position(|a| *a == SET_NAME).unwrap();
        assert_eq!(called[name_set - 1], FIXED_STRING_CONSTRUCT);
        assert_eq!(called[name_set + 1], FIXED_STRING_DESTRUCT);
        assert_eq!(log[name_set - 1].1[1], SPLATTER_ROOT_NAME);
        let update = log.iter().find(|(addr, _)| *addr == UPDATE).unwrap();
        assert_eq!(update.1[0], rig.root);
        let construct = log
            .iter()
            .find(|(addr, _)| *addr == UPDATE_DATA_CONSTRUCT)
            .unwrap();
        assert_eq!(construct.1[1..], [0.0f32.to_bits(), 0, 0]);
    }

    #[test]
    fn add_takes_the_setting_duration_when_none_is_given() {
        let mut e = prepared();
        let rig = add_rig(&mut e, false);
        e.call(0x004d_e8e0, &args![1u32, 0.0f32, 0u32, 0.0f32, 0.0f32]);
        assert_eq!(e.mem.f32(rig.extra + 0x10), 5.0);
        e.call(0x004d_e8e0, &args![1u32, -3.0f32, 0u32, 0.0f32, 0.0f32]);
        assert_eq!(e.mem.f32(rig.extra + 0x10), 5.0);
    }

    #[test]
    fn add_does_nothing_when_disabled_or_empty() {
        let mut e = prepared();
        add_rig(&mut e, false);
        e.register(CREATE_SPLATTER_NODE, |_, _| panic!("created"));
        e.call(0x004d_e8e0, &args![0u32, 1.0f32, 0u32, 0.0f32, 0.0f32]);
        e.mem.set_u8(SPLATTER_ENABLED, 0);
        e.call(0x004d_e8e0, &args![4u32, 1.0f32, 0u32, 0.0f32, 0.0f32]);
        assert_eq!(e.mem.u32(SPLATTER_COUNT), 0);
    }

    #[test]
    fn add_passes_a_null_extra_data_when_the_allocation_fails() {
        let mut e = prepared();
        let rig = add_rig(&mut e, true);
        e.register(EXTRA_DATA_CONSTRUCT, |_, _| panic!("constructed"));
        e.call_log = Some(vec![]);
        e.call(0x004d_e8e0, &args![1u32, 1.0f32, 0u32, 0.0f32, 0.0f32]);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(ADD_EXTRA_DATA, vec![rig.node, 0])));
    }

    /// A splatter scene: a root with one child that carries the extra data.
    struct UpdateRig {
        child: u32,
        data: u32,
        root: u32,
    }

    /// `ages`, `durations` and `counts` of the children.
    fn update_rig(e: &mut Engine, children: &[(f32, f32, u32)], frame_time: f32) -> Vec<UpdateRig> {
        let root = e.mem.alloc(0x20);
        e.put_vtable(NODE_VTABLE, &[NODE_DOUBLE_DETACH; 0xf0 / 4 + 1]);
        e.mem.set_u32(root, NODE_VTABLE);
        e.mem.set_u32(SPLATTER_ROOT, root);
        e.mem.set_u32(SPLATTER_TAG, 0x0abc);
        e.mem.set_u8(SPLATTER_ENABLED, 1);
        let mut rigs = vec![];
        for (age, duration, count) in children {
            let child = e.mem.alloc(0x10);
            let data = e.mem.alloc(0x10);
            e.mem.set_f32(data, *age);
            e.mem.set_f32(data + 4, *duration);
            e.mem.set_f32(data + 8, 0.5);
            e.mem.set_u32(data + 0xc, *count);
            rigs.push(UpdateRig { child, data, root });
        }
        let table: Vec<(u32, u32)> = rigs.iter().map(|r| (r.child, r.data)).collect();
        let children_list: Vec<u32> = rigs.iter().map(|r| r.child).collect();
        e.register(POINTER_GET, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        let n = children_list.len() as u32;
        e.register_double(NODE_CHILD_COUNT, move |_, _| Ret {
            eax: n,
            ..Ret::default()
        });
        e.register_double(NODE_CHILD_AT, move |_, a| Ret {
            eax: children_list.get(a[1] as usize).copied().unwrap_or(0),
            ..Ret::default()
        });
        e.register_double(GET_EXTRA_DATA, move |_, a| {
            assert_eq!(a[1], 0x0abc, "asked with the tag");
            Ret {
                eax: table.iter().find(|(c, _)| *c == a[0]).unwrap().1,
                ..Ret::default()
            }
        });
        e.register(EXTRA_DATA_SPLATTER_DATA, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register_double(FRAME_TIME_GET, move |_, _| Ret {
            st0: frame_time as f64,
            ..Ret::default()
        });
        e.register(LERP, |_, a| {
            // Real interpolation, as the game's function computes it.
            let f = |i: usize| f32::from_bits(a[i]) as f64;
            let ratio = (f(4) - f(2)) / (f(3) - f(2));
            Ret {
                st0: ratio * (f(1) - f(0)) + f(0),
                ..Ret::default()
            }
        });
        e.register(MIN, |_, a| {
            let (first, second) = (f32::from_bits(a[0]), f32::from_bits(a[1]));
            Ret {
                st0: if second <= first { second } else { first } as f64,
                ..Ret::default()
            }
        });
        e.register(UPDATE_DATA_CONSTRUCT, |_, _| Ret::default());
        e.register(UPDATE, |_, _| Ret::default());
        e.register(NODE_DOUBLE_DETACH, |_, _| Ret::default());
        rigs
    }

    fn settings(e: &mut Engine, fade_start: f32) {
        setting_doubles(
            e,
            &[
                (SETTING_FADE_START, fade_start),
                (SETTING_FLARE_OFFSET_SCALE, 0.43),
                (SETTING_FLARE_MULT, 1.5),
            ],
        );
    }

    #[test]
    fn update_does_nothing_while_disabled() {
        let mut e = prepared();
        e.register(POINTER_GET, |_, _| panic!("touched"));
        e.call(0x004d_e600, &[]);
    }

    #[test]
    fn update_ages_a_young_splatter_and_updates_it() {
        let mut e = prepared();
        let rigs = update_rig(&mut e, &[(1.0, 5.0, 3)], 0.5);
        settings(&mut e, 0.8);
        e.mem.set_f32(SPLATTER_FADE_TIME, 0.9);
        e.mem.set_u8(SPLATTER_SECONDS_FOR_FADE, 0);
        e.call_log = Some(vec![]);
        e.call(0x004d_e600, &[]);
        let log = e.call_log.take().unwrap();
        // 1.5 / 5.0 = 0.3 is below the fade time: no alpha written.
        assert_eq!(e.mem.f32(rigs[0].data), 1.5);
        assert_eq!(e.mem.f32(rigs[0].data + 8), 0.5);
        assert!(log.contains(&(UPDATE, vec![rigs[0].child, log_update_data(&log)])));
        let construct = log
            .iter()
            .find(|(addr, _)| *addr == UPDATE_DATA_CONSTRUCT)
            .unwrap();
        assert_eq!(construct.1[1..], [1.5f32.to_bits(), 1, 0]);
    }

    /// The address of the update data of the logged construction.
    fn log_update_data(log: &[(u32, Vec<u32>)]) -> u32 {
        log.iter()
            .find(|(addr, _)| *addr == UPDATE_DATA_CONSTRUCT)
            .unwrap()
            .1[0]
    }

    #[test]
    fn update_fades_by_fraction_of_the_duration() {
        let mut e = prepared();
        let rigs = update_rig(&mut e, &[(3.5, 5.0, 1)], 0.5);
        settings(&mut e, 0.5);
        e.mem.set_f32(SPLATTER_FADE_TIME, 0.75);
        e.mem.set_u8(SPLATTER_SECONDS_FOR_FADE, 0);
        e.call(0x004d_e600, &[]);
        // age 4.0, ratio 0.8 >= 0.75; start = 0.5 * 5 = 2.5;
        // lerp = (4 - 2.5) / (5 - 2.5) = 0.6; alpha = 0.4.
        assert_eq!(e.mem.f32(rigs[0].data + 8), (1.0 - 1.5f64 / 2.5) as f32);
    }

    #[test]
    fn update_fades_in_the_last_seconds() {
        let mut e = prepared();
        let rigs = update_rig(&mut e, &[(1.0, 5.0, 1), (3.0, 5.0, 1)], 0.5);
        settings(&mut e, 0.5);
        e.mem.set_f32(SPLATTER_FADE_TIME, 2.0);
        e.mem.set_u8(SPLATTER_SECONDS_FOR_FADE, 1);
        e.call(0x004d_e600, &[]);
        // First child: age 1.5 < 5 - 2: untouched alpha.
        assert_eq!(e.mem.f32(rigs[0].data + 8), 0.5);
        // Second child: age 3.5 >= 3; fraction = 3/5 = 0.6; start = 0.6 * 5
        // (rounded as the game does) ~ 3.0; lerp = (3.5 - 3) / (5 - 3).
        let fraction = ((5.0f64 - 2.0) / 5.0) as f32;
        let start = (fraction as f64 * 5.0) as f32;
        let lerp = (3.5f64 - start as f64) / (5.0 - start as f64);
        assert_eq!(e.mem.f32(rigs[1].data + 8), (1.0 - lerp) as f32);
    }

    #[test]
    fn update_detaches_a_finished_splatter() {
        let mut e = prepared();
        let rigs = update_rig(&mut e, &[(4.8, 5.0, 4)], 0.5);
        settings(&mut e, 0.5);
        e.mem.set_u32(SPLATTER_COUNT, 10);
        e.call_log = Some(vec![]);
        e.call(0x004d_e600, &[]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(SPLATTER_COUNT), 6);
        assert!(log.contains(&(NODE_DOUBLE_DETACH, vec![rigs[0].root, 0])));
        assert!(!log.iter().any(|(addr, _)| *addr == UPDATE));
    }

    #[test]
    fn update_skips_children_without_a_node() {
        let mut e = prepared();
        update_rig(&mut e, &[(1.0, 5.0, 1)], 0.5);
        settings(&mut e, 0.5);
        // A count larger than the children: the second index has no child.
        e.register(NODE_CHILD_COUNT, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.mem.set_f32(SPLATTER_FADE_TIME, 9.0);
        e.call_log = Some(vec![]);
        e.call(0x004d_e600, &[]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log.iter()
                .filter(|(addr, _)| *addr == GET_EXTRA_DATA)
                .count(),
            1
        );
    }

    #[test]
    fn update_refreshes_the_flare_statics_and_raises_the_fade_in() {
        let mut e = prepared();
        update_rig(&mut e, &[], 0.1);
        settings(&mut e, 0.5);
        e.mem.set_f32(FLARE_FADE_IN, 0.5);
        e.call(0x004d_e600, &[]);
        assert_eq!(e.mem.f32(FLARE_OFFSET_SCALE), 0.43);
        assert_eq!(e.mem.f32(FLARE_MULT), 1.5);
        // 0.1 / 0.4 + 0.5 = 0.75.
        assert_eq!(e.mem.f32(FLARE_FADE_IN), 0.75);
        // Near the top it is capped at 1.0.
        e.mem.set_f32(FLARE_FADE_IN, 0.9);
        e.call(0x004d_e600, &[]);
        assert_eq!(e.mem.f32(FLARE_FADE_IN), 1.0);
        // At 1.0 it is left alone.
        e.mem.set_f32(FLARE_FADE_IN, 1.0);
        e.register(MIN, |_, _| panic!("raised"));
        e.call(0x004d_e600, &[]);
        assert_eq!(e.mem.f32(FLARE_FADE_IN), 1.0);
    }
}
