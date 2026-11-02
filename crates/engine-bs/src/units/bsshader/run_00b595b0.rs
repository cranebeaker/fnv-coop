//! `BSShader/run_00b595b0` (Xbox PDB source unit), subsystem `BSShader`: its functions in
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
/// The vtables the two constructors store (`010adc48`, `010adc68`; no RTTI locator before
/// them).
const VTABLE_010ADC48: u32 = 0x010a_dc48;
const VTABLE_010ADC68: u32 = 0x010a_dc68;
/// The `float` constants they copy: 0.5 (`01016248`), 1000.0 (`01013974`), the word at
/// `012023e0` and 600.0 (`0101786c`) (values read in Ghidra).
const HALF_01016248: u32 = 0x0101_6248;
const CONSTANT_01013974: u32 = 0x0101_3974;
const CONSTANT_012023E0: u32 = 0x0120_23e0;
const CONSTANT_0101786C: u32 = 0x0101_786c;
/// The bits of 1.0 (`FLD1`).
const ONE_BITS: u32 = 0x3f80_0000;

// Translated from 00b5a410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`): an image-space effect's constructor, `thiscall`,
/// no stack words. Runs the base constructor `00ba4080`, stores the vtable `010adc48`, sizes
/// the array at +0x28 to 2 (`00a90180`) and copies the `float` at `01016248` (`FLD`/`FST`, a
/// signalling NaN quieted) to +0x6c and +0x70. Answers `this`.
// Translation incomplete: 00b5a410 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
pub fn fn_00b5a410(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010ADC48);
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 2u32]);
    let half = x87_quiet_bits(e.global::<u32>(HALF_01016248));
    e.mem.set_u32(base + 0x6c, half);
    e.mem.set_u32(base + 0x70, half);
    this
}

// Translated from 00b5a590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`): an image-space effect's constructor, `thiscall`,
/// no stack words. Runs the base constructor `00ba4080`, stores the vtable `010adc68`, stores
/// 0.0 at +0x5c to +0x98, clears the bytes +0xa4 to +0xa6 and the words +0xa8 to +0xb0,
/// constructs the two 16-byte textures at +0xb4 (`eh vector constructor iterator` with
/// `004ebb70` and `ImageSpaceTexture::~ImageSpaceTexture`), sizes the array at +0x28 to 4
/// (`00a90180`), then stores (through the FPU, signalling NaNs quieted) 1.0 at +0x7c, the
/// `float` at `01013974` at +0x80, 0.0 at +0x84 and +0x6c, the word at `012023e0` at +0x88,
/// the `float` at `0101786c` at +0x9c, 1.0 at +0x70 and +0x74, 0.0 at +0x78, and clears +0x58.
/// Answers `this`.
// Translation incomplete: 00b5a590 - the C++ exception frame (FS:[0] chain, unwind states 0 to 2 for the base and the textures) is not translated.
pub fn fn_00b5a590(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(EFFECT_CONSTRUCT_00BA4080, &args![base]);
    e.mem.set_u32(base, VTABLE_010ADC68);
    for offset in (0x5cu32..=0x98).step_by(4) {
        e.mem.set_u32(base + offset, 0);
    }
    for offset in [0xa4u32, 0xa5, 0xa6] {
        e.mem.set_u8(base + offset, 0);
    }
    for offset in [0xa8u32, 0xac, 0xb0] {
        e.mem.set_u32(base + offset, 0);
    }
    e.call(
        VECTOR_CONSTRUCT_00EC782F,
        &args![
            base + 0xb4,
            0x10u32,
            2u32,
            TEXTURE_CONSTRUCT_004EBB70,
            TEXTURE_DESTRUCT_00BA3A00
        ],
    );
    e.call(ARRAY_SET_SIZE_00A90180, &args![base + 0x28, 4u32]);
    let thousand = x87_quiet_bits(e.global::<u32>(CONSTANT_01013974));
    let copied = x87_quiet_bits(e.global::<u32>(CONSTANT_012023E0));
    let six_hundred = x87_quiet_bits(e.global::<u32>(CONSTANT_0101786C));
    e.mem.set_u32(base + 0x7c, ONE_BITS);
    e.mem.set_u32(base + 0x80, thousand);
    e.mem.set_u32(base + 0x84, 0);
    e.mem.set_u32(base + 0x6c, 0);
    e.mem.set_u32(base + 0x88, copied);
    e.mem.set_u32(base + 0x9c, six_hundred);
    e.mem.set_u32(base + 0x70, ONE_BITS);
    e.mem.set_u32(base + 0x74, ONE_BITS);
    e.mem.set_u32(base + 0x78, 0);
    e.mem.set_u32(base + 0x58, 0);
    this
}

use crate::units::bsshader::geometrydecalshader::shader_inline::{
    ni_pointer_clear, release_reference, MEMORY_ALLOCATE, MEMORY_MANAGER,
};

/// The two hash maps these functions empty: the getters `00b585c0` and `00b58670` (unnamed,
/// no arguments; each answers a map whose hash size is at +4, bucket array at +8 and count at
/// +0xc), their next-item steps `00b57f20` and `00e6e630` (unnamed; `thiscall` on the map: the
/// position, the key and the value, by address), their `00b601a0` (unnamed, `thiscall`, no
/// stack words: run after the walk), the first map's value destructor `00b595b0` (unnamed,
/// `thiscall`) and `MemoryManager::Deallocate` (`00aa4060`, `thiscall` on the manager, one
/// word).
const FIRST_MAP_00B585C0: u32 = 0x00b5_85c0;
const SECOND_MAP_00B58670: u32 = 0x00b5_8670;
const FIRST_MAP_STEP_00B57F20: u32 = 0x00b5_7f20;
const SECOND_MAP_STEP_00E6E630: u32 = 0x00e6_e630;
const MAP_AFTER_WALK_00B601A0: u32 = 0x00b6_01a0;
const VALUE_DESTRUCT_00B595B0: u32 = 0x00b5_95b0;
const MEMORY_DEALLOCATE_00AA4060: u32 = 0x00aa_4060;
/// The two words `00b59890` clears (`011f963c`, `011f9640`).
const CLEARED_011F963C: u32 = 0x011f_963c;
const CLEARED_011F9640: u32 = 0x011f_9640;

/// The first item of a map: the first non-null bucket's head, or null.
fn first_position(e: &Engine, map: u32) -> u32 {
    let size = e.mem.u32(map + 4);
    let buckets = e.mem.u32(map + 8);
    for index in 0..size {
        let head = e.mem.u32(buckets.wrapping_add(4 * index));
        if head != 0 {
            return head;
        }
    }
    0
}

// Translated from 00b59890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`), no arguments: empties the map `00b585c0`
/// answers. With items in it (count at +0xc) and a first position, walks it (`00b57f20` with
/// the position, key and value locals, the map asked again each step) and destroys each
/// non-null value (`00b595b0`) and frees it (`MemoryManager::Deallocate`) until the position
/// is null; then `00b601a0` on the map, and clears `011f963c` and `011f9640`.
pub fn fn_00b59890(e: &mut Engine) {
    let map = e.call(FIRST_MAP_00B585C0, &args![]).u32();
    let first = first_position(e, map);
    e.with_stack(0xc, |e, locals| {
        // value +0, position +4, key +8 (the exe's frame order).
        let value_slot = locals.addr();
        let position_slot = value_slot + 4;
        let key_slot = value_slot + 8;
        e.mem.set_u32(position_slot, first);
        e.mem.set_u32(value_slot, 0);
        let map = e.call(FIRST_MAP_00B585C0, &args![]).u32();
        if e.mem.u32(map + 0xc) != 0 && first != 0 {
            loop {
                let map = e.call(FIRST_MAP_00B585C0, &args![]).u32();
                e.call(
                    FIRST_MAP_STEP_00B57F20,
                    &args![map, position_slot, key_slot, value_slot],
                );
                let value = e.mem.u32(value_slot);
                if value != 0 {
                    e.call(VALUE_DESTRUCT_00B595B0, &args![value]);
                    e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, value]);
                }
                if e.mem.u32(position_slot) == 0 {
                    break;
                }
            }
        }
    });
    let map = e.call(FIRST_MAP_00B585C0, &args![]).u32();
    e.call(MAP_AFTER_WALK_00B601A0, &args![map]);
    e.set_global(CLEARED_011F963C, 0u32);
    e.set_global(CLEARED_011F9640, 0u32);
}

// Translated from 00b59940 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b59940 - the C++ exception frame (FS:[0] chain, unwind state 0 for the value pointer) is not translated.
/// Unnamed in the engine map (`run_00b595b0`), no arguments: runs `00b59890`, then empties the
/// map `00b58670` answers. The value local is an `NiPointer`. With items in it and a first
/// position, walks it (`00e6e630`, the map asked again each step) and for each non-null value
/// releases and clears its pointers at +0x08, frees the block at +0x10 (cleared), then
/// releases and clears its pointers at +0x18, +0x20, +0x1c and +0x24, until the position is
/// null; then `00b601a0` on the map (not when the map has no items). Last the value local's
/// reference is released.
pub fn fn_00b59940(e: &mut Engine) {
    fn_00b59890(e);
    let map = e.call(SECOND_MAP_00B58670, &args![]).u32();
    let first = first_position(e, map);
    let mut value = 0u32;
    e.with_stack(0xc, |e, locals| {
        // value +0, position +4, key +8 (the exe's frame order).
        let value_slot = locals.addr();
        let position_slot = value_slot + 4;
        let key_slot = value_slot + 8;
        e.mem.set_u32(position_slot, first);
        e.mem.set_u32(value_slot, 0);
        let map = e.call(SECOND_MAP_00B58670, &args![]).u32();
        if e.mem.u32(map + 0xc) == 0 {
            return;
        }
        if first != 0 {
            loop {
                let map = e.call(SECOND_MAP_00B58670, &args![]).u32();
                e.call(
                    SECOND_MAP_STEP_00E6E630,
                    &args![map, position_slot, key_slot, value_slot],
                );
                value = e.mem.u32(value_slot);
                if value != 0 {
                    ni_pointer_clear(e, value + 8);
                    let block = e.mem.u32(value + 0x10);
                    e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, block]);
                    e.mem.set_u32(value + 0x10, 0);
                    for offset in [0x18u32, 0x20, 0x1c, 0x24] {
                        ni_pointer_clear(e, value + offset);
                    }
                }
                if e.mem.u32(position_slot) == 0 {
                    break;
                }
            }
        }
        let map = e.call(SECOND_MAP_00B58670, &args![]).u32();
        e.call(MAP_AFTER_WALK_00B601A0, &args![map]);
    });
    if value != 0 {
        release_reference(e, value);
    }
}

/// The array setter `00ba3aa0` (`thiscall` on an `NiTPrimitiveArray`: index, the address of
/// the word) and `ImageSpaceEffect::~ImageSpaceEffect` (engine map, `00ba3d30`).
const ARRAY_SET_AT_00BA3AA0: u32 = 0x00ba_3aa0;
const EFFECT_DESTRUCT_00BA3D30: u32 = 0x00ba_3d30;

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

// Translated from 00b59c20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b59c20 - the C++ exception frame (FS:[0] chain, unwind state 0 for the base) is not translated.
/// Unnamed in the engine map (`run_00b595b0`): the destructor of the effect `00b5a410` builds,
/// `thiscall`, no stack words: the vtable `010adc48`, effect parameter 0 deleted, then
/// `ImageSpaceEffect::~ImageSpaceEffect`.
pub fn fn_00b59c20(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010ADC48);
    delete_param(e, base, 0);
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

/// `ImageSpaceShaderParam::SetPixelConstant` (engine map, `00b8a790`, `thiscall`: index, four
/// floats).
const PARAM_SET_PIXEL_CONSTANT_00B8A790: u32 = 0x00b8_a790;

// Translated from 00b59cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`): the `UpdateParams` slot of the effect
/// `00b5a410` builds, `thiscall`, one word it does not read. Effect parameter 0 takes pixel
/// constant 0 from the `float`s +0x58, +0x5c, +0x60, +0x64 and constant 1 from +0x6c, +0x70,
/// +0x68 and 0.0 (each through `FLD`/`FSTP`, a signalling NaN quieted). Answers true.
pub fn fn_00b59cb0(e: &mut Engine, this: Ptr, _unused_0: u32) -> bool {
    let base = this.addr();
    let param = e.mem.u32(e.mem.u32(base + 0x1c));
    let float = |e: &Engine, offset: u32| x87_quiet_bits(e.mem.u32(base + offset));
    let first = [
        float(e, 0x58),
        float(e, 0x5c),
        float(e, 0x60),
        float(e, 0x64),
    ];
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![param, 0u32, first[0], first[1], first[2], first[3]],
    );
    let second = [float(e, 0x6c), float(e, 0x70), float(e, 0x68)];
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![param, 1u32, second[0], second[1], second[2], 0u32],
    );
    true
}

/// The byte both effects' activity needs (`011f968c`).
const ENABLED_011F968C: u32 = 0x011f_968c;

/// 1 when `0.0 < value` (`FLDZ`, `FCOMP`, `TEST AH,5`, `JP`: false for NaN) and the byte
/// `011f968c` is set, else 0.
fn active_when_positive(e: &Engine, value: f32) -> u32 {
    u32::from(0.0 < value && e.global::<u8>(ENABLED_011F968C) != 0)
}

// Translated from 00b59d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`; Ghidra made no function of it): the `IsActive`
/// slot of the effect `00b5a410` builds, `thiscall`, no stack words: 1 when the `float` +0x58
/// is above 0 and the byte `011f968c` is set, else 0.
pub fn fn_00b59d10(e: &mut Engine, this: Ptr) -> u32 {
    let value = e.mem.f32(this.addr() + 0x58);
    active_when_positive(e, value)
}

/// `eh vector destructor iterator` (`00ec5fce`, `stdcall`: array, element size, count,
/// destructor) and `ImageSpaceTexture::~ImageSpaceTexture` (`00ba3a00`).
const VECTOR_DESTRUCT_00EC5FCE: u32 = 0x00ec_5fce;

// Translated from 00b5a110 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b5a110 - the C++ exception frame (FS:[0] chain, unwind states 2 to 0 for the textures and the base) is not translated.
/// Unnamed in the engine map (`run_00b595b0`): the destructor of the effect `00b5a590` builds,
/// `thiscall`, no stack words. Stores the vtable `010adc68`; a texture 2 of `pTextures` (+0x2c
/// elements) is destroyed, freed and its element set to null (the element tested twice);
/// effect parameters 1 and 0 are deleted; the two textures at +0xb4 are destroyed (`eh vector
/// destructor iterator`), then the texture at +0xa4 and `ImageSpaceEffect::~ImageSpaceEffect`.
pub fn fn_00b5a110(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, VTABLE_010ADC68);
    let element = e.mem.u32(base + 0x2c) + 8;
    if e.mem.u32(element) != 0 {
        let texture = e.mem.u32(element);
        if texture != 0 {
            e.call(TEXTURE_DESTRUCT_00BA3A00, &args![texture]);
            e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, texture]);
        }
        set_at(e, base + 0x28, 2, 0);
    }
    delete_param(e, base, 1);
    delete_param(e, base, 0);
    e.call(
        VECTOR_DESTRUCT_00EC5FCE,
        &args![base + 0xb4, 0x10u32, 2u32, TEXTURE_DESTRUCT_00BA3A00],
    );
    e.call(TEXTURE_DESTRUCT_00BA3A00, &args![base + 0xa4]);
    e.call(EFFECT_DESTRUCT_00BA3D30, &args![base]);
}

// Translated from 00b5a3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`; Ghidra made no function of it): the `IsActive`
/// slot of the effect `00b5a590` builds, `thiscall`, no stack words: 1 when the `float` +0x6c
/// is above 0 and the byte `011f968c` is set, else 0.
pub fn fn_00b5a3f0(e: &mut Engine, this: Ptr) -> u32 {
    let value = e.mem.f32(this.addr() + 0x6c);
    active_when_positive(e, value)
}

// Translated from 00b5a480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`): the scalar deleting destructor of the effect
/// `00b5a410` builds, `thiscall`, one word: `00b59c20`, then the memory freed when bit 0 of
/// `flags` is set. Answers `this`.
pub fn fn_00b5a480(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b59c20(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// The effect-array sizer `00ba3f90` (`thiscall`, one word), `ImageSpaceShaderParam`'s
/// constructor (`00b8a9e0`) and `InitConstantMap` (`00b8ab10`, two words), the input setters
/// `00ba3b10` and `00ba3ba0` and the effect setter `00ba3c70` (unit `run_00ba3aa0`), the
/// manager's effect index (`011f9698`) and the `float` constants the parameters take
/// (`0101e2bc`, `01013974`, `01013d84`).
const SIZE_ARRAYS_00BA3F90: u32 = 0x00ba_3f90;
const PARAM_CONSTRUCT_00B8A9E0: u32 = 0x00b8_a9e0;
const PARAM_INIT_CONSTANT_MAP_00B8AB10: u32 = 0x00b8_ab10;
const SET_INPUT_00BA3B10: u32 = 0x00ba_3b10;
const SET_INPUTS_00BA3BA0: u32 = 0x00ba_3ba0;
const SET_EFFECT_00BA3C70: u32 = 0x00ba_3c70;
const EFFECT_INDEX_011F9698: u32 = 0x011f_9698;
const CONSTANT_0101E2BC: u32 = 0x0101_e2bc;
const CONSTANT_01013D84: u32 = 0x0101_3d84;

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

/// The manager's effect picked by `011f9698` (its +8 list, +0xd8 onwards).
fn indexed_effect(e: &Engine, manager: u32) -> u32 {
    let index = e.global::<u32>(EFFECT_INDEX_011F9698);
    let list = e.mem.u32(manager + 8);
    e.mem
        .u32(list.wrapping_add(index.wrapping_mul(4)).wrapping_add(0xd8))
}

// Translated from 00b5a4a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b5a4a0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the parameter allocation) is not translated.
/// Unnamed in the engine map (`run_00b595b0`): the `Setup` slot of the effect `00b5a410`
/// builds, `thiscall`, two words (the manager; a word it does not read). Sizes the arrays to 1
/// (`00ba3f90`); one parameter (`InitConstantMap(0, 2)`, pixel constant 0 = the `float` at
/// `0101e2bc` (through `FLD`/`FSTP`), 0, 0, 0); effect 0 is the manager's effect picked by
/// `011f9698`; the parameter is stored as 0 and input 0 is texture 1 (filter mode 0).
pub fn fn_00b5a4a0(e: &mut Engine, this: Ptr, manager: u32, _unused_1: u32) {
    let base = this.addr();
    e.call(SIZE_ARRAYS_00BA3F90, &args![base, 1u32]);
    let param = new_param(e, 2);
    let scale = x87_quiet_bits(e.global::<u32>(CONSTANT_0101E2BC));
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![param, 0u32, scale, 0u32, 0u32, 0u32],
    );
    let effect = indexed_effect(e, manager);
    e.call(SET_EFFECT_00BA3C70, &args![base, 0u32, effect, param, 0u32]);
    set_at(e, base + 0x18, 0, param);
    e.call(SET_INPUT_00BA3B10, &args![base, 0u32, 1u32, 0u32]);
}

// Translated from 00b5a700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`): the scalar deleting destructor of the effect
/// `00b5a590` builds, `thiscall`, one word: `00b5a110`, then the memory freed when bit 0 of
/// `flags` is set. Answers `this`.
pub fn fn_00b5a700(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00b5a110(e, this);
    if flags & 1 != 0 {
        e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00b5a720 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b5a720 - the C++ exception frame (FS:[0] chain, unwind states 0 and 1 for the two parameter allocations) is not translated.
/// Unnamed in the engine map (`run_00b595b0`): the `Setup` slot of the effect `00b5a590`
/// builds, `thiscall`, two words (the manager; a word it does not read). Sizes the arrays to 2
/// (`00ba3f90`). Parameter 0 (`InitConstantMap(0, 2)`, pixel constant 0 = `0101e2bc`, 0, 0, 0)
/// goes with the manager's effect picked by `011f9698` to effect 0 (output 2), input 0 is
/// texture 1 (filter 1). Parameter 1 (`InitConstantMap(0, 3)`, pixel constant 0 = 1.0 and the
/// `float`s at `01013974`, `01013d84`, `012023e0`, byte +0x56 cleared) is stored as 1, input 1
/// is textures 1, 2, 3 (filters 0, 1, 0) and effect 1 is the manager's effect at +0xcc.
pub fn fn_00b5a720(e: &mut Engine, this: Ptr, manager: u32, _unused_1: u32) {
    let base = this.addr();
    e.call(SIZE_ARRAYS_00BA3F90, &args![base, 2u32]);
    let first = new_param(e, 2);
    let scale = x87_quiet_bits(e.global::<u32>(CONSTANT_0101E2BC));
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![first, 0u32, scale, 0u32, 0u32, 0u32],
    );
    let effect = indexed_effect(e, manager);
    e.call(SET_EFFECT_00BA3C70, &args![base, 0u32, effect, first, 2u32]);
    set_at(e, base + 0x18, 0, first);
    e.call(SET_INPUT_00BA3B10, &args![base, 0u32, 1u32, 1u32]);
    let second = new_param(e, 3);
    let last = x87_quiet_bits(e.global::<u32>(CONSTANT_012023E0));
    let middle = x87_quiet_bits(e.global::<u32>(CONSTANT_01013D84));
    let thousand = x87_quiet_bits(e.global::<u32>(CONSTANT_01013974));
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![second, 0u32, ONE_BITS, thousand, middle, last],
    );
    e.mem.set_u8(second + 0x56, 0);
    set_at(e, base + 0x18, 1, second);
    e.call(
        SET_INPUTS_00BA3BA0,
        &args![base, 1u32, 1u32, 2u32, 3u32, 0u32, 1u32, 0u32],
    );
    let effect = e.mem.u32(e.mem.u32(manager + 8) + 0xcc);
    e.call(
        SET_EFFECT_00BA3C70,
        &args![base, 1u32, effect, second, 0u32],
    );
}

/// What the value destructor `00b595b0` runs: `00b58530` (unnamed; `thiscall` on the member
/// at +0x1c, no stack words), the item destructor `00bd6570` (unnamed; `thiscall`, no stack
/// words) and `NiTPointerListBase::RemoveAll` (`00b71300`; `thiscall` on the list at +0xc).
const MEMBER_RESET_00B58530: u32 = 0x00b5_8530;
const ITEM_DESTRUCT_00BD6570: u32 = 0x00bd_6570;
const LIST_REMOVE_ALL_00B71300: u32 = 0x00b7_1300;

// Translated from 00b595b0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b595b0 - the C++ exception frame (FS:[0] chain, unwind states 2 to 0 for the members) is not translated.
/// Unnamed in the engine map (`run_00b595b0`; `thiscall`, no stack words): the destructor of
/// the first map's values. Releases and clears the reference-counted pointer at +0x18, runs
/// `00b58530` on +0x1c, destroys (`00bd6570`) and frees (`MemoryManager::Deallocate`) every
/// non-null item of the pointer list at +0xc (node: next at +0, item at +8), empties the list
/// (`00b71300`); then, as the compiler's member destructors, `00b58530` on +0x1c again, the
/// pointer at +0x18 released once more without clearing it (null by then unless `00b58530`
/// set it), and the list emptied again.
pub fn fn_00b595b0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    ni_pointer_clear(e, base + 0x18);
    e.call(MEMBER_RESET_00B58530, &args![base + 0x1c]);
    let mut node = e.mem.u32(base + 0xc);
    while node != 0 {
        let item = e.mem.u32(node + 8);
        node = e.mem.u32(node);
        if item != 0 {
            e.call(ITEM_DESTRUCT_00BD6570, &args![item]);
            e.call(MEMORY_DEALLOCATE_00AA4060, &args![MEMORY_MANAGER, item]);
        }
    }
    e.call(LIST_REMOVE_ALL_00B71300, &args![base + 0xc]);
    e.call(MEMBER_RESET_00B58530, &args![base + 0x1c]);
    let pointer = e.mem.u32(base + 0x18);
    if pointer != 0 {
        release_reference(e, pointer);
    }
    e.call(LIST_REMOVE_ALL_00B71300, &args![base + 0xc]);
}

/// What `00b596a0` and `00b59ae0` call: `DistantLODShaderProperty::GetCachedGeometry`
/// (engine map, `00b587c0`, `cdecl`, one word), the first map's find `00a53c30` (unnamed;
/// `thiscall`: the key and the address of the value; a `bool` in AL) and insert `00b57ea0`
/// (unnamed; `thiscall`: key, value), `00a5f860` (unnamed; `thiscall` on the value's +0x18,
/// one word), and the batch functions `00bd6190` (unnamed constructor; `thiscall`: three
/// words), `00bd5dd0` (`thiscall`, one word), `00b58570` (`thiscall` on the value's list at
/// +0xc, one word), `00bd5d70` (`thiscall`: two words), `00bd5e40` (`thiscall`, no stack
/// words) and `00b584e0` (`thiscall` on a value's list at +0x1c, one word).
const CACHED_GEOMETRY_00B587C0: u32 = 0x00b5_87c0;
const MAP_FIND_00A53C30: u32 = 0x00a5_3c30;
const MAP_INSERT_00B57EA0: u32 = 0x00b5_7ea0;
const VALUE_SET_00A5F860: u32 = 0x00a5_f860;
const BATCH_CONSTRUCT_00BD6190: u32 = 0x00bd_6190;
const BATCH_ATTACH_00BD5DD0: u32 = 0x00bd_5dd0;
const LIST_ADD_00B58570: u32 = 0x00b5_8570;
const BATCH_ADD_00BD5D70: u32 = 0x00bd_5d70;
const BATCH_FINISH_00BD5E40: u32 = 0x00bd_5e40;
const LIST_ADD_00B584E0: u32 = 0x00b5_84e0;
/// The grid size `00b596a0` rounds positions to (`011f9654`, a signed word) and the object
/// whose slot +0xdc it calls on a new batch (`011f9644`).
const GRID_SIZE_011F9654: u32 = 0x011f_9654;
const RENDERER_011F9644: u32 = 0x011f_9644;

// Translated from 00b596a0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00b596a0 - the C++ exception frame (FS:[0] chain, unwind state 0 for the batch allocation) is not translated.
/// Unnamed in the engine map (`run_00b595b0`), `cdecl`, four words (its one caller `00b59ae0`
/// pushes a fourth it does not read). `position` holds two signed 16-bit halves; with the grid
/// size `011f9654` above 1 each half is moved to the middle of its cell (`half - h % size + h`,
/// low half kept to 16 bits, high half shifted back), else the cell is the grid size itself.
/// Clears `*out`; with no cached geometry for `key` (`00b587c0`) answers 0. Otherwise finds the
/// first map's value for the key's word +8 (`00a53c30`), or makes one (0x28 bytes, cleared;
/// `00a5f860` on its +0x18 with the geometry; the key's three words copied in; inserted with
/// `00b57ea0`). Walks the value's list at +0xc (node: next +0, item +8) for an item whose
/// 16-bit words +0xc and +0xe differ and whose word +0x24 is the cell, stored to `*out`. With
/// no such item (or a full one), makes a batch (0x28 bytes; `00bd6190(geometry, cell, 0)` on
/// it, null when the allocation failed) into `*out`, counts it in `011f963c`, runs
/// `00bd5dd0(key)` on it, adds `out` to the list (`00b58570`) and calls slot +0xdc of
/// `011f9644` with the batch's first word and 1. Answers `*out`'s 16-bit +0xc minus +0xe.
pub fn fn_00b596a0(e: &mut Engine, key: Ptr, position: u32, out: Ptr, _unused_3: u32) -> u32 {
    let grid = e.global::<u32>(GRID_SIZE_011F9654);
    let size = grid as i32;
    let cell = if size > 1 {
        let half = size / 2;
        let high = (position >> 16) as u16 as i16 as i32;
        let low = position as u16 as i16 as i32;
        let high_cell = (position >> 16).wrapping_add(half.wrapping_sub(high % size) as u32);
        let low_cell = half.wrapping_sub(low % size).wrapping_add(position as i32) as u32;
        (low_cell & 0xffff) | ((high_cell & 0xffff) << 16)
    } else {
        grid
    };
    let out = out.addr();
    e.mem.set_u32(out, 0);
    let geometry = e.call(CACHED_GEOMETRY_00B587C0, &args![key]).u32();
    if geometry == 0 {
        return 0;
    }
    let map_key = e.mem.u32(key.addr() + 8);
    let value = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let map = e.call(FIRST_MAP_00B585C0, &args![]).u32();
        let found = e.call(MAP_FIND_00A53C30, &args![map, map_key, slot]).u8() != 0;
        if found {
            return e.mem.u32(slot.addr());
        }
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x28u32])
            .u32();
        if block != 0 {
            for offset in [8u32, 4, 0, 0x14, 0xc, 0x10, 0x18, 0x24, 0x1c, 0x20] {
                e.mem.set_u32(block + offset, 0);
            }
        }
        e.call(
            VALUE_SET_00A5F860,
            &args![block.wrapping_add(0x18), geometry],
        );
        for offset in [0u32, 4, 8] {
            let word = e.mem.u32(key.addr() + offset);
            e.mem.set_u32(block.wrapping_add(offset), word);
        }
        let map_key = e.mem.u32(key.addr() + 8);
        let map = e.call(FIRST_MAP_00B585C0, &args![]).u32();
        e.call(MAP_INSERT_00B57EA0, &args![map, map_key, block]);
        block
    });
    let list = value.wrapping_add(0xc);
    let mut current = e.mem.u32(out);
    if current == 0 {
        let mut node = e.mem.u32(list);
        while node != 0 {
            let item = e.mem.u32(node + 8);
            node = e.mem.u32(node);
            if e.mem.u16(item + 0xe) != e.mem.u16(item + 0xc) && cell == e.mem.u32(item + 0x24) {
                e.mem.set_u32(out, item);
                break;
            }
            if e.mem.u32(out) != 0 {
                break;
            }
        }
        current = e.mem.u32(out);
    }
    if current == 0 || e.mem.u16(current + 0xc) == e.mem.u16(current + 0xe) {
        let block = e
            .call(MEMORY_ALLOCATE, &args![MEMORY_MANAGER, 0x28u32])
            .u32();
        let batch = if block != 0 {
            e.call(
                BATCH_CONSTRUCT_00BD6190,
                &args![block, geometry, cell, 0u32],
            )
            .u32()
        } else {
            0
        };
        e.mem.set_u32(out, batch);
        let made: u32 = e.global(CLEARED_011F963C);
        e.set_global(CLEARED_011F963C, made.wrapping_add(1));
        let batch = e.mem.u32(out);
        e.call(BATCH_ATTACH_00BD5DD0, &args![batch, key]);
        e.call(LIST_ADD_00B58570, &args![list, out]);
        let renderer = e.global::<u32>(RENDERER_011F9644);
        let first = e.mem.u32(e.mem.u32(out));
        e.vcall(renderer, 0xdc, &args![first, 1u32]);
    }
    let batch = e.mem.u32(out);
    u32::from(e.mem.u16(batch + 0xc)).wrapping_sub(u32::from(e.mem.u16(batch + 0xe)))
}

// Translated from 00b59ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`), `cdecl`, seven words. Adds the 16-bit `count`
/// to `011f9640`. While the 16-bit count left is not 0: gets a batch with room for the
/// position (`00b596a0(key, position, &batch, object)`; returns when there is none) and adds
/// up to as many points as it has room for (and as are left), each the three `float`s of
/// `points` (12 bytes apart) and the `float` of `values` (4 apart) at the running index,
/// copied through the FPU (signalling NaNs quieted) into a local passed to `00bd5d70` with
/// `item`; then `00bd5e40` on the batch. Last: `object`'s slot +0xbc; the first map's value for
/// `key`'s word +8 (`00a53c30`); when `item` is not in the value's list at +0x1c (node: next
/// +0, item +8), `00b584e0` adds it (the address of the `item` word).
pub fn fn_00b59ae0(
    e: &mut Engine,
    position: u32,
    item: u32,
    object: u32,
    key: Ptr,
    points: u32,
    values: u32,
    count: u32,
) {
    let added: u32 = e.global(CLEARED_011F9640);
    e.set_global(CLEARED_011F9640, added.wrapping_add(count & 0xffff));
    let mut left = count;
    let mut index = 0u32;
    while left as u16 != 0 {
        let (room, batch) = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), 0);
            let room = fn_00b596a0(e, key, position, slot, object);
            (room & 0xffff, e.mem.u32(slot.addr()))
        });
        if batch == 0 {
            return;
        }
        if room as i32 > 0 {
            let mut done = 0u32;
            let mut point = points.wrapping_add(index.wrapping_mul(12)).wrapping_add(8);
            while left as u16 != 0 {
                let words = [
                    x87_quiet_bits(e.mem.u32(point.wrapping_sub(8))),
                    x87_quiet_bits(e.mem.u32(point.wrapping_sub(4))),
                    x87_quiet_bits(e.mem.u32(point)),
                    x87_quiet_bits(e.mem.u32(values.wrapping_add(index.wrapping_mul(4)))),
                ];
                e.with_stack(0x10, |e, local| {
                    for (i, word) in words.iter().enumerate() {
                        e.mem.set_u32(local.addr() + 4 * i as u32, *word);
                    }
                    e.call(BATCH_ADD_00BD5D70, &args![batch, local, item]);
                });
                done += 1;
                left = left.wrapping_add(0xffff);
                index = index.wrapping_add(1);
                point = point.wrapping_add(12);
                if done as i32 >= room as i32 {
                    break;
                }
            }
        }
        e.call(BATCH_FINISH_00BD5E40, &args![batch]);
    }
    e.vcall(object, 0xbc, &args![]);
    let map_key = e.mem.u32(key.addr() + 8);
    let value = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), 0);
        let map = e.call(FIRST_MAP_00B585C0, &args![]).u32();
        e.call(MAP_FIND_00A53C30, &args![map, map_key, slot]);
        e.mem.u32(slot.addr())
    });
    let list = value.wrapping_add(0x1c);
    let mut node = e.mem.u32(list);
    while node != 0 {
        if item == e.mem.u32(node + 8) {
            return;
        }
        node = e.mem.u32(node);
    }
    e.with_stack(4, |e, word| {
        e.mem.set_u32(word.addr(), item);
        e.call(LIST_ADD_00B584E0, &args![list, word]);
    });
}

/// What `00b59d30` reads: the object at `011f9684` (its 4x4 `float` matrix at +0x9c, copied
/// with `memcpy` `00ec44d0`, and four `float`s at +0x100 to +0x10c), the `float`s -1.0
/// (`01012054`), 0.5 (`01016248`) and 0.3 (`01017870`), the `double` 0.5 (`01011588`) and the
/// CRT square root `00ec6040` (argument and result in ST0).
const VIEW_011F9684: u32 = 0x011f_9684;
const MEMCPY_00EC44D0: u32 = 0x00ec_44d0;
const MINUS_ONE_01012054: u32 = 0x0101_2054;
const HALF_DOUBLE_01011588: u32 = 0x0101_1588;
const SCALE_01017870: u32 = 0x0101_7870;
const SQRT_00EC6040: u32 = 0x00ec_6040;

// Translated from 00b59d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`), `cdecl`, three words: projects the point
/// `point` (x, y, z, and a fourth `float` w) with the matrix of `011f9684` (rows of four at
/// +0x9c: s = (row 0 . p) / |row 3 . p|, t = (row 1 . p) / |row 3 . p|; the x87 operations in
/// the exe's order, stores to `float` where it stores). When (s, t) is inside [-1, 1] on both
/// axes (false for NaN), or the depth (row 3 . p) is not below 0, projects the point moved by w
/// along z as well and stores half the distance between the two projections (s, 1 - t), at
/// most 0.5, to `*scale`; else stores 0.3 there. `out` gets
/// `((a + b) / 2 + s (b - a) / 2, 1 - ((c + d) / 2 + t (c - d) / 2))` with a to d the four
/// `float`s at +0x100, each clamped to [0, 1]; with the depth below 0 the first is 0 when it is
/// at most 0 and 1 when above 0.
pub fn fn_00b59d30(e: &mut Engine, point: Ptr, out: Ptr, scale: Ptr) {
    let p = point.addr();
    let x_bits = e.mem.u32(p);
    let y_bits = e.mem.u32(p + 4);
    let z_bits = e.mem.u32(p + 8);
    let view = e.global::<u32>(VIEW_011F9684);
    let a_bits = e.mem.u32(view + 0x100);
    let b_bits = e.mem.u32(view + 0x104);
    let c_bits = e.mem.u32(view + 0x108);
    let d_bits = e.mem.u32(view + 0x10c);
    let m: Vec<u32> = e.with_stack(0x40, |e, local| {
        e.call(MEMCPY_00EC44D0, &args![local, view + 0x9c, 0x40u32]);
        (0..16).map(|i| e.mem.u32(local.addr() + 4 * i)).collect()
    });
    let ld = x87_load32;
    let y = ld(y_bits);
    let m13y = x87_mul(ld(m[13]), y);
    let x = ld(x_bits);
    let m12x = x87_mul(ld(m[12]), x);
    let depth_xy = x87_add(m12x, m13y);
    let z = ld(z_bits);
    let m14z = x87_mul(ld(m[14]), z);
    let depth = x87_add(x87_add(depth_xy, m14z), ld(m[15]));
    let depth_bits = x87_store32(depth);
    let below = ld(depth_bits) < 0.0;
    let abs_bits = x87_store32(ld(depth_bits).abs());
    let inverse_bits = x87_store32(x87_div(1.0, ld(abs_bits)));
    let inverse = ld(inverse_bits);
    let row0_xy = x87_add(x87_mul(ld(m[1]), y), x87_mul(ld(m[0]), x));
    let row0 = x87_add(x87_add(x87_mul(ld(m[2]), z), row0_xy), x87_m32(m[3]));
    let row0_bits = x87_store32(row0);
    let s_bits = x87_store32(x87_mul(ld(row0_bits), inverse));
    let m5y = x87_mul(y, ld(m[5]));
    let m4x = x87_mul(x, ld(m[4]));
    let row1_xy = x87_add(m4x, m5y);
    let row1 = x87_add(x87_add(x87_mul(ld(m[6]), z), row1_xy), x87_m32(m[7]));
    let row1_bits = x87_store32(row1);
    let t_bits = x87_store32(x87_mul(inverse, ld(row1_bits)));
    let minus_one = ld(e.global::<u32>(MINUS_ONE_01012054));
    let s = ld(s_bits);
    let t = ld(t_bits);
    let half = x87_load64(e.global::<u64>(HALF_DOUBLE_01011588));
    let inside = s >= minus_one && 1.0 >= s && minus_one <= t && 1.0 >= t;
    if inside || !below {
        let z2_bits = x87_store32(x87_add(z, ld(e.mem.u32(p + 0xc))));
        let z2 = ld(z2_bits);
        let depth2 = x87_add(x87_mul(ld(m[14]), z2), depth_xy);
        let depth2_bits = x87_store32(x87_add(depth2, x87_m32(m[15])));
        let q_bits = x87_store32(x87_sub(1.0, t));
        let row0_2 = x87_add(row0_xy, x87_mul(ld(m[2]), z2));
        let row0_2_bits = x87_store32(x87_add(row0_2, ld(m[3])));
        let depth2 = ld(depth2_bits);
        let s2_bits = x87_store32(x87_div(ld(row0_2_bits), depth2));
        let row1_2 = x87_add(row1_xy, x87_mul(z2, ld(m[6])));
        let row1_2_bits = x87_store32(x87_add(row1_2, ld(m[7])));
        let t2_bits = x87_store32(x87_div(ld(row1_2_bits), depth2));
        let q2_bits = x87_store32(x87_sub(1.0, ld(t2_bits)));
        let ds_bits = x87_store32(x87_sub(s, x87_m32(s2_bits)));
        let dq_bits = x87_store32(x87_sub(ld(q_bits), x87_m32(q2_bits)));
        let dq = ld(dq_bits);
        let ds = ld(ds_bits);
        let sum_bits = x87_store32(x87_add(x87_mul(dq, dq), x87_mul(ds, ds)));
        let root = e.call(SQRT_00EC6040, &args![ld(sum_bits)]).f64();
        let root_bits = x87_store32(root);
        let scaled_bits = x87_store32(x87_mul(ld(root_bits), half));
        let scaled = ld(scaled_bits);
        e.mem.set_u32(scale.addr(), x87_store32(scaled));
        let limit = ld(e.global::<u32>(HALF_01016248));
        if limit < scaled {
            e.mem.set_u32(scale.addr(), x87_store32(limit));
        }
    } else {
        let fallback = ld(e.global::<u32>(SCALE_01017870));
        e.mem.set_u32(scale.addr(), x87_store32(fallback));
    }
    let a = ld(a_bits);
    let b = ld(b_bits);
    let x1_bits = x87_store32(x87_mul(s, x87_mul(x87_sub(b, a), half)));
    let u = x87_add(ld(x1_bits), x87_mul(x87_add(b, a), half));
    let mut u_bits = x87_store32(u);
    let c = ld(c_bits);
    let d = ld(d_bits);
    let y1_bits = x87_store32(x87_mul(t, x87_mul(x87_sub(c, d), half)));
    let v = x87_add(x87_mul(half, x87_add(c, d)), ld(y1_bits));
    let v_bits = x87_store32(v);
    let mut q_bits = x87_store32(x87_sub(1.0, x87_m32(v_bits)));
    let u = ld(u_bits);
    let clamped_u = if u > 1.0 {
        u_bits = x87_store32(1.0);
        1.0
    } else if 0.0 > u {
        u_bits = x87_store32(0.0);
        0.0
    } else {
        u
    };
    let q = ld(q_bits);
    if q > 1.0 {
        q_bits = x87_store32(1.0);
    } else if q < 0.0 {
        q_bits = x87_store32(0.0);
    }
    let first = if below {
        if clamped_u <= 0.0 {
            x87_store32(0.0)
        } else if clamped_u > 0.0 {
            x87_store32(1.0)
        } else {
            u_bits
        }
    } else {
        u_bits
    };
    e.mem.set_u32(out.addr(), first);
    e.mem.set_u32(out.addr() + 4, q_bits);
}

/// `ImageSpaceShaderParam::SetPixelConstant`'s callers here pass the floats of `011f917c`
/// (+0xec, +0xf0: two `float`s of the object there).
const CAMERA_011F917C: u32 = 0x011f_917c;

/// `ImageSpaceShaderParam::SetPixelConstant(index, ...)` on `param` with the four `float`s at
/// `object + offsets` (each through `FLD`/`FSTP`, a signalling NaN quieted).
fn set_constant_from(e: &mut Engine, param: u32, index: u32, object: u32, offsets: [u32; 4]) {
    let words = offsets.map(|offset| x87_quiet_bits(e.mem.u32(object + offset)));
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![param, index, words[0], words[1], words[2], words[3]],
    );
}

// Translated from 00b5a220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`): the `UpdateParams` slot of the effect
/// `00b5a590` builds, `thiscall`, one word it does not read. With n and f the `float`s +0xec
/// and +0xf0 of the object at `011f917c`, stores n to +0x60, f - n to +0x64 and n * f to
/// +0x68; effect parameter 1 takes pixel constants 0 (+0x7c to +0x88), 1 (+0x6c to +0x78) and 2
/// (+0x5c to +0x68); parameter 0 takes constant 0 (+0x8c, +0x90, +0x94, 0.0) and constant 1
/// (0.5, 0.5, 0.0, 0.0; the 0.5 from `01016248`). Answers true.
pub fn fn_00b5a220(e: &mut Engine, this: Ptr, _unused_0: u32) -> bool {
    let base = this.addr();
    let second = e.mem.u32(e.mem.u32(base + 0x1c) + 4);
    let camera = e.global::<u32>(CAMERA_011F917C);
    let near_bits = x87_quiet_bits(e.mem.u32(camera + 0xec));
    let far_bits = x87_quiet_bits(e.mem.u32(camera + 0xf0));
    let near = x87_load32(near_bits);
    let far = x87_load32(far_bits);
    e.mem.set_u32(base + 0x60, x87_store32(near));
    e.mem.set_u32(base + 0x64, x87_store32(x87_sub(far, near)));
    e.mem.set_u32(base + 0x68, x87_store32(x87_mul(near, far)));
    set_constant_from(e, second, 0, base, [0x7c, 0x80, 0x84, 0x88]);
    set_constant_from(e, second, 1, base, [0x6c, 0x70, 0x74, 0x78]);
    set_constant_from(e, second, 2, base, [0x5c, 0x60, 0x64, 0x68]);
    let first = e.mem.u32(e.mem.u32(base + 0x1c));
    let words = [0x8cu32, 0x90, 0x94].map(|offset| x87_quiet_bits(e.mem.u32(base + offset)));
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![first, 0u32, words[0], words[1], words[2], 0u32],
    );
    let half = x87_quiet_bits(e.global::<u32>(HALF_01016248));
    e.call(
        PARAM_SET_PIXEL_CONSTANT_00B8A790,
        &args![first, 1u32, half, half, 0u32, 0u32],
    );
    true
}

/// `ImageSpaceTexture::BorrowRenderedTexture` (engine map, `00ba3840`; `thiscall`, one word),
/// `ImageSpaceTexture::ReturnRenderedTexture` (engine map, `00ba39a0`; `thiscall`, no stack
/// words), the texture setter `00ba37f0` (unit `imagespacetexture.cpp`, unnamed; `thiscall`,
/// one word) and `00b54090` (unit `bsshadermanager.cpp`, unnamed, no arguments: a texture of
/// the shader manager).
const BORROW_RENDERED_TEXTURE_00BA3840: u32 = 0x00ba_3840;
const RETURN_RENDERED_TEXTURE_00BA39A0: u32 = 0x00ba_39a0;
const TEXTURE_SET_00BA37F0: u32 = 0x00ba_37f0;
const MANAGER_TEXTURE_00B54090: u32 = 0x00b5_4090;

// Translated from 00b5a340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`; Ghidra made no function of it): slot 4 of the
/// vtable `010adc68` (the effect `00b5a590` builds), `thiscall`, one word it does not read.
/// Borrows a rendered texture of kind 0x28 into the texture at +0xb4 (`00ba3840`) and makes it
/// texture 2 of the array at +0x28; sets the texture at +0xc4 from `00b54090` (`00ba37f0`) and
/// makes it texture 3.
pub fn fn_00b5a340(e: &mut Engine, this: Ptr, _unused_0: u32) {
    let base = this.addr();
    e.call(
        BORROW_RENDERED_TEXTURE_00BA3840,
        &args![base + 0xb4, 0x28u32],
    );
    set_at(e, base + 0x28, 2, base + 0xb4);
    let texture = e.call(MANAGER_TEXTURE_00B54090, &args![]).u32();
    e.call(TEXTURE_SET_00BA37F0, &args![base + 0xc4, texture]);
    set_at(e, base + 0x28, 3, base + 0xc4);
}

// Translated from 00b5a3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`; Ghidra made no function of it): slot 5 of the
/// vtable `010adc68`, `thiscall`, no stack words. Returns texture 2 of `pTextures` (+0x2c
/// elements) (`00ba39a0`) and sets textures 2 and 3 of the array at +0x28 to null.
pub fn fn_00b5a3a0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    let texture = e.mem.u32(e.mem.u32(base + 0x2c) + 8);
    e.call(RETURN_RENDERED_TEXTURE_00BA39A0, &args![texture]);
    set_at(e, base + 0x28, 2, 0);
    set_at(e, base + 0x28, 3, 0);
}

// Translated from 00b5a8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`run_00b595b0`): an empty `thiscall` (`RET 0x18`) with six stack
/// words it does not read, in the vtables at `010ec018`, `010ec0f8`, `010ec188` and `010cc8a8`
/// and called from `005bb8c0`.
pub fn fn_00b5a8c0(
    _e: &mut Engine,
    _this: Ptr,
    _unused_0: u32,
    _unused_1: u32,
    _unused_2: u32,
    _unused_3: u32,
    _unused_4: u32,
    _unused_5: u32,
) {
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00b5_96a0, fn_00b596a0(Ptr, u32, Ptr, u32) -> u32),
        entry!(0x00b5_9ae0, fn_00b59ae0(u32, u32, u32, Ptr, u32, u32, u32)),
        entry!(0x00b5_9d30, fn_00b59d30(Ptr, Ptr, Ptr)),
        entry!(0x00b5_a220, fn_00b5a220(Ptr, u32) -> bool),
        entry!(0x00b5_a340, fn_00b5a340(Ptr, u32)),
        entry!(0x00b5_a3a0, fn_00b5a3a0(Ptr)),
        entry!(0x00b5_a8c0, fn_00b5a8c0(Ptr, u32, u32, u32, u32, u32, u32)),
        entry!(0x00b5_a410, fn_00b5a410(Ptr) -> Ptr),
        entry!(0x00b5_a590, fn_00b5a590(Ptr) -> Ptr),
        entry!(0x00b5_9890, fn_00b59890()),
        entry!(0x00b5_9940, fn_00b59940()),
        entry!(0x00b5_9c20, fn_00b59c20(Ptr)),
        entry!(0x00b5_9cb0, fn_00b59cb0(Ptr, u32) -> bool),
        entry!(0x00b5_9d10, fn_00b59d10(Ptr) -> u32),
        entry!(0x00b5_a110, fn_00b5a110(Ptr)),
        entry!(0x00b5_a3f0, fn_00b5a3f0(Ptr) -> u32),
        entry!(0x00b5_a480, fn_00b5a480(Ptr, u32) -> Ptr),
        entry!(0x00b5_a4a0, fn_00b5a4a0(Ptr, u32, u32)),
        entry!(0x00b5_a700, fn_00b5a700(Ptr, u32) -> Ptr),
        entry!(0x00b5_a720, fn_00b5a720(Ptr, u32, u32)),
        entry!(0x00b5_95b0, fn_00b595b0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An engine with the base constructor, the set-size and the vector constructor doubled
    /// and the constants mapped (a signalling NaN at `012023e0`).
    fn engine() -> Engine {
        let mut e = Engine::new();
        for target in [
            EFFECT_CONSTRUCT_00BA4080,
            ARRAY_SET_SIZE_00A90180,
            VECTOR_CONSTRUCT_00EC782F,
        ] {
            e.register(target, |_, a| a[0].into_ret());
        }
        for page in [0x0101_6000u32, 0x0101_3000, 0x0120_2000, 0x0101_7000] {
            e.map(page, 0x1000);
        }
        e.set_global(HALF_01016248, 0.5f32);
        e.set_global(CONSTANT_01013974, 1000.0f32);
        e.set_global(CONSTANT_012023E0, 0x7f80_0001u32);
        e.set_global(CONSTANT_0101786C, 600.0f32);
        e.call_log = Some(vec![]);
        e
    }

    fn log(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap().into_iter().skip(1).collect()
    }

    #[test]
    fn fn_00b5a410_builds_the_effect() {
        let mut e = engine();
        e.set_global(HALF_01016248, 0x7fa0_0000u32);
        let this = e.mem.alloc(0x74);
        assert_eq!(e.call(0x00b5_a410, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010ADC48);
        assert_eq!(e.mem.u32(this + 0x6c), 0x7fe0_0000);
        assert_eq!(e.mem.u32(this + 0x70), 0x7fe0_0000);
        assert_eq!(
            log(&mut e),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (ARRAY_SET_SIZE_00A90180, vec![this + 0x28, 2]),
            ]
        );
    }

    #[test]
    fn fn_00b5a590_builds_the_effect() {
        let mut e = engine();
        let this = e.mem.alloc(0xd4);
        e.mem.write(this, &[0xcc; 0xd4]);
        assert_eq!(e.call(0x00b5_a590, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), VTABLE_010ADC68);
        let floats = |e: &Engine, offset: u32| e.mem.u32(this + offset);
        assert_eq!(floats(&e, 0x58), 0);
        assert_eq!(floats(&e, 0x5c), 0);
        assert_eq!(floats(&e, 0x6c), 0);
        assert_eq!(floats(&e, 0x70), ONE_BITS);
        assert_eq!(floats(&e, 0x74), ONE_BITS);
        assert_eq!(floats(&e, 0x78), 0);
        assert_eq!(floats(&e, 0x7c), ONE_BITS);
        assert_eq!(floats(&e, 0x80), 1000.0f32.to_bits());
        assert_eq!(floats(&e, 0x84), 0);
        assert_eq!(floats(&e, 0x88), 0x7fc0_0001);
        assert_eq!(floats(&e, 0x8c), 0);
        assert_eq!(floats(&e, 0x98), 0);
        assert_eq!(floats(&e, 0x9c), 600.0f32.to_bits());
        assert_eq!(floats(&e, 0xa0), 0xcccc_cccc);
        assert_eq!(e.mem.bytes(this + 0xa4, 3), vec![0; 3]);
        assert_eq!(e.mem.u8(this + 0xa7), 0xcc);
        assert_eq!(e.mem.bytes(this + 0xa8, 0xc), vec![0; 0xc]);
        assert_eq!(
            log(&mut e),
            vec![
                (EFFECT_CONSTRUCT_00BA4080, vec![this]),
                (
                    VECTOR_CONSTRUCT_00EC782F,
                    vec![
                        this + 0xb4,
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
mod map_tests {
    use super::*;
    use crate::units::bsshader::geometrydecalshader::shader_test_rig::*;

    /// A map of `size` buckets with heads at the given indexes (heads are just non-null words)
    /// and the count `count`.
    fn map(e: &mut Engine, size: u32, heads: &[(u32, u32)], count: u32) -> u32 {
        let map = e.mem.alloc(0x10);
        let buckets = e.mem.alloc(4 * size.max(1));
        for (index, head) in heads {
            e.mem.set_u32(buckets + 4 * index, *head);
        }
        e.mem.set_u32(map + 4, size);
        e.mem.set_u32(map + 8, buckets);
        e.mem.set_u32(map + 0xc, count);
        map
    }

    /// The next-item double: hands out `values` in turn, the position null with the last.
    fn walker(e: &mut Engine, next: u32, values: Vec<u32>) {
        let mut index = 0usize;
        e.register_double(next, move |e, a| {
            e.mem.set_u32(a[3], values[index]);
            index += 1;
            let position = if index < values.len() { 1 } else { 0 };
            e.mem.set_u32(a[1], position);
            Ret::default()
        });
    }

    fn first_rig(heads: &[(u32, u32)], count: u32, values: Vec<u32>) -> (Engine, u32) {
        let mut e = rig();
        e.map(0x011f_9000, 0x1000);
        e.set_global(CLEARED_011F963C, 7u32);
        e.set_global(CLEARED_011F9640, 8u32);
        let first = map(&mut e, 4, heads, count);
        e.register_double(FIRST_MAP_00B585C0, move |_, _| first.into_ret());
        walker(&mut e, FIRST_MAP_STEP_00B57F20, values);
        for callee in [
            MAP_AFTER_WALK_00B601A0,
            VALUE_DESTRUCT_00B595B0,
            MEMORY_DEALLOCATE_00AA4060,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        (e, first)
    }

    #[test]
    fn fn_00b59890_destroys_and_frees_every_value() {
        let (mut e, first) = first_rig(&[(2, 0x99)], 3, vec![0x1000, 0, 0x3000]);
        e.call(0x00b5_9890, &args![]);
        let walks = calls(&e, FIRST_MAP_STEP_00B57F20);
        assert_eq!(walks.len(), 3);
        assert_eq!(walks[0][0], first);
        assert_eq!(walks[0][1], walks[0][3] + 4);
        assert_eq!(walks[0][2], walks[0][3] + 8);
        assert_eq!(
            calls(&e, VALUE_DESTRUCT_00B595B0),
            vec![vec![0x1000], vec![0x3000]]
        );
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, 0x1000], vec![MEMORY_MANAGER, 0x3000]]
        );
        assert_eq!(calls(&e, MAP_AFTER_WALK_00B601A0), vec![vec![first]]);
        assert_eq!(e.global::<u32>(CLEARED_011F963C), 0);
        assert_eq!(e.global::<u32>(CLEARED_011F9640), 0);
    }

    #[test]
    fn fn_00b59890_skips_an_empty_map_but_still_finishes() {
        let (mut e, first) = first_rig(&[], 5, vec![]);
        e.call(0x00b5_9890, &args![]);
        assert!(calls(&e, FIRST_MAP_STEP_00B57F20).is_empty());
        assert_eq!(calls(&e, MAP_AFTER_WALK_00B601A0), vec![vec![first]]);
        let (mut e, _) = first_rig(&[(0, 0x99)], 0, vec![]);
        e.call(0x00b5_9890, &args![]);
        assert!(calls(&e, FIRST_MAP_STEP_00B57F20).is_empty());
        assert_eq!(calls(&e, MAP_AFTER_WALK_00B601A0).len(), 1);
    }

    /// The second map's walk over `values` (after an empty first map).
    fn second_rig(heads: &[(u32, u32)], count: u32, values: Vec<u32>) -> (Engine, u32) {
        let (mut e, _) = first_rig(&[], 0, vec![]);
        let second = map(&mut e, 2, heads, count);
        e.register_double(SECOND_MAP_00B58670, move |_, _| second.into_ret());
        walker(&mut e, SECOND_MAP_STEP_00E6E630, values);
        (e, second)
    }

    #[test]
    fn fn_00b59940_clears_each_value_and_releases_the_last() {
        let (mut e, second) = second_rig(&[(1, 0x99)], 1, vec![]);
        let value = object(&mut e, 1);
        let members: Vec<u32> = (0..5).map(|_| object(&mut e, 1)).collect();
        for (offset, member) in [0x08u32, 0x18, 0x20, 0x1c, 0x24].iter().zip(&members) {
            e.mem.set_u32(value + offset, *member);
        }
        e.mem.set_u32(value + 0x10, 0x5555);
        walker(&mut e, SECOND_MAP_STEP_00E6E630, vec![value]);
        e.call(0x00b5_9940, &args![]);
        // 00b59890 ran first (its map's after-walk), then this map's.
        assert_eq!(calls(&e, MAP_AFTER_WALK_00B601A0).len(), 2);
        assert_eq!(calls(&e, MAP_AFTER_WALK_00B601A0)[1], vec![second]);
        let deleted = calls(&e, REFERENCE_DELETE);
        let expected: Vec<Vec<u32>> = [0usize, 1, 2, 3, 4]
            .iter()
            .map(|i| vec![members[*i]])
            .chain([vec![value]])
            .collect();
        assert_eq!(deleted, expected);
        for offset in [0x08u32, 0x10, 0x18, 0x1c, 0x20, 0x24] {
            assert_eq!(e.mem.u32(value + offset), 0);
        }
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, 0x5555]]
        );
    }

    #[test]
    fn fn_00b59940_with_no_items_skips_the_after_walk() {
        let (mut e, _) = second_rig(&[(0, 0x99)], 0, vec![]);
        e.call(0x00b5_9940, &args![]);
        assert!(calls(&e, SECOND_MAP_STEP_00E6E630).is_empty());
        assert_eq!(calls(&e, MAP_AFTER_WALK_00B601A0).len(), 1);
        // Items but no first position: the after-walk runs.
        let (mut e, second) = second_rig(&[], 2, vec![]);
        e.call(0x00b5_9940, &args![]);
        assert_eq!(calls(&e, MAP_AFTER_WALK_00B601A0)[1], vec![second]);
    }

    const PARAM_DELETE: u32 = 0x0e0a_0000;
    const PARAM_VTABLE: u32 = 0x0e0b_0000;

    /// An effect whose parameters 0 and 1 are deletable objects (when `params` is true).
    fn effect_rig(params: bool) -> (Engine, u32, [u32; 2]) {
        let mut e = rig();
        for callee in [
            ARRAY_SET_AT_00BA3AA0,
            EFFECT_DESTRUCT_00BA3D30,
            MEMORY_DEALLOCATE_00AA4060,
            TEXTURE_DESTRUCT_00BA3A00,
            VECTOR_DESTRUCT_00EC5FCE,
            PARAM_DELETE,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.put_vtable(PARAM_VTABLE, &[PARAM_DELETE]);
        let this = e.mem.alloc(0xd4);
        let list = e.mem.alloc(8);
        let mut made = [0u32; 2];
        if params {
            for (index, slot) in made.iter_mut().enumerate() {
                let param = e.mem.alloc(8);
                e.mem.set_u32(param, PARAM_VTABLE);
                e.mem.set_u32(list + 4 * index as u32, param);
                *slot = param;
            }
        }
        e.mem.set_u32(this + 0x1c, list);
        let textures = e.mem.alloc(0xc);
        e.mem.set_u32(this + 0x2c, textures);
        (e, this, made)
    }

    fn set_ats(e: &Engine) -> Vec<(u32, u32)> {
        calls(e, ARRAY_SET_AT_00BA3AA0)
            .iter()
            .map(|c| (c[0], c[1]))
            .collect()
    }

    #[test]
    fn fn_00b59c20_and_00b5a480_delete_parameter_0() {
        let (mut e, this, params) = effect_rig(true);
        e.call(0x00b5_9c20, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010ADC48);
        assert_eq!(calls(&e, PARAM_DELETE), vec![vec![params[0], 1]]);
        assert_eq!(set_ats(&e), vec![(this + 0x18, 0)]);
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        let (mut e, this, _) = effect_rig(false);
        assert_eq!(e.call(0x00b5_a480, &args![this, 1u32]).u32(), this);
        assert!(calls(&e, ARRAY_SET_AT_00BA3AA0).is_empty());
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, this]]
        );
    }

    #[test]
    fn fn_00b5a110_and_00b5a700_free_the_texture_and_both_parameters() {
        let (mut e, this, params) = effect_rig(true);
        let texture = e.mem.alloc(0x10);
        let textures = e.mem.u32(this + 0x2c);
        e.mem.set_u32(textures + 8, texture);
        e.call(0x00b5_a110, &args![this]);
        assert_eq!(e.mem.u32(this), VTABLE_010ADC68);
        assert_eq!(
            calls(&e, TEXTURE_DESTRUCT_00BA3A00),
            vec![vec![texture], vec![this + 0xa4]]
        );
        assert_eq!(
            calls(&e, PARAM_DELETE),
            vec![vec![params[1], 1], vec![params[0], 1]]
        );
        assert_eq!(
            set_ats(&e),
            vec![(this + 0x28, 2), (this + 0x18, 1), (this + 0x18, 0)]
        );
        assert_eq!(
            calls(&e, VECTOR_DESTRUCT_00EC5FCE),
            vec![vec![this + 0xb4, 0x10, 2, TEXTURE_DESTRUCT_00BA3A00]]
        );
        assert_eq!(calls(&e, EFFECT_DESTRUCT_00BA3D30), vec![vec![this]]);
        let (mut e, this, _) = effect_rig(false);
        assert_eq!(e.call(0x00b5_a700, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, MEMORY_DEALLOCATE_00AA4060).is_empty());
        assert!(calls(&e, ARRAY_SET_AT_00BA3AA0).is_empty());
    }

    #[test]
    fn fn_00b59cb0_sets_both_constants() {
        let mut e = rig();
        e.register(PARAM_SET_PIXEL_CONSTANT_00B8A790, |_, _| Ret::default());
        let this = e.mem.alloc(0x74);
        let list = e.mem.alloc(4);
        e.mem.set_u32(list, 0x0e0c_0000);
        e.mem.set_u32(this + 0x1c, list);
        for (index, offset) in (0x58u32..=0x70).step_by(4).enumerate() {
            e.mem.set_f32(this + offset, index as f32 + 1.0);
        }
        e.mem.set_u32(this + 0x70, 0x7f80_0003);
        assert!(e.call(0x00b5_9cb0, &args![this, 0u32]).bool());
        let bits = |v: f32| v.to_bits();
        assert_eq!(
            calls(&e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![
                vec![0x0e0c_0000, 0, bits(1.0), bits(2.0), bits(3.0), bits(4.0)],
                vec![0x0e0c_0000, 1, bits(6.0), 0x7fc0_0003, bits(5.0), 0],
            ]
        );
    }

    #[test]
    fn fn_00b59d10_and_00b5a3f0_need_a_positive_value_and_the_byte() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        let this = e.mem.alloc(0x74);
        e.mem.set_f32(this + 0x58, 0.5);
        e.mem.set_f32(this + 0x6c, 0.5);
        assert_eq!(e.call(0x00b5_9d10, &args![this]).u32(), 0);
        e.set_global(ENABLED_011F968C, 1u8);
        assert_eq!(e.call(0x00b5_9d10, &args![this]).u32(), 1);
        assert_eq!(e.call(0x00b5_a3f0, &args![this]).u32(), 1);
        for value in [0.0f32, -1.0, f32::NAN] {
            e.mem.set_f32(this + 0x58, value);
            e.mem.set_f32(this + 0x6c, value);
            assert_eq!(e.call(0x00b5_9d10, &args![this]).u32(), 0);
            assert_eq!(e.call(0x00b5_a3f0, &args![this]).u32(), 0);
        }
    }

    /// The set-ups' rig: parameters from 0x0e0d_0100 in steps of 0x100, the constants and the
    /// manager with effects at +0xd8 (index 2 picked) and +0xcc.
    fn setup_rig() -> (Engine, u32, u32) {
        let mut e = rig();
        for page in [
            0x0101_e000u32,
            0x0101_3000,
            0x0120_2000,
            0x011f_9000,
            0x0e0d_0000,
        ] {
            e.map(page, 0x1000);
        }
        e.mem.write(0x0e0d_0000, &[0xcc; 0x1000]);
        e.set_global(CONSTANT_0101E2BC, 0.1f32);
        e.set_global(CONSTANT_01013974, 1000.0f32);
        e.set_global(CONSTANT_01013D84, 0.25f32);
        e.set_global(CONSTANT_012023E0, 0x7f80_0001u32);
        e.set_global(EFFECT_INDEX_011F9698, 2u32);
        let mut next = 0x0e0d_0000u32;
        e.register_double(PARAM_CONSTRUCT_00B8A9E0, move |_, _| {
            next += 0x100;
            next.into_ret()
        });
        for callee in [
            SIZE_ARRAYS_00BA3F90,
            PARAM_INIT_CONSTANT_MAP_00B8AB10,
            PARAM_SET_PIXEL_CONSTANT_00B8A790,
            SET_EFFECT_00BA3C70,
            ARRAY_SET_AT_00BA3AA0,
            SET_INPUT_00BA3B10,
            SET_INPUTS_00BA3BA0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xd4);
        let manager = e.mem.alloc(0x10);
        let effects = e.mem.alloc(0x100);
        e.mem.set_u32(effects + 0xd8 + 8, 0x0e0e_0000);
        e.mem.set_u32(effects + 0xcc, 0x0e0f_0000);
        e.mem.set_u32(manager + 8, effects);
        (e, this, manager)
    }

    #[test]
    fn fn_00b5a4a0_sets_up_one_parameter() {
        let (mut e, this, manager) = setup_rig();
        e.call(0x00b5_a4a0, &args![this, manager, 0u32]);
        let param = 0x0e0d_0100;
        assert_eq!(calls(&e, SIZE_ARRAYS_00BA3F90), vec![vec![this, 1]]);
        assert_eq!(
            calls(&e, PARAM_INIT_CONSTANT_MAP_00B8AB10),
            vec![vec![param, 0, 2]]
        );
        assert_eq!(
            calls(&e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![vec![param, 0, 0.1f32.to_bits(), 0, 0, 0]]
        );
        assert_eq!(
            calls(&e, SET_EFFECT_00BA3C70),
            vec![vec![this, 0, 0x0e0e_0000, param, 0]]
        );
        assert_eq!(set_ats(&e), vec![(this + 0x18, 0)]);
        assert_eq!(calls(&e, SET_INPUT_00BA3B10), vec![vec![this, 0, 1, 0]]);
    }

    #[test]
    fn fn_00b5a720_sets_up_two_parameters() {
        let (mut e, this, manager) = setup_rig();
        e.call(0x00b5_a720, &args![this, manager, 0u32]);
        let (first, second) = (0x0e0d_0100, 0x0e0d_0200);
        assert_eq!(calls(&e, SIZE_ARRAYS_00BA3F90), vec![vec![this, 2]]);
        assert_eq!(
            calls(&e, PARAM_INIT_CONSTANT_MAP_00B8AB10),
            vec![vec![first, 0, 2], vec![second, 0, 3]]
        );
        assert_eq!(
            calls(&e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![
                vec![first, 0, 0.1f32.to_bits(), 0, 0, 0],
                vec![
                    second,
                    0,
                    ONE_BITS,
                    1000.0f32.to_bits(),
                    0.25f32.to_bits(),
                    0x7fc0_0001
                ],
            ]
        );
        assert_eq!(
            calls(&e, SET_EFFECT_00BA3C70),
            vec![
                vec![this, 0, 0x0e0e_0000, first, 2],
                vec![this, 1, 0x0e0f_0000, second, 0],
            ]
        );
        assert_eq!(set_ats(&e), vec![(this + 0x18, 0), (this + 0x18, 1)]);
        assert_eq!(calls(&e, SET_INPUT_00BA3B10), vec![vec![this, 0, 1, 1]]);
        assert_eq!(
            calls(&e, SET_INPUTS_00BA3BA0),
            vec![vec![this, 1, 1, 2, 3, 0, 1, 0]]
        );
        assert_eq!(e.mem.u8(second + 0x56), 0);
        assert_eq!(e.mem.u8(second + 0x55), 0xcc);
        assert_eq!(e.mem.u8(first + 0x56), 0xcc);
    }

    #[test]
    fn value_destructor_releases_frees_and_empties() {
        let mut e = rig();
        for callee in [
            MEMBER_RESET_00B58530,
            ITEM_DESTRUCT_00BD6570,
            LIST_REMOVE_ALL_00B71300,
            MEMORY_DEALLOCATE_00AA4060,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x20);
        let pointer = object(&mut e, 1);
        e.mem.set_u32(this + 0x18, pointer);
        // Two nodes: the first with item 0x5000, the second with a null item.
        let second = e.mem.alloc(0xc);
        let first = e.mem.alloc(0xc);
        e.mem.set_u32(first, second);
        e.mem.set_u32(first + 8, 0x5000);
        e.mem.set_u32(this + 0xc, first);
        e.call(0x00b5_95b0, &args![this]);
        assert_eq!(e.mem.u32(this + 0x18), 0);
        assert_eq!(calls(&e, REFERENCE_DELETE), vec![vec![pointer]]);
        assert_eq!(calls(&e, ITEM_DESTRUCT_00BD6570), vec![vec![0x5000]]);
        assert_eq!(
            calls(&e, MEMORY_DEALLOCATE_00AA4060),
            vec![vec![MEMORY_MANAGER, 0x5000]]
        );
        assert_eq!(calls(&e, MEMBER_RESET_00B58530), vec![vec![this + 0x1c]; 2]);
        assert_eq!(
            calls(&e, LIST_REMOVE_ALL_00B71300),
            vec![vec![this + 0xc]; 2]
        );
    }

    #[test]
    fn value_destructor_releases_a_pointer_the_member_reset_set() {
        let mut e = rig();
        let later = object(&mut e, 2);
        e.register_double(MEMBER_RESET_00B58530, move |e, a| {
            // The pointer at +0x18 (the member is at +0x1c).
            e.mem.set_u32(a[0] - 4, later);
            Ret::default()
        });
        e.register(LIST_REMOVE_ALL_00B71300, |_, _| Ret::default());
        let this = e.mem.alloc(0x20);
        e.call(0x00b5_95b0, &args![this]);
        // Released once by the end (count 2 to 1), not cleared.
        assert_eq!(e.mem.u32(later + 4), 1);
        assert_eq!(e.mem.u32(this + 0x18), later);
        assert!(calls(&e, REFERENCE_DELETE).is_empty());
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

    const MAP: u32 = 0x0e10_0000;
    const RENDERER_VTABLE: u32 = 0x0e11_0000;
    const RENDERER_SLOT: u32 = 0x0e12_0000;
    const OBJECT_SLOT: u32 = 0x0e13_0000;

    /// Doubles for every callee of `00b596a0`: the cached geometry is `geometry`, the map finds
    /// nothing, new batches have room 3 (+0xc 3, +0xe 0) and first word 0x77.
    fn batch_rig(geometry: u32, grid: u32) -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.set_global(GRID_SIZE_011F9654, grid);
        e.mem.map(MAP, 0x100);
        e.register_double(CACHED_GEOMETRY_00B587C0, move |_, _| geometry.into_ret());
        e.register(FIRST_MAP_00B585C0, |_, _| MAP.into_ret());
        e.register(MAP_FIND_00A53C30, |_, _| 0u32.into_ret());
        e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        for target in [
            VALUE_SET_00A5F860,
            MAP_INSERT_00B57EA0,
            BATCH_ATTACH_00BD5DD0,
            LIST_ADD_00B58570,
            RENDERER_SLOT,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(BATCH_CONSTRUCT_00BD6190, |e, a| {
            e.mem.set_u32(a[0], 0x77);
            e.mem.set_u16(a[0] + 0xc, 3);
            e.mem.set_u16(a[0] + 0xe, 0);
            a[0].into_ret()
        });
        let renderer = e.mem.alloc(4);
        e.mem.set_u32(renderer, RENDERER_VTABLE);
        e.mem.map(RENDERER_VTABLE, 0x100);
        e.mem.set_u32(RENDERER_VTABLE + 0xdc, RENDERER_SLOT);
        e.set_global(RENDERER_011F9644, renderer);
        e.call_log = Some(vec![]);
        e
    }

    /// The map finds `value`.
    fn finds(e: &mut Engine, value: u32) {
        e.register_double(MAP_FIND_00A53C30, move |e, a| {
            e.mem.set_u32(a[2], value);
            1u32.into_ret()
        });
    }

    fn key(e: &mut Engine) -> u32 {
        let key = e.mem.alloc(0xc);
        for (i, word) in [0x11u32, 0x22, 0x33].iter().enumerate() {
            e.mem.set_u32(key + 4 * i as u32, *word);
        }
        key
    }

    /// A batch of `cell` with the 16-bit words +0xc and +0xe.
    fn batch(e: &mut Engine, cell: u32, size: u16, used: u16) -> u32 {
        let batch = e.mem.alloc(0x28);
        e.mem.set_u16(batch + 0xc, size);
        e.mem.set_u16(batch + 0xe, used);
        e.mem.set_u32(batch + 0x24, cell);
        batch
    }

    /// A list (node: next +0, item +8) of `items` at `slot`.
    fn list(e: &mut Engine, slot: u32, items: &[u32]) {
        let mut next = 0;
        for item in items.iter().rev() {
            let node = e.mem.alloc(0xc);
            e.mem.set_u32(node, next);
            e.mem.set_u32(node + 8, *item);
            next = node;
        }
        e.mem.set_u32(slot, next);
    }

    #[test]
    fn fn_00b596a0_without_geometry_answers_zero() {
        let mut e = batch_rig(0, 4);
        let key = key(&mut e);
        let out = e.mem.alloc(4);
        e.mem.set_u32(out, 0x1234);
        assert_eq!(e.call(0x00b5_96a0, &args![key, 0u32, out, 0u32]).u32(), 0);
        assert_eq!(e.mem.u32(out), 0);
        assert!(calls(&e, FIRST_MAP_00B585C0).is_empty());
    }

    #[test]
    fn fn_00b596a0_reuses_an_item_of_the_cell() {
        let mut e = batch_rig(0x500, 4);
        // Grid 4, position (high 5, low -3): cell (6, 2).
        let cell = 0x0006_0002u32;
        let full = batch(&mut e, cell, 2, 2);
        let other = batch(&mut e, 0x0006_0006, 5, 1);
        let roomy = batch(&mut e, cell, 5, 1);
        let value = e.mem.alloc(0x28);
        list(&mut e, value + 0xc, &[full, other, roomy]);
        finds(&mut e, value);
        let key = key(&mut e);
        let out = e.mem.alloc(4);
        let position = 0x0005_fffdu32;
        assert_eq!(
            e.call(0x00b5_96a0, &args![key, position, out, 0u32]).u32(),
            4
        );
        assert_eq!(e.mem.u32(out), roomy);
        assert_eq!(calls(&e, MAP_FIND_00A53C30)[0][1], 0x33);
        assert!(calls(&e, MEMORY_ALLOCATE).is_empty());
        assert!(calls(&e, RENDERER_SLOT).is_empty());
    }

    #[test]
    fn fn_00b596a0_makes_the_value_and_a_batch() {
        // Grid 1: the cell is the grid word itself.
        let mut e = batch_rig(0x500, 1);
        let key = key(&mut e);
        let out = e.mem.alloc(4);
        e.set_global(CLEARED_011F963C, 7u32);
        assert_eq!(
            e.call(0x00b5_96a0, &args![key, 0x0009_0009u32, out, 0u32])
                .u32(),
            3
        );
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 2);
        let insert = &calls(&e, MAP_INSERT_00B57EA0)[0];
        let value = insert[2];
        assert_eq!(insert[1], 0x33);
        assert_eq!(e.mem.u32(value), 0x11);
        assert_eq!(e.mem.u32(value + 8), 0x33);
        assert_eq!(
            calls(&e, VALUE_SET_00A5F860),
            vec![vec![value + 0x18, 0x500]]
        );
        let batch = e.mem.u32(out);
        assert_eq!(
            calls(&e, BATCH_CONSTRUCT_00BD6190),
            vec![vec![batch, 0x500, 1, 0]]
        );
        assert_eq!(e.global::<u32>(CLEARED_011F963C), 8);
        assert_eq!(calls(&e, BATCH_ATTACH_00BD5DD0), vec![vec![batch, key]]);
        assert_eq!(calls(&e, LIST_ADD_00B58570), vec![vec![value + 0xc, out]]);
        assert_eq!(calls(&e, RENDERER_SLOT)[0][1..], [0x77, 1]);
    }

    #[test]
    fn fn_00b596a0_replaces_a_full_batch() {
        let mut e = batch_rig(0x500, 1);
        let full = batch(&mut e, 1, 4, 4);
        let value = e.mem.alloc(0x28);
        list(&mut e, value + 0xc, &[full]);
        finds(&mut e, value);
        let key = key(&mut e);
        let out = e.mem.alloc(4);
        assert_eq!(e.call(0x00b5_96a0, &args![key, 0u32, out, 0u32]).u32(), 3);
        assert_ne!(e.mem.u32(out), full);
        assert_eq!(calls(&e, MEMORY_ALLOCATE).len(), 1);
    }

    /// `00b59ae0`'s rig: the map finds a value whose list holds one batch of cell 1 with room
    /// `room`; the object's slot +0xbc is `OBJECT_SLOT`.
    fn points_rig(room: u16) -> (Engine, u32, u32, u32) {
        let mut e = batch_rig(0x500, 1);
        let batch = batch(&mut e, 1, room, 0);
        let value = e.mem.alloc(0x28);
        list(&mut e, value + 0xc, &[batch]);
        finds(&mut e, value);
        for target in [
            BATCH_ADD_00BD5D70,
            BATCH_FINISH_00BD5E40,
            LIST_ADD_00B584E0,
            OBJECT_SLOT,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        let vtable = e.mem.alloc(0x100);
        e.mem.set_u32(vtable + 0xbc, OBJECT_SLOT);
        let object = e.mem.alloc(4);
        e.mem.set_u32(object, vtable);
        (e, value, batch, object)
    }

    #[test]
    fn fn_00b59ae0_adds_points_and_the_item() {
        let (mut e, value, batch, object) = points_rig(5);
        let key = key(&mut e);
        let points = e.mem.alloc(24);
        for (i, v) in [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0].iter().enumerate() {
            e.mem.set_f32(points + 4 * i as u32, *v);
        }
        let values = e.mem.alloc(8);
        e.mem.set_f32(values, 7.0);
        e.mem.set_f32(values + 4, 8.0);
        e.register_double(BATCH_ADD_00BD5D70, |e, a| {
            let first = e.mem.u32(a[1]);
            let last = e.mem.u32(a[1] + 12);
            e.mem.set_u32(0x011f_9f00, first);
            e.mem.set_u32(0x011f_9f04, last);
            Ret::default()
        });
        e.set_global(CLEARED_011F9640, 1u32);
        e.call(
            0x00b5_9ae0,
            &args![0u32, 0x99u32, object, key, points, values, 0x0001_0002u32],
        );
        assert_eq!(e.global::<u32>(CLEARED_011F9640), 3);
        let adds = calls(&e, BATCH_ADD_00BD5D70);
        assert_eq!(adds.len(), 2);
        assert!(adds.iter().all(|a| a[0] == batch && a[2] == 0x99));
        // The last point added: x of point 1 and value 1.
        assert_eq!(e.mem.f32(0x011f_9f00), 4.0);
        assert_eq!(e.mem.f32(0x011f_9f04), 8.0);
        assert_eq!(calls(&e, BATCH_FINISH_00BD5E40), vec![vec![batch]]);
        assert_eq!(calls(&e, OBJECT_SLOT), vec![vec![object]]);
        // The item is not in the value's list at +0x1c: added.
        assert_eq!(calls(&e, LIST_ADD_00B584E0)[0][0], value + 0x1c);
    }

    #[test]
    fn fn_00b59ae0_quiets_values_and_skips_a_listed_item() {
        let (mut e, value, _, object) = points_rig(5);
        let key = key(&mut e);
        let points = e.mem.alloc(12);
        let values = e.mem.alloc(4);
        e.mem.set_u32(values, 0x7f80_0001);
        e.register_double(BATCH_ADD_00BD5D70, |e, a| {
            let word = e.mem.u32(a[1] + 12);
            e.mem.set_u32(0x011f_9f04, word);
            Ret::default()
        });
        list(&mut e, value + 0x1c, &[0x98, 0x99]);
        e.call(
            0x00b5_9ae0,
            &args![0u32, 0x99u32, object, key, points, values, 1u32],
        );
        assert_eq!(e.mem.u32(0x011f_9f04), 0x7fc0_0001);
        assert!(calls(&e, LIST_ADD_00B584E0).is_empty());
    }

    #[test]
    fn fn_00b59ae0_stops_without_a_batch() {
        let mut e = batch_rig(0, 1);
        let key = key(&mut e);
        e.call(0x00b5_9ae0, &args![0u32, 0u32, 0u32, key, 0u32, 0u32, 3u32]);
        assert!(calls(&e, BATCH_FINISH_00BD5E40).is_empty());
        assert_eq!(e.global::<u32>(CLEARED_011F9640), 3);
    }

    /// A view at `011f9684` with the identity matrix but row 3 = (0, 0, 0, depth), the four
    /// `float`s and the constants; `memcpy` and the square root doubled.
    fn view_rig(depth: f32, corners: [f32; 4]) -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011f_9000u32,
            0x0101_1000,
            0x0101_2000,
            0x0101_6000,
            0x0101_7000,
        ] {
            e.map(page, 0x1000);
        }
        e.set_global(MINUS_ONE_01012054, -1.0f32);
        e.set_global(HALF_DOUBLE_01011588, 0.5f64);
        e.set_global(HALF_01016248, 0.5f32);
        e.set_global(SCALE_01017870, 0.3f32);
        let view = e.mem.alloc(0x110);
        let matrix = [
            1.0f32, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, depth,
        ];
        for (i, v) in matrix.iter().enumerate() {
            e.mem.set_f32(view + 0x9c + 4 * i as u32, *v);
        }
        for (i, v) in corners.iter().enumerate() {
            e.mem.set_f32(view + 0x100 + 4 * i as u32, *v);
        }
        e.set_global(VIEW_011F9684, view);
        e.register(MEMCPY_00EC44D0, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            a[0].into_ret()
        });
        e.register(SQRT_00EC6040, |_, a| Ret {
            st0: f64::from_bits(u64::from(a[0]) | u64::from(a[1]) << 32).sqrt(),
            ..Ret::default()
        });
        e
    }

    fn project(e: &mut Engine, point: [f32; 4]) -> (f32, f32, f32) {
        let p = e.mem.alloc(16);
        for (i, v) in point.iter().enumerate() {
            e.mem.set_f32(p + 4 * i as u32, *v);
        }
        let out = e.mem.alloc(8);
        let scale = e.mem.alloc(4);
        e.call(0x00b5_9d30, &args![p, out, scale]);
        (e.mem.f32(out), e.mem.f32(out + 4), e.mem.f32(scale))
    }

    #[test]
    fn fn_00b59d30_projects_inside_the_view() {
        // a, b = 0, 1 and c, d = 1, 0: u = (1 + s) / 2 and v = 1 - (1 + t) / 2.
        let mut e = view_rig(1.0, [0.0, 1.0, 1.0, 0.0]);
        // Depth 1: s = 0.5, t = -0.5; the second point (z + 0.5) projects the same.
        let (u, v, scale) = project(&mut e, [0.5, -0.5, 0.0, 0.5]);
        assert_eq!(u, 0.75);
        assert_eq!(v, 0.75);
        assert_eq!(scale, 0.0);
        // With z in row 0 (m[2] = 1) the second projection moves by w.
        let view = e.global::<u32>(VIEW_011F9684);
        e.mem.set_f32(view + 0x9c + 8, 1.0);
        let (_, _, scale) = project(&mut e, [0.0, 0.0, 0.0, 0.5]);
        assert_eq!(scale, 0.25);
        // A large move is capped at 0.5.
        let (_, _, scale) = project(&mut e, [0.0, 0.0, 0.0, 4.0]);
        assert_eq!(scale, 0.5);
    }

    #[test]
    fn fn_00b59d30_behind_and_outside_uses_point_three_and_snaps() {
        let mut e = view_rig(-1.0, [0.0, 1.0, 1.0, 0.0]);
        // Depth -1, s = 2 (outside): 0.3; u (1.5, clamped to 1) is above 0: 1.
        let (u, v, scale) = project(&mut e, [2.0, 0.0, 0.0, 0.0]);
        assert_eq!(scale, 0.3);
        assert_eq!(u, 1.0);
        assert_eq!(v, 0.5);
        // s = -3: u = -1, clamped to 0, at most 0: 0.
        let (u, _, _) = project(&mut e, [-3.0, 0.0, 0.0, 0.0]);
        assert_eq!(u, 0.0);
        // Behind but inside: the second projection divides by the signed depth (-1), so
        // s2 = -0.5 and the scale is half of 1.
        let (u, _, scale) = project(&mut e, [0.5, 0.0, 0.0, 0.0]);
        assert_eq!(u, 1.0);
        assert_eq!(scale, 0.5);
    }

    #[test]
    fn fn_00b59d30_clamps_in_front() {
        let mut e = view_rig(1.0, [0.0, 1.0, 1.0, 0.0]);
        // s = 3, t = -3: outside but in front: u clamped to 1, v to 1.
        let (u, v, _) = project(&mut e, [3.0, -3.0, 0.0, 0.0]);
        assert_eq!(u, 1.0);
        assert_eq!(v, 1.0);
        let (u, v, _) = project(&mut e, [-3.0, 3.0, 0.0, 0.0]);
        assert_eq!(u, 0.0);
        assert_eq!(v, 0.0);
    }

    #[test]
    fn fn_00b5a220_sets_the_constants() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.set_global(HALF_01016248, 0.5f32);
        let camera = e.mem.alloc(0x100);
        e.mem.set_f32(camera + 0xec, 2.0);
        e.mem.set_f32(camera + 0xf0, 10.0);
        e.set_global(CAMERA_011F917C, camera);
        let this = e.mem.alloc(0xa0);
        let params = e.mem.alloc(8);
        e.mem.set_u32(params, 0x100);
        e.mem.set_u32(params + 4, 0x200);
        e.mem.set_u32(this + 0x1c, params);
        for i in 0..16u32 {
            e.mem.set_f32(this + 0x5c + 4 * i, i as f32);
        }
        e.mem.set_u32(this + 0x8c, 0x7f80_0001);
        e.register(PARAM_SET_PIXEL_CONSTANT_00B8A790, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00b5_a220, &args![this, 0u32]).bool());
        assert_eq!(e.mem.f32(this + 0x60), 2.0);
        assert_eq!(e.mem.f32(this + 0x64), 8.0);
        assert_eq!(e.mem.f32(this + 0x68), 20.0);
        let f = |v: f32| v.to_bits();
        assert_eq!(
            calls(&e, PARAM_SET_PIXEL_CONSTANT_00B8A790),
            vec![
                vec![0x200, 0, f(8.0), f(9.0), f(10.0), f(11.0)],
                vec![0x200, 1, f(4.0), f(5.0), f(6.0), f(7.0)],
                vec![0x200, 2, f(0.0), f(2.0), f(8.0), f(20.0)],
                vec![0x100, 0, 0x7fc0_0001, f(13.0), f(14.0), 0],
                vec![0x100, 1, f(0.5), f(0.5), 0, 0],
            ]
        );
    }

    #[test]
    fn fn_00b5a340_and_00b5a3a0_borrow_and_return_textures() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xd4);
        let textures = e.mem.alloc(0x10);
        e.mem.set_u32(textures + 8, 0x4242);
        e.mem.set_u32(this + 0x2c, textures);
        for target in [
            BORROW_RENDERED_TEXTURE_00BA3840,
            RETURN_RENDERED_TEXTURE_00BA39A0,
            TEXTURE_SET_00BA37F0,
        ] {
            e.register(target, |_, _| Ret::default());
        }
        e.register(MANAGER_TEXTURE_00B54090, |_, _| 0x5150u32.into_ret());
        e.mem.map(0x0e20_0000, 0x10);
        e.register(ARRAY_SET_AT_00BA3AA0, |e, a| {
            let word = e.mem.u32(a[2]);
            e.mem.set_u32(0x0e20_0000 + 4 * a[1], word);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x00b5_a340, &args![this, 0u32]);
        assert_eq!(
            calls(&e, BORROW_RENDERED_TEXTURE_00BA3840),
            vec![vec![this + 0xb4, 0x28]]
        );
        assert_eq!(
            calls(&e, TEXTURE_SET_00BA37F0),
            vec![vec![this + 0xc4, 0x5150]]
        );
        assert_eq!(e.mem.u32(0x0e20_0008), this + 0xb4);
        assert_eq!(e.mem.u32(0x0e20_000c), this + 0xc4);
        e.call(0x00b5_a3a0, &args![this]);
        assert_eq!(
            calls(&e, RETURN_RENDERED_TEXTURE_00BA39A0),
            vec![vec![0x4242]]
        );
        assert_eq!(e.mem.u32(0x0e20_0008), 0);
        assert_eq!(e.mem.u32(0x0e20_000c), 0);
        let sets = calls(&e, ARRAY_SET_AT_00BA3AA0);
        assert_eq!(sets.len(), 4);
        assert!(sets.iter().all(|a| a[0] == this + 0x28));
    }

    #[test]
    fn fn_00b5a8c0_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(
            0x00b5_a8c0,
            &args![0u32, 1u32, 2u32, 3u32, 4u32, 5u32, 6u32],
        );
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }
}
