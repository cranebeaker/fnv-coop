//! `BSShader/run_00bae560` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Four functions Ghidra never created (research/engine-map/ghidra_gaps.tsv), read from their
//! bytes. `00bae560` is slot 7 (+0x1c) of the HDR image-space effect's vtable `010b8c8c` (the
//! vtable the constructor `00bad500` of `imagespaceeffecthdr.cpp` stores; no RTTI locator),
//! the slot the Xbox PDB's `ImageSpaceEffect` calls `UpdateParams`. The other three are
//! slots of `SkyShaderProperty`'s vtable `010b8ce0` (see `run_00bae980.rs`): slot 27
//! (+0x6c, `PostLinkObject` in the Xbox PDB's order), slot 39 (+0x9c, `SetupGeometry`) and
//! slot 40 (+0xa0, `GetRenderPasses`).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bsshader::run_00bae980::SkyShaderProperty;

/// The byte `012005a1`: when set, the HDR effect hands back the texture at `012005a4`.
const RETURN_TEXTURE_FLAG_012005A1: u32 = 0x0120_05a1;
/// The byte `012005a2`: chooses 1.0 over 0.0 for pixel constant 0x17 of two parameters.
const CONSTANT_FLAG_012005A2: u32 = 0x0120_05a2;
/// The `ImageSpaceTexture` at `012005a4`, and its `GetTexture` (`00ba3780`),
/// `ReturnRenderedTexture` (`00ba39a0`) and `ClearTexture` (`00ba38b0`) (engine map; all
/// `thiscall`, no stack words).
const HDR_TEXTURE_012005A4: u32 = 0x0120_05a4;
const TEXTURE_GET: u32 = 0x00ba_3780;
const TEXTURE_RETURN_RENDERED: u32 = 0x00ba_39a0;
const TEXTURE_CLEAR: u32 = 0x00ba_38b0;
/// The object at `011f91ac` (made by `00b97a80`): its word at +8 is an object whose word at
/// +0xc gives the constants block (its vtable slot +0x20, no stack words).
const OBJECT_011F91AC: u32 = 0x011f_91ac;
/// `ImageSpaceShaderParam::SetPixelConstant` (engine map, `00b8a790`, `thiscall`: index and
/// four floats, `RET 0x14`) and the pixel-constant copy `00b8a8b0` (`thiscall`: first index,
/// source, byte count, `RET 0xc`).
const SET_PIXEL_CONSTANT: u32 = 0x00b8_a790;
const COPY_PIXEL_CONSTANTS: u32 = 0x00b8_a8b0;
/// The `float` `011f9410` the clamped constant is computed from, and the `double` factor
/// `01015a38` (15.0 in the exe) it is multiplied by.
const SCALE_011F9410: u32 = 0x011f_9410;
const FACTOR_01015A38: u32 = 0x0101_5a38;
/// The `double`s the product is clamped to: `01012060` (0.0 in the exe) from below and
/// `01012070` (1.0) from above.
const LOWER_01012060: u32 = 0x0101_2060;
const UPPER_01012070: u32 = 0x0101_2070;
/// The `float` `011add08` (1.0 in the exe) passed as the last float of the second 0x17
/// constant.
const LAST_CONSTANT_011ADD08: u32 = 0x011a_dd08;
/// The bits of `1.0f` (`FLD1`).
const ONE_BITS: u32 = 0x3f80_0000;

/// `NiObjectNET::PostLinkObject` (engine map, `00a5c6a0`, `thiscall`: the stream).
const POST_LINK_OBJECT: u32 = 0x00a5_c6a0;
/// `BSShaderManager::GetTexture` (engine map, `00b55840`, `cdecl`, five words: the file name,
/// 1, the address of the `NiPointer` that receives the texture, 1, 0).
const GET_TEXTURE: u32 = 0x00b5_5840;
/// The byte `011f941e` (a renderer setting the shaders test; when set, sky geometry skips the
/// property update and the extra render pass).
const SETTING_011F941E: u32 = 0x011f_941e;
/// `NiAVObject::UpdateProperties` (engine map, `00a5a040`, `thiscall`, no stack words).
const UPDATE_PROPERTIES: u32 = 0x00a5_a040;
/// `BSShaderProperty::CheckCreateRenderPassArray` (engine map, `00ba95e0`, `thiscall`, one
/// word) and `BSShaderProperty::RenderPassArray::Add` (`00ba9ee0`, `thiscall` on the array:
/// geometry, pass type, then the words 1, 0, 0, 0, 0, 0; `RET 0x20`).
const CHECK_CREATE_RENDER_PASS_ARRAY: u32 = 0x00ba_95e0;
const RENDER_PASS_ARRAY_ADD: u32 = 0x00ba_9ee0;
/// The byte `011ff8c4`: chooses pass 0x23e over 0x23d for sky object type 3.
const PASS_FLAG_011FF8C4: u32 = 0x011f_f8c4;

// Translated from 00bae560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `thiscall`, one stack word that is not read, `RET 4`): the HDR
/// effect's parameter update. Answers false when the effect has no data at +0x6c. Otherwise,
/// when the byte at `012005a1` is set and the texture at `012005a4` has a texture, returns
/// and clears it. Then, with the effect's parameter array at +0x1c (`params[i]` its words):
/// `params[2]` gets constant 1 = (data +0x10, 0, 0, 0) and 0x40 bytes from the constants block
/// of the object at `011f91ac` copied to constants 0x13 onwards; `params[11]` gets constant
/// 0 = (0, 0, v, 0) with `v` the float at `011f9410` times 15.0 clamped to 0..1 (as the x87
/// compares: a NaN stays), and constant 1 = (0, 0, data +0, data +0x14); `params[12]` gets
/// constant 1 = (data +0x14, 0, 0, 0); `params[3]` and `params[2]` get constant 0x17 =
/// (c, 0, 0, 0) and (c, 0, 0, the float at `011add08`), `c` 1.0 when the byte at `012005a2` is
/// set and 0.0 otherwise. Answers true.
pub fn fn_00bae560(e: &mut Engine, this: Ptr, _unused_1: u32) -> bool {
    let base = this.addr();
    if e.mem.u32(base + 0x6c) == 0 {
        return false;
    }
    if e.mem.u8(RETURN_TEXTURE_FLAG_012005A1) != 0
        && e.call(TEXTURE_GET, &args![HDR_TEXTURE_012005A4]).u32() != 0
    {
        e.call(TEXTURE_RETURN_RENDERED, &args![HDR_TEXTURE_012005A4]);
        e.call(TEXTURE_CLEAR, &args![HDR_TEXTURE_012005A4]);
    }
    let object = e.mem.u32(OBJECT_011F91AC);
    let holder = e.mem.u32(object.wrapping_add(8));
    // The effect's data (+0x6c) and parameter array (`pEffectParamList`'s elements, +0x1c).
    let data = e.mem.u32(base + 0x6c);
    let params = e.mem.u32(base + 0x1c);
    let params_2 = e.mem.u32(params.wrapping_add(8));
    let constants = e.mem.u32(holder.wrapping_add(0xc));
    let value = x87_quiet_bits(e.mem.u32(data.wrapping_add(0x10)));
    e.call(
        SET_PIXEL_CONSTANT,
        &args![params_2, 1u32, value, 0u32, 0u32, 0u32],
    );
    let block = e.vcall(constants, 0x20, &args![]).u32();
    e.call(
        COPY_PIXEL_CONSTANTS,
        &args![params_2, 0x13u32, block, 0x40u32],
    );

    let scale = x87_load32(e.mem.u32(SCALE_011F9410));
    let factor: f64 = e.global(FACTOR_01015A38);
    let scaled = x87_store32(x87_mul(scale, factor));
    let params = e.mem.u32(base + 0x1c);
    let params_11 = e.mem.u32(params.wrapping_add(0x2c));
    // FCOM with the lower bound, TEST AH,5 / JP: cleared only when below (ordered).
    let lower: f64 = e.global(LOWER_01012060);
    let low = if x87_load32(scaled) < lower {
        0
    } else {
        x87_store32(x87_load32(scaled))
    };
    // FCOM with the upper bound, TEST AH,0x41 / JNZ: replaced only when above (ordered).
    let upper: f64 = e.global(UPPER_01012070);
    let loaded = x87_load32(low);
    let clamped = if loaded > upper {
        ONE_BITS
    } else {
        x87_store32(loaded)
    };
    e.call(
        SET_PIXEL_CONSTANT,
        &args![params_11, 0u32, 0u32, 0u32, x87_quiet_bits(clamped), 0u32],
    );
    let data = e.mem.u32(base + 0x6c);
    let last = x87_quiet_bits(e.mem.u32(data.wrapping_add(0x14)));
    let first = x87_quiet_bits(e.mem.u32(data));
    e.call(
        SET_PIXEL_CONSTANT,
        &args![params_11, 1u32, 0u32, 0u32, first, last],
    );

    let data = e.mem.u32(base + 0x6c);
    let params = e.mem.u32(base + 0x1c);
    let params_12 = e.mem.u32(params.wrapping_add(0x30));
    let value = x87_quiet_bits(e.mem.u32(data.wrapping_add(0x14)));
    e.call(
        SET_PIXEL_CONSTANT,
        &args![params_12, 1u32, value, 0u32, 0u32, 0u32],
    );

    let choice = if e.mem.u8(CONSTANT_FLAG_012005A2) != 0 {
        ONE_BITS
    } else {
        0
    };
    let params = e.mem.u32(base + 0x1c);
    let params_3 = e.mem.u32(params.wrapping_add(0xc));
    e.call(
        SET_PIXEL_CONSTANT,
        &args![params_3, 0x17u32, choice, 0u32, 0u32, 0u32],
    );
    let last = x87_quiet_bits(e.mem.u32(LAST_CONSTANT_011ADD08));
    let params = e.mem.u32(base + 0x1c);
    let params_2 = e.mem.u32(params.wrapping_add(8));
    e.call(
        SET_PIXEL_CONSTANT,
        &args![params_2, 0x17u32, choice, 0u32, 0u32, last],
    );
    true
}

// Translated from 00bae740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `thiscall`, one stack word, `RET 4`): `SkyShaderProperty`'s
/// slot 27. Runs `NiObjectNET::PostLinkObject` with the stream; then, when the texture file
/// name (`TextureFilename`'s text, +0x74) is not null and `soType` is not 3, loads the texture
/// into `spBaseMap` (+0x70) through `BSShaderManager::GetTexture(name, 1, &spBaseMap, 1, 0)`.
pub fn fn_00bae740(e: &mut Engine, this: Ptr<SkyShaderProperty>, stream: u32) {
    e.call(POST_LINK_OBJECT, &args![this, stream]);
    // SkyShaderProperty::TextureFilename (Xbox PDB) +0x74: the BSStringT's text pointer.
    let name = e.mem.u32(this.addr() + 0x74);
    if name != 0 && e.get(this, SkyShaderProperty::soType) != 3 {
        e.call(
            GET_TEXTURE,
            &args![name, 1u32, this.addr() + 0x70, 1u32, 0u32],
        );
    }
}

// Translated from 00bae780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `thiscall`, one stack word, `RET 4`; `this` is not read):
/// `SkyShaderProperty`'s slot 39. When the byte at `011f941e` is set and `geometry + 0x9c` is
/// the null address (the exe tests the field's address, so only for `geometry` =
/// `0xffffff64`), runs `NiAVObject::UpdateProperties` on `geometry`. Answers true.
pub fn fn_00bae780(e: &mut Engine, _unused_this: Ptr, geometry: u32) -> bool {
    if e.mem.u8(SETTING_011F941E) != 0 && geometry.wrapping_add(0x9c) == 0 {
        e.call(UPDATE_PROPERTIES, &args![geometry]);
    }
    true
}

// Translated from 00bae7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; `thiscall`, six stack words of which only the first,
/// `geometry`, is read, `RET 0x18`): `SkyShaderProperty`'s slot 40. Runs
/// `CheckCreateRenderPassArray(2)`; when the render pass array (+0x3c) already holds passes
/// (its word at +0x10 is not 0) answers it. Otherwise adds one pass for `geometry` chosen by
/// `soType` (the exe's jump table on `soType - 1`): 1 gives 0x2d2, 3 gives 0x23e when the byte
/// at `011ff8c4` is set and 0x23d otherwise, 5 gives 0x23b, 7 gives 0x23a, 2 and 4 give 0x239,
/// every other value 0x23c. Then, unless the byte at `011f941e` is set, `soType` 0, 3 and 4 add
/// a second pass: 0x2d3, 0x2d4 and 0x2d5. Answers the render pass array (+0x3c, read again).
pub fn fn_00bae7b0(
    e: &mut Engine,
    this: Ptr<SkyShaderProperty>,
    geometry: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
    _unused_6: u32,
) -> u32 {
    e.call(CHECK_CREATE_RENDER_PASS_ARRAY, &args![this, 2u32]);
    // BSShaderProperty's render pass array pointer (+0x3c on PC).
    let passes = e.mem.u32(this.addr() + 0x3c);
    if e.mem.u32(passes.wrapping_add(0x10)) != 0 {
        return passes;
    }
    let sky_type = e.get(this, SkyShaderProperty::soType);
    let pass = match sky_type.wrapping_sub(1) {
        0 => 0x2d2,
        2 => {
            if e.mem.u8(PASS_FLAG_011FF8C4) != 0 {
                0x23e
            } else {
                0x23d
            }
        }
        4 => 0x23b,
        6 => 0x23a,
        // Cases 1, 3 and 5 of the table and every value above 6 share the default.
        _ => {
            if sky_type == 2 || sky_type == 4 {
                0x239
            } else {
                0x23c
            }
        }
    };
    add_pass(e, passes, geometry, pass);
    if e.mem.u8(SETTING_011F941E) == 0 {
        let sky_type = e.get(this, SkyShaderProperty::soType);
        let second = match sky_type {
            0 => Some(0x2d3),
            3 => Some(0x2d4),
            4 => Some(0x2d5),
            _ => None,
        };
        if let Some(second) = second {
            let passes = e.mem.u32(this.addr() + 0x3c);
            add_pass(e, passes, geometry, second);
        }
    }
    e.mem.u32(this.addr() + 0x3c)
}

/// `RenderPassArray::Add(geometry, pass, 1, 0, 0, 0, 0, 0)` on `passes`.
fn add_pass(e: &mut Engine, passes: u32, geometry: u32, pass: u32) {
    e.call(
        RENDER_PASS_ARRAY_ADD,
        &args![passes, geometry, pass, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
    );
}

/// Where `fn_00bae710` jumps (`00c9a420`, `thiscall`, one word: the other object).
const IS_EQUAL_00C9A420: u32 = 0x00c9_a420;

// Translated from 00bae710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: slot +0x5c of `SkyShaderProperty`'s vtable `010b8ce0`
/// (`IsEqual` in the Xbox PDB's slot list): a jump to `00c9a420` with `this` and its one word;
/// answers the target's `EAX`.
pub fn fn_00bae710(e: &mut Engine, this: Ptr, other: u32) -> u32 {
    e.call(IS_EQUAL_00C9A420, &args![this, other]).u32()
}

// Translated from 00bae720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Ghidra made no function here: slot +0xc8 of `SkyShaderProperty`'s vtable `010b8ce0`
/// (`SetClampD3D` in the Xbox PDB's slot list; `thiscall`, one word). Stores 0 in
/// `TextureClampMode` when the word is 3, otherwise 3.
pub fn fn_00bae720(e: &mut Engine, this: Ptr<SkyShaderProperty>, mode: u32) {
    let value = if mode.wrapping_sub(3) != 0 { 3 } else { 0 };
    e.set(this, SkyShaderProperty::TextureClampMode, value);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_e710, fn_00bae710(Ptr, u32) -> u32),
        entry!(0x00ba_e720, fn_00bae720(Ptr<SkyShaderProperty>, u32)),
        entry!(0x00ba_e560, fn_00bae560(Ptr, u32) -> bool),
        entry!(0x00ba_e740, fn_00bae740(Ptr<SkyShaderProperty>, u32)),
        entry!(0x00ba_e780, fn_00bae780(Ptr, u32) -> bool),
        entry!(
            0x00ba_e7b0,
            fn_00bae7b0(Ptr<SkyShaderProperty>, u32, u32, u32, u32, u32, u32) -> u32
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    const CONSTANTS_SLOT: u32 = 0x0e50_0000;
    const BLOCK: u32 = 0x0e51_0000;

    /// The HDR effect with its data and parameters, the globals it reads and the doubles of
    /// its callees; answers the engine, the effect and its parameters 2, 3, 11 and 12.
    fn hdr_engine(scale: f32) -> (Engine, u32, [u32; 4]) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x4000);
        e.map(0x011a_d000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.set_global(FACTOR_01015A38, 15.0f64);
        e.set_global(LOWER_01012060, 0.0f64);
        e.set_global(UPPER_01012070, 1.0f64);
        e.set_global(LAST_CONSTANT_011ADD08, 0x3f80_0000u32);
        e.set_global(SCALE_011F9410, scale);
        let constants_vtable = e.mem.alloc(0x40);
        e.mem.set_u32(constants_vtable + 0x20, CONSTANTS_SLOT);
        let constants = e.mem.alloc(0x10);
        e.mem.set_u32(constants, constants_vtable);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(holder + 0xc, constants);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 8, holder);
        e.set_global(OBJECT_011F91AC, object);
        let this = e.mem.alloc(0x70);
        let data = e.mem.alloc(0x18);
        e.mem.set_u32(data, 0x4000_0000);
        e.mem.set_u32(data + 0x10, 0x4040_0000);
        // A signalling NaN: the FLD/FSTP copy quiets it.
        e.mem.set_u32(data + 0x14, 0x7f80_0001);
        e.mem.set_u32(this + 0x6c, data);
        let params = e.mem.alloc(0x40);
        let mut chosen = [0u32; 4];
        for (slot, index) in [2u32, 3, 11, 12].iter().enumerate() {
            chosen[slot] = 0x0e60_0000 + index * 0x100;
            e.mem.set_u32(params + index * 4, chosen[slot]);
        }
        e.mem.set_u32(this + 0x1c, params);
        e.register(SET_PIXEL_CONSTANT, |_, _| Ret::default());
        e.register(COPY_PIXEL_CONSTANTS, |_, _| Ret::default());
        e.register(CONSTANTS_SLOT, |_, _| BLOCK.into_ret());
        e.register(TEXTURE_GET, |_, _| 1u32.into_ret());
        e.register(TEXTURE_RETURN_RENDERED, |_, _| Ret::default());
        e.register(TEXTURE_CLEAR, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        (e, this, chosen)
    }

    #[test]
    fn fn_00bae560_sets_the_hdr_constants() {
        let (mut e, this, [p2, p3, p11, p12]) = hdr_engine(0.05);
        e.set_global(CONSTANT_FLAG_012005A2, 1u8);
        assert!(e.call(0x00ba_e560, &args![this, 0u32]).bool());
        assert!(calls_to(&e, TEXTURE_GET).is_empty());
        let scaled = x87_store32(x87_mul(f64::from(0.05f32), 15.0));
        assert_eq!(
            calls_to(&e, SET_PIXEL_CONSTANT),
            vec![
                vec![p2, 1, 0x4040_0000, 0, 0, 0],
                vec![p11, 0, 0, 0, scaled, 0],
                vec![p11, 1, 0, 0, 0x4000_0000, 0x7fc0_0001],
                vec![p12, 1, 0x7fc0_0001, 0, 0, 0],
                vec![p3, 0x17, ONE_BITS, 0, 0, 0],
                vec![p2, 0x17, ONE_BITS, 0, 0, 0x3f80_0000],
            ]
        );
        assert_eq!(
            calls_to(&e, COPY_PIXEL_CONSTANTS),
            vec![vec![p2, 0x13, BLOCK, 0x40]]
        );
    }

    #[test]
    fn fn_00bae560_clamps_the_scaled_value_and_returns_the_texture() {
        for (scale, expected) in [(-1.0f32, 0u32), (2.0, ONE_BITS), (f32::NAN, 0x7fc0_0000)] {
            let (mut e, this, [_, p3, p11, _]) = hdr_engine(scale);
            e.set_global(RETURN_TEXTURE_FLAG_012005A1, 1u8);
            assert!(e.call(0x00ba_e560, &args![this, 0u32]).bool());
            let constants = calls_to(&e, SET_PIXEL_CONSTANT);
            assert_eq!(constants[1], vec![p11, 0, 0, 0, expected, 0], "{scale}");
            assert_eq!(constants[4], vec![p3, 0x17, 0, 0, 0, 0]);
            assert_eq!(
                calls_to(&e, TEXTURE_CLEAR),
                vec![vec![HDR_TEXTURE_012005A4]]
            );
        }
        // No texture to hand back: neither return nor clear.
        let (mut e, this, _) = hdr_engine(0.0);
        e.set_global(RETURN_TEXTURE_FLAG_012005A1, 1u8);
        e.register(TEXTURE_GET, |_, _| Ret::default());
        e.call(0x00ba_e560, &args![this, 0u32]);
        assert!(calls_to(&e, TEXTURE_RETURN_RENDERED).is_empty());
        // No data: false and nothing else.
        let (mut e, this, _) = hdr_engine(0.0);
        e.mem.set_u32(this + 0x6c, 0);
        assert!(!e.call(0x00ba_e560, &args![this, 0u32]).bool());
        assert!(calls_to(&e, SET_PIXEL_CONSTANT).is_empty());
    }

    #[test]
    fn fn_00bae740_loads_the_base_texture_unless_clouds() {
        for (name, sky_type, loads) in [
            (0x0e70_0000u32, 0u32, true),
            (0, 0, false),
            (0x0e70_0000, 3, false),
        ] {
            let mut e = Engine::new();
            e.register(POST_LINK_OBJECT, |_, _| Ret::default());
            e.register(GET_TEXTURE, |_, _| Ret::default());
            let this = e.new_object::<SkyShaderProperty>();
            e.mem.set_u32(this.addr() + 0x74, name);
            e.set(this, SkyShaderProperty::soType, sky_type);
            e.call_log = Some(vec![]);
            e.call(0x00ba_e740, &args![this, 0x0e80_0000u32]);
            assert_eq!(
                calls_to(&e, POST_LINK_OBJECT),
                vec![vec![this.addr(), 0x0e80_0000]]
            );
            let expected = if loads {
                vec![vec![name, 1, this.addr() + 0x70, 1, 0]]
            } else {
                vec![]
            };
            assert_eq!(calls_to(&e, GET_TEXTURE), expected);
        }
    }

    #[test]
    fn fn_00bae780_updates_only_at_the_wrapping_address() {
        for (setting, geometry, updates) in [
            (1u8, 0xffff_ff64u32, true),
            (0, 0xffff_ff64, false),
            (1, 0x0e00_0000, false),
        ] {
            let mut e = Engine::new();
            e.map(0x011f_9000, 0x1000);
            e.set_global(SETTING_011F941E, setting);
            e.register(UPDATE_PROPERTIES, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            assert!(e.call(0x00ba_e780, &args![0u32, geometry]).bool());
            let expected = if updates {
                vec![vec![geometry]]
            } else {
                vec![]
            };
            assert_eq!(calls_to(&e, UPDATE_PROPERTIES), expected);
        }
    }

    #[test]
    fn fn_00bae7b0_adds_the_passes_of_each_sky_type() {
        // (soType, byte 011ff8c4, byte 011f941e, the passes added)
        let cases: [(u32, u8, u8, &[u32]); 13] = [
            (0, 0, 0, &[0x23c, 0x2d3]),
            (1, 0, 0, &[0x2d2]),
            (2, 0, 0, &[0x239]),
            (3, 0, 0, &[0x23d, 0x2d4]),
            (3, 1, 0, &[0x23e, 0x2d4]),
            (3, 0, 1, &[0x23d]),
            (4, 0, 0, &[0x239, 0x2d5]),
            (4, 0, 1, &[0x239]),
            (5, 0, 0, &[0x23b]),
            (6, 0, 0, &[0x23c]),
            (7, 0, 0, &[0x23a]),
            (8, 0, 0, &[0x23c]),
            (0, 0, 1, &[0x23c]),
        ];
        for (sky_type, pass_flag, setting, expected) in cases {
            let mut e = Engine::new();
            e.map(0x011f_9000, 0x7000);
            e.set_global(PASS_FLAG_011FF8C4, pass_flag);
            e.set_global(SETTING_011F941E, setting);
            e.register(CHECK_CREATE_RENDER_PASS_ARRAY, |_, _| Ret::default());
            e.register(RENDER_PASS_ARRAY_ADD, |_, _| Ret::default());
            let this = e.new_object::<SkyShaderProperty>();
            let passes = e.mem.alloc(0x20);
            e.mem.set_u32(this.addr() + 0x3c, passes);
            e.set(this, SkyShaderProperty::soType, sky_type);
            e.call_log = Some(vec![]);
            let result = e
                .call(
                    0x00ba_e7b0,
                    &args![this, 0x0e90_0000u32, 9u32, 9u32, 9u32, 9u32, 9u32],
                )
                .u32();
            assert_eq!(result, passes);
            assert_eq!(
                calls_to(&e, CHECK_CREATE_RENDER_PASS_ARRAY),
                vec![vec![this.addr(), 2]]
            );
            let added: Vec<Vec<u32>> = expected
                .iter()
                .map(|pass| vec![passes, 0x0e90_0000, *pass, 1, 0, 0, 0, 0, 0])
                .collect();
            assert_eq!(calls_to(&e, RENDER_PASS_ARRAY_ADD), added, "{sky_type}");
        }
    }

    #[test]
    fn fn_00bae7b0_answers_a_filled_array_as_it_is() {
        let mut e = Engine::new();
        e.register(CHECK_CREATE_RENDER_PASS_ARRAY, |_, _| Ret::default());
        e.register(RENDER_PASS_ARRAY_ADD, |_, _| Ret::default());
        let this = e.new_object::<SkyShaderProperty>();
        let passes = e.mem.alloc(0x20);
        e.mem.set_u32(passes + 0x10, 1);
        e.mem.set_u32(this.addr() + 0x3c, passes);
        e.call_log = Some(vec![]);
        let result = e
            .call(
                0x00ba_e7b0,
                &args![this, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32],
            )
            .u32();
        assert_eq!(result, passes);
        assert!(calls_to(&e, RENDER_PASS_ARRAY_ADD).is_empty());
    }
}

#[cfg(test)]
mod n013_tests {
    use super::*;

    #[test]
    fn fn_00bae710_jumps_on() {
        let mut e = Engine::new();
        e.register(IS_EQUAL_00C9A420, |_, a| (a[0] ^ a[1]).into_ret());
        assert_eq!(e.call(0x00ba_e710, &args![0x10u32, 0x11u32]).u32(), 1);
    }

    #[test]
    fn fn_00bae720_maps_3_to_0_and_the_rest_to_3() {
        let mut e = Engine::new();
        let this = e.new_object::<SkyShaderProperty>();
        e.call(0x00ba_e720, &args![this, 3u32]);
        assert_eq!(e.get(this, SkyShaderProperty::TextureClampMode), 0);
        e.call(0x00ba_e720, &args![this, 0u32]);
        assert_eq!(e.get(this, SkyShaderProperty::TextureClampMode), 3);
    }
}
