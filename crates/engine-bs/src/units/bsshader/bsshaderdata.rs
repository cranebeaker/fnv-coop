//! `BSShader/bsshaderdata.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSShaderData` collects what a converted geometry's shader property needs (the lighting
//! shader kind, colours, alpha and the flag bytes) and makes the property from it
//! (`GetShaderProperty`, `AddShaderProperty`, `Convert`). The PC layout matches the Xbox
//! PDB's (0xb8 bytes, every offset the code uses).

#[allow(unused_imports)]
use crate::prelude::*;

/// The Ni allocation `00aa13e0` (`cdecl`, size).
const NI_ALLOCATE: u32 = 0x00aa_13e0;
/// The shader property constructors `GetShaderProperty` picks from (`thiscall`, no stack
/// words unless said): `00b66f50` (unit `bsshaderpplightingproperty.cpp`, 0x104 bytes),
/// `WaterShaderProperty::WaterShaderProperty` (`00b6abb0`, 0x150 bytes),
/// `BSShaderNoLightingProperty::BSShaderNoLightingProperty` (`00b6fc90`, 0x80 bytes),
/// `00bc37b0` (0x70 bytes), the `SkyShaderProperty` constructor `00baeb80` (0x90 bytes),
/// `00b60cb0` (0x98 bytes), `TileShaderProperty::TileShaderProperty` (`00bb7c30`, one word,
/// 0xb0 bytes) and `00c05880` (0x108 bytes) (names from the engine map).
const PP_LIGHTING_CONSTRUCT: u32 = 0x00b6_6f50;
const WATER_CONSTRUCT: u32 = 0x00b6_abb0;
const NO_LIGHTING_CONSTRUCT: u32 = 0x00b6_fc90;
const CONSTRUCT_00BC37B0: u32 = 0x00bc_37b0;
const SKY_CONSTRUCT: u32 = 0x00ba_eb80;
const CONSTRUCT_00B60CB0: u32 = 0x00b6_0cb0;
const TILE_CONSTRUCT: u32 = 0x00bb_7c30;
const CONSTRUCT_00C05880: u32 = 0x00c0_5880;
/// The `NiColor` the constructor copies into the specular, emittance and fog colours
/// (`011f4998`, three `float`s).
const DEFAULT_COLOR: u32 = 0x011f_4998;
/// The `float` constants the constructor loads: `01017b78` (the shininess and the fog
/// density) and `0101e570` (the parallax pass count).
const SHININESS_DEFAULT: u32 = 0x0101_7b78;
const PARALLAX_PASSES_DEFAULT: u32 = 0x0101_e570;
/// The `iShader` table by lighting shader kind (`011ae5a8`, one word per kind).
const SHADER_BY_KIND: u32 = 0x011a_e5a8;
/// Imported `InterlockedDecrement` (import slot).
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The empty `NiFixedString` handle (`0109b220`) the texture name falls back to.
const EMPTY_FIXED_STRING: u32 = 0x0109_b220;
/// The texture name read `00a5b690` (`cdecl`, the texture; answers a fixed string handle).
const TEXTURE_NAME_00A5B690: u32 = 0x00a5_b690;
/// The `BSStringT<char>` set `004037f0` (`thiscall`: the text, a length word).
const STRING_SET: u32 = 0x0040_37f0;
/// `NiAVObject::AttachProperty` (engine map, `00439410`, `thiscall`: the property).
const ATTACH_PROPERTY: u32 = 0x0043_9410;
/// Other functions of this unit (`thiscall` on the data, one word: the property):
/// `00bafa80` and `00baf520` (lighting kinds 8..12 and 7), `00bafaf0` (type 0x15),
/// `00bafcb0` (type 4) and `00bafc00` (type 0x13).
const SETUP_00BAFA80: u32 = 0x00ba_fa80;
const SETUP_00BAF520: u32 = 0x00ba_f520;
const SETUP_00BAFAF0: u32 = 0x00ba_faf0;
const SETUP_00BAFCB0: u32 = 0x00ba_fcb0;
const SETUP_00BAFC00: u32 = 0x00ba_fc00;
/// `BSShaderData::AddMaterialProperty` (`00bafe10`) and `::AddAlphaProperty` (`00baff20`)
/// (engine map; `thiscall`: the geometry).
const ADD_MATERIAL_PROPERTY: u32 = 0x00ba_fe10;
const ADD_ALPHA_PROPERTY: u32 = 0x00ba_ff20;

layout! {
    /// `BSShaderData` (Xbox PDB), 0xb8 bytes.
    pub struct BSShaderData: 0xB8 {
        /// `eLightingShader` (Xbox PDB).
        0x00 eLightingShader: u32,
        /// `SpecularColor` (Xbox PDB): an `NiColor`.
        0x04 SpecularColor: f32,
        /// `EmittanceColor` (Xbox PDB): an `NiColor`.
        0x10 EmittanceColor: f32,
        /// `fShininess` (Xbox PDB).
        0x1C fShininess: f32,
        /// `fAlpha` (Xbox PDB).
        0x20 fAlpha: f32,
        /// `pAlphaController` (Xbox PDB).
        0x30 pAlphaController: u32,
        /// `bVertexAlpha` (Xbox PDB).
        0x4D bVertexAlpha: bool,
        /// `VolumetricFogColor` (Xbox PDB): an `NiColor`, followed by
        /// `fVolumetricFogDensity` at +0x84.
        0x78 VolumetricFogColor: f32,
        /// `fEnvironmentMappingMaskScale` (Xbox PDB).
        0x8C fEnvironmentMappingMaskScale: f32,
        /// `iSkyTYpe` (Xbox PDB).
        0x90 iSkyTYpe: u32,
        /// `pTextureSet` (Xbox PDB): `BSShaderTextureSet*`.
        0xA8 pTextureSet: u32,
        /// `pbTextureHasAlpha` (Xbox PDB): one byte per texture slot.
        0xAC pbTextureHasAlpha: u8,
        /// `eClampMode` (Xbox PDB).
        0xB4 eClampMode: u32,
    }
}

/// A reference-counted object released: `InterlockedDecrement` on its count at +4, and
/// deleted through its vtable slot +4 when that answers 0.
fn release_reference(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// The shader property's inlined flag setter: `bit` of the flag word at `word` (+0x20 or
/// +0x24) set or cleared, `iLastRenderPassState` (+0x38) cleared when the bit changes.
fn set_property_flag(e: &mut Engine, property: u32, word: u32, bit: u32, on: bool) {
    let flags = e.mem.u32(property + word);
    if on {
        if flags & bit == 0 {
            e.mem.set_u32(property + 0x38, 0);
        }
        let flags = e.mem.u32(property + word);
        e.mem.set_u32(property + word, flags | bit);
    } else {
        if flags & bit != 0 {
            e.mem.set_u32(property + 0x38, 0);
        }
        let flags = e.mem.u32(property + word);
        e.mem.set_u32(property + word, flags & !bit);
    }
}

/// An allocation of `size` bytes constructed by `constructor` (with `extra` words), 0 when
/// the allocation fails.
fn make(e: &mut Engine, size: u32, constructor: u32, extra: &[u32]) -> u32 {
    let block = e.call(NI_ALLOCATE, &args![size]).u32();
    if block == 0 {
        return 0;
    }
    let mut words = vec![block];
    words.extend_from_slice(extra);
    e.call(constructor, &words).u32()
}

// Translated from 00baf120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderData::GetShaderProperty` (engine map; `thiscall`, no stack words): a new shader
/// property for `eLightingShader` 0 to 9 (0, 1, 2: `00b66f50`; 3: water; 4: no lighting; 5:
/// `00bc37b0`; 6: sky; 7: `00b60cb0`; 8: tile, with word 0; 9: `00c05880`), 0 for any other
/// kind or when the allocation fails.
// Translation incomplete: 00baf120 - the C++ exception frame (FS:[0] chain, unwind states 0 to 9 for the allocations) is not translated.
pub fn bs_shader_data_get_shader_property(e: &mut Engine, this: Ptr<BSShaderData>) -> u32 {
    match e.get(this, BSShaderData::eLightingShader) {
        0..=2 => make(e, 0x104, PP_LIGHTING_CONSTRUCT, &[]),
        3 => make(e, 0x150, WATER_CONSTRUCT, &[]),
        4 => make(e, 0x80, NO_LIGHTING_CONSTRUCT, &[]),
        5 => make(e, 0x70, CONSTRUCT_00BC37B0, &[]),
        6 => make(e, 0x90, SKY_CONSTRUCT, &[]),
        7 => make(e, 0x98, CONSTRUCT_00B60CB0, &[]),
        8 => make(e, 0xb0, TILE_CONSTRUCT, &[0]),
        9 => make(e, 0x108, CONSTRUCT_00C05880, &[]),
        _ => 0,
    }
}

// Translated from 00baf3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderData::BSShaderData` (engine map; `thiscall`, no stack words): lighting kind 0,
/// the specular, emittance and fog colours from `011f4998`, shininess and fog density from
/// `01017b78`, alpha 1, emittance multiplier 1, no controllers, no alpha blend, blend modes 6
/// and 7, no alpha test (function 4, reference 0), every flag byte clear but z-buffer write
/// and test, refraction power 0, parallax height scale 1 and passes from `0101e570`, mask
/// scale 1, sky type 0, no falloff (colour 0), no texture set, no texture alpha, clamp mode 3.
pub fn bs_shader_data_constructor(e: &mut Engine, this: Ptr<BSShaderData>) {
    let base = this.addr();
    for offset in [
        0x04u32, 0x08, 0x0c, 0x10, 0x14, 0x18, 0x78, 0x7c, 0x80, 0x98, 0x9c, 0xa0, 0xa4,
    ] {
        e.mem.set_f32(base + offset, 0.0);
    }
    let shininess = x87_quiet(e.global::<f32>(SHININESS_DEFAULT));
    e.mem.set_u32(base, 0);
    for offset in [0x04u32, 0x10] {
        for word in 0..3 {
            let value = e.mem.u32(DEFAULT_COLOR + 4 * word);
            e.mem.set_u32(base + offset + 4 * word, value);
        }
    }
    e.mem.set_f32(base + 0x1c, shininess);
    e.mem.set_f32(base + 0x20, 1.0);
    e.mem.set_f32(base + 0x24, 1.0);
    e.mem.set_u32(base + 0x28, 0);
    e.mem.set_u32(base + 0x2c, 0);
    e.mem.set_f32(base + 0x58, 0.0);
    e.mem.set_u32(base + 0x30, 0);
    e.mem.set_u32(base + 0x34, 0);
    e.mem.set_f32(base + 0x70, 1.0);
    e.mem.set_u8(base + 0x38, 0);
    let passes = x87_quiet(e.global::<f32>(PARALLAX_PASSES_DEFAULT));
    e.mem.set_u32(base + 0x3c, 6);
    e.mem.set_f32(base + 0x74, passes);
    e.mem.set_u32(base + 0x40, 7);
    e.mem.set_u8(base + 0x44, 0);
    e.mem.set_u32(base + 0x48, 4);
    for offset in 0x4cu32..=0x54 {
        e.mem.set_u8(base + offset, 0);
    }
    e.mem.set_u8(base + 0x55, 1);
    e.mem.set_u8(base + 0x56, 1);
    e.mem.set_u8(base + 0x57, 0);
    e.mem.set_u8(base + 0x5c, 0);
    e.mem.set_u32(base + 0x60, 0);
    e.mem.set_u32(base + 0x64, 0);
    e.mem.set_u32(base + 0x68, 0);
    e.mem.set_u8(base + 0x6c, 0);
    e.mem.set_u8(base + 0x6d, 0);
    for word in 0..3 {
        let value = e.mem.u32(DEFAULT_COLOR + 4 * word);
        e.mem.set_u32(base + 0x78 + 4 * word, value);
    }
    e.mem.set_f32(base + 0x84, shininess);
    e.mem.set_f32(base + 0x8c, 1.0);
    e.mem.set_u8(base + 0x88, 0);
    e.mem.set_u8(base + 0x89, 0);
    e.mem.set_u8(base + 0x8a, 0);
    e.mem.set_u32(base + 0xa8, 0);
    e.mem.set_u32(base + 0xac, 0);
    e.mem.set_u16(base + 0xb0, 0);
    e.mem.set_u32(base + 0xb4, 3);
}

// Translated from 00baf580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): with a texture set, for each of
/// the six slots 0..5 asks the set for its texture (vtable slot +0x90: the slot, the address
/// of an `NiPointer` local cleared first) and marks `pbTextureHasAlpha[slot]` when the
/// texture's `+0x24` record (its `+0x14` block, or 0 without the record) has format word
/// `+4` 5 or 6; the local's texture is then released.
// Translation incomplete: 00baf580 - the C++ exception frame (FS:[0] chain, unwind state 0 for the texture pointer) is not translated.
pub fn fn_00baf580(e: &mut Engine, this: Ptr<BSShaderData>) {
    let base = this.addr();
    if e.mem.u32(base + 0xa8) == 0 {
        return;
    }
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for slot in 0..6u32 {
            e.mem.set_u32(local, 0);
            let set = e.mem.u32(base + 0xa8);
            e.vcall(set, 0x90, &args![slot, local]);
            let texture = e.mem.u32(local);
            if texture != 0 {
                let record = e.mem.u32(texture + 0x24);
                let block = if record != 0 { record + 0x14 } else { 0 };
                let format = e.mem.u32(block + 4);
                if format == 5 || format == 6 {
                    e.mem.set_u8(base + 0xac + slot, 1);
                }
            }
            if texture != 0 {
                release_reference(e, texture);
            }
        }
    });
}

// Translated from 00baf640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the shader property, the geometry):
/// sets or clears the property's flags from the data, each through the inlined setter (the
/// pass state cleared when a bit changes), in this order (by mask): `ulFlags[0]` 2 (the geometry's
/// vtable slot +0x18 answers non-zero and it has a skin instance, `+0xbc`), 8 (`+0x4d`), 0x80
/// (`+0x88`), 0x80000 (`pAlphaController`), 0x100 (`+0xac`), 0x400 (kind 1), 0x800 (`+0x6c`),
/// 0x10000000 (`+0x6d`), 0x1000 (`+0x4e`), 0x8000 (`+0x57`), 0x10000 (`+0x5c`), 0x40000 (kind
/// 2), 0x2000000 (`+0x4f`), 0x4000000 and 0x8000000 (both `+0x50`), 0x80000000 (`+0x56`);
/// `ulFlags[1]` 1 (`+0x55`); `ulFlags[0]` 0x100000 (`+0x52`); `ulFlags[1]` 8 (`+0x53`) and
/// 0x8000 (`+0x54`); `ulFlags[0]` 0x40 (`+0x94`), 1 (`+0xad`), 0x20000 (`+0x89`), 0x200000
/// (`+0x8a`) and 0x20000000 (`+0x51`). Then the property's `+0x30` takes the mask scale.
pub fn fn_00baf640(e: &mut Engine, this: Ptr<BSShaderData>, property: u32, geometry: u32) {
    let base = this.addr();
    let skinned = e.vcall(geometry, 0x18, &args![]).u32() != 0 && e.mem.u32(geometry + 0xbc) != 0;
    set_property_flag(e, property, 0x20, 2, skinned);
    let byte = |e: &Engine, offset: u32| e.mem.u8(base + offset) != 0;
    let on = byte(e, 0x4d);
    set_property_flag(e, property, 0x20, 8, on);
    let on = byte(e, 0x88);
    set_property_flag(e, property, 0x20, 0x80, on);
    let on = e.mem.u32(base + 0x30) != 0;
    set_property_flag(e, property, 0x20, 0x80000, on);
    let on = byte(e, 0xac);
    set_property_flag(e, property, 0x20, 0x100, on);
    let on = e.mem.u32(base) == 1;
    set_property_flag(e, property, 0x20, 0x400, on);
    for (offset, bit) in [
        (0x6cu32, 0x800u32),
        (0x6d, 0x1000_0000),
        (0x4e, 0x1000),
        (0x57, 0x8000),
        (0x5c, 0x10000),
    ] {
        let on = byte(e, offset);
        set_property_flag(e, property, 0x20, bit, on);
    }
    let on = e.mem.u32(base) == 2;
    set_property_flag(e, property, 0x20, 0x40000, on);
    for (offset, word, bit) in [
        (0x4fu32, 0x20u32, 0x0200_0000u32),
        (0x50, 0x20, 0x0400_0000),
        (0x50, 0x20, 0x0800_0000),
        (0x56, 0x20, 0x8000_0000),
        (0x55, 0x24, 1),
        (0x52, 0x20, 0x10_0000),
        (0x53, 0x24, 8),
        (0x54, 0x24, 0x8000),
        (0x94, 0x20, 0x40),
        (0xad, 0x20, 1),
        (0x89, 0x20, 0x2_0000),
        (0x8a, 0x20, 0x20_0000),
        (0x51, 0x20, 0x2000_0000),
    ] {
        let on = byte(e, offset);
        set_property_flag(e, property, word, bit, on);
    }
    let scale = x87_quiet(e.mem.f32(base + 0x8c));
    e.mem.set_f32(property + 0x30, scale);
}

// Translated from 00bafd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word: a `SkyShaderProperty`): with a texture
/// set, takes the name of its texture 0 (vtable slot +0x8c with 0; `00a5b690` on it, or the
/// empty fixed string `0109b220` without a texture) into the property's `TextureFilename`
/// (`+0x74`, length word 0), releases the fixed string's reference (the count 8 bytes before
/// it, unless it is the empty string) and sets the property's clamp mode through its vtable
/// slot +0xd0 (the Xbox PDB's `SetClampNI` slot) from `eClampMode`.
// Translation incomplete: 00bafd50 - the C++ exception frame (FS:[0] chain, unwind state 0 for the fixed string) is not translated.
pub fn fn_00bafd50(e: &mut Engine, this: Ptr<BSShaderData>, property: u32) {
    let base = this.addr();
    if e.mem.u32(base + 0xa8) == 0 {
        return;
    }
    let set = e.mem.u32(base + 0xa8);
    let texture = e.vcall(set, 0x8c, &args![0u32]).u32();
    let name = if texture == 0 {
        e.global::<u32>(EMPTY_FIXED_STRING)
    } else {
        e.call(TEXTURE_NAME_00A5B690, &args![texture]).u32()
    };
    e.call(STRING_SET, &args![property + 0x74, name, 0u32]);
    if name != e.global::<u32>(EMPTY_FIXED_STRING) {
        e.call(IMPORT_INTERLOCKED_DECREMENT, &args![name.wrapping_sub(8)]);
    }
    let clamp = e.mem.u32(base + 0xb4);
    e.vcall(property, 0xd0, &args![clamp]);
}

// Translated from 00bb0040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderData::AddShaderProperty` (engine map; `thiscall`, one word: the geometry). Makes
/// the property (`GetShaderProperty`); when there is one: the texture alpha marks
/// (`00baf580`), the flags (`00baf640`), then by the property's type (`+0x1c`, read again
/// for each test): 8..12 `00bafa80` and `00baf520`; 0x15 `00bafaf0`; 0x16 the fog colour and
/// density into `+0x60..+0x6c`; 4 `00bafcb0`; 0xd `00bafd50` and the sky type into `+0x8c`;
/// 0x13 `00bafc00`; 7 `00bafa80` and `00baf520`. Then `iShader` from the table at `011ae5a8`
/// by lighting kind, and the property is attached to the geometry.
pub fn bs_shader_data_add_shader_property(e: &mut Engine, this: Ptr<BSShaderData>, geometry: u32) {
    let base = this.addr();
    let property = bs_shader_data_get_shader_property(e, this);
    if property == 0 {
        return;
    }
    fn_00baf580(e, this);
    fn_00baf640(e, this, property, geometry);
    let kind = e.mem.i32(property + 0x1c);
    if (8..=12).contains(&kind) {
        e.call(SETUP_00BAFA80, &args![base, property]);
        e.call(SETUP_00BAF520, &args![base, property]);
    }
    if e.mem.u32(property + 0x1c) == 0x15 {
        e.call(SETUP_00BAFAF0, &args![base, property]);
    }
    if e.mem.u32(property + 0x1c) == 0x16 {
        for word in 0..4 {
            let value = x87_quiet(e.mem.f32(base + 0x78 + 4 * word));
            e.mem.set_f32(property + 0x60 + 4 * word, value);
        }
    }
    if e.mem.u32(property + 0x1c) == 4 {
        e.call(SETUP_00BAFCB0, &args![base, property]);
    }
    if e.mem.u32(property + 0x1c) == 0xd {
        fn_00bafd50(e, this, property);
        let sky = e.mem.u32(base + 0x90);
        e.mem.set_u32(property + 0x8c, sky);
    }
    if e.mem.u32(property + 0x1c) == 0x13 {
        e.call(SETUP_00BAFC00, &args![base, property]);
    }
    if e.mem.u32(property + 0x1c) == 7 {
        e.call(SETUP_00BAFA80, &args![base, property]);
        e.call(SETUP_00BAF520, &args![base, property]);
    }
    let kind = e.mem.u32(base);
    let shader = e.mem.u32(SHADER_BY_KIND.wrapping_add(kind.wrapping_mul(4)));
    e.mem.set_u32(property + 0x58, shader);
    e.call(ATTACH_PROPERTY, &args![geometry, property]);
}

// Translated from 00bb0180 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSShaderData::Convert` (engine map; `thiscall`, two words: the geometry, the flags byte):
/// `AddShaderProperty` with bit 0, `AddMaterialProperty` (`00bafe10`) with bit 1 and
/// `AddAlphaProperty` (`00baff20`) with bit 2.
pub fn bs_shader_data_convert(e: &mut Engine, this: Ptr<BSShaderData>, geometry: u32, flags: u8) {
    if flags & 1 != 0 {
        bs_shader_data_add_shader_property(e, this, geometry);
    }
    if flags & 2 != 0 {
        e.call(ADD_MATERIAL_PROPERTY, &args![this, geometry]);
    }
    if flags & 4 != 0 {
        e.call(ADD_ALPHA_PROPERTY, &args![this, geometry]);
    }
}

/// The calls of the property set-ups: the `float` setter `00b66700` (`thiscall`, one
/// `float`), `InterlockedIncrement` (import slot), the `NiMaterialProperty` constructor
/// (`00a75650`, `thiscall`), the `NiProperty` base constructor (`00a5c200`), the
/// `NiAlphaProperty` vtable (`010162dc`) and the falloff "unset" value -1.0 (`01012054`).
const SET_VALUE_00B66700: u32 = 0x00b6_6700;
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const MATERIAL_PROPERTY_CONSTRUCT: u32 = 0x00a7_5650;
const PROPERTY_BASE_CONSTRUCT: u32 = 0x00a5_c200;
const ALPHA_PROPERTY_VTABLE: u32 = 0x0101_62dc;
const UNSET_01012054: u32 = 0x0101_2054;

/// The texture set's first texture name (slot +0x8c with 0) as a fixed string (`00a5b690`),
/// or the shared empty handle (the word at `0109b220`) when it has none.
fn first_texture_name(e: &mut Engine, set: u32) -> u32 {
    let name = e.vcall(set, 0x8c, &args![0u32]).u32();
    if name == 0 {
        e.mem.u32(EMPTY_FIXED_STRING)
    } else {
        e.call(TEXTURE_NAME_00A5B690, &args![name]).u32()
    }
}

/// A fixed string handle released: unless it is the shared empty one, the count 8 bytes
/// before it is lowered (`InterlockedDecrement`).
fn release_fixed_string(e: &mut Engine, handle: u32) {
    if handle != e.mem.u32(EMPTY_FIXED_STRING) {
        e.call(IMPORT_INTERLOCKED_DECREMENT, &args![handle.wrapping_sub(8)]);
    }
}

/// The property's file-name string at `offset` set from the texture set's first texture
/// name (`004037f0` with the handle and 0), the handle released afterwards.
fn set_file_name(e: &mut Engine, this: Ptr<BSShaderData>, property: u32, offset: u32) {
    let set = e.get(this, BSShaderData::pTextureSet);
    let handle = first_texture_name(e, set);
    e.call(
        STRING_SET,
        &args![property.wrapping_add(offset), handle, 0u32],
    );
    release_fixed_string(e, handle);
}

// Translated from 00baf520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with one word (a property; lighting kind 7): the
/// `float` at `+0x58` through `00b66700`, the word `+0x60` at the property's `+0xe4`, the
/// `float`s `+0x74` and `+0x70` (`FLD`/`FSTP`) at `+0xe8` and `+0xec`, then the objects at
/// `+0x64` and `+0x68`, when present, get their slot +0x98 with the property.
pub fn fn_00baf520(e: &mut Engine, this: Ptr<BSShaderData>, property: u32) {
    let base = this.addr();
    let value = x87_quiet_bits(e.mem.u32(base + 0x58));
    e.call(SET_VALUE_00B66700, &args![property, value]);
    let v = e.mem.u32(base + 0x60);
    e.mem.set_u32(property.wrapping_add(0xe4), v);
    let v = x87_quiet_bits(e.mem.u32(base + 0x74));
    e.mem.set_u32(property.wrapping_add(0xe8), v);
    let v = x87_quiet_bits(e.mem.u32(base + 0x70));
    e.mem.set_u32(property.wrapping_add(0xec), v);
    for offset in [0x64u32, 0x68] {
        let object = e.mem.u32(base + offset);
        if object != 0 {
            e.vcall(object, 0x98, &args![property]);
        }
    }
}

// Translated from 00bafa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with one word (a property; lighting kinds 8 to 12):
/// the texture set (`pTextureSet`) assigned to the property's `NiPointer` at `+0xa4`, then
/// the property's slot +0xe4 with `eClampMode`.
pub fn fn_00bafa80(e: &mut Engine, this: Ptr<BSShaderData>, property: u32) {
    let set = e.get(this, BSShaderData::pTextureSet);
    let slot = property.wrapping_add(0xa4);
    let old = e.mem.u32(slot);
    if old != set {
        if old != 0 {
            release_reference(e, old);
        }
        e.mem.set_u32(slot, set);
        if set != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![set.wrapping_add(4)]);
        }
    }
    let clamp = e.get(this, BSShaderData::eClampMode);
    e.vcall(property, 0xe4, &args![clamp]);
}

/// The falloff copy: each of the property's four `float`s `+0x70` to `+0x7c` that is not
/// -1.0 (a NaN counts as not equal) takes the data's `float` at `+0x98` on (`FLD`/`FSTP`).
fn copy_falloff(e: &mut Engine, this: u32, property: u32) {
    let unset = x87_m32(e.mem.u32(UNSET_01012054));
    for k in 0..4u32 {
        let field = property.wrapping_add(0x70 + 4 * k);
        if unset != x87_m32(e.mem.u32(field)) {
            let v = x87_quiet_bits(e.mem.u32(this + 0x98 + 4 * k));
            e.mem.set_u32(field, v);
        }
    }
}

// Translated from 00bafaf0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bafaf0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map, `thiscall` with one word (a property of type 0x15, a
/// `BSShaderNoLightingProperty`): with a texture set, the property's file name (`+0x64`) set
/// from its first texture name and the property's slot +0xd0 called with `eClampMode`; then
/// the falloff `float`s copied ([`copy_falloff`]).
pub fn fn_00bafaf0(e: &mut Engine, this: Ptr<BSShaderData>, property: u32) {
    if e.get(this, BSShaderData::pTextureSet) != 0 {
        set_file_name(e, this, property, 0x64);
        let clamp = e.get(this, BSShaderData::eClampMode);
        e.vcall(property, 0xd0, &args![clamp]);
    }
    copy_falloff(e, this.addr(), property);
}

// Translated from 00bafc00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bafc00 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map, `thiscall` with one word (a property of type 0x13): with a
/// texture set, the property's file name at `+0x94` set from its first texture name and
/// `eClampMode` stored at the property's `+0x8c`.
pub fn fn_00bafc00(e: &mut Engine, this: Ptr<BSShaderData>, property: u32) {
    if e.get(this, BSShaderData::pTextureSet) == 0 {
        return;
    }
    set_file_name(e, this, property, 0x94);
    let clamp = e.get(this, BSShaderData::eClampMode);
    e.mem.set_u32(property.wrapping_add(0x8c), clamp);
}

// Translated from 00bafcb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bafcb0 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// Unnamed in the engine map, `thiscall` with one word (a property of type 4): with a texture
/// set, the property's file name at `+0x88` set from its first texture name.
pub fn fn_00bafcb0(e: &mut Engine, this: Ptr<BSShaderData>, property: u32) {
    if e.get(this, BSShaderData::pTextureSet) == 0 {
        return;
    }
    set_file_name(e, this, property, 0x88);
}

// Translated from 00bafe10 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bafe10 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `BSShaderData::AddMaterialProperty` (engine map; the body matches): `thiscall`, one word
/// (a geometry). A new 0x4c-byte `NiMaterialProperty` (`00aa13e0`, `00a75650`; null when the
/// allocation fails, and the exe goes on with null) gets the specular and emittance colours
/// (`+4` to `+0x18` at its `+0x1c` to `+0x30`) and the three `float`s `+0x1c` to `+0x24`
/// (`FLD`/`FSTP`, at `+0x38` to `+0x40`), its revision word `+0x44` raised by five on the way;
/// the controllers at `+0x28` to `+0x34`, when present, get their slot +0x98 with it; then it
/// is attached to the geometry (`00439410`).
pub fn bs_shader_data_add_material_property(
    e: &mut Engine,
    this: Ptr<BSShaderData>,
    geometry: u32,
) {
    let base = this.addr();
    let block = e.call(NI_ALLOCATE, &args![0x4cu32]).u32();
    let material = if block != 0 {
        e.call(MATERIAL_PROPERTY_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    let v = e.mem.u32(base + 4);
    e.mem.set_u32(material.wrapping_add(0x1c), v);
    let v = e.mem.u32(base + 8);
    e.mem.set_u32(material.wrapping_add(0x20), v);
    let v = e.mem.u32(base + 0xc);
    let revision = e.mem.u32(material.wrapping_add(0x44)).wrapping_add(1);
    e.mem.set_u32(material.wrapping_add(0x44), revision);
    e.mem.set_u32(material.wrapping_add(0x24), v);
    let v = e.mem.u32(base + 0x10);
    e.mem.set_u32(material.wrapping_add(0x28), v);
    let v = e.mem.u32(base + 0x14);
    e.mem.set_u32(material.wrapping_add(0x2c), v);
    let v = e.mem.u32(base + 0x18);
    let mut revision = revision.wrapping_add(1);
    e.mem.set_u32(material.wrapping_add(0x44), revision);
    e.mem.set_u32(material.wrapping_add(0x30), v);
    for (source, target) in [(0x1cu32, 0x38u32), (0x20, 0x3c), (0x24, 0x40)] {
        let v = x87_quiet_bits(e.mem.u32(base + source));
        revision = revision.wrapping_add(1);
        e.mem.set_u32(material.wrapping_add(target), v);
        e.mem.set_u32(material.wrapping_add(0x44), revision);
    }
    for offset in [0x28u32, 0x2c, 0x30, 0x34] {
        let controller = e.mem.u32(base + offset);
        if controller != 0 {
            e.vcall(controller, 0x98, &args![material]);
        }
    }
    e.call(ATTACH_PROPERTY, &args![geometry, material]);
}

// Translated from 00baff20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00baff20 - the FS:[0] exception frame (unwinding on a throw) is not translated
/// `BSShaderData::AddAlphaProperty` (engine map; the body matches): `thiscall`, one word (a
/// geometry). Only when blending (`+0x38`) or testing (`+0x44`) is on and the byte `+0x57` is
/// clear: a new 0x1c-byte `NiAlphaProperty` (`00aa13e0`, the `NiProperty` base `00a5c200`,
/// vtable `010162dc`, flags 0xec, test reference 0; null when the allocation fails, and the
/// exe goes on with null) gets its flag word (`+0x18`): bit 0 the blending, bits 1 to 4 the
/// source blend (`+0x3c`), bits 5 to 8 the destination blend (`+0x40`), bit 9 the testing,
/// bits 10 to 12 the test mode (`+0x48`), and its test reference (`+0x1a`) the byte `+0x4c`;
/// then it is attached to the geometry (`00439410`).
pub fn bs_shader_data_add_alpha_property(e: &mut Engine, this: Ptr<BSShaderData>, geometry: u32) {
    let base = this.addr();
    if e.mem.u8(base + 0x38) == 0 && e.mem.u8(base + 0x44) == 0 {
        return;
    }
    if e.mem.u8(base + 0x57) != 0 {
        return;
    }
    let block = e.call(NI_ALLOCATE, &args![0x1cu32]).u32();
    let alpha = if block != 0 {
        e.call(PROPERTY_BASE_CONSTRUCT, &args![block]);
        e.mem.set_u32(block, ALPHA_PROPERTY_VTABLE);
        e.mem.set_u16(block + 0x18, 0xec);
        e.mem.set_u8(block + 0x1a, 0);
        block
    } else {
        0
    };
    let flags_at = alpha.wrapping_add(0x18);
    let flags = e.mem.u16(flags_at);
    let flags = if e.mem.u8(base + 0x38) != 0 {
        flags | 1
    } else {
        flags & 0xfffe
    };
    e.mem.set_u16(flags_at, flags);
    let source = e.mem.u16(base + 0x3c).wrapping_add(e.mem.u16(base + 0x3c));
    let flags = source | (e.mem.u16(flags_at) & 0xffe1);
    e.mem.set_u16(flags_at, flags);
    let destination = e.mem.u16(base + 0x40).wrapping_shl(5);
    let flags = destination | (flags & 0xfe1f);
    e.mem.set_u16(flags_at, flags);
    let flags = if e.mem.u8(base + 0x44) != 0 {
        flags | 0x200
    } else {
        flags & 0xfdff
    };
    e.mem.set_u16(flags_at, flags);
    let reference = e.mem.u8(base + 0x4c);
    e.mem.set_u8(alpha.wrapping_add(0x1a), reference);
    let mode = e.mem.u16(base + 0x48).wrapping_shl(0xa);
    let flags = mode | (e.mem.u16(flags_at) & 0xe3ff);
    e.mem.set_u16(flags_at, flags);
    e.call(ATTACH_PROPERTY, &args![geometry, alpha]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ba_f120,
            bs_shader_data_get_shader_property(Ptr<BSShaderData>) -> u32
        ),
        entry!(0x00ba_f3c0, bs_shader_data_constructor(Ptr<BSShaderData>)),
        entry!(0x00ba_f580, fn_00baf580(Ptr<BSShaderData>)),
        entry!(0x00ba_f640, fn_00baf640(Ptr<BSShaderData>, u32, u32)),
        entry!(0x00ba_fd50, fn_00bafd50(Ptr<BSShaderData>, u32)),
        entry!(
            0x00bb_0040,
            bs_shader_data_add_shader_property(Ptr<BSShaderData>, u32)
        ),
        entry!(
            0x00bb_0180,
            bs_shader_data_convert(Ptr<BSShaderData>, u32, u8)
        ),
        entry!(0x00ba_f520, fn_00baf520(Ptr<BSShaderData>, u32)),
        entry!(0x00ba_fa80, fn_00bafa80(Ptr<BSShaderData>, u32)),
        entry!(0x00ba_faf0, fn_00bafaf0(Ptr<BSShaderData>, u32)),
        entry!(0x00ba_fc00, fn_00bafc00(Ptr<BSShaderData>, u32)),
        entry!(0x00ba_fcb0, fn_00bafcb0(Ptr<BSShaderData>, u32)),
        entry!(
            0x00ba_fe10,
            bs_shader_data_add_material_property(Ptr<BSShaderData>, u32)
        ),
        entry!(
            0x00ba_ff20,
            bs_shader_data_add_alpha_property(Ptr<BSShaderData>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
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
        for &a in addrs {
            e.register(a, |_, _| Ret::default());
        }
    }

    const CONSTRUCTORS: [u32; 8] = [
        PP_LIGHTING_CONSTRUCT,
        WATER_CONSTRUCT,
        NO_LIGHTING_CONSTRUCT,
        CONSTRUCT_00BC37B0,
        SKY_CONSTRUCT,
        CONSTRUCT_00B60CB0,
        TILE_CONSTRUCT,
        CONSTRUCT_00C05880,
    ];

    fn constructing(e: &mut Engine) {
        for &a in &CONSTRUCTORS {
            e.register(a, |_, a| Ret {
                eax: a[0],
                ..Ret::default()
            });
        }
    }

    #[test]
    fn get_shader_property_picks_the_constructor_by_kind() {
        let cases = [
            (0u32, 0x104u32, PP_LIGHTING_CONSTRUCT),
            (1, 0x104, PP_LIGHTING_CONSTRUCT),
            (2, 0x104, PP_LIGHTING_CONSTRUCT),
            (3, 0x150, WATER_CONSTRUCT),
            (4, 0x80, NO_LIGHTING_CONSTRUCT),
            (5, 0x70, CONSTRUCT_00BC37B0),
            (6, 0x90, SKY_CONSTRUCT),
            (7, 0x98, CONSTRUCT_00B60CB0),
            (8, 0xb0, TILE_CONSTRUCT),
            (9, 0x108, CONSTRUCT_00C05880),
        ];
        for (kind, size, constructor) in cases {
            let mut e = Engine::new();
            constructing(&mut e);
            e.register(NI_ALLOCATE, |_, a| Ret {
                eax: 0x1000_0000 + a[0],
                ..Ret::default()
            });
            let this = e.mem.alloc(0xb8);
            e.mem.set_u32(this, kind);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x00ba_f120, &args![this]).u32(), 0x1000_0000 + size);
            assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![size]]);
            let block = 0x1000_0000 + size;
            let expected = if kind == 8 {
                vec![block, 0]
            } else {
                vec![block]
            };
            assert_eq!(calls(&e, constructor), vec![expected]);
        }
        let mut e = Engine::new();
        constructing(&mut e);
        quiet(&mut e, &[NI_ALLOCATE]);
        let this = e.mem.alloc(0xb8);
        e.call_log = Some(vec![]);
        // A failed allocation and an unknown kind answer 0 without constructing.
        assert_eq!(e.call(0x00ba_f120, &args![this]).u32(), 0);
        e.mem.set_u32(this, 10);
        assert_eq!(e.call(0x00ba_f120, &args![this]).u32(), 0);
        assert_eq!(calls(&e, NI_ALLOCATE).len(), 1);
        assert!(calls(&e, PP_LIGHTING_CONSTRUCT).is_empty());
    }

    #[test]
    fn constructor_sets_the_defaults() {
        let mut e = Engine::new();
        e.map(DEFAULT_COLOR, 12);
        e.map(SHININESS_DEFAULT, 4);
        e.map(PARALLAX_PASSES_DEFAULT, 4);
        e.set_global(DEFAULT_COLOR, 0.25f32);
        e.set_global(DEFAULT_COLOR + 4, 0.5f32);
        e.set_global(DEFAULT_COLOR + 8, 0.75f32);
        e.set_global(SHININESS_DEFAULT, 10.0f32);
        e.set_global(PARALLAX_PASSES_DEFAULT, 4.0f32);
        let this = e.mem.alloc(0xb8);
        for offset in (0..0xb8).step_by(4) {
            e.mem.set_u32(this + offset, 0xdead_beef);
        }
        e.call(0x00ba_f3c0, &args![this]);
        assert_eq!(e.mem.u32(this), 0);
        for offset in [0x04u32, 0x10, 0x78] {
            assert_eq!(e.mem.f32(this + offset), 0.25);
            assert_eq!(e.mem.f32(this + offset + 8), 0.75);
        }
        assert_eq!(e.mem.f32(this + 0x1c), 10.0);
        assert_eq!(e.mem.f32(this + 0x84), 10.0);
        assert_eq!(e.mem.f32(this + 0x20), 1.0);
        assert_eq!(e.mem.f32(this + 0x24), 1.0);
        assert_eq!(e.mem.f32(this + 0x70), 1.0);
        assert_eq!(e.mem.f32(this + 0x74), 4.0);
        assert_eq!(e.mem.f32(this + 0x8c), 1.0);
        assert_eq!(e.mem.f32(this + 0x58), 0.0);
        assert_eq!(e.mem.u32(this + 0x3c), 6);
        assert_eq!(e.mem.u32(this + 0x40), 7);
        assert_eq!(e.mem.u32(this + 0x48), 4);
        assert_eq!(e.mem.u32(this + 0x4c), 0);
        assert_eq!(e.mem.u32(this + 0x54), 0x0001_0100);
        assert_eq!(e.mem.u32(this + 0x98), 0);
        assert_eq!(e.mem.u32(this + 0xa8), 0);
        assert_eq!(e.mem.u32(this + 0xac), 0);
        assert_eq!(e.mem.u16(this + 0xb0), 0);
        assert_eq!(e.mem.u32(this + 0xb4), 3);
        // Words the constructor leaves alone.
        assert_eq!(e.mem.u32(this + 0x90), 0xdead_beef);
    }

    #[test]
    fn texture_alpha_marks_follow_the_texture_formats() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            Ret {
                eax: v,
                ..Ret::default()
            }
        });
        // Textures for slots 0..5: formats 5, 6, 7, none, 5 without a record (reads 0+0x14),
        // no texture.
        let mut textures = vec![];
        for format in [5u32, 6, 7, 1] {
            let record = e.mem.alloc(0x20);
            e.mem.set_u32(record + 0x18, format);
            let texture = e.mem.alloc(0x28);
            e.mem.set_u32(texture + 4, 2);
            e.mem.set_u32(texture + 0x24, record);
            textures.push(texture);
        }
        e.map(0, 0x20);
        e.mem.set_u32(4, 5);
        let bare = e.mem.alloc(0x28);
        e.mem.set_u32(bare + 4, 2);
        textures.push(bare);
        textures.push(0);
        e.register_double(0x0000_4440, move |e, a| {
            e.mem.set_u32(a[2], textures[a[1] as usize]);
            Ret::default()
        });
        let vtable = e.mem.alloc(0x94);
        e.mem.set_u32(vtable + 0x90, 0x0000_4440);
        let set = e.mem.alloc(4);
        e.mem.set_u32(set, vtable);
        let this = e.mem.alloc(0xb8);
        e.call(0x00ba_f580, &args![this]);
        assert_eq!(e.mem.u32(this + 0xac), 0);
        e.mem.set_u32(this + 0xa8, set);
        e.call_log = Some(vec![]);
        e.call(0x00ba_f580, &args![this]);
        let marks: Vec<u8> = (0..6).map(|i| e.mem.u8(this + 0xac + i)).collect();
        assert_eq!(marks, vec![1, 1, 0, 0, 1, 0]);
        assert_eq!(calls(&e, 0x0000_4440).len(), 6);
        assert_eq!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).len(), 5);
    }

    #[test]
    fn flags_follow_the_data() {
        let mut e = Engine::new();
        e.register(0x0000_5550, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let vtable = e.mem.alloc(0x1c);
        e.mem.set_u32(vtable + 0x18, 0x0000_5550);
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(geometry, vtable);
        e.mem.set_u32(geometry + 0xbc, 1);
        let this = e.mem.alloc(0xb8);
        let property = e.mem.alloc(0x40);
        // Everything on: kind 1, every byte set, an alpha controller.
        e.mem.set_u32(this, 1);
        for offset in 0x4du32..=0x57 {
            e.mem.set_u8(this + offset, 1);
        }
        for offset in [0x5cu32, 0x6c, 0x6d, 0x88, 0x89, 0x8a, 0x94, 0xac, 0xad] {
            e.mem.set_u8(this + offset, 1);
        }
        e.mem.set_u32(this + 0x30, 1);
        e.mem.set_f32(this + 0x8c, 2.5);
        e.mem.set_u32(property + 0x38, 9);
        e.call(0x00ba_f640, &args![this, property, geometry]);
        assert_eq!(e.mem.u32(property + 0x20), 0xbe3b_9dcb);
        assert_eq!(e.mem.u32(property + 0x24), 0x8009);
        assert_eq!(e.mem.u32(property + 0x38), 0);
        assert_eq!(e.mem.f32(property + 0x30), 2.5);
        // Kind 2 swaps bit 0x400 for 0x40000; no skin instance clears bit 0x2.
        e.mem.set_u32(this, 2);
        e.mem.set_u32(geometry + 0xbc, 0);
        e.call(0x00ba_f640, &args![this, property, geometry]);
        assert_eq!(e.mem.u32(property + 0x20), (0xbe3b_9dcb & !0x402) | 0x40000);
        // Same data again: nothing changes, so the pass state stays.
        e.mem.set_u32(property + 0x38, 9);
        e.call(0x00ba_f640, &args![this, property, geometry]);
        assert_eq!(e.mem.u32(property + 0x38), 9);
        // Everything off.
        e.mem.set_u32(this, 0);
        for offset in 0x4du32..=0xad {
            e.mem.set_u8(this + offset, 0);
        }
        e.mem.set_u32(this + 0x30, 0);
        e.call(0x00ba_f640, &args![this, property, geometry]);
        assert_eq!(e.mem.u32(property + 0x20), 0);
        assert_eq!(e.mem.u32(property + 0x24), 0);
        assert_eq!(e.mem.u32(property + 0x38), 0);
    }

    #[test]
    fn sky_texture_name_and_clamp_mode() {
        let mut e = Engine::new();
        quiet(&mut e, &[STRING_SET, 0x0000_6668]);
        e.register(IMPORT_INTERLOCKED_DECREMENT, |_, _| Ret::default());
        e.register(TEXTURE_NAME_00A5B690, |_, _| Ret {
            eax: 0x0077_0008,
            ..Ret::default()
        });
        let mut answers = vec![0u32, 0x5555];
        e.register_double(0x0000_6660, move |_, _| Ret {
            eax: answers.pop().unwrap(),
            ..Ret::default()
        });
        e.map(EMPTY_FIXED_STRING, 4);
        e.set_global(EMPTY_FIXED_STRING, 0x0099_0000u32);
        let set_vtable = e.mem.alloc(0x90);
        e.mem.set_u32(set_vtable + 0x8c, 0x0000_6660);
        let set = e.mem.alloc(4);
        e.mem.set_u32(set, set_vtable);
        let property_vtable = e.mem.alloc(0xd4);
        e.mem.set_u32(property_vtable + 0xd0, 0x0000_6668);
        let property = e.mem.alloc(0x90);
        e.mem.set_u32(property, property_vtable);
        let this = e.mem.alloc(0xb8);
        e.mem.set_u32(this + 0xb4, 2);
        e.call_log = Some(vec![]);
        e.call(0x00ba_fd50, &args![this, property]);
        assert!(calls(&e, STRING_SET).is_empty());
        e.mem.set_u32(this + 0xa8, set);
        e.call(0x00ba_fd50, &args![this, property]);
        assert_eq!(calls(&e, TEXTURE_NAME_00A5B690), vec![vec![0x5555]]);
        assert_eq!(calls(&e, 0x0000_6660), vec![vec![set, 0]]);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![0x0077_0000]]
        );
        assert_eq!(calls(&e, 0x0000_6668), vec![vec![property, 2]]);
        // No texture: the empty string, not released.
        e.call_log = Some(vec![]);
        e.call(0x00ba_fd50, &args![this, property]);
        assert_eq!(
            calls(&e, STRING_SET),
            vec![vec![property + 0x74, 0x0099_0000, 0]]
        );
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn add_shader_property_runs_the_setups_by_type() {
        for (kind_type, setups) in [
            (8u32, vec![SETUP_00BAFA80, SETUP_00BAF520]),
            (12, vec![SETUP_00BAFA80, SETUP_00BAF520]),
            (7, vec![SETUP_00BAFA80, SETUP_00BAF520]),
            (0x15, vec![SETUP_00BAFAF0]),
            (4, vec![SETUP_00BAFCB0]),
            (0x13, vec![SETUP_00BAFC00]),
            (0x16, vec![]),
        ] {
            let mut e = Engine::new();
            quiet(
                &mut e,
                &[
                    SETUP_00BAFA80,
                    SETUP_00BAF520,
                    SETUP_00BAFAF0,
                    SETUP_00BAFCB0,
                    SETUP_00BAFC00,
                    ATTACH_PROPERTY,
                    0x0000_5550,
                ],
            );
            let property = e.mem.alloc(0x90);
            e.mem.set_u32(property + 0x1c, kind_type);
            e.register_double(NI_ALLOCATE, move |_, _| Ret {
                eax: property,
                ..Ret::default()
            });
            constructing(&mut e);
            e.map(SHADER_BY_KIND, 8);
            e.set_global(SHADER_BY_KIND + 4, 0x11u32);
            let vtable = e.mem.alloc(0x1c);
            e.mem.set_u32(vtable + 0x18, 0x0000_5550);
            let geometry = e.mem.alloc(0xc0);
            e.mem.set_u32(geometry, vtable);
            let this = e.mem.alloc(0xb8);
            e.mem.set_u32(this, 1);
            e.mem.set_f32(this + 0x78, 0.5);
            e.mem.set_f32(this + 0x84, 3.0);
            e.call_log = Some(vec![]);
            e.call(0x00bb_0040, &args![this, geometry]);
            let ran: Vec<u32> = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .map(|(a, _)| *a)
                .filter(|a| {
                    [
                        SETUP_00BAFA80,
                        SETUP_00BAF520,
                        SETUP_00BAFAF0,
                        SETUP_00BAFCB0,
                        SETUP_00BAFC00,
                    ]
                    .contains(a)
                })
                .collect();
            assert_eq!(ran, setups);
            for setup in setups {
                assert_eq!(calls(&e, setup), vec![vec![this, property]]);
            }
            assert_eq!(e.mem.u32(property + 0x58), 0x11);
            assert_eq!(calls(&e, ATTACH_PROPERTY), vec![vec![geometry, property]]);
            if kind_type == 0x16 {
                assert_eq!(e.mem.f32(property + 0x60), 0.5);
                assert_eq!(e.mem.f32(property + 0x6c), 3.0);
            } else {
                assert_eq!(e.mem.f32(property + 0x60), 0.0);
            }
        }
    }

    #[test]
    fn add_shader_property_for_the_sky_and_without_a_property() {
        let mut e = Engine::new();
        quiet(&mut e, &[ATTACH_PROPERTY, 0x0000_5550]);
        let property = e.mem.alloc(0x90);
        e.mem.set_u32(property + 0x1c, 0xd);
        e.register_double(NI_ALLOCATE, move |_, _| Ret {
            eax: property,
            ..Ret::default()
        });
        constructing(&mut e);
        e.map(SHADER_BY_KIND, 4);
        let vtable = e.mem.alloc(0x1c);
        e.mem.set_u32(vtable + 0x18, 0x0000_5550);
        let geometry = e.mem.alloc(0xc0);
        e.mem.set_u32(geometry, vtable);
        let this = e.mem.alloc(0xb8);
        e.mem.set_u32(this + 0x90, 4);
        e.call(0x00bb_0040, &args![this, geometry]);
        assert_eq!(e.mem.u32(property + 0x8c), 4);
        // Kind 10 makes no property: nothing else runs.
        e.mem.set_u32(this, 10);
        e.call_log = Some(vec![]);
        e.call(0x00bb_0040, &args![this, geometry]);
        assert!(calls(&e, ATTACH_PROPERTY).is_empty());
    }

    #[test]
    fn convert_adds_the_properties_the_flags_ask_for() {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[ADD_MATERIAL_PROPERTY, ADD_ALPHA_PROPERTY, NI_ALLOCATE],
        );
        let this = e.mem.alloc(0xb8);
        e.call_log = Some(vec![]);
        e.call(0x00bb_0180, &args![this, 0x40u32, 6u32]);
        assert_eq!(calls(&e, ADD_MATERIAL_PROPERTY), vec![vec![this, 0x40]]);
        assert_eq!(calls(&e, ADD_ALPHA_PROPERTY), vec![vec![this, 0x40]]);
        assert!(calls(&e, NI_ALLOCATE).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x00bb_0180, &args![this, 0x40u32, 0x101u32]);
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x104]]);
        assert!(calls(&e, ADD_MATERIAL_PROPERTY).is_empty());
        assert!(calls(&e, ADD_ALPHA_PROPERTY).is_empty());
    }
}

#[cfg(test)]
mod n012_tests {
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

    fn rig() -> Engine {
        let mut e = Engine::new();
        e.map(0x0109_b000, 0x1000);
        e.mem.set_u32(EMPTY_FIXED_STRING, 0x0e0e);
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f32(UNSET_01012054, -1.0);
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let v = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], v);
            v.into_ret()
        });
        e.register(NI_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        e.call_log = Some(vec![]);
        e
    }

    /// An object whose vtable has doubles at the slots given (recorded in the call log).
    fn object(e: &mut Engine, slots: &[u32]) -> u32 {
        let mut table = [0u32; 0x40];
        for slot in slots {
            table[*slot as usize / 4] = 0x0f00_0000 + slot;
            e.register(0x0f00_0000 + slot, |_, _| Ret::default());
        }
        let vtable = e.mem.alloc(0x100);
        for (i, w) in table.iter().enumerate() {
            e.mem.set_u32(vtable + 4 * i as u32, *w);
        }
        let o = e.mem.alloc(0x110);
        e.mem.set_u32(o, vtable);
        o
    }

    #[test]
    fn fn_00baf520_sets_the_lighting_values() {
        let mut e = rig();
        e.register(SET_VALUE_00B66700, |_, _| Ret::default());
        let this = e.mem.alloc(0xb8);
        e.mem.set_u32(this + 0x58, 0x7f80_0001);
        e.mem.set_u32(this + 0x60, 6);
        e.mem.set_f32(this + 0x70, 0.5);
        e.mem.set_f32(this + 0x74, 0.25);
        let controller = object(&mut e, &[0x98]);
        e.mem.set_u32(this + 0x68, controller);
        let property = e.mem.alloc(0x104);
        e.call(0x00ba_f520, &args![this, property]);
        assert_eq!(
            calls(&e, SET_VALUE_00B66700),
            vec![vec![property, 0x7fc0_0001]]
        );
        assert_eq!(e.mem.u32(property + 0xe4), 6);
        assert_eq!(e.mem.f32(property + 0xe8), 0.25);
        assert_eq!(e.mem.f32(property + 0xec), 0.5);
        assert_eq!(calls(&e, 0x0f00_0098), vec![vec![controller, property]]);
    }

    #[test]
    fn fn_00bafa80_assigns_the_texture_set_and_clamp() {
        let mut e = rig();
        let this = e.mem.alloc(0xb8);
        let set = e.mem.alloc(0x10);
        e.mem.set_u32(this + 0xa8, set);
        e.mem.set_u32(this + 0xb4, 2);
        let property = object(&mut e, &[0xe4]);
        e.call(0x00ba_fa80, &args![this, property]);
        assert_eq!(e.mem.u32(property + 0xa4), set);
        assert_eq!(e.mem.u32(set + 4), 1);
        assert_eq!(calls(&e, 0x0f00_00e4), vec![vec![property, 2]]);
        e.call(0x00ba_fa80, &args![this, property]);
        assert_eq!(e.mem.u32(set + 4), 1);
    }

    #[test]
    fn file_name_setups_use_the_first_texture_name() {
        let mut e = rig();
        e.register(STRING_SET, |_, _| Ret::default());
        e.register(TEXTURE_NAME_00A5B690, |_, a| (a[0] + 8).into_ret());
        let this = e.mem.alloc(0xb8);
        e.mem.set_u32(this + 0xb4, 3);
        let property = object(&mut e, &[0xd0]);
        for k in 0..4u32 {
            e.mem
                .set_f32(property + 0x70 + 4 * k, if k == 1 { -1.0 } else { 0.0 });
            e.mem.set_f32(this + 0x98 + 4 * k, (k + 1) as f32);
        }
        // No texture set: only the falloff for 00bafaf0, nothing for the others.
        e.call(0x00ba_faf0, &args![this, property]);
        e.call(0x00ba_fc00, &args![this, property]);
        e.call(0x00ba_fcb0, &args![this, property]);
        assert!(calls(&e, STRING_SET).is_empty());
        assert_eq!(e.mem.f32(property + 0x70), 1.0);
        assert_eq!(e.mem.f32(property + 0x74), -1.0);
        assert_eq!(e.mem.f32(property + 0x7c), 4.0);
        let set = e.mem.alloc(0x10);
        let mut table = [0u32; 0x24];
        table[0x8c / 4] = 0x0f00_108c;
        e.put_vtable(0x0f00_1000, &table);
        e.register_double(0x0f00_108c, |e, a| {
            assert_eq!(a[1], 0);
            e.mem.u32(a[0] + 8).into_ret()
        });
        e.mem.set_u32(set, 0x0f00_1000);
        e.mem.set_u32(this + 0xa8, set);
        let counted = e.mem.alloc(0x10);
        e.mem.set_u32(counted, 5);
        e.mem.set_u32(set + 8, counted);
        e.call(0x00ba_faf0, &args![this, property]);
        e.call(0x00ba_fc00, &args![this, property]);
        e.mem.set_u32(set + 8, 0);
        e.call(0x00ba_fcb0, &args![this, property]);
        assert_eq!(
            calls(&e, STRING_SET),
            vec![
                vec![property + 0x64, counted + 8, 0],
                vec![property + 0x94, counted + 8, 0],
                vec![property + 0x88, 0x0e0e, 0]
            ]
        );
        assert_eq!(e.mem.u32(counted), 3);
        assert_eq!(calls(&e, 0x0f00_00d0), vec![vec![property, 3]]);
        assert_eq!(e.mem.u32(property + 0x8c), 3);
    }

    #[test]
    fn add_material_property_copies_colours_and_attaches() {
        let mut e = rig();
        e.register(MATERIAL_PROPERTY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(ATTACH_PROPERTY, |_, _| Ret::default());
        let this = e.mem.alloc(0xb8);
        for k in 0..9u32 {
            e.mem.set_f32(this + 4 + 4 * k, k as f32);
        }
        let controller = object(&mut e, &[0x98]);
        e.mem.set_u32(this + 0x30, controller);
        e.call(0x00ba_fe10, &args![this, 0x6060u32]);
        let material = calls(&e, ATTACH_PROPERTY)[0][1];
        assert_eq!(calls(&e, NI_ALLOCATE), vec![vec![0x4c]]);
        assert_eq!(e.mem.f32(material + 0x1c), 0.0);
        assert_eq!(e.mem.f32(material + 0x30), 5.0);
        assert_eq!(e.mem.f32(material + 0x38), 6.0);
        assert_eq!(e.mem.f32(material + 0x40), 8.0);
        assert_eq!(e.mem.u32(material + 0x44), 5);
        assert_eq!(calls(&e, 0x0f00_0098), vec![vec![controller, material]]);
        assert_eq!(calls(&e, ATTACH_PROPERTY), vec![vec![0x6060, material]]);
    }

    #[test]
    fn add_alpha_property_packs_the_blend_and_test_flags() {
        let mut e = rig();
        e.register(PROPERTY_BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(ATTACH_PROPERTY, |_, _| Ret::default());
        let this = e.mem.alloc(0xb8);
        e.call(0x00ba_ff20, &args![this, 0x6060u32]);
        assert!(calls(&e, ATTACH_PROPERTY).is_empty());
        e.mem.set_u8(this + 0x38, 1);
        e.mem.set_u16(this + 0x3c, 6);
        e.mem.set_u16(this + 0x40, 7);
        e.mem.set_u8(this + 0x44, 1);
        e.mem.set_u16(this + 0x48, 4);
        e.mem.set_u8(this + 0x4c, 0x80);
        e.call(0x00ba_ff20, &args![this, 0x6060u32]);
        let alpha = calls(&e, ATTACH_PROPERTY)[0][1];
        assert_eq!(e.mem.u32(alpha), ALPHA_PROPERTY_VTABLE);
        assert_eq!(
            e.mem.u16(alpha + 0x18),
            1 | 6 << 1 | 7 << 5 | 0x200 | 4 << 10
        );
        assert_eq!(e.mem.u8(alpha + 0x1a), 0x80);
        // Testing only: blending off; with +0x57 set: nothing.
        e.mem.set_u8(this + 0x38, 0);
        e.call(0x00ba_ff20, &args![this, 0x6060u32]);
        let alpha = calls(&e, ATTACH_PROPERTY)[1][1];
        assert_eq!(e.mem.u16(alpha + 0x18) & 1, 0);
        e.mem.set_u8(this + 0x57, 1);
        e.call(0x00ba_ff20, &args![this, 0x6060u32]);
        assert_eq!(calls(&e, ATTACH_PROPERTY).len(), 2);
    }
}
