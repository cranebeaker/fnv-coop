//! `BSShader/run_00ba3e30` (Xbox PDB source unit), subsystem `BSShader`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::bsshader::geometrydecalshader::shader_inline::*;

/// The shader's vtable (`010b80a8`; no RTTI locator before it).
const VTABLE_010B80A8: u32 = 0x010b_80a8;
/// The shared element count (`012003ec`), set on first use from the shader model setting
/// (`011f91c0`): 0x1a below model 2, else 0x4e.
const ELEMENT_COUNT_012003EC: u32 = 0x0120_03ec;
const SHADER_MODEL_011F91C0: u32 = 0x011f_91c0;

// Translated from 00ba53f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): the constructor the creator `00ba58b0` runs,
/// `thiscall` with one stack word (the declaration). Runs `BSShader::BSShader` (`00be2200`),
/// stores the vtable `010b80a8`, clears +0x74 to +0x80, stores 0.0 at +0xec to +0x128, sets
/// +0x68 to 0xe, the byte +0x24 to 1 and +0x6c to 0x241; clears the pass pointers at +0x74 and
/// +0x78 (their counts at +0x64 dropped, `00e80190` at 0), assigns the declaration to the
/// `NiPointer` at +0x28, clears the `NiPointer`s at +0x7c and +0x80, stores 0.0 at +0xd4 to
/// +0x128; the element count at `012003ec` becomes 0x1a (shader model below 2, signed) or
/// 0x4e when it is 0; that many 0x30-byte elements (`operator new[]` size check) from the
/// memory manager go to +0x70 and are cleared, and 0x50 bytes at +0x84 are cleared
/// (`memset`). Answers `this`.
// Translation incomplete: 00ba53f0 - the C++ exception frame (FS:[0] chain, unwind state 4 for the base and the pointers) is not translated.
pub fn fn_00ba53f0(e: &mut Engine, this: Ptr, declaration: Ptr) -> Ptr {
    let base = this.addr();
    e.call(BS_SHADER_CONSTRUCT, &args![base]);
    e.mem.set_u32(base, VTABLE_010B80A8);
    for offset in [0x74u32, 0x78, 0x7c, 0x80] {
        e.mem.set_u32(base + offset, 0);
    }
    for offset in (0xecu32..=0x128).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    e.mem.set_u32(base + 0x68, 0xe);
    e.mem.set_u8(base + 0x24, 1);
    e.mem.set_u32(base + 0x6c, 0x241);
    pass_pointer_clear(e, base + 0x74);
    pass_pointer_clear(e, base + 0x78);
    ni_pointer_assign(e, base + 0x28, declaration.addr());
    ni_pointer_clear(e, base + 0x7c);
    ni_pointer_clear(e, base + 0x80);
    for offset in (0xd4u32..=0x128).step_by(4) {
        e.mem.set_f32(base + offset, 0.0);
    }
    let mut count = e.global::<u32>(ELEMENT_COUNT_012003EC);
    if count == 0 {
        count = if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
            0x1a
        } else {
            0x4e
        };
        e.set_global::<u32>(ELEMENT_COUNT_012003EC, count);
    }
    let buffer = allocate_array(e, count, 0x30);
    e.mem.set_u32(base + 0x70, buffer);
    e.call(MEMSET, &args![buffer, 0u32, count.wrapping_mul(0x30)]);
    e.call(MEMSET, &args![base + 0x84, 0u32, 0x50u32]);
    this
}

/// The image-space effect base constructor `00ba4080` (unnamed; `thiscall`, no stack words;
/// it stores the vtable `010b7fe4` and builds the effect's arrays) and the `NiTArray` set-size
/// `00a90180` (`thiscall` on the array at +0x28, one word).
const EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
/// The element constructor `004ebb70` (unit `teswater.cpp`, unnamed) and
/// `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`) the texture array passes to the
/// `eh vector constructor iterator`.
const TEXTURE_CONSTRUCT_004EBB70: u32 = 0x004e_bb70;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;
/// The two effects' vtables (`010b8008`, `010b8030`; no RTTI locator before them).
const VTABLE_010B8008: u32 = 0x010b_8008;
const VTABLE_010B8030: u32 = 0x010b_8030;
/// The three words `00ba4880` copies (`011a9490` to `011a9498`, to `012003d4` to `012003dc`).
const WORDS_011A9490: u32 = 0x011a_9490;
const WORDS_012003D4: u32 = 0x0120_03d4;

// Translated from 00ba4640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): the constructor of the 0x60-byte effects
/// `00b94520` makes in pairs, `thiscall`, no stack words. Runs the base constructor
/// `00ba4080`, stores the vtable `010b8008`, sizes the array at +0x28 to 4 (`00a90180`) and
/// sets +0x58 to -1. Answers `this`.
// Translation incomplete: 00ba4640 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00ba4640(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B8008);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 4u32]);
    e.mem.set_u32(base + 0x58, 0xffff_ffff);
    this
}

// Translated from 00ba4880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): an image-space effect's constructor, `thiscall`,
/// no stack words. Runs the base constructor `00ba4080`, stores the vtable `010b8030`,
/// constructs the three 16-byte textures at +0x58 (`eh vector constructor iterator` with
/// `004ebb70` and `ImageSpaceTexture::~ImageSpaceTexture`), sizes the array at +0x28 to 4
/// (`00a90180`) and copies the three words at `011a9490` to `012003d4` (stored last word
/// first). Answers `this`.
// Translation incomplete: 00ba4880 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and the textures) is not translated.
pub fn fn_00ba4880(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010B8030);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base + 0x58,
            0x10u32,
            3u32,
            TEXTURE_CONSTRUCT_004EBB70,
            TEXTURE_DESTRUCT_00BA3A00
        ],
    );
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 4u32]);
    let third = e.global::<u32>(WORDS_011A9490 + 8);
    let first = e.global::<u32>(WORDS_011A9490);
    let second = e.global::<u32>(WORDS_011A9490 + 4);
    e.set_global(WORDS_012003D4 + 8, third);
    e.set_global(WORDS_012003D4, first);
    e.set_global(WORDS_012003D4 + 4, second);
    this
}

/// The image-space effect's vtable (`010b7fe4`) and the vtables of its five arrays
/// (`010b8004` for the two at +8 and +0x18, `010b9d24` for the three at +0x28, +0x38 and
/// +0x48; no RTTI locators before them).
const EFFECT_VTABLE_010B7FE4: u32 = 0x010b_7fe4;
const ARRAY_VTABLE_010B8004: u32 = 0x010b_8004;
const ARRAY_VTABLE_010B9D24: u32 = 0x010b_9d24;

// Translated from 00ba4080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): the image-space effect base constructor every
/// image-space effect's constructor starts with, `thiscall`, no stack words. Stores the vtable
/// `010b7fe4`, sets the byte +5 and clears the byte +4, then makes five empty `NiTArray`s at
/// +8, +0x18, +0x28, +0x38 and +0x48 (each: max size 0, grow-by 1, size and element count 0,
/// no elements, its vtable). Answers `this`.
// Translation incomplete: 00ba4080 - the C++ exception frame (FS:[0] chain set up and removed, no unwind state used) is not translated.
pub fn fn_00ba4080(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, EFFECT_VTABLE_010B7FE4);
    e.mem.set_u8(base + 5, 1);
    e.mem.set_u8(base + 4, 0);
    for (array, vtable) in [
        (8u32, ARRAY_VTABLE_010B8004),
        (0x18, ARRAY_VTABLE_010B8004),
        (0x28, ARRAY_VTABLE_010B9D24),
        (0x38, ARRAY_VTABLE_010B9D24),
        (0x48, ARRAY_VTABLE_010B9D24),
    ] {
        let array = base + array;
        e.mem.set_u16(array + 8, 0);
        e.mem.set_u16(array + 0xe, 1);
        e.mem.set_u16(array + 0xa, 0);
        e.mem.set_u16(array + 0xc, 0);
        e.mem.set_u32(array + 4, 0);
        e.mem.set_u32(array, vtable);
    }
    this
}

/// `ImageSpaceEffect` fields (Xbox PDB, offsets as the PC code uses them): `pEffectList`
/// (+8; elements +0xc, size +0x12), `pEffectParamList` (+0x18; elements +0x1c),
/// `pTextures` (+0x28; elements +0x2c), `pEffectInputs` (+0x38; elements +0x3c) and
/// `pEffectOutput` (+0x48; elements +0x4c), each an `NiTPrimitiveArray` (0x10 bytes).
const EFFECT_LIST: u32 = 0x8;
const EFFECT_PARAM_LIST: u32 = 0x18;
const EFFECT_TEXTURES: u32 = 0x28;
/// The `NiTArray` element store `00ba3aa0` (`thiscall` on the array: index, address of the
/// value) and the texture's `00d47a40` (unnamed, `thiscall`, one word) the input step gives
/// each texture with its input's second word.
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const TEXTURE_SET_00D47A40: u32 = 0x00d4_7a40;

/// The element store with the address of a stack word holding `value`.
fn set_at(e: &mut Engine, array: u32, index: u32, value: u32) {
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), value);
        e.call(ARRAY_SET_AT_00BA3AA0, &args![array, index, word]);
    });
}

/// Element `index` of the `NiTPrimitiveArray` whose element pointer is at `elements`.
fn element(e: &Engine, elements: u32, index: u32) -> u32 {
    let base = e.mem.u32(elements);
    e.mem.u32(base.wrapping_add(index.wrapping_mul(4)))
}

// Translated from 00ba3e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): an image-space effect's input step for its
/// sub-effect `index`, `thiscall` with one stack word. The sub-effect (`pEffectList[index]`)
/// gets for each of its texture slots from 1 below its texture count (+0x30, signed) the
/// texture its input pair (`pEffectInputs[index]`, pairs of words from the second) names, when
/// the pair's first word is not negative (`00ba3aa0` on the sub-effect's +0x28; a non-null
/// texture then gets `00d47a40` with the pair's second word). Slot 0 is stored in place as
/// `SetAt` does it (the size +0x32 raised to 1 when 0, the element count +0x34 following
/// null and non-null) with the texture `pEffectOutput[index]` names, and a non-null one gets
/// `00d47a40(0)`.
pub fn fn_00ba3e30(e: &mut Engine, this: Ptr, index: u32) {
    let base = this.addr();
    let sub = element(e, base + 0xc, index);
    let count = u32::from(e.mem.u16(sub + 0x30));
    let mut pair = element(e, base + 0x3c, index);
    let output = element(e, base + 0x4c, index);
    let mut slot = 1u32;
    if 1 < count as i32 {
        loop {
            let texture_index = e.mem.u32(pair);
            if (texture_index as i32) >= 0 {
                let texture = element(e, base + 0x2c, texture_index);
                let second = e.mem.u32(pair + 4);
                set_at(e, sub + 0x28, slot, texture);
                if texture != 0 {
                    e.call(TEXTURE_SET_00D47A40, &args![texture, second]);
                }
            }
            slot += 1;
            pair = pair.wrapping_add(8);
            if (slot as i32) >= count as i32 {
                break;
            }
        }
    }
    let texture = element(e, base + 0x2c, output);
    if e.mem.u16(sub + 0x32) == 0 {
        e.mem.set_u16(sub + 0x32, 1);
        if texture != 0 {
            let used = e.mem.u16(sub + 0x34).wrapping_add(1);
            e.mem.set_u16(sub + 0x34, used);
        }
    } else if texture != 0 {
        let first = e.mem.u32(sub + 0x2c);
        if e.mem.u32(first) == 0 {
            let used = e.mem.u16(sub + 0x34).wrapping_add(1);
            e.mem.set_u16(sub + 0x34, used);
        }
    } else {
        let first = e.mem.u32(sub + 0x2c);
        if e.mem.u32(first) != 0 {
            let used = e.mem.u16(sub + 0x34).wrapping_add(0xffff);
            e.mem.set_u16(sub + 0x34, used);
        }
    }
    let first = e.mem.u32(sub + 0x2c);
    e.mem.set_u32(first, texture);
    if texture != 0 {
        e.call(TEXTURE_SET_00D47A40, &args![texture, 0u32]);
    }
}

/// One sub-effect rendered as the effects' `Render` loops do it: the input step, the effect's
/// `UpdateParams` (vtable +0x1c) with the third word, then the sub-effect's `Render` (its
/// slot +4) with the first two words, the effect's parameter `index` and the fourth word.
fn render_sub_effect(e: &mut Engine, base: u32, index: u32, words: [u32; 4]) {
    fn_00ba3e30(e, Ptr::new(base), index);
    e.vcall(base, 0x1c, &args![words[2]]);
    let sub = element(e, base + 0xc, index);
    let param = element(e, base + 0x1c, index);
    e.vcall(sub, 4, &args![words[0], words[1], param, words[3]]);
}

// Translated from 00ba3f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): the image-space effect's `Render` (vtable +4 of
/// the base and of the effect `00ba4640` builds), `thiscall` with four stack words.
/// `BorrowTextures` (+0x10) with the third word, then every sub-effect of `pEffectList`
/// rendered (input step, `UpdateParams` +0x1c with the third word, the sub-effect's `Render`
/// with the first two words, its parameter and the fourth word), then `ReturnTextures` (+0x14).
pub fn fn_00ba3f20(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32, fourth: u32) {
    let base = this.addr();
    e.vcall(base, 0x10, &args![third]);
    let mut index = 0u32;
    if e.mem.u16(base + 0x12) != 0 {
        loop {
            render_sub_effect(e, base, index, [first, second, third, fourth]);
            index += 1;
            if index >= u32::from(e.mem.u16(base + 0x12)) {
                break;
            }
        }
    }
    e.vcall(base, 0x14, &args![]);
}

// Translated from 00ba3fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): another effect class's `Render` (two vtables
/// hold it), `thiscall` with four stack words: as `00ba3f20`, but only the sub-effects whose
/// byte in the array at +0x5c is set are rendered.
pub fn fn_00ba3fd0(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32, fourth: u32) {
    let base = this.addr();
    e.vcall(base, 0x10, &args![third]);
    let mut index = 0u32;
    if e.mem.u16(base + 0x12) != 0 {
        loop {
            let flags = e.mem.u32(base + 0x5c);
            if e.mem.u8(index.wrapping_add(flags)) != 0 {
                render_sub_effect(e, base, index, [first, second, third, fourth]);
            }
            index += 1;
            if index >= u32::from(e.mem.u16(base + 0x12)) {
                break;
            }
        }
    }
    e.vcall(base, 0x14, &args![]);
}

// Translated from 00ba3f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): sizes an image-space effect's arrays,
/// `thiscall` with one stack word: `00a90180` with it on `pEffectList` (+8),
/// `pEffectParamList` (+0x18), `pEffectInputs` (+0x38) and `pEffectOutput` (+0x48); the
/// textures (+0x28) are left.
pub fn fn_00ba3f90(e: &mut Engine, this: Ptr, size: u32) {
    let base = this.addr();
    for array in [EFFECT_LIST, EFFECT_PARAM_LIST, 0x38, 0x48] {
        e.call(ARRAY_SET_SIZE_00A90180, &args![base + array, size]);
    }
}

/// `ImageSpaceEffect::~ImageSpaceEffect` (engine map, `00ba3d30`), the memory manager's free
/// (`00aa4060`, `thiscall` on `011f6238`, one word) and the `eh vector destructor iterator`
/// (`00ec5fce`, `stdcall`: array, element size, count, destructor).
const EFFECT_DESTRUCT_00BA3D30: u32 = 0x00ba_3d30;
const MEMORY_FREE: u32 = 0x00aa_4060;
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;

/// Parameters `0..count` of `pEffectParamList` deleted (their vtable slot 0 with 1) and each
/// slot cleared through `00ba3aa0`, as the effects' destructors do.
fn delete_params(e: &mut Engine, base: u32, count: u32) {
    for index in 0..count {
        let elements = e.mem.u32(base + 0x1c);
        let slot = elements.wrapping_add(index * 4);
        if e.mem.u32(slot) != 0 {
            let param = e.mem.u32(slot);
            if param != 0 {
                e.vcall(param, 0, &args![1u32]);
            }
            set_at(e, base + EFFECT_PARAM_LIST, index, 0);
        }
    }
}

// Translated from 00ba4190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): the destructor of the effect `00ba4640` builds,
/// `thiscall`, no stack words. Its vtable `010b8008`; parameters 0 and 1 deleted and cleared;
/// then `ImageSpaceEffect::~ImageSpaceEffect`.
// Translation incomplete: 00ba4190 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00ba4190(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010B8008);
    delete_params(e, base, 2);
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

// Translated from 00ba46a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): that effect's scalar deleting destructor,
/// `thiscall` with one stack word (flags): the destructor `00ba4190`, then with bit 0 the
/// memory manager's free. Answers `this`.
pub fn fn_00ba46a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00ba4190(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00ba4480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): the destructor of the effect `00ba4880` builds,
/// `thiscall`, no stack words. Its vtable `010b8030`; parameters 0 to 10 deleted and cleared;
/// the three textures at +0x58 destroyed (`ImageSpaceTexture::~ImageSpaceTexture`); then
/// `ImageSpaceEffect::~ImageSpaceEffect`.
// Translation incomplete: 00ba4480 - the C++ exception frame (FS:[0] chain, unwind states 1 and 0 for the textures and the base) is not translated.
pub fn fn_00ba4480(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010B8030);
    delete_params(e, base, 0xb);
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x58, 0x10u32, 3u32, TEXTURE_DESTRUCT_00BA3A00],
    );
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

// Translated from 00ba4920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): that effect's scalar deleting destructor,
/// `thiscall` with one stack word (flags): the destructor `00ba4480`, then with bit 0 the
/// memory manager's free. Answers `this`.
pub fn fn_00ba4920(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00ba4480(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00ba4250 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (`run_00ba3e30`), vtable +0x20 of the effect `00ba4640`
/// builds: `thiscall` with one stack word, stored at +0x58.
pub fn fn_00ba4250(e: &mut Engine, this: Ptr, value: u32) {
    e.mem.set_u32(this.addr() + 0x58, value);
}

// Translated from 00ba4260 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (`run_00ba3e30`), vtable +0x24 of that effect: `thiscall`
/// with one stack word whose low byte is stored at +0x5c.
pub fn fn_00ba4260(e: &mut Engine, this: Ptr, value: u8) {
    e.mem.set_u8(this.addr() + 0x5c, value);
}

/// `ImageSpaceTexture::GetTexture` (`00ba3780`), `::BorrowRenderedTexture` (`00ba3840`, one
/// word) and `::ReturnRenderedTexture` (`00ba39a0`) (engine map; `thiscall` on the texture).
const GET_TEXTURE: u32 = 0x00ba_3780;
const BORROW_RENDERED_TEXTURE: u32 = 0x00ba_3840;
const RETURN_RENDERED_TEXTURE: u32 = 0x00ba_39a0;
/// `ImageSpaceShaderParam::SetPixelConstant` (`00b8a790`, `thiscall`: register, four floats),
/// the unnamed `00b8a8b0` (`thiscall`, three words: 1, an address, a byte count) and
/// `00b8a7c0` (`thiscall`, none), `ImageSpaceShaderParam::ImageSpaceShaderParam` (`00b8a9e0`)
/// and `::InitConstantMap` (`00b8ab10`, two words) (engine map names).
const SET_PIXEL_CONSTANT: u32 = 0x00b8_a790;
const SHADER_PARAM_00B8A8B0: u32 = 0x00b8_a8b0;
const SHADER_PARAM_00B8A7C0: u32 = 0x00b8_a7c0;
const SHADER_PARAM_CONSTRUCT: u32 = 0x00b8_a9e0;
const INIT_CONSTANT_MAP: u32 = 0x00b8_ab10;
/// `4294967296.0` (`float`): added to a negative `FILD` of an unsigned size.
const TWO_TO_32_01099378: u32 = 0x0109_9378;
/// The effect level the blur effects last used (`012003c4`) and the data `00ba4270` reads
/// and writes: two constants (`012003c8`, `011adbd4`), the 15 lerped floats at `011adbe0` (every
/// fourth of a 0x10-byte stride), the eight rows of 0xf0 bytes they are lerped between
/// (`011ade30` + 0xf0 per row), and the table `00b8a8b0` gets (`011adbd8` + 0x10 per level).
const EFFECT_LEVEL_012003C4: u32 = 0x0120_03c4;
const CONSTANT_012003C8: u32 = 0x0120_03c8;
const CONSTANT_011ADBD4: u32 = 0x011a_dbd4;
const LERPED_011ADBE0: u32 = 0x011a_dbe0;
const ROWS_011ADE30: u32 = 0x011a_de30;
const LEVEL_TABLE_011ADBD8: u32 = 0x011a_dbd8;

/// A size word as the effects convert it: `FILD`, plus 2^32 (`01099378`) when negative.
fn unsigned_size(e: &Engine, size: u32) -> f64 {
    let value = f64::from(size as i32);
    if (size as i32) < 0 {
        x87_add(value, x87_m32(e.mem.u32(TWO_TO_32_01099378)))
    } else {
        value
    }
}

// Translated from 00ba4270 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (`run_00ba3e30`), `UpdateParams` (vtable +0x1c) of the
/// effect `00ba4640` builds: `thiscall` with one stack word it does not read. With the level n
/// at +0x58: the third texture's width (its texture's vtable +0x94) and the second's height
/// (+0x98) as floats; the factor 1 - (n - `012003c4`), 1.0 when that is 0; the 15 floats at
/// `011adbe0` lerped by it between row max(n - 1, 1) and row min(n, 7) of the table at
/// `011ade30`. Parameter 0 then gets pixel constant 0 (0, 1 / height, 0, 0), with the byte +0x5c
/// constant 0x11 (`012003c8`, `011adbd4`, 0, 0), and `00b8a8b0(1, 011adbd8 + 0x10 (7 -
/// min(n, 7)), min(n, 7) 0x20 + 0x10)`; parameter 1 constant 0 (1 / width, 0, 0, 0) and the same
/// `00b8a8b0`. Every x87 step rounds as the exe's does. Answers 0 in the low byte, the last
/// call's upper bytes above it.
pub fn fn_00ba4270(e: &mut Engine, this: Ptr, _unused_0: u32) -> u32 {
    let base = this.addr();
    let textures = e.mem.u32(base + 0x2c);
    let holder = e.mem.u32(textures + 8);
    let texture = e.call(GET_TEXTURE, &args![holder]).u32();
    let width = e.vcall(texture, 0x94, &args![]).u32();
    let width = x87_store32(unsigned_size(e, width));
    let textures = e.mem.u32(base + 0x2c);
    let holder = e.mem.u32(textures + 4);
    let texture = e.call(GET_TEXTURE, &args![holder]).u32();
    let height = e.vcall(texture, 0x98, &args![]).u32();
    let height = x87_store32(unsigned_size(e, height));
    let level = e.mem.u32(base + 0x58) as i32;
    let high = level.min(7);
    let low = level.wrapping_sub(1).max(1);
    let shifted = x87_sub(
        f64::from(level),
        x87_m32(e.global::<u32>(EFFECT_LEVEL_012003C4)),
    );
    let stored = x87_store32(x87_sub(1.0, shifted));
    let mut factor = x87_load32(stored);
    if factor == 0.0 {
        factor = x87_load32(1.0f32.to_bits());
    }
    let high_row = (high as u32).wrapping_mul(0xf0).wrapping_add(ROWS_011ADE30);
    let low_row = (low as u32).wrapping_mul(0xf0).wrapping_add(ROWS_011ADE30);
    for index in 0..15u32 {
        let offset = index * 0x10;
        let from = e.mem.u32(low_row.wrapping_add(offset));
        let to = e.mem.u32(high_row.wrapping_add(offset));
        let value = x87_sub(x87_load32(to), x87_m32(from));
        let value = x87_mul(value, factor);
        let value = x87_add(value, x87_m32(from));
        e.mem.set_u32(LERPED_011ADBE0 + offset, x87_store32(value));
    }
    let params = e.mem.u32(base + 0x1c);
    let first = e.mem.u32(params);
    let inverse_height = x87_store32(x87_div(1.0, x87_load32(height)));
    e.call(
        SET_PIXEL_CONSTANT,
        &args![
            first,
            0u32,
            0u32,
            x87_quiet_bits(inverse_height),
            0u32,
            0u32
        ],
    );
    if e.mem.u8(base + 0x5c) != 0 {
        let x = x87_quiet_bits(e.global::<u32>(CONSTANT_012003C8));
        let y = x87_quiet_bits(e.global::<u32>(CONSTANT_011ADBD4));
        e.call(SET_PIXEL_CONSTANT, &args![first, 0x11u32, x, y, 0u32, 0u32]);
    }
    let high = high as u32;
    let table = LEVEL_TABLE_011ADBD8.wrapping_add(7u32.wrapping_sub(high).wrapping_mul(0x10));
    let bytes = (high << 5).wrapping_add(0x10);
    e.call(SHADER_PARAM_00B8A8B0, &args![first, 1u32, table, bytes]);
    let params = e.mem.u32(base + 0x1c);
    let second = e.mem.u32(params + 4);
    let inverse_width = x87_store32(x87_div(1.0, x87_load32(width)));
    e.call(
        SET_PIXEL_CONSTANT,
        &args![
            second,
            0u32,
            x87_quiet_bits(inverse_width),
            0u32,
            0u32,
            0u32
        ],
    );
    let answer = e
        .call(SHADER_PARAM_00B8A8B0, &args![second, 1u32, table, bytes])
        .u32();
    answer & 0xffff_ff00
}

// Translated from 00ba4530 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (`run_00ba3e30`), `BorrowTextures` (vtable +0x10) of the
/// effect `00ba4880` builds: `thiscall` with one stack word it does not read. Its textures at
/// +0x58 and +0x68 each borrow a rendered texture (`BorrowRenderedTexture(0x18)`) and become
/// texture 2 and 3 of `pTextures`.
pub fn fn_00ba4530(e: &mut Engine, this: Ptr, _unused_0: u32) {
    let base = this.addr();
    for (texture, index) in [(base + 0x58, 2u32), (base + 0x68, 3)] {
        e.call(BORROW_RENDERED_TEXTURE, &args![texture, 0x18u32]);
        set_at(e, base + EFFECT_TEXTURES, index, texture);
    }
}

// Translated from 00ba4580 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (`run_00ba3e30`), `ReturnTextures` (vtable +0x14) of that
/// effect and two others: `thiscall`, no stack words. Textures 2 and 3 of `pTextures` return
/// their rendered textures (`ReturnRenderedTexture`) and are cleared.
pub fn fn_00ba4580(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for index in [2u32, 3] {
        let texture = element(e, base + 0x2c, index);
        e.call(RETURN_RENDERED_TEXTURE, &args![texture]);
        set_at(e, base + EFFECT_TEXTURES, index, 0);
    }
}

/// The flag bytes the blur effect checks (`012003cc`, `011f94a9`) and its strength (`012003d0`).
const BLUR_FLAG_012003CC: u32 = 0x0120_03cc;
const BLUR_FLAG_011F94A9: u32 = 0x011f_94a9;
const BLUR_STRENGTH_012003D0: u32 = 0x0120_03d0;

/// Whether the three floats at `012003d4` differ from those at `011a9490` (the first that is
/// not equal, NaN included, decides; as `FUCOMPP` and the parity test do it).
fn blur_colour_changed(e: &Engine) -> bool {
    for index in 0..3 {
        let current = e.mem.f32(WORDS_012003D4 + 4 * index);
        let reference = e.mem.f32(WORDS_011A9490 + 4 * index);
        if reference != current {
            return true;
        }
    }
    false
}

// Translated from 00ba45d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (`run_00ba3e30`), `IsActive` (vtable +0x18) of the effect
/// `00ba4880` builds, `thiscall` (not read), no stack words: 1 when the floats at `012003d4`
/// differ from `011a9490` and both flag bytes (`012003cc`, `011f94a9`) are set, or when the
/// strength (`012003d0`) is above 0 (NaN is not); else 0.
pub fn fn_00ba45d0(e: &mut Engine, _unused_0: Ptr) -> u32 {
    if blur_colour_changed(e)
        && e.global::<u8>(BLUR_FLAG_012003CC) != 0
        && e.global::<u8>(BLUR_FLAG_011F94A9) != 0
    {
        return 1;
    }
    let strength = e.global::<f32>(BLUR_STRENGTH_012003D0);
    u32::from(0.0f32 < strength)
}

/// CRT `ceil` (`00ec9e10`, `cdecl`: a `double`, the result in ST0), `_ftol2` (`00ec62c0`, the
/// value in ST0, passed as an `f64`), the render-state setters `00b97fa0` and `00b97ff0`
/// (`bsrenderstate_xenon.cpp`, `cdecl`) and the counter the blur's last pass lowers
/// (`011ff9f0`).
const CEIL: u32 = 0x00ec_9e10;
const FTOL: u32 = 0x00ec_62c0;
const RENDER_STATE_00B97FA0: u32 = 0x00b9_7fa0;
const RENDER_STATE_00B97FF0: u32 = 0x00b9_7ff0;
const PASS_COUNTER_011FF9F0: u32 = 0x011f_f9f0;

/// Whether the blur strength (`012003d0`) is below 1.0 (`FLD1`, `FCOMP`; NaN is not).
fn strength_below_one(e: &Engine) -> bool {
    1.0f32 > e.global::<f32>(BLUR_STRENGTH_012003D0)
}

// Translated from 00ba4d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (`run_00ba3e30`), `Render` (vtable +4) of the effect
/// `00ba4880` builds: `thiscall` with four stack words. `BorrowTextures` (+0x10) with the third
/// word; the pass count is `ceil` of the strength (`012003d0`) truncated (`_ftol2`), at least 2
/// when the floats at `012003d4` differ from `011a9490` and both flags are set. Sub-effects 0
/// and the one at that count (of 1 to 7) are rendered as `Render` renders them. With the
/// strength below 1.0 sub-effect 10 follows, and if it still is, the render states
/// `00b97fa0(1, 1)` and `00b97ff0(6, 7, 0)` and sub-effect 9; else sub-effect 8. Still below
/// 1.0 at the end: the counter `011ff9f0` lowered and `00b97fa0(0, 0)`. Then `ReturnTextures`
/// (+0x14).
pub fn fn_00ba4d20(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32, fourth: u32) {
    let base = this.addr();
    let words = [first, second, third, fourth];
    e.vcall(base, 0x10, &args![third]);
    let strength = x87_load32(e.global::<u32>(BLUR_STRENGTH_012003D0));
    let rounded = e.call(CEIL, &args![strength]).f64();
    let rounded = x87_store32(rounded);
    let mut passes = e.call(FTOL, &args![x87_load32(rounded)]).i32();
    if blur_colour_changed(e)
        && e.global::<u8>(BLUR_FLAG_012003CC) != 0
        && e.global::<u8>(BLUR_FLAG_011F94A9) != 0
        && passes < 2
    {
        passes = 2;
    }
    for index in 0..8i32 {
        if index < 1 || index == passes {
            render_sub_effect(e, base, index as u32, words);
        }
    }
    let mut last = 8u32;
    if strength_below_one(e) {
        render_sub_effect(e, base, 0xa, words);
        if strength_below_one(e) {
            e.call(RENDER_STATE_00B97FA0, &args![1u32, 1u32]);
            e.call(RENDER_STATE_00B97FF0, &args![6u32, 7u32, 0u32]);
            last = 9;
        }
    }
    render_sub_effect(e, base, last, words);
    if strength_below_one(e) {
        let counter = e.global::<u32>(PASS_COUNTER_011FF9F0).wrapping_sub(1);
        e.set_global(PASS_COUNTER_011FF9F0, counter);
        e.call(RENDER_STATE_00B97FA0, &args![0u32, 0u32]);
    }
    e.vcall(base, 0x14, &args![]);
}

/// The renderer (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;

// Translated from 00ba4ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`), `UpdateParams` (vtable +0x1c) of the effect
/// `00ba4880` builds: `thiscall` with one stack word it does not read. The width and height:
/// of the first texture's texture (its vtable +0x94 and +0x98) when `pTextures[0]` and its
/// `GetTexture` are set, else of the renderer's object (`011f9508` vtable +0xc8, then +0x8c and
/// +0x90 with 0); as floats. Parameters 0 and 8 get pixel constant 2 (1 / width, 1 / height, 0,
/// 0), parameter 9 the same with the strength (`012003d0`) third; the strength is kept at
/// `012003c4`. Answers 0 in the low byte, the last call's upper bytes above it.
pub fn fn_00ba4ee0(e: &mut Engine, this: Ptr, _unused_0: u32) -> u32 {
    let base = this.addr();
    let textures = e.mem.u32(base + 0x2c);
    let mut texture = 0;
    if e.mem.u32(textures) != 0 {
        let holder = e.mem.u32(textures);
        texture = e.call(GET_TEXTURE, &args![holder]).u32();
    }
    let (width, height) = if texture != 0 {
        let textures = e.mem.u32(base + 0x2c);
        let holder = e.mem.u32(textures);
        let texture = e.call(GET_TEXTURE, &args![holder]).u32();
        let width = e.vcall(texture, 0x94, &args![]).u32();
        let width = x87_store32(unsigned_size(e, width));
        let textures = e.mem.u32(base + 0x2c);
        let holder = e.mem.u32(textures);
        let texture = e.call(GET_TEXTURE, &args![holder]).u32();
        let height = e.vcall(texture, 0x98, &args![]).u32();
        (width, unsigned_size(e, height))
    } else {
        let renderer = e.global::<u32>(RENDERER_011F9508);
        let object = e.vcall(renderer, 0xc8, &args![]).u32();
        let width = e.vcall(object, 0x8c, &args![0u32]).u32();
        let width = x87_store32(unsigned_size(e, width));
        let renderer = e.global::<u32>(RENDERER_011F9508);
        let object = e.vcall(renderer, 0xc8, &args![]).u32();
        let height = e.vcall(object, 0x90, &args![0u32]).u32();
        (width, unsigned_size(e, height))
    };
    let height = x87_store32(height);
    let inverse_height = x87_store32(x87_div(1.0, x87_load32(height)));
    let inverse_width = x87_store32(x87_div(1.0, x87_m32(width)));
    let params = e.mem.u32(base + 0x1c);
    let param = e.mem.u32(params);
    let (x, y) = (
        x87_quiet_bits(inverse_width),
        x87_quiet_bits(inverse_height),
    );
    e.call(SET_PIXEL_CONSTANT, &args![param, 2u32, x, y, 0u32, 0u32]);
    let params = e.mem.u32(base + 0x1c);
    let param = e.mem.u32(params + 0x20);
    e.call(SET_PIXEL_CONSTANT, &args![param, 2u32, x, y, 0u32, 0u32]);
    let params = e.mem.u32(base + 0x1c);
    let param = e.mem.u32(params + 0x24);
    let strength = x87_quiet_bits(e.global::<u32>(BLUR_STRENGTH_012003D0));
    let answer = e
        .call(
            SET_PIXEL_CONSTANT,
            &args![param, 2u32, x, y, strength, 0u32],
        )
        .u32();
    let strength = x87_quiet_bits(e.global::<u32>(BLUR_STRENGTH_012003D0));
    e.set_global(EFFECT_LEVEL_012003C4, strength);
    answer & 0xffff_ff00
}

/// The effect steps of `run_00ba3e30` the set-ups call (unnamed, `thiscall` on the effect):
/// `00ba3b10` (three words), `00ba3b50` (five words) and `00ba3c70` (four words).
const EFFECT_STEP_00BA3B10: u32 = 0x00ba_3b10;
const EFFECT_STEP_00BA3B50: u32 = 0x00ba_3b50;
const EFFECT_STEP_00BA3C70: u32 = 0x00ba_3c70;
/// The data the set-ups give `00b8a8b0` (`011add48` for the blur, `011adc28` - 0x10 per level
/// for the other effect).
const BLUR_DATA_011ADD48: u32 = 0x011a_dd48;
const LEVEL_DATA_011ADC28: u32 = 0x011a_dc28;

/// A new `ImageSpaceShaderParam`: 0x88 bytes from the memory manager and its constructor, or
/// null when the allocation fails.
fn new_shader_param(e: &mut Engine) -> u32 {
    let block = e
        .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x88u32])
        .u32();
    if block != 0 {
        e.call(SHADER_PARAM_CONSTRUCT, &args![block]).u32()
    } else {
        0
    }
}

/// The shader the manager's table (+8 of the manager given) holds at byte offset `offset`.
fn manager_shader(e: &Engine, manager: u32, offset: u32) -> u32 {
    let table = e.mem.u32(manager + 8);
    e.mem.u32(table.wrapping_add(offset))
}

// Translated from 00ba46c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`), `Setup` (vtable +8) of the effect `00ba4640`
/// builds: `thiscall` with two stack words, the image-space manager (its shader table at +8)
/// and a word it does not read. Two sub-effects (`00ba3f90(2)`), four textures with the byte
/// +0x5c and three without (`00a90180` on `pTextures`); with n - 1 from the level at +0x58:
/// two parameters (`ImageSpaceShaderParam`, `InitConstantMap(0, 0x12)`, `00b8a8b0(1, 011adc28 -
/// 0x10 (n - 1), (n - 1) 0x20 + 0x10)`, `00b8a7c0`), the first stored as parameter 0 with
/// `00ba3b50(0, 1, 3, 0, 0)` (byte +0x5c set) or `00ba3b10(0, 1, 0)` and `00ba3c70(0, shader
/// 0x4e + n - 1 (+7 with the byte), it, 2)`, the second as parameter 1 with `00ba3b10(1, 2, 0)`
/// and `00ba3c70(1, shader at +0x138 + 4 (n - 1), it, 0)`.
// Translation incomplete: 00ba46c0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the two allocations) is not translated.
pub fn fn_00ba46c0(e: &mut Engine, this: Ptr, manager: Ptr, _unused_1: u32) {
    let base = this.addr();
    let manager = manager.addr();
    fn_00ba3f90(e, this, 2);
    let textures = if e.mem.u8(base + 0x5c) != 0 { 4u32 } else { 3 };
    e.call(
        ARRAY_SET_SIZE_00A90180,
        &args![base + EFFECT_TEXTURES, textures],
    );
    let level = e.mem.u32(base + 0x58).wrapping_sub(1);
    let data = LEVEL_DATA_011ADC28.wrapping_sub(level.wrapping_shl(4));
    let extra = if e.mem.u8(base + 0x5c) != 0 { 7u32 } else { 0 };
    let shader_index = extra.wrapping_add(0x4e).wrapping_add(level);
    let first = new_shader_param(e);
    e.call(INIT_CONSTANT_MAP, &args![first, 0u32, 0x12u32]);
    let bytes = level.wrapping_shl(5).wrapping_add(0x10);
    e.call(SHADER_PARAM_00B8A8B0, &args![first, 1u32, data, bytes]);
    e.call(SHADER_PARAM_00B8A7C0, &args![first]);
    set_at(e, base + EFFECT_PARAM_LIST, 0, first);
    if e.mem.u8(base + 0x5c) != 0 {
        e.call(
            EFFECT_STEP_00BA3B50,
            &args![base, 0u32, 1u32, 3u32, 0u32, 0u32],
        );
    } else {
        e.call(EFFECT_STEP_00BA3B10, &args![base, 0u32, 1u32, 0u32]);
    }
    let shader = manager_shader(e, manager, shader_index.wrapping_mul(4));
    e.call(
        EFFECT_STEP_00BA3C70,
        &args![base, 0u32, shader, first, 2u32],
    );
    let second = new_shader_param(e);
    e.call(INIT_CONSTANT_MAP, &args![second, 0u32, 0x12u32]);
    e.call(SHADER_PARAM_00B8A8B0, &args![second, 1u32, data, bytes]);
    e.call(SHADER_PARAM_00B8A7C0, &args![second]);
    set_at(e, base + EFFECT_PARAM_LIST, 1, second);
    e.call(EFFECT_STEP_00BA3B10, &args![base, 1u32, 2u32, 0u32]);
    let shader = manager_shader(e, manager, level.wrapping_mul(4).wrapping_add(0x138));
    e.call(
        EFFECT_STEP_00BA3C70,
        &args![base, 1u32, shader, second, 0u32],
    );
}

// Translated from 00ba4940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`), `Setup` (vtable +8) of the effect `00ba4880`
/// builds: `thiscall` with two stack words, the image-space manager (its shader table at +8)
/// and a word it does not read. Eleven sub-effects (`00ba3f90(0xb)`). Sub-effect 0: a parameter
/// (`InitConstantMap(0, 0x18)`, its byte +0x55 cleared, `00b8a8b0(3, 011add48, 0x40)`),
/// `00ba3c70(0, shader +0x104, it, 2)`, stored as parameter 0, `00ba3b10(0, 1, 1)`.
/// Sub-effects 1 to 7: parameter cleared, `00ba3b50(i, 2, 3, 0, 0)`, `00ba3c70(i, shader +0x40 +
/// 4 (i - 1), 0, 2)`. Sub-effects 8 and 9: a parameter as the first (byte +0x55 left),
/// `00ba3c70(i, shader +0x104, it, 0)`, stored, `00ba3b10(i, 2, 1)`. Sub-effect 10: a parameter
/// with only `InitConstantMap(0, 0x18)`, `00ba3c70(10, shader +0x88, it, 0)`, stored,
/// `00ba3b10(10, 1, 0)`.
// Translation incomplete: 00ba4940 - the C++ exception frame (FS:[0] chain, unwind states 0 to 3 for the four allocations) is not translated.
pub fn fn_00ba4940(e: &mut Engine, this: Ptr, manager: Ptr, _unused_1: u32) {
    let base = this.addr();
    let manager = manager.addr();
    fn_00ba3f90(e, this, 0xb);
    let param = new_shader_param(e);
    e.call(INIT_CONSTANT_MAP, &args![param, 0u32, 0x18u32]);
    e.mem.set_u8(param.wrapping_add(0x55), 0);
    e.call(
        SHADER_PARAM_00B8A8B0,
        &args![param, 3u32, BLUR_DATA_011ADD48, 0x40u32],
    );
    let shader = manager_shader(e, manager, 0x104);
    e.call(
        EFFECT_STEP_00BA3C70,
        &args![base, 0u32, shader, param, 2u32],
    );
    set_at(e, base + EFFECT_PARAM_LIST, 0, param);
    e.call(EFFECT_STEP_00BA3B10, &args![base, 0u32, 1u32, 1u32]);
    for index in 1..=7u32 {
        set_at(e, base + EFFECT_PARAM_LIST, index, 0);
        e.call(
            EFFECT_STEP_00BA3B50,
            &args![base, index, 2u32, 3u32, 0u32, 0u32],
        );
        let shader = manager_shader(e, manager, 0x40 + 4 * (index - 1));
        e.call(
            EFFECT_STEP_00BA3C70,
            &args![base, index, shader, 0u32, 2u32],
        );
    }
    for index in [8u32, 9] {
        let param = new_shader_param(e);
        e.call(INIT_CONSTANT_MAP, &args![param, 0u32, 0x18u32]);
        e.call(
            SHADER_PARAM_00B8A8B0,
            &args![param, 3u32, BLUR_DATA_011ADD48, 0x40u32],
        );
        let shader = manager_shader(e, manager, 0x104);
        e.call(
            EFFECT_STEP_00BA3C70,
            &args![base, index, shader, param, 0u32],
        );
        set_at(e, base + EFFECT_PARAM_LIST, index, param);
        e.call(EFFECT_STEP_00BA3B10, &args![base, index, 2u32, 1u32]);
    }
    let param = new_shader_param(e);
    e.call(INIT_CONSTANT_MAP, &args![param, 0u32, 0x18u32]);
    let shader = manager_shader(e, manager, 0x88);
    e.call(
        EFFECT_STEP_00BA3C70,
        &args![base, 0xau32, shader, param, 0u32],
    );
    set_at(e, base + EFFECT_PARAM_LIST, 0xa, param);
    e.call(EFFECT_STEP_00BA3B10, &args![base, 0xau32, 1u32, 0u32]);
}

/// The `NiPointer` global `00ba50d0` releases (`012003f4`).
const POINTER_012003F4: u32 = 0x0120_03f4;

// Translated from 00ba50d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`), no arguments: the object at `012003f4`, when set,
/// is released (count at +4, deleted through its vtable slot +4 at 0) and the global cleared.
pub fn fn_00ba50d0(e: &mut Engine) {
    let object = e.global::<u32>(POINTER_012003F4);
    if object != 0 {
        release_reference(e, object);
        e.set_global(POINTER_012003F4, 0u32);
    }
}

// Translated from 00ba52d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`), no arguments: the shared element count at
/// `012003ec`, when 0, becomes 0x1a below shader model 2 (`011f91c0`, signed) and 0x4e
/// otherwise. Answers the count (`EAX` at the `RET` 00ba52ef); its callers read it (00ba6d0e,
/// 00506064, 008201bc, 00821167).
pub fn fn_00ba52d0(e: &mut Engine) -> u32 {
    let mut count = e.global::<u32>(ELEMENT_COUNT_012003EC); // 00ba52d0: MOV EAX,[012003ec]
    if count == 0 {
        count = if e.global::<i32>(SHADER_MODEL_011F91C0) < 2 {
            0x1au32
        } else {
            0x4e
        };
        e.set_global(ELEMENT_COUNT_012003EC, count); // 00ba52ea: MOV [012003ec],EAX
    }
    count // 00ba52ef: RET with the count in EAX on both paths (00ba52d7 JNZ, 00ba52ea)
}

/// The vertex and pixel sources of `00ba5130` (file names `010b808c`, `010b8060`), the compiled
/// names (`010b807c`, `010b8050`), the vertex profile (`010a93e4`) and the pixel profile maker
/// `00b4f380` (unnamed, `cdecl`, one word).
const VERTEX_FILE_010B808C: u32 = 0x010b_808c;
const VERTEX_OUTPUT_010B807C: u32 = 0x010b_807c;
const PIXEL_FILE_010B8060: u32 = 0x010b_8060;
const PIXEL_OUTPUT_010B8050: u32 = 0x010b_8050;
const VERTEX_PROFILE_010A93E4: u32 = 0x010a_93e4;
const PIXEL_PROFILE_00B4F380: u32 = 0x00b4_f380;

// Translated from 00ba5130 (decompiled, FalloutNV.exe 1.4.0.525)
/// A function Ghidra did not make (`run_00ba3e30`), vtable +0x13c of the shader `00ba53f0`
/// builds: its shader loader, `thiscall`, no stack words. The vertex shader from `010b808c`
/// (no macros; compiled name `010b807c`, profile `010a93e4`) into the `NiPointer` at +0x7c,
/// then the pixel shader from `010b8060` (compiled name `010b8050`, the profile `00b4f380(1)`
/// answers, taken before its source block is written) into +0x80, as the other loaders do it
/// (`MakePath`, the formatter, the compile).
// Translation incomplete: 00ba5130 - the /GS stack cookie check (00ec408c) on the 0x2a4-byte frame is not translated.
pub fn fn_00ba5130(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x2b4, |e, frame| {
        let frame = frame.addr();
        let buffers = ShaderBuffers {
            name: 0xa8,
            path: 0x1ac,
        };
        set_up_sources(
            e,
            frame,
            &[ShaderSource {
                offset: 0x5c,
                words: &[VERTEX_FILE_010B808C, 0, 0],
                cleared: 0x40,
            }],
        );
        compile_shaders(
            e,
            base,
            frame,
            &buffers,
            &[ShaderCompile {
                source: 0x5c,
                output: VERTEX_OUTPUT_010B807C,
                profile: VERTEX_PROFILE_010A93E4,
                pixel: false,
                slot: 0x7c,
            }],
        );
        let profile = e.call(PIXEL_PROFILE_00B4F380, &args![1u32]).u32();
        set_up_sources(
            e,
            frame,
            &[ShaderSource {
                offset: 0x10,
                words: &[PIXEL_FILE_010B8060, 0, 0],
                cleared: 0x40,
            }],
        );
        compile_shaders(
            e,
            base,
            frame,
            &buffers,
            &[ShaderCompile {
                source: 0x10,
                output: PIXEL_OUTPUT_010B8050,
                profile,
                pixel: true,
                slot: 0x80,
            }],
        );
    });
}

// Translated from 00ba53c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): a pass pointer's assignment, `thiscall` on the
/// pointer with one stack word, the address of the source pointer. Nothing when both hold the
/// same pass; else the old pass's count (+0x64) drops (the pass deleted through `00e80190` at
/// 0), the pointer takes the source's pass and its count rises. Answers `this`.
pub fn fn_00ba53c0(e: &mut Engine, this: Ptr, source: Ptr) -> Ptr {
    let slot = this.addr();
    let old = e.mem.u32(slot);
    if old != e.mem.u32(source.addr()) {
        if old != 0 {
            let left = e.mem.u32(old + 0x64).wrapping_sub(1);
            e.mem.set_u32(old + 0x64, left);
            if left == 0 {
                e.call(PASS_DELETE_00E80190, &args![old]);
            }
        }
        let pass = e.mem.u32(source.addr());
        e.mem.set_u32(slot, pass);
        if pass != 0 {
            let held = e.mem.u32(pass + 0x64).wrapping_add(1);
            e.mem.set_u32(pass + 0x64, held);
        }
    }
    this
}

/// The free `00aa10f0` (unnamed; `cdecl`, one word).
const FREE_00AA10F0: u32 = 0x00aa_10f0;

// Translated from 00ba4050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): the scalar deleting destructor of the array
/// with the vtable `010b9d24`, `thiscall`, one word: the vtable stored, its buffer (+4) freed
/// (`00aa10f0`), then the memory manager's `Deallocate` when bit 0 of `flags` is set. Answers
/// `this`.
pub fn fn_00ba4050(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    let base = this.addr();
    let buffer = e.mem.u32(base + 4);
    e.mem.set_u32(base, ARRAY_VTABLE_010B9D24);
    e.call(FREE_00AA10F0, &args![buffer]);
    if flags as u8 & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00ba4170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`): `ImageSpaceEffect`'s scalar deleting
/// destructor, `thiscall`, one word: `ImageSpaceEffect::~ImageSpaceEffect` (`00ba3d30`), then
/// the memory manager's `Deallocate` when bit 0 of `flags` is set. Answers `this`.
pub fn fn_00ba4170(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![this]);
    if flags as u8 & 1 != 0 {
        e.call(MEMORY_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00ba5060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`), `cdecl`, one word: builds the eleven words 1,
/// 0, 2, 3, 6, 7, 8, 9, 4, 5, 10 in a stack local and answers the one at `index - 1`. The exe
/// does not check `index`: an index outside 1 to 11 reads the stack beside the local (here
/// the memory beside it).
pub fn fn_00ba5060(e: &mut Engine, index: u32) -> u32 {
    e.with_stack(0x2c, |e, local| {
        for (i, word) in [1u32, 0, 2, 3, 6, 7, 8, 9, 4, 5, 10].iter().enumerate() {
            e.mem.set_u32(local.addr() + 4 * i as u32, *word);
        }
        e.mem.u32(
            local
                .addr()
                .wrapping_add(index.wrapping_mul(4))
                .wrapping_sub(4),
        )
    })
}

/// `00e814b0` (unnamed; `thiscall`, no stack words): the base the particle shader's
/// `00ba5100` jumps to.
const BASE_00E814B0: u32 = 0x00e8_14b0;

// Translated from 00ba5100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`), `thiscall`, no stack words: with +0x28 set,
/// the object at +0x7c's slot +0xa0 and the object at +0x80's slot +0x88; then a tail jump to
/// `00e814b0` (what it answers in EAX, EDX and ST0 is this function's result).
pub fn fn_00ba5100(e: &mut Engine, this: Ptr) -> Ret {
    let base = this.addr();
    if e.mem.u32(base + 0x28) != 0 {
        let first = e.mem.u32(base + 0x7c);
        e.vcall(first, 0xa0, &args![]);
        let second = e.mem.u32(base + 0x80);
        e.vcall(second, 0x88, &args![]);
    }
    e.call(BASE_00E814B0, &args![this])
}

/// The uniform-form entry of [`fn_00ba5100`]: `this`; the tail call's registers pass through.
fn fn_00ba5100_entry(e: &mut Engine, a: &[u32]) -> Ret {
    fn_00ba5100(e, Ptr::new(a[0]))
}

/// The current pass word (`0126f74c`).
const CURRENT_PASS_0126F74C: u32 = 0x0126_f74c;

// Translated from 00ba52b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 60 of the vtable `010b80a8`, `thiscall`, one
/// word (the pass kind): kind 0x41 makes the pass at +0x78 current (`0126f74c`), any other the
/// pass at +0x74.
pub fn fn_00ba52b0(e: &mut Engine, this: Ptr, kind: u32) {
    let offset = if kind == 0x41 { 0x78 } else { 0x74 };
    let pass = e.mem.u32(this.addr() + offset);
    e.set_global(CURRENT_PASS_0126F74C, pass);
}

// Translated from 00ba52f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`), `thiscall`, no stack words: gives each of the
/// two passes at +0x74 and +0x78 to `this`'s vtable slot +0x128.
pub fn fn_00ba52f0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    for offset in [0x74u32, 0x78] {
        let pass = e.mem.u32(base + offset);
        e.vcall(base, 0x128, &args![pass]);
    }
}

/// The renderer (`011f4748`) and the four `float`s `00ba5320` writes after the one it reads
/// (`011adcc8`: a scale; `011adccc` its reciprocal, `011adcd0` that times the target's aspect,
/// `011adcd4` and `011adcd8` 1.0).
const RENDERER_011F4748: u32 = 0x011f_4748;
const SCALE_011ADCC8: u32 = 0x011a_dcc8;
const INVERSE_011ADCCC: u32 = 0x011a_dccc;
const ASPECT_011ADCD0: u32 = 0x011a_dcd0;
const ONE_011ADCD4: u32 = 0x011a_dcd4;
const ONE_011ADCD8: u32 = 0x011a_dcd8;

// Translated from 00ba5320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00ba3e30`), no arguments: with the renderer's (`011f4748`)
/// current render-target group (slot +0xc8) asked for its width (slot +0x8c with 0) and,
/// asked again, its height (slot +0x90 with 0), both read unsigned (`FILD`, plus 2^32 when
/// negative): `011adccc` = 1 / `[011adcc8]`, `011adcd0` = that times width / height (the
/// width stored as a `float`, the ratio too), `011adcd4` and `011adcd8` = 1.0.
pub fn fn_00ba5320(e: &mut Engine) {
    let renderer = e.global::<u32>(RENDERER_011F4748);
    let group = e.vcall(renderer, 0xc8, &args![]).u32();
    let width = e.vcall(group, 0x8c, &args![0u32]).u32();
    let width_bits = x87_store32(unsigned_size(e, width));
    let renderer = e.global::<u32>(RENDERER_011F4748);
    let group = e.vcall(renderer, 0xc8, &args![]).u32();
    let width = x87_load32(width_bits);
    let height = e.vcall(group, 0x90, &args![0u32]).u32();
    let height = unsigned_size(e, height);
    let aspect_bits = x87_store32(x87_div(width, height));
    let scale = x87_load32(e.global::<u32>(SCALE_011ADCC8));
    let inverse = x87_div(1.0, scale);
    e.set_global(INVERSE_011ADCCC, x87_store32(inverse));
    let product = x87_mul(inverse, x87_m32(aspect_bits));
    e.set_global(ASPECT_011ADCD0, x87_store32(product));
    e.set_global(ONE_011ADCD4, x87_store32(1.0));
    e.set_global(ONE_011ADCD8, x87_store32(1.0));
}

/// What `00ba5730` answers: the address `012003f8` (no RTTI locator for the vtable; the word
/// is the class's `NiRTTI` by its place in the vtable, unconfirmed).
const RECORD_012003F8: u32 = 0x0120_03f8;

// Translated from 00ba5730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (Ghidra made no function of it): slot 2 of the vtable `010b80a8`, `thiscall`
/// without reading `this`: answers the address `012003f8`.
pub fn fn_00ba5730(_e: &mut Engine, _this: Ptr) -> u32 {
    RECORD_012003F8
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ba_4050, fn_00ba4050(Ptr, u32) -> Ptr),
        entry!(0x00ba_4170, fn_00ba4170(Ptr, u32) -> Ptr),
        entry!(0x00ba_5060, fn_00ba5060(u32) -> u32),
        (0x00ba_5100, fn_00ba5100_entry as AbiFn),
        entry!(0x00ba_52b0, fn_00ba52b0(Ptr, u32)),
        entry!(0x00ba_52f0, fn_00ba52f0(Ptr)),
        entry!(0x00ba_5320, fn_00ba5320()),
        entry!(0x00ba_5730, fn_00ba5730(Ptr) -> u32),
        entry!(0x00ba_53f0, fn_00ba53f0(Ptr, Ptr) -> Ptr),
        entry!(0x00ba_4640, fn_00ba4640(Ptr) -> Ptr),
        entry!(0x00ba_4880, fn_00ba4880(Ptr) -> Ptr),
        entry!(0x00ba_4080, fn_00ba4080(Ptr) -> Ptr),
        entry!(0x00ba_3e30, fn_00ba3e30(Ptr, u32)),
        entry!(0x00ba_3f20, fn_00ba3f20(Ptr, u32, u32, u32, u32)),
        entry!(0x00ba_3fd0, fn_00ba3fd0(Ptr, u32, u32, u32, u32)),
        entry!(0x00ba_3f90, fn_00ba3f90(Ptr, u32)),
        entry!(0x00ba_4190, fn_00ba4190(Ptr)),
        entry!(0x00ba_46a0, fn_00ba46a0(Ptr, u32) -> Ptr),
        entry!(0x00ba_4480, fn_00ba4480(Ptr)),
        entry!(0x00ba_4920, fn_00ba4920(Ptr, u32) -> Ptr),
        entry!(0x00ba_4250, fn_00ba4250(Ptr, u32)),
        entry!(0x00ba_4260, fn_00ba4260(Ptr, u8)),
        entry!(0x00ba_4270, fn_00ba4270(Ptr, u32) -> u32),
        entry!(0x00ba_4530, fn_00ba4530(Ptr, u32)),
        entry!(0x00ba_4580, fn_00ba4580(Ptr)),
        entry!(0x00ba_45d0, fn_00ba45d0(Ptr) -> u32),
        entry!(0x00ba_4d20, fn_00ba4d20(Ptr, u32, u32, u32, u32)),
        entry!(0x00ba_4ee0, fn_00ba4ee0(Ptr, u32) -> u32),
        entry!(0x00ba_46c0, fn_00ba46c0(Ptr, Ptr, u32)),
        entry!(0x00ba_4940, fn_00ba4940(Ptr, Ptr, u32)),
        entry!(0x00ba_50d0, fn_00ba50d0()),
        entry!(0x00ba_52d0, fn_00ba52d0() -> u32),
        entry!(0x00ba_5130, fn_00ba5130(Ptr)),
        entry!(0x00ba_53c0, fn_00ba53c0(Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    fn construct(count: u32, model: i32) -> (Engine, u32, u32) {
        let mut e = rig();
        e.register(MEMSET, |_, a| a[0].into_ret());
        e.map(0x0120_0000, 0x1000);
        e.map(0x011f_9000, 0x1000);
        e.set_global::<u32>(ELEMENT_COUNT_012003EC, count);
        e.set_global::<i32>(SHADER_MODEL_011F91C0, model);
        let this = e.mem.alloc(0x130);
        e.mem.write(this, &[0xcc; 0x130]);
        e.mem.set_u32(this + 0x28, 0);
        let declaration = object(&mut e, 0);
        assert_eq!(e.call(0x00ba_53f0, &args![this, declaration]).u32(), this);
        (e, this, declaration)
    }

    #[test]
    fn fn_00ba53f0_builds_the_shader() {
        let (e, this, declaration) = construct(0, 2);
        assert_eq!(calls(&e, BS_SHADER_CONSTRUCT), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), VTABLE_010B80A8);
        for offset in [0x74u32, 0x78, 0x7c, 0x80] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        for offset in (0xd4u32..=0x128).step_by(4) {
            assert_eq!(e.mem.u32(this + offset), 0, "{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0xd0), 0xcccc_cccc);
        assert_eq!(e.mem.u32(this + 0x68), 0xe);
        assert_eq!(e.mem.u8(this + 0x24), 1);
        assert_eq!(e.mem.u32(this + 0x6c), 0x241);
        assert_eq!(e.mem.u32(this + 0x28), declaration);
        assert_eq!(e.global::<u32>(ELEMENT_COUNT_012003EC), 0x4e);
        assert_eq!(
            calls(&e, MEMORY_ALLOCATE),
            vec![vec![MEMORY_MANAGER, 0xea0]]
        );
        let buffer = e.mem.u32(this + 0x70);
        assert_eq!(
            calls(&e, MEMSET),
            vec![vec![buffer, 0, 0xea0], vec![this + 0x84, 0, 0x50]]
        );
    }

    #[test]
    fn fn_00ba53f0_count_follows_the_model_once() {
        let (e, _, _) = construct(0, 1);
        assert_eq!(e.global::<u32>(ELEMENT_COUNT_012003EC), 0x1a);
        let (e, _, _) = construct(3, 1);
        assert_eq!(e.global::<u32>(ELEMENT_COUNT_012003EC), 3);
        assert_eq!(calls(&e, MEMORY_ALLOCATE), vec![vec![MEMORY_MANAGER, 0x90]]);
    }
}

#[cfg(test)]
mod effect_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        for target in [
            EFFECT_CONSTRUCT_00BA4080,
            ARRAY_SET_SIZE_00A90180,
            VECTOR_CONSTRUCT,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        e.map(0x011a_9000, 0x1000);
        e.map(0x0120_0000, 0x1000);
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00ba4640_builds_the_effect() {
        let mut e = engine();
        let this = e.mem.alloc(0x60);
        assert_eq!(e.call(0x00ba_4640, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B8008);
        assert_eq!(e.mem.u32(this + 0x58), 0xffff_ffff);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 4]),
            ]
        );
    }

    #[test]
    fn fn_00ba4880_builds_the_effect_and_copies_the_words() {
        let mut e = engine();
        for index in 0..3 {
            e.set_global(WORDS_011A9490 + 4 * index, 0x0e00_0000 + index);
        }
        let this = e.mem.alloc(0x88);
        assert_eq!(e.call(0x00ba_4880, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010B8030);
        for index in 0..3 {
            assert_eq!(
                e.global::<u32>(WORDS_012003D4 + 4 * index),
                0x0e00_0000 + index
            );
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (
                    VECTOR_CONSTRUCT,
                    vec![
                        this + 0x58,
                        0x10,
                        3,
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
mod effect_base_tests {
    use super::*;

    #[test]
    fn fn_00ba4080_makes_the_five_arrays() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x58);
        e.mem.write(this, &[0xcc; 0x58]);
        assert_eq!(e.call(0x00ba_4080, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), EFFECT_VTABLE_010B7FE4);
        assert_eq!(e.mem.u8(this + 4), 0);
        assert_eq!(e.mem.u8(this + 5), 1);
        assert_eq!(e.mem.u16(this + 6), 0xcccc);
        let vtables = [
            ARRAY_VTABLE_010B8004,
            ARRAY_VTABLE_010B8004,
            ARRAY_VTABLE_010B9D24,
            ARRAY_VTABLE_010B9D24,
            ARRAY_VTABLE_010B9D24,
        ];
        for (index, vtable) in vtables.iter().enumerate() {
            let array = this + 8 + 0x10 * index as u32;
            assert_eq!(e.mem.u32(array), *vtable);
            assert_eq!(e.mem.bytes(array + 4, 0xa), vec![0; 0xa]);
            assert_eq!(e.mem.u16(array + 0xe), 1);
        }
    }
}

#[cfg(test)]
mod b054_tests {
    use super::*;

    /// The effect's vtable (slots at `EFFECT_SLOT + offset`) and the sub-effects' (slot +4 is
    /// `SUB_RENDER`), the parameters' (slot 0 is `PARAM_DELETE`).
    const EFFECT_VTABLE: u32 = 0x0e40_0000;
    const EFFECT_SLOT: u32 = 0x0e41_0000;
    const SUB_VTABLE: u32 = 0x0e42_0000;
    const SUB_RENDER: u32 = 0x0e43_0004;
    const PARAM_VTABLE: u32 = 0x0e44_0000;
    const PARAM_DELETE: u32 = 0x0e45_0000;
    /// A texture object's vtable: +0x94 answers the width, +0x98 the height (from +8, +0xc).
    const TEXTURE_VTABLE: u32 = 0x0e46_0000;

    fn rig(doubles: &[u32]) -> Engine {
        let mut e = Engine::new();
        for target in doubles {
            e.register(*target, |_, _| Ret::default());
        }
        let slots: Vec<u32> = (0..0x10).map(|slot| EFFECT_SLOT + 4 * slot).collect();
        e.put_vtable(EFFECT_VTABLE, &slots);
        for slot in [0x10u32, 0x14, 0x1c] {
            e.register(EFFECT_SLOT + slot, |_, _| Ret::default());
        }
        e.put_vtable(SUB_VTABLE, &[0, SUB_RENDER]);
        e.register(SUB_RENDER, |_, _| Ret::default());
        e.put_vtable(PARAM_VTABLE, &[PARAM_DELETE]);
        e.register(PARAM_DELETE, |_, _| Ret::default());
        e.put_vtable(TEXTURE_VTABLE, &[0; 0x27]);
        e.mem.set_u32(TEXTURE_VTABLE + 0x94, 0x0e47_0094);
        e.mem.set_u32(TEXTURE_VTABLE + 0x98, 0x0e47_0098);
        e.register(0x0e47_0094, |e, a| e.mem.u32(a[0] + 8).into_ret());
        e.register(0x0e47_0098, |e, a| e.mem.u32(a[0] + 0xc).into_ret());
        for page in [
            0x0109_9000u32,
            0x011a_9000,
            0x011a_d000,
            0x011a_e000,
            0x011f_9000,
        ] {
            e.map(page, 0x1000);
        }
        e.map(0x011f_f000, 0x1000);
        e.map(0x0120_0000, 0x1000);
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

    fn sequence(e: &Engine, addrs: &[u32]) -> Vec<(u32, Vec<u32>)> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| addrs.contains(&c.0))
            .cloned()
            .collect()
    }

    /// A block of words.
    fn block(e: &mut Engine, words: &[u32]) -> u32 {
        let block = e.mem.alloc(4 * words.len().max(1) as u32);
        for (index, word) in words.iter().enumerate() {
            e.mem.set_u32(block + 4 * index as u32, *word);
        }
        block
    }

    /// A sub-effect with no texture slots past 0 and a one-element texture array.
    fn sub_effect(e: &mut Engine) -> u32 {
        let sub = e.mem.alloc(0x40);
        e.mem.set_u32(sub, SUB_VTABLE);
        let first = block(e, &[0]);
        e.mem.set_u32(sub + 0x2c, first);
        e.mem.set_u16(sub + 0x32, 1);
        sub
    }

    /// An effect (0x80 bytes, the test vtable) with `count` sub-effects, parameters
    /// 0x0e50_0000 + index, four textures (null) and empty inputs and outputs.
    fn effect(e: &mut Engine, count: u32) -> u32 {
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this, EFFECT_VTABLE);
        let subs: Vec<u32> = (0..count).map(|_| sub_effect(e)).collect();
        let subs = block(e, &subs);
        e.mem.set_u32(this + 0xc, subs);
        e.mem.set_u16(this + 0x12, count as u16);
        let params: Vec<u32> = (0..count).map(|index| 0x0e50_0000 + index).collect();
        let params = block(e, &params);
        e.mem.set_u32(this + 0x1c, params);
        let textures = block(e, &[0; 4]);
        e.mem.set_u32(this + 0x2c, textures);
        let pairs = block(e, &[0; 2]);
        let inputs: Vec<u32> = (0..count).map(|_| pairs).collect();
        let inputs = block(e, &inputs);
        e.mem.set_u32(this + 0x3c, inputs);
        let outputs = block(e, &vec![0; count as usize]);
        e.mem.set_u32(this + 0x4c, outputs);
        this
    }

    /// A double for the element store that records (array, index, value) at the call.
    fn record_set_at(e: &mut Engine) -> u32 {
        let log = e.mem.alloc(0x400);
        e.register_double(ARRAY_SET_AT_00BA3AA0, move |e, a| {
            let used = e.mem.u32(log);
            let value = e.mem.u32(a[2]);
            for (index, word) in [a[0], a[1], value].iter().enumerate() {
                e.mem.set_u32(log + 4 + 12 * used + 4 * index as u32, *word);
            }
            e.mem.set_u32(log, used + 1);
            Ret::default()
        });
        log
    }

    fn stored(e: &Engine, log: u32) -> Vec<[u32; 3]> {
        let used = e.mem.u32(log);
        (0..used)
            .map(|entry| {
                let at = log + 4 + 12 * entry;
                [e.mem.u32(at), e.mem.u32(at + 4), e.mem.u32(at + 8)]
            })
            .collect()
    }

    #[test]
    fn fn_00ba3e30_gives_the_sub_effect_its_textures() {
        let mut e = rig(&[TEXTURE_SET_00D47A40]);
        let log = record_set_at(&mut e);
        let this = effect(&mut e, 2);
        let textures = e.mem.u32(this + 0x2c);
        e.mem.set_u32(textures, 0x0e60_0000);
        e.mem.set_u32(textures + 4, 0x0e60_0001);
        let sub = element(&e, this + 0xc, 1);
        e.mem.set_u16(sub + 0x30, 3);
        e.mem.set_u16(sub + 0x32, 0);
        let pairs = block(&mut e, &[0, 5, 0xffff_ffff, 9]);
        let inputs = e.mem.u32(this + 0x3c);
        e.mem.set_u32(inputs + 4, pairs);
        let outputs = e.mem.u32(this + 0x4c);
        e.mem.set_u32(outputs + 4, 1);
        e.call(0x00ba_3e30, &args![this, 1u32]);
        assert_eq!(stored(&e, log), vec![[sub + 0x28, 1, 0x0e60_0000]]);
        assert_eq!(
            calls(&e, TEXTURE_SET_00D47A40),
            vec![vec![0x0e60_0000, 5], vec![0x0e60_0001, 0]]
        );
        let first = e.mem.u32(sub + 0x2c);
        assert_eq!(e.mem.u32(first), 0x0e60_0001);
        assert_eq!(e.mem.u16(sub + 0x32), 1);
        assert_eq!(e.mem.u16(sub + 0x34), 1);
        // A null output over a held slot lowers the count.
        e.mem.set_u32(outputs + 4, 2);
        e.mem.set_u16(sub + 0x30, 0);
        e.call(0x00ba_3e30, &args![this, 1u32]);
        assert_eq!(e.mem.u32(first), 0);
        assert_eq!(e.mem.u16(sub + 0x34), 0);
        assert_eq!(calls(&e, TEXTURE_SET_00D47A40).len(), 2);
    }

    #[test]
    fn fn_00ba3f20_renders_every_sub_effect() {
        let mut e = rig(&[]);
        let this = effect(&mut e, 2);
        e.call(0x00ba_3f20, &args![this, 1u32, 2u32, 3u32, 4u32]);
        let (s0, s1) = (element(&e, this + 0xc, 0), element(&e, this + 0xc, 1));
        assert_eq!(
            sequence(
                &e,
                &[
                    EFFECT_SLOT + 0x10,
                    EFFECT_SLOT + 0x14,
                    EFFECT_SLOT + 0x1c,
                    SUB_RENDER
                ]
            ),
            vec![
                (EFFECT_SLOT + 0x10, vec![this, 3]),
                (EFFECT_SLOT + 0x1c, vec![this, 3]),
                (SUB_RENDER, vec![s0, 1, 2, 0x0e50_0000, 4]),
                (EFFECT_SLOT + 0x1c, vec![this, 3]),
                (SUB_RENDER, vec![s1, 1, 2, 0x0e50_0001, 4]),
                (EFFECT_SLOT + 0x14, vec![this]),
            ]
        );
    }

    #[test]
    fn fn_00ba3fd0_renders_the_flagged_sub_effects() {
        let mut e = rig(&[]);
        let this = effect(&mut e, 3);
        let flags = e.mem.alloc(4);
        e.mem.write(flags, &[0, 1, 0]);
        e.mem.set_u32(this + 0x5c, flags);
        e.call(0x00ba_3fd0, &args![this, 1u32, 2u32, 3u32, 4u32]);
        let s1 = element(&e, this + 0xc, 1);
        assert_eq!(calls(&e, SUB_RENDER), vec![vec![s1, 1, 2, 0x0e50_0001, 4]]);
        assert_eq!(calls(&e, EFFECT_SLOT + 0x10), vec![vec![this, 3]]);
        assert_eq!(calls(&e, EFFECT_SLOT + 0x14), vec![vec![this]]);
    }

    #[test]
    fn fn_00ba3f90_sizes_four_arrays() {
        let mut e = rig(&[ARRAY_SET_SIZE_00A90180]);
        let this = e.mem.alloc(0x58);
        e.call(0x00ba_3f90, &args![this, 7u32]);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![
                vec![this + 8, 7],
                vec![this + 0x18, 7],
                vec![this + 0x38, 7],
                vec![this + 0x48, 7]
            ]
        );
    }

    /// An effect whose parameters are deletable objects (the given ones null).
    fn effect_with_params(e: &mut Engine, count: u32, null: &[u32]) -> (u32, Vec<u32>) {
        let this = effect(e, count);
        let params: Vec<u32> = (0..count)
            .map(|index| {
                if null.contains(&index) {
                    0
                } else {
                    let param = e.mem.alloc(8);
                    e.mem.set_u32(param, PARAM_VTABLE);
                    param
                }
            })
            .collect();
        let block = block(e, &params);
        e.mem.set_u32(this + 0x1c, block);
        (this, params)
    }

    #[test]
    fn fn_00ba4190_deletes_two_params() {
        let mut e = rig(&[EFFECT_DESTRUCT_00BA3D30]);
        let log = record_set_at(&mut e);
        let (this, params) = effect_with_params(&mut e, 2, &[1]);
        e.call(0x00ba_4190, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010B8008);
        assert_eq!(calls(&e, PARAM_DELETE), vec![vec![params[0], 1]]);
        assert_eq!(stored(&e, log), vec![[this + 0x18, 0, 0]]);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
    }

    #[test]
    fn fn_00ba46a0_frees_with_bit_0() {
        let mut e = rig(&[EFFECT_DESTRUCT_00BA3D30, MEMORY_FREE]);
        record_set_at(&mut e);
        let (this, _) = effect_with_params(&mut e, 2, &[0, 1]);
        assert_eq!(e.call(0x00ba_46a0, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, MEMORY_FREE).is_empty());
        assert_eq!(e.call(0x00ba_46a0, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, this]]);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30).len(), 2);
    }

    #[test]
    fn fn_00ba4480_deletes_eleven_params_and_the_textures() {
        let mut e = rig(&[EFFECT_DESTRUCT_00BA3D30, VECTOR_DESTRUCT]);
        let log = record_set_at(&mut e);
        let (this, params) = effect_with_params(&mut e, 11, &[3]);
        e.call(0x00ba_4480, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010B8030);
        assert_eq!(calls(&e, PARAM_DELETE).len(), 10);
        assert_eq!(calls(&e, PARAM_DELETE)[9], vec![params[10], 1]);
        let cleared: Vec<u32> = stored(&e, log).iter().map(|s| s[1]).collect();
        assert_eq!(cleared, vec![0, 1, 2, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(
            sequence(&e, &[VECTOR_DESTRUCT, EFFECT_DESTRUCT_00BA3D30]),
            vec![
                (
                    VECTOR_DESTRUCT,
                    vec![this + 0x58, 0x10, 3, TEXTURE_DESTRUCT_00BA3A00]
                ),
                (EFFECT_DESTRUCT_00BA3D30, vec![this]),
            ]
        );
    }

    #[test]
    fn fn_00ba4920_frees_with_bit_0() {
        let mut e = rig(&[EFFECT_DESTRUCT_00BA3D30, VECTOR_DESTRUCT, MEMORY_FREE]);
        record_set_at(&mut e);
        let null: Vec<u32> = (0..11).collect();
        let (this, _) = effect_with_params(&mut e, 11, &null);
        e.call(0x00ba_4920, &args![this, 2u32]);
        assert!(calls(&e, MEMORY_FREE).is_empty());
        e.call(0x00ba_4920, &args![this, 3u32]);
        assert_eq!(calls(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, this]]);
    }

    #[test]
    fn fn_00ba4250_stores_the_level() {
        let mut e = rig(&[]);
        let this = e.mem.alloc(0x60);
        e.call(0x00ba_4250, &args![this, 5u32]);
        assert_eq!(e.mem.u32(this + 0x58), 5);
    }

    #[test]
    fn fn_00ba4260_stores_the_byte() {
        let mut e = rig(&[]);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x5c, 0xaaaa_aaaa);
        e.call(0x00ba_4260, &args![this, 0x1234_5601u32]);
        assert_eq!(e.mem.u32(this + 0x5c), 0xaaaa_aa01);
    }

    /// A texture holder whose `GetTexture` (doubled to answer the holder's +4) gives a texture
    /// object of the given width and height.
    fn texture_holder(e: &mut Engine, width: u32, height: u32) -> u32 {
        let texture = e.mem.alloc(0x10);
        e.mem.set_u32(texture, TEXTURE_VTABLE);
        e.mem.set_u32(texture + 8, width);
        e.mem.set_u32(texture + 0xc, height);
        let holder = e.mem.alloc(8);
        e.mem.set_u32(holder + 4, texture);
        holder
    }

    fn texture_rig(doubles: &[u32]) -> Engine {
        let mut e = rig(doubles);
        e.register(GET_TEXTURE, |e, a| e.mem.u32(a[0] + 4).into_ret());
        e.register(SET_PIXEL_CONSTANT, |_, _| 0x7777_7777u32.into_ret());
        e.register(SHADER_PARAM_00B8A8B0, |_, _| 0x1234_5678u32.into_ret());
        e
    }

    #[test]
    fn fn_00ba4270_lerps_the_rows_and_sets_the_constants() {
        let mut e = texture_rig(&[]);
        let this = effect(&mut e, 2);
        let textures = e.mem.u32(this + 0x2c);
        let wide = texture_holder(&mut e, 4, 0);
        let tall = texture_holder(&mut e, 0, 0x8000_0000);
        e.mem.set_u32(textures + 8, wide);
        e.mem.set_u32(textures + 4, tall);
        e.set_global(TWO_TO_32_01099378, 4_294_967_296.0f32);
        e.mem.set_u32(this + 0x58, 3);
        e.set_global(EFFECT_LEVEL_012003C4, 2.5f32);
        for row in 0..8u32 {
            for index in 0..15u32 {
                let value = (row * 10 + index) as f32;
                e.mem
                    .set_f32(ROWS_011ADE30 + row * 0xf0 + index * 0x10, value);
            }
        }
        e.mem.set_u8(this + 0x5c, 1);
        e.set_global(CONSTANT_012003C8, 7.0f32);
        e.set_global(CONSTANT_011ADBD4, 8.0f32);
        let answer = e.call(0x00ba_4270, &args![this, 0u32]).u32();
        assert_eq!(answer, 0x1234_5600);
        for index in 0..15u32 {
            assert_eq!(
                e.mem.f32(LERPED_011ADBE0 + index * 0x10),
                (25 + index) as f32,
                "{index}"
            );
        }
        let (p0, p1) = (0x0e50_0000, 0x0e50_0001);
        let inverse_height = (1.0f32 / 2_147_483_648.0).to_bits();
        assert_eq!(
            calls(&e, SET_PIXEL_CONSTANT),
            vec![
                vec![p0, 0, 0, inverse_height, 0, 0],
                vec![p0, 0x11, 7.0f32.to_bits(), 8.0f32.to_bits(), 0, 0],
                vec![p1, 0, 0.25f32.to_bits(), 0, 0, 0],
            ]
        );
        assert_eq!(
            calls(&e, SHADER_PARAM_00B8A8B0),
            vec![
                vec![p0, 1, 0x011a_dc18, 0x70],
                vec![p1, 1, 0x011a_dc18, 0x70]
            ]
        );
        // A factor of 0 becomes 1: the high row.
        e.mem.set_u8(this + 0x5c, 0);
        e.set_global(EFFECT_LEVEL_012003C4, 2.0f32);
        e.call(0x00ba_4270, &args![this, 0u32]);
        assert_eq!(e.mem.f32(LERPED_011ADBE0), 30.0);
        assert_eq!(calls(&e, SET_PIXEL_CONSTANT).len(), 5);
    }

    #[test]
    fn fn_00ba4530_borrows_two_textures() {
        let mut e = rig(&[BORROW_RENDERED_TEXTURE]);
        let log = record_set_at(&mut e);
        let this = e.mem.alloc(0x80);
        e.call(0x00ba_4530, &args![this, 0u32]);
        assert_eq!(
            calls(&e, BORROW_RENDERED_TEXTURE),
            vec![vec![this + 0x58, 0x18], vec![this + 0x68, 0x18]]
        );
        assert_eq!(
            stored(&e, log),
            vec![[this + 0x28, 2, this + 0x58], [this + 0x28, 3, this + 0x68]]
        );
    }

    #[test]
    fn fn_00ba4580_returns_two_textures() {
        let mut e = rig(&[RETURN_RENDERED_TEXTURE]);
        let log = record_set_at(&mut e);
        let this = effect(&mut e, 1);
        let textures = e.mem.u32(this + 0x2c);
        e.mem.set_u32(textures + 8, 0x0e70_0002);
        e.mem.set_u32(textures + 0xc, 0x0e70_0003);
        e.call(0x00ba_4580, &args![this]);
        assert_eq!(
            calls(&e, RETURN_RENDERED_TEXTURE),
            vec![vec![0x0e70_0002], vec![0x0e70_0003]]
        );
        assert_eq!(
            stored(&e, log),
            vec![[this + 0x28, 2, 0], [this + 0x28, 3, 0]]
        );
    }

    /// The blur's globals: the colour pair equal or not, both flags, the strength.
    fn blur(e: &mut Engine, changed: bool, flags: u8, strength: f32) {
        for index in 0..3 {
            e.set_global(WORDS_011A9490 + 4 * index, 0.5f32);
            e.set_global(WORDS_012003D4 + 4 * index, 0.5f32);
        }
        if changed {
            e.set_global(WORDS_012003D4 + 8, f32::NAN);
        }
        e.set_global(BLUR_FLAG_012003CC, flags);
        e.set_global(BLUR_FLAG_011F94A9, flags);
        e.set_global(BLUR_STRENGTH_012003D0, strength);
    }

    #[test]
    fn fn_00ba45d0_is_active_with_a_changed_colour_or_strength() {
        let mut e = rig(&[]);
        let run = |e: &mut Engine| e.call(0x00ba_45d0, &args![0u32]).u32();
        blur(&mut e, false, 1, 0.0);
        assert_eq!(run(&mut e), 0);
        blur(&mut e, true, 1, 0.0);
        assert_eq!(run(&mut e), 1);
        blur(&mut e, true, 0, f32::NAN);
        assert_eq!(run(&mut e), 0);
        blur(&mut e, false, 0, 0.25);
        assert_eq!(run(&mut e), 1);
    }

    fn render_rig() -> Engine {
        let mut e = rig(&[RENDER_STATE_00B97FA0, RENDER_STATE_00B97FF0]);
        e.register(CEIL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            value.ceil().into_ret()
        });
        e.register(FTOL, |_, a| {
            let value = f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32);
            (value as i32).into_ret()
        });
        e
    }

    fn rendered(e: &Engine, this: u32) -> Vec<u32> {
        calls(e, SUB_RENDER)
            .iter()
            .map(|a| {
                (0..11)
                    .find(|index| element(e, this + 0xc, *index) == a[0])
                    .unwrap()
            })
            .collect()
    }

    #[test]
    fn fn_00ba4d20_renders_the_passes_by_strength() {
        let mut e = render_rig();
        let this = effect(&mut e, 11);
        blur(&mut e, false, 0, 0.5);
        e.set_global(PASS_COUNTER_011FF9F0, 5u32);
        e.call(0x00ba_4d20, &args![this, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(rendered(&e, this), vec![0, 1, 10, 9]);
        assert_eq!(
            sequence(&e, &[RENDER_STATE_00B97FA0, RENDER_STATE_00B97FF0]),
            vec![
                (RENDER_STATE_00B97FA0, vec![1, 1]),
                (RENDER_STATE_00B97FF0, vec![6, 7, 0]),
                (RENDER_STATE_00B97FA0, vec![0, 0]),
            ]
        );
        assert_eq!(e.global::<u32>(PASS_COUNTER_011FF9F0), 4);
        assert_eq!(calls(&e, EFFECT_SLOT + 0x10), vec![vec![this, 3]]);
        assert_eq!(calls(&e, EFFECT_SLOT + 0x14), vec![vec![this]]);
        // Strength 2.5: passes 0 and 3, then 8; a changed colour with the flags keeps 3.
        let mut e = render_rig();
        let this = effect(&mut e, 11);
        blur(&mut e, true, 1, 2.5);
        e.call(0x00ba_4d20, &args![this, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(rendered(&e, this), vec![0, 3, 8]);
        assert!(calls(&e, RENDER_STATE_00B97FA0).is_empty());
        // Strength 1.0 with a changed colour: at least 2 passes.
        let mut e = render_rig();
        let this = effect(&mut e, 11);
        blur(&mut e, true, 1, 1.0);
        e.call(0x00ba_4d20, &args![this, 1u32, 2u32, 3u32, 4u32]);
        assert_eq!(rendered(&e, this), vec![0, 2, 8]);
    }

    #[test]
    fn fn_00ba4ee0_sets_the_inverse_size() {
        let mut e = texture_rig(&[]);
        let this = effect(&mut e, 10);
        let textures = e.mem.u32(this + 0x2c);
        let holder = texture_holder(&mut e, 4, 8);
        e.mem.set_u32(textures, holder);
        e.set_global(BLUR_STRENGTH_012003D0, 0.75f32);
        let answer = e.call(0x00ba_4ee0, &args![this, 0u32]).u32();
        assert_eq!(answer, 0x7777_7700);
        let (x, y) = (0.25f32.to_bits(), 0.125f32.to_bits());
        assert_eq!(
            calls(&e, SET_PIXEL_CONSTANT),
            vec![
                vec![0x0e50_0000, 2, x, y, 0, 0],
                vec![0x0e50_0008, 2, x, y, 0, 0],
                vec![0x0e50_0009, 2, x, y, 0.75f32.to_bits(), 0],
            ]
        );
        assert_eq!(e.global::<f32>(EFFECT_LEVEL_012003C4), 0.75);
        // Without a texture the renderer's object gives the size.
        e.mem.set_u32(textures, 0);
        let renderer = e.mem.alloc(4);
        e.put_vtable(0x0e48_0000, &[0; 0x33]);
        e.mem.set_u32(0x0e48_0000 + 0xc8, 0x0e49_00c8);
        e.mem.set_u32(renderer, 0x0e48_0000);
        e.set_global(RENDERER_011F9508, renderer);
        let object = e.mem.alloc(4);
        e.register_double(0x0e49_00c8, move |_, _| object.into_ret());
        e.put_vtable(0x0e4a_0000, &[0; 0x25]);
        e.mem.set_u32(0x0e4a_0000 + 0x8c, 0x0e4b_008c);
        e.mem.set_u32(0x0e4a_0000 + 0x90, 0x0e4b_0090);
        e.mem.set_u32(object, 0x0e4a_0000);
        e.register(0x0e4b_008c, |_, _| 2u32.into_ret());
        e.register(0x0e4b_0090, |_, _| 16u32.into_ret());
        e.call(0x00ba_4ee0, &args![this, 0u32]);
        assert_eq!(
            calls(&e, SET_PIXEL_CONSTANT)[3],
            vec![0x0e50_0000, 2, 0.5f32.to_bits(), 0.0625f32.to_bits(), 0, 0]
        );
        assert_eq!(calls(&e, 0x0e4b_008c), vec![vec![object, 0]]);
    }

    /// Doubles for the set-ups: the memory manager hands out blocks, the parameter constructor
    /// answers its block.
    fn setup_rig() -> Engine {
        let mut e = rig(&[
            ARRAY_SET_SIZE_00A90180,
            INIT_CONSTANT_MAP,
            SHADER_PARAM_00B8A8B0,
            SHADER_PARAM_00B8A7C0,
            EFFECT_STEP_00BA3B10,
            EFFECT_STEP_00BA3B50,
            EFFECT_STEP_00BA3C70,
        ]);
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(SHADER_PARAM_CONSTRUCT, |_, a| a[0].into_ret());
        e
    }

    /// A manager whose shader table holds `0x0e80_0000 + offset` at each byte offset.
    fn manager(e: &mut Engine) -> u32 {
        let words: Vec<u32> = (0..0x60).map(|index| 0x0e80_0000 + 4 * index).collect();
        let table = block(e, &words);
        let manager = e.mem.alloc(0xc);
        e.mem.set_u32(manager + 8, table);
        manager
    }

    #[test]
    fn fn_00ba46c0_sets_up_two_sub_effects() {
        let mut e = setup_rig();
        let log = record_set_at(&mut e);
        let manager = manager(&mut e);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x58, 3);
        e.mem.set_u8(this + 0x5c, 1);
        e.call(0x00ba_46c0, &args![this, manager, 0u32]);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180).last().unwrap(),
            &vec![this + 0x28, 4]
        );
        let params: Vec<u32> = stored(&e, log).iter().map(|s| s[2]).collect();
        assert_eq!(stored(&e, log)[0][..2], [this + 0x18, 0]);
        assert_eq!(stored(&e, log)[1][..2], [this + 0x18, 1]);
        for param in &params {
            assert!(calls(&e, INIT_CONSTANT_MAP).contains(&vec![*param, 0, 0x12]));
            assert!(calls(&e, SHADER_PARAM_00B8A8B0).contains(&vec![
                *param,
                1,
                0x011a_dc28 - 0x20,
                0x50
            ]));
            assert!(calls(&e, SHADER_PARAM_00B8A7C0).contains(&vec![*param]));
        }
        assert_eq!(
            calls(&e, EFFECT_STEP_00BA3B50),
            vec![vec![this, 0, 1, 3, 0, 0]]
        );
        assert_eq!(calls(&e, EFFECT_STEP_00BA3B10), vec![vec![this, 1, 2, 0]]);
        // Shader 0x4e + 7 + 2 = 0x57 (offset 0x15c), then +0x138 + 8.
        assert_eq!(
            calls(&e, EFFECT_STEP_00BA3C70),
            vec![
                vec![this, 0, 0x0e80_015c, params[0], 2],
                vec![this, 1, 0x0e80_0140, params[1], 0]
            ]
        );
        // Without the byte: three textures, `00ba3b10(0, 1, 0)` and shader 0x50.
        let mut e = setup_rig();
        record_set_at(&mut e);
        let manager = self::manager(&mut e);
        let this = e.mem.alloc(0x60);
        e.mem.set_u32(this + 0x58, 3);
        e.call(0x00ba_46c0, &args![this, manager, 0u32]);
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180).last().unwrap(),
            &vec![this + 0x28, 3]
        );
        assert_eq!(calls(&e, EFFECT_STEP_00BA3B10)[0], vec![this, 0, 1, 0]);
        assert_eq!(calls(&e, EFFECT_STEP_00BA3C70)[0][2], 0x0e80_0140);
    }

    #[test]
    fn fn_00ba4940_sets_up_eleven_sub_effects() {
        let mut e = setup_rig();
        let log = record_set_at(&mut e);
        let manager = manager(&mut e);
        let this = e.mem.alloc(0x80);
        e.call(0x00ba_4940, &args![this, manager, 0u32]);
        assert_eq!(calls(&e, ARRAY_SET_SIZE_00A90180)[0], vec![this + 8, 0xb]);
        let stores = stored(&e, log);
        let indices: Vec<u32> = stores.iter().map(|s| s[1]).collect();
        assert_eq!(indices, (0..11).collect::<Vec<u32>>());
        let first = stores[0][2];
        assert_eq!(e.mem.u8(first + 0x55), 0);
        for store in &stores[1..8] {
            assert_eq!(store[2], 0);
        }
        assert_eq!(calls(&e, INIT_CONSTANT_MAP).len(), 4);
        assert_eq!(calls(&e, SHADER_PARAM_00B8A8B0).len(), 3);
        let steps = calls(&e, EFFECT_STEP_00BA3C70);
        assert_eq!(steps[0], vec![this, 0, 0x0e80_0104, first, 2]);
        assert_eq!(steps[1], vec![this, 1, 0x0e80_0040, 0, 2]);
        assert_eq!(steps[7], vec![this, 7, 0x0e80_0058, 0, 2]);
        assert_eq!(steps[8], vec![this, 8, 0x0e80_0104, stores[8][2], 0]);
        assert_eq!(steps[10], vec![this, 10, 0x0e80_0088, stores[10][2], 0]);
        assert_eq!(
            calls(&e, EFFECT_STEP_00BA3B10),
            vec![
                vec![this, 0, 1, 1],
                vec![this, 8, 2, 1],
                vec![this, 9, 2, 1],
                vec![this, 10, 1, 0]
            ]
        );
        assert_eq!(
            calls(&e, EFFECT_STEP_00BA3B50)[6],
            vec![this, 7, 2, 3, 0, 0]
        );
    }

    #[test]
    fn fn_00ba50d0_releases_the_global() {
        let mut e = rig(&[IMPORT_INTERLOCKED_DECREMENT]);
        e.call(0x00ba_50d0, &args![]);
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
        let object = e.mem.alloc(8);
        e.mem.set_u32(object, SUB_VTABLE);
        e.set_global(POINTER_012003F4, object);
        e.call(0x00ba_50d0, &args![]);
        assert_eq!(
            calls(&e, IMPORT_INTERLOCKED_DECREMENT),
            vec![vec![object + 4]]
        );
        assert_eq!(calls(&e, SUB_RENDER), vec![vec![object]]);
        assert_eq!(e.global::<u32>(POINTER_012003F4), 0);
    }

    #[test]
    fn fn_00ba52d0_sets_the_count_once() {
        let mut e = rig(&[]);
        e.set_global(SHADER_MODEL_011F91C0, 1i32);
        e.call(0x00ba_52d0, &args![]);
        assert_eq!(e.global::<u32>(ELEMENT_COUNT_012003EC), 0x1a);
        e.set_global(SHADER_MODEL_011F91C0, 3i32);
        e.call(0x00ba_52d0, &args![]);
        assert_eq!(e.global::<u32>(ELEMENT_COUNT_012003EC), 0x1a);
        e.set_global(ELEMENT_COUNT_012003EC, 0u32);
        e.call(0x00ba_52d0, &args![]);
        assert_eq!(e.global::<u32>(ELEMENT_COUNT_012003EC), 0x4e);
    }

    #[test]
    fn fn_00ba52d0_answers_the_count_in_eax() {
        // EAX is the count at the RET (00ba52ef) whether it was set (00ba52d7 JNZ) or just
        // computed (00ba52ea); 00ba6cf0 reads it (00ba6d0e MOV EBX,EAX).
        let mut e = rig(&[]);
        e.set_global(ELEMENT_COUNT_012003EC, 0u32);
        e.set_global(SHADER_MODEL_011F91C0, 1i32);
        assert_eq!(e.call(0x00ba_52d0, &args![]).u32(), 0x1a);
        e.set_global(ELEMENT_COUNT_012003EC, 0u32);
        e.set_global(SHADER_MODEL_011F91C0, 2i32);
        assert_eq!(e.call(0x00ba_52d0, &args![]).u32(), 0x4e);
        e.set_global(ELEMENT_COUNT_012003EC, 0x77u32);
        assert_eq!(e.call(0x00ba_52d0, &args![]).u32(), 0x77);
    }

    #[test]
    fn fn_00ba5130_compiles_both_shaders() {
        use crate::units::bsshader::geometrydecalshader::shader_test_rig::{
            compiled, loader_rig, Compiled,
        };
        let mut e = loader_rig();
        e.register(PIXEL_PROFILE_00B4F380, |_, _| 0x0e90_0000u32.into_ret());
        let this = e.mem.alloc(0x84);
        e.call(0x00ba_5130, &args![this]);
        let vertex = e.mem.u32(this + 0x7c);
        let pixel = e.mem.u32(this + 0x80);
        assert_eq!(
            compiled(&e, vertex),
            Compiled {
                file: VERTEX_FILE_010B808C,
                output: VERTEX_OUTPUT_010B807C,
                profile: VERTEX_PROFILE_010A93E4,
                pixel: false,
                macros: [0; 8],
            }
        );
        assert_eq!(
            compiled(&e, pixel),
            Compiled {
                file: PIXEL_FILE_010B8060,
                output: PIXEL_OUTPUT_010B8050,
                profile: 0x0e90_0000,
                pixel: true,
                macros: [0; 8],
            }
        );
        assert_eq!(e.mem.u32(vertex + 4), 1);
        assert_eq!(e.mem.u32(pixel + 4), 1);
    }

    #[test]
    fn fn_00ba53c0_moves_the_pass_counts() {
        let mut e = rig(&[PASS_DELETE_00E80190]);
        let old = e.mem.alloc(0x68);
        e.mem.set_u32(old + 0x64, 1);
        let new = e.mem.alloc(0x68);
        e.mem.set_u32(new + 0x64, 1);
        let slot = block(&mut e, &[old]);
        let source = block(&mut e, &[new]);
        assert_eq!(e.call(0x00ba_53c0, &args![slot, source]).u32(), slot);
        assert_eq!(e.mem.u32(slot), new);
        assert_eq!(e.mem.u32(new + 0x64), 2);
        assert_eq!(calls(&e, PASS_DELETE_00E80190), vec![vec![old]]);
        // The same pass: nothing changes.
        e.call(0x00ba_53c0, &args![slot, source]);
        assert_eq!(e.mem.u32(new + 0x64), 2);
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

    fn rig() -> Engine {
        let mut e = Engine::new();
        for target in [FREE_00AA10F0, MEMORY_FREE, EFFECT_DESTRUCT_00BA3D30] {
            e.register(target, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00ba4050_frees_the_buffer_and_maybe_itself() {
        let mut e = rig();
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array + 4, 0x4444);
        assert_eq!(e.call(0x00ba_4050, &args![array, 0x100u32]).u32(), array);
        assert_eq!(e.mem.u32(array), ARRAY_VTABLE_010B9D24);
        assert_eq!(calls(&e, FREE_00AA10F0), vec![vec![0x4444]]);
        assert!(calls(&e, MEMORY_FREE).is_empty());
        e.call(0x00ba_4050, &args![array, 1u32]);
        assert_eq!(calls(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, array]]);
    }

    #[test]
    fn fn_00ba4170_destroys_the_effect_and_maybe_frees_it() {
        let mut e = rig();
        let effect = e.mem.alloc(0x40);
        assert_eq!(e.call(0x00ba_4170, &args![effect, 0u32]).u32(), effect);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![effect]]);
        assert!(calls(&e, MEMORY_FREE).is_empty());
        e.call(0x00ba_4170, &args![effect, 3u32]);
        assert_eq!(calls(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, effect]]);
    }

    #[test]
    fn fn_00ba5060_maps_one_based_indices() {
        let mut e = rig();
        let got: Vec<u32> = (1..=11)
            .map(|i| e.call(0x00ba_5060, &args![i as u32]).u32())
            .collect();
        assert_eq!(got, vec![1, 0, 2, 3, 6, 7, 8, 9, 4, 5, 10]);
    }

    #[test]
    fn fn_00ba5100_restores_then_jumps_to_the_base() {
        let mut e = rig();
        e.register(0x0e90_0000, |_, _| Ret::default());
        e.register(0x0e90_0004, |_, _| Ret::default());
        e.register(BASE_00E814B0, |_, _| Ret {
            eax: 9,
            ..Ret::default()
        });
        let first_vtable = e.mem.alloc(0xa4);
        e.mem.set_u32(first_vtable + 0xa0, 0x0e90_0000);
        let first = e.mem.alloc(4);
        e.mem.set_u32(first, first_vtable);
        let second_vtable = e.mem.alloc(0x8c);
        e.mem.set_u32(second_vtable + 0x88, 0x0e90_0004);
        let second = e.mem.alloc(4);
        e.mem.set_u32(second, second_vtable);
        let this = e.mem.alloc(0x84);
        e.mem.set_u32(this + 0x7c, first);
        e.mem.set_u32(this + 0x80, second);
        assert_eq!(e.call(0x00ba_5100, &args![this]).u32(), 9);
        assert!(calls(&e, 0x0e90_0000).is_empty());
        e.mem.set_u32(this + 0x28, 1);
        assert_eq!(e.call(0x00ba_5100, &args![this]).u32(), 9);
        assert_eq!(calls(&e, 0x0e90_0000), vec![vec![first]]);
        assert_eq!(calls(&e, 0x0e90_0004), vec![vec![second]]);
        assert_eq!(calls(&e, BASE_00E814B0).len(), 2);
    }

    #[test]
    fn fn_00ba52b0_52f0_and_5730() {
        let mut e = rig();
        e.map(0x0126_f000, 0x1000);
        e.register(0x0e90_0008, |_, _| Ret::default());
        let vtable = e.mem.alloc(0x130);
        e.mem.set_u32(vtable + 0x128, 0x0e90_0008);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this, vtable);
        e.mem.set_u32(this + 0x74, 0x74);
        e.mem.set_u32(this + 0x78, 0x78);
        e.call(0x00ba_52b0, &args![this, 0x41u32]);
        assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), 0x78);
        e.call(0x00ba_52b0, &args![this, 0x40u32]);
        assert_eq!(e.global::<u32>(CURRENT_PASS_0126F74C), 0x74);
        e.call(0x00ba_52f0, &args![this]);
        assert_eq!(
            calls(&e, 0x0e90_0008),
            vec![vec![this, 0x74], vec![this, 0x78]]
        );
        assert_eq!(e.call(0x00ba_5730, &args![this]).u32(), 0x0120_03f8);
    }

    #[test]
    fn fn_00ba5320_sets_the_aspect_constants() {
        let mut e = rig();
        e.map(0x011f_4000, 0x1000);
        e.map(0x011a_d000, 0x1000);
        e.map(0x0109_9000, 0x1000);
        e.set_global(TWO_TO_32_01099378, 4_294_967_296.0f32);
        e.set_global(SCALE_011ADCC8, 2.0f32);
        e.register(0x0e90_0010, |e, _| e.mem.u32(0x011f_4f00).into_ret());
        e.register(0x0e90_0014, |_, _| 1600u32.into_ret());
        e.register(0x0e90_0018, |_, _| 800u32.into_ret());
        let renderer_vtable = e.mem.alloc(0xcc);
        e.mem.set_u32(renderer_vtable + 0xc8, 0x0e90_0010);
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, renderer_vtable);
        e.set_global(RENDERER_011F4748, renderer);
        let group_vtable = e.mem.alloc(0x94);
        e.mem.set_u32(group_vtable + 0x8c, 0x0e90_0014);
        e.mem.set_u32(group_vtable + 0x90, 0x0e90_0018);
        let group = e.mem.alloc(4);
        e.mem.set_u32(group, group_vtable);
        e.set_global(0x011f_4f00, group);
        e.call(0x00ba_5320, &args![]);
        assert_eq!(e.global::<f32>(INVERSE_011ADCCC), 0.5);
        assert_eq!(e.global::<f32>(ASPECT_011ADCD0), 1.0);
        assert_eq!(e.global::<f32>(ONE_011ADCD4), 1.0);
        assert_eq!(e.global::<f32>(ONE_011ADCD8), 1.0);
        assert_eq!(calls(&e, 0x0e90_0014), vec![vec![group, 0]]);
        // A width with the top bit set reads as unsigned.
        e.register(0x0e90_0014, |_, _| 0x8000_0000u32.into_ret());
        e.register(0x0e90_0018, |_, _| 0x4000_0000u32.into_ret());
        e.call(0x00ba_5320, &args![]);
        assert_eq!(e.global::<f32>(ASPECT_011ADCD0), 1.0);
    }
}
