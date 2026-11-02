//! `BSShader/imagespaceeffecthdr.cpp` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
/// The `NiTArray` set-size `00a90180` (`thiscall` on the array at +0x28, one word).
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
/// The HDR effect's vtable (`010b8c8c`; no RTTI locator before it) and the address it keeps at
/// +0x6c (`011add0c`).
const VTABLE_010B8C8C: u32 = 0x010b_8c8c;
const DATA_011ADD0C: u32 = 0x011a_dd0c;

// Translated from 00bad500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespaceeffecthdr.cpp`): the HDR image-space effect's
/// constructor, `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the
/// vtable `010b8c8c` and the address `011add0c` at +0x6c, and sizes the array at +0x28 to 3
/// (`00a90180`). Answers `this`.
// Translation incomplete: 00bad500 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00bad500(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B8C8C);
    e.mem.set_u32(base + 0x6c, DATA_011ADD0C);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 3u32]);
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{MEMORY_ALLOCATE, MEMORY_MANAGER};

/// The array setter `00ba3aa0` (unnamed; `thiscall` on an `NiTPrimitiveArray`: index, the
/// address of the word), `ImageSpaceEffect::~ImageSpaceEffect` (engine map, `00ba3d30`) and
/// `MemoryManager::Deallocate` (`00aa4060`, `thiscall` on the manager, one word).
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const EFFECT_DESTRUCT_00BA3D30: u32 = 0x00ba_3d30;
const MEMORY_DEALLOCATE_00AA4060: u32 = 0x00aa_4060;

/// Element `index` of the `NiTPrimitiveArray` at `array` set to `value` through `00ba3aa0`
/// (the word in a local).
fn set_at(e: &mut Engine, array: u32, index: u32, value: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), value);
        e.call(ARRAY_SET_AT_00BA3AA0, &args![array, index, local.addr()]);
    });
}

// Translated from 00bad400 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bad400 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
/// Unnamed in the engine map (unit `imagespaceeffecthdr.cpp`): the HDR effect's destructor,
/// `thiscall`, no stack words. Stores the vtable `010b8c8c`; each of the 13 effect parameters
/// (`pEffectParamList`, elements at +0x1c) that is set is deleted through its slot +0 with 1
/// and its element set to null; then `ImageSpaceEffect::~ImageSpaceEffect`.
pub fn fn_00bad400(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010B8C8C);
    for index in 0..0xdu32 {
        let element = e.mem.u32(base + 0x1c).wrapping_add(4 * index);
        if e.mem.u32(element) != 0 {
            let parameter = e.mem.u32(element);
            // The compiler tests the element again after the comparison above.
            if parameter != 0 {
                e.vcall(parameter, 0, &args![1u32]);
            }
            set_at(e, base + 0x18, index, 0);
        }
    }
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

/// The HDR effect's static adapted-light texture (an `ImageSpaceTexture` at `012005a4`) and
/// its `GetTexture` (`00ba3780`), `ReturnRenderedTexture` (`00ba39a0`) and `ClearTexture`
/// (`00ba38b0`) (unit `imagespacetexture.cpp`).
const ADAPTED_LIGHT_012005A4: u32 = 0x0120_05a4;
const TEXTURE_GET_TEXTURE_00BA3780: u32 = 0x00ba_3780;
const TEXTURE_RETURN_RENDERED_00BA39A0: u32 = 0x00ba_39a0;
const TEXTURE_CLEAR_00BA38B0: u32 = 0x00ba_38b0;

// Translated from 00bad4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ImageSpaceEffectHDR::ClearAdaptedLight` (Xbox PDB), no arguments: when the texture at
/// `012005a4` has a texture, it is returned to the texture manager (`ReturnRenderedTexture`)
/// and cleared (`ClearTexture`, a tail call).
pub fn image_space_effect_hdr_clear_adapted_light(e: &mut Engine) {
    let texture = e
        .call(TEXTURE_GET_TEXTURE_00BA3780, &args![ADAPTED_LIGHT_012005A4])
        .u32();
    if texture != 0 {
        e.call(
            TEXTURE_RETURN_RENDERED_00BA39A0,
            &args![ADAPTED_LIGHT_012005A4],
        );
        e.call(TEXTURE_CLEAR_00BA38B0, &args![ADAPTED_LIGHT_012005A4]);
    }
}

/// The byte `011f941e` the HDR effect's activity needs.
const HDR_ENABLED_011F941E: u32 = 0x011f_941e;

// Translated from 00bad4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespaceeffecthdr.cpp`; Ghidra made no function of it):
/// the HDR effect's `IsActive` slot, `thiscall`, no stack words: 1 when the byte `011f941e`
/// is set and `pfData` (+0x6c) is not null, else 0.
pub fn fn_00bad4d0(e: &mut Engine, this: Ptr) -> u32 {
    u32::from(e.global::<u8>(HDR_ENABLED_011F941E) != 0 && e.mem.u32(this.addr() + 0x6c) != 0)
}

// Translated from 00bad4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespaceeffecthdr.cpp`; Ghidra made no function of it),
/// `thiscall`, one word: sets `bParamsChanged` (+5) and keeps the word as `pfData` (+0x6c).
pub fn fn_00bad4f0(e: &mut Engine, this: Ptr, data: u32) {
    e.mem.set_u8(this.addr() + 5, 1);
    e.mem.set_u32(this.addr() + 0x6c, data);
}

// Translated from 00bad560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (unit `imagespaceeffecthdr.cpp`): the HDR effect's scalar
/// deleting destructor, `thiscall`, one word: the destructor `00bad400`, then the memory freed
/// (`MemoryManager::Deallocate`) when bit 0 of `flags` is set. Answers `this`.
pub fn fn_00bad560(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bad400(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The effect-array sizer `00ba3f90` (unnamed; `thiscall` on the effect, one word: the four
/// arrays at +8 to +0x38 sized through `00a90180`), the renderer (`011f4748`), the bytes
/// `011f941f` and `011f9420`, `ImageSpaceShaderParam`'s constructor (`00b8a9e0`, `thiscall`),
/// `InitConstantMap` (`00b8ab10`, `thiscall`, two words), `00b8a7c0` (unnamed, `thiscall`, no
/// stack words) and `00b8a8b0` (unnamed, `thiscall`, three words) (engine map), and the effect
/// setter `00ba3c70` (unit `run_00ba3aa0`, `thiscall`: index, effect, parameter, output).
const SIZE_ARRAYS_00BA3F90: u32 = 0x00ba_3f90;
const RENDERER_011F4748: u32 = 0x011f_4748;
const FLAG_011F941F: u32 = 0x011f_941f;
const FLAG_011F9420: u32 = 0x011f_9420;
const PARAM_CONSTRUCT_00B8A9E0: u32 = 0x00b8_a9e0;
const PARAM_INIT_CONSTANT_MAP_00B8AB10: u32 = 0x00b8_ab10;
const PARAM_SET_UP_00B8A7C0: u32 = 0x00b8_a7c0;
const PARAM_CONSTANTS_00B8A8B0: u32 = 0x00b8_a8b0;
const SET_EFFECT_00BA3C70: u32 = 0x00ba_3c70;
/// The constant tables `00b8a8b0` is given: `011add48` (0x40 bytes), `011add88` (0x100) and
/// `011ade88` (0x90).
const TABLE_011ADD48: u32 = 0x011a_dd48;
const TABLE_011ADD88: u32 = 0x011a_dd88;
const TABLE_011ADE88: u32 = 0x011a_de88;

/// What one parameter of `00bad580` gets: the second word of `InitConstantMap`, the bytes
/// +0x55 and +0x56 cleared, whether `00b8a7c0` runs, the constants (`None`; `Some(None)` for
/// the pair picked by `011f941f`; `Some(Some(..))` for a fixed table and size), the effect's
/// offset in the manager's list (or the two offsets picked by `011f941f`: set, clear), and
/// whether the parameter is stored before the effect is.
struct HdrParam {
    index: u32,
    map: u32,
    clear_55: bool,
    clear_56: bool,
    set_up: bool,
    constants: Option<Option<(u32, u32)>>,
    effect: (u32, u32),
    param_first: bool,
}

/// The eleven parameters in the exe's order.
const HDR_PARAMS: [HdrParam; 11] = [
    HdrParam {
        index: 7,
        map: 0,
        clear_55: true,
        clear_56: false,
        set_up: true,
        constants: None,
        effect: (0x88, 0x88),
        param_first: true,
    },
    HdrParam {
        index: 5,
        map: 0x18,
        clear_55: true,
        clear_56: false,
        set_up: true,
        constants: Some(None),
        effect: (0x104, 0xfc),
        param_first: false,
    },
    HdrParam {
        index: 0xc,
        map: 0x18,
        clear_55: false,
        clear_56: false,
        set_up: true,
        constants: Some(None),
        effect: (0x108, 0x110),
        param_first: false,
    },
    HdrParam {
        index: 0xb,
        map: 0x18,
        clear_55: false,
        clear_56: true,
        set_up: true,
        constants: Some(None),
        effect: (0x10c, 0x114),
        param_first: false,
    },
    HdrParam {
        index: 6,
        map: 0x18,
        clear_55: true,
        clear_56: false,
        set_up: true,
        constants: Some(Some((TABLE_011ADE88, 0x90))),
        effect: (0x100, 0x100),
        param_first: false,
    },
    HdrParam {
        index: 0xa,
        map: 0x18,
        clear_55: true,
        clear_56: false,
        set_up: true,
        constants: None,
        effect: (0x11c, 0x11c),
        param_first: false,
    },
    HdrParam {
        index: 9,
        map: 0x18,
        clear_55: true,
        clear_56: true,
        set_up: true,
        constants: None,
        effect: (0x118, 0x118),
        param_first: false,
    },
    HdrParam {
        index: 8,
        map: 0x18,
        clear_55: false,
        clear_56: false,
        set_up: true,
        constants: None,
        effect: (0xf8, 0xf8),
        param_first: false,
    },
    HdrParam {
        index: 2,
        map: 0x18,
        clear_55: false,
        clear_56: false,
        set_up: false,
        constants: None,
        effect: (0xe8, 0xe8),
        param_first: false,
    },
    HdrParam {
        index: 3,
        map: 0x18,
        clear_55: false,
        clear_56: false,
        set_up: false,
        constants: None,
        effect: (0xec, 0xec),
        param_first: false,
    },
    HdrParam {
        index: 4,
        map: 0x18,
        clear_55: false,
        clear_56: false,
        set_up: false,
        constants: None,
        effect: (0xf0, 0xf0),
        param_first: false,
    },
];

// Translated from 00bad580 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bad580 - the C++ exception frame (FS:[0] chain, unwind states 0 to 10 for the parameter allocations) is not translated.
/// Unnamed in the engine map (unit `imagespaceeffecthdr.cpp`): the HDR effect's `Setup` slot,
/// `thiscall`, two words (the manager whose +8 holds the effects; a word it does not read).
/// Sizes the effect's arrays to 13 (`00ba3f90`) and `pTextures` to 3; sets
/// `bUseAnisoDownsample` (+0x68) when the bytes `011f941f` and `011f9420` are set and the
/// renderer's slot +0xc8 answers an object whose slot +0x8c (with 0) answers at most 0x500.
/// Then eleven parameters (0x88 bytes from the memory manager, not checked for null:
/// `ImageSpaceShaderParam`'s constructor runs only on a block): `InitConstantMap(0, map)`,
/// bytes +0x55/+0x56 cleared, `00b8a7c0`, the constants (`00b8a8b0(3, table, size)`: with
/// `011f941f` set `011add48`/0x40, else `011add88`/0x100, or a fixed table), and each is
/// stored at its index of `pEffectParamList` (`00ba3aa0`) and given with the manager's effect
/// to `00ba3c70(index, effect, parameter, 0)`, in [`HDR_PARAMS`] order.
pub fn fn_00bad580(e: &mut Engine, this: Ptr, manager: u32, _unused_1: u32) {
    let base = this.addr();
    e.call(SIZE_ARRAYS_00BA3F90, &args![base, 0xdu32]);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 3u32]);
    let mut aniso = 0u8;
    if e.global::<u8>(FLAG_011F941F) != 0 && e.global::<u8>(FLAG_011F9420) != 0 {
        let renderer = e.global::<u32>(RENDERER_011F4748);
        let object = e.vcall(renderer, 0xc8, &args![]).u32();
        if e.vcall(object, 0x8c, &args![0u32]).u32() <= 0x500 {
            aniso = 1;
        }
    }
    e.mem.set_u8(base + 0x68, aniso);
    for spec in &HDR_PARAMS {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x88u32])
            .u32();
        let param = if block != 0 {
            e.call(PARAM_CONSTRUCT_00B8A9E0, &args![block]).u32()
        } else {
            0
        };
        e.call(
            PARAM_INIT_CONSTANT_MAP_00B8AB10,
            &args![param, 0u32, spec.map],
        );
        if spec.clear_55 {
            e.mem.set_u8(param + 0x55, 0);
        }
        if spec.clear_56 {
            e.mem.set_u8(param + 0x56, 0);
        }
        if spec.set_up {
            e.call(PARAM_SET_UP_00B8A7C0, &args![param]);
        }
        let offset = match spec.constants {
            Some(None) => {
                let (table, size, offset) = if e.global::<u8>(FLAG_011F941F) != 0 {
                    (TABLE_011ADD48, 0x40u32, spec.effect.0)
                } else {
                    (TABLE_011ADD88, 0x100, spec.effect.1)
                };
                e.call(PARAM_CONSTANTS_00B8A8B0, &args![param, 3u32, table, size]);
                offset
            }
            Some(Some((table, size))) => {
                e.call(PARAM_CONSTANTS_00B8A8B0, &args![param, 3u32, table, size]);
                spec.effect.0
            }
            None => spec.effect.0,
        };
        if spec.param_first {
            set_at(e, base + 0x18, spec.index, param);
        }
        let effect = e.mem.u32(e.mem.u32(manager + 8) + offset);
        e.call(
            SET_EFFECT_00BA3C70,
            &args![base, spec.index, effect, param, 0u32],
        );
        if !spec.param_first {
            set_at(e, base + 0x18, spec.index, param);
        }
    }
}

/// `ImageSpaceTexture`'s members (unit `imagespacetexture.cpp`, `thiscall` on the texture):
/// `BorrowRenderedTexture` (one word), `GetRenderedTexture`, `SetTexture` (one word), the
/// borrowed-byte setter `00ba3750` (one word), `operator=` (one word, answers `this`) and the
/// destructor.
const TEXTURE_BORROW_00BA3840: u32 = 0x00ba_3840;
const TEXTURE_GET_RENDERED_00BA3770: u32 = 0x00ba_3770;
const TEXTURE_SET_TEXTURE_00BA37A0: u32 = 0x00ba_37a0;
const TEXTURE_SET_BORROWED_00BA3750: u32 = 0x00ba_3750;
const TEXTURE_ASSIGN_00BA3940: u32 = 0x00ba_3940;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
/// `ImageSpaceEffect::SetTexture` (`00ba3cb0`, `thiscall` on the effect: index, the
/// `ImageSpaceTexture`, the filter mode), `ImageSpaceShaderParam::SetTexOffset` (`00b8a6e0`:
/// index, width, height) and `::SetPixelConstant` (`00b8a790`: index, four floats),
/// `BSShaderManager::GetShader` (`00b55560`, `cdecl`), `DebugShader::SetTextureToRender`
/// (`00bd22c0`, `thiscall`, one word), `BSTextureManager::BorrowRenderedTexture` (`00b6d5e0`,
/// `thiscall` on the manager, ten words) and `_ftol2` (`00ec62c0`, the value in `ST0`)
/// (engine map).
const EFFECT_SET_TEXTURE_00BA3CB0: u32 = 0x00ba_3cb0;
const PARAM_SET_TEX_OFFSET_00B8A6E0: u32 = 0x00b8_a6e0;
const PARAM_SET_PIXEL_CONSTANT_00B8A790: u32 = 0x00b8_a790;
const GET_SHADER_00B55560: u32 = 0x00b5_5560;
const DEBUG_SET_TEXTURE_00BD22C0: u32 = 0x00bd_22c0;
const MANAGER_BORROW_00B6D5E0: u32 = 0x00b6_d5e0;
const FTOL_00EC62C0: u32 = 0x00ec_62c0;
/// The texture manager (`011f91a8`), the Direct3D renderer (`011f9508`), the image-space
/// manager (`011f91ac`), the debug byte `012005a0`, the byte `012005a2`, the `float` 2^32
/// (`01099378`) and the three `float`s the render sets (`012003c4`, `012003c8`, `011adbd4`).
const TEXTURE_MANAGER_011F91A8: u32 = 0x011f_91a8;
const D3D_RENDERER_011F9508: u32 = 0x011f_9508;
const IMAGE_SPACE_MANAGER_011F91AC: u32 = 0x011f_91ac;
const DEBUG_TEXTURES_012005A0: u32 = 0x0120_05a0;
const FLAG_012005A2: u32 = 0x0120_05a2;
const TWO_TO_32_01099378: u32 = 0x0109_9378;
const LEVEL_012003C4: u32 = 0x0120_03c4;
const VALUE_012003C8: u32 = 0x0120_03c8;
const VALUE_011ADBD4: u32 = 0x011a_dbd4;

/// `1 / size` as the render computes it: `FILD` of the word, the `float` 2^32 added when it is
/// negative (an unsigned size), `FLD1`, `FDIVRP`, stored to a `float` (its bits).
fn reciprocal(e: &mut Engine, size: u32) -> u32 {
    let mut value = f64::from(size as i32);
    if (size as i32) < 0 {
        value = x87_add(value, x87_m32(e.global::<u32>(TWO_TO_32_01099378)));
    }
    x87_store32(x87_div(1.0, value))
}

/// The holder's texture asked twice (`GetTexture`); the first answers its height (slot +0x98),
/// the second its width (+0x94); `SetPixelConstant(param, 2, 1 / width, 1 / height, 0, 0)`.
fn set_texel_size(e: &mut Engine, param: u32, holder: u32) {
    let first = e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![holder]).u32();
    let second = e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![holder]).u32();
    let height = e.vcall(first, 0x98, &args![]).u32();
    let inverse_height = reciprocal(e, height);
    let width = e.vcall(second, 0x94, &args![]).u32();
    let inverse_width = reciprocal(e, width);
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![param, 2u32, inverse_width, inverse_height, 0u32, 0u32],
    );
}

/// `GetTexture` of `height_holder` and of `width_holder`, then
/// `SetTexOffset(param, 0, width, height)` with the first's height (asked first) and the
/// second's width.
fn set_tex_offset(e: &mut Engine, param: u32, height_holder: u32, width_holder: u32) {
    let first = e
        .call(TEXTURE_GET_TEXTURE_00BA3780, &args![height_holder])
        .u32();
    let second = e
        .call(TEXTURE_GET_TEXTURE_00BA3780, &args![width_holder])
        .u32();
    let height = e.vcall(first, 0x98, &args![]).u32();
    let width = e.vcall(second, 0x94, &args![]).u32();
    e.call(
        PARAM_SET_TEX_OFFSET_00B8A6E0,
        &args![param, 0u32, width, height],
    );
}

/// With the debug byte `012005a0` set, the holder's texture goes to the debug shader
/// (`GetShader(9)`, `SetTextureToRender`).
fn debug_texture(e: &mut Engine, holder: u32) {
    let texture = e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![holder]).u32();
    let shader = e.call(GET_SHADER_00B55560, &args![9u32]).u32();
    e.call(DEBUG_SET_TEXTURE_00BD22C0, &args![shader, texture]);
}

/// A sub-effect's render (its slot +4) with the render's geometry and renderer words, the
/// parameter and 1.
fn render_effect(e: &mut Engine, effect: u32, geometry: u32, renderer: u32, param: u32) {
    e.vcall(effect, 4, &args![geometry, renderer, param, 1u32]);
}

/// `ImageSpaceEffect::SetTexture` on `effect`.
fn set_texture(e: &mut Engine, effect: u32, index: u32, holder: u32, filter: u32) {
    e.call(
        EFFECT_SET_TEXTURE_00BA3CB0,
        &args![effect, index, holder, filter],
    );
}

/// The effect list's and the parameter list's element `index` (`pEffectList` +0x0c,
/// `pEffectParamList` +0x1c: their element pointers).
fn sub_effect(e: &Engine, this: u32, index: u32) -> (u32, u32) {
    let effect = e.mem.u32(e.mem.u32(this + 0xc) + 4 * index);
    let param = e.mem.u32(e.mem.u32(this + 0x1c) + 4 * index);
    (effect, param)
}

/// The Direct3D device (`011f4748` +0x288) slot +0x114 with 0, 0x0a and `value`.
fn device_state(e: &mut Engine, value: u32) {
    let renderer = e.global::<u32>(RENDERER_011F4748);
    let device = e.mem.u32(renderer + 0x288);
    e.vcall(device, 0x114, &args![0u32, 0xau32, value]);
}

// Translated from 00badb90 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00badb90 - the C++ exception frame (FS:[0] chain, unwind states 0 to 4 for the five local textures) is not translated.
/// `ImageSpaceEffectHDR::Render` (Xbox PDB), `thiscall`, four words (the geometry, the
/// renderer, the parameter, a word it does not read). Five `ImageSpaceTexture` locals (the
/// exe's frame; zeroed as their constructor does): the source `T1`, the targets `T2`, `T3`,
/// `T4` and, for the anisotropic down-sample only, `T5`.
///
/// With `bParamsChanged` the effect's slot +0x1c (the parameter) answers the new flag. `T2`
/// borrows a target (kind 0) and `T1` takes `pTextures[1]`'s rendered texture; the target's
/// width and height start the down-sampling. With `bUseAnisoDownsample` sub-effect 7 renders
/// twice (into `T5`, borrowed kind 1, then from `T5` into `T2`) with the device's
/// anisotropy state (0x0a) at 8 and back at 1, the second time with `pTextures[1]`'s
/// `bForceAniso` set; else sub-effect 5 renders `T1` into `T2` with the texel size. `T1`
/// takes `T2` and `T3` takes `T1`. While the width or the height is above 1, both are
/// divided by 4 (at least 1), `T2` borrows a target of that size (format 0x71) and, unless
/// both are 1, sub-effect 5 renders `T1` into it; `T1` is returned unless it holds
/// `pTextures[1]`'s texture or `T3`'s, and takes `T2`. Then the adapted light (`012005a4`):
/// without one sub-effect 12 renders `T1` into `T2`, with one sub-effect 11 blends it in and
/// it is returned; it takes `T2`. `T4` borrows kind 2; the light level (`pfData[1]`
/// truncated, minus 1, clamped to 0..6) picks the manager's effect (+8, +0x5c onwards) that
/// renders `T3` (twice), `T4` and the adapted light with the previous parameter; the
/// globals `012003c4` (level + 1), `012003c8` (`pfData[7]`) and `011adbd4` (`pfData[6]`) are
/// set. Last, sub-effect 4 (with the byte `012005a2`) or 3 renders `pTextures[0]`, `T3`,
/// `pTextures[1]` and the light with parameter 2. With the debug byte the textures go to the
/// debug shader; `T3` and `T4` are returned, `T1` unless it holds the light's texture, and
/// the locals destroyed.
pub fn image_space_effect_hdr_render(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    renderer: u32,
    param: u32,
    _unused_3: u32,
) {
    let base = this.addr();
    e.with_stack(0x60, |e, frame| {
        // The frame from the exe's +0x14: T1 +0x10, T2 +0x20, T3 +0x30, T5 +0x40, T4 +0x50.
        let frame = frame.addr();
        let t1 = frame + 0x10;
        let t2 = frame + 0x20;
        let t3 = frame + 0x30;
        let t5 = frame + 0x40;
        let t4 = frame + 0x50;
        if e.mem.u8(base + 5) != 0 {
            let changed = e.vcall(base, 0x1c, &args![param]).u32();
            e.mem.set_u8(base + 5, changed as u8);
        }
        e.call(TEXTURE_BORROW_00BA3840, &args![t2, 0u32]);
        let source = e.mem.u32(e.mem.u32(base + 0x2c) + 4);
        let rendered = e.call(TEXTURE_GET_RENDERED_00BA3770, &args![source]).u32();
        e.call(TEXTURE_SET_TEXTURE_00BA37A0, &args![t1, rendered]);
        let target = e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![t2]).u32();
        let mut width = e.vcall(target, 0x94, &args![]).u32() as i32;
        let target = e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![t2]).u32();
        let mut height = e.vcall(target, 0x98, &args![]).u32() as i32;
        let flag = u32::from(e.global::<u8>(FLAG_011F941F) != 0);
        if e.mem.u8(base + 0x68) != 0 {
            for offset in 0..3u32 {
                e.mem.set_u8(t5 + offset, 0);
            }
            for offset in [4u32, 8, 0xc] {
                e.mem.set_u32(t5 + offset, 0);
            }
            let (effect, effect_param) = sub_effect(e, base, 7);
            e.call(TEXTURE_BORROW_00BA3840, &args![t5, 1u32]);
            set_texture(e, effect, 0, t5, 0);
            set_texture(e, effect, 1, t1, 7);
            device_state(e, 8);
            set_tex_offset(e, effect_param, t1, t1);
            render_effect(e, effect, geometry, renderer, effect_param);
            set_texture(e, effect, 0, t2, 0);
            set_texture(e, effect, 1, t5, 0);
            set_tex_offset(e, effect_param, t2, t1);
            let input = e.mem.u32(e.mem.u32(effect + 0x2c) + 4);
            e.mem.set_u8(input, 1);
            render_effect(e, effect, geometry, renderer, effect_param);
            device_state(e, 1);
            e.call(TEXTURE_RETURN_RENDERED_00BA39A0, &args![t5]);
            e.call(TEXTURE_DESTRUCT_00BA3A00, &args![t5]);
        } else {
            let (effect, effect_param) = sub_effect(e, base, 5);
            set_texture(e, effect, 0, t2, 0);
            set_texture(e, effect, 1, t1, flag);
            set_texel_size(e, effect_param, t1);
            render_effect(e, effect, geometry, renderer, effect_param);
        }
        if e.global::<u8>(DEBUG_TEXTURES_012005A0) != 0 {
            debug_texture(e, t1);
        }
        let assigned = e.call(TEXTURE_ASSIGN_00BA3940, &args![t1, t2]).u32();
        e.call(TEXTURE_ASSIGN_00BA3940, &args![t3, assigned]);
        while width > 1 || height > 1 {
            width = (width / 4).max(1);
            height = (height / 4).max(1);
            let manager = e.global::<u32>(TEXTURE_MANAGER_011F91A8);
            let d3d = e.global::<u32>(D3D_RENDERER_011F9508);
            let borrowed = e
                .call(
                    MANAGER_BORROW_00B6D5E0,
                    &args![
                        manager,
                        d3d,
                        width as u32,
                        height as u32,
                        4u32,
                        0x71u32,
                        0u32,
                        0u32,
                        0u32,
                        0u32,
                        0u32
                    ],
                )
                .u32();
            e.call(TEXTURE_SET_TEXTURE_00BA37A0, &args![t2, borrowed]);
            e.call(TEXTURE_SET_BORROWED_00BA3750, &args![t2, 1u32]);
            if height > 1 || width > 1 {
                let (effect, effect_param) = sub_effect(e, base, 5);
                set_texture(e, effect, 0, t2, 0);
                set_texture(e, effect, 1, t1, flag);
                set_texel_size(e, effect_param, t1);
                render_effect(e, effect, geometry, renderer, effect_param);
                let source = e.mem.u32(e.mem.u32(base + 0x2c) + 4);
                let source_texture = e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![source]).u32();
                if e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![t1]).u32() != source_texture {
                    let kept = e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![t3]).u32();
                    if e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![t1]).u32() != kept {
                        e.call(TEXTURE_RETURN_RENDERED_00BA39A0, &args![t1]);
                    }
                }
                e.call(TEXTURE_ASSIGN_00BA3940, &args![t1, t2]);
            }
        }
        let light = e
            .call(TEXTURE_GET_TEXTURE_00BA3780, &args![ADAPTED_LIGHT_012005A4])
            .u32();
        let last_param = if light == 0 {
            let (effect, effect_param) = sub_effect(e, base, 0xc);
            set_texture(e, effect, 0, t2, 0);
            set_texture(e, effect, 1, t1, flag);
            set_texel_size(e, effect_param, t2);
            render_effect(e, effect, geometry, renderer, effect_param);
            effect_param
        } else {
            let (effect, effect_param) = sub_effect(e, base, 0xb);
            set_texture(e, effect, 0, t2, 0);
            set_texture(e, effect, 1, t1, flag);
            set_texture(e, effect, 2, ADAPTED_LIGHT_012005A4, 0);
            set_texel_size(e, effect_param, t2);
            render_effect(e, effect, geometry, renderer, effect_param);
            e.call(
                TEXTURE_RETURN_RENDERED_00BA39A0,
                &args![ADAPTED_LIGHT_012005A4],
            );
            effect_param
        };
        e.call(TEXTURE_ASSIGN_00BA3940, &args![ADAPTED_LIGHT_012005A4, t2]);
        e.call(TEXTURE_BORROW_00BA3840, &args![t4, 2u32]);
        let data = e.mem.u32(base + 0x6c);
        let level = x87_load32(e.mem.u32(data + 4));
        let truncated = e.call(FTOL_00EC62C0, &args![level]).u32() as i32;
        let level = truncated.wrapping_sub(1).clamp(0, 6) as u32;
        let copied = x87_quiet_bits(e.mem.u32(data + 0x1c));
        e.set_global(VALUE_012003C8, copied);
        let copied = x87_quiet_bits(e.mem.u32(e.mem.u32(base + 0x6c) + 0x18));
        e.set_global(VALUE_011ADBD4, copied);
        e.set_global(LEVEL_012003C4, x87_store32(f64::from(level as i32 + 1)));
        let manager = e.global::<u32>(IMAGE_SPACE_MANAGER_011F91AC);
        let effect = e.mem.u32(e.mem.u32(manager + 8) + 4 * level + 0x5c);
        set_texture(e, effect, 0, t3, 0);
        set_texture(e, effect, 1, t3, 0);
        set_texture(e, effect, 2, t4, 0);
        if level <= 6 {
            set_texture(e, effect, 3, ADAPTED_LIGHT_012005A4, 0);
        }
        render_effect(e, effect, geometry, renderer, last_param);
        let effect = if e.global::<u8>(FLAG_012005A2) != 0 {
            e.mem.u32(e.mem.u32(base + 0xc) + 0x10)
        } else {
            e.mem.u32(e.mem.u32(base + 0xc) + 0xc)
        };
        let textures = e.mem.u32(base + 0x2c);
        let first = e.mem.u32(textures);
        let final_param = e.mem.u32(e.mem.u32(base + 0x1c) + 8);
        set_texture(e, effect, 0, first, 0);
        set_texture(e, effect, 1, t3, flag);
        let second = e.mem.u32(e.mem.u32(base + 0x2c) + 4);
        set_texture(e, effect, 2, second, 0);
        set_texture(e, effect, 3, ADAPTED_LIGHT_012005A4, 0);
        render_effect(e, effect, geometry, renderer, final_param);
        if e.global::<u8>(DEBUG_TEXTURES_012005A0) != 0 {
            debug_texture(e, t3);
            debug_texture(e, ADAPTED_LIGHT_012005A4);
            debug_texture(e, t4);
        }
        e.call(TEXTURE_RETURN_RENDERED_00BA39A0, &args![t3]);
        e.call(TEXTURE_RETURN_RENDERED_00BA39A0, &args![t4]);
        let light = e
            .call(TEXTURE_GET_TEXTURE_00BA3780, &args![ADAPTED_LIGHT_012005A4])
            .u32();
        if e.call(TEXTURE_GET_TEXTURE_00BA3780, &args![t1]).u32() != light {
            e.call(TEXTURE_RETURN_RENDERED_00BA39A0, &args![t1]);
        }
        for holder in [t4, t3, t2, t1] {
            e.call(TEXTURE_DESTRUCT_00BA3A00, &args![holder]);
        }
    });
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_d500, fn_00bad500(Ptr) -> Ptr),
        entry!(0x00ba_d400, fn_00bad400(Ptr)),
        entry!(0x00ba_d4a0, image_space_effect_hdr_clear_adapted_light()),
        entry!(0x00ba_d4d0, fn_00bad4d0(Ptr) -> u32),
        entry!(0x00ba_d4f0, fn_00bad4f0(Ptr, u32)),
        entry!(0x00ba_d560, fn_00bad560(Ptr, u32) -> Ptr),
        entry!(0x00ba_d580, fn_00bad580(Ptr, u32, u32)),
        entry!(
            0x00ba_db90,
            image_space_effect_hdr_render(Ptr, u32, u32, u32, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bad500_builds_the_effect() {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.register(ARRAY_SET_SIZE_00A90180, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x70);
        assert_eq!(e.call(0x00ba_d500, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B8C8C);
        assert_eq!(e.mem.u32(this + 0x6c), DATA_011ADD0C);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 3]),
            ]
        );
    }
}

#[cfg(test)]
mod hdr_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    const PARAM_DELETE: u32 = 0x0e10_0000;
    const PARAM_VTABLE: u32 = 0x0e11_0000;

    #[test]
    fn fn_00bad400_deletes_every_parameter() {
        let mut e = rig();
        e.register(ARRAY_SET_AT_00BA3AA0, |_, _| Ret::default());
        e.register(EFFECT_DESTRUCT_00BA3D30, |_, _| Ret::default());
        e.register(PARAM_DELETE, |_, _| Ret::default());
        e.put_vtable(PARAM_VTABLE, &[PARAM_DELETE]);
        let this = e.mem.alloc(0x70);
        let params = e.mem.alloc(0xd * 4);
        let first = e.mem.alloc(8);
        let last = e.mem.alloc(8);
        e.mem.set_u32(first, PARAM_VTABLE);
        e.mem.set_u32(last, PARAM_VTABLE);
        e.mem.set_u32(params, first);
        e.mem.set_u32(params + 0xc * 4, last);
        e.mem.set_u32(this + 0x1c, params);
        e.call(0x00ba_d400, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010B8C8C);
        assert_eq!(calls(&e, PARAM_DELETE), vec![vec![first, 1], vec![last, 1]]);
        let sets: Vec<(u32, u32)> = calls(&e, ARRAY_SET_AT_00BA3AA0)
            .iter()
            .map(|c| (c[0], c[1]))
            .collect();
        assert_eq!(sets, vec![(this + 0x18, 0), (this + 0x18, 0xc)]);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
    }

    #[test]
    fn fn_00bad560_destroys_and_frees_on_bit_0() {
        let mut e = rig();
        for callee in [EFFECT_DESTRUCT_00BA3D30, MEMORY_DEALLOCATE_00AA4060] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x70);
        let params = e.mem.alloc(0xd * 4);
        e.mem.set_u32(this + 0x1c, params);
        assert_eq!(e.call(0x00ba_d560, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, MEMORY_DEALLOCATE_00AA4060).is_empty());
        assert_eq!(e.call(0x00ba_d560, &args![this, 3u32]).u32(), this);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, this]]
        );
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30).len(), 2);
    }

    #[test]
    fn fn_00bad4a0_returns_and_clears_a_held_light() {
        let mut e = rig();
        e.register(TEXTURE_RETURN_RENDERED_00BA39A0, |_, _| Ret::default());
        e.register(TEXTURE_CLEAR_00BA38B0, |_, _| Ret::default());
        e.register(TEXTURE_GET_TEXTURE_00BA3780, |_, _| Ret::default());
        e.call(0x00ba_d4a0, &args![]);
        assert!(calls(&e, TEXTURE_RETURN_RENDERED_00BA39A0).is_empty());
        e.register(TEXTURE_GET_TEXTURE_00BA3780, |_, _| 0x55u32.into_ret());
        e.call(0x00ba_d4a0, &args![]);
        assert_eq!(
            calls(&e, TEXTURE_GET_TEXTURE_00BA3780),
            vec![vec![ADAPTED_LIGHT_012005A4], vec![ADAPTED_LIGHT_012005A4]]
        );
        assert_eq!(
            calls(&e, TEXTURE_RETURN_RENDERED_00BA39A0),
            vec![vec![ADAPTED_LIGHT_012005A4]]
        );
        assert_eq!(
            calls(&e, TEXTURE_CLEAR_00BA38B0),
            vec![vec![ADAPTED_LIGHT_012005A4]]
        );
    }

    #[test]
    fn fn_00bad4d0_needs_the_byte_and_the_data() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        let this = e.mem.alloc(0x70);
        assert_eq!(e.call(0x00ba_d4d0, &args![this]).u32(), 0);
        e.set_global(HDR_ENABLED_011F941E, 1u8);
        assert_eq!(e.call(0x00ba_d4d0, &args![this]).u32(), 0);
        e.mem.set_u32(this + 0x6c, 0x1234);
        assert_eq!(e.call(0x00ba_d4d0, &args![this]).u32(), 1);
        e.set_global(HDR_ENABLED_011F941E, 0u8);
        assert_eq!(e.call(0x00ba_d4d0, &args![this]).u32(), 0);
    }

    #[test]
    fn fn_00bad4f0_keeps_the_data_and_marks_the_change() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x70);
        e.call(0x00ba_d4f0, &args![this, 0x4321u32]);
        assert_eq!(e.mem.u8(this + 5), 1);
        assert_eq!(e.mem.u32(this + 0x6c), 0x4321);
    }

    const RENDERER_SLOTS: u32 = 0x0e12_0000;
    const DEVICE_SLOTS: u32 = 0x0e13_0000;

    /// The set-up's doubles: the parameters numbered from 0x0e40_0000 by allocation, and
    /// the renderer's object answering `answer` from its slot +0x8c.
    fn setup_rig(picked: u8, aniso: u8, answer: u32) -> (Engine, u32, u32) {
        let mut e = rig();
        e.map(0x011f_4000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.set_global(FLAG_011F941F, picked);
        e.set_global(FLAG_011F9420, aniso);
        let renderer = e.mem.alloc(8);
        let device = e.mem.alloc(8);
        e.put_vtable(RENDERER_SLOTS, &[0; 0x33]);
        e.mem
            .set_u32(RENDERER_SLOTS + 0xc8, RENDERER_SLOTS + 0x1000);
        e.put_vtable(DEVICE_SLOTS, &[0; 0x24]);
        e.mem.set_u32(DEVICE_SLOTS + 0x8c, DEVICE_SLOTS + 0x1000);
        e.mem.set_u32(renderer, RENDERER_SLOTS);
        e.mem.set_u32(device, DEVICE_SLOTS);
        e.register_double(RENDERER_SLOTS + 0x1000, move |_, _| device.into_ret());
        e.register_double(DEVICE_SLOTS + 0x1000, move |_, _| answer.into_ret());
        e.set_global(RENDERER_011F4748, renderer);
        let mut next = 0x0e40_0000u32;
        e.register_double(PARAM_CONSTRUCT_00B8A9E0, move |_, _| {
            next += 0x100;
            next.into_ret()
        });
        for callee in [
            SIZE_ARRAYS_00BA3F90,
            ARRAY_SET_SIZE_00A90180,
            PARAM_INIT_CONSTANT_MAP_00B8AB10,
            PARAM_SET_UP_00B8A7C0,
            PARAM_CONSTANTS_00B8A8B0,
            SET_EFFECT_00BA3C70,
            ARRAY_SET_AT_00BA3AA0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.map(0x0e40_0000, 0x1000);
        e.mem.write(0x0e40_0000, &[0xcc; 0x1000]);
        let this = e.mem.alloc(0x70);
        let manager = e.mem.alloc(0x10);
        let effects = e.mem.alloc(0x120);
        for offset in (0..0x120u32).step_by(4) {
            e.mem.set_u32(effects + offset, 0x0e50_0000 + offset);
        }
        e.mem.set_u32(manager + 8, effects);
        (e, this, manager)
    }

    #[test]
    fn fn_00bad580_sets_up_the_eleven_parameters() {
        let (mut e, this, manager) = setup_rig(0, 0, 0);
        e.call(0x00ba_d580, &args![this, manager, 0u32]);
        assert_eq!(calls(&e, SIZE_ARRAYS_00BA3F90), vec![vec![this, 0xd]]);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + 0x28, 3]]
        );
        assert_eq!(e.mem.u8(this + 0x68), 0);
        let params: Vec<u32> = (1..=11).map(|i| 0x0e40_0000 + 0x100 * i).collect();
        let effects: Vec<Vec<u32>> = calls(&e, SET_EFFECT_00BA3C70);
        let expected: Vec<Vec<u32>> = [
            (7u32, 0x88u32),
            (5, 0xfc),
            (0xc, 0x110),
            (0xb, 0x114),
            (6, 0x100),
            (0xa, 0x11c),
            (9, 0x118),
            (8, 0xf8),
            (2, 0xe8),
            (3, 0xec),
            (4, 0xf0),
        ]
        .iter()
        .zip(&params)
        .map(|((index, offset), param)| vec![this, *index, 0x0e50_0000 + offset, *param, 0])
        .collect();
        assert_eq!(effects, expected);
        let maps: Vec<u32> = calls(&e, PARAM_INIT_CONSTANT_MAP_00B8AB10)
            .iter()
            .map(|c| c[2])
            .collect();
        assert_eq!(
            maps,
            vec![0, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18]
        );
        assert_eq!(calls(&e, PARAM_SET_UP_00B8A7C0).len(), 8);
        assert_eq!(
            calls(&e, PARAM_CONSTANTS_00B8A8B0),
            vec![
                vec![params[1], 3, TABLE_011ADD88, 0x100],
                vec![params[2], 3, TABLE_011ADD88, 0x100],
                vec![params[3], 3, TABLE_011ADD88, 0x100],
                vec![params[4], 3, TABLE_011ADE88, 0x90],
            ]
        );
        // The bytes +0x55 and +0x56: cleared on parameter 1 (+0x55 only), 3 (none), 4 (+0x56).
        assert_eq!(
            (e.mem.u8(params[0] + 0x55), e.mem.u8(params[0] + 0x56)),
            (0, 0xcc)
        );
        assert_eq!(
            (e.mem.u8(params[2] + 0x55), e.mem.u8(params[2] + 0x56)),
            (0xcc, 0xcc)
        );
        assert_eq!(
            (e.mem.u8(params[3] + 0x55), e.mem.u8(params[3] + 0x56)),
            (0xcc, 0)
        );
        assert_eq!(
            (e.mem.u8(params[6] + 0x55), e.mem.u8(params[6] + 0x56)),
            (0, 0)
        );
        let set_at: Vec<(u32, u32)> = calls(&e, ARRAY_SET_AT_00BA3AA0)
            .iter()
            .map(|c| (c[0], c[1]))
            .collect();
        assert_eq!(set_at.len(), 11);
        assert_eq!(set_at[0], (this + 0x18, 7));
        // Parameter 1 is stored before its effect, the others after: compare the log order.
        let log = e.call_log.as_ref().unwrap();
        let position = |addr: u32, index: u32| {
            log.iter()
                .position(|c| c.0 == addr && c.1.get(1) == Some(&index))
                .unwrap()
        };
        assert!(position(ARRAY_SET_AT_00BA3AA0, 7) < position(SET_EFFECT_00BA3C70, 7));
        assert!(position(ARRAY_SET_AT_00BA3AA0, 5) > position(SET_EFFECT_00BA3C70, 5));
    }

    #[test]
    fn fn_00bad580_with_both_bytes_picks_the_small_tables_and_the_aniso_flag() {
        let (mut e, this, manager) = setup_rig(1, 1, 0x500);
        e.call(0x00ba_d580, &args![this, manager, 0u32]);
        assert_eq!(e.mem.u8(this + 0x68), 1);
        let constants: Vec<(u32, u32)> = calls(&e, PARAM_CONSTANTS_00B8A8B0)
            .iter()
            .map(|c| (c[2], c[3]))
            .collect();
        assert_eq!(
            constants,
            vec![
                (TABLE_011ADD48, 0x40),
                (TABLE_011ADD48, 0x40),
                (TABLE_011ADD48, 0x40),
                (TABLE_011ADE88, 0x90),
            ]
        );
        let offsets: Vec<u32> = calls(&e, SET_EFFECT_00BA3C70)
            .iter()
            .map(|c| c[2] - 0x0e50_0000)
            .collect();
        assert_eq!(&offsets[1..4], &[0x104, 0x108, 0x10c]);
        let (mut e, this, manager) = setup_rig(1, 1, 0x501);
        e.call(0x00ba_d580, &args![this, manager, 0u32]);
        assert_eq!(e.mem.u8(this + 0x68), 0);
    }

    const TEXTURE_VTABLE: u32 = 0x0e14_0000;
    const TEXTURE_WIDTH: u32 = 0x0e15_0094;
    const TEXTURE_HEIGHT: u32 = 0x0e15_0098;
    const EFFECT_VTABLE: u32 = 0x0e16_0000;
    const EFFECT_RENDER: u32 = 0x0e17_0004;
    const HDR_VTABLE: u32 = 0x0e18_0000;
    const HDR_UPDATE: u32 = 0x0e19_001c;
    const DEVICE_STATE: u32 = 0x0e1a_0114;

    /// A texture object answering `width` and `height` from its slots +0x94 and +0x98.
    fn texture(e: &mut Engine, width: u32, height: u32) -> u32 {
        let texture = e.mem.alloc(0x10);
        e.mem.set_u32(texture, TEXTURE_VTABLE);
        e.mem.set_u32(texture + 8, width);
        e.mem.set_u32(texture + 0xc, height);
        texture
    }

    /// A sub-effect with a render slot and an input texture holder at its `pTextures[1]`.
    fn effect(e: &mut Engine) -> u32 {
        let effect = e.mem.alloc(0x40);
        e.mem.set_u32(effect, EFFECT_VTABLE);
        let inputs = e.mem.alloc(8);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(inputs + 4, holder);
        e.mem.set_u32(effect + 0x2c, inputs);
        effect
    }

    struct Hdr {
        e: Engine,
        this: u32,
        effects: Vec<u32>,
        levels: Vec<u32>,
    }

    /// The render's world: the effect's 13 sub-effects (parameters `0x0e70_0000 + 0x100 *
    /// index`), its `pTextures` (0 and 1, `pTextures[1]` holding a `source` texture), the
    /// textures the borrows of kinds 0, 1 and 2 answer, the managers and the globals, every
    /// callee doubled on the holders' `pTexture` words.
    fn hdr_rig(source: (u32, u32), first_target: (u32, u32), level: f32) -> Hdr {
        let mut e = rig();
        for page in [
            0x011f_4000u32,
            0x011f_9000,
            0x0120_0000,
            0x011a_d000,
            0x0109_9000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(TWO_TO_32_01099378, 4_294_967_296.0f32);
        e.put_vtable(TEXTURE_VTABLE, &[0; 0x27]);
        e.mem.set_u32(TEXTURE_VTABLE + 0x94, TEXTURE_WIDTH);
        e.mem.set_u32(TEXTURE_VTABLE + 0x98, TEXTURE_HEIGHT);
        e.register(TEXTURE_WIDTH, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(TEXTURE_HEIGHT, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        e.put_vtable(EFFECT_VTABLE, &[0, EFFECT_RENDER]);
        e.register(EFFECT_RENDER, |_, _| Ret::default());
        let mut slots = vec![0u32; 8];
        slots[7] = HDR_UPDATE;
        e.put_vtable(HDR_VTABLE, &slots);
        e.register(HDR_UPDATE, |_, _| 0x0000_0100u32.into_ret());
        let mut device_slots = vec![0u32; 0x46];
        device_slots[0x45] = DEVICE_STATE;
        e.put_vtable(0x0e1b_0000, &device_slots);
        e.register(DEVICE_STATE, |_, _| Ret::default());
        let device = e.mem.alloc(8);
        e.mem.set_u32(device, 0x0e1b_0000);
        let renderer = e.mem.alloc(0x290);
        e.mem.set_u32(renderer + 0x288, device);
        e.set_global(RENDERER_011F4748, renderer);
        e.set_global(TEXTURE_MANAGER_011F91A8, 0x0e1c_0000u32);
        e.set_global(D3D_RENDERER_011F9508, 0x0e1d_0000u32);
        let textures = [
            texture(&mut e, first_target.0, first_target.1),
            texture(&mut e, 2, 2),
            texture(&mut e, 64, 64),
        ];
        e.register_double(TEXTURE_BORROW_00BA3840, move |e, a| {
            e.mem.set_u32(a[0] + 4, textures[a[1] as usize]);
            Ret::default()
        });
        e.register(TEXTURE_GET_TEXTURE_00BA3780, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(TEXTURE_GET_RENDERED_00BA3770, |e, a| {
            e.mem.u32(a[0] + 4).into_ret()
        });
        e.register(TEXTURE_SET_TEXTURE_00BA37A0, |e, a| {
            e.mem.set_u32(a[0] + 4, a[1]);
            Ret::default()
        });
        e.register(TEXTURE_ASSIGN_00BA3940, |e, a| {
            let texture = e.mem.u32(a[1] + 4);
            e.mem.set_u32(a[0] + 4, texture);
            a[0].into_ret()
        });
        e.register(MANAGER_BORROW_00B6D5E0, |e, a| {
            let borrowed = e.mem.alloc(0x10);
            e.mem.set_u32(borrowed, TEXTURE_VTABLE);
            e.mem.set_u32(borrowed + 8, a[2]);
            e.mem.set_u32(borrowed + 0xc, a[3]);
            borrowed.into_ret()
        });
        e.register(FTOL_00EC62C0, |_, a| {
            let value = f64::take(a, &mut 0);
            (value as i32 as u32).into_ret()
        });
        e.register(GET_SHADER_00B55560, |_, _| 0x0e1e_0000u32.into_ret());
        for callee in [
            TEXTURE_RETURN_RENDERED_00BA39A0,
            TEXTURE_SET_BORROWED_00BA3750,
            TEXTURE_DESTRUCT_00BA3A00,
            EFFECT_SET_TEXTURE_00BA3CB0,
            PARAM_SET_TEX_OFFSET_00B8A6E0,
            PARAM_SET_PIXEL_CONSTANT_00B8A790,
            DEBUG_SET_TEXTURE_00BD22C0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x70);
        e.mem.set_u32(this, HDR_VTABLE);
        let effects: Vec<u32> = (0..13).map(|_| effect(&mut e)).collect();
        let list = e.mem.alloc(13 * 4);
        let params = e.mem.alloc(13 * 4);
        for (index, effect) in effects.iter().enumerate() {
            e.mem.set_u32(list + 4 * index as u32, *effect);
            e.mem.set_u32(
                params + 4 * index as u32,
                0x0e70_0000 + 0x100 * index as u32,
            );
        }
        e.mem.set_u32(this + 0xc, list);
        e.mem.set_u32(this + 0x1c, params);
        let inputs = e.mem.alloc(8);
        let first_input = e.mem.alloc(0x10);
        let second_input = e.mem.alloc(0x10);
        let source = texture(&mut e, source.0, source.1);
        e.mem.set_u32(second_input + 4, source);
        e.mem.set_u32(inputs, first_input);
        e.mem.set_u32(inputs + 4, second_input);
        e.mem.set_u32(this + 0x2c, inputs);
        let data = e.mem.alloc(0x20);
        e.mem.set_f32(data + 4, level);
        e.mem.set_f32(data + 0x18, 6.5);
        e.mem.set_f32(data + 0x1c, 7.5);
        e.mem.set_u32(this + 0x6c, data);
        let manager = e.mem.alloc(0x10);
        let levels: Vec<u32> = (0..7).map(|_| effect(&mut e)).collect();
        let level_list = e.mem.alloc(0x5c + 7 * 4);
        for (index, effect) in levels.iter().enumerate() {
            e.mem.set_u32(level_list + 0x5c + 4 * index as u32, *effect);
        }
        e.mem.set_u32(manager + 8, level_list);
        e.set_global(IMAGE_SPACE_MANAGER_011F91AC, manager);
        Hdr {
            e,
            this,
            effects,
            levels,
        }
    }

    /// The renders, as (effect, parameter).
    fn renders(e: &Engine) -> Vec<(u32, u32)> {
        calls(e, EFFECT_RENDER)
            .iter()
            .map(|c| (c[0], c[3]))
            .collect()
    }

    fn param(index: u32) -> u32 {
        0x0e70_0000 + 0x100 * index
    }

    #[test]
    fn fn_00badb90_down_samples_and_renders_without_a_light() {
        let mut hdr = hdr_rig((8, 2), (4, 4), 3.7);
        let e = &mut hdr.e;
        e.set_global(FLAG_011F941F, 1u8);
        let this = hdr.this;
        e.call(
            0x00ba_db90,
            &args![this, 0x6000u32, 0x7000u32, 0x8000u32, 0u32],
        );
        assert!(calls(e, HDR_UPDATE).is_empty());
        assert_eq!(
            renders(e),
            vec![
                (hdr.effects[5], param(5)),
                (hdr.effects[0xc], param(0xc)),
                (hdr.levels[2], param(0xc)),
                (hdr.effects[3], param(2)),
            ]
        );
        let render = &calls(e, EFFECT_RENDER)[0];
        assert_eq!(render[1..], [0x6000, 0x7000, param(5), 1]);
        // The source's texel size, then the 1 x 1 target's.
        assert_eq!(
            calls(e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![
                vec![param(5), 2, 0.125f32.to_bits(), 0.5f32.to_bits(), 0, 0],
                vec![param(0xc), 2, 1.0f32.to_bits(), 1.0f32.to_bits(), 0, 0],
            ]
        );
        let borrows = calls(e, MANAGER_BORROW_00B6D5E0);
        assert_eq!(
            borrows,
            vec![vec![0x0e1c_0000, 0x0e1d_0000, 1, 1, 4, 0x71, 0, 0, 0, 0, 0]]
        );
        assert_eq!(e.global::<f32>(LEVEL_012003C4), 3.0);
        assert_eq!(e.global::<f32>(VALUE_012003C8), 7.5);
        assert_eq!(e.global::<f32>(VALUE_011ADBD4), 6.5);
        // The source carries the flag 1 into sub-effect 5 and the last effect.
        let textures = calls(e, EFFECT_SET_TEXTURE_00BA3CB0);
        assert_eq!(textures[1][0], hdr.effects[5]);
        assert_eq!((textures[1][1], textures[1][3]), (1, 1));
        let last = &textures[textures.len() - 4..];
        let first_input = e.mem.u32(e.mem.u32(this + 0x2c));
        assert_eq!(last[0][..2], [hdr.effects[3], 0]);
        assert_eq!(last[0][2], first_input);
        assert_eq!(last[3][1..], [3, ADAPTED_LIGHT_012005A4, 0]);
        assert!(calls(e, DEBUG_SET_TEXTURE_00BD22C0).is_empty());
        assert_eq!(calls(e, TEXTURE_DESTRUCT_00BA3A00).len(), 4);
        assert!(calls(e, DEVICE_STATE).is_empty());
    }

    #[test]
    fn fn_00badb90_anisotropic_with_a_light_and_the_debug_textures() {
        let mut hdr = hdr_rig((8, 8), (16, 16), 0.5);
        let e = &mut hdr.e;
        let this = hdr.this;
        e.mem.set_u8(this + 5, 1);
        e.mem.set_u8(this + 0x68, 1);
        e.set_global(DEBUG_TEXTURES_012005A0, 1u8);
        e.set_global(FLAG_012005A2, 1u8);
        let light = texture(e, 1, 1);
        e.mem.set_u32(ADAPTED_LIGHT_012005A4 + 4, light);
        e.call(
            0x00ba_db90,
            &args![this, 0x6000u32, 0x7000u32, 0x8000u32, 0u32],
        );
        assert_eq!(calls(e, HDR_UPDATE), vec![vec![this, 0x8000]]);
        assert_eq!(e.mem.u8(this + 5), 0);
        assert_eq!(
            renders(e),
            vec![
                (hdr.effects[7], param(7)),
                (hdr.effects[7], param(7)),
                (hdr.effects[5], param(5)),
                (hdr.effects[0xb], param(0xb)),
                (hdr.levels[0], param(0xb)),
                (hdr.effects[4], param(2)),
            ]
        );
        let device = calls(e, DEVICE_STATE);
        assert_eq!(device.iter().map(|c| c[3]).collect::<Vec<_>>(), vec![8, 1]);
        let input = e.mem.u32(e.mem.u32(hdr.effects[7] + 0x2c) + 4);
        assert_eq!(e.mem.u8(input), 1);
        // The source is 8 x 8, the aniso target (kind 1) 2 x 2.
        let offsets: Vec<Vec<u32>> = calls(e, PARAM_SET_TEX_OFFSET_00B8A6E0);
        assert_eq!(
            offsets,
            vec![vec![param(7), 0, 8, 8], vec![param(7), 0, 8, 16]]
        );
        // 16 x 16 down to 4 x 4 (rendered) and 1 x 1 (not).
        let sizes: Vec<(u32, u32)> = calls(e, MANAGER_BORROW_00B6D5E0)
            .iter()
            .map(|c| (c[2], c[3]))
            .collect();
        assert_eq!(sizes, vec![(4, 4), (1, 1)]);
        assert_eq!(e.global::<f32>(LEVEL_012003C4), 1.0);
        assert_eq!(calls(e, DEBUG_SET_TEXTURE_00BD22C0).len(), 4);
        assert!(calls(e, TEXTURE_RETURN_RENDERED_00BA39A0).contains(&vec![ADAPTED_LIGHT_012005A4]));
    }

    #[test]
    fn reciprocal_treats_the_size_as_unsigned() {
        let mut e = Engine::new();
        e.map(0x0109_9000, 0x1000);
        e.set_global(TWO_TO_32_01099378, 4_294_967_296.0f32);
        assert_eq!(reciprocal(&mut e, 4), 0.25f32.to_bits());
        assert_eq!(
            reciprocal(&mut e, 0x8000_0000),
            (1.0f32 / 2_147_483_648.0).to_bits()
        );
    }
}
