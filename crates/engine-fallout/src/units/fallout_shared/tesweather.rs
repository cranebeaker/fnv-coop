//! `fallout shared/tesweather.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `TESWeather` (Xbox PDB), 0x374 bytes on PC (the Xbox `TESForm` base is 0x10 larger, so
    /// every field sits 0x10 lower than the PDB says; the PC code reads these offsets).
    pub struct TESWeather: 0x374 {
        /// The `TESImageSpaceModifiableCountForm<6>` base (Xbox PDB): vtable, then six
        /// `TESImageSpaceModifier` pointers.
        0x018 imageSpaceModifiers: u32,
        /// `cloudTexture` (Xbox PDB): four `TESTexture`s of 0xc bytes.
        0x034 cloudTexture: u32,
        /// `ucCloudLayerSpeed` (Xbox PDB): four bytes.
        0x064 ucCloudLayerSpeed: u8,
        /// `uiCloudColorData` (Xbox PDB): four layers of six colour words.
        0x068 uiCloudColorData: u32,
        /// `precipNIF` (Xbox PDB): a `TESModel`.
        0x0c8 precipNIF: u32,
        /// `ucWeatherData` (Xbox PDB): 15 bytes.
        0x0e0 ucWeatherData: u8,
        /// `fFogData` (Xbox PDB): six floats.
        0x0f0 fFogData: f32,
        /// `uiColorData` (Xbox PDB): ten colour kinds of six words.
        0x108 uiColorData: u32,
        /// `soundList` (Xbox PDB): a `BSSimpleList` of weather sounds.
        0x1f8 soundList: u32,
        /// `fHDRData` (Xbox PDB): 14 floats.
        0x200 fHDRData: f32,
        /// `pImageSpaceParameterMods` (Xbox PDB): two `ImageSpaceParameterData` of 0x98 bytes.
        0x238 pImageSpaceParameterMods: u32,
        /// `iNumCloudLayers` (Xbox PDB).
        0x368 iNumCloudLayers: i32,
        /// `strName` (Xbox PDB): a `BSStringT<char>`.
        0x36c strName: u32,
    }
}

/// The base constructor `BaseFormComponent` (`00402d30`, `ECX`, answers `this`).
const BASE_FORM_COMPONENT_CTOR: u32 = 0x0040_2d30;
/// The vtable of `TESImageSpaceModifiableCountForm<6>` (RTTI
/// `.?AV?$TESImageSpaceModifiableCountForm@$05@@`).
const MODIFIABLE_COUNT_FORM_VTABLE: u32 = 0x0103_1838;
/// The game's `memset` wrapper (`00403d30`, `cdecl`: destination, byte, size).
const MEMSET: u32 = 0x0040_3d30;
/// `TESSoundFile::SetSoundFile` (engine map, `00489100`; by its body: copies a C string into
/// the `BSStringT` at `this + 4`), `thiscall`, one stack word.
const SET_STRING_00489100: u32 = 0x0048_9100;
/// The texture names `TESWeather::MakeDefault` sets.
const CLOUD_UPPER_NAME: u32 = 0x0103_1810;
const CLOUD_HORIZON_NAME: u32 = 0x0103_17f0;
const CLOUD_LOWER_NAME: u32 = 0x0103_17cc;
/// The floats `TESWeather::MakeDefault` copies into the fog data.
const FOG_FAR_DEFAULT: u32 = 0x0103_17c8;
const FOG_POWER_DEFAULT: u32 = 0x0103_17c4;
/// `006815c0` (`ECX`, answers `this`: a list node read as its item word).
const LIST_ITEM_006815C0: u32 = 0x0068_15c0;
/// `00726070` (`ECX`, answers the word at +4: a list node's next).
const NODE_FOLLOWING_00726070: u32 = 0x0072_6070;
/// `operator delete` (`00401030`, `cdecl`, one word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `00470470` (`ECX`): empties a `BSSimpleList`.
const LIST_REMOVE_ALL_00470470: u32 = 0x0047_0470;
/// `004013e0` (`ECX`): whether form flag 8 is set (`AL`).
const FORM_FLAG_8_SET: u32 = 0x0040_13e0;
/// `00484ab0` (`ECX`, one stack word): sets or clears form flag 8.
const SET_FORM_FLAG_8: u32 = 0x0048_4ab0;
/// `TESForm::GetFile` (Xbox PDB, `00484e60`, `ECX`, one stack word).
const GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB, `00485d50`, `cdecl`: address of a form id, file).
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `004839c0` (`cdecl`, one word): the form of a form id, or null.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `__RTDynamicCast` (`00ec43fb`, `cdecl`: object, vftable delta, source type, target type,
/// is-reference).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// RTTI type descriptor `.?AVTESForm@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// RTTI type descriptor `.?AVTESImageSpaceModifier@@`.
const TYPE_IMAGE_SPACE_MODIFIER: u32 = 0x0118_62b8;
/// `00474cb0` (`ECX`): the length of the form's name (vtable slot +0x130), 0 without one.
const NAME_LENGTH_00474CB0: u32 = 0x0047_4cb0;
/// `0084e3a0` (`ECX`): the word at +0xc (the form id).
const FORM_ID_0084E3A0: u32 = 0x0084_e3a0;
/// The release build's logging stub (`005b5e40`, `cdecl`, variable words).
const LOG: u32 = 0x005b_5e40;
/// "MASTERFILE: Unable to find imagespacemodifier (%08X) on owner object \"%s\"."
const MESSAGE_NAMED_OWNER: u32 = 0x0101_cc80;
/// "MASTERFILE: Unable to find imagespacemodifier (%08X) on owner object (%08X)."
const MESSAGE_OWNER_ID: u32 = 0x0101_cc30;
/// "MASTERFILE: Unable to find imagespacemodifier (%08X) on unknown owner."
const MESSAGE_UNKNOWN_OWNER: u32 = 0x0101_cbe8;

/// `TESFile::GetTESForm` (Xbox PDB, `00472660`, `ECX` = file): the record's form type (`AL`).
const GET_TES_FORM: u32 = 0x0047_2660;
/// `TESForm::LoadForm` (Xbox PDB, `00485110`, `ECX` = form, one stack word: the file).
const LOAD_FORM: u32 = 0x0048_5110;
/// `TESFile::GetTESChunk` (Xbox PDB, `004726b0`, `ECX` = file): the chunk's code, 0 at the end.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `00401660` (`ECX` = file): the current chunk's size.
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESFile::GetChunkData` (Xbox PDB, `004727f0`, `ECX` = file, one stack word: destination).
const GET_CHUNK_DATA: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData_ov3` (Xbox PDB, `00472890`, `ECX` = file, two stack words:
/// destination, size).
const GET_CHUNK_DATA_SIZED: u32 = 0x0047_2890;
/// `00401680` (`ECX` = file): whether the file's data needs its byte order swapped (`AL`).
const NEEDS_SWAP: u32 = 0x0040_1680;
/// `00401080` (`cdecl`: address of a word, a byte flag): swaps the word's byte order.
const SWAP_WORD: u32 = 0x0040_1080;
/// `TESTexture::LoadTextureChunk` (Xbox PDB, `0048e3d0`, `cdecl`: texture, file).
const LOAD_TEXTURE_CHUNK: u32 = 0x0048_e3d0;
/// `TESModel::LoadModelChunk` (Xbox PDB, `004892d0`, `cdecl`: model, file).
const LOAD_MODEL_CHUNK: u32 = 0x0048_92d0;
/// `ImageSpaceParameterData::Endian` (Xbox PDB, `00b8ad00`, `ECX`).
const IMAGE_SPACE_PARAMETERS_ENDIAN: u32 = 0x00b8_ad00;
/// The allocator (`00401000`, `cdecl`, one word: the size).
const ALLOCATE: u32 = 0x0040_1000;
/// `00462230` (`ECX`): swaps the byte order of a weather sound record.
const SWAP_SOUND_00462230: u32 = 0x0046_2230;
/// `00905820` (`ECX` = list, one stack word: the address of the item): adds an item to a
/// `BSSimpleList`.
const LIST_ADD_00905820: u32 = 0x0090_5820;
/// `004726f0` (`ECX` = file): steps to the next chunk; false at the record's end (`AL`).
const NEXT_CHUNK: u32 = 0x0047_26f0;

/// Chunk codes `TESWeather::Load` reads (the four characters, first byte lowest).
const CHUNK_ANAM: u32 = 0x4d41_4e41;
const CHUNK_BNAM: u32 = 0x4d41_4e42;
const CHUNK_CNAM: u32 = 0x4d41_4e43;
const CHUNK_DNAM: u32 = 0x4d41_4e44;
const CHUNK_ENAM: u32 = 0x4d41_4e45;
const CHUNK_FNAM: u32 = 0x4d41_4e46;
const CHUNK_HNAM: u32 = 0x4d41_4e48;
const CHUNK_INAM: u32 = 0x4d41_4e49;
const CHUNK_LNAM: u32 = 0x4d41_4e4c;
const CHUNK_MNAM: u32 = 0x4d41_4e4d;
const CHUNK_ONAM: u32 = 0x4d41_4e4f;
const CHUNK_PNAM: u32 = 0x4d41_4e50;
const CHUNK_SNAM: u32 = 0x4d41_4e53;
/// `"\0IAD"` to `"\x06IAD"`: the six image space modifier chunks.
const CHUNK_IAD_0: u32 = 0x4441_4900;
const CHUNK_NAM0: u32 = 0x304d_414e;
const CHUNK_DATA: u32 = 0x4154_4144;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_MODL: u32 = 0x4c44_4f4d;
/// The weather record's form type.
const FORM_TYPE_WEATHER: u8 = 0x35;

// Translated from 00581fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESImageSpaceModifiableCountForm<6>` constructor (in this unit's range; `ECX`, no
/// stack words): the `BaseFormComponent` base (`00402d30`), its vtable `01031838`, then its
/// `InitializeDataComponent` (`00582000`); answers `this`.
pub fn fn_00581fd0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BASE_FORM_COMPONENT_CTOR, &args![this]);
    e.mem.set_u32(this.addr(), MODIFIABLE_COUNT_FORM_VTABLE);
    tes_image_space_modifiable_count_form_initialize_data_component(e, this);
    this
}

// Translated from 00582000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifiableCountForm<6>::InitializeDataComponent` (Xbox PDB), `ECX`: clears
/// the six modifier pointers (`+4` to `+0x18`).
pub fn tes_image_space_modifiable_count_form_initialize_data_component(e: &mut Engine, this: Ptr) {
    for index in 0..6u32 {
        e.mem.set_u32(this.addr() + index * 4 + 4, 0);
    }
}

// Translated from 00581fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range), `ECX` = the `TESImageSpaceModifiableCountForm<6>`
/// and two stack words (`RET 8`): stores `modifier` as modifier `index` (`+4 + index * 4`).
pub fn fn_00581fb0(e: &mut Engine, this: Ptr, index: u32, modifier: u32) {
    e.mem.set_u32(
        this.addr()
            .wrapping_add(index.wrapping_mul(4))
            .wrapping_add(4),
        modifier,
    );
}

// Translated from 005820a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range), `ECX` = the `TESImageSpaceModifiableCountForm<6>`
/// and one stack word (`RET 4`), the owning form or null: resolves each nonzero modifier,
/// which loading left as a form id. With an owner the id first gets the compile index of the
/// owner's file (`TESForm::AddCompileIndex` with `TESForm::GetFile(-1)`); the id's form
/// (`004839c0`) cast from `TESForm` to the modifier type (`__RTDynamicCast`) replaces it. A
/// null result is reported (`005b5e40`): without an owner "unknown owner"; with a named owner
/// (`00474cb0` nonzero) its name (vtable slot +0x130); otherwise its form id (`0084e3a0`).
pub fn fn_005820a0(e: &mut Engine, this: Ptr, owner: Ptr) {
    e.with_stack(4, |e, id_at| {
        for index in 0..6u32 {
            let slot = this.addr() + index * 4 + 4;
            if e.mem.u32(slot) == 0 {
                continue;
            }
            e.mem.set_u32(id_at.addr(), e.mem.u32(slot));
            if !owner.is_null() {
                let file = e.call(GET_FILE, &args![owner, u32::MAX]).u32();
                e.call(ADD_COMPILE_INDEX, &args![id_at, file]);
            }
            let id = e.mem.u32(id_at.addr());
            let form = e.call(LOOKUP_FORM, &args![id]).u32();
            let modifier = e
                .call(
                    DYNAMIC_CAST,
                    &args![form, 0u32, TYPE_TES_FORM, TYPE_IMAGE_SPACE_MODIFIER, 0u32],
                )
                .u32();
            e.mem.set_u32(slot, modifier);
            if e.mem.u32(slot) != 0 {
                continue;
            }
            let id = e.mem.u32(id_at.addr());
            if owner.is_null() {
                e.call(LOG, &args![MESSAGE_UNKNOWN_OWNER, id]);
            } else if e.call(NAME_LENGTH_00474CB0, &args![owner]).u32() != 0 {
                let name = e.vcall(owner.addr(), 0x130, &args![]).u32();
                e.call(LOG, &args![MESSAGE_NAMED_OWNER, id, name]);
            } else {
                let owner_id = e.call(FORM_ID_0084E3A0, &args![owner]).u32();
                e.call(LOG, &args![MESSAGE_OWNER_ID, id, owner_id]);
            }
        }
    });
}

// Translated from 005813b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range; `TESWeather::Load` runs it on each new 8-byte
/// sound record), `ECX`: clears the 8 bytes (`memset`) and answers `this`.
pub fn fn_005813b0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MEMSET, &args![this, 0u32, 8u32]);
    this
}

// Translated from 00581b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeather`'s vtable (`0103168c`, RTTI `.?AVTESWeather@@`) slot +0x14, the
/// `InitializeData` slot of the Xbox PDB's `TESForm` (`ECX`): three cloud layers, the colour
/// data, weather data and fog data cleared, the last two fog floats 1.0, the cloud speeds and
/// the cloud colours cleared.
pub fn tes_weather_initialize_data(e: &mut Engine, this: Ptr<TESWeather>) {
    e.set(this, TESWeather::iNumCloudLayers, 3);
    let base = this.addr();
    e.call(
        MEMSET,
        &args![base + TESWeather::uiColorData.off, 0u32, 0xf0u32],
    );
    e.call(
        MEMSET,
        &args![base + TESWeather::ucWeatherData.off, 0u32, 0xfu32],
    );
    e.call(
        MEMSET,
        &args![base + TESWeather::fFogData.off, 0u32, 0x18u32],
    );
    // FLD1; FSTP float.
    e.mem.set_u32(base + 0x100, x87_store32(1.0));
    e.mem.set_u32(base + 0x104, x87_store32(1.0));
    e.call(
        MEMSET,
        &args![base + TESWeather::ucCloudLayerSpeed.off, 0u32, 4u32],
    );
    e.call(
        MEMSET,
        &args![base + TESWeather::uiCloudColorData.off, 0u32, 0x60u32],
    );
}

// Translated from 00581be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeather`'s vtable slot +0x18, the `ClearData` slot of the Xbox PDB's `TESForm`
/// (`ECX`): empties the sound list (`00581f50` on `+0x1f8`).
pub fn tes_weather_clear_data(e: &mut Engine, this: Ptr<TESWeather>) {
    fn_00581f50(e, Ptr::new(this.addr() + TESWeather::soundList.off));
}

// Translated from 00581f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (in this unit's range), `ECX` = a `BSSimpleList` of heap items: from
/// the head node while there is a node with a nonzero item (both read through `006815c0`),
/// deletes the item (`operator delete`) and goes to the next node (`00726070`); then empties
/// the list (`00470470`).
pub fn fn_00581f50(e: &mut Engine, this: Ptr) {
    let mut node = this.addr();
    while node != 0 {
        let item_at = e.call(LIST_ITEM_006815C0, &args![node]).u32();
        if e.mem.u32(item_at) == 0 {
            break;
        }
        let item_at = e.call(LIST_ITEM_006815C0, &args![node]).u32();
        let item = e.mem.u32(item_at);
        e.call(OPERATOR_DELETE, &args![item]);
        node = e.call(NODE_FOLLOWING_00726070, &args![node]).u32();
    }
    e.call(LIST_REMOVE_ALL_00470470, &args![this]);
}

// Translated from 00581b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeather`'s vtable slot +0x88, the `InitItem` slot of the Xbox PDB's `TESForm` (`ECX`):
/// unless form flag 8 is set (`004013e0`), resolves the image space modifiers (`005820a0` on
/// `+0x18` with the weather as owner) and sets form flag 8 (`00484ab0` with 1).
pub fn tes_weather_init_item(e: &mut Engine, this: Ptr<TESWeather>) {
    if e.call(FORM_FLAG_8_SET, &args![this]).bool() {
        return;
    }
    fn_005820a0(
        e,
        Ptr::new(this.addr() + TESWeather::imageSpaceModifiers.off),
        this.cast(),
    );
    e.call(SET_FORM_FLAG_8, &args![this, 1u32]);
}

// Translated from 00581c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeather::MakeDefault` (Xbox PDB), `ECX`: three cloud layers, the three cloud textures
/// (`Sky\WastelandCloudCloudyUpper01.dds`, `...Horizon01.dds`, `...CloudyLower01.dds` through
/// `00489100` on `+0x34`, `+0x40`, `+0x4c`), the first cloud speed, the cloud colours of the
/// first layer, the weather data bytes, the fog distances (`010317c8`, `010317c4`, then 1.0
/// twice) and the colour words, each written in the exe's order.
pub fn tes_weather_make_default(e: &mut Engine, this: Ptr<TESWeather>) {
    let base = this.addr();
    e.set(this, TESWeather::iNumCloudLayers, 3);
    e.call(SET_STRING_00489100, &args![base + 0x34, CLOUD_UPPER_NAME]);
    e.call(SET_STRING_00489100, &args![base + 0x40, CLOUD_HORIZON_NAME]);
    e.call(SET_STRING_00489100, &args![base + 0x4c, CLOUD_LOWER_NAME]);
    e.mem.set_u8(base + 0x64, 0x33);
    for (offset, value) in [
        (0x68u32, 0x005b_4b46u32),
        (0x6c, 0x009b_9b9b),
        (0x70, 0x006c_5953),
        (0x74, 0x001c_140f),
    ] {
        e.mem.set_u32(base + offset, value);
    }
    for (offset, value) in [
        (0xe0u32, 0x33u8),
        (0xe3, 0xff),
        (0xe4, 0xff),
        (0xe5, 0xff),
        (0xe6, 0),
        (0xe7, 0xff),
        (0xe8, 0),
        (0xe9, 0xff),
        (0xea, 0x32),
        (0xeb, 0),
    ] {
        e.mem.set_u8(base + offset, value);
    }
    // FLD float [global]; FSTP float (a copy, signalling NaNs quieted).
    let far = x87_quiet_bits(e.global::<u32>(FOG_FAR_DEFAULT));
    e.mem.set_u32(base + 0xf8, far);
    let far = x87_quiet_bits(e.global::<u32>(FOG_FAR_DEFAULT));
    e.mem.set_u32(base + 0xf0, far);
    let power = x87_quiet_bits(e.global::<u32>(FOG_POWER_DEFAULT));
    e.mem.set_u32(base + 0xfc, power);
    let power = x87_quiet_bits(e.global::<u32>(FOG_POWER_DEFAULT));
    e.mem.set_u32(base + 0xf4, power);
    e.mem.set_u32(base + 0x104, x87_store32(1.0));
    e.mem.set_u32(base + 0x100, x87_store32(1.0));
    for (offset, value) in [
        (0x1b0u32, 0x0084_6c66u32),
        (0x108, 0x0084_6c66),
        (0x1b4, 0x0071_533e),
        (0x10c, 0x0071_533e),
        (0x1c0, 0x0071_533e),
        (0x118, 0x0071_533e),
        (0x1b8, 0x0084_6d66),
        (0x110, 0x0084_6d66),
        (0x1bc, 0x0010_0a05),
        (0x114, 0x0010_0a05),
        (0x1c4, 0x0010_0a05),
        (0x11c, 0x0010_0a05),
        (0x1c8, 0x005f_6db6),
        (0x120, 0x005f_6db6),
        (0x1cc, 0x00d1_9a83),
        (0x124, 0x00d1_9a83),
        (0x1d8, 0x00d1_9a83),
        (0x130, 0x00d1_9a83),
        (0x1d0, 0x0057_82a8),
        (0x128, 0x0057_82a8),
        (0x1d4, 0x0026_1b0f),
        (0x12c, 0x0026_1b0f),
        (0x1dc, 0x0026_1b0f),
        (0x134, 0x0026_1b0f),
        (0x150, 0x0025_2212),
        (0x154, 0x0085_7065),
        (0x160, 0x0085_7065),
        (0x158, 0x0035_221a),
        (0x15c, 0x0028_1c15),
        (0x164, 0x0028_1c15),
        (0x168, 0x0079_a5ff),
        (0x16c, 0x00ca_edff),
        (0x178, 0x00ca_edff),
        (0x170, 0x0049_9ae2),
        (0x174, 0x0076_4d34),
        (0x17c, 0x0076_4d34),
        (0x180, 0x0077_9ff2),
        (0x184, 0x00be_d9de),
        (0x190, 0x00be_d9de),
        (0x188, 0x009d_bdff),
        (0x18c, 0),
        (0x194, 0),
    ] {
        e.mem.set_u32(base + offset, value);
    }
}

/// `00401080` on each of `count` words from `first` (4 bytes apart), with the flag 0.
fn swap_words(e: &mut Engine, first: u32, count: u32) {
    for index in 0..count {
        e.call(SWAP_WORD, &args![first + index * 4, 0u32]);
    }
}

/// Reads a colour table chunk of `rows` rows of six words at `table` (rows 0x18 bytes
/// apart): the full size `rows * 0x18` straight in; the older size `rows * 0x10` (four words
/// a row) through a stack buffer, row by row into the first four words; any other size reads
/// nothing. Then, when the file needs it, every word of the table has its byte order swapped.
fn load_colour_table(e: &mut Engine, file: Ptr, table: u32, rows: u32, buffer: u32) {
    if e.call(CHUNK_SIZE, &args![file]).u32() == rows * 0x18 {
        e.call(GET_CHUNK_DATA_SIZED, &args![file, table, rows * 0x18]);
    } else if e.call(CHUNK_SIZE, &args![file]).u32() == rows * 0x10 {
        e.call(GET_CHUNK_DATA_SIZED, &args![file, buffer, rows * 0x10]);
        for row in 0..rows {
            for column in 0..4u32 {
                let value = e.mem.u32(buffer + row * 0x10 + column * 4);
                e.mem.set_u32(table + row * 0x18 + column * 4, value);
            }
        }
    }
    if e.call(NEEDS_SWAP, &args![file]).bool() {
        for row in 0..rows {
            swap_words(e, table + row * 0x18, 6);
        }
    }
}

/// A modifier chunk: its form id (`GetChunkData` into a zeroed word) stored as modifier
/// `index` (`00581fb0` on `+0x18`).
fn load_modifier(e: &mut Engine, this: u32, file: Ptr, index: u32, word: u32) {
    e.mem.set_u32(word, 0);
    e.call(GET_CHUNK_DATA, &args![file, word]);
    let id = e.mem.u32(word);
    fn_00581fb0(
        e,
        Ptr::new(this + TESWeather::imageSpaceModifiers.off),
        index,
        id,
    );
}

// Translated from 00580990 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00580990 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00ef7b5e, state variable) and the /GS cookie check (00ec408c) are not translated; the EDID buffer from __alloca_probe_16 (00ec5ec0) is a with_stack block of the chunk size (its 16-byte alignment padding, and the stack growing with each EDID chunk, are not modelled)
/// `TESWeather::Load` (Xbox PDB), `ECX` and one stack word, the file (`RET 4`). A record that
/// is not a weather (`TESFile::GetTESForm` not 0x35) answers false. Otherwise
/// `TESForm::LoadForm`, form flag 8 cleared (`00484ab0`), then each chunk until
/// `GetTESChunk` answers 0 or `004726f0` false:
/// - `EDID`: a stack buffer of the chunk's size, read with at most 0x200 bytes, given to
///   vtable slot +0x134; `OBND`: vtable slot +0xe0 with the file; `MODL`: the precipitation
///   model (`TESModel::LoadModelChunk` on `+0xc8`).
/// - `\0IAD` to `\x06IAD` and `MNAM`: a form id stored as modifier 0 to 6 (`MNAM` as 1).
/// - `ANAM`, `BNAM`, `CNAM`, `DNAM`, `ENAM`: `TESTexture::LoadTextureChunk` on `+0x4c`,
///   `+0x58`, `+0x40`, `+0x34`, `+0x64`; `ONAM`: 4 bytes into `+0x64`.
/// - `PNAM` (cloud colours, four rows, `+0x68`) and `NAM0` (colours, ten rows, `+0x108`):
///   [`load_colour_table`].
/// - `DATA`: 15 bytes into `+0xe0`, or 12 bytes with the three after them set to 0xff; other
///   sizes read nothing.
/// - `FNAM`: 0x18 bytes of fog data (`+0xf0`), byte-swapped (when the file needs it) for six
///   words, or four when the chunk is not 0x18 bytes.
/// - `HNAM`: 0x38 bytes of HDR data (`+0x200`, 14 words swapped); `INAM`: 0x130 bytes of the
///   two image space parameter sets (`+0x238`, each through `ImageSpaceParameterData::Endian`);
///   `LNAM`: the cloud layer count (`+0x368`, swapped).
/// - `SNAM`: an 8-byte sound record (`00401000`, cleared by `005813b0` when the allocation
///   succeeded) read in (swapped through `00462230`), its form id given the file's compile
///   index (`TESForm::AddCompileIndex`) and added to the sound list (`00905820` on `+0x1f8`
///   with the address of a stack copy of the record pointer).
///
/// Then true.
pub fn tes_weather_load(e: &mut Engine, this: Ptr<TESWeather>, file: Ptr) -> bool {
    let form_type = e.call(GET_TES_FORM, &args![file]).u32() as u8;
    if form_type != FORM_TYPE_WEATHER {
        return false;
    }
    let base = this.addr();
    e.call(LOAD_FORM, &args![this, file]);
    e.call(SET_FORM_FLAG_8, &args![this, 0u32]);
    e.with_stack(0xb0, |e, locals| {
        // The modifier words, the cloud colour buffer (0x40) and the colour buffer (0xa0)
        // share the block: they are never live at once.
        let word = locals.addr();
        let buffer = locals.addr() + 0x10;
        loop {
            let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
            if chunk == 0 {
                break;
            }
            match chunk {
                CHUNK_ANAM => {
                    e.call(LOAD_TEXTURE_CHUNK, &args![base + 0x4c, file]);
                }
                CHUNK_IAD_0..=0x4441_4906 => {
                    load_modifier(e, base, file, chunk - CHUNK_IAD_0, word);
                }
                CHUNK_MNAM => load_modifier(e, base, file, 1, word),
                CHUNK_NAM0 => load_colour_table(e, file, base + 0x108, 10, buffer),
                CHUNK_DATA => {
                    if e.call(CHUNK_SIZE, &args![file]).u32() == 0xf {
                        e.call(GET_CHUNK_DATA_SIZED, &args![file, base + 0xe0, 0xfu32]);
                    } else if e.call(CHUNK_SIZE, &args![file]).u32() == 0xc {
                        e.call(GET_CHUNK_DATA_SIZED, &args![file, base + 0xe0, 0xcu32]);
                        e.call(MEMSET, &args![base + 0xec, 0xffu32, 3u32]);
                    }
                }
                CHUNK_EDID => {
                    let size = e.call(CHUNK_SIZE, &args![file]).u32();
                    e.with_stack(size, |e, name| {
                        e.call(GET_CHUNK_DATA_SIZED, &args![file, name, 0x200u32]);
                        e.vcall(base, 0x134, &args![name]);
                    });
                }
                CHUNK_OBND => {
                    e.vcall(base, 0xe0, &args![file]);
                }
                CHUNK_MODL => {
                    e.call(LOAD_MODEL_CHUNK, &args![base + 0xc8, file]);
                }
                CHUNK_BNAM => {
                    e.call(LOAD_TEXTURE_CHUNK, &args![base + 0x58, file]);
                }
                CHUNK_CNAM => {
                    e.call(LOAD_TEXTURE_CHUNK, &args![base + 0x40, file]);
                }
                CHUNK_DNAM => {
                    e.call(LOAD_TEXTURE_CHUNK, &args![base + 0x34, file]);
                }
                CHUNK_ENAM => {
                    e.call(LOAD_TEXTURE_CHUNK, &args![base + 0x64, file]);
                }
                CHUNK_FNAM => {
                    let mut count = 6u32;
                    if e.call(CHUNK_SIZE, &args![file]).u32() != 0x18 {
                        count -= 2;
                    }
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, base + 0xf0, 0x18u32]);
                    if e.call(NEEDS_SWAP, &args![file]).bool() {
                        swap_words(e, base + 0xf0, count);
                    }
                }
                CHUNK_HNAM => {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, base + 0x200, 0x38u32]);
                    if e.call(NEEDS_SWAP, &args![file]).bool() {
                        swap_words(e, base + 0x200, 0xe);
                    }
                }
                CHUNK_INAM => {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, base + 0x238, 0x130u32]);
                    if e.call(NEEDS_SWAP, &args![file]).bool() {
                        for set in 0..2u32 {
                            e.call(
                                IMAGE_SPACE_PARAMETERS_ENDIAN,
                                &args![base + 0x238 + set * 0x98],
                            );
                        }
                    }
                }
                CHUNK_LNAM => {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, base + 0x368, 4u32]);
                    if e.call(NEEDS_SWAP, &args![file]).bool() {
                        e.call(SWAP_WORD, &args![base + 0x368, 0u32]);
                    }
                }
                CHUNK_ONAM => {
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, base + 0x64, 4u32]);
                }
                CHUNK_PNAM => load_colour_table(e, file, base + 0x68, 4, buffer),
                CHUNK_SNAM => {
                    let block = e.call(ALLOCATE, &args![8u32]).u32();
                    let sound = if block == 0 {
                        0
                    } else {
                        fn_005813b0(e, Ptr::new(block)).addr()
                    };
                    e.mem.set_u32(word, sound);
                    e.call(GET_CHUNK_DATA_SIZED, &args![file, sound, 8u32]);
                    if e.call(NEEDS_SWAP, &args![file]).bool() {
                        e.call(SWAP_SOUND_00462230, &args![sound]);
                    }
                    e.call(ADD_COMPILE_INDEX, &args![sound, file]);
                    e.call(LIST_ADD_00905820, &args![base + 0x1f8, word]);
                }
                _ => {}
            }
            if !e.call(NEXT_CHUNK, &args![file]).bool() {
                break;
            }
        }
    });
    true
}

/// `TESForm::StartForm` (engine map, `004855a0`, `ECX` = form).
const START_FORM: u32 = 0x0048_55a0;
/// `TESForm::CloseForm` (engine map, `00485680`, `ECX` = form).
const CLOSE_FORM: u32 = 0x0048_5680;
/// `TESTexture::Save` (Xbox PDB, `0048e370`, `ECX` = texture, one stack word: the chunk).
const TEXTURE_SAVE: u32 = 0x0048_e370;
/// `TESModel::Save` (engine map, `004891d0`, `ECX` = model, two stack words: the model and
/// texture chunks).
const MODEL_SAVE: u32 = 0x0048_91d0;
/// `00485910` (`cdecl`: chunk, word; `tesform.cpp` in the engine map): adds a one-word chunk.
const ADD_WORD_CHUNK: u32 = 0x0048_5910;
/// `TESForm::AddChunkArray` (engine map, `004856f0`, `cdecl`: chunk, data, size).
const ADD_CHUNK_ARRAY: u32 = 0x0048_56f0;
/// `TESForm::AddChunkArray_ov2` (engine map, `00485710`, `cdecl`: chunk, data, count).
const ADD_CHUNK_ARRAY_OV2: u32 = 0x0048_5710;
/// `TESForm::__AddChunkData` (engine map, `00485990`, `cdecl`: chunk, data, size).
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `00401500` (`cdecl`, no words): the byte at `011c54ba`, whether saved data has its byte
/// order swapped.
const SWAPS_BYTE_ORDER: u32 = 0x0040_1500;
/// `TESForm::CopyAllComponents` (engine map, `004851b0`, `ECX` = form, one stack word).
const COPY_ALL_COMPONENTS: u32 = 0x0048_51b0;
/// `TESForm::CompareAllComponents` (engine map, `00485270`, `ECX` = form, one stack word).
const COMPARE_ALL_COMPONENTS: u32 = 0x0048_5270;
/// The game's `memcpy` wrapper (`00401460`, `cdecl`: destination, source, size).
const MEMCPY: u32 = 0x0040_1460;
/// `_memcmp` (engine map, `00ec4835`, `cdecl`).
const MEMCMP: u32 = 0x00ec_4835;
/// `004b0680` (`ECX` = new record, `RET 8`: two words; `bsanimnotereceiver.cpp` in the engine
/// map): builds a weather sound record from the source record's two words.
const SOUND_RECORD_CTOR: u32 = 0x004b_0680;
/// `VATS::GetCount` in the engine map (`005ae380`, `ECX` = list; a folded body): the list's
/// item count.
const LIST_COUNT: u32 = 0x005a_e380;
/// RTTI type descriptor `.?AVTESWeather@@`.
const TYPE_TES_WEATHER: u32 = 0x0118_629c;
/// RTTI type descriptors `.?AVBaseFormComponent@@` and
/// `.?AV?$TESImageSpaceModifiableCountForm@$05@@`.
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
const TYPE_MODIFIABLE_COUNT_FORM: u32 = 0x0118_6ab0;
/// `005822a0` (`ECX`, `RET 4`: an index): modifier `index` of a
/// `TESImageSpaceModifiableCountForm<6>` (`+4 + index * 4`).
const GET_MODIFIER: u32 = 0x0058_22a0;

// Translated from 005813d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESWeather::Save` (Xbox PDB; `ECX`, plain `RET`): writes the weather record: the image
/// space modifiers (`00582040` on `+0x18`); the cloud textures (`TESTexture::Save` on `+0x34 +
/// i * 0xc` with `DNAM`, `CNAM`, `ANAM`, `BNAM`, `ENAM`, `GNAM`, `JNAM`, `KNAM` from a stack
/// table, for each of `iNumCloudLayers`); the precipitation model (`MODL`/`MODT`); `LNAM`
/// (the layer count), `ONAM` (cloud speeds, 4 bytes), `PNAM` (cloud colours), `NAM0`
/// (colours), `FNAM` (fog); `INAM` (the two image space parameter blocks, byte-swapped before
/// and after when `00401500` says so); `DATA` (15 bytes); an `SNAM` (8 bytes) per sound until
/// an empty item, each swapped (`00462230`) before and after under the same test.
/// A layer count above 8 reads past the stack table, as the exe does.
pub fn tes_weather_save(e: &mut Engine, this: Ptr<TESWeather>) {
    let base = this.addr();
    e.call(START_FORM, &args![this]);
    tes_image_space_modifiable_count_form_save(e, Ptr::new(base.wrapping_add(0x18)));
    let tags = [
        CHUNK_DNAM,
        CHUNK_CNAM,
        CHUNK_ANAM,
        CHUNK_BNAM,
        CHUNK_ENAM,
        0x4d41_4e47,
        0x4d41_4e4a,
        0x4d41_4e4b,
    ];
    e.with_stack(0x20, |e, table| {
        for (i, tag) in tags.iter().enumerate() {
            e.mem.set_u32(table.addr() + 4 * i as u32, *tag);
        }
        let mut layer = 0i32;
        while layer < e.mem.i32(base.wrapping_add(0x368)) {
            let tag = e
                .mem
                .u32(table.addr().wrapping_add((layer as u32).wrapping_mul(4)));
            let texture = base
                .wrapping_add((layer as u32).wrapping_mul(0xc))
                .wrapping_add(0x34);
            e.call(TEXTURE_SAVE, &args![texture, tag]);
            layer += 1;
        }
    });
    e.call(
        MODEL_SAVE,
        &args![base.wrapping_add(0xc8), CHUNK_MODL, 0x5444_4f4du32],
    );
    let layers = e.mem.u32(base.wrapping_add(0x368));
    e.call(ADD_WORD_CHUNK, &args![CHUNK_LNAM, layers]);
    e.call(
        ADD_CHUNK_ARRAY,
        &args![CHUNK_ONAM, base.wrapping_add(0x64), 4u32],
    );
    e.call(
        ADD_CHUNK_ARRAY_OV2,
        &args![CHUNK_PNAM, base.wrapping_add(0x68), 0x18u32],
    );
    e.call(
        ADD_CHUNK_ARRAY_OV2,
        &args![CHUNK_NAM0, base.wrapping_add(0x108), 0x3cu32],
    );
    e.call(
        ADD_CHUNK_ARRAY_OV2,
        &args![CHUNK_FNAM, base.wrapping_add(0xf0), 6u32],
    );
    swap_parameter_blocks(e, base);
    e.call(
        ADD_CHUNK_DATA,
        &args![CHUNK_INAM, base.wrapping_add(0x238), 0x130u32],
    );
    swap_parameter_blocks(e, base);
    e.call(
        ADD_CHUNK_ARRAY,
        &args![CHUNK_DATA, base.wrapping_add(0xe0), 0xfu32],
    );
    let mut node = base.wrapping_add(0x1f8);
    while node != 0 {
        let item = e.call(LIST_ITEM_006815C0, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        swap_sound(e, node);
        let item = e.call(LIST_ITEM_006815C0, &args![node]).u32();
        let sound = e.mem.u32(item);
        e.call(ADD_CHUNK_DATA, &args![CHUNK_SNAM, sound, 8u32]);
        swap_sound(e, node);
        node = e.call(NODE_FOLLOWING_00726070, &args![node]).u32();
    }
    e.call(CLOSE_FORM, &args![this]);
}

/// When saved data is byte-swapped (`00401500`): `ImageSpaceParameterData::Endian` on both
/// parameter blocks (`+0x238`, 0x98 bytes each).
fn swap_parameter_blocks(e: &mut Engine, base: u32) {
    if !e.call(SWAPS_BYTE_ORDER, &[]).bool() {
        return;
    }
    let mut index = 0u32;
    while index < 2 {
        let block = base
            .wrapping_add(index.wrapping_mul(0x98))
            .wrapping_add(0x238);
        e.call(IMAGE_SPACE_PARAMETERS_ENDIAN, &args![block]);
        index += 1;
    }
}

/// When saved data is byte-swapped (`00401500`): `00462230` on the node's sound record.
fn swap_sound(e: &mut Engine, node: u32) {
    if e.call(SWAPS_BYTE_ORDER, &[]).bool() {
        let item = e.call(LIST_ITEM_006815C0, &args![node]).u32();
        let sound = e.mem.u32(item);
        e.call(SWAP_SOUND_00462230, &args![sound]);
    }
}

// Translated from 00581630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00581630 - the FS:[0] exception frame (unwinding the sound record allocation) is not translated.
/// Unnamed in the map (`RET 4`: a form; the weather's copy by its body): when the form is a
/// `TESWeather` (`__RTDynamicCast` from `TESForm`), copies it: all components
/// (`TESForm::CopyAllComponents`), the layer count, each cloud texture and the precipitation
/// model through their vtable slot +8, the cloud speeds, cloud colours, weather data, colours,
/// fog and image space parameter blocks (`00401460`), then rebuilds the sound list: emptied
/// (`00470470`), each source sound (until an empty item) copied into a new 8-byte record
/// (`00401000`, `004b0680` with its two words; null when the allocation fails) and added
/// (`00905820`). The exception frame around the record's construction is not translated.
pub fn fn_00581630(e: &mut Engine, this: Ptr<TESWeather>, form: u32) {
    let base = this.addr();
    let source = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_WEATHER, 0u32],
        )
        .u32();
    if source == 0 {
        return;
    }
    e.call(COPY_ALL_COMPONENTS, &args![this, source]);
    let layers = e.mem.u32(source.wrapping_add(0x368));
    e.mem.set_u32(base.wrapping_add(0x368), layers);
    let mut layer = 0i32;
    while layer < e.mem.i32(base.wrapping_add(0x368)) {
        let offset = (layer as u32).wrapping_mul(0xc).wrapping_add(0x34);
        e.vcall(
            base.wrapping_add(offset),
            8,
            &args![source.wrapping_add(offset)],
        );
        layer += 1;
    }
    e.vcall(
        base.wrapping_add(0xc8),
        8,
        &args![source.wrapping_add(0xc8)],
    );
    for (offset, size) in [
        (0x64u32, 4u32),
        (0x68, 0x60),
        (0xe0, 0xf),
        (0x108, 0xf0),
        (0xf0, 0x18),
        (0x238, 0x130),
    ] {
        e.call(
            MEMCPY,
            &args![base.wrapping_add(offset), source.wrapping_add(offset), size],
        );
    }
    e.call(LIST_REMOVE_ALL_00470470, &args![base.wrapping_add(0x1f8)]);
    let mut node = source.wrapping_add(0x1f8);
    while node != 0 {
        let item = e.call(LIST_ITEM_006815C0, &args![node]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let memory = e.call(ALLOCATE, &args![8u32]).u32();
        let record = if memory != 0 {
            let item = e.call(LIST_ITEM_006815C0, &args![node]).u32();
            let second = e.mem.u32(e.mem.u32(item).wrapping_add(4));
            let item = e.call(LIST_ITEM_006815C0, &args![node]).u32();
            let first = e.mem.u32(e.mem.u32(item));
            e.call(SOUND_RECORD_CTOR, &args![memory, first, second])
                .u32()
        } else {
            0
        };
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), record);
            e.call(LIST_ADD_00905820, &args![base.wrapping_add(0x1f8), slot]);
        });
        node = e.call(NODE_FOLLOWING_00726070, &args![node]).u32();
    }
}

// Translated from 00581880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the map (`RET 4`: a form; the weather's comparison by its body): true
/// ("different") unless the form is a `TESWeather` with as many sounds (`005ae380`), the same
/// 8 sound bytes pairwise (`_memcmp`, until either list ends or holds an empty item), the
/// same layer count, cloud textures and precipitation model the same (vtable slot +0xc false),
/// `TESForm::CompareAllComponents` false, and equal weather data (0xf bytes), colours (0xf0),
/// fog (0x18), cloud speeds (4), cloud colours (0x60) and image space parameters (0x130).
pub fn fn_00581880(e: &mut Engine, this: Ptr<TESWeather>, form: u32) -> bool {
    let base = this.addr();
    let source = e
        .call(
            DYNAMIC_CAST,
            &args![form, 0u32, TYPE_TES_FORM, TYPE_TES_WEATHER, 0u32],
        )
        .u32();
    if source == 0 {
        return true;
    }
    let mut theirs = source.wrapping_add(0x1f8);
    let mut mine = base.wrapping_add(0x1f8);
    let their_count = e.call(LIST_COUNT, &args![theirs]).u32();
    if their_count != e.call(LIST_COUNT, &args![mine]).u32() {
        return true;
    }
    while theirs != 0 && mine != 0 {
        let item = e.call(LIST_ITEM_006815C0, &args![theirs]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(LIST_ITEM_006815C0, &args![mine]).u32();
        if e.mem.u32(item) == 0 {
            break;
        }
        let item = e.call(LIST_ITEM_006815C0, &args![mine]).u32();
        let my_sound = e.mem.u32(item);
        let item = e.call(LIST_ITEM_006815C0, &args![theirs]).u32();
        let their_sound = e.mem.u32(item);
        if e.call(MEMCMP, &args![their_sound, my_sound, 8u32]).u32() != 0 {
            return true;
        }
        theirs = e.call(NODE_FOLLOWING_00726070, &args![theirs]).u32();
        mine = e.call(NODE_FOLLOWING_00726070, &args![mine]).u32();
    }
    if e.mem.u32(base.wrapping_add(0x368)) != e.mem.u32(source.wrapping_add(0x368)) {
        return true;
    }
    let mut layer = 0i32;
    while layer < e.mem.i32(base.wrapping_add(0x368)) {
        let offset = (layer as u32).wrapping_mul(0xc).wrapping_add(0x34);
        if e.vcall(
            base.wrapping_add(offset),
            0xc,
            &args![source.wrapping_add(offset)],
        )
        .bool()
        {
            return true;
        }
        layer += 1;
    }
    if e.call(COMPARE_ALL_COMPONENTS, &args![this, source]).bool() {
        return true;
    }
    if e.vcall(
        base.wrapping_add(0xc8),
        0xc,
        &args![source.wrapping_add(0xc8)],
    )
    .bool()
    {
        return true;
    }
    for (offset, size) in [
        (0xe0u32, 0xfu32),
        (0x108, 0xf0),
        (0xf0, 0x18),
        (0x64, 4),
        (0x68, 0x60),
        (0x238, 0x130),
    ] {
        let differs = e
            .call(
                MEMCMP,
                &args![base.wrapping_add(offset), source.wrapping_add(offset), size],
            )
            .u32();
        if differs != 0 {
            return true;
        }
    }
    false
}

// Translated from 00582040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifiableCountForm<6>::Save` (engine map; `ECX`, plain `RET`): for each of
/// the six modifiers that is set, adds a chunk `"\0IAD"` + index holding its form id
/// (`0084e3a0`, `00485910`).
pub fn tes_image_space_modifiable_count_form_save(e: &mut Engine, this: Ptr) {
    let mut index = 0u32;
    while index < 6 {
        let modifier = e.mem.u32(
            this.addr()
                .wrapping_add(index.wrapping_mul(4))
                .wrapping_add(4),
        );
        if modifier != 0 {
            let id = e.call(FORM_ID_0084E3A0, &args![modifier]).u32();
            e.call(ADD_WORD_CHUNK, &args![index.wrapping_add(CHUNK_IAD_0), id]);
        }
        index += 1;
    }
}

/// The other component cast to `TESImageSpaceModifiableCountForm<6>` (from
/// `BaseFormComponent`), or 0.
fn as_modifiable_count_form(e: &mut Engine, other: u32) -> u32 {
    e.call(
        DYNAMIC_CAST,
        &args![
            other,
            0u32,
            TYPE_BASE_FORM_COMPONENT,
            TYPE_MODIFIABLE_COUNT_FORM,
            0u32
        ],
    )
    .u32()
}

// Translated from 005821b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifiableCountForm<6>::CopyComponent` (engine map, `RET 4`: the other
/// component): when it is one, copies its six modifiers (`005822a0`, then `00581fb0`).
pub fn tes_image_space_modifiable_count_form_copy_component(e: &mut Engine, this: Ptr, other: u32) {
    let source = as_modifiable_count_form(e, other);
    if source == 0 {
        return;
    }
    let mut index = 0u32;
    while index < 6 {
        let modifier = e.call(GET_MODIFIER, &args![source, index]).u32();
        fn_00581fb0(e, this, index, modifier);
        index += 1;
    }
}

// Translated from 00582220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESImageSpaceModifiableCountForm<6>::CompareComponent` (engine map, `RET 4`: the other
/// component): true ("different") unless it is one with the same six modifiers (`005822a0`
/// on each, this first).
pub fn tes_image_space_modifiable_count_form_compare_component(
    e: &mut Engine,
    this: Ptr,
    other: u32,
) -> bool {
    let source = as_modifiable_count_form(e, other);
    if source == 0 {
        return true;
    }
    let mut index = 0u32;
    while index < 6 {
        let mine = e.call(GET_MODIFIER, &args![this, index]).u32();
        if mine != e.call(GET_MODIFIER, &args![source, index]).u32() {
            return true;
        }
        index += 1;
    }
    false
}
/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0058_1fd0, fn_00581fd0(Ptr) -> Ptr),
        entry!(
            0x0058_2000,
            tes_image_space_modifiable_count_form_initialize_data_component(Ptr)
        ),
        entry!(0x0058_1fb0, fn_00581fb0(Ptr, u32, u32)),
        entry!(0x0058_20a0, fn_005820a0(Ptr, Ptr)),
        entry!(0x0058_13b0, fn_005813b0(Ptr) -> Ptr),
        entry!(0x0058_1b40, tes_weather_initialize_data(Ptr<TESWeather>)),
        entry!(0x0058_1be0, tes_weather_clear_data(Ptr<TESWeather>)),
        entry!(0x0058_1f50, fn_00581f50(Ptr)),
        entry!(0x0058_1b00, tes_weather_init_item(Ptr<TESWeather>)),
        entry!(0x0058_1c00, tes_weather_make_default(Ptr<TESWeather>)),
        entry!(0x0058_0990, tes_weather_load(Ptr<TESWeather>, Ptr) -> bool),
        entry!(0x0058_13d0, tes_weather_save(Ptr<TESWeather>)),
        entry!(0x0058_1630, fn_00581630(Ptr<TESWeather>, u32)),
        entry!(0x0058_1880, fn_00581880(Ptr<TESWeather>, u32) -> bool),
        entry!(0x0058_2040, tes_image_space_modifiable_count_form_save(Ptr)),
        entry!(
            0x0058_21b0,
            tes_image_space_modifiable_count_form_copy_component(Ptr, u32)
        ),
        entry!(
            0x0058_2220,
            tes_image_space_modifiable_count_form_compare_component(Ptr, u32) -> bool
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn memset_double(e: &mut Engine) {
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            a[0].into_ret()
        });
    }

    #[test]
    fn fn_00581fd0_constructs_the_count_form() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        e.mem.write(this, &[0xaa; 0x1c]);
        e.register(BASE_FORM_COMPONENT_CTOR, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0058_1fd0, &args![this]).u32(), this);
        assert_eq!(calls(&e, BASE_FORM_COMPONENT_CTOR), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), MODIFIABLE_COUNT_FORM_VTABLE);
        assert_eq!(e.mem.bytes(this + 4, 0x18), vec![0; 0x18]);
    }

    #[test]
    fn fn_00582000_and_00581fb0_clear_and_set_modifiers() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x20);
        e.mem.write(this, &[0xaa; 0x20]);
        e.call(0x0058_2000, &args![this]);
        assert_eq!(e.mem.u32(this), 0xaaaa_aaaa);
        assert_eq!(e.mem.bytes(this + 4, 0x18), vec![0; 0x18]);
        assert_eq!(e.mem.u32(this + 0x1c), 0xaaaa_aaaa);
        e.call(0x0058_1fb0, &args![this, 5u32, 0x1234u32]);
        assert_eq!(e.mem.u32(this + 0x18), 0x1234);
    }

    /// A count form with modifiers 0 (id 0x10) and 2 (id 0x30); id 0x30 has no form.
    fn resolve_rig() -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x1c);
        e.mem.set_u32(this + 4, 0x10);
        e.mem.set_u32(this + 0xc, 0x30);
        e.register(GET_FILE, |_, _| 0x0777u32.into_ret());
        // The compile index adds 0x0100_0000.
        e.register(ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 0x0100_0000);
            Ret::default()
        });
        e.register(LOOKUP_FORM, |_, a| {
            if a[0] & 0xff == 0x10 {
                (0x0900_0000 + a[0]).into_ret()
            } else {
                0u32.into_ret()
            }
        });
        e.register(DYNAMIC_CAST, |_, a| {
            if a[0] == 0 {
                0u32.into_ret()
            } else {
                (a[0] + 4).into_ret()
            }
        });
        e.register(LOG, |_, _| Ret::default());
        e.register(NAME_LENGTH_00474CB0, |_, _| 3u32.into_ret());
        e.register(FORM_ID_0084E3A0, |_, _| 0x0abcu32.into_ret());
        e.put_vtable(0x0f00_4000, &[0; 0x4d]);
        e.mem.set_u32(0x0f00_4130, 0x0f00_4500);
        e.register(0x0f00_4500, |_, _| 0x0f00_7000u32.into_ret());
        let owner = e.mem.alloc(0x18);
        e.mem.set_u32(owner, 0x0f00_4000);
        e.call_log = Some(vec![]);
        (e, this, owner)
    }

    #[test]
    fn fn_005820a0_resolves_with_an_owner() {
        let (mut e, this, owner) = resolve_rig();
        e.call(0x0058_20a0, &args![this, owner]);
        assert_eq!(e.mem.u32(this + 4), 0x0900_0000 + 0x0100_0010 + 4);
        assert_eq!(e.mem.u32(this + 8), 0);
        assert_eq!(e.mem.u32(this + 0xc), 0);
        assert_eq!(calls(&e, GET_FILE), vec![vec![owner, u32::MAX]; 2]);
        assert_eq!(
            calls(&e, DYNAMIC_CAST)[0],
            vec![
                0x0900_0000 + 0x0100_0010,
                0,
                TYPE_TES_FORM,
                TYPE_IMAGE_SPACE_MODIFIER,
                0
            ]
        );
        assert_eq!(
            calls(&e, LOG),
            vec![vec![MESSAGE_NAMED_OWNER, 0x0100_0030, 0x0f00_7000]]
        );
    }

    #[test]
    fn fn_005820a0_reports_an_unnamed_or_unknown_owner() {
        let (mut e, this, owner) = resolve_rig();
        e.register(NAME_LENGTH_00474CB0, |_, _| 0u32.into_ret());
        e.call(0x0058_20a0, &args![this, owner]);
        assert_eq!(
            calls(&e, LOG),
            vec![vec![MESSAGE_OWNER_ID, 0x0100_0030, 0x0abc]]
        );
        let (mut e, this, _) = resolve_rig();
        e.call(0x0058_20a0, &args![this, 0u32]);
        assert!(calls(&e, GET_FILE).is_empty());
        assert_eq!(e.mem.u32(this + 4), 0x0900_0010 + 4);
        assert_eq!(calls(&e, LOG), vec![vec![MESSAGE_UNKNOWN_OWNER, 0x30]]);
    }

    #[test]
    fn fn_005813b0_clears_eight_bytes() {
        let mut e = Engine::new();
        memset_double(&mut e);
        let this = e.mem.alloc(0x10);
        e.mem.write(this, &[0xaa; 0x10]);
        assert_eq!(e.call(0x0058_13b0, &args![this]).u32(), this);
        assert_eq!(e.mem.bytes(this, 8), vec![0; 8]);
        assert_eq!(e.mem.u32(this + 8), 0xaaaa_aaaa);
    }

    #[test]
    fn fn_00581b40_initializes_the_data() {
        let mut e = Engine::new();
        memset_double(&mut e);
        let this = e.mem.alloc(0x374);
        e.mem.write(this, &[0xaa; 0x374]);
        e.call(0x0058_1b40, &args![this]);
        assert_eq!(e.mem.u32(this + 0x368), 3);
        assert_eq!(e.mem.bytes(this + 0x108, 0xf0), vec![0; 0xf0]);
        assert_eq!(e.mem.bytes(this + 0xe0, 0xf), vec![0; 0xf]);
        assert_eq!(e.mem.u8(this + 0xef), 0xaa);
        assert_eq!(e.mem.bytes(this + 0xf0, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.f32(this + 0x100), 1.0);
        assert_eq!(e.mem.f32(this + 0x104), 1.0);
        assert_eq!(e.mem.bytes(this + 0x64, 0x64), vec![0; 0x64]);
        assert_eq!(e.mem.u32(this + 0x60), 0xaaaa_aaaa);
    }

    #[test]
    fn fn_00581be0_and_00581f50_free_every_item_then_empty_the_list() {
        let mut e = Engine::new();
        e.register(LIST_ITEM_006815C0, |_, a| a[0].into_ret());
        e.register(NODE_FOLLOWING_00726070, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(OPERATOR_DELETE, |_, _| Ret::default());
        e.register(LIST_REMOVE_ALL_00470470, |_, _| Ret::default());
        let this = e.mem.alloc(0x374);
        // Head (item 0x111) -> node (item 0x222) -> node (item 0, stops).
        let second = e.mem.alloc(8);
        let third = e.mem.alloc(8);
        e.mem.set_u32(this + 0x1f8, 0x111);
        e.mem.set_u32(this + 0x1fc, second);
        e.mem.set_u32(second, 0x222);
        e.mem.set_u32(second + 4, third);
        e.call_log = Some(vec![]);
        e.call(0x0058_1be0, &args![this]);
        assert_eq!(calls(&e, OPERATOR_DELETE), vec![vec![0x111], vec![0x222]]);
        assert_eq!(
            calls(&e, LIST_REMOVE_ALL_00470470),
            vec![vec![this + 0x1f8]]
        );
        // A list whose last node points nowhere ends at the null node.
        e.mem.set_u32(second + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x0058_1f50, &args![this + 0x1f8]);
        assert_eq!(calls(&e, OPERATOR_DELETE).len(), 2);
        assert_eq!(calls(&e, NODE_FOLLOWING_00726070).len(), 2);
    }

    #[test]
    fn fn_00581b00_resolves_once() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x374);
        e.register(FORM_FLAG_8_SET, |_, _| false.into_ret());
        e.register(SET_FORM_FLAG_8, |_, _| Ret::default());
        // One modifier id whose lookup answers nothing, reported with the unnamed owner.
        e.mem.set_u32(this + 0x1c, 0x55);
        e.register(GET_FILE, |_, _| 0u32.into_ret());
        e.register(ADD_COMPILE_INDEX, |_, _| Ret::default());
        e.register(LOOKUP_FORM, |_, _| 0u32.into_ret());
        e.register(DYNAMIC_CAST, |_, _| 0u32.into_ret());
        e.register(NAME_LENGTH_00474CB0, |_, _| 0u32.into_ret());
        e.register(FORM_ID_0084E3A0, |_, _| 0x99u32.into_ret());
        e.register(LOG, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0058_1b00, &args![this]);
        assert_eq!(calls(&e, GET_FILE), vec![vec![this, u32::MAX]]);
        assert_eq!(calls(&e, LOG), vec![vec![MESSAGE_OWNER_ID, 0x55, 0x99]]);
        assert_eq!(calls(&e, SET_FORM_FLAG_8), vec![vec![this, 1]]);
        e.register(FORM_FLAG_8_SET, |_, _| true.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x0058_1b00, &args![this]);
        assert!(calls(&e, GET_FILE).is_empty());
        assert!(calls(&e, SET_FORM_FLAG_8).is_empty());
    }

    #[test]
    fn fn_00581c00_makes_the_default_weather() {
        let mut e = Engine::new();
        e.map(0x0103_1000, 0x1000);
        e.mem.set_u32(FOG_FAR_DEFAULT, 0x46d9_999a);
        e.mem.set_u32(FOG_POWER_DEFAULT, 0x4820_0000);
        e.register(SET_STRING_00489100, |_, _| Ret::default());
        let this = e.mem.alloc(0x374);
        e.call_log = Some(vec![]);
        e.call(0x0058_1c00, &args![this]);
        assert_eq!(
            calls(&e, SET_STRING_00489100),
            vec![
                vec![this + 0x34, CLOUD_UPPER_NAME],
                vec![this + 0x40, CLOUD_HORIZON_NAME],
                vec![this + 0x4c, CLOUD_LOWER_NAME]
            ]
        );
        assert_eq!(e.mem.u32(this + 0x368), 3);
        assert_eq!(e.mem.u8(this + 0x64), 0x33);
        assert_eq!(e.mem.u32(this + 0x74), 0x001c_140f);
        assert_eq!(
            e.mem.bytes(this + 0xe0, 0xc),
            vec![0x33, 0, 0, 0xff, 0xff, 0xff, 0, 0xff, 0, 0xff, 0x32, 0]
        );
        assert_eq!(e.mem.u32(this + 0xf0), 0x46d9_999a);
        assert_eq!(e.mem.u32(this + 0xf8), 0x46d9_999a);
        assert_eq!(e.mem.u32(this + 0xf4), 0x4820_0000);
        assert_eq!(e.mem.u32(this + 0xfc), 0x4820_0000);
        assert_eq!(e.mem.f32(this + 0x100), 1.0);
        assert_eq!(e.mem.u32(this + 0x108), 0x0084_6c66);
        assert_eq!(e.mem.u32(this + 0x1dc), 0x0026_1b0f);
        assert_eq!(e.mem.u32(this + 0x188), 0x009d_bdff);
        assert_eq!(e.mem.u32(this + 0x194), 0);
    }

    #[test]
    fn fn_00581c00_quiets_a_signalling_fog_default() {
        let mut e = Engine::new();
        e.map(0x0103_1000, 0x1000);
        e.mem.set_u32(FOG_FAR_DEFAULT, 0x7f80_0001);
        e.register(SET_STRING_00489100, |_, _| Ret::default());
        let this = e.mem.alloc(0x374);
        e.call(0x0058_1c00, &args![this]);
        assert_eq!(e.mem.u32(this + 0xf0), 0x7fc0_0001);
    }

    type Script = Rc<RefCell<(Vec<(u32, u32)>, usize)>>;

    /// A file double: `chunks` (code, size) one after the other; `swap` for the byte-order
    /// test; chunk data reads fill the destination with 0x11 per byte, up to the size asked.
    fn load_rig(chunks: Vec<(u32, u32)>, swap: bool) -> (Engine, u32, u32) {
        let mut e = Engine::new();
        let script: Script = Rc::new(RefCell::new((chunks, 0)));
        e.register(GET_TES_FORM, |_, _| 0x35u32.into_ret());
        e.register(LOAD_FORM, |_, _| Ret::default());
        e.register(SET_FORM_FLAG_8, |_, _| Ret::default());
        let s = script.clone();
        e.register_double(GET_TES_CHUNK, move |_, _| {
            let s = s.borrow();
            s.0.get(s.1).map_or(0, |c| c.0).into_ret()
        });
        let s = script.clone();
        e.register_double(CHUNK_SIZE, move |_, _| {
            let s = s.borrow();
            s.0[s.1].1.into_ret()
        });
        let s = script;
        e.register_double(NEXT_CHUNK, move |_, _| {
            let mut s = s.borrow_mut();
            s.1 += 1;
            (s.1 < s.0.len()).into_ret()
        });
        e.register(GET_CHUNK_DATA, |e, a| {
            e.mem.set_u32(a[1], 0x0000_0042);
            Ret::default()
        });
        e.register(GET_CHUNK_DATA_SIZED, |e, a| {
            if a[1] != 0 {
                for i in 0..a[2] {
                    e.mem.set_u8(a[1] + i, 0x11);
                }
            }
            Ret::default()
        });
        e.register_double(NEEDS_SWAP, move |_, _| swap.into_ret());
        for addr in [
            SWAP_WORD,
            LOAD_TEXTURE_CHUNK,
            LOAD_MODEL_CHUNK,
            IMAGE_SPACE_PARAMETERS_ENDIAN,
            SWAP_SOUND_00462230,
            ADD_COMPILE_INDEX,
            LIST_ADD_00905820,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        memset_double(&mut e);
        let sound = e.mem.alloc(8);
        e.register_double(ALLOCATE, move |_, _| sound.into_ret());
        e.put_vtable(0x0f00_8000, &[0; 0x4e]);
        e.mem.set_u32(0x0f00_80e0, 0x0f00_8100);
        e.mem.set_u32(0x0f00_8134, 0x0f00_8200);
        e.register(0x0f00_8100, |_, _| Ret::default());
        e.register(0x0f00_8200, |_, _| Ret::default());
        let this = e.mem.alloc(0x374);
        e.mem.set_u32(this, 0x0f00_8000);
        e.call_log = Some(vec![]);
        (e, this, sound)
    }

    const FILE: u32 = 0x0e00_0000;

    #[test]
    fn fn_00580990_refuses_other_records() {
        let (mut e, this, _) = load_rig(vec![], false);
        e.register(GET_TES_FORM, |_, _| 0x34u32.into_ret());
        assert!(!e.call(0x0058_0990, &args![this, FILE]).bool());
        assert!(calls(&e, LOAD_FORM).is_empty());
    }

    #[test]
    fn fn_00580990_reads_modifiers_textures_and_objects() {
        let chunks = vec![
            (CHUNK_EDID, 9),
            (CHUNK_IAD_0, 4),
            (0x4441_4903, 4),
            (0x4441_4906, 4),
            (CHUNK_MNAM, 4),
            (CHUNK_ANAM, 0),
            (CHUNK_BNAM, 0),
            (CHUNK_CNAM, 0),
            (CHUNK_DNAM, 0),
            (CHUNK_ENAM, 0),
            (CHUNK_OBND, 0),
            (CHUNK_MODL, 0),
            (CHUNK_ONAM, 4),
            (0x1234_5678, 4),
        ];
        let (mut e, this, _) = load_rig(chunks, false);
        assert!(e.call(0x0058_0990, &args![this, FILE]).bool());
        assert_eq!(calls(&e, LOAD_FORM), vec![vec![this, FILE]]);
        assert_eq!(calls(&e, SET_FORM_FLAG_8), vec![vec![this, 0]]);
        for index in [0u32, 1, 3, 6] {
            assert_eq!(e.mem.u32(this + 0x1c + index * 4), 0x42, "modifier {index}");
        }
        assert_eq!(e.mem.u32(this + 0x1c + 2 * 4), 0);
        let textures: Vec<u32> = calls(&e, LOAD_TEXTURE_CHUNK)
            .iter()
            .map(|c| c[0] - this)
            .collect();
        assert_eq!(textures, vec![0x4c, 0x58, 0x40, 0x34, 0x64]);
        assert_eq!(calls(&e, 0x0f00_8100), vec![vec![this, FILE]]);
        assert_eq!(calls(&e, LOAD_MODEL_CHUNK), vec![vec![this + 0xc8, FILE]]);
        let edid = calls(&e, 0x0f00_8200);
        assert_eq!(edid.len(), 1);
        let reads = calls(&e, GET_CHUNK_DATA_SIZED);
        assert_eq!(reads[0], vec![FILE, edid[0][1], 0x200]);
        assert_eq!(reads[1], vec![FILE, this + 0x64, 4]);
        assert_eq!(calls(&e, NEXT_CHUNK).len(), 14);
    }

    #[test]
    fn fn_00580990_reads_colours_in_both_sizes_and_swaps() {
        let chunks = vec![
            (CHUNK_PNAM, 0x40),
            (CHUNK_NAM0, 0xf0),
            (CHUNK_DATA, 0xc),
            (CHUNK_FNAM, 0x10),
            (CHUNK_HNAM, 0x38),
            (CHUNK_INAM, 0x130),
            (CHUNK_LNAM, 4),
        ];
        let (mut e, this, _) = load_rig(chunks, true);
        assert!(e.call(0x0058_0990, &args![this, FILE]).bool());
        // PNAM at the old size: four words of each row copied, the rest untouched.
        assert_eq!(e.mem.u32(this + 0x68), 0x1111_1111);
        assert_eq!(e.mem.u32(this + 0x68 + 0xc), 0x1111_1111);
        assert_eq!(e.mem.u32(this + 0x68 + 0x10), 0);
        assert_eq!(e.mem.u32(this + 0x68 + 3 * 0x18 + 0xc), 0x1111_1111);
        assert_eq!(e.mem.u32(this + 0x108 + 0xec), 0x1111_1111);
        // DATA at 12 bytes: three 0xff after it.
        assert_eq!(e.mem.bytes(this + 0xec, 4), vec![0xff, 0xff, 0xff, 0]);
        let swapped: Vec<u32> = calls(&e, SWAP_WORD).iter().map(|c| c[0] - this).collect();
        // 24 cloud colour words, 60 colour words, 4 fog words (the chunk is not 0x18 bytes),
        // 14 HDR words, the layer count.
        assert_eq!(swapped.len(), 24 + 60 + 4 + 14 + 1);
        assert_eq!(swapped[0], 0x68);
        assert_eq!(swapped[23], 0x68 + 3 * 0x18 + 0x14);
        assert_eq!(swapped[84..88], [0xf0, 0xf4, 0xf8, 0xfc]);
        assert_eq!(swapped[88], 0x200);
        assert_eq!(swapped[102], 0x368);
        assert_eq!(
            calls(&e, IMAGE_SPACE_PARAMETERS_ENDIAN),
            vec![vec![this + 0x238], vec![this + 0x2d0]]
        );
        let reads = calls(&e, GET_CHUNK_DATA_SIZED);
        assert_eq!(reads[1], vec![FILE, this + 0x108, 0xf0]);
        assert_eq!(reads[3], vec![FILE, this + 0xf0, 0x18]);
    }

    #[test]
    fn fn_00580990_skips_odd_sizes_and_adds_sounds() {
        let chunks = vec![
            (CHUNK_PNAM, 0x41),
            (CHUNK_NAM0, 0xa0),
            (CHUNK_DATA, 0xf),
            (CHUNK_DATA, 7),
            (CHUNK_FNAM, 0x18),
            (CHUNK_SNAM, 8),
        ];
        let (mut e, this, sound) = load_rig(chunks, false);
        assert!(e.call(0x0058_0990, &args![this, FILE]).bool());
        let reads = calls(&e, GET_CHUNK_DATA_SIZED);
        assert_eq!(reads[0][2], 0xa0);
        assert_eq!(reads[1], vec![FILE, this + 0xe0, 0xf]);
        assert_eq!(reads[2], vec![FILE, this + 0xf0, 0x18]);
        assert_eq!(reads[3], vec![FILE, sound, 8]);
        assert_eq!(e.mem.u32(this + 0x68), 0);
        assert_eq!(e.mem.u32(this + 0x108 + 9 * 0x18 + 0xc), 0x1111_1111);
        assert_eq!(e.mem.u32(this + 0x108 + 0x10), 0);
        assert!(calls(&e, SWAP_WORD).is_empty());
        assert_eq!(calls(&e, ALLOCATE), vec![vec![8]]);
        assert_eq!(calls(&e, ADD_COMPILE_INDEX), vec![vec![sound, FILE]]);
        let added = calls(&e, LIST_ADD_00905820);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], this + 0x1f8);
        assert!(calls(&e, SWAP_SOUND_00462230).is_empty());
    }

    #[test]
    fn fn_00580990_with_a_failed_sound_allocation_reads_into_null() {
        let (mut e, this, _) = load_rig(vec![(CHUNK_SNAM, 8)], true);
        e.register(ALLOCATE, |_, _| 0u32.into_ret());
        assert!(e.call(0x0058_0990, &args![this, FILE]).bool());
        assert_eq!(calls(&e, GET_CHUNK_DATA_SIZED), vec![vec![FILE, 0, 8]]);
        assert_eq!(calls(&e, SWAP_SOUND_00462230), vec![vec![0]]);
        assert_eq!(calls(&e, ADD_COMPILE_INDEX), vec![vec![0, FILE]]);
        assert!(calls(&e, MEMSET).is_empty());
    }
}

#[cfg(test)]
mod n045_tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    fn only(log: &Log, addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn ok(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    /// List doubles (a node is item, next) and quiet doubles for the rest.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(LIST_ITEM_006815C0, |_, a| a[0].into_ret());
        e.register(NODE_FOLLOWING_00726070, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(LIST_COUNT, |e, a| {
            let mut node = a[0];
            let mut n = 0u32;
            while node != 0 {
                if e.mem.u32(node) != 0 {
                    n += 1;
                }
                node = e.mem.u32(node + 4);
            }
            n.into_ret()
        });
        for addr in [
            START_FORM,
            CLOSE_FORM,
            TEXTURE_SAVE,
            MODEL_SAVE,
            ADD_WORD_CHUNK,
            ADD_CHUNK_ARRAY,
            ADD_CHUNK_ARRAY_OV2,
            ADD_CHUNK_DATA,
            IMAGE_SPACE_PARAMETERS_ENDIAN,
            SWAP_SOUND_00462230,
            COPY_ALL_COMPONENTS,
            MEMCPY,
            LIST_REMOVE_ALL_00470470,
            LIST_ADD_00905820,
        ] {
            e.register(addr, ok);
        }
        e.register(FORM_ID_0084E3A0, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.register(DYNAMIC_CAST, |_, a| a[0].into_ret());
        e.register(MEMCMP, |e, a| {
            u32::from(e.mem.bytes(a[0], a[2]) != e.mem.bytes(a[1], a[2])).into_ret()
        });
        e
    }

    #[test]
    fn save_writes_the_weather_chunks_in_order() {
        let mut e = engine();
        let swap = std::rc::Rc::new(std::cell::Cell::new(false));
        let s = swap.clone();
        e.register_double(SWAPS_BYTE_ORDER, move |_, _| u32::from(s.get()).into_ret());
        let this = e.mem.alloc(0x374);
        e.mem.set_u32(this + 0x368, 2);
        let modifier = e.mem.alloc(0x10);
        e.mem.set_u32(modifier + 0xc, 0x77);
        e.mem.set_u32(this + 0x18 + 4 + 2 * 4, modifier);
        let sound = e.mem.alloc(8);
        e.mem.set_u32(this + 0x1f8, sound);
        e.call_log = Some(vec![]);
        e.call(0x0058_13d0, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(only(&log, START_FORM), vec![vec![this]]);
        assert_eq!(
            only(&log, TEXTURE_SAVE),
            vec![vec![this + 0x34, CHUNK_DNAM], vec![this + 0x40, CHUNK_CNAM]]
        );
        assert_eq!(
            only(&log, MODEL_SAVE),
            vec![vec![this + 0xc8, CHUNK_MODL, 0x5444_4f4d]]
        );
        assert_eq!(
            only(&log, ADD_WORD_CHUNK),
            vec![vec![CHUNK_IAD_0 + 2, 0x77], vec![CHUNK_LNAM, 2]]
        );
        assert_eq!(
            only(&log, ADD_CHUNK_ARRAY),
            vec![
                vec![CHUNK_ONAM, this + 0x64, 4],
                vec![CHUNK_DATA, this + 0xe0, 0xf]
            ]
        );
        assert_eq!(
            only(&log, ADD_CHUNK_ARRAY_OV2),
            vec![
                vec![CHUNK_PNAM, this + 0x68, 0x18],
                vec![CHUNK_NAM0, this + 0x108, 0x3c],
                vec![CHUNK_FNAM, this + 0xf0, 6]
            ]
        );
        assert_eq!(
            only(&log, ADD_CHUNK_DATA),
            vec![
                vec![CHUNK_INAM, this + 0x238, 0x130],
                vec![CHUNK_SNAM, sound, 8]
            ]
        );
        assert!(only(&log, IMAGE_SPACE_PARAMETERS_ENDIAN).is_empty());
        assert_eq!(only(&log, CLOSE_FORM), vec![vec![this]]);
        swap.set(true);
        e.call_log = Some(vec![]);
        e.call(0x0058_13d0, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            only(&log, IMAGE_SPACE_PARAMETERS_ENDIAN),
            vec![
                vec![this + 0x238],
                vec![this + 0x2d0],
                vec![this + 0x238],
                vec![this + 0x2d0]
            ]
        );
        assert_eq!(
            only(&log, SWAP_SOUND_00462230),
            vec![vec![sound], vec![sound]]
        );
    }

    #[test]
    fn copy_takes_the_layers_blocks_and_rebuilds_the_sounds() {
        let mut e = engine();
        let this = e.mem.alloc(0x374);
        let source = e.mem.alloc(0x374);
        e.mem.set_u32(source + 0x368, 1);
        let vt = e.mem.alloc(0x10);
        e.mem.set_u32(vt + 8, 0x0050_0000);
        e.register(0x0050_0000, ok);
        e.mem.set_u32(this + 0x34, vt);
        e.mem.set_u32(this + 0xc8, vt);
        let record = e.mem.alloc(8);
        e.mem.set_u32(record, 0x11);
        e.mem.set_u32(record + 4, 0x22);
        e.mem.set_u32(source + 0x1f8, record);
        e.register(ALLOCATE, |_, _| 0x9000u32.into_ret());
        e.register(SOUND_RECORD_CTOR, |_, a| a[0].into_ret());
        let added = std::rc::Rc::new(std::cell::Cell::new(0));
        let a2 = added.clone();
        e.register_double(LIST_ADD_00905820, move |e, a| {
            a2.set(e.mem.u32(a[1]));
            Ret::default()
        });
        e.call(0x0058_1630, &[this, 0]);
        assert_eq!(e.mem.u32(this + 0x368), 0);
        e.call_log = Some(vec![]);
        e.call(0x0058_1630, &[this, source]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this + 0x368), 1);
        assert_eq!(
            only(&log, 0x0050_0000),
            vec![
                vec![this + 0x34, source + 0x34],
                vec![this + 0xc8, source + 0xc8]
            ]
        );
        assert_eq!(only(&log, MEMCPY).len(), 6);
        assert_eq!(
            only(&log, MEMCPY)[5],
            vec![this + 0x238, source + 0x238, 0x130]
        );
        assert_eq!(
            only(&log, SOUND_RECORD_CTOR),
            vec![vec![0x9000, 0x11, 0x22]]
        );
        assert_eq!(added.get(), 0x9000);
        e.register(ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0058_1630, &[this, source]);
        let log = e.call_log.take().unwrap();
        assert!(only(&log, SOUND_RECORD_CTOR).is_empty());
        assert_eq!(added.get(), 0);
    }

    #[test]
    fn compare_checks_sounds_layers_and_blocks() {
        let mut e = engine();
        let this = e.mem.alloc(0x374);
        let source = e.mem.alloc(0x374);
        let vt = e.mem.alloc(0x10);
        e.mem.set_u32(vt + 0xc, 0x0050_0000);
        e.register(0x0050_0000, |_, _| Ret::default());
        e.mem.set_u32(this + 0xc8, vt);
        e.register(COMPARE_ALL_COMPONENTS, |_, _| Ret::default());
        let s1 = e.mem.alloc(8);
        let s2 = e.mem.alloc(8);
        e.mem.set_u32(this + 0x1f8, s1);
        e.mem.set_u32(source + 0x1f8, s2);
        assert!(e.call(0x0058_1880, &[this, 0]).bool());
        assert!(!e.call(0x0058_1880, &[this, source]).bool());
        e.mem.set_u8(s2 + 7, 1);
        assert!(e.call(0x0058_1880, &[this, source]).bool());
        e.mem.set_u8(s2 + 7, 0);
        e.mem.set_u32(source + 0x1f8, 0);
        assert!(e.call(0x0058_1880, &[this, source]).bool());
        e.mem.set_u32(source + 0x1f8, s2);
        e.mem.set_u8(source + 0x238 + 0x12f, 3);
        assert!(e.call(0x0058_1880, &[this, source]).bool());
        e.mem.set_u8(source + 0x238 + 0x12f, 0);
        e.mem.set_u32(source + 0x368, 1);
        assert!(e.call(0x0058_1880, &[this, source]).bool());
        e.register(COMPARE_ALL_COMPONENTS, |_, _| 1u32.into_ret());
        e.mem.set_u32(source + 0x368, 0);
        assert!(e.call(0x0058_1880, &[this, source]).bool());
    }

    #[test]
    fn modifiable_count_form_copy_and_compare() {
        let mut e = engine();
        e.register(GET_MODIFIER, |e, a| {
            e.mem.u32(a[0] + 4 + a[1] * 4).into_ret()
        });
        let this = e.mem.alloc(0x1c);
        let other = e.mem.alloc(0x1c);
        for i in 0..6 {
            e.mem.set_u32(other + 4 + i * 4, 0x100 + i);
        }
        assert!(e.call(0x0058_2220, &[this, other]).bool());
        assert!(e.call(0x0058_2220, &[this, 0]).bool());
        e.call(0x0058_21b0, &[this, 0]);
        assert_eq!(e.mem.u32(this + 4), 0);
        e.call(0x0058_21b0, &[this, other]);
        assert_eq!(e.mem.u32(this + 0x18), 0x105);
        assert!(!e.call(0x0058_2220, &[this, other]).bool());
    }
}
