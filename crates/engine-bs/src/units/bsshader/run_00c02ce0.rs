//! `BSShader/run_00c02ce0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays) and the `NiTArray` set-size
/// `00a90180` (`thiscall` on the array at +0x28, one word).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
/// The compiler's `eh vector constructor iterator` (`00ec782f`, `stdcall`: array, element
/// size, count, constructor, destructor), the element constructor `004ebb70` (unit
/// `teswater.cpp`, unnamed) and `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`).
const VECTOR_CONSTRUCT_00EC782F: u32 = 0x00ec_782f;
const TEXTURE_CONSTRUCT_004EBB70: u32 = 0x004e_bb70;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
/// The two effects' vtables (`010bc8a4`, `010bc8d0`; no RTTI locator before them).
const VTABLE_010BC8A4: u32 = 0x010b_c8a4;
const VTABLE_010BC8D0: u32 = 0x010b_c8d0;

// Translated from 00c033c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the vtable
/// `010bc8a4` and clears the 16-byte block at +0x58 (the bytes +0x58 to +0x5a, then the words
/// +0x5c to +0x64). Answers `this`.
pub fn fn_00c033c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010BC8A4);
    for offset in [0x58u32, 0x59, 0x5a] {
        e.mem.set_u8(base + offset, 0);
    }
    for offset in [0x5cu32, 0x60, 0x64] {
        e.mem.set_u32(base + offset, 0);
    }
    this
}

// Translated from 00c03810 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the vtable
/// `010bc8d0`, constructs the two 16-byte textures at +0x5c (`eh vector constructor iterator`
/// with `004ebb70` and `ImageSpaceTexture::~ImageSpaceTexture`) and sizes the array at +0x28 to
/// 4 (`00a90180`). Answers `this`.
// Translation incomplete: 00c03810 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and the textures) is not translated.
pub fn fn_00c03810(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010BC8D0);
    e.call(
        VECTOR_CONSTRUCT_00EC782F,
        &args![
            base + 0x5c,
            0x10u32,
            2u32,
            TEXTURE_CONSTRUCT_004EBB70,
            TEXTURE_DESTRUCT_00BA3A00
        ],
    );
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 4u32]);
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{MEMORY_ALLOCATE, MEMORY_MANAGER};

/// Two tables of flag objects (22 words each, from `011febd8` and from `011fec30`; each
/// object has a byte at +8) and the per-pass flag words they test (`011fcc80` and `011fc0a0`,
/// one word per pass number).
const FIRST_FLAGS_011FEBD8: u32 = 0x011f_ebd8;
const SECOND_FLAGS_011FEC30: u32 = 0x011f_ec30;
const FIRST_MASKS_011FCC80: u32 = 0x011f_cc80;
const SECOND_MASKS_011FC0A0: u32 = 0x011f_c0a0;

/// One table's 22 objects: object `i` (when not null) gets the byte +8 set to whether the
/// pass's word in `masks` has bit `i + 1` (the exe rotates a one bit left, `ROL`), the word
/// read again for each object.
fn set_flags(e: &mut Engine, objects: u32, masks: u32, pass: u32) {
    let mut bit = 1u32;
    for index in 0..22u32 {
        let object = e.mem.u32(objects + 4 * index);
        if object != 0 {
            let mask = bit.rotate_left(1);
            let word = e.mem.u32(masks.wrapping_add(pass.wrapping_mul(4)));
            e.mem.set_u8(object + 8, u8::from(word & mask != 0));
        }
        bit = bit.rotate_left(1);
    }
}

// Translated from 00c02f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`), `cdecl`, one word (a pass number): sets the
/// byte +8 of each of the 22 objects of the table at `011febd8` from the pass's word at
/// `011fcc80` and of the 22 objects at `011fec30` from its word at `011fc0a0` (object `i`
/// takes bit `i + 1`).
pub fn fn_00c02f70(e: &mut Engine, pass: u32) {
    set_flags(e, FIRST_FLAGS_011FEBD8, FIRST_MASKS_011FCC80, pass);
    set_flags(e, SECOND_FLAGS_011FEC30, SECOND_MASKS_011FC0A0, pass);
}

/// Two objects of the second table: `011fec6c` and `011fec44`.
const FLAG_OBJECT_011FEC6C: u32 = 0x011f_ec6c;
const FLAG_OBJECT_011FEC44: u32 = 0x011f_ec44;

// Translated from 00c02ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`), `cdecl`, one word (a pass number): the flags
/// of `00c02f70`, then the object at `011fec6c` gets its byte +8 set for the passes 0xc1 and
/// 0xc2 and cleared for every other. The object at `011fec44` would get its byte +8 cleared
/// for a pass below 0x2d8 and above 0x2ef, which no pass is (the exe makes both tests).
// The exe tests an empty range (pass < 0x2d8 and pass > 0x2ef); the test stays.
#[allow(clippy::impossible_comparisons)]
pub fn fn_00c02ce0(e: &mut Engine, pass: u32) {
    fn_00c02f70(e, pass);
    let object = e.global::<u32>(FLAG_OBJECT_011FEC6C);
    let set = pass == 0xc1 || pass == 0xc2;
    e.mem.set_u8(object + 8, u8::from(set));
    let object = e.global::<u32>(FLAG_OBJECT_011FEC44);
    // Unreachable: no pass is below 0x2d8 and above 0x2ef; kept as the exe tests it.
    if object != 0 && pass < 0x2d8 && pass > 0x2ef {
        e.mem.set_u8(object + 8, 0);
    }
}

/// The default colour (`011a9bd0`, four `float`s), the default third component
/// (`01018f5c`) and the colour this sets (`011fa2d0`, four `float`s).
const DEFAULT_COLOR_011A9BD0: u32 = 0x011a_9bd0;
const DEFAULT_BLUE_01018F5C: u32 = 0x0101_8f5c;
const COLOR_011FA2D0: u32 = 0x011f_a2d0;

// Translated from 00c02d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`): a vtable slot, `thiscall` (`this` not read),
/// three words. Sets the colour at `011fa2d0`: red 1.0 when bit 0x20 of the byte +0x24 of
/// `owner`'s +0xa8 is set, else 0.0; green 1.0 with `green` set, else the default's green
/// (`011a9bd4`, copied as a word); blue the `float` +0x38 of `source` (or `01018f5c` without
/// one; through `FLD`/`FSTP`, a signalling NaN quieted); alpha the default's (`011a9bdc`,
/// a word).
pub fn fn_00c02d90(e: &mut Engine, _this: Ptr, owner: u32, green: u32, source: u32) {
    let default_green = e.global::<u32>(DEFAULT_COLOR_011A9BD0 + 4);
    let alpha = e.global::<u32>(DEFAULT_COLOR_011A9BD0 + 0xc);
    let flags = e.mem.u8(e.mem.u32(owner + 0xa8) + 0x24);
    let red = if flags & 0x20 != 0 { 0x3f80_0000 } else { 0 };
    let green = if green != 0 {
        0x3f80_0000
    } else {
        default_green
    };
    let blue = if source != 0 {
        e.mem.u32(source + 0x38)
    } else {
        e.global::<u32>(DEFAULT_BLUE_01018F5C)
    };
    let blue = x87_quiet_bits(blue);
    e.set_global(COLOR_011FA2D0 + 4, green);
    e.set_global(COLOR_011FA2D0, red);
    e.set_global(COLOR_011FA2D0 + 8, blue);
    e.set_global(COLOR_011FA2D0 + 0xc, alpha);
}

/// `BSShaderManager::GetAccumulator` (engine map, `00b4f5c0`, no arguments), the byte
/// `011f9427` and the word `011f95d4`.
const GET_ACCUMULATOR_00B4F5C0: u32 = 0x00b4_f5c0;
const FLAG_011F9427: u32 = 0x011f_9427;
const FALLBACK_011F95D4: u32 = 0x011f_95d4;

// Translated from 00c02f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`), `stdcall`, two words (an object, a word):
/// asks `GetAccumulator` (its +0x194 is read and not used), then the object's slot +0xf4
/// with 4 and the word; a null answer is replaced by the word at `011f95d4` (the byte
/// `011f9427` is compared and the result not used).
pub fn fn_00c02f30(e: &mut Engine, object: u32, word: u32) -> u32 {
    let accumulator = e.call(GET_ACCUMULATOR_00B4F5C0, &args![]).u32();
    // `MOV EDI,[EAX+0x194]`: a load the compiler kept, its value never used.
    let _ = e.mem.u32(accumulator + 0x194);
    let answer = e.vcall(object, 0xf4, &args![4u32, word]).u32();
    if answer != 0 {
        return answer;
    }
    let _ = e.global::<u8>(FLAG_011F9427);
    e.global::<u32>(FALLBACK_011F95D4)
}

/// The ambient colour the lights add to (`011fa0c0`, four `float`s), the colour that adds
/// nothing (`011f4998`, three `float`s), the HDR byte `011f941e`, the image-space manager
/// (`011f91ac`: its +8 list, effect 1's `pfData` +0x6c, whose +0xc scales) and the colour
/// `00c03180` sets (`011fa270`).
const AMBIENT_011FA0C0: u32 = 0x011f_a0c0;
const NEUTRAL_011F4998: u32 = 0x011f_4998;
const HDR_ENABLED_011F941E: u32 = 0x011f_941e;
const IMAGE_SPACE_MANAGER_011F91AC: u32 = 0x011f_91ac;
const COLOR_011FA270: u32 = 0x011f_a270;

/// A light's colour: its +0x34 pointer, or the colour at +0x28 when that is null.
fn light_color(e: &Engine, light: u32) -> u32 {
    let color = e.mem.u32(light + 0x34);
    if color != 0 {
        color
    } else {
        light + 0x28
    }
}

/// The HDR scale: the `float` +0xc of `pfData` of the manager's effect 1, times the light's
/// `float` +0x40 (`FLD`, `FMUL dword`).
fn hdr_scale(e: &Engine, light: u32) -> f64 {
    let manager = e.global::<u32>(IMAGE_SPACE_MANAGER_011F91AC);
    let effect = e.mem.u32(e.mem.u32(manager + 8) + 4);
    let data = e.mem.u32(effect + 0x6c);
    x87_mul(
        x87_load32(e.mem.u32(data + 0xc)),
        x87_m32(e.mem.u32(light + 0x40)),
    )
}

// Translated from 00c03010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`), `stdcall`, one word (a light). Nothing when the
/// light's colour equals the neutral colour at `011f4998` (`FUCOMPP`, all three, NaN never
/// equal). Else the colour is scaled: with the HDR byte `011f941e` by the HDR scale, without it
/// by the light's `float` +0x40 only when that is below 1.0 (`FCOM`); the scaled components
/// are stored as `float`s. Each is added to the ambient colour at `011fa0c0` (red, green,
/// blue; alpha written back unchanged) and, without HDR, each sum not below 1.0 (or NaN)
/// becomes 1.0.
pub fn fn_00c03010(e: &mut Engine, light: u32) {
    let color = light_color(e, light);
    let red = x87_load32(e.mem.u32(color));
    let green = x87_load32(e.mem.u32(color + 4));
    let blue = x87_load32(e.mem.u32(color + 8));
    let neutral =
        |e: &Engine, index: u32| x87_load32(e.global::<u32>(NEUTRAL_011F4998 + 4 * index));
    if neutral(e, 0) == red && neutral(e, 1) == green && neutral(e, 2) == blue {
        return;
    }
    let hdr = e.global::<u8>(HDR_ENABLED_011F941E) != 0;
    let scale = if hdr {
        Some(hdr_scale(e, light))
    } else {
        let fade = e.mem.u32(light + 0x40);
        if 1.0 > x87_m32(fade) {
            Some(x87_load32(fade))
        } else {
            None
        }
    };
    let (red, green, blue) = match scale {
        Some(scale) => {
            let scale = x87_load32(x87_store32(scale));
            let red = x87_store32(x87_mul(red, scale));
            let green = x87_store32(x87_mul(green, scale));
            let blue = x87_store32(x87_mul(blue, scale));
            (x87_load32(red), x87_load32(green), x87_load32(blue))
        }
        None => (red, green, blue),
    };
    let alpha = e.global::<u32>(AMBIENT_011FA0C0 + 0xc);
    let mut sums = [0u32; 3];
    for (index, value) in [red, green, blue].iter().enumerate() {
        let ambient = e.global::<u32>(AMBIENT_011FA0C0 + 4 * index as u32);
        sums[index] = x87_store32(x87_add(*value, x87_m32(ambient)));
    }
    if !hdr {
        for sum in sums.iter_mut() {
            // `FCOM` of 1.0 with the sum: not above it (or unordered) stores 1.0.
            if !(1.0 > x87_m32(*sum)) {
                *sum = 0x3f80_0000;
            }
        }
    }
    for (index, sum) in sums.iter().enumerate() {
        e.set_global(AMBIENT_011FA0C0 + 4 * index as u32, *sum);
    }
    e.set_global(AMBIENT_011FA0C0 + 0xc, alpha);
}

// Translated from 00c03180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`), `stdcall`, one word (a light): the light's
/// colour, scaled by the HDR scale when the HDR byte `011f941e` is set (each component
/// `FMUL`ed and stored as a `float`), goes to `011fa270` (through `FLD`/`FSTP`: a signalling
/// NaN of an unscaled component quieted) with 1.0 as the fourth component.
pub fn fn_00c03180(e: &mut Engine, light: u32) {
    let color = light_color(e, light);
    let mut components = [e.mem.u32(color), e.mem.u32(color + 4), e.mem.u32(color + 8)];
    if e.global::<u8>(HDR_ENABLED_011F941E) != 0 {
        let scale = x87_load32(x87_store32(hdr_scale(e, light)));
        for component in components.iter_mut() {
            *component = x87_store32(x87_mul(scale, x87_m32(*component)));
        }
    }
    for (index, component) in components.iter().enumerate() {
        e.set_global(
            COLOR_011FA270 + 4 * index as u32,
            x87_quiet_bits(*component),
        );
    }
    e.set_global(COLOR_011FA270 + 0xc, 0x3f80_0000u32);
}

/// The two default textures `00c03230` falls back on (`011f9518`, `011f9510`).
const TEXTURE_011F9518: u32 = 0x011f_9518;
const TEXTURE_011F9510: u32 = 0x011f_9510;

// Translated from 00c03230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`), `stdcall`, two words (an object, an index):
/// the word `index` of the object's +0xb0 array; when null, the word at `011f9518` if bit
/// 0x80 of the object's +0x20 is set, else the word at `011f9510`.
pub fn fn_00c03230(e: &mut Engine, object: u32, index: u32) -> u32 {
    let array = e.mem.u32(object + 0xb0);
    let texture = e.mem.u32(array.wrapping_add(index.wrapping_mul(4)));
    if texture != 0 {
        return texture;
    }
    if e.mem.u32(object + 0x20) & 0x80 != 0 {
        e.global::<u32>(TEXTURE_011F9518)
    } else {
        e.global::<u32>(TEXTURE_011F9510)
    }
}

/// The array setter `00ba3aa0` (`thiscall` on an `NiTPrimitiveArray`: index, the address of
/// the word), `ImageSpaceEffect::~ImageSpaceEffect` (engine map, `00ba3d30`),
/// `MemoryManager::Deallocate` (`00aa4060`, `thiscall` on the manager, one word) and the
/// `eh vector destructor iterator` (`00ec5fce`, `stdcall`: array, size, count, destructor).
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const EFFECT_DESTRUCT_00BA3D30: u32 = 0x00ba_3d30;
const MEMORY_DEALLOCATE_00AA4060: u32 = 0x00aa_4060;
const VECTOR_DESTRUCT_00EC5FCE: u32 = 0x00ec_5fce;

/// Element `index` of the `NiTPrimitiveArray` at `array` set to `value` through `00ba3aa0`
/// (the word in a local).
fn set_at(e: &mut Engine, array: u32, index: u32, value: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), value);
        e.call(ARRAY_SET_AT_00BA3AA0, &args![array, index, local.addr()]);
    });
}

/// Effect parameter `index` (`pEffectParamList`, elements at +0x1c), when set, deleted through
/// its slot +0 with 1 (the element tested twice, as the compiler has it) and the element set
/// to null.
fn delete_param(e: &mut Engine, this: u32, index: u32) {
    let element = e.mem.u32(this + 0x1c) + 4 * index;
    if e.mem.u32(element) != 0 {
        let param = e.mem.u32(element);
        if param != 0 {
            e.vcall(param, 0, &args![1u32]);
        }
        set_at(e, this + 0x18, index, 0);
    }
}

/// Texture `index` of `pTextures` (elements at +0x2c), when set, destroyed
/// (`~ImageSpaceTexture`) and freed (the element tested twice) and the element set to null.
fn delete_texture(e: &mut Engine, this: u32, index: u32) {
    let element = e.mem.u32(this + 0x2c) + 4 * index;
    if e.mem.u32(element) != 0 {
        let texture = e.mem.u32(element);
        if texture != 0 {
            e.call(TEXTURE_DESTRUCT_00BA3A00, &args![texture]);
            e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, texture]);
        }
        set_at(e, this + 0x28, index, 0);
    }
}

// Translated from 00c03270 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c03270 - the C++ exception frame (FS:[0] chain, unwind states 1 and 0 for the texture and the base) is not translated.
/// Unnamed in the engine map (`run_00c02ce0`): the destructor of the effect `00c033c0` builds,
/// `thiscall`, no stack words: the vtable `010bc8a4`, effect parameters 1 and 0 deleted, the
/// texture at +0x58 destroyed, then `ImageSpaceEffect::~ImageSpaceEffect`.
pub fn fn_00c03270(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010BC8A4);
    delete_param(e, base, 1);
    delete_param(e, base, 0);
    e.call(TEXTURE_DESTRUCT_00BA3A00, &args![base + 0x58]);
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

/// The byte `00c03340` answers (`012023f0`).
const ACTIVE_012023F0: u32 = 0x0120_23f0;

// Translated from 00c03340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`; Ghidra made no function of it): an `IsActive`
/// slot, `thiscall` (`this` not read), no stack words: the byte at `012023f0` (`AL` only).
pub fn fn_00c03340(e: &mut Engine, _this: Ptr) -> u8 {
    e.global::<u8>(ACTIVE_012023F0)
}

// Translated from 00c033f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`): the scalar deleting destructor of the effect
/// `00c033c0` builds, `thiscall`, one word: `00c03270`, then the memory freed when bit 0 of
/// `flags` is set. Answers `this`.
pub fn fn_00c033f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00c03270(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The effect-array sizer `00ba3f90` (`thiscall`, one word), `ImageSpaceShaderParam`'s
/// constructor (`00b8a9e0`), `InitConstantMap` (`00b8ab10`, two words) and
/// `SetPixelConstant` (`00b8a790`, five words), the input setters `00ba3b10` and `00ba3b50`
/// and the effect setter `00ba3c70` (unit `run_00ba3aa0`), and the `float`s `012023f4` and
/// `012023f8`.
const SIZE_ARRAYS_00BA3F90: u32 = 0x00ba_3f90;
const PARAM_CONSTRUCT_00B8A9E0: u32 = 0x00b8_a9e0;
const PARAM_INIT_CONSTANT_MAP_00B8AB10: u32 = 0x00b8_ab10;
const PARAM_SET_PIXEL_CONSTANT_00B8A790: u32 = 0x00b8_a790;
const SET_INPUT_00BA3B10: u32 = 0x00ba_3b10;
const SET_INPUTS_00BA3B50: u32 = 0x00ba_3b50;
const SET_EFFECT_00BA3C70: u32 = 0x00ba_3c70;
const CONSTANT_012023F4: u32 = 0x0120_23f4;
const CONSTANT_012023F8: u32 = 0x0120_23f8;

/// A parameter of 0x88 bytes from the memory manager (`ImageSpaceShaderParam`'s constructor
/// runs only on a block, else the parameter is null) with `InitConstantMap(0, map)`.
fn new_param(e: &mut Engine, map: u32) -> u32 {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x88u32])
        .u32();
    let param = if block != 0 {
        e.call(PARAM_CONSTRUCT_00B8A9E0, &args![block]).u32()
    } else {
        0
    };
    e.call(PARAM_INIT_CONSTANT_MAP_00B8AB10, &args![param, 0u32, map]);
    param
}

/// The manager's effect at `offset` of its +8 list.
fn manager_effect(e: &Engine, manager: u32, offset: u32) -> u32 {
    e.mem.u32(e.mem.u32(manager + 8) + offset)
}

// Translated from 00c03410 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c03410 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the two parameter allocations) is not translated.
/// Unnamed in the engine map (`run_00c02ce0`): a `Setup` slot, `thiscall`, two words (the
/// manager; a word it does not read). Sizes the arrays to 2 (`00ba3f90`) and `pTextures` to 3.
/// Parameter 0 (`InitConstantMap(0, 1)`) is stored, input 0 is texture 1 and effect 0 the
/// manager's +0x9c (output 2); parameter 1 (`InitConstantMap(0, 1)`) is stored, input 1 is
/// texture 2 and effect 1 the manager's +0xe4 (output 0); parameter 1's pixel constant 0 is
/// the `float`s at `012023f4` and `012023f8` (through `FLD`/`FSTP`), 0, 0.
pub fn fn_00c03410(e: &mut Engine, this: Ptr, manager: u32, _unused_1: u32) {
    let base = this.addr();
    e.call(SIZE_ARRAYS_00BA3F90, &args![base, 2u32]);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 3u32]);
    let first = new_param(e, 1);
    set_at(e, base + 0x18, 0, first);
    e.call(SET_INPUT_00BA3B10, &args![base, 0u32, 1u32, 0u32]);
    let effect = manager_effect(e, manager, 0x9c);
    e.call(SET_EFFECT_00BA3C70, &args![base, 0u32, effect, first, 2u32]);
    let second = new_param(e, 1);
    set_at(e, base + 0x18, 1, second);
    e.call(SET_INPUT_00BA3B10, &args![base, 1u32, 2u32, 0u32]);
    let effect = manager_effect(e, manager, 0xe4);
    e.call(
        SET_EFFECT_00BA3C70,
        &args![base, 1u32, effect, second, 0u32],
    );
    let x = x87_quiet_bits(e.global::<u32>(CONSTANT_012023F4));
    let y = x87_quiet_bits(e.global::<u32>(CONSTANT_012023F8));
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![second, 0u32, x, y, 0u32, 0u32],
    );
}

// Translated from 00c03580 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c03580 - the C++ exception frame (FS:[0] chain, unwind states 1 and 0 for the textures and the base) is not translated.
/// Unnamed in the engine map (`run_00c02ce0`): the destructor of the effect `00c03810` builds,
/// `thiscall`, no stack words: the vtable `010bc8d0`; textures 2 and 3 of `pTextures`
/// destroyed, freed and set to null; effect parameters 1 and 3 deleted; the two textures at
/// +0x5c destroyed (`eh vector destructor iterator`); `ImageSpaceEffect::~ImageSpaceEffect`.
pub fn fn_00c03580(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010BC8D0);
    delete_texture(e, base, 2);
    delete_texture(e, base, 3);
    delete_param(e, base, 1);
    delete_param(e, base, 3);
    e.call(
        VECTOR_DESTRUCT_00EC5FCE,
        &args![base + 0x5c, 0x10u32, 2u32, TEXTURE_DESTRUCT_00BA3A00],
    );
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

/// The two `float`s `00c036b0` tests (`01202400`, `0120240c`).
const STRENGTH_01202400: u32 = 0x0120_2400;
const STRENGTH_0120240C: u32 = 0x0120_240c;

// Translated from 00c036b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`; Ghidra made no function of it): an `IsActive`
/// slot, `thiscall` (`this` not read), no stack words: 1 when the `float` at `01202400` or the
/// one at `0120240c` is above 0 (`FCOM` with 0.0; NaN is not), else 0.
pub fn fn_00c036b0(e: &mut Engine, _this: Ptr) -> u32 {
    u32::from(0.0 < e.global::<f32>(STRENGTH_01202400) || 0.0 < e.global::<f32>(STRENGTH_0120240C))
}

// Translated from 00c036e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`; Ghidra made no function of it), `thiscall`, one
/// word: sets `bParamsChanged` (+5) and keeps the word at +0x58.
pub fn fn_00c036e0(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u8(this.addr() + 5, 1);
    e.mem.set_u32(this.addr() + 0x58, value);
}

/// `ImageSpaceTexture::BorrowRenderedTexture` (`00ba3840`, `thiscall` on the texture, one
/// word).
const TEXTURE_BORROW_00BA3840: u32 = 0x00ba_3840;

// Translated from 00c036f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`; Ghidra made no function of it): the
/// `BorrowTextures` slot of the effect `00c03810` builds, `thiscall`, one word it does not
/// read: its textures at +0x5c and +0x6c borrow rendered textures of kind 0x1b and become
/// textures 2 and 3 of `pTextures`.
pub fn fn_00c036f0(e: &mut Engine, this: Ptr, _unused_0: u32) {
    let base = this.addr();
    e.call(TEXTURE_BORROW_00BA3840, &args![base + 0x5c, 0x1bu32]);
    set_at(e, base + 0x28, 2, base + 0x5c);
    e.call(TEXTURE_BORROW_00BA3840, &args![base + 0x6c, 0x1bu32]);
    set_at(e, base + 0x28, 3, base + 0x6c);
}

/// The `float` `011f940c`, the `double`s `01012640` and `0104eea0`, the `float` `012023fc`,
/// CRT `cos` (`00ec9f70`) and `sin` (`00eca0a0`) as `bsshadermanager.rs` names them (argument
/// and result in `ST0`), the `float` `010162c0` and the level `012003c4`.
const TIME_011F940C: u32 = 0x011f_940c;
const DIVISOR_01012640: u32 = 0x0101_2640;
const FACTOR_0104EEA0: u32 = 0x0104_eea0;
const FACTOR_012023FC: u32 = 0x0120_23fc;
const CRT_COS_00EC9F70: u32 = 0x00ec_9f70;
const CRT_SIN_00ECA0A0: u32 = 0x00ec_a0a0;
const LEVEL_SOURCE_010162C0: u32 = 0x0101_62c0;
const LEVEL_012003C4: u32 = 0x0120_03c4;

/// `x * [01202400]` as the slot computes it: the CRT result stored as a `float` and loaded
/// again, `FMUL dword`, stored as a `float`.
fn scaled(e: &Engine, value: f64) -> u32 {
    let value = x87_load32(x87_store32(value));
    let strength = x87_m32(e.global::<u32>(STRENGTH_01202400));
    x87_store32(x87_mul(value, strength))
}

// Translated from 00c03740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`): the `UpdateParams` slot of the effect
/// `00c03810` builds, `thiscall`, one word it does not read. The angle is the `float`
/// `011f940c` divided by the `double` `01012640` and times the `double` `0104eea0`, stored as
/// a `float`. Effect parameter 3 takes pixel constant 0 = (0, 0, `0120240c` x `012023fc`,
/// `0120240c`) and constant 1 = (sin x `01202400`, cos x `01202400`, 1.0, 1.0) of the angle
/// (CRT `cos` asked first); `012003c4` takes the `float` `010162c0`. Answers true.
pub fn fn_00c03740(e: &mut Engine, this: Ptr, _unused_0: u32) -> bool {
    let time = x87_load32(e.global::<u32>(TIME_011F940C));
    let param = e.mem.u32(e.mem.u32(this.addr() + 0x1c) + 0xc);
    let divided = x87_div(time, e.global::<f64>(DIVISOR_01012640));
    let angle = x87_store32(x87_mul(divided, e.global::<f64>(FACTOR_0104EEA0)));
    let strength = x87_load32(e.global::<u32>(STRENGTH_0120240C));
    let fourth = x87_store32(strength);
    let third = x87_store32(x87_mul(strength, x87_m32(e.global::<u32>(FACTOR_012023FC))));
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![param, 0u32, 0u32, 0u32, third, fourth],
    );
    let cosine = e.call(CRT_COS_00EC9F70, &args![x87_load32(angle)]).f64();
    let second = scaled(e, cosine);
    let sine = e.call(CRT_SIN_00ECA0A0, &args![x87_load32(angle)]).f64();
    let first = scaled(e, sine);
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![param, 1u32, first, second, 0x3f80_0000u32, 0x3f80_0000u32],
    );
    let level = x87_quiet_bits(e.global::<u32>(LEVEL_SOURCE_010162C0));
    e.set_global(LEVEL_012003C4, level);
    true
}

// Translated from 00c03890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00c02ce0`): the scalar deleting destructor of the effect
/// `00c03810` builds, `thiscall`, one word: `00c03580`, then the memory freed when bit 0 of
/// `flags` is set. Answers `this`.
pub fn fn_00c03890(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00c03580(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00c038b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00c038b0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the parameter allocation) is not translated.
/// Unnamed in the engine map (`run_00c02ce0`): the `Setup` slot of the effect `00c03810`
/// builds, `thiscall`, two words (the manager; a word it does not read). Sizes the arrays and
/// `pTextures` to 4. Parameters 0 to 2 are null: input 0 texture 1 with the manager's +0x88
/// (output 2), inputs 1 and 2 textures (2, 3) and (2, 3) with the manager's +0x44 (output 2).
/// Parameter 3 (`InitConstantMap(0, 2)`, byte +0x56 cleared) is stored, input 3 is textures
/// (1, 2) and effect 3 the manager's +0xb0 (output 0).
pub fn fn_00c038b0(e: &mut Engine, this: Ptr, manager: u32, _unused_1: u32) {
    let base = this.addr();
    e.call(SIZE_ARRAYS_00BA3F90, &args![base, 4u32]);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 4u32]);
    set_at(e, base + 0x18, 0, 0);
    e.call(SET_INPUT_00BA3B10, &args![base, 0u32, 1u32, 0u32]);
    let effect = manager_effect(e, manager, 0x88);
    e.call(SET_EFFECT_00BA3C70, &args![base, 0u32, effect, 0u32, 2u32]);
    set_at(e, base + 0x18, 1, 0);
    e.call(
        SET_INPUTS_00BA3B50,
        &args![base, 1u32, 2u32, 3u32, 0u32, 0u32],
    );
    let effect = manager_effect(e, manager, 0x44);
    e.call(SET_EFFECT_00BA3C70, &args![base, 1u32, effect, 0u32, 2u32]);
    set_at(e, base + 0x18, 2, 0);
    e.call(
        SET_INPUTS_00BA3B50,
        &args![base, 2u32, 2u32, 3u32, 0u32, 0u32],
    );
    let effect = manager_effect(e, manager, 0x44);
    e.call(SET_EFFECT_00BA3C70, &args![base, 2u32, effect, 0u32, 2u32]);
    let param = new_param(e, 2);
    e.mem.set_u8(param + 0x56, 0);
    set_at(e, base + 0x18, 3, param);
    e.call(
        SET_INPUTS_00BA3B50,
        &args![base, 3u32, 1u32, 2u32, 0u32, 0u32],
    );
    let effect = manager_effect(e, manager, 0xb0);
    e.call(SET_EFFECT_00BA3C70, &args![base, 3u32, effect, param, 0u32]);
}

/// The renderer (`011f9508`) and the `float` 2^32 added to a negative `FILD` (`01099378`).
const RENDERER_011F9508: u32 = 0x011f_9508;
const TWO_TO_32_01099378: u32 = 0x0109_9378;

// Translated from 00c02d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall`, no stack words; answers a `float` (`ST0`). The
/// renderer's (`011f9508`) vtable +0xc8 gives an object whose vtable +0x90 (with 0) gives a
/// count; answers the `float` +0x5c (widened to a `double`) divided by the count (`FILD`, 2^32
/// added when negative, `FDIVR`), rounded to a `float`.
pub fn fn_00c02d40(e: &mut Engine, this: Ptr) -> f32 {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let object = e.vcall(renderer, 0xc8, &args![]).u32();
    let value = x87_load32(e.mem.u32(this.addr() + 0x5c));
    let count = e.vcall(object, 0x90, &args![0u32]).u32();
    let mut divisor = f64::from(count as i32);
    if (count as i32) < 0 {
        divisor = x87_add(divisor, x87_m32(e.global::<u32>(TWO_TO_32_01099378)));
    }
    f32::from_bits(x87_store32(x87_div(value, divisor)))
}

/// The current pass (`0126f74c`, stages at +0x24), the stage call `00e7de90` and
/// `NiD3DTextureStage::SetFilterMode` (`00e7def0`) (unnamed / engine map; `thiscall`, one word),
/// the pass type (`011f91e4`), the rendering record (`011f91e0`), `BSShaderLightingProperty`'s
/// `00b704b0` (`thiscall`, answers a `u16`) and `BSRenderedTexture::GetTexture` (`004bc320`,
/// `thiscall`, one word).
const CURRENT_PASS_0126F74C: u32 = 0x0126_f74c;
const STAGE_CALL_00E7DE90: u32 = 0x00e7_de90;
const SET_FILTER_MODE_00E7DEF0: u32 = 0x00e7_def0;
const PASS_TYPE_011F91E4: u32 = 0x011f_91e4;
const RENDERING_011F91E0: u32 = 0x011f_91e0;
const LIGHT_COUNT_00B704B0: u32 = 0x00b7_04b0;
const RENDERED_TEXTURE_GET_TEXTURE: u32 = 0x004b_c320;

// Translated from 00c02e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (vtable entries at `010af434`, `010bb8e4`, `010bbabc`,
/// `010bbc8c`, `010bc1ac`), `thiscall` with two words: a pass type and a shader property. Does
/// nothing for the types 0x1bd..0x1e1 other than 0x1c4..0x1cb, nor for 0x2d8..0x2ef. Otherwise
/// the current pass's (`0126f74c`, read at the start) first stage gets the property's first
/// texture (`[+0xac][0]`) and its second stage, when there is one, the first texture of
/// `[+0xb0]` (`011f9510` when null), each with the property's mode (vtable +0xe0, `00e7de90`).
/// For the pass types (`011f91e4`) 0x1c4..0x1c7 with lights (`00b704b0`), a rendering record
/// (`011f91e0`) whose object (`[+0xc][0]`) has the byte +0xec and a rendered texture +0x10c,
/// and a non-zero byte +9: the third stage gets that texture (`GetTexture(0)`), filter mode 1
/// and `00e7de90(0)`.
pub fn fn_00c02e20(e: &mut Engine, _this: Ptr, pass_type: u32, property: u32) {
    let pass = e.global::<u32>(CURRENT_PASS_0126F74C);
    if pass_type.wrapping_sub(0x1bd) <= 0x24
        && !(0x1c4..=0x1c7).contains(&pass_type)
        && pass_type.wrapping_sub(0x1c8) > 3
    {
        return;
    }
    if (0x2d8..=0x2ef).contains(&pass_type) {
        return;
    }
    let stages = e.mem.u32(pass + 0x24);
    let textures = e.mem.u32(property + 0xac);
    let texture = e.mem.u32(textures);
    let first = e.mem.u32(stages);
    e.mem.set_u32(first + 8, texture);
    let mode = e.vcall(property, 0xe0, &args![]).u32();
    e.call(STAGE_CALL_00E7DE90, &args![first, mode]);
    let stages = e.mem.u32(pass + 0x24);
    let second = e.mem.u32(stages + 4);
    if second != 0 {
        let textures = e.mem.u32(property + 0xb0);
        let mut texture = e.mem.u32(textures);
        if texture == 0 {
            texture = e.global::<u32>(TEXTURE_011F9510);
        }
        e.mem.set_u32(second + 8, texture);
        let mode = e.vcall(property, 0xe0, &args![]).u32();
        e.call(STAGE_CALL_00E7DE90, &args![second, mode]);
    }
    if e.global::<u32>(PASS_TYPE_011F91E4).wrapping_sub(0x1c4) > 3 {
        return;
    }
    let lights = e.call(LIGHT_COUNT_00B704B0, &args![property]).u16();
    if lights == 0 {
        return;
    }
    let rendering = e.global::<u32>(RENDERING_011F91E0);
    let holder = e.mem.u32(rendering + 0xc);
    let object = e.mem.u32(holder);
    if e.mem.u8(object + 0xec) == 0 {
        return;
    }
    let rendered = e.mem.u32(object + 0x10c);
    if rendered == 0 || e.mem.u8(rendering + 9) == 0 {
        return;
    }
    let stages = e.mem.u32(pass + 0x24);
    let third = e.mem.u32(stages + 8);
    let texture = e
        .call(RENDERED_TEXTURE_GET_TEXTURE, &args![rendered, 0u32])
        .u32();
    e.mem.set_u32(third + 8, texture);
    e.call(SET_FILTER_MODE_00E7DEF0, &args![third, 1u32]);
    e.call(STAGE_CALL_00E7DE90, &args![third, 0u32]);
}

/// `00ba39a0` (unnamed, `thiscall`, no stack words), on the rendered-texture member at +0x58.
const TEXTURE_RETURN_00BA39A0: u32 = 0x00ba_39a0;

// Translated from 00c03260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010bc8b8`), `thiscall`,
/// no stack words: a jump to `00ba39a0` on the member at +0x58.
pub fn fn_00c03260(e: &mut Engine, this: Ptr) {
    e.call(TEXTURE_RETURN_00BA39A0, &args![this.addr() + 0x58]);
}

// Translated from 00c03350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010bc8c0`), `thiscall`
/// with one unread word; answers false. The second shader parameter of the array at +0x18
/// (elements at +0x1c) gets pixel constant 0 (`012023f4`, `012023f8` through `FLD`/`FSTP`, 0.0,
/// 0.0), and the word `011ae680` becomes 0x80.
pub fn fn_00c03350(e: &mut Engine, this: Ptr, _unused_0: u32) -> bool {
    let parameters = e.mem.u32(this.addr() + 0x1c);
    let parameter = e.mem.u32(parameters + 4);
    let first = x87_quiet_bits(e.global::<u32>(CONSTANT_012023F4));
    let second = x87_quiet_bits(e.global::<u32>(CONSTANT_012023F8));
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![parameter, 0u32, first, second, 0u32, 0u32],
    );
    e.set_global(STATE_VALUE_011AE680, 0x80u32);
    false
}

/// The word `00c03350` sets (`011ae680`).
const STATE_VALUE_011AE680: u32 = 0x011a_e680;

// Translated from 00c03390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (not a Ghidra function; vtable entry at `010bc8b4`), `thiscall`
/// with one unread word: runs `00ba3840(3)` on the member at +0x58 and makes the member's
/// address element 2 of the array at +0x28 (`00ba3aa0`, through a stack word).
pub fn fn_00c03390(e: &mut Engine, this: Ptr, _unused_0: u32) {
    let base = this.addr();
    e.call(TEXTURE_BORROW_00BA3840, &args![base + 0x58, 3u32]);
    set_at(e, base + 0x28, 2, base + 0x58);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c0_2d40, fn_00c02d40(Ptr) -> f32),
        entry!(0x00c0_2e20, fn_00c02e20(Ptr, u32, u32)),
        entry!(0x00c0_3260, fn_00c03260(Ptr)),
        entry!(0x00c0_3350, fn_00c03350(Ptr, u32) -> bool),
        entry!(0x00c0_3390, fn_00c03390(Ptr, u32)),
        entry!(0x00c0_33c0, fn_00c033c0(Ptr) -> Ptr),
        entry!(0x00c0_3810, fn_00c03810(Ptr) -> Ptr),
        entry!(0x00c0_2ce0, fn_00c02ce0(u32)),
        entry!(0x00c0_2d90, fn_00c02d90(Ptr, u32, u32, u32)),
        entry!(0x00c0_2f30, fn_00c02f30(u32, u32) -> u32),
        entry!(0x00c0_2f70, fn_00c02f70(u32)),
        entry!(0x00c0_3010, fn_00c03010(u32)),
        entry!(0x00c0_3180, fn_00c03180(u32)),
        entry!(0x00c0_3230, fn_00c03230(u32, u32) -> u32),
        entry!(0x00c0_3270, fn_00c03270(Ptr)),
        entry!(0x00c0_3340, fn_00c03340(Ptr) -> u8),
        entry!(0x00c0_33f0, fn_00c033f0(Ptr, u32) -> Ptr),
        entry!(0x00c0_3410, fn_00c03410(Ptr, u32, u32)),
        entry!(0x00c0_3580, fn_00c03580(Ptr)),
        entry!(0x00c0_36b0, fn_00c036b0(Ptr) -> u32),
        entry!(0x00c0_36e0, fn_00c036e0(Ptr, u32)),
        entry!(0x00c0_36f0, fn_00c036f0(Ptr, u32)),
        entry!(0x00c0_3740, fn_00c03740(Ptr, u32) -> bool),
        entry!(0x00c0_3890, fn_00c03890(Ptr, u32) -> Ptr),
        entry!(0x00c0_38b0, fn_00c038b0(Ptr, u32, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for target in [
            EFFECT_CONSTRUCT_00BA4080,
            ARRAY_SET_SIZE_00A90180,
            VECTOR_CONSTRUCT_00EC782F,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00c033c0_builds_the_effect() {
        let mut e = engine();
        let this = e.mem.alloc(0x68);
        e.mem.write(this, &[0xcc; 0x68]);
        assert_eq!(e.call(0x00c0_33c0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BC8A4);
        assert_eq!(e.mem.bytes(this + 0x58, 3), vec![0; 3]);
        assert_eq!(e.mem.u8(this + 0x5b), 0xcc);
        assert_eq!(e.mem.bytes(this + 0x5c, 0xc), vec![0; 0xc]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![(EFFECT_CONSTRUCT_00BA4080, vec![this])]
        );
    }

    #[test]
    fn fn_00c03810_builds_the_effect() {
        let mut e = engine();
        let this = e.mem.alloc(0x7c);
        assert_eq!(e.call(0x00c0_3810, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BC8D0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (
                    VECTOR_CONSTRUCT_00EC782F,
                    vec![
                        this + 0x5c,
                        0x10,
                        2,
                        TEXTURE_CONSTRUCT_004EBB70,
                        TEXTURE_DESTRUCT_00BA3A00
                    ]
                ),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 4]),
            ]
        );
    }
}

#[cfg(test)]
mod pass_flag_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    /// The two flag tables filled with objects (bytes +8 at 0xcc), the pass's two words set.
    fn flags_rig(pass: u32, first: u32, second: u32) -> (Engine, Vec<u32>) {
        let mut e = rig();
        e.map(0x011f_c000, 0x3000);
        let objects: Vec<u32> = (0..44)
            .map(|_| {
                let object = e.mem.alloc(0xc);
                e.mem.set_u8(object + 8, 0xcc);
                object
            })
            .collect();
        for (index, object) in objects.iter().enumerate() {
            e.mem
                .set_u32(FIRST_FLAGS_011FEBD8 + 4 * index as u32, *object);
        }
        e.mem.set_u32(FIRST_MASKS_011FCC80 + 4 * pass, first);
        e.mem.set_u32(SECOND_MASKS_011FC0A0 + 4 * pass, second);
        (e, objects)
    }

    #[test]
    fn fn_00c02f70_sets_each_flag_from_its_bit() {
        let (mut e, objects) = flags_rig(5, 0b1010, 0x0040_0002);
        // A null object in the first table is skipped (its bit is not used by the next).
        e.mem.set_u32(FIRST_FLAGS_011FEBD8 + 4 * 2, 0);
        e.call(0x00c0_2f70, &args![5u32]);
        let bytes: Vec<u8> = objects.iter().map(|o| e.mem.u8(o + 8)).collect();
        assert_eq!(&bytes[..4], &[1, 0, 0xcc, 0]);
        assert!(bytes[4..22].iter().all(|b| *b == 0));
        assert_eq!(bytes[22], 1);
        assert!(bytes[23..43].iter().all(|b| *b == 0));
        assert_eq!(bytes[43], 1);
    }

    #[test]
    fn fn_00c02ce0_sets_the_pass_flags_and_the_two_objects() {
        let (mut e, objects) = flags_rig(0xc1, 0, 0);
        e.call(0x00c0_2ce0, &args![0xc1u32]);
        // 011fec6c is object 15 of the second table (index 37 overall), 011fec44 object 5.
        assert_eq!(e.mem.u8(objects[37] + 8), 1);
        assert_eq!(e.mem.u8(objects[27] + 8), 0);
        let (mut e, objects) = flags_rig(0x10, 0, 0xffff_ffff);
        e.call(0x00c0_2ce0, &args![0x10u32]);
        assert_eq!(e.mem.u8(objects[37] + 8), 0);
        assert_eq!(e.mem.u8(objects[27] + 8), 1);
    }

    #[test]
    fn fn_00c02d90_sets_the_colour() {
        let mut e = Engine::new();
        for page in [0x011a_9000u32, 0x0101_8000, 0x011f_a000] {
            e.map(page, 0x1000);
        }
        e.set_global(DEFAULT_COLOR_011A9BD0, 0.25f32);
        e.set_global(DEFAULT_COLOR_011A9BD0 + 4, 0.5f32);
        e.set_global(DEFAULT_COLOR_011A9BD0 + 8, 0.75f32);
        e.set_global(DEFAULT_COLOR_011A9BD0 + 0xc, 0x7f80_0001u32);
        e.set_global(DEFAULT_BLUE_01018F5C, 0x7f80_0002u32);
        let owner = e.mem.alloc(0xb0);
        let inner = e.mem.alloc(0x28);
        e.mem.set_u32(owner + 0xa8, inner);
        e.call(0x00c0_2d90, &args![0u32, owner, 0u32, 0u32]);
        let color = |e: &Engine| -> Vec<u32> {
            (0..4)
                .map(|i| e.global::<u32>(COLOR_011FA2D0 + 4 * i))
                .collect()
        };
        assert_eq!(
            color(&e),
            vec![0, 0.5f32.to_bits(), 0x7fc0_0002, 0x7f80_0001]
        );
        e.mem.set_u8(inner + 0x24, 0x20);
        let source = e.mem.alloc(0x40);
        e.mem.set_f32(source + 0x38, 3.0);
        e.call(0x00c0_2d90, &args![0u32, owner, 1u32, source]);
        assert_eq!(
            color(&e),
            vec![
                1.0f32.to_bits(),
                1.0f32.to_bits(),
                3.0f32.to_bits(),
                0x7f80_0001
            ]
        );
    }

    #[test]
    fn fn_00c02f30_falls_back_on_a_null_answer() {
        let mut e = rig();
        e.map(0x011f_9000, 0x1000);
        e.set_global(FALLBACK_011F95D4, 0x4444u32);
        let accumulator = e.mem.alloc(0x198);
        e.register_double(GET_ACCUMULATOR_00B4F5C0, move |_, _| accumulator.into_ret());
        e.put_vtable(0x0e60_0000, &[0; 0x3e]);
        e.mem.set_u32(0x0e60_0000 + 0xf4, 0x0e61_00f4);
        e.register(0x0e61_00f4, |_, _| Ret::default());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, 0x0e60_0000);
        assert_eq!(e.call(0x00c0_2f30, &args![object, 9u32]).u32(), 0x4444);
        assert_eq!(calls(&e, 0x0e61_00f4), vec![vec![object, 4, 9]]);
        e.register(0x0e61_00f4, |_, _| 0x77u32.into_ret());
        assert_eq!(e.call(0x00c0_2f30, &args![object, 9u32]).u32(), 0x77);
    }

    /// A light with its colour inline at +0x28, the given fade at +0x40, and the globals.
    fn light_rig(color: [f32; 3], fade: f32, hdr: Option<f32>) -> (Engine, u32) {
        let mut e = Engine::new();
        for page in [0x011f_4000u32, 0x011f_9000, 0x011f_a000] {
            e.map(page, 0x1000);
        }
        for (index, value) in [0.0f32, 0.0, 0.0].iter().enumerate() {
            e.set_global(NEUTRAL_011F4998 + 4 * index as u32, *value);
        }
        for (index, value) in [0.25f32, 0.5, 0.75, 0.125].iter().enumerate() {
            e.set_global(AMBIENT_011FA0C0 + 4 * index as u32, *value);
        }
        if let Some(scale) = hdr {
            e.set_global(HDR_ENABLED_011F941E, 1u8);
            let manager = e.mem.alloc(0x10);
            let list = e.mem.alloc(8);
            let effect = e.mem.alloc(0x70);
            let data = e.mem.alloc(0x10);
            e.mem.set_f32(data + 0xc, scale);
            e.mem.set_u32(effect + 0x6c, data);
            e.mem.set_u32(list + 4, effect);
            e.mem.set_u32(manager + 8, list);
            e.set_global(IMAGE_SPACE_MANAGER_011F91AC, manager);
        }
        let light = e.mem.alloc(0x44);
        for (index, value) in color.iter().enumerate() {
            e.mem.set_f32(light + 0x28 + 4 * index as u32, *value);
        }
        e.mem.set_f32(light + 0x40, fade);
        (e, light)
    }

    fn ambient(e: &Engine) -> Vec<f32> {
        (0..4)
            .map(|i| e.global::<f32>(AMBIENT_011FA0C0 + 4 * i))
            .collect()
    }

    #[test]
    fn fn_00c03010_adds_the_faded_colour_and_clamps() {
        let (mut e, light) = light_rig([0.5, 0.25, 1.0], 0.5, None);
        e.call(0x00c0_3010, &args![light]);
        assert_eq!(ambient(&e), vec![0.5, 0.625, 1.0, 0.125]);
        // A fade of 1.0 or more leaves the colour unscaled.
        let (mut e, light) = light_rig([0.5, 0.25, 0.125], 2.0, None);
        e.call(0x00c0_3010, &args![light]);
        assert_eq!(ambient(&e), vec![0.75, 0.75, 0.875, 0.125]);
    }

    #[test]
    fn fn_00c03010_scales_by_hdr_without_clamping_and_skips_the_neutral_colour() {
        let (mut e, light) = light_rig([1.0, 2.0, 4.0], 0.5, Some(4.0));
        e.call(0x00c0_3010, &args![light]);
        assert_eq!(ambient(&e), vec![2.25, 4.5, 8.75, 0.125]);
        let (mut e, light) = light_rig([0.0, 0.0, 0.0], 0.5, None);
        e.call(0x00c0_3010, &args![light]);
        assert_eq!(ambient(&e), vec![0.25, 0.5, 0.75, 0.125]);
        // A colour behind a pointer.
        let (mut e, light) = light_rig([0.0, 0.0, 0.0], 3.0, None);
        let color = e.mem.alloc(0xc);
        e.mem.set_f32(color, 2.0);
        e.mem.set_u32(light + 0x34, color);
        e.call(0x00c0_3010, &args![light]);
        assert_eq!(ambient(&e), vec![1.0, 0.5, 0.75, 0.125]);
    }

    #[test]
    fn fn_00c03180_sets_the_light_colour() {
        let (mut e, light) = light_rig([0.5, 0.25, 1.0], 0.5, Some(4.0));
        e.call(0x00c0_3180, &args![light]);
        let color: Vec<f32> = (0..4)
            .map(|i| e.global::<f32>(COLOR_011FA270 + 4 * i))
            .collect();
        assert_eq!(color, vec![1.0, 0.5, 2.0, 1.0]);
        let (mut e, light) = light_rig([0.5, 0.25, 1.0], 0.5, None);
        e.mem.set_u32(light + 0x30, 0x7f80_0005);
        e.call(0x00c0_3180, &args![light]);
        assert_eq!(e.global::<f32>(COLOR_011FA270), 0.5);
        assert_eq!(e.global::<u32>(COLOR_011FA270 + 8), 0x7fc0_0005);
    }

    #[test]
    fn fn_00c03230_falls_back_on_the_default_textures() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.set_global(TEXTURE_011F9518, 0x9518u32);
        e.set_global(TEXTURE_011F9510, 0x9510u32);
        let object = e.mem.alloc(0xb4);
        let array = e.mem.alloc(8);
        e.mem.set_u32(array + 4, 0x1234);
        e.mem.set_u32(object + 0xb0, array);
        assert_eq!(e.call(0x00c0_3230, &args![object, 1u32]).u32(), 0x1234);
        assert_eq!(e.call(0x00c0_3230, &args![object, 0u32]).u32(), 0x9510);
        e.mem.set_u32(object + 0x20, 0x80);
        assert_eq!(e.call(0x00c0_3230, &args![object, 0u32]).u32(), 0x9518);
    }

    #[test]
    fn fn_00c03340_and_00c036b0_answer_the_globals() {
        let mut e = Engine::new();
        e.map(0x0120_2000, 0x1000);
        assert_eq!(e.call(0x00c0_3340, &args![0u32]).u32() & 0xff, 0);
        e.set_global(ACTIVE_012023F0, 1u8);
        assert_eq!(e.call(0x00c0_3340, &args![0u32]).u32() & 0xff, 1);
        assert_eq!(e.call(0x00c0_36b0, &args![0u32]).u32(), 0);
        e.set_global(STRENGTH_0120240C, 0.5f32);
        assert_eq!(e.call(0x00c0_36b0, &args![0u32]).u32(), 1);
        e.set_global(STRENGTH_0120240C, f32::NAN);
        e.set_global(STRENGTH_01202400, 0.25f32);
        assert_eq!(e.call(0x00c0_36b0, &args![0u32]).u32(), 1);
        e.set_global(STRENGTH_01202400, -1.0f32);
        assert_eq!(e.call(0x00c0_36b0, &args![0u32]).u32(), 0);
    }

    #[test]
    fn fn_00c036e0_keeps_the_word_and_marks_the_change() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x5c);
        e.call(0x00c0_36e0, &args![this, 0x55u32]);
        assert_eq!(e.mem.u8(this + 5), 1);
        assert_eq!(e.mem.u32(this + 0x58), 0x55);
    }

    const PARAM_DELETE: u32 = 0x0e62_0000;
    const PARAM_VTABLE: u32 = 0x0e63_0000;

    /// An effect with parameters 0, 1 and 3 (deletable objects) and textures 2 and 3.
    fn effect_rig() -> (Engine, u32, [u32; 4], [u32; 4]) {
        let mut e = rig();
        for callee in [
            ARRAY_SET_AT_00BA3AA0,
            EFFECT_DESTRUCT_00BA3D30,
            MEMORY_DEALLOCATE_00AA4060,
            TEXTURE_DESTRUCT_00BA3A00,
            VECTOR_DESTRUCT_00EC5FCE,
            PARAM_DELETE,
            TEXTURE_BORROW_00BA3840,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.put_vtable(PARAM_VTABLE, &[PARAM_DELETE]);
        let this = e.mem.alloc(0x80);
        let params_list = e.mem.alloc(0x10);
        let mut params = [0u32; 4];
        for index in [0usize, 1, 3] {
            let param = e.mem.alloc(8);
            e.mem.set_u32(param, PARAM_VTABLE);
            e.mem.set_u32(params_list + 4 * index as u32, param);
            params[index] = param;
        }
        e.mem.set_u32(this + 0x1c, params_list);
        let textures_list = e.mem.alloc(0x10);
        let mut textures = [0u32; 4];
        for index in [2usize, 3] {
            let texture = e.mem.alloc(0x10);
            e.mem.set_u32(textures_list + 4 * index as u32, texture);
            textures[index] = texture;
        }
        e.mem.set_u32(this + 0x2c, textures_list);
        (e, this, params, textures)
    }

    fn set_ats(e: &Engine) -> Vec<(u32, u32)> {
        calls(e, ARRAY_SET_AT_00BA3AA0)
            .iter()
            .map(|c| (c[0], c[1]))
            .collect()
    }

    #[test]
    fn fn_00c03270_and_00c033f0_delete_parameters_1_and_0() {
        let (mut e, this, params, _) = effect_rig();
        e.call(0x00c0_3270, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010BC8A4);
        assert_eq!(
            calls(&e, PARAM_DELETE),
            vec![vec![params[1], 1], vec![params[0], 1]]
        );
        assert_eq!(set_ats(&e), vec![(this + 0x18, 1), (this + 0x18, 0)]);
        assert_eq!(
            calls(&e, TEXTURE_DESTRUCT_00BA3A00),
            vec![vec![this + 0x58]]
        );
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        assert_eq!(e.call(0x00c0_33f0, &args![this, 1u32]).u32(), this);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }

    #[test]
    fn fn_00c03580_and_00c03890_free_textures_and_parameters() {
        let (mut e, this, params, textures) = effect_rig();
        e.call(0x00c0_3580, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010BC8D0);
        assert_eq!(
            calls(&e, TEXTURE_DESTRUCT_00BA3A00),
            vec![vec![textures[2]], vec![textures[3]]]
        );
        assert_eq!(
            calls(&e, PARAM_DELETE),
            vec![vec![params[1], 1], vec![params[3], 1]]
        );
        assert_eq!(
            set_ats(&e),
            vec![
                (this + 0x28, 2),
                (this + 0x28, 3),
                (this + 0x18, 1),
                (this + 0x18, 3)
            ]
        );
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT_00EC5FCE),
            vec![vec![this + 0x5c, 0x10, 2, TEXTURE_DESTRUCT_00BA3A00]]
        );
        assert_eq!(e.call(0x00c0_3890, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, MEMORY_DEALLOCATE_00AA4060)
            .iter()
            .all(|c| c[1] != this));
    }

    #[test]
    fn fn_00c036f0_borrows_both_textures() {
        type Stored = std::rc::Rc<std::cell::RefCell<Vec<(u32, u32, u32)>>>;
        let (mut e, this, _, _) = effect_rig();
        let stored: Stored = Default::default();
        let log = stored.clone();
        e.register_double(ARRAY_SET_AT_00BA3AA0, move |e, a| {
            log.borrow_mut().push((a[0], a[1], e.mem.u32(a[2])));
            Ret::default()
        });
        e.call(0x00c0_36f0, &args![this, 0u32]);
        assert_eq!(
            calls(&e, TEXTURE_BORROW_00BA3840),
            vec![vec![this + 0x5c, 0x1b], vec![this + 0x6c, 0x1b]]
        );
        assert_eq!(
            *stored.borrow(),
            vec![(this + 0x28, 2, this + 0x5c), (this + 0x28, 3, this + 0x6c)]
        );
    }

    /// The set-ups' rig: parameters from 0x0e64_0100 in steps of 0x100, a manager whose
    /// effects are their offsets plus 0x0e65_0000.
    fn setup_rig() -> (Engine, u32, u32) {
        let mut e = rig();
        for page in [0x0120_2000u32, 0x0e64_0000] {
            e.map(page, 0x1000);
        }
        e.mem.write(0x0e64_0000, &[0xcc; 0x1000]);
        e.set_global(CONSTANT_012023F4, 0.5f32);
        e.set_global(CONSTANT_012023F8, 0x7f80_0004u32);
        let mut next = 0x0e64_0000u32;
        e.register_double(PARAM_CONSTRUCT_00B8A9E0, move |_, _| {
            next += 0x100;
            next.into_ret()
        });
        for callee in [
            SIZE_ARRAYS_00BA3F90,
            ARRAY_SET_SIZE_00A90180,
            PARAM_INIT_CONSTANT_MAP_00B8AB10,
            PARAM_SET_PIXEL_CONSTANT_00B8A790,
            SET_EFFECT_00BA3C70,
            ARRAY_SET_AT_00BA3AA0,
            SET_INPUT_00BA3B10,
            SET_INPUTS_00BA3B50,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x80);
        let manager = e.mem.alloc(0x10);
        let effects = e.mem.alloc(0x100);
        for offset in (0..0x100u32).step_by(4) {
            e.mem.set_u32(effects + offset, 0x0e65_0000 + offset);
        }
        e.mem.set_u32(manager + 8, effects);
        (e, this, manager)
    }

    #[test]
    fn fn_00c03410_sets_up_two_parameters() {
        let (mut e, this, manager) = setup_rig();
        e.call(0x00c0_3410, &args![this, manager, 0u32]);
        let (first, second) = (0x0e64_0100, 0x0e64_0200);
        assert_eq!(calls(&e, SIZE_ARRAYS_00BA3F90), vec![vec![this, 2]]);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + 0x28, 3]]
        );
        assert_eq!(
            calls(&e, PARAM_INIT_CONSTANT_MAP_00B8AB10),
            vec![vec![first, 0, 1], vec![second, 0, 1]]
        );
        assert_eq!(set_ats(&e), vec![(this + 0x18, 0), (this + 0x18, 1)]);
        assert_eq!(
            calls(&e, SET_INPUT_00BA3B10),
            vec![vec![this, 0, 1, 0], vec![this, 1, 2, 0]]
        );
        assert_eq!(
            calls(&e, SET_EFFECT_00BA3C70),
            vec![
                vec![this, 0, 0x0e65_009c, first, 2],
                vec![this, 1, 0x0e65_00e4, second, 0]
            ]
        );
        assert_eq!(
            calls(&e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![vec![second, 0, 0.5f32.to_bits(), 0x7fc0_0004, 0, 0]]
        );
    }

    #[test]
    fn fn_00c038b0_sets_up_four_effects() {
        let (mut e, this, manager) = setup_rig();
        e.call(0x00c0_38b0, &args![this, manager, 0u32]);
        let param = 0x0e64_0100;
        assert_eq!(calls(&e, SIZE_ARRAYS_00BA3F90), vec![vec![this, 4]]);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + 0x28, 4]]
        );
        assert_eq!(
            set_ats(&e),
            vec![
                (this + 0x18, 0),
                (this + 0x18, 1),
                (this + 0x18, 2),
                (this + 0x18, 3)
            ]
        );
        assert_eq!(calls(&e, SET_INPUT_00BA3B10), vec![vec![this, 0, 1, 0]]);
        assert_eq!(
            calls(&e, SET_INPUTS_00BA3B50),
            vec![
                vec![this, 1, 2, 3, 0, 0],
                vec![this, 2, 2, 3, 0, 0],
                vec![this, 3, 1, 2, 0, 0]
            ]
        );
        assert_eq!(
            calls(&e, SET_EFFECT_00BA3C70),
            vec![
                vec![this, 0, 0x0e65_0088, 0, 2],
                vec![this, 1, 0x0e65_0044, 0, 2],
                vec![this, 2, 0x0e65_0044, 0, 2],
                vec![this, 3, 0x0e65_00b0, param, 0]
            ]
        );
        assert_eq!(
            calls(&e, PARAM_INIT_CONSTANT_MAP_00B8AB10),
            vec![vec![param, 0, 2]]
        );
        assert_eq!(e.mem.u8(param + 0x56), 0);
        assert_eq!(e.mem.u8(param + 0x55), 0xcc);
    }

    #[test]
    fn fn_00c03740_sets_the_wave_constants() {
        let mut e = rig();
        for page in [
            0x011f_9000u32,
            0x0101_2000,
            0x0104_e000,
            0x0120_2000,
            0x0101_6000,
            0x0120_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(TIME_011F940C, 3.0f32);
        e.set_global(DIVISOR_01012640, 2.0f64);
        e.set_global(FACTOR_0104EEA0, 0.5f64);
        e.set_global(STRENGTH_0120240C, 2.0f32);
        e.set_global(FACTOR_012023FC, 0.25f32);
        e.set_global(STRENGTH_01202400, 4.0f32);
        e.set_global(LEVEL_SOURCE_010162C0, 0x7f80_0006u32);
        e.register(CRT_COS_00EC9F70, |_, a| Ret {
            st0: f64::take(a, &mut 0) + 0.25,
            ..Ret::default()
        });
        e.register(CRT_SIN_00ECA0A0, |_, a| Ret {
            st0: f64::take(a, &mut 0) - 0.25,
            ..Ret::default()
        });
        e.register(PARAM_SET_PIXEL_CONSTANT_00B8A790, |_, _| Ret::default());
        let this = e.mem.alloc(0x80);
        let params = e.mem.alloc(0x10);
        e.mem.set_u32(params + 0xc, 0x0e66_0000);
        e.mem.set_u32(this + 0x1c, params);
        assert!(e.call(0x00c0_3740, &args![this, 0u32]).bool());
        // The angle is 3 / 2 * 0.5 = 0.75: the doubles answer cos 1.0 and sin 0.5, times 4.
        let angle = 0.75f64.to_bits();
        assert_eq!(
            calls(&e, CRT_COS_00EC9F70),
            vec![vec![angle as u32, (angle >> 32) as u32]]
        );
        let bits = |v: f32| v.to_bits();
        assert_eq!(
            calls(&e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![
                vec![0x0e66_0000, 0, 0, 0, bits(0.5), bits(2.0)],
                vec![0x0e66_0000, 1, bits(2.0), bits(4.0), bits(1.0), bits(1.0)],
            ]
        );
        assert_eq!(e.global::<u32>(LEVEL_012003C4), 0x7fc0_0006);
    }
}

#[cfg(test)]
mod n071_tests {
    use super::*;
    use crate::units::bsshader::shadowlightshader::n071_rig::{calls, engine, vobject};

    #[test]
    fn fn_00c02d40_divides_by_the_count() {
        let mut e = engine(&[]);
        e.map(0x011f_9000, 0x1000);
        e.map(0x0109_9000, 0x1000);
        e.set_global(TWO_TO_32_01099378, 4_294_967_296.0f32);
        let counter = vobject(&mut e, 8, &[(0x90, 4)]);
        let renderer = vobject(&mut e, 8, &[(0xc8, counter)]);
        e.set_global(RENDERER_011F9508, renderer);
        let this = e.mem.alloc(0x60);
        e.mem.set_f32(this + 0x5c, 10.0);
        assert_eq!(e.call(0x00c0_2d40, &args![this]).f32(), 2.5);
        let counter = vobject(&mut e, 8, &[(0x90, 0x8000_0000)]);
        let renderer = vobject(&mut e, 8, &[(0xc8, counter)]);
        e.set_global(RENDERER_011F9508, renderer);
        e.mem.set_f32(this + 0x5c, 2_147_483_648.0);
        assert_eq!(e.call(0x00c0_2d40, &args![this]).f32(), 1.0);
    }

    /// A current pass with three stages, a property (mode 9) with textures at +0xac and +0xb0.
    fn stage_rig() -> (Engine, Vec<u32>, u32) {
        let mut e = engine(&[STAGE_CALL_00E7DE90, SET_FILTER_MODE_00E7DEF0]);
        for page in [0x011f_9000u32, 0x0126_f000] {
            e.map(page, 0x1000);
        }
        let stages: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x10)).collect();
        let array = e.mem.alloc(0xc);
        for (i, s) in stages.iter().enumerate() {
            e.mem.set_u32(array + 4 * i as u32, *s);
        }
        let pass = e.mem.alloc(0x30);
        e.mem.set_u32(pass + 0x24, array);
        e.set_global(CURRENT_PASS_0126F74C, pass);
        let property = vobject(&mut e, 0xb8, &[(0xe0, 9)]);
        for offset in [0xacu32, 0xb0] {
            let textures = e.mem.alloc(4);
            e.mem.set_u32(property + offset, textures);
        }
        let first = e.mem.u32(property + 0xac);
        e.mem.set_u32(first, 0xac00);
        e.set_global(TEXTURE_011F9510, 0x9510u32);
        (e, stages, property)
    }

    #[test]
    fn fn_00c02e20_skips_the_excluded_types() {
        let (mut e, _, property) = stage_rig();
        for pass_type in [0x1bdu32, 0x1c3, 0x1cc, 0x1e1, 0x2d8, 0x2ef] {
            e.call(0x00c0_2e20, &args![0u32, pass_type, property]);
        }
        assert!(calls(&e, STAGE_CALL_00E7DE90).is_empty());
    }

    #[test]
    fn fn_00c02e20_sets_the_stages() {
        let (mut e, stages, property) = stage_rig();
        e.call(0x00c0_2e20, &args![0u32, 0x1c5u32, property]);
        assert_eq!(e.mem.u32(stages[0] + 8), 0xac00);
        assert_eq!(e.mem.u32(stages[1] + 8), 0x9510);
        assert_eq!(
            calls(&e, STAGE_CALL_00E7DE90),
            vec![vec![stages[0], 9], vec![stages[1], 9]]
        );
        // Pass type 0x1c4 with lights and a rendered texture: the third stage too.
        e.set_global(PASS_TYPE_011F91E4, 0x1c4u32);
        e.register(LIGHT_COUNT_00B704B0, |_, _| 0x0001_0002u32.into_ret());
        e.register(RENDERED_TEXTURE_GET_TEXTURE, |_, _| 0x5555u32.into_ret());
        let object = e.mem.alloc(0x110);
        e.mem.set_u8(object + 0xec, 1);
        e.mem.set_u32(object + 0x10c, 0x10c0);
        let holder = e.mem.alloc(4);
        e.mem.set_u32(holder, object);
        let rendering = e.mem.alloc(0x10);
        e.mem.set_u32(rendering + 0xc, holder);
        e.mem.set_u8(rendering + 9, 1);
        e.set_global(RENDERING_011F91E0, rendering);
        e.call_log = Some(vec![]);
        e.call(0x00c0_2e20, &args![0u32, 0x100u32, property]);
        assert_eq!(
            calls(&e, RENDERED_TEXTURE_GET_TEXTURE),
            vec![vec![0x10c0, 0]]
        );
        assert_eq!(e.mem.u32(stages[2] + 8), 0x5555);
        assert_eq!(
            calls(&e, SET_FILTER_MODE_00E7DEF0),
            vec![vec![stages[2], 1]]
        );
        assert_eq!(calls(&e, STAGE_CALL_00E7DE90)[2], vec![stages[2], 0]);
        // No lights: no third stage.
        e.register(LIGHT_COUNT_00B704B0, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00c0_2e20, &args![0u32, 0x100u32, property]);
        assert!(calls(&e, SET_FILTER_MODE_00E7DEF0).is_empty());
    }

    #[test]
    fn the_rendered_texture_slots() {
        let mut e = engine(&[
            TEXTURE_RETURN_00BA39A0,
            TEXTURE_BORROW_00BA3840,
            PARAM_SET_PIXEL_CONSTANT_00B8A790,
        ]);
        e.map(0x0120_2000, 0x1000);
        e.map(0x011a_e000, 0x1000);
        e.register(ARRAY_SET_AT_00BA3AA0, |e, a| {
            let value = e.mem.u32(a[2]);
            e.mem.set_u32(a[0] + 0x40, value);
            Ret::default()
        });
        let this = e.mem.alloc(0x80);
        e.call(0x00c0_3260, &args![this]);
        assert_eq!(calls(&e, TEXTURE_RETURN_00BA39A0), vec![vec![this + 0x58]]);
        e.call(0x00c0_3390, &args![this, 0u32]);
        assert_eq!(
            calls(&e, TEXTURE_BORROW_00BA3840),
            vec![vec![this + 0x58, 3]]
        );
        assert_eq!(
            calls(&e, ARRAY_SET_AT_00BA3AA0)[0][..2].to_vec(),
            vec![this + 0x28, 2]
        );
        assert_eq!(e.mem.u32(this + 0x68), this + 0x58);
        let parameters = e.mem.alloc(8);
        e.mem.set_u32(parameters + 4, 0x44);
        e.mem.set_u32(this + 0x1c, parameters);
        e.set_global(CONSTANT_012023F4, 1.5f32);
        assert!(!e.call(0x00c0_3350, &args![this, 0u32]).bool());
        assert_eq!(
            calls(&e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![vec![0x44, 0, 1.5f32.to_bits(), 0, 0, 0]]
        );
        assert_eq!(e.global::<u32>(STATE_VALUE_011AE680), 0x80);
    }
}
