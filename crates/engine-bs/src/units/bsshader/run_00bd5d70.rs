//! `BSShader/run_00bd5d70` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The base constructor `00bd6ec0` (`run_00bd5d70`, unnamed; `thiscall`, no stack words), the
/// `NiTArray` set-size `00a90180` (`thiscall` on the array at +0x28, one word), the set-up
/// `00bd6ae0` (unnamed; `thiscall` on +0x58, one word) and `00b8bc20` (unit
/// `imagespacemanager.cpp`, unnamed; `thiscall`, no stack words).
const BASE_CONSTRUCT_00BD6EC0: u32 = 0x00bd_6ec0;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const SET_UP_00BD6AE0: u32 = 0x00bd_6ae0;
const CALL_00B8BC20: u32 = 0x00b8_bc20;
/// The compiler's `eh vector constructor iterator` (`00ec782f`, `stdcall`: array, element
/// size, count, constructor, destructor), the element constructor `004ebb70` (unit
/// `teswater.cpp`, unnamed) and `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`).
const VECTOR_CONSTRUCT_00EC782F: u32 = 0x00ec_782f;
const TEXTURE_CONSTRUCT_004EBB70: u32 = 0x004e_bb70;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
/// This effect's vtable (`010bc410`; no RTTI locator before it).
const VTABLE_010BC410: u32 = 0x010b_c410;

// Translated from 00bd7120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00bd6ec0`, stores the vtable
/// `010bc410`, clears the bytes +0x7c to +0x7e and the words +0x80 to +0x88, constructs the
/// three 16-byte textures at +0x8c (`eh vector constructor iterator` with `004ebb70` and
/// `ImageSpaceTexture::~ImageSpaceTexture`), sizes the array at +0x28 to 6 (`00a90180`),
/// stores 0.0 at +0x68 to +0x70, clears +0x74, runs `00bd6ae0` on +0x58 with 4 and `00b8bc20`.
/// Answers `this`.
// Translation incomplete: 00bd7120 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the textures) is not translated.
pub fn fn_00bd7120(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BASE_CONSTRUCT_00BD6EC0, &args![base]);
    e.mem.set_u32(base, VTABLE_010BC410);
    for offset in [0x7cu32, 0x7d, 0x7e] {
        e.mem.set_u8(base + offset, 0);
    }
    for offset in [0x80u32, 0x84, 0x88] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(
        VECTOR_CONSTRUCT_00EC782F,
        &args![
            base + 0x8c,
            0x10u32,
            3u32,
            TEXTURE_CONSTRUCT_004EBB70,
            TEXTURE_DESTRUCT_00BA3A00
        ],
    );
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 6u32]);
    for offset in [0x68u32, 0x6c, 0x70, 0x74] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(SET_UP_00BD6AE0, &args![base + 0x58, 4u32]);
    e.call(CALL_00B8BC20, &args![base]);
    this
}

/// The Ni allocation `00aa1070` and free `00aa10f0` (`cdecl`, one word each).
const NI_ALLOCATE_00AA1070: u32 = 0x00aa_1070;
const NI_FREE_00AA10F0: u32 = 0x00aa_10f0;

// Translated from 00bd6ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`): the set-size of an `NiTArray` of bytes (base +4,
/// max size +8, size +0xa, element count +0xc, all 16-bit but the base), `thiscall` with one
/// stack word: the new size (compared whole with the 16-bit fields). Nothing when it equals the
/// max size. A size below the current size clears the set bytes from the new size up (each one
/// lowering the element count) and becomes the size. The max size takes the new size; with 0
/// the base is cleared, else a new block of that many bytes (`00aa1070`) takes the old bytes up
/// to the size and zeros up to the max size. The old block is freed (`00aa10f0`).
pub fn fn_00bd6ae0(e: &mut Engine, this: Ptr, size: u32) {
    let base = this.addr();
    if size == u32::from(e.mem.u16(base + 8)) {
        return;
    }
    let narrow = size as u16;
    if size < u32::from(e.mem.u16(base + 0xa)) {
        let mut index = narrow;
        if index < e.mem.u16(base + 0xa) {
            loop {
                let byte = e.mem.u32(base + 4).wrapping_add(u32::from(index));
                if e.mem.u8(byte) != 0 {
                    e.mem.set_u8(byte, 0);
                    let count = e.mem.u16(base + 0xc).wrapping_add(0xffff);
                    e.mem.set_u16(base + 0xc, count);
                }
                index = index.wrapping_add(1);
                if index >= e.mem.u16(base + 0xa) {
                    break;
                }
            }
        }
        e.mem.set_u16(base + 0xa, narrow);
    }
    let old = e.mem.u32(base + 4);
    e.mem.set_u16(base + 8, narrow);
    if size == 0 {
        e.mem.set_u32(base + 4, 0);
        e.call(NI_FREE_00AA10F0, &args![old]);
        return;
    }
    let block = e
        .call(NI_ALLOCATE_00AA1070, &args![u32::from(narrow)])
        .u32();
    e.mem.set_u32(base + 4, block);
    let mut index = 0u16;
    while index < e.mem.u16(base + 0xa) {
        let byte = e.mem.u8(old.wrapping_add(u32::from(index)));
        let block = e.mem.u32(base + 4);
        e.mem.set_u8(block.wrapping_add(u32::from(index)), byte);
        index = index.wrapping_add(1);
    }
    let mut index = e.mem.u16(base + 0xa);
    while index < e.mem.u16(base + 8) {
        let block = e.mem.u32(base + 4);
        e.mem.set_u8(block.wrapping_add(u32::from(index)), 0);
        index = index.wrapping_add(1);
    }
    e.call(NI_FREE_00AA10F0, &args![old]);
}

/// The image-space effect base constructor `00ba4080` (`thiscall`, no stack words), this
/// effect's vtable (`010bc3f0`) and its byte array's (`010b9d24`) (no RTTI locators before
/// them).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const VTABLE_010BC3F0: u32 = 0x010b_c3f0;
const ARRAY_VTABLE_010B9D24: u32 = 0x010b_9d24;

// Translated from 00bd6ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`): the base constructor of `00bd7120`'s effect,
/// `thiscall`, no stack words. Runs the image-space effect base constructor `00ba4080`, stores
/// the vtable `010bc3f0` and makes the empty array at +0x58 (max size +0x60 and element count
/// +0x64 0, base +0x5c null, grow-by +0x66 1, size +0x62 0, vtable `010b9d24`, in that order).
/// Answers `this`.
// Translation incomplete: 00bd6ec0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00bd6ec0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010BC3F0);
    e.mem.set_u16(base + 0x60, 0);
    e.mem.set_u16(base + 0x64, 0);
    e.mem.set_u32(base + 0x5c, 0);
    e.mem.set_u16(base + 0x66, 1);
    e.mem.set_u16(base + 0x62, 0);
    e.mem.set_u32(base + 0x58, ARRAY_VTABLE_010B9D24);
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{MEMORY_ALLOCATE, MEMORY_MANAGER};

/// The array store `00ba3aa0` (unnamed; `thiscall` on an effect array, `RET 8`: index, the
/// address of the word to store).
const ARRAY_SET_00BA3AA0: u32 = 0x00ba_3aa0;
/// `ImageSpaceTexture::BorrowRenderedTexture` (`00ba3840`, `RET 4`) and
/// `::ReturnRenderedTexture` (`00ba39a0`, no stack words) (engine map; `thiscall` on the
/// texture), the texture setter `00ba37f0` (unnamed, `RET 4`) and `00b54090` (unnamed, unit
/// `bsshadermanager.cpp`, no arguments; answers a word).
const BORROW_RENDERED_TEXTURE: u32 = 0x00ba_3840;
const RETURN_RENDERED_TEXTURE: u32 = 0x00ba_39a0;
const TEXTURE_SET_00BA37F0: u32 = 0x00ba_37f0;
const SHADER_MANAGER_CALL_00B54090: u32 = 0x00b5_4090;

// Translated from 00bd6980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the effect's vtable `010bc410`, slot +0x10), `thiscall`
/// with one stack word the code never reads (`RET 4`). Borrows rendered texture 0x25 into the
/// texture at +0x8c and 0x26 into +0x9c, storing them as elements 2 and 3 of the array at
/// +0x28 (`00ba3aa0`, through one stack word); element 5 is the texture at +0x7c when the byte
/// +3 of the object at +0x5c is set, else that texture is returned (`ReturnRenderedTexture`) and
/// element 5 cleared; the texture at +0xac gets `00b54090`'s answer (`00ba37f0`) and becomes
/// element 4.
pub fn fn_00bd6980(e: &mut Engine, this: Ptr, _unused_1: u32) {
    let base = this.addr();
    let array = base + 0x28;
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, base + 0x8c);
        e.call(BORROW_RENDERED_TEXTURE, &args![base + 0x8c, 0x25u32]);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 2u32, local]);
        e.mem.set_u32(local, base + 0x9c);
        e.call(BORROW_RENDERED_TEXTURE, &args![base + 0x9c, 0x26u32]);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 3u32, local]);
        let flags = e.mem.u32(base + 0x5c);
        if e.mem.u8(flags + 3) != 0 {
            e.mem.set_u32(local, base + 0x7c);
        } else {
            e.call(RETURN_RENDERED_TEXTURE, &args![base + 0x7c]);
            e.mem.set_u32(local, 0);
        }
        e.call(ARRAY_SET_00BA3AA0, &args![array, 5u32, local]);
        e.mem.set_u32(local, base + 0xac);
        let texture = e.call(SHADER_MANAGER_CALL_00B54090, &args![]).u32();
        e.call(TEXTURE_SET_00BA37F0, &args![base + 0xac, texture]);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 4u32, local]);
    });
}

// Translated from 00bd6a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the effect's vtable `010bc410`, slot +0x14), `thiscall`,
/// no stack words. Returns the rendered textures of elements 2 and 3 of the array at +0x28
/// (`ReturnRenderedTexture` on the elements, base at +0x2c) and clears the elements 2, 3 and
/// 4 (`00ba3aa0` with a null word); when element 5 is set, returns the texture at +0x7c and
/// clears element 5.
pub fn fn_00bd6a30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let array = base + 0x28;
    e.with_stack(4, |e, local| {
        let local = local.addr();
        let elements = e.mem.u32(base + 0x2c);
        let texture = e.mem.u32(elements + 8);
        e.call(RETURN_RENDERED_TEXTURE, &args![texture]);
        e.mem.set_u32(local, 0);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 2u32, local]);
        let elements = e.mem.u32(base + 0x2c);
        let texture = e.mem.u32(elements + 0xc);
        e.call(RETURN_RENDERED_TEXTURE, &args![texture]);
        e.mem.set_u32(local, 0);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 3u32, local]);
        e.mem.set_u32(local, 0);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 4u32, local]);
        let elements = e.mem.u32(base + 0x2c);
        if e.mem.u32(elements + 0x14) != 0 {
            e.call(RETURN_RENDERED_TEXTURE, &args![base + 0x7c]);
            e.mem.set_u32(local, 0);
            e.call(ARRAY_SET_00BA3AA0, &args![array, 5u32, local]);
        }
    });
}

/// The byte setting `00bd6ab0` tests (`012023d4`).
const ENABLED_012023D4: u32 = 0x0120_23d4;

// Translated from 00bd6ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function; the effect's vtable `010bc410`, slot +0x18), `thiscall`,
/// no stack words: true when the `float` at +0x70 is above 0.0 (`FLDZ; FCOMP`; NaN is not), the
/// byte setting `012023d4` is set and `00b54090` answers non-zero.
pub fn fn_00bd6ab0(e: &mut Engine, this: Ptr) -> bool {
    let value = x87_load32(e.mem.u32(this.addr() + 0x70));
    if 0.0 < value
        && e.global::<u8>(ENABLED_012023D4) != 0
        && e.call(SHADER_MANAGER_CALL_00B54090, &args![]).u32() != 0
    {
        return true;
    }
    false
}

/// `ImageSpaceShaderParam::SetPixelConstant` (engine map, `00b8a790`, `thiscall`, `RET 0x14`:
/// index, four `float`s).
const SHADER_PARAM_SET_PIXEL_CONSTANT: u32 = 0x00b8_a790;
/// The object whose `float`s `00bd66c0` reads (`011f917c`; +0x6c to +0x88 and +0xec, +0xf0).
const SCENE_011F917C: u32 = 0x011f_917c;
/// The `float`s the constants take: `012023e0`, the first word of constant 2 (`01070218`), the
/// vector `012004f0` to `012004f8`, the divisor setting `011f9410` with the `double` 30.0
/// (`0101db88`), and the 3.0 (`01017718`) stored in `012003c4`.
const CONSTANT_012023E0: u32 = 0x0120_23e0;
const CONSTANT_01070218: u32 = 0x0107_0218;
const VECTOR_012004F0: u32 = 0x0120_04f0;
const DIVISOR_011F9410: u32 = 0x011f_9410;
const THIRTY_0101DB88: u32 = 0x0101_db88;
const THREE_01017718: u32 = 0x0101_7718;
const RESULT_012003C4: u32 = 0x0120_03c4;

// Translated from 00bd66c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`; the effect's vtable `010bc410`, slot +0x1c),
/// `thiscall` with one stack word the code never reads (`RET 4`); answers true. With `a`, `b`
/// the `float`s +0xec and +0xf0 of the object at `011f917c` and the effect's mode at +0x74,
/// sets on the shader parameters of elements 2 and then 3 of the array at +0x18 (base +0x1c)
/// the pixel constants 0 (`[+0x68] - [+0x6c]`, `[+0x6c] + [+0x68]`, `[+0x68]`, `012023e0`), 1
/// (`[+0x70]`, 1.0 for modes 0 and 1 else 0.0, 1.0 for modes 0 and 2 else 0.0, 0.0) and 2
/// (`01070218`, `a`, `b - a`, `a * b`). When the setting `011f9410` is above 0.0, element 3
/// also gets constant 3: the dot products of the vector `012004f0` with the object's +0x70,
/// +0x7c, +0x88 and with the negated +0x6c, +0x78, +0x84, then `[+0x78] / (011f9410 * 30.0)`
/// and 0.0. Finally stores 3.0 (`01017718`) in `012003c4`. x87 arithmetic in the exe's order,
/// rounded to `float` where it stores.
pub fn fn_00bd66c0(e: &mut Engine, this: Ptr, _unused_1: u32) -> bool {
    let base = this.addr();
    let scene = e.global::<u32>(SCENE_011F917C);
    let a_bits = x87_quiet_bits(e.mem.u32(scene + 0xec));
    let b_bits = x87_quiet_bits(e.mem.u32(scene + 0xf0));
    let mode = e.mem.u32(base + 0x74);
    let a = x87_load32(a_bits);
    let b = x87_load32(b_bits);
    let b_minus_a = x87_store32(x87_sub(b, a));
    let a_times_b = x87_store32(x87_mul(b, a));
    let first_flag = if mode == 1 || mode == 0 { 1.0f32 } else { 0.0 };
    let second_flag = if mode == 2 || mode == 0 { 1.0f32 } else { 0.0 };
    let p_bits = e.mem.u32(base + 0x68);
    let q_bits = e.mem.u32(base + 0x6c);
    let difference = x87_store32(x87_sub(x87_load32(p_bits), x87_m32(q_bits)));
    let sum = x87_store32(x87_add(x87_load32(q_bits), x87_m32(p_bits)));
    let mut parameter = 0;
    for element in [8u32, 0xc] {
        let elements = e.mem.u32(base + 0x1c);
        parameter = e.mem.u32(elements + element);
        let constant = x87_quiet_bits(e.global::<u32>(CONSTANT_012023E0));
        let p = x87_quiet_bits(e.mem.u32(base + 0x68));
        e.call(
            SHADER_PARAM_SET_PIXEL_CONSTANT,
            &args![parameter, 0u32, difference, sum, p, constant],
        );
        let value = x87_quiet_bits(e.mem.u32(base + 0x70));
        e.call(
            SHADER_PARAM_SET_PIXEL_CONSTANT,
            &args![parameter, 1u32, value, first_flag, second_flag, 0.0f32],
        );
        let first_word = x87_quiet_bits(e.global::<u32>(CONSTANT_01070218));
        e.call(
            SHADER_PARAM_SET_PIXEL_CONSTANT,
            &args![parameter, 2u32, first_word, a_bits, b_minus_a, a_times_b],
        );
    }
    let negated = [0x6cu32, 0x78, 0x84]
        .map(|offset| x87_store32(-x87_load32(x87_quiet_bits(e.mem.u32(scene + offset)))));
    let plain = [0x70u32, 0x7c, 0x88].map(|offset| x87_quiet_bits(e.mem.u32(scene + offset)));
    let vector = [0u32, 4, 8].map(|offset| e.global::<u32>(VECTOR_012004F0 + offset));
    let divisor = x87_quiet_bits(e.global::<u32>(DIVISOR_011F9410));
    let divisor = x87_load32(divisor);
    if divisor > 0.0 {
        let thirty = f64::from_bits(e.global::<u64>(THIRTY_0101DB88));
        let numerator = x87_load32(e.mem.u32(base + 0x78));
        let ratio = x87_store32(x87_div(numerator, x87_mul(divisor, thirty)));
        let [v0, v1, v2] = vector.map(x87_load32);
        let m1 = x87_mul(v1, x87_m32(negated[1]));
        let m2 = x87_mul(x87_load32(negated[0]), v0);
        let sum1 = x87_add(m2, m1);
        let m3 = x87_mul(v2, x87_m32(negated[2]));
        let second = x87_store32(x87_add(sum1, m3));
        let m4 = x87_mul(v1, x87_load32(plain[1]));
        let m5 = x87_mul(v0, x87_load32(plain[0]));
        let sum3 = x87_add(m5, m4);
        let m6 = x87_mul(v2, x87_load32(plain[2]));
        let first = x87_store32(x87_add(m6, sum3));
        e.call(
            SHADER_PARAM_SET_PIXEL_CONSTANT,
            &args![parameter, 3u32, first, second, ratio, 0.0f32],
        );
    }
    let three = x87_quiet_bits(e.global::<u32>(THREE_01017718));
    e.set_global::<u32>(RESULT_012003C4, three);
    true
}

/// `00ba3f90` (unnamed; `thiscall`, one word), the renderer word (`011f9508`),
/// `ImageSpaceShaderParam::ImageSpaceShaderParam` (`00b8a9e0`) and `::InitConstantMap`
/// (`00b8ab10`, `RET 8`) (engine map), `00b8a8b0` (unnamed, `RET 0xc`), and the effect calls
/// `00ba3b10` (`RET 0xc`), `00ba3b50` (`RET 0x14`), `00ba3ba0` (`RET 0x1c`), `00ba3c00` (`RET
/// 0x24`) and `00ba3c70` (`RET 0x10`) (unnamed; `thiscall` on the effect).
const EFFECT_SET_UP_00BA3F90: u32 = 0x00ba_3f90;
const RENDERER_011F9508: u32 = 0x011f_9508;
const SHADER_PARAM_CONSTRUCT: u32 = 0x00b8_a9e0;
const SHADER_PARAM_INIT_CONSTANT_MAP: u32 = 0x00b8_ab10;
const SHADER_PARAM_CALL_00B8A8B0: u32 = 0x00b8_a8b0;
const EFFECT_CALL_00BA3B10: u32 = 0x00ba_3b10;
const EFFECT_CALL_00BA3B50: u32 = 0x00ba_3b50;
const EFFECT_CALL_00BA3BA0: u32 = 0x00ba_3ba0;
const EFFECT_CALL_00BA3C00: u32 = 0x00ba_3c00;
const EFFECT_CALL_00BA3C70: u32 = 0x00ba_3c70;
/// The `float` 2^32 added to a negative `FILD` (`01099378`), the data `00b8a8b0` takes
/// (`011add48`) and the second and third `float`s of the last two constants (`01013974`,
/// `01013d84`).
const TWO_TO_32_01099378: u32 = 0x0109_9378;
const DATA_011ADD48: u32 = 0x011a_dd48;
const CONSTANT_01013974: u32 = 0x0101_3974;
const CONSTANT_01013D84: u32 = 0x0101_3d84;

/// A renderer dimension: vtable slot +0xc8 of the renderer at `011f9508` gives an object whose
/// slot `slot` answers it with 0; loaded as signed (`FILD`), 2^32 added when negative, rounded
/// to a `float` (its bits).
fn renderer_dimension(e: &mut Engine, slot: u32) -> u32 {
    let renderer = e.global::<u32>(RENDERER_011F9508);
    let target = e.vcall(renderer, 0xc8, &args![]).u32();
    let value = e.vcall(target, slot, &args![0u32]).u32();
    let mut loaded = f64::from(value as i32);
    if (value as i32) < 0 {
        loaded = x87_add(loaded, x87_m32(e.global::<u32>(TWO_TO_32_01099378)));
    }
    x87_store32(loaded)
}

/// A shader parameter: 0x88 bytes from the memory manager, the constructor run when the block
/// is not null; answers the parameter, or 0 for a null block.
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

// Translated from 00bd6bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`; the effect's vtable `010bc410`, slot +0x8),
/// `thiscall` with two stack words (`RET 8`): an owner whose word at +0x8 holds the textures
/// (+0x104, +0x48, +0xcc, +0xd0), and a word the code never reads. Runs `00ba3f90` with 4, reads
/// the renderer's two dimensions (slots +0x8c and +0x90 of the object its slot +0xc8 gives).
/// Shader parameter 0 (`InitConstantMap(0, 0x18)`, `00b8a8b0(3, 011add48, 0x40)`, pixel
/// constant 2 (1 / first dimension, 1 / second, 0.0, 0.0), byte +0x55 cleared) goes to
/// `00ba3c70(0, texture +0x104, parameter, 2)` and element 0 of the array at +0x18; element 1 is
/// cleared after `00ba3b10(0, 1, 1)`, then `00ba3b50(1, 2, 3, 0, 0)` and `00ba3c70(1, texture
/// +0x48, 0, 2)`. Parameters 2 (`InitConstantMap(0, 3)`) and 3 (`InitConstantMap(0, 4)`) get
/// pixel constant 0 (1.0, `01013974`, `01013d84`, `012023e0`), byte +0x56 cleared, become
/// elements 2 and 3 with `00ba3ba0(2, 1, 2, 4, 0, 1, 0)` / `00ba3c00(3, 1, 2, 4, 5, 0, 1, 0, 0)`
/// and `00ba3c70` with the textures +0xcc / +0xd0 and 0. A null parameter block gives a null
/// parameter.
// Translation incomplete: 00bd6bb0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the parameters' allocations) is not translated.
pub fn fn_00bd6bb0(e: &mut Engine, this: Ptr, owner: Ptr, _unused_2: u32) {
    let base = this.addr();
    let array = base + 0x18;
    e.call(EFFECT_SET_UP_00BA3F90, &args![base, 4u32]);
    let first_dimension = renderer_dimension(e, 0x8c);
    let second_dimension = renderer_dimension(e, 0x90);
    let parameter = new_parameter(e);
    e.call(
        SHADER_PARAM_INIT_CONSTANT_MAP,
        &args![parameter, 0u32, 0x18u32],
    );
    e.call(
        SHADER_PARAM_CALL_00B8A8B0,
        &args![parameter, 3u32, DATA_011ADD48, 0x40u32],
    );
    let inverse_second = x87_store32(x87_div(1.0, x87_load32(second_dimension)));
    let inverse_first = x87_store32(x87_div(1.0, x87_m32(first_dimension)));
    e.call(
        SHADER_PARAM_SET_PIXEL_CONSTANT,
        &args![
            parameter,
            2u32,
            x87_quiet_bits(inverse_first),
            x87_quiet_bits(inverse_second),
            0.0f32,
            0.0f32
        ],
    );
    e.mem.set_u8(parameter + 0x55, 0);
    let textures = e.mem.u32(owner.addr() + 8);
    let texture = e.mem.u32(textures + 0x104);
    e.call(
        EFFECT_CALL_00BA3C70,
        &args![base, 0u32, texture, parameter, 2u32],
    );
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, parameter);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 0u32, local]);
        e.call(EFFECT_CALL_00BA3B10, &args![base, 0u32, 1u32, 1u32]);
        e.mem.set_u32(local, 0);
        e.call(ARRAY_SET_00BA3AA0, &args![array, 1u32, local]);
        e.call(
            EFFECT_CALL_00BA3B50,
            &args![base, 1u32, 2u32, 3u32, 0u32, 0u32],
        );
        let textures = e.mem.u32(owner.addr() + 8);
        let texture = e.mem.u32(textures + 0x48);
        e.call(
            EFFECT_CALL_00BA3C70,
            &args![base, 1u32, texture, 0u32, 2u32],
        );
        let one = 1.0f32.to_bits();
        for (index, registers, texture_offset) in [(2u32, 3u32, 0xccu32), (3, 4, 0xd0)] {
            let parameter = new_parameter(e);
            e.call(
                SHADER_PARAM_INIT_CONSTANT_MAP,
                &args![parameter, 0u32, registers],
            );
            let fourth = x87_quiet_bits(e.global::<u32>(CONSTANT_012023E0));
            let third = x87_quiet_bits(e.global::<u32>(CONSTANT_01013D84));
            let second = x87_quiet_bits(e.global::<u32>(CONSTANT_01013974));
            e.call(
                SHADER_PARAM_SET_PIXEL_CONSTANT,
                &args![parameter, 0u32, one, second, third, fourth],
            );
            e.mem.set_u8(parameter + 0x56, 0);
            e.mem.set_u32(local, parameter);
            e.call(ARRAY_SET_00BA3AA0, &args![array, index, local]);
            if index == 2 {
                e.call(
                    EFFECT_CALL_00BA3BA0,
                    &args![base, 2u32, 1u32, 2u32, 4u32, 0u32, 1u32, 0u32],
                );
            } else {
                e.call(
                    EFFECT_CALL_00BA3C00,
                    &args![base, 3u32, 1u32, 2u32, 4u32, 5u32, 0u32, 1u32, 0u32, 0u32],
                );
            }
            let textures = e.mem.u32(owner.addr() + 8);
            let texture = e.mem.u32(textures + texture_offset);
            e.call(
                EFFECT_CALL_00BA3C70,
                &args![base, index, texture, parameter, 0u32],
            );
        }
    });
}

/// The byte array's vtable restored by the base destructor (`010b9d24`), the Ni free
/// (`00aa10f0`, `cdecl`) and `ImageSpaceEffect::~ImageSpaceEffect` (engine map, `00ba3d30`).
const EFFECT_DESTRUCT_00BA3D30: u32 = 0x00ba_3d30;

// Translated from 00bd6f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`): the destructor of `00bd6ec0`'s base, `thiscall`,
/// no stack words. Restores the byte array's vtable at +0x58 (`010b9d24`), frees its base at
/// +0x5c (`00aa10f0`) and runs `ImageSpaceEffect::~ImageSpaceEffect` (`00ba3d30`).
// Translation incomplete: 00bd6f30 - the C++ exception frame (FS:[0] chain, unwind state 0 for the array) is not translated.
pub fn fn_00bd6f30(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let block = e.mem.u32(base + 0x5c);
    e.mem.set_u32(base + 0x58, ARRAY_VTABLE_010B9D24);
    e.call(NI_FREE_00AA10F0, &args![block]);
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

/// The compiler's `eh vector destructor iterator` (`00ec5fce`, `stdcall`: array, element size,
/// count, destructor) and `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`).
const VECTOR_DESTRUCT_00EC5FCE: u32 = 0x00ec_5fce;

// Translated from 00bd6fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`): the destructor of `00bd7120`'s effect,
/// `thiscall`, no stack words. Stores the vtable `010bc410`; for the elements 2, 1, 0, 0 again
/// and 3 of the array at +0x18 (base +0x1c): when set, deletes it through its vtable slot 0
/// with 1 (when not null) and clears it (`00ba3aa0` with a null word). Destroys the three
/// textures at +0x8c (`eh vector destructor iterator`) and the texture at +0x7c, then runs the
/// base destructor `00bd6f30`.
// Translation incomplete: 00bd6fc0 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the textures and the base) is not translated.
pub fn fn_00bd6fc0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010BC410);
    e.with_stack(4, |e, local| {
        let local = local.addr();
        for index in [2u32, 1, 0, 0, 3] {
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
    });
    e.call(
        VECTOR_DESTRUCT_00EC5FCE,
        &args![base + 0x8c, 0x10u32, 3u32, TEXTURE_DESTRUCT_00BA3A00],
    );
    e.call(TEXTURE_DESTRUCT_00BA3A00, &args![base + 0x7c]);
    fn_00bd6f30(e, this);
}

/// The memory manager's free (`00aa4060`, `thiscall` on `011f6238`, one word).
const MEMORY_DEALLOCATE: u32 = 0x00aa_4060;

// Translated from 00bd71e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00bd5d70`; the effect's vtable `010bc410`, slot 0): the
/// effect's scalar deleting destructor, `thiscall`, one stack word (the flags). Runs `00bd6fc0`;
/// with bit 0 set, frees `this` through the memory manager (`00aa4060`). Answers `this`.
pub fn fn_00bd71e0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bd6fc0(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    allocate_array, ni_pointer_assign, IMPORT_INTERLOCKED_DECREMENT, IMPORT_INTERLOCKED_INCREMENT,
};

// Translated from 00bd5d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with two words: a 16-byte entry and a word. When the
/// count (`u16` +0xe) is below the capacity (`u16` +0xc) and the entry is not null, copies the
/// entry into the entry array (+0x10) at the count, stores the word in the word array (+0x14) at
/// the count, counts one more and answers 1. Otherwise answers the count with its low byte
/// cleared (`XOR AL,AL` on the zero-extended count).
pub fn fn_00bd5d70(e: &mut Engine, this: Ptr, entry: u32, word: u32) -> u32 {
    let base = this.addr();
    let count = e.mem.u16(base + 0xe);
    if count == e.mem.u16(base + 0xc) || entry == 0 {
        return u32::from(count) & 0xff00;
    }
    let target = (u32::from(count) << 4).wrapping_add(e.mem.u32(base + 0x10));
    for offset in [0u32, 4, 8, 0xc] {
        let value = e.mem.u32(entry + offset);
        e.mem.set_u32(target + offset, value);
    }
    let count = u32::from(e.mem.u16(base + 0xe));
    let words = e.mem.u32(base + 0x14);
    e.mem.set_u32(words.wrapping_add(4 * count), word);
    let count = e.mem.u16(base + 0xe).wrapping_add(1);
    e.mem.set_u16(base + 0xe, count);
    1
}

// Translated from 00bd5dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall` with one word, a pointer to three words: copies them to
/// +0x18 when it is not null; then on the object +8, for each of the bits 0x400, 0x800 and
/// 0x1000 of its +0x24: clears its +0x38 when the bit is set, and clears the bit.
pub fn fn_00bd5dd0(e: &mut Engine, this: Ptr, words: u32) {
    let base = this.addr();
    if words != 0 {
        for offset in [0u32, 4, 8] {
            let value = e.mem.u32(words + offset);
            e.mem.set_u32(base + 0x18 + offset, value);
        }
    }
    for bit in [0x400u32, 0x800, 0x1000] {
        let object = e.mem.u32(base + 8);
        let flags = e.mem.u32(object + 0x24);
        if flags & bit != 0 {
            e.mem.set_u32(object + 0x38, 0);
        }
        let flags = e.mem.u32(object + 0x24);
        e.mem.set_u32(object + 0x24, flags & !bit);
    }
}

/// `00bd5e40`'s start value (`010bc3ec`), the height below which an entry is left out
/// (`01094cf4`), the half (`01011588`, a `double`), the RTTI record the object's chain is checked
/// for (`012024c0`) and the one `00653290` is asked for (`012024b8`; unnamed, `thiscall`, one
/// word).
const START_010BC3EC: u32 = 0x010b_c3ec;
const LOWEST_01094CF4: u32 = 0x0109_4cf4;
const HALF_01011588: u32 = 0x0101_1588;
const RTTI_012024C0: u32 = 0x0120_24c0;
const RTTI_012024B8: u32 = 0x0120_24b8;
const FIND_00653290: u32 = 0x0065_3290;
/// The CRT square root (`00ec6040`, value and result in `ST0`).
const CRT_SQRT_00EC6040: u32 = 0x00ec_6040;

/// `min` after `FLD min; FLD v; FCOM; TEST AH,0x41; JZ`: replaced unless `v` is above it.
fn keep_lower(min: u32, value: u32) -> u32 {
    if x87_load32(value) > x87_load32(min) {
        min
    } else {
        value
    }
}

/// `max` after `FLD max; FLD v; FCOM; TEST AH,5; JNP`: replaced unless `v` is below it.
fn keep_higher(max: u32, value: u32) -> u32 {
    if x87_load32(value) < x87_load32(max) {
        max
    } else {
        value
    }
}

// Translated from 00bd5e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall`, no stack words. The bounds of the entries (16 bytes
/// each at +0x10, `u16` count +0xe) whose `float` +8 is above `01094cf4`: minimum and maximum
/// start at `010bc3ec` and its negation (0.0 without entries), each axis compared as the x87
/// does (a NaN replaces). With the objects +4 and +0 set: the centre is min + (max - min) *
/// 0.5, the radius the distance from the centre to the maximum plus the `float` +0x28 of the
/// object +4. When the object +0 has `012024c0` in its RTTI chain (vtable +8, bases at +4), or
/// else when `00653290(012024b8)` on it answers an object, that object's vtable +0xfc gets the
/// bound (centre and radius, four words) and +0x100 the count.
pub fn fn_00bd5e40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let start = if e.mem.u16(base + 0xe) == 0 {
        0.0f32.to_bits()
    } else {
        x87_quiet_bits(e.global::<u32>(START_010BC3EC))
    };
    let negated = x87_store32(-x87_load32(start));
    let mut min = [start; 3];
    let mut max = [negated; 3];
    let count = u32::from(e.mem.u16(base + 0xe));
    if count > 0 {
        let lowest = x87_load32(e.global::<u32>(LOWEST_01094CF4));
        let mut entry = e.mem.u32(base + 0x10);
        for _ in 0..count {
            // `FCOM`, `TEST AH,5`, `JP`: only an entry whose height is above `lowest`.
            if lowest < x87_m32(e.mem.u32(entry + 8)) {
                for axis in 0..3 {
                    let value = x87_quiet_bits(e.mem.u32(entry + 4 * axis as u32));
                    min[axis] = keep_lower(min[axis], value);
                    max[axis] = keep_higher(max[axis], value);
                }
            }
            entry = entry.wrapping_add(0x10);
        }
    }
    let holder = e.mem.u32(base + 4);
    if holder == 0 {
        return;
    }
    let object = e.mem.u32(base);
    if object == 0 {
        return;
    }
    let mut size = [0u32; 3];
    for axis in 0..3 {
        size[axis] = x87_store32(x87_sub(x87_load32(max[axis]), x87_load32(min[axis])));
    }
    let half = f64::from_bits(e.global::<u64>(HALF_01011588));
    let halves = [
        x87_store32(x87_mul(x87_load32(size[0]), half)),
        x87_store32(x87_mul(x87_load32(size[1]), half)),
        x87_store32(x87_mul(half, x87_m32(size[2]))),
    ];
    let mut centre = [0u32; 3];
    let mut extent = [0u32; 3];
    for axis in 0..3 {
        centre[axis] = x87_store32(x87_add(x87_load32(min[axis]), x87_load32(halves[axis])));
    }
    for axis in 0..2 {
        extent[axis] = x87_store32(x87_sub(x87_load32(max[axis]), x87_load32(centre[axis])));
    }
    extent[2] = x87_store32(x87_sub(x87_load32(max[2]), x87_m32(centre[2])));
    let ex = x87_load32(extent[0]);
    let ey = x87_load32(extent[1]);
    let ez = x87_load32(extent[2]);
    let squares = x87_add(x87_mul(ey, ey), x87_mul(ex, ex));
    let total = x87_store32(x87_add(squares, x87_mul(ez, ez)));
    let root = e.call(CRT_SQRT_00EC6040, &args![x87_load32(total)]).f64();
    let length = x87_load32(x87_store32(root));
    let radius = x87_store32(x87_add(length, x87_m32(e.mem.u32(holder + 0x28))));
    let mut record = e.vcall(object, 8, &args![]).u32();
    let mut found = false;
    while record != 0 {
        if record == RTTI_012024C0 {
            found = true;
            break;
        }
        record = e.mem.u32(record + 4);
    }
    let target = if found {
        object
    } else {
        let object = e.mem.u32(base);
        if object == 0 {
            return;
        }
        let target = e.call(FIND_00653290, &args![object, RTTI_012024B8]).u32();
        if target == 0 {
            return;
        }
        target
    };
    e.vcall(
        target,
        0xfc,
        &args![centre[0], centre[1], centre[2], radius],
    );
    let count = u32::from(e.mem.u16(base + 0xe));
    e.vcall(target, 0x100, &args![count]);
}

// Translated from 00bd6150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall`, no stack words: for each of the `u16` +0xe entries,
/// clears the `float`s +8 and +0xc of its 16-byte entry (+0x10) and its word (+0x14); then sets
/// the count to 0 and jumps to `00bd5e40`.
pub fn fn_00bd6150(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(base + 0xe)) {
        let entries = e.mem.u32(base + 0x10);
        e.mem.set_f32(entries.wrapping_add(16 * index + 8), 0.0);
        let entries = e.mem.u32(base + 0x10);
        e.mem.set_f32(entries.wrapping_add(16 * index + 0xc), 0.0);
        let words = e.mem.u32(base + 0x14);
        e.mem.set_u32(words.wrapping_add(4 * index), 0);
        index += 1;
    }
    e.mem.set_u16(base + 0xe, 0);
    fn_00bd5e40(e, this);
}

/// The calls of `00bd6190`: the member constructor `00b57e70` (`thiscall`), the Ni allocation
/// `00aa13e0`, the geometry constructors `00c1a480` and `00c1a660` (`thiscall`, one word),
/// `NiAVObject::AttachProperty` (`00439410`, `thiscall`, one word), the property constructor
/// `00b57f90`, `BSShaderManager::PrepareObject` (`00b57e30`, `cdecl`, three words),
/// `NiAdditionalGeometryData`'s constructor (`00ba63e0`, two words), `SetDataBlockCount`
/// (`00a72ca0`), `SetDataBlock` (`00a72bb0`), `SetDataStream` (`00a726a0`) and the geometry
/// data's `00a67260` (one word).
const MEMBER_CONSTRUCT_00B57E70: u32 = 0x00b5_7e70;
const NI_ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
const GEOMETRY_CONSTRUCT_00C1A480: u32 = 0x00c1_a480;
const GEOMETRY_CONSTRUCT_00C1A660: u32 = 0x00c1_a660;
const ATTACH_PROPERTY: u32 = 0x0043_9410;
const PROPERTY_CONSTRUCT_00B57F90: u32 = 0x00b5_7f90;
const PREPARE_OBJECT: u32 = 0x00b5_7e30;
const ADDITIONAL_DATA_CONSTRUCT: u32 = 0x00ba_63e0;
const ADDITIONAL_DATA_SET_BLOCK_COUNT: u32 = 0x00a7_2ca0;
const ADDITIONAL_DATA_SET_BLOCK: u32 = 0x00a7_2bb0;
const ADDITIONAL_DATA_SET_STREAM: u32 = 0x00a7_26a0;
const GEOMETRY_DATA_00A67260: u32 = 0x00a6_7260;
/// The shader model (`011f91c0`).
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;

/// An `NiPointer` released when set (count at +4, deleted through the vtable +4 at 0).
fn release_held(e: &mut Engine, object: u32) {
    let left = e
        .call(IMPORT_INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32();
    if left == 0 {
        e.vcall(object, 4, &args![]);
    }
}

/// An `NiPointer` assignment whose new object's count goes up without a null test (as the exe
/// writes it when the object cannot be null).
fn assign_counted(e: &mut Engine, slot: u32, object: u32) {
    let old = e.mem.u32(slot);
    if old == object {
        return;
    }
    if old != 0 {
        release_held(e, old);
    }
    e.mem.set_u32(slot, object);
    e.call(IMPORT_INTERLOCKED_INCREMENT, &args![object.wrapping_add(4)]);
}

/// A bit of the `u32` at `flags` set or cleared, with the property's +0x38 cleared when the bit
/// changes.
fn set_flag(e: &mut Engine, property: u32, flags: u32, bit: u32, on: bool) {
    let was = e.mem.u32(property + flags) & bit != 0;
    if was != on {
        e.mem.set_u32(property + 0x38, 0);
    }
    let value = e.mem.u32(property + flags);
    e.mem.set_u32(
        property + flags,
        if on { value | bit } else { value & !bit },
    );
}

// Translated from 00bd6190 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bd6190 - the C++ exception frame (FS:[0] chain, unwind states 2 to 6) is not translated.
/// Unnamed in the engine map: a constructor, `thiscall` with three words (a source block, a
/// word, a flag byte). Clears +0, +4, +8, constructs the member at +0x18 (`00b57e70`), takes the
/// source's `u16` +0x2c as the capacity +0xc, holds the source at +4 (its count raised without a
/// null test), stores the word at +0x24, allocates the entry (16 bytes each) and word arrays
/// (`MemoryManager::Allocate`, all ones on overflow) and clears them, empties them again (the
/// inlined `00bd6150`, with `00bd5e40`). Then makes the geometry +0: drops the old one, 0xd8 bytes
/// constructed by `00c1a480` (source byte +0xc 1) or `00c1a660` with the source's +8; attaches
/// the source's property +0x1c (shader model 2 or more) or +0x20 and its +0x24 when set; makes a
/// property (0x8c bytes, `00b57f90`) holding the source's +0x18 at +0x80 and `this` at +0x7c,
/// sets its bit 2 of +0x20 by the flag and bit 13 of +0x24 by the source byte +0x30 (clearing
/// +0x38 when a bit changes), attaches it and holds it at +8. `PrepareObject(geometry, 0, 0)`;
/// when the geometry's +0xb8 data has no +0x30, an `NiAdditionalGeometryData` of the source's
/// +0x14 times +0x2c floats with one block (the source's +0x10) and one stream goes to the data's
/// `00a67260`. Answers `this`.
pub fn fn_00bd6190(e: &mut Engine, this: Ptr, source: u32, word: u32, flag: u32) -> Ptr {
    let base = this.addr();
    for offset in [0u32, 4, 8] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(MEMBER_CONSTRUCT_00B57E70, &args![base + 0x18]);
    let capacity = e.mem.u16(source + 0x2c);
    e.mem.set_u16(base + 0xc, capacity);
    assign_counted(e, base + 4, source);
    e.mem.set_u32(base + 0x24, word);
    let capacity = u32::from(e.mem.u16(base + 0xc));
    let entries = allocate_array(e, capacity, 0x10);
    e.mem.set_u32(base + 0x10, entries);
    let capacity = u32::from(e.mem.u16(base + 0xc));
    let words = allocate_array(e, capacity, 4);
    e.mem.set_u32(base + 0x14, words);
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(base + 0xc)) {
        for offset in [0u32, 4, 8, 0xc] {
            let entries = e.mem.u32(base + 0x10);
            e.mem
                .set_f32(entries.wrapping_add(16 * index + offset), 0.0);
        }
        let words = e.mem.u32(base + 0x14);
        e.mem.set_u32(words.wrapping_add(4 * index), 0);
        index += 1;
    }
    let capacity = e.mem.u16(base + 0xc);
    e.mem.set_u16(base + 0xe, capacity);
    let mut index = 0u32;
    while index < u32::from(e.mem.u16(base + 0xe)) {
        for offset in [8u32, 0xc] {
            let entries = e.mem.u32(base + 0x10);
            e.mem
                .set_f32(entries.wrapping_add(16 * index + offset), 0.0);
        }
        let words = e.mem.u32(base + 0x14);
        e.mem.set_u32(words.wrapping_add(4 * index), 0);
        index += 1;
    }
    e.mem.set_u16(base + 0xe, 0);
    fn_00bd5e40(e, this);
    let old = e.mem.u32(base);
    if old != 0 {
        release_held(e, old);
        e.mem.set_u32(base, 0);
    }
    let first = e.mem.u32(source + 8);
    let constructor = if e.mem.u8(source + 0xc) == 1 {
        GEOMETRY_CONSTRUCT_00C1A480
    } else {
        GEOMETRY_CONSTRUCT_00C1A660
    };
    let block = e.call(NI_ALLOCATE_00AA13E0, &args![0xd8u32]).u32();
    let geometry = if block != 0 {
        e.call(constructor, &args![block, first]).u32()
    } else {
        0
    };
    ni_pointer_assign(e, base, geometry);
    let property = if e.global::<i32>(SHADER_MODEL_011F91C0) >= 2 {
        e.mem.u32(source + 0x1c)
    } else {
        e.mem.u32(source + 0x20)
    };
    if property != 0 {
        let geometry = e.mem.u32(base);
        e.call(ATTACH_PROPERTY, &args![geometry, property]);
    }
    let property = e.mem.u32(source + 0x24);
    if property != 0 {
        let geometry = e.mem.u32(base);
        e.call(ATTACH_PROPERTY, &args![geometry, property]);
    }
    let block = e.call(NI_ALLOCATE_00AA13E0, &args![0x8cu32]).u32();
    let made = if block != 0 {
        e.call(PROPERTY_CONSTRUCT_00B57F90, &args![block]).u32()
    } else {
        0
    };
    let texture = e.mem.u32(source + 0x18);
    if e.mem.u32(made.wrapping_add(0x80)) != texture {
        let old = e.mem.u32(made.wrapping_add(0x80));
        if old != 0 {
            release_held(e, old);
        }
        let texture = e.mem.u32(source + 0x18);
        e.mem.set_u32(made.wrapping_add(0x80), texture);
        if texture != 0 {
            e.call(IMPORT_INTERLOCKED_INCREMENT, &args![texture + 4]);
        }
    }
    e.mem.set_u32(made.wrapping_add(0x7c), base);
    set_flag(e, made, 0x20, 4, flag as u8 != 0);
    set_flag(e, made, 0x24, 0x2000, e.mem.u8(source + 0x30) != 0);
    let geometry = e.mem.u32(base);
    e.call(ATTACH_PROPERTY, &args![geometry, made]);
    assign_counted(e, base + 8, made);
    let geometry = e.mem.u32(base);
    e.call(PREPARE_OBJECT, &args![geometry, 0u32, 0u32]);
    let geometry = e.mem.u32(base);
    let data = e.mem.u32(geometry + 0xb8);
    if data == 0 || e.mem.u32(data + 0x30) == 0 {
        let floats = e
            .mem
            .u32(source + 0x14)
            .wrapping_mul(e.mem.u32(source + 0x2c));
        let block = e.call(NI_ALLOCATE_00AA13E0, &args![0x2cu32]).u32();
        let additional = if block != 0 {
            e.call(ADDITIONAL_DATA_CONSTRUCT, &args![block, floats, 1u32])
                .u32()
        } else {
            0
        };
        e.call(ADDITIONAL_DATA_SET_BLOCK_COUNT, &args![additional, 1u32]);
        let values = e.mem.u32(source + 0x10);
        e.call(
            ADDITIONAL_DATA_SET_BLOCK,
            &args![additional, 0u32, values, floats.wrapping_mul(4), 0u32],
        );
        e.call(
            ADDITIONAL_DATA_SET_STREAM,
            &args![additional, 0u32, 0u32, 0u32, 1u32, floats, 4u32, 4u32],
        );
        let geometry = e.mem.u32(base);
        let data = e.mem.u32(geometry + 0xb8);
        e.call(GEOMETRY_DATA_00A67260, &args![data, additional]);
    }
    this
}

// Translated from 00bd6570 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00bd6570 - the C++ exception frame (FS:[0] chain, unwind states 2 to 0 for the members) is not translated.
/// Unnamed in the engine map: the destructor of `00bd6190`'s object, `thiscall`. Releases and
/// clears +8 and +4; calls the vtable +0xe8 of the geometry's (+0) +0x18 object with the
/// geometry when it is set; releases and clears the geometry; empties the entries (`00bd6150`);
/// frees the entry array when set and the word array (`MemoryManager::Deallocate`), clearing
/// both; then the members' own destructors release +8, +4 and +0 when still set.
pub fn fn_00bd6570(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for offset in [8u32, 4] {
        let held = e.mem.u32(base + offset);
        if held != 0 {
            release_held(e, held);
            e.mem.set_u32(base + offset, 0);
        }
    }
    let geometry = e.mem.u32(base);
    let owner = e.mem.u32(geometry + 0x18);
    if owner != 0 {
        e.vcall(owner, 0xe8, &args![geometry]);
    }
    let geometry = e.mem.u32(base);
    if geometry != 0 {
        release_held(e, geometry);
        e.mem.set_u32(base, 0);
    }
    fn_00bd6150(e, this);
    let entries = e.mem.u32(base + 0x10);
    if entries != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, entries]);
        e.mem.set_u32(base + 0x10, 0);
    }
    let words = e.mem.u32(base + 0x14);
    e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, words]);
    e.mem.set_u32(base + 0x14, 0);
    for offset in [8u32, 4, 0] {
        let held = e.mem.u32(base + offset);
        if held != 0 {
            release_held(e, held);
        }
    }
}

// Translated from 00bd6fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the scalar deleting destructor (vtable entry at `010bc3f0`),
/// `thiscall`, `RET 4`. Runs `00bd6f30` and, with bit 0 of the flags, frees `this` through the
/// memory manager (`00aa4060`). Answers `this`.
pub fn fn_00bd6fa0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00bd6f30(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00bd_5d70, fn_00bd5d70(Ptr, u32, u32) -> u32),
        entry!(0x00bd_5dd0, fn_00bd5dd0(Ptr, u32)),
        entry!(0x00bd_5e40, fn_00bd5e40(Ptr)),
        entry!(0x00bd_6150, fn_00bd6150(Ptr)),
        entry!(0x00bd_6190, fn_00bd6190(Ptr, u32, u32, u32) -> Ptr),
        entry!(0x00bd_6570, fn_00bd6570(Ptr)),
        entry!(0x00bd_6fa0, fn_00bd6fa0(Ptr, u32) -> Ptr),
        entry!(0x00bd_7120, fn_00bd7120(Ptr) -> Ptr),
        entry!(0x00bd_6ae0, fn_00bd6ae0(Ptr, u32)),
        entry!(0x00bd_6ec0, fn_00bd6ec0(Ptr) -> Ptr),
        entry!(0x00bd_6980, fn_00bd6980(Ptr, u32)),
        entry!(0x00bd_6a30, fn_00bd6a30(Ptr)),
        entry!(0x00bd_6ab0, fn_00bd6ab0(Ptr) -> bool),
        entry!(0x00bd_66c0, fn_00bd66c0(Ptr, u32) -> bool),
        entry!(0x00bd_6bb0, fn_00bd6bb0(Ptr, Ptr, u32)),
        entry!(0x00bd_6f30, fn_00bd6f30(Ptr)),
        entry!(0x00bd_6fc0, fn_00bd6fc0(Ptr)),
        entry!(0x00bd_71e0, fn_00bd71e0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00bd7120_builds_the_effect() {
        let mut e = Engine::new();
        for target in [
            BASE_CONSTRUCT_00BD6EC0,
            ARRAY_SET_SIZE_00A90180,
            SET_UP_00BD6AE0,
            CALL_00B8BC20,
            VECTOR_CONSTRUCT_00EC782F,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0xbc);
        e.mem.write(this, &[0xcc; 0xbc]);
        assert_eq!(e.call(0x00bd_7120, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BC410);
        assert_eq!(e.mem.bytes(this + 0x68, 0x10), vec![0; 0x10]);
        assert_eq!(e.mem.bytes(this + 0x7c, 3), vec![0; 3]);
        assert_eq!(e.mem.u8(this + 0x7f), 0xcc);
        assert_eq!(e.mem.bytes(this + 0x80, 0xc), vec![0; 0xc]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (BASE_CONSTRUCT_00BD6EC0, vec![this]),
                (
                    VECTOR_CONSTRUCT_00EC782F,
                    vec![
                        this + 0x8c,
                        0x10,
                        3,
                        TEXTURE_CONSTRUCT_004EBB70,
                        TEXTURE_DESTRUCT_00BA3A00
                    ]
                ),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 6]),
                (SET_UP_00BD6AE0, vec![this + 0x58, 4]),
                (CALL_00B8BC20, vec![this]),
            ]
        );
    }
}

#[cfg(test)]
mod b000n_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(NI_ALLOCATE_00AA1070, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(NI_FREE_00AA10F0, |_, _| Ret::default());
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An array of max size 4 and size 4 holding 1, 0, 2, 3 (element count 3).
    fn array(e: &mut Engine) -> (u32, u32) {
        let this = e.mem.alloc(0x10);
        let old = e.mem.alloc(4);
        e.mem.write(old, &[1, 0, 2, 3]);
        e.mem.set_u32(this + 4, old);
        e.mem.set_u16(this + 8, 4);
        e.mem.set_u16(this + 0xa, 4);
        e.mem.set_u16(this + 0xc, 3);
        (this, old)
    }

    #[test]
    fn fn_00bd6ae0_shrinks_and_grows_the_byte_array() {
        let mut e = engine();
        let (this, old) = array(&mut e);
        e.call(0x00bd_6ae0, &args![this, 2u32]);
        // Bytes 2 and 3 cleared (both set): count 1, size 2, max 2, a new block of 2.
        assert_eq!(e.mem.u16(this + 0xc), 1);
        assert_eq!(e.mem.u16(this + 0xa), 2);
        assert_eq!(e.mem.u16(this + 8), 2);
        let block = e.mem.u32(this + 4);
        assert_eq!(e.mem.bytes(block, 2), vec![1, 0]);
        assert_eq!(calls(&e, NI_ALLOCATE_00AA1070), vec![vec![2]]);
        assert_eq!(calls(&e, NI_FREE_00AA10F0), vec![vec![old]]);
        // Growing keeps the size and zeros the rest.
        e.mem.write(block, &[7, 8]);
        e.call(0x00bd_6ae0, &args![this, 5u32]);
        let grown = e.mem.u32(this + 4);
        assert_eq!(e.mem.bytes(grown, 5), vec![7, 8, 0, 0, 0]);
        assert_eq!(e.mem.u16(this + 0xa), 2);
        assert_eq!(e.mem.u16(this + 8), 5);
        // The same max size: nothing.
        e.call(0x00bd_6ae0, &args![this, 5u32]);
        assert_eq!(calls(&e, NI_ALLOCATE_00AA1070).len(), 2);
        // Size 0: base cleared, the old block freed.
        e.call(0x00bd_6ae0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u16(this + 0xa), 0);
        assert_eq!(calls(&e, NI_FREE_00AA10F0).last().unwrap(), &vec![grown]);
    }

    #[test]
    fn fn_00bd6ec0_builds_the_effect_and_its_array() {
        let mut e = engine();
        let this = e.mem.alloc(0x68);
        e.mem.write(this, &[0xcc; 0x68]);
        assert_eq!(e.call(0x00bd_6ec0, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BC3F0);
        assert_eq!(e.mem.u32(this + 0x58), ARRAY_VTABLE_010B9D24);
        assert_eq!(e.mem.bytes(this + 0x5c, 0xa), vec![0; 0xa]);
        assert_eq!(e.mem.u16(this + 0x66), 1);
        assert_eq!(calls(&e, EFFECT_CONSTRUCT_00BA4080), vec![vec![this]]);
    }
}

#[cfg(test)]
mod b056_tests {
    use super::*;

    /// No-op doubles for every callee outside the file, the array store as a real store and
    /// the call log on.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for target in [
            BORROW_RENDERED_TEXTURE,
            RETURN_RENDERED_TEXTURE,
            TEXTURE_SET_00BA37F0,
            SHADER_PARAM_SET_PIXEL_CONSTANT,
            EFFECT_SET_UP_00BA3F90,
            SHADER_PARAM_INIT_CONSTANT_MAP,
            SHADER_PARAM_CALL_00B8A8B0,
            EFFECT_CALL_00BA3B10,
            EFFECT_CALL_00BA3B50,
            EFFECT_CALL_00BA3BA0,
            EFFECT_CALL_00BA3C00,
            EFFECT_CALL_00BA3C70,
            EFFECT_DESTRUCT_00BA3D30,
            NI_FREE_00AA10F0,
            VECTOR_DESTRUCT_00EC5FCE,
            TEXTURE_DESTRUCT_00BA3A00,
            MEMORY_DEALLOCATE,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(SHADER_MANAGER_CALL_00B54090, |_, _| {
            0x0e66_0000u32.into_ret()
        });
        e.register(ARRAY_SET_00BA3AA0, |e, a| {
            let word = e.mem.u32(a[2]);
            let elements = e.mem.u32(a[0] + 4);
            e.mem.set_u32(elements + 4 * a[1], word);
            Ret::default()
        });
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(SHADER_PARAM_CONSTRUCT, |_, a| a[0].into_ret());
        for page in [
            0x0120_0000u32,
            0x0120_2000,
            0x011f_9000,
            0x0107_0000,
            0x0101_d000,
        ] {
            e.map(page, 0x1000);
        }
        e.map(0x0101_7000, 0x1000);
        e.map(0x0101_3000, 0x1000);
        e.map(0x0109_9000, 0x1000);
        e.map(0x011a_d000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// An effect with the arrays at +0x18 (base +0x1c) and +0x28 (base +0x2c), six slots each.
    fn effect(e: &mut Engine) -> u32 {
        let this = e.mem.alloc(0xbc);
        let first = e.mem.alloc(0x18);
        let second = e.mem.alloc(0x18);
        e.mem.set_u32(this + 0x1c, first);
        e.mem.set_u32(this + 0x2c, second);
        this
    }

    fn element(e: &Engine, this: u32, base: u32, index: u32) -> u32 {
        let elements = e.mem.u32(this + base);
        e.mem.u32(elements + 4 * index)
    }

    #[test]
    fn fn_00bd6980_borrows_the_textures() {
        let mut e = engine();
        let this = effect(&mut e);
        let flags = e.mem.alloc(4);
        e.mem.set_u32(this + 0x5c, flags);
        e.mem.set_u8(flags + 3, 1);
        e.call(0x00bd_6980, &args![this, 0xdeadu32]);
        assert_eq!(
            calls(&e, BORROW_RENDERED_TEXTURE),
            vec![vec![this + 0x8c, 0x25], vec![this + 0x9c, 0x26]]
        );
        assert_eq!(element(&e, this, 0x2c, 2), this + 0x8c);
        assert_eq!(element(&e, this, 0x2c, 3), this + 0x9c);
        assert_eq!(element(&e, this, 0x2c, 5), this + 0x7c);
        assert_eq!(element(&e, this, 0x2c, 4), this + 0xac);
        assert!(calls(&e, RETURN_RENDERED_TEXTURE).is_empty());
        assert_eq!(
            calls(&e, TEXTURE_SET_00BA37F0),
            vec![vec![this + 0xac, 0x0e66_0000]]
        );
        // Without the byte: the texture at +0x7c is returned and element 5 cleared.
        e.mem.set_u8(flags + 3, 0);
        e.call(0x00bd_6980, &args![this, 0u32]);
        assert_eq!(calls(&e, RETURN_RENDERED_TEXTURE), vec![vec![this + 0x7c]]);
        assert_eq!(element(&e, this, 0x2c, 5), 0);
    }

    #[test]
    fn fn_00bd6a30_returns_the_textures() {
        let mut e = engine();
        let this = effect(&mut e);
        let elements = e.mem.u32(this + 0x2c);
        for index in 2..6u32 {
            e.mem.set_u32(elements + 4 * index, 0x0e70_0000 + index);
        }
        e.call(0x00bd_6a30, &args![this]);
        assert_eq!(
            calls(&e, RETURN_RENDERED_TEXTURE),
            vec![vec![0x0e70_0002], vec![0x0e70_0003], vec![this + 0x7c]]
        );
        for index in 2..6u32 {
            assert_eq!(element(&e, this, 0x2c, index), 0);
        }
        // Element 5 empty: no third return.
        e.call(0x00bd_6a30, &args![this]);
        assert_eq!(calls(&e, RETURN_RENDERED_TEXTURE).len(), 5);
    }

    #[test]
    fn fn_00bd6ab0_needs_a_positive_value_the_setting_and_the_manager() {
        let mut e = engine();
        let this = effect(&mut e);
        e.mem.set_f32(this + 0x70, 0.5);
        assert!(!e.call(0x00bd_6ab0, &args![this]).bool());
        e.set_global::<u8>(ENABLED_012023D4, 1);
        assert!(e.call(0x00bd_6ab0, &args![this]).bool());
        for value in [0.0f32, -1.0, f32::NAN] {
            e.mem.set_f32(this + 0x70, value);
            assert!(!e.call(0x00bd_6ab0, &args![this]).bool());
        }
        e.mem.set_f32(this + 0x70, 2.0);
        e.register(SHADER_MANAGER_CALL_00B54090, |_, _| Ret::default());
        assert!(!e.call(0x00bd_6ab0, &args![this]).bool());
    }

    #[test]
    fn fn_00bd66c0_sets_the_constants_of_both_parameters() {
        let mut e = engine();
        let this = effect(&mut e);
        let scene = e.mem.alloc(0x100);
        e.set_global::<u32>(SCENE_011F917C, scene);
        e.mem.set_f32(scene + 0xec, 2.0);
        e.mem.set_f32(scene + 0xf0, 5.0);
        for (offset, value) in [
            (0x6cu32, 1.0f32),
            (0x78, 2.0),
            (0x84, 3.0),
            (0x70, 4.0),
            (0x7c, 5.0),
            (0x88, 6.0),
        ] {
            e.mem.set_f32(scene + offset, value);
        }
        for (index, value) in [1.0f32, 0.5, 0.25].iter().enumerate() {
            e.set_global::<f32>(VECTOR_012004F0 + 4 * index as u32, *value);
        }
        e.set_global::<f32>(CONSTANT_012023E0, 7.0);
        e.set_global::<f32>(CONSTANT_01070218, -1.0e8);
        e.set_global::<f32>(THREE_01017718, 3.0);
        e.set_global::<f64>(THIRTY_0101DB88, 30.0);
        e.set_global::<f32>(DIVISOR_011F9410, 2.0);
        e.mem.set_f32(this + 0x68, 3.0);
        e.mem.set_f32(this + 0x6c, 1.0);
        e.mem.set_f32(this + 0x70, 9.0);
        e.mem.set_u32(this + 0x74, 2);
        e.mem.set_f32(this + 0x78, 120.0);
        let elements = e.mem.u32(this + 0x1c);
        e.mem.set_u32(elements + 8, 0x0e80_0000);
        e.mem.set_u32(elements + 0xc, 0x0e81_0000);
        assert!(e.call(0x00bd_66c0, &args![this, 0u32]).bool());
        let f = |v: f32| v.to_bits();
        let set = calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT);
        assert_eq!(set.len(), 7);
        for (parameter, rows) in [(0x0e80_0000u32, &set[0..3]), (0x0e81_0000, &set[3..6])] {
            assert_eq!(
                rows,
                &[
                    vec![parameter, 0, f(2.0), f(4.0), f(3.0), f(7.0)],
                    vec![parameter, 1, f(9.0), f(0.0), f(1.0), 0],
                    vec![parameter, 2, f(-1.0e8), f(2.0), f(3.0), f(10.0)],
                ]
            );
        }
        // 1*4 + 0.5*5 + 0.25*6 = 8, -(1*1 + 0.5*2 + 0.25*3) = -2.75, 120 / 60 = 2.
        assert_eq!(set[6], vec![0x0e81_0000, 3, f(8.0), f(-2.75), f(2.0), 0]);
        assert_eq!(e.global::<f32>(RESULT_012003C4), 3.0);
        // Mode 0 gives both flags; a divisor of 0 (or NaN) no constant 3.
        e.mem.set_u32(this + 0x74, 0);
        e.set_global::<f32>(DIVISOR_011F9410, 0.0);
        e.call(0x00bd_66c0, &args![this, 0u32]);
        let set = calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT);
        assert_eq!(set.len(), 13);
        assert_eq!(set[8][2..], [f(9.0), f(1.0), f(1.0), 0]);
        e.mem.set_u32(this + 0x74, 1);
        e.set_global::<f32>(DIVISOR_011F9410, f32::NAN);
        e.call(0x00bd_66c0, &args![this, 0u32]);
        let set = calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT);
        assert_eq!(set.len(), 19);
        assert_eq!(set[14][2..], [f(9.0), f(1.0), f(0.0), 0]);
    }

    #[test]
    fn fn_00bd6bb0_builds_the_four_passes() {
        let mut e = engine();
        // The renderer: slot +0xc8 gives an object whose +0x8c answers 0x400, +0x90 0x80000000.
        let slots: Vec<u32> = (0..0x40u32).map(|i| 0x0e91_0000 + 4 * i).collect();
        e.put_vtable(0x0e90_0000, &slots);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, 0x0e90_0000);
        e.set_global::<u32>(RENDERER_011F9508, renderer);
        e.register_double(0x0e91_0000 + 0xc8, move |_, _| renderer.into_ret());
        e.register(0x0e91_0000 + 0x8c, |_, _| 0x400u32.into_ret());
        e.register(0x0e91_0000 + 0x90, |_, _| 0x8000_0000u32.into_ret());
        e.set_global::<f32>(TWO_TO_32_01099378, 4_294_967_296.0);
        e.set_global::<f32>(CONSTANT_01013974, 1000.0);
        e.set_global::<f32>(CONSTANT_01013D84, 500.0);
        e.set_global::<f32>(CONSTANT_012023E0, 7.0);
        let this = effect(&mut e);
        let owner = e.mem.alloc(0xc);
        let textures = e.mem.alloc(0x108);
        e.mem.set_u32(owner + 8, textures);
        for offset in [0x48u32, 0xcc, 0xd0, 0x104] {
            e.mem.set_u32(textures + offset, 0x0ea0_0000 + offset);
        }
        e.call(0x00bd_6bb0, &args![this, owner, 0u32]);
        assert_eq!(calls(&e, EFFECT_SET_UP_00BA3F90), vec![vec![this, 4]]);
        let parameters: Vec<u32> = (0..4).map(|i| element(&e, this, 0x1c, i)).collect();
        assert_eq!(parameters[1], 0);
        let f = |v: f32| v.to_bits();
        assert_eq!(
            calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT),
            vec![
                vec![
                    parameters[0],
                    2,
                    f(1.0 / 1024.0),
                    f(1.0 / 2_147_483_648.0),
                    0,
                    0
                ],
                vec![parameters[2], 0, f(1.0), f(1000.0), f(500.0), f(7.0)],
                vec![parameters[3], 0, f(1.0), f(1000.0), f(500.0), f(7.0)],
            ]
        );
        assert_eq!(
            calls(&e, SHADER_PARAM_INIT_CONSTANT_MAP),
            vec![
                vec![parameters[0], 0, 0x18],
                vec![parameters[2], 0, 3],
                vec![parameters[3], 0, 4]
            ]
        );
        assert_eq!(
            calls(&e, SHADER_PARAM_CALL_00B8A8B0),
            vec![vec![parameters[0], 3, DATA_011ADD48, 0x40]]
        );
        assert_eq!(
            calls(&e, EFFECT_CALL_00BA3C70),
            vec![
                vec![this, 0, 0x0ea0_0104, parameters[0], 2],
                vec![this, 1, 0x0ea0_0048, 0, 2],
                vec![this, 2, 0x0ea0_00cc, parameters[2], 0],
                vec![this, 3, 0x0ea0_00d0, parameters[3], 0],
            ]
        );
        assert_eq!(calls(&e, EFFECT_CALL_00BA3B10), vec![vec![this, 0, 1, 1]]);
        assert_eq!(
            calls(&e, EFFECT_CALL_00BA3B50),
            vec![vec![this, 1, 2, 3, 0, 0]]
        );
        assert_eq!(
            calls(&e, EFFECT_CALL_00BA3BA0),
            vec![vec![this, 2, 1, 2, 4, 0, 1, 0]]
        );
        assert_eq!(
            calls(&e, EFFECT_CALL_00BA3C00),
            vec![vec![this, 3, 1, 2, 4, 5, 0, 1, 0, 0]]
        );
    }

    #[test]
    fn fn_00bd71e0_destroys_the_effect() {
        let mut e = engine();
        e.register(0x0e74_0000, |_, _| Ret::default());
        e.put_vtable(0x0e75_0000, &[0x0e74_0000]);
        let this = effect(&mut e);
        e.mem.set_u32(this + 0x5c, 0x0e57_0000);
        let parameter = e.mem.alloc(8);
        e.mem.set_u32(parameter, 0x0e75_0000);
        let elements = e.mem.u32(this + 0x1c);
        e.mem.set_u32(elements, parameter);
        e.mem.set_u32(elements + 0xc, parameter);
        assert_eq!(e.call(0x00bd_71e0, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010BC410);
        assert_eq!(
            calls(&e, 0x0e74_0000),
            vec![vec![parameter, 1], vec![parameter, 1]]
        );
        let cleared: Vec<u32> = calls(&e, ARRAY_SET_00BA3AA0).iter().map(|a| a[1]).collect();
        assert_eq!(cleared, vec![0, 3]);
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT_00EC5FCE),
            vec![vec![this + 0x8c, 0x10, 3, TEXTURE_DESTRUCT_00BA3A00]]
        );
        assert_eq!(
            calls(&e, TEXTURE_DESTRUCT_00BA3A00),
            vec![vec![this + 0x7c]]
        );
        assert_eq!(e.mem.u32(this + 0x58), ARRAY_VTABLE_010B9D24);
        assert_eq!(calls(&e, NI_FREE_00AA10F0), vec![vec![0x0e57_0000]]);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this]]
        );
        e.call(0x00bd_71e0, &args![this, 0u32]);
        assert_eq!(calls(&e, MEMORY_DEALLOCATE).len(), 1);
    }
}

#[cfg(test)]
mod n071_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::{
        object, rig, REFERENCE_DELETE, REFERENCE_VTABLE,
    };
    use crate::units::bsshader::shadowlightshader::n071_rig::{calls, slot_target, vobject};

    /// An object with room for `capacity` entries (+0x10, 16 bytes each) and words (+0x14).
    fn holder(e: &mut Engine, capacity: u16) -> u32 {
        let this = e.mem.alloc(0x28);
        let entries = e.mem.alloc(16 * u32::from(capacity) + 16);
        let words = e.mem.alloc(4 * u32::from(capacity) + 4);
        e.mem.set_u16(this + 0xc, capacity);
        e.mem.set_u32(this + 0x10, entries);
        e.mem.set_u32(this + 0x14, words);
        this
    }

    #[test]
    fn fn_00bd5d70_adds_until_full() {
        let mut e = rig();
        let this = holder(&mut e, 1);
        let entry = e.mem.alloc(16);
        for i in 0..4 {
            e.mem.set_u32(entry + 4 * i, 0x10 + i);
        }
        assert_eq!(e.call(0x00bd_5d70, &args![this, 0u32, 5u32]).u32(), 0);
        assert_eq!(e.call(0x00bd_5d70, &args![this, entry, 5u32]).u32(), 1);
        let entries = e.mem.u32(this + 0x10);
        assert_eq!(e.mem.u32(entries + 0xc), 0x13);
        assert_eq!(e.mem.u32(e.mem.u32(this + 0x14)), 5);
        assert_eq!(e.mem.u16(this + 0xe), 1);
        // Full: the count with its low byte cleared (0x0101 -> 0x0100).
        e.mem.set_u16(this + 0xc, 0x101);
        e.mem.set_u16(this + 0xe, 0x101);
        assert_eq!(e.call(0x00bd_5d70, &args![this, entry, 5u32]).u32(), 0x100);
    }

    #[test]
    fn fn_00bd5dd0_copies_and_clears_the_bits() {
        let mut e = rig();
        let this = e.mem.alloc(0x28);
        let object = e.mem.alloc(0x40);
        e.mem.set_u32(this + 8, object);
        e.mem.set_u32(object + 0x24, 0x1c00 | 0x1);
        e.mem.set_u32(object + 0x38, 9);
        let words = e.mem.alloc(12);
        e.mem.set_u32(words + 8, 0x88);
        e.call(0x00bd_5dd0, &args![this, words]);
        assert_eq!(e.mem.u32(this + 0x20), 0x88);
        assert_eq!(e.mem.u32(object + 0x24), 1);
        assert_eq!(e.mem.u32(object + 0x38), 0);
        e.mem.set_u32(object + 0x38, 9);
        e.call(0x00bd_5dd0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(object + 0x38), 9);
    }

    /// A holder of two entries (one below the lowest height), its +4 object with radius extra
    /// 1.0 and a +0 object whose RTTI chain holds `012024c0` when `kind` says so.
    fn bound_rig(found: bool) -> (Engine, u32, u32) {
        let mut e = rig();
        for page in [0x0101_1000u32, 0x0109_4000, 0x010b_c000, 0x0120_2000] {
            e.map(page, 0x1000);
        }
        e.set_global(START_010BC3EC, 5.0e9f32);
        e.set_global(LOWEST_01094CF4, -2.0e9f32);
        e.set_global(HALF_01011588, 0.5f64);
        e.register(CRT_SQRT_00EC6040, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        let this = holder(&mut e, 4);
        let entries = e.mem.u32(this + 0x10);
        for (i, point) in [[0.0f32, 0.0, 0.0], [6.0, 8.0, 0.0], [100.0, 100.0, -3.0e9]]
            .iter()
            .enumerate()
        {
            for (axis, value) in point.iter().enumerate() {
                e.mem
                    .set_f32(entries + 16 * i as u32 + 4 * axis as u32, *value);
            }
        }
        e.mem.set_u16(this + 0xe, 3);
        let extra = e.mem.alloc(0x30);
        e.mem.set_f32(extra + 0x28, 1.0);
        e.mem.set_u32(this + 4, extra);
        let base_record = e.mem.alloc(8);
        let record = e.mem.alloc(8);
        e.mem.set_u32(record + 4, base_record);
        if found {
            e.mem.set_u32(base_record + 4, RTTI_012024C0);
        }
        let target = vobject(&mut e, 8, &[(8, record)]);
        e.mem.set_u32(this, target);
        (e, this, target)
    }

    #[test]
    fn fn_00bd5e40_hands_the_bound_to_the_object() {
        let (mut e, this, target) = bound_rig(true);
        e.call(0x00bd_5e40, &args![this]);
        // Centre (3, 4, 0), radius 5 + 1.
        assert_eq!(
            calls(&e, slot_target(&e, target, 0xfc)),
            vec![vec![
                target,
                3.0f32.to_bits(),
                4.0f32.to_bits(),
                0,
                6.0f32.to_bits()
            ]]
        );
        assert_eq!(
            calls(&e, slot_target(&e, target, 0x100)),
            vec![vec![target, 3]]
        );
        assert!(calls(&e, FIND_00653290).is_empty());
    }

    #[test]
    fn fn_00bd5e40_asks_00653290_without_the_record() {
        let (mut e, this, target) = bound_rig(false);
        let other = vobject(&mut e, 8, &[]);
        e.register_double(FIND_00653290, move |_, _| other.into_ret());
        e.call(0x00bd_5e40, &args![this]);
        assert_eq!(calls(&e, FIND_00653290), vec![vec![target, RTTI_012024B8]]);
        assert_eq!(calls(&e, slot_target(&e, other, 0xfc)).len(), 1);
        assert!(calls(&e, slot_target(&e, target, 0xfc)).is_empty());
        // Nothing found: nothing more.
        e.register(FIND_00653290, |_, _| 0u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00bd_5e40, &args![this]);
        assert!(calls(&e, slot_target(&e, other, 0x100)).is_empty());
        // Without the +4 object nothing is asked.
        e.mem.set_u32(this + 4, 0);
        e.call_log = Some(vec![]);
        e.call(0x00bd_5e40, &args![this]);
        assert!(calls(&e, CRT_SQRT_00EC6040).is_empty());
    }

    #[test]
    fn fn_00bd6150_empties_then_bounds() {
        let (mut e, this, target) = bound_rig(true);
        let entries = e.mem.u32(this + 0x10);
        e.mem.set_f32(entries + 0x18, 7.0);
        e.mem.set_u32(e.mem.u32(this + 0x14) + 4, 9);
        e.call(0x00bd_6150, &args![this]);
        assert_eq!(e.mem.f32(entries + 0x18), 0.0);
        assert_eq!(e.mem.f32(entries + 0x10), 6.0);
        assert_eq!(e.mem.u32(e.mem.u32(this + 0x14) + 4), 0);
        assert_eq!(e.mem.u16(this + 0xe), 0);
        // No entries: the bound is 0 at 0 with radius 1.
        assert_eq!(
            calls(&e, slot_target(&e, target, 0xfc)),
            vec![vec![target, 0, 0, 0, 1.0f32.to_bits()]]
        );
    }

    #[test]
    fn fn_00bd6190_builds_the_geometry() {
        let mut e = rig();
        for page in [
            0x0101_1000u32,
            0x0109_4000,
            0x010b_c000,
            0x011f_9000,
            0x0120_2000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(SHADER_MODEL_011F91C0, 2i32);
        for target in [
            MEMBER_CONSTRUCT_00B57E70,
            ATTACH_PROPERTY,
            PREPARE_OBJECT,
            ADDITIONAL_DATA_SET_BLOCK_COUNT,
            ADDITIONAL_DATA_SET_BLOCK,
            ADDITIONAL_DATA_SET_STREAM,
            GEOMETRY_DATA_00A67260,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(NI_ALLOCATE_00AA13E0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(GEOMETRY_CONSTRUCT_00C1A660, |e, a| {
            e.mem.set_u32(a[0], REFERENCE_VTABLE);
            let data = e.mem.alloc(0x40);
            e.mem.set_u32(a[0] + 0xb8, data);
            a[0].into_ret()
        });
        e.register(PROPERTY_CONSTRUCT_00B57F90, |e, a| {
            e.mem.set_u32(a[0], REFERENCE_VTABLE);
            e.mem.set_u32(a[0] + 0x20, 4);
            a[0].into_ret()
        });
        e.register(ADDITIONAL_DATA_CONSTRUCT, |_, a| a[0].into_ret());
        let source = object(&mut e, 0);
        e.mem.set_u32(source + 8, 0x88);
        e.mem.set_u32(source + 0x10, 0x1010);
        e.mem.set_u32(source + 0x14, 3);
        let texture = object(&mut e, 0);
        e.mem.set_u32(source + 0x18, texture);
        e.mem.set_u32(source + 0x1c, 0x1c1c);
        e.mem.set_u32(source + 0x24, 0x2424);
        e.mem.set_u16(source + 0x2c, 2);
        e.mem.set_u8(source + 0x30, 1);
        let this = e.mem.alloc(0x28);
        assert_eq!(
            e.call(0x00bd_6190, &args![this, source, 7u32, 0u32]).u32(),
            this
        );
        assert_eq!(
            calls(&e, MEMBER_CONSTRUCT_00B57E70),
            vec![vec![this + 0x18]]
        );
        assert_eq!(e.mem.u16(this + 0xc), 2);
        assert_eq!(e.mem.u16(this + 0xe), 0);
        assert_eq!(e.mem.u32(this + 4), source);
        assert_eq!(e.mem.u32(source + 4), 1);
        assert_eq!(e.mem.u32(this + 0x24), 7);
        let geometry = e.mem.u32(this);
        assert_eq!(
            calls(&e, GEOMETRY_CONSTRUCT_00C1A660),
            vec![vec![geometry, 0x88]]
        );
        assert_eq!(e.mem.u32(geometry + 4), 1);
        let property = e.mem.u32(this + 8);
        assert_eq!(e.mem.u32(property + 0x80), texture);
        assert_eq!(e.mem.u32(texture + 4), 1);
        assert_eq!(e.mem.u32(property + 0x7c), this);
        // Flag 0 clears bit 2 (and +0x38), the source byte sets bit 13.
        assert_eq!(e.mem.u32(property + 0x20), 0);
        assert_eq!(e.mem.u32(property + 0x24), 0x2000);
        assert_eq!(
            calls(&e, ATTACH_PROPERTY),
            vec![
                vec![geometry, 0x1c1c],
                vec![geometry, 0x2424],
                vec![geometry, property]
            ]
        );
        assert_eq!(calls(&e, PREPARE_OBJECT), vec![vec![geometry, 0, 0]]);
        let additional = calls(&e, ADDITIONAL_DATA_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, ADDITIONAL_DATA_CONSTRUCT)[0][1..].to_vec(),
            vec![6, 1]
        );
        assert_eq!(
            calls(&e, ADDITIONAL_DATA_SET_BLOCK),
            vec![vec![additional, 0, 0x1010, 24, 0]]
        );
        assert_eq!(
            calls(&e, ADDITIONAL_DATA_SET_STREAM),
            vec![vec![additional, 0, 0, 0, 1, 6, 4, 4]]
        );
        let data = e.mem.u32(geometry + 0xb8);
        assert_eq!(
            calls(&e, GEOMETRY_DATA_00A67260),
            vec![vec![data, additional]]
        );
    }

    #[test]
    fn fn_00bd6570_releases_everything() {
        let mut e = rig();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        let this = e.mem.alloc(0x28);
        let owner = vobject(&mut e, 8, &[]);
        let geometry = object(&mut e, 1);
        e.mem.set_u32(geometry + 0x18, owner);
        let held: Vec<u32> = (0..2).map(|_| object(&mut e, 1)).collect();
        e.mem.set_u32(this, geometry);
        e.mem.set_u32(this + 4, held[0]);
        e.mem.set_u32(this + 8, held[1]);
        e.mem.set_u32(this + 0x14, 0x1414);
        // No entries, so 00bd6150 runs 00bd5e40 with the +4 object gone: nothing more.
        e.call(0x00bd_6570, &args![this]);
        assert_eq!(
            calls(&e, slot_target(&e, owner, 0xe8)),
            vec![vec![owner, geometry]]
        );
        assert_eq!(calls(&e, REFERENCE_DELETE).len(), 3);
        for offset in [0u32, 4, 8, 0x10, 0x14] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x1414]]
        );
    }

    #[test]
    fn fn_00bd6fa0_destroys_and_frees() {
        let mut e = rig();
        e.register(MEMORY_DEALLOCATE, |_, _| Ret::default());
        e.register(NI_FREE_00AA10F0, |_, _| Ret::default());
        e.register(EFFECT_DESTRUCT_00BA3D30, |_, _| Ret::default());
        let this = e.mem.alloc(0x60);
        assert_eq!(e.call(0x00bd_6fa0, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, MEMORY_DEALLOCATE).is_empty());
        e.call(0x00bd_6fa0, &args![this, 1u32]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE),
            vec![vec![MEMORY_MANAGER, this]]
        );
        assert_eq!(
            calls(&e, EFFECT_DESTRUCT_00BA3D30),
            vec![vec![this], vec![this]]
        );
    }
}
