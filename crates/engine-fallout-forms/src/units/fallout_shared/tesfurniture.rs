//! `fallout shared/tesfurniture.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESFurniture` is a `TESObjectACTI` whose own field is the marker flag
//! word at +0x90 (`iFurnFlags` in the Xbox PDB, +0xA0 there; every field
//! after `TESForm` sits 0x10 lower on PC). Bit `n` of the word enables
//! furniture marker `n`; bit 30 means the furniture can be sat on and bit 31
//! that it can be slept on.
//!
//! The unit's address range also holds a few small members of other
//! classes, which the engine map files here by position only
//! (`00509490`, `005094b0`, `005094d0`, `00509890`, `005098c0`, `005098e0`).
//! They are translated as `fn_<address>` and described by what the body
//! does, because the exe names no class for them.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESForm::LoadForm` (`00485110`, one stack word: the file).
const LOAD_FORM: u32 = 0x0048_5110;
/// `00484ab0` (one stack word), called with 0 by the loader and with 1 by
/// `InitItem`; the Xbox PDB name is not given by the engine map.
const SET_FORM_FLAG_CHANGED: u32 = 0x0048_4ab0;
/// `TESFile::GetTESChunk`-style accessor: the type of the current chunk
/// (`004726b0`, 0 at the end of the record).
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
/// Advances the file to the next chunk (`004726f0`), true while there is one.
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
/// Size of the current chunk (`00401660`).
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
/// Reads chunk data into a buffer (`00472890`: buffer, byte count).
const FILE_GET_CHUNK_DATA: u32 = 0x0047_2890;
/// Reads chunk data into a four-byte variable (`004727f0`: its address).
const FILE_GET_CHUNK_DWORD: u32 = 0x0047_27f0;
/// The model-texture-swap count of the current chunk, a `u16` in AX
/// (`00403570`).
const FILE_CHUNK_SWAP_COUNT: u32 = 0x0040_3570;
/// `TESFullName` chunk loader (`00487050`: component, file; cdecl).
const FULL_NAME_LOAD_CHUNK: u32 = 0x0048_7050;
/// `TESModel::LoadModelChunk` (`004892d0`: component, file; cdecl).
const MODEL_LOAD_CHUNK: u32 = 0x0048_92d0;
/// `TESModelTextureSwap::UnpackTextureSwapChunkData` (`0048a7b0`: data,
/// size, count).
const MODEL_UNPACK_TEXTURE_SWAPS: u32 = 0x0048_a7b0;
/// `BGSDestructibleObjectForm::LoadChunk` (`004781e0`: component, file;
/// cdecl).
const DESTRUCTIBLE_LOAD_CHUNK: u32 = 0x0047_81e0;
/// `TESScriptableForm`'s script-reference setter (`006ecd40`, one stack
/// word) and its `InitItem` (`0048cd90`, one stack word: the form).
const SCRIPTABLE_SET_SCRIPT: u32 = 0x006e_cd40;
const SCRIPTABLE_INIT_ITEM: u32 = 0x0048_cd90;
/// `BGSDestructibleObjectForm::InitItem` (`00478be0`, one stack word).
const DESTRUCTIBLE_INIT_ITEM: u32 = 0x0047_8be0;

/// `TESForm::StartForm` (`004855a0`) and `TESForm::CloseForm` (`00485680`).
const START_FORM: u32 = 0x0048_55a0;
const CLOSE_FORM: u32 = 0x0048_5680;
/// The save functions of the components (all `this`-only): `TESFullName`
/// (`00487010`), `TESScriptableForm` (`0048cca0`) and
/// `BGSDestructibleObjectForm` (`00477f20`).
const FULL_NAME_SAVE: u32 = 0x0048_7010;
const SCRIPTABLE_SAVE: u32 = 0x0048_cca0;
const DESTRUCTIBLE_SAVE: u32 = 0x0047_7f20;
/// `TESModelTextureSwap::Save` (`0048a520`: the three chunk tags it
/// writes, model, texture hashes and swaps).
const MODEL_SAVE: u32 = 0x0048_a520;
/// Writes one four-byte chunk (`00485910`: tag, value; cdecl).
const WRITE_DWORD_CHUNK: u32 = 0x0048_5910;

/// `TESForm::CopyAllComponents` (`004851b0`, one stack word) and
/// `TESForm::CompareAllComponents` (`00485270`, one stack word).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// `__RTDynamicCast` (`00ec43fb`).
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Type descriptors `.?AVTESForm@@` and `.?AVTESFurniture@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
const TYPE_TES_FURNITURE: u32 = 0x0118_a220;
/// Type descriptors `.?AVTESObjectREFR@@` and `.?AVPlayerCharacter@@`.
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
const TYPE_PLAYER_CHARACTER: u32 = 0x0118_a23c;

/// `bool` flag tests on `TESForm::iFormFlags` (+0x08): bit 3 (`004013e0`)
/// and bit 14 (`004077c0`).
const FORM_FLAG_TEST_BIT_3: u32 = 0x0040_13e0;
const FORM_FLAG_TEST_BIT_14: u32 = 0x0040_77c0;
/// `BSFurnitureMarker::Find` (`00c54400`, cdecl: the model's marker data).
const FURNITURE_MARKER_FIND: u32 = 0x00c5_4400;
/// The script console print (`005b5e40`, cdecl, printf-style).
const PRINT_FORMATTED: u32 = 0x005b_5e40;
/// `"MODELS: No FurnitureMarkers found in '%s'."` and
/// `"MODELS: You have not selected any FurnitureMarkers for '%s'."`.
const NO_MARKERS_FORMAT: u32 = 0x0102_6ecc;
const NO_MARKERS_SELECTED_FORMAT: u32 = 0x0102_6e8c;

/// The vtable slots the furniture code uses (byte offsets).
const SLOT_LOAD_OBJECT_BOUND: u32 = 0xe0;
const SLOT_GET_NAME_STRING: u32 = 0x130;
const SLOT_SET_EDITOR_ID: u32 = 0x134;
const SLOT_SET_MARKER_FLAGS: u32 = 0x150;
const SLOT_GET_MODEL_PATH: u32 = 0x178;
/// `TESModel`'s vtable slot that returns the model path (`+0x14`).
const MODEL_SLOT_GET_PATH: u32 = 0x14;

/// Chunk tags, as the little-endian words the file reader returns.
const CHUNK_MODL: u32 = 0x4c44_4f4d;
const CHUNK_MODT: u32 = 0x5444_4f4d;
const CHUNK_MODS: u32 = 0x5344_4f4d;
const CHUNK_MODD: u32 = 0x4444_4f4d;
/// `"MOSD"`.
const CHUNK_MOSD: u32 = 0x4453_4f4d;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_DSTD: u32 = 0x4454_5344;
const CHUNK_DEST: u32 = 0x5453_4544;
const CHUNK_SCRI: u32 = 0x4952_4353;
const CHUNK_FULL: u32 = 0x4c4c_5546;
const CHUNK_MNAM: u32 = 0x4d41_4e4d;

/// The player (`PlayerCharacter *`), `011dea3c`.
const PLAYER: u32 = 0x011d_ea3c;
/// Static object whose `+4` is the string shown by a pop-up message
/// (`00403df0` returns it): the five "cannot use" messages.
const MESSAGE_NOT_OWNER: u32 = 0x011d_2898;
const MESSAGE_PLAYER_DISABLED: u32 = 0x011d_20ac;
const MESSAGE_PLAYER_NOT_ALLOWED: u32 = 0x011d_421c;
const MESSAGE_AREA_BLOCKED: u32 = 0x011d_384c;
const MESSAGE_SLEEP_UNAVAILABLE: u32 = 0x011d_252c;
const MESSAGE_CANNOT_SLEEP: u32 = 0x011d_1e0c;
/// The object (`011e0e80`) whose members `009764a0` and `005098e0` test.
const SLEEP_STATE: u32 = 0x011e_0e80;
/// `"Interface\Icons\Message Icons\glow_message_vaultboy_sad.dds"`.
const MESSAGE_ICON_PATH: u32 = 0x0102_08a0;
/// The `float` (2.0) passed as the pop-up's display time.
const MESSAGE_DISPLAY_TIME: u32 = 0x0101_62c0;
/// `1.0` as a `double`.
const ONE: u32 = 0x0101_2070;
/// The default marker offset (three `float`s) of `GetMarkerTargetOffset`.
const DEFAULT_MARKER_OFFSET: u32 = 0x011f_426c;
/// The table, 16 bytes per entry, whose first three words point to the
/// settings holding each marker type's x, y and z offset.
const MARKER_OFFSET_TABLE: u32 = 0x0118_9fa0;
/// `00403e20`: the `float` value of a setting (its `+4`), or 0 for null.
const SETTING_FLOAT: u32 = 0x0040_3e20;
/// `00403df0`: the string value of a setting (its `+4`), or 0 for null.
const SETTING_STRING: u32 = 0x0040_3df0;

/// `TESObjectREFR::GetClosestFreeMarker` (`005686b0`, six stack words).
const GET_CLOSEST_FREE_MARKER: u32 = 0x0056_86b0;
/// `TESObjectREFR::GetOwner` (`00567790`) and `TESObjectREFR::IsAnOwner`
/// (`005785e0`: the actor, a flag).
const GET_OWNER: u32 = 0x0056_7790;
const IS_AN_OWNER: u32 = 0x0057_85e0;
/// `008d6f30` (reads the player's `+0x40`) and `005444c0` (a test on that
/// value).
const PLAYER_GET_SEATED_OBJECT: u32 = 0x008d_6f30;
const OBJECT_TEST_005444C0: u32 = 0x0054_44c0;
/// `TESObjectREFR::GetInterior` (`00575d10`, returns the interior flag).
const GET_INTERIOR: u32 = 0x0057_5d10;
/// `009764a0` (this = the sleep state object, one stack word) and
/// `00822e00` (this = player + 0x94).
const SLEEP_STATE_TEST: u32 = 0x0097_64a0;
const PLAYER_STATE_TEST: u32 = 0x0082_2e00;
/// The message pop-up (`007052f0`, cdecl, six stack words) and
/// `Interface::CreateSleepMenu` (`007054f0`, cdecl, one stack word).
const SHOW_MESSAGE: u32 = 0x0070_52f0;
const CREATE_SLEEP_MENU: u32 = 0x0070_54f0;
/// `PlayerCharacter::ForceTemp3rdPerson` (`00950340`, one stack word) and
/// `PlayerCharacter::InitiateSitSleepPackage` (`00953d80`, one stack word).
const FORCE_TEMP_THIRD_PERSON: u32 = 0x0095_0340;
const INITIATE_SIT_SLEEP_PACKAGE: u32 = 0x0095_3d80;
/// The `PlayerCharacter` vtable slots `Activate` calls (byte offsets).
const SLOT_PLAYER_IS_NPC_STATE: u32 = 0x214;
const SLOT_PLAYER_GET_UP: u32 = 0x418;
const SLOT_PLAYER_TRESPASSING: u32 = 0x448;
/// The activator's vtable slot `0x1f4` (PC body `008ae4c0`, no stack
/// arguments) that yields the first argument of `GetClosestFreeMarker`.
const SLOT_ACTIVATOR_MARKER_SOURCE: u32 = 0x1f4;
/// `006815c0`: returns `this` (identical-code folded; called by address).
const RETURN_THIS: u32 = 0x0068_15c0;
/// `009938b0` and `00558dd0`: the members at `this + 0xc` that
/// `005094xx` forward to.
const FORWARD_009938B0: u32 = 0x0099_38b0;
const FORWARD_00558DD0: u32 = 0x0055_8dd0;

layout! {
    /// `TESFurniture` (Xbox PDB), only the field this unit uses. The Xbox
    /// PDB size is 0xA4; the PC size is 0x94 (the 0x10 bytes of `TESForm`
    /// that PC lacks), not checked against an allocation in the exe.
    pub struct TESFurniture: 0x94 {
        /// `iFurnFlags` (Xbox PDB, +0xA0 there): the enabled-marker bits and
        /// the sit (bit 30) and sleep (bit 31) bits.
        0x90 iFurnFlags: u32,
    }
}

/// The address of the `float` value of a setting (`setting + 4`, or the
/// exe's static zero for a null setting) through `00403e20`.
fn setting_float_address(e: &mut Engine, setting: u32) -> u32 {
    e.call(SETTING_FLOAT, &args![setting]).u32()
}

/// `member` pointer of `base`, null when `base` is null (the compiler's
/// null test before taking the address of a base subobject).
fn member(base: u32, offset: u32) -> u32 {
    if base == 0 {
        0
    } else {
        base + offset
    }
}

/// `__RTDynamicCast(object, 0, from, to, 0)`.
fn dynamic_cast(e: &mut Engine, object: u32, from: u32, to: u32) -> u32 {
    e.call(RT_DYNAMIC_CAST, &args![object, 0u32, from, to, 0u32])
        .u32()
}

// Translated from 00508f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFurniture::InitItem` (Xbox PDB): unless the form is flagged (bit 3 of
/// the form flags), runs `BGSDestructibleObjectForm::InitItem` on the
/// component at +0x68 with the form, verifies the markers and marks the form
/// (`00484ab0` with 1).
pub fn tes_furniture_init_item(e: &mut Engine, this: Ptr<TESFurniture>) {
    if e.call(FORM_FLAG_TEST_BIT_3, &args![this]).bool() {
        return;
    }
    e.call(DESTRUCTIBLE_INIT_ITEM, &args![this.addr() + 0x68, this]);
    tes_furniture_verify_markers(e, this);
    e.call(SET_FORM_FLAG_CHANGED, &args![this, 1u32]);
}

// Translated from 00508fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The furniture's form loader (vtable slot `0x20`; the engine map names
/// nothing here): `TESForm::LoadForm`, then reads every chunk of the record
/// and hands it to its component: the model chunks (`MODL`, `MODT`, `MODD`,
/// `MOSD`) to the model at +0x3c, `MODS` to its texture-swap unpacking,
/// `FULL` to the name component at +0x30, `DSTD`/`DEST` to the destructible
/// component at +0x68, `SCRI` to the script component at +0x5c, `EDID` to
/// vtable slot `0x134` with the (at most 0x200 bytes) text, `OBND` to slot
/// `0xe0`, and `MNAM` into the marker flags at +0x90. Returns true.
///
/// The `alloca` buffers of `EDID` and `MODS` are allocated in the engine's
/// heap and released at the end. The compiler's stack cookie check is not
/// translated.
// Translation incomplete: 00508fd0 - the stack security cookie (/GS) check 00ec408c is not translated.
pub fn fn_00508fd0(e: &mut Engine, this: Ptr<TESFurniture>, file: Ptr) -> bool {
    let base = this.addr();
    let mut buffers: Vec<u32> = Vec::new();
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_CHANGED, &args![this, 0u32]);
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_MODL | CHUNK_MOSD | CHUNK_MODD | CHUNK_MODT => {
                e.call(MODEL_LOAD_CHUNK, &args![member(base, 0x3c), file]);
            }
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                buffers.push(buffer);
                e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, 0x200u32]);
                e.vcall(base, SLOT_SET_EDITOR_ID, &args![buffer]);
            }
            CHUNK_OBND => {
                e.vcall(base, SLOT_LOAD_OBJECT_BOUND, &args![file]);
            }
            CHUNK_FULL => {
                e.call(FULL_NAME_LOAD_CHUNK, &args![member(base, 0x30), file]);
            }
            CHUNK_MODS => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                let buffer = e.mem.alloc(size);
                buffers.push(buffer);
                e.call(FILE_GET_CHUNK_DATA, &args![file, buffer, size]);
                let count = e.call(FILE_CHUNK_SWAP_COUNT, &args![file]).u32() & 0xffff;
                e.call(
                    MODEL_UNPACK_TEXTURE_SWAPS,
                    &args![base + 0x3c, buffer, size, count],
                );
            }
            CHUNK_SCRI => {
                let script = e.mem.alloc(4);
                buffers.push(script);
                e.mem.set_u32(script, 0);
                e.call(FILE_GET_CHUNK_DWORD, &args![file, script]);
                let value = e.mem.u32(script);
                e.call(SCRIPTABLE_SET_SCRIPT, &args![base + 0x5c, value]);
                e.call(SCRIPTABLE_INIT_ITEM, &args![base + 0x5c, this]);
            }
            CHUNK_DSTD | CHUNK_DEST => {
                e.call(DESTRUCTIBLE_LOAD_CHUNK, &args![member(base, 0x68), file]);
            }
            CHUNK_MNAM => {
                e.call(FILE_GET_CHUNK_DWORD, &args![file, base + 0x90]);
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    for buffer in buffers {
        e.mem.free(buffer);
    }
    true
}

// Translated from 00509280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The furniture's form writer (vtable slot `0x2c`; the engine map names
/// nothing here): starts the form, saves the name (+0x30), the model with its
/// `MODL`/`MODT`/`MODS` chunks (+0x3c), the script component (+0x5c) and the
/// destructible component (+0x68), writes the marker flags as an `MNAM`
/// chunk and closes the form.
pub fn fn_00509280(e: &mut Engine, this: Ptr<TESFurniture>) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    e.call(FULL_NAME_SAVE, &args![base + 0x30]);
    e.call(
        MODEL_SAVE,
        &args![base + 0x3c, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS],
    );
    e.call(SCRIPTABLE_SAVE, &args![base + 0x5c]);
    e.call(DESTRUCTIBLE_SAVE, &args![base + 0x68]);
    let flags = e.get(this, TESFurniture::iFurnFlags);
    e.call(WRITE_DWORD_CHUNK, &args![CHUNK_MNAM, flags]);
    e.call(CLOSE_FORM, &args![this]);
}

// Translated from 005092f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFurniture::Copy` (Xbox PDB): when the source form casts to a
/// `TESFurniture`, copies all components (`004851b0` with the source as
/// given), copies the marker flags and verifies the markers.
pub fn tes_furniture_copy(e: &mut Engine, this: Ptr<TESFurniture>, source: Ptr) {
    let furniture = dynamic_cast(e, source.addr(), TYPE_TES_FORM, TYPE_TES_FURNITURE);
    if furniture == 0 {
        return;
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, source]);
    let flags = e.mem.u32(furniture + 0x90);
    e.set(this, TESFurniture::iFurnFlags, flags);
    tes_furniture_verify_markers(e, this);
}

// Translated from 00509350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFurniture::VerifyMarkers` (Xbox PDB): for a form that is not flagged
/// (bit 14 of the form flags) and has no marker enabled, looks up the
/// furniture markers of the model (vtable slot `0x178` with 0, then
/// `BSFurnitureMarker::Find`); prints "MODELS: No FurnitureMarkers found in
/// '%s'." with the model path (slot `0x14` of the model at +0x3c) when there
/// are none, or "MODELS: You have not selected any FurnitureMarkers for
/// '%s'." with the form's name string (slot `0x130`) when there are some, and
/// calls vtable slot `0x150` with 0.
pub fn tes_furniture_verify_markers(e: &mut Engine, this: Ptr<TESFurniture>) {
    if e.call(FORM_FLAG_TEST_BIT_14, &args![this]).bool() {
        return;
    }
    if e.get(this, TESFurniture::iFurnFlags) != 0 {
        return;
    }
    let model_data = e
        .vcall(this.addr(), SLOT_GET_MODEL_PATH, &args![0u32])
        .u32();
    let markers = e.call(FURNITURE_MARKER_FIND, &args![model_data]).u32();
    if markers == 0 {
        let model = this.addr() + 0x3c;
        let path = e.vcall(model, MODEL_SLOT_GET_PATH, &args![]).u32();
        e.call(PRINT_FORMATTED, &args![NO_MARKERS_FORMAT, path]);
    } else {
        let name = e.vcall(this.addr(), SLOT_GET_NAME_STRING, &args![]).u32();
        e.call(PRINT_FORMATTED, &args![NO_MARKERS_SELECTED_FORMAT, name]);
    }
    e.vcall(this.addr(), SLOT_SET_MARKER_FLAGS, &args![0u32]);
}

// Translated from 005093f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFurniture::CanSitOn` (Xbox PDB): bit 30 of the marker flags.
pub fn tes_furniture_can_sit_on(e: &mut Engine, this: Ptr<TESFurniture>) -> bool {
    e.get(this, TESFurniture::iFurnFlags) & 0x4000_0000 != 0
}

// Translated from 00509420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFurniture::CanSleepOn` (Xbox PDB): bit 31 of the marker flags.
pub fn tes_furniture_can_sleep_on(e: &mut Engine, this: Ptr<TESFurniture>) -> bool {
    e.get(this, TESFurniture::iFurnFlags) & 0x8000_0000 != 0
}

// Translated from 00509450 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFurniture::GetMarkerEnabled` (Xbox PDB): whether bit `marker` of the
/// marker flags is set; false for `marker >= 32`.
pub fn tes_furniture_get_marker_enabled(
    e: &mut Engine,
    this: Ptr<TESFurniture>,
    marker: u32,
) -> bool {
    marker < 0x20 && e.get(this, TESFurniture::iFurnFlags) & (1 << marker) != 0
}

// Translated from 00509490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards to `009938b0` on the member at `this + 0xc`. The exe names no
/// class for this body; the callers are the furniture-marker code
/// (`GetClosestFreeMarker`, `GetMarkerAtIndex`, `FinishInitLoadGame`). Answers
/// `009938b0`'s value: the exe ends on that call and leaves its `EAX`, which
/// those callers read.
pub fn fn_00509490(e: &mut Engine, this: Ptr) -> u16 {
    e.call(FORWARD_009938B0, &args![this.addr() + 0xc]).u16()
}

// Translated from 005094b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the byte at `this + 0xe` (the marker type the structure built by
/// `00509890` holds; no class name in the exe).
pub fn fn_005094b0(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr() + 0xe)
}

// Translated from 005094d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards its stack word to `00558dd0` on the member at `this + 0xc` and
/// answers its value (the exe ends on that call and leaves its `EAX`, which
/// the callers use as the marker).
pub fn fn_005094d0(e: &mut Engine, this: Ptr, argument: u32) -> Ptr {
    e.call(FORWARD_00558DD0, &args![this.addr() + 0xc, argument])
        .ptr()
}

// Translated from 005094f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFurniture::IsSleepMarker` (Xbox PDB; static): marker types below 10.
pub fn tes_furniture_is_sleep_marker(_e: &mut Engine, marker: u32) -> bool {
    marker < 10
}

// Translated from 00509510 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFurniture::IsSitMarker` (Xbox PDB; static): marker types 10 to 20 and
/// 26.
pub fn tes_furniture_is_sit_marker(_e: &mut Engine, marker: u32) -> bool {
    (10..=20).contains(&marker) || marker == 26
}

// Translated from 00509540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The furniture's form comparison (vtable slot `0x10c`; the engine map names
/// nothing here): 1 ("differs") when `other` is not a `TESFurniture`, when
/// `CompareAllComponents` differs or when the marker flags differ; else 0.
pub fn fn_00509540(e: &mut Engine, this: Ptr<TESFurniture>, other: Ptr) -> bool {
    let furniture = dynamic_cast(e, other.addr(), TYPE_TES_FORM, TYPE_TES_FURNITURE);
    if furniture == 0 {
        return true;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, furniture])
        .bool()
    {
        return true;
    }
    e.get(this, TESFurniture::iFurnFlags) != e.mem.u32(furniture + 0x90)
}

/// Shows the pop-up with the sad Vault Boy icon and the text of the setting
/// object `message`, for the display time stored in the exe.
fn show_sad_message(e: &mut Engine, message: u32) {
    let text = e.call(SETTING_STRING, &args![message]).u32();
    let display_time: f32 = e.global(MESSAGE_DISPLAY_TIME);
    e.call(
        SHOW_MESSAGE,
        &args![text, 0u32, MESSAGE_ICON_PATH, 0u32, display_time, 0u32],
    );
}

// Translated from 005095b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFurniture::Activate` (Xbox PDB). `target` is the furniture reference
/// and `activator` the activating reference; the three further stack words
/// are never read. Returns false when the activator is not the player or
/// when no free marker was found or the player may not use the furniture;
/// true otherwise.
///
/// - Player already in a furniture state (slot `0x214` non-zero): forces a
///   temporary third-person camera, clears the float at the player's
///   `+0x6e4` (`005098c0`) and calls slot `0x418`.
/// - Otherwise it asks the target for the closest free marker
///   (`GetClosestFreeMarker` with the activator's slot `0x1f4` result and a
///   marker-description structure built by `00509890`). A sit marker
///   (anything not below 10) forces the third-person camera, clears the
///   float and starts the sit package; a sleep marker is first checked for an
///   owner (`GetOwner` / `IsAnOwner`: a foreign bed refuses with the sad
///   message), the player's own states (`005444c0`, slot `0x448`, the
///   interior and sleep-state tests, `005098e0`, `00822e00`) and opens the
///   sleep menu when nothing objects; each objection shows the sad-face
///   message and still returns true, except the ownership refusals, which
///   return false.
pub fn tes_furniture_activate(
    e: &mut Engine,
    _this: Ptr<TESFurniture>,
    target: Ptr,
    activator: Ptr,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
) -> bool {
    let player_character = dynamic_cast(
        e,
        activator.addr(),
        TYPE_TES_OBJECT_REFR,
        TYPE_PLAYER_CHARACTER,
    );
    if player_character == 0 {
        return false;
    }
    if e.vcall(player_character, SLOT_PLAYER_IS_NPC_STATE, &args![])
        .u32()
        != 0
    {
        e.call(FORCE_TEMP_THIRD_PERSON, &args![player_character, 1u32]);
        fn_005098c0(e, Ptr::new(player_character));
        e.vcall(player_character, SLOT_PLAYER_GET_UP, &args![]);
        return true;
    }
    // The marker-description structure, 16 bytes on the game's stack.
    let description = e.mem.alloc(0x10);
    let found_marker = e.mem.alloc(4);
    e.mem.set_u32(found_marker, 0);
    fn_00509890(e, Ptr::new(description), 0xff);
    let marker_source = e
        .vcall(activator.addr(), SLOT_ACTIVATOR_MARKER_SOURCE, &args![])
        .u32();
    let found = e
        .call(
            GET_CLOSEST_FREE_MARKER,
            &args![
                target,
                marker_source,
                1u32,
                1u32,
                description,
                found_marker,
                1u32
            ],
        )
        .bool();
    let marker_type = fn_005094b0(e, Ptr::new(description)) as u32;
    e.mem.free(description);
    e.mem.free(found_marker);
    if !found {
        return false;
    }
    if !tes_furniture_is_sleep_marker(e, marker_type) {
        e.call(FORCE_TEMP_THIRD_PERSON, &args![player_character, 1u32]);
        fn_005098c0(e, Ptr::new(player_character));
        e.call(INITIATE_SIT_SLEEP_PACKAGE, &args![player_character, target]);
        return true;
    }
    let owner = e.call(GET_OWNER, &args![target]).u32();
    let is_player = player_character == e.global::<u32>(PLAYER);
    if owner != 0
        && !e
            .call(IS_AN_OWNER, &args![target, player_character, 1u32])
            .bool()
    {
        if is_player {
            show_sad_message(e, MESSAGE_NOT_OWNER);
            return false;
        }
        return true;
    }
    if is_player {
        let player = e.global::<u32>(PLAYER);
        let seated = e.call(PLAYER_GET_SEATED_OBJECT, &args![player]).u32();
        if e.call(OBJECT_TEST_005444C0, &args![seated]).bool() {
            show_sad_message(e, MESSAGE_PLAYER_DISABLED);
            return false;
        }
    }
    let player = e.global::<u32>(PLAYER);
    if e.vcall(player, SLOT_PLAYER_TRESPASSING, &args![]).bool() {
        show_sad_message(e, MESSAGE_PLAYER_NOT_ALLOWED);
        return true;
    }
    let interior = e.call(GET_INTERIOR, &args![player]).u8() as u32;
    if e.call(SLEEP_STATE_TEST, &args![SLEEP_STATE, interior])
        .bool()
    {
        show_sad_message(e, MESSAGE_AREA_BLOCKED);
        return true;
    }
    if fn_005098e0(e, Ptr::new(SLEEP_STATE)) {
        show_sad_message(e, MESSAGE_SLEEP_UNAVAILABLE);
        return true;
    }
    if e.call(PLAYER_STATE_TEST, &args![player + 0x94]).bool() {
        show_sad_message(e, MESSAGE_CANNOT_SLEEP);
        return true;
    }
    e.call(CREATE_SLEEP_MENU, &args![1u32]);
    true
}

// Translated from 00509890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructs the 16-byte structure `Activate` passes to
/// `GetClosestFreeMarker`: calls `006815c0`, clears the word at +0xc and
/// stores the byte `marker_type` at +0xe. Returns `this`. (No class name in
/// the exe.)
pub fn fn_00509890(e: &mut Engine, this: Ptr, marker_type: u8) -> Ptr {
    e.call(RETURN_THIS, &args![this]);
    e.mem.set_u16(this.addr() + 0xc, 0);
    e.mem.set_u8(this.addr() + 0xe, marker_type);
    this
}

// Translated from 005098c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `float` at `this + 0x6e4` to 0.0. `Activate` and the load/clear
/// furniture code call it on the player; the exe names no class.
pub fn fn_005098c0(e: &mut Engine, this: Ptr) {
    e.mem.set_f32(this.addr() + 0x6e4, 0.0);
}

// Translated from 005098e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// True when either of the bytes at `this + 0x15c` and `this + 0x15d` is
/// non-zero (no class name in the exe).
pub fn fn_005098e0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x15c) != 0 || e.mem.u8(this.addr() + 0x15d) != 0
}

// Translated from 00509920 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESFurniture::GetMarkerTargetOffset` (Xbox PDB): writes the offset (three
/// `float`s) into `result` and returns it. The default is the three words at
/// `011f426c`. For a marker type `1..=0x28` x and y come from the settings
/// that the type's 16-byte table entry (`01189fa0`) points to, and z is
/// `(scale - 1.0)` times the third setting. `this` is unused.
pub fn tes_furniture_get_marker_target_offset(
    e: &mut Engine,
    _this: Ptr<TESFurniture>,
    result: Ptr,
    marker_type: u32,
    scale: f32,
) -> Ptr {
    let mut x = e.mem.u32(DEFAULT_MARKER_OFFSET);
    let mut y = e.mem.u32(DEFAULT_MARKER_OFFSET + 4);
    let mut z = e.mem.u32(DEFAULT_MARKER_OFFSET + 8);
    if marker_type != 0 && marker_type <= 0x28 {
        let entry = MARKER_OFFSET_TABLE + (marker_type - 1) * 16;
        let setting_x = e.mem.u32(entry);
        let address_x = setting_float_address(e, setting_x);
        x = e.mem.u32(address_x);
        let setting_y = e.mem.u32(entry + 4);
        let address_y = setting_float_address(e, setting_y);
        y = e.mem.u32(address_y);
        let setting_z = e.mem.u32(entry + 8);
        let address_z = setting_float_address(e, setting_z);
        let third = e.mem.f32(address_z);
        let one: f64 = e.global(ONE);
        // FLD float, FSUB double, FMUL float, FSTP float: computed in f64.
        z = ((((scale as f64) - one) * third as f64) as f32).to_bits();
    }
    e.mem.set_u32(result.addr(), x);
    e.mem.set_u32(result.addr() + 4, y);
    e.mem.set_u32(result.addr() + 8, z);
    result
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00508f90, tes_furniture_init_item(Ptr<TESFurniture>)),
        entry!(0x00508fd0, fn_00508fd0(Ptr<TESFurniture>, Ptr) -> bool),
        entry!(0x00509280, fn_00509280(Ptr<TESFurniture>)),
        entry!(0x005092f0, tes_furniture_copy(Ptr<TESFurniture>, Ptr)),
        entry!(0x00509350, tes_furniture_verify_markers(Ptr<TESFurniture>)),
        entry!(
            0x005093f0,
            tes_furniture_can_sit_on(Ptr<TESFurniture>) -> bool
        ),
        entry!(
            0x00509420,
            tes_furniture_can_sleep_on(Ptr<TESFurniture>) -> bool
        ),
        entry!(
            0x00509450,
            tes_furniture_get_marker_enabled(Ptr<TESFurniture>, u32) -> bool
        ),
        entry!(0x00509490, fn_00509490(Ptr) -> u16),
        entry!(0x005094b0, fn_005094b0(Ptr) -> u8),
        entry!(0x005094d0, fn_005094d0(Ptr, u32) -> Ptr),
        entry!(0x005094f0, tes_furniture_is_sleep_marker(u32) -> bool),
        entry!(0x00509510, tes_furniture_is_sit_marker(u32) -> bool),
        entry!(0x00509540, fn_00509540(Ptr<TESFurniture>, Ptr) -> bool),
        entry!(
            0x005095b0,
            tes_furniture_activate(Ptr<TESFurniture>, Ptr, Ptr, u32, u32, u32) -> bool
        ),
        entry!(0x00509890, fn_00509890(Ptr, u8) -> Ptr),
        entry!(0x005098c0, fn_005098c0(Ptr)),
        entry!(0x005098e0, fn_005098e0(Ptr) -> bool),
        entry!(
            0x00509920,
            tes_furniture_get_marker_target_offset(Ptr<TESFurniture>, Ptr, u32, f32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    /// A double that returns `eax` whatever it is called with.
    fn stub(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    fn furniture(e: &mut Engine, flags: u32) -> Ptr<TESFurniture> {
        let furniture: Ptr<TESFurniture> = e.new_object();
        e.set(furniture, TESFurniture::iFurnFlags, flags);
        furniture
    }

    /// Gives `object` a vtable of 0x500 bytes with `slots` filled in
    /// (byte offset, function address).
    fn with_vtable(e: &mut Engine, object: u32, slots: &[(u32, u32)]) {
        let table = e.mem.alloc(0x500);
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
        e.mem.set_u32(object, table);
    }

    /// The logged calls to `addr`, in order.
    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The addresses called, in order.
    fn addresses(e: &Engine) -> Vec<u32> {
        e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect()
    }

    #[test]
    fn init_item_does_nothing_for_a_flagged_form() {
        let mut e = Engine::new();
        let f = furniture(&mut e, 0);
        stub(&mut e, FORM_FLAG_TEST_BIT_3, 1);
        e.call_log = Some(vec![]);
        e.call(0x0050_8f90, &args![f]);
        assert_eq!(addresses(&e), [0x0050_8f90, FORM_FLAG_TEST_BIT_3]);
    }

    #[test]
    fn init_item_initializes_destructible_verifies_markers_and_flags_the_form() {
        let mut e = Engine::new();
        let f = furniture(&mut e, 0);
        stub(&mut e, FORM_FLAG_TEST_BIT_3, 0);
        stub(&mut e, DESTRUCTIBLE_INIT_ITEM, 0);
        stub(&mut e, FORM_FLAG_TEST_BIT_14, 1);
        stub(&mut e, SET_FORM_FLAG_CHANGED, 0);
        e.call_log = Some(vec![]);
        e.call(0x0050_8f90, &args![f]);
        assert_eq!(
            addresses(&e),
            [
                0x0050_8f90,
                FORM_FLAG_TEST_BIT_3,
                DESTRUCTIBLE_INIT_ITEM,
                FORM_FLAG_TEST_BIT_14,
                SET_FORM_FLAG_CHANGED
            ]
        );
        assert_eq!(
            calls_to(&e, DESTRUCTIBLE_INIT_ITEM),
            [[f.addr() + 0x68, f.addr()]]
        );
        assert_eq!(calls_to(&e, SET_FORM_FLAG_CHANGED), [[f.addr(), 1]]);
    }

    /// A file double that serves `chunks` (chunk type, chunk size) one after
    /// the other; `FILE_NEXT_CHUNK` advances. Returns the position.
    fn serve_chunks(e: &mut Engine, chunks: Vec<(u32, u32)>) -> Rc<Cell<usize>> {
        let position = Rc::new(Cell::new(0usize));
        let (p1, c1) = (position.clone(), chunks.clone());
        e.register_double(FILE_CHUNK_TYPE, move |_, _| Ret {
            eax: c1.get(p1.get()).map_or(0, |c| c.0),
            ..Ret::default()
        });
        let (p2, c2) = (position.clone(), chunks.clone());
        e.register_double(FILE_CHUNK_SIZE, move |_, _| Ret {
            eax: c2.get(p2.get()).map_or(0, |c| c.1),
            ..Ret::default()
        });
        let (p3, c3) = (position.clone(), chunks);
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            p3.set(p3.get() + 1);
            Ret {
                eax: (p3.get() < c3.len()) as u32,
                ..Ret::default()
            }
        });
        position
    }

    #[test]
    fn load_hands_every_chunk_to_its_component() {
        let mut e = Engine::new();
        let f = furniture(&mut e, 0);
        let file = Ptr::<()>::new(e.mem.alloc(8));
        let editor_id_slot = 0x7100_0134;
        let bound_slot = 0x7100_00e0;
        with_vtable(
            &mut e,
            f.addr(),
            &[
                (SLOT_SET_EDITOR_ID, editor_id_slot),
                (SLOT_LOAD_OBJECT_BOUND, bound_slot),
            ],
        );
        let position = serve_chunks(
            &mut e,
            vec![
                (CHUNK_FULL, 4),
                (CHUNK_MODL, 4),
                (CHUNK_EDID, 6),
                (CHUNK_OBND, 12),
                (CHUNK_MODS, 16),
                (CHUNK_SCRI, 4),
                (CHUNK_DSTD, 4),
                (CHUNK_MNAM, 4),
                (0x1234_5678, 4),
            ],
        );
        for addr in [
            LOAD_FORM,
            SET_FORM_FLAG_CHANGED,
            FULL_NAME_LOAD_CHUNK,
            MODEL_LOAD_CHUNK,
            DESTRUCTIBLE_LOAD_CHUNK,
            SCRIPTABLE_SET_SCRIPT,
            SCRIPTABLE_INIT_ITEM,
            MODEL_UNPACK_TEXTURE_SWAPS,
            editor_id_slot,
            bound_slot,
            FILE_GET_CHUNK_DATA,
        ] {
            stub(&mut e, addr, 0);
        }
        stub(&mut e, FILE_CHUNK_SWAP_COUNT, 0xabcd_0003);
        e.register(FILE_GET_CHUNK_DWORD, |e, a| {
            // SCRI writes the script reference, MNAM the marker flags.
            e.mem.set_u32(a[1], 0xcafe);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(0x0050_8fd0, &args![f, file]).bool();
        assert!(result);
        assert_eq!(position.get(), 9);
        assert_eq!(calls_to(&e, LOAD_FORM), [[f.addr(), file.addr()]]);
        assert_eq!(calls_to(&e, SET_FORM_FLAG_CHANGED), [[f.addr(), 0]]);
        assert_eq!(
            calls_to(&e, FULL_NAME_LOAD_CHUNK),
            [[f.addr() + 0x30, file.addr()]]
        );
        assert_eq!(
            calls_to(&e, MODEL_LOAD_CHUNK),
            [[f.addr() + 0x3c, file.addr()]]
        );
        assert_eq!(
            calls_to(&e, DESTRUCTIBLE_LOAD_CHUNK),
            [[f.addr() + 0x68, file.addr()]]
        );
        // EDID: at most 0x200 bytes into a buffer, then slot 0x134.
        let editor_id = calls_to(&e, editor_id_slot);
        assert_eq!(editor_id.len(), 1);
        assert_eq!(editor_id[0][0], f.addr());
        let reads = calls_to(&e, FILE_GET_CHUNK_DATA);
        assert_eq!(reads[0][1], editor_id[0][1]);
        assert_eq!(reads[0][2], 0x200);
        // MODS: a buffer of the chunk's size and the swap count (a u16).
        assert_eq!(reads[1][2], 16);
        let unpack = calls_to(&e, MODEL_UNPACK_TEXTURE_SWAPS);
        assert_eq!(unpack[0][0], f.addr() + 0x3c);
        assert_eq!(unpack[0][1], reads[1][1]);
        assert_eq!(unpack[0][2..], [16, 3]);
        assert_eq!(calls_to(&e, bound_slot), [[f.addr(), file.addr()]]);
        assert_eq!(
            calls_to(&e, SCRIPTABLE_SET_SCRIPT),
            [[f.addr() + 0x5c, 0xcafe]]
        );
        assert_eq!(
            calls_to(&e, SCRIPTABLE_INIT_ITEM),
            [[f.addr() + 0x5c, f.addr()]]
        );
        // MNAM read the marker flags straight into the form.
        assert_eq!(e.get(f, TESFurniture::iFurnFlags), 0xcafe);
    }

    #[test]
    fn load_without_chunks_only_loads_the_form() {
        let mut e = Engine::new();
        let f = furniture(&mut e, 0);
        let file = Ptr::<()>::new(e.mem.alloc(8));
        serve_chunks(&mut e, vec![]);
        stub(&mut e, LOAD_FORM, 0);
        stub(&mut e, SET_FORM_FLAG_CHANGED, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_8fd0, &args![f, file]).bool());
        assert_eq!(
            addresses(&e),
            [
                0x0050_8fd0,
                LOAD_FORM,
                SET_FORM_FLAG_CHANGED,
                FILE_CHUNK_TYPE
            ]
        );
    }

    #[test]
    fn save_writes_the_components_then_the_marker_flags() {
        let mut e = Engine::new();
        let f = furniture(&mut e, 0x4000_0003);
        for addr in [
            START_FORM,
            FULL_NAME_SAVE,
            MODEL_SAVE,
            SCRIPTABLE_SAVE,
            DESTRUCTIBLE_SAVE,
            WRITE_DWORD_CHUNK,
            CLOSE_FORM,
        ] {
            stub(&mut e, addr, 0);
        }
        e.call_log = Some(vec![]);
        e.call(0x0050_9280, &args![f]);
        assert_eq!(
            addresses(&e),
            [
                0x0050_9280,
                START_FORM,
                FULL_NAME_SAVE,
                MODEL_SAVE,
                SCRIPTABLE_SAVE,
                DESTRUCTIBLE_SAVE,
                WRITE_DWORD_CHUNK,
                CLOSE_FORM
            ]
        );
        let base = f.addr();
        assert_eq!(calls_to(&e, FULL_NAME_SAVE), [[base + 0x30]]);
        assert_eq!(
            calls_to(&e, MODEL_SAVE),
            [[base + 0x3c, CHUNK_MODL, CHUNK_MODT, CHUNK_MODS]]
        );
        assert_eq!(calls_to(&e, SCRIPTABLE_SAVE), [[base + 0x5c]]);
        assert_eq!(calls_to(&e, DESTRUCTIBLE_SAVE), [[base + 0x68]]);
        assert_eq!(calls_to(&e, WRITE_DWORD_CHUNK), [[CHUNK_MNAM, 0x4000_0003]]);
    }

    #[test]
    fn copy_ignores_a_source_that_is_not_a_furniture() {
        let mut e = Engine::new();
        let f = furniture(&mut e, 5);
        let source = Ptr::<()>::new(e.mem.alloc(0x100));
        stub(&mut e, RT_DYNAMIC_CAST, 0);
        e.call_log = Some(vec![]);
        e.call(0x0050_92f0, &args![f, source]);
        assert_eq!(addresses(&e), [0x0050_92f0, RT_DYNAMIC_CAST]);
        assert_eq!(
            calls_to(&e, RT_DYNAMIC_CAST),
            [[source.addr(), 0, TYPE_TES_FORM, TYPE_TES_FURNITURE, 0]]
        );
        assert_eq!(e.get(f, TESFurniture::iFurnFlags), 5);
    }

    #[test]
    fn copy_copies_components_and_marker_flags_then_verifies() {
        let mut e = Engine::new();
        let f = furniture(&mut e, 5);
        let source = furniture(&mut e, 0x22);
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        stub(&mut e, COPY_ALL_COMPONENTS, 0);
        stub(&mut e, FORM_FLAG_TEST_BIT_14, 1);
        e.call_log = Some(vec![]);
        e.call(0x0050_92f0, &args![f, source]);
        assert_eq!(
            addresses(&e),
            [
                0x0050_92f0,
                RT_DYNAMIC_CAST,
                COPY_ALL_COMPONENTS,
                FORM_FLAG_TEST_BIT_14
            ]
        );
        assert_eq!(
            calls_to(&e, COPY_ALL_COMPONENTS),
            [[f.addr(), source.addr()]]
        );
        assert_eq!(e.get(f, TESFurniture::iFurnFlags), 0x22);
    }

    /// A furniture with the vtable slots `VerifyMarkers` calls.
    fn verify_world(e: &mut Engine, flags: u32) -> Ptr<TESFurniture> {
        let f = furniture(e, flags);
        with_vtable(
            e,
            f.addr(),
            &[
                (SLOT_GET_MODEL_PATH, 0x7200_0178),
                (SLOT_GET_NAME_STRING, 0x7200_0130),
                (SLOT_SET_MARKER_FLAGS, 0x7200_0150),
            ],
        );
        // The model component at +0x3c has its own vtable.
        with_vtable(e, f.addr() + 0x3c, &[(MODEL_SLOT_GET_PATH, 0x7200_0014)]);
        stub(e, 0x7200_0178, 0x5000);
        stub(e, 0x7200_0130, 0x6000);
        stub(e, 0x7200_0150, 0);
        stub(e, 0x7200_0014, 0x7000);
        stub(e, PRINT_FORMATTED, 0);
        stub(e, FORM_FLAG_TEST_BIT_14, 0);
        f
    }

    #[test]
    fn verify_markers_does_nothing_for_a_flagged_form_or_with_markers() {
        let mut e = Engine::new();
        let f = verify_world(&mut e, 0);
        stub(&mut e, FORM_FLAG_TEST_BIT_14, 1);
        e.call_log = Some(vec![]);
        e.call(0x0050_9350, &args![f]);
        assert_eq!(addresses(&e), [0x0050_9350, FORM_FLAG_TEST_BIT_14]);
        let f = verify_world(&mut e, 4);
        e.call_log = Some(vec![]);
        e.call(0x0050_9350, &args![f]);
        assert_eq!(addresses(&e), [0x0050_9350, FORM_FLAG_TEST_BIT_14]);
    }

    #[test]
    fn verify_markers_reports_a_model_without_furniture_markers() {
        let mut e = Engine::new();
        let f = verify_world(&mut e, 0);
        stub(&mut e, FURNITURE_MARKER_FIND, 0);
        e.call_log = Some(vec![]);
        e.call(0x0050_9350, &args![f]);
        assert_eq!(calls_to(&e, 0x7200_0178), [[f.addr(), 0]]);
        assert_eq!(calls_to(&e, FURNITURE_MARKER_FIND), [[0x5000]]);
        assert_eq!(calls_to(&e, 0x7200_0014), [[f.addr() + 0x3c]]);
        assert_eq!(calls_to(&e, PRINT_FORMATTED), [[NO_MARKERS_FORMAT, 0x7000]]);
        assert_eq!(calls_to(&e, 0x7200_0150), [[f.addr(), 0]]);
    }

    #[test]
    fn verify_markers_reports_markers_that_were_not_selected() {
        let mut e = Engine::new();
        let f = verify_world(&mut e, 0);
        stub(&mut e, FURNITURE_MARKER_FIND, 0x9999);
        e.call_log = Some(vec![]);
        e.call(0x0050_9350, &args![f]);
        assert_eq!(calls_to(&e, 0x7200_0130), [[f.addr()]]);
        assert_eq!(
            calls_to(&e, PRINT_FORMATTED),
            [[NO_MARKERS_SELECTED_FORMAT, 0x6000]]
        );
        assert_eq!(calls_to(&e, 0x7200_0150), [[f.addr(), 0]]);
    }

    #[test]
    fn can_sit_on_and_can_sleep_on_test_bits_30_and_31() {
        let mut e = Engine::new();
        for (flags, sit, sleep) in [
            (0u32, false, false),
            (0x4000_0000, true, false),
            (0x8000_0000, false, true),
            (0xffff_ffff, true, true),
            (0x3fff_ffff, false, false),
        ] {
            let f = furniture(&mut e, flags);
            assert_eq!(e.call(0x0050_93f0, &args![f]).bool(), sit, "{flags:x}");
            assert_eq!(e.call(0x0050_9420, &args![f]).bool(), sleep, "{flags:x}");
        }
    }

    #[test]
    fn get_marker_enabled_tests_the_bit_and_rejects_32_and_above() {
        let mut e = Engine::new();
        let f = furniture(&mut e, 0x8000_0005);
        for (marker, expected) in [
            (0u32, true),
            (1, false),
            (2, true),
            (31, true),
            (32, false),
            (33, false),
        ] {
            assert_eq!(
                e.call(0x0050_9450, &args![f, marker]).bool(),
                expected,
                "{marker}"
            );
        }
        // Marker 32 would wrap around to bit 0 if it were not rejected.
        let f = furniture(&mut e, 1);
        assert!(!e.call(0x0050_9450, &args![f, 32u32]).bool());
    }

    #[test]
    fn sleep_and_sit_marker_ranges() {
        let mut e = Engine::new();
        for marker in 0..40u32 {
            assert_eq!(
                e.call(0x0050_94f0, &args![marker]).bool(),
                marker < 10,
                "sleep {marker}"
            );
            assert_eq!(
                e.call(0x0050_9510, &args![marker]).bool(),
                (10..=20).contains(&marker) || marker == 26,
                "sit {marker}"
            );
        }
    }

    #[test]
    fn small_members_forward_and_read_fields() {
        let mut e = Engine::new();
        let object = Ptr::<()>::new(e.mem.alloc(0x700));
        stub(&mut e, FORWARD_009938B0, 0);
        stub(&mut e, FORWARD_00558DD0, 0);
        stub(&mut e, RETURN_THIS, 0);
        e.call_log = Some(vec![]);
        e.call(0x0050_9490, &args![object]);
        e.call(0x0050_94d0, &args![object, 77u32]);
        assert_eq!(calls_to(&e, FORWARD_009938B0), [[object.addr() + 0xc]]);
        assert_eq!(calls_to(&e, FORWARD_00558DD0), [[object.addr() + 0xc, 77]]);
        // 00509890 builds the structure; 005094b0 reads its type byte.
        e.mem.set_u16(object.addr() + 0xc, 0x1234);
        let back = e.call(0x0050_9890, &args![object, 0xffu32]).ptr::<()>();
        assert_eq!(back, object);
        assert_eq!(e.mem.u16(object.addr() + 0xc), 0);
        assert_eq!(e.call(0x0050_94b0, &args![object]).u32(), 0xff);
        assert_eq!(calls_to(&e, RETURN_THIS), [[object.addr()]]);
        // 005098c0 clears the float at +0x6e4.
        e.mem.set_f32(object.addr() + 0x6e4, 3.5);
        e.call(0x0050_98c0, &args![object]);
        assert_eq!(e.mem.f32(object.addr() + 0x6e4), 0.0);
    }

    #[test]
    fn fn_005098e0_is_true_when_either_byte_is_set() {
        let mut e = Engine::new();
        let object = Ptr::<()>::new(e.mem.alloc(0x200));
        assert!(!e.call(0x0050_98e0, &args![object]).bool());
        e.mem.set_u8(object.addr() + 0x15c, 1);
        assert!(e.call(0x0050_98e0, &args![object]).bool());
        e.mem.set_u8(object.addr() + 0x15c, 0);
        e.mem.set_u8(object.addr() + 0x15d, 7);
        assert!(e.call(0x0050_98e0, &args![object]).bool());
    }

    #[test]
    fn compare_reports_a_difference_unless_everything_matches() {
        let mut e = Engine::new();
        let a = furniture(&mut e, 0x10);
        let b = furniture(&mut e, 0x10);
        e.register(RT_DYNAMIC_CAST, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        stub(&mut e, COMPARE_ALL_COMPONENTS, 0);
        assert!(!e.call(0x0050_9540, &args![a, b]).bool());
        e.mem.set_u32(b.addr() + 0x90, 0x11);
        assert!(e.call(0x0050_9540, &args![a, b]).bool());
        // Components differ.
        e.mem.set_u32(b.addr() + 0x90, 0x10);
        stub(&mut e, COMPARE_ALL_COMPONENTS, 1);
        assert!(e.call(0x0050_9540, &args![a, b]).bool());
        // Not a furniture: differs, without comparing components.
        stub(&mut e, RT_DYNAMIC_CAST, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0050_9540, &args![a, b]).bool());
        assert_eq!(addresses(&e), [0x0050_9540, RT_DYNAMIC_CAST]);
    }

    #[test]
    fn marker_target_offset_defaults_for_type_zero_and_out_of_range() {
        let mut e = Engine::new();
        for page in [0x011f_4000, 0x0118_9000, 0x0101_2000] {
            e.map(page, 0x1000);
        }
        e.mem.set_u32(DEFAULT_MARKER_OFFSET, 1);
        e.mem.set_u32(DEFAULT_MARKER_OFFSET + 4, 2);
        e.mem.set_u32(DEFAULT_MARKER_OFFSET + 8, 3);
        let f = furniture(&mut e, 0);
        for marker_type in [0u32, 0x29, 0x1000] {
            let out = Ptr::<()>::new(e.mem.alloc(12));
            let back = e
                .call(0x0050_9920, &args![f, out, marker_type, 3.0f32])
                .ptr::<()>();
            assert_eq!(back, out);
            assert_eq!(
                [
                    e.mem.u32(out.addr()),
                    e.mem.u32(out.addr() + 4),
                    e.mem.u32(out.addr() + 8)
                ],
                [1, 2, 3]
            );
        }
    }

    #[test]
    fn marker_target_offset_reads_the_settings_of_the_marker_type() {
        let mut e = Engine::new();
        for page in [0x011f_4000, 0x0118_9000, 0x0118_a000, 0x0101_2000] {
            e.map(page, 0x1000);
        }
        e.set_global(ONE, 1.0f64);
        // Settings: x = 1.5, y = -2.0, z base = 4.0 for type 1; type 0x28
        // has a null z setting (the exe's static zero).
        let make = |e: &mut Engine, value: f32| {
            let setting = e.mem.alloc(8);
            e.mem.set_f32(setting + 4, value);
            setting
        };
        let (x, y, z) = (make(&mut e, 1.5), make(&mut e, -2.0), make(&mut e, 4.0));
        e.mem.set_u32(MARKER_OFFSET_TABLE, x);
        e.mem.set_u32(MARKER_OFFSET_TABLE + 4, y);
        e.mem.set_u32(MARKER_OFFSET_TABLE + 8, z);
        let last = MARKER_OFFSET_TABLE + 0x27 * 16;
        e.mem.set_u32(last, x);
        e.mem.set_u32(last + 4, y);
        e.mem.set_u32(last + 8, 0);
        // A null setting yields the address of a zero float.
        let zero = e.mem.alloc(4);
        e.register_double(SETTING_FLOAT, move |_, a| Ret {
            eax: if a[0] != 0 { a[0] + 4 } else { zero },
            ..Ret::default()
        });
        let f = furniture(&mut e, 0);
        let out = Ptr::<()>::new(e.mem.alloc(12));
        e.call(0x0050_9920, &args![f, out, 1u32, 2.5f32]);
        assert_eq!(e.mem.f32(out.addr()), 1.5);
        assert_eq!(e.mem.f32(out.addr() + 4), -2.0);
        // (2.5 - 1.0) * 4.0
        assert_eq!(e.mem.f32(out.addr() + 8), 6.0);
        e.call(0x0050_9920, &args![f, out, 0x28u32, 2.5f32]);
        assert_eq!(e.mem.f32(out.addr() + 8), 0.0);
    }

    /// What the `Activate` test world answers.
    #[derive(Clone, Copy)]
    struct Scene {
        is_player: bool,
        in_furniture: u32,
        found_marker: bool,
        marker_type: u8,
        owner: u32,
        is_owner: bool,
        seated_test: bool,
        trespassing: bool,
        interior_blocked: bool,
        sleep_state: bool,
        player_state: bool,
    }

    impl Scene {
        fn sleeping_allowed() -> Scene {
            Scene {
                is_player: true,
                in_furniture: 0,
                found_marker: true,
                marker_type: 3,
                owner: 0,
                is_owner: false,
                seated_test: false,
                trespassing: false,
                interior_blocked: false,
                sleep_state: false,
                player_state: false,
            }
        }
    }

    /// Runs `Activate` in `scene`; returns the result, the engine (with its
    /// call log), the player and the target reference.
    fn activate(scene: Scene) -> (bool, Engine, Ptr<()>, Ptr<()>) {
        let mut e = Engine::new();
        let f = furniture(&mut e, 0);
        let target = Ptr::<()>::new(e.mem.alloc(0x100));
        let player = Ptr::<()>::new(e.mem.alloc(0x800));
        e.mem.set_f32(player.addr() + 0x6e4, 9.0);
        e.map(PLAYER & !0xfff, 0x1000);
        e.map(MESSAGE_DISPLAY_TIME & !0xfff, 0x1000);
        e.set_global(PLAYER, player.addr());
        e.set_global(MESSAGE_DISPLAY_TIME, 2.0f32);
        with_vtable(
            &mut e,
            player.addr(),
            &[
                (SLOT_PLAYER_IS_NPC_STATE, 0x7300_0214),
                (SLOT_PLAYER_GET_UP, 0x7300_0418),
                (SLOT_PLAYER_TRESPASSING, 0x7300_0448),
                (SLOT_ACTIVATOR_MARKER_SOURCE, 0x7300_01f4),
            ],
        );
        let cast = if scene.is_player { player.addr() } else { 0 };
        stub(&mut e, RT_DYNAMIC_CAST, cast);
        stub(&mut e, 0x7300_0214, scene.in_furniture);
        stub(&mut e, 0x7300_0418, 0);
        stub(&mut e, 0x7300_0448, scene.trespassing as u32);
        stub(&mut e, 0x7300_01f4, 0x4242);
        stub(&mut e, FORCE_TEMP_THIRD_PERSON, 0);
        stub(&mut e, INITIATE_SIT_SLEEP_PACKAGE, 0);
        stub(&mut e, GET_OWNER, scene.owner);
        stub(&mut e, IS_AN_OWNER, scene.is_owner as u32);
        stub(&mut e, PLAYER_GET_SEATED_OBJECT, 0x5151);
        stub(&mut e, OBJECT_TEST_005444C0, scene.seated_test as u32);
        stub(&mut e, GET_INTERIOR, 0x0000_0101);
        stub(&mut e, SLEEP_STATE_TEST, scene.interior_blocked as u32);
        stub(&mut e, PLAYER_STATE_TEST, scene.player_state as u32);
        stub(&mut e, SHOW_MESSAGE, 0);
        stub(&mut e, CREATE_SLEEP_MENU, 0);
        stub(&mut e, RETURN_THIS, 0);
        e.register(SETTING_STRING, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.map(SLEEP_STATE & !0xfff, 0x1000);
        e.mem.set_u8(SLEEP_STATE + 0x15c, scene.sleep_state as u8);
        let (found, kind) = (scene.found_marker, scene.marker_type);
        e.register_double(GET_CLOSEST_FREE_MARKER, move |e, a| {
            // this, source, 1, 1, description, found marker, 1
            assert_eq!(a[1..4], [0x4242, 1, 1]);
            assert_eq!(a[6], 1);
            assert_eq!(e.mem.u8(a[4] + 0xe), 0xff);
            e.mem.set_u8(a[4] + 0xe, kind);
            Ret {
                eax: found as u32,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        let result = e
            .call(0x0050_95b0, &args![f, target, player, 11u32, 22u32, 33u32])
            .bool();
        (result, e, player, target)
    }

    fn messages(e: &Engine) -> Vec<u32> {
        calls_to(e, SHOW_MESSAGE).iter().map(|c| c[0]).collect()
    }

    #[test]
    fn activate_ignores_an_activator_that_is_not_the_player() {
        let (result, e, _, _) = activate(Scene {
            is_player: false,
            ..Scene::sleeping_allowed()
        });
        assert!(!result);
        assert_eq!(addresses(&e), [0x0050_95b0, RT_DYNAMIC_CAST]);
    }

    #[test]
    fn activate_gets_the_player_up_when_already_in_furniture() {
        let (result, e, player, _) = activate(Scene {
            in_furniture: 1,
            ..Scene::sleeping_allowed()
        });
        assert!(result);
        assert_eq!(
            addresses(&e),
            [
                0x0050_95b0,
                RT_DYNAMIC_CAST,
                0x7300_0214,
                FORCE_TEMP_THIRD_PERSON,
                0x7300_0418
            ]
        );
        assert_eq!(calls_to(&e, FORCE_TEMP_THIRD_PERSON), [[player.addr(), 1]]);
        assert_eq!(e.mem.f32(player.addr() + 0x6e4), 0.0);
    }

    #[test]
    fn activate_fails_without_a_free_marker() {
        let (result, e, player, _) = activate(Scene {
            found_marker: false,
            ..Scene::sleeping_allowed()
        });
        assert!(!result);
        assert!(calls_to(&e, INITIATE_SIT_SLEEP_PACKAGE).is_empty());
        assert_eq!(e.mem.f32(player.addr() + 0x6e4), 9.0);
    }

    #[test]
    fn activate_starts_the_sit_package_for_a_sit_marker() {
        let (result, e, player, target) = activate(Scene {
            marker_type: 12,
            ..Scene::sleeping_allowed()
        });
        assert!(result);
        assert_eq!(calls_to(&e, FORCE_TEMP_THIRD_PERSON), [[player.addr(), 1]]);
        assert_eq!(e.mem.f32(player.addr() + 0x6e4), 0.0);
        assert_eq!(
            calls_to(&e, INITIATE_SIT_SLEEP_PACKAGE),
            [[player.addr(), target.addr()]]
        );
        assert!(calls_to(&e, GET_OWNER).is_empty());
    }

    #[test]
    fn activate_opens_the_sleep_menu_when_nothing_objects() {
        let (result, e, _, target) = activate(Scene::sleeping_allowed());
        assert!(result);
        assert_eq!(calls_to(&e, CREATE_SLEEP_MENU), [[1]]);
        assert!(messages(&e).is_empty());
        assert_eq!(calls_to(&e, GET_OWNER), [[target.addr()]]);
        // The interior byte reaches the sleep-state test.
        assert_eq!(calls_to(&e, SLEEP_STATE_TEST), [[SLEEP_STATE, 1]]);
    }

    #[test]
    fn activate_refuses_a_bed_owned_by_somebody_else() {
        let (result, e, player, target) = activate(Scene {
            owner: 0x777,
            ..Scene::sleeping_allowed()
        });
        assert!(!result);
        assert_eq!(messages(&e), [MESSAGE_NOT_OWNER]);
        assert_eq!(
            calls_to(&e, IS_AN_OWNER),
            [[target.addr(), player.addr(), 1]]
        );
        let message = calls_to(&e, SHOW_MESSAGE).remove(0);
        // Text, 0, icon path, 0, display time (2.0), 0.
        assert_eq!(
            message,
            [
                MESSAGE_NOT_OWNER,
                0,
                MESSAGE_ICON_PATH,
                0,
                2.0f32.to_bits(),
                0
            ]
        );
        assert!(calls_to(&e, CREATE_SLEEP_MENU).is_empty());
    }

    #[test]
    fn activate_lets_the_owner_sleep() {
        let (result, e, _, _) = activate(Scene {
            owner: 0x777,
            is_owner: true,
            ..Scene::sleeping_allowed()
        });
        assert!(result);
        assert_eq!(calls_to(&e, CREATE_SLEEP_MENU), [[1]]);
    }

    #[test]
    fn activate_refuses_when_the_seated_object_test_fails() {
        let (result, e, _, _) = activate(Scene {
            seated_test: true,
            ..Scene::sleeping_allowed()
        });
        assert!(!result);
        assert_eq!(messages(&e), [MESSAGE_PLAYER_DISABLED]);
        assert_eq!(calls_to(&e, OBJECT_TEST_005444C0), [[0x5151]]);
    }

    #[test]
    fn activate_shows_one_message_per_objection_and_still_succeeds() {
        for (scene, message) in [
            (
                Scene {
                    trespassing: true,
                    ..Scene::sleeping_allowed()
                },
                MESSAGE_PLAYER_NOT_ALLOWED,
            ),
            (
                Scene {
                    interior_blocked: true,
                    ..Scene::sleeping_allowed()
                },
                MESSAGE_AREA_BLOCKED,
            ),
            (
                Scene {
                    sleep_state: true,
                    ..Scene::sleeping_allowed()
                },
                MESSAGE_SLEEP_UNAVAILABLE,
            ),
            (
                Scene {
                    player_state: true,
                    ..Scene::sleeping_allowed()
                },
                MESSAGE_CANNOT_SLEEP,
            ),
        ] {
            let (result, e, player, _) = activate(scene);
            assert!(result, "{message:x}");
            assert_eq!(messages(&e), [message]);
            assert!(calls_to(&e, CREATE_SLEEP_MENU).is_empty());
            if message == MESSAGE_CANNOT_SLEEP {
                assert_eq!(calls_to(&e, PLAYER_STATE_TEST), [[player.addr() + 0x94]]);
            }
        }
    }

    #[test]
    fn small_forwarders_answer_their_callees_values() {
        let mut e = Engine::new();
        let object = Ptr::<()>::new(e.mem.alloc(0x700));
        stub(&mut e, FORWARD_009938B0, 3);
        stub(&mut e, FORWARD_00558DD0, 0x7000);
        assert_eq!(e.call(0x0050_9490, &args![object]).u16(), 3);
        assert_eq!(e.call(0x0050_94d0, &args![object, 77u32]).u32(), 0x7000);
    }
}
