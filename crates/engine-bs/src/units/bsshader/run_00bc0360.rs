//! `BSShader/run_00bc0360` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
/// The `NiTArray` set-size `00a90180` (`thiscall` on the array at +0x28, one word).
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
/// This effect's vtable (`010ba350`; no RTTI locator before it) and the four `float`s it sets
/// to 1.0 (`01201f30` to `01201f3c`).
const VTABLE_010BA350: u32 = 0x010b_a350;
const FLOATS_01201F30: u32 = 0x0120_1f30;

// Translated from 00bc0600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc0360`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the vtable
/// `010ba350`, sizes the array at +0x28 to 5 (`00a90180`), clears the byte +4 and stores 1.0
/// (`FLD1`, through four stack words) in the `float`s `01201f30`, `01201f3c`, `01201f34` and
/// `01201f38`, in that order. Answers `this`.
// Translation incomplete: 00bc0600 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00bc0600(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010BA350);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 5u32]);
    e.mem.set_u8(base + 4, 0);
    for index in [0u32, 3, 1, 2] {
        e.set_global(FLOATS_01201F30 + 4 * index, 1.0f32);
    }
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The three shared `NiPointer`s of this effect's textures (`01201f40`, `01201f44`,
/// `01201f48`) and the one `00bc0940` clears (`01201f58`).
const TEXTURE_01201F40: u32 = 0x0120_1f40;
const TEXTURE_01201F44: u32 = 0x0120_1f44;
const TEXTURE_01201F48: u32 = 0x0120_1f48;
const POINTER_01201F58: u32 = 0x0120_1f58;
/// The array store `00ba3aa0` (unnamed; `thiscall` on an effect array, `RET 8`: index, the
/// address of the word to store).
pub(crate) const ARRAY_SET_00BA3AA0: u32 = 0x00ba_3aa0;
/// `ImageSpaceTexture::~ImageSpaceTexture` (engine map, `00ba3a00`, `thiscall`).
pub(crate) const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
/// `ImageSpaceEffect::~ImageSpaceEffect` (engine map, `00ba3d30`, `thiscall`).
pub(crate) const EFFECT_DESTRUCT_00BA3D30: u32 = 0x00ba_3d30;
/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

/// One texture of the effect's array at +0x28 destroyed: when the element at `index` (the
/// array's base at +0x2c) is set, its destructor and the memory manager's free run (the
/// element tested again for null in between), and the element is cleared through `00ba3aa0`
/// with a null word.
pub(crate) fn destroy_texture(e: &mut Engine, base: u32, index: u32, local: u32) {
    let elements = e.mem.u32(base + 0x2c);
    if e.mem.u32(elements + 4 * index) != 0 {
        let texture = e.mem.u32(elements + 4 * index);
        if texture != 0 {
            e.call(TEXTURE_DESTRUCT_00BA3A00, &args![texture]);
            e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, texture]);
        }
        e.mem.set_u32(local, 0);
        e.call(ARRAY_SET_00BA3AA0, &args![base + 0x28, index, local]);
    }
}

/// One shader parameter of the effect's array at +0x18 deleted: when the element at `index`
/// (the array's base at +0x1c) is set, it is deleted through its vtable slot 0 with 1 (the
/// element tested again for null in between) and cleared through `00ba3aa0` with a null word.
pub(crate) fn delete_parameter(e: &mut Engine, base: u32, index: u32, local: u32) {
    let elements = e.mem.u32(base + 0x1c);
    if e.mem.u32(elements + 4 * index) != 0 {
        let parameter = e.mem.u32(elements + 4 * index);
        if parameter != 0 {
            e.vcall(parameter, 0, &args![1u32]);
        }
        e.mem.set_u32(local, 0);
        e.call(ARRAY_SET_00BA3AA0, &args![base + 0x18, index, local]);
    }
}

// Translated from 00bc0360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc0360`): the destructor of `00bc0600`'s effect,
/// `thiscall`, no stack words. Stores the vtable `010ba350`, clears the shared `NiPointer`s
/// `01201f40`, `01201f44` and `01201f48`; when the first element of the array at +0x18 (base at
/// +0x1c) is set, deletes it through its vtable slot 0 with 1 (when not null) and clears it
/// (`00ba3aa0` with a null word); destroys the textures 3, 4 and 2 of the array at +0x28; then
/// `ImageSpaceEffect::~ImageSpaceEffect` (`00ba3d30`).
// Translation incomplete: 00bc0360 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00bc0360(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010BA350);
    for pointer in [TEXTURE_01201F40, TEXTURE_01201F44, TEXTURE_01201F48] {
        ni_pointer_clear(e, pointer);
    }
    e.with_stack(4, |e, local| {
        let local = local.addr();
        delete_parameter(e, base, 0, local);
        for index in [3u32, 4, 2] {
            destroy_texture(e, base, index, local);
        }
    });
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

// Translated from 00bc06a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc0360`): the effect's scalar deleting destructor,
/// `thiscall`, one stack word (the flags). Runs `00bc0360`; with bit 0 set, frees `this`
/// through the memory manager (`00aa4060`). Answers `this`.
pub fn fn_00bc06a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bc0360(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// `00ba3f90` (unnamed; `thiscall`, one word), the `NiTArray` set-size `00a90180` and
/// `BSShaderManager::GetTexture` (engine map, `00b55840`, `cdecl`: path, a flag, the
/// `NiPointer` to fill, two words).
const EFFECT_SET_UP_00BA3F90: u32 = 0x00ba_3f90;
const GET_TEXTURE: u32 = 0x00b5_5840;
/// `"Data\Textures\Pipboy3000\PipboyScanlines.dds"` (`01078990`) and
/// `"Data\Textures\Pipboy3000\PipboyDistortEffectMap.dds"` (`010b9f24`).
const SCANLINES_PATH_01078990: u32 = 0x0107_8990;
const DISTORT_PATH_010B9F24: u32 = 0x010b_9f24;
/// The texture setter `00ba37f0` and `00ba3760` (unnamed, unit `imagespacetexture.cpp`;
/// `thiscall` on a texture, one word each).
const TEXTURE_SET_00BA37F0: u32 = 0x00ba_37f0;
const TEXTURE_CALL_00BA3760: u32 = 0x00ba_3760;
/// `ImageSpaceShaderParam::ImageSpaceShaderParam` (`00b8a9e0`, `thiscall`), `::InitConstantMap`
/// (`00b8ab10`, `RET 8`) and `::SetPixelConstant` (`00b8a790`, `RET 0x14`: index, four
/// `float`s) (engine map).
const SHADER_PARAM_CONSTRUCT: u32 = 0x00b8_a9e0;
const SHADER_PARAM_INIT_CONSTANT_MAP: u32 = 0x00b8_ab10;
const SHADER_PARAM_SET_PIXEL_CONSTANT: u32 = 0x00b8_a790;
/// `00ba3c00` (`RET 0x24`) and `00ba3c70` (`RET 0x10`) (unnamed; `thiscall` on the effect).
const EFFECT_CALL_00BA3C00: u32 = 0x00ba_3c00;
const EFFECT_CALL_00BA3C70: u32 = 0x00ba_3c70;
/// The `float`s the first pixel constant takes (`011ae614`, `01201f2c`, `011ae618`).
const CONSTANT_011AE614: u32 = 0x011a_e614;
const CONSTANT_01201F2C: u32 = 0x0120_1f2c;
const CONSTANT_011AE618: u32 = 0x011a_e618;

/// A new 16-byte texture record from the memory manager: three bytes and three words cleared
/// when the block is not null (a null block is stored as it is) into `local`, stored at
/// `index` of the array at +0x28 through `00ba3aa0`.
pub(crate) fn new_texture(e: &mut Engine, base: u32, index: u32, local: u32) {
    let texture = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x10u32])
        .u32();
    if texture != 0 {
        for offset in 0..3 {
            e.mem.set_u8(texture + offset, 0);
        }
        for offset in [4u32, 8, 0xc] {
            e.mem.set_u32(texture + offset, 0);
        }
    }
    e.mem.set_u32(local, texture);
    e.call(ARRAY_SET_00BA3AA0, &args![base + 0x28, index, local]);
}

/// The texture at `index` of the array at +0x28 (base at +0x2c).
pub(crate) fn texture_at(e: &Engine, base: u32, index: u32) -> u32 {
    let elements = e.mem.u32(base + 0x2c);
    e.mem.u32(elements + 4 * index)
}

// Translated from 00bc06c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc0360`): the effect's set-up, `thiscall` with two stack
/// words (`RET 8`): an owner whose word at +0x8 holds an object with a pointer at +0x12c, and
/// a word the code never reads. Runs `00ba3f90` with 1, sizes the array at +0x28 to 5, gets the
/// scan-line and distortion textures into `01201f40` and `01201f48` (`GetTexture`, flag 1),
/// makes the texture records 3 (given the scan lines and `00ba3760` with 3), 4 (the distortion
/// map) and 2 (no texture). Makes a shader parameter (0x88 bytes, `00b8a9e0`; a null block
/// gives none), `InitConstantMap(0, 3)`, sets its word +0x28 to 1 and the pixel constants 1
/// (`011ae614`, `01201f2c`, `011ae618`, 0.0) and 2 (`01201f30` to `01201f3c`), stores it as
/// element 0 of the array at +0x18, then calls `00ba3c00` with (0, 1, 2, 3, 4, 0, 0, 0, 0) and
/// `00ba3c70` with (0, the owner's pointer, the parameter, 0): the pointer goes through an
/// adjustment by -0x6c and back (null stays null).
// Translation incomplete: 00bc06c0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the parameter's allocation) is not translated.
pub fn fn_00bc06c0(e: &mut Engine, this: Ptr, owner: Ptr, _unused_2: u32) {
    let base = this.addr();
    e.call(EFFECT_SET_UP_00BA3F90, &args![base, 1u32]);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 5u32]);
    e.call(
        GET_TEXTURE,
        &args![SCANLINES_PATH_01078990, 1u32, TEXTURE_01201F40, 0u32, 0u32],
    );
    let scanlines = e.global::<u32>(TEXTURE_01201F40);
    e.call(
        GET_TEXTURE,
        &args![DISTORT_PATH_010B9F24, 1u32, TEXTURE_01201F48, 0u32, 0u32],
    );
    e.with_stack(8, |e, locals| {
        let first = locals.addr();
        let second = first + 4;
        let distort = e.global::<u32>(TEXTURE_01201F48);
        e.mem.set_u32(second, distort);
        new_texture(e, base, 3, first);
        let texture = texture_at(e, base, 3);
        e.call(TEXTURE_SET_00BA37F0, &args![texture, scanlines]);
        let texture = texture_at(e, base, 3);
        e.call(TEXTURE_CALL_00BA3760, &args![texture, 3u32]);
        new_texture(e, base, 4, first);
        let distort = e.mem.u32(second);
        let texture = texture_at(e, base, 4);
        e.call(TEXTURE_SET_00BA37F0, &args![texture, distort]);
        new_texture(e, base, 2, second);
        let texture = texture_at(e, base, 2);
        e.call(TEXTURE_SET_00BA37F0, &args![texture, 0u32]);
    });
    let holder = e.mem.u32(owner.addr() + 8);
    let pointer = e.mem.u32(holder + 0x12c);
    let adjusted = if pointer != 0 {
        pointer.wrapping_sub(0x6c)
    } else {
        0
    };
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x88u32])
        .u32();
    let parameter = if block != 0 {
        e.call(SHADER_PARAM_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(
        SHADER_PARAM_INIT_CONSTANT_MAP,
        &args![parameter, 0u32, 3u32],
    );
    e.mem.set_u32(parameter + 0x28, 1);
    let first = [CONSTANT_011AE614, CONSTANT_01201F2C, CONSTANT_011AE618]
        .map(|addr| x87_quiet_bits(e.global::<u32>(addr)));
    e.call(
        SHADER_PARAM_SET_PIXEL_CONSTANT,
        &args![parameter, 1u32, first[0], first[1], first[2], 0.0f32],
    );
    let second =
        [0u32, 1, 2, 3].map(|index| x87_quiet_bits(e.global::<u32>(FLOATS_01201F30 + 4 * index)));
    e.call(
        SHADER_PARAM_SET_PIXEL_CONSTANT,
        &args![parameter, 2u32, second[0], second[1], second[2], second[3]],
    );
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), parameter);
        e.call(ARRAY_SET_00BA3AA0, &args![base + 0x18, 0u32, local.addr()]);
    });
    e.call(
        EFFECT_CALL_00BA3C00,
        &args![base, 0u32, 1u32, 2u32, 3u32, 4u32, 0u32, 0u32, 0u32, 0u32],
    );
    let pointer = if adjusted != 0 {
        adjusted.wrapping_add(0x6c)
    } else {
        0
    };
    e.call(
        EFFECT_CALL_00BA3C70,
        &args![base, 0u32, pointer, parameter, 0u32],
    );
}

// Translated from 00bc0940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bc0360`): no arguments. Clears the shared `NiPointer` at
/// `01201f58` (the object released when it was set).
pub fn fn_00bc0940(e: &mut Engine) {
    ni_pointer_clear(e, POINTER_01201F58);
}

/// Strings of `00bc09b0` (read in Ghidra): the HLSL files `"bolt\v\bolt.v.hlsl"` (`010ba39c`)
/// and `"bolt\p\bolt.p.hlsl"` (`010ba37c`), the compiled names `"BOLT.vso"` (`010ba390`) and
/// `"BOLT.pso"` (`010ba370`), the vertex profile `"vs_1_1"` (`010a93ec`).
const BOLT_VERTEX_SOURCE_010BA39C: u32 = 0x010b_a39c;
const BOLT_PIXEL_SOURCE_010BA37C: u32 = 0x010b_a37c;
const BOLT_VERTEX_NAME_010BA390: u32 = 0x010b_a390;
const BOLT_PIXEL_NAME_010BA370: u32 = 0x010b_a370;
const PROFILE_VS_1_1_010A93EC: u32 = 0x010a_93ec;
/// `00b4f380` (unnamed, unit `bsshadermanager.cpp`; `cdecl`, one word): answers the pixel
/// profile string the second compile takes.
const PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;
/// The frame of `00bc09b0` (0x2b4 bytes up to the return address) and its buffers.
const BOLT_FRAME_BYTES: u32 = 0x2b4;
const BOLT_BUFFERS: ShaderBuffers = ShaderBuffers {
    name: 0xa8,
    path: 0x1ac,
};
const BOLT_VERTEX_BLOCK: ShaderSource = ShaderSource {
    offset: 0x5c,
    words: &[BOLT_VERTEX_SOURCE_010BA39C, 0, 0],
    cleared: 0x40,
};
const BOLT_PIXEL_BLOCK: ShaderSource = ShaderSource {
    offset: 0x10,
    words: &[BOLT_PIXEL_SOURCE_010BA37C, 0, 0],
    cleared: 0x40,
};

// Translated from 00bc09b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; reached from the bolt shader's vtable `010ba3b0`, slot
/// +0x13c): the bolt shader's shader loader, `thiscall`, no stack words. Compiles
/// `"bolt\v\bolt.v.hlsl"` without macros (`"BOLT.vso"`, `"vs_1_1"`) into the `NiPointer` at
/// +0x18c, then asks `00b4f380` (with 1) for the pixel profile and compiles
/// `"bolt\p\bolt.p.hlsl"` (`"BOLT.pso"`) into the `NiPointer` at +0x190. Each source block on
/// the stack holds the file, two null words and 0x40 bytes cleared by `memset`.
// Translation incomplete: 00bc09b0 - the /GS stack cookie check (00ec408c) is not translated.
pub fn fn_00bc09b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(BOLT_FRAME_BYTES, |e, frame| {
        let frame = frame.addr();
        set_up_sources(e, frame, &[BOLT_VERTEX_BLOCK]);
        let vertex = ShaderCompile {
            source: BOLT_VERTEX_BLOCK.offset,
            output: BOLT_VERTEX_NAME_010BA390,
            profile: PROFILE_VS_1_1_010A93EC,
            pixel: false,
            slot: 0x18c,
        };
        compile_shaders(e, base, frame, &BOLT_BUFFERS, &[vertex]);
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        set_up_sources(e, frame, &[BOLT_PIXEL_BLOCK]);
        let pixel = ShaderCompile {
            source: BOLT_PIXEL_BLOCK.offset,
            output: BOLT_PIXEL_NAME_010BA370,
            profile,
            pixel: true,
            slot: 0x190,
        };
        compile_shaders(e, base, frame, &BOLT_BUFFERS, &[pixel]);
    });
}

// Translated from 00bc0500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010ba364`), `thiscall`, no
/// stack words: releases and clears the shared `NiPointer` `01201f44`, then gives each of the
/// textures 0 to 4 of the array at +0x28 that is set no texture (`00ba37f0` with 0).
pub fn fn_00bc0500(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    ni_pointer_clear(e, TEXTURE_01201F44);
    for index in 0..5 {
        let texture = texture_at(e, base, index);
        if texture != 0 {
            e.call(TEXTURE_SET_00BA37F0, &args![texture, 0u32]);
        }
    }
}

// Translated from 00bc0550 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010ba36c`), `thiscall`
/// with one unread word; answers false. Textures 3, 4 and 2 get `01201f40`, `01201f48` and
/// `01201f44` (`00ba37f0`); the first shader parameter of the array at +0x18 gets the pixel
/// constants 1 (`011ae614`, `01201f2c`, `011ae618`, 0.0) and 2 (`01201f30`..`01201f3c`), the
/// `float`s through `FLD`/`FSTP`.
pub fn fn_00bc0550(e: &mut Engine, this: Ptr, _unused_0: u32) -> bool {
    let base = this.addr();
    for (index, shared) in [
        (3u32, TEXTURE_01201F40),
        (4, TEXTURE_01201F48),
        (2, TEXTURE_01201F44),
    ] {
        let value = e.global::<u32>(shared);
        let texture = texture_at(e, base, index);
        e.call(TEXTURE_SET_00BA37F0, &args![texture, value]);
    }
    let parameters = e.mem.u32(base + 0x1c);
    let parameter = e.mem.u32(parameters);
    let quiet = |e: &Engine, addr: u32| x87_quiet_bits(e.global::<u32>(addr));
    let words = [
        quiet(e, CONSTANT_011AE614),
        quiet(e, CONSTANT_01201F2C),
        quiet(e, CONSTANT_011AE618),
        0.0f32.to_bits(),
    ];
    e.call(
        SHADER_PARAM_SET_PIXEL_CONSTANT,
        &args![parameter, 1u32, words[0], words[1], words[2], words[3]],
    );
    let words = [0u32, 4, 8, 0xc].map(|offset| quiet(e, FLOATS_01201F30 + offset));
    e.call(
        SHADER_PARAM_SET_PIXEL_CONSTANT,
        &args![parameter, 2u32, words[0], words[1], words[2], words[3]],
    );
    false
}

// Translated from 00bc0920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable entries at `010b81c8`, `010ba4d0`, `010baeb8`), `thiscall`,
/// no stack words: calls the vtable +0x13c, then jumps through the vtable +0x140.
pub fn fn_00bc0920(e: &mut Engine, this: Ptr) {
    e.vcall(this.addr(), 0x13c, &args![]);
    e.vcall(this.addr(), 0x140, &args![]);
}

/// `00e814b0` (unnamed, `thiscall`, no stack words).
const RENDERER_CALL_00E814B0: u32 = 0x00e8_14b0;

// Translated from 00bc0970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable entry at `010ba480`), `thiscall`, no stack words: with the
/// word +0x28 set, calls the vtable +0xa0 of the object +0x18c and +0x88 of the object +0x190;
/// then jumps to `00e814b0`.
pub fn fn_00bc0970(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    if e.mem.u32(base + 0x28) != 0 {
        let object = e.mem.u32(base + 0x18c);
        e.vcall(object, 0xa0, &args![]);
        let object = e.mem.u32(base + 0x190);
        e.vcall(object, 0x88, &args![]);
    }
    e.call(RENDERER_CALL_00E814B0, &args![base]);
}

/// The bolt segment count (`01201f50`) and the shader model (`011f91c0`).
const SEGMENT_COUNT_01201F50: u32 = 0x0120_1f50;
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;

// Translated from 00bc0b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, no arguments: when the word `01201f50` is 0, sets it to 0x4b
/// below shader model 2 (`011f91c0`, signed) and 0xeb from it on (`SETL`, `DEC`, `AND 0xa0`,
/// `ADD 0x4b`). Answers the word.
pub fn fn_00bc0b40(e: &mut Engine) -> u32 {
    let count = e.global::<u32>(SEGMENT_COUNT_01201F50);
    if count != 0 {
        return count;
    }
    let below = u32::from(e.global::<i32>(SHADER_MODEL_011F91C0) < 2);
    let count = (below.wrapping_sub(1) & 0xa0) + 0x4b;
    e.set_global(SEGMENT_COUNT_01201F50, count);
    count
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bc_0500, fn_00bc0500(Ptr)),
        entry!(0x00bc_0550, fn_00bc0550(Ptr, u32) -> bool),
        entry!(0x00bc_0920, fn_00bc0920(Ptr)),
        entry!(0x00bc_0970, fn_00bc0970(Ptr)),
        entry!(0x00bc_0b40, fn_00bc0b40() -> u32),
        entry!(0x00bc_0600, fn_00bc0600(Ptr) -> Ptr),
        entry!(0x00bc_0360, fn_00bc0360(Ptr)),
        entry!(0x00bc_06a0, fn_00bc06a0(Ptr, u32) -> Ptr),
        entry!(0x00bc_06c0, fn_00bc06c0(Ptr, Ptr, u32)),
        entry!(0x00bc_0940, fn_00bc0940()),
        entry!(0x00bc_09b0, fn_00bc09b0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bc0600_builds_the_effect_and_sets_the_floats() {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.register(ARRAY_SET_SIZE_00A90180, |_, a| a[0].into_ret());
        e.map(0x0120_1000, 0x1000);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x58);
        e.mem.write(this, &[0xcc; 8]);
        assert_eq!(e.call(0x00bc_0600, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BA350);
        assert_eq!(e.mem.u8(this + 4), 0);
        for index in 0..4 {
            assert_eq!(e.global::<f32>(FLOATS_01201F30 + 4 * index), 1.0);
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 5]),
            ]
        );
    }
}

#[cfg(test)]
mod b056_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    /// An effect with the arrays at +0x18 (base +0x1c) and +0x28 (base +0x2c), five slots each.
    fn effect(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x58);
        let first = e.mem.alloc(0x14);
        let second = e.mem.alloc(0x14);
        e.mem.set_u32(this + 0x1c, first);
        e.mem.set_u32(this + 0x2c, second);
        this
    }

    /// [`rig`] with the effect array store as a real store and no-op doubles for the rest.
    fn engine() -> Engine {
        let mut e = rig();
        e.map(0x0120_1000, 0x1000);
        e.map(0x011a_e000, 0x1000);
        e.register(ARRAY_SET_00BA3AA0, |e, a| {
            let word = e.mem.u32(a[2]);
            let elements = e.mem.u32(a[0] + 4);
            e.mem.set_u32(elements + 4 * a[1], word);
            Ret::default()
        });
        for target in [
            TEXTURE_DESTRUCT_00BA3A00,
            EFFECT_DESTRUCT_00BA3D30,
            MEMORY_DEALLOCATE,
            EFFECT_SET_UP_00BA3F90,
            ARRAY_SET_SIZE_00A90180,
            GET_TEXTURE,
            TEXTURE_SET_00BA37F0,
            TEXTURE_CALL_00BA3760,
            SHADER_PARAM_INIT_CONSTANT_MAP,
            SHADER_PARAM_SET_PIXEL_CONSTANT,
            EFFECT_CALL_00BA3C00,
            EFFECT_CALL_00BA3C70,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(SHADER_PARAM_CONSTRUCT, |_, a| a[0].into_ret());
        e
    }

    #[test]
    fn fn_00bc0360_releases_the_textures_and_the_parameter() {
        let mut e = engine();
        e.register(0x0e74_0000, |_, _| Ret::default());
        e.put_vtable(0x0e75_0000, &[0x0e74_0000]);
        let this = effect(&mut e);
        let shared = object(&mut e, 1);
        e.set_global::<u32>(TEXTURE_01201F44, shared);
        let parameter = e.mem.alloc(8);
        e.mem.set_u32(parameter, 0x0e75_0000);
        let elements = e.mem.u32(this + 0x1c);
        e.mem.set_u32(elements, parameter);
        let textures = e.mem.u32(this + 0x2c);
        e.mem.set_u32(textures + 0xc, 0x0e56_0000);
        e.mem.set_u32(textures + 8, 0x0e57_0000);
        e.call(0x00bc_06a0, &args![this, 1u32]);
        assert_eq!(e.mem.u32(this), VTABLE_010BA350);
        assert_eq!(e.global::<u32>(TEXTURE_01201F44), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![shared]]);
        assert_eq!(calls(&e, 0x0e74_0000), vec![vec![parameter, 1]]);
        assert_eq!(e.mem.u32(elements), 0);
        assert_eq!(
            calls(&e, TEXTURE_DESTRUCT_00BA3A00),
            vec![vec![0x0e56_0000], vec![0x0e57_0000]]
        );
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![
                vec![MEMORY_MANAGER, 0x0e56_0000],
                vec![MEMORY_MANAGER, 0x0e57_0000],
                vec![MEMORY_MANAGER, this]
            ]
        );
        let sets: Vec<u32> = calls(&e, ARRAY_SET_00BA3AA0).iter().map(|a| a[1]).collect();
        assert_eq!(sets, vec![0, 3, 2]);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        // Without bit 0 nothing is freed.
        let this = effect(&mut e);
        e.call(0x00bc_06a0, &args![this, 0u32]);
        assert_eq!(calls(&e, MEMORY_DEALLOCATE).len(), 3);
    }

    #[test]
    fn fn_00bc06c0_sets_up_the_textures_and_the_parameter() {
        let mut e = engine();
        e.set_global::<u32>(TEXTURE_01201F40, 0x0e60_0000);
        e.set_global::<u32>(TEXTURE_01201F48, 0x0e61_0000);
        e.set_global::<f32>(CONSTANT_011AE614, 2.0);
        e.set_global::<f32>(CONSTANT_01201F2C, 3.0);
        e.set_global::<f32>(CONSTANT_011AE618, 4.0);
        // A signalling NaN is quieted by the FLD/FSTP copy.
        e.set_global::<u32>(FLOATS_01201F30, 0x7f80_0001);
        let this = effect(&mut e);
        let owner = e.mem.alloc(0xc);
        let holder = e.mem.alloc(0x130);
        e.mem.set_u32(owner + 8, holder);
        e.mem.set_u32(holder + 0x12c, 0x0e70_0100);
        e.call(0x00bc_06c0, &args![this, owner, 0xdead_beefu32]);
        assert_eq!(calls(&e, EFFECT_SET_UP_00BA3F90), vec![vec![this, 1]]);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + 0x28, 5]]
        );
        assert_eq!(
            calls(&e, GET_TEXTURE),
            vec![
                vec![SCANLINES_PATH_01078990, 1, TEXTURE_01201F40, 0, 0],
                vec![DISTORT_PATH_010B9F24, 1, TEXTURE_01201F48, 0, 0]
            ]
        );
        let (t3, t4, t2) = (
            texture_at(&e, this, 3),
            texture_at(&e, this, 4),
            texture_at(&e, this, 2),
        );
        assert_eq!(
            calls(&e, TEXTURE_SET_00BA37F0),
            vec![vec![t3, 0x0e60_0000], vec![t4, 0x0e61_0000], vec![t2, 0]]
        );
        assert_eq!(calls(&e, TEXTURE_CALL_00BA3760), vec![vec![t3, 3]]);
        let parameter = e.mem.u32(e.mem.u32(this + 0x1c));
        assert_eq!(e.mem.u32(parameter + 0x28), 1);
        assert_eq!(
            calls(&e, SHADER_PARAM_INIT_CONSTANT_MAP),
            vec![vec![parameter, 0, 3]]
        );
        assert_eq!(
            calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT),
            vec![
                vec![
                    parameter,
                    1,
                    2.0f32.to_bits(),
                    3.0f32.to_bits(),
                    4.0f32.to_bits(),
                    0
                ],
                vec![parameter, 2, 0x7fc0_0001, 0, 0, 0]
            ]
        );
        assert_eq!(
            calls(&e, EFFECT_CALL_00BA3C00),
            vec![vec![this, 0, 1, 2, 3, 4, 0, 0, 0, 0]]
        );
        assert_eq!(
            calls(&e, EFFECT_CALL_00BA3C70),
            vec![vec![this, 0, 0x0e70_0100, parameter, 0]]
        );
    }

    #[test]
    fn fn_00bc06c0_null_pointer_and_null_parameter_block() {
        let mut e = engine();
        e.register(MEMORY_ALLOCATE, |e, a| {
            if a[1] == 0x88 {
                Ret::default()
            } else {
                e.mem.alloc(a[1]).into_ret()
            }
        });
        // The null parameter's word +0x28 is written as the exe writes it.
        e.map(0, 0x1000);
        let this = effect(&mut e);
        let owner = e.mem.alloc(0xc);
        let holder = e.mem.alloc(0x130);
        e.mem.set_u32(owner + 8, holder);
        e.call(0x00bc_06c0, &args![this, owner, 0u32]);
        assert_eq!(calls(&e, SHADER_PARAM_CONSTRUCT), Vec::<Vec<u32>>::new());
        assert_eq!(
            calls(&e, EFFECT_CALL_00BA3C70),
            vec![vec![this, 0, 0, 0, 0]]
        );
        assert_eq!(e.mem.u32(0x28), 1);
    }

    #[test]
    fn fn_00bc0940_clears_the_shared_pointer() {
        let mut e = engine();
        let shared = object(&mut e, 2);
        e.set_global::<u32>(POINTER_01201F58, shared);
        e.call(0x00bc_0940, &[]);
        assert_eq!(e.global::<u32>(POINTER_01201F58), 0);
        assert_eq!(e.mem.u32(shared + 4), 1);
        e.call(0x00bc_0940, &[]);
        assert!(calls(&e, REFERENCE_DELETE).is_empty());
    }

    #[test]
    fn fn_00bc09b0_compiles_the_bolt_shaders() {
        let mut e = loader_rig();
        e.register(PIXEL_PROFILE_00B4F380, |_, _| 0x0e77_0000u32.into_ret());
        let this = e.mem.alloc(0x194);
        e.call(0x00bc_09b0, &args![this]);
        let vertex = compiled(&e, e.mem.u32(this + 0x18c));
        assert_eq!(
            vertex,
            Compiled {
                file: BOLT_VERTEX_SOURCE_010BA39C,
                output: BOLT_VERTEX_NAME_010BA390,
                profile: PROFILE_VS_1_1_010A93EC,
                pixel: false,
                macros: [0; 8],
            }
        );
        let pixel = compiled(&e, e.mem.u32(this + 0x190));
        assert_eq!(pixel.file, BOLT_PIXEL_SOURCE_010BA37C);
        assert_eq!(pixel.output, BOLT_PIXEL_NAME_010BA370);
        assert_eq!(pixel.profile, 0x0e77_0000);
        assert!(pixel.pixel);
        assert_eq!(calls(&e, PIXEL_PROFILE_00B4F380), vec![vec![1]]);
        assert_eq!(calls(&e, MEMSET).len(), 2);
    }
}

#[cfg(test)]
mod n071_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::{object, rig};
    use crate::units::bsshader::shadowlightshader::n071_rig::{calls, order, slot_target, vobject};

    /// An effect whose texture array (+0x2c) has slots 0, 1, 3 set and 2, 4 null.
    fn effect(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0x200);
        let array = e.mem.alloc(0x14);
        for index in [0u32, 1, 3] {
            e.mem.set_u32(array + 4 * index, 0x7000 + index);
        }
        e.mem.set_u32(this + 0x2c, array);
        this
    }

    #[test]
    fn fn_00bc0500_drops_the_shared_texture_and_the_slots() {
        let mut e = rig();
        e.map(0x0120_1000, 0x1000);
        e.register(TEXTURE_SET_00BA37F0, |_, _| Ret::default());
        let shared = object(&mut e, 1);
        e.set_global(TEXTURE_01201F44, shared);
        let this = effect(&mut e);
        e.call(0x00bc_0500, &args![this]);
        assert_eq!(e.global::<u32>(TEXTURE_01201F44), 0);
        assert_eq!(
            calls(&e, TEXTURE_SET_00BA37F0),
            vec![vec![0x7000, 0], vec![0x7001, 0], vec![0x7003, 0]]
        );
    }

    #[test]
    fn fn_00bc0550_sets_the_textures_and_constants() {
        let mut e = rig();
        e.map(0x0120_1000, 0x1000);
        e.map(0x011a_e000, 0x1000);
        e.register(TEXTURE_SET_00BA37F0, |_, _| Ret::default());
        e.register(SHADER_PARAM_SET_PIXEL_CONSTANT, |_, _| Ret::default());
        e.set_global(TEXTURE_01201F40, 0x40u32);
        e.set_global(TEXTURE_01201F44, 0x44u32);
        e.set_global(TEXTURE_01201F48, 0x48u32);
        e.set_global(CONSTANT_011AE614, 0x7f80_0001u32);
        e.set_global(FLOATS_01201F30 + 0xc, 3.0f32);
        let this = effect(&mut e);
        let parameters = e.mem.alloc(8);
        e.mem.set_u32(parameters, 0x88);
        e.mem.set_u32(this + 0x1c, parameters);
        assert!(!e.call(0x00bc_0550, &args![this, 0u32]).bool());
        assert_eq!(
            calls(&e, TEXTURE_SET_00BA37F0),
            vec![vec![0x7003, 0x40], vec![0, 0x48], vec![0, 0x44]]
        );
        let constants = calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT);
        assert_eq!(constants[0], vec![0x88, 1, 0x7fc0_0001, 0, 0, 0]);
        assert_eq!(constants[1], vec![0x88, 2, 0, 0, 0, 3.0f32.to_bits()]);
    }

    #[test]
    fn fn_00bc0920_and_00bc0970_call_their_slots() {
        let mut e = rig();
        e.register(RENDERER_CALL_00E814B0, |_, _| Ret::default());
        let this = vobject(&mut e, 0x200, &[]);
        e.call(0x00bc_0920, &args![this]);
        assert_eq!(
            order(&e),
            vec![
                0x00bc_0920,
                slot_target(&e, this, 0x13c),
                slot_target(&e, this, 0x140)
            ]
        );
        let first = vobject(&mut e, 8, &[]);
        let second = vobject(&mut e, 8, &[]);
        e.mem.set_u32(this + 0x18c, first);
        e.mem.set_u32(this + 0x190, second);
        e.call_log = Some(vec![]);
        e.call(0x00bc_0970, &args![this]);
        assert_eq!(order(&e), vec![0x00bc_0970, RENDERER_CALL_00E814B0]);
        e.mem.set_u32(this + 0x28, 1);
        e.call_log = Some(vec![]);
        e.call(0x00bc_0970, &args![this]);
        assert_eq!(
            order(&e),
            vec![
                0x00bc_0970,
                slot_target(&e, first, 0xa0),
                slot_target(&e, second, 0x88),
                RENDERER_CALL_00E814B0
            ]
        );
    }

    #[test]
    fn fn_00bc0b40_computes_the_count_once() {
        let mut e = rig();
        e.map(0x0120_1000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.set_global(SHADER_MODEL_011F91C0, 1i32);
        assert_eq!(e.call(0x00bc_0b40, &args![]).u32(), 0x4b);
        e.set_global(SHADER_MODEL_011F91C0, 3i32);
        assert_eq!(e.call(0x00bc_0b40, &args![]).u32(), 0x4b);
        e.set_global(SEGMENT_COUNT_01201F50, 0u32);
        assert_eq!(e.call(0x00bc_0b40, &args![]).u32(), 0xeb);
        assert_eq!(e.global::<u32>(SEGMENT_COUNT_01201F50), 0xeb);
    }
}
