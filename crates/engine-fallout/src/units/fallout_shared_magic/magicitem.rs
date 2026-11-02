//! `fallout shared/magic/magicitem.cpp` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `ModelLoader` singleton (pointer global).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// `MagicItem::GetAVEffect` as the decompiler names `0040a300` (`thiscall`, `RET 4`).
const MAGIC_ITEM_GET_AV_EFFECT: u32 = 0x0040_a300;
/// `0040b470` (unnamed in the engine map): `thiscall`, whether the item is loaded, in `AL`.
const MAGIC_ITEM_IS_LOADED: u32 = 0x0040_b470;
/// `0040b6b0` (unnamed in the engine map): `thiscall`, whether the item is loading, in `AL`.
const MAGIC_ITEM_IS_LOADING: u32 = 0x0040_b6b0;
/// `00444f10` (unnamed in the engine map): `thiscall` on the `ModelLoader`, `RET 0xc`.
const MODEL_LOADER_QUEUE_ITEM: u32 = 0x0044_4f10;
/// `0048cee0` (unnamed in the engine map): `thiscall`; the length of the model path of a
/// `TESModel` (zero when it has none).
const MODEL_PATH_LENGTH: u32 = 0x0048_cee0;
/// `ModelLoader::LoadFile` (`00447080`, `thiscall`, `RET 0x18`): the model path then
/// `0, 1, 0, 0, 0`.
const MODEL_LOADER_LOAD_FILE: u32 = 0x0044_7080;
/// `004088c0` (unnamed in the engine map): `thiscall`, adjusts a counter at `+0xa8`.
const FORM_COUNTER_A8: u32 = 0x0040_88c0;
/// `008256d0` (unnamed in the engine map): `thiscall` on a list node, whether the node is empty
/// (no item and no next).
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
/// `006815c0`: returns `this` (a node's address is the address of its item pointer).
const LIST_NODE_ITEM_SLOT: u32 = 0x0068_15c0;
/// `00726070`: the next-node pointer at `this + 4`.
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// `00825c00` (unnamed in the engine map): the dword at `this + 0x14` (the effect setting of an
/// effect item).
const EFFECT_ITEM_SETTING: u32 = 0x0082_5c00;
/// `004064b0` (unnamed in the engine map): `thiscall`; whether the setting qualifies (the
/// result of `00403c80` on a field of the setting).
const EFFECT_SETTING_QUALIFIES: u32 = 0x0040_64b0;
/// `005f5f80` (unnamed in the engine map): the dword at `this + 0x60` (the setting's associated
/// form).
const EFFECT_SETTING_ASSOCIATED_FORM: u32 = 0x005f_5f80;
/// `00403e60` (unnamed in the engine map): `thiscall`, `RET 4`; whether the flag mask is set on
/// the form.
const FORM_HAS_FLAG: u32 = 0x0040_3e60;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`, five stack words).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The `RTTI` type descriptor the associated form is cast from.
const RTTI_SOURCE_TYPE: u32 = 0x0118_3028;
/// The cast target for the `0x10000` flag.
const RTTI_CAST_FOR_10000: u32 = 0x0118_3998;
/// The cast target for the `0x20000` flag.
const RTTI_CAST_FOR_20000: u32 = 0x0118_3a34;
/// The first cast target for the `0x40000` flag.
const RTTI_CAST_FOR_40000_FIRST: u32 = 0x0118_3a1c;
/// The second cast target for the `0x40000` flag.
const RTTI_CAST_FOR_40000_SECOND: u32 = 0x0118_3a00;
/// `004811e0` (unnamed in the engine map): `thiscall`, `RET 4`; the model path of one of the two
/// biped models (index 0 or 1), through the model's virtual slot +0x14.
const BIPED_MODEL_BY_INDEX: u32 = 0x0048_11e0;
/// `0060af90` (unnamed in the engine map): `thiscall`, `RET 8`; the decompiler calls it
/// `TESNPC::BuildDefaultModelList(1, 1)`. Builds a list of model paths.
const BUILD_DEFAULT_MODEL_LIST: u32 = 0x0060_af90;
/// `00470470` (unnamed in the engine map): `thiscall`; empties a model-path list.
const FILENAME_LIST_CLEAR: u32 = 0x0047_0470;
/// `004702f0` (unnamed in the engine map): `thiscall`, `RET 4`; destroys a model-path list (frees
/// it when the argument's bit 0 is set).
const FILENAME_LIST_DESTROY: u32 = 0x0047_02f0;
/// `00717e50`: `this + 4`.
const MEMBER_LIST_AT_4: u32 = 0x0071_7e50;
/// `00406d30` (unnamed in the engine map): `cdecl`, (destination, size, source): copies a string.
const STRING_COPY: u32 = 0x0040_6d30;
/// `00406d50` (unnamed in the engine map): `cdecl`, (destination, size, source): appends a
/// string.
const STRING_APPEND: u32 = 0x0040_6d50;
/// The CRT `strrchr` (`00ec6e30`, `cdecl`): the last occurrence of a character.
const CRT_STRRCHR: u32 = 0x00ec_6e30;
/// `_strnicmp` (`00ec7ec0`, `cdecl`).
const CRT_STRNICMP: u32 = 0x00ec_7ec0;
/// `00447330` (unnamed in the engine map): `thiscall` on the `ModelLoader`; the decompiler calls
/// it `ModelLoader::BuildKFFileList(path, 0, 0, 0)`, which makes a list of animation files.
const MODEL_LOADER_BUILD_KF_LIST: u32 = 0x0044_7330;
/// `004471c0` (unnamed in the engine map): `thiscall` on the `ModelLoader`, `RET 4`; loads one
/// animation file by path.
const MODEL_LOADER_LOAD_KF: u32 = 0x0044_71c0;
/// `00401030` (unnamed in the engine map): `cdecl`, frees a block.
const FREE_BLOCK: u32 = 0x0040_1030;
/// `0047fd90` (unnamed in the engine map): `thiscall`; whether a `TESAnimation` has animation
/// files, in `AL`.
const ANIMATION_HAS_KF_FILES: u32 = 0x0047_fd90;
/// `00408880` (unnamed in the engine map): `thiscall`; whether the counter at `+0xac` is above
/// zero.
const SETTING_COUNTER_POSITIVE: u32 = 0x0040_8880;
/// `004088a0` (unnamed in the engine map): `thiscall`; whether the counter at `+0xac` is below
/// zero.
const SETTING_COUNTER_NEGATIVE: u32 = 0x0040_88a0;
/// `0040ab50` (unnamed in the engine map): no arguments; adds one to the global at `011c3438`.
const COUNT_PRELOADED: u32 = 0x0040_ab50;
/// `004086e0` (unnamed in the engine map): `thiscall`; adjusts the counter at `+0xac`.
const SETTING_COUNTER_ADJUST: u32 = 0x0040_86e0;
/// The `TESModel` virtual slot (byte offset) that answers the model path.
const SLOT_MODEL_PATH: u32 = 0x14;
/// `"Skeleton"`.
const SKELETON_TEXT: u32 = 0x0101_3458;
/// `"SpecialAnims"`.
const SPECIAL_ANIMS_TEXT: u32 = 0x0101_3448;
/// The text appended after `"SpecialAnims"`.
const SPECIAL_ANIMS_SEPARATOR_TEXT: u32 = 0x0101_3444;
/// Size of the path buffer the function keeps on its stack.
const PATH_BUFFER_SIZE: u32 = 0x104;

/// `ModelLoader::LoadFile(path, 0, 1, 0, 0, 0)` on the singleton.
fn load_model_file(e: &mut Engine, path: u32) {
    let loader = e.global::<u32>(MODEL_LOADER);
    e.call(
        MODEL_LOADER_LOAD_FILE,
        &args![loader, path, 0u32, 1u32, 0u32, 0u32, 0u32],
    );
}

/// `ModelLoader::LoadKF(path)` on the singleton.
fn load_animation_file(e: &mut Engine, path: u32) {
    let loader = e.global::<u32>(MODEL_LOADER);
    e.call(MODEL_LOADER_LOAD_KF, &args![loader, path]);
}

/// `__RTDynamicCast(form, 0, <source type>, target, 0)`.
fn dynamic_cast(e: &mut Engine, form: u32, target: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, RTTI_SOURCE_TYPE, target, 0u32],
    )
    .u32()
}

/// Loads the model path of a `TESModel` when it has one (`0048cee0` non-zero): the path comes
/// from the model's virtual slot +0x14.
fn load_model_with_path(e: &mut Engine, model: u32) {
    if e.call(MODEL_PATH_LENGTH, &args![model]).u32() != 0 {
        let path = e.vcall(model, SLOT_MODEL_PATH, &args![]).u32();
        load_model_file(e, path);
    }
}

/// The part of the loop body that depends on the setting's associated form: three arms chosen by
/// the flag masks `0x10000`, `0x20000` and `0x40000` of the setting (each tested only when there
/// is an associated form, in this order).
fn preload_associated_form(e: &mut Engine, setting: u32, associated: u32) {
    if associated != 0
        && e.call(FORM_HAS_FLAG, &args![setting, 0x0001_0000u32])
            .bool()
    {
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_10000);
        if form != 0 {
            load_model_with_path(e, form.wrapping_add(0x3c));
        }
    } else if associated != 0
        && e.call(FORM_HAS_FLAG, &args![setting, 0x0002_0000u32])
            .bool()
    {
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_20000);
        if form != 0 {
            let biped = form.wrapping_add(0x70);
            let first = e.call(BIPED_MODEL_BY_INDEX, &args![biped, 0u32]).u32();
            load_model_file(e, first);
            let second = e.call(BIPED_MODEL_BY_INDEX, &args![biped, 1u32]).u32();
            load_model_file(e, second);
        }
    } else if associated != 0
        && e.call(FORM_HAS_FLAG, &args![setting, 0x0004_0000u32])
            .bool()
    {
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_40000_FIRST);
        if form != 0 {
            let list = e
                .call(BUILD_DEFAULT_MODEL_LIST, &args![form, 1u32, 1u32])
                .u32();
            let mut node = list;
            while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
                let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
                let path = e.mem.u32(slot);
                load_model_file(e, path);
                node = e.call(LIST_NODE_NEXT, &args![node]).u32();
            }
            e.call(FILENAME_LIST_CLEAR, &args![list]);
            if list != 0 {
                e.call(FILENAME_LIST_DESTROY, &args![list, 1u32]);
            }
        }
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_40000_SECOND);
        if form != 0 {
            preload_model_with_animations(e, form);
        }
    }
}

/// The last arm: the form (cast to the class of `01183a00`) has a model at `+0xdc` and a
/// `TESAnimation` at `+0xc4`. The model's path is loaded (without the length test), then the
/// files named by the list at `+0x114` next to the model's folder, the `Skeleton` animation
/// files when the model's file name starts with `Skeleton`, and the `SpecialAnims` files the
/// animation lists. The path buffer is a 0x104-byte stack array in the exe.
fn preload_model_with_animations(e: &mut Engine, form: u32) {
    let model = form.wrapping_add(0xdc);
    let path_buffer = e.mem.alloc(PATH_BUFFER_SIZE);
    let first_path = e.vcall(model, SLOT_MODEL_PATH, &args![]).u32();
    load_model_file(e, first_path);
    let model_path = e.vcall(model, SLOT_MODEL_PATH, &args![]).u32();
    let list = e
        .call(MEMBER_LIST_AT_4, &args![form.wrapping_add(0x114)])
        .u32();
    e.call(
        STRING_COPY,
        &args![path_buffer, PATH_BUFFER_SIZE, model_path],
    );
    let mut folder_end = fn_0040ab30(e, Ptr::new(path_buffer), 0x5c).addr();
    let mut node = list;
    while folder_end != 0 && node != 0 {
        e.mem.set_u8(folder_end.wrapping_add(1), 0);
        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
        let name = e.mem.u32(slot);
        if name != 0 {
            e.call(STRING_APPEND, &args![path_buffer, PATH_BUFFER_SIZE, name]);
            load_model_file(e, path_buffer);
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    let last_slash = e.call(CRT_STRRCHR, &args![model_path, 0x5cu32]).u32();
    if last_slash != 0
        && e.call(
            CRT_STRNICMP,
            &args![last_slash.wrapping_add(1), SKELETON_TEXT, 8u32],
        )
        .u32()
            == 0
    {
        let loader = e.global::<u32>(MODEL_LOADER);
        let files = e
            .call(
                MODEL_LOADER_BUILD_KF_LIST,
                &args![loader, model_path, 0u32, 0u32, 0u32],
            )
            .u32();
        let mut node = files;
        while node != 0 {
            let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
            let file = e.mem.u32(slot);
            if file != 0 {
                load_animation_file(e, file);
            }
            e.call(FREE_BLOCK, &args![file]);
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
        e.call(FILENAME_LIST_CLEAR, &args![files]);
        if files != 0 {
            e.call(FILENAME_LIST_DESTROY, &args![files, 1u32]);
        }
    }
    let animation = form.wrapping_add(0xc4);
    if e.call(ANIMATION_HAS_KF_FILES, &args![animation]).bool() {
        e.mem.set_u8(folder_end.wrapping_add(1), 0);
        e.call(
            STRING_APPEND,
            &args![path_buffer, PATH_BUFFER_SIZE, SPECIAL_ANIMS_TEXT],
        );
        e.call(
            STRING_APPEND,
            &args![path_buffer, PATH_BUFFER_SIZE, SPECIAL_ANIMS_SEPARATOR_TEXT],
        );
        folder_end = fn_0040ab30(e, Ptr::new(path_buffer), 0x5c).addr();
        let mut node = e.call(MEMBER_LIST_AT_4, &args![animation]).u32();
        while node != 0 {
            e.mem.set_u8(folder_end.wrapping_add(1), 0);
            let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
            let name = e.mem.u32(slot);
            if name != 0 {
                e.call(STRING_APPEND, &args![path_buffer, PATH_BUFFER_SIZE, name]);
                load_animation_file(e, path_buffer);
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
    }
    e.mem.free(path_buffer);
}

// Translated from 0040ab30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The CRT `strrchr` as a `cdecl` wrapper: passes its two stack words (the string and the
/// character) to `00ec6e30` and answers its result, a pointer to the last occurrence or null.
pub fn fn_0040ab30(e: &mut Engine, text: Ptr, character: u32) -> Ptr {
    e.call(CRT_STRRCHR, &args![text, character]).ptr()
}

// Translated from 0040a420 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040a420 - the /GS stack-cookie check (00ec408c) at the end is not translated
/// `MagicItem::Preload` (Xbox PDB): `thiscall`, `RET 4` (the stack word is `force`, a byte).
/// Finds the item's first effect setting (`0040a300` with zero). When `force` is non-zero or the
/// item is loaded (`0040b470`), it is ready and counts as loaded; when it is only loading
/// (`0040b6b0`) it is ready but not loaded; otherwise the `ModelLoader` is only told about the
/// item (`00444f10`) and nothing else happens. A ready item with an effect loads the effect's
/// model (if the item counts as loaded and the model has a path) and counts the effect
/// (`004088c0`). Then each effect item of the list at `this + 0x10` (the walked pointer is 4
/// before the node) is looked at: when its setting qualifies (`004064b0`) and the item counts as
/// loaded, the setting's associated form is cast by the setting's flag masks and its models and
/// animation files are loaded ([`preload_associated_form`]); then, unless the setting's counter
/// at `+0xac` is positive or negative, a setting with the `0x40000` flag is counted globally
/// (`0040ab50`), and `004086e0` adjusts the counter.
pub fn magic_item_preload(e: &mut Engine, this: Ptr, force: u8) {
    let effect = e.call(MAGIC_ITEM_GET_AV_EFFECT, &args![this, 0u32]).u32();
    let (ready, loaded) = if force != 0 || e.call(MAGIC_ITEM_IS_LOADED, &args![this]).bool() {
        (true, true)
    } else if e.call(MAGIC_ITEM_IS_LOADING, &args![this]).bool() {
        (true, false)
    } else {
        (false, false)
    };
    if !ready {
        let loader = e.global::<u32>(MODEL_LOADER);
        e.call(MODEL_LOADER_QUEUE_ITEM, &args![loader, this, effect, 0u32]);
        return;
    }
    if effect != 0 {
        if loaded {
            load_model_with_path(e, effect.wrapping_add(0x18));
        }
        e.call(FORM_COUNTER_A8, &args![effect]);
    }
    let mut walked = if this.is_null() {
        0
    } else {
        this.addr().wrapping_add(0xc)
    };
    if e.call(LIST_NODE_IS_EMPTY, &args![walked.wrapping_add(4)])
        .bool()
    {
        return;
    }
    while walked != 0 {
        let node = walked.wrapping_add(4);
        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
        let effect_item = e.mem.u32(slot);
        let setting = e.call(EFFECT_ITEM_SETTING, &args![effect_item]).u32();
        if e.call(EFFECT_SETTING_QUALIFIES, &args![setting]).bool() {
            if loaded {
                let associated = e
                    .call(EFFECT_SETTING_ASSOCIATED_FORM, &args![setting])
                    .u32();
                preload_associated_form(e, setting, associated);
            }
            if !e.call(SETTING_COUNTER_POSITIVE, &args![setting]).bool()
                && !e.call(SETTING_COUNTER_NEGATIVE, &args![setting]).bool()
                && e.call(FORM_HAS_FLAG, &args![setting, 0x0004_0000u32])
                    .bool()
            {
                e.call(COUNT_PRELOADED, &args![]);
            }
            e.call(SETTING_COUNTER_ADJUST, &args![setting]);
        }
        let next = e.call(LIST_NODE_NEXT, &args![node]).u32();
        walked = if next != 0 { next.wrapping_sub(4) } else { 0 };
    }
}

/// `MagicItem`'s RTTI type descriptor (`.?AVMagicItem@@`).
const MAGIC_ITEM_TYPE: u32 = 0x0118_3140;
/// `MagicItemForm`'s RTTI type descriptor (`.?AVMagicItemForm@@`).
const MAGIC_ITEM_FORM_TYPE: u32 = 0x0118_395c;
/// `MagicItemObject`'s RTTI type descriptor (`.?AVMagicItemObject@@`).
const MAGIC_ITEM_OBJECT_TYPE: u32 = 0x0118_30e8;
/// `0084e3a0` (thiscall, returns a word; unnamed getter: the form id here, the level of
/// an effect item in `GetAVEffect`).
const GETTER_0084E3A0: u32 = 0x0084_e3a0;
/// `EffectItem::GetCost` (`00403f80`, thiscall: one word; `float` in `ST0`).
const EFFECT_ITEM_GET_COST: u32 = 0x0040_3f80;
/// `_ftol2` (`00ec62c0`): the `ST0` value as an integer (a leading `f64` argument).
const FTOL: u32 = 0x00ec_62c0;
/// `00405e40` (thiscall on an effect item, returns `AL`; unnamed): when true the item
/// is not considered.
const EFFECT_ITEM_SKIPPED: u32 = 0x0040_5e40;
/// `EffectSetting::IsEffectLoaded` (`00408a00`) and `IsEffectQueued` (`00408a20`).
const EFFECT_SETTING_IS_EFFECT_LOADED: u32 = 0x0040_8a00;
const EFFECT_SETTING_IS_EFFECT_QUEUED: u32 = 0x0040_8a20;
/// `ModelLoader::ReleaseModel` (`00445300`, thiscall on the loader: path, word, word).
const MODEL_LOADER_RELEASE_MODEL: u32 = 0x0044_5300;
/// `00445370` (thiscall on the loader: path, word; unnamed): releases an animation file.
const MODEL_LOADER_RELEASE_KF: u32 = 0x0044_5370;
/// `00408910` and `00408730` (thiscall on a setting; unnamed, `effectsetting.cpp`):
/// the counterparts of the counters `Preload` raises.
const SETTING_UNLOAD_00408910: u32 = 0x0040_8910;
const SETTING_UNLOAD_00408730: u32 = 0x0040_8730;
/// `0040bed0` (no arguments; this unit, not translated): lowers the global at `011c3438`.
const UNCOUNT_PRELOADED: u32 = 0x0040_bed0;
/// `EffectItem::InitItem` (`00403db0`, thiscall: the form).
const EFFECT_ITEM_INIT_ITEM: u32 = 0x0040_3db0;
/// `EffectItem::IsHostile` (`00404710`).
const EFFECT_ITEM_IS_HOSTILE: u32 = 0x0040_4710;

/// The effect list `MagicItem` walks: the `EffectItemList` at `this + 0xc` (null when
/// `this` is null), as the exe computes the pointer it walks (the node is 4 above it).
fn effect_list(this: Ptr) -> u32 {
    if this.is_null() {
        0
    } else {
        this.addr().wrapping_add(0xc)
    }
}

/// The next walked pointer after `node` (`00726070`, then 4 back; 0 at the end).
fn next_walked(e: &mut Engine, node: u32) -> u32 {
    let next = e.call(LIST_NODE_NEXT, &args![node]).u32();
    if next != 0 {
        next.wrapping_sub(4)
    } else {
        0
    }
}

/// The effect setting of the effect item in `node` (0 for an empty slot).
fn node_setting(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
    let item = e.mem.u32(slot);
    if item == 0 {
        0
    } else {
        e.call(EFFECT_ITEM_SETTING, &args![item]).u32()
    }
}

/// Whether the setting's effect model counts: a setting with a model path
/// (`0048cee0` of `+0x18` above 0) whose effect is not loaded.
fn effect_model_missing(e: &mut Engine, setting: u32) -> bool {
    setting != 0
        && e.call(MODEL_PATH_LENGTH, &args![setting.wrapping_add(0x18)])
            .u32()
            != 0
        && !e
            .call(EFFECT_SETTING_IS_EFFECT_LOADED, &args![setting])
            .bool()
}

/// Whether a setting's associated item counts as missing: it qualifies (`004064b0`)
/// and is not loaded (`00408880`).
fn associated_item_missing(e: &mut Engine, setting: u32) -> bool {
    setting != 0
        && e.call(EFFECT_SETTING_QUALIFIES, &args![setting]).bool()
        && !e.call(SETTING_COUNTER_POSITIVE, &args![setting]).bool()
}

/// `ModelLoader::ReleaseModel(path, unload, 1)` on the singleton.
fn release_model_file(e: &mut Engine, path: u32, unload: u8) {
    let loader = e.global::<u32>(MODEL_LOADER);
    e.call(
        MODEL_LOADER_RELEASE_MODEL,
        &args![loader, path, u32::from(unload), 1u32],
    );
}

/// `00445370(path, unload)` on the singleton.
fn release_animation_file(e: &mut Engine, path: u32, unload: u8) {
    let loader = e.global::<u32>(MODEL_LOADER);
    e.call(
        MODEL_LOADER_RELEASE_KF,
        &args![loader, path, u32::from(unload)],
    );
}

// Translated from 0040a1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicItem::GetMagicItemFormID` (Xbox PDB): the form id (`0084e3a0`) of the item as a
/// `MagicItemForm`, else as a `MagicItemObject` (`__RTDynamicCast` from `MagicItem`), else 0.
pub fn magic_item_get_magic_item_form_id(e: &mut Engine, this: Ptr) -> u32 {
    let as_form = e
        .call(
            RT_DYNAMIC_CAST,
            &args![this, 0u32, MAGIC_ITEM_TYPE, MAGIC_ITEM_FORM_TYPE, 0u32],
        )
        .u32();
    let as_object = e
        .call(
            RT_DYNAMIC_CAST,
            &args![this, 0u32, MAGIC_ITEM_TYPE, MAGIC_ITEM_OBJECT_TYPE, 0u32],
        )
        .u32();
    if as_form != 0 {
        e.call(GETTER_0084E3A0, &args![as_form]).u32()
    } else if as_object != 0 {
        e.call(GETTER_0084E3A0, &args![as_object]).u32()
    } else {
        0
    }
}

// Translated from 0040a300 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicItem::GetAVEffect` (Xbox PDB, `RET 4`): the effect setting of the item's
/// strongest effect item: among the items `00405e40` does not skip, the one with the
/// highest level (`0084e3a0`), above `minimum_level`, ties broken by the higher cost
/// (`GetCost(0)` truncated by `_ftol2`, above -1); 0 for an empty list or no such item.
pub fn magic_item_get_av_effect(e: &mut Engine, this: Ptr, minimum_level: i32) -> u32 {
    let mut best = 0u32;
    let mut walked = effect_list(this);
    if e.call(LIST_NODE_IS_EMPTY, &args![walked.wrapping_add(4)])
        .bool()
    {
        return 0;
    }
    let mut best_cost: i32 = -1;
    let mut best_level = minimum_level;
    while walked != 0 {
        let node = walked.wrapping_add(4);
        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        let cost = e.call(EFFECT_ITEM_GET_COST, &args![item, 0u32]).f64();
        let cost = e.call(FTOL, &args![cost]).i32();
        let level = e.call(GETTER_0084E3A0, &args![item]).i32();
        if !e.call(EFFECT_ITEM_SKIPPED, &args![item]).bool()
            && (level > best_level || (level == best_level && cost > best_cost))
        {
            best = item;
            best_cost = cost;
            best_level = e.call(GETTER_0084E3A0, &args![item]).i32();
        }
        walked = next_walked(e, node);
    }
    if best == 0 {
        0
    } else {
        e.call(EFFECT_ITEM_SETTING, &args![best]).u32()
    }
}

// Translated from 0040b470 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicItem::IsLoaded` (Xbox PDB): false when the strongest effect's model is not
/// loaded or any qualifying setting's associated item is not loaded; true otherwise
/// (also for an empty list).
pub fn magic_item_is_loaded(e: &mut Engine, this: Ptr) -> bool {
    let effect = magic_item_get_av_effect(e, this, 0);
    if effect_model_missing(e, effect) {
        return false;
    }
    let mut walked = effect_list(this);
    if e.call(LIST_NODE_IS_EMPTY, &args![walked.wrapping_add(4)])
        .bool()
    {
        return true;
    }
    while walked != 0 {
        let node = walked.wrapping_add(4);
        let setting = node_setting(e, node);
        if associated_item_missing(e, setting) {
            return false;
        }
        walked = next_walked(e, node);
    }
    true
}

// Translated from 0040b580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicItem::IsLoadingOrLoaded` (Xbox PDB): like `IsLoaded`, but a missing model or
/// associated item that is queued (`IsEffectQueued`, `IsAssociatedItemQueued`) counts.
pub fn magic_item_is_loading_or_loaded(e: &mut Engine, this: Ptr) -> bool {
    let effect = magic_item_get_av_effect(e, this, 0);
    if effect_model_missing(e, effect)
        && !e
            .call(EFFECT_SETTING_IS_EFFECT_QUEUED, &args![effect])
            .bool()
    {
        return false;
    }
    let mut walked = effect_list(this);
    if e.call(LIST_NODE_IS_EMPTY, &args![walked.wrapping_add(4)])
        .bool()
    {
        return true;
    }
    while walked != 0 {
        let node = walked.wrapping_add(4);
        let setting = node_setting(e, node);
        if associated_item_missing(e, setting)
            && !e.call(SETTING_COUNTER_NEGATIVE, &args![setting]).bool()
        {
            return false;
        }
        walked = next_walked(e, node);
    }
    true
}

// Translated from 0040b6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicItem::IsLoading` (Xbox PDB): true when something is missing (the strongest
/// effect's model or a qualifying setting's associated item) and everything missing is
/// queued; false when nothing is missing or something missing is not queued.
pub fn magic_item_is_loading(e: &mut Engine, this: Ptr) -> bool {
    let effect = magic_item_get_av_effect(e, this, 0);
    let mut all_loaded = true;
    if effect_model_missing(e, effect) {
        all_loaded = false;
        if !e
            .call(EFFECT_SETTING_IS_EFFECT_QUEUED, &args![effect])
            .bool()
        {
            return false;
        }
    }
    let mut walked = effect_list(this);
    if !e
        .call(LIST_NODE_IS_EMPTY, &args![walked.wrapping_add(4)])
        .bool()
    {
        while walked != 0 {
            let node = walked.wrapping_add(4);
            let setting = node_setting(e, node);
            if associated_item_missing(e, setting) {
                all_loaded = false;
                if !e.call(SETTING_COUNTER_NEGATIVE, &args![setting]).bool() {
                    return false;
                }
            }
            walked = next_walked(e, node);
        }
    }
    !all_loaded
}

/// The associated-form arms of `MagicItem::Unload`, the counterpart of
/// [`preload_associated_form`]: the same casts by the flags 0x10000, 0x20000 and 0x40000,
/// releasing what `Preload` loaded.
fn unload_associated_form(e: &mut Engine, setting: u32, associated: u32, unload: u8) {
    if associated != 0
        && e.call(FORM_HAS_FLAG, &args![setting, 0x0001_0000u32])
            .bool()
    {
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_10000);
        if form != 0
            && e.call(MODEL_PATH_LENGTH, &args![form.wrapping_add(0x3c)])
                .u32()
                != 0
        {
            let path = e
                .vcall(form.wrapping_add(0x3c), SLOT_MODEL_PATH, &args![])
                .u32();
            release_model_file(e, path, unload);
        }
    } else if associated != 0
        && e.call(FORM_HAS_FLAG, &args![setting, 0x0002_0000u32])
            .bool()
    {
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_20000);
        if form != 0 {
            let biped = form.wrapping_add(0x70);
            let first = e.call(BIPED_MODEL_BY_INDEX, &args![biped, 0u32]).u32();
            release_model_file(e, first, unload);
            let second = e.call(BIPED_MODEL_BY_INDEX, &args![biped, 1u32]).u32();
            release_model_file(e, second, unload);
        }
    } else if associated != 0
        && e.call(FORM_HAS_FLAG, &args![setting, 0x0004_0000u32])
            .bool()
    {
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_40000_FIRST);
        if form != 0 {
            let list = e
                .call(BUILD_DEFAULT_MODEL_LIST, &args![form, 1u32, 1u32])
                .u32();
            let mut node = list;
            while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
                let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
                let path = e.mem.u32(slot);
                release_model_file(e, path, unload);
                node = e.call(LIST_NODE_NEXT, &args![node]).u32();
            }
            e.call(FILENAME_LIST_CLEAR, &args![list]);
            if list != 0 {
                e.call(FILENAME_LIST_DESTROY, &args![list, 1u32]);
            }
        }
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_40000_SECOND);
        if form != 0 {
            unload_model_with_animations(e, form, unload);
        }
    }
}

/// The last arm of `MagicItem::Unload`, the counterpart of
/// [`preload_model_with_animations`]: releases the model at `+0xdc`, the files of the
/// list at `+0x114` next to it, the `Skeleton` animation files and the `SpecialAnims`
/// files of the animation at `+0xc4`. The path buffer is a 0x104-byte stack array.
fn unload_model_with_animations(e: &mut Engine, form: u32, unload: u8) {
    let model = form.wrapping_add(0xdc);
    e.with_stack(PATH_BUFFER_SIZE, |e, buffer| {
        let path_buffer = buffer.addr();
        let first_path = e.vcall(model, SLOT_MODEL_PATH, &args![]).u32();
        release_model_file(e, first_path, unload);
        let model_path = e.vcall(model, SLOT_MODEL_PATH, &args![]).u32();
        let list = e
            .call(MEMBER_LIST_AT_4, &args![form.wrapping_add(0x114)])
            .u32();
        e.call(
            STRING_COPY,
            &args![path_buffer, PATH_BUFFER_SIZE, model_path],
        );
        let mut folder_end = fn_0040ab30(e, buffer, 0x5c).addr();
        let mut node = list;
        while folder_end != 0 && node != 0 {
            e.mem.set_u8(folder_end.wrapping_add(1), 0);
            let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
            let name = e.mem.u32(slot);
            if name != 0 {
                e.call(STRING_APPEND, &args![path_buffer, PATH_BUFFER_SIZE, name]);
                release_model_file(e, path_buffer, unload);
            }
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
        }
        let last_slash = e.call(CRT_STRRCHR, &args![model_path, 0x5cu32]).u32();
        if last_slash != 0
            && e.call(
                CRT_STRNICMP,
                &args![last_slash.wrapping_add(1), SKELETON_TEXT, 8u32],
            )
            .u32()
                == 0
        {
            let loader = e.global::<u32>(MODEL_LOADER);
            let files = e
                .call(
                    MODEL_LOADER_BUILD_KF_LIST,
                    &args![loader, model_path, 0u32, 0u32, 0u32],
                )
                .u32();
            let mut node = files;
            while node != 0 {
                let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
                let file = e.mem.u32(slot);
                if file != 0 {
                    release_animation_file(e, file, unload);
                }
                e.call(FREE_BLOCK, &args![file]);
                node = e.call(LIST_NODE_NEXT, &args![node]).u32();
            }
            e.call(FILENAME_LIST_CLEAR, &args![files]);
            if files != 0 {
                e.call(FILENAME_LIST_DESTROY, &args![files, 1u32]);
            }
        }
        let animation = form.wrapping_add(0xc4);
        if e.call(ANIMATION_HAS_KF_FILES, &args![animation]).bool() {
            e.mem.set_u8(folder_end.wrapping_add(1), 0);
            e.call(
                STRING_APPEND,
                &args![path_buffer, PATH_BUFFER_SIZE, SPECIAL_ANIMS_TEXT],
            );
            e.call(
                STRING_APPEND,
                &args![path_buffer, PATH_BUFFER_SIZE, SPECIAL_ANIMS_SEPARATOR_TEXT],
            );
            folder_end = fn_0040ab30(e, buffer, 0x5c).addr();
            let mut node = e.call(MEMBER_LIST_AT_4, &args![animation]).u32();
            while node != 0 {
                e.mem.set_u8(folder_end.wrapping_add(1), 0);
                let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
                let name = e.mem.u32(slot);
                if name != 0 {
                    e.call(STRING_APPEND, &args![path_buffer, PATH_BUFFER_SIZE, name]);
                    release_animation_file(e, path_buffer, unload);
                }
                node = e.call(LIST_NODE_NEXT, &args![node]).u32();
            }
        }
    });
}

// Translated from 0040b800 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040b800 - the /GS stack-cookie check (00ec408c) at the end is not translated
/// `MagicItem::Unload` (Xbox PDB, `RET 4`; the stack word is a byte passed on to the
/// releases): the counterpart of `Preload`. Only for an item that is loading or loaded
/// (`0040b580`). Unless the item is only loading (`0040b6b0`), the strongest effect's
/// model is released; its setting is told (`00408910`). Then for each qualifying setting
/// of the effect list: unless loading, the associated form's models and animation files
/// are released (by the flags 0x10000, 0x20000, 0x40000, as `Preload` loads them); the
/// setting is told (`00408730`); and a 0x40000 setting whose counter is neither positive
/// nor negative lowers the global count (`0040bed0`).
pub fn magic_item_unload(e: &mut Engine, this: Ptr, unload: u8) {
    if !magic_item_is_loading_or_loaded(e, this) {
        return;
    }
    let effect = magic_item_get_av_effect(e, this, 0);
    let loading = magic_item_is_loading(e, this);
    if effect != 0
        && e.call(MODEL_PATH_LENGTH, &args![effect.wrapping_add(0x18)])
            .u32()
            != 0
    {
        if !loading {
            let path = e
                .vcall(effect.wrapping_add(0x18), SLOT_MODEL_PATH, &args![])
                .u32();
            release_model_file(e, path, unload);
        }
        e.call(SETTING_UNLOAD_00408910, &args![effect]);
    }
    let mut walked = effect_list(this);
    if e.call(LIST_NODE_IS_EMPTY, &args![walked.wrapping_add(4)])
        .bool()
    {
        return;
    }
    while walked != 0 {
        let node = walked.wrapping_add(4);
        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
        let effect_item = e.mem.u32(slot);
        let setting = e.call(EFFECT_ITEM_SETTING, &args![effect_item]).u32();
        if e.call(EFFECT_SETTING_QUALIFIES, &args![setting]).bool() {
            if !loading {
                let associated = e
                    .call(EFFECT_SETTING_ASSOCIATED_FORM, &args![setting])
                    .u32();
                unload_associated_form(e, setting, associated, unload);
            }
            e.call(SETTING_UNLOAD_00408730, &args![setting]);
            if !e.call(SETTING_COUNTER_POSITIVE, &args![setting]).bool()
                && !e.call(SETTING_COUNTER_NEGATIVE, &args![setting]).bool()
                && e.call(FORM_HAS_FLAG, &args![setting, 0x0004_0000u32])
                    .bool()
            {
                e.call(UNCOUNT_PRELOADED, &args![]);
            }
        }
        walked = next_walked(e, node);
    }
}

// Translated from 0040c030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicItem::MagicItemInitItem` (Xbox PDB, `RET 4`): initializes each effect item of
/// the list with `form` (`EffectItem::InitItem`) and counts the hostile ones
/// (`EffectItem::IsHostile`) into the word at `this + 0x18`.
pub fn magic_item_magic_item_init_item(e: &mut Engine, this: Ptr, form: u32) {
    let mut walked = effect_list(this);
    e.mem.set_u32(this.addr().wrapping_add(0x18), 0);
    while walked != 0 {
        let node = walked.wrapping_add(4);
        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
        let item = e.mem.u32(slot);
        if item != 0 {
            e.call(EFFECT_ITEM_INIT_ITEM, &args![item, form]);
            if e.call(EFFECT_ITEM_IS_HOSTILE, &args![item]).bool() {
                let count = e.mem.u32(this.addr().wrapping_add(0x18));
                e.mem
                    .set_u32(this.addr().wrapping_add(0x18), count.wrapping_add(1));
            }
        }
        walked = next_walked(e, node);
    }
}

/// Cdecl (form id): the form with that id, or 0 (`tesform.cpp`).
const LOOK_UP_FORM_004839C0: u32 = 0x0048_39c0;
/// The counter `0040ab50` raises and `0040bed0` lowers.
const PRELOADED_COUNT: u32 = 0x011c_3438;
/// Thiscall on a setting: the word at +4 (its value; `effectitem.cpp`,
/// null for a null setting).
const SETTING_VALUE_00403DF0: u32 = 0x0040_3df0;
/// Thiscall on a string (the text), `RET 4`: constructs it from the text
/// (`run_0040c0e0.rs`).
const STRING_CONSTRUCT_0040C0E0: u32 = 0x0040_c0e0;
/// The settings whose text `0040bef0` copies, by type: 1, 2, 3, 4, 5, 6 and
/// any other.
const TYPE_TEXT_SETTING_1: u32 = 0x011c_34c0;
const TYPE_TEXT_SETTING_2: u32 = 0x011c_344c;
const TYPE_TEXT_SETTING_3: u32 = 0x011c_348c;
const TYPE_TEXT_SETTING_4: u32 = 0x011c_34a0;
const TYPE_TEXT_SETTING_5: u32 = 0x011c_3458;
const TYPE_TEXT_SETTING_6: u32 = 0x011c_34d8;
const TYPE_TEXT_SETTING_OTHER: u32 = 0x011c_3474;

// Translated from 0040a250 (decompiled, FalloutNV.exe 1.4.0.525)
/// `MagicItem::GetMagicItemByNumericID` (engine map; cdecl): the form with
/// id `form_id` (`004839c0`) as a `MagicItem`: its `MagicItemForm`'s
/// `MagicItem` base (+0x18) when it is one, else its `MagicItemObject`'s
/// (+0x30) (`__RTDynamicCast` from `TESForm`, both casts made first), else
/// 0; 0 without such a form.
pub fn magic_item_get_magic_item_by_numeric_id(e: &mut Engine, form_id: u32) -> u32 {
    let form = e.call(LOOK_UP_FORM_004839C0, &args![form_id]).u32();
    if form == 0 {
        return 0;
    }
    let as_form = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, RTTI_SOURCE_TYPE, MAGIC_ITEM_FORM_TYPE, 0u32],
        )
        .u32();
    let as_object = e
        .call(
            RT_DYNAMIC_CAST,
            &args![form, 0u32, RTTI_SOURCE_TYPE, MAGIC_ITEM_OBJECT_TYPE, 0u32],
        )
        .u32();
    if as_form != 0 {
        as_form.wrapping_add(0x18)
    } else if as_object != 0 {
        as_object.wrapping_add(0x30)
    } else {
        0
    }
}

// Translated from 0040ab50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): adds one to the counter at
/// `011c3438`.
pub fn fn_0040ab50(e: &mut Engine) {
    let count = e.mem.u32(PRELOADED_COUNT);
    e.mem.set_u32(PRELOADED_COUNT, count.wrapping_add(1));
}

// Translated from 0040bed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no arguments): takes one from the counter at
/// `011c3438`.
pub fn fn_0040bed0(e: &mut Engine) {
    let count = e.mem.u32(PRELOADED_COUNT);
    e.mem.set_u32(PRELOADED_COUNT, count.wrapping_sub(1));
}

// Translated from 0040bef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`; `this` is not read):
/// constructs the string `result` (`0040c0e0`) from the text of the setting
/// for `kind` (`00403df0`): `011c34c0` for 1, `011c344c` for 2, `011c348c`
/// for 3, `011c34a0` for 4, `011c3458` for 5, `011c34d8` for 6 (the
/// switch's jump table), `011c3474` for any other. Returns `result`.
pub fn fn_0040bef0(e: &mut Engine, _this: Ptr, result: u32, kind: u32) -> u32 {
    let setting = match kind.wrapping_sub(1) {
        0 => TYPE_TEXT_SETTING_1,
        1 => TYPE_TEXT_SETTING_2,
        2 => TYPE_TEXT_SETTING_3,
        3 => TYPE_TEXT_SETTING_4,
        4 => TYPE_TEXT_SETTING_5,
        5 => TYPE_TEXT_SETTING_6,
        _ => TYPE_TEXT_SETTING_OTHER,
    };
    let text = e.call(SETTING_VALUE_00403DF0, &args![setting]).u32();
    e.call(STRING_CONSTRUCT_0040C0E0, &args![result, text]);
    result
}

/// `EffectSetting::IsEffectQueued` (`00408a20`) and
/// `EffectSetting::IsAssociatedItemQueued` (`004088a0`) (engine map), thiscall: `AL`.
const EFFECT_SETTING_IS_ASSOCIATED_ITEM_QUEUED: u32 = 0x0040_88a0;
/// Thiscall on a setting (`effectsetting.cpp`): an integer from the counter at +0xa8
/// (`004089c0`) and at +0xac (`004087e0`), the number of references to add.
const SETTING_EFFECT_COUNT_004089C0: u32 = 0x0040_89c0;
const SETTING_ASSOCIATED_COUNT_004087E0: u32 = 0x0040_87e0;
/// The model pointer the code keeps on its stack: set up by `00633c90` (thiscall, the
/// pointer, `RET 4`), read by `00559450` (thiscall: the word at `this`) and handed to
/// `0040c110` (`run_0040c0e0.rs`) when it goes out of scope.
const MODEL_POINTER_CONSTRUCT: u32 = 0x0063_3c90;
const WORD_AT_THIS_00559450: u32 = 0x0055_9450;
const MODEL_POINTER_DESTRUCT: u32 = 0x0040_c110;
/// `ModelLoader::FindModel` (engine map), thiscall on the loader (path, address of the
/// pointer), `RET 8`.
const MODEL_LOADER_FIND_MODEL: u32 = 0x0044_72a0;
/// `Model::ModManualRefCount` (engine map), thiscall on the model (count), `RET 4`.
const MODEL_MOD_MANUAL_REF_COUNT: u32 = 0x0043_b410;
/// Thiscall on a model (`0092c870`): called once per reference before the animation
/// files are counted.
const MODEL_0092C870: u32 = 0x0092_c870;

/// The model pointer `FindModel` filled in `pointer`, given `count` more manual
/// references when there is one (`00559450` read twice, then `ModManualRefCount`).
fn add_model_references(e: &mut Engine, pointer: u32, count: i32) {
    if e.call(WORD_AT_THIS_00559450, &args![pointer]).u32() != 0 {
        let model = e.call(WORD_AT_THIS_00559450, &args![pointer]).u32();
        e.call(MODEL_MOD_MANUAL_REF_COUNT, &args![model, count]);
    }
}

/// `ModelLoader::FindModel(path, &pointer)` on the singleton.
fn find_model(e: &mut Engine, path: u32, pointer: u32) {
    let loader = e.global::<u32>(MODEL_LOADER);
    e.call(MODEL_LOADER_FIND_MODEL, &args![loader, path, pointer]);
}

/// A `NiPointer<Model>` on the stack for `path`: constructed with null, filled by
/// `FindModel`, the model given `count` references, then destroyed.
fn reference_model(e: &mut Engine, path: u32, count: i32) {
    e.with_stack(4, |e, pointer| {
        e.call(MODEL_POINTER_CONSTRUCT, &args![pointer, 0u32]);
        find_model(e, path, pointer.addr());
        add_model_references(e, pointer.addr(), count);
        e.call(MODEL_POINTER_DESTRUCT, &args![pointer]);
    });
}

/// `FinishedLoading`'s arm for a setting with flag 0x40000 whose associated form casts to
/// the class of `01183a00`: the model at +0xdc is found and, when there is one, given its
/// references (`0092c870` once per reference on it), then the models named by the list
/// at +0x114 next to the model's folder, the `Skeleton` animation files when the model's
/// file name starts with `Skeleton`, and the `SpecialAnims` files of the `TESAnimation` at
/// +0xc4 are loaded `count` times each. The 0x104-byte path buffer is on the game's stack.
fn finish_model_with_animations(e: &mut Engine, form: u32, count: i32) {
    e.with_stack(4, |e, pointer| {
        e.call(MODEL_POINTER_CONSTRUCT, &args![pointer, 0u32]);
        let model_field = form.wrapping_add(0xdc);
        let path = e.vcall(model_field, SLOT_MODEL_PATH, &args![]).u32();
        find_model(e, path, pointer.addr());
        if e.call(WORD_AT_THIS_00559450, &args![pointer]).u32() != 0 {
            let mut i = 0i32;
            while i < count {
                let model = e.call(WORD_AT_THIS_00559450, &args![pointer]).u32();
                e.call(MODEL_0092C870, &args![model]);
                i += 1;
            }
            let model = e.call(WORD_AT_THIS_00559450, &args![pointer]).u32();
            let model_path = e.call(WORD_AT_THIS_00559450, &args![model]).u32();
            let names = e
                .call(MEMBER_LIST_AT_4, &args![form.wrapping_add(0x114)])
                .u32();
            e.with_stack(PATH_BUFFER_SIZE, |e, buffer| {
                let buffer = buffer.addr();
                e.call(STRING_COPY, &args![buffer, PATH_BUFFER_SIZE, model_path]);
                let mut folder_end = fn_0040ab30(e, Ptr::new(buffer), 0x5c).addr();
                let mut node = names;
                while folder_end != 0 && node != 0 {
                    e.mem.set_u8(folder_end.wrapping_add(1), 0);
                    let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
                    let name = e.mem.u32(slot);
                    if name != 0 {
                        e.call(STRING_APPEND, &args![buffer, PATH_BUFFER_SIZE, name]);
                        reference_model(e, buffer, count);
                    }
                    node = e.call(LIST_NODE_NEXT, &args![node]).u32();
                }
                let last_slash = e.call(CRT_STRRCHR, &args![model_path, 0x5cu32]).u32();
                if last_slash != 0
                    && e.call(
                        CRT_STRNICMP,
                        &args![last_slash.wrapping_add(1), SKELETON_TEXT, 8u32],
                    )
                    .u32()
                        == 0
                {
                    let loader = e.global::<u32>(MODEL_LOADER);
                    let files = e
                        .call(
                            MODEL_LOADER_BUILD_KF_LIST,
                            &args![loader, model_path, 0u32, 0u32, 0u32],
                        )
                        .u32();
                    let mut node = files;
                    while node != 0 {
                        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
                        let file = e.mem.u32(slot);
                        if file != 0 {
                            let mut i = 0i32;
                            while i < count {
                                load_animation_file(e, file);
                                i += 1;
                            }
                        }
                        e.call(FREE_BLOCK, &args![file]);
                        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
                    }
                    e.call(FILENAME_LIST_CLEAR, &args![files]);
                    if files != 0 {
                        e.call(FILENAME_LIST_DESTROY, &args![files, 1u32]);
                    }
                }
                let animation = form.wrapping_add(0xc4);
                if e.call(ANIMATION_HAS_KF_FILES, &args![animation]).bool() {
                    e.mem.set_u8(folder_end.wrapping_add(1), 0);
                    e.call(
                        STRING_APPEND,
                        &args![buffer, PATH_BUFFER_SIZE, SPECIAL_ANIMS_TEXT],
                    );
                    e.call(
                        STRING_APPEND,
                        &args![buffer, PATH_BUFFER_SIZE, SPECIAL_ANIMS_SEPARATOR_TEXT],
                    );
                    folder_end = fn_0040ab30(e, Ptr::new(buffer), 0x5c).addr();
                    let mut node = e.call(MEMBER_LIST_AT_4, &args![animation]).u32();
                    while node != 0 {
                        e.mem.set_u8(folder_end.wrapping_add(1), 0);
                        let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
                        let name = e.mem.u32(slot);
                        if name != 0 {
                            e.call(STRING_APPEND, &args![buffer, PATH_BUFFER_SIZE, name]);
                            let mut i = 0i32;
                            while i < count {
                                load_animation_file(e, buffer);
                                i += 1;
                            }
                        }
                        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
                    }
                }
            });
        }
        e.call(MODEL_POINTER_DESTRUCT, &args![pointer]);
    });
}

/// `FinishedLoading`'s arms for a setting's associated form, chosen by the setting's flag
/// masks 0x10000, 0x20000 and 0x40000 (each tested only when there is an associated form,
/// in this order); the models found get `count` references.
fn finish_associated_form(e: &mut Engine, setting: u32, associated: u32, count: i32) {
    if associated != 0
        && e.call(FORM_HAS_FLAG, &args![setting, 0x0001_0000u32])
            .bool()
    {
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_10000);
        if form != 0
            && e.call(MODEL_PATH_LENGTH, &args![form.wrapping_add(0x3c)])
                .u32()
                != 0
        {
            e.with_stack(4, |e, pointer| {
                e.call(MODEL_POINTER_CONSTRUCT, &args![pointer, 0u32]);
                let model = form.wrapping_add(0x3c);
                let path = e.vcall(model, SLOT_MODEL_PATH, &args![]).u32();
                find_model(e, path, pointer.addr());
                add_model_references(e, pointer.addr(), count);
                e.call(MODEL_POINTER_DESTRUCT, &args![pointer]);
            });
        }
    } else if associated != 0
        && e.call(FORM_HAS_FLAG, &args![setting, 0x0002_0000u32])
            .bool()
    {
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_20000);
        if form != 0 {
            e.with_stack(4, |e, pointer| {
                e.call(MODEL_POINTER_CONSTRUCT, &args![pointer, 0u32]);
                let biped = form.wrapping_add(0x70);
                let first = e.call(BIPED_MODEL_BY_INDEX, &args![biped, 0u32]).u32();
                find_model(e, first, pointer.addr());
                add_model_references(e, pointer.addr(), count);
                let second = e.call(BIPED_MODEL_BY_INDEX, &args![biped, 1u32]).u32();
                find_model(e, second, pointer.addr());
                add_model_references(e, pointer.addr(), count);
                e.call(MODEL_POINTER_DESTRUCT, &args![pointer]);
            });
        }
    } else if associated != 0
        && e.call(FORM_HAS_FLAG, &args![setting, 0x0004_0000u32])
            .bool()
    {
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_40000_FIRST);
        if form != 0 {
            let list = e
                .call(BUILD_DEFAULT_MODEL_LIST, &args![form, 1u32, 1u32])
                .u32();
            let mut node = list;
            while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
                let slot = e.call(LIST_NODE_ITEM_SLOT, &args![node]).u32();
                let path = e.mem.u32(slot);
                e.with_stack(4, |e, pointer| {
                    e.call(MODEL_POINTER_CONSTRUCT, &args![pointer, 0u32]);
                    find_model(e, path, pointer.addr());
                    add_model_references(e, pointer.addr(), count);
                    node = e.call(LIST_NODE_NEXT, &args![node]).u32();
                    e.call(MODEL_POINTER_DESTRUCT, &args![pointer]);
                });
            }
            e.call(FILENAME_LIST_CLEAR, &args![list]);
            if list != 0 {
                e.call(FILENAME_LIST_DESTROY, &args![list, 1u32]);
            }
        }
        let form = dynamic_cast(e, associated, RTTI_CAST_FOR_40000_SECOND);
        if form != 0 {
            finish_model_with_animations(e, form, count);
        }
    }
}

// Translated from 0040ab70 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0040ab70 - the C++ exception frame (FS:[0] chain, unwind states 0 to 5 around the model pointers) and the /GS security cookie check (00ec408c) are not translated.
/// `MagicItem::FinishedLoading` (engine map; `thiscall`, plain `RET`): when the item is
/// not loading (`0040b6b0`) it is preloaded and unloaded (`Preload(1)`, `Unload(1)`).
/// Otherwise the models its loading queued get their manual references: the model of its
/// strongest effect (`GetAVEffect(0)`, +0x18, when it has a path) by the effect's queued
/// count (`004089c0`, when `IsEffectQueued`, else 0); then, for each effect item of the
/// list at +0xc (none for an empty list), the models of its setting's associated form by
/// the setting's associated count (`004087e0` when `IsAssociatedItemQueued`; nothing when
/// that is 0), as [`finish_associated_form`] finds them.
pub fn magic_item_finished_loading(e: &mut Engine, this: Ptr) {
    let effect = e.call(MAGIC_ITEM_GET_AV_EFFECT, &args![this, 0u32]).u32();
    if !e.call(MAGIC_ITEM_IS_LOADING, &args![this]).bool() {
        magic_item_preload(e, this, 1);
        magic_item_unload(e, this, 1);
        return;
    }
    if effect != 0 {
        let count = if e
            .call(EFFECT_SETTING_IS_EFFECT_QUEUED, &args![effect])
            .bool()
        {
            e.call(SETTING_EFFECT_COUNT_004089C0, &args![effect]).i32()
        } else {
            0
        };
        let model = effect.wrapping_add(0x18);
        if e.call(MODEL_PATH_LENGTH, &args![model]).u32() != 0 {
            e.with_stack(4, |e, pointer| {
                e.call(MODEL_POINTER_CONSTRUCT, &args![pointer, 0u32]);
                let path = e.vcall(model, SLOT_MODEL_PATH, &args![]).u32();
                find_model(e, path, pointer.addr());
                add_model_references(e, pointer.addr(), count);
                e.call(MODEL_POINTER_DESTRUCT, &args![pointer]);
            });
        }
    }
    let mut walked = effect_list(this);
    if e.call(LIST_NODE_IS_EMPTY, &args![walked.wrapping_add(4)])
        .bool()
    {
        return;
    }
    while walked != 0 {
        let slot = e
            .call(LIST_NODE_ITEM_SLOT, &args![walked.wrapping_add(4)])
            .u32();
        let effect_item = e.mem.u32(slot);
        let setting = e.call(EFFECT_ITEM_SETTING, &args![effect_item]).u32();
        let associated = e
            .call(EFFECT_SETTING_ASSOCIATED_FORM, &args![setting])
            .u32();
        let count = if e
            .call(EFFECT_SETTING_IS_ASSOCIATED_ITEM_QUEUED, &args![setting])
            .bool()
        {
            e.call(SETTING_ASSOCIATED_COUNT_004087E0, &args![setting])
                .i32()
        } else {
            0
        };
        if count != 0 {
            finish_associated_form(e, setting, associated, count);
        }
        walked = next_walked(e, walked.wrapping_add(4));
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0040_a250,
            magic_item_get_magic_item_by_numeric_id(u32) -> u32
        ),
        entry!(0x0040_ab50, fn_0040ab50()),
        entry!(0x0040_bed0, fn_0040bed0()),
        entry!(0x0040_bef0, fn_0040bef0(Ptr, u32, u32) -> u32),
        entry!(0x0040_ab70, magic_item_finished_loading(Ptr)),
        entry!(0x0040_a420, magic_item_preload(Ptr, u8)),
        entry!(0x0040_ab30, fn_0040ab30(Ptr, u32) -> Ptr),
        entry!(0x0040_a1e0, magic_item_get_magic_item_form_id(Ptr) -> u32),
        entry!(0x0040_a300, magic_item_get_av_effect(Ptr, i32) -> u32),
        entry!(0x0040_b470, magic_item_is_loaded(Ptr) -> bool),
        entry!(0x0040_b580, magic_item_is_loading_or_loaded(Ptr) -> bool),
        entry!(0x0040_b6b0, magic_item_is_loading(Ptr) -> bool),
        entry!(0x0040_b800, magic_item_unload(Ptr, u8)),
        entry!(0x0040_c030, magic_item_magic_item_init_item(Ptr, u32)),
    ]
}

#[cfg(test)]
mod physics_work_ahead_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<String>>>;

    const LOADER: u32 = 0x00c0_ffee;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn text(e: &Engine, address: u32) -> String {
        String::from_utf8(e.mem.cstr(address)).unwrap()
    }

    fn put_text(e: &mut Engine, text: &str) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(address, text.as_bytes());
        address
    }

    /// A `TESModel`-shaped object at `address`: vtable (slot +0x14 answers the path), the path
    /// length at +4 and the path pointer at +8.
    fn put_model(e: &mut Engine, address: u32, path: &str) {
        let table = 0x0f00_8000;
        let mut slots = [0u32; 8];
        slots[SLOT_MODEL_PATH as usize / 4] = 0x0f00_8100;
        e.put_vtable(table, &slots);
        e.mem.set_u32(address, table);
        let pointer = if path.is_empty() {
            0
        } else {
            put_text(e, path)
        };
        e.mem.set_u32(address + 4, path.len() as u32);
        e.mem.set_u32(address + 8, pointer);
    }

    /// An engine with every callee of the function doubled over simple objects, and the model
    /// loader's `LoadFile` and `LoadKF` recording the paths they are given.
    fn rig(effect: u32) -> (Engine, Log, Log) {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.map(MODEL_LOADER, 4);
        e.set_global(MODEL_LOADER, LOADER);
        e.register_double(MAGIC_ITEM_GET_AV_EFFECT, move |_, _| effect.into_ret());
        e.register(MAGIC_ITEM_IS_LOADED, |_, _| Ret::default());
        e.register(MAGIC_ITEM_IS_LOADING, |_, _| Ret::default());
        e.register(MODEL_LOADER_QUEUE_ITEM, |_, _| Ret::default());
        e.register(MODEL_PATH_LENGTH, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(0x0f00_8100, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(FORM_COUNTER_A8, |_, _| Ret::default());
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_NODE_ITEM_SLOT, |_, a| a[0].into_ret());
        e.register(LIST_NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(EFFECT_ITEM_SETTING, |e, a| {
            e.mem.u32(a[0] + 0x14).into_ret()
        });
        e.register(EFFECT_SETTING_QUALIFIES, |_, a| {
            u32::from(a[0] != 0).into_ret()
        });
        e.register(EFFECT_SETTING_ASSOCIATED_FORM, |e, a| {
            e.mem.u32(a[0] + 0x60).into_ret()
        });
        e.register(FORM_HAS_FLAG, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) & a[1] != 0).into_ret()
        });
        e.register(SETTING_COUNTER_POSITIVE, |_, _| Ret::default());
        e.register(SETTING_COUNTER_NEGATIVE, |_, _| Ret::default());
        e.register(COUNT_PRELOADED, |_, _| Ret::default());
        e.register(SETTING_COUNTER_ADJUST, |_, _| Ret::default());
        let loaded: Log = Rc::new(RefCell::new(Vec::new()));
        let animations: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = loaded.clone();
        e.register_double(MODEL_LOADER_LOAD_FILE, move |e, a| {
            assert_eq!(a[0], LOADER);
            assert_eq!(a[2..], [0, 1, 0, 0, 0]);
            sink.borrow_mut().push(text(e, a[1]));
            Ret::default()
        });
        let sink = animations.clone();
        e.register_double(MODEL_LOADER_LOAD_KF, move |e, a| {
            assert_eq!(a[0], LOADER);
            sink.borrow_mut().push(text(e, a[1]));
            Ret::default()
        });
        (e, loaded, animations)
    }

    /// A magic item with one effect item whose setting has the given flags (at +8) and
    /// associated form (at +0x60); the list head is at `this + 0x10`.
    fn item_with_setting(e: &mut Engine, flags: u32, associated: u32) -> u32 {
        let this = e.mem.alloc(0x20);
        let effect_item = e.mem.alloc(0x20);
        let setting = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x10, effect_item);
        e.mem.set_u32(this + 0x14, 0);
        e.mem.set_u32(effect_item + 0x14, setting);
        e.mem.set_u32(setting + 8, flags);
        e.mem.set_u32(setting + 0x60, associated);
        this
    }

    #[test]
    fn at_0040a420_only_tells_the_loader_when_the_item_is_not_ready() {
        let (mut e, loaded, _) = rig(0x9000);
        let this = item_with_setting(&mut e, 0, 0);
        e.call(0x0040_a420, &args![this, 0u8]);
        assert_eq!(
            calls(&e, MODEL_LOADER_QUEUE_ITEM),
            vec![vec![LOADER, this, 0x9000, 0]]
        );
        assert_eq!(calls(&e, MAGIC_ITEM_GET_AV_EFFECT), vec![vec![this, 0]]);
        assert_eq!(calls(&e, MAGIC_ITEM_IS_LOADED), vec![vec![this]]);
        assert_eq!(calls(&e, MAGIC_ITEM_IS_LOADING), vec![vec![this]]);
        assert!(calls(&e, FORM_COUNTER_A8).is_empty());
        assert!(calls(&e, LIST_NODE_IS_EMPTY).is_empty());
        assert!(loaded.borrow().is_empty());
    }

    #[test]
    fn at_0040a420_force_skips_the_state_checks_and_loads_the_effect_model() {
        let (mut e, loaded, _) = rig(0);
        let effect = e.mem.alloc(0x40);
        put_model(&mut e, effect + 0x18, "effect.nif");
        e.register_double(MAGIC_ITEM_GET_AV_EFFECT, move |_, _| effect.into_ret());
        let this = e.mem.alloc(0x20);
        // An empty list head: nothing is walked.
        e.call(0x0040_a420, &args![this, 1u8]);
        assert!(calls(&e, MAGIC_ITEM_IS_LOADED).is_empty());
        assert!(calls(&e, MAGIC_ITEM_IS_LOADING).is_empty());
        assert_eq!(*loaded.borrow(), vec!["effect.nif".to_string()]);
        assert_eq!(calls(&e, FORM_COUNTER_A8), vec![vec![effect]]);
        assert_eq!(calls(&e, LIST_NODE_IS_EMPTY), vec![vec![this + 0x10]]);
        assert!(calls(&e, MODEL_LOADER_QUEUE_ITEM).is_empty());
    }

    #[test]
    fn at_0040a420_a_model_without_a_path_is_not_loaded() {
        let (mut e, loaded, _) = rig(0);
        let effect = e.mem.alloc(0x40);
        put_model(&mut e, effect + 0x18, "");
        e.register_double(MAGIC_ITEM_GET_AV_EFFECT, move |_, _| effect.into_ret());
        let this = e.mem.alloc(0x20);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert!(loaded.borrow().is_empty());
        assert_eq!(calls(&e, FORM_COUNTER_A8), vec![vec![effect]]);
    }

    #[test]
    fn at_0040a420_an_item_that_is_only_loading_counts_the_effect_without_loading() {
        let (mut e, loaded, _) = rig(0);
        let effect = e.mem.alloc(0x40);
        put_model(&mut e, effect + 0x18, "effect.nif");
        e.register_double(MAGIC_ITEM_GET_AV_EFFECT, move |_, _| effect.into_ret());
        e.register(MAGIC_ITEM_IS_LOADING, |_, _| 1u32.into_ret());
        let this = item_with_setting(&mut e, 0x0004_0000, 0);
        e.call(0x0040_a420, &args![this, 0u8]);
        assert!(loaded.borrow().is_empty());
        assert_eq!(calls(&e, FORM_COUNTER_A8), vec![vec![effect]]);
        // The setting still goes through the counters: the 0x40000 flag counts it globally.
        let setting = e.mem.u32(e.mem.u32(this + 0x10) + 0x14);
        assert_eq!(calls(&e, SETTING_COUNTER_POSITIVE), vec![vec![setting]]);
        assert_eq!(calls(&e, SETTING_COUNTER_NEGATIVE), vec![vec![setting]]);
        assert_eq!(calls(&e, COUNT_PRELOADED).len(), 1);
        assert_eq!(calls(&e, SETTING_COUNTER_ADJUST), vec![vec![setting]]);
        // No associated form is looked at for an item that is not loaded.
        assert!(calls(&e, EFFECT_SETTING_ASSOCIATED_FORM).is_empty());
    }

    #[test]
    fn at_0040a420_counters_decide_whether_the_setting_is_counted() {
        for (positive, negative, counted) in [(0u32, 0u32, true), (1, 0, false), (0, 1, false)] {
            let (mut e, _, _) = rig(0);
            e.register_double(SETTING_COUNTER_POSITIVE, move |_, _| positive.into_ret());
            e.register_double(SETTING_COUNTER_NEGATIVE, move |_, _| negative.into_ret());
            let this = item_with_setting(&mut e, 0x0004_0000, 0);
            e.call(0x0040_a420, &args![this, 1u8]);
            assert_eq!(calls(&e, COUNT_PRELOADED).len(), usize::from(counted));
            assert_eq!(calls(&e, SETTING_COUNTER_ADJUST).len(), 1);
        }
        // Without the 0x40000 flag nothing is counted globally.
        let (mut e, _, _) = rig(0);
        let this = item_with_setting(&mut e, 0x0001_0000, 0);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert!(calls(&e, COUNT_PRELOADED).is_empty());
    }

    #[test]
    fn at_0040a420_a_setting_that_does_not_qualify_is_skipped() {
        let (mut e, loaded, _) = rig(0);
        e.register(EFFECT_SETTING_QUALIFIES, |_, _| Ret::default());
        let this = item_with_setting(&mut e, 0x0004_0000, 0x1234);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert!(loaded.borrow().is_empty());
        assert!(calls(&e, SETTING_COUNTER_ADJUST).is_empty());
        assert!(calls(&e, EFFECT_SETTING_ASSOCIATED_FORM).is_empty());
    }

    #[test]
    fn at_0040a420_walks_the_whole_list_and_checks_it_is_not_empty_once() {
        let (mut e, _, _) = rig(0);
        let this = item_with_setting(&mut e, 0, 0);
        // A second effect item hangs off the first node: its `next` points 4 past the next
        // container (the list head sits at +4 of the walked pointer).
        let second_container = e.mem.alloc(0x20);
        let second_item = e.mem.alloc(0x20);
        let second_setting = e.mem.alloc(0x80);
        e.mem.set_u32(second_container + 4, second_item);
        e.mem.set_u32(second_item + 0x14, second_setting);
        e.mem.set_u32(this + 0x14, second_container + 4);
        e.call(0x0040_a420, &args![this, 1u8]);
        let first_setting = e.mem.u32(e.mem.u32(this + 0x10) + 0x14);
        assert_eq!(
            calls(&e, SETTING_COUNTER_ADJUST),
            vec![vec![first_setting], vec![second_setting]]
        );
        assert_eq!(calls(&e, LIST_NODE_IS_EMPTY).len(), 1);
    }

    #[test]
    fn at_0040a420_flag_0x10000_loads_the_model_of_the_cast_form() {
        let (mut e, loaded, _) = rig(0);
        let form = e.mem.alloc(0x100);
        put_model(&mut e, form + 0x3c, "weapon.nif");
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            assert_eq!(a[1..3], [0, RTTI_SOURCE_TYPE]);
            assert_eq!(a[4], 0);
            if a[3] == RTTI_CAST_FOR_10000 { a[0] } else { 0 }.into_ret()
        });
        let this = item_with_setting(&mut e, 0x0001_0000, form);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert_eq!(*loaded.borrow(), vec!["weapon.nif".to_string()]);
        assert_eq!(calls(&e, RT_DYNAMIC_CAST).len(), 1);
        // The other masks are not tested once the first one matched.
        assert_eq!(calls(&e, FORM_HAS_FLAG).len(), 2);
    }

    #[test]
    fn at_0040a420_a_failed_cast_loads_nothing() {
        let (mut e, loaded, _) = rig(0);
        e.register(RT_DYNAMIC_CAST, |_, _| Ret::default());
        let this = item_with_setting(&mut e, 0x0007_0000, 0x5555);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert!(loaded.borrow().is_empty());
        assert_eq!(calls(&e, RT_DYNAMIC_CAST).len(), 1);
    }

    #[test]
    fn at_0040a420_flag_0x20000_loads_both_biped_models() {
        let (mut e, loaded, _) = rig(0);
        let form = e.mem.alloc(0x100);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            if a[3] == RTTI_CAST_FOR_20000 { a[0] } else { 0 }.into_ret()
        });
        let male = put_text(&mut e, "male.nif");
        let female = put_text(&mut e, "female.nif");
        e.register_double(BIPED_MODEL_BY_INDEX, move |_, a| {
            assert_eq!(a[0], form + 0x70);
            [male, female][a[1] as usize].into_ret()
        });
        let this = item_with_setting(&mut e, 0x0002_0000, form);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert_eq!(
            *loaded.borrow(),
            vec!["male.nif".to_string(), "female.nif".to_string()]
        );
    }

    #[test]
    fn at_0040a420_flag_0x40000_loads_the_default_models_and_frees_the_list() {
        let (mut e, loaded, _) = rig(0);
        let form = e.mem.alloc(0x100);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            if a[3] == RTTI_CAST_FOR_40000_FIRST {
                a[0]
            } else {
                0
            }
            .into_ret()
        });
        let second = e.mem.alloc(8);
        let first = e.mem.alloc(8);
        let first_path = put_text(&mut e, "head.nif");
        let second_path = put_text(&mut e, "hair.nif");
        e.mem.set_u32(first, first_path);
        e.mem.set_u32(first + 4, second);
        e.mem.set_u32(second, second_path);
        e.register_double(BUILD_DEFAULT_MODEL_LIST, move |_, a| {
            assert_eq!(a[1..], [1, 1]);
            first.into_ret()
        });
        e.register(FILENAME_LIST_CLEAR, |_, _| Ret::default());
        e.register(FILENAME_LIST_DESTROY, |_, _| Ret::default());
        let this = item_with_setting(&mut e, 0x0004_0000, form);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert_eq!(
            *loaded.borrow(),
            vec!["head.nif".to_string(), "hair.nif".to_string()]
        );
        assert_eq!(calls(&e, BUILD_DEFAULT_MODEL_LIST), vec![vec![form, 1, 1]]);
        assert_eq!(calls(&e, FILENAME_LIST_CLEAR), vec![vec![first]]);
        assert_eq!(calls(&e, FILENAME_LIST_DESTROY), vec![vec![first, 1]]);
        // The second cast is tried as well (and fails here).
        assert_eq!(calls(&e, RT_DYNAMIC_CAST).len(), 2);
    }

    #[test]
    fn at_0040a420_flag_0x40000_without_a_list_still_clears_it() {
        let (mut e, loaded, _) = rig(0);
        let form = e.mem.alloc(0x100);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            if a[3] == RTTI_CAST_FOR_40000_FIRST {
                a[0]
            } else {
                0
            }
            .into_ret()
        });
        e.register(BUILD_DEFAULT_MODEL_LIST, |_, _| Ret::default());
        e.register(FILENAME_LIST_CLEAR, |_, _| Ret::default());
        e.register(FILENAME_LIST_DESTROY, |_, _| Ret::default());
        let this = item_with_setting(&mut e, 0x0004_0000, form);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert!(loaded.borrow().is_empty());
        assert_eq!(calls(&e, FILENAME_LIST_CLEAR), vec![vec![0]]);
        assert!(calls(&e, FILENAME_LIST_DESTROY).is_empty());
    }

    /// A node list whose items are the given strings.
    fn string_list(e: &mut Engine, items: &[&str]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            let pointer = put_text(e, item);
            e.mem.set_u32(node, pointer);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    /// Everything the last arm calls: the string functions work on game memory, the lists are
    /// the given ones. Returns the form (model at +0xdc, extra files at +0x114, animations at
    /// +0xc4) and the log of the paths `FREE_BLOCK` was given.
    fn rig_last_arm(
        e: &mut Engine,
        model_path: &str,
        extra: &[&str],
        kf_files: &[&str],
        animations: &[&str],
        has_special_animations: bool,
    ) -> (u32, Log) {
        for (address, content) in [
            (SKELETON_TEXT, "Skeleton"),
            (SPECIAL_ANIMS_TEXT, "SpecialAnims"),
            (SPECIAL_ANIMS_SEPARATOR_TEXT, "\\"),
        ] {
            e.map(address, 16);
            e.mem.set_cstr(address, content.as_bytes());
        }
        e.register(STRING_COPY, |e, a| {
            let source = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &source);
            Ret::default()
        });
        e.register(STRING_APPEND, |e, a| {
            let mut joined = e.mem.cstr(a[0]);
            joined.extend(e.mem.cstr(a[2]));
            e.mem.set_cstr(a[0], &joined);
            Ret::default()
        });
        e.register(CRT_STRRCHR, |e, a| {
            let bytes = e.mem.cstr(a[0]);
            match bytes.iter().rposition(|b| u32::from(*b) == a[1]) {
                Some(index) => (a[0] + index as u32).into_ret(),
                None => Ret::default(),
            }
        });
        e.register(CRT_STRNICMP, |e, a| {
            let left = e.mem.cstr(a[0]).to_ascii_lowercase();
            let right = e.mem.cstr(a[1]).to_ascii_lowercase();
            let n = a[2] as usize;
            let left = &left[..left.len().min(n)];
            let right = &right[..right.len().min(n)];
            u32::from(left != right).into_ret()
        });
        let form = e.mem.alloc(0x140);
        put_model(e, form + 0xdc, model_path);
        let extra_list = string_list(e, extra);
        let animation_list = string_list(e, animations);
        e.register_double(MEMBER_LIST_AT_4, move |_, a| {
            if a[0] == form + 0x114 {
                extra_list.into_ret()
            } else {
                assert_eq!(a[0], form + 0xc4);
                animation_list.into_ret()
            }
        });
        let kf_list = string_list(e, kf_files);
        e.register_double(MODEL_LOADER_BUILD_KF_LIST, move |_, a| {
            assert_eq!(a[0], LOADER);
            assert_eq!(a[2..], [0, 0, 0]);
            kf_list.into_ret()
        });
        e.register(FILENAME_LIST_CLEAR, |_, _| Ret::default());
        e.register(FILENAME_LIST_DESTROY, |_, _| Ret::default());
        e.register_double(ANIMATION_HAS_KF_FILES, move |_, a| {
            assert_eq!(a[0], form + 0xc4);
            u32::from(has_special_animations).into_ret()
        });
        let freed: Log = Rc::new(RefCell::new(Vec::new()));
        let sink = freed.clone();
        e.register_double(FREE_BLOCK, move |e, a| {
            sink.borrow_mut().push(text(e, a[0]));
            Ret::default()
        });
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            if a[3] == RTTI_CAST_FOR_40000_SECOND {
                a[0]
            } else {
                0
            }
            .into_ret()
        });
        (form, freed)
    }

    #[test]
    fn at_0040a420_the_last_arm_loads_the_model_its_files_and_the_animations() {
        let (mut e, loaded, animations) = rig(0);
        let (form, freed) = rig_last_arm(
            &mut e,
            "actors\\skeleton\\Skeleton.nif",
            &["a.nif", "b.nif"],
            &["idle.kf", "walk.kf"],
            &["x.kf", "y.kf"],
            true,
        );
        let this = item_with_setting(&mut e, 0x0004_0000, form);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert_eq!(
            *loaded.borrow(),
            vec![
                "actors\\skeleton\\Skeleton.nif".to_string(),
                "actors\\skeleton\\a.nif".to_string(),
                "actors\\skeleton\\b.nif".to_string(),
            ]
        );
        assert_eq!(
            *animations.borrow(),
            vec![
                "idle.kf".to_string(),
                "walk.kf".to_string(),
                "actors\\skeleton\\SpecialAnims\\x.kf".to_string(),
                "actors\\skeleton\\SpecialAnims\\y.kf".to_string(),
            ]
        );
        assert_eq!(
            *freed.borrow(),
            vec!["idle.kf".to_string(), "walk.kf".to_string()]
        );
        // The skeleton file list is emptied and destroyed.
        let list = calls(&e, MODEL_LOADER_BUILD_KF_LIST).len();
        assert_eq!(list, 1);
        assert_eq!(calls(&e, FILENAME_LIST_CLEAR).len(), 1);
        assert_eq!(calls(&e, FILENAME_LIST_DESTROY).len(), 1);
    }

    #[test]
    fn at_0040a420_the_last_arm_skips_what_does_not_apply() {
        let (mut e, loaded, animations) = rig(0);
        let (form, freed) = rig_last_arm(
            &mut e,
            "actors\\skeleton\\Other.nif",
            &[],
            &["idle.kf"],
            &["x.kf"],
            false,
        );
        let this = item_with_setting(&mut e, 0x0004_0000, form);
        e.call(0x0040_a420, &args![this, 1u8]);
        assert_eq!(
            *loaded.borrow(),
            vec!["actors\\skeleton\\Other.nif".to_string()]
        );
        assert!(animations.borrow().is_empty());
        assert!(freed.borrow().is_empty());
        assert!(calls(&e, MODEL_LOADER_BUILD_KF_LIST).is_empty());
        assert_eq!(calls(&e, ANIMATION_HAS_KF_FILES).len(), 1);
    }
}

#[cfg(test)]
mod b066_tests {
    use super::*;

    type Calls = Vec<Vec<u32>>;

    const MODEL_TABLE: u32 = 0x0f10_0000;
    const MODEL_PATH_SLOT: u32 = 0x0f10_0100;
    const LOADER: u32 = 0x00c0_ffee;

    fn answer(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn calls(e: &Engine, addr: u32) -> Calls {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn loaded_byte(e: &mut Engine, a: &[u32]) -> Ret {
        u32::from(e.mem.u8(a[0] + 0x30)).into_ret()
    }

    fn queued_byte(e: &mut Engine, a: &[u32]) -> Ret {
        u32::from(e.mem.u8(a[0] + 0x31)).into_ret()
    }

    fn associated_loaded_byte(e: &mut Engine, a: &[u32]) -> Ret {
        u32::from(e.mem.u8(a[0] + 0x32)).into_ret()
    }

    fn associated_queued_byte(e: &mut Engine, a: &[u32]) -> Ret {
        u32::from(e.mem.u8(a[0] + 0x33)).into_ret()
    }

    /// Doubles over plain objects: list nodes are (item, next) with `next` the next
    /// node; an effect item has its setting at +0x14, its level at +0x18, its cost
    /// (`float`) at +0x1c and its skip byte at +0x20; a setting has its flags at +8,
    /// a model at +0x18 (vtable, path length at +4, path at +8 of it), its loaded,
    /// queued, associated-loaded and associated-queued bytes at +0x30..+0x33 and its
    /// associated form at +0x60; a qualifying setting is any non-null one.
    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(MODEL_LOADER, 4);
        e.set_global(MODEL_LOADER, LOADER);
        e.put_vtable(MODEL_TABLE, &[0, 0, 0, 0, 0, MODEL_PATH_SLOT]);
        e.register(MODEL_PATH_SLOT, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(MODEL_PATH_LENGTH, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(LIST_NODE_ITEM_SLOT, |_, a| a[0].into_ret());
        e.register(LIST_NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(EFFECT_ITEM_SETTING, |e, a| {
            e.mem.u32(a[0] + 0x14).into_ret()
        });
        e.register(GETTER_0084E3A0, |e, a| e.mem.u32(a[0] + 0x18).into_ret());
        e.register(EFFECT_ITEM_GET_COST, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0] + 0x1c)),
            ..Ret::default()
        });
        e.register(FTOL, |_, a| {
            (f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32) as i32).into_ret()
        });
        e.register(EFFECT_ITEM_SKIPPED, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x20)).into_ret()
        });
        e.register(EFFECT_SETTING_QUALIFIES, |_, a| {
            u32::from(a[0] != 0).into_ret()
        });
        e.register(EFFECT_SETTING_IS_EFFECT_LOADED, loaded_byte);
        e.register(EFFECT_SETTING_IS_EFFECT_QUEUED, queued_byte);
        e.register(SETTING_COUNTER_POSITIVE, associated_loaded_byte);
        e.register(SETTING_COUNTER_NEGATIVE, associated_queued_byte);
        e.register(EFFECT_SETTING_ASSOCIATED_FORM, |e, a| {
            e.mem.u32(a[0] + 0x60).into_ret()
        });
        e.register(FORM_HAS_FLAG, |e, a| {
            u32::from(e.mem.u32(a[0] + 8) & a[1] != 0).into_ret()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        for f in [
            MODEL_LOADER_RELEASE_MODEL,
            MODEL_LOADER_RELEASE_KF,
            SETTING_UNLOAD_00408910,
            SETTING_UNLOAD_00408730,
            UNCOUNT_PRELOADED,
            FILENAME_LIST_CLEAR,
            FILENAME_LIST_DESTROY,
        ] {
            answer(&mut e, f, 0);
        }
        e.call_log = Some(vec![]);
        e
    }

    /// A setting with a model of path length `path` at +0x18 (path pointer `path + 0x100`).
    fn setting(e: &mut Engine, path: u32, flags: u32) -> u32 {
        let setting = e.mem.alloc(0x80);
        e.mem.set_u32(setting + 8, flags);
        e.mem.set_u32(setting + 0x18, MODEL_TABLE);
        e.mem.set_u32(setting + 0x1c, path);
        e.mem.set_u32(setting + 0x20, path + 0x100);
        setting
    }

    /// An effect item with `setting`, `level` and `cost`.
    fn effect_item(e: &mut Engine, setting: u32, level: u32, cost: f32) -> u32 {
        let item = e.mem.alloc(0x24);
        e.mem.set_u32(item + 0x14, setting);
        e.mem.set_u32(item + 0x18, level);
        e.mem.set_f32(item + 0x1c, cost);
        item
    }

    /// A magic item whose effect list holds `items` (the first node at +0x10).
    fn magic_item(e: &mut Engine, items: &[u32]) -> Ptr {
        let this = e.mem.alloc(0x20);
        let mut node = this + 0x10;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            let next = if i + 1 < items.len() {
                e.mem.alloc(8)
            } else {
                0
            };
            e.mem.set_u32(node + 4, next);
            node = next;
        }
        Ptr::new(this)
    }

    #[test]
    fn get_magic_item_form_id_casts_to_form_then_object() {
        for (form, object, want) in [(0x10u32, 0x20u32, 0x110u32), (0, 0x20, 0x120), (0, 0, 0)] {
            let mut e = Engine::new();
            e.register_double(RT_DYNAMIC_CAST, move |_, a| {
                (if a[3] == MAGIC_ITEM_FORM_TYPE {
                    form
                } else {
                    object
                })
                .into_ret()
            });
            e.register(GETTER_0084E3A0, |_, a| (a[0] + 0x100).into_ret());
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0040_a1e0, &args![0x5000u32]).u32(), want);
            let casts = calls(&e, RT_DYNAMIC_CAST);
            assert_eq!(
                casts[0],
                vec![0x5000, 0, MAGIC_ITEM_TYPE, MAGIC_ITEM_FORM_TYPE, 0]
            );
            assert_eq!(
                casts[1],
                vec![0x5000, 0, MAGIC_ITEM_TYPE, MAGIC_ITEM_OBJECT_TYPE, 0]
            );
        }
    }

    #[test]
    fn get_av_effect_picks_the_highest_level_then_cost() {
        let mut e = rig();
        let a = effect_item(&mut e, 0xa0, 2, 5.0);
        let b = effect_item(&mut e, 0xb0, 3, 1.0);
        let c = effect_item(&mut e, 0xc0, 3, 4.9);
        let d = effect_item(&mut e, 0xd0, 9, 9.0);
        e.mem.set_u8(d + 0x20, 1);
        let item = magic_item(&mut e, &[a, b, c, d]);
        assert_eq!(e.call(0x0040_a300, &args![item, 0u32]).u32(), 0xc0);
        // Nothing above the minimum level: none.
        assert_eq!(e.call(0x0040_a300, &args![item, 4u32]).u32(), 0);
        // Equal level to the minimum: the higher cost wins over -1.
        let item = magic_item(&mut e, &[a]);
        assert_eq!(e.call(0x0040_a300, &args![item, 2u32]).u32(), 0xa0);
        // An empty list.
        let item = magic_item(&mut e, &[]);
        assert_eq!(e.call(0x0040_a300, &args![item, 0u32]).u32(), 0);
    }

    /// A magic item whose strongest effect's setting has a model (loaded and queued as
    /// given) and one more qualifying setting (associated loaded and queued as given).
    fn states(model: (u8, u8), associated: (u8, u8)) -> (Engine, Ptr) {
        let mut e = rig();
        let first = setting(&mut e, 4, 0);
        e.mem.set_u8(first + 0x30, model.0);
        e.mem.set_u8(first + 0x31, model.1);
        e.mem.set_u8(first + 0x32, 1);
        let second = setting(&mut e, 0, 0);
        e.mem.set_u8(second + 0x32, associated.0);
        e.mem.set_u8(second + 0x33, associated.1);
        let a = effect_item(&mut e, first, 1, 0.0);
        let b = effect_item(&mut e, second, 0, 0.0);
        let item = magic_item(&mut e, &[a, b]);
        (e, item)
    }

    #[test]
    fn loaded_loading_and_loading_or_loaded_states() {
        // (model loaded, queued), (associated loaded, queued) -> (loaded, loading or
        // loaded, loading)
        for (model, associated, want) in [
            ((1, 0), (1, 0), (true, true, false)),
            ((0, 1), (1, 0), (false, true, true)),
            ((0, 0), (1, 0), (false, false, false)),
            ((1, 0), (0, 1), (false, true, true)),
            ((1, 0), (0, 0), (false, false, false)),
            ((0, 1), (0, 1), (false, true, true)),
        ] {
            let (mut e, item) = states(model, associated);
            let got = (
                e.call(0x0040_b470, &args![item]).bool(),
                e.call(0x0040_b580, &args![item]).bool(),
                e.call(0x0040_b6b0, &args![item]).bool(),
            );
            assert_eq!(got, want, "{model:?} {associated:?}");
        }
        // An empty list without an effect: loaded, not loading.
        let mut e = rig();
        let item = magic_item(&mut e, &[]);
        assert!(e.call(0x0040_b470, &args![item]).bool());
        assert!(e.call(0x0040_b580, &args![item]).bool());
        assert!(!e.call(0x0040_b6b0, &args![item]).bool());
    }

    /// The settings of the first two effect items of `item`.
    fn settings_of(e: &Engine, item: Ptr) -> (u32, u32) {
        let first = e.mem.u32(e.mem.u32(item.addr() + 0x10) + 0x14);
        let node = e.mem.u32(item.addr() + 0x14);
        let second = e.mem.u32(e.mem.u32(node) + 0x14);
        (first, second)
    }

    #[test]
    fn unload_releases_the_effect_and_associated_models() {
        let (mut e, item) = states((1, 0), (1, 0));
        let (first, second) = settings_of(&e, item);
        // The second setting gets the 0x10000 arm with a model of length 3 at +0x3c.
        let associated = e.mem.alloc(0x50);
        e.mem.set_u32(associated + 0x3c, MODEL_TABLE);
        e.mem.set_u32(associated + 0x40, 3);
        e.mem.set_u32(associated + 0x44, 0x7700);
        e.mem.set_u32(second + 8, 0x0001_0000);
        e.mem.set_u32(second + 0x60, associated);
        e.call_log = Some(vec![]);
        e.call(0x0040_b800, &args![item, 0x101u32]);
        assert_eq!(
            calls(&e, MODEL_LOADER_RELEASE_MODEL),
            vec![vec![LOADER, 0x104, 1, 1], vec![LOADER, 0x7700, 1, 1]]
        );
        assert_eq!(calls(&e, SETTING_UNLOAD_00408910), vec![vec![first]]);
        assert_eq!(
            calls(&e, SETTING_UNLOAD_00408730),
            vec![vec![first], vec![second]]
        );
        assert!(calls(&e, UNCOUNT_PRELOADED).is_empty());
    }

    #[test]
    fn unload_skips_releases_while_loading_and_needs_a_loaded_item() {
        // Loading: only the settings are told.
        let (mut e, item) = states((0, 1), (1, 0));
        e.call(0x0040_b800, &args![item, 0u32]);
        assert!(calls(&e, MODEL_LOADER_RELEASE_MODEL).is_empty());
        assert_eq!(calls(&e, SETTING_UNLOAD_00408910).len(), 1);
        assert_eq!(calls(&e, SETTING_UNLOAD_00408730).len(), 2);
        // Neither loading nor loaded: nothing.
        let (mut e, item) = states((0, 0), (1, 0));
        e.call(0x0040_b800, &args![item, 0u32]);
        assert!(calls(&e, SETTING_UNLOAD_00408910).is_empty());
        assert!(calls(&e, SETTING_UNLOAD_00408730).is_empty());
    }

    #[test]
    fn unload_releases_biped_and_default_models_and_uncounts() {
        let (mut e, item) = states((1, 0), (1, 0));
        let (first, second) = settings_of(&e, item);
        // The 0x20000 arm on the first setting: two biped models.
        e.mem.set_u32(first + 8, 0x0002_0000);
        e.mem.set_u32(first + 0x60, 0x7000);
        e.register(BIPED_MODEL_BY_INDEX, |_, a| (a[0] + a[1]).into_ret());
        // The 0x40000 arm on the second: a default model list of two paths; the second
        // cast finds nothing. Telling the setting (00408730) drops its counter, so it
        // is uncounted.
        e.mem.set_u32(second + 8, 0x0004_0000);
        e.mem.set_u32(second + 0x60, 0x7100);
        e.register(SETTING_UNLOAD_00408730, |e, a| {
            e.mem.set_u8(a[0] + 0x32, 0);
            Ret::default()
        });
        let list = e.mem.alloc(8);
        let tail = e.mem.alloc(8);
        e.mem.set_u32(list, 0x7200);
        e.mem.set_u32(list + 4, tail);
        e.mem.set_u32(tail, 0x7300);
        answer(&mut e, BUILD_DEFAULT_MODEL_LIST, list);
        e.register(RT_DYNAMIC_CAST, |_, a| {
            (if a[3] == RTTI_CAST_FOR_40000_SECOND {
                0
            } else {
                a[0]
            })
            .into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x0040_b800, &args![item, 0u32]);
        let released: Vec<u32> = calls(&e, MODEL_LOADER_RELEASE_MODEL)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(released, vec![0x104, 0x7070, 0x7071, 0x7200, 0x7300]);
        assert_eq!(
            calls(&e, BUILD_DEFAULT_MODEL_LIST),
            vec![vec![0x7100, 1, 1]]
        );
        assert_eq!(calls(&e, FILENAME_LIST_DESTROY), vec![vec![list, 1]]);
        assert_eq!(calls(&e, UNCOUNT_PRELOADED).len(), 1);
    }

    #[test]
    fn magic_item_init_item_counts_hostile_effects() {
        let mut e = rig();
        answer(&mut e, EFFECT_ITEM_INIT_ITEM, 0);
        e.register(EFFECT_ITEM_IS_HOSTILE, |e, a| {
            u32::from(e.mem.u8(a[0] + 0x20)).into_ret()
        });
        let a = effect_item(&mut e, 0, 0, 0.0);
        let b = effect_item(&mut e, 0, 0, 0.0);
        e.mem.set_u8(b + 0x20, 1);
        let item = magic_item(&mut e, &[a, 0, b]);
        e.mem.set_u32(item.addr() + 0x18, 9);
        e.call(0x0040_c030, &args![item, 0x5000u32]);
        assert_eq!(e.mem.u32(item.addr() + 0x18), 1);
        assert_eq!(
            calls(&e, EFFECT_ITEM_INIT_ITEM),
            vec![vec![a, 0x5000], vec![b, 0x5000]]
        );
    }
}

#[cfg(test)]
mod finished_loading_tests {
    use super::*;

    const LOADER: u32 = 0x00c0_ffee;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn put_text(e: &mut Engine, text: &str) -> u32 {
        let address = e.mem.alloc(text.len() as u32 + 1);
        e.mem.write(address, text.as_bytes());
        e.mem.set_u8(address + text.len() as u32, 0);
        address
    }

    fn text(e: &Engine, address: u32) -> String {
        String::from_utf8(e.mem.cstr(address)).unwrap()
    }

    /// A list of nodes (item word, next word) holding `items`.
    fn node_list(e: &mut Engine, items: &[u32]) -> u32 {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, *item);
            e.mem.set_u32(node + 4, next);
            next = node;
        }
        next
    }

    /// A `TESModel`-like object whose vtable slot +0x14 answers `path`.
    fn model_with_path(e: &mut Engine, at: u32, path: u32) {
        let vtable = e.mem.alloc(0x20);
        let function = 0x00f0_0000 + (at & 0xffff);
        e.mem.set_u32(vtable + 0x14, function);
        e.mem.set_u32(at, vtable);
        e.register_double(function, move |_, _| path.into_ret());
    }

    /// The engine with the list helpers, the loader singleton and the model pointer
    /// doubles: `FindModel` stores a model (0x7000 plus the path's low 12 bits) whose
    /// first word is the path.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.set_global(MODEL_LOADER, LOADER);
        e.register(LIST_NODE_ITEM_SLOT, |_, a| a[0].into_ret());
        e.register(LIST_NODE_NEXT, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(LIST_NODE_IS_EMPTY, |e, a| {
            u32::from(e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0).into_ret()
        });
        e.register(MODEL_POINTER_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(MODEL_POINTER_DESTRUCT, |_, _| Ret::default());
        e.register(WORD_AT_THIS_00559450, |e, a| e.mem.u32(a[0]).into_ret());
        e.register(MODEL_LOADER_FIND_MODEL, |e, a| {
            // The model keeps a copy of the path (the caller's buffer changes).
            let path = e.mem.cstr(a[1]);
            let copy = e.mem.alloc(path.len() as u32 + 1);
            e.mem.write(copy, &path);
            e.mem.set_u8(copy + path.len() as u32, 0);
            let model = e.mem.alloc(8);
            e.mem.set_u32(model, copy);
            e.mem.set_u32(a[2], model);
            Ret::default()
        });
        e.register(MODEL_MOD_MANUAL_REF_COUNT, |_, _| Ret::default());
        e.register(MODEL_PATH_LENGTH, |e, a| {
            u32::from(e.mem.u32(a[0] + 4) != 0).into_ret()
        });
        e.register(EFFECT_ITEM_SETTING, |e, a| {
            e.mem.u32(a[0] + 0x14).into_ret()
        });
        e.register(EFFECT_SETTING_ASSOCIATED_FORM, |e, a| {
            e.mem.u32(a[0] + 0x60).into_ret()
        });
        e.register(FORM_HAS_FLAG, |e, a| {
            u32::from(e.mem.u32(a[0]) & a[1] != 0).into_ret()
        });
        e.register(EFFECT_SETTING_IS_ASSOCIATED_ITEM_QUEUED, |e, a| {
            u32::from(e.mem.u32(a[0] + 0xac) != 0).into_ret()
        });
        e.register(SETTING_ASSOCIATED_COUNT_004087E0, |e, a| {
            e.mem.u32(a[0] + 0xac).into_ret()
        });
        e.register(MAGIC_ITEM_IS_LOADING, |_, _| 1u32.into_ret());
        e
    }

    /// A magic item whose list at +0xc holds one effect item with a setting of
    /// `flags`, `count` queued references and the associated form `form`.
    fn item_with_setting(e: &mut Engine, flags: u32, count: u32, form: u32) -> Ptr {
        let setting = e.mem.alloc(0xb0);
        e.mem.set_u32(setting, flags);
        e.mem.set_u32(setting + 0x60, form);
        e.mem.set_u32(setting + 0xac, count);
        let effect_item = e.mem.alloc(0x24);
        e.mem.set_u32(effect_item + 0x14, setting);
        let item = e.mem.alloc(0x20);
        e.mem.set_u32(item + 0x10, effect_item);
        Ptr::new(item)
    }

    fn model_references(e: &Engine) -> Vec<(String, u32)> {
        calls(e, MODEL_MOD_MANUAL_REF_COUNT)
            .into_iter()
            .map(|w| (text(e, e.mem.u32(w[0])), w[1]))
            .collect()
    }

    #[test]
    fn not_loading_preloads_and_unloads() {
        let mut e = engine();
        e.register(MAGIC_ITEM_IS_LOADING, |_, _| Ret::default());
        e.register(MAGIC_ITEM_GET_AV_EFFECT, |_, _| Ret::default());
        let item: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.call_log = Some(vec![]);
        e.call(0x0040_ab70, &args![item]);
        assert!(calls(&e, MODEL_LOADER_FIND_MODEL).is_empty());
        // `Preload` and `Unload` each looked at the (empty) list.
        assert!(calls(&e, LIST_NODE_IS_EMPTY).len() >= 2);
    }

    #[test]
    fn effect_model_gets_the_queued_count() {
        let mut e = engine();
        let effect = e.mem.alloc(0xb0);
        let path = put_text(&mut e, "effect.nif");
        e.mem.set_u32(effect + 0x1c, 1);
        model_with_path(&mut e, effect + 0x18, path);
        e.register_double(MAGIC_ITEM_GET_AV_EFFECT, move |_, _| effect.into_ret());
        e.register(EFFECT_SETTING_IS_EFFECT_QUEUED, |_, _| 1u32.into_ret());
        e.register(SETTING_EFFECT_COUNT_004089C0, |_, _| 3u32.into_ret());
        let item: Ptr = Ptr::new(e.mem.alloc(0x20));
        e.call_log = Some(vec![]);
        magic_item_finished_loading(&mut e, item);
        assert_eq!(model_references(&e), vec![("effect.nif".to_string(), 3)]);
        assert_eq!(calls(&e, MODEL_LOADER_FIND_MODEL)[0][0], LOADER);
        // Not queued: the count is 0.
        e.register(EFFECT_SETTING_IS_EFFECT_QUEUED, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        magic_item_finished_loading(&mut e, item);
        assert_eq!(model_references(&e), vec![("effect.nif".to_string(), 0)]);
    }

    #[test]
    fn associated_model_and_biped_models() {
        let mut e = engine();
        e.register(MAGIC_ITEM_GET_AV_EFFECT, |_, _| Ret::default());
        e.register(RT_DYNAMIC_CAST, |_, a| a[0].into_ret());
        let form = e.mem.alloc(0x100);
        let path = put_text(&mut e, "weapon.nif");
        e.mem.set_u32(form + 0x40, 1);
        model_with_path(&mut e, form + 0x3c, path);
        let item = item_with_setting(&mut e, 0x0001_0000, 2, form);
        e.call_log = Some(vec![]);
        magic_item_finished_loading(&mut e, item);
        assert_eq!(model_references(&e), vec![("weapon.nif".to_string(), 2)]);
        assert_eq!(calls(&e, RT_DYNAMIC_CAST)[0][3], RTTI_CAST_FOR_10000);

        let male = put_text(&mut e, "male.nif");
        let female = put_text(&mut e, "female.nif");
        e.register_double(BIPED_MODEL_BY_INDEX, move |_, a| {
            if a[1] == 0 { male } else { female }.into_ret()
        });
        let item = item_with_setting(&mut e, 0x0002_0000, 4, form);
        e.call_log = Some(vec![]);
        magic_item_finished_loading(&mut e, item);
        assert_eq!(
            model_references(&e),
            vec![("male.nif".to_string(), 4), ("female.nif".to_string(), 4)]
        );
        assert_eq!(calls(&e, BIPED_MODEL_BY_INDEX)[0], vec![form + 0x70, 0]);
        // Without a queued count nothing is looked at.
        let item = item_with_setting(&mut e, 0x0002_0000, 0, form);
        e.call_log = Some(vec![]);
        magic_item_finished_loading(&mut e, item);
        assert!(calls(&e, RT_DYNAMIC_CAST).is_empty());
    }

    #[test]
    fn default_models_and_animations() {
        let mut e = engine();
        e.register(MAGIC_ITEM_GET_AV_EFFECT, |_, _| Ret::default());
        let creature = e.mem.alloc(0x200);
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            if a[3] == RTTI_CAST_FOR_40000_FIRST {
                Ret::default()
            } else {
                creature.into_ret()
            }
        });
        let model_path = put_text(&mut e, "creatures\\dog\\Skeleton.nif");
        model_with_path(&mut e, creature + 0xdc, model_path);
        let extra = put_text(&mut e, "head.nif");
        let names = node_list(&mut e, &[extra, 0]);
        e.mem.set_u32(creature + 0x118, names);
        let special = put_text(&mut e, "idle.kf");
        let special_list = node_list(&mut e, &[special]);
        e.mem.set_u32(creature + 0xc8, special_list);
        e.register(MEMBER_LIST_AT_4, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(MODEL_0092C870, |_, _| Ret::default());
        e.register(STRING_COPY, |e, a| {
            let source = e.mem.cstr(a[2]);
            e.mem.write(a[0], &source);
            e.mem.set_u8(a[0] + source.len() as u32, 0);
            Ret::default()
        });
        e.register(STRING_APPEND, |e, a| {
            let start = a[0] + e.mem.cstr(a[0]).len() as u32;
            let source = e.mem.cstr(a[2]);
            e.mem.write(start, &source);
            e.mem.set_u8(start + source.len() as u32, 0);
            Ret::default()
        });
        e.register(CRT_STRRCHR, |e, a| {
            let bytes = e.mem.cstr(a[0]);
            match bytes.iter().rposition(|b| u32::from(*b) == a[1]) {
                Some(i) => (a[0] + i as u32).into_ret(),
                None => Ret::default(),
            }
        });
        e.register(CRT_STRNICMP, |e, a| {
            let left = e.mem.cstr(a[0]);
            let right = e.mem.cstr(a[1]);
            let n = a[2] as usize;
            let same = left
                .iter()
                .take(n)
                .map(u8::to_ascii_lowercase)
                .eq(right.iter().take(n).map(u8::to_ascii_lowercase));
            u32::from(!same).into_ret()
        });
        let kf = put_text(&mut e, "skeleton.kf");
        let kf_list = node_list(&mut e, &[kf]);
        e.register_double(MODEL_LOADER_BUILD_KF_LIST, move |_, _| kf_list.into_ret());
        e.register(FREE_BLOCK, |_, _| Ret::default());
        e.register(FILENAME_LIST_CLEAR, |_, _| Ret::default());
        e.register(FILENAME_LIST_DESTROY, |_, _| Ret::default());
        e.register(ANIMATION_HAS_KF_FILES, |_, _| 1u32.into_ret());
        e.map(0x0101_3000, 0x1000);
        e.mem.write(SPECIAL_ANIMS_TEXT, b"SpecialAnims\0");
        e.mem.write(SPECIAL_ANIMS_SEPARATOR_TEXT, b"\\\0");
        e.mem.write(SKELETON_TEXT, b"Skeleton\0");
        let loaded = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let seen = loaded.clone();
        e.register_double(MODEL_LOADER_LOAD_KF, move |e, a| {
            seen.borrow_mut().push(text(e, a[1]));
            Ret::default()
        });
        let item = item_with_setting(&mut e, 0x0004_0000, 2, creature);
        e.call_log = Some(vec![]);
        e.call(0x0040_ab70, &args![item]);
        assert_eq!(
            model_references(&e),
            vec![("creatures\\dog\\head.nif".to_string(), 2)]
        );
        assert_eq!(calls(&e, MODEL_0092C870).len(), 2);
        assert_eq!(
            *loaded.borrow(),
            vec![
                "skeleton.kf".to_string(),
                "skeleton.kf".to_string(),
                "creatures\\dog\\SpecialAnims\\idle.kf".to_string(),
                "creatures\\dog\\SpecialAnims\\idle.kf".to_string(),
            ]
        );
        assert_eq!(calls(&e, FREE_BLOCK), vec![vec![kf]]);
        let built = calls(&e, MODEL_LOADER_BUILD_KF_LIST);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][0], LOADER);
        assert_eq!(text(&e, built[0][1]), "creatures\\dog\\Skeleton.nif");
        assert_eq!(built[0][2..], [0, 0, 0]);
    }

    #[test]
    fn numeric_id_lookup() {
        let mut e = Engine::new();
        e.register(LOOK_UP_FORM_004839C0, |_, a| {
            if a[0] == 0 { 0 } else { a[0] + 0x1000 }.into_ret()
        });
        e.register(RT_DYNAMIC_CAST, |_, a| {
            let wanted = match a[0] & 0xf {
                1 => MAGIC_ITEM_FORM_TYPE,
                2 => MAGIC_ITEM_OBJECT_TYPE,
                _ => 0,
            };
            u32::from(a[3] == wanted).wrapping_mul(a[0]).into_ret()
        });
        assert_eq!(e.call(0x0040_a250, &args![0u32]).u32(), 0);
        assert_eq!(e.call(0x0040_a250, &args![0x101u32]).u32(), 0x1101 + 0x18);
        assert_eq!(
            magic_item_get_magic_item_by_numeric_id(&mut e, 0x102),
            0x1102 + 0x30
        );
        assert_eq!(magic_item_get_magic_item_by_numeric_id(&mut e, 0x103), 0);
    }

    #[test]
    fn preloaded_counter() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.set_global(PRELOADED_COUNT, 0xffff_ffffu32);
        e.call(0x0040_ab50, &args![]);
        assert_eq!(e.mem.u32(PRELOADED_COUNT), 0);
        e.call(0x0040_bed0, &args![]);
        e.call(0x0040_bed0, &args![]);
        assert_eq!(e.mem.u32(PRELOADED_COUNT), 0xffff_fffe);
    }

    #[test]
    fn type_text_by_kind() {
        let mut e = Engine::new();
        e.register(SETTING_VALUE_00403DF0, |_, a| (a[0] + 4).into_ret());
        e.register(STRING_CONSTRUCT_0040C0E0, |_, _| Ret::default());
        let cases = [
            (1, TYPE_TEXT_SETTING_1),
            (2, TYPE_TEXT_SETTING_2),
            (3, TYPE_TEXT_SETTING_3),
            (4, TYPE_TEXT_SETTING_4),
            (5, TYPE_TEXT_SETTING_5),
            (6, TYPE_TEXT_SETTING_6),
            (0, TYPE_TEXT_SETTING_OTHER),
            (7, TYPE_TEXT_SETTING_OTHER),
        ];
        for (kind, setting) in cases {
            e.call_log = Some(vec![]);
            let result = e
                .call(0x0040_bef0, &args![0x5000u32, 0x6000u32, kind])
                .u32();
            assert_eq!(result, 0x6000);
            assert_eq!(calls(&e, SETTING_VALUE_00403DF0), vec![vec![setting]]);
            assert_eq!(
                calls(&e, STRING_CONSTRUCT_0040C0E0),
                vec![vec![0x6000, setting + 4]]
            );
        }
    }
}
