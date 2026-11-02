//! `BSShader/run_00b9b380` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays) and this effect's vtable
/// (`010b7e6c`; no RTTI locator before it).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const VTABLE_010B7E6C: u32 = 0x010b_7e6c;
/// The global that keeps the effect (`011ffe44`), the 256-byte table it copies twice
/// (`011ad900`) and the fifteen `float`s it clears (`011ffe4c` to `011ffe94`, three of every
/// four).
const EFFECT_011FFE44: u32 = 0x011f_fe44;
const TABLE_011AD900: u32 = 0x011a_d900;
const FLOATS_011FFE4C: u32 = 0x011f_fe4c;

// Translated from 00b9b830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`): an image-space effect's constructor,
/// `thiscall`, no stack words. Runs the base constructor `00ba4080`, stores the vtable
/// `010b7e6c`, clears four 16-byte blocks at +0x58, +0x68, +0x78 and +0x88 (three bytes, then
/// three words each), keeps the effect in `011ffe44`, clears the byte +0x2b8, copies the 256
/// bytes at `011ad900` to +0x198 and to +0x98 (byte by byte, both copies per byte) and stores
/// 0.0 in the `float`s `011ffe4c`, `011ffe50`, `011ffe54`, `011ffe5c` .. `011ffe94` (three of
/// each four words). Answers `this`.
pub fn fn_00b9b830(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B7E6C);
    for block in [0x58u32, 0x68, 0x78, 0x88] {
        for byte in 0..3u32 {
            e.mem.set_u8(base + block + byte, 0);
        }
        for word in [4u32, 8, 0xc] {
            e.mem.set_u32(base + block + word, 0);
        }
    }
    e.set_global(EFFECT_011FFE44, base);
    e.mem.set_u8(base + 0x2b8, 0);
    for index in 0..0x100u32 {
        let byte = e.mem.u8(TABLE_011AD900 + index);
        e.mem.set_u8(base + 0x198 + index, byte);
        e.mem.set_u8(base + 0x98 + index, byte);
    }
    for group in 0..5u32 {
        for word in 0..3u32 {
            e.mem.set_u32(FLOATS_011FFE4C + 0x10 * group + 4 * word, 0);
        }
    }
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{MEMORY_ALLOCATE, MEMORY_MANAGER};

/// The destructor `00b9aa80` (unit `bsbatchrenderer.cpp`, unnamed, `thiscall`) and the sized
/// free `00aa1460` (unnamed, `cdecl`: the block, its size).
const DESTRUCT_00B9AA80: u32 = 0x00b9_aa80;
const FREE_SIZED_00AA1460: u32 = 0x00aa_1460;

// Translated from 00b9b380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`): a scalar deleting destructor, `thiscall`, one
/// word: the destructor `00b9aa80`, then the 0x98-byte block freed (`00aa1460`) when bit 0 of
/// `flags` is set. Answers `this`.
pub fn fn_00b9b380(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(DESTRUCT_00B9AA80, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE_SIZED_00AA1460, &args![this, 0x98u32]);
    }
    this
}

/// `00b9abc0` (unit `bsbatchrenderer.cpp`, unnamed; `thiscall`, two words).
const BATCH_00B9ABC0: u32 = 0x00b9_abc0;

// Translated from 00b9b3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`), `thiscall`, three words: `00b9abc0` on the
/// object `index` of the array at +0x3c with the first and the third word; answers its
/// result.
pub fn fn_00b9b3b0(e: &mut Engine, this: Ptr, value: u32, index: u32, extra: u32) -> u32 {
    let object = e.mem.u32(
        this.addr()
            .wrapping_add(index.wrapping_mul(4))
            .wrapping_add(0x3c),
    );
    e.call(BATCH_00B9ABC0, &args![object, value, extra]).u32()
}

// Translated from 00b9b3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`), `thiscall`, two words: `00b9abc0` with the
/// first word and 0 on the object `index` of the array at +0x90 when the word +0x34 is above 0
/// (signed), else of the array at +0x70; answers its result.
pub fn fn_00b9b3d0(e: &mut Engine, this: Ptr, value: u32, index: u32) -> u32 {
    let base = this.addr();
    let array = if (e.mem.u32(base + 0x34) as i32) > 0 {
        0x90
    } else {
        0x70
    };
    let object = e
        .mem
        .u32(base.wrapping_add(index.wrapping_mul(4)).wrapping_add(array));
    e.call(BATCH_00B9ABC0, &args![object, value, 0u32]).u32()
}

// Translated from 00b9b400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`), `thiscall`, one word (a seed): the start is
/// `(seed * 0x5391313d + 0x22fc2945) >> 24`; byte `i` of the two 256-byte tables at +0x198 and
/// +0x98 becomes byte `(start + i) & 0xff` of the table at `011ad900`.
pub fn fn_00b9b400(e: &mut Engine, this: Ptr, seed: u32) {
    let start = seed.wrapping_mul(0x5391_313d).wrapping_add(0x22fc_2945) >> 24;
    for index in 0..0x100u32 {
        let byte = e.mem.u8(TABLE_011AD900 + ((index + start) & 0xff));
        e.mem.set_u8(this.addr() + 0x198 + index, byte);
        e.mem.set_u8(this.addr() + 0x98 + index, byte);
    }
}

/// The array setter `00ba3aa0` (`thiscall` on an `NiTPrimitiveArray`: index, the address of
/// the word), `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`),
/// `ImageSpaceEffect::~ImageSpaceEffect` (`00ba3d30`) and `MemoryManager::Deallocate`
/// (`00aa4060`, `thiscall` on the manager, one word) (engine map).
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
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

// Translated from 00b9b450 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b9b450 - the C++ exception frame (FS:[0] chain, unwind states 4 to 0 for the textures and the base) is not translated.
/// Unnamed in the engine map (`run_00b9b380`): the destructor of the effect `00b9b830` builds,
/// `thiscall`, no stack words. Stores the vtable `010b7e6c`; effect parameters 0 and 1 (+0x1c
/// elements), when set, are deleted through their slot +0 with 1 (each tested twice) and
/// their elements set to null; the textures at +0x88, +0x78, +0x68 and +0x58 are destroyed;
/// then `ImageSpaceEffect::~ImageSpaceEffect`.
pub fn fn_00b9b450(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010B7E6C);
    for index in 0..2u32 {
        let element = e.mem.u32(base + 0x1c) + 4 * index;
        if e.mem.u32(element) != 0 {
            let param = e.mem.u32(element);
            if param != 0 {
                e.vcall(param, 0, &args![1u32]);
            }
            set_at(e, base + 0x18, index, 0);
        }
    }
    for offset in [0x88u32, 0x78, 0x68, 0x58] {
        e.call(TEXTURE_DESTRUCT_00BA3A00, &args![base + offset]);
    }
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

// Translated from 00b9b930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`): the scalar deleting destructor of the effect
/// `00b9b830` builds, `thiscall`, one word: `00b9b450`, then the memory freed
/// (`MemoryManager::Deallocate`) when bit 0 of `flags` is set. Answers `this`.
pub fn fn_00b9b930(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b9b450(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The effect-array sizer `00ba3f90` (`thiscall`, one word), the `NiTArray` set-size
/// `00a90180`, `ImageSpaceShaderParam`'s constructor (`00b8a9e0`) and `InitConstantMap`
/// (`00b8ab10`, two words), the effect setter `00ba3c70` (unit `run_00ba3aa0`) and the
/// `float` seed `011f9410`.
const SIZE_ARRAYS_00BA3F90: u32 = 0x00ba_3f90;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
const PARAM_CONSTRUCT_00B8A9E0: u32 = 0x00b8_a9e0;
const PARAM_INIT_CONSTANT_MAP_00B8AB10: u32 = 0x00b8_ab10;
const SET_EFFECT_00BA3C70: u32 = 0x00ba_3c70;
const SEED_011F9410: u32 = 0x011f_9410;

// Translated from 00b9b950 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b9b950 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the two parameter allocations) is not translated.
/// Unnamed in the engine map (`run_00b9b380`): the `Setup` slot of the effect `00b9b830`
/// builds, `thiscall`, two words (the manager; a word it does not read). Sizes the arrays to 2
/// (`00ba3f90`) and `pTextures` to 4. The first time (byte +0x2b8 clear) the tables are filled
/// (`00b9b400`) from the `float` seed `011f9410` truncated (`FISTP qword` with the rounding
/// set to chop; its low word) and the byte set. Then two parameters (0x88 bytes; the
/// constructor only on a block) with `InitConstantMap(3, 6)` and the byte +0x55 cleared,
/// stored at 0 and 1 and given with the manager's effects at +0x1c8 and +0x1cc to effects 0
/// and 1 (output 0).
pub fn fn_00b9b950(e: &mut Engine, this: Ptr, manager: u32, _unused_1: u32) {
    let base = this.addr();
    e.call(SIZE_ARRAYS_00BA3F90, &args![base, 2u32]);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 4u32]);
    if e.mem.u8(base + 0x2b8) == 0 {
        let seed = X87Ext::load32(e.global::<u32>(SEED_011F9410));
        let seed = x87_fistp64_chop(seed) as u32;
        fn_00b9b400(e, this, seed);
        e.mem.set_u8(base + 0x2b8, 1);
    }
    for index in 0..2u32 {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x88u32])
            .u32();
        let param = if block != 0 {
            e.call(PARAM_CONSTRUCT_00B8A9E0, &args![block]).u32()
        } else {
            0
        };
        e.call(PARAM_INIT_CONSTANT_MAP_00B8AB10, &args![param, 3u32, 6u32]);
        e.mem.set_u8(param + 0x55, 0);
        set_at(e, base + 0x18, index, param);
        let effect = e.mem.u32(e.mem.u32(manager + 8) + 0x1c8 + 4 * index);
        e.call(
            SET_EFFECT_00BA3C70,
            &args![base, index, effect, param, 0u32],
        );
    }
}

/// The `float` divisor `011ad834` and the `double` floor `01012060`.
const DIVISOR_011AD834: u32 = 0x011a_d834;
const FLOOR_01012060: u32 = 0x0101_2060;

// Translated from 00b9bb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`), `thiscall`, two words (a `float`, a byte).
/// Nothing when the `float` equals +0xd8 (`FUCOMP`; a NaN never does). Else with the byte set
/// +0xdc becomes 1.0; without it `q` = +0xdc / `[011ad834]` (stored as a `float`), 0 when below
/// the `double` at `01012060`, at most 1.0 (`FCOM`; a NaN kept), and +0xdc becomes
/// `[011ad834] * (1 - q)`. The `float` is stored at +0xd8 (through the FPU, a signalling NaN
/// quieted).
pub fn fn_00b9bb10(e: &mut Engine, this: Ptr, value: u32, reset: u8) {
    let base = this.addr();
    let current = x87_load32(e.mem.u32(base + 0xd8));
    let value = x87_load32(value);
    if value == current {
        return;
    }
    let time = if reset != 0 {
        0x3f80_0000
    } else {
        let previous = x87_load32(e.mem.u32(base + 0xdc));
        let divisor = x87_load32(e.global::<u32>(DIVISOR_011AD834));
        let ratio = x87_load32(x87_store32(x87_div(previous, divisor)));
        let floor = e.global::<f64>(FLOOR_01012060);
        let ratio = if ratio < floor { 0.0 } else { ratio };
        let ratio = x87_store32(ratio);
        let clamped = if 1.0 < x87_load32(ratio) {
            0x3f80_0000
        } else {
            ratio
        };
        x87_store32(x87_mul(divisor, x87_sub(1.0, x87_m32(clamped))))
    };
    e.mem.set_u32(base + 0xdc, time);
    e.mem.set_u32(base + 0xd8, x87_store32(value));
}

/// `ImageSpaceTexture::BorrowRenderedTexture` (`00ba3840`; `thiscall`, one word) and
/// `::ReturnRenderedTexture` (`00ba39a0`; `thiscall`, no stack words),
/// `ImageSpaceShaderParam::SetPixelConstant` (`00b8a790`; `thiscall`: index, four `float`s),
/// `ImageSpaceEffect::SetTexture` (`00ba3cb0`; `thiscall`: index, texture, filter mode) (engine
/// map) and `00ba3760` (unnamed; `thiscall` on a texture, one word).
const BORROW_RENDERED_TEXTURE_00BA3840: u32 = 0x00ba_3840;
const RETURN_RENDERED_TEXTURE_00BA39A0: u32 = 0x00ba_39a0;
const SET_PIXEL_CONSTANT_00B8A790: u32 = 0x00b8_a790;
const SET_TEXTURE_00BA3CB0: u32 = 0x00ba_3cb0;
const TEXTURE_00BA3760: u32 = 0x00ba_3760;

/// An effect of the array at `+0xc`: the pointer, null when it is null or 0x6c (the compiler's
/// adjustment of a null base pointer).
fn sub_effect(e: &Engine, this: u32, index: u32) -> u32 {
    let effect = e.mem.u32(e.mem.u32(this + 0xc) + 4 * index);
    if effect == 0 || effect.wrapping_sub(0x6c) == 0 {
        0
    } else {
        effect
    }
}

/// Pixel constants 0 to 3 of `param`: (`011ffe4c`, `011ffe50`, 0, 0), (`011ffe5c`,
/// `011ffe60`, 0, 0), (`011ffe6c`, `011ffe70`, 0, 0), (`011ffe48`, 0, 0, 0), each `float`
/// through `FLD`/`FSTP` (a signalling NaN quieted).
fn set_common_constants(e: &mut Engine, param: u32) {
    let float = |e: &Engine, addr: u32| x87_quiet_bits(e.global::<u32>(addr));
    for (index, first) in [(0u32, 0x011f_fe4cu32), (1, 0x011f_fe5c), (2, 0x011f_fe6c)] {
        let x = float(e, first);
        let y = float(e, first + 4);
        e.call(
            SET_PIXEL_CONSTANT_00B8A790,
            &args![param, index, x, y, 0u32, 0u32],
        );
    }
    let x = float(e, 0x011f_fe48);
    e.call(
        SET_PIXEL_CONSTANT_00B8A790,
        &args![param, 3u32, x, 0u32, 0u32, 0u32],
    );
}

// Translated from 00b9b540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`): the `Render` slot of the effect `00b9b830`
/// builds, `thiscall`, four words (two passed on to the effects' slot +4, one to `this`'s slot
/// +0x1c, one not read). With the byte +5 set it becomes what slot +0x1c answers for the third
/// word. Borrows a rendered texture of kind 0x32 into +0x68. Effect 0 (`+0xc` array, null for
/// a null or 0x6c pointer) with parameter 0 (`+0x1c` array): pixel constants 0 to 3 (see
/// `set_common_constants`), 4 (`011ffe7c` to `011ffe84`, 0) and 5 (`011ffe8c` to `011ffe94`,
/// 0); textures 0 = +0x68 (filter 0) and 1 = texture 1 of `pTextures` (+0x2c, filter 1);
/// `00ba3760(3)` on textures 0 and 1; its slot +4 with the first two words, the parameter
/// and 1. Effect 1 with parameter 1: constants 0 to 3; textures 0 = texture 0 of `pTextures`
/// and 1 = +0x68 (filters 0); `00ba3760(3)` twice; slot +4 likewise. Last the borrowed
/// texture is given back (`00ba39a0`).
pub fn fn_00b9b540(
    e: &mut Engine,
    this: Ptr,
    first: u32,
    second: u32,
    active: u32,
    _unused_3: u32,
) {
    let base = this.addr();
    if e.mem.u8(base + 5) != 0 {
        let value = e.vcall(base, 0x1c, &args![active]).u8();
        e.mem.set_u8(base + 5, value);
    }
    let borrowed = base + 0x68;
    e.call(BORROW_RENDERED_TEXTURE_00BA3840, &args![borrowed, 0x32u32]);
    let effect = sub_effect(e, base, 0);
    let param = e.mem.u32(e.mem.u32(base + 0x1c));
    set_common_constants(e, param);
    let float = |e: &Engine, addr: u32| x87_quiet_bits(e.global::<u32>(addr));
    for (index, first_float) in [(4u32, 0x011f_fe7cu32), (5, 0x011f_fe8c)] {
        let x = float(e, first_float);
        let y = float(e, first_float + 4);
        let z = float(e, first_float + 8);
        e.call(
            SET_PIXEL_CONSTANT_00B8A790,
            &args![param, index, x, y, z, 0u32],
        );
    }
    e.call(SET_TEXTURE_00BA3CB0, &args![effect, 0u32, borrowed, 0u32]);
    let texture_1 = e.mem.u32(e.mem.u32(base + 0x2c) + 4);
    e.call(SET_TEXTURE_00BA3CB0, &args![effect, 1u32, texture_1, 1u32]);
    for index in 0..2u32 {
        let texture = e.mem.u32(e.mem.u32(base + 0x2c) + 4 * index);
        e.call(TEXTURE_00BA3760, &args![texture, 3u32]);
    }
    e.vcall(effect, 4, &args![first, second, param, 1u32]);
    let effect = sub_effect(e, base, 1);
    let param = e.mem.u32(e.mem.u32(base + 0x1c) + 4);
    set_common_constants(e, param);
    let texture_0 = e.mem.u32(e.mem.u32(base + 0x2c));
    e.call(SET_TEXTURE_00BA3CB0, &args![effect, 0u32, texture_0, 0u32]);
    e.call(SET_TEXTURE_00BA3CB0, &args![effect, 1u32, borrowed, 0u32]);
    for index in 0..2u32 {
        let texture = e.mem.u32(e.mem.u32(base + 0x2c) + 4 * index);
        e.call(TEXTURE_00BA3760, &args![texture, 3u32]);
    }
    e.vcall(effect, 4, &args![first, second, param, 1u32]);
    e.call(RETURN_RENDERED_TEXTURE_00BA39A0, &args![borrowed]);
}

/// The tables' fill the first time: `00b9b400` with the seed `011f9410` truncated (`FISTP
/// qword` with the rounding set to chop; its low word), then the byte +0x2b8 set (the exe
/// keeps `this` in ECX across `00b9b400`, which does not change it).
fn fill_tables(e: &mut Engine, this: Ptr) {
    let seed = X87Ext::load32(e.global::<u32>(SEED_011F9410));
    let seed = x87_fistp64_chop(seed) as u32;
    fn_00b9b400(e, this, seed);
    e.mem.set_u8(this.addr() + 0x2b8, 1);
}

// Translated from 00b9b7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`), `thiscall`, no stack words: fills the tables
/// from the seed `011f9410` (`00b9b400`, the seed truncated) and sets the byte +0x2b8.
pub fn fn_00b9b7f0(e: &mut Engine, this: Ptr) {
    fill_tables(e, this);
}

// Translated from 00b9bac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b9b380`), `thiscall`, one word it does not read: the
/// first time (byte +0x2b8 clear) fills the tables as `00b9b7f0` does. Answers true.
pub fn fn_00b9bac0(e: &mut Engine, this: Ptr, _unused_0: u32) -> bool {
    if e.mem.u8(this.addr() + 0x2b8) == 0 {
        fill_tables(e, this);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b9_b540, fn_00b9b540(Ptr, u32, u32, u32, u32)),
        entry!(0x00b9_b7f0, fn_00b9b7f0(Ptr)),
        entry!(0x00b9_bac0, fn_00b9bac0(Ptr, u32) -> bool),
        entry!(0x00b9_b830, fn_00b9b830(Ptr) -> Ptr),
        entry!(0x00b9_b380, fn_00b9b380(Ptr, u32) -> Ptr),
        entry!(0x00b9_b3b0, fn_00b9b3b0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00b9_b3d0, fn_00b9b3d0(Ptr, u32, u32) -> u32),
        entry!(0x00b9_b400, fn_00b9b400(Ptr, u32)),
        entry!(0x00b9_b450, fn_00b9b450(Ptr)),
        entry!(0x00b9_b930, fn_00b9b930(Ptr, u32) -> Ptr),
        entry!(0x00b9_b950, fn_00b9b950(Ptr, u32, u32)),
        entry!(0x00b9_bb10, fn_00b9bb10(Ptr, u32, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00b9b830_builds_the_effect_and_its_tables() {
        let mut e = Engine::new();
        e.register(EFFECT_CONSTRUCT_00BA4080, |_, a| a[0].into_ret());
        e.map(0x011a_d000, 0x1000);
        e.map(0x011f_f000, 0x1000);
        let table: Vec<u8> = (0..=255u8).map(|b| b ^ 0x5a).collect();
        e.mem.write(TABLE_011AD900, &table);
        e.mem.write(FLOATS_011FFE4C - 4, &[0xff; 0x50]);
        e.call_log = Some(vec![]);
        let this = e.mem.alloc(0x2bc);
        e.mem.write(this, &[0xcc; 0x2bc]);
        assert_eq!(e.call(0x00b9_b830, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B7E6C);
        for block in [0x58u32, 0x68, 0x78, 0x88] {
            assert_eq!(e.mem.bytes(this + block, 3), vec![0; 3]);
            assert_eq!(e.mem.u8(this + block + 3), 0xcc);
            assert_eq!(e.mem.bytes(this + block + 4, 0xc), vec![0; 0xc]);
        }
        assert_eq!(e.mem.bytes(this + 0x98, 0x100), table);
        assert_eq!(e.mem.bytes(this + 0x198, 0x100), table);
        assert_eq!(e.mem.u8(this + 0x2b8), 0);
        assert_eq!(e.mem.u8(this + 0x2b9), 0xcc);
        assert_eq!(e.global::<u32>(EFFECT_011FFE44), this);
        for group in 0..5u32 {
            let first = FLOATS_011FFE4C + 0x10 * group;
            assert_eq!(e.mem.bytes(first, 0xc), vec![0; 0xc]);
            assert_eq!(e.mem.u32(first - 4), 0xffff_ffff);
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (EFFECT_CONSTRUCT_00BA4080, vec![this]));
    }
}

#[cfg(test)]
mod more_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    #[test]
    fn fn_00b9b380_destroys_and_frees_the_block() {
        let mut e = rig();
        e.register(DESTRUCT_00B9AA80, |_, _| Ret::default());
        e.register(FREE_SIZED_00AA1460, |_, _| Ret::default());
        assert_eq!(e.call(0x00b9_b380, &args![0x2000u32, 0u32]).u32(), 0x2000);
        assert!(calls(&e, FREE_SIZED_00AA1460).is_empty());
        assert_eq!(e.call(0x00b9_b380, &args![0x2000u32, 1u32]).u32(), 0x2000);
        assert_eq!(calls(&e, DESTRUCT_00B9AA80).len(), 2);
        assert_eq!(calls(&e, FREE_SIZED_00AA1460), vec![vec![0x2000, 0x98]]);
    }

    #[test]
    fn fn_00b9b3b0_and_00b9b3d0_forward_to_the_indexed_object() {
        let mut e = rig();
        e.register(BATCH_00B9ABC0, |_, a| (a[1] + a[2]).into_ret());
        let this = e.mem.alloc(0xa0);
        e.mem.set_u32(this + 0x3c + 8, 0x3c3c);
        e.mem.set_u32(this + 0x70 + 4, 0x7070);
        e.mem.set_u32(this + 0x90 + 4, 0x9090);
        assert_eq!(
            e.call(0x00b9_b3b0, &args![this, 5u32, 2u32, 7u32]).u32(),
            12
        );
        assert_eq!(calls(&e, BATCH_00B9ABC0), vec![vec![0x3c3c, 5, 7]]);
        assert_eq!(e.call(0x00b9_b3d0, &args![this, 3u32, 1u32]).u32(), 3);
        e.mem.set_u32(this + 0x34, 1);
        e.call(0x00b9_b3d0, &args![this, 3u32, 1u32]);
        e.mem.set_u32(this + 0x34, 0xffff_ffff);
        e.call(0x00b9_b3d0, &args![this, 3u32, 1u32]);
        let objects: Vec<u32> = calls(&e, BATCH_00B9ABC0)[1..]
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(objects, vec![0x7070, 0x9090, 0x7070]);
        assert_eq!(calls(&e, BATCH_00B9ABC0)[1], vec![0x7070, 3, 0]);
    }

    #[test]
    fn fn_00b9b400_fills_both_tables_from_the_seed() {
        let mut e = Engine::new();
        e.map(0x011a_d000, 0x1000);
        let table: Vec<u8> = (0..=255u8).collect();
        e.mem.write(TABLE_011AD900, &table);
        let this = e.mem.alloc(0x2bc);
        e.call(0x00b9_b400, &args![this, 0u32]);
        // (0 * k + 0x22fc2945) >> 24 = 0x22.
        assert_eq!(e.mem.u8(this + 0x98), 0x22);
        assert_eq!(e.mem.u8(this + 0x198 + 0xff), 0x21);
        e.call(0x00b9_b400, &args![this, 1u32]);
        let start = 0x5391_313du32.wrapping_add(0x22fc_2945) >> 24;
        assert_eq!(e.mem.u8(this + 0x98), start as u8);
        assert_eq!(
            e.mem.bytes(this + 0x98, 0x100),
            e.mem.bytes(this + 0x198, 0x100)
        );
    }

    #[test]
    fn fn_00b9b450_and_00b9b930_delete_parameters_and_textures() {
        let mut e = rig();
        for callee in [
            ARRAY_SET_AT_00BA3AA0,
            TEXTURE_DESTRUCT_00BA3A00,
            EFFECT_DESTRUCT_00BA3D30,
            MEMORY_DEALLOCATE_00AA4060,
            0x0e76_0000,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.put_vtable(0x0e77_0000, &[0x0e76_0000]);
        let this = e.mem.alloc(0x2bc);
        let list = e.mem.alloc(8);
        let param = e.mem.alloc(8);
        e.mem.set_u32(param, 0x0e77_0000);
        e.mem.set_u32(list + 4, param);
        e.mem.set_u32(this + 0x1c, list);
        assert_eq!(e.call(0x00b9_b930, &args![this, 1u32]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B7E6C);
        assert_eq!(calls(&e, 0x0e76_0000), vec![vec![param, 1]]);
        assert_eq!(calls(&e, ARRAY_SET_AT_00BA3AA0)[0][..2], [this + 0x18, 1]);
        let destroyed: Vec<u32> = calls(&e, TEXTURE_DESTRUCT_00BA3A00)
            .iter()
            .map(|c| c[0] - this)
            .collect();
        assert_eq!(destroyed, vec![0x88, 0x78, 0x68, 0x58]);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }

    #[test]
    fn fn_00b9b950_fills_the_tables_once_and_sets_up_two_parameters() {
        let mut e = rig();
        e.map(0x011a_d000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.map(0x0e78_0000, 0x1000);
        let table: Vec<u8> = (0..=255u8).collect();
        e.mem.write(TABLE_011AD900, &table);
        e.set_global(SEED_011F9410, -1.75f32);
        let mut next = 0x0e78_0000u32;
        e.register_double(PARAM_CONSTRUCT_00B8A9E0, move |_, _| {
            next += 0x100;
            next.into_ret()
        });
        for callee in [
            SIZE_ARRAYS_00BA3F90,
            ARRAY_SET_SIZE_00A90180,
            PARAM_INIT_CONSTANT_MAP_00B8AB10,
            ARRAY_SET_AT_00BA3AA0,
            SET_EFFECT_00BA3C70,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x2bc);
        let manager = e.mem.alloc(0x10);
        let effects = e.mem.alloc(0x1d0);
        e.mem.set_u32(effects + 0x1c8, 0x0e79_0000);
        e.mem.set_u32(effects + 0x1cc, 0x0e79_0001);
        e.mem.set_u32(manager + 8, effects);
        e.call(0x00b9_b950, &args![this, manager, 0u32]);
        // -1.75 chops to -1 (0xffffffff).
        let start = 0xffff_ffffu32
            .wrapping_mul(0x5391_313d)
            .wrapping_add(0x22fc_2945)
            >> 24;
        assert_eq!(e.mem.u8(this + 0x98), start as u8);
        assert_eq!(e.mem.u8(this + 0x2b8), 1);
        assert_eq!(calls(&e, SIZE_ARRAYS_00BA3F90), vec![vec![this, 2]]);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + 0x28, 4]]
        );
        assert_eq!(
            calls(&e, PARAM_INIT_CONSTANT_MAP_00B8AB10),
            vec![vec![0x0e78_0100, 3, 6], vec![0x0e78_0200, 3, 6]]
        );
        assert_eq!(
            calls(&e, SET_EFFECT_00BA3C70),
            vec![
                vec![this, 0, 0x0e79_0000, 0x0e78_0100, 0],
                vec![this, 1, 0x0e79_0001, 0x0e78_0200, 0]
            ]
        );
        // A second set-up keeps the tables.
        e.mem.set_u8(this + 0x98, 0x77);
        e.call(0x00b9_b950, &args![this, manager, 0u32]);
        assert_eq!(e.mem.u8(this + 0x98), 0x77);
    }

    fn fade_rig(current: f32, time: f32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011a_d000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.set_global(DIVISOR_011AD834, 2.0f32);
        e.set_global(FLOOR_01012060, 0.0f64);
        let this = e.mem.alloc(0xe0);
        e.mem.set_f32(this + 0xd8, current);
        e.mem.set_f32(this + 0xdc, time);
        (e, this)
    }

    #[test]
    fn fn_00b9bb10_restarts_or_reverses_the_fade() {
        let (mut e, this) = fade_rig(1.0, 0.5);
        e.call(0x00b9_bb10, &args![this, 1.0f32.to_bits(), 0u32]);
        assert_eq!(e.mem.f32(this + 0xdc), 0.5);
        e.call(0x00b9_bb10, &args![this, 3.0f32.to_bits(), 1u32]);
        assert_eq!(e.mem.f32(this + 0xdc), 1.0);
        assert_eq!(e.mem.f32(this + 0xd8), 3.0);
        // 0.5 / 2 = 0.25: 2 * (1 - 0.25) = 1.5.
        let (mut e, this) = fade_rig(1.0, 0.5);
        e.call(0x00b9_bb10, &args![this, 0x7f80_0001u32, 0u32]);
        assert_eq!(e.mem.f32(this + 0xdc), 1.5);
        assert_eq!(e.mem.u32(this + 0xd8), 0x7fc0_0001);
        // A ratio above 1 counts as 1, one below 0 as 0.
        let (mut e, this) = fade_rig(1.0, 6.0);
        e.call(0x00b9_bb10, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.f32(this + 0xdc), 0.0);
        let (mut e, this) = fade_rig(1.0, -6.0);
        e.call(0x00b9_bb10, &args![this, 0u32, 0u32]);
        assert_eq!(e.mem.f32(this + 0xdc), 2.0);
    }
}

#[cfg(test)]
mod n070_tests {
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

    const ACTIVE_SLOT: u32 = 0x0e60_0000;
    const RENDER_SLOT: u32 = 0x0e60_0004;

    /// The effect of `00b9b830` with two sub-effects (slot +4 `RENDER_SLOT`), two parameters,
    /// two textures, slot +0x1c `ACTIVE_SLOT` answering 0x2a, and the constants `011ffe48` to
    /// `011ffe94` holding 1.0, 2.0, ... (a signalling NaN at `011ffe4c`).
    fn render_rig(active: u8) -> (Engine, u32, [u32; 2]) {
        let mut e = Engine::new();
        e.map(0x011f_f000, 0x1000);
        for i in 0..20u32 {
            e.set_global(0x011f_fe48 + 4 * i, (i + 1) as f32);
        }
        e.set_global(0x011f_fe4c, 0x7f80_0001u32);
        let vtable = e.mem.alloc(0x20);
        e.mem.set_u32(vtable + 0x1c, ACTIVE_SLOT);
        let this = e.mem.alloc(0x2c0);
        e.mem.set_u32(this, vtable);
        e.mem.set_u8(this + 5, active);
        let effect_vtable = e.mem.alloc(8);
        e.mem.set_u32(effect_vtable + 4, RENDER_SLOT);
        let mut effects = [0u32; 2];
        for effect in effects.iter_mut() {
            *effect = e.mem.alloc(0x100) + 0x6c;
            e.mem.set_u32(*effect, effect_vtable);
        }
        let list = e.mem.alloc(8);
        e.mem.set_u32(list, effects[0]);
        e.mem.set_u32(list + 4, effects[1]);
        e.mem.set_u32(this + 0xc, list);
        let params = e.mem.alloc(8);
        e.mem.set_u32(params, 0x700);
        e.mem.set_u32(params + 4, 0x704);
        e.mem.set_u32(this + 0x1c, params);
        let textures = e.mem.alloc(8);
        e.mem.set_u32(textures, 0x800);
        e.mem.set_u32(textures + 4, 0x804);
        e.mem.set_u32(this + 0x2c, textures);
        e.register(ACTIVE_SLOT, |_, _| 0x2au32.into_ret());
        for target in [
            RENDER_SLOT,
            BORROW_RENDERED_TEXTURE_00BA3840,
            RETURN_RENDERED_TEXTURE_00BA39A0,
            SET_PIXEL_CONSTANT_00B8A790,
            SET_TEXTURE_00BA3CB0,
            TEXTURE_00BA3760,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        (e, this, effects)
    }

    #[test]
    fn fn_00b9b540_renders_both_effects() {
        let (mut e, this, effects) = render_rig(1);
        e.call(
            0x00b9_b540,
            &args![this, 0x11u32, 0x22u32, 0x33u32, 0x44u32],
        );
        assert_eq!(calls(&e, ACTIVE_SLOT), vec![vec![this, 0x33]]);
        assert_eq!(e.mem.u8(this + 5), 0x2a);
        let borrowed = this + 0x68;
        assert_eq!(
            calls(&e, BORROW_RENDERED_TEXTURE_00BA3840),
            vec![vec![borrowed, 0x32]]
        );
        let f = |v: f32| v.to_bits();
        let sets = calls(&e, SET_PIXEL_CONSTANT_00B8A790);
        assert_eq!(sets.len(), 10);
        assert_eq!(sets[0], vec![0x700, 0, 0x7fc0_0001, f(3.0), 0, 0]);
        assert_eq!(sets[3], vec![0x700, 3, f(1.0), 0, 0, 0]);
        assert_eq!(sets[4], vec![0x700, 4, f(14.0), f(15.0), f(16.0), 0]);
        assert_eq!(sets[5], vec![0x700, 5, f(18.0), f(19.0), f(20.0), 0]);
        assert_eq!(sets[8], vec![0x704, 2, f(10.0), f(11.0), 0, 0]);
        assert_eq!(
            calls(&e, SET_TEXTURE_00BA3CB0),
            vec![
                vec![effects[0], 0, borrowed, 0],
                vec![effects[0], 1, 0x804, 1],
                vec![effects[1], 0, 0x800, 0],
                vec![effects[1], 1, borrowed, 0],
            ]
        );
        assert_eq!(
            calls(&e, TEXTURE_00BA3760),
            vec![
                vec![0x800, 3],
                vec![0x804, 3],
                vec![0x800, 3],
                vec![0x804, 3]
            ]
        );
        assert_eq!(
            calls(&e, RENDER_SLOT),
            vec![
                vec![effects[0], 0x11, 0x22, 0x700, 1],
                vec![effects[1], 0x11, 0x22, 0x704, 1],
            ]
        );
        assert_eq!(
            calls(&e, RETURN_RENDERED_TEXTURE_00BA39A0),
            vec![vec![borrowed]]
        );
    }

    #[test]
    fn fn_00b9b540_skips_slot_0x1c_when_inactive() {
        let (mut e, this, _) = render_rig(0);
        e.call(0x00b9_b540, &args![this, 0u32, 0u32, 0u32, 0u32]);
        assert!(calls(&e, ACTIVE_SLOT).is_empty());
        assert_eq!(e.mem.u8(this + 5), 0);
    }

    #[test]
    fn fn_00b9b540_a_0x6c_effect_pointer_is_null() {
        let (mut e, this, _) = render_rig(0);
        let list = e.mem.u32(this + 0xc);
        e.mem.set_u32(list, 0x6c);
        // The null effect's vtable is then read at 0: map it with a render slot.
        e.map(0, 0x1000);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, RENDER_SLOT);
        e.mem.set_u32(0, vtable);
        e.call(0x00b9_b540, &args![this, 0u32, 0u32, 0u32, 0u32]);
        assert_eq!(calls(&e, SET_TEXTURE_00BA3CB0)[0][0], 0);
        assert_eq!(calls(&e, RENDER_SLOT)[0][0], 0);
    }

    fn seed_rig(seed: f32) -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.map(0x011a_d000, 0x1000);
        for i in 0..0x100u32 {
            e.mem.set_u8(TABLE_011AD900 + i, i as u8);
        }
        e.set_global(SEED_011F9410, seed);
        let this = e.mem.alloc(0x2c0);
        (e, this)
    }

    #[test]
    fn fn_00b9b7f0_fills_the_tables_from_the_truncated_seed() {
        // Seed 2.75 truncates to 2: start (2 * 0x5391313d + 0x22fc2945) >> 24.
        let (mut e, this) = seed_rig(2.75);
        e.call(0x00b9_b7f0, &args![this]);
        let start = 2u32.wrapping_mul(0x5391_313d).wrapping_add(0x22fc_2945) >> 24;
        assert_eq!(u32::from(e.mem.u8(this + 0x98)), start);
        assert_eq!(u32::from(e.mem.u8(this + 0x198)), start);
        assert_eq!(e.mem.u8(this + 0x2b8), 1);
        // Even when the byte was set already.
        e.mem.set_u8(this + 0x98, 0x77);
        e.call(0x00b9_b7f0, &args![this]);
        assert_eq!(u32::from(e.mem.u8(this + 0x98)), start);
    }

    #[test]
    fn fn_00b9bac0_fills_only_the_first_time() {
        let (mut e, this) = seed_rig(-1.5);
        assert!(e.call(0x00b9_bac0, &args![this, 0u32]).bool());
        let start = 1u32
            .wrapping_neg()
            .wrapping_mul(0x5391_313d)
            .wrapping_add(0x22fc_2945)
            >> 24;
        assert_eq!(u32::from(e.mem.u8(this + 0x98)), start);
        assert_eq!(e.mem.u8(this + 0x2b8), 1);
        e.mem.set_u8(this + 0x98, 0x77);
        assert!(e.call(0x00b9_bac0, &args![this, 0u32]).bool());
        assert_eq!(e.mem.u8(this + 0x98), 0x77);
    }
}
