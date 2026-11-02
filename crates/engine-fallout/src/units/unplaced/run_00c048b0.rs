//! `(unplaced)/run_00c048b0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::bsshader::bsshaderproperty::RenderPass;

// Translated from 00c1a450 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar-deleting-destructor-shaped function (no name in the engine map; class
/// unconfirmed): runs the destructor body at `00a89760`, then the sized delete
/// (`00aa1460`, `(this, size)`) of 0xd8 bytes when bit 0 of `flags` is set. Answers
/// `this`.
pub fn fn_00c1a450(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    const DESTRUCTOR_BODY: u32 = 0x00a8_9760;
    const SIZED_DELETE: u32 = 0x00aa_1460;
    e.call(DESTRUCTOR_BODY, &args![this]);
    if flags & 1 != 0 {
        e.call(SIZED_DELETE, &args![this, 0xd8u32]);
    }
    this
}

/// The `ImageSpaceEffect` base constructor `00ba4080` (unnamed in the engine map; `thiscall`,
/// no stack words) and the `NiTArray` resize `00a90180` (`thiscall` on the array, `RET 4`).
const IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080: u32 = 0x00ba_4080;
const ARRAY_SET_SIZE_00A90180: u32 = 0x00a9_0180;
/// `ImageSpaceEffect::pTextures` (Xbox PDB, +0x28; the same on PC).
const EFFECT_TEXTURES: u32 = 0x28;
/// The `float` `00c049c0` stores at +0x5c (`01016248`, through `FLD`).
const VALUE_01016248: u32 = 0x0101_6248;
/// `_eh_vector_constructor_iterator_` (`00ec782f`: array, element size, count, constructor,
/// destructor) with the element constructor `004ebb70` and `ImageSpaceTexture`'s destructor
/// (`00ba3a00`, as Ghidra names it) that `00c05420` gives it.
const VECTOR_CONSTRUCT: u32 = 0x00ec_782f;
const TEXTURE_CONSTRUCT_004EBB70: u32 = 0x004e_bb70;
const TEXTURE_DESTRUCT_00BA3A00: u32 = 0x00ba_3a00;

// Translated from 00c049c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall`, no stack words: the constructor of the 0x78-byte
/// image-space effect `00b94520` stores at index 0xc of the effect array (`EFFECT_MOTION_BLUR`
/// in the Xbox PDB's `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is
/// unconfirmed). Runs the base constructor `00ba4080`, sets the vtable `010bcaac`, stores 0.0
/// in the `float`s +0x58 to +0x64, clears the bytes +0x68 to +0x6a and the words +0x6c to
/// +0x74, sizes the texture array (+0x28) to three (`00a90180`), then stores 0.0 at +0x58,
/// the `float` at `01016248` (through the FPU, a signalling NaN quieted) at +0x5c and 0.0 at
/// +0x60 and +0x64. Answers `this`.
// Translation incomplete: 00c049c0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and the array) is not translated.
pub fn fn_00c049c0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    let zero = 0.0f32.to_bits();
    e.call(IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080, &args![this]);
    e.mem.set_u32(base, 0x010b_caac);
    for offset in [0x58u32, 0x5c, 0x60, 0x64] {
        e.mem.set_u32(base + offset, zero);
    }
    for offset in [0x68u32, 0x69, 0x6a] {
        e.mem.set_u8(base + offset, 0);
    }
    for offset in [0x6cu32, 0x70, 0x74] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(
        ARRAY_SET_SIZE_00A90180,
        &args![base + EFFECT_TEXTURES, 3u32],
    );
    let value = x87_quiet_bits(e.global::<u32>(VALUE_01016248));
    e.mem.set_u32(base + 0x58, zero);
    e.mem.set_u32(base + 0x5c, value);
    e.mem.set_u32(base + 0x60, zero);
    e.mem.set_u32(base + 0x64, zero);
    this
}

// Translated from 00c04d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall`, no stack words: the constructor of the 0xc8-byte
/// image-space effect `00b94520` stores at index 3 of the effect array (`EFFECT_CINEMATIC` in
/// the Xbox PDB's `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is unconfirmed).
/// Runs the base constructor `00ba4080`, clears the byte +0x58 and the word +0x64, sets the
/// vtable `010bcae8` and stores the addresses `011ae684` at +0x5c and `011ae694` at +0x60.
/// Answers `this`.
pub fn fn_00c04d20(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080, &args![this]);
    e.mem.set_u8(base + 0x58, 0);
    e.mem.set_u32(base + 0x64, 0);
    e.mem.set_u32(base, 0x010b_cae8);
    e.mem.set_u32(base + 0x5c, 0x011a_e684);
    e.mem.set_u32(base + 0x60, 0x011a_e694);
    this
}

// Translated from 00c05420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall`, no stack words: the constructor of the 0x7c-byte
/// image-space effect `00b94520` stores at index 0 of the effect array (`EFFECT_REFRACTION`
/// in the Xbox PDB's `ImageSpaceManager::ImageSpaceEffectEnum`; the class name is
/// unconfirmed). Runs the base constructor `00ba4080`, clears +0x58, sets the vtable
/// `010bcb18`, constructs two 0x10-byte elements at +0x5c (`00ec782f` with `004ebb70` and
/// `00ba3a00`) and sizes the texture array (+0x28) to four (`00a90180`). Answers `this`.
// Translation incomplete: 00c05420 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the base and the elements) is not translated.
pub fn fn_00c05420(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080, &args![this]);
    e.mem.set_u32(base + 0x58, 0);
    e.mem.set_u32(base, 0x010b_cb18);
    e.call(
        VECTOR_CONSTRUCT,
        &args![
            base + 0x5c,
            0x10u32,
            2u32,
            TEXTURE_CONSTRUCT_004EBB70,
            TEXTURE_DESTRUCT_00BA3A00
        ],
    );
    e.call(
        ARRAY_SET_SIZE_00A90180,
        &args![base + EFFECT_TEXTURES, 4u32],
    );
    this
}

// Translated from 00c05680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map, `thiscall`, no stack words: the constructor of the 0x68-byte
/// image-space effect `00b94520` stores at index 0xd of the effect array
/// (`EFFECT_VOLUMETRIC_FOG` in the Xbox PDB's `ImageSpaceManager::ImageSpaceEffectEnum`; the
/// class name is unconfirmed). Runs the base constructor `00ba4080`, sets the vtable
/// `010bcb3c` and stores 0.0 in the `float`s +0x58 to +0x64. Answers `this`.
pub fn fn_00c05680(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080, &args![this]);
    e.mem.set_u32(base, 0x010b_cb3c);
    for offset in [0x58u32, 0x5c, 0x60, 0x64] {
        e.mem.set_u32(base + offset, 0.0f32.to_bits());
    }
    this
}

/// The memory manager (`011f6238`) and its allocate / free (`00aa3e40` / `00aa4060`,
/// `thiscall`, one stack word each).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const MEMORY_ALLOCATE: u32 = 0x00aa_3e40;
const MEMORY_FREE: u32 = 0x00aa_4060;
/// `ImageSpaceEffect::~ImageSpaceEffect` (engine map, `00ba3d30`), the `NiTArray` element
/// store `00ba3aa0` (`thiscall` on the array, `RET 8`: index, address of the value) and the
/// effect steps `00ba3f90` (`RET 4`), `00ba3b10` (`RET 0xc`), `00ba3b50` (`RET 0x14`) and
/// `00ba3c70` (`RET 0x10`), unnamed in the engine map.
const IMAGE_SPACE_EFFECT_DESTRUCT: u32 = 0x00ba_3d30;
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const EFFECT_STEP_00BA3F90: u32 = 0x00ba_3f90;
const EFFECT_STEP_00BA3B10: u32 = 0x00ba_3b10;
const EFFECT_STEP_00BA3B50: u32 = 0x00ba_3b50;
const EFFECT_STEP_00BA3C70: u32 = 0x00ba_3c70;
/// `ImageSpaceShaderParam::ImageSpaceShaderParam`, `InitConstantMap` (`RET 8`) and
/// `SetPixelConstant` (`RET 0x14`), engine map names (`00b8a9e0`, `00b8ab10`, `00b8a790`).
const SHADER_PARAM_CONSTRUCT: u32 = 0x00b8_a9e0;
const SHADER_PARAM_INIT_CONSTANT_MAP: u32 = 0x00b8_ab10;
const SHADER_PARAM_SET_PIXEL_CONSTANT: u32 = 0x00b8_a790;
/// `ImageSpaceEffect::pEffectParamList` (Xbox PDB, +0x18; its data pointer at +0x1c).
const EFFECT_PARAMS: u32 = 0x18;
/// `` `eh vector destructor iterator' `` (`00ec5fce`, `stdcall`: array, element size, count,
/// destructor).
const VECTOR_DESTRUCT: u32 = 0x00ec_5fce;

/// The memory manager's allocate on the singleton.
fn memory_allocate(e: &mut Engine, size: u32) -> u32 {
    e.call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, size]).u32()
}

/// The memory manager's free on the singleton.
fn memory_free(e: &mut Engine, block: u32) {
    e.call(MEMORY_FREE, &args![MEMORY_MANAGER, block]);
}

/// Stores the word `value` at `index` of the effect's parameter list (`00ba3aa0` with the
/// address of a local holding it).
fn set_effect_param(e: &mut Engine, effect: u32, index: u32, value: u32) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), value);
        e.call(
            ARRAY_SET_AT_00BA3AA0,
            &args![effect + EFFECT_PARAMS, index, local.addr()],
        );
    });
}

/// The destructors' step for parameter `index`: when the list's data (+0x1c) holds one
/// there, it is deleted through its vtable slot 0 (with 1) and the entry set to null.
fn delete_effect_param(e: &mut Engine, effect: u32, index: u32) {
    let data = e.mem.u32(effect + 0x1c);
    if e.mem.u32(data.wrapping_add(4 * index)) == 0 {
        return;
    }
    let param = e.mem.u32(data.wrapping_add(4 * index));
    if param != 0 {
        e.vcall(param, 0, &args![1u32]);
    }
    set_effect_param(e, effect, index, 0);
}

/// A new `ImageSpaceShaderParam` (0x88 bytes from the memory manager, constructed with
/// `00b8a9e0`; null when the allocation fails) and its `InitConstantMap(0, registers)`.
fn new_shader_param(e: &mut Engine, registers: u32) -> u32 {
    let block = memory_allocate(e, 0x88);
    let param = if block != 0 {
        e.call(SHADER_PARAM_CONSTRUCT, &args![block]).u32()
    } else {
        0
    };
    e.call(
        SHADER_PARAM_INIT_CONSTANT_MAP,
        &args![param, 0u32, registers],
    );
    param
}

/// `SetPixelConstant(0, a, b, c, d)` with the four `float`s as their bits.
fn set_pixel_constant(e: &mut Engine, param: u32, values: [u32; 4]) {
    e.call(
        SHADER_PARAM_SET_PIXEL_CONSTANT,
        &args![param, 0u32, values[0], values[1], values[2], values[3]],
    );
}

/// The word at `offset` of the object at +8 of the image-space manager `manager`.
fn manager_texture(e: &Engine, manager: u32, offset: u32) -> u32 {
    let holder = e.mem.u32(manager.wrapping_add(8));
    e.mem.u32(holder.wrapping_add(offset))
}

// Translated from 00c048b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the destructor body of the
/// effect `00c049c0` constructs (vtable `010bcaac`): deletes parameter 0, destroys the
/// `ImageSpaceTexture` at +0x68 (`00ba3a00`) and runs `~ImageSpaceEffect` (`00ba3d30`).
// Translation incomplete: 00c048b0 - the C++ exception frame (FS:[0] chain, unwind states 1 and 0) is not translated.
pub fn fn_00c048b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010b_caac);
    delete_effect_param(e, base, 0);
    e.call(TEXTURE_DESTRUCT_00BA3A00, &args![base + 0x68]);
    e.call(IMAGE_SPACE_EFFECT_DESTRUCT, &args![this]);
}

// Translated from 00c04a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the scalar deleting destructor of that
/// effect: `00c048b0`, then the memory manager's free when bit 0 of `flags` is set. Answers
/// `this`.
pub fn fn_00c04a80(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00c048b0(e, this);
    if flags & 1 != 0 {
        memory_free(e, this.addr());
    }
    this
}

// Translated from 00c04aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: the image-space manager and a word it
/// does not read): that effect's setup. `00ba3f90(1)`; a shader parameter with two
/// registers, its bytes +0x55 and +0x56 cleared and pixel constant `(+0x58, 0, 0, 0)`;
/// parameter 0; `00ba3b50(0, 1, 2, 0, 1)`; `00ba3c70(0, texture +0xd4, parameter, 0)`.
// Translation incomplete: 00c04aa0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the parameter's allocation) is not translated.
pub fn fn_00c04aa0(e: &mut Engine, this: Ptr, manager: Ptr, _unused_2: u32) {
    let base = this.addr();
    e.call(EFFECT_STEP_00BA3F90, &args![this, 1u32]);
    let param = new_shader_param(e, 2);
    e.mem.set_u8(param.wrapping_add(0x55), 0);
    e.mem.set_u8(param.wrapping_add(0x56), 0);
    let zero = 0.0f32.to_bits();
    let value = x87_quiet_bits(e.mem.u32(base + 0x58));
    set_pixel_constant(e, param, [value, zero, zero, zero]);
    set_effect_param(e, base, 0, param);
    e.call(
        EFFECT_STEP_00BA3B50,
        &args![this, 0u32, 1u32, 2u32, 0u32, 1u32],
    );
    let texture = manager_texture(e, manager.addr(), 0xd4);
    e.call(
        EFFECT_STEP_00BA3C70,
        &args![this, 0u32, texture, param, 0u32],
    );
}

// Translated from 00c04990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; that effect's `IsActive` slot):
/// true when the word at `011f91c0` is above 4, the byte at `012024a0` is set and 0.0 is
/// below the `float` +0x58 (`FLDZ; FCOMP`; false for a NaN).
pub fn fn_00c04990(e: &mut Engine, this: Ptr) -> bool {
    if e.mem.i32(0x011f_91c0) <= 4 || e.mem.u8(0x0120_24a0) == 0 {
        return false;
    }
    let value = x87_m32(e.mem.u32(this.addr() + 0x58));
    0.0 < value
}

// Translated from 00c04950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; a slot of that effect's vtable): a jump to `00b4f510` with
/// the same `this`, answering what it answers.
// 00b4f510 takes no `this`: 00876850 calls it with ECX left from 00b540b0.
pub fn fn_00c04950(e: &mut Engine, _this: Ptr) -> u32 {
    e.call(0x00b4_f510, &args![]).u32()
}

// Translated from 00c04da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the destructor body of the
/// effect `00c04d20` constructs (vtable `010bcae8`): deletes parameter 0 and runs
/// `~ImageSpaceEffect`.
// Translation incomplete: 00c04da0 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
pub fn fn_00c04da0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x010b_cae8);
    delete_effect_param(e, this.addr(), 0);
    e.call(IMAGE_SPACE_EFFECT_DESTRUCT, &args![this]);
}

// Translated from 00c05160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the scalar deleting destructor of that
/// effect (`00c04da0`, then the memory manager's free on bit 0 of `flags`). Answers `this`.
pub fn fn_00c05160(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00c04da0(e, this);
    if flags & 1 != 0 {
        memory_free(e, this.addr());
    }
    this
}

// Translated from 00c04d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; a slot of the `00c04d20` effect's vtable, `thiscall`,
/// `RET 4`): copies the byte +0x58 to `bParamsChanged` (+5, Xbox PDB) and stores `value` at
/// +0x5c.
pub fn fn_00c04d50(e: &mut Engine, this: Ptr, value: u32) {
    let base = this.addr();
    let flag = e.mem.u8(base + 0x58);
    e.mem.set_u8(base + 5, flag);
    e.mem.set_u32(base + 0x5c, value);
}

// Translated from 00c04d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; that effect's `IsActive` slot): true when +0x60 is set and
/// the byte at `011f941e` is clear.
pub fn fn_00c04d60(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x60) != 0 && e.mem.u8(0x011f_941e) == 0
}

// Translated from 00c04f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; that effect's `UpdateParams`
/// slot): fills the `float`s +0x88 to +0xc4 from the data at +0x60 (every value an
/// `FLD`/`FSTP` copy, a signalling NaN quieted). With the byte +0x58 set, the fourth value is
/// the larger of the +0xc of the data at +0x5c and that of +0x60 (the second when not below,
/// NaN included), the next three come from +0x5c's first three and three of the last ones
/// are 1.0; otherwise they are the data's +0xc to +0x1c. The words +0x68 to +0x74 and the
/// data's +0 to +8 and +0x20 to +0x2c fill the rest. Answers the block +0x88 (`LEA EAX,[ECX+0x88]`
/// at 00c04fef, left in `EAX`): its caller copies the constants from it (00bae5dd).
pub fn fn_00c04f50(e: &mut Engine, this: Ptr) -> u32 {
    let base = this.addr();
    // The locals +0 to +0x34 of the exe's frame, by index.
    let mut local = [0u32; 14];
    local[7] = e.mem.u32(base + 0x6c);
    local[6] = e.mem.u32(base + 0x68);
    local[9] = e.mem.u32(base + 0x74);
    local[8] = e.mem.u32(base + 0x70);
    let data = e.mem.u32(base + 0x60);
    let one = 1.0f32.to_bits();
    if e.mem.u8(base + 0x58) != 0 {
        let other = e.mem.u32(base + 0x5c);
        let first = x87_quiet_bits(e.mem.u32(other + 0xc));
        let second = x87_quiet_bits(e.mem.u32(data.wrapping_add(0xc)));
        let (a, b) = (f32::from_bits(first), f32::from_bits(second));
        local[0] = if b < a { first } else { second };
        local[2] = x87_quiet_bits(e.mem.u32(other));
        local[3] = x87_quiet_bits(e.mem.u32(other + 4));
        local[4] = x87_quiet_bits(e.mem.u32(other + 8));
        local[5] = one;
        local[8] = one;
        local[9] = one;
    } else {
        for (slot, offset) in [(0usize, 0xcu32), (2, 0x10), (3, 0x14), (4, 0x18), (5, 0x1c)] {
            local[slot] = x87_quiet_bits(e.mem.u32(data.wrapping_add(offset)));
        }
    }
    for (slot, offset) in [(10usize, 0x20u32), (11, 0x24), (12, 0x28), (13, 0x2c)] {
        local[slot] = x87_quiet_bits(e.mem.u32(data.wrapping_add(offset)));
    }
    for (index, offset) in [0u32, 4, 8].iter().enumerate() {
        let value = x87_quiet_bits(e.mem.u32(data.wrapping_add(*offset)));
        e.mem.set_u32(base + 0x88 + 4 * index as u32, value);
    }
    for (index, slot) in [0usize, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]
        .iter()
        .enumerate()
    {
        e.mem
            .set_u32(base + 0x94 + 4 * index as u32, x87_quiet_bits(local[*slot]));
    }
    base + 0x88
}

// Translated from 00c050b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; a slot of that effect's vtable, `thiscall`, `RET 8`): sets
/// `bParamsChanged` (+5), clears the `float`s +0x68 to +0x84, stores `first` at +0x60 and
/// `mask` at +0x64, then each of the `float`s +0x68 to +0x74 is 1.0 when its bit (0 to 3) of
/// `mask` is set and 0.0 otherwise.
pub fn fn_00c050b0(e: &mut Engine, this: Ptr, first: u32, mask: u32) {
    let base = this.addr();
    e.mem.set_u8(base + 5, 1);
    for offset in (0x68u32..=0x84).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    e.mem.set_u32(base + 0x60, first);
    e.mem.set_u32(base + 0x64, mask);
    for bit in 0..4u32 {
        let value = if mask & (1 << bit) != 0 { 1.0f32 } else { 0.0 };
        e.mem.set_u32(base + 0x68 + 4 * bit, value.to_bits());
    }
}

// Translated from 00c05180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: the image-space manager and a word it
/// does not read): the `00c04d20` effect's setup. `00ba3f90(1)`, the texture array sized to
/// two (`00a90180`), a shader parameter with four registers as parameter 0,
/// `00ba3b10(0, 1, 0)` and `00ba3c70(0, texture +0xc8, parameter, 0)`.
// Translation incomplete: 00c05180 - the C++ exception frame (FS:[0] chain, unwind state 0 for the parameter's allocation) is not translated.
pub fn fn_00c05180(e: &mut Engine, this: Ptr, manager: Ptr, _unused_2: u32) {
    let base = this.addr();
    e.call(EFFECT_STEP_00BA3F90, &args![this, 1u32]);
    e.call(
        ARRAY_SET_SIZE_00A90180,
        &args![base + EFFECT_TEXTURES, 2u32],
    );
    let param = new_shader_param(e, 4);
    set_effect_param(e, base, 0, param);
    e.call(EFFECT_STEP_00BA3B10, &args![this, 0u32, 1u32, 0u32]);
    let texture = manager_texture(e, manager.addr(), 0xc8);
    e.call(
        EFFECT_STEP_00BA3C70,
        &args![this, 0u32, texture, param, 0u32],
    );
}

// Translated from 00c05250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the destructor body of the
/// effect `00c05420` constructs (vtable `010bcb18`): deletes parameters 0 to 2, destroys the
/// two `ImageSpaceTexture`s at +0x5c (`00ec5fce` with `00ba3a00`) and runs
/// `~ImageSpaceEffect`.
// Translation incomplete: 00c05250 - the C++ exception frame (FS:[0] chain, unwind states 1 and 0) is not translated.
pub fn fn_00c05250(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x010b_cb18);
    for index in 0..3 {
        delete_effect_param(e, base, index);
    }
    e.call(
        VECTOR_DESTRUCT,
        &args![base + 0x5c, 0x10u32, 2u32, TEXTURE_DESTRUCT_00BA3A00],
    );
    e.call(IMAGE_SPACE_EFFECT_DESTRUCT, &args![this]);
}

// Translated from 00c054a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the scalar deleting destructor of that
/// effect (`00c05250`, then the memory manager's free on bit 0 of `flags`). Answers `this`.
pub fn fn_00c054a0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00c05250(e, this);
    if flags & 1 != 0 {
        memory_free(e, this.addr());
    }
    this
}

// Translated from 00c05350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; that effect's `IsActive` slot): true when the byte at
/// `011f9422` is set and +0x58 is not zero.
pub fn fn_00c05350(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(0x011f_9422) != 0 && e.mem.u32(this.addr() + 0x58) != 0
}

// Translated from 00c054c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: the image-space manager and a word it
/// does not read): that effect's setup. `00ba3f90(3)`, four textures; parameter 0 with one
/// register and pixel constant `(0, [011ae6c8], [011ae6cc], 0)`, `00ba3b10(0, 1, 1)`,
/// `00ba3c70(0, texture +0xb4, it, 2)`; parameter 1 null, `00ba3b50(1, 2, 3, 0, 0)`,
/// `00ba3c70(1, texture +0x4c, null, 2)`; parameter 2 with one register, byte +0x56 cleared
/// and pixel constant `(1, 1, 1, 1)`, `00ba3b50(2, 2, 1, 1, 0)`,
/// `00ba3c70(2, texture +0xb8, it, 0)`. The manager's argument slot is the local of the
/// later parameters.
// Translation incomplete: 00c054c0 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the parameters' allocations) is not translated.
pub fn fn_00c054c0(e: &mut Engine, this: Ptr, manager: Ptr, _unused_2: u32) {
    let base = this.addr();
    let manager = manager.addr();
    e.call(EFFECT_STEP_00BA3F90, &args![this, 3u32]);
    e.call(
        ARRAY_SET_SIZE_00A90180,
        &args![base + EFFECT_TEXTURES, 4u32],
    );
    let first = new_shader_param(e, 1);
    let zero = 0.0f32.to_bits();
    let low = x87_quiet_bits(e.mem.u32(0x011a_e6c8));
    let high = x87_quiet_bits(e.mem.u32(0x011a_e6cc));
    set_pixel_constant(e, first, [zero, low, high, zero]);
    set_effect_param(e, base, 0, first);
    e.call(EFFECT_STEP_00BA3B10, &args![this, 0u32, 1u32, 1u32]);
    let texture = manager_texture(e, manager, 0xb4);
    e.call(
        EFFECT_STEP_00BA3C70,
        &args![this, 0u32, texture, first, 2u32],
    );
    set_effect_param(e, base, 1, 0);
    e.call(
        EFFECT_STEP_00BA3B50,
        &args![this, 1u32, 2u32, 3u32, 0u32, 0u32],
    );
    let texture = manager_texture(e, manager, 0x4c);
    e.call(
        EFFECT_STEP_00BA3C70,
        &args![this, 1u32, texture, 0u32, 2u32],
    );
    let second = new_shader_param(e, 1);
    e.mem.set_u8(second.wrapping_add(0x56), 0);
    let one = 1.0f32.to_bits();
    set_pixel_constant(e, second, [one; 4]);
    set_effect_param(e, base, 2, second);
    e.call(
        EFFECT_STEP_00BA3B50,
        &args![this, 2u32, 2u32, 1u32, 1u32, 0u32],
    );
    let texture = manager_texture(e, manager, 0xb8);
    e.call(
        EFFECT_STEP_00BA3C70,
        &args![this, 2u32, texture, second, 0u32],
    );
}

// Translated from 00c056a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the destructor body of the
/// effect `00c05680` constructs (vtable `010bcb3c`): deletes parameter 0 and runs
/// `~ImageSpaceEffect`.
// Translation incomplete: 00c056a0 - the C++ exception frame (FS:[0] chain, unwind state 0) is not translated.
pub fn fn_00c056a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x010b_cb3c);
    delete_effect_param(e, this.addr(), 0);
    e.call(IMAGE_SPACE_EFFECT_DESTRUCT, &args![this]);
}

// Translated from 00c05760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): the scalar deleting destructor of that
/// effect (`00c056a0`, then the memory manager's free on bit 0 of `flags`). Answers `this`.
pub fn fn_00c05760(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00c056a0(e, this);
    if flags & 1 != 0 {
        memory_free(e, this.addr());
    }
    this
}

// Translated from 00c05780 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`: the image-space manager and a word it
/// does not read): that effect's setup. `00ba3f90(1)`, two textures, a shader parameter with
/// one register and pixel constant `(1, 1, 1, 1)` as parameter 0,
/// `00ba3b50(0, 1, -1, 1, 0)` and `00ba3c70(0, texture +0xa8, parameter, 0)`.
// Translation incomplete: 00c05780 - the C++ exception frame (FS:[0] chain, unwind state 0 for the parameter's allocation) is not translated.
pub fn fn_00c05780(e: &mut Engine, this: Ptr, manager: Ptr, _unused_2: u32) {
    let base = this.addr();
    e.call(EFFECT_STEP_00BA3F90, &args![this, 1u32]);
    e.call(
        ARRAY_SET_SIZE_00A90180,
        &args![base + EFFECT_TEXTURES, 2u32],
    );
    let param = new_shader_param(e, 1);
    set_pixel_constant(e, param, [1.0f32.to_bits(); 4]);
    set_effect_param(e, base, 0, param);
    e.call(
        EFFECT_STEP_00BA3B50,
        &args![this, 0u32, 1u32, u32::MAX, 1u32, 0u32],
    );
    let texture = manager_texture(e, manager.addr(), 0xa8);
    e.call(
        EFFECT_STEP_00BA3C70,
        &args![this, 0u32, texture, param, 0u32],
    );
}

/// Imported `InterlockedIncrement` / `InterlockedDecrement` (import slots).
const IMPORT_INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
const IMPORT_INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The shader-pass callees (engine-map names where it has them): `00e804d0` (`cdecl`, one
/// word, see `00c0d170`), `NiD3DTextureStage::CreateNewStage` (`00e7edf0`, `cdecl`: the
/// address of the stage pointer), `BSShader::SetupDefaultStage` (`00be0cf0`, `cdecl`: stage
/// and four words), `NiD3DPass::SetStage` (`00e802b0`, `thiscall`, `RET 8`: index, stage),
/// the pass's render-state block constructor `00e7f5d0` (no arguments) and its setter
/// `00e7f430` (`thiscall`, `RET 0xc`), the stage release `00e7ecd0` and the pass release
/// `00e80190` (`thiscall`, no stack words).
const PASS_STEP_00E804D0: u32 = 0x00e8_04d0;
const CREATE_NEW_STAGE: u32 = 0x00e7_edf0;
const SETUP_DEFAULT_STAGE: u32 = 0x00be_0cf0;
const PASS_SET_STAGE: u32 = 0x00e8_02b0;
const RENDER_STATES_NEW_00E7F5D0: u32 = 0x00e7_f5d0;
const RENDER_STATE_SET_00E7F430: u32 = 0x00e7_f430;
const STAGE_RELEASE_00E7ECD0: u32 = 0x00e7_ecd0;
const PASS_RELEASE_00E80190: u32 = 0x00e8_0190;
/// The `float` the stage blocks copy into a stage's +8 (`011f9534`).
const STAGE_VALUE_011F9534: u32 = 0x011f_9534;

/// The compiler's `NiPointer` assignment with interlocked counts: nothing when `new` is
/// already at `slot`; else the old object's count (+4) is decremented (deleted through its
/// slot +4 at zero), `new` stored and, when not null, counted.
fn assign_counted(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old == new {
        return;
    }
    if old != 0
        && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old.wrapping_add(4)])
            .u32()
            == 0
    {
        e.vcall(old, 4, &args![]);
    }
    e.mem.set_u32(slot, new);
    if new != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![new.wrapping_add(4)]);
    }
}

/// A pass's use count (+0x64) lowered by one (a plain decrement), released (`00e80190`) at
/// zero.
fn release_pass(e: &mut Engine, pass: u32) {
    let count = e.mem.u32(pass.wrapping_add(0x64)).wrapping_sub(1);
    e.mem.set_u32(pass.wrapping_add(0x64), count);
    if count == 0 {
        e.call(PASS_RELEASE_00E80190, &args![pass]);
    }
}

/// A pass's use count raised by one when the pass is not null.
fn hold_pass(e: &mut Engine, pass: u32) {
    if pass != 0 {
        let count = e.mem.u32(pass + 0x64).wrapping_add(1);
        e.mem.set_u32(pass + 0x64, count);
    }
}

/// The stage held in a local, given up at the end: its count (+0x60) lowered, released
/// (`00e7ecd0`) at zero.
fn release_stage(e: &mut Engine, stage: u32) {
    if stage != 0 {
        let count = e.mem.u32(stage + 0x60).wrapping_sub(1);
        e.mem.set_u32(stage + 0x60, count);
        if count == 0 {
            e.call(STAGE_RELEASE_00E7ECD0, &args![stage]);
        }
    }
}

/// A new stage into the local `stage` (`CreateNewStage`), set up with the four words
/// (`SetupDefaultStage(stage, a, b, c, d)`).
fn new_default_stage(e: &mut Engine, stage: u32, words: [u32; 4]) -> u32 {
    e.call(CREATE_NEW_STAGE, &args![stage]);
    let made = e.mem.u32(stage);
    e.call(
        SETUP_DEFAULT_STAGE,
        &args![made, words[0], words[1], words[2], words[3]],
    );
    e.mem.u32(stage)
}

/// The pass's render-state block (+0x30, made by `00e7f5d0` when null) given
/// `(state, value, flag)` (`00e7f430`).
fn set_render_state(e: &mut Engine, pass: u32, state: u32, value: u32, flag: u32) {
    if e.mem.u32(pass + 0x30) == 0 {
        let block = e.call(RENDER_STATES_NEW_00E7F5D0, &args![]).u32();
        e.mem.set_u32(pass + 0x30, block);
    }
    let block = e.mem.u32(pass + 0x30);
    e.call(RENDER_STATE_SET_00E7F430, &args![block, state, value, flag]);
}

/// `00e804d0` on `count` consecutive words from `first`.
fn pass_steps(e: &mut Engine, first: u32, count: u32) {
    for index in 0..count {
        e.call(PASS_STEP_00E804D0, &args![first + 4 * index]);
    }
}

// Translated from 00c0d170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; a shader's slot, `thiscall`, no stack words): `00e804d0` on
/// each of the 0x21 words from `01202230`, then the shader's slot +0x158; true (only `AL`
/// is set).
pub fn fn_00c0d170(e: &mut Engine, this: Ptr) -> bool {
    pass_steps(e, 0x0120_2230, 0x21);
    e.vcall(this.addr(), 0x158, &args![]);
    true
}

// Translated from 00c0d1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the shader's slots +0x15c,
/// +0x160, +0x164, +0x168 and +0x16c, then a jump to its slot +0x170, answering what that
/// answers.
pub fn fn_00c0d1b0(e: &mut Engine, this: Ptr) -> u32 {
    for slot in [0x15cu32, 0x160, 0x164, 0x168, 0x16c] {
        e.vcall(this.addr(), slot, &args![]);
    }
    e.vcall(this.addr(), 0x170, &args![]).u32()
}

// Translated from 00c12480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): `00e804d0` on each of the 0x1a
/// words from `012022c0`, then the shader's slot +0x158; true.
pub fn fn_00c12480(e: &mut Engine, this: Ptr) -> bool {
    pass_steps(e, 0x0120_22c0, 0x1a);
    e.vcall(this.addr(), 0x158, &args![]);
    true
}

// Translated from 00c16350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): `00e804d0` on each of the 0x1e
/// words from `01202350`, the shader's slot +0x15c and, when the word at `011f91c0` is at
/// least 2, its slot +0x160; true.
pub fn fn_00c16350(e: &mut Engine, this: Ptr) -> bool {
    pass_steps(e, 0x0120_2350, 0x1e);
    e.vcall(this.addr(), 0x15c, &args![]);
    if e.mem.i32(0x011f_91c0) >= 2 {
        e.vcall(this.addr(), 0x160, &args![]);
    }
    true
}

// Translated from 00c163a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, one word): the pass at `011fda48 + 4 * index` for
/// `index` up to 0xcf, else 0 (read from the argument slot just cleared). The release of a
/// held temporary that follows never runs: its flag is set only on the out-of-range path,
/// where the temporary is null.
pub fn fn_00c163a0(e: &mut Engine, index: u32) -> u32 {
    let (held, value, flagged) = if index <= 0xcf {
        (index, e.mem.u32(0x011f_da48 + index * 4), false)
    } else {
        (0, 0, true)
    };
    if flagged && held != 0 {
        // Unreachable: `held` is 0 whenever the flag is set.
        if e.call(IMPORT_INTERLOCKED_DECREMENT, &args![held.wrapping_add(4)])
            .u32()
            == 0
        {
            e.vcall(held, 4, &args![]);
        }
    }
    value
}

// Translated from 00c06dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`, eight words read as signed bytes): packs them into a
/// mask, the first as bit 0: `((((w8 << 1 | w7) << 1 | w6) ...) << 1 | w1)` with each byte
/// sign-extended before the `OR`.
#[allow(clippy::too_many_arguments)]
pub fn fn_00c06dc0(
    _e: &mut Engine,
    w1: u32,
    w2: u32,
    w3: u32,
    w4: u32,
    w5: u32,
    w6: u32,
    w7: u32,
    w8: u32,
) -> u32 {
    let byte = |word: u32| word as u8 as i8 as i32 as u32;
    let mut mask = byte(w8);
    for word in [w7, w6, w5, w4, w3, w2, w1] {
        mask = mask.wrapping_add(mask) | byte(word);
    }
    mask
}

// Translated from 00c06e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`cdecl`: pass, mask byte): when the pass's +0x18 is below 7,
/// seven default stages (`SetupDefaultStage(stage, i, 1, 2, 1)`) are added at the pass's
/// +0x14 (`SetStage`), each with its byte +0x64 set to bit `i` of the sign-extended mask;
/// the mask byte goes to the pass's +0x4c and the stage local is given up.
// Translation incomplete: 00c06e10 - the C++ exception frame (FS:[0] chain, unwind state 0 for the stage local) is not translated.
pub fn fn_00c06e10(e: &mut Engine, pass: Ptr, mask: u32) {
    let pass = pass.addr();
    e.with_stack(4, |e, stage| {
        let stage = stage.addr();
        if e.mem.u32(pass + 0x18) < 7 {
            let bits = mask as u8 as i8 as i32 as u32;
            let mut bit = 1u32;
            for index in 0..7u32 {
                let made = new_default_stage(e, stage, [index, 1, 2, 1]);
                let slot = e.mem.u32(pass + 0x14);
                e.call(PASS_SET_STAGE, &args![pass, slot, made]);
                let made = e.mem.u32(stage);
                e.mem.set_u8(made + 0x64, u8::from(bit & bits != 0));
                bit = bit.rotate_left(1);
            }
        }
        e.mem.set_u8(pass + 0x4c, mask as u8);
        let made = e.mem.u32(stage);
        release_stage(e, made);
    });
}

/// The vtable slot `00c06f80` gives a constant to: +0x84 when the constant's top four bits
/// (+0x14) are 2, else +0x80.
fn add_constant(e: &mut Engine, shader: u32, constant: u32) {
    let slot = if e.mem.u32(constant + 0x14) & 0xf000_0000 == 0x2000_0000 {
        0x84
    } else {
        0x80
    };
    e.vcall(shader, slot, &args![constant]);
}

// Translated from 00c06f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words). For each of the 0x70 passes at
/// `01200ec0` (the held pass switched: the old one released, the new one held): except for
/// 0x66 and 0x67, two 0x44-byte shader programs are made (`00aa13e0`, `00b7e330` with the
/// shader's +0x18 and 0 or 2; null when the allocation fails); the first gets the constants
/// of `01201080` (17) whose bit (from 1 up, rotating) is set in the mask at `01200a28`, the
/// second those of `012010c4` (10) by the mask at `01200868` (slot +0x84 or +0x80); they
/// become the pass's +0x48 and +0x34. Then the passes at `01201058` and `0120105c` get the
/// shader's +0x70 and +0x6c the same way, and the held pass is released.
// Translation incomplete: 00c06f80 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the held pass and the allocations) is not translated.
pub fn fn_00c06f80(e: &mut Engine, this: Ptr) {
    const PASSES: u32 = 0x0120_0ec0;
    const ALLOCATE_00AA13E0: u32 = 0x00aa_13e0;
    const PROGRAM_CONSTRUCT_00B7E330: u32 = 0x00b7_e330;
    let base = this.addr();
    let mut held = 0u32;
    for index in 0..0x70u32 {
        let pass = e.mem.u32(PASSES + index * 4);
        if held != pass {
            if held != 0 {
                release_pass(e, held);
            }
            held = e.mem.u32(PASSES + index * 4);
            hold_pass(e, held);
        }
        if index == 0x66 || index == 0x67 {
            continue;
        }
        let mut programs = [0u32; 2];
        for (made, kind) in programs.iter_mut().zip([0u32, 2]) {
            let block = e.call(ALLOCATE_00AA13E0, &args![0x44u32]).u32();
            *made = if block != 0 {
                let shader = e.mem.u32(base + 0x18);
                e.call(PROGRAM_CONSTRUCT_00B7E330, &args![block, shader, kind])
                    .u32()
            } else {
                0
            };
        }
        for (program, (masks, first, last)) in programs.iter().zip([
            (0x0120_0a28u32, 0x0120_1080u32, 0x0120_10c4u32),
            (0x0120_0868, 0x0120_10c4, 0x0120_10ec),
        ]) {
            let mut bit = 1u32;
            let mut at = first;
            while at < last {
                bit = bit.rotate_left(1);
                let mask = e.mem.u32(masks + index * 4);
                if mask & bit != 0 {
                    let constant = e.mem.u32(at);
                    if constant != 0 {
                        add_constant(e, *program, constant);
                    }
                }
                at += 4;
            }
        }
        assign_counted(e, held + 0x48, programs[0]);
        assign_counted(e, held + 0x34, programs[1]);
    }
    for global in [0x0120_1058u32, 0x0120_105c] {
        let vertex = e.mem.u32(base + 0x70);
        let pass = e.mem.u32(global);
        assign_counted(e, pass + 0x48, vertex);
        let pixel = e.mem.u32(base + 0x6c);
        let pass = e.mem.u32(global);
        assign_counted(e, pass + 0x34, pixel);
    }
    if held != 0 {
        release_pass(e, held);
    }
}

/// One numbered stage of the pass-setup functions: a default stage `(index, a, b, c)` set at
/// `index` of `pass`.
fn numbered_stage(e: &mut Engine, pass: u32, stage: u32, index: u32, words: [u32; 3]) {
    let made = new_default_stage(e, stage, [index, words[0], words[1], words[2]]);
    e.call(PASS_SET_STAGE, &args![pass, index, made]);
}

/// The fourth stage of `00c113b0`'s passes: its +8 is the `float` at `011f9534`, its +0xc
/// is 1, then `SetupDefaultStage(stage, 3, 3, 0, 1)` at index 3.
fn copy_stage(e: &mut Engine, pass: u32, stage: u32) {
    e.call(CREATE_NEW_STAGE, &args![stage]);
    let made = e.mem.u32(stage);
    let value = e.mem.u32(STAGE_VALUE_011F9534);
    e.mem.set_u32(made + 8, value);
    let made = e.mem.u32(stage);
    e.mem.set_u32(made + 0xc, 1);
    let made = e.mem.u32(stage);
    e.call(SETUP_DEFAULT_STAGE, &args![made, 3u32, 3u32, 0u32, 1u32]);
    let made = e.mem.u32(stage);
    e.call(PASS_SET_STAGE, &args![pass, 3u32, made]);
}

// Translated from 00c113b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): sets up the passes at
/// `01202294` and `01202298` (each held while in use): when the pass's +0x18 is below 4,
/// three default stages `(i, 1, 2, 1)` and the copy stage at 3; its +0x5c and +0x44 become
/// the shader's +0xbc / +0x130 (first pass) and +0xc0 / +0x134 (second; counted); render
/// states `(0x13, 2, 1)` and `(0x14, 2, 1)`; then the words at `011fd150` / `011fd180` get
/// 0x480 and those at `011fc570` / `011fc5a0` get 8. The stage local and the held pass are
/// given up at the end (the second switch and the final release do not test the held pass
/// for null).
// Translation incomplete: 00c113b0 - the C++ exception frame (FS:[0] chain, unwind states 1 and 0 for the stage local and the held pass) is not translated.
pub fn fn_00c113b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, stage| {
        let stage = stage.addr();
        let mut held = e.mem.u32(0x0120_2294);
        hold_pass(e, held);
        if e.mem.u32(held + 0x18) < 4 {
            for index in 0..3 {
                numbered_stage(e, held, stage, index, [1, 2, 1]);
            }
            copy_stage(e, held, stage);
        }
        let shader = e.mem.u32(base + 0xbc);
        assign_counted(e, held + 0x5c, shader);
        let shader = e.mem.u32(base + 0x130);
        assign_counted(e, held + 0x44, shader);
        set_render_state(e, held, 0x13, 2, 1);
        set_render_state(e, held, 0x14, 2, 1);
        e.mem.set_u32(0x011f_d150, 0x480);
        e.mem.set_u32(0x011f_c570, 8);
        if held != e.mem.u32(0x0120_2298) {
            release_pass(e, held);
            held = e.mem.u32(0x0120_2298);
            hold_pass(e, held);
        }
        if e.mem.u32(held + 0x18) < 4 {
            for index in 0..3 {
                numbered_stage(e, held, stage, index, [1, 2, 1]);
            }
            copy_stage(e, held, stage);
        }
        let shader = e.mem.u32(base + 0xc0);
        assign_counted(e, held + 0x5c, shader);
        let shader = e.mem.u32(base + 0x134);
        assign_counted(e, held + 0x44, shader);
        set_render_state(e, held, 0x13, 2, 1);
        set_render_state(e, held, 0x14, 2, 1);
        e.mem.set_u32(0x011f_d180, 0x480);
        e.mem.set_u32(0x011f_c5a0, 8);
        let made = e.mem.u32(stage);
        release_stage(e, made);
        release_pass(e, held);
    });
}

// Translated from 00c11790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): sets up the pass at `0120229c`
/// (held while in use): when its +0x18 is below 3, three default stages `(i, 1, 2, 1)` added
/// at its +0x14; its +0x5c and +0x44 become the shader's +0xc4 and +0x138 (counted); render
/// states `(0x13, 9, 1)` and `(0x14, 1, 1)`; the words at `011fd1cc` and `011fc5ec` get
/// 0x400 and 0x20; the stage local and the held pass (not tested for null) are given up.
// Translation incomplete: 00c11790 - the C++ exception frame (FS:[0] chain, unwind states 1 and 0 for the stage local and the held pass) is not translated.
pub fn fn_00c11790(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, stage| {
        let stage = stage.addr();
        let held = e.mem.u32(0x0120_229c);
        hold_pass(e, held);
        if e.mem.u32(held + 0x18) < 3 {
            for index in 0..3 {
                let made = new_default_stage(e, stage, [index, 1, 2, 1]);
                let slot = e.mem.u32(held + 0x14);
                e.call(PASS_SET_STAGE, &args![held, slot, made]);
            }
        }
        let shader = e.mem.u32(base + 0xc4);
        assign_counted(e, held + 0x5c, shader);
        let shader = e.mem.u32(base + 0x138);
        assign_counted(e, held + 0x44, shader);
        set_render_state(e, held, 0x13, 9, 1);
        set_render_state(e, held, 0x14, 1, 1);
        e.mem.set_u32(0x011f_d1cc, 0x400);
        e.mem.set_u32(0x011f_c5ec, 0x20);
        let made = e.mem.u32(stage);
        release_stage(e, made);
        release_pass(e, held);
    });
}

// Translated from 00c0bb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`; `this` is not read): with `shader`'s
/// stage list (+0x24), the stage at +0x10 gets the word at `011f9530` at its +8 and the stage
/// at +0x14 gets the word `geometry`'s +0xb4 points to.
pub fn fn_00c0bb20(e: &mut Engine, _unused_this: Ptr, shader: Ptr, geometry: Ptr) {
    let stages = e.mem.u32(shader.addr() + 0x24);
    let stage = e.mem.u32(stages + 0x10);
    let value = e.mem.u32(0x011f_9530);
    e.mem.set_u32(stage + 8, value);
    let source = e.mem.u32(geometry.addr() + 0xb4);
    let stages = e.mem.u32(shader.addr() + 0x24);
    let stage = e.mem.u32(stages + 0x14);
    let value = e.mem.u32(source);
    e.mem.set_u32(stage + 8, value);
}

// Translated from 00c1a3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; a slot of the vtable `010bcc90`): answers the address
/// `012024b8` (the `GetRTTI` shape).
pub fn fn_00c1a3a0(_e: &mut Engine) -> Ptr {
    Ptr::new(0x0120_24b8)
}

// Translated from 00c1a480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): runs `NiTriStrips::NiTriStrips_ov2`
/// (`00a71cc0`) with the data, stores the vtable `010bcc90` and answers `this`.
pub fn fn_00c1a480(e: &mut Engine, this: Ptr, data: Ptr) -> Ptr {
    e.call(0x00a7_1cc0, &args![this, data]);
    e.mem.set_u32(this.addr(), 0x010b_cc90);
    this
}

// Translated from 00c1a3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0x10`; a slot of the vtable `010bcd8c`
/// refers to it): stores the centre `(x, y, z)` and `radius` at +0xc8 to +0xd4, makes the
/// world bound (+0x20) when there is none (16 bytes through slot +0x78, the centre at
/// `011f426c`, radius 0.0) and sets it to `(x, y, z, radius)` (the radius an `FLD`/`FSTP`
/// copy, a signalling NaN quieted).
pub fn fn_00c1a3b0(e: &mut Engine, this: Ptr, x: u32, y: u32, z: u32, radius: f32) {
    let base = this.addr();
    let radius = x87_quiet_bits(radius.to_bits());
    e.mem.set_u32(base + 0xd4, radius);
    e.mem.set_u32(base + 0xc8, x);
    e.mem.set_u32(base + 0xcc, y);
    e.mem.set_u32(base + 0xd0, z);
    if e.mem.u32(base + 0x20) == 0 {
        let block = memory_allocate(e, 0x10);
        e.vcall(base, 0x78, &args![block]);
        let bound = e.mem.u32(base + 0x20);
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(0x011f_426c + offset);
            e.mem.set_u32(bound + offset, word);
        }
        e.mem.set_u32(bound + 0xc, 0.0f32.to_bits());
    }
    let bound = e.mem.u32(base + 0x20);
    e.mem.set_u32(bound, x);
    e.mem.set_u32(bound + 4, y);
    e.mem.set_u32(bound + 0xc, radius);
    e.mem.set_u32(bound + 8, z);
}

// Translated from 00c1a310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 0xc`; a slot of the vtable at `010bc1c4`,
/// `this` and the first word are not read): fills the four words at `011fa2d0`: 1.0, 1.0
/// when `flag` is set (else the word at `011a9bd4`), the `float` +0x38 of `object` (or the
/// one at `01018f5c` without it, an `FLD`/`FSTP` copy) and the word at `011a9bdc`. The words
/// at `011a9bd0` and `011a9bd8` are read and overwritten in the frame.
pub fn fn_00c1a310(e: &mut Engine, _unused_this: Ptr, _unused_1: u32, flag: u32, object: Ptr) {
    let one = 1.0f32.to_bits();
    let _ = e.mem.u32(0x011a_9bd0);
    let _ = e.mem.u32(0x011a_9bd8);
    let fourth = e.mem.u32(0x011a_9bdc);
    let second = if flag != 0 {
        one
    } else {
        e.mem.u32(0x011a_9bd4)
    };
    let third = if object.addr() != 0 {
        e.mem.u32(object.addr() + 0x38)
    } else {
        e.mem.u32(0x0101_8f5c)
    };
    e.mem.set_u32(0x011f_a2d4, second);
    e.mem.set_u32(0x011f_a2d0, one);
    e.mem.set_u32(0x011f_a2d8, x87_quiet_bits(third));
    e.mem.set_u32(0x011f_a2dc, fourth);
}

/// The shader-constant callees of `00c0cd80`: `00bb9640` (`thiscall`, `RET 8`: stage,
/// geometry), `00b704b0` (`thiscall` on the geometry, a 16-bit answer; unit
/// `bsshaderlightingproperty.cpp`), `BSRenderedTexture::GetTexture` (`004bc320`, `RET 4`),
/// `NiD3DTextureStage::SetFilterMode` (`00e7def0`, `RET 4`), `00e7de90` (`RET 4`) and
/// `004a2020` (`thiscall`, `RET 4`, a byte answer; unit `bgsdecalmanager.cpp`).
const STAGE_SETUP_00BB9640: u32 = 0x00bb_9640;
const GEOMETRY_COUNT_00B704B0: u32 = 0x00b7_04b0;
const RENDERED_TEXTURE_GET_TEXTURE: u32 = 0x004b_c320;
const STAGE_SET_FILTER_MODE: u32 = 0x00e7_def0;
const STAGE_STEP_00E7DE90: u32 = 0x00e7_de90;
const GEOMETRY_FLAG_004A2020: u32 = 0x004a_2020;
/// The object at `0126f74c` whose +0x24 holds the stage array `00c0cd80` fills.
const STAGE_OWNER_0126F74C: u32 = 0x0126_f74c;

/// The first stage of `owner`'s stage array (+0x24) gets the geometry's +0xac texture at its
/// +8, then `00bb9640` on it.
fn first_stage_texture(e: &mut Engine, this: u32, owner: u32, geometry: u32) {
    let stages = e.mem.u32(owner + 0x24);
    let source = e.mem.u32(geometry + 0xac);
    let stage = e.mem.u32(stages);
    let texture = e.mem.u32(source);
    e.mem.set_u32(stage + 8, texture);
    e.call(STAGE_SETUP_00BB9640, &args![this, stage, geometry]);
}

// Translated from 00c0cd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed (no Ghidra function; a shader's slot, `thiscall`, `RET 8`: the pass number and
/// the geometry). Unless the number is in 0x1bd..=0x1e1 but outside 0x1c4..=0x1cb, or in
/// 0x2d8..=0x2ef: the first stage of the stage array (`0126f74c` +0x24) gets the geometry's
/// +0xac texture (`00bb9640`), the second (when there is one) its +0xb0 texture or the one
/// at `011f9510`; then, when the word at `011f91e4` is 0x1c4..=0x1c7, `00b704b0` is not
/// zero, the object `011f91e0` points to through +0xc has its byte +0xec and a rendered
/// texture at +0x10c and its byte +9 is set, the third stage gets that texture
/// (`GetTexture(0)`), its filter mode (1 when the word at `011f91b0` is 3) and `00e7de90(0)`,
/// and the `float` at `01201c7c` becomes 1.0 or 0.0 by `004a2020(8)`. For numbers 0x2e0,
/// 0x2e1, 0x2ec, 0x2ed and 0x1d3..=0x1d6 the first stage gets the +0xac texture again.
pub fn fn_00c0cd80(e: &mut Engine, this: Ptr, pass: u32, geometry: Ptr) {
    let this = this.addr();
    let geometry = geometry.addr();
    let owner = e.mem.u32(STAGE_OWNER_0126F74C);
    let skipped = (pass.wrapping_sub(0x1bd) <= 0x24
        && !(0x1c4..=0x1c7).contains(&pass)
        && pass.wrapping_sub(0x1c8) > 3)
        || (0x2d8..=0x2ef).contains(&pass);
    if !skipped {
        first_stage_texture(e, this, owner, geometry);
        let stages = e.mem.u32(owner + 0x24);
        let second = e.mem.u32(stages + 4);
        if second != 0 {
            let source = e.mem.u32(geometry + 0xb0);
            let mut texture = e.mem.u32(source);
            if texture == 0 {
                texture = e.mem.u32(0x011f_9510);
            }
            e.mem.set_u32(second + 8, texture);
            e.call(STAGE_SETUP_00BB9640, &args![this, second, geometry]);
        }
        'third: {
            if e.mem.u32(0x011f_91e4).wrapping_sub(0x1c4) > 3 {
                break 'third;
            }
            if e.call(GEOMETRY_COUNT_00B704B0, &args![geometry]).u16() == 0 {
                break 'third;
            }
            let holder = e.mem.u32(0x011f_91e0);
            let list = e.mem.u32(holder + 0xc);
            let object = e.mem.u32(list);
            if e.mem.u8(object + 0xec) == 0 {
                break 'third;
            }
            let rendered = e.mem.u32(object + 0x10c);
            if rendered == 0 || e.mem.u8(holder + 9) == 0 {
                break 'third;
            }
            let stages = e.mem.u32(owner + 0x24);
            let third = e.mem.u32(stages + 8);
            let texture = e
                .call(RENDERED_TEXTURE_GET_TEXTURE, &args![rendered, 0u32])
                .u32();
            e.mem.set_u32(third + 8, texture);
            let mode = u32::from(e.mem.u32(0x011f_91b0) == 3);
            e.call(STAGE_SET_FILTER_MODE, &args![third, mode]);
            e.call(STAGE_STEP_00E7DE90, &args![third, 0u32]);
            let flag = e
                .call(GEOMETRY_FLAG_004A2020, &args![geometry, 8u32])
                .bool();
            e.mem
                .set_u32(0x0120_1c7c, x87_store32(f64::from(u8::from(flag))));
        }
    }
    if matches!(pass, 0x2e0 | 0x2e1 | 0x2ec | 0x2ed) {
        first_stage_texture(e, this, owner, geometry);
    }
    if pass.wrapping_sub(0x1d3) <= 3 {
        first_stage_texture(e, this, owner, geometry);
    }
}

/// `00b79950` (run_00b79930 unit: the pass's +0x5c assignment), `00b80600` (the pass's +0x44
/// assignment, unit `bsshaderbloodsplatter.cpp` in the engine map) and `00b80040` (render
/// states 0x13 and 0x14 from its words), all `thiscall` on the pass.
const PASS_SET_00B79950: u32 = 0x00b7_9950;
const PASS_SET_00B80600: u32 = 0x00b8_0600;
const PASS_STEP_00B80040: u32 = 0x00b8_0040;

/// The held pass moved on to the pass at `global`: nothing when it is already held; else the
/// held one is released (not tested for null) and the new one held (counted when not null).
fn switch_pass(e: &mut Engine, held: u32, global: u32) -> u32 {
    if held == e.mem.u32(global) {
        return held;
    }
    release_pass(e, held);
    let next = e.mem.u32(global);
    hold_pass(e, next);
    next
}

/// A new default stage `(a, b, c, d)` in the local `stage` set at `index` of `pass`.
fn stage_at(e: &mut Engine, pass: u32, stage: u32, words: [u32; 4], index: u32) {
    let made = new_default_stage(e, stage, words);
    e.call(PASS_SET_STAGE, &args![pass, index, made]);
}

/// A new default stage `(a, b, c, d)` in the local `stage` added at `pass`'s +0x14 (read after
/// the stage is set up).
fn stage_next(e: &mut Engine, pass: u32, stage: u32, words: [u32; 4]) {
    let made = new_default_stage(e, stage, words);
    let index = e.mem.u32(pass + 0x14);
    e.call(PASS_SET_STAGE, &args![pass, index, made]);
}

/// `00a5f860` (`thiscall` on a pointer slot, `RET 4`, unnamed in the engine map): the
/// counted pointer assignment `00c0abd0` uses on +0x28.
const POINTER_ASSIGN_00A5F860: u32 = 0x00a5_f860;

/// The cases of `00c0abd0`'s switch (from its jump tables: `0xa..=0xf` through the table at
/// `00c0b7a8`, 0x178 apart, `0x17f..=0x2ed` through the byte map at `00c0b970` and the table
/// at `00c0b7c0`): the shader's field whose word is assigned, the pass global answered and
/// whether that global is read after the assignment. Other numbers answer 0.
fn case_00c0abd0(pass: u32) -> Option<(u32, u32, bool)> {
    match pass {
        0xa => Some((0x7c, 0x0120_0ec4, false)),
        0xb => Some((0x8c, 0x0120_0ec8, false)),
        0xc => Some((0x84, 0x0120_0ecc, false)),
        0xd => Some((0x94, 0x0120_0ed0, false)),
        0xe => Some((0x7c, 0x0120_0ed4, false)),
        0xf => Some((0x8c, 0x0120_0ed8, false)),
        0x178 => Some((0x7c, 0x0120_0ec0, false)),
        0x17f => Some((0x7c, 0x0120_0edc, false)),
        0x180 => Some((0x8c, 0x0120_0ee0, false)),
        0x181 => Some((0x84, 0x0120_0ee4, false)),
        0x182 => Some((0x7c, 0x0120_0ee8, false)),
        0x183 => Some((0x7c, 0x0120_0eec, false)),
        0x184 => Some((0x7c, 0x0120_0ef0, false)),
        0x185 => Some((0x8c, 0x0120_0ef4, false)),
        0x186 => Some((0x7c, 0x0120_0ef8, false)),
        0x187 => Some((0x8c, 0x0120_0efc, false)),
        0x188 => Some((0x84, 0x0120_0f00, false)),
        0x189 => Some((0x8c, 0x0120_0f04, false)),
        0x18a => Some((0x84, 0x0120_0f08, false)),
        0x18b => Some((0x84, 0x0120_0f0c, false)),
        0x18c => Some((0x84, 0x0120_0f10, false)),
        0x18d => Some((0x94, 0x0120_0f14, false)),
        0x18e => Some((0x7c, 0x0120_0f18, false)),
        0x18f => Some((0x8c, 0x0120_0f1c, false)),
        0x190 => Some((0x84, 0x0120_0f20, false)),
        0x191 => Some((0x7c, 0x0120_0f24, false)),
        0x192 => Some((0x7c, 0x0120_0f28, false)),
        0x193 => Some((0x7c, 0x0120_0f2c, false)),
        0x194 => Some((0x8c, 0x0120_0f30, false)),
        0x195 => Some((0x7c, 0x0120_0f34, false)),
        0x196 => Some((0x8c, 0x0120_0f38, false)),
        0x197 => Some((0x84, 0x0120_0f3c, false)),
        0x198 => Some((0x94, 0x0120_0f40, false)),
        0x199 => Some((0x84, 0x0120_0f44, false)),
        0x19a => Some((0x84, 0x0120_0f48, false)),
        0x19b => Some((0x84, 0x0120_0f4c, false)),
        0x19c => Some((0x94, 0x0120_0f50, false)),
        0x19d => Some((0x7c, 0x0120_0f54, false)),
        0x19e => Some((0x8c, 0x0120_0f58, false)),
        0x19f => Some((0x84, 0x0120_0f5c, false)),
        0x1a0 => Some((0x7c, 0x0120_0f60, false)),
        0x1a1 => Some((0x7c, 0x0120_0f64, false)),
        0x1a2 => Some((0x7c, 0x0120_0f68, false)),
        0x1a3 => Some((0x8c, 0x0120_0f6c, false)),
        0x1a4 => Some((0x7c, 0x0120_0f70, false)),
        0x1a5 => Some((0x8c, 0x0120_0f74, false)),
        0x1a6 => Some((0x84, 0x0120_0f78, false)),
        0x1a7 => Some((0x94, 0x0120_0f7c, false)),
        0x1a8 => Some((0x84, 0x0120_0f80, false)),
        0x1a9 => Some((0x84, 0x0120_0f84, false)),
        0x1aa => Some((0x84, 0x0120_0f88, false)),
        0x1ab => Some((0x94, 0x0120_0f8c, false)),
        0x1ac => Some((0x7c, 0x0120_0f90, false)),
        0x1ad => Some((0x8c, 0x0120_0f94, false)),
        0x1ae => Some((0x84, 0x0120_0f98, false)),
        0x1af => Some((0x7c, 0x0120_0f9c, false)),
        0x1b0 => Some((0x7c, 0x0120_0fa0, false)),
        0x1b1 => Some((0x7c, 0x0120_0fa4, false)),
        0x1b2 => Some((0x8c, 0x0120_0fa8, false)),
        0x1b3 => Some((0x7c, 0x0120_0fac, false)),
        0x1b4 => Some((0x8c, 0x0120_0fb0, false)),
        0x1b5 => Some((0x84, 0x0120_0fb4, false)),
        0x1b6 => Some((0x94, 0x0120_0fb8, false)),
        0x1b7 => Some((0x84, 0x0120_0fbc, false)),
        0x1b8 => Some((0x84, 0x0120_0fc0, false)),
        0x1b9 => Some((0x84, 0x0120_0fc4, false)),
        0x1ba => Some((0x94, 0x0120_0fc8, false)),
        0x1bb => Some((0x7c, 0x0120_0fcc, false)),
        0x1bc => Some((0x84, 0x0120_0fd0, false)),
        0x1bd => Some((0x7c, 0x0120_0fd4, false)),
        0x1be => Some((0x7c, 0x0120_0fd8, false)),
        0x1bf => Some((0x84, 0x0120_0fdc, false)),
        0x1c0 => Some((0x84, 0x0120_0fe0, false)),
        0x1c1 => Some((0x8c, 0x0120_0fe4, false)),
        0x1c2 => Some((0x94, 0x0120_0fe8, false)),
        0x1c3 => Some((0x7c, 0x0120_0fec, false)),
        0x1c4 => Some((0x7c, 0x0120_0ff0, false)),
        0x1c5 => Some((0x84, 0x0120_0ff4, false)),
        0x1c6 => Some((0x8c, 0x0120_0ff8, false)),
        0x1c7 => Some((0x8c, 0x0120_0ffc, false)),
        0x1c8 => Some((0x7c, 0x0120_1000, true)),
        0x1c9 => Some((0x84, 0x0120_1004, true)),
        0x1ca => Some((0x8c, 0x0120_1008, false)),
        0x1cb => Some((0x94, 0x0120_100c, false)),
        0x1cc => Some((0x7c, 0x0120_1010, true)),
        0x1cd => Some((0x84, 0x0120_1010, true)),
        0x1ce..=0x1cf => Some((0x8c, 0x0120_1014, false)),
        0x1d0 => Some((0x7c, 0x0120_1018, true)),
        0x1d1 => Some((0x7c, 0x0120_101c, true)),
        0x1d2 => Some((0x8c, 0x0120_1020, false)),
        0x1d3 => Some((0x7c, 0x0120_1024, false)),
        0x1d4 => Some((0x84, 0x0120_1028, false)),
        0x1d5 => Some((0x8c, 0x0120_102c, false)),
        0x1d6 => Some((0x94, 0x0120_1030, false)),
        0x1d7 => Some((0x7c, 0x0120_1034, true)),
        0x1d8 => Some((0x84, 0x0120_1038, true)),
        0x1d9 => Some((0x8c, 0x0120_103c, false)),
        0x1da => Some((0x94, 0x0120_1040, false)),
        0x1db => Some((0x7c, 0x0120_1044, false)),
        0x1dc => Some((0x8c, 0x0120_1048, false)),
        0x1dd => Some((0x7c, 0x0120_104c, true)),
        0x1de => Some((0x8c, 0x0120_1050, false)),
        0x1df => Some((0x7c, 0x0120_1054, true)),
        0x1e0 => Some((0x7c, 0x0120_1058, false)),
        0x1e1 => Some((0x8c, 0x0120_105c, false)),
        0x2da => Some((0x7c, 0x0120_1060, false)),
        0x2db => Some((0x8c, 0x0120_1064, false)),
        0x2e0 => Some((0x7c, 0x0120_1068, false)),
        0x2e1 => Some((0x8c, 0x0120_106c, false)),
        0x2e6 => Some((0x84, 0x0120_1070, false)),
        0x2e7 => Some((0x94, 0x0120_1074, false)),
        0x2ec => Some((0x84, 0x0120_1078, false)),
        0x2ed => Some((0x94, 0x0120_107c, false)),
        _ => None,
    }
}

// Translated from 00c0abd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`: the pass number): for a number the
/// switch knows, the shader's word at the case's offset (+0x7c, +0x84, +0x8c, +0x94, ...) is
/// assigned to +0x28 (`00a5f860`) and the case's pass global is answered (read before or
/// after the assignment as the exe does); 0 for any other number.
pub fn fn_00c0abd0(e: &mut Engine, this: Ptr, pass: u32) -> u32 {
    let Some((offset, global, after)) = case_00c0abd0(pass) else {
        return 0;
    };
    let base = this.addr();
    let word = e.mem.u32(base + offset);
    let before = if after { 0 } else { e.mem.u32(global) };
    e.call(POINTER_ASSIGN_00A5F860, &args![base + 0x28, word]);
    if after {
        e.mem.u32(global)
    } else {
        before
    }
}

// Translated from 00c0d200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a shader's slot, referenced from
/// `010bb904`): sets up 8 shader passes in turn, read from the pass globals `01202230` to `0120224c`
/// in the body's order, each held while in use (the held pass switched with `switch_pass`):
/// while a pass has fewer stages (+0x18)
/// than the limit, default stages are added (`stage_at` / `stage_next`, some with their
/// +8 and +0xc set first); the shader's programs become the pass's +0x5c and +0x44
/// (counted), its render states and the globals the exe writes follow, in the exe's order.
/// The stage local and the last pass are given up at the end.
// Translation incomplete: 00c0d200 - the C++ exception frame (FS:[0] chain, unwind states for the stage local and the held pass) is not translated.
pub fn fn_00c0d200(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, stage| {
        let stage = stage.addr();
        let mut held: u32;
        let word_1 = e.mem.u32(0x0120_2230);
        held = word_1;
        hold_pass(e, held);
        let word_2 = e.mem.u32(held + 0x18);
        if word_2 < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_11 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_11, 4u32, 3u32, 0u32, 1u32]);
            let made_12 = e.mem.u32(stage);
            let word_13 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_12 + 0x8, word_13);
            let made_14 = e.mem.u32(stage);
            e.mem.set_u32(made_14 + 0xc, 1u32);
            let made_15 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_15]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_20 = e.mem.u32(base + 0xac);
        assign_counted(e, held + 0x5c, word_20);
        let word_21 = e.mem.u32(base + 0x110);
        assign_counted(e, held + 0x44, word_21);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cfe4, 0x780u32);
        e.mem.set_u32(0x011f_c404, 0x2cu32);
        held = switch_pass(e, held, 0x0120_2234);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_30 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_30, 4u32, 3u32, 0u32, 1u32]);
            let made_31 = e.mem.u32(stage);
            let word_32 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_31 + 0x8, word_32);
            let made_33 = e.mem.u32(stage);
            e.mem.set_u32(made_33 + 0xc, 1u32);
            let made_34 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_34]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_39 = e.mem.u32(base + 0xac);
        assign_counted(e, held + 0x5c, word_39);
        let word_40 = e.mem.u32(base + 0x114);
        assign_counted(e, held + 0x44, word_40);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cfe8, 0x780u32);
        e.mem.set_u32(0x011f_c408, 0x3cu32);
        held = switch_pass(e, held, 0x0120_2238);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_49 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_49, 4u32, 3u32, 0u32, 1u32]);
            let made_50 = e.mem.u32(stage);
            let word_51 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_50 + 0x8, word_51);
            let made_52 = e.mem.u32(stage);
            e.mem.set_u32(made_52 + 0xc, 1u32);
            let made_53 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_53]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_58 = e.mem.u32(base + 0xb0);
        assign_counted(e, held + 0x5c, word_58);
        let word_59 = e.mem.u32(base + 0x118);
        assign_counted(e, held + 0x44, word_59);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d008, 0x7f0u32);
        e.mem.set_u32(0x011f_c428, 0x2cu32);
        held = switch_pass(e, held, 0x0120_223c);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_68 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_68, 4u32, 3u32, 0u32, 1u32]);
            let made_69 = e.mem.u32(stage);
            let word_70 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_69 + 0x8, word_70);
            let made_71 = e.mem.u32(stage);
            e.mem.set_u32(made_71 + 0xc, 1u32);
            let made_72 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_72]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_77 = e.mem.u32(base + 0xb0);
        assign_counted(e, held + 0x5c, word_77);
        let word_78 = e.mem.u32(base + 0x11c);
        assign_counted(e, held + 0x44, word_78);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d00c, 0x7f0u32);
        e.mem.set_u32(0x011f_c42c, 0x3cu32);
        held = switch_pass(e, held, 0x0120_2240);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_87 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_87, 4u32, 3u32, 0u32, 1u32]);
            let made_88 = e.mem.u32(stage);
            let word_89 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_88 + 0x8, word_89);
            let made_90 = e.mem.u32(stage);
            e.mem.set_u32(made_90 + 0xc, 1u32);
            let made_91 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_91]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_96 = e.mem.u32(base + 0xb4);
        assign_counted(e, held + 0x5c, word_96);
        let word_97 = e.mem.u32(base + 0x120);
        assign_counted(e, held + 0x44, word_97);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d02c, 0x780u32);
        e.mem.set_u32(0x011f_c44c, 0x2cu32);
        held = switch_pass(e, held, 0x0120_2244);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_106 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_106, 4u32, 3u32, 0u32, 1u32],
            );
            let made_107 = e.mem.u32(stage);
            let word_108 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_107 + 0x8, word_108);
            let made_109 = e.mem.u32(stage);
            e.mem.set_u32(made_109 + 0xc, 1u32);
            let made_110 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_110]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_115 = e.mem.u32(base + 0xb4);
        assign_counted(e, held + 0x5c, word_115);
        let word_116 = e.mem.u32(base + 0x124);
        assign_counted(e, held + 0x44, word_116);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d030, 0x780u32);
        e.mem.set_u32(0x011f_c450, 0x3cu32);
        held = switch_pass(e, held, 0x0120_2248);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_125 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_125, 4u32, 3u32, 0u32, 1u32],
            );
            let made_126 = e.mem.u32(stage);
            let word_127 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_126 + 0x8, word_127);
            let made_128 = e.mem.u32(stage);
            e.mem.set_u32(made_128 + 0xc, 1u32);
            let made_129 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_129]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_134 = e.mem.u32(base + 0xb8);
        assign_counted(e, held + 0x5c, word_134);
        let word_135 = e.mem.u32(base + 0x128);
        assign_counted(e, held + 0x44, word_135);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d050, 0x7f0u32);
        e.mem.set_u32(0x011f_c470, 0x2cu32);
        held = switch_pass(e, held, 0x0120_224c);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_144 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_144, 4u32, 3u32, 0u32, 1u32],
            );
            let made_145 = e.mem.u32(stage);
            let word_146 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_145 + 0x8, word_146);
            let made_147 = e.mem.u32(stage);
            e.mem.set_u32(made_147 + 0xc, 1u32);
            let made_148 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_148]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_153 = e.mem.u32(base + 0xb8);
        assign_counted(e, held + 0x5c, word_153);
        let word_154 = e.mem.u32(base + 0x12c);
        assign_counted(e, held + 0x44, word_154);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let made_155 = e.mem.u32(stage);
        e.mem.set_u32(0x011f_d054, 0x7f0u32);
        e.mem.set_u32(0x011f_c474, 0x3cu32);
        release_stage(e, made_155);
        release_pass(e, held);
    });
}

// Translated from 00c0e630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a shader's slot, referenced from
/// `010bb908`): sets up 9 shader passes in turn, read from the pass globals `01202250` to `01202270`
/// in the body's order, each held while in use (the held pass switched with `switch_pass`):
/// while a pass has fewer stages (+0x18)
/// than the limit, default stages are added (`stage_at` / `stage_next`, some with their
/// +8 and +0xc set first); the shader's programs become the pass's +0x5c and +0x44
/// (counted), its render states and the globals the exe writes follow, in the exe's order.
/// The stage local and the last pass are given up at the end.
// Translation incomplete: 00c0e630 - the C++ exception frame (FS:[0] chain, unwind states for the stage local and the held pass) is not translated.
pub fn fn_00c0e630(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, stage| {
        let stage = stage.addr();
        let mut held: u32;
        let word_1 = e.mem.u32(0x0120_2250);
        held = word_1;
        hold_pass(e, held);
        let word_2 = e.mem.u32(held + 0x18);
        if word_2 < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_13 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_13, 5u32, 3u32, 0u32, 1u32]);
            let made_14 = e.mem.u32(stage);
            let word_15 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_14 + 0x8, word_15);
            let made_16 = e.mem.u32(stage);
            e.mem.set_u32(made_16 + 0xc, 1u32);
            let made_17 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_17]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_22 = e.mem.u32(base + 0x8c);
        assign_counted(e, held + 0x5c, word_22);
        let word_23 = e.mem.u32(base + 0xdc);
        assign_counted(e, held + 0x44, word_23);
        e.mem.set_u32(0x011f_d084, 0x780u32);
        e.mem.set_u32(0x011f_c4a4, 0x2cu32);
        held = switch_pass(e, held, 0x0120_2254);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_34 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_34, 5u32, 3u32, 0u32, 1u32]);
            let made_35 = e.mem.u32(stage);
            let word_36 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_35 + 0x8, word_36);
            let made_37 = e.mem.u32(stage);
            e.mem.set_u32(made_37 + 0xc, 1u32);
            let made_38 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_38]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_43 = e.mem.u32(base + 0x8c);
        assign_counted(e, held + 0x5c, word_43);
        let word_44 = e.mem.u32(base + 0xe0);
        assign_counted(e, held + 0x44, word_44);
        e.mem.set_u32(0x011f_d088, 0x780u32);
        e.mem.set_u32(0x011f_c4a8, 0x2cu32);
        held = switch_pass(e, held, 0x0120_2258);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_55 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_55, 5u32, 3u32, 0u32, 1u32]);
            let made_56 = e.mem.u32(stage);
            let word_57 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_56 + 0x8, word_57);
            let made_58 = e.mem.u32(stage);
            e.mem.set_u32(made_58 + 0xc, 1u32);
            let made_59 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_59]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_64 = e.mem.u32(base + 0x8c);
        assign_counted(e, held + 0x5c, word_64);
        let word_65 = e.mem.u32(base + 0xe4);
        assign_counted(e, held + 0x44, word_65);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d08c, 0x780u32);
        e.mem.set_u32(0x011f_c4ac, 0x3cu32);
        held = switch_pass(e, held, 0x0120_225c);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_76 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_76, 5u32, 3u32, 0u32, 1u32]);
            let made_77 = e.mem.u32(stage);
            let word_78 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_77 + 0x8, word_78);
            let made_79 = e.mem.u32(stage);
            e.mem.set_u32(made_79 + 0xc, 1u32);
            let made_80 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_80]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_85 = e.mem.u32(base + 0x90);
        assign_counted(e, held + 0x5c, word_85);
        let word_86 = e.mem.u32(base + 0xe8);
        assign_counted(e, held + 0x44, word_86);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d0b8, 0x7f0u32);
        e.mem.set_u32(0x011f_c4d8, 0x2cu32);
        held = switch_pass(e, held, 0x0120_2260);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_97 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_97, 5u32, 3u32, 0u32, 1u32]);
            let made_98 = e.mem.u32(stage);
            let word_99 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_98 + 0x8, word_99);
            let made_100 = e.mem.u32(stage);
            e.mem.set_u32(made_100 + 0xc, 1u32);
            let made_101 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_101]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_106 = e.mem.u32(base + 0x90);
        assign_counted(e, held + 0x5c, word_106);
        let word_107 = e.mem.u32(base + 0xec);
        assign_counted(e, held + 0x44, word_107);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d0bc, 0x7f0u32);
        e.mem.set_u32(0x011f_c4dc, 0x3cu32);
        held = switch_pass(e, held, 0x0120_2264);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_118 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_118, 5u32, 3u32, 0u32, 1u32],
            );
            let made_119 = e.mem.u32(stage);
            let word_120 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_119 + 0x8, word_120);
            let made_121 = e.mem.u32(stage);
            e.mem.set_u32(made_121 + 0xc, 1u32);
            let made_122 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_122]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_127 = e.mem.u32(base + 0x94);
        assign_counted(e, held + 0x5c, word_127);
        let word_128 = e.mem.u32(base + 0xf0);
        assign_counted(e, held + 0x44, word_128);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d0e4, 0x780u32);
        e.mem.set_u32(0x011f_c504, 0x2cu32);
        held = switch_pass(e, held, 0x0120_2268);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_139 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_139, 5u32, 3u32, 0u32, 1u32],
            );
            let made_140 = e.mem.u32(stage);
            let word_141 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_140 + 0x8, word_141);
            let made_142 = e.mem.u32(stage);
            e.mem.set_u32(made_142 + 0xc, 1u32);
            let made_143 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_143]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_148 = e.mem.u32(base + 0x94);
        assign_counted(e, held + 0x5c, word_148);
        let word_149 = e.mem.u32(base + 0xf4);
        assign_counted(e, held + 0x44, word_149);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d0e8, 0x780u32);
        e.mem.set_u32(0x011f_c508, 0x3cu32);
        held = switch_pass(e, held, 0x0120_226c);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_160 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_160, 5u32, 3u32, 0u32, 1u32],
            );
            let made_161 = e.mem.u32(stage);
            let word_162 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_161 + 0x8, word_162);
            let made_163 = e.mem.u32(stage);
            e.mem.set_u32(made_163 + 0xc, 1u32);
            let made_164 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_164]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_169 = e.mem.u32(base + 0x98);
        assign_counted(e, held + 0x5c, word_169);
        let word_170 = e.mem.u32(base + 0xf8);
        assign_counted(e, held + 0x44, word_170);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d110, 0x7f0u32);
        e.mem.set_u32(0x011f_c530, 0x2cu32);
        held = switch_pass(e, held, 0x0120_2270);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_181 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_181, 5u32, 3u32, 0u32, 1u32],
            );
            let made_182 = e.mem.u32(stage);
            let word_183 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_182 + 0x8, word_183);
            let made_184 = e.mem.u32(stage);
            e.mem.set_u32(made_184 + 0xc, 1u32);
            let made_185 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_185]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_190 = e.mem.u32(base + 0x98);
        assign_counted(e, held + 0x5c, word_190);
        let word_191 = e.mem.u32(base + 0xfc);
        assign_counted(e, held + 0x44, word_191);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let made_192 = e.mem.u32(stage);
        e.mem.set_u32(0x011f_d114, 0x7f0u32);
        e.mem.set_u32(0x011f_c534, 0x3cu32);
        release_stage(e, made_192);
        release_pass(e, held);
    });
}

// Translated from 00c0fdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a shader's slot, referenced from
/// `010bb90c`): sets up 8 shader passes in turn, read from the pass globals `01202274` to `01202290`
/// in the body's order, each held while in use (the held pass switched with `switch_pass`):
/// while a pass has fewer stages (+0x18)
/// than the limit, default stages are added (`stage_at` / `stage_next`, some with their
/// +8 and +0xc set first); the shader's programs become the pass's +0x5c and +0x44
/// (counted), its render states and the globals the exe writes follow, in the exe's order.
/// The stage local and the last pass are given up at the end.
// Translation incomplete: 00c0fdc0 - the C++ exception frame (FS:[0] chain, unwind states for the stage local and the held pass) is not translated.
pub fn fn_00c0fdc0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, stage| {
        let stage = stage.addr();
        let mut held: u32;
        let word_1 = e.mem.u32(0x0120_2274);
        held = word_1;
        hold_pass(e, held);
        let word_2 = e.mem.u32(held + 0x18);
        if word_2 < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_13 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_13, 5u32, 3u32, 0u32, 1u32]);
            let made_14 = e.mem.u32(stage);
            let word_15 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_14 + 0x8, word_15);
            let made_16 = e.mem.u32(stage);
            e.mem.set_u32(made_16 + 0xc, 1u32);
            let made_17 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_17]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_22 = e.mem.u32(base + 0x9c);
        assign_counted(e, held + 0x5c, word_22);
        let word_23 = e.mem.u32(base + 0x100);
        assign_counted(e, held + 0x44, word_23);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cee4, 0x780u32);
        e.mem.set_u32(0x011f_c304, 0x2cu32);
        held = switch_pass(e, held, 0x0120_2278);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_34 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_34, 5u32, 3u32, 0u32, 1u32]);
            let made_35 = e.mem.u32(stage);
            let word_36 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_35 + 0x8, word_36);
            let made_37 = e.mem.u32(stage);
            e.mem.set_u32(made_37 + 0xc, 1u32);
            let made_38 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_38]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_43 = e.mem.u32(base + 0x9c);
        assign_counted(e, held + 0x5c, word_43);
        let word_44 = e.mem.u32(base + 0x104);
        assign_counted(e, held + 0x44, word_44);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cee8, 0x780u32);
        e.mem.set_u32(0x011f_c308, 0x3cu32);
        held = switch_pass(e, held, 0x0120_227c);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_55 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_55, 5u32, 3u32, 0u32, 1u32]);
            let made_56 = e.mem.u32(stage);
            let word_57 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_56 + 0x8, word_57);
            let made_58 = e.mem.u32(stage);
            e.mem.set_u32(made_58 + 0xc, 1u32);
            let made_59 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_59]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_64 = e.mem.u32(base + 0xa0);
        assign_counted(e, held + 0x5c, word_64);
        let word_65 = e.mem.u32(base + 0x108);
        assign_counted(e, held + 0x44, word_65);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cf10, 0x7f0u32);
        e.mem.set_u32(0x011f_c330, 0x2cu32);
        held = switch_pass(e, held, 0x0120_2280);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_76 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_76, 5u32, 3u32, 0u32, 1u32]);
            let made_77 = e.mem.u32(stage);
            let word_78 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_77 + 0x8, word_78);
            let made_79 = e.mem.u32(stage);
            e.mem.set_u32(made_79 + 0xc, 1u32);
            let made_80 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_80]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_85 = e.mem.u32(base + 0xa0);
        assign_counted(e, held + 0x5c, word_85);
        let word_86 = e.mem.u32(base + 0x10c);
        assign_counted(e, held + 0x44, word_86);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cf14, 0x7f0u32);
        e.mem.set_u32(0x011f_c334, 0x3cu32);
        held = switch_pass(e, held, 0x0120_2284);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_97 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_97, 5u32, 3u32, 0u32, 1u32]);
            let made_98 = e.mem.u32(stage);
            let word_99 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_98 + 0x8, word_99);
            let made_100 = e.mem.u32(stage);
            e.mem.set_u32(made_100 + 0xc, 1u32);
            let made_101 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_101]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_106 = e.mem.u32(base + 0xa4);
        assign_counted(e, held + 0x5c, word_106);
        let word_107 = e.mem.u32(base + 0x150);
        assign_counted(e, held + 0x44, word_107);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cf3c, 0x780u32);
        e.mem.set_u32(0x011f_c35c, 0x2cu32);
        held = switch_pass(e, held, 0x0120_2288);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_118 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_118, 5u32, 3u32, 0u32, 1u32],
            );
            let made_119 = e.mem.u32(stage);
            let word_120 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_119 + 0x8, word_120);
            let made_121 = e.mem.u32(stage);
            e.mem.set_u32(made_121 + 0xc, 1u32);
            let made_122 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_122]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_127 = e.mem.u32(base + 0xa4);
        assign_counted(e, held + 0x5c, word_127);
        let word_128 = e.mem.u32(base + 0x154);
        assign_counted(e, held + 0x44, word_128);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cf40, 0x780u32);
        e.mem.set_u32(0x011f_c360, 0x3cu32);
        held = switch_pass(e, held, 0x0120_228c);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_139 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_139, 5u32, 3u32, 0u32, 1u32],
            );
            let made_140 = e.mem.u32(stage);
            let word_141 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_140 + 0x8, word_141);
            let made_142 = e.mem.u32(stage);
            e.mem.set_u32(made_142 + 0xc, 1u32);
            let made_143 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_143]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_148 = e.mem.u32(base + 0xa8);
        assign_counted(e, held + 0x5c, word_148);
        let word_149 = e.mem.u32(base + 0x158);
        assign_counted(e, held + 0x44, word_149);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cf68, 0x7f0u32);
        e.mem.set_u32(0x011f_c388, 0x2cu32);
        held = switch_pass(e, held, 0x0120_2290);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_160 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_160, 5u32, 3u32, 0u32, 1u32],
            );
            let made_161 = e.mem.u32(stage);
            let word_162 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_161 + 0x8, word_162);
            let made_163 = e.mem.u32(stage);
            e.mem.set_u32(made_163 + 0xc, 1u32);
            let made_164 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_164]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_169 = e.mem.u32(base + 0xa8);
        assign_counted(e, held + 0x5c, word_169);
        let word_170 = e.mem.u32(base + 0x15c);
        assign_counted(e, held + 0x44, word_170);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let made_171 = e.mem.u32(stage);
        e.mem.set_u32(0x011f_cf6c, 0x7f0u32);
        e.mem.set_u32(0x011f_c38c, 0x3cu32);
        release_stage(e, made_171);
        release_pass(e, held);
    });
}

// Translated from 00c11980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a shader's slot, referenced from
/// `010bb918`): sets up 5 shader passes in turn, read from the pass globals `012022a0` to `012022b0`
/// in the body's order, each held while in use (the held pass switched with `switch_pass`):
/// while a pass has fewer stages (+0x18)
/// than the limit, default stages are added (`stage_at` / `stage_next`, some with their
/// +8 and +0xc set first); the shader's programs become the pass's +0x5c and +0x44
/// (counted), its render states and the globals the exe writes follow, in the exe's order.
/// The stage local and the last pass are given up at the end.
// Translation incomplete: 00c11980 - the C++ exception frame (FS:[0] chain, unwind states for the stage local and the held pass) is not translated.
pub fn fn_00c11980(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, stage| {
        let stage = stage.addr();
        let mut held: u32;
        let word_1 = e.mem.u32(0x0120_22a0);
        held = word_1;
        hold_pass(e, held);
        let word_2 = e.mem.u32(held + 0x18);
        if word_2 < 6u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [1u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [2u32, 1u32, 2u32, 1u32]);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_12 = e.mem.u32(stage);
            let word_13 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_12 + 0x8, word_13);
            let made_14 = e.mem.u32(stage);
            e.mem.set_u32(made_14 + 0xc, 1u32);
            let made_15 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_15, 3u32, 3u32, 0u32, 1u32]);
            let made_16 = e.mem.u32(stage);
            let word_17 = e.mem.u32(held + 0x14);
            e.call(PASS_SET_STAGE, &args![held, word_17, made_16]);
            stage_next(e, held, stage, [4u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [5u32, 1u32, 2u32, 1u32]);
        }
        let word_24 = e.mem.u32(base + 0xc8);
        assign_counted(e, held + 0x5c, word_24);
        let word_25 = e.mem.u32(base + 0x13c);
        assign_counted(e, held + 0x44, word_25);
        set_render_state(e, held, 0x13u32, 2u32, 1u32);
        set_render_state(e, held, 0x14u32, 2u32, 1u32);
        e.mem.set_u32(0x011f_d1f0, 0x480u32);
        e.mem.set_u32(0x011f_c610, 0x28u32);
        held = switch_pass(e, held, 0x0120_22a4);
        if e.mem.u32(held + 0x18) < 6u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [1u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [2u32, 1u32, 2u32, 1u32]);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_35 = e.mem.u32(stage);
            let word_36 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_35 + 0x8, word_36);
            let made_37 = e.mem.u32(stage);
            e.mem.set_u32(made_37 + 0xc, 1u32);
            let made_38 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_38, 3u32, 3u32, 0u32, 1u32]);
            let made_39 = e.mem.u32(stage);
            let word_40 = e.mem.u32(held + 0x14);
            e.call(PASS_SET_STAGE, &args![held, word_40, made_39]);
            stage_next(e, held, stage, [4u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [5u32, 1u32, 2u32, 1u32]);
        }
        let word_47 = e.mem.u32(base + 0xcc);
        assign_counted(e, held + 0x5c, word_47);
        let word_48 = e.mem.u32(base + 0x140);
        assign_counted(e, held + 0x44, word_48);
        set_render_state(e, held, 0x13u32, 2u32, 1u32);
        set_render_state(e, held, 0x14u32, 2u32, 1u32);
        e.mem.set_u32(0x011f_d208, 0x4f0u32);
        e.mem.set_u32(0x011f_c628, 0x28u32);
        held = switch_pass(e, held, 0x0120_22a8);
        if e.mem.u32(held + 0x18) < 6u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [1u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [2u32, 1u32, 2u32, 1u32]);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_58 = e.mem.u32(stage);
            let word_59 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_58 + 0x8, word_59);
            let made_60 = e.mem.u32(stage);
            e.mem.set_u32(made_60 + 0xc, 1u32);
            let made_61 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_61, 3u32, 3u32, 0u32, 1u32]);
            let made_62 = e.mem.u32(stage);
            let word_63 = e.mem.u32(held + 0x14);
            e.call(PASS_SET_STAGE, &args![held, word_63, made_62]);
            stage_next(e, held, stage, [4u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [5u32, 1u32, 2u32, 1u32]);
        }
        let word_70 = e.mem.u32(base + 0xd0);
        assign_counted(e, held + 0x5c, word_70);
        let word_71 = e.mem.u32(base + 0x144);
        assign_counted(e, held + 0x44, word_71);
        set_render_state(e, held, 0x13u32, 2u32, 1u32);
        set_render_state(e, held, 0x14u32, 2u32, 1u32);
        let word_72 = e.mem.u32(0x011f_c638);
        e.mem.set_u32(0x011f_d220, 0x480u32);
        e.mem.set_u32(0x011f_c640, word_72);
        held = switch_pass(e, held, 0x0120_22ac);
        if e.mem.u32(held + 0x18) < 6u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [1u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [2u32, 1u32, 2u32, 1u32]);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_82 = e.mem.u32(stage);
            let word_83 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_82 + 0x8, word_83);
            let made_84 = e.mem.u32(stage);
            e.mem.set_u32(made_84 + 0xc, 1u32);
            let made_85 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_85, 3u32, 3u32, 0u32, 1u32]);
            let made_86 = e.mem.u32(stage);
            let word_87 = e.mem.u32(held + 0x14);
            e.call(PASS_SET_STAGE, &args![held, word_87, made_86]);
            stage_next(e, held, stage, [4u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [5u32, 1u32, 2u32, 1u32]);
        }
        let word_94 = e.mem.u32(base + 0xd4);
        assign_counted(e, held + 0x5c, word_94);
        let word_95 = e.mem.u32(base + 0x148);
        assign_counted(e, held + 0x44, word_95);
        set_render_state(e, held, 0x13u32, 2u32, 1u32);
        set_render_state(e, held, 0x14u32, 2u32, 1u32);
        let word_96 = e.mem.u32(0x011f_c650);
        e.mem.set_u32(0x011f_d238, 0x480u32);
        e.mem.set_u32(0x011f_c658, word_96);
        held = switch_pass(e, held, 0x0120_22b0);
        if e.mem.u32(held + 0x18) < 6u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [1u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [2u32, 1u32, 2u32, 1u32]);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_106 = e.mem.u32(stage);
            let word_107 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_106 + 0x8, word_107);
            let made_108 = e.mem.u32(stage);
            e.mem.set_u32(made_108 + 0xc, 1u32);
            let made_109 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_109, 3u32, 3u32, 0u32, 1u32],
            );
            let made_110 = e.mem.u32(stage);
            let word_111 = e.mem.u32(held + 0x14);
            e.call(PASS_SET_STAGE, &args![held, word_111, made_110]);
            stage_next(e, held, stage, [4u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [5u32, 1u32, 2u32, 1u32]);
        }
        let word_118 = e.mem.u32(base + 0xd8);
        assign_counted(e, held + 0x5c, word_118);
        let word_119 = e.mem.u32(base + 0x14c);
        assign_counted(e, held + 0x44, word_119);
        set_render_state(e, held, 0x13u32, 2u32, 1u32);
        set_render_state(e, held, 0x14u32, 2u32, 1u32);
        let made_120 = e.mem.u32(stage);
        let word_121 = e.mem.u32(0x011f_c668);
        e.mem.set_u32(0x011f_d250, 0x480u32);
        e.mem.set_u32(0x011f_c670, word_121);
        release_stage(e, made_120);
        release_pass(e, held);
    });
}

// Translated from 00c124c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a shader's slot, referenced from
/// `010bbad8`): sets up 26 shader passes in turn, read from the pass globals `012022c0` to `01202324`
/// in the body's order, each held while in use (the held pass switched with `switch_pass`):
/// while a pass has fewer stages (+0x18)
/// than the limit, default stages are added (`stage_at` / `stage_next`, some with their
/// +8 and +0xc set first); the shader's programs become the pass's +0x5c and +0x44
/// (counted), its render states and the globals the exe writes follow, in the exe's order.
/// The stage local and the last pass are given up at the end.
// Translation incomplete: 00c124c0 - the C++ exception frame (FS:[0] chain, unwind states for the stage local and the held pass) is not translated.
pub fn fn_00c124c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, stage| {
        let stage = stage.addr();
        let mut held: u32;
        let word_1 = e.mem.u32(0x0120_22c0);
        held = word_1;
        hold_pass(e, held);
        let word_2 = e.mem.u32(held + 0x18);
        if word_2 < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_11 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_11, 4u32, 3u32, 0u32, 1u32]);
            let made_12 = e.mem.u32(stage);
            let word_13 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_12 + 0x8, word_13);
            let made_14 = e.mem.u32(stage);
            e.mem.set_u32(made_14 + 0xc, 1u32);
            let made_15 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_15]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_20 = e.mem.u32(base + 0xac);
        assign_counted(e, held + 0x5c, word_20);
        let word_21 = e.mem.u32(base + 0x104);
        assign_counted(e, held + 0x44, word_21);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cfec, 0x480u32);
        e.mem.set_u32(0x011f_c40c, 0xcu32);
        held = switch_pass(e, held, 0x0120_22c4);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_30 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_30, 4u32, 3u32, 0u32, 1u32]);
            let made_31 = e.mem.u32(stage);
            let word_32 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_31 + 0x8, word_32);
            let made_33 = e.mem.u32(stage);
            e.mem.set_u32(made_33 + 0xc, 1u32);
            let made_34 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_34]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_39 = e.mem.u32(base + 0xb0);
        assign_counted(e, held + 0x5c, word_39);
        let word_40 = e.mem.u32(base + 0x104);
        assign_counted(e, held + 0x44, word_40);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cff8, 0xc80u32);
        e.mem.set_u32(0x011f_c418, 0xcu32);
        held = switch_pass(e, held, 0x0120_22c8);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_49 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_49, 4u32, 3u32, 0u32, 1u32]);
            let made_50 = e.mem.u32(stage);
            let word_51 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_50 + 0x8, word_51);
            let made_52 = e.mem.u32(stage);
            e.mem.set_u32(made_52 + 0xc, 1u32);
            let made_53 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_53]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_58 = e.mem.u32(base + 0xb4);
        assign_counted(e, held + 0x5c, word_58);
        let word_59 = e.mem.u32(base + 0x108);
        assign_counted(e, held + 0x44, word_59);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d010, 0x4f0u32);
        e.mem.set_u32(0x011f_c430, 0xcu32);
        held = switch_pass(e, held, 0x0120_22cc);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_68 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_68, 4u32, 3u32, 0u32, 1u32]);
            let made_69 = e.mem.u32(stage);
            let word_70 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_69 + 0x8, word_70);
            let made_71 = e.mem.u32(stage);
            e.mem.set_u32(made_71 + 0xc, 1u32);
            let made_72 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_72]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_77 = e.mem.u32(base + 0xb8);
        assign_counted(e, held + 0x5c, word_77);
        let word_78 = e.mem.u32(base + 0x108);
        assign_counted(e, held + 0x44, word_78);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d01c, 0xcf0u32);
        e.mem.set_u32(0x011f_c43c, 0xcu32);
        held = switch_pass(e, held, 0x0120_22d0);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_87 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_87, 4u32, 3u32, 0u32, 1u32]);
            let made_88 = e.mem.u32(stage);
            let word_89 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_88 + 0x8, word_89);
            let made_90 = e.mem.u32(stage);
            e.mem.set_u32(made_90 + 0xc, 1u32);
            let made_91 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_91]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_96 = e.mem.u32(base + 0xbc);
        assign_counted(e, held + 0x5c, word_96);
        let word_97 = e.mem.u32(base + 0x10c);
        assign_counted(e, held + 0x44, word_97);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d034, 0x480u32);
        e.mem.set_u32(0x011f_c454, 0xcu32);
        held = switch_pass(e, held, 0x0120_22d4);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_106 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_106, 4u32, 3u32, 0u32, 1u32],
            );
            let made_107 = e.mem.u32(stage);
            let word_108 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_107 + 0x8, word_108);
            let made_109 = e.mem.u32(stage);
            e.mem.set_u32(made_109 + 0xc, 1u32);
            let made_110 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_110]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_115 = e.mem.u32(base + 0xc0);
        assign_counted(e, held + 0x5c, word_115);
        let word_116 = e.mem.u32(base + 0x10c);
        assign_counted(e, held + 0x44, word_116);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d040, 0xc80u32);
        e.mem.set_u32(0x011f_c460, 0xcu32);
        held = switch_pass(e, held, 0x0120_22d8);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_125 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_125, 4u32, 3u32, 0u32, 1u32],
            );
            let made_126 = e.mem.u32(stage);
            let word_127 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_126 + 0x8, word_127);
            let made_128 = e.mem.u32(stage);
            e.mem.set_u32(made_128 + 0xc, 1u32);
            let made_129 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_129]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_134 = e.mem.u32(base + 0xc4);
        assign_counted(e, held + 0x5c, word_134);
        let word_135 = e.mem.u32(base + 0x110);
        assign_counted(e, held + 0x44, word_135);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d058, 0x4f0u32);
        e.mem.set_u32(0x011f_c478, 0xcu32);
        held = switch_pass(e, held, 0x0120_22dc);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_144 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_144, 4u32, 3u32, 0u32, 1u32],
            );
            let made_145 = e.mem.u32(stage);
            let word_146 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_145 + 0x8, word_146);
            let made_147 = e.mem.u32(stage);
            e.mem.set_u32(made_147 + 0xc, 1u32);
            let made_148 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_148]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 0u32, 1u32], 6u32);
        }
        let word_153 = e.mem.u32(base + 0xc8);
        assign_counted(e, held + 0x5c, word_153);
        let word_154 = e.mem.u32(base + 0x110);
        assign_counted(e, held + 0x44, word_154);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d064, 0xcf0u32);
        e.mem.set_u32(0x011f_c484, 0xcu32);
        held = switch_pass(e, held, 0x0120_22e0);
        let word_155 = e.mem.u32(held + 0x18);
        if word_155 < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_166 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_166, 5u32, 3u32, 0u32, 1u32],
            );
            let made_167 = e.mem.u32(stage);
            let word_168 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_167 + 0x8, word_168);
            let made_169 = e.mem.u32(stage);
            e.mem.set_u32(made_169 + 0xc, 1u32);
            let made_170 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_170]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_175 = e.mem.u32(base + 0x8c);
        assign_counted(e, held + 0x5c, word_175);
        let word_176 = e.mem.u32(base + 0xf4);
        assign_counted(e, held + 0x44, word_176);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d080, 0x780u32);
        e.mem.set_u32(0x011f_c4a0, 0x2cu32);
        held = switch_pass(e, held, 0x0120_22e4);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_187 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_187, 5u32, 3u32, 0u32, 1u32],
            );
            let made_188 = e.mem.u32(stage);
            let word_189 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_188 + 0x8, word_189);
            let made_190 = e.mem.u32(stage);
            e.mem.set_u32(made_190 + 0xc, 1u32);
            let made_191 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_191]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_196 = e.mem.u32(base + 0x90);
        assign_counted(e, held + 0x5c, word_196);
        let word_197 = e.mem.u32(base + 0xf4);
        assign_counted(e, held + 0x44, word_197);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d09c, 0xf80u32);
        e.mem.set_u32(0x011f_c4bc, 0x2cu32);
        held = switch_pass(e, held, 0x0120_22e8);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_208 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_208, 5u32, 3u32, 0u32, 1u32],
            );
            let made_209 = e.mem.u32(stage);
            let word_210 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_209 + 0x8, word_210);
            let made_211 = e.mem.u32(stage);
            e.mem.set_u32(made_211 + 0xc, 1u32);
            let made_212 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_212]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_217 = e.mem.u32(base + 0x94);
        e.call(PASS_SET_00B79950, &args![held, word_217]);
        let word_218 = e.mem.u32(base + 0xf8);
        e.call(PASS_SET_00B80600, &args![held, word_218]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d0b4, 0x7f0u32);
        e.mem.set_u32(0x011f_c4d4, 0x2cu32);
        held = switch_pass(e, held, 0x0120_22ec);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_229 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_229, 5u32, 3u32, 0u32, 1u32],
            );
            let made_230 = e.mem.u32(stage);
            let word_231 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_230 + 0x8, word_231);
            let made_232 = e.mem.u32(stage);
            e.mem.set_u32(made_232 + 0xc, 1u32);
            let made_233 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_233]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_238 = e.mem.u32(base + 0x98);
        e.call(PASS_SET_00B79950, &args![held, word_238]);
        let word_239 = e.mem.u32(base + 0xf8);
        e.call(PASS_SET_00B80600, &args![held, word_239]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d0cc, 0xff0u32);
        e.mem.set_u32(0x011f_c4ec, 0x2cu32);
        held = switch_pass(e, held, 0x0120_22f0);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_250 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_250, 5u32, 3u32, 0u32, 1u32],
            );
            let made_251 = e.mem.u32(stage);
            let word_252 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_251 + 0x8, word_252);
            let made_253 = e.mem.u32(stage);
            e.mem.set_u32(made_253 + 0xc, 1u32);
            let made_254 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_254]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_259 = e.mem.u32(base + 0x9c);
        e.call(PASS_SET_00B79950, &args![held, word_259]);
        let word_260 = e.mem.u32(base + 0xfc);
        e.call(PASS_SET_00B80600, &args![held, word_260]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d0e0, 0x780u32);
        e.mem.set_u32(0x011f_c500, 0x2cu32);
        held = switch_pass(e, held, 0x0120_22f4);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_271 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_271, 5u32, 3u32, 0u32, 1u32],
            );
            let made_272 = e.mem.u32(stage);
            let word_273 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_272 + 0x8, word_273);
            let made_274 = e.mem.u32(stage);
            e.mem.set_u32(made_274 + 0xc, 1u32);
            let made_275 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_275]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_280 = e.mem.u32(base + 0xa0);
        e.call(PASS_SET_00B79950, &args![held, word_280]);
        let word_281 = e.mem.u32(base + 0xfc);
        e.call(PASS_SET_00B80600, &args![held, word_281]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d0f8, 0xf80u32);
        e.mem.set_u32(0x011f_c518, 0x2cu32);
        held = switch_pass(e, held, 0x0120_22f8);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_292 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_292, 5u32, 3u32, 0u32, 1u32],
            );
            let made_293 = e.mem.u32(stage);
            let word_294 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_293 + 0x8, word_294);
            let made_295 = e.mem.u32(stage);
            e.mem.set_u32(made_295 + 0xc, 1u32);
            let made_296 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_296]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_301 = e.mem.u32(base + 0xa4);
        e.call(PASS_SET_00B79950, &args![held, word_301]);
        let word_302 = e.mem.u32(base + 0x100);
        e.call(PASS_SET_00B80600, &args![held, word_302]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d10c, 0x7f0u32);
        e.mem.set_u32(0x011f_c52c, 0x2cu32);
        held = switch_pass(e, held, 0x0120_22fc);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_313 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_313, 5u32, 3u32, 0u32, 1u32],
            );
            let made_314 = e.mem.u32(stage);
            let word_315 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_314 + 0x8, word_315);
            let made_316 = e.mem.u32(stage);
            e.mem.set_u32(made_316 + 0xc, 1u32);
            let made_317 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_317]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_322 = e.mem.u32(base + 0xa8);
        e.call(PASS_SET_00B79950, &args![held, word_322]);
        let word_323 = e.mem.u32(base + 0x100);
        e.call(PASS_SET_00B80600, &args![held, word_323]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_d124, 0xff0u32);
        e.mem.set_u32(0x011f_c544, 0x2cu32);
        held = switch_pass(e, held, 0x0120_2300);
        if e.mem.u32(held + 0x18) < 4u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_330 = e.mem.u32(stage);
            let word_331 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_330 + 0x8, word_331);
            let made_332 = e.mem.u32(stage);
            e.mem.set_u32(made_332 + 0xc, 1u32);
            let made_333 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_333, 3u32, 3u32, 0u32, 1u32],
            );
            let made_334 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 3u32, made_334]);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
        }
        let word_337 = e.mem.u32(base + 0xcc);
        e.call(PASS_SET_00B79950, &args![held, word_337]);
        let word_338 = e.mem.u32(base + 0x114);
        e.call(PASS_SET_00B80600, &args![held, word_338]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        e.mem.set_u32(0x011f_d14c, 0x780u32);
        e.mem.set_u32(0x011f_c56c, 0xcu32);
        held = switch_pass(e, held, 0x0120_2304);
        if e.mem.u32(held + 0x18) < 4u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_345 = e.mem.u32(stage);
            let word_346 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_345 + 0x8, word_346);
            let made_347 = e.mem.u32(stage);
            e.mem.set_u32(made_347 + 0xc, 1u32);
            let made_348 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_348, 3u32, 3u32, 0u32, 1u32],
            );
            let made_349 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 3u32, made_349]);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
        }
        let word_352 = e.mem.u32(base + 0xd0);
        e.call(PASS_SET_00B79950, &args![held, word_352]);
        let word_353 = e.mem.u32(base + 0x114);
        e.call(PASS_SET_00B80600, &args![held, word_353]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        e.mem.set_u32(0x011f_d158, 0xf80u32);
        e.mem.set_u32(0x011f_c578, 0xcu32);
        held = switch_pass(e, held, 0x0120_2308);
        if e.mem.u32(held + 0x18) < 4u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_360 = e.mem.u32(stage);
            let word_361 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_360 + 0x8, word_361);
            let made_362 = e.mem.u32(stage);
            e.mem.set_u32(made_362 + 0xc, 1u32);
            let made_363 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_363, 3u32, 3u32, 0u32, 1u32],
            );
            let made_364 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 3u32, made_364]);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
        }
        let word_367 = e.mem.u32(base + 0xd4);
        e.call(PASS_SET_00B79950, &args![held, word_367]);
        let word_368 = e.mem.u32(base + 0x118);
        e.call(PASS_SET_00B80600, &args![held, word_368]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        e.mem.set_u32(0x011f_d17c, 0x780u32);
        e.mem.set_u32(0x011f_c59c, 0xcu32);
        held = switch_pass(e, held, 0x0120_230c);
        if e.mem.u32(held + 0x18) < 4u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_375 = e.mem.u32(stage);
            let word_376 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_375 + 0x8, word_376);
            let made_377 = e.mem.u32(stage);
            e.mem.set_u32(made_377 + 0xc, 1u32);
            let made_378 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_378, 3u32, 3u32, 0u32, 1u32],
            );
            let made_379 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 3u32, made_379]);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
        }
        let word_382 = e.mem.u32(base + 0xd8);
        e.call(PASS_SET_00B79950, &args![held, word_382]);
        let word_383 = e.mem.u32(base + 0x118);
        e.call(PASS_SET_00B80600, &args![held, word_383]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        e.mem.set_u32(0x011f_d188, 0xf80u32);
        e.mem.set_u32(0x011f_c5a8, 0xcu32);
        held = switch_pass(e, held, 0x0120_2310);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_400 = e.mem.u32(base + 0xdc);
        e.call(PASS_SET_00B79950, &args![held, word_400]);
        let word_401 = e.mem.u32(base + 0x11c);
        e.call(PASS_SET_00B80600, &args![held, word_401]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cf94, 0x0008_0780u32);
        e.mem.set_u32(0x011f_c3b4, 0x13cu32);
        e.mem.set_u32(0x011f_b7d4, 0x300u32);
        held = switch_pass(e, held, 0x0120_2314);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_418 = e.mem.u32(base + 0xe0);
        e.call(PASS_SET_00B79950, &args![held, word_418]);
        let word_419 = e.mem.u32(base + 0x11c);
        e.call(PASS_SET_00B80600, &args![held, word_419]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cfa0, 0x0008_0f80u32);
        e.mem.set_u32(0x011f_c3c0, 0x13cu32);
        e.mem.set_u32(0x011f_b7e0, 0x300u32);
        held = switch_pass(e, held, 0x0120_2318);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_436 = e.mem.u32(base + 0xe4);
        e.call(PASS_SET_00B79950, &args![held, word_436]);
        let word_437 = e.mem.u32(base + 0x120);
        e.call(PASS_SET_00B80600, &args![held, word_437]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cfb0, 0x0008_0780u32);
        e.mem.set_u32(0x011f_c3d0, 0x13cu32);
        e.mem.set_u32(0x011f_b7f0, 0x300u32);
        held = switch_pass(e, held, 0x0120_231c);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_454 = e.mem.u32(base + 0xe8);
        e.call(PASS_SET_00B79950, &args![held, word_454]);
        let word_455 = e.mem.u32(base + 0x120);
        e.call(PASS_SET_00B80600, &args![held, word_455]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cfbc, 0x0008_0f80u32);
        e.mem.set_u32(0x011f_c3dc, 0x13cu32);
        e.mem.set_u32(0x011f_b7fc, 0x300u32);
        held = switch_pass(e, held, 0x0120_2320);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_472 = e.mem.u32(base + 0xec);
        e.call(PASS_SET_00B79950, &args![held, word_472]);
        let word_473 = e.mem.u32(base + 0x124);
        e.call(PASS_SET_00B80600, &args![held, word_473]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        e.mem.set_u32(0x011f_cfcc, 0x0008_0780u32);
        e.mem.set_u32(0x011f_c3ec, 0x13cu32);
        e.mem.set_u32(0x011f_b80c, 0x300u32);
        held = switch_pass(e, held, 0x0120_2324);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_490 = e.mem.u32(base + 0xf0);
        e.call(PASS_SET_00B79950, &args![held, word_490]);
        let word_491 = e.mem.u32(base + 0x124);
        e.call(PASS_SET_00B80600, &args![held, word_491]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let made_492 = e.mem.u32(stage);
        e.mem.set_u32(0x011f_cfd8, 0x0008_0f80u32);
        e.mem.set_u32(0x011f_c3f8, 0x13cu32);
        e.mem.set_u32(0x011f_b818, 0x300u32);
        release_stage(e, made_492);
        release_pass(e, held);
    });
}

// Translated from 00c16400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a shader's slot, referenced from
/// `010bc1cc`): sets up 10 shader passes in turn, read from the pass globals `01202350` to `01202374`
/// in the body's order, each held while in use (the held pass switched with `switch_pass`):
/// while a pass has fewer stages (+0x18)
/// than the limit, default stages are added (`stage_at` / `stage_next`, some with their
/// +8 and +0xc set first); the shader's programs become the pass's +0x5c and +0x44
/// (counted), its render states and the globals the exe writes follow, in the exe's order.
/// The stage local and the last pass are given up at the end.
// Translation incomplete: 00c16400 - the C++ exception frame (FS:[0] chain, unwind states for the stage local and the held pass) is not translated.
pub fn fn_00c16400(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, stage| {
        let stage = stage.addr();
        let mut held: u32;
        let word_1 = e.mem.u32(0x0120_2350);
        held = word_1;
        hold_pass(e, held);
        let word_2 = e.mem.u32(held + 0x18);
        if word_2 < 1u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
        }
        let word_6 = e.mem.u32(base + 0x8c);
        assign_counted(e, held + 0x5c, word_6);
        let word_7 = e.mem.u32(base + 0x104);
        assign_counted(e, held + 0x44, word_7);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_8 = e.mem.u32(0x011f_cde0);
        let word_9 = e.mem.u32(0x011f_c200);
        let word_10 = e.mem.u32(0x011f_b620);
        e.mem.set_u32(0x011f_ce00, word_8);
        let word_11 = e.mem.u32(0x011f_aa40);
        e.mem.set_u32(0x011f_c220, word_9);
        e.mem.set_u32(0x011f_b640, word_10);
        e.mem.set_u32(0x011f_aa60, word_11);
        held = switch_pass(e, held, 0x0120_2354);
        if e.mem.u32(held + 0x18) < 2u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
        }
        let word_18 = e.mem.u32(base + 0x90);
        assign_counted(e, held + 0x5c, word_18);
        let word_19 = e.mem.u32(base + 0x108);
        assign_counted(e, held + 0x44, word_19);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_20 = e.mem.u32(0x011f_ce10);
        let word_21 = e.mem.u32(0x011f_c230);
        let word_22 = e.mem.u32(0x011f_b650);
        e.mem.set_u32(0x011f_ce68, word_20);
        let word_23 = e.mem.u32(0x011f_aa70);
        e.mem.set_u32(0x011f_c288, word_21);
        e.mem.set_u32(0x011f_b6a8, word_22);
        e.mem.set_u32(0x011f_aac8, word_23);
        held = switch_pass(e, held, 0x0120_2358);
        if e.mem.u32(held + 0x18) < 2u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
        }
        let word_30 = e.mem.u32(base + 0x94);
        assign_counted(e, held + 0x5c, word_30);
        let word_31 = e.mem.u32(base + 0x10c);
        assign_counted(e, held + 0x44, word_31);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_32 = e.mem.u32(0x011f_ce7c);
        let word_33 = e.mem.u32(0x011f_c29c);
        let word_34 = e.mem.u32(0x011f_b6bc);
        e.mem.set_u32(0x011f_ce6c, word_32);
        let word_35 = e.mem.u32(0x011f_aadc);
        e.mem.set_u32(0x011f_c28c, word_33);
        e.mem.set_u32(0x011f_b6ac, word_34);
        e.mem.set_u32(0x011f_aacc, word_35);
        held = switch_pass(e, held, 0x0120_235c);
        let word_36 = e.mem.u32(held + 0x18);
        if word_36 < 4u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_40 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_40, 1u32, 3u32, 2u32, 1u32]);
            let made_41 = e.mem.u32(stage);
            let word_42 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_41 + 0x8, word_42);
            let made_43 = e.mem.u32(stage);
            let word_44 = e.mem.u32(held + 0x14);
            e.call(PASS_SET_STAGE, &args![held, word_44, made_43]);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_45 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_45, 2u32, 3u32, 2u32, 1u32]);
            let made_46 = e.mem.u32(stage);
            let word_47 = e.mem.u32(0x011f_9538);
            e.mem.set_u32(made_46 + 0x8, word_47);
            let made_48 = e.mem.u32(stage);
            let word_49 = e.mem.u32(held + 0x14);
            e.call(PASS_SET_STAGE, &args![held, word_49, made_48]);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_50 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_50, 3u32, 1u32, 2u32, 1u32]);
            let made_51 = e.mem.u32(stage);
            let word_52 = e.mem.u32(0x011f_950c);
            e.mem.set_u32(made_51 + 0x8, word_52);
            let made_53 = e.mem.u32(stage);
            let word_54 = e.mem.u32(held + 0x14);
            e.call(PASS_SET_STAGE, &args![held, word_54, made_53]);
        }
        let word_55 = e.mem.u32(base + 0x98);
        assign_counted(e, held + 0x5c, word_55);
        let word_56 = e.mem.u32(base + 0x110);
        assign_counted(e, held + 0x44, word_56);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_57 = e.mem.u32(0x011f_ce70);
        let word_58 = e.mem.u32(0x011f_c290);
        let word_59 = e.mem.u32(0x011f_b6b0);
        e.mem.set_u32(0x011f_ce78, word_57);
        let word_60 = e.mem.u32(0x011f_aad0);
        e.mem.set_u32(0x011f_c298, word_58);
        e.mem.set_u32(0x011f_b6b8, word_59);
        e.mem.set_u32(0x011f_aad8, word_60);
        held = switch_pass(e, held, 0x0120_2360);
        if e.mem.u32(held + 0x18) < 1u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
        }
        let word_64 = e.mem.u32(base + 0x9c);
        assign_counted(e, held + 0x5c, word_64);
        let word_65 = e.mem.u32(base + 0x114);
        assign_counted(e, held + 0x44, word_65);
        set_render_state(e, held, 0x13u32, 2u32, 1u32);
        set_render_state(e, held, 0x14u32, 2u32, 1u32);
        let word_66 = e.mem.u32(0x011f_d130);
        let word_67 = e.mem.u32(0x011f_c550);
        let word_68 = e.mem.u32(0x011f_b970);
        e.mem.set_u32(0x011f_d140, word_66);
        let word_69 = e.mem.u32(0x011f_ad90);
        e.mem.set_u32(0x011f_c560, word_67);
        e.mem.set_u32(0x011f_b980, word_68);
        e.mem.set_u32(0x011f_ada0, word_69);
        held = switch_pass(e, held, 0x0120_2364);
        if e.mem.u32(held + 0x18) < 4u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_73 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_73, 1u32, 3u32, 2u32, 1u32]);
            let made_74 = e.mem.u32(stage);
            let word_75 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_74 + 0x8, word_75);
            let made_76 = e.mem.u32(stage);
            let word_77 = e.mem.u32(held + 0x14);
            e.call(PASS_SET_STAGE, &args![held, word_77, made_76]);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_78 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_78, 2u32, 3u32, 2u32, 1u32]);
            let made_79 = e.mem.u32(stage);
            let word_80 = e.mem.u32(0x011f_9538);
            e.mem.set_u32(made_79 + 0x8, word_80);
            let made_81 = e.mem.u32(stage);
            let word_82 = e.mem.u32(held + 0x14);
            e.call(PASS_SET_STAGE, &args![held, word_82, made_81]);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_83 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_83, 3u32, 1u32, 2u32, 1u32]);
            let made_84 = e.mem.u32(stage);
            let word_85 = e.mem.u32(0x011f_950c);
            e.mem.set_u32(made_84 + 0x8, word_85);
            let made_86 = e.mem.u32(stage);
            let word_87 = e.mem.u32(held + 0x14);
            e.call(PASS_SET_STAGE, &args![held, word_87, made_86]);
        }
        let word_88 = e.mem.u32(base + 0xa0);
        assign_counted(e, held + 0x5c, word_88);
        let word_89 = e.mem.u32(base + 0x118);
        assign_counted(e, held + 0x44, word_89);
        set_render_state(e, held, 0x13u32, 2u32, 1u32);
        set_render_state(e, held, 0x14u32, 2u32, 1u32);
        let word_90 = e.mem.u32(0x011f_d134);
        let word_91 = e.mem.u32(0x011f_c554);
        let word_92 = e.mem.u32(0x011f_b974);
        e.mem.set_u32(0x011f_d144, word_90);
        let word_93 = e.mem.u32(0x011f_ad94);
        e.mem.set_u32(0x011f_c564, word_91);
        e.mem.set_u32(0x011f_b984, word_92);
        e.mem.set_u32(0x011f_ada4, word_93);
        held = switch_pass(e, held, 0x0120_236c);
        if e.mem.u32(held + 0x18) < 1u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
        }
        let word_97 = e.mem.u32(base + 0xa8);
        assign_counted(e, held + 0x5c, word_97);
        let word_98 = e.mem.u32(base + 0x120);
        assign_counted(e, held + 0x44, word_98);
        set_render_state(e, held, 0x13u32, 2u32, 1u32);
        set_render_state(e, held, 0x14u32, 2u32, 1u32);
        let word_99 = e.mem.u32(0x011f_d1d0);
        let word_100 = e.mem.u32(0x011f_c5f0);
        let word_101 = e.mem.u32(0x011f_ba10);
        e.mem.set_u32(0x011f_d1e0, word_99);
        let word_102 = e.mem.u32(0x011f_ae30);
        e.mem.set_u32(0x011f_c600, word_100);
        e.mem.set_u32(0x011f_ba20, word_101);
        e.mem.set_u32(0x011f_ae40, word_102);
        held = switch_pass(e, held, 0x0120_2370);
        if e.mem.u32(held + 0x18) < 4u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_106 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_106, 1u32, 3u32, 2u32, 1u32],
            );
            let made_107 = e.mem.u32(stage);
            let word_108 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_107 + 0x8, word_108);
            let made_109 = e.mem.u32(stage);
            let word_110 = e.mem.u32(held + 0x14);
            e.call(PASS_SET_STAGE, &args![held, word_110, made_109]);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_111 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_111, 2u32, 3u32, 2u32, 1u32],
            );
            let made_112 = e.mem.u32(stage);
            let word_113 = e.mem.u32(0x011f_9538);
            e.mem.set_u32(made_112 + 0x8, word_113);
            let made_114 = e.mem.u32(stage);
            let word_115 = e.mem.u32(held + 0x14);
            e.call(PASS_SET_STAGE, &args![held, word_115, made_114]);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_116 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_116, 3u32, 1u32, 2u32, 1u32],
            );
            let made_117 = e.mem.u32(stage);
            let word_118 = e.mem.u32(0x011f_950c);
            e.mem.set_u32(made_117 + 0x8, word_118);
            let made_119 = e.mem.u32(stage);
            let word_120 = e.mem.u32(held + 0x14);
            e.call(PASS_SET_STAGE, &args![held, word_120, made_119]);
        }
        let word_121 = e.mem.u32(base + 0xac);
        assign_counted(e, held + 0x5c, word_121);
        let word_122 = e.mem.u32(base + 0x124);
        assign_counted(e, held + 0x44, word_122);
        set_render_state(e, held, 0x13u32, 2u32, 1u32);
        set_render_state(e, held, 0x14u32, 2u32, 1u32);
        let word_123 = e.mem.u32(0x011f_d1d4);
        let word_124 = e.mem.u32(0x011f_c5f4);
        let word_125 = e.mem.u32(0x011f_ba14);
        e.mem.set_u32(0x011f_d1e4, word_123);
        let word_126 = e.mem.u32(0x011f_ae34);
        e.mem.set_u32(0x011f_c604, word_124);
        e.mem.set_u32(0x011f_ba24, word_125);
        e.mem.set_u32(0x011f_ae44, word_126);
        held = switch_pass(e, held, 0x0120_2368);
        if e.mem.u32(held + 0x18) < 1u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
        }
        let word_130 = e.mem.u32(base + 0xa4);
        assign_counted(e, held + 0x5c, word_130);
        let word_131 = e.mem.u32(base + 0x11c);
        assign_counted(e, held + 0x44, word_131);
        set_render_state(e, held, 0x13u32, 9u32, 1u32);
        set_render_state(e, held, 0x14u32, 1u32, 1u32);
        let word_132 = e.mem.u32(0x011f_d1a8);
        let word_133 = e.mem.u32(0x011f_c5c8);
        let word_134 = e.mem.u32(0x011f_b9e8);
        e.mem.set_u32(0x011f_d1c4, word_132);
        let word_135 = e.mem.u32(0x011f_ae08);
        e.mem.set_u32(0x011f_c5e4, word_133);
        e.mem.set_u32(0x011f_ba04, word_134);
        e.mem.set_u32(0x011f_ae24, word_135);
        held = switch_pass(e, held, 0x0120_2374);
        if e.mem.u32(held + 0x18) < 1u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
        }
        let word_139 = e.mem.u32(base + 0xb0);
        assign_counted(e, held + 0x5c, word_139);
        let word_140 = e.mem.u32(base + 0x128);
        assign_counted(e, held + 0x44, word_140);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        set_render_state(e, held, 0x13u32, 5u32, 1u32);
        set_render_state(e, held, 0x14u32, 6u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 3u32, 1u32);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        let word_141 = e.mem.u32(0x011f_d5c8);
        let word_142 = e.mem.u32(0x011f_c9e8);
        let word_143 = e.mem.u32(0x011f_be08);
        e.mem.set_u32(0x011f_d5d8, word_141);
        let word_144 = e.mem.u32(0x011f_b228);
        e.mem.set_u32(0x011f_b238, word_144);
        let made_145 = e.mem.u32(stage);
        e.mem.set_u32(0x011f_c9f8, word_142);
        e.mem.set_u32(0x011f_be18, word_143);
        release_stage(e, made_145);
        release_pass(e, held);
    });
}

// Translated from 00c17510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; a shader's slot, referenced from
/// `010bc1d0`): sets up 20 shader passes in turn, read from the pass globals `01202378` to `012023c4`
/// in the body's order, each held while in use (the held pass switched with `switch_pass`):
/// while a pass has fewer stages (+0x18)
/// than the limit, default stages are added (`stage_at` / `stage_next`, some with their
/// +8 and +0xc set first); the shader's programs become the pass's +0x5c and +0x44
/// (counted), its render states and the globals the exe writes follow, in the exe's order.
/// The stage local and the last pass are given up at the end.
// Translation incomplete: 00c17510 - the C++ exception frame (FS:[0] chain, unwind states for the stage local and the held pass) is not translated.
pub fn fn_00c17510(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, stage| {
        let stage = stage.addr();
        let mut held: u32;
        let word_1 = e.mem.u32(0x0120_2378);
        held = word_1;
        hold_pass(e, held);
        let word_2 = e.mem.u32(held + 0x18);
        if word_2 < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_11 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_11, 4u32, 3u32, 2u32, 1u32]);
            let made_12 = e.mem.u32(stage);
            let word_13 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_12 + 0x8, word_13);
            let made_14 = e.mem.u32(stage);
            e.mem.set_u32(made_14 + 0xc, 1u32);
            let made_15 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_15]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 2u32, 1u32], 6u32);
        }
        let word_20 = e.mem.u32(base + 0xb4);
        assign_counted(e, held + 0x5c, word_20);
        let word_21 = e.mem.u32(0x011f_db9c);
        assign_counted(e, held + 0x44, word_21);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_22 = e.mem.u32(0x011f_cfdc);
        let word_23 = e.mem.u32(0x011f_c3fc);
        let word_24 = e.mem.u32(0x011f_b81c);
        e.mem.set_u32(0x011f_cffc, word_22);
        let word_25 = e.mem.u32(0x011f_ac3c);
        e.mem.set_u32(0x011f_c41c, word_23);
        e.mem.set_u32(0x011f_b83c, word_24);
        e.mem.set_u32(0x011f_ac5c, word_25);
        held = switch_pass(e, held, 0x0120_237c);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_34 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_34, 4u32, 3u32, 2u32, 1u32]);
            let made_35 = e.mem.u32(stage);
            let word_36 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_35 + 0x8, word_36);
            let made_37 = e.mem.u32(stage);
            e.mem.set_u32(made_37 + 0xc, 1u32);
            let made_38 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_38]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 2u32, 1u32], 6u32);
        }
        let word_43 = e.mem.u32(base + 0xb8);
        assign_counted(e, held + 0x5c, word_43);
        let word_44 = e.mem.u32(0x011f_dba4);
        assign_counted(e, held + 0x44, word_44);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_45 = e.mem.u32(0x011f_d000);
        let word_46 = e.mem.u32(0x011f_c420);
        let word_47 = e.mem.u32(0x011f_b840);
        e.mem.set_u32(0x011f_d020, word_45);
        let word_48 = e.mem.u32(0x011f_ac60);
        e.mem.set_u32(0x011f_c440, word_46);
        e.mem.set_u32(0x011f_b860, word_47);
        e.mem.set_u32(0x011f_ac80, word_48);
        held = switch_pass(e, held, 0x0120_2380);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_57 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_57, 4u32, 3u32, 2u32, 1u32]);
            let made_58 = e.mem.u32(stage);
            let word_59 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_58 + 0x8, word_59);
            let made_60 = e.mem.u32(stage);
            e.mem.set_u32(made_60 + 0xc, 1u32);
            let made_61 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_61]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 2u32, 1u32], 6u32);
        }
        let word_66 = e.mem.u32(base + 0xbc);
        assign_counted(e, held + 0x5c, word_66);
        let word_67 = e.mem.u32(0x011f_dbac);
        assign_counted(e, held + 0x44, word_67);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_68 = e.mem.u32(0x011f_d024);
        let word_69 = e.mem.u32(0x011f_c444);
        let word_70 = e.mem.u32(0x011f_b864);
        e.mem.set_u32(0x011f_d044, word_68);
        let word_71 = e.mem.u32(0x011f_ac84);
        e.mem.set_u32(0x011f_c464, word_69);
        e.mem.set_u32(0x011f_b884, word_70);
        e.mem.set_u32(0x011f_aca4, word_71);
        held = switch_pass(e, held, 0x0120_2384);
        if e.mem.u32(held + 0x18) < 7u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_80 = e.mem.u32(stage);
            e.call(SETUP_DEFAULT_STAGE, &args![made_80, 4u32, 3u32, 2u32, 1u32]);
            let made_81 = e.mem.u32(stage);
            let word_82 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_81 + 0x8, word_82);
            let made_83 = e.mem.u32(stage);
            e.mem.set_u32(made_83 + 0xc, 1u32);
            let made_84 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 4u32, made_84]);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
            stage_at(e, held, stage, [6u32, 3u32, 2u32, 1u32], 6u32);
        }
        let word_89 = e.mem.u32(base + 0xc0);
        assign_counted(e, held + 0x5c, word_89);
        let word_90 = e.mem.u32(0x011f_dbb4);
        assign_counted(e, held + 0x44, word_90);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_91 = e.mem.u32(0x011f_d048);
        let word_92 = e.mem.u32(0x011f_c468);
        let word_93 = e.mem.u32(0x011f_b888);
        e.mem.set_u32(0x011f_d068, word_91);
        let word_94 = e.mem.u32(0x011f_aca8);
        e.mem.set_u32(0x011f_c488, word_92);
        e.mem.set_u32(0x011f_b8a8, word_93);
        e.mem.set_u32(0x011f_acc8, word_94);
        held = switch_pass(e, held, 0x0120_2388);
        let word_95 = e.mem.u32(held + 0x18);
        if word_95 < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_106 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_106, 5u32, 3u32, 2u32, 1u32],
            );
            let made_107 = e.mem.u32(stage);
            let word_108 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_107 + 0x8, word_108);
            let made_109 = e.mem.u32(stage);
            e.mem.set_u32(made_109 + 0xc, 1u32);
            let made_110 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_110]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 2u32, 1u32], 7u32);
        }
        let word_115 = e.mem.u32(base + 0xc4);
        assign_counted(e, held + 0x5c, word_115);
        let word_116 = e.mem.u32(0x011f_db08);
        assign_counted(e, held + 0x44, word_116);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_117 = e.mem.u32(0x011f_d06c);
        let word_118 = e.mem.u32(0x011f_c48c);
        let word_119 = e.mem.u32(0x011f_b8ac);
        e.mem.set_u32(0x011f_d0a0, word_117);
        let word_120 = e.mem.u32(0x011f_accc);
        e.mem.set_u32(0x011f_c4c0, word_118);
        e.mem.set_u32(0x011f_b8e0, word_119);
        e.mem.set_u32(0x011f_ad00, word_120);
        held = switch_pass(e, held, 0x0120_238c);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_131 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_131, 5u32, 3u32, 2u32, 1u32],
            );
            let made_132 = e.mem.u32(stage);
            let word_133 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_132 + 0x8, word_133);
            let made_134 = e.mem.u32(stage);
            e.mem.set_u32(made_134 + 0xc, 1u32);
            let made_135 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_135]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 2u32, 1u32], 7u32);
        }
        let word_140 = e.mem.u32(base + 0xc8);
        assign_counted(e, held + 0x5c, word_140);
        let word_141 = e.mem.u32(0x011f_db1c);
        assign_counted(e, held + 0x44, word_141);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_142 = e.mem.u32(0x011f_d0a8);
        let word_143 = e.mem.u32(0x011f_c4c8);
        let word_144 = e.mem.u32(0x011f_b8e8);
        e.mem.set_u32(0x011f_d0d0, word_142);
        let word_145 = e.mem.u32(0x011f_ad08);
        e.mem.set_u32(0x011f_c4f0, word_143);
        e.mem.set_u32(0x011f_b910, word_144);
        e.mem.set_u32(0x011f_ad30, word_145);
        held = switch_pass(e, held, 0x0120_2390);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_156 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_156, 5u32, 3u32, 2u32, 1u32],
            );
            let made_157 = e.mem.u32(stage);
            let word_158 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_157 + 0x8, word_158);
            let made_159 = e.mem.u32(stage);
            e.mem.set_u32(made_159 + 0xc, 1u32);
            let made_160 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_160]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 2u32, 1u32], 7u32);
        }
        let word_165 = e.mem.u32(base + 0xcc);
        assign_counted(e, held + 0x5c, word_165);
        let word_166 = e.mem.u32(0x011f_db34);
        assign_counted(e, held + 0x44, word_166);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_167 = e.mem.u32(0x011f_d0d8);
        let word_168 = e.mem.u32(0x011f_c4f8);
        let word_169 = e.mem.u32(0x011f_b918);
        e.mem.set_u32(0x011f_d0fc, word_167);
        let word_170 = e.mem.u32(0x011f_ad38);
        e.mem.set_u32(0x011f_c51c, word_168);
        e.mem.set_u32(0x011f_b93c, word_169);
        e.mem.set_u32(0x011f_ad5c, word_170);
        held = switch_pass(e, held, 0x0120_2394);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_181 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_181, 5u32, 3u32, 2u32, 1u32],
            );
            let made_182 = e.mem.u32(stage);
            let word_183 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_182 + 0x8, word_183);
            let made_184 = e.mem.u32(stage);
            e.mem.set_u32(made_184 + 0xc, 1u32);
            let made_185 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_185]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 2u32, 1u32], 7u32);
        }
        let word_190 = e.mem.u32(base + 0xd0);
        assign_counted(e, held + 0x5c, word_190);
        let word_191 = e.mem.u32(0x011f_db40);
        assign_counted(e, held + 0x44, word_191);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_192 = e.mem.u32(0x011f_d104);
        let word_193 = e.mem.u32(0x011f_c524);
        let word_194 = e.mem.u32(0x011f_b944);
        e.mem.set_u32(0x011f_d128, word_192);
        let word_195 = e.mem.u32(0x011f_ad64);
        e.mem.set_u32(0x011f_c548, word_193);
        e.mem.set_u32(0x011f_b968, word_194);
        e.mem.set_u32(0x011f_ad88, word_195);
        held = switch_pass(e, held, 0x0120_2398);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_206 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_206, 5u32, 3u32, 2u32, 1u32],
            );
            let made_207 = e.mem.u32(stage);
            let word_208 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_207 + 0x8, word_208);
            let made_209 = e.mem.u32(stage);
            e.mem.set_u32(made_209 + 0xc, 1u32);
            let made_210 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_210]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 2u32, 1u32], 7u32);
        }
        let word_215 = e.mem.u32(base + 0xd4);
        assign_counted(e, held + 0x5c, word_215);
        let word_216 = e.mem.u32(0x011f_db4c);
        assign_counted(e, held + 0x44, word_216);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_217 = e.mem.u32(0x011f_ced4);
        let word_218 = e.mem.u32(0x011f_c2f4);
        let word_219 = e.mem.u32(0x011f_b714);
        e.mem.set_u32(0x011f_cefc, word_217);
        let word_220 = e.mem.u32(0x011f_ab34);
        e.mem.set_u32(0x011f_c31c, word_218);
        e.mem.set_u32(0x011f_b73c, word_219);
        e.mem.set_u32(0x011f_ab5c, word_220);
        held = switch_pass(e, held, 0x0120_239c);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_231 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_231, 5u32, 3u32, 2u32, 1u32],
            );
            let made_232 = e.mem.u32(stage);
            let word_233 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_232 + 0x8, word_233);
            let made_234 = e.mem.u32(stage);
            e.mem.set_u32(made_234 + 0xc, 1u32);
            let made_235 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_235]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 2u32, 1u32], 7u32);
        }
        let word_240 = e.mem.u32(base + 0xd8);
        assign_counted(e, held + 0x5c, word_240);
        let word_241 = e.mem.u32(0x011f_db58);
        assign_counted(e, held + 0x44, word_241);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_242 = e.mem.u32(0x011f_cf00);
        let word_243 = e.mem.u32(0x011f_c320);
        let word_244 = e.mem.u32(0x011f_b740);
        e.mem.set_u32(0x011f_cf28, word_242);
        let word_245 = e.mem.u32(0x011f_ab60);
        e.mem.set_u32(0x011f_c348, word_243);
        e.mem.set_u32(0x011f_b768, word_244);
        e.mem.set_u32(0x011f_ab88, word_245);
        held = switch_pass(e, held, 0x0120_23a0);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_256 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_256, 5u32, 3u32, 2u32, 1u32],
            );
            let made_257 = e.mem.u32(stage);
            let word_258 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_257 + 0x8, word_258);
            let made_259 = e.mem.u32(stage);
            e.mem.set_u32(made_259 + 0xc, 1u32);
            let made_260 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_260]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 2u32, 1u32], 7u32);
        }
        let word_265 = e.mem.u32(base + 0xdc);
        assign_counted(e, held + 0x5c, word_265);
        let word_266 = e.mem.u32(0x011f_db64);
        assign_counted(e, held + 0x44, word_266);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_267 = e.mem.u32(0x011f_cf2c);
        let word_268 = e.mem.u32(0x011f_c34c);
        let word_269 = e.mem.u32(0x011f_b76c);
        e.mem.set_u32(0x011f_cf54, word_267);
        let word_270 = e.mem.u32(0x011f_ab8c);
        e.mem.set_u32(0x011f_c374, word_268);
        e.mem.set_u32(0x011f_b794, word_269);
        e.mem.set_u32(0x011f_abb4, word_270);
        held = switch_pass(e, held, 0x0120_23a4);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_281 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_281, 5u32, 3u32, 2u32, 1u32],
            );
            let made_282 = e.mem.u32(stage);
            let word_283 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_282 + 0x8, word_283);
            let made_284 = e.mem.u32(stage);
            e.mem.set_u32(made_284 + 0xc, 1u32);
            let made_285 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_285]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 2u32, 1u32], 7u32);
        }
        let word_290 = e.mem.u32(base + 0xe0);
        assign_counted(e, held + 0x5c, word_290);
        let word_291 = e.mem.u32(0x011f_db70);
        e.call(PASS_SET_00B80600, &args![held, word_291]);
        set_render_state(e, held, 0x1bu32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 1u32, 1u32);
        let word_292 = e.mem.u32(0x011f_cf58);
        let word_293 = e.mem.u32(0x011f_c378);
        let word_294 = e.mem.u32(0x011f_b798);
        e.mem.set_u32(0x011f_cf80, word_292);
        let word_295 = e.mem.u32(0x011f_abb8);
        e.mem.set_u32(0x011f_c3a0, word_293);
        e.mem.set_u32(0x011f_b7c0, word_294);
        e.mem.set_u32(0x011f_abe0, word_295);
        held = switch_pass(e, held, 0x0120_23a8);
        if e.mem.u32(held + 0x18) < 4u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_302 = e.mem.u32(stage);
            let word_303 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_302 + 0x8, word_303);
            let made_304 = e.mem.u32(stage);
            e.mem.set_u32(made_304 + 0xc, 1u32);
            let made_305 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_305, 3u32, 3u32, 2u32, 1u32],
            );
            let made_306 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 3u32, made_306]);
        }
        let word_307 = e.mem.u32(base + 0xe4);
        e.call(PASS_SET_00B79950, &args![held, word_307]);
        let word_308 = fn_00c163a0(e, 0x5du32);
        e.call(PASS_SET_00B80600, &args![held, word_308]);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        let word_309 = e.mem.u32(0x011f_d148);
        let word_310 = e.mem.u32(0x011f_c568);
        let word_311 = e.mem.u32(0x011f_b988);
        e.mem.set_u32(0x011f_d15c, word_309);
        let word_312 = e.mem.u32(0x011f_ada8);
        e.mem.set_u32(0x011f_c57c, word_310);
        e.mem.set_u32(0x011f_b99c, word_311);
        e.mem.set_u32(0x011f_adbc, word_312);
        held = switch_pass(e, held, 0x0120_23ac);
        if e.mem.u32(held + 0x18) < 4u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_319 = e.mem.u32(stage);
            let word_320 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_319 + 0x8, word_320);
            let made_321 = e.mem.u32(stage);
            e.mem.set_u32(made_321 + 0xc, 1u32);
            let made_322 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_322, 3u32, 3u32, 2u32, 1u32],
            );
            let made_323 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 3u32, made_323]);
        }
        let word_324 = e.mem.u32(base + 0xe8);
        e.call(PASS_SET_00B79950, &args![held, word_324]);
        let word_325 = fn_00c163a0(e, 0x5eu32);
        e.call(PASS_SET_00B80600, &args![held, word_325]);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        let word_326 = e.mem.u32(0x011f_d178);
        let word_327 = e.mem.u32(0x011f_c598);
        let word_328 = e.mem.u32(0x011f_b9b8);
        e.mem.set_u32(0x011f_d18c, word_326);
        let word_329 = e.mem.u32(0x011f_add8);
        e.mem.set_u32(0x011f_c5ac, word_327);
        e.mem.set_u32(0x011f_b9cc, word_328);
        e.mem.set_u32(0x011f_adec, word_329);
        held = switch_pass(e, held, 0x0120_23b0);
        if e.mem.u32(held + 0x18) < 6u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_337 = e.mem.u32(stage);
            let word_338 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_337 + 0x8, word_338);
            let made_339 = e.mem.u32(stage);
            e.mem.set_u32(made_339 + 0xc, 1u32);
            let made_340 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_340, 3u32, 3u32, 2u32, 1u32],
            );
            let made_341 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 3u32, made_341]);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
        }
        let word_346 = e.mem.u32(base + 0xec);
        e.call(PASS_SET_00B79950, &args![held, word_346]);
        let word_347 = fn_00c163a0(e, 0x5fu32);
        e.call(PASS_SET_00B80600, &args![held, word_347]);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        let word_348 = e.mem.u32(0x011f_d1e8);
        let word_349 = e.mem.u32(0x011f_c608);
        let word_350 = e.mem.u32(0x011f_ba28);
        e.mem.set_u32(0x011f_d1fc, word_348);
        let word_351 = e.mem.u32(0x011f_ae48);
        e.mem.set_u32(0x011f_c61c, word_349);
        e.mem.set_u32(0x011f_ba3c, word_350);
        e.mem.set_u32(0x011f_ae5c, word_351);
        held = switch_pass(e, held, 0x0120_23b4);
        if e.mem.u32(held + 0x18) < 6u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_359 = e.mem.u32(stage);
            let word_360 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_359 + 0x8, word_360);
            let made_361 = e.mem.u32(stage);
            e.mem.set_u32(made_361 + 0xc, 1u32);
            let made_362 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_362, 3u32, 3u32, 2u32, 1u32],
            );
            let made_363 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 3u32, made_363]);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
        }
        let word_368 = e.mem.u32(base + 0xf0);
        e.call(PASS_SET_00B79950, &args![held, word_368]);
        let word_369 = fn_00c163a0(e, 0x61u32);
        e.call(PASS_SET_00B80600, &args![held, word_369]);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        let word_370 = e.mem.u32(0x011f_d200);
        let word_371 = e.mem.u32(0x011f_c620);
        let word_372 = e.mem.u32(0x011f_ba40);
        e.mem.set_u32(0x011f_d214, word_370);
        let word_373 = e.mem.u32(0x011f_ae60);
        e.mem.set_u32(0x011f_c634, word_371);
        e.mem.set_u32(0x011f_ba54, word_372);
        e.mem.set_u32(0x011f_ae74, word_373);
        held = switch_pass(e, held, 0x0120_23b8);
        if e.mem.u32(held + 0x18) < 6u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_381 = e.mem.u32(stage);
            let word_382 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_381 + 0x8, word_382);
            let made_383 = e.mem.u32(stage);
            e.mem.set_u32(made_383 + 0xc, 1u32);
            let made_384 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_384, 3u32, 3u32, 2u32, 1u32],
            );
            let made_385 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 3u32, made_385]);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
        }
        let word_390 = e.mem.u32(base + 0xf4);
        e.call(PASS_SET_00B79950, &args![held, word_390]);
        let word_391 = fn_00c163a0(e, 0x63u32);
        e.call(PASS_SET_00B80600, &args![held, word_391]);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        let word_392 = e.mem.u32(0x011f_d218);
        let word_393 = e.mem.u32(0x011f_c638);
        e.mem.set_u32(0x011f_d22c, word_392);
        e.mem.set_u32(0x011f_c64c, word_393);
        held = switch_pass(e, held, 0x0120_23bc);
        if e.mem.u32(held + 0x18) < 6u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_401 = e.mem.u32(stage);
            let word_402 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_401 + 0x8, word_402);
            let made_403 = e.mem.u32(stage);
            e.mem.set_u32(made_403 + 0xc, 1u32);
            let made_404 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_404, 3u32, 3u32, 2u32, 1u32],
            );
            let made_405 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 3u32, made_405]);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
        }
        let word_410 = e.mem.u32(base + 0xf8);
        e.call(PASS_SET_00B79950, &args![held, word_410]);
        let word_411 = fn_00c163a0(e, 0x65u32);
        e.call(PASS_SET_00B80600, &args![held, word_411]);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        let word_412 = e.mem.u32(0x011f_d230);
        let word_413 = e.mem.u32(0x011f_c650);
        e.mem.set_u32(0x011f_d244, word_412);
        e.mem.set_u32(0x011f_c664, word_413);
        held = switch_pass(e, held, 0x0120_23c0);
        if e.mem.u32(held + 0x18) < 6u32 {
            stage_next(e, held, stage, [0u32, 1u32, 2u32, 1u32]);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_421 = e.mem.u32(stage);
            let word_422 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_421 + 0x8, word_422);
            let made_423 = e.mem.u32(stage);
            e.mem.set_u32(made_423 + 0xc, 1u32);
            let made_424 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_424, 3u32, 3u32, 2u32, 1u32],
            );
            let made_425 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 3u32, made_425]);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            stage_at(e, held, stage, [5u32, 1u32, 2u32, 1u32], 5u32);
        }
        let word_430 = e.mem.u32(base + 0xfc);
        e.call(PASS_SET_00B79950, &args![held, word_430]);
        let word_431 = fn_00c163a0(e, 0x67u32);
        e.call(PASS_SET_00B80600, &args![held, word_431]);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        let word_432 = e.mem.u32(0x011f_d248);
        let word_433 = e.mem.u32(0x011f_c668);
        e.mem.set_u32(0x011f_d25c, word_432);
        e.mem.set_u32(0x011f_c67c, word_433);
        held = switch_pass(e, held, 0x0120_23c4);
        if e.mem.u32(held + 0x18) < 8u32 {
            stage_at(e, held, stage, [0u32, 1u32, 2u32, 1u32], 0u32);
            stage_at(e, held, stage, [1u32, 1u32, 2u32, 1u32], 1u32);
            stage_at(e, held, stage, [2u32, 1u32, 2u32, 1u32], 2u32);
            stage_at(e, held, stage, [3u32, 1u32, 2u32, 1u32], 3u32);
            stage_at(e, held, stage, [4u32, 1u32, 2u32, 1u32], 4u32);
            e.call(CREATE_NEW_STAGE, &args![stage]);
            let made_444 = e.mem.u32(stage);
            e.call(
                SETUP_DEFAULT_STAGE,
                &args![made_444, 5u32, 3u32, 0u32, 1u32],
            );
            let made_445 = e.mem.u32(stage);
            let word_446 = e.mem.u32(0x011f_9534);
            e.mem.set_u32(made_445 + 0x8, word_446);
            let made_447 = e.mem.u32(stage);
            e.mem.set_u32(made_447 + 0xc, 1u32);
            let made_448 = e.mem.u32(stage);
            e.call(PASS_SET_STAGE, &args![held, 5u32, made_448]);
            stage_at(e, held, stage, [6u32, 1u32, 2u32, 1u32], 6u32);
            stage_at(e, held, stage, [7u32, 3u32, 0u32, 1u32], 7u32);
        }
        let word_453 = e.mem.u32(base + 0x100);
        e.call(PASS_SET_00B79950, &args![held, word_453]);
        let word_454 = fn_00c163a0(e, 0xc8u32);
        e.call(PASS_SET_00B80600, &args![held, word_454]);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 9u32, 1u32, 0u32]);
        set_render_state(e, held, 0xfu32, 0u32, 1u32);
        set_render_state(e, held, 7u32, 1u32, 1u32);
        set_render_state(e, held, 0x17u32, 4u32, 1u32);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        let word_455 = e.mem.u32(0x011f_d544);
        let word_456 = e.mem.u32(0x011f_c964);
        let word_457 = e.mem.u32(0x011f_bd84);
        e.mem.set_u32(0x011f_d550, word_455);
        let word_458 = e.mem.u32(0x011f_b1a4);
        e.mem.set_u32(0x011f_b1b0, word_458);
        let made_459 = e.mem.u32(stage);
        e.mem.set_u32(0x011f_c970, word_456);
        e.mem.set_u32(0x011f_bd90, word_457);
        release_stage(e, made_459);
        release_pass(e, held);
    });
}

/// `00ba53c0` (`thiscall` on the held-pass local, `RET 4`: the address of a pass global;
/// unnamed in the engine map): the counted switch of that local to the global's pass.
const POINTER_SWITCH_00BA53C0: u32 = 0x00ba_53c0;

/// `switch_pass` that also stores the new pass in the local `cell` when it switches.
fn switch_pass_cell(e: &mut Engine, held: u32, global: u32, cell: u32) -> u32 {
    if held == e.mem.u32(global) {
        return held;
    }
    release_pass(e, held);
    let next = e.mem.u32(global);
    e.mem.set_u32(cell, next);
    hold_pass(e, next);
    next
}

/// `switch_pass_cell` whose release first tests the held pass for null.
fn switch_pass_checked_cell(e: &mut Engine, held: u32, global: u32, cell: u32) -> u32 {
    if held == e.mem.u32(global) {
        return held;
    }
    if held != 0 {
        release_pass(e, held);
    }
    let next = e.mem.u32(global);
    e.mem.set_u32(cell, next);
    hold_pass(e, next);
    next
}

// Translated from 00c072c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words; called from `00bb9560` and
/// `00bb9590`): sets up the image-space shader passes from `01200ec0` on, the pass in use
/// held in a local (`held_cell`): each pass gets its stages by a mask (`00c06e10`, some
/// masks packed by `00c06dc0`), its +0x5c and +0x44 programs (counted, or through `00b79950`
/// and `00b80600`), its render states (state 0xc3 with the `float` at `01200458` or twice it,
/// an x87 `FADD` rounded to `float`) and the globals the exe writes; the later passes are
/// switched in place by `00ba53c0` on the local. Ends with `00c06f80` and the release of
/// the held pass.
// Translation incomplete: 00c072c0 - the C++ exception frame (FS:[0] chain, unwind state for the held pass) is not translated.
pub fn fn_00c072c0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(4, |e, held_cell| {
        let held_cell = held_cell.addr();
        let mut held: u32;
        e.mem.set_u32(held_cell, 0u32);
        let bits_1 = e.mem.u32(0x0120_0458);
        let wide_2 = x87_load32(bits_1);
        let word_3 = e.mem.u32(0x0120_0ec0);
        let float_4 = x87_store32(wide_2);
        held = word_3;
        e.mem.set_u32(held_cell, held);
        hold_pass(e, held);
        fn_00c06e10(e, Ptr::new(held), 0u32);
        let word_5 = e.mem.u32(0x0120_0e9c);
        let _word_6 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_5);
        let word_7 = e.mem.u32(0x0120_0cf8);
        assign_counted(e, held + 0x44, word_7);
        set_render_state(e, held, 0x16u32, 1u32, 1u32);
        set_render_state(e, held, 0xa8u32, 0u32, 1u32);
        e.mem.set_u32(0x0120_0a28, 0x8au32);
        e.mem.set_u32(0x0120_0868, 0u32);
        held = switch_pass_cell(e, held, 0x0120_0ec4, held_cell);
        fn_00c06e10(e, Ptr::new(held), 1u32);
        let word_8 = e.mem.u32(0x0120_0e9c);
        let _word_9 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_8);
        let word_10 = e.mem.u32(0x0120_0cf8);
        assign_counted(e, held + 0x44, word_10);
        set_render_state(e, held, 0xa8u32, 0u32, 1u32);
        e.mem.set_u32(0x0120_0a2c, 2u32);
        e.mem.set_u32(0x0120_086c, 0u32);
        held = switch_pass_cell(e, held, 0x0120_0ec8, held_cell);
        fn_00c06e10(e, Ptr::new(held), 1u32);
        let word_11 = e.mem.u32(0x0120_0ea0);
        let _word_12 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_11);
        let word_13 = e.mem.u32(0x0120_0cfc);
        assign_counted(e, held + 0x44, word_13);
        set_render_state(e, held, 0xa8u32, 0u32, 1u32);
        e.mem.set_u32(0x0120_0a30, 0x24u32);
        e.mem.set_u32(0x0120_0870, 0u32);
        held = switch_pass_cell(e, held, 0x0120_0ecc, held_cell);
        fn_00c06e10(e, Ptr::new(held), 1u32);
        let word_14 = e.mem.u32(0x0120_0ea4);
        let _word_15 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_14);
        let word_16 = e.mem.u32(0x0120_0cf8);
        assign_counted(e, held + 0x44, word_16);
        set_render_state(e, held, 0xa8u32, 0u32, 1u32);
        e.mem.set_u32(0x0120_0a34, 2u32);
        e.mem.set_u32(0x0120_0874, 0u32);
        held = switch_pass_cell(e, held, 0x0120_0ed0, held_cell);
        fn_00c06e10(e, Ptr::new(held), 1u32);
        let word_17 = e.mem.u32(0x0120_0ea8);
        let _word_18 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_17);
        let word_19 = e.mem.u32(0x0120_0cfc);
        assign_counted(e, held + 0x44, word_19);
        set_render_state(e, held, 0xa8u32, 0u32, 1u32);
        e.mem.set_u32(0x0120_0a38, 0x24u32);
        e.mem.set_u32(0x0120_0878, 0u32);
        held = switch_pass_cell(e, held, 0x0120_0ed4, held_cell);
        fn_00c06e10(e, Ptr::new(held), 1u32);
        let word_20 = e.mem.u32(0x0120_0eac);
        let _word_21 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_20);
        let word_22 = e.mem.u32(0x0120_0d00);
        assign_counted(e, held + 0x44, word_22);
        set_render_state(e, held, 0xa8u32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 1u32, 1u32);
        set_render_state(e, held, 0x19u32, 5u32, 1u32);
        set_render_state(e, held, 0x18u32, 0u32, 1u32);
        let wide_23 = x87_load32(float_4);
        let wide_24 = x87_add(wide_23, wide_23);
        let float_25 = x87_store32(wide_24);
        let wide_26 = x87_load32(float_25);
        let float_27 = x87_store32(wide_26);
        set_render_state(e, held, 0xc3u32, float_27, 1u32);
        e.mem.set_u32(0x0120_0a3c, 2u32);
        e.mem.set_u32(0x0120_087c, 0u32);
        held = switch_pass_cell(e, held, 0x0120_0ed8, held_cell);
        fn_00c06e10(e, Ptr::new(held), 1u32);
        let word_28 = e.mem.u32(0x0120_0eb0);
        let _word_29 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_28);
        let word_30 = e.mem.u32(0x0120_0d04);
        assign_counted(e, held + 0x44, word_30);
        set_render_state(e, held, 0xa8u32, 0u32, 1u32);
        set_render_state(e, held, 0xfu32, 1u32, 1u32);
        set_render_state(e, held, 0x19u32, 5u32, 1u32);
        set_render_state(e, held, 0x18u32, 0u32, 1u32);
        let wide_31 = x87_load32(float_25);
        let float_32 = x87_store32(wide_31);
        set_render_state(e, held, 0xc3u32, float_32, 1u32);
        e.mem.set_u32(0x0120_0a40, 0x24u32);
        e.mem.set_u32(0x0120_0880, 0u32);
        held = switch_pass_cell(e, held, 0x0120_1024, held_cell);
        fn_00c06e10(e, Ptr::new(held), 1u32);
        let word_33 = e.mem.u32(0x0120_0eb4);
        let _word_34 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_33);
        let word_35 = e.mem.u32(0x0120_0d08);
        assign_counted(e, held + 0x44, word_35);
        set_render_state(e, held, 0xa8u32, 3u32, 1u32);
        e.mem.set_u32(0x0120_0b8c, 2u32);
        e.mem.set_u32(0x0120_09cc, 0x12u32);
        held = switch_pass_cell(e, held, 0x0120_1028, held_cell);
        fn_00c06e10(e, Ptr::new(held), 1u32);
        let word_36 = e.mem.u32(0x0120_0ea4);
        let _word_37 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_36);
        let word_38 = e.mem.u32(0x0120_0d08);
        assign_counted(e, held + 0x44, word_38);
        set_render_state(e, held, 0xa8u32, 3u32, 1u32);
        e.mem.set_u32(0x0120_0b90, 2u32);
        e.mem.set_u32(0x0120_09d0, 0x12u32);
        held = switch_pass_cell(e, held, 0x0120_102c, held_cell);
        fn_00c06e10(e, Ptr::new(held), 1u32);
        let word_39 = e.mem.u32(0x0120_0eb8);
        let _word_40 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_39);
        let word_41 = e.mem.u32(0x0120_0d0c);
        assign_counted(e, held + 0x44, word_41);
        set_render_state(e, held, 0xa8u32, 3u32, 1u32);
        e.mem.set_u32(0x0120_0b94, 0x24u32);
        e.mem.set_u32(0x0120_09d4, 0x12u32);
        held = switch_pass_cell(e, held, 0x0120_1030, held_cell);
        fn_00c06e10(e, Ptr::new(held), 1u32);
        let word_42 = e.mem.u32(0x0120_0ea8);
        let _word_43 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_42);
        let word_44 = e.mem.u32(0x0120_0d0c);
        assign_counted(e, held + 0x44, word_44);
        set_render_state(e, held, 0xa8u32, 3u32, 1u32);
        e.mem.set_u32(0x0120_0b98, 0x24u32);
        e.mem.set_u32(0x0120_09d8, 0x12u32);
        held = switch_pass_cell(e, held, 0x0120_0edc, held_cell);
        fn_00c06e10(e, Ptr::new(held), 3u32);
        let word_45 = e.mem.u32(0x0120_0dd8);
        let _word_46 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_45);
        let word_47 = e.mem.u32(0x0120_0c48);
        assign_counted(e, held + 0x44, word_47);
        e.mem.set_u32(0x0120_0a44, 0x0001_0002u32);
        e.mem.set_u32(0x0120_0884, 0x412u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0ee0, held_cell);
        fn_00c06e10(e, Ptr::new(held), 3u32);
        let word_48 = e.mem.u32(0x0120_0de0);
        let _word_49 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_48);
        let word_50 = e.mem.u32(0x0120_0c48);
        assign_counted(e, held + 0x44, word_50);
        e.mem.set_u32(0x0120_0a48, 0x0001_0024u32);
        e.mem.set_u32(0x0120_0888, 0x412u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0ee4, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x33u32);
        let word_51 = e.mem.u32(0x0120_0de4);
        let _word_52 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_51);
        let word_53 = e.mem.u32(0x0120_0c50);
        assign_counted(e, held + 0x44, word_53);
        e.mem.set_u32(0x0120_0a4c, 0x0001_0002u32);
        e.mem.set_u32(0x0120_088c, 0x41eu32);
        held = switch_pass_checked_cell(e, held, 0x0120_0ee8, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x13u32);
        let word_54 = e.mem.u32(0x0120_0dd8);
        let _word_55 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_54);
        let word_56 = e.mem.u32(0x0120_0c58);
        assign_counted(e, held + 0x44, word_56);
        e.mem.set_u32(0x0120_0a50, 0x0001_0002u32);
        e.mem.set_u32(0x0120_0890, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0eec, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x13u32);
        let word_57 = e.mem.u32(0x0120_0ddc);
        let _word_58 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_57);
        let word_59 = e.mem.u32(0x0120_0c5c);
        assign_counted(e, held + 0x44, word_59);
        e.mem.set_u32(0x0120_0a54, 0x0001_0002u32);
        e.mem.set_u32(0x0120_0894, 0x496u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0ef0, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x33u32);
        let word_60 = e.mem.u32(0x0120_0dd8);
        let _word_61 = e.mem.u32(held + 0x5c);
        assign_counted(e, held + 0x5c, word_60);
        let word_62 = e.mem.u32(0x0120_0c68);
        e.call(PASS_SET_00B80600, &args![held, word_62]);
        e.mem.set_u32(0x0120_0a58, 0x0001_0002u32);
        e.mem.set_u32(0x0120_0898, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0ef4, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x73u32);
        let word_63 = e.mem.u32(0x0120_0de0);
        e.call(PASS_SET_00B79950, &args![held, word_63]);
        let word_64 = e.mem.u32(0x0120_0c68);
        e.call(PASS_SET_00B80600, &args![held, word_64]);
        e.mem.set_u32(0x0120_0a5c, 0x0001_0024u32);
        e.mem.set_u32(0x0120_089c, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0ef8, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x43u32);
        let word_65 = e.mem.u32(0x0120_0dd8);
        e.call(PASS_SET_00B79950, &args![held, word_65]);
        let word_66 = e.mem.u32(0x0120_0c70);
        e.call(PASS_SET_00B80600, &args![held, word_66]);
        e.mem.set_u32(0x0120_0a60, 0x0001_0002u32);
        e.mem.set_u32(0x0120_08a0, 0x432u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0efc, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x43u32);
        let word_67 = e.mem.u32(0x0120_0de0);
        e.call(PASS_SET_00B79950, &args![held, word_67]);
        let word_68 = e.mem.u32(0x0120_0c70);
        e.call(PASS_SET_00B80600, &args![held, word_68]);
        e.mem.set_u32(0x0120_0a64, 0x0001_0024u32);
        e.mem.set_u32(0x0120_08a4, 0x432u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f00, held_cell);
        fn_00c06e10(e, Ptr::new(held), 3u32);
        let word_69 = e.mem.u32(0x0120_0de4);
        e.call(PASS_SET_00B79950, &args![held, word_69]);
        let word_70 = e.mem.u32(0x0120_0c48);
        e.call(PASS_SET_00B80600, &args![held, word_70]);
        e.mem.set_u32(0x0120_0a68, 0x0001_0002u32);
        e.mem.set_u32(0x0120_08a8, 0x412u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f04, held_cell);
        fn_00c06e10(e, Ptr::new(held), 3u32);
        let word_71 = e.mem.u32(0x0120_0dec);
        e.call(PASS_SET_00B79950, &args![held, word_71]);
        let word_72 = e.mem.u32(0x0120_0c48);
        e.call(PASS_SET_00B80600, &args![held, word_72]);
        e.mem.set_u32(0x0120_0a6c, 0x0001_0024u32);
        e.mem.set_u32(0x0120_08ac, 0x412u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f08, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x13u32);
        let word_73 = e.mem.u32(0x0120_0de4);
        e.call(PASS_SET_00B79950, &args![held, word_73]);
        let word_74 = e.mem.u32(0x0120_0c58);
        e.call(PASS_SET_00B80600, &args![held, word_74]);
        e.mem.set_u32(0x0120_0a70, 0x0001_000au32);
        e.mem.set_u32(0x0120_08b0, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f0c, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x13u32);
        let word_75 = e.mem.u32(0x0120_0de8);
        e.call(PASS_SET_00B79950, &args![held, word_75]);
        let word_76 = e.mem.u32(0x0120_0c5c);
        e.call(PASS_SET_00B80600, &args![held, word_76]);
        e.mem.set_u32(0x0120_0a74, 0x0001_000au32);
        e.mem.set_u32(0x0120_08b4, 0x496u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f10, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x43u32);
        let word_77 = e.mem.u32(0x0120_0de4);
        e.call(PASS_SET_00B79950, &args![held, word_77]);
        let word_78 = e.mem.u32(0x0120_0c70);
        e.call(PASS_SET_00B80600, &args![held, word_78]);
        e.mem.set_u32(0x0120_0a78, 0x0001_0002u32);
        e.mem.set_u32(0x0120_08b8, 0x432u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f14, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x43u32);
        let word_79 = e.mem.u32(0x0120_0dec);
        e.call(PASS_SET_00B79950, &args![held, word_79]);
        let word_80 = e.mem.u32(0x0120_0c70);
        e.call(PASS_SET_00B80600, &args![held, word_80]);
        e.mem.set_u32(0x0120_0a7c, 0x0001_0024u32);
        e.mem.set_u32(0x0120_08bc, 0x432u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f18, held_cell);
        fn_00c06e10(e, Ptr::new(held), 3u32);
        let word_81 = e.mem.u32(0x0120_0dd8);
        e.call(PASS_SET_00B79950, &args![held, word_81]);
        let word_82 = e.mem.u32(0x0120_0c4c);
        e.call(PASS_SET_00B80600, &args![held, word_82]);
        e.mem.set_u32(0x0120_0a80, 0x0001_0002u32);
        e.mem.set_u32(0x0120_08c0, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f1c, held_cell);
        fn_00c06e10(e, Ptr::new(held), 3u32);
        let word_83 = e.mem.u32(0x0120_0de0);
        e.call(PASS_SET_00B79950, &args![held, word_83]);
        let word_84 = e.mem.u32(0x0120_0c4c);
        e.call(PASS_SET_00B80600, &args![held, word_84]);
        e.mem.set_u32(0x0120_0a84, 0x0001_0024u32);
        e.mem.set_u32(0x0120_08c4, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f20, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x33u32);
        let word_85 = e.mem.u32(0x0120_0de4);
        e.call(PASS_SET_00B79950, &args![held, word_85]);
        let word_86 = e.mem.u32(0x0120_0c54);
        e.call(PASS_SET_00B80600, &args![held, word_86]);
        e.mem.set_u32(0x0120_0a88, 0x0001_0002u32);
        e.mem.set_u32(0x0120_08c8, 0x41eu32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f24, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x13u32);
        let word_87 = e.mem.u32(0x0120_0dd8);
        e.call(PASS_SET_00B79950, &args![held, word_87]);
        let word_88 = e.mem.u32(0x0120_0c60);
        e.call(PASS_SET_00B80600, &args![held, word_88]);
        e.mem.set_u32(0x0120_0a8c, 0x0001_0002u32);
        e.mem.set_u32(0x0120_08cc, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f28, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x13u32);
        let word_89 = e.mem.u32(0x0120_0ddc);
        e.call(PASS_SET_00B79950, &args![held, word_89]);
        let word_90 = e.mem.u32(0x0120_0c64);
        e.call(PASS_SET_00B80600, &args![held, word_90]);
        e.mem.set_u32(0x0120_0a90, 0x0001_0002u32);
        e.mem.set_u32(0x0120_08d0, 0x496u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f2c, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x33u32);
        let word_91 = e.mem.u32(0x0120_0dd8);
        e.call(PASS_SET_00B79950, &args![held, word_91]);
        let word_92 = e.mem.u32(0x0120_0c6c);
        e.call(PASS_SET_00B80600, &args![held, word_92]);
        e.mem.set_u32(0x0120_0a94, 0x0001_0002u32);
        e.mem.set_u32(0x0120_08d4, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f30, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x73u32);
        let word_93 = e.mem.u32(0x0120_0de0);
        e.call(PASS_SET_00B79950, &args![held, word_93]);
        let word_94 = e.mem.u32(0x0120_0c6c);
        e.call(PASS_SET_00B80600, &args![held, word_94]);
        e.mem.set_u32(0x0120_0a98, 0x0001_0024u32);
        e.mem.set_u32(0x0120_08d8, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f34, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x43u32);
        let word_95 = e.mem.u32(0x0120_0dd8);
        e.call(PASS_SET_00B79950, &args![held, word_95]);
        let word_96 = e.mem.u32(0x0120_0c74);
        e.call(PASS_SET_00B80600, &args![held, word_96]);
        e.mem.set_u32(0x0120_0a9c, 0x0001_0002u32);
        e.mem.set_u32(0x0120_08dc, 0x436u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f38, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x43u32);
        let word_97 = e.mem.u32(0x0120_0de0);
        e.call(PASS_SET_00B79950, &args![held, word_97]);
        let word_98 = e.mem.u32(0x0120_0c74);
        e.call(PASS_SET_00B80600, &args![held, word_98]);
        e.mem.set_u32(0x0120_0aa0, 0x0001_0024u32);
        e.mem.set_u32(0x0120_08e0, 0x436u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f3c, held_cell);
        fn_00c06e10(e, Ptr::new(held), 3u32);
        let word_99 = e.mem.u32(0x0120_0de4);
        e.call(PASS_SET_00B79950, &args![held, word_99]);
        let word_100 = e.mem.u32(0x0120_0c4c);
        e.call(PASS_SET_00B80600, &args![held, word_100]);
        e.mem.set_u32(0x0120_0aa4, 0x0001_0002u32);
        e.mem.set_u32(0x0120_08e4, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f40, held_cell);
        fn_00c06e10(e, Ptr::new(held), 3u32);
        let word_101 = e.mem.u32(0x0120_0dec);
        e.call(PASS_SET_00B79950, &args![held, word_101]);
        let word_102 = e.mem.u32(0x0120_0c4c);
        e.call(PASS_SET_00B80600, &args![held, word_102]);
        e.mem.set_u32(0x0120_0aa8, 0x0001_0024u32);
        e.mem.set_u32(0x0120_08e8, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f44, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x13u32);
        let word_103 = e.mem.u32(0x0120_0de4);
        e.call(PASS_SET_00B79950, &args![held, word_103]);
        let word_104 = e.mem.u32(0x0120_0c60);
        e.call(PASS_SET_00B80600, &args![held, word_104]);
        e.mem.set_u32(0x0120_0aac, 0x0001_000au32);
        e.mem.set_u32(0x0120_08ec, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f48, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x13u32);
        let word_105 = e.mem.u32(0x0120_0de8);
        e.call(PASS_SET_00B79950, &args![held, word_105]);
        let word_106 = e.mem.u32(0x0120_0c64);
        e.call(PASS_SET_00B80600, &args![held, word_106]);
        e.mem.set_u32(0x0120_0ab0, 0x0001_000au32);
        e.mem.set_u32(0x0120_08f0, 0x496u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f4c, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x43u32);
        let word_107 = e.mem.u32(0x0120_0de4);
        e.call(PASS_SET_00B79950, &args![held, word_107]);
        let word_108 = e.mem.u32(0x0120_0c74);
        e.call(PASS_SET_00B80600, &args![held, word_108]);
        e.mem.set_u32(0x0120_0ab4, 0x0001_0002u32);
        e.mem.set_u32(0x0120_08f4, 0x436u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f50, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x43u32);
        let word_109 = e.mem.u32(0x0120_0dec);
        e.call(PASS_SET_00B79950, &args![held, word_109]);
        let word_110 = e.mem.u32(0x0120_0c74);
        e.call(PASS_SET_00B80600, &args![held, word_110]);
        e.mem.set_u32(0x0120_0ab8, 0x0001_0024u32);
        e.mem.set_u32(0x0120_08f8, 0x436u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f54, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0xfu32);
        let word_111 = e.mem.u32(0x0120_0df0);
        e.call(PASS_SET_00B79950, &args![held, word_111]);
        let word_112 = e.mem.u32(0x0120_0c78);
        e.call(PASS_SET_00B80600, &args![held, word_112]);
        e.mem.set_u32(0x0120_0abc, 0x0001_4002u32);
        e.mem.set_u32(0x0120_08fc, 0x412u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f58, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0xfu32);
        let word_113 = e.mem.u32(0x0120_0df8);
        e.call(PASS_SET_00B79950, &args![held, word_113]);
        let word_114 = e.mem.u32(0x0120_0c78);
        e.call(PASS_SET_00B80600, &args![held, word_114]);
        e.mem.set_u32(0x0120_0ac0, 0x0001_4024u32);
        e.mem.set_u32(0x0120_0900, 0x412u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f5c, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x3fu32);
        let word_115 = e.mem.u32(0x0120_0dfc);
        e.call(PASS_SET_00B79950, &args![held, word_115]);
        let word_116 = e.mem.u32(0x0120_0c80);
        e.call(PASS_SET_00B80600, &args![held, word_116]);
        e.mem.set_u32(0x0120_0ac4, 0x0001_4002u32);
        e.mem.set_u32(0x0120_0904, 0x41eu32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f60, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x1fu32);
        let word_117 = e.mem.u32(0x0120_0df0);
        e.call(PASS_SET_00B79950, &args![held, word_117]);
        let word_118 = e.mem.u32(0x0120_0c88);
        e.call(PASS_SET_00B80600, &args![held, word_118]);
        e.mem.set_u32(0x0120_0ac8, 0x0001_4002u32);
        e.mem.set_u32(0x0120_0908, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f64, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x1fu32);
        let word_119 = e.mem.u32(0x0120_0df4);
        e.call(PASS_SET_00B79950, &args![held, word_119]);
        let word_120 = e.mem.u32(0x0120_0c8c);
        e.call(PASS_SET_00B80600, &args![held, word_120]);
        e.mem.set_u32(0x0120_0acc, 0x0001_4002u32);
        e.mem.set_u32(0x0120_090c, 0x496u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f68, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x3fu32);
        let word_121 = e.mem.u32(0x0120_0df0);
        e.call(PASS_SET_00B79950, &args![held, word_121]);
        let word_122 = e.mem.u32(0x0120_0c98);
        e.call(PASS_SET_00B80600, &args![held, word_122]);
        e.mem.set_u32(0x0120_0ad0, 0x0001_4002u32);
        e.mem.set_u32(0x0120_0910, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f6c, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x7fu32);
        let word_123 = e.mem.u32(0x0120_0df8);
        e.call(PASS_SET_00B79950, &args![held, word_123]);
        let word_124 = e.mem.u32(0x0120_0c98);
        e.call(PASS_SET_00B80600, &args![held, word_124]);
        e.mem.set_u32(0x0120_0ad4, 0x0001_4024u32);
        e.mem.set_u32(0x0120_0914, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f70, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x4fu32);
        let word_125 = e.mem.u32(0x0120_0df0);
        e.call(PASS_SET_00B79950, &args![held, word_125]);
        let word_126 = e.mem.u32(0x0120_0ca0);
        e.call(PASS_SET_00B80600, &args![held, word_126]);
        e.mem.set_u32(0x0120_0ad8, 0x0001_4002u32);
        e.mem.set_u32(0x0120_0918, 0x432u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f74, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x4fu32);
        let word_127 = e.mem.u32(0x0120_0df8);
        e.call(PASS_SET_00B79950, &args![held, word_127]);
        let word_128 = e.mem.u32(0x0120_0ca0);
        e.call(PASS_SET_00B80600, &args![held, word_128]);
        e.mem.set_u32(0x0120_0adc, 0x0001_4024u32);
        e.mem.set_u32(0x0120_091c, 0x432u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f78, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0xfu32);
        let word_129 = e.mem.u32(0x0120_0dfc);
        e.call(PASS_SET_00B79950, &args![held, word_129]);
        let word_130 = e.mem.u32(0x0120_0c78);
        e.call(PASS_SET_00B80600, &args![held, word_130]);
        e.mem.set_u32(0x0120_0ae0, 0x0001_4002u32);
        e.mem.set_u32(0x0120_0920, 0x412u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f7c, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0xfu32);
        let word_131 = e.mem.u32(0x0120_0e04);
        e.call(PASS_SET_00B79950, &args![held, word_131]);
        let word_132 = e.mem.u32(0x0120_0c78);
        e.call(PASS_SET_00B80600, &args![held, word_132]);
        e.mem.set_u32(0x0120_0ae4, 0x0001_4024u32);
        e.mem.set_u32(0x0120_0924, 0x412u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f80, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x1fu32);
        let word_133 = e.mem.u32(0x0120_0dfc);
        e.call(PASS_SET_00B79950, &args![held, word_133]);
        let word_134 = e.mem.u32(0x0120_0c88);
        e.call(PASS_SET_00B80600, &args![held, word_134]);
        e.mem.set_u32(0x0120_0ae8, 0x0001_4002u32);
        e.mem.set_u32(0x0120_0928, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f84, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x1fu32);
        let word_135 = e.mem.u32(0x0120_0e00);
        e.call(PASS_SET_00B79950, &args![held, word_135]);
        let word_136 = e.mem.u32(0x0120_0c8c);
        e.call(PASS_SET_00B80600, &args![held, word_136]);
        e.mem.set_u32(0x0120_0aec, 0x0001_4002u32);
        e.mem.set_u32(0x0120_092c, 0x496u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f88, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x4fu32);
        let word_137 = e.mem.u32(0x0120_0dfc);
        e.call(PASS_SET_00B79950, &args![held, word_137]);
        let word_138 = e.mem.u32(0x0120_0ca0);
        e.call(PASS_SET_00B80600, &args![held, word_138]);
        e.mem.set_u32(0x0120_0af0, 0x0001_4002u32);
        e.mem.set_u32(0x0120_0930, 0x432u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f8c, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x4fu32);
        let word_139 = e.mem.u32(0x0120_0e04);
        e.call(PASS_SET_00B79950, &args![held, word_139]);
        let word_140 = e.mem.u32(0x0120_0ca0);
        e.call(PASS_SET_00B80600, &args![held, word_140]);
        e.mem.set_u32(0x0120_0af4, 0x0001_4024u32);
        e.mem.set_u32(0x0120_0934, 0x432u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f90, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0xfu32);
        let word_141 = e.mem.u32(0x0120_0df0);
        e.call(PASS_SET_00B79950, &args![held, word_141]);
        let word_142 = e.mem.u32(0x0120_0c7c);
        e.call(PASS_SET_00B80600, &args![held, word_142]);
        e.mem.set_u32(0x0120_0af8, 0x0001_4002u32);
        e.mem.set_u32(0x0120_0938, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f94, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0xfu32);
        let word_143 = e.mem.u32(0x0120_0df8);
        e.call(PASS_SET_00B79950, &args![held, word_143]);
        let word_144 = e.mem.u32(0x0120_0c7c);
        e.call(PASS_SET_00B80600, &args![held, word_144]);
        e.mem.set_u32(0x0120_0afc, 0x0001_4024u32);
        e.mem.set_u32(0x0120_093c, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f98, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x3fu32);
        let word_145 = e.mem.u32(0x0120_0dfc);
        e.call(PASS_SET_00B79950, &args![held, word_145]);
        let word_146 = e.mem.u32(0x0120_0c84);
        e.call(PASS_SET_00B80600, &args![held, word_146]);
        e.mem.set_u32(0x0120_0b00, 0x0001_4002u32);
        e.mem.set_u32(0x0120_0940, 0x41eu32);
        held = switch_pass_checked_cell(e, held, 0x0120_0f9c, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x1fu32);
        let word_147 = e.mem.u32(0x0120_0df0);
        e.call(PASS_SET_00B79950, &args![held, word_147]);
        let word_148 = e.mem.u32(0x0120_0c90);
        e.call(PASS_SET_00B80600, &args![held, word_148]);
        e.mem.set_u32(0x0120_0b04, 0x0001_4002u32);
        e.mem.set_u32(0x0120_0944, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0fa0, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x1fu32);
        let word_149 = e.mem.u32(0x0120_0df4);
        e.call(PASS_SET_00B79950, &args![held, word_149]);
        let word_150 = e.mem.u32(0x0120_0c94);
        e.call(PASS_SET_00B80600, &args![held, word_150]);
        e.mem.set_u32(0x0120_0b08, 0x0001_4002u32);
        e.mem.set_u32(0x0120_0948, 0x496u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0fa4, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x3fu32);
        let word_151 = e.mem.u32(0x0120_0df0);
        e.call(PASS_SET_00B79950, &args![held, word_151]);
        let word_152 = e.mem.u32(0x0120_0c9c);
        e.call(PASS_SET_00B80600, &args![held, word_152]);
        e.mem.set_u32(0x0120_0b0c, 0x0001_4002u32);
        e.mem.set_u32(0x0120_094c, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0fa8, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x7fu32);
        let word_153 = e.mem.u32(0x0120_0df8);
        e.call(PASS_SET_00B79950, &args![held, word_153]);
        let word_154 = e.mem.u32(0x0120_0c9c);
        e.call(PASS_SET_00B80600, &args![held, word_154]);
        e.mem.set_u32(0x0120_0b10, 0x0001_4024u32);
        e.mem.set_u32(0x0120_0950, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0fac, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x4fu32);
        let word_155 = e.mem.u32(0x0120_0df0);
        e.call(PASS_SET_00B79950, &args![held, word_155]);
        let word_156 = e.mem.u32(0x0120_0ca4);
        e.call(PASS_SET_00B80600, &args![held, word_156]);
        e.mem.set_u32(0x0120_0b14, 0x0001_4002u32);
        e.mem.set_u32(0x0120_0954, 0x436u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0fb0, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x4fu32);
        let word_157 = e.mem.u32(0x0120_0df8);
        e.call(PASS_SET_00B79950, &args![held, word_157]);
        let word_158 = e.mem.u32(0x0120_0ca4);
        e.call(PASS_SET_00B80600, &args![held, word_158]);
        e.mem.set_u32(0x0120_0b18, 0x0001_4024u32);
        e.mem.set_u32(0x0120_0958, 0x436u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0fb4, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0xfu32);
        let word_159 = e.mem.u32(0x0120_0dfc);
        e.call(PASS_SET_00B79950, &args![held, word_159]);
        let word_160 = e.mem.u32(0x0120_0c7c);
        e.call(PASS_SET_00B80600, &args![held, word_160]);
        e.mem.set_u32(0x0120_0b1c, 0x0001_4002u32);
        e.mem.set_u32(0x0120_095c, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0fb8, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0xfu32);
        let word_161 = e.mem.u32(0x0120_0e04);
        e.call(PASS_SET_00B79950, &args![held, word_161]);
        let word_162 = e.mem.u32(0x0120_0c7c);
        e.call(PASS_SET_00B80600, &args![held, word_162]);
        e.mem.set_u32(0x0120_0b20, 0x0001_4024u32);
        e.mem.set_u32(0x0120_0960, 0x416u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0fbc, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x1fu32);
        let word_163 = e.mem.u32(0x0120_0dfc);
        e.call(PASS_SET_00B79950, &args![held, word_163]);
        let word_164 = e.mem.u32(0x0120_0c90);
        e.call(PASS_SET_00B80600, &args![held, word_164]);
        e.mem.set_u32(0x0120_0964, 0x416u32);
        e.mem.set_u32(0x0120_0b24, 0x0001_4002u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0fc0, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x1fu32);
        let word_165 = e.mem.u32(0x0120_0e00);
        e.call(PASS_SET_00B79950, &args![held, word_165]);
        let word_166 = e.mem.u32(0x0120_0c94);
        e.call(PASS_SET_00B80600, &args![held, word_166]);
        e.mem.set_u32(0x0120_0b28, 0x0001_4002u32);
        e.mem.set_u32(0x0120_0968, 0x496u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0fc4, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x4fu32);
        let word_167 = e.mem.u32(0x0120_0dfc);
        e.call(PASS_SET_00B79950, &args![held, word_167]);
        let word_168 = e.mem.u32(0x0120_0ca4);
        e.call(PASS_SET_00B80600, &args![held, word_168]);
        e.mem.set_u32(0x0120_0b2c, 0x0001_4002u32);
        e.mem.set_u32(0x0120_096c, 0x436u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0fc8, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0x4fu32);
        let word_169 = e.mem.u32(0x0120_0e04);
        e.call(PASS_SET_00B79950, &args![held, word_169]);
        let word_170 = e.mem.u32(0x0120_0ca4);
        e.call(PASS_SET_00B80600, &args![held, word_170]);
        e.mem.set_u32(0x0120_0b30, 0x0001_4024u32);
        e.mem.set_u32(0x0120_0970, 0x436u32);
        held = switch_pass_checked_cell(e, held, 0x0120_0fd4, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0xbu32);
        let word_171 = e.mem.u32(held + 0x24);
        let word_172 = e.mem.u32(word_171 + 0x4);
        e.call(STAGE_STEP_00E7DE90, &args![word_172, 0u32]);
        let word_173 = e.mem.u32(0x0120_0e08);
        e.call(PASS_SET_00B79950, &args![held, word_173]);
        let word_174 = e.mem.u32(0x0120_0ca8);
        e.call(PASS_SET_00B80600, &args![held, word_174]);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        e.mem.set_u32(0x0120_0b3c, 0x0003_0082u32);
        e.mem.set_u32(0x0120_097c, 0x110u32);
        held = switch_pass_cell(e, held, 0x0120_0fd8, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0xbu32);
        let word_175 = e.mem.u32(held + 0x24);
        let word_176 = e.mem.u32(word_175 + 0x4);
        e.call(STAGE_STEP_00E7DE90, &args![word_176, 0u32]);
        let word_177 = e.mem.u32(0x0120_0e08);
        e.call(PASS_SET_00B79950, &args![held, word_177]);
        let word_178 = e.mem.u32(0x0120_0cac);
        e.call(PASS_SET_00B80600, &args![held, word_178]);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        e.mem.set_u32(0x0120_0b40, 0x0003_0082u32);
        e.mem.set_u32(0x0120_0980, 0x110u32);
        held = switch_pass_cell(e, held, 0x0120_0fdc, held_cell);
        fn_00c06e10(e, Ptr::new(held), 0xbu32);
        let word_179 = e.mem.u32(held + 0x24);
        let word_180 = e.mem.u32(word_179 + 0x4);
        e.call(STAGE_STEP_00E7DE90, &args![word_180, 0u32]);
        let word_181 = e.mem.u32(0x0120_0e0c);
        e.call(PASS_SET_00B79950, &args![held, word_181]);
        let word_182 = e.mem.u32(0x0120_0ca8);
        e.call(PASS_SET_00B80600, &args![held, word_182]);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        e.mem.set_u32(0x0120_0b44, 0x0003_0082u32);
        e.mem.set_u32(0x0120_0984, 0x110u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_0fe0u32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), 0xbu32);
        let word_183 = e.mem.u32(held + 0x24);
        let word_184 = e.mem.u32(word_183 + 0x4);
        e.call(STAGE_STEP_00E7DE90, &args![word_184, 0u32]);
        let word_185 = e.mem.u32(0x0120_0e0c);
        e.call(PASS_SET_00B79950, &args![held, word_185]);
        let word_186 = e.mem.u32(0x0120_0cac);
        e.call(PASS_SET_00B80600, &args![held, word_186]);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        e.mem.set_u32(0x0120_0b48, 0x0003_0082u32);
        e.mem.set_u32(0x0120_0988, 0x110u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_0fe4u32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), 0xbu32);
        let word_187 = e.mem.u32(held + 0x24);
        let word_188 = e.mem.u32(word_187 + 0x4);
        e.call(STAGE_STEP_00E7DE90, &args![word_188, 0u32]);
        let word_189 = e.mem.u32(0x0120_0e10);
        e.call(PASS_SET_00B79950, &args![held, word_189]);
        let word_190 = e.mem.u32(0x0120_0ca8);
        e.call(PASS_SET_00B80600, &args![held, word_190]);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        e.mem.set_u32(0x0120_0b4c, 0x0003_8024u32);
        e.mem.set_u32(0x0120_098c, 0x110u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_0fe8u32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), 0xbu32);
        let word_191 = e.mem.u32(held + 0x24);
        let word_192 = e.mem.u32(word_191 + 0x4);
        e.call(STAGE_STEP_00E7DE90, &args![word_192, 0u32]);
        let word_193 = e.mem.u32(0x0120_0e14);
        e.call(PASS_SET_00B79950, &args![held, word_193]);
        let word_194 = e.mem.u32(0x0120_0ca8);
        e.call(PASS_SET_00B80600, &args![held, word_194]);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        e.mem.set_u32(0x0120_0b50, 0x0003_8024u32);
        e.mem.set_u32(0x0120_0990, 0x110u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_0fecu32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), 0xbu32);
        let word_195 = e.mem.u32(held + 0x24);
        let word_196 = e.mem.u32(word_195 + 0x4);
        e.call(STAGE_STEP_00E7DE90, &args![word_196, 0u32]);
        let word_197 = e.mem.u32(0x0120_0e18);
        e.call(PASS_SET_00B79950, &args![held, word_197]);
        let word_198 = e.mem.u32(0x0120_0cb0);
        e.call(PASS_SET_00B80600, &args![held, word_198]);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 2u32, 2u32, 0u32]);
        e.mem.set_u32(0x0120_0b54, 0x0003_2082u32);
        e.mem.set_u32(0x0120_0994, 0x510u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1060u32]);
        let word_199 = e.mem.u32(0x0120_0e1c);
        held = e.mem.u32(held_cell);
        e.call(PASS_SET_00B79950, &args![held, word_199]);
        let word_200 = e.mem.u32(0x0120_0cb4);
        e.call(PASS_SET_00B80600, &args![held, word_200]);
        set_render_state(e, held, 0xa8u32, 3u32, 1u32);
        let wide_201 = x87_load32(float_4);
        let float_202 = x87_store32(wide_201);
        set_render_state(e, held, 0xc3u32, float_202, 1u32);
        e.mem.set_u32(0x0120_0bc8, 0x8au32);
        e.mem.set_u32(0x0120_0a08, 0x200u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1064u32]);
        let word_203 = e.mem.u32(0x0120_0e20);
        held = e.mem.u32(held_cell);
        e.call(PASS_SET_00B79950, &args![held, word_203]);
        let word_204 = e.mem.u32(0x0120_0cb4);
        e.call(PASS_SET_00B80600, &args![held, word_204]);
        set_render_state(e, held, 0xa8u32, 3u32, 1u32);
        let wide_205 = x87_load32(float_4);
        let float_206 = x87_store32(wide_205);
        set_render_state(e, held, 0xc3u32, float_206, 1u32);
        e.mem.set_u32(0x0120_0bcc, 0x34u32);
        e.mem.set_u32(0x0120_0a0c, 0x200u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1068u32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), 1u32);
        let word_207 = e.mem.u32(0x0120_0e24);
        e.call(PASS_SET_00B79950, &args![held, word_207]);
        let word_208 = e.mem.u32(0x0120_0cb8);
        e.call(PASS_SET_00B80600, &args![held, word_208]);
        set_render_state(e, held, 0xa8u32, 3u32, 1u32);
        let wide_209 = x87_load32(float_4);
        let float_210 = x87_store32(wide_209);
        set_render_state(e, held, 0xc3u32, float_210, 1u32);
        e.mem.set_u32(0x0120_0bd0, 0x8au32);
        e.mem.set_u32(0x0120_0a10, 0x200u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_106cu32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), 1u32);
        let word_211 = e.mem.u32(0x0120_0e28);
        e.call(PASS_SET_00B79950, &args![held, word_211]);
        let word_212 = e.mem.u32(0x0120_0cb8);
        e.call(PASS_SET_00B80600, &args![held, word_212]);
        set_render_state(e, held, 0xa8u32, 3u32, 1u32);
        let wide_213 = x87_load32(float_4);
        let float_214 = x87_store32(wide_213);
        set_render_state(e, held, 0xc3u32, float_214, 1u32);
        e.mem.set_u32(0x0120_0bd4, 0x34u32);
        e.mem.set_u32(0x0120_0a14, 0x200u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1070u32]);
        let word_215 = e.mem.u32(0x0120_0e2c);
        held = e.mem.u32(held_cell);
        e.call(PASS_SET_00B79950, &args![held, word_215]);
        let word_216 = e.mem.u32(0x0120_0cb4);
        e.call(PASS_SET_00B80600, &args![held, word_216]);
        set_render_state(e, held, 0xa8u32, 3u32, 1u32);
        let wide_217 = x87_load32(float_4);
        let float_218 = x87_store32(wide_217);
        set_render_state(e, held, 0xc3u32, float_218, 1u32);
        e.mem.set_u32(0x0120_0bd8, 0x8au32);
        e.mem.set_u32(0x0120_0a18, 0x200u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1074u32]);
        let word_219 = e.mem.u32(0x0120_0e30);
        held = e.mem.u32(held_cell);
        e.call(PASS_SET_00B79950, &args![held, word_219]);
        let word_220 = e.mem.u32(0x0120_0cb4);
        e.call(PASS_SET_00B80600, &args![held, word_220]);
        set_render_state(e, held, 0xa8u32, 3u32, 1u32);
        let wide_221 = x87_load32(float_4);
        let float_222 = x87_store32(wide_221);
        set_render_state(e, held, 0xc3u32, float_222, 1u32);
        e.mem.set_u32(0x0120_0bdc, 0x34u32);
        e.mem.set_u32(0x0120_0a1c, 0x200u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1078u32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), 1u32);
        let word_223 = e.mem.u32(0x0120_0e34);
        e.call(PASS_SET_00B79950, &args![held, word_223]);
        let word_224 = e.mem.u32(0x0120_0cb8);
        e.call(PASS_SET_00B80600, &args![held, word_224]);
        set_render_state(e, held, 0xa8u32, 3u32, 1u32);
        let wide_225 = x87_load32(float_4);
        let float_226 = x87_store32(wide_225);
        set_render_state(e, held, 0xc3u32, float_226, 1u32);
        e.mem.set_u32(0x0120_0be0, 0x8au32);
        e.mem.set_u32(0x0120_0a20, 0x200u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_107cu32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), 1u32);
        let word_227 = e.mem.u32(0x0120_0e38);
        e.call(PASS_SET_00B79950, &args![held, word_227]);
        let word_228 = e.mem.u32(0x0120_0cb8);
        e.call(PASS_SET_00B80600, &args![held, word_228]);
        set_render_state(e, held, 0xa8u32, 3u32, 1u32);
        let wide_229 = x87_load32(float_4);
        let float_230 = x87_store32(wide_229);
        set_render_state(e, held, 0xc3u32, float_230, 1u32);
        e.mem.set_u32(0x0120_0be4, 0x34u32);
        e.mem.set_u32(0x0120_0a24, 0x200u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_0ff0u32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), 5u32);
        let word_231 = e.mem.u32(held + 0x24);
        let word_232 = e.mem.u32(word_231 + 0x8);
        e.call(STAGE_STEP_00E7DE90, &args![word_232, 0u32]);
        let word_233 = e.mem.u32(0x011f_91b0);
        if word_233 == 1u32 || word_233 == 2u32 {
            let word_234 = e.mem.u32(held + 0x24);
            let word_235 = e.mem.u32(word_234 + 0x8);
            e.call(STAGE_SET_FILTER_MODE, &args![word_235, 0u32]);
        }
        let word_236 = e.mem.u32(0x0120_0e3c);
        e.call(PASS_SET_00B79950, &args![held, word_236]);
        let word_237 = e.mem.u32(0x0120_0cbc);
        e.call(PASS_SET_00B80600, &args![held, word_237]);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        let wide_238 = x87_load32(float_4);
        let float_239 = x87_store32(wide_238);
        set_render_state(e, held, 0xc3u32, float_239, 1u32);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 9u32, 1u32, 0u32]);
        set_render_state(e, held, 0x98u32, 0x3fu32, 1u32);
        e.mem.set_u32(0x0120_0b58, 0x0001_0082u32);
        e.mem.set_u32(0x0120_0998, 0x410u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_0ff4u32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), 5u32);
        let word_240 = e.mem.u32(held + 0x24);
        let word_241 = e.mem.u32(word_240 + 0x8);
        e.call(STAGE_STEP_00E7DE90, &args![word_241, 0u32]);
        let word_242 = e.mem.u32(0x011f_91b0);
        if word_242 == 1u32 || word_242 == 2u32 {
            let word_243 = e.mem.u32(held + 0x24);
            let word_244 = e.mem.u32(word_243 + 0x8);
            e.call(STAGE_SET_FILTER_MODE, &args![word_244, 0u32]);
        }
        let word_245 = e.mem.u32(0x0120_0e40);
        e.call(PASS_SET_00B79950, &args![held, word_245]);
        let word_246 = e.mem.u32(0x0120_0cbc);
        e.call(PASS_SET_00B80600, &args![held, word_246]);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        let wide_247 = x87_load32(float_4);
        let float_248 = x87_store32(wide_247);
        set_render_state(e, held, 0xc3u32, float_248, 1u32);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 9u32, 1u32, 0u32]);
        set_render_state(e, held, 0x98u32, 0x3fu32, 1u32);
        e.mem.set_u32(0x0120_0b5c, 0x0001_0082u32);
        e.mem.set_u32(0x0120_099c, 0x410u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_0ff8u32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), 5u32);
        let word_249 = e.mem.u32(held + 0x24);
        let word_250 = e.mem.u32(word_249 + 0x8);
        e.call(STAGE_STEP_00E7DE90, &args![word_250, 0u32]);
        let word_251 = e.mem.u32(0x011f_91b0);
        if word_251 == 1u32 || word_251 == 2u32 {
            let word_252 = e.mem.u32(held + 0x24);
            let word_253 = e.mem.u32(word_252 + 0x8);
            e.call(STAGE_SET_FILTER_MODE, &args![word_253, 0u32]);
        }
        let word_254 = e.mem.u32(0x0120_0e44);
        e.call(PASS_SET_00B79950, &args![held, word_254]);
        let word_255 = e.mem.u32(0x0120_0cbc);
        e.call(PASS_SET_00B80600, &args![held, word_255]);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        let wide_256 = x87_load32(float_4);
        let float_257 = x87_store32(wide_256);
        set_render_state(e, held, 0xc3u32, float_257, 1u32);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 9u32, 1u32, 0u32]);
        set_render_state(e, held, 0x98u32, 0x3fu32, 1u32);
        e.mem.set_u32(0x0120_0b60, 0x0001_00a4u32);
        e.mem.set_u32(0x0120_09a0, 0x410u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_0ffcu32]);
        let word_258 = fn_00c06dc0(e, 1u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        held = e.mem.u32(held_cell);
        let byte_259 = word_258 & 0xff;
        fn_00c06e10(e, Ptr::new(held), byte_259);
        let word_260 = e.mem.u32(held + 0x24);
        let word_261 = e.mem.u32(word_260 + 0x8);
        e.call(STAGE_STEP_00E7DE90, &args![word_261, 0u32]);
        let word_262 = e.mem.u32(0x011f_91b0);
        if word_262 == 1u32 || word_262 == 2u32 {
            let word_263 = e.mem.u32(held + 0x24);
            let word_264 = e.mem.u32(word_263 + 0x8);
            e.call(STAGE_SET_FILTER_MODE, &args![word_264, 0u32]);
        }
        let word_265 = e.mem.u32(0x0120_0e48);
        e.call(PASS_SET_00B79950, &args![held, word_265]);
        let word_266 = e.mem.u32(0x0120_0cbc);
        e.call(PASS_SET_00B80600, &args![held, word_266]);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        let wide_267 = x87_load32(float_4);
        let float_268 = x87_store32(wide_267);
        set_render_state(e, held, 0xc3u32, float_268, 1u32);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 9u32, 1u32, 0u32]);
        set_render_state(e, held, 0x98u32, 0x3fu32, 1u32);
        e.mem.set_u32(0x0120_0b64, 0x0001_00a4u32);
        e.mem.set_u32(0x0120_09a4, 0x410u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1010u32]);
        let word_269 = fn_00c06dc0(e, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        held = e.mem.u32(held_cell);
        let byte_270 = word_269 & 0xff;
        fn_00c06e10(e, Ptr::new(held), byte_270);
        let word_271 = e.mem.u32(0x0120_0e5c);
        e.call(PASS_SET_00B79950, &args![held, word_271]);
        let word_272 = e.mem.u32(0x0120_0cc8);
        e.call(PASS_SET_00B80600, &args![held, word_272]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        e.mem.set_u32(0x0120_0b78, 0xau32);
        e.mem.set_u32(0x0120_09b8, 0x40u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1014u32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), byte_270);
        let word_273 = e.mem.u32(0x0120_0e60);
        e.call(PASS_SET_00B79950, &args![held, word_273]);
        let word_274 = e.mem.u32(0x0120_0ccc);
        e.call(PASS_SET_00B80600, &args![held, word_274]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        e.mem.set_u32(0x0120_0b7c, 0x34u32);
        e.mem.set_u32(0x0120_09bc, 0x40u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1018u32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), byte_270);
        let word_275 = e.mem.u32(0x0120_0e64);
        e.call(PASS_SET_00B79950, &args![held, word_275]);
        let word_276 = e.mem.u32(0x0120_0cd0);
        e.call(PASS_SET_00B80600, &args![held, word_276]);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 5u32, 6u32, 0u32]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        e.mem.set_u32(0x0120_0b80, 0x140au32);
        e.mem.set_u32(0x0120_09c0, 0x40u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_101cu32]);
        let word_277 = e.mem.u32(0x0120_0e68);
        held = e.mem.u32(held_cell);
        e.call(PASS_SET_00B79950, &args![held, word_277]);
        let word_278 = e.mem.u32(0x0120_0cd4);
        e.call(PASS_SET_00B80600, &args![held, word_278]);
        set_render_state(e, held, 7u32, 0u32, 1u32);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        e.mem.set_u32(0x0120_0b84, 2u32);
        e.mem.set_u32(0x0120_09c4, 0u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1020u32]);
        let word_279 = e.mem.u32(0x0120_0e6c);
        held = e.mem.u32(held_cell);
        e.call(PASS_SET_00B79950, &args![held, word_279]);
        let word_280 = e.mem.u32(0x0120_0cd8);
        e.call(PASS_SET_00B80600, &args![held, word_280]);
        set_render_state(e, held, 7u32, 0u32, 1u32);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        e.mem.set_u32(0x0120_0b88, 0x24u32);
        e.mem.set_u32(0x0120_09c8, 0u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_0fccu32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), byte_270);
        let word_281 = e.mem.u32(0x0120_0dd8);
        e.call(PASS_SET_00B79950, &args![held, word_281]);
        let word_282 = e.mem.u32(0x0120_0cdc);
        e.call(PASS_SET_00B80600, &args![held, word_282]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        e.mem.set_u32(0x0120_0b34, 0x0001_0002u32);
        e.mem.set_u32(0x0120_0974, 0x12u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_0fd0u32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), byte_270);
        let word_283 = e.mem.u32(0x0120_0de4);
        e.call(PASS_SET_00B79950, &args![held, word_283]);
        let word_284 = e.mem.u32(0x0120_0cdc);
        e.call(PASS_SET_00B80600, &args![held, word_284]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        e.mem.set_u32(0x0120_0b38, 0x0001_0002u32);
        e.mem.set_u32(0x0120_0978, 0x12u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1034u32]);
        let word_285 = e.mem.u32(0x0120_0e70);
        held = e.mem.u32(held_cell);
        e.call(PASS_SET_00B79950, &args![held, word_285]);
        let word_286 = e.mem.u32(0x0120_0ce0);
        e.call(PASS_SET_00B80600, &args![held, word_286]);
        e.mem.set_u32(0x0120_0b9c, 0x80au32);
        e.mem.set_u32(0x0120_09dc, 0u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1038u32]);
        let word_287 = e.mem.u32(0x0120_0e74);
        held = e.mem.u32(held_cell);
        e.call(PASS_SET_00B79950, &args![held, word_287]);
        let word_288 = e.mem.u32(0x0120_0ce0);
        e.call(PASS_SET_00B80600, &args![held, word_288]);
        e.mem.set_u32(0x0120_0ba0, 0x80au32);
        e.mem.set_u32(0x0120_09e0, 0u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_103cu32]);
        let word_289 = e.mem.u32(0x0120_0e78);
        held = e.mem.u32(held_cell);
        e.call(PASS_SET_00B79950, &args![held, word_289]);
        let word_290 = e.mem.u32(0x0120_0ce0);
        e.call(PASS_SET_00B80600, &args![held, word_290]);
        e.mem.set_u32(0x0120_0ba4, 0x834u32);
        e.mem.set_u32(0x0120_09e4, 0u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1040u32]);
        let word_291 = e.mem.u32(0x0120_0e7c);
        held = e.mem.u32(held_cell);
        e.call(PASS_SET_00B79950, &args![held, word_291]);
        let word_292 = e.mem.u32(0x0120_0ce0);
        e.call(PASS_SET_00B80600, &args![held, word_292]);
        e.mem.set_u32(0x0120_0ba8, 0x834u32);
        e.mem.set_u32(0x0120_09e8, 0u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1044u32]);
        let word_293 = e.mem.u32(0x0120_0e80);
        held = e.mem.u32(held_cell);
        e.call(PASS_SET_00B79950, &args![held, word_293]);
        let word_294 = e.mem.u32(0x0120_0ce4);
        e.call(PASS_SET_00B80600, &args![held, word_294]);
        let wide_295 = x87_load32(float_4);
        let float_296 = x87_store32(wide_295);
        set_render_state(e, held, 0xc3u32, float_296, 1u32);
        set_render_state(e, held, 0x16u32, 1u32, 1u32);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        e.mem.set_u32(0x0120_0bac, 0xb0au32);
        e.mem.set_u32(0x0120_09ec, 0u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1048u32]);
        let word_297 = e.mem.u32(0x0120_0e84);
        held = e.mem.u32(held_cell);
        e.call(PASS_SET_00B79950, &args![held, word_297]);
        let word_298 = e.mem.u32(0x0120_0ce4);
        e.call(PASS_SET_00B80600, &args![held, word_298]);
        let wide_299 = x87_load32(float_4);
        let float_300 = x87_store32(wide_299);
        set_render_state(e, held, 0xc3u32, float_300, 1u32);
        set_render_state(e, held, 0x16u32, 1u32, 1u32);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        e.mem.set_u32(0x0120_0bb0, 0xb74u32);
        e.mem.set_u32(0x0120_09f0, 0u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1058u32]);
        let word_301 = fn_00c06dc0(e, 1u32, 1u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32);
        held = e.mem.u32(held_cell);
        let byte_302 = word_301 & 0xff;
        fn_00c06e10(e, Ptr::new(held), byte_302);
        let word_303 = e.mem.u32(0x0120_0e94);
        e.call(PASS_SET_00B79950, &args![held, word_303]);
        let word_304 = e.mem.u32(0x0120_0cf0);
        e.call(PASS_SET_00B80600, &args![held, word_304]);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 5u32, 6u32, 0u32]);
        set_render_state(e, held, 0xabu32, 1u32, 1u32);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        let wide_305 = x87_load32(float_4);
        let float_306 = x87_store32(wide_305);
        set_render_state(e, held, 0xc3u32, float_306, 1u32);
        e.mem.set_u32(0x0120_0bc0, 0u32);
        e.mem.set_u32(0x0120_0a00, 0u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_105cu32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), byte_302);
        let word_307 = e.mem.u32(0x0120_0e98);
        e.call(PASS_SET_00B79950, &args![held, word_307]);
        let word_308 = e.mem.u32(0x0120_0cf4);
        e.call(PASS_SET_00B80600, &args![held, word_308]);
        set_render_state(e, held, 0xabu32, 1u32, 1u32);
        set_render_state(e, held, 0x1bu32, 1u32, 1u32);
        e.call(PASS_STEP_00B80040, &args![held, 5u32, 6u32, 0u32]);
        set_render_state(e, held, 0xeu32, 0u32, 1u32);
        let wide_309 = x87_load32(float_4);
        let float_310 = x87_store32(wide_309);
        set_render_state(e, held, 0xc3u32, float_310, 1u32);
        e.mem.set_u32(0x0120_0bc4, 0u32);
        e.mem.set_u32(0x0120_0a04, 0u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1000u32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), byte_302);
        let word_311 = e.mem.u32(0x0120_0e4c);
        e.call(PASS_SET_00B79950, &args![held, word_311]);
        let word_312 = e.mem.u32(0x0120_0cc0);
        e.call(PASS_SET_00B80600, &args![held, word_312]);
        e.mem.set_u32(0x0120_0b68, 0xau32);
        e.mem.set_u32(0x0120_09a8, 0x400u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1004u32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), byte_302);
        let word_313 = e.mem.u32(0x0120_0e50);
        e.call(PASS_SET_00B79950, &args![held, word_313]);
        let word_314 = e.mem.u32(0x0120_0cc0);
        e.call(PASS_SET_00B80600, &args![held, word_314]);
        e.mem.set_u32(0x0120_0b6c, 0xau32);
        e.mem.set_u32(0x0120_09ac, 0x400u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1008u32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), byte_302);
        let word_315 = e.mem.u32(0x0120_0e54);
        e.call(PASS_SET_00B79950, &args![held, word_315]);
        let word_316 = e.mem.u32(0x0120_0cc0);
        e.call(PASS_SET_00B80600, &args![held, word_316]);
        e.mem.set_u32(0x0120_0b70, 0x34u32);
        e.mem.set_u32(0x0120_09b0, 0x400u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_100cu32]);
        held = e.mem.u32(held_cell);
        fn_00c06e10(e, Ptr::new(held), byte_302);
        let word_317 = e.mem.u32(0x0120_0e58);
        e.call(PASS_SET_00B79950, &args![held, word_317]);
        let word_318 = e.mem.u32(0x0120_0cc0);
        e.call(PASS_SET_00B80600, &args![held, word_318]);
        e.mem.set_u32(0x0120_0b74, 0x34u32);
        e.mem.set_u32(0x0120_09b4, 0x400u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_104cu32]);
        let word_319 = e.mem.u32(0x0120_0e88);
        held = e.mem.u32(held_cell);
        e.call(PASS_SET_00B79950, &args![held, word_319]);
        let word_320 = e.mem.u32(0x0120_0ce8);
        e.call(PASS_SET_00B80600, &args![held, word_320]);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        e.mem.set_u32(0x0120_0bb4, 0xau32);
        e.mem.set_u32(0x0120_09f4, 0u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1050u32]);
        let word_321 = e.mem.u32(0x0120_0e8c);
        held = e.mem.u32(held_cell);
        e.call(PASS_SET_00B79950, &args![held, word_321]);
        let word_322 = e.mem.u32(0x0120_0ce8);
        e.call(PASS_SET_00B80600, &args![held, word_322]);
        e.mem.set_u32(0x0120_0bb8, 0x34u32);
        e.mem.set_u32(0x0120_09f8, 0u32);
        e.call(POINTER_SWITCH_00BA53C0, &args![held_cell, 0x0120_1054u32]);
        let word_323 = e.mem.u32(0x0120_0e90);
        held = e.mem.u32(held_cell);
        e.call(PASS_SET_00B79950, &args![held, word_323]);
        let word_324 = e.mem.u32(0x0120_0cec);
        e.call(PASS_SET_00B80600, &args![held, word_324]);
        set_render_state(e, held, 0x17u32, 8u32, 1u32);
        set_render_state(e, held, 0xa8u32, 0xfu32, 1u32);
        e.mem.set_u32(0x0120_0bbc, 2u32);
        e.mem.set_u32(0x0120_09fc, 0u32);
        fn_00c06f80(e, Ptr::new(base));
        release_pass(e, held);
    });
}

/// `00b4f510` (`thiscall`, no stack words; unnamed in the engine map), the step that ends
/// the base texture return of the effect `00c049c0` constructs.
const EFFECT_RETURN_00B4F510: u32 = 0x00b4_f510;
/// `ImageSpaceTexture::BorrowRenderedTexture` (`00ba3840`, engine map; `thiscall`, one word).
const BORROW_RENDERED_TEXTURE: u32 = 0x00ba_3840;
/// `00b8a860` (`thiscall` on a shader parameter, `RET 8`: constant index, address of four
/// floats; unit `imagespaceshaderparam.cpp`, unnamed in the engine map).
const SHADER_PARAM_SET_PIXEL_CONSTANT_ARRAY: u32 = 0x00b8_a860;
/// `BSShaderManager::GetAccumulator` (`00b4f5c0`, engine map; no words).
const GET_ACCUMULATOR: u32 = 0x00b4_f5c0;
/// The renderer singleton pointer (`011f9508`).
const RENDERER_011F9508: u32 = 0x011f_9508;
/// `4294967296.0` (`float`, `01099378`): added to a negative integer loaded with `FILD`, the
/// compiler's unsigned conversion.
const TWO_TO_32_01099378: u32 = 0x0109_9378;

/// The first parameter of the effect's list (`pEffectParamList` data at +0x1c, entry 0).
fn first_param(e: &Engine, effect: u32) -> u32 {
    let data = e.mem.u32(effect + 0x1c);
    e.mem.u32(data)
}

// Translated from 00c04960 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): slot
/// +0x14 of the effect vtable `010bcaac` (`ReturnTextures` in the Xbox PDB's slot order;
/// `thiscall`, no stack words): `00b4f510`, then texture 2 of the texture array (+0x28) set
/// to null (`00ba3aa0` with the address of a zeroed local). Ends in that call; its `EAX` is
/// answered.
pub fn fn_00c04960(e: &mut Engine, this: Ptr) -> u32 {
    e.call(EFFECT_RETURN_00B4F510, &args![]);
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), 0);
        e.call(
            ARRAY_SET_AT_00BA3AA0,
            &args![this.addr() + EFFECT_TEXTURES, 2u32, local],
        )
        .u32()
    })
}

// Translated from 00c04b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): slot
/// +0x1c of the effect vtable `010bcaac` (`UpdateParams` in the Xbox PDB's slot order;
/// `thiscall`, one word not read). On parameter 0: pixel constant 0 is `(a, b, +0x60, 1.0)`
/// where `a` is the float +0x58 when the byte at `01202498` is set or 0.0 is below the float
/// at `011f9418`, else 0.0, and `b` is the float +0x5c divided by the unsigned answer of the
/// renderer's (`011f9508`) slot +0xc8 object's slot +0x90 (with 0); pixel constant 1 is
/// the dot products of the point at `012004f0` with the negated row `(+0x6c, +0x78, +0x84)`
/// and with `(+0x70, +0x7c, +0x88)` of the object at `011f917c`, then 0.0, 0.0 (every value
/// through the FPU in the exe's order). Answers true.
pub fn fn_00c04b90(e: &mut Engine, this: Ptr, _unused_1: u32) -> bool {
    let base = this.addr();
    let param = first_param(e, base);
    let a = if e.mem.u8(0x0120_2498) != 0 {
        x87_load32(e.mem.u32(base + 0x58))
    } else {
        // FLDZ; FCOM against the stored global: C0 (0.0 below it), ordered, takes +0x58.
        let limit = x87_quiet_bits(e.mem.u32(0x011f_9418));
        if 0.0 < x87_m32(limit) {
            x87_load32(e.mem.u32(base + 0x58))
        } else {
            0.0
        }
    };
    let first = x87_store32(a);
    let renderer = e.mem.u32(RENDERER_011F9508);
    let object = e.vcall(renderer, 0xc8, &[]).u32();
    let numerator = x87_load32(e.mem.u32(base + 0x5c));
    let count = e.vcall(object, 0x90, &args![0u32]).u32();
    let mut denominator = f64::from(count as i32);
    if (count as i32) < 0 {
        denominator = x87_add(denominator, x87_m32(e.mem.u32(TWO_TO_32_01099378)));
    }
    let ratio = x87_store32(x87_div(numerator, denominator));
    let one = 1.0f32.to_bits();
    let third = x87_quiet_bits(e.mem.u32(base + 0x60));
    e.call(
        SHADER_PARAM_SET_PIXEL_CONSTANT,
        &args![
            param,
            0u32,
            x87_quiet_bits(first),
            x87_quiet_bits(ratio),
            third,
            one
        ],
    );
    let object = e.mem.u32(0x011f_917c);
    let read = |e: &Engine, offset: u32| x87_quiet_bits(e.mem.u32(object.wrapping_add(offset)));
    let row_x = read(e, 0x6c);
    let row_y = read(e, 0x78);
    let row_z = read(e, 0x84);
    let point = [
        e.mem.u32(0x0120_04f0),
        e.mem.u32(0x0120_04f4),
        e.mem.u32(0x0120_04f8),
    ];
    let col_x = read(e, 0x70);
    let col_y = read(e, 0x7c);
    let col_z = read(e, 0x88);
    let negate = |bits: u32| x87_store32(-x87_load32(bits));
    let (neg_x, neg_y, neg_z) = (negate(row_x), negate(row_y), negate(row_z));
    let g0 = x87_load32(point[0]);
    let g1 = x87_load32(point[1]);
    let g2 = x87_load32(point[2]);
    // ST1 = g1 * -y; ST2 = -x * g0 (FMULP ST2); the sum, then + g2 * -z (FADDP ST2).
    let y_term = x87_mul(g1, x87_m32(neg_y));
    let x_term = x87_mul(x87_m32(neg_x), g0);
    let sum = x87_add(x_term, y_term);
    let z_term = x87_mul(g2, x87_m32(neg_z));
    let first_dot = x87_store32(x87_add(sum, z_term));
    // FMULP ST3: g1 * col_y; FMULP ST2: g0 * col_x; their sum; FMULP ST2: g2 * col_z, then
    // FADDP ST1 adds the sum (ST0) to it (ST1).
    let y_term = x87_mul(g1, x87_m32(col_y));
    let x_term = x87_mul(g0, x87_m32(col_x));
    let sum = x87_add(x_term, y_term);
    let z_term = x87_mul(g2, x87_m32(col_z));
    let second_dot = x87_store32(x87_add(z_term, sum));
    let zero = 0.0f32.to_bits();
    e.call(
        SHADER_PARAM_SET_PIXEL_CONSTANT,
        &args![
            param,
            1u32,
            x87_quiet_bits(second_dot),
            x87_quiet_bits(first_dot),
            zero,
            zero
        ],
    );
    true
}

// Translated from 00c04d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): slot
/// +0x2c of the effect vtable `010bcae8` (`thiscall`, one word): stores its low byte at
/// +0x58 and, when that changed the byte, sets `bParamsChanged` (+5, Xbox PDB).
pub fn fn_00c04d80(e: &mut Engine, this: Ptr, value: u8) {
    let base = this.addr();
    let changed = value != e.mem.u8(base + 0x58);
    e.mem.set_u8(base + 0x58, value);
    if changed {
        e.mem.set_u8(base + 5, 1);
    }
}

// Translated from 00c04e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0x1c of the effect vtable `010bcae8` (`UpdateParams` in
/// the Xbox PDB's slot order; `thiscall`, one word not read). Fills four float groups in
/// its frame and sets parameter 0's pixel constant 0 to `(d +0, d +4, d +8, first)` (the data
/// `d` at +0x60) and constants 1 to 3 from the groups (`00b8a860` with their addresses). With
/// the byte +0x58 set, `first` is the larger of the +0xc of the data at +0x5c and `d`'s
/// (the second when not below, NaN included), group 1 is that data's first three floats and
/// 1.0, and the last two words of group 3 are 1.0; otherwise `first` and group 1 are `d`'s
/// +0xc to +0x1c. Group 2 is `d`'s +0x20 to +0x2c, group 3 the words +0x68 to +0x74 (every
/// float an `FLD`/`FSTP` copy, a signalling NaN quieted). Answers false.
pub fn fn_00c04e30(e: &mut Engine, this: Ptr, _unused_1: u32) -> bool {
    let base = this.addr();
    let param = first_param(e, base);
    e.with_stack(0x38, |e, frame| {
        let frame = frame.addr();
        let quiet = |e: &Engine, address: u32| x87_quiet_bits(e.mem.u32(address));
        e.mem.set_u32(frame + 0x1c, e.mem.u32(base + 0x6c));
        e.mem.set_u32(frame + 0x18, e.mem.u32(base + 0x68));
        e.mem.set_u32(frame + 0x24, e.mem.u32(base + 0x74));
        e.mem.set_u32(frame + 0x20, e.mem.u32(base + 0x70));
        let data = e.mem.u32(base + 0x60);
        if e.mem.u8(base + 0x58) != 0 {
            let other = e.mem.u32(base + 0x5c);
            let a = quiet(e, other + 0xc);
            let b = quiet(e, data.wrapping_add(0xc));
            // FLD a; FLD b; FCOM: C0 (b below a) keeps a, else (NaN included) b.
            let larger = if x87_load32(b) < x87_load32(a) { a } else { b };
            e.mem.set_u32(frame, x87_quiet_bits(larger));
            for (slot, offset) in [(8u32, 0u32), (0xc, 4), (0x10, 8)] {
                let value = quiet(e, other + offset);
                e.mem.set_u32(frame + slot, value);
            }
            let one = 1.0f32.to_bits();
            for slot in [0x14u32, 0x20, 0x24] {
                e.mem.set_u32(frame + slot, one);
            }
        } else {
            for (slot, offset) in [
                (0u32, 0xcu32),
                (8, 0x10),
                (0xc, 0x14),
                (0x10, 0x18),
                (0x14, 0x1c),
            ] {
                let value = quiet(e, data.wrapping_add(offset));
                e.mem.set_u32(frame + slot, value);
            }
        }
        let data = e.mem.u32(base + 0x60);
        for (slot, offset) in [(0x28u32, 0x20u32), (0x2c, 0x24), (0x30, 0x28), (0x34, 0x2c)] {
            let value = quiet(e, data.wrapping_add(offset));
            e.mem.set_u32(frame + slot, value);
        }
        let fourth = quiet(e, frame);
        let third = quiet(e, data.wrapping_add(8));
        let second = quiet(e, data.wrapping_add(4));
        let first = quiet(e, data);
        e.call(
            SHADER_PARAM_SET_PIXEL_CONSTANT,
            &args![param, 0u32, first, second, third, fourth],
        );
        for (index, slot) in [(1u32, 8u32), (2, 0x28), (3, 0x18)] {
            e.call(
                SHADER_PARAM_SET_PIXEL_CONSTANT_ARRAY,
                &args![param, index, frame + slot],
            );
        }
    });
    false
}

// Translated from 00c05370 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): slot
/// +0x10 of the effect vtable `010bcb18` (`BorrowTextures` in the Xbox PDB's slot order;
/// `thiscall`, one word not read): borrows the rendered textures of the two
/// `ImageSpaceTexture`s at +0x5c and +0x6c (`BorrowRenderedTexture(0x1a)`) and stores their
/// addresses as textures 2 and 3 (`00ba3aa0`, through one local). Ends in the second store;
/// its `EAX` is answered.
pub fn fn_00c05370(e: &mut Engine, this: Ptr, _unused_1: u32) -> u32 {
    let base = this.addr();
    e.with_stack(4, |e, local| {
        let local = local.addr();
        e.mem.set_u32(local, base + 0x5c);
        e.call(BORROW_RENDERED_TEXTURE, &args![base + 0x5c, 0x1au32]);
        e.call(
            ARRAY_SET_AT_00BA3AA0,
            &args![base + EFFECT_TEXTURES, 2u32, local],
        );
        e.mem.set_u32(local, base + 0x6c);
        e.call(BORROW_RENDERED_TEXTURE, &args![base + 0x6c, 0x1au32]);
        e.call(
            ARRAY_SET_AT_00BA3AA0,
            &args![base + EFFECT_TEXTURES, 3u32, local],
        )
        .u32()
    })
}

// Translated from 00c053c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): slot
/// +0x1c of the effect vtable `010bcb18` (`UpdateParams`; `thiscall`, one word not read):
/// parameter 0's pixel constant 0 is `(0.0, y, z, 0.0)` with `y`, `z` the floats +4 and +8
/// of the object at +0x58, or the floats at `011ae6c8`/`011ae6cc` without one; then the
/// float at `0101e570` is copied to `012003c4` (through the FPU, quieted). Answers true.
pub fn fn_00c053c0(e: &mut Engine, this: Ptr, _unused_1: u32) -> bool {
    let base = this.addr();
    let param = first_param(e, base);
    let zero = 0.0f32.to_bits();
    let object = e.mem.u32(base + 0x58);
    let (y, z) = if object != 0 {
        let z = x87_quiet_bits(e.mem.u32(object + 8));
        (x87_quiet_bits(e.mem.u32(object + 4)), z)
    } else {
        let z = x87_quiet_bits(e.mem.u32(0x011a_e6cc));
        (x87_quiet_bits(e.mem.u32(0x011a_e6c8)), z)
    };
    e.call(
        SHADER_PARAM_SET_PIXEL_CONSTANT,
        &args![param, 0u32, zero, y, z, zero],
    );
    let value = x87_quiet_bits(e.mem.u32(0x0101_e570));
    e.mem.set_u32(0x0120_03c4, value);
    true
}

// Translated from 00c05730 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): slot
/// +0x1c of the effect vtable `010bcb3c` (`UpdateParams`; `thiscall`, one word not read):
/// parameter 0's pixel constant 0 is the floats `(+0x58, +0x5c, +0x60, 1.0)` (copies through
/// the FPU, quieted). Answers false.
pub fn fn_00c05730(e: &mut Engine, this: Ptr, _unused_1: u32) -> bool {
    let base = this.addr();
    let param = first_param(e, base);
    let values = [0x58u32, 0x5c, 0x60].map(|offset| x87_quiet_bits(e.mem.u32(base + offset)));
    e.call(
        SHADER_PARAM_SET_PIXEL_CONSTANT,
        &args![
            param,
            0u32,
            values[0],
            values[1],
            values[2],
            1.0f32.to_bits()
        ],
    );
    false
}

// Translated from 00c05870 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): slots
/// +0x10c and +0x110 of the vtable `010bcb60` (no stack words): answers 1.0 (`FLD1`).
pub fn fn_00c05870(_e: &mut Engine, _this: Ptr) -> f32 {
    1.0
}

/// The base constructor and destructor of the property `00c05880` constructs (`00bb4520`
/// and `00bb45b0`, `thiscall`, unnamed in the engine map) and the sized free `00aa1460`.
const PROPERTY_BASE_CONSTRUCT_00BB4520: u32 = 0x00bb_4520;
const PROPERTY_BASE_DESTRUCT_00BB45B0: u32 = 0x00bb_45b0;
const NI_FREE_SIZED: u32 = 0x00aa_1460;
/// That property's vtable and size.
const PROPERTY_VTABLE_010BCB60: u32 = 0x010b_cb60;
const PROPERTY_SIZE_00C05880: u32 = 0x108;

// Translated from 00c05880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the constructor of a 0x108-byte shader property built on the
/// `Lighting30ShaderProperty` class (the `NiRTTI` record its inherited slot +8, `00bb45c0`,
/// answers; `thiscall`, no words; called from `GetShaderProperty` at `00baf367`): the base
/// constructor `00bb4520`, the vtable `010bcb60`, 0 at +0x100, 0x1d at +0x58 and 0xc at +0x1c; when bit
/// 0x800000 of +0x24 is clear, +0x38 is cleared; then that bit is set. Answers `this`.
pub fn fn_00c05880(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(PROPERTY_BASE_CONSTRUCT_00BB4520, &args![this]);
    e.mem.set_u32(base, PROPERTY_VTABLE_010BCB60);
    e.mem.set_u32(base + 0x100, 0);
    e.mem.set_u32(base + 0x58, 0x1d);
    e.mem.set_u32(base + 0x1c, 0xc);
    if e.mem.u32(base + 0x24) & 0x0080_0000 == 0 {
        e.mem.set_u32(base + 0x38, 0);
    }
    let flags = e.mem.u32(base + 0x24) | 0x0080_0000;
    e.mem.set_u32(base + 0x24, flags);
    this
}

// Translated from 00c058c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot 0 of the vtable `010bcb60`, that property's deleting
/// destructor (`thiscall`, one word: flags): the vtable stored, the base destructor
/// `00bb45b0`, and with bit 0 the 0x108 bytes freed (`00aa1460`). Answers `this`.
pub fn fn_00c058c0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.mem.set_u32(this.addr(), PROPERTY_VTABLE_010BCB60);
    e.call(PROPERTY_BASE_DESTRUCT_00BB45B0, &args![this]);
    if flags & 1 != 0 {
        e.call(NI_FREE_SIZED, &args![this, PROPERTY_SIZE_00C05880]);
    }
    this
}

/// The counted-pointer assignment shared by `00c06ee0` and `00c06f30`: when `new` differs
/// from the word at `slot`, the old object is released (`DeleteThis`, slot +4, when its
/// count drops to zero), `new` stored and its count raised when not null.
fn assign_counted_at(e: &mut Engine, slot: u32, new: u32) {
    let old = e.mem.u32(slot);
    if old == new {
        return;
    }
    if old != 0 && e.call(IMPORT_INTERLOCKED_DECREMENT, &args![old + 4]).u32() == 0 {
        e.vcall(old, 4, &[]);
    }
    e.mem.set_u32(slot, new);
    if new != 0 {
        e.call(IMPORT_INTERLOCKED_INCREMENT, &args![new + 4]);
    }
}

// Translated from 00c06ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; called twice from `00e8dca0`): the
/// counted pointer at +0x34 set to the word (the old one released, the new one held).
pub fn fn_00c06ee0(e: &mut Engine, this: Ptr, object: u32) {
    assign_counted_at(e, this.addr() + 0x34, object);
}

// Translated from 00c06f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, one word; called twice from `00e8dca0`): the
/// counted pointer at +0x48 set to the word, as `fn_00c06ee0` does for +0x34.
pub fn fn_00c06f30(e: &mut Engine, this: Ptr, object: u32) {
    assign_counted_at(e, this.addr() + 0x48, object);
}

/// The stage at `index` of the stage array at +0x24 of `owner`.
fn owner_stage(e: &Engine, owner: u32, index: u32) -> u32 {
    let stages = e.mem.u32(owner.wrapping_add(0x24));
    e.mem.u32(stages.wrapping_add(4 * index))
}

// Translated from 00c0bae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words: the stage owner, the geometry and a
/// stage index; called from `00c0bd40`): the stage gets the geometry's slot +0xf4 answer
/// for `(3, 0)` at +8, then `00bb9640` runs on it; its `EAX` is answered.
pub fn fn_00c0bae0(e: &mut Engine, this: Ptr, owner: u32, geometry: u32, index: u32) -> u32 {
    let stage = owner_stage(e, owner, index);
    let texture = e.vcall(geometry, 0xf4, &args![3u32, 0u32]).u32();
    e.mem.set_u32(stage + 8, texture);
    e.call(STAGE_SETUP_00BB9640, &args![this, stage, geometry])
        .u32()
}

// Translated from 00c0bb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` with `this` not read, three words: the stage
/// owner, the geometry and a flag byte): stages 4 and 5 get the second words of the
/// geometry's +0xac and +0xb0 objects at +8; with the flag, stage 6 gets the geometry's slot
/// +0xf4 answer for `(2, 0)`.
pub fn fn_00c0bb50(e: &mut Engine, _this: Ptr, owner: u32, geometry: u32, flag: u8) {
    let texture = e.mem.u32(e.mem.u32(geometry + 0xac).wrapping_add(4));
    let stage = owner_stage(e, owner, 4);
    e.mem.set_u32(stage + 8, texture);
    let source = e.mem.u32(geometry + 0xb0);
    let stage = owner_stage(e, owner, 5);
    let texture = e.mem.u32(source.wrapping_add(4));
    e.mem.set_u32(stage + 8, texture);
    if flag != 0 {
        let stage = owner_stage(e, owner, 6);
        let texture = e.vcall(geometry, 0xf4, &args![2u32, 0u32]).u32();
        e.mem.set_u32(stage + 8, texture);
    }
}

// Translated from 00c0bba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` with `this` not read, two words: the stage owner
/// and the geometry): stage 0 gets the first word of the geometry's +0xb0 object at +8, or
/// the word at `011f9510` when that is null.
pub fn fn_00c0bba0(e: &mut Engine, _this: Ptr, owner: u32, geometry: u32) {
    let stage = owner_stage(e, owner, 0);
    let mut texture = e.mem.u32(e.mem.u32(geometry + 0xb0));
    if texture == 0 {
        texture = e.mem.u32(0x011f_9510);
    }
    e.mem.set_u32(stage + 8, texture);
}

// Translated from 00c0bbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` with `this` not read, one word: the stage owner):
/// stages 2 and 3 get the words at `011f95e4` and `011f95e8` at +8.
pub fn fn_00c0bbd0(e: &mut Engine, _this: Ptr, owner: u32) {
    let stage = owner_stage(e, owner, 2);
    e.mem.set_u32(stage + 8, e.mem.u32(0x011f_95e4));
    let stage = owner_stage(e, owner, 3);
    e.mem.set_u32(stage + 8, e.mem.u32(0x011f_95e8));
}

// Translated from 00c0bc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` with `this` not read, three words: the stage
/// owner, the geometry and the pass number): stage 1 gets, at +8, the geometry's slot
/// +0xf4 answer for `(4, 0)`, or when that is null the word at `011f9544` for a geometry
/// with bit 0x20000 of +0x20 (pass numbers 0x1be and 0x1c0 aside) if it is not null, else
/// the word at `011f95d4`. Before that the accumulator's word +0x194 is read
/// (`BSShaderManager::GetAccumulator`) and, with the byte at `011f9427` set, `004a2020(0x15)`
/// asked, though every path but the bit's ends with `011f95d4`. Answers the stored word.
pub fn fn_00c0bc00(e: &mut Engine, _this: Ptr, owner: u32, geometry: u32, pass: u32) -> u32 {
    let accumulator = e.call(GET_ACCUMULATOR, &[]).u32();
    let held = e.mem.u32(accumulator.wrapping_add(0x194));
    let mut texture = e.vcall(geometry, 0xf4, &args![4u32, 0u32]).u32();
    if texture == 0 {
        let fallback = e.mem.u32(0x011f_95d4);
        let listed = pass == 0x1be || pass == 0x1c0;
        texture = if !listed && e.mem.u32(geometry + 0x20) & 0x0002_0000 != 0 {
            let value = e.mem.u32(0x011f_9544);
            if value != 0 {
                value
            } else {
                fallback
            }
        } else {
            if !listed && e.mem.u8(0x011f_9427) != 0 {
                // Its answer and the accumulator's word (`TEST EDI,EDI`) only choose
                // between paths that all store the fallback.
                e.call(GEOMETRY_FLAG_004A2020, &args![geometry, 0x15u32]);
            }
            let _ = held;
            fallback
        };
    }
    let stage = owner_stage(e, owner, 1);
    e.mem.set_u32(stage + 8, texture);
    texture
}

// Translated from 00c0bc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the stage owner and the geometry):
/// stage 3 gets the geometry's slot +0xf4 answer for `(5, 0)` with 1.0 stored at
/// `01201bac`, or when that is null the word at `011f9514` with 0.0 there; `00bb9640` runs
/// on the stage, then the geometry's float +0x30 is copied to `01201ba8` (quieted). Answers
/// `00bb9640`'s `EAX`.
pub fn fn_00c0bc90(e: &mut Engine, this: Ptr, owner: u32, geometry: u32) -> u32 {
    let mut texture = e.vcall(geometry, 0xf4, &args![5u32, 0u32]).u32();
    e.mem.set_u32(0x0120_1bac, 1.0f32.to_bits());
    let stage = owner_stage(e, owner, 3);
    if texture == 0 {
        texture = e.mem.u32(0x011f_9514);
        e.mem.set_u32(0x0120_1bac, 0.0f32.to_bits());
    }
    e.mem.set_u32(stage + 8, texture);
    let answer = e
        .call(STAGE_SETUP_00BB9640, &args![this, stage, geometry])
        .u32();
    let value = x87_quiet_bits(e.mem.u32(geometry + 0x30));
    e.mem.set_u32(0x0120_1ba8, value);
    answer
}

// Translated from 00c0bcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, three words: the stage owner, the geometry and a
/// stage index): the stage gets, at +8, the first word of the geometry's +0xb0 object, or
/// when that is null the word at `011f9518` (bit 0x80 of the geometry's +0x20 set) or at
/// `011f9510`; then `00bb9640` runs on it and its `EAX` is answered.
pub fn fn_00c0bcf0(e: &mut Engine, this: Ptr, owner: u32, geometry: u32, index: u32) -> u32 {
    let stage = owner_stage(e, owner, index);
    let mut texture = e.mem.u32(e.mem.u32(geometry + 0xb0));
    if texture == 0 {
        texture = if e.mem.u32(geometry + 0x20) & 0x80 != 0 {
            e.mem.u32(0x011f_9518)
        } else {
            e.mem.u32(0x011f_9510)
        };
    }
    e.mem.set_u32(stage + 8, texture);
    e.call(STAGE_SETUP_00BB9640, &args![this, stage, geometry])
        .u32()
}

/// The `lighting30shader.cpp` callees of `00c0bd40` (engine map units; `thiscall`, two
/// words each): `00bbc610` (stage owner, flag), `00bbdcd0` and `00bbdda0` (geometry, stage
/// owner); `00b78a70` (`shadowlightshader.cpp`, no words) and `00c02d40` (`thiscall`, no
/// words, a float answer in ST0).
const OWNER_STEP_00BBC610: u32 = 0x00bb_c610;
const GEOMETRY_STEP_00BBDCD0: u32 = 0x00bb_dcd0;
const GEOMETRY_STEP_00BBDDA0: u32 = 0x00bb_dda0;
const FINISH_00B78A70: u32 = 0x00b7_8a70;
const LIGHT_VALUE_00C02D40: u32 = 0x00c0_2d40;

/// What `00c0bd40` does for a pass before answering its pass global (the targets of its
/// jump table at `00c0ca6c`).
#[derive(Clone, Copy, Debug, PartialEq)]
enum Case {
    /// Nothing.
    Plain,
    /// The geometry's words +0x84..+0x90 copied to `01201b50`; stage 4 gets the word at
    /// `011f9530`, stage 5 the first word of the geometry's +0xb4 object.
    LightStages,
    /// `LightStages`, then `00c0bbd0`.
    LightStagesShared,
    /// `00c0bae0` with stage 4.
    Stage4,
    /// The geometry's floats +0xe8 and +0xec copied to `01201b90`/`01201b94`, then
    /// `00c0bae0` with stage 4.
    Floats4,
    /// `00c0bbd0`, then `00c0bae0` with stage 4.
    SharedStage4,
    /// The two floats, `00c0bbd0`, then `00c0bae0` with stage 4.
    SharedFloats4,
    /// Stages 4 and 5 get the second words of the geometry's +0xac and +0xb0 objects.
    Pair,
    /// `Pair`, then stage 6 the geometry's slot +0xf4 answer for `(2, 0)`.
    PairCall,
    /// `00c0bbd0`, then `Pair`.
    SharedPair,
    /// `00c0bbd0`, then `PairCall`.
    SharedPairCall,
    /// Stage 6 gets the first word of the geometry's +0xb4 object.
    Stage6,
    /// `Stage6`, then `00c0bbd0`.
    Stage6Shared,
    /// `00c0bbd0`.
    Shared,
    /// `00c0bc00` (with the pass number), `00c0bc90`, `00c0bcf0` with stage 0.
    Shadow,
    /// `Shadow`, then the offset of the light position (see `fn_00c0bd40`).
    ShadowOffset,
    /// Stage 0 gets the first word of the geometry's +0xac object; `00bbc610(1)`.
    FirstFromAc,
    /// Stage 0 gets the first word of the geometry's +0xb0 object (or the word at
    /// `011f9510`); `00bbc610(1)`.
    FirstFromB0,
    /// `00bbc610(1)`.
    Flagged,
    /// `00bbc610(0)`.
    Unflagged,
    /// `00bbc610(0)`, then `00b78a70`.
    UnflaggedFinish,
    /// `00bbc610(0)`, the light values (see `fn_00c0bd40`), then with `true` `00b78a70`.
    UnflaggedLight(bool),
    /// `00bbdcd0` with the geometry and the pass global.
    Geometry00BBDCD0,
}

/// `00c0bd40`'s switch on pass numbers 0x17f..=0x2ed: the byte map at `00c0cc08` picks
/// one of the cases of the jump table at `00c0ca6c` (0x19c shares 0x18d's case); the
/// pass global each case answers and what it does. Numbers the map sends to its last
/// case (and numbers outside the map) answer 0.
fn case_00c0bd40(pass: u32) -> Option<(u32, Case)> {
    match pass {
        0x17f => Some((0x0120_0edc, Case::Plain)),
        0x180 => Some((0x0120_0ee0, Case::Plain)),
        0x181 => Some((0x0120_0ee4, Case::LightStages)),
        0x182 => Some((0x0120_0ee8, Case::Stage4)),
        0x183 => Some((0x0120_0eec, Case::Floats4)),
        0x184 => Some((0x0120_0ef0, Case::Pair)),
        0x185 => Some((0x0120_0ef4, Case::PairCall)),
        0x186 => Some((0x0120_0ef8, Case::Stage6)),
        0x187 => Some((0x0120_0efc, Case::Stage6)),
        0x188 => Some((0x0120_0f00, Case::Plain)),
        0x189 => Some((0x0120_0f04, Case::Plain)),
        0x18a => Some((0x0120_0f08, Case::Stage4)),
        0x18b => Some((0x0120_0f0c, Case::Floats4)),
        0x18c => Some((0x0120_0f10, Case::Stage6)),
        0x18d => Some((0x0120_0f14, Case::Stage6)),
        0x18e => Some((0x0120_0f18, Case::Plain)),
        0x18f => Some((0x0120_0f1c, Case::Plain)),
        0x190 => Some((0x0120_0f20, Case::LightStages)),
        0x191 => Some((0x0120_0f24, Case::Stage4)),
        0x192 => Some((0x0120_0f28, Case::Floats4)),
        0x193 => Some((0x0120_0f2c, Case::Pair)),
        0x194 => Some((0x0120_0f30, Case::PairCall)),
        0x195 => Some((0x0120_0f34, Case::Stage6)),
        0x196 => Some((0x0120_0f38, Case::Stage6)),
        0x197 => Some((0x0120_0f3c, Case::Plain)),
        0x198 => Some((0x0120_0f40, Case::Plain)),
        0x199 => Some((0x0120_0f44, Case::Stage4)),
        0x19a => Some((0x0120_0f48, Case::Floats4)),
        0x19b => Some((0x0120_0f4c, Case::Stage6)),
        0x19c => Some((0x0120_0f14, Case::Stage6)),
        0x19d => Some((0x0120_0f54, Case::Shared)),
        0x19e => Some((0x0120_0f58, Case::Shared)),
        0x19f => Some((0x0120_0f5c, Case::LightStagesShared)),
        0x1a0 => Some((0x0120_0f60, Case::SharedStage4)),
        0x1a1 => Some((0x0120_0f64, Case::SharedFloats4)),
        0x1a2 => Some((0x0120_0f68, Case::SharedPair)),
        0x1a3 => Some((0x0120_0f6c, Case::SharedPairCall)),
        0x1a4 => Some((0x0120_0f70, Case::Stage6Shared)),
        0x1a5 => Some((0x0120_0f74, Case::Stage6Shared)),
        0x1a6 => Some((0x0120_0f78, Case::Shared)),
        0x1a7 => Some((0x0120_0f7c, Case::Shared)),
        0x1a8 => Some((0x0120_0f80, Case::SharedStage4)),
        0x1a9 => Some((0x0120_0f84, Case::SharedFloats4)),
        0x1aa => Some((0x0120_0f88, Case::Stage6Shared)),
        0x1ab => Some((0x0120_0f8c, Case::Stage6Shared)),
        0x1ac => Some((0x0120_0f90, Case::Shared)),
        0x1ad => Some((0x0120_0f94, Case::Shared)),
        0x1ae => Some((0x0120_0f98, Case::LightStagesShared)),
        0x1af => Some((0x0120_0f9c, Case::SharedStage4)),
        0x1b0 => Some((0x0120_0fa0, Case::SharedFloats4)),
        0x1b1 => Some((0x0120_0fa4, Case::SharedPair)),
        0x1b2 => Some((0x0120_0fa8, Case::SharedPairCall)),
        0x1b3 => Some((0x0120_0fac, Case::Stage6Shared)),
        0x1b4 => Some((0x0120_0fb0, Case::Stage6Shared)),
        0x1b5 => Some((0x0120_0fb4, Case::Shared)),
        0x1b6 => Some((0x0120_0fb8, Case::Shared)),
        0x1b7 => Some((0x0120_0fbc, Case::SharedStage4)),
        0x1b8 => Some((0x0120_0fc0, Case::SharedFloats4)),
        0x1b9 => Some((0x0120_0fc4, Case::Stage6Shared)),
        0x1ba => Some((0x0120_0fc8, Case::Stage6Shared)),
        0x1bb => Some((0x0120_0fcc, Case::Flagged)),
        0x1bc => Some((0x0120_0fd0, Case::Flagged)),
        0x1bd => Some((0x0120_0fd4, Case::Shadow)),
        0x1be => Some((0x0120_0fd8, Case::Shadow)),
        0x1bf => Some((0x0120_0fdc, Case::Shadow)),
        0x1c0 => Some((0x0120_0fe0, Case::Shadow)),
        0x1c1 => Some((0x0120_0fe4, Case::Shadow)),
        0x1c2 => Some((0x0120_0fe8, Case::Shadow)),
        0x1c3 => Some((0x0120_0fec, Case::ShadowOffset)),
        0x1c4 => Some((0x0120_0ff0, Case::Plain)),
        0x1c5 => Some((0x0120_0ff4, Case::Plain)),
        0x1c6 => Some((0x0120_0ff8, Case::Plain)),
        0x1c7 => Some((0x0120_0ffc, Case::Plain)),
        0x1c8 => Some((0x0120_1000, Case::FirstFromAc)),
        0x1ca => Some((0x0120_1008, Case::FirstFromAc)),
        0x1cc => Some((0x0120_1010, Case::FirstFromB0)),
        0x1ce => Some((0x0120_1014, Case::FirstFromB0)),
        0x1d0 => Some((0x0120_1018, Case::FirstFromB0)),
        0x1d1 => Some((0x0120_101c, Case::Unflagged)),
        0x1d2 => Some((0x0120_1020, Case::Unflagged)),
        0x1d3 => Some((0x0120_1024, Case::Plain)),
        0x1d4 => Some((0x0120_1028, Case::Plain)),
        0x1d5 => Some((0x0120_102c, Case::Plain)),
        0x1d6 => Some((0x0120_1030, Case::Plain)),
        0x1d7 => Some((0x0120_1034, Case::Unflagged)),
        0x1d8 => Some((0x0120_1038, Case::Unflagged)),
        0x1d9 => Some((0x0120_103c, Case::UnflaggedFinish)),
        0x1da => Some((0x0120_1040, Case::UnflaggedFinish)),
        0x1db => Some((0x0120_1044, Case::UnflaggedLight(true))),
        0x1dc => Some((0x0120_1048, Case::UnflaggedLight(false))),
        0x1dd => Some((0x0120_104c, Case::Unflagged)),
        0x1de => Some((0x0120_1050, Case::Unflagged)),
        0x1df => Some((0x0120_1054, Case::Unflagged)),
        0x1e0 => Some((0x0120_1058, Case::Geometry00BBDCD0)),
        0x1e1 => Some((0x0120_105c, Case::Geometry00BBDCD0)),
        0x2da => Some((0x0120_1060, Case::Plain)),
        0x2db => Some((0x0120_1064, Case::Plain)),
        0x2e0 => Some((0x0120_1068, Case::Plain)),
        0x2e1 => Some((0x0120_106c, Case::Plain)),
        0x2e6 => Some((0x0120_1070, Case::Plain)),
        0x2e7 => Some((0x0120_1074, Case::Plain)),
        0x2ec => Some((0x0120_1078, Case::Plain)),
        0x2ed => Some((0x0120_107c, Case::Plain)),
        _ => None,
    }
}

/// The first word of the object at `offset` of the geometry.
fn geometry_first(e: &Engine, geometry: u32, offset: u32) -> u32 {
    e.mem.u32(e.mem.u32(geometry.wrapping_add(offset)))
}

/// The second word of the object at `offset` of the geometry.
fn geometry_second(e: &Engine, geometry: u32, offset: u32) -> u32 {
    e.mem
        .u32(e.mem.u32(geometry.wrapping_add(offset)).wrapping_add(4))
}

/// Sets +8 of the stage at `index` of `owner`'s stage array.
fn set_owner_stage(e: &mut Engine, owner: u32, index: u32, value: u32) {
    let stage = owner_stage(e, owner, index);
    e.mem.set_u32(stage.wrapping_add(8), value);
}

// Translated from 00c0bd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, two words: the pass number and the geometry;
/// called from `00bbb82a` in `lighting30shader.cpp`): answers the pass object kept in the
/// global for the pass number, after setting up its stages for the geometry. Numbers
/// 10..=13 and 0x178 only answer theirs (`01200ec4`..`01200ed0`, `01200ec0`); 14 and 15 run
/// `00bbdda0(geometry, pass)` first (`01200ed4`, `01200ed8`); 0x17f..=0x2ed go through the
/// byte map's cases (`case_00c0bd40`; the pass's stage owner is the global itself). The
/// light offset case stores, at `01201360`..`01201368`, the position at +0x20 of the
/// object `011f91e0` points to (or the point at `011f4288` without one) minus the point at
/// `011f474c`; the light cases store the float `00c02d40` answers on the object at +0x30 of
/// +8 of `011f91ac` at `01201384`, that object's float +0x58 at `01201388` and 0.0 at
/// `0120138c`. Any other number answers 0.
pub fn fn_00c0bd40(e: &mut Engine, this: Ptr, pass: u32, geometry: u32) -> u32 {
    let this = this.addr();
    match pass {
        10 => return e.mem.u32(0x0120_0ec4),
        11 => return e.mem.u32(0x0120_0ec8),
        12 => return e.mem.u32(0x0120_0ecc),
        13 => return e.mem.u32(0x0120_0ed0),
        14 | 15 => {
            let owner = e
                .mem
                .u32(if pass == 14 { 0x0120_0ed4 } else { 0x0120_0ed8 });
            e.call(GEOMETRY_STEP_00BBDDA0, &args![this, geometry, owner]);
            return owner;
        }
        0x178 => return e.mem.u32(0x0120_0ec0),
        _ => {}
    }
    let Some((global, case)) = case_00c0bd40(pass) else {
        return 0;
    };
    let owner = e.mem.u32(global);
    let stage4 = |e: &mut Engine| {
        fn_00c0bae0(e, Ptr::new(this), owner, geometry, 4);
    };
    let shared = |e: &mut Engine| {
        fn_00c0bbd0(e, Ptr::new(this), owner);
    };
    let floats = |e: &mut Engine| {
        let first = x87_quiet_bits(e.mem.u32(geometry.wrapping_add(0xe8)));
        e.mem.set_u32(0x0120_1b90, first);
        let second = x87_quiet_bits(e.mem.u32(geometry.wrapping_add(0xec)));
        e.mem.set_u32(0x0120_1b94, second);
    };
    let light_stages = |e: &mut Engine| {
        for word in 0..4 {
            let value = e.mem.u32(geometry.wrapping_add(0x84 + 4 * word));
            e.mem.set_u32(0x0120_1b50 + 4 * word, value);
        }
        let value = e.mem.u32(0x011f_9530);
        set_owner_stage(e, owner, 4, value);
        let value = geometry_first(e, geometry, 0xb4);
        set_owner_stage(e, owner, 5, value);
    };
    let pair = |e: &mut Engine| {
        let value = geometry_second(e, geometry, 0xac);
        set_owner_stage(e, owner, 4, value);
        let value = geometry_second(e, geometry, 0xb0);
        set_owner_stage(e, owner, 5, value);
    };
    let pair_call = |e: &mut Engine| {
        pair(e);
        let stage = owner_stage(e, owner, 6);
        let value = e.vcall(geometry, 0xf4, &args![2u32, 0u32]).u32();
        e.mem.set_u32(stage.wrapping_add(8), value);
    };
    let stage6 = |e: &mut Engine| {
        let value = geometry_first(e, geometry, 0xb4);
        set_owner_stage(e, owner, 6, value);
    };
    let shadow = |e: &mut Engine| {
        fn_00c0bc00(e, Ptr::new(this), owner, geometry, pass);
        fn_00c0bc90(e, Ptr::new(this), owner, geometry);
        fn_00c0bcf0(e, Ptr::new(this), owner, geometry, 0);
    };
    let flag = |e: &mut Engine, value: u32| {
        e.call(OWNER_STEP_00BBC610, &args![this, owner, value]);
    };
    match case {
        Case::Plain => {}
        Case::LightStages => light_stages(e),
        Case::LightStagesShared => {
            light_stages(e);
            shared(e);
        }
        Case::Stage4 => stage4(e),
        Case::Floats4 => {
            floats(e);
            stage4(e);
        }
        Case::SharedStage4 => {
            shared(e);
            stage4(e);
        }
        Case::SharedFloats4 => {
            floats(e);
            shared(e);
            stage4(e);
        }
        Case::Pair => pair(e),
        Case::PairCall => pair_call(e),
        Case::SharedPair => {
            shared(e);
            pair(e);
        }
        Case::SharedPairCall => {
            shared(e);
            pair_call(e);
        }
        Case::Stage6 => stage6(e),
        Case::Stage6Shared => {
            stage6(e);
            shared(e);
        }
        Case::Shared => shared(e),
        Case::Shadow => shadow(e),
        Case::ShadowOffset => {
            shadow(e);
            let holder = e.mem.u32(e.mem.u32(0x011f_91e0));
            let mut position = e.mem.u32(holder.wrapping_add(0x20));
            if position == 0 {
                position = 0x011f_4288;
            }
            for word in 0..3 {
                let value = x87_sub(
                    x87_load32(e.mem.u32(position + 4 * word)),
                    x87_m32(e.mem.u32(0x011f_474c + 4 * word)),
                );
                let value = x87_quiet_bits(x87_store32(value));
                e.mem.set_u32(0x0120_1360 + 4 * word, value);
            }
        }
        Case::FirstFromAc => {
            let value = geometry_first(e, geometry, 0xac);
            set_owner_stage(e, owner, 0, value);
            flag(e, 1);
        }
        Case::FirstFromB0 => {
            let mut value = geometry_first(e, geometry, 0xb0);
            if value == 0 {
                value = e.mem.u32(0x011f_9510);
            }
            set_owner_stage(e, owner, 0, value);
            flag(e, 1);
        }
        Case::Flagged => flag(e, 1),
        Case::Unflagged => flag(e, 0),
        Case::UnflaggedFinish => {
            flag(e, 0);
            e.call(FINISH_00B78A70, &[]);
        }
        Case::UnflaggedLight(finish) => {
            flag(e, 0);
            let light = |e: &Engine| {
                let holder = e.mem.u32(0x011f_91ac);
                e.mem
                    .u32(e.mem.u32(holder.wrapping_add(8)).wrapping_add(0x30))
            };
            let object = light(e);
            let value = e.call(LIGHT_VALUE_00C02D40, &args![object]).f64();
            e.mem.set_u32(0x0120_1384, x87_store32(value));
            let object = light(e);
            let value = x87_quiet_bits(e.mem.u32(object.wrapping_add(0x58)));
            e.mem.set_u32(0x0120_1388, value);
            e.mem.set_u32(0x0120_138c, 0.0f32.to_bits());
            if finish {
                e.call(FINISH_00B78A70, &[]);
            }
        }
        Case::Geometry00BBDCD0 => {
            e.call(GEOMETRY_STEP_00BBDCD0, &args![this, geometry, owner]);
        }
    }
    owner
}

/// The callees of `fn_00c058f0` (the `Lighting30ShaderProperty` render-pass code it shares
/// with `00bb4740`; `thiscall` unless said otherwise): `BSShaderProperty`'s pass-array makers
/// (`00ba95e0`, `00ba9660`, `00ba9740`, `00ba96d0`, `00ba9820`, one word each),
/// `RenderPassArray::Add` (`00ba9ee0`, eight words), the pass light-array free and allocate
/// (`00ba8c30`, `00ba8c00`), `SetFlag` (`00441130`, two words), `SetAlpha` (`00ba8ab0`, one
/// float), the light walk of `BSShaderLightingProperty` (`ResortLights` `00b70390`,
/// `GetFirstActiveLight` `00b70590`, `GetNextActiveLight` `00b70680`,
/// `GetFirstActiveNonShadowLight` `00b70600`, `GetNextActiveNonShadowLight` `00b70700`, each
/// with an iterator's address), `NiPoint3::Length` (`00457990`, ST0), `004b3ab0` (`cdecl`,
/// five floats, ST0) and the `cdecl` setting tests `005bf7b0`, `00b60330` and `00b60170`.
const L30_CHECK_CREATE_PASSES: u32 = 0x00ba_95e0;
const L30_CHECK_CREATE_DEPTH_PASSES: u32 = 0x00ba_9660;
const L30_CHECK_CREATE_CONST_ALPHA_PASSES: u32 = 0x00ba_9740;
const L30_CHECK_CREATE_LOCAL_MAP_PASSES: u32 = 0x00ba_96d0;
const L30_CHECK_CREATE_PASSES_54: u32 = 0x00ba_9820;
const L30_ADD_PASS: u32 = 0x00ba_9ee0;
const L30_PASS_FREE_LIGHTS: u32 = 0x00ba_8c30;
const L30_PASS_ALLOCATE_LIGHTS: u32 = 0x00ba_8c00;
const L30_SET_FLAG: u32 = 0x0044_1130;
const L30_SET_ALPHA: u32 = 0x00ba_8ab0;
const L30_RESORT_LIGHTS: u32 = 0x00b7_0390;
const L30_FIRST_ACTIVE_LIGHT: u32 = 0x00b7_0590;
const L30_NEXT_ACTIVE_LIGHT: u32 = 0x00b7_0680;
const L30_FIRST_NON_SHADOW_LIGHT: u32 = 0x00b7_0600;
const L30_NEXT_NON_SHADOW_LIGHT: u32 = 0x00b7_0700;
const L30_VECTOR_LENGTH: u32 = 0x0045_7990;
const L30_INTERPOLATE: u32 = 0x004b_3ab0;
const L30_DEPTH_PASSES_WANTED: u32 = 0x005b_f7b0;
const L30_SETTING_TEST_00B60330: u32 = 0x00b6_0330;
const L30_SETTING_VALUE_00B60170: u32 = 0x00b6_0170;
/// The renderer (`011f4748`), the default bound (`011f4288`), the fade distances and the
/// byte that keeps the passes on under flag bit 20 (`011f91a7`).
const L30_RENDERER: u32 = 0x011f_4748;
const L30_DEFAULT_BOUND: u32 = 0x011f_4288;
const L30_FADE_FAR_011F9458: u32 = 0x011f_9458;
const L30_FADE_FAR_011F9460: u32 = 0x011f_9460;
const L30_FADE_FAR_011F9468: u32 = 0x011f_9468;
const L30_FADE_NEAR_011F946C: u32 = 0x011f_946c;
const L30_FADE_FAR_011F9470: u32 = 0x011f_9470;
const L30_PASSES_FORCED_011F91A7: u32 = 0x011f_91a7;
/// `BSShaderProperty` fields (Xbox PDB names, PC offsets as `00bb4740` reads them): the two
/// flag words, the alpha and fade alpha, the camera distance, the last render pass state,
/// the pass arrays, the "resort lights" byte and the words +0xb4 and +0xdc.
const L30_FLAGS0: u32 = 0x20;
const L30_FLAGS1: u32 = 0x24;
const L30_ALPHA: u32 = 0x28;
const L30_FADE_ALPHA: u32 = 0x2c;
const L30_CAMERA_DISTANCE: u32 = 0x34;
const L30_LAST_STATE: u32 = 0x38;
const L30_PASSES: u32 = 0x3c;
const L30_DEPTH_PASSES: u32 = 0x40;
const L30_CONST_ALPHA_PASSES: u32 = 0x44;
const L30_LOCAL_MAP_PASSES: u32 = 0x48;
const L30_PASSES_54: u32 = 0x54;
const L30_LIGHTS_DIRTY: u32 = 0x74;
const L30_WORD_B4: u32 = 0xb4;
const L30_OBJECT_DC: u32 = 0xdc;

/// `004a2020` on the property: its flag bit.
fn l30_flag(e: &mut Engine, this: u32, flag: u32) -> bool {
    e.call(GEOMETRY_FLAG_004A2020, &args![this, flag]).bool()
}

/// The renderer's light word (`[[011f4748] + 8] + 0x1a0`).
fn l30_renderer_light(e: &Engine) -> u32 {
    let renderer = e.mem.u32(L30_RENDERER);
    e.mem.u32(e.mem.u32(renderer + 8) + 0x1a0)
}

/// The accumulator's +0x194 object.
fn l30_accumulator_object(e: &mut Engine) -> u32 {
    let accumulator = e.call(GET_ACCUMULATOR, &[]).u32();
    e.mem.u32(accumulator + 0x194)
}

/// `RenderPassArray::Add` on `list` with the eight words.
fn l30_add_pass(e: &mut Engine, list: u32, words: [u32; 8]) {
    let mut call = vec![list];
    call.extend_from_slice(&words);
    e.call(L30_ADD_PASS, &call);
}

/// The main list's last pass.
fn l30_last_pass(e: &Engine, this: u32) -> Ptr<RenderPass> {
    let list = e.mem.u32(this + L30_PASSES);
    let size = e.mem.u32(list + 0x10);
    let items = e.mem.u32(list + 4);
    Ptr::new(
        e.mem
            .u32(items.wrapping_add(size.wrapping_mul(4)).wrapping_sub(4)),
    )
}

/// A float as `FLD dword` loads it.
fn l30_load(e: &Engine, addr: u32) -> f64 {
    x87_load32(e.mem.u32(addr))
}

/// The first pass of `list` gets the renderer's light in its first light slot when it has
/// lights.
fn l30_refresh_first_light(e: &mut Engine, list: u32) {
    let pass: Ptr<RenderPass> = Ptr::new(e.mem.u32(e.mem.u32(list + 4)));
    if e.get(pass, RenderPass::ucNumLights) > 0 {
        let light = l30_renderer_light(e);
        let lights = e.get(pass, RenderPass::ppSceneLights);
        e.mem.set_u32(lights, light);
    }
}

/// The exe's fade-distance crossing test (two `FCOM` pairs, every compare ordered): the
/// limit is above the new distance and at most the old one, or at most the new one and
/// above the old one.
fn l30_crossed(new: f64, old: f64, limit: f64) -> bool {
    (limit > new && limit <= old) || (new >= limit && limit > old)
}

/// The accumulation hint from flags 0x1a and 0x1b (1 with 0x35, else 4 or 5), or `None`.
fn l30_blend_hint(e: &mut Engine, this: u32) -> Option<u8> {
    if l30_flag(e, this, 0x1a) {
        Some(if l30_flag(e, this, 0x35) { 1 } else { 4 })
    } else if l30_flag(e, this, 0x1b) {
        Some(if l30_flag(e, this, 0x35) { 1 } else { 5 })
    } else {
        None
    }
}

/// The main pass enum of the rebuild (`None`: it keeps the 0 it was added with), by the
/// flags the exe tests in its decision tree's order.
#[allow(clippy::too_many_arguments)]
fn l30_main_pass_enum(
    flag_12: bool,
    specular: bool,
    flag_1: bool,
    flag_0b: bool,
    flag_1c: bool,
    flag_0a: bool,
    has_b4: bool,
    colours: bool,
    shadowed: bool,
) -> Option<u16> {
    let pick = |yes: u16, no: u16| if shadowed { yes } else { no };
    if flag_12 {
        return Some(if specular {
            pick(0x1ae, 0x190)
        } else {
            pick(0x19f, 0x181)
        });
    }
    let value = match (specular, flag_1) {
        (true, false) => {
            if flag_0b {
                match (colours, flag_1c) {
                    (true, true) => pick(0x1b8, 0x19a),
                    (true, false) => pick(0x1b7, 0x199),
                    (false, true) => pick(0x1b0, 0x192),
                    (false, false) => pick(0x1af, 0x191),
                }
            } else if flag_0a {
                pick(0x1b1, 0x193)
            } else if has_b4 {
                if colours {
                    pick(0x1b9, 0x19b)
                } else {
                    pick(0x1b3, 0x195)
                }
            } else if colours {
                pick(0x1b5, 0x197)
            } else {
                pick(0x1ac, 0x18e)
            }
        }
        (true, true) => {
            if flag_0b {
                return None;
            } else if flag_0a {
                pick(0x1b2, 0x194)
            } else if has_b4 {
                if colours {
                    pick(0x1ba, 0x19c)
                } else {
                    pick(0x1b4, 0x196)
                }
            } else if colours {
                pick(0x1b6, 0x198)
            } else {
                pick(0x1ad, 0x18f)
            }
        }
        (false, false) => {
            if flag_0b {
                match (colours, flag_1c) {
                    (true, true) => pick(0x1a9, 0x18b),
                    (true, false) => pick(0x1a8, 0x18a),
                    (false, true) => pick(0x1a1, 0x183),
                    (false, false) => pick(0x1a0, 0x182),
                }
            } else if flag_0a {
                pick(0x1a2, 0x184)
            } else if has_b4 {
                if colours {
                    pick(0x1aa, 0x18c)
                } else {
                    pick(0x1a4, 0x186)
                }
            } else if colours {
                pick(0x1a6, 0x188)
            } else {
                pick(0x19d, 0x17f)
            }
        }
        (false, true) => {
            if flag_0b {
                return None;
            } else if flag_0a {
                pick(0x1a3, 0x185)
            } else if has_b4 {
                if colours {
                    pick(0x1ab, 0x18d)
                } else {
                    pick(0x1a5, 0x187)
                }
            } else if colours {
                pick(0x1a7, 0x189)
            } else {
                pick(0x19e, 0x180)
            }
        }
    };
    Some(value)
}

/// The light refresh for an unchanged pass state (the exe's three light iterators at
/// `frame`): each main-list pass gets its lights again; a shadow pass without a usable
/// shadow light clears the state and stops. `count_word` is the stack word whose low byte
/// holds the light count passed to `00ba8c00`.
fn l30_refresh_lights(e: &mut Engine, this: u32, frame: u32, word: u32, count_word: &mut u32) {
    let first_iterator = frame;
    let second_iterator = frame + 0x10;
    let third_iterator = frame + 0x14;
    let list = e.mem.u32(this + L30_PASSES);
    if e.mem.u32(list + 0x10) == 0 {
        return;
    }
    let mut index = 0u32;
    loop {
        let list = e.mem.u32(this + L30_PASSES);
        let items = e.mem.u32(list + 4);
        let pass: Ptr<RenderPass> = Ptr::new(e.mem.u32(items.wrapping_add(index * 4)));
        let pass_enum = e.get(pass, RenderPass::usPassEnum);
        if (0x17f..=0x1ba).contains(&pass_enum) {
            let mut count: u8 = 1;
            *count_word = (*count_word & 0xffff_ff00) | 1;
            let mut light = e
                .call(L30_FIRST_NON_SHADOW_LIGHT, &args![this, first_iterator])
                .u32();
            let mut skip = false;
            if light != 0 {
                while light != 0 {
                    count = count.wrapping_add(1);
                    light = e
                        .call(L30_NEXT_NON_SHADOW_LIGHT, &args![this, first_iterator])
                        .u32();
                }
                *count_word = (*count_word & 0xffff_ff00) | u32::from(count);
                if count == 0 {
                    e.set(pass, RenderPass::ucNumLights, 0);
                    skip = true;
                }
            }
            if !skip {
                if e.get(pass, RenderPass::ucMaxNumLights) < count {
                    e.call(L30_PASS_FREE_LIGHTS, &args![pass]);
                    e.call(L30_PASS_ALLOCATE_LIGHTS, &args![pass, *count_word]);
                    e.set(pass, RenderPass::ucMaxNumLights, count);
                }
                let mut light = e
                    .call(L30_FIRST_NON_SHADOW_LIGHT, &args![this, first_iterator])
                    .u32();
                let lights = e.get(pass, RenderPass::ppSceneLights);
                e.mem.set_u32(lights, word);
                let mut stored: u32 = 1;
                if light != 0 {
                    let limit = *count_word & 0xff;
                    while (stored as i32) < limit as i32 {
                        let lights = e.get(pass, RenderPass::ppSceneLights);
                        e.mem.set_u32(lights + stored * 4, light);
                        stored += 1;
                        light = e
                            .call(L30_NEXT_NON_SHADOW_LIGHT, &args![this, first_iterator])
                            .u32();
                        if light == 0 {
                            break;
                        }
                    }
                }
                e.set(pass, RenderPass::ucNumLights, stored as u8);
            }
        } else if (0x1bd..=0x1c3).contains(&pass_enum) {
            if e.get(pass, RenderPass::ucNumLights) == 1 {
                let light = e
                    .call(L30_FIRST_NON_SHADOW_LIGHT, &args![this, second_iterator])
                    .u32();
                let lights = e.get(pass, RenderPass::ppSceneLights);
                e.mem.set_u32(lights, if light != 0 { light } else { word });
            }
        } else if (0x1c4..=0x1c7).contains(&pass_enum) && e.get(pass, RenderPass::ucNumLights) == 1
        {
            let light = e
                .call(L30_FIRST_ACTIVE_LIGHT, &args![this, third_iterator])
                .u32();
            if light == 0
                || e.mem.u8(light + 0xec) == 0
                || e.mem.u8(e.mem.u32(light + 0xf8) + 0x30) & 1 != 0
            {
                e.mem.set_u32(this + L30_LAST_STATE, 0);
                return;
            }
            let lights = e.get(pass, RenderPass::ppSceneLights);
            e.mem.set_u32(lights, light);
        }
        index += 1;
        let list = e.mem.u32(this + L30_PASSES);
        if index >= e.mem.u32(list + 0x10) {
            return;
        }
    }
}

// Translated from 00c058f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: slot +0xa0 of the vtable `010bcb60` (`GetRenderPasses` in the
/// Xbox PDB's slot order; the class `fn_00c05880` constructs, built on the
/// `Lighting30ShaderProperty` class (the `NiRTTI` its inherited slot +8, `00bb45c0`,
/// answers), whose `GetRenderPasses` is `00bb4740`; `thiscall`, six words: the geometry, the
/// render mode, a word not read, the pass kind, the accumulator and a word not read).
///
/// The same code as `00bb4740` but for three points: no part depends on bit 23 of the second
/// flag word (`00bb4740`'s "wide" test), so the main fade's crossing test always runs; after
/// it the exe stores 1 into the main-fade byte when the distance reached the limit (the
/// byte is already 1 there, so it changes nothing); and the flag-7 passes do not depend on
/// the `011f9460` distance. Kinds 7, 8, 9 and 0xf make their array and pass; any other kind
/// measures the camera distance against the fades, sets the alpha, resorts the lights when
/// asked, refreshes the lights of an unchanged state or rebuilds the main list (the main
/// pass with its lights and enum, shadow-light passes, the translucency and +0xdc passes,
/// the flag-7 passes) and stores the state `kind << 8 | render mode`. The render mode's
/// stack slot is reused as the exe reuses it (fade byte, alpha, light count, flag 1).
#[allow(clippy::too_many_arguments)]
pub fn fn_00c058f0(
    e: &mut Engine,
    this: Ptr,
    geometry: u32,
    render_mode: u32,
    _unused_3: u32,
    kind: u32,
    accumulator: u32,
    _unused_6: u32,
) -> u32 {
    let base = this.addr();
    let state = (kind << 8) | render_mode;
    let properties = geometry.wrapping_add(0x9c);

    if kind == 7 {
        let flags = e.mem.u32(base + L30_FLAGS0);
        if flags & 0x8000 != 0 || flags & 0x0001_0000 != 0 {
            return 0;
        }
        e.call(L30_CHECK_CREATE_DEPTH_PASSES, &args![base, 1u32]);
        let list = e.mem.u32(base + L30_DEPTH_PASSES);
        if e.mem.u32(list + 0x10) != 0 {
            l30_refresh_first_light(e, list);
            return e.mem.u32(base + L30_DEPTH_PASSES);
        }
        let data = e.mem.u32(geometry + 0xb8);
        let colours = e.mem.u32(data + 0x28) != 0;
        let flag_1 = l30_flag(e, base, 1);
        let light = l30_renderer_light(e);
        let pass_enum = match (colours, flag_1) {
            (true, true) => 0x1cb,
            (true, false) => 0x1c9,
            (false, true) => 0x1ca,
            (false, false) => 0x1c8,
        };
        l30_add_pass(e, list, [geometry, pass_enum, 1, 1, light, 0, 0, 0]);
        return e.mem.u32(base + L30_DEPTH_PASSES);
    }
    if kind == 8 {
        let flags = e.mem.u32(base + L30_FLAGS0);
        if flags & 0x8000 != 0
            || flags & 0x0001_0000 != 0
            || e.mem.u8(base + L30_FLAGS1) & 0x10 != 0
        {
            return e.mem.u32(base + L30_CONST_ALPHA_PASSES);
        }
        e.call(L30_CHECK_CREATE_CONST_ALPHA_PASSES, &args![base, 1u32]);
        let list = e.mem.u32(base + L30_CONST_ALPHA_PASSES);
        if e.mem.u32(list + 0x10) != 0 {
            l30_refresh_first_light(e, list);
            return e.mem.u32(base + L30_CONST_ALPHA_PASSES);
        }
        let data = e.mem.u32(geometry + 0xb8);
        let colours = e.mem.u32(data + 0x28) != 0;
        let flag_1 = l30_flag(e, base, 1);
        let light = l30_renderer_light(e);
        let pass_enum = match (flag_1, colours) {
            (false, true) => 0x1d4,
            (false, false) => 0x1d3,
            (true, true) => 0x1d6,
            (true, false) => 0x1d5,
        };
        l30_add_pass(e, list, [geometry, pass_enum, 1, 1, light, 0, 0, 0]);
        return e.mem.u32(base + L30_CONST_ALPHA_PASSES);
    }
    if kind == 9 {
        e.call(L30_CHECK_CREATE_LOCAL_MAP_PASSES, &args![base, 1u32]);
        let list = e.mem.u32(base + L30_LOCAL_MAP_PASSES);
        if e.mem.u32(list + 0x10) == 0 {
            let pass_enum = if e.mem.u32(base + L30_FLAGS0) & 0x0010_0000 != 0 {
                0x1df
            } else if l30_flag(e, base, 1) {
                0x1de
            } else {
                0x1dd
            };
            l30_add_pass(e, list, [geometry, pass_enum, 1, 0, 0, 0, 0, 0]);
        }
        return e.mem.u32(base + L30_LOCAL_MAP_PASSES);
    }
    if kind == 0xf {
        e.call(L30_CHECK_CREATE_PASSES_54, &args![base, 1u32]);
        let list = e.mem.u32(base + L30_PASSES_54);
        if e.mem.u32(list + 0x10) == 0 && e.mem.u32(geometry + 0xbc) == 0 {
            let pass_enum = if l30_flag(e, base, 0x25) {
                0x1bc
            } else {
                0x1bb
            };
            l30_add_pass(e, list, [geometry, pass_enum, 1, 0, 0, 0, 0, 0]);
        }
        return e.mem.u32(base + L30_PASSES_54);
    }

    e.with_stack(0x40, |e, frame| {
        let frame = frame.addr();
        // The render mode's stack slot, reused by the exe.
        let mut slot = render_mode;
        let mut fade_allowed = true;
        let mut translucent = false;
        let old = x87_load32(x87_quiet_bits(e.mem.u32(base + L30_CAMERA_DISTANCE)));
        let flags = e.mem.u32(base + L30_FLAGS0);
        let fade_80 = flags & 0x80 != 0 && 0.0 < l30_load(e, L30_FADE_FAR_011F9460);
        let fade_20000 = flags & 0x0002_0000 != 0 && 0.0 < l30_load(e, L30_FADE_FAR_011F9468);
        slot = (slot & 0xffff_ff00) | u32::from(fade_20000);
        let lod_on = flags & 0x0400_0000 != 0
            || e.call(GEOMETRY_FLAG_004A2020, &args![base, 0x1bu32]).u8() == 1;
        let lod_fade =
            lod_on && 0.0 < l30_load(e, L30_FADE_FAR_011F9470) && !l30_flag(e, base, 0x23);
        let main_fade = flags & 1 != 0 && 0.0 < l30_load(e, L30_FADE_FAR_011F9458);
        let below_main = main_fade;

        if fade_20000 || fade_80 || lod_fade || main_fade {
            let camera = e.mem.u32(accumulator + 0x194);
            let mut bound = e.mem.u32(geometry + 0x20);
            if bound == 0 {
                bound = L30_DEFAULT_BOUND;
            }
            // The camera position at `frame + 0x18`, the bound at `+0x30`, the difference
            // at `+0x24`.
            for word in 0..3 {
                let value = e.mem.u32(camera + 0x1f0 + 4 * word);
                e.mem.set_u32(frame + 0x18 + 4 * word, value);
            }
            for word in 0..4 {
                let value = e.mem.u32(bound + 4 * word);
                e.mem.set_u32(frame + 0x30 + 4 * word, value);
            }
            for word in 0..3 {
                let difference = x87_sub(
                    l30_load(e, frame + 0x30 + 4 * word),
                    x87_m32(e.mem.u32(frame + 0x18 + 4 * word)),
                );
                e.mem
                    .set_u32(frame + 0x24 + 4 * word, x87_store32(difference));
            }
            let length = e.call(L30_VECTOR_LENGTH, &args![frame + 0x24]).f64();
            let distance = x87_sub(length, x87_m32(e.mem.u32(frame + 0x3c)));
            e.mem
                .set_u32(base + L30_CAMERA_DISTANCE, x87_store32(distance));
            let accumulator_now = e.call(GET_ACCUMULATOR, &[]).u32();
            if e.mem.u32(accumulator_now + 8) != 0 {
                let accumulator_now = e.call(GET_ACCUMULATOR, &[]).u32();
                let object = e.mem.u32(accumulator_now + 8);
                let scaled = x87_mul(
                    l30_load(e, object + 0x110),
                    x87_m32(e.mem.u32(base + L30_CAMERA_DISTANCE)),
                );
                e.mem
                    .set_u32(base + L30_CAMERA_DISTANCE, x87_store32(scaled));
            }
            let new = l30_load(e, base + L30_CAMERA_DISTANCE);
            // `FUCOMP; TEST AH,0x44; JNP`: on unless the distance is unchanged.
            if !(old == new) {
                if fade_20000 {
                    if l30_crossed(new, old, l30_load(e, L30_FADE_FAR_011F9468)) {
                        e.mem.set_u32(base + L30_LAST_STATE, 0);
                    }
                } else if fade_80 && l30_crossed(new, old, l30_load(e, L30_FADE_FAR_011F9460)) {
                    e.mem.set_u32(base + L30_LAST_STATE, 0);
                }
                if lod_fade && l30_crossed(new, old, l30_load(e, L30_FADE_FAR_011F9470)) {
                    e.mem.set_u32(base + L30_LAST_STATE, 0);
                }
                if main_fade {
                    let limit = l30_load(e, L30_FADE_FAR_011F9458);
                    if l30_crossed(new, old, limit) {
                        e.mem.set_u32(base + L30_LAST_STATE, 0);
                    }
                    // The exe then stores 1 into the main-fade byte when the distance is
                    // not below the limit; the byte is already 1 here.
                }
            }
        }

        if properties != 0 {
            let material = e.mem.u32(geometry + 0xa4);
            slot = 1.0f32.to_bits();
            if material == 0 {
                slot = x87_quiet_bits(e.mem.u32(base + L30_FADE_ALPHA));
            } else if !lod_fade {
                let alpha = x87_mul(
                    l30_load(e, material + 0x3c),
                    x87_m32(e.mem.u32(base + L30_FADE_ALPHA)),
                );
                slot = x87_store32(alpha);
            } else {
                let far = l30_load(e, L30_FADE_FAR_011F9470);
                let distance = l30_load(e, base + L30_CAMERA_DISTANCE);
                if !(0.0 == far) && distance >= far {
                    fade_allowed = false;
                } else {
                    let near = l30_load(e, L30_FADE_NEAR_011F946C);
                    if !(near == 0.0) && !(0.0 == far) && distance >= near && distance < far {
                        let faded = e
                            .call(
                                L30_INTERPOLATE,
                                &args![
                                    0u32,
                                    1.0f32.to_bits(),
                                    x87_store32(far),
                                    x87_store32(near),
                                    x87_store32(distance)
                                ],
                            )
                            .f64();
                        slot = x87_store32(faded);
                    } else {
                        let alpha = x87_mul(
                            l30_load(e, material + 0x3c),
                            x87_m32(e.mem.u32(base + L30_FADE_ALPHA)),
                        );
                        slot = x87_store32(alpha);
                    }
                }
            }
            e.call(L30_SET_ALPHA, &args![base, x87_quiet_bits(slot)]);
            if 1.0 > x87_m32(slot) {
                translucent = true;
            }
        }

        if e.mem.u8(base + L30_LIGHTS_DIRTY) != 0 {
            let mut bound = e.mem.u32(geometry + 0x20);
            if bound == 0 {
                bound = L30_DEFAULT_BOUND;
            }
            for word in 0..4 {
                let value = e.mem.u32(bound + 4 * word);
                e.mem.set_u32(frame + 0x30 + 4 * word, value);
            }
            e.call(L30_RESORT_LIGHTS, &args![base, frame + 0x30]);
        }

        if e.mem.u32(base + L30_LAST_STATE) == state && e.mem.u8(base + L30_LIGHTS_DIRTY) != 0 {
            let object = l30_accumulator_object(e);
            let word = e.mem.u32(object + 0xe0);
            l30_refresh_lights(e, base, frame, word, &mut slot);
            e.mem.set_u8(base + L30_LIGHTS_DIRTY, 0);
        }

        if e.mem.u32(base + L30_LAST_STATE) == state {
            return;
        }
        e.call(L30_CHECK_CREATE_PASSES, &args![base, u32::MAX]);
        let list = e.mem.u32(base + L30_PASSES);
        e.mem.set_u32(list + 0x10, 0);
        e.mem.set_u8(base + L30_LIGHTS_DIRTY, 0);
        let enabled = e.mem.u32(base + L30_FLAGS0) & 0x0010_0000 == 0
            || e.mem.u8(L30_PASSES_FORCED_011F91A7) != 0;
        let fade_flags = fade_allowed && (l30_flag(e, base, 0x1a) || l30_flag(e, base, 0x1b));
        // `FLDZ; FCOMP [alpha]; TEST AH,0x44; JNP`: stop on a zero alpha.
        if 0.0 == x87_m32(e.mem.u32(base + L30_ALPHA)) || !enabled || (lod_fade && !fade_flags) {
            return;
        }
        let flag_1 = l30_flag(e, base, 1);
        let flag_7 = l30_flag(e, base, 7);
        let flag_11 = l30_flag(e, base, 0x11);
        let flag_15 = l30_flag(e, base, 0x15);
        let flag_0f = l30_flag(e, base, 0xf);
        let flag_10 = l30_flag(e, base, 0x10);
        let flag_0b = l30_flag(e, base, 0xb);
        let flag_1c = l30_flag(e, base, 0x1c);
        let flag_0a = l30_flag(e, base, 0xa);
        let has_b4 = e.mem.u32(e.mem.u32(base + L30_WORD_B4)) != 0;
        let specular = l30_flag(e, base, 0) && state & 8 != 0 && below_main;
        let alpha_blend = translucent || {
            let property_state = e.mem.u32(geometry + 0x9c);
            e.mem.u8(property_state + 0x18) & 1 != 0
        };
        let mut shadowed = e.call(L30_SETTING_TEST_00B60330, &[]).bool()
            && !l30_flag(e, base, 0xd)
            && e.call(L30_SETTING_VALUE_00B60170, &[]).u32() != 0
            && {
                let object = l30_accumulator_object(e);
                e.mem.u8(object + 0x120) != 1
            }
            && {
                let object = l30_accumulator_object(e);
                e.mem.u8(object + 0x120) != 3
            };
        let data = e.mem.u32(geometry + 0xb8);
        let colours = e.mem.u32(data + 0x28) != 0 || l30_flag(e, base, 0x25);
        if e.mem.u32(base + L30_OBJECT_DC) != 0 {
            shadowed = false;
        }
        let one = u32::from(flag_1);
        let colour = u32::from(colours);

        if flag_0f {
            let pass_enum = if flag_10 {
                0x1d0
            } else if colours {
                0x1cd + 2 * one
            } else {
                0x1cc + 2 * one
            };
            let list = e.mem.u32(base + L30_PASSES);
            l30_add_pass(e, list, [geometry, pass_enum, 1, 0, 0, 0, 0, 0]);
            let pass = l30_last_pass(e, base);
            e.set(pass, RenderPass::eAccumulationHint, 6);
            e.mem.set_u32(base + L30_LAST_STATE, state);
            return;
        }
        if e.mem.u8(base + L30_FLAGS1) & 0x40 != 0 {
            let list = e.mem.u32(base + L30_PASSES);
            l30_add_pass(e, list, [geometry, 0x1d1 + one, 1, 0, 0, 0, 0, 0]);
            let pass = l30_last_pass(e, base);
            e.set(pass, RenderPass::eAccumulationHint, 7);
        }
        let word = {
            let object = l30_accumulator_object(e);
            e.mem.u32(object + 0xe0)
        };
        let list = e.mem.u32(base + L30_PASSES);
        l30_add_pass(e, list, [geometry, 0, 1, 0, 0, 0, 0, 0]);
        let pass = l30_last_pass(e, base);
        let iterator = frame;
        e.set(pass, RenderPass::ucNumLights, 1);
        let mut light = e.call(L30_FIRST_ACTIVE_LIGHT, &args![base, iterator]).u32();
        while light != 0 {
            if e.mem.u8(light + 0xec) == 0 {
                let count = e.get(pass, RenderPass::ucNumLights);
                e.set(pass, RenderPass::ucNumLights, count.wrapping_add(1));
            }
            light = e.call(L30_NEXT_ACTIVE_LIGHT, &args![base, iterator]).u32();
        }
        let count = e.get(pass, RenderPass::ucNumLights);
        if count == 0 {
            e.set(pass, RenderPass::ppSceneLights, 0);
        } else {
            if e.get(pass, RenderPass::ucMaxNumLights) < count {
                e.call(L30_PASS_FREE_LIGHTS, &args![pass]);
                let count = u32::from(e.get(pass, RenderPass::ucNumLights));
                e.call(L30_PASS_ALLOCATE_LIGHTS, &args![pass, count]);
                let count = e.get(pass, RenderPass::ucNumLights);
                e.set(pass, RenderPass::ucMaxNumLights, count);
            }
            let mut light = e.call(L30_FIRST_ACTIVE_LIGHT, &args![base, iterator]).u32();
            let lights = e.get(pass, RenderPass::ppSceneLights);
            e.mem.set_u32(lights, word);
            let mut stored: u8 = 1;
            while light != 0 {
                if e.mem.u8(light + 0xec) == 0 {
                    let lights = e.get(pass, RenderPass::ppSceneLights);
                    e.mem.set_u32(lights + u32::from(stored) * 4, light);
                    stored = stored.wrapping_add(1);
                }
                light = e.call(L30_NEXT_ACTIVE_LIGHT, &args![base, iterator]).u32();
            }
            e.set(pass, RenderPass::ucNumLights, stored);
        }
        e.call(L30_SET_FLAG, &args![base, 0x1eu32, 0u32]);
        let flag_12 = l30_flag(e, base, 0x12);
        if let Some(pass_enum) = l30_main_pass_enum(
            flag_12, specular, flag_1, flag_0b, flag_1c, flag_0a, has_b4, colours, shadowed,
        ) {
            e.set(pass, RenderPass::usPassEnum, pass_enum);
        }
        if let Some(hint) = l30_blend_hint(e, base) {
            e.set(pass, RenderPass::eAccumulationHint, hint);
        } else if alpha_blend {
            // `FLD1; FCOMP [fade]; TEST AH,0x41; JZ`: 0xe only when 1 is above.
            let hint = if 1.0 > x87_m32(e.mem.u32(base + L30_FADE_ALPHA)) {
                0xe
            } else {
                1
            };
            e.set(pass, RenderPass::eAccumulationHint, hint);
        } else if e.mem.u32(accumulator + 0x34) == 2
            || l30_flag(e, base, 0x22)
            || l30_flag(e, base, 0x21)
        {
            e.set(pass, RenderPass::eAccumulationHint, 0xa);
        }

        if e.call(L30_DEPTH_PASSES_WANTED, &[]).bool() {
            let mut light = e.call(L30_FIRST_ACTIVE_LIGHT, &args![base, iterator]).u32();
            while light != 0 {
                if e.mem.u8(light + 0xec) != 0 {
                    let list = e.mem.u32(base + L30_PASSES);
                    l30_add_pass(e, list, [geometry, 0, 0, 1, light, 0, 0, 0]);
                    let shadow = l30_last_pass(e, base);
                    let pass_enum = (if colours { 0x1c5 } else { 0x1c4 }) + 2 * one as u16;
                    e.set(shadow, RenderPass::usPassEnum, pass_enum);
                    if let Some(hint) = l30_blend_hint(e, base) {
                        e.set(shadow, RenderPass::eAccumulationHint, hint);
                    }
                }
                light = e.call(L30_NEXT_ACTIVE_LIGHT, &args![base, iterator]).u32();
            }
        }

        let extra = e.mem.u32(base + L30_OBJECT_DC);
        if extra != 0 && e.mem.u32(extra + 0x6c) != 0 {
            let list = e.mem.u32(base + L30_PASSES);
            l30_add_pass(e, list, [geometry, 0xe + one, 1, 0, 0, 0, 0, 0]);
            let pass = l30_last_pass(e, base);
            e.set(pass, RenderPass::eAccumulationHint, 0xe);
        } else if (l30_flag(e, base, 9) || 1.0 > x87_m32(e.mem.u32(base + L30_FADE_ALPHA))) && {
            let property_state = e.mem.u32(geometry + 0x9c);
            property_state == 0 || e.mem.u8(property_state + 0x18) & 1 == 0
        } {
            let pass_enum = if colours { 0xc + one } else { 0xa + one };
            let list = e.mem.u32(base + L30_PASSES);
            l30_add_pass(e, list, [geometry, pass_enum, 1, 0, 0, 0, 0, 0]);
            let pass = l30_last_pass(e, base);
            let hint = if l30_flag(e, base, 9) { 1 } else { 0xe };
            e.set(pass, RenderPass::eAccumulationHint, hint);
            if 1.0 > x87_m32(e.mem.u32(base + L30_FADE_ALPHA))
                || 1.0 > x87_m32(e.mem.u32(base + L30_ALPHA))
            {
                e.set(pass, RenderPass::eAccumulationHint, 0xe);
            }
        }
        if e.mem.u32(base + L30_OBJECT_DC) != 0 {
            let list = e.mem.u32(base + L30_PASSES);
            l30_add_pass(e, list, [geometry, 0x1e0 + one, 1, 0, 0, 0, 0, 0]);
            let pass = l30_last_pass(e, base);
            e.set(pass, RenderPass::eAccumulationHint, 1);
        }
        if flag_7 {
            let pass = if !flag_15 && flag_11 {
                let light = e.call(L30_FIRST_ACTIVE_LIGHT, &args![base, iterator]).u32();
                let light = if light != 0 { light } else { word };
                let list = e.mem.u32(base + L30_PASSES);
                l30_add_pass(e, list, [geometry, 0, 0, 1, light, 0, 0, 0]);
                let pass = l30_last_pass(e, base);
                e.set(pass, RenderPass::usPassEnum, 0x1c3);
                pass
            } else {
                let pass_enum = if flag_15 {
                    0x1be + 2 * colour
                } else if flag_1 {
                    0x1c1 + colour
                } else {
                    0x1bd + 2 * colour
                };
                let list = e.mem.u32(base + L30_PASSES);
                l30_add_pass(e, list, [geometry, 0, 0, 0, 0, 0, 0, 0]);
                let pass = l30_last_pass(e, base);
                e.set(pass, RenderPass::usPassEnum, pass_enum as u16);
                pass
            };
            if alpha_blend {
                e.set(pass, RenderPass::eAccumulationHint, 1);
            }
        }
        e.mem.set_u32(base + L30_LAST_STATE, state);
    });
    e.mem.u32(base + L30_PASSES)
}

/// `00b7a870` (`cdecl`, two words; run_00b79de0 unit) and `NiTriStrips::CreateClone`
/// (`00a71db0`, engine map).
const CALL_00B7A870: u32 = 0x00b7_a870;
const STRIPS_CREATE_CLONE: u32 = 0x00a7_1db0;

// Translated from 00c1a2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): slot
/// +0x158 of the shader vtable at `010bc070` (`thiscall`, one word, `this` not read):
/// `00b7a870(word & 0xffff, 0)`; ends in that call, so its `EAX` is answered.
pub fn fn_00c1a2f0(e: &mut Engine, _this: Ptr, pass: u32) -> u32 {
    e.call(CALL_00B7A870, &args![pass & 0xffff, 0u32]).u32()
}

// Translated from 00c1a390 (decompiled, FalloutNV.exe 1.4.0.525)
/// No function in the engine map (Ghidra made none; disassembled from its bytes): slot
/// +0x48 of the `TallGrassTriStrips` vtable `010bcc90` (`CreateClone`; `thiscall`, one
/// word): a jump to `NiTriStrips::CreateClone`; its answer.
pub fn fn_00c1a390(e: &mut Engine, this: Ptr, process: u32) -> u32 {
    e.call(STRIPS_CREATE_CLONE, &args![this, process]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00c0_4960, fn_00c04960(Ptr) -> u32),
        entry!(0x00c0_4b90, fn_00c04b90(Ptr, u32) -> bool),
        entry!(0x00c0_4d80, fn_00c04d80(Ptr, u8)),
        entry!(0x00c0_4e30, fn_00c04e30(Ptr, u32) -> bool),
        entry!(0x00c0_5370, fn_00c05370(Ptr, u32) -> u32),
        entry!(0x00c0_53c0, fn_00c053c0(Ptr, u32) -> bool),
        entry!(0x00c0_5730, fn_00c05730(Ptr, u32) -> bool),
        entry!(0x00c0_5870, fn_00c05870(Ptr) -> f32),
        entry!(0x00c0_5880, fn_00c05880(Ptr) -> Ptr),
        entry!(0x00c0_58c0, fn_00c058c0(Ptr, u32) -> Ptr),
        entry!(0x00c0_6ee0, fn_00c06ee0(Ptr, u32)),
        entry!(0x00c0_6f30, fn_00c06f30(Ptr, u32)),
        entry!(0x00c0_bae0, fn_00c0bae0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00c0_bb50, fn_00c0bb50(Ptr, u32, u32, u8)),
        entry!(0x00c0_bba0, fn_00c0bba0(Ptr, u32, u32)),
        entry!(0x00c0_bbd0, fn_00c0bbd0(Ptr, u32)),
        entry!(0x00c0_bc00, fn_00c0bc00(Ptr, u32, u32, u32) -> u32),
        entry!(0x00c0_bc90, fn_00c0bc90(Ptr, u32, u32) -> u32),
        entry!(0x00c0_bcf0, fn_00c0bcf0(Ptr, u32, u32, u32) -> u32),
        entry!(0x00c1_a2f0, fn_00c1a2f0(Ptr, u32) -> u32),
        entry!(0x00c1_a390, fn_00c1a390(Ptr, u32) -> u32),
        entry!(0x00c0_bd40, fn_00c0bd40(Ptr, u32, u32) -> u32),
        entry!(0x00c0_58f0, fn_00c058f0(Ptr, u32, u32, u32, u32, u32, u32) -> u32),
        entry!(0x00c0_72c0, fn_00c072c0(Ptr)),
        entry!(0x00c0_abd0, fn_00c0abd0(Ptr, u32) -> u32),
        entry!(0x00c0_d200, fn_00c0d200(Ptr)),
        entry!(0x00c0_e630, fn_00c0e630(Ptr)),
        entry!(0x00c0_fdc0, fn_00c0fdc0(Ptr)),
        entry!(0x00c1_1980, fn_00c11980(Ptr)),
        entry!(0x00c1_24c0, fn_00c124c0(Ptr)),
        entry!(0x00c1_6400, fn_00c16400(Ptr)),
        entry!(0x00c1_7510, fn_00c17510(Ptr)),
        entry!(0x00c0_48b0, fn_00c048b0(Ptr)),
        entry!(0x00c0_4950, fn_00c04950(Ptr) -> u32),
        entry!(0x00c0_4990, fn_00c04990(Ptr) -> bool),
        entry!(0x00c0_4a80, fn_00c04a80(Ptr, u32) -> Ptr),
        entry!(0x00c0_4aa0, fn_00c04aa0(Ptr, Ptr, u32)),
        entry!(0x00c0_4d50, fn_00c04d50(Ptr, u32)),
        entry!(0x00c0_4d60, fn_00c04d60(Ptr) -> bool),
        entry!(0x00c0_4da0, fn_00c04da0(Ptr)),
        entry!(0x00c0_4f50, fn_00c04f50(Ptr) -> u32),
        entry!(0x00c0_50b0, fn_00c050b0(Ptr, u32, u32)),
        entry!(0x00c0_5160, fn_00c05160(Ptr, u32) -> Ptr),
        entry!(0x00c0_5180, fn_00c05180(Ptr, Ptr, u32)),
        entry!(0x00c0_5250, fn_00c05250(Ptr)),
        entry!(0x00c0_5350, fn_00c05350(Ptr) -> bool),
        entry!(0x00c0_54a0, fn_00c054a0(Ptr, u32) -> Ptr),
        entry!(0x00c0_54c0, fn_00c054c0(Ptr, Ptr, u32)),
        entry!(0x00c0_56a0, fn_00c056a0(Ptr)),
        entry!(0x00c0_5760, fn_00c05760(Ptr, u32) -> Ptr),
        entry!(0x00c0_5780, fn_00c05780(Ptr, Ptr, u32)),
        entry!(
            0x00c0_6dc0,
            fn_00c06dc0(u32, u32, u32, u32, u32, u32, u32, u32) -> u32
        ),
        entry!(0x00c0_6e10, fn_00c06e10(Ptr, u32)),
        entry!(0x00c0_6f80, fn_00c06f80(Ptr)),
        entry!(0x00c0_bb20, fn_00c0bb20(Ptr, Ptr, Ptr)),
        entry!(0x00c0_cd80, fn_00c0cd80(Ptr, u32, Ptr)),
        entry!(0x00c0_d170, fn_00c0d170(Ptr) -> bool),
        entry!(0x00c0_d1b0, fn_00c0d1b0(Ptr) -> u32),
        entry!(0x00c1_13b0, fn_00c113b0(Ptr)),
        entry!(0x00c1_1790, fn_00c11790(Ptr)),
        entry!(0x00c1_2480, fn_00c12480(Ptr) -> bool),
        entry!(0x00c1_6350, fn_00c16350(Ptr) -> bool),
        entry!(0x00c1_63a0, fn_00c163a0(u32) -> u32),
        entry!(0x00c1_a310, fn_00c1a310(Ptr, u32, u32, Ptr)),
        entry!(0x00c1_a3a0, fn_00c1a3a0() -> Ptr),
        entry!(0x00c1_a3b0, fn_00c1a3b0(Ptr, u32, u32, u32, f32)),
        entry!(0x00c1_a480, fn_00c1a480(Ptr, Ptr) -> Ptr),
        entry!(0x00c1a450, fn_00c1a450(Ptr, u32) -> Ptr),
        entry!(0x00c0_49c0, fn_00c049c0(Ptr) -> Ptr),
        entry!(0x00c0_4d20, fn_00c04d20(Ptr) -> Ptr),
        entry!(0x00c0_5420, fn_00c05420(Ptr) -> Ptr),
        entry!(0x00c0_5680, fn_00c05680(Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_00c04f50_answers_the_block_at_0x88() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xd0);
        let data = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x5c, data);
        e.mem.set_u32(this + 0x60, data);
        // `LEA EAX,[ECX+0x88]` at 00c04fef; 00bae560 copies the constants from it (00bae5dd).
        assert_eq!(e.call(0x00c0_4f50, &args![this]).u32(), this + 0x88);
        e.mem.set_u8(this + 0x58, 1);
        assert_eq!(e.call(0x00c0_4f50, &args![this]).u32(), this + 0x88);
    }

    #[test]
    fn fn_00c1a450_deletes_only_on_bit_0() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(0x00a8_9760, |_, _| Ret::default());
        e.register(0x00aa_1460, |_, _| Ret::default());
        let this = 0x5000u32;
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        assert_eq!(e.call(0x00c1a450, &args![this, 0xfffe_u32]).u32(), this);
        assert!(calls_to(&e, 0x00aa_1460).is_empty());
        assert_eq!(e.call(0x00c1a450, &args![this, 1u32]).u32(), this);
        assert_eq!(calls_to(&e, 0x00aa_1460), vec![vec![this, 0xd8]]);
        assert_eq!(calls_to(&e, 0x00a8_9760), vec![vec![this]; 2]);
    }
}

#[cfg(test)]
mod effect_constructor_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The base constructor fills the object with 0xee bytes and a marker vtable.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080, |e, a| {
            for offset in 0..0xc8u32 {
                e.mem.set_u8(a[0] + offset, 0xee);
            }
            e.mem.set_u32(a[0], 0x1111);
            a[0].into_ret()
        });
        e.register(ARRAY_SET_SIZE_00A90180, |e, a| {
            // The vtable is the class's own by then.
            assert_ne!(e.mem.u32(a[0] - EFFECT_TEXTURES), 0x1111);
            Ret::default()
        });
        e.register(VECTOR_CONSTRUCT, |_, _| Ret::default());
        let this = e.mem.alloc(0xc8);
        e.call_log = Some(vec![]);
        (e, this)
    }

    #[test]
    fn fn_00c049c0_constructs_the_effect() {
        let (mut e, this) = engine();
        e.map(0x0101_6000, 0x1000);
        // A signalling NaN: the FLD/FSTP copy quiets it.
        e.set_global::<u32>(VALUE_01016248, 0x7f80_0003);
        assert_eq!(e.call(0x00c0_49c0, &args![this]).u32(), this);
        assert_eq!(
            calls(&e, IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080),
            vec![vec![this]]
        );
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + EFFECT_TEXTURES, 3]]
        );
        assert_eq!(e.mem.u32(this), 0x010b_caac);
        assert_eq!(e.mem.u32(this + 0x58), 0);
        assert_eq!(e.mem.u32(this + 0x5c), 0x7fc0_0003);
        assert_eq!(e.mem.u32(this + 0x60), 0);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert_eq!(e.mem.bytes(this + 0x68, 3), vec![0; 3]);
        assert_eq!(e.mem.u8(this + 0x6b), 0xee);
        for offset in [0x6cu32, 0x70, 0x74] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
    }

    #[test]
    fn fn_00c04d20_constructs_the_effect() {
        let (mut e, this) = engine();
        assert_eq!(e.call(0x00c0_4d20, &args![this]).u32(), this);
        assert_eq!(
            calls(&e, IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080),
            vec![vec![this]]
        );
        assert_eq!(e.mem.u32(this), 0x010b_cae8);
        assert_eq!(e.mem.u8(this + 0x58), 0);
        assert_eq!(e.mem.u8(this + 0x59), 0xee);
        assert_eq!(e.mem.u32(this + 0x5c), 0x011a_e684);
        assert_eq!(e.mem.u32(this + 0x60), 0x011a_e694);
        assert_eq!(e.mem.u32(this + 0x64), 0);
        assert!(calls(&e, ARRAY_SET_SIZE_00A90180).is_empty());
    }

    #[test]
    fn fn_00c05420_constructs_the_effect() {
        let (mut e, this) = engine();
        assert_eq!(e.call(0x00c0_5420, &args![this]).u32(), this);
        let order: Vec<u32> = e.call_log.as_ref().unwrap().iter().map(|c| c.0).collect();
        assert_eq!(
            order,
            [
                0x00c0_5420,
                IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080,
                VECTOR_CONSTRUCT,
                ARRAY_SET_SIZE_00A90180
            ]
        );
        assert_eq!(
            calls(&e, VECTOR_CONSTRUCT),
            vec![vec![
                this + 0x5c,
                0x10,
                2,
                TEXTURE_CONSTRUCT_004EBB70,
                TEXTURE_DESTRUCT_00BA3A00
            ]]
        );
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + EFFECT_TEXTURES, 4]]
        );
        assert_eq!(e.mem.u32(this), 0x010b_cb18);
        assert_eq!(e.mem.u32(this + 0x58), 0);
    }

    #[test]
    fn fn_00c05680_constructs_the_effect() {
        let (mut e, this) = engine();
        assert_eq!(e.call(0x00c0_5680, &args![this]).u32(), this);
        assert_eq!(
            calls(&e, IMAGE_SPACE_EFFECT_CONSTRUCT_00BA4080),
            vec![vec![this]]
        );
        assert_eq!(e.mem.u32(this), 0x010b_cb3c);
        for offset in [0x58u32, 0x5c, 0x60, 0x64] {
            assert_eq!(e.mem.u32(this + offset), 0);
        }
        assert_eq!(e.mem.u8(this + 0x68), 0xee);
        assert!(calls(&e, ARRAY_SET_SIZE_00A90180).is_empty());
    }
}

#[cfg(test)]
mod batch_b011_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// One vtable for every object: slot `n` is the double `SLOTS + n`, answering 0.
    const VTABLE: u32 = 0x0e80_0000;
    const SLOTS: u32 = 0x0e81_0000;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn order(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .map(|c| c.0)
            .collect()
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x70u32).map(|i| SLOTS + 4 * i).collect();
        e.put_vtable(VTABLE, &slots);
        for slot in &slots {
            e.register(*slot, |_, _| Ret::default());
        }
        for global in [
            0x011a_e000u32,
            0x011f_9000,
            0x011f_a000,
            0x011f_c000,
            0x011f_d000,
            0x0120_0000,
            0x0120_1000,
            0x0120_2000,
            0x0126_f000,
            0x0101_8000,
            0x011f_4000,
            0x011a_9000,
        ] {
            e.map(global, 0x1000);
        }
        for double in [
            MEMORY_FREE,
            IMAGE_SPACE_EFFECT_DESTRUCT,
            TEXTURE_DESTRUCT_00BA3A00,
            ARRAY_SET_AT_00BA3AA0,
            EFFECT_STEP_00BA3F90,
            EFFECT_STEP_00BA3B10,
            EFFECT_STEP_00BA3B50,
            EFFECT_STEP_00BA3C70,
            SHADER_PARAM_INIT_CONSTANT_MAP,
            SHADER_PARAM_SET_PIXEL_CONSTANT,
            ARRAY_SET_SIZE_00A90180,
            VECTOR_DESTRUCT,
            PASS_STEP_00E804D0,
            SETUP_DEFAULT_STAGE,
            PASS_SET_STAGE,
            RENDER_STATE_SET_00E7F430,
            STAGE_RELEASE_00E7ECD0,
            PASS_RELEASE_00E80190,
            STAGE_SETUP_00BB9640,
            STAGE_SET_FILTER_MODE,
            STAGE_STEP_00E7DE90,
            0x00b4_f510,
            0x00a7_1cc0,
        ] {
            e.register(double, |_, _| Ret::default());
        }
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        e.register(SHADER_PARAM_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            a[0].into_ret()
        });
        e.register(RENDER_STATES_NEW_00E7F5D0, |e, _| {
            e.mem.alloc(0x10).into_ret()
        });
        e.register(CREATE_NEW_STAGE, |e, a| {
            let stage = e.mem.alloc(0x68);
            e.mem.set_u32(stage + 0x60, 1);
            e.mem.set_u32(a[0], stage);
            Ret::default()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.call_log = Some(vec![]);
        e
    }

    fn object(e: &mut Engine, size: u32) -> u32 {
        let object = e.mem.alloc(size);
        e.mem.set_u32(object, VTABLE);
        object
    }

    /// An effect whose parameter list's data (+0x1c) has `count` entries.
    fn effect(e: &mut Engine, count: u32) -> (u32, u32) {
        let this = object(e, 0xd0);
        let data = e.mem.alloc(4 * count);
        e.mem.set_u32(this + 0x1c, data);
        (this, data)
    }

    #[test]
    fn effect_destructors_delete_their_parameters() {
        for (address, vtable, count, texture) in [
            (0x00c0_48b0u32, 0x010b_caacu32, 1u32, true),
            (0x00c0_4da0, 0x010b_cae8, 1, false),
            (0x00c0_5250, 0x010b_cb18, 3, false),
            (0x00c0_56a0, 0x010b_cb3c, 1, false),
        ] {
            let mut e = engine();
            let (this, data) = effect(&mut e, count);
            let param = object(&mut e, 0x88);
            e.mem.set_u32(data + 4 * (count - 1), param);
            e.call(address, &args![this]);
            assert_eq!(e.mem.u32(this), vtable);
            assert_eq!(calls(&e, SLOTS), vec![vec![param, 1]]);
            let stores = calls(&e, ARRAY_SET_AT_00BA3AA0);
            assert_eq!(stores.len(), 1);
            assert_eq!(stores[0][..2], [this + EFFECT_PARAMS, count - 1]);
            assert_eq!(
                calls(&e, TEXTURE_DESTRUCT_00BA3A00),
                if texture {
                    vec![vec![this + 0x68]]
                } else {
                    vec![]
                }
            );
            assert_eq!(calls(&e, IMAGE_SPACE_EFFECT_DESTRUCT), vec![vec![this]]);
            if count == 3 {
                assert_eq!(
                    calls(&e, VECTOR_DESTRUCT),
                    vec![vec![this + 0x5c, 0x10, 2, TEXTURE_DESTRUCT_00BA3A00]]
                );
            }
        }
    }

    #[test]
    fn scalar_deleting_destructors_free_on_bit_0() {
        for (address, body) in [
            (0x00c0_4a80u32, IMAGE_SPACE_EFFECT_DESTRUCT),
            (0x00c0_5160, IMAGE_SPACE_EFFECT_DESTRUCT),
            (0x00c0_54a0, IMAGE_SPACE_EFFECT_DESTRUCT),
            (0x00c0_5760, IMAGE_SPACE_EFFECT_DESTRUCT),
        ] {
            let mut e = engine();
            let (this, _) = effect(&mut e, 3);
            assert_eq!(e.call(address, &args![this, 0u32]).u32(), this);
            assert!(calls(&e, MEMORY_FREE).is_empty());
            assert_eq!(e.call(address, &args![this, 1u32]).u32(), this);
            assert_eq!(calls(&e, MEMORY_FREE), vec![vec![MEMORY_MANAGER, this]]);
            assert_eq!(calls(&e, body).len(), 2);
        }
    }

    fn make_manager(e: &mut Engine) -> u32 {
        let holder = e.mem.alloc(0x100);
        for offset in (0..0x100u32).step_by(4) {
            e.mem.set_u32(holder + offset, 0x5000 + offset);
        }
        let manager = e.mem.alloc(0x10);
        e.mem.set_u32(manager + 8, holder);
        manager
    }

    #[test]
    fn effect_setups_make_their_parameters() {
        let mut e = engine();
        let (this, _) = effect(&mut e, 1);
        e.mem.set_f32(this + 0x58, 0.75);
        let manager = make_manager(&mut e);
        e.call(0x00c0_4aa0, &args![this, manager, 0u32]);
        assert_eq!(calls(&e, EFFECT_STEP_00BA3F90), vec![vec![this, 1]]);
        let param = calls(&e, SHADER_PARAM_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, SHADER_PARAM_INIT_CONSTANT_MAP),
            vec![vec![param, 0, 2]]
        );
        assert_eq!(
            calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT),
            vec![vec![param, 0, 0.75f32.to_bits(), 0, 0, 0]]
        );
        assert_eq!(
            calls(&e, EFFECT_STEP_00BA3B50),
            vec![vec![this, 0, 1, 2, 0, 1]]
        );
        assert_eq!(
            calls(&e, EFFECT_STEP_00BA3C70),
            vec![vec![this, 0, 0x5000 + 0xd4, param, 0]]
        );

        let mut e = engine();
        let (this, _) = effect(&mut e, 1);
        let manager = make_manager(&mut e);
        e.call(0x00c0_5180, &args![this, manager, 0u32]);
        let param = calls(&e, SHADER_PARAM_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, ARRAY_SET_SIZE_00A90180),
            vec![vec![this + EFFECT_TEXTURES, 2]]
        );
        assert_eq!(calls(&e, EFFECT_STEP_00BA3B10), vec![vec![this, 0, 1, 0]]);
        assert_eq!(
            calls(&e, EFFECT_STEP_00BA3C70),
            vec![vec![this, 0, 0x5000 + 0xc8, param, 0]]
        );

        let mut e = engine();
        let (this, _) = effect(&mut e, 1);
        let manager = make_manager(&mut e);
        e.call(0x00c0_5780, &args![this, manager, 0u32]);
        let one = 1.0f32.to_bits();
        let param = calls(&e, SHADER_PARAM_CONSTRUCT)[0][0];
        assert_eq!(
            calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT),
            vec![vec![param, 0, one, one, one, one]]
        );
        assert_eq!(
            calls(&e, EFFECT_STEP_00BA3B50),
            vec![vec![this, 0, 1, u32::MAX, 1, 0]]
        );
    }

    #[test]
    fn three_parameter_setup() {
        let mut e = engine();
        e.mem.set_f32(0x011a_e6c8, 2.0);
        e.mem.set_u32(0x011a_e6cc, 0x7f80_0001);
        let (this, _) = effect(&mut e, 3);
        let manager = make_manager(&mut e);
        e.call(0x00c0_54c0, &args![this, manager, 0u32]);
        let made: Vec<u32> = calls(&e, SHADER_PARAM_CONSTRUCT)
            .iter()
            .map(|c| c[0])
            .collect();
        assert_eq!(made.len(), 2);
        let zero = 0;
        let one = 1.0f32.to_bits();
        assert_eq!(
            calls(&e, SHADER_PARAM_SET_PIXEL_CONSTANT),
            vec![
                vec![made[0], 0, zero, 2.0f32.to_bits(), 0x7fc0_0001, zero],
                vec![made[1], 0, one, one, one, one]
            ]
        );
        assert_eq!(e.mem.u8(made[1] + 0x56), 0);
        let stores: Vec<u32> = calls(&e, ARRAY_SET_AT_00BA3AA0)
            .iter()
            .map(|c| c[1])
            .collect();
        assert_eq!(stores, vec![0, 1, 2]);
        assert_eq!(
            calls(&e, EFFECT_STEP_00BA3C70),
            vec![
                vec![this, 0, 0x5000 + 0xb4, made[0], 2],
                vec![this, 1, 0x5000 + 0x4c, 0, 2],
                vec![this, 2, 0x5000 + 0xb8, made[1], 0]
            ]
        );
        assert_eq!(
            calls(&e, EFFECT_STEP_00BA3B50),
            vec![vec![this, 1, 2, 3, 0, 0], vec![this, 2, 2, 1, 1, 0]]
        );
        assert_eq!(calls(&e, EFFECT_STEP_00BA3B10), vec![vec![this, 0, 1, 1]]);
    }

    #[test]
    fn is_active_slots() {
        let mut e = engine();
        let (this, _) = effect(&mut e, 1);
        e.mem.set_f32(this + 0x58, 0.5);
        assert!(!e.call(0x00c0_4990, &args![this]).bool());
        e.mem.set_u32(0x011f_91c0, 5);
        assert!(!e.call(0x00c0_4990, &args![this]).bool());
        e.mem.set_u8(0x0120_24a0, 1);
        assert!(e.call(0x00c0_4990, &args![this]).bool());
        e.mem.set_u32(this + 0x58, 0x7fc0_0000);
        assert!(!e.call(0x00c0_4990, &args![this]).bool());
        e.mem.set_f32(this + 0x58, -1.0);
        assert!(!e.call(0x00c0_4990, &args![this]).bool());

        assert!(!e.call(0x00c0_4d60, &args![this]).bool());
        e.mem.set_u32(this + 0x60, 1);
        assert!(e.call(0x00c0_4d60, &args![this]).bool());
        e.mem.set_u8(0x011f_941e, 1);
        assert!(!e.call(0x00c0_4d60, &args![this]).bool());

        assert!(!e.call(0x00c0_5350, &args![this]).bool());
        e.mem.set_u8(0x011f_9422, 1);
        assert!(e.call(0x00c0_5350, &args![this]).bool());
        e.mem.set_u32(this + 0x58, 0);
        assert!(!e.call(0x00c0_5350, &args![this]).bool());
    }

    #[test]
    fn thunk_and_small_slots() {
        let mut e = engine();
        e.call(0x00c0_4950, &args![0x1234u32]);
        assert_eq!(calls(&e, 0x00b4_f510), vec![Vec::<u32>::new()]);
        let (this, _) = effect(&mut e, 1);
        e.mem.set_u8(this + 0x58, 7);
        e.call(0x00c0_4d50, &args![this, 0x99u32]);
        assert_eq!((e.mem.u8(this + 5), e.mem.u32(this + 0x5c)), (7, 0x99));
        e.mem.set_u32(this + 0x80, 5);
        e.call(0x00c0_50b0, &args![this, 0x42u32, 0b1010u32]);
        assert_eq!(e.mem.u8(this + 5), 1);
        assert_eq!((e.mem.u32(this + 0x60), e.mem.u32(this + 0x64)), (0x42, 10));
        let values: Vec<f32> = (0..4).map(|i| e.mem.f32(this + 0x68 + 4 * i)).collect();
        assert_eq!(values, vec![0.0, 1.0, 0.0, 1.0]);
        assert_eq!(e.mem.u32(this + 0x80), 0);
        assert_eq!(e.call(0x00c1_a3a0, &args![]).u32(), 0x0120_24b8);
    }

    #[test]
    fn update_params_copies_floats() {
        let mut e = engine();
        let (this, _) = effect(&mut e, 1);
        let data = e.mem.alloc(0x30);
        for i in 0..12u32 {
            e.mem.set_f32(data + 4 * i, 10.0 + i as f32);
        }
        e.mem.set_u32(data + 0x2c, 0x7f80_0001);
        e.mem.set_u32(this + 0x60, data);
        for i in 0..4u32 {
            e.mem.set_f32(this + 0x68 + 4 * i, 100.0 + i as f32);
        }
        e.call(0x00c0_4f50, &args![this]);
        let out: Vec<u32> = (0..16).map(|i| e.mem.u32(this + 0x88 + 4 * i)).collect();
        let f = |v: f32| v.to_bits();
        assert_eq!(
            out,
            vec![
                f(10.0),
                f(11.0),
                f(12.0),
                f(13.0),
                f(14.0),
                f(15.0),
                f(16.0),
                f(17.0),
                f(100.0),
                f(101.0),
                f(102.0),
                f(103.0),
                f(18.0),
                f(19.0),
                f(20.0),
                0x7fc0_0001
            ]
        );
        // With the flag: the larger fourth value, the other's first three, 1.0s.
        let other = e.mem.alloc(0x10);
        for i in 0..4u32 {
            e.mem.set_f32(other + 4 * i, 50.0 + i as f32);
        }
        e.mem.set_u32(this + 0x5c, other);
        e.mem.set_u8(this + 0x58, 1);
        e.call(0x00c0_4f50, &args![this]);
        let out: Vec<u32> = (3..12).map(|i| e.mem.u32(this + 0x88 + 4 * i)).collect();
        assert_eq!(
            out,
            vec![
                f(53.0),
                f(50.0),
                f(51.0),
                f(52.0),
                f(1.0),
                f(100.0),
                f(101.0),
                f(1.0),
                f(1.0)
            ]
        );
        // A NaN fourth value on the data side wins.
        e.mem.set_u32(data + 0xc, 0x7fc0_0002);
        e.call(0x00c0_4f50, &args![this]);
        assert_eq!(e.mem.u32(this + 0x94), 0x7fc0_0002);
    }

    #[test]
    fn pass_step_lists_call_their_slots() {
        for (address, first, count, slots) in [
            (0x00c0_d170u32, 0x0120_2230u32, 0x21u32, vec![0x158u32]),
            (0x00c1_2480, 0x0120_22c0, 0x1a, vec![0x158]),
            (0x00c1_6350, 0x0120_2350, 0x1e, vec![0x15c]),
        ] {
            let mut e = engine();
            let this = object(&mut e, 0x10);
            assert!(e.call(address, &args![this]).bool());
            let steps = calls(&e, PASS_STEP_00E804D0);
            assert_eq!(steps.len(), count as usize);
            assert_eq!(
                (steps[0][0], steps[count as usize - 1][0]),
                (first, first + 4 * (count - 1))
            );
            let called: Vec<u32> = slots
                .iter()
                .flat_map(|s| calls(&e, SLOTS + s))
                .map(|c| c[0])
                .collect();
            assert_eq!(called, vec![this]);
        }
        let mut e = engine();
        let this = object(&mut e, 0x10);
        e.mem.set_u32(0x011f_91c0, 2);
        e.call(0x00c1_6350, &args![this]);
        assert_eq!(calls(&e, SLOTS + 0x160), vec![vec![this]]);

        let mut e = engine();
        let this = object(&mut e, 0x10);
        e.call(0x00c0_d1b0, &args![this]);
        let slots: Vec<u32> = order(&e).iter().map(|a| a - SLOTS).collect();
        assert_eq!(slots, vec![0x15c, 0x160, 0x164, 0x168, 0x16c, 0x170]);
    }

    #[test]
    fn pass_table_lookup() {
        let mut e = engine();
        e.mem.set_u32(0x011f_da48 + 4 * 0xcf, 0x7777);
        assert_eq!(e.call(0x00c1_63a0, &args![0xcfu32]).u32(), 0x7777);
        assert_eq!(e.call(0x00c1_63a0, &args![0xd0u32]).u32(), 0);
        assert!(calls(&e, IMPORT_INTERLOCKED_DECREMENT).is_empty());
    }

    #[test]
    fn pack_bits_sign_extends() {
        let mut e = Engine::new();
        let words = args![1u32, 0u32, 1u32, 0u32, 0u32, 0u32, 0u32, 1u32];
        assert_eq!(e.call(0x00c0_6dc0, &words).u32(), 0b1000_0101);
        let words = args![0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0xffu32];
        assert_eq!(e.call(0x00c0_6dc0, &words).u32(), 0xffff_ff80);
        let words = args![0x1ffu32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32];
        assert_eq!(e.call(0x00c0_6dc0, &words).u32(), 0xffff_ffff);
    }

    #[test]
    fn mask_stages_get_their_bits() {
        let mut e = engine();
        let pass = e.mem.alloc(0x68);
        e.mem.set_u32(pass + 0x14, 9);
        let stages: Rc<RefCell<Vec<u32>>> = Rc::default();
        let seen = stages.clone();
        e.register_double(PASS_SET_STAGE, move |_, a| {
            seen.borrow_mut().push(a[2]);
            Ret::default()
        });
        e.call(0x00c0_6e10, &args![pass, 0x45u32]);
        let made = stages.borrow().clone();
        assert_eq!(made.len(), 7);
        let bytes: Vec<u8> = made.iter().map(|s| e.mem.u8(s + 0x64)).collect();
        assert_eq!(bytes, vec![1, 0, 1, 0, 0, 0, 1]);
        let setups: Vec<Vec<u32>> = calls(&e, SETUP_DEFAULT_STAGE)
            .iter()
            .map(|c| c[1..].to_vec())
            .collect();
        assert_eq!(setups[6], vec![6, 1, 2, 1]);
        assert_eq!(calls(&e, PASS_SET_STAGE)[0][..2], [pass, 9]);
        assert_eq!(e.mem.u8(pass + 0x4c), 0x45);
        // The last stage local is given up.
        assert_eq!(calls(&e, STAGE_RELEASE_00E7ECD0), vec![vec![made[6]]]);
        // A pass with 7 stages already: none added.
        let mut e = engine();
        let pass = e.mem.alloc(0x68);
        e.mem.set_u32(pass + 0x18, 7);
        e.call(0x00c0_6e10, &args![pass, 1u32]);
        assert!(calls(&e, CREATE_NEW_STAGE).is_empty());
        assert_eq!(e.mem.u8(pass + 0x4c), 1);
    }

    fn pass(e: &mut Engine, count: u32) -> u32 {
        let pass = e.mem.alloc(0x68);
        e.mem.set_u32(pass + 0x64, count);
        pass
    }

    #[test]
    fn programs_for_every_pass() {
        let mut e = engine();
        e.register(0x00aa_13e0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x00b7_e330, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            e.mem.set_u32(a[0] + 4, 0);
            a[0].into_ret()
        });
        let shared = pass(&mut e, 1);
        for index in 0..0x70u32 {
            e.mem.set_u32(0x0120_0ec0 + 4 * index, shared);
        }
        let constant = e.mem.alloc(0x18);
        e.mem.set_u32(constant + 0x14, 0x2000_0000);
        e.mem.set_u32(0x0120_1080, constant);
        e.mem.set_u32(0x0120_10c4, constant);
        e.mem.set_u32(0x0120_0a28, 2);
        e.mem.set_u32(0x0120_0868, 2);
        let this = e.mem.alloc(0x80);
        e.mem.set_u32(this + 0x18, 0x5151);
        let last = [pass(&mut e, 1), pass(&mut e, 1)];
        e.mem.set_u32(0x0120_1058, last[0]);
        e.mem.set_u32(0x0120_105c, last[1]);
        let vertex = object(&mut e, 8);
        let pixel = object(&mut e, 8);
        e.mem.set_u32(this + 0x70, vertex);
        e.mem.set_u32(this + 0x6c, pixel);
        e.call(0x00c0_6f80, &args![this]);
        // One hold of the shared pass, released at the end: the count is back to 1.
        assert_eq!(e.mem.u32(shared + 0x64), 1);
        assert!(calls(&e, PASS_RELEASE_00E80190).is_empty());
        // 0x6e passes with two programs each.
        assert_eq!(calls(&e, 0x00b7_e330).len(), 2 * 0x6e);
        assert_eq!(calls(&e, 0x00b7_e330)[1][1..], [0x5151, 2]);
        // Only pass 0's masks have bit 1: one constant each through slot +0x84.
        assert_eq!(calls(&e, SLOTS + 0x84).len(), 2);
        assert_eq!(e.mem.u32(last[0] + 0x48), vertex);
        assert_eq!(e.mem.u32(last[1] + 0x34), pixel);
        assert_eq!(e.mem.u32(vertex + 4), 2);
    }

    #[test]
    fn two_pass_setup_counts_and_releases() {
        let mut e = engine();
        let first = pass(&mut e, 1);
        let second = pass(&mut e, 1);
        e.mem.set_u32(0x0120_2294, first);
        e.mem.set_u32(0x0120_2298, second);
        e.mem.set_u32(first + 0x18, 4);
        let this = e.mem.alloc(0x140);
        let shader = object(&mut e, 8);
        e.mem.set_u32(this + 0xbc, shader);
        e.mem.set_f32(STAGE_VALUE_011F9534, 3.5);
        e.call(0x00c1_13b0, &args![this]);
        // Only the second pass (stage count below 4) gets stages.
        let set: Vec<Vec<u32>> = calls(&e, PASS_SET_STAGE)
            .iter()
            .map(|c| c[..2].to_vec())
            .collect();
        assert_eq!(
            set,
            vec![
                vec![second, 0],
                vec![second, 1],
                vec![second, 2],
                vec![second, 3]
            ]
        );
        let setups: Vec<Vec<u32>> = calls(&e, SETUP_DEFAULT_STAGE)
            .iter()
            .map(|c| c[1..].to_vec())
            .collect();
        assert_eq!(
            setups,
            vec![
                vec![0, 1, 2, 1],
                vec![1, 1, 2, 1],
                vec![2, 1, 2, 1],
                vec![3, 3, 0, 1]
            ]
        );
        let copied = calls(&e, PASS_SET_STAGE)[3][2];
        assert_eq!((e.mem.f32(copied + 8), e.mem.u32(copied + 0xc)), (3.5, 1));
        assert_eq!(e.mem.u32(first + 0x5c), shader);
        assert_eq!(e.mem.u32(shader + 4), 1);
        assert_eq!(
            calls(&e, RENDER_STATE_SET_00E7F430)
                .iter()
                .map(|c| c[1..].to_vec())
                .collect::<Vec<_>>(),
            vec![
                vec![0x13, 2, 1],
                vec![0x14, 2, 1],
                vec![0x13, 2, 1],
                vec![0x14, 2, 1]
            ]
        );
        assert_eq!((e.mem.u32(0x011f_d150), e.mem.u32(0x011f_c570)), (0x480, 8));
        assert_eq!((e.mem.u32(0x011f_d180), e.mem.u32(0x011f_c5a0)), (0x480, 8));
        // Both passes back to one use (no release); the last stage local given up.
        assert_eq!((e.mem.u32(first + 0x64), e.mem.u32(second + 0x64)), (1, 1));
        assert_eq!(calls(&e, STAGE_RELEASE_00E7ECD0), vec![vec![copied]]);
    }

    #[test]
    fn one_pass_setup() {
        let mut e = engine();
        let held = pass(&mut e, 0);
        e.mem.set_u32(held + 0x14, 0x22);
        e.mem.set_u32(0x0120_229c, held);
        let this = e.mem.alloc(0x140);
        e.call(0x00c1_1790, &args![this]);
        let set: Vec<Vec<u32>> = calls(&e, PASS_SET_STAGE)
            .iter()
            .map(|c| c[..2].to_vec())
            .collect();
        assert_eq!(set, vec![vec![held, 0x22]; 3]);
        assert_eq!(
            calls(&e, RENDER_STATE_SET_00E7F430)
                .iter()
                .map(|c| c[1..].to_vec())
                .collect::<Vec<_>>(),
            vec![vec![0x13, 9, 1], vec![0x14, 1, 1]]
        );
        assert_eq!(
            (e.mem.u32(0x011f_d1cc), e.mem.u32(0x011f_c5ec)),
            (0x400, 0x20)
        );
        // Held from 0 to 1, then released back to 0.
        assert_eq!(calls(&e, PASS_RELEASE_00E80190), vec![vec![held]]);
    }

    #[test]
    fn stage_textures_from_the_geometry() {
        let mut e = engine();
        let this = 0x4444u32;
        let stages = e.mem.alloc(0xc);
        let stage = [e.mem.alloc(0x10), e.mem.alloc(0x10), e.mem.alloc(0x10)];
        for (i, s) in stage.iter().enumerate() {
            e.mem.set_u32(stages + 4 * i as u32, *s);
        }
        let owner = e.mem.alloc(0x28);
        e.mem.set_u32(owner + 0x24, stages);
        e.mem.set_u32(STAGE_OWNER_0126F74C, owner);
        let geometry = e.mem.alloc(0xb8);
        let texture = e.mem.alloc(4);
        e.mem.set_u32(texture, 0x7e7e);
        let empty = e.mem.alloc(4);
        e.mem.set_u32(geometry + 0xac, texture);
        e.mem.set_u32(geometry + 0xb0, empty);
        e.mem.set_u32(0x011f_9510, 0x9510);
        // A number outside every range: both stages, nothing else.
        e.call(0x00c0_cd80, &args![this, 0x100u32, geometry]);
        assert_eq!(e.mem.u32(stage[0] + 8), 0x7e7e);
        assert_eq!(e.mem.u32(stage[1] + 8), 0x9510);
        assert_eq!(calls(&e, STAGE_SETUP_00BB9640).len(), 2);
        // 0x1bd: skipped; 0x1c5: runs; 0x2e0: skipped, then the first stage alone.
        e.call(0x00c0_cd80, &args![this, 0x1bdu32, geometry]);
        assert_eq!(calls(&e, STAGE_SETUP_00BB9640).len(), 2);
        e.call(0x00c0_cd80, &args![this, 0x1c5u32, geometry]);
        assert_eq!(calls(&e, STAGE_SETUP_00BB9640).len(), 4);
        e.call(0x00c0_cd80, &args![this, 0x2e0u32, geometry]);
        assert_eq!(calls(&e, STAGE_SETUP_00BB9640).len(), 5);
        e.call(0x00c0_cd80, &args![this, 0x1d4u32, geometry]);
        // 0x1d4 is in 0x1bd..=0x1e1 outside 0x1c4..=0x1cb: only the late first stage.
        assert_eq!(calls(&e, STAGE_SETUP_00BB9640).len(), 6);
        // The third stage's rendered texture.
        e.mem.set_u32(0x011f_91e4, 0x1c6);
        e.register(GEOMETRY_COUNT_00B704B0, |_, _| 1u32.into_ret());
        e.register(RENDERED_TEXTURE_GET_TEXTURE, |_, _| 0x7171u32.into_ret());
        e.register(GEOMETRY_FLAG_004A2020, |_, _| 1u32.into_ret());
        let object = e.mem.alloc(0x110);
        e.mem.set_u8(object + 0xec, 1);
        e.mem.set_u32(object + 0x10c, 0x1010);
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, object);
        let holder = e.mem.alloc(0x10);
        e.mem.set_u32(holder + 0xc, list);
        e.mem.set_u8(holder + 9, 1);
        e.mem.set_u32(0x011f_91e0, holder);
        e.mem.set_u32(0x011f_91b0, 3);
        e.call(0x00c0_cd80, &args![this, 0x100u32, geometry]);
        assert_eq!(e.mem.u32(stage[2] + 8), 0x7171);
        assert_eq!(
            calls(&e, RENDERED_TEXTURE_GET_TEXTURE),
            vec![vec![0x1010, 0]]
        );
        assert_eq!(calls(&e, STAGE_SET_FILTER_MODE), vec![vec![stage[2], 1]]);
        assert_eq!(calls(&e, STAGE_STEP_00E7DE90), vec![vec![stage[2], 0]]);
        assert_eq!(e.mem.f32(0x0120_1c7c), 1.0);
    }

    #[test]
    fn stage_values_and_bounds() {
        let mut e = engine();
        let shader = e.mem.alloc(0x28);
        let stages = e.mem.alloc(0x18);
        let (a, b) = (e.mem.alloc(0x10), e.mem.alloc(0x10));
        e.mem.set_u32(stages + 0x10, a);
        e.mem.set_u32(stages + 0x14, b);
        e.mem.set_u32(shader + 0x24, stages);
        e.mem.set_u32(0x011f_9530, 0x9530);
        let geometry = e.mem.alloc(0xb8);
        let source = e.mem.alloc(4);
        e.mem.set_u32(source, 0xb4b4);
        e.mem.set_u32(geometry + 0xb4, source);
        e.call(0x00c0_bb20, &args![0u32, shader, geometry]);
        assert_eq!((e.mem.u32(a + 8), e.mem.u32(b + 8)), (0x9530, 0xb4b4));

        // The bound: made when missing, then set.
        let this = object(&mut e, 0xd8);
        e.register(SLOTS + 0x78, |e, a| {
            e.mem.set_u32(a[0] + 0x20, a[1]);
            Ret::default()
        });
        let radius = f32::from_bits(0x7f80_0003);
        e.call(0x00c1_a3b0, &args![this, 1u32, 2u32, 3u32, radius]);
        let bound = e.mem.u32(this + 0x20);
        assert_eq!(
            [
                e.mem.u32(bound),
                e.mem.u32(bound + 4),
                e.mem.u32(bound + 8),
                e.mem.u32(bound + 0xc)
            ],
            [1, 2, 3, 0x7fc0_0003]
        );
        assert_eq!(e.mem.u32(this + 0xd4), 0x7fc0_0003);
        assert_eq!(e.mem.u32(this + 0xcc), 2);
        e.call(0x00c1_a3b0, &args![this, 4u32, 5u32, 6u32, 1.0f32]);
        assert_eq!(e.mem.u32(this + 0x20), bound);
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 1);

        // The global four words.
        e.mem.set_u32(0x011a_9bd4, 0xbd4);
        e.mem.set_u32(0x011a_9bdc, 0xbdc);
        e.mem.set_f32(0x0101_8f5c, 0.25);
        e.call(0x00c1_a310, &args![0u32, 0u32, 0u32, 0u32]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(0x011f_a2d0 + 4 * i)).collect();
        assert_eq!(
            words,
            vec![1.0f32.to_bits(), 0xbd4, 0.25f32.to_bits(), 0xbdc]
        );
        let item = e.mem.alloc(0x40);
        e.mem.set_f32(item + 0x38, 9.0);
        e.call(0x00c1_a310, &args![0u32, 0u32, 1u32, item]);
        let words: Vec<u32> = (0..4).map(|i| e.mem.u32(0x011f_a2d0 + 4 * i)).collect();
        assert_eq!(
            words,
            vec![1.0f32.to_bits(), 1.0f32.to_bits(), 9.0f32.to_bits(), 0xbdc]
        );

        // The strips constructor.
        let strips = e.mem.alloc(0xd0);
        assert_eq!(e.call(0x00c1_a480, &args![strips, 0x88u32]).u32(), strips);
        assert_eq!(calls(&e, 0x00a7_1cc0), vec![vec![strips, 0x88]]);
        assert_eq!(e.mem.u32(strips), 0x010b_cc90);
    }
}

#[cfg(test)]
mod batch_b011_pass_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    const VTABLE: u32 = 0x0e90_0000;
    const DELETE_DOUBLE: u32 = 0x0e91_0004;

    /// An engine whose stage, pass and render-state callees are doubles that log, with every
    /// pass global pointing at its own pass (stage count `stages`, use count `uses`).
    fn engine(stages: u32, uses: u32) -> (Engine, Log, Vec<u32>) {
        let mut e = Engine::new();
        e.map(0x011f_0000, 0x14000);
        e.put_vtable(VTABLE, &[0, DELETE_DOUBLE]);
        e.register(DELETE_DOUBLE, |_, _| Ret::default());
        let log: Log = Rc::default();
        for double in [
            SETUP_DEFAULT_STAGE,
            PASS_SET_STAGE,
            RENDER_STATE_SET_00E7F430,
            STAGE_RELEASE_00E7ECD0,
            PASS_RELEASE_00E80190,
            PASS_SET_00B79950,
            PASS_SET_00B80600,
            PASS_STEP_00B80040,
            POINTER_ASSIGN_00A5F860,
        ] {
            let seen = log.clone();
            e.register_double(double, move |_, a| {
                seen.borrow_mut().push((double, a.to_vec()));
                Ret::default()
            });
        }
        e.register(RENDER_STATES_NEW_00E7F5D0, |e, _| {
            e.mem.alloc(0x10).into_ret()
        });
        e.register(CREATE_NEW_STAGE, |e, a| {
            let stage = e.mem.alloc(0x68);
            e.mem.set_u32(stage + 0x60, 1);
            e.mem.set_u32(a[0], stage);
            Ret::default()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        let mut passes = vec![];
        for global in (0x0120_2230u32..0x0120_23c8).step_by(4) {
            let pass = e.mem.alloc(0x68);
            e.mem.set_u32(pass + 0x18, stages);
            e.mem.set_u32(pass + 0x64, uses);
            e.mem.set_u32(global, pass);
            passes.push(pass);
        }
        (e, log, passes)
    }

    fn count(log: &Log, addr: u32) -> usize {
        log.borrow().iter().filter(|c| c.0 == addr).count()
    }

    /// The shader: every word a counted program object.
    fn shader(e: &mut Engine) -> u32 {
        let shader = e.mem.alloc(0x200);
        for offset in (4..0x200u32).step_by(4) {
            let program = e.mem.alloc(8);
            e.mem.set_u32(program, VTABLE);
            e.mem.set_u32(program + 4, 1);
            e.mem.set_u32(shader + offset, program);
        }
        shader
    }

    #[test]
    fn pass_setups_add_stages_and_balance_their_holds() {
        // (function, stage blocks, render states, programs, passes, 00b80040, 00b79950,
        // 00b80600) as the exe has them.
        let cases: [(u32, usize, usize, usize, usize, usize, usize, usize); 7] = [
            (0x00c0_d200, 56, 36, 16, 8, 0, 0, 0),
            (0x00c0_e630, 72, 35, 18, 9, 0, 0, 0),
            (0x00c0_fdc0, 64, 40, 16, 8, 0, 0, 0),
            (0x00c1_1980, 30, 10, 10, 5, 0, 0, 0),
            (0x00c1_24c0, 188, 136, 20, 26, 4, 16, 16),
            (0x00c1_6400, 21, 37, 20, 10, 0, 0, 0),
            (0x00c1_7510, 138, 65, 23, 20, 8, 8, 9),
        ];
        for (address, stages, states, programs, passes, blend, vertex, pixel) in cases {
            // Fresh passes: every stage block runs; each pass is held from 0 and released.
            let (mut e, log, _) = engine(0, 0);
            let this = shader(&mut e);
            e.call(address, &args![this]);
            assert_eq!(count(&log, SETUP_DEFAULT_STAGE), stages, "{address:08x}");
            assert_eq!(count(&log, PASS_SET_STAGE), stages, "{address:08x}");
            assert_eq!(
                count(&log, RENDER_STATE_SET_00E7F430),
                states,
                "{address:08x}"
            );
            assert_eq!(count(&log, PASS_RELEASE_00E80190), passes, "{address:08x}");
            assert_eq!(count(&log, STAGE_RELEASE_00E7ECD0), 1, "{address:08x}");
            assert_eq!(count(&log, PASS_STEP_00B80040), blend, "{address:08x}");
            assert_eq!(count(&log, PASS_SET_00B79950), vertex, "{address:08x}");
            assert_eq!(count(&log, PASS_SET_00B80600), pixel, "{address:08x}");
            let counted = (0..0x200u32)
                .step_by(4)
                .skip(1)
                .map(|offset| e.mem.u32(e.mem.u32(this + offset) + 4))
                .filter(|uses| *uses > 1)
                .count();
            assert!(counted > 0 && counted <= programs, "{address:08x}");
            // Full passes: no stage at all; held passes keep their one use.
            let (mut e, log, all) = engine(0xff, 1);
            let this = shader(&mut e);
            e.call(address, &args![this]);
            assert_eq!(count(&log, SETUP_DEFAULT_STAGE), 0, "{address:08x}");
            assert_eq!(count(&log, PASS_RELEASE_00E80190), 0, "{address:08x}");
            assert!(all.iter().all(|pass| e.mem.u32(pass + 0x64) == 1));
            assert_eq!(
                count(&log, RENDER_STATE_SET_00E7F430),
                states,
                "{address:08x}"
            );
        }
    }

    #[test]
    fn five_pass_setup_in_detail() {
        let (mut e, log, passes) = engine(0, 0);
        let this = shader(&mut e);
        e.mem.set_f32(STAGE_VALUE_011F9534, 6.5);
        e.call(0x00c1_1980, &args![this]);
        let calls = log.borrow();
        // The first pass: six stages added at its +0x14 (0 here), the fourth one with +8 and
        // +0xc set.
        let first: Vec<&(u32, Vec<u32>)> = calls
            .iter()
            .filter(|c| c.0 == SETUP_DEFAULT_STAGE)
            .take(6)
            .collect();
        let words: Vec<Vec<u32>> = first.iter().map(|c| c.1[1..].to_vec()).collect();
        assert_eq!(
            words,
            vec![
                vec![0, 1, 2, 1],
                vec![1, 1, 2, 1],
                vec![2, 1, 2, 1],
                vec![3, 3, 0, 1],
                vec![4, 1, 2, 1],
                vec![5, 1, 2, 1]
            ]
        );
        let fourth = first[3].1[0];
        assert_eq!((e.mem.f32(fourth + 8), e.mem.u32(fourth + 0xc)), (6.5, 1));
        let set: Vec<&(u32, Vec<u32>)> = calls.iter().filter(|c| c.0 == PASS_SET_STAGE).collect();
        assert_eq!(set[0].1[..2], [passes[0x1c], 0]);
        assert_eq!(e.mem.u32(passes[0x1c] + 0x5c), e.mem.u32(this + 0xc8));
        assert_eq!(e.mem.u32(passes[0x1c] + 0x44), e.mem.u32(this + 0x13c));
        let states: Vec<Vec<u32>> = calls
            .iter()
            .filter(|c| c.0 == RENDER_STATE_SET_00E7F430)
            .take(2)
            .map(|c| c.1[1..].to_vec())
            .collect();
        assert_eq!(states, vec![vec![0x13, 2, 1], vec![0x14, 2, 1]]);
        assert_eq!(e.mem.u32(0x011f_d1f0), 0x480);
        assert_eq!(e.mem.u32(0x011f_c610), 0x28);
        assert_eq!(e.mem.u32(0x011f_d208), 0x4f0);
    }

    #[test]
    fn pass_switch_keeps_the_same_pass() {
        let (mut e, log, passes) = engine(0, 0);
        e.mem.set_u32(passes[0] + 0x64, 1);
        assert_eq!(switch_pass(&mut e, passes[0], 0x0120_2230), passes[0]);
        assert_eq!(e.mem.u32(passes[0] + 0x64), 1);
        assert_eq!(switch_pass(&mut e, passes[0], 0x0120_2234), passes[1]);
        assert_eq!(e.mem.u32(passes[0] + 0x64), 0);
        assert_eq!(e.mem.u32(passes[1] + 0x64), 1);
        assert_eq!(count(&log, PASS_RELEASE_00E80190), 1);
    }

    #[test]
    fn pass_globals_by_number() {
        let (mut e, log, _) = engine(0, 0);
        let this = e.mem.alloc(0x100);
        for (offset, value) in [(0x7cu32, 0x77u32), (0x84, 0x88), (0x8c, 0x99)] {
            e.mem.set_u32(this + offset, value);
        }
        e.mem.set_u32(0x0120_0ec4, 0x4c4);
        e.mem.set_u32(0x0120_0ec0, 0x4c0);
        e.mem.set_u32(0x0120_1000, 0x1000);
        assert_eq!(e.call(0x00c0_abd0, &args![this, 0xau32]).u32(), 0x4c4);
        assert_eq!(e.call(0x00c0_abd0, &args![this, 0x178u32]).u32(), 0x4c0);
        // 0x1c8 reads its global after the assignment.
        e.register(POINTER_ASSIGN_00A5F860, |e, _| {
            e.mem.set_u32(0x0120_1000, 0x2000);
            Ret::default()
        });
        assert_eq!(e.call(0x00c0_abd0, &args![this, 0x1c8u32]).u32(), 0x2000);
        for pass in [0u32, 9, 0x10, 0x177, 0x179, 0x17e, 0x2ee, 0xffff_ffff] {
            assert_eq!(
                e.call(0x00c0_abd0, &args![this, pass]).u32(),
                0,
                "{pass:#x}"
            );
        }
        let assigned: Vec<Vec<u32>> = log
            .borrow()
            .iter()
            .filter(|c| c.0 == POINTER_ASSIGN_00A5F860)
            .map(|c| c.1.clone())
            .collect();
        assert_eq!(
            assigned,
            vec![vec![this + 0x28, 0x77], vec![this + 0x28, 0x77]]
        );
    }
}

#[cfg(test)]
mod batch_b011_image_space_pass_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<u32>>>;

    const VTABLE: u32 = 0x0ea0_0000;
    const SLOT_DOUBLE: u32 = 0x0ea1_0000;

    #[test]
    fn image_space_passes_balance_their_holds() {
        let mut e = Engine::new();
        e.map(0x011f_0000, 0x14000);
        let slots: Vec<u32> = (0..0x30u32).map(|i| SLOT_DOUBLE + 4 * i).collect();
        e.put_vtable(VTABLE, &slots);
        for slot in &slots {
            e.register(*slot, |_, _| Ret::default());
        }
        let log: Log = Rc::default();
        for double in [
            SETUP_DEFAULT_STAGE,
            PASS_SET_STAGE,
            RENDER_STATE_SET_00E7F430,
            STAGE_RELEASE_00E7ECD0,
            PASS_RELEASE_00E80190,
            PASS_SET_00B79950,
            PASS_SET_00B80600,
            PASS_STEP_00B80040,
            STAGE_STEP_00E7DE90,
            STAGE_SET_FILTER_MODE,
        ] {
            let seen = log.clone();
            e.register_double(double, move |_, _| {
                seen.borrow_mut().push(double);
                Ret::default()
            });
        }
        // The in-place switch of the held local: the old pass released, the new one held.
        let seen = log.clone();
        e.register_double(POINTER_SWITCH_00BA53C0, move |e, a| {
            seen.borrow_mut().push(POINTER_SWITCH_00BA53C0);
            let old = e.mem.u32(a[0]);
            let new = e.mem.u32(a[1]);
            if old != new {
                if old != 0 {
                    release_pass(e, old);
                }
                e.mem.set_u32(a[0], new);
                hold_pass(e, new);
            }
            Ret::default()
        });
        e.register(RENDER_STATES_NEW_00E7F5D0, |e, _| {
            e.mem.alloc(0x10).into_ret()
        });
        e.register(CREATE_NEW_STAGE, |e, a| {
            let stage = e.mem.alloc(0x68);
            e.mem.set_u32(stage + 0x60, 1);
            e.mem.set_u32(a[0], stage);
            Ret::default()
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_add(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            count.into_ret()
        });
        e.register(0x00aa_13e0, |e, a| e.mem.alloc(a[0]).into_ret());
        e.register(0x00b7_e330, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            a[0].into_ret()
        });
        // Every pass global from 01200e9c up holds its own pass (stage count 7: no stages in
        // the mask helper), program globals hold counted objects.
        let mut passes = vec![];
        for global in (0x0120_0c00u32..0x0120_1100).step_by(4) {
            let object = e.mem.alloc(0x68);
            e.mem.set_u32(object, VTABLE);
            e.mem.set_u32(object + 0x18, 7);
            let stages = e.mem.alloc(0x20);
            e.mem.set_u32(object + 0x24, stages);
            e.mem.set_u32(global, object);
            passes.push(object);
        }
        e.mem.set_f32(0x0120_0458, 1.5);
        let this = e.mem.alloc(0x80);
        e.call(0x00c0_72c0, &args![this]);
        let calls = log.borrow();
        let count = |addr: u32| calls.iter().filter(|a| **a == addr).count();
        assert_eq!(count(POINTER_SWITCH_00BA53C0), 38);
        assert_eq!(count(PASS_SET_00B79950), 95);
        assert_eq!(count(PASS_SET_00B80600), 96);
        assert_eq!(count(PASS_STEP_00B80040), 14);
        assert_eq!(count(STAGE_STEP_00E7DE90), 11);
        // 96 render states here plus those of `00c06e10` (none: every pass is full).
        assert_eq!(count(RENDER_STATE_SET_00E7F430), 96);
        assert_eq!(count(SETUP_DEFAULT_STAGE), 0);
        // Every pass held during the run is back to its count (the pass objects started at 0
        // uses: a hold and a release each, released at zero).
        assert!(passes.iter().all(|pass| e.mem.u32(pass + 0x64) == 0));
        // The doubled `float` (1.5 + 1.5) is the value of state 0xc3.
        assert_eq!(e.mem.u32(0x0120_0a28), 0x8a);
    }
}

#[cfg(test)]
mod n080_tests {
    use super::*;

    fn answer(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An effect whose parameter list holds `param` first.
    fn effect(e: &mut Engine, param: u32) -> u32 {
        let this = e.mem.alloc(0xd0);
        let data = e.mem.alloc(0x10);
        e.mem.set_u32(data, param);
        e.mem.set_u32(this + 0x1c, data);
        this
    }

    #[test]
    fn return_textures_clears_texture_two() {
        let mut e = Engine::new();
        e.register(EFFECT_RETURN_00B4F510, |_, _| Ret::default());
        e.register(ARRAY_SET_AT_00BA3AA0, |e, a| answer(e.mem.u32(a[2]) + 9));
        let this = effect(&mut e, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c0_4960, &args![this]).u32(), 9);
        assert_eq!(
            calls_to(&e, EFFECT_RETURN_00B4F510),
            vec![Vec::<u32>::new()]
        );
        assert_eq!(
            calls_to(&e, ARRAY_SET_AT_00BA3AA0)[0][..2],
            [this + 0x28, 2]
        );
    }

    fn update_setup(e: &mut Engine, count: u32) -> u32 {
        for page in [0x0120_2000u32, 0x011f_9000, 0x0120_0000, 0x0109_9000] {
            e.map(page, 0x1000);
        }
        e.set_global(TWO_TO_32_01099378, 4_294_967_296.0f32);
        let object = e.mem.alloc(0x10);
        let object_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(object, object_vtable);
        e.mem.set_u32(object_vtable + 0x90, 0x0019_0090);
        e.register_double(0x0019_0090, move |_, a| {
            assert_eq!(a[1], 0);
            answer(count)
        });
        let renderer = e.mem.alloc(0x10);
        let renderer_vtable = e.mem.alloc(0x100);
        e.mem.set_u32(renderer, renderer_vtable);
        e.mem.set_u32(renderer_vtable + 0xc8, 0x0019_00c8);
        e.register_double(0x0019_00c8, move |_, _| answer(object));
        e.set_global(RENDERER_011F9508, renderer);
        let camera = e.mem.alloc(0x90);
        for (index, offset) in [0x6cu32, 0x78, 0x84, 0x70, 0x7c, 0x88].iter().enumerate() {
            e.mem.set_f32(camera + offset, (index + 1) as f32);
        }
        e.set_global(0x011f_917cu32, camera);
        e.set_global(0x0120_04f0u32, 1.0f32);
        e.set_global(0x0120_04f4u32, 2.0f32);
        e.set_global(0x0120_04f8u32, 3.0f32);
        e.register(SHADER_PARAM_SET_PIXEL_CONSTANT, |_, _| Ret::default());
        effect(e, 0x4400)
    }

    fn floats(words: &[u32]) -> Vec<f32> {
        words[2..].iter().map(|b| f32::from_bits(*b)).collect()
    }

    #[test]
    fn update_params_sets_two_constants() {
        let mut e = Engine::new();
        let this = update_setup(&mut e, 4);
        e.mem.set_f32(this + 0x58, 0.5);
        e.mem.set_f32(this + 0x5c, 2.0);
        e.mem.set_f32(this + 0x60, 7.0);
        e.set_global(0x011f_9418u32, 1.0f32);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c0_4b90, &args![this, 0u32]).bool());
        let calls = calls_to(&e, SHADER_PARAM_SET_PIXEL_CONSTANT);
        assert_eq!(calls[0][..2], [0x4400, 0]);
        assert_eq!(floats(&calls[0]), vec![0.5, 0.5, 7.0, 1.0]);
        // (1, 2, 3) dot (4, 5, 6), then minus (1, 2, 3) dot (1, 2, 3).
        assert_eq!(calls[1][..2], [0x4400, 1]);
        assert_eq!(floats(&calls[1]), vec![32.0, -14.0, 0.0, 0.0]);
        // A limit not above 0.0 gives 0.0; a negative count is taken unsigned.
        let mut e = Engine::new();
        let this = update_setup(&mut e, 0x8000_0000);
        e.mem.set_f32(this + 0x58, 0.5);
        e.mem.set_f32(this + 0x5c, 2_147_483_648.0);
        e.set_global(0x011f_9418u32, -1.0f32);
        e.call_log = Some(vec![]);
        e.call(0x00c0_4b90, &args![this, 0u32]);
        let calls = calls_to(&e, SHADER_PARAM_SET_PIXEL_CONSTANT);
        assert_eq!(f32::from_bits(calls[0][2]), 0.0);
        assert_eq!(f32::from_bits(calls[0][3]), 1.0);
        // The byte at 01202498 takes +0x58 without the test.
        e.set_global(0x0120_2498u32, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x00c0_4b90, &args![this, 0u32]);
        let calls = calls_to(&e, SHADER_PARAM_SET_PIXEL_CONSTANT);
        assert_eq!(f32::from_bits(calls[0][2]), 0.5);
    }

    #[test]
    fn flag_setter_marks_a_change() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x60);
        e.call(0x00c0_4d80, &args![this, 0u32]);
        assert_eq!(e.mem.u8(this + 5), 0);
        e.call(0x00c0_4d80, &args![this, 0x0101u32]);
        assert_eq!(e.mem.u8(this + 0x58), 1);
        assert_eq!(e.mem.u8(this + 5), 1);
    }

    type Captured = std::rc::Rc<std::cell::RefCell<Vec<(u32, Vec<f32>)>>>;

    #[test]
    fn update_params_fills_four_constants_from_its_data() {
        let mut e = Engine::new();
        e.register(SHADER_PARAM_SET_PIXEL_CONSTANT, |_, _| Ret::default());
        let captured: Captured = Default::default();
        let sink = captured.clone();
        e.register_double(SHADER_PARAM_SET_PIXEL_CONSTANT_ARRAY, move |e, a| {
            let values = (0..4).map(|i| e.mem.f32(a[2] + 4 * i)).collect();
            sink.borrow_mut().push((a[1], values));
            Ret::default()
        });
        let this = effect(&mut e, 0x4400);
        let data = e.mem.alloc(0x30);
        for index in 0..12 {
            e.mem.set_f32(data + 4 * index, index as f32);
        }
        e.mem.set_u32(this + 0x60, data);
        for (index, offset) in [0x68u32, 0x6c, 0x70, 0x74].iter().enumerate() {
            e.mem.set_f32(this + offset, 20.0 + index as f32);
        }
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c0_4e30, &args![this, 0u32]).bool());
        let first = &calls_to(&e, SHADER_PARAM_SET_PIXEL_CONSTANT)[0];
        assert_eq!(first[2..], [0.0f32, 1.0, 2.0, 3.0].map(f32::to_bits));
        assert_eq!(
            captured.borrow().clone(),
            vec![
                (1, vec![4.0, 5.0, 6.0, 7.0]),
                (2, vec![8.0, 9.0, 10.0, 11.0]),
                (3, vec![20.0, 21.0, 22.0, 23.0]),
            ]
        );
        let other = e.mem.alloc(0x10);
        for index in 0..4 {
            e.mem.set_f32(other + 4 * index, 30.0 + index as f32);
        }
        e.mem.set_u32(this + 0x5c, other);
        e.mem.set_u8(this + 0x58, 1);
        captured.borrow_mut().clear();
        e.call_log = Some(vec![]);
        e.call(0x00c0_4e30, &args![this, 0u32]);
        let first = &calls_to(&e, SHADER_PARAM_SET_PIXEL_CONSTANT)[0];
        assert_eq!(f32::from_bits(first[5]), 33.0);
        assert_eq!(captured.borrow()[0], (1, vec![30.0, 31.0, 32.0, 1.0]));
        assert_eq!(captured.borrow()[2], (3, vec![20.0, 21.0, 1.0, 1.0]));
    }

    #[test]
    fn borrow_textures_stores_both_textures() {
        let mut e = Engine::new();
        e.register(BORROW_RENDERED_TEXTURE, |_, _| Ret::default());
        e.register(ARRAY_SET_AT_00BA3AA0, |e, a| answer(e.mem.u32(a[2])));
        let this = effect(&mut e, 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c0_5370, &args![this, 0u32]).u32(), this + 0x6c);
        assert_eq!(
            calls_to(&e, BORROW_RENDERED_TEXTURE),
            vec![vec![this + 0x5c, 0x1a], vec![this + 0x6c, 0x1a]]
        );
        let stores: Vec<_> = calls_to(&e, ARRAY_SET_AT_00BA3AA0)
            .iter()
            .map(|w| w[1])
            .collect();
        assert_eq!(stores, vec![2, 3]);
    }

    #[test]
    fn small_update_params_set_constant_zero() {
        let mut e = Engine::new();
        for page in [0x011a_e000u32, 0x0101_e000, 0x0120_0000] {
            e.map(page, 0x1000);
        }
        e.set_global(0x011a_e6c8u32, 5.0f32);
        e.set_global(0x011a_e6ccu32, 6.0f32);
        e.set_global(0x0101_e570u32, 0x7f80_0001u32);
        e.register(SHADER_PARAM_SET_PIXEL_CONSTANT, |_, _| Ret::default());
        let this = effect(&mut e, 0x4400);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00c0_53c0, &args![this, 0u32]).bool());
        let call = &calls_to(&e, SHADER_PARAM_SET_PIXEL_CONSTANT)[0];
        assert_eq!(call[2..], [0.0f32, 5.0, 6.0, 0.0].map(f32::to_bits));
        assert_eq!(e.global::<u32>(0x0120_03c4), 0x7fc0_0001);
        let object = e.mem.alloc(0x10);
        e.mem.set_f32(object + 4, 1.5);
        e.mem.set_f32(object + 8, 2.5);
        e.mem.set_u32(this + 0x58, object);
        e.call_log = Some(vec![]);
        e.call(0x00c0_53c0, &args![this, 0u32]);
        let call = &calls_to(&e, SHADER_PARAM_SET_PIXEL_CONSTANT)[0];
        assert_eq!(call[2..], [0.0f32, 1.5, 2.5, 0.0].map(f32::to_bits));
        let this = effect(&mut e, 0x4500);
        e.mem.set_f32(this + 0x58, 1.0);
        e.mem.set_f32(this + 0x5c, 2.0);
        e.mem.set_f32(this + 0x60, 3.0);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x00c0_5730, &args![this, 0u32]).bool());
        let call = &calls_to(&e, SHADER_PARAM_SET_PIXEL_CONSTANT)[0];
        assert_eq!(call[..2], [0x4500, 0]);
        assert_eq!(call[2..], [1.0f32, 2.0, 3.0, 1.0].map(f32::to_bits));
        assert_eq!(e.call(0x00c0_5870, &args![this]).f32(), 1.0);
    }

    #[test]
    fn property_constructor_and_destructor() {
        let mut e = Engine::new();
        e.register(PROPERTY_BASE_CONSTRUCT_00BB4520, |_, _| Ret::default());
        e.register(PROPERTY_BASE_DESTRUCT_00BB45B0, |_, _| Ret::default());
        e.register(NI_FREE_SIZED, |_, _| Ret::default());
        let this = e.mem.alloc(PROPERTY_SIZE_00C05880);
        e.mem.set_u32(this + 0x38, 5);
        assert_eq!(e.call(0x00c0_5880, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), PROPERTY_VTABLE_010BCB60);
        assert_eq!(e.mem.u32(this + 0x38), 0);
        assert_eq!(e.mem.u32(this + 0x24), 0x0080_0000);
        assert_eq!([0x58u32, 0x1c].map(|o| e.mem.u32(this + o)), [0x1d, 0xc]);
        e.mem.set_u32(this + 0x38, 5);
        e.call(0x00c0_5880, &args![this]);
        assert_eq!(e.mem.u32(this + 0x38), 5);
        e.call_log = Some(vec![]);
        e.call(0x00c0_58c0, &args![this, 0u32]);
        assert!(calls_to(&e, NI_FREE_SIZED).is_empty());
        e.call(0x00c0_58c0, &args![this, 1u32]);
        assert_eq!(
            calls_to(&e, NI_FREE_SIZED),
            vec![vec![this, PROPERTY_SIZE_00C05880]]
        );
    }

    #[test]
    fn counted_pointer_setters_release_and_hold() {
        let mut e = Engine::new();
        e.register(IMPORT_INTERLOCKED_DECREMENT, |e, a| {
            let count = e.mem.u32(a[0]).wrapping_sub(1);
            e.mem.set_u32(a[0], count);
            answer(count)
        });
        e.register(IMPORT_INTERLOCKED_INCREMENT, |e, a| {
            let count = e.mem.u32(a[0]) + 1;
            e.mem.set_u32(a[0], count);
            answer(count)
        });
        e.register(0x001a_0004, |_, _| Ret::default());
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable + 4, 0x001a_0004);
        let old = e.mem.alloc(8);
        e.mem.set_u32(old, vtable);
        let new = e.mem.alloc(8);
        let this = e.mem.alloc(0x50);
        for (address, offset) in [(0x00c0_6ee0u32, 0x34u32), (0x00c0_6f30, 0x48)] {
            e.mem.set_u32(old + 4, 1);
            e.mem.set_u32(this + offset, old);
            e.call_log = Some(vec![]);
            e.call(address, &args![this, new]);
            assert_eq!(e.mem.u32(this + offset), new);
            assert_eq!(calls_to(&e, 0x001a_0004), vec![vec![old]]);
            e.call_log = Some(vec![]);
            e.call(address, &args![this, new]);
            assert!(calls_to(&e, IMPORT_INTERLOCKED_INCREMENT).is_empty());
        }
        assert_eq!(e.mem.u32(new + 4), 2);
    }

    /// A stage owner with seven stages and a geometry whose slot +0xf4 answers
    /// `0x500 + first word` (or 0 with `none`).
    fn stages(e: &mut Engine, none: bool) -> (u32, Vec<u32>, u32) {
        e.map(0x011f_9000, 0x1000);
        let owner = e.mem.alloc(0x28);
        let array = e.mem.alloc(0x1c);
        let list: Vec<u32> = (0..7).map(|_| e.mem.alloc(0x10)).collect();
        for (index, stage) in list.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *stage);
        }
        e.mem.set_u32(owner + 0x24, array);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0xf4, 0x001b_00f4);
        if none {
            e.register(0x001b_00f4, |_, _| Ret::default());
        } else {
            e.register(0x001b_00f4, |_, a| answer(0x500 + a[1]));
        }
        let geometry = e.mem.alloc(0x100);
        e.mem.set_u32(geometry, vtable);
        e.register(STAGE_SETUP_00BB9640, |_, a| answer(a[1]));
        (owner, list, geometry)
    }

    #[test]
    fn stage_helpers_store_textures() {
        let mut e = Engine::new();
        let (owner, list, geometry) = stages(&mut e, false);
        assert_eq!(
            e.call(0x00c0_bae0, &args![0x9000u32, owner, geometry, 2u32])
                .u32(),
            list[2]
        );
        assert_eq!(e.mem.u32(list[2] + 8), 0x503);
        let first = e.mem.alloc(8);
        let second = e.mem.alloc(8);
        e.mem.set_u32(first + 4, 0x601);
        e.mem.set_u32(second + 4, 0x602);
        e.mem.set_u32(geometry + 0xac, first);
        e.mem.set_u32(geometry + 0xb0, second);
        e.call(0x00c0_bb50, &args![0u32, owner, geometry, 0u32]);
        assert_eq!([4usize, 5].map(|i| e.mem.u32(list[i] + 8)), [0x601, 0x602]);
        assert_eq!(e.mem.u32(list[6] + 8), 0);
        e.call(0x00c0_bb50, &args![0u32, owner, geometry, 1u32]);
        assert_eq!(e.mem.u32(list[6] + 8), 0x502);
        e.set_global(0x011f_9510u32, 0x710u32);
        e.call(0x00c0_bba0, &args![0u32, owner, geometry]);
        assert_eq!(e.mem.u32(list[0] + 8), 0x710);
        e.mem.set_u32(second, 0x720);
        e.call(0x00c0_bba0, &args![0u32, owner, geometry]);
        assert_eq!(e.mem.u32(list[0] + 8), 0x720);
        e.set_global(0x011f_95e4u32, 0x7e4u32);
        e.set_global(0x011f_95e8u32, 0x7e8u32);
        e.call(0x00c0_bbd0, &args![0u32, owner]);
        assert_eq!([2usize, 3].map(|i| e.mem.u32(list[i] + 8)), [0x7e4, 0x7e8]);
        e.mem.set_u32(second, 0);
        e.set_global(0x011f_9518u32, 0x718u32);
        e.mem.set_u32(geometry + 0x20, 0x80);
        assert_eq!(
            e.call(0x00c0_bcf0, &args![0x9000u32, owner, geometry, 1u32])
                .u32(),
            list[1]
        );
        assert_eq!(e.mem.u32(list[1] + 8), 0x718);
        e.mem.set_u32(geometry + 0x20, 0);
        e.call(0x00c0_bcf0, &args![0x9000u32, owner, geometry, 1u32]);
        assert_eq!(e.mem.u32(list[1] + 8), 0x710);
    }

    #[test]
    fn stage_one_falls_back_by_pass_and_flags() {
        let mut e = Engine::new();
        let accumulator = e.mem.alloc(0x200);
        e.register_double(GET_ACCUMULATOR, move |_, _| answer(accumulator));
        e.register(GEOMETRY_FLAG_004A2020, |_, _| answer(1));
        let (owner, _, geometry) = stages(&mut e, false);
        assert_eq!(
            e.call(0x00c0_bc00, &args![0u32, owner, geometry, 0x1c8u32])
                .u32(),
            0x504
        );
        let (owner, list, geometry) = stages(&mut e, true);
        e.set_global(0x011f_95d4u32, 0x7d4u32);
        e.set_global(0x011f_9544u32, 0x744u32);
        e.mem.set_u32(geometry + 0x20, 0x0002_0000);
        e.call(0x00c0_bc00, &args![0u32, owner, geometry, 0x1c8u32]);
        assert_eq!(e.mem.u32(list[1] + 8), 0x744);
        e.call(0x00c0_bc00, &args![0u32, owner, geometry, 0x1beu32]);
        assert_eq!(e.mem.u32(list[1] + 8), 0x7d4);
        e.set_global(0x011f_9544u32, 0u32);
        e.call(0x00c0_bc00, &args![0u32, owner, geometry, 0x1c8u32]);
        assert_eq!(e.mem.u32(list[1] + 8), 0x7d4);
        e.mem.set_u32(geometry + 0x20, 0);
        e.set_global(0x011f_9427u32, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x00c0_bc00, &args![0u32, owner, geometry, 0x1c8u32]);
        assert_eq!(
            calls_to(&e, GEOMETRY_FLAG_004A2020),
            vec![vec![geometry, 0x15]]
        );
        assert_eq!(e.mem.u32(list[1] + 8), 0x7d4);
    }

    #[test]
    fn stage_three_records_whether_a_texture_was_found() {
        let mut e = Engine::new();
        e.map(0x0120_1000, 0x1000);
        let (owner, list, geometry) = stages(&mut e, false);
        e.mem.set_f32(geometry + 0x30, 0.25);
        assert_eq!(
            e.call(0x00c0_bc90, &args![0x9000u32, owner, geometry])
                .u32(),
            list[3]
        );
        assert_eq!(e.mem.u32(list[3] + 8), 0x505);
        assert_eq!(e.global::<f32>(0x0120_1bac), 1.0);
        assert_eq!(e.global::<f32>(0x0120_1ba8), 0.25);
        let (owner, list, geometry) = stages(&mut e, true);
        e.set_global(0x011f_9514u32, 0x714u32);
        e.call(0x00c0_bc90, &args![0x9000u32, owner, geometry]);
        assert_eq!(e.mem.u32(list[3] + 8), 0x714);
        assert_eq!(e.global::<f32>(0x0120_1bac), 0.0);
    }

    #[test]
    fn forwarders_pass_their_words() {
        let mut e = Engine::new();
        e.register(CALL_00B7A870, |_, a| answer(a[0] + a[1]));
        e.register(STRIPS_CREATE_CLONE, |_, a| answer(a[1] + 1));
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00c1_a2f0, &args![0u32, 0x0003_0005u32]).u32(), 5);
        assert_eq!(calls_to(&e, CALL_00B7A870), vec![vec![5, 0]]);
        assert_eq!(e.call(0x00c1_a390, &args![0x9000u32, 0x40u32]).u32(), 0x41);
    }
}

#[cfg(test)]
mod n080_switch_tests {
    use super::*;

    fn answer(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// The pass globals mapped, a stage owner with seven stages in `global`, and a
    /// geometry whose slot +0xf4 answers `0x500 + first word`.
    fn setup(e: &mut Engine, global: u32) -> (u32, Vec<u32>, u32) {
        for page in [0x0120_0000u32, 0x0120_1000, 0x011f_9000, 0x011f_4000] {
            e.map(page, 0x1000);
        }
        let owner = e.mem.alloc(0x28);
        let array = e.mem.alloc(0x1c);
        let list: Vec<u32> = (0..7).map(|_| e.mem.alloc(0x10)).collect();
        for (index, stage) in list.iter().enumerate() {
            e.mem.set_u32(array + 4 * index as u32, *stage);
        }
        e.mem.set_u32(owner + 0x24, array);
        e.set_global(global, owner);
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0xf4, 0x001c_00f4);
        e.register(0x001c_00f4, |_, a| answer(0x500 + a[1]));
        let geometry = e.mem.alloc(0x100);
        e.mem.set_u32(geometry, vtable);
        for offset in [0xacu32, 0xb0, 0xb4] {
            let object = e.mem.alloc(8);
            e.mem.set_u32(object, 0x6000 + offset);
            e.mem.set_u32(object + 4, 0x7000 + offset);
            e.mem.set_u32(geometry + offset, object);
        }
        for address in [
            STAGE_SETUP_00BB9640,
            OWNER_STEP_00BBC610,
            GEOMETRY_STEP_00BBDCD0,
            GEOMETRY_STEP_00BBDDA0,
            FINISH_00B78A70,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        (owner, list, geometry)
    }

    #[test]
    fn low_numbers_and_unmapped_numbers() {
        let mut e = Engine::new();
        let (owner, _, geometry) = setup(&mut e, 0x0120_0ed4);
        e.set_global(0x0120_0ec4u32, 0x44u32);
        e.set_global(0x0120_0ec0u32, 0x40u32);
        assert_eq!(
            e.call(0x00c0_bd40, &args![0x9000u32, 10u32, geometry])
                .u32(),
            0x44
        );
        assert_eq!(
            e.call(0x00c0_bd40, &args![0x9000u32, 0x178u32, geometry])
                .u32(),
            0x40
        );
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00c0_bd40, &args![0x9000u32, 14u32, geometry])
                .u32(),
            owner
        );
        assert_eq!(
            calls_to(&e, GEOMETRY_STEP_00BBDDA0),
            vec![vec![0x9000, geometry, owner]]
        );
        for pass in [9u32, 0x10, 0x179, 0x1c9, 0x1e2, 0x2ee] {
            assert_eq!(
                e.call(0x00c0_bd40, &args![0x9000u32, pass, geometry]).u32(),
                0
            );
        }
    }

    #[test]
    fn light_stages_and_floats() {
        let mut e = Engine::new();
        let (owner, list, geometry) = setup(&mut e, 0x0120_0ee4);
        e.set_global(0x011f_9530u32, 0x9530u32);
        for word in 0..4 {
            e.mem.set_u32(geometry + 0x84 + 4 * word, 0x840 + word);
        }
        assert_eq!(
            e.call(0x00c0_bd40, &args![0x9000u32, 0x181u32, geometry])
                .u32(),
            owner
        );
        assert_eq!(e.global::<u32>(0x0120_1b5c), 0x843);
        assert_eq!(e.mem.u32(list[4] + 8), 0x9530);
        assert_eq!(e.mem.u32(list[5] + 8), 0x60b4);
        let (owner, list, geometry) = setup(&mut e, 0x0120_0f64);
        e.mem.set_f32(geometry + 0xe8, 2.0);
        e.mem.set_u32(geometry + 0xec, 0x7f80_0007);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00c0_bd40, &args![0x9000u32, 0x1a1u32, geometry])
                .u32(),
            owner
        );
        assert_eq!(e.global::<f32>(0x0120_1b90), 2.0);
        assert_eq!(e.global::<u32>(0x0120_1b94), 0x7fc0_0007);
        // 00c0bbd0 (stages 2, 3) and 00c0bae0 (stage 4, slot +0xf4 with 3).
        assert_eq!(e.mem.u32(list[4] + 8), 0x503);
        assert_eq!(
            calls_to(&e, STAGE_SETUP_00BB9640),
            vec![vec![0x9000, list[4], geometry]]
        );
    }

    #[test]
    fn pair_cases_fill_stages_four_to_six() {
        let mut e = Engine::new();
        let (owner, list, geometry) = setup(&mut e, 0x0120_0f6c);
        e.set_global(0x011f_95e4u32, 0x95e4u32);
        assert_eq!(
            e.call(0x00c0_bd40, &args![0x9000u32, 0x1a3u32, geometry])
                .u32(),
            owner
        );
        assert_eq!(e.mem.u32(list[2] + 8), 0x95e4);
        assert_eq!(
            [4usize, 5, 6].map(|i| e.mem.u32(list[i] + 8)),
            [0x70ac, 0x70b0, 0x502]
        );
        let (_, list, geometry) = setup(&mut e, 0x0120_0f10);
        e.call(0x00c0_bd40, &args![0x9000u32, 0x18cu32, geometry]);
        assert_eq!(e.mem.u32(list[6] + 8), 0x60b4);
        // 0x19c shares that case and its global.
        let (_, list, geometry) = setup(&mut e, 0x0120_0f14);
        e.call(0x00c0_bd40, &args![0x9000u32, 0x19cu32, geometry]);
        assert_eq!(e.mem.u32(list[6] + 8), 0x60b4);
    }

    #[test]
    fn shadow_offset_subtracts_the_point() {
        let mut e = Engine::new();
        let (owner, _, geometry) = setup(&mut e, 0x0120_0fec);
        let accumulator = e.mem.alloc(0x200);
        e.register_double(GET_ACCUMULATOR, move |_, _| answer(accumulator));
        let holder = e.mem.alloc(0x24);
        let position = e.mem.alloc(0xc);
        for (index, value) in [5.0f32, 6.0, 7.0].iter().enumerate() {
            e.mem.set_f32(position + 4 * index as u32, *value);
            e.set_global(0x011f_474c + 4 * index as u32, 1.0f32);
        }
        e.mem.set_u32(holder + 0x20, position);
        let cell = e.mem.alloc(4);
        e.mem.set_u32(cell, holder);
        e.set_global(0x011f_91e0u32, cell);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00c0_bd40, &args![0x9000u32, 0x1c3u32, geometry])
                .u32(),
            owner
        );
        assert_eq!(
            [0u32, 4, 8].map(|o| e.global::<f32>(0x0120_1360 + o)),
            [4.0, 5.0, 6.0]
        );
        // Without a position the point at 011f4288 is used.
        e.mem.set_u32(holder + 0x20, 0);
        e.set_global(0x011f_4288u32, 3.0f32);
        e.call(0x00c0_bd40, &args![0x9000u32, 0x1c3u32, geometry]);
        assert_eq!(e.global::<f32>(0x0120_1360), 2.0);
    }

    #[test]
    fn owner_steps_and_light_values() {
        let mut e = Engine::new();
        let (owner, list, geometry) = setup(&mut e, 0x0120_1014);
        e.set_global(0x011f_9510u32, 0x9510u32);
        e.mem.set_u32(e.mem.u32(geometry + 0xb0), 0);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00c0_bd40, &args![0x9000u32, 0x1ceu32, geometry])
                .u32(),
            owner
        );
        assert_eq!(e.mem.u32(list[0] + 8), 0x9510);
        assert_eq!(
            calls_to(&e, OWNER_STEP_00BBC610),
            vec![vec![0x9000, owner, 1]]
        );
        let (owner, _, geometry) = setup(&mut e, 0x0120_1044);
        let object = e.mem.alloc(0x60);
        e.mem.set_f32(object + 0x58, 0.75);
        let middle = e.mem.alloc(0x34);
        e.mem.set_u32(middle + 0x30, object);
        let holder = e.mem.alloc(0xc);
        e.mem.set_u32(holder + 8, middle);
        e.set_global(0x011f_91acu32, holder);
        e.register(LIGHT_VALUE_00C02D40, |_, _| Ret {
            st0: 1.25,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00c0_bd40, &args![0x9000u32, 0x1dbu32, geometry])
                .u32(),
            owner
        );
        assert_eq!(calls_to(&e, LIGHT_VALUE_00C02D40), vec![vec![object]]);
        assert_eq!(e.global::<f32>(0x0120_1384), 1.25);
        assert_eq!(e.global::<f32>(0x0120_1388), 0.75);
        assert_eq!(
            calls_to(&e, OWNER_STEP_00BBC610),
            vec![vec![0x9000, owner, 0]]
        );
        assert_eq!(calls_to(&e, FINISH_00B78A70).len(), 1);
        let (owner2, _, geometry2) = setup(&mut e, 0x0120_1058);
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00c0_bd40, &args![0x9000u32, 0x1e0u32, geometry2])
                .u32(),
            owner2
        );
        assert_eq!(
            calls_to(&e, GEOMETRY_STEP_00BBDCD0),
            vec![vec![0x9000, geometry2, owner2]]
        );
    }
}

#[cfg(test)]
mod n080_render_pass_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn answer(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn calls_to(e: &Engine, address: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    type Flags = Rc<RefCell<Vec<u32>>>;

    struct Scene {
        e: Engine,
        this: u32,
        geometry: u32,
        accumulator: u32,
        flags: Flags,
        lights: Vec<u32>,
    }

    /// A property with its five pass arrays, a geometry, an accumulator and doubles for the
    /// callees: the flag test answers the numbers in `flags`, the light walk goes over
    /// `lights` (shadow ones have +0xec set), `Add` appends a pass.
    fn scene(shadow_lights: &[bool]) -> Scene {
        let mut e = Engine::new();
        for page in [0x011f_4000u32, 0x011f_9000] {
            e.map(page, 0x1000);
        }
        let this = e.mem.alloc(0x108);
        for offset in [0x3cu32, 0x40, 0x44, 0x48, 0x54] {
            let list = e.mem.alloc(0x14);
            let items = e.mem.alloc(0x40);
            e.mem.set_u32(list + 4, items);
            e.mem.set_u32(this + offset, list);
        }
        let b4 = e.mem.alloc(4);
        e.mem.set_u32(this + L30_WORD_B4, b4);
        e.mem.set_f32(this + L30_ALPHA, 1.0);
        e.mem.set_f32(this + L30_FADE_ALPHA, 1.0);
        let geometry = e.mem.alloc(0xc0);
        let data = e.mem.alloc(0x30);
        e.mem.set_u32(geometry + 0xb8, data);
        let property_state = e.mem.alloc(0x20);
        e.mem.set_u32(geometry + 0x9c, property_state);
        let accumulator = e.mem.alloc(0x200);
        let camera = e.mem.alloc(0x200);
        e.mem.set_u32(accumulator + 0x194, camera);
        e.mem.set_u32(camera + 0xe0, 0xe0e0);
        let renderer = e.mem.alloc(0x10);
        let renderer_object = e.mem.alloc(0x1a4);
        e.mem.set_u32(renderer + 8, renderer_object);
        e.mem.set_u32(renderer_object + 0x1a0, 0x1a01);
        e.set_global(L30_RENDERER, renderer);
        e.register_double(GET_ACCUMULATOR, move |_, _| answer(accumulator));
        let flags: Flags = Default::default();
        let wanted = flags.clone();
        e.register_double(GEOMETRY_FLAG_004A2020, move |_, a| {
            answer(u32::from(wanted.borrow().contains(&a[1])))
        });
        e.register(L30_ADD_PASS, |e, a| {
            let pass = e.mem.alloc(0x10);
            e.mem.set_u32(pass, a[1]);
            e.mem.set_u16(pass + 4, a[2] as u16);
            e.mem.set_u8(pass + 9, a[4] as u8);
            let lights = e.mem.alloc(0x10);
            e.mem.set_u32(lights, a[5]);
            e.mem.set_u32(pass + 0xc, lights);
            let size = e.mem.u32(a[0] + 0x10);
            let items = e.mem.u32(a[0] + 4);
            e.mem.set_u32(items + 4 * size, pass);
            e.mem.set_u32(a[0] + 0x10, size + 1);
            Ret::default()
        });
        e.register(L30_PASS_ALLOCATE_LIGHTS, |e, a| {
            let lights = e.mem.alloc(4 * (a[1] & 0xff));
            e.mem.set_u32(a[0] + 0xc, lights);
            Ret::default()
        });
        let lights: Vec<u32> = shadow_lights
            .iter()
            .map(|shadow| {
                let light = e.mem.alloc(0xfc);
                e.mem.set_u8(light + 0xec, u8::from(*shadow));
                light
            })
            .collect();
        let walk = lights.clone();
        let first = move |e: &mut Engine, iterator: u32, shadow_too: bool| {
            let mut index = 0usize;
            while index < walk.len() && !shadow_too && e.mem.u8(walk[index] + 0xec) != 0 {
                index += 1;
            }
            e.mem.set_u32(iterator, index as u32);
            walk.get(index).copied().unwrap_or(0)
        };
        let walk = lights.clone();
        let next = move |e: &mut Engine, iterator: u32, shadow_too: bool| {
            let mut index = e.mem.u32(iterator) as usize + 1;
            while index < walk.len() && !shadow_too && e.mem.u8(walk[index] + 0xec) != 0 {
                index += 1;
            }
            e.mem.set_u32(iterator, index as u32);
            walk.get(index).copied().unwrap_or(0)
        };
        let (first_all, next_all) = (first.clone(), next.clone());
        e.register_double(L30_FIRST_ACTIVE_LIGHT, move |e, a| {
            answer(first_all(e, a[1], true))
        });
        e.register_double(L30_NEXT_ACTIVE_LIGHT, move |e, a| {
            answer(next_all(e, a[1], true))
        });
        e.register_double(L30_FIRST_NON_SHADOW_LIGHT, move |e, a| {
            answer(first(e, a[1], false))
        });
        e.register_double(L30_NEXT_NON_SHADOW_LIGHT, move |e, a| {
            answer(next(e, a[1], false))
        });
        for address in [
            L30_CHECK_CREATE_PASSES,
            L30_CHECK_CREATE_DEPTH_PASSES,
            L30_CHECK_CREATE_CONST_ALPHA_PASSES,
            L30_CHECK_CREATE_LOCAL_MAP_PASSES,
            L30_CHECK_CREATE_PASSES_54,
            L30_PASS_FREE_LIGHTS,
            L30_SET_FLAG,
            L30_SET_ALPHA,
            L30_RESORT_LIGHTS,
            L30_DEPTH_PASSES_WANTED,
            L30_SETTING_TEST_00B60330,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        Scene {
            e,
            this,
            geometry,
            accumulator,
            flags,
            lights,
        }
    }

    fn run(s: &mut Scene, mode: u32, kind: u32) -> u32 {
        s.e.call(
            0x00c0_58f0,
            &args![s.this, s.geometry, mode, 0u32, kind, s.accumulator, 0u32],
        )
        .u32()
    }

    fn passes(e: &Engine, list: u32) -> Vec<u32> {
        let items = e.mem.u32(list + 4);
        (0..e.mem.u32(list + 0x10))
            .map(|i| e.mem.u32(items + 4 * i))
            .collect()
    }

    #[test]
    fn side_kinds_make_their_one_pass() {
        let mut s = scene(&[]);
        s.flags.borrow_mut().push(1);
        let depth = s.e.mem.u32(s.this + L30_DEPTH_PASSES);
        assert_eq!(run(&mut s, 0, 7), depth);
        let made = passes(&s.e, depth);
        assert_eq!(s.e.mem.u16(made[0] + 4), 0x1ca);
        assert_eq!(s.e.mem.u32(s.e.mem.u32(made[0] + 0xc)), 0x1a01);
        // A filled list refreshes its first light.
        s.e.mem.set_u32(s.e.mem.u32(made[0] + 0xc), 0);
        run(&mut s, 0, 7);
        assert_eq!(passes(&s.e, depth).len(), 1);
        assert_eq!(s.e.mem.u32(s.e.mem.u32(made[0] + 0xc)), 0x1a01);
        let alpha = s.e.mem.u32(s.this + L30_CONST_ALPHA_PASSES);
        assert_eq!(run(&mut s, 0, 8), alpha);
        assert_eq!(s.e.mem.u16(passes(&s.e, alpha)[0] + 4), 0x1d5);
        let local = s.e.mem.u32(s.this + L30_LOCAL_MAP_PASSES);
        run(&mut s, 0, 9);
        assert_eq!(s.e.mem.u16(passes(&s.e, local)[0] + 4), 0x1de);
        let list_54 = s.e.mem.u32(s.this + L30_PASSES_54);
        s.flags.borrow_mut().push(0x25);
        run(&mut s, 0, 0xf);
        assert_eq!(s.e.mem.u16(passes(&s.e, list_54)[0] + 4), 0x1bc);
        s.e.mem.set_u32(s.this + L30_FLAGS0, 0x8000);
        assert_eq!(run(&mut s, 0, 7), 0);
    }

    #[test]
    fn rebuild_adds_the_main_pass_with_its_lights() {
        let mut s = scene(&[false, true]);
        s.flags.borrow_mut().push(1);
        s.e.call_log = Some(vec![]);
        let main = s.e.mem.u32(s.this + L30_PASSES);
        assert_eq!(run(&mut s, 2, 0), main);
        let made = passes(&s.e, main);
        assert_eq!(made.len(), 1);
        assert_eq!(s.e.mem.u16(made[0] + 4), 0x180);
        assert_eq!(s.e.mem.u8(made[0] + 9), 2);
        let lights = s.e.mem.u32(made[0] + 0xc);
        assert_eq!(
            [s.e.mem.u32(lights), s.e.mem.u32(lights + 4)],
            [0xe0e0, s.lights[0]]
        );
        assert_eq!(
            calls_to(&s.e, L30_PASS_ALLOCATE_LIGHTS),
            vec![vec![made[0], 2]]
        );
        assert_eq!(calls_to(&s.e, L30_SET_FLAG), vec![vec![s.this, 0x1e, 0]]);
        assert_eq!(s.e.mem.u32(s.this + L30_LAST_STATE), 2);
        // The same state again builds nothing.
        s.e.call_log = Some(vec![]);
        run(&mut s, 2, 0);
        assert!(calls_to(&s.e, L30_ADD_PASS).is_empty());
    }

    #[test]
    fn flag_0f_makes_one_hinted_pass() {
        let mut s = scene(&[]);
        s.flags.borrow_mut().extend([1, 0xf]);
        let main = s.e.mem.u32(s.this + L30_PASSES);
        run(&mut s, 5, 0);
        let made = passes(&s.e, main);
        assert_eq!(made.len(), 1);
        assert_eq!(s.e.mem.u16(made[0] + 4), 0x1ce);
        assert_eq!(s.e.mem.u8(made[0] + 6), 6);
        assert_eq!(s.e.mem.u32(s.this + L30_LAST_STATE), 5);
        // A zero alpha stops before any pass.
        let mut s = scene(&[]);
        s.e.mem.set_f32(s.this + L30_ALPHA, 0.0);
        s.e.call_log = Some(vec![]);
        run(&mut s, 5, 0);
        assert!(calls_to(&s.e, L30_ADD_PASS).is_empty());
        assert_eq!(s.e.mem.u32(s.this + L30_LAST_STATE), 0);
    }

    #[test]
    fn unchanged_state_refreshes_the_lights() {
        let mut s = scene(&[false, false]);
        let main = s.e.mem.u32(s.this + L30_PASSES);
        let pass = s.e.mem.alloc(0x10);
        s.e.mem.set_u16(pass + 4, 0x180);
        s.e.mem.set_u8(pass + 0xa, 1);
        let items = s.e.mem.u32(main + 4);
        s.e.mem.set_u32(items, pass);
        s.e.mem.set_u32(main + 0x10, 1);
        s.e.mem.set_u32(s.this + L30_LAST_STATE, 3);
        s.e.mem.set_u8(s.this + L30_LIGHTS_DIRTY, 1);
        s.e.call_log = Some(vec![]);
        run(&mut s, 3, 0);
        assert_eq!(calls_to(&s.e, L30_RESORT_LIGHTS).len(), 1);
        assert!(calls_to(&s.e, L30_ADD_PASS).is_empty());
        assert_eq!(s.e.mem.u8(pass + 9), 3);
        let lights = s.e.mem.u32(pass + 0xc);
        assert_eq!(
            [0u32, 4, 8].map(|o| s.e.mem.u32(lights + o)),
            [0xe0e0, s.lights[0], s.lights[1]]
        );
        assert_eq!(s.e.mem.u8(s.this + L30_LIGHTS_DIRTY), 0);
    }

    #[test]
    fn main_fade_crossing_clears_the_state() {
        let mut s = scene(&[]);
        s.e.mem.set_u32(s.this + L30_FLAGS0, 1);
        s.e.set_global(L30_FADE_FAR_011F9458, 10.0f32);
        s.e.mem.set_f32(s.this + L30_CAMERA_DISTANCE, 5.0);
        s.e.mem.set_u32(s.this + L30_LAST_STATE, 0x300);
        let bound = s.e.mem.alloc(0x10);
        s.e.mem.set_f32(bound, 20.0);
        s.e.mem.set_f32(bound + 0xc, 2.0);
        s.e.mem.set_u32(s.geometry + 0x20, bound);
        s.e.register(L30_VECTOR_LENGTH, |e, a| Ret {
            st0: f64::from(e.mem.f32(a[0])),
            ..Ret::default()
        });
        s.e.mem.set_f32(s.this + L30_ALPHA, 0.0);
        run(&mut s, 0, 3);
        assert_eq!(s.e.mem.f32(s.this + L30_CAMERA_DISTANCE), 18.0);
        assert_eq!(s.e.mem.u32(s.this + L30_LAST_STATE), 0);
    }
}
