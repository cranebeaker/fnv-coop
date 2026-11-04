//! `fallout shared/region system/tesregiondatalandscape.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESRegionDataLandscape` is the landscape part of a region record: the
//! canopy shadow texture (a `TESTexture` the object owns). The unit also
//! builds the shared default full-mask texturing property (a 64x64 image of
//! random bytes) and holds a few small helpers the compiler emitted here
//! (`NiTexturingProperty::SetBaseClampMode` and its map helper, a revision
//! counter bump and a three-word format initializer).
//!
//! Functions with a C++ exception frame (`FS:[0]`) are translated without the
//! unwinding and marked incomplete.

#[allow(unused_imports)]
use crate::prelude::*;

/// `TESRegionDataLandscape`'s vtable (`.?AVTESRegionDataLandscape@@`,
/// checked through its RTTI locator at `01023c88`).
const VTABLE: u32 = 0x0102_3c8c;
/// `"Trees\\CanopyShadow.dds"`.
const CANOPY_SHADOW_PATH: u32 = 0x0102_3c70;
/// `""` (an empty string in `.rdata`).
const EMPTY_STRING: u32 = 0x0101_1584;
/// `"DefaultFullMaskTextureProp"`.
const FULL_MASK_NAME: u32 = 0x0102_3cbc;
/// The full-mask property's `NiPointer<NiTexturingProperty>` holder
/// (`TESRegionDataLandscape::spDefaultFullMaskTextureProp`, Xbox PDB
/// static; the address is the one the PC code uses).
const DEFAULT_FULL_MASK_PROP: u32 = 0x011c_7f0c;
/// The pixel format passed to the `NiPixelData` constructor.
const FULL_MASK_PIXEL_FORMAT: u32 = 0x011a_a2a0;
/// `255.0f`: upper bound of the random bytes.
const RANDOM_BYTE_MAX: u32 = 0x0102_3cd8;
/// `0.005` (`double`): the blend's scale (one half of one percent squared).
const BLEND_SCALE: u32 = 0x0102_3b88;

/// `'ICON'` as the chunk id word the game compares against.
const CHUNK_ICON: u32 = 0x4e4f_4349;

/// `MemoryManager::Allocate` wrapper (`operator new`, cdecl).
const ALLOCATE: u32 = 0x0040_1000;
/// `operator delete` (cdecl).
const DEALLOCATE: u32 = 0x0040_1030;
/// The `NiNew` allocator (cdecl, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// `TESTexture` constructor (thiscall, no argument; the map has no name).
const TEXTURE_CONSTRUCT: u32 = 0x0048_e270;
/// `TESTexture` destructor body (thiscall, no argument).
const TEXTURE_DESTRUCT: u32 = 0x0048_e2e0;
/// `TESSoundFile::SetSoundFile` (map name; it sets the texture's file name
/// through the shared code), thiscall with the name.
const SET_FILE_NAME: u32 = 0x0048_9100;
/// `MapMarkerData::GetLocationName` (map name; the shared getter of a
/// `BSString`'s text), thiscall.
const GET_FILE_NAME: u32 = 0x0040_8da0;
/// `TESTexture::Save` (Xbox PDB), thiscall with the chunk id.
const TEXTURE_SAVE: u32 = 0x0048_e370;
/// `TESTexture::LoadTextureChunk` (Xbox PDB), cdecl `(texture, file)`.
const TEXTURE_LOAD_CHUNK: u32 = 0x0048_e3d0;
/// `TESFile::GetTESChunk` (Xbox PDB), thiscall on the file.
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// Nonzero when the texture has a file name (thiscall; the map has no name).
const TEXTURE_HAS_NAME: u32 = 0x0048_cee0;
/// Returns the first word of the object (thiscall): a string's text, a
/// `NiPointer`'s object.
const FIRST_WORD: u32 = 0x0055_9450;
/// Case-insensitive string comparison (cdecl `(a, b)`, zero when equal).
const COMPARE_NO_CASE: u32 = 0x0040_4dc0;
/// `BSString` assignment from text (thiscall `(text, length)`).
const STRING_SET: u32 = 0x0040_37f0;
/// `BSString` constructor and destructor (thiscall).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_DESTRUCT: u32 = 0x0040_37d0;

/// `TESRegionData` constructor (Xbox PDB `TESRegionData::TESRegionData`).
const REGION_DATA_CONSTRUCT: u32 = 0x004f_1400;
/// `TESRegionData` copy constructor (thiscall with the source).
const REGION_DATA_COPY_CONSTRUCT: u32 = 0x004f_1460;
/// `TESRegionData` destructor body (thiscall).
const REGION_DATA_DESTRUCT: u32 = 0x004f_14a0;
/// `TESRegionData::Save` (Xbox PDB).
const REGION_DATA_SAVE: u32 = 0x004f_14c0;
/// Region data getters and setters the blend uses (thiscall, byte results
/// and byte arguments). `004f1540` reads a byte of the source, `005bb4d0`
/// the byte the blend weighs (both shared, folded code), `004f15a0` and
/// `004f15c0` store the two bytes.
const REGION_DATA_GET_FLAG: u32 = 0x004f_1540;
const REGION_DATA_GET_WEIGHT: u32 = 0x005b_b4d0;
const REGION_DATA_SET_FLAG: u32 = 0x004f_15a0;
const REGION_DATA_SET_WEIGHT: u32 = 0x004f_15c0;
/// The float helper the blend passes its weighted value through (cdecl,
/// one `float`, result in ST0).
const BLEND_ROUND: u32 = 0x0040_6cc0;
/// `RandomFloat` (map name), cdecl `(min, max)`, result in ST0.
const RANDOM_FLOAT: u32 = 0x0047_6b70;

/// `GetID` (Xbox PDB vtable slot of `TESRegionData`).
const SLOT_GET_ID: u32 = 0x10;
/// `GetCanopyShadowTexture` / `SetCanopyShadowTexture` (Xbox PDB vtable
/// slots; the PC vtable holds `004f2a70` and `004f2ab0` there).
const SLOT_GET_CANOPY_SHADOW_TEXTURE: u32 = 0x28;
const SLOT_SET_CANOPY_SHADOW_TEXTURE: u32 = 0x2c;
/// The id `GetID` returns for landscape data.
const LANDSCAPE_ID: u32 = 5;

layout! {
    /// `TESRegionDataLandscape` (Xbox PDB), 0x0C bytes on both builds.
    pub struct TESRegionDataLandscape: 0x0C {
        /// `TESRegionData::bOverride` (Xbox PDB, inherited).
        0x04 bOverride: u8,
        /// `TESRegionData::bIgnore` (Xbox PDB, inherited).
        0x05 bIgnore: u8,
        /// `pCanopyShadowTexture` (Xbox PDB): `TESTexture*`.
        0x08 pCanopyShadowTexture: Ptr,
    }

    /// The 12-byte three-word value `004f31d0` initializes (6, 3, 2); the
    /// full-mask code passes it to the `NiTexturingProperty` constructor
    /// next to the pixel data. Its class is not confirmed.
    pub struct ThreeWordPrefs: 0x0C {
        0x00 first: u32,
        0x04 second: u32,
        0x08 third: u32,
    }
}

/// Stack frame of `GetDefaultFullMaskTextureProp`: the prefs value and the
/// `NiFixedString` name temporary.
mod full_mask_frame {
    pub const PREFS: u32 = 0x00;
    pub const NAME: u32 = 0x0C;
    pub const SIZE: u32 = 0x10;
}

/// `FISTP dword` with the truncating control word: the integer, or the
/// "integer indefinite" `0x80000000` for NaN and out-of-range values.
fn fistp_truncate_i32(value: f64) -> i32 {
    if value.is_nan() || value >= 2_147_483_648.0 || value <= -2_147_483_649.0 {
        i32::MIN
    } else {
        value.trunc() as i32
    }
}

/// A new `TESTexture` (`operator new(0xc)` and its constructor), or null.
fn new_texture(e: &mut Engine) -> Ptr {
    let block = e.call(ALLOCATE, &args![0x0cu32]).u32();
    if block == 0 {
        Ptr::NULL
    } else {
        e.call(TEXTURE_CONSTRUCT, &args![block]).ptr()
    }
}

// Translated from 004f2790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataLandscape::TESRegionDataLandscape` (Xbox PDB): constructs
/// the base, sets the vtable, makes the canopy shadow texture and names it
/// `"Trees\\CanopyShadow.dds"`.
// Translation incomplete: 004f2790 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn tes_region_data_landscape_constructor(
    e: &mut Engine,
    this: Ptr<TESRegionDataLandscape>,
) -> Ptr<TESRegionDataLandscape> {
    e.call(REGION_DATA_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), VTABLE);
    let texture = new_texture(e);
    e.set(this, TESRegionDataLandscape::pCanopyShadowTexture, texture);
    e.call(SET_FILE_NAME, &args![texture, CANOPY_SHADOW_PATH]);
    this
}

// Translated from 004f2840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataLandscape::`scalar deleting destructor'` (Xbox PDB): runs
/// the destructor and frees the object when bit 0 of `flags` is set.
pub fn tes_region_data_landscape_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<TESRegionDataLandscape>,
    flags: u32,
) -> Ptr<TESRegionDataLandscape> {
    fn_004f2930(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 004f2870 (decompiled, FalloutNV.exe 1.4.0.525)
/// The copy constructor: copies the base from `source`, sets the vtable,
/// makes a new canopy shadow texture and gives it `source`'s texture name.
// Translation incomplete: 004f2870 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_004f2870(
    e: &mut Engine,
    this: Ptr<TESRegionDataLandscape>,
    source: Ptr<TESRegionDataLandscape>,
) -> Ptr<TESRegionDataLandscape> {
    e.call(REGION_DATA_COPY_CONSTRUCT, &args![this, source]);
    e.mem.set_u32(this.addr(), VTABLE);
    let texture = new_texture(e);
    e.set(this, TESRegionDataLandscape::pCanopyShadowTexture, texture);
    let source_texture = e.get(source, TESRegionDataLandscape::pCanopyShadowTexture);
    let name = e.call(GET_FILE_NAME, &args![source_texture]).u32();
    let texture = e.get(this, TESRegionDataLandscape::pCanopyShadowTexture);
    e.call(SET_FILE_NAME, &args![texture, name]);
    this
}

// Translated from 004f2930 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor: resets the vtable, deletes the canopy shadow texture
/// (through `004f29c0` with flag 1) and destroys the base.
// Translation incomplete: 004f2930 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_004f2930(e: &mut Engine, this: Ptr<TESRegionDataLandscape>) {
    e.mem.set_u32(this.addr(), VTABLE);
    let texture = e.get(this, TESRegionDataLandscape::pCanopyShadowTexture);
    if !texture.is_null() {
        fn_004f29c0(e, texture, 1);
    }
    e.call(REGION_DATA_DESTRUCT, &args![this]);
}

// Translated from 004f29c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESTexture` scalar deleting destructor emitted in this unit: runs
/// the texture destructor (`0048e2e0`) and frees it when bit 0 of `flags` is
/// set.
pub fn fn_004f29c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(TEXTURE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 004f29f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataLandscape::Save` (Xbox PDB): saves the base, then the
/// canopy shadow texture as an `ICON` chunk.
pub fn tes_region_data_landscape_save(e: &mut Engine, this: Ptr<TESRegionDataLandscape>) {
    e.call(REGION_DATA_SAVE, &args![this]);
    let texture = e.get(this, TESRegionDataLandscape::pCanopyShadowTexture);
    e.call(TEXTURE_SAVE, &args![texture, CHUNK_ICON]);
}

// Translated from 004f2a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataLandscape::LoadRegionDataLandscape` (Xbox PDB): false
/// without a file; an `ICON` chunk is loaded into the canopy shadow
/// texture; true for every chunk.
pub fn tes_region_data_landscape_load_region_data_landscape(
    e: &mut Engine,
    this: Ptr<TESRegionDataLandscape>,
    file: Ptr,
) -> bool {
    if file.is_null() {
        return false;
    }
    let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
    if chunk == CHUNK_ICON {
        let texture = e.get(this, TESRegionDataLandscape::pCanopyShadowTexture);
        e.call(TEXTURE_LOAD_CHUNK, &args![texture, file]);
    }
    true
}

// Translated from 004f2a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataLandscape::GetCanopyShadowTexture` (Xbox PDB): sets the
/// string at `result` to the texture's name (the empty string without a
/// texture). Returns what the string assignment (`004037f0`) leaves in EAX.
pub fn tes_region_data_landscape_get_canopy_shadow_texture(
    e: &mut Engine,
    this: Ptr<TESRegionDataLandscape>,
    result: Ptr,
) -> u32 {
    let texture = e.get(this, TESRegionDataLandscape::pCanopyShadowTexture);
    if texture.is_null() {
        e.call(STRING_SET, &args![result, EMPTY_STRING, 0u32]).u32()
    } else {
        let name = e.call(GET_FILE_NAME, &args![texture]).u32();
        e.call(STRING_SET, &args![result, name, 0u32]).u32()
    }
}

// Translated from 004f2ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataLandscape::SetCanopyShadowTexture` (Xbox PDB): false
/// without a texture, or when the texture already has a name equal (case
/// ignored) to `name`'s text; otherwise sets the name and returns true.
pub fn tes_region_data_landscape_set_canopy_shadow_texture(
    e: &mut Engine,
    this: Ptr<TESRegionDataLandscape>,
    name: Ptr,
) -> bool {
    let texture = e.get(this, TESRegionDataLandscape::pCanopyShadowTexture);
    if texture.is_null() {
        return false;
    }
    if e.call(TEXTURE_HAS_NAME, &args![texture]).u32() != 0 {
        let new_text = e.call(FIRST_WORD, &args![name]).u32();
        let texture = e.get(this, TESRegionDataLandscape::pCanopyShadowTexture);
        let old_text = e.call(GET_FILE_NAME, &args![texture]).u32();
        if e.call(COMPARE_NO_CASE, &args![old_text, new_text]).u32() == 0 {
            return false;
        }
    }
    let new_text = e.call(FIRST_WORD, &args![name]).u32();
    let texture = e.get(this, TESRegionDataLandscape::pCanopyShadowTexture);
    e.call(SET_FILE_NAME, &args![texture, new_text]);
    true
}

// Translated from 004f2b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataLandscape::Copy` (Xbox PDB): a new copy of this object
/// (`operator new(0xc)` and the copy constructor), or null.
// Translation incomplete: 004f2b10 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn tes_region_data_landscape_copy(
    e: &mut Engine,
    this: Ptr<TESRegionDataLandscape>,
) -> Ptr<TESRegionDataLandscape> {
    let block: Ptr<TESRegionDataLandscape> = e.call(ALLOCATE, &args![0x0cu32]).ptr();
    if block.is_null() {
        Ptr::NULL
    } else {
        fn_004f2870(e, block, this)
    }
}

/// The four calls the blend makes when `source` wins: the flag byte and the
/// weight byte are copied, then the canopy shadow texture name through the
/// string temporary.
fn blend_take_source(e: &mut Engine, this: Ptr<TESRegionDataLandscape>, source: Ptr, text: Ptr) {
    let flag = e.call(REGION_DATA_GET_FLAG, &args![source]).u8();
    e.call(REGION_DATA_SET_FLAG, &args![this, flag as u32]);
    let weight = e.call(REGION_DATA_GET_WEIGHT, &args![source]).u8();
    e.call(REGION_DATA_SET_WEIGHT, &args![this, weight as u32]);
    blend_take_texture(e, this, source, text);
}

fn blend_take_texture(e: &mut Engine, this: Ptr<TESRegionDataLandscape>, source: Ptr, text: Ptr) {
    e.vcall(source.addr(), SLOT_GET_CANOPY_SHADOW_TEXTURE, &args![text]);
    e.vcall(this.addr(), SLOT_SET_CANOPY_SHADOW_TEXTURE, &args![text]);
}

/// The weight byte of region data.
fn weight(e: &mut Engine, data: Ptr) -> u32 {
    e.call(REGION_DATA_GET_WEIGHT, &args![data]).u8() as u32
}

/// The weighted blend of the two weight bytes both blends compute
/// (`this`'s weight w, `source`'s s):
/// `(w*w + (100-w)*s + ((100-s)*w + s*s)) * 0.005`, passed through
/// `00406cc0` and truncated to a byte. The getter is called in the game's
/// order.
pub(crate) fn blended_weight(e: &mut Engine, this: Ptr, source: Ptr) -> u8 {
    let w1 = weight(e, this);
    let w2 = weight(e, this);
    let w3 = weight(e, this);
    let s1 = weight(e, source);
    let first = (w1 as i32)
        .wrapping_mul(w2 as i32)
        .wrapping_add(100i32.wrapping_sub(w3 as i32).wrapping_mul(s1 as i32));
    let first = first as f32;
    let s2 = weight(e, source);
    let w4 = weight(e, this);
    let s3 = weight(e, source);
    let s4 = weight(e, source);
    let second = 100i32
        .wrapping_sub(s2 as i32)
        .wrapping_mul(w4 as i32)
        .wrapping_add((s3 as i32).wrapping_mul(s4 as i32));
    let sum = (second as f64 + first as f64) as f32;
    let scale: f64 = e.global(BLEND_SCALE);
    let scaled = (sum as f64 * scale) as f32;
    let rounded = e
        .call(BLEND_ROUND, &args![x87_quiet(scaled).to_bits()])
        .f64();
    fistp_truncate_i32(rounded) as u8
}

// Translated from 004f2b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataLandscape::BlendInto` (Xbox PDB): merges landscape data
/// `source` (its `GetID` must be 5, and `enabled` nonzero) into this one.
/// When this data is ignored, `source`'s flag byte, weight byte and canopy
/// texture are taken; when `source` is ignored nothing happens; with this
/// data's override set, `source` wins only with its flag byte set and a
/// higher weight; otherwise `source` wins with its flag byte set, else its
/// texture is taken when its weight is higher and the weight becomes the
/// blended weight. The texture travels through a `BSString` temporary.
// Translation incomplete: 004f2b90 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn tes_region_data_landscape_blend_into(
    e: &mut Engine,
    this: Ptr<TESRegionDataLandscape>,
    source: Ptr,
    enabled: u32,
) {
    e.with_stack(8, |e, text| {
        e.call(STRING_CONSTRUCT, &args![text]);
        if source.is_null()
            || e.vcall(source.addr(), SLOT_GET_ID, &[]).u32() != LANDSCAPE_ID
            || enabled == 0
        {
            e.call(STRING_DESTRUCT, &args![text]);
            return;
        }
        if e.get(this, TESRegionDataLandscape::bIgnore) != 0 {
            blend_take_source(e, this, source, text);
        } else if e.mem.u8(source.addr() + 5) != 0 {
            // `TESRegionData::bIgnore` (Xbox PDB) +0x05 of the source.
            e.call(STRING_DESTRUCT, &args![text]);
            return;
        } else if e.get(this, TESRegionDataLandscape::bOverride) != 0 {
            if e.call(REGION_DATA_GET_FLAG, &args![source]).u8() != 0
                && weight(e, source) > weight(e, this.cast())
            {
                blend_take_source(e, this, source, text);
            }
        } else if e.call(REGION_DATA_GET_FLAG, &args![source]).u8() != 0 {
            blend_take_source(e, this, source, text);
        } else {
            if weight(e, source) > weight(e, this.cast()) {
                blend_take_texture(e, this, source, text);
            }
            let blended = blended_weight(e, this.cast(), source);
            e.call(REGION_DATA_SET_WEIGHT, &args![this, blended as u32]);
        }
        e.call(STRING_DESTRUCT, &args![text]);
    });
}

// Translated from 004f2eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataLandscape::Validate` (Xbox PDB): true when there is a
/// canopy shadow texture and it has a name.
pub fn tes_region_data_landscape_validate(
    e: &mut Engine,
    this: Ptr<TESRegionDataLandscape>,
) -> bool {
    let texture = e.get(this, TESRegionDataLandscape::pCanopyShadowTexture);
    !texture.is_null() && e.call(TEXTURE_HAS_NAME, &args![texture]).u32() != 0
}

/// One random byte: `RandomFloat(0.0, 255.0)` truncated.
fn random_byte(e: &mut Engine) -> u8 {
    let max = x87_quiet(e.global::<f32>(RANDOM_BYTE_MAX));
    let value = e
        .call(RANDOM_FLOAT, &args![0.0f32.to_bits(), max.to_bits()])
        .f64();
    fistp_truncate_i32(value) as u8
}

// Translated from 004f2ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataLandscape::GetDefaultFullMaskTextureProp` (Xbox PDB): the
/// shared full-mask texturing property, made on first use: 64x64 pixel
/// data (`NiPixelData`, format `011aa2a0`, one mipmap level, one face)
/// filled with random bytes (four per pixel, row by row), its revision
/// bumped, then a `NiTexturingProperty` named
/// `"DefaultFullMaskTextureProp"` on it, stored in the holder at
/// `011c7f0c`, and its base map's clamp mode set to 0. Returns the holder's
/// property.
// Translation incomplete: 004f2ee0 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn tes_region_data_landscape_get_default_full_mask_texture_prop(e: &mut Engine) -> Ptr {
    if e.call(FIRST_WORD, &args![DEFAULT_FULL_MASK_PROP]).u32() != 0 {
        return e.call(FIRST_WORD, &args![DEFAULT_FULL_MASK_PROP]).ptr();
    }
    let block = e.call(NI_ALLOCATE, &args![0x74u32]).u32();
    let pixels: Ptr = if block == 0 {
        Ptr::NULL
    } else {
        e.call(
            0x00a7_c190,
            &args![block, 0x40u32, 0x40u32, FULL_MASK_PIXEL_FORMAT, 1u32, 1u32],
        )
        .ptr()
    };
    let bytes = e.call(0x0064_dac0, &args![pixels, 0u32, 0u32]).u32();
    for row in 0..0x40u32 {
        for column in 0..0x40u32 {
            let at = bytes.wrapping_add((row * 0x40 + column) * 4);
            for channel in 0..4 {
                let value = random_byte(e);
                e.mem.set_u8(at.wrapping_add(channel), value);
            }
        }
    }
    fn_004f31b0(e, pixels);
    e.with_stack(full_mask_frame::SIZE, |e, frame| {
        let prefs: Ptr<ThreeWordPrefs> = Ptr::new(frame.addr() + full_mask_frame::PREFS);
        let name: Ptr = Ptr::new(frame.addr() + full_mask_frame::NAME);
        fn_004f31d0(e, prefs);
        let property_block = e.call(NI_ALLOCATE, &args![0x30u32]).u32();
        let property = if property_block == 0 {
            0
        } else {
            let name_ref = e.call(0x0043_8170, &args![name, FULL_MASK_NAME]).u32();
            e.call(0x00a6_abb0, &args![property_block, pixels, name_ref, prefs])
                .u32()
        };
        e.call(0x0066_b0d0, &args![DEFAULT_FULL_MASK_PROP, property]);
        if property_block != 0 {
            e.call(0x0043_81b0, &args![name]);
        }
    });
    let property = e.call(FIRST_WORD, &args![DEFAULT_FULL_MASK_PROP]).u32();
    e.call(0x004f_3200, &args![property, 0u32]);
    e.call(FIRST_WORD, &args![DEFAULT_FULL_MASK_PROP]).ptr()
}

// Translated from 004f31b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds one to the word at +0x68 (the full-mask code calls it on the new
/// `NiPixelData` after filling it; the field's name is not confirmed).
pub fn fn_004f31b0(e: &mut Engine, this: Ptr) {
    let value = e.mem.u32(this.addr() + 0x68);
    e.mem.set_u32(this.addr() + 0x68, value.wrapping_add(1));
}

// Translated from 004f31d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the three words at `this` to 6, 3 and 2; returns `this`.
pub fn fn_004f31d0(e: &mut Engine, this: Ptr<ThreeWordPrefs>) -> Ptr<ThreeWordPrefs> {
    e.set(this, ThreeWordPrefs::first, 6);
    e.set(this, ThreeWordPrefs::second, 3);
    e.set(this, ThreeWordPrefs::third, 2);
    this
}

// Translated from 004f3200 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTexturingProperty::SetBaseClampMode` (map name, matching the body):
/// takes map 0 of the property's map array (`NiTArray` at +0x1c, read with
/// `00877a30`); when empty, makes a 0x10-byte map (`00a69dd0`) and stores
/// it at index 0 (`0096ae90`); then sets the map's clamp mode (`004f32b0`).
// Translation incomplete: 004f3200 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn ni_texturing_property_set_base_clamp_mode(e: &mut Engine, this: Ptr, mode: u32) {
    let maps = this.addr() + 0x1c;
    e.with_stack(4, |e, slot| {
        let entry = e.call(0x0087_7a30, &args![maps, 0u32]).u32();
        let mut map = e.mem.u32(entry);
        e.mem.set_u32(slot.addr(), map);
        if map == 0 {
            let block = e.call(ALLOCATE, &args![0x10u32]).u32();
            map = if block == 0 {
                0
            } else {
                e.call(0x00a6_9dd0, &args![block]).u32()
            };
            e.mem.set_u32(slot.addr(), map);
            e.call(0x0096_ae90, &args![maps, 0u32, slot]);
        }
        let map = e.mem.u32(slot.addr());
        fn_004f32b0(e, Ptr::new(map), mode as u16);
    });
}

// Translated from 004f32b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets a texturing map's clamp mode: `004f32e0(mode, 0x3000, 12)` (the
/// mode in the bits the mask `0x3000` selects, shifted by 12).
pub fn fn_004f32b0(e: &mut Engine, this: Ptr, mode: u16) {
    e.call(0x004f_32e0, &args![this, mode as u32, 0x3000u32, 0x0cu32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004f2790,
            tes_region_data_landscape_constructor(
                Ptr<TESRegionDataLandscape>,
            ) -> Ptr<TESRegionDataLandscape>
        ),
        entry!(
            0x004f2840,
            tes_region_data_landscape_scalar_deleting_destructor(
                Ptr<TESRegionDataLandscape>,
                u32,
            )
                -> Ptr<TESRegionDataLandscape>
        ),
        entry!(
            0x004f2870,
            fn_004f2870(
                Ptr<TESRegionDataLandscape>,
                Ptr<TESRegionDataLandscape>,
            ) -> Ptr<TESRegionDataLandscape>
        ),
        entry!(0x004f2930, fn_004f2930(Ptr<TESRegionDataLandscape>)),
        entry!(0x004f29c0, fn_004f29c0(Ptr, u32) -> Ptr),
        entry!(
            0x004f29f0,
            tes_region_data_landscape_save(Ptr<TESRegionDataLandscape>)
        ),
        entry!(
            0x004f2a20,
            tes_region_data_landscape_load_region_data_landscape(
                Ptr<TESRegionDataLandscape>,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x004f2a70,
            tes_region_data_landscape_get_canopy_shadow_texture(
                Ptr<TESRegionDataLandscape>,
                Ptr,
            ) -> u32
        ),
        entry!(
            0x004f2ab0,
            tes_region_data_landscape_set_canopy_shadow_texture(
                Ptr<TESRegionDataLandscape>,
                Ptr,
            ) -> bool
        ),
        entry!(
            0x004f2b10,
            tes_region_data_landscape_copy(
                Ptr<TESRegionDataLandscape>,
            ) -> Ptr<TESRegionDataLandscape>
        ),
        entry!(
            0x004f2b90,
            tes_region_data_landscape_blend_into(Ptr<TESRegionDataLandscape>, Ptr, u32)
        ),
        entry!(
            0x004f2eb0,
            tes_region_data_landscape_validate(Ptr<TESRegionDataLandscape>) -> bool
        ),
        entry!(
            0x004f2ee0,
            tes_region_data_landscape_get_default_full_mask_texture_prop() -> Ptr
        ),
        entry!(0x004f31b0, fn_004f31b0(Ptr)),
        entry!(
            0x004f31d0,
            fn_004f31d0(Ptr<ThreeWordPrefs>) -> Ptr<ThreeWordPrefs>
        ),
        entry!(
            0x004f3200,
            ni_texturing_property_set_base_clamp_mode(Ptr, u32)
        ),
        entry!(0x004f32b0, fn_004f32b0(Ptr, u16)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// A double at `addr` returning `eax`.
    fn fixed(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    /// Doubles at `addrs` returning 0.
    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            fixed(e, addr, 0);
        }
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn landscape(e: &mut Engine, texture: u32) -> Ptr<TESRegionDataLandscape> {
        let this: Ptr<TESRegionDataLandscape> = e.new_object();
        e.set(
            this,
            TESRegionDataLandscape::pCanopyShadowTexture,
            Ptr::new(texture),
        );
        this
    }

    #[test]
    fn constructor_makes_and_names_the_canopy_texture() {
        let mut e = Engine::new();
        let this: Ptr<TESRegionDataLandscape> = e.new_object();
        let block = e.mem.alloc(0x0c);
        stub(&mut e, &[REGION_DATA_CONSTRUCT, SET_FILE_NAME]);
        fixed(&mut e, ALLOCATE, block);
        e.register(TEXTURE_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_2790, &args![this]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(
            e.get(this, TESRegionDataLandscape::pCanopyShadowTexture)
                .addr(),
            block
        );
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x0c]]);
        assert_eq!(
            calls_to(&e, SET_FILE_NAME),
            vec![vec![block, CANOPY_SHADOW_PATH]]
        );

        // A failed allocation leaves the texture null (and names it anyway).
        fixed(&mut e, ALLOCATE, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_2790, &args![this]);
        assert!(e
            .get(this, TESRegionDataLandscape::pCanopyShadowTexture)
            .is_null());
        assert!(calls_to(&e, TEXTURE_CONSTRUCT).is_empty());
        assert_eq!(
            calls_to(&e, SET_FILE_NAME),
            vec![vec![0, CANOPY_SHADOW_PATH]]
        );
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        let this = landscape(&mut e, 0);
        stub(&mut e, &[REGION_DATA_DESTRUCT, DEALLOCATE]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_2840, &args![this, 0u32]).u32(), this.addr());
        assert_eq!(calls_to(&e, REGION_DATA_DESTRUCT), vec![vec![this.addr()]]);
        assert!(calls_to(&e, DEALLOCATE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x004f_2840, &args![this, 3u32]);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![this.addr()]]);
    }

    #[test]
    fn copy_constructor_copies_the_source_texture_name() {
        let mut e = Engine::new();
        let source = landscape(&mut e, 0x5000);
        let this: Ptr<TESRegionDataLandscape> = e.new_object();
        let block = e.mem.alloc(0x0c);
        stub(&mut e, &[REGION_DATA_COPY_CONSTRUCT, SET_FILE_NAME]);
        fixed(&mut e, ALLOCATE, block);
        fixed(&mut e, TEXTURE_CONSTRUCT, block);
        e.register(GET_FILE_NAME, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_2870, &args![this, source]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(
            calls_to(&e, REGION_DATA_COPY_CONSTRUCT),
            vec![vec![this.addr(), source.addr()]]
        );
        assert_eq!(calls_to(&e, GET_FILE_NAME), vec![vec![0x5000]]);
        assert_eq!(calls_to(&e, SET_FILE_NAME), vec![vec![block, 0x5001]]);
    }

    #[test]
    fn destructor_deletes_the_texture_when_present() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[TEXTURE_DESTRUCT, DEALLOCATE, REGION_DATA_DESTRUCT],
        );
        let this = landscape(&mut e, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x004f_2930, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(calls_to(&e, TEXTURE_DESTRUCT), vec![vec![0x6000]]);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![0x6000]]);
        assert_eq!(calls_to(&e, REGION_DATA_DESTRUCT), vec![vec![this.addr()]]);

        let empty = landscape(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_2930, &args![empty]);
        assert!(calls_to(&e, TEXTURE_DESTRUCT).is_empty());
        assert_eq!(calls_to(&e, REGION_DATA_DESTRUCT), vec![vec![empty.addr()]]);
    }

    #[test]
    fn texture_deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        stub(&mut e, &[TEXTURE_DESTRUCT, DEALLOCATE]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_29c0, &args![0x7000u32, 0u32]).u32(), 0x7000);
        assert_eq!(calls_to(&e, TEXTURE_DESTRUCT), vec![vec![0x7000]]);
        assert!(calls_to(&e, DEALLOCATE).is_empty());
        e.call(0x004f_29c0, &args![0x7000u32, 1u32]);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![0x7000]]);
    }

    #[test]
    fn save_writes_the_base_then_the_icon_chunk() {
        let mut e = Engine::new();
        stub(&mut e, &[REGION_DATA_SAVE, TEXTURE_SAVE]);
        let this = landscape(&mut e, 0x6000);
        e.call_log = Some(vec![]);
        e.call(0x004f_29f0, &args![this]);
        assert_eq!(calls_to(&e, REGION_DATA_SAVE), vec![vec![this.addr()]]);
        assert_eq!(calls_to(&e, TEXTURE_SAVE), vec![vec![0x6000, CHUNK_ICON]]);
    }

    #[test]
    fn load_reads_the_icon_chunk_into_the_texture() {
        let mut e = Engine::new();
        stub(&mut e, &[TEXTURE_LOAD_CHUNK]);
        let this = landscape(&mut e, 0x6000);
        assert!(!e.call(0x004f_2a20, &args![this, 0u32]).bool());

        fixed(&mut e, GET_TES_CHUNK, CHUNK_ICON);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_2a20, &args![this, 0x8000u32]).bool());
        assert_eq!(calls_to(&e, TEXTURE_LOAD_CHUNK), vec![vec![0x6000, 0x8000]]);

        fixed(&mut e, GET_TES_CHUNK, 0x1234_5678);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_2a20, &args![this, 0x8000u32]).bool());
        assert!(calls_to(&e, TEXTURE_LOAD_CHUNK).is_empty());
    }

    #[test]
    fn get_canopy_shadow_texture_copies_the_name_or_the_empty_string() {
        let mut e = Engine::new();
        fixed(&mut e, STRING_SET, 0x99);
        e.register(GET_FILE_NAME, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        let this = landscape(&mut e, 0x6000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_2a70, &args![this, 0x9000u32]).u32(), 0x99);
        assert_eq!(calls_to(&e, STRING_SET), vec![vec![0x9000, 0x6001, 0]]);

        let empty = landscape(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_2a70, &args![empty, 0x9000u32]);
        assert_eq!(
            calls_to(&e, STRING_SET),
            vec![vec![0x9000, EMPTY_STRING, 0]]
        );
        assert!(calls_to(&e, GET_FILE_NAME).is_empty());
    }

    #[test]
    fn set_canopy_shadow_texture_skips_an_equal_name() {
        let mut e = Engine::new();
        stub(&mut e, &[SET_FILE_NAME]);
        e.register(FIRST_WORD, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        fixed(&mut e, GET_FILE_NAME, 0x4400);
        let name: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(name.addr(), 0x4500);

        let none = landscape(&mut e, 0);
        assert!(!e.call(0x004f_2ab0, &args![none, name]).bool());

        let this = landscape(&mut e, 0x6000);
        // A texture with an equal name: nothing changes.
        fixed(&mut e, TEXTURE_HAS_NAME, 1);
        fixed(&mut e, COMPARE_NO_CASE, 0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x004f_2ab0, &args![this, name]).bool());
        assert_eq!(calls_to(&e, COMPARE_NO_CASE), vec![vec![0x4400, 0x4500]]);
        assert!(calls_to(&e, SET_FILE_NAME).is_empty());

        // A different name is set.
        fixed(&mut e, COMPARE_NO_CASE, 1);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_2ab0, &args![this, name]).bool());
        assert_eq!(calls_to(&e, SET_FILE_NAME), vec![vec![0x6000, 0x4500]]);

        // A texture without a name gets one without comparing.
        fixed(&mut e, TEXTURE_HAS_NAME, 0);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_2ab0, &args![this, name]).bool());
        assert!(calls_to(&e, COMPARE_NO_CASE).is_empty());
        assert_eq!(calls_to(&e, SET_FILE_NAME), vec![vec![0x6000, 0x4500]]);
    }

    #[test]
    fn copy_makes_a_new_object_with_the_copy_constructor() {
        let mut e = Engine::new();
        let this = landscape(&mut e, 0x6000);
        fixed(&mut e, ALLOCATE, 0);
        assert_eq!(e.call(0x004f_2b10, &args![this]).u32(), 0);

        let block = e.mem.alloc(0x0c);
        let texture = e.mem.alloc(0x0c);
        fixed(&mut e, ALLOCATE, block);
        fixed(&mut e, TEXTURE_CONSTRUCT, texture);
        stub(
            &mut e,
            &[REGION_DATA_COPY_CONSTRUCT, SET_FILE_NAME, GET_FILE_NAME],
        );
        assert_eq!(e.call(0x004f_2b10, &args![this]).u32(), block);
        assert_eq!(e.mem.u32(block), VTABLE);
        assert_eq!(e.mem.u32(block + 8), texture);
    }

    /// Doubles for the blend: `GetID` through a vtable, the flag and weight
    /// getters reading +0x07 and +0x06 of the object, the setters, the
    /// string temporary, the rounding helper (identity) and the scale.
    fn blend_engine(id: u32) -> Engine {
        let mut e = Engine::new();
        fixed(&mut e, 0x0000_9100, id);
        stub(&mut e, &[0x0000_9128, 0x0000_912c]);
        e.put_vtable(
            0x0000_a000,
            &[0, 0, 0, 0, 0x9100, 0, 0, 0, 0, 0, 0x9128, 0x912c],
        );
        e.register(REGION_DATA_GET_FLAG, |e, a| Ret {
            eax: e.mem.u8(a[0] + 7) as u32,
            ..Ret::default()
        });
        e.register(REGION_DATA_GET_WEIGHT, |e, a| Ret {
            eax: e.mem.u8(a[0] + 6) as u32,
            ..Ret::default()
        });
        stub(
            &mut e,
            &[
                REGION_DATA_SET_FLAG,
                REGION_DATA_SET_WEIGHT,
                STRING_CONSTRUCT,
                STRING_DESTRUCT,
            ],
        );
        e.register(BLEND_ROUND, |_, a| Ret {
            st0: f32::from_bits(a[0]) as f64,
            ..Ret::default()
        });
        e.map(0x0102_3000, 0x1000);
        e.set_global(BLEND_SCALE, 0.005f64);
        e
    }

    fn region_data(e: &mut Engine, ignore: u8, overriding: u8, weight: u8, flag: u8) -> Ptr {
        let p = Ptr::new(e.mem.alloc(0x0c));
        e.mem.set_u32(p.addr(), 0xa000);
        e.mem.set_u8(p.addr() + 4, overriding);
        e.mem.set_u8(p.addr() + 5, ignore);
        e.mem.set_u8(p.addr() + 6, weight);
        e.mem.set_u8(p.addr() + 7, flag);
        p
    }

    #[test]
    fn blend_into_follows_ignore_override_flag_and_weight() {
        // Not landscape data: only the string temporary.
        let mut e = blend_engine(4);
        let this = region_data(&mut e, 0, 0, 40, 0);
        let source = region_data(&mut e, 0, 0, 80, 1);
        e.call_log = Some(vec![]);
        e.call(0x004f_2b90, &args![this, source, 1u32]);
        assert!(calls_to(&e, REGION_DATA_SET_WEIGHT).is_empty());
        assert_eq!(calls_to(&e, STRING_DESTRUCT).len(), 1);

        // Disabled.
        let mut e = blend_engine(5);
        let this = region_data(&mut e, 0, 0, 40, 0);
        let source = region_data(&mut e, 0, 0, 80, 1);
        e.call_log = Some(vec![]);
        e.call(0x004f_2b90, &args![this, source, 0u32]);
        assert!(calls_to(&e, 0x9128).is_empty());

        // This data ignored: the source's bytes and texture are taken.
        let this = region_data(&mut e, 1, 0, 40, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_2b90, &args![this, source, 1u32]);
        assert_eq!(
            calls_to(&e, REGION_DATA_SET_FLAG),
            vec![vec![this.addr(), 1]]
        );
        assert_eq!(
            calls_to(&e, REGION_DATA_SET_WEIGHT),
            vec![vec![this.addr(), 80]]
        );
        assert_eq!(calls_to(&e, 0x9128).len(), 1);
        assert_eq!(calls_to(&e, 0x912c).len(), 1);
        assert_eq!(calls_to(&e, STRING_DESTRUCT).len(), 1);

        // The source ignored: nothing.
        let this = region_data(&mut e, 0, 0, 40, 0);
        let ignored = region_data(&mut e, 1, 0, 80, 1);
        e.call_log = Some(vec![]);
        e.call(0x004f_2b90, &args![this, ignored, 1u32]);
        assert!(calls_to(&e, 0x9128).is_empty());
        assert_eq!(calls_to(&e, STRING_DESTRUCT).len(), 1);

        // Override: the source wins only with its flag and a higher weight.
        let this = region_data(&mut e, 0, 1, 90, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_2b90, &args![this, source, 1u32]);
        assert!(calls_to(&e, REGION_DATA_SET_FLAG).is_empty());
        let this = region_data(&mut e, 0, 1, 40, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_2b90, &args![this, source, 1u32]);
        assert_eq!(
            calls_to(&e, REGION_DATA_SET_WEIGHT),
            vec![vec![this.addr(), 80]]
        );

        // No override, flag set: the source wins.
        let this = region_data(&mut e, 0, 0, 90, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_2b90, &args![this, source, 1u32]);
        assert_eq!(
            calls_to(&e, REGION_DATA_SET_WEIGHT),
            vec![vec![this.addr(), 80]]
        );

        // No flag: texture by weight, then the blended weight
        // (40*40 + 60*80 + 20*40 + 80*80) * 0.005 = 68.
        let plain = region_data(&mut e, 0, 0, 80, 0);
        let this = region_data(&mut e, 0, 0, 40, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_2b90, &args![this, plain, 1u32]);
        assert_eq!(calls_to(&e, 0x9128).len(), 1);
        assert!(calls_to(&e, REGION_DATA_SET_FLAG).is_empty());
        assert_eq!(
            calls_to(&e, REGION_DATA_SET_WEIGHT),
            vec![vec![this.addr(), 68]]
        );
        // A lighter source keeps the texture; the blend is symmetric here.
        let this = region_data(&mut e, 0, 0, 80, 0);
        let light = region_data(&mut e, 0, 0, 40, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_2b90, &args![this, light, 1u32]);
        assert!(calls_to(&e, 0x9128).is_empty());
        assert_eq!(
            calls_to(&e, REGION_DATA_SET_WEIGHT),
            vec![vec![this.addr(), 68]]
        );
    }

    #[test]
    fn blended_weight_truncates_and_wraps_to_a_byte() {
        let mut e = blend_engine(5);
        // (90*90 + 10*95 + 5*90 + 95*95) * 0.005 = 92.625 -> 92.
        let this = region_data(&mut e, 0, 0, 90, 0);
        let source = region_data(&mut e, 0, 0, 95, 0);
        assert_eq!(blended_weight(&mut e, this, source), 92);
        // (250*250 - 150*250 - 150*250 + 250*250) * 0.005 = 250.
        let this = region_data(&mut e, 0, 0, 250, 0);
        let source = region_data(&mut e, 0, 0, 250, 0);
        assert_eq!(blended_weight(&mut e, this, source), 250);
        assert_eq!(fistp_truncate_i32(f64::NAN), i32::MIN);
        assert_eq!(fistp_truncate_i32(-1.5), -1);
    }

    #[test]
    fn validate_needs_a_named_texture() {
        let mut e = Engine::new();
        fixed(&mut e, TEXTURE_HAS_NAME, 1);
        let none = landscape(&mut e, 0);
        assert!(!e.call(0x004f_2eb0, &args![none]).bool());
        let this = landscape(&mut e, 0x6000);
        assert!(e.call(0x004f_2eb0, &args![this]).bool());
        fixed(&mut e, TEXTURE_HAS_NAME, 0);
        assert!(!e.call(0x004f_2eb0, &args![this]).bool());
    }

    #[test]
    fn default_full_mask_property_is_made_once() {
        let mut e = Engine::new();
        e.map(0x011c_7000, 0x1000);
        e.map(0x0102_3000, 0x1000);
        e.set_global(RANDOM_BYTE_MAX, 255.0f32);
        e.register(FIRST_WORD, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });

        // Already made: returned as is.
        e.set_global(DEFAULT_FULL_MASK_PROP, 0x4242u32);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_2ee0, &[]).u32(), 0x4242);
        assert_eq!(calls_to(&e, FIRST_WORD).len(), 2);
        assert!(calls_to(&e, NI_ALLOCATE).is_empty());

        e.set_global(DEFAULT_FULL_MASK_PROP, 0u32);
        let pixel_block = e.mem.alloc(0x74);
        let property_block = e.mem.alloc(0x30);
        let buffer = e.mem.alloc(0x40 * 0x40 * 4);
        let blocks = Rc::new(RefCell::new(vec![property_block, pixel_block]));
        let pending = blocks.clone();
        e.register_double(NI_ALLOCATE, move |_, _| Ret {
            eax: pending.borrow_mut().pop().unwrap(),
            ..Ret::default()
        });
        e.register(0x00a7_c190, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        fixed(&mut e, 0x0064_dac0, buffer);
        let mut n = 0u32;
        e.register_double(RANDOM_FLOAT, move |_, _| {
            n += 1;
            Ret {
                st0: (n % 200) as f64 + 0.75,
                ..Ret::default()
            }
        });
        e.register(0x0043_8170, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(0x0066_b0d0, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        stub(&mut e, &[0x0043_81b0, 0x004f_3200]);
        let prefs: Log = Rc::new(RefCell::new(vec![]));
        let seen = prefs.clone();
        e.register_double(0x00a6_abb0, move |e, a| {
            seen.borrow_mut().push((
                a[3],
                vec![e.mem.u32(a[3]), e.mem.u32(a[3] + 4), e.mem.u32(a[3] + 8)],
            ));
            Ret {
                eax: 0x5150,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_2ee0, &[]).u32(), 0x5150);
        assert_eq!(e.global::<u32>(DEFAULT_FULL_MASK_PROP), 0x5150);
        assert_eq!(
            calls_to(&e, 0x00a7_c190),
            vec![vec![pixel_block, 0x40, 0x40, FULL_MASK_PIXEL_FORMAT, 1, 1]]
        );
        assert_eq!(calls_to(&e, 0x0064_dac0), vec![vec![pixel_block, 0, 0]]);
        assert_eq!(e.mem.u8(buffer), 1);
        assert_eq!(e.mem.u8(buffer + 3), 4);
        assert_eq!(e.mem.u8(buffer + 4 * 0x40 * 0x40 - 1), (16384 % 200) as u8);
        assert_eq!(e.mem.u32(pixel_block + 0x68), 1);
        let random = calls_to(&e, RANDOM_FLOAT);
        assert_eq!(random.len(), 16384);
        assert_eq!(random[0], vec![0, 255.0f32.to_bits()]);
        let (prefs_at, words) = prefs.borrow()[0].clone();
        assert_eq!(words, vec![6, 3, 2]);
        let names = calls_to(&e, 0x0043_8170);
        assert_eq!(names[0][1], FULL_MASK_NAME);
        assert_eq!(
            calls_to(&e, 0x00a6_abb0),
            vec![vec![property_block, pixel_block, names[0][0], prefs_at]]
        );
        assert_eq!(calls_to(&e, 0x0043_81b0), vec![vec![names[0][0]]]);
        assert_eq!(calls_to(&e, 0x004f_3200), vec![vec![0x5150, 0]]);
        assert!(blocks.borrow().is_empty());
    }

    #[test]
    fn revision_bump_adds_one_at_0x68() {
        let mut e = Engine::new();
        let p = e.mem.alloc(0x70);
        e.mem.set_u32(p + 0x68, 41);
        e.call(0x004f_31b0, &args![p]);
        assert_eq!(e.mem.u32(p + 0x68), 42);
    }

    #[test]
    fn three_word_prefs_are_6_3_2() {
        let mut e = Engine::new();
        let p: Ptr<ThreeWordPrefs> = e.new_object();
        assert_eq!(e.call(0x004f_31d0, &args![p]).u32(), p.addr());
        assert_eq!(
            [
                e.get(p, ThreeWordPrefs::first),
                e.get(p, ThreeWordPrefs::second),
                e.get(p, ThreeWordPrefs::third)
            ],
            [6, 3, 2]
        );
    }

    #[test]
    fn set_base_clamp_mode_makes_the_base_map_when_missing() {
        let mut e = Engine::new();
        let property = e.mem.alloc(0x40);
        let entry = e.mem.alloc(4);
        fixed(&mut e, 0x0087_7a30, entry);
        stub(&mut e, &[0x004f_32e0]);

        // An existing map.
        e.mem.set_u32(entry, 0x7700);
        e.call_log = Some(vec![]);
        e.call(0x004f_3200, &args![property, 0x1_0002u32]);
        assert_eq!(calls_to(&e, 0x0087_7a30), vec![vec![property + 0x1c, 0]]);
        assert_eq!(
            calls_to(&e, 0x004f_32e0),
            vec![vec![0x7700, 2, 0x3000, 0x0c]]
        );

        // None: a new one is stored at index 0.
        e.mem.set_u32(entry, 0);
        let block = e.mem.alloc(0x10);
        fixed(&mut e, ALLOCATE, block);
        fixed(&mut e, 0x00a6_9dd0, 0x7800);
        let stored: Log = Rc::new(RefCell::new(vec![]));
        let seen = stored.clone();
        e.register_double(0x0096_ae90, move |e, a| {
            seen.borrow_mut().push((a[1], vec![e.mem.u32(a[2])]));
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x004f_3200, &args![property, 1u32]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x10]]);
        assert_eq!(calls_to(&e, 0x00a6_9dd0), vec![vec![block]]);
        assert_eq!(*stored.borrow(), vec![(0, vec![0x7800])]);
        assert_eq!(
            calls_to(&e, 0x004f_32e0),
            vec![vec![0x7800, 1, 0x3000, 0x0c]]
        );
    }

    #[test]
    fn map_clamp_mode_passes_the_mode_mask_and_shift() {
        let mut e = Engine::new();
        stub(&mut e, &[0x004f_32e0]);
        e.call_log = Some(vec![]);
        e.call(0x004f_32b0, &args![0x7700u32, 0xffff_0003u32]);
        assert_eq!(
            calls_to(&e, 0x004f_32e0),
            vec![vec![0x7700, 3, 0x3000, 0x0c]]
        );
    }
}
