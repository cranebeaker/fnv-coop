//! `BSShader/run_00bb8250` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
/// The `NiTArray` set-size `00a90180` (`thiscall` on the array at +0x28, one word).
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
/// The two effects' vtables (`010b9f04`, `010b9f58`; no RTTI locator before them).
const VTABLE_010B9F04: u32 = 0x010b_9f04;
const VTABLE_010B9F58: u32 = 0x010b_9f58;
/// The four words `00bb8900` copies (`011a9be0` to `011a9bec`, to `01200800` to `0120080c`).
const WORDS_011A9BE0: u32 = 0x011a_9be0;
const WORDS_01200800: u32 = 0x0120_0800;

// Translated from 00bb8900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the vtable
/// `010b9f04`, sizes the array at +0x28 to 5 (`00a90180`), clears the byte +4 and copies the
/// four words at `011a9be0` to `01200800` (stored in the order first, fourth, second, third).
/// Answers `this`.
// Translation incomplete: 00bb8900 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00bb8900(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B9F04);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 5u32]);
    e.mem.set_u8(base + 4, 0);
    let words: Vec<u32> = (0..4u32)
        .map(|index| e.global::<u32>(WORDS_011A9BE0 + 4 * index))
        .collect();
    for index in [0usize, 3, 1, 2] {
        e.set_global(WORDS_01200800 + 4 * index as u32, words[index]);
    }
    this
}

// Translated from 00bb9180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`): an image-space effect's constructor,
/// `thiscall`, no stack words: the base constructor `00ba4080`, the vtable `010b9f58`, the
/// array at +0x28 sized to 4 (`00a90180`). Answers `this`.
// Translation incomplete: 00bb9180 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00bb9180(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B9F58);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 4u32]);
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;
#[cfg(test)]
use crate::units::bsshader::run_00bc0360::TEXTURE_DESTRUCT_00BA3A00;
use crate::units::bsshader::run_00bc0360::{
    delete_parameter, destroy_texture, new_texture, texture_at, ARRAY_SET_00BA3AA0,
    EFFECT_DESTRUCT_00BA3D30,
};

/// The vtable `00bb8250` stores (`010b9e38`; no RTTI locator before it), the base destructor it
/// runs (`00a5c240`, unit `niobjectnet.cpp`, unnamed; `thiscall`) and the sized free `00aa1460`
/// (`cdecl`: block, size).
const VTABLE_010B9E38: u32 = 0x010b_9e38;
const BASE_DESTRUCT_00A5C240: u32 = 0x00a5_c240;
const NI_FREE_SIZED_00AA1460: u32 = 0x00aa_1460;
/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00bb8250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`; slot 0 of the vtable `010b9e38`): a scalar
/// deleting destructor, `thiscall` with one stack word (the flags). Stores the vtable
/// `010b9e38`, runs `00a5c240` and, with bit 0 set, frees the 0x64 bytes (`00aa1460`). Answers
/// `this`.
pub fn fn_00bb8250(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), VTABLE_010B9E38);
    e.call(BASE_DESTRUCT_00A5C240, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED_00AA1460, &args![this, 0x64u32]);
    }
    this
}

/// The shared texture `NiPointer`s of `00bb8900`'s effect (`01200810`, `01200814`, `01200818`)
/// and of `00bb9180`'s (`0120083c`, `01200840`).
const TEXTURE_01200810: u32 = 0x0120_0810;
const TEXTURE_01200814: u32 = 0x0120_0814;
const TEXTURE_01200818: u32 = 0x0120_0818;
const TEXTURE_0120083C: u32 = 0x0120_083c;
const TEXTURE_01200840: u32 = 0x0120_0840;

// Translated from 00bb83d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`): the destructor of `00bb8900`'s effect,
/// `thiscall`, no stack words. Stores the vtable `010b9f04`, clears the shared `NiPointer`s
/// `01200810`, `01200814` and `01200818`, deletes the shader parameters 0 and 1 of the array at
/// +0x18, destroys the textures 3, 4 and 2 of the array at +0x28 and runs
/// `ImageSpaceEffect::~ImageSpaceEffect` (`00ba3d30`).
// Translation incomplete: 00bb83d0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00bb83d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010B9F04);
    for pointer in [TEXTURE_01200810, TEXTURE_01200814, TEXTURE_01200818] {
        ni_pointer_clear(e, pointer);
    }
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for index in [0u32, 1] {
            delete_parameter(e, base, index, local);
        }
        for index in [3u32, 4, 2] {
            destroy_texture(e, base, index, local);
        }
    });
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

// Translated from 00bb8990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`): the scalar deleting destructor of `00bb8900`'s
/// effect, `thiscall`, one stack word (the flags). Runs `00bb83d0`; with bit 0 set, frees `this`
/// through the memory manager (`00aa4060`). Answers `this`.
pub fn fn_00bb8990(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bb83d0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// `00ba3f90` (unnamed; `thiscall`, one word), `BSShaderManager::GetTexture` (engine map,
/// `00b55840`, `cdecl`: path, a flag, the `NiPointer` to fill, two words), the texture setter
/// `00ba37f0` and `00ba3760` (unnamed; `thiscall` on a texture, one word each) and
/// `ImageSpaceTexture::GetTexture` (engine map, `00ba3780`, `thiscall`).
const EFFECT_SET_UP_00BA3F90: u32 = 0x00ba_3f90;
const GET_TEXTURE: u32 = 0x00b5_5840;
const TEXTURE_SET_00BA37F0: u32 = 0x00ba_37f0;
const TEXTURE_CALL_00BA3760: u32 = 0x00ba_3760;
const IMAGE_SPACE_TEXTURE_GET_TEXTURE: u32 = 0x00ba_3780;
/// `"Data\Textures\Pipboy3000\PipboyScanlines.dds"` (`01078990`) and
/// `"Data\Textures\Pipboy3000\PipboyDistortEffectMap.dds"` (`010b9f24`).
const SCANLINES_PATH_01078990: u32 = 0x0107_8990;
const DISTORT_PATH_010B9F24: u32 = 0x010b_9f24;
/// `ImageSpaceShaderParam::ImageSpaceShaderParam` (`00b8a9e0`), `::InitConstantMap`
/// (`00b8ab10`, `RET 8`) and `::SetPixelConstant` (`00b8a790`, `RET 0x14`) (engine map), and the
/// effect calls `00ba3ba0` (`RET 0x1c`), `00ba3c00` (`RET 0x24`) and `00ba3c70` (`RET 0x10`)
/// (unnamed; `thiscall` on the effect).
const SHADER_PARAM_CONSTRUCT: u32 = 0x00b8_a9e0;
const SHADER_PARAM_INIT_CONSTANT_MAP: u32 = 0x00b8_ab10;
const SHADER_PARAM_SET_PIXEL_CONSTANT: u32 = 0x00b8_a790;
const EFFECT_CALL_00BA3BA0: u32 = 0x00ba_3ba0;
const EFFECT_CALL_00BA3C00: u32 = 0x00ba_3c00;
const EFFECT_CALL_00BA3C70: u32 = 0x00ba_3c70;
/// The `float` 2^32 added to a negative `FILD` (`01099378`).
const TWO_TO_32_01099378: u32 = 0x0109_9378;

/// `1 / n` for a dimension of the texture object `ImageSpaceTexture::GetTexture` gives for
/// `texture`'s slot `slot`, as the x87 has it before the store (`FILD`, 2^32 added when
/// negative, `FLD1; FDIVRP`).
fn inverse_dimension(e: &mut Engine, object: u32, slot: u32) -> f64 {
    let value = e.vcall(object, slot, &args![]).u32();
    let mut loaded = f64::from(value as i32);
    if (value as i32) < 0 {
        loaded = x87_add(loaded, x87_m32(e.global::<u32>(TWO_TO_32_01099378)));
    }
    x87_div(1.0, loaded)
}

/// A shader parameter: 0x88 bytes from the memory manager, the constructor run when the block
/// is not null; 0 for a null block.
fn new_parameter(e: &mut Engine) -> u32 {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x88u32])
        .u32();
    if block != 0 {
        e.call(SHADER_PARAM_CONSTRUCT, &args![block]).u32()
    } else {
        0
    }
}

/// `SetPixelConstant(parameter, index, ...)` with the four `float`s copied from memory (`FLD` /
/// `FSTP` pairs) or given as bits.
fn set_pixel_constant(e: &mut Engine, parameter: u32, index: u32, words: [u32; 4]) {
    e.call(
        SHADER_PARAM_SET_PIXEL_CONSTANT,
        &args![parameter, index, words[0], words[1], words[2], words[3]],
    );
}

/// A `float` global copied through the x87 (`FLD dword` / `FSTP dword`).
fn float_global(e: &Engine, addr: u32) -> u32 {
    x87_quiet_bits(e.global::<u32>(addr))
}

/// The constants `00bb89b0` sets: `011ae5ec`, `012007e8`, `011ae5f4`, `011ae5f0`; `012007f0`,
/// `012007f4`, `011ae5f8` with the byte `012007f8` as 1.0 or 0.0; `01200800` to `0120080c`; and
/// the scale `011ae5e8` the inverse texture size is multiplied by.
const CONSTANT_011AE5EC: u32 = 0x011a_e5ec;
const CONSTANT_012007E8: u32 = 0x0120_07e8;
const CONSTANT_011AE5F4: u32 = 0x011a_e5f4;
const CONSTANT_011AE5F0: u32 = 0x011a_e5f0;
const CONSTANT_012007F0: u32 = 0x0120_07f0;
const CONSTANT_012007F4: u32 = 0x0120_07f4;
const CONSTANT_011AE5F8: u32 = 0x011a_e5f8;
const FLAG_012007F8: u32 = 0x0120_07f8;
const SCALE_011AE5E8: u32 = 0x011a_e5e8;

// Translated from 00bb89b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`; the vtable `010b9f04`, slot +0x8): the set-up of
/// `00bb8900`'s effect, `thiscall` with two stack words (`RET 8`): an owner whose word +0x8 holds
/// the textures (+0x120, +0x124), and a word the code never reads. Clears the byte +4, runs
/// `00ba3f90` with 2, sizes the array at +0x28 to 5, gets the scan-line and distortion textures
/// into `01200810` and `01200814`; texture records 3 (scan lines, then `00ba3760(3)`), 4
/// (distortion, `00ba3760(0)`) and 2 (`00ba3760(0)`, no texture); clears `01200818`. With texture
/// 0 set, its texture object's slots +0x94 and +0x98 give the sizes whose inverses times
/// `011ae5e8` are kept (0.0 without). Two shader parameters (`InitConstantMap(0, 4)` and
/// `(0, 5)`, bytes +0x55 and +0x56 set) get pixel constants 0 to 3 (the second also 4 = (1.0, 0,
/// 0, 0)), become elements 0 and 1 of the array at +0x18, and go to `00ba3c00(n, 2, 1, 3, 4, 0,
/// 0, 0, 0)` and `00ba3c70(n, texture, parameter, 0)` with the owner's textures +0x120 / +0x124.
// Translation incomplete: 00bb89b0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the parameters' allocations) is not translated.
pub fn fn_00bb89b0(e: &mut Engine, this: Ptr, owner: Ptr, _unused_2: u32) {
    let base = this.addr();
    e.mem.set_u8(base + 4, 0);
    e.call(EFFECT_SET_UP_00BA3F90, &args![base, 2u32]);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 5u32]);
    e.call(
        GET_TEXTURE,
        &args![SCANLINES_PATH_01078990, 1u32, TEXTURE_01200810, 0u32, 0u32],
    );
    let scanlines = e.global::<u32>(TEXTURE_01200810);
    e.call(
        GET_TEXTURE,
        &args![DISTORT_PATH_010B9F24, 1u32, TEXTURE_01200814, 0u32, 0u32],
    );
    let distort = e.global::<u32>(TEXTURE_01200814);
    e.with_stack(4, |e, local| {
        let local = local.addr();
        new_texture(e, base, 3, local);
        let texture = texture_at(e, base, 3);
        e.call(TEXTURE_SET_00BA37F0, &args![texture, scanlines]);
        let texture = texture_at(e, base, 3);
        e.call(TEXTURE_CALL_00BA3760, &args![texture, 3u32]);
        new_texture(e, base, 4, local);
        let texture = texture_at(e, base, 4);
        e.call(TEXTURE_SET_00BA37F0, &args![texture, distort]);
        let texture = texture_at(e, base, 4);
        e.call(TEXTURE_CALL_00BA3760, &args![texture, 0u32]);
        new_texture(e, base, 2, local);
        let texture = texture_at(e, base, 2);
        e.call(TEXTURE_CALL_00BA3760, &args![texture, 0u32]);
        let texture = texture_at(e, base, 2);
        e.call(TEXTURE_SET_00BA37F0, &args![texture, 0u32]);
    });
    ni_pointer_clear(e, TEXTURE_01200818);
    let mut width = 0.0f32.to_bits();
    let mut height = 0.0f32.to_bits();
    let first = texture_at(e, base, 0);
    if first != 0 {
        let object = e.call(IMAGE_SPACE_TEXTURE_GET_TEXTURE, &args![first]).u32();
        let scale = e.global::<u32>(SCALE_011AE5E8);
        let inverse = inverse_dimension(e, object, 0x94);
        width = x87_store32(x87_mul(inverse, x87_m32(scale)));
        let scale = e.global::<u32>(SCALE_011AE5E8);
        let inverse = inverse_dimension(e, object, 0x98);
        height = x87_store32(x87_mul(inverse, x87_m32(scale)));
    }
    let mut parameters = [0u32; 2];
    for (which, registers) in [(0usize, 4u32), (1, 5)] {
        let parameter = new_parameter(e);
        parameters[which] = parameter;
        e.call(
            SHADER_PARAM_INIT_CONSTANT_MAP,
            &args![parameter, 0u32, registers],
        );
        e.mem.set_u8(parameter + 0x55, 1);
        e.mem.set_u8(parameter + 0x56, 1);
        let words = [
            float_global(e, CONSTANT_011AE5EC),
            float_global(e, CONSTANT_012007E8),
            float_global(e, CONSTANT_011AE5F4),
            float_global(e, CONSTANT_011AE5F0),
        ];
        set_pixel_constant(e, parameter, 0, words);
        let flag = if e.global::<u8>(FLAG_012007F8) != 0 {
            1.0f32
        } else {
            0.0
        };
        let words = [
            float_global(e, CONSTANT_012007F0),
            float_global(e, CONSTANT_012007F4),
            float_global(e, CONSTANT_011AE5F8),
            flag.to_bits(),
        ];
        set_pixel_constant(e, parameter, 1, words);
        let words = [0u32, 4, 8, 0xc].map(|offset| float_global(e, WORDS_01200800 + offset));
        set_pixel_constant(e, parameter, 2, words);
        set_pixel_constant(
            e,
            parameter,
            3,
            [x87_quiet_bits(width), x87_quiet_bits(height), 0, 0],
        );
        if which == 1 {
            set_pixel_constant(e, parameter, 4, [1.0f32.to_bits(), 0, 0, 0]);
        }
    }
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for (index, parameter) in parameters.iter().enumerate() {
            e.mem.set_u32(local, *parameter);
            e.call(ARRAY_SET_00BA3AA0, &args![base + 0x18, index as u32, local]);
        }
    });
    for index in [0u32, 1] {
        e.call(
            EFFECT_CALL_00BA3C00,
            &args![base, index, 2u32, 1u32, 3u32, 4u32, 0u32, 0u32, 0u32, 0u32],
        );
    }
    for (index, offset) in [(0u32, 0x120u32), (1, 0x124)] {
        let textures = e.mem.u32(owner.addr() + 8);
        let texture = e.mem.u32(textures + offset);
        e.call(
            EFFECT_CALL_00BA3C70,
            &args![base, index, texture, parameters[index as usize], 0u32],
        );
    }
}

// Translated from 00bb8ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`): the destructor of `00bb9180`'s effect,
/// `thiscall`, no stack words. Stores the vtable `010b9f58`, clears the shared `NiPointer`s
/// `0120083c` and `01200840`, destroys the textures 1, 2 and 3 of the array at +0x28, deletes
/// the shader parameter 0 of the array at +0x18 and runs `ImageSpaceEffect::~ImageSpaceEffect`
/// (`00ba3d30`).
// Translation incomplete: 00bb8ea0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00bb8ea0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010B9F58);
    for pointer in [TEXTURE_0120083C, TEXTURE_01200840] {
        ni_pointer_clear(e, pointer);
    }
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for index in [1u32, 2, 3] {
            destroy_texture(e, base, index, local);
        }
        delete_parameter(e, base, 0, local);
    });
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

// Translated from 00bb9010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the vtable `010b9f58`, slot +0x14), `thiscall`, no stack
/// words: gives texture 1 of the array at +0x28 no texture (`00ba37f0` with 0).
pub fn fn_00bb9010(e: &mut Engine, this: Ptr) {
    let texture = texture_at(e, this.addr(), 1);
    e.call(TEXTURE_SET_00BA37F0, &args![texture, 0u32]);
}

/// The constants of `00bb9180`'s effect: `011ae600`; `0120081c`, `01200820`, `011ae604`;
/// `01200824`, `01200828`, `011ae608`; `0120082c`, `01200830`, `01200834`.
const CONSTANT_011AE600: u32 = 0x011a_e600;
const CONSTANT_0120081C: u32 = 0x0120_081c;
const CONSTANT_01200820: u32 = 0x0120_0820;
const CONSTANT_011AE604: u32 = 0x011a_e604;
const CONSTANT_01200824: u32 = 0x0120_0824;
const CONSTANT_01200828: u32 = 0x0120_0828;
const CONSTANT_011AE608: u32 = 0x011a_e608;
const CONSTANT_0120082C: u32 = 0x0120_082c;
const CONSTANT_01200830: u32 = 0x0120_0830;
const CONSTANT_01200834: u32 = 0x0120_0834;

/// Pixel constants 1 to 4 of `00bb9180`'s effect on `parameter`; answers the last call's `EAX`.
fn effect_constants(e: &mut Engine, parameter: u32) -> u32 {
    let words = [float_global(e, CONSTANT_011AE600), 0, 0, 0];
    set_pixel_constant(e, parameter, 1, words);
    let words = [
        float_global(e, CONSTANT_0120081C),
        float_global(e, CONSTANT_01200820),
        float_global(e, CONSTANT_011AE604),
        0,
    ];
    set_pixel_constant(e, parameter, 2, words);
    let words = [
        float_global(e, CONSTANT_01200824),
        float_global(e, CONSTANT_01200828),
        float_global(e, CONSTANT_011AE608),
        0,
    ];
    set_pixel_constant(e, parameter, 3, words);
    let words = [
        float_global(e, CONSTANT_0120082C),
        float_global(e, CONSTANT_01200830),
        float_global(e, CONSTANT_01200834),
        1.0f32.to_bits(),
    ];
    e.call(
        SHADER_PARAM_SET_PIXEL_CONSTANT,
        &args![parameter, 4u32, words[0], words[1], words[2], words[3]],
    )
    .u32()
}

// Translated from 00bb9020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`; the vtable `010b9f58`, slot +0x1c), `thiscall`
/// with one stack word the code never reads (`RET 4`). The inverse sizes of texture 1's texture
/// object (slots +0x94 and +0x98; 0.0 without texture 1) are pixel constant 0 of the shader
/// parameter 0 (base of the array at +0x1c), then constants 1 to 4 (`effect_constants`). Answers
/// the last call's `EAX` with `AL` cleared (`XOR AL, AL`).
pub fn fn_00bb9020(e: &mut Engine, this: Ptr, _unused_1: u32) -> u32 {
    let base = this.addr();
    let mut width = 0.0f32.to_bits();
    let mut height = 0.0f32.to_bits();
    let texture = texture_at(e, base, 1);
    if texture != 0 {
        let object = e
            .call(IMAGE_SPACE_TEXTURE_GET_TEXTURE, &args![texture])
            .u32();
        width = x87_store32(inverse_dimension(e, object, 0x94));
        height = x87_store32(inverse_dimension(e, object, 0x98));
    }
    let elements = e.mem.u32(base + 0x1c);
    let parameter = e.mem.u32(elements);
    set_pixel_constant(
        e,
        parameter,
        0,
        [x87_quiet_bits(width), x87_quiet_bits(height), 0, 0],
    );
    effect_constants(e, parameter) & 0xffff_ff00
}

// Translated from 00bb91e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`): the scalar deleting destructor of `00bb9180`'s
/// effect, `thiscall`, one stack word (the flags). Runs `00bb8ea0`; with bit 0 set, frees `this`
/// through the memory manager (`00aa4060`). Answers `this`.
pub fn fn_00bb91e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bb8ea0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00bb9200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`; the vtable `010b9f58`, slot +0x8): the set-up of
/// `00bb9180`'s effect, `thiscall` with two stack words (`RET 8`): an owner whose word +0x8 holds
/// the texture +0x134, and a word the code never reads. Runs `00ba3f90` with 1, sizes the array at
/// +0x28 to 4, gets the distortion and scan-line textures into `01200840` and `0120083c`; texture
/// records 1 (`00ba3760(0)`, no texture), 2 (`00ba3760(0)`, distortion) and 3 (`00ba3760(3)`,
/// scan lines). With texture 0 set, its texture object's slots +0x94 and +0x98 give the inverse
/// sizes (0.0 without). A shader parameter (`InitConstantMap(0, 5)`, bytes +0x55 and +0x56
/// cleared) gets pixel constant 0 (the inverse sizes, 0.0, 0.0) and constants 1 to 4, becomes
/// element 0 of the array at +0x18, then `00ba3ba0(0, 1, 2, 3, 0, 0, 0)` and `00ba3c70(0, the
/// owner's texture +0x134, parameter, 0)`.
// Translation incomplete: 00bb9200 - the C++ exception frame (FS:[0] chain, unwind state 0 for the parameter's allocation) is not translated.
pub fn fn_00bb9200(e: &mut Engine, this: Ptr, owner: Ptr, _unused_2: u32) {
    let base = this.addr();
    e.call(EFFECT_SET_UP_00BA3F90, &args![base, 1u32]);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 4u32]);
    e.call(
        GET_TEXTURE,
        &args![DISTORT_PATH_010B9F24, 1u32, TEXTURE_01200840, 0u32, 0u32],
    );
    e.call(
        GET_TEXTURE,
        &args![SCANLINES_PATH_01078990, 1u32, TEXTURE_0120083C, 0u32, 0u32],
    );
    let scanlines = e.global::<u32>(TEXTURE_0120083C);
    let distort = e.global::<u32>(TEXTURE_01200840);
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for (index, mode, texture) in [(1u32, 0u32, 0u32), (2, 0, distort), (3, 3, scanlines)] {
            new_texture(e, base, index, local);
            let record = texture_at(e, base, index);
            e.call(TEXTURE_CALL_00BA3760, &args![record, mode]);
            let record = texture_at(e, base, index);
            e.call(TEXTURE_SET_00BA37F0, &args![record, texture]);
        }
    });
    let mut width = 0.0f32.to_bits();
    let mut height = 0.0f32.to_bits();
    let first = texture_at(e, base, 0);
    if first != 0 {
        let object = e.call(IMAGE_SPACE_TEXTURE_GET_TEXTURE, &args![first]).u32();
        width = x87_store32(inverse_dimension(e, object, 0x94));
        height = x87_store32(inverse_dimension(e, object, 0x98));
    }
    let parameter = new_parameter(e);
    e.call(
        SHADER_PARAM_INIT_CONSTANT_MAP,
        &args![parameter, 0u32, 5u32],
    );
    e.mem.set_u8(parameter + 0x55, 0);
    e.mem.set_u8(parameter + 0x56, 0);
    set_pixel_constant(
        e,
        parameter,
        0,
        [x87_quiet_bits(width), x87_quiet_bits(height), 0, 0],
    );
    effect_constants(e, parameter);
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), parameter);
        e.call(ARRAY_SET_00BA3AA0, &args![base + 0x18, 0u32, local.addr()]);
    });
    e.call(
        EFFECT_CALL_00BA3BA0,
        &args![base, 0u32, 1u32, 2u32, 3u32, 0u32, 0u32, 0u32],
    );
    let textures = e.mem.u32(owner.addr() + 8);
    let texture = e.mem.u32(textures + 0x134);
    e.call(
        EFFECT_CALL_00BA3C70,
        &args![base, 0u32, texture, parameter, 0u32],
    );
}

// Translated from 00bb9540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`; the lighting 3.0 shader's vtable `010ba0f8`, slot
/// +0x148), `thiscall`, no stack words: calls its own slot +0x14c, then (a tail jump) slot
/// +0x150, and answers what that one answers in `EAX`.
pub fn fn_00bb9540(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    e.vcall(base, 0x14c, &args![]);
    e.vcall(base, 0x150, &args![]).u32()
}

/// The 0x70 pass pointers `00bb9560` fills (`01200ec0` onwards) and the call after them
/// (`00c072c0`, unnamed; `thiscall`).
const PASSES_01200EC0: u32 = 0x0120_0ec0;
const CALL_00C072C0: u32 = 0x00c0_72c0;

// Translated from 00bb9560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`; the vtable `010ba0f8`, slot +0x154), `thiscall`,
/// no stack words; answers true. Gives each of the 0x70 pass pointers from `01200ec0` a new pass
/// (`00e804d0`, no null test), then runs `00c072c0` on `this`.
pub fn fn_00bb9560(e: &mut Engine, this: Ptr) -> bool {
    for index in 0..0x70u32 {
        e.call(NEW_PASS_00E804D0, &args![PASSES_01200EC0 + 4 * index]);
    }
    e.call(CALL_00C072C0, &args![this]);
    true
}

/// The table `00bb95e0` writes: 0x20 bytes per entry, the second four words at `01201c90`, the
/// first four at `01201c80`.
const LIGHT_TABLE_01201C80: u32 = 0x0120_1c80;

// Translated from 00bb95e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`), `cdecl` with nine words: an index and two groups
/// of four words (`float`s, copied as words). Entry `index` of the 0x20-byte table at `01201c80`
/// gets the first group in its second half (+0x10 to +0x1c) and the second group in its first
/// half (+0 to +0xc).
pub fn fn_00bb95e0(
    e: &mut Engine,
    index: u32,
    first_0: u32,
    first_1: u32,
    first_2: u32,
    first_3: u32,
    second_0: u32,
    second_1: u32,
    second_2: u32,
    second_3: u32,
) {
    let entry = LIGHT_TABLE_01201C80.wrapping_add(index.wrapping_shl(5));
    for (offset, word) in [
        (0x10u32, first_0),
        (0x14, first_1),
        (0x18, first_2),
        (0x1c, first_3),
        (0, second_0),
        (4, second_1),
        (8, second_2),
        (0xc, second_3),
    ] {
        e.mem.set_u32(entry.wrapping_add(offset), word);
    }
}

/// The texture stage call `00e7de90` (unnamed; `thiscall` on the stage, one word).
const STAGE_CALL_00E7DE90: u32 = 0x00e7_de90;

// Translated from 00bb9640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`), `thiscall` (its callers load ECX; `this` is not
/// read) with two words (`RET 8`): a texture stage and an object. Passes the object's slot
/// +0xe0 answer to `00e7de90` on the stage.
pub fn fn_00bb9640(e: &mut Engine, _this: Ptr, stage: u32, object: Ptr) {
    let value = e.vcall(object.addr(), 0xe0, &args![]).u32();
    e.call(STAGE_CALL_00E7DE90, &args![stage, value]);
}

/// The byte `00bb96d0` clears (`01200863`) and the object whose byte +0x8 it clears
/// (`012010bc`).
const FLAG_01200863: u32 = 0x0120_0863;
const HOLDER_012010BC: u32 = 0x0120_10bc;

// Translated from 00bb96d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bb8250`), `thiscall` with one word the code never reads
/// (`RET 4`; its caller `RenderPassImmediately` pushes one): clears the byte `01200863` and the
/// byte +0x8 of the object at `012010bc`.
/// `this` (`ECX`, the current shader `011ffe2c`) is not read, but `RenderPassImmediately`
/// calls it with `ECX` set.
pub fn fn_00bb96d0(e: &mut Engine, _this: Ptr, _unused_0: u32) {
    let holder = e.global::<u32>(HOLDER_012010BC);
    e.set_global::<u8>(FLAG_01200863, 0);
    e.mem.set_u8(holder + 8, 0);
}

/// The Ni allocation (`00aa13e0`, `cdecl`), `BSFogProperty::BSFogProperty` (`00bb8180`) and the
/// copy `00a99e00` (unit `niproperty.cpp` range, unnamed; `thiscall`, two words).
const NI_ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const FOG_PROPERTY_CONSTRUCT: u32 = 0x00bb_8180;
const PROPERTY_COPY_00A99E00: u32 = 0x00a9_9e00;

// Translated from 00bb8280 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bb8280 - the C++ exception frame (FS:[0] chain, unwind state 0 for the allocation) is not translated.
/// Unnamed in the engine map (vtable entry at `010b9e80`), `thiscall` with one word: the fog
/// property's clone. Allocates 100 bytes (`00aa13e0`), runs `BSFogProperty::BSFogProperty` on
/// them when that succeeds, and copies `this` into the result (`00a99e00(result, word)`, a null
/// result passed as it is). Answers the result.
pub fn fn_00bb8280(e: &mut Engine, this: Ptr, word: u32) -> u32 {
    let block = e.call(NI_ALLOCATE_00AA13E0, &args![0x64u32]).u32();
    let made = if block != 0 {
        e.call(FOG_PROPERTY_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(PROPERTY_COPY_00A99E00, &args![this, made, word]);
    made
}

use crate::units::bsshader::bsshaderproperty::{
    viewer_strings_add, VIEWER_STRING, VIEWER_STRING_FLOAT,
};

/// `NiProperty`'s viewer strings `00a9a2a0` (unnamed, `thiscall`, one word), the fog property's
/// RTTI record (`012007e0`) and the labels of its two `float`s (`010b9ee0`, `010b9ecc`).
const PROPERTY_VIEWER_STRINGS_00A9A2A0: u32 = 0x00a9_a2a0;
const FOG_RTTI_012007E0: u32 = 0x0120_07e0;
const LABEL_010B9EE0: u32 = 0x010b_9ee0;
const LABEL_010B9ECC: u32 = 0x010b_9ecc;

// Translated from 00bb8300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010b9e98`): the fog
/// property's `GetViewerStrings`, `thiscall` with one word, the string array. Runs `00a9a2a0`,
/// then adds (the inlined `NiTArray` add) the class name (`00a7d330` of `012007e0`) and the
/// `float`s +0x2c and +0x30 (through `FLD`/`FSTP`) with their labels (`00a7d550`).
pub fn fn_00bb8300(e: &mut Engine, this: Ptr, strings: u32) {
    let base = this.addr();
    e.call(PROPERTY_VIEWER_STRINGS_00A9A2A0, &args![base, strings]);
    let rtti = e.global::<u32>(FOG_RTTI_012007E0);
    let string = e.call(VIEWER_STRING, &args![rtti]).u32();
    viewer_strings_add(e, strings, string);
    for (offset, label) in [(0x2cu32, LABEL_010B9EE0), (0x30, LABEL_010B9ECC)] {
        let value = x87_quiet_bits(e.mem.u32(base + offset));
        let string = e.call(VIEWER_STRING_FLOAT, &args![label, value]).u32();
        viewer_strings_add(e, strings, string);
    }
}

// Translated from 00bb8590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable `010b9f04` entry at `010b9f18`),
/// `thiscall`, no stack words: releases and clears the shared `NiPointer` `01200818`, then
/// gives each of the textures 0 to 4 of the array at +0x28 that is set no texture
/// (`00ba37f0` with 0).
pub fn fn_00bb8590(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    ni_pointer_clear(e, TEXTURE_01200818);
    for index in 0..5 {
        let texture = texture_at(e, base, index);
        if texture != 0 {
            e.call(TEXTURE_SET_00BA37F0, &args![texture, 0u32]);
        }
    }
}

/// The values `00bb85e0` adds to `00bb89b0`'s constants: `011ae5fc` (constant 4 of the second
/// parameter) and the word `012007ec` it stores at the parameter's +0x28; the pass chooser
/// `012007fc`; `00d47a40` (unnamed, `thiscall`, one word).
const CONSTANT_011AE5FC: u32 = 0x011a_e5fc;
const PARAMETER_VALUE_012007EC: u32 = 0x0120_07ec;
const PASS_012007FC: u32 = 0x0120_07fc;
const TEXTURE_CALL_00D47A40: u32 = 0x00d4_7a40;

// Translated from 00bb85e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable `010b9f04` entry at `010b9f20`),
/// `thiscall` with one unread word; answers false. Textures 3 and 4 get `01200810` and
/// `01200814`, texture 2 `01200818` (or, when that is null, what `ImageSpaceTexture::
/// GetTexture` gives for texture 1). With texture 2 set, its texture object's slots +0x94 and
/// +0x98 give the inverse sizes times `011ae5e8` (0.0 otherwise). The parameter `012007fc`
/// chooses (0: the first of the array at +0x18, else the second) gets the pixel constants 0 (
/// `011ae5ec`, `012007e8`, `011ae5f4`, `011ae5f0`), 1 (`012007f0`, `012007f4`, `011ae5f8`, the
/// byte `012007f8` as 1.0 or 0.0), 2 (`01200800`..`0120080c`) and 3 (the inverse sizes, then 0,
/// 0), the second also 4 (`011ae5fc`, 0, 0, 0); its +0x28 takes `012007ec`. Then
/// `00d47a40(1)` on each of the first `u16` +0x32 textures.
pub fn fn_00bb85e0(e: &mut Engine, this: Ptr, _unused_0: u32) -> bool {
    let base = this.addr();
    let value = e.global::<u32>(TEXTURE_01200810);
    let texture = texture_at(e, base, 3);
    e.call(TEXTURE_SET_00BA37F0, &args![texture, value]);
    let value = e.global::<u32>(TEXTURE_01200814);
    let texture = texture_at(e, base, 4);
    e.call(TEXTURE_SET_00BA37F0, &args![texture, value]);
    let shared = e.global::<u32>(TEXTURE_01200818);
    let value = if shared != 0 {
        shared
    } else {
        let texture = texture_at(e, base, 1);
        e.call(IMAGE_SPACE_TEXTURE_GET_TEXTURE, &args![texture])
            .u32()
    };
    let texture = texture_at(e, base, 2);
    e.call(TEXTURE_SET_00BA37F0, &args![texture, value]);
    let mut width = 0.0f32.to_bits();
    let mut height = 0.0f32.to_bits();
    let second = texture_at(e, base, 2);
    if second != 0 {
        let object = e
            .call(IMAGE_SPACE_TEXTURE_GET_TEXTURE, &args![second])
            .u32();
        let inverse = inverse_dimension(e, object, 0x94);
        let scale = e.global::<u32>(SCALE_011AE5E8);
        width = x87_store32(x87_mul(inverse, x87_m32(scale)));
        let inverse = inverse_dimension(e, object, 0x98);
        let scale = e.global::<u32>(SCALE_011AE5E8);
        height = x87_store32(x87_mul(inverse, x87_m32(scale)));
    }
    let first_words = [
        float_global(e, CONSTANT_011AE5EC),
        float_global(e, CONSTANT_012007E8),
        float_global(e, CONSTANT_011AE5F4),
        float_global(e, CONSTANT_011AE5F0),
    ];
    let parameters = e.mem.u32(base + 0x1c);
    let second_pass = e.global::<u32>(PASS_012007FC) != 0;
    let parameter = e.mem.u32(parameters + if second_pass { 4 } else { 0 });
    set_pixel_constant(e, parameter, 0, first_words);
    let flag = if e.global::<u8>(FLAG_012007F8) != 0 {
        1.0f32
    } else {
        0.0
    };
    let words = [
        float_global(e, CONSTANT_012007F0),
        float_global(e, CONSTANT_012007F4),
        float_global(e, CONSTANT_011AE5F8),
        flag.to_bits(),
    ];
    set_pixel_constant(e, parameter, 1, words);
    let words = [0u32, 4, 8, 0xc].map(|offset| float_global(e, WORDS_01200800 + offset));
    set_pixel_constant(e, parameter, 2, words);
    set_pixel_constant(
        e,
        parameter,
        3,
        [x87_quiet_bits(width), x87_quiet_bits(height), 0, 0],
    );
    if second_pass {
        let words = [float_global(e, CONSTANT_011AE5FC), 0, 0, 0];
        set_pixel_constant(e, parameter, 4, words);
    }
    let word = e.global::<u32>(PARAMETER_VALUE_012007EC);
    e.mem.set_u32(parameter + 0x28, word);
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(base + 0x32)) {
        let texture = texture_at(e, base, index);
        e.call(TEXTURE_CALL_00D47A40, &args![texture, 1u32]);
        index += 1;
    }
    false
}

/// `00ba3e30` (unit `imagespaceeffect.cpp` range, unnamed; `thiscall`, one word).
const EFFECT_CALL_00BA3E30: u32 = 0x00ba_3e30;

// Translated from 00bb8890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable `010b9f04` entry at `010b9f08`), `thiscall` with four words
/// (the third unread). Calls the effect's vtable +0x10 with the fourth word; takes parameter
/// `012007fc` of the array at +0x18 (elements at +0x1c), runs `00ba3e30(012007fc)` and the
/// vtable +0x1c with the parameter; calls slot +4 of the second (`012007fc` 1) or first object of
/// the array at +0xc with (first word, second word, parameter, fourth word); then the vtable
/// +0x14.
pub fn fn_00bb8890(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    _unused_2: u32,
    fourth: u32,
) {
    let base = this.addr();
    e.vcall(base, 0x10, &args![fourth]);
    let index = e.global::<u32>(PASS_012007FC);
    let parameters = e.mem.u32(base + 0x1c);
    let parameter = e.mem.u32(parameters.wrapping_add(index.wrapping_mul(4)));
    e.call(EFFECT_CALL_00BA3E30, &args![base, index]);
    e.vcall(base, 0x1c, &args![parameter]);
    let objects = e.mem.u32(base + 0xc);
    let object = if e.global::<u32>(PASS_012007FC) == 1 {
        e.mem.u32(objects + 4)
    } else {
        e.mem.u32(objects)
    };
    e.vcall(object, 4, &args![first, second, parameter, fourth]);
    e.vcall(base, 0x14, &args![]);
}

/// `00c072c0` (unit `imagespacemanager.cpp` range, unnamed; `thiscall`, no stack words).
const MANAGER_CALL_00C072C0: u32 = 0x00c0_72c0;

// Translated from 00bb9590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable entry at `010ba218`), `thiscall`, no stack words: calls the
/// vtable +0x148, then jumps to `00c072c0`.
pub fn fn_00bb9590(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0x148, &args![]);
    e.call(MANAGER_CALL_00C072C0, &args![this]);
}

/// The table of 0x20-byte records `00bb95b0` fills (`01201c80`).
const RECORDS_01201C80: u32 = 0x0120_1c80;

// Translated from 00bb95b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `cdecl` with five words (its callers clean 0x14 bytes): stores the
/// last four words as the first four words of record `index` of the table at `01201c80`.
pub fn fn_00bb95b0(e: &mut Engine, index: u32, a: u32, b: u32, c: u32, d: u32) {
    let record = (index << 5).wrapping_add(RECORDS_01201C80);
    for (offset, word) in [(0u32, a), (4, b), (8, c), (0xc, d)] {
        e.mem.set_u32(record.wrapping_add(offset), word);
    }
}

/// The object tables `00bb9660` walks: `01200dd8` (0x39 words), `01200c48` and `01200d10`
/// (0x32 words each); and `00e814b0` (unnamed, `thiscall`, no stack words) it jumps to.
const OBJECTS_01200DD8: u32 = 0x0120_0dd8;
const OBJECTS_01200C48: u32 = 0x0120_0c48;
const OBJECTS_01200D10: u32 = 0x0120_0d10;
const RENDERER_CALL_00E814B0: u32 = 0x00e8_14b0;

// Translated from 00bb9660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable entry at `010ba1c8`), `thiscall`, no stack words. With the
/// word +0x28 set, calls the vtable +0xa0 of each object of the table `01200dd8` (0xe4 bytes);
/// then the vtable +0x88 of each object of `01200c48` and, in the same step, of `01200d10` (200
/// bytes each); then jumps to `00e814b0`.
pub fn fn_00bb9660(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x28) != 0 {
        for offset in (0..0xe4u32).step_by(4) {
            let object = e.global::<u32>(OBJECTS_01200DD8 + offset);
            if object != 0 {
                e.vcall(object, 0xa0, &args![]);
            }
        }
    }
    for offset in (0..200u32).step_by(4) {
        let object = e.global::<u32>(OBJECTS_01200C48 + offset);
        if object != 0 {
            e.vcall(object, 0x88, &args![]);
        }
        let object = e.global::<u32>(OBJECTS_01200D10 + offset);
        if object != 0 {
            e.vcall(object, 0x88, &args![]);
        }
    }
    e.call(RENDERER_CALL_00E814B0, &args![base]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bb_8280, fn_00bb8280(Ptr, u32) -> u32),
        entry!(0x00bb_8300, fn_00bb8300(Ptr, u32)),
        entry!(0x00bb_8590, fn_00bb8590(Ptr)),
        entry!(0x00bb_85e0, fn_00bb85e0(Ptr, u32) -> bool),
        entry!(0x00bb_8890, fn_00bb8890(Ptr, u32, u32, u32, u32)),
        entry!(0x00bb_9590, fn_00bb9590(Ptr)),
        entry!(0x00bb_95b0, fn_00bb95b0(u32, u32, u32, u32, u32)),
        entry!(0x00bb_9660, fn_00bb9660(Ptr)),
        entry!(0x00bb_8900, fn_00bb8900(Ptr) -> Ptr),
        entry!(0x00bb_9180, fn_00bb9180(Ptr) -> Ptr),
        entry!(0x00bb_8250, fn_00bb8250(Ptr, u32) -> Ptr),
        entry!(0x00bb_83d0, fn_00bb83d0(Ptr)),
        entry!(0x00bb_8990, fn_00bb8990(Ptr, u32) -> Ptr),
        entry!(0x00bb_89b0, fn_00bb89b0(Ptr, Ptr, u32)),
        entry!(0x00bb_8ea0, fn_00bb8ea0(Ptr)),
        entry!(0x00bb_9010, fn_00bb9010(Ptr)),
        entry!(0x00bb_9020, fn_00bb9020(Ptr, u32) -> u32),
        entry!(0x00bb_91e0, fn_00bb91e0(Ptr, u32) -> Ptr),
        entry!(0x00bb_9200, fn_00bb9200(Ptr, Ptr, u32)),
        entry!(0x00bb_9540, fn_00bb9540(Ptr) -> u32),
        entry!(0x00bb_9560, fn_00bb9560(Ptr) -> bool),
        entry!(
            0x00bb_95e0,
            fn_00bb95e0(u32, u32, u32, u32, u32, u32, u32, u32, u32)
        ),
        entry!(0x00bb_9640, fn_00bb9640(Ptr, u32, Ptr)),
        entry!(0x00bb_96d0, fn_00bb96d0(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.register(ARRAY_SET_SIZE_00A90180, |_, a| a[0].into_ret());
        e.map(0x011a_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00bb8900_builds_the_effect_and_copies_the_words() {
        let mut e = engine();
        for index in 0..4 {
            e.set_global(WORDS_011A9BE0 + 4 * index, 0x0e00_0010 + index);
        }
        let this = e.mem.alloc(0x58);
        e.mem.write(this, &[0xcc; 8]);
        assert_eq!(e.call(0x00bb_8900, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B9F04);
        assert_eq!(e.mem.u8(this + 4), 0);
        assert_eq!(e.mem.u8(this + 5), 0xcc);
        for index in 0..4 {
            assert_eq!(
                e.global::<u32>(WORDS_01200800 + 4 * index),
                0x0e00_0010 + index
            );
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(log[2], (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 5]));
    }

    #[test]
    fn fn_00bb9180_builds_the_effect() {
        let mut e = engine();
        let this = e.mem.alloc(0x58);
        assert_eq!(e.call(0x00bb_9180, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B9F58);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 4]),
            ]
        );
    }
}

#[cfg(test)]
mod b056_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    /// [`rig`] with the effect array store as a real store, the texture object doubles (the
    /// texture's object answers 0x200 and 0x80000000 for slots +0x94 and +0x98) and no-op
    /// doubles for the rest.
    fn engine() -> Engine {
        let mut e = rig();
        for page in [0x0120_0000u32, 0x0120_1000, 0x011a_e000, 0x0109_9000] {
            e.map(page, 0x1000);
        }
        e.register(ARRAY_SET_00BA3AA0, |e, a| {
            let word = e.mem.u32(a[2]);
            let elements = e.mem.u32(a[0] + 4);
            e.mem.set_u32(elements + 4 * a[1], word);
            Ret::default()
        });
        for target in [
            BASE_DESTRUCT_00A5C240,
            NI_FREE_SIZED_00AA1460,
            MEMORY_DEALLOCATE,
            TEXTURE_DESTRUCT_00BA3A00,
            EFFECT_DESTRUCT_00BA3D30,
            EFFECT_SET_UP_00BA3F90,
            ARRAY_SET_SIZE_00A90180,
            GET_TEXTURE,
            TEXTURE_SET_00BA37F0,
            TEXTURE_CALL_00BA3760,
            SHADER_PARAM_INIT_CONSTANT_MAP,
            SHADER_PARAM_SET_PIXEL_CONSTANT,
            EFFECT_CALL_00BA3BA0,
            EFFECT_CALL_00BA3C00,
            EFFECT_CALL_00BA3C70,
            CALL_00C072C0,
            STAGE_CALL_00E7DE90,
            NEW_PASS_00E804D0,
            0x0e9f_0094,
            0x0e9f_0098,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(SHADER_PARAM_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(0x0e9f_0094, |_, _| 0x200u32.into_ret());
        e.register(0x0e9f_0098, |_, _| 0x8000_0000u32.into_ret());
        let slots: Vec<u32> = (0..0x60u32).map(|i| 0x0e9f_0000 + 4 * i).collect();
        e.put_vtable(0x0e9e_0000, &slots);
        e.register(IMAGE_SPACE_TEXTURE_GET_TEXTURE, |e, _| {
            let object = e.mem.alloc(4);
            e.mem.set_u32(object, 0x0e9e_0000);
            object.into_ret()
        });
        e.set_global::<f32>(TWO_TO_32_01099378, 4_294_967_296.0);
        e
    }

    /// An effect with the arrays at +0x18 (base +0x1c) and +0x28 (base +0x2c), five slots each.
    fn effect(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x58);
        let first = e.mem.alloc(0x14);
        let second = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x1c, first);
        e.mem.set_u32(this + 0x2c, second);
        this
    }

    fn parameter_at(e: &Engine, this: u32, index: u32) -> u32 {
        let elements = e.mem.u32(this + 0x1c);
        e.mem.u32(elements + 4 * index)
    }

    #[test]
    fn fn_00bb8250_stores_the_vtable_and_frees_with_bit_0() {
        let mut e = engine();
        let this = e.mem.alloc(0x64);
        assert_eq!(e.call(0x00bb_8250, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B9E38);
        assert_eq!(calls(&e, BASE_DESTRUCT_00A5C240), vec![vec![this]]);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460), vec![vec![this, 0x64]]);
        e.call(0x00bb_8250, &args![this, 0u32]);
        assert_eq!(calls(&e, NI_FREE_SIZED_00AA1460).len(), 1);
    }

    #[test]
    fn fn_00bb83d0_releases_the_first_effect_and_keeps_the_memory() {
        let mut e = engine();
        e.register(0x0e74_0000, |_, _| Ret::default());
        e.put_vtable(0x0e75_0000, &[0x0e74_0000]);
        let shared = object(&mut e, 1);
        e.set_global::<u32>(TEXTURE_01200814, shared);
        let this = effect(&mut e);
        let parameter = e.mem.alloc(8);
        e.mem.set_u32(parameter, 0x0e75_0000);
        let parameters = e.mem.u32(this + 0x1c);
        e.mem.set_u32(parameters + 4, parameter);
        e.call(0x00bb_83d0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010B9F04);
        assert_eq!(e.global::<u32>(TEXTURE_01200814), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![shared]]);
        assert_eq!(calls(&e, 0x0e74_0000), vec![vec![parameter, 1]]);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        assert!(!calls(&e, MEMORY_DEALLOCATE).contains(&vec![MEMORY_MANAGER, this]));
    }

    #[test]
    fn fn_00bb8ea0_releases_the_second_effect_and_keeps_the_memory() {
        let mut e = engine();
        e.register(0x0e74_0000, |_, _| Ret::default());
        e.put_vtable(0x0e75_0000, &[0x0e74_0000]);
        let this = effect(&mut e);
        let parameter = e.mem.alloc(8);
        e.mem.set_u32(parameter, 0x0e75_0000);
        let parameters = e.mem.u32(this + 0x1c);
        e.mem.set_u32(parameters, parameter);
        e.call(0x00bb_8ea0, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010B9F58);
        assert_eq!(calls(&e, 0x0e74_0000), vec![vec![parameter, 1]]);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        assert!(!calls(&e, MEMORY_DEALLOCATE).contains(&vec![MEMORY_MANAGER, this]));
    }

    #[test]
    fn fn_00bb8990_destroys_the_first_effect() {
        let mut e = engine();
        e.register(0x0e74_0000, |_, _| Ret::default());
        e.put_vtable(0x0e75_0000, &[0x0e74_0000]);
        let shared = object(&mut e, 1);
        e.set_global::<u32>(TEXTURE_01200814, shared);
        let this = effect(&mut e);
        let parameter = e.mem.alloc(8);
        e.mem.set_u32(parameter, 0x0e75_0000);
        let parameters = e.mem.u32(this + 0x1c);
        e.mem.set_u32(parameters + 4, parameter);
        let textures = e.mem.u32(this + 0x2c);
        e.mem.set_u32(textures + 0x10, 0x0e56_0000);
        assert_eq!(e.call(0x00bb_8990, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B9F04);
        assert_eq!(e.global::<u32>(TEXTURE_01200814), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![shared]]);
        assert_eq!(calls(&e, 0x0e74_0000), vec![vec![parameter, 1]]);
        assert_eq!(
            calls(&e, TEXTURE_DESTRUCT_00BA3A00),
            vec![vec![0x0e56_0000]]
        );
        let cleared: Vec<u32> = calls(&e, ARRAY_SET_00BA3AA0).iter().map(|a| a[1]).collect();
        assert_eq!(cleared, vec![1, 4]);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![
                vec![MEMORY_MANAGER, 0x0e56_0000],
                vec![MEMORY_MANAGER, this]
            ]
        );
    }

    #[test]
    fn fn_00bb91e0_destroys_the_second_effect() {
        let mut e = engine();
        e.register(0x0e74_0000, |_, _| Ret::default());
        e.put_vtable(0x0e75_0000, &[0x0e74_0000]);
        let this = effect(&mut e);
        let parameter = e.mem.alloc(8);
        e.mem.set_u32(parameter, 0x0e75_0000);
        let parameters = e.mem.u32(this + 0x1c);
        e.mem.set_u32(parameters, parameter);
        let textures = e.mem.u32(this + 0x2c);
        for index in 1..4u32 {
            e.mem.set_u32(textures + 4 * index, 0x0e56_0000 + index);
        }
        e.call(0x00bb_91e0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this), VTABLE_010B9F58);
        let cleared: Vec<u32> = calls(&e, ARRAY_SET_00BA3AA0).iter().map(|a| a[1]).collect();
        assert_eq!(cleared, vec![1, 2, 3, 0]);
        assert_eq!(calls(&e, 0x0e74_0000), vec![vec![parameter, 1]]);
        assert_eq!(calls(&e, MEMORY_DEALLOCATE).len(), 3);
    }

    #[test]
    fn fn_00bb89b0_sets_up_the_textures_and_two_parameters() {
        let mut e = engine();
        e.set_global::<u32>(TEXTURE_01200810, 0x0e60_0000);
        e.set_global::<u32>(TEXTURE_01200814, 0x0e61_0000);
        let shared = object(&mut e, 1);
        e.set_global::<u32>(TEXTURE_01200818, shared);
        e.set_global::<f32>(SCALE_011AE5E8, 2.0);
        e.set_global::<u8>(FLAG_012007F8, 1);
        e.set_global::<f32>(CONSTANT_011AE5EC, 5.0);
        let this = effect(&mut e);
        let textures = e.mem.u32(this + 0x2c);
        e.mem.set_u32(textures, 0x0e70_0000);
        e.mem.set_u8(this + 4, 9);
        let owner = e.mem.alloc(0xc);
        let held = e.mem.alloc(0x128);
        e.mem.set_u32(owner + 8, held);
        e.mem.set_u32(held + 0x120, 0x0e71_0120);
        e.mem.set_u32(held + 0x124, 0x0e71_0124);
        e.call(0x00bb_89b0, &args![this, owner, 0u32]);
        assert_eq!(e.mem.u8(this + 4), 0);
        assert_eq!(calls(&e, EFFECT_SET_UP_00BA3F90), vec![vec![this, 2]]);
        assert_eq!(e.global::<u32>(TEXTURE_01200818), 0);
        let (t3, t4, t2) = (
            texture_at(&e, this, 3),
            texture_at(&e, this, 4),
            texture_at(&e, this, 2),
        );
        assert_eq!(
            calls(&e, TEXTURE_SET_00BA37F0),
            vec![vec![t3, 0x0e60_0000], vec![t4, 0x0e61_0000], vec![t2, 0]]
        );
        assert_eq!(
            calls(&e, TEXTURE_CALL_00BA3760),
            vec![vec![t3, 3], vec![t4, 0], vec![t2, 0]]
        );
        let (p0, p1) = (parameter_at(&e, this, 0), parameter_at(&e, this, 1));
        assert_eq!(e.mem.u8(p0 + 0x55), 1);
        assert_eq!(e.mem.u8(p1 + 0x56), 1);
        let set = calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT);
        assert_eq!(set.len(), 9);
        assert_eq!(set[0][..3], [p0, 0, 5.0f32.to_bits()]);
        assert_eq!(set[1][5], 1.0f32.to_bits());
        // 2 / 0x200 and 2 / 2^31.
        assert_eq!(
            set[3],
            vec![
                p0,
                3,
                (1.0f32 / 256.0).to_bits(),
                (1.0f32 / 1_073_741_824.0).to_bits(),
                0,
                0
            ]
        );
        assert_eq!(set[8], vec![p1, 4, 1.0f32.to_bits(), 0, 0, 0]);
        assert_eq!(
            calls(&e, SHADER_PARAM_INIT_CONSTANT_MAP),
            vec![vec![p0, 0, 4], vec![p1, 0, 5]]
        );
        assert_eq!(
            calls(&e, EFFECT_CALL_00BA3C00),
            vec![
                vec![this, 0, 2, 1, 3, 4, 0, 0, 0, 0],
                vec![this, 1, 2, 1, 3, 4, 0, 0, 0, 0]
            ]
        );
        assert_eq!(
            calls(&e, EFFECT_CALL_00BA3C70),
            vec![
                vec![this, 0, 0x0e71_0120, p0, 0],
                vec![this, 1, 0x0e71_0124, p1, 0]
            ]
        );
    }

    #[test]
    fn fn_00bb89b0_without_texture_0_or_the_flag() {
        let mut e = engine();
        let this = effect(&mut e);
        let owner = e.mem.alloc(0xc);
        let held = e.mem.alloc(0x128);
        e.mem.set_u32(owner + 8, held);
        e.call(0x00bb_89b0, &args![this, owner, 0u32]);
        let set = calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT);
        assert_eq!(set[1][5], 0);
        assert_eq!(set[3][2..], [0, 0, 0, 0]);
        assert!(calls(&e, IMAGE_SPACE_TEXTURE_GET_TEXTURE).is_empty());
    }

    #[test]
    fn fn_00bb9010_clears_texture_1() {
        let mut e = engine();
        let this = effect(&mut e);
        let textures = e.mem.u32(this + 0x2c);
        e.mem.set_u32(textures + 4, 0x0e56_0001);
        e.call(0x00bb_9010, &args![this]);
        assert_eq!(calls(&e, TEXTURE_SET_00BA37F0), vec![vec![0x0e56_0001, 0]]);
    }

    #[test]
    fn fn_00bb9020_sets_the_constants() {
        let mut e = engine();
        e.register(SHADER_PARAM_SET_PIXEL_CONSTANT, |_, _| {
            0x1234_5601u32.into_ret()
        });
        e.set_global::<f32>(CONSTANT_011AE600, 3.0);
        e.set_global::<f32>(CONSTANT_01200834, 4.0);
        let this = effect(&mut e);
        let parameters = e.mem.u32(this + 0x1c);
        e.mem.set_u32(parameters, 0x0e80_0000);
        let textures = e.mem.u32(this + 0x2c);
        e.mem.set_u32(textures + 4, 0x0e56_0001);
        assert_eq!(e.call(0x00bb_9020, &args![this, 0u32]).u32(), 0x1234_5600);
        let p = 0x0e80_0000;
        assert_eq!(
            calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT),
            vec![
                vec![
                    p,
                    0,
                    (1.0f32 / 512.0).to_bits(),
                    (1.0f32 / 2_147_483_648.0).to_bits(),
                    0,
                    0
                ],
                vec![p, 1, 3.0f32.to_bits(), 0, 0, 0],
                vec![p, 2, 0, 0, 0, 0],
                vec![p, 3, 0, 0, 0, 0],
                vec![p, 4, 0, 0, 4.0f32.to_bits(), 1.0f32.to_bits()],
            ]
        );
        // Without texture 1 the first constant is zero.
        e.mem.set_u32(textures + 4, 0);
        e.call(0x00bb_9020, &args![this, 0u32]);
        assert_eq!(
            calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT)[5],
            vec![p, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn fn_00bb9200_sets_up_the_second_effect() {
        let mut e = engine();
        e.set_global::<u32>(TEXTURE_0120083C, 0x0e60_0000);
        e.set_global::<u32>(TEXTURE_01200840, 0x0e61_0000);
        let this = effect(&mut e);
        let textures = e.mem.u32(this + 0x2c);
        e.mem.set_u32(textures, 0x0e70_0000);
        let owner = e.mem.alloc(0xc);
        let held = e.mem.alloc(0x138);
        e.mem.set_u32(owner + 8, held);
        e.mem.set_u32(held + 0x134, 0x0e71_0134);
        e.call(0x00bb_9200, &args![this, owner, 0u32]);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + 0x28, 4]]
        );
        assert_eq!(
            calls(&e, GET_TEXTURE),
            vec![
                vec![DISTORT_PATH_010B9F24, 1, TEXTURE_01200840, 0, 0],
                vec![SCANLINES_PATH_01078990, 1, TEXTURE_0120083C, 0, 0]
            ]
        );
        let (t1, t2, t3) = (
            texture_at(&e, this, 1),
            texture_at(&e, this, 2),
            texture_at(&e, this, 3),
        );
        assert_eq!(
            calls(&e, TEXTURE_SET_00BA37F0),
            vec![vec![t1, 0], vec![t2, 0x0e61_0000], vec![t3, 0x0e60_0000]]
        );
        assert_eq!(
            calls(&e, TEXTURE_CALL_00BA3760),
            vec![vec![t1, 0], vec![t2, 0], vec![t3, 3]]
        );
        let p = parameter_at(&e, this, 0);
        assert_eq!(
            calls(&e, SHADER_PARAM_INIT_CONSTANT_MAP),
            vec![vec![p, 0, 5]]
        );
        assert_eq!(calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT).len(), 5);
        assert_eq!(
            calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT)[0][2],
            (1.0f32 / 512.0).to_bits()
        );
        assert_eq!(
            calls(&e, EFFECT_CALL_00BA3BA0),
            vec![vec![this, 0, 1, 2, 3, 0, 0, 0]]
        );
        assert_eq!(
            calls(&e, EFFECT_CALL_00BA3C70),
            vec![vec![this, 0, 0x0e71_0134, p, 0]]
        );
    }

    #[test]
    fn fn_00bb9540_calls_both_slots_and_answers_the_second() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x0e9e_0000);
        e.register(0x0e9f_0150, |_, _| 0x77u32.into_ret());
        e.register(0x0e9f_014c, |_, _| Ret::default());
        assert_eq!(e.call(0x00bb_9540, &args![this]).u32(), 0x77);
        assert_eq!(calls(&e, 0x0e9f_014c), vec![vec![this]]);
        assert_eq!(calls(&e, 0x0e9f_0150), vec![vec![this]]);
    }

    #[test]
    fn fn_00bb9560_makes_the_passes() {
        let mut e = engine();
        let this = e.mem.alloc(4);
        assert!(e.call(0x00bb_9560, &args![this]).bool());
        let made = calls(&e, NEW_PASS_00E804D0);
        assert_eq!(made.len(), 0x70);
        assert_eq!(made[0], vec![PASSES_01200EC0]);
        assert_eq!(made[0x6f], vec![PASSES_01200EC0 + 0x1bc]);
        assert_eq!(calls(&e, CALL_00C072C0), vec![vec![this]]);
    }

    #[test]
    fn fn_00bb95e0_stores_the_two_groups() {
        let mut e = engine();
        e.call(
            0x00bb_95e0,
            &args![2u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32, 7u32, 8u32],
        );
        let entry = LIGHT_TABLE_01201C80 + 0x40;
        let words: Vec<u32> = (0..8).map(|i| e.mem.u32(entry + 4 * i)).collect();
        assert_eq!(words, vec![5, 6, 7, 8, 1, 2, 3, 4]);
    }

    #[test]
    fn fn_00bb9640_passes_the_object_value_to_the_stage() {
        let mut e = engine();
        e.register(0x0e9f_00e0, |_, _| 0x55u32.into_ret());
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, 0x0e9e_0000);
        e.call(0x00bb_9640, &args![0u32, 0x0e33_0000u32, object]);
        assert_eq!(
            calls(&e, STAGE_CALL_00E7DE90),
            vec![vec![0x0e33_0000, 0x55]]
        );
    }

    #[test]
    fn fn_00bb96d0_clears_the_flags() {
        let mut e = engine();
        let holder = e.mem.alloc(0xc);
        e.mem.set_u8(holder + 8, 1);
        e.set_global::<u32>(HOLDER_012010BC, holder);
        e.set_global::<u8>(FLAG_01200863, 1);
        e.call(0x00bb_96d0, &args![0x5555u32, 0u32]);
        assert_eq!(e.global::<u8>(FLAG_01200863), 0);
        assert_eq!(e.mem.u8(holder + 8), 0);
    }
}

#[cfg(test)]
mod n071_tests {
    use super::*;
    use crate::units::bsshader::bsshaderproperty::{STRINGS_SET_AT, STRINGS_SET_SIZE};
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::{object, rig};
    use crate::units::bsshader::shadowlightshader::n071_rig::{calls, order, slot_target, vobject};

    #[test]
    fn fn_00bb8280_clones_the_fog_property() {
        let mut e = rig();
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(FOG_PROPERTY_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(PROPERTY_COPY_00A99E00, |_, _| Ret::default());
        let made = e.call(0x00bb_8280, &args![0x500u32, 7u32]).u32();
        assert_eq!(calls(&e, NI_ALLOCATE_00AA13E0), vec![vec![0x64]]);
        assert_eq!(calls(&e, FOG_PROPERTY_CONSTRUCT), vec![vec![made]]);
        assert_eq!(
            calls(&e, PROPERTY_COPY_00A99E00),
            vec![vec![0x500, made, 7]]
        );
        e.register(NI_ALLOCATE_00AA13E0, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00bb_8280, &args![0x500u32, 7u32]).u32(), 0);
        assert_eq!(calls(&e, PROPERTY_COPY_00A99E00)[1], vec![0x500, 0, 7]);
    }

    #[test]
    fn fn_00bb8300_adds_the_name_and_both_floats() {
        let mut e = rig();
        e.map(0x0120_0000, 0x1000);
        e.set_global(FOG_RTTI_012007E0, 0x7e0u32);
        e.register(PROPERTY_VIEWER_STRINGS_00A9A2A0, |_, _| Ret::default());
        e.register(VIEWER_STRING, |_, _| 0x51u32.into_ret());
        e.register(VIEWER_STRING_FLOAT, |_, a| a[0].into_ret());
        e.register(STRINGS_SET_SIZE, |_, _| Ret::default());
        e.register(STRINGS_SET_AT, |e, a| {
            let size = e.mem.u16(a[0] + 0xa);
            e.mem.set_u16(a[0] + 0xa, size + 1);
            let word = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x20 + 4 * a[1], word);
            Ret::default()
        });
        let strings = e.mem.alloc(0x40);
        e.mem.set_u16(strings + 8, 8);
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x2c, 0x7f80_0001);
        e.mem.set_f32(this + 0x30, 2.5);
        e.call(0x00bb_8300, &args![this, strings]);
        assert_eq!(
            calls(&e, PROPERTY_VIEWER_STRINGS_00A9A2A0),
            vec![vec![this, strings]]
        );
        assert_eq!(calls(&e, VIEWER_STRING), vec![vec![0x7e0]]);
        assert_eq!(
            calls(&e, VIEWER_STRING_FLOAT),
            vec![
                vec![LABEL_010B9EE0, 0x7fc0_0001],
                vec![LABEL_010B9ECC, 2.5f32.to_bits()]
            ]
        );
        let added: Vec<u32> = (0..3).map(|i| e.mem.u32(strings + 0x20 + 4 * i)).collect();
        assert_eq!(added, vec![0x51, LABEL_010B9EE0, LABEL_010B9ECC]);
    }

    /// An effect with five texture slots (2 and 4 null), the given parameters at +0x1c.
    fn effect(e: &mut Engine) -> (u32, Vec<u32>) {
        let this = vobject(e, 0x40, &[]);
        let textures: Vec<u32> = (0..5)
            .map(|i| if i == 2 || i == 4 { 0 } else { 0x7000 + i })
            .collect();
        let array = e.mem.alloc(0x14);
        for (i, t) in textures.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *t);
        }
        e.mem.set_u32(this + 0x2c, array);
        (this, textures)
    }

    #[test]
    fn fn_00bb8590_drops_the_shared_texture_and_the_slots() {
        let mut e = rig();
        e.map(0x0120_0000, 0x1000);
        e.register(TEXTURE_SET_00BA37F0, |_, _| Ret::default());
        let shared = object(&mut e, 1);
        e.set_global(TEXTURE_01200818, shared);
        let (this, textures) = effect(&mut e);
        e.call(0x00bb_8590, &args![this]);
        assert_eq!(e.global::<u32>(TEXTURE_01200818), 0);
        assert_eq!(e.mem.u32(shared + 4), 0);
        assert_eq!(
            calls(&e, TEXTURE_SET_00BA37F0),
            vec![
                vec![textures[0], 0],
                vec![textures[1], 0],
                vec![textures[3], 0]
            ]
        );
    }

    fn constants_rig() -> (Engine, u32, [u32; 2]) {
        let mut e = rig();
        for page in [0x0109_9000u32, 0x011a_e000, 0x0120_0000] {
            e.map(page, 0x1000);
        }
        for target in [
            TEXTURE_SET_00BA37F0,
            SHADER_PARAM_SET_PIXEL_CONSTANT,
            TEXTURE_CALL_00D47A40,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.set_global(TEXTURE_01200810, 0x810u32);
        e.set_global(TEXTURE_01200814, 0x814u32);
        e.set_global(SCALE_011AE5E8, 2.0f32);
        e.set_global(TWO_TO_32_01099378, 4_294_967_296.0f32);
        e.set_global(CONSTANT_011AE5EC, 1.0f32);
        e.set_global(CONSTANT_011AE5FC, 9.0f32);
        e.set_global(PARAMETER_VALUE_012007EC, 0x7ec_u32);
        let (this, _) = effect(&mut e);
        let parameters = [e.mem.alloc(0x88), e.mem.alloc(0x88)];
        let array = e.mem.alloc(8);
        e.mem.set_u32(array, parameters[0]);
        e.mem.set_u32(array + 4, parameters[1]);
        e.mem.set_u32(this + 0x1c, array);
        (e, this, parameters)
    }

    #[test]
    fn fn_00bb85e0_sets_the_first_parameter() {
        let (mut e, this, parameters) = constants_rig();
        e.register(IMAGE_SPACE_TEXTURE_GET_TEXTURE, |_, _| 0x1780u32.into_ret());
        e.mem.set_u16(this + 0x32, 2);
        assert!(!e.call(0x00bb_85e0, &args![this, 0u32]).bool());
        let sets = calls(&e, TEXTURE_SET_00BA37F0);
        // Texture 3 and 4 slots: 4 is null in the rig, 3 holds 0x7003.
        assert_eq!(sets[0], vec![0x7003, 0x810]);
        assert_eq!(sets[1], vec![0, 0x814]);
        // No shared 01200818: texture 1's texture goes to slot 2 (null).
        assert_eq!(
            calls(&e, IMAGE_SPACE_TEXTURE_GET_TEXTURE),
            vec![vec![0x7001]]
        );
        assert_eq!(sets[2], vec![0, 0x1780]);
        let constants = calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT);
        assert_eq!(constants.len(), 4);
        assert!(constants.iter().all(|c| c[0] == parameters[0]));
        assert_eq!(constants[0][2], 1.0f32.to_bits());
        assert_eq!(constants[1][5], 0.0f32.to_bits());
        assert_eq!(constants[3][2..].to_vec(), vec![0, 0, 0, 0]);
        assert_eq!(e.mem.u32(parameters[0] + 0x28), 0x7ec);
        assert_eq!(
            calls(&e, TEXTURE_CALL_00D47A40),
            vec![vec![0x7000, 1], vec![0x7001, 1]]
        );
    }

    #[test]
    fn fn_00bb85e0_second_parameter_with_the_texture_sizes() {
        let (mut e, this, parameters) = constants_rig();
        let shared = e.mem.alloc(8);
        e.set_global(TEXTURE_01200818, shared);
        e.set_global(PASS_012007FC, 1u32);
        e.set_global(FLAG_012007F8, 1u8);
        let sized = vobject(&mut e, 8, &[(0x94, 4), (0x98, 0xffff_fffc)]);
        e.register_double(IMAGE_SPACE_TEXTURE_GET_TEXTURE, move |_, _| {
            sized.into_ret()
        });
        let array = e.mem.u32(this + 0x2c);
        e.mem.set_u32(array + 8, 0x7002);
        e.call(0x00bb_85e0, &args![this, 0u32]);
        let sets = calls(&e, TEXTURE_SET_00BA37F0);
        assert_eq!(sets[2], vec![0x7002, shared]);
        let constants = calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT);
        assert_eq!(constants.len(), 5);
        assert!(constants.iter().all(|c| c[0] == parameters[1]));
        assert_eq!(constants[1][5], 1.0f32.to_bits());
        // 1/4 * 2 and 1/(2^32 - 4) * 2.
        let width = f32::from_bits(constants[3][2]);
        let height = f32::from_bits(constants[3][3]);
        assert_eq!(width, 0.5);
        assert_eq!(height, (2.0 / 4_294_967_292.0f64) as f32);
        assert_eq!(
            constants[4][1..].to_vec(),
            vec![4, 9.0f32.to_bits(), 0, 0, 0]
        );
        assert!(calls(&e, TEXTURE_CALL_00D47A40).is_empty());
    }

    #[test]
    fn fn_00bb8890_renders_through_the_chosen_object() {
        let mut e = rig();
        e.map(0x0120_0000, 0x1000);
        e.register(EFFECT_CALL_00BA3E30, |_, _| Ret::default());
        let this = vobject(&mut e, 0x40, &[]);
        let parameters = e.mem.alloc(8);
        e.mem.set_u32(parameters, 0xa0);
        e.mem.set_u32(parameters + 4, 0xa1);
        e.mem.set_u32(this + 0x1c, parameters);
        let first = vobject(&mut e, 8, &[]);
        let second = vobject(&mut e, 8, &[]);
        let objects = e.mem.alloc(8);
        e.mem.set_u32(objects, first);
        e.mem.set_u32(objects + 4, second);
        e.mem.set_u32(this + 0xc, objects);
        e.call(0x00bb_8890, &args![this, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(calls(&e, slot_target(&e, this, 0x10)), vec![vec![this, 4]]);
        assert_eq!(calls(&e, EFFECT_CALL_00BA3E30), vec![vec![this, 0]]);
        assert_eq!(
            calls(&e, slot_target(&e, this, 0x1c)),
            vec![vec![this, 0xa0]]
        );
        assert_eq!(
            calls(&e, slot_target(&e, first, 4)),
            vec![vec![first, 1, 2, 0xa0, 4]]
        );
        let log = order(&e);
        assert_eq!(*log.last().unwrap(), slot_target(&e, this, 0x14));
        e.set_global(PASS_012007FC, 1u32);
        e.call(0x00bb_8890, &args![this, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(
            calls(&e, slot_target(&e, second, 4)),
            vec![vec![second, 1, 2, 0xa1, 4]]
        );
    }

    #[test]
    fn fn_00bb9590_and_00bb95b0() {
        let mut e = rig();
        e.map(0x0120_1000, 0x1000);
        e.register(MANAGER_CALL_00C072C0, |_, _| Ret::default());
        let this = vobject(&mut e, 8, &[]);
        e.call(0x00bb_9590, &args![this]);
        assert_eq!(
            order(&e),
            vec![
                0x00bb_9590,
                slot_target(&e, this, 0x148),
                MANAGER_CALL_00C072C0
            ]
        );
        e.call(0x00bb_95b0, &args![3u32, 1u32, 2u32, 3u32, 4u32]);
        let words: Vec<u32> = (0..5)
            .map(|i| e.global::<u32>(RECORDS_01201C80 + 0x60 + 4 * i))
            .collect();
        assert_eq!(words, vec![1, 2, 3, 4, 0]);
    }

    #[test]
    fn fn_00bb9660_updates_the_tables() {
        let mut e = rig();
        e.map(0x0120_0000, 0x1000);
        e.register(RENDERER_CALL_00E814B0, |_, _| Ret::default());
        let this = e.mem.alloc(0x30);
        let a = vobject(&mut e, 8, &[]);
        let b = vobject(&mut e, 8, &[]);
        let c = vobject(&mut e, 8, &[]);
        e.set_global(OBJECTS_01200DD8 + 0xe0, a);
        e.set_global(OBJECTS_01200C48 + 0xc4, b);
        e.set_global(OBJECTS_01200D10, c);
        e.call(0x00bb_9660, &args![this]);
        assert!(calls(&e, slot_target(&e, a, 0xa0)).is_empty());
        assert_eq!(calls(&e, slot_target(&e, b, 0x88)).len(), 1);
        assert_eq!(calls(&e, slot_target(&e, c, 0x88)).len(), 1);
        e.mem.set_u32(this + 0x28, 1);
        e.call(0x00bb_9660, &args![this]);
        assert_eq!(calls(&e, slot_target(&e, a, 0xa0)), vec![vec![a]]);
        assert_eq!(
            calls(&e, RENDERER_CALL_00E814B0),
            vec![vec![this], vec![this]]
        );
    }
}
